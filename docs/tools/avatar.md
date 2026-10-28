# Avatar: player puppets (plugin/avatar), Milestone 2

Owner: avatar agent. **Everything here is game-free and unit tested against a fake game. Nothing has run in the game.**
The puppet layer follows FULL-SYNC-PLAN section 3.1: a restrained actor made from a template NPC, driven every frame from
the interpolated snapshot of the remote player (`synccore/live`), with an equipment correction loop, never saved, purged on load,
and a slot -> puppet ref map (the "ghost-ref map" of the acting-player design, `plugin/coop` `bindPlayerRefs`).

## Run

- `plugin\avatar\run_tests.cmd` (one command, ends in `avatar: ALL OK`). It builds standalone in `%TEMP%\nvav_build` (Ninja, 2 jobs,
  takes `out\locks\build.lock` only while compiling) and then runs:
  1. `python -I tools/avatar/check_groups.py` (+ its `--mutant`): `avatar_groups.h` equals the generated xNVSE group table.
  2. `avatar_tests`: the suite, then the whole suite once per runtime mutation twin (38 twins, each must fail at least one check).
     `--no-twins`, `--twin NAME`, `--list-twins`, `--verbose` for debugging.
  3. `avatar_live_tests`: the driver on top of the REAL `synccore/live` (sender delta codec, lossy jittery link, receiver, InterpBuffer),
     judged by game-free versions of the T7 detectors (slide, skate, T-pose proxy, float, jitter, rubber band, silence). 5 twins incl. two live-layer faults and the engine push.
  4. `python -I tools/avatar/proof_puppet.py --selftest`: the in-game proof's analysis and its assertions on synthetic T7 streams,
     with 5 injected-defect twins (never touches a game).
- Added to the parent build with one line (`plugin/CMakeLists.txt` sub-directory list). The targets are `EXCLUDE_FROM_ALL`:
  the canonical `nvcoop.dll` build does not compile or link the puppet layer yet (see "Not wired").

## Files

| File | What |
|---|---|
| `avatar_types.h` | value types (`PuppetPose`, `Anim`, `Appearance`, `Config`, `Stats`), all numeric thresholds in `Config` (first guesses) |
| `loco.h` | locomotion codes carried in `live::AnimState::locomotion`, direction maths, sender-side `CaptureLoco` |
| `avatar_groups.h` | the animation group ids the driver asks for + `GroupName` (script name); checked against the xNVSE table |
| `anim_planner.*` | owner `AnimState` + velocity -> group (speed matching, jump, reload, draw/holster, aim pitch) |
| `equip_corrector.*` | per-puppet equipment correction loop (own item first, extras off, weapon last, backoff, flap guard) |
| `puppet_manager.*` | the driver: lifecycle, per-frame pose/anim/restraint/equipment, park, cross-cell, save/load, ref map |
| `puppet_engine.h` | `IPuppetEngine`: every engine call, nothing else touches the game |
| `script_engine.*` | `ScriptPuppetEngine`: `IPuppetEngine` on script-command text (`IScriptRunner` + `IEngineReads` supplied by plugin glue) |
| `live_bridge.h` | `live::InterpOut` -> `PuppetPose`, and the sender-side `MakeLocalSnapshot` |
| `testing/fake_engine.h` | the fake game (3D loads late, rebuild drops restraint, menus reset animation, AI strips/re-equips, id reuse, orphans in saves) |
| `avatar_faults.h` | runtime mutation switches |
| `tools/avatar/proof_puppet.py` | the in-game proof (WRITTEN, NOT RUN), `tools/avatar/check_groups.py` |

## Puppet traps -> test -> twin

Sources: VaultMP list (`docs/research/vaultmp-actor-sync.md` sections 2.2 to 4), FULL-SYNC-PLAN 3.1 ("restrained actors refusing
commands, animation re-assert after menus, speed matching, cell borders vs transitions"), live layer rules (snap above 10 m, smooth above 2 m).

| # | Trap | Handler | Test (`tests/test_avatar.cpp`) | Twin |
|---|---|---|---|---|
| 1 | commands to a ref without 3D are refused / lost | `Loading3D` phase; nothing but a 3D check until `Has3D` | `spawn_waits_for_3d` | `noWait3D` |
| 2 | AI takes a loose actor over | `SetRestrained` first, then re-checked every 250 ms and re-asserted | `restraint_reasserted`, order check in #1 | `noRestrainReassert` |
| 3 | menus reset animation | replay on the first Tick after a menu closed, nothing sent while one is open | `anim_after_menu` | `noMenuReassert` |
| 4 | a menu leaves the puppet near, not at, its place | forced pose on menu close | `post_menu_snap` | `noPostMenuSnap` |
| 5 | engine forgets the group without any event | `CurrentGroup` mismatch (300 ms) or heartbeat when unreadable; one-shot groups excluded | `anim_heartbeat` | `noAnimHeartbeat` |
| 6 | slide: owner code says idle but speed says walk (also walk/run mismatch) | speed matching with 150 ms debounce and run/walk hysteresis; a moving code with zero speed (wall) is kept | `speed_matching_planner`, `no_slide`, `planner_groups` | `trustLocoCode` |
| 7 | remote stops sending: run in place | `frozen` (dry buffer) or no pose for 3 s means idle at once | `frozen_and_stale` | `frozenKeepsRunning` |
| 8 | jump needs start / loop / land | `JumpStart`, `JumpLoop`, `JumpLand` (+ forward variants) | `jump_sequence`, planner jump test | `noJumpLand` |
| 9 | drawing a weapon the puppet does not hold | planner sees `weaponDrawn` only once the weapon is equipped | `weapon_draw_after_equip` | `weaponDrawBeforeEquip` |
| 10 | equip needs ownership; default outfit is worn; slot replacement | own item first, extras off, weapon last | `equipment_converges` | `noEquipAddFirst` |
| 11 | engine refuses equipment calls forever | exponential backoff, give up after 6, cleared when the wanted set changes | `equipment_backoff_giveup` | `noEquipBackoff` |
| 12 | AI puts the default outfit back on (STR-style fight) | after 3 reverts, `RemoveItem` instead of `UnequipItem` | `equipment_flap_guard` | `noEquipFlapGuard` |
| 13 | puppet of the local player | refused | `local_slot_and_limits` | `allowLocalSlot` |
| 14 | reconnect / double join duplicates | one puppet per slot, the old ref is deleted | `duplicate_slot`, `remove_player` | `duplicateSlotSpawns` |
| 15 | ref gone (cell unload) or id reused by something else | `IsValid` before every drive; respawn; never delete a foreign ref | `stale_and_reused_refs` | `noStaleRefCheck` |
| 16 | remote's cell is not loaded here: puppet floats in the void | delete after 500 ms grace, respawn when the cell loads | `park_when_cell_unloaded` | `neverPark` |
| 17 | cell borders vs transitions | exterior neighbours: `SetPos`; anything else: `MoveToCell` then wait for the rebuilt 3D and re-apply restraint, pose, animation | `cross_cell_move`, `exterior_border` | `crossCellSetPos` |
| 18 | 3D rebuilt in place (cell reload) drops animation | `Generation3D` change forces pose + animation + restraint check | `rebuild_3d_reasserts` | `no3DGenReassert` |
| 19 | never saved | despawn on `OnSaveBegin`, no respawn until `OnSaveEnd` | `save_despawns` | `saveKeepsPuppets`, `noSaveWait` |
| 20 | purged on load, orphans in an old save | `OnLoadBegin` drops refs without touching the world; `FindTaggedPuppets` purge after load, on the first Tick of a session, and when `WorldEpoch` changes without an event | `load_purges_orphans`, `first_tick_purge` | `noLoadPurge` |
| 21 | heading wrap and pitch | yaw wrapped to (-pi, pi], no SetAngle storm at the seam, pitch/roll never sent | `yaw_handling` | `noYawNormalize`, `usesPitchAsYaw` |
| 22 | too many engine calls per frame | `Config::maxEngineCalls`, round-robin start so nobody starves | `call_budget` | `ignoreBudget` |
| 23 | stale ghost-ref map | `RefOf`/`SlotOf` only for live refs | `ref_map` | `staleMapEntries` |
| 24 | spawn fails / 3D never arrives | backoff 0.5 s doubling to 8 s, `Failed` after 8 attempts, 3D timeout 5 s replaces the ref | `spawn_retry` | (covered by #1, counts) |
| 25 | snap and physics push | snapped sample forces SetPos + animation; drift > 2 m is corrected and counted | `snap_and_drift` | |
| 26 | script text must be exact | `ScriptPuppetEngine` pins every line | `script_backend`, `manager_on_script_backend` | |
| 26b | a body in the way (the real player standing in the puppet) pushes the puppet hundreds of units, and the buffer eases it back from the pushed place | `PuppetManager::ActualPose` reports the last applied place when the engine place is farther than `driftUnits` (the same Tick's drift check then undoes the push with one `SetPos`); `avatar.status` counts `pushesIgnored` | `physics_push_not_adopted`; live: 4 pushes in the integration scenario (`shoveFrames`) | `adoptPhysicsPush` |
| 26c | the remote player stands in (or runs through) the place of the REAL player of this machine: the driver's `SetPos` puts the puppet into the same spot every frame and the engine shoves the real player, hundreds of units in a second | `PuppetManager::MakeRoom` sends the engine a place at `Config::bodyRoomUnits` (70) from the local player when the true place is closer (same cell, within `bodyRoomHeightUnits`); the true place is used again once the bodies are apart; `IPuppetEngine::GetLocalPose` (engine: the player ref) says where the local player is; `avatar.status` counts `bodyYields` | `body_room` (fake engine shoves the real player by the overlap of every write), engine `get_local_pose_reads_the_player`, `stack_makes_room_for_the_local_player` | `noBodyRoom`, engine `86` |

Bench note (2026-10-23, `tools/bench/coop_bench.py` step walk): the client's puppet of the host jumped about 6 times a minute while the host walked, the host's puppet of the
client never did. The step walk issues `player.SetPos` console lines; on the host those are captured and replayed on the guest (a guest's own `player.` lines are muted,
`suppress_policy.cpp`), so the guest's real player is carried along and stands inside its puppet of the host. The engine pushes the two bodies apart and the old
`ActualPose` handed the pushed place to `InterpBuffer`, which eased the puppet back over 0.3 to 0.6 s (a 2 to 10 m error is eased, not snapped). The fix above removes the
easing; the carrying of the guest's player by the host's console lines is outside this module and is not changed here. UNVERIFIED in the game.

Bench note (2026-10-25, `tools/bench/coop_bench.py` with real movement input in the arena): the recorded run showed the GUEST's real player displaced, not only its puppet.
While the host walked (the guest idle) the guest's player moved 113, 122, 67 and 106 units in single samples, in the walking direction of the host's puppet, each time the puppet
passed through the guest's standing place (the two players start 90 units apart). Later, during the guest's own legs, the guest's position changed by 1000 to 2400 units per
sample for 1 to 1.5 s, five times, ending 3.5 to 26 km from the course (the legs that "did not arrive"). The puppet side of the host followed those jumps as designed (a source
that moves more than 10 m between snapshots is held and then snapped; at 5 Hz far-tier rate that is the staircase the bench counted as teleports and freezes), so the p95 error
of 574 / 4,307 units measures the displaced PLAYER, not a puppet defect. The driver's part that is certain from the code: a puppet standing in the real player was re-sent by
`SetPos` into the same spot on every drift check, which the engine answers by pushing the bodies apart. 26c removes that. Whether it also removes the large displacements is
UNVERIFIED (nothing in the data shows a puppet near the player during those); after the next arena run read `avatar.status` `local.teleports` (the sampler counts position jumps of
the local player above 350 units) next to `bodyYields` and `pushesIgnored` to tell the two apart.
Live integration (`avatar_live_tests`): a scripted 24 s owner (stand, run 300 u/s, sneak-walk, strafe, draw/aim, jump, 28 m teleport, 2 s silence)
through the real codec, 0% and 10% loss, 3 seeds each. Asserted: no slide, no skate, no T-pose proxy, no float, no jitter, exactly one big jump (the teleport),
max step < 12 units, legs stop within 800 ms of silence, exactly one spawn, no refused or foreign commands.
Twins: `frozenKeepsRunning`, `noWait3D`, live `noSnap`, live `extrapolateOnly`.

Observation for live-core (found by the integration test, not fixed here): with 25 ms of latency jitter on the link, `InterpBuffer` output advances
in lumps: single frames of 14 to 16 units where the nominal step at 300 u/s is 4.8, about every 100 ms (seen at t=2.3 s in all seeds, with and without loss).
The cause looks like the clock-offset min filter re-basing the shown time. It is below the 150-unit T7 rubber-band threshold, so the integration test only
bounds it at 20 units; it would show as slight stutter when running. `AVATAR_DEBUG=1 avatar_live_tests.exe` prints the frames around it.

Mutation twins here are runtime switches (`avatar_faults.h`, `--twin NAME`), the same idea as `synccore/live` `LiveFaults`, not yet the repo-wide
`FNV_MUT(n)` / `FNVCOOP_MUT` mechanism of `tools/mut`; converting is mechanical if the build-speed work wants one convention.

## What the driver needs from the live layer and the plugin

- Per remote player: one `live::InterpBuffer`; each frame `pm.SetPose(slot, FromInterp(buf.sample(now, actual)))` then `pm.Tick(now)`.
  `pm.ActualPose(slot, cell, pos)` feeds `sample()`'s `ActualPose`.
- Local player, 20 Hz: `MakeLocalSnapshot(netId, epoch, seq, tick, cell, pos, rot, vel, anim, &LocalRead)`. `rot = (pitch, yaw, roll)`, yaw = REFR rotZ.
  `anim` flags (weapon drawn, sneak, jump, reload, aim) come from the T7 reads (`weapon.out`, `weapon.aiming`, `move_flags` bit 9, ...), `LocalRead` from the mover flags.
- Equipment changes: a reliable event with the full wanted set -> `pm.SetEquipment(slot, set)`. Not part of the snapshot.
- Game events: `kMessage_SaveGame` -> `OnSaveBegin`, then `OnSaveEnd` when the save finished; `kMessage_PreLoadGame` -> `OnLoadBegin`; `kMessage_PostLoadGame` -> `OnLoadEnd`
  (names from `third_party/NVSE/nvse/nvse/PluginAPI.h`). `WorldEpoch()` is the backstop if one is missed.

## Bridge contract for the in-game proof (implemented by the real engine, see "Real engine" below)

`tools/avatar/proof_puppet.py` needs, on the machine that shows the puppet, `avatar.status` (puppets, stats, `tagged`, `engine`, `backend`, `local`),
`anim.probe` with `refs` accepting the puppet ref (exists), and on the machine whose real player is copied the bot input driver (`[dev] bots=1`).
Exit code 3 means "not wired / not ready" (nothing judged), 1 = an assertion failed, 0 = all asserted.

## Not wired before the engine existed (done: plugin/avatar/engine, next section)

The original list was: an `IScriptRunner` / `IEngineReads`, the module with `avatar.status`, the NVSE message hooks, `bindPlayerRefs`, a template NPC
base. The engine round replaced the first two by a `GamePuppetEngine` over an `IGameBackend` (below), added the module and the bridge methods, and
made the template configurable (`template=player` needs no ESP). `bindPlayerRefs` and the network side live in plugin/coop/live, which calls `avatar_glue.h`.

## Real engine (plugin/avatar/engine), round of 2026-10-21

**Everything below is unit tested on a fake game (`FakeGameBackend`) and compiles into nvcoop.dll. NOTHING of it has run in the game.**
The fake models what we expect the engine to do and is strict about console grammar; the first in-game run is `proof_puppet.py --loopback`.

Run: `plugin\avatar\engine\run_tests.cmd` (one command, ends in `avatar engine: ALL OK`). It builds `avatar_engine_tests` in the shared tree
(`tools/build/build.py`), runs the suite once for real and once per runtime mutation twin (`FNV_MUT`, 71 twins: 1-34, 40-56, 60-72, 80-86, each must fail
at least one check), builds nvcoop.dll (the game side must compile), then `tools/avatar/verify_engine_pins.py` (+ `--selftest`) and
`tools/avatar/proof_puppet.py --selftest`. Registered in `tools/test_all.py` as "M2 avatar engine".

| File | What |
|---|---|
| `game_backend.h` | `IGameBackend`: the raw primitives (ref / cell / animation / equipment reads, `RunLine` on a calling ref, condition evaluators). The only door to the game |
| `puppet_game_engine.*` | `GamePuppetEngine : IPuppetEngine`. All policy: guards, spawn, per-axis writes, 3D tracking, `CurrentGroup`, cost accounting |
| `local_sampler.*` | `LocalSampler` (local player -> `live::ActorSnapshot` at 20 Hz), `EquipWatcher` (worn items -> reliable event), `ReadLocalFrame` |
| `avatar_runtime.*` | `AvatarRuntime`: sampler + watcher + `InterpBuffer` per remote + `PuppetManager` + dev loopback + `StatusJson` |
| `avatar_glue.h` | what plugin/coop calls (sinks, identity, `AddRemote`, `OnRemoteSnapshot`, ghost-ref map, `PreSave` / `PostSave`) |
| `engine_pins.h` | every engine address / offset used, prologue bytes pinned, with its source |
| `win/game_backend_win.*` | `WinBackend`: SEH-guarded reads, pinned calls, the console. x86 + xNVSE headers, nvcoop.dll only |
| `win/avatar_module.cpp` | module `m2_avatar`: ini, lazy runtime creation, bridge methods, NVSE messages, the glue functions |
| `testing/fake_backend.h`, `tests/test_engine.cpp` | the fake game and 59 tests (540 checks) |
| `tools/avatar/verify_engine_pins.py` | proves `engine_pins.h` against a reference exe (24 checks, 6 twins) |

### What the real engine does (each line has a test and, mostly, a twin)

- **Guard in front of every write**: a reference is touched only when it is an actor, not the player, not deleted / disabled, its base is one of the
  template bases AND its id is a created reference (`0xFFxxxxxx`). A placed NPC that happens to share the template base (template = a vanilla NPC) is
  never ours, so the load-time purge cannot delete the real one. A recorded base per spawned ref catches a reused id that now belongs to something else.
- **Spawn finds its own ref** (a console `PlaceAtMe` returns nothing): list the anchor cell's actors before / after. Anchor = the local player when the
  target cell is the player's cell, else a placed non-actor ref in that cell (one with a loaded 3D preferred). Reused ids are handled (a deleted ref
  that is still listed does not hide the new object). The new ref is positioned in the same frame (SetPos / SetAngle, best effort). It must pass the
  guard or it is deleted again at once. **Circuit breaker**: at most 8 spawns in 10 s, because a driver that keeps losing its refs would otherwise flood
  the world with actors.
- **SetPos / SetAngle compare with where the ref really is** and send one console line per axis that differs by more than 0.25 units: a standing puppet
  costs no line, a runner about one per frame (Y only), a diagonal runner two. Nothing is sent while the ref has no 3D.
- **`CurrentGroup`** counts a group as playing when ANY AnimData slot holds it (the slot layout is not proven); a reset (menu, rebuilt 3D) empties every
  slot and the driver re-issues the group. Unreadable = -1 (the driver's heartbeat).
- **3D generation** from the scene object's identity, plus "lost then regained". A rebuilt 3D forgets what we believed about the restraint, so the
  fallback of `IsRestrained` (used when the `GetRestrained` evaluator is unavailable) can never claim a restraint that the rebuild dropped.
- **Puppets stay hittable** (`MakeNonCombatant` = `SetIgnoreFriendlyHits 1`, no `SetGhost` unless `GamePuppetEngine` is built with `ghostPuppets`): the
  combat layer (plugin/combat) maps a puppet to its player slot through `SlotOf` and needs the engine's hit event on it, to forward the damage to the
  remote player and neutralise it locally. A ghost would let enemy shots pass through it. UNVERIFIED which of the two the engine really does.
- **Weapon draw = `SetAlert`** (no script command draws a weapon; VaultMP drives its puppets this way). `IsWeaponDrawn` reports the engine's own
  `IsWeaponOut`, never the request, so `proof_puppet.py` sees it if SetAlert does not do it.
- **Appearance is honest**: race and sex come from the template choice; face / hair / eyes cannot be set from the console (command table), so
  `ApplyAppearance` returns false and counts `appearanceApproximate` instead of pretending.
- **Cost accounting**: every console line is timed with QueryPerformanceCounter; `avatar.status engine` shows p50 / p95 / max microseconds. This answers
  "what does a script line cost" (was a TODO-RE).
- **Local player**: position and rotation from the reference ((pitch, yaw, roll) = (rotX, rotZ, rotY)), finite-difference velocity (smoothed, restarted by a
  teleport above 350 units, a cell change, a gap, clamped to 2200 u/s), locomotion from the speed (mover flags only when the walk / run bit is set),
  jump / reload from the running animation groups (227..244 / 173..199), aim from the process, weapon from `IsWeaponOut`, sneak from `IsSneaking`.
  The schedule advances by one period so 16.7 ms frames still give 20 Hz (a naive "now + period" gives 15 Hz; found by a test).
  Equipment is polled and published only when stable (250 ms), sorted, weapons flagged.
- **Hook layer interplay** (a real trap found by reading plugin/hooks): puppet console lines run inside `Script::Execute`. On the host the capture
  hub would record `Disable` / `AddItem` / `MoveToMarker` on a puppet as story effects and replicate them; on a guest the suppression gate would drop
  them. `WinBackend::RunLine` therefore holds `CaptureHub::pushMute`, `nvhooks::ApplyScope` and `ApplierExecScope` around every line.

### Engine facts used (engine_pins.h; `python -I tools/avatar/verify_engine_pins.py` re-proves them)

| Fact | Source | Status |
|---|---|---|
| `LookupFormByID` 0x4839C0 (cdecl) | engine-addresses.md | verified, prologue pinned, same bytes as plugin/hooks |
| `MenuMode` without argument 0x702360 (cdecl bool) | docs/re/dialogmenu-menus-vats-inventory.md | verified; also used by the `menuOpen` of the driver |
| evaluators `GetRestrained` 0x59FF90, `IsWeaponOut` 0x5A0550, `IsSneaking` 0x5A2F60, `GetDead` 0x59D840, `GetItemCount` 0x59D8E0, `GetEquipped` 0x59DA90 | eval column of command_table.json | address = table, prologue pinned; calling pattern as `nvworld::queryCondition` |
| `TESObjectREFR::IsDisabled` 0x565090 | engine_addrs.h (conf M) | disassembled: ExtraData 0x92 flags & 8 |
| ref fields (flags, base, cell, pos, rot, renderState -> niNode +0x14, NiAVObject world translate +0x8C) | xNVSE headers, animprobe | header offsets; +0x8C verified by animprobe's static check |
| process vtable slots 0x6E / 0xF9 / 0x101 / 0x103 / 0x115, ActorMover slot 8 | tools/animcheck/verify_engine.py | verified statically; a slot is called only when the vtable lies in .rdata and the target in .text |
| AnimData sequences at +0xE0.. , group id at animGroup(+0x68)+8 | animprobe | NOT proven (runtime guarded by the BSAnimGroupSequence vtable) |
| worn items: ExtraContainerChanges (0x15) -> Data -> objList -> EntryData{extendData, count, type}; ExtraWorn 0x16 / WornLeft 0x17 | xNVSE GameExtraData.h | header layout, SEH guarded, unproven in a run |
| loaded cells: `TESWorldSpace::cellMap` (+0x30) of the player's worldspace, cellState == Attached | xNVSE GameForms.h | header layout, unproven in a run |

### Wiring (done in plugin/coop/live against `avatar_glue.h`; the module needs no other change in main.cpp)

`nvcoop.ini [avatar]`: `enabled=1` (default 0), `template=player` (default: the local player's own base form, no ESP; UNVERIFIED that `PlaceAtMe` of it
works) or a hex NPC_ form id, `templates=RACE:m|f:BASE,...`, `localSlot`, `maxPuppets`, `loopback=1` (start the dev loopback), `loopbackOffset=160,0,0`.
Bridge (all on the main thread): `avatar.status`, `avatar.local`, `avatar.loopback {on, offset?}`, `avatar.add {slot, race?, female?}`,
`avatar.remove {slot}`, `avatar.feed {slot, x, y, z, yaw, vx, vy, vz, loco, cell?}` (one synthetic remote snapshot: a remote player without a network),
`avatar.presave` / `avatar.postsave`.
The runtime is created on the first frame in which the player exists in a cell; calls made before are remembered and replayed.

### In-game proof (written, NOT run)

`python -I tools/avatar/proof_puppet.py --loopback` with ONE game instance (`[avatar] enabled=1`, `[dev] animprobe=1 bots=1`, 15 m of free floor):
the loopback puppet copies the real player through the same interpolation buffer and driver a remote player gets, `anim.probe` reports both actors in
one reply, the proof splits it into two machines (removing the offset again) and runs the unchanged T7 analysis (`avatar.*` counters) plus the engine
counters of `avatar.status` (no console exceptions, no refused foreign write, no spawn without a new ref, p95 line cost, 3D drift) and the backend pins.

### Known limits of this round (each is also in the TODO-RE list)

- `kMessage_SaveGame` arrives AFTER the .fos is written, so the module cannot despawn puppets before an engine-started save (autosave, quicksave).
  The guarantee is the purge on load (orphans are found by template base + created id in every attached cell). `glue::PreSave()` / `PostSave()` and the
  bridge `avatar.presave` / `avatar.postsave` exist for saves that tools start; a missing `PostSave` times out after 10 s. A pre-save detour of
  `BGSSaveLoadGame::SaveGame` (0x8503B0) was NOT written: a hook on the save path is a risk to the owner's saves.
- Face, hair and eyes of a remote player are not applied (no console command): the look is the template's. With `template=player` every puppet looks like
  the LOCAL player, which is right for the loopback and wrong for remote players; a real build needs one template NPC per race / sex in an ESP
  (`templates=`), and a FaceGen transfer is future RE.
- `SetAlert` as the weapon-draw is a hypothesis; the Equip / Unequip groups carry the pose either way.

## TODO-RE (unknown engine facts, all unmeasured; the driver treats each as an ordinary refusal)

- Whether `SetPos` on a restrained actor and `PlayGroup` on an AI-less actor are accepted without VaultMP-style patches (`research/vaultmp-actor-sync.md` section 3 lists
  the FO3 sites; the NV equivalents would be found from the command handlers `Cmd_SetPos 0x5C05F0`, `Cmd_PlayGroup 0x5C0DF0`, `Cmd_SetRestrained 0x5D0920` in
  `plugin/hooks/tools/re/command_table.json`, early-out branches not read yet).
- Exact meaning of `PlayGroup`'s flag, `EquipItem`'s two flags, the sign/direction of `SetAngle Z`, and whether `PlayGroup` by name resolves the weapon-type variant.
- Sneak: whether `SetForceSneak` changes which sequence `PlayGroup Forward` plays.
- A way to draw/holster a weapon (no script command exists; `IsWeaponOut` 0x5CB660 reads it). `GamePuppetEngine::SetWeaponDrawn` uses `SetAlert` (hypothesis, proof asserts `avatar.wrong_weapon == 0`); `ScriptPuppetEngine` still returns false. The process `SetWeaponOut` virtual (neighbour of slot 0x115) is unknown.
- Appearance (face, hair): no console command exists; `GamePuppetEngine::ApplyAppearance` reports "approximate" and the template NPC's look is used.
- Cost of a script line per call (`Script::Run` 0x5AC400 compiles every line): the engine now MEASURES it (`avatar.status engine.costP50Us / costP95Us / costMaxUs`, bounded by `proof_puppet.py`). If 3 puppets at 60 fps are too slow, replace the hot calls with direct member calls after RE; `IGameBackend` does not change.
- `MoveToMarker 0x5CCF50` accepting a non-marker anchor ref; whether `kMessage_SaveGame` fires before the engine serialises refs (if after, only the load-time purge protects saves).
- `SetPos` across exterior cells of one worldspace moving the ref's parent cell (`SameSpace` assumes yes).
- Whether the engine uses the xNVSE group numbering (the T7 first in-game check measures this).
- Dead/ragdoll puppets, VATS, knock-down and cutscene animations are not handled at all in Milestone 2.

## Round of 2026-10-22: avatars finished in code (roadmap step 3)

**Everything below is unit tested on a fake game / a simulated network and compiles into nvcoop.dll. NOTHING of it has run in the game.**
Run: `plugin\avatar\full\run_tests.cmd` (one command, ends in `avatar full: ALL OK`; registered in `tools/test_all.py` as "M2 avatar full"). It builds in the
shared tree, runs the new 3-machine suite once for real and once per runtime twin (101..109), re-runs the suites that share the changed code (`avatar_tests`
with 38 twins, `avatar_engine_tests` with 71 twins, dll-wiring's `nvcoop_live_tests` with 34 twins) and `tools/avatar/proof_avatars.py --selftest`.

| Piece | Where | What it does |
|---|---|---|
| Rates 20 Hz near / 5 Hz far / 1 Hz elsewhere | `plugin/avatar/stream_rate.h` (planner), `plugin/coop/live/avatar_net.*` (use) | Every machine publishes its 20 Hz snapshots; the host relays; each (observer, subject) pair has a tier (same cell within 2800 units = near, 1.25x hysteresis; same cell beyond or adjacent = far; else out). A snapshot that is not due is kept and released later (latest wins), so a stopped player's last place is never lost. A client's uplink to the host takes the HIGHEST tier any other player has for it. Measured in the simulation over 6 s: near 120 snapshots, far 31, out 3 to 6. |
| Appearance sync | `avatar_types.h` (Appearance: hair colour, height, weight, 130 face coefficients; EquipEntry: weapon mod flags), `plugin/coop/live/avatar_wire.*` (Look v2, kind 0x43), `win/game_backend_win.cpp` (reads) | The local look is read from the game every 2 s (race, sex, hair, eyes, hair colour, height, weight, name, FaceGen coefficients when the guarded read validates) and sent when it changes, once to every newcomer. The first look of a player now reaches its puppet (it used to be compared with an empty previous look and dropped). |
| Applying a look | `puppet_game_engine.cpp`, `puppet_manager.cpp` (`SetAppearance`), `equip_corrector.cpp` | Same template (race, sex): updated in place, no respawn. Height: `SetScale ratio` on the puppet (the template height is taken into account). Another race or sex: a new puppet from the other template, equipment kept. Weapon mods: after the weapon is worn, one `EnableWeaponMod <weapon> <slot>` per write, verified by reading the `ExtraWeaponModFlags` back; an engine that cannot tell is asked once per slot; a refusing engine is given up on after 6 tries. |
| Hit-target puppets | `puppet_manager.cpp` (health check), `puppet_engine.h` (`IsDead`, `ResetHealth`), `puppet_hits.*` (router) | A puppet is hittable by design (the combat layer needs the hit on it). It is topped up with `ResetHealth` every 2 s; one that died anyway (burst, explosion that skipped the gate) is deleted and respawned after 1 s. `PuppetHitRouter` says "is this victim a puppet and which player does it stand for" (avatar API) and hands the hit to a port; the production port is the combat layer's gate (`glueGate`, `win/avatar_module.cpp`), which sends the HitEvent to the real player's machine and answers Clamp. No combat layer: the stand-in is clamped and nothing is forwarded (counted). |
| The ghost-ref map | `plugin/avatar/avatar_api.h/.cpp` | `puppetRefOf(slot)`, `slotOfPuppet(ref)`, `actingRefOf(slot)` (the local slot answers the player's own ref 0x14, a remote slot its puppet), `slotOfActingRef(ref)`, `isPuppet`, `puppetCount`, `forEachPuppet`. Registered by the avatar module as a `ManagerDirectory` over the runtime's manager; answers live puppets only. dll-wiring's `GhostOf(slot)` and `setPlayerRefMap` can call these instead of `avatar_glue.h`. |
| Lifecycle | tests (`full/tests/test_full.cpp`) plus two real fixes | join, leave, rejoin, interior (parked), exterior neighbour cell (SetPos only, same object), another worldspace (parked), host load (orphans purged, one puppet per player), client load, link loss. Fixed on the way: equipment that arrived before the runtime existed was dropped (now remembered and replayed); a look change used to respawn the puppet. |
| Bridge | `win/avatar_module.cpp` | new: `avatar.api`, `avatar.look`, `avatar.puppetlook {slot}`, `avatar.hit {slot, damage, limb?, attackerRef?}`; `coop.live` -> `avatars` now carries per slot `tier`, `rx` (accepted snapshots, the proof turns the difference into a rate), `lookInfo`, and `selfLook`. |
| In-game proof | `tools/avatar/proof_avatars.py` (WRITTEN, NOT RUN) | Two instances: wiring, look (sent equals stored, puppet gear and weapon mods read back), rates (15..24 Hz near, 3.5..7.5 Hz after a 60 m teleport, near again), lifecycle (interior park, come back, client load, no orphan), hits (router forwarded, the client's combat counters). `--selftest` runs it against a scripted fake game with 24 injected defects, 8 of them in the scripted loadout (all caught). |

### New traps, test, twin

| # | Trap | Handler | Test | Twin |
|---|---|---|---|---|
| 27 | a far player at full rate floods the link; a near one at 5 Hz stutters | `StreamRatePlanner` tiers + hysteresis | `rates_near_far_and_out`, `planner_tiers_hysteresis_and_pending` | `107` (rate ignored) |
| 28 | a relay would show a near neighbour at the far rate | uplink = highest tier of the observers | `uplink_follows_the_highest_tier` | `108` |
| 29 | the last position is stuck behind the rate gate when the stream stops | kept snapshot released by `poll` | `last_position_is_never_lost_behind_the_rate` | `109` |
| 30 | the first look of a player never reached its puppet | compare with what the puppet was BUILT from | `look_travels_and_applies_in_place` | `106` |
| 31 | every look change rebuilt the puppet | in place unless race or sex change | same test, `appearance_update` | `105`, `appearanceUpdateRespawns` |
| 32 | a weapon mod change is not a changed look | `AvLook ==` and `SameSet` include the mods | same test, `weapon_mods_applied` | `103`, `modsIgnoredInSet` |
| 33 | mods never applied / retried forever when refused | corrector mod step with give-up | `weapon_mods_*` (3 tests) | `noWeaponMods`, `modsRetryForever` |
| 34 | junk look data on the wire | reserved bits, mod flags, face cap rejected | `wire_look_v2_roundtrip_and_rejections` | `101`, `102`, `104` |
| 35 | a puppet dies of a stray hit and stays a corpse | `IsDead` poll, `ResetHealth` top-up | `puppet_hit_target_health`, `stack_hit_target_*` | `noPuppetHeal`, `noPuppetDeathRespawn`, engine `85` |
| 36 | the API repeats a lie (local slot or the real player as a puppet) | filtered in `avatar_api.cpp` | `api_map` | `apiLocalGetsPuppet`, `apiPlayerIsPuppet`, `apiActingNeedsPuppet` |
| 37 | a hit on a puppet kills the stand-in, or is forwarded without damage | `PuppetHitRouter` | `hit_router`, `hits_on_a_puppet_reach_the_real_player` | `puppetHitProceeds`, `hitsForwardEmpty` |
| 38 | body height of the remote player | `SetScale` ratio on the puppet | `appearance_height_is_matched_with_setscale` | engine `80` |

### Engine facts found this round (static, from the command table and the CommandInfo structs of a reference exe)

| Fact | Source | Status |
|---|---|---|
| `EnableWeaponMod` (opcode 0x1CB, handler 0x5DDAE0): needs a calling reference, parameters (Weapon, Mod Slot optional), help text "Mods the current weapon with the slot" | CommandInfo at 0x1190898 | the semantics (1-based slot, acts on the worn weapon of the calling actor) are a HYPOTHESIS; the proof reads the flags back |
| `ResetHealth` (0x1150, handler 0x5C6B60): needs a calling reference, no parameters | CommandInfo | used by the health top-up; unmeasured |
| `SetScale` (0x113C): calling reference, one float | CommandInfo | used for the height; unmeasured |
| `ModifyFaceGen` (0x1C4: String, optional Integer x2), `HairTint` (0x14C: 3 integers) take NO calling reference | CommandInfo | NOT used: they could change the LOCAL player. `MatchFaceGeometry` (0x11DF: Actor, Value) copies a face between two actors on one machine, which needs the remote player's NPC here |
| `ExtraWeaponModFlags` = extra data type 0x8D, flags byte at +0x0C | xNVSE GameExtraData.h | read from the WORN instance's extra data list (`ReadEquippedRaw`), guarded |
| `TESNPC::faceGenData[3]` (`values`, `useOffset`, `count`, `size`) | xNVSE GameForms.h comments | layout NOT proven: two readings are tried, one counts only if every float is finite and plausible; the numbers are sent and logged, nothing applies them |

### Still not done (honest)

- **Face, hair style and eyes of a remote player are not applied.** The data (hair, eyes, hair colour, face coefficients) travels, the puppet shows the template's face. A real
  fix needs a clone of the remote player's NPC base on every machine (or a FaceGen write path); `MatchFaceGeometry` is the nearest console tool and is not enough. The weight has no
  command at all. Both are reported through `appearanceApproximate`, never faked.
- **Hits on a puppet** are forwarded only through the combat layer (`[live] combat=1`). The router exists so the answer is the same everywhere; without combat nobody carries the report.
- Everything under "TODO-RE" above, plus: whether `EnableWeaponMod` and `ResetHealth` do what the command table suggests, whether `SetScale` on a restrained actor is accepted,
  whether the FaceGen reading validates, whether a puppet shot by an enemy plays a hit reaction that breaks the pose for a moment (the re-assert logic should catch it).

## Honest status

- Unit tested on a fake game and on the real live layer: yes (see Run). Proven in the game: **nothing**. The fake models the engine as we expect it to behave; every row in the traps
  table is a hypothesis about the engine until `proof_puppet.py` has run once.
- Thresholds (speed bands 20/190/230 u/s, debounces, parking grace, heartbeats) are guesses to tune from T7 counters.

- Engine round (2026-10-21): the real `IPuppetEngine`, the local-player sampler, the runtime with the dev loopback and the Windows backend are written and
  unit tested on a fake game (59 tests, 540 checks, 63 twins). The game side compiles into nvcoop.dll. Every pinned address is proven against the
  reference exe (24 checks); what the engine DOES when called that way (PlaceAtMe of the template, SetPos on a restrained actor, SetAlert drawing a weapon,
  the group numbering, the cost of a line, the cell map walk, the worn-item walk) is unknown until `proof_puppet.py --loopback` has run once.

### Scripted client loadout for the look phase (replaces the hand step "give the client armor and a modified weapon")

`tools/avatar/proof_avatars.py` has `--loadout auto|off` (default `auto`, applied when the look phase runs) with `--armor`, `--weapon`, `--weapon-mod` and `--mod-flags`. On the CLIENT it runs
`player.additem` and `player.EquipItem` for the armor and the weapon, then the xNVSE function `player.SetEquippedWeaponModFlags <flags>` (xNVSE `Commands_Inventory.cpp`, flags 0..7, bit n-1 = mod
slot n) on the worn weapon, and reads each step back: `player.GetEquipped`, `player.GetEquippedWeaponModFlags`, then the avatar layer's own `avatar.look` (worn item with `mods` equal to the flags).
The look phase additionally FAILS (it no longer passes silently) if the client holds no modified weapon or no armor. Defaults, from `out/questdata/forms.json.gz`: armor `FalloutNV.esm:1264FE`
(ArmorLeatherReinforced, ARMO), weapon `FalloutNV.esm:0CD539` (WeapNVTrailCarbine, WEAP), mod `FalloutNV.esm:0EEA70` (ModNVTrailCarbineScope, IMOD, the one IMOD of that dump whose name pairs with a WEAP),
flags 1. There is no console function that attaches an IMOD to an inventory item (`AttachModToInventoryItem` is in neither the vanilla command table nor the xNVSE source of this repo; only
`EnableWeaponMod`, whose meaning is still the open hypothesis above), so the flag byte is set directly: it is exactly what the avatar layer reads (`ExtraWeaponModFlags` of the worn instance) and syncs.
Risk to read first: `SetEquippedWeaponModFlags` returns silently when the equipped item has no extra-data list; the read-back line then fails with "read back 0 wanted 1". Twins in the self-test:
loadout skipped, `additem` refused, weapon or armor not equipped, flags line without effect, flags set but not visible to the avatar layer, wrong flags.

## Round of 2026-10-27: how far behind the other player's puppet appears

**Unit tested and replayed on a model, game-free. NOTHING of it has run in the game yet.** The lab (`tools/lab`, room "walk") was the complaint: puppet error p95 97 to 128 units on a clean
link, about 80 behind good and bad Wi-Fi, sync lag 94 to 141 ms. The code is `synccore/live` (the `InterpBuffer` the avatar stream is played back through, a new opt-in low-latency mode; the NPC
path keeps the old behaviour) and `plugin/avatar/engine` (`avatar_tuning.h`, the constants; `local_sampler`, the sender); the test is `plugin/avatar/engine/tests/test_lag_sim.cpp`
(`avatar_lag_tests`, run by `plugin\avatar\engine\run_tests.cmd`).

### Where the lag came from (each line is a measurement in the model)

| Source | What it did | Now |
|---|---|---|
| Fixed render delay | `InterpConfig::delayMs = 100`. The "adaptive" delay only ever raised it (1.25 x the snapshot interval is 62 ms at 20 Hz), so a clean link had 100 ms of buffer for nothing: 104 of the 116 ms steady lag at 60 fps | delay = 20 ms + the measured arrival jitter (lateness over the fastest arrival, one frame of quantisation taken off), at most 120 ms |
| Interpolation only | the pose was always between two snapshots; nothing predicted the stretch after the newest one, so the delay could not be cut | dead reckoning on the snapshot's velocity, damped (the displacement saturates at v x 150 ms, so a stop cannot run away), capped at 250 ms |
| A prediction miss popped | a snapshot that disagreed with the prediction moved the pose in one frame | the difference is carried as an offset that fades out (70 ms) |
| Lumpy render clock | the clock offset is the minimum of the last 16 arrivals and moves by whole ms at once; 14 to 16 unit steps at 300 u/s were seen (note in `test_live_integration.cpp`) | the render clock (offset + delay) slews at 10 percent |
| 20 Hz near | information about a start, stop or turn is up to 50 ms old when it is sent | 30 Hz near (sampler and relay); far 5 Hz and out 1 Hz untouched |
| Sender velocity | `velAlpha = 0.6` made a stop report 0.4 x the old speed for several samples | 0.8 |
| How positions are timestamped | NOT a cause: `tick` is the sampling time, taken in the same call that reads the position | unchanged |
| Waiting for 2 samples | the old buffer needed a sample after the render time, so its delay had to exceed the interval | the new one extrapolates, so it does not wait |
| Smoothing constants (2 m / 0.25 m / 200 ms) | only act on errors above 140 units; not a cause when walking | unchanged |
| The frame rate of the game copies | the floor of everything: a position is only sampled, sent, received and applied once per game frame | see the table: 10 fps copies cannot be fixed by the buffer |

The model reproduces the lab's OLD numbers at about 8 to 10 fps (clean 88 to 106 units, bad Wi-Fi 111 to 124), but gives 46 at 60 fps. So the lab's two copies on one PC probably run at roughly
10 fps, and at that speed a good part of the error is the frame length itself (a full game frame between two samples of a 400 u/s walker is 40 units). Read the lab again with that in mind.

### Before / after

Error p95 while the walker moves / max error / overshoot past a stop, in game units, worst of 3 seeds, 400 u/s running walker (`avatar_lag_tests --report --fps N`). The walker starts, runs,
turns 90 degrees, stops dead, strafes, walks, turns, stops, reverses at full speed (zig zag) and stops; both game copies run their own frames (15 percent frame time jitter, the receiver a third of
a frame later) and the walker's position is the one his copy last set, exactly what the lab reads. No teleport and no snap in any row, before or after.

| profile | 60 fps before | 60 fps after | 30 fps before | 30 fps after | 10 fps before | 10 fps after |
|---|---|---|---|---|---|---|
| lan | 46.4 / 111 / 0 | **15.1** / 28.6 / 4.8 | 54.0 / 86 / 0 | **21.0** / 41.6 / 5.9 | 88.4 / 123 / 0 | 62.3 / 98 / 20.6 |
| good_wifi | 49.2 / 59.5 / 0 | **19.3** / 32.4 / 4.4 | 56.4 / 96 / 0 | **27.1** / 41.0 / 5.7 | 93.6 / 123 / 0 | 67.7 / 104 / 25.2 |
| bad_wifi | 52.0 / 66.5 / 19.6 | **33.9** / 54.6 / 4.6 | 78.6 / 145 / 1.8 | **37.0** / 60.1 / 10.8 | 124 / 173 / 2.2 | 98.0 / 139 / 25.6 |
| mobile_tether | 63.6 / 90.0 / 15.1 | 48.3 / 59.4 / 7.9 | 87.2 / 173 / 13.5 | 51.5 / 78.2 / 8.7 | 185 / 328 / 0 | 102.8 / 157 / 28.7 |
| transatlantic | 79.2 / 86.5 / 0 | 50.6 / 59.4 / 4.2 | 88.8 / 118 / 0.1 | 58.1 / 65.6 / 6.7 | 122 / 158 / 0 | 102.8 / 138 / 22.8 |

Sync lag (time until within 80 units after a stop): 0 ms in every row at 60 and 30 fps, before and after (the model's stop never leaves the puppet more than 80 units away); time until within 10 units: lan 30
to 59 ms before, 0 after; bad_wifi 429 ms before (it overshot by 20 units), 0 after. Jitter (largest per-frame step deviation in a steady run, percent of the nominal step; frame time jitter alone gives
17 to 25): lan 17 before, 24 after; bad_wifi 100 before, 53 after; mobile_tether 240 before, 48 after.

The goal of the round (p95 under 40 clean, under 60 behind bad Wi-Fi) is met for game copies running at 30 fps or better. At 10 fps it is not: the improvement there is about 30 percent, and the rest is
one frame time of the sender plus one of the receiver, which no playback buffer can recover.

### Bandwidth

Payload of the snapshot datagrams of one stream (`SnapshotSender::stats().bytes`) while running steadily:

| near rate | snapshots/s | payload B/s | with about 44 B of UDP, IP and ENet header per datagram |
|---|---|---|---|
| 20 Hz (before) | 20 | 360 | about 1240 B/s |
| 30 Hz (now) | 30 | 549 | about 1870 B/s |

About 18 bytes a snapshot (the delta codec). Standing still costs one snapshot a second. The host relaying three players sends three such streams to each client: well under 20 KB/s.

### Settings (nvcoop.ini [avatar], all optional; `avatar_tuning.h` has the defaults and is the one place they live)

`lowLatency=1` (0 = the old fixed 100 ms buffer for the receiver), `nearHz=30` (sampler and relay, 10..60), `minDelayMs=20`, `jitterFactor=1.0`, `maxDelayMs=120`, `extrapolateTauMs=150`,
`maxExtrapolateMs=250`, `blendMs=70`, `slewPerMille=100`, `velAlpha=0.8`. The relay (live_glue.cpp) reads the same keys through `iniOrEnv`, so `NVCOOP_AVATAR_NEARHZ` works for it; the avatar module reads
the ini only. `LAG_TUNE="minDelayMs=10,nearHz=60" avatar_lag_tests --report` sweeps them without rebuilding.

### Tests and twins

- `avatar_lag_tests` (59 checks): the five chaos profiles x 3 seeds at 60 fps with limits for p95, max error, overshoot, teleports, snaps, backward steps, jitter, settle time and bytes; 30 and 20 fps
  copies; "the new one is at least 2x (clean) / 1.4x (bad Wi-Fi) closer than the old one". Twins 201 to 208 (`FNVCOOP_MUT`, all caught): delay ignores jitter, undamped dead reckoning, no fade of
  prediction misses, unslewed clock, the old buffer, a velocity filter of 0.15, a 10 Hz near rate, no extrapolation.
- `synccore/live`: four scenarios with twins (`ll_fixed_delay`, `ll_no_damping`, `ll_no_blend`, `ll_no_slew`) on the buffer alone.
- `avatar_live_tests` runs its walk, outage, teleport and engine-push scenarios through the new buffer as well (largest single step 5.7 to 7.9 units at 300 u/s; the old buffer's was 14 to 16).

### Not done / honest

- Nothing has run in the game. The model is the real sender, codec, planner, receiver, buffer and puppet driver on a fake game; it is not the game's frame pacing, its physics step or the bridge read of the lab.
- A stop that the sender's snapshot announces late still overshoots, bounded by the damping: 3 to 11 units at 30 and 60 fps in the model, 18 to 29 at 10 fps.
- Possible extra frame on the send side: if the coop module's frame runs before the avatar module's (module registration order), the snapshot sampled in frame N leaves in frame N+1 (`AvatarNet::tick`
  takes `localOut_`). It would add one sender frame to every number above. Not verified; fixing it means calling the net tick from the avatar module (plugin/coop/live).
- Considered and not built: event driven sends at a turn or a stop (up to the frame rate, only then; no effect at 10 fps). Modelled and left out because the gain was small: acceleration aware dead reckoning
  (overshoot 4.8 to 3.2 units, p95 unchanged; velocity noise in the real engine could turn it into jitter) and a longer clock-offset window (no change).

## Round of 2026-10-28: the missing ~100 ms found in the game (branch wt/avatarlag2)

The lab's walk room (`lab_run.ps1 -Rooms "walk,spawn" -Pairs 1`, second PC, every copy at 60 fps) showed the puppet 130 to 150 ms behind on a clean link where the model said 27. Stage timers
(`avatar.status` "stages") and the game's own trace (`avatar.trace`) say where the time goes.

### Stage timers (ms on one clock unless the name ends in Us; every figure is a gauge `{n, last, mean, max, sum}`: the difference of two `sum`s over the difference of two `n`s is the mean between two readings)

| field | what | clean link, 60 fps, host/client copy |
|---|---|---|
| `stages.send` | the snapshot's sampling time to the moment its datagram is handed to the transport (sender, `AvatarNet::flushStreams`) | 2 to 7 |
| `stages.rx` | sampling time to the moment the other copy's network layer has it (a shared clock on one PC; across PCs it includes the clock offset) | 7 to 17 |
| rx minus send | the wire and the receiver's poll (the lab's `transport_ms`) | 6 to 13 |
| `stages.remotes[].hold` | arrival stamp to the first drive that could use it: the co-op module is a late module, it polls AFTER the avatar module drove | 15 (one frame, by design) |
| `stages.remotes[].delay` | the buffer delay in use (render age minus the clock offset); `wantDelayMs`, `jitterMs`, `offsetMs`, `lagErrMs` explain it, `lowLatency` says which playback runs | **21 to 27 now, 150 to 170 before** |
| `stages.netPhase` | the network frame minus the avatar frame of the same frame | +0.5 (the late module order holds) |
| `stages.applyUs`, `poseToSetPosMs` | the engine calls of a tick (SetPos ...); from the start of the tick to their end | 0.3 ms, 0.4 ms |
| `stages.localDrift3D` | the local player's reference position against his 3D node | not read out in these runs |
| `lowLatency` (top level and per remote) | whether the low-latency playback is on | true |

`avatar.trace {n}`: the last n published samples of the walker and each puppet's fed pose per frame (with the delay in use, the wanted delay, the jitter and the newest snapshot's age), on one clock.
The lab's walk room lays them over each other (`lab_rooms.trace_lag`: `{who}_trace_lag_ms`, `_trace_err_p95_now`): no bridge read in it. `LAB_TRACE_DIR=<dir>` keeps the raw traces. The lab's per-leg
numbers are `{who}_leg_send_ms`, `_rx_ms`, `_transport_ms`, `_hold_ms`, `_delay_in_use_ms`, `_apply_ms`, `_net_phase_ms`, `_buf_*`. Spawn-room traces (`puppet_trace`) now carry the puppet's pose and applied place and the network service's counters.

### The stage that added the ~100 ms: the render delay, not the pipeline

Everything between the sampler and the puppet is as designed (send 2 to 7 ms, wire 6 to 13, one frame of hold, SetPos 0.3 ms, the net tick after the avatar tick). The buffer's delay in use was
**150 to 170 ms while the delay it asked for was 20 to 30**. Cause (`[avatar] legacyRules=1` reproduces it): a player who stands still sends one heartbeat a second; the interval estimate averaged
those gaps in, so the buffer took the stream for a slow (far tier) one and asked for 1.25 x the interval, its 350 ms ceiling; the render clock follows the wanted delay at 10 percent (`slewPerMille`),
so after every stand it needed seconds to come back down, and the lab's walk (four 300 unit legs with stops) never got there. The same cause showed a teleport, or a start to walk after a stand, 350 ms
late. The model never had a stand longer than 1.5 s, and its "before" row (the old fixed buffer) hid it.

Fixes (each has a test and a twin):
- `InterpBuffer`: the stream's interval is the smallest of the last four gaps, gaps over `intervalCapMs` (400) are silence and not counted, a rate is claimed from three gaps on (twin `llOldInterval`: lag sim 210, synccore `ll_old_interval`).
- Snap rule: a snapshot is a **jump** (`Snap::jump`) when the cell changes, when it is further from its predecessor than `teleportUnits` (350) plus `teleportSpeed` (800 u/s) x the time between them, or when a standing player is more than `silentJumpUnits` (140) from where the previous snapshot stood after a silence or as a standing one again. The puppet holds the old place until the jump's time and then snaps; the render time crossing a jump snaps even when the way looks short, and a target that moves further than that in one frame snaps too (the old rule eased every error between 2 and 10 m with a 200 ms time constant). Twin `noTeleportSnap` (lag sim 211, synccore `no_teleport_snap`).
- Sender: a jump of more than 100 units and faster than `maxSpeed` in one period is a teleport (velocity restarted); only more than 350 units were. A 200 unit jump went on the wire as 2200 u/s and the receiver extrapolated it 330 units past the target. Twin 40 and engine test `sampler_small_jump_is_a_teleport_not_a_speed`.
- Rate planner: a snapshot more than `jumpUnits` (350) from the last one sent on a link, or in another cell, goes at once whatever the tier's period says (twin 209, live wiring test `avatars_teleport_arrives_at_once`).
- Network service: a snapshot that arrives before the roster that names its player was acked and then dropped, so a standing host sent nothing but a heartbeat a second from then on: the puppet the roster made stood 773 units off for 0.7 to 2.7 s. It is kept (newest per player) and fed to the puppet the moment it exists (twin 212, `avatars_a_snapshot_ahead_of_the_roster_reaches_the_puppet`).
- Dev switch: `[avatar] legacyRules=1` turns the four rules above off for before/after runs (`lab_run.ps1 -ExtraIni avatar.legacyRules=1`; the lab keeps the key in nvcoop.ini from the run before, so a normal run passes `avatar.legacyRules=0`).

### Before / after in the game (lab walk room, one pair; error p95 in units, host copy / client copy; delay in use and the game's trace lag in ms)

| pass | before: error p95 | before: delay in use | before: trace lag | after: error p95 | after: delay in use | after: trace lag |
|---|---|---|---|---|---|---|
| clean (4 and 5 runs) | 75 to 86 / 95 to 102 | 150 to 171 | 55 to 125 | **10 to 28 / 19 to 30** | 21 to 27 | 35 to 50 |
| good_wifi | 92 / 94 | 166 / 171 | 120 / 175 | 21 / 18 | 32 / 30 | 65 / 55 |
| bad_wifi | 83 / 101 | 180 / 166 | 125 / 145 | 31 / 28 | 50 / 57 | 85 / 85 |
| mobile_tether | 84 / 92 | 170 / 188 | 165 / 200 | 44 / 40 | 66 / 76 | 110 / 120 |
| transatlantic | 79 / 104 | 107 / 193 | 175 / 285 | 46 / 50 (the link alone is 90 ms one way: 34 units at this speed) | 30 / 34 | 135 / 130 |

The model, `avatar_lag_tests --report` (it now runs the co-op module after the drive and the planner on a 15.6 ms clock, the order and the clock of the game, and has a teleport, a start from a long stand
and the lab's walk room as tracks): lab walk room p95 clean 65.6 before / 22.3 after, bad Wi-Fi 64 / 38; a 450 unit teleport arrives after 435 ms before / 36 ms after (clean); the first 1.5 s after a
6 s stand have an error of up to 113 before / 24 after.

### Not fixed / honest
- The spawn room's slow first sight of the other player (650 to 2700 ms): after the roster fix 0.19, 0.20 and 0.70 s in three runs on good Wi-Fi (before it: 1.0 to 2.8 s in four of four profile passes, and fast in others; the slowness is random per run and also seen with `legacyRules=1`). The 0.70 s case still has the newest snapshot 1.5 s old when the wait starts: a snapshot that reaches the client late. Not closed completely.
- ENet's packet throttle may drop unreliable snapshots on a clean link (published 28 to 30 Hz, received 24 to 25 Hz in every pass). Not touched (the transport is not this round's); it costs no lag, only gaps the buffer extrapolates over.
- Behind mobile_tether and transatlantic the error is mostly the link's own one-way delay times the speed; the buffer adds 30 to 75 ms.
- The tests above ran in the game on branch wt/avatarlag2 (build of 2026-10-28 06:32 for the final DLLs); the profile table is from the build before the roster fix (the walk code is the same).
