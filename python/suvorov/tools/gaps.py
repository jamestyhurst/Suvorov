"""Working out what is missing, so that "go work on the game" is a complete instruction.

The queue is computed from the plan and the content on disk rather than stored in a file.
Nothing has to be maintained, nothing goes stale, and a finished task disappears from the
queue the moment its file is merged.

Two properties are deliberate:

* While the engine does not exist, only the exemplar duchy and its research are offered. Bulk
  map work against an unproven schema is the rework this project exists to avoid.
* The queue never empties while the engine is missing: research tasks remain once content
  work runs out. A model handed nothing invents work, and invented work is the expensive kind.
"""

from __future__ import annotations

import random
from dataclasses import dataclass, field

from suvorov.tools.content import Game

# The repository path a task refers to. Deliberately a constant rather than derived from the
# game root: tasks are instructions to an agent working in a checkout, not paths on this disk.
GAME_PREFIX = "games/premyslid"

# How many of the highest-priority gaps a random choice may come from. Wide enough that two
# agents working at once rarely collide, narrow enough that priority still means something.
CHOICE_WINDOW = 10

PRIORITY_DUCHY_TITLE = 10
PRIORITY_COUNTY_TITLE = 20
PRIORITY_HOLDER = 30
PRIORITY_CAPITAL = 40
PRIORITY_HOLDING = 50
PRIORITY_EVENT = 70
PRIORITY_RESEARCH = 80

# Added once per priority step of a non-exemplar duchy, so that unlocked bulk work always
# sorts below anything still outstanding in the exemplar duchy.
BULK_PENALTY = 100


@dataclass(frozen=True)
class Gap:
    """One piece of missing content or research, phrased as a task."""

    priority: int
    kind: str
    target: str
    task_id: str
    file: str
    schema: str | None
    what: str
    rules: list[str] = field(default_factory=list)

    def render(self) -> str:
        lines = [
            f"TASK {self.task_id}",
            f"kind:   {self.kind}",
            f"target: {self.target}",
            f"file:   {self.file}",
        ]
        if self.schema:
            lines.append(f"schema: {self.schema}")
        lines.append("")
        lines.append(self.what)
        if self.rules:
            lines.append("")
            lines.append("Rules:")
            lines.extend(f"  - {rule}" for rule in self.rules)
        return "\n".join(lines)


def compute_gaps(game: Game) -> list[Gap]:
    """Every outstanding task, most important first."""

    exemplar = game.plan.get("exemplar_duchy")
    bulk_unlocked = bool(game.plan.get("bulk_unlocked"))

    collected: list[Gap] = []
    for duchy in game.planned_duchies:
        is_exemplar = duchy["id"] == exemplar
        if not is_exemplar and not bulk_unlocked:
            continue
        penalty = 0 if is_exemplar else BULK_PENALTY * int(duchy.get("priority", 1))
        collected.extend(_duchy_gaps(game, duchy, penalty, is_exemplar))

    return sorted(collected, key=lambda gap: (gap.priority, gap.task_id))


def exemplar_is_complete(game: Game) -> bool:
    """True when the exemplar duchy has no outstanding content work.

    Research and exemplar events are excluded on purpose: they are useful work, but they are
    not evidence that the schema holds up, which is what the engine milestone turns on.
    """

    exemplar = game.plan.get("exemplar_duchy")
    for duchy in game.planned_duchies:
        if duchy["id"] != exemplar:
            continue
        content_gaps = [
            gap
            for gap in _duchy_gaps(game, duchy, 0, True)
            if gap.kind not in ("research", "event")
        ]
        return not content_gaps
    return False


def choose(candidates: list[Gap], *, seed: int | None = None) -> Gap | None:
    """Pick one gap at random from the highest-priority band.

    Random rather than strictly first, so that two sessions starting at the same time usually
    pick different work. No claim file, no lock, no coordination state to go stale: when a
    collision does happen the loser's pull request is an empty diff, and the work was free.
    """

    if not candidates:
        return None
    return random.Random(seed).choice(candidates[:CHOICE_WINDOW])


def _duchy_gaps(game: Game, duchy: dict, penalty: int, is_exemplar: bool) -> list[Gap]:
    duchy_id = duchy["id"]
    collected: list[Gap] = []
    titles = game.of_kind("titles")

    if duchy_id not in titles:
        return [
            Gap(
                priority=PRIORITY_DUCHY_TITLE + penalty,
                kind="title",
                target=duchy_id,
                task_id=f"title-{duchy_id}",
                file=f"{GAME_PREFIX}/content/titles/{duchy_id}.json",
                schema=f"{GAME_PREFIX}/schema/title.schema.json",
                what=(
                    f"Create the duchy title '{duchy_id}'. It is the top of this region's "
                    f"hierarchy, so set de_jure_liege to null unless its kingdom title already "
                    f"exists. Give it a succession law, a holder who is alive on the scenario "
                    f"start date, and a capital county."
                ),
                rules=[
                    "Every duchy and above must name a succession law.",
                    "The capital must be a county whose de_jure_liege is this duchy.",
                    "Cite at least one public source.",
                ],
            )
        ]

    duchy_record = titles[duchy_id]
    if duchy_record.data.get("holder") is None:
        collected.append(_holder_gap(duchy_id, "duchy", penalty))
    if duchy_record.data.get("capital") is None:
        collected.append(_capital_gap(duchy_id, "duchy", "county", penalty))

    for county_id in duchy.get("counties", []):
        collected.extend(_county_gaps(game, duchy_id, county_id, penalty))

    if is_exemplar:
        collected.extend(_event_gaps(game))

    collected.extend(_research_gaps(game, duchy_id, duchy.get("counties", []), penalty))
    return collected


def _county_gaps(game: Game, duchy_id: str, county_id: str, penalty: int) -> list[Gap]:
    titles = game.of_kind("titles")
    if county_id not in titles:
        return [
            Gap(
                priority=PRIORITY_COUNTY_TITLE + penalty,
                kind="title",
                target=county_id,
                task_id=f"title-{county_id}",
                file=f"{GAME_PREFIX}/content/titles/{county_id}.json",
                schema=f"{GAME_PREFIX}/schema/title.schema.json",
                what=(
                    f"Create the county title '{county_id}' inside the duchy '{duchy_id}'. "
                    f"Set de_jure_liege to '{duchy_id}', leave succession_law null, and give it "
                    f"a holder who is alive on the scenario start date."
                ),
                rules=[
                    f"de_jure_liege must be '{duchy_id}'.",
                    "A county must not carry its own succession law in version 0.",
                    "The capital is a barony inside this county; it may be added later.",
                    "Cite at least one public source.",
                ],
            )
        ]

    collected: list[Gap] = []
    record = titles[county_id]
    if record.data.get("holder") is None:
        collected.append(_holder_gap(county_id, "county", penalty))
    if record.data.get("capital") is None:
        collected.append(_capital_gap(county_id, "county", "barony", penalty))

    wanted = int(game.plan.get("holdings_per_county", 0))
    present = sum(
        1 for holding in game.of_kind("holdings").values() if holding.data.get("county") == county_id
    )
    for index in range(present, wanted):
        collected.append(
            Gap(
                priority=PRIORITY_HOLDING + penalty,
                kind="holding",
                target=county_id,
                task_id=f"holding-{county_id}-{index + 1}",
                file=f"{GAME_PREFIX}/content/holdings/<barony id>.json",
                schema=f"{GAME_PREFIX}/schema/holding.schema.json",
                what=(
                    f"Add holding {index + 1} of {wanted} to the county '{county_id}'. This "
                    f"means two files: a barony title in content/titles/ whose de_jure_liege is "
                    f"'{county_id}', and a holding record in content/holdings/ with the same id. "
                    f"Pick a real settlement in that region and name the files after it."
                ),
                rules=[
                    "The barony title and the holding record must share one id, starting with b_.",
                    "A holding is a castle, a city, or a temple.",
                    "Cite at least one public source on both files.",
                ],
            )
        )
    return collected


def _holder_gap(title_id: str, tier: str, penalty: int) -> Gap:
    return Gap(
        priority=PRIORITY_HOLDER + penalty,
        kind="holder",
        target=title_id,
        task_id=f"holder-{title_id}",
        file=f"{GAME_PREFIX}/content/titles/{title_id}.json",
        schema=f"{GAME_PREFIX}/schema/character.schema.json",
        what=(
            f"The {tier} '{title_id}' has no holder. Identify who held it on the scenario start "
            f"date, create that character in content/characters/ if they do not exist yet, and "
            f"set them as the holder."
        ),
        rules=[
            "The holder must be born on or before the start date and must not have died by then.",
            "A character needs a birth date; a death date is optional if it is unknown.",
            "Cite at least one public source.",
        ],
    )


def _capital_gap(title_id: str, tier: str, expected_tier: str, penalty: int) -> Gap:
    return Gap(
        priority=PRIORITY_CAPITAL + penalty,
        kind="capital",
        target=title_id,
        task_id=f"capital-{title_id}",
        file=f"{GAME_PREFIX}/content/titles/{title_id}.json",
        schema=f"{GAME_PREFIX}/schema/title.schema.json",
        what=(
            f"The {tier} '{title_id}' has no capital. Set it to the {expected_tier} that held "
            f"the seat of power, and make sure that {expected_tier}'s de_jure_liege points back "
            f"at '{title_id}'."
        ),
        rules=[f"The capital of a {tier} must be a {expected_tier} that belongs to it."],
    )


def _event_gaps(game: Game) -> list[Gap]:
    budget = int(game.plan.get("exemplar_event_budget", 0))
    present = len(game.of_kind("events"))
    collected: list[Gap] = []
    for index in range(present, budget):
        collected.append(
            Gap(
                priority=PRIORITY_EVENT,
                kind="event",
                target=f"exemplar-{index + 1}",
                task_id=f"event-{index + 1}",
                file=f"{GAME_PREFIX}/content/events/<event id>.json",
                schema=f"{GAME_PREFIX}/schema/event.schema.json",
                what=(
                    f"Write exemplar event {index + 1} of {budget}. These exist to show the "
                    f"shape the engine will execute, not to build an event library. Use only "
                    f"the triggers and effects the schema allows."
                ),
                rules=[
                    "Effects are limited to grant_title, kill_character, and change_opinion.",
                    "Triggers are limited to on_death, on_birth, and on_succession.",
                    "Do not invent new effect names; the vocabulary is frozen until an engine exists.",
                ],
            )
        )
    return collected


def _research_gaps(game: Game, duchy_id: str, counties: list[str], penalty: int) -> list[Gap]:
    collected: list[Gap] = []
    for target in [duchy_id, *counties]:
        if (game.root / "research" / f"{target}.md").exists():
            continue
        collected.append(
            Gap(
                priority=PRIORITY_RESEARCH + penalty,
                kind="research",
                target=target,
                task_id=f"research-{target}",
                file=f"{GAME_PREFIX}/research/{target}.md",
                schema=None,
                what=(
                    f"Write a research note on '{target}' as it stood around the scenario start "
                    f"date: who ruled it, what it was called, which settlements mattered, and "
                    f"whether the plan's county list is right. Findings are input, not truth: "
                    f"the note never changes plan/regions.json or any schema file."
                ),
                rules=[
                    "Every claim carries a citation to a public source.",
                    "Say plainly when a fact could not be established.",
                    "A finding that contradicts the plan is reported in the note, not acted on.",
                ],
            )
        )
    return collected
