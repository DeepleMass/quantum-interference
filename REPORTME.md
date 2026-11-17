# REPORTME — quantum-interference

This file is the index of all learning reports in the repository. The learning
journals live in the packages, longer deep-dive analyses live in `docs/progress/`,
and both are indexed here. Static project reference (build, conventions, CI) is kept at
the bottom of this file. The complexity-theoretic vocabulary shared by the SPECs, the
dialogue in `README.md`, and these reports is collected in [COMPLEXITY.md](COMPLEXITY.md); the
linear-algebra machinery underneath is collected in [LINALG.md](LINALG.md).

## Contents

1. [How progress is logged](#how-progress-is-logged)
2. [Milestone status](#milestone-status)
3. [Deep-dive report index](#deep-dive-report-index)
4. [Scaffold status](#scaffold-status)
5. [Build](#build)
6. [Architecture rationale](#architecture-rationale)
7. [Naming and layout conventions](#naming-and-layout-conventions)
8. [Reports](#reports)
9. [CI](#ci)
10. [Release convention](#release-convention)

## How progress is logged

- One journal per package: `packages/qsim/REPORTME.md`, `packages/qhsp/REPORTME.md`, `packages/qae/REPORTME.md`.
- One entry per milestone, written on completion — not per work session.
- Fixed fields: milestone, date, status, misconception overcome, deviations from the SPEC (with rationale), open questions.
- The misconception field records what you initially had wrong and how you noticed; the error you worked through is the part of the entry worth rereading later.
- Hard limit: about ten lines per entry. Anything longer becomes a deep-dive report in `docs/progress/` and is indexed below.
- Newest entry at the top of each journal.
- A milestone counts as closed only when its entry carries a one-line self-explanation of the central mechanism, written without opening the SPEC. If that sentence does not come, the milestone is not done — whatever the pipeline says.

Every journal ships with pre-created placeholder entries, so logging means filling
fields, not inventing structure.

## Milestone status

| Phase | M1 | M2 | M3 | M4 | M5 | M6 |
|---|---|---|---|---|---|---|
| [qsim](packages/qsim/REPORTME.md) | done (scaffold) | open | open | open | open | open |
| [qhsp](packages/qhsp/REPORTME.md) | open | open | open | open | open | open |
| [qae](packages/qae/REPORTME.md) | open | open | open | open | open | open |

Set a cell to `done <date>` once the milestone acceptance criteria hold; if the
journal entry carries deviations, link the entry from the cell.

## Deep-dive report index

Longer analyses live in `docs/progress/` and are indexed here. Use
[TEMPLATE.md](docs/progress/TEMPLATE.md) to start one. Rows marked [suggested]
are recommended optional deep-dives: start them from the template when the moment
comes, and replace the row with a link once the report exists.

| Report | Phase | Milestone | Date |
|---|---|---|---|
| [suggested] Stabilizer formalism and Gottesman–Knill — where classical simulation stays efficient, and why Shor escapes it | qsim | after M3 | — |
| [suggested] Heisenberg scaling and the Cramér–Rao bound — quantum vs classical estimation as one information-theoretic picture | qae | after M4 | — |
| [suggested] State vectors on the metal — memory layout, cache behavior, and where a GPU would and would not help | qsim | after M6 | — |

## Scaffold status

**Scaffold, Phase 1 ready.** All modules of `qsim-core` and `packages/qsim` exist with
their final API signatures, documentation, and placeholder bodies (`TODO(Mx)` markers,
default return values). The packages `qhsp` and `qae` exist as stubs. CI is green:
Rust smoke tests (unit + integration) pass, Python smoke tests pass. The detailed test
cases from SPEC sections 7.1–7.4 are recorded as test plans (`TODO` comments in the test
files) and are implemented milestone by milestone — by design, the scaffold ships green
with smoke tests only.

## Build

Rust core and bindings:

```bash
cargo test --workspace --release   # unit + integration smoke tests
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

Python package (uv workspace, maturin build backend):

```bash
uv venv && source .venv/bin/activate
uv pip install maturin pytest numpy "qiskit>=2.1,<2.2"   # qiskit only for slow tests
cd packages/qsim && maturin develop --release && cd ../..
pytest packages/qsim/tests -m "not slow" -v               # add -m slow for the Qiskit cross-check
ruff check . && ruff format --check .
```

Demo experiment (writes one markdown report to `docs/reports/`):

```bash
python scripts/run_period_finding.py --n 15 --a 7 --shots 1024 --seed 42
```

## Architecture rationale

- **Why a state vector:** an n-qubit state is a vector of 2^n complex amplitudes; the
  2^n growth is course material, not an optimization target (anything beyond ~24 qubits
  is out of scope, SPEC section 1).
- **Why a Rust core:** the simulator is the object of study — gates, QFT, and measurement
  should be readable and owned in one place, with no runtime dependencies beyond
  num-complex, thiserror, and rand/rand_chacha. Python orchestrates experiments and
  classical post-processing.
- **Why Qiskit conventions:** little-endian qubit indexing (qubit 0 = least significant
  bit of the basis index) with big-endian bitstring display, so Phase 2 (Qiskit
  migration) and the Qiskit cross-validation work without semantic breaks.
- **Why one joint state vector** for input + output register (not two): entanglement
  between the registers is exactly what Shor exploits — course material (SPEC section 5).
- **Ownership decision:** `amplitudes()` is exposed to Python as a read-only NumPy array
  (the WRITEABLE flag is cleared); no Python-side state mutation, ownership stays in the
  Rust core (SPEC section 11). The single `unsafe` block in the binding crate exists
  only because rust-numpy 0.22 has no safe setter for that flag.
- **Seed discipline:** every stochastic path takes its seed from the `Simulator` handle,
  never from a global RNG (SPEC section 11) — runs are reproducible via the documented
  seed configuration.

## Naming and layout conventions

- Rust: edition 2021, no panics in the public API — everything through
  `Result<T, QSimError>`; unitarity and norm checks are test-time assertions, not runtime
  checks.
- Python: >= 3.10, fully type-hinted, strict ruff (google docstring convention),
  pytest with a `slow` marker for the Qiskit cross-validation.
- Layout: `crates/qsim-core` (Rust core), `packages/qsim` (pyo3 bindings + Python
  orchestration, maturin mixed layout with `python-source = "src"`), `packages/qhsp` and
  `packages/qae` (Phase 2/3 stubs), `scripts/` (one experiment script per phase),
  `docs/reports/` (generated experiment reports, gitignored except `.gitkeep`),
  `docs/progress/` (committed deep-dive reports, indexed above).
- Placeholder convention in this scaffold: quantum semantics are `TODO(Mx)` functions
  returning default values; trivial accessors and formatting helpers are provided.
- Reading pointers: the milestone tables in the SPECs carry a Reading column. N&C citations refer to Nielsen & Chuang, *Quantum Computation and Quantum Information* (10th anniversary edition), cited by section and print page; papers are cited as in the README references section.

## Reports

`scripts/run_period_finding.py` writes one markdown report per experiment to
`docs/reports/<N>_<a>_<timestamp>.md` with configuration (qubits, Q, seed, shots), the
measurement histogram, reconstructed r candidates, success rate, and an interpretation.
Reports are committed when generated by hand; large raw histograms go to CI workflow
artifacts instead. CI summarizes the acceptance criteria of SPEC 7.2 in the job summary
once M5 is implemented.

## CI

`.github/workflows/ci.yml`: `rust-checks` (fmt, clippy -D warnings, cargo test
--release) → `python-checks` (ruff, maturin develop, pytest without the slow marker,
Qiskit pinned to a fixed minor version; `needs: rust-checks`) → `build` (maturin sdist +
wheel for linux-x86_64, uploaded as artifact). Triggers: pushes and PRs to main.

![CI](https://github.com/DeepleMass/quantum-interference/actions/workflows/ci.yml/badge.svg)

## Release convention

Every milestone completed and merged to main gets a tagged release. Tag names follow the
phase-milestone scheme: `qsim-m2`, `qsim-m3`, ..., `qhsp-m1`, ..., `qae-m6`. Semantic-version tags are
reserved for project-level states: `v0.1.0` is the scaffold, `docs-baseline` preserves the
documentation-only beginning, and phase completions raise the minor version.

Each tag gets a GitHub release whose notes state three things: what the milestone
implements per the SPEC, which acceptance criteria were met, and what remains for the
next milestone. Notes are written after the journal entry, never before.

Tags exist for the forks, not for upstream vanity: every fork tracks its own progress
against the upstream tags, so `qsim-m2` tells a learner exactly which upstream state their
implementation corresponds to. When in doubt, tag from main only — learn carries work in
progress, and a tag must always point at a state where CI is green.
