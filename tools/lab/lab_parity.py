"""The pure part of the lab's parity room: what each copy SHOWS, as numbers, and the comparison of two copies (no game, no bridge).

    assemble(anim, look, avatar, peek_hex, vats_mode)   -> a snapshot of one copy: {"player", "puppets", "actors", "time", "missing"}
    compare_snapshots(host_snap, client_snap, seen)     -> (mismatches, stats): every field that differs, with both values and a likely source module
    attack_mismatch(entity, owner, owner_saw, copy_saw) -> the "puppet did not punch" finding
    ParityLog                                           -> collects the findings of the scenario steps for results.json

The inputs are the answers of existing bridge methods (anim.probe: weapon out, sit state, animation groups; avatar.status: the puppets; world.peek:
the time multiplier) and of the read-only method parity.look (plugin/probe/parity_probe.cpp: the worn list and the look of any actor, the time
multiplier and the VATS mode). A field a build cannot read is None in the view and is simply not compared; `missing` names what was not available.

A view (one actor on one copy):
    ref, base           hex ids ("0x0100086f")
    valid3d, dead       the engine had a 3D for it / it is dead (a dead actor is not compared)
    weaponOut, aiming   BaseProcess::IsWeaponOut / IsAiming
    weapon              the weapon the process holds (hex id, "none" for fists, None when unreadable)
    sit, pose           BaseProcess::GetSitSleepState and its class: stand | sit | sleep
    action, group, cat  the current animation action, the heaviest playing group (low byte) and its category (animcheck.animgroups: idle, walk, attack ...)
    equipped            sorted list of worn item ids (None when the build or the actor cannot say)
    race, female, hair, eyes   the look of the actor's base form (None when unreadable)
"""
from __future__ import annotations

import os
import struct
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Set, Tuple

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT / "tools"))
try:
    from animcheck import animgroups as _groups   # noqa: E402
except Exception:   # noqa: BLE001 - the lab must not need the animation tools; categories are then "unknown"
    _groups = None

MUTANT = os.environ.get("LAB_MUTANT", "")   # test-only; names in tools/lab/run_tests.py

HOST, CLIENT = "host", "client"
PARITY_WEAPON = "FalloutNV.esm:0E3778"       # 9mm pistol (the same item lab_live.WEAPON arms the players with)
TIME_ADDR_CURRENT, TIME_ADDR_TARGET = 0x011AC3A0, 0x011AC3A4     # plugin/vats/vats_addrs.h kTimeScaleCurrent / kTimeScaleTarget
TIME_TOL = 0.01
ANIM_ACTIVE = 1                               # NiControllerSequence::state of a sequence that is playing

# Which module most likely owns a field that differs (docs/sync-model.md: stream = continuous state, parts = equipment and look, events = one-shot).
SOURCES: Dict[Tuple[str, str], str] = {
    ("npc", "base"): "plugin/actors, plugin/spawn (leveled pick, base swap)",
    ("npc", "equipped"): "plugin/actors (parts: equipment of the NPC copy)",
    ("npc", "weapon"): "plugin/actors (parts: equipment of the NPC copy)",
    ("npc", "weaponOut"): "plugin/actors/anim (stream: weapon drawn)",
    ("npc", "pose"): "plugin/actors/anim (stream: sit / furniture)",
    ("npc", "anim"): "plugin/actors/anim (stream: current idle)",
    ("puppet", "present"): "plugin/avatar (puppet spawn)",
    ("puppet", "equipped"): "plugin/avatar (puppet equipment)",
    ("puppet", "weapon"): "plugin/avatar (puppet equipment)",
    ("puppet", "weaponOut"): "plugin/avatar (stream: weapon out)",
    ("puppet", "pose"): "plugin/avatar (puppet sitting, not in the stream yet)",
    ("puppet", "anim"): "plugin/avatar (puppet animation)",
    ("puppet", "attack"): "plugin/avatar (player attack events on puppets)",
    ("puppet", "race"): "plugin/avatar (puppet look: template NPC / Look message)",
    ("puppet", "female"): "plugin/avatar (puppet look: template NPC / Look message)",
    ("puppet", "hair"): "plugin/avatar (puppet look: template NPC / Look message)",
    ("puppet", "eyes"): "plugin/avatar (puppet look: template NPC / Look message)",
    ("machine", "timeScale"): "plugin/hooks_world (TimeScale effect), plugin/world_time, plugin/vats (slow motion)",
}
FIELDS = {
    "npc": ("base", "equipped", "weapon", "weaponOut", "pose"),
    "puppet": ("equipped", "weapon", "weaponOut", "pose", "race", "female", "hair", "eyes"),
}


# ------------------------------------------------------------------ small helpers
def norm_hex(v: Any) -> Optional[str]:
    """'0x0100086F' | '0100086f' | 16779375 -> '0x0100086f'; None for anything that is not an id (0 is an id: 'no form')."""
    try:
        n = int(v) if isinstance(v, int) and not isinstance(v, bool) else int(str(v), 16)
    except (TypeError, ValueError):
        return None
    return "0x%08x" % (n & 0xFFFFFFFF)


def _form_or_none(v: Any) -> Optional[str]:
    h = norm_hex(v)
    return None if h in (None, "0x00000000") else h


def pose_of(sit: Optional[int]) -> Optional[str]:
    """BaseProcess::GetSitSleepState -> stand (0) | sit (1..5: going to, in, leaving a seat) | sleep (6..10). The class, not the step: the copy may be a frame behind."""
    if sit is None:
        return None
    if sit == 0:
        return "stand"
    return "sit" if 1 <= sit <= 5 else ("sleep" if 6 <= sit <= 10 else f"state{sit}")


def category_of(group: Optional[int]) -> Optional[str]:
    if group is None or group < 0:
        return None
    if _groups is None:
        return "unknown"
    try:
        return _groups.category(int(group))
    except Exception:   # noqa: BLE001
        return "unknown"


def primary_group(seqs: List[Dict[str, Any]]) -> Optional[int]:
    """The group of the heaviest sequence that is playing (anim.probe 'seqs'); None when none plays."""
    best: Optional[Tuple[float, int]] = None
    for s in seqs or []:
        g = s.get("group")
        if not isinstance(g, int) or g < 0 or s.get("state") != ANIM_ACTIVE:
            continue
        w = float(s.get("weight") or 0.0)
        if best is None or w > best[0]:
            best = (w, g)
    return None if best is None else best[1]


def primary_category(anim_actor: Optional[Dict[str, Any]]) -> Optional[str]:
    if not anim_actor:
        return None
    return category_of(primary_group((anim_actor.get("anim") or {}).get("seqs") or []))


def decode_time_peek(hex_text: str) -> Optional[Tuple[float, float]]:
    """world.peek {addr: 0x11AC3A0, len: 8} -> (current multiplier, target multiplier); None when the answer is not 8 bytes."""
    try:
        raw = bytes.fromhex(hex_text)
    except (TypeError, ValueError):
        return None
    if len(raw) < 8:
        return None
    cur, tgt = struct.unpack("<ff", raw[:8])
    return cur, tgt


# ------------------------------------------------------------------ views
def view_of(anim_actor: Optional[Dict[str, Any]], look_actor: Optional[Dict[str, Any]]) -> Dict[str, Any]:
    """One actor on one copy from the answers of anim.probe (`anim_actor`) and parity.look (`look_actor`), either of which may be missing."""
    v: Dict[str, Any] = {"ref": None, "base": None, "valid3d": None, "dead": False, "weaponOut": None, "aiming": None, "weapon": None, "sit": None, "pose": None,
                         "action": None, "group": None, "cat": None, "equipped": None, "race": None, "female": None, "hair": None, "eyes": None}
    a = anim_actor
    if a:
        v["ref"], v["base"] = norm_hex(a.get("ref")), norm_hex(a.get("base"))
        v["valid3d"] = bool(a.get("valid3d"))
        v["dead"] = bool(a.get("dead"))
        w = a.get("weapon") or {}
        if "out" in w:
            v["weaponOut"], v["aiming"], v["weapon"] = bool(w.get("out")), bool(w.get("aiming")), _form_or_none(w.get("form")) or "none"
        st = a.get("stance") or {}
        if isinstance(st.get("sit_sleep"), int):
            v["sit"] = int(st["sit_sleep"])
            v["pose"] = pose_of(v["sit"])
        an = a.get("anim") or {}
        if isinstance(an.get("action"), int):
            v["action"] = an["action"]
        v["group"] = primary_group(an.get("seqs") or [])
        v["cat"] = category_of(v["group"])
    la = look_actor
    if la and la.get("ok"):
        v["ref"] = v["ref"] or norm_hex(la.get("ref"))
        v["base"] = v["base"] or norm_hex(la.get("base"))
        if la.get("wornKnown"):
            v["equipped"] = sorted({h for h in (norm_hex(w.get("form")) for w in la.get("worn") or []) if h})
        if "race" in la:
            v["race"], v["female"] = norm_hex(la.get("race")), bool(la.get("female"))
            v["hair"], v["eyes"] = norm_hex(la.get("hair")), norm_hex(la.get("eyes"))
    return v


def puppet_refs(avatar: Optional[Dict[str, Any]]) -> List[str]:
    """The refs of the puppets avatar.status lists (a puppet that is announced and not placed yet has ref 0 and is left out)."""
    return [h for h in (_form_or_none(p.get("ref")) for p in (avatar or {}).get("puppets") or []) if h]


def assemble(anim: Optional[Dict[str, Any]], look: Optional[Dict[str, Any]], avatar: Optional[Dict[str, Any]], peek_hex: str = "", vats_mode: Optional[int] = None) -> Dict[str, Any]:
    """The snapshot of one copy. `anim` = anim.probe {refs: [player, puppets...], near: ...}, `look` = parity.look for the same refs (None: the build has no such
    method), `avatar` = avatar.status, `peek_hex` = world.peek of the time multiplier (used when `look` has none), `vats_mode` = the VATS camera mode when known."""
    missing: List[str] = []
    looks = {norm_hex(a.get("ref")): a for a in (look or {}).get("actors") or []}
    if look is None:
        missing.append("parity.look (equipment and look of actors, time scale)")
    pup_slot: Dict[str, int] = {}
    for p in (avatar or {}).get("puppets") or []:
        h = _form_or_none(p.get("ref"))
        if h:
            pup_slot[h] = int(p.get("slot", len(pup_slot) + 1))
    snap: Dict[str, Any] = {"player": None, "puppets": {}, "actors": {}, "time": None, "missing": missing}
    for a in (anim or {}).get("actors") or []:
        ref = norm_hex(a.get("ref"))
        view = view_of(a, looks.get(ref))
        if a.get("player"):
            snap["player"] = view
        elif ref in pup_slot:
            snap["puppets"][str(pup_slot[ref])] = view
        elif ref:
            snap["actors"][ref] = view
    if look is not None and not snap["player"]:       # anim.probe did not list the player: the look alone still has its worn list
        pl = norm_hex((look or {}).get("player"))
        if pl in looks:
            snap["player"] = view_of(None, looks[pl])
    t = (look or {}).get("time") or {}
    if t.get("ok"):
        snap["time"] = {"current": float(t["current"]), "target": float(t["target"]), "source": "parity.look"}
        vats_mode = (look or {}).get("vatsMode", vats_mode)
    else:
        ct = decode_time_peek(peek_hex) if peek_hex else None
        if ct:
            snap["time"] = {"current": ct[0], "target": ct[1], "source": "world.peek"}
        else:
            missing.append("time scale (no parity.look and no world.peek answer)")
    if snap["time"] is not None:
        snap["time"]["vatsMode"] = vats_mode
    if look is not None and snap["player"] is not None and snap["player"]["equipped"] is None:
        missing.append("worn list of the player")
    return snap


# ------------------------------------------------------------------ comparison
def mismatch(entity: str, kind: str, field: str, host: Any, client: Any, **extra: Any) -> Dict[str, Any]:
    m = {"entity": entity, "kind": kind, "field": field, "host": host, "client": client, "source": SOURCES.get((kind, field), "")}
    m.update(extra)
    return m


def compare_entity(entity: str, kind: str, views: Dict[str, Optional[Dict[str, Any]]], seen: Optional[Dict[str, Set[str]]] = None) -> List[Dict[str, Any]]:
    """The fields of one entity that differ between the copies. views = {"host": view | None, "client": view | None}. A dead actor is not compared (the fight
    room judges deaths); a field either copy cannot read is not compared. `seen` = {"host": {categories}, "client": {...}}: the animation categories each copy
    showed over the window, compared as sets (a walk is never the same frame on two copies): disjoint sets are a mismatch."""
    h, c = views.get(HOST), views.get(CLIENT)
    if h is None or c is None:
        if kind == "puppet":
            return [mismatch(entity, kind, "present", h is not None, c is not None)]
        return []
    if h.get("dead") or c.get("dead"):
        return []
    out: List[Dict[str, Any]] = []
    for f in FIELDS[kind]:
        if MUTANT == "parity_skips_equipment" and f in ("equipped", "weapon"):      # twin: equipment is never compared
            continue
        a, b = h.get(f), c.get(f)
        if a is None or b is None:
            continue
        if a != b:
            out.append(mismatch(entity, kind, f, a, b))
    if seen:
        a, b = seen.get(HOST) or set(), seen.get(CLIENT) or set()
        if a and b and not (a & b) and MUTANT != "parity_anim_unchecked":      # twin: the animation sets are never compared
            out.append(mismatch(entity, kind, "anim", sorted(a), sorted(b)))
    return out


def compare_time(times: Dict[str, Optional[Dict[str, Any]]]) -> List[Dict[str, Any]]:
    """The global time multiplier of each copy must be 1.0 (current and target) outside VATS. A copy in VATS slow motion is not judged at that moment."""
    if MUTANT == "parity_time_unchecked":          # twin: the multiplier is never checked
        return []
    out = []
    fmt = {m: (None if not t else f"{t['current']:g}/{t['target']:g}") for m, t in times.items()}
    for m, t in times.items():
        if not t or t.get("vatsMode"):
            continue
        if abs(t["current"] - 1.0) > TIME_TOL or abs(t["target"] - 1.0) > TIME_TOL:
            out.append(mismatch(f"{m} machine", "machine", "timeScale", fmt.get(HOST), fmt.get(CLIENT), expected="1/1"))
    return out


def puppet_entity(player: str) -> str:
    """The name of the pair 'a player and the other copy's puppet of him' in the findings."""
    return f"puppet of {player} (on {CLIENT if player == HOST else HOST})"


def note_seen(seen: Dict[str, Dict[str, Set[str]]], entity: str, views: Dict[str, Optional[Dict[str, Any]]]) -> Dict[str, Set[str]]:
    s = seen.setdefault(entity, {HOST: set(), CLIENT: set()})
    for m in (HOST, CLIENT):
        v = views.get(m)
        if v and v.get("cat"):
            s[m].add(v["cat"])
    return s


def compare_snapshots(host: Dict[str, Any], client: Dict[str, Any], seen: Optional[Dict[str, Dict[str, Set[str]]]] = None) -> Tuple[List[Dict[str, Any]], Dict[str, Any]]:
    """Everything the two copies show: every NPC (and creature) loaded on both, each player against the other copy's puppet of him, the time multiplier of each
    copy. -> (the mismatches, stats). `seen` (kept by the caller across the rounds of one step) collects the animation categories shown."""
    seen = seen if seen is not None else {}
    out: List[Dict[str, Any]] = []
    ha, ca = host.get("actors") or {}, client.get("actors") or {}
    common = sorted(set(ha) & set(ca))
    for ref in common:
        ent = f"npc {ref}"
        views = {HOST: ha[ref], CLIENT: ca[ref]}
        note_seen(seen, ent, views)
        out += compare_entity(ent, "npc", views, seen.get(ent))
    # a player is compared with the puppet the OTHER copy shows of him (a pair has one puppet per copy; the first slot)
    hp = next(iter(sorted((host.get("puppets") or {}).items())), (None, None))[1]
    cp = next(iter(sorted((client.get("puppets") or {}).items())), (None, None))[1]
    for ent, views in ((puppet_entity(CLIENT), {HOST: hp, CLIENT: client.get("player")}), (puppet_entity(HOST), {HOST: host.get("player"), CLIENT: cp})):
        if views[HOST] is None and views[CLIENT] is None:
            continue
        note_seen(seen, ent, views)
        out += compare_entity(ent, "puppet", views, seen.get(ent))
    out += compare_time({HOST: host.get("time"), CLIENT: client.get("time")})
    stats = {"npcs": len(common), "only_host": sorted(set(ha) - set(ca)), "only_client": sorted(set(ca) - set(ha)),
             "puppets": int(hp is not None) + int(cp is not None)}
    return out, stats


def attack_mismatch(entity: str, owner: str, owner_saw: bool, copy_saw: bool) -> List[Dict[str, Any]]:
    """The player (owner) punched and his own copy showed the attack: the other copy's puppet must show an attack animation too."""
    if MUTANT == "parity_attack_unchecked" or not owner_saw or copy_saw:      # twin: a puppet that never punches is accepted
        return []
    host_saw, client_saw = (owner_saw, copy_saw) if owner == HOST else (copy_saw, owner_saw)
    return [mismatch(entity, "puppet", "attack", "attack shown" if host_saw else "no attack", "attack shown" if client_saw else "no attack")]


# ------------------------------------------------------------------ the findings of a room
def _short_hex(x: Any) -> str:
    s = str(x)
    return "0x" + (s[2:].lstrip("0") or "0") if s.startswith("0x") and len(s) == 10 else s


def show(v: Any, n: int = 60) -> str:
    """A value in a finding's line: ids without their leading zeros (the full values are in the room's metrics)."""
    if isinstance(v, (list, tuple)):
        s = "[" + ",".join(_short_hex(x) for x in v) + "]"
    elif isinstance(v, float):
        s = f"{v:g}"
    else:
        s = _short_hex(v)
    return s if len(s) <= n else s[: n - 1] + "~"


def line_of(m: Dict[str, Any]) -> str:
    src = m.get("source") or ""
    return f"{m['entity']}: {m['field']} host {show(m['host'])} vs client {show(m['client'])}" + (f" -> {src}" if src else "") + (f" [{m['step']}]" if m.get("step") else "")


class ParityLog:
    """The findings of all steps of one parity room: one entry per (entity, field), the first time it was seen with the step it was seen at."""

    def __init__(self) -> None:
        self.items: List[Dict[str, Any]] = []
        self._keys: Dict[Tuple[str, str], int] = {}
        self.steps: List[Dict[str, Any]] = []

    def add(self, step: str, found: List[Dict[str, Any]], rounds: int = 0, stats: Optional[Dict[str, Any]] = None) -> None:
        self.steps.append({"step": step, "mismatches": len(found), "rounds": rounds, **({"npcs": stats["npcs"], "puppets": stats["puppets"]} if stats else {})})
        for m in found:
            key = (m["entity"], m["field"])
            if key in self._keys:
                self.items[self._keys[key]]["steps"] = self.items[self._keys[key]].get("steps", 1) + 1
                continue
            self._keys[key] = len(self.items)
            self.items.append(dict(m, step=step, steps=1))

    def lines(self) -> List[str]:
        return [line_of(m) for m in self.items]

    def headline(self, n: int = 8) -> List[str]:
        """The lines a failure reason can hold: the first finding of each distinct field first, then the rest (a results row keeps 8 values; all of them are in the metrics)."""
        picked: List[Dict[str, Any]] = []
        fields: Set[str] = set()
        for m in self.items:
            if m["field"] not in fields:
                fields.add(m["field"])
                picked.append(m)
        picked += [m for m in self.items if all(m is not p for p in picked)]
        return [line_of(m) for m in picked[:n]]

    def sources(self) -> str:
        seen: List[str] = []
        for m in self.items:
            mod = (m.get("source") or "tools/lab (parity)").split(" (")[0]
            for part in mod.split(", "):
                if part not in seen:
                    seen.append(part)
        return ", ".join(seen)

    def fields(self) -> Dict[str, int]:
        out: Dict[str, int] = {}
        for m in self.items:
            out[m["field"]] = out.get(m["field"], 0) + 1
        return out
