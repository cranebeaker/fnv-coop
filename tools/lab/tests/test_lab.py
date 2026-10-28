"""Offline tests of the lab runner (no game): the rooms on the fake world, one test per fake bug, the time boxes, fail fast, the reports.

    python -I tools/lab/tests/test_lab.py            # all of it
    python -I tools/lab/run_tests.py                 # these tests, then the mutation twins (LAB_MUTANT=<name>: each must make a test fail)
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
LAB = HERE.parent
ROOT = LAB.parent.parent
sys.path.insert(0, str(LAB))

import lab                       # noqa: E402
import lab_core as K             # noqa: E402
import lab_layout as LY          # noqa: E402
import lab_report as R           # noqa: E402
import labfake                   # noqa: E402
from lab_rooms import PARAMS, ROOMS, split_rooms, walk_stats   # noqa: E402

EM_DASH = "\u2014"
ALL = list(LY.ROOM_ORDER)

# bug -> (room that must fail, text of the check it fails at, class)
EXPECT = {
    "client_stays_out": ("spawn", "both copies are in the lab cell", "ENV"),
    "not_joined": ("spawn", "connected", "SYNC"),
    "no_puppet": ("spawn", "puppet", "SYNC"),
    "puppet_offset": ("spawn", "puppet", "SYNC"),
    "digest_noise": ("fight", "no digest repair", "SYNC"),
    "ref_id_differs": ("fight", "same ids", "SYNC"),
    "global_missing": ("spawn", "idle digest stayed clean", "SYNC"),
    "puppet_lag": ("walk", "error p95", "SYNC"),
    "puppet_teleports": ("walk", "error p95", "SYNC"),
    "enable_unsynced": ("fight", "enabled hostiles", "SYNC"),
    "weak_unsynced": ("fight", "weakened health on both copies", "SYNC"),
    "host_kill_unseen": ("fight", "killed by the host", "SYNC"),
    "client_kill_unseen": ("fight", "killed by the client", "SYNC"),
    "slow_death": ("fight", "killed by the host", "SYNC"),
    "corpse_revives": ("fight", "no corpse gets up", "SYNC"),
    "survivor_hurt": ("fight", "survivor", "SYNC"),
    "container_unsynced": ("loot", "container count", "SYNC"),
    "take_dupe": ("loot", "container count", "SYNC"),
    "dup_pickup": ("loot", "exactly one player", "SYNC"),
    "pickup_unsynced": ("loot", "is gone on both", "SYNC"),
    "door_unsynced": ("door", "the host opened", "SYNC"),
    "lock_unsynced": ("door", "unlocked on both", "SYNC"),
    "terminal_unsynced": ("door", "unlocked on both", "SYNC"),
    "terminal_noop": ("door", "the terminal click unlocked the door", "TEST"),
    "locked_door_opens": ("door", "cannot open the locked", "SYNC"),
    "no_handoff": ("npc", "ownership of the NPC moves", "SYNC"),
    "nearest_owner": ("npc", "the host keeps the NPC", "SYNC"),
    "owner_split": ("npc", "ownership of the NPC moves", "SYNC"),
    "npc_pos_stale": ("npc", "same place", "SYNC"),
    "handoff_snap": ("npc", "never snaps", "SYNC"),
    "npc_static": ("npc", "patrols", "TEST"),
    "vats_unsynced": ("vats", "health agrees", "SYNC"),
    "vats_double": ("vats", "health agrees", "SYNC"),
    "stage_unsynced": ("quest", "client sees stage 10", "SYNC"),
    "stage_client_lost": ("quest", "host agrees on stage 20", "SYNC"),
    "var_unsynced": ("quest", "quest variable", "SYNC"),
    "stage_rollback": ("quest", "does not roll back", "SYNC"),
}
NOT_A_FAILURE = set(labfake.NET_ONLY) | {"var_unreadable",         # a build that cannot read a quest variable skips that check, it does not fail the lab
                 "kill_health_split",      # how the real game reports a kill (own test below)
                 "target_drifts",          # the fight room holds each target in place, so none drifts out of range (own test below)
                 "hostiles_charge",        # the fight room enables one hostile at a time, so none has walked up to its shooter (own test below); the VATS room holds its target
                 "locked_try_opens_lockpick",
                 "slow_npc_samples"}     # a slow sample is no snap (own test below)   # the rooms close a menu an earlier step left open (own test below)


def layout() -> LY.Layout:
    return LY.Layout(LY.sample_layout())


def run_lab(bugs=(), rooms=None, pairs=1, budget_s=1200.0, room_budget_s=120.0, params=None, confirm=0, env=None, lay=None):
    lay = lay or layout()
    env = env or labfake.FakeEnv(lay, bugs=bugs)
    return lab.execute(lay, env, rooms or ALL, pairs, budget_s, room_budget_s, params or dict(PARAMS), confirm, "20990101-000000", "fake", lambda s: None), env


def row(res, room, pair=None):
    return next(r for r in res["rooms"] if r["room"] == room and (pair is None or r["pair"] == pair))


def failed(r):
    return [c for c in r["checks"] if not c["ok"]]


class Layout(unittest.TestCase):
    def test_sample_layout_is_valid_and_complete(self):
        lay = layout()
        self.assertEqual(lay.problems(), [])
        for room in ALL:
            self.assertEqual(lay.missing(room), [], room)

    def test_forms_are_normalised_and_refs_come_in_key_order(self):
        self.assertEqual(LY.norm_form("nvcooplab.esp:8a0"), "nvcooplab.esp:0008A0")
        d = LY.sample_layout()
        d["rooms"]["fight"]["refs"] = {"enemy10": "NVCoopLab.esp:000830", "enemy2": "NVCoopLab.esp:000821", "enemy1": "NVCoopLab.esp:000831"}
        self.assertEqual(LY.Layout(d).refs("fight", "enemy"), ["NVCoopLab.esp:000831", "NVCoopLab.esp:000821", "NVCoopLab.esp:000830"])

    def test_bad_layouts_say_what_is_wrong(self):
        d = LY.sample_layout()
        d["rooms"]["fight"]["refs"]["enemy1"] = "not a form"
        del d["rooms"]["loot"]["stand"]
        d["rooms"]["walk"]["origin"] = [1, 2]
        bad = LY.Layout(d).problems()
        self.assertTrue(any("not a form" in b for b in bad) and any("no stand marker" in b for b in bad) and any("origin" in b for b in bad), bad)
        with tempfile.TemporaryDirectory() as t:
            p = Path(t) / "layout.json"
            self.assertRaises(LY.LayoutError, LY.Layout.load, p)
            p.write_text("{nope", encoding="utf-8")
            self.assertRaises(LY.LayoutError, LY.Layout.load, p)
            p.write_text(json.dumps(LY.sample_layout()), encoding="utf-8")
            self.assertEqual(LY.Layout.load(p).plugin, "NVCoopLab.esp")

    def test_a_room_with_too_few_refs_is_reported(self):
        d = LY.sample_layout()
        del d["rooms"]["fight"]["refs"]["enemy3"]
        del d["rooms"]["quest"]
        lay = LY.Layout(d)
        self.assertIn("3 ref(s)", lay.missing("fight")[0])
        self.assertIn("not in the layout", lay.missing("quest")[0])


class RealLayout(unittest.TestCase):
    """tests/layout_generated.json is a copy of what tools/lab/esp wrote (out/lab/layout.json): the runner must read the real key names."""

    def setUp(self):
        self.lay = LY.Layout.load(HERE / "layout_generated.json")

    def test_every_room_has_what_it_needs(self):
        for room in ALL:
            self.assertEqual(self.lay.missing(room), [], room)
        self.assertEqual(self.lay.cell_edid, "NVCoopLab")
        self.assertEqual(self.lay.cell_alt, "GreenRoom")

    def test_disabled_hostiles_chest_items_and_quest_info_are_found(self):
        self.assertEqual(self.lay.initially_disabled("fight"), self.lay.refs("fight", "enemy"))
        self.assertEqual(self.lay.initially_disabled("vats"), self.lay.refs("vats", "target"))
        ci = self.lay.container_items()
        self.assertEqual(ci["FalloutNV.esm:015169"], 5)
        self.assertEqual(len(self.lay.loose_bases()), 4)
        self.assertEqual(self.lay.ref("loot", "container", "chest"), "NVCoopLab.esp:000841")
        self.assertEqual(self.lay.extra("quest", "var"), "labvar")

    def test_the_fake_lab_passes_on_the_generated_layout(self):
        res, _ = run_lab(lay=self.lay, pairs=2)
        self.assertEqual(res["verdict"], "PASS", [(r["room"], r["reason"]) for r in res["rooms"] if r["verdict"] != "PASS"])

    def test_the_hostiles_are_enabled_for_the_fight_and_put_away_after_it(self):
        env = labfake.FakeEnv(self.lay)
        res, _ = run_lab(env=env, lay=self.lay, rooms=["fight"])
        w = env.worlds[0]
        for r in self.lay.refs("fight", "enemy"):
            self.assertFalse(w.sides["host"].objs[r]["enabled"])
        # and a fight room that never enables them fails at the first look
        res, _ = run_lab(env=labfake.FakeEnv(self.lay, bugs=["enable_unsynced"]), lay=self.lay, rooms=["fight"])
        self.assertIn("enabled hostiles", failed(row(res, "fight"))[0]["name"])


class Split(unittest.TestCase):
    def test_spawn_is_first_on_every_pair_and_every_room_runs_once(self):
        for n in (1, 2, 3, 5):
            plan = split_rooms(ALL, n)
            self.assertEqual(len(plan), n)
            self.assertTrue(all(p[0] == "spawn" for p in plan))
            rest = [r for p in plan for r in p[1:]]
            self.assertEqual(sorted(rest), sorted(r for r in ALL if r != "spawn"))

    def test_the_load_is_balanced(self):
        plan = split_rooms(ALL, 3)
        loads = [sum(ROOMS[r].cost_s for r in p) for p in plan]
        self.assertLess(max(loads) - min(loads), 60, (plan, loads))
        self.assertEqual(split_rooms(["fight"], 1), [["spawn", "fight"]])


class Pure(unittest.TestCase):
    def test_walk_stats(self):
        ok = [(i * 0.25, (i * 50.0, 0, 0), (i * 50.0 - 40, 0, 0)) for i in range(20)]
        st = walk_stats(ok, 250)
        self.assertEqual(st["teleports"], 0)
        self.assertAlmostEqual(st["err_max"], 40.0, 1)
        jumpy = list(ok)
        jumpy[10] = (2.5, (500, 0, 0), (1500, 0, 0))
        self.assertGreaterEqual(walk_stats(jumpy, 250)["teleports"], 1)
        self.assertEqual(walk_stats([(0, (0, 0, 0), None)], 250)["samples"], 0)

    def test_lag_estimate_finds_the_playback_delay_whatever_the_two_bridge_reads_took(self):
        # the puppet shows the walker as he was 150 ms ago; the puppet is read 80 ms after the walker (4 bridge calls at 60 fps). The raw error is v x (lag - skew), not v x lag.
        from lab_rooms import lag_estimate
        v, lag, skew = 200.0, 0.150, 0.080
        rows = []
        for i in range(24):
            tm = 10.0 + i * 0.25
            tp = tm + skew
            mp = (v * (tm - 10.0), 0.0, 0.0)
            pp = (v * max(0.0, tp - lag - 10.0), 0.0, 0.0)
            rows.append((tm, mp, tp, pp))
        r = lag_estimate(rows)
        self.assertLessEqual(abs(r["lag_ms"] - 150), 20, r)
        self.assertLessEqual(r["err_p95"], 8.0, r)
        self.assertEqual(r["skew_ms"], 80)

    def test_trace_lag_finds_the_delay_on_one_clock_and_the_engine_lag_behind_it(self):
        # the game's own trace: the walker publishes at 30 Hz, the puppet is fed every 16 ms and shows him as he was 140 ms ago; the engine reports the puppet one frame behind the fed pose
        from lab_rooms import trace_lag
        v, lag = 300.0, 140
        local = [[1000 + i * 33, v * (i * 33) / 1000.0, 0.0, 0.0, v, 0.0, 0.0] for i in range(60)]
        def path(t): return v * max(0.0, (t - 1000)) / 1000.0
        remote = []
        for t in range(1400, 2900, 16):
            remote.append([t, path(t - lag), 0.0, 0.0, path(t - lag - 16), 0.0, 0.0, 16 | 1])
        r = trace_lag(local, remote)
        self.assertLessEqual(abs(r["lag_ms"] - 140), 6, r)
        self.assertLessEqual(abs(r["engine_lag_ms"] - 156), 6, r)
        self.assertLessEqual(r["err_p95_aligned"], 3.0, r)
        self.assertLessEqual(abs(r["err_p95_now"] - v * 0.14), 4.0, r)       # the real error at the same instant: speed x delay
        self.assertIsNone(trace_lag([], remote)["lag_ms"])

    def test_stage_metrics_are_leg_means_and_the_wire_time_is_the_difference(self):
        from lab_rooms import stage_metrics, gauge_leg_mean
        g = lambda n, s: {"n": n, "sum": s, "last": 0, "mean": 0, "max": 0}
        self.assertEqual(gauge_leg_mean(g(10, 100.0), g(30, 500.0)), 20.0)      # 400 over 20 samples
        self.assertIsNone(gauge_leg_mean(g(10, 100.0), g(10, 100.0)))
        w0, w1 = {"send": g(0, 0.0)}, {"send": g(100, 300.0)}                   # the walker: 3 ms from sample to the transport
        v0 = {"rx": g(0, 0.0), "applyUs": g(0, 0.0), "remotes": [{"pushes": 0, "delay": g(0, 0.0), "hold": g(0, 0.0)}]}
        v1 = {"rx": g(100, 1300.0), "applyUs": g(100, 20000.0),                 # the viewer: 13 ms from sample to its network layer
              "remotes": [{"pushes": 100, "delay": g(100, 3000.0), "hold": g(100, 800.0), "lowLatency": True, "jitterMs": 12.0, "offsetMs": 4}]}
        m = stage_metrics(w0, w1, v0, v1)
        self.assertEqual((m["send_ms"], m["rx_ms"], m["transport_ms"]), (3.0, 13.0, 10.0))
        self.assertEqual((m["delay_in_use_ms"], m["hold_ms"], m["apply_ms"], m["low_latency"]), (30.0, 8.0, 0.2, 1))

    def test_lag_estimate_without_enough_samples_says_none(self):
        from lab_rooms import lag_estimate
        self.assertIsNone(lag_estimate([(1.0, (0, 0, 0), 1.1, (0, 0, 0))])["lag_ms"])

    def test_new_desync_entries_oldest_first_and_only_newer(self):
        ds = {"desyncs": [{"id": 3, "entries": [{"key": "c", "host": 1, "client": 2}]}, {"id": 1, "entries": [{"key": "a", "host": 1, "client": 2}]},
                          {"id": 2, "entries": [{"key": "b", "host": 5, "client": 6}]}]}
        self.assertEqual([e["key"] for e in K.new_desync_entries(ds, 0)], ["a", "b", "c"])
        self.assertEqual([e["key"] for e in K.new_desync_entries(ds, 1)], ["b", "c"])

    def test_connected_reads_both_sides(self):
        h = {"role": "host", "host": {"allSynced": True, "peers": [{"joined": True}]}}
        c = {"role": "client", "client": {"joined": True}, "transport": {"connected": True}}
        self.assertTrue(K.connected(h, c)[0])
        self.assertIn("not joined", K.connected(h, dict(c, client={"joined": False}))[1])
        self.assertIn("transport down", K.connected(h, dict(c, transport={"connected": False}))[1])

    def test_exit_codes_and_overall(self):
        self.assertEqual(R.exit_code("PASS", None), 0)
        self.assertEqual(R.exit_code("FAIL", "SYNC"), 1)
        self.assertEqual(R.exit_code("TIMEOUT", "ENV"), 1)
        self.assertEqual(R.exit_code("BLOCKED", "TEST"), 3)
        self.assertEqual(R.exit_code("BLOCKED", "ENV"), 4)
        rows = [{"verdict": "PASS"}, {"verdict": "BLOCKED", "class": "ENV"}, {"verdict": "FAIL", "class": "SYNC"}]
        self.assertEqual(R.overall(rows), ("FAIL", "SYNC"))
        self.assertEqual(R.overall(rows + [{"verdict": "FAIL", "class": "CRASH"}]), ("FAIL", "CRASH"))


class FakeLab(unittest.TestCase):
    def test_a_clean_world_passes_every_room_on_three_pairs(self):
        res, _ = run_lab(pairs=3)
        self.assertEqual(res["verdict"], "PASS", [(r["room"], r["reason"]) for r in res["rooms"] if r["verdict"] != "PASS"])
        self.assertEqual([r["room"] for r in res["rooms"]].count("spawn"), 3)
        self.assertEqual({r["room"] for r in res["rooms"]}, set(ALL))
        for r in res["rooms"]:
            self.assertTrue(r["checks"] and all(c["ok"] for c in r["checks"]))
            self.assertLess(r["seconds"], 120)

    def test_every_fake_bug_has_a_test_and_every_test_a_bug(self):
        self.assertEqual(set(EXPECT) | NOT_A_FAILURE, set(labfake.BUGS))

    def test_each_bug_fails_its_room_at_its_check(self):
        for bug, (room, what, cls) in EXPECT.items():
            res, _ = run_lab(bugs=[bug])
            r = row(res, room)
            self.assertEqual(r["verdict"], "FAIL", f"{bug}: {room} {r['verdict']} {r['reason']}")
            self.assertEqual(r["class"], cls, bug)
            f = failed(r)
            self.assertEqual(len(f), 1, f"{bug}: fail fast, one failed check, got {[c['name'] for c in f]}")
            self.assertIn(what, f[0]["name"], bug)
            self.assertFalse(r["checks"][-1]["ok"], f"{bug}: nothing runs after the failed check")
            self.assertEqual(res["verdict"], "FAIL", bug)

    def test_a_digest_that_never_agrees_fails_spawn_fast_with_the_divergence(self):
        # the first in-game run: SetGlobal 37 missing on the client, no digest ever ok, and the spawn room sat in its wait until the 120 s box ended (TIMEOUT)
        res, _ = run_lab(bugs=["global_missing"], rooms=["spawn"])
        r = row(res, "spawn")
        self.assertEqual((r["verdict"], r["class"]), ("FAIL", "SYNC"), r["reason"])
        self.assertLess(r["seconds"], PARAMS["spawn.idle_s"] + PARAMS["spawn.digest_wait_s"] + 10, "it must not wait for an ok that cannot come")
        self.assertEqual(r["first_divergence"]["key"], "global/37")
        self.assertEqual(r["first_divergence"]["client"], None)
        self.assertTrue(r["digests_moved"]["mismatch"] >= 1)

    def test_a_pair_with_no_digest_verdict_at_all_fails_the_ok_check_within_the_bounded_wait(self):
        lay = layout()

        class Mute(labfake.FakeEnv):
            def prepare(self, pair):
                super().prepare(pair)
                self.worlds[pair].last_digest = 1e9          # the fake digest never runs: no verdict, ok or bad
        res, _ = run_lab(rooms=["spawn"], env=Mute(lay), lay=lay)
        r = row(res, "spawn")
        self.assertEqual((r["verdict"], r["class"]), ("FAIL", "SYNC"), r["reason"])
        self.assertIn("at least one digest ok", failed(r)[0]["name"])
        self.assertLess(r["seconds"], PARAMS["spawn.idle_s"] + PARAMS["spawn.digest_wait_s"] + 10)

    def test_spawn_has_its_own_larger_time_box_than_the_other_rooms(self):
        self.assertGreater(lab.box_of("spawn", 120.0), 120.0)
        self.assertEqual(lab.box_of("walk", 120.0), 120.0)
        self.assertEqual(lab.box_of("spawn", 500.0), 500.0)
        res, _ = run_lab(rooms=["door"], room_budget_s=20.0, params=dict(PARAMS, **{"spawn.idle_s": 40.0}))
        self.assertEqual(row(res, "spawn")["verdict"], "PASS", "a 40 s idle window inside a 20 s --room-budget: spawn has its own box")
        self.assertEqual(row(res, "spawn")["budget_s"], lab.box_of("spawn", 20.0))

    def test_an_eligible_host_keeps_the_npc_however_far_away_it_stands(self):
        # docs/FULL-SYNC-PLAN.md 1.2 and 3.2 (pickOwner): the lab is one interior cell, so a host far from the NPC and a client right next to it
        # still leave the NPC with the host; only a host in another cell hands it over (and takes it back on its return)
        def world(bugs):
            lay = layout()
            env = labfake.FakeEnv(lay, bugs=bugs)
            env.prepare(0)
            w = env.world(0)
            env.enter(0)()
            ox, oy, oz = w.sides[K.HOST].objs[lay.ref("npc", "npc")]["pos"]
            w.sides[K.CLIENT].pos = [ox + 100, oy, oz]
            w.sides[K.HOST].pos = [ox + 5000, oy, oz]
            w.clock.sleep(12.0)
            return lay, env, w
        lay, env, w = world(())
        self.assertEqual((w.truth["owner"], w.truth["epoch"]), (0, 1), "far host, near client, same cell: the host keeps it")
        h, c = env.machines(0)
        h.coc(["GSDocMitchellHouse"])
        w.clock.sleep(4.0)
        self.assertEqual((w.truth["owner"], w.truth["epoch"]), (1, 2), "a host in another cell hands the NPC to the client")
        h.coc([lay.cell_edid])
        w.clock.sleep(4.0)
        self.assertEqual((w.truth["owner"], w.truth["epoch"]), (0, 3), "and takes it back when it returns")
        self.assertEqual(world(("nearest_owner",))[2].truth["owner"], 1, "the old nearest-player rule would have given it to the client")

    def test_the_npc_room_sends_the_host_to_another_interior_and_back(self):
        res, _ = run_lab(rooms=["npc"])
        r = row(res, "npc")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        for want in ("the host goes to GSDocMitchellHouse", "the host returns to the lab cell"):
            self.assertIn(want, names)
        self.assertLess(names.index("the host goes to GSDocMitchellHouse"), names.index("the host returns to the lab cell"))

    def test_the_lab_ini_leaves_pickup_and_container_to_the_loot_service(self):
        # intents pickup / container on top of the loot service handled a guest's pickup twice; the keys are [intents] pickup / container (intents_module.cpp)
        import lab_live
        self.assertIn("intents.enable=1", lab_live.LAB_INI)
        self.assertIn("intents.pickup=0", lab_live.LAB_INI)
        self.assertIn("intents.container=0", lab_live.LAB_INI)
        self.assertNotIn("intents.terminal=0", lab_live.LAB_INI, "the door room needs the terminal intent")
        self.assertIn("worldtime.enabled=1", lab_live.LAB_INI, "the door poll and the client heal are off without it")

    def test_the_vats_room_enables_one_target_at_a_time_and_weakens_it_before_the_shot(self):
        # lab run 2026-10-27: both geckos enabled at once were on top of the host when its VATS shot started (vats_not_entered)
        res, _ = run_lab(rooms=["vats"])
        r = row(res, "vats")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        enable = [i for i, n in enumerate(names) if n.startswith("the host enables")]
        self.assertEqual(len(enable), 2)
        self.assertLess(enable[0], names.index("the host weakens the host's target"))
        self.assertLess(names.index("the host's target shows the weakened health on both copies"), names.index("host VATS shot 1"))
        self.assertGreater(enable[1], names.index("host VATS shot 1"), "the client's target is enabled after the host's shots")
        self.assertLess(enable[1], names.index("client VATS shot 1"))

    def test_the_fight_room_weakens_each_target_through_the_host_before_it_is_shot(self):
        res, _ = run_lab(rooms=["fight"])
        r = row(res, "fight")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        for idx, shooter in ((1, "host"), (2, "client")):
            weak, shown, kill = (names.index(n) for n in (f"the host weakens enemy {idx}", f"enemy {idx} shows the weakened health on both copies", f"{shooter} kills enemy {idx}"))
            self.assertLess(weak, shown)
            self.assertLess(shown, kill, "the shot comes after both copies show the weak health")
        self.assertNotIn("the host weakens enemy 3", names, "the survivor keeps its health")

    def test_the_fight_room_enables_one_hostile_at_a_time_so_none_walks_up_to_the_shooter(self):
        # the first in-game run: all three charged, by the client's turn one stood 60 units away and aim_at refused the shot (too_close)
        res, _ = run_lab(bugs=["hostiles_charge"], rooms=["fight"])
        r = row(res, "fight")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        enable = [i for i, n in enumerate(names) if n.startswith("the host enables")]
        self.assertEqual(len(enable), 3)
        self.assertLess(enable[0], names.index("host kills enemy 1"))
        self.assertGreater(enable[1], names.index("host kills enemy 1"), "enemy 2 is enabled after the host's kill")
        self.assertLess(enable[1], names.index("client kills enemy 2"))
        self.assertGreater(enable[2], names.index("client kills enemy 2"), "the survivor is enabled last")

    def test_the_vats_room_holds_each_target_in_place_so_none_walks_up_to_the_shooter(self):
        # lab run 2026-10-27: an enabled gecko ran 368 units in about 4 s and stood on the shooter: vats_not_entered / hit chance 0. The host restrains it.
        res, _ = run_lab(bugs=["hostiles_charge"], rooms=["vats"])
        r = row(res, "vats")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        held = [i for i, n in enumerate(names) if "holds the" in n]
        self.assertEqual(len(held), 2)
        self.assertLess(held[0], names.index("host VATS shot 1"))
        self.assertLess(held[1], names.index("client VATS shot 1"))

    def test_the_fight_room_holds_each_target_in_place_so_none_drifts_out_of_the_pistols_range(self):
        # lab run 2026-10-28: behind bad_wifi and transatlantic the second target drifted 659 and 875 units away: "no clear shot even from the standoff"
        res, _ = run_lab(bugs=["target_drifts"], rooms=["fight"])
        r = row(res, "fight")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        names = [c["name"] for c in r["checks"]]
        for idx, shooter in ((1, "host"), (2, "client")):
            hold = names.index(f"the host holds enemy {idx} in place")
            self.assertLess(hold, names.index(f"the host weakens enemy {idx}"))
            self.assertLess(hold, names.index(f"{shooter} kills enemy {idx}"))
            self.assertIn(f"enemy {idx} killed by the {shooter} is dead on the {'client' if shooter == 'host' else 'host'} copy", names, "the kill still crosses the link")

    def test_the_npc_handoff_time_does_not_include_the_length_of_the_old_owners_cell_load(self):
        # lab run 2026-10-28: "NPC handoff 1030 ms behind good and bad Wi-Fi" (78 ms clean). The host's copy answers no bridge call while it loads the cell it `coc`s into, and the
        # number waited for BOTH copies' owner views. Now the new owner's own copy is what the time ends on.
        lay = layout()
        env = labfake.FakeEnv(lay)

        class BusyHost(labfake.FakeMachine):
            def owner(self, ref):
                if self.name == "host" and self.s.cell != 1:
                    self.w.clock.sleep(3.0)           # the host's copy is in its load: the call is served seconds later
                return super().owner(ref)
        env.machines = lambda pair: (BusyHost(env.worlds[pair], "host"), BusyHost(env.worlds[pair], "client"))
        res, _ = run_lab(env=env, lay=lay, rooms=["npc"])
        r = row(res, "npc")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        m = r["metrics"]
        self.assertLess(m["npc_client_owns_ms"], 2000, m)
        for k in ("npc_client_coc_ms", "npc_client_left_ms", "npc_client_owns_ms", "npc_host_coc_ms", "npc_host_owns_ms"):
            self.assertIn(k, m)

    def test_the_spawn_room_says_when_each_copy_first_had_the_other_players_puppet_and_how_long_the_entry_had_already_taken(self):
        res, _ = run_lab(rooms=["spawn"])
        m = row(res, "spawn")["metrics"]
        for k in ("host_puppet_placed_ms", "client_puppet_placed_ms", "host_puppet_near_ms", "client_puppet_near_ms", "puppet_wait_started_ms_after_enter", "puppet_seen_ms"):
            self.assertIn(k, m)
        self.assertLessEqual(max(m["host_puppet_near_ms"], m["client_puppet_near_ms"]), m["puppet_seen_ms"] + 300)

    def test_a_patrol_step_between_two_slow_samples_is_no_snap_but_a_real_snap_still_is(self):
        # lab run 2026-10-27, behind good_wifi: "the NPC never snaps: got 327.9" - the steps are judged by what the patrol covers in the time between the samples
        res, _ = run_lab(bugs=["slow_npc_samples"], rooms=["npc"])
        r = row(res, "npc")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        self.assertGreater(r["metrics"]["npc_max_step"], 300.0, "the plain walk between two slow samples is a step over the old limit")
        res, _ = run_lab(bugs=["slow_npc_samples", "handoff_snap"], rooms=["npc"])
        r = row(res, "npc")
        self.assertEqual(r["verdict"], "FAIL")
        self.assertIn("never snaps", [c for c in r["checks"] if not c["ok"]][0]["name"])

    def test_a_menu_an_earlier_step_left_open_is_closed_before_the_next_room_starts(self):
        # lab run 2026-10-27: the host's try at the locked door opened the lockpick menu, which stayed open and ate the next open and the VATS of the next room
        res, _ = run_lab(bugs=["locked_try_opens_lockpick"], rooms=["door", "vats"])
        for room in ("door", "vats"):
            r = row(res, room)
            self.assertEqual(r["verdict"], "PASS", f"{room}: {r['reason']}")

    def test_a_loot_room_whose_guest_cannot_take_skips_that_take_and_still_checks_the_rest(self):
        lay = layout()

        class NoGuestTake(labfake.FakeMachine):
            def can_take(self):
                return (self.name != "client", "no driver for the guest's container menu")
        env = labfake.FakeEnv(lay)
        env.machines = lambda pair: (NoGuestTake(env.worlds[pair], "host"), NoGuestTake(env.worlds[pair], "client"))
        res, _ = run_lab(env=env, lay=lay, rooms=["loot"])
        r = row(res, "loot")
        self.assertEqual(r["verdict"], "PASS", r["reason"])
        skipped = [c for c in r["checks"] if c.get("status") == "skipped"]
        self.assertEqual([c["name"] for c in skipped], ["client takes 1 from the container"])
        self.assertIn("no driver", skipped[0]["note"])
        self.assertIn("takers got exactly what they took, nobody else got anything", [c["name"] for c in r["checks"]])

    def test_a_vats_kill_that_reads_minus_14_on_the_host_and_1_on_the_client_passes(self):          # twin vats_compares_kill_health
        res, _ = run_lab(bugs=["kill_health_split"], rooms=["vats"])
        r = row(res, "vats")
        self.assertEqual(r["verdict"], "PASS", [c for c in r["checks"] if not c["ok"]])
        names = [c["name"] for c in r["checks"]]
        self.assertTrue(any("is dead on both copies" in n for n in names))
        self.assertFalse(any("shot" in n and "health agrees" in n and n.startswith("host VATS shot 2") for n in names), "the killing shot's health is not compared")

    def test_a_vats_health_that_really_differs_on_a_living_target_still_fails(self):
        res, _ = run_lab(bugs=["vats_double"], rooms=["vats"])
        self.assertEqual(row(res, "vats")["verdict"], "FAIL")
        self.assertIn("health agrees", failed(row(res, "vats"))[0]["name"])

    def test_a_bug_in_one_room_leaves_the_others_passing(self):
        res, _ = run_lab(bugs=["door_unsynced"])
        self.assertEqual(sorted(r["room"] for r in res["rooms"] if r["verdict"] != "PASS"), ["door"])

    def test_an_unreadable_quest_variable_is_skipped_not_failed(self):
        res, _ = run_lab(bugs=["var_unreadable"], rooms=["quest"])
        r = row(res, "quest")
        self.assertEqual(r["verdict"], "PASS")
        self.assertEqual([c["status"] for c in r["checks"] if c.get("status")], ["skipped"])
        self.assertIn("Skipped checks", R.render_summary(dict(res, previous={"stamp": None, "changes": []})))

    def test_the_failed_check_has_expected_and_actual_for_both_machines(self):
        res, _ = run_lab(bugs=["stage_unsynced"])
        c = failed(row(res, "quest"))[0]
        self.assertEqual(c["expected"], 10)
        self.assertEqual(c["actual"], {"host": 10, "client": 0})

    def test_the_first_digest_key_that_differed_is_recorded_with_both_values(self):
        res, _ = run_lab(bugs=["container_unsynced"])
        r = row(res, "loot")
        fd = r["first_divergence"]
        self.assertTrue(fd and fd["key"].startswith("container/NVCoopLab.esp:000841"), fd)
        self.assertNotEqual(fd["host"], fd["client"])
        self.assertTrue(r["digests_moved"]["mismatch"] >= 1)

    def test_counters_that_moved_are_listed(self):
        res, _ = run_lab(bugs=["digest_noise"])
        r = row(res, "fight")
        self.assertTrue(any("repairs" in k for k in r["counters_moved"]), r["counters_moved"])

    def test_a_pass_has_no_divergence_and_no_failure_fields(self):
        res, _ = run_lab()
        self.assertNotIn("first_divergence", row(res, "loot"))

    def test_metrics_are_numbers(self):
        res, _ = run_lab()
        m = row(res, "fight")["metrics"]
        self.assertTrue(0 < m["host_kill_ms"] < 3000 and 0 < m["client_kill_ms"] < 3000, m)
        self.assertIn("host_err_p95", row(res, "walk")["metrics"])

    def test_thresholds_can_be_changed_per_run(self):
        res, _ = run_lab(rooms=["walk"], params=dict(PARAMS, **{"walk.err_p95": 1.0}))
        self.assertIn("error p95", failed(row(res, "walk"))[0]["name"])
        self.assertRaises(ValueError, lab.parse_params, ["walk.nope=1"])
        self.assertEqual(lab.parse_params(["walk.err_p95=7"])["walk.err_p95"], 7.0)

    def test_the_lab_is_checked_by_placed_ref_not_by_count(self):
        # the enemies are looked up by the layout's own ids on both copies: a copy that resolves one to another id fails before any fight
        res, _ = run_lab(bugs=["ref_id_differs"], rooms=["fight"])
        c = failed(row(res, "fight"))[0]
        self.assertIn("NVCoopLab.esp:000831", json.dumps(c["actual"]))



class SpawnEntry(unittest.TestCase):
    """A pair that got into the lab and connected keeps its other rooms even when spawn's idle digest check fails (a finding, not a block)."""

    @staticmethod
    def _row(ok_names):
        names = ["host is moved to its spawn marker", "client is moved to its spawn marker", "both copies are in the lab cell",
                 "both copies are connected", "each copy shows the other player's puppet near him", "idle digest stayed clean for the window"]
        return {"checks": [{"name": n, "ok": n in ok_names} for n in names]}

    def test_entered_when_the_entry_checks_pass(self):
        r = self._row({"host is moved to its spawn marker", "client is moved to its spawn marker", "both copies are in the lab cell",
                       "both copies are connected"})
        self.assertTrue(lab.spawn_entered(r))

    def test_not_entered_when_one_copy_is_not_moved(self):
        r = self._row({"host is moved to its spawn marker", "both copies are in the lab cell", "both copies are connected"})
        self.assertFalse(lab.spawn_entered(r))

    def test_not_entered_when_not_connected(self):
        r = self._row({"host is moved to its spawn marker", "client is moved to its spawn marker", "both copies are in the lab cell"})
        self.assertFalse(lab.spawn_entered(r))

class Evidence(unittest.TestCase):
    """A failure reason is read whole by an AI: results.json keeps 400 characters of 'actual' (the first run cut them at 60 with a ~)."""
    WHY = ("take: FAILED postcondition (5.09s) {'why': 'destination count wrong', 'before': 3, 'wanted': 2, 'got': 3, 'item': '0x15169'} "
           "and a long tail of bot data " + "z" * 200)

    def test_short_keeps_400_characters_of_actual_and_200_of_expected(self):
        self.assertEqual(K.short("a" * 400), "a" * 400)
        self.assertTrue(K.short("a" * 401).endswith("~"))
        self.assertEqual(len(K.short("a" * 401)), 400)
        w = labfake.World(layout())
        ctx = K.Ctx("x", layout(), labfake.FakeMachine(w, "host"), labfake.FakeMachine(w, "client"), clock=w.clock)
        rec = ctx._rec("c", False, "e" * 300, self.WHY, "", "SYNC", "", 0.0)
        self.assertEqual(rec["actual"], self.WHY)
        self.assertEqual(len(rec["expected"]), 200)
        self.assertEqual(K.short(12.3456), 12.35)

    def failing_row(self):
        res, _ = run_lab(bugs=["container_unsynced"], rooms=["loot"])
        r = row(res, "loot")
        c = failed(r)[0]
        c["actual"], c["expected"] = K.short(self.WHY), "taken"
        r["reason"] = K._why(c)
        return res, r

    def test_the_reason_and_the_summary_keep_the_why(self):
        res, r = self.failing_row()
        self.assertIn("'why': 'destination count wrong'", r["reason"])
        self.assertNotIn("~", r["reason"])
        txt = R.render_summary(res)
        self.assertIn("'why': 'destination count wrong', 'before': 3, 'wanted': 2, 'got': 3", txt)

    def test_a_tight_summary_still_has_the_key_clause(self):
        res, r = self.failing_row()
        for tight in (1, 2, 3):
            blk = "\n".join(R.failure_block(r, tight))
            self.assertIn("destination count wrong", blk, tight)
        res["previous"] = {"stamp": "20990101-000000", "changes": [f"room{i}: PASS -> FAIL (REGRESSED) " + "y" * 200 for i in range(30)]}
        self.assertLessEqual(len(R.render_summary(res).encode("utf-8")), 5000)


class Boxes(unittest.TestCase):
    def test_a_room_over_its_time_box_is_a_timeout_with_the_checks_so_far(self):
        res, _ = run_lab(rooms=["fight"], room_budget_s=10.0, params=dict(PARAMS, **{"spawn.idle_s": 1.0}))
        self.assertEqual(row(res, "spawn")["verdict"], "PASS")
        r = row(res, "fight")
        self.assertEqual((r["verdict"], r["class"]), ("TIMEOUT", "ENV"))
        self.assertTrue(r["checks"])
        self.assertLess(r["seconds"], 12)

    def test_the_run_budget_stops_the_rooms_that_have_not_started(self):
        res, _ = run_lab(budget_s=26.0)
        verdicts = {r["room"]: r["verdict"] for r in res["rooms"]}
        self.assertEqual(verdicts["spawn"], "PASS")
        late = [r for r in res["rooms"] if r["room"] != "spawn"]
        self.assertTrue(late and all(r["verdict"] == "TIMEOUT" for r in late), verdicts)
        self.assertTrue(any("budget" in r["reason"] for r in late))
        self.assertEqual(res["verdict"], "TIMEOUT")

    def test_a_room_never_runs_longer_than_what_is_left_of_the_run(self):
        res, _ = run_lab(budget_s=40.0)
        self.assertLessEqual(max(r["seconds"] for r in res["rooms"]), 41.0)

    def test_a_crashed_copy_is_a_crash_and_the_pair_stops(self):
        class Crashy(labfake.FakeEnv):
            def prepare(self, pair):
                super().prepare(pair)
                import heapq
                w = self.worlds[pair]
                heapq.heappush(w.q, (40.0, 0, "", lambda: w.crash("client")))
        lay = layout()
        res, _ = run_lab(env=Crashy(lay), lay=lay)
        r = row(res, "walk")
        self.assertEqual((r["verdict"], r["class"]), ("FAIL", "CRASH"))
        self.assertEqual(res["class"], "CRASH")
        after = [x for x in res["rooms"] if x["room"] not in ("spawn", "walk")]
        self.assertTrue(after and all(x["verdict"] == "BLOCKED" and "exited" in x["reason"] for x in after))

    def test_a_pair_that_cannot_get_into_the_lab_blocks_its_other_rooms(self):
        res, _ = run_lab(bugs=["client_stays_out"], pairs=2)
        for r in res["rooms"]:
            self.assertNotEqual(r["verdict"], "PASS")
        self.assertTrue(any("did not get into the lab" in r["reason"] for r in res["rooms"]))

    def test_a_pair_that_cannot_boot_blocks_its_rooms_with_the_reason(self):
        class NoBoot(labfake.FakeEnv):
            def prepare(self, pair):
                return ("ENV", "owner is playing") if pair == 1 else super().prepare(pair)
        lay = layout()
        res, _ = run_lab(env=NoBoot(lay), lay=lay, pairs=2)
        mine = [r for r in res["rooms"] if r["pair"] == 1]
        self.assertTrue(mine and all(r["verdict"] == "BLOCKED" and r["class"] == "ENV" and "owner is playing" in r["reason"] for r in mine))
        self.assertTrue(all(r["verdict"] == "PASS" for r in res["rooms"] if r["pair"] == 0))

    def test_a_layout_without_the_refs_of_a_room_blocks_only_that_room(self):
        d = LY.sample_layout()
        del d["rooms"]["fight"]["refs"]["enemy3"]
        res, _ = run_lab(lay=LY.Layout(d))
        r = row(res, "fight")
        self.assertEqual((r["verdict"], r["class"]), ("BLOCKED", "TEST"))
        self.assertIn("enemy", r["reason"])
        self.assertEqual(row(res, "door")["verdict"], "PASS")

    def test_refs_missing_on_both_copies_are_a_test_problem_not_a_sync_one(self):
        enemies = layout().refs("fight", "enemy")

        class NoPlugin(labfake.FakeMachine):
            def ref_id(self, ref):
                return "" if ref in enemies else super().ref_id(ref)
        lay = layout()
        env = labfake.FakeEnv(lay)
        env.machines = lambda pair: (NoPlugin(env.worlds[pair], "host"), NoPlugin(env.worlds[pair], "client"))
        res, _ = run_lab(env=env, lay=lay, rooms=["fight"])
        r = row(res, "fight")
        self.assertEqual((r["verdict"], r["class"]), ("BLOCKED", "TEST"), r["reason"])

    def test_confirm_reruns_only_passed_rooms_on_a_fresh_world(self):
        lay = layout()
        env = labfake.FakeEnv(lay)
        resets = []
        orig = env.reset
        env.reset = lambda p: (resets.append(p), orig(p))[1]
        res, _ = run_lab(env=env, lay=lay, rooms=["door", "quest"], confirm=2)
        self.assertEqual(len(resets), 2 * 3)             # spawn, door, quest: two fresh loads each
        for r in res["rooms"]:
            self.assertEqual(r["confirm"], {"runs": 2, "passed": 2})
        res, _ = run_lab(bugs=["door_unsynced"], rooms=["door"], confirm=2)
        self.assertNotIn("confirm", row(res, "door"))    # a failure is never rerun


class Reports(unittest.TestCase):
    def failing(self, n=3):
        res, _ = run_lab(bugs=["container_unsynced", "door_unsynced", "host_kill_unseen", "stage_unsynced", "no_handoff"][:n], pairs=2)
        return res

    def test_summary_is_short_and_has_the_table_the_blocks_and_the_comparison(self):
        res = self.failing(5)
        res["previous"] = {"stamp": None, "changes": []}
        txt = R.render_summary(res)
        self.assertLessEqual(len(txt.encode("utf-8")), 5000)
        self.assertIn("| room | pair | verdict |", txt)
        for room in ("fight", "loot", "door", "npc", "quest"):
            self.assertIn(f"### FAIL {room}", txt)
        self.assertIn("## vs previous", txt)
        self.assertNotIn(EM_DASH, txt)

    def test_a_failure_block_is_three_to_six_lines_with_numbers_and_the_source_area(self):
        res = self.failing(2)
        for r in res["rooms"]:
            if r["verdict"] == "FAIL":
                blk = R.failure_block(r)
                self.assertTrue(4 <= len(blk) <= 7, blk)       # the heading plus 3 to 6 lines
                joined = "\n".join(blk)
                self.assertIn("expected", joined)
                self.assertIn("likely source: plugin/", joined)
        loot = "\n".join(R.failure_block(row(res, "loot")))
        self.assertIn("first digest divergence: container/", loot)
        self.assertIn("plugin/loot", loot)

    def test_the_summary_stays_under_5kb_even_with_long_texts(self):
        res = self.failing(5)
        for r in res["rooms"]:
            r["reason"] = "x" * 900
            for c in r["checks"]:
                c["name"] = "n" * 400
                c["actual"] = {"host": "a" * 300, "client": "b" * 300}
        res["previous"] = {"stamp": "20990101-000000", "changes": [f"room{i}: PASS -> FAIL (REGRESSED) " + "y" * 200 for i in range(30)]}
        self.assertLessEqual(len(R.render_summary(res).encode("utf-8")), 5000)

    def test_write_run_makes_both_files_and_compares_with_the_newest_earlier_run(self):
        with tempfile.TemporaryDirectory() as t:
            root = Path(t)
            a = dict(run_lab(bugs=["door_unsynced"])[0], stamp="20990101-000001")
            R.write_run(root / a["stamp"], a, root)
            b = dict(run_lab()[0], stamp="20990101-000002")
            R.write_run(root / b["stamp"], b, root)
            c = dict(run_lab(bugs=["stage_unsynced"])[0], stamp="20990101-000003")
            R.write_run(root / c["stamp"], c, root)
            txt = (root / c["stamp"] / "summary.md").read_text(encoding="utf-8")
            self.assertIn("vs previous (20990101-000002)", txt)
            self.assertIn("quest: PASS -> FAIL (REGRESSED)", txt)
            self.assertNotIn("door:", txt.split("## vs previous")[1])
            txt_b = (root / b["stamp"] / "summary.md").read_text(encoding="utf-8")
            self.assertIn("door: FAIL -> PASS (FIXED)", txt_b)
            self.assertEqual(json.loads((root / c["stamp"] / "results.json").read_text(encoding="utf-8"))["previous"]["stamp"], "20990101-000002")
            self.assertEqual(R.reprint(root / c["stamp"]), txt)
            self.assertEqual(R.previous_run(root, "20990101-000001"), None)
            self.assertEqual(R.previous_run(root, "20990101-000003")["stamp"], "20990101-000002")

    def test_metric_changes_are_listed(self):
        a = {"stamp": "a", "rooms": [{"room": "walk", "verdict": "PASS", "metrics": {"host_err_p95": 100.0}, "checks": []}]}
        b = {"stamp": "b", "rooms": [{"room": "walk", "verdict": "PASS", "metrics": {"host_err_p95": 180.0}, "checks": []}]}
        self.assertIn("walk.host_err_p95: 100 -> 180", R.compare(b, a)["changes"])
        small = {"stamp": "c", "rooms": [dict(b["rooms"][0], metrics={"host_err_p95": 110.0})]}
        self.assertEqual(R.compare(small, a)["changes"], [])

    def test_results_json_has_the_documented_fields(self):
        res = self.failing(2)
        for k in ("schema", "stamp", "build", "mode", "pairs", "seconds", "verdict", "rooms", "budget_min", "layout_name"):
            self.assertIn(k, res)
        for r in res["rooms"]:
            for k in ("room", "pair", "verdict", "class", "seconds", "checks", "area"):
                self.assertIn(k, r)
            self.assertIn(r["verdict"], ("PASS", "FAIL", "BLOCKED", "TIMEOUT"))
        json.dumps(res)


class Cli(unittest.TestCase):
    def cli(self, *args):
        cp = subprocess.run([sys.executable, "-I", str(LAB / "lab.py")] + list(args), capture_output=True, text=True, cwd=ROOT, timeout=300)
        return cp.returncode, cp.stdout, cp.stderr

    def test_run_fake_writes_the_files_and_exits_0(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("run", "--fake", "--pairs", "2", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 0, out)
            d = Path(t) / "20990101-000000"
            self.assertTrue((d / "results.json").is_file() and (d / "summary.md").is_file())
            self.assertLess((d / "summary.md").stat().st_size, 5000)
            rc, out2, _ = self.cli("summary", str(d))
            self.assertEqual(rc, 0)
            self.assertEqual(out2.strip(), (d / "summary.md").read_text(encoding="utf-8").strip())

    def test_a_fake_bug_gives_exit_1_and_names_the_check(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("run", "--fake", "--bug", "lock_unsynced", "--rooms", "door", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 1)
            self.assertIn("### FAIL door", out)
            self.assertIn("the door is unlocked on both copies", out)

    def test_smoke_runs_only_spawn(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("smoke", "--fake", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 0, out)
            rooms = json.loads((Path(t) / "20990101-000000" / "results.json").read_text(encoding="utf-8"))["rooms"]
            self.assertEqual([r["room"] for r in rooms], ["spawn"])

    def test_bad_arguments_exit_2(self):
        self.assertEqual(self.cli("run", "--fake", "--rooms", "nope")[0], 2)
        self.assertEqual(self.cli("run", "--fake", "--bug", "nope")[0], 2)
        self.assertEqual(self.cli("run", "--fake", "--param", "x.y=1")[0], 2)
        self.assertEqual(self.cli("summary", str(ROOT / "no" / "such"))[0], 2)

    def test_a_missing_layout_is_blocked_with_a_clear_reason_and_still_writes_results(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("run", "--layout", str(Path(t) / "layout.json"), "--rooms", "fight", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 3, out)
            res = json.loads((Path(t) / "20990101-000000" / "results.json").read_text(encoding="utf-8"))
            self.assertEqual(res["verdict"], "BLOCKED")
            self.assertTrue(all("no usable layout" in r["reason"] for r in res["rooms"]))


class Live(unittest.TestCase):
    """LiveMachine against the bot layer's own fake bridge: the calls it makes are the ones the bots already make."""

    def setUp(self):
        sys.path.insert(0, str(ROOT / "tools"))
        from bots import Bot, FakeBridge, World
        import lab_live
        self.lab_live = lab_live
        w = World()
        w.add_container(0x600, (500, 0, 0), {0x500: 5})
        w.add_door(0x601, (200, 200, 0), locked=True)
        w.add_actor(0x603, (1000, 1000, 0))
        w.add_item(0x604, 0x500, (400, 400, 0))
        w.add_terminal(0x605, (0, 300, 0))
        w.set_stage(0x700, 20)
        self.lines = []
        lines = self.lines

        class Bridge(FakeBridge):               # the bots' fake knows neither setstage nor KillActor: record the lines the lab sends
            def exec(self, cmd, timeout_ms=None, *, ref=None):
                if cmd.lower().startswith("setstage") or cmd == "KillActor":
                    lines.append((cmd, ref))
                    if cmd == "KillActor":
                        w.things[int(ref.split(":")[1], 16)].health = 0
                    else:
                        w.stages[int(cmd.split()[1], 16)] = int(cmd.split()[2])
                    return {"ok": True, "output": ""}
                return super().exec(cmd, timeout_ms, ref=ref)
        self.loot_calls = []
        loot_calls = self.loot_calls
        self.helpers_on = True
        test_self = self

        class Bridge2(Bridge):                  # the two bridge methods this round added to the lab: coop.loot (grab) and bot.questvar
            def call(self, method, params=None, timeout_s=35.0):
                from bots.fake import RpcError
                if method == "coop.loot":
                    g = (params or {}).get("grab", {})
                    loot_calls.append((getattr(self, "_menu", "none"), dict(g)))
                    if not test_self.helpers_on:
                        raise RpcError(-32000, "test helpers disabled (start the game with NVCOOP_TEST_HELPERS=1)", method)
                    if int(g.get("count", 0)) <= 0:
                        raise RpcError(-32000, "grab refused: bad item or count", method)
                    cont, item = w.things[int(g["container"], 16)], int(g["item"], 16)
                    cont.items[item] -= int(g["count"])
                    self.me.items[item] = self.me.items.get(item, 0) + int(g["count"])
                    return {"grab": True, "phase": "both"}
                if method == "bot.questvar":
                    key = ((params or {}).get("quest"), (params or {}).get("var"))
                    if key not in test_self.quest_vars:
                        raise RpcError(-32000, "quest " + str(key[0]) + " has no readable variable " + str(key[1]), method)
                    return {"value": test_self.quest_vars[key]}
                return super().call(method, params, timeout_s)
        self.quest_vars = {}
        self.nv = Bridge2(w)
        self.m = lab_live.LiveMachine(self.nv, "host", Bot(self.nv, "t"))
        self.f = lambda i: f"FalloutNV.esm:{i:06X}"

    def test_reads_and_moves(self):
        self.assertEqual(self.m.pos(self.f(0x603)), (1000.0, 1000.0, 0.0))
        self.assertTrue(self.m.alive(self.f(0x603)))
        self.assertTrue(self.m.enabled(self.f(0x604)))
        self.assertTrue(self.m.exists(self.f(0x604)))
        self.assertFalse(self.m.exists("NVCoopLab.esp:000999"))
        self.assertEqual(self.m.stage(self.f(0x700)), 20)
        ok, why = self.m.goto_ref(self.f(0x600), 50.0)
        self.assertTrue(ok, why)
        self.assertLess(abs(self.m.pos()[0] - 550.0), 60)

    def test_container_loot_doors_and_terminal(self):
        item, cont = self.f(0x500), self.f(0x600)
        self.assertEqual(self.m.item_count(item, cont), 5)
        self.m.goto_ref(cont, 50.0)
        ok, why = self.m.take(cont, item, 2)
        self.assertTrue(ok, why)
        self.assertEqual((self.m.item_count(item, cont), self.m.item_count(item)), (3, 2))
        self.m.goto_ref(self.f(0x604), 50.0)
        ok, why = self.m.pickup(self.f(0x604))
        self.assertTrue(ok, why)
        self.assertFalse(self.m.enabled(self.f(0x604)))
        door = self.f(0x601)
        self.assertTrue(self.m.locked(door))
        self.assertEqual(self.m.door_state(door), "closed")
        self.m.goto_ref(door, 50.0)
        self.assertFalse(self.m.open_door(door)[0])
        self.nv.world.things[0x601].locked = False
        self.assertTrue(self.m.open_door(door)[0])
        self.assertEqual(self.m.door_state(door), "open")
        self.assertTrue(self.m.close_door(door)[0])

    def test_arm_and_kill_use_the_console(self):
        ok, why = self.m.arm()
        self.assertTrue(ok, why)
        self.assertTrue(self.m.kill(self.f(0x603))[0])
        self.assertEqual(self.lines, [("KillActor", self.f(0x603))])
        self.assertFalse(self.m.alive(self.f(0x603)))

    def test_set_stage_and_a_missing_bridge_method_is_a_clear_answer(self):
        ok, why = self.m.set_stage(self.f(0x700), 30)
        self.assertTrue(ok, why)
        self.assertEqual(self.lines, [("setstage 00000700 30", None)])
        self.assertEqual(self.m.stage(self.f(0x700)), 30)
        self.assertIsNone(self.m.var(self.f(0x700), "labvar"))      # the fake game cannot read quest variables: the room skips the check
        self.assertFalse(self.m.set_var(self.f(0x700), "labvar", 3)[0])

    def stub(self, **kw):
        """A LiveMachine on a stub bot: the bot calls the lab makes, answered as the real bot answers a failure (a falsy ActionResult)."""
        from bots.bot import ActionResult

        class StubBot:
            auto_dismiss = False

            def __init__(self):
                self.me, self.hops, self.dead, self.seen, self.order, self.out, self.hp = [0.0, 0.0, 0.0], [], False, [], [], True, 30.0
                self.__dict__.update(kw)

            def pos(self, ref=None):
                return tuple(self.me if ref is None else {"T": self.target, "S": self.stand}[ref])

            def alive(self, ref):
                return not self.dead

            def health(self, ref):
                return self.hp

            def _weapon_note(self):
                return {"weapon_out": self.out, "weapon_equipped": True}

            def aim_at(self, ref):
                self.order.append("aim")
                return ActionResult(True, "", {}, "aim_at", 0.1)

            def goto_pos(self, x, y, z, **k):
                self.hops.append((x, y))
                self.me = [x, y, z]
                return ActionResult(True, "", {}, "goto", 0.1)
        b = StubBot()
        return self.lab_live.LiveMachine(self.nv, "host", b), b, ActionResult

    def test_a_failed_walk_says_why_it_failed_not_that_the_thread_gave_no_result(self):
        # the first in-game run: "client walks to waypoint 1: walk thread gave no result". A failed ActionResult is falsy, so `r and r.ok` hid its reason
        m, b, AR = self.stub()
        why_bot = "stuck: 340 units left (wanted within 100), no menu in the way, the avatar never left its start (0 units moved)"
        b.goto_pos = lambda *a, **k: AR(False, why_bot, {}, "goto", 3.0)
        ok, why = m.walk_to(1.0, 2.0, 3.0, 5.0)
        self.assertFalse(ok)
        self.assertIn(why_bot, why)
        self.assertNotIn("no result", why)

    def test_a_walk_that_raises_reports_the_exception_and_a_lost_game_is_not_swallowed(self):
        m, b, AR = self.stub()

        def boom(*a, **k):
            raise ValueError("bridge said no")
        b.goto_pos = boom
        ok, why = m.walk_to(1.0, 2.0, 3.0, 5.0)
        self.assertFalse(ok)
        self.assertIn("ValueError: bridge said no", why)

        class ConnectionLost(Exception):
            pass

        def lost(*a, **k):
            raise ConnectionLost("gone")
        b.goto_pos = lost
        with self.assertRaises(ConnectionLost):
            m.walk_to(1.0, 2.0, 3.0, 5.0)

    def test_a_live_machine_steers_its_walks_with_the_console_line(self):
        # bot.look changes nothing on a client copy (in game 2026-10-27: the client walked 1700 units straight north); the console's SetAngle works on both
        self.assertTrue(self.m.bot.look_via_console)

    def test_a_guest_takes_through_the_helper_with_the_container_closed_like_a_quick_loot_mod(self):
        # measured 2026-10-27: the guest's console RemoveItem on the chest changes nothing (the plugin blocks it on purpose), its AddItem does. 2026-10-28: a grab with the
        # container menu OPEN is healed away at the close (guest 2, host 3), so the guest takes with the container closed and the loot service's menu-less path decides.
        from bots import Bot
        guest = self.lab_live.LiveMachine(self.nv, "client", Bot(self.nv, "g"))
        self.assertFalse(guest.can_take()[0], "off by default: the helper path did not reach the host when measured")
        self.assertIn("LAB_GUEST_TAKE=helper", guest.can_take()[1])
        os.environ["LAB_GUEST_TAKE"] = "helper"
        self.addCleanup(os.environ.pop, "LAB_GUEST_TAKE", None)
        self.assertEqual(guest.can_take(), (True, ""))
        cont, item = self.f(0x600), self.f(0x500)
        self.assertEqual(guest.item_count(item), 0)
        self.assertEqual(guest.take(cont, item, 1), (True, ""))
        self.assertEqual(guest.item_count(item), 1)
        self.assertEqual(guest.item_count(item, cont), 4)
        grabs = [(menu, g) for menu, g in self.loot_calls if int(g.get("count", 0))]
        self.assertEqual([m for m, _ in grabs], ["none"], "no container menu was open when the item moved")
        self.assertEqual(grabs[0][1]["phase"], "both")
        self.assertIn("loot status after the take", guest.take_note)

    def test_a_guest_take_that_cannot_move_the_item_fails_with_the_reason(self):
        from bots import Bot
        guest = self.lab_live.LiveMachine(self.nv, "client", Bot(self.nv, "g"))
        os.environ["LAB_GUEST_TAKE"] = "helper"
        self.addCleanup(os.environ.pop, "LAB_GUEST_TAKE", None)
        self.helpers_on = False
        ok, why = guest.can_take()
        self.assertFalse(ok)
        self.assertIn("NVCOOP_TEST_HELPERS", why)
        ok, why = guest.take(self.f(0x600), self.f(0x500), 1)
        self.assertFalse(ok)
        self.assertIn("test helpers disabled", why)
        self.assertEqual((self.nv.call("bot.menu", {}) or {}).get("kind"), "none")

    def test_the_host_still_takes_with_console_lines_and_never_opens_the_menu(self):
        self.assertEqual(self.m.can_take(), (True, ""))
        self.assertEqual(self.m.take(self.f(0x600), self.f(0x500), 2), (True, ""))
        self.assertEqual([g for _, g in self.loot_calls if int(g.get("count", 0))], [])

    def test_a_quest_variable_is_read_through_the_bridge_on_each_copy_and_an_unreadable_one_says_why(self):
        q = self.f(0x700)
        qn = self.m.bot._norm(q)[0]              # the form the bridge is asked about: "Plugin.esm:HEX"
        self.quest_vars[(qn, "labvar")] = 7.0
        self.assertEqual(self.m.var(q, "labvar"), 7.0)
        self.assertEqual(self.m.var_why, "")
        self.assertIsNone(self.m.var(q, "nope"))
        self.assertIn("no readable variable nope", self.m.var_why)
        self.quest_vars[(qn, "labvar")] = 9.0
        self.assertEqual(self.m.var(q, "labvar"), 9.0, "the live value, read again")

    def test_a_walk_that_arrives_is_ok(self):
        m, b, AR = self.stub()
        self.assertEqual(m.walk_to(1.0, 2.0, 3.0, 5.0), (True, ""))

    def test_the_host_picks_up_through_the_console_and_a_guest_through_the_bridge_method(self):
        # activate.ref (last argument 0) deleted the lab's placed Stimpak without giving it to the player (measured 2026-10-27); the console line gives it, but
        # a guest's console line is dropped by the script block
        m, b, AR = self.stub()
        b.pickup = lambda ref, **k: (b.seen.append(("pickup", k)), AR(True, "", {}, "pickup", 0.1))[1]
        self.assertEqual(m.pickup("R"), (True, ""))
        guest = self.lab_live.LiveMachine(self.nv, "client", b)
        self.assertEqual(guest.pickup("R"), (True, ""))
        self.assertEqual(b.seen, [("pickup", {"console": True}), ("pickup", {})])

    def test_two_vats_entries_close_together_wait_for_the_cooldown(self):
        # the game crashed twice in the first ticks of a VATS entry that followed another one within seconds (FalloutNV.exe+400F66)
        m, b, AR = self.stub()
        b.vats_target = lambda ref, limb=None, **k: AR(True, "", {}, "vats_target", 1.0)
        slept = []
        real = self.lab_live.time.sleep
        self.lab_live.time.sleep = lambda s: slept.append(s)
        try:
            self.assertTrue(m.vats("R")[0])
            first = sum(slept)
            self.assertTrue(m.vats("R")[0])
        finally:
            self.lab_live.time.sleep = real
        self.assertGreaterEqual(sum(slept) - first, self.lab_live.VATS_COOLDOWN_S - 0.5, "the second entry waited for the first one to be over")

    def test_vats_asks_for_no_limb(self):
        # plugin/bots answers any limb with "unsupported: limb selection in VATS is not implemented"
        m, b, AR = self.stub()
        b.vats_target = lambda ref, limb=None, **k: (b.seen.append(limb), b.order.append("vats"), AR(True, "", {"hit": True}, "vats_target", 1.0))[2]
        self.assertTrue(m.vats("R")[0])
        self.assertEqual(b.order, ["aim", "vats"], "VATS only opens on a target in view: the player faces it first")
        self.assertEqual(b.seen, [None])
        self.assertTrue(m.vats("R", "torso")[0])
        self.assertEqual(b.seen, [None, "Torso"])

    def test_vats_takes_a_holstered_weapon_out_first_and_leaves_a_drawn_one_alone(self):
        m, b, AR = self.stub()
        calls = []
        m.nv = type("NV", (), {"call": lambda s, method, params=None, **k: (calls.append((method, params)), b.__setattr__("out", True))[0]})()
        b.vats_target = lambda ref, limb=None, **k: AR(True, "", {}, "vats_target", 1.0)
        b.out = False
        self.assertTrue(m.vats("R")[0])
        self.assertEqual(calls, [("bot.input", {"tap": ["ReadyItem"]})])
        calls.clear()
        self.assertTrue(m.vats("R")[0])             # drawn now: the toggle must not be pressed again
        self.assertEqual(calls, [])

    def test_a_vats_job_that_lost_the_race_to_the_modules_auto_fire_counts_when_the_target_was_hit(self):
        # measured 2026-10-27: target 4.8 -> -4.4 and dead, the job FAILED "vats_closed (camera mode 0 before the attack was queued)": the mod's real-time VATS
        # fired the queued attack itself. A closed VATS with an untouched target is still a failure.
        m, b, AR = self.stub()
        why_bot = "vats_target: FAILED vats_failed (5.14s) {'why': 'vats_closed (camera mode 0 before the attack was queued)'}"

        def hit_and_close(ref, limb=None, **k):
            b.hp = 4.8
            return AR(False, why_bot, {}, "vats_target", 5.1)
        b.vats_target = hit_and_close
        self.assertEqual(m.vats("R"), (True, ""))
        b.hp = 30.0
        b.vats_target = lambda ref, limb=None, **k: AR(False, why_bot, {}, "vats_target", 5.1)
        ok, why = m.vats("R")
        self.assertFalse(ok, "nothing hit the target: the closed VATS stays a failure")
        self.assertIn("vats_closed", why)

        def kill_and_close(ref, limb=None, **k):
            b.dead = True
            return AR(False, why_bot, {}, "vats_target", 5.1)
        b.dead, b.vats_target = False, kill_and_close
        self.assertEqual(m.vats("R"), (True, ""))

    def test_a_failed_vats_shot_keeps_the_whole_reason(self):
        m, b, AR = self.stub()
        why_bot = "rpc_error:unsupported: limb selection in VATS is not implemented (bot_engine.h)"
        b.vats_target = lambda ref, limb=None, **k: AR(False, why_bot, {}, "vats_target", 0.1)
        ok, why = m.vats("R", "torso")
        self.assertFalse(ok)
        self.assertIn(why_bot, why)

    def test_a_shooter_with_a_hostile_in_his_face_steps_back_to_the_stand_marker_and_shoots(self):
        # the first in-game run: "client kills enemy 2: fire: FAILED too_close: the player is 60 u..."
        m, b, AR = self.stub(target=[60.0, 0.0, 0.0], stand=[-350.0, 0.0, 0.0])

        def fire(ref):
            d = abs(b.me[0] - b.target[0])
            if d < 100:
                return AR(False, f"too_close: the player is {d:.0f} units from the target (minimum 100)", {}, "fire", 0.1)
            b.dead = True
            return AR(True, "", {}, "fire", 0.1)
        b.fire = fire
        self.assertEqual(m.shoot("T", 4, "S"), (True, ""))
        self.assertEqual(b.hops, [(-350.0, 0.0)])

    def test_without_a_usable_stand_marker_the_shooter_steps_back_along_the_line_from_the_target(self):
        m, b, AR = self.stub(target=[60.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])        # the stand marker is as close as the shooter: no use

        def fire(ref):
            if abs(b.me[0] - 60) < 100:
                return AR(False, "too_close: the player is 60 units", {}, "fire", 0.1)
            b.dead = True
            return AR(True)
        b.fire = fire
        self.assertEqual(m.shoot("T", 4, "S"), (True, ""))
        self.assertEqual(b.hops, [(60.0 - 350.0, 0.0)])

    def test_a_target_that_drifted_out_of_range_is_walked_up_to_along_the_line_not_the_stand_marker_again(self):
        # lab run 2026-10-28: the stand marker was 659 units from the drifted target and counted as "far enough", so the shooter hopped to it 24 times
        m, b, AR = self.stub(target=[659.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])

        def fire(ref):
            d = abs(b.me[0] - b.target[0])
            if d < 150 or d > 500:
                return AR(False, f"out of range {d:.0f}", {}, "fire", 0.1)
            b.dead = True
            return AR(True)
        b.fire = fire
        self.assertEqual(m.shoot("T", 6, "S"), (True, ""))
        self.assertEqual(b.hops, [(659.0 - 350.0, 0.0)])

    def test_a_stand_marker_that_is_a_good_shooting_spot_is_still_used(self):
        m, b, AR = self.stub(target=[60.0, 0.0, 0.0], stand=[-290.0, 0.0, 0.0])      # 350 from the target
        b.fire = lambda ref: (setattr(b, "dead", True) or AR(True)) if abs(b.me[0] - 60) >= 100 else AR(False, "too_close: 60 units", {}, "fire", 0.1)
        self.assertEqual(m.shoot("T", 4, "S"), (True, ""))
        self.assertEqual(b.hops, [(-290.0, 0.0)])

    def test_a_target_that_stays_out_of_reach_is_approached_nearer_each_time_and_the_reason_says_how_often(self):
        m, b, AR = self.stub(target=[2000.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])
        b.goto_pos = lambda x, y, z, **k: (b.hops.append((x, y)), AR(True, "", {}, "goto", 0.1))[1]      # the hop does not move the stub: every try sees the same 2000 units
        b.fire = lambda ref: AR(False, "out of range", {}, "fire", 0.1)
        ok, why = m.shoot("T", 4, "S")
        self.assertFalse(ok)
        self.assertEqual([round(2000.0 - x) for x, _ in b.hops], [350, 275, 200, 200])
        self.assertIn("approach", why)

    def test_a_holstered_weapon_after_a_talk_gets_the_controls_back_and_is_drawn_before_the_shot(self):
        # story run 20261028-023656 (S05, S15): "weapon still holstered after the shots, the pulses never reached the game" after a dialogue; a scene had switched the controls off
        m, b, AR = self.stub(target=[300.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])
        lines = []
        state = {"controls": False}
        m.nv = type("NV", (), {"call": lambda s, method, params=None, **k: (b.__setattr__("out", state["controls"]) or {}),
                               "exec": lambda s, cmd, **k: (lines.append(cmd), state.__setitem__("controls", True), {"ok": True})[2],
                               "eval": lambda s, expr, **k: 1.0})()
        b.out = False

        def fire(ref):
            b.dead = True
            return AR(True)
        b.fire = fire
        self.assertEqual(m.shoot("T", 4, "S"), (True, ""))
        self.assertEqual(lines, ["player.EnablePlayerControls"])

    def test_a_shot_that_reports_the_weapon_still_holstered_draws_again_and_shoots_on(self):
        m, b, AR = self.stub(target=[300.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])
        calls = []
        m.nv = type("NV", (), {"call": lambda s, method, params=None, **k: (calls.append(method), b.__setattr__("out", True), {})[2],
                               "exec": lambda s, cmd, **k: {"ok": True}, "eval": lambda s, expr, **k: 1.0})()
        b.out = True
        n = {"i": 0}

        def fire(ref):
            n["i"] += 1
            if n["i"] == 1:
                b.out = False
                return AR(False, "FAILED postcondition: weapon still holstered after the shots, the pulses never reached the game", {}, "fire", 0.5)
            b.dead = True
            return AR(True)
        b.fire = fire
        self.assertEqual(m.shoot("T", 4, "S"), (True, ""))
        self.assertEqual(n["i"], 2)
        self.assertIn("bot.input", calls, "the Ready control was tapped after the holstered report")

    def test_a_shooter_that_cannot_get_away_reports_the_too_close_reason_whole(self):
        m, b, AR = self.stub(target=[60.0, 0.0, 0.0], stand=[0.0, 0.0, 0.0])
        b.fire = lambda ref: AR(False, "too_close: the player is 60 units from the target (minimum 100); standing inside it aims at the floor, step back first", {}, "fire", 0.1)
        ok, why = m.shoot("T", 2, "S")
        self.assertFalse(ok)
        self.assertIn("step back first", why)

    def test_env_without_the_plugin_generator_is_blocked_with_a_reason(self):
        env = object.__new__(self.lab_live.LiveEnv)
        env.install_script = Path(tempfile.gettempdir()) / "no_such_dir" / "install_lab.py"
        env.layout, env.log = layout(), lambda s: None
        cls, why = self.lab_live.LiveEnv.prepare(env, 0)
        self.assertEqual(cls, "BLOCKED")
        self.assertIn("install_lab.py does not exist", why.replace("\\", "/").split("tools/lab/esp/")[-1] if "tools/lab/esp/" in why else why)

    def test_copy_info_reports_what_could_hold_the_fps_down_for_each_copy_that_is_up(self):
        # results.json fps.provision: the power throttling state of the process, the bridge's window options, the frame-rate keys of the ini files
        sys.path.insert(0, str(ROOT / "tools" / "runner"))
        import instances
        import winproc
        with tempfile.TemporaryDirectory() as t:
            d = Path(t) / "0"
            (d / "Data" / "NVSE" / "Plugins").mkdir(parents=True)
            (d / "Data" / "NVSE" / "Plugins" / "nvcoop.ini").write_text("[bridge]\nminimized=1\nfocusGuard=1\n", encoding="utf-8")
            (d / "Fallout.ini").write_text("[General]\nbAlwaysActive=1\niFPSClamp=0\n", encoding="latin-1")
            (d / "FalloutPrefs.ini").write_text("[Display]\niPresentInterval=0\niSize W=1280\niSize H=720\n", encoding="latin-1")
            env = object.__new__(self.lab_live.LiveEnv)
            env.held = {0}
            env.pv = SimpleNamespace(owned=SimpleNamespace(load=lambda: {"0": {"pid": 321}}), idir=lambda i: d, inst=instances)
            old = winproc.throttle_state
            winproc.throttle_state = lambda pid: {"control": 5, "state": 0}
            try:
                info = env.copy_info()
            finally:
                winproc.throttle_state = old
        self.assertEqual(len(info), 1)
        self.assertEqual(info[0]["throttle"], {"control": 5, "state": 0})
        self.assertEqual(info[0]["bridge"], {"minimized": "1", "focusGuard": "1"})
        self.assertEqual(info[0]["ini"]["iPresentInterval"], "0")
        self.assertEqual(info[0]["ini"]["iFPSClamp"], "0")
        self.assertEqual(info[0]["ini"]["iSize"], "1280x720")

    def test_install_runs_the_generators_script_with_the_instance_dir(self):
        with tempfile.TemporaryDirectory() as t:
            script = Path(t) / "install_lab.py"
            script.write_text("import sys, pathlib\nargs = sys.argv[1:]\nassert args[0] == '--instance-dir'\npathlib.Path(args[1], 'installed.txt').write_text('ok')\n", encoding="utf-8")
            env = object.__new__(self.lab_live.LiveEnv)
            env.install_script = script
            env.pv = SimpleNamespace(idir=lambda i: Path(t) / f"inst{i}")
            (Path(t) / "inst3").mkdir()
            self.assertIsNone(env.install(3))
            self.assertTrue((Path(t) / "inst3" / "installed.txt").is_file())
            script.write_text("import sys\nsys.exit('boom')\n", encoding="utf-8")
            self.assertIn("boom", env.install(3))


class Fps(unittest.TestCase):
    """The lab reports frames per second per copy, per room and per pass (lab_fps.py), from the plugin's frame counter."""

    def setUp(self):
        import lab_fps
        self.F = lab_fps

    def test_rates_are_frames_over_seconds_between_samples(self):
        r = self.F.rates([(0.0, 0), (1.0, 60), (2.0, 90), (3.0, 150)])
        self.assertEqual(r, [(1.0, 60.0), (1.0, 30.0), (1.0, 60.0)])

    def test_two_samples_closer_than_a_bridge_round_trip_are_one_and_a_reset_counter_ends_the_interval(self):
        r = self.F.rates([(0.0, 0), (0.1, 6), (1.0, 60), (2.0, 5), (3.0, 65)])
        self.assertEqual(r, [(1.0, 60.0), (1.0, 60.0)], "0.1 s later is skipped; 60 -> 5 is a restart, not -55 fps")

    def test_mean_is_all_frames_over_all_seconds_and_a_stall_counts_as_long_as_it_lasted(self):
        st = self.F.stats([(10.0, 60.0), (30.0, 10.0)])
        self.assertAlmostEqual(st["mean"], (600 + 300) / 40.0, places=1)
        self.assertEqual((st["n"], st["secs"], st["min"]), (2, 40.0, 10.0))

    def test_p5_is_the_speed_of_the_slowest_5_percent_of_the_time_not_of_the_samples(self):
        pairs = [(95.0, 60.0), (5.0, 10.0)]
        st = self.F.stats(pairs)
        self.assertEqual(st["p5"], 10.0)
        self.assertLess(st["p5"], st["mean"])
        many_fast = [(1.0, 60.0)] * 99 + [(1.0, 10.0)]                      # 1 percent of the time: p5 is still fast
        self.assertEqual(self.F.stats(many_fast)["p5"], 60.0)

    def test_nothing_measured_is_none_not_zero(self):
        st = self.F.stats([])
        self.assertIsNone(st["mean"])
        self.assertIsNone(st["p5"])

    def run_fps(self, fps, nets=(), rooms=None, pairs=1):
        lay = layout()
        env = labfake.FakeEnv(lay, fps=fps)
        res = lab.execute(lay, env, rooms or ALL, pairs, 1200.0, 120.0, dict(PARAMS), 0, "20990101-000000", "fake", lambda s: None, nets=list(nets), total_s=1800.0)
        return res

    def test_every_room_reports_its_fps_per_copy_and_the_copies_running(self):
        res = self.run_fps({"host": 30.0, "client": 20.0}, pairs=3)
        for r in res["rooms"]:
            f = r["fps"]
            self.assertAlmostEqual(f["host"]["mean"], 30.0, delta=1.0, msg=r["room"])
            self.assertAlmostEqual(f["client"]["mean"], 20.0, delta=1.0, msg=r["room"])
            self.assertLessEqual(f["host"]["p5"], f["host"]["mean"] + 0.01)
            self.assertGreater(f["host"]["n"], 2, r["room"])
            self.assertGreaterEqual(f["copies"], 2)
        self.assertEqual(res["fps"]["passes"]["clean"]["copies"], 6)
        self.assertEqual(res["fps"]["copies"], 6)

    def test_a_network_pass_has_its_own_fps_row_and_results_keep_no_raw_intervals(self):
        res = self.run_fps({"host": 40.0, "client": 40.0}, nets=["good_wifi"], rooms=["spawn", "walk"])
        self.assertEqual(list(res["fps"]["passes"]), ["clean", "good_wifi"])
        self.assertAlmostEqual(res["fps"]["passes"]["good_wifi"]["host"]["mean"], 40.0, delta=1.0)
        self.assertNotIn("_rates", json.dumps(res, default=str))
        for r in res["net_rooms"]:
            self.assertIn("fps", r)

    def test_the_walk_reports_the_fps_of_both_copies_during_each_leg_and_the_lower_one(self):
        res = self.run_fps({"host": 50.0, "client": 25.0}, rooms=["walk"])
        m = row(res, "walk")["metrics"]
        self.assertAlmostEqual(m["host_leg_fps_walker"], 50.0, delta=2.0)
        self.assertAlmostEqual(m["host_leg_fps_viewer"], 25.0, delta=2.0)
        self.assertAlmostEqual(m["host_leg_fps"], 25.0, delta=2.0)
        self.assertAlmostEqual(m["client_leg_fps_walker"], 25.0, delta=2.0)

    def test_the_summary_has_the_fps_table_with_the_copies_and_stays_under_5kb(self):
        res = self.run_fps({"host": 33.0, "client": 22.0}, nets=["good_wifi", "bad_wifi", "mobile_tether", "transatlantic"], pairs=3)
        text = R.render_summary(dict(res, previous={"stamp": None, "changes": []}))
        self.assertIn("## FPS per copy", text)
        self.assertIn("copies running | 6 | 6 | 6 | 6 | 6 |", text)
        self.assertIn("33/22 p5", text)
        self.assertLessEqual(len(text.encode("utf-8")), 5000)

    def test_a_run_without_fps_numbers_prints_no_table(self):
        self.assertEqual(self.F.table({"rooms": [], "fps": {}}), [])
        self.assertEqual(self.F.table({"rooms": []}), [])

    def test_a_copy_that_cannot_answer_costs_samples_not_the_room(self):
        lay = layout()
        env = labfake.FakeEnv(lay)
        orig = labfake.FakeMachine.frames if "frames" in labfake.FakeMachine.__dict__ else K.Machine.frames
        calls = {"n": 0}

        def flaky(self):
            calls["n"] += 1
            if self.name == "client" and calls["n"] % 3 == 0:
                raise RuntimeError("bridge: timed out")
            return orig(self)
        labfake.FakeMachine.frames = flaky
        try:
            res = lab.execute(lay, env, ["spawn", "quest"], 1, 1200.0, 120.0, dict(PARAMS), 0, "20990101-000000", "fake", lambda s: None)
        finally:
            del labfake.FakeMachine.frames
        self.assertEqual(res["verdict"], "PASS")
        self.assertGreater(row(res, "quest")["fps"]["client"]["n"], 0)


class Files(unittest.TestCase):
    def test_remote_script_mirrors_proof_run(self):
        txt = (ROOT / "tools" / "bench" / "remote" / "lab_run.ps1").read_text(encoding="utf-8")
        for need in ("NVCOOP_ADAPTER", "PROOF_NVSE_DIR", "PROOF_MAX_PAIRS", r"C:\fnv\lab_done.txt", "tools\\lab\\lab.py", "--pairs", "NVCOOP_LEAN"):
            self.assertIn(need, txt)
        self.assertNotIn(EM_DASH, txt)

    def test_docs_are_clean(self):
        txt = (ROOT / "docs" / "tools" / "lab.md").read_text(encoding="utf-8")
        self.assertNotIn(EM_DASH, txt)
        low = txt.lower()
        for bad in ("c:" + "\\" + "users", "mth" + "wj", "/users/"):
            self.assertNotIn(bad, low)
        for need in ("lab.py run", "results.json", "summary.md", "smoke", "BLOCKED", "Uncertain"):
            self.assertIn(need, txt)

    def test_no_em_dash_or_personal_path_in_the_sources(self):
        for p in list(LAB.glob("*.py")) + list((LAB / "tests").glob("*.py")):
            if p.parent.name == "esp":
                continue
            src = p.read_text(encoding="utf-8")
            if p.name == "test_lab.py":
                continue
            self.assertNotIn(EM_DASH, src, p.name)
            self.assertNotIn("mth" + "wj", src.lower(), p.name)

    def test_the_suite_is_registered_in_test_all(self):
        self.assertIn("tools/lab/run_tests.py", (ROOT / "tools" / "test_all.py").read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main(verbosity=1)
