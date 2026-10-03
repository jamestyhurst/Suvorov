"""Load schema-shaped game records into a Rust World.

The record shape matches Premyslid schema v0 (character + title JSON) so that
PR #3 tools can hand validated content to the engine later. This module does
not read `games/premyslid/` and does not embed a real setting. Tests use
fictional Aurora/Helia records.

Mapping
-------
- Each title becomes a named location (the title id).
- Each title also becomes a polity (the title id). A holder's allegiance is
  every title they hold at start. Characters who hold nothing get a required
  allegiance to a polity named ``unlanded``.
- Dead characters (death date on or before start) are omitted here. The
  compiled Rust loader (`suvorov_core.world_from_records`) keeps them and
  records the historical death date. PyO3 does not export that path yet.
- Birth and current location: the holder's first title, else ``unlocated``.
"""

from __future__ import annotations

from dataclasses import dataclass, field

from suvorov.core import Date, Person, World


def parse_iso_date(text: str) -> Date:
    year_s, month_s, day_s = text.split("-")
    date = Date(int(year_s), int(month_s), int(day_s))
    if not date.valid():
        raise ValueError(f"invalid date {text}")
    return date


def _date_tuple(date: Date) -> tuple[int, int, int]:
    return (date.year, date.month, date.day)


@dataclass
class LoadedWorld:
    world: World
    person_ids: dict[str, int] = field(default_factory=dict)
    polity_ids: dict[str, int] = field(default_factory=dict)
    location_ids: dict[str, int] = field(default_factory=dict)
    skipped_dead: list[str] = field(default_factory=list)


def world_from_records(
    start_date: str,
    titles: list[dict],
    characters: list[dict],
) -> LoadedWorld:
    """Build a World from title and character dicts.

    ``titles`` and ``characters`` are already-parsed records. Validation
    belongs to Premyslid's tools, not to the engine.
    """
    start = parse_iso_date(start_date)
    world = World(start.year, start.month, start.day)
    loaded = LoadedWorld(world=world)

    loaded.polity_ids["unlanded"] = world.add_polity("unlanded")
    loaded.location_ids["unlocated"] = world.add_location("unlocated")

    holders: dict[str, list[str]] = {}
    for title in titles:
        title_id = title["id"]
        loaded.polity_ids[title_id] = world.add_polity(title_id)
        loaded.location_ids[title_id] = world.add_location(title_id)
        holder = title.get("holder")
        if holder:
            holders.setdefault(holder, []).append(title_id)

    for character in characters:
        char_id = character["id"]
        death = character.get("death")
        if death:
            if _date_tuple(parse_iso_date(death)) <= _date_tuple(start):
                loaded.skipped_dead.append(char_id)
                continue

        held = holders.get(char_id, [])
        if held:
            allegiances = [loaded.polity_ids[title_id] for title_id in held]
            place = loaded.location_ids[held[0]]
        else:
            allegiances = [loaded.polity_ids["unlanded"]]
            place = loaded.location_ids["unlocated"]

        person = Person(
            names=[character["name"]],
            allegiances=allegiances,
            birth_date=parse_iso_date(character["birth"]),
            birth_location_id=place,
            current_location_id=place,
        )
        loaded.person_ids[char_id] = world.add_person(person)

    return loaded
