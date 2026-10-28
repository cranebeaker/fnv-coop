"""The quick-suite scripts that pass on a machine without the game print what tools/test_all.py takes as proof.

test_all passes a suite only on exit 0 with a line that shows tests ran (EVIDENCE). At ace2b7b two suites exited 0 without
one, so the python quick suite was red on CI and on every machine for a suite that had passed:
1. tools/lint/check_host_readers_wiring.py printed only "ok: ..." (everywhere);
2. tools/lab/esp/run_tests.py without FalloutNV.esm printed only its SKIPPED line (CI, which has no game).
Each script runs here as test_all runs it, the Lab one with FNV_ESM naming a file that does not exist, and test_all.passed()
must accept its output.
Twins (each must make its test fail): HOSTREADERS_MUTANT=bare_ok (the old "ok" line), NVLABESP_RUNNER_MUTANT=no_ran_count
(the SKIPPED line alone).
Run: python -I tools/test_all_evidence.py   (prints ALL OK)
"""
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import test_all  # noqa: E402

TWINS = (("HOSTREADERS_MUTANT", "bare_ok"), ("NVLABESP_RUNNER_MUTANT", "no_ran_count"))


def run(cmd, **env):
    """One script as test_all runs it. A twin's variable, set for this whole file, reaches the script through the environment."""
    p = subprocess.run([sys.executable, "-I"] + cmd, cwd=ROOT, capture_output=True, text=True, errors="replace", env={**os.environ, **env})
    return p.returncode, p.stdout + p.stderr


class QuickSuiteEvidence(unittest.TestCase):
    def test_host_readers_lint(self):
        code, out = run(["tools/lint/check_host_readers_wiring.py"])
        self.assertTrue(test_all.passed(code, out), out)

    def test_lab_generator_without_the_game(self):
        with tempfile.TemporaryDirectory() as d:
            code, out = run(["tools/lab/esp/run_tests.py"], FNV_ESM=str(Path(d) / "FalloutNV.esm"))
        self.assertIn("SKIPPED", out)            # still says what did not run
        self.assertNotIn("ALL OK", out)          # and never claims the full run
        self.assertTrue(test_all.passed(code, out), out)


def main() -> int:
    r = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(QuickSuiteEvidence))
    if not r.wasSuccessful():
        return 1
    if any(os.environ.get(k) for k, _v in TWINS):   # a twin run: the tests alone
        return 0
    survived = []
    for k, v in TWINS:
        p = subprocess.run([sys.executable, "-I", str(Path(__file__).resolve())], cwd=ROOT, capture_output=True, text=True,
                           env={**os.environ, k: v})
        if p.returncode == 0:
            survived.append(f"{k}={v}")
    if survived:
        print("SURVIVED: " + ", ".join(survived))
        return 1
    print(f"{len(TWINS)} of {len(TWINS)} twins caught")
    print("ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
