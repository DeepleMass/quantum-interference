# quantum-interference

A didactic monorepo on quantum computation: from a hand-written state-vector simulator, through a hidden-subgroup framework on Qiskit, to Grover-based amplitude estimation as a statistical learning problem.

Human-designed, AI-generated: the specifications, the curriculum, and the dialogue are human work; the scaffold, the test suite, and this documentation were generated from them with AI assistance. Learning means closing the gap between the two.

## Contents

1. [Purpose of This Project](#purpose-of-this-project)
2. [Status and Workflow](#status-and-workflow)
3. [Best Practice for This Repo](#best-practice-for-this-repo)
4. [References](#references)
5. [License](#license)
6. [A Dialogue on Quantum Computation](#a-dialogue-on-quantum-computation)

## Purpose of This Project

This repository exists to understand quantum computation by building it, not by consuming it. It is its author's own self-learning project first: one personal journey through the subject, worked and documented in the open, so that other self-learners and instructors can use it in whichever role fits them. It contains three packages, one per phase, each documented by a specification at the repository root with module-level requirements, test strategy, CI wiring, and milestones with explicit acceptance criteria:

- `SPEC-PHASE-1-SIMULATOR.md` — `qsim`: a state-vector simulator written from scratch (Rust core, Python bindings). Order finding as the core of Shor's algorithm, own QFT, seeded measurement statistics, cross-validated against Qiskit.
- `SPEC-PHASE-2-HSP-FRAMEWORK.md` — `qhsp`: the hidden subgroup problem as an abstract framework on Qiskit with three instances (order finding, Simon, discrete logarithm), and a hardware abstraction built exclusively on Qiskit Runtime primitives, so that moving from ideal simulation via noise models to real devices is a configuration change.
- `SPEC-PHASE-3-MLAE.md` — `qae`: Grover and maximum-likelihood amplitude estimation. The quadratic speedup is treated as a statistical estimation problem — likelihood, Fisher information, Cramér–Rao bound, own optimizers, Bayesian variant — with Monte Carlo applications.

A complexity-theoretic companion, `COMPLEXITY.md`, collects the asymptotic vocabulary used throughout, relates the terms, and places the three phases within that landscape. A linear-algebra companion, `LINALG.md`, does the same for the objects themselves: amplitudes and the Born rule, tensor products, unitaries and projectors, the spectral theorem, and the QFT as the DFT — the mathematics the three phases make executable.

The repository was bootstrapped by an AI coding agent in VS Code working from these specifications: main now carries the complete scaffold — package structure, CI wiring, and the full test suite — with placeholders instead of implementations. Learning means filling those placeholders against the specifications. The documentation-only state that preceded the scaffold is preserved by the tag [docs-baseline](https://github.com/DeepleMass/quantum-interference/tree/docs-baseline).

## Status and Workflow

The scaffold is in place: package structure, CI, and the full test suite exist; the implementations are placeholders. Work proceeds as specified: one milestone per PR into the learn branch, every CI stage green before merge, one SPEC section at a time as the working prompt. Deviations from the specifications are documented in the corresponding PR and in the package journal indexed by the root `REPORTME.md`; silent deviations are rejected.

## Best Practice for This Repo

The recommended way to learn from this repository: fork it at the [v0.1.0](https://github.com/DeepleMass/quantum-interference/tree/v0.1.0) tag. Your fork opens on main, and main stays there as an untouched reference — a permanent diff target that shows, at any moment, how far you have come. So the first command after forking is explicit: ```git switch learn```. All implementation work happens on the learn branch of your fork; use the three specifications as your blueprint and the milestone acceptance criteria as your checkpoints; each milestone lands as a pull request into your learn branch, one per milestone, every CI stage green before merge. The scaffold deliberately removes everything that is not learning content — packaging, CI wiring, project structure — so you can concentrate on the implementations.

The tests are part of the contract, and the first exercise comes before any implementation: review the provided test suite against the specifications, check whether it covers the acceptance criteria completely and correctly, and fix gaps as your first own commits. This validates the AI-generated tests and trains the translation of physical requirements — norm invariance, unitarity, the p(k) curve — into checkable code.

After that, one rule holds for every milestone: read the failing test, explain why it fails and what the specification demands, predict the failure, then implement. A test you cannot explain is a lesson not yet learned, regardless of what the pipeline says.

Progress is logged in a scalable journal system: the root `REPORTME.md` indexes everything, each package carries its own journal (`packages/qsim/REPORTME.md`, `packages/qhsp/REPORTME.md`, `packages/qae/REPORTME.md`) with pre-created placeholder entries per milestone — one entry per milestone, about ten lines, newest first, deviations from the specification each with its rationale, and one field naming the misconception you had to overcome. Longer analyses go into `docs/progress/` and are indexed in the root file. Filling placeholders instead of inventing structure keeps the barrier low; reviewing your own deviations at the end of a milestone is the most effective reflection step in the whole process. A milestone counts as closed only when you can explain its central mechanism in one sentence without opening the specification — the entry records that sentence, and if it does not come, the milestone is not done.

Building the simulator, the framework, and the estimator yourself — instead of reading finished code — is the point. The milestones are ordered so that each phase only assumes what the previous one has taught; if you can meet the acceptance criteria of Phase 1 without looking at a reference, you have already understood the state vector, and Phases 2 and 3 build on exactly that foundation. Corrections to the specifications and to the scaffold are welcome as PRs against this repository; implementation progress, on the other hand, belongs in your own copy, whichever form it takes. Which route fits is each person's own decision. A self-learner outside a course can fork this repository and work the milestones on a learn branch of the fork. An equally valid route is a clone pushed to your own GitHub or GitLab with this repository as an upstream remote — fully decoupled, and the only route for an instructor adopting this repo for a course, who needs a course repository under their own control anyway. That instructor runs the named-student-branch mode of [COURSE.md](COURSE.md) and, if walking the curriculum personally, keeps implementations in a private copy — a clone pushed to their own remote — so worked solutions stay invisible to students. The scaffold, the specifications, and the milestones read the same either way.

## References

Standard literature, from textbook to paper. Each entry names the phase it accompanies, and the vocabulary is that of the dialogue and `COMPLEXITY.md`.

Textbooks and lecture notes:

- M. A. Nielsen and I. L. Chuang, *Quantum Computation and Quantum Information*, Cambridge University Press, 10th anniversary edition, 2010. The standard reference; its chapters on the quantum Fourier transform, order finding, and the hidden subgroup problem cover exactly the machinery of Phases 1 and 2.
- J. Preskill, *Lecture Notes on Quantum Computation*, Caltech, available online. Physics-first, free, and deeper on fault tolerance than anything in this repository.
- N. D. Mermin, *Quantum Computer Science: An Introduction*, Cambridge University Press, 2007. The shortest rigorous path through the subject, and the strictest about notation.
- A. Kitaev, A. Shen, and M. Vyalyi, *Classical and Quantum Computation*, AMS, 2002. The complexity-theoretic view behind `COMPLEXITY.md`.

Foundational papers:

- P. W. Shor, "Polynomial-Time Algorithms for Prime Factorization and Discrete Logarithms on a Quantum Computer," SIAM Journal on Computing 26(5), 1997. Phases 1 and 2: order finding and the abelian Fourier transform.
- D. R. Simon, "On the Power of Quantum Computation," SIAM Journal on Computing 26(5), 1997. Phase 2: the Simon instance and the oracle separation behind the one provable exponential gap.
- L. K. Grover, "A Fast Quantum Mechanical Algorithm for Database Search," Proceedings of the 28th ACM STOC, 1996. Phase 3: amplitude amplification.
- G. Brassard, P. Høyer, M. Mosca, and A. Tapp, "Quantum Amplitude Estimation," Information and Computation 191, 79–113, 2004; arXiv:quant-ph/0005055. Phase 3: canonical amplitude estimation, the reference method the MLAE variant is validated against.
- C. H. Bennett, E. Bernstein, G. Brassard, and U. Vazirani, "Strengths and Weaknesses of Quantum Computing," SIAM Journal on Computing 26(5), 1997. The √N query lower bound: why Grover is optimal and search stays quadratic.
- Y. Suzuki, S. Endo, J. Chen, and K. Fujii, "Amplitude estimation without phase estimation," Quantum 4, 261, 2020. Phase 3: maximum-likelihood amplitude estimation.
- S. Aaronson and D. Gottesman, "Improved Simulation of Stabilizer Circuits," Physical Review A 70, 052328, 2004. The suggested stabilizer deep-dive, made practical.
- NIST, FIPS 203, 204, and 205, August 2024. ML-KEM, ML-DSA, SLH-DSA — the post-quantum migration named in the dialogue.

One detail worth noticing: Shor, Simon, and the Bennett–Bernstein–Brassard–Vazirani bound all appeared in the same 1997 issue of SIAM Journal on Computing. The field's founding moment is that compact.

## License

MIT — see [LICENSE](LICENSE). Fork it, learn from it, build on it.

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
