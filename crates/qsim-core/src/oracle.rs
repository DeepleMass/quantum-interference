//! Function oracles (SPEC-PHASE-1 section 4.5).
//!
//! A function oracle `f: {0,1}^m -> {0,1}^n` is applied as the permutation
//! `|x>|y> -> |x>|y XOR f(x)>` on basis states — O(2^m) evaluations of `f`. The oracle is
//! the only place where problem size enters the simulator; `f` is computed **classically**,
//! exactly as in real Shor circuits.
//!
//! [`ModularExponentiation`] implements `f(x) = a^x mod n` via square-and-multiply.
//!
//! # Scaffold status
//!
//! The trait and struct are in place; `eval` and [`apply_oracle`] are placeholders
//! (`TODO(M5)`).
//!
//! Reading: Nielsen & Chuang, §5.3, pp. 226–233 (10th anniversary edition).

use crate::state::StateVector;
use crate::Result;

/// A classical function oracle `f: {0,1}^width_in -> {0,1}^width_out`.
pub trait FnOracle {
    /// Evaluates `f(x)`.
    fn eval(&self, x: u64) -> u64;
    /// Input register width in qubits (m).
    fn width_in(&self) -> usize;
    /// Output register width in qubits (n).
    fn width_out(&self) -> usize;
}

/// Oracle for `f(x) = a^x mod n` (modular exponentiation, square-and-multiply).
#[derive(Clone, Debug)]
pub struct ModularExponentiation {
    a: u64,
    n: u64,
    width_in: usize,
    width_out: usize,
}

impl ModularExponentiation {
    /// Creates the oracle for `f(x) = a^x mod n` on an input register of `width_in`
    /// qubits. The output register needs `ceil(log2(n))` qubits.
    ///
    /// TODO(M5): validate `n > 1`, `1 < a < n`, `gcd(a, n) == 1` — else
    /// [`crate::QSimError::InvalidOracleParameters`]. Placeholder: accepts everything.
    pub fn new(a: u64, n: u64, width_in: usize) -> Result<Self> {
        let _ = (a, n, width_in); // TODO(M5): parameter validation
        let width_out = if n > 1 {
            (64 - (n - 1).leading_zeros()) as usize
        } else {
            1
        };
        Ok(Self {
            a,
            n,
            width_in,
            width_out,
        })
    }

    /// The base `a`.
    pub fn base(&self) -> u64 {
        self.a
    }

    /// The modulus `n`.
    pub fn modulus(&self) -> u64 {
        self.n
    }
}

impl FnOracle for ModularExponentiation {
    /// TODO(M5): `a^x mod n` via square-and-multiply. Placeholder: 0.
    fn eval(&self, x: u64) -> u64 {
        let _ = x; // TODO(M5)
        0
    }

    fn width_in(&self) -> usize {
        self.width_in
    }

    fn width_out(&self) -> usize {
        self.width_out
    }
}

/// Applies the oracle as `|x>|y> -> |x>|y XOR f(x)>` on the joint register.
///
/// `input_offset` is the index of the least significant input qubit; the output register
/// occupies the `width_out` qubits above it. TODO(M5): iterate over all `2^m` input
/// values, XOR `f(x)` into the output bits of each basis state. Placeholder: no-op.
pub fn apply_oracle(
    sv: &mut StateVector,
    oracle: &dyn FnOracle,
    input_offset: usize,
) -> Result<()> {
    let _ = (sv, oracle, input_offset); // TODO(M5)
    Ok(())
}

/// Checks the oracle parameters without constructing the permutation.
///
/// TODO(M5): `gcd(a, n) == 1` etc. Placeholder: `Ok(())`.
pub fn validate_oracle_params(a: u64, n: u64) -> Result<()> {
    let _ = (a, n); // TODO(M5): gcd check, see crate::QSimError::InvalidOracleParameters
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Smoke tests for the scaffold. The spec test plan for this module (SPEC 7.1):
    //! TODO(M5): f(x) = a^x mod n against a pow reference for all x < 2^m, XOR
    //! composition correct, bijectivity of y XOR f(x) in y.

    use super::*;

    #[test]
    fn smoke_modular_exponentiation_constructs() {
        let oracle = ModularExponentiation::new(7, 15, 8).unwrap();
        assert_eq!(oracle.base(), 7);
        assert_eq!(oracle.modulus(), 15);
        assert_eq!(oracle.width_in(), 8);
        assert_eq!(oracle.width_out(), 4); // ceil(log2(15)) = 4
        assert_eq!(oracle.eval(3), 0); // placeholder, see TODO(M5)
    }

    #[test]
    fn smoke_apply_oracle_is_infallible_noop() {
        let oracle = ModularExponentiation::new(2, 21, 10).unwrap();
        let mut sv = StateVector::new(14);
        assert!(apply_oracle(&mut sv, &oracle, 0).is_ok());
        assert!(validate_oracle_params(2, 21).is_ok());
    }
}
