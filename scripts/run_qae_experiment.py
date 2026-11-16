"""Main experiment script for Phase 3 (Grover / MLAE) — scaffold stub.

SPEC-PHASE-3 section 5: three modes — --mode search (Grover demo with the p(k) curve),
--mode compare (classical vs QPE-AE vs MLAE with budget alignment), --mode scaling
(error vs budget, regression of error ~ budget^-alpha). Reports as markdown.

TODO(Phase-3 M1): implement once packages/qae exists.
"""

from __future__ import annotations

import argparse
import sys


def main(argv: list[str] | None = None) -> int:
    """CLI entry point (stub). Returns the process exit code."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--mode", choices=["search", "compare", "scaling"], required=True)
    parser.add_argument("--shots", type=int, default=1024)
    parser.add_argument("--seed", type=int, default=42)
    args = parser.parse_args(argv)
    print(f"qae scaffold: mode={args.mode} (TODO Phase-3 M1)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
