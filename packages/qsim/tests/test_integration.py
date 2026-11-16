"""Integration smoke test: the main script callable as a library function.

Spec test plan (SPEC-PHASE-1 7.4, TODO(M6)): for N in {15, 21} the factorization is
found within at most 20 attempts, deterministic under a fixed seed.
"""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

import pytest

_SCRIPT = Path(__file__).resolve().parents[3] / "scripts" / "run_period_finding.py"


def _load_script():
    spec = importlib.util.spec_from_file_location("run_period_finding", _SCRIPT)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module  # dataclasses resolve the module via sys.modules
    spec.loader.exec_module(module)
    return module


def test_run_period_finding_as_library(tmp_path: Path) -> None:
    module = _load_script()
    result = module.run_period_finding(n=15, a=7, shots=8, seed=42, runs=1, out_dir=tmp_path)
    assert result.config.n == 15
    assert result.config.a == 7
    assert result.config.seed == 42
    assert result.report_path.exists()  # placeholder pipeline still writes a report


def test_cli_smoke(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    module = _load_script()
    rc = module.main(
        ["--n", "15", "--a", "7", "--shots", "8", "--seed", "42", "--out-dir", str(tmp_path)]
    )
    assert rc == 0
    assert "N = 15" in capsys.readouterr().out
