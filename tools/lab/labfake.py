"""A game-free two-copy world for the lab rooms: the fake machines, a virtual clock, and one deliberate bug per name (the mutation twins).

    python -I tools/lab/lab.py run --fake [--bug NAME]      # the whole lab on this world; with a bug the matching room must FAIL

Two sides (host and client) each hold their own view of the lab; what one does reaches the other as a message that arrives `latency` seconds
later (the host is authoritative: a pickup race is won by the host). A digest compare runs once a second when no message is in flight,
like the real one tolerates things in flight. Sleeping on the clock steps the world in 50 ms slices, so a whole lab run takes a second.
Layout positions are made up; the point is that the rooms ask the same questions of this world as of the bridge (lab_live.py).
"""
from __future__ import annotations

import heapq
import math
from typing import Any, Callable, Dict, List, Optional, Set, Tuple

import lab_net
from lab_core import CLIENT, HOST, Machine, dist2
from lab_layout import Layout

DT = 0.05
SPEED = 220.0
VATS_DAMAGE = 3.0       # a VATS pistol hit (the real one does about 9)
BUGS = {
    # spawn / connection
    "client_stays_out": "the client never follows into the lab cell",
    "not_joined": "the client never joins the session",
    "no_puppet": "the host copy has no puppet of the client",
    "digest_noise": "every kill, take and pickup raises the digest repair counter",
    "ref_id_differs": "the client resolves the first enemy to another id than the host",
    "global_missing": "the client lacks a global the host has (the digest never agrees and no repair fixes it; the first in-game run's SetGlobal 37)",
    # walk
    "puppet_lag": "puppets trail their player by about 2 s",
    "puppet_teleports": "a puppet jumps 600 units every few seconds",
    "puppet_offset": "the client shows the host's puppet 400 units off",
    # fight
    "enable_unsynced": "the client never sees the hostiles the host enabled",
    "weak_unsynced": "the client never sees the health the host set on a hostile (the owner's SetAV does not reach the other copy)",
    "hostiles_charge": "a hostile enabled more than 1 s before a shot has walked up to the shooter (aim_at: too_close; VATS: vats_not_entered); a restrained one stays where it is",
    "target_drifts": "a hostile that nobody restrains drifts 659 units away from the shooter (out of the pistol's range) a second after it was enabled",
    "host_kill_unseen": "the client never sees a death the host caused",
    "client_kill_unseen": "the host never sees a death the client caused",
    "slow_death": "a death reaches the other copy 6 s late",
    "corpse_revives": "a corpse on the client gets up 4 s after it died",
    "survivor_hurt": "a kill also hurts the survivor, on the client only",
    # loot
    "container_unsynced": "container changes never reach the other copy",
    "take_dupe": "a take is also kept in the container on the host copy",
    "dup_pickup": "the player who lost a pickup race keeps the item",
    "pickup_unsynced": "a pickup never reaches the other copy",
    # door
    "door_unsynced": "door open and close never reach the other copy",
    "lock_unsynced": "an unlock never reaches the other copy",
    "terminal_unsynced": "using the terminal only unlocks on the copy that used it",
    "terminal_noop": "the terminal menu takes the click but its script unlocks nothing, on either copy (measured on the second PC with the stock unlock terminal)",
    "locked_door_opens": "a locked door opens for the host anyway",
    "locked_try_opens_lockpick": "activating the locked door opens the lockpick menu on that copy and it stays open: it eats the input (open, close, VATS) until the room closes it (the first lab run's door and VATS rooms)",
    # npc
    "no_handoff": "NPC ownership never leaves the host, not even when the host has left the lab cell",
    "nearest_owner": "NPC ownership goes to the nearest player (the host loses it to a nearer client while it is still in the lab cell)",
    "owner_split": "the client never learns of an ownership change",
    "npc_pos_stale": "the non-owner copy never gets NPC positions",
    "handoff_snap": "the new owner puts the NPC 700 units off its place",
    "npc_static": "the NPC does not patrol",
    "slow_npc_samples": "reading the NPC's position takes 0.5 s (a busy PC, two copies read one after the other) and the NPC walks 300 units per second: a step between two samples is far over the snap limit and is no snap",
    # vats
    "vats_unsynced": "VATS damage never reaches the other copy",
    "vats_double": "VATS damage is applied twice on the other copy",
    # quest
    "stage_unsynced": "the client never sees a stage the host set",
    "stage_client_lost": "the host ignores a stage the client set",
    "var_unsynced": "the client never sees a quest variable",
    "stage_rollback": "the client's stage drops back 3 s after the host set a later one",
    "var_unreadable": "this build cannot read quest variables",
    # not a bug of the product: how the real game reports a kill
    "kill_health_split": "a VATS kill reads health -14 on the host (the owner publishes below 0) and 1 on the client (the actor port never writes under 1), dead on both",
    # network pass only (the clean pass is untouched)
    "net_slow": "behind a chaos profile every change takes 12 s to reach the other copy",
}
NET_ONLY = {"net_slow"}


class ConnectionLost(Exception):
    """Named like nvbridge's: the room runner classifies a lost game by this name."""


class VClock:
    virtual = True

    def __init__(self, world: "World"):
        self.world = world

    def now(self) -> float:
        return self.world.t

    def sleep(self, s: float) -> None:
        n = max(1, int(math.ceil(s / DT - 1e-9)))
        for _ in range(n):
            self.world.advance(DT)


class Side:
    def __init__(self, name: str, slot: int):
        self.name, self.slot = name, slot
        self.pos = [0.0, 0.0, 0.0]
        self.cell = 0
        self.items: Dict[str, int] = {}
        self.objs: Dict[str, Dict[str, Any]] = {}
        self.stages: Dict[str, int] = {}
        self.vars: Dict[str, float] = {}
        self.hist: List[Tuple[float, List[float]]] = []     # this player's real positions, for the other side's puppet
        self.armed = False
        self.owner = {"owner": 0, "epoch": 1}
        self.digest = {"ok": 0, "mismatch": 0, "repairs": 0}
        self.reports: List[Dict[str, Any]] = []
        self.picked: Set[str] = set()
        self.vats_shots = 0
        self.menu = ""                                      # a menu this copy has open and has not closed


class World:
    def __init__(self, layout: Layout, bugs=(), latency: float = 0.3):
        self.layout, self.bugs, self.latency = layout, set(bugs), latency
        unknown = self.bugs - set(BUGS)
        if unknown:
            raise ValueError(f"unknown fake bug(s) {sorted(unknown)}; the names are in labfake.BUGS")
        self.t = 0.0
        self.q: List[Tuple[float, int, str, Callable[[], None]]] = []
        self.n = 0
        self.dead: Set[str] = set()
        self.sides = {HOST: Side(HOST, 0), CLIENT: Side(CLIENT, 1)}
        self.refpos: Dict[str, Tuple[float, float, float]] = {}
        self.clock = VClock(self)
        dflt = layout.extra("loot", "floor_item") or "FalloutNV.esm:015169"
        floors = layout.refs("loot", "item", "loose", "floor")
        bases = layout.loose_bases() or [dflt] * len(floors)
        self.loose_base = dict(zip(floors, bases))
        self.report_id = 0
        self.last_digest = 0.0
        self.npc_target = 0
        self.npc_pending: Optional[Tuple[float, int]] = None
        self.npc_tick = 0
        self.truth = {"owner": 0, "epoch": 1}
        self.seq = 0
        self.enabled_at: Dict[str, float] = {}
        self.held: Set[str] = set()
        self.fps = {HOST: 60.0, CLIENT: 60.0}   # frames per second of each copy (the frame counter of coop.status; FakeEnv(fps=...) changes it)
        self.resend_every = 0         # a network pass: every Nth message is lost once and arrives `rto` late
        self.rto = 0.0
        self.profile = ""
        self._build()

    def has(self, bug: str) -> bool:
        return bug in self.bugs

    # ---- the lab
    def _build(self) -> None:
        L = self.layout
        for room, r in L.rooms.items():
            ox, oy, oz = (float(v) for v in r["origin"])
            for k, f in (r.get("markers") or {}).items():
                self.refpos[f] = (ox + (0 if k == "host" else 100), oy, oz)
            if r.get("stand"):
                self.refpos.setdefault(r["stand"], (ox, oy, oz))
            for i, (k, f) in enumerate(sorted((r.get("refs") or {}).items())):
                self.refpos[f] = (ox + 350 + 40 * i, oy + 30 * i, oz)
        for i, w in enumerate(L.refs("walk", "wp", "waypoint")):
            ox, oy, oz = L.origin("walk")
            self.refpos[w] = (ox + 250 * (i + 1), oy + (150 if i % 2 else -150), oz)
        npc = L.ref("npc", "npc")
        if npc:
            ox, oy, oz = L.origin("npc")
            self.refpos[L.ref("npc", "marker_a", "patrol_a")] = (ox + 1000, oy, oz)
            self.refpos[L.ref("npc", "marker_b", "patrol_b")] = (ox + 2200, oy, oz)
        for s in self.sides.values():
            off = set(L.initially_disabled("fight")) | set(L.initially_disabled("vats"))
            for e in L.refs("fight", "enemy") + L.refs("vats", "target"):
                s.objs[e] = {"kind": "actor", "hp": 100.0, "alive": True, "enabled": e not in off, "pos": list(self.refpos[e])}
            if npc:
                s.objs[npc] = {"kind": "actor", "hp": 100.0, "alive": True, "enabled": True, "pos": list(self.refpos[L.ref("npc", "marker_a", "patrol_a")])}
            cont = L.ref("loot", "container", "chest")
            if cont:
                ci = L.container_items() or {"FalloutNV.esm:015169": 5}
                s.objs[cont] = {"kind": "container", "items": dict(ci), "enabled": True, "pos": list(self.refpos[cont])}
            for f in L.refs("loot", "item", "loose", "floor"):
                s.objs[f] = {"kind": "item", "enabled": True, "pos": list(self.refpos[f])}
            d, tm = L.ref("door", "door"), L.ref("door", "terminal")
            if d:
                s.objs[d] = {"kind": "door", "open": False, "locked": True, "enabled": True, "pos": list(self.refpos[d])}
                s.objs[tm] = {"kind": "terminal", "enabled": True, "pos": list(self.refpos[tm])}
            q = L.ref("quest", "quest")
            if q:
                s.stages[q] = 0
                s.vars[q + "/" + str(L.extra("quest", "var", "labvar"))] = 0.0
        self.npc = npc

    # ---- messages
    def send(self, to: str, fn: Callable[["Side"], None], extra_delay: float = 0.0, tag: str = "") -> None:
        self.n += 1
        side = self.sides[to]
        late = self.rto if self.resend_every and self.n % self.resend_every == 0 else 0.0
        heapq.heappush(self.q, (self.t + self.latency + extra_delay + late, self.n, tag, lambda: fn(side)))

    def in_flight(self) -> bool:
        """Anything but a position update still on its way (the digest and allSynced wait for quiet, like the real ones)."""
        return any(e[2] != "pos" for e in self.q)

    def other(self, name: str) -> str:
        return CLIENT if name == HOST else HOST

    # ---- time
    def advance(self, dt: float) -> None:
        self.t = round(self.t + dt, 6)
        while self.q and self.q[0][0] <= self.t + 1e-9:
            _, _, _, fn = heapq.heappop(self.q)
            fn()
        for s in self.sides.values():
            s.hist.append((self.t, list(s.pos)))
            while s.hist and s.hist[0][0] < self.t - 6.0:
                s.hist.pop(0)
        self._npc(dt)
        if self.t - self.last_digest >= 1.0:
            self.last_digest = self.t
            self._digest()

    def _npc(self, dt: float) -> None:
        if not self.npc:
            return
        L = self.layout
        H, C = self.sides[HOST], self.sides[CLIENT]
        a, b = self.refpos[L.ref("npc", "marker_a", "patrol_a")], self.refpos[L.ref("npc", "marker_b", "patrol_b")]
        owner = self.truth["owner"]
        o = (H if owner == 0 else C).objs[self.npc]
        dh, dc = dist2(H.pos, o["pos"]), dist2(C.pos, o["pos"])
        want = owner
        if self.has("nearest_owner"):            # the old, wrong rule: the nearer player owns it, wherever the host is
            if owner == 0 and dc + 200 < dh:
                want = 1
            elif owner == 1 and dh + 200 < dc:
                want = 0
        elif not self.has("no_handoff"):         # synccore pickOwner: an eligible host (in the NPC's cell) always owns it; else the client
            if H.cell == 1:
                want = 0
            elif C.cell == 1:
                want = 1
        if want == owner:
            self.npc_pending = None
        elif self.npc_pending is None or self.npc_pending[1] != want:
            self.npc_pending = (self.t, want)
        elif self.t - self.npc_pending[0] >= 1.0:
            self.npc_pending = None
            self.truth = {"owner": want, "epoch": self.truth["epoch"] + 1}
            if self.has("handoff_snap"):          # the new owner's copy puts it 700 units off: a visible snap wherever it was
                (H if want == 0 else C).objs[self.npc]["pos"][0] += 700.0
            H.owner = dict(self.truth)
            if not self.has("owner_split"):
                self.send(CLIENT, lambda s, n=dict(self.truth): setattr(s, "owner", n))
            owner = want
            o = (H if owner == 0 else C).objs[self.npc]
        if not self.has("npc_static"):
            tgt = b if self.npc_target else a
            d = dist2(o["pos"], tgt)
            if d < 20:
                self.npc_target ^= 1
            else:
                step = min(d, (300.0 if self.has("slow_npc_samples") else 60.0) * dt)
                o["pos"][0] += (tgt[0] - o["pos"][0]) / d * step
                o["pos"][1] += (tgt[1] - o["pos"][1]) / d * step
        self.npc_tick += 1
        if self.npc_tick % 5 == 0 and not self.has("npc_pos_stale"):
            pos = list(o["pos"])
            self.send(CLIENT if owner == 0 else HOST, lambda s, p=pos: None if s.slot == self.truth["owner"] else s.objs[self.npc].__setitem__("pos", list(p)), tag="pos")   # a stale update from the old owner is dropped

    def _key_state(self, s: Side) -> Dict[str, Any]:
        out: Dict[str, Any] = {}
        for ref, o in s.objs.items():
            if o["kind"] == "actor" and ref != self.npc:
                out[f"actor/{ref}/alive"] = o["alive"]
                out[f"actor/{ref}/enabled"] = o["enabled"]
            if o["kind"] == "container":
                for it, n in o["items"].items():
                    out[f"container/{ref}/{it}"] = n
            if o["kind"] == "item":
                out[f"item/{ref}/enabled"] = o["enabled"]
            if o["kind"] == "door":
                out[f"door/{ref}/open"] = o["open"]
                out[f"door/{ref}/locked"] = o["locked"]
        if s.name == HOST and self.has("global_missing"):
            out["global/37"] = 19
        for q, v in s.stages.items():
            out[f"quest/{q}/stage"] = v
        for k, v in s.vars.items():
            out[f"quest/{k}"] = v
        return out

    def _digest(self) -> None:
        if self.in_flight():
            return
        h, c = self._key_state(self.sides[HOST]), self._key_state(self.sides[CLIENT])
        diff = [(k, h.get(k), c.get(k)) for k in sorted(set(h) | set(c)) if h.get(k) != c.get(k)]
        d = self.sides[HOST].digest
        if not diff:
            d["ok"] += 1
            return
        d["mismatch"] += 1
        self.report_id += 1
        self.sides[HOST].reports.append({"id": self.report_id, "slot": 1, "repaired": False,
                                         "entries": [{"key": k, "host": hv, "client": cv} for k, hv, cv in diff[:16]]})

    def noise(self) -> None:
        if self.has("digest_noise"):
            self.sides[HOST].digest["repairs"] += 1

    def crash(self, name: str) -> None:
        self.dead.add(name)


class FakeMachine(Machine):
    def __init__(self, world: World, name: str):
        self.w, self.name, self.role = world, name, name
        self.s = world.sides[name]
        self.o = world.sides[world.other(name)]

    def _up(self) -> None:
        if self.name in self.w.dead:
            raise ConnectionLost(f"{self.name} copy exited")

    def _obj(self, ref: str) -> Dict[str, Any]:
        self._up()
        if ref not in self.s.objs:
            raise KeyError(f"no such ref {ref}")
        return self.s.objs[ref]

    # ---- reads
    def status(self) -> Dict[str, Any]:
        self._up()
        w = self.w
        base = {"role": self.name, "lastSeq": int(w.t * 3), "counters": {"frames": int(w.t * w.fps[self.name])}}
        if self.name == HOST:
            d = self.s.digest
            base.update(host={"allSynced": not w.in_flight(), "connected": 1, "peers": [{"slot": 1, "acked": int(w.t * 3), "joined": not w.has("not_joined")}]},
                        desyncs={"digests": dict(d, received=d["ok"] + d["mismatch"]), "desyncs": list(self.s.reports)},
                        areadigest={"status": {"heals": {"total": 0}}})
        else:
            base.update(client={"joined": not w.has("not_joined"), "applied": int(w.t * 3), "rebases": 0}, transport={"connected": True})
        return base

    def cell_of(self, ref: Optional[str] = None) -> int:
        self._up()
        return self.s.cell if ref is None else 1

    def ref_id(self, ref: str) -> str:
        self._up()
        if ref in self.refs_known():
            if self.name == CLIENT and self.w.has("ref_id_differs") and ref == self.w.layout.refs("fight", "enemy")[0]:
                return "02" + ref.rsplit(":", 1)[1]
            return "01" + ref.rsplit(":", 1)[1]
        return ""

    def refs_known(self):
        return set(self.s.objs) | set(self.w.refpos)

    def exists(self, ref: str) -> bool:
        return bool(self.ref_id(ref))

    def pos(self, ref: Optional[str] = None) -> Tuple[float, float, float]:
        self._up()
        if ref is None:
            return tuple(self.s.pos)
        if ref in self.s.objs:
            if ref == self.w.npc and self.w.has("slow_npc_samples"):
                self.w.clock.sleep(0.5)
            return tuple(self.s.objs[ref]["pos"])
        return tuple(self.w.refpos[ref])

    def puppet_pos(self) -> Optional[Tuple[float, float, float]]:
        self._up()
        w = self.w
        if self.name == HOST and w.has("no_puppet"):
            return None
        lag = w.latency + (2.0 if w.has("puppet_lag") else 0.1)
        cut = w.t - lag
        pos = None
        for t, p in self.o.hist:
            if t <= cut:
                pos = p
        if pos is None:
            pos = self.o.pos
        pos = list(pos)
        if w.has("puppet_teleports") and int(w.t / 1.3) % 3 == 0:
            pos[0] += 600
        if w.has("puppet_offset") and self.name == CLIENT:
            pos[1] += 400
        return tuple(pos)

    def alive(self, ref: str) -> bool:
        return bool(self._obj(ref)["alive"])

    def health(self, ref: str) -> float:
        return float(self._obj(ref)["hp"])

    def enabled(self, ref: str) -> bool:
        return bool(self._obj(ref)["enabled"])

    def item_count(self, item: str, holder: Optional[str] = None) -> int:
        self._up()
        if holder is None:
            return int(self.s.items.get(item, 0))
        return int(self._obj(holder)["items"].get(item, 0))

    def door_state(self, ref: str) -> str:
        return "open" if self._obj(ref)["open"] else "closed"

    def locked(self, ref: str) -> bool:
        return bool(self._obj(ref)["locked"])

    def stage(self, quest: str) -> int:
        self._up()
        return int(self.s.stages.get(quest, 0))

    def var(self, quest: str, name: str) -> Optional[float]:
        self._up()
        if self.w.has("var_unreadable"):
            return None
        return float(self.s.vars.get(quest + "/" + name, 0.0))

    def owner(self, ref: str) -> Dict[str, Any]:
        self._up()
        return {"owner": self.s.owner["owner"], "epoch": self.s.owner["epoch"], "known": True, "mine": self.s.owner["owner"] == self.s.slot}

    # ---- moving
    def goto_ref(self, ref: str, dist: float = 0.0) -> Tuple[bool, str]:
        self._up()
        p = self.s.objs[ref]["pos"] if ref in self.s.objs else self.w.refpos[ref]
        self.s.pos = [p[0] + dist, p[1], p[2]]
        self.w.clock.sleep(0.2)
        return True, ""

    def coc(self, cells: List[str]) -> Tuple[bool, str]:
        self._up()
        lay = self.w.layout
        self.s.cell = 1 if cells[0] in (lay.cell_edid, getattr(lay, "cell_alt", "")) else 2
        self.s.pos = [0.0, 0.0, 0.0]
        self.w.clock.sleep(0.5)       # the real coc takes seconds, but the room's watchers sample all the while there; the fake has no watcher threads
        return True, ""

    def goto_pos(self, x: float, y: float, z: float) -> Tuple[bool, str]:
        self._up()
        self.s.pos = [x, y, z]
        self.w.clock.sleep(0.2)
        return True, ""

    def walk_to(self, x: float, y: float, z: float, timeout_s: float = 30.0, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]:
        self._up()
        t_end = self.w.t + timeout_s
        while self.w.t < t_end:
            d = dist2(self.s.pos, (x, y))
            if d <= 60:
                return True, ""
            step = min(d, SPEED * 0.25)
            self.s.pos[0] += (x - self.s.pos[0]) / d * step
            self.s.pos[1] += (y - self.s.pos[1]) / d * step
            self.w.clock.sleep(0.25)
            if tick:
                tick()
        return False, f"{dist2(self.s.pos, (x, y)):.0f} units short after {timeout_s:.0f} s"

    # ---- combat
    def arm(self) -> Tuple[bool, str]:
        self._up()
        self.s.armed = True
        return True, ""

    def _hurt(self, ref: str, hp: float, killer: str) -> None:
        o = self.s.objs[ref]
        o["hp"] = max(0.0, hp)
        o["alive"] = o["hp"] > 0
        w = self.w
        died = not o["alive"]
        if died and ((killer == HOST and w.has("host_kill_unseen")) or (killer == CLIENT and w.has("client_kill_unseen"))):
            return

        def apply(side: Side, ref=ref, hp=o["hp"], alive=o["alive"]):
            side.objs[ref]["hp"], side.objs[ref]["alive"] = hp, alive
            if alive:
                return
            if w.has("corpse_revives") and side.name == CLIENT:
                w.send(CLIENT, lambda s2: s2.objs[ref].update(hp=100.0, alive=True), 4.0)
            if w.has("survivor_hurt") and side.name == CLIENT:
                for r2, o2 in side.objs.items():
                    if o2["kind"] == "actor" and o2["alive"] and r2 != ref and r2 in w.layout.refs("fight", "enemy"):
                        o2["hp"] = 60.0
        w.send(w.other(self.name), apply, 6.0 if (died and w.has("slow_death")) else 0.0)
        if died:
            w.noise()

    def hold(self, ref: str) -> Tuple[bool, str]:
        self._up()
        if self.name == HOST:
            self.w.held.add(ref)
        return True, ""

    def kill(self, ref: str) -> Tuple[bool, str]:
        self._up()
        if self.name != HOST:
            return False, "a guest's console kill is refused"
        self._hurt(ref, 0.0, HOST)
        return True, ""

    def shoot(self, ref: str, volleys: int = 12, stand: Optional[str] = None) -> Tuple[bool, str]:
        self._up()
        if not self.s.armed:
            return False, "no weapon"
        if self.w.has("hostiles_charge") and ref not in self.w.held and self.w.t - self.w.enabled_at.get(ref, self.w.t) > 1.0:
            return False, "the shot failed: fire: FAILED too_close: the player is 60 units from the target (minimum 100)"
        if self.w.has("target_drifts") and ref not in self.w.held and self.w.t - self.w.enabled_at.get(ref, self.w.t) > 1.0:
            return False, "no clear shot even from the standoff (659 units, line of sight unknown); 3 approach(es)"
        for _ in range(volleys):
            o = self._obj(ref)
            if not o["alive"]:
                return True, ""
            self.w.clock.sleep(0.6)
            self._hurt(ref, o["hp"] - 40.0, self.name)
        return (not self._obj(ref)["alive"]), f"{volleys} volleys did not kill it"

    def vats(self, ref: str, limb: Optional[str] = None, stand: Optional[str] = None) -> Tuple[bool, str]:
        self._up()
        if limb:
            return False, "vats_target: FAILED rpc_error:unsupported: limb selection in VATS is not implemented"
        if not self.s.armed:
            return False, "VATS did not open (no weapon)"
        ok, why = self._menu_eats_input()
        if not ok:
            return False, "vats_target: FAILED vats_failed (3.28s) {'why': 'vats_not_entered (no weapon, no target in view, or no AP)'} " + why
        o = self._obj(ref)
        if not o["alive"]:
            return False, "dead"
        if self.w.has("hostiles_charge") and ref not in self.w.held and self.w.t - self.w.enabled_at.get(ref, self.w.t) > 1.0:
            return False, "vats_target: FAILED vats_failed (3.28s) {'why': 'vats_not_entered (no weapon, no target in view, or no AP)'}"
        self.s.vats_shots += 1
        self.w.clock.sleep(1.5)
        if self.s.vats_shots % 3 == 0:
            return True, "miss"
        hp = o["hp"] - VATS_DAMAGE
        o["hp"] = max(0.0, hp)
        o["alive"] = o["hp"] > 0
        w = self.w
        split = w.has("kill_health_split") and not o["alive"]
        if split:             # the owner (the host) publishes -14, a copy that is not the owner never holds under 1
            o["hp"] = -14.0 if self.name == HOST else 1.0
        if not w.has("vats_unsynced"):
            n = 2 if w.has("vats_double") else 1
            if split:
                far = 1.0 if self.name == HOST else -14.0
                w.send(w.other(self.name), lambda s, ref=ref, far=far: (s.objs[ref].__setitem__("hp", far), s.objs[ref].__setitem__("alive", False)))
            else:
                w.send(w.other(self.name), lambda s, ref=ref, hp=hp, n=n: (s.objs[ref].__setitem__("hp", max(0.0, s.objs[ref]["hp"] - VATS_DAMAGE * n)),
                                                                              s.objs[ref].__setitem__("alive", s.objs[ref]["hp"] > 0)))
        return True, ""

    # ---- loot
    def take(self, container: str, item: str, n: int = 1) -> Tuple[bool, str]:
        self._up()
        c = self._obj(container)
        if c["items"].get(item, 0) < n:
            return False, "not enough in the container"
        c["items"][item] -= n
        self.s.items[item] = self.s.items.get(item, 0) + n
        w = self.w
        if self.name == HOST and w.has("take_dupe"):
            c["items"][item] += n
        if not w.has("container_unsynced"):
            left = c["items"][item]
            w.send(w.other(self.name), lambda s, container=container, item=item, left=left: s.objs[container]["items"].__setitem__(item, left))
        w.noise()
        self.w.clock.sleep(0.3)
        return True, ""

    def pickup(self, ref: str) -> Tuple[bool, str]:
        self._up()
        o = self._obj(ref)
        if not o["enabled"]:
            return False, "world ref already gone"
        o["enabled"] = False
        item = self.w.loose_base[ref]
        self.s.items[item] = self.s.items.get(item, 0) + 1
        self.s.picked.add(ref)
        w, me = self.w, self.name
        w.noise()
        if w.has("pickup_unsynced"):
            return True, ""

        def claim(side: Side, ref=ref, me=me):
            if ref in side.picked:                      # both took it in the same moment: the host's pick stands
                if side.name == CLIENT and not w.has("dup_pickup"):
                    side.items[item] -= 1
                return
            side.objs[ref]["enabled"] = False
        w.send(w.other(me), claim)
        return True, ""

    def enable(self, ref: str) -> Tuple[bool, str]:
        self._up()
        self._obj(ref)["enabled"] = True
        self.w.enabled_at[ref] = self.w.t
        if self.name == HOST and not self.w.has("enable_unsynced"):
            self.w.send(CLIENT, lambda s, ref=ref: s.objs[ref].__setitem__("enabled", True))
        return True, ""

    def set_health(self, ref: str, hp: float) -> Tuple[bool, str]:
        self._up()
        o = self._obj(ref)
        if self.name != HOST:
            return True, ""                       # a guest's SetAV on an actor it does not own is dropped, as in the game
        o["hp"] = float(hp)
        if not self.w.has("weak_unsynced"):
            self.w.send(CLIENT, lambda s, ref=ref, hp=float(hp): s.objs[ref].__setitem__("hp", hp))
        return True, ""

    def disable(self, ref: str) -> Tuple[bool, str]:
        self._up()
        self._obj(ref)["enabled"] = False
        if self.name == HOST:
            self.w.send(CLIENT, lambda s, ref=ref: s.objs[ref].__setitem__("enabled", False))
        return True, ""

    # ---- doors
    def _menu_eats_input(self) -> Tuple[bool, str]:
        return (False, f"FAILED postcondition: the {self.s.menu} menu is open and eats the input") if self.s.menu else (True, "")

    def close_menus(self) -> Tuple[bool, str]:
        self.s.menu = ""
        return True, ""

    def open_door(self, ref: str) -> Tuple[bool, str]:
        d = self._obj(ref)
        ok, why = self._menu_eats_input()
        if not ok:
            return False, why
        if d["locked"] and not (self.name == HOST and self.w.has("locked_door_opens")):
            if self.w.has("locked_try_opens_lockpick"):
                self.s.menu = "lockpick"
            return False, "locked"
        d["open"] = True
        self._door_sync(ref, "open", True)
        return True, ""

    def close_door(self, ref: str) -> Tuple[bool, str]:
        d = self._obj(ref)
        ok, why = self._menu_eats_input()
        if not ok:
            return False, why
        d["open"] = False
        self._door_sync(ref, "open", False)
        return True, ""

    def _door_sync(self, ref: str, key: str, val: Any) -> None:
        w = self.w
        bug = "door_unsynced" if key == "open" else "lock_unsynced"
        if w.has(bug):
            return
        w.send(w.other(self.name), lambda s, ref=ref, key=key, val=val: s.objs[ref].__setitem__(key, val))

    def use_terminal(self, ref: str, entry: int) -> Tuple[bool, str]:
        self._obj(ref)
        door = self.w.layout.ref("door", "door")
        if self.w.has("terminal_noop"):
            self.w.clock.sleep(0.3)
            return True, ""
        self.s.objs[door]["locked"] = False
        if not self.w.has("terminal_unsynced"):
            self._door_sync(door, "locked", False)
        self.w.clock.sleep(0.3)
        return True, ""

    # ---- quest
    def set_stage(self, quest: str, stage: int) -> Tuple[bool, str]:
        self._up()
        prev = self.s.stages.get(quest, 0)
        self.s.stages[quest] = stage
        w = self.w
        lost = (self.name == HOST and w.has("stage_unsynced")) or (self.name == CLIENT and w.has("stage_client_lost"))
        if not lost:
            w.send(w.other(self.name), lambda s, q=quest, st=stage: s.stages.__setitem__(q, st))
        if self.name == HOST and w.has("stage_rollback") and stage >= 30:
            w.send(CLIENT, lambda s, q=quest, p=prev: s.stages.__setitem__(q, 20), 3.0)
        w.clock.sleep(0.2)
        return True, ""

    def set_var(self, quest: str, name: str, value: float) -> Tuple[bool, str]:
        self._up()
        self.s.vars[quest + "/" + name] = float(value)
        if not self.w.has("var_unsynced"):
            self.w.send(self.w.other(self.name), lambda s, k=quest + "/" + name, v=float(value): s.vars.__setitem__(k, v))
        return True, ""


class FakeEnv:
    """What lab.py needs from a world: pairs, machines, the way into the lab, the crash probe, a reset. One World per pair."""
    live = False

    def __init__(self, layout: Layout, bugs=(), pairs_max: int = 3, fps: Optional[Dict[str, float]] = None):
        self.layout, self.bugs, self.pairs_max = layout, set(bugs), pairs_max
        self.fps = dict(fps or {})        # {"host": 30, "client": 20}: the frame rate of the fake copies (default 60)
        unknown = self.bugs - set(BUGS)
        if unknown:
            raise ValueError(f"unknown fake bug(s) {sorted(unknown)}; the names are in labfake.BUGS")
        self.worlds: Dict[int, World] = {}
        self.prepared: List[int] = []
        self.finished = False

    def build_id(self) -> str:
        return "fake"

    def pairs_allowed(self, n: int) -> Tuple[int, str]:
        k = min(n, self.pairs_max)
        return k, (f"only {k} pair(s) fit" if k < n else "")

    def prepare(self, pair: int) -> Optional[Tuple[str, str]]:
        self.prepared.append(pair)
        self.worlds[pair] = self._world()
        return None

    def _world(self) -> World:
        w = World(self.layout, self.bugs)
        w.fps.update(self.fps)
        return w

    def copies_running(self) -> int:
        return 2 * len(self.worlds)

    def world(self, pair: int) -> World:
        return self.worlds[pair]

    def machines(self, pair: int) -> Tuple[FakeMachine, FakeMachine]:
        w = self.worlds[pair]
        return FakeMachine(w, HOST), FakeMachine(w, CLIENT)

    def clock(self, pair: int):
        return self.worlds[pair].clock

    def enter(self, pair: int) -> Callable[[], None]:
        w = self.worlds[pair]

        def go():
            w.clock.sleep(3.0)
            w.sides[HOST].cell = 1
            if not w.has("client_stays_out"):
                w.sides[CLIENT].cell = 1
        return go

    def probe(self, pair: int) -> Callable[[], List[str]]:
        w = self.worlds[pair]
        return lambda: sorted(w.dead)

    def reset(self, pair: int) -> Optional[Tuple[str, str]]:
        self.worlds[pair] = self._world()
        return None

    # ---- the network pass: the same world with the profile's delay and a lost-once message now and then
    def net_begin(self, pair: int, profile: str, seed: int = 7) -> Optional[str]:
        w = self.worlds[pair]
        p = lab_net.profiles().get(profile)
        if p is None:
            return f"no profile {profile!r} in tools/chaos/profiles.json"
        d, j = float(p.get("latency_ms", 0)), float(p.get("jitter_ms", 0))
        w.profile = profile
        w.latency += (d + j / 2.0) / 1000.0
        loss = float(p.get("loss", 0)) + (0.05 if float(p.get("burst_enter", 0)) else 0.0)
        w.resend_every = int(round(1.0 / loss)) if loss > 0 else 0
        w.rto = (2 * d + 4 * j) / 1000.0
        if w.has("net_slow"):
            w.latency = 12.0
        self.net_active = getattr(self, "net_active", set()) | {pair}
        return None

    def net_end(self, pair: int) -> Optional[Dict[str, Any]]:
        self.net_active = getattr(self, "net_active", set()) - {pair}
        return {"relay": {"c2s": {"forwarded": 100, "dropped": 1}, "s2c": {"forwarded": 100, "dropped": 1}}, "fake": True}

    def finish(self) -> None:
        self.finished = True
