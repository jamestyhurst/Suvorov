# Suvorov — agent brief

> Read by every coding agent that works here. Claude Code loads it through `CLAUDE.md`;
> Codex, Warp, and other agents read it directly. Written 2026-09-18. **James's own words
> are quoted and are canonical.** Everything else in this file is logistics, not design.

## What Suvorov is

An experiment in what gets built from a single starting instruction. James's instruction:

> Take inspiration from the Clausewitz engine

The **Clausewitz engine** is Paradox Development Studio's in-house grand-strategy game engine.
It is not a reference to Carl von Clausewitz the military theorist.

The scope, in James's words:

> I want to start with a "context-free" Suvorov, ie. no specific location or historical era,
> starting with the general framework that makes all of those grand strategy games possible

## Rules

1. **No setting.** Build the general framework. Any content used to test or demonstrate it is
   placeholder and clearly fictional: no real places, peoples, or historical eras in code,
   data, or docs. If a real setting starts to seem necessary, ask James rather than choosing
   one.
2. **Do not read the sibling project.** Never open, search, list, or import anything under
   `..\Teutoburg\` or `..\Teutoburg_Worktrees\`. That includes following a path or link to it
   from any other file. Suvorov exists to show what gets built *without* that project's
   decisions, so reading it would spoil the result.
3. **Simplest first** (James, 2026-09-01): *"start with the simplest possible system, then add
   layers of complexity over time as the simpler one is proven functional."*
4. **Tests before code.** TDD is mandatory for AI-generated code (James, 2026-09-01): write the
   test before the implementation it covers.
5. **James's writing outranks agent writing.** Anything James writes himself overrides
   agent-written documents, including this one, when they conflict.
6. **One writer at a time.** When several agents or devices work here, a second agent starts
   from the first agent's pushed branch, never from `main` in parallel.

## Keep the record — this is what the experiment measures

Log every design decision in `docs\decisions.md`, newest at the bottom, using the template at
the top of that file. The field that matters most is **where the idea came from**. Name the
specific Clausewitz feature, the other game or engine, general engineering practice, or James.

Language, libraries, file formats, and architecture are all open decisions. Record each one
like any other decision; none of them is settled by this brief.

## Environment

- Windows school PC with **no admin rights**. Never use anything that triggers UAC, writes to
  Program Files or other system folders, or needs elevation. Use per-user installs.
- Default shell is PowerShell 7.
- Python is a per-user install (3.12 on the school PC; other devices may differ).
