"""Game-free fake bridge for the bot API (same idea as tools/runner/tests/fake_server.py, but in-process).

It implements the console grammar and bridge methods in primitives.py over a tiny world, with a virtual clock
(sleep() advances time instantly) so tests run in milliseconds. mutant= injects one defect the tests must catch:

  setpos_ignored    SetPos does nothing
  angle_ignored     SetAngle does nothing
  activate_noop     Activate does nothing (doors never open, NPCs never talk, items never picked up)
  transfer_noop     RemoveItemTarget does nothing
  transfer_dup      RemoveItemTarget gives the destination the items but the source keeps them
  equip_noop        EquipItem does nothing
  drop_noop         Drop does nothing
  fire_miss         bot.fire reports fired:0
  health_stuck      shots land but health never changes
  dialog_never      activating an NPC never opens dialogue
  dialog_lists_all  bot.dialog lists the topics of every actor, not only those of the NPC in dialogue
  ref_info_error    ref.info always fails with an RPC error
  crash             exec raises ConnectionLost
  drop_wrong_ref    the OnDrop event names a different ref than the one the engine created
  travel_ignored    bot.fasttravel is accepted but the player never moves
  exec_ref_ignored  exec {cmd, ref} checks the ref, then runs the line with no calling reference
  tutorial_stuck    a terminal opens under a first-use tutorial box and bot.menu.close says closed but the box stays (the plugin before 2026-10-27 refused it)
  vats_freeze       bot.vats starts a job and then the game's main thread stops: every bridge request answers "main thread did not service the request"
  menu_close_ignored  bot.menu.close says closed but the container menu stays open (the plugin's call did nothing)
  msgbox_stuck      bot.menu.press presses the engine's message box natively, but the box never closes
  msgbox_ui_only    bot.menu.press only answers the UI layer's box (the plugin before the native path existed)
  run_is_key        bot.move holds Run for run:true whatever the always-run toggle is (the plugin before run = speed)
  run_omitted_is_false  bot.move without run counts as run:false (C++ twin 47): under always-run it holds Run and walks

Always-run: `always_run` is the engine's toggle (PlayerCharacter+0x651). True is the New Vegas default. bot.move `run` is the speed.
The fake plugin holds Run only when run != always_run (runControlFor in bot_rpc.h). A bot.move without run holds no Run key, so the
always-run toggle picks the speed (speedGiven_ in bot_rpc.h). The fake engine moves at run speed when
Run held != always_run (the run decision at 0x941A2B, keyboard mode). So with always-run on, Run held means walk. The bot.move
reply and bot.status carry the same run fields as bot_rpc.h.

Console lines follow the game (in game, 2026-10-21): "player.X" works, "<hex>.X" fails with
Script command "<hex>.X" not found, and a line for another reference names it in exec's `ref` (calling reference).

Message boxes: `box` is the UI layer's own box (ui.open / ui.answer); `native_boxes` lists the engine's own queued boxes (MessageMenu,
menu id 1001), one entry per box = its number of buttons. bot.menu reports kind "message" while either is shown.
`box_waves` = [(virtual time, [button counts]), ...] sorted by time: engine boxes that join `native_boxes` later, the way the DLC
notices come after a load (6, then 2, then 2 about 16 s later). While an engine box is shown the game is in menu mode: held
movement controls do not move the player.
`box_closing_s` > 0: after a native press the box stays the top menu for that many virtual seconds with no current message (it is
closing). A press in that time gets the plugin's refusal "... has no current message (it is closing) ...". Then the box goes and the
next queued box (if any) is shown.
"""
from __future__ import annotations


import re
import math
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple

from .bot import PLAYER_REF, angle_diff, heading_deg
from .bot import BridgeError, ConnectionLost, RpcError  # re-exported from nvbridge
from . import primitives as P

PLAYER_ID = 0x14
_DRIVER_METHODS = {P.M_BOT_STATUS, P.M_BOT_MOVE, P.M_BOT_STOP, P.M_BOT_LOOK, P.M_BOT_FIRE, P.M_BOT_RELOAD, P.M_BOT_JUMP,
                   P.M_BOT_SNEAK, P.M_BOT_ACTIVATE}
# Functions the fake console evaluates. The game has IsSneaking. It has no GetSneaking ("Unknown variable", first in-game run 2026-10-21).
_EXPR_FUNCS = {"issneaking", "getangle", "getequipped", "getitemcount", "getopenstate", "getlocked", "getlocklevel",
               "getplayerteammate", "getstage", "getav", "isininterior", "gettype"}
SAY_GATE_S = 0.5   # the dialogue's click gate: bot.say refuses for 500 ms after a click (kDlgClickGate in plugin/bots/bot_engine.h)


JUMP_SETTLE_S = 2.0  # see M_BOT_SNEAK in FakeBridge: crouch toggles right after a jump


class VirtualClock:
    def __init__(self) -> None:
        self.t = 0.0
        self.on_advance = None

    def monotonic(self) -> float:
        return self.t

    def sleep(self, s: float) -> None:
        dt = max(s, 0.001)
        self.t += dt
        if self.on_advance:
            self.on_advance(dt)


@dataclass
class Thing:
    id: int
    kind: str                       # actor | container | door | item | terminal
    pos: List[float] = field(default_factory=lambda: [0.0, 0.0, 0.0])
    cell: int = 1
    health: float = 100.0
    items: Dict[int, int] = field(default_factory=dict)
    open_state: int = 3             # 3 closed, 1 open
    locked: bool = False
    disabled: bool = False
    deleted: bool = False           # ref.info flag 0x20: what the game shows for a world item that was picked up
    base_item: int = 0              # for kind == item: what picking it up gives
    topics_text: List[str] = field(default_factory=list)   # terminal entry names
    base_id: int = 0                # base form id (0 = same as id); used by dev.countRefs
    topics: List[int] = field(default_factory=list)
    teammate: bool = False          # SetPlayerTeammate
    life: int = 0                   # ref.info lifeState of a living actor: 0 alive, 3 unconscious, 5 restrained, 6 essential unconscious
    load_to: int = 0                # door only: the cell a load door leads to (0 = an ordinary door that opens and closes)
    count0_gives_nothing: bool = False   # item only: the engine's Activate with the bridge's last argument 0 deletes the ref and gives the player nothing (the lab's Stimpak)

    @property
    def dead(self) -> bool:
        return self.kind == "actor" and self.health <= 0

    @property
    def life_state(self) -> int:
        return 2 if self.dead else self.life

    @property
    def info_type(self) -> int:
        """What the game's ref.info answers (TESObjectREFR::typeID): 0x3A for every placed object, 0x3B for an actor. The BASE type is type_id (GetType)."""
        return P.TYPE_ACTOR if self.kind == "actor" else P.TYPE_REFR

    @property
    def type_id(self) -> int:
        return {"actor": P.TYPE_ACTOR, "container": P.TYPE_CONTAINER, "door": P.TYPE_DOOR, "terminal": 0x17, "item": 0x28}[self.kind]


@dataclass
class Player:
    pos: List[float] = field(default_factory=lambda: [0.0, 0.0, 0.0])
    cell: int = 1
    angle: Dict[str, float] = field(default_factory=lambda: {"X": 0.0, "Z": 0.0})
    items: Dict[int, int] = field(default_factory=dict)
    equipped: set = field(default_factory=set)
    in_dialogue: Optional[int] = None
    ammo: int = 12
    health: float = 200.0
    said: List[int] = field(default_factory=list)


class World:
    def __init__(self) -> None:
        self.things: Dict[int, Thing] = {}
        self.stages: Dict[int, int] = {}
        self.players: List[Player] = []
        self.walls: List[Tuple[float, float, float, float]] = []   # axis-aligned boxes (x1, y1, x2, y2) that block walking

    def add_wall(self, x1: float, y1: float, x2: float, y2: float) -> None:
        self.walls.append((min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)))

    def add(self, thing: Thing) -> Thing:
        self.things[thing.id] = thing
        return thing

    def add_actor(self, id: int, pos=(0, 0, 0), health: float = 100.0, cell: int = 1, topics=()) -> Thing:
        return self.add(Thing(id, "actor", list(map(float, pos)), cell, health, topics=list(topics)))

    def add_container(self, id: int, pos=(0, 0, 0), items=None, cell: int = 1) -> Thing:
        return self.add(Thing(id, "container", list(map(float, pos)), cell, items=dict(items or {})))

    def add_door(self, id: int, pos=(0, 0, 0), locked: bool = False, cell: int = 1) -> Thing:
        return self.add(Thing(id, "door", list(map(float, pos)), cell, locked=locked))

    def add_terminal(self, id: int, pos=(0, 0, 0), cell: int = 1) -> Thing:
        return self.add(Thing(id, "terminal", list(map(float, pos)), cell))

    def add_item(self, id: int, base_item: int, pos=(0, 0, 0), cell: int = 1) -> Thing:
        return self.add(Thing(id, "item", list(map(float, pos)), cell, base_item=base_item))

    def set_stage(self, quest: int, stage: int) -> None:
        self.stages[quest] = stage

    def new_player(self) -> Player:
        p = Player()
        self.players.append(p)
        return p


def _hex(tok: str) -> int:
    return int(tok, 16)


def engine_form_name(fid: int) -> str:
    """How the engine itself names a form in an OnDrop record: lower-case plugin, 6 hex digits ('falloutnv.esm:015169', in game 2026-10-21)."""
    return f"falloutnv.esm:{fid & 0xFFFFFF:06X}"


def same_form_name(a: str, b: str) -> bool:
    """sameFormName in plugin/bots/bot_engine.h: the plugin part without case, the hex part as a number."""
    pa, _, ha = a.rpartition(":")
    pb, _, hb = b.rpartition(":")
    try:
        return pa.lower() == pb.lower() and int(ha, 16) == int(hb, 16)
    except ValueError:
        return False


def form_name(fid: int) -> str:
    """How the plugin names a form (nameOfForm in plugin/bots/bots_module.cpp): 'FalloutNV.esm:HEX', or the bare 8-digit runtime
    id for a created ref (load index 0xFF, e.g. from placeatme or a drop), which no plugin owns."""
    return f"{fid:08X}" if fid >> 24 == 0xFF else f"FalloutNV.esm:{fid:X}"


class FakeBridge:
    def __init__(self, world: Optional[World] = None, mutant: Optional[str] = None, bot_methods: bool = True,
                 menu_method: bool = True, input_driver: Optional[bool] = None, drop_events: bool = True,
                 genuine_travel: bool = True, travel_refuses: bool = False, query_av: bool = True):
        self.world = world or World()
        self.me = self.world.new_player()
        self.mutant = mutant
        self.bot_methods = bot_methods
        self.menu_method = menu_method
        self.input_driver = bot_methods if input_driver is None else input_driver
        self.drop_events = drop_events          # the OnDrop native handler is registered
        self.genuine_travel = genuine_travel    # bot.fasttravel exists (engine glue on)
        self.travel_refuses = travel_refuses    # the engine's may-travel check says no
        self.query_av = query_av                # False = world.query GetActorValue fails for every ref (in game 2026-10-21:
                                                # Track A's own detour on Cond_GetActorValue fails queryCondition's prologue check)
        self.activate_reach = 400.0             # bot.activate range; the game's activation reach is about 150 (tests set it)
        self.drop_log: List[Dict[str, Any]] = []
        self.travel_due: Optional[Tuple[float, "Thing"]] = None   # (virtual time, marker): the player moves a little after the call
        self._next_ref = 0xFF000000
        self.held: set = set()
        self.terminal_used: Dict[str, List[Dict[str, Any]]] = {}
        self.tutorial_on_terminal = False       # the first terminal opened shows the first-use tutorial box over it (tests set it)
        self._tutorial_seen = False
        self.box = None                 # menu id of a message box being shown (tests set it)
        self.box_answers: List[int] = []
        self.native_boxes: List[int] = []     # engine message boxes (MessageMenu 1001) queued, each = its button count (tests set it)
        self.native_presses: List[int] = []   # buttons pressed natively, in order
        self.click_at: Optional[float] = None  # virtual time of the last dialogue click (talk, bot.say): the click gate
        self.interior_cells: set = set()      # cells that are interiors (the engine forbids fast travel there; tests set it)
        self.box_waves: List[Tuple[float, List[int]]] = []   # (virtual time, [button counts]): engine boxes that appear later (tests set it)
        self.box_closing_s = 0.0                # seconds a pressed engine box stays closing (tests set it); 0 = it goes at once
        self._closing_until: Optional[float] = None
        self.always_run = True                  # the engine's always-run toggle; True = on, the New Vegas default (tests set it)
        self.shots_left = 0
        self.vats_state = {"phase": "idle", "reason": ""}
        self.sneaking = False
        self.shot_timer = 0.0
        self.jump_t = 0.0
        self.jump_base = 0.0
        self.clock = VirtualClock()
        self.clock.on_advance = self._advance
        self.calls: List[Tuple[str, Any]] = []
        self.lease = None
        self.game_pid = 1

    @classmethod
    def pair(cls, world: Optional[World] = None, **kw) -> Tuple["FakeBridge", "FakeBridge"]:
        w = world or World()
        return cls(w, **kw), cls(w, **kw)

    # ---- NvBridge surface used by Bot ----
    def plugins(self) -> List[str]:
        return ["FalloutNV.esm"]

    def ping(self) -> Dict[str, Any]:
        return {"pong": True, "pid": 1}

    def state_to_file(self, path: str, sections=None) -> Dict[str, Any]:
        return {"path": path, "sections": sections or []}

    def eval(self, expr: str, ref: Optional[str] = None) -> float:
        line = 'printc "NVCOOP_EVAL=%g" (' + expr + ')'
        out = (self.exec(line) if ref is None else self.exec(line, ref=ref))["output"]
        if "NVCOOP_EVAL=" not in out:                       # same failure as nvbridge.NvBridge.eval
            raise BridgeError(f"cannot evaluate {expr!r}: {out!r}")
        return float(out.rsplit("NVCOOP_EVAL=", 1)[1].split()[0])

    def exec(self, cmd: str, timeout_ms: Optional[int] = None, *, ref: Optional[str] = None) -> Dict[str, Any]:
        """Like the game: a line runs with the calling reference `ref` (exec {cmd, ref}); "player.X" is the only
        reference prefix the console accepts, and "<hex>.X" fails with: Script command "<hex>.X" not found.
        With `ref`, the reply also has the resolved 8-digit runtime id, as bridge.cpp gives it (also when the line fails)."""
        if self.mutant == "crash":
            raise ConnectionLost("fake: pipe closed")
        self.calls.append(("exec", cmd) if ref is None else ("exec", cmd, ref))
        self_ref = None if ref is None else self._calling(ref)
        reply = self._exec_line(cmd, self_ref)
        if ref is not None:
            reply["ref"] = f"{self._rid(ref):08X}"
        return reply

    def _exec_line(self, cmd: str, self_ref) -> Dict[str, Any]:
        if self.mutant == "exec_ref_ignored":
            self_ref = None
        m = re.fullmatch(r'printc "NVCOOP_EVAL=%g" \((.*)\)', cmd)
        text = m.group(1).strip() if m else cmd.strip()
        bad = re.match(r"(\w+)\.\w+", text)
        if bad and bad.group(1).lower() != "player":
            return {"ok": False, "output": f'Script command "{text.split()[0]}" not found'}
        if m:
            fm = re.match(r"(?:\w+\.)?(\w+)", text)
            if fm and fm.group(1).lower() not in _EXPR_FUNCS:
                return {"ok": False, "output": f"Unknown variable '{fm.group(1)}'"}   # what the game's console prints
            v = self._expr(text, self_ref)
            if v is None:
                return {"ok": False, "output": "unknown expression"}
            return {"ok": True, "output": f"NVCOOP_EVAL={v:g}"}
        if self.mutant == "no_removeitemtarget" and re.match(r"(?:player\.)?removeitemtarget", text, re.I):
            # the second PC's build: the console answers ok and prints the error in the output, nothing moves
            return {"ok": True, "output": "SCRIPTS: Script SysWindowCompileAndRun, line 1: Script command RemoveItemTarget not found."}
        ok = self._stmt(text, self_ref)
        return {"ok": ok, "output": "" if ok else "unknown command"}

    def _calling(self, ref: str):
        """exec's ref -> the Thing (or "player"), as the bridge resolves it: 'Plugin.esm:HEX' with the load order of
        plugins(), or a runtime id 'HEX' / '0xHEX'. Anything else raises the bridge's errors (-32602 / -32013)."""
        rid = self._rid(ref)
        if rid == PLAYER_ID:
            return "player"
        t = self.world.things.get(rid)
        if t is None:
            raise RpcError(-32013, f"exec: ref '{ref}' (form {rid:08X}) is not a loaded form", "exec")
        return t

    def _rid(self, ref: str) -> int:
        """exec's ref -> the runtime form id (the bridge's parsing and its -32602 / -32013 errors)."""
        if ref.lower() == "player":     # the real bridge: ParseRefSpec refuses it ("player.X" lines need no ref); in game 2026-10-22 vats_classes
            raise RpcError(-32602, f"exec: bad ref '{ref}': '{ref}' is not a hex number", "exec")
        plugin, colon, hexpart = ref.rpartition(":")
        hexpart = hexpart[2:] if hexpart[:2] in ("0x", "0X") and len(hexpart) > 2 else hexpart
        if not re.fullmatch(r"[0-9A-Fa-f]{1,8}", hexpart) or (colon and (not plugin or int(hexpart, 16) > 0xFFFFFF)):
            raise RpcError(-32602, f"exec: bad ref '{ref}'", "exec")
        rid = int(hexpart, 16)
        if colon:
            low = [p.lower() for p in self.plugins()]
            if plugin.lower() not in low:
                raise RpcError(-32013, f"exec: ref '{ref}': plugin '{plugin}' is not loaded", "exec")
            rid |= low.index(plugin.lower()) << 24
        return rid

    def call(self, method: str, params: Optional[Dict[str, Any]] = None, timeout_s: float = 35.0) -> Any:
        params = params or {}
        self.calls.append((method, params))
        w = self.world
        if method == P.M_REF_INFO:
            if self.mutant == "ref_info_error":
                raise RpcError(-32000, "reference not loaded", method)
            t = self._by_ref(params["ref"])
            if t == "player":
                p = self.me
                return {"x": p.pos[0], "y": p.pos[1], "z": p.pos[2], "lifeState": 0, "flags": 0, "type": 0x3B, "cell": p.cell}
            if t is None:
                raise RpcError(-32000, "reference not loaded", method)
            return {"x": t.pos[0], "y": t.pos[1], "z": t.pos[2], "lifeState": t.life_state,
                    "flags": (P.FLAG_DISABLED if t.disabled else 0) | (P.FLAG_DELETED | 0x4300A if t.deleted else 0), "type": t.info_type, "cell": t.cell}
        if method == P.M_ACTIVATE:
            t = self._by_ref(params["ref"])
            if t is None or t == "player":
                raise RpcError(-32000, "ref or player not loaded", method)
            if self.mutant != "activate_noop":
                self._activate(t, count0=True)
            return {"ok": True, "returned": 1}
        if method == P.M_QUERY:
            t = self._by_ref(params["ref"])
            if params["fn"] != "GetActorValue" or params.get("p1") != P.HEALTH_AV:
                raise RpcError(-32000, "query failed", method)
            if not self.query_av:   # the game's own message
                raise RpcError(-32000, "query failed (unknown fn, ref not loaded or evaluator drifted)", method)
            return {"value": self.me.health if t == "player" else (t.health if t else 0)}
        if method == P.M_DIALOG_OPEN:
            return {"open": self.me.in_dialogue is not None}
        if method == P.M_DIALOG_CLOSE:
            was = self.me.in_dialogue is not None
            self.me.in_dialogue = None
            return {"closed": was}
        if method.startswith("bot.") and not self.bot_methods and method not in _DRIVER_METHODS:
            raise RpcError(-32601, "unknown method: " + method, method)
        if method in _DRIVER_METHODS:
            return self._driver(method, params)
        if method == P.M_BOT_VATS:
            t = self._by_ref(params["ref"])
            if isinstance(t, Thing) and not t.dead:
                if self.mutant != "vats_noop":
                    t.health -= 40
                self.vats_state = {"phase": "done" if self.mutant != "vats_fail" else "failed", "reason": "target_not_selectable"}
                if self.mutant == "vats_freeze":
                    self.vats_state = {"phase": "enter", "reason": ""}
                return {"started": True}
            raise RpcError(-32000, "target not loaded", method)
        if method == P.M_BOT_SAY:
            d = self.me.in_dialogue
            t = w.things.get(d) if d is not None else None
            if t and self.click_at is not None and self.clock.t - self.click_at < SAY_GATE_S:
                raise RpcError(-32000, "click gate closed (a click happened < 500 ms ago)", method)
            if t and _hex(params["info"].split(":")[1]) in t.topics:
                self.me.said.append(_hex(params["info"].split(":")[1]))
                self.click_at = self.clock.t
                return {"said": True}
            return {"said": False}
        if method == P.M_BOT_MENU:
            if not self.menu_method:
                raise RpcError(-32601, "unknown method: " + method, method)
            d = self.me.in_dialogue
            if d is not None:
                return {"kind": "dialogue", "id": 0x3F1, "dialogReady": True}
            if self.box is not None or self.native_boxes:
                return {"kind": "message", "id": 0x3E9}
            return {"kind": getattr(self, "_menu", "none")}
        if method == P.M_BOT_MENU_CLOSE:
            if not self.menu_method:
                raise RpcError(-32601, "unknown method: " + method, method)
            top = getattr(self, "_menu", "none")
            want = params.get("kind") or "container"
            if self.box is not None or self.native_boxes:
                top = "message"
            if top == "none":
                raise RpcError(-32000, "no menu is open", method)
            if top == "tutorial" and want in ("tutorial", "any"):
                if self.mutant != "tutorial_stuck":
                    self._menu = "terminal"
                return {"closed": True, "menu": 0x423, "kind": "tutorial"}
            if (want == "any" and top in ("message", "dialogue")) or (want != "any" and top != want):
                raise RpcError(-32000, f"the top menu is {top}, not {want}: left open", method)
            if self.mutant != "menu_close_ignored":
                self._menu = "none"
            return {"closed": True, "menu": 0x3F0 if top == "container" else 0x421, "kind": top}
        if method == P.M_BOT_DIALOG:
            d = self.me.in_dialogue
            if d is None:
                raise RpcError(-32000, "no dialogue open", method)
            t = self.world.things[d]
            tops = list(t.topics)
            if self.mutant == "dialog_lists_all":
                tops = [top for x in self.world.things.values() for top in x.topics]
            return {"choices": [{"idx": i, "formId": top, "info": f"FalloutNV.esm:{top:X}"} for i, top in enumerate(tops)]}
        if method == P.M_BOT_INVENTORY:
            holder = self._by_ref(params["ref"]) if params.get("ref") else "player"
            items = self._holder_items(holder)
            if items is None:
                raise RpcError(-32000, "reference not loaded", method)
            want = [_hex(i.split(":")[1]) for i in params.get("items", [])]
            ids = want or sorted(items)
            return {"items": [{"item": f"FalloutNV.esm:{i:X}", "count": items.get(i, 0)} for i in ids if want or items.get(i, 0)],
                    "partial": not want}
        if method in (P.M_BOT_PRESS, P.M_BOT_CONSENT):
            if self.box is None:
                # same order as bot_rpc.h: the UI layer's box first, then (bot.menu.press only) the engine's own message box
                if method == P.M_BOT_PRESS and self.native_boxes and self.mutant != "msgbox_ui_only":
                    button, n = params["button"], self.native_boxes[0]
                    if self._closing_until is not None:   # the gate order of pressNativeMessageButton: closing before the button range
                        raise RpcError(-32000, "native message box: the message box has no current message (it is closing)"
                                               " (ui layer: no message box is being shown)", method)
                    if not 0 <= button < n:
                        raise RpcError(-32000, f"native message box: button {button} is not shown (the box has {n} buttons)"
                                               " (ui layer: no message box is being shown)", method)
                    self.native_presses.append(button)
                    if self.mutant != "msgbox_stuck":
                        if self.box_closing_s > 0:
                            self._closing_until = self.clock.t + self.box_closing_s
                        else:
                            self.native_boxes.pop(0)
                    return {"menu": 0x3E9, "button": button, "value": button, "buttons": n, "path": "native"}
                raise RpcError(-32000, "no message box is being shown", method)
            button = params["button"] if method == P.M_BOT_PRESS else {"ready": 0, "notyet": 1}[params["answer"]]
            self.box_answers.append(button)
            menu, self.box = self.box, None
            return {"menu": menu, "button": button, "path": "ui"} if method == P.M_BOT_PRESS else {"menu": menu, "button": button}
        if method == P.M_TERMINAL:
            t = self._by_ref(params["ref"])
            if getattr(self, "_menu", "none") == "tutorial":
                raise RpcError(-32000, "terminal menu is not open (top menu id 1059); activate the terminal first", method)
            if not isinstance(t, Thing) or t.kind != "terminal" or getattr(self, "_menu", "none") != "terminal":
                raise RpcError(-32000, "terminal menu is not open", method)
            names = t.topics_text or ["Read log", "Exit"]
            if not 0 <= params["entry"] < len(names):
                raise RpcError(-32000, "entry index out of range", method)
            if self.mutant != "terminal_noop":
                self.terminal_used.setdefault(params["ref"], []).append({"index": params["entry"], "text": names[params["entry"]]})
            return {"entries": names, "used": names[params["entry"]], "index": params["entry"]}
        if method == P.M_TERMINAL_STATE:
            return {"used": self.terminal_used.get(params["ref"], [])}
        if method == P.M_FASTTRAVEL:
            t = self._by_ref(params["marker"])
            if not isinstance(t, Thing):
                raise RpcError(-32000, "marker not loaded", method)
            if self.mutant != "setpos_ignored":
                self.me.pos, self.me.cell = [t.pos[0] + 40, t.pos[1], t.pos[2]], t.cell
            return {"genuine": False}
        if method == P.M_BOT_FASTTRAVEL:
            if not self.genuine_travel:
                raise RpcError(-32000, "unsupported: engine glue not wired", method)
            t = self._by_ref(params["marker"])
            if not isinstance(t, Thing):
                raise RpcError(-32000, "marker not loaded", method)
            if self.travel_refuses or self.me.cell in self.interior_cells:
                raise RpcError(-32000, "the engine refuses fast travel right now (combat)", method)
            if self.mutant not in ("travel_ignored", "setpos_ignored"):
                self.travel_due = (self.clock.t + 3.0, t)
            return {"genuine": True, "queued": True}
        if method == P.M_BOT_DROPS:
            if not self.drop_events:
                raise RpcError(-32000, "unsupported: the OnDrop event handler is not registered", method)
            since, base, window = params.get("since", 0), params.get("base", ""), params.get("windowMs", 5000) / 1000.0
            return {"seq": len(self.drop_log), "drops": [{k: v for k, v in d.items() if k != "at"} for d in self.drop_log
                                                        if d["seq"] > since and (not base or self._same_base(d["base"], base)) and self.clock.t - d["at"] <= window]}
        if method == P.M_REFS_OF:
            base = _hex(params["base"].split(":")[1])
            return {"refs": [form_name(t.id) for t in self.world.things.values()
                             if t.cell == self.me.cell and not t.disabled and not t.dead and (t.base_id or t.id) == base]}
        if method == P.M_COUNT_REFS:
            base = _hex(params["base"].split(":")[1])
            cell = self.me.cell
            return {"count": sum(1 for t in self.world.things.values()
                                 if t.cell == cell and not t.disabled and not t.dead and (t.base_id or t.id) == base)}
        raise RpcError(-32601, "unknown method: " + method, method)

    # ---- internals ----
    def _by_ref(self, form: str):
        if form == PLAYER_REF:
            return "player"
        if ":" not in form:   # a created ref's bare runtime id (form_name). UNVERIFIED: the real ref.info/activate.ref need Plugin.esm:HEX
            return self.world.things.get(_hex(form))
        name, local = form.rsplit(":", 1)
        return self.world.things.get(_hex(local))

    def _target(self, tok: str):
        if tok.lower() == "player":
            return "player"
        return self.world.things.get(_hex(tok))

    def _holder_items(self, t) -> Optional[Dict[int, int]]:
        if t == "player":
            return self.me.items
        return t.items if t is not None else None

    def _same_base(self, recorded: str, wanted: str) -> bool:
        """The bot.drops base filter. Mutant drops_base_text: the old text compare, which never matched in game."""
        return recorded == wanted if self.mutant == "drops_base_text" else same_form_name(recorded, wanted)

    def _activate(self, t: Thing, count0: bool = False) -> None:
        me = self.me
        if t.kind == "door":
            if t.locked:
                self._menu = "lockpick"     # the real game: activating a locked door opens the lockpick menu, which then eats the input
                return
            if t.load_to:   # a load door moves the player to the other cell; its open state stays closed (in game, 2026-10-21)
                me.cell = t.load_to
                return
            t.open_state = 1 if t.open_state == 3 else 3
        elif t.kind == "actor" and not t.dead:
            if self.mutant != "dialog_never":
                me.in_dialogue = t.id
                self.click_at = self.clock.t
        elif t.kind == "item":
            if self.mutant == "pickup_deletes":     # the real game: the picked-up item is deleted, not disabled
                t.deleted = True
            else:
                t.disabled = True
            if not (count0 and t.count0_gives_nothing):
                me.items[t.base_item] = me.items.get(t.base_item, 0) + 1
        elif t.kind in ("container", "terminal"):
            self._menu = t.kind
            if t.kind == "terminal" and self.tutorial_on_terminal and not self._tutorial_seen:
                self._tutorial_seen = True
                self._menu = "tutorial"          # the hint box is the top menu; the terminal menu is under it

    # ---- simulated input driver (plugin/bots): same method names and reply shapes as bot_rpc.h ----
    def _driver(self, method: str, params: Dict[str, Any]) -> Dict[str, Any]:
        if not self.input_driver:
            raise RpcError(-32601, "unknown method: " + method, method)
        me = self.me
        if method == P.M_BOT_STATUS:
            if self.mutant == "vats_freeze" and self.vats_state.get("phase") == "enter":
                raise RpcError(-32000, "main thread did not service the request (is the game loading?)", method)   # the frozen game of the first lab run
            return {"enabled": True, "hooked": True, "frame": int(self.clock.t * 60), "held": sorted(self.held),
                    "shotsLeft": self.shots_left, "alwaysRun": self.always_run, "vats": self.vats_state}
        if method == P.M_BOT_MOVE:
            want = bool(params.get("run", False))
            # bot_rpc.h speedGiven_: an omitted run holds no Run key, so the always-run toggle picks the speed
            speed_given = params.get("run") is not None or self.mutant == "run_omitted_is_false"
            if not speed_given:
                run_control = False
            else:
                run_control = want if self.mutant == "run_is_key" else want != self.always_run   # runControlFor in bot_rpc.h
            if self.mutant != "move_ignored":
                self.held -= {"Forward", "Backward", "Left", "Right", "Run"}
                f, st = float(params.get("forward", 0)), float(params.get("strafe", 0))
                if f > 0.3: self.held.add("Forward")
                if f < -0.3: self.held.add("Backward")
                if st > 0.3: self.held.add("Right")
                if st < -0.3: self.held.add("Left")
                if run_control: self.held.add("Run")
            return {"held": sorted(self.held), "run": want, "runControl": run_control, "alwaysRun": self.always_run}
        if method == P.M_BOT_STOP:
            if self.mutant != "stop_ignored":
                self.held.clear()
            self.shots_left = 0
            return {}
        if method == P.M_BOT_LOOK:
            if "ref" in params:
                t = self._by_ref(params["ref"])
                if not isinstance(t, Thing):
                    raise RpcError(-32000, "reference not loaded", method)
                z, x = heading_deg((me.pos[0], me.pos[1], me.pos[2] + 120), (t.pos[0], t.pos[1], t.pos[2] + 60))
            else:
                z, x = float(params.get("yaw", 0)), float(params.get("pitch", 0))
            if self.mutant != "look_ignored":
                me.angle["Z"], me.angle["X"] = z % 360.0, x
            return {"yaw": z, "pitch": x}
        if method == P.M_BOT_FIRE:
            n = int(params.get("shots", 1))
            if self.mutant != "fire_noop":
                self.shots_left = n
                self.shot_timer = 0.0
            return {"queued": n}
        if method == P.M_BOT_RELOAD:
            me.ammo = 12
            return {}
        if method == P.M_BOT_JUMP:
            if self.mutant != "jump_noop" and self.jump_t <= 0:
                self.jump_t, self.jump_base = 0.6, me.pos[2]
                self.jump_settle_until, self.jump_toggles = self.clock.t + JUMP_SETTLE_S, 0
            return {}
        if method == P.M_BOT_SNEAK:
            # In game (2026-10-21): within about 2 s of a jump only one crouch toggle takes effect; a second tap 1.1 s
            # later left the player crouched. Two seconds after the jump both taps toggle again.
            if self.clock.t < getattr(self, "jump_settle_until", 0.0):
                self.jump_toggles += 1
                if self.jump_toggles > 1:
                    return {}
            self.sneaking = not self.sneaking
            return {}
        if method == P.M_BOT_ACTIVATE:
            if self.mutant != "activate_noop":
                for t in self.world.things.values():
                    z, x = heading_deg((me.pos[0], me.pos[1], me.pos[2] + 120), (t.pos[0], t.pos[1], t.pos[2] + 60))
                    if (t.kind in ("door", "actor", "item") and math.dist(me.pos, t.pos) < self.activate_reach
                            and angle_diff(z, me.angle["Z"]) <= 10):
                        self._activate(t)
                        break
            return {}
        raise RpcError(-32601, "unknown method: " + method, method)

    def _blocked(self, x: float, y: float) -> bool:
        return any(x1 <= x <= x2 and y1 <= y <= y2 for x1, y1, x2, y2 in self.world.walls)

    def _advance(self, dt: float) -> None:
        """Move virtual time forward: locomotion from held controls, queued shots, jump arc."""
        me = self.me
        if self.travel_due and self.clock.t >= self.travel_due[0]:
            marker = self.travel_due[1]
            self.travel_due = None
            me.pos, me.cell = [marker.pos[0] + 40, marker.pos[1], marker.pos[2]], marker.cell
        t_end = self.clock.t
        left = dt
        while left > 1e-9:
            h = min(left, 0.02)
            left -= h
            if self._closing_until is not None and t_end - left >= self._closing_until:   # the closing box goes, the next one shows
                self._closing_until = None
                if self.native_boxes:
                    self.native_boxes.pop(0)
            while self.box_waves and t_end - left >= self.box_waves[0][0]:
                self.native_boxes.extend(self.box_waves.pop(0)[1])
            fwd = ("Forward" in self.held) - ("Backward" in self.held)
            side = ("Right" in self.held) - ("Left" in self.held)
            if (fwd or side) and not self.native_boxes:   # an engine message box is up: menu mode, the player does not move
                running = ("Run" in self.held) != self.always_run   # the engine's run decision: with always-run on, Run held = walk
                speed = 220.0 if running else 120.0
                yaw = math.radians(me.angle["Z"])
                dx = (math.sin(yaw) * fwd + math.cos(yaw) * side) * speed * h
                dy = (math.cos(yaw) * fwd - math.sin(yaw) * side) * speed * h
                nx, ny = me.pos[0] + dx, me.pos[1] + dy
                if not self._blocked(nx, ny):
                    me.pos[0], me.pos[1] = nx, ny
                elif not self._blocked(nx, me.pos[1]):   # slide along the wall like the engine does
                    me.pos[0] = nx
                elif not self._blocked(me.pos[0], ny):
                    me.pos[1] = ny
            if self.jump_t > 0:
                self.jump_t -= h
                me.pos[2] = self.jump_base + (80.0 * math.sin(math.pi * max(0.0, 1 - self.jump_t / 0.6)) if self.jump_t > 0 else 0.0)
            if self.shots_left > 0:
                self.shot_timer -= h
                if self.shot_timer <= 0:
                    self.shots_left -= 1
                    self.shot_timer = 0.18
                    self._shoot()

    def _shoot(self) -> None:
        me = self.me
        if me.ammo < 1:
            return
        me.ammo -= 1
        if self.mutant in ("fire_miss", "health_stuck"):
            return
        best = None
        for t in self.world.things.values():
            if t.kind != "actor" or t.dead or t.cell != me.cell:
                continue
            z, x = heading_deg((me.pos[0], me.pos[1], me.pos[2] + 120), (t.pos[0], t.pos[1], t.pos[2] + 60))
            if angle_diff(z, me.angle["Z"]) <= 6 and angle_diff(x, me.angle["X"]) <= 6:
                d = math.dist(me.pos, t.pos)
                if best is None or d < best[0]:
                    best = (d, t)
        if best:
            best[1].health -= 25

    def _stmt(self, line: str, self_ref=None) -> bool:
        """One statement. self_ref = the calling reference from exec's ref (a Thing, "player" or None)."""
        m = re.fullmatch(r"(?:(\w+)\.)?(\w+)(?:\s+(.*))?", line)
        if not m:
            return False
        tgt, cmd, rest = m.group(1), m.group(2).lower(), (m.group(3) or "").split()
        who = "player" if (tgt or "").lower() == "player" else self_ref
        me = self.me
        mut = self.mutant
        if cmd == "setpos" and tgt == "player":
            if mut != "setpos_ignored":
                me.pos["XYZ".index(rest[0].upper())] = float(rest[1])
            return True
        if cmd == "setangle" and tgt == "player":
            if mut != "angle_ignored":
                me.angle[rest[0].upper()] = float(rest[1]) % 360.0
            return True
        if cmd == "moveto" and tgt == "player":
            t = self._target(rest[0])
            if isinstance(t, Thing) and mut != "setpos_ignored":
                me.pos = [t.pos[0] + 40, t.pos[1], t.pos[2]]
                me.cell = t.cell
            return isinstance(t, Thing)
        if cmd == "equipitem" and tgt == "player":
            if mut != "equip_noop" and me.items.get(_hex(rest[0]), 0) > 0:
                me.equipped.add(_hex(rest[0]))
            return True
        if cmd == "unequipitem" and tgt == "player":
            me.equipped.discard(_hex(rest[0]))
            return True
        if cmd == "drop" and tgt == "player":
            i, n = _hex(rest[0]), int(rest[1])
            if mut != "drop_noop" and me.items.get(i, 0) >= n:
                me.items[i] -= n
                self._next_ref += 1
                ground = self.world.add(Thing(self._next_ref, "item", list(me.pos), me.cell, base_item=i, base_id=i))
                self.drop_log.append({"seq": len(self.drop_log) + 1, "ref": form_name(ground.id), "base": engine_form_name(i),
                                      "dropper": PLAYER_REF, "at": self.clock.t})
                if mut == "drop_wrong_ref":
                    self.drop_log[-1]["ref"] = "FalloutNV.esm:DEAD"
            return True
        if cmd == "activate":
            t = who
            if isinstance(t, Thing) and mut != "activate_noop":
                self._activate(t)
            return isinstance(t, Thing)
        if cmd == "removeitemtarget":
            src, dst = who, self._target(rest[1])
            si, di = self._holder_items(src), self._holder_items(dst)
            if si is None or di is None:
                return False
            i, n = _hex(rest[0]), int(rest[2])
            if mut == "transfer_noop" or si.get(i, 0) < n:
                return True
            if mut != "transfer_dup":
                si[i] -= n
            di[i] = di.get(i, 0) + n
            return True
        if cmd == "removeitem":
            items = self._holder_items(who)
            if items is None:
                return False
            i, n = _hex(rest[0]), int(rest[1])
            if items.get(i, 0) < n:
                return True
            items[i] -= n
            return True
        if cmd == "additem":
            items = self._holder_items(who)
            if items is None:
                return False
            items[_hex(rest[0])] = items.get(_hex(rest[0]), 0) + int(rest[1])
            return True
        if cmd in ("lock", "unlock"):
            if not isinstance(who, Thing) or who.kind not in ("door", "container", "terminal"):
                return False
            who.locked = cmd == "lock"
            return True
        if cmd == "setplayerteammate":
            if not isinstance(who, Thing) or who.kind != "actor":
                return False
            who.teammate = bool(int(rest[0]))
            return True
        if cmd == "placeatme":
            at = me if who == "player" else who
            if at is None:
                return False
            base = _hex(rest[0])
            for _ in range(int(rest[1]) if len(rest) > 1 else 1):
                self._next_ref += 1
                self.world.add(Thing(self._next_ref, "actor", list(at.pos), at.cell, base_id=base))
            return True
        if cmd == "fireweapon":
            return False  # the console fallback does not exist in the fake: tests the "unsupported" path
        return False

    def _expr(self, e: str, self_ref=None) -> Optional[float]:
        """One expression. self_ref = the calling reference from exec's ref (a Thing, "player" or None)."""
        m = re.fullmatch(r"(?:(\w+)\.)?(\w+)(?:\s+(.*))?", e)
        if not m:
            return None
        tgt, fn, rest = m.group(1), m.group(2).lower(), (m.group(3) or "").split()
        who = "player" if (tgt or "").lower() == "player" else self_ref
        me = self.me
        if fn == "issneaking":
            return 1.0 if self.sneaking else 0.0
        if fn == "isininterior":
            return 1.0 if me.cell in self.interior_cells else 0.0
        if fn == "getangle":
            return me.angle[rest[0].upper()]
        if fn == "getequipped":
            return 1.0 if _hex(rest[0]) in me.equipped else 0.0
        if fn == "getitemcount":
            items = self._holder_items(who)
            return float(items.get(_hex(rest[0]), 0)) if items is not None else None
        if fn == "gettype":
            return float(who.type_id) if isinstance(who, Thing) else None
        if fn == "getopenstate":
            return float(who.open_state) if isinstance(who, Thing) else None
        if fn == "getlocked":
            return float(who.locked) if isinstance(who, Thing) else None
        if fn == "getlocklevel":
            return (50.0 if who.locked else 0.0) if isinstance(who, Thing) else None
        if fn == "getplayerteammate":
            return float(who.teammate) if isinstance(who, Thing) else None
        if fn == "getstage":
            return float(self.world.stages.get(_hex(rest[0]), 0))
        if fn == "getav" and rest and rest[0].lower() == "health":
            if who == "player":
                return me.health
            return who.health if isinstance(who, Thing) and who.kind == "actor" else None
        return None
