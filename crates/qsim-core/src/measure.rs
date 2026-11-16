//! Measurement (SPEC-PHASE-1 section 4.4).
//!
//! Full-register sampling from `|amplitude|^2` with a seeded RNG (rand_chacha; the seed is
//! held explicitly by the simulator handle in the binding layer, never a global RNG),
//! marginal distributions over sub-registers, single-qubit measurement with collapse
//! (renormalization), and a counting API for multi-shot statistics.
//! No GPU, no multithreading.
//!
//! # Scaffold status
//!
//! All routines are placeholders (`TODO(M4)`): sampling returns empty counts,
//! probabilities are all zero, collapse is a no-op returning 0.
//!
//! Reading: Nielsen & Chuang, §2.2.3, pp. 84–86; §4.4, pp. 185–187 (10th anniversary edition).

use std::collections::HashMap;

use rand_chacha::ChaCha8Rng;

use crate::state::StateVector;
use crate::Result;

/// Counting API result: bitstring (big-endian display) -> number of occurrences.
pub type Counts = HashMap<String, u64>;

/// Samples `shots` full-register outcomes from `|amplitude|^2` using `rng`.
///
/// TODO(M4): build the cumulative distribution over `|a_i|^2` once, then draw `shots`
/// samples with the seeded RNG; keys are big-endian bitstrings via
/// [`StateVector::index_to_bitstring`]. Placeholder: empty map.
pub fn sample(sv: &StateVector, shots: usize, rng: &mut ChaCha8Rng) -> Counts {
    let _ = (sv, shots, rng); // TODO(M4)
    Counts::new()
}

/// Full probability distribution `p(x) = |a_x|^2` over all `2^n` basis states.
///
/// TODO(M4): `amplitudes.iter().map(|a| a.norm_sqr())`. Placeholder: zeros.
pub fn probabilities(sv: &StateVector) -> Vec<f64> {
    vec![0.0; sv.len()] // TODO(M4)
}

/// Marginal distribution over the sub-register `keep` (all other bits summed out).
/// Keys are big-endian bitstrings of the kept qubits.
///
/// TODO(M4): for each basis state, add `|a|^2` to the bucket of its kept bits.
/// Placeholder: empty map.
pub fn marginal(sv: &StateVector, keep: &[usize]) -> Result<HashMap<String, f64>> {
    let _ = (sv, keep); // TODO(M4)
    Ok(HashMap::new())
}

/// Measures a single qubit: returns 0 or 1 with probability `|a|^2` of the respective
/// branch and collapses the state (zero the other branch, renormalize).
///
/// TODO(M4): collapse + renormalization per SPEC 4.4. Placeholder: no-op, returns 0.
pub fn measure_qubit(sv: &mut StateVector, qubit: usize, rng: &mut ChaCha8Rng) -> Result<u8> {
    let _ = (sv, qubit, rng); // TODO(M4)
    Ok(0)
}

#[cfg(test)]
mod tests {
    //! Smoke tests for the scaffold. The spec test plan for this module (SPEC 7.1):
    //! TODO(M4): chi-square tolerance test of the empirical against the theoretical
    //! distribution with fixed seed and >= 10^4 samples, collapse renormalization,
    //! deterministic reproducibility under the same seed.

    use rand::SeedableRng;

    use super::*;

    #[test]
    fn smoke_sample_returns_counts_map() {
        let sv = StateVector::new(2);
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let counts = sample(&sv, 100, &mut rng);
        assert!(counts.is_empty()); // placeholder behavior, see TODO(M4)
    }

    #[test]
    fn smoke_probabilities_shape() {
        let sv = StateVector::new(3);
        assert_eq!(probabilities(&sv).len(), 8);
    }

    #[test]
    fn smoke_measure_qubit_placeholder() {
        let mut sv = StateVector::new(1);
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        assert_eq!(measure_qubit(&mut sv, 0, &mut rng).unwrap(), 0);
        assert!(marginal(&sv, &[0]).unwrap().is_empty());
    }
}
