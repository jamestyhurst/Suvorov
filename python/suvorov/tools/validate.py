"""Checking a game's content for everything the schema alone cannot see.

This module is the review step. There is no engine to run the content against and no human
reading every pull request, so these checks are the only thing standing between a plausible
looking file and a scenario that quietly makes no sense.

Messages are written as instructions to whoever has to fix the file, not as specification
citations, because the reader is usually a small model with no context beyond the message.
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from datetime import date
from pathlib import Path

from suvorov.tools import content, schema_lite
from suvorov.tools.content import Game, Record

MINIMUM_PARENT_AGE_YEARS = 12

# Records making claims about real people and places must cite something. Cultures, faiths,
# and succession laws are modelling abstractions rather than claims, so they do not.
KINDS_REQUIRING_SOURCES = ("titles", "characters", "dynasties", "holdings")


@dataclass(frozen=True)
class Issue:
    """One problem with the content, addressed to whoever has to fix it."""

    where: str
    message: str

    def __str__(self) -> str:
        return f"{self.where}: {self.message}"


def validate_game(game: Game) -> list[Issue]:
    """Return every problem found, in file order. Never raises on bad content."""

    issues: list[Issue] = []
    for unreadable in game.unreadable:
        issues.append(
            Issue(game.relative(unreadable.path), f"could not be read as JSON ({unreadable.reason})")
        )

    issues.extend(_check_structure(game))
    issues.extend(_check_identity(game))
    issues.extend(_check_hierarchy(game))
    issues.extend(_check_capitals(game))
    issues.extend(_check_succession_law_placement(game))
    issues.extend(_check_holdings(game))
    issues.extend(_check_people(game))
    issues.extend(_check_holders_alive(game))
    issues.extend(_check_sources(game))
    return issues


def _check_structure(game: Game) -> list[Issue]:
    """Schema conformance, plus resolution of every ``x-ref`` the schema collected."""

    issues: list[Issue] = []
    for kind, records in game.records.items():
        schema = game.schemas[kind]
        for record in records.values():
            references: list[schema_lite.Reference] = []
            for error in schema_lite.validate(record.data, schema, references=references):
                issues.append(Issue(game.relative(record.path), str(error)))
            for reference in references:
                if game.exists(reference.kind, reference.target):
                    continue
                singular = content.KIND_SPECS[reference.kind][2]
                issues.append(
                    Issue(
                        game.relative(record.path),
                        f"{reference.path} points at a record that does not exist: there is no "
                        f"{singular} called '{reference.target}'. Either create it or correct "
                        f"the spelling.",
                    )
                )
    return issues


def _check_identity(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    for records in game.records.values():
        for record in records.values():
            declared = record.data.get("id")
            if isinstance(declared, str) and declared != record.record_id:
                issues.append(
                    Issue(
                        game.relative(record.path),
                        f"the file name says '{record.record_id}' but the id inside says "
                        f"'{declared}'. Rename the file or change the id so they agree.",
                    )
                )

    for title in game.of_kind("titles").values():
        expected = content.tier_of(title.record_id)
        declared = title.data.get("tier")
        if expected is not None and declared is not None and declared != expected:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"the id prefix means this is a {expected}, but the tier says "
                    f"'{declared}'. The prefixes are e, k, d, c, b for empire, kingdom, "
                    f"duchy, county, barony.",
                )
            )
    return issues


def _check_hierarchy(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    titles = game.of_kind("titles")

    for title in titles.values():
        liege_id = title.data.get("de_jure_liege")
        if not isinstance(liege_id, str):
            continue
        liege = titles.get(liege_id)
        if liege is None:
            continue  # already reported as a dangling reference
        own_tier = title.data.get("tier")
        if not isinstance(own_tier, str) or own_tier not in content.TIER_ORDER:
            continue
        expected = content.tier_above(own_tier)
        actual = liege.data.get("tier")
        if expected is not None and actual != expected:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"de_jure_liege must be one tier above this title. A {own_tier} belongs to "
                    f"a {expected}, but '{liege_id}' is a {actual}.",
                )
            )

    for title_id in sorted(titles):
        seen: list[str] = []
        current: str | None = title_id
        while isinstance(current, str) and current in titles:
            if current in seen:
                issues.append(
                    Issue(
                        game.relative(titles[title_id].path),
                        "the de jure hierarchy forms a cycle: "
                        + " -> ".join(seen[seen.index(current) :] + [current]),
                    )
                )
                break
            seen.append(current)
            current = titles[current].data.get("de_jure_liege")
    return _first_cycle_only(issues)


def _first_cycle_only(issues: list[Issue]) -> list[Issue]:
    """A cycle is visible from every title in it. Report it once, from the first."""

    kept: list[Issue] = []
    cycles_seen: set[str] = set()
    for issue in issues:
        if "cycle" not in issue.message:
            kept.append(issue)
            continue
        signature = frozenset(issue.message.split(": ", 1)[-1].split(" -> "))
        key = "|".join(sorted(signature))
        if key in cycles_seen:
            continue
        cycles_seen.add(key)
        kept.append(issue)
    return kept


def _check_capitals(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    titles = game.of_kind("titles")

    for title in titles.values():
        capital_id = title.data.get("capital")
        own_tier = title.data.get("tier")
        if own_tier == "barony":
            if capital_id is not None:
                issues.append(
                    Issue(
                        game.relative(title.path),
                        "a barony is the bottom of the hierarchy and has no capital. Set capital to null.",
                    )
                )
            continue
        if not isinstance(capital_id, str):
            continue
        capital = titles.get(capital_id)
        if capital is None:
            continue
        expected_tier = content.tier_below(own_tier) if isinstance(own_tier, str) else None
        if expected_tier is not None and capital.data.get("tier") != expected_tier:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"the capital of a {own_tier} must be a {expected_tier}, but '{capital_id}' "
                    f"is a {capital.data.get('tier')}.",
                )
            )
            continue
        if capital.data.get("de_jure_liege") != title.record_id:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"the capital '{capital_id}' does not belong to this title: its "
                    f"de_jure_liege is '{capital.data.get('de_jure_liege')}'.",
                )
            )
    return issues


def _check_succession_law_placement(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    for title in game.of_kind("titles").values():
        tier = title.data.get("tier")
        law = title.data.get("succession_law")
        if tier in ("empire", "kingdom", "duchy") and law is None:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"a {tier} must have a succession law; this is the rule that decides who "
                    f"inherits it.",
                )
            )
        if tier in ("county", "barony") and law is not None:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"a {tier} must not carry its own succession law in version 0; it follows "
                    f"its liege. Set succession_law to null.",
                )
            )
    return issues


def _check_holdings(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    holdings = game.of_kind("holdings")
    baronies = game.titles_of_tier("barony")

    for barony_id, barony in baronies.items():
        if barony_id not in holdings:
            issues.append(
                Issue(
                    game.relative(barony.path),
                    f"there is no holding record for this barony. Create "
                    f"content/holdings/{barony_id}.json.",
                )
            )

    for holding_id, holding in holdings.items():
        if holding_id not in baronies:
            issues.append(
                Issue(
                    game.relative(holding.path),
                    f"there is no barony title for this holding. Create "
                    f"content/titles/{holding_id}.json, or delete this file.",
                )
            )
            continue
        county_id = holding.data.get("county")
        barony_liege = baronies[holding_id].data.get("de_jure_liege")
        if isinstance(county_id, str) and barony_liege is not None and county_id != barony_liege:
            issues.append(
                Issue(
                    game.relative(holding.path),
                    f"this holding says it is in '{county_id}', but its barony title says its "
                    f"liege is '{barony_liege}'.",
                )
            )
    return issues


def _parse_date(value: object) -> date | None:
    if not isinstance(value, str):
        return None
    try:
        year, month, day = (int(part) for part in value.split("-"))
        return date(year, month, day)
    except (ValueError, TypeError):
        return None


def _check_people(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    characters = game.of_kind("characters")

    for character in characters.values():
        birth = _parse_date(character.data.get("birth"))
        death = _parse_date(character.data.get("death"))
        if birth is not None and death is not None and death < birth:
            issues.append(
                Issue(
                    game.relative(character.path),
                    f"this character dies before they are born: born {character.data['birth']}, "
                    f"died {character.data['death']}.",
                )
            )

        for role, expected_sex in (("father", "male"), ("mother", "female")):
            parent_id = character.data.get(role)
            if not isinstance(parent_id, str):
                continue
            parent = characters.get(parent_id)
            if parent is None:
                continue
            if parent.data.get("sex") != expected_sex:
                issues.append(
                    Issue(
                        game.relative(character.path),
                        f"the {role} '{parent_id}' is recorded as "
                        f"{parent.data.get('sex')}; a {role} must be {expected_sex}.",
                    )
                )
                continue
            parent_birth = _parse_date(parent.data.get("birth"))
            if birth is None or parent_birth is None:
                continue
            if (birth - parent_birth).days < MINIMUM_PARENT_AGE_YEARS * 365:
                issues.append(
                    Issue(
                        game.relative(character.path),
                        f"the {role} '{parent_id}' must be at least "
                        f"{MINIMUM_PARENT_AGE_YEARS} years older than this character.",
                    )
                )

        for spouse_id in character.data.get("spouses", []) or []:
            spouse = characters.get(spouse_id)
            if spouse is None:
                continue
            if character.record_id not in (spouse.data.get("spouses") or []):
                issues.append(
                    Issue(
                        game.relative(character.path),
                        f"'{spouse_id}' does not list '{character.record_id}' as a spouse. A "
                        f"marriage must be recorded on both characters.",
                    )
                )

    issues.extend(_check_parentage_cycles(game))
    return issues


def _check_parentage_cycles(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    characters = game.of_kind("characters")
    for character_id in sorted(characters):
        for role in ("father", "mother"):
            seen: list[str] = []
            current: str | None = character_id
            while isinstance(current, str) and current in characters:
                if current in seen:
                    issues.append(
                        Issue(
                            game.relative(characters[character_id].path),
                            f"the {role} line forms a cycle: "
                            + " -> ".join(seen[seen.index(current) :] + [current]),
                        )
                    )
                    break
                seen.append(current)
                current = characters[current].data.get(role)
    return _first_cycle_only(issues)


def _check_holders_alive(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    start = _parse_date(game.plan.get("start_date"))
    if start is None:
        return [Issue("plan/regions.json", "start_date is missing or is not a real date.")]

    characters = game.of_kind("characters")
    for title in game.of_kind("titles").values():
        holder_id = title.data.get("holder")
        if not isinstance(holder_id, str):
            continue
        holder = characters.get(holder_id)
        if holder is None:
            continue
        birth = _parse_date(holder.data.get("birth"))
        death = _parse_date(holder.data.get("death"))
        if birth is not None and birth > start:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"the holder '{holder_id}' is not yet born on the scenario start date "
                    f"{game.plan['start_date']}.",
                )
            )
        if death is not None and death <= start:
            issues.append(
                Issue(
                    game.relative(title.path),
                    f"the holder '{holder_id}' was already dead on the scenario start date "
                    f"{game.plan['start_date']}.",
                )
            )
    return issues


def _check_sources(game: Game) -> list[Issue]:
    issues: list[Issue] = []
    for kind in KINDS_REQUIRING_SOURCES:
        for record in game.of_kind(kind).values():
            if not record.data.get("sources"):
                issues.append(
                    Issue(
                        game.relative(record.path),
                        "add at least one source: this record makes a claim about a real place "
                        "or person, and the claim has to be checkable.",
                    )
                )
    return issues


def _default_game_root() -> Path:
    return Path(__file__).resolve().parents[3] / "games" / "premyslid"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Check game content against its schema and rules.")
    parser.add_argument(
        "--game",
        type=Path,
        default=_default_game_root(),
        help="Path to the game folder. Defaults to games/premyslid.",
    )
    arguments = parser.parse_args(argv)

    game = content.load_game(arguments.game)
    issues = validate_game(game)
    record_count = sum(len(records) for records in game.records.values())

    if not issues:
        print(f"Content is valid: {record_count} record(s) checked.")
        return 0

    for issue in issues:
        print(issue)
    print(f"\n{len(issues)} problem(s) in {record_count} record(s).")
    return 1


if __name__ == "__main__":
    sys.exit(main())
