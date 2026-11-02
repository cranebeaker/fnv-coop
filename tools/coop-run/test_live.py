"""live.py (client, replay, watch, up/down) against fake_viewer.py's control port. No game needed."""
import json
import os
import socket
import subprocess
import sys
import tempfile
import time
import unittest

import live
import runner
import scenario as sc

HERE = os.path.dirname(os.path.abspath(__file__))
FAKE = os.path.join(HERE, "fake_viewer.py")


def free_port(kind=socket.SOCK_STREAM):
    s = socket.socket(socket.AF_INET, kind)
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


class Pair:
    """A fake host and guest, both with control ports, joined directly."""

    def __init__(self, tmp, host_mode="", guest_mode=""):
        self.udp = free_port(socket.SOCK_DGRAM)
        self.ports = {"host": free_port(), "guest": free_port()}
        self.procs = []
        for role, mode in (("host", host_mode), ("guest", guest_mode)):
            a = [sys.executable, "-I", FAKE, "FakeData", "GSDocMitchellHouse", "--no-hud", "--coop-test-token",
                 "--coop-host" if role == "host" else "--coop-join",
                 str(self.udp) if role == "host" else "127.0.0.1:%d" % self.udp,
                 "--coop-log", os.path.join(tmp, role + ".jsonl"), "--coop-state-dump", os.path.join(tmp, role + ".state.json"),
                 "--coop-quit-file", os.path.join(tmp, role + ".quit"), "--coop-quit-after", "120",
                 "--coop-control", str(self.ports[role])]
            if mode:
                a += ["--fake", mode]
            self.procs.append(subprocess.Popen(a, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
        self.host = live.Peer("host", self.ports["host"], timeout=10)
        self.guest = live.Peer("guest", self.ports["guest"], timeout=10)

    def wait(self):
        self.host.wait_ready(20)
        self.guest.wait_ready(20)

    def stop(self):
        for p in (self.host, self.guest):
            if p.up():
                p.quit()
        for pr in self.procs:
            try:
                pr.wait(8)
            except subprocess.TimeoutExpired:
                pr.kill()
                pr.wait(5)


class Client(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        cls.pair = Pair(cls.tmp.name)
        cls.pair.wait()

    @classmethod
    def tearDownClass(cls):
        cls.pair.stop()
        cls.tmp.cleanup()

    def test_state_has_the_dump_and_live_figures(self):
        r = self.pair.host.state()
        self.assertTrue(r["ok"] and r["ready"])
        self.assertEqual((r["cmd"], r["role"]), ("state", "host"))
        self.assertEqual(r["puppets"], 1)
        self.assertEqual(r["puppets_visible"], 1)
        self.assertEqual(r["npcs_visible"], 1)
        self.assertIn("hash", r["state"])
        s = live.split_state(r)
        self.assertEqual(sc.get_path(s, "live.puppets_visible"), 1)
        self.assertEqual(self.pair.guest.state()["role"], "guest")

    def test_entities_and_visible_filter(self):
        r = self.pair.guest.entities()
        self.assertEqual(r["count"], 2)
        kinds = {e["kind"] for e in r["entities"]}
        self.assertEqual(kinds, {"npc", "puppet"})
        self.assertEqual(len(self.pair.guest.entities(visible=False)["entities"]), 0)

    def test_run_host_and_guest_then_state_shows_it(self):
        self.assertEqual(self.pair.host.run("SetStage LiveQ 30")["how"], "host")
        self.assertEqual(self.pair.guest.run("player.additem Caps001 7")["how"], "sent")
        end = time.time() + 5
        while time.time() < end:
            st = self.pair.guest.state()["state"]
            if st["quests"].get("LiveQ", {}).get("stage") == 30 and st["items"]["guest"].get("Caps001") == 7:
                break
            time.sleep(0.1)
        self.assertEqual(st["quests"]["LiveQ"]["stage"], 30)
        self.assertEqual(st["items"]["guest"]["Caps001"], 7)

    def test_bot_local_and_keys(self):
        r = self.pair.guest.bot("teleport 10 20 30")
        self.assertEqual((r["ok"], r["how"]), (True, "bot"))
        self.assertEqual(self.pair.guest.state()["state"]["positions"]["guest"][:3], [10.0, 20.0, 30.0])
        self.assertEqual(self.pair.guest.run("local coop teleport 5 6 7", local=True)["how"], "local")
        before = self.pair.host.state()["state"]["positions"]["host"][1]
        self.assertEqual(self.pair.host.keys("w", 1.0)["key"], "w")
        end = time.time() + 5
        while time.time() < end and self.pair.host.state()["state"]["positions"]["host"][1] <= before:
            time.sleep(0.1)
        self.assertGreater(self.pair.host.state()["state"]["positions"]["host"][1], before)

    def test_screenshot_writes_the_file_before_replying(self):
        p = os.path.join(self.tmp.name, "sub", "shot.png")
        r = self.pair.host.shot(p)
        self.assertTrue(r["ok"])
        self.assertTrue(os.path.exists(p))

    def test_errors_are_replies_not_disconnects(self):
        with self.assertRaises(live.LiveError) as cm:
            self.pair.host.call(cmd="nope")
        self.assertIn("nope", str(cm.exception))
        r = self.pair.host.call(cmd="run", check=False)
        self.assertFalse(r["ok"])
        self.assertTrue(self.pair.host.state()["ok"])  # still serving

    def test_connection_refused_is_a_live_error(self):
        with self.assertRaises(live.LiveError):
            live.Peer("x", free_port(), timeout=1).state()

    def test_garbage_line_gets_an_error_reply(self):
        with socket.create_connection(("127.0.0.1", self.pair.ports["host"]), timeout=5) as s:
            s.sendall(b"this is not json\n")
            reply = json.loads(s.makefile().readline())
        self.assertFalse(reply["ok"])


def scenario(**kw):
    s = {"name": "live_t", "cell": "GSDocMitchellHouse", "duration_s": 2, "settle_s": 2,
         "host": {"script": [{"at": 0.1, "stage": "LiveR 10"}]},
         "guest": {"script": [{"at": 0.1, "run": "player.additem Caps001 25"}, {"at": 0.2, "run": "coop teleport 1 2 3"}]},
         "expect": [{"id": "stage", "path": "quests.LiveR.stage", "op": "eq", "value": 10, "on": "both"},
                    {"id": "caps", "path": "items.guest.Caps001", "op": "ge", "value": 25, "on": "both"},
                    {"id": "pup", "path": "live.puppets_visible", "op": "ge", "value": 1, "on": "both"},
                    {"id": "same", "path": "hash.shared", "on": "equal"}]}
    s.update(kw)
    return sc.validate(s)


class Replay(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        cls.pair = Pair(cls.tmp.name)
        cls.pair.wait()

    @classmethod
    def tearDownClass(cls):
        cls.pair.stop()
        cls.tmp.cleanup()

    def test_replay_passes_per_assertion(self):
        lines = []
        res = live.replay(scenario(), self.pair.host, self.pair.guest, log=lambda *_: None)
        live.report(scenario(), res, out=lines.append)
        self.assertTrue(res["passed"], lines)
        self.assertEqual(len(res["results"]), 7)  # stage, caps and pup on both peers, plus same
        self.assertTrue(lines[-1].startswith("RESULT: PASS live_t"))
        self.assertTrue(any(l.startswith("PASS  stage@host") for l in lines), lines)

    def test_replay_fails_with_the_failing_assertion_named(self):
        s = scenario(settle_s=1)
        s["expect"].append({"id": "never", "path": "quests.Nope.stage", "op": "eq", "value": 1, "on": "host"})
        lines = []
        res = live.replay(s, self.pair.host, self.pair.guest, log=lambda *_: None)
        live.report(s, res, out=lines.append)
        self.assertFalse(res["passed"])
        self.assertTrue(any(l.startswith("FAIL  never") for l in lines), lines)
        self.assertTrue(lines[-1].startswith("RESULT: FAIL"))

    def test_replay_reports_a_dead_viewer(self):
        res = live.replay(scenario(), self.pair.host, live.Peer("gone", free_port(), timeout=1), log=lambda *_: None)
        self.assertFalse(res["passed"])
        self.assertTrue(res["errors"])

    def test_skips_startup_flags(self):
        s = scenario(host={"script": [{"talk": True}]}, guest={"script": []}, expect=[])
        seen = []
        res = live.replay(s, self.pair.host, self.pair.guest, log=seen.append)
        self.assertTrue(any("skip host.talk" in x for x in seen), seen)
        self.assertTrue(res["passed"])

    def test_runner_attach(self):
        d = os.path.join(self.tmp.name, "scn")
        os.makedirs(d, exist_ok=True)
        p = os.path.join(d, "a.json")
        with open(p, "w", encoding="utf-8") as f:
            json.dump(scenario(), f)
        rc = runner.main(["run", p, "--attach", "host:%d" % self.pair.ports["host"], "guest:%d" % self.pair.ports["guest"]])
        self.assertEqual(rc, 0)
        rc = runner.main(["run", p, "--attach", "host:%d" % free_port(), "guest:%d" % self.pair.ports["guest"]])
        self.assertEqual(rc, 1)

    def test_watch_replays_after_each_save(self):
        d = os.path.join(self.tmp.name, "w")
        os.makedirs(d, exist_ok=True)
        p = os.path.join(d, "w.json")
        s = scenario()
        with open(p, "w", encoding="utf-8") as f:
            json.dump(s, f)
        out = []
        r = live.watch(p, self.pair.host, self.pair.guest, once=True, out=out.append)
        self.assertTrue(r["passed"], out)
        # a second save with a broken assertion is replayed and fails; a broken file is reported
        runs = []

        def fake_sleep(sec):
            runs.append(sec)
            if len(runs) == 1:
                s["expect"][0]["value"] = 99
                s["settle_s"] = 0.5
                with open(p, "w", encoding="utf-8") as f:
                    json.dump(s, f)
                os.utime(p, ns=(time.time_ns() + 10 ** 9, time.time_ns() + 10 ** 9))
            elif len(runs) > 6:
                raise StopIteration

        out2 = []
        try:
            live.watch(p, self.pair.host, self.pair.guest, out=out2.append, sleep=lambda s_: (time.sleep(min(s_, 0.05)), fake_sleep(s_)))
        except StopIteration:
            pass
        self.assertTrue(any(l.startswith("FAIL  stage@host") for l in out2), out2)


class UpDown(unittest.TestCase):
    def test_up_state_down_with_the_cli(self):
        with tempfile.TemporaryDirectory() as tmp:
            cfgp = os.path.join(tmp, "cfg.json")
            with open(cfgp, "w", encoding="utf-8") as f:
                json.dump({"viewer": [sys.executable, "-I", FAKE], "data": "FakeData", "out_dir": tmp,
                           "base_port": free_port(socket.SOCK_DGRAM), "launch_timeout_s": 15}, f)
            sess = os.path.join(tmp, "live", "session.json")
            out = []
            rc = live.main(["--session", sess, "up", "--cell", "GSDocMitchellHouse", "--config", cfgp,
                            "--control-base", str(free_port()), "--timeout", "30"], out=out.append)
            try:
                self.assertEqual(rc, 0, out)
                self.assertTrue(out[-1].startswith("RESULT: PASS up"))
                s = live.load_session(sess)
                self.assertTrue(live.Peer("h", s["host"]["control"]).up())
                # a second `up` while running is refused
                again = []
                self.assertEqual(live.main(["--session", sess, "up", "--cell", "X", "--config", cfgp], out=again.append), 1)
                # the CLI by role name
                self.assertEqual(live.main(["--session", sess, "run", "host", "SetStage CliQ 5"]), 0)
                self.assertEqual(live.main(["--session", sess, "state", "host", "--path", "quests.CliQ.stage"]), 0)
                # reset: the guest quits and comes back with the same command
                old = s["guest"]["pid"]
                live.reset_guest(sess, out=out.append, timeout=30)
                self.assertNotEqual(live.load_session(sess)["guest"]["pid"], old)
                self.assertEqual(live.Peer("g", s["guest"]["control"]).state()["role"], "guest")
            finally:
                down = []
                self.assertEqual(live.main(["--session", sess, "down"], out=down.append), 0, down)
            self.assertFalse(live.Peer("h", s["host"]["control"]).up())
            self.assertFalse(live.Peer("g", s["guest"]["control"]).up())


if __name__ == "__main__":
    unittest.main()
