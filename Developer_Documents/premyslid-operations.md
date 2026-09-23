# Premyslid: running the grind

How the loop actually runs, what to check, and what to do when it breaks. Written
2026-09-22, when nothing had been run against a real free model yet — treat the
troubleshooting section as predictions rather than observations until it has been.

## The loop, end to end

1. A free model is told: *"Go work on the Premyslid clone. Read
   `games\premyslid\AGENTS.md` and follow it."*
2. `pwsh scripts\premyslid_start.ps1` switches to `main`, pulls, asks
   `python -m suvorov.tools.next` for one task, makes a branch named after it, and prints the
   task. The chosen task id is left in `.premyslid-session`, which is git-ignored.
3. The model writes the files the task names — only inside
   `games\premyslid\content\` or `games\premyslid\research\`.
4. `python -m suvorov.tools.validate` reports every problem at once. The model fixes and
   repeats.
5. `pwsh scripts\premyslid_finish.ps1 -Model <name>` re-validates, refuses anything outside
   the two writable folders, commits with `Task:` and `Model:` trailers, pushes, and opens a
   pull request.
6. Continuous integration runs the tests, the validator, and the write-boundary check. A pull
   request carrying a `Model:` trailer then merges itself.

The bash equivalents — `scripts\premyslid_start.sh` and `scripts\premyslid_finish.sh` — exist
for agents that do not run PowerShell. The PowerShell versions are the reference; if the two
disagree, fix the bash one.

## Commands worth knowing

| Command | What it does |
| --- | --- |
| `python -m suvorov.tools.next` | Print one task, chosen at random from the ten highest-priority gaps. |
| `python -m suvorov.tools.next --all` | Print every outstanding task, in priority order. |
| `python -m suvorov.tools.next --json` | The same, for scripts. |
| `python -m suvorov.tools.validate` | Check all content. Exit code 1 means problems, listed. |
| `python -m suvorov.tools.metrics` | Sessions, content files, research files, and blocked tasks, grouped by model. |
| `python -m suvorov.tools.metrics --csv` | One row per attributed commit. |
| `python -m unittest discover -s tests -t .` | The test suite. Run from `python\`. |

All of these run from the `python\` directory, except the test suite's `-t .`, which also
expects it.

## What continuous integration enforces

`.github\workflows\premyslid.yml` runs on every pull request:

- the full test suite;
- the content validator;
- the write boundary, **only** for pull requests whose commits carry a `Model:` trailer;
- a metrics summary, written to the run summary page.

A content session's pull request merges itself once those pass. Anything without a `Model:`
trailer — James's work, a strong model's engine work — is left for review. Two settings have
to be right on the repository for the merge step to work: auto-merge must be enabled, and
the default `GITHUB_TOKEN` needs permission to merge. If the merge job logs a permission
error, that is why.

## The engine milestone

`next` announces the milestone when the exemplar duchy has no outstanding content work. The
full condition is: exemplar duchy complete, validator green, and
`games\premyslid\schema\` unchanged across ten merges. Until it is met and an engine runs a
succession loop, `plan\regions.json` keeps `bulk_unlocked` at `false` and the queue offers
research rather than the rest of the map.

Unlocking is a deliberate act: set `bulk_unlocked` to `true`, and log the decision. Do not
unlock to keep the models busy — research tasks exist for that.

## What to watch in the first sessions

The design's weak point is tool use, not writing. In roughly this order:

1. Does the model run `premyslid_start` at all, or does it start editing files directly?
2. Does it write the file the task named, or a different one?
3. Does it run the validator before finishing, or only after being told to?
4. Does it stop after one task?
5. Does the pull request contain exactly the intended files?

If the answer to 1 or 4 is no, the fix belongs in `games\premyslid\AGENTS.md` — shorter,
blunter, fewer options — not in the model.

## Troubleshooting

**The queue is empty.** This should not happen while the engine is missing; research tasks
should always remain. If it does, either every planned duchy and county has a research note,
or `plan\regions.json` is not being read. Check the plan first.

**Two sessions did the same task.** Expected occasionally, and cheap. The second pull request
will either be an empty diff or conflict; close it. Do not add a claim protocol without a
decision log entry explaining what went wrong with the random pick.

**A content session's pull request fails the boundary check.** Correct behaviour. The session
should have written a blocked note instead. If it happens repeatedly for the same kind of
task, that is the signal the boundary is there to produce: a missing engine capability. Read
the blocked notes before changing the rule.

**The validator complains about something historically true.** The validator's rules are
modelling assumptions, not facts about the eleventh century — for example that a parent is at
least twelve years older than their child, or that a county carries no succession law of its
own. A real conflict is a schema decision for James, logged in
`games\premyslid\docs\decisions.md`. It is never fixed by a content session editing the
schema.

**Metrics show no sessions.** Either no content session has merged yet, or the finish script
was bypassed and the trailers were never written. The second case is unrecoverable for the
commits affected; fix the script invocation rather than back-filling.
