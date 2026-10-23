"""Self-test for tools/test_all.py decisions made before a suite runs.

1. Without CMake or Ninja (neither on PATH nor bundled with VS 2022), every suite that calls CMake is FAIL with one
   clear reason instead of 15 build scripts dying on "'cmake' is not recognized".
2. Without the vcvars scripts that the build scripts call by full path, every suite that calls vcvars is FAIL, and the
   reason names the missing scripts and their folder.
3. The tool list of each suite agrees with a scan of its runner script and the scripts that it calls (two levels).
   A Python-only slow suite (T1b full) runs without CMake or MSVC. A cl-only suite (L6, L8) runs without CMake.
4. Without out/questdata, or with only part of it, the suites that read it (T6b, T4b, L8) SKIP with the command that
   generates it and the names of the missing files.
5. Off Windows (no MSVC, no vcvars, no build_l1.cmd) the suites that need CMake or MSVC SKIP with that reason instead of FAIL.
Twins (TEST_ALL_MUTANT): no_preflight, no_questdata_gate, questdata_one_file, no_vcvars_check, tools_from_quick, no_platform_skip.
Each must make the Behaviour tests fail.
Run: python -I tools/test_all_selftest.py   (prints ALL OK)
"""
import ast
import io
import os
import re
import subprocess
import sys
import tempfile
import tokenize
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import test_all  # noqa: E402

MUTANTS = ("no_preflight", "no_questdata_gate", "questdata_one_file", "no_vcvars_check", "tools_from_quick", "no_platform_skip")
DATA_SUITES = ("T6b co-op playbooks", "T4b scenarios", "L8 dialogue")
SUITE = {name: (slow, tools) for name, _, slow, tools in test_all.SUITES}
OK_TOOLS = {"cmake": "", "msvc": ""}

# ------------------------------------------------------------------------------------------ runner script scan
CMAKE_CALL = re.compile(r"""(?im)^[ \t]*(?:call[ \t]+)?"?cmake(?:\.exe)?"?[ \t]+-|["']cmake(?:\.exe)?["'][ \t]*,""")
VCVARS_CALL = re.compile(r"(?i)\bvcvars(?:32|64|all)\.bat\b")
SCRIPT_REF = re.compile(r"[\w%~.\\/-]+\.(?:cmd|bat|py)\b")
VAR_PREFIX = re.compile(r"^(?:%~dp0|%\w+%[\\/]?)+")
BUILD_DRIVER = ROOT / "tools" / "build" / "build.py"  # MSVC env + cmake + build lock; every run_tests script goes through it
HARNESS = {HERE / "test_all.py", HERE / "test_all_selftest.py", ROOT / "tools" / "lint" / "check_repo_hygiene.py"}
# they name cmake, vcvars or build.py (in prose, or in the fixtures of their own tests) but build nothing


def code_text(path: Path) -> str:
    """The text of a script without comments and docstrings: a mention in prose is not a call."""
    text = path.read_text(encoding="utf-8", errors="replace")
    if path.suffix.lower() in (".cmd", ".bat"):
        return "\n".join(l for l in text.splitlines() if not re.match(r"(?i)^\s*(?:@?rem\b|::)", l))
    lines = text.splitlines(keepends=True)
    drop = set()
    for node in ast.walk(ast.parse(text)):
        body = getattr(node, "body", None)
        if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)) and body and \
                isinstance(body[0], ast.Expr) and isinstance(body[0].value, ast.Constant) and \
                isinstance(body[0].value.value, str):
            drop.update(range(body[0].lineno - 1, body[0].end_lineno))
    for tok in tokenize.generate_tokens(io.StringIO(text).readline):
        if tok.type == tokenize.COMMENT:
            row, col = tok.start
            lines[row - 1] = lines[row - 1][:col] + "\n"
    return "".join(l for i, l in enumerate(lines) if i not in drop)


def runner_scripts(cmd) -> list:
    """The repo scripts a SUITES command runs: the script itself, or every module of a unittest discover folder."""
    if "discover" in cmd:
        return sorted((ROOT / cmd[cmd.index("-s") + 1]).resolve().glob("*.py"))
    return [(ROOT / a.replace("\\", "/")).resolve() for a in cmd
            if a.endswith((".py", ".cmd", ".bat")) and (ROOT / a.replace("\\", "/")).is_file()]   # SUITES spell some paths with backslashes


def called_scripts(path: Path) -> list:
    """The repo scripts that the code of `path` names (one level). %~dp0 and %VAR%\\ prefixes are removed."""
    out = []
    text = code_text(path)
    for tok in SCRIPT_REF.findall(text):
        tok = VAR_PREFIX.sub("", tok).replace("\\", "/")
        for base in (ROOT, path.parent):
            p = (base / tok).resolve()
            if p.is_file() and ROOT in p.parents:
                out.append(p)
                break
    # ROOT / "tools" / "build" / "build.py" is a path built in pieces, which SCRIPT_REF cannot see: the shared build driver
    if BUILD_DRIVER.is_file() and re.search(r"build\.py", text) and BUILD_DRIVER not in out:
        out.append(BUILD_DRIVER)
    return out


def scanned_tools(cmd) -> set:
    """{'cmake', 'msvc'} subset that the runner scripts of `cmd` (and the scripts they call, two levels) call.
    Two levels, because the build scripts now go through tools/build/build.py (MSVC env, build lock): the runner tests
    name tools\\runner\\build_l1.cmd, which calls build.py, which calls vcvars and cmake."""
    first = runner_scripts(cmd)
    files = set(first)
    level = set(first)
    for _ in range(2):
        nxt = set()
        for f in level - HARNESS:
            nxt.update(called_scripts(f))
        files.update(nxt)
        level = nxt - HARNESS
    text = "\n".join(code_text(f) for f in sorted(files - HARNESS))
    return {t for t, rx in (("cmake", CMAKE_CALL), ("msvc", VCVARS_CALL)) if rx.search(text)}


# ------------------------------------------------------------------------------------------------- behaviour
class Behaviour(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.empty = Path(self.tmp.name)
        patcher = mock.patch.object(test_all, "WINDOWS", True)   # the decisions below are the Windows ones; the platform tests switch it off
        patcher.start()
        self.addCleanup(patcher.stop)

    def tearDown(self):
        self.tmp.cleanup()

    def mkdir_with(self, sub: str, files) -> Path:
        d = self.empty / sub
        for f in files:
            (d / f).parent.mkdir(parents=True, exist_ok=True)
            (d / f).write_bytes(b"")
        d.mkdir(parents=True, exist_ok=True)
        return d

    def vcvars_ok(self) -> Path:
        return self.mkdir_with("vc", test_all.VCVARS)

    def decide(self, name, quick=False, problems=None):
        slow, tools = SUITE[name]
        return test_all.decide(name, slow, tools, quick, OK_TOOLS if problems is None else problems)

    # --- CMake and Ninja
    def test_no_cmake_anywhere_is_reported(self):
        msg = test_all.toolchain(which=lambda t: None, vs_dir=self.empty, vcvars_dir=self.vcvars_ok())["cmake"][1]
        self.assertIn("cmake", msg)
        self.assertIn("ninja", msg)

    def test_vs_bundled_tools_are_enough(self):
        vs = self.mkdir_with("vs", (r"CMake\bin\cmake.exe", r"Ninja\ninja.exe"))
        tc = test_all.toolchain(which=lambda t: None, vs_dir=vs, vcvars_dir=self.vcvars_ok())
        self.assertEqual(tc["cmake"][1], "")
        self.assertIn("cmake.exe", tc["cmake"][0])

    def test_tools_on_path_are_enough(self):
        tc = test_all.toolchain(which=lambda t: "C:/x/" + t + ".exe", vs_dir=self.empty, vcvars_dir=self.vcvars_ok())
        self.assertEqual(tc["cmake"], ("cmake C:/x/cmake.exe, ninja C:/x/ninja.exe", ""))

    # --- vcvars
    def test_missing_vcvars_is_reported_with_its_folder(self):
        tc = test_all.toolchain(which=lambda t: "C:/x/" + t + ".exe", vs_dir=self.empty, vcvars_dir=self.empty)
        msg = tc["msvc"][1]
        for name in test_all.VCVARS:
            self.assertIn(name, msg)
        self.assertIn(str(self.empty), msg)
        self.assertEqual(tc["cmake"][1], "")  # CMake on PATH does not hide a missing vcvars

    def test_one_missing_vcvars_is_reported(self):
        vc = self.mkdir_with("vc", ("vcvars32.bat", "vcvarsall.bat"))
        msg = test_all.toolchain(which=lambda t: "x", vs_dir=self.empty, vcvars_dir=vc)["msvc"][1]
        self.assertIn("vcvars64.bat not found", msg)
        self.assertNotIn("vcvars32.bat", msg)

    def test_present_vcvars_is_reported_as_found(self):
        tc = test_all.toolchain(which=lambda t: "x", vs_dir=self.empty, vcvars_dir=self.vcvars_ok())
        self.assertEqual(tc["msvc"][1], "")
        self.assertIn("vcvars32.bat", tc["msvc"][0])

    def test_preflight_prints_the_vcvars_problem(self):
        bad = {"cmake": ("cmake x, ninja y", ""), "msvc": ("", "vcvars32.bat not found in C:/vc")}
        out = io.StringIO()
        with mock.patch.object(test_all, "toolchain", lambda: bad), redirect_stdout(out):
            problems = test_all.preflight(quick=False)
        self.assertEqual(problems, {"cmake": "", "msvc": "vcvars32.bat not found in C:/vc"})
        self.assertIn("PREFLIGHT FAIL msvc: vcvars32.bat not found in C:/vc", out.getvalue())
        self.assertIn("PREFLIGHT OK cmake: cmake x, ninja y", out.getvalue())

    def test_quick_run_needs_no_toolchain(self):
        out = io.StringIO()
        with redirect_stdout(out):
            self.assertEqual(test_all.preflight(quick=True), {})
        self.assertEqual(out.getvalue(), "")

    # --- tools per suite, separate from --quick
    def test_declared_tools_match_the_runner_scripts(self):
        seen = set()
        for name, cmd, slow, tools in test_all.SUITES:
            declared = {t.replace("-optional", "") for t in test_all.required_tools(slow, tools)}
            scanned = scanned_tools(cmd)
            seen.add(frozenset(scanned))
            self.assertEqual(declared, scanned, f"{name}: declared {sorted(declared)}, the scripts call {sorted(scanned)}")
        # the scan tells the three kinds apart, so a constant answer cannot pass
        self.assertTrue({frozenset(), frozenset({"msvc"}), frozenset({"cmake", "msvc"})} <= seen, seen)

    def test_suites_fail_fast_without_cmake(self):
        reason = test_all.toolchain(which=lambda t: None, vs_dir=self.empty, vcvars_dir=self.vcvars_ok())["cmake"][1]
        problems = {"cmake": reason, "msvc": ""}
        self.with_all_data()
        for name, (slow, tools) in SUITE.items():
            want = ("fail", reason) if "cmake" in tools else ("run", "")
            self.assertEqual(self.decide(name, problems=problems), want, name)

    def test_suites_fail_fast_without_vcvars(self):
        reason = test_all.toolchain(which=lambda t: "x", vs_dir=self.empty, vcvars_dir=self.empty)["msvc"][1]
        problems = {"cmake": "", "msvc": reason}
        self.with_all_data()
        for name, (slow, tools) in SUITE.items():
            want = ("fail", reason) if "msvc" in tools else ("run", "")
            self.assertEqual(self.decide(name, problems=problems), want, name)

    def test_python_and_cl_suites_run_without_cmake(self):
        problems = {"cmake": "cmake not found", "msvc": ""}
        self.with_all_data()
        for name in ("T1b flight analyzer (full + mutation twin)", "L6 nvmp tests", "L6 client patcher",
                     "L6 server hook", "L8 dialogue"):
            self.assertEqual(self.decide(name, problems=problems), ("run", ""), name)
        nothing = {"cmake": "cmake not found", "msvc": "vcvars not found"}
        self.assertEqual(self.decide("T1b flight analyzer (full + mutation twin)", problems=nothing), ("run", ""))
        self.assertEqual(self.decide("L7 snapshot", problems=nothing), ("run", ""))  # uses cl only when vcvars exists
        self.assertEqual(self.decide("L5 hooks", problems=problems), ("fail", "cmake not found"))

    # --- not Windows
    def test_off_windows_toolchain_suites_skip_with_a_reason_instead_of_failing(self):
        self.with_all_data()
        broken = {"cmake": "cmake not found", "msvc": "vcvars32.bat not found"}
        with mock.patch.object(test_all, "WINDOWS", False):
            for name, (slow, tools) in SUITE.items():
                needs = any(t in ("cmake", "msvc") for t in tools) or name in test_all.WINDOWS_ONLY
                action, reason = self.decide(name, problems=broken)
                if needs:
                    self.assertEqual(action, "skip", name)
                    self.assertIn("Windows only", reason)
                else:
                    self.assertEqual(action, "run", name)

    def test_windows_only_suites_exist_and_run_on_windows(self):
        self.with_all_data()
        for name in test_all.WINDOWS_ONLY:
            self.assertIn(name, SUITE, "a WINDOWS_ONLY name that is not a suite would never skip anything")
            self.assertEqual(self.decide(name, problems={"cmake": "", "msvc": ""}), ("run", ""), name)

    def test_a_windows_only_suite_skips_off_windows_and_runs_on_windows(self):
        # The table may be empty; the mechanism is checked with a Python-only suite put into it for the test.
        self.with_all_data()
        name = next(n for n, (slow, tools) in SUITE.items() if not tools and not slow)
        with mock.patch.dict(test_all.WINDOWS_ONLY, {name: "it needs a Windows API"}):
            with mock.patch.object(test_all, "WINDOWS", False):
                action, reason = self.decide(name, problems={})
                self.assertEqual(action, "skip", name)
                self.assertIn("it needs a Windows API, which is Windows only", reason)
            self.assertEqual(self.decide(name, problems={}), ("run", ""), name)

    def test_off_windows_preflight_checks_nothing_and_prints_nothing(self):
        out = io.StringIO()
        with mock.patch.object(test_all, "WINDOWS", False), mock.patch.object(test_all, "toolchain", lambda: self.fail("toolchain probed")), \
                redirect_stdout(out):
            self.assertEqual(test_all.preflight(quick=False), {})
        self.assertEqual(out.getvalue(), "")

    def test_on_windows_toolchain_problems_still_fail(self):
        self.with_all_data()
        action, _ = self.decide("L5 hooks", problems={"cmake": "cmake not found", "msvc": ""})
        self.assertEqual(action, "fail")

    def test_quick_still_skips_slow_suites_silently(self):
        self.assertEqual(self.decide("L4 synccore", quick=True, problems={"cmake": "x", "msvc": "y"}), ("skip", ""))
        self.assertEqual(self.decide("T1b flight analyzer (full + mutation twin)", quick=True), ("skip", ""))

    # --- out/questdata
    def with_all_data(self):
        self.mkdir_with("", [f for files, _ in test_all.NEEDS.values() for f in files])
        patcher = mock.patch.object(test_all, "ROOT", self.empty)
        patcher.start()
        self.addCleanup(patcher.stop)

    def test_data_suites_skip_without_questdata(self):
        with mock.patch.object(test_all, "ROOT", self.empty):
            for name in DATA_SUITES:
                action, reason = self.decide(name)
                self.assertEqual(action, "skip", name)
                self.assertIn("out/questdata", reason)
                self.assertIn("run_all.py", reason)

    def test_data_suites_run_with_questdata(self):
        self.with_all_data()
        for name in DATA_SUITES:
            self.assertEqual(self.decide(name), ("run", ""), name)

    def test_partial_questdata_skips_and_names_the_missing_file(self):
        self.with_all_data()
        for name in DATA_SUITES:
            files = test_all.NEEDS[name][0]
            self.assertTrue(len(files) > 1, name)
            for f in files:
                (self.empty / f).rename(self.empty / (f + ".off"))
                try:
                    action, reason = self.decide(name)
                    self.assertEqual(action, "skip", f"{name} without {f}")
                    self.assertIn(f, reason)
                finally:
                    (self.empty / (f + ".off")).rename(self.empty / f)


def run_twin(mutant: str) -> int:
    env = dict(os.environ, TEST_ALL_MUTANT=mutant)
    return subprocess.run([sys.executable, "-I", __file__, "--behaviour-only"], env=env, capture_output=True,
                          text=True).returncode


def main() -> int:
    if "--behaviour-only" in sys.argv:
        res = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(Behaviour))
        return 0 if res.wasSuccessful() else 1
    res = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Behaviour))
    survivors = [m for m in MUTANTS if run_twin(m) == 0]
    print(f"mutation twins: {len(MUTANTS)} run, {len(survivors)} survived{': ' + ', '.join(survivors) if survivors else ''}")
    ok = res.wasSuccessful() and not survivors
    print("test_all preflight: " + ("ALL OK" if ok else "FAILED"))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
