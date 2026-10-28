"""Bot action layer: a scripted player driven only through nvbridge (never OS keyboard/mouse input).

Every action returns ActionResult(ok, action, reason, elapsed_s, data). With strict=True a failed action raises
ActionFailed instead. Every action has a timeout and checks a postcondition by reading the game back.
"""
from __future__ import annotations

import math
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple, Union

_RUNNER = str(Path(__file__).resolve().parent.parent / "runner")
if _RUNNER not in sys.path:
    sys.path.insert(0, _RUNNER)
from nvbridge import BridgeError, ConnectionLost, RpcError, WaitTimeout  # noqa: E402

from . import primitives as P  # noqa: E402

METHOD_NOT_FOUND = -32601
PLAYER_REF = "FalloutNV.esm:14"
# The refusal of the native message-box press while menu 1001 is still the top menu but its current message is gone
# (pressNativeMessageButton in plugin/bots/bot_engine.h). dismiss_messages waits and retries on this text only.
MSGBOX_CLOSING = "the message box has no current message (it is closing)"
# The bot.say refusals that clear by themselves (dialogPick in plugin/bots/bot_engine.h). choose_topic waits and retries on these only.
SAY_WAIT = ("click gate closed", "dialogue not ready")


class Pos(Tuple[float, float, float]):
    def __new__(cls, x: float, y: float, z: float):
        return super().__new__(cls, (float(x), float(y), float(z)))

    x = property(lambda s: s[0])
    y = property(lambda s: s[1])
    z = property(lambda s: s[2])


def dist(a: Tuple[float, float, float], b: Tuple[float, float, float]) -> float:
    return math.dist(a, b)


def heading_deg(frm: Tuple[float, float, float], to: Tuple[float, float, float]) -> Tuple[float, float]:
    """(angleZ, angleX) in degrees to look from `frm` at `to`. Z = atan2(dx, dy) (clockwise from +Y), X positive looks
    down. UNVERIFIED sign conventions: check once in game."""
    dx, dy, dz = to[0] - frm[0], to[1] - frm[1], to[2] - frm[2]
    z = math.degrees(math.atan2(dx, dy)) % 360.0
    x = -math.degrees(math.atan2(dz, math.hypot(dx, dy)))
    return z, x


def angle_diff(a: float, b: float) -> float:
    return abs((a - b + 180.0) % 360.0 - 180.0)


@dataclass
class ActionResult:
    ok: bool
    reason: str = ""
    data: Dict[str, Any] = field(default_factory=dict)
    action: str = ""
    elapsed_s: float = 0.0

    def __bool__(self) -> bool:
        return self.ok

    @property
    def code(self) -> str:
        """The short failure code ('stuck', 'timeout', 'too_close', ...). `reason` is 'code: what the bot saw' when the action could say why,
        so compare codes with this, never with the whole reason."""
        return self.reason.split(":", 1)[0]

    def __str__(self) -> str:
        return f"{self.action}: {'ok' if self.ok else 'FAILED ' + self.reason} ({self.elapsed_s:.2f}s) {self.data or ''}"


class ActionFailed(Exception):
    def __init__(self, result: ActionResult):
        super().__init__(str(result))
        self.result = result


class _Fail(Exception):
    def __init__(self, reason: str, say: str = "", **data: Any):
        """`reason` is the short code. `say` is what the bot saw (distance left, a menu in the way, ammo): the result's reason
        becomes 'code: say', which is the text a playbook prints when its step fails."""
        full = f"{reason}: {say}" if say else reason
        super().__init__(full)
        self.reason = full
        self.data = data


class RealClock:
    def monotonic(self) -> float:
        return time.monotonic()

    def sleep(self, s: float) -> None:
        time.sleep(s)


Ref = Union[str, int]
Target = Union[str, int, Tuple[float, float, float]]


class Bot:
    def __init__(self, nv, name: str = "bot", strict: bool = False, speed: float = 220.0, step_s: float = 0.15,
                 clock=None, slot: int = 0):
        self.slot = slot
        self.nv = nv
        self.name = name
        self.strict = strict
        self.speed = speed          # units per second while walking
        self.step_s = step_s        # seconds between position steps
        self.auto_dismiss = False   # fire() presses away a message box first (playbooks turn it on; proofs report boxes instead)
        self.ammo_item: Optional[Ref] = None   # the equipped weapon's ammo form: set it and fire() reports the reserve count before and after
        self.clock = clock or getattr(nv, "clock", None) or RealClock()
        self._plugins: Optional[List[str]] = None
        self._input: Optional[bool] = None
        self.log: List[ActionResult] = []

    # ------------------------------------------------------------------ plumbing
    def _act(self, action: str, fn: Callable[[], Dict[str, Any]]) -> ActionResult:
        t0 = self.clock.monotonic()
        try:
            data = fn() or {}
            res = ActionResult(True, "", data, action)
        except _Fail as f:
            res = ActionResult(False, f.reason, f.data, action)
        except WaitTimeout as e:
            res = ActionResult(False, "timeout", {"detail": str(e)}, action)
        except RpcError as e:
            res = ActionResult(False, f"rpc_error:{e.message}", {"code": e.code, "method": e.method}, action)
        except ConnectionLost:
            raise  # the game is gone: not an action failure, let the playbook see it (GameCrashed is a subclass)
        except BridgeError as e:
            res = ActionResult(False, f"bridge_error:{e}", {}, action)
        res.elapsed_s = self.clock.monotonic() - t0
        self.log.append(res)
        if self.strict and not res.ok:
            raise ActionFailed(res)
        return res

    def _plugin_list(self) -> List[str]:
        if self._plugins is None:
            self._plugins = list(self.nv.plugins())
        return self._plugins

    def _norm(self, ref: Optional[Ref]) -> Tuple[str, str, str]:
        """-> (info form 'plugin:HEX', console prefix, runtime hex id). None means the player."""
        if ref is None or ref == "player":
            return PLAYER_REF, "player", "00000014"
        plugins = self._plugin_list()
        if isinstance(ref, int) or (isinstance(ref, str) and ":" not in ref):
            rid = ref if isinstance(ref, int) else int(ref, 16)
            idx = rid >> 24
            if idx == 0xFF:   # a created ref (placeatme, a drop): no plugin owns it, the plugin names it by its runtime id
                return f"{rid:08X}", f"{rid:08X}", f"{rid:08X}"
            if idx >= len(plugins):
                raise _Fail("not_found", ref=ref, why="load index not in plugin list")
            return f"{plugins[idx]}:{rid & 0xFFFFFF:X}", f"{rid:08X}", f"{rid:08X}"
        name, local = ref.rsplit(":", 1)
        low = [p.lower() for p in plugins]
        if name.lower() not in low:
            raise _Fail("not_found", ref=ref, why="plugin not loaded")
        rid = (low.index(name.lower()) << 24) | int(local, 16)
        return f"{plugins[low.index(name.lower())]}:{int(local, 16):X}", f"{rid:08X}", f"{rid:08X}"

    def _item(self, item: Ref) -> str:
        return self._norm(item)[2]

    def _ref_of(self, ref: Optional[Ref]) -> str:
        """The calling reference of a console line: 'player' (the line keeps its 'player.' prefix) or 'Plugin.esm:HEX',
        which the bridge's exec {cmd, ref} resolves with the game's own load order."""
        info, prefix, _ = self._norm(ref)
        return "player" if prefix == "player" else info

    def _info(self, ref: Optional[Ref]) -> Dict[str, Any]:
        info_ref = self._norm(ref)[0]
        try:
            return self.nv.call(P.M_REF_INFO, {"ref": info_ref})
        except RpcError as e:
            if e.code == METHOD_NOT_FOUND:
                raise
            raise _Fail("not_found", ref=info_ref, why=e.message)

    def _eval(self, expr: Union[str, P.Line]) -> float:
        """Evaluate an expression: a plain str, or a P.Line whose ref is the calling reference (never '<hex>.Func')."""
        if isinstance(expr, P.Line):
            return self.nv.eval(expr.cmd) if expr.ref is None else self.nv.eval(expr.cmd, ref=expr.ref)
        return self.nv.eval(expr)

    def _optional_eval(self, expr: Union[str, P.Line]) -> Optional[float]:
        """_eval for a check that is only a diagnostic: None when the game (or a fake) cannot evaluate it."""
        try:
            return self._eval(expr)
        except Exception:  # noqa: BLE001
            return None

    def _optional_call(self, method: str, params: Optional[Dict[str, Any]] = None) -> Optional[Dict[str, Any]]:
        """Call a dev-only bridge method; None if the plugin does not have it."""
        try:
            return self.nv.call(method, params or {})
        except RpcError as e:
            if e.code == METHOD_NOT_FOUND:
                return None
            raise

    def _exec(self, cmd: Union[str, P.Line], what: str = "") -> str:
        """Run a console line: a plain str, or a P.Line whose ref is the calling reference (exec {cmd, ref})."""
        line = cmd if isinstance(cmd, P.Line) else P.Line(cmd)
        r = self.nv.exec(line.cmd) if line.ref is None else self.nv.exec(line.cmd, ref=line.ref)
        if not r.get("ok", False):
            extra = {} if line.ref is None else {"ref": line.ref}
            raise _Fail("unsupported", cmd=line.cmd, output=r.get("output", ""), what=what, **extra)
        return r.get("output", "")

    def _exec_on(self, ref: Optional[Ref], cmd: str, what: str = "") -> str:
        """Run `cmd` on `ref` with exec {cmd, ref} (the player keeps the 'player.' prefix). The console has no
        '<hexFormID>.Command' form, so never build one."""
        return self._exec(P.on(self._ref_of(ref), cmd), what)

    def _poll(self, pred: Callable[[], bool], timeout_s: float, poll_s: float = 0.1) -> bool:
        end = self.clock.monotonic() + timeout_s
        while True:
            if pred():
                return True
            if self.clock.monotonic() >= end:
                return False
            self.clock.sleep(poll_s)

    # ------------------------------------------------------------------ say why
    # A failed input action names its cause: the distance left, the menu that ate the input, the weapon and the ammo. These reads only
    # look (bot.menu, an expression); they never press, dismiss or move anything, so a proof that did not opt in still leaves boxes alone.
    def _menu_note(self) -> Dict[str, Any]:
        """What the game shows now: {menu: kind, menu_open: bool, msgs_queued?: n}; {} when bot.menu is missing or cannot answer.
        msgs_queued = boxes still waiting in the engine queue (msgCount - msgIdx of bot.menu)."""
        try:
            m = self._optional_call(P.M_BOT_MENU)
        except ConnectionLost:
            raise
        except (RpcError, BridgeError):
            return {}
        if not m:
            return {}
        kind = str(m.get("kind", "?"))
        note: Dict[str, Any] = {"menu": kind, "menu_open": kind not in P.MENU_FREE}
        if "id" in m:
            note["menu_id"] = m["id"]
        if "msgCount" in m:
            left = max(0, int(m["msgCount"]) - int(m.get("msgIdx", 0)))
            if left:
                note["msgs_queued"] = left
        return note

    def _menu_blocks(self) -> bool:
        """True when a menu that stops the player is up (P.MENU_BLOCKING; an unknown 'other' menu does not count)."""
        return self._menu_note().get("menu") in P.MENU_BLOCKING

    @staticmethod
    def _menu_words(note: Dict[str, Any]) -> str:
        """The menu part of a failure text: 'no menu is open', 'a message box is up (3 queued)', 'the terminal menu is open'."""
        kind = note.get("menu")
        if kind is None:
            return "menu state unknown"
        if not note.get("menu_open"):
            return "no menu is open"
        if kind == "message":
            q = note.get("msgs_queued")
            return "a message box is up" + (f" ({q} queued)" if q else "") + " and eats the input"
        if kind in P.MENU_BLOCKING:
            return f"the {kind} menu is open and eats the input"
        return f"the {kind} menu is open" + (f" (id {note['menu_id']:#x})" if isinstance(note.get("menu_id"), int) else "")

    def _weapon_note(self) -> Dict[str, Any]:
        """{weapon_out, weapon_equipped}: only the keys the game could answer. Both are diagnostics (UNVERIFIED expressions)."""
        out: Dict[str, Any] = {}
        drawn = self._optional_eval(P.is_weapon_out())
        if drawn is not None:
            out["weapon_out"] = drawn != 0
        slot = self._optional_eval(P.equipped_weapon())
        if slot is not None:
            out["weapon_equipped"] = slot != 0
        return out

    def _ammo_now(self) -> Optional[int]:
        """The reserve count of ammo_item (the loaded clip is not in it, so a shot can leave it unchanged until a reload); None when unknown."""
        if self.ammo_item is None:
            return None
        v = self._optional_eval(P.item_count("player", self._item(self.ammo_item)))
        return None if v is None else int(v)

    def _clear_message_boxes(self) -> None:
        """Opt-in pre-flight shared by the input actions: with auto_dismiss, press away a message box that would eat the input.
        Does nothing (not even a bot.menu read) when auto_dismiss is off."""
        if self.auto_dismiss and self.has_input_driver():
            # a message box (a script's ShowMessage, the DLC notices) keeps the game in menu mode and eats every Attack and Activate pulse
            kind = (self._optional_call(P.M_BOT_MENU) or {}).get("kind")
            if kind == "tutorial":
                self.close_tutorial()    # a hint box eats the input like a message box does
            if kind == "message":
                d = self.dismiss_messages(quiet_s=0.0, max_s=15.0)
                if not d.ok:
                    raise _Fail("menu", say="a message box is up and would not close: " + d.reason, why="a message box is up and would not close",
                                dismiss=d.reason)

    # ------------------------------------------------------------------ queries
    def pos(self, ref: Optional[Ref] = None) -> Pos:
        i = self._info(ref)
        return Pos(i["x"], i["y"], i["z"])

    def cell(self, ref: Optional[Ref] = None) -> int:
        return int(self._info(ref)["cell"])

    def alive(self, ref: Optional[Ref] = None) -> bool:
        """Not dead by the engine's own rule (P.LIFE_DEAD): a restrained or unconscious actor is alive."""
        return int(self._info(ref)["lifeState"]) not in P.LIFE_DEAD

    def health(self, ref: Optional[Ref] = None) -> float:
        """world.query first. In game (2026-10-21) world.query GetActorValue fails for every ref (Track A's own detour on
        Cond_GetActorValue fails its prologue check), so on an RPC error read 'GetAV Health' with the ref as the calling
        reference (eval {ref}, proven in game: 65.0)."""
        try:
            return float(self.nv.call(P.M_QUERY, {"ref": self._norm(ref)[0], "fn": "GetActorValue", "p1": P.HEALTH_AV})["value"])
        except RpcError:
            return float(self._eval(P.get_health(self._ref_of(ref))))

    def inventory_count(self, item: Ref, holder: Optional[Ref] = None) -> int:
        return int(self._eval(P.item_count(self._ref_of(holder), self._item(item))))

    def quest_stage(self, quest: Ref) -> int:
        return int(self._eval(P.get_stage(self._item(quest))))

    def dialogue_open(self) -> bool:
        return bool(self.nv.call(P.M_DIALOG_OPEN).get("open"))

    def door_state(self, ref: Ref) -> str:
        return P.OPEN_STATES.get(int(self._eval(P.open_state(self._ref_of(ref)))), "none")

    def locked(self, ref: Ref) -> bool:
        return self._eval(P.locked(self._ref_of(ref))) != 0

    def target_pos(self, target: Target) -> Pos:
        if isinstance(target, tuple):
            return Pos(*target)
        return self.pos(target)

    def snapshot(self, path: str, sections: Optional[List[str]] = None) -> ActionResult:
        return self._act("snapshot", lambda: dict(self.nv.state_to_file(path, sections), path=path))

    # ------------------------------------------------------------------ movement
    def _set_pos(self, p: Tuple[float, float, float]) -> None:
        for axis, v in zip("XYZ", p):
            self._exec(P.setpos(axis, v), "setpos")

    def _face(self, frm, to) -> None:
        z, _ = heading_deg(frm, to)
        self._exec(P.setangle("Z", z), "setangle")

    def has_input_driver(self) -> bool:
        """True if the game has the in-process input driver (plugin/bots, ini [dev] bots=1). Probed once."""
        if self._input is None:
            try:
                self._input = bool(self.nv.call(P.M_BOT_STATUS).get("hooked"))
            except RpcError as e:
                if e.code != METHOD_NOT_FOUND and "disabled" not in e.message:
                    raise
                self._input = False
        return self._input

    def goto(self, target: Target, radius: float = 100.0, timeout_s: float = 30.0, mode: str = "walk") -> ActionResult:
        """Move to a ref/coordinates. Post: within `radius` of the target in the same cell.
        mode 'walk'     real locomotion through the input driver when present (hold Forward+Run, steer with bot.look, stuck
                        detection), else stepped SetPos; 'input' same but fails with 'unsupported' if there is no driver;
        mode 'step'     stepped SetPos at `speed` (no animation, but works without the driver);
        mode 'teleport' MoveTo / SetPos. Walking across cells is refused (use 'teleport' or a door)."""
        def run():
            if mode not in ("walk", "input", "step", "teleport"):
                raise _Fail("bad_mode", mode=mode)
            dest = self.target_pos(target)
            is_ref = not isinstance(target, tuple)
            if is_ref and self.cell(target) != self.cell():
                if mode != "teleport":
                    raise _Fail("other_cell", mine=self.cell(), target=self.cell(target))
            if mode == "teleport":
                if is_ref:
                    self._exec(P.moveto(self._norm(target)[2]), "moveto")
                else:
                    self._set_pos(dest)
                self.clock.sleep(self.step_s)
                tol = max(radius, 250.0) if is_ref else radius
                d = dist(self.pos(), self.target_pos(target))
                if d > tol:
                    raise _Fail("postcondition", say=f"{d:.0f} units from the target after the teleport (wanted within {tol:g})", distance=d, wanted=tol)
                return {"distance": d}
            if mode == "input" and not self.has_input_driver():
                raise _Fail("unsupported", need="bot.move (ini [dev] bots=1)")
            use_input = mode == "input" or (mode == "walk" and self.has_input_driver())
            data = self._walk_input(target, radius, timeout_s) if use_input else self._walk_step(target, radius, timeout_s)
            if is_ref and self.cell(target) != self.cell():
                raise _Fail("postcondition", say="arrived in a different cell than the target", why="different cell on arrival")
            return data
        return self._act("goto", run)

    # bot.look runs SetAngle inside the plugin, and on the CLIENT copy of a co-op session it changes nothing (in game 2026-10-27: reply {yaw: 200}, the
    # angle read back unchanged, the avatar walked straight north for 1700 units), while the bridge's exec of the same line works on both copies. A
    # caller on a client copy sets this and the walk steers with the console line.
    look_via_console = False

    STANDING_STILL = 25.0       # a walk that gained less than this many units did not move the avatar at all

    def _walk_fail(self, code: str, d: float, radius: float, start: Optional[Pos] = None, **data: Any) -> _Fail:
        """A walk that did not arrive: how far it still was and whether a menu is open (a message box eats Forward, so the avatar stands still).
        With `start` it also says how far the avatar got from where the walk began: ~0 means the avatar never moved (input not reaching the
        game, or boxed in at the start), a few hundred means a wall or slope stopped it on the way. In game 2026-10-22 both players of a
        matrix run stood at their start while the NPC / container was 220..857 units away, and the message could not tell the two apart."""
        note = self._menu_note()
        say = f"{d:.0f} units left (wanted within {radius:g}), {self._menu_words(note)}"
        if start is not None:
            try:
                moved = dist(self.pos(), start)
            except Exception:       # the position read must never hide the failure
                moved = None
            if moved is not None:
                data["moved"] = moved
                say += (f", the avatar never left its start ({moved:.0f} units moved)" if moved < self.STANDING_STILL
                        else f", moved {moved:.0f} units from the start")
        return _Fail(code, say=say, distance=d, **note, **data)

    def _walk_step(self, target: Target, radius: float, timeout_s: float) -> Dict[str, Any]:
        end = self.clock.monotonic() + timeout_s
        stuck = 0
        start = self.pos()
        while True:
            cur = self.pos()
            dest = self.target_pos(target)  # moving targets are tracked
            d = dist(cur, dest)
            if d <= radius:
                return {"distance": d, "via": "step"}
            if self.clock.monotonic() >= end:
                raise self._walk_fail("timeout", d, radius, start=start)
            step = min(self.speed * self.step_s, d - radius * 0.5)
            f = step / d
            nxt = (cur[0] + (dest[0] - cur[0]) * f, cur[1] + (dest[1] - cur[1]) * f, cur[2] + (dest[2] - cur[2]) * f)
            self._face(cur, dest)
            self._set_pos(nxt)
            self.clock.sleep(self.step_s)
            if dist(self.pos(), cur) < 1.0:
                stuck += 1
                if stuck >= 4:
                    raise self._walk_fail("stuck", d, radius, start=start, at=tuple(cur))
            else:
                stuck = 0

    def _walk_input(self, target: Target, radius: float, timeout_s: float, turn_dps: float = 360.0) -> Dict[str, Any]:
        """Steer with bot.look (turn-rate limited) and hold Forward+Run through the input driver. Stuck detection: if the
        player gained < 25% of the expected progress over a window, try a sidestep, then a jump, then give up."""
        end = self.clock.monotonic() + timeout_s
        window_s, tries = 1.0, 0
        w_start, w_pos = self.clock.monotonic(), self.pos()
        start = w_pos
        yaw = self._eval(P.get_angle("Z")) % 360.0
        try:
            while True:
                cur = self.pos()
                dest = self.target_pos(target)
                d = dist(cur, dest)
                if d <= radius:
                    return {"distance": d, "via": "input", "unstick": tries}
                if self.clock.monotonic() >= end:
                    raise self._walk_fail("timeout", d, radius, start=start, unstick=tries)
                want, _ = heading_deg(cur, dest)
                delta = (want - yaw + 180.0) % 360.0 - 180.0
                maxturn = turn_dps * self.step_s
                yaw = (yaw + max(-maxturn, min(maxturn, delta))) % 360.0
                if self.look_via_console:
                    self._exec(P.setangle("Z", yaw), "setangle")
                else:
                    self.nv.call(P.M_BOT_LOOK, {"yaw": yaw, "pitch": 0})
                self.nv.call(P.M_BOT_MOVE, {"forward": 1, "strafe": 0, "run": True})
                self.clock.sleep(self.step_s)
                if self.clock.monotonic() - w_start >= window_s:
                    moved = dist(self.pos(), w_pos)
                    if moved < 0.25 * 200.0 * window_s and abs(delta) < 45.0:
                        if self._menu_blocks():   # standing still under a menu: sidesteps and jumps cannot help, say so now
                            raise self._walk_fail("stuck", d, radius, start=start, at=tuple(cur), unstick=tries)
                        tries += 1
                        if tries > 4:
                            raise self._walk_fail("stuck", d, radius, start=start, at=tuple(cur), unstick=tries)
                        if tries % 2:  # sidestep one way, later the other
                            side = 1 if (tries // 2) % 2 == 0 else -1
                            self.nv.call(P.M_BOT_MOVE, {"forward": 1, "strafe": side, "run": True})
                            self.clock.sleep(0.6)
                        else:
                            self.nv.call(P.M_BOT_JUMP)
                            self.clock.sleep(0.5)
                    w_start, w_pos = self.clock.monotonic(), self.pos()
        finally:
            try:
                self.nv.call(P.M_BOT_STOP)
            except RpcError:
                pass

    def walk_path(self, points: List[Tuple[float, float, float]], radius: float = 120.0, timeout_s: float = 60.0) -> ActionResult:
        """Path follower: walk waypoint to waypoint (each via goto, so the input driver when present). The last point uses
        `radius`; earlier points use twice that so the avatar does not stop and turn on every corner."""
        def run():
            end = self.clock.monotonic() + timeout_s
            for i, p in enumerate(points):
                last = i == len(points) - 1
                r = self.goto(tuple(p), radius=radius if last else radius * 2, timeout_s=max(1.0, end - self.clock.monotonic()))
                if not r.ok:
                    raise _Fail(r.reason, waypoint=i, **r.data)
            return {"waypoints": len(points)}
        return self._act("walk_path", run)

    # ---- direct controls through the input driver (all need plugin/bots) ----
    def _need_driver(self) -> None:
        if not self.has_input_driver():
            raise _Fail("unsupported", need="bot.* input driver (ini [dev] bots=1)")

    def look(self, yaw: Optional[float] = None, pitch: float = 0.0, ref: Optional[Ref] = None) -> ActionResult:
        """Turn the view. With `ref`, face it (yaw and pitch); else absolute yaw/pitch in degrees. Post: the angle reads back."""
        def run():
            self._need_driver()
            p: Dict[str, Any] = {"ref": self._norm(ref)[0]} if ref is not None else {"yaw": yaw or 0.0, "pitch": pitch}
            r = dict(self.nv.call(P.M_BOT_LOOK, p))
            gz = self._eval(P.get_angle("Z"))
            if angle_diff(gz, r["yaw"]) > 3.0:
                raise _Fail("postcondition", wanted=r["yaw"], got=gz)
            return r
        return self._act("look", run)

    def stop(self) -> ActionResult:
        """Release every forced control. Post: bot.status reports nothing held."""
        def run():
            self._need_driver()
            self.nv.call(P.M_BOT_STOP)
            held = self.nv.call(P.M_BOT_STATUS).get("held", [])
            if held:
                raise _Fail("postcondition", held=held)
            return {}
        return self._act("stop", run)

    def jump(self) -> ActionResult:
        """Tap Jump. Post: the player's Z rises by more than 5 units within 1.5 s."""
        def run():
            self._need_driver()
            z0 = self.pos().z
            self.nv.call(P.M_BOT_JUMP)
            if not self._poll(lambda: self.pos().z > z0 + 5.0, 1.5, 0.05):
                raise _Fail("postcondition", why="player never left the ground", z=z0)
            return {}
        return self._act("jump", run)

    def sneak_toggle(self) -> ActionResult:
        """Tap Crouch (a toggle). No reliable crouch-state read exists yet, so the result carries verified=False."""
        def run():
            self._need_driver()
            self.nv.call(P.M_BOT_SNEAK)
            self.clock.sleep(0.3)
            return {"verified": False}
        return self._act("sneak_toggle", run)

    def activate_crosshair(self) -> ActionResult:
        """Press Activate on whatever the crosshair is on (aim with look first). Engine key path, like a real key press.
        data carries the menu state after the press (a box that ate it shows as menu 'message'); with auto_dismiss a box is pressed away first."""
        def run():
            self._need_driver()
            self._clear_message_boxes()
            self.nv.call(P.M_BOT_ACTIVATE)
            self.clock.sleep(0.3)
            return {"verified": False, **self._menu_note()}
        return self._act("activate_crosshair", run)

    def follow(self, target: Ref, keep: float = 150.0, duration_s: float = 10.0) -> ActionResult:
        """Keep within `keep` units of a moving ref for duration_s. Post: ends within keep*1.5 of it."""
        def run():
            end = self.clock.monotonic() + duration_s
            while self.clock.monotonic() < end:
                d = dist(self.pos(), self.pos(target))
                if d > keep:
                    r = self.goto(target, radius=keep * 0.8, timeout_s=max(1.0, end - self.clock.monotonic()))
                    if not r.ok and r.code != "timeout":
                        raise _Fail(r.reason, **r.data)
                else:
                    self.clock.sleep(self.step_s)
            d = dist(self.pos(), self.pos(target))
            if d > keep * 1.5:
                raise _Fail("postcondition", distance=d, keep=keep)
            return {"distance": d}
        return self._act("follow", run)

    def wait(self, seconds: float) -> ActionResult:
        def run():
            self.clock.sleep(seconds)
            return {"waited": seconds}
        return self._act("wait", run)

    def wait_until(self, fn: Callable[[], bool], timeout_s: float = 10.0, poll_s: float = 0.1) -> ActionResult:
        def run():
            if not self._poll(fn, timeout_s, poll_s):
                raise _Fail("timeout")
            return {}
        return self._act("wait_until", run)

    # ------------------------------------------------------------------ combat
    def equip(self, item: Ref, timeout_s: float = 5.0) -> ActionResult:
        def run():
            h = self._item(item)
            if self.inventory_count(item) < 1:
                raise _Fail("not_in_inventory", item=h)
            self._exec(P.equip(h), "equip")
            if not self._poll(lambda: self._eval(P.is_equipped(h)) != 0, timeout_s):
                raise _Fail("postcondition", why="item not equipped", item=h)
            return {"item": h}
        return self._act("equip", run)

    def give(self, item: Ref, count: int = 1, timeout_s: float = 5.0) -> ActionResult:
        """Add `count` of `item` to the player (player.additem). Post: the inventory count rose by `count`."""
        def run():
            h = self._item(item)
            before = self.inventory_count(item)
            self._exec(f"player.additem {h} {int(count)}", "give")
            if not self._poll(lambda: self.inventory_count(item) >= before + count, timeout_s):
                raise _Fail("postcondition", why="item not added", item=h, before=before)
            return {"item": h, "count": int(count)}
        return self._act("give", run)

    def unequip(self, item: Ref, timeout_s: float = 5.0) -> ActionResult:
        def run():
            h = self._item(item)
            self._exec(P.unequip(h), "unequip")
            if not self._poll(lambda: self._eval(P.is_equipped(h)) == 0, timeout_s):
                raise _Fail("postcondition", why="item still equipped", item=h)
            return {"item": h}
        return self._act("unequip", run)

    def aim_at(self, ref: Ref, tolerance_deg: float = 3.0, eye_height: float = 120.0, target_height: float = 60.0,
               min_dist: float = 100.0, max_pitch: float = 80.0) -> ActionResult:
        """Set view angles toward `ref`. Post: the player's angles read back within tolerance.
        Refuses, with the reason, a target closer than `min_dist` units (a player standing inside the target aims at the floor, every
        shot misses) or one that needs more than `max_pitch` degrees of pitch (straight up or down). Codes: too_close, pitch."""
        def run():
            me = self.pos()
            tp = self.pos(ref)
            d = dist(me, tp)
            if d < min_dist:
                raise _Fail("too_close", say=f"the player is {d:.0f} units from the target (minimum {min_dist:g}); standing inside it "
                                             "aims at the floor, step back first", distance=d, min_dist=min_dist)
            z, x = heading_deg((me[0], me[1], me[2] + eye_height), (tp[0], tp[1], tp[2] + target_height))
            if abs(x) > max_pitch:
                raise _Fail("pitch", say=f"aiming at the target needs {x:.0f} degrees of pitch (limit {max_pitch:g}), distance {d:.0f}, "
                                         f"height difference {tp[2] + target_height - me[2] - eye_height:.0f}", pitch=x, distance=d, max_pitch=max_pitch)
            self._exec(P.setangle("Z", z), "setangle")
            self._exec(P.setangle("X", x), "setangle")
            gz = self._eval(P.get_angle("Z"))
            gx = self._eval(P.get_angle("X"))
            if angle_diff(gz, z) > tolerance_deg or angle_diff(gx, x) > tolerance_deg:
                note = self._menu_note()
                raise _Fail("postcondition", say=f"asked for angles Z {z:.1f} X {x:.1f}, the game reports Z {gz:.1f} X {gx:.1f}; {self._menu_words(note)}",
                            wanted=(z, x), got=(gz, gx), **note)
            return {"z": z, "x": x, "distance": d}
        return self._act("aim_at", run)

    def fire(self, ref: Optional[Ref] = None, shots: int = 1, expect_hit: bool = False) -> ActionResult:
        """Fire the equipped weapon. With `ref`: aim first and refuse a dead target. expect_hit=True requires the
        target's health to drop. Without bridge method bot.fire this falls back to a console line that cannot report ammo.
        A failure says why: the menu in the way, whether the weapon is drawn and equipped, and (with `ammo_item` set, or the bridge's own
        `ammo` in its reply) the ammo before and after, all in data and in the reason. data['ammo_before'] / ['ammo_after'] are reserve counts."""
        def run():
            before = None
            ammo_before: Optional[int] = None
            ammo_after: Optional[int] = None
            if ref is not None:
                if not self.alive(ref):
                    raise _Fail("dead", ref=str(ref))
                a = self.aim_at(ref)
                if not a.ok:
                    raise _Fail(a.reason, **a.data)
                before = self.health(ref)
            self._clear_message_boxes()
            ammo_before = self._ammo_now()

            def ammo_text() -> str:
                if ammo_before is None and ammo_after is None:
                    return ""
                return f"ammo {'?' if ammo_before is None else ammo_before} -> {'?' if ammo_after is None else ammo_after}"

            def ammo_data() -> Dict[str, Any]:
                return {k: v for k, v in (("ammo_before", ammo_before), ("ammo_after", ammo_after)) if v is not None}

            def diagnosis() -> Tuple[str, Dict[str, Any]]:
                """(text, data) for a shot that did nothing: menu, weapon drawn / equipped, ammo."""
                nonlocal ammo_after
                ammo_after = self._ammo_now()
                note = self._menu_note()
                weapon = self._weapon_note()
                parts = [self._menu_words(note)]
                if "weapon_out" in weapon:
                    parts.append("weapon drawn" if weapon["weapon_out"] else "weapon holstered")
                if weapon.get("weapon_equipped") is False:
                    parts.append("no weapon equipped")
                at = ammo_text()
                if at:
                    parts.append(at)
                return "; ".join(parts), {**note, **weapon, **ammo_data()}

            r = self._optional_call(P.M_BOT_FIRE, {"shots": shots, **({"ref": self._norm(ref)[0]} if ref is not None else {})})
            data: Dict[str, Any] = {}
            if r is None:
                for _ in range(shots):
                    self._exec(P.fire_console(), "fire")
                    self.clock.sleep(self.step_s)
                data["verified_shots"] = False
            else:
                n = int(r.get("fired", r.get("queued", 0)))
                if "ammo" in r:
                    data["ammo"] = r["ammo"]    # the bridge's own count after the shots
                if n < shots:
                    text, extra = diagnosis()
                    raise _Fail("postcondition", say=f"the bridge fired {n} of {shots} shots; {text}", why="fewer shots fired", fired=n,
                                wanted=shots, **extra)
                if "queued" in r:  # input driver: the pulses are played out over the next frames
                    if not self._poll(lambda: int(self.nv.call(P.M_BOT_STATUS).get("shotsLeft", 0)) == 0, shots * 0.5 + 3.0):
                        text, extra = diagnosis()
                        raise _Fail("timeout", say=f"shots still queued after {shots * 0.5 + 3.0:g} s; {text}", why="shots still queued", **extra)
                    self.clock.sleep(0.3)
                    # post-flight: an Attack pulse that reached the game drew the weapon; still holstered means no shot was taken
                    out = self._optional_eval(P.is_weapon_out())
                    if out is not None and out == 0:
                        text, extra = diagnosis()
                        raise _Fail("postcondition", say=f"weapon still holstered after the shots, the pulses never reached the game; {text}",
                                    why="weapon still holstered after the shots: the pulses never reached the game", **extra)
                data["fired"] = n
            ammo_after = self._ammo_now()
            data.update(ammo_data())
            if expect_hit:
                if before is None:
                    raise _Fail("bad_args", why="expect_hit needs ref")
                if not self._poll(lambda: self.health(ref) < before or not self.alive(ref), 3.0):
                    text, extra = diagnosis()
                    raise _Fail("postcondition", say=f"target health did not drop (stayed {before:g}); {text}", why="target health did not drop",
                                health=before, **extra)
                data["health"] = self.health(ref)
            return data
        return self._act("fire", run)

    def reload(self) -> ActionResult:
        def run():
            r = self._optional_call(P.M_BOT_RELOAD)
            if r is None:
                self._exec(P.reload_console(), "reload")
                return {"verified": False}
            if "reloaded" not in r:  # input driver: a key tap, nothing to read back yet
                self.clock.sleep(1.0)
                return {"verified": False}
            if not r.get("reloaded"):
                raise _Fail("postcondition", why="bridge says not reloaded", reply=r)
            return {"ammo": r.get("ammo")}
        return self._act("reload", run)

    def vats_target(self, ref: Ref, limb: Optional[str] = None, expect_hit: bool = False) -> ActionResult:
        """Enter VATS, pick the target, accept, play it out. data['hit'] says whether the target's health dropped;
        expect_hit=True makes a miss a failure (VATS can legitimately miss)."""
        def run():
            if not self.alive(ref):
                raise _Fail("dead", ref=str(ref))
            before = self.health(ref)
            p: Dict[str, Any] = {"ref": self._norm(ref)[0]}
            if limb:
                p["limb"] = limb
            r = self._optional_call(P.M_BOT_VATS, p)
            if r is None:
                raise _Fail("unsupported", need=P.M_BOT_VATS)
            if r.get("started"):   # real driver: a tick-driven job, poll its phase
                st = {}
                stuck = [0]

                def finished() -> bool:
                    # A game whose main thread no longer runs frames cannot tick the job (its own 25 s limit counts frames), and the bridge then answers
                    # every request with "main thread did not service the request". Three of those in a row end the wait with that reason instead of
                    # polling a frozen game for the rest of the limit (first in-game run: more than 50 s).
                    try:
                        st.update(self.nv.call(P.M_BOT_STATUS, timeout_s=6.0).get("vats", {}))
                        stuck[0] = 0
                    except RpcError as e:
                        if "did not service" not in str(e):
                            raise
                        stuck[0] += 1
                        if stuck[0] >= 3:
                            raise _Fail("main_thread_stuck", say="the game's main thread stopped running frames during VATS (bridge: main thread did not "
                                                                  f"service the request, {stuck[0]} times in a row); the job's last phase was {st.get('phase')!r}",
                                        phase=st.get("phase"))
                    return st.get("phase") in ("done", "failed")
                if not self._poll(finished, 30.0, 0.25):
                    raise _Fail("timeout", why="VATS job still running", phase=st.get("phase"))
                if st.get("phase") == "failed":
                    raise _Fail("vats_failed", why=st.get("reason", ""))
                hit = self.health(ref) < before or not self.alive(ref)
                if expect_hit and not hit:
                    raise _Fail("postcondition", why="VATS finished but the target was not hurt", health=before)
                return {"phase": "done", "hit": hit}
            if not r.get("queued"):
                raise _Fail("postcondition", why="VATS attack not queued", reply=r)
            return dict(r)
        return self._act("vats_target", run)

    # ---- menu / dialogue / inventory / refs (bridge methods in plugin/bots, see docs/tools/T6a-actions.md) ----
    def _try_call(self, method: str, params: Optional[Dict[str, Any]] = None) -> Optional[Dict[str, Any]]:
        """Like _optional_call, and also None when the plugin has the method but says "unsupported: ..." (engine glue or event handler missing)."""
        try:
            return self.nv.call(method, params or {})
        except RpcError as e:
            if e.code == METHOD_NOT_FOUND or "unsupported" in str(e).lower():
                return None
            raise

    def _need(self, method: str, params: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        r = self._optional_call(method, params)
        if r is None:
            raise _Fail("unsupported", need=method)
        return r

    def menu_info(self) -> Dict[str, Any]:
        """Current menu: {kind, id, vatsMode, dialogState?, dialogReady?}. kind in none|hud|message|pipboy|dialogue|container|terminal|vats|..."""
        return self._need(P.M_BOT_MENU)

    def dialog_choices(self) -> List[Dict[str, Any]]:
        return self._need(P.M_BOT_DIALOG)["choices"]

    def inventory(self, holder: Optional[Ref] = None, items: Optional[List[Ref]] = None) -> Dict[str, int]:
        """{form: count}. Without `items` it lists entries the engine tracks (reply 'partial'); pass items for exact totals."""
        p: Dict[str, Any] = {}
        if holder is not None:
            p["ref"] = self._norm(holder)[0]
        if items:
            p["items"] = [self._norm(i)[0] for i in items]
        return {i["item"]: i["count"] for i in self._need(P.M_BOT_INVENTORY, p)["items"]}

    def close_menu(self, kind: str = "container", timeout_s: float = 5.0) -> ActionResult:
        """Close the open menu from code, the engine's own way (bridge method bot.menu.close, CloseAllMenusCore 0x7037E0). Only a menu of `kind`
        ("container" by default; "terminal", "barter", ...; "any" = whatever a player can cancel out of, never a message box) is closed: another
        menu on top is refused and left open. Post: bot.menu reports no menu ("any") or a menu that is not `kind` any more."""
        def run():
            r = self._try_call(P.M_BOT_MENU_CLOSE, {"kind": kind})
            if r is None:
                raise _Fail("unsupported", need=P.M_BOT_MENU_CLOSE, why="this build cannot close a menu from code")
            if not r.get("closed"):
                raise _Fail("postcondition", why="bot.menu.close did not say closed", reply=r)

            def gone() -> bool:
                k = (self._optional_call(P.M_BOT_MENU) or {}).get("kind")
                return k in P.MENU_FREE if kind == "any" else k != kind
            if not self._poll(gone, timeout_s):
                raise _Fail("postcondition", say=f"the {kind} menu is still open {timeout_s:g} s after bot.menu.close", **self._menu_note())
            return {"closed": r.get("kind"), "menu": r.get("menu")}
        return self._act("close_menu", run)

    def close_container(self, ref: Optional[Ref] = None, timeout_s: float = 5.0) -> ActionResult:
        """Leave the container menu (the loot lock is released by the module when the menu is gone). `ref` is unused: the menu on top is closed."""
        def run():
            m = self._optional_call(P.M_BOT_MENU)
            if m is not None and m.get("kind") in P.MENU_FREE:
                return {"already": True}      # the loot module (a refused second opener) or a script already closed it
            r = self.close_menu("container", timeout_s)
            if not r.ok:
                raise _Fail(r.reason, **r.data)
            return dict(r.data)
        return self._act("close_container", run)

    def close_tutorial(self, timeout_s: float = 3.0) -> bool:
        """Close the first-use hint box (TutorialMenu 0x423) when it is the top menu. It lies over a terminal, VATS or lockpicking and eats every
        click and key until it is closed (measured on the second PC, 2026-10-27: the unlock terminal's click did nothing under it). True when no
        tutorial box is up afterwards. Does nothing, and asks the game nothing but bot.menu, when the build has no bot.menu.close."""
        try:
            if (self._optional_call(P.M_BOT_MENU) or {}).get("kind") != "tutorial":
                return True
            if self._try_call(P.M_BOT_MENU_CLOSE, {"kind": "tutorial"}) is None:
                return False
        except _Fail:
            return False
        return self._poll(lambda: (self._optional_call(P.M_BOT_MENU) or {}).get("kind") != "tutorial", timeout_s)

    def press_box(self, button: int) -> ActionResult:
        """Answer a message box with `button` (0-based, in the order the box lists its buttons). The UI layer's own box (consent prompt,
        PNR confirm) answers first; when it shows none and the engine's own message box (MessageMenu, menu id 1001) is the top menu, the
        plugin presses that box natively. data['path'] is 'ui' or 'native'; a native press also gives data['value'] (what the engine records)."""
        return self._act("press_box", lambda: dict(self._need(P.M_BOT_PRESS, {"button": button})))

    def dismiss_messages(self, quiet_s: float = 20.0, max_s: float = 120.0, max_n: int = 8, button: int = 0, settle_s: float = 0.3,
                         poll_s: float = 0.5, max_closing: int = 10) -> ActionResult:
        """Clear the message boxes the engine shows after a load (MessageMenu, menu id 1001), for example the DLC notices, which keep a
        test copy in menu mode. They come in waves: with all DLC, 6 boxes about 5 s after a fresh save loads, then 2, then 2 more about
        16 s later (first in-game run, 2026-10-21). So this keeps watching. While bot.menu reports kind 'message', it presses `button`
        with bot.menu.press (the UI layer's box first, else the engine's box natively) and waits `settle_s`. Otherwise it polls every
        `poll_s`. Done when no box has been shown for `quiet_s` seconds (quiet_s=0: only the boxes shown now).
        A box that is closing (menu 1001 is still the top menu but its current message is gone) refuses the press with MSGBOX_CLOSING.
        Then it waits `settle_s` and tries again. That retry is not a press. At most `max_closing` retries in a row, and the count starts
        again after each press. Any other refusal ends the dismissal (reason 'rpc_error:...').
        Fails with 'timeout' when `max_s` passes first (boxes keep coming), and with 'postcondition' when a box is still shown after
        `max_n` presses in a row (a box that does not close) or stays closing for more than `max_closing` retries.
        data: {pressed, paths, waited_s, quiet_s, closing}: waited_s = seconds from the call to the end of the quiet window (or to the
        failure), closing = the closing refusals retried. No proof calls this on its own (proofs: --dismiss-messages)."""
        def run():
            paths: List[str] = []
            t0 = self.clock.monotonic()
            last = t0        # the last time a box was shown; the quiet window starts at the call
            in_row = 0       # presses since a poll last saw no box
            closing = 0      # closing refusals since the last press
            retried = 0      # closing refusals in all
            while True:
                kind = self.menu_info().get("kind")
                now = self.clock.monotonic()
                waited = round(now - t0, 2)
                if kind == "message":
                    if in_row >= max_n:
                        raise _Fail("postcondition", why="a message box is still shown", pressed=len(paths), paths=paths, waited_s=waited,
                                    closing=retried)
                    if now - t0 >= max_s:
                        raise _Fail("timeout", why=f"message boxes still shown after {max_s:g} s", pressed=len(paths), paths=paths,
                                    waited_s=waited, closing=retried)
                    try:
                        r = self._need(P.M_BOT_PRESS, {"button": button})
                    except RpcError as e:
                        if MSGBOX_CLOSING not in e.message:
                            raise
                        closing += 1
                        retried += 1
                        if closing > max_closing:
                            raise _Fail("postcondition", why=f"a message box stayed closing for {closing} tries", pressed=len(paths),
                                        paths=paths, waited_s=waited, closing=retried)
                        self.clock.sleep(settle_s)
                        last = self.clock.monotonic()
                        continue
                    closing = 0
                    paths.append(str(r.get("path", "?")))
                    in_row += 1
                    self.clock.sleep(settle_s)
                    last = self.clock.monotonic()
                    continue
                in_row = 0
                if now - last >= quiet_s:
                    return {"pressed": len(paths), "paths": paths, "waited_s": waited, "quiet_s": quiet_s, "closing": retried}
                if now - t0 >= max_s:
                    raise _Fail("timeout", why=f"no {quiet_s:g} s without a message box within {max_s:g} s", pressed=len(paths), paths=paths,
                                waited_s=waited, closing=retried)
                self.clock.sleep(min(poll_s, quiet_s - (now - last), max_s - (now - t0)))
        return self._act("dismiss_messages", run)

    def consent(self, answer: str = "ready") -> ActionResult:
        """Answer the party-consent prompt: 'ready' or 'notyet'."""
        return self._act("consent", lambda: dict(self._need(P.M_BOT_CONSENT, {"answer": answer})))

    def fast_travel(self, marker: Ref, timeout_s: float = 30.0, allow_fallback: bool = True) -> ActionResult:
        """Fast travel to a map marker. Genuine path (bot.fasttravel): the engine's own travel object is queued, the player moves a few frames
        later (cell load included), data['genuine'] is True. A refusal from the engine (combat, busy menu, ...) is a failure with reason
        `refused` and is NOT retried as a teleport. Only when bot.fasttravel does not exist (engine glue off) the MoveTo fallback runs
        (test.fasttravel, data['genuine'] False). Post: the player is within 400 units of the marker."""
        def run():
            m = self._norm(marker)[0]
            try:
                r = self._try_call(P.M_BOT_FASTTRAVEL, {"marker": m})
            except RpcError as e:
                raise _Fail("refused", why=str(e))
            if r is None:
                if not allow_fallback:
                    raise _Fail("unsupported", need=P.M_BOT_FASTTRAVEL)
                r = self._need(P.M_FASTTRAVEL, {"marker": m})
                r = dict(r, genuine=False)
                timeout = 2.0
            else:
                r = dict(r, genuine=True)
                timeout = timeout_s
            if not self._poll(lambda: dist(self.pos(), self.pos(marker)) <= 400.0, timeout, poll_s=0.25):
                raise _Fail("postcondition", distance=dist(self.pos(), self.pos(marker)), genuine=r["genuine"])
            return dict(r)
        return self._act("fast_travel", run)

    def use_terminal(self, ref: Ref, entry: int, timeout_s: float = 5.0) -> ActionResult:
        """Open the terminal (if its menu is not already up) and choose menu entry `entry` (0-based, as listed on screen).
        Post: the bridge confirms the click and names the entry. data['entries'] lists what the menu showed."""
        def run():
            if self.menu_info().get("kind") != "terminal":
                r = self.open(ref, timeout_s)
                if not r.ok:
                    raise _Fail(r.reason, **r.data)
            if not self.close_tutorial():
                raise _Fail("postcondition", say="the tutorial box stays open over the terminal and eats the click; " + self._menu_words(self._menu_note()))
            r = self._need(P.M_TERMINAL, {"ref": self._norm(ref)[0], "entry": entry})
            self.clock.sleep(0.3)
            return dict(r)
        return self._act("use_terminal", run)

    def terminal_used(self, ref: Ref) -> List[Dict[str, Any]]:
        """Entries this bot picked on that terminal ([{index,text}]). The engine keeps no used flag, so humans' clicks are not included."""
        return self._need(P.M_TERMINAL_STATE, {"ref": self._norm(ref)[0]})["used"]

    def refs_of(self, base: Ref, cell: Optional[Ref] = None) -> Optional[List[str]]:
        """dev.refsOf: the enabled refs of a base form in a cell (default: the player's), or None when the plugin cannot list them."""
        p: Dict[str, Any] = {"base": self._norm(base)[0]}
        if cell is not None:
            p["cell"] = self._norm(cell)[0]
        r = self._try_call(P.M_REFS_OF, p)
        return None if r is None else list(r["refs"])

    def count_refs(self, base: Ref, cell: Optional[Ref] = None) -> int:
        """dev.countRefs: enabled refs of a base form in a cell (default: the player's cell)."""
        p: Dict[str, Any] = {"base": self._norm(base)[0]}
        if cell is not None:
            p["cell"] = self._norm(cell)[0]
        return int(self._need(P.M_COUNT_REFS, p)["count"])

    # ------------------------------------------------------------------ interaction
    def _do_activate(self, ref: Ref, console: bool = False) -> None:
        info = self._norm(ref)[0]
        if console:
            # The console's Activate calls the engine's Activate(activator, 0, 0, 1); the bridge method activate.ref passes 0 as the last argument, and the
            # engine then gives the player NOTHING for some placed items while it deletes the world ref (the lab's Stimpak, lab run 2026-10-27: both copies,
            # the loot sync untouched). Only a host can use this: a guest's console line is dropped by the script block.
            self._exec(P.on(self._ref_of(ref), "Activate player 1"), "activate")
            return
        try:
            r = self.nv.call(P.M_ACTIVATE, {"ref": info})
            if not r.get("ok", False):
                raise _Fail("activate_failed", reply=r)
        except RpcError as e:
            if e.code != METHOD_NOT_FOUND:
                raise
            self._exec(P.activate_console(info), "activate")

    def activate(self, ref: Ref) -> ActionResult:
        """Activate a ref as the player. Post: the engine accepted it. Use open/pickup/talk for state checks."""
        def run():
            self._info(ref)  # not_found if the ref is not loaded
            self._clear_message_boxes()
            try:
                self._do_activate(ref)
            except _Fail as f:
                if f.reason != "activate_failed":
                    raise
                note = self._menu_note()
                raise _Fail("activate_failed", say=f"the engine did not accept the activation; {self._menu_words(note)}", **f.data, **note)
            return {}
        return self._act("activate", run)

    def _kind(self, ref: Ref) -> int:
        """The form type of the ref's BASE object (P.TYPE_*). The game's ref.info answers the type of the reference itself: 0x3A for every placed
        object (door, chest, terminal, item), 0x3B for an actor. A door that read as 0x3A went down the container/terminal path of open(), which
        waits for a menu: it activated a locked door (lockpick menu left open) and reported an opened one as failed (lab run 2026-10-27). GetType
        gives the base form's type (door 28, container 27, terminal 23)."""
        t = int(self._info(ref)["type"])
        if t != P.TYPE_REFR:
            return t
        v = self._optional_eval(P.get_type(self._ref_of(ref)))
        return t if v is None or int(v) <= 0 else int(v)

    def open(self, ref: Ref, timeout_s: float = 5.0) -> ActionResult:
        """Open a door (post: GetOpenState reads open) or a container/terminal (post: bot.menu reports it)."""
        def run():
            kind = self._kind(ref)
            if kind == P.TYPE_DOOR:
                if self.door_state(ref) == "open":
                    return {"already": True}
                if self.locked(ref):
                    raise _Fail("locked")
                self._do_activate(ref)
                if not self._poll(lambda: self.door_state(ref) == "open", timeout_s):
                    raise _Fail("postcondition", state=self.door_state(ref))
                return {"state": "open"}
            self._do_activate(ref)
            m = self._optional_call(P.M_BOT_MENU)
            if m is None:
                raise _Fail("unsupported", need=P.M_BOT_MENU, why="cannot verify a container/terminal menu opened")
            def menu_up() -> bool:
                k = (self._optional_call(P.M_BOT_MENU) or {}).get("kind")
                if k == "tutorial":      # the first-use hint of a terminal opens over it: close it, the menu is under it
                    self.close_tutorial(1.5)
                    k = (self._optional_call(P.M_BOT_MENU) or {}).get("kind")
                return k in ("container", "terminal")
            if not self._poll(menu_up, timeout_s):
                note = self._menu_note()
                raise _Fail("postcondition", say=f"the container or terminal menu did not open; {self._menu_words(note)}",
                            **{"menu": m.get("kind"), **note})
            return {"menu": self._optional_call(P.M_BOT_MENU).get("kind")}
        return self._act("open", run)

    def close(self, ref: Ref, timeout_s: float = 5.0) -> ActionResult:
        def run():
            if self._kind(ref) != P.TYPE_DOOR:
                raise _Fail("unsupported", why="close is only for doors")
            if self.door_state(ref) == "closed":
                return {"already": True}
            self._do_activate(ref)
            if not self._poll(lambda: self.door_state(ref) == "closed", timeout_s):
                raise _Fail("postcondition", state=self.door_state(ref))
            return {"state": "closed"}
        return self._act("close", run)

    def _transfer(self, name: str, src: Optional[Ref], dst: Optional[Ref], item: Ref, count: int, timeout_s: float) -> Dict[str, Any]:
        # the source runs RemoveItemTarget (calling reference); the destination is an argument (runtime hex or "player")
        sh, dh, ih = self._ref_of(src), self._ref_of(dst), self._item(item)
        have = int(self._eval(P.item_count(sh, ih)))
        if have < count:
            raise _Fail("not_enough", have=have, wanted=count)
        dst_before = int(self._eval(P.item_count(dh, ih)))
        out = self._exec(P.transfer(sh, ih, self._norm(dst)[1], count), name)
        if "not found" in out.lower():
            # The second PC's build has no RemoveItemTarget (in game 2026-10-27 the console answers ok:true with 'Script command "RemoveItemTarget" not
            # found.' in the output, so nothing moved and the postcondition below failed at once). A take is then remove from the source + add to the
            # destination, as tools/bench/real_stage.py does for a corpse.
            self._exec(P.on(sh, f"RemoveItem {ih} {count} 1"), name + " (RemoveItem)")
            self._exec(P.on(dh, f"AddItem {ih} {count} 1"), name + " (AddItem)")
        if not self._poll(lambda: int(self._eval(P.item_count(dh, ih))) == dst_before + count, timeout_s):
            raise _Fail("postcondition", why="destination count wrong", before=dst_before, wanted=count,
                        got=int(self._eval(P.item_count(dh, ih))))
        left = int(self._eval(P.item_count(sh, ih)))
        if left != have - count:
            raise _Fail("postcondition", why="source count wrong (item duplicated?)", before=have, left=left)
        return {"moved": count, "left": left}

    def take(self, container: Ref, item: Ref, count: int = 1, timeout_s: float = 5.0) -> ActionResult:
        return self._act("take", lambda: self._transfer("take", container, None, item, count, timeout_s))

    def put(self, container: Ref, item: Ref, count: int = 1, timeout_s: float = 5.0) -> ActionResult:
        return self._act("put", lambda: self._transfer("put", None, container, item, count, timeout_s))

    def pickup(self, ref: Ref, item: Optional[Ref] = None, timeout_s: float = 5.0, console: bool = False) -> ActionResult:
        """Activate a world item. Post: the world ref is gone/disabled and, if `item` is given, its count rose by 1. `console`: activate with the console line
        (host only) instead of activate.ref, see _do_activate."""
        def run():
            before = self.inventory_count(item) if item is not None else None
            self._info(ref)
            self._do_activate(ref, console)

            def gone() -> bool:
                try:
                    i = self._info(ref)
                except _Fail:
                    return True
                return bool(int(i["flags"]) & P.FLAGS_GONE)
            if not self._poll(gone, timeout_s):
                raise _Fail("postcondition", why="world ref still present")
            if item is not None and self.inventory_count(item) < before + 1:
                raise _Fail("postcondition", why="inventory did not gain the item", before=before)
            return {}
        return self._act("pickup", run)

    def drop(self, item: Ref, count: int = 1, timeout_s: float = 5.0) -> ActionResult:
        def run():
            h = self._item(item)
            have = self.inventory_count(item)
            if have < count:
                raise _Fail("not_enough", have=have, wanted=count)
            base = self._norm(item)[0]
            log = self._try_call(P.M_BOT_DROPS, {})                # the engine's OnDrop event log (native handler), position before the drop
            before = None if log is not None else self.refs_of(item)
            self._exec(P.drop(h, count), "drop")
            if not self._poll(lambda: self.inventory_count(item) == have - count, timeout_s):
                raise _Fail("postcondition", have=have, now=self.inventory_count(item))
            out: Dict[str, Any] = {"dropped": count, "ref": None, "refs": [], "source": None}
            if log is not None:
                # correlate by base form plus the event's own time window; the event may land a frame after the count changes
                def seen():
                    d = self._try_call(P.M_BOT_DROPS, {"since": log["seq"], "base": base})
                    return d["drops"] if d and d["drops"] else None
                if self._poll(lambda: seen() is not None, 1.0, poll_s=0.05):
                    refs = [d["ref"] for d in seen()]
                    out.update(ref=refs[-1], refs=refs, source="event")
            if out["ref"] is None and before is not None:
                after = self.refs_of(item) or []
                new = [r for r in after if r not in before]       # fallback: what appeared in the cell since
                if new:
                    out.update(ref=new[-1], refs=new, source="scan")
            return out
        return self._act("drop", run)

    def talk(self, npc: Ref, timeout_s: float = 5.0) -> ActionResult:
        """Activate an NPC. Post: the dialogue menu is open."""
        def run():
            if not self.alive(npc):
                raise _Fail("dead", ref=str(npc))
            if self.dialogue_open():
                raise _Fail("already_in_dialogue")
            self._clear_message_boxes()
            self._do_activate(npc)
            if not self._poll(self.dialogue_open, timeout_s):
                note = self._menu_note()
                d = dist(self.pos(), self.pos(npc))
                raise _Fail("postcondition", say=f"the dialogue menu did not open; the NPC is {d:.0f} units away, {self._menu_words(note)}",
                            why="dialogue menu did not open", distance=d, **note)
            return {}
        return self._act("talk", run)

    def choose_topic(self, info_id: Ref, timeout_s: float = 5.0, poll_s: float = 0.1) -> ActionResult:
        """Speak one dialogue line by INFO form id. Needs bridge method bot.say (no console primitive exists).
        bot.say refuses while the engine's click gate is closed (500 ms after a click, the talk activation included) and while the
        NPC still speaks (dialogue state not 2). A player waits there too, so this tries again every `poll_s` until `timeout_s`
        (in game 2026-10-21: the first bot.say right after talk was refused by the click gate). Other refusals fail at once."""
        def run():
            if not self.dialogue_open():
                raise _Fail("no_dialogue")
            end = self.clock.monotonic() + timeout_s
            while True:
                try:
                    r = self._optional_call(P.M_BOT_SAY, {"info": self._norm(info_id)[0]})
                    break
                except RpcError as e:
                    left = end - self.clock.monotonic()
                    if not any(w in e.message for w in SAY_WAIT) or left <= 0:
                        raise
                    if poll_s >= left:            # the next try would come after timeout_s: give up now (PR #14 review)
                        self.clock.sleep(left)
                        raise
                    refusal = e
                self.clock.sleep(poll_s)
                if not self.dialogue_open():
                    raise _Fail("no_dialogue", why="the dialogue closed while bot.say waited")
                if self.clock.monotonic() >= end:   # the sleep or the dialogue check used up the time left (PR #22 review)
                    raise refusal
            if r is None:
                raise _Fail("unsupported", need=P.M_BOT_SAY)
            if not r.get("said"):
                raise _Fail("postcondition", why="line not said (condition false or wrong NPC?)", reply=r)
            return {"info": self._norm(info_id)[0], "speech_check": bool(r.get("speechCheck", False))}
        return self._act("choose_topic", run)

    def end_dialogue(self, timeout_s: float = 5.0) -> ActionResult:
        def run():
            if not self.dialogue_open():
                return {"already": True}
            self.nv.call(P.M_DIALOG_CLOSE)
            if not self._poll(lambda: not self.dialogue_open(), timeout_s):
                raise _Fail("postcondition", why="dialogue still open")
            return {}
        return self._act("end_dialogue", run)

    # ------------------------------------------------------------------ aliases for T6b playbooks
    def goto_ref(self, ref: Ref, dist: float = 150.0, timeout_s: float = 30.0, mode: str = "walk") -> ActionResult:
        return self.goto(ref, radius=dist, timeout_s=timeout_s, mode=mode)

    def goto_pos(self, x: float, y: float, z: float, cell: Optional[int] = None, timeout_s: float = 30.0,
                 mode: str = "walk") -> ActionResult:
        r = self.goto((x, y, z), radius=100.0, timeout_s=timeout_s, mode=mode)
        if r.ok and cell is not None and self.cell() != cell:
            r = ActionResult(False, "postcondition", {"cell": self.cell(), "wanted": cell}, "goto_pos", r.elapsed_s)
            self.log.append(r)
        return r

    def open_door(self, ref: Ref, timeout_s: float = 5.0) -> ActionResult:
        return self.open(ref, timeout_s)

    def close_door(self, ref: Ref, timeout_s: float = 5.0) -> ActionResult:
        return self.close(ref, timeout_s)

    def open_container(self, ref: Ref, timeout_s: float = 5.0) -> ActionResult:
        return self.open(ref, timeout_s)

    def talk_to(self, npc: Ref, timeout_s: float = 5.0) -> ActionResult:
        return self.talk(npc, timeout_s)

    def pick_topic(self, info_id: Ref, timeout_s: float = 5.0) -> ActionResult:
        return self.choose_topic(info_id, timeout_s)

    def target_alive(self, ref: Ref) -> bool:
        return self.alive(ref)

    def is_dead(self, ref: Ref) -> bool:
        return not self.alive(ref)

    def in_dialogue(self) -> bool:
        return self.dialogue_open()

    def stage_done(self, quest: Ref, stage: int) -> bool:
        return bool(self._eval(f"GetStageDone {self._item(quest)} {stage}"))

    def ref_enabled(self, ref: Ref) -> bool:
        return not (int(self._info(ref)["flags"]) & P.FLAGS_GONE)

    def coop_status(self) -> Dict[str, Any]:
        return self.nv.call("coop.status")
