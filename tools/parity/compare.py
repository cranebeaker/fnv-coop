#!/usr/bin/env python
"""Offline half of the live sync parity checker (docs/tools/parity.md): compares the two nvcoop_parity.jsonl files of one co-op run.

    python -I tools/parity/compare.py HOST_FILE CLIENT_FILE          the two files, in any order (the "role" field of a line says which is which)
    python -I tools/parity/compare.py RUN_FOLDER                     a folder holding parity_host.jsonl and parity_client.jsonl (the benchmark saves them)
        --json            print the result as JSON (what tools/bench reads)
        --top N           worst offenders to list (default 12)
        --net-cap N       cut the guest's actor list to its N nearest, like the datagram does in game (default 30; 0 = the whole file sample)
        --rotated         also read <file>.1 (an older part of a long run; without it only the current file is used, because <file>.1 is the PREVIOUS game run after a restart)
        --tol KEY=VALUE   pos_tol, pos_hard, lag_s, skew_s, persist_ms, hp_gap, acts_gap

The host writes, besides its own tick samples, one "cmp" sample the moment each guest sample arrives, naming that guest sample (cmp.slot, cmp.seq): the pairs are
exact and need no clock agreement between the two machines. Without such links (an older file) samples are paired by wall clock within 1.5 s.
The rules are the ones plugin/parity/parity_compare.cpp applies in game (tools/parity/vectors.json pins the two implementations to the same answers).
"""
from __future__ import annotations

import argparse
import json
import math
import os
import sys
from pathlib import Path

NO_GROUP = 255

# Twins for tests (tools/parity/run_tests.py): PARITY_MUTANT=<name> breaks one rule on purpose
_MUT = os.environ.get("PARITY_MUTANT", "")


def mut(name):
    return _MUT == name


class Tol:
    def __init__(self, **kw):
        self.pos_tol = 210.0
        self.pos_hard = 1400.0
        self.lag_s = 0.9
        self.skew_s = 0.15
        self.persist_ms = 3000
        self.stale_ms = 12000
        self.hp_gap = 2
        self.acts_gap = 2
        for k, v in kw.items():
            if not hasattr(self, k):
                raise SystemExit("unknown tolerance " + k)
            setattr(self, k, type(getattr(self, k))(v))


# ---- animation classes, health, tolerance (mirrors plugin/parity/parity_anim.h) ------------------------------------------------------------------------------

def group_class(g):
    if g is None or g < 0 or g > 255 or g == NO_GROUP:
        return "unknown"
    if g == 224:
        return "death"
    if 3 <= g <= 6:
        return "walk"
    if 7 <= g <= 10:
        return "run"
    if 11 <= g <= 16 or 23 <= g <= 25 or 26 <= g <= 223 or 227 <= g <= 244:
        return "transient"   # dodge, turn, draw / holster, every attack / reload / jam / reaction, jumps
    return "idle"            # idle, dynamic idle, special idle, aim, talking and anything that is not movement


def primary_group(groups):
    death = transient = run = walk = idle = -1
    for g in groups:
        if g < 0 or g > 244:
            continue
        c = group_class(g)
        if c == "death":
            death = g
        elif c == "transient":
            transient = g if transient < 0 else transient
        elif c == "run":
            run = g if run < 0 else run
        elif c == "walk":
            walk = g if walk < 0 else walk
        elif c == "idle":
            idle = g if idle < 0 else idle
    if mut("walk_over_run"):
        loco = walk if walk >= 0 else run
    else:
        loco = run if run >= 0 else walk
    pick = death if death >= 0 else transient if transient >= 0 else loco if loco >= 0 else idle
    return NO_GROUP if pick < 0 else pick


def hp_bucket(hp):
    if hp is None:
        return NO_GROUP
    if hp <= 0:
        return 0
    return min(20, 1 + int(hp / 25.0))


def pos_tolerance(t, a16, b16):
    if mut("ignore_speed"):
        return t.pos_tol
    speed = (a16 + b16) * 16.0 / 2.0
    return t.pos_tol + speed * (t.skew_s + t.lag_s)


# ---- samples -----------------------------------------------------------------------------------------------------------------------------------------------

def _hex(v, d=0):
    if isinstance(v, int):
        return v
    try:
        return int(v, 16)
    except (TypeError, ValueError):
        return d


class Ent:
    __slots__ = ("kind", "slot", "ref", "x", "y", "z", "speed16", "seated", "dead", "disabled", "hp", "group", "acts", "look", "name")

    def pos(self):
        return (self.x, self.y, self.z)


def parse_ent(e):
    x = Ent()
    x.kind = e.get("t", "n")
    x.slot = e.get("sl", 255) if x.kind != "n" else 255
    x.ref = _hex(e.get("r", 0))
    x.x, x.y, x.z = int(e.get("x", 0)), int(e.get("y", 0)), int(e.get("z", 0))
    x.speed16 = int(e.get("sp", 0)) // 16
    x.seated = bool(e.get("se", 0))
    x.dead = bool(e.get("d", 0))
    x.disabled = bool(e.get("dis", 0))
    hp = e.get("hp", -1)
    x.hp = None if hp is None or hp < 0 or hp == 255 else int(hp)
    g = e.get("g", -1)
    x.group = None if g is None or g < 0 or g == NO_GROUP else int(g)
    x.acts = int(e.get("ac", 0))
    lk = e.get("look")
    x.look = (_hex(lk.get("race")), int(lk.get("f", 0)), _hex(lk.get("hair")), _hex(lk.get("eyes")), _hex(lk.get("fh"))) if lk else None
    x.name = e.get("nm", "")
    return x


class Sample:
    def __init__(self, o):
        self.raw = o
        self.slot = o.get("slot", 255)
        self.seq = o.get("seq", 0)
        self.ms = o.get("ms", 0)
        self.wall = o.get("wall", 0)
        self.cell = _hex(o.get("cell", 0))
        self.ws = _hex(o.get("ws", 0))
        self.cov = o.get("cov", 0)
        self.menu = bool(o.get("menu", 0))
        self.role = o.get("role", "")
        self.why = o.get("why", "tick")
        self.cmp = o.get("cmp")
        self.ents = [parse_ent(e) for e in o.get("e", [])]

    def by_slot(self, s):
        for e in self.ents:
            if e.kind != "n" and e.slot == s:
                return e
        return None

    def by_ref(self, r):
        for e in self.ents:
            if e.kind == "n" and e.ref == r:
                return e
        return None


def net_sample(s, cap):
    """The part of a guest sample that fits the datagram (plugin/parity/parity_sampler.cpp netSample)."""
    if cap <= 0 or mut("no_net_cap"):
        return s
    o = Sample.__new__(Sample)
    o.__dict__.update(s.__dict__)
    me = s.by_slot(s.slot)
    ents, npcs, cut = [], 0, False
    for e in s.ents:
        if e.kind != "n":
            continue
        if npcs >= cap:
            if not cut:
                o.cov = min(o.cov, int(dist3(e, me))) if me else 0
            cut = True
            continue
        npcs += 1
        ents.append(e)
    ents += [e for e in s.ents if e.kind != "n"]
    o.ents = ents
    return o


def dist3(a, b):
    return math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2 + (a.z - b.z) ** 2)


def same_area(a, b):
    if a.ws != 0 or b.ws != 0:
        return a.ws == b.ws
    return a.cell == b.cell


# ---- the comparison (mirrors plugin/parity/parity_compare.cpp) --------------------------------------------------------------------------------------------------

class Result:
    def __init__(self):
        self.confirmed = []   # (kind, key, host_text, client_text)
        self.refs = self.refs_bad = self.players = self.players_bad = 0
        self.keys_compared = []
        self.keys_bad = []
        self.skipped = False
        self.skip_reason = ""

    def ids(self):
        return sorted("%s|%s" % (k, key) for k, key, _, _ in self.confirmed)


def _at(e):
    return "(%d,%d,%d)" % (e.x, e.y, e.z)


def _group_text(g):
    return "none" if g is None else "%s[%d]" % (group_class(g), g)


def _look_text(l):
    if l is None:
        return "unknown"
    return "race=0x%08X sex=%s hair=0x%08X eyes=0x%08X face=%08X" % (l[0], "F" if l[1] else "M", l[2], l[3], l[4])


class Comparer:
    def __init__(self, tol=None):
        self.tol = tol or Tol()
        self.pending = {}
        self.acts = {}
        self.kinds = {}   # kind -> [checks, bad]

    def _observe(self, ident, bad, hard, now):
        if not bad:
            if not mut("keep_pending"):
                self.pending.pop(ident, None)
            return False
        p = self.pending.get(ident)
        if p is None:
            p = self.pending[ident] = [now, now]
        else:
            p[1] = now
        if mut("no_persistence"):
            return True
        return hard or now - p[0] >= self.tol.persist_ms

    def _check(self, r, anybad, kind, key, bad, hard, now, host, client):
        kc = self.kinds.setdefault(kind, [0, 0])
        kc[0] += 1
        if self._observe(key + "|" + kind, bad, hard, now):
            kc[1] += 1
            r.confirmed.append((kind, key, host, client))
            return True
        return anybad

    def _pair(self, r, h, c, key, players, now):
        t = self.tol
        bad_any = False
        self.pending.pop(key + "|absent", None)
        bad = h.disabled != c.disabled
        bad_any = self._check(r, bad_any, "disabled", key, bad, False, now, ("disabled" if h.disabled else "enabled") if bad else "", ("disabled" if c.disabled else "enabled") if bad else "")
        if not h.disabled and not c.disabled:
            bad = h.dead != c.dead
            bad_any = self._check(r, bad_any, "dead", key, bad, False, now, ("dead" if h.dead else "alive") if bad else "", ("dead" if c.dead else "alive") if bad else "")
            if not h.dead and not c.dead:
                d = dist3(h, c)
                tol = pos_tolerance(t, h.speed16, c.speed16)
                bad = d > tol
                bad_any = self._check(r, bad_any, "pos", key, bad, d > t.pos_hard, now, _at(h) if bad else "", (_at(c) + " off=%.0f tol=%.0f" % (d, tol)) if bad else "")
                sb = h.seated != c.seated
                bad_any = self._check(r, bad_any, "seated", key, sb, False, now, ("seated" if h.seated else "standing") if sb else "", ("seated" if c.seated else "standing") if sb else "")
                hc, cc = group_class(h.group), group_class(c.group)
                if hc != "unknown" and cc != "unknown" and hc != "transient" and cc != "transient":
                    b2 = hc != cc
                    bad_any = self._check(r, bad_any, "anim", key, b2, False, now, _group_text(h.group) if b2 else "", _group_text(c.group) if b2 else "")
                if h.hp is not None and c.hp is not None:
                    b3 = abs(h.hp - c.hp) >= t.hp_gap
                    bad_any = self._check(r, bad_any, "hp", key, b3, False, now, "bucket %d" % h.hp if b3 else "", "bucket %d" % c.hp if b3 else "")
        if players:
            self_e = h if h.kind == "s" else c if c.kind == "s" else None
            copy = h if h.kind == "p" else c if c.kind == "p" else None
            if self_e is not None and copy is not None and self_e.look is not None:
                a, b = self_e.look, copy.look
                differs = b is None or a[:4] != b[:4]
                if not mut("ignore_face_hash") and b is not None and a[4] != b[4]:
                    differs = True
                bad_any = self._check(r, bad_any, "look", key, differs, False, now,
                                      (_look_text(a) if h is self_e else _look_text(b)) if differs else "", (_look_text(a) if c is self_e else _look_text(b)) if differs else "")
            if h.kind != "n" and c.kind != "n":
                st = self.acts.get(h.slot)
                if st is None or h.acts < st["h"] or c.acts < st["c"]:
                    st = self.acts[h.slot] = {"base": h.acts - c.acts, "h": h.acts, "c": c.acts}
                st["h"], st["c"] = h.acts, c.acts
                dev = (h.acts - c.acts) - st["base"]
                bad = abs(dev) >= t.acts_gap
                bad_any = self._check(r, bad_any, "action", key, bad, False, now, "%d attacks" % h.acts if bad else "", "%d attacks (%+d apart)" % (c.acts, -dev) if bad else "")
        r.refs += 1
        r.keys_compared.append(key)
        if players:
            r.players += 1
        if bad_any:
            r.refs_bad += 1
            r.keys_bad.append(key)
            if players:
                r.players_bad += 1

    def compare(self, host, client, now):
        r = Result()
        for k in [k for k, v in self.pending.items() if now - v[1] > self.tol.stale_ms]:
            del self.pending[k]
        if host.menu or client.menu:
            r.skipped, r.skip_reason = True, "a menu is open"
            return r
        if not same_area(host, client):
            r.skipped, r.skip_reason = True, "the players are in different places"
            return r
        hp = host.by_slot(host.slot)
        cp = client.by_slot(client.slot)
        for h in host.ents:
            if h.kind != "n" or (h.ref >> 24) == 0xFF:
                continue
            c = client.by_ref(h.ref)
            key = "0x%08X" % h.ref
            if c is not None:
                self._pair(r, h, c, key, False, now)
            elif cp is not None and not h.disabled:
                if mut("absent_ignores_coverage") or dist3(h, cp) <= client.cov:
                    any_ = self._check(r, False, "absent", key, True, False, now, _at(h), "missing")
                    r.refs += 1
                    r.keys_compared.append(key)
                    if any_:
                        r.refs_bad += 1
                        r.keys_bad.append(key)
        for c in client.ents:
            if c.kind != "n" or (c.ref >> 24) == 0xFF or host.by_ref(c.ref) is not None or c.disabled or hp is None:
                continue
            if not (mut("absent_ignores_coverage") or dist3(c, hp) <= host.cov):
                continue
            key = "0x%08X" % c.ref
            any_ = self._check(r, False, "absent", key, True, False, now, "missing", _at(c))
            r.refs += 1
            r.keys_compared.append(key)
            if any_:
                r.refs_bad += 1
                r.keys_bad.append(key)
        slots = sorted({e.slot for e in host.ents if e.kind != "n"} | {e.slot for e in client.ents if e.kind != "n"})
        for s in slots:
            h, c = host.by_slot(s), client.by_slot(s)
            key = "player%d" % s
            if h is not None and c is not None:
                self._pair(r, h, c, key, True, now)
            else:
                any_ = self._check(r, False, "absent", key, True, False, now, _at(h) if h else "missing", _at(c) if c else "missing")
                r.refs += 1
                r.players += 1
                r.keys_compared.append(key)
                if any_:
                    r.refs_bad += 1
                    r.players_bad += 1
                    r.keys_bad.append(key)
        return r


# ---- files and pairing -------------------------------------------------------------------------------------------------------------------------------------

def load_samples(path, rotated=False):
    paths = []
    if rotated and Path(str(path) + ".1").is_file():
        paths.append(Path(str(path) + ".1"))
    paths.append(Path(path))
    out, bad = [], 0
    for p in paths:
        with open(p, encoding="utf-8", errors="replace") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    o = json.loads(line)
                except ValueError:
                    bad += 1   # a line cut short by a crash
                    continue
                if o.get("k") == "s":
                    out.append(Sample(o))
    return out, bad


def pair_by_link(host_samples, client_samples):
    if mut("link_ignored"):
        return []
    by_seq = {(s.slot, s.seq): s for s in client_samples if s.role != "host"}
    pairs = []
    for h in host_samples:
        if h.why == "cmp" and h.cmp:
            c = by_seq.get((h.cmp.get("slot"), h.cmp.get("seq")))
            if c is not None:
                pairs.append((h, c))
    return pairs


def pair_by_wall(host_samples, client_samples, window_ms=1500):
    pairs = []
    cs = sorted(client_samples, key=lambda s: s.wall)
    for h in host_samples:
        if h.why == "cmp" or not h.wall:
            continue
        best = min(cs, key=lambda c: abs(c.wall - h.wall), default=None)
        if best is not None and abs(best.wall - h.wall) <= window_ms:
            pairs.append((h, best))
    return pairs


def analyse(host_samples, client_samples, tol=None, net_cap=30, top=12):
    tol = tol or Tol()
    pairs = pair_by_link(host_samples, client_samples)
    paired_by = "link"
    if not pairs:
        pairs = pair_by_wall(host_samples, client_samples)
        paired_by = "wall clock" if pairs else "none"
    cmps = {}
    seen = {}
    conf = {}
    example = {}
    first_last = {}
    totals = {"rounds": 0, "skipped": 0, "refs": 0, "refs_bad": 0, "players": 0, "players_bad": 0}
    skip_reasons = {}
    t0 = pairs[0][0].ms if pairs else 0
    for h, c in pairs:
        cmp = cmps.setdefault(c.slot, Comparer(tol))
        r = cmp.compare(h, net_sample(c, net_cap), h.ms)
        totals["rounds"] += 1
        if r.skipped:
            totals["skipped"] += 1
            skip_reasons[r.skip_reason] = skip_reasons.get(r.skip_reason, 0) + 1
            continue
        totals["refs"] += r.refs
        totals["refs_bad"] += r.refs_bad
        totals["players"] += r.players
        totals["players_bad"] += r.players_bad
        t = (h.ms - t0) / 1000.0
        for k in r.keys_compared:
            seen[k] = seen.get(k, 0) + 1
        for kind, key, ht, ct in r.confirmed:
            conf[(key, kind)] = conf.get((key, kind), 0) + 1
            example.setdefault((key, kind), (ht, ct))
            fl = first_last.setdefault((key, kind), [t, t])
            fl[1] = t
    kinds = {}
    for cmp in cmps.values():
        for k, (n, b) in cmp.kinds.items():
            e = kinds.setdefault(k, [0, 0])
            e[0] += n
            e[1] += b
    offenders = sorted(conf.items(), key=lambda kv: (-kv[1], kv[0]))[:top]
    npc_refs = totals["refs"] - totals["players"]
    npc_bad = totals["refs_bad"] - totals["players_bad"]
    res = {
        "paired_by": paired_by,
        "pairs": len(pairs),
        "span_s": round((pairs[-1][0].ms - t0) / 1000.0, 1) if pairs else 0,
        "rounds": totals["rounds"], "skipped": totals["skipped"], "skip_reasons": skip_reasons,
        "refs": totals["refs"], "refs_bad": totals["refs_bad"],
        "npc_refs": npc_refs, "npc_bad": npc_bad, "player_refs": totals["players"], "player_bad": totals["players_bad"],
        "mismatch_pct": round(100.0 * totals["refs_bad"] / totals["refs"], 2) if totals["refs"] else None,
        "npc_mismatch_pct": round(100.0 * npc_bad / npc_refs, 2) if npc_refs else None,
        "puppet_mismatch_pct": round(100.0 * totals["players_bad"] / totals["players"], 2) if totals["players"] else None,
        "kinds": {k: {"checks": n, "bad": b, "rate_pct": round(100.0 * b / n, 2) if n else None} for k, (n, b) in sorted(kinds.items())},
        "worst": [{"key": key, "kind": kind, "rounds_bad": n, "rounds_seen": seen.get(key, 0), "first_s": round(first_last[(key, kind)][0], 1),
                   "last_s": round(first_last[(key, kind)][1], 1), "host": example[(key, kind)][0], "client": example[(key, kind)][1]} for (key, kind), n in offenders],
    }
    return res


def format_report(res, host_label="host", client_label="client"):
    L = ["Parity report", "  host   : %s" % host_label, "  client : %s" % client_label,
         "  pairing: %s, %d pairs over %.0f s of host time; %d rounds compared, %d skipped%s" % (
             res["paired_by"], res["pairs"], res["span_s"], res["rounds"] - res["skipped"], res["skipped"],
             " (" + ", ".join("%s: %d" % kv for kv in res["skip_reasons"].items()) + ")" if res["skip_reasons"] else "")]
    if not res["refs"]:
        L.append("  nothing to compare: no paired samples (is [parity] enabled=1 on both copies, and did the guest join?)")
        return "\n".join(L)
    L.append("Overall: %d references compared, %d mismatching (%.1f %%)  [actors %d / %d (%s %%), players %d / %d (%s %%)]" % (
        res["refs"], res["refs_bad"], res["mismatch_pct"], res["npc_refs"], res["npc_bad"], res["npc_mismatch_pct"], res["player_refs"], res["player_bad"], res["puppet_mismatch_pct"]))
    L += ["", "%-10s %8s %8s %8s" % ("kind", "checks", "bad", "rate")]
    for k, v in res["kinds"].items():
        L.append("%-10s %8d %8d %7s%%" % (k, v["checks"], v["bad"], v["rate_pct"]))
    if res["worst"]:
        L += ["", "Worst offenders (confirmed in rounds / rounds the reference was compared):"]
        for w in res["worst"]:
            L.append("  %-12s %-9s %3d/%-3d  %6.0fs..%.0fs  host=%s client=%s" % (w["key"], w["kind"], w["rounds_bad"], w["rounds_seen"], w["first_s"], w["last_s"], w["host"], w["client"]))
    return "\n".join(L)


def bench_metrics(host_path, client_path, rotated=False):
    """What tools/bench stores under extras["parity"]: None when there is nothing to compare."""
    try:
        h, _ = load_samples(host_path, rotated)
        c, _ = load_samples(client_path, rotated)
    except OSError:
        return None
    if any(s.role == "guest" for s in h) and not any(s.role == "guest" for s in c):
        h, c = c, h
    res = analyse(h, c)
    if res["rounds"] - res["skipped"] < 3 or not res["refs"]:
        return None
    return res


def _pick_files(args):
    if len(args) == 1 and Path(args[0]).is_dir():
        d = Path(args[0])
        return d / "parity_host.jsonl", d / "parity_client.jsonl"
    if len(args) == 2:
        return Path(args[0]), Path(args[1])
    raise SystemExit("give the two nvcoop_parity.jsonl files, or one run folder with parity_host.jsonl and parity_client.jsonl")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("files", nargs="+")
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--top", type=int, default=12)
    ap.add_argument("--net-cap", type=int, default=30)
    ap.add_argument("--rotated", action="store_true")
    ap.add_argument("--tol", action="append", default=[], metavar="KEY=VALUE")
    a = ap.parse_args(argv)
    a_path, b_path = _pick_files(a.files)
    for p in (a_path, b_path):
        if not p.is_file():
            print("not found:", p, file=sys.stderr)
            return 2
    tol = Tol(**dict(kv.split("=", 1) for kv in a.tol))
    sa, bad_a = load_samples(a_path, a.rotated)
    sb, bad_b = load_samples(b_path, a.rotated)
    host, client, hp, cp = sa, sb, a_path, b_path
    host_is_a = any(s.role == "host" for s in sa)
    host_is_b = any(s.role == "host" for s in sb)
    if host_is_b and not host_is_a:
        host, client, hp, cp = sb, sa, b_path, a_path
    elif not host_is_a and not host_is_b:
        print("neither file has host samples (role host): were both games started with [parity] enabled=1 and a session?", file=sys.stderr)
        return 2
    res = analyse(host, client, tol, a.net_cap, a.top)
    res["bad_lines"] = bad_a + bad_b
    if a.json:
        print(json.dumps(res, indent=1))
    else:
        print(format_report(res, str(hp), str(cp)))
        if res["bad_lines"]:
            print("(%d unreadable lines skipped)" % res["bad_lines"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
