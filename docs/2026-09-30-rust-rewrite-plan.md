# Suvorov Rust rewrite — implementation plan

Written by Grok from iPhone, 2026-09-30. **Agent input, not a designer note, not canonical.** James decides whether any of this becomes a decision-log entry.

This branch does not flip the engine language. It does not edit draft PRs #2, #3, or #4. It does not merge.

## Why this file exists

James asked (2026-09-30, iPhone) for an implementation plan to rewrite Suvorov into Rust, using Gemini answers he had just read:

1. Clausewitz is written primarily in C++. Game events and logic run on Paradox's own plaintext scripting language. Some tools, shaders, or legacy pieces have used Lua or C#.
2. To *imitate* Clausewitz in Rust, Gemini named four architectural problems — massive text/data parsing, a heavy simulation loop, a discrete province map, and an information-dense UI — and recommended Bevy, `jomini` / `clauser` + serde, Rhai or mlua, WGSL province shaders, egui / Xilem / Taffy, and petgraph.

Those answers are research input. They are not Suvorov decisions. Clausewitz-the-engine and "what an LLM listed as the Rust clone kit" are two different things.

## What Suvorov already is

Standing James rulings that this plan must not walk past:

- Suvorov is a **standalone engine**. Premyslid imports it. Engine gaps get fixed here, then pulled. (James, 2026-09-25, iPhone.)
- Context-free core: no real places, peoples, or eras in engine code, data, or docs. (`AGENTS.md` rule 1.)
- Simplest first; tests before implementation. (`AGENTS.md` rules 3–4.)
- Locations on the current Person record are **named ids, not a map**. (`docs/decisions.md` on PR #2, 2026-09-19.)
- Rendering is not the engine. A pending #3 note names Pygame as the later skin when a machine can take it. James is skeptical that Pygame or Godot *counts as* building an engine. (James, 2026-09-25.)
- School PC has no C++ or Rust compiler. Do not install one without asking. (`AGENTS.md` environment.)

Current tree, not rewritten by this PR:

| Surface | Where | Language |
| --- | --- | --- |
| Docs-only default branch | `main` | — |
| World date, polities, Person | draft PR #2 `feat/world-date-and-polities` | C++20 + CMake |
| Premyslid schema, validator, queue | draft PR #3, stacked on #2 | Python tools + JSON schema |
| Same World API in Rust + bench + PyO3 load seam | draft PR #4 `device/iphone/2026-09-29-language-spike` | Rust crate `rust/` |

PR #4 already measured 20,000 persons × 365 scanned days (checksum `363840000`):

| Language | Time |
| --- | ---: |
| Rust release | 48.9 ms |
| C++20 Release | 52.6 ms |
| Python 3.12 | 5140 ms |

Person still has no death field. Dead Premyslid characters are skipped at load. Engine language is **not** flipped.

## Map Gemini's four problems onto Suvorov layers

Clausewitz's useful split, from the first Gemini answer and from how Paradox games are actually shipped:

- **Compiled core** for the clock, state, and hot loops.
- **Plaintext data / scripts** for almost all content.
- **A renderer and UI** that sit on top of that state. They are not the simulation.

Gemini then sold a *game* stack as if it were the engine. For Suvorov that is the wrong cut.

| Gemini item | What it actually is | Suvorov placement |
| --- | --- | --- |
| Bevy ECS | A Rust *game* engine with a renderer and a scheduler | **Not the Suvorov core.** Optional later client. Adopting Bevy as "foundational engine" would make Premyslid a Bevy game and would ignore the 2026-09-25 standalone-engine ruling. |
| `jomini` (rakaly) / `clauser` | Parsers for *Paradox's* Clausewitz/Jomini file dialects | Research tool or optional content adapter. Not required to *be* an engine. Premyslid already has a JSON schema and a validator on PR #3. |
| serde | Rust data in/out | Fine, when a Rust loader exists. |
| Rhai / mlua / homemade AST | Embedded scripting | Not needed while content is schema-validated data with a frozen effect vocabulary (`grant_title`, `kill_character`, `change_opinion` on PR #3). Add a script VM only after that vocabulary is too small. |
| Province bitmap + WGSL + data texture | Clausewitz map *client* | Out of the engine. Locations are ids. A province bitmap is a game renderer problem. |
| egui / Xilem / Taffy | Immediate or flex UI | Out of the engine. Pygame is the pending first skin. |
| petgraph | Graph library | Candidate **after** James specifies adjacency / travel time. Do not invent a map to justify the crate. |

Name collision to keep straight: Paradox's later engine is also called Jomini. The Rust crate `jomini` (rakaly) and the JS library of the same name parse Paradox files. None of those is Suvorov.

## Proposed architecture (candidate, not decided)

Keep three layers. Only layer 0 is a language question.

```
Layer 0  suvorov-core     compiled World tick     candidate: rust/ from PR #4
Layer 1  content + tools  schema, validate, load  Python on PR #3; optional Rust serde later
Layer 2  clients          render, input, UI       Pygame first; Bevy/egui later if wanted
```

Premyslid stays a consumer. It does not become the engine, and the engine does not grow a Bohemia.

### Layer 0 — core crate

Promote the existing `rust/` tree on PR #4 rather than starting a second World.

Already present and should stay the public surface until James adds fields:

- `Date` with Gregorian leap rules and `advance_one_day`
- `biological_age(birth, on)` as completed years
- `Person`: ≥1 name, ≥1 polity allegiance, birth date, birth location id, current location id
- `World`: named polities, named locations (ids only), persons, `advance_one_day` as the only tick
- `Result` + `Error::{InvalidArgument, OutOfRange}`

Do **not** take a Bevy `App`, `System`, or `Resource` in this crate. Simulation code is functions over `World` (or later, over explicit system-input structs). If an ECS *storage* pattern is ever justified, it can be a plain Rust module measured against the 20k-person bench. Bevy's scheduler is not a requirement for a daily tick of tens of thousands of records.

PyO3 feature `python` stays optional so Python tools can call the tick without becoming the tick.

### Layer 1 — data

Do not stand up a second content format in order to look like Clausewitz.

1. Keep Premyslid schema v0 and `python -m suvorov.tools.validate` as the review step.
2. Keep `python/suvorov/load.py` as the first bridge (`world_from_records`).
3. After #3 and #4 can see the same tree, a Rust loader that accepts the same dict/JSON shape is the next compiled-side slice. serde belongs there.
4. A Clausewitz-syntax experiment (`jomini` or `clauser`) is a **tool spike**, in `spikes/`, with fictional fixtures only. It is how agents study the format. It is not how Premyslid ships content unless James later asks for Paradox-file compatibility.

Dynamic modifiers ("+10% tax if greedy") stay out until there is an economy field to modify.

### Layer 2 — clients

Out of this plan's build list. When a machine can compile and display:

- Pygame first, as already pending.
- Bevy + egui + a province shader is a *client* research item, in a consumer crate or game, not in `suvorov-core`.

## Phases

Each phase ends with tests written first, then code, then a decision-log entry **only if James accepts the result**. School-PC work stays Python-only until he approves a toolchain.

### Phase 0 — gates (James)

Nothing after this phase is licensed without answers.

1. **Language.** Flip the simulation core to Rust (promote PR #4's `rust/`), keep C++ as the core (PR #2), or keep experimenting? PR #4 already recommended: one compiled language; Python for tools; do not keep three living cores.
2. **Bevy.** Reject as core (this plan's recommendation), allow as an optional later client, or adopt as foundation? Adopting as foundation conflicts with "standalone engine" and with "Pygame/Godot is not the engine."
3. **Content format.** Stay on Premyslid JSON schema; add a Clausewitz-syntax spike as a tool; or replace the schema with Paradox plaintext.
4. **Death.** Add a death field on `Person` so dead characters can remain in `World`, or keep skipping them at load?
5. **PR hygiene.** Close, park, or keep #2/#3/#4 as-is until a flip is accepted. This plan's PR is docs only and must not be used as a silent merge vehicle.

### Phase 1 — promote the existing Rust World (needs gate 1 = Rust)

- Treat `rust/` on PR #4 as the candidate core, not a disposable spike copy.
- Document the public API next to PR #2's C++ headers so the two can be compared line by line.
- Keep the 20k × 365 bench as a regression check.
- Do not delete the C++ on PR #2 until James says the C++ slice is retired.
- Do not install a compiler on the school PC to "finish" this phase.

Exit: James writes or accepts a decision-log entry that names the engine language. Until that entry exists, C++ on #2 and Rust on #4 are both experiments.

### Phase 2 — first engine gap Premyslid already reported

- `Person` death date (optional). Skip-at-load remains valid if the field is absent.
- Tick still only advances the calendar unless a later spec says otherwise. No destined death age. If mortality is added later, it is chance-based on the live tree (James, Teutoburg 2026-09-12, is **not** imported as a Suvorov spec; record a Suvorov-local ruling if he wants the same model here).
- Tests first: born after world date rejected; death before birth rejected; age derived only while living.

### Phase 3 — compiled loader for schema-shaped records

- Rust function equivalent to `world_from_records`.
- Fictional fixtures only (`Aurora` / `Helia` / `Calen` / `Mira` already used on #4).
- Does not copy `games/premyslid/`.
- Python validate remains the review step.

### Phase 4 — location graph, only after James specifies travel

- Adjacency list and travel duration on named location ids.
- petgraph is allowed as an implementation crate *after* the fields exist.
- Still no bitmap, no owners-as-colors, no shader.

### Phase 5 — client, not engine

- Separate crate or game tree.
- Pygame first when a machine can take it.
- Bevy/egui/WGSL only if James wants a second client.

## What this session will not do

- Merge any Suvorov PR.
- Edit `#2`, `#3`, or `#4`.
- Declare Rust the engine language in `docs/decisions.md`.
- Add Bevy, egui, petgraph, Rhai, mlua, `jomini`, or `clauser` as dependencies.
- Invent a province map.
- Install a compiler.
- Read Teutoburg.

## Suggested next commit, after the gates

If James answers Phase 0 with "Rust core, Bevy rejected, keep Premyslid JSON, add death field":

1. Open a **new** iPhone code branch off whatever he names as the base (likely #4, not this docs PR).
2. Write death-field tests in `rust/tests/`.
3. Implement the field and the load-path change.
4. Leave C++ on #2 untouched until a retirement decision.

If he answers "keep C++": this plan stays on the shelf and work returns to PR #2.
