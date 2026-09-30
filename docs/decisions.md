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
