# Suvorov
A general purpose framework for creating a grand strategy game engine from scratch using agentic coding techniques, inspired by the Clausewitz engine.

Agents: start with [AGENTS.md](AGENTS.md). Design history: [docs/decisions.md](docs/decisions.md).

## Build and test (Rust)

Requires a stable Rust toolchain (`cargo`); standard library only, no external crates.

```
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Last updated 2026-09-30.
