# Co-op scenes: players in different cells

Status: decided and implemented in `coop-core::scene` (host model, wire, queue),
`coop-session::live_roam` (view, dump), `world::coop::roam_ops` (script
commands), `viewer/src/coop_live/scene.rs` (glue, not compiled by the author).

## The problem

The engine loads one scene around one player: an interior cell, or a square of
exterior grid cells. Free roam (the owner's decision: players may be anywhere,
with a story tether) means two players can be in cells that the other machine
never loads. Party warp, the tether, late join, quest scripts that touch
references in unloaded cells and actor ownership all depend on how that is
resolved.

## Decision

**Every peer simulates the scene around its own player.** Nothing else is
simulated anywhere.

1. **Loaded set per peer.** A peer's loaded set is its home cell plus, in an
   exterior, the grid square around it (radius 2, the game's `uGridsToLoad` 5),
   plus explicitly linked cells. Both sides derive it from one table
   (`CellGraph`), so nothing is sent for it. Load and unload are independent per
   peer.
2. **Actors.** An actor in a peer's loaded cells is owned by the nearest player
   standing in the actor's cell. If nobody stands there, the nearest player who
   has the cell loaded around theirs owns it (a same-cell player always beats a
   loaded-only one). The owner publishes it (epoch fenced, as before) to every
   peer that has the cell loaded (interest = loaded set). If nobody has the cell
   loaded the actor is dormant: the host ledger holds its last state and the
   time it went dormant, nothing is simulated, and the next peer to load the
   cell becomes owner with a newer epoch.
3. **Entering a cell.** A peer that gains a cell (moving home, or a grid cell
   scrolling into its square) receives the snapshot of every cell it gained:
   current owners with their last state, dynamic refs, and who is dead. It
   adopts the existing owner; it does not take over unless it is nearer by the
   hysteresis margin.
4. **Other players.** A player in a cell I have loaded is a normal puppet
   (avatar stream relayed by cell interest). A player in a cell I have not
   loaded exists for me only as data: name, cell, position (`Presence`, every
   500 ms, only to peers that do not share a scene with them). The viewer draws
   a puppet only for players in its loaded cells.
5. **Story runs only on the host.** Quest, stage and reference scripts run on
   the host and nowhere else. Shared persistent state is global and keyed by
   FormID (enabled or disabled, destroyed, moved, inventories, actor values,
   locks, quest stages): it applies to a cell whenever any peer loads it, with no
   queue. Commands that need a live entity wait in a host queue (below).
6. **Tether and barrier across cells.** The tether zone is a disc in a cell. A
   player is inside if in that cell, or in a loaded grid cell of the same
   worldspace and within the radius. A player in another cell or another scene
   is a straggler: the stage is held, a warning goes out, and after 120 s the
   player is warped to the anchor. The unanimous barrier requires every player
   present when it opens wherever they stand, then warps the party together
   (spread on a ring so nobody stacks). Fast travel by a free-roaming player
   moves that player (plus followers); under a tether it needs a party vote.
7. **Late join.** The host places the joiner at the tether anchor if a tether
   is active, else beside the host.
8. **Free roam means no automatic following.** The guest is placed once, at the
   start. After that the viewer's `follow_place` no longer yanks the guest into
   the host's cell.

## Script commands on references in unloaded cells

`world::coop::roam_ops::classify` is the table. Unknown commands are state
commands.

* **State commands** run at once on `GameState` (keyed by FormID, including
  `GameState::place` for refs the host never loaded) and replicate as before:
  `Enable`, `Disable`, `MoveTo`, `MoveToMarker`, `SetPos`, `SetAngle`,
  `AddItem`, `RemoveItem`, `RemoveAllItems`, `SetStage`, `StartQuest`,
  `SetObjective*`, `SetAV`, `ModAV`, `ForceAV`, `Kill`, `KillActor`, `Lock`,
  `Unlock`, `SetOwnership`, `SetEssential`, `SetFactionRank`, `AddToFaction`,
  `SetOpenState`, `SetDestroyed`, `SetPlayerTeammate`, `SetCurrentHealth`.
* **Needs a loaded entity** (queued while the reference's cell is not loaded by
  the host, with the cell and the arguments): `Activate`, `PlayGroup`,
  `PlayIdle`, `PlayIdleEx`, `StopCombat`, `StopCombatAlarm`, `StartCombat`,
  `StartConversation`, `Say`, `SayTo`, `Look`, `StopLook`, `EvaluatePackage`,
  `SetRestrained`, `SetAlert`, `SetUnconscious`, `PushActorAway`, `KnockDown`,
  `ForceFlee`, `ForceTakeCover`, `AddScriptPackage`, `RemoveScriptPackage`,
  `DropMe`, `Dispel`, `CastSpell`, `PlaceAtMe`, `CreateFullActorCopy`,
  `PlaySound`, `PlaySound3D`, `StopSound`, `EquipItem`, `UnequipItem`,
  `ModifyFaceGen`, `PlayMagicEffectVisuals`, `PlayMagicShaderVisuals`,
  `SetCombatStyle`, `RewardKarma`.

How the queue works: the hook in `Host::call` (`roam_ops::defer`) checks the
host authority, the host's loaded cells (reported by the session each frame),
the command class and the reference's cell. A queued command reports success
(value 0) to the script. The session moves queued commands into
`DeferredQueue` (per cell, oldest first, 128 per cell and 1024 in all,
set-style commands such as `Look` and `EvaluatePackage` keep only the newest per
reference). When the host loads the cell the commands are replayed in order
through `Runner::apply_call`, which bypasses the hook. Forwarding to a guest that
has the cell loaded is implemented (`RoamMsg::RefOp`, ack `RefOpDone`) but off
by default (`Session::live_roam_forward_ops`) until the viewer applies
`RoamView::ref_ops` to its loaded entities.

## API for other modules (doors, fast travel, interactions)

Do not edit the model; call it. On the host, `HostLive::scene` holds a
`HostRoam` with `model: SceneModel`:

* `model.plan_transit(mover, Placement) -> Transit { moves, needs_vote }`: what
  a door or fast travel does to the party (followers within 600 units come
  along; a tether makes a destination outside the zone a vote).
* `model.set_follow(player, Some(leader))`: opt-in following through doors.
* `model.party_warp(members, Placement)`, `model.join_placement(joiner)`.
* `HostRoam::place_msgs(&moves)`: the `Place` messages that move them; the
  viewer applies `LiveView.scene.place` (see `coop_live/scene.rs`).
* `model.in_zone(player, cell, pos, radius)`, `model.loaded_of(player)`,
  `model.peers_loading(cell)`, `model.share_scene(a, b)`.
* Register cells: `Session::live_roam_graph()` then
  `world::coop::roam_ops::register_world_grid` / `register_link`.

## Counters (state dump, `live.roam`)

`cells.<peer>` (cell id of every known peer), `owned.<peer>` (actors owned),
`dormant`, `remote_actors` (mirrored actors in the loaded cells), `loaded`
(cells loaded by this machine, host), `deferred_pending`, `deferred_queued`,
`deferred_replayed`, `forwarded`, `confirmed`, `placed`.

## Tests

* coop-core: `roam::*` unit tests; `sim_test::scene` (chaos profiles): owners
  per cell, nothing leaks across cells, adoption on entering, dead actors on
  entering, exterior neighbours, dormancy and re-adoption with a newer epoch,
  late-join placement, tether anchor placement.
* coop-headless `tests/scene.rs`: the five scenarios `live_two_cells_actor_owner`,
  `live_cell_enter_adopts_state`, `live_deferred_script_unloaded_ref`,
  `live_tether_across_cells`, `live_late_join_other_cell`, each under four
  profiles, with a mutation twin each (`twin_*`, expect_fail). They run on the
  generated test data and use the bot verb `coop cell CELL` and the scenario key
  `host_cell`.
* world: `coop::roam_ops` unit tests.

## Not covered

* The viewer does not yet apply `ref_ops` to entities (forwarding is off).
* Interior door links are registered by hand; nothing reads door destinations
  automatically.
* Dormant actors are not simulated while dormant (no catch-up beyond keeping the
  ledger state and the dormant-since time).
