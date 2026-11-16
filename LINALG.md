# LINALG — A Linear-Algebra Companion

Quantum computing is linear algebra over complex vector spaces with a
probability interpretation. This file collects the machinery the three SPECs
use, so that "why does this gate work" has one place to look. Read it
alongside `COMPLEXITY.md`, which covers the asymptotic vocabulary; this
file covers the objects themselves.

## Contents

1. [The state is a unit vector](#the-state-is-a-unit-vector)
2. [Inner products and interference](#inner-products-and-interference)
3. [Tensor products: where the 2^n comes from](#tensor-products-where-the-2n-comes-from)
4. [Unitaries: what a gate is allowed to be](#unitaries-what-a-gate-is-allowed-to-be)
5. [Projectors and the measurement postulate](#projectors-and-the-measurement-postulate)
6. [The spectral theorem and why phase estimation works](#the-spectral-theorem-and-why-phase-estimation-works)
7. [The QFT is the DFT](#the-qft-is-the-dft)
8. [Statistics on top: sampling and estimation](#statistics-on-top-sampling-and-estimation)
9. [Where the three phases sit](#where-the-three-phases-sit)

## The state is a unit vector

An n-qubit state is a unit vector in C^(2^n): 2^n complex amplitudes whose
squared magnitudes sum to 1. That sentence is the whole memory model of Phase 1
— the simulator is a disciplined array of length 2^n plus rules for updating
it. Indexing convention: qubit 0 is the least significant bit of the basis
index (little-endian, matching Qiskit), while bitstrings display big-endian.
The basis states are orthonormal, and the vector is the complete description
of the system; the formalism has no hidden variables.

## Inner products and interference

The inner product of two states measures overlap: the squared magnitude of
the projection of one onto the other is a probability. Interference is what
happens when overlaps cancel: amplitudes are complex numbers and add with
their phases, so two paths can reinforce or annihilate. This is the entire
engine of quantum speedup — Fourier sampling works because the QFT makes
non-peak amplitudes cancel across the whole superposition at once. Parallelism
is a metaphor; interference is the mechanism.

## Tensor products: where the 2^n comes from

A single qubit lives in C²; n qubits live in the tensor-product space, whose
dimension is the product 2 · 2 · … · 2. A gate on qubit k acts as the matrix
tensored with the identity on the rest, and costs O(2^n) to apply: touching
one qubit means rewriting every amplitude. Entangled states are exactly those
that do not factor into a tensor product of smaller states; Shor's second
register entangles with the first on purpose, which is why `qsim` keeps
one joint state vector (SPEC section 5).

## Unitaries: what a gate is allowed to be

Gates are unitary matrices: U†U = I, which is precisely the statement that
they preserve the norm — total probability stays 1. Composition of gates is
matrix product, and the inverse of a circuit is its adjoint reversed. Not
every useful matrix is a gate: projectors are not unitary, which is why
measurement is not a gate. Any unitary can be approximated to arbitrary
precision by products of H, T, and CNOT — a universal set — and Phase 1's
cross-validation against Qiskit checks that the same unitaries yield the same
distribution.

## Projectors and the measurement postulate

Measurement in the computational basis applies the projectors
P_b = |b⟩⟨b|: outcome b has probability equal to the squared magnitude of the
b-th amplitude, and the state collapses to the outcome, renormalized. For an
observable A, the expectation is ⟨ψ|A|ψ⟩, estimated by repeated preparation
and sampling. Measurement is the only non-unitary step in the stack — in the
simulator it is a seeded draw from the distribution of squared amplitudes,
which is why seed discipline is a SPEC rule rather than a nicety.

## The spectral theorem and why phase estimation works

Every Hermitian matrix has an orthonormal eigenbasis with real eigenvalues;
every unitary has an orthonormal eigenbasis with eigenvalues on the unit
circle, e^{-iθ}. Quantum phase estimation is eigendecomposition as a
measurement: it reads the eigenphase of an eigenvector of U. This is the
load-bearing fact for Shor: the states built from the modular-multiplication
oracle are eigenvectors whose eigenphases encode the order r, and the QFT —
run backwards — is exactly the readout. Phase 1 implements this in code;
Phase 3 rebuilds it as the canonical amplitude-estimation reference.

## The QFT is the DFT

The quantum Fourier transform over 2^n points is the discrete Fourier
transform matrix with a normalization factor: no more, no less. What differs
is the interface. A classical DFT reads and writes 2^n numbers; the quantum
circuit acts on the n qubits in O(n²) gates — but you never get the
transformed vector out, you sample it. That is why "use the QFT to speed up
classical signal processing" is a fallacy (the dialogue makes the same point):
loading classical data and reading results back costs the exponential you
saved.

## Statistics on top: sampling and estimation

The state vector gives a distribution; experiments sample it. Repeated shots
give binomial counts, and estimation theory takes over: likelihood, Fisher
information, Cramér–Rao. Maximum-likelihood amplitude estimation is this
machinery applied to a quantum parameter — Grover powers shape the likelihood,
and the estimator extracts the amplitude. The scaling statements (1/√N
classical, 1/N quantum) live in `COMPLEXITY.md`; this file only notes
where they plug in: the Born rule is the interface between linear algebra and
statistics.

## Where the three phases sit

| Phase | Linear-algebra concept made executable |
|---|---|
| qsim | The vector space itself: amplitudes, tensor-product growth, unitary updates, projector sampling |
| qhsp | Eigenstructure of the oracle; Fourier sampling as structured readout |
| qae | Estimation theory on top of the Born rule: likelihood from amplitudes, Cramér–Rao as the limit |
