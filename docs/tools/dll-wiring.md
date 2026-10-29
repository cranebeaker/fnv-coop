# dll-wiring: the live layer inside nvcoop.dll (enemy / NPC / avatar sync)

Round: enemy / NPC sync (2026-10-21). Owner: dll-wiring. **Everything here is unit-tested game-free. NOTHING has run in a game.**
What the lanes do is described in their own pages (`M3-actors.md`, `avatar.md`, `combat-hooks.md`, `enemy-anim.md`, `spawn-arb.md`); this page is
about how they are switched on, ticked, fed with the network and told about loads and leaves.

## The switches (nvcoop.ini, all OFF by default)

```
[live]
actors=0     NPC ownership + the real engine port (plugin/actors, actors/engine, actors/anim)
avatars=0    player puppets + the avatar network channel (plugin/avatar, avatar/engine, plugin/coop/live/avatar_net.*)
combat=0     hits, deaths, projectiles (plugin/combat)               needs actors=1
spawns=0     spawn-site arbitration (plugin/spawn)                   needs actors=1
social=0     companions / crime / kill rewards (plugin/companions)   needs actors=1; gives the module the avatar puppet map and a status line. The module is
             started by [companions] enabled=1 and its frame is driven by coop_module.cpp (the one glueFrame call site); this switch never starts it.
             The status line lists " social=1" only when it is on.
[actors]     tuning of the real actor port; every value is clamped (live_actors_cfg.h); engine=0 = no port at all
```
* Values `1 true on yes` (any case) are on; everything else, a typo included, is off.
* Environment beats the ini: `NVCOOP_LIVE_ACTORS=1` (also `_AVATARS _COMBAT _SPAWNS _SOCIAL`), `NVCOOP_ACTORS_SCANMS=250` ... (`NVCOOP_<SECTION>_<KEY>`).
* A dependency that is missing turns the dependent switch off and says why (`coop.live.warnings`, nvcoop.log): `combat=1` without `actors=1` is ignored.
* Provision test instances with `python -I tools/runner/run.py provision --n 2 --ini coop.enable=1 --ini live.actors=1 --ini live.avatars=1 ...`.
  **The co-op playbooks that used `coop.owner` (`pb_actors`, `run_actor_handoff`) now need `--ini live.actors=1`**: before this round the actor
  module ticked with a no-op port whether you asked or not.
* `[live] avatars=1` also creates the avatar runtime (the avatar module normally needs `[avatar] enabled=1`; the module reads the switch). `[avatar] enabled=1`
  alone still gives the dev loopback and the `avatar.*` bridge methods without any network.
* `[live] actors=0` keeps the actor port INSTALLED but unfed (`[actors] engine=1`, default): it never tracks an actor without a session, and its only
  work is undoing the restraints a crashed run left in its ledger. `engine=0` removes even that.

## How the player is carried: `[entities]` (docs/entity-core-plan.md, wave 2)

```
[entities]
parts=unified    look, equipment, held weapon, facts and pack are parts of the player entities (ActorSync's part replicator); the roster is the set of player records.
                 AvatarNet sends no Roster / Look / Facts / Pack / Resync (it still decodes all of them). legacy = the old messages on the avatar channel.
stream=unified   the player's snapshot stream goes through ActorSync's player profile (own sender / receiver, relay on arrival, near / far / out tiers) instead of
                 the avatar channel 0xAD. legacy = AvatarNet's own stream, unchanged.
```
* Both default to `unified` (the lag gate of step S11 held, `avatar_lag_tests --gate`); `legacy` is a fallback switch. A typo keeps the default. Environment: `NVCOOP_ENTITIES_PARTS`,
  `NVCOOP_ENTITIES_STREAM`. Read once, when the session starts.
* Both need `[live] actors=1` and `[live] avatars=1` (ActorSync carries them, the player layer is the other end); without those they stay with the avatar channel.
* `coop.live` shows what runs under `entities` ("parts=unified stream=unified"); the `switches` line keeps its four lanes. The live glue gives each new `ActorSync` the player port
  every frame (`bindPlayerEntities`), the roster comes from its player records, and the entity map's puppet binds tell it to dress a respawned puppet again.
* With `stream=unified` AvatarNet's own stream counters (`sent`, the `stream` block) stay 0; the traffic is in ActorSync's stats (`playerSampled`, `playerSent`, `playerAccepted`,
  `playerRelayed`, `playerForged`, ...).

## Shape

```
NVSEPlugin_Load (main.cpp)  glueConfigure(ini)      [live] + [actors] read BEFORE any module's onLoad
coop module onLoad           glueLoad()              hub + one service per lane (enabled or placeholder), bridge methods coop.live, coop.spawn
every frame (coop OnFrame)   glueFrame(rt, now)      AFTER Runtime::frame() (it polls the mux), also with no session
NVSE messages                glueMessage(...)        PreLoadGame, PostLoadGame, NewGame, SaveGame, ExitToMainMenu, ExitGame
hot reload / unload          glueShutdown()          ends the session part of every service before the runtime goes
```
`plugin/coop/live/` (CMake target `nvcoop_live`, game-free) holds the switches, the **LiveHub**, the avatar network service and `live_actors_cfg`;
`live_glue.cpp` is the only game-facing file (a direct source of nvcoop.dll) and the lane services are small classes in it.

### LiveHub (live_hub.h)
* A **service** declares the mux channel bytes it owns; a **disabled service is never called**. The hub registers the channels on the session's
  `MuxTransport` and routes datagrams by sender slot; peers that have not joined are dropped and counted. Two services cannot claim one byte.
* A **session** begins when the runtime is running with a mux (a client also needs its slot or an up link) and ends when it stops, when the mux object
  changes or when a client is re-slotted. Handlers of an old session stay on the old mux but are inert (token check).
* Events: `sessionBegin/End`, `playerJoined/Left` (host; clients that were already there are announced right after the begin, each exactly once),
  `hostLinkUp/Down` (client), `tick` (session only), `idleTick` (no session), `loadBegin/loadEnd/saved/gameExit` (always, session or not).
* A C++ exception from a service switches THAT service off (`serviceFaults`, `fault` in `coop.live`) and the frame goes on; the others never notice (twin 23).
* Services end in reverse order; `ILiveLink::send(slot, reliable, bytes)` maps slot to peer (host: joined clients only; client: the host).

### Who does what on which event
| Event | actors | avatars | combat | spawns | anim |
|---|---|---|---|---|---|
| session starts | `ActorRuntime` is built on the first tick (own mux channel 0xAC) | `AvatarNet.sessionBegin`: identity, sinks, roster | `CombatRuntime` built by its `glueFrame` (0xCB) | `SpawnRuntime` (0xA5); engine adapter built over the port's engine | listeners attach to the new ActorSync |
| player joins / leaves | inside `ActorRuntime` | host: puppet, Roster to everybody, Looks to the newcomer; leave: puppet removed | inside `CombatRuntime` | inside `SpawnRuntime` | - |
| host link drops / returns (client) | `hostLost / hostConnected` | puppets down; Resync asks the host for roster + looks | same | same | - |
| PreLoadGame | the port's own module marks the world as loading | the avatar module drops its refs without calling the engine | - | - | epoch bump |
| PostLoadGame / NewGame | `purgeStandIns()` (stand-ins are never saved or kept) | the avatar module schedules the orphan purge, puppets respawn after | - | `onWorldLoaded()` (bindings, held rolls, candidates cleared) + the host reads `<save>.fnvspawn` | epoch bump |
| SaveGame | - | (puppets are purged at load; the module has `avatar.presave/postsave` for scripted saves) | - | host writes `<save>.fnvspawn` | - |
| session ends / exit | `glueFrame(nullptr)` tears the ActorSync down | `removeRemote` for all, resolvers cleared | `glueFrame(nullptr)` | `glueFrame(nullptr)` | - |

Channels in use: `0xFC` story, `0xD1` dialogue, `0xE5` slides, `0xAC` actors, `0xCB` combat, `0xA5` spawn, **`0xAD` avatars** (registered in `docs/protocol.md`).

### Avatar network (avatar_net.*, avatar_wire.*)
Per machine: the local player's 20 Hz snapshots come from the avatar module (sampler) through its sink, are published as `ActorSnapshot`s of netId
`0xFE0000<slot>` with the live delta codec (unreliable); the other players' snapshots go to the module's interpolation buffers and puppets.
* Star topology: a client talks to the host only; the host relays every accepted snapshot to the other clients (`avAll` copied into `avDeltas`).
* **No forged avatars**: the host accepts a client's snapshot only when its netId is that client's own slot (`stats.forged`), and a Look only for the
  sender's own slot. Rosters are accepted from the host only. Every message is decoded whole or rejected (truncation, trailing bytes, slot >= 8).
* `Roster 0x40` (host to clients, reliable): who exists. `Look 0x41`: appearance + worn items, when they change and once to every newcomer.
  `Resync 0x42` (client to host): "my link came back", answered with the roster and every look.
* The ownership service learns every remote player's place from the newest snapshot (`actors::engine::setPlayerPose`), so interest management works
  even when no puppet exists on this machine. `setPlayerNetIdResolver` lets combat targets between actors and players survive the wire.
* The acting-player design (`GhostOf(slot)` in coop_module) now prefers the puppet of that slot over the NV:MP ghost.
* Clock: the avatar module's buffers run on its QueryPerformanceCounter milliseconds; `GlueSide` stamps arrivals with the same clock (a mismatch would
  freeze every puppet).

### Lanes
* **actors**: `ActorsService` ticks `actors::glueFrame(rt)` and sets `setRefExclusion` (player puppets and NV:MP ghosts are never tracked as NPCs).
  The real port is installed by `actors/engine/engine_module.cpp`; its `ReadConfig()` now reads through `readActorsConfig` (same keys, clamped,
  environment override). `EngineWorld::apply` (coop_module) calls `glueOnEffect` first, so a story `KillActor` / `ResurrectActor` / `CompanionOwner`
  reaches `engine::onEffect` before the engine acts on it. The bridge's `saveGame` calls `engine::beforeSave()` (clones out of the world before the
  .fos is written); a save the player triggers (menu, quicksave, autosave) has no known pre-save hook (TODO-RE), the load-time purge covers it.
* **anim**: `actors/anim/anim_module.cpp` self-registers and attaches to the port's callbacks; with `actors=0` no callback ever fires, so it needs no switch.
* **combat**: `CombatService` ticks `combat::glueFrame(rt)` (which installs the hit gate only while a session runs); with avatars on,
  `setPlayerRefMap` gives it the puppet map.
* **spawns**: `SpawnService` sets the engine adapter (built from `actors::engine::actorEngine()` + the world lane's spawn ledger + the
  WorldApplier that coop_module hands over), `setGameClock` (host: GameDaysPassed x 24 h, SEH read, wall clock when unreadable), the created-id
  resolver, the tracked / died listeners, the ignore list (puppets, stand-ins), every player's place (`spawn::setPlayerPose`, from the avatar pose sink, the
  local slot included) and the host's script `PlaceAtMe` (`nvworld::setTxnObserver`, a read-only look at each captured txn BEFORE the coop runtime's sink
  takes it; the SpawnRef's cause becomes the site anchor, `describe()` supplies the interior flag, then `spawn::onHostSpawnEffect`).
  `rebase` and `extras` have no verified engine path (TODO-RE, plugin/spawn/engine_adapter.h): left absent and counted, not faked.

### Edits made inside other lanes (small, additive; the owners should keep them)
* `plugin/actors/engine/actor_engine_api.{h,cpp}`, `registry.h`: `IActorEngine* actorEngine()` (the engine the port runs on, for spawn-arb's adapter).
* `plugin/actors/engine/engine_module.cpp`: `ReadConfig()` calls `readActorsConfig` when built with the live layer (the old body stays for builds without).
* `plugin/hooks_world/world_hooks.{h,cpp}`: `setTxnObserver` (read-only look at captured txns for spawn arbitration); `world_query.*` (the evaluator plan).
* `plugin/avatar/engine/win/avatar_module.cpp`: `[live] avatars=1` enables the module.
* `plugin/CMakeLists.txt`: `combat spawn` added to the lane list; `coop/live` added after all lanes. `synccore/CMakeLists.txt`: perf guard tests.

## Track A inbox fixes

1. **`world.query GetActorValue` failed for every ref.** Our L5 detour on `Cond_GetActorValue` (0x59C4F0; also `Cond_GetItemCount` 0x59D8E0) puts a
   `jmp` at the evaluator's entry and `queryCondition` refused anything without `55 8B EC`. Now `plugin/hooks_world/world_query.cpp`
   (`resolveEvaluator`) decides: original prologue = call the live entry; our own jump (the hook registry knows its trampoline:
   `nvhooks::trampolineOfAny`, new in `hook_patch.*`) = call the TRAMPOLINE (the original code, and it does not run our capture detour); a foreign
   jump, a changed entry or a trampoline that does not start with the original prologue = refuse. Twins 43 to 46 in the L12 suite. The Track B bots may keep
   their `eval GetAV` fallback; it is no longer needed.
   Note: `plugin/avatar/engine` pins its evaluators once at load (before the hooks exist); after a hot reload `GetItemCount` would be pin-refused and
   falls back to its own bookkeeping. Not fixed (their code); the same helper would do it.
2. **perf group flaked under load.** `synccore/tests/perf_guard.h` (tests: `test_perfguard.cpp`, group `perfguard`, twins 1 to 10, all caught):
   before every attempt it probes the CPU slowdown (calibration loop) AND the longest scheduler stall; budgets get `x k` for throughput and `+ stall`
   for worst-frame numbers; a saturated machine is waited on (up to 2 s), gets 6 attempts instead of 3 and is judged on the per-iteration MEDIAN instead
   of the p90 (a preempted iteration lasts whole time slices, so a tenth of them can be inflated 5x; the median still moves with real slowness);
   caps (16x, 60 ms) stop an absurd probe from excusing a regression. Measured here (not a gate): 24 busy processes on 12 cores, 6 runs of the
   perf group, 6 passes (the first run needed 4 retries). A real regression still fails: the unit tests prove it on a simulated machine, and the ctest twin
   `twin_perf_regression_still_fails` runs the real timing test with every budget 10x tighter and must fail.
3. Not mine, still open: the playbook convergence window (0.3 s) is handled by fight-test (`docs/tools/enemy-tests.md`).

## Tests and how to run them
| What | Command |
|---|---|
| ini switches, hub, avatar net on a 3-machine loopback (real `AvatarRuntime` on a fake backend), wire codec, `[actors]` ini; 34 twins | `plugin\coop\live\run_tests.cmd` |
| world.query evaluator plan (+ twins 43..46, suite 1..46) | `plugin\hooks_world\run_tests.cmd` |
| perf guard (twins 1..10 + the regression twin), the real perf group | `synccore\run_tests.bat` (ctest names `unit_perfguard`, `twin_perfguard_*`, `unit_perf`, `twin_perf_regression_still_fails`) |
| in-game proof logic (fake game + 17 injected defects) | `python -I tools/runner/live_wiring_proof.py --selftest` |
All four are in `tools/test_all.py`. Twin numbers of the live suite: 1-4 switches, 11-23 hub, 31-33 wire, 41-45 and 47-51 avatar net, 61-64 `[actors]`.

## In-game proof (for the helper; nothing below has been run)
1. Defaults: provision one instance with no `live.*` keys. `python -I tools/runner/live_wiring_proof.py --instances 0 --expect none`. PASS = every lane off,
   `coop.owner` answers `known:false`, nothing of the live layer in `nvcoop.log` except the `live: switches actors=0 ...` line.
2. Actors: two instances, `--ini coop.enable=1 --ini live.actors=1` (host + client as usual), `--expect actors`. PASS = both in a session, `actors.port`
   says `session:true`.
3. Avatars: add `--ini live.avatars=1`, `--expect actors,avatars`: each side lists the other in its roster, has a puppet ref, snapshots accepted, none forged.
   Then `--load <save>` makes the client load: `loads` goes up and the puppets come back.
4. Everything: `--ini live.combat=1 --ini live.spawns=1`, `--expect actors,avatars,combat,spawns`; `combat.status` says running; `coop.spawn` answers.
5. Anything that fails: send `coop.live` from both sides and the tail of `nvcoop.log`.

## Not done / unknown (be honest)
* Nothing here ran in a game. The glue (live_glue.cpp) is compiled, not executed; the hub, avatar net and ini parsing are executed in unit tests only.
* Appearance (race / sex) of the local player is not read by the avatar module, so a puppet is made from the template (`[avatar] template=player` = the
  LOCAL player's base form). Equipment does travel.
* Spawn: cell-attach events do not call `onSiteCellLoaded` (re-enforcing the sites of a cell that just loaded happens on the lane's own 2 s sweep instead);
  `nvworld::makeResolver(hostSide=true)` is identity for created ids, so effects on a client-rolled actor fail on the host (spawn-arb.md item 4, L12's).
* `coop.combat {ref}` (death / credit / hit counters that the enemy playbooks ask for) is combat-hooks' to add; `combat.status` has the totals.
* The avatar module's pins for `GetItemCount` do not survive a hot reload (see above).
