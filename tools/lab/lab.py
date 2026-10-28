#!/usr/bin/env python3
"""The lab runner: every co-op mechanic checked in about 20 minutes, failing fast, with one short results file (docs/LAB.md, docs/tools/lab.md).

    python -I tools/lab/lab.py run --pairs 3                      # all rooms, split over 3 pairs of copies (spawn first on every pair)
    python -I tools/lab/lab.py run --pairs 1 --rooms fight,loot   # only these rooms (spawn always runs first: it puts the pair into the lab)
    python -I tools/lab/lab.py smoke                              # spawn room only, about 3 minutes
    python -I tools/lab/lab.py summary out/lab/<stamp>            # print a finished run's summary again
    python -I tools/lab/lab.py down                               # stop the copies a lab run left running (--keep)
    python -I tools/lab/lab.py run --pairs 3 --net good_wifi,bad_wifi   # then the same rooms again, the client joined THROUGH the chaos relay (tools/chaos) with each profile
    python -I tools/lab/lab.py run --split                        # two-PC smoke: host on the second PC, client on this PC, over the real LAN (spawn, walk, quest)
    add --fake to run / smoke for the game-free run on the fake world (tools/lab/labfake.py); --bug NAME switches one fake bug on

A room stops at its first failed check and records what differed (no reruns; --confirm K reruns only PASSED rooms K more times on a fresh load).
Output: out/lab/<stamp>/results.json (machine readable) and summary.md (under 5 KB: a table, 3 to 6 lines per failure, a comparison with the previous run).
Exit: 0 all rooms PASS, 1 a room FAILed / timed out / crashed, 3 BLOCKED (layout, plugin or build missing), 4 ENV (the PC was not usable), 2 bad arguments.
"""
from __future__ import annotations

import argparse
import json
import sys
import threading
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import lab_core as K            # noqa: E402
import lab_fps                  # noqa: E402
import lab_net as N             # noqa: E402
import lab_report as R          # noqa: E402
from lab_layout import ROOM_ORDER, Layout, LayoutError, sample_layout   # noqa: E402
from lab_rooms import PARAMS, ROOMS, split_rooms    # noqa: E402

DEFAULT_LAYOUT = ROOT / "out" / "lab" / "layout.json"
LAB_ROOT = ROOT / "out" / "lab"
HANG_GRACE_S = 20.0           # a room that is still running this long after its time box is stuck in a bridge call


def err(*a) -> None:
    print(*a, file=sys.stderr, flush=True)


def blocked_row(name: str, pair: int, reason: str, cls: str, verdict: str = "BLOCKED") -> Dict[str, Any]:
    return {"room": name, "pair": pair, "verdict": verdict, "class": cls, "reason": reason, "seconds": 0, "checks": [], "metrics": {},
            "area": ROOMS[name].area if name in ROOMS else "", "first_divergence": None, "counters_moved": {}}


SPAWN_ENTRY_CHECKS = ("is moved to its spawn marker", "both copies are in the lab cell", "both copies are connected")


def spawn_entered(row: Dict[str, Any]) -> bool:
    """The pair is in the lab and connected (spawn's entry checks passed): a later spawn failure (idle digest) is a finding, not a reason to skip the other rooms."""
    checks = row.get("checks") or []
    return all(any(e in c.get("name", "") and c.get("ok") for c in checks) for e in SPAWN_ENTRY_CHECKS) and \
        sum(1 for c in checks if "is moved to its spawn marker" in c.get("name", "") and c.get("ok")) >= 2


def parse_params(items: List[str]) -> Dict[str, float]:
    out = dict(PARAMS)
    for it in items or []:
        k, _, v = it.partition("=")
        if k not in PARAMS:
            raise ValueError(f"unknown parameter {k!r}; the names are in lab_rooms.PARAMS")
        out[k] = float(v)
    return out


def box_of(name: str, room_budget_s: float) -> float:
    """The hard time box of one room: --room-budget, or the room's own larger one (spawn: it also loads the save and enters the lab)."""
    return max(room_budget_s, ROOMS[name].budget_s)


def run_one(name: str, layout: Layout, env, pair: int, host, client, params: Dict[str, float], budget_s: float, log: Callable[[str], None], profile: str = "") -> Dict[str, Any]:
    """One room on one pair, inside a hard time box. Never raises. `profile`: the chaos profile of a network pass ("" = the clean pass)."""
    spec = ROOMS[name]
    clock = env.clock(pair)
    ctx = K.Ctx(name, layout, host, client, clock, budget_s, params, probe=env.probe(pair), enter=env.enter(pair) if name == "spawn" else None,
                area=spec.area, log=log, profile=profile, copies=getattr(env, "copies_running", None))
    if getattr(clock, "virtual", False):
        row = K.run_room(name, spec.fn, ctx)
    else:
        box: Dict[str, Any] = {}
        th = threading.Thread(target=lambda: box.setdefault("row", K.run_room(name, spec.fn, ctx)), daemon=True)
        th.start()
        th.join(budget_s + HANG_GRACE_S)
        row = box.get("row")
        if row is None:       # still inside a bridge call: report what was recorded so far, and do not touch this pair again
            row = {"room": name, "verdict": "TIMEOUT", "class": "ENV", "reason": f"room still running {HANG_GRACE_S:.0f} s after its {budget_s:.0f} s time box (a bridge call hung)",
                   "area": spec.area, "seconds": round(ctx.elapsed(), 1), "checks": list(ctx.checks), "metrics": dict(ctx.metrics), "hung": True,
                   "fps": ctx.fps_report(), "first_divergence": None, "counters_moved": {}}
    row["pair"] = pair
    row["budget_s"] = round(budget_s, 1)
    if profile:
        row["profile"] = profile
    return row


def run_pass(k: int, names: List[str], layout: Layout, env, params: Dict[str, float], budget_s: float, room_budget_s: float, log: Callable[[str], None],
             profile: str = "") -> Tuple[List[Dict[str, Any]], str, float]:
    """All rooms of one pair in one pass (clean, or one chaos profile), in order. -> (rows, why the pair is unusable afterwards or "", the pass's start on the pair's clock)."""
    host, client = env.machines(k)
    clock = env.clock(k)
    t0 = clock.now()
    dead_reason = ""
    rows: List[Dict[str, Any]] = []
    tag = f"pair {k}" + (f" [{profile}]" if profile else "")

    def tagged(row: Dict[str, Any]) -> Dict[str, Any]:
        return dict(row, profile=profile) if profile else row
    for n in names:
        left = budget_s - (clock.now() - t0)
        if dead_reason:
            rows.append(tagged(blocked_row(n, k, dead_reason, "ENV")))
            continue
        miss = layout.missing(n)
        if miss:
            rows.append(tagged(blocked_row(n, k, "; ".join(miss), "TEST")))
            if n == "spawn":
                dead_reason = "spawn cannot run (layout), so the pair is never put into the lab"
            continue
        if left < 5.0 and K.MUTANT != "no_run_budget":
            rows.append(tagged(blocked_row(n, k, f"the run budget of {budget_s / 60:.1f} min was used up before this room started", "ENV", "TIMEOUT")))
            continue
        box = box_of(n, room_budget_s)
        row = run_one(n, layout, env, k, host, client, params, min(box, left) if K.MUTANT != "no_run_budget" else box, log, profile)
        rows.append(row)
        log(f"{tag}: {n} {row['verdict']}{' (' + str(row['class']) + ')' if row.get('class') else ''} in {row['seconds']:g} s" + ("" if row["verdict"] == "PASS" else f": {row['reason'][:120]}"))
        if row.get("hung"):
            dead_reason = f"a bridge call hung in room {n}; this pair is not used any more"
        elif row.get("class") == "CRASH":
            dead_reason = f"a game copy exited in room {n}"
            if hasattr(env, "crash_info"):
                row["crash"] = env.crash_info(k)
        elif n == "spawn" and row["verdict"] != "PASS" and not spawn_entered(row) and (row.get("class") in ("ENV", "TEST") or row["verdict"] in ("BLOCKED", "TIMEOUT")):
            dead_reason = f"the pair did not get into the lab (spawn: {row['reason'][:100]})"
    return rows, dead_reason, t0


def run_pair(k: int, names: List[str], layout: Layout, env, params: Dict[str, float], budget_s: float, room_budget_s: float, confirm: int,
             out: List[Dict[str, Any]], log: Callable[[str], None], nets: Optional[List[str]] = None, net_out: Optional[List[Dict[str, Any]]] = None,
             total_s: float = 0.0, net_seed: int = 7, net_info: Optional[List[Dict[str, Any]]] = None) -> float:
    """All rooms of one pair, in order, then (with `nets`) the same rooms again for each chaos profile. Returns the seconds the pair took on its clock."""
    err_ = env.prepare(k)
    if err_:
        cls, why = err_
        for n in names:
            out.append(blocked_row(n, k, why, cls))
        for prof in (nets or []):
            for n in names:
                net_out.append(dict(blocked_row(n, k, why, cls), profile=prof))
        log(f"pair {k}: not usable: {why}")
        return 0.0
    host, client = env.machines(k)
    clock = env.clock(k)
    rows, dead_reason, t0 = run_pass(k, names, layout, env, params, budget_s, room_budget_s, log)
    # --confirm K: rooms that passed run again K times, each on a fresh load of the lab (the first failure counts, there is no retry)
    for n in names:
        row = next((r for r in rows if r["room"] == n), None)
        if not confirm or row is None or row["verdict"] != "PASS" or dead_reason:
            continue
        passed = 0
        for j in range(1, confirm + 1):
            left = budget_s - (clock.now() - t0)
            if left < 5.0:
                break
            if env.reset(k):
                break
            host, client = env.machines(k)
            r2 = run_one("spawn", layout, env, k, host, client, params, min(box_of("spawn", room_budget_s), left), log)      # spawn puts the fresh pair into the lab
            if r2["verdict"] == "PASS" and n != "spawn":
                r2 = run_one(n, layout, env, k, host, client, params, min(box_of(n, room_budget_s), max(5.0, budget_s - (clock.now() - t0))), log)
            if r2["verdict"] != "PASS":
                row.update(verdict="FAIL", reason=f"confirm run {j}/{confirm}: " + r2["reason"], checks=r2["checks"], first_divergence=r2.get("first_divergence"),
                           counters_moved=r2.get("counters_moved"), digests_moved=r2.get("digests_moved"), **{"class": r2.get("class") or "SYNC"})
                break
            passed += 1
        row["confirm"] = {"runs": confirm, "passed": passed}
    out.extend(rows)
    # the network passes: a fresh lab (the clean pass left doors open, hostiles dead and the quest at stage 30), the client rejoined through the relay
    for prof in (nets or []):
        if K.MUTANT == "net_pass_skipped":       # twin: the profiles are accepted and never run
            break
        left_total = (total_s or budget_s) - (clock.now() - t0)
        # A copy that EXITED in an earlier pass is no reason to skip the profiles: every profile pass restarts the pair (env.reset). Only a pair that cannot
        # be used at all (a hung bridge, a spawn that never got into the lab) stays blocked. One intermittent engine crash in one room used to blank the
        # whole network table of its pair (lab run 2026-10-27: VATS, FalloutNV.exe+400F66).
        if dead_reason and (not dead_reason.startswith("a game copy exited") or K.MUTANT == "crash_blocks_profiles"):   # twin: a crash blocks the profiles again
            net_out.extend(dict(blocked_row(n, k, f"clean pass: {dead_reason}", "ENV"), profile=prof) for n in names)
            continue
        if left_total < 30.0:
            net_out.extend(dict(blocked_row(n, k, f"the {(total_s or budget_s) / 60:.0f} min total budget was used up before the {prof} pass", "ENV", "TIMEOUT"), profile=prof) for n in names)
            continue
        if not hasattr(env, "net_begin"):
            net_out.extend(dict(blocked_row(n, k, "this environment cannot join through the chaos relay", "ENV"), profile=prof) for n in names)
            continue
        bad = env.reset(k)         # fresh copies, fresh lab (a live pair is restarted: see LiveEnv.reset)
        why = (f"{bad[1]}" if bad else None) or env.net_begin(k, prof, net_seed)
        if why:
            net_out.extend(dict(blocked_row(n, k, why, "ENV"), profile=prof) for n in names)
            log(f"pair {k} [{prof}]: not usable: {why}")
            continue
        try:
            nrows, ndead, _t = run_pass(k, names, layout, env, params, min(budget_s, left_total), room_budget_s, log, prof)
        finally:
            info = env.net_end(k)
            if info is not None and net_info is not None:
                net_info.append(dict(info, pair=k, profile=prof))
        net_out.extend(nrows)
        dead_reason = ndead if ndead and not ndead.startswith("a game copy exited") else ("" if dead_reason.startswith("a game copy exited") else dead_reason)
    return clock.now() - t0


def execute(layout: Layout, env, rooms: List[str], pairs: int, budget_s: float, room_budget_s: float, params: Dict[str, float], confirm: int = 0,
            stamp: str = "", mode: str = "live", log: Callable[[str], None] = err, started: Optional[float] = None, nets: Optional[List[str]] = None,
            total_s: float = 0.0, net_seed: int = 7) -> Dict[str, Any]:
    """Run the rooms on `pairs` pairs at the same time and return the results dict (what results.json holds)."""
    t_wall = time.time()
    n, note = env.pairs_allowed(pairs)
    names = ["spawn"] + [r for r in ROOM_ORDER if r in rooms and r != "spawn"]
    plan = split_rooms(names, n)
    rows: List[Dict[str, Any]] = []
    net_rows: List[Dict[str, Any]] = []
    net_info: List[Dict[str, Any]] = []
    lock = threading.Lock()
    took: List[float] = []
    nets = list(nets or [])

    def go(k: int, ns: List[str]):
        mine: List[Dict[str, Any]] = []
        mine_net: List[Dict[str, Any]] = []
        try:
            took.append(run_pair(k, ns, layout, env, params, budget_s, room_budget_s, confirm, mine, log, nets, mine_net, total_s, net_seed, net_info))
        except Exception as e:   # noqa: BLE001 - a bug in the runner must still leave a results file
            mine += [blocked_row(x, k, f"runner error {type(e).__name__}: {str(e)[:120]}", "TEST") for x in ns if not any(r["room"] == x for r in mine)]
            for prof in nets:
                mine_net += [dict(blocked_row(x, k, f"runner error {type(e).__name__}: {str(e)[:120]}", "TEST"), profile=prof) for x in ns
                             if not any(r["room"] == x and r.get("profile") == prof for r in mine_net)]
        with lock:
            rows.extend(mine)
            net_rows.extend(mine_net)
    ths = [threading.Thread(target=go, args=(k, ns), daemon=True) for k, ns in enumerate(plan)]
    [t.start() for t in ths]
    [t.join() for t in ths]
    rows.sort(key=lambda r: (ROOM_ORDER.index(r["room"]), r["pair"]))
    net_rows.sort(key=lambda r: (nets.index(r["profile"]), ROOM_ORDER.index(r["room"]), r["pair"]))
    verdict, cls = R.overall(rows + net_rows)
    notes = [note] if note else []
    if n < pairs:
        notes.append(f"{n} of {pairs} requested pairs used")
    fps = lab_fps.collect(rows, net_rows, nets)          # the pooled fps per pass; drops the raw intervals from the rows
    if hasattr(env, "copy_info"):
        try:
            fps["provision"] = env.copy_info()            # throttling state, window options and frame-rate ini keys of the copies (what could hold the fps down)
        except Exception:   # noqa: BLE001
            pass
    return {"schema": R.SCHEMA, "stamp": stamp or time.strftime("%Y%m%d-%H%M%S"), "build": env.build_id(), "mode": mode, "pairs": n, "budget_min": round(budget_s / 60, 1),
            "wall_seconds": round(time.time() - t_wall, 1), "room_budget_s": room_budget_s, "layout_name": Path(layout.path).name if layout.path else "sample", "seconds": round(max(took, default=time.time() - t_wall), 1),
            "verdict": verdict, "class": cls, "notes": notes, "rooms": rows, "confirm": confirm, "params": {k: v for k, v in params.items() if PARAMS.get(k) != v},
            "nets": nets, "net_seed": net_seed, "net_rooms": net_rows, "net_info": sorted(net_info, key=lambda i: (nets.index(i["profile"]), i["pair"])),
            "clean_verdict": R.overall(rows)[0], "fps": fps}


def blocked_results(rooms: List[str], reason: str, cls: str, stamp: str, mode: str, budget_min: float) -> Dict[str, Any]:
    names = ["spawn"] + [r for r in ROOM_ORDER if r in rooms and r != "spawn"]
    rows = [blocked_row(n, 0, reason, cls) for n in names]
    return {"schema": R.SCHEMA, "stamp": stamp, "build": "n/a", "mode": mode, "pairs": 0, "budget_min": budget_min, "room_budget_s": 0, "layout_name": "?", "seconds": 0,
            "verdict": "BLOCKED", "class": cls, "notes": [], "rooms": rows, "confirm": 0, "params": {}, "nets": [], "net_rooms": [], "net_info": []}


def pick_rooms(arg: str) -> List[str]:
    if not arg:
        return list(ROOM_ORDER)
    want = [x.strip() for x in arg.split(",") if x.strip()]
    bad = [x for x in want if x not in ROOMS]
    if bad:
        raise ValueError(f"unknown room(s) {bad}; the rooms are {list(ROOM_ORDER)}")
    return want


def pick_nets(arg: str) -> List[str]:
    """--net good_wifi,bad_wifi: profile names of tools/chaos/profiles.json (ValueError for an unknown one)."""
    want = [x.strip() for x in (arg or "").split(",") if x.strip()]
    known = N.profiles()
    bad = [x for x in want if x not in known]
    if bad:
        raise ValueError(f"unknown chaos profile(s) {bad}; the profiles are {sorted(known)} (tools/chaos/profiles.json)")
    return list(dict.fromkeys(want))


def cmd_run(a, smoke: bool = False) -> int:
    stamp = a.stamp or time.strftime("%Y%m%d-%H%M%S")
    lab_root = Path(a.out_root) if a.out_root else LAB_ROOT
    out_dir = lab_root / stamp
    split = bool(getattr(a, "split", False)) and not smoke
    try:
        rooms = ["spawn"] if smoke else pick_rooms(a.rooms if (a.rooms or not split) else "spawn,walk,quest")
        params = parse_params(a.param)
        nets = [] if smoke else pick_nets(getattr(a, "net", ""))
        if split and (nets or a.fake):
            raise ValueError("--split runs the real LAN between two PCs: it does not combine with --net (the chaos relay) or --fake")
    except ValueError as e:
        err(str(e))
        return 2
    budget_s = (3.0 if smoke else a.budget_min) * 60
    total_s = float(getattr(a, "total_min", 0) or 0) * 60 if nets else 0.0
    mode = "fake" if a.fake else "live"
    pairs = 1 if (smoke or split) else max(1, a.pairs)
    mode = "split" if split else mode
    if a.fake:
        import labfake
        layout = Layout.load(a.layout) if a.layout else Layout(sample_layout())
        try:
            env = labfake.FakeEnv(layout, bugs=a.bug)
        except ValueError as e:
            err(str(e))
            return 2
    else:
        lp = Path(a.layout) if a.layout else DEFAULT_LAYOUT
        try:
            layout = Layout.load(lp)
        except LayoutError as e:
            res = blocked_results(rooms, f"no usable layout: {e}. Generate it with tools/lab/esp (out/lab/layout.json)", "TEST", stamp, mode, a.budget_min)
            return finish(res, out_dir, lab_root)
        if split:
            import lab_split
            env = lab_split.SplitEnv(layout, a, log=err, lab_dir=lab_root)
        else:
            import lab_live
            env = lab_live.LiveEnv(layout, a, log=err, lab_dir=lab_root)
    try:
        res = execute(layout, env, rooms, pairs, budget_s, float(a.room_budget), params, a.confirm, stamp, mode, err, nets=nets, total_s=total_s,
                      net_seed=int(getattr(a, "net_seed", 7)))
        if split:
            res["split"] = env.split_info()
    finally:
        env.finish()
    return finish(res, out_dir, lab_root)


def finish(res: Dict[str, Any], out_dir: Path, lab_root: Path) -> int:
    path = R.write_run(out_dir, res, lab_root)
    print(path.read_text(encoding="utf-8"))
    print(f"results: {out_dir / 'results.json'}")
    return R.exit_code(res["verdict"], res.get("class"))


def cmd_summary(a) -> int:
    p = Path(a.dir)
    if not (p / "results.json").is_file():
        err(f"{p}: no results.json")
        return 2
    print(R.reprint(p))
    return 0


def cmd_down(a) -> int:
    import lab_live
    from types import SimpleNamespace
    env = lab_live.LiveEnv(SimpleNamespace(plugin="NVCoopLab.esp", cell_edid="NVCoopLab"), SimpleNamespace(pairs=1, keep=False, reserve_gb=1.5), lab_dir=LAB_ROOT)
    env.pv.finish(True)
    print("stopped the copies this tool started (owner 'lab'); the owner's own game is never touched")
    return 0


def main(argv: Optional[List[str]] = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    def common(p):
        p.add_argument("--layout", default="", help="layout.json of the lab plugin (default out/lab/layout.json)")
        p.add_argument("--fake", action="store_true", help="game-free run on the fake world")
        p.add_argument("--bug", action="append", default=[], help="(fake) switch one fake bug on; the matching room must then fail")
        p.add_argument("--param", action="append", default=[], metavar="ROOM.KEY=VALUE", help="change one threshold of a room for this run")
        p.add_argument("--room-budget", type=float, default=120.0, help="hard time box per room in seconds (default 120)")
        p.add_argument("--save", default="nvcoop_autosave_ready", help="the save loaded once per pair (boxes already pressed away)")
        p.add_argument("--keep", action="store_true", help="leave the game copies running afterwards")
        p.add_argument("--reserve-gb", type=float, default=1.5)
        p.add_argument("--stamp", default="", help=argparse.SUPPRESS)
        p.add_argument("--out-root", default="", help=argparse.SUPPRESS)
    r = sub.add_parser("run", help="run the rooms")
    common(r)
    r.add_argument("--pairs", type=int, default=1)
    r.add_argument("--rooms", default="", help="comma list (default all); spawn always runs first")
    r.add_argument("--budget-min", type=float, default=20.0, help="whole run, minutes (default 20)")
    r.add_argument("--confirm", type=int, default=0, metavar="K", help="rerun rooms that PASSED K more times, each on a fresh load")
    r.add_argument("--net", default="", metavar="P1,P2", help="after the clean pass run the rooms again for each chaos profile (names in tools/chaos/profiles.json, "
                   "for example good_wifi,bad_wifi): the client joins the host THROUGH the chaos relay")
    r.add_argument("--split", action="store_true", help="two-PC smoke: the HOST copy runs on the second PC, the CLIENT copy on this PC, over the real LAN (rooms spawn, walk, quest "
                   "unless --rooms); refuses under 3 GB free RAM here, and when Steam is not running and signed in on both PCs")
    r.add_argument("--remote", default="dino", help="--split: ssh name of the second PC")
    r.add_argument("--remote-ip", default="192.168.0.10", help="--split: its address on the local network (the client joins it there)")
    r.add_argument("--min-free-gb", type=float, default=3.0, help="--split: free RAM this PC must have before the one local copy starts")
    r.add_argument("--net-seed", type=int, default=7, help="seed of the relay's loss and jitter (the same seed gives the same drops)")
    r.add_argument("--total-min", type=float, default=15.0, help="with --net: the most one pair works on all its passes together, minutes (default 15)")
    s = sub.add_parser("smoke", help="spawn room only, about 3 minutes")
    common(s)
    s.set_defaults(pairs=1, rooms="", budget_min=3.0, confirm=0)
    m = sub.add_parser("summary", help="print the summary of a finished run")
    m.add_argument("dir")
    sub.add_parser("down", help="stop the game copies a lab run left running")
    a = ap.parse_args(argv)
    if a.cmd == "run":
        return cmd_run(a)
    if a.cmd == "smoke":
        return cmd_run(a, smoke=True)
    if a.cmd == "summary":
        return cmd_summary(a)
    return cmd_down(a)


if __name__ == "__main__":
    sys.exit(main())
