"""Markdown/CSV report generation (SPEC-PHASE-1 section 9).

One markdown report per experiment goes to ``docs/reports/<N>_<a>_<timestamp>.md``
containing: configuration (qubits, Q, seed, shots), the measurement histogram as an
ASCII/CSV table, reconstructed ``r`` candidates, success rate, and a short
interpretation. A run is reproducible via the documented seed configuration.

Scaffold status: the file layout, naming, and markdown skeleton are in place; the
histogram table and statistics sections are placeholders (``TODO(M6)``).
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path


@dataclass
class ExperimentConfig:
    """Configuration block of one experiment run.

    Attributes:
        n: Modulus.
        a: Base.
        input_qubits: Number of input-register qubits m.
        q: Register size Q = 2^m.
        seed: Seed of the simulator handle.
        shots: Measurements per run.
        runs: Number of independent runs.
        timestamp: UTC timestamp of the run.
    """

    n: int
    a: int
    input_qubits: int
    q: int
    seed: int
    shots: int
    runs: int = 1
    timestamp: str = field(
        default_factory=lambda: datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    )


def render_markdown_report(
    config: ExperimentConfig,
    counts: dict[str, int],
    r_candidates: list[int],
    success_rate: float,
    interpretation: str,
) -> str:
    """Renders the experiment report as a markdown string.

    TODO(M6): histogram as an ASCII/CSV table (sorted by count, top entries), r
    candidates, success rate in percent, interpretation paragraph. Placeholder:
    skeleton with the configuration filled in and empty sections.

    Args:
        config: Run configuration.
        counts: Measurement histogram (bitstring -> count).
        r_candidates: Reconstructed order candidates.
        success_rate: Fraction of successful runs in [0, 1].
        interpretation: Free-text interpretation.

    Returns:
        The markdown document.
    """
    _ = (counts, r_candidates, success_rate, interpretation)  # TODO(M6)
    return (
        f"# Period finding: N = {config.n}, a = {config.a}\n\n"
        f"- input qubits m = {config.input_qubits}, Q = {config.q}\n"
        f"- seed = {config.seed}, shots = {config.shots}, runs = {config.runs}\n"
        f"- timestamp = {config.timestamp}\n\n"
        "## Histogram\n\nTODO(M6)\n\n"
        "## Reconstructed r candidates\n\nTODO(M6)\n\n"
        "## Success rate\n\nTODO(M6)\n\n"
        "## Interpretation\n\nTODO(M6)\n"
    )


def write_report(
    config: ExperimentConfig,
    counts: dict[str, int],
    r_candidates: list[int],
    success_rate: float,
    interpretation: str,
    out_dir: str | Path = "docs/reports",
) -> Path:
    """Writes one markdown report per experiment to ``docs/reports/``.

    File name: ``<N>_<a>_<timestamp>.md`` (SPEC section 9). Returns the path.
    """
    out = Path(out_dir)
    out.mkdir(parents=True, exist_ok=True)
    path = out / f"{config.n}_{config.a}_{config.timestamp}.md"
    path.write_text(
        render_markdown_report(config, counts, r_candidates, success_rate, interpretation),
        encoding="utf-8",
    )
    return path
