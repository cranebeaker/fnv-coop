#!/usr/bin/env python
"""One-run co-op benchmark: boot two copies once, run a scripted scenario for a few minutes, get a scorecard back.

    python -I tools/bench/coop_bench.py                       # boot 2 copies, load the save, run all phases (about 4 minutes)
    python -I tools/bench/coop_bench.py --no-boot             # use the two copies that are already up and in the world
    python -I tools/bench/coop_bench.py --label after-fix --baseline out/bench/baseline.json
    python -I tools/bench/coop_bench.py --quick               # shorter phases (about 2 minutes), for a smoke run

What it measures (sampled about 4 times a second per copy, read-only bridge calls; the bots only walk):
  performance  fps from the plugin frame counter (mean, p5 / p1 low, frame time p50 / p95 / p99, hitch share), game CPU %, RAM
  sync         message lag between host and client (lastSeq), digest mismatches / repairs / effects repaired per minute
  network      host bytes in / out per second
  avatars      distance between a player and the other copy's puppet of that player (p50 / p95 / max), puppet teleports, freezes
Every metric is also reported per phase. Scores are 0..100 between a GOOD and a BAD threshold (TARGETS below); the composite is the mean of
the category scores. Results: out/bench/<time>/{results.json, samples.jsonl, report.md}; one summary line is appended to out/bench/history.jsonl.
Honest limits: fps and frame time come from a 4 Hz sample of a frame counter (not per-frame times), and puppet error includes the sampling skew
between the two reads (about the distance a player walks in 0.25 s).
"""
import argparse, json, math, os, subprocess, sys, threading, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path[:0] = [str(HERE), str(ROOT / "tools"), str(ROOT / "tools" / "runner")]
import phases  # noqa: E402

# metric -> (GOOD, BAD): 100 points at GOOD, 0 at BAD, linear between; works for both directions
TARGETS = {
    "perf.fps_p5":              (55.0, 15.0),
    "perf.hitch_pct":           (0.0, 30.0),      # share of samples under 30 fps
    "perf.bridge_rtt_p95":      (40.0, 400.0),    # ms: how long the game's main thread takes to answer a bridge call (stall proxy)
    "sync.seq_lag_p95":         (3.0, 30.0),      # messages behind
    "sync.digest_mismatch_pm":  (0.5, 6.0),
    "sync.repair_effects_pm":   (5.0, 300.0),
    "avatar.err_p95":           (200.0, 1500.0),  # world units
    "avatar.teleports_pm":      (0.0, 10.0),
    "play.fight_pass_pct":      (100.0, 0.0),     # share of the fight playbook's steps that passed (both screens agree on health, deaths, loot)
    "play.loot_pass_pct":       (100.0, 0.0),     # loot race playbook: exactly one winner, nothing duplicated
    "play.kill_latency_s":      (1.0, 8.0),       # seconds from a kill on one copy until the other copy shows the actor dead (worst of the kills)
    "play.npc_pos_err":         (50.0, 500.0),    # world units: worst distance between the two copies' view of a living NPC
    "area.recovery_s":          (3.0, 20.0),      # seconds until the client's puppet of the host is near the host again after the host leaves a building
    "net.bad_wifi_seq_lag_p95": (10.0, 100.0),    # messages behind on the bad_wifi profile (35 ms +- 25, 2 % loss with bursts)
    "net.bad_wifi_avatar_err_p95": (300.0, 2500.0),
    "net.recover_seq_lag_p95":  (5.0, 50.0),      # after the worst profile ("hell") is switched back to lan: how fast the session recovers
    # parity: what the two screens disagree about during the run, from the plugin's own samples (docs/tools/parity.md); share of compared references
    # (actors / player puppets) with a confirmed difference
    "parity.npc_mismatch_pct":    (2.0, 25.0),
    "parity.puppet_mismatch_pct": (2.0, 25.0),
}
CATEGORIES = {"performance": ["perf.fps_p5", "perf.hitch_pct", "perf.bridge_rtt_p95"],
              "sync": ["sync.seq_lag_p95", "sync.digest_mismatch_pm", "sync.repair_effects_pm"],
              "avatars": ["avatar.err_p95", "avatar.teleports_pm"],
              "gameplay": ["play.fight_pass_pct", "play.loot_pass_pct", "play.kill_latency_s", "play.npc_pos_err", "area.recovery_s"],
              "network": ["net.bad_wifi_seq_lag_p95", "net.bad_wifi_avatar_err_p95", "net.recover_seq_lag_p95"],
              "parity": ["parity.npc_mismatch_pct", "parity.puppet_mismatch_pct"]}
# Categories shown next to the others but not part of the composite (so the composite stays comparable with every earlier run). Take "parity" out of this
# tuple once a few runs have set its baseline.
NOT_IN_COMPOSITE = ("parity",)


def score(metric, value):
    if value is None or isinstance(value, float) and math.isnan(value):
        return None
    good, bad = TARGETS[metric]
    f = (value - bad) / (good - bad)
    return round(100 * max(0.0, min(1.0, f)), 1)


def pct(vals, q):
    if not vals:
        return None
    s = sorted(vals)
    return s[min(len(s) - 1, max(0, int(round(q * (len(s) - 1)))))]


def dist(a, b):
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))


class Sampler(threading.Thread):
    """Reads both copies about 4 times a second; never writes game state."""
    def __init__(self, conns, pids, t0, out_path):
        super().__init__(daemon=True)
        self.conns, self.pids, self.t0 = conns, pids, t0
        self.phase = ["warmup"]
        self.stop_ev = threading.Event()
        self.rows = []
        self.out = open(out_path, "w", encoding="utf-8")
        self.puppet = {0: None, 1: None}   # copy -> ref of the puppet that copy has for the OTHER player
        self.errors = 0
        import winproc
        self.winproc = winproc
        self.prev_cpu = {}

    def stat_proc(self, i):
        try:
            st = self.winproc.proc_stats(self.pids[i])
            return st
        except Exception:
            return {}

    def run(self):
        n = 0
        while not self.stop_ev.is_set():
            t = time.time()
            row = {"t": round(t - self.t0, 2), "phase": self.phase[0]}
            for i, nv in enumerate(self.conns):
                c = {}
                try:
                    p0 = time.perf_counter()
                    cs = nv.call("coop.status", {}, timeout_s=5)
                    c["rtt_ms"] = round((time.perf_counter() - p0) * 1000, 1)   # bridge round trip: the main thread's service delay
                    ctr = cs.get("counters", {})
                    dg = cs.get("desyncs", {}).get("digests", {})
                    tr = cs.get("transport", {})
                    c.update(frames=ctr.get("frames"), seq=cs.get("lastSeq"), dok=dg.get("ok"), dmis=dg.get("mismatch"), drep=dg.get("repairs"),
                             deff=dg.get("effectsRepaired"), dset=dg.get("effectsSettled"), dack=dg.get("divergenceAcked"), bin=tr.get("bytesIn"), bout=tr.get("bytesOut"))
                    c["pos"] = [nv.eval("Player.GetPos X"), nv.eval("Player.GetPos Y"), nv.eval("Player.GetPos Z")]
                    if n % 8 == 0:   # every ~2 s: which puppet does this copy have for the other player
                        av = nv.call("avatar.status", {}, timeout_s=5)
                        pu = av.get("puppets", [])
                        self.puppet[i] = pu[0].get("ref") if pu else None
                        st = self.stat_proc(i)
                        c["proc"] = {k: st.get(k) for k in st}
                    if self.puppet[i]:
                        try:
                            rid = self.puppet[i].replace("0x", "")
                            c["pup"] = [nv.eval("GetPos X", ref=rid), nv.eval("GetPos Y", ref=rid), nv.eval("GetPos Z", ref=rid)]
                        except Exception as e:
                            c["pup_err"] = str(e)[:60]
                except Exception as e:
                    c["err"] = str(e)[:80]
                    self.errors += 1
                row[str(i)] = c
            self.rows.append(row)
            self.out.write(json.dumps(row) + "\n")
            n += 1
            time.sleep(max(0.0, 0.25 - (time.time() - t)))
        self.out.close()


def lean_inis(log):
    """A machine short of RAM: apply the pool's low-cost profile (shadows, grass, water, effects, sound off) to both instances' own INIs.
    The window size and the frame cap stay as they are (see instances.LEAN_SKIP). Set NVCOOP_LEAN=0 to skip."""
    if os.environ.get("NVCOOP_LEAN", "1") == "0":
        return
    try:
        sys.path.insert(0, str(ROOT / "tools" / "pool"))
        sys.path.insert(0, str(ROOT / "tools" / "runner"))
        import instances
        import lowcost
        for i in (0, 1):
            d = instances.instance_dir(i)
            texts = {}
            for name in (lowcost.FO, lowcost.PR):
                lowcost.guard(d / name)
                texts[name] = (d / name).read_text(encoding="latin-1")
            texts[lowcost.FO], texts[lowcost.PR] = instances._apply_lean(texts[lowcost.FO], texts[lowcost.PR])
            for name, t in texts.items():
                (d / name).write_text(t, encoding="latin-1", newline="")
        log("lean game settings applied to both copies (no sound, no shadows, no grass, no water effects)")
    except Exception as e:   # noqa: BLE001
        log(f"lean settings not applied: {str(e)[:120]}")


def boot(save, attempts=3):
    """Bring up two copies and load the save. A copy can hang while loading (its main thread stops answering the bridge): a hung boot is stopped
    and tried again, up to `attempts` times, so a run never goes on against a dead game."""
    last = None
    for k in range(attempts):
        try:
            _boot_once(save)
            return
        except (Exception, SystemExit) as e:   # noqa: BLE001
            last = e
            print(f"   boot attempt {k + 1}/{attempts} failed: {str(e)[:140]}")
    raise SystemExit(f"could not boot two responsive copies in {attempts} attempts: {str(last)[:200]}")


REMOTE = [None]   # a remote.RemoteHost when the host copy runs on another machine (--remote)


def _boot_once(save):
    env = dict(os.environ, NVCOOP_VISIBLE="1", NVCOOP_TEST_HELPERS="1", NVCOOP_LEAN="1", PYTHONIOENCODING="utf-8")
    run_py = str(ROOT / "tools" / "runner" / "run.py")
    subprocess.run([sys.executable, "-I", run_py, "down"], cwd=str(ROOT), env=env, capture_output=True)
    time.sleep(8)
    lean_inis(print)
    remote = REMOTE[0]
    if remote is not None:   # the host copy (instance 0) runs on the other machine; only the client (instance 1) starts here
        remote.up()
    up = [sys.executable, "-I", run_py, "up", "--n", "2", "--wait-pipe", "180"] + (["--only", "1"] if remote is not None else [])
    r = subprocess.run(up, cwd=str(ROOT), env=env, capture_output=True, text=True)
    if ("pipes: {1: True}" if remote is not None else "pipes: {0: True, 1: True}") not in r.stdout:
        raise SystemExit("could not bring up two copies:\n" + r.stdout[-400:] + r.stderr[-400:])
    from nvbridge import NvBridge
    from bots.bot import Bot
    conns = [NvBridge.connect(instance=i, wait_s=20) for i in (0, 1)]
    for nv in conns:
        nv.call("loadSave", {"name": save, "timeoutMs": 120000}, timeout_s=130)
    for i, nv in enumerate(conns):
        Bot(nv, name=f"b{i}", slot=i).dismiss_messages(quiet_s=25, max_s=150, max_n=20)
    for i, nv in enumerate(conns):
        if not phases.wait_serviced(nv, print, timeout_s=60):
            raise RuntimeError(f"copy {i} is hung after loading the save (its main thread does not answer)")


def pids_of():
    import run
    st = run.load_state()["pids"]
    return {int(k): v for k, v in st.items()}


WALK_MODE = ["step"]   # "step": the bot sets positions in small increments (repeatable, no collisions); "input": real movement input (needs open ground)


def walk_square(bot, leg, deadline, stats):
    """Walk a square of side `leg` units from the current position until `deadline`; counts legs that arrived."""
    x, y, z = [float(v) for v in bot.pos()]
    pts = [(x + leg, y, z), (x + leg, y + leg, z), (x, y + leg, z), (x, y, z)]
    k = 0
    while time.time() < deadline:
        px, py, pz = pts[k % 4]
        r = bot.goto_pos(px, py, pz, timeout_s=max(5.0, min(25.0, deadline - time.time())), mode=WALK_MODE[0])
        stats["legs"] += 1
        stats["legs_ok"] += 1 if r.ok else 0
        if not r.ok:
            why = str(getattr(r, "reason", "") or "")[:70]
            stats.setdefault("why", {})
            stats["why"][why] = stats["why"].get(why, 0) + 1
        k += 1


def run_phases(samp, bots, quick, opts=None, relay=None, outdir=None, log=print):
    """The scripted scenario. Returns (walk stats, extras): extras holds the area-change, input-probe and playbook results."""
    from bots.bot import Bot  # noqa: F401
    opts = opts or {}
    k = 0.5 if quick else 1.0
    stats = {"legs": 0, "legs_ok": 0}
    extras = {}

    def phase(name, secs, fn=None):
        samp.phase[0] = name
        end = time.time() + secs * k
        if fn:
            fn(end)
        rest = end - time.time()
        if rest > 0:
            time.sleep(rest)

    phase("warmup", 15)
    phase("idle", 30)
    phase("host_walks", 60, lambda end: walk_square(bots[0], 450, end, stats))
    phase("client_walks", 60, lambda end: walk_square(bots[1], 450, end, stats))

    def both(end):
        ts = [threading.Thread(target=walk_square, args=(b, 400, end, stats), daemon=True) for b in bots]
        [t.start() for t in ts]
        [t.join(timeout=max(1, end - time.time() + 30)) for t in ts]
    phase("both_walk", 40, both)
    phase("settle", 10)
    if opts.get("probe", True):
        samp.phase[0] = "input_probe"
        extras["input_probe"] = phases.input_probe(bots, log)
    if opts.get("area", True):
        extras["area"] = phases.area_change(samp, bots, k, log)
        phase("settle_area", 8)
    if opts.get("playbooks", True):
        for pb, label in (("fight", "fight"), ("loot_race", "loot")):
            samp.phase[0] = f"playbook_{label}"
            extras[f"playbook_{label}"] = phases.run_playbook(pb, Path(outdir) / f"pb_{label}", log)
    if relay is not None:
        for prof in phases.NET_PROFILES:
            try:
                relay.profile(prof)
            except Exception as e:   # noqa: BLE001
                log(f"relay profile {prof}: {e}")
            phase(f"net_{prof}", 35, both)
        relay.profile("lan")
        phase("net_recover", 25)
    return stats, extras


def analyse(rows):
    """rows -> per-phase and overall metrics."""
    def series(i, key, ph=None):
        return [(r["t"], r[str(i)].get(key)) for r in rows if (ph is None or r["phase"] == ph) and r[str(i)].get(key) is not None]

    def rate(vals):   # counter -> per-second between successive samples
        out = []
        for (t0, a), (t1, b) in zip(vals, vals[1:]):
            if t1 > t0 and b >= a:
                out.append((t1, (b - a) / (t1 - t0)))
        return out

    def metrics(ph):
        m = {}
        sel = [r for r in rows if (ph is None or r["phase"] == ph) and r["phase"] != "warmup"]
        if len(sel) < 4:
            return m
        dur = sel[-1]["t"] - sel[0]["t"]
        m["seconds"] = round(dur, 1)
        for i in (0, 1):
            fr = rate([(r["t"], r[str(i)].get("frames")) for r in sel if r[str(i)].get("frames") is not None])
            fps = [v for _, v in fr]
            if fps:
                m[f"copy{i}.fps_mean"] = round(sum(fps) / len(fps), 1)
                m[f"copy{i}.fps_p5"] = round(pct(fps, 0.05), 1)
                m[f"copy{i}.fps_p1"] = round(pct(fps, 0.01), 1)
                ft = [1000.0 / v for v in fps if v > 0.5]
                m[f"copy{i}.frame_ms_p50"] = round(pct(ft, 0.50), 1)
                m[f"copy{i}.frame_ms_p95"] = round(pct(ft, 0.95), 1)
                m[f"copy{i}.frame_ms_p99"] = round(pct(ft, 0.99), 1)
                m[f"copy{i}.hitch_pct"] = round(100.0 * sum(1 for v in fps if v < 30) / len(fps), 1)
            cs = [(r["t"], r[str(i)]["proc"]["cpu_s"]) for r in sel if r[str(i)].get("proc", {}).get("cpu_s") is not None]
            cpu = [100.0 * (b - a) / (t1 - t0) for (t0, a), (t1, b) in zip(cs, cs[1:]) if t1 > t0]
            ws = [r[str(i)].get("proc", {}).get("ws_mb") for r in sel if r[str(i)].get("proc", {}).get("ws_mb") is not None]
            rtt = [r[str(i)]["rtt_ms"] for r in sel if r[str(i)].get("rtt_ms") is not None]
            if rtt:
                m[f"copy{i}.bridge_rtt_ms_p50"] = round(pct(rtt, 0.5), 1)
                m[f"copy{i}.bridge_rtt_ms_p95"] = round(pct(rtt, 0.95), 1)
                m[f"copy{i}.bridge_rtt_ms_max"] = round(max(rtt), 1)
            wst = [(r["t"], r[str(i)]["proc"]["ws_mb"]) for r in sel if r[str(i)].get("proc", {}).get("ws_mb") is not None]
            if len(wst) >= 3 and wst[-1][0] > wst[0][0]:
                m[f"copy{i}.ram_growth_mb_per_min"] = round((wst[-1][1] - wst[0][1]) * 60.0 / (wst[-1][0] - wst[0][0]), 1)
            if ws:
                m[f"copy{i}.ram_mb_max"] = round(max(ws))
            if cpu:
                m[f"copy{i}.cpu_pct_mean"] = round(sum(cpu) / len(cpu))
        lag = [abs(r["0"]["seq"] - r["1"]["seq"]) for r in sel if r["0"].get("seq") is not None and r["1"].get("seq") is not None]
        if lag:
            m["sync.seq_lag_p50"] = pct(lag, 0.5)
            m["sync.seq_lag_p95"] = pct(lag, 0.95)
            m["sync.seq_lag_max"] = max(lag)
        # sync latency, INFORMATION ONLY (not scored): how long after the host's message N first shows up does the client reach N. The two reads are
        # milliseconds apart and the sample period is 0.25 s, so 0 means "under about a quarter second". A real latency probe needs a marker effect.
        hs = [(r["t"], r["0"]["seq"]) for r in sel if r["0"].get("seq") is not None]
        cs_ = [(r["t"], r["1"]["seq"]) for r in sel if r["1"].get("seq") is not None]
        lat, j, last = [], 0, None
        for t, v in hs:
            if last is not None and v <= last:
                continue
            last = v
            while j < len(cs_) and cs_[j][1] < v:
                j += 1
            if j >= len(cs_):
                break
            lat.append(max(0.0, cs_[j][0] - t) * 1000.0)
        if lat:
            m["sync.latency_ms_p50"] = round(pct(lat, 0.5))
            m["sync.latency_ms_p95"] = round(pct(lat, 0.95))
            m["sync.latency_ms_max"] = round(max(lat))
        h = [(r["t"], r["0"]) for r in sel if r["0"].get("dmis") is not None]
        if len(h) >= 2 and dur > 0:
            m["sync.digest_mismatch_pm"] = round((h[-1][1]["dmis"] - h[0][1]["dmis"]) * 60.0 / (h[-1][0] - h[0][0]), 2)
            m["sync.digest_ok_total"] = h[-1][1].get("dok")
            m["sync.ever_converged"] = bool((h[-1][1].get("dok") or 0) > 0)
            m["sync.repairs_pm"] = round((h[-1][1]["drep"] - h[0][1]["drep"]) * 60.0 / (h[-1][0] - h[0][0]), 2)
            m["sync.repair_effects_pm"] = round((h[-1][1]["deff"] - h[0][1]["deff"]) * 60.0 / (h[-1][0] - h[0][0]), 1)
            if h[-1][1].get("dset") is not None and h[0][1].get("dset") is not None:   # keys the host recorded once (bookkeeping, not repair): informational
                m["sync.settled_effects_total"] = h[-1][1]["dset"] - h[0][1]["dset"]
                m["sync.divergences_acked"] = h[-1][1].get("dack")
        bo = rate([(r["t"], r["0"].get("bout")) for r in sel if r["0"].get("bout") is not None])
        bi = rate([(r["t"], r["0"].get("bin")) for r in sel if r["0"].get("bin") is not None])
        if bo:
            m["net.host_out_kb_s_mean"] = round(sum(v for _, v in bo) / len(bo) / 1024, 2)
            m["net.host_out_kb_s_p95"] = round(pct([v for _, v in bo], 0.95) / 1024, 2)
        if bi:
            m["net.host_in_kb_s_mean"] = round(sum(v for _, v in bi) / len(bi) / 1024, 2)
        # avatars: copy 1's puppet follows copy 0's player, copy 0's puppet follows copy 1's player
        errs, tele, frozen, moved_s, n = [], 0, 0, 0.0, 0
        for src, obs in ((0, 1), (1, 0)):
            prev = None
            for r in sel:
                p, q = r[str(src)].get("pos"), r[str(obs)].get("pup")
                if not p or not q:
                    prev = None
                    continue
                errs.append(dist(p, q))
                if prev:
                    dt = r["t"] - prev[0]
                    src_move, pup_move = dist(p, prev[1]), dist(q, prev[2])
                    if pup_move > 300 and pup_move > src_move + 250:
                        tele += 1
                    if src_move > 80 and pup_move < 5:
                        frozen += 1
                    moved_s += dt
                prev = (r["t"], p, q)
        if errs:
            m["avatar.err_p50"] = round(pct(errs, 0.5))
            m["avatar.err_p95"] = round(pct(errs, 0.95))
            m["avatar.err_max"] = round(max(errs))
            minutes = max(moved_s / 60.0, 1e-6)
            m["avatar.teleports_pm"] = round(tele / minutes, 2)
            m["avatar.frozen_samples"] = frozen
        else:
            m["avatar.err_unavailable"] = True
        return m

    phases = []
    for r in rows:
        if r["phase"] not in phases:
            phases.append(r["phase"])
    return {"overall": metrics(None), "phases": {p: metrics(p) for p in phases if p != "warmup"}}


def finalize_extras(rows, extras):
    """Derive what needs the samples: seconds until the client's puppet of the host is near the host again after the way out of the building."""
    ar = (extras or {}).get("area") or {}
    tb = ar.get("t_back")
    if tb is not None and "recovery_s" not in ar:
        good, rec = 0, None
        for r in rows:
            if r["t"] < tb:
                continue
            p, q = r["0"].get("pos"), r["1"].get("pup")
            if p and q and dist(p, q) < 300:
                good += 1
                if good >= 2:
                    rec = round(r["t"] - tb, 1)
                    break
            else:
                good = 0
        ar["recovery_s"] = rec if rec is not None else 99.0   # never recovered inside the phase
    return extras


def derive(overall, phase_metrics, extras):
    """Scored metrics that are not plain overall values: gameplay playbooks, the area change, the bad-network profiles."""
    m = dict(overall)
    for lab in ("fight", "loot"):
        pb = (extras or {}).get(f"playbook_{lab}")
        if pb is not None:
            m[f"play.{lab}_pass_pct"] = pb.get("pass_pct", 0.0)
            for k in ("kill_latency_s", "npc_pos_err"):
                if (pb.get("metrics") or {}).get(k) is not None:
                    m[f"play.{k}"] = pb["metrics"][k]
    ar = (extras or {}).get("area") or {}
    if ar.get("recovery_s") is not None:
        m["area.recovery_s"] = ar["recovery_s"]
    par = (extras or {}).get("parity") or {}
    m["parity.npc_mismatch_pct"] = par.get("npc_mismatch_pct")
    m["parity.puppet_mismatch_pct"] = par.get("puppet_mismatch_pct")
    nb = (phase_metrics or {}).get("net_bad_wifi") or {}
    m["net.bad_wifi_seq_lag_p95"] = nb.get("sync.seq_lag_p95")
    m["net.bad_wifi_avatar_err_p95"] = nb.get("avatar.err_p95")
    nr = (phase_metrics or {}).get("net_recover") or {}
    m["net.recover_seq_lag_p95"] = nr.get("sync.seq_lag_p95")
    return m


def scorecard(overall, phase_metrics=None, extras=None):
    m = derive(overall, phase_metrics, extras)
    # worst copy for fps
    for key in ("fps_p5", "hitch_pct"):
        vals = [overall.get(f"copy{i}.{key}") for i in (0, 1) if overall.get(f"copy{i}.{key}") is not None]
        if vals:
            m[f"perf.{key}"] = min(vals) if key == "fps_p5" else max(vals)
    rv = [overall.get(f"copy{i}.bridge_rtt_ms_p95") for i in (0, 1) if overall.get(f"copy{i}.bridge_rtt_ms_p95") is not None]
    if rv:
        m["perf.bridge_rtt_p95"] = max(rv)
    out, cats = {}, {}
    for met in TARGETS:
        out[met] = {"value": m.get(met), "score": score(met, m.get(met)), "good": TARGETS[met][0], "bad": TARGETS[met][1]}
    for c, mets in CATEGORIES.items():
        sc = [out[x]["score"] for x in mets if out[x]["score"] is not None]
        cats[c] = round(sum(sc) / len(sc), 1) if sc else None
    vals = [v for c, v in cats.items() if v is not None and c not in NOT_IN_COMPOSITE]
    return out, cats, round(sum(vals) / len(vals), 1) if vals else None


def report_extras(L, analysis, extras):
    """Playbooks, area change, input probe and the bad-network table."""
    extras = extras or {}
    pbs = [(k, v) for k, v in extras.items() if k.startswith("playbook_")]
    if pbs:
        L += ["", "## Gameplay playbooks (both screens must agree)", ""]
        for k, v in pbs:
            L.append(f"**{v['name']}**: {v.get('steps_ok')}/{v.get('steps_total')} steps passed in {v.get('seconds')} s; "
                     f"snapshots equal {v.get('snapdiff_equal')}/{v.get('steps_total')}. {v.get('reason') or v.get('error') or ''}")
            if v.get("steps"):
                L += ["", "| Step | OK | s | Snapshot compare | Digest mismatches | Why not |", "|---|---|---|---|---|---|"]
                for s in v["steps"]:
                    L.append(f"| {s['name']} | {s['ok']} | {s.get('seconds')} | {s.get('snapdiff')} | {s.get('digest_mismatch')} | {s.get('reason')} |")
            ls = (v.get("metrics") or {}).get("loot_service")
            if ls:
                keys = ["containers", "known", "scans", "seen", "suspects", "arbitrated", "reverted", "revertedItems", "refused", "busy", "setsDeferred", "graceExpired", "countFails"]
                L += ["Loot service counters during the step (coop.loot menuless block; `seen` = looks at a corpse whose engine copy differed, `arbitrated` = transfers sent through the lock, "
                      "`reverted` = items taken back from the player):", "", "| Copy | " + " | ".join(keys) + " |", "|---|" + "---|" * len(keys)]
                for who, d in ls.items():
                    L.append(f"| {who} | " + " | ".join(str((d or {}).get(k)) for k in keys) + " |")
            L.append("")
    ar = extras.get("area")
    if ar:
        L += ["", "## Area change (host into a building and back out)", "", f"coc: `{ar.get('coc')}`, cow: `{ar.get('cow')}`, ok {ar.get('ok')}. "
              f"Seconds until the client's puppet of the host was near the host again: **{ar.get('recovery_s')}** (99 = never).", ""]
    par = extras.get("parity")
    if par:
        L += ["", "## Parity (what the two screens disagree about; plugin/parity samples, tools/parity/compare.py)", "",
              f"Paired by {par.get('paired_by')}: {par.get('pairs')} sample pairs over {par.get('span_s')} s, {par.get('rounds', 0) - par.get('skipped', 0)} compared rounds "
              f"({par.get('skipped')} skipped). References compared {par.get('refs')}, with a difference {par.get('refs_bad')} ({par.get('mismatch_pct')} %): "
              f"actors {par.get('npc_mismatch_pct')} %, player puppets {par.get('puppet_mismatch_pct')} %.", "",
              "| Kind | Checks | Mismatching | Rate % |", "|---|---|---|---|"]
        for k, v in (par.get("kinds") or {}).items():
            L.append(f"| {k} | {v['checks']} | {v['bad']} | {v['rate_pct']} |")
        if par.get("worst"):
            L += ["", "Worst offenders:", ""]
            for w in par["worst"]:
                L.append(f"- `{w['key']}` {w['kind']}: {w['rounds_bad']} of {w['rounds_seen']} rounds, {w['first_s']} s to {w['last_s']} s, host {w['host']}, client {w['client']}")
        L.append("")
    ip = extras.get("input_probe")
    if ip:
        L += ["", "## Input-walk probe (does real movement input move a player?)", "", "```json", json.dumps(ip), "```", ""]
    net = [(p, m) for p, m in analysis["phases"].items() if p.startswith("net_")]
    if net:
        keys = ["copy0.fps_mean", "copy1.fps_mean", "sync.seq_lag_p50", "sync.seq_lag_p95", "sync.seq_lag_max", "sync.digest_mismatch_pm",
                "sync.repair_effects_pm", "net.host_out_kb_s_mean", "avatar.err_p95", "avatar.teleports_pm"]
        L += ["", "## Bad-network matrix (the client joins through the chaos relay; profiles from tools/chaos/profiles.json)", "",
              "| Profile | s | " + " | ".join(keys) + " |", "|---|---|" + "---|" * len(keys)]
        for p, m in net:
            L.append(f"| {p[4:]} | {m.get('seconds')} | " + " | ".join(str(m.get(k)) for k in keys) + " |")
    return L


def write_report(path, label, meta, analysis, sc, cats, comp, base, extras=None):
    L = [f"# Co-op benchmark {meta['stamp']}" + (f" ({label})" if label else ""), ""]
    L.append(f"Composite score: **{comp}** / 100.  " + "  ".join(f"{c} **{v}**" for c, v in cats.items()))
    if base:
        d = None if base.get("composite") is None or comp is None else round(comp - base["composite"], 1)
        L.append(f"Baseline `{base.get('label') or base.get('stamp')}`: composite {base.get('composite')} (change {d:+})" if d is not None else "Baseline has no composite")
    L += ["", "| Metric | Value | Score | Good | Bad |", "|---|---|---|---|---|"]
    for k, v in sc.items():
        L.append(f"| {k} | {v['value']} | {v['score']} | {v['good']} | {v['bad']} |")
    L += ["", "## Per phase", ""]
    keys = ["copy0.fps_mean", "copy1.fps_mean", "copy0.fps_p5", "copy1.fps_p5", "sync.seq_lag_p95", "sync.digest_mismatch_pm", "sync.repair_effects_pm",
            "net.host_out_kb_s_mean", "avatar.err_p50", "avatar.err_p95", "avatar.teleports_pm", "avatar.frozen_samples"]
    L.append("| Phase | s | " + " | ".join(keys) + " |")
    L.append("|---|---|" + "---|" * len(keys))
    for p, m in analysis["phases"].items():
        L.append(f"| {p} | {m.get('seconds')} | " + " | ".join(str(m.get(k)) for k in keys) + " |")
    L = report_extras(L, analysis, extras)
    L += ["", "## Keys the digest still disagrees on (host's last reports, count = reports it appeared in)", ""]
    for k, n in analysis.get("desync_top_keys", []):
        L.append(f"- {n}x {k}")
    L += ["", "## Everything (overall)", "", "```json", json.dumps(analysis["overall"], indent=1), "```", "",
          f"Walk legs: {meta['walk']['legs_ok']} of {meta['walk']['legs']} arrived. Why not: {json.dumps(meta['walk'].get('why', {}))}. "
          f"Sampler read errors: {meta['sampler_errors']}."]
    Path(path).write_text("\n".join(L), encoding="utf-8")


def top_desync_keys(conn):
    """Which keys still differ between host and client: the host's last few desync reports."""
    import collections
    try:
        reps = conn.call("coop.status", {}, timeout_s=10).get("desyncs", {}).get("desyncs", [])
        cnt = collections.Counter()
        for rep in reps[-6:]:
            for e in rep.get("entries", [])[:64]:
                tag = "client-only" if e.get("clientOnly") else ("race" if e.get("race") else "differs")
                cnt[f"{tag}: {(e.get('key') or '')[:100]}  host={str(e.get('host'))[:40]} client={str(e.get('client'))[:40]}"] += 1
        return cnt.most_common(20)
    except Exception as ex:
        return [("unavailable: " + str(ex)[:80], 0)]


def _parity_module():
    import importlib.util
    spec = importlib.util.spec_from_file_location("parity_compare", ROOT / "tools" / "parity" / "compare.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def parity_extras(folder):
    """Score the two sample files saved in a run folder (parity_host.jsonl, parity_client.jsonl); None when there are none or too little to compare."""
    h, c = Path(folder) / "parity_host.jsonl", Path(folder) / "parity_client.jsonl"
    if not (h.is_file() and c.is_file()):
        return None
    try:
        return _parity_module().bench_metrics(h, c)
    except Exception as e:   # a broken file must not sink a benchmark run
        return {"error": str(e)[:120]}


def collect_parity(out, log=print):
    """After the phases: copy each local copy's nvcoop_parity.jsonl into the run folder (copy 0 = host, copy 1 = client) and score them."""
    if REMOTE[0] is not None:
        return None   # the host copy runs on the other PC: its file is not here
    try:
        import instances
        root = instances.instances_root()
        got = 0
        for i, name in ((0, "parity_host.jsonl"), (1, "parity_client.jsonl")):
            src = root / str(i) / "Data" / "NVSE" / "Plugins" / "nvcoop_parity.jsonl"
            if src.is_file() and src.stat().st_size:
                (Path(out) / name).write_bytes(src.read_bytes())
                got += 1
        if got < 2:
            log("parity: no nvcoop_parity.jsonl from both copies (provision with --ini parity.enabled=1); the parity score is skipped")
            return None
        return parity_extras(out)
    except Exception as e:
        log("parity: skipped (%s)" % str(e)[:100])
        return None


def run_once(a, label, stamp):
    """One full benchmark: (re)boot unless --no-boot, run the phases, analyse, write the folder. Returns (result dict, folder)."""
    out = ROOT / "out" / "bench" / stamp
    out.mkdir(parents=True, exist_ok=True)
    print("bench folder:", out, flush=True)
    log = lambda s: print("   " + s, flush=True)   # noqa: E731
    relay, client_ini, ini_backup = None, None, None
    try:
        if REMOTE[0] is not None:
            phases.HOST_IP[0] = REMOTE[0].ip
            if not a.net:   # the client joins the host on the other machine directly
                import instances
                client_ini = phases.client_ini_path(instances.instances_root())
                ini_backup = phases.point_client_at_relay(client_ini, True, host=REMOTE[0].ip, port=phases.HOST_PORT)
        if a.net:   # the client joins through the chaos relay: start it and point the client's ini at it BEFORE the copies boot
            import instances
            client_ini = phases.client_ini_path(instances.instances_root())
            ini_backup = phases.point_client_at_relay(client_ini, True)
            relay = phases.Relay(log)
            relay.start("lan")
        if not a.no_boot:
            print("booting two copies, loading the save ...", flush=True)
            boot(a.save)
        from nvbridge import NvBridge
        from bots.bot import Bot
        sconns = [NvBridge.connect(instance=i, wait_s=20) for i in (0, 1)]       # sampler: its own pipes
        bconns = [NvBridge.connect(instance=i, wait_s=20) for i in (0, 1)]       # bots
        bots = [Bot(nv, name=f"bench{i}", slot=i) for i, nv in enumerate(bconns)]
        for b in bots:
            b.auto_dismiss = True
        pids = pids_of()
        t0 = time.time()
        samp = Sampler(sconns, pids, t0, out / "samples.jsonl")
        samp.start()
        print("running phases ...", flush=True)
        opts = {"probe": not a.no_probe, "area": not a.no_area, "playbooks": not a.no_playbooks}
        try:
            if a.scenario:   # a scenario file: tools/bench/scenarios/<name>.json (see scenario.py)
                import bench_scenario as scenario_mod
                import arena as arena_mod
                ctx = scenario_mod.Ctx(samp, bots, arena_mod.Arena.load(a.arena or None), k=0.5 if a.quick else 1.0, relay=relay, outdir=out,
                                       log=log, walk_mode=a.walk_mode, phases_mod=phases)
                scenario_mod.run(ctx, scenario_mod.load(a.scenario))
                walk, extras = ctx.walk, ctx.extras
            else:
                walk, extras = run_phases(samp, bots, a.quick, opts, relay, out, log)
        finally:
            samp.stop_ev.set()
            samp.join(timeout=10)
        analysis = analyse(samp.rows)
        analysis["desync_top_keys"] = top_desync_keys(sconns[0])
        extras = finalize_extras(samp.rows, extras)
        par = collect_parity(out, log)
        if par is not None:
            extras = dict(extras or {}, parity=par)
        sc, cats, comp = scorecard(analysis["overall"], analysis["phases"], extras)
        base = None
        if a.baseline and Path(a.baseline).is_file():
            base = json.loads(Path(a.baseline).read_text())
        meta = {"stamp": stamp, "label": label, "seconds": round(time.time() - t0), "walk": walk, "sampler_errors": samp.errors, "quick": a.quick,
                "walk_mode": a.walk_mode, "net": a.net}
        res = {"meta": meta, "composite": comp, "categories": cats, "scores": sc, "extras": extras, **analysis}
        (out / "results.json").write_text(json.dumps(res, indent=1, default=str), encoding="utf-8")
        write_report(out / "report.md", label, meta, analysis, sc, cats, comp, base, extras)
        return res, out
    finally:
        if relay is not None:
            relay.stop()
        if client_ini is not None and ini_backup is not None:
            client_ini.write_text(ini_backup, encoding="latin-1", newline="")   # the client's own ini, exactly as it was
        if not a.keep_up and not a.no_boot:
            subprocess.run([sys.executable, "-I", str(ROOT / "tools" / "runner" / "run.py"), "down"], cwd=str(ROOT), capture_output=True)
            if REMOTE[0] is not None:
                REMOTE[0].stop()


def aggregate(results, label):
    """Median per scored metric across runs; spread = (max - min) and the composite's coefficient of variation (noise floor)."""
    med = {}
    for met in TARGETS:
        vals = [r["scores"][met]["value"] for r in results if r["scores"].get(met, {}).get("value") is not None]
        if vals:
            med[met] = {"median": pct(vals, 0.5), "min": min(vals), "max": max(vals), "runs": len(vals)}
    # re-score the medians
    overall = {m: v["median"] for m, v in med.items()}
    scores, cats = {}, {}
    for met in TARGETS:
        v = overall.get(met)
        scores[met] = {"value": v, "score": score(met, v), "good": TARGETS[met][0], "bad": TARGETS[met][1],
                       "min": med.get(met, {}).get("min"), "max": med.get(met, {}).get("max")}
    for c, mets in CATEGORIES.items():
        sc = [scores[x]["score"] for x in mets if scores[x]["score"] is not None]
        cats[c] = round(sum(sc) / len(sc), 1) if sc else None
    comps = [r["composite"] for r in results if r["composite"] is not None]
    vals = [v for c, v in cats.items() if v is not None and c not in NOT_IN_COMPOSITE]
    comp = round(sum(vals) / len(vals), 1) if vals else None
    cov = None
    if len(comps) >= 2:
        mean = sum(comps) / len(comps)
        sd = (sum((x - mean) ** 2 for x in comps) / (len(comps) - 1)) ** 0.5
        cov = round(100.0 * sd / mean, 1) if mean else None
    return {"label": label, "runs": len(results), "composite": comp, "composite_each": comps, "composite_cov_pct": cov,
            "categories": cats, "scores": scores, "folders": [r["meta"]["stamp"] for r in results]}


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--no-boot", action="store_true", help="use the two copies already up (and in the world, boxes cleared); with --runs only one run is possible")
    ap.add_argument("--save", default="nvcoop_autosave_ready")
    ap.add_argument("--label", default="")
    ap.add_argument("--quick", action="store_true")
    ap.add_argument("--baseline", default="")
    ap.add_argument("--runs", type=int, default=1, help="repeat the whole benchmark N times, each on a fresh boot; report the median and the spread")
    ap.add_argument("--keep-up", action="store_true", help="leave the copies running afterwards")
    ap.add_argument("--walk-mode", default=None, choices=["step", "input", "walk"],
                    help="how the bots move: step (positions in small increments, repeatable) or input (real movement input, needs open ground: use the arena). "
                         "Default: the scenario's own walk_mode, else step")
    ap.add_argument("--net", action="store_true", help="add the bad-network matrix: the client joins through the chaos relay and the profiles lan, good_wifi, "
                    "bad_wifi, mobile_tether, transatlantic, hell are stepped through (about 4 extra minutes)")
    ap.add_argument("--no-area", action="store_true", help="skip the area-change phase")
    ap.add_argument("--no-playbooks", action="store_true", help="skip the fight and loot playbooks")
    ap.add_argument("--no-probe", action="store_true", help="skip the input-walk probe")
    ap.add_argument("--scenario", default="", help="run a scenario file instead of the built-in phases: a name in tools/bench/scenarios (standard, smoke) or a path")
    ap.add_argument("--arena", default="", help="arena layout file (default tools/bench/arena.json; without one the courses start where each player stands)")
    ap.add_argument("--full", action="store_true", help="everything: same as --net (the area, playbook and probe phases are on by default)")
    ap.add_argument("--remote", default="", help="ssh name of a second PC that runs the HOST copy (this PC runs the client): see tools/bench/remote.py")
    ap.add_argument("--remote-ip", default="192.168.0.10", help="that PC's address on the local network (the client joins it there)")
    a = ap.parse_args()
    if a.remote:
        if a.no_boot:
            raise SystemExit("--remote needs a fresh boot: drop --no-boot")
        import remote as remote_mod
        REMOTE[0] = remote_mod.RemoteHost(ssh=a.remote, ip=a.remote_ip, log=lambda s: print("   " + s, flush=True))
    if a.full:
        a.net = True
    if a.scenario:
        import bench_scenario as _sc
        if any(s["phase"] == "net" for s in _sc.load(a.scenario)["steps"]):
            a.net = True   # the scenario steps through the bad-network profiles, so the client must join through the relay
    if a.net and a.no_boot:
        raise SystemExit("--net needs a fresh boot (the client must join through the relay): drop --no-boot")
    if not a.walk_mode:
        a.walk_mode = "step"
        if a.scenario:
            import bench_scenario as _sc2
            a.walk_mode = _sc2.load(a.scenario).get("walk_mode", "step")
    WALK_MODE[0] = a.walk_mode
    if a.no_boot and a.runs > 1:
        raise SystemExit("--runs N needs a fresh boot per run: drop --no-boot")

    results, folders = [], []
    for k in range(max(1, a.runs)):
        stamp = time.strftime("%Y%m%d-%H%M%S")
        label = a.label + (f" run{k + 1}" if a.runs > 1 else "")
        res, out = run_once(a, label.strip(), stamp)
        results.append(res)
        folders.append(out)
        with open(ROOT / "out" / "bench" / "history.jsonl", "a", encoding="utf-8") as f:
            f.write(json.dumps({"stamp": stamp, "label": label.strip(), "composite": res["composite"], "categories": res["categories"],
                                "key": {m: v["value"] for m, v in res["scores"].items()}}) + "\n")
        print(f"--- run {k + 1}/{a.runs}: composite {res['composite']}  {res['categories']}", flush=True)
    if a.runs > 1:
        agg = aggregate(results, a.label)
        agg_dir = ROOT / "out" / "bench" / (time.strftime("%Y%m%d-%H%M%S") + "-agg")
        agg_dir.mkdir(parents=True, exist_ok=True)
        (agg_dir / "results.json").write_text(json.dumps(agg, indent=1), encoding="utf-8")
        lines = [f"# Co-op benchmark, {a.runs} runs ({a.label})", "",
                 f"Composite (median metrics): **{agg['composite']}**; each run: {agg['composite_each']}; noise (coefficient of variation): {agg['composite_cov_pct']} %",
                 "A change smaller than the noise is not a change.", "", "| Metric | Median | Min | Max | Score |", "|---|---|---|---|---|"]
        for m, v in agg["scores"].items():
            lines.append(f"| {m} | {v['value']} | {v['min']} | {v['max']} | {v['score']} |")
        lines += ["", "Folders: " + ", ".join(agg["folders"])]
        (agg_dir / "report.md").write_text("\n".join(lines), encoding="utf-8")
        print("\n".join(lines))
        print("aggregate folder:", agg_dir)
    else:
        print((folders[0] / "report.md").read_text(encoding="utf-8")[:2600])
    return 0


if __name__ == "__main__":
    sys.exit(main())
