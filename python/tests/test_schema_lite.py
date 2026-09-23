"""Tests for the small JSON Schema subset used by the Premyslid content schema.

The validator deliberately implements only the keywords documented in
``games/premyslid/schema/README.md``. These tests pin that subset: both that the
supported keywords work, and that an unsupported keyword is reported as an error in the
schema rather than being ignored.
"""

from __future__ import annotations

import unittest

from suvorov.tools import schema_lite


class TestTypes(unittest.TestCase):
    def test_accepts_a_matching_type(self) -> None:
        errors = schema_lite.validate("abc", {"type": "string"})
        self.assertEqual(errors, [])

    def test_rejects_a_mismatched_type(self) -> None:
        errors = schema_lite.validate(5, {"type": "string"})
        self.assertEqual(len(errors), 1)
        self.assertIn("expected string", errors[0].message)

    def test_accepts_any_of_a_type_list(self) -> None:
        schema = {"type": ["string", "null"]}
        self.assertEqual(schema_lite.validate("abc", schema), [])
        self.assertEqual(schema_lite.validate(None, schema), [])
        self.assertEqual(len(schema_lite.validate(3, schema)), 1)

    def test_a_boolean_is_not_an_integer(self) -> None:
        errors = schema_lite.validate(True, {"type": "integer"})
        self.assertEqual(len(errors), 1)


class TestObjects(unittest.TestCase):
    schema = {
        "type": "object",
        "additionalProperties": False,
        "required": ["id"],
        "properties": {"id": {"type": "string"}, "count": {"type": "integer"}},
    }

    def test_accepts_a_valid_object(self) -> None:
        self.assertEqual(schema_lite.validate({"id": "a", "count": 1}, self.schema), [])

    def test_reports_a_missing_required_property(self) -> None:
        errors = schema_lite.validate({"count": 1}, self.schema)
        self.assertEqual(len(errors), 1)
        self.assertIn("required property 'id'", errors[0].message)

    def test_reports_an_unknown_property(self) -> None:
        errors = schema_lite.validate({"id": "a", "idd": 1}, self.schema)
        self.assertEqual(len(errors), 1)
        self.assertIn("unknown property 'idd'", errors[0].message)

    def test_error_paths_name_the_offending_property(self) -> None:
        errors = schema_lite.validate({"id": 3}, self.schema)
        self.assertEqual(errors[0].path, "id")


class TestStringsAndNumbers(unittest.TestCase):
    def test_pattern_is_anchored_as_written(self) -> None:
        schema = {"type": "string", "pattern": "^c_[a-z]+$"}
        self.assertEqual(schema_lite.validate("c_praha", schema), [])
        self.assertEqual(len(schema_lite.validate("d_praha", schema)), 1)

    def test_enum_rejects_a_value_outside_the_list(self) -> None:
        schema = {"enum": ["castle", "city"]}
        self.assertEqual(schema_lite.validate("city", schema), [])
        self.assertEqual(len(schema_lite.validate("temple", schema)), 1)

    def test_min_length_is_enforced(self) -> None:
        self.assertEqual(len(schema_lite.validate("", {"type": "string", "minLength": 1})), 1)

    def test_bounds_are_enforced(self) -> None:
        schema = {"type": "integer", "minimum": -100, "maximum": 100}
        self.assertEqual(schema_lite.validate(0, schema), [])
        self.assertEqual(len(schema_lite.validate(101, schema)), 1)


class TestArrays(unittest.TestCase):
    def test_items_applies_to_every_element(self) -> None:
        schema = {"type": "array", "items": {"type": "string"}}
        self.assertEqual(schema_lite.validate(["a", "b"], schema), [])
        errors = schema_lite.validate(["a", 2], schema)
        self.assertEqual(len(errors), 1)
        self.assertEqual(errors[0].path, "[1]")

    def test_min_items_is_enforced(self) -> None:
        schema = {"type": "array", "items": {"type": "string"}, "minItems": 1}
        self.assertEqual(len(schema_lite.validate([], schema)), 1)


class TestCustomKeywords(unittest.TestCase):
    def test_a_date_must_be_a_real_calendar_date(self) -> None:
        schema = {"type": "string", "x-date": True}
        self.assertEqual(schema_lite.validate("1066-09-15", schema), [])
        self.assertEqual(len(schema_lite.validate("1066-02-30", schema)), 1)
        self.assertEqual(len(schema_lite.validate("15-09-1066", schema)), 1)

    def test_references_are_collected_rather_than_resolved(self) -> None:
        schema = {
            "type": "object",
            "additionalProperties": False,
            "properties": {"holder": {"type": ["string", "null"], "x-ref": "characters"}},
        }
        refs: list[schema_lite.Reference] = []
        errors = schema_lite.validate({"holder": "vratislav_ii"}, schema, references=refs)
        self.assertEqual(errors, [])
        self.assertEqual(refs, [schema_lite.Reference(path="holder", kind="characters", target="vratislav_ii")])

    def test_a_null_reference_is_not_collected(self) -> None:
        schema = {"type": "object", "properties": {"holder": {"type": ["string", "null"], "x-ref": "characters"}}}
        refs: list[schema_lite.Reference] = []
        schema_lite.validate({"holder": None}, schema, references=refs)
        self.assertEqual(refs, [])


class TestUnsupportedKeywords(unittest.TestCase):
    def test_an_unsupported_keyword_is_an_error_not_a_silent_pass(self) -> None:
        errors = schema_lite.validate("anything", {"type": "string", "oneOf": [{"type": "string"}]})
        self.assertEqual(len(errors), 1)
        self.assertIn("unsupported schema keyword 'oneOf'", errors[0].message)


if __name__ == "__main__":
    unittest.main()
