# Decision log

One entry per design decision, newest at the bottom. **Don't rewrite old entries.** If a
decision is reversed, add a new entry that names the one it replaces.

## Template

```markdown
### YYYY-MM-DD — <decision in a few words>

- **Decided:** what was chosen
- **Alternatives:** what else was considered
- **Why:** the reason this option won
- **Source of the idea:** Clausewitz (name the feature) | <other game or engine> | general practice | James
- **Decided by:** James | <agent name>
```

---

### 2026-09-18 — Start context-free, inspired by the Clausewitz engine

- **Decided:** Build the general framework that grand strategy games share, with no specific
  location or historical era, taking inspiration from the Clausewitz engine.
- **Alternatives:** Starting with a specific setting and era.
- **Why:** See James's words in `AGENTS.md`.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-29 — Language spike recorded; engine language not flipped

- **Decided:** Open a non-destructive language experiment on
  `device/iphone/2026-09-29-language-spike`. Add a first-class Rust port of the PR #2 World
  API under `rust/`, plus throwaway Python and C++ copies under
  `spikes/language-2026-09-29/`. Leave draft PR #2's C++ slice untouched. Do not merge this
  branch. Do not treat Rust as the engine language until James writes that down.
- **Alternatives:** Rewrite PR #2 in place; keep three living cores; make Python the engine.
- **Why:** James (2026-09-29, iPhone) asked for the spike *and* a Rust rewrite of the PR #2
  API, non-destructively, with tokens to spare. Parallel living cores were rejected in the
  same session as an agent-maintenance trap. Speed on this slice does not separate Rust from
  C++.
- **Source of the idea:** James (experiment); general practice (spike before replacing a
  recorded language choice)
- **Decided by:** James (do the spike and the Rust port); Grok from iPhone (layout:
  `rust/` + `spikes/`, do not touch PR #2)

### 2026-09-29 — Premyslid talks to the core through Python bindings, not a second engine

- **Decided:** Add an optional PyO3 surface on the Rust core and a setting-free loader
  (`python/suvorov/load.py`) that accepts Premyslid-schema-shaped title and character
  dicts. Do not copy `games/premyslid/` onto this branch. Do not make the Python tools
  the tick loop.
- **Alternatives:** Rewrite Premyslid tools against the C++ slice; embed Bohemian content
  in engine tests; leave Python and Rust disconnected until a merge.
- **Why:** James (2026-09-29) asked to go ahead with the Premyslid consumption idea from
  the language-spike session. Engine tests stay fictional. PR #3 stays the content repo.
- **Source of the idea:** James (Premyslid as first consumer); general practice (validate
  in tools, simulate in the core)
- **Decided by:** James (do the bridge); Grok from iPhone (PyO3 + loader mapping)

### 2026-09-30 — Rust as the implementation language

- **Decided:** Implement Suvorov in Rust, added alongside the existing material (nothing
  existing is deleted or rewritten). This supersedes the open language question in
  `AGENTS.md`.
- **Alternatives:** Python (more familiar to James); C++ (James, 2026-09-18: "probably a
  better game engine language", and a chance to practice reading it).
- **Why:** James has directed Rust. It keeps the native-performance profile that made C++
  attractive for an engine, with memory safety and a single built-in build/test tool
  (`cargo`), which also avoids needing a C++ compiler or CMake, absent on the school PC.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-30 — Single Cargo crate, not a workspace

- **Decided:** One crate (`suvorov`) with a library root and a `core` module
  (`src/core/`). Standard library only, no external dependencies.
- **Alternatives:** A Cargo workspace with separate crates (e.g. `core`, later others).
- **Why:** Simplest first (James, 2026-09-01). A workspace adds manifests and indirection
  with no benefit until there are several parts that need separate versioning or build
  isolation. Modules can be promoted to crates later if that need appears.
- **Source of the idea:** general practice (Cargo project layout); simplest-first rule from
  James
- **Decided by:** Claude (agent)

### 2026-09-30 — Adopt the `rust/` crate from PR #4; drop the root crate

- **Decided:** The Rust core is `rust/` (crate `suvorov-core`, from draft PR #4). The
  root-level `suvorov` crate and its `Clock` placeholder from this branch are removed
  (they were this branch's own unmerged files). This supersedes the "single Cargo crate"
  entry above only on *where* the crate lives; the single-crate, no-workspace choice stands.
- **Alternatives:** Keep two crates (root skeleton plus `rust/`); restart `rust/` from scratch.
- **Why:** One writer at a time (rule 6): start from the pushed branch that already holds
  the World core, date, persons, and tests, rather than duplicate it. Simplest first.
- **Source of the idea:** general practice; AGENTS.md rule 6
- **Decided by:** Claude (agent), on James's instruction (2026-09-30) to proceed naturally

### 2026-09-30 — First map slice: adjacency graph, ownership, derived borders

- **Decided:** `World` gains an undirected adjacency graph over locations, an optional
  owning polity per location, and `borders()`, computed on demand as adjacent location
  pairs with different owners. Borders are never stored. No geometry, terrain, or rendering.
- **Alternatives:** Store borders as data; start with a pixel/bitmap province map; start
  with rendering.
- **Why:** Simplest first. A province graph with derived borders gives "flexible borders"
  (they move when ownership changes) with no extra state to keep consistent.
- **Source of the idea:** Clausewitz (province map with province ownership); general
  practice (derive, don't store)
- **Decided by:** Claude (agent)

### 2026-09-30 — Premyslid moves to its own repository (pending; supersedes PR #3's layout)

- **Decided:** James intends to split the medieval game (Premyslid, a Crusader Kings 1
  clone) into its own repository that depends on Suvorov as an engine library. Suvorov
  stays setting-free. Nothing is moved yet; PR #3 stays open and unmerged.
- **Alternatives:** Keep the game inside Suvorov under `games/premyslid/` (PR #3's
  recorded choice).
- **Why:** James: a cleaner architecture. The engine never sees the game's setting.
- **Source of the idea:** James
- **Decided by:** James

### 2026-10-01 — Death is a world-side record; scheduled events fire into a queue

- **Decided:** `World` records an optional death date per person (`kill_person`,
  `is_alive`, `person_death_date`, `living_person_count`); `Person` itself is unchanged.
  `World::schedule_event(date, name)` queues a named event for a strictly future date;
  `advance_one_day` moves due events into a queue read with `drain_fired_events`. Events
  carry only a name; there are no effects or triggers yet.
- **Alternatives:** Add a `death_date` field to `Person` (breaks the PyO3 and bench
  constructors now); have `advance_one_day` return the fired events; run callbacks inside
  the tick.
- **Why:** Simplest first. This answers PR #5 gate 4 (person death field) without an API
  break, and the pull-style queue keeps the tick free of game logic.
- **Source of the idea:** Clausewitz (date-scheduled events, pulses); general practice
  (event queue)
- **Decided by:** Claude (agent), on James's instruction (2026-10-01) to continue

### 2026-10-03 — Scheduled events apply a frozen effect vocabulary

- **Decided:** `World::schedule_event_with_effects` attaches a list of `Effect` values to a
  named future event. The first two effects are `KillPerson` and `SetLocationOwner`.
  `advance_one_day` applies due effects, then queues the event name for
  `drain_fired_events`. Unknown ids are rejected when the event is scheduled. A scheduled
  kill of someone already dead is a no-op so the tick never fails. Name-only
  `schedule_event` remains (empty effect list).
- **Replaces:** the clause in "Death is a world-side record; scheduled events fire into a
  queue" that said events carry only a name and have no effects yet.
- **Alternatives:** callbacks inside the tick; an embedded script VM (Rhai/mlua); a
  separate command stream not tied to dates.
- **Why:** Simplest first. PR #6 left events as names. Clausewitz fires dated events into
  world changes. A frozen enum keeps the tick free of game scripts; the rewrite plan said
  not to add a VM until that vocabulary is too small. `SetLocationOwner` is how a peace
  or cession moves derived borders without storing border data.
- **Source of the idea:** Clausewitz (date-scheduled events with immediate effects);
  `docs/2026-09-30-rust-rewrite-plan.md` on PR #5 (frozen effect vocabulary); PR #6
  suggested next step (event/trigger system)
- **Decided by:** Grok (Zhantianzhe), on James's instruction (2026-10-03) to continue the
  Rust rewrite

### 2026-10-03 — Compiled loader; historical deaths stay in World

- **Decided:** `world_from_records` lives in the Rust crate (`rust/src/load.rs`). Title and
  character records are typed structs (`TitleRecord`, `CharacterRecord`), not serde JSON.
  Characters whose death date is on or before start are loaded and marked dead with
  `World::record_death` using that historical date. `kill_person` stays "die today" and
  delegates to `record_death` with the current world date. Python `load.py` still skips the
  dead; PyO3 does not yet expose the compiled loader.
- **Replaces:** the skip-at-load mapping in `docs/2026-09-29-premyslid-bridge.md` (dead
  characters omitted because Person had no death field).
- **Alternatives:** serde_json dicts in this slice; keep skipping the dead; stamp death as
  the world start date via `kill_person`.
- **Why:** Rewrite plan Phase 3 asked for a compiled equivalent of `world_from_records`
  with fictional fixtures. `kill_person` would record the start date, which is the wrong
  death. serde waits until a JSON-on-disk loader needs it. Simplest first.
- **Source of the idea:** `docs/2026-09-30-rust-rewrite-plan.md` Phase 3 (compiled loader);
  Clausewitz history files keep the dead; PR #7 suggested next step
- **Decided by:** Grok (Zhantianzhe), on James's instruction (2026-10-03, session 2) to
  continue the Rust rewrite

### 2026-10-03 — Rust is canonical; features are opt-in; scripts are named, not run

- **Decided:** The engine language is Rust. This confirms the 2026-09-30 "Rust as the
  implementation language" entry and replaces the open-language sentence in `AGENTS.md`.
  C++ on draft PR #2 stays historical; new engine work lands in `rust/`. Optional
  capabilities are a `FeatureSet` on `World`: Marriage, Titles, Inheritance, Scripting.
  A Crusader Kings-like profile enables all four. A Hearts of Iron-like profile enables
  Scripting only. `Effect::RunScript` records a bound script name; the tick does not
  interpret the body. Inheritance implies Titles. A bare `World::new` enables nothing
  optional.
- **Alternatives:** Keep C++ as the core and use the compiler there; always-on marriage
  and titles; embed Rhai or Lua in this slice.
- **Why:** James (2026-10-03, iPhone): start from scratch in Rust so the compiler
  complements unsupervised agents; Clausewitz ran both Hearts of Iron and Crusader Kings,
  so marriage, titles, and inheritance must be opt-in; a scripting language is in scope
  where Rust is the wrong tool, but not inside every function and not for every game.
- **Source of the idea:** James (language and opt-in); Clausewitz (one engine, many
  games, plaintext script beside the core)
- **Decided by:** James

### 2026-10-03 — Rune is the scripting language; opt-in modes have their own tests

- **Decided:** Rune (`rune` 0.14, rune-rs) is the canonical scripting language. Not Lua,
  not Python, not Rhai. A bound script must define `pub fn on_fire(year, month, day)`
  and return a string. `bind_script` compiles it. `Effect::RunScript` calls it and
  records `name=return`. Stdio is off. Every `Feature` has an isolated suite file under
  `rust/tests/`; `regime.rs` fails if that file is missing. Inheritance's suite may also
  enable Titles, because Inheritance implies Titles.
- **Replaces:** the clause in "Rust is canonical; features are opt-in" that said the tick
  does not interpret the script body.
- **Alternatives:** Lua; Python; Rhai; leave scripts stored and unrun; test opt-in modes
  only inside a full profile.
- **Why:** James (2026-10-03, iPhone) asked for a testing regime for isolated modes, and
  chose Rune over the other embeddable languages because he prefers the name.
- **Source of the idea:** James
- **Decided by:** James

### 2026-10-03 — Rune world API is a binding, not a second world

- **Decided:** Scripts may call `world::date_text` and `world::contract_marriage`.
  Those are Rust functions installed into the Rune context. `contract_marriage`
  returns `refused` unless Marriage is enabled, and the world applies the pair
  only after `on_fire` returns. The script does not receive the `World` struct.
- **Alternatives:** Pass the whole world into Rune; let the script mutate persons
  directly; keep scripts unable to see the world.
- **Why:** James (2026-10-03, iPhone) asked what a Rune world API means and told
  the session to add the slice. Binding is the word Rune uses for installing a
  native function, and the word this engine already uses for attaching a script.
- **Source of the idea:** James; Rune (`Module::function`); Clausewitz (script
  effects call engine commands, they do not own the gamestate)
- **Decided by:** James (do the slice); Grok from iPhone (two functions, command
  applied after the call)

### 2026-10-03 — World API asks, plus move and allegiance effects

- **Decided:** A Rune script may ask for marriage, a title grant, an heir, a move,
  a death, or an allegiance change. The world applies the asks in order after
  `on_fire` returns. Marriage, titles, and inheritance still refuse when disabled.
  Move, kill, and allegiance are core. `Effect::MovePerson` and
  `Effect::SetAllegiance` schedule the same two core changes without a script.
  `grant_title` returns the title id so the same script can name an heir.
- **Alternatives:** One ask per script; let the script hold `World`; make movement
  a feature.
- **Why:** James (2026-10-03, iPhone) asked for more work per prompt and had
  already accepted a title binding. Persons already have a location and an
  allegiance, so those changes are not a new game.
- **Source of the idea:** James; Clausewitz (scripted effects call engine
  commands)
- **Decided by:** James (continue); Grok from iPhone (the ask list and two effects)

### 2026-10-03 — General grand-strategy layer before more dynastic rules

- **Decided:** Offices are core: a polity seats a living member in a named office.
  Diplomacy (shared stance plus directed opinion), forces (owner, game-defined
  kind, location, strength), intelligence (hidden operations), and fog of war
  are features. Both Crusader Kings-like and Hearts of Iron-like profiles enable
  those four. Fog off sees every location. Fog on sees owned and revealed land.
  Force kinds are strings, not a closed list.
- **Alternatives:** Hard-code army and fleet; make a ruler a dynastic title;
  leave fog always on.
- **Why:** James (2026-10-03, iPhone) asked for the general grand-strategy
  layer first: political entities, leaders, diplomacy, forces, spies, fog.
  Polities already existed. The rest did not.
- **Source of the idea:** James; Clausewitz (countries, diplomacy, units,
  intelligence, fog)
- **Decided by:** James
