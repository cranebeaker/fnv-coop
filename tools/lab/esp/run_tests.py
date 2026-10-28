"""One command for the NVCoopLab.esp generator:  python -I tools/lab/esp/run_tests.py   -> prints ALL OK

1. the unit suite (tests/test_lab_esp.py) must pass on the real code;
2. every mutation twin (NVLABESP_MUTANT=<name>, one deliberate bug each in build_lab.py or install_lab.py) must make that suite FAIL.
Tests that read FalloutNV.esm (a read-only copy: FNV_ESM, or <games>/fnv-instances/0/Data) skip without it; then the twins that
need it are reported as not run and the script ends in SKIPPED (exit 0), never ALL OK, plus a "N twins run, 0 failed" line for the
part that did run. Format and installer tests always run.
"""
from __future__ import annotations

import os
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent.parent
sys.path.insert(0, str(HERE))

# twins in the generator (need the game file) and in the installer (do not)
BUILD_MUTANTS = ("wrong_subblock", "not_persistent", "dup_form_id", "layout_wrong_form", "hedr_count", "no_master", "terminal_unlinked",
                 "patrol_open", "quest_stage_missing", "chest_count", "edge_margin", "disabled_flag_lost", "stand_far", "lock_level_zero",
                 "cell_edid_kept", "npc_pkg_inherited", "persistent_actor_enabled", "unsorted_top")
# twins for decor.py (lights, partitions): the clearance twin is wall_in_room; wall_no_collision also needs Fallout - Meshes.bsa
DECOR_MUTANTS = ("wall_in_room", "no_light_room", "dim_cell", "light_flicker", "decor_id_clash")
DECOR_BSA_MUTANTS = ("wall_no_collision",)
INSTALL_MUTANTS = ("clobbers_slot1", "keeps_esp", "keeps_ini", "game_dir_allowed")


TEST_FILES = ("test_lab_esp.py", "test_lab_decor.py")


def run(mutant: str = "") -> subprocess.CompletedProcess:
    """Both test files under one mutant; the result is the first failing one (or the last, when all pass)."""
    env = dict(os.environ)
    env.pop("NVLABESP_MUTANT", None)
    if mutant:
        env["NVLABESP_MUTANT"] = mutant
    res = None
    for name in TEST_FILES:
        res = subprocess.run([sys.executable, "-I", str(HERE / "tests" / name)], cwd=ROOT, capture_output=True, text=True, env=env, errors="replace")
        if res.returncode != 0:
            break
    return res


def main() -> int:
    import build_lab
    t0 = time.time()
    have_esm = build_lab.default_esm().is_file()
    r = run()
    tail = (r.stdout + r.stderr).strip().splitlines()
    print("real suite:", tail[-1] if tail else "(no output)")
    bad = 0
    if r.returncode != 0:
        print("\n".join(tail[-40:]))
        bad += 1
    have_bsa = have_esm and build_lab.default_esm().with_name("Fallout - Meshes.bsa").is_file()
    twins = list(INSTALL_MUTANTS) + (list(BUILD_MUTANTS + DECOR_MUTANTS) if have_esm else []) + (list(DECOR_BSA_MUTANTS) if have_bsa else [])
    survived = [m for m in twins if run(m).returncode == 0]
    if survived:
        print("SURVIVED:", ", ".join(survived))
        bad += 1
    if bad:
        print("FAILED")
        return 1
    print("NVCoopLab.esp generator: tests pass, %d of %d twins caught (%.1fs)" % (len(twins), len(INSTALL_MUTANTS) + len(BUILD_MUTANTS) + len(DECOR_MUTANTS) + len(DECOR_BSA_MUTANTS), time.time() - t0))
    if not have_esm:
        print("SKIPPED: FalloutNV.esm not found, the game-file tests and %d generator twins did not run" % (len(BUILD_MUTANTS) + len(DECOR_MUTANTS) + len(DECOR_BSA_MUTANTS)))
        # what did run, in a form tools/test_all.py takes as proof (without it a machine with no game, as CI, FAILed the suite)
        if os.environ.get("NVLABESP_RUNNER_MUTANT") != "no_ran_count":   # twin, tools/test_all_evidence.py
            print("format and installer part: real suite and %d twins run, 0 failed" % len(twins))
        return 0
    print("ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
