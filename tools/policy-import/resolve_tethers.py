#!/usr/bin/env python3
"""Fill the tethers and party warps the first import dropped, from the game data.

    python -I tools/policy-import/resolve_tethers.py --pending <rules.pending.json> --data <esm dir> \
        [--policy crates/coop-policy/data/policy.json] [--check]

The first import (import.py) could not resolve the cells and markers of 37 PartyTether and 8 PartyWarp
rules. This script reads the quest records (QUST, with stage result scripts) and the placed references of
the game masters with a minimal streaming reader, then derives:

  tether: cells   EDIDs of the interior cells that objective targets (QSTA) sit in and that stage scripts
                  `MoveTo` into; for exterior-only quests an `anchor` (EDID of the target or marker
                  reference) with radius 2048 when that reference has an EDID.
          stages  stage numbers whose scripts move the player, only when the cells come from those scripts
                  alone; otherwise empty (every stage is held).
          gather  `wait`.
  warp:   markers EDIDs of the placed references that stage or quest scripts `MoveTo`, plus the marker
                  references (with EDIDs) among the objective targets.

Rows that already carry a tether or warp are never touched. Nothing is invented: a quest with nothing
resolvable stays without the gate and the report says why. The run is deterministic and idempotent.
Inputs are read-only; only the policy file is rewritten (same layout as import.py).
"""
import argparse
import json
import mmap
import os
import re
import struct
import sys
import zlib
from collections import OrderedDict

PLUGINS = ["FalloutNV.esm", "DeadMoney.esm", "HonestHearts.esm", "OldWorldBlues.esm", "LonesomeRoad.esm"]
REF_TYPES = (b"REFR", b"ACHR", b"ACRE")
DEFAULT_RADIUS = 2048
MAX_CELLS = 6  # more distinct interior cells than this is a quest spread over the map, not one scene


def subrecords(data):
    """Yield (type, bytes) for a record payload, handling XXXX oversized subrecords."""
    pos, n, big = 0, len(data), None
    while pos + 6 <= n:
        t = data[pos:pos + 4]
        sz = struct.unpack_from("<H", data, pos + 4)[0]
        pos += 6
        if t == b"XXXX":
            big = struct.unpack_from("<I", data, pos)[0]
            pos += sz
            continue
        if big is not None:
            sz, big = big, None
        yield t, data[pos:pos + sz]
        pos += sz


class Esm:
    def __init__(self, path):
        self.name = os.path.basename(path)
        self.f = open(path, "rb")
        self.m = mmap.mmap(self.f.fileno(), 0, access=mmap.ACCESS_READ)
        self.masters = []
        self.top = []  # (label, start, end) of each top-level group content
        self._header()

    def _header(self):
        m = self.m
        t, size = m[0:4], struct.unpack_from("<I", m, 4)[0]
        if t != b"TES4":
            raise SystemExit("%s: not a TES4 plugin" % self.name)
        for st, d in subrecords(m[24:24 + size]):
            if st == b"MAST":
                self.masters.append(d.split(b"\0")[0].decode("latin-1"))
        pos = 24 + size
        end = len(m)
        while pos < end:
            if m[pos:pos + 4] != b"GRUP":
                raise SystemExit("%s: expected a group at 0x%X" % (self.name, pos))
            gsize = struct.unpack_from("<I", m, pos + 4)[0]
            self.top.append((m[pos + 8:pos + 12], pos + 24, pos + gsize))
            pos += gsize

    def key(self, raw):
        """Global key (plugin, low 24 bits) of a raw form id as stored in this file."""
        h = raw >> 24
        plug = self.masters[h] if h < len(self.masters) else self.name
        return (plug, raw & 0xFFFFFF)

    def walk(self, start, end):
        """Yield ('rec', type, flags, rawid, payload) and ('grp', label, gtype, start, end) in order."""
        m, pos = self.m, start
        while pos < end:
            t = m[pos:pos + 4]
            size = struct.unpack_from("<I", m, pos + 4)[0]
            if t == b"GRUP":
                gtype = struct.unpack_from("<I", m, pos + 12)[0]
                yield ("grp", m[pos + 8:pos + 12], gtype, pos + 24, pos + size)
                pos += size
            else:
                flags, rid = struct.unpack_from("<II", m, pos + 8)
                yield ("rec", t, flags, rid, (pos + 24, size))
                pos += 24 + size

    def payload(self, flags, loc):
        start, size = loc
        d = self.m[start:start + size]
        if flags & 0x00040000:
            d = zlib.decompress(d[4:])
        return d


def interesting_walk(esm, start, end, state, on_rec):
    """Depth-first over groups, giving (type, flags, rawid, loc) to on_rec; state tracks the cell scope."""
    for item in esm.walk(start, end):
        if item[0] == "grp":
            _, label, gtype, s, e = item
            interesting_walk(esm, s, e, state, on_rec)
        else:
            on_rec(item[1], item[2], item[3], item[4])


class World:
    def __init__(self):
        self.quests = OrderedDict()   # edid -> dict
        self.scripts = {}             # (plug,low) -> (edid, text)
        self.cells = {}               # (plug,low) -> dict(edid, interior, world, grid)
        self.refs = {}                # (plug,low) -> dict(edid, cell, pos)
        self.ref_by_edid = {}         # lower edid -> key
        self.cell_by_edid = {}        # lower edid -> key

    def load(self, esms):
        for esm in esms:
            self._pass_quests(esm)
        for esm in esms:
            self._pass_cells(esm)

    def _pass_quests(self, esm):
        for label, s, e in esm.top:
            if label == b"QUST":
                for it in esm.walk(s, e):
                    if it[0] == "rec" and it[1] == b"QUST":
                        self._quest(esm, it[2], it[3], it[4])
            elif label == b"SCPT":
                for it in esm.walk(s, e):
                    if it[0] == "rec" and it[1] == b"SCPT":
                        d = esm.payload(it[2], it[4])
                        edid, text = "", ""
                        for t, b in subrecords(d):
                            if t == b"EDID":
                                edid = b.split(b"\0")[0].decode("latin-1")
                            elif t == b"SCTX":
                                text = b.decode("latin-1")
                        self.scripts[esm.key(it[3])] = (edid, text)

    def _quest(self, esm, flags, rid, loc):
        d = esm.payload(flags, loc)
        q = dict(edid="", key=esm.key(rid), script=None, targets=[], stages=OrderedDict(), objectives=set())
        stage = None
        for t, b in subrecords(d):
            if t == b"EDID":
                q["edid"] = b.split(b"\0")[0].decode("latin-1")
            elif t == b"SCRI":
                q["script"] = esm.key(struct.unpack_from("<I", b)[0])
            elif t == b"QSTA":
                q["targets"].append(esm.key(struct.unpack_from("<I", b)[0]))
            elif t == b"INDX":
                stage = struct.unpack_from("<h", b)[0]
                q["stages"].setdefault(stage, dict(text=[], refs=[]))
            elif t == b"SCTX" and stage is not None:
                q["stages"][stage]["text"].append(b.decode("latin-1"))
            elif t == b"SCRO" and stage is not None:
                q["stages"][stage]["refs"].append(esm.key(struct.unpack_from("<I", b)[0]))
        if q["edid"]:
            self.quests[q["edid"]] = q

    def _pass_cells(self, esm):
        # Refs are kept when they have an EDID or are quest targets or script references.
        wanted = set()
        for q in self.quests.values():
            wanted.update(q["targets"])
            for st in q["stages"].values():
                wanted.update(st["refs"])
        st = dict(cell=None, world="")

        def on_rec(t, flags, rid, loc):
            if t == b"CELL":
                d = esm.payload(flags, loc)
                edid, interior, grid = "", False, None
                for sub, b in subrecords(d):
                    if sub == b"EDID":
                        edid = b.split(b"\0")[0].decode("latin-1")
                    elif sub == b"DATA" and b:
                        interior = bool(b[0] & 1)
                    elif sub == b"XCLC" and len(b) >= 8:
                        grid = struct.unpack_from("<ii", b)
                k = esm.key(rid)
                self.cells[k] = dict(edid=edid, interior=interior, world="" if interior else st["world"], grid=grid)
                if edid:
                    self.cell_by_edid.setdefault(edid.lower(), k)
                st["cell"] = k
            elif t == b"WRLD":
                d = esm.payload(flags, loc)
                for sub, b in subrecords(d):
                    if sub == b"EDID":
                        st["world"] = b.split(b"\0")[0].decode("latin-1")
                        break
            elif t in REF_TYPES:
                k = esm.key(rid)
                d = esm.payload(flags, loc)
                edid, pos = "", None
                for sub, b in subrecords(d):
                    if sub == b"EDID":
                        edid = b.split(b"\0")[0].decode("latin-1")
                    elif sub == b"DATA" and len(b) >= 12:
                        pos = struct.unpack_from("<fff", b)
                if edid or k in wanted:
                    self.refs[k] = dict(edid=edid, cell=st["cell"], pos=pos)
                    if edid:
                        self.ref_by_edid.setdefault(edid.lower(), k)

        for label, s, e in esm.top:
            if label in (b"CELL", b"WRLD"):
                interesting_walk(esm, s, e, st, on_rec)


MOVETO = re.compile(r"\bmoveto\s+([A-Za-z_][A-Za-z0-9_]*)", re.I)


def moveto_targets(text):
    return [m.group(1) for m in MOVETO.finditer(re.sub(r";[^\n]*", "", text))]


def scan_quest(w, q):
    """Return dict with objective refs, script move targets (edid tokens) and the stages that move."""
    out = dict(targets=[], moves=[], move_stages={})
    for k in q["targets"]:
        out["targets"].append(k)
    texts = []
    if q["script"] and q["script"] in w.scripts:
        texts.append((None, w.scripts[q["script"]][1]))
    for n, st in q["stages"].items():
        texts.append((n, "\n".join(st["text"])))
    for n, text in texts:
        for tok in moveto_targets(text):
            out["moves"].append((n, tok))
    return out


def resolve_token(w, tok):
    """A MoveTo token is a reference EDID (marker) or occasionally a cell-less alias; return ref key or None."""
    return w.ref_by_edid.get(tok.lower())


def cell_edid(w, key):
    c = w.cells.get(key)
    return c["edid"] if c and c["interior"] and c["edid"] else None


def derive(w, edid):
    q = w.quests.get(edid)
    if q is None:
        return None, "quest record not found"
    sc = scan_quest(w, q)
    cells, anchors, markers, stage_cells = [], [], [], {}
    moved_markers = []
    for n, tok in sc["moves"]:
        rk = resolve_token(w, tok)
        if rk is None:
            continue
        r = w.refs[rk]
        moved_markers.append(r["edid"])
        ce = cell_edid(w, r["cell"]) if r["cell"] else None
        if ce:
            stage_cells.setdefault(ce, set())
            if n is not None:
                stage_cells[ce].add(n)
        elif r["edid"]:
            anchors.append(r["edid"])
    target_cells, target_anchors, target_markers = [], [], []
    for k in sc["targets"]:
        r = w.refs.get(k)
        if not r:
            continue
        ce = cell_edid(w, r["cell"]) if r["cell"] else None
        if ce:
            target_cells.append(ce)
        if r["edid"]:
            target_anchors.append(r["edid"])
            if "marker" in r["edid"].lower():
                target_markers.append(r["edid"])
    info = dict(
        quest=edid,
        cells=sorted(set(target_cells) | set(stage_cells)),
        script_cells=sorted(stage_cells),
        target_cells=sorted(set(target_cells)),
        stage_cells={c: sorted(s) for c, s in stage_cells.items()},
        anchors=target_anchors,
        move_anchors=anchors,
        markers=sorted(set(moved_markers) | set(target_markers)),
        n_targets=len(sc["targets"]),
        n_moves=len(sc["moves"]),
    )
    return info, None


def tether_for(info):
    t = OrderedDict()
    if len(info["cells"]) > MAX_CELLS:
        return None
    if info["cells"]:
        t["cells"] = info["cells"]
        if not info["target_cells"]:
            st = sorted(set(s for c in info["script_cells"] for s in info["stage_cells"][c]))
            if st:
                t["stages"] = st
    else:
        cands = info["anchors"] + info["move_anchors"]
        if not cands:
            return None
        t["anchor"] = cands[0]
        t["radius"] = DEFAULT_RADIUS
    t["gather"] = "wait"
    return t


def write_policy(path, obj_head, rows):
    with open(path, "w", encoding="utf8", newline="\n") as f:
        f.write("{\n")
        for k in ("schema", "policy", "shadow_classes", "groups"):
            f.write(' "%s": %s,\n' % (k, json.dumps(obj_head[k], separators=(",", ":"))))
        f.write(' "quests": [\n')
        for i, r in enumerate(rows):
            f.write("  " + json.dumps(r, separators=(",", ":"), ensure_ascii=True) + ("," if i + 1 < len(rows) else "") + "\n")
        f.write(" ]\n}\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--pending", required=True, help="old rules.pending.json (read-only)")
    ap.add_argument("--data", required=True, help="directory holding the game masters")
    ap.add_argument("--policy", default="crates/coop-policy/data/policy.json")
    ap.add_argument("--check", action="store_true", help="report only, do not write")
    a = ap.parse_args()

    with open(a.pending, encoding="utf8") as f:
        pend = json.load(f)
    with open(a.policy, encoding="utf8") as f:
        pol = json.load(f, object_pairs_hook=OrderedDict)
    rows = pol["quests"]
    by = {r["edid"]: r for r in rows}

    esms = [Esm(os.path.join(a.data, p)) for p in PLUGINS if os.path.exists(os.path.join(a.data, p))]
    w = World()
    w.load(esms)

    report = []
    changed = 0
    for kind, names in (("tether", pend["tethers_dropped"]), ("warp", pend["warps_dropped"])):
        for edid in sorted(set(names)):
            row = by.get(edid)
            if row is None:
                report.append((kind, edid, "no such row in the policy table"))
                continue
            if row.get(kind) is not None:
                report.append((kind, edid, "already present, kept: %s" % json.dumps(row[kind], separators=(",", ":"))))
                continue
            info, err = derive(w, edid)
            if err:
                report.append((kind, edid, "UNRESOLVED: " + err))
                continue
            if kind == "tether":
                t = tether_for(info)
                if t is None:
                    report.append((kind, edid, "UNRESOLVED: %d targets, %d MoveTo calls, %s"
                                   % (info["n_targets"], info["n_moves"],
                                      "objectives span %d interior cells, no single scene place" % len(info["cells"])
                                      if len(info["cells"]) > MAX_CELLS else "no interior cell and no named anchor")))
                    continue
                row["tether"] = t
                report.append((kind, edid, "cells=%s anchor=%s stages=%s" % (t.get("cells"), t.get("anchor"), t.get("stages"))))
            else:
                if not info["markers"]:
                    report.append((kind, edid, "UNRESOLVED: %d targets, %d MoveTo calls, no named marker reference"
                                   % (info["n_targets"], info["n_moves"])))
                    continue
                row["warp"] = OrderedDict(markers=info["markers"])
                report.append((kind, edid, "markers=%s" % info["markers"]))
            changed += 1

    print("%-8s %-34s %s" % ("gate", "quest", "result"))
    for kind, edid, msg in report:
        print("%-8s %-34s %s" % (kind, edid, msg))
    bad = sum(1 for r in report if "UNRESOLVED" in r[2])
    print("filled %d, unresolved %d, total %d" % (changed, bad, len(report)))

    if not a.check:
        write_policy(a.policy, pol, rows)


if __name__ == "__main__":
    main()
