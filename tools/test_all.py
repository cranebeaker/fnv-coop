"""Run every lane's game-free test suite; exit non-zero if any fails.

Usage: python -I tools/test_all.py [--quick] [--affected [--base REF] [--list]] [--only SUBSTR]
--quick skips the slow suites: the C++ builds (L1, L4, L5, L6, L8, L10-L12, T1a, T2b, T3, T6a driver, T7 probe, M2/M3
plugins, splitscreen) and the T1b full run.
--affected runs only the suites whose sources changed vs git HEAD (+ untracked files); --list just prints the selection.
--only SUBSTR runs only suites whose name contains SUBSTR (repeatable).
Before the suites run, the tools they need (CMake + Ninja, the vcvars scripts) are checked once; a suite that needs a missing
tool is FAIL with one clear reason instead of dying inside its build script (see tools/test_all_selftest.py).
"""
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Tools a suite needs before it can run. The self-test scans each runner script (and the scripts it calls) for cmake
# and vcvars calls and fails when this list disagrees with the scan.
CM = ("cmake", "msvc")  # calls vcvars, then cmake -G Ninja (or needs the build_l1.cmd output)
MS = ("msvc",)          # calls vcvars, then cl directly
MS_OPT = ("msvc-optional",)  # calls vcvars and cl only when the vcvars file exists, else skips that part
NO = ()                 # Python only

SUITES = [
    # (name, command, skipped by --quick, tools it needs). "Skipped by --quick" is the old "compiles C++" flag, plus the T1b full run.
    # first and fast: a string literal split by a real line break (C2001) fails here in a second, not after a long compile
    ("lint split string literals", [sys.executable, "-I", "tools/lint/fix_newline_literals.py", "--check"], False, NO),
    ("lint tool tests (+ twin)", [sys.executable, "-I", "tools/lint/test_fix_newline_literals.py", "--twin"], False, NO),
    ("lint every Python file parses", [sys.executable, "-I", "tools/lint/check_py_compiles.py"], False, NO),
    ("lint loot service is in the live hub (+ twins)", [sys.executable, "-I", "tools/lint/check_loot_wiring.py", "--self-test"], False, NO),
    ("lint loot service wiring", [sys.executable, "-I", "tools/lint/check_loot_wiring.py"], False, NO),
    ("lint host engine readers are wired (+ twins)", [sys.executable, "-I", "tools/lint/check_host_readers_wiring.py", "--self-test"], False, NO),
    ("lint host engine readers wiring", [sys.executable, "-I", "tools/lint/check_host_readers_wiring.py"], False, NO),
    ("lint workflow inputs stay out of run scripts, soak seed and count checked (+ twins)", [sys.executable, "-I", "tools/lint/check_workflow_inputs.py", "--self-test"], False, NO),
    ("lint workflow inputs", [sys.executable, "-I", "tools/lint/check_workflow_inputs.py"], False, NO),
    ("test_all preflight", [sys.executable, "-I", "tools/test_all_selftest.py"], False, NO),
    ("canonical nvcoop.dll build", ["cmd", "/c", r"tools\runner\build_l1.cmd"], True, CM),
    ("T5 chaos box (+ chaos_pair: a pair joined through the relay)", [sys.executable, "-I", "tools/chaos/run_tests.py"], False, NO),
    ("L3 questdata", [sys.executable, "-I", "tools/questdata/run_tests.py"], False, NO),
    ("L7 snapshot", [sys.executable, "-I", "plugin/snapshot/tools/run_tests.py", "--no-engine"], False, MS_OPT),
    ("L9 coverage", [sys.executable, "-I", "tools/coverage/run_tests.py"], False, NO),
    ("T6a bot actions", [sys.executable, "-I", "tools/bots/run_tests.py"], False, NO),
    ("T6b co-op playbooks", [sys.executable, "-I", "tools/bots/playbooks/tests/run_tests.py"], False, NO),
    ("M3 actor ownership playbook (fake world + twins)", [sys.executable, "-I", "tools/bots/playbooks/tests/run_actor_tests.py"], False, NO),
    ("M3 actor port in-game proof (offline self-test + twins)", [sys.executable, "-I", "tools/actors/run_tests.py"], False, NO),
    ("Enemy sync tests: fight, handoff, spawn, corpse loot (fake world + twins, load-robust clock)", [sys.executable, "-I", "tools/bots/playbooks/tests/run_enemy_tests.py"], False, NO),
    ("Questline sweeps: 17 plans, two-copy playthrough playbooks (fake world + twins), rules overlay", [sys.executable, "-I", "tools/questsweep/run_tests.py"], False, NO),
    ("T4a warm pool", [sys.executable, "-I", "tools/pool/run_tests.py"], False, NO),
    ("Lab plugin generator NVCoopLab.esp: TES4 reader/writer, real-esm layout proofs, generated plugin + layout.json, installer (+ 22 twins; the game-file part skips without FalloutNV.esm)", [sys.executable, "-I", "tools/lab/esp/run_tests.py"], False, NO),
    ("Live wiring in-game proof (selftest + injected defects)", [sys.executable, "-I", "tools/runner/live_wiring_proof.py", "--selftest"], False, NO),
    ("Release: installer, uninstaller, packager (fake game folder + twins)", [sys.executable, "-I", "tools/installer/run_tests.py"], False, NO),
    ("Release: launcher (settings, nvcoop.ini, port checks, start; + twins)", [sys.executable, "-I", "tools/launcher/run_tests.py"], False, NO),
    ("Proof harness: prove.py classifier, packet, ledger, owner-PC safety, --pairs scheduler, RAM guard (fake worlds, stub runners + 22 twins)", [sys.executable, "-I", "tools/proof/tests/run_tests.py"], False, NO),
    ("Lab runner (rooms on a fake two-copy world, one test per fake bug, fail fast, time boxes, summary.md under 5 KB + 7 twins)", [sys.executable, "-I", "tools/lab/run_tests.py"], False, NO),
    ("Proof drivers: release .cmd scripts, launcher loopback, public tree build, split-screen smoke (fake runners, damaged-release and launcher-fault twins + 5 twins)", [sys.executable, "-I", "tools/proof/drivers/tests/run_tests.py"], False, NO),
    ("T4b scenarios", [sys.executable, "-I", "-X", "utf8", "tools/scenarios/run_tests.py"], False, NO),
    ("T1b flight analyzer (fast)", [sys.executable, "-I", "tools/flight/run_tests.py"], False, NO),
    ("T1b flight analyzer (full + mutation twin)", [sys.executable, "-I", "tools/flight/run_tests.py", "--full"], True, NO),
    ("T7 animcheck", [sys.executable, "-I", "tools/animcheck/run_tests.py"], False, NO),
    ("T7 animcheck NPC fight assertions (+ twins)", [sys.executable, "-I", "tools/animcheck/run_npc_tests.py"], False, NO),
    ("L2+L1 runner (py)", [sys.executable, "-I", "-m", "unittest", "discover", "-s", "tools/runner/tests"], True, CM),
    ("L6 nvmp sigs (pinned 9.4)", [sys.executable, "-I", "plugin/nvmp/tools/verify_sigs.py"], False, NO),
    ("L6 nvmp compat tool", [sys.executable, "-I", "plugin/nvmp/tools/test_compat.py"], False, NO),
    ("L6 nvmp tests", ["cmd", "/c", r"plugin\nvmp\run_tests.cmd"], True, MS),
    ("L8 dialogue", [sys.executable, "-I", "plugin/dialogue/run_tests.py"], True, MS),
    ("L5 hooks", ["cmd", "/c", r"plugin\hooks\run_tests.bat"], True, CM),
    ("story-capture (RefCommand ops, console lines, engine argument layouts, 11 runtime twins)", ["cmd", "/c", r"plugin\hooks\run_story_tests.cmd"], True, CM),
    ("story-capture generated tables + in-game proof checker (selftest, injected defects, twin)", [sys.executable, "-I", "plugin/hooks/tools/run_story_checks.py"], False, NO),
    ("T2b simfuzz", [sys.executable, "-I", "tools/simfuzz/run_tests.py"], True, CM),
    ("L4 synccore", ["cmd", "/c", r"synccore\run_tests.bat"], True, CM),
    ("combat-core (rules + 65 runtime twins, engine-model combat + twins)", ["cmd", "/c", r"synccore\combat\run_tests.cmd"], True, CM),
    # after the C++ suites: the twin-registration half needs the built exes in build/all (a missing exe is skipped, not failed)
    ("repo hygiene tool tests (+ twins)", [sys.executable, "-I", "tools/lint/check_repo_hygiene.py", "--self-test"], False, NO),
    ("repo hygiene (ignored sources, unregistered twins)", [sys.executable, "-I", "tools/lint/check_repo_hygiene.py"], False, NO),
    ("L6 client patcher", ["cmd", "/c", r"plugin\nvmp\client_patches\run_tests.cmd"], True, MS),
    ("L6 server hook", ["cmd", "/c", r"plugin\nvmp\server_hook\run_tests.cmd"], True, MS),
    ("T6a bot input driver", ["cmd", "/c", r"plugin\bots\run_tests.cmd"], True, CM),
    ("L10 coop runtime", ["cmd", "/c", r"plugin\coop\run_tests.cmd"], True, CM),
    ("Live wiring (ini switches, hub, avatar net, twins)", ["cmd", "/c", r"plugin\coop\live\run_tests.cmd"], True, CM),
    ("L11 hooks_ui", ["cmd", "/c", r"plugin\hooks_ui\run_tests.cmd"], True, CM),
    ("L12 hooks_world", ["cmd", "/c", r"plugin\hooks_world\run_tests.cmd"], True, CM),
    ("M3 plugin actors (ownership, twins)", ["cmd", "/c", r"plugin\actors\run_tests.cmd"], True, CM),
    ("Spawn-site arbitration (synccore core + plugin, storm, twins, proof selftest)", ["cmd", "/c", r"plugin\spawn\run_tests.cmd"], True, CM),
    ("Loot, containers, drops, pickups, clutter (synccore core + plugin, model fuzz, 54 runtime twins, proof selftest)", ["cmd", "/c", r"plugin\loot\run_tests.cmd"], True, CM),
    ("Area desync digests (synccore core + plugin, storm, 58 runtime twins, flight export, dashboard, proof selftest)", ["cmd", "/c", r"plugin\areadigest\run_tests.cmd"], True, CM),
    ("Area desync dashboard + playbook hook (python, 6 twins)", [sys.executable, "-I", "tools/areadigest/run_tests.py"], False, NO),
    ("Guest intents (wire, policy, host + guest, real Runtime with 2 guests, save gate, dialogue runner, pins, 25 twins, proof selftest)", ["cmd", "/c", r"plugin\intents\run_tests.cmd"], True, CM),
    ("World time (clock authority, wait / sleep / travel consent, saves, object poll, wire, twins, proof selftest)", ["cmd", "/c", r"plugin\world_time\run_tests.cmd"], True, CM),
    ("M3 actor port (engine adapter, feed, API, 2-machine chain, twins)", ["cmd", "/c", r"plugin\actors\engine\run_tests.cmd"], True, CM),
    ("Enemy-anim NPC animation (sampler, driver, corpse, script backend, pins, twins)", ["cmd", "/c", r"plugin\actors\anim\run_tests.cmd"], True, CM),
    ("M4 combat-hooks (hit gate, owner-applied hits, deaths, projectiles, call-site patches, twins)", ["cmd", "/c", r"plugin\combat\run_tests.cmd"], True, CM),
    ("M4 combat in-game proof checker (selftest)", [sys.executable, "-I", "plugin/combat/proof/combat_proof.py", "--selftest"], False, NO),
    ("M4 real-time VATS (policy, party slow-mo, limb API, perks, wire, sync on 3 machines, call-site patcher, pins verifier + proof selftest, 60 runtime twins)", ["cmd", "/c", r"plugin\vats\run_tests.cmd"], True, CM),
    ("M4 VATS playbooks (gun, melee, explosive, energy on the fake VATS world, 19 twins)", [sys.executable, "-I", "tools/bots/playbooks/tests/run_vats_tests.py"], False, NO),
    ("social-core (companions, crime, kill rewards: rules + 66 runtime twins)", ["cmd", "/c", r"synccore\social\run_tests.cmd"], True, CM),
    ("companions-crime (SocialSync on 3 machines: companions, crime, kill rewards + 28 twins)", ["cmd", "/c", r"plugin\companions\run_tests.cmd"], True, CM),
    ("companions-crime in-game proof checker (selftest)", [sys.executable, "-I", "plugin/companions/proof/social_proof.py", "--selftest"], False, NO),
    ("Readiness wait shared by prove.py and the proof scripts (+ twins, wiring of every script)", [sys.executable, "-I", "tools/runner/tests/test_nvready.py"], False, NO),
    ("NPC pick for the in-game NPC proofs: loaded NPC or the nearest one, runtime ids (+ 7 twins)", [sys.executable, "-I", "tools/runner/tests/test_npc_pick.py"], False, NO),
    ("Proton launch scripts: Steam launch option and in-session runner (stubbed Steam client + 7 twins)", [sys.executable, "-I", "tools/runner/tests/test_proton_scripts.py"], False, NO),
    ("provision --no-pad: copies ignore game controllers, the low-cost restore keeps it or stops (+ 6 twins)", [sys.executable, "-I", "tools/runner/tests/test_no_pad.py"], False, NO),
    ("loot in-game proof checker (selftest + late-session twin)", [sys.executable, "-I", "plugin/loot/tools/loot_proof.py", "--selftest"], False, NO),
    ("world-time in-game proof checker (selftest + press-blind twin)", [sys.executable, "-I", "plugin/world_time/tools/worldtime_proof.py", "--selftest"], False, NO),
    ("T1a flight recorder", ["cmd", "/c", r"plugin\flight\run_tests.cmd"], True, CM),
    ("T3b reload state", ["cmd", "/c", r"plugin\reload\run_tests.cmd"], True, CM),
    ("T3a loader", ["cmd", "/c", r"plugin\loader\run_tests.cmd"], True, CM),
    ("T7 animprobe core", ["cmd", "/c", r"plugin\animprobe\run_tests.cmd"], True, CM),
    ("Track B splitscreen focus guard", ["cmd", "/c", r"plugin\splitscreen\run_tests.cmd"], True, CM),
    ("Track B splitscreen tools (launcher, layout, placement, plan, playbook selftest, twins, pins)", [sys.executable, "-I", "tools/splitscreen/run_tests.py", "--skip-cpp"], True, CM),
    ("M2 avatar puppets (driver, live layer, twins, proof selftest)", ["cmd", "/c", r"plugin\avatar\run_tests.cmd"], True, CM),
    ("M2 avatar engine (real puppet engine, local sampler, runtime, twins, pins, proof selftest)", ["cmd", "/c", r"plugin\avatar\engine\run_tests.cmd"], True, CM),
    ("M2 avatar full (3-machine chain: 20/5 Hz rates, look + weapon mods, lifecycle, ghost-ref API, puppet hits, wire v2, twins, proof selftest)", ["cmd", "/c", r"plugin\avatar\full\run_tests.cmd"], True, CM),
    ("build-speed tooling (runtime twins, runner, lock, --affected)", [sys.executable, "-I", "tools/mut/run_tests.py"], True, CM),
]


import re

MUTANT = os.environ.get("TEST_ALL_MUTANT", "")
WINDOWS = sys.platform == "win32"   # MSVC and the build scripts (vcvars, build_l1.cmd) exist on Windows only

# Public export: suites that need your own game install or generated game data skip with a reason instead of failing.
# Each list holds every file the suite opens (found by reading its loaders and by tracing a run). A partial
# out/questdata (for example a run_all.py that stopped before m1.py) must skip, not die on FileNotFoundError.
QD = "out/questdata/"
QUESTDATA = "needs out/questdata (python -I tools/questdata/run_all.py --data \"<your game>\\Data\")"
NEEDS = {
    "L6 nvmp sigs (pinned 9.4)": (("third_party/nvmp/nvmp/client.dll",),
                                  "needs your own NV:MP 9.4 client.dll under third_party/nvmp/nvmp/"),
    # plugin/dialogue/tools/gen_m1_dialogue.py and gen_companion_topics.py
    "L8 dialogue": (tuple(QD + f for f in ("forms.json.gz", "dials.json.gz", "infos.jsonl.gz", "scripts.json.gz")),
                    QUESTDATA),
    # goodsprings_plan.load_plan / check_plan (scenarios/VCG0{1,2,3}.json), scenario_adapter, test_playbooks (forms)
    "T6b co-op playbooks": (tuple(QD + f for f in ("quests.json", "m1_goodsprings.json", "refnames.json.gz",
                                                   "forms.json.gz", "scenarios/VCG01.json", "scenarios/VCG02.json",
                                                   "scenarios/VCG03.json")), QUESTDATA),
    # scenlib.validate_file (forms, quests) and scenario_adapter._base_of (forms) in tests/test_enemy.py; the rest of that suite is fake-world only
    "Enemy sync tests: fight, handoff, spawn, corpse loot (fake world + twins, load-robust clock)": (
        tuple(QD + f for f in ("forms.json.gz", "quests.json")), QUESTDATA),
    # scenlib.load_forms / load_quests / load_dials / load_infos / reset_ids (m1_goodsprings.json) / stage_world_refs
    # (scenarios/VCG02.json: without it the jump tests fail on an assertion, not on a missing file)
    "T4b scenarios": (tuple(QD + f for f in ("forms.json.gz", "quests.json", "dials.json.gz", "infos.jsonl.gz",
                                             "m1_goodsprings.json", "scenarios/VCG02.json")), QUESTDATA),
}


# Suites that drive Windows processes themselves (kernel32 through ctypes, taskkill, the mbcs codec, C:\ paths), not
# through MSVC: off Windows they skip with this reason instead of failing on the missing API. Empty now: the proof harness
# answers from /proc off Windows (tools/proof/pf_safety.py, pf_crash.py), so its fake-world tests run on Linux too.
WINDOWS_ONLY = {}


def missing_data(name: str) -> list:
    """The files under ROOT that suite `name` needs and that do not exist."""
    files, _ = NEEDS.get(name, ((), ""))
    if MUTANT == "no_questdata_gate" and files and files[0].startswith(QD):
        files = ()  # twin: runs data suites without their data
    if MUTANT == "questdata_one_file":
        files = files[:1]  # twin: checks only the first file, so a partial out/questdata gets through
    return [f for f in files if not (ROOT / f).exists()]


# The build scripts call vcvars by this full path, then cmake -G Ninja. vcvars puts the VS-bundled CMake and Ninja on
# PATH only when the VS component "C++ CMake tools for Windows" is installed. Without CMake every CMake script dies on
# "'cmake' is not recognized". Without vcvars every C++ script dies on "The system cannot find the path specified".
VS_ROOT = Path(r"C:\Program Files\Microsoft Visual Studio\2022\Community")
VS_CMAKE_DIR = VS_ROOT / r"Common7\IDE\CommonExtensions\Microsoft\CMake"
VCVARS_DIR = VS_ROOT / r"VC\Auxiliary\Build"
VCVARS = ("vcvars32.bat", "vcvars64.bat", "vcvarsall.bat")  # every name a build script calls


def toolchain(which=shutil.which, vs_dir=VS_CMAKE_DIR, vcvars_dir=VCVARS_DIR) -> dict:
    """{"cmake": (found, problem), "msvc": (found, problem)}. Exactly one string of each pair is empty."""
    if MUTANT == "no_preflight":  # twin: never checks
        return {"cmake": ("not checked", ""), "msvc": ("not checked", "")}
    found, missing = [], []
    for tool, sub in (("cmake", r"CMake\bin\cmake.exe"), ("ninja", r"Ninja\ninja.exe")):
        where = which(tool) or ((vs_dir / sub).exists() and str(vs_dir / sub))
        (found if where else missing).append(f"{tool} {where}" if where else tool)
    cmake = ("", " and ".join(missing) + " not found: install them (winget install Kitware.CMake Ninja-build.Ninja) "
             "or the Visual Studio 2022 component 'C++ CMake tools for Windows'") if missing else (", ".join(found), "")
    gone = [] if MUTANT == "no_vcvars_check" else [n for n in VCVARS if not (vcvars_dir / n).exists()]
    names = " and ".join(filter(None, (", ".join(gone[:-1]), gone[-1]))) if gone else ""
    msvc = ("", f"{names} not found in {vcvars_dir} (the build scripts call vcvars by this path): install "
            "Visual Studio 2022 Community with the workload 'Desktop development with C++'") if gone else \
        (f"{', '.join(VCVARS)} in {vcvars_dir}", "")
    return {"cmake": cmake, "msvc": msvc}


def required_tools(slow: bool, tools: tuple) -> tuple:
    """The tools a suite needs. This is not the --quick flag: T1b full is slow but Python only."""
    if MUTANT == "tools_from_quick":  # twin: the old proxy, every slow suite needs CMake and MSVC
        return CM if slow else NO
    return tools


def decide(name: str, slow: bool, tools: tuple, quick: bool, problems: dict):
    """('run', '') or ('skip' | 'fail', reason) for one suite, before it runs. problems: tool -> reason or ''."""
    if quick and slow:
        return "skip", ""
    missing = missing_data(name)
    if missing:
        shown = ", ".join(missing[:3]) + (f" and {len(missing) - 3} more" if len(missing) > 3 else "")
        return "skip", f"{NEEDS[name][1]}. Missing: {shown}"
    if not WINDOWS and MUTANT != "no_platform_skip" and any(t in ("cmake", "msvc") for t in required_tools(slow, tools)):
        return "skip", f"needs the MSVC/CMake toolchain, which is Windows only (this is {sys.platform})"
    if not WINDOWS and MUTANT != "no_platform_skip" and name in WINDOWS_ONLY:
        return "skip", f"{WINDOWS_ONLY[name]}, which is Windows only (this is {sys.platform})"
    reasons = [problems[t] for t in required_tools(slow, tools) if problems.get(t)]
    if reasons:
        return "fail", ". ".join(reasons)
    return "run", ""

# Positive evidence that tests actually executed (unittest, ctest, custom runners).
EVIDENCE = re.compile(
    r"Ran [1-9]\d* tests?|100% tests passed|ALL OK|RESULT: ALL OK|all signatures unique|"
    r"\b[1-9]\d* run, 0 failed|, 0 failed|\b[1-9]\d* checks?,? 0 failed|all signatures resolve uniquely|"
    r"Linking CXX shared library \S*nvcoop\.dll|ninja: no work to do",
    re.I,
)
# Explicit failure markers of the REAL run. Mutation-twin output legitimately contains [FAIL] lines,
# so only unittest summaries and non-zero ctest failure counts count here; real-run counts rely on exit code.
NEGATIVE = re.compile(r"^FAILED \((failures|errors)=|\b[1-9]\d* tests failed out of|is not recognized as an internal", re.M)


# ---- --affected: run only the suites whose sources changed vs git HEAD (build-speed, docs/TOOLS-BRIEF.md) ----
# Sources that every C++ suite depends on: a change here selects all suites that compile C++.
SHARED_PREFIXES = (
    "synccore/src/", "synccore/include/", "plugin/common/", "plugin/hooks/", "tools/mut/", "tools/build/",
    "CMakeLists.txt", "plugin/CMakeLists.txt", "synccore/CMakeLists.txt", "third_party/", "docs/quest-policies/",
)
# Paths that never need a test run.
IGNORED_PREFIXES = ("docs/", "out/", "build/", "runs/", "run/", "out_flight_dir/", ".git/", "tools/__pycache__/")
# ...except these, which are TEST INPUT under an ignored prefix: rules.json is compiled into the synccore tests, the saved fuzz scenarios are replayed by T2b.
NOT_IGNORED_PREFIXES = ("docs/quest-policies/", "scenarios/simfuzz/")
# Extra dependencies by suite name (substring) for suites whose command does not reveal them.
EXTRA_DEPS = {
    "canonical nvcoop.dll build": ("plugin/", "synccore/", "tools/runner/build_l1.cmd"),
    "build-speed tooling": ("tools/test_all.py", "tools/mut/", "tools/build/"),
    "test_all preflight": ("tools/test_all.py", "tools/test_all_selftest.py", "plugin/", "synccore/"),   # scans the runner scripts of every suite
    "L4 synccore": ("synccore/",),
    "repo hygiene (": ("synccore/", "plugin/", "tools/lint/"),
    "T2b simfuzz": ("synccore/", "tools/simfuzz/", "scenarios/simfuzz/"),
    "NPC pick for": ("tools/actors/", "tools/animcheck/npc_proof.py"),   # the scripts that must use it (Wiring)
    "Enemy sync tests": ("tools/animcheck/", "tools/scenarios/", "scenarios/goodsprings_enemies.json"),   # thresholds, scenario validator, the scenario file
}


def _norm(p: str) -> str:
    p = p.replace("\\", "/")
    while p.startswith("./"):
        p = p[2:]
    return p


def suite_dirs(name: str, cmd: list) -> tuple:
    """Directories (prefixes) a suite owns, derived from its command line so new suites need no registration here."""
    dirs = []
    for tok in cmd:
        t = _norm(str(tok))
        if "/" not in t or t.startswith("/") or ":" in t:
            continue  # flags, plain words, absolute paths
        parts = t.split("/")
        if parts[0] in ("plugin", "synccore", "tools") and len(parts) >= 3 and "." not in parts[1]:
            dirs.append("/".join(parts[:2]) + "/")
        elif parts[0] == "tools" and len(parts) == 2 and "." in parts[1]:
            dirs.append(t)  # a script directly under tools/ (test_all_selftest.py): owns that file, not all of tools/
        else:
            dirs.append(parts[0] + "/")
    for key, extra in EXTRA_DEPS.items():
        if key in name:
            dirs.extend(extra)
    return tuple(dict.fromkeys(dirs))


def changed_files(base: str = "HEAD") -> list:
    out = []
    for cmd in (["git", "diff", "--name-only", base], ["git", "ls-files", "--others", "--exclude-standard"]):
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, errors="replace")
        out += [l.strip() for l in p.stdout.splitlines() if l.strip()]
    return sorted(set(out))


def select_affected(files=None, base: str = "HEAD") -> list:
    """Names of the suites whose sources are among `files` (default: changed vs git HEAD + untracked)."""
    files = changed_files(base) if files is None else files
    rel = [_norm(f) for f in files]
    rel = [f for f in rel if f.startswith(NOT_IGNORED_PREFIXES) or not f.startswith(IGNORED_PREFIXES)]
    if not rel:
        return []
    picked = []
    for name, cmd, compiles, _tools in SUITES:
        dirs = suite_dirs(name, cmd)
        hit = False
        for f in rel:
            if "/" not in f and f != "CMakeLists.txt":
                continue  # stray files in the repo root (scratch dirs, notes) never trigger a run
            known_root = f.startswith(("plugin/", "synccore/", "tools/", "scenarios/", "third_party/", "questlines/", "tests/")) or f == "CMakeLists.txt"
            if not known_root:
                hit = True  # unknown location: be safe and run everything
                break
            if any(f.startswith(d) for d in dirs) or (compiles and any(f.startswith(s) or f == s for s in SHARED_PREFIXES)):
                hit = True
                break
        if hit:
            picked.append(name)
    return picked
def preflight(quick: bool, names=None) -> dict:
    """Checks only the tools that the suites of this run need (`names`: the suites picked by --affected / --only; default all).
    Prints one line per tool. Returns tool -> problem."""
    if not WINDOWS and MUTANT != "no_platform_skip":
        return {}   # decide() skips every suite that needs the toolchain, with the reason; nothing to check here
    users = {}
    for name, _, slow, tools in SUITES:
        if names is not None and name not in names:
            continue
        if not (quick and slow):
            for t in required_tools(slow, tools):
                if t in ("cmake", "msvc"):  # an "-optional" tool never blocks a suite
                    users.setdefault(t, []).append(name)
    if not users:
        return {}
    problems = {}
    for tool, (found, problem) in toolchain().items():
        if tool not in users:
            continue
        problems[tool] = problem
        if problem:
            print(f"PREFLIGHT FAIL {tool}: {problem}. The {len(users[tool])} suites that need it are marked FAIL "
                  "without running.")
        else:
            print(f"PREFLIGHT OK {tool}: {found}")
    return problems


def main() -> int:
    quick = "--quick" in sys.argv
    only = [sys.argv[i + 1] for i, a in enumerate(sys.argv[:-1]) if a == "--only"]
    affected = None
    if "--affected" in sys.argv:
        base = "HEAD"
        if "--base" in sys.argv:
            base = sys.argv[sys.argv.index("--base") + 1]
        affected = select_affected(base=base)
        print(f"--affected vs {base}: {len(affected)} of {len(SUITES)} suites")
        for n in affected:
            print(f"  would run: {n}")
        if "--list" in sys.argv:
            return 0
    failed = []
    def picked(name):
        return (affected is None or name in affected) and (not only or any(o.lower() in name.lower() for o in only))

    problems = preflight(quick, [n for n, _c, _s, _t in SUITES if picked(n)])   # only the tools of the suites that will run
    for name, cmd, slow, tools in SUITES:
        if affected is not None and name not in affected:
            print(f"SKIP (unchanged) {name}")
            continue
        if only and not any(o.lower() in name.lower() for o in only):
            continue
        action, reason = decide(name, slow, tools, quick, problems)
        if action == "skip":
            print(f"SKIP {name}" + (f": {reason}" if reason else ""))
            continue
        if action == "fail":
            print(f"FAIL {name}: {reason}")
            failed.append(name)
            continue
        # The data check is decide()'s (missing_data). A second check here called NEEDS[name][0]() from an older NEEDS of lambdas, and
        # NEEDS now maps to file lists: with out/questdata present every data suite died on "'tuple' object is not callable".
        if MUTANT == "stale_needs_check":  # twin: the old check
            need = NEEDS.get(name)
            if need and not need[0]():
                print(f"SKIP {name}: {need[1]}")
                continue
        t0 = time.time()
        p = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, errors="replace")
        out = p.stdout + p.stderr
        # Exit code alone is not trusted: a test exe that never started can still exit 0.
        evidence = EVIDENCE.search(out)
        ok = p.returncode == 0 and evidence is not None and not NEGATIVE.search(out)
        if p.returncode == 0 and not ok:
            print(f"    (exit 0 but no proof tests ran, or a failure line was printed)")
        print(f"{'PASS' if ok else 'FAIL'} {name} ({time.time() - t0:.0f}s)")
        if not ok:
            failed.append(name)
            tail = (p.stdout + p.stderr).strip().splitlines()[-15:]
            print("    " + "\n    ".join(tail))
    print(f"\n{len(failed)} suite(s) failed" if failed else "\nALL SUITES PASS")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
