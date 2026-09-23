"""``next`` — print one task to do.

This is the entry point a content session starts from. It answers "what should I do" with a
single concrete instruction naming the file to write and the schema to follow, so that the
model never has to decide what is worth doing. Deciding what is worth doing is the thing
small models are worst at, and it is the thing this command removes.

    python -m suvorov.tools.next
    python -m suvorov.tools.next --all
    python -m suvorov.tools.next --json
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import asdict
from pathlib import Path

from suvorov.tools import content, gaps

# How many merges the schema must survive unchanged before the engine milestone is announced.
SCHEMA_STABILITY_MERGES = 10


def _default_game_root() -> Path:
    return Path(__file__).resolve().parents[3] / "games" / "premyslid"


def _merges_since_schema_changed(game_root: Path) -> int | None:
    """Commits on this branch since the schema last changed, or None if git cannot answer.

    Used only to announce the engine milestone. A wrong answer delays a message; it never
    changes what work is handed out, so a missing git is not worth failing over.
    """

    schema_path = game_root / "schema"
    try:
        last_change = subprocess.run(
            ["git", "log", "-1", "--format=%H", "--", str(schema_path)],
            capture_output=True,
            text=True,
            check=True,
            cwd=game_root,
        ).stdout.strip()
        if not last_change:
            return None
        count = subprocess.run(
            ["git", "rev-list", "--count", f"{last_change}..HEAD"],
            capture_output=True,
            text=True,
            check=True,
            cwd=game_root,
        ).stdout.strip()
        return int(count)
    except (subprocess.CalledProcessError, FileNotFoundError, ValueError):
        return None


def _milestone_note(game: content.Game, game_root: Path) -> str | None:
    if game.plan.get("bulk_unlocked"):
        return None
    if not gaps.exemplar_is_complete(game):
        return None

    merges = _merges_since_schema_changed(game_root)
    stability = (
        "schema change history unavailable"
        if merges is None
        else f"{merges} commit(s) since the schema last changed"
    )
    ready = merges is not None and merges >= SCHEMA_STABILITY_MERGES
    verdict = "The milestone is met." if ready else "The milestone is not met yet."

    return (
        "ENGINE SESSION DUE\n"
        f"The exemplar duchy is complete and {stability}. {verdict}\n"
        f"The milestone is: exemplar duchy complete, validator green, and the schema unchanged "
        f"across {SCHEMA_STABILITY_MERGES} merges.\n"
        "Bulk map work stays locked until an engine runs a succession loop. Until then the "
        "tasks below are research only. This message is for James, not a task."
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Print the next task for this game.")
    parser.add_argument("--game", type=Path, default=_default_game_root())
    parser.add_argument("--all", action="store_true", help="List every outstanding task.")
    parser.add_argument("--json", action="store_true", help="Emit JSON instead of text.")
    parser.add_argument("--seed", type=int, default=None, help="Fix the random choice, for tests.")
    arguments = parser.parse_args(argv)

    game = content.load_game(arguments.game)
    outstanding = gaps.compute_gaps(game)
    note = _milestone_note(game, arguments.game)

    if arguments.json:
        payload = {
            "milestone": note,
            "outstanding": len(outstanding),
            "tasks": [asdict(gap) for gap in (outstanding if arguments.all else [])],
        }
        if not arguments.all:
            chosen = gaps.choose(outstanding, seed=arguments.seed)
            payload["task"] = asdict(chosen) if chosen else None
        print(json.dumps(payload, indent=2))
        return 0

    if note:
        print(note)
        print()

    if not outstanding:
        print("Nothing outstanding. Tell James: the queue is empty, which should not happen.")
        return 0

    if arguments.all:
        for gap in outstanding:
            print(gap.render())
            print()
        print(f"{len(outstanding)} task(s) outstanding.")
        return 0

    chosen = gaps.choose(outstanding, seed=arguments.seed)
    assert chosen is not None
    print(chosen.render())
    print()
    print(f"({len(outstanding)} task(s) outstanding. This one was chosen at random from the top "
          f"{gaps.CHOICE_WINDOW}.)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
