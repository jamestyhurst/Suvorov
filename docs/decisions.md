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
