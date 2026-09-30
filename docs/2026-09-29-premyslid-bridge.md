# Premyslid bridge — agent input, 2026-09-29

Written by Grok from iPhone. Not a designer note. Not a merge of draft PR #3.

## What this is

Premyslid (draft PR #3) is schema-validated content plus Python tools. It does not
import a simulation core. The language spike (draft PR #4) now adds the missing
seam:

- PyO3 module `suvorov_core` built from `rust/`
- `python/suvorov/core.py` — import that module
- `python/suvorov/load.py` — map title + character *records* onto `World`

The record shape is Premyslid schema v0 (id, name, birth, death, holder). The
records used in tests are fictional (`d_aurora`, `Calen`, `Mira`). This branch
does not copy `games/premyslid/` and does not put Bohemia into the engine.

## How Premyslid would use it after the branches meet

1. `python -m suvorov.tools.validate` stays the review step (PR #3).
2. A later caller does `world_from_records(start_date, titles, characters)` with
   the already-parsed dicts from `suvorov.tools.content`.
3. The Rust `World` ticks. Python tools never become the tick loop.

## Build

```bash
cd rust
maturin develop --features python
PYTHONPATH=../python python -m unittest discover -s ../python/tests -t ../python
```

`cargo test` without `--features python` still runs the pure Rust suite and does
not link Python.

School PC: no Rust toolchain. Do not install one without asking James. The
import error tells you that.

## Mapping choices (candidate, not a ruling)

- Title id → polity and location. Person.allegiances are the titles a character
  holds at start.
- Unlanded living characters get polity `unlanded` and location `unlocated`.
- Characters whose death date is on or before start are skipped. Person has no
  death field yet; that is the next engine gap Premyslid will report if genealogy
  of the dead must stay in World.
