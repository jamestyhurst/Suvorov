---
regime: product
---

# CLAUDE.md

@AGENTS.md

The brief above governs. The notes below apply only to Claude Code.

- **Shell:** use the PowerShell tool. The Bash tool fails on the school PC with a cygwin
  `add_item` error, so don't retry it.
- **Questions for James:** ask through the AskUserQuestion tool. Write every option in plain
  words that say what will actually happen and who does it, and avoid process jargon.
- **Memory:** this repository gets its own auto-memory folder. It starts empty, and that's
  intentional.

## Suvorov project rules

- Keep simulation behavior deterministic when given the same initial state and inputs.
- Keep the simulation core independent of rendering, input, and platform code.
- Express time and state changes through public interfaces so they can be tested without
  reaching into implementation details.
- Add or update tests for every simulation behavior.

The initial engine slice models a world date and named polities. A simulation tick advances
the world by one day. Map, economy, diplomacy, military, and rendering systems are future
modules; do not add them to the core until their behavior is specified.
