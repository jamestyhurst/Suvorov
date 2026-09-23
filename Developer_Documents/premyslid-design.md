# Premyslid: why it is built this way

Written 2026-09-22, at the end of the session that designed it. The decisions themselves are
logged in `games\premyslid\docs\decisions.md`; this document holds the reasoning that a log
entry is too short to carry.

## What the project is

Premyslid is a clone of Crusader Kings 1, built inside Suvorov as its first game. It exists
for three reasons at once, and keeping all three in view explains most of the design:

1. **A question about models.** James had seen frontier models used to one-shot clones of
   small games, and never of a Paradox game. A grand strategy game is interlocking in a way
   a snake clone is not, so the interesting question is what happens when you point weaker
   models at one — and specifically, what a *well-defined* task does for them. The schema is
   the definition. It is the artifact under test, and the game is what falls out.
2. **A consumer for the engine.** Suvorov claims to be a context-free grand strategy
   framework. A framework with no game cannot have that claim tested. Premyslid is the test:
   systems that turn out to be general get promoted into the engine, and the friction of
   promotion is itself information.
3. **Something to compare against Teutoburg.** Teutoburg was built from an ill-defined
   target — design a game that has never existed. Premyslid is built from a well-defined one.
   The comparison is not controlled and never will be, but it is worth being able to make it
   at all, which is why provenance is recorded at commit time.

What it is *not*: a product, a passion project, or a thing worth spending API credits on. The
budget is a subscription for the design work and free OpenCode models for the grind. Every
design choice below is cheaper than the obvious alternative, on purpose.

## The shape: a strong model writes the definition, weak models execute it

The alternative shapes were considered and rejected in order.

A literal one-shot — one prompt, accept the output — is the demo James had seen, and it is
the cheapest thing possible. It was rejected for a practical reason rather than a principled
one: the models available on this subscription are not the ones those demos use, and
spending the whole month's allowance on a single attempt likely to fail is a bad trade when
the failure teaches little that was not already predictable.

An unlimited single session, or a fixed multi-session budget, both produce a more playable
result and both cost real money.

What is left is the split this project uses: spend the expensive model's time once, on the
thing that compounds — a schema precise enough that a cheap model can produce correct work
against it — and then spend nothing per unit of output. This also happens to be the version
that answers question 1 most directly, because the schema's quality is exactly the variable.

## Why content is data rather than code

Free models write plausible, confident, broken logic. They fill structured forms much more
reliably. If content were Python implementing engine interfaces, every contribution would
need tests written by somebody to catch what went wrong, and writing those tests is most of
the work the arrangement exists to avoid.

Declarative records validated against a schema turn review into a computation. A misspelled
effect name is not a subtle bug that surfaces in a month; it is an immediate validation
failure with a message naming the file and the allowed values. This is also how Crusader
Kings itself is built — an engine binary plus scripted content files with a fixed trigger and
effect vocabulary — so the architecture is a copy of the thing being cloned rather than an
invention.

The cost is expressiveness. Anything a content record cannot say has to be said by the
engine, and the engine does not exist yet. That cost is deliberately converted into a signal:
see the write boundary below.

## Why the boundary between content and engine is enforced by a machine

A content session may write two folders and nothing else. The rule is checked by continuous
integration, not stated in a brief, because these merges have no human reviewer and a small
model given an inconvenient instruction will route around it.

The more interesting reason is what the boundary produces. When a task cannot be finished
inside content, the session writes a blocked note instead. That note says: here is a thing
the game needs to express and cannot. That is a missing engine capability, discovered by
attempting real work rather than by speculation. It is the mechanism by which Premyslid
feeds Suvorov, and it only works if the boundary actually holds.

## Why the work queue is computed rather than written

The requirement was that "go work on the Premyslid clone" always produces something useful,
with no task-picking by James and none by the model either. Deciding what is worth doing is
the weakest capability of a weak model: given a repository and freedom, it redoes finished
work, invents scope, or rewrites something that already works.

A hand-written backlog solves the picking problem but runs out, and refilling it costs a
strong-model session each time. A computed queue does not run out, because the plan and the
schema together define what completeness means: every gap between them is a task, and a task
disappears from the queue the moment its file merges.

The random pick from the top ten is a deliberate non-solution to collisions. A claim protocol
would need a commit before work starts, a release on merge, and a way to clear stale claims
left by abandoned sessions — three mechanisms, all of which a weak model can get wrong. The
random pick makes a collision unlikely, and a collision that does happen costs one empty pull
request. The work was free; the coordination would not have been.

## Why the engine was deferred, and what holds the risk down

Handing off at schema-and-validator is the cheapest possible start and the riskiest ordering:
content accumulates with nothing to prove it is shaped right, so rework scales with volume.
James chose it knowingly to conserve subscription usage. Three things hold the risk down:

- **Depth before breadth.** One duchy is finished completely before any second duchy begins,
  and bulk work is locked until the engine runs. A wrong schema costs one duchy.
- **A validator that checks meaning.** Shape-checking would catch a typo; it would not catch
  a count whose holder died before the scenario starts, a capital belonging to a different
  county, a hierarchy that loops, or a marriage recorded on one side only. Those checks are
  the only review this content gets.
- **Research as parallel work.** When exemplar content runs out, the queue hands out research
  notes instead of unlocking the map. This keeps free models busy without accumulating
  content against an unproven schema.

The milestone that ends the deferral is stated up front — exemplar duchy complete, validator
green, schema unchanged across ten merges — because a deferral with no condition drifts. The
grind produces visible progress every day, which is exactly the pressure that would postpone
an engine indefinitely.

## Why research findings are quarantined

A small model asked about eleventh-century Bohemia will produce fluent, confident, cited-
looking prose, some of which will be wrong. Findings therefore land in `research\` as
non-canon input: no tool reads them, and they never change the plan, a schema, or a content
record. Promotion is James's decision and is logged.

The asymmetry is the point. A wrong note costs one read. A wrong content record propagates
into everything that references it, and with no engine running, nothing will notice.

## Why provenance is a commit trailer

Git records when work landed, what changed, and how much. It does not record which model
produced it, and that is the variable the whole project is about. The trailer is written at
commit time by the session script because it is the one fact that cannot be recovered later.

Metrics are derived from the history on demand rather than accumulated in a file. A file
would have to be written by continuous integration, which means a commit on every merge and a
race whenever two pull requests merge at once — and a stored number can drift from the
history it claims to describe, while a derived one cannot.

## Known weaknesses

- **Tool use, not writing, is the likely failure point.** The loop depends on free models
  running two scripts and editing files. Everything else — branching, staging, the trailer,
  the pull request — is automated for exactly this reason, but a model that cannot follow a
  two-step script will still fail, and that failure will look like "the project does not
  work" rather than "the harness does not fit the model". Watch for it early.
- **The plan's county list is a guess.** `plan\regions.json` names Bohemian and Moravian
  counties that have not been checked against the historical record. Research tasks exist to
  correct it, and the file says so.
- **No engine means no proof.** Until a succession loop runs, every claim that the schema
  models Crusader Kings correctly is an argument, not evidence.
- **The comparison with Teutoburg is not controlled.** Different year, different tooling,
  different quantity of James's own attention. The trailers make the comparison possible;
  they do not make it rigorous, and it should not be written up as though they did.
