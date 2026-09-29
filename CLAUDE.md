---
regime: product
---

# CLAUDE.md

@AGENTS.md

The brief above governs. The notes below apply only to Claude Code.

## Cloud / iPhone sessions — token cap

James, 2026-09-20. Also in `.claude/rules/iphone-cloud-sessions.md` (cloud sessions load that file automatically).

- Do not watch a pull request.
- Do not run /autofix-pr.
- Do not enable Auto-fix.
- Do not poll GitHub Checks, Actions, or `gh run`.
- Do not keep running tests, linters, or builds until they pass.
- If the user asked for a code change, you may run the project test command once. Then stop and wait.
- If tests fail, report the failure and wait. Do not start another fix-and-retest loop unless the user says to.
- Prefer a short plan and a small diff over unattended iteration.

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
