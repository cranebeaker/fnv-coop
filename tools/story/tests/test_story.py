"""Offline tests of the story run (no game): the script against the plan, the steps on the fake two-copy Goodsprings, one test per fake bug, the time boxes,
fail fast, the reports.

    python -I tools/story/tests/test_story.py        # all of it
    python -I tools/story/run_tests.py               # these tests, then the mutation twins (STORY_MUTANT / LAB_MUTANT=<name>: each must make a test fail)
"""
from __future__ import annotations

import copy
import json
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
STORY = HERE.parent
ROOT = STORY.parent.parent
sys.path.insert(0, str(STORY))
sys.path.insert(0, str(ROOT / "tools" / "lab"))

import lab_core as K              # noqa: E402,F401
import story                      # noqa: E402
import story_core as C            # noqa: E402
import story_data as D            # noqa: E402
import story_report as R          # noqa: E402
import storyfake                  # noqa: E402

SCRIPT = D.load_script()
PLAN = D.load_plan()
IDS = [s["id"] for s in SCRIPT.steps]

# fake bug -> (the first step that must fail, a text its first failed check must contain)
EXPECT = {
    "start_unsynced": ("S03", "client shows VCG01 stage 200"),
    "stage_unsynced": ("S03", "client shows VCG01 stage 200"),
    "slow_stage": ("S03", "client shows VCG01 stage 200"),
    "stage_rollback": ("S03", "other copy shows it too"),
    "objective_unsynced": ("S03", "agree on quest state"),
    "guest_talk_vetoed": ("S03", "dialogue opens"),
    "guest_lines_missing": ("S03", "offers its lines"),
    "guest_pick_lost": ("S03", "reaches the goal on one copy"),
    "guest_result_twice": ("S16", "exactly once"),
    "reward_host_only": ("S16", "exactly once"),
    "rep_unsynced": ("S16", "agree on quest state"),
    "pickup_unsynced": ("S07", "is gone on both copies"),
    "kill_unseen": ("S05", "is dead on both copies"),
    "barrier_open": ("S02", "has NOT fired yet"),
    "digest_noise": ("S03", "no digest repair"),
}


# the start-state sync bugs also break the exit scene of S02 first, so they are tried on S03 alone (S00 loads the save)
ONLY = {"start_unsynced": "S03", "stage_unsynced": "S03", "slow_stage": "S03"}


def run_fake(bugs=(), steps=None, script=None, budget_s=3000.0, stop_on_fail=False, params=None):
    script = script or SCRIPT
    env = storyfake.FakeStoryEnv(script, PLAN, bugs)
    return story.execute(script, env, steps or list(script.steps), dict(params or C.PARAMS), budget_s, stop_on_fail, "t", "fake", lambda s: None)


def first_failed(row):
    return next((c for c in row["checks"] if not c["ok"]), None)


class Script(unittest.TestCase):
    def test_the_script_is_valid_against_the_plan_and_the_game_data(self):
        self.assertEqual(D.script_problems(SCRIPT, PLAN), [])
        if (ROOT / "out" / "questdata" / "infos.jsonl.gz").is_file():
            self.assertTrue(D.infos_table(), "out/questdata is there, so the INFO ids must be looked up in it")

    def test_the_host_and_the_client_both_lead_steps_and_guest_dialogue_is_marked(self):
        who = {s["who"] for s in SCRIPT.steps}
        self.assertTrue({"host", "client"} <= who)
        guest = [s for s in SCRIPT.steps if s.get("guest_dialogue")]
        self.assertGreaterEqual(len(guest), 5)
        for s in guest:
            self.assertTrue(any(a["a"] in ("pick", "talk") and a["who"] == "client" for a in s["do"]), s["id"])

    def test_it_covers_the_three_quests_the_gunfight_and_the_freeform_quest(self):
        quests = {s["quest"] for s in SCRIPT.steps}
        self.assertTrue({"VCG01", "VCG02", "VCG03", "VMS16", "VFreeformGoodsprings"} <= quests)

    def test_a_broken_script_is_reported(self):
        bad = copy.deepcopy(SCRIPT.data)
        bad["steps"][3]["goal"]["stages"]["VCG02"] = 77
        bad["steps"][4]["box_s"] = 5
        bad["steps"][5]["do"][0]["a"] = "teleport"
        probs = D.script_problems(D.Script(bad), PLAN)
        self.assertTrue(any("stage 77" in p for p in probs), probs)
        self.assertTrue(any("box_s" in p for p in probs), probs)
        self.assertTrue(any("unknown action" in p for p in probs), probs)

    def test_an_info_the_plan_ties_to_another_stage_is_reported(self):
        bad = copy.deepcopy(SCRIPT.data)
        bad["npcs"]["sunny"]["topics"][0]["sets"] = ["VCG02", 25]
        self.assertTrue(any("plan says" in p for p in D.script_problems(D.Script(bad), PLAN)))


class Views(unittest.TestCase):
    def snap(self, stage=10, obj=None, karma=0.0, xp=100, inv=None, fame=15.0):
        q = {"currentStage": stage, "running": True, "completed": False, "failed": False, "completedStages": [5, stage], "objectives": obj or {}}
        return {"shared": {"quests": {"FalloutNV.esm:10A214": q}, "reputations": {"FalloutNV.esm:104C22": {"fame": fame, "infamy": 0}}, "karma": karma},
                "perPlayer": {"local": {"actorValues": {"Karma": {"current": karma}, "XP": {"current": xp}}, "inventory": {k: {"count": v} for k, v in (inv or {}).items()}, "caps": 5}}}

    def test_view_reads_stage_objectives_karma_reputation_xp_items(self):
        obj = {"30": {"displayed": True, "completed": True}, "40": {"displayed": True, "completed": False}, "50": {"displayed": False, "completed": False}}
        v = D.view_of(self.snap(obj=obj, inv={"FalloutNV.esm:13B2BA": 1}), SCRIPT)
        q = v["quests"]["VCG02"]
        self.assertEqual((q["stage"], q["obj"]), (10, {"30": "dc", "40": "d"}))
        self.assertEqual((v["xp"], v["caps"], v["karma"], v["rep"]["Goodsprings"]), (100, 5, 0.0, [15.0, 0.0]))
        self.assertEqual(v["inv"], {"FalloutNV.esm:13B2BA": 1})

    def test_first_diff_names_the_first_shared_value_that_differs(self):
        a, b = D.view_of(self.snap(10), SCRIPT), D.view_of(self.snap(5), SCRIPT)
        d = D.first_diff(a, b)
        self.assertEqual((d["key"], d["host"], d["client"]), ("VCG02.stage", 10, 5))
        self.assertIsNone(D.first_diff(a, D.view_of(self.snap(10, xp=999, inv={"x:1": 3}), SCRIPT)), "xp and items are per player")
        self.assertEqual(D.first_diff(a, D.view_of(self.snap(10, fame=18.0), SCRIPT))["key"], "reputation Goodsprings [fame, infamy]")
        self.assertEqual(D.first_diff(a, D.view_of(self.snap(10, karma=5.0), SCRIPT))["key"], "karma")
        o = D.first_diff(D.view_of(self.snap(10, obj={"30": {"displayed": True}}), SCRIPT), a)
        self.assertEqual(o["key"], "VCG02.objective 30")

    def test_delta_reports_what_one_copy_gained(self):
        b = D.view_of(self.snap(10, xp=100), SCRIPT)
        a = D.view_of(self.snap(25, obj={"20": {"displayed": True}}, xp=150, inv={"FalloutNV.esm:13B2BA": 2}, karma=1.0, fame=18.0), SCRIPT)
        d = D.delta(b, a)
        self.assertEqual(d["stages"]["VCG02"][:2], [10, 25])
        self.assertEqual((d["xp"], d["karma"], d["objectives"], d["items"]), (50, 1.0, {"VCG02:20": "d"}, {"FalloutNV.esm:13B2BA": 2}))
        self.assertEqual(d["rep"], {"Goodsprings": [3.0, 0.0]})


class FakeStory(unittest.TestCase):
    def test_the_clean_story_passes_every_step_and_reports_both_copies(self):
        res = run_fake()
        self.assertEqual([r["verdict"] for r in res["steps"]], ["PASS"] * len(IDS), [(r["room"], r.get("reason")) for r in res["steps"] if r["verdict"] != "PASS"])
        s16 = next(r for r in res["steps"] if r["room"] == "S16")
        self.assertEqual(s16["story"]["delta"]["host"].get("xp"), 50)
        self.assertEqual(s16["story"]["delta"]["client"].get("xp"), 50)
        self.assertEqual(s16["story"]["after"]["host"]["stages"]["VMS16"], 100)
        self.assertEqual(s16["story"]["after"]["client"]["stages"]["VMS16"], 100)
        self.assertIsNone(s16["story"]["divergence"])
        self.assertIn("stage_ms", s16["story"])

    def test_the_initiators_picks_are_recorded_and_the_client_leads_the_guest_steps(self):
        res = run_fake()
        for r in res["steps"]:
            if r["guest_dialogue"]:
                picks = r["story"].get("picks", [])
                if not any(a["a"] == "pick" for a in SCRIPT.by_id[r["room"]]["do"]):
                    self.assertIn("client", r["story"]["talk_ms"], r["room"])
                    continue
                self.assertTrue(picks, r["room"])
                self.assertTrue(all(p["who"] == "client" and p["said"] for p in picks), r["room"])
        s03 = next(r for r in res["steps"] if r["room"] == "S03")
        self.assertEqual(s03["story"]["picks"][0]["info"], "FalloutNV.esm:10A1E4")
        self.assertEqual(s03["story"]["picks"][0]["how"], "script")

    def test_items_go_to_the_player_who_picked_them_up(self):
        res = run_fake()
        s07 = next(r for r in res["steps"] if r["room"] == "S07")
        self.assertEqual(s07["story"]["delta"]["client"]["items"], {"FalloutNV.esm:13B2BA": 1})
        self.assertNotIn("items", s07["story"]["delta"]["host"])

    def test_start_states_forced_by_console_are_listed_per_step(self):
        res = run_fake()
        s03 = next(r for r in res["steps"] if r["room"] == "S03")
        self.assertIn("setstage VCG02 5", s03["story"]["prep"])
        self.assertIn("startquest VCG02", s03["story"]["prep"])
        self.assertEqual(next(r for r in res["steps"] if r["room"] == "S01")["story"].get("prep"), [], "S01 is the saved start state: nothing is forced")

    def test_a_walk_that_ends_far_in_3d_is_corrected_before_the_talk(self):
        # the real game: the walk says ok at its 2D radius, the player stands 247 units from Doc in 3D and the engine opens no menu from there
        res = run_fake(["walk_stops_far"])
        self.assertEqual([r["verdict"] for r in res["steps"]], ["PASS"] * len(IDS), [(r["room"], r.get("reason")) for r in res["steps"] if r["verdict"] != "PASS"])
        s03 = next(r for r in res["steps"] if r["room"] == "S03")
        walk = next(w for w in s03["story"]["walks"] if w["ref"] == SCRIPT.refs["sunny"])
        self.assertTrue(walk.get("teleported"), walk)
        self.assertGreater(walk["dist3d"], 200, walk)
        self.assertTrue(any("in 3D" in n for n in s03["story"].get("notes", [])), s03["story"].get("notes"))
        clean = run_fake()                                                   # a walk that really arrives is not replaced by a teleport
        w0 = next(w for w in next(r for r in clean["steps"] if r["room"] == "S03")["story"]["walks"] if w["ref"] == SCRIPT.refs["sunny"])
        self.assertFalse(w0.get("teleported"), w0)

    def test_a_hostile_is_held_and_weakened_by_its_owner_before_it_is_shot(self):
        # the real game: a free full-health gecko left 24 volleys at health 11, 53 units away (S05 run 20261028-104118); the lab holds the target and sets its health low first
        res = run_fake(["tough_geckos"], steps=story.pick_steps(SCRIPT, "S05"))
        s05 = next(r for r in res["steps"] if r["room"] == "S05")
        self.assertEqual(s05["verdict"], "PASS", (s05.get("reason"), [c for c in s05.get("checks", []) if not c.get("ok", True)][:2]))
        names = [c.get("name", "") for c in s05.get("checks", [])]
        self.assertEqual(sum(1 for n in names if n.startswith("the host weakens ")), 2, names)
        self.assertEqual(sum(1 for n in names if n.startswith("the host holds ")), 2, names)

    def test_boxes_are_closed_and_puppets_settled_on_both_copies_before_a_shot(self):
        # the real game: the DLC load messages pause the avatar layer of the host, which then judges the client's shots against a puppet still in Doc's house (S05, run 20261028-113423)
        res = run_fake(["dlc_boxes"], steps=story.pick_steps(SCRIPT, "S05"))
        s05 = next(r for r in res["steps"] if r["room"] == "S05")
        self.assertEqual(s05["verdict"], "PASS", (s05.get("reason"), first_failed(s05)))
        self.assertTrue(any("message box(es) closed on the two copies" in n for n in s05["story"].get("notes", [])), s05["story"].get("notes"))

    def test_the_shooter_stands_on_the_far_side_of_the_target_from_the_other_player(self):
        # the real game: 19 of the client's 21 hits landed on the host's puppet next to the first gecko (S05, run 20261028-113423)
        spot = C.opposite_spot((1000.0, 0.0, 5.0), (0.0, 0.0, 0.0), 300.0)
        self.assertEqual(spot, (1300.0, 0.0, 5.0))                          # beyond the target, away from the other player
        spot = C.opposite_spot((10.0, 10.0, 0.0), (10.0, 10.0, 0.0), 300.0)  # the other player stands on the target: any direction, still 300 away
        self.assertAlmostEqual(spot[0], 310.0)
        res = run_fake(["host_in_line"], steps=story.pick_steps(SCRIPT, "S05"))
        s05 = next(r for r in res["steps"] if r["room"] == "S05")
        self.assertEqual(s05["verdict"], "PASS", (s05.get("reason"), first_failed(s05)))
        self.assertTrue(any("stood on the far side" in n for n in s05["story"].get("notes", [])), s05["story"].get("notes"))

    def test_queued_message_boxes_are_closed_before_a_talk(self):
        res = run_fake(["boxes_queued"])
        self.assertEqual([r["verdict"] for r in res["steps"]], ["PASS"] * len(IDS), [(r["room"], r.get("reason")) for r in res["steps"] if r["verdict"] != "PASS"])
        s03 = next(r for r in res["steps"] if r["room"] == "S03")
        self.assertTrue(any("message box" in n for n in s03["story"].get("notes", [])), s03["story"].get("notes"))

    def test_the_talky_fallback_never_picks_a_line_that_ends_the_talk(self):
        offered = [{"idx": 0, "info": "FalloutNV.esm:10A1E3"}, {"idx": 1, "info": "FalloutNV.esm:15F531"}]       # [END TUTORIAL] ... and Goodbye
        pick, how = C._choose(offered, [], set(), "talky", SCRIPT)
        self.assertIsNone(pick)
        pick, how = C._choose(offered, [], set(), "first", SCRIPT)                                              # the old fallback takes one of them when nothing else is left
        self.assertIsNotNone(pick)

    def test_a_harvested_plant_is_gone_by_its_own_destroyed_mark_and_its_start_state_has_it_standing(self):
        # the plants end in `setdestroyed 1`: they stay enabled, so the old enabled-flag check never saw one gone
        for sid, who in (("S07", "client"), ("S08", "host")):
            res = run_fake(steps=story.pick_steps(SCRIPT, sid))
            row = next(r for r in res["steps"] if r["room"] == sid)
            self.assertEqual(row["verdict"], "PASS", (sid, row.get("reason")))
            self.assertIn("setstage VCG03 10", row["story"]["prep"], sid)    # the plants are enabled by VCG03 stage 10: forced by the host console, as the story does
            names = [c["name"] for c in row["checks"]]
            self.assertTrue(any("stands in the world" in n for n in names), names)
            self.assertTrue(any("is gone on both copies" in n for n in names), names)

    def test_a_pickup_whose_plant_was_never_enabled_fails_at_the_start_of_the_step(self):
        bad = copy.deepcopy(SCRIPT.data)
        next(s for s in bad["steps"] if s["id"] == "S07").pop("prep")        # VCG03 stays at stage 0: the plants are disabled and "gone on both copies" would be true from the start
        res = run_fake(script=D.Script(bad), steps=story.pick_steps(D.Script(bad), "S07"))
        row = next(r for r in res["steps"] if r["room"] == "S07")
        self.assertEqual(row["verdict"], "FAIL")
        self.assertIn("stands in the world", first_failed(row)["name"])

    def test_each_fake_bug_fails_its_step_at_its_check(self):
        for bug, (step, text) in EXPECT.items():
            with self.subTest(bug=bug):
                res = run_fake([bug], steps=story.pick_steps(SCRIPT, ONLY[bug]) if bug in ONLY else None)
                bad = [r for r in res["steps"] if r["verdict"] != "PASS"]
                self.assertTrue(bad, f"{bug}: nothing failed")
                self.assertEqual(bad[0]["room"], step, bug)
                c = first_failed(bad[0])
                self.assertIsNotNone(c, bug)
                self.assertIn(text, c["name"], bug)
                self.assertEqual(c.get("class"), "SYNC", bug)

    def test_every_fake_bug_has_a_test(self):
        self.assertEqual(set(EXPECT), set(storyfake.BUGS))

    def test_a_failed_step_names_the_first_divergence_with_both_values(self):
        res = run_fake(["objective_unsynced"])
        r = next(x for x in res["steps"] if x["room"] == "S03")
        d = r["story"]["divergence"]
        self.assertTrue(d["key"].startswith("VCG02.objective"), d)
        self.assertNotEqual(d["host"], d["client"])
        c = first_failed(r)
        self.assertEqual(c["actual"]["key"], d["key"])

    def test_the_rest_goes_on_after_a_failed_step_from_its_own_start_state(self):
        res = run_fake(["pickup_unsynced"])
        v = {r["room"]: r["verdict"] for r in res["steps"]}
        self.assertEqual(v["S07"], "FAIL")
        self.assertEqual(v["S11"], "PASS")
        self.assertEqual(v["S16"], "PASS")

    def test_stop_on_fail_blocks_the_steps_after_the_first_failure(self):
        res = run_fake(["pickup_unsynced"], stop_on_fail=True)
        v = {r["room"]: r["verdict"] for r in res["steps"]}
        self.assertEqual((v["S07"], v["S08"]), ("FAIL", "BLOCKED"))

    def test_a_subset_runs_s00_first_and_forces_the_start_state(self):
        res = run_fake(steps=story.pick_steps(SCRIPT, "S11"))
        self.assertEqual([r["room"] for r in res["steps"]], ["S00", "S11"])
        self.assertEqual(res["verdict"], "PASS")

    def test_a_step_over_its_time_box_is_a_timeout_with_the_checks_so_far(self):
        d = copy.deepcopy(SCRIPT.data)
        st = next(s for s in d["steps"] if s["id"] == "S04")
        st["do"] = [{"a": "wait", "who": "host", "seconds": 400}] + st["do"]
        st["box_s"] = 30
        res = run_fake(script=D.Script(d))
        r = next(x for x in res["steps"] if x["room"] == "S04")
        self.assertEqual(r["verdict"], "TIMEOUT", r)
        self.assertLess(r["seconds"], 40)

    def test_a_crashed_copy_blocks_the_rest(self):
        env = storyfake.FakeStoryEnv(SCRIPT, PLAN)
        env.probe = lambda pair: (lambda: ["client"] if env.w.t > 120 else [])
        res = story.execute(SCRIPT, env, list(SCRIPT.steps), dict(C.PARAMS), 3000, False, "t", "fake", lambda s: None)
        classes = [(r["room"], r["verdict"], r.get("class")) for r in res["steps"]]
        first = next(c for c in classes if c[1] != "PASS")
        self.assertEqual(first[2], "CRASH")
        self.assertTrue(all(c[1] == "BLOCKED" for c in classes[classes.index(first) + 1:]))

    def test_the_run_budget_stops_the_steps_that_have_not_started(self):
        res = run_fake(budget_s=60.0)
        self.assertTrue(any(r["verdict"] == "TIMEOUT" and "budget" in r["reason"] for r in res["steps"]))

    def test_an_unknown_step_or_bug_is_refused(self):
        with self.assertRaises(ValueError):
            story.pick_steps(SCRIPT, "S99")
        with self.assertRaises(ValueError):
            storyfake.FakeStoryEnv(SCRIPT, PLAN, ["nope"])
        with self.assertRaises(ValueError):
            story.parse_params(["story.nothing=1"])


class LiveItemCount(unittest.TestCase):
    """S07 raised on `player.GetItemCount FalloutNV.esm:13B2BA`: the console takes a hex form id, not Plugin:ID."""

    class Nv:
        def __init__(self):
            self.lines = []

        def plugins(self):
            return ["FalloutNV.esm", "Update.esm"]

        def eval(self, cmd, ref=None):
            self.lines.append(cmd)
            if ":" in cmd:
                raise RuntimeError("Syntax Error. Reference not allowed in this context. Missing parameter ObjectID.")
            return 3.0

    def test_item_count_reads_the_item_by_its_hex_runtime_id(self):
        import story_live as SL
        nv = self.Nv()
        m = SL.StoryLiveMachine(nv, "client")
        self.assertEqual(m.item_count("FalloutNV.esm:13B2BA"), 3)
        self.assertEqual(nv.lines, ["player.GetItemCount 0013B2BA"])
        self.assertEqual(m.item_count("Update.esm:000F"), 3)
        self.assertEqual(nv.lines[-1], "player.GetItemCount 0100000F")


class Reports(unittest.TestCase):
    def test_summary_is_under_5kb_with_the_table_and_the_guest_line(self):
        res = run_fake(["objective_unsynced"])
        res["previous"] = {"stamp": None, "changes": []}
        text = R.render_summary(res)
        self.assertLess(len(text.encode("utf-8")), 5000)
        self.assertIn("| step | lead | verdict |", text)
        self.assertIn("Guest-initiated dialogue:", text)
        self.assertIn("first divergence", text)
        self.assertIn("### FAIL S03", text)
        for sid in IDS:
            self.assertIn(f"| {sid} |", text)

    def test_summary_stays_under_5kb_even_with_long_texts_and_many_failures(self):
        res = run_fake(["start_unsynced"])
        for r in res["steps"]:
            for c in r["checks"]:
                c["actual"] = "x" * 400
                c["name"] = "y" * 190
            r["title"] = "t" * 300
        self.assertLess(len(R.render_summary(res).encode("utf-8")), 5000)

    def test_results_json_and_summary_are_written_and_compared_with_the_previous_run(self):
        with tempfile.TemporaryDirectory() as td:
            a = run_fake()
            a["stamp"] = "20260101-000001"
            R.write_run(Path(td) / a["stamp"], a, Path(td))
            b = run_fake(["pickup_unsynced"])
            b["stamp"] = "20260101-000002"
            R.write_run(Path(td) / b["stamp"], b, Path(td))
            res = json.loads((Path(td) / b["stamp"] / "results.json").read_text(encoding="utf-8"))
            self.assertEqual(res["previous"]["stamp"], a["stamp"])
            self.assertTrue(any("S07: PASS -> FAIL (REGRESSED)" in c for c in res["previous"]["changes"]), res["previous"])
            self.assertTrue((Path(td) / b["stamp"] / "summary.md").is_file())
            self.assertEqual(res["steps"][0]["room"], "S00")

    def test_exit_codes_follow_the_lab(self):
        self.assertEqual(R.exit_code("PASS", None), 0)
        self.assertEqual(R.exit_code("FAIL", "SYNC"), 1)
        self.assertEqual(R.exit_code("BLOCKED", "TEST"), 3)
        self.assertEqual(R.exit_code("BLOCKED", "ENV"), 4)


class Cli(unittest.TestCase):
    def test_list_and_a_fake_run_work_from_the_command_line(self):
        self.assertEqual(story.main(["list"]), 0)
        with tempfile.TemporaryDirectory() as td:
            self.assertEqual(story.main(["run", "--fake", "--steps", "S03", "--out-root", td, "--stamp", "x"]), 0)
            self.assertTrue((Path(td) / "x" / "results.json").is_file())
            self.assertEqual(story.main(["run", "--fake", "--bug", "guest_pick_lost", "--steps", "S03", "--out-root", td, "--stamp", "y"]), 1)
            self.assertEqual(story.main(["run", "--fake", "--steps", "S99", "--out-root", td]), 2)


if __name__ == "__main__":
    unittest.main(verbosity=1)
