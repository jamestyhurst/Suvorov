import unittest

from suvorov_core import Date, Person, World, biological_age


class WorldTests(unittest.TestCase):
    def aurora(self):
        world = World(2024, 1, 1)
        polity = world.add_polity("Aurora")
        second = world.add_polity("Helia")
        vale = world.add_location("Amber Vale")
        ridge = world.add_location("Glass Ridge")
        return world, polity, second, vale, ridge

    def test_names_and_person(self):
        world, polity, second, vale, ridge = self.aurora()
        self.assertEqual(world.polity_name(polity), "Aurora")
        self.assertNotEqual(second, polity)
        self.assertEqual(world.location_name(vale), "Amber Vale")

        calen = Person(
            names=["Calen", "of Vale"],
            allegiances=[polity, second],
            birth_date=Date(2000, 3, 1),
            birth_location_id=vale,
            current_location_id=vale,
        )
        person_id = world.add_person(calen)
        stored = world.person(person_id)
        self.assertEqual(stored.names, ["Calen", "of Vale"])
        self.assertEqual(stored.allegiances, [polity, second])
        self.assertEqual(world.person_age(person_id), 23)

        mira = Person(
            names=["Mira"],
            allegiances=[polity],
            birth_date=Date(2000, 3, 1),
            birth_location_id=vale,
            current_location_id=ridge,
        )
        mira_id = world.add_person(mira)
        self.assertNotEqual(mira_id, person_id)
        world.set_person_location(person_id, ridge)
        self.assertEqual(world.person(person_id).current_location_id, ridge)
        self.assertEqual(world.person(person_id).birth_location_id, vale)

        world.advance_one_day()
        self.assertEqual(world.date(), Date(2024, 1, 2))
        self.assertEqual(world.person_age(person_id), 23)

    def test_birthday_leap_rollover(self):
        world = World(2024, 2, 29)
        polity = world.add_polity("Aurora")
        place = world.add_location("Amber Vale")
        pid = world.add_person(
            Person(["Calen"], [polity], Date(2000, 3, 1), place, place)
        )
        self.assertEqual(world.person_age(pid), 23)
        world.advance_one_day()
        self.assertEqual(world.date(), Date(2024, 3, 1))
        self.assertEqual(world.person_age(pid), 24)

    def test_infant(self):
        world = World(2024, 6, 15)
        polity = world.add_polity("Helia")
        place = world.add_location("Glass Ridge")
        pid = world.add_person(Person(["Ryn"], [polity], Date(2024, 6, 15), place, place))
        self.assertEqual(world.person_age(pid), 0)

    def test_calendar_edges(self):
        end = World(2024, 12, 31)
        end.advance_one_day()
        self.assertEqual(end.date(), Date(2025, 1, 1))

        leap = World(2024, 2, 28)
        leap.advance_one_day()
        self.assertEqual(leap.date(), Date(2024, 2, 29))
        leap.advance_one_day()
        self.assertEqual(leap.date(), Date(2024, 3, 1))

        non_leap = World(2023, 2, 28)
        non_leap.advance_one_day()
        self.assertEqual(non_leap.date(), Date(2023, 3, 1))

        self.assertEqual(biological_age(Date(2000, 2, 29), Date(2023, 2, 28)), 22)
        self.assertEqual(biological_age(Date(2000, 2, 29), Date(2023, 3, 1)), 23)

    def test_rejections(self):
        world, polity, _, vale, _ = self.aurora()
        with self.assertRaises(ValueError):
            World(2024, 2, 30)
        with self.assertRaises(ValueError):
            world.add_polity("")
        with self.assertRaises(LookupError):
            world.polity_name(99)
        with self.assertRaises(ValueError):
            world.add_location("")
        with self.assertRaises(LookupError):
            world.location_name(99)

        mira = Person(["Mira"], [polity], Date(2000, 3, 1), vale, vale)
        pid = world.add_person(Person(["Mira"], [polity], Date(2000, 3, 1), vale, vale))

        bad = Person([], [polity], Date(2000, 3, 1), vale, vale)
        with self.assertRaises(ValueError):
            world.add_person(bad)
        with self.assertRaises(ValueError):
            world.add_person(Person([""], [polity], Date(2000, 3, 1), vale, vale))
        with self.assertRaises(ValueError):
            world.add_person(Person(["Mira"], [], Date(2000, 3, 1), vale, vale))
        with self.assertRaises(LookupError):
            world.add_person(Person(["Mira"], [99], Date(2000, 3, 1), vale, vale))
        with self.assertRaises(ValueError):
            world.add_person(Person(["Mira"], [polity], Date(2024, 2, 30), vale, vale))
        with self.assertRaises(ValueError):
            world.add_person(Person(["Mira"], [polity], Date(2025, 1, 1), vale, vale))
        with self.assertRaises(LookupError):
            world.add_person(Person(["Mira"], [polity], Date(2000, 3, 1), 99, vale))
        with self.assertRaises(LookupError):
            world.person(99)
        with self.assertRaises(LookupError):
            world.set_person_location(99, vale)
        with self.assertRaises(LookupError):
            world.set_person_location(pid, 99)
        self.assertEqual(mira.names[0], "Mira")


if __name__ == "__main__":
    unittest.main()
