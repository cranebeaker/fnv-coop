"""Runs with more than one guest: scenario parsing, command lines, checks and the verdict.

The process launching of runner.run_scenario for several guests needs a stand-in viewer that
speaks to more than one peer (fake_viewer.py does not), so it is covered by the real 4-viewer
scenario on the test PC, not here.
"""
import json
import os
import tempfile
import unittest

import scenario as scn_mod
import verdict as verdict_mod

BASE = {"name": "m", "cell": "X", "network": "perfect", "duration_s": 5,
        "host": {"script": []},
        "guests": [{"script": [{"run": "coop ping"}]},
                   {"script": [{"run": "coop voice 1"}]},
                   {"script": [{"at": 2, "run": "coop ping 100"}]}],
        "expect": [{"id": "p", "path": "live.markers", "op": "ge", "value": 1, "on": "all"},
                   {"id": "q", "path": "live.pings", "op": "ge", "value": 1, "on": "guest3"},
                   {"id": "h", "path": "hash.shared", "on": "equal"}]}


def state(**live):
    return {"role": "x", "live": dict({"markers": 2, "pings": 1}, **live), "hash": {"shared": "a"}}


class Parse(unittest.TestCase):
    def test_guests_list_defines_the_peers(self):
        s = scn_mod.validate(json.loads(json.dumps(BASE)))
        self.assertEqual(len(s["guests"]), 3)
        self.assertIs(s["guest"], s["guests"][0])
        self.assertIs(s["guest2"], s["guests"][1])
        self.assertIs(s["guest3"], s["guests"][2])

    def test_without_guests_there_is_one(self):
        s = scn_mod.validate({"name": "m", "cell": "X"})
        self.assertEqual(len(s["guests"]), 1)
        self.assertNotIn("guest2", s)

    def test_bad_guest_lists_are_refused(self):
        for bad in ([], [{}] * 4, "x", [1]):
            with self.assertRaises(scn_mod.ScenarioError):
                scn_mod.validate({"name": "m", "cell": "X", "guests": bad})

    def test_each_guest_gets_its_own_script_and_name(self):
        s = scn_mod.validate(json.loads(json.dumps(BASE)))
        files = {"log": "l", "state": "s", "quit": "q"}
        a2 = scn_mod.viewer_args(s, "guest2", "D", {"join": "127.0.0.1:1"}, files, 30)
        self.assertIn("coop voice 1", a2)
        self.assertEqual(a2[a2.index("--coop-name") + 1], "Guest 2")
        a1 = scn_mod.viewer_args(s, "guest", "D", {"join": "127.0.0.1:1"}, files, 30)
        self.assertIn("coop ping", a1)
        self.assertNotIn("--coop-name", a1)
        a3 = scn_mod.viewer_args(s, "guest3", "D", {"join": "127.0.0.1:1"}, files, 30)
        self.assertEqual(a3[a3.index("--run-at") + 1], "2")


class Evaluate(unittest.TestCase):
    def setUp(self):
        self.s = scn_mod.validate(json.loads(json.dumps(BASE)))

    def test_all_checks_every_peer(self):
        res = scn_mod.evaluate(self.s["expect"][:1], state(), state(), {"guest2": state(), "guest3": state(markers=0)})
        ok = {c["id"]: c["ok"] for c in res}
        self.assertEqual(ok, {"p@host": True, "p@guest": True, "p@guest2": True, "p@guest3": False})

    def test_named_extra_guest(self):
        res = scn_mod.evaluate(self.s["expect"][1:2], state(), state(), {"guest2": state(), "guest3": state(pings=0)})
        self.assertEqual([(c["id"], c["ok"]) for c in res], [("q", False)])

    def test_equal_compares_the_host_with_every_guest(self):
        bad = state()
        bad["hash"]["shared"] = "b"
        res = scn_mod.evaluate(self.s["expect"][2:], state(), state(), {"guest2": bad, "guest3": state()})
        self.assertEqual({c["id"]: c["ok"] for c in res}, {"h#guest": True, "h#guest2": False, "h#guest3": True})

    def test_a_check_for_a_peer_that_is_not_there_fails(self):
        res = scn_mod.evaluate(self.s["expect"][1:2], state(), state())
        self.assertFalse(res[0]["ok"])


class Verdict(unittest.TestCase):
    def peer(self, d, name, ok=True, hash_="a"):
        lg = os.path.join(d, name + ".jsonl")
        with open(lg, "w", encoding="utf-8") as f:
            for e in [{"ev": "start"}, {"ev": "coop_ready"}, {"ev": "shutdown", "clean": True}]:
                f.write(json.dumps(e) + "\n")
        st = os.path.join(d, name + ".state.json")
        with open(st, "w", encoding="utf-8") as f:
            json.dump({"role": name if name in ("host", "guest") else "guest", "live": {"markers": 2, "pings": 1},
                       "hash": {"shared": hash_}}, f)
        return {"exit_code": 0, "log": lg, "state": st}

    def run_it(self, third_hash="a", twin=False):
        s = json.loads(json.dumps(BASE))
        s["expect"] = [{"id": "h", "path": "hash.shared", "on": "equal"}]
        s["expect_fail"] = twin
        s = scn_mod.validate(s)
        relay = {"armed": True, "profile": "perfect", "g2h": {"in": 5}, "h2g": {"in": 5}, "events": []}
        with tempfile.TemporaryDirectory() as d:
            run = {"host": self.peer(d, "host"), "guest": self.peer(d, "guest"), "guest2": self.peer(d, "guest2"),
                   "guest3": self.peer(d, "guest3", hash_=third_hash), "relay": relay}
            return verdict_mod.judge(s, run)

    def test_four_peers_agreeing_pass(self):
        v = self.run_it()
        self.assertEqual((v["result"], v["class"]), ("pass", "pass"), v["reason"])
        self.assertEqual(sorted(v["peers"]), ["guest", "guest2", "guest3", "host"])

    def test_a_desynced_fourth_peer_fails(self):
        v = self.run_it(third_hash="zzz")
        self.assertEqual(v["class"], "desync")
        self.assertIn("guest3", v["reason"] + json.dumps(v["checks"]))

    def test_the_twin_of_a_desync_passes_as_a_twin(self):
        v = self.run_it(third_hash="zzz", twin=True)
        self.assertEqual((v["result"], v["class"]), ("pass", "twin"))


if __name__ == "__main__":
    unittest.main()
