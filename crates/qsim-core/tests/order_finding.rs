//! Rust integration tests: end-to-end order finding at the core level
//! (SPEC-PHASE-1 section 7.2).
//!
//! # Scaffold status
//!
//! Smoke test only — with placeholders in place, the pipeline runs as a no-op.
//!
//! Spec test plan (TODO(M5)) — reference instances:
//! - `ord(7, 15) = 4`, `ord(11, 15) = 2`, `ord(2, 21) = 6`, `ord(4, 21) = 3`.
//!
//! Acceptance: over 50 measurement runs, `r` is correctly reconstructed in at least 80 %
//! of reconstructible runs; the fraction of non-reconstructible measurements
//! (`gcd(j, r) > 1`) stays below 50 %, consistent with theory.

use qsim_core::oracle::{apply_oracle, FnOracle, ModularExponentiation};
use qsim_core::qft::{qft_gate_sequence, qft_inverse_gate_sequence};
use qsim_core::state::StateVector;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Runs the order-finding pipeline once at the core level and returns the measured
/// input-register bitstring. Placeholder: returns the zero bitstring.
///
/// TODO(M5): H on all input qubits -> oracle -> QFT on input register -> sample.
fn run_once(a: u64, n: u64, m: usize, seed: u64) -> String {
    let oracle = ModularExponentiation::new(a, n, m).expect("valid oracle params");
    let mut sv = StateVector::new(m + oracle.width_out());
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let _ = apply_oracle(&mut sv, &oracle, 0);
    let _ = qft_gate_sequence(&mut sv, &(0..m).collect::<Vec<_>>());
    let _ = qft_inverse_gate_sequence(&mut sv, &(0..m).collect::<Vec<_>>()); // TODO(M5): remove, placeholder round-trip
    let _ = &mut rng;
    StateVector::index_to_bitstring(0, m)
}

#[test]
fn smoke_pipeline_runs_as_noop() {
    // ord(7, 15) = 4 — reference instance from SPEC 7.2 (reconstruction is TODO(M5)).
    let outcome = run_once(7, 15, 8, 42);
    assert_eq!(outcome, "00000000");
}

#[test]
fn smoke_all_reference_instances_construct() {
    for (a, n) in [(7u64, 15u64), (11, 15), (2, 21), (4, 21)] {
        let oracle = ModularExponentiation::new(a, n, 10).unwrap();
        assert_eq!(oracle.eval(0), 0); // placeholder, TODO(M5)
    }
}
