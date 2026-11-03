#!/usr/bin/env python3
"""Launch and judge scripted two-process (host + guest) nv-rs viewer runs.

    python -I tools/coop-run/runner.py run    SCENARIO.json [--config F] [--seed N] [--out DIR]
    python -I tools/coop-run/runner.py clean  SCENARIO.json [--n 3] [--vary-seed]
    python -I tools/coop-run/runner.py suite  DIR_OR_FILES... [--n 3]
    python -I tools/coop-run/runner.py selftest
    python -I tools/coop-run/runner.py run SCENARIO.json --attach host:PORT guest:PORT
        (replay it on viewers already running with --coop-control; see live.py)

Each run writes <out>/<scenario>/<stamp>-<n>/verdict.json plus the raw logs. The last
output line is `RESULT: PASS ...` or `RESULT: FAIL <class> ...`. Exit code 0 pass, 1 fail,
2 usage or config error.

Safety: this script never deletes anything and only stops processes it started itself
(through their own Popen handles). It sends no mouse or keyboard input.
"""
import argparse
import json
import os
import socket
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

import scenario as scn_mod  # noqa: E402
import verdict as verdict_mod  # noqa: E402

DEFAULT_CONFIG = os.path.join(HERE, "coop-run.local.json")
# Seconds the runner waits for a relay to write its ready file, and the
# number of starts before the run is a harness failure.
RELAY_START_S = 15
RELAY_TRIES = 3


class ConfigError(Exception):
    pass


def load_config(path=None):
    path = path or DEFAULT_CONFIG
    if not os.path.exists(path):
        raise ConfigError("config %s not found; copy config.example.json to coop-run.local.json and edit it" % path)
    with open(path, encoding="utf-8") as f:
        c = json.load(f)
    viewer = c.get("viewer")
    if isinstance(viewer, str):
        viewer = [viewer]
    if not viewer or not isinstance(viewer, list):
        raise ConfigError("config needs viewer (exe path, or a list: exe plus leading arguments)")
    c["viewer"] = viewer
    if not c.get("data"):
        raise ConfigError("config needs data (the game's Data folder)")
    c.setdefault("out_dir", os.path.join(HERE, "out"))
    c.setdefault("base_port", 27960)
    c.setdefault("launch_timeout_s", 90)
    c.setdefault("grace_s", 15)
    c.setdefault("python", sys.executable)
    c.setdefault("background", True)
    c.setdefault("screen_size", "640,360")
    return c


def free_udp_port(start):
    # Probe the wildcard address: the viewer binds 0.0.0.0. On Windows a bind on
    # 127.0.0.1 succeeds while another socket holds 0.0.0.0 on the same port.
    for p in range(start, start + 200):
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        try:
            s.bind(("0.0.0.0", p))
            return p
        except OSError:
            continue
        finally:
            s.close()
    raise ConfigError("no free UDP port from %d" % start)


def _read_events(path):
    return verdict_mod.read_jsonl(path)[0]


def _has_event(path, name):
    return any(e["ev"] == name for e in _read_events(path))


class Proc:
    """A child process this run started. The only things we ever stop."""

    def __init__(self, role, cmd, stdout_path, cwd=None):
        self.role = role
        self.cmd = cmd
        self.stdout_path = stdout_path
        self.p = None
        self.launch_error = None
        self.timed_out = False
        try:
            self.out = open(stdout_path, "wb")
            self.p = subprocess.Popen(cmd, stdout=self.out, stderr=subprocess.STDOUT, cwd=cwd)
        except OSError as e:
            self.launch_error = "%s: %s" % (type(e).__name__, e)

    def alive(self):
        return self.p is not None and self.p.poll() is None

    def stop(self):
        """Ask nicely is the caller's job (quit file); this is the forced end for our own child."""
        if self.alive():
            self.p.terminate()
            try:
                self.p.wait(5)
            except subprocess.TimeoutExpired:
                self.p.kill()
                self.p.wait(5)

    def close(self):
        try:
            self.out.close()
        except (OSError, AttributeError):
            pass

    def tail(self, n=300):
        try:
            self.out.flush()
            with open(self.stdout_path, "rb") as f:
                return f.read()[-n:].decode("utf-8", "replace")
        except OSError:
            return ""


def _touch(path):
    with open(path, "w", encoding="utf-8") as f:
        f.write("quit\n")


def rerun_hint(scn_path, cfg_path, seed):
    c = " --config %s" % cfg_path if cfg_path else ""
    return "python -I tools/coop-run/runner.py run %s --seed %d%s" % (scn_path, seed, c)


def run_scenario(scn, cfg, run_dir, seed=None, scn_path="", cfg_path="", slot=0, log=print):
    """Run one scenario once. Returns the verdict dict (also written to run_dir/verdict.json).
    A scenario with "guests": [..] runs one host and up to three guests, each guest on its own
    relay (and so its own seeded chaos stream) to the host."""
    seed = scn["seed"] if seed is None else seed
    os.makedirs(run_dir, exist_ok=True)
    f = lambda n: os.path.join(run_dir, n)  # noqa: E731
    started = time.monotonic()
    deadline = started + scn["timeout_s"]
    n_guests = len(scn.get("guests") or [1])
    roles = scn_mod.ROLES[:1 + n_guests]
    host_port = free_udp_port(cfg["base_port"] + 8 * slot)
    relay_ports = []
    nxt = host_port + 1
    for _ in range(n_guests):
        rp = free_udp_port(nxt)
        relay_ports.append(rp)
        nxt = rp + 1
    files = {r: {"log": f(r + ".jsonl"), "state": f(r + ".state.json"), "quit": f(r + ".quit"),
                 "go": f(r + ".go")} for r in roles}
    run = {"run_id": os.path.basename(run_dir), "seed": seed, "rerun": rerun_hint(scn_path, cfg_path, seed),
           "relay": None, "timed_out_run": False, "harness_error": None}
    for r in roles:
        run[r] = {"log": files[r]["log"], "state": files[r]["state"]}
    procs = {}
    relay_procs = []

    def relay_name(i, ext):
        return f("relay.%s" % ext) if i == 0 else f("relay%d.%s" % (i + 1, ext))

    def start_relay(i, cmd):
        """Starts relay i and waits for its ready file. A busy machine can stall a new
        Python process for 12 s to more than 30 s (seen in full suites, median start
        0.1 s). The runner stops a stalled relay and starts it again, RELAY_TRIES times."""
        tail = ""
        for attempt in range(RELAY_TRIES):
            out = relay_name(i, "stdout.txt") if attempt == 0 else relay_name(i, "try%d.stdout.txt" % (attempt + 1))
            rproc = Proc("relay", cmd, out)
            if rproc.launch_error:
                raise RuntimeError("relay: " + rproc.launch_error)
            _wait(lambda: os.path.exists(relay_name(i, "ready")) or not rproc.alive(), RELAY_START_S)
            if os.path.exists(relay_name(i, "ready")):
                return rproc
            tail = rproc.tail()
            rproc.stop()
            rproc.close()
            run["relay_restarts"] = run.get("relay_restarts", 0) + 1
            log("  relay %d wrote no ready file in %d s (try %d of %d)" % (i + 1, RELAY_START_S, attempt + 1, RELAY_TRIES))
        raise RuntimeError("relay did not start: " + tail)

    stats_path = relay_name(0, "stats.json")
    try:
        sched = scn_mod.relay_schedule(scn)
        with open(f("relay.schedule.json"), "w", encoding="utf-8") as fh:
            json.dump(sched, fh)
        for i, rp in enumerate(relay_ports):
            relay_cmd = [cfg["python"], "-I", os.path.join(HERE, "chaos_relay.py"),
                         "--listen", str(rp), "--target", "127.0.0.1:%d" % host_port,
                         "--profile", scn["network"], "--seed", str(seed + 7919 * i), "--schedule", f("relay.schedule.json"),
                         "--stats", relay_name(i, "stats.json"), "--log", relay_name(i, "log.jsonl"),
                         "--stop-file", relay_name(i, "stop"), "--ready-file", relay_name(i, "ready")]
            relay_procs.append(start_relay(i, relay_cmd))

        # a quit request must outlast the script plus the settle window; the backstop is longer
        script_end = scn["duration_s"] + scn["settle_s"]
        backstop = script_end + cfg["grace_s"] + 30
        args = {"host": scn_mod.viewer_args(scn, "host", cfg["data"], {"host": host_port}, files["host"], backstop,
                                            cfg["background"], cfg["screen_size"])}
        for i, r in enumerate(roles[1:]):
            args[r] = scn_mod.viewer_args(scn, r, cfg["data"], {"join": "127.0.0.1:%d" % relay_ports[i]},
                                          files[r], backstop, cfg["background"], cfg["screen_size"])
        log("host  port %d, relay port(s) %s, profile %s, seed %d, %d guest(s)" %
            (host_port, ",".join(str(p) for p in relay_ports), scn["network"], seed, n_guests))
        procs["host"] = Proc("host", cfg["viewer"] + args["host"], f("host.stdout.txt"))
        if not procs["host"].launch_error:
            _wait(lambda: _has_event(files["host"]["log"], "coop_listening") or not procs["host"].alive(),
                  min(cfg["launch_timeout_s"], deadline - time.monotonic()))
        if not procs["host"].launch_error and procs["host"].alive():
            # guests join one at a time, in launch order: the host gives slots in join
            # order, so "guest" is slot 1, "guest2" slot 2 (the scenarios' paths say so)
            for i, r in enumerate(roles[1:]):
                if i > 0:
                    prev = roles[i]
                    _wait(lambda: _has_event(files[prev]["log"], "coop_connected") or not procs[prev].alive(),
                          min(cfg["launch_timeout_s"], max(0, deadline - time.monotonic())))
                procs[r] = Proc(r, cfg["viewer"] + args[r], f(r + ".stdout.txt"))
        all_up = all(r in procs and not procs[r].launch_error for r in roles)

        def ready_all():
            return all(_has_event(files[r]["log"], "coop_ready") for r in roles)

        def all_alive():
            return all(p.alive() for p in procs.values())

        both_ready = False
        if all_up:
            both_ready = _wait(lambda: ready_all() or not all_alive(),
                               min(cfg["launch_timeout_s"], deadline - time.monotonic()))
            both_ready = both_ready and ready_all()
        if both_ready:
            log("all %d peers ready; running %.0f s + %.0f s settle" % (len(roles), scn["duration_s"], scn["settle_s"]))
            # script time starts now on every peer (README "Timeline")
            for r in roles:
                _touch(files[r]["go"])
            end = min(time.monotonic() + script_end, deadline)
            _wait(lambda: not all_alive(), max(0, end - time.monotonic()))
            if time.monotonic() >= deadline:
                run["timed_out_run"] = True
        # quit request, then wait for our own children to leave
        for r in roles:
            _touch(files[r]["quit"])
        grace_end = time.monotonic() + cfg["grace_s"]
        _wait(lambda: not any(p.alive() for p in procs.values()), max(0, grace_end - time.monotonic()))
        for r, p in procs.items():
            if p.alive():
                p.timed_out = True
                p.stop()
    except Exception as e:  # the runner itself failed
        run["harness_error"] = "%s: %s" % (type(e).__name__, e)
    finally:
        for p in procs.values():
            if p.alive():
                p.timed_out = True
                p.stop()
        for i, rproc in enumerate(relay_procs):
            if not rproc.launch_error:
                _touch(relay_name(i, "stop"))
                _wait(lambda: not rproc.alive(), 5)
                rproc.stop()
    for r in roles:
        p = procs.get(r)
        if p is not None:
            run[r].update(exit_code=None if p.launch_error else p.p.returncode, timed_out=p.timed_out,
                          launch_error=p.launch_error, stdout_tail=p.tail())
        else:
            run[r].update(exit_code=None, timed_out=False, launch_error=None if run["harness_error"] is None else "not started",
                          stdout_tail="")
    for p in list(procs.values()) + relay_procs:
        p.close()
    run["relay"] = verdict_mod.read_json(stats_path)
    run["duration_s"] = time.monotonic() - started
    v = verdict_mod.judge(scn, run)
    with open(f("verdict.json"), "w", encoding="utf-8") as fh:
        json.dump(v, fh, indent=2)
    return v


def _wait(cond, timeout, poll=0.1):
    end = time.monotonic() + max(0, timeout)
    while True:
        if cond():
            return True
        if time.monotonic() >= end:
            return False
        time.sleep(poll)


def stamp():
    return time.strftime("%Y%m%d-%H%M%S")


def line(v):
    if v["result"] == "pass" and v["class"] == "twin":
        return "RESULT: PASS twin %s (%s) %s" % (v["scenario"], v["network"], v["reason"][:140])
    if v["result"] == "pass":
        return "RESULT: PASS %s (%s, seed %s, %.0f s)" % (v["scenario"], v["network"], v["seed"], v["duration_s"])
    return "RESULT: FAIL %s %s: %s | rerun: %s" % (v["class"], v["scenario"], v["reason"], v["rerun"])


def clean_runs(scn, cfg, out_base, n=3, vary_seed=False, scn_path="", cfg_path="", log=print):
    """The N clean runs rule: n consecutive passes (default 3). The first failure ends it.
    Writes <out>/<scenario>/clean-<stamp>.json."""
    sdir = os.path.join(out_base, scn["name"])
    os.makedirs(sdir, exist_ok=True)
    st = stamp()
    runs, ok = [], True
    for i in range(n):
        seed = scn["seed"] + (i if vary_seed else 0)
        v = run_scenario(scn, cfg, os.path.join(sdir, "%s-%d" % (st, i + 1)), seed, scn_path, cfg_path, log=log)
        log("  run %d/%d: %s" % (i + 1, n, line(v)))
        runs.append({"run_id": v["run_id"], "result": v["result"], "class": v["class"], "seed": v["seed"]})
        if v["result"] != "pass":
            ok = False
            break
    summary = {"schema": 1, "scenario": scn["name"], "n": n, "clean": ok and len(runs) == n,
               "passes": sum(1 for r in runs if r["result"] == "pass"), "runs": runs,
               "note": "" if ok else "failed on run %d after %d pass(es); rerun with the same seed to see if it repeats"
               % (len(runs), len(runs) - 1)}
    with open(os.path.join(sdir, "clean-%s.json" % st), "w", encoding="utf-8") as fh:
        json.dump(summary, fh, indent=2)
    return summary, v


def main(argv=None):
    ap = argparse.ArgumentParser(description="two-process co-op run harness for nv-viewer")
    sub = ap.add_subparsers(dest="cmd", required=True)
    for name in ("run", "clean", "suite"):
        sp = sub.add_parser(name)
        sp.add_argument("scenarios", nargs="+")
        sp.add_argument("--config")
        sp.add_argument("--out")
        sp.add_argument("--seed", type=int)
        if name == "run":
            sp.add_argument("--attach", nargs=2, metavar=("HOST:PORT", "GUEST:PORT"),
                            help="run against viewers already started with --coop-control (no launch, no relay)")
        if name != "run":
            sp.add_argument("--n", type=int, default=3)
            sp.add_argument("--vary-seed", action="store_true")
    sub.add_parser("selftest")
    a = ap.parse_args(argv)

    if a.cmd == "selftest":
        probs = verdict_mod.scorer_selftest()
        for p in probs:
            print("SCORER SELFTEST FAIL:", p)
        print("RESULT: %s scorer selftest" % ("FAIL" if probs else "PASS"))
        return 1 if probs else 0
    if a.cmd == "run" and a.attach:
        import live
        try:
            s = scn_mod.load(a.scenarios[0])
            host, guest = live.peer_from(a.attach[0]), live.peer_from(a.attach[1])
        except (scn_mod.ScenarioError, live.LiveError, OSError, ValueError) as e:
            print("RESULT: FAIL config: %s" % e)
            return 2
        res = live.replay(s, host, guest, log=print)
        live.report(s, res)
        return 0 if res["passed"] else 1
    try:
        cfg = load_config(a.config)
        files = []
        for s in a.scenarios:
            if os.path.isdir(s):
                files += [os.path.join(s, x) for x in sorted(os.listdir(s)) if x.endswith(".json")]
            else:
                files.append(s)
        scns = [(p, scn_mod.load(p)) for p in files]
        for p, s in scns:
            if s.get("headless_only"):
                print("SKIP %s: headless-only (it restarts both peers from a party save); run it with coop-sim" % p)
        scns = [(p, s) for p, s in scns if not s.get("headless_only")]
    except (ConfigError, scn_mod.ScenarioError, OSError, ValueError) as e:
        print("RESULT: FAIL config: %s" % e)
        return 2
    out = a.out or cfg["out_dir"]
    if a.cmd == "run":
        if len(scns) != 1:
            print("run takes one scenario")
            return 2
        p, s = scns[0]
        d = os.path.join(out, s["name"], "%s-1" % stamp())
        v = run_scenario(s, cfg, d, a.seed, p, a.config or "")
        print(line(v))
        print("verdict: " + os.path.join(d, "verdict.json"))
        return 0 if v["result"] == "pass" else 1
    if a.cmd == "suite":
        probs = verdict_mod.scorer_selftest()  # first gate: the scorer can fail
        if probs:
            print("RESULT: FAIL harness: scorer selftest failed: " + "; ".join(probs))
            return 1
    worst = 0
    results = []
    for p, s in scns:
        summary, last = clean_runs(s, cfg, out, a.n, a.vary_seed, p, a.config or "")
        results.append(summary)
        if not summary["clean"]:
            worst = 1
            print(line(last))
    print("SUMMARY: %d/%d scenarios clean (%d consecutive passes each)" %
          (sum(1 for r in results if r["clean"]), len(results), a.n))
    print("RESULT: %s" % ("PASS" if worst == 0 else "FAIL"))
    return worst


if __name__ == "__main__":
    sys.exit(main())
