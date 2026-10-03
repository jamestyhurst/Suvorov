# suvorov-core (Rust)

Rust simulation core (canonical engine, James 2026-10-03): world date, named
polities and locations, persons, derived borders, scheduled events with a frozen
`Effect` list, `world_from_records`, and an opt-in `FeatureSet` (marriage, titles,
inheritance, script seam). A bare world enables none of those.

```bash
cargo test
cargo run --release --bin suvorov-bench -- scan 20000 200 50 365
```

See `../spikes/language-2026-09-29/` for the three-language comparison.

Python bindings (optional feature `python`):

```bash
maturin develop --features python
PYTHONPATH=../python python -m unittest discover -s ../python/tests -t ../python
```
