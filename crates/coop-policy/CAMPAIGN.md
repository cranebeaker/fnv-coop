# Campaign gaps that still need engine support in nv-rs

`coop-policy` answers "what should happen" for a quest event. It cannot make the engine produce
those events or honour the answers. This file lists the gaps from the old campaign backlog
(G01 to G48) that remain open once the policy table exists, and maps each to a hook point in the
nv-rs integration map (research/06-nvrs-integration-map.md; section numbers below refer to it).
Line numbers there will drift; re-grep the symbol before editing.

Gaps that the nv-rs design removes outright (no work needed): G01 and G02 (script effects are
emitted by the VM with the owning quest attached, so no scope guessing or cause resolution),
G15 and G16 (variables are written through `Host::set_var`, no polling), G47 (guest-local scripts
are the `guest_local` field of a row, one list).

How to read the tables: "Policy side" says what `coop-policy` already carries; "Engine side" is
what is still missing; "Hook" is where it plugs in.

## 1. Event sources (the policy has nothing to decide until these emit events)

| Gap | Policy side | Engine side still needed | Hook |
|---|---|---|---|
| G03 guest activation, triggers, doors, terminals as intents | `advance`, `GuestScript` | guest input becomes `Origin::Player(slot)` intents run on the host | section 6 (doors, containers), section 2.2 L2 (`run_reference_script`, `run_event`) |
| G04 kill and hit attribution for `OnHit`, `OnDeath`, `IsKiller` | `friendly_fire` data | the damage path must carry the attacker slot to the script event | `world::combat::hurt` (damage and death funnel), section 5 |
| G05 guest-initiated dialogue end to end | `advance = initiator`, `Reward` routing, payment by initiator | pick intent, host runs `begin_line` and result scripts with the initiator as "the player" | `viewer/src/dialogue.rs` `begin_line`, `world::dialogue::pick/after_line`, `PlayerShadow` swap |
| G07 MenuMode scripts for a guest (surgery, launch terminal, AutoDoc) | `shadow` rows exist | menu choice routed to the host script, spectators get a read-only view | `Runner::menu_mode`, section 6 terminals |
| G10 note pickup adds a note and `GetHasNote` is party aware | `party_cond` map | pickup records AddNote, condition reducers evaluated over the party | `GameState::pick_up`, `Facts` |
| G39 presentation of host-run scripts (fades, sound, flashes) | none | forward `Event::*` presentation messages (section 1.3 last paragraph) to guests | `GameState.events` drain in `viewer/src/scripts.rs` |

## 2. Gates the engine must feed and obey

| Gap | Policy side | Engine side still needed | Hook |
|---|---|---|---|
| G23 party-aware conditions (GetDistance min, GetInCell all/any, GetItemCount any, IsKiller any) | `party_cond` function to reducer map per row | `Facts` must accept a reducer over player shadows instead of one `PLAYER_REF` | `Facts` (scripting.rs ~3032), `PlayerShadow` |
| G25 tether anchors and worldspaces, zone membership | `Tether` cells, anchor, radius; `State.in_zone` | compute `in_zone` per quest from each player's cell, worldspace and position; the single-cell sim scope is the real blocker (section 4.2 multi-cell caveat) | `GameState.player_cell/player_world/player_position`, `viewer/src/scripts.rs` ~1150 |
| G26 barrier pass detection (a player "passes" a one-way stage) | `Barrier`, `State.barrier_passed` | detect the per-player step that counts as passing (chargen done, gate crossed, stage reached on that player) and report it | `Runner::set_stage` (scripting.rs ~3849) plus per-quest pass trigger |
| G27 ready check requests from INFO end scripts | `Ready.infos`, `ReadyRequest` event | raise the request when the listed INFO ends, collect yes and no, expose elapsed time | `dialogue.rs` end-script site (~495) |
| G08 consent for irreversible forks | `Consent` rows (3 quests), `Choice` event, vote tally | message-box and dialogue-topic hooks that raise `Choice` and hold the script until the decision; rows only cover the DLC forks found so far, endgame boxes and other forks are not tagged | message box event, `Host::call` |
| G28 mutually exclusive outcomes | `Exclusive`, `groups`, `State.committed_*` | track which stage or rival quest committed first and keep it in the party save | `Runner::set_stage`, save |
| G29 party warp and cross-cell moves | `Warp` markers, slots, `absent` | place each player at a slot offset around the marker and park absent players; needs multi-cell load for players in other cells | `MoveToMarker` (scripting.rs ~4438), `map::travel` |
| G30 ending slideshow for all, released together | `host_decision = ending`, `Scene` event, `barrier.before = slideshow` | slide channel to guests, ordered playback, late-joiner replay | `viewer` movie and slideshow systems |
| G18 control locks with scope and per-source release | `controls` (scope, masks, timeout) | lock leases by source; today `controls_off[7]` is one set for one player | `GameState.controls_off`, `player_looking_blocked()` |
| G19 fast-travel lock and save suppression | `fast_travel_lock`, `save_suppress`, `FastTravel` and `Save` events | ask the policy before `map::travel` and before any save | `map::travel` (map.rs ~271), `scripts::save_and_load` |
| G38 time: host clock, wait and sleep by consent, once-per-event time skips | `host_decision = time`, `once`, `Scene::TimeSkip` | host owns `advance_clock`, `wait`; guests cannot call them | `advance_clock` (scripting.rs ~1064), `wait` (~1154), `Time<Virtual>` writers |
| G41 downed, respawn at zone checkpoint, late join into scenes | `policy.death`, `policy.late_join`, `shadow.down/checkpoint`, `PlayerDown` | downed state, checkpoint warp, joiner excluded from `present` until the scene ends | `combat::hurt` death branch, join handler in `coop-core` |

## 3. Per-player state (the shadow work)

The policy says which values are private (`shadow_classes`, row `shadow`, `scope = per_player`).
The engine still keeps everything under `PLAYER_REF`.

| Gap | Policy side | Engine side still needed | Hook |
|---|---|---|---|
| G24 per-player script variables and menu state | `ShadowWrite`, `PerPlayerShadow` decision | a `PlayerShadow` swap in and out of `GameState` around host script runs: `actor_values`, `items`, `perks`, `skill_points` keyed by `PLAYER_REF` | `Runner::with`, `take_locals/put_locals` (scripting.rs ~3056) |
| G42 Dead Money collars, cloud, per-player down state | `NVDLC01BombCollarQuest` row (18 vars, checkpoints, `down`) and 4 related rows | a `KillActor` on the player ref must become a per-slot health drain, collar timers must live in the shadow | `ResurrectActor` / kill funnel (scripting.rs ~4305), `combat::hurt` |
| G43 Old World Blues organs, implants, AutoDoc | `NVDLC03GenericPerkQUEST`, `NVDLC03AutoDoc` rows | organ and implant perks applied to one slot only | `perks::add` (perks.rs ~582) with a recipient |
| G14 perk recipient by kind | `perks` lists (all, initiator, recruiter) and `Reward::Perk` decision | `perks::add` takes a slot | `perks::add` |
| G33 per-owner follower variables | `Companion` rows, `PerPlayerShadow Followers`, follower caps in `State.followers` | the vanilla follower quest variables (VNPCFollowers and friends) become per-owner and are swapped per run; "waiting companion fires only when no owner's party is in the cell" | `companions.rs` (state at ~61, spawn ~471), `GameState.variables` |
| G34 companion set pieces (plot companions, single-slot DLC companions) | `owner = party` rows (6) | commands from any player move the shared companion; Dead Money single follower slot per player | `companions.rs` |
| G35 shared storage becomes per-player (gear confiscation, entry strip, return triggers) | none: no row tags storage containers yet | container redirect keyed by slot, and rows that name those containers | `viewer/src/game_menus/container.rs` (~222), `GameState::move_item` |
| G37 XP tiers follow the host level, scaling uses the strongest member | `party_cond` reducer `host` | level reads in OWB and Dead Money scripts go through a reducer | `experience::reward`, `Facts` |

## 4. Rows and rules the data does not carry yet

| Gap | State |
|---|---|
| Unreviewed rows | 115 of 487 rows are `default` (derived from risk categories, never read by a person). `PolicyTable::unreviewed()` lists them. The T4 ones among them (VMQ04 and similar Hoover Dam transitions) matter most. |
| Tethers with no resolvable cell | 37 tethers were dropped in the old table for lack of a cell, and 8 party warps for lack of a marker (old `rules.pending.json`). They are not in `policy.json`; the affected quests (several Honest Hearts and Lonesome Road main quests, the Dead Money gala, timers) hold no tether. Needs cell and marker ids resolved from the plugin data. |
| Pending parameters | 371 parameters and several rule kinds the old engine never implemented (map-marker sharing, local quest scripts for OWB, companion cap parameters) were not imported. |
| G12, G13, G11, G09, G17, G20, G21, G22 | State capture items (factions, relations, journal, markers, world objects, NPC orders, form lists). Not policy; they belong to `coop-core` effect capture and are listed here only so nothing is dropped: hook points `faction_changes`, `faction_relations`, `AddTopic/AddNote`, `Enable/Disable`, `GameState::placed`. |
| G40 friendly-fire guard | Data is in 5 rows (`friendly_fire`); there is no decision for it yet because it needs hit events with attacker slots (G04). |
| G44, G45 Lonesome Road and Honest Hearts set pieces | Pooled items are tagged (`pooled_items`, 10 rows) but the counting itself (totems across players, weight gate as max over the party) is engine work. |
| G46 feature switches | Not applicable: nv-rs has one switch, `Authority`, and the policy is consulted only when it is not `Single`. |

## 5. Integration order (suggested)

1. `Authority` on `GameState` and the three guest-skip levels (section 2.2), so guests stop running quest scripts.
2. Emit `Event` values from the cause layer funnels (section 1.3): `Runner::set_stage`, rewards (`experience::reward`, `reputation::reward_karma`, `GameState::added`), `perks::add`.
3. Feed `State`: present set, initiator (from the dialogue session), zone membership, barrier passes.
4. `PlayerShadow` swap, then Dead Money collars and AutoDoc as the first proof rows.
5. Endings: `Scene` events, slideshow channel, group commitment saved in the party save.

Everything above is proven only at the policy level (unit and property tests in this crate).
Nothing here has run inside nv-rs.
