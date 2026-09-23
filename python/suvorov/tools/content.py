"""Loading a game's plan, schema, and content records off disk.

One record per file, with the file name equal to the record's id. That rule costs a little
convenience and buys two things worth more: two agents working at the same time almost never
touch the same file, and a record's identity can be checked without trusting its contents.
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path

# kind -> (folder under content/, schema file under schema/, singular word for messages)
KIND_SPECS: dict[str, tuple[str, str, str]] = {
    "titles": ("titles", "title.schema.json", "title"),
    "holdings": ("holdings", "holding.schema.json", "holding"),
    "characters": ("characters", "character.schema.json", "character"),
    "dynasties": ("dynasties", "dynasty.schema.json", "dynasty"),
    "cultures": ("cultures", "culture.schema.json", "culture"),
    "faiths": ("faiths", "faith.schema.json", "faith"),
    "succession_laws": ("succession_laws", "succession_law.schema.json", "succession law"),
    "events": ("events", "event.schema.json", "event"),
}

TIER_PREFIXES = {"e": "empire", "k": "kingdom", "d": "duchy", "c": "county", "b": "barony"}
TIER_ORDER = ["empire", "kingdom", "duchy", "county", "barony"]


@dataclass(frozen=True)
class Record:
    """One content file that parsed as JSON."""

    kind: str
    record_id: str
    path: Path
    data: dict


@dataclass(frozen=True)
class UnreadableFile:
    """One content file that did not parse. Kept rather than raised, so one broken file does
    not hide every other problem in the same run."""

    path: Path
    reason: str


@dataclass
class Game:
    """Everything the tooling knows about a game, with no rules applied yet."""

    root: Path
    plan: dict
    schemas: dict[str, dict]
    records: dict[str, dict[str, Record]] = field(default_factory=dict)
    unreadable: list[UnreadableFile] = field(default_factory=list)

    def of_kind(self, kind: str) -> dict[str, Record]:
        return self.records.get(kind, {})

    def get(self, kind: str, record_id: str) -> Record | None:
        return self.records.get(kind, {}).get(record_id)

    def exists(self, kind: str, record_id: str) -> bool:
        return record_id in self.records.get(kind, {})

    def titles_of_tier(self, tier: str) -> dict[str, Record]:
        return {
            title_id: record
            for title_id, record in self.of_kind("titles").items()
            if record.data.get("tier") == tier
        }

    def relative(self, path: Path) -> str:
        try:
            return path.relative_to(self.root).as_posix()
        except ValueError:
            return path.as_posix()

    @property
    def start_date(self) -> str:
        return self.plan["start_date"]

    @property
    def planned_duchies(self) -> list[dict]:
        """Every duchy in the plan, flattened, each carrying the kingdom it belongs to."""

        duchies: list[dict] = []
        for kingdom in self.plan.get("hierarchy", []):
            for duchy in kingdom.get("duchies", []):
                duchies.append({**duchy, "kingdom": kingdom["id"]})
        return duchies


def load_game(root: Path) -> Game:
    """Read ``root`` into a :class:`Game`. Missing folders are treated as empty, not as errors:
    an empty content folder is the normal state of a game that has not been written yet."""

    plan = json.loads((root / "plan" / "regions.json").read_text(encoding="utf-8"))
    schemas = {
        kind: json.loads((root / "schema" / schema_name).read_text(encoding="utf-8"))
        for kind, (_, schema_name, _) in KIND_SPECS.items()
    }

    game = Game(root=root, plan=plan, schemas=schemas)
    for kind, (folder, _, _) in KIND_SPECS.items():
        game.records[kind] = {}
        directory = root / "content" / folder
        if not directory.is_dir():
            continue
        for path in sorted(directory.glob("*.json")):
            try:
                data = json.loads(path.read_text(encoding="utf-8"))
            except (json.JSONDecodeError, UnicodeDecodeError) as exc:
                game.unreadable.append(UnreadableFile(path=path, reason=str(exc)))
                continue
            if not isinstance(data, dict):
                game.unreadable.append(
                    UnreadableFile(path=path, reason="the file holds a list; one record per file")
                )
                continue
            # The file name is the identity the tooling trusts. A record whose `id` disagrees
            # is still loaded under the file name so that the mismatch can be reported once,
            # rather than appearing as a pile of dangling references.
            game.records[kind][path.stem] = Record(
                kind=kind, record_id=path.stem, path=path, data=data
            )
    return game


def tier_of(title_id: str) -> str | None:
    prefix = title_id.split("_", 1)[0]
    return TIER_PREFIXES.get(prefix)


def tier_above(tier: str) -> str | None:
    index = TIER_ORDER.index(tier)
    return TIER_ORDER[index - 1] if index > 0 else None


def tier_below(tier: str) -> str | None:
    index = TIER_ORDER.index(tier)
    return TIER_ORDER[index + 1] if index + 1 < len(TIER_ORDER) else None
