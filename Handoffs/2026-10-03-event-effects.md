# Handoff — scheduled event effects

Session: Grok on Zhantianzhe, 2026-10-03.
Branch: `device/zhantianzhe/2026-10-03-event-effects`
Base: `rust/2026-09-30-skeleton` (draft PR #6)
Status: draft PR, not merged.

## Done

- Frozen `Effect` vocabulary on scheduled events: `KillPerson`, `SetLocationOwner`.
- `World::schedule_event_with_effects`; name-only `schedule_event` still works.
- Tests in `rust/tests/life_and_events_tests.rs` (TDD, public API).
- Decision-log entry 2026-10-03.
- Per-user GNU rustup on this machine (James approved this session).

## Do not

- Merge this PR or PR #6.
- Touch the dirty `feat/premyslid-schema-v0` checkout under `Software_Engineering/Suvorov`.
- Install a compiler on the school PC.
- Add Bevy, Rhai, petgraph, or a second content format.
- Read Teutoburg.

## Zhantianzhe toolchain trap

GNU `ld` cannot open the rustup sysroot under the non-ASCII home path `C:\Users\詹天哲`. This session moved the install:

```text
RUSTUP_HOME=C:\rustup
CARGO_HOME=C:\cargo
CARGO_TARGET_DIR=C:\cargo-target\suvorov-core
PATH prepend C:\cargo\bin
host: stable-x86_64-pc-windows-gnu
```

Those three env vars are User-scoped. New shells should pick them up.

## Suggested skills

- `/tdd` — mandatory for AI-generated Suvorov code
- `/caveman` — default chat on this machine

## Next if James wants more

1. Rust loader equivalent of `python/suvorov/load.py` (`world_from_records`), fictional fixtures only.
2. Load already-dead characters through `kill_person` instead of skipping them.
3. Terminal client crate over `borders()` (not core).
4. CI job that is not named only for the language-spike branch (the existing workflow already runs `cargo test` on pull requests).
5. Fate of C++ (#2) and Premyslid-in-tree (#3).
