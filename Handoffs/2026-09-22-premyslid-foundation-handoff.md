# Handoff — Premyslid foundation (Suvorov)

**Date:** 2026-09-22. **Project:** Suvorov (`%USERPROFILE%\Software_Engineering\Suvorov`).
**Session type:** interactive, James present throughout.
**Branch:** `feat/premyslid-schema-v0`, pushed. **PR:** [#3](https://github.com/jamestyhurst/Suvorov/pull/3), draft, CI green.

This session designed and built `games\premyslid\`, a Crusader Kings 1 clone that is Suvorov's
first game. Nothing below repeats what the repository already records — read those first:

| Artifact | What it holds |
| --- | --- |
| `Developer_Documents\premyslid-design.md` | Why every design choice was made, what it traded away, and the failure mode it guards against. **Start here.** |
| `Developer_Documents\premyslid-operations.md` | How to run the grind: loop, commands, CI, milestone, troubleshooting. |
| `Developer_Documents\pending-root-changes.md` | Root-document edits identified but deliberately not applied, with exact proposed wording. |
| `games\premyslid\docs\decisions.md` | 14 decision entries covering every fork resolved this session. |
| `games\premyslid\AGENTS.md` | The brief free models read. Short and mechanical on purpose. |
| PR #3 description | Summary of the change set and its verification. |

## Where things stand

The schema, validator, work queue, session scripts, and CI all exist and run. There is **no
engine**: nothing simulates a day, a death, or an inheritance. Content is written depth-first
inside the exemplar duchy `d_bohemia`; the rest of the map is locked by
`plan\regions.json` → `bulk_unlocked: false`.

Current state, verified at handoff time:

```
python -m unittest discover -s tests -t .   # 70 tests, green   (run from python\)
python -m suvorov.tools.validate            # 9 records, valid
python -m suvorov.tools.next                # 21 tasks outstanding
```

## What the next session should do

**1. Install PyGame. James asked for this explicitly and wants it done soon — treat it as the
session's first job, not an optional extra.**

The rendering decision was made this session (PyGame, for Suvorov as a whole, until James's
machine is upgraded) but deliberately not acted on: `AGENTS.md` requires asking James before
installing anything, because the school PC's IT monitoring raises an alert on new software.
James has now asked for it, so the remaining constraints are technical:

- Per-user install only, no admin rights, no UAC: `python -m pip install --user pygame`.
- Confirm which machine the session is on first. The constraint above is a school-PC rule; the
  laptop (`ZHANTIANZHE-LAP`) is where this work has been happening.
- PyGame belongs in a presentation layer that imports the simulation core. `CLAUDE.md` requires
  the core stay independent of rendering — the dependency runs one way only, and nothing in
  `suvorov\core\` may import it.
- There is nothing to render yet. A first step is a window that displays the scenario date and
  the exemplar duchy's title tree read through the tooling, which proves the layering without
  needing an engine.
- Record the install in the learning log, and add a decision-log entry if the approach departs
  from what `pending-root-changes.md` proposes.

**2. Merge the stack.** PR #2 (`feat/world-date-and-polities`) is the base of PR #3. Merge #2
first, then retarget or merge #3. PR #3 is a draft; mark it ready when #2 lands.

**3. Enable repository settings the CI depends on.** Auto-merge must be on, and the default
`GITHUB_TOKEN` needs merge permission, or the content-session merge job fails with a
permission error. Untested so far — no content session has ever run.

**4. Apply `pending-root-changes.md`.** Four root files had uncommitted PR #2 work in the tree
this session, so they were left alone. Once #2 settles, apply the proposed Rule 1 amendment and
the three root decision-log entries.

**5. First real OpenCode content session.** This is the untested half of the whole design. The
scripts have never been run by a free model. Watch, in order: does it run
`premyslid_start` at all; does it write the file the task named; does it run the validator
unprompted; does it stop after one task. If it fails at step 1 or 4, fix
`games\premyslid\AGENTS.md` — shorter and blunter — not the model.

## Things a fresh agent will otherwise get wrong

- **Do not read `..\Teutoburg\`.** `AGENTS.md` rule 2. James's stated principle: *"I want to
  know what coding agents would do if told to imitate Clausewitz, rather than imitating
  Teutoburg."* This session breached it before reading the brief, and that breach is on the
  record in `games\premyslid\docs\decisions.md`. Do not compound it.
- **The engine/content boundary is the point, not an inconvenience.** A content task that
  cannot be completed inside `content\` is a *finding* about a missing engine capability, to be
  written up as a blocked note. Never widen the boundary to make a task pass.
- **Never edit the schema to make content validate.** The schema and `plan\regions.json` are
  protected. A genuine conflict is a decision for James, logged.
- **`Premyslid` is spelled without diacritics deliberately** (properly Přemyslid). James ruled
  that diacritics are bad in this context. Two facts recorded so nobody invents around them:
  Trutnov was founded in the thirteenth century so it does not exist at the 1066 start date,
  and "Sudetenland" is a twentieth-century German term, not anything a Premyslid would know.
- **Metrics are derived, never stored.** Do not add a CI step that commits a metrics file; the
  reasons are in the design document.

## Suggested skills

- **`/wrap`** — at the end of the next session. Workspace-scoped, so it only appears when the
  session's working directory is inside `Software_Engineering\`. It writes the learning-log
  entry, then hands off to the project's own duties.
- **`/teach`** — if the PyGame work teaches a concept. Likely candidates with existing notes:
  `separation-of-concerns-pipeline`, `configuration-vs-code`, `packaging-a-desktop-app`.
- **`/tdd`** — for any engine work. `AGENTS.md` rule 4 makes tests-before-code mandatory for
  AI-generated code, and this session's 70 tests were written that way.
- **`/code-review`** — before marking PR #3 ready, if a second opinion on 3,686 added lines is
  wanted.
- **`/grill-me`** — before building the engine. The engine's design has not been grilled; only
  the content pipeline has.

## Session provenance

Work produced inside a Claude Code session on `ZHANTIANZHE-LAP`, model Opus 5 (1M context),
assistance level 4 (agentic, supervised). James resolved every design fork himself; the
session proposed options and recommendations. Two of his redirections changed the project's
shape materially — from a one-shot attempt to a schema-first grind, and from a standalone
repository to a game inside Suvorov.
