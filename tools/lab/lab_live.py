"""The real game behind the lab rooms: LiveMachine (one copy through the bridge and the bot layer) and LiveEnv (pairs, boot, lab plugin, load, coc).

Nothing in this file has run in a game yet. LiveMachine only calls what tools/bots/bot.py and tools/runner/nvbridge.py already do; the parts that are
guesses are marked UNVERIFIED and listed in docs/tools/lab.md. LiveEnv reuses prove.py's LiveEnv (slot, RAM guard, provisioning, boot, saves, crash watch)
as its own lane ("lab"), so a proof queue and a lab run never share a lease name or an owned.json.
"""
from __future__ import annotations

import hashlib
import json
import math
import os
import subprocess
import sys
import threading
import time
from pathlib import Path
from types import SimpleNamespace
from typing import Any, Callable, Dict, List, Optional, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
for _p in (ROOT / "tools", ROOT / "tools" / "runner", ROOT / "tools" / "proof", ROOT / "tools" / "scenarios", HERE):
    if str(_p) not in sys.path:
        sys.path.insert(0, str(_p))

import lab_core as K   # noqa: E402
import lab_net   # noqa: E402
import lab_parity   # noqa: E402
from lab_core import CLIENT, HOST, Blocked, Machine   # noqa: E402

VATS_COOLDOWN_S = 3.0     # seconds between the end of one VATS shot and the start of the next one on a copy

PY = [sys.executable, "-I", "-X", "utf8"]
WHY = 400                             # characters of a bot failure kept in a check (the why of a bot failure is 150..300 characters and an AI reads it whole)
ENTER_DIGEST_S = 25.0                 # how long entering the lab waits for the first ok digest (the spawn room judges it afterwards)
SHOT_MIN, SHOT_MAX = 150.0, 500.0      # a shot is taken from between these distances (the pistol is accurate there; aim_at refuses under 100)
STANDOFF = 350.0                      # the distance the bench shoots from (tools/bench/arena_content.py)
WEAPON = "FalloutNV.esm:0E3778"       # 9mm pistol (the Goodsprings enemy scenarios use it)
AMMO = "FalloutNV.esm:08ED03"         # 9mm ammo
# the switches the lab needs in nvcoop.ini: the union of what the proof manifest's two-copy tests switch on (loot, vats, digests, dialogue included)
LAB_INI = ["coop.enable=1", "dev.bots=1", "dev.animprobe=1", "live.actors=1", "live.avatars=1", "live.combat=1", "live.spawns=1", "avatar.enabled=1",
           "combat.enabled=1", "combat.gate=1", "combat.native=1", "anim.enabled=1", "vats.enabled=1", "areadigest.enabled=1", "loot.enabled=1",
           "coop.baseline=1", "coop.dialogue=1", "intents.enable=1", "intents.pickup=0", "intents.container=0", "worldtime.enabled=1", "dev.actorport=1", "dev.parityprobe=1"]
# extra nvcoop.ini keys for one run, "section.key=value,section.key=value" (a "+" does the same as the comma: schtasks hands a comma list to PowerShell as separate arguments) (an A/B run of a fix switch: `LAB_INI_EXTRA=actors.norepeatfinal=1+actors.noavaudit=1`)
LAB_INI = LAB_INI + [x.strip() for v in ("LAB_INI_EXTRA", "LAB_EXTRA_INI") for x in os.environ.get(v, "").replace("+", ",").split(",") if "=" in x]   # LAB_EXTRA_INI: the same, under the avatar docs' name


def _hexid(form_or_hex: str) -> str:
    return form_or_hex.split(":")[-1].lstrip("0") or "0"


class LiveMachine(Machine):
    """One game copy. `nv` is an NvBridge (or the bots' FakeBridge in the unit tests)."""

    def __init__(self, nv, name: str, bot=None, weapon: str = WEAPON, ammo: str = AMMO):
        from bots import Bot
        self.nv, self.name, self.role = nv, name, name
        self.bot = bot or Bot(nv, name=name)
        self.bot.auto_dismiss = True
        self.bot.look_via_console = True    # bot.look does nothing on a client copy (in game): a walk steers with the console line, which works on both
        self.weapon, self.ammo = weapon, ammo
        self._var_seen: Dict[Tuple[str, str], float] = {}
        self._var_expr: Optional[str] = None
        self._vats_done_at = -1e9

    # ---- reads
    def status(self) -> Dict[str, Any]:
        st = dict(self.nv.call("coop.status", {}, timeout_s=10))
        try:
            st["areadigest"] = {"status": self.nv.call("areadigest.status", {}, timeout_s=10)}
        except Exception as e:   # noqa: BLE001 - a build without the module: the heal counter is simply not there
            if getattr(e, "code", None) != -32601:
                raise
        return st

    COMBAT_KEYS = ("engineHits", "proceeded", "clamped", "neutralized", "ghostHitsNeutralized", "envHitsNeutralized", "hitsSent", "hitsReceived", "hitsApplied", "hitsRejected",
                   "hitDuplicates", "nacksSent", "nacksReceived", "hitRetries", "hitsAbandoned", "hitsResolvedLate", "damageApplied", "damageRefused", "engineRefused", "naturalHits",
                   "killsForced", "deathsDeclared", "deathsAccepted", "unroutable")

    def combat_words(self) -> str:
        """combat.status of this copy in one line: the non-zero counters, the gate's last decision and the host's newest rejections. The failure of a shot that landed nowhere
        (fight, transatlantic, 2026-10-28) could not say whether the hit was never forwarded, clamped, rejected by the owner or lost: it now names both copies' numbers."""
        try:
            st = self.nv.call("combat.status", {}, timeout_s=10)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return f"combat.status: {str(e)[:80]}"
        if not isinstance(st, dict):
            return ""
        c = st.get("stats") or {}
        nums = " ".join(f"{k}={c[k]}" for k in self.COMBAT_KEYS if c.get(k))
        gate = st.get("gate") or {}
        extra = "; ".join(f"{k}: {str(c[k])[:90].strip()}" for k in ("lastGate", "rejections") if c.get(k))
        return f"{nums or 'all counters 0'}; gate calls={gate.get('calls')} errors={gate.get('errors')} offThread={gate.get('offThread')}" + (f"; {extra}" if extra else "")

    def actor_stats(self) -> Dict[str, Any]:
        try:
            st = self.nv.call("coop.owner", {"stats": True}, timeout_s=10)
        except Exception as e:   # noqa: BLE001 - a build without the stats answer
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return {}
        out = dict(st) if isinstance(st, dict) else {}
        try:   # the port's own count of drifted healths it wrote back (plugin/actors/engine)
            out["avDrifts"] = int(((self.nv.call("actors.port", {}, timeout_s=10) or {}).get("stats") or {}).get("avDrifts") or 0)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
        return out

    def heal_notes(self) -> List[str]:
        try:
            st = self.nv.call("areadigest.status", {}, timeout_s=10)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return []
        notes = [str(x) for x in (st.get("lastHeals") or [])] if isinstance(st, dict) else []
        try:   # the actor port's audit of this copy's health against the owner's (plugin/actors): how many drifts it had to write back, and the newest
            ps = (self.nv.call("actors.port", {}, timeout_s=10) or {}).get("stats") or {}
            if ps.get("avDrifts"):
                notes.append(f"actor port wrote back {ps['avDrifts']} drifted health(s); newest: net {ps.get('lastDriftNet')} read {ps.get('lastDriftCur')} owner {ps.get('lastDriftWant')}")
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
        return notes

    def frames(self) -> Optional[int]:
        """The frame counter alone (coop.status without the area digest call: it is read twice a second per copy by every room)."""
        v = (self.nv.call("coop.status", {}, timeout_s=10).get("counters") or {}).get("frames")
        return None if v is None else int(v)

    def avatar_counts(self) -> Dict[str, int]:
        av = self.nv.call("avatar.status", {}, timeout_s=5)
        loc = av.get("local") or {}
        out = {"published": loc.get("published"), "remote": av.get("remoteSnapshots"), "fed": av.get("posesFed")}
        return {k: int(v) for k, v in out.items() if isinstance(v, (int, float))}

    def avatar_trace(self, n: int = 300) -> Dict[str, Any]:
        return self.nv.call("avatar.trace", {"n": int(n)}, timeout_s=15)

    def avatar_stages(self) -> Dict[str, Any]:
        av = self.nv.call("avatar.status", {}, timeout_s=5)
        st = dict(av.get("stages") or {})
        if st:
            st["lowLatency"] = av.get("lowLatency")
        return st

    def cell_of(self, ref: Optional[str] = None) -> int:
        return int(self.bot.cell(ref))

    def ref_id(self, ref: str) -> str:
        try:
            self.bot._info(ref)             # noqa: SLF001 (the bot's own lookup: the plugin must be loaded and the ref placed)
            return self.bot._norm(ref)[1]   # noqa: SLF001 (the runtime id with this copy's load-order index)
        except Exception:                   # noqa: BLE001
            return ""

    def exists(self, ref: str) -> bool:
        return bool(self.ref_id(ref))

    def pos(self, ref: Optional[str] = None) -> Tuple[float, float, float]:
        return tuple(self.bot.pos(ref))

    def alive(self, ref: str) -> bool:
        return bool(self.bot.alive(ref))

    def health(self, ref: str) -> float:
        return float(self.bot.health(ref))

    def enabled(self, ref: str) -> bool:
        from bots import primitives as P
        try:
            return not (int(self.bot._info(ref)["flags"]) & P.FLAGS_GONE)   # noqa: SLF001 (disabled, or deleted: a picked-up item is deleted, see primitives.py)
        except Exception:   # noqa: BLE001 - a picked-up item may stop resolving at all
            return False

    def item_count(self, item: str, holder: Optional[str] = None) -> int:
        return int(self.bot.inventory_count(item, holder))

    def door_state(self, ref: str) -> str:
        return self.bot.door_state(ref)

    def locked(self, ref: str) -> bool:
        return bool(self.bot.locked(ref))

    def stage(self, quest: str) -> int:
        return int(self.bot.quest_stage(quest))

    def owner(self, ref: str) -> Dict[str, Any]:
        return dict(self.nv.call("coop.owner", {"ref": self.bot._norm(ref)[0]}, timeout_s=10))   # noqa: SLF001

    def entities(self) -> Dict[str, Any]:
        return dict(self.nv.call("coop.entities", {}, timeout_s=10))

    puppet_diag: Dict[str, Any] = {}

    def puppet_pos(self) -> Optional[Tuple[float, float, float]]:
        """Where this copy shows the OTHER player (avatar.status puppets[0]; the ref's position through the console, as coop_bench samples it).
        `puppet_diag` keeps what that read saw (the puppet's phase and ref, the remote snapshots received, poses fed, spawn calls), for the spawn room's trace."""
        av = self.nv.call("avatar.status", {}, timeout_s=5)
        pu = av.get("puppets") or []
        eng = av.get("engine") or {}
        self.puppet_diag = {"n": len(pu), "phase": pu[0].get("phase") if pu else None, "ref": pu[0].get("ref") if pu else None,
                            "snaps": av.get("remoteSnapshots"), "fed": av.get("posesFed"), "spawnCalls": eng.get("spawnCalls"), "throttled": eng.get("spawnThrottled")}
        if pu:      # where the buffer says the puppet stands and where the driver last put it (plugin/avatar, round of 2026-10-28), and the newest snapshot's age at the buffer
            rm = ((av.get("stages") or {}).get("remotes") or [{}])
            rm = rm[0] if rm else {}
            self.puppet_diag.update({"pose": pu[0].get("pose"), "applied": pu[0].get("applied"), "haveApplied": pu[0].get("haveApplied"), "pushes": rm.get("pushes"),
                                     "newestAge": rm.get("newestAgeMs"), "teleportSnaps": rm.get("teleportSnaps"), "snapsN": rm.get("snaps")})
            try:    # the network service's own counters: what it sent / skipped / was refused (coop.live "avatars")
                av2 = (self.nv.call("coop.live", {}, timeout_s=5).get("avatars") or {})
                st = av2.get("stream") or {}
                self.puppet_diag["net"] = (f"sent {av2.get('sent')} rx {av2.get('received')} acc {av2.get('accepted')} rej {av2.get('rejected')} noRoster {av2.get('noRoster')} kept {av2.get('rateKept')}"
                                           f" rel {av2.get('rateReleased')} q {st.get('senderQueued')} idle {st.get('senderSkippedIdle')} needBase {st.get('recvNeedBaseline')} stale {st.get('recvStaleSeq')}")
            except Exception as e:   # noqa: BLE001 - a build without the counters
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
        if not pu or not pu[0].get("ref"):
            return None
        rid = str(pu[0]["ref"]).replace("0x", "")
        if not rid.strip("0"):
            return None             # the puppet is announced and not placed yet (ref 0): behind a slow link that lasts seconds (bad_wifi, 2026-10-27)
        try:
            return tuple(self.nv.eval(f"GetPos {a}", ref=rid) for a in "XYZ")
        except Exception as e:   # noqa: BLE001 - "ref ... is not a loaded form" while the puppet is being created
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return None

    # ---- the parity room (lab_parity.py). Reads only existing bridge methods (avatar.status, anim.probe, world.peek, vats.state) and parity.look
    # (plugin/probe/parity_probe.cpp); the situations are console lines and dev methods the lab already uses elsewhere. UNVERIFIED in a game: the NPC lines.
    PARITY_RADIUS = 20000.0        # every actor of the lab cell
    PARITY_MAX = 64                # anim.probe's limit per call
    _puppet_cache: Tuple[float, Optional[str]] = (0.0, None)

    def _probe(self, method: str, params: Dict[str, Any], timeout_s: float = 10.0) -> Tuple[Optional[Dict[str, Any]], str]:
        """(answer, "") or (None, why): a build without the method (-32601) or a method that refuses is a missing read, not a crash. A lost game still raises."""
        try:
            r = self.nv.call(method, params, timeout_s=timeout_s)
            return (r if isinstance(r, dict) else {}), ""
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return None, ("no such method" if getattr(e, "code", None) == -32601 else str(e)[:WHY])

    def parity_view(self) -> Dict[str, Any]:
        av, _ = self._probe("avatar.status", {}, 5)
        pu = lab_parity.puppet_refs(av)
        anim, why = self._probe("anim.probe", {"refs": ["player"] + pu, "near": {"radius": self.PARITY_RADIUS}, "bones": False, "max": self.PARITY_MAX}, 15)
        if anim is None:
            raise Blocked(f"anim.probe is not available on the {self.name} copy ({why}); the lab starts it with dev.animprobe=1", "BUILD")
        refs = ["player"] + [str(a.get("ref")) for a in anim.get("actors") or [] if not a.get("player")]
        look, lwhy = self._probe("parity.look", {"refs": refs}, 15)
        peek, vats = "", None
        if look is None:      # a build without the probe: the multiplier through world.peek, the VATS mode through vats.state
            pk, _ = self._probe("world.peek", {"addr": "0x%X" % lab_parity.TIME_ADDR_CURRENT, "len": 8}, 5)
            peek = str((pk or {}).get("hex") or "")
            vs, _ = self._probe("vats.state", {}, 5)
            vats = (vs or {}).get("mode")
        snap = lab_parity.assemble(anim, look, av, peek, vats)
        if look is None:
            snap["missing"][0] = f"parity.look ({lwhy}): equipment and look of actors"
        return snap

    def _puppet_ref(self) -> Optional[str]:
        t, ref = self._puppet_cache
        if time.monotonic() - t > 2.0:
            av, _ = self._probe("avatar.status", {}, 5)
            refs = lab_parity.puppet_refs(av)
            ref = refs[0] if refs else None
            self._puppet_cache = (time.monotonic(), ref)
        return ref

    def anim_category(self, who: Optional[str] = None) -> Optional[str]:
        ref = "player" if who is None else (self._puppet_ref() if who == "puppet" else self.bot._norm(who)[2])   # noqa: SLF001
        if not ref:
            return None
        anim, _ = self._probe("anim.probe", {"refs": [ref], "bones": False}, 5)
        acts = (anim or {}).get("actors") or []
        return lab_parity.primary_category(acts[0]) if acts else None

    def _line(self, cmd: str, ref: str) -> Tuple[bool, str]:
        r = self.nv.exec(cmd, ref=self.bot._ref_of(ref))   # noqa: SLF001
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def npc_equip(self, ref: str, item: str) -> Tuple[bool, str]:
        """`AddItem` then `EquipItem` on the NPC (console lines run with the NPC as the calling reference; only its owner's lines stick). UNVERIFIED."""
        h = self.bot._item(item)   # noqa: SLF001
        ok, why = self._line(f"AddItem {h} 1", ref)
        return self._line(f"EquipItem {h}", ref) if ok else (False, "AddItem: " + why)

    def npc_draw(self, ref: str, on: bool = True) -> Tuple[bool, str]:
        """The dev method anim.npc.do op=weapon (plugin/actors/anim: BaseProcess::SetWeaponOut through a pinned slot)."""
        r, why = self._probe("anim.npc.do", {"ref": self.bot._norm(ref)[2], "op": "weapon", "on": bool(on)}, 10)   # noqa: SLF001
        if r is None:
            return False, f"anim.npc.do: {why} (needs [dev] animprobe=1)"
        return r.get("result") == "ok", "" if r.get("result") == "ok" else f"anim.npc.do weapon: {r.get('result')}"

    def npc_sit(self, ref: str, furniture: str) -> Tuple[bool, str]:
        """`<furniture>.Activate <npc>`: the furniture is the calling reference, the NPC the activator. UNVERIFIED (the engine may want the NPC's AI package to agree)."""
        return self._line(f"Activate {self.bot._norm(ref)[2]}", furniture)   # noqa: SLF001

    def npc_stand(self, ref: str) -> Tuple[bool, str]:
        """`EvaluatePackage` on the NPC: it drops what it is doing and picks its package again (the patrol), which makes it leave the seat. UNVERIFIED."""
        return self._line("EvaluatePackage", ref)

    def npc_reset(self, ref: str, item: str) -> Tuple[bool, str]:
        h = self.bot._item(item)   # noqa: SLF001
        self.npc_draw(ref, False)
        self._line(f"UnequipItem {h}", ref)
        return self._line(f"RemoveItem {h} 1", ref)

    def unarm(self) -> Tuple[bool, str]:
        """Holster and unequip the lab's pistol: the next Attack is a punch."""
        r = self.bot.unequip(self.weapon)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def punch(self, n: int = 3, gap_s: float = 0.8) -> Tuple[bool, str]:
        """n Attack pulses through the bridge's input driver (bot.fire) with no weapon equipped. UNVERIFIED: the driver was built for guns."""
        for i in range(max(1, int(n))):
            r, why = self._probe("bot.fire", {"shots": 1}, 10)
            if r is None:
                return False, f"bot.fire: {why}"
            time.sleep(gap_s)
        return True, ""

    # ---- moving
    def goto_ref(self, ref: str, dist: float = 0.0) -> Tuple[bool, str]:
        r = self.bot.goto(ref, radius=100.0, mode="teleport")
        if not r.ok:
            return False, str(r)[:WHY]
        if dist:
            x, y, z = self.pos()
            return self.goto_pos(x + dist, y, z)
        return True, ""

    def goto_pos(self, x: float, y: float, z: float) -> Tuple[bool, str]:
        r = self.bot.goto_pos(x, y, z, mode="teleport")
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def coc(self, cells: List[str]) -> Tuple[bool, str]:
        try:
            coc_into(self.nv, cells)
            return True, ""
        except Blocked as e:
            return False, str(e)[:WHY]

    def walk_to(self, x: float, y: float, z: float, timeout_s: float = 30.0, tick: Optional[Callable[[], None]] = None) -> Tuple[bool, str]:
        """Real locomotion through the input driver when the build has one (the bot decides), else stepped SetPos. `tick` runs on this thread
        every 0.25 s while the walk goes on in a worker; the bridge serialises the calls of the two threads."""
        res: Dict[str, Any] = {}

        def work():
            try:
                res["r"] = self.bot.goto_pos(x, y, z, timeout_s=timeout_s, mode="walk")
            except BaseException as e:   # noqa: BLE001 - handed to the caller's thread below (a bridge error must not vanish with the worker)
                res["exc"] = e
        th = threading.Thread(target=work, daemon=True)
        th.start()
        while th.is_alive():
            if tick:
                tick()
            time.sleep(0.25)
        th.join(1.0)
        if "exc" in res:
            if type(res["exc"]).__name__ in ("ConnectionLost", "GameCrashed"):
                raise res["exc"]
            return False, f"the walk raised {type(res['exc']).__name__}: {str(res['exc'])[:WHY]}"
        r = res.get("r")
        # NOT `r and r.ok`: a failed ActionResult is falsy (its __bool__ is ok), which once hid every walk failure behind "gave no result"
        if r is None:
            return False, "walk thread gave no result"
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    # ---- combat
    def arm(self) -> Tuple[bool, str]:
        """Health 5000 (a fight cannot end the lab), ammo, the 9mm pistol equipped. UNVERIFIED console lines from scenarios/goodsprings_enemies.json."""
        try:
            for line in ("player.setav Health 5000", f"player.additem {self.bot._norm(self.ammo)[1]} 300"):   # noqa: SLF001
                self.nv.exec(line)
            self.bot.ammo_item = self.ammo
            r = self.bot.give(self.weapon, 1)
            if not r.ok:
                return False, "give weapon: " + str(r)[:WHY]
            r = self.bot.equip(self.weapon)
            return bool(r.ok), "" if r.ok else "equip: " + str(r)[:WHY]
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, str(e)[:WHY]

    def enable(self, ref: str) -> Tuple[bool, str]:
        """`ref.Enable` through exec with the ref as the calling reference (the lab's hostiles are placed disabled). Verified as a console line by the lab plugin author."""
        r = self.nv.exec("enable", ref=self.bot._ref_of(ref))   # noqa: SLF001
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def disable(self, ref: str) -> Tuple[bool, str]:
        r = self.nv.exec("disable", ref=self.bot._ref_of(ref))   # noqa: SLF001
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def kill(self, ref: str) -> Tuple[bool, str]:
        r = self.nv.exec("KillActor", ref=ref)
        return bool(r.get("ok")), "" if r.get("ok") else str(r.get("output", ""))[:WHY]

    def step_back(self, ref: str, stand: Optional[str] = None, dist: float = STANDOFF) -> Tuple[bool, str]:
        """Hop (teleport) to a spot `dist` units from `ref`: the stand marker only when it is itself a good shooting spot for where the target IS now (SHOT_MIN + 50 to
        SHOT_MAX - 50 units from it), else on the line from the ref through the player. Closer than SHOT_MIN the shooter is too near (aim_at refuses under 100 units: a player
        standing inside a target aims at the floor); farther than SHOT_MAX the pistol is out of range. A target that drifted off (659 and 875 units from the stand marker in the
        lab run of 2026-10-28, behind bad_wifi and transatlantic) used to count the stand marker as 'far enough' and the shooter hopped to a spot as far away as before."""
        tp = self.pos(ref)
        me = self.pos()
        if stand and K.MUTANT != "shooter_hops_to_far_stand":        # twin: the stand marker is taken whenever it is 0.8 x the standoff from the target, however far
            try:
                sp = self.pos(stand)
                if SHOT_MIN + 50.0 <= math.hypot(sp[0] - tp[0], sp[1] - tp[1]) <= SHOT_MAX - 50.0:
                    return self.goto_pos(sp[0], sp[1], me[2])
            except Exception as e:   # noqa: BLE001
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
        elif stand:
            try:
                sp = self.pos(stand)
                if math.hypot(sp[0] - tp[0], sp[1] - tp[1]) >= 0.8 * dist:
                    return self.goto_pos(sp[0], sp[1], me[2])
            except Exception as e:   # noqa: BLE001
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
        dx, dy = me[0] - tp[0], me[1] - tp[1]
        n = math.hypot(dx, dy) or 1.0
        if n < 1.0:
            dx, dy, n = 0.0, -1.0, 1.0
        return self.goto_pos(tp[0] + dx / n * dist, tp[1] + dy / n * dist, me[2])

    def set_health(self, ref: str, hp: float) -> Tuple[bool, str]:
        """`SetAV Health <hp>` on the ref (the owner of the actor, the host, does it; tools/bench/real_stage.py weakens a gecko the same way)."""
        r = self.nv.exec(f"SetAV Health {hp:g}", ref=self.bot._ref_of(ref))   # noqa: SLF001
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def hold(self, ref: str) -> Tuple[bool, str]:
        """`SetRestrained 1` on the actor: it stays where it stands. An enabled gecko ran 368 units to the player in about 4 s (measured 2026-10-27) and stood
        on top of it, out of view and inside aim_at's 100 units, when the VATS shot began (`vats_not_entered`, then `attack_not_queued (hit chance 0.0)`);
        a restrained one stays at the standoff and takes any number of VATS shots."""
        r = self.nv.exec("SetRestrained 1", ref=self.bot._ref_of(ref))   # noqa: SLF001
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    def line_of_fire(self, ref: str) -> Tuple[bool, str]:
        """(clear, words): the target is between SHOT_MIN and SHOT_MAX units away and this copy's player has line of sight to it (the console GetLOS
        condition; when the build cannot answer it, only the distance counts). `words` says what was seen either way."""
        d = math.dist(self.pos(ref), self.pos())
        los: Optional[bool] = None
        try:
            los = bool(self.nv.eval(f"GetLOS {self.bot._norm(ref)[1]}", ref="player"))   # noqa: SLF001
        except Exception as e:   # noqa: BLE001 - the expression may not exist in this build: then only the distance is judged
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
        words = f"{d:.0f} units, line of sight {'unknown' if los is None else ('yes' if los else 'NO')}"
        return (SHOT_MIN <= d <= SHOT_MAX and los is not False), words

    def _shot_note(self, ref: str, r) -> str:
        """What a shot that hit nothing leaves to know: the target's health and distance, and what bot.fire said (ammo, weapon drawn)."""
        try:
            note = f" (target health {self.health(ref):.0f}, {self.line_of_fire(ref)[1]}; last fire: {str(r)[:WHY]})"
            # Did any bullet reach the hit funnel at all? engineHits 0 after 24 volleys = the shots hit nothing (the target has no 3D or collision here, or is not where it is
            # drawn): not a sync fault. engineHits > 0 with clamped / neutralized / hitsSent says where the hit went (Goodsprings story S15 and the lab's fight room).
            note += f" [{self.name} combat: {self.combat_words()}]"
            try:   # the target as the actor port reads it (3D loaded, disabled, life state), when the dev method is on
                raw = self.nv.call("actors.read", {"ref": self.bot._norm(ref)[0]}, timeout_s=5)   # noqa: SLF001
                note += f" [target has3D={raw.get('has3D')} disabled={raw.get('disabled')} lifeState={raw.get('lifeState')} cell={raw.get('cell')}]"
            except Exception as e:   # noqa: BLE001
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
            return note
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return ""

    def shoot(self, ref: str, volleys: int = 12, stand: Optional[str] = None) -> Tuple[bool, str]:
        """A real shot, as a player kills (a guest's console kill is refused on purpose): aim and fire until this copy shows the ref dead.
        Before every volley the distance (SHOT_MIN..SHOT_MAX) and the line of sight are checked; a shooter that is too close, too far or blocked steps to
        a spot STANDOFF away on the line from the ref through the stand marker (see step_back) and aims again from there."""
        why = "no volley fired"
        hops = 0
        drawn = self.ready_weapon()           # a talk or a scene leaves the weapon holstered (and the fighting controls off)
        for _ in range(volleys):
            if not self.alive(ref):
                return True, ""
            clear, words = self.line_of_fire(ref)
            if not clear:
                # Re-approach: a target that is out of range (or hidden) is walked up to along the line to it, nearer on each try (350, 275, 200 units), instead of
                # hopping to the same far spot again and again until the volleys run out.
                ok, hop = self.step_back(ref, stand, max(SHOT_MIN + 50.0, STANDOFF - 75.0 * min(hops, 2)))
                hops += 1
                why = f"not a clear shot ({words}); stepped to the standoff: {'ok' if ok else hop}"
                time.sleep(0.3)
                clear, words = self.line_of_fire(ref)
                if not clear:
                    why = f"no clear shot even from the standoff ({words}); {hops} approach(es)"
                    continue
            r = self.bot.fire(ref)
            if not r.ok and r.code == "too_close":
                ok, hop = self.step_back(ref, stand)
                why = f"the shot failed: {str(r)[:WHY]}; stepped back: {'ok' if ok else hop}"
                time.sleep(0.3)
                continue
            if not r.ok and "weapon still holstered" in str(r):
                drawn = self.ready_weapon()
                why = f"the shot failed: {str(r)[:WHY]}; weapon drawn again: {'yes' if drawn else 'NO (controls switched on and Ready tapped)'}"
                time.sleep(0.3)
                continue
            if not r.ok and "dead" not in str(r):
                why = f"the shot failed: {str(r)[:WHY]}"
                time.sleep(0.3)
                continue
            t0 = time.time()
            while time.time() - t0 < 1.5:
                if not self.alive(ref):
                    return True, ""
                time.sleep(0.1)
            why = f"{volleys} volleys did not kill it"
            if r.ok:
                why += self._shot_note(ref, r)
        return (not self.alive(ref)), why

    def draw_weapon(self) -> bool:
        """Take the equipped weapon out (the Ready control is a toggle, so only when it is holstered). True when it is out afterwards."""
        try:
            if self.bot._weapon_note().get("weapon_out") is not False:   # noqa: SLF001 (True, or the game could not say)
                return True
            self.nv.call("bot.input", {"tap": ["ReadyItem"]})
            time.sleep(1.2)
            return bool(self.bot._weapon_note().get("weapon_out"))       # noqa: SLF001
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False

    def free_controls(self) -> Tuple[bool, str]:
        """`player.EnablePlayerControls` (every control back on; a local-player line, so a guest's gate lets it through). Quest scripts switch controls off around their scenes
        (the Goodsprings intro, a talk that just closed); with the fighting controls off our input pulses (Attack, Ready) are dropped inside the game and the weapon stays
        holstered: story run 20261028-023656, steps S05 and S15, 'weapon still holstered after the shots, the pulses never reached the game'."""
        try:
            r = self.nv.exec("player.EnablePlayerControls")
            return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, str(e)[:WHY]

    def ready_weapon(self) -> bool:
        """The weapon is out for a shot: drawn at once, or after the controls were switched on again. A draw that works is left alone (the Ready control is a toggle)."""
        if self.draw_weapon():
            return True
        if K.MUTANT == "shoot_no_controls_retry":      # twin: a holstered weapon is not helped by switching the controls on
            return False
        self.free_controls()
        time.sleep(0.3)
        return self.draw_weapon()

    def _health_or_none(self, ref: str) -> Optional[float]:
        try:
            return self.health(ref)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return None

    def vats(self, ref: str, limb: Optional[str] = None, stand: Optional[str] = None) -> Tuple[bool, str]:
        """Real-time VATS on `ref`. No limb by default: plugin/bots answers any limb with "unsupported: limb selection in VATS is not implemented"
        (bot_engine.h VatsJob::start); the engine picks the part itself, which is what run_vats.py's job does too."""
        # VATS only opens with the weapon out and the target in view ("vats_not_entered (no weapon, no target in view, or no AP)", first in-game runs:
        # weapon holstered): draw it and face the target first, as a player does
        # A hostile enabled a moment ago has no 3D yet and is not "in view": VATS then stays closed (vats_not_entered, lab run 2026-10-27). Face it, give the
        # engine a moment and try again, up to 3 times; any other refusal is final.
        # A hostile that has walked up to the player (aim_at refuses under 100 units) is not in view either: step back to the standoff first, as shoot() does.
        r = None
        # Two VATS entries close together crashed the game twice in the lab (FalloutNV.exe+400F66, a null pointer in a menu update, always in the first ticks of an
        # entry right after "tutorial -> zoom", host on its 4th shot, client right after the host's): the entry waits until the last one is a few seconds over.
        wait = VATS_COOLDOWN_S - (time.monotonic() - self._vats_done_at) if K.MUTANT != "vats_no_cooldown" else 0.0     # twin: no pause between entries
        if wait > 0:
            time.sleep(wait)
        hp0 = self._health_or_none(ref)
        for attempt in range(3):
            try:
                if math.dist(self.pos(ref), self.pos()) < SHOT_MIN:
                    self.step_back(ref, stand)
            except Exception as e:   # noqa: BLE001
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
            self.draw_weapon()
            self.bot.aim_at(ref)
            r = self.bot.vats_target(ref, limb.capitalize() if limb else None)
            if r.ok or "vats_not_entered" not in str(r):
                break
            time.sleep(1.5)
        self._vats_done_at = time.monotonic()
        if not r.ok and "vats_closed" in str(r):
            # The mod's real-time VATS fires the queued attack by itself (the vats module's auto fire); when it wins the race against the job's own accept the
            # camera is back at mode 0 before the job saw the queue, and the job calls a shot that landed a failure (measured 2026-10-27: target 4.8 -> -4.4,
            # dead, job FAILED vats_closed). What the room needs is the shot, and the target says whether it landed.
            try:
                if K.MUTANT == "vats_closed_unchecked" or (hp0 is not None and (not self.alive(ref) or self.health(ref) < hp0)):   # twin: believed without looking at the target
                    return True, ""
            except Exception as e:   # noqa: BLE001
                if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                    raise
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    # ---- loot, doors, terminals
    def can_take(self) -> Tuple[bool, str]:
        """The host takes with console lines. A guest's console RemoveItem on a container is dropped on purpose (measured 2026-10-27: the chest stayed at 3 on both copies
        for 7 s while the guest's own AddItem went through), and no bridge method clicks a line of the container menu: the menu's transfer call is not pinned anywhere in
        plugin/bots, and a take done while the menu is open behind the loot service's back is healed away at the close (measured 2026-10-28: guest 2, host 3). So a guest takes
        the way a quick-loot style mod does and the loot service's menu-less path is built for (docs/tools/loot.md item 13): the chest is closed, the item leaves the container
        and enters the player's inventory through `coop.loot grab` (a test helper: RemoveItem on the container with the guest gate lifted, AddItem on the player), the service
        scans the difference and runs open (lock), take, close with the host's answer deciding. It needs NVCOOP_TEST_HELPERS=1; without it the take is skipped with that reason."""
        if self.role == HOST:
            return True, ""
        if os.environ.get("LAB_GUEST_TAKE", "") not in ("helper", "menu"):
            return False, ("a guest has no menu click and the helper path (coop.loot grab) did not reach the host when measured on 2026-10-28: guest 2, host 3 after 10 s, the guest's loot "
                           "service listed 0 containers and polled 0 times (docs/tools/lab.md); run with LAB_GUEST_TAKE=helper (lab_run.ps1 -GuestTake helper) to make the check")
        try:
            self.nv.call("coop.loot", {"grab": {"container": "0", "item": "0", "count": 0}}, timeout_s=10)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            t = str(e)
            if "test helpers disabled" in t:
                return False, "this game copy was started without NVCOOP_TEST_HELPERS=1, so the guest's take helper (coop.loot grab) is off"
            if "grab refused" in t or "bad item" in t:
                return True, ""             # the helper answered: it refused the empty probe, as it should
            return False, f"the guest's take helper coop.loot grab is not usable here: {t[:WHY]}"
        return True, ""

    take_note = ""

    def take(self, container: str, item: str, n: int = 1) -> Tuple[bool, str]:
        if self.role == HOST:
            r = self.bot.take(container, item, n)
            return bool(r.ok), "" if r.ok else str(r)[:WHY]
        return self._take_menuless(container, item, n)

    def _loot_status(self) -> str:
        try:
            return json.dumps(self.nv.call("coop.loot", {}, timeout_s=10), default=str)[:300]
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return f"(no loot status: {str(e)[:60]})"

    def _take_menuless(self, container: str, item: str, n: int) -> Tuple[bool, str]:
        """A guest's take through the helper with the container closed (see can_take). A container menu left open by an earlier step is closed first."""
        self.close_menus()
        before = self.item_count(item)
        menu_mode = K.MUTANT == "guest_take_menu_opened" or os.environ.get("LAB_GUEST_TAKE") == "menu"     # twin / experiment: the item moves with the container menu open
        if menu_mode:
            self.bot.open(container)
        try:
            self.nv.call("coop.loot", {"grab": {"container": self.bot._norm(container)[1], "item": self.bot._norm(item)[1], "count": int(n), "phase": "both"}},   # noqa: SLF001
                         timeout_s=10)
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, f"coop.loot grab: {str(e)[:WHY]}"
        end = time.time() + 3.0
        while self.item_count(item) < before + n and time.time() < end:
            time.sleep(0.1)
        got = self.item_count(item) - before
        if menu_mode:
            time.sleep(float(os.environ.get("LAB_GUEST_TAKE_HOLD_S", "2")))      # let the loot service's menu watcher notice the change before the menu closes
            self.take_note = "loot status with the menu open: " + self._loot_status()
            self.bot.close_menu("container", timeout_s=3.0)
        else:
            self.take_note = "loot status after the take: " + self._loot_status()
        return (got >= n), "" if got >= n else f"the guest's inventory gained {got} of {n}"

    def pickup(self, ref: str) -> Tuple[bool, str]:
        # The host activates through the console line, the engine's own Activate(activator, 0, 0, 1). The bridge method activate.ref passes 0 as the last
        # argument, and for the lab's placed Stimpak the engine then deletes the ref and gives the player nothing (measured 2026-10-27: host and client, the
        # loot sync untouched; the console line gave the Stimpak). A guest's console line is dropped by the script block, so a guest uses the bridge method.
        if self.role == HOST and K.MUTANT != "host_pickup_via_bridge":    # twin: the host's pickup goes through activate.ref too
            r = self.bot.pickup(ref, console=True)
        else:
            r = self.bot.pickup(ref)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def close_menus(self) -> Tuple[bool, str]:
        from bots.primitives import MENU_FREE
        for _ in range(3):                               # a hint box can sit on top of another menu
            if (self.bot.menu_info() or {}).get("kind") in MENU_FREE:
                return True, ""
            self.bot.close_menu("any", timeout_s=2.0)
        return (self.bot.menu_info() or {}).get("kind") in MENU_FREE, ""

    def open_door(self, ref: str) -> Tuple[bool, str]:
        r = self.bot.open_door(ref)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def close_door(self, ref: str) -> Tuple[bool, str]:
        r = self.bot.close_door(ref)
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    def use_terminal(self, ref: str, entry: int) -> Tuple[bool, str]:
        """Open the terminal, close the first-use hint box over it, click the entry (its result script runs), then leave the terminal menu: an open
        menu eats the next door activation of this copy."""
        r = self.bot.use_terminal(ref, entry)
        if r.ok:
            time.sleep(0.5)                               # the entry's result script ran with the click; the pause lets its result text show
            self.bot.close_menu("any", timeout_s=3.0)     # best effort: a menu that stays is reported by the next step with the menu named
        return bool(r.ok), "" if r.ok else str(r)[:WHY]

    # ---- quest
    def set_stage(self, quest: str, stage: int) -> Tuple[bool, str]:
        r = self.nv.exec(f"setstage {self.bot._norm(quest)[1]} {int(stage)}")   # noqa: SLF001 (as scenlib's setstage op)
        return bool(r.get("ok", True)), "" if r.get("ok", True) else str(r.get("output", ""))[:WHY]

    var_why = ""

    def var(self, quest: str, name: str) -> Optional[float]:
        """The live value of a quest script variable on this copy through `bot.questvar` (plugin/bots, nvhooks::questVarValue). None when the copy cannot read it (a build
        without the method, no script, the script never ran, no such variable); `var_why` then says why, and the quest room skips the check with that reason."""
        self.var_why = ""
        try:
            key = (quest, name)
            if K.MUTANT == "quest_var_cached" and key in self._var_seen:         # twin: the first value read is kept (the client would "see" 0 forever)
                return self._var_seen[key]
            r = self.nv.call("bot.questvar", {"quest": self.bot._norm(quest)[0], "var": name}, timeout_s=10)   # noqa: SLF001
            self._var_seen.setdefault(key, float(r["value"]))
            return float(r["value"])
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            self.var_why = f"bot.questvar: {str(e)[:WHY]}"
            return None

    def set_var(self, quest: str, name: str, value: float) -> Tuple[bool, str]:
        try:
            self.nv.call("test.quest.setvar", {"quest": self.bot._norm(quest)[1], "var": name, "value": float(value)})   # noqa: SLF001
            return True, ""
        except Exception as e:   # noqa: BLE001
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            return False, f"test.quest.setvar: {str(e)[:WHY]} (needs NVCOOP_TEST_HELPERS=1)"


def coc_into(nv, names: List[str], wait_s: float = 90.0) -> None:
    """`coc <name>` on one copy: the first editor id that the console accepts, retried while a cell load keeps the main thread busy, then the load
    notices are dismissed. Blocked (class ENV) when none works within `wait_s`."""
    end = time.time() + wait_s
    last: Any = None
    k = 0
    while time.time() < end:
        try:
            nv.eval("GetGameHour")
            out = nv.exec(f"coc {names[k]}")
            if out.get("ok", True):
                break
            last = out
            k = min(k + 1, len(names) - 1)
        except Exception as e:   # noqa: BLE001 - "main thread did not service the request" while a cell loads
            if type(e).__name__ in ("ConnectionLost", "GameCrashed"):
                raise
            last = e
        time.sleep(2)
    else:
        raise Blocked(f"coc {'/'.join(names)} was refused: {str(last)[:WHY]}", "ENV")
    from bots import Bot
    Bot(nv).dismiss_messages(quiet_s=2, max_s=20, max_n=6)


# ------------------------------------------------------------------ the live environment
def _scenario_runner():
    """tools/scenarios/runner.py by path: `import runner` would find the folder tools/runner first."""
    import importlib.util
    spec = importlib.util.spec_from_file_location("scen_runner", ROOT / "tools" / "scenarios" / "runner.py")
    mod = importlib.util.module_from_spec(spec)
    sys.modules["scen_runner"] = mod
    spec.loader.exec_module(mod)
    return mod


def _rel(p: Path) -> str:
    try:
        return Path(p).relative_to(ROOT).as_posix()
    except ValueError:
        return Path(p).as_posix()


def sha1(p: Path) -> str:
    return hashlib.sha1(p.read_bytes()).hexdigest()


class _Borrowed:
    """`with` on a bridge the caller keeps: chaos_pair opens a bridge per call and closes it; the lab's pair connections stay open."""

    def __init__(self, nv):
        self.nv = nv

    def __enter__(self):
        return self.nv

    def __exit__(self, *a):
        return False


class LiveEnv:
    """Pairs of real copies. The same interface as labfake.FakeEnv."""
    live = True

    def __init__(self, layout, args, log: Callable[[str], None] = lambda s: None, lab_dir: Optional[Path] = None):
        import os
        os.environ.setdefault("NVCOOP_TEST_HELPERS", "1")     # the lab's copies are test copies: test.quest.setvar and the guest's menu-open take (coop.loot grab) need it
        import prove                                  # tools/proof/prove.py: the slot, the guard, provisioning, boot, the crash watch
        self.layout, self.args, self.log = layout, args, log
        self.lab_dir = Path(lab_dir or ROOT / "out" / "lab")
        prove.OWNER = "lab"                            # its own slot lease and pid registry: a proof queue may run beside a lab run
        prove.OUT = self.lab_dir / "state"
        prove.OUT.mkdir(parents=True, exist_ok=True)
        self.prove = prove
        outer = self

        class _Env(prove.LiveEnv):
            def _boot(self, i, env_extra):
                bad = outer.install(i)
                return bad or super()._boot(i, env_extra)
        self.pv = _Env(SimpleNamespace(provision=True, boot=True, pairs=int(getattr(args, "pairs", 1) or 1), reserve_gb=float(getattr(args, "reserve_gb", 1.5))))
        self.nvs: Dict[int, Any] = {}
        self.held: set = set()
        self.t_start = time.time()
        self.entered: Dict[int, bool] = {}
        self.net_prof: Dict[int, Tuple[str, int]] = {}       # pair -> (chaos profile, seed) of the network pass that is running
        self.chaos: Dict[int, Any] = {}                     # pair -> the chaos_pair.ChaosPair whose relay the client joined through
        self.install_script = ROOT / "tools" / "lab" / "esp" / "install_lab.py"
        self.esp = self.lab_dir / layout.plugin

    def build_id(self) -> str:
        return self.prove.build_id()

    def pairs_allowed(self, n: int) -> Tuple[int, str]:
        import pf_sched as X
        cap = min(n, X.MAX_PAIRS)
        k, note = self.pv.pairs_allowed(cap)
        why = note or (f"PROOF_MAX_PAIRS allows {X.MAX_PAIRS} pair(s)" if cap < n else "")
        return k, why

    # ---- the lab plugin
    def install(self, i: int) -> Optional[str]:
        """Put the lab plugin into copy i (before it boots). A reason when that cannot be done."""
        if not self.install_script.is_file():
            return f"{_rel(self.install_script)} does not exist (the plugin generator in tools/lab/esp is missing or not merged)"
        d = self.pv.idir(i)
        cp = subprocess.run(PY + [str(self.install_script), "--instance-dir", str(d)], capture_output=True, text=True, cwd=ROOT)
        if cp.returncode:
            return f"install_lab.py failed for copy {i}: {(cp.stdout + cp.stderr).strip()[-200:]}"
        return None

    def stale_copies(self, instances: List[int]) -> List[int]:
        """Running copies whose installed lab plugin differs from out/lab's: only a restart loads the new one."""
        out = []
        for i in instances:
            f = self.pv.idir(i) / "Data" / self.layout.plugin
            if self.pv.up(i) and f.is_file() and self.esp.is_file() and sha1(f) != sha1(self.esp):
                out.append(i)
        return out

    def prepare(self, pair: int) -> Optional[Tuple[str, str]]:
        """Provision, install the plugin, boot and wait until both copies serve requests. -> None, or (class, reason)."""
        ins = [2 * pair, 2 * pair + 1]
        if not self.install_script.is_file():
            return "BLOCKED", self.install(ins[0]) or "install_lab.py missing"
        test = {"id": f"lab{pair}", "copies": 2, "pair": pair, "n": 1, "ini": LAB_INI, "timeout_s": 1800, "snapshot": [], "commands": [],
                "requires_methods": ["coop.status", "coop.owner", "bot.status", "avatar.status"], "also_roadmap": []}
        self.pv.hold(ins)                                  # the RAM guard and the slot count every pair that runs beside this one
        self.held.update(ins)
        why = self.pv.preflight(test)
        if why:
            return "ENV", why
        stale = self.stale_copies(ins)
        if stale:
            self.log(f"restarting copies {stale}: they run another lab plugin")
            self.pv.finish(True, stale)
            t0 = time.time()
            while any(self.pv.up(i) for i in stale) and time.time() - t0 < 30:
                time.sleep(1)
        env_reason, build_reason, _fresh = self.pv.prepare(test, ins)
        if env_reason:
            return "ENV", env_reason
        if build_reason:
            return "BLOCKED", build_reason
        import nvbridge
        for i in ins:
            if i in self.nvs:
                continue
            self.nvs[i] = nvbridge.NvBridge.connect(i, wait_s=30)
        for i in ins:
            plugs = [p.lower() for p in self.nvs[i].plugins()]
            if self.layout.plugin.lower() not in plugs:
                return "BLOCKED", (f"copy {i} runs without {self.layout.plugin} (loaded: {len(plugs)} plugins). It was started before the plugin was installed: "
                                   f"python -I tools/lab/lab.py down, then run again")
        return None

    def copy_info(self) -> List[Dict[str, Any]]:
        """What can slow the copies down, read from each copy that is up (results.json fps.provision): its process's power throttling state, the window options of the
        bridge ([bridge] minimized, focusGuard) and the frame-rate keys of its ini files. Best effort: a key it cannot read is left out."""
        out: List[Dict[str, Any]] = []
        try:
            import winproc
        except ImportError:
            winproc = None
        owned = self.pv.owned.load()
        for i in sorted(self.held):
            d = self.pv.idir(i)
            info: Dict[str, Any] = {"instance": i}
            pid = (owned.get(str(i)) or {}).get("pid")
            if pid and winproc:
                info["pid"] = int(pid)
                info["throttle"] = winproc.throttle_state(int(pid))
            try:
                inst = self.pv.inst
                plug = (d / "Data" / "NVSE" / "Plugins" / "nvcoop.ini").read_text(encoding="mbcs", errors="replace")
                info["bridge"] = {k: inst.ini_get(plug, "bridge", k) for k in ("minimized", "focusGuard")}
                fo = (d / "Fallout.ini").read_text(encoding="latin-1")
                pr = (d / "FalloutPrefs.ini").read_text(encoding="latin-1")
                info["ini"] = {"iFPSClamp": inst.ini_get(pr, "General", "iFPSClamp") or inst.ini_get(fo, "General", "iFPSClamp"),
                               "iPresentInterval": inst.ini_get(pr, "Display", "iPresentInterval"), "bAlwaysActive": inst.ini_get(fo, "General", "bAlwaysActive"),
                               "iSize": f"{inst.ini_get(pr, 'Display', 'iSize W')}x{inst.ini_get(pr, 'Display', 'iSize H')}"}
            except Exception:   # noqa: BLE001
                pass
            out.append(info)
        return out

    def copies_running(self) -> int:
        """How many game copies of this run are up right now (every pair's two; the fps of one is a share of one PC)."""
        return sum(1 for i in sorted(self.held) if self.pv.up(i))

    def machines(self, pair: int):
        return (LiveMachine(self.nvs[2 * pair], HOST), LiveMachine(self.nvs[2 * pair + 1], CLIENT))

    def clock(self, pair: int):
        from lab_core import RealClock
        return RealClock()

    def probe(self, pair: int) -> Callable[[], List[str]]:
        names = {2 * pair: HOST, 2 * pair + 1: CLIENT}
        return lambda: [names[e["instance"]] for e in self.pv.crashes([2 * pair, 2 * pair + 1]) if e.get("instance") in names]

    def crash_info(self, pair: int) -> List[str]:
        """One line per copy of the pair that exited: instance, exit code, faulting module (pf_crash.collect reads the crash log and the minidump)."""
        import pf_crash
        return [pf_crash.one_line(self.pv.crash_info(i, self.t_start, None)) for i in (2 * pair, 2 * pair + 1) if self.pv.crashes([i])]

    # ---- into the lab cell
    def enter(self, pair: int) -> Callable[[], None]:
        """The hook the spawn room calls first: load the save once on both copies, wait for the session, then `coc NVCoopLab` on both. UNVERIFIED in game."""
        def go():
            scen_runner = _scenario_runner()
            nvs = [self.nvs[2 * pair], self.nvs[2 * pair + 1]]
            save = getattr(self.args, "save", "nvcoop_autosave_ready")
            r = scen_runner.Runner(nvs, prepared=False, log=self.log)
            r._load_start_save({"start": {"save": save}})                  # noqa: SLF001 (the scenario runner's own load: prepared saves, refusals)
            r._clear_load_messages()                                       # noqa: SLF001 (the DLC notices a load raises)
            # clients rejoined, a digest judged ok. The digest part is short on purpose: the spawn room judges the digest itself (a pair that never gets
            # an ok digest, first in-game run: SetGlobal 37 missing on the client, used 60 s of the room's time here before its first check)
            sync = r._wait_coop_synced(digest_timeout_s=ENTER_DIGEST_S, digest_stall_s=ENTER_DIGEST_S / 2)   # noqa: SLF001
            if sync and sync.get("synced") is False:
                raise Blocked(f"the pair did not sync after loading {save!r}: {sync.get('reason')}", "ENV")
            for k, nv in enumerate(nvs):
                self._coc(nv)
            self.entered[pair] = True
            if pair in self.net_prof:         # a network pass: the load above ran on the direct join; from here the client talks to the host through the relay
                self._net_attach(pair)
        return go

    # ---- the network pass: the client joins the host THROUGH the chaos relay (tools/chaos), with the profile of the pass
    def net_begin(self, pair: int, profile: str, seed: int = 7) -> Optional[str]:
        """Register the profile for the pair's next spawn (its enter hook attaches the relay after the save is loaded and the lab entered). A reason when it cannot run."""
        try:
            self._chaos_modules()
        except ImportError as e:
            return f"tools/chaos is not usable: {e}"
        self.net_prof[pair] = (profile, int(seed))
        return None

    def _chaos_modules(self):
        d = str(ROOT / "tools" / "chaos")
        if d not in sys.path:
            sys.path.append(d)               # last: tools/chaos has a client.py that must not shadow another client
        import chaos_pair   # noqa: WPS433
        import relay        # noqa: WPS433
        return chaos_pair, relay

    def chaos_factory(self, pair: int, seed: int):
        """The ChaosPair of this pair: relay in front of the host's real port (read from the host copy's nvcoop.ini), joined by the client copy's bridge."""
        chaos_pair, relay = self._chaos_modules()
        host_i, client_i = 2 * pair, 2 * pair + 1
        ini = self.pv.idir(host_i) / "Data" / "NVSE" / "Plugins" / "nvcoop.ini"
        return chaos_pair.ChaosPair(host_ini=lambda: ini.read_text(encoding="mbcs", errors="replace"), bridge=lambda: _Borrowed(self.nvs[client_i]),
                                    relay_factory=lambda port: relay.Relay((chaos_pair.LOOPBACK, port), 0, "lan", seed, None).start(), join_wait_s=60.0)

    def _net_attach(self, pair: int) -> None:
        profile, seed = self.net_prof[pair]
        cp = self.chaos_factory(pair, seed)
        try:
            cp.setup()
        except Exception as e:   # noqa: BLE001 - SetupError (the client did not join through the relay) or a bridge error
            cp.teardown()
            raise Blocked(f"the client could not join through the chaos relay: {str(e)[:300]}", "ENV") from e
        ok, why = lab_net.through_relay(cp.relay.get_stats())
        if not ok and K.MUTANT != "net_bypass_unchecked":     # twin: a client that bypassed the relay would pass every network room
            cp.teardown()
            raise Blocked(f"the client reports joined but nothing crossed the relay ({why}): the network pass would measure the clean link", "ENV")
        cp.relay.set_profile(profile, seed)
        self.chaos[pair] = cp
        self.log(f"pair {pair}: the client is joined through the chaos relay, profile {profile}")

    def net_end(self, pair: int) -> Optional[Dict[str, Any]]:
        """Read the relay's counters, put the client back on the host's real port and stop the relay. -> {"relay": stats} or None when no relay was up."""
        self.net_prof.pop(pair, None)
        cp = self.chaos.pop(pair, None)
        if cp is None:
            return None
        info: Dict[str, Any] = {}
        try:
            info["relay"] = cp.relay.get_stats()
        except Exception:   # noqa: BLE001
            pass
        cp.teardown()
        return info

    def _coc(self, nv) -> None:
        """`coc NVCoopLab` (verified on the second PC after loadSave); the stock cell name is the fallback if a build keeps the old editor id."""
        coc_into(nv, [self.layout.cell_edid] + ([self.layout.cell_alt] if getattr(self.layout, "cell_alt", "") else []))

    def reset(self, pair: int) -> Optional[Tuple[str, str]]:
        """A fresh lab for the next pass: both copies are stopped (only these two, only because this tool started them) and started again. The first pass's end state
        (doors open, hostiles dead, the quest at stage 30) needs a fresh load, and a second loadSave in the SAME game made the host reject the client's rejoin ("your mod
        list differs from the host's", measured on the second PC 2026-10-27: every pair, first network pass), so the lab restarts instead of reloading.
        -> None, or (class, reason) when the pair cannot be brought up again."""
        ins = [2 * pair, 2 * pair + 1]
        for i in ins:
            nv = self.nvs.pop(i, None)
            if nv is not None:
                try:
                    nv.close()
                except Exception:   # noqa: BLE001
                    pass
        self.pv.unhold(ins)
        self.held.difference_update(ins)
        self.pv.finish(True, ins)
        t0 = time.time()
        while any(self.pv.up(i) for i in ins) and time.time() - t0 < 60:
            time.sleep(1)
        self.entered.pop(pair, None)
        return self.prepare(pair)

    def finish(self) -> None:
        for pair in list(self.chaos):         # a client still joined through a relay would be left talking to a dead port
            self.net_end(pair)
        for nv in self.nvs.values():
            try:
                nv.close()
            except Exception:   # noqa: BLE001
                pass
        self.nvs.clear()
        self.pv.unhold(sorted(self.held))
        self.held.clear()
        if not getattr(self.args, "keep", False):
            self.pv.finish(True)
