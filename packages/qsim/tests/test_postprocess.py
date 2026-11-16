"""Smoke tests for qsim.postprocess (scaffold state).

Spec test plan (SPEC-PHASE-1 7.4, TODO(M5/M6)):
- continued fractions against ``fractions.Fraction.limit_denominator`` as reference
  (test reference only, never on the production path),
- retry statistics logic,
- report format.
"""

from __future__ import annotations

from qsim.postprocess import (
    OrderFindingResult,
    best_convergent,
    check_order,
    continued_fraction_expansion,
    extract_factors,
    find_order_with_retries,
    gcd,
    reconstruct_order,
)


def test_placeholders_return_documented_defaults() -> None:
    assert continued_fraction_expansion(3, 8, 4) == []  # TODO(M5)
    assert best_convergent(3, 8, 4) is None  # TODO(M5)
    assert reconstruct_order(3, 8, 15) is None  # TODO(M5)
    assert check_order(7, 4, 15) is False  # TODO(M5)
    assert extract_factors(15, 7, 4) is None  # TODO(M5)


def test_find_order_with_retries_returns_result_dataclass() -> None:
    result = find_order_with_retries(15, 7, 256, measure=lambda: 0, max_attempts=5)
    assert isinstance(result, OrderFindingResult)
    assert result.n == 15 and result.a == 7 and result.q == 256
    assert result.order is None and result.attempts == 0  # TODO(M5/M6)


def test_gcd_wrapper() -> None:
    assert gcd(12, 18) == 6
