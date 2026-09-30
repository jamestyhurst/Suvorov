# 2026-09-29 language spike

Three implementations of the draft PR #2 World API:

| Path | Role |
| --- | --- |
| `rust/` at repo root | First-class Rust rewrite. Candidate engine core. |
| `spikes/language-2026-09-29/cpp/` | Throwaway C++ copy so the bench is in-tree on this branch (does not edit PR #2). |
| `spikes/language-2026-09-29/python/` | Throwaway Python port for the same tests and bench. |

Placeholder names (`Aurora`, `Helia`, `Amber Vale`, `Calen`) are fictional.

```bash
bash spikes/language-2026-09-29/run.sh
```

See `RESULTS.md` for numbers and `docs/2026-09-29-language-spike.md` for the decision candidate.
