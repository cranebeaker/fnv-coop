"""The parity sub-score in the benchmark scorecard (tools/bench/coop_bench.py): present, scored, and kept out of the composite and out of the sync number."""
import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
sys.path[:0] = [str(HERE), str(HERE.parent)]


def load_bench():
    sys.path[:0] = [str(ROOT / "tools" / "bench"), str(ROOT / "tools"), str(ROOT / "tools" / "runner")]
    spec = importlib.util.spec_from_file_location("coop_bench_for_parity_test", ROOT / "tools" / "bench" / "coop_bench.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


CB = load_bench()

OVERALL = {"copy0.fps_p5": 58.0, "copy1.fps_p5": 56.0, "copy0.hitch_pct": 0.0, "copy1.hitch_pct": 1.0, "copy0.bridge_rtt_ms_p95": 30.0, "copy1.bridge_rtt_ms_p95": 35.0,
           "sync.seq_lag_p95": 6.0, "sync.digest_mismatch_pm": 1.0, "sync.repair_effects_pm": 20.0, "avatar.err_p95": 300.0, "avatar.teleports_pm": 1.0}


class Hook(unittest.TestCase):
    def test_the_metrics_and_the_category_exist(self):
        self.assertIn("parity.npc_mismatch_pct", CB.TARGETS)
        self.assertIn("parity.puppet_mismatch_pct", CB.TARGETS)
        self.assertEqual(CB.CATEGORIES["parity"], ["parity.npc_mismatch_pct", "parity.puppet_mismatch_pct"])

    def test_without_samples_the_parity_score_is_empty_and_nothing_else_changes(self):
        sc, cats, comp = CB.scorecard(dict(OVERALL))
        self.assertIsNone(cats["parity"])
        self.assertIsNone(sc["parity.npc_mismatch_pct"]["score"])
        self.assertIsNotNone(cats["sync"])
        self.assertIsNotNone(comp)

    def test_parity_is_scored_but_neither_the_composite_nor_sync_move(self):
        sc0, cats0, comp0 = CB.scorecard(dict(OVERALL))
        sc1, cats1, comp1 = CB.scorecard(dict(OVERALL), None, {"parity": {"npc_mismatch_pct": 13.5, "puppet_mismatch_pct": 2.0}})
        self.assertEqual(cats1["parity"], round((50.0 + 100.0) / 2, 1))
        self.assertEqual(comp1, comp0)
        self.assertEqual(cats1["sync"], cats0["sync"])
        self.assertEqual(cats1["avatars"], cats0["avatars"])
        self.assertEqual(sc1["sync.seq_lag_p95"], sc0["sync.seq_lag_p95"])

    def test_good_and_bad_ends_of_the_scale(self):
        self.assertEqual(CB.score("parity.npc_mismatch_pct", 2.0), 100.0)
        self.assertEqual(CB.score("parity.npc_mismatch_pct", 25.0), 0.0)
        self.assertEqual(CB.score("parity.puppet_mismatch_pct", 60.0), 0.0)

    def test_aggregate_leaves_parity_out_of_the_composite_too(self):
        def run(par):
            sc, cats, comp = CB.scorecard(dict(OVERALL), None, {"parity": par} if par else None)
            return {"composite": comp, "categories": cats, "scores": sc, "meta": {"stamp": "x"}}
        a = CB.aggregate([run(None), run(None)], "a")
        b = CB.aggregate([run({"npc_mismatch_pct": 25.0, "puppet_mismatch_pct": 25.0}), run({"npc_mismatch_pct": 25.0, "puppet_mismatch_pct": 25.0})], "b")
        self.assertEqual(a["composite"], b["composite"])
        self.assertEqual(b["categories"]["parity"], 0.0)

    def test_the_run_folder_files_are_scored_again_by_parity_extras(self):
        import test_parity as tp   # the sample builders of the comparer tests
        files = tp.Files()
        with tempfile.TemporaryDirectory() as d:
            h, g = files.world(n=8)
            files.write(Path(d) / "parity_host.jsonl", h)
            files.write(Path(d) / "parity_client.jsonl", g)
            par = CB.parity_extras(d)
            self.assertIsNotNone(par)
            self.assertGreater(par["npc_mismatch_pct"], 0)
        with tempfile.TemporaryDirectory() as d:
            self.assertIsNone(CB.parity_extras(d))

    def test_the_report_has_a_parity_section(self):
        L = CB.report_extras([], {"phases": {}}, {"parity": {"paired_by": "link", "pairs": 8, "span_s": 28.0, "rounds": 8, "skipped": 0, "refs": 24, "refs_bad": 7,
                                                              "mismatch_pct": 29.2, "npc_mismatch_pct": 87.5, "puppet_mismatch_pct": 0.0,
                                                              "kinds": {"seated": {"checks": 8, "bad": 7, "rate_pct": 87.5}},
                                                              "worst": [{"key": "0x00000100", "kind": "seated", "rounds_bad": 7, "rounds_seen": 8, "first_s": 4.0,
                                                                         "last_s": 28.0, "host": "standing", "client": "seated"}]}})
        text = "\n".join(L)
        self.assertIn("## Parity", text)
        self.assertIn("0x00000100", text)
        self.assertIn("seated", text)


if __name__ == "__main__":
    unittest.main(verbosity=1)
