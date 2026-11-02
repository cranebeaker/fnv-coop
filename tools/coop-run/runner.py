#!/usr/bin/env python3
"""Launch and judge scripted two-process (host + guest) nv-rs viewer runs.

    python -I tools/coop-run/runner.py run    SCENARIO.json [--config F] [--seed N] [--out DIR]
    python -I tools/coop-run/runner.py clean  SCENARIO.json [--n 3] [--vary-seed]
    python -I tools/coop-run/runner.py suite  DIR_OR_FILES... [--n 3]
    python -I tools/coop-run/runner.py selftest

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
    for p in range(start, start + 200):
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        try:
            s.bind(("127.0.0.1", p))
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
    """Run one scenario once. Returns the verdict dict (also written to run_dir/verdict.json)."""
    seed = scn["seed"] if seed is None else seed
    os.makedirs(run_dir, exist_ok=True)
    f = lambda n: os.path.join(run_dir, n)  # noqa: E731
    started = time.monotonic()
    deadline = started + scn["timeout_s"]
    host_port = free_udp_port(cfg["base_port"] + 4 * slot)
    relay_port = free_udp_port(host_port + 1)
    files = {r: {"log": f(r + ".jsonl"), "state": f(r + ".state.json"), "quit": f(r + ".quit")} for r in ("host", "guest")}
    run = {"run_id": os.path.basename(run_dir), "seed": seed, "rerun": rerun_hint(scn_path, cfg_path, seed),
           "host": {"log": files["host"]["log"], "state": files["host"]["state"]},
           "guest": {"log": files["guest"]["log"], "state": files["guest"]["state"]},
           "relay": None, "timed_out_run": False, "harness_error": None}
    procs = {}
    relay_proc = None
    stats_path = f("relay.stats.json")
    try:
        sched = scn_mod.relay_schedule(scn)
        with open(f("relay.schedule.json"), "w", encoding="utf-8") as fh:
            json.dump(sched, fh)
        relay_cmd = [cfg["python"], "-I", os.path.join(HERE, "chaos_relay.py"),
                     "--listen", str(relay_port), "--target", "127.0.0.1:%d" % host_port,
                     "--profile", scn["network"], "--seed", str(seed), "--schedule", f("relay.schedule.json"),
                     "--stats", stats_path, "--log", f("relay.log.jsonl"), "--stop-file", f("relay.stop"),
                     "--ready-file", f("relay.ready")]
        relay_proc = Proc("relay", relay_cmd, f("relay.stdout.txt"))
        if relay_proc.launch_error:
            raise RuntimeError("relay: " + relay_proc.launch_error)
        _wait(lambda: os.path.exists(f("relay.ready")) or not relay_proc.alive(), 10)
        if not os.path.exists(f("relay.ready")):
            raise RuntimeError("relay did not start: " + relay_proc.tail())

        # a quit request must outlast the script plus the settle window; the backstop is longer
        script_end = scn["duration_s"] + scn["settle_s"]
        backstop = script_end + cfg["grace_s"] + 30
        host_args = scn_mod.viewer_args(scn, "host", cfg["data"], {"host": host_port}, files["host"], backstop,
                                        cfg["background"], cfg["screen_size"])
        guest_args = scn_mod.viewer_args(scn, "guest", cfg["data"], {"join": "127.0.0.1:%d" % relay_port},
                                         files["guest"], backstop, cfg["background"], cfg["screen_size"])
        log("host  port %d, relay port %d, profile %s, seed %d" % (host_port, relay_port, scn["network"], seed))
        procs["host"] = Proc("host", cfg["viewer"] + host_args, f("host.stdout.txt"))
        if not procs["host"].launch_error:
            _wait(lambda: _has_event(files["host"]["log"], "coop_listening") or not procs["host"].alive(),
                  min(cfg["launch_timeout_s"], deadline - time.monotonic()))
        if not procs["host"].launch_error and procs["host"].alive():
            procs["guest"] = Proc("guest", cfg["viewer"] + guest_args, f("guest.stdout.txt"))
        both_ready = False
        if "guest" in procs and not procs["guest"].launch_error:
            both_ready = _wait(
                lambda: (_has_event(files["host"]["log"], "coop_ready") and _has_event(files["guest"]["log"], "coop_ready"))
                or not (procs["host"].alive() and procs["guest"].alive()),
                min(cfg["launch_timeout_s"], deadline - time.monotonic()))
            both_ready = both_ready and _has_event(files["host"]["log"], "coop_ready") and _has_event(files["guest"]["log"], "coop_ready")
        if both_ready:
            log("both peers ready; running %.0f s + %.0f s settle" % (scn["duration_s"], scn["settle_s"]))
            end = min(time.monotonic() + script_end, deadline)
            _wait(lambda: not (procs["host"].alive() and procs["guest"].alive()), max(0, end - time.monotonic()))
            if time.monotonic() >= deadline:
                run["timed_out_run"] = True
        # quit request, then wait for our own children to leave
        for r in ("host", "guest"):
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
        if relay_proc and not relay_proc.launch_error:
            _touch(f("relay.stop"))
            _wait(lambda: not relay_proc.alive(), 5)
            relay_proc.stop()
    for r in ("host", "guest"):
        p = procs.get(r)
        if p is not None:
            run[r].update(exit_code=None if p.launch_error else p.p.returncode, timed_out=p.timed_out,
                          launch_error=p.launch_error, stdout_tail=p.tail())
        else:
            run[r].update(exit_code=None, timed_out=False, launch_error=None if run["harness_error"] is None else "not started",
                          stdout_tail="")
    for p in list(procs.values()) + ([relay_proc] if relay_proc else []):
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
    try:
        cfg = load_config(a.config)
        files = []
        for s in a.scenarios:
            if os.path.isdir(s):
                files += [os.path.join(s, x) for x in sorted(os.listdir(s)) if x.endswith(".json")]
            else:
                files.append(s)
        scns = [(p, scn_mod.load(p)) for p in files]
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
