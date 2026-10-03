# 2026-10-03 — Rune world API

Binding runs both ways. `bind_script` attaches a Rune body to a name on the world. The world API attaches Rust functions to the name `world::` inside that script.

This slice exposes `world::date_text()` and `world::contract_marriage(a, b)`. The marriage function returns `refused` unless Marriage is enabled, and the world applies the pair only after the script returns. Stdio stays off. The script does not hold the world.
