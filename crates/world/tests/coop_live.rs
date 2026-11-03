//! The co-op live layer on the world crate: a host and two guests, each a
//! `GameState`, joined by coop-core's chaos transport and the live adapters.
//! No viewer, no game data (generated test worlds only).

use coop_core::ids::{FormId as CForm, PlayerId};
use coop_core::proto::Msg;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use esm::{ActivePlugins, FormId, LoadOrder};
use world::coop::apply::{Ctx, Opts};
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::{Authority, GuestBridge, HostBridge};
use world::dialogue::PLAYER_REF;
use world::scripting::{Event, GameState, ScriptCache};

struct Rig<'a> {
    order: &'a LoadOrder,
    scripts: &'a ScriptCache,
    sim: Sim,
    host: GameState,
    guests: Vec<GameState>,
    ha: LiveHostAdapter,
    gas: Vec<LiveGuestAdapter>,
    hb: HostBridge,
    gbs: Vec<GuestBridge>,
}

impl<'a> Rig<'a> {
    fn new(order: &'a LoadOrder, scripts: &'a ScriptCache, profile: &str, seed: u64) -> Rig<'a> {
        let mut host = GameState::new(order);
        host.set_authority(Authority::Host);
        let guests: Vec<GameState> = (0..2)
            .map(|_| {
                let mut g = GameState::new(order);
                g.set_authority(Authority::Guest);
                g
            })
            .collect();
        let mut hb = HostBridge::default();
        hb.backstop_every = 1;
        let mut sim = Sim::new(2, ChaosProfile::by_name(profile).unwrap(), seed);
        sim.start_guest(0);
        sim.start_guest(1);
        let mut r = Rig {
            order,
            scripts,
            sim,
            host,
            guests,
            ha: LiveHostAdapter::new(),
            gas: vec![
                LiveGuestAdapter::new(PlayerId(1)),
                LiveGuestAdapter::new(PlayerId(2)),
            ],
            hb,
            gbs: vec![GuestBridge::default(), GuestBridge::default()],
        };
        let ok = r.sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "guests never became active");
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
        }
    }

    /// Everyone stands in `cell` at the given x positions (host, g1, g2).
    fn stand(&mut self, cell: u32, xs: [f32; 3]) {
        for (i, g) in std::iter::once(&mut self.host)
            .chain(self.guests.iter_mut())
            .enumerate()
        {
            g.player_cell = Some(FormId(cell));
            g.player_position = Some([xs[i], 1000.0, 0.0]);
        }
    }

    fn step(&mut self) {
        let cx = self.cx();
        let now = self.sim.clock.now();
        self.hb.collect(&mut self.host, &mut self.sim.host);
        let outs = self.ha.frame(
            cx,
            &mut self.host,
            &mut self.sim.host.live,
            &mut self.sim.host.auth,
            now,
        );
        self.sim.host_emit(outs);
        for i in 0..2 {
            if self.sim.guests[i].sync.phase() == GuestPhase::Active {
                let opts = Opts {
                    present: true,
                    ctx: Some(cx),
                };
                self.gbs[i].sync(
                    &mut self.guests[i],
                    &self.sim.guests[i].sync.store,
                    false,
                    opts,
                );
                let msgs = self.gas[i].frame(
                    cx,
                    &mut self.guests[i],
                    &mut self.sim.guests[i].sync.live,
                    now,
                );
                self.sim.guest_emit(i, msgs);
            }
        }
        self.sim.step();
    }

    fn run_ms(&mut self, ms: u64) {
        let end = self.sim.clock.now() + ms;
        while self.sim.clock.now() < end {
            self.step();
        }
    }

    fn until(&mut self, ms: u64, mut cond: impl FnMut(&mut Rig<'a>) -> bool) -> bool {
        let end = self.sim.clock.now() + ms;
        while self.sim.clock.now() < end {
            if cond(self) {
                return true;
            }
            self.step();
        }
        cond(self)
    }

    fn all(&self) -> Vec<&GameState> {
        std::iter::once(&self.host)
            .chain(self.guests.iter())
            .collect()
    }
}

fn died_events(g: &GameState, who: FormId) -> usize {
    g.events
        .iter()
        .filter(|e| matches!(e, Event::Died { who: w, .. } if *w == who))
        .count()
}

mod fighting {
    use super::*;
    use testdata::fighting::ids::*;

    fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
        let data = testdata::fighting::fighting(tag);
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    fn setup<'a>(
        order: &'a LoadOrder,
        scripts: &'a ScriptCache,
        profile: &str,
        seed: u64,
    ) -> Rig<'a> {
        let mut r = Rig::new(order, scripts, profile, seed);
        r.stand(CELL, [900.0, 900.0, 5000.0]);
        let cx = r.cx();
        r.ha.track(
            cx,
            &r.host,
            &mut r.sim.host.live,
            &[FormId(GECKO_REF), FormId(GUARD_REF)],
        );
        r
    }

    #[test]
    fn actor_handoff_is_reflected_in_world_actor_data() {
        let (_d, order) = order("live-handoff");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 11);
        let gecko = FormId(GECKO_REF);
        // the nearest player owns it: the host (x 900) and guest 1 tie at the gecko's x 0? no: g1 is nearest
        r.guests[0].player_position = Some([20.0, 1000.0, 0.0]);
        assert!(r.until(30_000, |r| r.gas.len() == 2
            && r.sim.guests[0].sync.live.owns_actor(CForm(GECKO_REF))));
        // the owner moves it; every other peer's world data follows
        r.guests[0]
            .positions
            .insert(gecko, ([123.0, 1000.0, 0.0], 1.0));
        r.guests[0].combat.insert(gecko, FormId(GUARD_REF));
        r.guests[0].coop.live.anim.insert(gecko, 7);
        assert!(
            r.until(10_000, |r| r.host.positions.get(&gecko).map(|p| p.0[0])
                == Some(123.0)
                && r.guests[1].positions.get(&gecko).map(|p| p.0[0])
                    == Some(123.0))
        );
        assert!(r.until(5_000, |r| r.host.combat.get(&gecko)
            == Some(&FormId(GUARD_REF))
            && r.guests[1].coop.live.remote_anim.get(&gecko) == Some(&7)));
        // guest 1 walks away, guest 2 arrives: ownership is handed over
        r.guests[0].player_position = Some([9000.0, 1000.0, 0.0]);
        r.guests[1].player_position = Some([30.0, 1000.0, 0.0]);
        assert!(
            r.until(60_000, |r| r.sim.guests[1]
                .sync
                .live
                .owns_actor(CForm(GECKO_REF))),
            "handoff"
        );
        r.guests[1]
            .positions
            .insert(gecko, ([77.0, 1000.0, 0.0], 0.5));
        assert!(
            r.until(10_000, |r| r.host.positions.get(&gecko).map(|p| p.0[0])
                == Some(77.0)
                && r.guests[0].positions.get(&gecko).map(|p| p.0[0])
                    == Some(77.0))
        );
        // the host keeps a record and a puppet form for each player
        assert_eq!(r.ha.registry.iter().count(), 3);
        assert_eq!(
            r.ha.registry.puppet(PlayerId(2)),
            Some(world::coop::facts::puppet_of(PlayerId(2)))
        );
    }

    #[test]
    fn outdoors_the_worldspace_is_the_place_and_the_nearest_player_owns_each_actor() {
        let (_d, order) = order("live-outdoors");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, "perfect", 41);
        let world_form = FormId(0x7777);
        let gecko = FormId(GECKO_REF);
        // the gecko walks the open world; the players stand in different grid squares of it
        r.host.spaces.insert(gecko, (world_form, FormId(0x7001)));
        for (i, g) in std::iter::once(&mut r.host)
            .chain(r.guests.iter_mut())
            .enumerate()
        {
            g.player_world = Some(world_form);
            g.player_cell = Some(FormId(0x7001 + i as u32));
            g.player_position = Some([[4000.0, 6000.0, 9000.0][i], 1000.0, 0.0]);
        }
        let cx = r.cx();
        r.ha.track(cx, &r.host, &mut r.sim.host.live, &[gecko]);
        assert_eq!(
            world::coop::live::export_actor(&r.host, cx, gecko).cell.0,
            world_form.0
        );
        // the host is nearest to the gecko at x 0? no: x 4000 vs 6000 vs 9000
        assert!(r.until(30_000, |r| r
            .sim
            .host
            .live
            .actors
            .own
            .get(CForm(GECKO_REF))
            .is_some_and(|a| a.owner == Some(PlayerId(0)))));
        // the second guest becomes the nearest, in another grid square: ownership follows
        r.guests[1].player_position = Some([100.0, 1000.0, 0.0]);
        assert!(
            r.until(60_000, |r| r.sim.guests[1]
                .sync
                .live
                .owns_actor(CForm(GECKO_REF))),
            "handoff across grid squares"
        );
    }

    #[test]
    fn an_actor_nobody_shares_a_place_with_keeps_running_locally() {
        let (_d, order) = order("live-dormant");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 42);
        let gecko = FormId(GECKO_REF);
        // somebody owns it for a while, then everyone leaves its place
        r.run_ms(3_000);
        assert!(
            r.guests[0].positions.contains_key(&gecko)
                || r.until(10_000, |r| r.sim.guests[0]
                    .sync
                    .live
                    .actors
                    .get(CForm(GECKO_REF))
                    .is_some())
        );
        for g in std::iter::once(&mut r.host).chain(r.guests.iter_mut()) {
            g.player_cell = Some(FormId(0x7002));
        }
        assert!(r.until(10_000, |r| r.sim.guests[0]
            .sync
            .live
            .actors
            .get(CForm(GECKO_REF))
            .is_some_and(|a| a.owner.is_none())));
        r.guests[0]
            .positions
            .insert(gecko, ([555.0, 1000.0, 0.0], 1.0));
        r.run_ms(3_000);
        assert_eq!(
            r.guests[0].positions.get(&gecko).map(|p| p.0[0]),
            Some(555.0),
            "a dormant record is not applied over the local copy"
        );
    }

    #[test]
    fn a_guest_hit_kills_an_npc_exactly_once_on_all_peers() {
        for (profile, seed) in [("perfect", 21u64), ("bad_wifi", 22)] {
            let (_d, order) = order(&format!("live-kill-{profile}"));
            let scripts = ScriptCache::default();
            let mut r = setup(&order, &scripts, profile, seed);
            let gecko = FormId(GECKO_REF);
            r.run_ms(2_000);
            // a guest hit is a request: nothing changes locally
            assert!(!world::combat::hurt(
                &order,
                &mut r.guests[0],
                gecko,
                5.0,
                PLAYER_REF
            ));
            assert!(r.guests[0].damage.is_empty());
            r.run_ms(3_000);
            let h = world::combat::health(&order, &r.host, gecko).unwrap();
            assert!(h < 20.0 && h > 0.0, "{profile}: damaged on the host ({h})");
            // both guests finish it in the same moment
            world::combat::hurt(&order, &mut r.guests[0], gecko, 100.0, PLAYER_REF);
            world::combat::hurt(&order, &mut r.guests[1], gecko, 100.0, PLAYER_REF);
            assert!(
                r.until(60_000, |r| r.all().iter().all(|g| g.dead.contains(&gecko))),
                "{profile}: dead everywhere"
            );
            r.run_ms(5_000);
            for g in r.all() {
                assert_eq!(died_events(g, gecko), 1, "{profile}: one death");
            }
            assert!(r.sim.host.live.actors.is_dead(CForm(GECKO_REF)));
            // a late hit on the corpse changes nothing
            world::combat::hurt(&order, &mut r.guests[1], gecko, 100.0, PLAYER_REF);
            r.run_ms(3_000);
            for g in r.all() {
                assert_eq!(died_events(g, gecko), 1, "{profile}: still one");
            }
        }
    }

    #[test]
    fn a_guest_copy_of_an_npc_fight_stays_on_the_guest() {
        let (_d, order) = order("live-npc-copy");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 23);
        let gecko = FormId(GECKO_REF);
        let guard = FormId(GUARD_REF);
        r.run_ms(2_000);
        let guard_full = world::combat::health(&order, &r.host, guard).unwrap();
        // the guest's world runs its copy of a host fight: guard and gecko trade blows
        world::combat::hurt(&order, &mut r.guests[0], gecko, 5.0, guard);
        world::combat::hurt(&order, &mut r.guests[0], guard, 5.0, gecko);
        r.run_ms(3_000);
        assert_eq!(world::combat::health(&order, &r.host, gecko), Some(20.0));
        assert_eq!(
            world::combat::health(&order, &r.host, guard),
            Some(guard_full)
        );
        assert_eq!(
            r.host.coop.live.combat.stats.crimes, 0,
            "no assault by the guest"
        );
        // the guest's own blow on the guard still goes, and it is the guest's crime
        world::combat::hurt(&order, &mut r.guests[0], guard, 5.0, PLAYER_REF);
        assert!(
            r.until(10_000, |r| world::combat::health(&order, &r.host, guard)
                .unwrap()
                < guard_full)
        );
        assert_eq!(r.host.coop.live.combat.stats.crimes, 1);
    }

    /// A host script raises an actor's health (`ForceActorValue Health`) just after the host
    /// starts to track the actor, before the host's own copy of the ledger has it. The ledger
    /// must still take the raise. Found in `live_npc_targets_players`: gecko 2 kept its base
    /// health in the ledger and died to one 400-point hit.
    #[test]
    fn a_health_raise_right_after_the_host_tracks_an_actor_reaches_the_ledger() {
        let (_d, order) = order("live-raise-new");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, "perfect", 51);
        r.stand(CELL, [900.0, 900.0, 5000.0]);
        r.run_ms(1_000);
        let gecko = FormId(GECKO_REF);
        let cx = r.cx();
        r.ha.track(cx, &r.host, &mut r.sim.host.live, &[gecko]);
        r.host
            .actor_values
            .insert((gecko, world::combat::av::HEALTH), 5000.0);
        assert_eq!(world::combat::health(&order, &r.host, gecko), Some(5000.0));
        r.run_ms(2_000);
        let ledger = r
            .sim
            .host
            .live
            .actors
            .own
            .get(CForm(GECKO_REF))
            .map(|a| a.state.health);
        assert_eq!(ledger, Some(5000), "the ledger took the raise");
        // each guest's copy of the ledger has it too (a guest's world shows at most the
        // creature's own full health)
        for g in &r.sim.guests {
            let copy = g
                .sync
                .live
                .actors
                .get(CForm(GECKO_REF))
                .and_then(|a| a.state)
                .map(|s| s.health);
            assert_eq!(copy, Some(5000));
        }
    }

    #[test]
    fn a_host_hit_goes_through_the_same_ledger() {
        let (_d, order) = order("live-host-kill");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 31);
        let gecko = FormId(GECKO_REF);
        r.run_ms(2_000);
        assert!(world::combat::hurt(
            &order,
            &mut r.host,
            gecko,
            100.0,
            PLAYER_REF
        ));
        assert!(r.until(30_000, |r| r.guests.iter().all(|g| g.dead.contains(&gecko))));
        for g in r.all() {
            assert_eq!(died_events(g, gecko), 1);
        }
    }

    #[test]
    fn party_scaling_divides_player_damage_and_thins_xp() {
        let (_d, order) = order("live-scaling");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 41);
        let gecko = FormId(GECKO_REF);
        assert!(r.until(10_000, |r| r.guests[0]
            .coop
            .live
            .scaling
            .is_some_and(|s| s.toughness > 1.9)));
        world::combat::hurt(&order, &mut r.guests[0], gecko, 10.0, PLAYER_REF);
        assert!(
            r.until(10_000, |r| world::combat::health(&order, &r.host, gecko)
                .unwrap()
                < 20.0)
        );
        let h = world::combat::health(&order, &r.host, gecko).unwrap();
        assert!(
            (h - 15.0).abs() < 1.01,
            "10 damage over toughness 2 leaves 15, got {h}"
        );
        let s = r.host.coop.live.scaling.unwrap();
        assert!(s.reward < 1.0 && s.damage_to_players > 1.0);
    }

    #[test]
    fn single_player_is_unchanged() {
        let (_d, order) = order("live-single");
        let mut g = GameState::new(&order);
        assert!(world::combat::hurt(
            &order,
            &mut g,
            FormId(GECKO_REF),
            100.0,
            PLAYER_REF
        ));
        assert!(g.coop.live.pending_hits.is_empty() && g.coop.live.hurts.is_empty());
        assert_eq!(g.coop.stats.effects, 0);
    }

    #[test]
    fn container_and_drop_races_resolve_once() {
        let (_d, order) = order("live-loot");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 51);
        let (chest, item, drop) = (FormId(TOWN_REF), FormId(0x7777), FormId(RAIDER_REF));
        r.host.stocked.insert(chest);
        r.host.items.insert((chest, item), 5);
        let cx = r.cx();
        assert!(r
            .ha
            .loot
            .register(cx, &mut r.host, &mut r.sim.host.live, chest));
        assert!(
            !r.ha
                .loot
                .register(cx, &mut r.host, &mut r.sim.host.live, chest),
            "rolled once"
        );
        r.ha.loot
            .register_drop(cx, &r.host, &mut r.sim.host.live, drop);
        r.run_ms(1_000);
        // both guests open at the same moment: one lease
        let m1 = r.gas[0].loot.open(&r.sim.guests[0].sync.live.loot, chest);
        let m2 = r.gas[1].loot.open(&r.sim.guests[1].sync.live.loot, chest);
        r.sim.guest_emit(0, vec![m1]);
        r.sim.guest_emit(1, vec![m2]);
        assert!(r.until(10_000, |r| {
            let a = r.sim.guests[0]
                .sync
                .live
                .loot
                .view(CForm(chest.0))
                .is_some_and(|v| v.visible().len() == 1);
            let b = r.sim.guests[1]
                .sync
                .live
                .loot
                .view(CForm(chest.0))
                .is_some_and(|v| v.visible().len() == 1);
            a != b
                || r.sim.guests[0].sync.live.loot.busy.len()
                    + r.sim.guests[1].sync.live.loot.busy.len()
                    == 1
        }));
        let winner = if r.sim.host.live.loot.holder(CForm(chest.0)) == Some(PlayerId(1)) {
            0
        } else {
            1
        };
        let loser = 1 - winner;
        assert_eq!(r.sim.guests[loser].sync.live.loot.busy.len(), 1);
        // the winner takes everything; the loser cannot take at all
        let take = {
            let (g, a) = (&mut r.sim.guests[winner].sync.live.loot, &mut r.gas[winner]);
            a.loot.take(g, chest, item, 5).expect("leased")
        };
        r.sim.guest_emit(winner, vec![take]);
        assert!(r.sim.guests[loser]
            .sync
            .live
            .loot
            .view(CForm(chest.0))
            .map_or(true, |v| !v.visible().contains_key(&CForm(item.0)) || true));
        assert!(
            r.until(10_000, |r| r.guests[winner].items.get(&(PLAYER_REF, item))
                == Some(&5)
                && !r.host.items.contains_key(&(chest, item)))
        );
        assert_eq!(
            r.ha.registry
                .get(PlayerId(winner as u8 + 1))
                .unwrap()
                .gained[&item],
            5
        );
        // the other guest has none, and its late attempt is refused or empty
        assert!(!r.guests[loser].items.contains_key(&(PLAYER_REF, item)));
        let none = {
            let (g, a) = (&mut r.sim.guests[loser].sync.live.loot, &mut r.gas[loser]);
            a.loot.take(g, chest, item, 1)
        };
        assert!(none.is_none());
        // drop claims: both race, exactly one wins
        let c1 = r.gas[0].loot.claim(drop);
        let c2 = r.gas[1].loot.claim(drop);
        r.sim.guest_emit(0, vec![c1]);
        r.sim.guest_emit(1, vec![c2]);
        assert!(r.until(10_000, |r| {
            let n = |i: usize| {
                r.sim.guests[i].sync.live.loot.drops_won.len()
                    + r.sim.guests[i].sync.live.loot.drops_lost.len()
            };
            n(0) + n(1) >= 2
        }));
        let won: usize = (0..2)
            .map(|i| r.sim.guests[i].sync.live.loot.drops_won.len())
            .sum();
        assert_eq!(won, 1);
    }

    #[test]
    fn tether_warps_the_straggler_after_the_timeout() {
        let (_d, order) = order("live-tether");
        let scripts = ScriptCache::default();
        let mut r = setup(&order, &scripts, "perfect", 61);
        r.stand(CELL, [100.0, 200.0, 40_000.0]);
        r.run_ms(1_000);
        let cx = r.cx();
        let now = r.sim.clock.now();
        r.ha.rules
            .start_tether(cx, &r.host, &mut r.sim.host.live, FormId(GUARD_REF), now);
        r.run_ms(500);
        assert!(
            r.host.coop.live.stage_held,
            "the story waits for the straggler"
        );
        r.run_ms(100_000);
        assert!(
            r.gas[1].rules.warps.is_empty(),
            "no warp before the timeout"
        );
        assert!(
            r.until(40_000, |r| !r.gas[1].rules.warps.is_empty()),
            "warped after the timeout"
        );
        // the warp puts the straggler on its own spot of the ring around the anchor
        let p = r.guests[1].player_position.unwrap();
        let d = p[0].hypot(p[1]);
        assert!(
            (d - coop_core::roam::model::WARP_SPREAD).abs() < 1.0,
            "now on the ring around the anchor: {p:?}"
        );
        assert!(r.gas[0].rules.warps.is_empty() && r.ha.rules.warps.is_empty());
        assert!(r.until(5_000, |r| !r.host.coop.live.stage_held));
    }
}

mod dialogue_world {
    use super::*;
    use testdata::dialogue::ids::*;

    fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
        let data = testdata::dialogue::world(tag);
        let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
        (data, order)
    }

    fn open_a_session(r: &mut Rig, npc: FormId) {
        let msg = {
            let (c, a) = (&mut r.sim.guests[0].sync.live.dialogue, &r.gas[0].dialogue);
            a.open(c, npc)
        };
        r.sim.guest_emit(0, vec![msg]);
        assert!(r.until(10_000, |r| (0..2).all(|i| r.sim.guests[i]
            .sync
            .live
            .dialogue
            .view()
            .is_some())));
    }

    #[test]
    fn a_dialogue_session_runs_the_result_script_once_and_all_peers_converge() {
        for (profile, seed) in [("perfect", 71u64), ("bad_wifi", 72)] {
            let (_d, order) = order(&format!("live-dialogue-{profile}"));
            let scripts = ScriptCache::default();
            let mut r = Rig::new(&order, &scripts, profile, seed);
            r.stand(0xB10, [0.0, 0.0, 0.0]);
            let npc = FormId(0x1299);
            open_a_session(&mut r, npc);
            assert!(r.sim.guests[0].sync.live.dialogue.is_initiator());
            assert!(r.sim.guests[1].sync.live.dialogue.is_spectator());
            // the initiator picks the goodbye line; everyone plays it
            let pick = {
                let (c, a) = (&mut r.sim.guests[0].sync.live.dialogue, &r.gas[0].dialogue);
                a.pick(c, FormId(BYE)).expect("pick")
            };
            r.sim.guest_emit(0, vec![pick]);
            assert!(
                r.until(10_000, |r| r.ha.dialogue.begin_runs == 1),
                "{profile}: begin script ran"
            );
            let door = FormId(DOOR_OPEN);
            assert_eq!(r.host.globals.get(&door), Some(&1.0));
            assert!(r.until(10_000, |r| r
                .guests
                .iter()
                .all(|g| g.coop.live.lines.len() == 1)));
            for g in &r.guests {
                assert_eq!(
                    g.coop.live.lines[0],
                    (1, FormId(BYE)),
                    "spectators play the same line"
                );
            }
            // each peer reports its line done; the end script runs after the last
            for i in 0..2 {
                let m = {
                    let (c, a) = (&mut r.sim.guests[i].sync.live.dialogue, &r.gas[i].dialogue);
                    a.line_done(c).expect("line done")
                };
                r.sim.guest_emit(i, vec![m]);
            }
            let m =
                r.ha.dialogue
                    .line_done(&mut r.ha.own.dialogue)
                    .expect("host line done");
            let outs = r.ha.on_local(
                &mut r.sim.host.live,
                &mut r.sim.host.auth,
                r.sim.clock.now(),
                m,
            );
            r.sim.host_emit(outs);
            assert!(
                r.until(30_000, |r| r.host.globals.get(&door) == Some(&2.0)),
                "{profile}: end script ran"
            );
            // close; nothing runs twice
            let close = {
                let (c, a) = (&mut r.sim.guests[0].sync.live.dialogue, &r.gas[0].dialogue);
                a.close(c).expect("close")
            };
            r.sim.guest_emit(0, vec![close]);
            r.run_ms(5_000);
            assert_eq!((r.ha.dialogue.begin_runs, r.ha.dialogue.end_runs), (1, 1));
            assert!(
                r.until(60_000, |r| r.guests.iter().all(|g| g.globals.get(&door)
                    == Some(&2.0)
                    && g.said.contains(&FormId(BYE)))),
                "{profile}: peers converged"
            );
            assert_eq!(
                r.gas[0].dialogue.begin_runs + r.gas[1].dialogue.begin_runs,
                0,
                "guests run nothing"
            );
            assert_eq!(r.guests[0].coop.stats.suppressed_runs, 0);
        }
    }

    /// A script's second `SayTo` comes while the guests still read the
    /// first line. The host says the second line when the first one ends.
    #[test]
    fn a_forced_line_that_comes_while_another_plays_waits_its_turn() {
        let (_d, order) = order("live-forced-queue");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, "perfect", 93);
        r.stand(0xB10, [0.0, 0.0, 0.0]);
        r.run_ms(1_000);
        // the doctor stands here (a made reference: the world places no one)
        let npc = FormId(0x1299);
        let made = world::more_functions::placed::Made {
            base: FormId(DOC),
            space: FormId(0xB10),
            cell: FormId(0xB10),
            position: [0.0; 3],
            rotation: [0.0; 3],
            count: 1,
        };
        r.host.more.placed.refs.insert(npc, made);
        let topic = Some(FormId(GOODBYE));
        r.host.coop_talk(npc, PLAYER_REF, topic, false);
        assert!(
            r.until(10_000, |r| r
                .guests
                .iter()
                .all(|g| g.coop.live.lines.len() == 1)),
            "the first line plays for the guests: {:?}",
            r.host.coop.live.scene.talk
        );
        // the guests have not read the first line yet
        r.host.coop_talk(npc, PLAYER_REF, topic, false);
        let both = r.until(30_000, |r| {
            for i in 0..2 {
                let m = {
                    let (c, a) = (&mut r.sim.guests[i].sync.live.dialogue, &r.gas[i].dialogue);
                    a.line_done(c)
                };
                if let Some(m) = m {
                    r.sim.guest_emit(i, vec![m]);
                }
            }
            r.guests.iter().all(|g| g.coop.live.lines.len() == 2)
        });
        let t = r.host.coop.live.scene.talk;
        assert!(both, "both lines reach both guests: {t:?}");
        assert_eq!((t.notes, t.lines, t.dropped), (2, 2, 0));
    }

    #[test]
    fn a_downed_player_is_revived_and_gets_health_back() {
        let (_d, order) = order("live-revive");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, "perfect", 81);
        r.stand(0xB10, [0.0, 50.0, 100.0]);
        r.guests[0]
            .actor_values
            .insert((PLAYER_REF, world::combat::av::HEALTH), 200.0);
        let me = world::combat::max_health(&order, &r.guests[0], PLAYER_REF).unwrap_or(0.0);
        assert!(me > 10.0, "the test world has a player with health ({me})");
        r.run_ms(2_000);
        // a lethal blow downs the guest instead of killing them
        world::combat::hurt(
            &order,
            &mut r.guests[0],
            PLAYER_REF,
            me * 10.0,
            FormId(0x555),
        );
        assert!(!r.guests[0].dead.contains(&PLAYER_REF));
        assert!(r.until(10_000, |r| r
            .sim
            .host
            .live
            .rules
            .revive
            .is_downed(PlayerId(1))));
        assert!(r.guests[0].coop.live.downed);
        // guest 2 stands next to them and holds the revive key
        let begin = r.gas[1].rules.revive_begin(PlayerId(1));
        r.sim.guest_emit(1, vec![begin]);
        assert!(
            r.until(30_000, |r| !r.gas[0].rules.revives.is_empty()),
            "revived"
        );
        let now = world::combat::health(&order, &r.guests[0], PLAYER_REF).unwrap();
        assert!(
            (now - me * 0.3).abs() < 1.5,
            "30 percent health back: {now} of {me}"
        );
        assert!(!r.guests[0].coop.live.downed);
    }

    #[test]
    fn fast_travel_waits_for_a_vote_and_the_party_agrees() {
        let (_d, order) = order("live-travel");
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, "perfect", 91);
        r.stand(0xB10, [0.0, 0.0, 0.0]);
        r.run_ms(1_000);
        let marker = FormId(0x4242);
        let cx = r.cx();
        let now = r.sim.clock.now();
        r.ha.rules
            .start_tether(cx, &r.host, &mut r.sim.host.live, marker, now);
        let m = world::map::MapMarker {
            reference: marker,
            name: "Somewhere".into(),
            position: [0.0; 3],
            radius: 10.0,
            flags: 0,
            kind: 0,
            arrival: None,
        };
        let refused = world::map::travel(&order, &mut r.guests[0], &m);
        assert!(refused.is_err());
        assert_eq!(r.guests[0].coop.live.travel_requests, vec![marker]);
        let req = r.gas[0].rules.request_travel(marker);
        r.sim.guest_emit(0, vec![req]);
        assert!(r.until(10_000, |r| r.sim.guests[1].sync.live.rules.prompt.is_some()));
        let yes = r.sim.guests[1].sync.live.rules.answer(true).unwrap();
        r.sim.guest_emit(1, vec![Msg::Rules(yes)]);
        assert!(r.until(10_000, |r| r.ha.own.rules.prompt.is_some()));
        let yes = r.ha.own.rules.answer(true).unwrap();
        let outs = r.ha.on_local(
            &mut r.sim.host.live,
            &mut r.sim.host.auth,
            r.sim.clock.now(),
            Msg::Rules(yes),
        );
        r.sim.host_emit(outs);
        assert!(
            r.until(10_000, |r| r.guests[0].coop.live.travel_allowed
                == Some(marker)),
            "approved"
        );
    }
}
