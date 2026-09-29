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
