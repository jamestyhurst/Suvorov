# 2026-10-03 — wider world API and core effects

James asked for more per prompt. Still on the iPhone branch. Still draft.

Rune `world::` now reads `date_text`, `person_name`, and `is_alive`. It may ask for a marriage, a title, an heir, a move, a death, or an allegiance change. Asks apply after `on_fire` returns. Marriage, titles, and inheritance still refuse when that feature is off. Move, kill, and allegiance are core: every game has persons and places.

Scheduled effects gained `MovePerson` and `SetAllegiance`, so a game can do those without a script.
