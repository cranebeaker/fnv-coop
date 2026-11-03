#!/usr/bin/env python3
"""A stand-in for nv-viewer that honours the same command line and the co-op log contract.

It exists so runner.py and the relay are tested without the game. It is strict about flags
(unknown flag or wrong argument count exits 2, like a bad viewer command line would), and
it speaks a tiny UDP sync protocol so chaos profiles really do hit it:

  host:  authoritative shared state (quests, globals, items). Applies its own script lines
         and the guest's "intents" (the guest's script lines), each intent once by id.
  guest: sends its intents (resent until acknowledged) and adopts the host's shared state.

Test modes, selected with --fake MODE (never passed by the real runner config):
  crash        exit 3 two seconds after ready, no shutdown event
  hang         ignore quit requests (the runner must time out and stop it)
  no_sync      guest never adopts the host state (a desync)
  double_apply host applies each guest intent twice (items counted double)
  silent       write no log and no state dump
  no_connect   guest never sends hello (never joins)
  error_event  log an error event
  repair       log three coop_repair events
  bad_exit     shut down cleanly but exit code 5

With --coop-control PORT it also speaks the live control protocol (viewer/src/coop_control.rs):
JSON lines on 127.0.0.1, requests handled on the main loop like the real viewer's frame.
Extra modes for the control port: ctl_not_ready (state says ready:false), ctl_slow (state takes 2 s),
ctl_hidden (the other player's puppet is not visible).
"""
import hashlib
import json
import os
import queue
import re
import socket
import sys
import threading
import time

FLAGS0 = {"--official", "--keep-root-transforms", "--walk", "--fps", "--talk", "--new-game", "--movies",
          "--no-movies", "--no-hud", "--answer-boxes", "--background", "--freeze-ai", "--pad", "--coop-test-token", "--coop-no-chargen"}
FLAGS1 = {"--plugins", "--ini", "--brightness", "--at", "--screenshot", "--wait", "--screen-size", "--choose",
          "--load-fos", "--character", "--weapon", "--weather-region", "--run", "--say", "--pipboy",
          "--pipboy-keys", "--lockpick", "--cloud-time", "--open-menu", "--use", "--menu-pointer",
          "--menu-click", "--menu-keys",
          "--coop-host", "--coop-join", "--coop-log", "--coop-state-dump", "--coop-quit-file",
          "--coop-quit-after", "--coop-control", "--fake", "--coop-name", "--coop-go-file"}
SPAWN = [2352.0, 1603.6, 7360.0, 0.0]
WALK_SPEED = 150.0  # units a second in the fake; w/s move y, a/d move x
FLAGS2 = {"--stage", "--run-at", "--key-at"}


class Usage(Exception):
    pass


def parse(argv):
    o = {"pos": [], "run": [], "run_at": [], "stage": [], "say": [], "choose": [], "talk": False,
         "new_game": False}
    i = 0
    while i < len(argv):
        a = argv[i]
        if a.startswith("--"):
            if a in FLAGS0:
                if a == "--talk":
                    o["talk"] = True
                if a == "--new-game":
                    o["new_game"] = True
                if a == "--coop-test-token":
                    o["coop_test_token"] = True
                i += 1
            elif a in FLAGS1:
                if i + 1 >= len(argv):
                    raise Usage("%s needs a value" % a)
                v = argv[i + 1]
                if a == "--run":
                    o["run"].append(v)
                elif a == "--say":
                    o["say"].append(v)
                elif a == "--choose":
                    o["choose"] = [int(x) for x in v.split(",")]
                else:
                    o[a[2:].replace("-", "_")] = v
                i += 2
            elif a in FLAGS2:
                if i + 2 >= len(argv):
                    raise Usage("%s needs two values" % a)
                if a == "--run-at":
                    o["run_at"].append((float(argv[i + 1]), argv[i + 2]))
                elif a == "--key-at":
                    o.setdefault("key_at", []).append((float(argv[i + 1]), argv[i + 2]))
                elif a == "--stage":
                    o["stage"].append((argv[i + 1], int(argv[i + 2])))
                i += 3
            else:
                raise Usage("unknown option %s" % a)
        else:
            o["pos"].append(a)
            i += 1
    if not o["pos"]:
        raise Usage("DATA FOLDER missing")
    if len(o["pos"]) > 2:
        raise Usage("too many positional arguments")
    if len(o["pos"]) == 1 and not o["new_game"]:
        raise Usage("CELL missing (only --new-game may leave it out)")
    if bool(o.get("coop_host")) == bool(o.get("coop_join")):
        raise Usage("exactly one of --coop-host / --coop-join")
    if o.get("coop_join") and not o.get("coop_test_token") and not o["coop_join"].upper().startswith("NVRS-"):
        raise Usage("a plain HOST:PORT needs --coop-test-token (or use an NVRS- join code)")
    if not o.get("coop_log") or not o.get("coop_state_dump"):
        raise Usage("--coop-log and --coop-state-dump are required")
    return o


class Fake:
    def __init__(self, o):
        self.o = o
        self.role = "host" if o.get("coop_host") else "guest"
        self.mode = o.get("fake", "")
        self.t0 = time.monotonic()
        self.shared = {"quests": {}, "globals": {}, "items": {"host": {}, "guest": {}}}
        # both avatars start at the same spawn point, like the real viewer in Doc Mitchell's house
        self.pos = {"host": list(SPAWN), "guest": list(SPAWN)}
        self.intents = []          # guest: (id, cmd)
        self.applied = set()       # host: intent ids
        self.acked = set()
        self.hseq = 0
        self.last_hseq = -1
        self.ready = False
        self.ready_at = None
        self.dump_seq = 0
        self.log = None if self.mode == "silent" else open(o["coop_log"], "w", encoding="utf-8")
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.peer = None
        self.pending_script = sorted(o["run_at"])
        self.pending_keys = sorted(o.get("key_at", []))
        self.next_id = 1
        self.ctl = queue.Queue()
        self.ctl_quit = False

    # ---- live control port (the real protocol: crates/coop-session/src/control.rs) ----
    def ctl_start(self, port):
        srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        srv.bind(("127.0.0.1", port))
        srv.listen(8)

        def serve_conn(c):
            f = c.makefile("rwb")
            for raw in f:
                raw = raw.strip()
                if not raw:
                    continue
                try:
                    req = json.loads(raw.decode("utf-8"))
                    if not isinstance(req, dict) or any(isinstance(v, (dict, list)) for v in req.values()):
                        raise ValueError("expected a flat JSON object")
                except ValueError as e:
                    f.write((json.dumps({"ok": False, "cmd": "error", "error": str(e)}) + "\n").encode())
                    f.flush()
                    continue
                box = queue.Queue()
                self.ctl.put((req, box))
                try:
                    rep = box.get(timeout=30)
                except queue.Empty:
                    rep = {"ok": False, "cmd": req.get("cmd"), "error": "no answer from the viewer"}
                if "id" in req:
                    rep = dict({"id": req["id"]}, **rep)
                f.write((json.dumps(rep) + "\n").encode())
                f.flush()
            c.close()

        def accept():
            while True:
                try:
                    c, _ = srv.accept()
                except OSError:
                    return
                threading.Thread(target=serve_conn, args=(c,), daemon=True).start()

        threading.Thread(target=accept, daemon=True).start()

    def ctl_entities(self):
        rows = [{"form": "000104C0", "name": "DocMitchellREF", "kind": "npc", "player": None, "pos": [2400.0, 1700.0, 7360.0],
                 "heading": 0.0, "visible": True, "anim": 0, "anim_names": "idle"}]
        if self.ready:
            other = "guest" if self.role == "host" else "host"
            rows.append({"form": "FF000001", "name": other, "kind": "puppet", "player": 1 if other == "guest" else 0,
                         "pos": self.pos[other][:3], "heading": 0.0, "visible": self.mode != "ctl_hidden", "anim": 0,
                         "anim_names": "idle"})
        return rows

    def ctl_counts(self, rows):
        n = [r for r in rows if r["kind"] != "puppet"]
        p = [r for r in rows if r["kind"] == "puppet"]
        return {"npcs": len(n), "npcs_visible": sum(r["visible"] for r in n), "puppets": len(p),
                "puppets_visible": sum(r["visible"] for r in p)}

    def ctl_handle(self, req):
        cmd = req.get("cmd")
        ok = {"ok": True, "cmd": cmd}
        if cmd == "state":
            if self.mode == "ctl_slow":
                time.sleep(2)
            rows = self.ctl_entities()
            r = dict(ok, role=self.role, ready=self.ready and self.mode != "ctl_not_ready", cell=self.o["pos"][-1], fps=60.0,
                     t=round(time.monotonic() - self.t0, 2), counters={"lines_played": 0, "chat_in": 0, "travels": 0, "hits_sent": 0},
                     state=self.state())
            r.update(self.ctl_counts(rows))
            return r
        if cmd == "entities":
            rows = self.ctl_entities()
            return dict(ok, count=len(rows), entities=rows, **self.ctl_counts(rows))
        if cmd in ("run", "bot"):
            line = req.get("line")
            if not line:
                return {"ok": False, "cmd": cmd, "error": 'missing "line"'}
            if cmd == "bot":
                line = line if line.lower().startswith("coop ") else "coop " + line
                self.run_cmd(line)
                return dict(ok, how="bot")
            if line.startswith("local ") or req.get("local") in (True, "true"):
                self.run_cmd(line[6:] if line.startswith("local ") else line, local=True)
                return dict(ok, how="local", flow="Done")
            self.run_cmd(line)
            if self.role == "host":
                return dict(ok, how="host", flow="Done")
            return dict(ok, how="sent")
        if cmd == "keys":
            key = req.get("key")
            if not key:
                return {"ok": False, "cmd": cmd, "error": 'missing "key"'}
            el = (time.monotonic() - self.ready_at) if self.ready_at else 0.0
            self.pending_keys.append((el + float(req.get("delay", 0)), "%s:%s" % (key, req.get("hold", 0.1))))
            self.pending_keys.sort()
            return dict(ok, key=key, hold=req.get("hold", 0.1), delay=req.get("delay", 0))
        if cmd == "screenshot":
            path = req.get("path")
            if not path:
                return {"ok": False, "cmd": cmd, "error": 'missing "path"'}
            os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
            with open(path, "wb") as fh:
                fh.write(b"\x89PNG\r\n\x1a\n" + b"fake")
            return dict(ok, path=path)
        if cmd == "quit":
            self.ctl_quit = True
            return ok
        return {"ok": False, "cmd": cmd, "error": 'unknown cmd "%s"' % cmd}

    def ev(self, name, **kw):
        if not self.log:
            return
        rec = {"t": round(time.monotonic() - self.t0, 3), "ev": name}
        rec.update(kw)
        self.log.write(json.dumps(rec) + "\n")
        self.log.flush()

    # ---- game commands ----
    def run_cmd(self, cmd, via_intent=False, local=False):
        """Apply one script line. On a guest, shared-state lines become intents."""
        self.ev("script", line=cmd)
        if cmd.startswith("local "):
            cmd = cmd[6:]
        tp = re.match(r"(?i)^\s*coop\s+teleport\s+(-?[\d.]+)\s+(-?[\d.]+)\s+(-?[\d.]+)\s*$", cmd)
        if tp:
            self.pos[self.role][:3] = [float(tp.group(1)), float(tp.group(2)), float(tp.group(3))]
            return
        m = re.match(r"(?i)^\s*setstage\s+(\S+)\s+(\d+)\s*$", cmd)
        g = re.match(r"(?i)^\s*set\s+(\S+)\s+to\s+(\S+)\s*$", cmd)
        a = re.match(r"(?i)^\s*player\.additem\s+(\S+)\s+(\d+)\s*$", cmd)
        p = re.match(r"(?i)^\s*player\.setpos\s+([xyz])\s+(-?[\d.]+)\s*$", cmd)
        if p:
            # like the real viewer: SetPos edits the script state only, the physical player stays
            self.ev("setpos_ignored", line=cmd)
            return
        if not (m or g or a):
            self.ev("script_unknown", line=cmd)
            return
        if self.role == "guest" and not via_intent and not local:
            self.intents.append((self.next_id, cmd))
            self.next_id += 1
            return
        who = "guest" if via_intent else self.role
        if m:
            q = m.group(1)
            cur = self.shared["quests"].setdefault(q, {"stage": 0})
            cur["stage"] = max(cur["stage"], int(m.group(2)))
        elif g:
            self.shared["globals"][g.group(1)] = float(g.group(2))
        elif a:
            it = self.shared["items"][who]
            it[a.group(1)] = it.get(a.group(1), 0) + int(a.group(2))

    # ---- state dump ----
    def state(self):
        h = hashlib.sha1(json.dumps(self.shared, sort_keys=True).encode()).hexdigest()[:16]
        return {"schema": 1, "role": self.role, "t": round(time.monotonic() - self.t0, 3), "seq": self.dump_seq,
                "quests": self.shared["quests"], "globals": self.shared["globals"],
                "items": self.shared["items"], "positions": self.pos, "hash": {"shared": h}}

    def dump(self):
        if self.mode == "silent":
            return
        self.dump_seq += 1
        tmp = self.o["coop_state_dump"] + ".tmp"
        with open(tmp, "w", encoding="utf-8") as f:
            json.dump(self.state(), f)
        os.replace(tmp, self.o["coop_state_dump"])
        self.ev("state_dump", path=self.o["coop_state_dump"], seq=self.dump_seq)

    # ---- network ----
    def send(self, obj):
        if self.peer:
            try:
                self.sock.sendto(json.dumps(obj).encode(), self.peer)
            except OSError:
                pass

    def pump(self):
        while True:
            try:
                data, addr = self.sock.recvfrom(65535)
            except (BlockingIOError, InterruptedError):
                return
            except OSError:
                return
            try:
                m = json.loads(data.decode())
            except ValueError:
                continue
            self.on_msg(m, addr)

    def on_msg(self, m, addr):
        k = m.get("k")
        if self.role == "host":
            if k == "hello":
                self.peer = addr
                if not self.ready:
                    self.ev("coop_connected", role="host", peer=str(addr[1]), peers=1)
                    self.set_ready()
            elif k == "sync" and self.peer:
                self.pos["guest"] = m["pos"]
                self.acked_by_guest = m["intents"]
                for iid, cmd in m["intents"]:
                    if iid in self.applied:
                        continue
                    self.applied.add(iid)
                    self.run_cmd(cmd, via_intent=True)
                    if self.mode == "double_apply":
                        self.run_cmd(cmd, via_intent=True)
        else:
            if k == "state":
                if not self.ready:
                    self.ev("coop_connected", role="guest", peer="host", peers=1)
                    self.set_ready()
                if m["seq"] > self.last_hseq:
                    self.last_hseq = m["seq"]
                    self.pos["host"] = m["pos"]
                    self.acked = set(m["applied"])
                    self.intents = [(i, c) for (i, c) in self.intents if i not in self.acked]
                    if self.mode != "no_sync":
                        self.shared = m["shared"]

    def set_ready(self):
        self.ready = True
        self.ready_at = time.monotonic()
        self.ev("coop_ready", role=self.role, peers=1)
        for q, n in self.o["stage"]:
            self.run_cmd("SetStage %s %d" % (q, n))
        for c in self.o["run"]:
            self.run_cmd(c)
        if self.mode == "repair":
            for _ in range(3):
                self.ev("coop_repair", kind="hash", subsystem="quests")

    def tick_send(self):
        if self.role == "host":
            self.hseq += 1
            self.send({"k": "state", "seq": self.hseq, "pos": self.pos["host"], "shared": self.shared,
                       "applied": sorted(self.applied)})
        else:
            if self.mode == "no_connect":
                return
            if not self.peer:
                self.send({"k": "hello"})
            else:
                self.send({"k": "sync", "pos": self.pos["guest"], "intents": self.intents})

    def main(self):
        o = self.o
        self.ev("start", role=self.role, pid=os.getpid(), fake=True, cell=o["pos"][1] if len(o["pos"]) > 1 else "")
        self.ev("loaded", cell=o["pos"][1] if len(o["pos"]) > 1 else "")
        self.sock.setblocking(False)
        if self.role == "host":
            self.sock.bind(("127.0.0.1", int(o["coop_host"])))
            self.ev("coop_listening", port=int(o["coop_host"]))
        else:
            host, _, port = o["coop_join"].rpartition(":")
            self.peer = (host or "127.0.0.1", int(port))
            self.sock.bind(("127.0.0.1", 0))
        if self.mode == "error_event":
            self.ev("error", msg="fake error event")
        if o.get("coop_control"):
            self.ctl_start(int(o["coop_control"]))
        quit_after = float(o.get("coop_quit_after", 0)) or None
        next_send = next_dump = 0.0
        reason = None
        while reason is None:
            now = time.monotonic()
            self.pump()
            while True:
                try:
                    req, box = self.ctl.get_nowait()
                except queue.Empty:
                    break
                try:
                    box.put(self.ctl_handle(req))
                except Exception as e:  # a bad request must not kill the fake
                    box.put({"ok": False, "cmd": req.get("cmd"), "error": str(e)})
            if self.ctl_quit:
                reason = "control"
                time.sleep(0.3)  # let the reply out, as the real viewer waits a few frames
            if now >= next_send:
                next_send = now + 0.1
                if self.role == "guest" and not self.ready and self.peer and self.mode != "no_connect":
                    # guest says hello until a state message arrives
                    self.send({"k": "hello"})
                    self.send({"k": "sync", "pos": self.pos["guest"], "intents": self.intents})
                elif self.ready or self.role == "host":
                    if self.role == "host" and not self.peer:
                        pass
                    else:
                        self.tick_send()
            if self.ready:
                el = now - self.ready_at
                while self.pending_script and self.pending_script[0][0] <= el:
                    _, cmd = self.pending_script.pop(0)
                    self.run_cmd(cmd)
                while self.pending_keys and self.pending_keys[0][0] <= el:
                    _, key = self.pending_keys.pop(0)
                    name, _, hold = key.partition(":")
                    d = WALK_SPEED * (float(hold) if hold else 0.1)
                    dx, dy = {"w": (0, d), "s": (0, -d), "a": (-d, 0), "d": (d, 0)}.get(name.lower(), (0, 0))
                    self.pos[self.role][0] += dx
                    self.pos[self.role][1] += dy
                    self.ev("key", key=key)
                if self.mode == "crash" and el > 2.0:
                    sys.exit(3)
                if self.mode != "hang":
                    if quit_after and el >= quit_after:
                        reason = "quit_after"
                    qf = o.get("coop_quit_file")
                    if qf and os.path.exists(qf):
                        reason = "quit_file"
            if not self.ready and self.mode != "hang":
                qf = o.get("coop_quit_file")
                if qf and os.path.exists(qf):
                    reason = "quit_file"
            if now >= next_dump:
                next_dump = now + 1.0
                self.dump()
            time.sleep(0.01)
        self.dump()
        self.ev("shutdown", reason=reason, clean=True, code=5 if self.mode == "bad_exit" else 0)
        return 5 if self.mode == "bad_exit" else 0


def main(argv=None):
    argv = sys.argv[1:] if argv is None else argv
    try:
        o = parse(argv)
    except (Usage, ValueError) as e:
        print("fake_viewer: %s" % e, file=sys.stderr)
        return 2
    return Fake(o).main()


if __name__ == "__main__":
    sys.exit(main())
