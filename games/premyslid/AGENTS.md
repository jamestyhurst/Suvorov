# Premyslid — content session brief

Read this file and nothing else. The brief at the repository root does not apply here.

## The loop

1. `pwsh scripts/premyslid_start.ps1` — makes a branch and prints one task.
2. Do exactly that task. Write only the files the task names.
3. `python -m suvorov.tools.validate` — fix every problem it reports, then run it again.
4. `pwsh scripts/premyslid_finish.ps1 -Model <your model name>` — commits, pushes, opens a pull request.
5. Stop. Do not start a second task.

## What you may write

Only these two folders:

- `games/premyslid/content/`
- `games/premyslid/research/`

Everything else is protected, including the schema, the plan, and all Python code. A pull
request touching anything else is rejected automatically.

## If the task cannot be done

Do not work around it, and do not edit a protected file to make it possible. Write a note at
`games/premyslid/research/blocked-<task id>.md` saying what the task was, what stopped you,
and what would have to change. Commit that note instead, and finish the session normally.
A blocked task is a useful result, not a failure.

## Rules that the validator cannot check for you

- Every claim about a real person or place needs a citation to a public source.
- Never copy text or data from Crusader Kings' own game files.
- If you are unsure of a historical fact, say so in the record's source list rather than
  inventing a confident answer.
- Research notes are input, not truth. A note never changes `plan/regions.json` or any schema.

## Facts you need

- The scenario starts on the date in `games/premyslid/plan/regions.json`.
- One record per file; the file name is the record's id.
- The schema is in `games/premyslid/schema/`, and its README explains the dialect.
- Effect and trigger names are frozen. Do not invent new ones.
