"""Smoke tests for the pyo3 Simulator binding (scaffold state).

Spec test plan (SPEC-PHASE-1 7.4, TODO(M6)):
- unit: the full Simulator API against the Rust core semantics,
- integration: run_period_finding callable as a library function, N in {15, 21}
  factorized within at most 20 attempts, deterministic under a fixed seed,
- cross-validation against Qiskit (slow, optional in CI): TV distance < 0.05 at 10^4
  shots, identical little-endian bit convention.
"""

from __future__ import annotations

import numpy as np
from qsim import Simulator


def test_simulator_constructs_with_register_sizes() -> None:
    sim = Simulator(4, 4)
    assert sim.input_qubits == 4
    assert sim.output_qubits == 4
    assert sim.num_qubits() == 8


def test_simulator_methods_are_infallible_placeholders() -> None:
    sim = Simulator(3, 4)
    sim.apply("h", [0, 1, 2], [])
    sim.apply("cnot", [0, 3], [])
    sim.apply_oracle(7, 15)
    sim.apply_qft()
    sim.reset()
    counts = sim.sample(shots=16, seed=42)
    assert isinstance(counts, dict)
    assert counts == {}  # placeholder behavior, see TODO(M6) in the Rust binding


def test_probabilities_and_amplitudes_shapes() -> None:
    sim = Simulator(3, 4)
    probs = sim.probabilities()
    amps = sim.amplitudes()
    assert isinstance(probs, np.ndarray)
    assert probs.shape == (2**7,)
    assert amps.shape == (2**7,)
    assert amps.dtype == np.complex128
    assert not amps.flags.writeable  # read-only view: ownership stays in Rust (SPEC 11)
