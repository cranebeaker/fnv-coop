"""layout.json of the lab plugin (docs/LAB.md): the only thing the runner reads from the generator in tools/lab/esp.

    Layout.load(path)  -> Layout (raises LayoutError with one clear line)
    layout.refs("fight", "enemy")  -> ["NVCoopLab.esp:000820", ...]  (the room's refs whose key starts with a prefix, in key order)
    sample_layout()  -> a complete layout dict for the fake world and the tests

Forms are `Plugin.esp:XXXXXX` with the plugin's own object id. The runtime load-order index is filled in by the bots (Bot._norm), the
same way they resolve `FalloutNV.esm:` forms, so the same form is the same runtime id on both copies.

The real layout (tools/lab/esp, out/lab/layout.json) puts per-room facts under `rooms.<r>.info` and lists hostiles that start disabled in
`rooms.<r>.initially_disabled`; `cell.editor_id_alt` is the stock name of the overridden cell. Ref names the runner looks for (by prefix):
  walk: wp1..wp4        fight: enemy1..enemy3 (initially_disabled)   loot: chest (or container), loose1..loose4
  door: door, terminal  npc: npc, patrol_a, patrol_b                  vats: target1, target2 (initially_disabled)
  quest: quest, anchor
  parity: no room of its own (it borrows the npc room: npc, stand); a `chair` (or furniture / seat) ref in the parity or npc room lets it make the NPC sit.
Info keys (read from `rooms.<r>.<key>` or `rooms.<r>.info.<key>`): loot container_items [{form, count}] and loose_items [form x4, same order as the
loose refs], door terminal_entry (default 0), quest var (default labvar).
"""
from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any, Dict, List, Optional

ROOM_ORDER = ("spawn", "walk", "fight", "loot", "door", "npc", "vats", "quest", "parity")
# A room that has no placed content of its own and borrows another room's (the parity room compares what both copies show of the npc room's NPC). A layout that has
# a room of that name (a generator that adds a parity room with its own chair) wins over the borrowed one.
BORROWS = {"parity": "npc"}
FORM_RE = re.compile(r"^[A-Za-z0-9_ .\-]+\.(?:esp|esm|esl):[0-9A-Fa-f]{1,8}$")

# what each room needs from the layout: (kind, prefixes, minimum count)
NEEDS = {
    "spawn": [("markers", ("host", "client"), 2)],
    "walk": [("refs", ("wp", "waypoint"), 2)],
    "fight": [("refs", ("enemy",), 3)],
    "loot": [("refs", ("container", "chest"), 1), ("refs", ("item", "loose", "floor"), 4)],
    "door": [("refs", ("door",), 1), ("refs", ("terminal",), 1)],
    "npc": [("refs", ("npc",), 1)],
    "vats": [("refs", ("target",), 2)],
    "quest": [("refs", ("quest",), 1)],
    "parity": [("refs", ("npc",), 1)],
}


class LayoutError(Exception):
    pass


def norm_form(s: str) -> str:
    """'nvcooplab.esp:0008a0' -> 'nvcooplab.esp:0008A0' (plugin name as given, id as 6 upper-case hex digits)."""
    if not isinstance(s, str) or not FORM_RE.match(s.strip()):
        raise LayoutError(f"not a form (want Plugin.esp:XXXXXX): {s!r}")
    plug, _, h = s.strip().rpartition(":")
    return f"{plug}:{int(h, 16):06X}"


def _sort_key(k: str):
    m = re.match(r"^(\D*)(\d+)$", k)
    return (m.group(1), int(m.group(2))) if m else (k, 0)


class Layout:
    def __init__(self, data: Dict[str, Any], path: Optional[Path] = None):
        self.data = data
        self.path = path
        self.plugin: str = data.get("plugin") or ""
        cell = data.get("cell") or {}
        self.cell_edid: str = cell.get("editor_id") or "NVCoopLab"
        self.cell_form: Optional[str] = cell.get("form")
        self.cell_alt: str = cell.get("editor_id_alt") or ""
        self.rooms: Dict[str, Dict[str, Any]] = data.get("rooms") or {}

    @classmethod
    def load(cls, path) -> "Layout":
        p = Path(path)
        try:
            data = json.loads(p.read_text(encoding="utf-8"))
        except OSError as e:
            raise LayoutError(f"cannot read {p.name}: {e.strerror or e}") from None
        except ValueError as e:
            raise LayoutError(f"{p.name} is not json: {e}") from None
        lay = cls(data, p)
        bad = lay.problems()
        if bad:
            raise LayoutError(f"{p.name}: " + "; ".join(bad[:4]))
        return lay

    def problems(self) -> List[str]:
        out = []
        if not isinstance(self.data, dict) or not self.plugin:
            return ["no plugin name"]
        if not isinstance(self.rooms, dict) or not self.rooms:
            return ["no rooms"]
        if self.cell_form:
            try:
                norm_form(self.cell_form)
            except LayoutError as e:
                out.append(f"cell form: {e}")
        for name, r in self.rooms.items():
            if not isinstance(r, dict):
                out.append(f"room {name} is not an object")
                continue
            o = r.get("origin")
            if not (isinstance(o, list) and len(o) == 3 and all(isinstance(v, (int, float)) for v in o)):
                out.append(f"room {name}: origin must be [x, y, z]")
            forms = list((r.get("refs") or {}).values()) + list((r.get("markers") or {}).values()) + ([r["stand"]] if r.get("stand") else [])
            for f in forms:
                try:
                    norm_form(f)
                except LayoutError as e:
                    out.append(f"room {name}: {e}")
            if name != "spawn" and not r.get("stand"):
                out.append(f"room {name}: no stand marker")
        return out

    # ---- accessors
    def has_room(self, name: str) -> bool:
        return name in self.rooms or BORROWS.get(name) in self.rooms

    def room(self, name: str) -> Dict[str, Any]:
        return self.rooms.get(name) or self.rooms.get(BORROWS.get(name, "")) or {}

    def origin(self, name: str):
        o = self.room(name).get("origin")
        return tuple(float(v) for v in o) if o else None

    def stand(self, name: str) -> Optional[str]:
        s = self.room(name).get("stand")
        return norm_form(s) if s else None

    def marker(self, room: str, key: str) -> Optional[str]:
        m = (self.room(room).get("markers") or {}).get(key)
        return norm_form(m) if m else None

    def refs(self, room: str, *prefixes: str) -> List[str]:
        """The room's refs whose key starts with one of the prefixes, ordered by key (enemy1, enemy2, ... enemy10)."""
        refs = self.room(room).get("refs") or {}
        keys = sorted((k for k in refs if any(k.lower().startswith(p) for p in prefixes)), key=_sort_key)
        return [norm_form(refs[k]) for k in keys]

    def ref(self, room: str, *prefixes: str) -> Optional[str]:
        r = self.refs(room, *prefixes)
        return r[0] if r else None

    def extra(self, room: str, key: str, default=None):
        """A room fact: `rooms.<r>.<key>`, else `rooms.<r>.info.<key>`, else the default."""
        r = self.room(room)
        if key in r:
            return r[key]
        return (r.get("info") or {}).get(key, default)

    def initially_disabled(self, room: str) -> List[str]:
        """The room's refs that the plugin places disabled (the runner enables them when the room starts). Entries are ref keys or forms."""
        refs = self.room(room).get("refs") or {}
        out = []
        for k in self.room(room).get("initially_disabled") or []:
            if k in refs or FORM_RE.match(str(k)):
                out.append(norm_form(refs.get(k, k)))
        return out

    def container_items(self, room: str = "loot") -> Dict[str, int]:
        """{item form: count} the plugin put into the room's container (list of {form, count} or a dict)."""
        ci = self.extra(room, "container_items")
        if isinstance(ci, list):
            return {norm_form(x["form"]): int(x["count"]) for x in ci}
        if isinstance(ci, dict):
            return {norm_form(k): int(v) for k, v in ci.items()}
        return {}

    def loose_bases(self, room: str = "loot") -> List[str]:
        """The base item of each loose ref, in ref order; [] when the layout does not say."""
        return [norm_form(x) for x in self.extra(room, "loose_items") or []]

    def missing(self, room: str) -> List[str]:
        """What the layout lacks for this room (empty: it can run)."""
        if not self.has_room(room):
            return [f"room {room} is not in the layout"]
        out = []
        for kind, prefixes, n in NEEDS.get(room, []):
            if kind == "markers":
                got = [k for k in prefixes if self.marker(room, k)]
                if len(got) < n:
                    out.append(f"room {room}: markers {list(prefixes)} needed, found {got}")
            else:
                got = self.refs(room, *prefixes)
                if len(got) < n:
                    out.append(f"room {room}: {n} ref(s) named {'/'.join(p + '*' for p in prefixes)} needed, found {len(got)}")
        if room != "spawn" and not self.stand(room):
            out.append(f"room {room}: no stand marker")
        return out


def sample_layout() -> Dict[str, Any]:
    """A complete layout of the shape tools/lab/esp writes (made-up ids and positions), used by the fake world and the tests."""
    P = "NVCoopLab.esp"

    def f(n):
        return f"{P}:{n:06X}"
    return {
        "plugin": P,
        "cell": {"editor_id": "NVCoopLab", "form": "FalloutNV.esm:168FBC", "editor_id_alt": "GreenRoom", "floor_z": 7040.0},
        "rooms": {
            "spawn": {"origin": [2992, -6048, 7040], "markers": {"host": f(0x810), "client": f(0x811)}, "stand": f(0x81F)},
            "walk": {"origin": [3472, -6048, 7040], "stand": f(0x82F), "refs": {"wp1": f(0x821), "wp2": f(0x822), "wp3": f(0x823), "wp4": f(0x824)}},
            "fight": {"origin": [3952, -6048, 7040], "stand": f(0x83F), "refs": {"enemy1": f(0x831), "enemy2": f(0x832), "enemy3": f(0x833)},
                      "initially_disabled": ["enemy1", "enemy2", "enemy3"], "info": {"enemy_base": "FalloutNV.esm:10CD73"}},
            "loot": {"origin": [4432, -6048, 7040], "stand": f(0x84F),
                     "refs": {"chest": f(0x841), "loose1": f(0x842), "loose2": f(0x843), "loose3": f(0x844), "loose4": f(0x845)},
                     "info": {"container_items": [{"form": "FalloutNV.esm:015169", "count": 5}, {"form": "FalloutNV.esm:015167", "count": 3}],
                              "loose_items": ["FalloutNV.esm:015169", "FalloutNV.esm:01519E", "FalloutNV.esm:015197", "FalloutNV.esm:015167"]}},
            "door": {"origin": [2992, -5216, 7040], "stand": f(0x85F), "refs": {"door": f(0x851), "terminal": f(0x852)}, "info": {"lock_level": 25}},
            "npc": {"origin": [3472, -5216, 7040], "stand": f(0x86F), "refs": {"patrol_a": f(0x861), "patrol_b": f(0x862), "npc": f(0x863), "chair": f(0x864)}},
            "vats": {"origin": [3952, -5216, 7040], "stand": f(0x87F), "refs": {"target1": f(0x871), "target2": f(0x872)},
                     "initially_disabled": ["target1", "target2"]},
            "quest": {"origin": [4432, -5216, 7040], "stand": f(0x88F), "refs": {"anchor": f(0x881), "quest": f(0x802)},
                      "info": {"editor_id": "NVCoopLabQuest", "stages": [10, 20, 30], "var": "labvar"}},
        },
    }
