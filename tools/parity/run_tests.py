"""One command for the Python half of the parity tooling:  python -I tools/parity/run_tests.py   -> prints ALL OK

1. the unit suite (tests/test_parity.py, including tools/parity/vectors.json being current) must pass on the real code;
2. every mutation twin (PARITY_MUTANT=<name>, one deliberate bug each in compare.py) must make that suite FAIL;
3. the bench hook (tools/bench/coop_bench.py parity scoring) is checked by tools/parity/tests/test_bench_hook.py in the same way.
The C++ half has its own command: plugin\parity\run_tests.cmd (it calls this script last and also ends in ALL OK).
"""
from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent

MUTANTS = ("walk_over_run", "ignore_speed", "keep_pending", "no_persistence", "ignore_face_hash", "absent_ignores_coverage", "no_net_cap", "link_ignored")
SUITES = ("test_parity.py", "test_bench_hook.py")


def run(suite: str, mutant: str = "") -> subprocess.CompletedProcess:
    env = dict(os.environ)
    env.pop("PARITY_MUTANT", None)
    if mutant:
        env["PARITY_MUTANT"] = mutant
    return subprocess.run([sys.executable, "-I", str(HERE / "tests" / suite)], cwd=ROOT, capture_output=True, text=True, env=env, errors="replace")


def main() -> int:
    t0 = time.time()
    bad = 0
    for suite in SUITES:
        r = run(suite)
        tail = (r.stdout + r.stderr).strip().splitlines()
        print("real suite %s: %s" % (suite, tail[-1] if tail else "(no output)"))
        if r.returncode != 0:
            print("\n".join(tail[-30:]))
            bad += 1
    survived = [m for m in MUTANTS if all(run(s, m).returncode == 0 for s in SUITES)]
    if survived:
        print("SURVIVED:", ", ".join(survived))
        bad += 1
    if bad:
        print("FAILED")
        return 1
    print("parity tooling: real tests pass, all %d twins caught (%.1fs)" % (len(MUTANTS), time.time() - t0))
    print("ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
