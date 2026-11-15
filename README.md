# quantum-interference

A didactic monorepo on quantum computation: from a hand-written state-vector simulator, through a hidden-subgroup framework on Qiskit, to Grover-based amplitude estimation as a statistical learning problem.

## Purpose of This Project

This repository exists to understand quantum computation by building it, not by consuming it. It contains three packages, one per phase, each documented by a specification at the repository root with module-level requirements, test strategy, CI wiring, and milestones with explicit acceptance criteria:

- `SPEC-PHASE-1-SIMULATOR.md` — `qsim`: a state-vector simulator written from scratch (Rust core, Python bindings). Order finding as the core of Shor's algorithm, own QFT, seeded measurement statistics, cross-validated against Qiskit.
- `SPEC-PHASE-2-HSP-FRAMEWORK.md` — `qhsp`: the hidden subgroup problem as an abstract framework on Qiskit with three instances (order finding, Simon, discrete logarithm), and a hardware abstraction built exclusively on Qiskit Runtime primitives, so that moving from ideal simulation via noise models to real devices is a configuration change.
- `SPEC-PHASE-3-MLAE.md` — `qae`: Grover and maximum-likelihood amplitude estimation. The quadratic speedup is treated as a statistical estimation problem — likelihood, Fisher information, Cramér–Rao bound, own optimizers, Bayesian variant — with Monte Carlo applications.

Coding is done by an AI coding agent in VS Code, working milestone by milestone from these specifications. No code exists yet; this is the documentation baseline. The tag `Introduction` marks this point.

## A Dialogue on Quantum Computation

The following dialogue reproduces, in condensed form, the discussion that led to this project: what a quantum computer can and cannot do, where the advantage comes from, and what follows for cryptography and applications.

**Student:** Shor specified the quantum computer by seeing in it a way to solve the hidden subgroup problem, and he gave an algorithm for it. So every problem that reduces to the hidden subgroup problem is solvable on a quantum computer. Correct?

**Professor:** In essence yes, with two qualifications. Shor's algorithm is an instance of the hidden subgroup problem over abelian groups: factoring reduces to it over Z, discrete logarithms over Z × Z. The quantum Fourier transform over such groups runs in polynomial time, which is what makes the solution efficient. The non-abelian generalization, however, is open: graph isomorphism reduces to the HSP over the symmetric group, and no efficient quantum algorithm for it is known. And "solvable" here always means efficiently solvable, in polynomial time.

**Student:** Which other categories of problems can a quantum computer handle, beyond the hidden subgroup problem?

**Professor:** Three matter. First, simulation of quantum systems — Feynman's original motivation: chemistry, materials, ground states, reaction paths; classically exponential in the number of electrons, quantumly essentially linear in simulation time. Second, search and amplitude amplification: Grover finds a hit among N items in about √N steps, and a lower-bound theorem says no quantum algorithm can do better on an unstructured problem — so quadratic, never exponential. Third, amplitude estimation, the analytic sibling of Grover: it quadratically accelerates Monte Carlo methods, which is its most credible economic application. Quantum linear algebra (HHL) exists but carries caveats about input loading and output readout. Adiabatic schemes and QAOA target optimization; their advantage is unproven.

**Student:** Please explain Shor's factoring algorithm.

**Professor:** It reduces factoring to order finding. For a coprime a, the function f(x) = a^x mod N is periodic with the order r, the smallest exponent with a^r ≡ 1 mod N. If r is even and a^{r/2} is not congruent to −1, then gcd(a^{r/2} ± 1, N) yields factors of N with probability at least one half per attempt. Finding r is the hard part, and there sits the quantum kernel: uniform superposition over an input register of size Q ≥ N², the oracle writes a^x mod N, and the quantum Fourier transform turns the periodic structure into sharp peaks at multiples of Q/r. A measurement plus continued fractions reconstructs r. The decisive concept is interference, not parallelism: the machine does not try all candidates and pick one; the Fourier structure condenses global algebraic information into a probability distribution whose measurement yields the period with high probability. That is why Shor is exponential while Grover, a pure search without structure, is capped at quadratic.

**Student:** To what extent is this implementable on a classical binary computer, simulating the quantum machine?

**Professor:** Fully in principle, exponentially expensive in practice. An n-qubit state is a vector of 2^n complex amplitudes; simulators reach roughly 45 to 50 qubits on supercomputers. Remarkably, the QFT itself is classically trivial — it is just the discrete Fourier transform. The cost comes solely from entanglement: an entangled state factorizes into no product of smaller pieces, so the full vector must be stored and updated. Efficient classical simulation exists exactly where entanglement is structurally limited, as in the Gottesman–Knill theorem for Clifford circuits — and Shor's circuit lies outside that class. Running a full classical simulation of Shor for factoring is circular: you pay back the very exponentiality the quantum computer was supposed to avoid.

**Student:** Is that why symmetric encryption cannot be broken, not even by a quantum computer — the effort grows exponentially?

**Professor:** The conclusion is right, the reasoning needs correction. The reason is neither entanglement entropy nor energy, but a proven lower bound and the absence of exploitable structure. Key search against a well-designed cipher is an unstructured black-box problem, and the Bennett–Bernstein–Brassard–Vazirani bound says every quantum algorithm needs at least √N oracle queries. Grover meets that bound exactly: AES-128 falls from 2^128 to 2^64 steps, still exponential — hence the standard recommendation of AES-256. One caveat: this protection is an assumption about the cipher having no hidden algebraic structure, not a theorem. Certain classically secure constructions, such as Even–Mansour and some Feistel and MAC schemes, fall to the Simon algorithm given superposition access to the oracle. For AES no such structure is known.

**Student:** So the quantum advantage arises when an algorithm costs exponential effort on a binary computer and polynomial effort on a quantum computer?

**Professor:** Yes, with three refinements. First, the separation is relative: it holds against the best known classical algorithm — for factoring that is subexponential, the number field sieve; proven exponential classical hardness is not known. Second, not every speedup is of that kind: Grover's quadratic advantage is real and provable, but a longer key neutralizes it, whereas an exponential advantage breaks cryptographic assumptions for good. Third, the asymptotics refer to logical gates: fault tolerance adds a polynomial overhead in physical qubits and gates — hundreds to thousands of physical qubits per logical one — so the constants are enormous while the asymptotic picture stands.

**Student:** What concrete applications follow from all this?

**Professor:** From the structure-exploiting side: Shor breaks RSA and elliptic-curve cryptography, which is why stored traffic encrypted today must be treated as decryptable later and why the migration to post-quantum schemes (ML-KEM, ML-DSA, SLH-DSA, standardized by NIST in 2024) is under time pressure. From the search side: amplitude estimation accelerates Monte Carlo risk and pricing calculations in finance. From simulation: drug discovery, catalysts — the nitrogen-fixation problem alone consumes one to two percent of world energy via the Haber–Bosch process — batteries and materials. The safest near-term benefit is simulation, the most consequential mid-term one is cryptanalysis, and the broadest-advertised but least certain one is optimization.

**Student:** And this repository?

**Professor:** One phase per insight. `qsim` builds the foundation by hand, so you feel the 2^n and the interference in code you own. `qhsp` makes the thesis "one algorithm, three instances" executable: the same abstract procedure, instantiated over three groups, with a hardware abstraction that survives contact with real devices. And `qae` turns Grover's quadratic speedup into a statistical estimation problem — likelihood, Fisher information, Cramér–Rao — the bridge from quantum computing to machine learning. If you have walked all three, you know not only that the quantum advantage exists, but exactly where it comes from and where it ends.

## Status and Workflow

This tag contains documentation only. Work proceeds as specified: one milestone per PR, every CI stage green before merge, the coding agent receiving one SPEC section at a time. Deviations from the specifications are documented in the corresponding PR; silent deviations are rejected.
