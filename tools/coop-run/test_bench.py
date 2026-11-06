"""The benchmark's metrics on made-up samples, and one benchmark run against fake_viewer.py."""
import json
import os
import sys
import tempfile
import unittest

import bench

HERE = os.path.dirname(os.path.abspath(__file__))


def line(w, speed=100.0):
    """A position at time w on a straight walk along x."""
    return [speed * w, 0.0, 0.0]


class Metrics(unittest.TestCase):
    def test_the_lag_of_a_copy_that_follows_late_is_found(self):
        ws = [i * 0.2 for i in range(60)]
        owner = bench.Track([(w, line(w)) for w in ws])
        replica = [(w, line(w - 0.2)) for w in ws[5:]]
        m = bench.pos_metrics(owner, replica)
        self.assertAlmostEqual(m["lag_ms"], 200, delta=20)
        self.assertLess(bench.pct(m["comp"], 95), 2.0)
        self.assertAlmostEqual(bench.pct(m["raw"], 50), 20.0, delta=0.5)

    def test_an_owner_that_stands_still_gives_no_lag(self):
        ws = [i * 0.2 for i in range(30)]
        owner = bench.Track([(w, [5.0, 5.0, 0.0]) for w in ws])
        m = bench.pos_metrics(owner, [(w, [8.0, 9.0, 0.0]) for w in ws])
        self.assertIsNone(m["lag_ms"])
        self.assertAlmostEqual(bench.pct(m["raw"], 50), 5.0)

    def test_animation_bits_agree_inside_the_lag_window(self):
        ws = [i * 0.2 for i in range(30)]
        owner = [(w, 1 if w >= 2.0 else 0) for w in ws]
        late = [(w, 1 if w >= 2.2 else 0) for w in ws]
        ag = bench.anim_agreement(owner, late, 200, 0.15)
        self.assertEqual(ag["all"][0], ag["all"][1])
        wrong = [(w, 8) for w in ws]
        ag = bench.anim_agreement(owner, wrong, 0, 0.15)
        self.assertEqual(ag["fighting"][0], 0)
        self.assertGreater(ag["fighting"][1], 0)

    def test_a_late_walk_animation_agrees_and_a_teleport_is_left_out(self):
        ws = [round(i * 0.2, 3) for i in range(40)]

        def at(w):
            x = 200.0 * min(w, 2.0)  # walks for 2 s, then stands
            return [x + (1000.0 if w >= 5.0 else 0.0), 0.0, 0.0]  # teleported at 5 s
        owner = bench.Track([(w, at(w)) for w in ws])
        puppet = [(w, 1 if 0.0 < w <= 2.2 else 0) for w in ws]  # 0.2 s late
        ag = bench.walk_agreement(owner, puppet, 200, 0.3)
        self.assertEqual(ag["moving"][0], ag["moving"][1])
        self.assertGreater(ag["teleports"][1], 0)
        still = [(w, 1) for w in ws if 3.0 <= w <= 4.6]
        ag = bench.walk_agreement(owner, still, 0, 0.3)
        self.assertEqual(ag["moving"], [0, len(still)], "a walk animation on a player that stands")

    def test_two_samples_close_in_time_do_not_make_a_standing_player_move(self):
        # the player stands, but settles 3 units between two samples 0.02 s apart (150 u/s)
        pts = [(0.0, [0.0, 0.0, 0.0]), (0.2, [0.0, 0.0, 0.0]), (0.22, [3.0, 0.0, 0.0])]
        pts += [(round(0.22 + 0.2 * i, 3), [3.0, 0.0, 0.0]) for i in range(1, 10)]
        owner = bench.Track(pts)
        puppet = [(w, 0) for w, _ in pts]
        ag = bench.walk_agreement(owner, puppet, 0, 0.3)
        self.assertEqual(ag["moving"], [0, 0], "nobody moved")

    def test_a_walk_finding_needs_enough_samples(self):
        base = {"verdict": "pass", "av_anim_moving": 0.0}
        self.assertEqual(bench.flags(dict(base, av_anim_n=1), {}), [])
        self.assertEqual(len(bench.flags(dict(base, av_anim_n=bench.MIN_WALK_SAMPLES), {})), 1)

    def test_a_change_is_timed_on_the_other_machine_and_a_missing_one_is_lost(self):
        def tl(changes, end=10.0):
            out, cur = [], {"quests.Q.stage": 0}
            w = 0.0
            while w <= end + 1e-9:
                for at, path, v in changes:
                    if abs(at - w) < 1e-6:
                        cur = dict(cur)
                        cur[path] = v
                out.append((round(w, 3), cur))
                w = round(w + 0.1, 3)
            return out
        host = tl([(1.0, "quests.Q.stage", 10), (2.0, "dead.X", True), (3.2, "player_quests.guest.R.stage", 5)])
        guest = tl([(1.4, "quests.Q.stage", 10), (3.0, "player_quests.guest.R.stage", 5)])
        ev = bench.propagation({"host": host, "guest": guest})
        by = {(e["path"], e["from"]): e for e in ev}
        self.assertEqual(by[("quests.Q.stage", "host")]["state"], "arrived")
        self.assertAlmostEqual(by[("quests.Q.stage", "host")]["ms"], 400, delta=1)
        self.assertEqual(by[("dead.X", "host")]["state"], "lost")
        self.assertEqual(by[("player_quests.guest.R.stage", "guest")]["ms"], 200)
        self.assertNotIn(("player_quests.guest.R.stage", "host"), by, "the host only followed")
        self.assertNotIn(("quests.Q.stage", "guest"), by, "the guest only followed")

    def test_a_value_changed_again_is_superseded_and_an_early_copy_counts_zero(self):
        def tl(points, end=10.0, start=0):
            out, cur, w = [], {"quests.Q.stage": start}, 0.0
            while w <= end + 1e-9:
                for at, v in points:
                    if abs(at - w) < 1e-6:
                        cur = dict(cur)
                        cur["quests.Q.stage"] = v
                out.append((round(w, 3), cur))
                w = round(w + 0.2, 3)
            return out
        # the host shows 5 for one sample, then 7; the guest never shows 5
        host = tl([(1.0, 5), (1.2, 7)])
        guest = tl([(1.6, 7)])
        ev = bench.propagation({"host": host, "guest": guest})
        states = sorted((e["state"], e["ms"]) for e in ev)
        self.assertEqual(states, [("arrived", 400.0), ("superseded", None)])
        # the guest's sample shows 9 before the host's next sample does
        host = [x for x in tl([(3.0, 9)]) if not 2.2 < x[0] < 3.0]
        guest = tl([(2.4, 9)])
        ev = bench.propagation({"host": host, "guest": guest})
        self.assertEqual([(e["from"], e["state"], e["ms"]) for e in ev], [("host", "arrived", 0.0)])

    def test_a_hash_labelled_one_revision_early_is_no_divergence(self):
        host = [(0.0, 10, "a", [[10, "a"]]), (1.0, 11, "b", [[11, "b"], [10, "a"]])]
        h = bench.hash_metrics(host, [(0.5, 10, "b", None)])
        self.assertEqual((h["divergent"], h["label_skew"]), (0, 1))

    def test_the_hash_lag_and_a_divergence_are_found(self):
        host = [(0.0, 10, "a", [[10, "a"]]), (1.0, 20, "b", [[20, "b"], [10, "a"]]),
                (2.0, 30, "c", [[30, "c"], [20, "b"], [10, "a"]])]
        guest = [(0.5, 10, "a", None), (1.6, 20, "X", None)]
        h = bench.hash_metrics(host, guest)
        # the host reached 11 at 0.1 s and 21 at 1.1 s
        self.assertAlmostEqual(h["lag_ms"][0], 400, delta=1)
        self.assertAlmostEqual(h["lag_ms"][1], 500, delta=1)
        self.assertEqual(h["checked"], 2)
        self.assertEqual(h["divergent"], 1)

    def test_a_guest_with_the_hosts_newest_revision_has_no_lag(self):
        # the host shows revision 7 until 2.0 s and 8 from 2.2 s
        host = [(round(0.2 * i, 1), 7 if i <= 10 else 8, "a" if i <= 10 else "b", None) for i in range(13)]
        guest = [(0.5, 7, "a", None), (1.9, 7, "a", None), (2.4, 7, "a", None), (2.4, 8, "b", None)]
        h = bench.hash_metrics(host, guest)
        self.assertEqual([round(x) for x in h["lag_ms"]], [0, 0, 200, 0])
        self.assertEqual(h["divergent"], 0)

    def test_the_shared_values_leave_out_the_per_machine_ones(self):
        d = {"quests": {"Q": {"stage": 10}}, "scene": {"active": 1, "talk": {"clock_ms": 5}},
             "campaign": {"xp": {"host": 5}, "boxes": {"shown": 3}}, "combat": {"targets": {"G": "guest"}, "downs": 2},
             "globals": {"X": 1}, "positions": {"host": [1, 2, 3]}}
        lv = bench.shared_leaves(d)
        self.assertEqual(lv, {"quests.Q.stage": 10, "scene.active": 1, "campaign.xp.host": 5,
                              "combat.targets.G": "guest", "globals.X": 1})


class FakeRun(unittest.TestCase):
    """One benchmark run against the fake viewer: the sampler writes samples and the report works."""

    def test_a_run_against_the_fake_viewer_gives_a_report(self):
        with tempfile.TemporaryDirectory() as tmp:
            scn = {"name": "bench_fake", "cell": "GSDocMitchellHouse", "network": "perfect", "seed": 5,
                   "duration_s": 3, "settle_s": 1, "timeout_s": 60,
                   "host": {"script": [{"at": 0.3, "stage": "VCG01 10"}]}, "guest": {"script": []},
                   "expect": [{"id": "stage", "path": "quests.VCG01.stage", "op": "eq", "value": 10, "on": "both"}]}
            path = os.path.join(tmp, "bench_fake.json")
            with open(path, "w", encoding="utf-8") as fh:
                json.dump(scn, fh)
            cfg = {"viewer": [sys.executable, "-I", os.path.join(HERE, "fake_viewer.py")], "data": "FakeData",
                   "out_dir": tmp, "base_port": 29400, "launch_timeout_s": 15, "grace_s": 4,
                   "python": sys.executable, "background": True, "screen_size": "640,360"}
            run_dir = os.path.join(tmp, "bench", "bench_fake__good_wifi__1")
            s, res = bench.run_one(path, "good_wifi", cfg, run_dir, 10, slot=7, log=lambda *_: None)
            self.assertEqual(s["profile"], "good_wifi")
            self.assertEqual(s["own_profile"], "perfect")
            self.assertEqual(s["verdict"], "pass", s)
            self.assertNotIn("error", s)
            self.assertGreater(s["samples_ok"], 10)
            self.assertTrue(any(e["path"] == "quests.VCG01.stage" for e in res["state"]), res["state"])
            md = bench.write_report(os.path.dirname(run_dir), [(s, res)], {"quests": ["bench_fake"]})
            self.assertIn("## Per profile", md)
            self.assertTrue(os.path.exists(os.path.join(os.path.dirname(run_dir), "bench.json")))


if __name__ == "__main__":
    unittest.main()
