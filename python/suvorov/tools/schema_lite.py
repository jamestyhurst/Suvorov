"""A deliberately small JSON Schema subset, implemented with the standard library only.

Why not use the ``jsonschema`` package: Suvorov has no third-party dependencies, the school
PC raises an IT alert on new installs, and the free models that will be writing content need
error messages phrased as instructions rather than as specification citations. Implementing
the subset costs about two hundred lines and buys all three.

The supported keywords are documented in ``games\\premyslid\\schema\\README.md``. Anything
outside that list is reported as an error in the *schema*, so a schema file that quietly
reaches for ``oneOf`` fails loudly instead of validating nothing.
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from datetime import date

SUPPORTED_KEYWORDS = frozenset(
    {
        "$id",
        "title",
        "description",
        "comment",
        "type",
        "properties",
        "required",
        "additionalProperties",
        "enum",
        "pattern",
        "items",
        "minItems",
        "minimum",
        "maximum",
        "minLength",
        "x-ref",
        "x-date",
    }
)

_TYPE_CHECKS = {
    "object": lambda value: isinstance(value, dict),
    "array": lambda value: isinstance(value, list),
    "string": lambda value: isinstance(value, str),
    # bool is a subclass of int in Python; a boolean is not an integer in this dialect.
    "integer": lambda value: isinstance(value, int) and not isinstance(value, bool),
    "number": lambda value: isinstance(value, (int, float)) and not isinstance(value, bool),
    "boolean": lambda value: isinstance(value, bool),
    "null": lambda value: value is None,
}

_DATE_PATTERN = re.compile(r"^(\d{4})-(\d{2})-(\d{2})$")


@dataclass(frozen=True)
class Error:
    """One validation failure, addressed to whoever has to fix the file."""

    path: str
    message: str

    def __str__(self) -> str:
        where = self.path or "<record>"
        return f"{where}: {self.message}"


@dataclass(frozen=True)
class Reference:
    """A value that names another record. Resolving it is the caller's job."""

    path: str
    kind: str
    target: str


def validate(
    instance: object,
    schema: dict,
    *,
    path: str = "",
    references: list[Reference] | None = None,
) -> list[Error]:
    """Check ``instance`` against ``schema`` and return every failure found.

    Validation does not stop at the first error: a content author working without a human
    reviewer benefits far more from one list of everything wrong than from a sequence of
    single failures discovered one run at a time.

    When ``references`` is given, every non-null value carrying ``x-ref`` is appended to it
    rather than resolved here. This module has no idea what records exist; the caller does.
    """

    errors: list[Error] = []

    unsupported = sorted(set(schema) - SUPPORTED_KEYWORDS)
    if unsupported:
        errors.append(
            Error(path, f"unsupported schema keyword '{unsupported[0]}' in the schema itself")
        )
        return errors

    if "type" in schema and not _matches_type(instance, schema["type"]):
        expected = schema["type"] if isinstance(schema["type"], str) else " or ".join(schema["type"])
        errors.append(Error(path, f"expected {expected}, found {_describe(instance)}"))
        return errors

    if "enum" in schema and instance not in schema["enum"]:
        allowed = ", ".join(repr(value) for value in schema["enum"])
        errors.append(Error(path, f"{instance!r} is not one of: {allowed}"))
        return errors

    if isinstance(instance, str):
        errors.extend(_check_string(instance, schema, path))
    if isinstance(instance, (int, float)) and not isinstance(instance, bool):
        errors.extend(_check_number(instance, schema, path))
    if isinstance(instance, dict):
        errors.extend(_check_object(instance, schema, path, references))
    if isinstance(instance, list):
        errors.extend(_check_array(instance, schema, path, references))

    if references is not None and "x-ref" in schema and isinstance(instance, str):
        references.append(Reference(path=path, kind=schema["x-ref"], target=instance))

    return errors


def _matches_type(instance: object, expected: object) -> bool:
    names = [expected] if isinstance(expected, str) else list(expected)
    return any(_TYPE_CHECKS[name](instance) for name in names)


def _describe(instance: object) -> str:
    if instance is None:
        return "null"
    if isinstance(instance, bool):
        return "boolean"
    if isinstance(instance, str):
        return "string"
    if isinstance(instance, int):
        return "integer"
    if isinstance(instance, float):
        return "number"
    if isinstance(instance, list):
        return "array"
    if isinstance(instance, dict):
        return "object"
    return type(instance).__name__


def _check_string(instance: str, schema: dict, path: str) -> list[Error]:
    errors: list[Error] = []
    pattern = schema.get("pattern")
    if pattern is not None and re.search(pattern, instance) is None:
        errors.append(Error(path, f"{instance!r} does not match the required pattern {pattern}"))
    minimum_length = schema.get("minLength")
    if minimum_length is not None and len(instance) < minimum_length:
        errors.append(Error(path, f"must be at least {minimum_length} character(s) long"))
    if schema.get("x-date"):
        errors.extend(_check_date(instance, path))
    return errors


def _check_date(instance: str, path: str) -> list[Error]:
    match = _DATE_PATTERN.match(instance)
    if match is None:
        return [Error(path, f"{instance!r} is not a date in YYYY-MM-DD form")]
    year, month, day = (int(part) for part in match.groups())
    try:
        date(year, month, day)
    except ValueError:
        return [Error(path, f"{instance!r} is not a real calendar date")]
    return []


def _check_number(instance: float, schema: dict, path: str) -> list[Error]:
    errors: list[Error] = []
    minimum = schema.get("minimum")
    if minimum is not None and instance < minimum:
        errors.append(Error(path, f"must be {minimum} or greater"))
    maximum = schema.get("maximum")
    if maximum is not None and instance > maximum:
        errors.append(Error(path, f"must be {maximum} or less"))
    return errors


def _check_object(
    instance: dict, schema: dict, path: str, references: list[Reference] | None
) -> list[Error]:
    errors: list[Error] = []
    properties = schema.get("properties", {})

    for name in schema.get("required", []):
        if name not in instance:
            errors.append(Error(path, f"missing required property '{name}'"))

    if schema.get("additionalProperties") is False:
        for name in instance:
            if name not in properties:
                known = ", ".join(sorted(properties)) or "none"
                errors.append(
                    Error(path, f"unknown property '{name}'; the allowed ones are: {known}")
                )

    for name, value in instance.items():
        subschema = properties.get(name)
        if subschema is None:
            continue
        errors.extend(validate(value, subschema, path=_join(path, name), references=references))

    return errors


def _check_array(
    instance: list, schema: dict, path: str, references: list[Reference] | None
) -> list[Error]:
    errors: list[Error] = []
    minimum_items = schema.get("minItems")
    if minimum_items is not None and len(instance) < minimum_items:
        errors.append(Error(path, f"must have at least {minimum_items} item(s)"))
    subschema = schema.get("items")
    if subschema is not None:
        for index, value in enumerate(instance):
            errors.extend(validate(value, subschema, path=f"{path}[{index}]", references=references))
    return errors


def _join(path: str, name: str) -> str:
    return f"{path}.{name}" if path else name
