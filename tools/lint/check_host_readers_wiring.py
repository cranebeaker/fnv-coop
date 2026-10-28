#!/usr/bin/env python3
"""The baseline-less host must read its own engine for the digest settle (two-PC benchmark 2026-10-27).

The host runs without a world baseline, so nothing else loads its volatile list or reads its quests. Three one-line wirings are easy to lose and were absent:
  * coop_module.cpp calls snapshot::PrepareVolatileKeys from the host's state poll (without it the host's list is empty: it commits per-machine vars the
    client dump leaves out, and the digest differs on them for ever);
  * coop_module.cpp sets a quest running reader (without it the host commits "not running" for every quest the client runs, and the client stops them);
  * coop_module.cpp sets a global reader, and coop_runtime.cpp turns the readers into HostSession::hostQuestRunning / hostGlobals when there is no baseline dump.

    python -I tools/lint/check_host_readers_wiring.py              # check the repo (exit 1 on a problem)
    python -I tools/lint/check_host_readers_wiring.py --self-test  # the checker's own tests and twins (prints ALL OK)
"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path
from typing import List

ROOT = Path(__file__).resolve().parents[2]
MODULE = ROOT / "plugin" / "coop" / "coop_module.cpp"
RUNTIME = ROOT / "plugin" / "coop" / "coop_runtime.cpp"


def _code(src: str) -> str:
    return re.sub(r"//[^\n]*", "", src)


def module_problems(src: str) -> List[str]:
    src = _code(src)
    bad: List[str] = []
    m = re.search(r"setStatePoll\(\[\]\(.*?\)\s*\{(.*?)\n        \}\);", src, re.S)
    if not m:
        bad.append("coop_module.cpp: the host's setStatePoll lambda was not found")
    elif "PrepareVolatileKeys" not in m.group(1):
        bad.append("the host state poll does not call snapshot::PrepareVolatileKeys: the baseline-less host never loads the volatile list")
    elif m.group(1).find("PrepareVolatileKeys") > m.group(1).find("enableStatePolling"):
        bad.append("PrepareVolatileKeys must run BEFORE the poll is primed (enableStatePolling)")
    if "setQuestRunningReader" not in src:
        bad.append("coop_module.cpp never calls setQuestRunningReader: the host stops the quests the client runs")
    if "setGlobalReader" not in src:
        bad.append("coop_module.cpp never calls setGlobalReader")
    return bad


def runtime_problems(src: str) -> List[str]:
    src = _code(src)
    bad: List[str] = []
    if not re.search(r"!host_->hostQuestRunning\s*&&\s*questRunning_", src):
        bad.append("coop_runtime.cpp does not turn the quest running reader into hostQuestRunning for a host without a baseline dump")
    if "host_->hostGlobals" not in src:
        bad.append("coop_runtime.cpp never sets hostGlobals")
    return bad


# module: the state poll lambda found, it prepares the volatile keys, before the prime; the running reader; the global reader.
# runtime: hostQuestRunning, hostGlobals.
CHECKS = 7


def problems() -> List[str]:
    return module_problems(MODULE.read_text(encoding="utf-8")) + runtime_problems(RUNTIME.read_text(encoding="utf-8"))


GOOD_MODULE = """
    g_rt->setQuestRunningReader([](const R& q, bool& r) { return true; });
    g_rt->setGlobalReader([](const R& g, double& v) { return true; });
    if (g_cfg.role == Role::Host) {
        g_rt->setStatePoll([](fnvcoop::StateReplicator& to, bool first, bool prime) {
            if (first) { fnvcoop::snapshot::PrepareVolatileKeys(o, &sum); }
            if (first) {
                nvhooks::enableStatePolling(true, prime);
            }
        });
    }
"""
GOOD_RUNTIME = """
        if (!host_->hostQuestRunning && questRunning_) { host_->hostQuestRunning = f; }
        if (globals_) { host_->hostGlobals = g; }
"""


def self_test() -> int:
    fails = 0

    def expect(name: str, got: List[str], want_ok: bool) -> None:
        nonlocal fails
        if bool(got) == want_ok:
            print("FAIL %s: %s" % (name, got))
            fails += 1

    expect("good module", module_problems(GOOD_MODULE), True)
    expect("good runtime", runtime_problems(GOOD_RUNTIME), True)
    expect("twin: no volatile prepare", module_problems(GOOD_MODULE.replace("PrepareVolatileKeys", "Other")), False)
    expect("twin: prepare after prime", module_problems(GOOD_MODULE.replace("if (first) { fnvcoop::snapshot::PrepareVolatileKeys(o, &sum); }\n            if (first) {\n                nvhooks::enableStatePolling(true, prime);",
                                                                          "if (first) {\n                nvhooks::enableStatePolling(true, prime);\n                fnvcoop::snapshot::PrepareVolatileKeys(o, &sum);")), False)
    expect("twin: no running reader", module_problems(GOOD_MODULE.replace("setQuestRunningReader", "setSomething")), False)
    expect("twin: no global reader", module_problems(GOOD_MODULE.replace("setGlobalReader", "setSomething")), False)
    expect("twin: runtime without running hook", runtime_problems(GOOD_RUNTIME.replace("questRunning_", "other_")), False)
    expect("twin: runtime without globals hook", runtime_problems(GOOD_RUNTIME.replace("hostGlobals", "other")), False)
    if fails:
        return 1
    print("ALL OK")
    return 0


def main(argv: List[str]) -> int:
    if "--self-test" in argv:
        return self_test()
    bad = problems()
    for b in bad:
        print("FAIL: " + b)
    if not bad:
        # the count is what tools/test_all.py takes as proof that the check ran; "ok" alone made test_all FAIL the suite
        if os.environ.get("HOSTREADERS_MUTANT") == "bare_ok":   # twin, tools/test_all_evidence.py
            print("ok: the host's engine readers are wired")
        else:
            print("host engine readers: %d checks, 0 failed (coop_module.cpp, coop_runtime.cpp)" % CHECKS)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
