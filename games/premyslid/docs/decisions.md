# Premyslid decision log

One entry per design decision, newest at the bottom. **Don't rewrite old entries.** If a
decision is reversed, add a new entry that names the one it replaces.

This log covers decisions about the *game*. Decisions about the Suvorov engine belong in
`docs\decisions.md` at the repository root. When a decision made here turns out to apply to
grand strategy games in general, it is promoted: a new entry in the root log records the
engine-level decision and names this entry as its origin.

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

### 2026-09-22 — The project is a Crusader Kings 1 clone, built by free models against a written schema

- **Decided:** Build a clone of Crusader Kings 1 as a game inside Suvorov. A strong model
  writes the schema, the validator, and the work queue; free models running under OpenCode
  do the grinding over time.
- **Alternatives:** A one-shot attempt with a single prompt to a frontier model; an unlimited
  single session; a fixed multi-session budget; a measured experiment with controlled
  conditions for comparison against Teutoburg.
- **Why:** James's words: *"I'd be scared of burning my whole subscription if I actually try
  to one-shot it… I think it'd be cheaper to let a smart model like Opus design a schema that
  I can throw free OpenCode models at to grind away and see what I can get over time."* The
  trigger was seeing frontier models used to one-shot clones of games, and never of a Paradox
  game. The interesting question is what a well-defined task does for weak models, so the
  schema — the definition — is the artifact under test.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-22 — Scope is characters, titles, and succession

- **Decided:** The clone covers landed characters, titles, succession law, vassal opinion,
  and one crude war. Trade, plots, crusades, claims, de jure drift, culture and religion
  conversion, and technology are out of scope.
- **Alternatives:** Enumerate the full Crusader Kings 1 feature list; build the map and armies
  first and add characters later.
- **Why:** Heritable titles and a succession crisis are what make Crusader Kings that game
  rather than a map-painting game. It is also the part most likely to defeat a weak model,
  which makes it the honest test rather than the flattering one.
- **Source of the idea:** Crusader Kings 1 (character and succession model); James (choice)
- **Decided by:** James

### 2026-09-22 — The game lives inside Suvorov, and general systems are promoted into the engine

- **Decided:** The game is `games\premyslid\` in this repository rather than a separate
  project. When a system built for the game applies to grand strategy games generally, it is
  promoted into the Suvorov engine.
- **Alternatives:** A new repository borrowing Suvorov's rules but not its code; a new
  repository starting from a fork of another project's simulation layer.
- **Why:** James's words: *"One purpose is to give Suvorov a use case or test case… If a
  system in the CK1 clone would apply to grand strategy games in general, then it should
  ideally also exist in the Suvorov engine as well."* An engine with no consumer cannot have
  its layering claim tested. Reuse of Suvorov's existing code was not a factor: at the time
  of this decision the engine was forty-four lines holding a date and a list of polities.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-22 — Free models may write game content only; the boundary is enforced mechanically

- **Decided:** A content session may write `games\premyslid\content\` and
  `games\premyslid\research\`, and nothing else. Continuous integration rejects a pull
  request from a content session that touches anything else. A task that cannot be finished
  inside those folders is written up as a blocked note instead.
- **Alternatives:** Let free models edit the engine too; keep the game in a separate
  repository so the engine is physically unreachable.
- **Why:** The boundary is the measuring instrument. A content task that cannot be expressed
  in content is evidence of a missing engine capability, which is exactly what this game
  exists to surface. Enforcement is mechanical rather than written because a small model
  cannot be relied on to obey an instruction it finds inconvenient, and there is no human
  reviewing these merges.
- **Source of the idea:** James (the promotion idea); general practice (path-scoped
  continuous integration checks)
- **Decided by:** James

### 2026-09-22 — Content is schema-validated data with a frozen effect vocabulary

- **Decided:** Content is declarative JSON records validated against a schema, plus a frozen
  vocabulary of three effects and four targets for exemplar events. Behaviour lives in the
  engine, not in content.
- **Alternatives:** Content as Python modules implementing engine interfaces; data for static
  entities and Python for anything needing logic.
- **Why:** Weak models fill structured data far more reliably than they write correct logic,
  and schema validation is automatic review: a misspelled effect name fails immediately
  rather than silently doing nothing. Writing content as code would need tests written by
  somebody, which is most of the work this arrangement exists to avoid.
- **Source of the idea:** Clausewitz (engine binary plus scripted content files; trigger and
  effect vocabularies); James (the schema framing)
- **Decided by:** James

### 2026-09-22 — Work comes from a queue computed from schema gaps, not from a written backlog

- **Decided:** `python -m suvorov.tools.next` compares the plan against the content on disk
  and prints one concrete task. The task is chosen at random from the ten highest-priority
  gaps.
- **Alternatives:** A hand-written backlog file with numbered tasks; letting the model survey
  the repository and decide what to do.
- **Why:** James's requirement was that *"I can just tell OpenCode models 'Go work on the CK1
  clone' and it can always do something useful."* A computed queue never runs dry and needs
  no maintenance, because the schema and the plan define what completeness means. Choosing
  what matters is the thing weak models are worst at, so it is removed from their job. The
  random pick within the top ten avoids a claim protocol: collisions are rare, and a
  collision costs an empty pull request, which is cheap because the work was free.
- **Source of the idea:** James (the requirement); general practice (derive the work list
  from a specification rather than maintaining it by hand)
- **Decided by:** James

### 2026-09-22 — Hand off at schema and validator, before the engine exists

- **Decided:** Free models start work as soon as the schema, the validator, and the queue
  exist. The engine is deferred.
- **Alternatives:** Build a running succession loop first so that every contribution is
  visible in a working game; build war and armies as well.
- **Why:** James chose the cheapest handoff to conserve subscription usage. The cost is that
  content accumulates with no simulation proving it is shaped correctly, so rework risk
  scales with volume. The next two decisions exist to hold that risk down.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-22 — Depth-first exemplar, a validator that checks meaning, and research tasks

- **Decided:** One duchy is finished completely before any second duchy is started. The
  validator checks referential and temporal meaning, not only shape: references resolve, the
  de jure hierarchy has no cycles, a capital belongs to its own title, a holder is alive on
  the start date, a parent is at least twelve years older than their child, a marriage is
  recorded on both characters. Research tasks run alongside content tasks.
- **Alternatives:** Fill the whole map at once and accept the rework; do research and
  specification work first with content as a side activity.
- **Why:** James asked for a combination of the first and third options. With no engine, a
  wrong schema costs one rewritten duchy rather than four hundred. The semantic checks are
  the only review these merges get, so a rule not enforced there is a rule broken silently
  for weeks.
- **Source of the idea:** James (the combination); general practice (referential integrity
  checks; a vertical slice before breadth)
- **Decided by:** James

### 2026-09-22 — Research findings are non-canon until promoted

- **Decided:** Research notes live in `games\premyslid\research\`, cite public sources, and
  never change the plan, the schema, or a content record. Promotion is a human decision,
  logged here.
- **Alternatives:** Let research sessions correct the plan directly; forbid research tasks
  and have a strong model write all specification material.
- **Why:** Small models write confident and fluent prose about the eleventh century,
  including about things that are not so. Quarantining findings makes being wrong cheap: a
  wrong note costs a read, while a wrong content record propagates into everything that
  references it.
- **Source of the idea:** Teutoburg, by way of a session that had already read it — see the
  contamination entry below; also general practice (treating unreviewed findings as input)
- **Decided by:** James

### 2026-09-22 — World data is written from history, never copied from Crusader Kings

- **Decided:** Counties, dynasties, and characters are written from general historical
  knowledge with public sources cited. Crusader Kings' own game files are never parsed,
  copied, or included.
- **Alternatives:** Import the real province and character files for accuracy and
  completeness.
- **Why:** Bulk, bounded, source-cited work is exactly what free models are good at, and it
  fills the queue for months. Importing Paradox's content would put their material in this
  repository, which matters the moment anything here is shown to anyone.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-22 — Sessions land through continuous integration with no human review

- **Decided:** Each session works on its own branch and opens a pull request. Continuous
  integration runs the tests, the validator, and the write-boundary check; a pull request
  carrying a `Model:` trailer merges itself when they pass. Anything without that trailer
  waits for James.
- **Alternatives:** James reviews every pull request; commit straight to main.
- **Why:** Unattended grinding is the point, and a human approval step makes James the
  bottleneck. The trailer is what distinguishes a content session from human work, so the
  same signal both identifies the author and scopes the boundary rule.
- **Source of the idea:** James; general practice (branch protection with automated gates)
- **Decided by:** James

### 2026-09-22 — Which model wrote what is recorded in a commit trailer

- **Decided:** Every content commit carries `Model:` and `Task:` trailers. Session metrics
  are derived from the git history on demand by `python -m suvorov.tools.metrics` rather than
  accumulated in a file.
- **Alternatives:** A metrics file appended by continuous integration on each merge; a
  hand-written learning log; reconstructing everything from branch names later.
- **Why:** Git records when work landed and which files changed, but not which model produced
  it, and that is the variable this project is about. Deriving the summary instead of storing
  it avoids a write-back commit on every merge and the race between concurrent merges, and
  the numbers can never drift from the history.
- **Source of the idea:** general practice (commit trailers as provenance); James (the
  requirement to be able to compare against Teutoburg later)
- **Decided by:** James

### 2026-09-22 — The engine returns at a stated milestone, and bulk work unlocks only after it runs

- **Decided:** The milestone is: the exemplar duchy complete, the validator green, and the
  schema unchanged across ten merges. `next` announces it. Bulk map work stays locked until
  an engine runs a succession loop.
- **Alternatives:** Build the engine whenever it feels right; fire on a fixed date or a fixed
  number of merged pull requests.
- **Why:** A deferral with no stated condition drifts, because the grind keeps producing
  visible progress while the unverified pile grows. A fixed date would fire whether or not
  the schema had actually settled, which is the thing the milestone is meant to establish.
- **Source of the idea:** James; general practice (stability as a release gate)
- **Decided by:** James

### 2026-09-22 — Schema version 0 is static entities plus a frozen stub vocabulary

- **Decided:** Version 0 covers titles, holdings, characters, dynasties, cultures, faiths,
  and succession laws, plus three effects (`grant_title`, `kill_character`, `change_opinion`)
  and three triggers (`on_death`, `on_birth`, `on_succession`) for a small number of exemplar
  events.
- **Alternatives:** Static entities only; a full event, trigger, and effect vocabulary from
  the start.
- **Why:** James chose the hedge. A vocabulary invented before the engine exists is a guess
  about an engine that does not exist, so it is kept to the smallest set that gives the
  engine a concrete first target and shows content authors the shape events will take. The
  queue will not hand out event work beyond the budget in the plan.
- **Source of the idea:** Clausewitz (scripted events with trigger and effect vocabularies);
  James (the hedge)
- **Decided by:** James

### 2026-09-22 — The game is named Premyslid, after the dynasty that ruled Trutnov's region

- **Decided:** The game folder is `games\premyslid\`, named for the Premyslid dynasty of
  Bohemia. Diacritics are dropped from the folder and record ids: the dynasty is properly
  written Přemyslid.
- **Alternatives:** A literal `games\ck1\`; keeping the diacritics.
- **Why:** James's words: *"I'd keep 'Premyslid' because diacritics seem bad for this
  context… I'd want to specify for a dynasty that ruled over the territory of the modern day
  city/town of Trautenau/Trutnov."* Trutnov lies in north-eastern Bohemia, in the county the
  plan calls `c_hradec`, and Bohemia was Premyslid territory in the eleventh century. Two
  facts are recorded so that no content session invents around them: the town of Trutnov was
  founded in the thirteenth century, so it does not exist at the 1066 start date, and
  "Sudetenland" is a twentieth-century German term for Bohemia's border regions rather than
  anything a Premyslid ruler would have recognised.
- **Source of the idea:** James
- **Decided by:** James

### 2026-09-22 — This session had read Teutoburg before it read the Suvorov brief

- **Decided:** Record the contamination rather than conceal it. The session that designed
  this game had already read `..\Teutoburg\` — its file layout, its simulation module list,
  and its research conventions — before opening `AGENTS.md` and finding rule 2. The
  non-canon research convention above came from there.
- **Alternatives:** Say nothing; drop the convention and re-derive it.
- **Why:** James's stated principle for rule 2 is *"I want to know what coding agents would
  do if told to imitate Clausewitz, rather than imitating Teutoburg."* One convention crossed
  that line. It is a general engineering practice as well as a Teutoburg one, so it is kept,
  with its provenance stated. Content sessions remain barred from reading Teutoburg.
- **Source of the idea:** n/a — this is a record of a rule breach, not a design choice
- **Decided by:** Claude, reported to James the same session
