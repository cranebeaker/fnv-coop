# T6a-actions: bot action layer (tools/bots/)

Status: Python API, fake bridge, in-process input driver (C++), engine-glue bridge methods, tests and mutation twins done and green (`python -I tools/bots/run_tests.py` prints ALL OK; wired into tools/test_all.py).
In-game status: two real runs (2026-10-21), see "In-game runs" below. The second run passed `proof_input.py` (PROOF OK):
hook, forward, back, strafes, turn, stop, jump and sneak. `fire` and `activate` did not run (no `--target` and `--door` given).
Every other console line and engine call is still UNVERIFIED in the game. C++ tests: `plugin\bots\run_tests.cmd` (47 runtime twins, FNVCOOP_MUT=1..47).

No OS keyboard or mouse input anywhere. Everything goes through nvbridge (`exec`, `ref.info`, `activate.ref`, `world.query`, `ui.dialogopen`, `ui.closedialog`, `state`, `plugins`).

## Using it
```python
sys.path.insert(0, "<repo>/tools")
from bots import Bot, FakeBridge, World, ActionFailed
bot = Bot(nv, "host")            # nv = NvBridge (real) or FakeBridge (game-free)
r = bot.goto("FalloutNV.esm:104C79", radius=100)   # r.ok, r.reason, r.elapsed_s, r.data
```
Each action returns `ActionResult`. `Bot(strict=True)` raises `ActionFailed` instead. A dead game (ConnectionLost / GameCrashed) is never turned into a failed action; it propagates.
Refs are `"Plugin.esm:HEX"`, bare hex (runtime FormID), or int. `None` means the player. `bot.log` keeps every result.
Failure reasons: `timeout`, `not_found`, `dead`, `postcondition`, `unsupported`, `stuck`, `other_cell`, `locked`, `not_enough`, `not_in_inventory`, `no_dialogue`, `already_in_dialogue`, `rpc_error:<msg>`.

## Actions and their postconditions
| Action | How | Postcondition |
|---|---|---|
| goto(target, radius, timeout_s, mode) | walk: stepped `player.SetPos` at 220 u/s (exercises position sync); teleport: `player.MoveTo` / SetPos | within radius, same cell; walk detects "stuck" and refuses other cells |
| follow(ref, keep, duration_s) | repeated goto | ends within 1.5x keep |
| wait, wait_until | clock | predicate true |
| equip / unequip(item) | `player.EquipItem` / `UnequipItem` | `GetEquipped` reads 1 / 0 |
| aim_at(ref) | `SetAngle Z/X` computed from positions | `GetAngle` within 3 deg |
| fire(ref, shots, expect_hit) | `bot.fire`, console fallback | fired >= shots; with expect_hit the target's health drops |
| reload, vats_target | `bot.reload`, `bot.vats` | bridge confirms |
| activate(ref) | `activate.ref`, console fallback | engine accepted |
| open / close(door) | activate | `GetOpenState` open / closed; locked doors fail with `locked` |
| open(container/terminal) | activate | `bot.menu` kind (unsupported without it) |
| take / put(container, item, count) | `RemoveItemTarget` | destination +count AND source -count (catches duplication) |
| pickup(ref, item?) | activate | world ref gone or disabled; item count +1 if given |
| drop(item, count) | `player.Drop` | count -n; `data.ref` is the dropped world ref (`source` event via OnDrop, or `scan` via before/after dev.refsOf, or None) |
| talk(npc) | activate | `ui.dialogopen` true |
| choose_topic(info) | `bot.say` | bridge reply `said` |
| end_dialogue | `ui.closedialog` | `ui.dialogopen` false |

Queries: pos, cell, alive, health, inventory_count(item, holder), quest_stage, dialogue_open, door_state, locked.

## Lines for a reference: exec {cmd, ref} (VERIFIED in game, 2026-10-21)
The game's console does not accept `<hexFormID>.Command`. On FalloutNV.exe 1.4.0.525 the line `00103E61.GetOpenState` got
`Script command "00103E61.GetOpenState" not found`, and the EditorID of a non-persistent ref got `Invalid reference ...`.
Only `player.` works as a prefix. So every line for another reference names it in the bridge's `ref` parameter, and the
bridge runs the line with that reference as the console's calling reference (`RunScriptLine2(cmd, ref)`, see
docs/CONTRACTS.md):
- primitives.py returns a `Line(cmd, ref)` for these lines (`open_state`, `locked`, `item_count`, `transfer`,
  `activate_console`). `P.on(holder, text)` keeps `player.` for the player and puts any other holder in `ref`.
- `Bot._exec` / `Bot._eval` send a `Line` as `nv.exec(cmd, ref=ref)` / `nv.eval(expr, ref=ref)`. `Bot._exec_on(ref, cmd)`
  does the same for a raw command. The ref is `Plugin.esm:HEX`, so the bridge resolves it with the game's own load order.
- Argument refs (for example the destination of `RemoveItemTarget`, the marker of `player.MoveTo`) stay runtime hex ids
  in the line: the console accepts hex FormID arguments.
- fake.py behaves like the game: a `<hex>.Func` line fails with `Script command "<hex>.Func" not found`, and
  `exec(cmd, ref=...)` runs the line on that reference. A bad ref raises the bridge's errors (-32602, -32013).
- Twins: the fake mutant `exec_ref_ignored` (the host checks the ref, then drops it) and `LegacyBot` in
  tests/test_bots.py (a bot that still builds `<hex>.Func` lines) must both fail.
- `Bot.health`: in game (2026-10-21) `world.query {fn: "GetActorValue"}` failed for every ref (the player, Doc Mitchell,
  a door): "query failed (unknown fn, ref not loaded or evaluator drifted)". Other functions worked, also on actors
  (`GetRestrained` on Doc Mitchell -> 1). The cause is in Track A: `plugin/hooks` detours `Cond_GetActorValue` (0x59C4F0),
  so the live bytes start with `E9` (a jump into nvcoop.dll), and `queryCondition` refuses an evaluator without its
  `55 8B EC` prologue. `GetItemCount` (0x59D8E0) fails the same way. `eval("GetAV Health", ref=...)` gave 65.0.
  So `health()` tries `world.query` first and on an RPC error reads `GetAV Health` with the ref as the calling reference
  (`P.get_health`). The fake's `query_av=False` models the game. Twins: with that fallback, `health_stuck`, `fire_miss`
  and `exec_ref_ignored` must still fail `fire(expect_hit=True)`.

## Unverified grammar (fix in tools/bots/primitives.py only)
All of primitives.py except `GetStage`. In particular: `player.SetPos/SetAngle/GetAngle`, `EquipItem/UnequipItem/GetEquipped`, `RemoveItemTarget item target count silent` argument order, `Drop`, `GetOpenState` values (1 open, 3 closed), `GetLocked`, the FLAG_DISABLED bit 0x800, Health = actor value 16 for `world.query p1`, angle sign conventions in `heading_deg`, and whether an expression inside `printc` uses the calling reference that exec's `ref` gives it (`eval(expr, ref=...)`). Update fake.py to the same grammar if you change a string.
Walking is a stepped SetPos: it does not path around walls, so use it inside open areas and teleport otherwise.

## In-process input driver and bridge methods (plugin/bots, ini `[dev] bots=1`) - BUILT, PARTLY RUN IN GAME
Every engine address below was read statically, and each live call is gated. Only the items in "First in-game run" below have run in the real game.
To turn it on in test instances, provision with `python -I tools/runner/run.py provision --n 2 --ini dev.bots=1`. Provisioning rewrites nvcoop.ini, so a hand edit is lost.
**Input driver.** `plugin/bots/bots_module.cpp` detours `OSInputGlobals::GetControlState(ctrl, state)` (thiscall 0xA24660, `ret 8`, prologue pinned in
`plugin/bots/bots_addrs.h`, stolen 6 bytes, restored by the hot-reload shutdown). It is the single function the game asks "is control X down" (state 0 held,
1 pressed this frame, 2 released this frame; seen at ~170 call sites, a count from the `--table` output of the pin verifier, which is not in the public repo: see "Pin verifier" below). `ForcedInput` (input_core.h) holds or taps
controls and serves edge events exactly once, because toggles (Crouch, Jump) fire on the edge. No SendInput, no desktop events, works minimized.
Look/aim sets the player angles with `SetAngle`; movement is real held Forward/Run so the avatar plays locomotion animation.
Controls 0..9, 12, 14, 16 are proven from call sites; 10, 11 and 13..27 follow the NVSE table.
| Method | Notes |
|---|---|
| bot.status | {enabled, hooked, frame, held[], shotsLeft, alwaysRun, vats{phase,reason}} |
| bot.move {forward,strafe,run?,ms?} / bot.stop | axes in [-1,1], digital with a 0.3 dead zone. `run` is the SPEED (true = run, false = walk), not the Run key: see Always-run below |
| bot.look {yaw,pitch} or {ref} | degrees, same convention as SetAngle |
| bot.fire {shots,gapMs} | queued Attack pulses; Reload is a tap of ReadyItem (UNVERIFIED which control) |
| bot.reload / bot.sneak / bot.jump / bot.activate / bot.input | taps and generic hold/release/tap by name or id |
| bot.menu | {kind,id,vatsMode,dialogState?,dialogReady?} from the menu stack top (0x714F00) and the VATS camera mode. Id 1059 (0x423) is the tutorial box: kind `tutorial`, also while VATS is active. VATS itself is 1056 (0x420) |
| bot.dialog / bot.say {info} | list the DialogMenu ListBox choices; pick one by replicating the plain-topic path of HandleClick (0x7624F0) with gates: state 2, click gate, special INFOs refused |
| bot.menu.close {kind?="container"} | close the open menu the engine's own way: `CloseAllMenusCore` 0x7037E0 (what the CloseAllMenus command calls; pinned, `bots_addrs.h`). Only a menu of `kind` is closed (`container`, `terminal`, `barter`, ...; `any` = anything a player can cancel out of, never a message box, the loading screen, the pause menu, VATS or the tutorial box); another menu on top is refused and left open. Reply `{closed,menu,kind}`; the menu is gone from `bot.menu` a frame or two later. Python: `Bot.close_menu(kind)`, `Bot.close_container()` (idempotent) |
| bot.menu.press {button} / bot.consent {answer} | answer the message box the UI layer shows (new `ui.answer`); consent: Ready=0, Not yet=1. When the UI layer shows no box and the engine's own MessageMenu (1001) is the top menu, `bot.menu.press` presses that box natively (see "Native message box press" below). The reply has `path` `ui` or `native`. `bot.consent` never uses the native path |
| bot.inventory {ref?,items?} | totals via 0x4C8F30 (base + delta, equipped included); without items only ExtraContainerChanges entries, partial:true |
| bot.vats {ref} / test.vats.hit | tick-driven job: tap Vats, close the tutorial box, wait for limb select, select the target with VATSMenu HandleClick(3), queue the attack through the attack-button detour, execute, wait for playback. See "bot.vats: queue and execute" below. Limb choice is refused (unsupported) |
| bot.fasttravel {marker} | GENUINE fast travel, queued the way the map menu does it (see below); gated on busy menus/VATS, a pending travel, and the engine's own may-travel check 0x93D660. Reply `{genuine:true,queued:true}`; the player moves a few frames later |
| test.fasttravel {marker} | fallback only (needs the game started with env `NVCOOP_TEST_HELPERS=1`, twin 64): a `MoveTo` teleport that crosses cells; reply `genuine:false`. `Bot.fast_travel` uses it only when `bot.fasttravel` is missing, never after an engine refusal |
| bot.drops {since?,base?,windowMs?} | what the xNVSE `OnDrop` event reported (native handler, params `[dropper, droppedRef]`): `{seq,drops:[{seq,ref,base,dropper}]}`. Correlated by base form plus a 5 s window. `unsupported` if the handler could not register |
| dev.refsOf {base,cell?} | enabled refs of a base form in the cell; `Bot.drop` uses a before/after difference as the fallback when `bot.drops` is unavailable |
| dev.terminal {ref,entry} / dev.terminalState {ref} | click entry N of the open terminal menu (0x757F70 on the ListBox tile); state = what this bot picked, the engine has no used flag; needs the terminal menu open (Bot.use_terminal activates it first) |
| dev.countRefs {base,cell?} | walks TESObjectCELL::objectList (cell+0xAC), skips disabled/deleted |

Python: `Bot.goto` walks with the driver (turn-rate limited steering, sidestep then jump when stuck, `walk_path`), falls back to stepped SetPos; `look, stop, jump,
sneak_toggle, activate_crosshair, menu_info, dialog_choices, inventory, press_box, dismiss_messages, consent, fast_travel, count_refs, vats_target(expect_hit)`.
In-game proofs (lead runs, one game at a time): `tools/bots/proof_input.py` (movement, strafe axes, turn, jump edge, sneak toggle, fire, activate) and
`tools/bots/proof_glue.py` (menu, dialogue pick, inventory vs console, VATS, consent, fast travel, countRefs). Both also self-test over the fake bridge.
Both exit 0 when every check passed, 1 when a check failed, and 2 when the proof could not run. "Could not run" is one of these: a bad command line (refused before the bridge is opened), no game on the instance, the game lost during the run (a crash or a pipe loss: `ConnectionLost` or `GameCrashed`), or a `proof_glue.py` form that the load order cannot resolve.
`main()` returns the exit code and never raises `SystemExit` (`--help` returns 0). Any other Python exception is a bug in the proof: it ends in a traceback and exit 1.
`proof_glue.py` needs `--npc` and `--info` without `--fake`. Every form argument is `Plugin.esm:HEX` or a bare hex runtime id, with an optional `0x` prefix and leading zeros. An empty form value is a bad command line.
`--other-info` names an INFO that the NPC does not offer now: the dialog check then also fails if `bot.dialog` lists it.
**How `--info` and `--other-info` are compared** (`proof_args.form_key`): as numbers, as (plugin, local id), never as text. So `FalloutNV.esm:801`, `FalloutNV.esm:0x801`, `FalloutNV.esm:0801`, `0x801` and `00000801` name the same INFO, and `FalloutNV.esm:1800` is not `FalloutNV.esm:800`.
A bare id is a runtime id: its top byte is the load index, and the game's plugin list (bridge method `plugins`) gives its plugin. Its low 24 bits are the local id.
`Plugin.esm:HEX` with more than 6 significant digits is a runtime id with its plugin given: it is accepted only when its top byte is the load index of that plugin.
So the low 24 bits are compared only when the plugin is known to match. A top byte outside the load order, a plugin that is not loaded, or a top byte that is not the load index of the given plugin gives exit 2 before the proof drives anything.
`--other-info` and `--info` that name the same INFO also give exit 2: before the bridge is opened when the spellings show it (the same plugin, or both bare), else after the load order is read and before the first check.
**Pin verifier: not in the public repo.** The pin verifier `tools/bots/re/verify_pins.py` is not in this repository, so its commands cannot run from a clone.
From this doc, it checked the pinned prologue bytes of every engine function called (the pins in `plugin/bots/bots_addrs.h`, the fast-travel pins included) against the exe.
Its `--table` output is the source of the ~170 call sites of `GetControlState` given above. The C++ test with `BOTS_EXE` also checks the pins (C++ tests: `plugin\bots\run_tests.cmd`).

### Round 3 additions (static RE, nothing run in game)
- **Inventory totals** (confirmed): `bot.inventory` answers with 0x4C8F30 (base container + delta, equipped included) for explicit `items`, and for every ExtraContainerChanges entry otherwise.
- **Speech checks:** `bot.say` allows INFOs for which 0x7627E0/0x762820 is true (reply `speechCheck:true`; `Bot.choose_topic` returns `data.speech_check`). HandleClick only plays a UI sound for them (0x4D5C60 with 0x1F or 0xC), then runs the same select/click/after/speak tail we replicate, so the engine rolls the check in its own speak path 0x762FF0 and the host result syncs through the normal dialogue layer. Flag-0x80 INFOs (0x7627C0) stay refused: they take another branch (item select 0x83E850, speak flag 1).
- **Drop ref:** the module registers a native `OnDrop` handler through `NVSEEventManagerInterface::SetNativeEventHandler` (interface taken from the NVSE interface at load, handler set at DeferredInit, removed on shutdown and hot reload). Params are `void*[2] = {dropper, droppedRef}`; the base form is read at ref+0x20. If registration fails `bot.drops` is "unsupported" and `Bot.drop` falls back to the cell scan.
- **Genuine fast travel** (found statically in the map menu): clicking a marker stores the ref at MapMenu(*0x11DA368)+0x118; the Yes callback queues deferred worker 0x798A00, which does `obj = heap(0x34) [0x401000]; ctor 0x456540(obj); obj+0x30 = marker; player(*0x11DEA3C)->0x93BE30(obj)`. The main loop (0x86F94F calling 0x93BEA0) consumes player+0x1EC and calls 0x93CDF0 to move. The may-travel check is 0x93D660(player) -> bool and shows the engine's own refusal message (combat, hostiles, cell, script block, overencumbered, health damage, undiscovered, alarm, in air). `fastTravel()` in bot_engine.h replicates the worker with those gates. Pins: PlayerCanTravel, PendingTravelSet, TravelObjCtor, HeapAlloc in bots_addrs.h (checked against the exe by the pin verifier `verify_pins.py`, which is not in the public repo: see "Pin verifier" above). UNVERIFIED in game: that the marker handed over (any TESObjectREFR, e.g. the map marker ref) is what +0x118 holds, and that the worker has no hidden dependency on the MapMenu being open (we skip the menu). If the first run shows no movement, read player+0x1EC after the call.
- **VATS limb choice: unsupported, documented.** Limb-select is mode 2, but the RE has no field for the currently highlighted limb and no mapping from limb to Left/Right presses, so a bot cannot know when it landed on the wanted limb. `bot.vats {limb}` keeps refusing with `unsupported`. **Update 2026-10-21:** the highlighted-limb state is now exposed by `plugin/vats` (`vats.state`, `vats.select_limb`, `vats.select_target`, `vats.fire`; docs/tools/vats.md "How bots drive VATS", RE in docs/re/vats-realtime.md). Limb navigation turned out to be directional (stick / mouse), not a Left/Right cycle, so the state words are written and read back instead of pressing keys. `bot.vats` itself is unchanged (plugin/bots is Track B's).

### Always-run (first in-game run, 2026-10-21, then static RE)
- **What the game did.** New Vegas has always-run on by default. `bot.move run:true` held the Run control (9) and the player WALKED: 152-165 units in 2 s. `run:false` RAN: 608-646 units in 2 s. proof_input forward (300..700) and back (< -150) failed.
- **Engine state** (static RE, 1.4.0.525 exe). `PlayerCharacter+0x651` is the always-run byte. The player-controls update `PlayerCharacter::vf2F8` (0x93E860) flips it on the AlwaysRun (10) press edge (0x940C7D). Player init 0x938A52 fills it from `bAlwaysRunByDefault:Controls` (Setting 0x11E0B4C). The run decision at 0x941A2B is: keyboard mode and byte set: run unless Run is pressed or held. Otherwise: run only while Run is pressed or held. Gamepad mode (0x4B71D0) = byte 0x11D8A84 (an XInput pad answered) and `bDisable360Controller:Interface` (Setting 0x11D8C4C, value at +4) is 0. In gamepad mode the byte is ignored. Then forward/back/strafe set walk (0x100) when run (0x200) is clear.
- **Driver.** `run` is now the speed. `alwaysRunState()` (bot_engine.h) reads the byte and the gamepad gate. `bot.move` holds Run only when run != always-run, and the tick re-checks it while the move lasts (a toggle press during a move is followed). The move reply and bot.status give `alwaysRun` (true/false, null when unknown) and the move reply gives `runControl` (Run held or not). Unknown (the player is not loaded, or a pin does not match): Run is held for run:true, the old key meaning.
- **Pins** (`kRunPins` in bots_addrs.h, code bytes, nothing is called or patched): AlwaysRunEdge 0x940C7D, AlwaysRunFlip 0x940C8D, RunPadGate 0x941A2B, RunAlwaysRead 0x941A3E, PadModeConnected 0x4B71D0, PadModeSetting 0x4B71E0. Init compares them with the running exe. The tests compare them with the reference exe (BOTS_EXE). A game-free test checks that the pinned bytes hold the addresses the reader uses.
- **Sneak read.** The console has `IsSneaking`. `player.GetSneaking` gave "Unknown variable 'GetSneaking'". proof_input reads `player.IsSneaking` and a failed read is now a FAIL.
- **Verified in game** (second run, 2026-10-21): with always-run on, `run:true` held no Run key (`runControl:false`) and the player ran:
  forward +610, back -482, strafes +600 / -590 units in 2 s. Only unit-tested: gamepad mode (no pad connected), the unknown-state
  fallback and a toggle change during a move.
- **Gamepad mode in game** (under Proton on a Steam Deck, 2026-10-25, where Steam Input always answers as a pad): `bot.move`
  forward for 3 s held Forward (`held:["Forward"]`, no menu open) and moved the player 0 units. With `bDisable360Controller:Interface`
  set to 1 in memory, the same move went 1008 units. In gamepad mode the game takes movement from the pad, so the keyboard
  controls that `bot.move` holds do nothing. `instances.py provision --no-pad` writes that key for the test copies.
- **`run` omitted** (review finding on PR #10): `bot.move` without `run` holds no Run key, so the game's toggle picks the speed, as
  before `run` meant the speed. Test `move_without_run_leaves_the_speed_to_the_game`; twin 47 (an omitted run counts as run:false).

### Native message box press (static RE on the 1.4.0.525 exe, the press ran in game on 2026-10-21)
Problem seen in game: with all DLC installed, the engine shows its own message boxes (MessageMenu, menu id 1001) about 5 s after a fresh save loads.
`bot.menu` reports `{"kind":"message","id":1001}`. The old `bot.menu.press` only forwarded to `ui.answer`, which knows only the boxes the UI layer showed, so it answered "no message box is being shown".
A bot Activate tap does not close the box either, because menus do not read input through GetControlState. A minimized test copy stays in menu mode and every movement proof fails.

How the native press works (`pressNativeMessageButton` in `plugin/bots/bot_engine.h`, live call in `bots_module.cpp`):
1. The UI layer answers first (`ui.answer`). The native path runs only when that fails.
2. Gates, in this order. The call must run on the game main thread. The interface manager must be ready and the top menu (0x714F00) must be 1001. If the top menu is not 1001, the reply is the UI layer's error, unchanged.
3. The menu object comes from xNVSE `GetMenuByType(1001)`: `g_TileMenuArray` data pointer at 0x11F350C, entry `[1001 - 1001]`, then `TileMenu::menu` at +0x3C. The engine also keeps this object in its MessageMenu singleton 0x11DA4F0.
4. The object's vtable must be 0x107566C (MessageMenu, from RTTI). Any other vtable is refused.
5. A current message must exist: `*0x11DA4F4 < count` of the queue array at 0x11DA53C (data +4, count +8). This is what 0x7A92A0 reads. No current message means the box is closing, so the press is refused.
6. The buttons are a `ListBox<int>` at menu+0x40 (vtable 0x107286C). Its list head node is at menu+0x44: `{item*, next*}`, item = `{Tile* @0, int value @4}`. Button N is the N-th item in list order (0-based, the order the box shows). A button outside the list or an item with a null tile is refused.
7. The press calls vtable slot 3 (+0x0C, HandleClick 0x7AA070, thiscall, `ret 8`) with `(7, tile)`. The engine does the same for a mouse click and for its own Enter and hotkey path (0x7AA110: list index, then tile, then `HandleClick(7, tile)`).
8. HandleClick with id 7 reads the item value of the tile (0x7E1110 on the ListBox), writes it to InterfaceManager+0xE4 (`msgBoxButton`), runs the message callback, and calls 0x7AA530. 0x7AA530 closes the box or shows the next queued message.

Reply: `{menu:1001, button, value, buttons, path:"native"}`. `value` is what the engine records (what `GetButtonPressed` gets). The builder 0x7A92E0 sets value = `(signed char)entry[0x1C]` + the index of the button text. ShowMessageBox passes 0 for entry[0x1C], so value equals the button index unless a button text is empty (an empty text keeps its index but gets no list item).

Pins (`plugin/bots/bots_addrs.h`, checked at init and by the C++ test with `BOTS_EXE`):
- `MsgMenuHandleClick` 0x7AA070, 16 prologue bytes `55 8b ec 51 53 89 4d fc 83 7d 0c 00 75 19 83 7d`, `ret 8`.
- `MessageMenuVtbl` 0x107566C, the vtable gate.
- Data pin `MessageMenu.vtbl[3]`: the dword at 0x1075678 must be 0x7AA070. A mismatch turns off the engine glue, as the call pins do.
- The live call reads the vtable again and calls slot 3 only when the slot holds the pinned 0x7AA070.

Differences from the first brief (verified in the decompile and with dumpbin):
- The six tiles at menu+0x28..0x3C are not the buttons. MessageMenu::SetField (0x7A8AB0) stores the XML `<id>` 0..5 parts there. HandleClick reads that array only for ids 0..5, and then it does nothing, because it acts only on id 7. A press with one of those tiles would record button value 0 (0x7E1110 returns 0 for a tile that is not in the list). So the press takes the tile from the button ListBox.
- The button count is not limited to 6. The list holds one item per non-empty button text. `bot.menu.press` accepts 0..9, and the native path refuses a button the list does not have.

Limits: only MessageMenu 1001, and only when it is the top menu. Button order is the box's list order. The UI layer's box always wins.
In-game status: the press path ran in the game on 2026-10-21 (see "First in-game run" below). The refusal paths (wrong vtable, bad button, closing box, not on the main thread) did not occur in that run. Only the unit tests check them.
Python: `Bot.dismiss_messages(quiet_s=20, max_s=120, max_n=8, button=0, settle_s=0.3, poll_s=0.5, max_closing=10)` presses `button` while `bot.menu` reports kind `message`, and polls every `poll_s` otherwise. It is done when no box has been shown for `quiet_s` seconds. The boxes come in waves: in the first in-game run (all DLC, a fresh save) 6 boxes came at once, then 2, then 2 more about 16 s later. A press-only-now loop returned after the first wave and the later waves stopped movement in the middle of the proof. It fails with `timeout` when `max_s` passes first and with `postcondition` when a box is still shown after `max_n` presses in a row (a box that does not close). A closing box (menu 1001 is still the top menu, but its current message is gone) refuses the native press with "the message box has no current message (it is closing)". dismiss_messages matches only that text (`MSGBOX_CLOSING` in bot.py), waits `settle_s` and tries again. A retry is not a press. It fails with `postcondition` after more than `max_closing` retries in a row, and the count starts again after each press. Every other refusal still ends it (`rpc_error:...`). data: `{pressed, paths, waited_s, quiet_s, closing}`. `quiet_s=0` gives the old press-only-now behaviour. No proof calls it on its own. Pass `--dismiss-messages` to `proof_input.py` or `proof_glue.py` to clear the boxes before the checks (`--dismiss-quiet-s`, `--dismiss-max-s`). It adds a `dismiss_messages` row with `pressed` and `waited_s`.

### bot.vats: queue and execute (static RE on the 1.4.0.525 exe, nothing run in game)
Why the first job failed in the game (2026-10-21): the job entered VATS with the Vats control. Then it tapped Right to cycle and
Activate to accept. VATS never reads those controls. It reads raw keys and the left mouse button only. Also, `bot.menu` said
kind `vats` with id 1059. Id 1059 is the tutorial box (TutorialMenu). VATS is 1056. VATSMenu::Update does nothing while the
tutorial box is open.

The job now runs these steps, one tick per frame on the main thread. Each step has a gate. A failed gate stops the job with a reason.
1. **enter**: the job taps Vats (16). It waits for camera mode != 0. After 3 s it fails with `vats_not_entered`.
2. **tutorial**: while GetMenuByType(0x423) (0xA09030) is open, the job calls 0x7E8D60 (the tutorial close, cdecl, no arguments).
   It tries again every 500 ms. After 3 s it fails with `tutorial_box_stays_open`.
3. **zoom**: the job waits for camera mode 2 (the zoom 1 -> 3 -> 2 ends) and top menu 1056. After 5 s it fails with `vats_not_ready`.
4. **target**: the job reads the selected target 0x11F21CC. While it is not the wanted reference, the job calls VATSMenu
   HandleClick(3, nullptr) (next target, vtable slot +0x0C = 0x7EECB0). It clicks at most once per 300 ms and at most 12 times.
   Then it fails with `target_not_selectable`. The job refuses an object at 0x11DB0D4 whose vtable is not 0x10700D4 (`vats_menu_wrong_type`).
5. **queue**: the job checks that a body part is selected (VATSMenu+0x100), that its hit chance (+0x28) is above 0, and that AP left
   (VATSMenu+0xE0) is above 0. The engine checks the exact AP cost itself. A failed check gives `attack_not_queued` with the hit
   chance and AP in the reason. Then the job arms a one-shot accept. A detour on the attack-button test 0x7EEB50 reads it. While it
   is armed, the call with slot 1 and held 0 (the left mouse press edge) answers true. All other calls go to the original.
   VATSMenu::Update then queues the attack with all its own checks. The accept stays armed over frames, because the mouse pick
   0x7F6AF0 can end a frame first. It disarms when the queue count (0x5AE380, ECX = 0x11F2250) rises. After 3 s the job disarms it
   and fails with `attack_not_queued (hit chance, AP, queue, camera mode)`. Without the detour the job fails with `accept_hook_missing`.
6. **execute**: the job calls 0x705780 (VATSCameraData::Accept) only when the queue count is 1 or more. An empty queue makes the
   engine leave VATS, so the job fails with `queue_empty_at_execute` instead.
7. **playback**: the job waits for camera mode 4. It is done when the mode is 0 again.

The global limit is 25 s (`timeout (phase ...)`). `bot.status.vats.phase` is one of idle, enter, tutorial, zoom, target, queue,
execute, playback, done, failed.
Pins (bots_addrs.h, checked against the reference 1.4.0.525 exe by `pins_match_real_exe_if_available` with `BOTS_EXE`):
hook VatsAttackButton 0x7EEB50 (6 stolen bytes, plain `ret`), calls GetMenuByType, TutorialClose, VatsMenuHandleClick (ret 8),
VatsQueueCount, VatsExecute, data pins VATSMenu.vtbl[3] = 0x7EECB0 and VATSMenu.vtbl[13] = 0x720100 (GetID, returns 0x420).
Not verified in the game: all of it. Static open points: that 0x11DB088 is the tutorial menu pointer that 0x7E8D60 tests; that the
mouse pick 0x7F6AF0 does not end every frame while the cursor is over another body part; and that HandleClick does nothing in
gamepad mode (PadMode) unless VATSMenu+0x10D is set, so the target cycle can fail with a pad connected.

### In-game runs (2026-10-21)
First run.
Setup: FalloutNV.exe 1.4.0.525 (Steam), all DLC, one minimized copy with the focus guard.
What the run verified:
- After a fresh save loaded, 10 native MessageMenu (1001) boxes appeared in waves of 6, 2 and 2. `bot.menu.press {button:0}` answered `path` `native` for each box.
- After the presses, `bot.menu` reported kind `none` and the game time advanced.
- Then `proof_input.py` passed `hook`, `strafe_right`, `strafe_left`, `turn`, `stop` and `jump`.

What the first run did not verify: `forward` and `back` walked (the always-run inversion), and `sneak` failed (`player.GetSneaking`
gave "Unknown variable").

Second run, with this branch: `proof_input.py --dismiss-messages` printed PROOF OK (exit 0): forward +610, back -482, strafe_right
+600, strafe_left -590, turn, stop, jump, sneak 0 -> 1 -> 0. The sneak check needed the landing wait (`SETTLE_AFTER_JUMP_S`): within
about 2 s of a jump the engine applies only one crouch toggle. The forward check also needs open ground: right outside Doc
Mitchell's door an obstacle cut a forward burst to 180 units.

Still not verified in the game:
- The refusal paths of the native press (wrong vtable, bad button, closing box, not on the main thread) are only unit-tested (`plugin/bots/tests/test_bots.cpp` and its twins).
- `fire` and `activate` (no `--target` / `--door` given), and every check of `proof_glue.py`. These need ref-targeted console lines,
  which the bridge `exec` cannot run yet (a `<hex>.Command` line fails in the game).

Known limits: terminal state only knows the bot's own picks; VATS limb choice; Reload control id; full inventory enumeration of base-container-only items.

## Tests
`tools/bots/tests/test_bots.py`: 24 tests over the fake bridge (virtual clock, milliseconds). Mutation twins in fake.py (`setpos_ignored, angle_ignored, activate_noop, transfer_noop, transfer_dup, equip_noop, drop_noop, fire_miss, health_stuck, dialog_never, ref_info_error, crash, msgbox_stuck, msgbox_ui_only, exec_ref_ignored`): each must make the matching action fail. `msgbox_stuck` (the box never closes) and `msgbox_ui_only` (the plugin before the native path) must make `dismiss_messages` fail. The fake's `box_waves` adds engine boxes later in virtual time, and while an engine box is shown the fake player does not move (menu mode). The waves tests press all 10 boxes of the in-game pattern. Their twin (`_press_only_now`, the old loop) leaves 4 boxes, and in proof_input it makes `forward` fail. The fake's `box_closing_s` keeps a pressed box in the closing state for that many seconds. Its twin (`_any_refusal_fatal`, the old loop) fails on the first closing refusal.
The fake's `always_run` (default True, the New Vegas default) models the always-run toggle. The fake plugin holds Run only when run != always_run, and the fake engine runs when Run held != always_run. Its twin `run_is_key` (Run held for run:true, the plugin before run = speed) makes the proof_input `forward` row FAIL with always-run on (`tools/bots/tests/test_proof.py`).
The fake uses the same grammar as primitives.py, so it proves the Bot logic, not the real console.
C++ twins for the native press (`plugin/bots/tests/test_bots.cpp`, runtime twins FNVCOOP_MUT=33..41): 33 native path never used, 34 vtable check skipped, 35 button index off by one, 36 native path tried before the UI layer, 37 null tile not refused, 38 top-menu gate skipped, 39 closing-box gate skipped, 40 native reply says path "ui", 41 main-thread gate skipped.
C++ twins for always-run (`bot.move` run = speed): 42 Run held for run:true whatever the toggle is, 43 a toggle change during a move is not followed, 44 always-run state ignores gamepad mode, 45 always-run state reads player+0x652 (the auto-move byte), 46 an unknown always-run state is treated as on.
C++ twins for `bot.vats` (the fake engine models the tutorial box, the zoom, the target list, the mouse pick that ends a frame, the AP cost and an undo): 49 the accept is never armed, 50 execute is called with an empty queue, 51 the tutorial box is never closed, 52 the target cycle is skipped, 53 the tutorial box is reported as `vats`, 54 the accept gate forces every slot and the held query, 55 the VATSMenu vtable check is skipped, 56 the hit-chance gate is skipped.
`tools/bots/tests/test_proof.py` and `test_proof_cli.py`: the two proof scripts over the fake, and their command lines. A bad command line (an empty form value included) must make `main()` return 2 and name the argument before the bridge is opened, without `SystemExit`.
A missing game, and a game lost in the middle of the run (`ConnectionLost` and `GameCrashed` from the fake), must give 2 without a traceback. `--other-info` in every accepted spelling must make the dialog check catch the fake mutant `dialog_lists_all`.
A game lost during the `player.IsSneaking` read must also give 2: the sneak FAIL is for a failed read only, so proof_input lets `ConnectionLost` through to the run guard.
`--info FalloutNV.esm:1800 --other-info FalloutNV.esm:800` must pass on a fake whose NPC offers 1800. A `form_key` table and the load-order refusals (exit 2 before the first check) check the runtime id rule.
Twins: in-process copies of the scripts with each check removed or with the old code put back (text and suffix matching, the number-only same-INFO check, empty values let through, `ap.error` exits, an unguarded run, low 24 bits compared without the plugin, a sneak check that also catches a lost game).
proof_input activate: the proof walks back to `--door-dist` units (default 100) from the door first, and a cell change (a load door) also passes. The fake's `activate_reach` (the game's reach is about 150) and `Thing.load_to` (a load door) model the game. Twins: `--door-dist 0` (the door must already be in reach) fails after the movement checks, and the state-only rule fails a load door. An RPC error in the fire or activate check gives a FAIL row. Its twin (no guard) stops the run with the error.

## For T6b
FakeBridge.pair() gives two bridges on one World (host and client views are not synchronised by the fake; a playbook that needs sync behaviour must simulate it on top). Agreed API was sent by message; any requested change goes to T6a-actions.
