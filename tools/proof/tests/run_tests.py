"""One command for the proof harness: manifest check, unit tests, then the mutation twins. Prints ALL OK when everything holds.

    python -I tools/proof/tests/run_tests.py

A twin runs the unit tests with one deliberate bug switched on (PROOF_MUTANT=<name>); the tests must FAIL, otherwise they would not notice that bug.
"""
from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent.parent
PY = [sys.executable, "-I", "-X", "utf8"]

# name -> (what the bug does, the unit tests that must notice it)
TWINS = {
    "repro_is_test": ("a classifier that labels a REPRODUCED sync failure as TEST", ["Classify", "Pipeline.test_reproduced_failure_is_routed_as_sync"]),
    "relpath_cross_drive": ("an instance folder on another drive than the checkout (CI: D: and C:) kills the run on relpath", ["CrossDrive"]),
    "packet_oversize": ("a failure packet that ignores the 3 KB limit", ["Packet"]),
    "kill_foreign": ("kill_owned stops a pid that prove.py did not start", ["Safety"]),
    "no_clean_counters": ("a PASS with digest repairs or area heals still counts as clean", ["Classify", "Pipeline.test_pass_with_digest_repairs_is_not_clean"]),
    "ledger_no_fresh": ("proven without any run from a fresh boot", ["Ledger"]),
    "foreign_exe_ignored": ("the owner's FalloutNV.exe (outside the test instance folders) does not stop the queue", ["Safety"]),
    "steam_not_checked": ("Steam not running does not stop the queue", ["Safety"]),
    "lock_ignored": ("a second queue runner is allowed while the first one lives", ["Safety"]),
    "flaky_as_pass": ("FAIL then PASS on the rerun is reported as PASS, not FLAKY", ["Pipeline.test_fail_then_pass_is_flaky"]),
    "pair_overlap": ("two tests that run at the same time are given the same pair", ["Sched", "Parallel"]),
    "exclusive_not_alone": ("a three-copy test starts beside a running pair (or a pair beside it)", ["Sched", "Parallel"]),
    "pair_missing": ("the verdict line does not say which pair ran the test", ["Parallel", "Pipeline.test_pass_prints_one_json_line_and_appends_ledger"]),
    "ram_reserve_ignored": ("the RAM guard lets 4 copies start without the reserve left free", ["Ram"]),
    "watcher_unscoped": ("a copy that died on pair 1 fails the test that runs on pair 0", ["WatcherScope"]),
    "watcher_forgets_exit": ("the second crash check of one dead copy reports exit -1 (off Windows waitpid has reaped it)", ["WatcherScope"]),
    "kill_runner_only": ("off Windows, a runner stopped on a timeout leaves the child it started running", ["PosixProcessHelpers"]),
    "zombie_alive": ("off Windows, an exited copy that was not reaped yet counts as still running", ["PosixProcessHelpers"]),
    "helpers_env_missing": ("a copy prove.py boots does not get NVCOOP_TEST_HELPERS=1", ["BootEnv"]),
    "env_reuse_unchecked": ("a running copy is reused although it was started without the env the test needs", ["BootEnv"]),
    "slot_need_fixed": ("the game slot is asked for a fixed 3 GB instead of the caller's own need", ["SlotRam"]),
    "steam_signin_ignored": ("a Steam that runs but is signed out passes the preflight", ["SteamSignedIn"]),
    "steam_error_ignored": ("a booted copy showing a 'Steam Error' window is waited for 300 s", ["SteamSignedIn"]),
    "wiring_unchecked": ("a test may provision other live.* lanes than its wiring command expects", ["ManifestSetup"]),
    "npc_load_unchecked": ("a test that reads a live NPC may skip loading the scenario first", ["ManifestSetup"]),
    "world_load_unchecked": ("a test whose script needs the player in a cell may skip loading the scenario first (avatar runtime never starts)", ["ManifestSetup.test_a_world_test_without_the_scenario_or_the_mark_is_reported"]),
    "weapon_edid_unchecked": ("a VATS weapon param may be an editor id the bridge console cannot resolve", ["ManifestSetup.test_the_vats_weapons_are_form_ids_not_editor_ids"]),
    "adopt_foreign": ("a game copy of another lane or the owner's own game is adopted as ours when owned.json is rebuilt", ["OrphanCopies.test_a_copy_of_another_lane_and_the_owners_own_game_are_left_alone"]),
    "orphans_not_adopted": ("copies our lane launched are not taken back when owned.json is lost: BLOCKED instead of re-provisioned", ["OrphanCopies.test_a_running_copy_with_the_wrong_ini_is_stopped_reprovisioned_and_rebooted_although_owned_json_was_lost"]),
    "round_ignored": ("a restarted queue with --round starts from the top", ["RetestMode.test_a_restarted_round_skips_the_tests_that_have_a_verdict_in_it", "RetestMode.test_round_done_rules"]),
    "coop_unchecked": ("a game test may leave out coop.enable (two copies BLOCKED on copies a one-copy test left at 0)", ["ManifestSetup.test_a_two_copy_test_without_coop_enable_is_flagged"]),
    "short_unmarked": ("a queue --runs below N records its passes as if they were full runs", ["Queue.test_queue_runs_overrides_n_and_the_short_passes_never_prove"]),
    "ledger_short_counts": ("a short pass counts toward proven", ["Ledger.test_short_passes_never_count_toward_proven_and_status_says_so"]),
    "queue_runs_resume_blind": ("a restarted queue with --runs forgets the clean runs already on the ledger", ["Queue.test_queue_with_runs_resumes_where_it_stopped", "Queue.test_queue_resumes_across_different_runs"]),
    "queue_runs_ignored_proven": ("a queue --runs runs tests that are proven already", ["Queue.test_queue_runs_skips_proven_tests"]),
    "provision_default_off": ("queue and run do not re-provision a copy whose nvcoop.ini lacks the test switches unless --provision is given", ["Queue.test_queue_and_run_provision_by_default_and_take_runs"]),
    "ram_halts_queue": ("a test the RAM guard refuses stops the whole queue (exit 4) instead of being recorded as ENV and retried at the end", ["Parallel.test_ram_guard_for_one_test_does_not_halt_the_queue", "Parallel.test_ram_guard_refusal_is_retried_at_the_end_and_can_pass"]),
    "spawn_base_unchecked": ("the spawn test may roll with an editor id the bridge console cannot resolve", ["ManifestSetup"]),
    "stub_shared_log": ("two stub runners append to one log file and lose lines when they start together", ["Parallel.test_stubs_that_start_together_each_keep_their_own_log"]),
    "build_unchecked": ("a copy of pair 1 with another nvcoop_mod.dll than pair 0 is run (and fails as a test, 'bridge method missing')", ["PairBuildAndSaves.test_a_pair_one_copy_with_another_core_is_a_build_problem", "PairBuildAndSaves.test_a_missing_core_a_stale_core_and_a_loader_that_would_not_load_it"]),
    "ready_skipped": ("a copy is used as soon as its pipe is up, before the game serves requests", ["PairBuildAndSaves.test_a_copy_that_does_not_serve_requests_yet_is_env_and_is_not_asked_for_methods"]),
    "saves_not_seeded": ("the copies of pair 1 start a scenario with an empty Saves folder", ["PairBuildAndSaves.test_prepare_seeds_the_saves_of_every_copy", "PairBuildAndSaves.test_an_empty_copy_gets_the_save_family_of_a_donor_of_its_own_role"]),
}
if os.name == "nt" or not os.path.isdir("/proc/self"):   # their tests are the POSIX branches: PosixProcessHelpers skips here
    del TWINS["zombie_alive"], TWINS["kill_runner_only"]


def run(args, env_extra=None, timeout=900):
    env = dict(os.environ)
    env.update(env_extra or {})
    return subprocess.run(PY + args, cwd=ROOT, capture_output=True, text=True, env=env, timeout=timeout)


def main() -> int:
    bad = 0
    cp = run(["tools/proof/prove.py", "check"])
    print(cp.stdout.strip())
    if cp.returncode:
        bad += 1
    cp = run(["tools/proof/tests/test_proof.py"])
    tail = (cp.stdout + cp.stderr).strip().splitlines()[-3:]
    print("unit tests:", " | ".join(tail))
    if cp.returncode:
        print((cp.stdout + cp.stderr)[-3000:])
        bad += 1
    for name, (what, tests) in TWINS.items():
        t = run(["tools/proof/tests/test_proof.py"] + tests, {"PROOF_MUTANT": name})
        caught = t.returncode != 0
        print(f"twin {name:20s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    if bad:
        print(f"RESULT: {bad} problem(s)")
        return 1
    print("RESULT: ALL OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
