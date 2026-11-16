"""Main experiment script for Phase 2 (HSP framework) — scaffold stub.

SPEC-PHASE-2 section 6: runs instance x target x parameter combinations and writes
one markdown report per combination. Call pattern: --instance order|simon|dlog,
--params, --target ideal|noisy|remote, --device, --shots, --seed, --runs.

TODO(Phase-2 M1): implement once packages/qhsp exists.
"""

from __future__ import annotations

import argparse
import sys


def main(argv: list[str] | None = None) -> int:
    """CLI entry point (stub). Returns the process exit code."""
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--instance", choices=["order", "simon", "dlog"], required=True)
    parser.add_argument("--target", choices=["ideal", "noisy", "remote"], default="ideal")
    parser.add_argument("--shots", type=int, default=1024)
    parser.add_argument("--seed", type=int, default=42)
    args = parser.parse_args(argv)
    print(f"qhsp scaffold: instance={args.instance} target={args.target} (TODO Phase-2 M1)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
