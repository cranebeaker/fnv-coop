"""results.json, summary.md (under 5 KB) and the comparison with the previous lab run.

    write_run(dir, results, previous_root)   -> writes results.json and summary.md into dir
    render_summary(results, previous)        -> the text (pure; tested)
    previous_run(root, stamp)                -> the newest earlier results.json under out/lab, or None
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lab_fps   # noqa: E402

MUTANT = os.environ.get("LAB_MUTANT", "")
LIMIT = 5000
GOT_CUT = {0: 10 ** 6, 1: 300, 2: 220, 3: 150}      # characters of the "expected .., got .." line per tightness level of the summary
PRIORITY = ["CRASH", "FAIL", "TIMEOUT", "BLOCKED", "PASS"]    # worst first
SCHEMA = 1


def overall(rooms: List[Dict[str, Any]]) -> Tuple[str, Optional[str]]:
    """-> (verdict, class) of the whole run: PASS only when every room passed. A crash outranks a failure, a failure a timeout, a timeout a block."""
    if not rooms:
        return "BLOCKED", "TEST"
    crash = [r for r in rooms if r.get("class") == "CRASH"]
    if crash:
        return "FAIL", "CRASH"
    for v in ("FAIL", "TIMEOUT", "BLOCKED"):
        hit = [r for r in rooms if r["verdict"] == v]
        if hit:
            return v, hit[0].get("class")
    return "PASS", None


def exit_code(verdict: str, cls: Optional[str]) -> int:
    if verdict == "PASS":
        return 0
    if verdict in ("FAIL", "TIMEOUT") or cls == "CRASH":
        return 1
    return 4 if cls == "ENV" else 3


def previous_run(root: Path, stamp: str) -> Optional[Dict[str, Any]]:
    """The newest results.json in a sibling folder of an EARLIER stamp (stamps sort as time)."""
    root = Path(root)
    if not root.is_dir():
        return None
    for d in sorted((p for p in root.iterdir() if p.is_dir() and p.name < stamp), reverse=True):
        f = d / "results.json"
        if f.is_file():
            try:
                data = json.loads(f.read_text(encoding="utf-8"))
            except ValueError:
                continue
            if data.get("rooms") is not None:
                return data
    return None


def _by_room(rooms: List[Dict[str, Any]]) -> Dict[str, Dict[str, Any]]:
    """One row per room name; a room run on several pairs (spawn) shows its worst result."""
    out: Dict[str, Dict[str, Any]] = {}
    for r in rooms:
        cur = out.get(r["room"])
        if cur is None or PRIORITY.index(r["verdict"]) < PRIORITY.index(cur["verdict"]):
            out[r["room"]] = r
    return out


def _failed_check(r: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    for c in r.get("checks") or []:
        if not c.get("ok"):
            return c
    return None


def compare(cur: Dict[str, Any], prev: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    """What changed since the previous run: verdicts per room, and numbers that moved by a quarter or more."""
    if not prev:
        return {"stamp": None, "changes": []}
    if MUTANT == "compare_blind":
        return {"stamp": prev.get("stamp"), "changes": []}
    a, b = _by_room(prev.get("rooms") or []), _by_room(cur.get("rooms") or [])
    ch: List[str] = []
    for name in list(b):
        n = b[name]
        o = a.get(name)
        if o is None:
            ch.append(f"{name}: new in this run ({n['verdict']})")
            continue
        if o["verdict"] != n["verdict"]:
            tag = "FIXED" if n["verdict"] == "PASS" else ("REGRESSED" if o["verdict"] == "PASS" else "CHANGED")
            ch.append(f"{name}: {o['verdict']} -> {n['verdict']} ({tag})")
        elif n["verdict"] != "PASS":
            fo, fn = _failed_check(o), _failed_check(n)
            if fo and fn and fo["name"] != fn["name"]:
                ch.append(f"{name}: still {n['verdict']}, now at '{fn['name'][:60]}' (was '{fo['name'][:60]}')")
            else:
                ch.append(f"{name}: still {n['verdict']}, same check")
        for k, v in sorted((n.get("metrics") or {}).items()):
            pv = (o.get("metrics") or {}).get(k)
            if isinstance(v, (int, float)) and isinstance(pv, (int, float)) and not isinstance(v, bool) and abs(v - pv) >= 0.25 * max(abs(pv), 1.0) and abs(v - pv) >= 1.0:
                ch.append(f"{name}.{k}: {pv:g} -> {v:g}")
    for name in a:
        if name not in b:
            ch.append(f"{name}: not run this time (was {a[name]['verdict']})")
    return {"stamp": prev.get("stamp"), "changes": ch}


def _counts(rooms: List[Dict[str, Any]]) -> str:
    n: Dict[str, int] = {}
    for r in rooms:
        n[r["verdict"]] = n.get(r["verdict"], 0) + 1
    return ", ".join(f"{n[v]} {v}" for v in PRIORITY if v in n)


def _mmss(s: float) -> str:
    s = int(round(s))
    return f"{s // 60}m{s % 60:02d}s"


# the numbers of the network table, per room: (label, metric keys shown as a/b (host copy / client copy), unit)
NET_CELLS = {
    "spawn": [("pup", ["puppet_seen_ms"], "ms")],
    "walk": [("e95", ["host_err_p95", "client_err_p95"], "u"), ("lag", ["host_lag_ms", "client_lag_ms"], "ms"), ("fps", ["host_leg_fps", "client_leg_fps"], "")],
    "fight": [("kill", ["host_kill_ms", "client_kill_ms"], "ms")],
    "loot": [("pick", ["host_pickup_ms", "client_pickup_ms"], "ms")],
    "door": [("unlock", ["unlock_ms"], "ms")],
    "npc": [("hand", ["npc_handoff_client_ms", "npc_handoff_host_ms"], "ms")],
    "vats": [("vats", ["vats_ms"], "ms")],
    "quest": [("stage", ["host_stage_ms", "client_stage_ms"], "ms")],
}
SHORT = {"PASS": "PASS", "FAIL": "FAIL", "BLOCKED": "BLK", "TIMEOUT": "TMO"}


def _num(v: Any) -> str:
    if isinstance(v, bool) or not isinstance(v, (int, float)):
        return "-"
    return f"{v:.0f}"


def net_cell(r: Optional[Dict[str, Any]]) -> str:
    """One cell of the network table: the verdict and the room's key numbers (ms, or units for the puppet error), '-' for a number the room did not get to measure."""
    if r is None:
        return "n/a"
    m = r.get("metrics") or {}
    bits = [SHORT.get(r["verdict"], r["verdict"])]
    for label, keys, unit in NET_CELLS.get(r["room"], []):
        vals = [m.get(k) for k in keys]
        if all(not isinstance(v, (int, float)) or isinstance(v, bool) for v in vals):
            continue
        bits.append(f"{label} {'/'.join(_num(v) for v in vals)}{unit}")
    return " ".join(bits)


def net_table(res: Dict[str, Any]) -> List[str]:
    """room x profile: the clean pass first, then each profile; pass/fail and the key numbers (the worst row when a room ran on several pairs)."""
    nets = res.get("nets") or []
    if not nets:
        return []
    cols = [("clean", _by_room(res.get("rooms") or []))] + [(p, _by_room([r for r in res.get("net_rooms") or [] if r.get("profile") == p])) for p in nets]
    order: List[str] = []
    for r in res.get("rooms") or []:
        if r["room"] not in order:
            order.append(r["room"])
    out = ["", "## Network: room x profile (host copy/client copy; ms, u = units)", "| room | " + " | ".join(c for c, _ in cols) + " |", "|---|" + "---|" * len(cols)]
    for room in order:
        out.append(f"| {room} | " + " | ".join(net_cell(by.get(room)) for _, by in cols) + " |")
    tot: Dict[str, Dict[str, int]] = {}
    for i in res.get("net_info") or []:
        t = tot.setdefault(str(i.get("profile")), {"c2s": 0, "c2s_drop": 0, "s2c": 0, "s2c_drop": 0, "pairs": 0})
        rl = i.get("relay") or {}
        for d in ("c2s", "s2c"):
            t[d] += int((rl.get(d) or {}).get("forwarded", 0) or 0)
            t[d + "_drop"] += int((rl.get(d) or {}).get("dropped", 0) or 0)
        t["pairs"] += 1
    for prof, t in tot.items():
        out.append(f"- relay {prof} ({t['pairs']} pair(s)): client to host {t['c2s']} packets ({t['c2s_drop']} dropped), host to client {t['s2c']} ({t['s2c_drop']} dropped)")
    return out


def split_block(res: Dict[str, Any]) -> List[str]:
    """The two-PC smoke: the real LAN numbers next to the sync timings of the rooms it ran."""
    sp = res.get("split")
    if not sp:
        return []
    out = ["", f"## Two-PC LAN (host on the second PC {sp.get('ip', '')}, client on this PC)"]
    p = sp.get("ping_ms") or {}
    out.append(f"- ping: min {_num(p.get('min'))} / avg {_num(p.get('avg'))} / max {_num(p.get('max'))} ms, loss {_num(p.get('loss_pct'))}%" if p else "- ping: no answer")
    for key, label in (("bridge_host_ms", "host (over the ssh tunnel)"), ("bridge_client_ms", "client (local pipe)")):
        b = sp.get(key) or {}
        out.append(f"- bridge round trip, {label}: p50 {_num(b.get('p50'))} ms, p95 {_num(b.get('p95'))} ms" if b else f"- bridge round trip, {label}: no answer")
    out.append(f"- co-op transport round trip: {sp['transport_rtt_ms']:g} ms" if sp.get("transport_rtt_ms") is not None
               else "- co-op transport round trip: none in this build (coop.status counts datagrams and bytes only); the bridge round trip is the number to read")
    out.append("")
    out.append("| room | verdict, timings (host copy/client copy; ms, u = units) |")
    out.append("|---|---|")
    for room, r in _by_room(res.get("rooms") or []).items():
        out.append(f"| {room} | {net_cell(r)} |")
    return out


def _line_moved(d: Dict[str, Any]) -> str:
    return ", ".join(f"{k} {v:+g}" for k, v in list(d.items())[:4])


def failure_block(r: Dict[str, Any], tight: int = 0) -> List[str]:
    """3 to 6 lines for one failed room: the check, the numbers, the first divergence, the counters, the likely source."""
    cut = (lambda s, n: s if len(s) <= n else s[: n - 1] + "~") if tight else (lambda s, n: s)
    c = _failed_check(r)
    out = [f"### {r['verdict']} {r['room']}{' [' + r['profile'] + ']' if r.get('profile') else ''} (pair {r.get('pair', 0)}, class {r.get('class')}, {r.get('seconds', 0):g} s)"]
    if c:
        act = c.get("actual")
        out.append(cut(f"- check: {c['name']}", 110 if tight else 200))
        # the got clause keeps its head (a bot failure says what failed first: "FAILED postcondition ... why ..."), so a tight summary cuts the tail only
        out.append(cut(f"- expected {json.dumps(c.get('expected'), default=str)}, got {json.dumps(act, default=str)}" + (f" ({c['note']})" if c.get("note") else ""), GOT_CUT[min(tight, 3)]))
    else:
        out.append(cut(f"- {r.get('reason', '')}", GOT_CUT[min(tight, 3)]))
    if r.get("crash"):
        out.append(cut(f"- crash: {r['crash'][0]}", 130))
    fd = r.get("first_divergence")
    if fd:
        out.append(cut(f"- first digest divergence: {fd.get('key')} host={fd.get('host')} client={fd.get('client')}", 120))
    dm = r.get("digests_moved") or {}
    mv = r.get("counters_moved") or {}
    if (any(dm.values()) or mv) and tight < 2:
        bits = [f"digest {k} {v:+d}" for k, v in dm.items() if v]
        out.append(cut("- moved: " + "; ".join(bits + ([_line_moved(mv)] if mv else [])), 120))
    out.append(cut(f"- likely source: {(c or {}).get('area') or r.get('area') or '?'}", 100))
    if len(out) < 4 and not tight:
        out.append(f"- rerun: python -I tools/lab/lab.py run --pairs 1 --rooms {r['room']}")
    return out[:6]


def render_summary(res: Dict[str, Any], prev_cmp: Optional[Dict[str, Any]] = None) -> str:
    rooms = res.get("rooms") or []
    prev_cmp = prev_cmp if prev_cmp is not None else res.get("previous") or {"stamp": None, "changes": []}
    net_rooms = res.get("net_rooms") or []

    def build(tight: int) -> str:
        head = [f"# Lab run {res.get('stamp')}: {res.get('verdict')} ({_counts(rooms)}" + (f"; network {_counts(net_rooms)}" if net_rooms else "") + f") in {_mmss(res.get('seconds', 0))}",
                f"build {res.get('build')} | {res.get('mode')} | {res.get('pairs')} pair(s) | layout {res.get('layout_name', '?')}"
                + (f" | {'; '.join(res.get('notes') or [])[:160]}" if res.get("notes") else ""), "",
                "| room | pair | verdict | class | secs | checks | first failed check |", "|---|---|---|---|---|---|---|"]
        for r in rooms:
            ch = [c for c in r.get("checks") or [] if c.get("status") != "skipped"]
            c = _failed_check(r)
            first = (c["name"][:60] if c else (r.get("reason") or "")[:60]) if r["verdict"] != "PASS" else ""
            head.append(f"| {r['room']} | {r.get('pair', 0)} | {r['verdict']} | {r.get('class') or ''} | {r.get('seconds', 0):g} | "
                        f"{sum(1 for x in ch if x['ok'])}/{len(ch)} | {first} |")
        body: List[str] = net_table(res) + lab_fps.table(res, detail=tight < 2) + split_block(res)
        bad = [r for r in rooms if r["verdict"] != "PASS"]
        limit = len(bad) if tight < 3 else max(1, 3)
        for r in bad[:limit]:
            body += [""] + failure_block(r, tight)
        if len(bad) > limit:
            body += ["", f"(+{len(bad) - limit} more failed rooms: see results.json)"]
        bad_net = [r for r in net_rooms if r["verdict"] != "PASS"]
        nlimit = min(len(bad_net), (6, 4, 3, 2)[min(tight, 3)])
        for r in bad_net[:nlimit]:
            body += [""] + failure_block(r, tight)
        if len(bad_net) > nlimit:
            body += ["", f"(+{len(bad_net) - nlimit} more failed network rooms: see results.json)"]
        skipped = [(r["room"], c["name"]) for r in rooms for c in r.get("checks") or [] if c.get("status") == "skipped"]
        if skipped and tight < 2:
            body += ["", "Skipped checks: " + "; ".join(f"{a}: {b[:50]}" for a, b in skipped[:3])]
        tail = ["", f"## vs previous ({prev_cmp.get('stamp') or 'none'})"]
        chs = prev_cmp.get("changes") or []
        if not prev_cmp.get("stamp"):
            tail.append("- no earlier lab run to compare with")
        elif not chs:
            tail.append("- no change")
        else:
            keep = 8 if tight < 2 else 4
            tail += [f"- {c}" for c in chs[:keep]] + ([f"- (+{len(chs) - keep} more in results.json)"] if len(chs) > keep else [])
        return "\n".join(head + body + tail) + "\n"

    if MUTANT == "summary_unbounded":
        return build(0)
    text = build(0)
    for tight in (1, 2, 3):
        if len(text.encode("utf-8")) <= LIMIT:
            break
        text = build(tight)
    if len(text.encode("utf-8")) > LIMIT:       # last resort: keep the head and the comparison, cut the middle
        b = text.encode("utf-8")
        text = b[: LIMIT - 40].decode("utf-8", "ignore").rsplit("\n", 1)[0] + "\n(cut at 5 KB; see results.json)\n"
    return text


def write_run(out_dir: Path, res: Dict[str, Any], lab_root: Path) -> Path:
    """results.json + summary.md in out_dir; the previous run is the newest earlier folder of lab_root."""
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    prev = previous_run(Path(lab_root), res["stamp"])
    res["previous"] = compare(res, prev)
    (out_dir / "results.json").write_text(json.dumps(res, indent=1, default=str) + "\n", encoding="utf-8")
    (out_dir / "summary.md").write_text(render_summary(res), encoding="utf-8")
    return out_dir / "summary.md"


def reprint(out_dir: Path) -> str:
    """lab.py summary <dir>: the summary of a finished run, rebuilt from its results.json (so it is always current with this code)."""
    res = json.loads((Path(out_dir) / "results.json").read_text(encoding="utf-8"))
    return render_summary(res)
