"""compare_headless.py: table, newest-verdict pick and agreement, with a fake simulator (no game, no cargo)."""
import io
import json
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stdout

import compare_headless as cmp

FAKE_SIM = r'''
import json, os, sys
out = sys.argv[sys.argv.index("--out") + 1]
for name, res in (("a", "pass"), ("b", "fail"), ("c", "pass")):
    d = os.path.join(out, name, "seed1")
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(d, "verdict.json"), "w") as f:
        json.dump({"scenario": name, "result": res, "class": "assert" if res == "fail" else "pass", "reason": "x"}, f)
'''


def put(root, name, result, stamp):
    d = os.path.join(root, name, stamp)
    os.makedirs(d)
    p = os.path.join(d, "verdict.json")
    with open(p, "w") as f:
        json.dump({"scenario": name, "result": result}, f)
    return p


class Compare(unittest.TestCase):
    def test_the_newest_real_verdict_wins(self):
        with tempfile.TemporaryDirectory() as t:
            old = put(t, "a", "fail", "1-0")
            new = put(t, "a", "pass", "2-0")
            os.utime(old, (1, 1))
            os.utime(new, (2000000000, 2000000000))
            self.assertEqual(cmp.newest_verdicts(t)["a"]["result"], "pass")

    def test_table_and_agreement_against_a_fake_simulator(self):
        with tempfile.TemporaryDirectory() as t, tempfile.TemporaryDirectory() as o:
            fake = os.path.join(t, "fake_sim.py")
            with open(fake, "w") as f:
                f.write(FAKE_SIM)
            real = os.path.join(t, "real")
            put(real, "a", "pass", "1-0")
            put(real, "b", "pass", "1-0")   # real passes, simulated fails: a difference
            put(real, "c", "pass", "1-0")
            buf = io.StringIO()
            with redirect_stdout(buf):
                code = cmp.main([real, "--sim", '"%s" -I "%s"' % (sys.executable, fake), "--keep", o])
            text = buf.getvalue()
            self.assertEqual(code, 1)
            self.assertIn("agreement: 2 of 3", text)
            self.assertIn("differences: b", text)
            self.assertIn("assert: x", text)

    def test_the_known_results_cover_the_ten_scenarios_of_the_brief(self):
        fails = sorted(k for k, v in cmp.KNOWN.items() if v == "fail")
        self.assertEqual(fails, ["live_dialogue_session", "live_loot_race", "live_tether_warp"])
        self.assertEqual(sum(1 for k in cmp.KNOWN if not k.startswith("twin_")), 10)


if __name__ == "__main__":
    unittest.main()
