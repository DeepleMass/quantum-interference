"""Main experiment script: period finding (order finding) end to end.

SPEC-PHASE-1 section 6: for given N and a, choose Q = 2^m with N^2 <= Q < 2N^2,
build the oracle, apply Hadamards on the input register, the oracle, the QFT on the
input register, measure the input register, reconstruct r via continued fractions,
check a^r ≡ 1 (mod N), and optionally extract factors via gcd(a^(r/2) ± 1, N) for
N <= 35. Statistics over a configurable number of runs with seeded reproducibility.

Scaffold status: the pipeline structure and CLI are in place; the quantum steps call
the placeholder bindings (TODO(M6)) and the report is a skeleton.

Usage:
    python scripts/run_period_finding.py --n 15 --a 7 --shots 1024 --seed 42
"""

from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

from qsim import Simulator
from qsim.postprocess import OrderFindingResult, find_order_with_retries
from qsim.report import ExperimentConfig, write_report


@dataclass
class RunResult:
    """One experiment run: configuration, post-processing result, report path."""

    config: ExperimentConfig
    result: OrderFindingResult
    report_path: Path


def choose_q(n: int) -> tuple[int, int]:
    """Returns ``(m, Q)`` with ``Q = 2^m`` the smallest power of two with ``N^2 <= Q``.

    This automatically satisfies ``Q < 2N^2`` (SPEC section 6).
    """
    m = max(1, (n * n - 1).bit_length())
    return m, 1 << m


def run_period_finding(
    n: int,
    a: int,
    shots: int,
    seed: int,
    runs: int = 1,
    out_dir: str | Path = "docs/reports",
) -> RunResult:
    """Runs the full period-finding pipeline and writes one markdown report.

    TODO(M6): Hadamards on the input register -> oracle -> QFT on the input register
    -> measurement of the input register -> continued-fraction reconstruction of r ->
    consistency check a^r ≡ 1 (mod N) -> optional factor extraction; statistics over
    `runs` repetitions with success/failure reporting (odd r, gcd(j, r) > 1,
    degenerate a^(r/2) ≡ -1 mod N).

    Args:
        n: Modulus N.
        a: Base a (coprime to N).
        shots: Measurements per run.
        seed: Seed of the simulator handle (reproducibility, SPEC section 11).
        runs: Number of independent runs.
        out_dir: Report output directory.

    Returns:
        The :class:`RunResult` with config, result, and report path.
    """
    m, q = choose_q(n)
    output_qubits = max(1, (n - 1).bit_length())
    sim = Simulator(m, output_qubits)
    # TODO(M6): replace the placeholder calls below with the real circuit:
    #   sim.apply("h", list(range(m)), [])      # uniform superposition on input
    #   sim.apply_oracle(a, n)                   # |x>|y> -> |x>|y XOR a^x mod n>
    #   sim.apply_qft()                          # QFT on the input register
    #   counts = sim.sample(shots, seed)         # measure, then reconstruct per shot
    sim.apply("h", list(range(m)), [])
    sim.apply_oracle(a, n)
    sim.apply_qft()
    counts = sim.sample(shots, seed)  # placeholder: empty until M6
    config = ExperimentConfig(n=n, a=a, input_qubits=m, q=q, seed=seed, shots=shots, runs=runs)
    result = find_order_with_retries(n, a, q, measure=lambda: 0, max_attempts=20)
    report_path = write_report(
        config,
        counts,
        [] if result.order is None else [result.order],
        0.0,
        "Scaffold placeholder run — implement milestones M2..M6 to fill this report.",
        out_dir=out_dir,
    )
    return RunResult(config=config, result=result, report_path=report_path)


def main(argv: list[str] | None = None) -> int:
    """CLI entry point. Returns the process exit code."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--n", type=int, required=True, help="modulus N (<= 1021)")
    parser.add_argument("--a", type=int, required=True, help="base a, coprime to N")
    parser.add_argument("--shots", type=int, default=1024, help="measurements per run")
    parser.add_argument("--seed", type=int, default=42, help="seed of the simulator handle")
    parser.add_argument("--runs", type=int, default=1, help="independent runs")
    parser.add_argument("--out-dir", type=str, default="docs/reports", help="report directory")
    args = parser.parse_args(argv)
    res = run_period_finding(args.n, args.a, args.shots, args.seed, args.runs, args.out_dir)
    print(f"N = {res.config.n}, a = {res.config.a}, Q = {res.config.q}, seed = {res.config.seed}")
    print(f"report: {res.report_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
