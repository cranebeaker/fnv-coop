"""Frames per second of each game copy during a room (the lab's speed gauge).

The numbers come from the plugin's frame counter (`coop.status` counters.frames), the same counter tools/bench/coop_bench.py turns into fps. A room samples both copies
about every 0.5 s on the runner's clock (Ctx.tick), once more when it starts and once when it ends. Between two samples the copy ran (frames b - frames a) / (t1 - t0)
frames per second. Pure functions here, tested alone (tools/lab/tests/test_lab.py, class Fps).

    mean   all frames of the room / all its seconds (so a long stall counts as long as it lasted)
    p5     the fps the slowest 5 percent of the room's TIME ran at or below (weighted by the length of each interval, not by the number of samples)
    min    the slowest interval
    n      intervals; secs  seconds covered

A pass (the clean pass, or one chaos profile) pools the intervals of its rooms; `copies` is the number of game copies that were running while the room ran: six on the
second PC with three pairs, and the fps of every copy is a share of one PC.
"""
from __future__ import annotations

import os
from typing import Any, Dict, List, Optional, Sequence, Tuple

MUTANT = os.environ.get("LAB_MUTANT", "")   # test-only; names in tools/lab/run_tests.py

MIN_DT = 0.3         # seconds: two samples closer than this are one (a counter read twice within a bridge round trip says nothing)
EVERY_S = 0.5        # seconds between two samples taken by Ctx.tick


def rates(samples: Sequence[Tuple[float, Optional[float]]], min_dt: float = MIN_DT) -> List[Tuple[float, float]]:
    """[(t, frames)] -> [(seconds, fps)] per interval between kept samples. A sample less than `min_dt` after the last kept one is skipped (the interval grows to the next
    one); a counter that went DOWN (the game restarted, the plugin reloaded) ends that interval without a number."""
    out: List[Tuple[float, float]] = []
    last: Optional[Tuple[float, float]] = None
    for t, f in samples:
        if f is None:
            continue
        if last is None:
            last = (t, float(f))
            continue
        dt = t - last[0]
        if dt < min_dt:
            continue
        if f < last[1] or dt <= 0:
            last = (t, float(f))
            continue
        out.append((dt, (f - last[1]) / dt))
        last = (t, float(f))
    return out


def weighted_p(pairs: Sequence[Tuple[float, float]], q: float) -> float:
    """The value (fps) at or below which `q` of the TOTAL TIME lies: pairs are (seconds, fps)."""
    if not pairs:
        return 0.0
    order = sorted(pairs, key=lambda p: p[1])
    total = sum(d for d, _ in order)
    need = q * total
    acc = 0.0
    for d, v in order:
        acc += d
        if acc >= need:
            return v
    return order[-1][1]


def stats(pairs: Sequence[Tuple[float, float]]) -> Dict[str, Any]:
    """[(seconds, fps)] -> {"mean","p5","min","n","secs"}; None for the numbers when nothing was measured."""
    pairs = [(d, v) for d, v in pairs if d > 0]
    if not pairs:
        return {"mean": None, "p5": None, "min": None, "n": 0, "secs": 0.0}
    secs = sum(d for d, _ in pairs)
    mean = sum(d * v for d, v in pairs) / secs
    p5 = mean if MUTANT == "fps_p5_is_mean" else weighted_p(pairs, 0.05)       # twin: the low end is not reported
    return {"mean": round(mean, 1), "p5": round(p5, 1), "min": round(min(v for _, v in pairs), 1), "n": len(pairs), "secs": round(secs, 1)}


class FpsMeter:
    """The frame-counter samples of the copies of one room."""

    def __init__(self, names: Sequence[str]):
        self.samples: Dict[str, List[Tuple[float, Optional[float]]]] = {n: [] for n in names}

    def add(self, name: str, t: float, frames: Optional[float]) -> None:
        self.samples.setdefault(name, []).append((t, None if frames is None else float(frames)))

    def since(self, name: str, t0: float) -> Dict[str, Any]:
        """stats of one copy over the samples taken at or after `t0` (a leg of a room: the walk of one player)."""
        return stats(rates([(t, f) for t, f in self.samples.get(name, []) if t >= t0]))

    def report(self, copies: int = 0) -> Dict[str, Any]:
        """{"host": stats, "client": stats, "copies": n, "_rates": {name: [[seconds, fps], ..]}} (_rates is for the pass totals and is dropped from results.json)."""
        out: Dict[str, Any] = {}
        raw: Dict[str, List[List[float]]] = {}
        for name, s in self.samples.items():
            r = rates(s)
            out[name] = stats(r)
            raw[name] = [[round(d, 2), round(v, 1)] for d, v in r]
        out["copies"] = 0 if MUTANT == "fps_copies_dropped" else int(copies)     # twin: the number of copies running is not reported
        out["_rates"] = raw
        return out


def pass_totals(rows: Sequence[Dict[str, Any]]) -> Dict[str, Any]:
    """The fps of one pass: the intervals of all its rooms pooled, per copy role. -> {"host": stats, "client": stats, "copies": max, "rooms": n}."""
    pooled: Dict[str, List[Tuple[float, float]]] = {}
    copies = 0
    n = 0
    for r in rows:
        f = r.get("fps") or {}
        if not f:
            continue
        n += 1
        copies = max(copies, int(f.get("copies") or 0))
        for name, lst in (f.get("_rates") or {}).items():
            pooled.setdefault(name, []).extend((float(d), float(v)) for d, v in lst)
    out: Dict[str, Any] = {name: stats(p) for name, p in pooled.items()}
    out["copies"] = copies
    out["rooms"] = n
    return out


def collect(rows: Sequence[Dict[str, Any]], net_rows: Sequence[Dict[str, Any]], nets: Sequence[str]) -> Dict[str, Any]:
    """results['fps']: the pooled fps of the clean pass and of each chaos profile, then the raw intervals are dropped from the rows (they would triple results.json)."""
    passes: Dict[str, Any] = {"clean": pass_totals(rows)}
    for p in nets:
        passes[p] = pass_totals([r for r in net_rows if r.get("profile") == p])
    allrows = list(rows) + list(net_rows)
    overall = pass_totals(allrows)
    for r in allrows:
        if isinstance(r.get("fps"), dict):
            r["fps"].pop("_rates", None)
    return {"passes": passes, "overall": overall, "copies": overall.get("copies", 0)}


def _n(v: Any) -> str:
    if not isinstance(v, (int, float)) or isinstance(v, bool):
        return "-"
    return f"{v:.1f}" if v < 10 else f"{v:.0f}"


def cell(f: Optional[Dict[str, Any]]) -> str:
    """'host/client mean (p5 host/client)' for one room in one pass."""
    if not f:
        return "-"
    h, c = f.get("host") or {}, f.get("client") or {}
    if h.get("mean") is None and c.get("mean") is None:
        return "-"
    return f"{_n(h.get('mean'))}/{_n(c.get('mean'))} p5 {_n(h.get('p5'))}/{_n(c.get('p5'))}"


def table(res: Dict[str, Any], detail: bool = True) -> List[str]:
    """The summary.md block: room x pass (clean first), then the pooled pass row and the copies running. [] when the run measured no fps."""
    fp = res.get("fps")
    if not fp or not (fp.get("passes") or {}):
        return []
    passes = list(fp["passes"])
    cols: List[Tuple[str, Dict[str, Dict[str, Any]]]] = []
    for p in passes:
        rows = res.get("rooms") or [] if p == "clean" else [r for r in res.get("net_rooms") or [] if r.get("profile") == p]
        by: Dict[str, Dict[str, Any]] = {}
        for r in rows:
            if r.get("fps"):
                cur = by.get(r["room"])
                # a room run on several pairs (spawn): the slowest mean shows
                if cur is None or ((r["fps"].get("host") or {}).get("mean") or 1e9) < ((cur.get("host") or {}).get("mean") or 1e9):
                    by[r["room"]] = r["fps"]
        cols.append((p, by))
    out = ["", "## FPS per copy (game frames/s, mean and p5, host copy/client copy)", "| room | " + " | ".join(p for p, _ in cols) + " |", "|---|" + "---|" * len(cols)]
    if detail:
        order: List[str] = []
        for r in res.get("rooms") or []:
            if r["room"] not in order:
                order.append(r["room"])
        for room in order:
            out.append(f"| {room} | " + " | ".join(cell(by.get(room)) for _, by in cols) + " |")
    out.append("| all rooms | " + " | ".join(cell(fp["passes"].get(p)) for p, _ in cols) + " |")
    out.append("| copies running | " + " | ".join(str(fp["passes"].get(p, {}).get("copies", 0)) for p, _ in cols) + " |")
    return out
