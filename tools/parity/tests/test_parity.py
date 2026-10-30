"""Tests of tools/parity/compare.py (game-free). Run by tools/parity/run_tests.py, once for real and once per twin (PARITY_MUTANT=<name>).

The shared vectors (tools/parity/vectors.json) are the same ones plugin/parity/tests/test_parity.cpp runs: both comparers must give the same answers.
"""
import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import compare as pc  # noqa: E402
import gen_vectors  # noqa: E402

VEC = json.loads((HERE.parent / "vectors.json").read_text(encoding="utf-8"))


def ent(t="n", ref="0x00000100", x=0, **kw):
    e = {"t": t, "r": ref, "x": x, "y": 0, "z": 0, "sp": 0, "se": 0, "d": 0, "dis": 0, "hp": 5, "g": 0}
    if t != "n":
        e.update({"sl": kw.pop("sl", 1), "ac": 0})
    e.update(kw)
    return e


def sample(slot, ents, role="host", why="tick", seq=1, ms=0, wall=0, cmp=None, cov=6000, cell="0x00001234"):
    o = {"v": 1, "k": "s", "role": role, "slot": slot, "seq": seq, "ms": ms, "wall": wall, "gt": 1.0, "cell": cell, "ws": "0x00000000", "rad": 6000, "cov": cov, "menu": 0,
         "why": why, "e": ents}
    if cmp:
        o["cmp"] = cmp
    return pc.Sample(o)


class SharedVectors(unittest.TestCase):
    def test_vectors_are_current(self):
        self.assertEqual(json.dumps(gen_vectors.vectors(), indent=1, sort_keys=True) + "\n", (HERE.parent / "vectors.json").read_text(encoding="utf-8"))

    def test_group_classes(self):
        for g, c in VEC["group_class"]:
            self.assertEqual(pc.group_class(g), c, g)
        self.assertEqual(len(VEC["group_class_all"]), 256)
        for g in range(256):
            self.assertEqual(pc.group_class(g), VEC["group_class_all"][g])

    def test_primary_group(self):
        for groups, want in VEC["primary_group"]:
            self.assertEqual(pc.primary_group(groups), want, groups)

    def test_hp_bucket(self):
        for hp, want in VEC["hp_bucket"]:
            self.assertEqual(pc.hp_bucket(hp), want, hp)

    def test_pos_tolerance(self):
        t = pc.Tol()
        for row in VEC["pos_tol"]:
            self.assertAlmostEqual(pc.pos_tolerance(t, row["a"], row["b"]), row["tol"], places=2)

    def test_every_case(self):
        self.assertGreaterEqual(len(VEC["cases"]), 20)
        for c in VEC["cases"]:
            cmp = pc.Comparer()
            for i, st in enumerate(c["steps"]):
                r = cmp.compare(pc.Sample(st["host"]), pc.Sample(st["client"]), st["t"])
                self.assertEqual(r.ids(), sorted(st["expect"]), "%s step %d" % (c["name"], i))
                self.assertEqual(r.skipped, bool(st.get("skipped")), "%s step %d skipped" % (c["name"], i))


class Rules(unittest.TestCase):
    def test_persistence_boundaries(self):
        c = pc.Comparer()
        h = sample(0, [ent(se=0)])
        g = sample(1, [ent(se=1)], role="guest")
        self.assertEqual(c.compare(h, g, 0).ids(), [])
        self.assertEqual(c.compare(h, g, 2999).ids(), [])
        self.assertEqual(c.compare(h, g, 3000).ids(), ["seated|0x00000100"])

    def test_a_pending_difference_goes_stale(self):
        c = pc.Comparer()
        h, g = sample(0, [ent(se=0)]), sample(1, [ent(se=1)], role="guest")
        c.compare(h, g, 0)
        # the actor drops out of the samples for longer than stale_ms, then comes back: the clock starts over
        c.compare(sample(0, []), sample(1, [], role="guest"), 13000)
        self.assertEqual(c.compare(h, g, 14000).ids(), [])

    def test_a_runner_gets_a_wider_tolerance(self):
        c = pc.Comparer()
        h, g = sample(0, [ent(x=0, sp=304)]), sample(1, [ent(x=500, sp=304)], role="guest")
        c.compare(h, g, 0)
        self.assertEqual(c.compare(h, g, 4000).ids(), [])

    def test_net_sample_cuts_and_narrows_coverage(self):
        ents = [ent(ref="0x%08X" % (0x1000 + i), x=100 + 10 * i) for i in range(40)] + [ent("s", ref="0x00000014", sl=1, x=0)]
        s = sample(1, ents, role="guest")
        n = pc.net_sample(s, 30)
        self.assertEqual(len([e for e in n.ents if e.kind == "n"]), 30)
        self.assertEqual(n.cov, 400)
        self.assertIs(pc.net_sample(s, 0), s)


class Files(unittest.TestCase):
    def write(self, path, samples, junk=False):
        with open(path, "w", encoding="utf-8") as f:
            for s in samples:
                f.write(json.dumps(s.raw) + "\n")
            if junk:
                f.write('{"v":1,"k":"s","ro')   # a line cut short by a crash

    def run_main(self, args):
        out = io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(io.StringIO()):
            rc = pc.main([str(a) for a in args])
        return rc, out.getvalue()

    def world(self, n=8):
        """host file with its cmp samples, guest file with the sent ones; the guest sees the actor seated from the second round on"""
        host, guest = [], []
        for i in range(n):
            t = 4000 * i
            se = 1 if i >= 1 else 0
            guest.append(sample(1, [ent(se=se), ent("s", ref="0x00000014", sl=1, x=100), ent("p", ref="0x00201000", sl=0, x=0)], role="guest", seq=i + 1, ms=t + 77, wall=1000 + t))
            host.append(sample(0, [ent(se=0), ent("p", ref="0x00201001", sl=1, x=100), ent("s", ref="0x00000014", sl=0, x=0)], role="host", why="cmp", seq=i + 1, ms=t,
                               wall=1000 + t, cmp={"slot": 1, "seq": i + 1}))
        return host, guest

    def test_pairs_by_link_and_reports_the_seated_difference(self):
        with tempfile.TemporaryDirectory() as d:
            h, g = self.world()
            self.write(Path(d) / "h.jsonl", h)
            self.write(Path(d) / "g.jsonl", g, junk=True)
            rc, out = self.run_main([Path(d) / "g.jsonl", Path(d) / "h.jsonl", "--json"])   # files in the "wrong" order: the role field decides
            self.assertEqual(rc, 0)
            res = json.loads(out)
            self.assertEqual(res["paired_by"], "link")
            self.assertEqual(res["pairs"], 8)
            self.assertEqual(res["worst"][0]["kind"], "seated")
            self.assertEqual(res["worst"][0]["key"], "0x00000100")
            self.assertGreaterEqual(res["npc_mismatch_pct"], 50.0)
            self.assertEqual(res["puppet_mismatch_pct"], 0.0)
            self.assertEqual(res["bad_lines"], 1)

    def test_text_report_names_kinds_and_worst_offenders(self):
        with tempfile.TemporaryDirectory() as d:
            h, g = self.world()
            self.write(Path(d) / "h.jsonl", h)
            self.write(Path(d) / "g.jsonl", g)
            rc, out = self.run_main([Path(d) / "h.jsonl", Path(d) / "g.jsonl"])
            self.assertEqual(rc, 0)
            for word in ("kind", "seated", "Worst offenders", "0x00000100"):
                self.assertIn(word, out)

    def test_falls_back_to_the_wall_clock_without_links(self):
        with tempfile.TemporaryDirectory() as d:
            h, g = self.world()
            for s in h:
                s.raw.pop("cmp", None)
                s.raw["why"] = "tick"
            self.write(Path(d) / "h.jsonl", h)
            self.write(Path(d) / "g.jsonl", g)
            rc, out = self.run_main([Path(d) / "h.jsonl", Path(d) / "g.jsonl", "--json"])
            self.assertEqual(json.loads(out)["paired_by"], "wall clock")

    def test_nothing_to_compare_says_so(self):
        res = pc.analyse([], [])
        self.assertIn("nothing to compare", pc.format_report(res))
        self.assertIsNone(pc.bench_metrics("/no/such/file", "/no/such/file"))

    def test_bench_metrics_need_three_rounds(self):
        with tempfile.TemporaryDirectory() as d:
            h, g = self.world(n=2)
            self.write(Path(d) / "h.jsonl", h)
            self.write(Path(d) / "g.jsonl", g)
            self.assertIsNone(pc.bench_metrics(Path(d) / "h.jsonl", Path(d) / "g.jsonl"))
            h, g = self.world(n=8)
            self.write(Path(d) / "h.jsonl", h)
            self.write(Path(d) / "g.jsonl", g)
            m = pc.bench_metrics(Path(d) / "h.jsonl", Path(d) / "g.jsonl")
            self.assertIsNotNone(m)
            self.assertIn("npc_mismatch_pct", m)

    def test_a_run_folder_is_enough(self):
        with tempfile.TemporaryDirectory() as d:
            h, g = self.world()
            self.write(Path(d) / "parity_host.jsonl", h)
            self.write(Path(d) / "parity_client.jsonl", g)
            rc, out = self.run_main([d])
            self.assertEqual(rc, 0)
            self.assertIn("Overall", out)


class Format(unittest.TestCase):
    def test_a_line_written_by_the_plugin_is_read(self):
        line = VEC.get("plugin_line")
        self.assertTrue(line, "vectors.json holds a line written by the C++ sampleJson")
        s = pc.Sample(json.loads(line["line"]))
        want = pc.Sample(line["sample"])
        self.assertEqual(s.slot, want.slot)
        self.assertEqual(s.cell, want.cell)
        self.assertEqual(len(s.ents), len(want.ents))
        for a, b in zip(s.ents, want.ents):
            for f in ("kind", "slot", "ref", "x", "y", "z", "speed16", "seated", "dead", "disabled", "hp", "group", "acts", "look"):
                self.assertEqual(getattr(a, f), getattr(b, f), f)
        self.assertEqual(s.cmp, {"slot": 1, "seq": 9})
        self.assertEqual(s.why, "cmp")
        self.assertEqual(s.role, "host")


if __name__ == "__main__":
    unittest.main(verbosity=1)
