"""Python port of the PR #2 World API for the language spike.

Not the engine. Exists so the same tests and bench can run in three languages.
"""

from __future__ import annotations

from dataclasses import dataclass, field


def _is_leap_year(year: int) -> bool:
    return (year % 4 == 0 and year % 100 != 0) or year % 400 == 0


def _days_in_month(year: int, month: int) -> int:
    if month == 2:
        return 29 if _is_leap_year(year) else 28
    if month in (4, 6, 9, 11):
        return 30
    return 31


@dataclass(order=True, frozen=False)
class Date:
    year: int
    month: int
    day: int

    def valid(self) -> bool:
        if self.month < 1 or self.month > 12 or self.day < 1:
            return False
        return self.day <= _days_in_month(self.year, self.month)

    def advance_one_day(self) -> None:
        self.day += 1
        if self.day <= _days_in_month(self.year, self.month):
            return
        self.day = 1
        self.month += 1
        if self.month <= 12:
            return
        self.month = 1
        self.year += 1


def biological_age(birth: Date, on: Date) -> int:
    if not birth.valid() or not on.valid():
        raise ValueError("Age date is invalid")
    if (on.year, on.month, on.day) < (birth.year, birth.month, birth.day):
        raise ValueError("Birth date is after the age date")
    years = on.year - birth.year
    if on.month < birth.month or (on.month == birth.month and on.day < birth.day):
        years -= 1
    return years


@dataclass
class Person:
    names: list[str] = field(default_factory=list)
    allegiances: list[int] = field(default_factory=list)
    birth_date: Date = field(default_factory=lambda: Date(1, 1, 1))
    birth_location_id: int = 0
    current_location_id: int = 0


class World:
    def __init__(self, year: int, month: int, day: int) -> None:
        self._date = Date(year, month, day)
        if not self._date.valid():
            raise ValueError("World date is invalid")
        self._polities: list[str] = []
        self._locations: list[str] = []
        self._persons: list[Person] = []

    def add_polity(self, name: str) -> int:
        if name == "":
            raise ValueError("Polity name cannot be empty")
        self._polities.append(name)
        return len(self._polities) - 1

    def polity_name(self, polity_id: int) -> str:
        if polity_id < 0 or polity_id >= len(self._polities):
            raise LookupError("Polity id does not exist")
        return self._polities[polity_id]

    def add_location(self, name: str) -> int:
        if name == "":
            raise ValueError("Location name cannot be empty")
        self._locations.append(name)
        return len(self._locations) - 1

    def location_name(self, location_id: int) -> str:
        if location_id < 0 or location_id >= len(self._locations):
            raise LookupError("Location id does not exist")
        return self._locations[location_id]

    def add_person(self, person: Person) -> int:
        if not person.names:
            raise ValueError("Person must have at least one name")
        if any(name == "" for name in person.names):
            raise ValueError("Person name cannot be empty")
        if not person.allegiances:
            raise ValueError("Person must have at least one allegiance")
        for allegiance in person.allegiances:
            if allegiance < 0 or allegiance >= len(self._polities):
                raise LookupError("Polity id does not exist")
        if not person.birth_date.valid():
            raise ValueError("Person birth date is invalid")
        if (self._date.year, self._date.month, self._date.day) < (
            person.birth_date.year,
            person.birth_date.month,
            person.birth_date.day,
        ):
            raise ValueError("Person birth date is after the world date")
        if (
            person.birth_location_id < 0
            or person.birth_location_id >= len(self._locations)
            or person.current_location_id < 0
            or person.current_location_id >= len(self._locations)
        ):
            raise LookupError("Location id does not exist")
        self._persons.append(person)
        return len(self._persons) - 1

    def person(self, person_id: int) -> Person:
        if person_id < 0 or person_id >= len(self._persons):
            raise LookupError("Person id does not exist")
        return self._persons[person_id]

    def person_age(self, person_id: int) -> int:
        return biological_age(self.person(person_id).birth_date, self._date)

    def set_person_location(self, person_id: int, location_id: int) -> None:
        if person_id < 0 or person_id >= len(self._persons):
            raise LookupError("Person id does not exist")
        if location_id < 0 or location_id >= len(self._locations):
            raise LookupError("Location id does not exist")
        self._persons[person_id].current_location_id = location_id

    def advance_one_day(self) -> None:
        self._date.advance_one_day()

    def date(self) -> Date:
        return self._date
