# suvorov-core (Rust)

First-class Rust port of the Suvorov World API from draft PR #2. Candidate
engine core, not yet the recorded engine language.

```bash
cargo test
cargo run --release --bin suvorov-bench -- scan 20000 200 50 365
```

See `../spikes/language-2026-09-29/` for the three-language comparison.
