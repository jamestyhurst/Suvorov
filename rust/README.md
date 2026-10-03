# suvorov-core (Rust)

Rust simulation core: world date, named polities and locations, persons, derived
borders, scheduled events with a frozen `Effect` list, and `world_from_records`
(schema-shaped title/character structs, fictional fixtures only).

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
