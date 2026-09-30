import argparse
import time

from suvorov_core import Date, Person, World


def clock_only(days: int) -> Date:
    world = World(1000, 1, 1)
    for _ in range(days):
        world.advance_one_day()
    return world.date()


def scenario(persons: int, locations: int, polities: int, days: int):
    world = World(1000, 1, 1)
    polity_ids = [world.add_polity(f"Polity{i}") for i in range(polities)]
    location_ids = [world.add_location(f"Place{i}") for i in range(locations)]
    for i in range(persons):
        world.add_person(
            Person(
                names=[f"Person{i}"],
                allegiances=[polity_ids[i % polities]],
                birth_date=Date(950 + (i % 50), 3, 1),
                birth_location_id=location_ids[i % locations],
                current_location_id=location_ids[i % locations],
            )
        )
    checksum = 0
    for day in range(days):
        world.advance_one_day()
        for pid in range(persons):
            checksum += world.person_age(pid)
            checksum += world.person(pid).allegiances[0]
        if persons:
            mover = day % persons
            dest = location_ids[(day + 1) % locations]
            world.set_person_location(mover, dest)
    return checksum, world.date()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=["clock", "scan"], default="scan", nargs="?")
    parser.add_argument("args", nargs="*", type=int)
    ns = parser.parse_args()
    if ns.mode == "clock":
        days = ns.args[0] if ns.args else 1_000_000
        start = time.perf_counter()
        date = clock_only(days)
        ms = (time.perf_counter() - start) * 1000.0
        print(f"lang=python scenario=clock days={days} ms={ms:.3f} date={date.year}-{date.month}-{date.day}")
        return
    persons = ns.args[0] if len(ns.args) > 0 else 20_000
    locations = ns.args[1] if len(ns.args) > 1 else 200
    polities = ns.args[2] if len(ns.args) > 2 else 50
    days = ns.args[3] if len(ns.args) > 3 else 365
    start = time.perf_counter()
    checksum, date = scenario(persons, locations, polities, days)
    ms = (time.perf_counter() - start) * 1000.0
    print(
        f"lang=python scenario=scan persons={persons} locations={locations} "
        f"polities={polities} days={days} ms={ms:.3f} checksum={checksum} "
        f"date={date.year}-{date.month}-{date.day}"
    )


if __name__ == "__main__":
    main()
