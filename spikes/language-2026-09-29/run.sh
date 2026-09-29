#!/usr/bin/env bash
# Run the three-language tests and benches. From repo root:
#   bash spikes/language-2026-09-29/run.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SPIKE="$ROOT/spikes/language-2026-09-29"

echo "== Rust tests =="
(cd "$ROOT/rust" && cargo test)

echo "== Python tests =="
(cd "$SPIKE/python" && python3 -m unittest test_world.py)

echo "== C++ tests =="
cmake -S "$SPIKE/cpp" -B "$SPIKE/cpp/build" -DCMAKE_BUILD_TYPE=Release
cmake --build "$SPIKE/cpp/build" --config Release
"$SPIKE/cpp/build/suvorov_spike_tests"

echo "== Benches =="
(cd "$ROOT/rust" && cargo build --release --bin suvorov-bench)
"$ROOT/rust/target/release/suvorov-bench" clock 1000000
"$SPIKE/cpp/build/suvorov_spike_bench" clock 1000000
python3 "$SPIKE/python/bench.py" clock 1000000
"$ROOT/rust/target/release/suvorov-bench" scan 20000 200 50 365
"$SPIKE/cpp/build/suvorov_spike_bench" scan 20000 200 50 365
python3 "$SPIKE/python/bench.py" scan 20000 200 50 365
