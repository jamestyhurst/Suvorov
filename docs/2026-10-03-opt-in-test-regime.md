# 2026-10-03 — opt-in test regime

James, from iPhone. An isolated mode is still part of the engine. `cargo test` in `rust/` runs the base world and every feature suite. Adding a `Feature` without a file named by `Feature::suite_file` fails `tests/regime.rs`.

Suites:

- `marriage.rs` enables Marriage only.
- `titles.rs` enables Titles only. Inheritance stays off.
- `inheritance.rs` enables Inheritance, which also enables Titles.
- `scripting.rs` enables Scripting only and runs Rune.

A profile test may enable several features. It does not replace the isolated suite.
