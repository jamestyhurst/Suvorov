# Language spike — agent input, 2026-09-29

Written by Grok from iPhone. This is agent input, not a designer note and not
canonical. James decides whether Rust becomes the engine language.

## Why this branch exists

Suvorov `main` is docs-only. Draft PR #2 is a C++20 World slice. Draft PR #3 is
Python tools + Premyslid, stacked on #2. James asked to experiment with Rust
versus C++ versus Python without destroying the C++ slice.

This branch does two things on a new iPhone device branch off `main`:

1. A three-language spike of the same World API, with matching tests and a bench.
2. A first-class Rust rewrite of that API under `rust/`.

It does not edit PR #2 files. It does not merge. It does not declare a winner.

## API that was ported

From `feat/world-date-and-polities` (PR #2 head `1ebba6a`):

- `Date` value type, Gregorian leap rules, `advance_one_day`
- `biological_age(birth, on)` as completed years
- `Person`: one or more names, one or more polity allegiances, birth date,
  birth location id, current location id
- `World`: named polities, named locations (ids only, no map), persons,
  `advance_one_day` as the only tick
- Invalid dates and unknown ids rejected on the public API

Rust uses `Result` + `Error::{InvalidArgument, OutOfRange}` instead of C++
exceptions. Behavior matches.

## Numbers (sandbox, 2026-09-29)

See `spikes/language-2026-09-29/RESULTS.md`. Headline: Rust 49 ms / C++ 53 ms /
Python 5140 ms for 20k persons × 365 scanned days. Checksums identical.

## Candidate recommendation (not a decision)

Keep Python for tools, content, and a later Pygame skin.

Pick **one** compiled language for the simulation core. On this evidence, pick
Rust if the goal is an agentically-written core that stays memory-safe. Keep C++
if the goal is "what agents build when told to imitate Clausewitz," which is
itself a C++ engine.

Do not keep three living cores. The Python and C++ trees under `spikes/` should
die when James picks. The C++ on PR #2 stays until he says otherwise.

School PC still cannot compile Rust or C++ without an approved install. Python
is the only language that machine can run today.

## Out of scope on this branch

- Map, travel time, economy, diplomacy, military
- PyO3 / cxx bindings
- Replacing PR #2
- Merging anything
