"""qsim — Phase 1: didactic state-vector quantum simulator.

Rust core (`qsim-core`) exposed via pyo3 as :class:`Simulator`, plus pure-Python
orchestration: classical post-processing (continued fractions, order reconstruction,
retry logic) and report generation. See ``SPEC-PHASE-1-SIMULATOR.md``.

Scaffold status: all quantum semantics live behind placeholders in the Rust core
(``TODO(Mx)`` markers); this package re-exports the binding class and provides the
Python-side scaffolding with the same placeholder convention.
"""

from qsim.postprocess import (
    OrderFindingResult,
    best_convergent,
    check_order,
    continued_fraction_expansion,
    extract_factors,
    find_order_with_retries,
    reconstruct_order,
)
from qsim.qsim import Simulator
from qsim.report import ExperimentConfig, render_markdown_report, write_report

__version__ = "0.1.0"

__all__ = [
    "ExperimentConfig",
    "OrderFindingResult",
    "Simulator",
    "best_convergent",
    "check_order",
    "continued_fraction_expansion",
    "extract_factors",
    "find_order_with_retries",
    "reconstruct_order",
    "render_markdown_report",
    "write_report",
]
