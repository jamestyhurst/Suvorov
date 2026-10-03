# 2026-10-03 — Rune and isolated suites

Device: iPhone. Author: Grok. Draft, do not merge. Same branch as PR #9.

James chose Rune (rune-rs), not Lua, Python, or Rhai. Opt-in modes have their own test files. `regime.rs` fails if a feature has no suite file.

`bind_script` compiles `pub fn on_fire(year, month, day)`. The tick calls it. Stdio is off.
