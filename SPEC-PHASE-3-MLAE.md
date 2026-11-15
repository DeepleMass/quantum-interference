# Implementation Specification: Grover and ML Amplitude Estimation (Phase 3)

## 1. Purpose and Scope

This project implements Grover-based amplitude estimation as a statistical learning problem. The didactic thesis of this phase: unstructured search and quadratic speedup can be expressed entirely in the language of estimation theory — parametric model, likelihood, estimator, efficiency bound — and this is exactly where quantum computation meets the ML discipline. The core is maximum-likelihood amplitude estimation (MLAE) following Suzuki, Uno, Yoda, Kobayashi, and Yamamoto: measurements at variable Grover iteration counts produce a Bernoulli sample whose parameter is estimated by maximum likelihood — without a phase register, with the same asymptotic precision as canonical QAE.

The didactic ladder has four rungs: a classical Monte Carlo baseline, Grover search as demonstrator, canonical amplitude estimation via phase estimation as the reference method, and MLAE including Fisher information, the Cramér–Rao bound, and an own optimizer. Applications are the estimation of pi via an indicator oracle and, as the advanced instance, Grover-accelerated option pricing as Monte Carlo integration.

Non-goals: production financial mathematics, QUBO optimization, QAOA, amplitude estimation under noise beyond a simple readout-error model, performance tuning.

## 2. Architecture

Three layers. The quantum layer builds Grover operators: indicator oracle as phase oracle, diffuser, Grover iteration G, plus state preparation for the distribution loading of the Monte Carlo instances. The statistics layer is the core of the project: Bernoulli model with the parametrization a = sin²(theta), likelihood over a measurement schedule K = (k_1, …, k_m) with shot counts (n_1, …, n_m), MLE, Fisher information, Cramér–Rao bound, confidence intervals, an own gradient and Newton optimizer on the log-likelihood, and a Bayesian variant with a grid posterior. The experiment layer orchestrates method comparisons, scaling measurements, and report generation.

Backend access is inherited from Phase 2: the project depends on the qhsp package and uses its Target abstraction (Sampler V2, profiles ideal/noisy/remote) unchanged. Phase 3 is thus hardware-capable without maintaining a second backend layer. The quantum layer knows Qiskit only for circuit construction; all statistics is framework-free NumPy.

Central documented decision: SciPy and statsmodels are test references only. The production path — likelihood, derivatives, Fisher information, optimizer, intervals — is implemented independently, because deriving these is the actual learning content.

## 3. Repository Structure (monorepo excerpt)

```
quantum-interference/
├── packages/
│   └── qae/
│       ├── src/qae/
│       │   ├── grover.py        # oracle, diffuser, iteration, search demo
│       │   ├── qpe_ae.py        # canonical AE via phase estimation (reference)
│       │   ├── mlae/
│       │   │   ├── model.py     # Bernoulli model, log-likelihood, derivatives
│       │   │   ├── fisher.py    # Fisher information, CRB
│       │   │   ├── estimator.py # MLE: grid, gradient, Newton
│       │   │   ├── bayes.py     # grid posterior, credible intervals
│       │   │   └── schedule.py  # measurement schedules K, adaptive variants
│       │   ├── mc_apps/
│       │   │   ├── pi_oracle.py # pi estimation via an indicator
│       │   │   └── option.py    # European call, discretized distribution
│       │   └── report.py        # scaling and comparison reports
│       ├── tests/
│       │   ├── unit/
│       │   ├── integration/
│       │   └── scaling/         # empirical Heisenberg scaling
│       └── pyproject.toml        # dependency on qhsp via the workspace
├── scripts/
│   └── run_qae_experiment.py    # main script (experiment)
├── docs/reports/
├── .github/workflows/ci.yml
└── README.md
```

## 4. Module Requirements

### 4.1 grover.py — Grover Core

Indicator oracle: a function marks hits; as a circuit, a phase oracle is built over the search space of 2^n states, compiled from the classical function for small n. Diffuser as 2|psi><psi| − I. Iteration G = D·O. Search demo: locating the marked element, empirical hit probability against the theoretical curve p(k) = sin²((2k+1)theta) for all k up to the optimal iteration count — this curve is documented as a plot in the report and its maximum checked against k_opt = floor(pi/(4 theta)). Essential for MLAE: G^k as a unified operator with configurable k, because the measurement schedule varies k.

### 4.2 qpe_ae.py — Canonical AE as Reference Method

Amplitude estimation via quantum phase estimation: phase register, controlled G^j, inverse QFT, reading theta from the phase measurement, a = sin²(theta). This module is deliberately classical in structure, as discussed in Phase 1: the QFT is classically trivial; the cost lies in the phase register and the controlled powers. It serves as the accuracy and cost reference: qubits (phase register) and gate depth versus MLAE, which needs neither.

### 4.3 mlae/model.py — Bernoulli Model and Log-Likelihood

Parametrization a = sin²(theta), 0 ≤ theta ≤ pi/2. For a schedule (k_i, n_i) with hit counts h_i, the log-likelihood is the sum of Bernoulli terms with single-shot probability p_i(theta) = sin²((2k_i+1)theta). Independent implementation of value, first and second derivative with respect to theta, analytically; the derivatives are the core of the learning bridge to optimization and Fisher information. Numerical stability: handling of angles near pi/2 where sin(2 theta) terms vanish, and of p near 0 and 1.

### 4.4 mlae/fisher.py — Fisher Information and CRB

Fisher information as the negative expected second derivative of the log-likelihood, closed-form evaluable for the Bernoulli case. From it, the Cramér–Rao bound for the variance of any unbiased estimator of theta and of a. The module provides the theoretical lower bound against which the own estimator is measured empirically in integration tests: the measured variance of the MLE across repetitions must lie above the CRB and converge toward it for large samples — this is the central acceptance of the project.

### 4.5 mlae/estimator.py — MLE with Own Optimizer

Three estimators on the same model, didactically staged: grid search over theta as a robust baseline, gradient ascent on the log-likelihood with step-size control, Newton's method with the analytic second derivative. Convergence criteria, restart strategy against local maxima (the likelihood in theta is multimodal at large k_i — documented didactic content, not an implementation bug). Cross-validation: on fixed datasets all three must find the same maximizer (tolerance), and the Newton estimator must hold up against SciPy minimize as a test reference.

### 4.6 mlae/bayes.py — Bayesian Variant

Grid posterior over theta with a flat prior, update with the same data, maximum a posteriori and credible intervals from the grid. Bridge to ML training: comparing MLE and Bayes on the same Bernoulli model shows shrinkage and interval quality. Optional adaptive schedule: choice of the next k_i by expected Fisher information (experimental-design loop), documented as a sequential extension of the schedule conventions.

### 4.7 mlae/schedule.py — Measurement Schedules

Linear schedule K = {0, 1, …, K_max}, geometric schedule K = {2^0, 2^1, …}, configurable budget allocation over the n_i. The budget is the sum of Grover applications Σ k_i·n_i; it is the fair cost currency for the comparison against classical Monte Carlo, not the number of measurements.

### 4.8 mc_apps/ — Applications

pi_oracle.py: indicator on a square-circle discretization grid, a = pi/4, back-computing pi = 4a. option.py: European call on a log-normal distribution discretized over 2^n scenarios, payoff indicator via a threshold, state preparation as amplitude encoding of the distribution, estimation of the discounted expectation value — classically computable by exhaustion as ground truth, since 2^n is small. The option provides the realistic framing; it stays didactically small.

## 5. Main Script

scripts/run_qae_experiment.py with three modes: --mode search (Grover search demo with the p(k) curve), --mode compare (classical, QPE-AE, and MLAE on the same oracle with budget alignment via Σ Grover applications), --mode scaling (scaling measurement: precision error versus budget for classical and MLAE over logarithmically graded budgets, regression fit of the exponent alpha in the law error ~ budget^{-alpha}). Reports as markdown with configuration, point cloud as CSV, summarized exponents, and a method table; scaling reports show the Heisenberg line alpha ≈ 1 against the Monte Carlo line alpha ≈ 0.5. The target is always ideal; the noisy profile is for observation experiments, not part of acceptance.

## 6. Test Strategy

### 6.1 Unit Tests

grover.py: phase oracle acts only on marked states (Statevector check), diffuser unitary and symmetric, p(k) curve against the formula for all k at n = 4. model.py: log-likelihood values against hand-computed fixed points, derivatives against numerical differentiation (two-point, tolerance-checked), multimodality at large k reproduces the documented structure. fisher.py: analytic Fisher information against the numerical second derivative of the log-likelihood in expectation, CRB positivity, scaling ~ k². estimator.py: all three optimizers find the same maximum on fixed datasets, Newton against the SciPy reference, convergence within a documented number of iterations. bayes.py: posterior normalized, MAP and MLE close together for a flat prior and large data. schedule.py: budget computation, schedules well-formed.

### 6.2 Integration Tests

MLAE end-to-end on the ideal target with fixed seed: known amplitudes a ∈ {0.1, 0.4, 0.785 (pi/4), 0.9}, linear and geometric schedule, a returned with relative error < 5 % at moderate budget. Variance acceptance: over 200 repetitions the empirical variance of the estimator lies above the CRB and at most a factor 4 above it; the quotient tends to 1 with growing budget. QPE-AE as reference on the same instance: point estimators agree within the combined standard errors.

### 6.3 Scaling Tests (Project Acceptance)

Empirical exponent alpha from the regression fit error ~ budget^{-alpha} over at least five budget levels, two seeds, ideal target: MLAE alpha ∈ [0.9, 1.1], classical Monte Carlo alpha ∈ [0.4, 0.6]. Tolerances deliberately wide, since small-sample noise dominates the fit; the test checks the order of magnitude, not the constant. The fit itself (linear regression in log-log space) is part of the project code and is itself tested.

### 6.4 Cross-References

Grover search demo against Phase-1 qsim: the same circuit as a qsim sequence, distribution comparison as established in Phase 2. The model layer needs no quantum computer and is tested exclusively against analytic and numerical references — statistics errors here are never quantum errors.

## 7. CI (GitHub Actions)

Jobs: lint (ruff), test-fast (unit, QAE integration on the smallest instances), test-scaling (scaling acceptance, only with label scaling or weekly, compute-intensive), build (wheel, artifact). Dependencies: qiskit and qhsp pinned; qhsp must be available from the Phase-2 build, prerequisite is the green M6 state of Phase 2. JUnit XML as artifact. CI is the single monorepo workflow; the jobs described here are the qae jobs and depend on the qhsp build being green.

## 8. Reports on GitHub

README with the thesis "quadratic speedup as a statistical estimation problem", setup, the derivation chain from the Bernoulli model to the CRB in brief, the method matrix (qubits, gate depth, phase-register requirements: classical vs QPE-AE vs MLAE), reproduction instructions with seed discipline. docs/reports/ receives one markdown report per experiment: configuration, point estimates with intervals, empirics versus CRB, scaling exponents, method comparison table; raw data as CSV, fits as coordinate lists, no binary formats in the repository.

## 9. Milestones and Acceptance Criteria

| M | Content | Acceptance |
|---|---------|------------|
| M1 | Package integration into the monorepo, qhsp hookup, green CI with a trivial test | Ideal target from qhsp reachable in CI |
| M2 | grover.py including search demo | p(k) curve matches the formula, k_opt confirmed |
| M3 | qpe_ae.py | QPE estimator returns a on the reference instance, cost matrix documented |
| M4 | mlae/model.py + fisher.py | Analytic derivatives green against numerical reference, CRB correct |
| M5 | mlae/estimator.py + bayes.py + schedule.py | Integration tests from 6.2 met, Newton passed against SciPy |
| M6 | mc_apps + scaling acceptance + reports | alpha window from 6.3 held, pi estimate < 2 % relative error, option against exhaustive ground truth |

Each milestone is one PR; the agent works milestone-wise with the SPEC section as its prompt.

## 10. Conventions and Documented Decisions

SciPy exclusively in tests, never on the production path; every statistical function has an analytic or numerical reference in the test. The budget is Σ k_i·n_i Grover applications, not the number of measurements — all method comparisons convert to this currency, otherwise they compare apples with oranges. theta is the natural parameter of the model, a the reported quantity; variance transformation via the delta method is to be implemented and tested. The multimodality of the likelihood at large k is documented behavior: the grid starting point of Newton is part of the estimator contract. Seeds come from the target or experiment handle. Python ≥ 3.10, fully typed, strict ruff.

## 11. Instructions for the Coding Agent (VS Code)

One prompt per milestone with the SPEC section, the repository state, and the acceptance criterion. Ground rules: have the statistics layer fully tested before the quantum layer, since it is deterministically checkable without a quantum computer — errors there are never noise, always logic. No abstraction beyond the milestones, reference libraries only in tests, every deviation from the specification justified in the PR. After M5 the review question stands: does the MLE empirically meet the CRB with the expected efficiency factor, and is the Heisenberg scaling visible in scaling mode — precisely these two numbers are the didactic end result of this phase.
