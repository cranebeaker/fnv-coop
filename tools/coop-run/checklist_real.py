#!/usr/bin/env python3
"""Run the `real-game` rows of the master co-op checklist on the test PC.

The manifest (`crates/coop-headless/checklist/*.json`) names, per row, the
`tools/coop-run` scenarios that prove it in two real processes (`real`). This
script collects them, runs each (and its mutation twin when the row asks for
one) through `runner.py`, and prints a table of verdicts. Run it ON the test PC,
as the other `tools/coop-run` scripts; the headless checklist reads the result
with `coop-sim checklist --real-out DIR` and shows the date in its `real-game`
column.

    python -I tools/coop-run/checklist_real.py --list
    python -I tools/coop-run/checklist_real.py --out C:/coop-run-out
    python -I tools/coop-run/checklist_real.py --out OUT --area CORE --n 3

Standard library only. Exit code 0 when every selected scenario passed (twins
failing as expected count as passes), 1 otherwise, 2 for a usage error.
"""
import argparse
import glob
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
MANIFEST = os.path.join(ROOT, "crates", "coop-headless", "checklist")
SCENARIO_DIRS = [os.path.join(HERE, "scenarios"), os.path.join(ROOT, "crates", "coop-headless", "scenarios")]


def load_rows(manifest):
    rows = []
    for f in sorted(glob.glob(os.path.join(manifest, "*.json"))):
        with open(f, encoding="utf-8") as fh:
            rows.extend(json.load(fh)["rows"])
    return rows


def find_scenario(name):
    for d in SCENARIO_DIRS:
        p = os.path.join(d, name + ".json")
        if os.path.isfile(p):
            return p
    return None


def plan(rows, area=None):
    """[(scenario name, path or None, is_twin, [row ids])] for rows with a real scenario."""
    want = {}
    for r in rows:
        if area and not r["area"].lower().startswith(area.lower()):
            continue
        for name in r.get("real", []):
            want.setdefault((name, False), []).append(r["id"])
            if r.get("twin"):
                tw = r.get("twin_ids") or ["twin_" + name]
                for t in tw:
                    want.setdefault((t, True), []).append(r["id"])
    out = []
    for (name, twin), ids in sorted(want.items()):
        out.append((name, find_scenario(name), twin, sorted(set(ids))))
    return out


def run_one(path, out, config, n, seed):
    cmd = [sys.executable, "-I", os.path.join(HERE, "runner.py")]
    if n > 1:
        cmd += ["clean", path, "--n", str(n)]
    else:
        cmd += ["run", path]
    if out:
        cmd += ["--out", out]
    if config:
        cmd += ["--config", config]
    if seed is not None:
        cmd += ["--seed", str(seed)]
    p = subprocess.run(cmd, capture_output=True, text=True)
    last = ""
    for line in p.stdout.splitlines():
        if line.startswith("RESULT:"):
            last = line
    return p.returncode, last or "(no RESULT line)"


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--manifest", default=MANIFEST)
    ap.add_argument("--area")
    ap.add_argument("--out", help="runner output folder (verdict.json files); pass the same to coop-sim --real-out")
    ap.add_argument("--config")
    ap.add_argument("--n", type=int, default=1, help="clean runs per scenario (runner.py clean)")
    ap.add_argument("--seed", type=int)
    ap.add_argument("--list", action="store_true", help="print the plan and exit")
    ap.add_argument("--no-twins", action="store_true")
    a = ap.parse_args(argv)
    try:
        rows = load_rows(a.manifest)
    except (OSError, ValueError, KeyError) as e:
        print("RESULT: FAIL config: %s" % e)
        return 2
    items = [i for i in plan(rows, a.area) if not (a.no_twins and i[2])]
    if not items:
        print("no row names a real-game scenario")
        return 0
    missing = [i for i in items if i[1] is None]
    for name, path, twin, ids in items:
        print("%-34s %-5s %s %s" % (name, "twin" if twin else "main", "MISSING FILE" if path is None else "ok", ",".join(ids)))
    if a.list:
        return 1 if missing else 0
    bad = 0
    for name, path, twin, ids in items:
        if path is None:
            print("FAIL %s: scenario file not found" % name)
            bad += 1
            continue
        code, line = run_one(path, a.out, a.config, a.n, a.seed)
        print("%s %s: %s" % ("ok  " if code == 0 else "FAIL", name, line))
        bad += code != 0
    print("checklist_real: %d scenarios, %d not passing" % (len(items), bad))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
