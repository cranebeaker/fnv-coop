"""The eight rooms of the lab (docs/LAB.md). Each is `room(ctx)`: it records checks through the ctx, and the first failed check ends it.

Every check compares the SAME placed ref (same id) on both copies, never counts. Numbers that are guesses until a real run are in PARAMS
(--param room.key=value changes one for a run) and are listed as uncertain in docs/tools/lab.md.
"""
from __future__ import annotations

import json
import math
import os
from typing import Any, Callable, Dict, List, Optional, Tuple

import lab_core as K
import lab_net
import lab_parity
from lab_core import CLIENT, HOST, Blocked, Ctx, connected, dist2, dist3, digest_counts

# the numbers a room compares against; the first in-game run may move them
PARAMS: Dict[str, float] = {
    "spawn.idle_s": 20.0,            # idle digest window
    "spawn.connect_s": 30.0,
    "spawn.digest_wait_s": 12.0,     # after the idle window: how long to wait for the first digest verdict (ok, or a mismatch / repair, which ends the wait)
    "spawn.puppet_wait_s": 30.0,     # how long the other player's puppet may take to show up near him
    "spawn.puppet_err": 250.0,       # units between the other player's real position and this copy's puppet of him, standing still
    "walk.err_p95": 150.0,           # puppet error while the other player walks (units, 2D)
    "walk.err_max": 300.0,
    "walk.jump": 250.0,              # a puppet that moves more than this between two samples (0.25 s) teleported
    "walk.lag_s": 3.0,               # after the walker stops, the puppet is within walk.near of him by then
    "walk.near": 80.0,
    "walk.min_samples": 8.0,
    "walk.leg_timeout_s": 25.0,
    "fight.latency_s": 3.0,          # a death seen on the shooter's copy shows on the other copy within this
    "fight.corpse_s": 8.0,           # how long the dead are watched
    "fight.weak_hp": 5.0,            # the host (owner) sets the target's health this low first, so one pistol hit kills it (as tools/bench/real_stage.py does)
    "fight.weak_s": 8.0,             # the client's copy shows the weakened health within this
    "fight.volleys": 24.0,           # first in-game run: 12 single pistol shots from 350 units left a hostile at 21 health
    "loot.settle_s": 10.0,
    "door.settle_s": 10.0,
    "npc.handoff_s": 25.0,
    "npc.min_far": 600.0,            # the stand marker must be this far from the NPC for a handoff to be possible
    "npc.watch_s": 8.0,
    "npc.hold_s": 5.0,               # the host keeps the NPC this long with the client next to it (minHoldMs 1500 + the switch margin, with room to spare)
    "npc.err_max": 150.0,            # the two copies' views of the NPC's position
    "npc.jump": 300.0,               # one NPC position step larger than this, beyond what the patrol covers in the time between the two samples, is a snap
    "npc.speed": 200.0,              # units per second a patrolling NPC may cover (a walk is about 140; the samples are not evenly spaced: a busy PC answers a bridge call in 0.3 s)
    "npc.min_moved": 20.0,           # the patrolling NPC moved at least this far on its owner's copy
    "vats.tries": 6.0,
    "vats.weak_hp": 5.0,             # the host sets the VATS target's health this low first (one pistol hit of about 9 kills it: every VATS entry is a crash risk, see docs/tools/lab.md)
    "vats.weak_s": 8.0,
    "vats.settle_s": 8.0,
    "quest.settle_s": 10.0,
    "quest.hold_s": 4.0,
    "parity.settle_s": 6.0,          # a change on its owner's copy shows on the other copy within this (the room compares until they agree, or this is used up)
    "parity.min_rounds": 3.0,        # rounds of both copies' views per step before a step may end (animation categories are compared as sets over the rounds)
    "parity.sit_s": 6.0,             # the owner's own copy shows the scripted situation (weapon out, seated, standing) within this, or the step is skipped (not driven)
    "parity.punch_window_s": 3.0,    # after the owner's copy first shows the attack, the other copy's puppet shows one within this
    "parity.punches": 3.0,           # punches per player (a short animation is easy to miss between two bridge reads)
}

# A network pass (lab.py run --net good_wifi,bad_wifi) judges a room by `room.key@profile` where it exists: the clean value plus what that profile may add.
# The allowances are derived from tools/chaos/profiles.json in lab_net.py (its docstring says why); `python -I tools/lab/lab_net.py` prints them.
PARAMS.update(lab_net.net_params(PARAMS))

AWAY_CELL = "GSDocMitchellHouse"      # a small stock interior (Doc Mitchell's house, CELL in FalloutNV.esm) the host `coc`s into so it stops being eligible for the lab NPC
DEFAULT_FLOOR_ITEM = "FalloutNV.esm:015169"   # a Stimpak, the same item the bench arena uses


def _ok_why(r: Tuple[bool, str]) -> Tuple[bool, str]:
    return bool(r[0]), r[1] or ""


# ------------------------------------------------------------------ spawn
def room_spawn(ctx: Ctx) -> Dict[str, Any]:
    """Both copies in the lab cell, connected, an idle digest that stays clean."""
    t_enter = ctx.now()
    if ctx.enter_hook:
        ctx.enter_hook()
        t_enter = ctx.now()              # from here both copies are in the lab and (in a network pass) joined through the relay
    L = ctx.layout
    mk = {HOST: L.marker("spawn", "host"), CLIENT: L.marker("spawn", "client")}
    ctx.refs_exist(list(mk.values()), "the spawn markers")
    for m in ctx.ms:
        ok, why = m.goto_ref(mk[m.name], 0.0)
        ctx.check(f"{m.name} is moved to its spawn marker", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (teleport)")
    ctx.agree("both copies are in the lab cell", lambda m: m.cell_of() == m.cell_of(mk[HOST]), 20.0, True, cls="ENV", area="plugin/coop (area change)")
    ctx.wait("both copies are connected", lambda: (lambda r: (r[0], r[1] or "joined"))(connected(ctx.host.status(), ctx.client.status())),
             ctx.p("connect_s"), "joined", area="plugin/coop, plugin/loader")

    def puppet_err(m, other):
        pp, op = m.puppet_pos(), other.pos()
        return None if pp is None else dist2(pp, op)
    lim = ctx.p("puppet_err")
    # Where the time goes when the puppet is slow to appear (docs/tools/lab.md "First appearances"): per copy, the moment it first had a puppet at all, a placed one, and one near
    # the player, and every change of the puppet's phase / ref / snapshot count, all in ms from the start of this wait.
    t_wait = ctx.now()
    first: Dict[str, Dict[str, int]] = {HOST: {}, CLIENT: {}}
    trace: List[str] = []
    last_note: Dict[str, Any] = {}

    def puppets_near():
        errs = {}
        for m, other in ((ctx.host, ctx.client), (ctx.client, ctx.host)):
            errs[m.name] = puppet_err(m, other)
            now_ms = int((ctx.now() - t_wait) * 1000)
            d = getattr(m, "puppet_diag", None) or {}
            if d.get("n") and "listed" not in first[m.name]:
                first[m.name]["listed"] = now_ms
            if errs[m.name] is not None and "placed" not in first[m.name]:
                first[m.name]["placed"] = now_ms
            if errs[m.name] is not None and errs[m.name] <= lim and "near" not in first[m.name]:
                first[m.name]["near"] = now_ms
            note = (d.get("phase"), d.get("ref"), d.get("snaps"), errs[m.name] is not None, tuple(round(x / 50) for x in (d.get("pose") or [])), tuple(round(x / 50) for x in (d.get("applied") or [])))
            if d and last_note.get(m.name) != note and len(trace) < 24:
                last_note[m.name] = note
                trace.append(f"{now_ms}ms {m.name}: {d.get('phase')} ref {d.get('ref')} snaps {d.get('snaps')} fed {d.get('fed')} spawn {d.get('spawnCalls')}/{d.get('throttled')} err "
                             + ("-" if errs[m.name] is None else f"{errs[m.name]:.0f}") + (f" pose {d.get('pose')} applied {d.get('applied')} tp {d.get('teleportSnaps')} age {d.get('newestAge')} net[{d.get('net')}]" if d.get("pose") else ""))
        a, b = errs[HOST], errs[CLIENT]
        ctx.metrics.update(host_puppet_err=None if a is None else round(a, 1), client_puppet_err=None if b is None else round(b, 1))     # the last reading, even when the wait fails
        return (a is not None and b is not None and a <= lim and b <= lim), {HOST: a, CLIENT: b}
    ctx.wait("each copy shows the other player's puppet near him", puppets_near, ctx.p("puppet_wait_s"), f"<= {lim:.0f} units", area="plugin/avatar")
    ctx.metric("puppet_seen_ms", int(round(ctx.last_wait_s * 1000)))
    ctx.metric("puppet_wait_started_ms_after_enter", int(round((t_wait - t_enter) * 1000)))      # how long the entry's own steps (teleports, cell, connected) had already given it
    for who in (HOST, CLIENT):
        for k in ("listed", "placed", "near"):
            if k in first[who]:
                ctx.metric(f"{who}_puppet_{k}_ms", first[who][k])
    if trace and ctx.last_wait_s > 0.5:
        ctx.metric("puppet_trace", " | ".join(trace)[:3200])
    ctx.rebase()
    d0 = digest_counts(ctx.host.status())
    ctx.sleep(ctx.p("idle_s"))
    # The wait for a first "ok" is short and ends the moment a digest is judged bad: a copy pair that never agrees (a mismatch the repair cannot
    # fix) must fail HERE with the divergence, not use up the room's time box waiting for an ok that cannot come.
    slow = K.MUTANT == "digest_wait_unbounded"          # twin: the old 30 s wait for an ok that did not end on a bad digest
    end = ctx.now() + (30.0 if slow else ctx.p("digest_wait_s"))
    while True:
        d = digest_counts(ctx.host.status())
        dm = ctx.digests_moved()
        if d["ok"] > d0["ok"] or (not slow and (dm["mismatch"] or dm["repairs"])) or ctx.now() >= end:
            break
        ctx.sleep(1.0)
    ctx.check("idle digest stayed clean for the window", dm["mismatch"] == 0 and dm["repairs"] == 0, "mismatch 0, repairs 0", dm, area="plugin/areadigest, synccore digest")
    ok_after = d["ok"] - d0["ok"]
    ctx.check("the host judged at least one digest ok while idle", ok_after > 0, "> 0", ok_after, f"waited {ctx.p('idle_s') + ctx.p('digest_wait_s'):.0f} s at most",
              area="plugin/areadigest, synccore digest")
    return {"digest_ok_idle": ok_after}


# ------------------------------------------------------------------ walk
def walk_stats(samples: List[Tuple[float, Tuple[float, float, float], Optional[Tuple[float, float, float]]]], jump: float) -> Dict[str, float]:
    """samples: (time, the walker's real position, where the OTHER copy shows him). -> error p95 / max, teleports, count (pure, tested alone)."""
    errs, tele, prev = [], 0, None
    for _t, mp, pp in samples:
        if pp is None:
            continue
        errs.append(dist2(mp, pp))
        if prev is not None and dist2(prev, pp) > jump:
            tele += 1
        prev = pp
    errs.sort()
    p95 = errs[min(len(errs) - 1, int(0.95 * len(errs)))] if errs else 0.0
    return {"samples": len(errs), "err_p95": round(p95, 1), "err_max": round(errs[-1], 1) if errs else 0.0, "teleports": tele}


def gauge_leg_mean(g0: Optional[Dict[str, Any]], g1: Optional[Dict[str, Any]]) -> Optional[float]:
    """Mean of an avatar.status gauge {n, sum, ...} over the stretch between two readings (None when nothing was added or the build has no such gauge). Pure, tested alone."""
    if not g1 or "sum" not in g1 or "n" not in g1:
        return None
    n0, s0 = (g0 or {}).get("n", 0), (g0 or {}).get("sum", 0.0)
    dn = g1["n"] - n0
    return None if dn <= 0 else round((g1["sum"] - s0) / dn, 2)


def stage_metrics(mover_st0: Dict[str, Any], mover_st1: Dict[str, Any], viewer_st0: Dict[str, Any], viewer_st1: Dict[str, Any]) -> Dict[str, Any]:
    """Where the time between the walker's sample and the shown puppet goes (plugin/avatar "stages", the round of 2026-10-28), as leg means in ms: send (walker: sample -> handed to the
    transport), rx (viewer: sample -> in its network layer, a shared clock on one PC), transport = rx - send (the wire and the viewer's poll), hold (arrival -> the first drive that could use
    it), delay (the buffer's delay in use), apply (the engine calls of a tick, ms), net_phase (the viewer's network frame minus its avatar frame), frame (the viewer's frame length), plus the
    playback mode, jitter and clock offset of the viewer's buffer. Pure, tested alone."""
    out: Dict[str, Any] = {}

    def put(name: str, v: Optional[float]):
        if v is not None:
            out[name] = v
    send = gauge_leg_mean(mover_st0.get("send"), mover_st1.get("send"))
    rx = gauge_leg_mean(viewer_st0.get("rx"), viewer_st1.get("rx"))
    put("send_ms", send)
    put("rx_ms", rx)
    if send is not None and rx is not None:
        out["transport_ms"] = round(rx + send if K.MUTANT == "stage_transport_sum" else rx - send, 2)   # twin: the wire time adds the sender's send age again
    put("walker_sample_ms", None if gauge_leg_mean(mover_st0.get("sampleUs"), mover_st1.get("sampleUs")) is None else round(gauge_leg_mean(mover_st0.get("sampleUs"), mover_st1.get("sampleUs")) / 1000.0, 3))
    put("walker_frame_ms", gauge_leg_mean(mover_st0.get("frameGap"), mover_st1.get("frameGap")))
    put("viewer_frame_ms", gauge_leg_mean(viewer_st0.get("frameGap"), viewer_st1.get("frameGap")))
    put("net_phase_ms", gauge_leg_mean(viewer_st0.get("netPhase"), viewer_st1.get("netPhase")))
    ap = gauge_leg_mean(viewer_st0.get("applyUs"), viewer_st1.get("applyUs"))
    put("apply_ms", None if ap is None else round(ap / 1000.0, 3))
    put("pose_to_setpos_ms", gauge_leg_mean(viewer_st0.get("poseToSetPosMs"), viewer_st1.get("poseToSetPosMs")))
    r0 = (viewer_st0.get("remotes") or [{}])[0] if viewer_st0.get("remotes") else {}
    r1 = (viewer_st1.get("remotes") or [{}])[0] if viewer_st1.get("remotes") else {}
    if r1:
        put("delay_in_use_ms", gauge_leg_mean(r0.get("delay"), r1.get("delay")))
        put("hold_ms", gauge_leg_mean(r0.get("hold"), r1.get("hold")))
        put("extrapolated_ms", gauge_leg_mean(r0.get("extrap"), r1.get("extrap")))
        put("arrival_gap_ms", gauge_leg_mean(r0.get("arrivalGap"), r1.get("arrivalGap")))
        for k in ("jitterMs", "frameMs", "wantDelayMs", "offsetMs"):
            if k in r1:
                out["buf_" + k] = r1[k]
        out["low_latency"] = 1 if r1.get("lowLatency") else 0
        for k in ("snaps", "teleportSnaps", "eased", "extrapolating", "holding", "samples", "pushes"):
            if k in r1:
                out["buf_" + k] = r1[k] - r0.get(k, 0)
    return out


def trace_lag(local: List[List[float]], remote: List[List[float]], max_ms: int = 600, step_ms: int = 5, min_speed: float = 20.0) -> Dict[str, Any]:
    """The playback delay from the game's own trace (avatar.trace, plugin/avatar), no bridge reads in it: `local` = the walker's published samples [tick_ms, x, y, z, vx, vy, vz], `remote` =
    the viewer's per-frame poses [frame_ms, x, y, z, ax, ay, az, state] (x y z = the pose fed to the puppet, ax ay az = where the engine said the puppet stood at the start of the
    frame, 16 added to the state when it said). Both on one clock (a single PC). The walker's path is linear between his samples; for a shift s the puppet at frame time t is compared with
    the walker at t - s, over the frames where the walker moves (>= min_speed). -> {"lag_ms" (best shift of the fed pose), "err_p95_aligned", "err_p95_now" (shift 0: the real error,
    lag included), "engine_lag_ms" (best shift of the engine's own position), "n"}. Pure, tested alone."""
    if len(local) < 3 or len(remote) < 3:
        return {"lag_ms": None, "err_p95_aligned": None, "err_p95_now": None, "engine_lag_ms": None, "n": 0}
    lt = [p[0] for p in local]

    def walker(t: float) -> Optional[Tuple[float, float, float, float]]:
        if t < lt[0] or t > lt[-1]:
            return None
        lo, hi = 0, len(lt) - 1
        while hi - lo > 1:
            mid = (lo + hi) // 2
            if lt[mid] <= t:
                lo = mid
            else:
                hi = mid
        a, b = local[lo], local[hi]
        k = 0.0 if b[0] == a[0] else (t - a[0]) / (b[0] - a[0])
        sp = math.hypot(b[1] - a[1], b[2] - a[2]) / max(1e-3, (b[0] - a[0]) / 1000.0)
        return (a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k, a[3] + (b[3] - a[3]) * k, sp)

    def sweep(cols: Tuple[int, int, int], need_flag: bool):
        best = None
        errs0: List[float] = []
        for shift in range(0, max_ms + 1, step_ms):
            errs = []
            for r in remote:
                if need_flag and not (int(r[7]) & 16):
                    continue
                w = walker(r[0] + shift if K.MUTANT == "trace_lag_wrong_sign" else r[0] - shift)   # twin: the puppet is compared with where the walker WILL be
                if w is None or w[3] < min_speed:
                    continue
                errs.append(math.hypot(w[0] - r[cols[0]], w[1] - r[cols[1]]))
            if len(errs) < 5:
                continue
            if shift == 0:
                errs0 = sorted(errs)
            mean = sum(errs) / len(errs)
            if best is None or mean < best[0]:
                best = (mean, shift, sorted(errs))
        return best, errs0
    fed, e0 = sweep((1, 2, 3), False)
    eng, _ = sweep((4, 5, 6), True)

    def p95(v: List[float]) -> float:
        return round(v[min(len(v) - 1, int(0.95 * len(v)))], 1)
    if fed is None:
        return {"lag_ms": None, "err_p95_aligned": None, "err_p95_now": None, "engine_lag_ms": None if eng is None else eng[1], "n": 0}
    return {"lag_ms": fed[1], "err_p95_aligned": p95(fed[2]), "err_p95_now": p95(e0) if e0 else None, "engine_lag_ms": None if eng is None else eng[1], "n": len(fed[2])}


def lag_estimate(rows: List[Tuple[float, Tuple[float, float, float], float, Optional[Tuple[float, float, float]]]], max_ms: int = 800, step_ms: int = 10) -> Dict[str, Any]:
    """rows: (time the walker's position was read, that position, time the puppet's position was read, that position). The puppet shows the walker as he was `lag` ago: the shift
    (0..max_ms) that makes the puppet's positions closest to the walker's path (linear between his samples) is the playback delay, free of the time the two bridge reads took
    (each read of a position is a few frames; they come one after the other). -> {"lag_ms", "err_p95" at that shift, "skew_ms" median time between the two reads, "n"}. Pure, tested alone."""
    mv = sorted((tm, mp) for tm, mp, _tp, pp in rows if pp is not None)
    pts = [(tp, pp, tm) for tm, _mp, tp, pp in rows if pp is not None]
    if len(mv) < 3:
        return {"lag_ms": None, "err_p95": None, "skew_ms": None, "n": len(mv)}

    def at(t: float) -> Optional[Tuple[float, float, float]]:
        if t < mv[0][0] or t > mv[-1][0]:
            return None
        for (t0, a), (t1, b) in zip(mv, mv[1:]):
            if t0 <= t <= t1:
                k = 0.0 if t1 == t0 else (t - t0) / (t1 - t0)
                return tuple(a[i] + (b[i] - a[i]) * k for i in range(3))
        return None
    best = None
    for shift in range(0, max_ms + 1, step_ms):
        errs = []
        for tp, pp, tm in pts:
            when = (tm if K.MUTANT == "lag_est_ignores_read_times" else tp) - shift / 1000.0      # twin: the puppet read is stamped with the walker's read time
            m = at(when)
            if m is not None:
                errs.append(dist2(m, pp))
        if len(errs) < 3:
            continue
        mean = sum(errs) / len(errs)
        if best is None or mean < best[0]:
            errs.sort()
            best = (mean, shift, errs[min(len(errs) - 1, int(0.95 * len(errs)))], len(errs))
    skews = sorted(tp - tm for tp, _pp, tm in pts)
    skew = skews[len(skews) // 2] * 1000.0 if skews else None
    if best is None:
        return {"lag_ms": None, "err_p95": None, "skew_ms": None if skew is None else round(skew), "n": len(pts)}
    return {"lag_ms": best[1], "err_p95": round(best[2], 1), "skew_ms": None if skew is None else round(skew), "n": best[3]}


def room_walk(ctx: Ctx) -> Dict[str, Any]:
    """Host walks the waypoints, then the client does: the other copy's puppet follows without error spikes, teleports or lag."""
    wps = ctx.ref("wp", "waypoint")
    ctx.stand_all()
    ctx.refs_exist(wps + [ctx.layout.stand("walk")], "the walk waypoints")
    pts = [ctx.host.pos(w) for w in wps]
    out: Dict[str, Any] = {}
    for mover, obs in ((ctx.host, ctx.client), (ctx.client, ctx.host)):
        samples: List[Tuple[float, Any, Any]] = []
        leg_t0 = ctx.now()
        ctx.fps_sample(True)
        cnt0 = {mover.name: mover.avatar_counts(), obs.name: obs.avatar_counts()}
        st0 = {mover.name: mover.avatar_stages(), obs.name: obs.avatar_stages()}

        timed_rows: List[Tuple[float, Any, float, Any]] = []

        def tick():
            t0 = ctx.now()
            mp = mover.pos()
            t1 = ctx.now()
            pp = obs.puppet_pos()
            t2 = ctx.now()
            samples.append((t0, mp, pp))
            timed_rows.append(((t0 + t1) / 2.0, mp, (t1 + t2) / 2.0, pp))
        for i, (x, y, z) in enumerate(pts, 1):
            ok, why = mover.walk_to(x, y, z, ctx.p("leg_timeout_s"), tick)
            ctx.check(f"{mover.name} walks to waypoint {i}", ok, "arrived", why or "arrived", cls="TEST", area="tools/lab (walking), plugin/bots")
        st = walk_stats(samples, ctx.p("jump"))
        ctx.metrics.update({f"{mover.name}_err_p95": st["err_p95"], f"{mover.name}_err_max": st["err_max"]})
        # the delay itself, free of the bridge reads: the shift that matches the puppet to the walker's path (lag_estimate), its error p95, and the time between the two reads
        le = lag_estimate(timed_rows)
        for k, name in (("lag_ms", "lag_est_ms"), ("err_p95", "err_p95_aligned"), ("skew_ms", "read_skew_ms")):
            if le.get(k) is not None:
                ctx.metrics[f"{mover.name}_{name}"] = le[k]
        # the error against the frame rate: a position is sampled, sent, received and applied once per game frame, so the walker's copy and the viewer's copy both bound the error
        ctx.fps_sample(True)
        fw, fv = ctx.fps.since(mover.name, leg_t0).get("mean"), ctx.fps.since(obs.name, leg_t0).get("mean")
        if fw is not None and fv is not None:
            ctx.metrics.update({f"{mover.name}_leg_fps_walker": fw, f"{mover.name}_leg_fps_viewer": fv, f"{mover.name}_leg_fps": min(fw, fv)})
        # the stream's own rates over the leg: snapshots the walker published and the viewer received per second (30 with the 30 Hz near rate of the lag round, 20 before it,
        # never more than the game's fps), and the poses fed to the puppet per second
        secs = max(0.001, ctx.now() - leg_t0)
        cnt1 = {mover.name: mover.avatar_counts(), obs.name: obs.avatar_counts()}
        st1 = {mover.name: mover.avatar_stages(), obs.name: obs.avatar_stages()}
        try:       # the game's own trace of the leg: the walker's published path against the puppet's fed poses, on one clock
            tw, tv = mover.avatar_trace(450), obs.avatar_trace(450)
            if os.environ.get("LAB_TRACE_DIR"):      # the raw traces of the leg, for a look at the time series (frame, pose, delay in use, wanted delay, jitter, newest age)
                os.makedirs(os.environ["LAB_TRACE_DIR"], exist_ok=True)
                with open(os.path.join(os.environ["LAB_TRACE_DIR"], f"trace_{mover.name}.json"), "w") as fh:
                    json.dump({"walker": tw, "viewer": tv}, fh)
            tl = trace_lag(tw.get("local") or [], next(iter((tv.get("remote") or {}).values()), []))
            for k, name in (("lag_ms", "trace_lag_ms"), ("err_p95_aligned", "trace_err_p95_aligned"), ("err_p95_now", "trace_err_p95_now"), ("engine_lag_ms", "trace_engine_lag_ms")):
                if tl.get(k) is not None:
                    ctx.metrics[f"{mover.name}_{name}"] = tl[k]
        except Exception as e:    # noqa: BLE001 - a build without avatar.trace, a fake lab: the leg's own metrics stand
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
        for k, v in stage_metrics(st0[mover.name], st1[mover.name], st0[obs.name], st1[obs.name]).items():
            ctx.metrics[f"{mover.name}_leg_{k}"] = v
        for key, who, field in (("published_hz", mover.name, "published"), ("received_hz", obs.name, "remote"), ("fed_hz", obs.name, "fed")):
            if field in cnt0[who] and field in cnt1[who]:
                ctx.metrics[f"{mover.name}_leg_{key}"] = round((cnt1[who][field] - cnt0[who][field]) / secs, 1)
        ctx.check(f"the {obs.name} copy saw the {mover.name} walk", st["samples"] >= ctx.p("min_samples"), f">= {ctx.p('min_samples'):.0f} samples",
                  st["samples"], "no puppet to measure" if not st["samples"] else "", cls="SYNC", area="plugin/avatar")
        ctx.check(f"{mover.name}'s puppet on the {obs.name} copy: error p95", st["err_p95"] <= ctx.p("err_p95"), f"<= {ctx.p('err_p95'):.0f}", st["err_p95"],
                  area="plugin/avatar, plugin/avatar/engine")
        ctx.check(f"{mover.name}'s puppet on the {obs.name} copy: error max", st["err_max"] <= ctx.p("err_max"), f"<= {ctx.p('err_max'):.0f}", st["err_max"],
                  area="plugin/avatar, plugin/avatar/engine")
        ctx.check(f"{mover.name}'s puppet on the {obs.name} copy: teleports", st["teleports"] == 0, 0, st["teleports"], area="plugin/avatar, plugin/avatar/engine")
        near = ctx.p("near")

        def settled():
            pp = obs.puppet_pos()
            d = None if pp is None else dist2(pp, mover.pos())
            return (d is not None and d <= near), (None if d is None else round(d, 1))
        _, lag = ctx.wait(f"{mover.name}'s puppet catches up after he stops", settled, ctx.p("lag_s"), f"within {near:.0f} units", poll_s=0.05, area="plugin/avatar")
        out[f"{mover.name}_lag_ms"] = int(round(lag * 1000))
    return out


# ------------------------------------------------------------------ fight
def room_fight(ctx: Ctx) -> Dict[str, Any]:
    """Three hostiles: the host kills one, the client kills one; both copies see both deaths, nothing gets up, the third lives on both."""
    en = ctx.ref("enemy")[:3]
    ctx.stand_all()
    ctx.refs_exist(en + [ctx.layout.stand("fight")], "the enemies")
    for m in ctx.ms:
        ok, why = m.arm()
        ctx.check(f"{m.name} is armed (weapon, ammo, health)", ok, "armed", why or "armed", cls="TEST", area="tools/lab (arming)")
    volleys = int(ctx.p("volleys"))
    stand = ctx.layout.stand("fight")
    for shooter, other, e, idx in ((ctx.host, ctx.client, en[0], 1), (ctx.client, ctx.host, en[1], 2)):
        # one hostile at a time: enabled together they all charge the players, and by the client's turn a gecko stood inside aim_at's 100 units
        # (first in-game run: "the player is 60 units from the target")
        ctx.enable_hostiles([e])
        if K.MUTANT != "fight_target_free":     # twin: the target is left free and drifts away from the shooter
            # Behind bad_wifi and transatlantic the second target drifted to 659 and 875 units from the client, out of the pistol's range, and no shot was possible
            # ("no clear shot even from the standoff", lab run 2026-10-28). The host (owner) restrains it where it stands, as the vats room does; the shot and the death
            # that has to cross the link are still real.
            ok, why = ctx.host.hold(e)
            ctx.check(f"the host holds enemy {idx} in place", ok, "held", why or "held", cls="TEST", area="tools/lab (SetRestrained)")
        ctx.agree(f"enemy {idx} is alive on both copies", lambda m, e=e: m.alive(e), 15.0, True, area="plugin/actors, plugin/spawn")
        # the host, as owner of the actor, makes it weak; the shooter's copy must show that before it shoots (a hit on a 65 health gecko took 8 of 24
        # volleys in the first in-game run: the pistol hits a moving gecko now and then)
        if K.MUTANT != "no_weaken":     # twin: the targets are shot at full health again
            ok, why = ctx.host.set_health(e, ctx.p("weak_hp"))
            ctx.check(f"the host weakens enemy {idx}", ok, "health set", why or "health set", cls="TEST", area="tools/lab (SetAV Health)")
            ctx.agree(f"enemy {idx} shows the weakened health on both copies", lambda m, e=e: m.health(e) <= ctx.p("weak_hp") + 1.0, ctx.p("weak_s"), True,
                      area="plugin/actors (owner's health to the other copy)")
        # watchers on both copies time the death from before the first shot: the kill is the moment the shooter's copy first shows it dead, the latency the time until
        # the other copy shows it too (both polled on the runner's clock, ~30 ms apart), however many volleys the shooter needed
        (ok, why), tm = ctx.timed(lambda: shooter.shoot(e, volleys, stand), lambda: not shooter.alive(e), lambda: not other.alive(e), ctx.p("latency_s"))
        # A shot that killed nothing names the combat counters of BOTH copies (fight behind transatlantic, run 20261028-032920: 24 volleys spent ammo and the target stayed at 5):
        # the shooter's gate (clamped / neutralized / hitsSent) and the owner's receiving end (hitsReceived / hitsRejected / rejections) say where the hit went.
        words = ""
        if not ok:
            words = " | ".join(f"{m.name} combat: {m.combat_words()}" for m in ctx.ms)
            try:   # where everything stood: a shot that hits nothing is also a shot at the wrong place (the target as each copy sees it, the shooter, the angles)
                where = ", ".join(f"{m.name} copy has it at {tuple(round(x) for x in m.pos(e))}" for m in ctx.ms)
                words += f" | target {e.split(':')[1]}: {where}; {shooter.name} stands at {tuple(round(x) for x in shooter.pos())}"
            except Exception as ex:   # noqa: BLE001
                if type(ex).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
        ctx.check(f"{shooter.name} kills enemy {idx}", ok, "dead on the shooter's copy", why or "dead", note=words, cls="TEST", area="tools/lab (shooting), plugin/combat")
        ctx.metric(f"{shooter.name}_kill_ms", ctx.latency_check(f"enemy {idx} killed by the {shooter.name} is dead on the {other.name} copy", tm, ctx.p("latency_s"), "dead on both copies",
                                                                 area="plugin/combat, plugin/actors", show=lambda: ctx.both(lambda m: [m.alive(e), m.health(e)])))
    ctx.enable_hostiles([en[2]])
    ctx.agree("deaths agree: enemies 1 and 2 dead, enemy 3 alive, on both copies", lambda m: [m.alive(e) for e in en], 10.0, [False, False, True], area="plugin/combat, plugin/actors")
    ctx.agree("the survivor has the same health on both copies", lambda m: round(m.health(en[2])), 10.0, area="plugin/combat, plugin/actors")
    end = ctx.now() + ctx.p("corpse_s")
    seen: Dict[str, Any] = {}
    while ctx.now() < end:
        ctx.sleep(1.0)
        for m in ctx.ms:
            a = [m.alive(e) for e in en]
            if a != [False, False, True] and m.name not in seen:
                seen[m.name] = a
    ctx.check("no corpse gets up and the survivor stays alive for the watch window", not seen, [False, False, True], seen or [False, False, True],
              area="plugin/combat, plugin/actors (revive / stale echo)")
    return {}


# ------------------------------------------------------------------ loot
def room_loot(ctx: Ctx) -> Dict[str, Any]:
    """A container with known items and four loose items: takes and pickups on both sides, a race for one: no dupes, nothing lost, totals match."""
    cont = ctx.ref("container", "chest", n=0)
    floors = ctx.ref("item", "loose", "floor")[:4]
    ci = ctx.layout.container_items() or {DEFAULT_FLOOR_ITEM: 5}
    item_c = next(iter(ci))
    bases = ctx.layout.loose_bases()
    if len(bases) < len(floors):
        bases = [ctx.layout.extra("loot", "floor_item") or DEFAULT_FLOOR_ITEM] * len(floors)
    ctx.stand_all()
    ctx.refs_exist([cont] + floors, "the container and the loose items")
    c0 = ctx.agree("the container holds the same count of the item on both copies", lambda m: m.item_count(item_c, cont), 10.0, area="plugin/loot")
    ctx.check("the container holds what the plugin put in it", c0 >= 3 and c0 == ci[item_c], ci[item_c], c0, cls="TEST", area="tools/lab/esp")
    ctx.agree("the four loose items are in the world on both copies", lambda m: [m.enabled(r) for r in floors], 10.0, [True] * 4, area="plugin/loot")
    base = {m.name: {it: m.item_count(it) for it in set([item_c] + bases)} for m in ctx.ms}
    settle = ctx.p("settle_s")

    def gains(m, item):
        return m.item_count(item) - base[m.name][item]
    taken = 0
    want = {"host": 0, "client": 0}
    for taker, n in ((ctx.host, 2), (ctx.client, 1)):
        can, why_not = taker.can_take()
        if not can:
            ctx.skip(f"{taker.name} takes {n} from the container", why_not)
            continue
        giver = ctx.client if taker is ctx.host else ctx.host
        left = c0 - taken - n
        (ok, why), tm = ctx.timed(lambda: taker.take(cont, item_c, n), lambda: taker.item_count(item_c, cont) == left, lambda: giver.item_count(item_c, cont) == left, settle)
        ctx.check(f"{taker.name} takes {n} from the container", ok, "taken", why or "taken", cls="SYNC", area="plugin/loot, plugin/intents")
        taken += n
        want[taker.name] = n
        if getattr(taker, "take_note", ""):
            ctx.metrics[f"{taker.name}_take_note"] = taker.take_note
        ctx.metric(f"{taker.name}_take_ms", ctx.latency_check(f"container count after the {taker.name}'s take is the same on both copies", tm, settle, c0 - taken, area="plugin/loot",
                                                              show=lambda: ctx.both(lambda m: m.item_count(item_c, cont))))
    got = {"host": gains(ctx.host, item_c), "client": gains(ctx.client, item_c)}
    ctx.check("takers got exactly what they took, nobody else got anything", got == want, want, got, area="plugin/loot (dupe)")
    for picker, idx in ((ctx.host, 0), (ctx.client, 1)):
        g0 = picker.item_count(bases[idx])
        seer = ctx.client if picker is ctx.host else ctx.host
        (ok, why), tm = ctx.timed(lambda: picker.pickup(floors[idx]), lambda: not picker.enabled(floors[idx]), lambda: not seer.enabled(floors[idx]), settle)
        ctx.check(f"{picker.name} picks up loose item {idx + 1}", ok, "picked up", why or "picked up", area="plugin/loot, plugin/intents")
        ctx.metric(f"{picker.name}_pickup_ms", ctx.latency_check(f"loose item {idx + 1} is gone on both copies", tm, settle, "gone on both copies", area="plugin/loot",
                                                                 show=lambda: ctx.both(lambda m: m.enabled(floors[idx]))))
        # The count is read for up to a second: a gain that arrives a few frames after the world ref is gone is not a failure, a gain that never comes is. The trace
        # (ms since the pickup returned: count gained so far) is kept in the check's note, and the pickup counters of both copies when it fails.
        d, trace = 0, []
        t0 = ctx.now()
        while True:
            d = picker.item_count(bases[idx]) - g0
            trace.append((int((ctx.now() - t0) * 1000), d))
            if d >= 1 or ctx.now() - t0 >= 1.0:
                break
            ctx.sleep(0.1)
        ctx.check(f"only the {picker.name} gained loose item {idx + 1}", d == 1, 1, d, note=f"gain trace {trace[:12]}" if d != 1 else "", area="plugin/loot (dupe)")
    before = {m.name: m.item_count(bases[2]) for m in ctx.ms}
    ctx.par(lambda: ctx.host.pickup(floors[2]), lambda: ctx.client.pickup(floors[2]))
    ctx.agree("the raced loose item 3 is gone on both copies", lambda m: m.enabled(floors[2]), settle, False, area="plugin/loot (race)")
    ctx.sleep(2.0)
    won = {m.name: m.item_count(bases[2]) - before[m.name] for m in ctx.ms}
    ctx.check("the race gave loose item 3 to exactly one player", sum(won.values()) == 1 and all(v in (0, 1) for v in won.values()), "one winner, 1 item", won,
              area="plugin/loot (dupe / lost item)")
    ctx.agree("the untouched loose item 4 is still there on both copies", lambda m: m.enabled(floors[3]), 5.0, True, area="plugin/loot")
    return {"race_winner": max(won, key=won.get)}


# ------------------------------------------------------------------ door
def room_door(ctx: Ctx) -> Dict[str, Any]:
    """A locked door and its terminal: a locked door stays shut, the terminal unlocks it, then it opens and closes from either side, the same on both copies."""
    door, term = ctx.ref("door", n=0), ctx.ref("terminal", n=0)
    entry = int(ctx.layout.extra("door", "terminal_entry", 0))      # the stock unlock terminal has one entry, "Disengage Lock"
    settle = ctx.p("settle_s")
    ctx.stand_all()
    ctx.refs_exist([door, term], "the door and the terminal")
    ctx.agree("the door is closed on both copies", lambda m: m.door_state(door), settle, "closed", area="plugin/hooks_world, plugin/intents")
    locked = ctx.agree("the door has the same lock state on both copies", lambda m: m.locked(door), settle, area="plugin/hooks_world, plugin/intents")
    if not locked:
        raise Blocked("door room: the door must start locked (the terminal unlocks it); the plugin placed it unlocked", "TEST")
    ok, _ = ctx.host.open_door(door)
    ctx.check("the host cannot open the locked door", not ok, False, ok, area="plugin/intents")
    ctx.close_menus([ctx.host])                  # a copy that activated the locked door has the lockpick menu open; it would eat the next open
    ctx.agree("the locked door is still closed on both copies", lambda m: m.door_state(door), 3.0, "closed", area="plugin/hooks_world")
    # timed from before the click: the first copy to show the door unlocked is the one the script ran on (the host, for a guest's click); the time until the OTHER copy shows
    # it too is the lock sync
    (ok, why), tm_unlock = ctx.timed(lambda: ctx.client.use_terminal(term, entry), lambda: any(not m.locked(door) for m in ctx.ms), lambda: all(not m.locked(door) for m in ctx.ms), settle)
    ctx.check("the client uses the terminal to unlock the door", ok, "entry used", why or "entry used", area="plugin/intents (terminal)")
    # Control: the click must have run the entry's unlock SOMEWHERE. A guest's click does not run the result script on its own copy: the TerminalEntryRun hook
    # forwards it as an intent and the host runs it (measured 2026-10-27: after the client's click the host showed the door unlocked within a second while the
    # client's copy stayed locked for 15 s). So the control looks at both copies; when neither unlocks, the click did nothing (a TEST problem, not a sync one),
    # and the next check, unlocked on both, is the SYNC check of the lock state.
    ctx.wait("the terminal click unlocked the door on at least one copy (if neither, the click unlocked nothing: not a sync problem)",
             lambda: (any(not m.locked(door) for m in ctx.ms), [m.locked(door) for m in ctx.ms]), 3.0, True, cls="TEST",
             area="plugin/bots (dev.terminal click), tools/lab/esp (terminal)")
    ctx.metric("unlock_ms", ctx.latency_check("the door is unlocked on both copies", tm_unlock, settle, "unlocked on both copies", area="plugin/hooks_world, plugin/intents (lock)",
                                              show=lambda: ctx.both(lambda m: m.locked(door))))
    for opener, closer in ((ctx.host, ctx.client), (ctx.client, ctx.host)):
        ok, why = opener.open_door(door)
        ctx.check(f"{opener.name} opens the door", ok, "opened", why or "opened", area="plugin/intents, plugin/hooks_world")
        ctx.metric(f"{opener.name}_open_ms", ctx.latency(f"the door the {opener.name} opened is open on both copies", lambda: opener.door_state(door) == "open",
                                                          lambda: closer.door_state(door) == "open", settle, "open on both copies", area="plugin/hooks_world",
                                                          show=lambda: ctx.both(lambda m: m.door_state(door))))
        ok, why = closer.close_door(door)
        ctx.check(f"{closer.name} closes the door", ok, "closed", why or "closed", area="plugin/intents, plugin/hooks_world")
        ctx.metric(f"{closer.name}_close_ms", ctx.latency(f"the door the {closer.name} closed is closed on both copies", lambda: closer.door_state(door) == "closed",
                                                           lambda: opener.door_state(door) == "closed", settle, "closed on both copies", area="plugin/hooks_world",
                                                           show=lambda: ctx.both(lambda m: m.door_state(door))))
    return {}


# ------------------------------------------------------------------ npc
def _owner_view(m, npc):
    o = m.owner(npc)
    return {"owner": o.get("owner"), "epoch": o.get("epoch"), "mine": bool(o.get("mine")), "known": bool(o.get("known", True))}


def room_npc(ctx: Ctx) -> Dict[str, Any]:
    """A friendly NPC on a patrol. Ownership is the HOST while it is eligible (in or next to the NPC's cell), else the nearest player
    (docs/FULL-SYNC-PLAN.md 1.2 and 3.2, synccore pickOwner). The whole lab is one interior cell, so no distance inside it moves the NPC away from the host:
    the room (1) keeps the client right next to the NPC and the host far away and expects the host to keep it, (2) sends the host into another stock interior
    and expects the client to take it (new epoch), (3) brings the host back to the lab and expects it to take it back (new epoch).
    While the host is away its copy has the NPC unloaded, so that copy is not sampled."""
    npc = ctx.ref("npc", n=0)
    stats0 = {m.name: m.actor_stats() for m in ctx.ms}
    stand = ctx.layout.stand("npc")
    away_cell = str(ctx.layout.extra("npc", "away_cell", AWAY_CELL))
    lab_cells = [ctx.layout.cell_edid] + ([ctx.layout.cell_alt] if getattr(ctx.layout, "cell_alt", "") else [])
    ctx.stand_all()
    ctx.refs_exist([npc, stand], "the NPC and its stand marker")
    ctx.agree("the NPC is alive on both copies", lambda m: m.alive(npc), 10.0, True, area="plugin/actors")
    # the host waits at the origin of the room farthest from the NPC; the client stands next to the NPC
    npc_at = ctx.host.pos(npc)
    spots = [(dist3(o, npc_at), o) for r in ctx.layout.rooms if r != "npc" and (o := ctx.layout.origin(r))]
    far, far_at = max(spots, default=(0.0, None))
    if far < ctx.p("min_far"):
        raise Blocked(f"npc room: the farthest room is {far:.0f} units from the NPC, the host must stand at least {ctx.p('min_far'):.0f} away (plugin layout)", "TEST")
    traces: Dict[str, List[Optional[Tuple[float, ...]]]] = {HOST: [], CLIENT: []}
    times: Dict[str, List[float]] = {HOST: [], CLIENT: []}     # when each sample was read (the room's clock): the samples are not evenly spaced
    where: Dict[str, List[str]] = {HOST: [], CLIENT: []}       # which stage of the room each sample belongs to
    stage = ["hold"]
    own_note = [""]                  # the newest ownership views of both copies (owner/epoch/pending), kept with every sample so the worst pair says who owned the NPC then
    present = {HOST: True, CLIENT: True}      # a copy that is not in the lab cell has the NPC unloaded: no sample

    last_at = [-1.0]

    def sample(every: float = 0.0):
        if every and ctx.now() - last_at[0] < every:
            return
        last_at[0] = ctx.now()
        for m in ctx.ms:
            traces[m.name].append(tuple(m.pos(npc)) if present[m.name] else None)
            times[m.name].append(ctx.now())
            where[m.name].append(stage[0] + own_note[0])

    def watch(secs):
        end = ctx.now() + secs
        while ctx.now() < end:
            sample()
            ctx.sleep(0.5)
    epochs: List[int] = []

    def views():
        return {m.name: _owner_view(m, npc) for m in ctx.ms}

    def agreed(v, owner):
        mine = owner == HOST
        return (all(x["known"] for x in v.values()) and v[HOST]["mine"] == mine and v[CLIENT]["mine"] == (not mine) and v[HOST]["epoch"] == v[CLIENT]["epoch"])

    marks: Dict[str, float] = {}

    def moved_to(who, tm=None):
        def settled():
            sample()                                   # the snap of a handoff happens inside this wait, so the NPC is watched here too
            v = views()
            own_note[0] = " [" + " ".join(f"{k[0]}:{x['owner']}/e{x['epoch']}" for k, x in v.items()) + "]"
            return agreed(v, who) and (not epochs or (isinstance(v[HOST]["epoch"], int) and v[HOST]["epoch"] > epochs[-1])), v
        v, _ = ctx.wait(f"ownership of the NPC moves to the {who} with a new epoch and both copies agree", settled, ctx.p("handoff_s"), {"owner": who},
                        poll_s=0.2, area="plugin/actors, synccore ownership (pickOwner)")
        epochs.append(v[HOST]["epoch"] if isinstance(v[HOST]["epoch"], int) else 0)
        # the handoff time: from the host's copy leaving (or entering) the lab cell to the NEW owner's own copy saying it owns the NPC, timed by watchers from before the coc
        ms = ctx.ms_of(tm) if tm else None
        ctx.metric(f"npc_handoff_{who}_ms", ms if ms is not None else int(round(ctx.last_wait_s * 1000)))
        # Where the time goes (lab run 2026-10-28: 1030 ms behind good and bad Wi-Fi, 78 ms clean): ms from the start of the host's coc to (coc) the call returning, (left) the host's
        # copy first showing the cell change, (owns) the new owner's copy first saying it owns the NPC. The host's copy answers no bridge call while it loads a cell, so any number that
        # needs the HOST's copy to answer cannot be smaller than its load.
        if tm and marks.get("t0") is not None:
            for k, key in (("coc", "coc"), ("left", "src"), ("owns", "dst")):
                t = marks.get("coc") if k == "coc" else (tm or {}).get(key)
                if t is not None:
                    ctx.metric(f"npc_{who}_{k}_ms", int(round((t - marks["t0"]) * 1000)))

    # 1. the client next to the NPC, the host far away in the same cell: the eligible host keeps it
    ok, why = ctx.client.goto_ref(npc, 150.0)
    ctx.check("the client walks up to the NPC", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (teleport)")
    ok, why = ctx.host.goto_pos(*far_at)
    ctx.check("the host waits far from the NPC", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (teleport)")
    v, _ = ctx.wait("the host owns the NPC while it is in the lab, however far it is and however near the client", lambda: (lambda v: (agreed(v, HOST), v))(views()),
                    ctx.p("handoff_s"), {"owner": HOST}, poll_s=0.2, area="plugin/actors, synccore ownership (pickOwner: an eligible host owns)")
    epochs.append(v[HOST]["epoch"] if isinstance(v[HOST]["epoch"], int) else 0)
    sample()
    end = ctx.now() + ctx.p("hold_s")
    stolen = None
    while ctx.now() < end and stolen is None:
        sample()
        ctx.sleep(0.5)
        v = views()
        if not agreed(v, HOST) or v[HOST]["epoch"] != epochs[-1]:
            stolen = v
    ctx.check("the host keeps the NPC for the whole hold (the client is nearer, the host is eligible)", stolen is None, {"owner": HOST, "epoch": epochs[-1]}, stolen or "kept",
              area="plugin/actors, synccore ownership (pickOwner)")
    # 2. the host leaves for a stock interior: the client takes the NPC
    lab_cell = ctx.host.cell_of()
    base_epoch = epochs[-1]
    stage[0] = "host leaves"

    def moved_now(who):
        sample(0.25)          # the host's coc takes seconds: the NPC is watched all the while, or a snap hides in one long gap between two samples
        # The NEW owner's own copy says it owns the NPC under a newer epoch. (Asking the old owner's copy too made the number the length of that copy's cell load: it answers
        # nothing meanwhile. The wait in moved_to still needs both copies to agree.)
        if K.MUTANT == "npc_handoff_waits_for_both":          # twin: both copies must answer, as before
            v = views()
            return agreed(v, who) and isinstance(v[HOST]["epoch"], int) and v[HOST]["epoch"] > base_epoch
        v = _owner_view(ctx.client if who == CLIENT else ctx.host, npc)
        return bool(v["known"] and v["mine"] and isinstance(v["epoch"], int) and v["epoch"] > base_epoch)

    def leaving():
        marks["t0"] = ctx.now()
        r = ctx.host.coc([away_cell])
        marks["coc"] = ctx.now()
        return r
    (ok, why), tm = ctx.timed(leaving, lambda: ctx.host.cell_of() != lab_cell, lambda: moved_now(CLIENT), ctx.p("handoff_s"))
    ctx.check(f"the host goes to {away_cell}", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (coc)")
    present[HOST] = False
    moved_to(CLIENT, tm)
    watch(ctx.p("watch_s"))
    # 3. the host comes back to the lab and the NPC returns to it
    base_epoch = epochs[-1]
    stage[0] = "host returns"

    def back():
        marks["t0"] = ctx.now()
        r = ctx.host.coc(lab_cells)
        marks["coc"] = ctx.now()
        return r, (ctx.host.goto_ref(stand) if r[0] else (False, "not back"))
    ((ok, why), (ok2, why2)), tm = ctx.timed(back, lambda: ctx.host.cell_of() == lab_cell, lambda: moved_now(HOST), ctx.p("handoff_s"))
    ctx.check("the host returns to the lab cell", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (coc)")
    ctx.check("the host stands at the NPC room's stand marker", ok2, "arrived", why2 or "arrived", cls="ENV", area="tools/lab (teleport)")
    present[HOST] = True
    moved_to(HOST, tm)
    watch(ctx.p("watch_s"))
    # A step is judged by what is left of it after the patrol's own walk in the time between the two samples (npc.speed): the samples are not evenly spaced
    # (a bridge call on a busy PC takes 0.3 s, and the wait loops sample as often as they poll), so a fixed step limit calls a plain walk a snap.
    # Each step keeps its distance, its time, the copy and the stage of the room, so a real snap is told from a slow sample in the failure text.
    steps: List[Tuple[float, float, float, str, str]] = []     # (excess over the walk, distance, seconds, copy, stage)
    for name, tr in traces.items():
        for i in range(1, len(tr)):
            a, b = tr[i - 1], tr[i]
            if a is None or b is None:
                continue
            d, dt = dist3(a, b), max(0.0, times[name][i] - times[name][i - 1])
            steps.append((d if K.MUTANT == "npc_step_fixed_limit" else d - ctx.p("speed") * min(dt, 1.5), d, dt, name, where[name][i]))    # twin: the walk is not subtracted
    worst = max(steps, default=(0.0, 0.0, 0.0, "", ""))
    ctx.metric("npc_max_step", round(max((s[1] for s in steps), default=0.0), 1))
    ctx.metric("npc_max_speed", round(max((s[1] / s[2] for s in steps if s[2] > 0.05), default=0.0), 1))
    ctx.check("the NPC never snaps (largest step between samples, less the patrol's walk in that time)", worst[0] <= ctx.p("jump"), f"<= {ctx.p('jump'):.0f}",
              f"{worst[0]:.1f} ({worst[1]:.0f} units in {worst[2]:.2f} s on the {worst[3] or '-'} copy, {worst[4] or '-'})", area="plugin/actors (handoff snap / Reset3D)")
    both = [(a, b) for a, b in zip(traces[HOST], traces[CLIENT]) if a is not None and b is not None]
    err = max((dist3(a, b) for a, b in both), default=0.0)
    # Where the worst pair came from (lab run 20261028-032920: 178.8 units once, 25 to 37 in every other run): the stage of the room, how far apart in time the two copies' reads were
    # (a bridge call that waited for a frame, or for a cell load, moves one read by up to a second = 200 units of patrol), and the error when the client's path is read at the
    # host's time (`npc_err_aligned`: the part of the error that is not the two reads being apart).
    idx = [i for i, (a, b) in enumerate(zip(traces[HOST], traces[CLIENT])) if a is not None and b is not None]
    if idx:
        w = max(idx, key=lambda i: dist3(traces[HOST][i], traces[CLIENT][i]))
        ctx.metric("npc_err_stage", where[HOST][w])
        ctx.metric("npc_err_reads_apart_ms", int(round((times[CLIENT][w] - times[HOST][w]) * 1000)))
        ctx.metric("npc_err_at_s", round(times[HOST][w], 2))
        ctx.metric("npc_err_pair", f"host {tuple(round(x) for x in traces[HOST][w])} client {tuple(round(x) for x in traces[CLIENT][w])}")
        aligned = 0.0
        for i in idx:
            t_h, ct = times[HOST][i], times[CLIENT]
            a = traces[HOST][i]
            b = traces[CLIENT][i]
            j = i - 1 if t_h < ct[i] else i + 1       # the client sample on the other side of the host's read time, if there is one
            if 0 <= j < len(ct) and traces[CLIENT][j] is not None and ct[j] != ct[i]:
                f = (t_h - ct[i]) / (ct[j] - ct[i])
                if 0.0 <= f <= 1.0:
                    b = tuple(x + (y - x) * f for x, y in zip(traces[CLIENT][i], traces[CLIENT][j]))
            aligned = max(aligned, dist3(a, b))
        ctx.metric("npc_err_aligned", round(aligned, 1))
    ctx.check("the two copies show the NPC at the same place while both have it loaded (max position error)", err <= ctx.p("err_max"), f"<= {ctx.p('err_max'):.0f}", round(err, 1),
              area="plugin/actors (position sync)")
    moved = max((sum(dist3(a, b) for a, b in zip(tr, tr[1:]) if a is not None and b is not None) for tr in traces.values()), default=0.0)
    ctx.check("the NPC patrols (it moved while watched)", moved >= ctx.p("min_moved"), f">= {ctx.p('min_moved'):.0f}", round(moved, 1), cls="TEST", area="tools/lab/esp (patrol package)")
    # What the actor module did during the room (lab run 20261028-032920: ownership reached the client 1.1 s after the host left on mobile_tether and transatlantic = the host's
    # handoff timeout, because the new owner never acknowledged): forced commits (host), finals that repeated what the copy held, snapshots dropped as stale, drift written back.
    delta: Dict[str, Any] = {}
    for m in ctx.ms:
        st1 = m.actor_stats()
        for k in ("forcedCommits", "immediateCommits", "finalsRepeated", "staleSeqDropped", "avDriftFixed"):
            if k in st1:
                d = int(st1.get(k) or 0) - int(stats0[m.name].get(k) or 0)
                if d:
                    delta[f"npc_{m.name}_{k}"] = d
    return {"npc_err_max": round(err, 1), **delta}


# ------------------------------------------------------------------ vats
def room_vats(ctx: Ctx) -> Dict[str, Any]:
    """Real-time VATS on both sides, each player on its own target: the hits and the deaths agree on both copies."""
    tg = ctx.ref("target")[:2]
    stats0 = {m.name: m.actor_stats() for m in ctx.ms}
    ctx.stand_all()
    ctx.refs_exist(tg + [ctx.layout.stand("vats")], "the VATS targets")
    for m in ctx.ms:
        ok, why = m.arm()
        ctx.check(f"{m.name} is armed (weapon, ammo, health)", ok, "armed", why or "armed", cls="TEST", area="tools/lab (arming)")
    settle = ctx.p("settle_s")
    stand = ctx.layout.stand("vats")
    hits: Dict[str, int] = {}
    for m, t in ((ctx.host, tg[0]), (ctx.client, tg[1])):
        hits[m.name] = 0
        # One hostile at a time, as in the fight room: both enabled together charge the players and are on top of the shooter (not in view, aim_at refuses
        # under 100 units) when the VATS shot starts (lab run 2026-10-27: vats_not_entered). The host, owner of the actor, makes it a little weaker so a few
        # VATS hits kill it.
        ctx.enable_hostiles([t])
        if K.MUTANT != "vats_target_free":      # twin: the target is left free and walks up to the shooter
            # An enabled gecko ran 368 units in about 4 s and stood on top of the shooter (measured 2026-10-27): no hit chance, no view, VATS stays shut.
            # The host (owner) holds it where it stands, at the standoff.
            ok, why = ctx.host.hold(t)
            ctx.check(f"the host holds the {m.name}'s target in place", ok, "held", why or "held", cls="TEST", area="tools/lab (SetRestrained)")
        ctx.agree(f"{m.name}'s target is alive on both copies", lambda x, t=t: x.alive(t), 15.0, True, area="plugin/actors, plugin/spawn")
        ok, why = ctx.host.set_health(t, ctx.p("weak_hp"))
        ctx.check(f"the host weakens the {m.name}'s target", ok, "health set", why or "health set", cls="TEST", area="tools/lab (SetAV Health)")
        ctx.agree(f"the {m.name}'s target shows the weakened health on both copies", lambda x, t=t: x.health(t) <= ctx.p("weak_hp") + 1.0, ctx.p("weak_s"), True,
                  area="plugin/actors (owner's health to the other copy)")
        for attempt in range(1, int(ctx.p("tries")) + 1):
            h0 = m.health(t)
            ok, why = m.vats(t, None, stand)
            ctx.check(f"{m.name} VATS shot {attempt}", ok, "played out", why or "played out", cls="TEST", area="plugin/vats, plugin/bots")
            # The shot that kills: the owner (the host) publishes health below 0 (host -14) while the actor port never writes a health under 1 to a copy (client 1), so a
            # correctly delivered kill reads different numbers. Once the target is dead on the owner there is no health to compare: the "dead on both copies" check below judges it.
            killed = (not ctx.host.alive(t)) or ctx.host.health(t) <= 0 if K.MUTANT != "vats_compares_kill_health" else False      # twin: the killing shot's health is compared too
            if not killed:
                ctx.agree(f"{m.name} VATS shot {attempt}: the target's health agrees on both copies",
                          lambda x: "dead" if (not x.alive(t) or x.health(t) <= 0) and K.MUTANT != "vats_compares_kill_health" else round(x.health(t)), settle, area="plugin/vats, plugin/combat")     # a target that died meanwhile on the other copy: no number to compare
                ctx.metric("vats_ms", max(int(ctx.metrics.get("vats_ms", 0)), int(round(ctx.last_wait_s * 1000))))     # the slowest shot-to-agreement
            if m.health(t) < h0 or not m.alive(t) or killed:
                hits[m.name] += 1
            if killed or not m.alive(t):
                break
        ctx.agree(f"{m.name}'s target is dead on both copies", lambda x: x.alive(t), settle, False, area="plugin/vats, plugin/combat")
        ctx.check(f"{m.name} hit with VATS at least once", hits[m.name] >= 1, ">= 1", hits[m.name], cls="TEST", area="plugin/vats")
    ctx.agree("deaths agree: both targets dead on both copies", lambda m: [m.alive(t) for t in tg], settle, [False, False], area="plugin/vats, plugin/combat")
    # how often the actor port had to write a copy's drifted health back to the owner's during the room (lab run 20261028-032920: the client's copy read -55 for the owner's 5)
    drift = {}
    for m in ctx.ms:
        d = int(m.actor_stats().get("avDrifts") or 0) - int(stats0[m.name].get("avDrifts") or 0)
        if d:
            drift[f"{m.name}_health_drifts_fixed"] = d
    return {**{f"{k}_vats_hits": v for k, v in hits.items()}, **drift}


# ------------------------------------------------------------------ quest
def room_quest(ctx: Ctx) -> Dict[str, Any]:
    """The lab quest: the host sets a stage and a variable and the client sees them; the client triggers a stage and the host agrees."""
    q = ctx.ref("quest", n=0)
    var = str(ctx.layout.extra("quest", "var", "labvar"))
    settle = ctx.p("settle_s")
    ctx.refs_exist([q], "the lab quest")
    ctx.agree("the quest is at the same stage on both copies at the start", lambda m: m.stage(q), settle, area="plugin/hooks (quest stage), plugin/snapshot")
    (ok, why), tm = ctx.timed(lambda: ctx.host.set_stage(q, 10), lambda: ctx.host.stage(q) == 10, lambda: ctx.client.stage(q) == 10, settle)
    ctx.check("the host sets stage 10", ok, "set", why or "set", cls="TEST", area="tools/lab (setstage)")
    ctx.metric("host_stage_ms", ctx.latency_check("the client sees stage 10", tm, settle, 10, area="plugin/hooks (quest stage), synccore", show=lambda: ctx.both(lambda m: m.stage(q))))
    if ctx.client.var(q, var) is None:
        ctx.skip("the client sees the quest variable", f"this copy cannot read quest variable {var!r}: " + (getattr(ctx.client, "var_why", "") or "no bridge read in this build") + " (see docs/tools/lab.md)")
    else:
        ok, why = ctx.host.set_var(q, var, 7)
        ctx.check("the host sets the quest variable to 7", ok, "set", why or "set", cls="TEST", area="tools/lab (setvar)")
        ctx.agree("the client sees the quest variable", lambda m: m.var(q, var), settle, 7, area="plugin/hooks (quest var), synccore")
    (ok, why), tm = ctx.timed(lambda: ctx.client.set_stage(q, 20), lambda: ctx.client.stage(q) == 20, lambda: ctx.host.stage(q) == 20, settle)
    ctx.check("the client triggers stage 20", ok, "set", why or "set", cls="TEST", area="tools/lab (setstage)")
    ctx.metric("client_stage_ms", ctx.latency_check("the host agrees on stage 20", tm, settle, 20, area="plugin/intents (guest stage), plugin/hooks", show=lambda: ctx.both(lambda m: m.stage(q))))
    (ok, why), tm = ctx.timed(lambda: ctx.host.set_stage(q, 30), lambda: ctx.host.stage(q) == 30, lambda: ctx.client.stage(q) == 30, settle)
    ctx.check("the host sets stage 30", ok, "set", why or "set", cls="TEST", area="tools/lab (setstage)")
    ctx.metric("host_stage30_ms", ctx.latency_check("both copies are at stage 30", tm, settle, 30, area="plugin/hooks (quest stage), synccore", show=lambda: ctx.both(lambda m: m.stage(q))))
    ctx.sleep(ctx.p("hold_s"))
    st = ctx.both(lambda m: m.stage(q))
    ctx.check("the stage does not roll back", st[HOST] == 30 and st[CLIENT] == 30, 30, st, area="plugin/hooks (guest rollback), synccore")
    return {}


# ------------------------------------------------------------------ parity
def _owner_copy(ctx: Ctx, npc: str):
    """The copy whose console lines stick on the NPC: the one that says it owns it (the host when both or neither do)."""
    mine = {m.name: bool(m.owner(npc).get("mine")) for m in ctx.ms}
    return ctx.client if mine[CLIENT] and not mine[HOST] and K.MUTANT != "parity_owner_ignored" else ctx.host      # twin: the lines always go to the host


def _parity_settle(ctx: Ctx, log: lab_parity.ParityLog, step: str) -> Dict[str, Any]:
    """Compare what both copies show (lab_parity.compare_snapshots) round after round until they agree, or parity.settle_s is used up: a change takes a moment to cross
    the link, and animation categories are compared as sets over the rounds. What still differs at the end is the step's finding. -> the last snapshots of both copies."""
    seen: Dict[str, Any] = {}
    end = ctx.now() + ctx.p("settle_s")
    need = max(1, int(ctx.p("min_rounds")))
    rounds = 0
    while True:
        snaps = ctx.par(ctx.host.parity_view, ctx.client.parity_view)
        found, stats = lab_parity.compare_snapshots(snaps[HOST], snaps[CLIENT], seen)
        rounds += 1
        if rounds < need:
            found = [m for m in found if m["field"] != "anim"]       # a set of categories needs a few samples before it says anything
        if (rounds >= need and not found) or ctx.now() >= end or K.MUTANT == "parity_no_settle":      # twin: one look, no time for the change to cross the link
            break
        ctx.sleep(0.4)
    log.add(step, found, rounds, stats)
    for k in ("only_host", "only_client"):
        if stats[k]:
            ctx.metrics[f"parity_npcs_{k}"] = sorted(set(ctx.metrics.get(f"parity_npcs_{k}", [])) | set(stats[k]))
    return snaps


def _owner_shows(ctx: Ctx, copy, pred: Callable[[Dict[str, Any]], bool], timeout_s: float, who: str) -> bool:
    """Poll one copy's own view of an actor (`who`: its runtime id) until pred(view) holds, i.e. the scripted situation took effect where it happens, or timeout_s."""
    end = ctx.now() + timeout_s
    while True:
        v = copy.parity_view()["actors"].get(who)
        if v is not None and pred(v):
            return True
        if ctx.now() >= end:
            return False
        ctx.sleep(0.4)


def room_parity(ctx: Ctx) -> Dict[str, Any]:
    """What each copy SHOWS (docs/sync-model.md section 4): for every NPC loaded on both copies and for each player against the other copy's puppet of him, the equipped
    items, weapon out, sitting, the animation, the look (race, sex) and, per copy, the global time multiplier (1.0 outside VATS). A baseline, then scripted situations (an NPC
    equips and draws a weapon, sits down and stands up, a player draws a weapon and punches), each compared on both copies. Every mismatch is reported with the entity, the field,
    both values and a likely source module; the room fails at the end if any was found."""
    log = lab_parity.ParityLog()
    npc = ctx.ref("npc", n=0)
    chairs = ctx.layout.refs("parity", "chair", "furniture", "seat")
    stand = ctx.layout.stand("parity")
    ctx.stand_all()
    ctx.refs_exist([npc, stand] + chairs[:1], "the NPC and its stand marker")
    ctx.agree("the NPC is alive on both copies", lambda m: m.alive(npc), 10.0, True, area="plugin/actors")
    nid = lab_parity.norm_hex(ctx.host.ref_id(npc))
    weapon = lab_parity.PARITY_WEAPON
    wlow = int(weapon.split(":")[1], 16)

    snaps = _parity_settle(ctx, log, "baseline")
    missing = sorted({x for m in snaps.values() for x in m.get("missing") or []})
    if missing:
        ctx.skip("fields this build cannot read", "; ".join(missing)[:300])
        ctx.metric("parity_missing", missing)
    listed = {m.name: nid in snaps[m.name]["actors"] for m in ctx.ms}
    ctx.check("the lab NPC is in the parity view of both copies", all(listed.values()), "listed on both", listed,
              cls="TEST", area="plugin/animprobe (anim.probe near), tools/lab (parity view)")
    owner = _owner_copy(ctx, npc)
    ctx.metric("parity_npc_owner", owner.name)
    sit_s = ctx.p("sit_s")

    # 1. the NPC equips a weapon and draws it
    ctx.cleanups.append(lambda: owner.npc_reset(npc, weapon))
    ok, why = owner.npc_equip(npc, weapon)
    if ok:
        ok, why = owner.npc_draw(npc, True)
    if not ok:
        ctx.skip("the NPC equips and draws a weapon", f"the {owner.name} copy could not drive it: {why}")
    elif not _owner_shows(ctx, owner, lambda v: v["weaponOut"] is True and any(int(h, 16) & 0xFFFFFF == wlow for h in v["equipped"] or []), sit_s, nid):
        ctx.skip("the NPC equips and draws a weapon", f"the {owner.name} copy never showed the NPC with the weapon equipped and drawn, so there is nothing to compare")
    else:
        _parity_settle(ctx, log, "the NPC equips and draws a weapon")
        ok, why = owner.npc_draw(npc, False)
        if ok and _owner_shows(ctx, owner, lambda v: v["weaponOut"] is False, sit_s, nid):
            _parity_settle(ctx, log, "the NPC puts the weapon away")
        else:
            ctx.skip("the NPC puts the weapon away", why or f"the {owner.name} copy never showed it holstered")

    # 2. the NPC sits down and stands up (needs a placed furniture in the layout)
    if not chairs:
        ctx.skip("the NPC sits down and stands up", "the layout has no furniture ref (a `chair` ref in the npc or parity room, tools/lab/esp): only NPCs that sit by themselves are compared")
    else:
        ctx.cleanups.append(lambda: owner.npc_stand(npc))
        ok, why = owner.npc_sit(npc, chairs[0])
        if not ok or not _owner_shows(ctx, owner, lambda v: v["pose"] == "sit", sit_s, nid):
            ctx.skip("the NPC sits down and stands up", why or f"the {owner.name} copy never showed the NPC seated")
        else:
            _parity_settle(ctx, log, "the NPC sits down")
            ok, why = owner.npc_stand(npc)
            if ok and _owner_shows(ctx, owner, lambda v: v["pose"] == "stand", sit_s, nid):
                _parity_settle(ctx, log, "the NPC stands up")
            else:
                ctx.skip("the NPC stands up", why or f"the {owner.name} copy never showed the NPC standing")

    # 3. each player draws a weapon and punches; the other copy's puppet must follow
    window = ctx.p("punch_window_s")
    for me, other in ((ctx.host, ctx.client), (ctx.client, ctx.host)):
        who = me.name
        ok, why = me.arm()
        if not ok or not me.draw_weapon():
            ctx.skip(f"the {who} player equips and draws a weapon", why or "the weapon would not come out")
        else:
            _parity_settle(ctx, log, f"the {who} player equips and draws a weapon")
        ok, why = me.unarm()
        if not ok:
            ctx.skip(f"the {who} player punches", f"could not put the weapon away: {why}")
            continue
        (hit, hwhy), tm = ctx.timed(lambda me=me: me.punch(int(ctx.p("punches"))), lambda me=me: me.anim_category(None) == "attack",
                                    lambda other=other: other.anim_category("puppet") == "attack", window)
        if not hit or tm["src"] is None:
            ctx.skip(f"the {who} player punches", hwhy or f"the {who} copy never showed an attack animation on its own player (the input driver was made for guns)")
        else:
            log.add(f"the {who} player punches", lab_parity.attack_mismatch(lab_parity.puppet_entity(who), who, True, tm["dst"] is not None))
        _parity_settle(ctx, log, f"the {who} player is unarmed again")

    ctx.metrics.update(parity_steps=log.steps, parity_mismatch_count=len(log.items), parity_fields=log.fields(), parity_mismatches=log.items)
    ctx.check("what the two copies show agrees: NPCs, puppets, time scale", not log.items, "no mismatch", log.headline(),
              note=f"{len(log.items)} mismatch(es) in {len(log.steps)} steps, by field {log.fields()}; every one with both values is in the room's metrics (parity_mismatches)",
              area=log.sources())
    return {}


# spawn also loads the save and enters the lab cell (minutes on a loaded PC: the first in-game run used 70+ s of its 120 s before the first check);
# every other room starts inside the lab
SPAWN_BOX_S = 240.0


class RoomSpec:
    def __init__(self, name: str, fn: Callable[[Ctx], Any], area: str, cost_s: float, budget_s: float = 0.0):
        self.name, self.fn, self.area, self.cost_s, self.budget_s = name, fn, area, cost_s, budget_s


# cost_s: a guess of the run time in a game, only used to split rooms over pairs
ROOMS: Dict[str, RoomSpec] = {s.name: s for s in (
    RoomSpec("spawn", room_spawn, "plugin/coop, plugin/avatar, plugin/areadigest", 60, budget_s=SPAWN_BOX_S),
    RoomSpec("walk", room_walk, "plugin/avatar, plugin/avatar/engine", 90),
    RoomSpec("fight", room_fight, "plugin/combat, plugin/actors", 120),
    RoomSpec("loot", room_loot, "plugin/loot, plugin/intents", 70),
    RoomSpec("door", room_door, "plugin/hooks_world, plugin/intents", 50),
    RoomSpec("npc", room_npc, "plugin/actors, synccore/include/fnvcoop/ownership.h", 100),
    RoomSpec("vats", room_vats, "plugin/vats, plugin/combat", 120),
    RoomSpec("quest", room_quest, "plugin/hooks, plugin/snapshot, synccore", 50),
    RoomSpec("parity", room_parity, "plugin/actors, plugin/avatar, plugin/world_time", 110, budget_s=180.0),
)}
ROOM_ORDER = tuple(ROOMS)


def split_rooms(rooms: List[str], pairs: int) -> List[List[str]]:
    """Spawn first on every pair; the other rooms go, most expensive first, onto the pair with the least work so far (stable)."""
    pairs = max(1, pairs)
    plan: List[List[str]] = [["spawn"] for _ in range(pairs)]
    load = [ROOMS["spawn"].cost_s] * pairs
    rest = sorted((r for r in rooms if r != "spawn"), key=lambda r: (-ROOMS[r].cost_s, ROOM_ORDER.index(r)))
    for r in rest:
        k = min(range(pairs), key=lambda i: (load[i], i))
        plan[k].append(r)
        load[k] += ROOMS[r].cost_s
    for p in plan:
        p[1:] = sorted(p[1:], key=ROOM_ORDER.index)
    return plan
