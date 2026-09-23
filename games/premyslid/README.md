# Premyslid

A clone of Crusader Kings 1, and Suvorov's first game. Named for the Premyslid dynasty, whose
Bohemian realm included the region of modern Trutnov. The scenario starts on 15 September
1066.

Content sessions: read [AGENTS.md](AGENTS.md) and nothing else.
Everyone else: start with [../../Developer_Documents/premyslid-design.md](../../Developer_Documents/premyslid-design.md).

## What this is for

It is not a product. It exists to find out what weak models do with a task that is well
defined — the schema here is the definition, and it is the thing actually under test — and to
give the Suvorov engine a real consumer, so that its claim to be setting-independent can be
tested rather than asserted.

## Layout

| Path | What it holds | Who may write it |
| --- | --- | --- |
| `schema\` | The content contract, and the frozen effect vocabulary | Protected |
| `plan\regions.json` | The target scope: which duchies and counties the game covers | Protected |
| `content\` | The game itself, one record per file | Content sessions |
| `research\` | Non-canon findings and blocked-task reports | Content sessions |
| `docs\decisions.md` | Decisions about this game | James |

Tooling lives outside this folder, in `python\suvorov\tools\`, and is protected. A content
session that needs to change anything protected writes a blocked note instead — those notes
are how the engine's missing capabilities get found.

## Current state

Schema version 0 covers static entities: titles, holdings, characters, dynasties, cultures,
faiths, and succession laws, plus three effects and three triggers for a handful of exemplar
events. There is **no engine**: nothing simulates a day, a death, or an inheritance yet.
Content is being written depth-first, one duchy at a time, and the rest of the map is locked
until an engine runs a succession loop.

```powershell
Set-Location python
python -m suvorov.tools.next        # one task
python -m suvorov.tools.validate    # check all content
python -m suvorov.tools.metrics     # who has written what
```
