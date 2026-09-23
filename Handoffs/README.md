# Handoffs

Documents written at the end of a session so the next one — a different agent, a different
device, or James weeks later — can pick the work up without reconstructing it from the diff.

Tracked in git on purpose. A handoff left in a temporary folder is invisible to a session
running anywhere else, which defeats the point of writing one. The convention matches the
`Handoffs\` folders in Greco and Teutoburg.

## Naming

`YYYY-MM-DD-topic-slug.md` — date stamp and topic, both. Never date-only (two sessions in one
day collide) and never topic-only (the reader cannot tell which handoff is current).

## What belongs in one

A handoff points at the durable record; it does not duplicate it. Decisions belong in a
decision log, reasoning in `Developer_Documents\`, and the change itself in the commit and the
pull request. What a handoff adds is everything those cannot hold:

- where the work actually stands, including what was verified and when;
- what the next session should do, in priority order;
- what a fresh agent would otherwise get wrong — the traps, the rules that look optional and
  are not, the spellings and facts chosen deliberately;
- which skills to invoke.

## Retiring one

A handoff describes a moment, not a standing truth, and a stale one is worse than none: it
reads as current. When its work is finished, move it into a `handled\` subfolder rather than
deleting it — the record of what was handed over is itself evidence about how the project ran.

## Current

| Handoff | Covers |
| --- | --- |
| [2026-09-22-premyslid-foundation-handoff.md](2026-09-22-premyslid-foundation-handoff.md) | The session that designed and built `games\premyslid\`: schema, validator, work queue, session scripts, CI. Next job is installing PyGame. |
