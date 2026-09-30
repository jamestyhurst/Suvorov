# Gemini Clausewitz / Rust notes — source record

Written by Grok from iPhone, 2026-09-30. Agent input. Not a designer note.

James photographed Gemini answers on 2026-09-30 (~07:28–07:49 local) and asked that they feed a Suvorov Rust rewrite plan. The plan itself is `docs/2026-09-30-rust-rewrite-plan.md`. This file only records what the screenshots said, so a later agent does not have to re-read the photos.

## Query 1 — "what language is clausewitz engine"

- Clausewitz Engine is written primarily in C++.
- C++ core: underlying engine and performance-heavy systems (AI, graphics).
- In-house scripting: game events, decisions, and logic in Paradox's proprietary plaintext language (key-value and bracket syntax).
- Additional languages: some tools, shaders, or legacy pieces have used Lua or C# depending on game and version.

## Query 2 — "I want to imitate the Clausewitz engine for strategy games"

Gemini framed four problems and a Rust kit:

1. Data parsing and modding — plaintext scripting files; recommended `jomini` ("over 1 GB/s") or `clauser` + serde; dynamic modifiers via Rhai, mlua, or a homemade AST.
2. High-performance simulation — thousands of entities on a daily or hourly tick; recommended Bevy ECS and multi-core systems, contrasted with old Clausewitz struggling to scale across cores.
3. Map interface — discrete provinces as a flat bitmap, color = id; Bevy + WGSL; data texture of owners; cursor raycast to pixel color to province id.
4. UI — "beautiful spreadsheets"; egui + `bevy_egui`; Xilem / Taffy as a flex-style alternative.

Blueprint table in the last screenshot:

| Layer | Recommended Rust tools |
| --- | --- |
| Foundational engine and ECS | Bevy |
| Data scripting and saves | jomini + serde |
| User interface and menus | bevy_egui / egui |
| Pathfinding (armies/fleets) | petgraph |

Gemini offered a follow-up on Bevy ECS database layout or Clausewitz moddable event scripts.

## Fact check against public crates (2026-09-30)

- `jomini` (rakaly/jomini) is a real, maintained parser for Paradox save and game files. The 1 GB/s claim is the crate's own.
- `clauser` (azrogers/clauser) is a real serde deserializer for Clausewitz text. One crates.io release (0.1.0, 2024-06), small download count.
- Bevy, egui, petgraph, Rhai, mlua are real crates.
- Paradox also named a later in-house engine Jomini. That is not the Rust crate.

The rewrite plan does not take these as Suvorov dependencies.
