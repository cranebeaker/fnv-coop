# Scripts on items: their own runs, `OnAdd`, `RemoveMe`

An item record can carry a script (`SCRI` on the item). The official
master has 201 `OnAdd` blocks: ammunition bundles that give their rounds
and remove themselves (`Case10mmAddScript`: `Player.AddItem Case10mm 25`,
`RemoveMe`), schematics and recipes that give a note, key cards, and the
main quest's note from Novac (`VMQ01NoteObjectSCRIPT`: objectives,
`VStoryEventKnowsKhansInvolved`, `RemoveMe`). Until 2026-10-06 nv-rs ran
none of them, and `RemoveMe` (92 calls) stopped any script that reached it.

## Read from `FalloutNV.exe` 1.4.0.525

| What | PC |
| --- | --- |
| Script block table (name, block type, its test) | `0118e2f0`, 0x28 bytes an entry |
| `OnAdd`'s test: the event's bit 1, the block's argument the container | `005ca200` → `005ca1c0` → `005a8ef0` |
| Setting an event on an item's script | `005ac750(container, extra list, mask)` → `005a8e20` |
| A container's `AddItem` (sets `OnAdd`, mask 1) | `00574fa0`; pick-up `00574b30` |
| Adding items with a notice (scripted items one at a time, each a script of its own) | `004821a0` |
| A reference's script run: its own, then a container's or actor's items' | `00565870` → `004d2480` |
| Running a script (events cleared afterwards) | `005ac1e0` → `005e2590` → `005e0d20` (`005a8ea0`) |
| `RemoveMe`, `DropMe` | `005b53d0`, `005b5860` (command table `011922d8`, `01192300`) |

Event bits set by the code (`005ac750`'s callers): 1 `OnAdd`, 2 `OnEquip`,
4 `OnDrop`, 8 `OnUnequip`.

- Adding a scripted item flags `OnAdd` on that item's script with the
  container (`00574fa0`); the block runs on the container's next script
  run (`00565870`, for actors and containers `004d2480` runs every
  scripted stack's script with the container), its argument empty or the
  container (`OnAdd Player`), then the events are cleared.
- `004821a0` adds scripted items one at a time, each with a fresh script
  (`0055a2f0`) run once before the add.
- `RemoveMe` (`005b53d0`): the container the script runs in removes one of
  the item (`RemoveItem`, vtable `+0x17C`, the item's own stack), and the
  inventory run stops (`00952c30`). `DropMe` drops it into the world the
  same way. On an actor wearing the item (`00575400`: `004bfda0(item, 0)`
  finds an instance with extra data 0x16 or 0x17, worn) that worn instance
  is the one removed, and removing a worn instance takes it off first
  (`004c37d0`: `0088d7d0` for the player, the actor's `+0x188` otherwise);
  otherwise one of the others goes.

## Here (`world::scripting`)

Each scripted item gets a script of its own (`GameState::item_scripts`:
holder, item, its variables, its events waiting), made when it's added at
runtime (`GameState::added`: script `AddItem`, leveled lists through it,
`AddItemHealthPercent`, picking up, a person picking up food) with `OnAdd`
for the holder; moved between holders (`move_item`, `take_all`,
`RemoveAllItems` into a container) it keeps its variables and sees `OnAdd`
again; equipping (`equip`) and unequipping (`unequip_item`: the Pip-Boy,
scripts, selling, giving away) add `OnEquip` / `OnUnequip`. A holder's
own contents get scripts only when they move or are equipped.

`Runner::run_item_scripts`, at each `update`: items no longer held lose
their scripts; every item the player holds runs, and any other with events
waiting; a run takes the script's blocks in order: `GameMode`, and the
event blocks whose events wait and whose argument is empty or names the
event's reference; the holder is the reference it runs on; the events are
then cleared. `RemoveMe` takes one of the item from its holder, and its
script with it; when the holder wears the item it's taken off first
(`OnUnequip` flagged), so what's left isn't worn. Check:
`removeme_takes_the_worn_one` (`crates/world/tests/scripting.rs`).

Check: `scripted_items_see_their_onadd` (`crates/world/tests/scripting.rs`):
a bundle giving 25 cases added twice to the player (50 cases, no bundle);
in a chest nothing (`OnAdd Player`); taken from the chest, it runs; a
bundle whose `OnAdd` only marks it and whose `GameMode` gives 5 cases and
removes it (as `PrimerShotshellAddScript`); a hat whose `OnEquip` /
`OnUnequip` set a global (as the faction outfits' warnings).

## Dropping (`Drop`, `DropMe`, `OnDrop`)

- `RemoveItem` (`005750a0`, the actor's slot 0x17C) flags `OnDrop` (event
  4) on the item's script for the holder whatever becomes of it
  (`005ac750(…, 4)`): put in a container, the script goes with it and runs
  `OnDrop Player` there; destroyed (a plain `RemoveItem`), it never runs (as
  the snow globe's script notes). The Platinum Chip's `OnDrop Player`
  removes its note; given back, its `OnAdd Player` adds it again.
- With its drop flag (`Drop item count`, `005b58d0`; `DropMe`, `005b5860`,
  one of the item its script runs for) the things become a new reference
  (`004c37d0` → `004c6dd0`) at the holder's position plus (0, 50, 30)
  turned by their rotation (50 in front, 30 up), turned as they are, with
  an extra count for more than one; the script goes with it.
- Here: `more_functions::placed::drop_into_world` (a made reference with a
  `count`, saved), `GameState::scripts_follow` / `drop_item`, `moved`
  adding `OnDrop` for the giver; `DropMe` moves the running item's script
  to the new reference with `OnDrop` waiting; scripts on dropped things run
  for their events; `GameState::pick_up` brings a dropped one's script
  back (with `OnAdd`); `more_functions::placed::dropped_items` lets the viewer pick
  dropped things up. The player's heading is kept
  (`GameState::player_heading`) so things land in front of them.
- Checks: `crates/world/tests/dropping.rs`.

## The Pip-Boy's Drop

- ITEMS' Drop is `IM_DropButton` (id 7), reached only through the pad's X:
  the menu's `xbuttonx` refers to its `clicked`, and the button shows only
  with `_Has360Controller` (it copies `IM_EquipButton`'s `visible`). The
  exe sets that global from `004b71d0` (XInput's pad 0 connected,
  `00709fd0`, and `[Interface] bDisable360Controller` off) as the menus
  load (`0070adb0`) and when the pad comes or goes (`00719630`). Shift +
  Enter is the pad's X on the keyboard (`0070c4a0` turns Enter with the
  interface's flag 4 into 0xb for `0070f6e0`), which clicks the referred
  tile only while it shows. Without a pad the PC has no other way: a
  click on a row equips (`00780140` case 0x1d → `00780d60`), and moving
  the mouse off a row lets the chosen item go (`00781620` → `00781b10`).
- With an item chosen `00781680` makes the pad's buttons clickable
  (`target`): Equip (6) as `_EquippableItem`, Drop (7) and Mod (19),
  Hot Key (9) for anything but ammo; none without one.
- The click (`00780140` case 7) refuses, as a corner message, in turn: a
  quest item (form flag 0x400 through `TESForm` slot 0x94, `00401190`;
  `sDropQuestItemWarning`), something equipped while the player is in
  the middle of an action (`008a7570` not -1; `sDropEquippedItemWarning`),
  the player in the air (the character controller's state 2, `009337b0`;
  `sNoJumpWarning`), a worn item that can't come off (`00418b10`,
  `00418ab0`; `sCantRemoveWornItem`), and no room in front of them
  (`009614b0`; `sNotEnoughRoomWarning`). With the keyring open
  (`_KeyringOpen`) it only plays `UIVATSInsufficientAP`. Then more than
  `iInventoryAskQuantityAt` (5) asks "How many?" (`007aba00`, up to all
  of them) and the answer is dropped (`00780c50`); fewer drop one.
- Here: `world::items::drop_refusal` / `drop_asks`, `ui::pipboy`'s
  `ItemsMenu::pad_button` and the buttons' `target`, `ui::game::set_pad`;
  the viewer (`viewer/src/pipboy.rs`) says the refusal, drops one or asks
  with the quantity menu (`game_menus::pipboy_drop`), and shows the pad's
  buttons when Bevy sees a gamepad (or with `--pad`). Seen live in Doc
  Mitchell's house with `--pad`: one of three Stimpaks dropped, then 9 of
  10 through "How many?". Checks: `crates/world/tests/dropping.rs`,
  `ui::pipboy::items` tests.

## Not done

- The Pip-Boy's Drop: the player's current action (`008a7570`) isn't
  tracked, so something equipped is never refused; nor are the worn item
  that can't come off and the room in front (`sCantRemoveWornItem`,
  `sNotEnoughRoomWarning`); the keyring isn't listed. "How many?" is
  drawn flat over the screen, not on the Pip-Boy; the pad's own buttons
  (A, X, Y) aren't read, only the keyboard's Shift + Enter.
- The free spot the game's physics finds for the player's drop
  (`009614b0`) and the fall; the inventory run stopping after `RemoveMe`
  or `DropMe`.
- `RemoveMe` on a worn item removes the running script with it; the game
  removes the worn instance's own script, which is another one when a
  second, unworn instance's script called it (the instances aren't kept
  apart here).
- Items held by others run only for their events; the game runs them with
  their holder's script run (`00565870`) when it's processed.
- A crippled arm dropping its weapon doesn't send `OnUnequip`.
- Item scripts aren't saved.
