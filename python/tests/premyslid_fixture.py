"""A minimal valid Premyslid game on disk, for tests to break in one way at a time.

Kept out of the test modules themselves so that the validation tests and the gap tests share
one definition of what "valid content" looks like. If the fixture and the real schema ever
disagree, the fixture is wrong: it copies the real schema directory rather than restating it.
"""

from __future__ import annotations

import json
import shutil
import tempfile
import unittest
from pathlib import Path

START_DATE = "1066-09-15"

SCHEMA_SOURCE = Path(__file__).resolve().parents[2] / "games" / "premyslid" / "schema"


def write(path: Path, record: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(record, indent=2), encoding="utf-8")


class GameFixture:
    """One kingdom, two duchies, one fully-finished county in the exemplar duchy."""

    def __init__(self, root: Path) -> None:
        self.root = root
        shutil.copytree(SCHEMA_SOURCE, root / "schema")
        write(
            root / "plan" / "regions.json",
            {
                "start_date": START_DATE,
                "exemplar_duchy": "d_bohemia",
                "bulk_unlocked": False,
                "holdings_per_county": 1,
                "exemplar_event_budget": 0,
                "hierarchy": [
                    {
                        "id": "k_bohemia",
                        "tier": "kingdom",
                        "priority": 1,
                        "duchies": [
                            {"id": "d_bohemia", "priority": 1, "counties": ["c_praha"]},
                            {"id": "d_moravia", "priority": 2, "counties": ["c_brno"]},
                        ],
                    }
                ],
            },
        )
        write(
            root / "content" / "cultures" / "czech.json",
            {"id": "czech", "name": "Czech", "group": "west_slavic"},
        )
        write(
            root / "content" / "faiths" / "catholic.json",
            {"id": "catholic", "name": "Catholic", "group": "christian"},
        )
        write(
            root / "content" / "succession_laws" / "agnatic_seniority.json",
            {
                "id": "agnatic_seniority",
                "name": "Agnatic seniority",
                "kind": "seniority",
                "gender_law": "agnatic",
            },
        )
        write(
            root / "content" / "dynasties" / "premyslid.json",
            {"id": "premyslid", "name": "Premyslid", "sources": ["x"]},
        )
        write(
            root / "content" / "characters" / "vratislav_ii.json",
            {
                "id": "vratislav_ii",
                "name": "Vratislav",
                "sex": "male",
                "birth": "1032-01-01",
                "death": "1092-01-14",
                "dynasty": "premyslid",
                "sources": ["x"],
            },
        )
        write(
            root / "content" / "titles" / "d_bohemia.json",
            {
                "id": "d_bohemia",
                "tier": "duchy",
                "name": "Bohemia",
                "de_jure_liege": None,
                "capital": "c_praha",
                "holder": "vratislav_ii",
                "succession_law": "agnatic_seniority",
                "sources": ["x"],
            },
        )
        write(
            root / "content" / "titles" / "c_praha.json",
            {
                "id": "c_praha",
                "tier": "county",
                "name": "Praha",
                "de_jure_liege": "d_bohemia",
                "capital": "b_praha",
                "holder": "vratislav_ii",
                "sources": ["x"],
            },
        )
        write(
            root / "content" / "titles" / "b_praha.json",
            {
                "id": "b_praha",
                "tier": "barony",
                "name": "Prague Castle",
                "de_jure_liege": "c_praha",
                "holder": "vratislav_ii",
                "sources": ["x"],
            },
        )
        write(
            root / "content" / "holdings" / "b_praha.json",
            {"id": "b_praha", "county": "c_praha", "type": "castle", "sources": ["x"]},
        )
        (root / "research").mkdir(parents=True, exist_ok=True)

    def title(self, title_id: str) -> Path:
        return self.root / "content" / "titles" / f"{title_id}.json"

    def character(self, character_id: str) -> Path:
        return self.root / "content" / "characters" / f"{character_id}.json"

    def plan(self) -> Path:
        return self.root / "plan" / "regions.json"

    def edit(self, path: Path, **changes: object) -> None:
        record = json.loads(path.read_text(encoding="utf-8"))
        record.update(changes)
        path.write_text(json.dumps(record, indent=2), encoding="utf-8")

    def issues(self) -> list[str]:
        from suvorov.tools import content, validate

        return [str(issue) for issue in validate.validate_game(content.load_game(self.root))]

    def assert_one_issue_mentioning(self, test: unittest.TestCase, fragment: str) -> None:
        issues = self.issues()
        matching = [issue for issue in issues if fragment in issue]
        test.assertEqual(
            len(matching), 1, f"expected exactly one issue mentioning {fragment!r}, got {issues}"
        )


class FixtureTestCase(unittest.TestCase):
    """Base class that gives each test its own throwaway copy of the fixture game."""

    def setUp(self) -> None:
        self._directory = tempfile.TemporaryDirectory()
        self.game = GameFixture(Path(self._directory.name))

    def tearDown(self) -> None:
        self._directory.cleanup()
