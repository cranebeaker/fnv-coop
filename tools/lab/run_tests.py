"""One command for the lab runner: the offline tests, then the mutation twins. Prints ALL OK when everything holds.

    python -I tools/lab/run_tests.py

A twin runs the tests with one deliberate bug in the RUNNER switched on (LAB_MUTANT=<name>); the tests that guard that rule must then FAIL.
The bugs of the fake WORLD (tools/lab/labfake.py BUGS, 43 of them) are twins too: test_lab.py checks that each one fails its room at its check,
so a room that stopped noticing one would fail the unit tests directly.
"""
from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PY = [sys.executable, "-I", "-X", "utf8"]
TEST = "tools/lab/tests/test_lab.py"
TEST_NET = "tools/lab/tests/test_net.py"         # the network pass (lab.py run --net ...)
TEST_SPLIT = "tools/lab/tests/test_split.py"     # the two-PC smoke (lab.py run --split)
TEST_PARITY = "tools/lab/tests/test_parity.py"   # the parity room: lab_parity.py, the room on the fake world, the live reads, the probe source

# name -> (what the bug does, the tests that must notice it)
TWINS = {
    "checks_never_fail": ("every check passes, whatever it compared", ["FakeLab.test_each_bug_fails_its_room_at_its_check"]),
    "no_fail_fast": ("a room goes on after its first failed check", ["FakeLab.test_each_bug_fails_its_room_at_its_check"]),
    "no_time_box": ("a room is not stopped by its time box", ["Boxes.test_a_room_over_its_time_box_is_a_timeout_with_the_checks_so_far"]),
    "no_run_budget": ("the whole-run budget is ignored by rooms that have not started", ["Boxes.test_the_run_budget_stops_the_rooms_that_have_not_started",
                                                                                         "Boxes.test_a_room_never_runs_longer_than_what_is_left_of_the_run"]),
    "no_clean_rule": ("a PASS with digest repairs still passes", ["FakeLab.test_each_bug_fails_its_room_at_its_check", "FakeLab.test_counters_that_moved_are_listed"]),
    "summary_unbounded": ("summary.md is not kept under 5 KB", ["Reports.test_the_summary_stays_under_5kb_even_with_long_texts", "Reports.test_summary_is_short_and_has_the_table_the_blocks_and_the_comparison"]),
    "digest_wait_unbounded": ("spawn waits 30 s for a first ok digest even after one was judged bad", ["FakeLab.test_a_digest_that_never_agrees_fails_spawn_fast_with_the_divergence"]),
    "enable_all_at_once": ("the fight room enables all its hostiles together again", ["FakeLab.test_the_fight_room_enables_one_hostile_at_a_time_so_none_walks_up_to_the_shooter"]),
    "no_weaken": ("the fight room shoots its targets at full health again", ["FakeLab.test_the_fight_room_weakens_each_target_through_the_host_before_it_is_shot"]),
    "vats_compares_kill_health": ("the VATS room compares the killing shot's health on both copies (host -14, client 1)", ["FakeLab.test_a_vats_kill_that_reads_minus_14_on_the_host_and_1_on_the_client_passes"]),
    "menus_left_open": ("a menu an earlier step left open (the lockpick menu of a locked door that was activated) stays open and eats the next room's input", ["FakeLab.test_a_menu_an_earlier_step_left_open_is_closed_before_the_next_room_starts"]),
    "vats_closed_unchecked": ("a VATS job that failed with the camera closed counts as a shot without looking at the target", ["Live.test_a_vats_job_that_lost_the_race_to_the_modules_auto_fire_counts_when_the_target_was_hit"]),
    "host_pickup_via_bridge": ("the host picks up through activate.ref again (the engine gives it nothing for a placed Stimpak)", ["Live.test_the_host_picks_up_through_the_console_and_a_guest_through_the_bridge_method"]),
    "npc_step_fixed_limit": ("an NPC step between two samples is judged by a fixed limit, not by what the patrol covers in that time", ["FakeLab.test_a_patrol_step_between_two_slow_samples_is_no_snap_but_a_real_snap_still_is"]),
    "vats_no_cooldown": ("VATS entries follow each other without a pause (the game crashed in the first ticks of such an entry)", ["Live.test_two_vats_entries_close_together_wait_for_the_cooldown"]),
    "vats_target_free": ("the VATS room leaves its target free: it walks up to the shooter and VATS cannot enter", ["FakeLab.test_the_vats_room_holds_each_target_in_place_so_none_walks_up_to_the_shooter"]),
    "fps_p5_is_mean": ("the lab reports the mean fps as the p5 too, so a stalling copy looks smooth", ["Fps.test_p5_is_the_speed_of_the_slowest_5_percent_of_the_time_not_of_the_samples"]),
    "fps_copies_dropped": ("the number of copies running is not reported with the fps", ["Fps.test_every_room_reports_its_fps_per_copy_and_the_copies_running"]),
    "fps_unsampled": ("a room reads the frame counters only at its start and its end", ["Fps.test_every_room_reports_its_fps_per_copy_and_the_copies_running"]),
    "fight_target_free": ("the fight room leaves its target free and it drifts out of the pistol's range (lab run 2026-10-28, bad_wifi and transatlantic)", ["FakeLab.test_the_fight_room_holds_each_target_in_place_so_none_drifts_out_of_the_pistols_range"]),
    "shooter_hops_to_far_stand": ("a shooter hops to the stand marker again although the target drifted 659 units from it", ["Live.test_a_target_that_drifted_out_of_range_is_walked_up_to_along_the_line_not_the_stand_marker_again"]),
    "guest_take_menu_opened": ("a guest's take moves the item with the container menu open (the loot service heals it away at the close: guest 2, host 3)", ["Live.test_a_guest_takes_through_the_helper_with_the_container_closed_like_a_quick_loot_mod"]),
    "quest_var_cached": ("the quest variable is read once and the first value is kept", ["Live.test_a_quest_variable_is_read_through_the_bridge_on_each_copy_and_an_unreadable_one_says_why"]),
    "npc_handoff_waits_for_both": ("the NPC handoff time waits for the old owner's copy too, so it includes that copy's cell load (1030 ms in the lab run of 2026-10-28)", ["FakeLab.test_the_npc_handoff_time_does_not_include_the_length_of_the_old_owners_cell_load"]),
    "shoot_no_controls_retry": ("a holstered weapon after a talk is not helped by switching the player's controls on (story run 20261028-023656, S05 and S15)", ["Live.test_a_holstered_weapon_after_a_talk_gets_the_controls_back_and_is_drawn_before_the_shot"]),
    "lag_est_ignores_read_times": ("the playback delay is estimated with the walker's read time for the puppet's reading too, so the bridge read skew counts as lag", ["Pure.test_lag_estimate_finds_the_playback_delay_whatever_the_two_bridge_reads_took"]),
    "trace_lag_wrong_sign": ("the trace compares the puppet with where the walker will be, so a delay of 140 ms reads as a negative one", ["Pure.test_trace_lag_finds_the_delay_on_one_clock_and_the_engine_lag_behind_it"]),
    "stage_transport_sum": ("the wire time is the viewer's rx plus the walker's send age instead of the difference", ["Pure.test_stage_metrics_are_leg_means_and_the_wire_time_is_the_difference"]),
    "compare_blind": ("the comparison with the previous run shows nothing", ["Reports.test_write_run_makes_both_files_and_compares_with_the_newest_earlier_run"]),
}
# the twins of the network pass, in TEST_NET
NET_TWINS = {
    "net_clean_limits": ("a chaos profile is judged by the clean limits, not its own", ["FakeNet.test_a_profile_is_judged_by_its_own_limits_not_the_clean_ones"]),
    "net_pass_skipped": ("--net is accepted and the profiles never run", ["FakeNet.test_the_network_pass_runs_at_all"]),
    "crash_blocks_profiles": ("a game copy that exited in the clean pass blocks every profile pass of its pair again", ["FakeNet.test_a_crash_in_the_clean_pass_does_not_blank_the_profile_passes_of_its_pair"]),
    "net_bypass_unchecked": ("a client that never crossed the relay is accepted as joined through it", ["LiveNet.test_a_client_that_never_crossed_the_relay_blocks_the_pass"]),
}
# the twins of the parity room, in TEST_PARITY
PARITY_TWINS = {
    "parity_skips_equipment": ("equipment (worn items, the weapon in hand) is never compared", ["Compare.test_a_settler_holding_nothing_while_the_owner_holds_a_rifle",
                                                                                              "Room.test_each_parity_bug_names_the_entity_the_field_both_values_and_the_source"]),
    "parity_anim_unchecked": ("the animation categories of the two copies are never compared", ["Compare.test_animation_categories_are_compared_as_sets_over_the_rounds",
                                                                                                "Room.test_each_parity_bug_names_the_entity_the_field_both_values_and_the_source"]),
    "parity_time_unchecked": ("the global time multiplier of a copy is never checked (the guest at 30x game speed)", ["Compare.test_the_time_multiplier_must_be_one_on_every_copy_outside_vats",
                                                                                                                       "Room.test_each_parity_bug_names_the_entity_the_field_both_values_and_the_source"]),
    "parity_attack_unchecked": ("a puppet that does not punch is accepted", ["Compare.test_the_punch_that_did_not_play_on_the_puppet",
                                                                              "Room.test_each_parity_bug_names_the_entity_the_field_both_values_and_the_source"]),
    "parity_no_settle": ("the room looks once and does not give a change time to cross the link", ["Room.test_the_fake_world_passes_and_every_step_ran"]),
    "parity_owner_ignored": ("the scripted lines go to the host although the client owns the NPC", ["Room.test_when_the_client_owns_the_npc_the_scripted_lines_go_to_the_client"]),
}
# the twins of the two-PC smoke, in TEST_SPLIT
SPLIT_TWINS = {
    "split_ram_unchecked": ("the free RAM of this PC is not checked before the local copy starts", ["Guards.test_under_three_gb_free_is_refused_with_a_clear_message",
                                                                                               "Refusals.test_low_ram_refuses_before_anything_is_pushed_started_or_stopped"]),
    "split_steam_unchecked": ("Steam is not checked on this PC", ["Guards.test_steam_not_running_here_is_a_reason_and_the_lab_does_not_start_it"]),
    "split_stops_unstarted": ("a refused run still stops the second PC's copies", ["Refusals.test_finish_after_a_refusal_does_not_stop_anything_on_the_second_pc"]),
}


def run(args, env_extra=None, timeout=900):
    env = dict(os.environ)
    env.update(env_extra or {})
    return subprocess.run(PY + args, cwd=ROOT, capture_output=True, text=True, env=env, timeout=timeout)


def main() -> int:
    bad = 0
    for f in (TEST, TEST_NET, TEST_SPLIT, TEST_PARITY):
        cp = run([f])
        tail = (cp.stdout + cp.stderr).strip().splitlines()[-3:]
        print(f"unit tests {Path(f).name}:", " | ".join(tail))
        if cp.returncode:
            print((cp.stdout + cp.stderr)[-4000:])
            bad += 1
    for name, (what, tests) in TWINS.items():
        t = run([TEST] + tests, {"LAB_MUTANT": name})
        caught = t.returncode != 0
        print(f"twin {name:18s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    for name, (what, tests) in NET_TWINS.items():
        t = run([TEST_NET] + tests, {"LAB_MUTANT": name})
        caught = t.returncode != 0
        print(f"twin {name:18s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    for name, (what, tests) in PARITY_TWINS.items():
        t = run([TEST_PARITY] + tests, {"LAB_MUTANT": name})
        caught = t.returncode != 0
        print(f"twin {name:18s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    for name, (what, tests) in SPLIT_TWINS.items():
        t = run([TEST_SPLIT] + tests, {"LAB_MUTANT": name})
        caught = t.returncode != 0
        print(f"twin {name:18s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    print("RESULT: " + (f"{bad} problem(s)" if bad else "ALL OK"))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
