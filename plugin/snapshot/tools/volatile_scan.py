#!/usr/bin/env python3
"""Static scan of the quest scripts for variables that hold per-machine state: a position copy (GetPos / GetAngle /
GetDistance), a per-frame GetSecondsPassed timer, an unguarded per-frame copy of the game clock, and anything derived from those
every frame. Each machine computes its own value of these, so they belong in the volatile list (plugin/snapshot/data/volatile_keys.txt).

    python -I plugin/snapshot/tools/volatile_scan.py check [--questdata out/questdata]   exit 1 and list what the shipped list misses
    python -I plugin/snapshot/tools/volatile_scan.py list  [--questdata out/questdata]   print every candidate as a `var` line

A fifth kind, "cursor", is looked for in the random-encounter spawner quests only (questlines/systems/inventory.json type "encounter"):
a round-robin step counter (iSpawnMonsters 0..3, iFrame, iCount, iEvent, ...) that the host-only spawner script moves every update. The
guest never runs that script (it is not on the GuestLocalRun list on purpose), so the counter is a host-private cursor: two machines
can never agree on it at an instant, and the fight tests showed it as a "shared" difference (iSpawnMonsters host 0, guest 3).

Input is out/questdata/scripts.json.gz + forms.json.gz (tools/questdata). One-shot markers ("set fStart to GameDaysPassed" inside an
if) and random rolls are NOT per-machine state: the host's value must reach the guest, so they are left out on purpose.
"""
import gzip
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
DATA_TXT = os.path.join(HERE, "..", "data", "volatile_keys.txt")

POS = r"getpos|getangle|getdistance|getheadingangle|getlos|getlocalpos"
TIME = r"getsecondspassed|getcurrenttime|gamedayspassed|gamehour"
CMP = r"[<>]|==|!=|&&|\|\|"
FRAME_BLOCKS = ("gamemode", "menumode")
INVENTORY = os.path.join(ROOT, "questlines", "systems", "inventory.json")
CURSOR_MIN_SETS = 4      # `set v to <integer literal>` statements
CURSOR_MIN_VALUES = 3    # distinct literals: a 0/1 flag is a one-shot marker, a step counter walks through 3 or more values


def _statements(src):
    """-> [(block, if-depth, lower-case line)] without comments and blank lines."""
    out, block, depth = [], None, 0
    for line in src.replace("\r", "").split("\n"):
        ll = re.sub(r";.*", "", line).strip().lower()
        if not ll:
            continue
        m = re.match(r"begin\s+(\w+)", ll)
        if m:
            block, depth = m.group(1), 0
            continue
        if ll == "end":
            block = None
            continue
        if re.match(r"if\b", ll):
            depth += 1
            continue
        if ll == "endif":
            depth -= 1
            continue
        out.append((block, depth, ll))
    return out


def _set(ll):
    m = re.match(r"set\s+(\w+)\s+to\s+(.*)$", ll)
    return (m.group(1), m.group(2)) if m else (None, None)


def _cursors(stmts, var_names):
    """Vars the script walks through >= CURSOR_MIN_VALUES integer literals with >= CURSOR_MIN_SETS plain `set v to N` statements."""
    sets = {}
    for _blk, _dep, ll in stmts:
        m = re.match(r"set\s+(\w+)\s+to\s+(-?\d+)\s*$", ll)
        if m and m.group(1) in var_names:
            sets.setdefault(m.group(1), []).append(m.group(2))
    return {v: "set %s to %s ... (%d sets, %d values)" % (v, vals[-1], len(vals), len(set(vals)))
            for v, vals in sets.items() if len(vals) >= CURSOR_MIN_SETS and len(set(vals)) >= CURSOR_MIN_VALUES}


def scan_script(src, var_names, rules=("pos", "timer", "frametime", "derived")):
    """-> {var: (kind, line)} for the per-machine variables of one quest script. var_names: lower-case declared names."""
    stmts = _statements(src)
    kind = {}
    for blk, dep, ll in stmts:
        v, r = _set(ll)
        if not v or v not in var_names or re.search(CMP, r):
            continue
        if "pos" in rules and re.search(r"\b(" + POS + r")\b", r):
            kind[v] = ("pos", ll)
        elif ("timer" in rules and re.search(r"\b" + v + r"\b", r) and re.search(r"\bgetsecondspassed\b", r)
              and blk in FRAME_BLOCKS + ("scripteffectupdate",)):
            kind[v] = ("timer", ll)
        elif "frametime" in rules and re.search(r"\b(" + TIME + r")\b", r) and blk in FRAME_BLOCKS and dep == 0:
            kind[v] = ("frametime", ll)
    if "cursor" in rules:
        for v, ll in _cursors(stmts, var_names).items():
            kind.setdefault(v, ("cursor", ll))
    if "derived" in rules:
        for _ in range(3):
            for blk, dep, ll in stmts:
                v, r = _set(ll)
                if not v or v not in var_names or v in kind or re.search(CMP, r) or blk not in FRAME_BLOCKS:
                    continue
                if any(re.search(r"\b" + t + r"\b", r) for t in list(kind)):
                    kind[v] = ("derived", ll)
    return kind


def load_encounters(path=INVENTORY):
    """{quest script ref lower: (quest ref, quest edid)} of the random-encounter spawner quests (forms.json holds only some QUSTs).
    inventory.json is generated by the systems lane and not committed: without it the "cursor" kind is skipped, the others still run."""
    if not os.path.exists(path):
        print("volatile_scan: %s not found: the cursor kind (encounter spawners) is skipped" % path, file=sys.stderr)
        return {}
    with open(path, encoding="utf-8") as f:
        qs = json.load(f)["quests"]
    return {q["quest_script"].lower(): (q["ref"], q["edid"]) for q in qs if q.get("type") == "encounter" and q.get("quest_script")}


def scan(scripts, forms, rules=("pos", "timer", "frametime", "derived"), encounters=None):
    """-> sorted [(quest ref, quest edid, var name, kind, line)] over every quest script that belongs to a QUST.
    encounters: {quest script ref lower: (quest ref, edid)}; those quests also get the "cursor" rule (default: the shipped inventory)."""
    owner = {}
    for ref, f in forms.items():
        if f.get("sig") == "QUST" and f.get("script"):
            owner.setdefault(f["script"].lower(), (ref, f["edid"]))
    if encounters is None:
        encounters = load_encounters()
    out = []
    for x in scripts:
        s = x.get("script") or {}
        key = x["ref"].lower()
        if s.get("type") != "quest" or (key not in owner and key not in encounters):
            continue
        q = owner.get(key) or encounters[key]
        names = {v["name"].lower() for v in s.get("vars", [])}
        r = tuple(rules) + (("cursor",) if key in encounters and "cursor" not in rules else ())
        for v, (k, line) in scan_script(s.get("source", ""), names, r).items():
            out.append((q[0], q[1], v, k, line))
    return sorted(out)


def quests_missing_from_forms(forms, quests_doc):
    """{quest ref: {"sig": "QUST", "script": script ref, "edid": ...}} for the quests of quests.json that forms.json does not list as a QUST
    with a script. forms.json holds only some QUSTs: the DLC quests and about 270 base-game ones (NVDLC02RainQuest, VStreetLighting,
    every *Timer quest) are only in quests.json, and the scan used to skip their scripts (two-copy tests 2026-10-22: rain UPang / fTimer,
    VStreetLighting LightsOn, all 'differs' in the digest)."""
    have = {f["script"].lower() for f in forms.values() if f.get("sig") == "QUST" and f.get("script")}
    qs = quests_doc.get("quests", quests_doc) if isinstance(quests_doc, dict) else quests_doc
    out = {}
    for x in (qs.values() if isinstance(qs, dict) else qs):
        ref, script = x.get("ref"), x.get("script")
        if ref and script and script.lower() not in have and ref not in forms:
            out[ref] = {"sig": "QUST", "script": script, "edid": x.get("edid", "")}
    return out


def load_questdata(d):
    def rd(n):
        with gzip.open(os.path.join(d, n), "rt", encoding="utf-8") as f:
            return json.load(f)
    forms = rd("forms.json.gz")
    qpath = os.path.join(d, "quests.json")
    if os.path.exists(qpath):
        with open(qpath, encoding="utf-8") as f:
            forms = dict(forms, **quests_missing_from_forms(forms, json.load(f)))
    return rd("scripts.json.gz"), forms


def shipped_vars(path=DATA_TXT):
    """{(owner lower, name lower)} the shipped list covers by name; `owner` lines cover every var of that owner."""
    vars_, owners = set(), set()
    with open(path, encoding="utf-8") as f:
        for line in f:
            t = line.split("#", 1)[0].split()
            if len(t) == 3 and t[0] == "var":
                vars_.add((t[1].lower(), t[2].lower()))
            elif len(t) == 2 and t[0] == "owner":
                owners.add(t[1].lower())
    return vars_, owners


def missing(cands, vars_, owners):
    return [c for c in cands if (c[0].lower(), c[2]) not in vars_ and c[0].lower() not in owners]


def main(argv):
    cmd = argv[0] if argv else "check"
    qd = os.path.join(ROOT, "out", "questdata")
    if "--questdata" in argv:
        qd = argv[argv.index("--questdata") + 1]
    cands = scan(*load_questdata(qd))
    if cmd == "list":
        for q, edid, v, k, line in cands:
            print("var %s %s   # %s (%s: %s)" % (q, v, edid, k, line))
        return 0
    vars_, owners = shipped_vars()
    miss = missing(cands, vars_, owners)
    for q, edid, v, k, line in miss:
        print("MISSING var %s %s   # %s (%s: %s)" % (q, v, edid, k, line))
    print("%d candidates, %d missing from the shipped list" % (len(cands), len(miss)))
    return 1 if miss else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
