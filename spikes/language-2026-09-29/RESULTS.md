# Language spike results

Measured 2026-09-29 by Grok from iPhone, in the session sandbox
(Linux, rustc 1.75, g++ 13.3 -O2 via CMake Release, CPython 3.12.3).
Not James's school PC. School PC still has no C++ or Rust toolchain.

Same public World API in all three languages. Checksums matched.

## Clock: 1,000,000 `advance_one_day` on an empty world

| Language | Wall time | Final date |
| --- | ---: | --- |
| Rust (release) | 2.497 ms | 3737-11-28 |
| C++20 (Release) | 2.616 ms | 3737-11-28 |
| Python 3.12 | 143.942 ms | 3737-11-28 |

Empty-clock is not a grand-strategy workload. It only shows that date math is cheap everywhere.

## Scan: 20,000 persons, 200 locations, 50 polities, 365 days

Each day: tick, derive every person's age, read first allegiance, move one person.

| Language | Wall time | Checksum | Final date |
| --- | ---: | ---: | --- |
| Rust (release) | 48.929 ms | 363840000 | 1001-1-1 |
| C++20 (Release) | 52.560 ms | 363840000 | 1001-1-1 |
| Python 3.12 | 5140.486 ms | 363840000 | 1001-1-1 |

Smaller check (5,000 persons, 100 days): Rust 4.031 ms, C++ 4.551 ms, Python 373.681 ms, checksum 24710000.

## What this does and does not decide

- Rust and C++ are in the same band on this slice. Neither wins on speed.
- Python is about 100× slower on the scan. Still fine for one duchy. Not fine as the tick core at CK scale.
- The interesting difference is not nanoseconds. It is agent-written memory safety, toolchain availability, and whether Premyslid/Python tools talk to the core through a boundary or *are* the core.
- This spike does **not** replace draft PR #2. The C++ core there stays. This branch adds a first-class Rust port under `rust/` plus throwaway Python/C++ copies under `spikes/`.
