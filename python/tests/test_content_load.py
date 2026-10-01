"""Load Premyslid-shaped records without using the Premyslid setting."""

import unittest

from suvorov.load import world_from_records


def aurora_records():
    titles = [
        {
            "id": "d_aurora",
            "tier": "duchy",
            "name": "Aurora",
            "holder": "calen",
            "capital": "c_vale",
        },
        {
            "id": "c_vale",
            "tier": "county",
            "name": "Amber Vale",
            "holder": "calen",
            "de_jure_liege": "d_aurora",
        },
    ]
    characters = [
        {
            "id": "calen",
            "name": "Calen",
            "sex": "male",
            "birth": "0980-03-01",
            "death": None,
        },
        {
            "id": "mira",
            "name": "Mira",
            "sex": "female",
            "birth": "0988-06-15",
            "death": None,
        },
        {
            "id": "old_ryn",
            "name": "Ryn",
            "sex": "male",
            "birth": "0940-01-01",
            "death": "0999-12-31",
        },
    ]
    return titles, characters


class ContentLoadTests(unittest.TestCase):
    def test_holders_become_persons_with_title_allegiance(self):
        titles, characters = aurora_records()
        loaded = world_from_records("1000-01-01", titles, characters)
        self.assertEqual(loaded.skipped_dead, ["old_ryn"])
        self.assertIn("calen", loaded.person_ids)
        self.assertIn("mira", loaded.person_ids)
        self.assertNotIn("old_ryn", loaded.person_ids)

        calen_id = loaded.person_ids["calen"]
        mira_id = loaded.person_ids["mira"]
        calen = loaded.world.person(calen_id)
        self.assertEqual(calen.names, ["Calen"])
        self.assertEqual(len(calen.allegiances), 2)
        self.assertEqual(loaded.world.person_age(calen_id), 19)
        self.assertEqual(loaded.world.person_age(mira_id), 11)

        mira = loaded.world.person(mira_id)
        self.assertEqual(
            mira.allegiances,
            [loaded.polity_ids["unlanded"]],
        )

    def test_tick_after_load(self):
        titles, characters = aurora_records()
        loaded = world_from_records("1000-02-28", titles, characters)
        loaded.world.advance_one_day()
        # 1000 is not a Gregorian leap year (divisible by 100, not 400).
        self.assertEqual(loaded.world.date().year, 1000)
        self.assertEqual(loaded.world.date().month, 3)
        self.assertEqual(loaded.world.date().day, 1)


if __name__ == "__main__":
    unittest.main()
