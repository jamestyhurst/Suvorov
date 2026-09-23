"""Tests for the work queue: what ``next`` hands out, and in what order.

The gap engine is the piece that makes "go work on the Premyslid clone" a complete
instruction. It compares the plan against the content on disk and reports what is missing.
Two properties matter more than any individual rule:

* It never runs dry while work remains, because a model that is handed nothing invents work.
* It refuses to hand out bulk map work before the engine exists, because content written
  against an unproven schema is the rework this project is trying to avoid.
"""

from __future__ import annotations

import unittest

from suvorov.tools import content, gaps
from tests.premyslid_fixture import FixtureTestCase, write


class GapTestCase(FixtureTestCase):
    def gaps(self) -> list[gaps.Gap]:
        return gaps.compute_gaps(content.load_game(self.game.root))

    def kinds(self) -> list[str]:
        return [gap.kind for gap in self.gaps()]

    def targets(self) -> list[str]:
        return [gap.target for gap in self.gaps()]


class TestExemplarFirst(GapTestCase):
    def test_a_missing_county_in_the_exemplar_duchy_is_offered(self) -> None:
        self.game.edit(self.game.plan(), hierarchy=[
            {
                "id": "k_bohemia",
                "tier": "kingdom",
                "priority": 1,
                "duchies": [
                    {"id": "d_bohemia", "priority": 1, "counties": ["c_praha", "c_kourim"]},
                    {"id": "d_moravia", "priority": 2, "counties": ["c_brno"]},
                ],
            }
        ])
        self.assertIn("c_kourim", self.targets())

    def test_work_outside_the_exemplar_duchy_is_withheld_while_bulk_is_locked(self) -> None:
        self.assertNotIn("c_brno", self.targets())
        self.assertNotIn("d_moravia", self.targets())

    def test_unlocking_bulk_releases_the_other_duchies(self) -> None:
        self.game.edit(self.game.plan(), bulk_unlocked=True)
        self.assertIn("d_moravia", self.targets())

    def test_a_missing_holding_is_offered_when_the_county_is_short(self) -> None:
        self.game.edit(self.game.plan(), holdings_per_county=2)
        holding_gaps = [gap for gap in self.gaps() if gap.kind == "holding"]
        self.assertEqual(len(holding_gaps), 1)
        self.assertEqual(holding_gaps[0].target, "c_praha")

    def test_a_county_without_a_holder_is_offered(self) -> None:
        self.game.edit(self.game.title("c_praha"), holder=None)
        holder_gaps = [gap for gap in self.gaps() if gap.kind == "holder"]
        self.assertEqual([gap.target for gap in holder_gaps], ["c_praha"])


class TestOrdering(GapTestCase):
    def test_content_gaps_outrank_research_gaps(self) -> None:
        self.game.edit(self.game.plan(), holdings_per_county=2)
        ordered = self.gaps()
        first_content = next(index for index, gap in enumerate(ordered) if gap.kind == "holding")
        first_research = next(index for index, gap in enumerate(ordered) if gap.kind == "research")
        self.assertLess(first_content, first_research)

    def test_gaps_are_returned_in_priority_order(self) -> None:
        ordered = self.gaps()
        self.assertEqual([gap.priority for gap in ordered], sorted(gap.priority for gap in ordered))


class TestNeverRunsDry(GapTestCase):
    def test_a_complete_exemplar_duchy_still_yields_research_work(self) -> None:
        remaining = self.gaps()
        self.assertTrue(remaining, "the queue must never be empty while the engine is missing")
        self.assertTrue(all(gap.kind == "research" for gap in remaining), self.kinds())

    def test_a_written_research_note_is_not_offered_again(self) -> None:
        before = [gap for gap in self.gaps() if gap.kind == "research"]
        target = before[0].target
        (self.game.root / "research" / f"{target}.md").write_text("# note\n", encoding="utf-8")
        after = [gap.target for gap in self.gaps() if gap.kind == "research"]
        self.assertNotIn(target, after)


class TestEngineMilestone(GapTestCase):
    def test_the_engine_is_reported_as_due_when_the_exemplar_duchy_is_complete(self) -> None:
        game = content.load_game(self.game.root)
        self.assertTrue(gaps.exemplar_is_complete(game))

    def test_the_engine_is_not_due_while_the_exemplar_duchy_is_unfinished(self) -> None:
        self.game.edit(self.game.title("c_praha"), holder=None)
        game = content.load_game(self.game.root)
        self.assertFalse(gaps.exemplar_is_complete(game))


class TestTaskRendering(GapTestCase):
    def test_a_task_names_the_file_to_write_and_the_schema_to_follow(self) -> None:
        self.game.edit(self.game.plan(), holdings_per_county=2)
        holding_gap = next(gap for gap in self.gaps() if gap.kind == "holding")
        rendered = holding_gap.render()
        self.assertIn("games/premyslid/content/holdings/", rendered)
        self.assertIn("games/premyslid/schema/holding.schema.json", rendered)
        self.assertIn(holding_gap.task_id, rendered)

    def test_a_task_id_is_stable_across_runs(self) -> None:
        first = {gap.task_id for gap in self.gaps()}
        second = {gap.task_id for gap in self.gaps()}
        self.assertEqual(first, second)


class TestChoosing(GapTestCase):
    def test_a_choice_comes_from_the_highest_priority_band(self) -> None:
        self.game.edit(self.game.plan(), holdings_per_county=4)
        ordered = self.gaps()
        top = {gap.task_id for gap in ordered[: gaps.CHOICE_WINDOW]}
        for seed in range(25):
            self.assertIn(gaps.choose(ordered, seed=seed).task_id, top)

    def test_choosing_from_an_empty_queue_returns_nothing(self) -> None:
        self.assertIsNone(gaps.choose([], seed=0))


if __name__ == "__main__":
    unittest.main()
