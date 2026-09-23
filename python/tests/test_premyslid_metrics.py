"""Tests for reconstructing session metrics from git history.

The commit trailers are the record. Git already knows when a commit landed and which files it
touched; what git does not know is which model wrote it, so the Model trailer is the one
thing that would be lost forever if it were not written at commit time.

Metrics are derived rather than stored. A file of accumulated rows would have to be written
by continuous integration, which means a commit on every merge and a race between concurrent
merges. Reading the history instead means the numbers can never drift from what happened.
"""

from __future__ import annotations

import unittest

from suvorov.tools import metrics

LOG = (
    "\x1e"
    "a1b2c3\x1f2026-09-22T10:00:00+01:00\x1fcontent(premyslid): title-c_kourim\x1f"
    "Task: title-c_kourim\nModel: qwen2.5-coder:7b\n\x1f"
    "12\t0\tgames/premyslid/content/titles/c_kourim.json\n"
    "\x1e"
    "d4e5f6\x1f2026-09-22T11:30:00+01:00\x1fcontent(premyslid): research-d_bohemia\x1f"
    "Task: research-d_bohemia\nModel: llama3.1:8b\n\x1f"
    "40\t0\tgames/premyslid/research/d_bohemia.md\n"
    "\x1e"
    "090909\x1f2026-09-21T09:00:00+01:00\x1ffeat(core): world date\x1f"
    "\x1f"
    "44\t0\tpython/suvorov/core/world.py\n"
)


class TestParsing(unittest.TestCase):
    def test_every_commit_is_read(self) -> None:
        self.assertEqual(len(metrics.parse_log(LOG)), 3)

    def test_trailers_are_extracted(self) -> None:
        first = metrics.parse_log(LOG)[0]
        self.assertEqual(first.model, "qwen2.5-coder:7b")
        self.assertEqual(first.task, "title-c_kourim")

    def test_a_commit_without_trailers_is_kept_but_unattributed(self) -> None:
        last = metrics.parse_log(LOG)[2]
        self.assertIsNone(last.model)
        self.assertIsNone(last.task)

    def test_files_are_counted_by_area(self) -> None:
        commits = metrics.parse_log(LOG)
        self.assertEqual(commits[0].content_files, 1)
        self.assertEqual(commits[0].research_files, 0)
        self.assertEqual(commits[1].research_files, 1)
        self.assertEqual(commits[2].content_files, 0)

    def test_the_date_is_kept_as_a_day(self) -> None:
        self.assertEqual(metrics.parse_log(LOG)[0].day, "2026-09-22")

    def test_an_empty_log_is_not_an_error(self) -> None:
        self.assertEqual(metrics.parse_log(""), [])


class TestSummary(unittest.TestCase):
    def test_sessions_are_grouped_by_model(self) -> None:
        summary = metrics.summarise(metrics.parse_log(LOG))
        self.assertEqual(summary["qwen2.5-coder:7b"].sessions, 1)
        self.assertEqual(summary["qwen2.5-coder:7b"].content_files, 1)
        self.assertNotIn(None, summary)

    def test_unattributed_commits_are_excluded_from_the_summary(self) -> None:
        summary = metrics.summarise(metrics.parse_log(LOG))
        self.assertEqual(sum(row.sessions for row in summary.values()), 2)


if __name__ == "__main__":
    unittest.main()
