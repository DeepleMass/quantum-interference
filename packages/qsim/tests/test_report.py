"""Smoke tests for qsim.report (scaffold state).

Spec test plan (SPEC-PHASE-1 7.4/9, TODO(M6)): report format — configuration block,
histogram table, r candidates, success rate, interpretation; file name
``<N>_<a>_<timestamp>.md`` under ``docs/reports/``.
"""

from __future__ import annotations

from pathlib import Path

from qsim.report import ExperimentConfig, render_markdown_report, write_report


def _config() -> ExperimentConfig:
    return ExperimentConfig(n=15, a=7, input_qubits=8, q=256, seed=42, shots=1024, runs=3)


def test_render_markdown_report_contains_config() -> None:
    md = render_markdown_report(_config(), {}, [], 0.0, "")
    assert "# Period finding: N = 15, a = 7" in md
    assert "Q = 256" in md
    assert "seed = 42" in md


def test_write_report_creates_timestamped_file(tmp_path: Path) -> None:
    path = write_report(_config(), {"0000": 10}, [4], 1.0, "demo", out_dir=tmp_path)
    assert path.exists()
    assert path.name.startswith("15_7_")
    assert path.suffix == ".md"
    assert "N = 15" in path.read_text(encoding="utf-8")
