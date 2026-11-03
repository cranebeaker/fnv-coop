import json
import os
import unittest

import fake_viewer
import scenario as sc

HERE = os.path.dirname(os.path.abspath(__file__))


def base(**kw):
    s = {"name": "t", "cell": "GSDocMitchellHouse"}
    s.update(kw)
    return s


class Validate(unittest.TestCase):
    def test_shipped_scenarios_are_valid(self):
        d = os.path.join(HERE, "scenarios")
        names = [n for n in os.listdir(d) if n.endswith(".json")]
        self.assertGreaterEqual(len(names), 4)
        for n in names:
            s = sc.load(os.path.join(d, n))
            self.assertIn(s["network"], ("perfect", "good_wifi", "bad_wifi", "mobile", "congested", "flaky", "partition"))

    def test_defaults(self):
        s = sc.validate(base())
        self.assertEqual(s["network"], "perfect")
        self.assertEqual(s["host"]["script"], [])

    def test_errors_are_listed(self):
        with self.assertRaises(sc.ScenarioError) as cm:
            sc.validate({"name": "t", "network": "hell",
                         "host": {"script": [{"at": 1}, {"stage": "ONLYQUEST"}, {"talk": True, "at": 2}]},
                         "expect": [{"path": "a", "op": "bogus"}, {"id": "x", "path": "p", "op": "near", "value": 1}]})
        m = str(cm.exception)
        for frag in ("cell is required", "not a profile", "exactly one of", '"QUEST STAGE"', "cannot have a time",
                     "op 'bogus'", "needs tol"):
            self.assertIn(frag, m)

    def test_new_game_needs_no_cell(self):
        sc.validate({"name": "t", "new_game": True})

    def test_duplicate_expect_ids(self):
        with self.assertRaises(sc.ScenarioError):
            sc.validate(base(expect=[{"id": "a", "path": "p", "value": 1}, {"id": "a", "path": "p", "value": 1}]))


class KeyLines(unittest.TestCase):
    def bad(self, line):
        with self.assertRaises(sc.ScenarioError):
            sc.validate(base(host={"script": [line]}))

    def test_valid(self):
        sc.validate(base(host={"script": [{"at": 3, "key": "w:2"}, {"at": 0, "key": "mouse-left"}]}))

    def test_invalid(self):
        self.bad({"key": "w:2"})                 # needs a time
        self.bad({"at": 1, "key": "w:0"})        # hold must be > 0
        self.bad({"at": 1, "key": "w:x"})
        self.bad({"at": 1, "key": "w 2"})
        self.bad({"at": 1, "key": ""})
        self.bad({"at": 1, "key": "w", "run": "x"})

    def test_fake_viewer_walks(self):
        o = fake_viewer.parse(["D", "C", "--coop-host", "1", "--coop-log", "l", "--coop-state-dump", "s",
                               "--key-at", "3", "w:2"])
        self.assertEqual(o["key_at"], [(3.0, "w:2")])


class Args(unittest.TestCase):
    def test_script_args(self):
        got = sc.script_args([{"run": "player.additem X 1"}, {"at": 2.5, "run": "foo"}, {"stage": "VCG01 0"},
                              {"at": 3, "stage": "VCG01 10"}, {"talk": True}, {"choose": [1, 2]}, {"say": "hi"}, {"at": 3, "key": "w:2"}, {"at": 5, "key": "escape"}])
        self.assertEqual(got, ["--run", "player.additem X 1", "--run-at", "2.5", "foo", "--stage", "VCG01", "0",
                               "--run-at", "3", "SetStage VCG01 10", "--talk", "--choose", "1,2", "--say", "hi",
                               "--key-at", "3", "w:2", "--key-at", "5", "escape"])

    def test_viewer_args_roles(self):
        s = sc.validate(base(character="c.txt", host={"script": [{"stage": "Q 1"}]}))
        files = {"log": "l", "state": "s", "quit": "q"}
        h = sc.viewer_args(s, "host", "D", {"host": 27960}, files, 99)
        self.assertEqual(h[:2], ["D", "GSDocMitchellHouse"])
        self.assertIn("--coop-host", h)
        self.assertNotIn("--coop-join", h)
        self.assertIn("--coop-test-token", h)
        self.assertEqual(h[h.index("--character") + 1], "c.txt")
        g = sc.viewer_args(s, "guest", "D", {"join": "127.0.0.1:1"}, files, 99)
        self.assertIn("--coop-join", g)
        self.assertIn("--coop-test-token", g)
        self.assertNotIn("--stage", g)
        for a in (h, g):
            self.assertEqual(a[a.index("--coop-quit-after") + 1], "99")
            self.assertEqual(a[a.index("--coop-log") + 1], "l")

    def test_args_accepted_by_strict_parser(self):
        # the fake mirrors args.rs arities, so a bad flag shape in the builder fails here
        s = sc.validate(base(new_game=True, character="c.txt",
                             host={"script": [{"stage": "Q 1"}, {"at": 1, "run": "x"}, {"talk": True}, {"choose": [1]}, {"say": "a"}]}))
        files = {"log": "l", "state": "s", "quit": "q"}
        for role, ports in (("host", {"host": 5}), ("guest", {"join": "127.0.0.1:5"})):
            fake_viewer.parse(sc.viewer_args(s, role, "D", ports, files, 5))

    def test_plain_address_needs_the_test_token(self):
        with self.assertRaises(fake_viewer.Usage):
            fake_viewer.parse(["D", "C", "--coop-join", "127.0.0.1:5", "--coop-log", "l", "--coop-state-dump", "s"])
        o = fake_viewer.parse(["D", "C", "--coop-join", "127.0.0.1:5", "--coop-test-token", "--coop-log", "l", "--coop-state-dump", "s"])
        self.assertTrue(o["coop_test_token"])

    def test_relay_schedule(self):
        s = sc.validate(base(network="bad_wifi", duration_s=10, outages=[{"at": 4, "seconds": 2}]))
        self.assertEqual(sc.relay_schedule(s), [{"at": 4, "outage": "both", "seconds": 2}, {"at": 10, "profile": "perfect"}])
        s = sc.validate(base(network="perfect"))
        self.assertEqual(sc.relay_schedule(s), [])


class Evaluate(unittest.TestCase):
    H = {"quests": {"Q": {"stage": 10}}, "items": {"guest": {"Caps": 25}}, "positions": {"host": [100.0, 1.0, 2.0, 0.5]}, "x": 1}
    G = {"quests": {"Q": {"stage": 10}}, "items": {"guest": {"Caps": 50}}, "positions": {"host": [102.0, 1.0, 2.0, 0.0]}}

    def ev(self, **e):
        e.setdefault("id", "a")
        e = sc.validate(base(expect=[e]))["expect"]
        return sc.evaluate(e, self.H, self.G)

    def test_ops(self):
        self.assertTrue(self.ev(path="quests.Q.stage", op="ge", value=10)[0]["ok"])
        self.assertFalse(self.ev(path="quests.Q.stage", op="gt", value=10)[0]["ok"])
        self.assertFalse(self.ev(path="quests.Q.stage", op="eq", value=9)[0]["ok"])
        self.assertTrue(self.ev(path="quests.Q.stage", op="ne", value=9)[0]["ok"])
        self.assertTrue(self.ev(path="x", op="present", on="host")[0]["ok"])
        self.assertFalse(self.ev(path="x", op="present", on="guest")[0]["ok"])
        self.assertTrue(self.ev(path="x", op="absent", on="guest")[0]["ok"])

    def test_both_gives_one_result_per_peer(self):
        r = self.ev(path="items.guest.Caps", op="eq", value=25)
        self.assertEqual([c["ok"] for c in r], [True, False])
        self.assertEqual([c["id"] for c in r], ["a@host", "a@guest"])

    def test_equal_and_missing(self):
        self.assertFalse(self.ev(path="items.guest.Caps", on="equal")[0]["ok"])
        self.assertEqual(self.ev(path="items.guest.Caps", on="equal")[0]["kind"], "desync")
        self.assertFalse(self.ev(path="nope", on="equal")[0]["ok"])
        self.assertFalse(self.ev(path="nope", op="eq", value=1, on="host")[0]["ok"])

    def test_near_positions(self):
        self.assertTrue(self.ev(path="positions.host", op="near", value=[101, 1, 2], tol=3, on="guest")[0]["ok"])
        self.assertFalse(self.ev(path="positions.host", op="near", value=[110, 1, 2], tol=3, on="guest")[0]["ok"])
        self.assertTrue(self.ev(path="positions.host", on="equal", tol=2.5)[0]["ok"])
        self.assertFalse(self.ev(path="positions.host", on="equal", tol=1)[0]["ok"])

    def test_dist_from(self):
        e = dict(path="positions.host", op="dist_from", **{"from": [100, 1, 999]})
        self.assertTrue(self.ev(cmp="ge", value=0, on="host", **e)[0]["ok"])
        self.assertTrue(self.ev(cmp="le", value=0.5, on="host", **e)[0]["ok"])      # H is exactly at 100,1
        self.assertFalse(self.ev(cmp="ge", value=1, on="host", **e)[0]["ok"])
        e2 = dict(path="positions.host", op="dist_from", **{"from": [102, 5]})      # XY only, z ignored
        self.assertTrue(self.ev(cmp="ge", value=4, on="host", **e2)[0]["ok"])      # 4.47 away
        self.assertFalse(self.ev(cmp="ge", value=5, on="host", **e2)[0]["ok"])
        self.assertFalse(self.ev(cmp="le", value=1, on="guest", **e2)[0]["ok"])    # G at 102,1 -> 4 away
        e3 = dict(path="nope", op="dist_from", **{"from": [0, 0]})
        self.assertFalse(self.ev(cmp="ge", value=0, on="host", **e3)[0]["ok"])      # missing fails

    def test_dist_from_validation(self):
        for bad in ({"cmp": "ge", "value": 1}, {"from": [1, 2], "value": 1}, {"from": [1, 2], "cmp": "ge"},
                    {"from": [1], "cmp": "ge", "value": 1}, {"from": [1, 2], "cmp": "near", "value": 1}):
            with self.assertRaises(sc.ScenarioError):
                sc.validate(base(expect=[dict(id="a", path="p", op="dist_from", **bad)]))

    def test_dist_from_never_moved_fails(self):
        spawn = [2352, 1603.6, 7360]
        state = {"positions": {"host": [2352.0, 1603.6, 7360.0, 0.0]}}
        e = sc.validate(base(expect=[dict(id="m", path="positions.host", op="dist_from", cmp="ge", value=150,
                                          on="host", **{"from": spawn})]))["expect"]
        self.assertFalse(sc.evaluate(e, state, state)[0]["ok"])
        state = {"positions": {"host": [2552.0, 1603.6, 7360.0, 0.0]}}
        self.assertTrue(sc.evaluate(e, state, state)[0]["ok"])

    def test_equal_with_tolerance_and_dims(self):
        h = {"positions": {"host": [100.0, 1.0, 2.0, 0.5]}}
        g = {"positions": {"host": [120.0, 30.0, 400.0, 0.0]}}
        def run(**kw):
            e = sc.validate(base(expect=[dict(id="a", path="positions.host", on="equal", **kw)]))["expect"]
            return sc.evaluate(e, h, g)[0]["ok"]
        self.assertFalse(run(tol=40))                  # z differs by 398
        self.assertTrue(run(tol=40, dims=2))           # x 20, y 29
        self.assertFalse(run(tol=25, dims=2))

    def test_sum(self):
        h = {"items": {"host": {"Caps": 20}, "guest": {"Caps": 30}}}
        def run(cmp, value, state=h):
            e = sc.validate(base(expect=[dict(id="a", path="items.host.Caps", op="sum", add=["items.guest.Caps"],
                                              cmp=cmp, value=value, on="host")]))["expect"]
            return sc.evaluate(e, state, state)[0]["ok"]
        self.assertTrue(run("eq", 50))
        self.assertFalse(run("eq", 100))                # double grant
        self.assertFalse(run("eq", 50, {"items": {"host": {"Caps": 20}}}))   # guest take lost
        self.assertTrue(run("eq", 20, {"items": {"host": {"Caps": 20}}}))    # a missing path counts 0

    def test_get_path_list_index(self):
        self.assertEqual(sc.get_path(self.H, "positions.host.0"), 100.0)



class HashMatch(unittest.TestCase):
    HOST = {"hash": {"shared": "c", "rev": 12, "recent": [[12, "c"], [11, "b"], [10, "a"]]}}

    def test_equal_hashes_match(self):
        ok, _ = sc.hash_match({"hash": {"shared": "a"}}, {"hash": {"shared": "a"}})
        self.assertTrue(ok)

    def test_guest_behind_matches_the_host_root_at_its_revision(self):
        ok, detail = sc.hash_match(self.HOST, {"hash": {"shared": "a", "rev": 10}})
        self.assertTrue(ok, detail)
        self.assertIn("2 revisions behind", detail)

    def test_guest_behind_with_a_wrong_hash_fails(self):
        ok, _ = sc.hash_match(self.HOST, {"hash": {"shared": "z", "rev": 10}})
        self.assertFalse(ok)

    def test_guest_too_far_behind_fails(self):
        ok, detail = sc.hash_match(self.HOST, {"hash": {"shared": "a", "rev": 3}})
        self.assertFalse(ok)
        self.assertIn("no host root", detail)

    def test_no_revision_fails_on_a_difference(self):
        ok, _ = sc.hash_match(self.HOST, {"hash": {"shared": "a"}})
        self.assertFalse(ok)

    def test_equal_expect_on_hash_uses_the_revision(self):
        e = [{"id": "h", "path": "hash.shared", "on": "equal", "op": "eq"}]
        r = sc.evaluate(e, self.HOST, {"hash": {"shared": "b", "rev": 11}})
        self.assertTrue(r[0]["ok"], r)
        r = sc.evaluate(e, self.HOST, {"hash": {"shared": "a", "rev": 11}})
        self.assertFalse(r[0]["ok"], r)


if __name__ == "__main__":
    unittest.main()
