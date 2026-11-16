# Implementation Specification: Didactic State-Vector Quantum Simulator (Phase 1)

## 1. Purpose and Scope

This project implements a state-vector simulator for quantum circuits as a Rust core library with Python bindings. The driving application is period finding (order finding), the core of Shor's algorithm, for small numbers N ≤ 1021 — didactically motivated, with no production ambitions. The simulator deliberately follows Qiskit conventions so that Phase 2 (Qiskit migration) and later cross-validation work without semantic breaks.

Non-goals, which the coding agent must not implement: fault tolerance, noise models, density-matrix simulation, GPU backends, end-to-end Shor for large N. Anything beyond ~24 qubits is explicitly out of scope; the 2^n amplitude explosion is course material, not an optimization task.

## 2. Architecture

Three layers. The Rust core implements state vector, gates, QFT, measurement, and oracles as pure, self-contained units with no runtime dependencies beyond num-complex, thiserror, and rand/rand_chacha. The binding layer exposes the core API to Python via pyo3 and rust-numpy; amplitudes are exposed as a NumPy array without copying. The Python layer orchestrates experiments and classical pre- and post-processing (gcd, continued fractions, retry logic) and generates reports.

The package qsim lives in the quantum-interference monorepo (Cargo workspace for Rust crates, uv workspace for Python packages). Repository language is English throughout — code, comments, reports, commits. Commit convention: Conventional Commits.

## 3. Repository Structure (monorepo excerpt)

```
quantum-interference/
├── Cargo.toml                  # Rust workspace root
├── crates/
│   └── qsim-core/              # Rust core library
│       ├── src/
│       │   ├── lib.rs
│       │   ├── state.rs        # StateVector
│       │   ├── gates.rs        # single-qubit, controlled, general gates
│       │   ├── qft.rs          # QFT, inverse QFT, as gate sequence
│       │   ├── measure.rs      # sampling, marginals, collapse
│       │   ├── oracle.rs       # function oracles, modular exponentiation
│       │   └── cfrac.rs        # continued fractions
│       └── tests/              # Rust integration tests
├── packages/
│   └── qsim/                   # Python package (bindings + orchestration
│       ├── src/qsim/
│       │   ├── __init__.py     # re-exports the pyo3 module
│       │   ├── postprocess.py  # continued fractions, order reconstruction, retries
│       │   └── report.py       # markdown/CSV report generation
│       ├── tests/              # pytest: unit + integration + Qiskit cross-check
│       └── pyproject.toml      # maturin build backend
├── scripts/
│   └── run_period_finding.py   # main script (experiment)
├── docs/reports/               # generated experiment reports
├── .github/workflows/ci.yml
└── README.md
└── REPORTME.md
```

## 4. Rust Core Library: Requirements per Module

### 4.1 state.rs — State Vector

Type StateVector with Vec of Complex<f64>, length 2^n. Bit convention: little-endian like Qiskit, qubit 0 is the least significant bit of the basis index. Invariant: Euclidean norm 1, tolerance 1e-12, checked in debug assertions and tests. API: construction in basis state |0…0>, arbitrary basis state, uniform superposition, amplitude access with index-to-bitstring conversion (display big-endian, as customary in the literature).

### 4.2 gates.rs — Gates

Implemented as 2x2 and 4x4 complex matrices plus a generic application routine apply_unitary(sv, u, targets) that applies a k-qubit gate to arbitrary qubit positions (index-permutation technique over bit masks, not via the Kronecker product of the full space; this is the central didactic optimization step, O(2^n) instead of O(4^n)). Minimum set: H, X, Y, Z, S, T, P(theta), general U(theta, phi, lambda) following the Qiskit parametrization, CNOT, CZ, SWAP, generic controlled gate apply_controlled(u, targets, controls). Numerics: unitarity check max|U†U − I| < 1e-12 as a test function, not at runtime.

### 4.3 qft.rs — Quantum Fourier Transform

Implemented as a gate sequence (Hadamards and controlled phase rotations, O(n²) gates), not as a matrix — the matrix variant is kept as a function for small n and serves as the reference in tests: sequence QFT and matrix QFT must agree on basis states to 1e-12. Plus qft_inverse. Phase factors are e^(2πi/2^k); f64 precision suffices for n ≤ 24.

### 4.4 measure.rs — Measurement

Full-register measurement: sampling from |amplitude|² with a seeded RNG (rand_chacha, seed held explicitly by the Simulator handle, reproducible). Marginal distribution over sub-registers (summing all amplitudes with fixed bits at the remaining positions). Single-qubit measurement with collapse (renormalization). Counting API for multi-shot statistics. No GPU, no multithreading.

### 4.5 oracle.rs — Function Oracles

Trait FnOracle with eval(x: u64) -> u64, width_in(), width_out(). Application as a permutation on basis states: |x>|y> -> |x>|y XOR f(x)>, O(2^{n_in}) evaluations of f. Concrete implementation ModularExponentiation with f(x) = a^x mod n, internally square-and-multiply. The oracle is the only place where problem size enters the simulator; it is deliberately computed classically, exactly as in real Shor circuits.

### 4.6 cfrac.rs — Continued Fractions

Own implementation of the continued-fraction expansion of y/Q with the denominator bounded by r < N; returns the best approximant j/r according to the standard convergent logic of order finding. No Python fractions in Rust; in Python, Fraction.limit_denominator may serve as a cross-check in tests.

### 4.7 Error Handling

thiserror error enum (QSimError): qubit index out of range, dimension mismatch on gate application, norm violations, invalid oracle parameters. No panics in the public API; everything through Result.

## 5. Python API (pyo3 contract)

Class Simulator with an m-qubit input register and an n-qubit output register (one joint state vector over m + n qubits, not two separate vectors — this, too, is course material). Methods, minimal and aligned with the later Primitives world: apply(gate, targets, controls), apply_oracle(oracle), sample(shots, seed) -> Counts, probabilities() -> numpy array, amplitudes() -> numpy array (read-only), reset().

## 6. Main Script: Period Finding

scripts/run_period_finding.py implements the full pipeline for given N and a: parameter choice Q = 2^m with N² ≤ Q < 2N², oracle construction, Hadamards on the input register, oracle, QFT on the input register, measurement of the input register, continued-fraction reconstruction of r, consistency check a^r ≡ 1 mod N, optional factor extraction via gcd(a^{r/2} ± 1, N) as a proof of concept for N ≤ 35. The classical part may use math.gcd and pow. Statistics: configurable number of runs, success probability across repetitions, reporting of failed runs (odd r, divisor loss through gcd(j, r) > 1, degenerate case a^{r/2} ≡ −1 mod N).

## 7. Test Strategy

### 7.1 Rust Unit Tests (per module)

state.rs: norm invariant after every operation, basis-state initialization, bitstring round-trip. gates.rs: action of every gate on every basis state (n ≤ 3, exhaustive), X∘X = I, H∘H = I, H produces the uniform superposition, controlled gates as controlled versions of base gates, generic unitarity check on random 2x2 unitaries (QR decomposition as source). qft.rs: QFT of |0> is uniform, QFT of a basis state |k> is the plane wave with the expected phase structure, QFT∘QFT-inverse = I (1e-12), sequence against matrix implementation. measure.rs: chi-square tolerance test of the empirical against the theoretical distribution with fixed seed and ≥ 10^4 samples, collapse renormalization, deterministic reproducibility under the same seed. oracle.rs: f(x) = a^x mod n against a pow reference for all x < 2^m, XOR composition correct, bijectivity of y XOR f(x) in y. cfrac.rs: known approximants (y/Q -> j/r) against documented reference cases, denominator bound respected.

### 7.2 Rust Integration Tests (tests/)

End-to-end order finding at the Rust level for known instances: ord(7, 15) = 4, ord(11, 15) = 2, ord(2, 21) = 6, ord(4, 21) = 3. Acceptance: over 50 measurement runs, r is correctly reconstructed in at least 80 % of reconstructible runs; the fraction of non-reconstructible measurements (gcd(j, r) > 1) stays below 50 %, consistent with theory.

### 7.3 Property-Based Tests (proptest)

Norm preservation under random gate sequences (length ≤ 50, register width ≤ 8), amplitude modulus within [0, 1], no NaN/Inf under any allowed parameters. At least 100 cases per property in CI, seed in the report.

### 7.4 Python Tests (pytest)

Unit: postprocess continued fractions against fractions.Fraction.limit_denominator as reference (allowed only as a test reference, never in the production path), retry statistics logic, report format. Integration: run_period_finding callable as a library function; for N ∈ {15, 21} the factorization is found within at most 20 attempts, deterministic under a fixed seed. Cross-validation against Qiskit (marked slow, optional in CI): the same order-finding circuit in Qiskit and in the own simulator, total variation of the measurement distributions < 0.05 at 10^4 shots. The prerequisite is the identical bit convention (little-endian); this test is the actual acceptance of Phase-1 compatibility.

## 8. CI (GitHub Actions, .github/workflows/ci.yml)

Jobs: rust-checks (fmt --check, clippy -D warnings, cargo test in release profile, coverage via cargo-llvm-cov optional), python-checks (ruff, maturin develop, pytest without the slow marker, Qiskit pinned to a fixed minor version), build (maturin build sdist + wheel for linux-x86_64, uploaded as workflow artifact). Triggers: PRs and pushes to main. Python tests only run when the Rust build is green (needs: dependency). Test reports via the actions summary; junitxml upload as needed.

## 9. Reports on GitHub

REPORTME as a living document with sections: build instructions (maturin develop), architecture rationale (why state vector, why Rust, why Qiskit conventions), naming conventions, pointer to docs/reports. The main script writes one markdown report per experiment to docs/reports/<N>_<a>_<timestamp>.md containing configuration (qubits, Q, seed, shots), the measurement histogram as an ASCII/CSV table, reconstructed r candidates, success rate, and a short interpretation; an experiment run reproduces via the documented seed configuration. The CI test report aggregates the acceptance criteria from 7.2 as a compact table in the job summary. Reports are committed; large raw histograms go to workflow artifacts instead.

## 10. Milestones and Acceptance Criteria

| M | Content | Acceptance | Reading |
|---|---------|------------|---------|
| M1 | Repo skeleton, workspaces, green CI with a trivial test | CI passes, badge in REPORTME | — (tooling) |
| M2 | state.rs + gates.rs complete, unit tests 7.1 | All gate tests green, norm invariant verified | N&C §2.1 Linear algebra, pp. 61–79; §2.2.1–2.2.2, pp. 80–83; §4.2–4.3, pp. 174–184 |
| M3 | qft.rs including sequence and matrix variant | QFT reference tests from 7.1 green | N&C §5.1 The quantum Fourier transform, pp. 217–220 |
| M4 | measure.rs + seeded sampling | Chi-square and reproducibility tests green | N&C §2.2.3 Quantum measurement, pp. 84–86; §4.4 Measurement, pp. 185–187 |
| M5 | oracle.rs + cfrac.rs + Rust integration tests | Order finding meets the rates from 7.2 for all four reference instances | N&C §5.3 Applications: order-finding and factoring, pp. 226–233; continued fractions on p. 229 |
| M6 | Python bindings, postprocess, main script, reports, Qiskit cross-check | N = 15 and N = 21 factorized in a demo, TV-distance test passes, report generated | — (tooling; conventions per section 11) |

Reading column: references are to Nielsen & Chuang, *Quantum Computation and Quantum Information* (10th anniversary edition), cited by section and print page; papers are cited as in the README references section. See also `LINALG.md` and `COMPLEXITY.md`.

Each milestone is one PR against main, every CI stage must be green, merge only after full acceptance. The agent works milestone by milestone: feed it the respective SPEC section, not the whole document at once.

## 11. Conventions and Documented Decisions

Little-endian indexing (Qiskit convention), big-endian display of bitstrings. Central documented decision in the REPORTME: amplitudes are exposed as a read-only NumPy view; no Python-side state mutation, so ownership stays in the Rust core. Seed discipline: every stochastic path takes its seed from the simulator handle, never from a global RNG. Rust: edition 2021, no unsafe blocks except where rust-numpy forces them. Python: fully type-hinted, strict ruff, Python ≥ 3.10.

## 12. Instructions for the Coding Agent (VS Code)

One prompt per milestone containing the SPEC section, the repository state, and the acceptance criterion. Ground rules: no functionality before its tests, no abstraction beyond what a milestone requires, on any error read the failing test first and quote the specification. Deviations from this specification are justified and documented in the PR; silent deviations are rejected.
