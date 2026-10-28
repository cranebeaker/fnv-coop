"""Offline tests of the lab's network pass (no game): the rooms run again for each chaos profile on the fake world, the limits per profile, the report table,
the live side's relay attach (fake chaos pair, one real relay on loopback), the command line.

    python -I tools/lab/tests/test_net.py          # all of it
    python -I tools/lab/run_tests.py               # these tests too, then the mutation twins (LAB_MUTANT=<name>)
"""
from __future__ import annotations

import json
import socket
import subprocess
import sys
import tempfile
import threading
import unittest
from pathlib import Path
from types import SimpleNamespace

HERE = Path(__file__).resolve().parent
LAB = HERE.parent
ROOT = LAB.parent.parent
sys.path.insert(0, str(LAB))
sys.path.insert(0, str(HERE))

import lab                       # noqa: E402
import lab_core as K             # noqa: E402
import lab_layout as LY          # noqa: E402
import lab_net as N              # noqa: E402
import lab_report as R           # noqa: E402
import labfake                   # noqa: E402
from lab_rooms import PARAMS, ROOMS   # noqa: E402

ALL = list(LY.ROOM_ORDER)
NETS = ["good_wifi", "bad_wifi"]


def run_net(bugs=(), nets=NETS, rooms=None, pairs=1, params=None, total_s=0.0, env=None):
    lay = LY.Layout(LY.sample_layout())
    env = env or labfake.FakeEnv(lay, bugs=bugs)
    res = lab.execute(lay, env, rooms or ALL, pairs, 1200.0, 120.0, params or dict(PARAMS), 0, "20990101-000000", "fake", lambda s: None, nets=nets, total_s=total_s)
    return res, env


def prow(res, room, profile, pair=None):
    return next(r for r in res["net_rooms"] if r["room"] == room and r["profile"] == profile and (pair is None or r["pair"] == pair))


class Limits(unittest.TestCase):
    def test_every_profile_of_profiles_json_has_every_limit(self):
        profs = N.profiles()
        self.assertTrue({"lan", "good_wifi", "bad_wifi", "mobile_tether", "transatlantic", "hell"} <= set(profs), set(profs))
        for name in profs:
            for k in N.TIME_KEYS + tuple(N.UNIT_KEYS):
                self.assertIn(f"{k}@{name}", PARAMS, f"{k}@{name}")
            self.assertIn(f"spawn.idle_s@{name}", PARAMS)

    def test_a_worse_link_is_allowed_more_and_lan_nothing(self):
        ex = {n: N.extra_for(p) for n, p in N.profiles().items()}
        self.assertEqual(ex["lan"], (0.0, 0.0))
        self.assertLess(ex["good_wifi"][0], ex["bad_wifi"][0])
        self.assertLess(ex["good_wifi"][1], ex["bad_wifi"][1])
        self.assertLess(ex["bad_wifi"][0], ex["hell"][0])
        self.assertLess(ex["bad_wifi"][1], ex["hell"][1])
        self.assertGreaterEqual(ex["good_wifi"][0], N.FLOOR_S)

    def test_resends_follow_loss_and_bursts(self):
        P = N.profiles()
        self.assertEqual([N.resends(P[n]) for n in ("lan", "good_wifi", "bad_wifi", "hell")], [0, 1, 2, 3])

    def test_the_limits_are_the_clean_value_plus_the_allowance(self):
        secs, units = N.extra_for(N.profiles()["bad_wifi"])
        self.assertAlmostEqual(PARAMS["fight.latency_s@bad_wifi"], PARAMS["fight.latency_s"] + secs, places=2)
        self.assertAlmostEqual(PARAMS["walk.err_p95@bad_wifi"], PARAMS["walk.err_p95"] + units, places=0)
        self.assertLess(PARAMS["spawn.idle_s@bad_wifi"], PARAMS["spawn.idle_s"], "a network pass spends less time idle")

    def test_the_context_reads_the_profile_limit_and_falls_back_to_the_clean_one(self):
        lay = LY.Layout(LY.sample_layout())
        mk = lambda prof: K.Ctx("fight", lay, None, None, params=dict(PARAMS), profile=prof)   # noqa: E731
        self.assertEqual(mk("").p("latency_s"), PARAMS["fight.latency_s"])
        self.assertEqual(mk("bad_wifi").p("latency_s"), PARAMS["fight.latency_s@bad_wifi"])
        self.assertEqual(mk("no_such_profile").p("latency_s"), PARAMS["fight.latency_s"])
        self.assertEqual(mk("bad_wifi").p("corpse_s"), PARAMS["fight.corpse_s"], "a key with no profile limit stays the clean one")

    def test_the_limits_can_be_changed_per_profile_from_the_command_line(self):
        p = lab.parse_params(["fight.latency_s@bad_wifi=9"])
        self.assertEqual(p["fight.latency_s@bad_wifi"], 9.0)
        self.assertEqual(p["fight.latency_s"], PARAMS["fight.latency_s"])
        self.assertRaises(ValueError, lab.parse_params, ["fight.latency_s@nope=9"])

    def test_pick_nets_takes_profile_names_only(self):
        self.assertEqual(lab.pick_nets("good_wifi, bad_wifi,good_wifi"), ["good_wifi", "bad_wifi"])
        self.assertEqual(lab.pick_nets(""), [])
        with self.assertRaises(ValueError) as e:
            lab.pick_nets("good_wifi,wifi")
        self.assertIn("wifi", str(e.exception))

    def test_through_relay_needs_packets_in_both_directions(self):
        self.assertTrue(N.through_relay({"c2s": {"seen": 3}, "s2c": {"seen": 2}})[0])
        self.assertFalse(N.through_relay({"c2s": {"seen": 3}, "s2c": {"seen": 0}})[0])
        self.assertFalse(N.through_relay({"c2s": {"seen": 0}, "s2c": {"seen": 0}})[0])
        self.assertIn("client to host 0", N.through_relay({})[1])


class FakeNet(unittest.TestCase):
    def test_a_clean_world_passes_every_room_under_every_profile(self):
        res, _ = run_net(pairs=3)
        self.assertEqual(res["verdict"], "PASS", [(r["room"], r["profile"], r["reason"]) for r in res["net_rooms"] if r["verdict"] != "PASS"])
        self.assertEqual(res["nets"], NETS)
        self.assertEqual(len(res["net_rooms"]), len(NETS) * len(res["rooms"]))
        for prof in NETS:
            self.assertEqual({r["room"] for r in res["net_rooms"] if r["profile"] == prof}, set(ALL))
        for r in res["net_rooms"]:
            self.assertTrue(r["checks"] and all(c["ok"] for c in r["checks"]), (r["room"], r["profile"]))

    def test_the_clean_rows_are_the_same_as_without_a_network_pass(self):
        with_net, _ = run_net()
        without, _ = run_net(nets=[])
        self.assertEqual([(r["room"], r["verdict"], r["pair"]) for r in with_net["rooms"]], [(r["room"], r["verdict"], r["pair"]) for r in without["rooms"]])
        self.assertTrue(all("profile" not in r for r in with_net["rooms"]))
        self.assertEqual(without["net_rooms"], [])

    def test_the_network_pass_runs_at_all(self):          # twin net_pass_skipped
        res, _ = run_net()
        self.assertTrue(res["net_rooms"], "no network rows: the profiles were accepted and never run")

    def test_each_room_reports_its_timings_in_milliseconds(self):
        res, _ = run_net(nets=["bad_wifi"])
        want = {"spawn": ["puppet_seen_ms"], "walk": ["host_lag_ms", "client_lag_ms", "host_err_p95", "client_err_p95"], "fight": ["host_kill_ms", "client_kill_ms"],
                "loot": ["host_take_ms", "host_pickup_ms", "client_pickup_ms"], "door": ["unlock_ms", "host_open_ms", "client_close_ms"],
                "npc": ["npc_handoff_client_ms", "npc_handoff_host_ms"], "vats": ["vats_ms"], "quest": ["host_stage_ms", "client_stage_ms"]}
        for room, keys in want.items():
            m = prow(res, room, "bad_wifi")["metrics"]
            for k in keys:
                self.assertIsInstance(m.get(k), (int, float), f"{room}.{k} in {m}")
        for room, keys in want.items():       # the clean pass reports the same keys, so the table has numbers next to each other
            m = next(r for r in res["rooms"] if r["room"] == room)["metrics"]
            for k in keys:
                self.assertIn(k, m, f"clean {room}.{k}")

    def test_a_slower_link_shows_in_the_numbers(self):
        res, _ = run_net(nets=["good_wifi", "bad_wifi", "hell"])
        clean = next(r for r in res["rooms"] if r["room"] == "fight")["metrics"]["host_kill_ms"]
        kills = [clean] + [prow(res, "fight", p)["metrics"]["host_kill_ms"] for p in ("good_wifi", "bad_wifi", "hell")]
        self.assertEqual(kills, sorted(kills))
        self.assertLess(kills[0], kills[-1])
        stage = [next(r for r in res["rooms"] if r["room"] == "quest")["metrics"]["host_stage_ms"]] + [prow(res, "quest", p)["metrics"]["host_stage_ms"] for p in ("good_wifi", "bad_wifi", "hell")]
        self.assertEqual(stage, sorted(stage))
        self.assertLess(stage[0], stage[-1])

    def test_a_failing_room_keeps_the_numbers_it_measured_before_it_failed(self):
        res, _ = run_net(bugs=["client_kill_unseen"], rooms=["fight"], nets=["good_wifi"])
        r = prow(res, "fight", "good_wifi")
        self.assertEqual(r["verdict"], "FAIL")
        self.assertIsInstance(r["metrics"].get("host_kill_ms"), int)
        self.assertNotIn("client_kill_ms", r["metrics"])

    def test_a_bug_that_only_shows_behind_a_profile_fails_the_network_row_only(self):
        res, _ = run_net(bugs=["net_slow"])
        self.assertEqual(res["clean_verdict"], "PASS")
        self.assertEqual(res["verdict"], "FAIL")
        bad = [r for r in res["net_rooms"] if r["verdict"] == "FAIL"]
        self.assertTrue(bad)
        self.assertTrue(all(r["profile"] in NETS for r in bad))
        f = prow(res, "fight", "bad_wifi")
        self.assertEqual(f["verdict"], "FAIL")
        self.assertEqual(len([c for c in f["checks"] if not c["ok"]]), 1)
        self.assertTrue(all(r["verdict"] == "PASS" for r in res["rooms"]))

    def test_a_profile_is_judged_by_its_own_limits_not_the_clean_ones(self):          # twin net_clean_limits
        params = dict(PARAMS, **{"fight.latency_s": 0.5})
        res, _ = run_net(rooms=["fight"], nets=["hell"], params=params)
        self.assertEqual(next(r for r in res["rooms"] if r["room"] == "fight")["verdict"], "PASS", "the clean pass still meets the tight clean limit")
        r = prow(res, "fight", "hell")
        self.assertEqual(r["verdict"], "PASS", [c for c in r["checks"] if not c["ok"]])
        self.assertGreater(r["metrics"]["host_kill_ms"], 500, "behind hell the kill takes longer than the clean limit, and that is allowed")

    def test_a_limit_set_for_one_profile_fails_that_profile_only(self):
        params = dict(PARAMS, **{"fight.latency_s@bad_wifi": 0.2})
        res, _ = run_net(rooms=["fight"], params=params)
        self.assertEqual(prow(res, "fight", "good_wifi")["verdict"], "PASS")
        self.assertEqual(prow(res, "fight", "bad_wifi")["verdict"], "FAIL")

    def test_each_profile_starts_on_a_fresh_lab(self):
        env = labfake.FakeEnv(LY.Layout(LY.sample_layout()))
        res, _ = run_net(env=env)
        self.assertEqual(res["verdict"], "PASS")
        self.assertGreaterEqual(len(env.worlds), 1)
        self.assertEqual(env.net_active, set(), "the relay is released after every pass")

    def test_an_environment_that_cannot_join_through_a_relay_blocks_the_network_rows(self):
        inner = labfake.FakeEnv(LY.Layout(LY.sample_layout()))

        class NoNet:                  # the fake world without the two network methods
            def __getattr__(self, name):
                if name in ("net_begin", "net_end"):
                    raise AttributeError(name)
                return getattr(inner, name)
        res, _ = run_net(env=NoNet(), nets=["good_wifi"], rooms=["fight"])
        r = prow(res, "fight", "good_wifi")
        self.assertEqual(r["verdict"], "BLOCKED")
        self.assertIn("cannot join through the chaos relay", r["reason"])
        self.assertEqual(res["clean_verdict"], "PASS")

    def test_a_refused_profile_blocks_the_rows_with_the_reason(self):
        class Refuses(labfake.FakeEnv):
            def net_begin(self, pair, profile, seed=7):
                return "the client could not join through the relay"
        res, _ = run_net(env=Refuses(LY.Layout(LY.sample_layout())), nets=["bad_wifi"], rooms=["quest"])
        self.assertTrue(all(r["verdict"] == "BLOCKED" and "could not join" in r["reason"] for r in res["net_rooms"]))

    def test_the_total_budget_stops_a_pass_that_has_no_time_left(self):
        res, _ = run_net(total_s=25.0, rooms=["spawn"])
        self.assertTrue(res["net_rooms"])
        r = res["net_rooms"][0]
        self.assertEqual((r["verdict"], r["class"]), ("TIMEOUT", "ENV"))
        self.assertIn("total budget", r["reason"])

    def test_a_pair_that_is_dead_after_the_clean_pass_blocks_the_network_rows(self):
        # a pair that never got into the lab stays blocked (a copy that merely exited is restarted by every profile pass, see the crash test below)
        res, _ = run_net(bugs=["client_stays_out"], nets=["good_wifi"], rooms=["fight"])
        self.assertTrue(all(r["verdict"] == "BLOCKED" and "clean pass" in r["reason"] for r in res["net_rooms"]), res["net_rooms"])

    def test_a_pair_that_cannot_be_restarted_blocks_the_network_rows_with_the_reason(self):
        class NoRestart(labfake.FakeEnv):
            def reset(self, pair):
                return ("ENV", "copy 1 did not come up again")
        res, _ = run_net(env=NoRestart(LY.Layout(LY.sample_layout())), nets=["good_wifi"], rooms=["quest"])
        self.assertTrue(res["net_rooms"])
        self.assertTrue(all(r["verdict"] == "BLOCKED" and "did not come up again" in r["reason"] and r["profile"] == "good_wifi" for r in res["net_rooms"]))

    def test_every_row_of_a_network_pass_knows_its_profile_even_when_the_pair_dies_in_it(self):
        class Dies(labfake.FakeEnv):
            def net_begin(self, pair, profile, seed=7):
                self.worlds[pair].crash("host")
        res, _ = run_net(env=Dies(LY.Layout(LY.sample_layout())), nets=["good_wifi", "bad_wifi"], rooms=["fight", "quest"], pairs=1)
        self.assertEqual(len(res["net_rooms"]), 6)
        self.assertTrue(all(r.get("profile") in NETS for r in res["net_rooms"]), res["net_rooms"])
        self.assertEqual(res["rooms"][0]["verdict"], "PASS")
        self.assertIn("CRASH", {r.get("class") for r in res["net_rooms"]})

    def test_a_crash_in_the_clean_pass_does_not_blank_the_profile_passes_of_its_pair(self):
        # lab run 2026-10-27: one intermittent VATS crash made every cell of the pair's network table BLK; each profile pass restarts the pair anyway
        class CrashOnce(labfake.FakeEnv):
            def prepare(self, pair):
                r = super().prepare(pair)
                import heapq
                w = self.worlds[pair]
                heapq.heappush(w.q, (40.0, 0, "", lambda: w.crash("client")))
                return r
        res, _ = run_net(env=CrashOnce(LY.Layout(LY.sample_layout())), nets=["good_wifi"], rooms=["walk", "quest"], pairs=1)
        self.assertEqual(res["rooms"][1]["class"], "CRASH")
        self.assertEqual([r["verdict"] for r in res["net_rooms"]], ["PASS", "PASS", "PASS"], res["net_rooms"])      # spawn, walk, quest behind good_wifi

    def test_results_json_has_the_network_fields(self):
        res, _ = run_net(nets=["good_wifi"])
        for k in ("nets", "net_seed", "net_rooms", "net_info", "clean_verdict"):
            self.assertIn(k, res)
        self.assertEqual(res["net_info"][0]["profile"], "good_wifi")
        json.dumps(res, default=str)


class Report(unittest.TestCase):
    def test_the_summary_has_the_room_by_profile_table_with_verdicts_and_milliseconds(self):
        res, _ = run_net(pairs=3)
        txt = R.render_summary(res)
        self.assertIn("## Network: room x profile", txt)
        self.assertIn("| room | clean | good_wifi | bad_wifi |", txt)
        line = next(x for x in txt.splitlines() if x.startswith("| fight | PASS kill"))
        self.assertEqual(line.count("PASS"), 3, line)
        self.assertRegex(line, r"kill \d+/\d+ms.*kill \d+/\d+ms.*kill \d+/\d+ms")
        self.assertIn("relay good_wifi (3 pair(s))", txt)
        self.assertLess(len(txt.encode("utf-8")), 5000)

    def test_a_failed_network_room_names_its_profile_and_the_summary_stays_under_5kb(self):
        res, _ = run_net(bugs=["net_slow"], pairs=3)
        txt = R.render_summary(res)
        self.assertIn("### FAIL fight [good_wifi]", txt)
        self.assertRegex(txt, r"\| fight \| PASS [^|]*\| FAIL")
        self.assertLess(len(txt.encode("utf-8")), 5000)
        self.assertIn("network", txt.splitlines()[0])

    def test_a_cell_shows_a_dash_for_a_number_the_room_did_not_reach(self):
        self.assertEqual(R.net_cell({"room": "fight", "verdict": "FAIL", "metrics": {"host_kill_ms": 120}}), "FAIL kill 120/-ms")
        self.assertEqual(R.net_cell({"room": "fight", "verdict": "FAIL", "metrics": {}}), "FAIL")
        self.assertEqual(R.net_cell({"room": "walk", "verdict": "PASS", "metrics": {"host_err_p95": 80.5, "client_err_p95": 110.2, "host_lag_ms": 100, "client_lag_ms": 150}}),
                         "PASS e95 80/110u lag 100/150ms")
        self.assertEqual(R.net_cell(None), "n/a")
        self.assertEqual(R.net_cell({"room": "quest", "verdict": "BLOCKED", "metrics": {}}), "BLK")

    def test_no_table_without_a_network_pass(self):
        res, _ = run_net(nets=[])
        self.assertNotIn("## Network", R.render_summary(res))

    def test_the_summary_can_be_rebuilt_from_results_json(self):
        res, _ = run_net(nets=["bad_wifi"])
        with tempfile.TemporaryDirectory() as t:
            R.write_run(Path(t) / "20990101-000000", res, Path(t))
            self.assertIn("## Network", R.reprint(Path(t) / "20990101-000000"))


class FakeChaos:
    """What the live side needs of a ChaosPair: setup, teardown, a relay with counters and a profile switch."""

    def __init__(self, seen=(5, 5), fail=None):
        self.relay = SimpleNamespace(get_stats=lambda: {"c2s": {"seen": seen[0], "forwarded": seen[0], "dropped": 0}, "s2c": {"seen": seen[1], "forwarded": seen[1], "dropped": 0}},
                                     set_profile=lambda p, s=None: self.profiles.append((p, s)))
        self.profiles, self.fail, self.torn_down, self.set_up = [], fail, False, False

    def setup(self):
        self.set_up = True
        if self.fail:
            raise self.fail

    def teardown(self):
        self.torn_down = True


class LiveNet(unittest.TestCase):
    def env(self, chaos):
        import lab_live
        env = object.__new__(lab_live.LiveEnv)
        env.net_prof, env.chaos, env.log = {}, {}, lambda s: None
        env.chaos_factory = lambda pair, seed: chaos
        return env

    def test_the_client_is_attached_with_the_profile_after_the_relay_saw_traffic(self):
        c = FakeChaos()
        env = self.env(c)
        self.assertIsNone(env.net_begin(1, "bad_wifi", 9))
        env._net_attach(1)
        self.assertTrue(c.set_up)
        self.assertEqual(c.profiles, [("bad_wifi", 9)])
        info = env.net_end(1)
        self.assertTrue(c.torn_down, "the client is put back on the host's real port")
        self.assertEqual(info["relay"]["c2s"]["seen"], 5)
        self.assertEqual((env.net_prof, env.chaos), ({}, {}))
        self.assertIsNone(env.net_end(1))

    def test_a_client_that_never_crossed_the_relay_blocks_the_pass(self):          # twin net_bypass_unchecked
        c = FakeChaos(seen=(0, 0))
        env = self.env(c)
        env.net_begin(0, "good_wifi", 7)
        with self.assertRaises(K.Blocked) as e:
            env._net_attach(0)
        self.assertEqual(e.exception.cls, "ENV")
        self.assertIn("nothing crossed the relay", e.exception.reason)
        self.assertTrue(c.torn_down)
        self.assertEqual(c.profiles, [])
        self.assertEqual(env.chaos, {})

    def test_a_join_that_fails_blocks_the_pass_with_the_reason_and_cleans_up(self):
        import chaos_pair
        c = FakeChaos(fail=chaos_pair.SetupError("FAIL chaos setup: the client did not join through the relay", 1))
        env = self.env(c)
        env.net_begin(0, "good_wifi", 7)
        with self.assertRaises(K.Blocked) as e:
            env._net_attach(0)
        self.assertIn("did not join through the relay", e.exception.reason)
        self.assertTrue(c.torn_down)

    def test_a_live_pair_is_restarted_for_the_next_pass_not_reloaded(self):
        import lab_live
        env = self.env(FakeChaos())
        events = []
        closed = []
        env.nvs = {2: SimpleNamespace(close=lambda: closed.append(2)), 3: SimpleNamespace(close=lambda: closed.append(3)), 0: SimpleNamespace(close=lambda: closed.append(0))}
        env.held, env.entered = {2, 3, 0}, {1: True}
        env.pv = SimpleNamespace(unhold=lambda ins: events.append(("unhold", list(ins))), finish=lambda stop, ins=None: events.append(("finish", stop, list(ins))), up=lambda i: False)
        env.prepare = lambda pair: events.append(("prepare", pair)) or None
        self.assertIsNone(env.reset(1))
        self.assertEqual(events, [("unhold", [2, 3]), ("finish", True, [2, 3]), ("prepare", 1)], "only this pair's copies are stopped, then started again")
        self.assertEqual(sorted(closed), [2, 3])
        self.assertEqual(env.held, {0})
        self.assertNotIn(1, env.entered)
        env.prepare = lambda pair: ("ENV", "x")
        self.assertEqual(env.reset(1), ("ENV", "x"))

    def test_a_puppet_that_is_not_placed_yet_is_no_puppet_not_an_error(self):
        import lab_live

        class Nv:
            def __init__(self, ref, raises=None):
                self.ref, self.raises = ref, raises

            def call(self, method, params=None, timeout_s=5):
                return {"puppets": [{"ref": self.ref}]}

            def eval(self, expr, ref=None):
                if self.raises:
                    raise self.raises
                return 1.0
        mk = lambda nv: lab_live.LiveMachine.puppet_pos(SimpleNamespace(nv=nv))   # noqa: E731
        self.assertIsNone(mk(Nv("0x00000000")))
        self.assertIsNone(mk(Nv("0x0")))
        self.assertIsNone(mk(Nv("0x0012AB", RuntimeError("exec: ref '0012AB' is not a loaded form"))))
        self.assertEqual(mk(Nv("0x0012AB")), (1.0, 1.0, 1.0))
        ConnectionLost = type("ConnectionLost", (Exception,), {})
        with self.assertRaises(ConnectionLost):
            mk(Nv("0x0012AB", ConnectionLost("gone")))

    def test_finish_puts_every_client_back(self):
        a, b = FakeChaos(), FakeChaos()
        env = self.env(a)
        env.chaos = {0: a, 1: b}
        env.nvs, env.pv, env.held, env.args = {}, SimpleNamespace(unhold=lambda x: None, finish=lambda *x: None), set(), SimpleNamespace(keep=True)
        env.finish()
        self.assertTrue(a.torn_down and b.torn_down)

    def test_the_enter_hook_attaches_after_the_lab_is_entered(self):
        import lab_live
        order = []
        env = self.env(FakeChaos())
        env.nvs = {0: object(), 1: object()}
        env.entered = {}
        env.args = SimpleNamespace(save="s")
        env._coc = lambda nv: order.append("coc")
        env._net_attach = lambda pair: order.append("attach")
        env.net_prof[0] = ("bad_wifi", 7)

        class R:
            def __init__(self, *a, **k): pass
            def _load_start_save(self, *a): order.append("load")
            def _clear_load_messages(self): pass
            def _wait_coop_synced(self, **k): order.append("sync")
        mod = SimpleNamespace(Runner=R)
        lab_live._scenario_runner = lambda: mod
        env.enter(0)()
        self.assertEqual(order, ["load", "sync", "coc", "coc", "attach"])
        order.clear()
        env.net_prof.clear()
        env.enter(0)()
        self.assertNotIn("attach", order, "the clean pass never attaches a relay")

    def test_the_real_chaos_pair_joins_through_a_real_relay_and_the_traffic_is_seen(self):
        host = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        host.bind(("127.0.0.1", 0))
        host.settimeout(0.2)
        hport = host.getsockname()[1]
        stop = threading.Event()

        def echo():
            while not stop.is_set():
                try:
                    d, a = host.recvfrom(2048)
                    host.sendto(d, a)
                except OSError:
                    pass
        th = threading.Thread(target=echo, daemon=True)
        th.start()
        calls = []

        class Nv:
            joined = True

            def call(self, method, params=None, timeout_s=35.0):
                calls.append((method, dict(params or {})))
                if method == "coop.leave":
                    self.joined = False
                elif method == "coop.join":
                    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
                    s.settimeout(2.0)
                    s.sendto(b"hello", ("127.0.0.1", params["port"]))
                    try:
                        s.recvfrom(100)
                    except OSError:
                        pass
                    s.close()
                    self.joined = True
                elif method == "coop.status":
                    return {"role": "client", "running": True, "client": {"joined": self.joined}}
                return {"ok": True}
        with tempfile.TemporaryDirectory() as t:
            ini = Path(t) / "Data" / "NVSE" / "Plugins" / "nvcoop.ini"
            ini.parent.mkdir(parents=True)
            ini.write_text(f"[coop]\nrole=host\nport={host.getsockname()[1]}\n", encoding="utf-8")
            import lab_live as LL
            env = object.__new__(LL.LiveEnv)
            env.pv = SimpleNamespace(idir=lambda i: Path(t))
            env.nvs = {1: Nv()}
            env.net_prof, env.chaos, env.log = {}, {}, lambda s: None
            env.net_begin(0, "good_wifi", 3)
            env._net_attach(0)
            try:
                relay = env.chaos[0].relay
                self.assertEqual(relay.profile_name, "good_wifi")
                self.assertTrue(N.through_relay(relay.get_stats())[0])
                self.assertIn(("coop.join", {"host": "127.0.0.1", "port": relay.port}), calls)
            finally:
                info = env.net_end(0)
                stop.set()
                host.close()
            self.assertEqual(calls[-1], ("coop.join", {"host": "127.0.0.1", "port": hport}), "after the pass the client joins the host's own port again")
            self.assertGreater(info["relay"]["c2s"]["seen"], 0)


class Cli(unittest.TestCase):
    def cli(self, *args):
        cp = subprocess.run([sys.executable, "-I", str(LAB / "lab.py")] + list(args), capture_output=True, text=True, cwd=ROOT, timeout=300)
        return cp.returncode, cp.stdout, cp.stderr

    def test_run_fake_with_net_writes_the_table_and_exits_0(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("run", "--fake", "--pairs", "2", "--net", "good_wifi,bad_wifi", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 0, out[-1500:])
            d = Path(t) / "20990101-000000"
            self.assertIn("## Network: room x profile", (d / "summary.md").read_text(encoding="utf-8"))
            self.assertLess((d / "summary.md").stat().st_size, 5000)
            res = json.loads((d / "results.json").read_text(encoding="utf-8"))
            self.assertEqual(res["nets"], ["good_wifi", "bad_wifi"])

    def test_a_net_only_bug_gives_exit_1(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("run", "--fake", "--bug", "net_slow", "--rooms", "fight", "--net", "good_wifi", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 1, out[-800:])
            self.assertIn("### FAIL fight [good_wifi]", out)

    def test_an_unknown_profile_is_a_bad_argument(self):
        self.assertEqual(self.cli("run", "--fake", "--net", "dialup")[0], 2)
        self.assertEqual(self.cli("run", "--fake", "--net", "good_wifi", "--param", "fight.latency_s@nope=1")[0], 2)

    def test_smoke_ignores_net(self):
        with tempfile.TemporaryDirectory() as t:
            rc, out, _ = self.cli("smoke", "--fake", "--stamp", "20990101-000000", "--out-root", t)
            self.assertEqual(rc, 0, out)
            self.assertNotIn("## Network", out)


class Files(unittest.TestCase):
    def test_remote_script_takes_a_net_parameter(self):
        txt = (ROOT / "tools" / "bench" / "remote" / "lab_run.ps1").read_text(encoding="utf-8")
        self.assertIn("$Net", txt)
        self.assertIn("--net", txt)

    def test_no_em_dash_or_personal_path_in_the_new_sources(self):
        for p in (LAB / "lab_net.py", LAB / "tests" / "test_net.py"):
            src = p.read_text(encoding="utf-8")
            self.assertNotIn(chr(0x2014), src, p.name)
            self.assertNotIn("mth" + "wj", src.lower(), p.name)


if __name__ == "__main__":
    unittest.main(verbosity=1)
