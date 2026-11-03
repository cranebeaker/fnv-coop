"""End to end: the real runner, relay and scorer against fake_viewer.py (no game needed)."""
import json
import os
import sys
import tempfile
import unittest

import runner
import scenario as sc
import verdict

HERE = os.path.dirname(os.path.abspath(__file__))


def cfg(**kw):
    c = {"viewer": [sys.executable, "-I", os.path.join(HERE, "fake_viewer.py")], "data": "FakeData",
         "out_dir": "unused", "base_port": 29000, "launch_timeout_s": 15, "grace_s": 4,
         "python": sys.executable, "background": True, "screen_size": "640,360"}
    c.update(kw)
    return c


def scn(**kw):
    s = {"name": "e2e", "cell": "GSDocMitchellHouse", "network": "perfect", "seed": 5, "duration_s": 2,
         "settle_s": 1, "timeout_s": 60,
         "host": {"script": [{"at": 0.3, "stage": "VCG01 10"}]},
         "guest": {"script": [{"at": 0.3, "run": "player.additem Caps001 25"}]},
         "expect": [{"id": "stage", "path": "quests.VCG01.stage", "op": "eq", "value": 10, "on": "both"},
                    {"id": "caps", "path": "items.guest.Caps001", "op": "eq", "value": 25, "on": "both"},
                    {"id": "same", "path": "hash.shared", "on": "equal"}]}
    s.update(kw)
    return sc.validate(s)


class E2E(unittest.TestCase):
    slot = 0

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        type(self).slot += 1

    def go(self, s, **cfg_kw):
        d = os.path.join(self.tmp.name, "run")
        v = runner.run_scenario(s, cfg(**cfg_kw), d, scn_path="x.json", slot=self.slot, log=lambda *_: None)
        self.assertTrue(os.path.exists(os.path.join(d, "verdict.json")))
        with open(os.path.join(d, "verdict.json"), encoding="utf-8") as f:
            self.assertEqual(json.load(f)["class"], v["class"])
        return v

    def with_fake(self, mode, role="host", **kw):
        s = scn(**kw)
        s[role]["extra_args"] = ["--fake", mode]
        return s

    # ---- teleport, keys and twins (fake viewer) ----
    def moving(self, **kw):
        s = scn(host={"script": [{"at": 0.3, "run": "coop teleport 2552 1603.6 7360"}, {"at": 0.8, "key": "w:1"}]},
                guest={"script": [{"at": 0.3, "run": "coop teleport 2152 1803.6 7360"}]},
                expect=[{"id": "h", "path": "positions.host", "op": "dist_from", "from": [2352, 1603.6],
                         "cmp": "ge", "value": 250, "on": "both"},
                        {"id": "g", "path": "positions.guest", "op": "dist_from", "from": [2352, 1603.6],
                         "cmp": "ge", "value": 150, "on": "both"},
                        {"id": "he", "path": "positions.host", "on": "equal", "tol": 40, "dims": 2}],
                duration_s=3, settle_s=2, **kw)
        return s

    def test_teleport_and_key_move_avatars_on_both_peers(self):
        v = self.go(self.moving())
        self.assertEqual((v["result"], v["class"]), ("pass", "pass"), v["reason"])

    def test_setpos_moves_nobody(self):
        s = self.moving()
        s["host"]["script"] = [{"at": 0.3, "run": "player.setpos x 100"}]
        v = self.go(s)
        self.assertEqual((v["result"], v["class"]), ("fail", "assert"), v["reason"])

    def test_twin_passes_only_when_assertions_fail(self):
        s = self.moving(expect_fail=True)
        s["host"]["script"] = []
        s["guest"]["script"] = []
        v = self.go(s)
        self.assertEqual((v["result"], v["class"]), ("pass", "twin"), v["reason"])
        self.assertIn("RESULT: PASS twin", runner.line(v))

    def test_twin_that_passes_is_a_failure(self):
        v = self.go(self.moving(expect_fail=True))
        self.assertEqual((v["result"], v["class"]), ("fail", "twin"), v["reason"])

    def test_twin_does_not_hide_a_crash(self):
        s = self.with_fake("crash", expect_fail=True)
        v = self.go(s)
        self.assertEqual(v["result"], "fail")
        self.assertNotEqual(v["class"], "twin")

    # ---- passes ----
    def test_pass_perfect(self):
        v = self.go(scn())
        self.assertEqual((v["result"], v["class"]), ("pass", "pass"), v["reason"])
        self.assertNotIn("packet", v)
        self.assertGreater(v["relay"]["g2h"]["in"], 3)
        self.assertGreater(v["relay"]["h2g"]["in"], 3)
        self.assertTrue(all(c["ok"] for c in v["checks"]))
        self.assertIn("rerun", v)
        self.assertEqual(v["peers"]["host"]["exit_code"], 0)

    def test_pass_under_chaos(self):
        s = scn(network="flaky", seed=3, duration_s=4, settle_s=4, max_repairs=10,
                outages=[{"at": 1, "seconds": 1, "dir": "both"}])
        v = self.go(s)
        self.assertEqual(v["result"], "pass", v["reason"])
        self.assertGreater(v["relay"]["g2h"]["dropped"] + v["relay"]["h2g"]["dropped"], 0)

    # ---- mutation twins: each broken viewer must be caught, with the right class ----
    def test_twin_no_sync_is_desync_or_assert(self):
        v = self.go(self.with_fake("no_sync", "guest"))
        self.assertEqual(v["result"], "fail")
        self.assertIn(v["class"], ("assert", "desync"))
        self.assertIn("packet", v)

    def test_twin_double_apply_is_assert(self):
        v = self.go(self.with_fake("double_apply", "host"))
        self.assertEqual(v["class"], "assert", v["reason"])
        self.assertIn("caps", v["reason"])

    def test_twin_crash(self):
        v = self.go(self.with_fake("crash", "guest", duration_s=6))
        self.assertEqual(v["class"], "crash", v["reason"])

    def test_twin_hang_is_timeout(self):
        v = self.go(self.with_fake("hang", "host"), grace_s=2)
        self.assertEqual(v["class"], "timeout", v["reason"])

    def test_twin_silent_is_hollow(self):
        v = self.go(self.with_fake("silent", "host"), launch_timeout_s=3)
        self.assertEqual(v["class"], "hollow", v["reason"])

    def test_twin_no_connect_is_hollow(self):
        v = self.go(self.with_fake("no_connect", "guest"), launch_timeout_s=3)
        self.assertEqual(v["class"], "hollow", v["reason"])

    def test_twin_error_event(self):
        v = self.go(self.with_fake("error_event", "host"))
        self.assertEqual(v["class"], "log", v["reason"])

    def test_twin_repair_on_perfect_link(self):
        v = self.go(self.with_fake("repair", "host"))
        self.assertEqual(v["class"], "repair", v["reason"])

    def test_twin_bad_exit_code(self):
        v = self.go(self.with_fake("bad_exit", "guest"))
        self.assertEqual(v["class"], "crash", v["reason"])

    def test_launch_error(self):
        v = self.go(scn(), viewer=[os.path.join(self.tmp.name, "no-such-viewer.exe")])
        self.assertEqual(v["class"], "launch", v["reason"])

    def test_bad_flags_exit_two(self):
        s = scn()
        s["host"]["extra_args"] = ["--no-such-flag"]
        v = self.go(s)
        self.assertEqual(v["result"], "fail")
        self.assertIn(v["class"], ("crash", "hollow"))

    def test_failure_packet_under_3kb(self):
        v = self.go(self.with_fake("error_event", "host"))
        self.assertLess(len(json.dumps(v["packet"])), 3072)

    # ---- N clean runs ----
    def test_clean_runs_three(self):
        out = os.path.join(self.tmp.name, "clean")
        summary, last = runner.clean_runs(scn(duration_s=1, settle_s=1), cfg(base_port=29200), out, 3,
                                          log=lambda *_: None)
        self.assertTrue(summary["clean"])
        self.assertEqual(summary["passes"], 3)
        self.assertEqual(len(summary["runs"]), 3)

    def test_clean_stops_at_first_failure(self):
        out = os.path.join(self.tmp.name, "clean")
        summary, last = runner.clean_runs(self.with_fake("double_apply", "host", duration_s=1, settle_s=1),
                                          cfg(base_port=29300), out, 3, log=lambda *_: None)
        self.assertFalse(summary["clean"])
        self.assertEqual(len(summary["runs"]), 1)
        self.assertIn("same seed", summary["note"])


class RelayStart(unittest.TestCase):
    """A stalled relay start is stopped and started again (runner start_relay)."""
    slot = 40

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        type(self).slot += 1
        saved = (runner.Proc, runner.RELAY_START_S, runner.RELAY_TRIES)
        self.real_proc = saved[0]
        self.addCleanup(self.restore, saved)
        runner.RELAY_START_S = 2

    def restore(self, saved):
        runner.Proc, runner.RELAY_START_S, runner.RELAY_TRIES = saved

    def stall_relays(self, n):
        """The next n relay starts sleep and write no ready file."""
        left = [n]

        class StallProc(self.real_proc):
            def __init__(self, role, cmd, stdout_path, cwd=None):
                if role == "relay" and left[0] > 0:
                    left[0] -= 1
                    cmd = [sys.executable, "-I", "-c", "import time; time.sleep(60)"]
                super().__init__(role, cmd, stdout_path, cwd)

        runner.Proc = StallProc

    def go(self):
        lines = []
        d = os.path.join(self.tmp.name, "run")
        v = runner.run_scenario(scn(), cfg(), d, scn_path="x.json", slot=self.slot, log=lines.append)
        return v, lines

    def test_a_stalled_relay_is_started_again(self):
        self.stall_relays(1)
        v, lines = self.go()
        self.assertEqual((v["result"], v["class"]), ("pass", "pass"), v["reason"])
        self.assertTrue(any("try 1 of 3" in x for x in lines), lines)

    def test_a_relay_that_never_starts_is_a_harness_failure(self):
        runner.RELAY_TRIES = 2
        self.stall_relays(2)
        v, lines = self.go()
        self.assertEqual(v["class"], "harness", v["reason"])
        self.assertIn("relay did not start", v["reason"])
        self.assertEqual(len(lines), 2, lines)


class Pieces(unittest.TestCase):
    def test_free_udp_port_skips_a_port_held_on_every_address(self):
        import socket
        held = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        held.bind(("0.0.0.0", 0))
        try:
            port = held.getsockname()[1]
            self.assertNotEqual(runner.free_udp_port(port), port)
        finally:
            held.close()

    def test_scorer_selftest_passes(self):
        self.assertEqual(verdict.scorer_selftest(), [])

    def test_packet_limit_with_huge_logs(self):
        s = scn(expect=[{"id": "c%d" % i, "path": "nope.%d" % i, "op": "eq", "value": "x" * 200, "on": "both"} for i in range(30)])
        big = {"role": "host", "tail": [{"ev": "x", "msg": "y" * 500}] * 6, "exit_code": 1, "ready": True, "stdout_tail": "z" * 500}
        v = {"class": "assert", "reason": "r" * 500, "network": "perfect", "seed": 1, "rerun": "cmd " * 100, "relay": None,
             "checks": [{"id": "c%d" % i, "ok": False, "detail": "d" * 300} for i in range(30)]}
        pk = verdict.build_packet(s, v, big, dict(big, role="guest"))
        self.assertLess(len(json.dumps(pk)), 3072)

    def test_missing_config_is_a_config_error(self):
        with self.assertRaises(runner.ConfigError):
            runner.load_config(os.path.join(HERE, "no-such-config.json"))

    def test_main_reports_config_error_exit_2(self):
        import contextlib
        import io
        with contextlib.redirect_stdout(io.StringIO()):
            rc = runner.main(["run", os.path.join(HERE, "scenarios", "doc_stage_sync.json"),
                              "--config", os.path.join(HERE, "no-such-config.json")])
        self.assertEqual(rc, 2)


if __name__ == "__main__":
    unittest.main()
