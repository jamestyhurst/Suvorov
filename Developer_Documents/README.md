# Developer documents

Documents written for a capable agent or a human who is about to work on this repository and
needs the reasoning, not just the rules.

The split is deliberate. The briefs — `AGENTS.md` at the root, `games\premyslid\AGENTS.md`
for content sessions — are short and mechanical, because the models that read them work
better with fewer instructions and no rationale to improvise against. Everything those briefs
leave out lives here.

Read this folder when you are being pointed at a hard piece of work: designing an engine
system, changing the schema, deciding whether a system should be promoted from a game into
the engine, or judging whether the project is going the way it was meant to.

## Contents

| Document | What it holds |
| --- | --- |
| [premyslid-design.md](premyslid-design.md) | Why the Premyslid game is built the way it is: the full chain of decisions, what each one traded away, and the failure modes each one guards against. |
| [premyslid-operations.md](premyslid-operations.md) | How to run and maintain the grind: the session loop, continuous integration, metrics, the engine milestone, and what to do when something goes wrong. |
| [pending-root-changes.md](pending-root-changes.md) | Changes to root-level documents that were identified but not applied, with the exact wording proposed, and why they were left for James. |

## Related records

- `docs\decisions.md` — engine-level decisions for Suvorov itself.
- `games\premyslid\docs\decisions.md` — decisions about the Premyslid game.
- `CONCEPTS.md` — the pointer index into the shared concept notes.
