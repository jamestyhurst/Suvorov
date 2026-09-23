"""Tests for the semantic checks the Premyslid validator applies to game content.

Structural checking is covered by ``test_schema_lite.py``. What is tested here is the layer
above it: the rules that need to see the whole content set at once, such as whether a title's
holder was alive on the scenario start date, or whether a county claims a capital that sits
in a different county.

These checks carry most of the project's weight. There is no engine yet and no human
reviewer in the loop, so a rule that is not enforced here is a rule that will be silently
broken for weeks.
"""

from __future__ import annotations

import unittest

from tests.premyslid_fixture import FixtureTestCase, write


class TestCleanContent(FixtureTestCase):
    def test_the_fixture_itself_is_valid(self) -> None:
        self.assertEqual(self.game.issues(), [])


class TestFilesAndIdentity(FixtureTestCase):
    def test_a_file_name_that_does_not_match_the_id_is_reported(self) -> None:
        path = self.game.title("c_praha")
        path.rename(path.with_name("c_prague.json"))
        self.game.assert_one_issue_mentioning(self, "file name")

    def test_unparseable_json_is_reported_without_crashing(self) -> None:
        self.game.title("c_praha").write_text("{ not json", encoding="utf-8")
        self.game.assert_one_issue_mentioning(self, "could not be read as JSON")

    def test_a_tier_that_disagrees_with_the_id_prefix_is_reported(self) -> None:
        # A wrong tier cascades: the liege check, the capital check, and the succession law
        # check all fail downstream of it. That cascade is correct, so this test pins only the
        # one message that names the cause.
        self.game.edit(self.game.title("c_praha"), tier="duchy")
        self.game.assert_one_issue_mentioning(self, "id prefix")


class TestReferences(FixtureTestCase):
    def test_a_reference_to_a_missing_record_is_reported(self) -> None:
        self.game.edit(self.game.title("c_praha"), holder="nobody")
        self.game.assert_one_issue_mentioning(self, "no character called 'nobody'")

    def test_a_liege_at_the_wrong_tier_is_reported(self) -> None:
        self.game.edit(self.game.title("b_praha"), de_jure_liege="d_bohemia")
        self.game.assert_one_issue_mentioning(self, "one tier above")

    def test_a_cycle_in_the_hierarchy_is_reported(self) -> None:
        self.game.edit(self.game.title("d_bohemia"), de_jure_liege="c_praha")
        self.game.assert_one_issue_mentioning(self, "cycle")

    def test_a_capital_outside_its_own_title_is_reported(self) -> None:
        write(
            self.game.title("c_kourim"),
            {
                "id": "c_kourim",
                "tier": "county",
                "name": "Kourim",
                "de_jure_liege": "d_bohemia",
                "capital": "b_praha",
                "holder": "vratislav_ii",
                "sources": ["x"],
            },
        )
        self.game.assert_one_issue_mentioning(self, "capital")


class TestSuccessionLawPlacement(FixtureTestCase):
    def test_a_duchy_without_a_succession_law_is_reported(self) -> None:
        self.game.edit(self.game.title("d_bohemia"), succession_law=None)
        self.game.assert_one_issue_mentioning(self, "succession law")

    def test_a_county_with_its_own_succession_law_is_reported(self) -> None:
        self.game.edit(self.game.title("c_praha"), succession_law="agnatic_seniority")
        self.game.assert_one_issue_mentioning(self, "succession law")


class TestPeopleAndTime(FixtureTestCase):
    def test_a_holder_who_dies_before_the_start_date_is_reported(self) -> None:
        self.game.edit(self.game.character("vratislav_ii"), death="1060-01-01")
        issues = self.game.issues()
        self.assertTrue(any("was already dead" in issue for issue in issues), issues)

    def test_a_holder_not_yet_born_is_reported(self) -> None:
        self.game.edit(self.game.character("vratislav_ii"), birth="1080-01-01")
        issues = self.game.issues()
        self.assertTrue(any("not yet born" in issue for issue in issues), issues)

    def test_a_death_before_a_birth_is_reported(self) -> None:
        self.game.edit(self.game.character("vratislav_ii"), birth="1090-01-01", death="1089-01-01")
        issues = self.game.issues()
        self.assertTrue(any("dies before" in issue for issue in issues), issues)

    def test_a_father_recorded_as_female_is_reported(self) -> None:
        write(
            self.game.character("judith"),
            {
                "id": "judith",
                "name": "Judith",
                "sex": "female",
                "birth": "1000-01-01",
                "sources": ["x"],
            },
        )
        self.game.edit(self.game.character("vratislav_ii"), father="judith")
        self.game.assert_one_issue_mentioning(self, "father")

    def test_a_parent_too_young_to_be_one_is_reported(self) -> None:
        write(
            self.game.character("bretislav"),
            {
                "id": "bretislav",
                "name": "Bretislav",
                "sex": "male",
                "birth": "1025-01-01",
                "sources": ["x"],
            },
        )
        self.game.edit(self.game.character("vratislav_ii"), father="bretislav")
        self.game.assert_one_issue_mentioning(self, "years older")

    def test_a_one_sided_marriage_is_reported(self) -> None:
        write(
            self.game.character("svatava"),
            {
                "id": "svatava",
                "name": "Svatava",
                "sex": "female",
                "birth": "1046-01-01",
                "sources": ["x"],
            },
        )
        self.game.edit(self.game.character("vratislav_ii"), spouses=["svatava"])
        self.game.assert_one_issue_mentioning(self, "does not list")


class TestHoldings(FixtureTestCase):
    def test_a_barony_title_without_a_holding_record_is_reported(self) -> None:
        (self.game.root / "content" / "holdings" / "b_praha.json").unlink()
        self.game.assert_one_issue_mentioning(self, "no holding record")

    def test_a_holding_without_a_barony_title_is_reported(self) -> None:
        write(
            self.game.root / "content" / "holdings" / "b_vysehrad.json",
            {"id": "b_vysehrad", "county": "c_praha", "type": "castle", "sources": ["x"]},
        )
        self.game.assert_one_issue_mentioning(self, "no barony title")


class TestSources(FixtureTestCase):
    def test_a_title_without_a_source_is_reported(self) -> None:
        self.game.edit(self.game.title("c_praha"), sources=[])
        self.game.assert_one_issue_mentioning(self, "source")


if __name__ == "__main__":
    unittest.main()
