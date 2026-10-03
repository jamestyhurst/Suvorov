# 2026-10-03 — Rust canonical, features opt-in

James, from iPhone, 2026-10-03. Agent writeup of that ruling. Not a designer note.

The engine is Rust. Clausewitz being C++ is the analogy (one core, many games), not the language to copy. The compiler is part of the unsupervised-agent loop. Draft PR #2's C++ slice is not the place new engine work lands.

A game opts into capabilities. Marriage, titles, and inheritance are on for a dynastic game and off for a state-and-war game. Both kinds of game may enable the script seam. The seam stores a named body and can fire that name from a scheduled effect. This slice does not embed a VM. A later host (Rhai or Lua class) can read the body; the tick still will not.

Profiles in code: `profile_crusader_kings_like`, `profile_hearts_of_iron_like`. Fictional fixtures only in tests.
