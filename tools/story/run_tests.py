"""One command for the story run: the offline tests, then the mutation twins. Prints ALL OK when everything holds.

    python -I tools/story/run_tests.py

A twin runs the tests with one deliberate bug in the RUNNER switched on (STORY_MUTANT=<name> for tools/story, LAB_MUTANT=<name> for the lab engine it reuses);
the tests that guard that rule must then FAIL. The bugs of the fake WORLD (tools/story/storyfake.py BUGS) are twins too: test_story.py checks that each one fails
its step at its check.
"""
from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PY = [sys.executable, "-I", "-X", "utf8"]
TEST = "tools/story/tests/test_story.py"

# name -> (env var, what the bug does, the tests that must notice it)
TWINS = {
    "checks_never_fail": ("LAB_MUTANT", "every check passes, whatever it compared", ["FakeStory.test_each_fake_bug_fails_its_step_at_its_check"]),
    "no_fail_fast": ("LAB_MUTANT", "a step goes on after its first failed check", ["FakeStory.test_each_fake_bug_fails_its_step_at_its_check"]),
    "no_time_box": ("LAB_MUTANT", "a step is not stopped by its time box", ["FakeStory.test_a_step_over_its_time_box_is_a_timeout_with_the_checks_so_far"]),
    "no_clean_rule": ("LAB_MUTANT", "a step with digest repairs still passes", ["FakeStory.test_each_fake_bug_fails_its_step_at_its_check"]),
    "host_does_all": ("STORY_MUTANT", "the client's actions are done by the host (the guest-initiated dialogue is never played by a guest)",
                      ["FakeStory.test_the_initiators_picks_are_recorded_and_the_client_leads_the_guest_steps"]),
    "picks_unrecorded": ("STORY_MUTANT", "the report does not list the initiator's picks", ["FakeStory.test_the_initiators_picks_are_recorded_and_the_client_leads_the_guest_steps"]),
    "divergence_blind": ("STORY_MUTANT", "the first divergence between the copies is never found", ["Views.test_first_diff_names_the_first_shared_value_that_differs",
                                                                                                    "FakeStory.test_each_fake_bug_fails_its_step_at_its_check"]),
    "summary_unbounded": ("STORY_MUTANT", "summary.md is not kept under 5 KB", ["Reports.test_summary_stays_under_5kb_even_with_long_texts_and_many_failures"]),
    "no_reach_check": ("STORY_MUTANT", "a walk that ends far away in 3D is taken as arrived (the conversation menu then never opens)", ["FakeStory.test_a_walk_that_ends_far_in_3d_is_corrected_before_the_talk"]),
    "boxes_not_cleared": ("STORY_MUTANT", "queued hint boxes are left up before a talk (they eat the input)", ["FakeStory.test_queued_message_boxes_are_closed_before_a_talk"]),
    "talky_takes_junk": ("STORY_MUTANT", "the talky fallback picks a line that ends the talk when nothing else is left", ["FakeStory.test_the_talky_fallback_never_picks_a_line_that_ends_the_talk"]),
    "take_checks_enabled": ("STORY_MUTANT", "a pickup is judged by the enabled flag, which a self-destroying plant keeps", ["FakeStory.test_a_harvested_plant_is_gone_by_its_own_destroyed_mark_and_its_start_state_has_it_standing"]),
    "kill_unweakened": ("STORY_MUTANT", "the story's hostiles are shot at full health, free to run up to the shooter (24 volleys left a gecko at health 11)", ["FakeStory.test_a_hostile_is_held_and_weakened_by_its_owner_before_it_is_shot"]),
    "no_settle_world": ("STORY_MUTANT", "a shot or a talk is made with the DLC load-message boxes up and the puppets still in the old cell", ["FakeStory.test_boxes_are_closed_and_puppets_settled_on_both_copies_before_a_shot"]),
    "stand_anywhere": ("STORY_MUTANT", "the shooter fires from wherever the walk left it (the other player's puppet in the line of fire takes the hits)", ["FakeStory.test_the_shooter_stands_on_the_far_side_of_the_target_from_the_other_player"]),
    "item_console_plugin_id": ("STORY_MUTANT", "the item count is read with the Plugin:ID in the console line (a syntax error in the game)", ["LiveItemCount.test_item_count_reads_the_item_by_its_hex_runtime_id"]),
    "no_questdata": ("STORY_MUTANT", "the INFO ids are not checked against the game data", ["Script.test_the_script_is_valid_against_the_plan_and_the_game_data"]),
}


def run(args, env_extra=None, timeout=900):
    env = dict(os.environ)
    env.update(env_extra or {})
    return subprocess.run(PY + args, cwd=ROOT, capture_output=True, text=True, env=env, timeout=timeout)


def main() -> int:
    bad = 0
    cp = run([TEST])
    tail = (cp.stdout + cp.stderr).strip().splitlines()[-3:]
    print(f"unit tests {Path(TEST).name}:", " | ".join(tail))
    if cp.returncode:
        print((cp.stdout + cp.stderr)[-4000:])
        bad += 1
    for name, (var, what, tests) in TWINS.items():
        if name == "no_questdata" and not (ROOT / "out" / "questdata" / "infos.jsonl.gz").is_file():
            print(f"twin {name:18s} skipped: out/questdata is not there (the check it guards needs it)")
            continue
        t = run([TEST] + tests, {var: name})
        caught = t.returncode != 0
        print(f"twin {name:18s} {'caught' if caught else 'NOT CAUGHT'}   ({what})")
        if not caught:
            bad += 1
    print("RESULT: " + (f"{bad} problem(s)" if bad else "ALL OK"))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
