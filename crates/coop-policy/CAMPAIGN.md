# Campaign gaps in nv-rs

## Status 2026-11-02: closed in code (see `crates/world/src/coop/campaign/`)

Everything below is implemented in the new `campaign` module of `world` (one field,
`GameState::coop.campaign`, plus one-line hooks), with unit tests, world tests
(`crates/world/tests/coop_campaign.rs`, 29 tests), eight headless scenarios with mutation twins
(`tools/coop-run/scenarios/live_*` and `twin_live_*`: kill_xp_attribution, companion_owner_only,
personal_storage, collar_per_player, party_condition, downed_respawn, fork_consent, friendly_fire_guard)
and a coop-sim soak (8 scenarios x 7 chaos profiles x 3 seeds: 168 of 168 pass).
The old tables further down are kept as the record of what each gap was.

| Gap | Closed by |
|---|---|
| G04 kill and hit attribution, guest kills give XP, karma, kill stats | `campaign::attribution` (host adapter feeds `Damaged`/`Died` by slot; XP through the gate ledger per contributor; `experience::on_kill` stays out when attribution is on) |
| G03, G05 initiator-picks: host scripts run for a player | `dialogue.rs` runs begin and end scripts inside `campaign::producers::as_initiator` (shadow scope, `begin_interaction`, `PolicyGate::acting_origin` gives `Origin::Player(slot)` for companion and shadow-write events; rewards stay host origin) |
| G33, G34 per-owner followers, owner-only commands, critter vs humanoid caps | `campaign::owners` (owner from shadow teammates, `command_ok`, `recruit`, `dismiss`, `sync_critters`), follower quest variables swapped per player in `campaign::extra` |
| G35 personal storage | row field `storage` (FortEquipmentConfiscationQuest lists FortWeatherStationStorageRef), container rows swapped per player in `extra`, loot ledger redirect `storage::redirect_op` in `Session::live_loot` |
| G42, G43 collars, per-player script object variables, hardcore needs, `.fos` stacks | `campaign::extra` (objvars from rows' `shadow.objvars`, owners learned by a hook in `run_reference_script`; `Living` and `inventory_stacks` swapped) |
| Perk reward strings (`plugin:id`, editor id, hex) | `coop_policy::same_form`; the gate sends `EDID|hex` aliases |
| G23, G37 party aware conditions | `coop_policy::Reducer` + `campaign::party` (hooked in `Facts::value` while a quest context is set in `Runner::with`) |
| G25, G26, G27 tether zones, barrier passes, ready checks, votes, interactions | `campaign::producers` and `PolicyGate::{vote,begin_vote,begin_ready,ready_request}` |
| G08 fork consent | `campaign::consent` (`check`, `vote`, `poll`, one-way once decided) |
| G41 downed, respawn at checkpoint, late join | `coop_policy::lifecycle` + `campaign::downed` (held joiners via `PolicyGate::hold_joiner`) |
| G40 friendly fire guard | `EventKind::Hit`, `Reject::FriendlyFire`, `campaign::ffguard` hooked in `hurt_hooked` |
| hash.shared dead shard | `export::tag::DEAD`: dead actors are in the shared export; a lost kill is repaired once |
| Guest script variable names | `campaign::names::heal` when game data arrives |
| 37 dropped tethers, 8 dropped warps | `tools/policy-import/resolve_tethers.py` (31 resolved; the rest have no place by construction: pure scripted scenes, see below) |
| 115 `default` rows | `tools/policy-import/review_defaults.py`: 19 left, none can be advanced by anyone at tier T3/T4 |
| DLC rows keyed by the plugin's own index byte | `PolicyGate::set_remap` from `configure_campaign` (gate and shadow plan use load-order ids) |

Host adapters call `campaign::host_frame` each frame; guests call `configure_campaign`.

Remaining by construction, not engine work: 7 tether quests with no objective target and no `MoveTo`
(BeatrixFX, FistoFX, SantiagoFX, VDialogueCaesar, VGenericTimer, VMQCaesarFail host-run scenes;
VMS49 spans 16 cells), NVDLC02MQ07 (no interior cell, no editor id), and 5 warps whose quests have no
`MoveTo` or marker (NVDLC02MQ04, NVDLC02MQ06, NVDLC03MQ03, NVDLC04MQ04, VCG03). The 19 default rows are
4 synthetic rows, 8 "Systems & misc" T3 quests and 6 side quests the old table never gave an
initiator rule, plus NVDLC02MQ07.
Viewer wiring still to do by the viewer owner (not touched here): call `Session::live_loot_ops`, raise
`consent::check` at the fork message boxes, `downed::revive` from the revive UI.

# Original gap list (history)

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

## Row keys (data schema 2)

A row is keyed by `(plugin, local)`: the plugin file name and the quest's id inside that plugin
(`local`, top byte clear), plus the editor id. `id` is only what the quest has when its plugin
loads straight after the base game; it is not the runtime id. A session remaps the table to the
real load order at start (`PolicyTable::remap`, `coop_session::policy_ids`): by `(plugin, local)`,
then by editor id; rows whose quest is not loaded are dropped and reported
(`RemapReport::describe`, logged as the `policy_ids` event), never an error. A different DLC
order, a missing DLC or a store with another order all resolve the same way.

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

## Scenes, movies, control locks and time (G18, G30, G38, G41): wired

`coop-core::scene` (`SceneHost`, `SceneClient`, `SceneMsg`) is the replicated scene state; `world::coop::scene`
(`SceneBridge`, `SceneFacts`) is the engine side. The host's story opens scenes from three sites: a script's
`PlayBink`, a script's `DisablePlayerControls` / `EnablePlayerControls` (lock set, scope from `controls.scope`,
safety limit from `controls.timeout_s`), and a stage of a quest whose row says `host_decision` `cinematic` or
`ending` (`barrier.before` gives the Together rule; masks give the lock set). Every peer, the host included, plays
the same movie at the same scene time (start-at lead minus half the round trip; a late joiner or a merge starts
`elapsed_ms` into it), gets its own lock set (the initiator only, or everyone), and ends with the host or by a
unanimous skip vote. `late_join.during_scene = wait` holds a joiner (all controls locked) until the scene ends and
replays it the remaining state; a rejoin is replayed too; the guest drops every scene when the host link is gone.
Waiting and sleeping are party votes (`VoteKind::Sleep` / `Wait`, everyone present must agree; answered with
`Intent::Answer`, F8 / F9 in the viewer); the host runs the approved hours once and announces one `TimeSkip` event
(applied once per event id). The guest's clock follows the host's smoothly (`follow_clock`), and the shared hash
reads the value the host replicated. Ending commitments (`SceneHost::committed`) travel in the join sync and are
ready for the party save (`restore_committed`). Tests: `coop-core` scene unit tests, `world/tests/coop_scene.rs`,
`coop-headless/tests/scene.rs` (scenarios `live_scene_barrier`, `live_scene_late_join`, `live_sleep_vote`,
`live_sleep_guest_asks`, `live_sleep_refused`, `live_movie_sync`, `live_control_lock_per_player` with
`twin_*` mutation twins, and a soak over every chaos profile).
## 6. Player-world interactions (the `coop-core::interact` layer)

How the rows of sections 2 to 4 that concern what a player does in the world are carried now. A guest's
action is an `Act` the host applies once (exactly-once by `(guest, txn)`), in that player's shadow when
it is personal; shared state travels in the existing domains or as `Shared` records.

| Gap | How it is carried |
|---|---|
| G19 fast travel | consent stays the party vote (`VoteKind::FastTravel`); `Act::Discover` shares a found map marker (the `map_markers` domain), so the unlock is the party's |
| G33 per-owner followers | `Act::Companion` runs in the commander's shadow: `Recruit` refuses a companion another player already has (`Why::Busy`), commands need the commander's own teammate |
| G34 plot companions | a companion whose policy row says `owner = party` takes commands from any player and cannot be recruited or dismissed by one (`Why::Policy`) |
| G35 shared storage | `Act::Steal` (loot, pickpocket, plant) moves items between a body or pack and the acting player's shadow; a storage redirect keyed by slot is still a policy-row matter (no row tags storage) |
| G09, G17, G20 world objects | doors, locks, terminals and mines no domain carries are `Shared` records: published when an act changes them and by a 200 ms sweep of the host world (scripts, quests, the host player), kept in a log for late joiners; picked-up items, destroyed objects, notes and map markers ride the `disabled`, `destroyed`, `note` and `map_marker` domains |
| load doors, one-way trips | `Transit`: the leader (host) goes anywhere and the members of the cell group follow, anyone else only into the leader's cell group, one-way trips open a party barrier (`rules` barrier, tag `0x5452xxxx`) |
| barter, vendor stock | `Act::Barter` runs `barter::accept` in the buyer's shadow; the vendor's stock and caps are the shared `items` rows, so one stock serves the party |
| crafting, repair, item use, equipment, ammo, hot keys | `Act::Craft`, `Repair`, `Use`, `Equip`, `Ammo`, `Pip(Hotkey)` run in the acting player's shadow: ingredients, skill, health, rads and addiction are theirs |
| minigames | lockpicking and hacking are played on the acting machine; the outcome is `Act::Lockpick` / `Act::Hack` (pins spent and experience go to that player, the lock or terminal state is shared) |

Not carried by this layer (still policy or engine work): storage redirects per slot (G35 rows), the
explosion damage of a mine a guest set off (the guest's own viewer resolves its blast; the host only
retires the mine), and any scripted `OnActivate` whose script needs the activator's *skills* read from a
player other than the one in the shadow (the shadow is swapped in for the run, so the common case works).
