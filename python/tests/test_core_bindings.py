import unittest

from suvorov.core import Date, Person, World, biological_age


class BindingTests(unittest.TestCase):
    def test_person_and_tick(self):
        world = World(2024, 1, 1)
        aurora = world.add_polity("Aurora")
        vale = world.add_location("Amber Vale")
        person_id = world.add_person(
            Person(["Calen"], [aurora], Date(2000, 3, 1), vale, vale)
        )
        self.assertEqual(world.person_age(person_id), 23)
        self.assertEqual(world.person(person_id).names, ["Calen"])
        world.advance_one_day()
        self.assertEqual(world.date(), Date(2024, 1, 2))

    def test_rejects_bad_date(self):
        with self.assertRaises(ValueError):
            World(2024, 2, 30)

    def test_rejects_unknown_id(self):
        world = World(2024, 1, 1)
        with self.assertRaises(LookupError):
            world.polity_name(9)

    def test_biological_age(self):
        self.assertEqual(biological_age(Date(2000, 2, 29), Date(2023, 2, 28)), 22)
        self.assertEqual(biological_age(Date(2000, 2, 29), Date(2023, 3, 1)), 23)


if __name__ == "__main__":
    unittest.main()
