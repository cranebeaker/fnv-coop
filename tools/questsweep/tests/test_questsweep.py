"""Game-free tests for the questline sweeps: plans, the rules overlay, the playthrough playbooks on the fake co-op world, and their twins.

Run: python -I tools/questsweep/run_tests.py     (prints RESULT: ALL OK)

What is proven (fake world and static data only; NOTHING here has run in a game):
  * every questline has a committed plan that is self-consistent and covers every stage of the lane's own sweep config,
  * the rules block of every plan equals rules.json, and rules.json holds the whole overlay and validates like the C++ loader does,
  * all 17 playbooks pass on a correct fake world with 2 players (a subset with 3 and 4),
  * every twin (a playbook plus one deliberately wrong behaviour of the fake or of the model) FAILS with a reason that names the problem.
"""
from __future__ import annotations

import copy
import json
import os
import subprocess
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
for p in (os.path.join(REPO, "tools", "bots"), os.path.join(REPO, "tools")):
    if p not in sys.path:
        sys.path.insert(0, p)

from playbooks import core, fakecoop, questlines as QL  # noqa: E402
from playbooks.questlines import model as QM, plan as QP  # noqa: E402
from questsweep import checks, common, gen_plans, rules_overlay as RO, rulesview  # noqa: E402
from questsweep.lines import LINES  # noqa: E402

# The plans (questlines/<line>/playthrough.json) are generated from the game's own data files by gen_plans.py, so the public
# repository does not ship them. Without them every test here would only report the missing files: skip with that reason.
MISSING_PLANS = [l.slug for l in LINES if not os.path.exists(gen_plans.plan_path(l.slug))]


def load_tests(loader, tests, pattern):
    if not MISSING_PLANS:
        return tests

    class NoPlans(unittest.TestCase):
        @unittest.skip(f"no playthrough plans for {len(MISSING_PLANS)} of {len(LINES)} lines: generate them from your game with "
                       f"python -I tools/questsweep/gen_plans.py")
        def test_needs_the_generated_plans(self):
            pass
    return loader.loadTestsFromTestCase(NoPlans)


OPTS = {"fast": True, "anim": False, "ui_s": 0.2, "barrier_s": 0.2, "stage_s": 0.2, "handoff_s": 0.2, "spawn_s": 0.2, "settle_s": 0.3,
        "settle_poll_s": 0.0, "repo_root": REPO}
_TMP = tempfile.mkdtemp(prefix="qs-tests-")


def run(line, bugs=(), n=2, plan=None, opts=None, model_patch=None):
    name = "ql_" + line
    pb = QL.make(name, plan=plan)
    world, refs = QL.fake(name, n, set(bugs), plan=pb.plan)
    d = tempfile.mkdtemp(prefix=line + "-", dir=_TMP)
    world.flight_dir = os.path.join(d, "flight")
    o = dict(OPTS)
    o.update(opts or {})
    return core.run_playbook(pb, fakecoop.make_machines(world), fakecoop.FakeComparer(), d, refs, o), pb, world


def tiny_plan():
    """A hand-made plan that exercises every rule: barrier, ready, exclusive, guest-local, items, karma, world, cross-quest stages."""
    R = lambda **k: {**{"guest_local": False, "barrier": None, "ready": [], "exclusive": [], "vars": [], "reward_all": True, "item_dup": False,
                       "initiator_dialogue": False, "companion_owner": False, "tether": None, "slideshow": False}, **k}
    A, B, C, G = "00000A01", "00000A02", "00000A03", "00000A04"
    item = "FalloutNV.esm:000AAA"
    return {
        "schema": 1, "kind": "questline-playthrough", "line": "tiny", "title": "tiny", "category": "test", "players": {"min": 2, "max": 4},
        "reset": [], "warnings": [], "stats": {}, "ready_stages": [{"quest": B, "stage": 30, "info": "FalloutNV.esm:0000F1", "rule_quest": "QB"}],
        "quests": [
            {"edid": "QA", "ref": "FalloutNV.esm:000A01", "id": A, "name": "A", "plugin": "FalloutNV.esm", "start": True, "prereq": [], "run_last": False,
             "objectives": {}, "rules": R(exclusive=[[30, 40]]),
             "stages": [{"index": 10, "fx": {"obj": [[A, 1, "displayed", 1]], "items": [[item, 2]], "xp": 100, "karma": 5,
                                              "enable": ["FalloutNV.esm:000B01"], "stage": [[B, 10]]},
                         "cond": {}, "objective_effects": [{"objective": 1, "kind": "displayed", "value": 1}],
                         "trigger": {"info": "FalloutNV.esm:0000F0", "on": "end", "quest": A, "speaker_base": None, "alternatives": 0}},
                        {"index": 20, "fx": {"disable": ["FalloutNV.esm:000B01"], "quest": [[C, "start"]]}, "cond": {}, "objective_effects": [],
                         "act": [{"o": 1, "verb": "kill", "refs": ["FalloutNV.esm:000B02"]}]},
                        {"index": 30, "fx": {"items": [[item, 1]]}, "cond": {}, "objective_effects": []},
                        {"index": 40, "fx": {"items": [[item, 7]]}, "cond": {}, "objective_effects": []}]},
            {"edid": "QB", "ref": "FalloutNV.esm:000A02", "id": B, "name": "B", "plugin": "FalloutNV.esm", "start": True, "prereq": [], "run_last": False,
             "objectives": {}, "rules": R(barrier=[10], ready=["FalloutNV.esm:0000F1"]),
             "stages": [{"index": 10, "fx": {"items": [[item, 3]]}, "cond": {}, "objective_effects": []},
                        {"index": 30, "fx": {"obj": [[B, 2, "completed", 1]]}, "cond": {}, "objective_effects": [],
                         "trigger": {"info": "FalloutNV.esm:0000F1", "on": "end", "quest": B, "speaker_base": None, "alternatives": 0}}]},
            {"edid": "QG", "ref": "FalloutNV.esm:000A04", "id": G, "name": "G", "plugin": "FalloutNV.esm", "start": True, "prereq": [], "run_last": False,
             "objectives": {}, "rules": R(guest_local=True, vars=["iMine"]),
             "stages": [{"index": 10, "fx": {"obj": [[G, 1, "displayed", 1]], "items": [[item, 1]]}, "cond": {}, "objective_effects": []}]},
        ]}


# ------------------------------------------------------------------------------------------------- committed data
class Plans(unittest.TestCase):
    def test_every_line_has_a_plan(self):
        have = set(QP.available_lines(REPO))
        self.assertEqual({l.slug for l in LINES}, have)
        self.assertEqual(len(have), 17)
        self.assertEqual(set(QL.names()), {"ql_" + l.slug for l in LINES})

    def test_plans_are_self_consistent(self):
        for l in LINES:
            with self.subTest(l.slug):
                data = common.load_json(QP.plan_file(l.slug))
                self.assertEqual(QP.check_plan(data), [])
                self.assertGreaterEqual(data["stats"]["stages"], 19)

    def test_check_plan_twins(self):
        base = common.load_json(QP.plan_file("yesman"))
        def mut(fn):
            d = copy.deepcopy(base)
            fn(d)
            return QP.check_plan(d)
        self.assertTrue(mut(lambda d: d["quests"][0].update(id="12")), "a short id must be rejected")
        self.assertTrue(mut(lambda d: d["quests"][0]["stages"].append(copy.deepcopy(d["quests"][0]["stages"][0]))), "a duplicate stage must be rejected")
        self.assertTrue(mut(lambda d: d["quests"][0]["stages"][0]["fx"].update(frobnicate=[1])), "an unknown fx key must be rejected")
        self.assertTrue(mut(lambda d: d["quests"][0]["rules"].update(exclusive=[[100]])), "a one-stage exclusive group must be rejected")
        self.assertTrue(mut(lambda d: d.update(kind="x")), "the wrong kind must be rejected")
        self.assertTrue(mut(lambda d: d["quests"][0]["stages"][0]["fx"].update(items=[["bad", 1]])), "a malformed item must be rejected")
        self.assertEqual(mut(lambda d: None), [])

    def test_plans_cover_every_stage_of_the_lane_sweeps(self):
        for l in LINES:
            if l.slug == "companions":      # a subset of the systems sweep: the companions' own quests
                continue
            with self.subTest(l.slug):
                plan = common.load_json(QP.plan_file(l.slug))
                self.assertEqual(checks.coverage_gaps(plan, checks.lane_stages(l.slug)), [])

    def test_coverage_twin_a_missing_stage_is_found(self):
        plan = common.load_json(QP.plan_file("ncr"))
        lane = checks.lane_stages("ncr")
        plan["quests"][0]["stages"].pop()
        self.assertTrue(checks.coverage_gaps(plan, lane))
        plan["quests"].pop(0)
        self.assertTrue(any("missing from the plan" in g for g in checks.coverage_gaps(plan, lane)))

    def test_goodsprings_has_a_lane_format_sweep_config(self):
        sys.path.insert(0, os.path.join(REPO, "plugin", "snapshot", "tools"))
        import sweep_config as sc
        cfg = sc.load_config(os.path.join(REPO, "questlines", "goodsprings", "sweep.json"))
        self.assertEqual(sc.plan_stats(cfg)["quests_with_stages"], 6)
        self.assertEqual(sc.plan_stats(cfg)["stages"], 104)

    def test_plan_rules_blocks_equal_rules_json(self):
        rules = rulesview.load_rules()
        for l in LINES:
            with self.subTest(l.slug):
                self.assertEqual(checks.rules_drift(common.load_json(QP.plan_file(l.slug)), rules), [])

    def test_rules_drift_twin(self):
        rules = copy.deepcopy(rulesview.load_rules())
        plan = common.load_json(QP.plan_file("prologue"))
        rules["VNipton"]["rules"] = [r for r in rules["VNipton"]["rules"] if r["kind"] != "ExclusiveStages"]
        self.assertTrue(checks.rules_drift(plan, rules), "dropping ExclusiveStages from rules.json must show up as drift")

    def test_the_rules_the_playbooks_exercise_are_in_the_plans(self):
        total = {"exclusive": 0, "ready": 0, "guest_local": 0, "vars": 0, "barrier": 0}
        for l in LINES:
            d = common.load_json(QP.plan_file(l.slug))
            total["ready"] += len(d["ready_stages"])
            for q in d["quests"]:
                total["exclusive"] += len(q["rules"]["exclusive"])
                total["guest_local"] += bool(q["rules"]["guest_local"])
                total["vars"] += len(q["rules"]["vars"])
                total["barrier"] += bool(q["rules"]["barrier"])
        for k, v in total.items():
            self.assertGreater(v, 0, f"no plan exercises {k}")

    def test_gen_check_matches_the_committed_plans(self):
        if not common.have_questdata():
            self.skipTest(common.questdata_missing_reason())
        self.assertEqual(gen_plans.main(["--check"]), 0)

    def test_generator_helpers(self):
        for text, verb in [("Kill the Geckos at the well.", "kill"), ("Talk to Sunny Smiles.", "talk"), ("Go to the Prospector Saloon", "walk"),
                           ("Retrieve the package", "take"), ("Meet the Doc", "talk"), ("", "walk")]:
            self.assertEqual(gen_plans.verb_of(text), verb, text)
        fx, cond = gen_plans.summarize([
            {"type": "SetObjective", "quest": "FalloutNV.esm:000A01", "objective": 3, "kind": "displayed", "value": 1},
            {"type": "AddItem", "item": "FalloutNV.esm:000AAA", "count": 2, "to": "FalloutNV.esm:000014"},
            {"type": "AddItem", "item": "FalloutNV.esm:000AAB", "count": 2, "to": "FalloutNV.esm:000999"},           # to an NPC: not a reward
            {"type": "ModActorValue", "av_name": "XP", "mode": "mod", "delta": 50, "to": "FalloutNV.esm:000014"},
            {"type": "ModKarma", "delta": -10}, {"type": "EnableRef", "ref": "FalloutNV.esm:000B01", "conditional": True},
            {"type": "KillActor", "actor": "FalloutNV.esm:000B02", "dynamic": True}, {"type": "SetStage", "quest": "DeadMoney.esm:005229", "stage": 10}], False)
        self.assertEqual(fx["obj"], [["00000A01", 3, "displayed", 1]])
        self.assertEqual(fx["items"], [["FalloutNV.esm:000AAA", 2]])
        self.assertEqual((fx["xp"], fx["karma"]), (50, -10))
        self.assertEqual(fx["stage"], [["01005229", 10]])
        self.assertEqual(cond.get("EnableRef"), 1)
        self.assertEqual(cond.get("KillActor"), 1)
        self.assertEqual(cond.get("unmodeled:AddItem"), 1)
        fx2, cond2 = gen_plans.summarize([{"type": "EnableRef", "ref": "FalloutNV.esm:000B01"}], True)       # an entry with conditions: nothing is certain
        self.assertEqual((fx2, cond2), ({}, {"EnableRef": 1}))
        self.assertEqual(common.runtime_id("DeadMoney.esm:005229"), "01005229")
        self.assertEqual(common.runtime_id("OldWorldBlues.esm:002FCA"), "03002FCA")


# ------------------------------------------------------------------------------------------------- the model and the fake
class ModelAndFake(unittest.TestCase):
    def test_model_rules(self):
        p = QP.Plan(tiny_plan())
        m = QM.Model(p)
        r = m.apply("A01", 10)
        self.assertEqual(r.stages, [("A01", 10)])
        self.assertEqual([h[:2] for h in r.held], [("A02", 10)], "the cross-quest stage is held by QB's barrier")
        self.assertEqual(r.items, {"FalloutNV.esm:000AAA": 2, "xp": 100})
        self.assertEqual(r.karma, 5)
        self.assertEqual(m.apply("A01", 10).stages, [], "a stage fires once")
        rel = m.release("A02", 10)
        self.assertEqual(rel.stages, [("A02", 10)])
        self.assertEqual(m.apply("A01", 30).stages, [("A01", 30)])
        self.assertEqual(m.apply("A01", 40).dropped, [("A01", 40)], "the second outcome of an exclusive set loses")
        self.assertEqual(m.apply("A02", 30).held[0][2], "ready")

    def test_model_cycle_guard(self):
        d = tiny_plan()
        d["quests"][0]["stages"][0]["fx"]["stage"] = [["00000A01", 20]]
        d["quests"][0]["stages"][1]["fx"]["stage"] = [["00000A01", 10]]
        r = QM.Model(QP.Plan(d)).apply("A01", 10)
        self.assertEqual(r.stages, [("A01", 10), ("A01", 20)])

    def test_tiny_plan_passes_with_2_3_4_players(self):
        for n in (2, 3, 4):
            r, pb, w = run("tiny", n=n, plan=QP.Plan(tiny_plan()))
            self.assertTrue(r.ok, f"{n} players: {r.reason}")
            st = pb.driver.stats
            self.assertGreaterEqual(st["barrier"], 1)
            self.assertGreaterEqual(st["ready"], 1)
            self.assertGreaterEqual(st["exclusive_refused"], 1)
            self.assertGreaterEqual(st["guest_local"], 1)
            self.assertEqual(w.exclusive_rejected, 1)
            self.assertEqual(sorted({s for s, _, _ in w.local_log}), list(range(n)), "every machine ran the guest-local quest itself")

    def test_tiny_plan_twins(self):
        for bug, frag in [("exclusive_ignored", "exclusive"), ("ready_skipped", "ready check"), ("barrier_ignored", "barrier"),
                          ("guestlocal_replicated", "guest-local"), ("guestlocal_clients_blocked", "guest"), ("reward_to_host_only", "reward"),
                          ("reward_twice", "reward"), ("no_world_sync", "enabled"), ("karma_local", "karma"), ("no_quest_run_sync", "running"),
                          ("cross_stage_dropped", "stage"), ("pvars_shared", "per-player"), ("no_stage_sync", "stage"), ("no_objective_sync", "objective"),
                          ("counter_noise", "desync counters")]:
            with self.subTest(bug):
                r, _, _ = run("tiny", [bug], plan=QP.Plan(tiny_plan()))
                self.assertFalse(r.ok, f"{bug} was not caught")
                self.assertIn(frag, r.reason.lower(), r.reason)

    def test_unknown_fake_bug_is_an_error(self):
        with self.assertRaises(ValueError):
            QL.fake("ql_yesman", 2, {"no_such_bug"})

    def test_early_barrier_passes_are_consumed_once(self):
        w, _ = QL.fake("ql_yesman", 2, plan=QP.Plan(tiny_plan()))
        w.pass_scene(0, "A02")
        w.pass_scene(1, "A02")                          # both finished before anybody asked
        self.assertEqual(w.early_passes["A02"], {0, 1})
        w.request_stage("A02", 10)
        self.assertTrue(w.bots[0].stage_done("A02", 10), "early passes release the barrier at once")
        self.assertFalse(w.early_passes.get("A02"), "...and they are used up: the next barrier of the quest waits for the players again")

    def test_follower_caps_per_player(self):
        r, pb, w = run("companions")
        self.assertTrue(r.ok, r.reason)
        r, _, _ = run("companions", ["no_follower_cap"])
        self.assertFalse(r.ok)
        r, _, _ = run("companions", ["recruit_not_exclusive"])
        self.assertFalse(r.ok)
        r, _, _ = run("companions", ["recruit_local"])
        self.assertFalse(r.ok)


# ------------------------------------------------------------------------------------------------- the playbooks
class Playthroughs(unittest.TestCase):
    def test_every_line_passes_with_two_players(self):
        for l in LINES:
            with self.subTest(l.slug):
                r, pb, w = run(l.slug)
                self.assertTrue(r.ok, f"{l.slug}: {r.reason}")
                self.assertGreaterEqual(len(r.steps), 3)
                stages = pb.plan.stats()["stages"]
                self.assertGreaterEqual(pb.driver.stats["stages"] + pb.driver.stats["skipped_excluded"], stages * 0.9, "the run must visit the plan's stages")

    def test_three_and_four_players(self):
        for line in ("goodsprings", "yesman", "prologue", "owb", "companions", "systems"):
            for n in (3, 4):
                with self.subTest(f"{line} x{n}"):
                    r, _, _ = run(line, n=n)
                    self.assertTrue(r.ok, f"{line} x{n}: {r.reason}")

    def test_players_out_of_range_are_refused(self):
        pb = QL.make("ql_yesman")
        w, refs = QL.fake("ql_yesman", 1, plan=pb.plan)
        res = core.run_playbook(pb, fakecoop.make_machines(w), fakecoop.FakeComparer(), tempfile.mkdtemp(dir=_TMP), refs, dict(OPTS))
        self.assertFalse(res.ok)

    TWINS = [
        ("prologue", "exclusive_ignored", "exclusive outcome"),
        ("yesman", "exclusive_ignored", "exclusive outcome"),
        ("yesman", "ready_skipped", "without asking the party"),
        ("yesman", "ready_veto_ignored", "ready check"),
        ("goodsprings", "barrier_ignored", "before the players passed"),
        ("goodsprings", "pnr_early", "barrier"),
        ("systems", "guestlocal_replicated", "reached the clients"),
        ("systems", "guestlocal_clients_blocked", "guest scripts do not run"),
        ("act2", "reward_to_host_only", "reward"),
        ("act2", "reward_twice", "reward"),
        ("owb", "reward_twice", "reward"),
        ("goodsprings", "no_world_sync", "enabled"),
        ("act2", "karma_local", "karma"),
        ("towns", "karma_local", "karma"),
        ("lonesomeroad", "no_quest_run_sync", "running"),
        ("lonesomeroad", "cross_stage_dropped", "stage"),
        ("owb", "pvars_shared", "per-player variable"),
        ("companions", "no_follower_cap", "should not follow"),
        ("companions", "recruit_not_exclusive", "owned by slot"),
        ("companions", "recruit_local", "teammate"),
        ("house", "no_stage_sync", "stage"),
        ("ncr", "no_objective_sync", "objective"),
        ("legion", "no_fight_sync", "alive"),
        ("honesthearts", "counter_noise", "desync counters"),
        ("strip", "counter_noise", "desync counters"),
    ]

    def test_twins_fail(self):
        for line, bug, frag in self.TWINS:
            with self.subTest(f"{line}+{bug}"):
                r, _, _ = run(line, [bug])
                self.assertFalse(r.ok, f"{line} with {bug} passed")
                self.assertIn(frag.lower(), r.reason.lower(), f"{line}+{bug}: {r.reason}")

    def test_model_twins(self):
        """A wrong model must disagree with the (correct) fake: the playbook fails instead of passing by agreeing with itself."""
        real = QM.Model.apply

        def double_items(self, qk, n, released=False, _depth=0):
            r = real(self, qk, n, released, _depth)
            r.items = {k: 2 * v for k, v in r.items.items()}
            return r

        def no_exclusive(self, qk, n, released=False, _depth=0):
            g = self.plan.exclusive_group
            self.plan.exclusive_group = lambda *_: None
            try:
                return real(self, qk, n, released, _depth)
            finally:
                self.plan.exclusive_group = g

        def no_barrier(self, qk, n, released=False, _depth=0):
            return real(self, qk, n, True, _depth)

        for name, fn, line in [("model_expects_double_rewards", double_items, "act2"), ("model_ignores_exclusive", no_exclusive, "prologue"),
                               ("model_ignores_barrier", no_barrier, "goodsprings")]:
            with self.subTest(name):
                QM.Model.apply = fn
                try:
                    r, _, _ = run(line)
                finally:
                    QM.Model.apply = real
                self.assertFalse(r.ok, f"{name} was not caught")
        r, _, _ = run("act2")
        self.assertTrue(r.ok, r.reason)

    def test_quest_filter_and_excluded_stages(self):
        r, pb, _ = run("deadmoney", opts={"ql_quests": ["NVDLC01MQ00"]})
        self.assertTrue(r.ok, r.reason)
        self.assertLess(pb.driver.stats["stages"], 10)
        r, pb, _ = run("deadmoney", opts={"ql_include_excluded": False})
        self.assertTrue(r.ok, r.reason)
        self.assertGreater(pb.driver.stats["skipped_excluded"], 0, "the lane's excluded stages are skipped when asked")

    def test_failure_files_are_saved(self):
        r, _, _ = run("yesman", ["no_stage_sync"])
        self.assertFalse(r.ok)
        self.assertTrue(any(s.fail_dir and os.path.isdir(s.fail_dir) for s in r.steps))

    def test_cli(self):
        env = dict(os.environ)
        py = [sys.executable, "-I", os.path.join(REPO, "tools", "questsweep", "run_questlines.py")]
        out = tempfile.mkdtemp(dir=_TMP)
        p = subprocess.run(py + ["--fake", "--line", "yesman,ncr", "--outdir", out], capture_output=True, text=True, env=env)
        self.assertEqual(p.returncode, 0, p.stdout + p.stderr)
        self.assertIn("PASS  ql_yesman", p.stdout)
        p = subprocess.run(py + ["--fake", "--line", "yesman", "--bug", "reward_twice", "--outdir", out], capture_output=True, text=True, env=env)
        self.assertEqual(p.returncode, 1, "a run with a bug must fail")
        p = subprocess.run(py + ["--list"], capture_output=True, text=True, env=env)
        self.assertEqual(p.returncode, 0)
        self.assertEqual(sum(l.startswith("ql_") for l in p.stdout.splitlines()), 17)
        p = subprocess.run(py + ["--fake", "--line", "nope"], capture_output=True, text=True, env=env)
        self.assertNotEqual(p.returncode, 0)


# ------------------------------------------------------------------------------------------------- the rules overlay
class RulesOverlay(unittest.TestCase):
    def test_committed_rules_hold_the_overlay_and_validate(self):
        doc = common.load_json(common.RULES_JSON)["quests"]
        ov = RO.load_overlay()
        self.assertGreater(len(ov["ops"]), 50)
        self.assertEqual(RO.held(doc, ov["ops"]), [])
        self.assertEqual(RO.validate(doc), [])

    def test_the_policy_gaps_are_closed(self):
        doc = common.load_json(common.RULES_JSON)["quests"]
        kinds = lambda e: {r["kind"]: r["params"] for r in doc[e]["rules"]}
        self.assertEqual(kinds("VNipton")["ExclusiveStages"]["stages"], [30, 35, 70])
        self.assertEqual(kinds("nVPrimmDeputyConv")["ExclusiveStages"]["stages"], [110, 120, 130])
        self.assertEqual(kinds("VMQYesMan01a")["ExclusiveStages"]["stages"], [100, 110])
        self.assertIn("FalloutNV.esm:15EB69", kinds("VDialogueMrHouse")["ReadyCheck"]["infos"])
        self.assertEqual(sorted(kinds("VMQTops")["ReadyCheck"]["infos"]), ["FalloutNV.esm:160723", "FalloutNV.esm:160735"])
        self.assertEqual(kinds("VMQ05")["Barrier"]["stages"], [100], "the barrier must not hold every stage of the quest")
        self.assertEqual(kinds("vHDBattleController")["Barrier"]["stages"], [30])
        self.assertEqual(kinds("VEnding")["Barrier"]["before"], "slideshow")
        self.assertEqual(kinds("NVDLC03MQ00")["PartyTether"]["anchor"], "NVDLC03CrashedSatelliteREF")
        self.assertEqual(kinds("NVDLC03MQ05")["PerPlayerVars"]["vars"], ["bRestoredBrain", "bRestoredHeart", "bRestoredSpine"], "organs are per player (wave 3b)")
        self.assertEqual(kinds("AchievementQuest")["PerPlayerScene"], {"run": "every-machine", "scope": "local-machine"})

    def test_dlc_setup_quests_keep_replicating_but_their_scripts_run_everywhere(self):
        doc = common.load_json(common.RULES_JSON)["quests"]
        guest = common.load_json(common.GUEST_LOCAL_JSON)
        self.assertEqual(checks.guest_local_gaps(doc, guest), [])
        for e in ("NVDLC01MQ00", "NVDLC02MQ00", "NVDLC03MQ00", "NVDLC04MQ00"):
            self.assertFalse(any(r["kind"] == "GuestLocalRun" for r in doc[e]["rules"]),
                             f"{e}: its stage results start the DLC story; a quest-level GuestLocalRun would drop them")
            self.assertFalse(rulesview.view(doc, e)["guest_local"])
        holder = [r for r in doc["VDLCPackQuest"]["rules"] if r["kind"] == "GuestLocalRun"][0]["params"]["scripts"]
        for s in ("DeadMoney.esm:0062B1", "HonestHearts.esm:008A32", "OldWorldBlues.esm:002FD0", "LonesomeRoad.esm:003604"):
            self.assertIn(s, holder)
        self.assertTrue(rulesview.view(doc, "AchievementQuest")["guest_local"])

    def test_guest_local_gap_twin(self):
        doc = copy.deepcopy(common.load_json(common.RULES_JSON)["quests"])
        guest = common.load_json(common.GUEST_LOCAL_JSON)
        for r in doc["VDLCPackQuest"]["rules"]:
            if r["kind"] == "GuestLocalRun":
                r["params"]["scripts"] = r["params"]["scripts"][:1]
        self.assertTrue(checks.guest_local_gaps(doc, guest), "dropping the DLC setup scripts from the allow-list must be found")

    def test_apply_is_idempotent_and_complete_on_a_synthetic_file(self):
        rules = {"A": {"tier": "T3", "questline": "x", "curated": False, "rules": [{"kind": "Barrier", "params": {"step": "all-players-pass"}, "origin": "curated"}]}}
        ops = [{"quest": "A", "kind": "Barrier", "mode": "merge", "params": {"stages": [100]}},
               {"quest": "A", "kind": "ExclusiveStages", "mode": "add", "params": {"stages": [1, 2], "resolution": "first-wins"}},
               {"quest": "A", "kind": "GuestLocalRun", "mode": "merge", "params": {"scripts": ["X.esm:0001"]}},
               {"quest": "NEW", "kind": "GuestLocalRun", "mode": "merge", "params": {"scripts": ["X.esm:0002"]},
                "entry": {"tier": "T1", "questline": "y", "curated": False}}]
        self.assertTrue(RO.held(rules, ops))
        self.assertGreater(RO.apply_ops(rules, ops), 0)
        self.assertEqual(RO.held(rules, ops), [])
        self.assertEqual(RO.apply_ops(rules, ops), 0, "a second apply changes nothing")
        self.assertEqual(rules["A"]["rules"][0]["params"], {"step": "all-players-pass", "stages": [100]})
        self.assertEqual(RO.validate(rules), [])
        with self.assertRaises(KeyError):
            RO.apply_ops(rules, [{"quest": "MISSING", "kind": "Barrier", "mode": "merge", "params": {}}])

    def test_validator_twins(self):
        def bad(kind, params):
            return RO.validate({"Q": {"rules": [{"kind": kind, "params": params}]}})
        self.assertTrue(bad("ExclusiveStages", {"stages": [5]}))
        self.assertTrue(bad("ExclusiveStages", {"stages": [5, 6], "resolution": "last-wins"}))
        self.assertTrue(bad("ReadyCheck", {}))
        self.assertTrue(bad("ReadyCheck", {"infos": ["FalloutNV.esm:15EB69"], "on_timeout": "maybe"}))
        self.assertTrue(bad("ReadyCheck", {"infos": ["15EB69"]}))
        self.assertTrue(bad("ReadyCheck", {"infos": ["FalloutNV.esm:15EB69"], "timeout_s": 0}))
        self.assertTrue(bad("PartyTether", {"cells": []}))
        self.assertTrue(bad("PartyTether", {"anchor": "Ref", "radius": 0}))
        self.assertTrue(bad("PerPlayerVars", {"vars": []}))
        self.assertTrue(bad("Barrier", {"before": "never"}))
        self.assertTrue(bad("Barrier", {"stages": ["x"]}))
        self.assertTrue(bad("GuestLocalRun", {"scripts": ["nonsense"]}))
        self.assertEqual(bad("Barrier", {"stages": [3], "before": "slideshow"}), [])
        self.assertEqual(bad("PartyTether", {"anchor": "Ref", "radius": 10}), [])

    def test_unapplied_overlay_is_detected(self):
        doc = copy.deepcopy(common.load_json(common.RULES_JSON)["quests"])
        doc["VNipton"]["rules"] = [r for r in doc["VNipton"]["rules"] if r["kind"] != "ExclusiveStages"]
        self.assertTrue(RO.held(doc, RO.load_overlay()["ops"]))

    def test_overlay_cli_check(self):
        p = subprocess.run([sys.executable, "-I", os.path.join(REPO, "tools", "questsweep", "rules_overlay.py"), "--check"], capture_output=True, text=True)
        self.assertEqual(p.returncode, 0, p.stdout + p.stderr)
        self.assertIn("ALL OK", p.stdout)

    def test_derived_overlay_equals_the_committed_one(self):
        if not common.have_questdata():
            self.skipTest(common.questdata_missing_reason())
        # derive from the rules WITHOUT the overlay's own additions is not possible offline; the derivation only reads pending + guest-local + questdata,
        # so deriving against the current file must give the same op list
        self.assertEqual(RO.derive(common.load_json(common.RULES_JSON)["quests"]), RO.load_overlay())

    def test_real_cpp_rules_loader_accepts_rules_json(self):
        exe = os.path.join(REPO, "build", "all", "synccore", "fnvcoop_tests.exe")
        if not os.path.exists(exe):
            self.skipTest("build/all/synccore/fnvcoop_tests.exe is not built (python -I tools/build/build.py --target fnvcoop_tests)")
        p = subprocess.run([exe, "--group", "rules"], capture_output=True, text=True, timeout=600)
        self.assertEqual(p.returncode, 0, p.stdout[-800:])
        self.assertIn("0 failed", p.stdout)
        self.assertIn("loads_the_real_rules_json_and_is_strict", p.stdout)


class Wave1Data(unittest.TestCase):
    """Campaign fix wave 1 rule data (tools/questsweep/wave1_data.py, gen_cause_quests.py): the committed files are current, name real things, and a stale file is caught."""

    @classmethod
    def setUpClass(cls):
        from questsweep import gen_cause_quests as GC, wave1_data as W1
        cls.GC, cls.W1 = GC, W1
        cls.rules = common.load_json(common.RULES_JSON)

    def test_cause_map_is_current(self):
        if not common.have_questdata():
            self.skipTest(common.questdata_missing_reason())
        self.assertEqual(self.GC.main(["--check"]), 0)

    def test_cause_map_names_the_endgame_controllers(self):
        doc = common.load_json(self.GC.OUT)
        self.assertEqual(doc["schema"], 1)
        s = doc["scripts"]
        # object scripts that start an ending / a battle map to the quest that carries the scene rules (PartyWarp, SetPlayerControls, SaveSuppress, SlideshowEnd)
        self.assertEqual(s["falloutnv.esm:144484"], "VHDHouseBattle")        # MrHouseSCRIPT
        self.assertEqual(s["falloutnv.esm:15f047"], "VHDIndependentBattle")   # YesManDeadSCRIPT
        self.assertEqual(s["falloutnv.esm:145ec2"], "VMQ05")                  # VHDColonelMooreSCRIPT
        for ref, quest in s.items():
            self.assertTrue(quest in self.rules["quests"], f"{ref} -> {quest} has no rules")
        for key, ref in doc["keys"].items():
            self.assertIn(key, self.rules["quests"])
            self.assertEqual(s[ref], key)

    def test_stale_cause_map_is_caught(self):
        if not common.have_questdata():
            self.skipTest(common.questdata_missing_reason())
        doc = self.GC.build()
        doc["scripts"]["falloutnv.esm:144484"] = "VCG01"      # the twin: a wrong owner for the House script
        keep = self.GC.OUT
        with tempfile.TemporaryDirectory() as d:
            self.GC.OUT = os.path.join(d, "cause_quests.json")
            try:
                with open(self.GC.OUT, "w", encoding="utf-8", newline=chr(10)) as f:
                    f.write(json.dumps(doc, indent=1) + chr(10))
                self.assertEqual(self.GC.main(["--check"]), 1)
                self.assertEqual(self.GC.main([]), 0)           # regenerating repairs it
                self.assertEqual(self.GC.main(["--check"]), 0)
            finally:
                self.GC.OUT = keep

    def test_perk_routing_is_written_for_the_named_cases(self):
        q = self.rules["quests"]

        def lists(edid):
            return next(r["params"] for r in q[edid]["rules"] if r["kind"] == "PerkRecipient")
        self.assertIn("RangerTakedown", lists("VDialogueNovac")["initiator"])        # the trainer teaches the player who talks
        self.assertIn("PowerArmorTraining", lists("VDialogueHiddenValley")["all"])   # a quest-reward perk goes to everybody
        self.assertTrue(lists("VDialogueCraigBoone")["recruiter"])                   # a recruit perk goes to the recruiter

    def test_policy_object_and_new_kinds_validate(self):
        self.assertEqual(self.rules["policy"]["death"]["mode"], "downed")
        self.assertEqual(RO.validate(self.rules["quests"]), [])
        kinds = {r["kind"] for v in self.rules["quests"].values() for r in v["rules"]}
        for k in ("ChoiceConsent", "PerkRecipient", "FriendlyFireGuard", "PerPlayerState"):
            self.assertIn(k, kinds)

    def test_validator_rejects_bad_wave1_params(self):
        for kind, params in (("ChoiceConsent", {"topics": []}), ("ChoiceConsent", {"topics": ["not an id"]}), ("ChoiceConsent", {"topics": ["A.esm:01"], "mode": "coin"}),
                             ("PerkRecipient", {"default": "nobody"}), ("PerkRecipient", {}), ("FriendlyFireGuard", {}), ("PerPlayerState", {}),
                             ("SetPlayerControls", {"scope": "nobody"}), ("CompanionOwner", {"owner": "nobody"})):
            self.assertTrue(RO.validate({"Q": {"rules": [{"kind": kind, "params": params}]}}), f"{kind} {params} should be rejected")

    def test_funcs_merge_keeps_existing_reductions(self):
        dst = {"funcs": {"IsKiller": "any-party-member"}}
        self.assertTrue(RO._merge(dst, {"funcs": {"GetLevel": "host"}}))
        self.assertEqual(dst["funcs"], {"IsKiller": "any-party-member", "GetLevel": "host"})
        self.assertFalse(RO._merge(dst, {"funcs": {"GetLevel": "host"}}))


class Wave3bData(unittest.TestCase):
    """Campaign fix wave 3b rule data (tools/questsweep/wave3b_data.py, refcheck.py): nothing invented, nothing that cannot work, and each check has a twin that proves it can fail."""

    REF_KEYS = ("markers", "cells", "anchor", "anchor_ref", "actors", "items", "topics", "boxes", "all", "initiator", "recruiter", "npcs", "globals", "checkpoint", "infos")

    @classmethod
    def setUpClass(cls):
        if not common.have_questdata():
            raise unittest.SkipTest(common.questdata_missing_reason())
        from questsweep import refcheck, wave3b_data as W3
        cls.refcheck, cls.W3 = refcheck, W3
        cls.refs = refcheck.Refs()
        cls.doc = common.load_json(common.RULES_JSON)
        cls.rules = cls.doc["quests"]
        cls.quests = {q["edid"]: q for q in common.load_json(os.path.join(common.QD, "quests.json"))["quests"]}
        cls.scripts = {s["edid"]: s for s in common.load_json(os.path.join(common.QD, "scripts.json.gz"))}

    # ---- every id resolves ----------------------------------------------------------------------------------------------------------------
    def unresolved(self, rules, refs=None):
        refs = refs or self.refs
        bad = []
        for q, v in rules.items():
            if not q.startswith("NVDLC"):
                continue
            for rule in v["rules"]:
                for k, val in rule.get("params", {}).items():
                    if k not in self.REF_KEYS:
                        continue
                    for x in (val if isinstance(val, list) else [val]):
                        if isinstance(x, str) and not refs.resolve(x):
                            bad.append((q, rule["kind"], k, x))
        return bad

    def test_every_form_the_dlc_rules_name_resolves_in_questdata_or_the_game_data(self):
        self.assertEqual(self.unresolved(self.rules), [])

    def test_twin_an_invented_form_is_found(self):
        doc = json.loads(json.dumps(self.rules))
        doc["NVDLC02MQ03a"]["rules"].append({"kind": "PooledItems", "params": {"items": ["NVDLC02TotemThatDoesNotExist", "HonestHearts.esm:0FFFFE"]}})
        bad = self.unresolved(doc)
        self.assertEqual(sorted(x[3] for x in bad), ["HonestHearts.esm:0FFFFE", "NVDLC02TotemThatDoesNotExist"])

    def test_the_nuke_boxes_and_perks_are_what_the_data_says_they_are(self):
        for ref, sig, edid in (("LonesomeRoad.esm:009C99", "MESG", "NVDLC04TempleCaesarConfirmMSG"), ("LonesomeRoad.esm:009C98", "MESG", "NVDLC04TempleNCRConfirmMSG"),
                               ("LonesomeRoad.esm:00BF02", "MESG", "NVDLC04TempleNukeAllConfirmMSG"), ("OldWorldBlues.esm:015BBC", "PERK", "NVDLC03ReinforcedSpinePerk"),
                               ("LonesomeRoad.esm:004D16", "ACRE", "NVDLC04EDEREF"), ("FalloutNV.esm:000037", "GLOB", "GameDay")):
            r = self.refs.resolve(ref)
            self.assertIsNotNone(r, ref)
            self.assertEqual((r[1], r[2]), (sig, edid), ref)

    # ---- variable lists name real variables ---------------------------------------------------------------------------------------------
    def var_problems(self, rules):
        out = []
        for q, v in rules.items():
            if not q.startswith("NVDLC"):
                continue
            have = {x["name"] for x in (self.quests[q].get("script_data") or {}).get("vars", [])} if q in self.quests else None
            for rule in v["rules"]:
                p = rule.get("params", {})
                if rule["kind"] in ("PerPlayerVars", "PerPlayerState"):
                    for n in p.get("vars", []):
                        if have is None or n not in have:
                            out.append((q, rule["kind"], "vars", n))
                    for n in p.get("scripts", []):
                        if n not in self.scripts:
                            out.append((q, rule["kind"], "scripts", n))
                    for n in p.get("objvars", []):
                        sc, _, var = n.partition(".")
                        if sc not in self.scripts or var not in {x["name"] for x in self.scripts[sc]["script"].get("vars", [])}:
                            out.append((q, rule["kind"], "objvars", n))
        return out

    def test_per_player_variable_lists_name_variables_their_scripts_declare(self):
        self.assertEqual(self.var_problems(self.rules), [])

    def test_twin_a_made_up_variable_is_found(self):
        doc = json.loads(json.dumps(self.rules))
        for r in doc["NVDLC03MQ05"]["rules"]:
            if r["kind"] == "PerPlayerVars":
                r["params"]["vars"] = ["bRestoreBodyParts", "perks"]       # the lists wave 1 shipped
        doc["NVDLC01BombCollarQuest"]["rules"].append({"kind": "PerPlayerState", "params": {"scope": "x", "objvars": ["NVDLC01RadioSpeakerSCRIPT.bNoSuchFlag"], "scripts": ["NVDLC01NoSuchScript"]}})
        got = {(x[0], x[3]) for x in self.var_problems(doc)}
        self.assertEqual(got, {("NVDLC03MQ05", "bRestoreBodyParts"), ("NVDLC03MQ05", "perks"), ("NVDLC01BombCollarQuest", "NVDLC01RadioSpeakerSCRIPT.bNoSuchFlag"),
                               ("NVDLC01BombCollarQuest", "NVDLC01NoSuchScript")})

    # ---- stage scoping ---------------------------------------------------------------------------------------------------------------------
    def unscoped_holds(self, rules):
        out = []
        for q, v in rules.items():
            if not q.startswith("NVDLC") or q not in self.quests or not self.quests[q]["stages"]:
                continue
            have = {s["index"] for s in self.quests[q]["stages"]}
            for r in v["rules"]:
                if r["kind"] in ("Barrier", "PartyTether"):
                    st = r["params"].get("stages")
                    if not st:
                        out.append((q, r["kind"], "no stages list"))
                    elif not set(st) <= have:
                        out.append((q, r["kind"], "stages %s are not stages of the quest" % sorted(set(st) - have)))
        return out

    def test_no_dlc_barrier_or_tether_holds_every_stage_of_its_quest(self):
        self.assertEqual(self.unscoped_holds(self.rules), [])

    def test_twin_a_stage_less_hold_is_found(self):
        doc = json.loads(json.dumps(self.rules))
        for r in doc["NVDLC01MQ03a"]["rules"]:
            if r["kind"] == "PartyTether":
                r["params"].pop("stages")
        doc["NVDLC03MQ05"]["rules"].append({"kind": "Barrier", "params": {"step": "all-players-pass"}})
        got = self.unscoped_holds(doc)
        self.assertEqual({(x[0], x[1]) for x in got}, {("NVDLC01MQ03a", "PartyTether"), ("NVDLC03MQ05", "Barrier")})

    def test_stage_holds_table_matches_the_committed_rules(self):
        for quest, kind, stages, _why in self.W3.STAGE_HOLDS:
            rule = next(r for r in self.rules[quest]["rules"] if r["kind"] == kind)
            self.assertEqual(rule["params"]["stages"], stages, f"{quest}/{kind}")
        for quest, kind, _why in self.W3.DROPS:
            self.assertFalse([r for r in self.rules[quest]["rules"] if r["kind"] == kind], f"{quest}/{kind} should have been dropped")

    # ---- the cause map, kinds, ops ---------------------------------------------------------------------------------------------------------
    def test_cause_map_carries_the_set_piece_scripts(self):
        cm = common.load_json(os.path.join(common.REPO, "docs", "quest-policies", "cause_quests.json"))
        s = cm["scripts"]

        def key(edid):
            sc = self.scripts[edid]
            return self.refcheck.norm_ref(sc["ref"]).lower()
        for edid, (entry, _why) in self.W3.CAUSE.items():
            self.assertEqual(s[key(edid)], entry, edid)
            self.assertIn(entry, self.rules, edid)
        for edid in ("NVDLC03SinkMainComputerSCRIPT", "NVDLC03SinkMuggySCRIPT", "NVDLC03SinkBioResearchStationSCRIPT"):
            self.assertEqual(s[key(edid)], "NVDLC03HQBuddy", edid)
        self.assertEqual(sorted(k for k in cm["keys"] if k.startswith("NVDLC03")), ["NVDLC03AutoDoc", "NVDLC03Teleporter", "NVDLC03ThinkTankWeaponLock"])

    def test_the_new_params_validate_and_bad_ones_are_rejected(self):
        self.assertEqual(RO.validate(self.rules), [])
        for kind, params in (("PerPlayerState", {"scope": "c", "down": "forever"}), ("PerPlayerState", {"scope": "c", "objvars": ["NoDot"]}), ("PerPlayerState", {"scope": "c", "scripts": ["not an id"]}),
                             ("PerPlayerState", {"scope": "c", "checkpoint": "NotAList"}), ("PooledItems", {}), ("OncePerEvent", {}), ("OncePerEvent", {"effects": ["RemoveItem"]}),
                             ("OncePerEvent", {"globals": ["GameDay"], "window_s": 0}), ("PartyWarp", {"markers": ["M"], "range": 0})):
            self.assertTrue(RO.validate({"Q": {"rules": [{"kind": kind, "params": params}]}}), f"{kind} {params} should be rejected")

    def test_drop_mode_removes_a_rule_and_is_reported_until_applied(self):
        rules = {"Q": {"rules": [{"kind": "Barrier", "params": {"step": "x"}}, {"kind": "PartyTether", "params": {"cells": ["C"]}}]}}
        ops = [{"quest": "Q", "kind": "Barrier", "mode": "drop", "params": {}}]
        self.assertEqual(len(RO.held(rules, ops)), 1)
        self.assertEqual(RO.apply_ops(rules, ops), 1)
        self.assertEqual([r["kind"] for r in rules["Q"]["rules"]], ["PartyTether"])
        self.assertEqual(RO.held(rules, ops), [])
        self.assertEqual(RO.apply_ops(rules, ops), 0)

    # ---- the set pieces the backlog names have data -------------------------------------------------------------------------------------------
    def kinds(self, quest):
        out = {}
        for r in self.rules[quest]["rules"]:
            out.setdefault(r["kind"], []).append(r["params"])
        return out

    def test_dead_money_collar_cloud_vault_suites_kitchen_and_entry_are_per_player(self):
        c = self.kinds("NVDLC01BombCollarQuest")["PerPlayerState"][0]
        self.assertEqual(c["down"], "until-checkpoint")                 # D9: collar death is downed until a checkpoint
        self.assertEqual(len(c["vars"]), 18)
        for v in ("iNumRadii", "fCollarTimer", "bBoom"):
            self.assertIn(v, c["vars"])
        self.assertIn("NVDLC01RadioSpeakerSCRIPT.bInRadius", c["objvars"])
        for quest, script in (("NVDLC01MQ03", "NVDLC01VaultCollarBeepScript"), ("NVDLC01MQ03", "NVDLC01KitchenGasTrapSCRIPT"), ("NVDLC01MQ03c", "NVDLC01SuitesSetupSCRIPT"),
                              ("NVDLC01MQ00", "NVDLC01VillaTravelTriggerScript"), ("NVDLC01GlobalToxicDamage", "NVDLC01GlobalToxicDamageSCRIPT"), ("NVDLC01ToxicCloud", "NVDLC01ToxicCloudSCRIPT")):
            self.assertIn(script, self.kinds(quest)["PerPlayerState"][0]["scripts"], f"{quest} {script}")
        # the 70 s vault timer and the 16 s suites timer are the scripts' own constants: the data keeps one copy of their state per slot
        self.assertIn("NVDLC01VaultCollarBeepScript.fTimer", self.kinds("NVDLC01MQ03")["PerPlayerState"][0]["objvars"])
        self.assertIn("NVDLC01SuitesSetupSCRIPT.fCollarTimer", self.kinds("NVDLC01MQ03c")["PerPlayerState"][0]["objvars"])
        # the collar item is duplicated and equipped for everybody (a stage result: QuestItemDup + the RefCommand EquipItem of the player is a reward)
        self.assertEqual(self.kinds("NVDLC01MQ01")["QuestItemDup"][0], {"recipient": "all"})
        for cell in c["checkpoint"]:
            self.assertEqual(self.refs.resolve(cell)[1], "CELL", cell)

    def test_old_world_blues_organs_implants_autodoc_teleporter_and_ending_hold(self):
        ad = self.kinds("NVDLC03AutoDoc")
        self.assertEqual(ad["SetPlayerControls"][0]["scope"], "initiator")
        init = ad["PerkRecipient"][0]["initiator"]
        for edid in ("NVDLC03BrainlessPerk", "NVDLC03HeartlessPerk", "NVDLC03SpinelessPerk", "NVDLC03BigBrainedPerk", "NVDLC03CardiacArrestPerk", "NVDLC03ReinforcedSpinePerk"):
            self.assertIn(self.refs.ref_of(edid), init, edid)
        self.assertTrue([x for x in init if self.refs.resolve(x)[2].startswith("NVDLC03Implant")], "the implants")
        self.assertEqual(self.kinds("NVDLC03MQ05")["PerPlayerVars"][0]["vars"], ["bRestoredBrain", "bRestoredHeart", "bRestoredSpine"])
        self.assertIn("nImplantGRXRank", self.kinds("NVDLC03GenericPerkQUEST")["PerPlayerState"][0]["vars"])      # implants run per slot (GenericPerkQUEST)
        self.assertNotIn("PerPlayerVars", self.kinds("NVDLC03GenericPerkQUEST"))
        tp = self.kinds("NVDLC03Teleporter")["PartyWarp"][0]
        self.assertEqual(tp["range"], self.W3.TELEPORTER_RANGE)
        self.assertEqual(sorted(tp["markers"]), ["MojaveDriveInMarkerHeadingREF", "NVDLC03SinkArrivalMarker"])
        sat = next(p for p in self.kinds("NVDLC03MQ00")["PartyTether"] if p.get("anchor"))
        self.assertEqual((sat["gather"], sat["timeout_s"], sat["stages"]), ("auto-warp", 120, [100]))       # D10: wait up to a timeout, then warp the rest
        self.assertEqual(self.kinds("NVDLC03MQ05")["PartyTether"][0]["stages"], [60, 70])                       # the Think Tank ending decision waits for everybody

    def test_lonesome_road_vote_party_ede_clock_and_weak_player_scaling(self):
        boxes = self.kinds("NVDLC04MQ03b")["ChoiceConsent"][0]["boxes"]
        for ref in ("LonesomeRoad.esm:009C99", "LonesomeRoad.esm:009C98", "LonesomeRoad.esm:00BF02"):
            self.assertIn(ref, boxes)
        ede = self.refs.ref_of("NVDLC04EDEREF")
        for quest in ("NVDLC04MQ01a", "NVDLC04MQ03a", "NVDLC04MQ03b", "NVDLC04MQ04", "NVDLC04DialogueEDE", "NVDLC04EDEFinaleQuest"):
            co = self.kinds(quest)["CompanionOwner"]
            self.assertTrue(any(p.get("owner") == "party" for p in co), quest)
        for quest in ("NVDLC04MQ01a", "NVDLC04MQ03a", "NVDLC04MQ03b", "NVDLC04MQ04"):
            self.assertEqual([p for p in self.kinds(quest)["CompanionOwner"] if p.get("owner") == "party"][0]["npcs"], [ede])
        once = self.kinds("NVDLC04MQ00")["OncePerEvent"][0]
        self.assertEqual(sorted(self.refs.resolve(g)[2] for g in once["globals"]), ["GameDay", "GameHour", "GameMonth", "GameYear"])
        self.assertEqual(self.kinds("NVDLC04MQ03a")["PartyAwareCond"][0]["funcs"]["GetActorValue"], "max-over-party")
        self.assertEqual(self.kinds("NVDLC04MQ03b")["PartyTether"][0]["stages"], [30, 255])

    def test_honest_hearts_gate_ambush_fork_pools_and_the_clock(self):
        self.assertEqual(self.kinds("NVDLC02DialogueCaravanMaster")["PartyAwareCond"][0]["funcs"]["GetActorValue"], "max-over-party")
        self.assertEqual(self.kinds("NVDLC02MQ00")["PartyAwareCond"][0]["funcs"]["IsKiller"], "any-party-member")
        self.assertEqual(self.kinds("NVDLC02MQ00")["ExclusiveStages"][0]["stages"], [100, 255])
        self.assertTrue(self.kinds("NVDLC02MQ00")["FriendlyFireGuard"][0]["actors"])
        topics = self.kinds("NVDLC02DialogueDaniel")["ChoiceConsent"][0]["topics"]
        for t in ("HonestHearts.esm:010718", "HonestHearts.esm:010723", "HonestHearts.esm:0093AA", "HonestHearts.esm:01049F"):
            self.assertIn(t, topics)
        for quest, item in (("NVDLC02MQ03a", "NVDLC02WhiteLegTotem"), ("NVDLC02MS01", "NVDLC02SacredDaturaRoot"), ("NVDLC02MQ02c", "NVDLC02LilScoutLunch"),
                            ("NVDLC02MQ02b", "NVDLC02WalkieTalkie"), ("NVDLC02DialogueShaman", "NVDLC02SacredDaturaRoot"), ("NVDLC02DialogueDaniel", "NVDLC02LilScoutLunch")):
            self.assertIn(self.refs.ref_of(item), self.kinds(quest)["PooledItems"][0]["items"], f"{quest} {item}")
            if quest != "NVDLC02DialogueDaniel":      # Daniel's INFO only removes the items; the count is read in the quests above
                self.assertEqual(self.kinds(quest)["PartyAwareCond"][0]["funcs"]["GetItemCount"], "sum-over-party", quest)
        self.assertIn("OncePerEvent", self.kinds("NVDLC02MQ00"))

    def test_derived_ops_equal_the_committed_overlay_for_the_wave3b_tables(self):
        ov = RO.load_overlay()
        derived = {json.dumps(o, sort_keys=True) for o in self.W3.ops(self.rules, gen_plans.QData(), self.refs)}
        committed = {json.dumps(o, sort_keys=True) for o in ov["ops"]}
        self.assertEqual(derived - committed, set())


if __name__ == "__main__":
    unittest.main(verbosity=1)
