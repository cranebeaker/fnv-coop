"""The room engine of the lab runner: checks that fail fast, time boxes, the first digest divergence, moved counters.

A room is a function `room(ctx)`. It talks to two machines (`ctx.host`, `ctx.client`, both a `Machine`) and records checks through the ctx:

    ctx.check(name, ok, expected, actual)           one fact; the first one that is not ok ends the room (fail fast)
    ctx.agree(name, fn, timeout_s, expected)        wait until fn(machine) gives the same answer on both machines (and `expected`)
    ctx.wait(name, fn, timeout_s, expected)         wait until fn() -> (ok, actual) holds
    ctx.skip(name, why)                              a check that cannot be made in this build (shown, never a failure)

Nothing here knows the game: the same rooms run against the fake world (tools/lab/labfake.py) and against the bridge (lab_live.py).
LAB_MUTANT=<name> switches on one deliberate bug for the twins in tools/lab/run_tests.py.
"""
from __future__ import annotations

import math
import os
import sys
import threading
import time
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT / "tools" / "proof"))
import pf_classify as C   # noqa: E402
import lab_fps   # noqa: E402

MUTANT = os.environ.get("LAB_MUTANT", "")   # test-only; names in tools/lab/run_tests.py

HOST, CLIENT = "host", "client"


# ------------------------------------------------------------------ how a room ends
class RoomError(Exception):
    pass


class Fail(RoomError):
    """A check was not ok. `rec` is the check record (already stored in ctx.checks)."""

    def __init__(self, rec: Dict[str, Any]):
        super().__init__(rec.get("name", ""))
        self.rec = rec


class Blocked(RoomError):
    """The room cannot run here: the layout lacks something (cls TEST), the build lacks a bridge method (BUILD), the game is not ready (ENV)."""

    def __init__(self, reason: str, cls: str = "TEST"):
        super().__init__(reason)
        self.reason, self.cls = reason, cls


class TimeBox(RoomError):
    pass


class Crashed(RoomError):
    def __init__(self, which: List[str], detail: str = ""):
        super().__init__(detail or f"game copy exited: {', '.join(which)}")
        self.which = which


class RealClock:
    virtual = False

    def now(self) -> float:
        return time.monotonic()

    def sleep(self, s: float) -> None:
        time.sleep(max(0.0, s))


# ------------------------------------------------------------------ the machine API (rooms use only these)
class Machine:
    """One copy of the game. lab_live.LiveMachine drives it through the bridge, labfake.FakeMachine simulates it. Refs are forms
    `Plugin.esp:XXXXXX`; the same form is the same runtime id on both copies. Methods that act return (ok, why) unless said otherwise."""
    name = HOST
    role = HOST

    def status(self) -> Dict[str, Any]: raise NotImplementedError                 # coop.status

    def frames(self) -> Optional[int]:
        """The plugin's frame counter of this copy (coop.status counters.frames; the bench turns it into fps), None when the build has none."""
        v = ((self.status() or {}).get("counters") or {}).get("frames")
        return None if v is None else int(v)
    def avatar_counts(self) -> Dict[str, int]:
        """Counters of the avatar stream on this copy: {"published": snapshots this copy's player published, "remote": snapshots received for the other player, "fed": poses fed
        to the puppet}. {} when the build cannot say. Their rates over a walk show the sender's near rate (nearHz) and the receiver's, bounded by the game's own fps."""
        return {}

    def avatar_stages(self) -> Dict[str, Any]:
        """avatar.status "stages" of this copy (plugin/avatar, the round of 2026-10-28): gauges {n, last, mean, max, sum} for send / rx / frameGap / sampleUs / driveUs / applyUs / poseToSetPosMs
        and "remotes": per remote player the playback buffer (lowLatency, offsetMs, jitterMs, wantDelayMs, delay, hold, extrap ...). {} when the build cannot say."""
        return {}

    def avatar_trace(self, n: int = 300) -> Dict[str, Any]:
        """avatar.trace of this copy: {"local": [[tick, x, y, z, vx, vy, vz]...], "remote": {slot: [[frame, x, y, z, ax, ay, az, state]...]}} on one clock. {} when the build cannot say."""
        return {}

    def cell_of(self, ref: Optional[str] = None) -> int: raise NotImplementedError   # the player's cell, or a ref's cell
    def ref_id(self, ref: str) -> str: raise NotImplementedError                  # the runtime id this copy resolves the form to ('' when it does not)
    def exists(self, ref: str) -> bool: raise NotImplementedError
    def pos(self, ref: Optional[str] = None) -> Tuple[float, float, float]: raise NotImplementedError
    def goto_ref(self, ref: str, dist: float = 0.0) -> Tuple[bool, str]: raise NotImplementedError          # teleport next to a ref
    def goto_pos(self, x: float, y: float, z: float) -> Tuple[bool, str]: raise NotImplementedError         # teleport
    def coc(self, cells: List[str]) -> Tuple[bool, str]: raise NotImplementedError                          # `coc <cell>` (first editor id that works), then wait until the load is over
    def walk_to(self, x: float, y: float, z: float, timeout_s: float = 30.0, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]: raise NotImplementedError
    def puppet_pos(self) -> Optional[Tuple[float, float, float]]: raise NotImplementedError                 # where THIS copy shows the other player
    def alive(self, ref: str) -> bool: raise NotImplementedError
    def health(self, ref: str) -> float: raise NotImplementedError
    def enabled(self, ref: str) -> bool: raise NotImplementedError                                          # placed and not disabled / picked up
    def arm(self) -> Tuple[bool, str]: raise NotImplementedError                                            # health up, a weapon with ammo equipped
    def enable(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError                              # console enable (host)
    def disable(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def set_health(self, ref: str, hp: float) -> Tuple[bool, str]: raise NotImplementedError               # SetAV Health on an actor (only its owner, the host, can make that stick)
    def kill(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def hold(self, ref: str) -> Tuple[bool, str]:                                                           # keep an actor where it stands (restrained); only its owner, the host, can make that stick
        return True, ""                                 # console kill (only the owner may)
    def shoot(self, ref: str, volleys: int = 12, stand: Optional[str] = None) -> Tuple[bool, str]: raise NotImplementedError   # fire until the ref is dead on this copy; `stand`: a safe spot to step back to when a hostile gets too close
    def vats(self, ref: str, limb: Optional[str] = None, stand: Optional[str] = None) -> Tuple[bool, str]: raise NotImplementedError   # no limb: the engine job cannot choose one
    def item_count(self, item: str, holder: Optional[str] = None) -> int: raise NotImplementedError         # holder None: this copy's player
    def combat_words(self) -> str:
        """This copy's combat module counters and last gate decision / rejections, short (for a failed shot); empty when it cannot say."""
        return ""

    def actor_stats(self) -> Dict[str, Any]:
        """The actor module's counters on this copy (coop.owner {stats:true}); empty when it cannot say."""
        return {}

    def heal_notes(self) -> List[str]:
        """The newest area-digest heals on this copy as text (key, before -> after); empty when it cannot say."""
        return []

    def close_menus(self) -> Tuple[bool, str]:
        """Close whatever menu this copy has open (best effort; nothing open is fine). A menu that stays (the lockpick menu of a locked door that was
        activated, a terminal, the first-use hint) eats the input of the next step on this copy: VATS does not enter, a door does not open."""
        return True, ""

    def can_take(self) -> Tuple[bool, str]:
        """(True, "") when this copy can take from a container the way the lab drives it; else (False, why). The lab takes with console lines (RemoveItem +
        AddItem), and a guest's console RemoveItem on a container is blocked on purpose (plugin/hooks gate: the guest's world changes go through the
        container menu and the intents). No bridge method clicks that menu yet, so the guest's take has no honest driver."""
        return True, ""

    def take(self, container: str, item: str, n: int = 1) -> Tuple[bool, str]: raise NotImplementedError
    def pickup(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def door_state(self, ref: str) -> str: raise NotImplementedError                                        # open | closed | ...
    def locked(self, ref: str) -> bool: raise NotImplementedError
    def open_door(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def close_door(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def use_terminal(self, ref: str, entry: int) -> Tuple[bool, str]: raise NotImplementedError
    def stage(self, quest: str) -> int: raise NotImplementedError
    def set_stage(self, quest: str, stage: int) -> Tuple[bool, str]: raise NotImplementedError
    def var(self, quest: str, name: str) -> Optional[float]: raise NotImplementedError                      # None: this build cannot read it
    def set_var(self, quest: str, name: str, value: float) -> Tuple[bool, str]: raise NotImplementedError
    def owner(self, ref: str) -> Dict[str, Any]: raise NotImplementedError                                  # coop.owner: owner, epoch, known, mine

    # ---- the parity room (lab_parity.py): what this copy shows, and the scripted situations that change it
    def parity_view(self) -> Dict[str, Any]:
        """What this copy SHOWS now (lab_parity.assemble): {"player": view, "puppets": {slot: view}, "actors": {runtime ref: view}, "time": {...}, "missing": [...]}."""
        raise NotImplementedError

    def npc_equip(self, ref: str, item: str) -> Tuple[bool, str]: raise NotImplementedError                 # give the NPC an item and equip it (only its owner can make that stick)
    def npc_draw(self, ref: str, on: bool = True) -> Tuple[bool, str]: raise NotImplementedError            # weapon out / away
    def npc_sit(self, ref: str, furniture: str) -> Tuple[bool, str]: raise NotImplementedError              # the NPC uses a furniture
    def npc_stand(self, ref: str) -> Tuple[bool, str]: raise NotImplementedError
    def npc_reset(self, ref: str, item: str) -> Tuple[bool, str]: raise NotImplementedError                 # holster, unequip and remove the item again (cleanup of the room)
    def draw_weapon(self) -> bool: raise NotImplementedError                                               # this copy's player takes the equipped weapon out
    def unarm(self) -> Tuple[bool, str]: raise NotImplementedError                                         # this copy's player puts every weapon away (fists)
    def punch(self, n: int = 3, gap_s: float = 0.8) -> Tuple[bool, str]: raise NotImplementedError         # this copy's player attacks n times with his fists
    def anim_category(self, who: Optional[str] = None) -> Optional[str]: raise NotImplementedError         # category of the playing animation: own player (None), "puppet", or a ref


def dist2(a, b) -> float:
    return math.hypot(a[0] - b[0], a[1] - b[1])


def dist3(a, b) -> float:
    return math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b)))


SHORT_EXPECTED = 200       # characters of an expected value kept in results.json
SHORT_NOTE = 1500          # ... of a check's note (the combat counters of both copies, the healed keys: evidence, not a verdict)
SHORT_ACTUAL = 400         # ... of an actual value (a failure reason must be readable whole by an AI: the why of a bot failure is 150..300 characters)


def short(v: Any, n: int = SHORT_ACTUAL) -> Any:
    """A value as it goes into results.json: numbers stay numbers, long text is cut (at n characters, with a ~)."""
    if isinstance(v, float):
        return round(v, 2)
    if isinstance(v, (int, bool)) or v is None:
        return v
    if isinstance(v, (list, tuple)):
        return [short(x, n) for x in v][:8]
    if isinstance(v, dict):
        return {str(k): short(x, n) for k, x in list(v.items())[:8]}
    s = str(v)
    return s if len(s) <= n else s[: n - 1] + "~"


# ------------------------------------------------------------------ digests
def new_desync_entries(desyncs: Dict[str, Any], since_id: int) -> List[Dict[str, Any]]:
    """The mismatching entries of the desync reports newer than `since_id`, OLDEST first: [{id, key, host, client, repaired}]."""
    out = []
    for r in sorted((desyncs or {}).get("desyncs") or [], key=lambda r: r.get("id", 0)):
        if r.get("id", 0) <= since_id:
            continue
        for e in r.get("entries") or []:
            out.append({"id": r.get("id"), "key": e.get("key"), "host": e.get("host"), "client": e.get("client"), "repaired": bool(r.get("repaired"))})
    return out


def digest_counts(status: Dict[str, Any]) -> Dict[str, int]:
    dg = ((status or {}).get("desyncs") or {}).get("digests") or {}
    return {k: int(dg.get(k, 0) or 0) for k in ("ok", "mismatch", "repairs")}


def connected(host_status: Dict[str, Any], client_status: Dict[str, Any]) -> Tuple[bool, str]:
    """Both copies agree they are in one session: the host sees the peer joined and everything acked, the client is joined and its transport is up."""
    h = (host_status or {}).get("host") or {}
    c = (client_status or {}).get("client") or {}
    tr = (client_status or {}).get("transport") or {}
    peers = h.get("peers") if isinstance(h.get("peers"), list) else []
    why = []
    if (host_status or {}).get("role") != "host":
        why.append(f"host role is {(host_status or {}).get('role')!r}")
    if (client_status or {}).get("role") != "client":
        why.append(f"client role is {(client_status or {}).get('role')!r}")
    if not peers or not all(p.get("joined") for p in peers):
        why.append("host sees no joined peer")
    if h.get("allSynced") is False:
        why.append("host: not all synced")
    if not c.get("joined"):
        why.append("client not joined")
    if tr.get("connected") is False:
        why.append("client transport down")
    return (not why), "; ".join(why)


# ------------------------------------------------------------------ the context of a room
class Ctx:
    def __init__(self, room: str, layout, host: Machine, client: Machine, clock=None, budget_s: float = 120.0, params: Optional[Dict[str, Any]] = None,
                 probe: Optional[Callable[[], List[str]]] = None, enter: Optional[Callable[[], None]] = None, area: str = "", log: Callable[[str], None] = lambda s: None,
                 profile: str = "", copies: Optional[Callable[[], int]] = None):
        self.room, self.layout, self.host, self.client = room, layout, host, client
        self.copies_fn = copies           # -> the number of game copies running now (the fps of a copy is a share of one PC)
        self.fps = lab_fps.FpsMeter([HOST, CLIENT])
        self._fps_at = -1e9
        self.profile = profile            # the chaos profile the client joined the host through ("" = the clean pass, a direct join)
        self.last_wait_s = 0.0            # seconds the last wait / agree / latency took
        self.clock = clock or RealClock()
        self.params = dict(params or {})
        self.probe, self.enter_hook, self.area, self.log = probe, enter, area, log
        self.t0 = self.clock.now()
        self.deadline = self.t0 + budget_s
        self.budget_s = budget_s
        self.cleanups: List[Callable[[], None]] = []
        self.checks: List[Dict[str, Any]] = []
        self.metrics: Dict[str, Any] = {}
        self.failure: Dict[str, Any] = {}
        self._flat0: Dict[str, float] = {}
        self._base_id = 0
        self.rebase()
        self.fps_sample(True)

    # ---- time
    @property
    def ms(self) -> List[Machine]:
        return [self.host, self.client]

    def now(self) -> float:
        return self.clock.now()

    def elapsed(self) -> float:
        return self.clock.now() - self.t0

    def sleep(self, s: float) -> None:
        self.tick()
        self.clock.sleep(s)

    def p(self, key: str) -> float:
        """A tunable number (--param room.key=value); the defaults are PARAMS in lab_rooms.py, merged into ctx.params by the runner.
        In a network pass `room.key@profile` wins over `room.key` when PARAMS has it (the per-profile thresholds)."""
        if self.profile and f"{self.room}.{key}@{self.profile}" in self.params and MUTANT != "net_clean_limits":   # twin: the clean limits judge every profile
            return float(self.params[f"{self.room}.{key}@{self.profile}"])
        try:
            return float(self.params[f"{self.room}.{key}"])
        except KeyError:
            raise Blocked(f"no parameter {self.room}.{key} (lab_rooms.PARAMS)", "TEST") from None

    def metric(self, key: str, value: Any) -> None:
        """Keep a number in the room's metrics at once, so a room that fails later still reports what it had measured (the network table needs them)."""
        self.metrics[key] = value

    def fps_sample(self, force: bool = False) -> None:
        """Read both copies' frame counters (every lab_fps.EVERY_S seconds of the room's clock, or at once when `force`). A copy that cannot answer is simply not sampled."""
        now = self.clock.now()
        if not force and now - self._fps_at < lab_fps.EVERY_S:
            return
        self._fps_at = now
        if MUTANT == "fps_unsampled":                      # twin: the room never samples after its start
            return
        for m in self.ms:
            try:
                f = m.frames()
            except Exception as e:   # noqa: BLE001 - a lost game is reported by the probe in tick(), a one-off bridge error costs one sample
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    return
                continue
            if f is not None:
                self.fps.add(m.name, now, f)

    def fps_report(self) -> Dict[str, Any]:
        """lab_fps.FpsMeter.report with the copies running now (the room's own end)."""
        try:
            n = int(self.copies_fn()) if self.copies_fn else 0
        except Exception:   # noqa: BLE001
            n = 0
        return self.fps.report(n)

    def tick(self) -> None:
        if self.clock.now() > self.deadline and MUTANT != "no_time_box":
            raise TimeBox(f"room time box of {self.budget_s:.0f} s used up")
        self.fps_sample()
        if self.probe:
            dead = self.probe()
            if dead:
                raise Crashed(list(dead))

    # ---- counters and digests
    def _flat(self) -> Dict[str, float]:
        flat: Dict[str, float] = {}
        for m in self.ms:
            try:
                C.flatten(m.status(), m.name + ":", flat)
            except Exception:   # noqa: BLE001 - a copy that cannot answer is reported by the check that needs it
                pass
        return {k.replace(":.", ":"): v for k, v in flat.items()}

    def _desyncs(self) -> Dict[str, Any]:
        try:
            return (self.host.status() or {}).get("desyncs") or {}
        except Exception:   # noqa: BLE001
            return {}

    def rebase(self) -> None:
        """From here on, counters and digest reports are measured against now (the spawn room does this once both copies are in the cell)."""
        self._flat0 = self._flat()
        self._base_id = max([r.get("id", 0) for r in self._desyncs().get("desyncs") or []] or [0])

    def moved(self) -> Dict[str, float]:
        return C.suspicious_moved(C.moved(self._flat0, self._flat()))

    def divergence(self) -> Optional[Dict[str, Any]]:
        ents = new_desync_entries(self._desyncs(), self._base_id)
        return ents[0] if ents else None

    def digests_moved(self) -> Dict[str, int]:
        now = digest_counts({"desyncs": self._desyncs()})
        before = {k: int(self._flat0.get(f"host:desyncs.digests.{k}", 0)) for k in now}
        return {k: now[k] - before[k] for k in now}

    # ---- recording
    def _rec(self, name: str, ok: bool, expected: Any, actual: Any, note: str, cls: str, area: str, secs: float, status: str = "") -> Dict[str, Any]:
        rec = {"name": name, "ok": bool(ok), "expected": short(expected, SHORT_EXPECTED), "actual": short(actual, SHORT_ACTUAL), "secs": round(secs, 2)}
        if note:
            rec["note"] = str(note)[:SHORT_NOTE]
        if status:
            rec["status"] = status
        if not ok:
            rec["class"] = cls
            rec["area"] = area or self.area
        self.checks.append(rec)
        return rec

    def check(self, name: str, ok: bool, expected: Any = None, actual: Any = None, note: str = "", cls: str = "SYNC", area: str = "") -> bool:
        """Record one fact. Not ok: the room ends here with the evidence (fail fast), so nothing after it runs."""
        self.tick()
        if MUTANT == "checks_never_fail":
            ok = True
        rec = self._rec(name, ok, expected, actual, note, cls, area, 0.0)
        if not ok:
            self._evidence()
            if MUTANT != "no_fail_fast":
                raise Fail(rec)
            return False
        return True

    def skip(self, name: str, why: str) -> None:
        self._rec(name, True, None, None, why, "", "", 0.0, status="skipped")

    def _evidence(self) -> None:
        self.failure = {"first_divergence": self.divergence(), "counters_moved": self.moved(), "digests_moved": self.digests_moved()}

    def wait(self, name: str, fn: Callable[[], Tuple[bool, Any]], timeout_s: float, expected: Any = None, poll_s: float = 0.25, note: str = "",
             cls: str = "SYNC", area: str = "") -> Tuple[Any, float]:
        """Poll fn() -> (ok, actual) until ok or timeout_s. Returns (actual, seconds). A timeout is the failed check."""
        t0 = self.now()
        end = t0 + timeout_s
        last: Any = None
        while True:
            self.tick()
            ok, last = fn()
            if ok:
                secs = self.now() - t0
                self.last_wait_s = secs
                self._rec(name, True, expected, last, note, cls, area, secs)
                return last, secs
            if self.now() >= end:
                break
            self.clock.sleep(poll_s)
        secs = self.now() - t0
        rec = self._rec(name, False, expected, last, (note + " " if note else "") + f"not true after {secs:.1f} s", cls, area, secs)
        self._evidence()
        raise Fail(rec)

    def both(self, fn: Callable[[Machine], Any]) -> Dict[str, Any]:
        return {HOST: fn(self.host), CLIENT: fn(self.client)}

    def par(self, fh: Callable[[], Any], fc: Callable[[], Any]) -> Dict[str, Any]:
        """Run one action per machine at the same moment (real threads in a game; one after the other, in no time, in the fake world)."""
        out: Dict[str, Any] = {}
        if getattr(self.clock, "virtual", False):
            out[HOST], out[CLIENT] = fh(), fc()
            return out
        errs: List[BaseException] = []

        def run(k, f):
            try:
                out[k] = f()
            except BaseException as e:   # noqa: BLE001
                errs.append(e)
        ts = [threading.Thread(target=run, args=(k, f), daemon=True) for k, f in ((HOST, fh), (CLIENT, fc))]
        [t.start() for t in ts]
        [t.join(timeout=max(1.0, self.deadline - self.now() + 5)) for t in ts]
        if errs:
            raise errs[0]
        return out

    def agree(self, name: str, fn: Callable[[Machine], Any], timeout_s: float, expected: Any = None, poll_s: float = 0.25, note: str = "",
              cls: str = "SYNC", area: str = "") -> Any:
        """Wait until fn(host) == fn(client) (and == expected when given). Returns the agreed value; a timeout fails with both values."""
        def probe():
            vals = self.both(fn)
            ok = vals[HOST] == vals[CLIENT] and (expected is None or vals[HOST] == expected)
            return ok, vals
        vals, _ = self.wait(name, probe, timeout_s, expected, poll_s, note, cls, area)
        return vals[HOST]

    def timed(self, action: Callable[[], Any], src: Callable[[], bool], dst: Callable[[], bool], window_s: float, poll_s: float = 0.03) -> Tuple[Any, Dict[str, Optional[float]]]:
        """Run `action` and time the change it causes on two copies with ONE clock: `src` (true once the copy that acted shows the change) and `dst` (true once the OTHER
        copy shows it) are polled by a watcher thread each, every ~30 ms, from before the action starts, so the moment a copy first showed the change is not blurred by how
        long the action itself took (a shot that needs 20 volleys, a pickup that walks). After the action the watchers go on until both have seen it or `window_s` seconds
        passed since the acting copy showed it. -> (the action's result, {"src": time or None, "dst": time or None}) in the runner's clock (time.monotonic).
        A virtual clock (the fake world) cannot run threads: the action runs first and both are polled after it."""
        times: Dict[str, Optional[float]] = {"src": None, "dst": None}
        if getattr(self.clock, "virtual", False):
            res = action()
            end = self.now() + window_s
            while True:
                self.tick()
                if times["src"] is None and src():
                    times["src"] = self.now()
                if times["src"] is not None and times["dst"] is None and dst():
                    times["dst"] = self.now()
                if times["dst"] is not None or self.now() >= end:
                    break
                self.clock.sleep(poll_s)
            return res, times
        stop = threading.Event()
        errs: List[BaseException] = []

        def watch(key: str, fn: Callable[[], bool]) -> None:
            while not stop.is_set() and times[key] is None:
                try:
                    if fn():
                        times[key] = self.now()
                        return
                except BaseException as e:   # noqa: BLE001 - a lost game is reported by the probe in tick(); a one-off bridge error is retried
                    if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                        errs.append(e)
                        return
                time.sleep(poll_s)
        ths = [threading.Thread(target=watch, args=(k, f), daemon=True) for k, f in (("src", src), ("dst", dst))]
        [t.start() for t in ths]
        try:
            res = action()
            after = self.now()
            while not errs and not (times["src"] is not None and times["dst"] is not None):
                self.tick()
                if self.now() >= ((times["src"] if times["src"] is not None else after) + window_s):
                    break
                time.sleep(0.02)
        finally:
            stop.set()
            [t.join(timeout=5) for t in ths]
        if errs:
            raise errs[0]
        return res, times

    @staticmethod
    def ms_of(times: Dict[str, Optional[float]]) -> Optional[int]:
        """The spread of a `timed` change in ms, or None when a copy never showed it (for numbers a check does not judge)."""
        a, b = times.get("src"), times.get("dst")
        return None if a is None or b is None else int(round(abs(b - a) * 1000.0))

    def latency_check(self, name: str, times: Dict[str, Optional[float]], limit_s: float, expected: Any = None, note: str = "", cls: str = "SYNC", area: str = "",
                      show: Optional[Callable[[], Any]] = None) -> int:
        """Record the check for a `timed` change: ok when the other copy showed it no later than `limit_s` after the acting copy did. -> milliseconds between the two copies showing it (0 when
        the poll could not tell them apart). A change the other copy never showed, or showed too late, is the failed check, with `show()` as its actual value."""
        t_src, t_dst = times.get("src"), times.get("dst")
        # abs(): the copy that acted is not always the first to show the change (a guest's kill or stage is applied by the host first and comes back), so the number is the
        # spread between the two copies showing it, whichever was first
        ms = None if t_src is None or t_dst is None else int(round(abs(t_dst - t_src) * 1000.0))
        ok = ms is not None and ms <= limit_s * 1000.0
        if MUTANT == "checks_never_fail":
            ok = True
        if ok:
            self.last_wait_s = ms / 1000.0
            self._rec(name, True, expected, {"ms": ms}, note, cls, area, ms / 1000.0)
            return ms
        why = ("the acting copy never showed it" if t_src is None else "the other copy never showed it" if t_dst is None else f"the other copy showed it {ms / 1000.0:.1f} s after the acting copy")
        rec = self._rec(name, False, expected, show() if show else {"acting": t_src is not None, "other": t_dst is not None}, (note + " " if note else "") + why + f" (limit {limit_s:.1f} s)",
                        cls, area, (ms or 0) / 1000.0)
        self._evidence()
        raise Fail(rec)

    def latency(self, name: str, src: Callable[[], bool], dst: Callable[[], bool], timeout_s: float, expected: Any = None, poll_s: float = 0.05,
                note: str = "", cls: str = "SYNC", area: str = "", show: Optional[Callable[[], Any]] = None) -> int:
        """`timed` with no action: poll both copies for a change that has already happened (or is under way) and check it. -> milliseconds."""
        _, tm = self.timed(lambda: None, src, dst, timeout_s, poll_s)
        return self.latency_check(name, tm, timeout_s, expected, note, cls, area, show)

    # ---- helpers every room uses
    def ref(self, *prefixes: str, n: Optional[int] = None) -> Any:
        """Refs of this room by key prefix; n=None gives the list, n=0 the first one."""
        refs = self.layout.refs(self.room, *prefixes)
        if n is None:
            return refs
        if len(refs) <= n:
            raise Blocked(f"layout room {self.room} has no ref #{n} named {'/'.join(prefixes)}*", "TEST")
        return refs[n]

    def close_menus(self, who: Optional[List[Machine]] = None) -> None:
        """Close what a step left open on these copies (default both): a menu stays open when the step that opened it failed (the lockpick menu of a locked
        door that was activated, a terminal), and it eats the input of every later step: the first lab run's door and VATS rooms."""
        if MUTANT == "menus_left_open":                        # twin: the menu stays open
            return
        for m in (who if who is not None else self.ms):
            m.close_menus()

    def stand_all(self) -> None:
        """Both players to the room's stand marker (the host a little apart from the client). A menu an earlier room left open is closed first."""
        self.close_menus()
        stand = self.layout.stand(self.room)
        for i, m in enumerate(self.ms):
            ok, why = m.goto_ref(stand, 60.0 * i)
            self.check(f"{m.name} stands at the {self.room} stand marker", ok, "arrived", why or "arrived", cls="ENV", area="tools/lab (teleport)")
        self.clock.sleep(1.0)

    def enable_hostiles(self, only: Optional[List[str]] = None) -> None:
        """The plugin places the room's hostiles disabled (they would charge the players of other rooms). The host enables them now (all of them, or
        `only` these: a room that shoots one hostile at a time enables just that one, so the others do not walk up to the shooter meanwhile), they must
        show on both copies, and they are disabled again when the room ends."""
        refs = list(only) if only is not None and MUTANT != "enable_all_at_once" else self.layout.initially_disabled(self.room)   # twin: all at once
        for r in refs:
            ok, why = self.host.enable(r)
            self.check(f"the host enables {r.split(':')[1]}", ok, "enabled", why or "enabled", cls="TEST", area="tools/lab (enable)")
        if refs:
            self.agree("the enabled hostiles are in the world on both copies", lambda m: [m.enabled(r) for r in refs], 15.0, [True] * len(refs), area="plugin/spawn, plugin/actors")
            self.cleanups.append(lambda: [self.host.disable(r) for r in refs])

    def refs_exist(self, refs: List[str], what: str) -> None:
        """Placed refs are persistent: each one resolves to the SAME id on both copies. Missing on both = the plugin or the layout is wrong (TEST)."""
        ids = {m.name: {r: m.ref_id(r) for r in refs} for m in self.ms}
        gone = [r for r in refs if not ids[HOST][r] and not ids[CLIENT][r]]
        if gone:
            raise Blocked(f"{what}: {gone[:3]} resolve on neither copy (is NVCoopLab.esp loaded? does the layout match the plugin?)", "TEST")
        bad = {r: (ids[HOST][r], ids[CLIENT][r]) for r in refs if not ids[HOST][r] or ids[HOST][r] != ids[CLIENT][r]}
        self.check(f"{what} resolve to the same ids on both copies", not bad, "same id", bad or "same", area="plugin/coop (persistent refs)")


# ------------------------------------------------------------------ running one room
def classify_exception(e: BaseException) -> Tuple[str, str, str]:
    """-> (verdict, class, reason) for something a room did not expect."""
    names = [c.__name__ for c in type(e).__mro__]
    text = str(e)[:400]
    if "ConnectionLost" in names or "GameCrashed" in names:
        return "FAIL", "CRASH", f"lost the game: {text}"
    if "RpcError" in names and getattr(e, "code", None) == -32601:
        return "BLOCKED", "BUILD", f"bridge method missing in this build: {text}"
    if "BridgeError" in names or "WaitTimeout" in names:
        low = text.lower()
        cls = "ENV" if any(w in low for w in ("service", "loading", "timed out", "timeout")) else "TEST"
        return "FAIL", cls, f"bridge: {text}"
    return "FAIL", "TEST", f"room code error {type(e).__name__}: {text}"


def run_room(name: str, fn: Callable[[Ctx], Optional[Dict[str, Any]]], ctx: Ctx) -> Dict[str, Any]:
    """Run one room function and return its result dict (no exception escapes). The clean rule runs last: no digest repair and no area heal."""
    res: Dict[str, Any] = {"room": name, "verdict": "PASS", "class": None, "reason": "", "area": ctx.area}
    t0 = time.time()
    try:
        extra = fn(ctx)
        if isinstance(extra, dict):
            ctx.metrics.update(extra)
        mv = {k: v for k, v in C.moved(ctx._flat0, ctx._flat()).items()}
        bad = [] if MUTANT == "no_clean_rule" else C.unclean(mv)
        notes = []
        if bad:   # name the keys: the counter alone ("heals.total +1") cannot say which actor was healed from what to what (lab run 20261028-032920, vats behind bad_wifi)
            for m in ctx.ms:
                try:
                    notes += [f"{m.name} healed {h}" for h in m.heal_notes()[-3:]]
                except Exception as e:   # noqa: BLE001
                    if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                        raise
        ctx.check("no digest repair or area heal during the room", not bad, 0, bad or 0, note=" | ".join(notes), area="plugin/areadigest, synccore digest")
    except Fail as e:
        res.update(verdict="FAIL", **{"class": e.rec.get("class") or "SYNC"}, reason=_why(e.rec), area=e.rec.get("area") or ctx.area)
    except Blocked as e:
        res.update(verdict="BLOCKED", **{"class": e.cls}, reason=e.reason)
    except TimeBox as e:
        res.update(verdict="TIMEOUT", **{"class": "ENV"}, reason=str(e))
        ctx._evidence()
    except Crashed as e:
        res.update(verdict="FAIL", **{"class": "CRASH"}, reason=str(e))
    except Exception as e:   # noqa: BLE001
        v, c, why = classify_exception(e)
        res.update(verdict=v, **{"class": c}, reason=why)
        if v == "FAIL" and c != "CRASH":
            ctx._rec("room step raised", False, None, why, "", c, ctx.area, 0.0)
        if v == "FAIL":
            ctx._evidence()
    if res["verdict"] != "PASS" and not ctx.failure:
        try:
            ctx._evidence()
        except Exception:   # noqa: BLE001
            pass
    try:
        ctx.fps_sample(True)                     # the room's last reading: the end of the interval that the rooms' last step ran in
    except Exception:   # noqa: BLE001
        pass
    for fn_ in reversed(ctx.cleanups):       # put the lab back for the next room (not judged: a failing cleanup is not a finding)
        try:
            fn_()
        except Exception:   # noqa: BLE001
            pass
    res["seconds"] = round(ctx.elapsed(), 1)
    res["wall_seconds"] = round(time.time() - t0, 1)
    res["checks"] = ctx.checks
    res["metrics"] = ctx.metrics
    res["fps"] = ctx.fps_report()
    if res["verdict"] != "PASS":
        res["first_divergence"] = ctx.failure.get("first_divergence")
        res["counters_moved"] = ctx.failure.get("counters_moved") or {}
        res["digests_moved"] = ctx.failure.get("digests_moved") or {}
    return res


def _why(rec: Dict[str, Any]) -> str:
    return f"{rec.get('name')}: expected {rec.get('expected')!r}, got {rec.get('actual')!r}" + (f" ({rec['note']})" if rec.get("note") else "")
