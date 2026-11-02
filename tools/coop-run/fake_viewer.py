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
"""
import hashlib
import json
import os
import re
import socket
import sys
import time

FLAGS0 = {"--official", "--keep-root-transforms", "--walk", "--fps", "--talk", "--new-game", "--movies",
          "--no-movies", "--no-hud", "--answer-boxes", "--background", "--freeze-ai", "--pad", "--coop-test-token"}
FLAGS1 = {"--plugins", "--ini", "--brightness", "--at", "--screenshot", "--wait", "--screen-size", "--choose",
          "--load-fos", "--character", "--weapon", "--weather-region", "--run", "--say", "--pipboy",
          "--pipboy-keys", "--lockpick", "--cloud-time", "--open-menu", "--use", "--menu-pointer",
          "--menu-click", "--menu-keys",
          "--coop-host", "--coop-join", "--coop-log", "--coop-state-dump", "--coop-quit-file",
          "--coop-quit-after", "--fake"}
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

    def ev(self, name, **kw):
        if not self.log:
            return
        rec = {"t": round(time.monotonic() - self.t0, 3), "ev": name}
        rec.update(kw)
        self.log.write(json.dumps(rec) + "\n")
        self.log.flush()

    # ---- game commands ----
    def run_cmd(self, cmd, via_intent=False):
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
        if self.role == "guest" and not via_intent:
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
        quit_after = float(o.get("coop_quit_after", 0)) or None
        next_send = next_dump = 0.0
        reason = None
        while reason is None:
            now = time.monotonic()
            self.pump()
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
