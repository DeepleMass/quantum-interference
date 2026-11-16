# COMPLEXITY — A Complexity-Theoretic Companion

This file collects the complexity-theoretic vocabulary used across the three
specifications, the dialogue in `README.md`, and the journals: what each
term means, how the terms relate, and where quantum computing sits inside that
landscape. It is a companion, not a textbook — read it when a SPEC or the
dialogue uses a term whose exact content you want to check.

## Contents

1. [Machine models and complexity classes](#machine-models-and-complexity-classes)
2. [What is and is not known about BQP](#what-is-and-is-not-known-about-bqp)
3. [A taxonomy of speedups](#a-taxonomy-of-speedups)
4. [Efficient classical simulation as a complexity statement](#efficient-classical-simulation-as-a-complexity-statement)
5. [Statistical complexity: estimation and its limits](#statistical-complexity-estimation-and-its-limits)
6. [Cryptographic asymmetries](#cryptographic-asymmetries)
7. [Fault tolerance: the price of "polynomial"](#fault-tolerance-the-price-of-polynomial)
8. [Where the three phases sit](#where-the-three-phases-sit)
9. [Standing caveats](#standing-caveats)

## Machine models and complexity classes

- **P** — problems decidable by a deterministic machine in polynomial time. The class of "efficiently solvable", by convention.
- **NP** — problems whose solutions are verifiable in polynomial time. Solving and verifying are different acts; NP-completeness is a statement about verification structure, not about how hard typical instances are.
- **BPP** — problems solvable by a randomized classical machine in polynomial time with error probability at most 1/3 per side. The bound 1/3 is conventional: independent repetition plus majority vote drives the error down exponentially (Chernoff). Repetition-as-amplification is the same statistical technique Phase 3 rebuilds with seeded sampling.
- **BQP** — problems solvable by a quantum computer (a uniform family of polynomial-size quantum circuits) with the same bounded-error convention. Measurement yields a random outcome; BQP is a probability statement, exactly like BPP.

## What is and is not known about BQP

The known inclusions are

```
P ⊆ BPP ⊆ BQP ⊆ AWPP ⊆ PP ⊆ PSPACE
```

No inclusion in this chain is known to be strict. In particular, nobody has
proved that a quantum computer solves any natural problem faster than a
classical one: every concrete "quantum advantage" — Shor, Grover, amplitude
estimation — is a statement relative to the best known classical algorithm, not
to a proven lower bound on all classical algorithms.

Two facts discipline the intuition:

- BQP is not believed to contain NP. If NP ⊆ BQP, then — since BQP ⊆ PP and by Toda's theorem PH ⊆ P^PP — the polynomial hierarchy collapses. Complexity theory reads this as strong evidence that BQP does not contain NP: quantum computers are not general NP solvers, and Shor's algorithm is not a generic optimization engine.
- Oracle separations exist where unconditional statements do not. Simon's problem (Phase 2) has a provable exponential separation in the oracle model: any classical algorithm needs exponentially many oracle queries, the quantum one linearly many. This is the cleanest provable statement behind Phase 2 — with the caveat that oracle results do not transfer to concrete problems.

## A taxonomy of speedups

- **Superpolynomial, structure-exploiting — Shor.** Best known classical factoring is the number field sieve, subexponential of the form L_N[1/3]; Shor is polynomial, so the gap is superpolynomial. The structure (periodicity) is the resource; this is why the same algorithm covers discrete logarithms and why it says nothing about unstructured search.
- **Provable quadratic, unstructured — Grover.** About √N oracle queries, and the Bennett–Bernstein–Brassard–Vazirani bound says every quantum algorithm needs at least √N. Grover is optimal, quadratic is the ceiling for black-box search, and a longer key neutralizes the advantage exactly (AES-128 → 2^64, AES-256 → 2^128).
- **Quadratic in statistical estimation — amplitude estimation.** N Grover-powered quantum calls buy the accuracy of N² classical Monte Carlo samples. This is Grover's speedup in analytic form, provable in the query model, and the subject of Phase 3.

## Efficient classical simulation as a complexity statement

"Hard to simulate classically" is itself a complexity claim, and a nuanced one:

- **Gottesman–Knill.** Circuits built from H, S, CNOT and Pauli measurements are classically simulable in polynomial time via the stabilizer formalism: a tableau of O(n²) bits replaces the 2^n amplitude vector. Entanglement alone is not the resource — highly entangled stabilizer states simulate in microseconds.
- **The resource is non-Clifford behavior.** T-count and magic states measure how far a circuit escapes the stabilizer class. Shor's circuit is non-Clifford (the QFT and the controlled modular multiplier are), which is why it resists stabilizer simulation.
- **Limited entanglement admits tensor-network simulation.** Again, structure bounds the classical cost; the exponential price is paid only where structure is absent.
- **Phase 1 deliberately ignores all of this.** `qsim` carries the full 2^n vector and pays O(2^n) per gate. The exponential is the course material: the simulator exists to make you feel it, and the ~24-qubit scope limit is the same statement in engineering terms.

Suggested deep-dive: implement a minimal stabilizer simulator and compare what it can and cannot reproduce — see the root `REPORTME.md` index.

## Statistical complexity: estimation and its limits

Phase 3 treats a speedup as an estimation problem, and estimation has its own
complexity theory:

- **Classical Monte Carlo:** estimating a mean to additive error ε needs Θ(1/ε²) samples; error ~ 1/√N, a scaling exponent α = 1/2.
- **Amplitude estimation:** N Grover-powered queries reach error ~ 1/N, α = 1 — the Heisenberg limit, known from quantum metrology.
- **The Cramér–Rao bound unifies both:** Var(θ̂) ≥ 1/(n · F), with Fisher information F per sample. Classical queries spend a bounded amount of Fisher information each; coherent Grover powers accumulate it quadratically. Maximum-likelihood amplitude estimation is the machinery of classical statistics — likelihood, Fisher information, estimator efficiency — applied to quantum data. This is the bridge from this repository to machine learning.
- **The Phase 3 acceptance test operationalizes the Heisenberg limit:** the fitted scaling exponent α ∈ [0.9, 1.1] against a classical baseline in [0.4, 0.6] tests the exponent, not the constant. The exponent is the claim under test.

## Cryptographic asymmetries

- **Information-theoretic vs computational security.** A scheme unbreakable against unbounded adversaries (one-time pad, QKD keys) versus a scheme whose best attack is a hard problem. Quantum computers threaten the second kind, and only where structure exists.
- **Oracle access defines the threat model.** Simon's algorithm breaks Even–Mansour and some Feistel and MAC constructions given quantum access to the cipher oracle (the Q2 model); AES resists because no exploitable algebraic structure is known. The lesson generalizes: every post-quantum security statement is relative to an adversary model.
- **Harvest now, decrypt later.** Shor's asymmetry is retroactive for recorded traffic. The migration pressure (ML-KEM, ML-DSA, SLH-DSA, standardized by NIST in 2024) follows from that timeline, not from any deployed quantum computer.

## Fault tolerance: the price of "polynomial"

BQP counts logical gates. Physical devices err per operation, and the
threshold theorem buys logical reliability at a polylogarithmic overhead in
1/ε per gate — in current practice, hundreds to thousands of physical qubits
per logical one. The asymptotics survive; the constants are enormous. Every
"quantum computers will do X" claim should be read at this double resolution:
polynomial in n, brutal in the prefactor.

## Where the three phases sit

| Phase | Complexity concept made executable |
|---|---|
| qsim | The 2^n itself: state-vector memory and per-gate cost, felt in code you own |
| qhsp | Algebraic structure as the resource separating exponential (Fourier sampling) from quadratic (black-box search); oracle separations (Simon) as the provable case |
| qae | Statistical query complexity: 1/√N vs 1/N estimation, the Cramér–Rao bound, Heisenberg scaling as an acceptance criterion |

## Standing caveats

- Every speedup here is relative to the best known classical algorithm unless stated otherwise.
- Oracle separations prove separations about oracles, not about the concrete problems they model.
- Asymptotic statements hide constants; fault tolerance hides the largest ones.
