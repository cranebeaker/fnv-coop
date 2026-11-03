#!/usr/bin/env python3
"""Seeded UDP chaos relay for two-process co-op runs.

Sits between a guest (client) and a host (target) on loopback:

    guest viewer --> relay LISTEN port --> host viewer --coop-host PORT
    guest viewer <-- relay LISTEN port <-- host viewer

The named profiles carry the same names and parameters as coop-core's transport profiles
(crates/coop-core/src/transport/profile.rs): perfect, good_wifi, bad_wifi, mobile,
congested, flaky, partition. The relay works on raw datagrams, so every datagram gets the
"unreliable" treatment (loss, duplicate, reorder, partition). The random stream is
Python's, not the Rust SeededRng, so a seed reproduces a run of THIS relay, not a
coop-core simulation. Same inputs and same seed give the same verdict sequence.

Usage:
    python -I chaos_relay.py --listen 27961 --target 127.0.0.1:27960 --profile bad_wifi \
        --seed 7 --stats relay.stats.json --stop-file relay.stop [--schedule sched.json]

Schedule file: a JSON list of actions, "at" in seconds after the relay is armed (the first
guest datagram):
    {"at": 20, "profile": "perfect"}                     switch profile (RNG state kept)
    {"at": 5, "outage": "both", "seconds": 3}            hard blackout; "both", "g2h" or "h2g"
"""
import argparse
import heapq
import json
import os
import random
import select
import socket
import sys
import time
from dataclasses import dataclass, field

DIRS = ("g2h", "h2g")


@dataclass(frozen=True)
class Profile:
    name: str
    latency_ms: int = 0
    jitter_ms: int = 0
    loss: float = 0.0
    dup: float = 0.0
    reorder: float = 0.0
    reorder_extra_ms: int = 0
    partition_period_ms: int = 0
    partition_len_ms: int = 0

    def partition_end(self, now_ms):
        """If the profile's periodic outage is active at now_ms, when it ends (else None)."""
        if self.partition_period_ms == 0 or self.partition_len_ms == 0:
            return None
        phase = int(now_ms) % self.partition_period_ms
        start = max(0, self.partition_period_ms - self.partition_len_ms)
        if phase >= start:
            return int(now_ms) - phase + self.partition_period_ms
        return None


# Same names and numbers as coop-core ChaosProfile (perfect plus the six).
PROFILES = {
    "perfect": Profile("perfect"),
    "good_wifi": Profile("good_wifi", 15, 5, 0.005),
    "bad_wifi": Profile("bad_wifi", 50, 40, 0.05, 0.01, 0.05, 80),
    "mobile": Profile("mobile", 90, 60, 0.03, 0.005, 0.03, 120),
    "congested": Profile("congested", 150, 100, 0.10, 0.02, 0.10, 200),
    "flaky": Profile("flaky", 40, 30, 0.25, 0.05, 0.15, 100),
    "partition": Profile("partition", 30, 10, 0.02, 0, 0, 0, 6000, 1500),
}
PROFILE_NAMES = tuple(n for n in PROFILES if n != "perfect")


def get_profile(name):
    if name not in PROFILES:
        raise KeyError("unknown profile %r; have %s" % (name, sorted(PROFILES)))
    return PROFILES[name]


@dataclass
class Verdict:
    delays_ms: list = field(default_factory=list)  # empty means dropped
    reason: str = ""
    reordered: bool = False
    duplicated: bool = False

    @property
    def dropped(self):
        return not self.delays_ms


class Impairment:
    """One direction of one link. Pure and seeded: no sockets, virtual time in ms."""

    def __init__(self, profile, seed, label):
        self.profile = profile
        self.rng = random.Random("%s:%s" % (seed, label))
        self.outage_until_ms = -1.0

    def set_profile(self, profile):
        self.profile = profile

    def outage(self, until_ms):
        self.outage_until_ms = max(self.outage_until_ms, until_ms)

    def decide(self, now_ms):
        p, rng = self.profile, self.rng
        if now_ms < self.outage_until_ms:
            return Verdict(reason="outage")
        if p.partition_end(now_ms) is not None:
            return Verdict(reason="partition")
        if rng.random() < p.loss:
            return Verdict(reason="loss")
        dup = rng.random() < p.dup
        copies = 2 if dup else 1
        delays, reordered = [], False
        for _ in range(copies):
            d = p.latency_ms + rng.randint(0, p.jitter_ms)
            if rng.random() < p.reorder:
                d += p.reorder_extra_ms
                reordered = True
            delays.append(float(d))
        return Verdict(delays_ms=delays, reordered=reordered, duplicated=dup)


def new_dir_stats():
    # chaos_in counts the datagrams that met the starting profile (before a schedule change)
    return {"in": 0, "chaos_in": 0, "bytes_in": 0, "forwarded": 0, "dropped": 0, "drop_reasons": {},
            "duplicated": 0, "reordered": 0}


class ChaosRelay:
    def __init__(self, listen_port, target, profile="perfect", seed=1, schedule=None,
                 stats_path=None, log_path=None, stop_file=None, bind="127.0.0.1",
                 clock=time.monotonic, max_log=20000):
        self.listen_port = listen_port
        self.target = target
        self.seed = seed
        self.profile = get_profile(profile)
        self.profile_name = profile
        self.schedule = sorted(schedule or [], key=lambda a: a["at"])
        self.stats_path = stats_path
        self.log_path = log_path
        self.stop_file = stop_file
        self.bind = bind
        self.clock = clock
        self.imp = {d: Impairment(self.profile, seed, d) for d in DIRS}
        self.stats = {"profile": profile, "seed": seed, "armed": False,
                      "g2h": new_dir_stats(), "h2g": new_dir_stats(), "events": []}
        self.t0 = None
        self.listen = None
        self.upstreams = {}  # client addr -> socket
        self.by_sock = {}    # socket -> client addr
        self.heap = []
        self.seq = 0
        self.max_log = max_log
        self.log_n = 0
        self.log_f = None
        self._next_stats = 0.0
        self.sched_i = 0

    def now_ms(self):
        """Milliseconds since arming (0 before the first guest datagram)."""
        if self.t0 is None:
            return 0.0
        return (self.clock() - self.t0) * 1000.0

    def arm(self):
        if self.t0 is None:
            self.t0 = self.clock()
            self.stats["armed"] = True
            self.stats["events"].append({"t": 0.0, "ev": "armed"})

    def _apply_schedule(self):
        if self.t0 is None:
            return
        now = self.now_ms()
        while self.sched_i < len(self.schedule) and self.schedule[self.sched_i]["at"] * 1000.0 <= now:
            a = self.schedule[self.sched_i]
            self.sched_i += 1
            if "profile" in a:
                self.profile = get_profile(a["profile"])
                self.profile_name = a["profile"]
                for imp in self.imp.values():
                    imp.set_profile(self.profile)
                self.stats["events"].append({"t": round(now / 1000, 3), "ev": "profile", "name": a["profile"]})
            if "outage" in a:
                until = a["at"] * 1000.0 + a["seconds"] * 1000.0
                which = DIRS if a["outage"] == "both" else (a["outage"],)
                for d in which:
                    self.imp[d].outage(until)
                self.stats["events"].append({"t": round(now / 1000, 3), "ev": "outage",
                                             "dir": a["outage"], "seconds": a["seconds"]})

    def _log(self, direction, size, v):
        if not self.log_path or self.log_n >= self.max_log:
            return
        if self.log_f is None:
            self.log_f = open(self.log_path, "w", encoding="utf-8")
        self.log_n += 1
        verdict = ("drop:" + v.reason) if v.dropped else "fwd" + ("x%d" % len(v.delays_ms) if len(v.delays_ms) > 1 else "")
        self.log_f.write(json.dumps({"t": round(self.now_ms(), 1), "dir": direction, "n": size, "v": verdict}) + "\n")

    def handle(self, direction, data, client):
        """Apply the impairment to one datagram and queue it. Returns the Verdict."""
        if direction == "g2h":
            self.arm()
        self._apply_schedule()
        st = self.stats[direction]
        st["in"] += 1
        if self.profile_name == self.stats["profile"]:
            st["chaos_in"] += 1
        st["bytes_in"] += len(data)
        v = self.imp[direction].decide(self.now_ms())
        if v.dropped:
            st["dropped"] += 1
            st["drop_reasons"][v.reason] = st["drop_reasons"].get(v.reason, 0) + 1
        else:
            st["forwarded"] += len(v.delays_ms)
            if v.duplicated:
                st["duplicated"] += 1
            if v.reordered:
                st["reordered"] += 1
            for d in v.delays_ms:
                self.seq += 1
                heapq.heappush(self.heap, (self.clock() + d / 1000.0, self.seq, direction, data, client))
        self._log(direction, len(data), v)
        return v

    def _upstream_for(self, client):
        s = self.upstreams.get(client)
        if s is None:
            s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
            s.bind((self.bind, 0))
            s.setblocking(False)
            self.upstreams[client] = s
            self.by_sock[s] = client
        return s

    def open(self):
        self.listen = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.listen.bind((self.bind, self.listen_port))
        self.listen.setblocking(False)
        self.listen_port = self.listen.getsockname()[1]
        return self.listen_port

    def _release_due(self):
        now = self.clock()
        while self.heap and self.heap[0][0] <= now:
            _, _, direction, data, client = heapq.heappop(self.heap)
            try:
                if direction == "g2h":
                    self._upstream_for(client).sendto(data, self.target)
                else:
                    self.listen.sendto(data, client)
            except OSError:
                pass

    def write_stats(self):
        if not self.stats_path:
            return
        out = dict(self.stats)
        out["clients"] = len(self.upstreams)
        out["profile"] = self.profile_name
        tmp = self.stats_path + ".tmp"
        with open(tmp, "w", encoding="utf-8") as f:
            json.dump(out, f)
        os.replace(tmp, self.stats_path)

    def step(self, timeout=0.01):
        """One loop turn: receive, release due datagrams, refresh stats."""
        socks = [self.listen] + list(self.by_sock)
        wait = timeout
        if self.heap:
            wait = max(0.0, min(timeout, self.heap[0][0] - self.clock()))
        try:
            ready, _, _ = select.select(socks, [], [], wait)
        except (OSError, ValueError):
            ready = []
        for s in ready:
            for _ in range(64):
                try:
                    data, addr = s.recvfrom(65535)
                except BlockingIOError:
                    break
                except OSError:
                    break  # Windows reports ICMP port-unreachable as a reset; ignore
                if s is self.listen:
                    self.handle("g2h", data, addr)
                else:
                    self.handle("h2g", data, self.by_sock[s])
        self._apply_schedule()
        self._release_due()
        now = self.clock()
        if now >= self._next_stats:
            self._next_stats = now + 0.5
            self.write_stats()

    def run(self, duration=None, stop_event=None):
        if self.listen is None:
            self.open()
        end = self.clock() + duration if duration else None
        while True:
            self.step()
            if stop_event is not None and stop_event.is_set():
                break
            if self.stop_file and os.path.exists(self.stop_file):
                break
            if end is not None and self.clock() >= end:
                break
        self.close()

    def close(self):
        self.write_stats()
        if self.log_f:
            self.log_f.close()
            self.log_f = None
        for s in list(self.by_sock) + [self.listen]:
            try:
                s.close()
            except Exception:
                pass


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--listen", type=int, required=True, help="UDP port the guest joins")
    ap.add_argument("--target", required=True, help="HOST:PORT of the host viewer")
    ap.add_argument("--profile", default="perfect", choices=sorted(PROFILES))
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--schedule", help="JSON file with timed actions")
    ap.add_argument("--stats", help="write stats JSON here (every 0.5 s and on exit)")
    ap.add_argument("--log", help="write a per-datagram JSONL log here (capped)")
    ap.add_argument("--stop-file", help="exit cleanly when this file appears")
    ap.add_argument("--duration", type=float, help="exit after this many seconds")
    ap.add_argument("--ready-file", help="write the bound port here once listening")
    a = ap.parse_args(argv)
    host, _, port = a.target.rpartition(":")
    sched = None
    if a.schedule:
        with open(a.schedule, encoding="utf-8") as f:
            sched = json.load(f)
    r = ChaosRelay(a.listen, (host or "127.0.0.1", int(port)), a.profile, a.seed, sched,
                   a.stats, a.log, a.stop_file)
    bound = r.open()
    if a.ready_file:
        with open(a.ready_file, "w", encoding="utf-8") as f:
            f.write(str(bound))
    print("chaos relay: listen %d -> %s profile=%s seed=%d" % (bound, a.target, a.profile, a.seed), flush=True)
    r.run(a.duration)
    return 0


if __name__ == "__main__":
    sys.exit(main())
