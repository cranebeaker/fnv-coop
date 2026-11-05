#!/usr/bin/env python3
"""Sync benchmark on two real viewers: how close and how fast the two machines agree.

    python -I tools/coop-run/bench.py run    [--set full] [--profiles perfect,good_wifi,bad_wifi]
                                             [--n 1] [--hz 5] [--only NAME ...] [--out DIR]
    python -I tools/coop-run/bench.py report BENCH_DIR

`run` runs each scenario of the set once per network profile and repeat. It uses the runner's
own launch, chaos relay and verdict. The profile replaces the scenario's own profile. During each
run a sampler asks both viewers for their state on the live control port (--coop-control),
`hz` times a second and at nearly the same moment. It writes samples.jsonl in the run folder.
`report` reads the run folders again and writes bench.json and bench.md in BENCH_DIR.

Metrics (per run, then per category and profile):
- verdict: the scenario's own checks (the runner's verdict).
- npc_pos: the distance in game units between the owner's NPC or creature and the copy on the
  other machine. "raw" compares the same moment. "lag" is the time shift (0 to 1000 ms) that
  makes the copy closest to the owner, and "comp" is the distance after that shift.
- avatar_pos: the same for each player: the owner's own position and the puppet on the other
  machine.
- anim: how often a copy's animation bits (walking, running, sneaking, fighting) equal the
  owner's bits in the lag window. For avatars the owner's "moving" comes from its speed.
- presence: how often the other machine has a copy of a tracked actor that the owner shows,
  while both players are in the same cell.
- ownership: actor samples that both machines claim (a conflict) or that no machine claims.
- state: for each shared value (quest stages, player quest rows, deaths, dialogue scene, globals,
  factions, topics, map markers, combat targets ...), the time from a change on one machine to
  the same value on the other machine. A value that never arrives counts as "lost".
- hash: how long the host has had a shared-state revision that the guest does not have yet, in
  ms, and samples where the guest's hash differs from the host's at the same revision
  ("divergent").
- net: relay bytes and packets a second each way, drops, RTT, desyncs, repairs, decode errors.
- fps: each viewer's frame rate, and the time a control reply takes (about one frame).

The sampler reads only. It changes nothing in the game. Python 3 standard library only.
"""
import argparse
import bisect
import json
import math
import os
import socket
import statistics
import sys
import threading
import time

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

import live as live_mod  # noqa: E402
import runner as runner_mod  # noqa: E402
import scenario as scn_mod  # noqa: E402

# The scenarios of each set, per category. Each one is an existing runner scenario.
SETS = {
    "full": {
        "avatars": ["live_avatars_visible", "bench_party_fight"],
        "npcs": ["live_npc_targets_players", "live_guest_takes_moved_actor",
                 "live_extra_enemies_scaling", "live_door_follow"],
        "combat": ["live_guest_kills_npc", "live_npc_attacks_remote_player", "live_friendly_fire_guard",
                   "live_guest_hits_essential", "live_explosion_area", "live_downed_revive_from_npc",
                   "live_kill_xp_attribution", "live_vats_no_freeze"],
        "dialogue": ["live_dialogue_session", "live_dialogue_forced_line", "live_scene_forced_dialogue",
                     "live_dialogue_pause_real", "live_chat_roundtrip"],
        "quests": ["doc_stage_sync", "doc_player_quest_sync", "live_fork_consent",
                   "story_topics_notes", "story_map_markers"],
    },
    "quick": {
        "avatars": ["bench_party_fight"],
        "npcs": ["live_npc_targets_players"],
        "combat": ["live_guest_kills_npc"],
        "dialogue": ["live_dialogue_forced_line"],
        "quests": ["doc_stage_sync"],
    },
}
DEFAULT_PROFILES = ("perfect", "good_wifi", "bad_wifi")
ROLE_PLAYER = {"host": 0, "guest": 1}
CLOCK_GLOBALS = ("GameHour", "GameDaysPassed")
# Dump keys that hold shared values. The others are per machine (counters, positions, status).
SHARED_TOP = ("quests", "player_quests", "dead", "disabled", "globals", "topics", "notes", "map_markers",
              "karma", "reputations", "factions", "faction_relations", "objects", "time")
# `scene.active` counts the scenes a machine knows. `id`, `phase` and `kind` are the machine's own main
# scene: a player outside a scene has none, so they are not shared values.
SCENE_KEYS = ("active",)
# `campaign.down` and the other campaign rows are the host's own bookkeeping (the scenarios check
# them on the host only). The xp rows are mirrored.
CAMPAIGN_KEYS = ("xp",)
STORE_ALWAYS = ("positions", "actors", "hash", "status")
SKIP_TOP = ("seq", "t", "role", "live")
LAG_MAX_MS = 1000
LAG_STEP_MS = 20
MOVE_MIN = 120.0  # an owner track shorter than this (game units) gives no lag estimate
MOVING_SPEED = 40.0  # units a second: an avatar faster than this is moving
# A walk step spans at least this long: two samples close in time make a small settle look fast.
MIN_STEP_S = 0.15
# A run's avatar walk agreement is a finding only with at least this many samples.
MIN_WALK_SAMPLES = 5
END_GRACE_S = 2.0  # a change this close to the end that did not arrive is "late", not "lost"


# ---- sampling during a run -----------------------------------------------------------------------


def free_tcp_port(start):
    for p in range(start, start + 400):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            s.bind(("127.0.0.1", p))
            return p
        except OSError:
            continue
        finally:
            s.close()
    raise RuntimeError("no free TCP port from %d" % start)


class Sampler(threading.Thread):
    """Asks every peer for `state` at the same moment, `hz` times a second, until stop().
    Writes one line per peer and tick to `path`. Big dump parts go in only when they change."""

    def __init__(self, ports, path, run_dir, hz):
        super().__init__(daemon=True)
        self.peers = {r: live_mod.Peer(r, p, timeout=3) for r, p in ports.items()}
        self.path = path
        self.run_dir = run_dir
        self.dt = 1.0 / hz
        self.halt = threading.Event()
        self.last = {r: {} for r in ports}
        self.k = 0
        self.n_ok = 0
        self.n_err = 0

    def stop(self):
        self.halt.set()
        self.join(timeout=10)

    def _ask(self, role, out):
        """`state` (the dump, no entity rows) and then `entities` (the rows), one frame apart."""
        w0 = time.perf_counter()
        try:
            r = self.peers[role].call(cmd="state", check=False, timeout=3)
            w1 = time.perf_counter()
            e = self.peers[role].call(cmd="entities", check=False, timeout=3)
            w2 = time.perf_counter()
        except live_mod.LiveError:
            return
        r["rows"] = e.get("entities") or []
        r["rows_w"] = (w1 + w2) / 2
        out[role] = (w0, w1, r)

    def sample(self, role, w0, w1, r, go):
        dump = r.get("state") or {}
        g = dump.get("globals") or {}
        changed = {}
        for key, val in dump.items():
            if key in SKIP_TOP:
                continue
            if key == "globals" and isinstance(val, dict):
                val = {k: v for k, v in val.items() if k not in CLOCK_GLOBALS}
            if key in STORE_ALWAYS:
                changed[key] = val
                continue
            txt = json.dumps(val, sort_keys=True)
            if self.last[role].get(key) != txt:
                self.last[role][key] = txt
                changed[key] = val
        return {"k": self.k, "role": role, "w": (w0 + w1) / 2, "ask_ms": round((w1 - w0) * 1000, 2),
                "t": r.get("t"), "fps": r.get("fps"), "cell": r.get("cell"), "go": go,
                "clock": [g.get("GameDaysPassed"), g.get("GameHour")],
                "counters": r.get("counters") or {}, "rows": r.get("rows") or [],
                "rw": r.get("rows_w", (w0 + w1) / 2), "d": changed}

    def run(self):
        go_file = os.path.join(self.run_dir, "host.go")
        with open(self.path, "w", encoding="utf-8") as fh:
            nxt = time.monotonic()
            while not self.halt.is_set():
                out = {}
                ts = [threading.Thread(target=self._ask, args=(r, out)) for r in self.peers]
                for t in ts:
                    t.start()
                for t in ts:
                    t.join()
                go = os.path.exists(go_file)
                for role in self.peers:
                    got = out.get(role)
                    if not got:
                        self.n_err += 1
                        continue
                    w0, w1, r = got
                    if not r.get("ok") or not r.get("ready"):
                        continue
                    fh.write(json.dumps(self.sample(role, w0, w1, r, go)) + "\n")
                    self.n_ok += 1
                fh.flush()
                self.k += 1
                nxt += self.dt
                delay = nxt - time.monotonic()
                if delay > 0:
                    self.halt.wait(delay)
                else:
                    nxt = time.monotonic()


def run_one(scn_path, profile, cfg, run_dir, hz, slot=0, seed=None, log=print):
    """Runs one scenario under `profile` with the sampler. Returns the run's metrics."""
    scn = scn_mod.load(scn_path)
    scn["network"] = profile
    ctl = {}
    base = 28100 + 16 * slot
    for role in ("host", "guest"):
        ctl[role] = free_tcp_port(base)
        base = ctl[role] + 1
        scn[role]["extra_args"] = list(scn[role]["extra_args"]) + ["--coop-control", str(ctl[role])]
    os.makedirs(run_dir, exist_ok=True)
    samples = os.path.join(run_dir, "samples.jsonl")
    sampler = Sampler(ctl, samples, run_dir, hz)
    sampler.start()
    try:
        runner_mod.run_scenario(scn, cfg, run_dir, seed=seed, scn_path=scn_path, slot=slot, log=log)
    finally:
        sampler.stop()
    meta = {"scenario": scn["name"], "profile": profile, "own_profile": scn_mod.load(scn_path)["network"],
            "hz": hz, "samples_ok": sampler.n_ok, "samples_failed": sampler.n_err}
    with open(os.path.join(run_dir, "bench_meta.json"), "w", encoding="utf-8") as fh:
        json.dump(meta, fh, indent=2)
    return analyse_run(run_dir)


# ---- analysis helpers ------------------------------------------------------------------------------


def pct(xs, q):
    """Nearest-rank percentile (q in 0..100) of a list, or None."""
    if not xs:
        return None
    s = sorted(xs)
    i = min(len(s) - 1, max(0, int(math.ceil(q / 100.0 * len(s))) - 1))
    return s[i]


def mean(xs):
    return statistics.fmean(xs) if xs else None


def dist(a, b):
    return math.sqrt(sum((float(x) - float(y)) ** 2 for x, y in zip(a[:3], b[:3])))


class Track:
    """Positions over time, linear in between. at(w) is None outside the sampled span."""

    def __init__(self, pts):
        pts = sorted(pts, key=lambda p: p[0])
        self.ws = [p[0] for p in pts]
        self.ps = [p[1] for p in pts]

    def __len__(self):
        return len(self.ws)

    def at(self, w):
        ws = self.ws
        if not ws or w < ws[0] - 1e-6 or w > ws[-1] + 1e-6:
            return None
        i = bisect.bisect_left(ws, w)
        if i < len(ws) and abs(ws[i] - w) < 1e-9:
            return self.ps[i]
        if i == 0:
            return self.ps[0]
        if i >= len(ws):
            return self.ps[-1]
        w0, w1 = ws[i - 1], ws[i]
        f = (w - w0) / (w1 - w0) if w1 > w0 else 0.0
        a, b = self.ps[i - 1], self.ps[i]
        return [a[j] + (b[j] - a[j]) * f for j in range(3)]

    def path_len(self, w_from, w_to):
        n = 0.0
        for i in range(1, len(self.ws)):
            if self.ws[i - 1] >= w_from and self.ws[i] <= w_to:
                n += dist(self.ps[i - 1], self.ps[i])
        return n

    def speed(self, w, span=0.25):
        a, b = self.at(w - span), self.at(w)
        if a is None or b is None:
            return None
        return math.hypot(b[0] - a[0], b[1] - a[1]) / span


def pos_metrics(owner, replica):
    """owner: Track (the truth), replica: [(w, pos)] on the other machine.
    Returns raw errors, the best lag in ms (None when the owner hardly moved) and the
    errors at that lag."""
    raw = []
    for w, p in replica:
        o = owner.at(w)
        if o is not None:
            raw.append(dist(p, o))
    out = {"n": len(raw), "raw": raw, "lag_ms": None, "comp": raw}
    if not replica or len(raw) < 5:
        return out
    moved = owner.path_len(replica[0][0] - LAG_MAX_MS / 1000.0, replica[-1][0])
    out["moved"] = moved
    if moved < MOVE_MIN:
        return out
    best = None
    for lag in range(0, LAG_MAX_MS + 1, LAG_STEP_MS):
        errs = []
        for w, p in replica:
            o = owner.at(w - lag / 1000.0)
            if o is not None:
                errs.append(dist(p, o))
        if len(errs) < max(5, len(raw) // 2):
            continue
        m = statistics.fmean(errs)
        if best is None or m < best[1] - 1e-9:
            best = (lag, m, errs)
    if best is not None:
        out["lag_ms"] = best[0]
        out["comp"] = best[2]
    return out


ANIM_BITS = {"moving": 0b0011, "sneaking": 0b0100, "fighting": 0b1000}


def anim_agreement(owner_anims, replica_anims, lag_ms, window_s):
    """owner_anims, replica_anims: [(w, bits)]. A replica sample agrees on a bit group when an
    owner sample in [w - lag - window, w - lag + window] has the same bits in that group.
    Returns {group: [agreeing, counted]} over samples where either side shows the group."""
    ows = [a[0] for a in owner_anims]
    out = {g: [0, 0] for g in ANIM_BITS}
    out["all"] = [0, 0]
    shift = (lag_ms or 0) / 1000.0
    for w, bits in replica_anims:
        c = w - shift
        lo = bisect.bisect_left(ows, c - window_s)
        hi = bisect.bisect_right(ows, c + window_s)
        near = [owner_anims[i][1] for i in range(lo, hi)]
        if not near:
            continue
        out["all"][1] += 1
        if bits in near:
            out["all"][0] += 1
        for g, mask in ANIM_BITS.items():
            if not (bits & mask) and not any(n & mask for n in near):
                continue
            out[g][1] += 1
            if any((n & mask) == (bits & mask) for n in near):
                out[g][0] += 1
    return out


TELEPORT_STEP = 200.0  # units between two owner samples: a teleport, not a walk


def walk_agreement(owner, replica_anims, lag_ms, window_s):
    """owner: Track of the owner's own positions; replica_anims: [(w, bits)] of its puppet.
    The owner moves between two of its samples when its speed is over MOVING_SPEED. A puppet
    sample agrees when its moving bits (walking or running) equal the owner's state at some
    owner sample step in the lag window. Steps with a teleport are left out ("teleports")."""
    out = {"moving": [0, 0], "teleports": [0, 0]}
    ws, ps = owner.ws, owner.ps
    steps = []  # (w at the end of the step, moving, teleport)
    j = 0  # the start of the step (a step spans at least MIN_STEP_S)
    for i in range(1, len(ws)):
        dt = ws[i] - ws[j]
        if dt < MIN_STEP_S:
            continue
        d = math.hypot(ps[i][0] - ps[j][0], ps[i][1] - ps[j][1])
        steps.append((ws[i], d / dt > MOVING_SPEED, d > TELEPORT_STEP))
        j = i
    sws = [x[0] for x in steps]
    shift = (lag_ms or 0) / 1000.0
    for w, bits in replica_anims:
        c = w - shift
        lo = bisect.bisect_left(sws, c - window_s)
        hi = bisect.bisect_right(sws, c + window_s)
        near = steps[lo:hi]
        if not near:
            continue
        rep = bool(bits & ANIM_BITS["moving"])
        if any(x[2] for x in near):
            out["teleports"][1] += 1
            continue
        if not rep and not any(x[1] for x in near):
            continue
        out["moving"][1] += 1
        if any(x[1] == rep for x in near):
            out["moving"][0] += 1
    return out


def flatten(v, prefix, out):
    if isinstance(v, dict):
        for k, x in v.items():
            flatten(x, prefix + "." + str(k), out)
    elif isinstance(v, list):
        out[prefix] = json.dumps(v, sort_keys=True)
    else:
        out[prefix] = v


def shared_leaves(d):
    """The shared values of one dump as {path: value}."""
    out = {}
    for top in SHARED_TOP:
        if top in d and d[top] is not None:
            flatten(d[top], top, out)
    sc = d.get("scene") or {}
    for k in SCENE_KEYS:
        if k in sc:
            out["scene." + k] = sc[k]
    camp = d.get("campaign") or {}
    for k in CAMPAIGN_KEYS:
        if k in camp:
            flatten(camp[k], "campaign." + k, out)
    tg = (d.get("combat") or {}).get("targets")
    if tg is not None:
        flatten(tg, "combat.targets", out)
    return out


def category_of(path):
    top = path.split(".", 1)[0]
    if top in ("quests", "player_quests"):
        return "quests"
    if top in ("dead", "disabled"):
        return "deaths"
    if top == "scene":
        return "dialogue"
    if top == "combat":
        return "combat_targets"
    if top in ("topics", "notes", "map_markers"):
        return "journal"
    if top in ("karma", "reputations", "factions", "faction_relations"):
        return "factions"
    return "other"


_MISSING = object()


def authority(path):
    """The machine that decides a shared value: the guest for its own player quest rows, either
    machine for combat targets (the enemy's owner decides), else the host."""
    if path.startswith("player_quests.guest."):
        return "guest"
    if path.startswith("combat.targets."):
        return None
    return "host"


def propagation(tl):
    """tl: {role: [(w, leaves)]} for two roles. For each change on the deciding machine
    (`authority`), the time until the other machine shows the same value. Returns a list of
    events {path, cat, from, ms | None, state: arrived | lost | late | superseded}.
    A value that the deciding machine changes again before the other machine shows it is
    "superseded". A value that the other machine shows before the deciding machine's own sample
    counts 0 ms: it arrived between two samples. So the resolution is about one sample period."""
    roles = list(tl)
    events = []
    if len(roles) != 2 or not all(tl[r] for r in roles):
        return events
    end_w = min(tl[r][-1][0] for r in roles)
    for src, dst in ((roles[0], roles[1]), (roles[1], roles[0])):
        s_list, d_list = tl[src], tl[dst]
        d_ws = [x[0] for x in d_list]
        for i in range(1, len(s_list)):
            w, cur = s_list[i]
            w_prev, prev = s_list[i - 1]
            if cur is prev:
                continue
            for path in set(cur) | set(prev):
                v = cur.get(path, _MISSING)
                if v == prev.get(path, _MISSING):
                    continue
                auth = authority(path)
                if auth is not None and auth != src:
                    continue
                if auth is None:
                    jw = bisect.bisect_right(d_ws, w) - 1
                    if jw >= 0 and d_list[jw][1].get(path, _MISSING) == v:
                        continue  # either machine decides, and dst showed it first
                j = bisect.bisect_right(d_ws, w_prev) - 1
                if j < 0:
                    continue  # dst not sampled yet: the join snapshot carries it
                nxt = next((x[0] for x in s_list[i + 1:] if x[1].get(path, _MISSING) != v), None)
                got = None
                for jj in range(j + 1, len(d_list)):
                    if nxt is not None and d_list[jj][0] > nxt:
                        break
                    if d_list[jj][1].get(path, _MISSING) == v:
                        got = d_list[jj][0]
                        break
                ev = {"path": path, "cat": category_of(path), "from": src}
                if got is not None:
                    ev.update(state="arrived", ms=round(max(0.0, got - w) * 1000, 1))
                elif nxt is not None:
                    ev.update(state="superseded", ms=None)
                elif end_w - w < END_GRACE_S:
                    ev.update(state="late", ms=None)
                else:
                    ev.update(state="lost", ms=None)
                events.append(ev)
    return events


def hash_metrics(host, guest):
    """host, guest: [(w, rev, shared, recent)]. The guest's revision lag in ms: how long the host has
    had a revision newer than the guest's (the moment the host reached rev + 1, interpolated on the
    host's revision curve). A guest with the host's newest revision has no lag, however long the
    host keeps that revision. And the divergent samples: the guest's hash is not
    the host's root at the guest's revision, and not a root of a revision next to it either (a
    dump can label its hash one revision early: "label_skew")."""
    lags = []
    checked = divergent = skew = 0
    examples = []
    hws = [h[0] for h in host]
    for w, rev, shared, _ in guest:
        if not isinstance(rev, int):
            continue
        # the moment the host reached rev + 1: from then on the guest is behind
        nxt = rev + 1
        for i in range(len(host)):
            if isinstance(host[i][1], int) and host[i][1] >= nxt:
                (w0, r0), (w1, r1) = host[max(0, i - 1)][:2], host[i][:2]
                if i == 0 or not isinstance(r0, int) or r1 == r0:
                    wh = w1
                else:
                    wh = w0 + (nxt - r0) / float(r1 - r0) * (w1 - w0)
                lags.append(max(0.0, (w - wh) * 1000))
                break
        else:
            lags.append(0.0)  # the host has no newer revision
        # the host's root at that revision, from the host sample next to w
        j = min(len(host) - 1, max(0, bisect.bisect_left(hws, w)))
        for h in host[max(0, j - 2):j + 3]:
            root = None
            if h[1] == rev:
                root = h[2]
            else:
                for pair in h[3] or []:
                    if isinstance(pair, list) and len(pair) == 2 and pair[0] == rev:
                        root = pair[1]
            if root is not None:
                checked += 1
                if root != shared:
                    near = host[max(0, j - 3):j + 4]
                    roots = {x[2] for x in near}
                    roots |= {pr[1] for x in near for pr in (x[3] or []) if isinstance(pr, list) and len(pr) == 2}
                    if shared in roots:
                        skew += 1  # the guest's hash belongs to a revision next to its label
                    else:
                        divergent += 1
                        if len(examples) < 5:
                            examples.append({"w": round(w, 2), "rev": rev, "host": root, "guest": shared})
                break
    return {"lag_ms": lags, "checked": checked, "divergent": divergent, "label_skew": skew, "examples": examples}


# ---- one run ---------------------------------------------------------------------------------------


def load_samples(path):
    """{role: [sample]} with each sample's dump `d` carried forward from earlier samples."""
    by_role, full = {}, {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            try:
                s = json.loads(line)
            except ValueError:
                continue
            role = s["role"]
            s["changed"] = set(s["d"])
            s.setdefault("rw", s["w"])
            cur = dict(full.get(role, {}))
            cur.update(s["d"])
            full[role] = cur
            s["d"] = cur
            by_role.setdefault(role, []).append(s)
    return by_role


def _rows(s, kind):
    return [r for r in s["rows"] if r.get("kind") == kind]


def analyse_samples(by_role, window_s=0.3):
    """The sync metrics of one run from its samples (see the module doc)."""
    host, guest = by_role.get("host", []), by_role.get("guest", [])
    res = {"samples": {r: len(v) for r, v in by_role.items()}}
    if not host or not guest:
        res["error"] = "no samples from %s" % ("host" if not host else "guest")
        return res
    roles = {"host": host, "guest": guest}
    other = {"host": "guest", "guest": "host"}
    by_k = {r: {s["k"]: s for s in v} for r, v in roles.items()}
    both_k = sorted(set(by_k["host"]) & set(by_k["guest"]))

    # tracked actors: positions and animations on each machine
    tracks, anims, owner_at = {}, {}, {}
    for role, ss in roles.items():
        for s in ss:
            for r in s["rows"]:
                if r.get("kind") == "puppet":
                    continue
                name = r.get("name")
                tracks.setdefault(name, {}).setdefault(role, []).append((s["rw"], r["pos"]))
                anims.setdefault(name, {}).setdefault(role, []).append((s["rw"], int(r.get("anim") or 0)))
            for name, a in (s["d"].get("actors") or {}).items():
                owner_at.setdefault(name, {}).setdefault(s["k"], {})[role] = a.get("owner")
    npc = {"raw": [], "comp": [], "lags": [], "anim": {}, "present": [0, 0], "conflict": 0, "unowned": 0,
           "actor_ticks": 0, "per_actor": {}}
    for name, ticks in owner_at.items():
        rep_by_owner = {"host": [], "guest": []}
        for k in both_k:
            claims = ticks.get(k, {})
            if not claims:
                continue
            npc["actor_ticks"] += 1
            mine = [r for r, o in claims.items() if o == "me"]
            if len(mine) == 2:
                npc["conflict"] += 1
                continue
            if not mine:
                npc["unowned"] += 1
                continue
            own = mine[0]
            rep = other[own]
            so, sr = by_k[own][k], by_k[rep][k]
            o_row = next((r for r in so["rows"] if r.get("name") == name and r.get("kind") != "puppet"), None)
            r_row = next((r for r in sr["rows"] if r.get("name") == name and r.get("kind") != "puppet"), None)
            if o_row is not None and so.get("cell") and so.get("cell") == sr.get("cell"):
                npc["present"][1] += 1
                if r_row is not None:
                    npc["present"][0] += 1
            if r_row is not None:
                rep_by_owner[own].append((sr["rw"], r_row["pos"]))
        info = {}
        for own, pts in rep_by_owner.items():
            if not pts or own not in tracks.get(name, {}):
                continue
            m = pos_metrics(Track(tracks[name][own]), pts)
            npc["raw"] += m["raw"]
            npc["comp"] += m["comp"]
            if m["lag_ms"] is not None:
                npc["lags"].append(m["lag_ms"])
            rep_w = {round(w, 6) for w, _ in pts}
            rep_anims = [a for a in anims.get(name, {}).get(other[own], []) if round(a[0], 6) in rep_w]
            ag = anim_agreement(anims[name].get(own, []), rep_anims, m["lag_ms"], window_s)
            for g, (a, n) in ag.items():
                acc = npc["anim"].setdefault(g, [0, 0])
                acc[0] += a
                acc[1] += n
            info[own] = {"n": m["n"], "raw_p95": pct(m["raw"], 95), "comp_p95": pct(m["comp"], 95),
                         "lag_ms": m["lag_ms"], "moved": round(m.get("moved", 0.0), 1)}
        if info:
            npc["per_actor"][name] = info
    res["npc"] = npc

    # avatars: the owner's own position, the puppet on the other machine
    av = {"raw": [], "comp": [], "lags": [], "moving": [0, 0], "teleports": [0, 0], "present": [0, 0],
          "per_player": {}}
    for own, rep in (("host", "guest"), ("guest", "host")):
        pid = ROLE_PLAYER[own]
        own_pts = [(s["w"], (s["d"].get("positions") or {}).get(own)) for s in roles[own]]
        own_tr = Track([p for p in own_pts if p[1]])
        pts, moving = [], []
        for s in roles[rep]:
            row = next((r for r in _rows(s, "puppet") if r.get("player") == pid), None)
            o = by_k[own].get(s["k"])
            if o is not None and o.get("cell") and o.get("cell") == s.get("cell"):
                av["present"][1] += 1
                if row is not None:
                    av["present"][0] += 1
            if row is None:
                continue
            pts.append((s["rw"], row["pos"]))
            moving.append((s["rw"], int(row.get("anim") or 0)))
        m = pos_metrics(own_tr, pts)
        av["raw"] += m["raw"]
        av["comp"] += m["comp"]
        if m["lag_ms"] is not None:
            av["lags"].append(m["lag_ms"])
        for key, n in walk_agreement(own_tr, moving, m["lag_ms"], window_s).items():
            av[key][0] += n[0]
            av[key][1] += n[1]
        av["per_player"][own] = {"n": m["n"], "raw_p95": pct(m["raw"], 95), "comp_p95": pct(m["comp"], 95),
                                 "lag_ms": m["lag_ms"], "moved": round(m.get("moved", 0.0), 1)}
    res["avatars"] = av

    # shared values: change on one machine, same value on the other
    tl = {}
    for role, ss in roles.items():
        seq, last = [], None
        for s in ss:
            if last is None or s["changed"] - set(STORE_ALWAYS):
                last = shared_leaves(s["d"])
            seq.append((s["w"], last))
        tl[role] = seq
    res["state"] = propagation(tl)

    # shared-state hash
    def hs(ss):
        out = []
        for s in ss:
            h = s["d"].get("hash") or {}
            out.append((s["w"], h.get("rev"), h.get("shared"), h.get("recent")))
        return out
    res["hash"] = hash_metrics(hs(host), hs(guest))

    # game clock skew (game seconds) and frame rates
    skew = []
    for k in both_k:
        a, b = by_k["host"][k]["clock"], by_k["guest"][k]["clock"]
        if a and b and a[0] is not None and b[0] is not None:
            skew.append(abs(float(a[0]) - float(b[0])) * 86400.0)
    res["clock_skew_s"] = skew
    res["fps"] = {r: [s["fps"] for s in ss if s.get("go") and s.get("fps")] for r, ss in roles.items()}
    res["ask_ms"] = {r: [s["ask_ms"] for s in ss if s.get("go")] for r, ss in roles.items()}
    last_status = {r: (ss[-1]["d"].get("status") or {}) for r, ss in roles.items()}
    rtts = []
    for s in host:
        st = s["d"].get("status") or {}
        if isinstance(st.get("rtt_ms"), (int, float)):
            rtts.append(st["rtt_ms"])
    res["rtt_ms"] = rtts
    res["status"] = {r: {k: st.get(k) for k in ("desyncs", "repairs", "snapshots", "decode_errors", "rev")}
                     for r, st in last_status.items()}
    res["pending_max"] = max([(s["d"].get("status") or {}).get("pending_commands") or 0 for s in guest] or [0])
    res["span_s"] = max(host[-1]["w"], guest[-1]["w"]) - min(host[0]["w"], guest[0]["w"])
    return res


def ratio(pair):
    return None if not pair or not pair[1] else pair[0] / float(pair[1])


def r1(x):
    return None if x is None else round(x, 1)


def summarize_run(res, verdict, relay, meta):
    """A flat summary of one run (no lists), for bench.json and the tables."""
    out = dict(meta)
    out["verdict"] = verdict.get("result") if verdict else None
    out["verdict_class"] = verdict.get("class") if verdict else None
    out["failed_checks"] = [c.get("id") for c in (verdict or {}).get("checks", []) if not c.get("ok")]
    if "error" in res:
        out["error"] = res["error"]
        return out
    n, a = res["npc"], res["avatars"]
    out.update({
        "npc_n": len(n["raw"]), "npc_raw_p50": r1(pct(n["raw"], 50)), "npc_raw_p95": r1(pct(n["raw"], 95)),
        "npc_comp_p50": r1(pct(n["comp"], 50)), "npc_comp_p95": r1(pct(n["comp"], 95)),
        "npc_lag_ms": r1(pct(n["lags"], 50)), "npc_present": r1(100 * ratio(n["present"])) if n["present"][1] else None,
        "npc_conflict": n["conflict"], "npc_unowned": n["unowned"], "npc_actor_ticks": n["actor_ticks"],
        "npc_anim_all": r1(100 * ratio(n["anim"].get("all"))) if ratio(n["anim"].get("all")) is not None else None,
        "npc_anim_moving": r1(100 * ratio(n["anim"].get("moving"))) if ratio(n["anim"].get("moving")) is not None else None,
        "npc_anim_fighting": r1(100 * ratio(n["anim"].get("fighting"))) if ratio(n["anim"].get("fighting")) is not None else None,
        "av_n": len(a["raw"]), "av_raw_p50": r1(pct(a["raw"], 50)), "av_raw_p95": r1(pct(a["raw"], 95)),
        "av_comp_p50": r1(pct(a["comp"], 50)), "av_comp_p95": r1(pct(a["comp"], 95)),
        "av_lag_ms": r1(max(a["lags"]) if a["lags"] else None), "av_present": r1(100 * ratio(a["present"])) if a["present"][1] else None,
        "av_anim_moving": r1(100 * ratio(a["moving"])) if a["moving"][1] else None,
        "av_anim_n": a["moving"][1],
    })
    arrived = [e["ms"] for e in res["state"] if e["state"] == "arrived"]
    out.update({
        "state_changes": len(res["state"]), "state_p50_ms": r1(pct(arrived, 50)), "state_p95_ms": r1(pct(arrived, 95)),
        "state_max_ms": r1(max(arrived) if arrived else None),
        "state_lost": sum(1 for e in res["state"] if e["state"] == "lost"),
        "state_late": sum(1 for e in res["state"] if e["state"] == "late"),
        "lost_paths": sorted({e["path"] for e in res["state"] if e["state"] == "lost"})[:12],
    })
    cats = {}
    for e in res["state"]:
        c = cats.setdefault(e["cat"], {"n": 0, "lost": 0, "ms": []})
        c["n"] += 1
        if e["state"] == "lost":
            c["lost"] += 1
        if e["state"] == "arrived":
            c["ms"].append(e["ms"])
    out["state_by_cat"] = {k: {"n": v["n"], "lost": v["lost"], "p50_ms": r1(pct(v["ms"], 50)),
                               "p95_ms": r1(pct(v["ms"], 95))} for k, v in sorted(cats.items())}
    h = res["hash"]
    out.update({"hash_lag_p50_ms": r1(pct(h["lag_ms"], 50)), "hash_lag_p95_ms": r1(pct(h["lag_ms"], 95)),
                "hash_checked": h["checked"], "hash_divergent": h["divergent"],
                "hash_label_skew": h["label_skew"], "hash_examples": h["examples"]})
    out["clock_skew_p95_s"] = r1(pct(res["clock_skew_s"], 95))
    for r in ("host", "guest"):
        f = res["fps"].get(r) or []
        out["fps_%s_p50" % r] = r1(pct(f, 50))
        out["fps_%s_p5" % r] = r1(pct(f, 5))
        out["ask_%s_p50_ms" % r] = r1(pct(res["ask_ms"].get(r) or [], 50))
    out["rtt_p50_ms"] = r1(pct(res["rtt_ms"], 50))
    out["rtt_p95_ms"] = r1(pct(res["rtt_ms"], 95))
    out["status"] = res["status"]
    out["pending_max"] = res["pending_max"]
    span = res["span_s"] or 1.0
    if relay:
        for d in ("g2h", "h2g"):
            x = relay.get(d) or {}
            out["%s_kBps" % d] = r1((x.get("bytes_in") or 0) / 1024.0 / span)
            out["%s_pps" % d] = r1((x.get("in") or 0) / span)
            out["%s_dropped" % d] = x.get("dropped")
    out["span_s"] = r1(span)
    return out


def read_json(path):
    try:
        with open(path, encoding="utf-8") as fh:
            return json.load(fh)
    except (OSError, ValueError):
        return None


def analyse_run(run_dir):
    meta = read_json(os.path.join(run_dir, "bench_meta.json")) or {}
    verdict = read_json(os.path.join(run_dir, "verdict.json"))
    relay = read_json(os.path.join(run_dir, "relay.stats.json"))
    path = os.path.join(run_dir, "samples.jsonl")
    res = analyse_samples(load_samples(path)) if os.path.exists(path) else {"error": "no samples.jsonl"}
    s = summarize_run(res, verdict, relay, meta)
    s["run_dir"] = run_dir
    with open(os.path.join(run_dir, "bench_run.json"), "w", encoding="utf-8") as fh:
        json.dump(s, fh, indent=2)
    return s, res


# ---- the report -------------------------------------------------------------------------------------


def _fmt(v, unit=""):
    if v is None:
        return "-"
    if isinstance(v, float):
        v = ("%.1f" % v).rstrip("0").rstrip(".")
    return "%s%s" % (v, unit)


def aggregate(group):
    """group: [(summary, res)] -> one row of pooled figures."""
    raw = lambda key: [x for _, r in group if "error" not in r for x in key(r)]  # noqa: E731
    npc_raw, npc_comp = raw(lambda r: r["npc"]["raw"]), raw(lambda r: r["npc"]["comp"])
    av_raw, av_comp = raw(lambda r: r["avatars"]["raw"]), raw(lambda r: r["avatars"]["comp"])
    npc_lag, av_lag = raw(lambda r: r["npc"]["lags"]), raw(lambda r: r["avatars"]["lags"])
    arrived = raw(lambda r: [e["ms"] for e in r["state"] if e["state"] == "arrived"])
    hash_lag = raw(lambda r: r["hash"]["lag_ms"])

    def summ(get):
        a = n = 0
        for _, r in group:
            if "error" in r:
                continue
            x = get(r)
            if x:
                a += x[0]
                n += x[1]
        return None if not n else round(100.0 * a / n, 1)
    fps = [s.get("fps_guest_p5") for s, _ in group if s.get("fps_guest_p5") is not None]
    fps += [s.get("fps_host_p5") for s, _ in group if s.get("fps_host_p5") is not None]
    return {
        "runs": len(group),
        "passed": sum(1 for s, _ in group if s.get("verdict") == "pass"),
        "npc_raw_p50": r1(pct(npc_raw, 50)), "npc_raw_p95": r1(pct(npc_raw, 95)),
        "npc_comp_p50": r1(pct(npc_comp, 50)), "npc_comp_p95": r1(pct(npc_comp, 95)),
        "npc_lag_ms": r1(pct(npc_lag, 50)),
        "npc_present": summ(lambda r: r["npc"]["present"]),
        "npc_anim": summ(lambda r: r["npc"]["anim"].get("all")),
        "npc_anim_fighting": summ(lambda r: r["npc"]["anim"].get("fighting")),
        "npc_conflict": sum(r["npc"]["conflict"] for _, r in group if "error" not in r),
        "av_raw_p50": r1(pct(av_raw, 50)), "av_raw_p95": r1(pct(av_raw, 95)),
        "av_comp_p50": r1(pct(av_comp, 50)), "av_comp_p95": r1(pct(av_comp, 95)),
        "av_lag_ms": r1(max(av_lag) if av_lag else None),
        "av_present": summ(lambda r: r["avatars"]["present"]),
        "av_anim_moving": summ(lambda r: r["avatars"]["moving"]),
        "state_n": len(arrived), "state_p50_ms": r1(pct(arrived, 50)), "state_p95_ms": r1(pct(arrived, 95)),
        "state_lost": sum(s.get("state_lost") or 0 for s, _ in group),
        "hash_lag_p50_ms": r1(pct(hash_lag, 50)), "hash_lag_p95_ms": r1(pct(hash_lag, 95)),
        "hash_divergent": sum(s.get("hash_divergent") or 0 for s, _ in group),
        "hash_checked": sum(s.get("hash_checked") or 0 for s, _ in group),
        "h2g_kBps": r1(mean([s["h2g_kBps"] for s, _ in group if s.get("h2g_kBps") is not None])),
        "g2h_kBps": r1(mean([s["g2h_kBps"] for s, _ in group if s.get("g2h_kBps") is not None])),
        "fps_p5_min": r1(min(fps) if fps else None),
        "desyncs": sum(sum((st or {}).get("desyncs") or 0 for st in (s.get("status") or {}).values())
                       for s, _ in group),
    }


AGG_COLS = [
    ("runs", "runs"), ("passed", "pass"),
    ("npc_raw_p50", "NPC err p50"), ("npc_raw_p95", "NPC err p95"), ("npc_comp_p95", "NPC err p95 at lag"),
    ("npc_lag_ms", "NPC lag ms"), ("npc_present", "NPC copy %"), ("npc_anim", "NPC anim %"),
    ("npc_conflict", "own conflicts"),
    ("av_raw_p50", "avatar err p50"), ("av_raw_p95", "avatar err p95"), ("av_comp_p95", "avatar err p95 at lag"),
    ("av_lag_ms", "avatar lag ms (max)"), ("av_anim_moving", "avatar walk anim %"),
    ("state_n", "state changes"), ("state_p50_ms", "state ms p50"), ("state_p95_ms", "state ms p95"),
    ("state_lost", "lost"), ("hash_lag_p50_ms", "hash lag p50"), ("hash_lag_p95_ms", "hash lag p95"),
    ("hash_divergent", "divergent"), ("h2g_kBps", "h->g kB/s"), ("g2h_kBps", "g->h kB/s"),
    ("fps_p5_min", "fps p5 min"), ("desyncs", "desyncs"),
]


def table(rows, cols, first):
    head = "| %s | %s |" % (first, " | ".join(c[1] for c in cols))
    sep = "|" + "---|" * (len(cols) + 1)
    lines = [head, sep]
    for name, row in rows:
        lines.append("| %s | %s |" % (name, " | ".join(_fmt(row.get(c[0])) for c in cols)))
    return "\n".join(lines)


def flags(summary, res):
    """Plain findings for one run that a person must look at."""
    out = []
    if summary.get("verdict") != "pass":
        out.append("verdict %s (%s): %s" % (summary.get("verdict"), summary.get("verdict_class"),
                                             ", ".join(summary.get("failed_checks") or []) or "-"))
    if summary.get("error"):
        out.append("no analysis: %s" % summary["error"])
        return out
    if summary.get("hash_divergent"):
        out.append("%d samples with a different shared hash at the same revision" % summary["hash_divergent"])
    if summary.get("state_lost"):
        out.append("%d shared changes never arrived: %s" % (summary["state_lost"], ", ".join(summary["lost_paths"])))
    if summary.get("npc_conflict"):
        out.append("%d actor samples that both machines claim" % summary["npc_conflict"])
    for key, label, limit in (("npc_comp_p95", "NPC error p95 after the lag", 150),
                              ("av_comp_p95", "avatar error p95 after the lag", 150)):
        if (summary.get(key) or 0) > limit:
            out.append("%s %.0f units (over %d)" % (label, summary[key], limit))
    for key, label, limit in (("npc_present", "NPC copies present", 95), ("av_present", "avatar copies present", 95)):
        if summary.get(key) is not None and summary[key] < limit:
            out.append("%s %.1f %% (under %d %%)" % (label, summary[key], limit))
    for key, label in (("npc_anim_all", "NPC animation agreement"), ("av_anim_moving", "avatar walk animation agreement")):
        if key == "av_anim_moving" and (summary.get("av_anim_n") or 0) < MIN_WALK_SAMPLES:
            continue  # a few samples are no measure
        if summary.get(key) is not None and summary[key] < 80:
            out.append("%s %.1f %% (under 80 %%)" % (label, summary[key]))
    return out


def write_report(bench_dir, runs, categories):
    """runs: [(summary, res)]. Writes bench.json and bench.md; returns the Markdown text."""
    cat_of = {n: c for c, names in categories.items() for n in names}
    profiles = []
    for s, _ in runs:
        if s.get("profile") not in profiles:
            profiles.append(s.get("profile"))
    by_profile = [(p, aggregate([x for x in runs if x[0].get("profile") == p])) for p in profiles]
    by_cat = []
    for c in categories:
        for p in profiles:
            g = [x for x in runs if cat_of.get(x[0].get("scenario")) == c and x[0].get("profile") == p]
            if g:
                by_cat.append(("%s / %s" % (c, p), aggregate(g)))
    data = {"profiles": dict(by_profile), "categories": dict(by_cat), "runs": [s for s, _ in runs]}
    with open(os.path.join(bench_dir, "bench.json"), "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=2)
    md = ["# Co-op sync benchmark", "",
          "%d runs, %d scenarios, profiles: %s. Distances are game units (about 1.4 cm each). "
          "\"at lag\" means after the best time shift between the owner and the copy." % (
              len(runs), len({s.get("scenario") for s, _ in runs}), ", ".join(profiles)), "",
          "## Per profile", "", table(by_profile, AGG_COLS, "profile"), "",
          "## Per category and profile", "", table(by_cat, AGG_COLS, "category / profile"), "",
          "## Shared values per kind (all runs)", ""]
    kinds = {}
    for _, r in runs:
        if "error" in r:
            continue
        for e in r["state"]:
            k = kinds.setdefault((e["cat"]), {"n": 0, "lost": 0, "late": 0, "ms": []})
            k["n"] += 1
            if e["state"] in ("lost", "late"):
                k[e["state"]] += 1
            if e["state"] == "arrived":
                k["ms"].append(e["ms"])
    md.append(table([(k, {"n": v["n"], "p50": r1(pct(v["ms"], 50)), "p95": r1(pct(v["ms"], 95)),
                          "max": r1(max(v["ms"]) if v["ms"] else None), "lost": v["lost"], "late": v["late"]})
                     for k, v in sorted(kinds.items())],
                    [("n", "changes"), ("p50", "ms p50"), ("p95", "ms p95"), ("max", "ms max"),
                     ("lost", "lost"), ("late", "late at the end")], "kind"))
    md += ["", "## Runs", ""]
    run_cols = [("verdict", "verdict"), ("npc_raw_p95", "NPC err p95"), ("npc_comp_p95", "at lag"),
                ("npc_lag_ms", "NPC lag"), ("npc_anim_all", "NPC anim %"), ("av_raw_p95", "avatar err p95"),
                ("av_comp_p95", "at lag"), ("av_lag_ms", "avatar lag"), ("state_changes", "state chg"),
                ("state_p95_ms", "state p95 ms"), ("state_lost", "lost"), ("hash_lag_p95_ms", "hash lag p95"),
                ("hash_divergent", "divergent"), ("h2g_kBps", "h->g kB/s"), ("fps_guest_p50", "guest fps")]
    md.append(table([("%s / %s" % (s.get("scenario"), s.get("profile")), s) for s, _ in runs], run_cols, "run"))
    md += ["", "## Findings", ""]
    any_flag = False
    for s, r in runs:
        fl = flags(s, r)
        if fl:
            any_flag = True
            md.append("- **%s / %s** (%s): %s" % (s.get("scenario"), s.get("profile"),
                                                  os.path.basename(s.get("run_dir", "")), "; ".join(fl)))
    if not any_flag:
        md.append("No run has a finding.")
    text = "\n".join(md) + "\n"
    with open(os.path.join(bench_dir, "bench.md"), "w", encoding="utf-8") as fh:
        fh.write(text)
    return text


def report(bench_dir, categories=None):
    plan = read_json(os.path.join(bench_dir, "bench_plan.json")) or {}
    categories = categories or plan.get("categories") or SETS["full"]
    runs = []
    for name in sorted(os.listdir(bench_dir)):
        d = os.path.join(bench_dir, name)
        if os.path.isdir(d) and os.path.exists(os.path.join(d, "bench_meta.json")):
            runs.append(analyse_run(d))
    return write_report(bench_dir, runs, categories)


# ---- command line -----------------------------------------------------------------------------------


def main(argv=None):
    ap = argparse.ArgumentParser(description="co-op sync benchmark on two real viewers")
    sub = ap.add_subparsers(dest="cmd", required=True)
    rp = sub.add_parser("run")
    rp.add_argument("--set", default="full", choices=sorted(SETS))
    rp.add_argument("--profiles", default=",".join(DEFAULT_PROFILES))
    rp.add_argument("--n", type=int, default=1)
    rp.add_argument("--hz", type=float, default=5.0)
    rp.add_argument("--only", nargs="*")
    rp.add_argument("--config")
    rp.add_argument("--out")
    pp = sub.add_parser("report")
    pp.add_argument("bench_dir")
    a = ap.parse_args(argv)
    if a.cmd == "report":
        print(report(a.bench_dir))
        return 0
    cfg = runner_mod.load_config(a.config)
    categories = {c: [n for n in names if not a.only or n in a.only] for c, names in SETS[a.set].items()}
    categories = {c: v for c, v in categories.items() if v}
    profiles = [p for p in a.profiles.split(",") if p]
    bench_dir = a.out or os.path.join(cfg["out_dir"], "bench-" + runner_mod.stamp())
    os.makedirs(bench_dir, exist_ok=True)
    with open(os.path.join(bench_dir, "bench_plan.json"), "w", encoding="utf-8") as fh:
        json.dump({"set": a.set, "profiles": profiles, "n": a.n, "hz": a.hz, "categories": categories}, fh, indent=2)
    plan = [(c, n, p, i) for c, names in categories.items() for n in names for p in profiles for i in range(a.n)]
    print("bench: %d runs in %s" % (len(plan), bench_dir), flush=True)
    t0 = time.monotonic()
    for idx, (c, name, prof, i) in enumerate(plan):
        path = os.path.join(HERE, "scenarios", name + ".json")
        run_dir = os.path.join(bench_dir, "%s__%s__%d" % (name, prof, i + 1))
        print("[%d/%d] %s / %s / %d" % (idx + 1, len(plan), name, prof, i + 1), flush=True)
        try:
            s, _ = run_one(path, prof, cfg, run_dir, a.hz, log=lambda *x: None)
            print("    %s npc p95 %s avatar p95 %s state p95 %s ms lost %s divergent %s (%.0f min so far)" % (
                s.get("verdict"), _fmt(s.get("npc_raw_p95")), _fmt(s.get("av_raw_p95")),
                _fmt(s.get("state_p95_ms")), _fmt(s.get("state_lost")), _fmt(s.get("hash_divergent")),
                (time.monotonic() - t0) / 60), flush=True)
        except Exception as e:  # one broken run must not stop the benchmark
            print("    harness error: %s: %s" % (type(e).__name__, e), flush=True)
    print(report(bench_dir, categories))
    return 0


if __name__ == "__main__":
    sys.exit(main())
