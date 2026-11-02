import os
import unittest

import checklist_real as c


class PlanTests(unittest.TestCase):
    def test_every_real_scenario_exists(self):
        rows = c.load_rows(c.MANIFEST)
        items = c.plan(rows)
        self.assertTrue(items, "no real rows in the manifest")
        for name, path, twin, ids in items:
            if not twin:  # a missing twin is a PENDING row in the headless checklist, not an error here
                self.assertIsNotNone(path, "%s (rows %s) has no scenario file" % (name, ids))

    def test_twins_follow_rows_that_ask_for_one(self):
        rows = [{"id": "a.b", "area": "A", "real": ["doc_stage_sync"], "twin": True}, {"id": "a.c", "area": "A", "real": ["doc_positions"]}]
        names = [(n, t) for n, _, t, _ in c.plan(rows)]
        self.assertIn(("twin_doc_stage_sync", True), names)
        self.assertNotIn(("twin_doc_positions", True), names)

    def test_area_filter(self):
        rows = [{"id": "a", "area": "CORE", "real": ["doc_stage_sync"]}, {"id": "b", "area": "WORLD", "real": ["doc_positions"]}]
        self.assertEqual([n for n, *_ in c.plan(rows, "world")], ["doc_positions"])


if __name__ == "__main__":
    unittest.main()
