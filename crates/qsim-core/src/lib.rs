//! `qsim-core` — state-vector quantum simulator core (Phase 1 of quantum-interference).
//!
//! Implemented from scratch per `SPEC-PHASE-1-SIMULATOR.md`: state vector, gates, QFT,
//! measurement, function oracles, and continued fractions as pure, self-contained units.
//!
//! # Scaffold status
//!
//! This crate is a **scaffold**: data structures, signatures, error types, and trivial
//! accessors are in place; every piece of quantum semantics is a placeholder returning a
//! default value and is marked with `TODO(Mx)` referencing the spec milestone that owns it.
//! The implementations are the exercise — see the test plans in each module.
//!
//! # Conventions (SPEC section 11)
//!
//! - Little-endian qubit indexing (Qiskit convention): qubit 0 is the least significant
//!   bit of the basis-state index.
//! - Big-endian display of bitstrings (as customary in the literature).
//! - No panics in the public API: everything goes through [`Result`].

pub mod cfrac;
pub mod gates;
pub mod measure;
pub mod oracle;
pub mod qft;
pub mod state;

/// Error type of the whole crate (SPEC section 4.7). No panics in the public API.
#[derive(Debug, thiserror::Error)]
pub enum QSimError {
    /// A qubit index was outside the valid range for the register.
    #[error("qubit index {index} out of range for a register of {num_qubits} qubits")]
    QubitIndexOutOfRange {
        /// The offending index.
        index: usize,
        /// Number of qubits in the register.
        num_qubits: usize,
    },
    /// A gate's dimension does not match the number of target qubits.
    #[error("dimension mismatch: gate acts on {gate_dim} amplitudes but {targets} target qubits were given")]
    DimensionMismatch {
        /// Dimension of the supplied gate matrix (2^k for k qubits).
        gate_dim: usize,
        /// Number of target qubits.
        targets: usize,
    },
    /// The Euclidean norm of the state deviates from 1 beyond tolerance.
    #[error("norm violation: ||psi|| = {0}, expected 1 within {NORM_TOLERANCE}")]
    NormViolation(f64),
    /// Oracle parameters are invalid (e.g. a not coprime to n).
    #[error("invalid oracle parameters: {0}")]
    InvalidOracleParameters(String),
}

/// Convenient result alias used by the whole public API.
pub type Result<T> = std::result::Result<T, QSimError>;

/// Tolerance for the norm invariant ||psi|| = 1 (SPEC section 4.1).
pub const NORM_TOLERANCE: f64 = 1e-12;

/// Tolerance for unitary / QFT agreement checks in tests (SPEC sections 4.2, 4.3).
pub const UNITARY_TOLERANCE: f64 = 1e-12;

#[cfg(test)]
mod tests {
    //! Smoke tests only — the crate is a scaffold (see module docs).
    //! TODO(M1): error type displays, Result alias works.

    use super::*;

    #[test]
    fn smoke_error_variants_construct() {
        let err = QSimError::QubitIndexOutOfRange {
            index: 5,
            num_qubits: 3,
        };
        assert!(err.to_string().contains("out of range"));
        let err = QSimError::NormViolation(0.5);
        assert!(err.to_string().contains("norm violation"));
    }
}
