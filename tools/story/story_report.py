"""results.json and summary.md (under 5 KB) of a story run, in the lab's report style (tools/lab/lab_report.py: verdicts, classes, exit codes, the vs-previous block).

    write_run(dir, results, root)  -> writes results.json and summary.md into dir (the previous run is the newest earlier folder of root)
    render_summary(results)        -> the text (pure; tested)

One row per step. Per step the table says who led it, the quest's stage on the host copy / the client copy, how many lines the initiator picked, what each copy
gained (XP, items, caps, karma, reputation) and, for a step that did not pass, the first check that failed or the first value the two copies disagree on.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (HERE, ROOT / "tools" / "lab"):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_report as R                    # noqa: E402
import story_data as D                    # noqa: E402

MUTANT = D.MUTANT
LIMIT = 5000
SCHEMA = 1
GOT_CUT = R.GOT_CUT
PRIORITY = R.PRIORITY
overall = R.overall
exit_code = R.exit_code


def _failed(r: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    return R._failed_check(r)    # noqa: SLF001


def previous_run(root: Path, stamp: str) -> Optional[Dict[str, Any]]:
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
            if data.get("steps") is not None:
                return data
    return None


def compare(cur: Dict[str, Any], prev: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    if not prev:
        return {"stamp": None, "changes": []}
    return R.compare({"rooms": cur.get("steps") or []}, {"rooms": prev.get("steps") or [], "stamp": prev.get("stamp")})


def _short_item(k: str) -> str:
    return k.rpartition(":")[2].lstrip("0") or "0"


def gained(d: Optional[Dict[str, Any]]) -> str:
    """'xp+50 caps+100 14B2BA+1 karma+1 rep Goodsprings+3' for one copy's delta."""
    if not d:
        return "-"
    bits = []
    if d.get("xp"):
        bits.append(f"xp{d['xp']:+g}")
    if d.get("caps"):
        bits.append(f"caps{d['caps']:+g}")
    for k, v in (d.get("items") or {}).items():
        bits.append(f"{_short_item(k)}{v:+d}")
    if d.get("karma"):
        bits.append(f"karma{d['karma']:+g}")
    for n, (f, i) in (d.get("rep") or {}).items():
        bits.append(f"rep {n[:6]}{f:+g}/{i:+g}")
    return " ".join(bits[:4]) or "-"


def stage_cell(r: Dict[str, Any]) -> str:
    s = r.get("story") or {}
    q = r.get("quest")
    a = s.get("after") or {}
    h = ((a.get("host") or {}).get("stages") or {}).get(q)
    c = ((a.get("client") or {}).get("stages") or {}).get(q)
    return "-" if h is None and c is None else f"{q} {h if h is not None else '?'}/{c if c is not None else '?'}"


def obj_cell(r: Dict[str, Any]) -> str:
    """The objectives that changed during the step on each copy: '30c 40d' when both copies agree, 'h 30c / c 30d' when they do not."""
    d = (r.get("story") or {}).get("delta") or {}
    def one(k):
        return " ".join(f"{a.rpartition(':')[2]}{v}" for a, v in ((d.get(k) or {}).get("objectives") or {}).items())[:24]
    h, c = one("host"), one("client")
    return (h or "-") if h == c else f"h {h or '-'} / c {c or '-'}"


def _what(r: Dict[str, Any], n: int) -> str:
    if r["verdict"] == "PASS":
        return ""
    div = (r.get("story") or {}).get("divergence")
    c = _failed(r)
    txt = c["name"] if c else (r.get("reason") or "")
    if div and not c:
        txt = f"{div['key']} host {div['host']} client {div['client']}"
    return txt[:n]


def guest_block(res: Dict[str, Any]) -> List[str]:
    steps = [r for r in res.get("steps") or [] if r.get("guest_dialogue")]
    if not steps:
        return []
    picks = [p for r in steps for p in (r.get("story") or {}).get("picks", []) if p.get("who") == "client"]
    ok = sum(1 for r in steps if r["verdict"] == "PASS")
    ms = [(r.get("story") or {}).get("talk_ms", {}).get("client") for r in steps]
    ms = [m for m in ms if m is not None]
    return [f"Guest-initiated dialogue: {ok}/{len(steps)} steps passed; the client picked {len(picks)} lines ({sum(1 for p in picks if p['said'])} said)"
            + (f"; talk opened in {min(ms)}..{max(ms)} ms" if ms else "")]


def failure_block(r: Dict[str, Any], tight: int = 0) -> List[str]:
    cut = (lambda s, n: s if len(s) <= n else s[: n - 1] + "~") if tight else (lambda s, n: s)
    s = r.get("story") or {}
    out = [f"### {r['verdict']} {r['room']} (lead {r.get('who')}, class {r.get('class')}, {r.get('seconds', 0):g} s): {cut(r.get('title', ''), 70)}"]
    c = _failed(r)
    if c:
        out.append(cut(f"- check: {c['name']}", 110 if tight else 220))
        out.append(cut(f"- expected {json.dumps(c.get('expected'), default=str)}, got {json.dumps(c.get('actual'), default=str)}" + (f" ({c['note']})" if c.get("note") else ""), GOT_CUT[min(tight, 3)]))
    else:
        out.append(cut(f"- {r.get('reason', '')}", GOT_CUT[min(tight, 3)]))
    d = s.get("divergence") or r.get("first_divergence")
    if d:
        out.append(cut(f"- first divergence: {d.get('key')} host={d.get('host')} client={d.get('client')}", 120))
    picks = s.get("picks") or []
    if picks and tight < 2:
        out.append(cut("- picks: " + "; ".join(f"{p['who']} {p['info'].rpartition(':')[2]}{'' if p['said'] else ' REFUSED'}" for p in picks[:5]), 140))
    if s.get("log") and tight < 2:
        for who, ls in list(s["log"].items())[:2]:
            out.append(cut(f"- plugin log ({who}): " + " | ".join(l.replace("[nvcoop] ", "")[:70] for l in ls[-2:]), 190))
    if s.get("intents") and tight < 1:
        out.append(cut("- intents.status: " + "; ".join(f"{w} " + ", ".join(f"{k.rpartition('.')[2]}={v:g}" for k, v in list(c.items())[:4] if not isinstance(v, str)) for w, c in s["intents"].items()), 190))
    if s.get("prep") and tight < 2:
        out.append(cut("- start state forced by console: " + ", ".join(s["prep"][:3]), 120))
    if s.get("notes") and tight < 1:
        out.append(cut("- " + s["notes"][0], 160))
    out.append(cut(f"- likely source: {(c or {}).get('area') or r.get('area') or '?'}", 110))
    return out[:8]


def render_summary(res: Dict[str, Any], prev_cmp: Optional[Dict[str, Any]] = None) -> str:
    steps = res.get("steps") or []
    prev_cmp = prev_cmp if prev_cmp is not None else res.get("previous") or {"stamp": None, "changes": []}

    def build(tight: int) -> str:
        head = [f"# Story run {res.get('stamp')}: {res.get('verdict')} ({R._counts(steps)}) in {R._mmss(res.get('seconds', 0))}",     # noqa: SLF001
                f"build {res.get('build')} | {res.get('mode')} | {res.get('line')} | save {res.get('save', '?')}"
                + (f" | {'; '.join(res.get('notes') or [])[:150]}" if res.get("notes") else ""), ""] + guest_block(res) + ["",
                "| step | lead | verdict | s | stage h/c | objectives | picks | gained host / client | first failure |", "|---|---|---|---|---|---|---|---|---|"]
        for r in steps:
            s = r.get("story") or {}
            d = s.get("delta") or {}
            head.append(f"| {r['room']} | {r.get('who')} | {r['verdict']}{'/' + r['class'] if r.get('class') else ''} | {r.get('seconds', 0):g} | {stage_cell(r)} | {obj_cell(r)} | "
                        f"{sum(1 for p in s.get('picks', []) if p['said'])}/{len(s.get('picks', []))} | {gained(d.get('host'))} / {gained(d.get('client'))} | {_what(r, 44 if tight < 2 else 28)} |")
        body: List[str] = []
        bad = [r for r in steps if r["verdict"] != "PASS"]
        limit = len(bad) if tight < 2 else (3 if tight == 2 else 2)
        for r in bad[:limit]:
            body += [""] + failure_block(r, tight)
        if len(bad) > limit:
            body += ["", f"(+{len(bad) - limit} more steps did not pass: see results.json)"]
        forced = [(r["room"], len((r.get("story") or {}).get("prep", []))) for r in steps if (r.get("story") or {}).get("prep")]
        fb = [r["room"] for r in steps if any(w.get("teleported") for w in (r.get("story") or {}).get("walks", []))]
        if tight < 3:
            body += ["", f"Start states forced by console: {len(forced)} steps ({', '.join(a for a, _ in forced)[:90]}). Walks replaced by a teleport: {', '.join(fb) or 'none'}."]
        tail = ["", f"## vs previous ({prev_cmp.get('stamp') or 'none'})"]
        chs = prev_cmp.get("changes") or []
        if not prev_cmp.get("stamp"):
            tail.append("- no earlier story run to compare with")
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
    if len(text.encode("utf-8")) > LIMIT:
        b = text.encode("utf-8")
        text = b[: LIMIT - 40].decode("utf-8", "ignore").rsplit("\n", 1)[0] + "\n(cut at 5 KB; see results.json)\n"
    return text


def write_run(out_dir: Path, res: Dict[str, Any], root: Path) -> Path:
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    prev = previous_run(Path(root), res["stamp"])
    res["previous"] = compare(res, prev)
    (out_dir / "results.json").write_text(json.dumps(res, indent=1, default=str) + "\n", encoding="utf-8")
    (out_dir / "summary.md").write_text(render_summary(res), encoding="utf-8")
    return out_dir / "summary.md"


def reprint(out_dir: Path) -> str:
    return render_summary(json.loads((Path(out_dir) / "results.json").read_text(encoding="utf-8")))
