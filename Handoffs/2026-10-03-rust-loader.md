# Handoff — compiled world_from_records

Session: Grok on Zhantianzhe, 2026-10-03 (session 2).
Branch: `device/zhantianzhe/2026-10-03-rust-loader`
Base: `device/zhantianzhe/2026-10-03-event-effects` (draft PR #7)
Status: draft PR, not merged.

## Done

- Rust `world_from_records` (`rust/src/load.rs`): typed `TitleRecord` / `CharacterRecord`, fictional Aurora fixtures in `rust/tests/load_tests.rs`.
- Dead characters load through `World::record_death` with the historical date. `kill_person` still means die today.
- No serde. No JSON-on-disk parse. Python `load.py` still skips the dead (PyO3 does not export this path yet).
- Decision-log entry 2026-10-03 (compiled loader).

## Do not

- Merge this PR or PRs #4–#7.
- Touch the dirty `feat/premyslid-schema-v0` checkout under `Software_Engineering/Suvorov`.
- Install a compiler on the school PC.
- Add Bevy, Rhai, petgraph, serde, or a second content format.
- Read Teutoburg.

## Zhantianzhe toolchain trap

GNU `ld` cannot open the rustup sysroot under the non-ASCII home path `C:\Users\詹天哲`. User-scoped env:

```text
RUSTUP_HOME=C:\rustup
CARGO_HOME=C:\cargo
CARGO_TARGET_DIR=C:\cargo-target\suvorov-core
PATH prepend C:\cargo\bin
host: stable-x86_64-pc-windows-gnu
```

Worktree: `Software_Engineering/Suvorov_Worktrees/2026-10-03-rust-loader`.

## Suggested skills

- `/tdd` — mandatory for AI-generated Suvorov code
- `/caveman` — default chat on this machine

## Next if James wants more

1. PyO3: export `world_from_records`, `record_death`, `kill_person`; point Python `load.py` at the compiled loader (or delete the skip-dead Python mapping).
2. Set location owners from title holders at load so `borders()` is non-empty without a later event.
3. Terminal client crate over `borders()` (not core).
4. CI job that is not named only for the language-spike branch (the existing workflow already runs `cargo test` on pull requests).
5. Fate of C++ (#2) and Premyslid-in-tree (#3).
