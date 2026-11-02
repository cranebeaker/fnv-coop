#!/usr/bin/env python3
"""Live control of running nv-rs viewers: client library and CLI (Python 3 stdlib only).

The viewer's `--coop-control PORT` opens a JSON-lines TCP port on 127.0.0.1 (see
viewer/src/coop_control.rs and crates/coop-session/src/control.rs). This module talks to it.

    python -I tools/coop-run/live.py up --cell GSDocMitchellHouse [--visible]
    python -I tools/coop-run/live.py state host            # one line; --json whole reply; --path items.player
    python -I tools/coop-run/live.py run guest "SetStage MyQuest 10"      # --local to keep it on this peer
    python -I tools/coop-run/live.py bot host "teleport 100 200 0"
    python -I tools/coop-run/live.py keys guest w --hold 2
    python -I tools/coop-run/live.py shot host C:/tmp/host.png
    python -I tools/coop-run/live.py entities guest --visible
    python -I tools/coop-run/live.py watch tools/coop-run/scenarios/live_loot_sequence.json
    python -I tools/coop-run/live.py down

`HOST` in the commands is `host`, `guest` (from the session file `up` writes), a port number, or
`127.0.0.1:PORT`, so any viewer started with --coop-control works. Exit code 0 ok, 1 a command
failed or an assertion failed, 2 usage, connection or config error.

Safety: only processes `up` started are ever stopped (quit through the port first, then the pid it
recorded). Nothing is deleted. No mouse or keyboard input is sent anywhere.
"""
import argparse
import json
import os
import signal
import socket
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

import scenario as scn_mod  # noqa: E402

LOCAL = "127.0.0.1"
DEFAULT_CONTROL_BASE = 27990


class LiveError(Exception):
    pass


# ---- client --------------------------------------------------------------------------------


class Peer:
    """One viewer's control port. Each call opens its own short connection."""

    def __init__(self, name, port, host=LOCAL, timeout=40.0):
        self.name = name
        self.port = int(port)
        self.host = host
        self.timeout = timeout
        self._id = 0

    def __repr__(self):
        return "Peer(%s:%d)" % (self.name, self.port)

    def call(self, check=True, timeout=None, **req):
        """Send one request; returns the reply dict. With check, raises LiveError on ok:false."""
        self._id += 1
        req["id"] = self._id
        try:
            with socket.create_connection((self.host, self.port), timeout=timeout or self.timeout) as s:
                s.settimeout(timeout or self.timeout)
                s.sendall((json.dumps(req) + "\n").encode("utf-8"))
                buf = b""
                while not buf.endswith(b"\n"):
                    chunk = s.recv(1 << 20)
                    if not chunk:
                        raise LiveError("%s: connection closed without a reply" % self.name)
                    buf += chunk
        except (OSError, socket.timeout) as e:
            raise LiveError("%s (port %d): %s" % (self.name, self.port, e))
        try:
            reply = json.loads(buf.decode("utf-8"))
        except ValueError as e:
            raise LiveError("%s: bad reply (%s): %r" % (self.name, e, buf[:200]))
        if reply.get("id") not in (None, self._id):
            raise LiveError("%s: reply id %r is not %r" % (self.name, reply.get("id"), self._id))
        if check and not reply.get("ok"):
            raise LiveError("%s %s: %s" % (self.name, req.get("cmd"), reply.get("error", "failed")))
        return reply

    def state(self):
        return self.call(cmd="state")

    def entities(self, visible=None):
        r = self.call(cmd="entities")
        if visible is not None:
            r["entities"] = [e for e in r["entities"] if e["visible"] == visible]
        return r

    def run(self, line, local=False):
        return self.call(cmd="run", line=line, local=bool(local))

    def bot(self, line):
        return self.call(cmd="bot", line=line)

    def keys(self, key, hold=0.1, delay=0.0):
        return self.call(cmd="keys", key=key, hold=hold, delay=delay)

    def shot(self, path):
        return self.call(cmd="screenshot", path=os.path.abspath(path).replace("\\", "/"), timeout=60)

    def quit(self):
        return self.call(cmd="quit", check=False, timeout=10)

    def up(self):
        """True while the port answers."""
        try:
            with socket.create_connection((self.host, self.port), timeout=1):
                return True
        except OSError:
            return False

    def wait_ready(self, timeout=180, poll=0.5):
        """Waits until the viewer answers `state` with ready:true; returns that reply."""
        end = time.monotonic() + timeout
        last = "no answer"
        while time.monotonic() < end:
            try:
                r = self.call(cmd="state", check=False, timeout=10)
                if r.get("ready"):
                    return r
                last = "not ready yet"
            except LiveError as e:
                last = str(e)
            time.sleep(poll)
        raise LiveError("%s did not become ready in %.0f s (%s)" % (self.name, timeout, last))


def split_state(reply):
    """A `state` reply as the dict assertions run on: the dump, plus the live figures under `live`."""
    s = dict(reply.get("state") or {})
    s["live"] = {k: v for k, v in reply.items() if k != "state"}
    return s


# ---- replaying a scenario against running viewers ----------------------------------------------


def timeline(scn):
    """[(at, role, kind, args)] for the scenario's scripts, in time order. Startup-only
    flags (talk, choose, say) cannot be replayed live and are returned as ('skip', ...)."""
    out = []
    for role in ("host", "guest"):
        for ln in scn[role]["script"]:
            at = float(ln.get("at", 0))
            if "run" in ln:
                out.append((at, role, "run", ln["run"]))
            elif "stage" in ln:
                out.append((at, role, "run", "SetStage " + ln["stage"]))
            elif "key" in ln:
                out.append((at, role, "key", ln["key"]))
            else:
                out.append((at, role, "skip", [k for k in scn_mod.SCRIPT_KEYS if k in ln][0]))
    return sorted(out, key=lambda x: x[0])


def dispatch(peer, kind, arg):
    if kind == "key":
        name, _, hold = arg.partition(":")
        return peer.keys(name, float(hold) if hold else 0.1)
    low = arg.strip().lower()
    if low.startswith("coop "):
        return peer.bot(arg)
    if low.startswith("local "):
        return peer.run(arg.strip()[6:], local=True)
    return peer.run(arg)


def replay(scn, host, guest, log=print, poll=0.4, clock=time.monotonic, sleep=time.sleep):
    """Replays the scenario's scripts on the running pair and judges its `expect` list.

    The assertions are only judged after the last scripted command went out, then every `poll`
    seconds until they all hold or settle_s runs out. Returns {passed, results, elapsed, errors}."""
    peers = {"host": host, "guest": guest}
    t0 = clock()
    errors = []
    for role, p in peers.items():
        try:
            if not p.state().get("ready"):
                errors.append("%s is not ready (still loading or not connected)" % role)
        except LiveError as e:
            errors.append(str(e))
    if errors:
        return {"passed": False, "results": [], "elapsed": clock() - t0, "errors": errors}
    events = timeline(scn)
    start = clock()
    for at, role, kind, arg in events:
        wait = start + at - clock()
        if wait > 0:
            sleep(wait)
        if kind == "skip":
            log("  skip %s.%s (a startup flag, not replayable live)" % (role, arg))
            continue
        try:
            r = dispatch(peers[role], kind, arg)
            log("  %5.1fs %-5s %s -> %s" % (clock() - start, role, arg, r.get("how", r.get("flow", "ok"))))
        except LiveError as e:
            errors.append(str(e))
            log("  %5.1fs %-5s %s -> ERROR %s" % (clock() - start, role, arg, e))
    deadline = clock() + max(0.0, float(scn.get("settle_s", 3)))
    results = []
    while True:
        sleep(poll)
        try:
            hs = split_state(host.state())
            gs = split_state(guest.state())
        except LiveError as e:
            errors.append(str(e))
            break
        results = scn_mod.evaluate(scn["expect"], hs, gs)
        if all(r["ok"] for r in results) or clock() >= deadline:
            break
    passed = not errors and bool(results or not scn["expect"]) and all(r["ok"] for r in results)
    return {"passed": passed, "results": results, "elapsed": clock() - t0, "errors": errors}


def report(scn, res, out=print):
    """Prints PASS/FAIL per assertion and the closing RESULT line."""
    for e in res["errors"]:
        out("ERROR %s" % e)
    for r in res["results"]:
        out("%s  %-28s %s" % ("PASS" if r["ok"] else "FAIL", r["id"], r["detail"]))
    n = len(res["results"])
    ok = sum(1 for r in res["results"] if r["ok"])
    out("RESULT: %s %s (%d/%d assertions, %.1f s)" % ("PASS" if res["passed"] else "FAIL", scn["name"], ok, n, res["elapsed"]))


# ---- session: the viewers `up` started -------------------------------------------------------------


def session_path(arg=None):
    if arg:
        return arg
    try:
        import runner
        base = runner.load_config(os.environ.get("COOP_RUN_CONFIG"))["out_dir"]
    except Exception:  # no config: the session file lives next to the tool
        base = os.path.join(HERE, "out")
    return os.path.join(base, "live", "session.json")


def load_session(path):
    try:
        with open(path, encoding="utf-8") as f:
            return json.load(f)
    except (OSError, ValueError):
        return None


def peer_from(arg, session_file=None):
    """host | guest | PORT | 127.0.0.1:PORT -> Peer."""
    if arg in ("host", "guest"):
        s = load_session(session_path(session_file))
        if not s:
            raise LiveError("no live session (%s); run `live.py up` first or give a port" % session_path(session_file))
        return Peer(arg, s[arg]["control"])
    port = arg.rsplit(":", 1)[-1]
    if not port.isdigit():
        raise LiveError("%r is not host, guest or a port" % arg)
    return Peer(arg, int(port))


def _free_tcp_port(start):
    for p in range(start, start + 200):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            s.bind((LOCAL, p))
            return p
        except OSError:
            continue
        finally:
            s.close()
    raise LiveError("no free TCP port from %d" % start)


def spawn(cmd, stdout_path):
    """Starts a viewer that outlives this process (its own process group)."""
    out = open(stdout_path, "wb")
    kw = {"creationflags": getattr(subprocess, "CREATE_NEW_PROCESS_GROUP", 0)} if os.name == "nt" else {"start_new_session": True}
    try:
        p = subprocess.Popen(cmd, stdout=out, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, **kw)
    except OSError as e:
        raise LiveError("could not start %s: %s" % (cmd[0], e))
    finally:
        out.close()
    return p.pid


def viewer_cmd(cfg, scn_like, role, ports, files, control_port, background):
    args = scn_mod.viewer_args(scn_like, role, cfg["data"], ports, files, 10 ** 6, background, cfg["screen_size"])
    return cfg["viewer"] + args + ["--coop-control", str(control_port)]


def cmd_up(a, out=print):
    import runner
    cfg = runner.load_config(a.config)
    if a.visible:
        cfg["background"] = False
    if a.background:
        cfg["background"] = True
    sp = session_path(a.session)
    old = load_session(sp)
    if old and any(Peer(r, old[r]["control"]).up() for r in ("host", "guest") if r in old):
        raise LiveError("a live session is already running (%s); `live.py down` first" % sp)
    d = os.path.dirname(sp)
    os.makedirs(d, exist_ok=True)
    scn_like = {"cell": a.cell, "new_game": a.new_game, "character": None, "host": {"script": [], "extra_args": a.extra},
                "guest": {"script": [], "extra_args": a.extra}}
    hp = runner.free_udp_port(cfg["base_port"])
    cp = {"host": _free_tcp_port(a.control_base)}
    cp["guest"] = _free_tcp_port(cp["host"] + 1)
    session = {"cell": a.cell, "udp_port": hp, "background": cfg["background"], "started": time.strftime("%Y-%m-%d %H:%M:%S")}
    for role in ("host", "guest"):
        files = {"log": os.path.join(d, role + ".jsonl"), "state": os.path.join(d, role + ".state.json"),
                 "quit": os.path.join(d, role + ".quit")}
        ports = {"host": hp} if role == "host" else {"join": "%s:%d" % (LOCAL, hp)}
        cmd = viewer_cmd(cfg, scn_like, role, ports, files, cp[role], cfg["background"])
        session[role] = {"control": cp[role], "cmd": cmd, "stdout": os.path.join(d, role + ".stdout.txt"), "pid": None}
    session["host"]["pid"] = spawn(session["host"]["cmd"], session["host"]["stdout"])
    with open(sp, "w", encoding="utf-8") as f:
        json.dump(session, f, indent=2)
    out("host  pid %d control port %d, waiting for it to load %s ..." % (session["host"]["pid"], cp["host"], a.cell))
    host = Peer("host", cp["host"])
    t = time.monotonic()
    try:
        # the host's port opens early; the guest may join as soon as it answers
        end = time.monotonic() + cfg["launch_timeout_s"] * 4
        while not host.up() and time.monotonic() < end:
            time.sleep(0.5)
        session["guest"]["pid"] = spawn(session["guest"]["cmd"], session["guest"]["stdout"])
        with open(sp, "w", encoding="utf-8") as f:
            json.dump(session, f, indent=2)
        out("guest pid %d control port %d" % (session["guest"]["pid"], cp["guest"]))
        guest = Peer("guest", cp["guest"])
        host.wait_ready(a.timeout)
        guest.wait_ready(a.timeout)
    except LiveError as e:
        out("RESULT: FAIL up: %s (logs in %s)" % (e, d))
        return 1
    out("RESULT: PASS up: both ready after %.0f s; session %s" % (time.monotonic() - t, sp))
    return 0


def _stop(role, info, out):
    p = Peer(role, info["control"])
    if p.up():
        p.quit()
        end = time.monotonic() + 15
        while p.up() and time.monotonic() < end:
            time.sleep(0.3)
    if p.up() and info.get("pid"):
        out("%s did not quit; stopping pid %s" % (role, info["pid"]))
        try:
            os.kill(info["pid"], signal.SIGTERM)  # a pid `up` recorded; on Windows this ends the process
        except OSError:
            pass
        end = time.monotonic() + 5
        while p.up() and time.monotonic() < end:
            time.sleep(0.3)
    return not p.up()


def cmd_down(a, out=print):
    sp = session_path(a.session)
    s = load_session(sp)
    if not s:
        out("no live session at %s" % sp)
        return 0
    ok = True
    for role in ("guest", "host"):
        if role in s:
            good = _stop(role, s[role], out)
            ok = ok and good
            out("%s %s" % (role, "stopped" if good else "STILL RUNNING"))
    out("RESULT: %s down" % ("PASS" if ok else "FAIL"))
    return 0 if ok else 1


def reset_guest(session_file, out=print, timeout=180):
    """Reconnect the guest: quit it and start the same command again. The host keeps its state."""
    s = load_session(session_path(session_file))
    if not s or not s.get("guest", {}).get("cmd"):
        raise LiveError("no session file with the guest's command; reset needs a viewer started by `live.py up`")
    if not _stop("guest", s["guest"], out):
        raise LiveError("the guest did not quit")
    s["guest"]["pid"] = spawn(s["guest"]["cmd"], s["guest"]["stdout"])
    with open(session_path(session_file), "w", encoding="utf-8") as f:
        json.dump(s, f, indent=2)
    Peer("guest", s["guest"]["control"]).wait_ready(timeout)
    return s


# ---- watch ---------------------------------------------------------------------------------------


def watch(path, host, guest, interval=0.3, once=False, reset=None, out=print, sleep=time.sleep, stop=lambda: False):
    """Replays the scenario after every save. `reset` is a callable run before each replay
    (reconnecting the guest) or None. Returns the last result."""
    last = None
    result = None
    while not stop():
        try:
            m = os.stat(path).st_mtime_ns
        except OSError:
            m = None
        if m is not None and m != last:
            if last is not None:
                sleep(0.2)  # editors write in two steps
            last = m
            try:
                scn = scn_mod.load(path)
            except (scn_mod.ScenarioError, OSError, ValueError) as e:
                out("SCENARIO ERROR %s" % e)
                if once:
                    return None
                continue
            out("--- %s %s" % (time.strftime("%H:%M:%S"), scn["name"]))
            if reset:
                try:
                    reset()
                except LiveError as e:
                    out("ERROR reset: %s" % e)
            result = replay(scn, host, guest, log=out)
            report(scn, result, out)
            if once:
                return result
        sleep(interval)
    return result


# ---- CLI -----------------------------------------------------------------------------------------


def _print_state(reply, a):
    if a.path:
        cur = split_state(reply)
        v = scn_mod.get_path(cur, a.path)
        print("<missing>" if v is scn_mod._MISSING else json.dumps(v, sort_keys=True))
    elif a.json:
        print(json.dumps(reply))
    else:
        print("%s ready=%s cell=%s fps=%.0f npcs=%d(%d visible) puppets=%d(%d visible) hash=%s" % (
            reply.get("role"), reply.get("ready"), reply.get("cell"), reply.get("fps", 0), reply.get("npcs", 0),
            reply.get("npcs_visible", 0), reply.get("puppets", 0), reply.get("puppets_visible", 0),
            (reply.get("state") or {}).get("hash", {}).get("shared")))


def build_parser():
    ap = argparse.ArgumentParser(description="live control of running nv-rs viewers")
    ap.add_argument("--session", help="session file (default: <out_dir>/live/session.json)")
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("up", help="start a host and a guest viewer with control ports")
    p.add_argument("--cell", required=True)
    p.add_argument("--config")
    p.add_argument("--visible", action="store_true", help="open the windows normally")
    p.add_argument("--background", action="store_true", help="open them behind other windows (config default)")
    p.add_argument("--new-game", action="store_true")
    p.add_argument("--control-base", type=int, default=DEFAULT_CONTROL_BASE)
    p.add_argument("--timeout", type=float, default=240, help="seconds to wait for both to be ready")
    p.add_argument("--extra", nargs="*", default=[], help="more viewer arguments for both")
    sub.add_parser("down", help="quit both viewers")
    sub.add_parser("reset", help="reconnect the guest (quit and start it again)")
    p = sub.add_parser("state")
    p.add_argument("target")
    p.add_argument("--json", action="store_true")
    p.add_argument("--path", help="print one value of the state, e.g. items.player or live.fps")
    p = sub.add_parser("run")
    p.add_argument("target")
    p.add_argument("line")
    p.add_argument("--local", action="store_true")
    p = sub.add_parser("bot")
    p.add_argument("target")
    p.add_argument("line")
    p = sub.add_parser("keys")
    p.add_argument("target")
    p.add_argument("key")
    p.add_argument("--hold", type=float, default=0.1)
    p.add_argument("--delay", type=float, default=0.0)
    p = sub.add_parser("shot")
    p.add_argument("target")
    p.add_argument("out")
    p = sub.add_parser("entities")
    p.add_argument("target")
    p.add_argument("--visible", action="store_true")
    p.add_argument("--json", action="store_true")
    p = sub.add_parser("watch", help="replay a scenario against the running pair after every save")
    p.add_argument("scenario")
    p.add_argument("--once", action="store_true", help="replay now and exit")
    p.add_argument("--reset", choices=("none", "guest"), default="none",
                   help="reconnect the guest before each replay (slow: it reloads the place)")
    p.add_argument("--host")
    p.add_argument("--guest")
    return ap


def main(argv=None, out=print):
    a = build_parser().parse_args(argv)
    try:
        if a.cmd == "up":
            return cmd_up(a, out)
        if a.cmd == "down":
            return cmd_down(a, out)
        if a.cmd == "reset":
            reset_guest(a.session, out)
            out("RESULT: PASS guest reconnected")
            return 0
        if a.cmd == "watch":
            host = peer_from(a.host or "host", a.session)
            guest = peer_from(a.guest or "guest", a.session)
            r = watch(a.scenario, host, guest, once=a.once, reset=(lambda: reset_guest(a.session, out)) if a.reset == "guest" else None, out=out)
            return 0 if (r and r["passed"]) or not a.once else 1
        peer = peer_from(a.target, a.session)
        if a.cmd == "state":
            _print_state(peer.state(), a)
        elif a.cmd == "run":
            print(json.dumps(peer.run(a.line, a.local)))
        elif a.cmd == "bot":
            print(json.dumps(peer.bot(a.line)))
        elif a.cmd == "keys":
            print(json.dumps(peer.keys(a.key, a.hold, a.delay)))
        elif a.cmd == "shot":
            print(json.dumps(peer.shot(a.out)))
        elif a.cmd == "entities":
            r = peer.entities(True if a.visible else None)
            if a.json:
                print(json.dumps(r))
            else:
                for e in r["entities"]:
                    print("%s %-24s %-6s %s pos=%s %s" % (e["form"], e["name"], e["kind"], "VISIBLE" if e["visible"] else "hidden",
                                                         e["pos"], e["anim_names"]))
                print("%d npcs (%d visible), %d puppets (%d visible)" % (r["npcs"], r["npcs_visible"], r["puppets"], r["puppets_visible"]))
        return 0
    except LiveError as e:
        print("RESULT: FAIL %s" % e)
        return 2 if any(w in str(e).lower() for w in ("connection", "refused", "no live session")) else 1
    except KeyboardInterrupt:
        return 0
    except Exception as e:  # config problems from runner
        if type(e).__name__ == "ConfigError":
            print("RESULT: FAIL config: %s" % e)
            return 2
        raise


if __name__ == "__main__":
    sys.exit(main())
