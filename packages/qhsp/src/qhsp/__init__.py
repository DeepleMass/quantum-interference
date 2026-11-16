"""qhsp — Phase 2: the hidden subgroup problem as an abstract framework on Qiskit.

Scaffold stub (see ``SPEC-PHASE-2-HSP-FRAMEWORK.md`` section 3). Planned modules:

- ``groups`` — AbelianGroup as a product of cyclic groups,
- ``problem`` — HSPProblem data class with ``verify_invariance()``,
- ``base`` — AbelianHSP abstract procedure (template method),
- ``fourier`` — QFT over AbelianGroup with instance dispatch,
- ``instances`` — order_finding, simon, discrete_log,
- ``backend`` — Target abstraction around Sampler V2, profiles ideal/noisy/remote,
- ``report`` — markdown reports with transpilation statistics.

TODO(Phase-2 M1): package integration into the monorepo with green CI.
"""

__version__ = "0.1.0"
