//! State vector (SPEC-PHASE-1 section 4.1).
//!
//! [`StateVector`] stores the `2^n` complex amplitudes of an `n`-qubit register.
//! Bit convention: **little-endian** like Qiskit — qubit 0 is the least significant bit
//! of the basis index. Display of bitstrings is **big-endian** (literature convention).
//!
//! Invariant: Euclidean norm 1 within [`NORM_TOLERANCE`], checked in debug assertions
//! and tests — *not* on every operation at runtime.
//!
//! # Scaffold status
//!
//! The struct, its trivial accessors, and bitstring formatting are in place.
//! The constructors and the norm check are placeholders (`TODO(M2)`).
//!
//! Reading: Nielsen & Chuang, §2.1, pp. 61–79; §2.2.1–2.2.2, pp. 80–83 (10th anniversary edition).

use num_complex::Complex;

use crate::{Result, NORM_TOLERANCE};

/// A pure state of `n` qubits: `2^n` complex amplitudes, little-endian qubit order.
#[derive(Clone, Debug)]
pub struct StateVector {
    /// Amplitudes indexed by basis state, `amplitudes.len() == 2^num_qubits`.
    amplitudes: Vec<Complex<f64>>,
    /// Number of qubits `n`.
    num_qubits: usize,
}

impl StateVector {
    /// Constructs the register in the basis state |0...0>.
    ///
    /// TODO(M2): initialize `amplitudes[0] = 1`, all others 0, and `debug_assert!` the
    /// norm invariant. Placeholder: zero vector (norm 0) so the scaffold compiles and
    /// smoke tests can inspect the shape without panicking.
    pub fn new(num_qubits: usize) -> Self {
        Self {
            amplitudes: vec![Complex::new(0.0, 0.0); 1 << num_qubits],
            num_qubits,
        }
    }

    /// Constructs the register in the arbitrary basis state `|index>`.
    ///
    /// TODO(M2): return [`QSimError::QubitIndexOutOfRange`] for `index >= 2^num_qubits`
    /// and set exactly one amplitude to 1. Placeholder: same as [`StateVector::new`].
    ///
    /// [`QSimError::QubitIndexOutOfRange`]: crate::QSimError::QubitIndexOutOfRange
    pub fn from_basis_state(num_qubits: usize, index: usize) -> Result<Self> {
        let _ = (num_qubits, index); // TODO(M2): bounds check + set amplitude
        Ok(Self::new(num_qubits))
    }

    /// Constructs the uniform superposition `(1/sqrt(2^n)) * sum_x |x>`.
    ///
    /// TODO(M2): every amplitude `1/sqrt(2^n)`. Placeholder: same as [`StateVector::new`].
    pub fn uniform_superposition(num_qubits: usize) -> Self {
        Self::new(num_qubits) // TODO(M2)
    }

    /// Number of qubits `n`.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Number of amplitudes, `2^n`.
    pub fn len(&self) -> usize {
        self.amplitudes.len()
    }

    /// Whether the register has zero qubits (always `false` for a real register).
    pub fn is_empty(&self) -> bool {
        self.amplitudes.is_empty()
    }

    /// Read-only view of all amplitudes.
    pub fn amplitudes(&self) -> &[Complex<f64>] {
        &self.amplitudes
    }

    /// Amplitude of the basis state `|index>`.
    ///
    /// TODO(M2): bounds-check `index` against `2^n`. Placeholder: returns the zero
    /// amplitude for out-of-range indices instead of an error.
    pub fn amplitude(&self, index: usize) -> Result<Complex<f64>> {
        Ok(self.amplitudes.get(index).copied().unwrap_or_default()) // TODO(M2)
    }

    /// Euclidean norm `||psi||`.
    ///
    /// TODO(M2): `sqrt(sum |a_i|^2)`. Placeholder: 0.0.
    pub fn norm(&self) -> f64 {
        0.0 // TODO(M2)
    }

    /// Checks the norm invariant within [`NORM_TOLERANCE`].
    ///
    /// TODO(M2): compare `self.norm()` against 1. Placeholder: always `true`.
    pub fn check_norm(&self) -> bool {
        let _ = NORM_TOLERANCE;
        true // TODO(M2)
    }

    /// Converts a basis-state index to its big-endian bitstring, e.g. `0b101 -> "101"`.
    ///
    /// This is a pure formatting helper and is provided by the scaffold; the quantum
    /// semantics around it are still placeholders.
    pub fn index_to_bitstring(index: usize, num_qubits: usize) -> String {
        format!("{index:0width$b}", width = num_qubits)
    }

    /// Converts a big-endian bitstring back to its basis-state index.
    ///
    /// TODO(M2): parse with `u64::from_str_radix` / bit iteration; error on invalid
    /// characters or length > 64. Placeholder: 0.
    pub fn bitstring_to_index(bitstring: &str) -> Result<u64> {
        let _ = bitstring; // TODO(M2)
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    //! Smoke tests for the scaffold. The spec test plan for this module (SPEC 7.1):
    //! TODO(M2): norm invariant after every operation, basis-state initialization,
    //! bitstring round-trip (`index_to_bitstring` <-> `bitstring_to_index`).

    use super::*;

    #[test]
    fn smoke_new_has_2_pow_n_amplitudes() {
        let sv = StateVector::new(3);
        assert_eq!(sv.num_qubits(), 3);
        assert_eq!(sv.len(), 8);
        assert!(!sv.is_empty());
    }

    #[test]
    fn smoke_bitstring_display_is_big_endian() {
        assert_eq!(StateVector::index_to_bitstring(0b101, 3), "101");
        assert_eq!(StateVector::index_to_bitstring(1, 4), "0001");
    }
}
