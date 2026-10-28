#!/usr/bin/env python3
"""CI's plugin build artifact holds everything provision installs from out/L1, and docs/tools/proton-deck.md downloads it by its name.

A tester without Visual Studio (Linux, Proton on a Steam Deck) takes nvcoop.dll from CI (.github/workflows/ci.yml, job
`plugin`, artifact `nvcoop-plugin-win32`). tools/runner/instances.py provision installs the DLL together with the nvcoop_*.json
and nvcoop_*.txt files next to it; without nvcoop_guest_local_run.json a guest blocks every DLC setup script. So:
1. The plugin job uploads the artifact, after the step that checks nvcoop.dll and with no `if:` (a DLL that failed the check is
   never uploaded), with `if-no-files-found: error`.
2. Its `path:` patterns cover out/L1/nvcoop.dll and every file the CMake build writes to ${NVCOOP_OUT_DIR} that provision
   installs (nvcoop_*.json, nvcoop_*.txt). A data file added later must not silently go missing from the download.
   A later `!pattern` excludes, as in upload-artifact.
3. The artifact name is the one docs/tools/proton-deck.md downloads (`gh run download ... -n <name>`), and the doc only downloads
   after it found the commit's run: with an empty run ID `gh run download` takes the newest artifact of any commit.

    python -I tools/lint/check_plugin_artifact.py              # check the repo (prints N checks, 0 failed)
    python -I tools/lint/check_plugin_artifact.py --self-test  # the checker's own tests and twins (prints ALL OK)

The workflow is read as text (no YAML library: the quick suite needs only the standard library).
"""
from __future__ import annotations

import fnmatch
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple

ROOT = Path(__file__).resolve().parents[2]
CI = ROOT / ".github" / "workflows" / "ci.yml"
DOC = ROOT / "docs" / "tools" / "proton-deck.md"
PLUGIN = ROOT / "plugin"

OUT_FILE = re.compile(r"\$\{NVCOOP_OUT_DIR\}/([A-Za-z0-9_.\-]+)")
DOC_DOWNLOAD = re.compile(r"gh run download\b[^\n]*?\s-n\s+(\S+)")
# The download only runs after the run ID was found: `gh run download` with an empty ID takes the newest artifact of that name
# from any run, so a checkout CI never built would get another commit's DLL (found running the doc's command, 2026-10-26).
DOC_RUN_GUARD = re.compile(r"\[\s+-n\s+\"\$(\w+)\"\s+\]\s+&&\s+gh run download\b[^\n]*\"\$\1\"")


def uploaded(out: str, patterns: List[str]) -> bool:
    """Whether actions/upload-artifact uploads `out`: the patterns in order, the last one that matches wins, `!` excludes."""
    keep = False
    for p in patterns:
        neg = p.startswith("!")
        if fnmatch.fnmatchcase(out, p[1:] if neg else p):
            keep = not neg
    return keep


def provision_installs(name: str) -> bool:
    """The rule of instances.provision(): next to a plugin DLL it installs nvcoop_* files that end in .json or .txt."""
    return name.lower().startswith("nvcoop_") and Path(name).suffix.lower() in (".json", ".txt")


def build_outputs(cmake_texts: Dict[str, str]) -> List[str]:
    """out/L1/<name> of every file a CMakeLists.txt writes to ${NVCOOP_OUT_DIR} that provision installs, plus the DLL."""
    names = {m.group(1) for text in cmake_texts.values() for m in OUT_FILE.finditer(text)}
    return ["out/L1/nvcoop.dll"] + sorted(f"out/L1/{n}" for n in names if provision_installs(n))


def job_block(ci: str, job: str) -> Optional[List[str]]:
    """The lines of one job under `jobs:` (two-space key), or None."""
    lines = ci.splitlines()
    try:
        start = lines.index(f"  {job}:")
    except ValueError:
        return None
    end = next((i for i in range(start + 1, len(lines)) if re.match(r"^  \S", lines[i]) or re.match(r"^\S", lines[i])), len(lines))
    return lines[start:end]


def steps(job: List[str]) -> List[List[str]]:
    """The job's steps, each as its lines (a step starts at `      - `)."""
    out: List[List[str]] = []
    for ln in job:
        if re.match(r"^      - ", ln):
            out.append([ln])
        elif out and (not ln.strip() or ln.startswith("        ")):
            out[-1].append(ln)
    return out


def step_value(step: List[str], key: str) -> Optional[str]:
    for ln in step:
        m = re.match(rf"^\s*(?:-\s+)?{re.escape(key)}:\s*(.*)$", ln)
        if m:
            return m.group(1).strip()
    return None


def path_patterns(step: List[str]) -> List[str]:
    """The `path:` of an upload step: one inline value or a block scalar (|), one pattern per line."""
    for i, ln in enumerate(step):
        m = re.match(r"^(\s*)path:\s*(.*)$", ln)
        if not m:
            continue
        indent, rest = len(m.group(1)), m.group(2).strip()
        if rest and rest[0] not in "|>":
            return [rest]
        return [s.strip() for s in step[i + 1:] if s.strip() and len(s) - len(s.lstrip()) > indent]
    return []


def problems_in(ci: str, cmake_texts: Dict[str, str], doc: str) -> Tuple[List[str], int]:
    """(problems, number of checks) for the given workflow, CMake files and doc."""
    bad: List[str] = []
    checks = 0
    job = job_block(ci, "plugin")
    checks += 1
    if job is None:
        return ["ci.yml has no `plugin` job"], checks
    st = steps(job)
    uploads = [i for i, s in enumerate(st) if (step_value(s, "uses") or "").startswith("actions/upload-artifact@")]
    dll_checks = [i for i, s in enumerate(st) if (step_value(s, "name") or "").startswith("Check nvcoop.dll")]
    checks += 1
    if len(uploads) != 1:
        return bad + [f"the plugin job has {len(uploads)} actions/upload-artifact steps, want 1"], checks
    up = st[uploads[0]]
    checks += 1
    if not dll_checks or uploads[0] < dll_checks[0]:
        bad.append("the artifact upload must come after the 'Check nvcoop.dll' step, so a DLL that failed it is never uploaded")
    checks += 1
    if step_value(up, "if") is not None:
        bad.append(f"the artifact upload has `if: {step_value(up, 'if')}`; it must run only when every check before it passed")
    checks += 1
    if step_value(up, "if-no-files-found") != "error":
        bad.append("the artifact upload needs `if-no-files-found: error`")
    patterns = path_patterns(up)
    outputs = build_outputs(cmake_texts)
    checks += 1
    if len(outputs) < 2:
        bad.append("found no nvcoop_* data file that CMake writes to ${NVCOOP_OUT_DIR} (nvcoop_guest_local_run.json): "
                   "the checker no longer sees the build's outputs")
    for out in outputs:
        checks += 1
        if not uploaded(out, patterns):
            bad.append(f"the artifact's path ({patterns}) does not cover {out}, which provision installs with nvcoop.dll")
    name = step_value(up, "name")
    doc_lines = [ln for ln in doc.splitlines() if DOC_DOWNLOAD.search(ln)]
    doc_names = [DOC_DOWNLOAD.search(ln).group(1) for ln in doc_lines]
    checks += 1
    if not doc_names:
        bad.append("docs/tools/proton-deck.md has no `gh run download ... -n <name>` command for the plugin artifact")
    elif any(n.strip("'\"") != name for n in doc_names):
        bad.append(f"docs/tools/proton-deck.md downloads {doc_names}, the plugin job uploads {name!r}")
    checks += 1
    unguarded = [ln.strip() for ln in doc_lines if not DOC_RUN_GUARD.search(ln)]
    if unguarded:
        bad.append("docs/tools/proton-deck.md runs `gh run download` without checking the run ID first (`[ -n \"$RUN\" ] && ...`): "
                   f"with no run for the commit, gh downloads the newest artifact of any commit: {unguarded[0][:80]}")
    return bad, checks


def repo_cmake() -> Dict[str, str]:
    return {str(p.relative_to(ROOT)): p.read_text(encoding="utf-8", errors="replace") for p in sorted(PLUGIN.rglob("CMakeLists.txt"))}


GOOD_CI = """\
jobs:
  quick:
    steps:
      - run: echo quick
  plugin:
    steps:
      - run: cmake --build build/L1 --config Release --parallel
      - name: Check nvcoop.dll (and that the check rejects a 64-bit DLL)
        run: |
          python -I check.py
      - uses: actions/upload-artifact@v4
        with:
          name: nvcoop-plugin-win32
          retention-days: 14
          if-no-files-found: error
          path: |
            out/L1/nvcoop.dll
            out/L1/nvcoop_*.json
  other:
    steps:
      - run: echo other
"""
GOOD_CMAKE = {"plugin/hooks/CMakeLists.txt":
              "configure_file(${NVH_DIR}/data/guest_local_run.json ${NVCOOP_OUT_DIR}/nvcoop_guest_local_run.json COPYONLY)\n"
              "set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${NVCOOP_OUT_DIR}/bin)\n"}
GOOD_DOC = ("   RUN=\"$(gh run list ...)\"\n"
            "   if [ -n \"$RUN\" ] && gh run download -R o/r -n nvcoop-plugin-win32 -D \"$DL\" \"$RUN\"; then mv -f \"$DL\"/* out/L1/; fi\n")


def self_test() -> int:
    ok = True

    def expect(name: str, ci: str, cmake: Dict[str, str], doc: str, want_bad: bool) -> None:
        nonlocal ok
        got, _ = problems_in(ci, cmake, doc)
        if bool(got) != want_bad:
            print(f"FAIL {name}: {got}")
            ok = False

    expect("good", GOOD_CI, GOOD_CMAKE, GOOD_DOC, False)
    expect("an inline path that covers both", GOOD_CI.replace(
        "          path: |\n            out/L1/nvcoop.dll\n            out/L1/nvcoop_*.json\n", "          path: out/L1/nvcoop*\n"),
        GOOD_CMAKE, GOOD_DOC, False)
    expect("an exclusion that leaves the required files", GOOD_CI.replace("            out/L1/nvcoop_*.json\n",
           "            out/L1/nvcoop_*.json\n            !out/L1/*.pdb\n"), GOOD_CMAKE, GOOD_DOC, False)
    expect("an exclusion undone by a later pattern", GOOD_CI.replace("            out/L1/nvcoop_*.json\n",
           "            !out/L1/nvcoop_*.json\n            out/L1/nvcoop_*.json\n"), GOOD_CMAKE, GOOD_DOC, False)
    # twins: each one is a real defect the checker must refuse
    expect("twin: no upload", GOOD_CI.replace("      - uses: actions/upload-artifact@v4\n", "      - uses: actions/cache@v4\n"),
           GOOD_CMAKE, GOOD_DOC, True)
    # the quick job gets the upload, the plugin job's step becomes a cache step
    expect("twin: the upload is in another job", GOOD_CI.replace("      - uses: actions/upload-artifact@v4\n        with:",
           "      - uses: actions/cache@v4\n        with:").replace("      - run: echo quick\n",
           "      - uses: actions/upload-artifact@v4\n"), GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: the DLL alone (no data file)", GOOD_CI.replace("            out/L1/nvcoop_*.json\n", ""), GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: a new data file the list misses", GOOD_CI.replace("out/L1/nvcoop_*.json", "out/L1/nvcoop_guest_local_run.json"),
           {**GOOD_CMAKE, "plugin/x/CMakeLists.txt": "configure_file(a.txt ${NVCOOP_OUT_DIR}/nvcoop_extra.txt COPYONLY)\n"},
           GOOD_DOC, True)
    expect("twin: upload before the DLL check", GOOD_CI.replace(
        "      - name: Check nvcoop.dll (and that the check rejects a 64-bit DLL)\n        run: |\n          python -I check.py\n", "")
        .replace("            out/L1/nvcoop_*.json\n", "            out/L1/nvcoop_*.json\n"
                 "      - name: Check nvcoop.dll (and that the check rejects a 64-bit DLL)\n        run: |\n          python -I check.py\n"),
        GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: upload even after a failed check", GOOD_CI.replace("      - uses: actions/upload-artifact@v4\n",
           "      - uses: actions/upload-artifact@v4\n        if: always()\n"), GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: no if-no-files-found", GOOD_CI.replace("          if-no-files-found: error\n", ""), GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: the checker sees no build output", GOOD_CI, {"plugin/CMakeLists.txt": "project(nvcoop)\n"}, GOOD_DOC, True)
    expect("twin: the doc downloads another name", GOOD_CI, GOOD_CMAKE, GOOD_DOC.replace("-n nvcoop-plugin-win32", "-n nvcoop-win32"), True)
    expect("twin: the doc has no download", GOOD_CI, GOOD_CMAKE, "Build it on Windows.\n", True)
    expect("twin: a data file excluded after its pattern", GOOD_CI.replace("            out/L1/nvcoop_*.json\n",
           "            out/L1/nvcoop_*.json\n            !out/L1/nvcoop_guest_local_run.json\n"), GOOD_CMAKE, GOOD_DOC, True)
    expect("twin: the doc downloads with an unchecked run ID", GOOD_CI, GOOD_CMAKE,
           GOOD_DOC.replace("if [ -n \"$RUN\" ] && gh run download", "if gh run download"), True)
    expect("twin: the doc checks one ID and downloads another", GOOD_CI, GOOD_CMAKE,
           GOOD_DOC.replace("-D \"$DL\" \"$RUN\"", "-D \"$DL\" \"$(gh run list ...)\""), True)
    if ok:
        print("ALL OK")
        return 0
    return 1


def main(argv: List[str]) -> int:
    if "--self-test" in argv:
        return self_test()
    bad, checks = problems_in(CI.read_text(encoding="utf-8"), repo_cmake(), DOC.read_text(encoding="utf-8"))
    for b in bad:
        print(b)
    print(f"plugin artifact: {checks} checks, {len(bad)} failed (ci.yml plugin job, plugin CMake outputs, proton-deck.md)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
