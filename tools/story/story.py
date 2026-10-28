#!/usr/bin/env python3
"""The story run: Goodsprings (milestone M1) played with real player actions by a host copy and a client copy, one report an AI can read whole.

    python -I tools/story/story.py run                     # all steps on one pair of copies (S00 first: it loads the save and resets the quests)
    python -I tools/story/story.py run --steps S03,S05     # some steps (S00 always runs first; a step's start state is forced by the host's console and recorded)
    python -I tools/story/story.py run --fake [--bug NAME] # the game-free run on the fake world (tools/story/storyfake.py); with a bug the matching step must FAIL
    python -I tools/story/story.py list                    # the steps
    python -I tools/story/story.py summary out/story/<stamp>
    python -I tools/story/story.py down                    # stop the copies a story run left running (only ones this tool started)

A step stops at its first failed check and records what differed; later steps still run from their own start state (--stop-on-fail stops the run).
Output: out/story/<stamp>/results.json and summary.md (under 5 KB), plus the comparison with the previous run. Exit: 0 all PASS, 1 FAIL / TIMEOUT / CRASH,
3 BLOCKED (build, save, method missing), 4 ENV, 2 bad arguments. Contract and findings: docs/tools/story.md.
"""
from __future__ import annotations

import argparse
import json
import sys
import threading
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (HERE, ROOT / "tools" / "lab"):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_core as K                       # noqa: E402
import story_core as C                     # noqa: E402
import story_data as D                     # noqa: E402
import story_report as R                   # noqa: E402

OUT_ROOT = ROOT / "out" / "story"
HANG_GRACE_S = 20.0


def err(*a) -> None:
    print(*a, file=sys.stderr, flush=True)


def parse_params(items: List[str]) -> Dict[str, float]:
    out = dict(C.PARAMS)
    for it in items or []:
        k, _, v = it.partition("=")
        if k not in C.PARAMS:
            raise ValueError(f"unknown parameter {k!r}; the names are in story_core.PARAMS")
        out[k] = float(v)
    return out


def blocked_row(step: Dict[str, Any], reason: str, cls: str, verdict: str = "BLOCKED") -> Dict[str, Any]:
    return {"room": step["id"], "title": step["title"], "who": step["who"], "quest": step.get("quest"), "guest_dialogue": bool(step.get("guest_dialogue")), "pair": 0,
            "verdict": verdict, "class": cls, "reason": reason, "seconds": 0, "checks": [], "metrics": {}, "area": "", "first_divergence": None, "counters_moved": {}, "story": {}}


def run_one(step: Dict[str, Any], script: D.Script, plan_idx: Dict[str, Any], env, host, client, params: Dict[str, float], budget_s: float,
            log: Callable[[str], None]) -> Dict[str, Any]:
    """One step on the pair inside a hard time box. Never raises."""
    clock = env.clock(0)
    mark = env.log_mark(0) if hasattr(env, "log_mark") else None
    ctx = K.Ctx("story", None, host, client, clock, budget_s, params, probe=env.probe(0), enter=env.enter(0) if step.get("enter") else None, area="plugin/hooks, plugin/dialogue", log=log)
    if getattr(clock, "virtual", False):
        row = C.run_story_step(step, script, plan_idx, ctx)
    else:
        box: Dict[str, Any] = {}
        th = threading.Thread(target=lambda: box.setdefault("row", C.run_story_step(step, script, plan_idx, ctx)), daemon=True)
        th.start()
        th.join(budget_s + HANG_GRACE_S)
        row = box.get("row")
        if row is None:
            row = dict(blocked_row(step, f"step still running {HANG_GRACE_S:.0f} s after its {budget_s:.0f} s time box (a bridge call hung)", "ENV", "TIMEOUT"),
                       seconds=round(ctx.elapsed(), 1), checks=list(ctx.checks), metrics=dict(ctx.metrics), hung=True)
    row["pair"] = 0
    row["budget_s"] = round(budget_s, 1)
    if mark is not None and row["verdict"] != "PASS":
        try:
            lg = env.log_since(0, mark)
            if lg:
                row.setdefault("story", {})["log"] = lg
        except Exception:   # noqa: BLE001 - evidence only
            pass
    return row


def execute(script: D.Script, env, steps: List[Dict[str, Any]], params: Dict[str, float], budget_s: float, stop_on_fail: bool = False, stamp: str = "", mode: str = "live",
            log: Callable[[str], None] = err, save: str = "") -> Dict[str, Any]:
    t_wall = time.time()
    plan = D.load_plan()
    plan_idx = D.plan_index(plan)
    rows: List[Dict[str, Any]] = []
    notes: List[str] = []
    n, note = env.pairs_allowed(1)
    if note:
        notes.append(note)
    bad = env.prepare(0)
    clock = env.clock(0)
    t0 = clock.now()
    dead = ""
    if bad:
        cls, why = bad
        dead = why
        notes.append(f"pair not usable: {why[:120]}")
        rows = [blocked_row(s, why, cls) for s in steps]
    else:
        host, client = env.machines(0)
        entered = False
        for s in steps:
            left = budget_s - (clock.now() - t0)
            if dead:
                rows.append(blocked_row(s, dead, "ENV"))
                continue
            if not entered and not s.get("enter"):
                rows.append(blocked_row(s, "S00 did not run, so the save was never loaded", "TEST"))
                continue
            if left < 5.0 and not s.get("enter"):
                rows.append(blocked_row(s, f"the run budget of {budget_s / 60:.1f} min was used up before this step started", "ENV", "TIMEOUT"))
                continue
            row = run_one(s, script, plan_idx, env, host, client, params, min(float(s["box_s"]), max(left, 30.0)) if not s.get("enter") else float(s["box_s"]), log)
            rows.append(row)
            log(f"{s['id']} {row['verdict']}{' (' + str(row['class']) + ')' if row.get('class') else ''} in {row['seconds']:g} s: {s['title'][:70]}"
                + ("" if row["verdict"] == "PASS" else f" -> {row.get('reason', '')[:120]}"))
            if s.get("enter"):
                entered = row["verdict"] == "PASS" or bool((row.get("story") or {}).get("before"))
                if not entered:
                    dead = f"the start state was not reached (S00: {row.get('reason', '')[:100]})"
            if row.get("hung"):
                dead = f"a bridge call hung in step {s['id']}; the pair is not used any more"
            elif row.get("class") == "CRASH":
                dead = f"a game copy exited in step {s['id']}"
                if hasattr(env, "crash_info"):
                    row["crash"] = env.crash_info(0)
            elif stop_on_fail and row["verdict"] != "PASS":
                dead = f"--stop-on-fail: step {s['id']} did not pass"
    verdict, cls = R.overall(rows)
    return {"schema": R.SCHEMA, "kind": "story-run", "stamp": stamp or time.strftime("%Y%m%d-%H%M%S"), "build": env.build_id(), "mode": mode, "line": script.line,
            "script": Path(script.path).name if script.path else "?", "save": save, "budget_min": round(budget_s / 60, 1), "seconds": round(clock.now() - t0, 1),
            "wall_seconds": round(time.time() - t_wall, 1), "verdict": verdict, "class": cls, "notes": notes, "steps": rows, "params": {k: v for k, v in params.items() if C.PARAMS.get(k) != v}}


def pick_steps(script: D.Script, arg: str) -> List[Dict[str, Any]]:
    if not arg:
        return list(script.steps)
    want = [x.strip() for x in arg.split(",") if x.strip()]
    bad = [x for x in want if x not in script.by_id]
    if bad:
        raise ValueError(f"unknown step(s) {bad}; the steps are {[s['id'] for s in script.steps]}")
    first = [s for s in script.steps if s.get("enter")]
    return first + [s for s in script.steps if s["id"] in want and not s.get("enter")]


def finish(res: Dict[str, Any], out_dir: Path, root: Path) -> int:
    path = R.write_run(out_dir, res, root)
    print(path.read_text(encoding="utf-8"))
    print(f"results: {out_dir / 'results.json'}")
    return R.exit_code(res["verdict"], res.get("class"))


def cmd_run(a) -> int:
    stamp = a.stamp or time.strftime("%Y%m%d-%H%M%S")
    root = Path(a.out_root) if a.out_root else OUT_ROOT
    out_dir = root / stamp
    try:
        script = D.load_script(a.script or None)
        problems = D.script_problems(script)
        if problems:
            err("the script is not valid:\n  " + "\n  ".join(problems[:8]))
            return 2
        steps = pick_steps(script, a.steps)
        params = parse_params(a.param)
    except (ValueError, D.ScriptError) as e:
        err(str(e))
        return 2
    budget_s = a.budget_min * 60
    if a.fake:
        import storyfake
        try:
            env = storyfake.FakeStoryEnv(script, D.load_plan(), a.bug)
        except ValueError as e:
            err(str(e))
            return 2
    else:
        import story_live
        env = story_live.StoryLiveEnv(script, a, log=err, out_dir=root)
    try:
        res = execute(script, env, steps, params, budget_s, a.stop_on_fail, stamp, "fake" if a.fake else "live", err, getattr(a, "save", ""))
    finally:
        env.finish()
    return finish(res, out_dir, root)


def cmd_list(a) -> int:
    script = D.load_script(a.script or None)
    for s in script.steps:
        print(f"{s['id']}  {s['who']:6}  {s['box_s']:>4g} s  {s.get('quest', ''):22} {s['title']}")
    return 0


def cmd_probe(a) -> int:
    """Load a save on a pair and print where it stands (cell, position, every story quest's stage / running / objectives on both copies); --save-as NAME saves it
    through the bridge on both copies afterwards (the ready save the story starts from)."""
    import story_live
    script = D.load_script(a.script or None)
    env = story_live.StoryLiveEnv(script, a, log=err, out_dir=Path(a.out_root) if a.out_root else OUT_ROOT)
    try:
        bad = env.prepare(0)
        if bad:
            err(f"pair not usable: {bad}")
            return 4
        host, client = env.machines(0)
        info = env.load_keep(0, a.save)
        rows = {}
        for m in (host, client):
            v = D.view_of(m.state(), script)
            rows[m.name] = {"cell": m.cell_of(), "pos": [round(x) for x in m.pos()], "xp": v["xp"], "karma": v["karma"], "rep": v["rep"],
                            "quests": {q: ({"stage": x["stage"], "running": x["running"], "done": x["done"][-6:], "obj": x["obj"]} if x else None) for q, x in v["quests"].items()}}
            try:
                d = m.pos(script.refs["doc"])
                rows[m.name]["to_doc"] = round(sum((p_ - q_) ** 2 for p_, q_ in zip(m.pos(), d)) ** 0.5)
            except Exception as e:   # noqa: BLE001 - Doc is not loaded here
                rows[m.name]["to_doc"] = f"?: {str(e)[:60]}"
        out = {"save": a.save, "load": info.get("boxes"), "state": rows}
        if a.save_as:
            out["saved_as"] = env.save_as(0, a.save_as)
        print(json.dumps(out, indent=1, default=str))
        return 0
    finally:
        env.finish()


def cmd_summary(a) -> int:
    p = Path(a.dir)
    if not (p / "results.json").is_file():
        err(f"{p}: no results.json")
        return 2
    print(R.reprint(p))
    return 0


def cmd_down(a) -> int:
    import story_live
    from types import SimpleNamespace
    env = story_live.StoryLiveEnv(D.load_script(), SimpleNamespace(reserve_gb=1.5, save=""), out_dir=OUT_ROOT)
    env.pv.finish(True)
    print("stopped the copies this tool started (lease owner 'story'); the owner's own game and the lab's copies are never touched")
    return 0


def main(argv: Optional[List[str]] = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run", help="run the steps")
    r.add_argument("--script", default="", help="a step script (default tools/story/goodsprings.json)")
    r.add_argument("--steps", default="", help="comma list (default all); S00 always runs first")
    r.add_argument("--fake", action="store_true", help="game-free run on the fake world")
    r.add_argument("--bug", action="append", default=[], help="(fake) switch one fake bug on; the matching step must then fail")
    r.add_argument("--param", action="append", default=[], metavar="story.KEY=VALUE", help="change one threshold for this run")
    r.add_argument("--budget-min", type=float, default=25.0, help="whole run, minutes (default 25)")
    r.add_argument("--save", default="", help="the save loaded by S00 (default: the script's start.save)")
    r.add_argument("--stop-on-fail", action="store_true", help="stop at the first step that does not pass (default: later steps force their own start state and go on)")
    r.add_argument("--keep", action="store_true", help="(live) leave the game copies running afterwards")
    r.add_argument("--reserve-gb", type=float, default=1.5)
    r.add_argument("--stamp", default="", help=argparse.SUPPRESS)
    r.add_argument("--out-root", default="", help=argparse.SUPPRESS)
    l = sub.add_parser("list", help="the steps")
    l.add_argument("--script", default="")
    s = sub.add_parser("summary", help="print the summary of a finished run")
    s.add_argument("dir")
    sub.add_parser("down", help="stop the game copies a story run left running")
    pr = sub.add_parser("probe", help="load a save on a pair, print where it stands, optionally save it as a ready save")
    pr.add_argument("--save", required=True)
    pr.add_argument("--save-as", default="")
    pr.add_argument("--script", default="")
    pr.add_argument("--keep", action="store_true")
    pr.add_argument("--reserve-gb", type=float, default=1.5)
    pr.add_argument("--out-root", default="")
    a = ap.parse_args(argv)
    return {"run": cmd_run, "list": cmd_list, "summary": cmd_summary, "down": cmd_down, "probe": cmd_probe}[a.cmd](a)


if __name__ == "__main__":
    sys.exit(main())
