//! Quantum Fourier Transform (SPEC-PHASE-1 section 4.3).
//!
//! The QFT is implemented as a **gate sequence** (Hadamards and controlled phase
//! rotations, O(n^2) gates), not as a matrix. The matrix variant [`qft_matrix`] is kept
//! for small `n` and serves as the reference in tests: sequence QFT and matrix QFT must
//! agree on basis states to 1e-12.
//!
//! Phase factors are `e^(2 pi i / 2^k)`; f64 precision suffices for n <= 24.
//!
//! # Scaffold status
//!
//! Everything is a placeholder (`TODO(M3)`): the sequence routines are no-ops, the
//! matrix routine returns the identity.

use num_complex::Complex;

use crate::gates::Matrix;
use crate::state::StateVector;
use crate::Result;

/// Applies the QFT to the given qubits, as a gate sequence of H and controlled phase
/// rotations (O(n^2) gates). TODO(M3): sequence per SPEC 4.3; placeholder: no-op.
pub fn qft_gate_sequence(sv: &mut StateVector, qubits: &[usize]) -> Result<()> {
    let _ = (sv, qubits); // TODO(M3)
    Ok(())
}

/// Applies the inverse QFT to the given qubits (reverse order, conjugated rotations).
/// TODO(M3); placeholder: no-op.
pub fn qft_inverse_gate_sequence(sv: &mut StateVector, qubits: &[usize]) -> Result<()> {
    let _ = (sv, qubits); // TODO(M3)
    Ok(())
}

/// The QFT as an explicit `2^n x 2^n` matrix: `F[j][k] = e^{2 pi i j k / 2^n} / sqrt(2^n)`.
///
/// Kept for small n as the test reference (SPEC 4.3). TODO(M3); placeholder: identity.
pub fn qft_matrix(n: usize) -> Matrix {
    let dim = 1usize << n;
    let mut m = vec![vec![Complex::new(0.0, 0.0); dim]; dim];
    for (i, row) in m.iter_mut().enumerate() {
        row[i] = Complex::new(1.0, 0.0);
    }
    m // TODO(M3)
}

/// Applies the matrix QFT [`qft_matrix`] to the first `n` qubits of `sv`.
/// TODO(M3); placeholder: no-op.
pub fn qft_matrix_apply(sv: &mut StateVector, n: usize) -> Result<()> {
    let _ = (sv, n); // TODO(M3)
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Smoke tests for the scaffold. The spec test plan for this module (SPEC 7.1):
    //! TODO(M3): QFT of |0> is the uniform superposition, QFT of a basis state |k> is the
    //! plane wave with the expected phase structure, QFT∘QFT-inverse = I (1e-12),
    //! gate sequence against the matrix implementation on basis states (1e-12).

    use super::*;

    #[test]
    fn smoke_qft_matrix_shape() {
        assert_eq!(qft_matrix(3).len(), 8);
        assert_eq!(qft_matrix(3)[0].len(), 8);
    }

    #[test]
    fn smoke_sequences_are_infallible_noops() {
        let mut sv = StateVector::new(3);
        assert!(qft_gate_sequence(&mut sv, &[0, 1, 2]).is_ok());
        assert!(qft_inverse_gate_sequence(&mut sv, &[0, 1, 2]).is_ok());
        assert!(qft_matrix_apply(&mut sv, 3).is_ok());
    }
}
