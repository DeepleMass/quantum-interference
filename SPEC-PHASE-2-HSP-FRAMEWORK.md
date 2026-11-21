# Implementation Specification: HSP Framework on Qiskit (Phase 2)

## 1. Purpose and Scope

This project implements the hidden subgroup problem ($HSP$) as an abstract `Python` framework on top of `Qiskit`, with three concrete instances:  
* order finding ($HSP$ over $Z_Q$, the core of Shor's algorithm)
* Simon's algorithm ($HSP$ over $Z_2^n$), and the discrete logarithm ($HSP$ over a product of cyclic groups). 

**The goal** is to make visible in code that all three are the same abstract procedure, differing only in group, Fourier transform, and classical post-processing — the central thesis of the Phase-1 discussion.

**Second, equal-ranking goal**: a hardware abstraction layer built exclusively on `Qiskit Runtime primitives` (`Sampler V2`), so that the transition from ideal simulation via noise models to real `IBM` devices is a configuration change, not a refactoring. The Phase-1 simulator (`qsim`) remains a dependency and serves as the zero-noise reference in cross-tests.

**Non-goals**: end-to-end Shor for large N, non-abelian HSP, own fault-tolerance logic, circuit performance tuning beyond what is didactically necessary.

## 2. Architecture

Four layers. 
* The domain layer models abelian groups as products of cyclic factors and defines the $HSP$ problem as a data class: group G, hidden subgroup H (known only to tests), and a coset-invariant function f. 

* The algorithm layer contains the abstract base class `AbelianHSP` with the generic procedure — `superposition`, `oracle`, `Fourier transform over G`, `measurement`, classical `post-processing` — and the three concrete instances as subclasses. 

* The backend layer encapsulates exclusively through a Target object that internally provides a Sampler V2; no Qiskit backend is ever referenced in algorithm code. 

* The experiment layer bundles runs, statistics, and report generation.

**Dependency rule**: the domain and algorithm layers know `Qiskit` only through circuit construction and the Target interface. Everything concerning a concrete device, noise, or transpilation lives in the backend and experiment layers.

## 3. Repository Structure (monorepo excerpt)

```
quantum-interference/
├── packages/
│   └── qhsp/
│       ├── src/qhsp/
│       │   ├── groups.py         # AbelianGroup (product of cyclic groups)
│       │   ├── problem.py       # HSPProblem data class, invariance check
│       │   ├── base.py           # AbelianHSP (abstract procedure)
│       │   ├── instances/
│       │   │   ├── order_finding.py
│       │   │   ├── simon.py
│       │   │   └── discrete_log.py
│       │   ├── fourier.py       # QFT over AbelianGroup, instance dispatch
│       │   ├── backend/
│       │   │   ├── target.py     # Target abstraction around Sampler V2
│       │   │   └── profiles.py  # ideal / noisy / remote profiles
│       │   └── report.py         # markdown reports, transpilation statistics
│       ├── tests/
│       │   ├── unit/
│       │   ├── integration/
│       │   └── cross/           # Qiskit against qsim (Phase 1)
│       └── pyproject.toml        # dependency on qsim via the workspace
├── scripts/
│   └── run_hsp_experiment.py     # main script (experiment)
├── docs/reports/
├── .github/workflows/ci.yml
└── README.md
└── REPORTME.md
```

## 4. Domain Layer: Requirements

### 4.1 groups.py — Abelian Groups

**Class AbelianGroup** as a product of cyclic groups $Z_{n_1} x … x Z_{n_k}$, internally represented as a factor list.   

**Operations**: exponentiation, composition, inverse, element order, generator search, coset representation. 

**For circuit purposes**: mapping of each factor to a qubit register of width $ceil(log2(n_i))$; factors that are not powers of two are embedded into a register via the power-of-two embedding ($Q ≥ N²$ rule as in Phase 1), surplus states documented and discarded in post-processing. 

**Constructors**: $Z(n)$, $Z2_n(k)$, $Z_{p_star}(p)$ with a generator.

### 4.2 problem.py — HSP Problem

Data class `HSPProblem(group, oracle, hidden_subgroup, coset_representative)`. The oracle type is a `callable` from group elements to `int`, plus `width` specifications. 

Verification method `verify_invariance()`: for `sampled x` and all generators `h` of `H`, `f(x) = f(x·h)` must hold, and for samples from different `cosets f` must differ. 

This method is the formal check of the `HSP` precondition and runs as a `unit test` for every concrete instance; an `oracle` that fails it is by definition not a valid `HSP oracle`. `hidden_subgroup` is test knowledge only, never an input to the algorithm.

## 5. Algorithm Layer: Requirements

### 5.1 base.py — Abstract Procedure

Abstract class `AbelianHSP` with a template-method structure. 

The concrete flow in `solve(problem, target, shots) -> HSPResult`: derive the register layout from the `group`, uniform superposition on the `input register`, `oracle` application, `Fourier transform` over the `group` on the `input register`, `measurement`, delegation of the `measurement` data to `classical_postprocess(samples)`, and returning `candidate generators` plus `statistics`. 

Abstract and instance-specific are only: the `oracle circuit construction`, the `Fourier transform` used, and the classical `post-processing`. Repetition logic (enough independent measurement runs for the kernel space of `H`) sits generically in the `base class`, configurable via a `minimum number` of linearly independent samples.

### 5.2 fourier.py — Fourier Transforms

Dispatch over the group structure. 

$Z_2^n$: Hadamard on every qubit — this is the $QFT$ over $Z_2$ and explains why Simon needs no phase gates; the fact is made explicit in the docstring. 

$Z_2^m$: `Qiskit`'s `QFTGate` and its `inverse`. 

Products: tensor product of the component transforms over register partitions. Every Fourier operation has a test correspondence: application to the uniform superposition is a fixed point, application to coset states produces the peaks — both are verified numerically via Statevector in tests, not only via sampling.

### 5.3 instances/ — The Three Instances

order_finding.py: oracle f(x) = a^x mod N, embedded per the Phase-1 rule (Q = 2^m, N² ≤ Q < 2N²), post-processing via continued fractions (reusing the Phase-1 logic as a pure Python function), returning the order r with the consistency check a^r ≡ 1 mod N. simon.py: oracle f: Z_2^n -> Z_2^{n-1} with hidden mask s, classical post-processing as Gaussian elimination over GF(2) on the measurement vectors, returning s; solving a GF(2) linear system is to be implemented independently, not imported from a library. discrete_log.py: discrete logarithm via HSP for a small prime p with generator g, oracle f(a, b) = g^a · y^b mod p, hidden subgroup generated by (t, −1) where g^t = y; post-processing via kernel computation in the group model. All instances are verified against SymPy: n_order and discrete_log as ground truth, again in tests only.

### 5.4 backend/target.py and profiles.py — Hardware Abstraction

Class Target with the profiles ideal (AerSimulator without noise), noisy (AerSimulator with the noise model of a FakeBackend from qiskit_ibm_runtime.fake_provider), and remote (IBM Runtime, real devices, activatable only with credentials). Uniform interface run(circuits, shots, seed) -> CountsList; transpilation happens inside the target against its coupling map and is part of the report (depth, two-qubit gate count, fidelity estimate after transpilation). Algorithm code receives only counts, never backend objects. Switching profiles is an argument in the main script. For the remote profile: no credentials in the repository, configuration via environment variables, CI never executes remote.

## 6. Main Script

scripts/run_hsp_experiment.py runs instance x target x parameter combinations and writes reports. Call pattern: --instance order|simon|dlog, --params (e.g. N, a or n), --target ideal|noisy|remote, --device (FakeBackend name or real device), --shots, --seed, --runs. Output: one markdown report per combination with configuration, measured distribution, reconstructed result, success probability over repetitions, and transpilation statistics of the target; ideal versus noisy directly comparable in the same report. This is the didactic core of Phase 2: the visible effect of noise on the same circuit.

## 7. Test Strategy

### 7.1 Unit Tests

groups.py: group operations, inverses, element orders, generators against brute force for groups up to order 100. problem.py: verify_invariance passes for all valid test problems and demonstrably fails for a mutated oracle (no coset invariance) — the latter as a negative test. fourier.py: Hadamard dispatch for Z_2^n, QFTGate consistency via Statevector comparison, product-register composition. Instance post-processing: order reconstruction from documented (y, Q) pairs, Simon Gaussian elimination over GF(2) with test matrices of known rank, discrete-log kernel against SymPy.

### 7.2 Integration Tests

End-to-end per instance against the ideal target with fixed seeds. Order finding: N ∈ {15, 21, 33}, criterion r correct in ≥ 80 % of runs, analogous to the Phase-1 reference rates. Simon: n ∈ {4, 6, 8}, mask always found with ≤ n + 2 independent measurements, 100 % over 50 repetitions. Discrete log: p ∈ {29, 61}, exponent correct in ≥ 80 %. All ground-truth comparisons via SymPy.

### 7.3 Cross-Tests against Phase 1

The same order-finding circuit as a Qiskit circuit and as a qsim sequence, identical seeds per the Phase-1 convention, total variation of the ideal measurement distributions < 0.05 at 10^4 shots. The test is marked slow and runs in CI. It deliberately does not break on small deviations of the sampling APIs; it compares distributions, not samples.

### 7.4 Noisy Integration Tests

Against a FakeBackend noise model: the success probability drops but must stay above a documented threshold, calibrated once per instance and maintained in the test as a constant with justification (Simon n = 4: ≥ 50 %, order finding N = 15: ≥ 40 % as starting values, to be re-validated on the first real run). These tests are didactic: they measure how much structure survives under noise.

### 7.5 Property-Based Tests (hypothesis)

For generated groups and random hidden subgroups: verify_invariance consistent, Simon post-processing always returns the mask given a complete basis, continued fractions converge for valid (y, Q) pairs. Deterministic seeds, 100 cases in CI.

## 8. CI (GitHub Actions)

Jobs: lint (ruff, format check), test-fast (unit + integration ideal, without slow), test-slow (cross against qsim, FakeBackend noise, only for PRs with a label or manually), build (wheel build, artifact upload). qiskit, qiskit-aer, and qiskit-ibm-runtime are pinned to fixed minor versions; upgrade PRs are separate and tested independently. The remote profile is structurally excluded from CI; no secret in the workflow. JUnit XML reports as artifacts. CI is a single monorepo workflow; the jobs described here are the qhsp jobs and depend on the qsim build being green.

## 9. Reports on GitHub

REPORTME sections: setup, architecture with the thesis "one algorithm, three instances", register conventions, the Target concept and the profile matrix, reproduction instructions including seed discipline. docs/reports/ receives one markdown report per experiment with configuration, histogram, success rates, and transpilation statistics; ideal/noisy comparisons as a table in the same document. Raw distributions as CSV attachments, large histograms as workflow artifacts. The CI job summarizes the acceptance criteria from 7.2 and 7.4 as a table in the job summary.

## 10. Milestones and Acceptance Criteria

| M | Content | Acceptance | Reading |
|---|---------|------------|---------|
| M1 | Package integration into the monorepo, packaging, green CI with a trivial test, qsim dependency resolved | CI runs, both packages imported in the same test | — (tooling) |
| M2 | groups.py + problem.py + unit tests | Invariance and negative tests green | N&C §5.4.3 The hidden subgroup problem, pp. 240–241 (Figure 5.5, p. 241); Appendix 2: Group theory, pp. 611 ff. |
| M3 | base.py + fourier.py + simon.py, end-to-end | Simon acceptance from 7.2 met | N&C §5.4.3, pp. 240–243; D. R. Simon (1997), SIAM J. Comput. 26(5), pp. 1474–1483 (not treated in N&C) |
| M4 | order_finding.py + cross-test against qsim | Order rates and TV distance met | N&C §5.3.1 Application: order-finding, pp. 226–233 |
| M5 | discrete_log.py + hypothesis suite | Discrete-log rate met, property tests green | N&C §5.4.1 Period-finding, pp. 236–237; §5.4.2 Discrete logarithms, pp. 238–239 |
| M6 | Target profiles noisy + remote, transpilation statistics, reports | Noisy thresholds documented and held, remote runbook written | N&C ch. 8, Quantum noise and quantum operations, pp. 353 ff.; §8.1, pp. 354–356; §8.2, pp. 357 ff. |

Reading column: references are to Nielsen & Chuang, *Quantum Computation and Quantum Information* (10th anniversary edition), cited by section and print page; papers are cited as in the README references section. See also `LINALG.md` and `COMPLEXITY.md`.

Each milestone is one PR; the agent works milestone-wise with the respective SPEC section as its prompt. Remote runs on real devices are a manual step after M6, not part of CI.

## 11. Conventions and Documented Decisions

Exclusively Sampler V2, no deprecated backend interface, no execute. Oracles are built as circuits with classically precomputed permutation contents, never as black-box execution at runtime. The hidden_subgroup field is test knowledge; any use on the algorithm path is a test error that must be recognizable through an assertion structure in review. Seed discipline as in Phase 1: seeds come from the target handle, never globally. Qiskit versions pinned, deviations only via upgrade PRs. Python ≥ 3.10, fully typed, strict ruff.

## 12. Instructions for the Coding Agent (VS Code)

One prompt per milestone with the SPEC section, the repository state, and the acceptance criterion. Ground rules: tests before implementation, no abstraction beyond what the milestones require, the dependency rule from section 2 is never violated — in particular, algorithm code never touches a Qiskit backend object. On every boundary case, quote the specification and document the decision in the PR; silent deviations are rejected. After M3, the thesis "one algorithm, three instances" is an explicit review question: the reviewer checks that the base class contains no instance-specific branching for Simon.
