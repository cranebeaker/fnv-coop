"""Every console line and bridge method the bot layer uses, in one place.

The console grammar here has NOT been run against the real game yet (code-only sprint). Each entry is tagged:
  VERIFIED   - already used by existing tools/tests in this repo
  UNVERIFIED - written from NVSE/console knowledge; the lead's first in-game run may need to fix the text here
Fixing a string in this file is the only change needed; the Bot code and the fake bridge tests keep working as long
as the fake bridge (fake.py) is updated to the same grammar.

A line for a reference other than the player is a Line(cmd, ref): the bridge runs `cmd` with `ref` as the console's
calling reference (exec {cmd, ref}). VERIFIED in game on 2026-10-21: the console does not accept "<hexFormID>.Command"
(Script command "00103E61.GetOpenState" not found); "player." lines work and stay plain strings.
"""
from __future__ import annotations

from typing import NamedTuple, Optional


class Line(NamedTuple):
    """One console line and its calling reference (None: no reference, for example 'player.X' or 'GetStage Q')."""
    cmd: str
    ref: Optional[str] = None


def on(holder: str, text: str) -> Line:
    """`text` run on `holder`. 'player' keeps the 'player.' prefix (a valid script reference). Any other holder
    ('Plugin.esm:HEX' or a runtime hex id) becomes the exec ref."""
    if holder.lower() == "player":
        return Line(f"player.{text}")
    return Line(text, holder)

# ---- bridge methods (existing, VERIFIED to be registered in plugin/) ----
M_REF_INFO = "ref.info"            # {ref:"plugin:HEX"} -> {x,y,z,lifeState,flags,type,cell}
M_ACTIVATE = "activate.ref"        # {ref} -> {ok, returned}; activator is the host player
M_QUERY = "world.query"            # {ref, fn, p1} -> {value}
M_DIALOG_OPEN = "ui.dialogopen"    # -> {open}
M_DIALOG_CLOSE = "ui.closedialog"  # -> {closed}

# ---- dev-only bridge methods the bots need (plugin/bots, ini [dev] bots=1); see docs/tools/T6a-actions.md ----
M_BOT_FIRE = "bot.fire"        # {ref?, shots} -> {fired, ammo}
M_BOT_RELOAD = "bot.reload"    # {} -> {reloaded, ammo}
M_BOT_VATS = "bot.vats"        # {ref, limb?} -> {queued, hit}
M_BOT_SAY = "bot.say"          # {info: "plugin:HEX"} -> {said}
M_BOT_MENU = "bot.menu"        # {} -> {kind: "none|dialogue|container|terminal|pipboy|...", ref?}
M_BOT_INVENTORY = "bot.inventory"  # {ref?} -> {items:[{item, count}]}
# in-process input driver (plugin/bots/bot_rpc.h)
M_BOT_MENU_CLOSE = "bot.menu.close"   # {kind?="container"|"any"} -> {closed, menu, kind}: CloseAllMenusCore, only when the top menu is that kind
M_BOT_MENU_PRESS = "bot.menu.press"   # {button} -> {menu, button, path: ui|native, value?, buttons?}
M_BOT_PRESS = M_BOT_MENU_PRESS
M_BOT_CONSENT = "bot.consent"          # {answer: ready|notyet}
M_BOT_DIALOG = "bot.dialog"            # -> {choices:[{idx,formId,info}]}
M_FASTTRAVEL = "test.fasttravel"       # {marker} -> {genuine:false}: a MoveTo teleport, the fallback
M_BOT_FASTTRAVEL = "bot.fasttravel"    # {marker} -> {genuine:true, queued}: the map menu's own travel path
M_BOT_DROPS = "bot.drops"              # {since?, base?, windowMs?} -> {seq, drops:[{seq,ref,base,dropper}]} from the OnDrop event
M_REFS_OF = "dev.refsOf"               # {base, cell?} -> {refs:[..], count}
M_TERMINAL = "dev.terminal"           # {ref, entry} -> {entries, used, index}
M_TERMINAL_STATE = "dev.terminalState" # {ref} -> {used:[{index,text}]}
M_COUNT_REFS = "dev.countRefs"         # {base, cell?} -> {count}
M_BOT_STATUS = "bot.status"    # -> {enabled, hooked, frame, held:[..], shotsLeft}
M_BOT_MOVE = "bot.move"        # {forward, strafe, run?, ms?}
M_BOT_STOP = "bot.stop"
M_BOT_LOOK = "bot.look"        # {yaw, pitch} | {ref}
M_BOT_JUMP = "bot.jump"
M_BOT_SNEAK = "bot.sneak"
M_BOT_ACTIVATE = "bot.activate"

HEALTH_AV = 16                 # actor value code of Health (decimal); UNVERIFIED for world.query p1

# FNV TESForm typeIDs, from world.actors docs in hooks_world
TYPE_ACTOR = 0x3B
TYPE_CONTAINER = 0x1B
TYPE_DOOR = 0x1C
TYPE_REFR = 0x3A               # what ref.info answers for a placed object: the type of the reference, not of its base (GetType gives the base's)
FLAG_DISABLED = 0x800          # UNVERIFIED TESForm flag bit for "initially disabled / disabled"
FLAG_DELETED = 0x20            # TESForm "deleted": a world item that was picked up shows this and NOT the disabled bit (in game 2026-10-27, ref.info flags
                               # 0x408 -> 0x4032A after activate.ref took it; the pickup postcondition watched only the disabled bit and failed on a pickup that worked)
FLAGS_GONE = FLAG_DISABLED | FLAG_DELETED

# Actor life state (ref.info lifeState, Actor+0x108): 0 alive, 1 dying, 2 dead, 3 unconscious, 5 restrained, 6 essential unconscious.
# Actor::IsDead (0x8844F0, called with false) counts 1, 2 and 6 as dead. 3 and 5 are alive: in game (2026-10-21) Doc Mitchell
# read 5 after SetRestrained 1.
LIFE_DEAD = frozenset({1, 2, 6})

# bot.menu kinds in which the game takes input normally. Any other kind (message, dialogue, pipboy, loading, ...) means a menu is open.
MENU_FREE = frozenset({"none", "hud"})
# bot.menu kinds that stop the player from moving, shooting and activating. Not "other" (a menu id the plugin does not know): that may be a
# HUD overlay, so a walk that stands still under it still tries its sidesteps.
MENU_BLOCKING = frozenset({"message", "dialogue", "container", "terminal", "pipboy", "barter", "hacking", "lockpick", "pause", "loading", "vats", "tutorial"})

# GetOpenState return values, UNVERIFIED
OPEN_STATES = {1: "open", 2: "opening", 3: "closed", 4: "closing"}


def setpos(axis: str, v: float) -> str:        # UNVERIFIED
    return f"player.SetPos {axis} {v:.2f}"


def setangle(axis: str, deg: float) -> str:   # UNVERIFIED
    return f"player.SetAngle {axis} {deg:.2f}"


def get_angle(axis: str) -> str:              # UNVERIFIED (expression)
    return f"player.GetAngle {axis}"


def moveto(hexid: str) -> str:                # VERIFIED name, UNVERIFIED use on player from console
    return f"player.MoveTo {hexid}"


def equip(item: str) -> str:                  # UNVERIFIED
    return f"player.EquipItem {item}"


def unequip(item: str) -> str:                # UNVERIFIED
    return f"player.UnequipItem {item}"


def is_equipped(item: str) -> str:            # UNVERIFIED (expression)
    return f"player.GetEquipped {item}"


def is_weapon_out() -> str:                   # expression: 1 while the player's weapon is drawn (checked in game 2026-10-21)
    return "player.IsWeaponOut"


def equipped_weapon() -> str:                 # UNVERIFIED (expression): NVSE GetEquippedObject, slot 5 = weapon; 0 = nothing equipped.
    # Only used to explain a failure, only 0 / not 0 is read. The bare "player.GetEquippedObject 5" returns a Form and printc refuses it
    # ("Invalid expression for parameter 2. Expected variable (got Form)", in game 2026-10-22): `&& 1` turns the form into a number.
    return "(player.GetEquippedObject 5) && 1"


def item_count(holder: str, item: str) -> Line:  # UNVERIFIED (expression); holder is "player" or the holder's ref
    return on(holder, f"GetItemCount {item}")


def transfer(src: str, item: str, dst: str, count: int) -> Line:   # UNVERIFIED: NVSE RemoveItemTarget, run on src
    return on(src, f"RemoveItemTarget {item} {dst} {count} 1")     # dst: "player" or a runtime hex id (an argument)


def drop(item: str, count: int) -> str:       # UNVERIFIED
    return f"player.Drop {item} {count}"


def activate_console(ref: str) -> Line:       # UNVERIFIED fallback when activate.ref is missing
    return Line("Activate player 1", ref)


def get_type(ref: str) -> Line:               # expression; verified in game 2026-10-27: door 28, terminal 23, container 27, item 47, NPC 42
    return Line("GetType", ref)


def open_state(ref: str) -> Line:             # UNVERIFIED (expression)
    return Line("GetOpenState", ref)


def locked(ref: str) -> Line:                 # UNVERIFIED (expression)
    return Line("GetLocked", ref)


def get_health(holder: str) -> Line:          # VERIFIED in game 2026-10-21 (eval with ref: Doc Mitchell -> 65.0)
    return on(holder, "GetAV Health")


def get_stage(quest: str) -> str:             # VERIFIED (used by nvbridge.stage)
    return f"GetStage {quest}"


def fire_console() -> str:                    # UNVERIFIED fallback when bot.fire is missing
    return "player.FireWeapon"


def reload_console() -> str:                  # UNVERIFIED fallback when bot.reload is missing
    return "player.ReloadWeapon"
