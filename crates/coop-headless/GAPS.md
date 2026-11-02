# Gaps the headless simulator found

`coop-sim` reaches the live adapters without the viewer's Bevy glue, so it can say which hook is not
called or not wired. Pointers are to the tree this branch is cut from (line numbers drift; the names do not).
"Fixed" means fixed on this branch in `world` / `coop-session` / `coop-core` with a test; "viewer" means the
fix belongs in `viewer/` and is left for the coordinator.

## The three real failures of 2026-11-02

### live_dialogue_session (live.lines_played stayed 0)

Cause, found and fixed (two layers):

1. `Session::live_intent` dropped an intent silently while the guest had not joined yet
   (`crates/coop-session/src/live.rs`, the `else if let (Some(g), Some((_, ad)))` arm: `l.guest` is created
   only in `live_setup` once the host assigned a slot). The viewer's `--talk` fires as soon as the scene is
   loaded, and `tap_dialogue` pops a `DialogueOpen` unconditionally
   (`viewer/src/coop_live/intents.rs`, `Intent::DialogueOpen(_) => true`, then `link.0.live_intent`). The open
   went nowhere, the session never started, `Pick` waited its 5 s (`WAIT`) and was dropped, `view.lines` stayed
   empty, `lines_played` (`intents.rs`, `play_lines`) stayed 0. Reproduced headless with the stand-in talking on
   the first frame (the default; `--talk-at-ready` is the stand-in that waits). Fix: `live_intent` now keeps the
   intent (64 at most, 15 s of session time) and `live_frame` carries it once the layer can
   (`live_ready`, `flush_queued`). Test: `coop-session/tests/live_gaps.rs`
   `an_intent_asked_for_before_the_guest_joined_is_carried_once_it_has`.
2. Behind it, a real race: `GuestSync::on_msg` ignored every live message (`Avatar`, `Actor`, `Dialogue`,
   `Loot`, `Rules`, `Social`) unless the guest was `Active`
   (`crates/coop-core/src/sync/guest.rs`, the `m @ (Msg::Avatar(_) | ...)` arm). The host starts routing them
   when it accepts the join, so on a slow or lossy link the reliable ones (the cell-entry snapshot of the
   actors, container grants, dialogue results) could beat the join snapshot and were lost for good. The soak
   found it: a guest whose kill the host resolved never learned the actor (and `dead` is not in `hash.shared`,
   so nothing flagged it): 23 of 60 seeds on `bad_wifi` in a generated cell. Fix: reliable live messages that
   arrive while `Joining` are kept (512 at most) and handed to the live layer when the guest turns active.
   Test: `coop-core` `live_messages_that_beat_the_join_snapshot_are_kept_not_dropped` (fails without the
   buffer) and `coop-headless/tests/sim.rs` `no_seed_leaves_a_guest_without_the_hosts_actors` (0 of 30 seeds fail
   now).

Viewer side, still worth doing: `tap_dialogue` could hold the open until `Session::is_ready()`. Not needed for
correctness any more.

### live_loot_race (items.host + items.guest Caps001 summed to 0)

Four independent reasons, three fixed:

1. Fixed, `world`: `HostLoot::register` thinned the stacks of any container by the party's reward factor
   (`crates/world/src/coop/loot.rs`, `register`, the `scaling.keeps_loot` filter) and removed the thinned
   stacks from the world. Doc Mitchell's stock is a living person's inventory; with two players each stack has a
   1 in 6 chance of being deleted, and Doc's Caps001 stack (4 own + 50 added) was. That is the "sum is 0":
   nothing was left to take. Now only a dead body's supplies are thinned. Test:
   `live_gaps.rs` `party_scaling_thins_a_dead_bodys_supplies_but_not_a_living_persons_stock`.
2. Fixed, `coop-session` (viewer must call it): the bot's host `coop loottake` did nothing. It reaches
   `Session::live_intent(LootTake)`, whose host arm is `{}` ("the host edits the world itself",
   `live.rs`, `Intent::LootTake { .. } | Intent::LootPut { .. } => {}`), because the host's container screen
   moves the items. The bot has no screen (`viewer/src/coop_live/bot.rs`, `loottake` / `lootput`, then
   `viewer/src/coop_live/intents.rs`, `loot`). New `Session::live_loot` / `live_loot_ops`
   (`crates/coop-session/src/live_loot.rs`) register the container, move the items with `GameState::move_item`
   and carry the rest. Viewer to do: `intents::loot` should call `link.0.live_loot_ops(cx, gs)` instead of its
   own match. `coop-sim --glue-fixed` simulates that. Test: `live_gaps.rs`
   `a_host_container_take_moves_the_items_and_the_guest_can_take_the_rest`.
3. Fixed, `world`: what a guest took was credited to the ledger (`registry.gained`) and to the guest's local
   rows only. The guest's own rows are re-read from the host's authoritative copy of its shadow on every sync
   (`crates/world/src/coop/apply.rs`, `apply_changes` calls `shadow::apply_changes` first), and that copy never
   got the items, so they vanished one frame after the guest got them and `items.guest` stayed empty on both
   peers. `HostLoot::reconcile` now credits the guest's shadow (`credit_shadow` in `loot.rs`). Same test.
4. Scenario, not code: the scenario cannot pass as written. A container has one lease holder at a time
   (`world/tests/coop_live.rs`, `container_and_drop_races_resolve_once`: "the loser cannot take at all"), so a
   guest that opens in the same second as the host is refused until the host closes at +10 s, and its take
   (+7 s) returns nothing. The scenario also assumes Doc starts with no caps (he has 4, so the total is 54, not
   50) and asks for more than the container shows (`LootClient::take` refuses an over-ask locally instead of
   clipping). `crates/coop-headless/scenarios/live_loot_sequence.json` is the corrected design (host takes 20
   and closes, guest opens after and takes the 34 left); it passes with `--glue-fixed`.

### live_tether_warp (guest at 9000 units never warped back)

No hook is missing. The straggler timeout is `coop_core::rules::tether::TIMEOUT_MS` = 120 s (warning 10 s
before), counted from the moment the guest leaves the zone (`crates/coop-core/src/rules/tether.rs`). The
scenario runs 45 s + 6 s settle, so the warp can never be seen. Everything after the timeout works headless:
the warp arrives as `Notice::Warp`, `world::coop::rules::warp_player` pushes `Event::MoveTo` for the player
(`crates/world/src/coop/rules.rs`) and the viewer handles that event by moving the player
(`viewer/src/scripts.rs`, `Event::MoveTo { what, to } if what == PLAYER_REF` then `move_player`).
`crates/coop-headless/scenarios/live_tether_warp_long.json` (140 s) passes. Who starts the tether: only the
bot (`coop tether REF`, `viewer/src/coop_live/bot.rs` then `Session::live_tether`) or a script; nothing starts
one when a player is far away, by design (it is for story-critical moments). Fix options for the coordinator:
run the scenario 140 s, or give the rules host a test knob for the timeout.

## Other findings

* `live_guest_hits_essential` fails headless (and would on the viewer): Doc Mitchell is not essential in this
  data (his base NPC record has flags 0 in `ACBS`; `world::more_functions::is_essential` agrees), so the host
  ledger correctly lets 100000 damage kill him. The scenario needs an actually essential person. The ledger's
  protection itself is covered by `coop-core` tests.
* `hash.shared` does not cover `dead`, loot containers or per-player rows. Two peers can disagree on whether
  an actor is dead while the hash matches. The scenarios assert `dead` directly; a `dead` shard in the hash
  would catch this class of bug without a dedicated check.
* `movement`: the real viewer walks at the Always Run speed (77 x 4 = 308 units a second, the default in
  `viewer/src/controls.rs`), not the 150 of the fake viewer; the simulator uses 308.
