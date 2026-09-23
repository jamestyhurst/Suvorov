# Research notes

Everything in this folder is **non-canon input**. A note records what an agent found and
where it found it. It does not change the plan, the schema, or any content record, and no
tool reads it.

Promotion is a human decision. When James accepts a finding, the change is made to the file
that owns it — `plan/regions.json`, a schema file, or a content record — and the decision is
logged in `docs/decisions.md`. Until then a note is a claim, not a fact, however confident
it sounds.

## Why the folder works this way

Small models write fluent and confident prose about the eleventh century, including about
things that are not so. Keeping findings in a quarantine folder means that being wrong is
cheap: a wrong note costs a read, while a wrong content record propagates into every
reference to it.

## Writing a note

- Name the file after what it is about: `d_bohemia.md`, `c_hradec.md`.
- Every claim carries a citation. A claim with no source is written as "not established".
- Say plainly where the evidence ran out. "The county boundaries could not be confirmed" is
  a better note than a confident boundary list.
- If a finding contradicts `plan/regions.json`, say so in the note. Do not edit the plan.

## Blocked tasks

A task that cannot be finished is written up here as `blocked-<task id>.md`, naming the task,
what stopped it, and what would have to change. These notes are how the engine's missing
capabilities get discovered: a content task that cannot be expressed is evidence about the
schema or the engine, and that evidence is the reason this game sits inside Suvorov.
