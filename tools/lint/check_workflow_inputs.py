#!/usr/bin/env python3
"""Workflow inputs stay out of shell source, and the sanitizer soak refuses a manual seed or count that is not a positive integer.

    python -I tools/lint/check_workflow_inputs.py              # the checks on .github/workflows/*.yml and *.yaml (prints N checks, 0 failed)
    python -I tools/lint/check_workflow_inputs.py --self-test  # the checker's own tests and twins (prints ALL OK)

1. No `run:` script in any workflow may expand a manual input (`${{ github.event.inputs.X }}`, `${{ inputs.X }}`) or event text
   someone else controls (pull request title or body, branch name, comment, ...) directly: GitHub pastes the text into the script
   before the shell runs, so `$(command)` in it runs on the runner. Pass such a value through the step's `env:` instead.
2. The "Seed base" step of sanitizers.yml, run with bash: the default and valid manual values pass; a count or seed that is 0,
   negative, not a number or a command substitution fails the step and never runs. A count of 0 would otherwise run no session
   and pass. Skipped with a reason where bash is not installed.
3. Every `grep` in a `run:` script that reads a log written by `ctest -V` passes `-a`: some test output holds NUL bytes, and
   GNU grep then prints only "binary file matches" instead of the lines, so a failed run would lose its summary and details.
The workflow files are read as text (no YAML library: the quick suite needs only the standard library).
"""
from __future__ import annotations

import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import List, Optional, Tuple

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"
SANITIZERS = WORKFLOWS / "sanitizers.yml"

# Expressions whose text comes from whoever triggers the run, not from the repository.
UNTRUSTED = re.compile(
    r"\$\{\{[^}]*\b(github\.event\.inputs\.|inputs\.|github\.head_ref|github\.event\.(pull_request|issue|comment|review|review_comment|"
    r"head_commit|commits|discussion|pages)\b)[^}]*\}\}")
RUN_KEY = re.compile(r"^(\s*(?:-\s+)?)run:\s*(.*)$")   # group 1 ends at the column of `run`: a block scalar is indented past it


def run_scripts(text: str) -> List[Tuple[int, str]]:
    """(line number, script line) of every line inside a `run:` value, inline or block scalar (| or >)."""
    lines = text.splitlines()
    out: List[Tuple[int, str]] = []
    i = 0
    while i < len(lines):
        m = RUN_KEY.match(lines[i])
        if not m:
            i += 1
            continue
        key_indent, rest = len(m.group(1)), m.group(2)
        if rest and rest[0] not in "|>":
            out.append((i + 1, rest))
            i += 1
            continue
        i += 1
        while i < len(lines) and (not lines[i].strip() or len(lines[i]) - len(lines[i].lstrip()) > key_indent):
            out.append((i + 1, lines[i]))
            i += 1
    return out


def untrusted_in_run(text: str, name: str) -> List[str]:
    return ["%s:%d: %s is pasted into a run script; pass it through the step's env: instead" % (name, n, UNTRUSTED.search(s).group(0))
            for n, s in run_scripts(text) if UNTRUSTED.search(s)]


CTEST_V_LOG = re.compile(r"\bctest\b[^>|;]*\s-V\b[^>|;]*>\s*([^\s;|&]+)")
GREP_OPTIONS = re.compile(r"\bgrep((?:\s+-[^\s]+)*)")


def grep_without_text(text: str, name: str) -> List[str]:
    """A grep that reads a `ctest -V` log without -a (or --text)."""
    # Quoted text goes first: a pattern's | or ; is not a pipe or a separator. Comments are not commands.
    lines = [(n, re.sub(r'"(?:[^"\\]|\\.)*"|\'[^\']*\'', '""', s)) for n, s in run_scripts(text) if not s.lstrip().startswith("#")]
    logs = {m.group(1) for _, s in lines for m in CTEST_V_LOG.finditer(s)}
    bad = []
    for n, s in lines:
        for m in GREP_OPTIONS.finditer(s):
            rest = s[m.end():].split(";")[0].split("|")[0]
            log = next((l for l in logs if re.search(r"(^|\s)%s(\s|$)" % re.escape(l), rest)), None)
            opts = m.group(1).split()
            text_opt = any(o in ("--text", "--binary-files=text") or (not o.startswith("--") and "a" in o[1:]) for o in opts)
            if log and not text_opt:
                bad.append("%s:%d: grep reads the ctest -V log %s without -a: a NUL byte in it hides the lines" % (name, n, log))
    return bad


def step_script(text: str, step_name: str) -> Optional[str]:
    """The `run: |` script of the step called `step_name`, dedented, or None."""
    lines = text.splitlines()
    for i, l in enumerate(lines):
        if re.match(r"^\s*-?\s*name:\s*%s\s*$" % re.escape(step_name), l):
            step_indent = len(l) - len(l.lstrip(" -"))
            for j in range(i + 1, len(lines)):
                lj = lines[j]
                if lj.strip() and len(lj) - len(lj.lstrip()) < step_indent:
                    return None
                m = RUN_KEY.match(lj)
                if m and m.group(2).startswith("|"):
                    body, k = [], j + 1
                    while k < len(lines) and (not lines[k].strip() or len(lines[k]) - len(lines[k].lstrip()) > len(m.group(1))):
                        body.append(lines[k])
                        k += 1
                    pad = min(len(b) - len(b.lstrip()) for b in body if b.strip())
                    return "\n".join(b[pad:] for b in body) + "\n"
    return None


def run_seed_step(script: str, bash: str, seed: str = "", count: str = "", event: str = "pull_request"):
    """(exit code, what the step wrote to GITHUB_ENV, whether a planted command ran)."""
    with tempfile.TemporaryDirectory() as d:
        envf, marker = Path(d) / "env", Path(d) / "ran"
        envf.write_text("")
        env = dict(os.environ, IN_SEED_BASE=seed.replace("MARK", marker.as_posix()), IN_COUNT=count, EVENT=event, GITHUB_ENV=envf.as_posix())
        p = subprocess.run([bash, "-c", script], env=env, capture_output=True, text=True)
        got = dict(l.split("=", 1) for l in envf.read_text().splitlines() if "=" in l)
        return p.returncode, got, marker.exists()


def seed_step_problems(script: Optional[str], bash: str) -> List[str]:
    if script is None:
        return ["sanitizers.yml has no 'Seed base' step with a run: | script"]
    bad: List[str] = []
    rc, got, _ = run_seed_step(script, bash)
    if rc != 0 or got != {"SEED_BASE": "1", "COUNT": "300"}:
        bad.append("default run: want SEED_BASE=1 COUNT=300, got exit %d %s" % (rc, got))
    rc, got, _ = run_seed_step(script, bash, seed="5000", count="12")
    if rc != 0 or got != {"SEED_BASE": "5000", "COUNT": "12"}:
        bad.append("manual 5000/12: got exit %d %s" % (rc, got))
    rc, got, _ = run_seed_step(script, bash, event="schedule")
    if rc != 0 or not got.get("SEED_BASE", "").isdigit() or int(got["SEED_BASE"]) <= 1:
        bad.append("schedule: want a weekly seed block above 1, got exit %d %s" % (rc, got))
    for c in ("0", "-3", "abc", "1e3", "12 13", "214748365", "999999999"):   # the last two: --count is an int and mini runs COUNT * 10
        rc, got, _ = run_seed_step(script, bash, count=c)
        if rc == 0 or "COUNT" in got:
            bad.append("count %r was accepted (exit %d, %s): the soak would run no session or fail later" % (c, rc, got))
    rc, got, _ = run_seed_step(script, bash, count="99999999")
    if rc != 0 or got.get("COUNT") != "99999999":
        bad.append("the largest allowed count (8 digits) was refused: exit %d %s" % (rc, got))
    rc, got, ran = run_seed_step(script, bash, seed="1$(touch MARK)")
    if ran or rc == 0 or "SEED_BASE" in got:
        bad.append("a seed holding a command substitution was %s" % ("EXECUTED" if ran else "accepted"))
    return bad


def find_bash() -> Optional[str]:
    return shutil.which("bash")


def workflow_files(d: Path = WORKFLOWS) -> List[Path]:
    """Every workflow GitHub loads: both extensions."""
    return sorted(p for p in d.iterdir() if p.is_file() and p.suffix in (".yml", ".yaml")) if d.is_dir() else []


def checks() -> Tuple[int, List[str], List[str]]:
    """(number of checks, problems, skip notes)."""
    n, bad, skips = 0, [], []
    for wf in workflow_files():
        n += 1
        bad += untrusted_in_run(wf.read_text(encoding="utf-8"), wf.name)
        bad += grep_without_text(wf.read_text(encoding="utf-8"), wf.name)
    if SANITIZERS.is_file():
        bash = find_bash()
        if bash:
            n += 1
            bad += seed_step_problems(step_script(SANITIZERS.read_text(encoding="utf-8"), "Seed base"), bash)
        else:
            skips.append("the sanitizers.yml 'Seed base' step check needs bash, which is not installed here")
    return n, bad, skips


# ------------------------------------------------------------------------------------------------ self-test
OLD_SEED_STEP = '''      - name: Seed base
        run: |
          if [ -n "${{ github.event.inputs.seed_base }}" ]; then base="${{ github.event.inputs.seed_base }}"
          else base=1; fi
          echo "SEED_BASE=$base" >> "$GITHUB_ENV"
          echo "COUNT=${{ github.event.inputs.count || 300 }}" >> "$GITHUB_ENV"
'''
SAFE_STEP = '''      - name: Safe
        env:
          IN_COUNT: ${{ github.event.inputs.count }}
          TITLE: ${{ github.event.pull_request.title }}
        run: echo "$IN_COUNT $TITLE"
      - run: >
          echo building with ${{ matrix.cxx }}
'''
CTEST_STEP = '''      - run: |
          reports() { grep -aE "runtime error" "$1"; }
          ctest --test-dir build -V -j 4 > ctest-all.log 2>&1 || rc=$?
          grep -E "tests passed|tests failed" ctest-all.log | tail -40
          if reports ctest-all.log > hits.txt; then grep -c x hits.txt; fi
          [ "$rc" -eq 0 ] || { grep -B2 -A40 -E "Failed" ctest-all.log | tail -200; }
'''
UNVALIDATED = '''base="${IN_SEED_BASE:-1}"
echo "SEED_BASE=$base" >> "$GITHUB_ENV"
echo "COUNT=${IN_COUNT:-300}" >> "$GITHUB_ENV"
'''


def self_test() -> int:
    fails = 0

    def expect(name, ok):
        nonlocal fails
        print(("ok    " if ok else "FAIL  ") + name)
        fails += 0 if ok else 1

    expect("an input pasted into a run block is found on both lines (twin: the old Seed base step)",
           len(untrusted_in_run(OLD_SEED_STEP, "old.yml")) == 2)
    expect("a pull request title in an inline run is found",
           untrusted_in_run("    - run: echo ${{ github.event.pull_request.title }}\n", "t.yml") != [])
    expect("inputs passed through env and matrix values are fine", untrusted_in_run(SAFE_STEP, "safe.yml") == [])
    expect("the block ends where the indentation does",
           untrusted_in_run("      - run: |\n          echo ok\n        env:\n          X: ${{ inputs.x }}\n", "e.yml") == [])
    twin = grep_without_text(CTEST_STEP, "c.yml")
    expect("twin: the two diagnostic greps of #59's first ctest step, without -a, are found (lines %s)"
           % [b.split(":")[1] for b in twin], [b.split(":")[1] for b in twin] == ["4", "6"])
    expect("grep -a, -aE, -a -B2, --text on a ctest -V log, and grep on other files, are fine",
           grep_without_text(CTEST_STEP.replace("grep -E", "grep -aE").replace("grep -B2", "grep -a -B2"), "c.yml") == []
           and grep_without_text("  - run: |\n      ctest -V > l.log\n      grep --text x l.log\n      grep x other.log\n", "d.yml") == [])
    grep = shutil.which("grep")
    if grep:
        with tempfile.TemporaryDirectory() as d:
            log = Path(d) / "ctest-all.log"
            log.write_bytes(b"1/2 Test #1: a ....\x00 Passed\n100% tests passed, 0 tests failed out of 2\n")
            plain = subprocess.run([grep, "-E", "tests passed", log.as_posix()], capture_output=True, text=True).stdout
            text = subprocess.run([grep, "-aE", "tests passed", log.as_posix()], capture_output=True, text=True).stdout
        expect("why: on a log with a NUL byte, grep without -a hides the summary line and grep -a shows it",
               "tests passed" not in plain and "tests passed" in text)
    else:
        print("skip  the grep behaviour check needs grep")
    real = step_script(SANITIZERS.read_text(encoding="utf-8"), "Seed base") if SANITIZERS.is_file() else None
    with tempfile.TemporaryDirectory() as d:
        for name in ("a.yml", "b.yaml", "notes.txt"):
            (Path(d) / name).write_text("x\n")
        found = [p.name for p in workflow_files(Path(d))]
        expect("both workflow extensions are scanned, nothing else (twin: a *.yml glob misses b.yaml)",
               found == ["a.yml", "b.yaml"] and [p.name for p in sorted(Path(d).glob("*.yml"))] != found)
    expect("the Seed base script is found in sanitizers.yml", real is not None and "positive" in real)
    bash = find_bash()
    if bash and real:
        expect("the real Seed base step passes its checks", seed_step_problems(real, bash) == [])
        twin = seed_step_problems(UNVALIDATED, bash)
        expect("twin: a Seed base step without the number check is caught (%d problems)" % len(twin), len(twin) >= 6)
    else:
        print("skip  the Seed base behaviour checks need bash")
    print("ALL OK" if not fails else "%d failed" % fails)
    return 1 if fails else 0


def main(argv: List[str]) -> int:
    if "--self-test" in argv:
        return self_test()
    n, bad, skips = checks()
    for s in skips:
        print("SKIP " + s)
    for b in bad:
        print(b)
    print("workflow inputs: %d checks, %d failed" % (n, len(bad)))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
