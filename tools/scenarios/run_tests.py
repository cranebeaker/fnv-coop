#!/usr/bin/env python3
"""Game-free tests for the scenario tool, plus the mutation twin.  Run: python -I tools/scenarios/run_tests.py   (prints ALL OK).

Part 1: unit tests (validator, translation, comparison, jump planning, runner against tools/scenarios/fakegame.py).
Part 2: twin. The same unit tests are run again under each SCEN_MUTANT value; every mutant must make at least one test fail.
"""
import argparse
import contextlib
import copy
import io
import json
import os
import subprocess
import sys
import tempfile
import time
import types
import unittest
import unittest.mock
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parents[1] / "tools" / "pool"))
# Runners given game process keys look for prepared saves in the instance folders: keep these tests off the real ones.
_INST_ROOT = tempfile.TemporaryDirectory()
os.environ["FNV_INSTANCES_ROOT"] = _INST_ROOT.name
import scenlib  # noqa: E402
import runner  # noqa: E402
import prepare_save  # noqa: E402
import fakegame  # noqa: E402
import proof  # noqa: E402
import scen  # noqa: E402
import backend as pool_backend  # noqa: E402  (tools/pool: the pool policy on its in-memory fake backend; no game)
import core as pool_core  # noqa: E402
import client as pool_client  # noqa: E402

SCN = scenlib.SCEN_DIR
ORDER = scenlib.load_order()


def load(name):
    return scenlib.load_scenario(SCN / (name + ".json"))


def hexof(sc, alias):
    return scenlib.Resolver(sc, ORDER).hex("$" + alias)


def prime(w):
    """Give a fake world the refs and quests the shipped scenarios use."""
    w.bases = {hexof(load("loot_race"), "teddyRef"): hexof(load("loot_race"), "teddy")}
    nov = hexof(load("ownership_handoff"), "novac")
    w.flags.setdefault("novac", nov)
    w.flags.setdefault("vcg02", hexof(load("quest_stage_sync"), "vcg02"))
    w.flags.setdefault("vmq05", hexof(load("pnr_vote"), "vmq05"))


def run_pair(names, caps=None, **flags):
    """Run scenarios in order on one fake 2-player world. Returns (world, results, nvs)."""
    w, clock, nvs = fakegame.make(2, **flags)
    prime(w)
    r = runner.Runner(nvs, runner.StateStore(), capabilities=caps, sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER)
    results = [r.run(load(n)) for n in names]
    return w, results, r


class FakePoolLease(pool_client.Lease):
    """The real client.Lease, built from a lease of a real core.Pool on the fake backend. bridge() gives fakegame bridges."""

    def __init__(self, data, nvs):
        super().__init__(data)
        self.nvs = nvs

    def bridge(self, i=None):
        i = self.instances[0] if i is None else i
        return self.nvs[self.instances.index(i)]


class PoolRun:
    """One pool start on the fake machine (tools/pool core + FakeBackend). Its games start with nothing loaded, as a real
    fresh game does. lease() has the signature of client.lease, so proof.py and scen.py can use it."""

    def __init__(self, size=2):
        self.pool = pool_core.Pool(pool_backend.FakeBackend(), pool_backend.FakeSlots(), size=size, sync=True)
        self.pool.up()
        self.world, self.clock, self.nvs = fakegame.make(size)
        prime(self.world)

    @contextlib.contextmanager
    def lease(self, owner, n=1, ttl_s=900, timeout_s=600):
        d = self.pool.lease(owner, n, ttl_s, timeout_s)
        try:
            yield FakePoolLease(d, self.nvs[:n])
        finally:
            self.pool.release(d["id"])

    def kw(self):
        return dict(sleep=self.clock.sleep, now=self.clock.now, log=lambda *a: None, order=ORDER)

    def down(self):
        self.pool.down()


NOTHING_LOADED = "no save loaded on this instance yet"


ALL_CAPS = set(scenlib.CAPABILITIES)


class ShippedScenarios(unittest.TestCase):
    def test_ten_starters_exist_and_are_valid(self):
        names = {p.stem for p in scenlib.list_scenarios()}
        want = {"goodsprings_fight", "loot_race", "spawn_once", "ownership_handoff", "door_terminal_container_matrix",
                "companion_recruit", "quest_stage_sync", "save_load_rejoin", "vats_hit", "fast_travel_tether"}
        self.assertTrue(want <= names, want - names)
        for p in scenlib.list_scenarios():
            self.assertEqual(scenlib.validate_file(p), [], p.name)

    def test_every_form_in_every_scenario_resolves_with_matching_edid_and_sig(self):
        forms = scenlib.load_forms()
        n = 0
        for p in scenlib.list_scenarios():
            for alias, d in load(p.stem)["forms"].items():
                if d["sig"] in ("DIAL", "INFO"):
                    continue    # resolved against dials/infos by the validator (tests below)
                rec = forms.get(d["ref"]) or {"sig": scenlib.BUILTIN.get(d["ref"], (None, None))[0],
                                                "edid": scenlib.BUILTIN.get(d["ref"], (None, None))[1]}
                self.assertEqual((rec["edid"], rec["sig"]), (d["edid"], d["sig"]), "%s.%s" % (p.stem, alias))
                n += 1
        self.assertGreaterEqual(n, 20)

    def test_runtime_hex_uses_the_load_order(self):
        self.assertEqual(scenlib.runtime_hex("FalloutNV.esm:10A214", ORDER), "0010A214")
        self.assertEqual(scenlib.runtime_hex("DeadMoney.esm:00ECE0", ORDER), "0100ECE0")
        self.assertEqual(scenlib.runtime_hex("OldWorldBlues.esm:00D4BF", ORDER), "0300D4BF")


class ValidatorRejects(unittest.TestCase):
    """Each mutation of a good scenario must be reported."""

    def base(self):
        return copy.deepcopy(load("quest_stage_sync"))

    def errs(self, sc):
        return scenlib.validate(sc, name=sc.get("name"))

    def test_base_is_clean(self):
        self.assertEqual(self.errs(self.base()), [])

    def expect(self, mut, text):
        sc = self.base()
        mut(sc)
        errs = self.errs(sc)
        self.assertTrue(any(text in e for e in errs), "wanted %r in %s" % (text, errs))

    def test_invented_formid(self):
        self.expect(lambda s: s["forms"]["vcg02"].update(ref="FalloutNV.esm:ABCDEF"), "does not resolve")

    def test_wrong_edid(self):
        self.expect(lambda s: s["forms"]["vcg02"].update(edid="VCG01"), "edid")

    def test_wrong_record_type(self):
        self.expect(lambda s: s["forms"]["vcg02"].update(sig="NPC_"), "sig")

    def test_unnormalized_ref(self):
        self.expect(lambda s: s["forms"]["vcg02"].update(ref="FalloutNV.esm:10a214"), "normalized")

    def test_plugin_not_in_load_order(self):
        self.expect(lambda s: s["forms"]["vcg02"].update(ref="Nope.esm:10A214"), "load order")

    def test_unknown_alias_in_step(self):
        self.expect(lambda s: s["steps"][1].update(quest="$nothere"), "unknown alias")

    def test_raw_ref_that_does_not_exist(self):
        self.expect(lambda s: s["steps"][1].update(quest="FalloutNV.esm:ABCDEF"), "does not resolve")

    def test_stage_not_in_quest(self):
        self.expect(lambda s: s["steps"][1].update(stage=77), "not a stage")

    def test_var_not_in_quest_script(self):
        sc = self.base()
        v = [x for x in sc["steps"] if x["op"] == "setvar"][0]
        v["var"] = "nNoSuchVar"
        self.assertTrue(any("script variable" in e for e in self.errs(sc)))

    def test_wrong_sig_for_field(self):
        def m(s):
            s["forms"]["gangBase"] = {"ref": "FalloutNV.esm:104DF1", "edid": "GSPGHM2", "sig": "NPC_"}
            s["steps"][1] = {"id": "bad", "op": "moveto", "who": "player", "to": "$gangBase"}
        self.expect(m, "expected one of")

    def test_no_checks(self):
        self.expect(lambda s: s.update(steps=[x for x in s["steps"] if x["op"] != "check"] or [{"id": "w", "op": "wait", "seconds": 1}]),
                    "no check")

    def test_duplicate_step_id(self):
        self.expect(lambda s: s["steps"][1].update(id=s["steps"][0]["id"]), "duplicate id")

    def test_unknown_op_and_key(self):
        self.expect(lambda s: s["steps"][1].update(op="teleport"), "unknown op")
        self.expect(lambda s: s["steps"][1].update(color="red"), "unexpected key")

    def test_bad_save_name(self):
        def m(s):
            s["steps"].insert(0, {"id": "sv", "op": "save", "name": "mysave"})
        self.expect(m, "autosave")

    def test_raw_formid_in_console(self):
        def m(s):
            s["steps"].insert(0, {"id": "c", "op": "console", "cmd": "00104C1C.kill"})
        self.expect(m, "raw FormID")

    def test_reference_call_not_first_in_console_or_expr(self):
        """'{alias}.Func' must be the first word (it becomes exec's calling reference); one per line."""
        def m(s):
            s["steps"].insert(0, {"id": "c", "op": "console", "cmd": "printc \"%g\" ({vcg02}.GetDead)"})
        self.expect(m, "first word")

        def m2(s):
            s["steps"].append({"id": "c2", "op": "check", "kind": "expr", "expr": "{vcg02}.GetAV Health + {vcg02}.GetDead",
                               "value": 1})
        self.expect(m2, "first word")
        sc = self.base()
        sc["steps"].insert(0, {"id": "c", "op": "console", "cmd": "{vcg02}.setav Health 100"})
        self.assertFalse([e for e in self.errs(sc) if "first word" in e])

    def test_unknown_capability_and_player_count(self):
        self.expect(lambda s: s["steps"][1].update(needs=["warp_drive"]), "unknown capability")
        self.expect(lambda s: s["steps"][1].update(on=5), "bad 'on'")

    def test_missing_start_save_and_reset(self):
        self.expect(lambda s: s["start"].pop("save"), "start.save")
        self.expect(lambda s: s.pop("reset"), "reset must")

    def test_check_without_value(self):
        def m(s):
            c = [x for x in s["steps"] if x["op"] == "check"][0]
            c.pop("value", None)
        self.expect(m, "needs a 'value'")

    def test_all_shipped_scenarios_reject_a_corrupted_ref(self):
        for p in scenlib.list_scenarios():
            sc = load(p.stem)
            first = next(iter(sc["forms"]))
            sc["forms"][first]["ref"] = "FalloutNV.esm:FFFFF1"
            self.assertTrue(any("does not resolve" in e for e in scenlib.validate(sc, name=sc["name"])), p.stem)


class DialogueAndMessages(unittest.TestCase):
    def test_info_and_dial_refs_resolve_and_are_checked(self):
        sc = copy.deepcopy(load("dialogue_spectator"))
        self.assertEqual(scenlib.validate(sc, name="dialogue_spectator"), [])
        bad = copy.deepcopy(sc)
        bad["forms"]["info"]["line"] = "Something else"
        self.assertTrue(any("INFO line" in e for e in scenlib.validate(bad, name="dialogue_spectator")))
        bad = copy.deepcopy(sc)
        bad["forms"]["info"]["ref"] = "FalloutNV.esm:FFFFF1"
        self.assertTrue(any("does not resolve" in e for e in scenlib.validate(bad, name="dialogue_spectator")))
        bad = copy.deepcopy(sc)
        bad["forms"]["topic"]["edid"] = "Nope"
        self.assertTrue(any("edid" in e for e in scenlib.validate(bad, name="dialogue_spectator")))

    def test_translation(self):
        r = scenlib.Resolver(load("pnr_vote"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "message_click", "button": 1}, r),
                         [("call", "bot.menu.press", {"button": 1})])
        r = scenlib.Resolver(load("dialogue_spectator"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "pick", "topic": "$topic", "info": "$info"}, r),
                         [("call", "bot.say", {"info": "FalloutNV.esm:10A1E5"})])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "console", "cmd": "SetObjectiveCompleted {vcg02} 10 1"}, r),
                         [("exec", "SetObjectiveCompleted 0010A214 10 1")])

    def test_consent_translation_and_vats_names_follow_t6a(self):
        r = scenlib.Resolver(load("pnr_vote"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "consent", "answer": "notyet"}, r),
                         [("call", "bot.consent", {"answer": "notyet"})])
        g = scenlib.Resolver(load("vats_hit"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "vats_hit", "target": "$gang1", "limb": "Head"}, g),
                         [("call", "bot.vats", {"ref": "00104C70"})])

    def test_pnr_vote_makes_the_next_jump_reload(self):
        self.assertIn("*", scenlib.footprint(load("pnr_vote"))["dirty"])


class Translation(unittest.TestCase):
    def test_actions(self):
        sc = load("door_terminal_container_matrix")
        res = scenlib.Resolver(sc, ORDER)
        step = {"id": "x", "op": "lock", "ref": "$docDoor", "level": 50}
        # a line for a reference names it as exec's calling reference: the game's console has no "<hex>.Func" form
        self.assertEqual(scenlib.step_actions(step, res), [("exec", "lock 50", "00103E61")])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "additem", "who": "$dresser", "item": "$stimpak", "count": 3}, res),
                         [("exec", "additem 00015169 3", "00103E37")])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "additem", "who": "player", "item": "$stimpak", "count": 3}, res),
                         [("exec", "player.additem 00015169 3")])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "moveto", "who": "player", "to": "$docDoor"}, res),
                         [("exec", "player.moveto 00103E61")])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "recruit", "ref": "$docDoor"}, res),
                         [("exec", "enable", "00103E61"), ("exec", "moveto player", "00103E61"),
                          ("exec", "setplayerteammate 1", "00103E61")])
        q = scenlib.Resolver(load("quest_stage_sync"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "setstage", "quest": "$vcg02", "stage": 10}, q)[0],
                         ("exec", "setstage 0010A214 10"))
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "setvar", "quest": "$vcg02", "var": "nGeckosKilled", "value": 2}, q),
                         [("call", "test.quest.setvar", {"quest": "0010A214", "var": "nGeckosKilled", "value": 2.0})])
        g = scenlib.Resolver(load("vats_hit"), ORDER)
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "console", "cmd": "{gang1}.setav Health 100"}, g),
                         [("exec", "setav Health 100", "00104C70")])
        self.assertEqual(scenlib.step_actions({"id": "x", "op": "console", "cmd": "{player}.setav Health 100"}, g),
                         [("exec", "player.setav Health 100")])
        with self.assertRaises(scenlib.StepError):     # the console cannot run a reference call inside a line
            scenlib.step_actions({"id": "x", "op": "console", "cmd": "printc \"%g\" ({gang1}.GetDead)"}, g)

    def test_check_expressions(self):
        sc = load("goodsprings_fight")
        res = scenlib.Resolver(sc, ORDER)
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "dead", "ref": "$gang1", "value": 1}, res)
        self.assertEqual(e[:3], ("GetDead", "==", 1))
        self.assertEqual(e[6], "00104C70")          # evaluated with the actor as the calling reference
        res2 = scenlib.Resolver(load("quest_stage_sync"), ORDER)
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "stage_done", "quest": "$vcg02", "stage": 20, "value": 1}, res2)
        self.assertEqual((e[0], e[6]), ("GetStageDone 0010A214 20", None))
        d = scenlib.Resolver(load("door_terminal_container_matrix"), ORDER)
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "item_count", "who": "player", "item": "$stimpak", "value": 1}, d)
        self.assertEqual((e[0], e[6]), ("player.GetItemCount 00015169", None))
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "item_count", "who": "$dresser", "item": "$stimpak", "value": 1}, d)
        self.assertEqual((e[0], e[6]), ("GetItemCount 00015169", "00103E37"))
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "distance", "a": "$gang1", "b": "player", "value": 1}, res)
        self.assertEqual((e[0], e[6]), ("GetDistance player", "00104C70"))
        e = scenlib.check_expr({"id": "c", "op": "check", "kind": "expr", "expr": "{gang1}.GetAV Health", "value": 1}, res)
        self.assertEqual((e[0], e[6]), ("GetAV Health", "00104C70"))

    def test_no_console_line_names_a_reference_with_a_hex_prefix(self):
        """Every action and check of every shipped scenario: no '<hex>.Func' text goes to the console."""
        import re
        bad = []
        for p in scenlib.list_scenarios():
            sc = scenlib.load_scenario(p)
            res = scenlib.Resolver(sc, ORDER)
            for st in scenlib.all_steps(sc):
                if st["op"] == "check":
                    texts = [scenlib.check_expr(st, res)[0] or ""]
                else:
                    texts = [a[1] for a in scenlib.step_actions(st, res) if a[0] == "exec"]
                bad += ["%s/%s: %s" % (p.stem, st["id"], t) for t in texts if re.match(r"\s*[0-9A-Fa-f]{8}\.\w", t)]
        self.assertEqual(bad, [])

    def test_fake_console_rejects_hex_prefixes_like_the_game(self):
        w, clock, nvs = fakegame.make(2)
        r = nvs[0].exec("00104C70.kill")
        self.assertEqual(r, {"ok": False, "output": 'Script command "00104C70.kill" not found'})
        self.assertNotIn("00104C70", w.dead)
        with self.assertRaises(RuntimeError):
            nvs[0].eval("00104C70.GetDead")
        self.assertEqual(nvs[0].exec("kill", ref="00104C70"), {"ok": True, "output": ""})
        self.assertEqual(nvs[0].eval("GetDead", ref="00104C70"), 1.0)

    def test_proof_probes_name_the_reference_in_ref(self):
        w, clock, nvs = fakegame.make(1)
        prime(w)
        rows = proof.probe_rows(nvs[0])
        self.assertEqual([r for r in rows if not r["ok"]], [], "every probe form must run on the game-like fake console")
        self.assertGreaterEqual(len([r for r in rows if r["ref"]]), 7)
        self.assertEqual([r for r in rows if r["ref"] and r["expr"].startswith(r["ref"])], [])

    def test_host_that_ignores_the_ref_is_caught(self):
        """Twin of the bridge: a host that drops exec's ref runs the line with no calling reference."""
        _, results, _ = run_pair(["goodsprings_fight"], exec_ref_ignored=True)
        self.assertNotEqual(results[0]["status"], "pass")
        _, results, _ = run_pair(["goodsprings_fight"])
        self.assertEqual(results[0]["status"], "pass")

    def test_compare_table(self):
        c = scenlib.compare
        self.assertTrue(c([1, 1], "==", 1, None, 0))
        self.assertFalse(c([1, 0], "==", 1, None, 0))
        self.assertTrue(c([1, 0], "==", 1, "sum", 0))
        self.assertFalse(c([1, 1], "==", 1, "sum", 0))      # duplicated loot
        self.assertFalse(c([0, 0], "==", 1, "sum", 0))      # vanished loot
        self.assertTrue(c([3, 3.5], "same", None, None, 1))
        self.assertFalse(c([3, 9], "same", None, None, 1))
        self.assertTrue(c([5, 6], "<=", 6, "max", 0))
        self.assertTrue(c([50], "<", 100, None, 0))
        self.assertTrue(c([99.5], "==", 100, None, 1))


class JumpPlanning(unittest.TestCase):
    def test_rules(self):
        fight, quest = load("goodsprings_fight"), load("quest_stage_sync")
        st = {"loaded": True, "save": "autosave", "dirty": [], "failed": False}
        self.assertFalse(scenlib.plan_jump(st, quest)["reload"])
        self.assertTrue(scenlib.plan_jump({"loaded": False}, quest)["reload"])
        self.assertTrue(scenlib.plan_jump(dict(st, save="other"), quest)["reload"])
        self.assertTrue(scenlib.plan_jump(dict(st, failed=True), quest)["reload"])
        self.assertTrue(scenlib.plan_jump(dict(st, dirty=["*"]), quest)["reload"])
        d = sorted(scenlib.footprint(fight)["dirty"])
        self.assertTrue(scenlib.plan_jump(dict(st, dirty=d), fight)["reload"])           # same gang again
        self.assertFalse(scenlib.plan_jump(dict(st, dirty=d), quest)["reload"])          # unrelated refs
        self.assertTrue(scenlib.plan_jump(st, quest, force_reload=True)["reload"])

    def test_load_op_makes_the_world_unnameable(self):
        self.assertIn("*", scenlib.footprint(load("save_load_rejoin"))["dirty"])

    def test_reset_ids_reuse_the_l7_goodsprings_group(self):
        ids = scenlib.reset_ids(load("quest_stage_sync"), ORDER)
        self.assertEqual(ids[:6], ["00104C1C", "0010A214", "0015D912", "00104EAE", "0015EC5B", "00102037"])


class RunnerOnFake(unittest.TestCase):
    def status(self, results, name):
        return [r for r in results if r["scenario"] == name][0]

    def test_synced_world_passes_every_runnable_scenario(self):
        names = ["goodsprings_fight", "loot_race", "spawn_once", "door_terminal_container_matrix", "companion_recruit",
                 "quest_stage_sync", "vats_hit", "fast_travel_tether", "dialogue_spectator", "pnr_vote", "pnr_vote_guest_declines"]
        _, results, _ = run_pair(names)
        for r in results:
            self.assertEqual(r["status"], "pass", (r["scenario"], [s for s in r["steps"] if s["status"] != "pass"]))

    def test_two_player_scenario_on_one_copy_moves_only_that_copy(self):
        """proof tests on ONE copy load goodsprings_enemies (players: 2): steps on 'all'/'guest' must not index a copy that is not there."""
        w, clock, nvs = fakegame.make(1)
        prime(w)
        r = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER)
        sc = load("goodsprings_enemies")
        self.assertEqual(sc["players"], 2)
        j = r.jump(sc)
        bad = [s for s in j["setup"] if s["status"] in ("error", "blocked")]
        self.assertEqual(bad, [], bad)
        self.assertTrue(any(s["id"] == "players_to_gang" and s["status"] == "pass" for s in j["setup"]))

    def test_check_that_needs_a_missing_player_errors_not_passes(self):
        w, clock, nvs = fakegame.make(1)
        prime(w)
        r = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER)
        sc = copy.deepcopy(load("quest_stage_sync"))
        st = [s for s in sc["steps"] if s["op"] == "check"][0]
        st["on"] = "guest"
        rec = r._step(sc, st, scenlib.Resolver(sc, ORDER))
        self.assertEqual(rec["status"], "error", rec)

    def test_proposed_capabilities_are_blocked_not_passed(self):
        _, results, _ = run_pair(["ownership_handoff", "save_load_rejoin"])
        for r in results:
            self.assertEqual(r["status"], "partial", (r["scenario"], r["status"]))
            self.assertTrue(any(s["status"] == "blocked" for s in r["steps"]))
            self.assertFalse(any(s["status"] in ("fail", "error") for s in r["steps"]), r["steps"])

    def test_with_the_capabilities_present_they_pass_too(self):
        _, results, _ = run_pair(["vats_hit", "fast_travel_tether", "ownership_handoff", "save_load_rejoin", "dialogue_spectator", "pnr_vote", "pnr_vote_guest_declines"], caps=ALL_CAPS)
        for r in results:
            self.assertEqual(r["status"], "pass", (r["scenario"], [s for s in r["steps"] if s["status"] != "pass"]))

    def test_guest_blind_world_fails_the_sync_checks(self):
        _, results, _ = run_pair(["quest_stage_sync", "goodsprings_fight"], desync_guest=True)
        for r in results:
            self.assertEqual(r["status"], "fail", r["scenario"])

    def test_loot_duplication_is_caught(self):
        _, results, _ = run_pair(["loot_race"], dup_loot=True)
        r = results[0]
        self.assertEqual(r["status"], "fail")
        self.assertTrue(any(s["id"] == "exactly_one_copy" and s["status"] == "fail" for s in r["steps"]))

    def test_double_spawn_is_caught(self):
        _, results, _ = run_pair(["spawn_once"], double_spawn=True)
        self.assertEqual(results[0]["status"], "fail")

    def test_missing_ownership_handoff_is_caught(self):
        _, results, _ = run_pair(["ownership_handoff"], caps=ALL_CAPS, no_handoff=True)
        self.assertEqual(results[0]["status"], "fail")

    def test_steps_run_on_the_named_player_only(self):
        w, _, _ = run_pair(["goodsprings_fight"])
        kills = [(i, t) for i, k, t in w.calls if k == "exec" and t.startswith("kill")]
        self.assertIn((1, "kill @00104C77"), kills)      # guest_kills_2 (exec "kill" with ref 00104C77)
        self.assertNotIn((0, "kill @00104C77"), kills)
        self.assertIn((0, "kill @00104C70"), kills)
        self.assertNotIn((1, "kill @00104C70"), kills)

    def test_unavailable_check_times_out_with_a_detail(self):
        w, clock, nvs = fakegame.make(2)
        sc = copy.deepcopy(load("quest_stage_sync"))
        sc["steps"] = [{"id": "never", "op": "check", "kind": "stage", "quest": "$vcg02", "value": 55, "timeout": 2}]
        r = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, order=ORDER).run(sc)
        self.assertEqual(r["status"], "fail")
        self.assertIn("not met after", r["steps"][-1]["detail"])

    def test_console_parse_error_is_an_error_not_a_pass(self):
        w, clock, nvs = fakegame.make(2)
        sc = copy.deepcopy(load("quest_stage_sync"))
        sc["forms"]["gang1"] = {"ref": "FalloutNV.esm:104C70", "edid": "GSPG01Ref", "sig": "ACHR"}
        sc["steps"] = [{"id": "bad", "op": "console", "cmd": "bogus {gang1}"},
                       {"id": "c", "op": "check", "kind": "stage", "quest": "$vcg02", "value": 0}]
        r = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, order=ORDER).run(sc)
        self.assertEqual(r["status"], "error")

    def test_invalid_scenario_is_not_run(self):
        w, clock, nvs = fakegame.make(2)
        sc = copy.deepcopy(load("quest_stage_sync"))
        sc["forms"]["vcg02"]["ref"] = "FalloutNV.esm:ABCDEF"
        r = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, order=ORDER).run(sc)
        self.assertEqual(r["status"], "invalid")
        self.assertEqual(w.calls, [])


class InstantJump(unittest.TestCase):
    def loads(self, w):
        return len(w.loads)

    def test_second_unrelated_scenario_jumps_in_place(self):
        w, results, _ = run_pair(["quest_stage_sync", "goodsprings_fight", "door_terminal_container_matrix"])
        self.assertEqual([r["jump"]["reload"] for r in results], [True, False, False])
        self.assertEqual(self.loads(w), 2)                 # only the first jump loaded the save (once per player)
        self.assertTrue(all(r["status"] == "pass" for r in results))

    def test_conflicting_scenario_reloads(self):
        w, results, _ = run_pair(["goodsprings_fight", "ownership_handoff"], caps=ALL_CAPS)
        self.assertTrue(results[1]["jump"]["reload"])
        self.assertTrue(any("changed refs" in x for x in results[1]["jump"]["reasons"]))

    def test_repeating_a_scenario_that_killed_actors_reloads(self):
        w, results, _ = run_pair(["goodsprings_fight", "goodsprings_fight"])
        self.assertTrue(results[1]["jump"]["reload"])
        self.assertEqual(results[1]["status"], "pass")     # and it passes because the world was restored

    def test_failed_scenario_forces_a_reload_next(self):
        w, results, _ = run_pair(["quest_stage_sync", "quest_stage_sync"], desync_guest=True)
        self.assertEqual(results[0]["status"], "fail")
        self.assertTrue(results[1]["jump"]["reload"])

    def test_inplace_jump_resets_quest_state(self):
        w, results, r = run_pair(["quest_stage_sync", "door_terminal_container_matrix"])
        self.assertEqual(w.stage.get("0010A214", 0), 0)    # VCG02 was reset by the in-place jump

    def test_in_place_jump_is_cheap_in_bridge_calls(self):
        w, clock, nvs = fakegame.make(2)
        w.bases = {}
        rn = runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, order=ORDER)
        rn.run(load("quest_stage_sync"))
        n0 = len(w.calls)
        j = rn.jump(load("door_terminal_container_matrix"))
        self.assertFalse(j["reload"])
        self.assertLess(len(w.calls) - n0, 20)
        self.assertLess(j["seconds"], 10)

    def test_state_survives_in_a_store_file(self):
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "s.json"
            s = runner.StateStore(p)
            s.put("0@1", {"loaded": True, "save": "autosave", "dirty": [], "failed": False})
            self.assertEqual(runner.StateStore(p).get("0@1")["save"], "autosave")
            self.assertIsNone(runner.StateStore(p).get("0@2"))      # new generation = nothing loaded


class GameProcessKeys(unittest.TestCase):
    """The state store is keyed by game process. A new game process must reload the save, also after a pool restart where
    the old instance@generation key repeats. The same live game keeps its key, so a later lease still jumps in place."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.state = Path(self.tmp.name) / "state.json"

    def tearDown(self):
        self.tmp.cleanup()

    def test_new_game_process_with_the_same_old_key_reloads(self):
        a = PoolRun()
        with a.lease("t", n=2) as L:
            first = runner.for_lease(L, self.state, **a.kw()).run(load("quest_stage_sync"))
            old_gens = dict(L._gens)
        self.assertTrue(first["jump"]["reload"])
        with a.lease("t", n=2) as L:          # same live games, next lease (a second CLI run): in place, in seconds
            same = runner.for_lease(L, self.state, **a.kw()).run(load("door_terminal_container_matrix"))
        self.assertFalse(same["jump"]["reload"], same["jump"])
        self.assertEqual(len(a.world.loads), 2)
        a.down()
        b = PoolRun()                           # pool restart: new games at the main menu, generation 1 again
        with b.lease("t", n=2) as L:
            self.assertEqual(L._gens, old_gens)  # so the old key instance@generation is the same as before
            # unrelated to what the last scenario dirtied: with the old key this jump went in place on an empty game
            r = runner.for_lease(L, self.state, **b.kw()).run(load("quest_stage_sync"))
        b.down()
        self.assertTrue(r["jump"]["reload"], r["jump"])
        self.assertIn(NOTHING_LOADED, r["jump"]["reasons"])
        self.assertEqual(sorted(b.world.loads), [(0, "autosave"), (1, "autosave")])
        self.assertEqual(r["status"], "pass")

    def test_lease_without_a_process_id_reloads_first(self):
        # an older pool reports no process id; the store holds what the old keying wrote for an earlier game
        store = runner.StateStore(self.state)
        for k in ("0@1", "1@1"):
            store.put(k, {"loaded": True, "save": "autosave", "dirty": [], "failed": False, "quest_hex": [],
                          "scenario": "quest_stage_sync"})
        w, clock, nvs = fakegame.make(2)
        L = types.SimpleNamespace(instances=[0, 1], _gens={0: 1, 1: 1}, bridge=lambda i: nvs[i])
        keys = runner.StateStore.lease_keys(L)
        self.assertNotEqual(keys, runner.StateStore.lease_keys(L))
        rn = runner.for_lease(L, self.state, sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER)
        j = rn.jump(load("door_terminal_container_matrix"))
        self.assertTrue(j["reload"], j)
        self.assertIn(NOTHING_LOADED, j["reasons"])
        self.assertFalse(rn.jump(load("quest_stage_sync"))["reload"])   # later jumps of the same run go in place

    def test_runner_without_keys_never_borrows_old_state(self):
        w1, c1, nvs1 = fakegame.make(2)
        runner.Runner(nvs1, runner.StateStore(self.state), sleep=c1.sleep, now=c1.now, log=lambda *a: None,
                      order=ORDER).run(load("quest_stage_sync"))
        w2, c2, nvs2 = fakegame.make(2)       # another game, same store file, no keys given
        j = runner.Runner(nvs2, runner.StateStore(self.state), sleep=c2.sleep, now=c2.now, log=lambda *a: None,
                          order=ORDER).jump(load("door_terminal_container_matrix"))
        self.assertTrue(j["reload"], j)
        self.assertEqual(len(w2.loads), 2)

    def test_proof_jump_keys_by_game_process(self):
        a = PoolRun()
        rows = proof.mode_jump(a.lease, self.state, **a.kw())
        self.assertTrue(rows[0]["reload"])
        rows = proof.mode_jump(a.lease, self.state, **a.kw())      # same live game: the first jump stays in place
        self.assertFalse(rows[0]["reload"], rows[0])
        a.down()
        b = PoolRun()
        rows = proof.mode_jump(b.lease, self.state, **b.kw())      # new game process: the first jump must reload
        b.down()
        self.assertTrue(rows[0]["reload"], rows[0])
        self.assertIn(NOTHING_LOADED, rows[0]["reasons"])
        self.assertIn((0, "autosave"), b.world.loads)

    def test_proof_suite_keys_by_game_process(self):
        quiet = dict(show=lambda *a: None)
        a = PoolRun()
        res = proof.mode_suite(a.lease, self.state, **quiet, **a.kw())
        self.assertIn(NOTHING_LOADED, res[0]["jump"]["reasons"])
        res = proof.mode_suite(a.lease, self.state, **quiet, **a.kw())
        self.assertNotIn(NOTHING_LOADED, res[0]["jump"]["reasons"])   # same live games: the state is found
        a.down()
        b = PoolRun()
        res = proof.mode_suite(b.lease, self.state, **quiet, **b.kw())
        b.down()
        self.assertTrue(res[0]["jump"]["reload"])
        self.assertIn(NOTHING_LOADED, res[0]["jump"]["reasons"])

    def test_scen_cli_keys_by_game_process(self):
        def jump(run, name):
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                rc = scen.cmd_jump(argparse.Namespace(names=[name], force_reload=False, json=None),
                                   lease_fn=run.lease, state_path=self.state, **run.kw())
            self.assertEqual(rc, 0, buf.getvalue())
            return json.loads(buf.getvalue())
        a = PoolRun()
        self.assertTrue(jump(a, "quest_stage_sync")["reload"])
        self.assertFalse(jump(a, "door_terminal_container_matrix")["reload"])
        a.down()
        b = PoolRun()
        j = jump(b, "quest_stage_sync")
        b.down()
        self.assertTrue(j["reload"], j)
        self.assertIn(NOTHING_LOADED, j["reasons"])
        self.assertEqual(sorted(b.world.loads), [(0, "autosave"), (1, "autosave")])


def clean_state(scenario="quest_stage_sync", dirty=()):
    return {"loaded": True, "save": "autosave", "dirty": sorted(dirty), "failed": False, "quest_hex": [],
            "scenario": scenario}


def hammer(path, prefix, n, ready, go):
    """Worker of SharedStateFile.test_two_processes_hammering_put_keep_all_keys (run_tests.py --hammer ...): make one
    store, report ready, wait for the go file, then put n different keys one by one."""
    store = runner.StateStore(path)
    Path(ready).write_text("ready", encoding="utf-8")
    deadline = time.monotonic() + 60
    while not Path(go).exists():
        if time.monotonic() > deadline:
            return 3
        time.sleep(0.002)
    for i in range(n):
        store.put("%s%03d@hammer" % (prefix, i), {"by": prefix, "i": i})
    return 0


class SharedStateFile(unittest.TestCase):
    """Two scen.py (or two proof.py) processes can share one state file, with leases on different instances at the same
    time. A put must not lose a key that another store wrote, a plan must see what another store wrote, and a put never
    leaves a partial or invalid file."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.dir = Path(self.tmp.name)
        self.state = self.dir / "state.json"

    def tearDown(self):
        self.tmp.cleanup()

    def on_disk(self):
        return json.loads(self.state.read_text(encoding="utf-8"))

    def test_two_stores_on_one_file_keep_each_others_keys(self):
        old = clean_state()
        runner.StateStore(self.state).put("0@gameA", old)        # an earlier run left game A clean
        a = runner.StateStore(self.state)
        b = runner.StateStore(self.state)                        # both loaded before either put
        newer = clean_state("goodsprings_fight", ["00104C70"])
        a.put("0@gameA", newer)                                  # process A (lease on instance 0) dirties game A
        b.put("1@gameB", clean_state("loot_race"))               # process B (lease on instance 1), stale copy of 0@gameA
        disk = self.on_disk()
        self.assertEqual(sorted(disk), ["0@gameA", "1@gameB"])
        self.assertEqual(disk["0@gameA"], newer)                 # B did not write back its older copy of A's key
        fresh = runner.StateStore(self.state)
        self.assertEqual(fresh.get_many(["0@gameA", "1@gameB"]), [newer, clean_state("loot_race")])

    def test_plan_sees_dirty_state_written_by_another_store(self):
        keys = ["0@gameA", "1@gameB"]
        runner.StateStore(self.state).put_many({k: clean_state() for k in keys})
        w, clock, nvs = fakegame.make(2)
        prime(w)
        b = runner.StateStore(self.state)                        # this runner's store, made first
        rn = runner.Runner(nvs, b, keys, sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER)
        sc = load("goodsprings_fight")
        fp = scenlib.footprint(sc, order=ORDER)
        self.assertFalse(scenlib.plan_jump(rn._same_world(), sc, fp)["reload"])   # clean: the jump would go in place
        a = runner.StateStore(self.state)                        # another process writes after b was made
        a.put_many({k: clean_state("goodsprings_fight", fp["dirty"]) for k in keys})
        j = rn.jump(sc)
        self.assertTrue(j["reload"], j)
        self.assertTrue(any("changed refs" in x for x in j["reasons"]), j["reasons"])
        self.assertEqual(sorted(w.loads), [(0, "autosave"), (1, "autosave")])

    def test_failed_put_leaves_the_old_valid_file(self):
        store = runner.StateStore(self.state)
        store.put("0@old", clean_state())
        before = self.state.read_bytes()
        names = sorted(os.listdir(self.dir))
        seen = {}

        def crash(src, dst):
            seen["dir"] = Path(src).parent
            seen["new"] = json.loads(Path(src).read_text(encoding="utf-8"))
            raise OSError("simulated failure between the write and the replace")
        store._replace = crash
        with self.assertRaises(OSError):
            store.put("1@new", clean_state("loot_race"))
        self.assertEqual(seen["dir"], self.dir)                  # the temporary file is in the same folder
        self.assertIn("1@new", seen["new"])                      # and it was written in full before the replace
        self.assertEqual(self.state.read_bytes(), before)        # the target is still the old file
        self.assertEqual(self.on_disk(), {"0@old": clean_state()})
        self.assertEqual(sorted(os.listdir(self.dir)), names)    # no temporary file is left
        fresh = runner.StateStore(self.state)
        self.assertEqual(fresh.get_many(["0@old", "1@new"]), [clean_state(), None])

    def test_lock_timeout_gives_a_clear_error(self):
        holder = runner.StateStore(self.state)
        waiter = runner.StateStore(self.state, lock_timeout_s=0.3)
        with holder._lock():
            t0 = time.monotonic()
            with self.assertRaises(runner.StateLockTimeout) as cm:
                waiter.put("0@x", clean_state())
            self.assertLess(time.monotonic() - t0, 5)
        self.assertIn(str(self.state) + ".lock", str(cm.exception))
        self.assertIn("another process held the lock", str(cm.exception))
        self.assertFalse(self.state.exists())                    # nothing was written while the lock was held
        waiter.put("0@x", clean_state())                         # free again: the put goes through
        self.assertEqual(self.on_disk(), {"0@x": clean_state()})

    def test_two_processes_hammering_put_keep_all_keys(self):
        n = 200
        go = self.dir / "go"
        procs = []
        try:
            for name in ("a", "b"):
                procs.append(subprocess.Popen(
                    [sys.executable, "-I", str(HERE / "run_tests.py"), "--hammer", str(self.state), name, str(n),
                     str(self.dir / (name + ".ready")), str(go)],
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True))
            deadline = time.monotonic() + 60
            while not all((self.dir / (x + ".ready")).exists() for x in ("a", "b")):
                self.assertTrue(all(p.poll() is None for p in procs), "a worker stopped before it was ready")
                self.assertLess(time.monotonic(), deadline, "workers not ready after 60 s")
                time.sleep(0.01)
            go.write_text("go", encoding="utf-8")                # both stores are loaded before the first put
            for p in procs:
                out, err = p.communicate(timeout=180)
                self.assertEqual(p.returncode, 0, out + err)
        finally:
            for p in procs:
                if p.poll() is None:
                    p.kill()
                    p.communicate()
        want = {"%s%03d@hammer" % (x, i): {"by": x, "i": i} for x in ("a", "b") for i in range(n)}
        self.assertEqual(len(self.on_disk()), 2 * n)
        self.assertEqual(self.on_disk(), want)
        self.assertEqual(runner.StateStore(self.state).get_many(sorted(want)), [want[k] for k in sorted(want)])
        self.assertEqual([x for x in os.listdir(self.dir) if x.endswith(".tmp")], [])


BOX_OFFSETS = [2, 3, 4, 5, 18, 19, 32, 33, 34, 35, 48, 49, 50, 51]   # 14 message boxes in waves over ~50 s (measured 2026-10-21)


def boxed(n=2, sim=None, root=None, **flags):
    """A fake world whose plain save raises 14 message boxes after each load, and whose coop.status is the CoopSim `sim`."""
    w, clock, nvs = fakegame.make(n, **flags)
    prime(w)
    w.box_offsets = list(BOX_OFFSETS)
    w.sim = sim
    w.saves_root = root
    return w, clock, nvs


def saves_of(root):
    return prepare_save.PreparedSaves(lambda inst: Path(root) / str(inst))


def runner_on(nvs, clock, **kw):
    return runner.Runner(nvs, runner.StateStore(), sleep=clock.sleep, now=clock.now, log=lambda *a: None, order=ORDER, **kw)


class PreparedSave(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = self.tmp.name

    def tearDown(self):
        self.tmp.cleanup()

    def prepare(self, w, clock, nvs, save="autosave", **kw):
        return prepare_save.prepare(nvs, save, saves_of(self.root), log=lambda *a: None, sleep=clock.sleep, now=clock.now, **kw)

    def test_prepare_presses_every_box_before_it_saves_and_the_manifest_says_so(self):
        w, clock, nvs = boxed(root=self.root)
        r = self.prepare(w, clock, nvs)
        self.assertTrue(r["ok"], r)
        self.assertEqual(r["boxesPressed"], 28)                      # 14 boxes on each of the two copies
        self.assertEqual(sorted(w.save_log), [(0, "nvcoop_autosave_ready"), (1, "nvcoop_autosave_ready")])
        self.assertIn("nvcoop_autosave_ready", w.clean_saves)        # saved after the last box was pressed
        self.assertEqual(saves_of(self.root).pick(1, "autosave")[0], "nvcoop_autosave_ready")

    def test_direct_mode_prepares_the_given_instances_on_their_own_pipes_without_a_pool(self):
        # run_fight.py --instances 2,3 talks to the pipes itself; prepare_save --direct 2,3 must do the same and record per instance id
        w, clock, nvs = boxed(root=self.root)
        store = prepare_save.PreparedSaves(lambda inst: Path(self.root) / str(inst - 2))   # the fake writes <root>/<position>
        opened = []

        @contextlib.contextmanager
        def connect(i):
            opened.append(i)
            yield nvs[len(opened) - 1]
        with contextlib.redirect_stdout(io.StringIO()):
            rc = prepare_save.main(["--direct", "2,3", "--save", "autosave"], store=store, connect_fn=connect,
                                   sleep=clock.sleep, now=clock.now, log=lambda *a: None)
        self.assertEqual(rc, 0)
        self.assertEqual(opened, [2, 3])
        self.assertEqual(store.pick(2, "autosave")[0], "nvcoop_autosave_ready")
        self.assertEqual(store.pick(3, "autosave")[0], "nvcoop_autosave_ready")
        self.assertIsNone(store.pick(4, "autosave")[0])

    def test_jump_loads_the_prepared_save_and_skips_the_long_quiet_window(self):
        w, clock, nvs = boxed(root=self.root)
        plain = runner_on(nvs, clock, prepared=False).jump(load("quest_stage_sync"))
        self.assertGreater(plain["seconds"], 60)                      # the waves plus the 20 s quiet window
        self.assertEqual(plain["loadMessages"]["pressed"], 28)
        self.assertEqual([x["prepared"] for x in plain["loadedSaves"]], [False, False])
        self.prepare(w, clock, nvs)
        w.loads.clear()
        rn = runner_on(nvs, clock, prepared=saves_of(self.root))
        j = rn.jump(load("quest_stage_sync"), force_reload=True)
        self.assertEqual(sorted(w.loads), [(0, "nvcoop_autosave_ready"), (1, "nvcoop_autosave_ready")])
        self.assertEqual([x["prepared"] for x in j["loadedSaves"]], [True, True])
        self.assertIn("nvcoop_autosave_ready", j["loadedSaves"][0]["why"])
        self.assertEqual(j["loadMessages"]["pressed"], 0)
        self.assertLess(j["seconds"], 12)                             # 1.5 s load sleep + a 4 s quiet window, not 70 s

    def test_state_still_names_the_plain_start_save(self):
        # the jump plan compares the loaded save with start.save: loading nvcoop_autosave_ready must not look like a different save
        w, clock, nvs = boxed(root=self.root)
        self.prepare(w, clock, nvs)
        rn = runner_on(nvs, clock, prepared=saves_of(self.root))
        sc = load("quest_stage_sync")
        rn.jump(sc, force_reload=True)
        self.assertEqual(rn._state()["save"], "autosave")
        self.assertFalse(rn.jump(load("door_terminal_container_matrix"))["reload"])

    def test_without_a_prepared_save_the_plain_one_loads_with_the_full_quiet_window(self):
        w, clock, nvs = boxed(root=self.root)
        j = runner_on(nvs, clock, prepared=saves_of(self.root)).jump(load("quest_stage_sync"))
        self.assertEqual(sorted(w.loads), [(0, "autosave"), (1, "autosave")])
        self.assertIn("no prepared save", j["loadedSaves"][0]["why"])
        self.assertEqual(j["loadMessages"]["pressed"], 28)

    def test_one_copy_without_a_prepared_save_keeps_the_long_window_for_both(self):
        w, clock, nvs = boxed(root=self.root)
        self.prepare(w, clock, nvs)
        (Path(self.root) / "1" / "nvcoop_autosave_ready.fos").unlink()
        w.loads.clear()
        j = runner_on(nvs, clock, prepared=saves_of(self.root)).jump(load("quest_stage_sync"), force_reload=True)
        self.assertEqual(sorted(w.loads), [(0, "nvcoop_autosave_ready"), (1, "autosave")])
        self.assertEqual(j["loadMessages"]["pressed"], 14)
        self.assertGreater(j["seconds"], 60)

    def test_a_prepared_save_the_game_refuses_falls_back_to_the_plain_save(self):
        w, clock, nvs = boxed(root=self.root)
        self.prepare(w, clock, nvs)
        w.refuse_loads.add("nvcoop_autosave_ready")
        w.loads.clear()
        j = runner_on(nvs, clock, prepared=saves_of(self.root)).jump(load("quest_stage_sync"), force_reload=True)
        self.assertEqual(sorted(w.loads), [(0, "autosave"), (1, "autosave")])
        self.assertIn("refused", j["loadedSaves"][0]["why"])
        self.assertEqual([x["prepared"] for x in j["loadedSaves"]], [False, False])

    def test_scenario_can_opt_out(self):
        w, clock, nvs = boxed(root=self.root)
        self.prepare(w, clock, nvs)
        w.loads.clear()
        sc = copy.deepcopy(load("quest_stage_sync"))
        sc["start"]["prepared"] = False
        self.assertEqual(scenlib.validate(sc), [])
        j = runner_on(nvs, clock, prepared=saves_of(self.root)).jump(sc, force_reload=True)
        self.assertEqual(sorted(w.loads), [(0, "autosave"), (1, "autosave")])
        self.assertIn("start.prepared", j["loadedSaves"][0]["why"])
        sc["start"]["prepared"] = "no"
        self.assertTrue(any("start.prepared" in e for e in scenlib.validate(sc)))

    def test_a_source_save_that_changed_makes_the_prepared_one_stale(self):
        w, clock, nvs = boxed(root=self.root)
        src = [Path(self.root) / str(i) / "mysave.fos" for i in (0, 1)]
        for p in src:
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_bytes(b"one")
        self.prepare(w, clock, nvs, save="mysave")
        store = saves_of(self.root)
        self.assertEqual(store.pick(0, "mysave")[0], "nvcoop_mysave_ready")
        src[0].write_bytes(b"a different world")
        name, why = store.pick(0, "mysave")
        self.assertIsNone(name)
        self.assertIn("changed", why)
        self.assertEqual(store.pick(1, "mysave")[0], "nvcoop_mysave_ready")     # the other copy is untouched

    def test_autosave_source_is_exempt_from_the_stale_check(self):
        # the game rewrites "autosave" itself, so its size and time prove nothing about the prepared copy
        w, clock, nvs = boxed(root=self.root)
        src = Path(self.root) / "0" / "autosave.fos"
        src.parent.mkdir(parents=True, exist_ok=True)
        src.write_bytes(b"one")
        self.prepare(w, clock, nvs)
        src.write_bytes(b"the game autosaved again")
        self.assertEqual(saves_of(self.root).pick(0, "autosave")[0], "nvcoop_autosave_ready")

    def test_a_save_whose_reload_still_raised_boxes_is_not_used(self):
        w, clock, nvs = boxed(root=self.root, boxes_persist=True)     # the prepared save raises the boxes again
        r = self.prepare(w, clock, nvs)
        self.assertFalse(r["ok"])
        self.assertTrue(all(not x["verified"] and "still raised" in x["error"] for x in r["perInstance"]), r)
        name, why = saves_of(self.root).pick(0, "autosave")
        self.assertIsNone(name)
        self.assertIn("reload check", why)

    def test_a_missing_file_is_not_used(self):
        w, clock, nvs = boxed(root=self.root)
        self.prepare(w, clock, nvs)
        (Path(self.root) / "0" / "nvcoop_autosave_ready.fos").unlink()
        name, why = saves_of(self.root).pick(0, "autosave")
        self.assertIsNone(name)
        self.assertIn("gone", why)

    def test_without_bot_menu_nothing_is_saved(self):
        w, clock, nvs = fakegame.make(2)
        w.saves_root = self.root
        with self.assertRaises(RuntimeError):
            self.prepare(w, clock, nvs)
        self.assertEqual(w.save_log, [])

    def test_engine_that_only_takes_autosave_names_gets_the_second_name(self):
        w, clock, nvs = boxed(root=self.root)
        real = fakegame.FakeNv.save_game

        def picky(self_, name, timeout_ms=0, overwrite=False):
            if not name.startswith("autosave"):
                raise RuntimeError("engine refused " + name)
            return real(self_, name, timeout_ms, overwrite)
        with unittest.mock.patch.object(fakegame.FakeNv, "save_game", picky):
            r = self.prepare(w, clock, nvs)
        self.assertTrue(r["ok"], r)
        self.assertEqual(saves_of(self.root).pick(0, "autosave")[0], "autosave_nvcoop_autosave_ready")

    def test_runner_with_game_process_keys_looks_for_prepared_saves_and_one_without_does_not(self):
        w, clock, nvs = fakegame.make(2)
        self.assertIsNone(runner_on(nvs, clock).prepared)
        self.assertIsNotNone(runner_on(nvs, clock, keys=["3@pid-1", "4@pid-2"]).prepared)
        self.assertEqual(runner_on(nvs, clock, keys=["3@pid-1", "4@pid-2"]).instance_ids, [3, 4])
        self.assertIsNone(runner_on(nvs, clock, keys=["3@pid-1", "4@pid-2"], prepared=False).prepared)

    def test_instance_ids_come_from_the_pipe_name_when_the_key_only_has_a_position(self):
        # run_parallel pair 1 drives instances 2 and 3; the playbook adapter keys its Runner "0@pid-x", "1@pid-y" by position
        w, clock, nvs = fakegame.make(2)
        for nv, n in zip(nvs, (2, 3)):
            nv.name = "pipe-nvcoop-%d" % n
        self.assertEqual(runner_on(nvs, clock, keys=["0@pid-1", "1@pid-2"]).instance_ids, [2, 3])


class EventDrivenWaits(unittest.TestCase):
    def jump(self, sim, **flags):
        w, clock, nvs = boxed(sim=sim, **flags)
        j = runner_on(nvs, clock, prepared=False).jump(load("quest_stage_sync"))
        return j, j["coopSync"]

    def test_synced_world_returns_the_moment_the_digest_is_ok(self):
        j, cs = self.jump(fakegame.CoopSim())
        self.assertTrue(cs["synced"] and cs["digestOk"], cs)
        self.assertEqual(cs["settleReason"], "ok")
        self.assertLess(cs["settleSeconds"], 10)               # the first digest comes 6 s after the load; the old wait took 60 s here
        self.assertLess(cs["seconds"], 25)

    def test_slow_but_moving_client_is_not_given_up_on(self):
        _, cs = self.jump(fakegame.CoopSim(catch_up_s=100.0, head=400))   # 100 s to catch up, applied changes at every poll
        self.assertTrue(cs["synced"], cs)

    def test_client_stuck_half_way_fails_fast_and_names_the_numbers(self):
        _, cs = self.jump(fakegame.CoopSim(stuck_at=50))
        self.assertFalse(cs["synced"])
        self.assertIn("no client progress", cs["reason"])
        self.assertIn("applied 50 of 100", cs["reason"])
        self.assertLess(cs["seconds"], 45)                      # not the 180 s limit

    def test_client_that_never_joins_fails_fast(self):
        _, cs = self.jump(fakegame.CoopSim(joined=False))
        self.assertFalse(cs["synced"])
        self.assertIn("not joined", cs["reason"])
        self.assertLess(cs["seconds"], 45)

    def test_client_with_a_dead_link_fails_fast(self):
        _, cs = self.jump(fakegame.CoopSim(disconnected=True))
        self.assertFalse(cs["synced"])
        self.assertIn("transport not connected", cs["reason"])
        self.assertLess(cs["seconds"], 45)

    def test_rejected_join_fails_at_once_with_the_text(self):
        _, cs = self.jump(fakegame.CoopSim(rejected=True, joined=False))
        self.assertFalse(cs["synced"])
        self.assertIn("version mismatch", cs["reason"])
        self.assertLess(cs["seconds"], 3)

    def test_same_mismatch_again_fails_fast_and_lists_the_desync_entries(self):
        j, cs = self.jump(fakegame.CoopSim(digests="mismatch_same"))
        self.assertTrue(cs["synced"])
        self.assertFalse(cs["digestOk"])
        self.assertIn("same mismatch", cs["settleReason"])
        self.assertLess(cs["settleSeconds"], 15)                # 3 digests, not the 60 s limit
        e = cs["desyncEntries"][0]
        self.assertEqual((e["key"], e["host"], e["client"]), ("actor:00104C70.dead", "1", "0"))
        self.assertEqual(cs["digestDelta"]["mismatch"], 3)

    def test_one_mismatch_that_the_repair_fixes_is_a_success(self):
        _, cs = self.jump(fakegame.CoopSim(digests="mismatch_once", digest_from=10.0))
        self.assertTrue(cs["digestOk"], cs)
        self.assertEqual(cs["settleRepairs"], 1)

    def test_a_heal_after_the_sync_wait_is_settled_inside_the_jump(self):
        # A1c: the guest's late load-time scripts differ from the host only after the first sync wait (the 14 message boxes take ~50 s);
        # the digest that finds it repaired 256 effects in the playbook's first step and its strict counters failed.
        sim = fakegame.CoopSim(digests="mismatch_late", late_at=75.0)
        w, clock, nvs = boxed(sim=sim)
        j = runner_on(nvs, clock, prepared=False).jump(load("quest_stage_sync"))
        d = nvs[0].call("coop.status")["desyncs"]["digests"]
        self.assertEqual((d["repairs"], d["effectsRepaired"]), (1, 256), d)          # the repair happened inside the jump ...
        self.assertTrue(j["coopSync"]["settledAfterSetup"]["digestOk"], j["coopSync"])
        self.assertEqual(j["coopSync"]["settledAfterSetup"]["settleRepairs"], 1)
        clock.sleep(120.0)                                                              # ... and nothing is left for the test that runs next
        self.assertEqual(nvs[0].call("coop.status")["desyncs"]["digests"]["repairs"], 1)

    def test_the_wait_ends_at_an_ok_after_the_mismatch_not_at_one_from_before_it(self):
        # digests arrive faster than the 0.5 s poll: one poll sees ok rounds AND the mismatch. Ending there leaves the repair (256 effects,
        # possibly deferred by the budget) unfinished for the next test; the wait must see an ok digest from after the mismatch.
        sim = fakegame.CoopSim(digests="mismatch_late", late_at=75.0, every=0.2)
        w, clock, nvs = boxed(sim=sim)
        rn = runner_on(nvs, clock, prepared=False)
        sim.on_load(clock.now() - 74.6)                    # 74.6 s since the load: ok rounds so far, the mismatch round is at 75.0
        res = rn._wait_digest_ok(60.0, 0.5, 20.0)
        d = nvs[0].call("coop.status")["desyncs"]["digests"]
        ok_before = int(round((75.0 - 6.0) / 0.2))          # the rounds before the mismatch round
        self.assertTrue(res["digestOk"], res)
        self.assertGreater(d["ok"], ok_before, d)           # at least one ok round after the mismatch
        self.assertEqual(res["settleRepairs"], 1)

    def test_an_in_place_jump_with_nothing_to_reset_does_not_wait_again(self):
        w, clock, nvs = boxed(sim=fakegame.CoopSim())
        rn = runner_on(nvs, clock, prepared=False)
        rn.jump(load("quest_stage_sync"))
        j = rn.jump(load("quest_stage_sync"))
        if not j["reload"] and not j["reset"] and not j["setup"]:
            self.assertNotIn("settledAfterSetup", j["coopSync"] or {})

    def test_mismatches_on_changing_keys_are_not_called_the_same(self):
        _, cs = self.jump(fakegame.CoopSim(digests="mismatch_moving"))
        self.assertFalse(cs["digestOk"])
        self.assertIn("no ok digest within", cs["settleReason"])      # healing still moves: only the outer limit ends it
        self.assertGreater(len(cs["desyncEntries"]), 1)

    def test_stale_digests_fail_fast(self):
        _, cs = self.jump(fakegame.CoopSim(digests="stale"))
        self.assertFalse(cs["digestOk"])
        self.assertIn("stale", cs["settleReason"])
        self.assertLess(cs["settleSeconds"], 15)

    def test_silent_client_fails_after_the_digest_stall_window(self):
        _, cs = self.jump(fakegame.CoopSim(digests="silent"))
        self.assertFalse(cs["digestOk"])
        self.assertIn("no digest arrived", cs["settleReason"])
        self.assertTrue(20 <= cs["settleSeconds"] < 30, cs)

    def test_run_result_carries_the_reasons(self):
        w, clock, nvs = boxed(sim=fakegame.CoopSim(digests="mismatch_same"))
        r = runner_on(nvs, clock, prepared=False).run(load("quest_stage_sync"))
        cs = r["jump"]["coopSync"]
        self.assertIn("same mismatch", cs["settleReason"])
        self.assertIn("loadedSaves", r["jump"])
        self.assertEqual(r["status"], "pass")                    # a digest that never matches does not block the steps, as before


class Cli(unittest.TestCase):
    def test_validate_and_list_exit_codes(self):
        py = [sys.executable, "-I", "-X", "utf8", str(HERE / "scen.py")]
        p = subprocess.run(py + ["validate"], capture_output=True, text=True)
        self.assertEqual(p.returncode, 0, p.stdout + p.stderr)
        self.assertIn("0 invalid", p.stdout)
        p = subprocess.run(py + ["list"], capture_output=True, text=True)
        self.assertEqual(p.returncode, 0)
        self.assertEqual(len([l for l in p.stdout.splitlines() if l.strip()]), 16)    # 14 + goodsprings_enemies (2026-10-21) + story_goodsprings (story runs, 2026-10-28)
        p = subprocess.run(py + ["plan", "vats_hit", "--prev", "goodsprings_fight"], capture_output=True, text=True)
        self.assertEqual(p.returncode, 0)
        self.assertIn('"reload": true', p.stdout)


MUTANTS = {
    "noresolve": "validator stops resolving FormIDs, EditorIDs and record types",
    "noreload": "jump never reloads the save",
    "nodirty": "scenarios never record what they changed",
    "passall": "every check passes",
    "firsthost": "every step runs only on the host",
    "allplayers": "a step asks for every player the scenario lists, also those with no copy (IndexError on one copy)",
    "oldkey": "state key is instance@generation again (repeats after a pool restart)",
    "wholedict": "put writes the whole in-memory dict (loses keys of other processes)",
    "noreread": "the plan uses the state read at start, not the file now",
    "inplace": "put writes into the state file itself, no temp file and replace",
    "hexprefix": "console lines name a reference as '<hex>.Func' again",
    "refline": "the validator accepts a {alias}.Func call that is not first",
    "noprepared": "jump never loads the prepared save",
    "longquiet": "a prepared save still waits the 20 s quiet window of the plain save",
    "nostale": "a prepared save is used although its source save changed",
    "unverified": "a prepared save that failed its reload check is used",
    "noverify": "prepare_save skips the reload check",
    "earlysave": "prepare_save saves before the boxes are pressed away",
    "fullwait": "the digest wait never gives up early (60 s every time)",
    "latesuccess": "the digest wait notices an ok digest only at its time limit",
    "hairtrigger": "the first mismatch already ends the digest wait",
    "nodesyncs": "a failed digest wait leaves out the desync entries",
    "nobridgeid": "instance ids come from the key only (pair 1 would use the prepared saves of instances 0 and 1)",
    "directpos": "prepare_save --direct records instances by position, not by the ids given (pair 1 would write into instances 0 and 1)",
    "nostall": "the sync wait never gives up on a client that stopped (180 s)",
    "nofinalsettle": "the jump does not wait for the first digest round after its message boxes, reset and setup steps",
    "anyok": "the digest wait ends at any ok digest, also one from before a mismatch it saw",
}


def main():
    if os.environ.get("SCEN_MUTANT"):
        runner_ = unittest.TextTestRunner(stream=open(os.devnull, "w"), verbosity=0)
        res = runner_.run(unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__]))
        return 0 if res.wasSuccessful() else 1
    res = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromModule(sys.modules[__name__]))
    if not res.wasSuccessful():
        return 1
    bad = []
    for m, what in MUTANTS.items():
        env = dict(os.environ, SCEN_MUTANT=m)
        p = subprocess.run([sys.executable, "-I", str(Path(__file__).resolve())], env=env, capture_output=True, text=True)
        caught = p.returncode != 0
        print("twin %-10s %-52s %s" % (m, what, "caught" if caught else "NOT CAUGHT"))
        if not caught:
            bad.append(m)
    if bad:
        print("RESULT: twin mutants not caught: %s" % bad)
        return 1
    print("RESULT: ALL OK")
    return 0


if __name__ == "__main__":
    if sys.argv[1:2] == ["--hammer"]:   # worker of the two-process state file test
        _, _, path, prefix, n, ready, go = sys.argv
        sys.exit(hammer(path, prefix, int(n), ready, go))
    sys.exit(main())
