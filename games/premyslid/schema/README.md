# Přemyslid content schema, version 0

These files are the contract between the content in `..\content\` and the tooling in
`python\suvorov\tools\`. They are **protected**: free models working on this game may not
edit anything in this folder. Continuous integration rejects any pull request that changes a
file here without a human author.

## Dialect

The schema files use a deliberately small subset of JSON Schema, plus two custom keywords.
The validator in `python\suvorov\tools\schema_lite.py` implements exactly this subset and
nothing else, so a schema file that reaches for an unsupported keyword fails loudly rather
than silently passing everything.

Supported keywords:

| Keyword | Meaning |
| --- | --- |
| `type` | One of `object`, `array`, `string`, `integer`, `boolean`, `null`, or a list of those |
| `properties` | Per-property subschemas, for objects |
| `required` | Property names that must be present |
| `additionalProperties` | Always `false` in this project: unknown keys are errors, because a typo in a key name is the most common way a weak model produces content that looks correct and does nothing |
| `enum` | Allowed literal values |
| `pattern` | Regular expression, for strings |
| `items` | Subschema applied to every array element |
| `minItems`, `minimum`, `maximum`, `minLength` | Bounds |

Custom keywords:

| Keyword | Meaning |
| --- | --- |
| `x-ref` | The string is the id of a record of the named kind, or of one of several kinds when given a list. The validator checks that the target record exists. |
| `x-date` | The string is a calendar date in `YYYY-MM-DD` form and must be a real date. Years before 1000 are written with leading zeros. |

## One record per file

Every content file holds exactly one record, and the file name must equal that record's `id`
with a `.json` extension. A duchy called `d_bohemia` lives in
`content\titles\d_bohemia.json`. This is not a stylistic choice: it means two agents working
at once almost never touch the same file, so their pull requests merge without conflict.

## Record kinds

| Kind | Folder | Schema |
| --- | --- | --- |
| Title | `content\titles\` | `title.schema.json` |
| Holding | `content\holdings\` | `holding.schema.json` |
| Character | `content\characters\` | `character.schema.json` |
| Dynasty | `content\dynasties\` | `dynasty.schema.json` |
| Culture | `content\cultures\` | `culture.schema.json` |
| Faith | `content\faiths\` | `faith.schema.json` |
| Succession law | `content\succession_laws\` | `succession_law.schema.json` |
| Event | `content\events\` | `event.schema.json` |

## What version 0 deliberately excludes

Version 0 describes static entities only. Trade, plots, crusades, claims, de jure drift,
culture and religion conversion, technology, and armies have no representation here. The
frozen effect vocabulary in `effects.json` is the single exception, and it is small on
purpose: three effects and four targets, enough for a handful of exemplar events, not enough
to invite event authoring at scale before an engine exists to execute them.
