# Suvorov

A general-purpose framework for creating a grand strategy game engine from scratch
using agentic coding techniques, inspired by the Clausewitz engine.

Agents: start with [AGENTS.md](AGENTS.md). Design history: [docs/decisions.md](docs/decisions.md).

## Current foundation

The first vertical slice is a deterministic simulation core:

- `World` owns the simulation date and named polities.
- `advance_one_day()` moves the simulation clock by exactly one calendar day.
- Invalid dates and unknown polity identifiers are rejected through the public API.
- Rendering, input, map, economy, diplomacy, and military systems remain outside the core.

## Build and test

The project uses CMake and requires a C++20 compiler on `PATH`. The three commands below
configure the build, compile the library and tests, then run the test executable.

```powershell
cmake -S . -B build -DBUILD_TESTING=ON
cmake --build build --config Debug
ctest --test-dir build -C Debug --output-on-failure
```

The school PC this slice was written on does not yet have that compiler. Do not install one
without asking James; new software can raise an IT alert. See `AGENTS.md`.
