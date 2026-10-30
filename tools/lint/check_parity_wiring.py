#!/usr/bin/env python3
"""Static wiring check of the live sync parity checker (plugin/parity, docs/tools/parity.md).

parity_module.cpp needs the game to run, so no unit test executes it. What can go wrong is that a call site gets lost: the module drops out of the build, the
per-frame call or the bridge methods vanish, the release ini stops switching it on, the channel byte collides with another module, a twin number is no longer
run, or the engine source starts to WRITE to the game (the checker is read-only by design). This script reads the files and fails on any of those.

    python -I tools/lint/check_parity_wiring.py              check the real files (exit 1 on a problem)
    python -I tools/lint/check_parity_wiring.py --self-test  also remove every required call site in turn (and add every forbidden one) and prove the check
                                                             catches each: the mutation twins of this check (prints ALL OK)
"""
from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple

ROOT = Path(__file__).resolve().parents[2]
PLUGIN_CMAKE = "plugin/CMakeLists.txt"
PARITY_CMAKE = "plugin/parity/CMakeLists.txt"
MODULE = "plugin/parity/parity_module.cpp"
CONFIG = "plugin/parity/parity_config.cpp"
RELEASE = "tools/installer/package_release.py"
PROTOCOL = "docs/protocol.md"
GUIDE = "docs/PLAYER-GUIDE.md"
DOC = "docs/tools/parity.md"
RUN_TESTS = "plugin/parity/run_tests.cmd"
TYPES = "plugin/parity/parity_types.h"

FILES = [PLUGIN_CMAKE, PARITY_CMAKE, MODULE, CONFIG, RELEASE, PROTOCOL, GUIDE, DOC, RUN_TESTS, TYPES,
         "plugin/parity/parity_service.cpp", "plugin/parity/parity_sampler.cpp", "plugin/parity/parity_compare.cpp", "plugin/parity/parity_wire.cpp",
         "plugin/parity/parity_json.cpp", "plugin/parity/parity_anim.h", "plugin/parity/tests/test_parity.cpp"]

# (file, pattern, why it is needed)
REQUIRED: List[Tuple[str, str, str]] = [
    (PLUGIN_CMAKE, r"foreach\(sub[^)]*\bparity\b", "plugin/CMakeLists.txt does not list parity: the module is not in the build"),
    (PARITY_CMAKE, r"target_sources\(nvcoop PRIVATE \$\{NVPY_DIR\}/parity_module\.cpp\)", "parity_module.cpp is not a direct source of nvcoop (NVCOOP_MODULE must be)"),
    (PARITY_CMAKE, r"target_link_libraries\(nvcoop PRIVATE nvcoop_parity_core\)", "nvcoop does not link the parity core"),
    (MODULE, r"NVCOOP_MODULE\(sync_parity,\s*\"sync_parity\",\s*onLoad,\s*onMessage,\s*onFrame\)", "the module is not registered with its load / message / frame callbacks"),
    (MODULE, r"NVCOOP_RELOAD_STATE\(sync_parity", "no hot-reload participant: the sample file stays open across a core reload"),
    (MODULE, r"RegisterMethodClosure\(\"parity\.status\"", "bridge method parity.status is gone"),
    (MODULE, r"RegisterMethodClosure\(\"parity\.sample\"", "bridge method parity.sample is gone"),
    (MODULE, r"nvcoop::coop::Runtime::active\(\)", "the frame no longer reads the co-op runtime: the service never learns about the session"),
    (MODULE, r"g\.svc->frame\(f\)", "the per-frame call of the service is gone"),
    (MODULE, r"if \(!g\.configured \|\| !g\.set\.enabled\) return;", "the frame no longer returns early when [parity] is off"),
    (MODULE, r"GetPrivateProfileStringA", "the ini is no longer read"),
    (CONFIG, r"get\(\"parity\",\s*\"enabled\",\s*\"0\"\)", "[parity] enabled must default to 0 in code (the release ini switches it on)"),
    (RELEASE, r"\"\[parity\]\\n\"\s*\n\s*\"enabled=1\\n\"", "the release ini does not switch [parity] on"),
    (PROTOCOL, r"\|\s*`0x9E`\s*\|", "docs/protocol.md section 2 has no row for the parity channel 0x9E"),
    (GUIDE, r"nvcoop_parity\.jsonl", "docs/PLAYER-GUIDE.md does not ask for nvcoop_parity.jsonl when reporting desync"),
    (DOC, r"nvcoop_parity\.jsonl", "docs/tools/parity.md is missing or does not describe the sample file"),
    (TYPES, r"kChannelParity\s*=\s*0x9E", "the channel byte constant changed (update docs/protocol.md and this check)"),
    (RUN_TESTS, r"--twins\s+1800-1827", "run_tests.cmd does not run the twin range"),
]

# Read-only guarantee: nothing in the parity sources may call these (a second WinBackend with no console cannot run script lines, but the intent is checked too)
FORBIDDEN = ["RunLine(", "WriteNpcLook", "CloneNpcBase", "SafeWrite", "WriteProcessMemory", "VirtualProtect", "console_", "->Disable", "SetPos(", "PlayGroup"]
BACKEND_ALLOWED = {"PinOk", "ReadRef", "ReadCell", "MenuOpen", "LoadedCells", "ListActors", "ReadAnim", "ReadAppearance", "Eval"}


def _code(src: str) -> str:
    return re.sub(r"//[^\n]*", "", src)


def problems(files: Dict[str, str]) -> List[str]:
    bad: List[str] = []
    for f, pat, why in REQUIRED:
        text = files.get(f)
        if text is None:
            bad.append(f"{f}: missing ({why})")
        elif not re.search(pat, text if f.endswith((".md", ".py", ".cmd")) else _code(text), re.S):
            bad.append(f"{f}: {why}")
    # read-only
    for f in files:
        if f.startswith("plugin/parity/") and not f.startswith("plugin/parity/tests/") and f.endswith((".cpp", ".h")):
            code = _code(files[f])
            for tok in FORBIDDEN:
                if tok in code:
                    bad.append(f"{f}: uses {tok!r}: the parity checker must only read the game")
    mod = _code(files.get(MODULE, ""))
    for m in sorted(set(re.findall(r"\bbe_\.(\w+)\(", mod))):
        if m not in BACKEND_ALLOWED:
            bad.append(f"{MODULE}: calls be_.{m}(), which is not in the read-only list {sorted(BACKEND_ALLOWED)}")
    # twins: every FNV_MUT number used in the parity sources is run by run_tests.cmd, and run_tests.cmd runs no number nothing uses
    used = set()
    for f, text in files.items():
        if f.startswith("plugin/parity/") and f.endswith((".cpp", ".h")):
            used |= {int(n) for n in re.findall(r"FNV_MUT\((\d+)\)", _code(text))}
    spec = re.search(r"--twins\s+([\d,\-]+)", files.get(RUN_TESTS, ""))
    run: set = set()
    if spec:
        for part in spec.group(1).split(","):
            if "-" in part:
                a, b = part.split("-")
                run |= set(range(int(a), int(b) + 1))
            elif part:
                run.add(int(part))
    for n in sorted(used - run):
        bad.append(f"twin {n} is in the sources but run_tests.cmd does not run it")
    tests = files.get("plugin/parity/tests/test_parity.cpp", "")
    for n in sorted(used):
        if str(n) not in tests:
            bad.append(f"twin {n} is not named in tests/test_parity.cpp (no test is documented to catch it)")
    if not used:
        bad.append("no FNV_MUT twin found in plugin/parity")
    # the channel byte is unique among the other kChannel constants
    m = re.search(r"kChannelParity\s*=\s*(0x[0-9A-Fa-f]+)", files.get(TYPES, ""))
    if m:
        byte = int(m.group(1), 16)
        for other in _other_channels():
            if other == byte:
                bad.append(f"channel byte {m.group(1)} is used by another module")
    return bad


def _other_channels() -> List[int]:
    out = []
    for base in ("plugin", "synccore"):
        for p in (ROOT / base).rglob("*.h"):
            if "parity" in p.parts or "tests" in p.parts or "build" in p.parts:
                continue
            try:
                text = p.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for v in re.findall(r"constexpr\s+uint8_t\s+k(?:Channel|Slide)\w*\s*=\s*(0x[0-9A-Fa-f]+)", text):
                out.append(int(v, 16))
    return out


def read_files() -> Dict[str, str]:
    out = {}
    for f in FILES:
        p = ROOT / f
        if p.is_file():
            out[f] = p.read_text(encoding="utf-8", errors="replace")
    return out


def self_test() -> int:
    files = read_files()
    base = problems(files)
    if base:
        print("the real files already fail:", *base, sep="\n  ")
        return 1
    ok = True
    # every required call site removed in turn
    for f, pat, why in REQUIRED:
        text = files[f]
        m = re.search(pat, text if f.endswith((".md", ".py", ".cmd")) else _code(text), re.S)
        mutated = dict(files)
        mutated[f] = text.replace(m.group(0), "") if m and m.group(0) in text else ""
        if not problems(mutated):
            print("MISSED: removing the required text did not fail:", f, pat)
            ok = False
    # every forbidden token added
    for tok in FORBIDDEN:
        mutated = dict(files)
        mutated[MODULE] = files[MODULE] + "\nvoid sneaky() { be_." + tok + "; }\n"
        if not problems(mutated):
            print("MISSED: adding", tok, "did not fail")
            ok = False
    mutated = dict(files)
    mutated[MODULE] = files[MODULE] + "\nvoid sneaky2() { be_.SetEverything(1); }\n"
    if not problems(mutated):
        print("MISSED: a backend call outside the read-only list did not fail")
        ok = False
    # a twin that run_tests.cmd forgets
    mutated = dict(files)
    mutated[RUN_TESTS] = files[RUN_TESTS].replace("1800-1827", "1800-1826")
    if not problems(mutated):
        print("MISSED: a twin that run_tests.cmd does not run did not fail")
        ok = False
    # a channel byte that collides
    mutated = dict(files)
    mutated[TYPES] = files[TYPES].replace("0x9E", "0xC8")
    if not [p for p in problems(mutated) if "channel byte" in p]:
        print("MISSED: a colliding channel byte did not fail")
        ok = False
    if ok:
        print("ALL OK")
        return 0
    return 1


def main(argv: List[str]) -> int:
    if "--self-test" in argv:
        return self_test()
    bad = problems(read_files())
    for b in bad:
        print(b)
    print("parity wiring: OK (%d required call sites, read-only and twin checks: %d checks, 0 failed)" % (len(REQUIRED), len(REQUIRED) + 4) if not bad
          else f"parity wiring: {len(bad)} problem(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
