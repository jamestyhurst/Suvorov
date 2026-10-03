# Suvorov
A general purpose framework for creating a grand strategy game engine from scratch using agentic coding techniques, inspired by the Clausewitz engine.

Agents: start with [AGENTS.md](AGENTS.md). Design history: [docs/decisions.md](docs/decisions.md).

## Build and test (Rust)

Requires a stable Rust toolchain (`cargo`). The core uses the standard library only; `pyo3` is an optional feature (see `rust/README.md`).

```
cd rust
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Last updated 2026-10-03. Engine language: Rust (James, iPhone).
