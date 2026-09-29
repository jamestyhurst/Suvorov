# Handoff — Suvorov language spike

Session: Grok from iPhone, 2026-09-29.
Branch: `device/iphone/2026-09-29-language-spike`
Status: draft PR, not merged.

## Done

- First-class Rust core at `rust/` matching PR #2 World / Date / Person API.
- Spike copies: Python and C++ under `spikes/language-2026-09-29/`.
- Matching tests. Matching bench checksums.
- Results written. Decision candidate written. PR #2 not edited.

## Do not

- Merge this PR.
- Stack new iPhone *code* onto draft PR #2 or #3 while this spike is the live
  iPhone Suvorov branch.
- Install a compiler on the school PC without asking James.
- Call the results file a designer note.

## Also done later the same day

- PyO3 module `suvorov_core` (`rust` feature `python`).
- `python/suvorov/core.py` and `python/suvorov/load.py`.
- Binding + fictional content-load tests. Does not copy `games/premyslid/`.

## Next if James wants more

- After #2/#3/#4 can see each other: wire `suvorov.tools.content.Game` into `world_from_records`.
- Person death field, if dead characters must stay in World.
- A larger bench (event queue, pathfinding stub) once those exist.
- James ruling: Rust becomes core, or C++ on #2 stays, or keep experimenting.
