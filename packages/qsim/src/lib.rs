//! pyo3 binding layer (SPEC-PHASE-1 section 5).
//!
//! Exposes the Rust core to Python as the class `qsim.Simulator`. One joint state vector
//! over `m + n` qubits (input register `m`, output register `n`) — not two separate
//! vectors; that is course material, not an implementation detail.
//!
//! Documented ownership decision (SPEC section 11): `amplitudes()` is exposed as a
//! **read-only** NumPy view without copying; no Python-side state mutation, ownership
//! stays in the Rust core. Seed discipline: every stochastic path takes its seed from the
//! `Simulator` handle (stored here), never from a global RNG.
//!
//! # Scaffold status
//!
//! The class and method signatures are in place; all methods are placeholders
//! (`TODO(M6)`) returning defaults and delegating nothing yet.

// Known clippy false positive: `PyResult<T>` return types of `#[pymethods]` are flagged
// as `useless_conversion` by clippy 1.99; allowed here, CI runs clippy with -D warnings.
#![allow(clippy::useless_conversion)]

use std::collections::HashMap;

use num_complex::Complex;
use numpy::npyffi::NPY_ARRAY_WRITEABLE;
use numpy::{PyArray1, PyUntypedArrayMethods};
use pyo3::prelude::*;

use qsim_core::measure::Counts;

/// Python-facing simulator: `input_qubits`-qubit input register plus
/// `output_qubits`-qubit output register, one joint state vector.
#[pyclass]
pub struct Simulator {
    input_qubits: usize,
    output_qubits: usize,
    // TODO(M6): state vector (qsim_core::state::StateVector over input+output qubits)
    // and the seeded RNG handle (rand_chacha::ChaCha8Rng) owned by this struct.
}

#[pymethods]
impl Simulator {
    /// Creates a simulator with an `input_qubits` input register and an
    /// `output_qubits` output register, initialized in |0...0>.
    ///
    /// TODO(M6): construct the joint state vector and seed the RNG handle.
    #[new]
    pub fn new(input_qubits: usize, output_qubits: usize) -> Self {
        Self {
            input_qubits,
            output_qubits,
        } // TODO(M6)
    }

    /// Number of input qubits (m).
    #[getter]
    pub fn input_qubits(&self) -> usize {
        self.input_qubits
    }

    /// Number of output qubits (n).
    #[getter]
    pub fn output_qubits(&self) -> usize {
        self.output_qubits
    }

    /// Total qubit count m + n.
    pub fn num_qubits(&self) -> usize {
        self.input_qubits + self.output_qubits
    }

    /// Applies a named gate to `targets`, optionally controlled on `controls`.
    ///
    /// `gate` is one of "h", "x", "y", "z", "s", "t", "p", "u", "cnot", "cz", "swap"
    /// (parametric gates take extra args — TODO(M6): extend the signature).
    /// Aligned with the later Primitives world (SPEC section 5).
    ///
    /// TODO(M6): dispatch to qsim_core::gates. Placeholder: no-op.
    pub fn apply(&mut self, gate: &str, targets: Vec<usize>, controls: Vec<usize>) -> PyResult<()> {
        let _ = (gate, targets, controls); // TODO(M6)
        Ok(())
    }

    /// Applies the modular-exponentiation oracle `f(x) = a^x mod n` to the joint
    /// register (input register at the bottom, output register above it).
    ///
    /// TODO(M6): build qsim_core::oracle::ModularExponentiation and apply it.
    /// Placeholder: no-op.
    pub fn apply_oracle(&mut self, a: u64, n: u64) -> PyResult<()> {
        let _ = (a, n); // TODO(M6)
        Ok(())
    }

    /// Applies the QFT to the input register. TODO(M6); placeholder: no-op.
    pub fn apply_qft(&mut self) -> PyResult<()> {
        // TODO(M6): qsim_core::qft::qft_gate_sequence on qubits 0..input_qubits
        Ok(())
    }

    /// Samples `shots` full-register outcomes; bitstrings are big-endian display strings.
    ///
    /// TODO(M6): seeded sampling from the handle's RNG. Placeholder: empty counts.
    pub fn sample(&mut self, shots: usize, seed: u64) -> PyResult<Counts> {
        let _ = (shots, seed); // TODO(M6)
        Ok(HashMap::new())
    }

    /// Probability distribution over all `2^(m+n)` basis states as a NumPy array.
    ///
    /// TODO(M6): |amplitude|^2 per basis state. Placeholder: zeros.
    pub fn probabilities<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let n = 1usize << self.num_qubits();
        Ok(PyArray1::zeros_bound(py, [n], false)) // TODO(M6)
    }

    /// Complex amplitudes as a **read-only** NumPy array (SPEC section 11: no copy of
    /// ownership into Python, no Python-side mutation — ownership stays in the Rust
    /// core). Shape `(2^(m+n),)`, dtype complex128.
    ///
    /// TODO(M6): zero-copy read-only view onto the core's amplitude buffer.
    /// Placeholder: freshly allocated zeros with the WRITEABLE flag cleared.
    pub fn amplitudes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<Complex<f64>>>> {
        let n = 1usize << self.num_qubits();
        let arr = PyArray1::zeros_bound(py, [n], false); // TODO(M6)
                                                         // SAFETY: we own `arr` and only clear the WRITEABLE flag; the buffer and its
                                                         // lifetime are untouched. rust-numpy 0.22 has no safe setter for this flag.
        unsafe {
            (*arr.as_array_ptr()).flags &= !NPY_ARRAY_WRITEABLE;
        }
        Ok(arr)
    }

    /// Resets the joint register to |0...0> and reseeds the RNG handle.
    /// TODO(M6); placeholder: no-op.
    pub fn reset(&mut self) -> PyResult<()> {
        // TODO(M6)
        Ok(())
    }
}

/// The native module, re-exported by `qsim/__init__.py`.
#[pymodule]
fn qsim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Simulator>()?;
    Ok(())
}
