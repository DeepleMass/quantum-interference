"""Cross-validation against Qiskit (SPEC-PHASE-1 7.4) — slow, optional in CI.

The same order-finding circuit in Qiskit and in the own simulator, total variation of
the measurement distributions < 0.05 at 10^4 shots. Prerequisite: identical
little-endian bit convention; this test is the actual acceptance of Phase-1
compatibility.

Scaffold status: skipped until M6 — the detailed test case comes with the
implementation (repo decision: scaffold stays green with smoke tests only).
"""

from __future__ import annotations

import pytest

pytestmark = [
    pytest.mark.slow,
    pytest.mark.skip(reason="TODO(M6): implement the Qiskit cross-check"),
]


def test_tv_distance_against_qiskit() -> None:
    """TODO(M6): build the order-finding circuit in Qiskit and in qsim, compare."""
    raise NotImplementedError
