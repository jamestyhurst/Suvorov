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
