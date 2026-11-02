"""Compare end-to-end coop-run results with the headless simulator (crates/coop-headless).

    python -I tools/coop-run/compare_headless.py REAL_OUT_DIR [--scenarios DIR] [--sim EXE] [--data DIR]
                                                 [--seeds 1..5] [--glue-fixed] [--keep OUT]
    python -I tools/coop-run/compare_headless.py --known      # the 2026-11-02 test PC results, no folder needed

REAL_OUT_DIR is a coop-run output folder (`<scenario>/<stamp>-<n>/verdict.json`, also the
`remote-<tag>` folders `run_remote.ps1` copies back); the newest verdict of each scenario is used.
`coop-sim suite` runs the same scenario files; its `verdict.json` files (written with --out) are read
back, so the table is built from the two sets of verdicts and not from log text.

A scenario agrees when both say pass or both say fail. A mutation twin counts as pass when it failed as
expected (`class: twin`), which is what the real runner reports too. Exit code 0 when everything compared
agrees, 1 otherwise. Python 3 standard library only.
"""
import argparse
import glob
import json
import os
import shlex
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))

# Verdicts of the real viewer on the test PC, 2026-11-02 (twins all failed as expected).
KNOWN = {
    "doc_stage_sync": "pass", "doc_stage_bad_wifi": "pass", "doc_player_quest_sync": "pass",
    "doc_positions": "pass", "live_avatars_visible": "pass", "live_guest_kills_npc": "pass",
    "live_reconnect": "pass",
    "live_loot_race": "fail", "live_dialogue_session": "fail", "live_tether_warp": "fail",
    "twin_doc_player_quest_sync": "pass", "twin_doc_positions": "pass", "twin_doc_stage_bad_wifi": "pass",
    "twin_doc_stage_sync": "pass", "twin_live_avatars_visible": "pass", "twin_live_dialogue_session": "pass",
    "twin_live_guest_kills_npc": "pass", "twin_live_loot_race": "pass", "twin_live_reconnect": "pass",
    "twin_live_tether_warp": "pass",
}


# Failures of the real viewer that this branch fixes in world / coop-session / coop-core: the viewer on the
# test PC predates the fix, so the simulator passing them is expected (see crates/coop-headless/GAPS.md).
FIXED_SINCE = {
    "live_dialogue_session": "Session::live_intent keeps an early intent; GuestSync keeps early live messages",
}


def newest_verdicts(root):
    """{scenario: verdict dict} from <root>/**/verdict.json, the newest file wins."""
    found = {}
    for path in glob.glob(os.path.join(root, "**", "verdict.json"), recursive=True):
        try:
            with open(path, encoding="utf-8") as f:
                v = json.load(f)
        except (OSError, ValueError):
            continue
        name = v.get("scenario")
        if not name:
            continue
        m = os.path.getmtime(path)
        if name not in found or m > found[name][0]:
            found[name] = (m, v)
    return {k: v for k, (m, v) in found.items()}


def outcome(v):
    """pass / fail / skip for a verdict dict (twins that failed as expected are pass)."""
    return v.get("result", "fail")


def run_sim(sim, scenarios, data, seeds, glue_fixed, out):
    cmd = ([os.path.abspath(sim)] if os.path.exists(sim) else shlex.split(sim)) + ["suite", scenarios, "--out", out]
    if data:
        cmd += ["--data", data]
    if seeds:
        cmd += ["--seeds", seeds]
    if glue_fixed:
        cmd.append("--glue-fixed")
    p = subprocess.run(cmd, capture_output=True, text=True)
    return p.returncode, p.stdout


def sim_verdicts(out):
    """{scenario: [verdict, ...]} for every seed the simulator ran."""
    found = {}
    for path in sorted(glob.glob(os.path.join(out, "*", "seed*", "verdict.json"))):
        with open(path, encoding="utf-8") as f:
            v = json.load(f)
        found.setdefault(v["scenario"], []).append(v)
    return found


def table(rows):
    w = [max(len(str(r[i])) for r in rows + [HEAD]) for i in range(len(HEAD))]
    line = lambda r: "  ".join(str(c).ljust(w[i]) for i, c in enumerate(r))  # noqa: E731
    out = [line(HEAD), line(["-" * n for n in w])] + [line(r) for r in rows]
    return "\n".join(out)


HEAD = ["scenario", "real viewer", "headless", "agree", "headless class / note"]


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("real", nargs="?", help="coop-run output folder with the real verdict.json files")
    ap.add_argument("--known", action="store_true", help="use the built-in 2026-11-02 results instead of a folder")
    ap.add_argument("--scenarios", default=os.path.join(HERE, "scenarios"))
    ap.add_argument("--sim", default=os.environ.get("COOP_SIM", "coop-sim"),
                    help="the coop-sim executable (or a command line that ends where `suite` goes)")
    ap.add_argument("--data", help="Data folder (default FNV_ESM_DIR)")
    ap.add_argument("--seeds", help="seeds for the simulator, e.g. 1..5 (default: each scenario's own)")
    ap.add_argument("--glue-fixed", action="store_true", help="simulate with the proposed viewer glue changes")
    ap.add_argument("--keep", help="keep the simulator's output here (default: a temp folder)")
    a = ap.parse_args(argv)
    if not a.known and not a.real:
        ap.error("give a REAL_OUT_DIR or --known")
    real = {k: {"result": v} for k, v in KNOWN.items()} if a.known else newest_verdicts(a.real)
    if not real:
        print("no verdict.json files under %s" % a.real)
        return 2
    ctx = tempfile.TemporaryDirectory() if not a.keep else None
    out = a.keep or ctx.name
    code, text = run_sim(a.sim, a.scenarios, a.data, a.seeds, a.glue_fixed, out)
    sims = sim_verdicts(out)
    if not sims:
        print("the simulator produced no verdicts (exit %s):\n%s" % (code, text[-2000:]))
        return 2
    rows, disagree = [], []
    for name in sorted(set(real) | set(sims)):
        r = real.get(name)
        s = sims.get(name)
        rv = outcome(r) if r else "-"
        if not s:
            rows.append([name, rv, "-", "-", "not run headless"])
            continue
        outs = [outcome(v) for v in s]
        hv = "pass" if all(o == "pass" for o in outs) else ("skip" if all(o == "skip" for o in outs) else "fail")
        if len(s) > 1:
            hv += " (%d/%d)" % (sum(o == "pass" for o in outs), len(outs))
        note = s[0].get("class", "")
        bad = next((v for v in s if outcome(v) != "pass"), None)
        if bad:
            note = "%s: %s" % (bad.get("class"), bad.get("reason", "")[:90])
        mod = s[0].get("modelled") or []
        if mod and not bad:
            note = "modelled: %d stand-in(s)" % len(mod)
        if not r:
            agree = "-"
        elif outs and all(o == rv for o in outs):
            agree = "yes"
        elif outs and rv == "pass" and hv.startswith("skip"):
            agree = "skip"
        elif rv == "fail" and name in FIXED_SINCE and all(o == "pass" for o in outs):
            agree = "fixed"
            note = "fixed since: " + FIXED_SINCE[name]
        else:
            agree = "NO"
            disagree.append(name)
        rows.append([name, rv, hv, agree, note])
    print(table(rows))
    n = sum(1 for r in rows if r[3] == "yes")
    fixed = [r[0] for r in rows if r[3] == "fixed"]
    print("\nagreement: %d of %d compared scenarios%s" % (
        n, sum(1 for r in rows if r[3] in ("yes", "NO", "fixed")),
        ("; %d fixed since the real run (%s)" % (len(fixed), ", ".join(fixed))) if fixed else ""))
    if disagree:
        print("differences: " + ", ".join(disagree))
    return 1 if disagree else 0


if __name__ == "__main__":
    sys.exit(main())
