# 2026-10-03 — Rust canonical feature gate

Device: iPhone. Author: Grok. Draft, do not merge.

Stacked on `device/zhantianzhe/2026-10-03-rust-loader` (PR #8). Does not edit #4, #5, or #6 in place.

## Ruling

James 2026-10-03: Suvorov's engine is Rust. Features are opt-in per game. A scripting language belongs beside the core, not inside every function.

## This slice

- `FeatureSet` / `GameProfile` on `World::with_features` and `World::for_profile`.
- Marriage, titles, inheritance (inheritance implies titles), scripting.
- `Effect::RunScript` queues the script name. Body is not executed.
- Hearts of Iron-like profile refuses marriage and titles. Crusader Kings-like profile passes a title to a designated heir on death.

## Not in this slice

No Rhai/Lua crate. No C++ deletion. No Premyslid content. No renderer.
