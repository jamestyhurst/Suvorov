# Decision log

One entry per design decision, newest at the bottom. **Don't rewrite old entries.** If a
decision is reversed, add a new entry that names the one it replaces.

## Template

```markdown
### YYYY-MM-DD — <decision in a few words>

- **Decided:** what was chosen
- **Alternatives:** what else was considered
- **Why:** the reason this option won
- **Source of the idea:** Clausewitz (name the feature) | <other game or engine> | general practice | James
- **Decided by:** James | <agent name>
```

---

### 2026-09-18 — Start context-free, inspired by the Clausewitz engine

- **Decided:** Build the general framework that grand strategy games share, with no specific
  location or historical era, taking inspiration from the Clausewitz engine.
- **Alternatives:** Starting with a specific setting and era.
- **Why:** See James's words in `AGENTS.md`.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-18 — Implement the first slice in C++20 with CMake

- **Decided:** The simulation core is a C++20 library configured by CMake. Tests are a CMake
  executable registered with CTest. Python remains allowed later for tools; it is not the
  engine language for this slice.
- **Alternatives:** Python for the whole core (James is more fluent in it); defer the language
  until a compiler is installed on the school PC.
- **Why:** James (2026-09-18, quoted in `AGENTS.md`) named C++ as the better game-engine
  language and a reasonable chance to practise reading C++. Copilot on this device (Zhantianzhe)
  had already written the C++ slice uncommitted; this entry records that choice instead of
  leaving language "open" while C++ files exist. The school PC still has no compiler or CMake
  on PATH — building is blocked until James approves a per-user install.
- **Source of the idea:** James (language lean); Copilot (this device's first implementation);
  general practice (CMake + CTest as the C++ build/test pair)
- **Decided by:** James, recorded this session

### 2026-09-18 — First vertical slice is world date and named polities

- **Decided:** The first working core is `World`: a validated calendar date, named polities
  with integer ids, and `advance_one_day()` as the only tick. Map, economy, diplomacy,
  military, rendering, and input stay out until specified.
- **Alternatives:** Start with a map; start with a CLI; start with characters or armies.
- **Why:** James's standing rule in `AGENTS.md` is simplest-first. A clock plus named actors
  is the smallest state later systems can hang off, and it is testable through a public API
  with no renderer.
- **Source of the idea:** James (simplest first, 2026-09-01); general practice (a simulation
  clock as the first engine object). Not copied from a named Clausewitz subsystem.
- **Decided by:** James, this session (chose the World-date-and-polities slice as the work to
  finish)

### 2026-09-19 — Named persons belong to a polity

- **Decided:** The next core object after date + named polities is a person: an integer id, a
  non-empty name, and a required existing polity id. `World` exposes `add_person`,
  `person_name`, and `person_polity`. Persons have no location, age, title, death, traits, or
  tick side effects. `advance_one_day()` still only moves the calendar.
- **Alternatives:** A player-controlled polity field; a scheduled-event queue; provinces or a
  map; leave polities as labels with no people.
- **Why:** James's standing rule is simplest-first. A polity with no people is only a label.
  Persons reuse the existing integer-id pattern and stay testable on the public API without a
  map, economy, diplomacy, military, or renderer.
- **Source of the idea:** James (simplest first, 2026-09-01); general grand-strategy practice
  (named countries have named people). Not copied from a named Clausewitz subsystem and not
  taken from a sibling project.
- **Decided by:** Grok (iPhone), this session

### 2026-09-19 — Person record: names, allegiances, birth, derived age, places

- **Decided:** `Person` is a public data struct (`include/suvorov/core/person.hpp`). Every
  person has at least one non-empty name, at least one allegiance to an existing polity, a
  valid birth date that is not after the world date, a birth location, and a current
  location. Biological age is derived as completed calendar years from `birth_date` to the
  world date (`biological_age` / `World::person_age`). `set_person_location` changes only
  current location. Locations are World-owned named ids with no adjacency, owner, size, or
  travel time — not a map. This replaces the single-name / single-polity helpers and the "no
  location or age" part of the previous person entry. Still no title, death, traits, economy,
  diplomacy, or military.
- **Alternatives:** Keep a single name string and a single polity id; store age as a field;
  treat locations as free strings; add provinces and adjacency now.
- **Why:** James asked for a person data class with multiple names, multiple allegiances,
  birth date, derived biological age, birth location, and current location, stacked on the
  same draft PR. Named location ids give those fields somewhere to point without inventing
  geography.
- **Source of the idea:** James (2026-09-19, person fields); general practice (age derived
  from birth date, not stored). Not a map. Not taken from a sibling project.
- **Decided by:** James (the fields); Grok (iPhone) (named location ids, derived age,
  `Person` as a public struct)
