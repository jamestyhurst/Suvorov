"""Reconstructing what each model contributed, from the git history alone.

The Premyslid experiment compares how models behave when the task is well defined. Git
already records when work landed and which files it touched. The one fact it cannot recover
afterwards is which model wrote a commit, which is why the session scripts write a Model
trailer and why this module reads it back.

Nothing is stored. Deriving the numbers on demand means they cannot drift from the history,
and it avoids having continuous integration commit a metrics file on every merge, which
would race with itself the moment two pull requests merge at once.

    python -m suvorov.tools.metrics
    python -m suvorov.tools.metrics --csv
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

RECORD_SEPARATOR = "\x1e"
FIELD_SEPARATOR = "\x1f"
LOG_FORMAT = f"{RECORD_SEPARATOR}%H{FIELD_SEPARATOR}%aI{FIELD_SEPARATOR}%s{FIELD_SEPARATOR}%b{FIELD_SEPARATOR}"

CONTENT_PREFIX = "games/premyslid/content/"
RESEARCH_PREFIX = "games/premyslid/research/"


@dataclass(frozen=True)
class Commit:
    """One commit, with its trailers read and its files bucketed by area."""

    commit_hash: str
    day: str
    subject: str
    task: str | None
    model: str | None
    files: tuple[str, ...]

    @property
    def content_files(self) -> int:
        return sum(1 for path in self.files if path.startswith(CONTENT_PREFIX))

    @property
    def research_files(self) -> int:
        return sum(1 for path in self.files if path.startswith(RESEARCH_PREFIX))

    @property
    def blocked(self) -> bool:
        """A session that reported a blocked task instead of producing content."""

        return any(path.startswith(f"{RESEARCH_PREFIX}blocked-") for path in self.files)


@dataclass
class ModelSummary:
    sessions: int = 0
    content_files: int = 0
    research_files: int = 0
    blocked: int = 0
    tasks: list[str] = field(default_factory=list)


def parse_log(text: str) -> list[Commit]:
    """Turn ``git log`` output in :data:`LOG_FORMAT` with ``--numstat`` into commits."""

    commits: list[Commit] = []
    for raw in text.split(RECORD_SEPARATOR):
        if not raw.strip():
            continue
        parts = raw.split(FIELD_SEPARATOR)
        if len(parts) < 5:
            continue
        commit_hash, iso_date, subject, body, numstat = parts[0], parts[1], parts[2], parts[3], parts[4]
        commits.append(
            Commit(
                commit_hash=commit_hash.strip(),
                day=iso_date.strip()[:10],
                subject=subject.strip(),
                task=_trailer(body, "Task"),
                model=_trailer(body, "Model"),
                files=tuple(_paths(numstat)),
            )
        )
    return commits


def _trailer(body: str, name: str) -> str | None:
    prefix = f"{name}:"
    for line in body.splitlines():
        stripped = line.strip()
        if stripped.startswith(prefix):
            value = stripped[len(prefix) :].strip()
            return value or None
    return None


def _paths(numstat: str) -> list[str]:
    paths: list[str] = []
    for line in numstat.splitlines():
        columns = line.split("\t")
        if len(columns) == 3 and columns[2]:
            paths.append(columns[2].strip())
    return paths


def summarise(commits: list[Commit]) -> dict[str, ModelSummary]:
    """Group attributed commits by model. Commits with no Model trailer are left out: they
    were not written by a content session, and counting them would flatter the numbers."""

    summary: dict[str, ModelSummary] = {}
    for commit in commits:
        if commit.model is None:
            continue
        row = summary.setdefault(commit.model, ModelSummary())
        row.sessions += 1
        row.content_files += commit.content_files
        row.research_files += commit.research_files
        row.blocked += 1 if commit.blocked else 0
        if commit.task:
            row.tasks.append(commit.task)
    return summary


def read_history(repository_root: Path) -> list[Commit]:
    result = subprocess.run(
        ["git", "log", "--no-merges", "--numstat", f"--format={LOG_FORMAT}"],
        capture_output=True,
        text=True,
        check=True,
        cwd=repository_root,
    )
    return parse_log(result.stdout)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Summarise content sessions from git history.")
    parser.add_argument("--repository", type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument("--csv", action="store_true", help="Emit one row per commit as CSV.")
    arguments = parser.parse_args(argv)

    commits = read_history(arguments.repository)

    if arguments.csv:
        print("commit,day,model,task,content_files,research_files,blocked")
        for commit in commits:
            if commit.model is None:
                continue
            print(
                f"{commit.commit_hash[:10]},{commit.day},{commit.model},{commit.task},"
                f"{commit.content_files},{commit.research_files},{int(commit.blocked)}"
            )
        return 0

    summary = summarise(commits)
    if not summary:
        print("No attributed content sessions yet.")
        return 0

    width = max(len(model) for model in summary)
    print(f"{'model'.ljust(width)}  sessions  content  research  blocked")
    for model in sorted(summary):
        row = summary[model]
        print(
            f"{model.ljust(width)}  {row.sessions:>8}  {row.content_files:>7}  "
            f"{row.research_files:>8}  {row.blocked:>7}"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
