"""Python view of the simulation core.

Imports the PyO3 module built from `rust/`. School PCs without a compiler
cannot build that module; the error says so instead of inventing a second core.
"""

from __future__ import annotations

try:
    from suvorov_core import Date, Person, World, biological_age
except ImportError as exc:  # pragma: no cover - depends on a local maturin build
    raise ImportError(
        "suvorov_core is not installed. From the repo: "
        "`cd rust && maturin develop --features python`. "
        "The school PC has no Rust toolchain; do not install one without asking James."
    ) from exc

__all__ = ["Date", "Person", "World", "biological_age"]
