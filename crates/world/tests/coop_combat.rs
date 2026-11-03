//! Combat depth on the world crate: NPCs hurting remote players, downed and
//! revive from NPC damage, friendly fire, per-player stealth, area blasts,
//! real-time VATS, shots relayed, extras. A host and two guests, each a
//! `GameState`, over coop-core's chaos transport. Generated data only.

use coop_core::combat::FriendlyFire;
use coop_core::ids::PlayerId;
use coop_core::rules::Mode;
use coop_core::sim_test::Sim;
use coop_core::sync::GuestPhase;
use coop_core::transport::ChaosProfile;
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fighting::ids::*;
use world::coop::apply::{Ctx, Opts};
use world::coop::combat_live::{
    bot_line, remote_seen, world_clock_may_change, CombatBridge, Remote,
};
use world::coop::facts::puppet_of;
use world::coop::live::{LiveGuestAdapter, LiveHostAdapter};
use world::coop::{Authority, GuestBridge, HostBridge};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, ScriptCache};

const G1: PlayerId = PlayerId(1);
const G2: PlayerId = PlayerId(2);

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

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
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
            gas: vec![LiveGuestAdapter::new(G1), LiveGuestAdapter::new(G2)],
            hb,
            gbs: vec![GuestBridge::default(), GuestBridge::default()],
        };
        let ok = r.sim.run_until(120_000, |s| {
            s.guests
                .iter()
                .all(|g| g.sync.phase() == GuestPhase::Active)
        });
        assert!(ok, "guests never became active");
        for g in std::iter::once(&mut r.host).chain(r.guests.iter_mut()) {
            // the generated world has no player record: give everyone 100 health
            g.actor_values.insert((PLAYER_REF, 16), 100.0);
        }
        r.stand([900.0, 900.0, 5000.0]);
        let cx = r.cx();
        r.ha.track(
            cx,
            &r.host,
            &mut r.sim.host.live,
            &[FormId(GECKO_REF), FormId(GUARD_REF)],
        );
        r
    }

    fn cx(&self) -> Ctx<'a> {
        Ctx {
            order: self.order,
            scripts: self.scripts,
        }
    }

    fn stand(&mut self, xs: [f32; 3]) {
        for (i, g) in std::iter::once(&mut self.host)
            .chain(self.guests.iter_mut())
            .enumerate()
        {
            g.player_cell = Some(FormId(CELL));
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

    /// Everyone sees everyone as a puppet.
    fn settled(&mut self) {
        assert!(
            self.until(30_000, |r| {
                r.host.coop.live.combat.puppets.len() == 2
                    && r.guests
                        .iter()
                        .all(|g| g.coop.live.combat.puppets.len() == 2)
            }),
            "puppets never appeared"
        );
    }
}

fn form(order: &LoadOrder, w: &str) -> Option<FormId> {
    order
        .form_by_editor_id(w)
        .or_else(|| u32::from_str_radix(w, 16).ok().map(FormId))
}

fn lost(g: &GameState) -> f64 {
    g.damage.get(&PLAYER_REF).copied().unwrap_or(0.0)
}

#[test]
fn an_npc_hurts_remote_players_and_only_the_victim_feels_it() {
    let profiles: Vec<&str> = std::iter::once("perfect")
        .chain(coop_core::transport::PROFILE_NAMES.iter().copied())
        .collect();
    for (seed, profile) in profiles.into_iter().enumerate() {
        let seed = seed as u64 + 1;
        let (_d, order) = order(&format!("cc-npc-{profile}"));
        let scripts = ScriptCache::default();
        let mut r = Rig::new(&order, &scripts, profile, seed);
        r.settled();
        let gecko = FormId(GECKO_REF);
        // guest 1's machine runs the gecko: it bites guest 2 and the host player
        world::combat::hurt(&order, &mut r.guests[0], puppet_of(G2), 10.0, gecko);
        world::combat::hurt(
            &order,
            &mut r.guests[0],
            puppet_of(PlayerId::HOST),
            10.0,
            gecko,
        );
        // the host runs the guard: it shoots guest 1
        world::combat::hurt(&order, &mut r.host, puppet_of(G1), 10.0, FormId(GUARD_REF));
        assert!(
            r.until(30_000, |r| lost(&r.guests[1]) > 0.0
                && lost(&r.host) > 0.0
                && lost(&r.guests[0]) > 0.0),
            "{profile}"
        );
        assert!(
            lost(&r.guests[1]) >= 10.0,
            "{profile}: party scaling only raises it: {}",
            lost(&r.guests[1])
        );
        r.run_ms(2_000);
        for (i, g) in r.guests.iter().enumerate() {
            assert_eq!(
                g.coop.live.combat.stats.hurts_applied, 1,
                "{profile}: guest {i} took exactly one hurt"
            );
        }
        assert_eq!(r.host.coop.live.combat.stats.hurts_applied, 1, "{profile}");
        // nothing was applied to the puppets locally
        assert!(!r.guests[0].damage.contains_key(&puppet_of(G2)));
        assert!(
            r.sim.host.live.combat.vitals.stats.applied >= 3,
            "{profile}"
        );
    }
}

#[test]
fn a_flood_of_npc_blows_is_coalesced_and_bounded() {
    let (_d, order) = order("cc-flood");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 3);
    r.settled();
    for _ in 0..50 {
        world::combat::hurt(
            &order,
            &mut r.guests[0],
            puppet_of(G2),
            1.0,
            FormId(GECKO_REF),
        );
    }
    r.run_ms(3_000);
    assert_eq!(
        r.guests[0].coop.live.combat.stats.npc_hits_sent, 1,
        "one request per attacker per frame"
    );
    assert!(
        (lost(&r.guests[1]) - 50.0 * 1.1).abs() < 6.0,
        "{}",
        lost(&r.guests[1])
    );
}

#[test]
fn downed_by_an_npc_then_revived_and_the_party_wipe_rule() {
    let (_d, order) = order("cc-down");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 4);
    r.settled();
    let full = world::combat::max_health(&order, &r.guests[1], PLAYER_REF).expect("player health");
    assert_eq!(full, 100.0);
    // a blow that would kill guest 2
    world::combat::hurt(
        &order,
        &mut r.guests[0],
        puppet_of(G2),
        full * 2.0,
        FormId(GECKO_REF),
    );
    assert!(
        r.until(30_000, |r| r.guests[1].coop.live.downed),
        "guest 2 goes down, not dead"
    );
    assert!(!r.guests[1].dead.contains(&PLAYER_REF));
    assert!(r.guests[1].coop.live.combat.stats.downs >= 1);
    assert!(
        r.until(30_000, |r| r.sim.host.live.rules.revive.is_downed(G2)),
        "the host knows"
    );
    // more blows on a downed player do nothing
    let before = lost(&r.guests[1]);
    world::combat::hurt(
        &order,
        &mut r.guests[0],
        puppet_of(G2),
        30.0,
        FormId(GECKO_REF),
    );
    r.run_ms(3_000);
    assert_eq!(lost(&r.guests[1]), before);
    assert!(r.sim.host.live.combat.vitals.stats.while_downed >= 1);
    // guest 1 stands next to them and holds the revive key
    r.guests[0].player_position = Some([910.0, 1000.0, 0.0]);
    r.guests[1].player_position = Some([900.0, 1000.0, 0.0]);
    r.run_ms(1_000);
    r.sim.guest_emit(0, vec![Msg_revive(G2)]);
    assert!(
        r.until(60_000, |r| !r.guests[1].coop.live.downed),
        "revived"
    );
    let after = world::combat::health(&order, &r.guests[1], PLAYER_REF).unwrap();
    assert!(
        after > 0.0 && after < full,
        "back up on part of their health: {after} of {full}"
    );
    // and they can be hurt again
    r.run_ms(1_500);
    world::combat::hurt(
        &order,
        &mut r.guests[0],
        puppet_of(G2),
        5.0,
        FormId(GECKO_REF),
    );
    assert!(r.until(30_000, |r| lost(&r.guests[1]) > full - after));
}

#[allow(non_snake_case)]
fn Msg_revive(target: PlayerId) -> coop_core::proto::Msg {
    coop_core::proto::Msg::Rules(coop_core::rules::RulesMsg::ReviveBegin { target })
}

#[test]
fn friendly_fire_is_off_until_the_campaign_turns_it_on() {
    let (_d, order) = order("cc-ff");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 5);
    r.settled();
    world::combat::hurt(&order, &mut r.guests[0], puppet_of(G2), 20.0, PLAYER_REF);
    r.run_ms(3_000);
    assert_eq!(lost(&r.guests[1]), 0.0, "guest 1 cannot hurt guest 2");
    assert_eq!(r.sim.host.live.combat.vitals.stats.friendly_blocked, 1);
    r.sim.host.live.combat.vitals.friendly = FriendlyFire::Scaled(0.5);
    world::combat::hurt(&order, &mut r.guests[0], puppet_of(G2), 20.0, PLAYER_REF);
    assert!(r.until(30_000, |r| lost(&r.guests[1]) > 0.0));
    assert!(
        (lost(&r.guests[1]) - 10.0).abs() < 0.6,
        "{}",
        lost(&r.guests[1])
    );
    // the host player hitting a guest follows the same rule
    r.sim.host.live.combat.vitals.friendly = FriendlyFire::Off;
    world::combat::hurt(&order, &mut r.host, puppet_of(G1), 20.0, PLAYER_REF);
    r.run_ms(3_000);
    assert_eq!(lost(&r.guests[0]), 0.0);
}

#[test]
fn each_player_has_their_own_stealth_for_the_ai() {
    let (_d, order) = order("cc-stealth");
    let mut gs = GameState::new(&order);
    gs.player_cell = Some(FormId(CELL));
    gs.player_position = Some([5000.0, 0.0, 0.0]);
    let gecko = FormId(GECKO_REF);
    gs.positions.insert(gecko, ([0.0, 0.0, 0.0], 0.0));
    let mut bridge = CombatBridge::new();
    let remote = |p: PlayerId, y: f32, flags: u16| Remote {
        player: p,
        cell: CELL,
        pos: [0.0, y, 0.0],
        heading: 0.0,
        vel: [0.0, 100.0, 0.0],
        flags,
    };
    // same distance in front of the gecko, same speed; one sneaks
    gs.positions.insert(gecko, ([0.0, 0.0, 0.0], 0.0));
    bridge.sync_remotes(
        &mut gs,
        PlayerId::HOST,
        &[
            remote(G1, 500.0, coop_core::avatar::FLAG_SNEAK),
            remote(G2, 500.0, 0),
        ],
    );
    let facts = Facts {
        order: &order,
        state: &gs,
        speaker: None,
    };
    let sneaker = facts
        .detection(gecko, puppet_of(G1))
        .expect("puppets have a place");
    let walker = facts
        .detection(gecko, puppet_of(G2))
        .expect("puppets have a place");
    assert!(
        sneaker < walker,
        "sneaking hides only the one who sneaks: {sneaker} vs {walker}"
    );
    // a downed player is no target at all
    bridge.sync_remotes(
        &mut gs,
        PlayerId::HOST,
        &[
            remote(G1, 500.0, coop_core::avatar::FLAG_DOWNED),
            remote(G2, 500.0, 0),
        ],
    );
    let seen: Vec<FormId> = remote_seen(&gs).into_iter().map(|s| s.0).collect();
    assert_eq!(seen, vec![puppet_of(G2)]);
    // a shot is that player's noise, not the local player's
    gs.noise.insert(
        puppet_of(G2),
        world::noise::Noise {
            value: 100,
            timer: 5.0,
        },
    );
    let facts = Facts {
        order: &order,
        state: &gs,
        speaker: None,
    };
    let loud = facts.detection(gecko, puppet_of(G2)).unwrap();
    assert!(loud > walker, "{loud} vs {walker}");
    assert_eq!(world::noise::value(&gs, PLAYER_REF), 0);
}

#[test]
fn enemy_attention_moves_to_the_player_who_hurts_it_and_skips_the_downed() {
    let (_d, order) = order("cc-threat");
    let mut gs = GameState::new(&order);
    gs.set_authority(Authority::Host);
    gs.player_cell = Some(FormId(CELL));
    gs.player_position = Some([1500.0, 0.0, 0.0]);
    let gecko = FormId(GECKO_REF);
    gs.positions.insert(gecko, ([0.0, 0.0, 0.0], 0.0));
    gs.combat.insert(gecko, PLAYER_REF);
    let mut b = CombatBridge::new();
    let at = |p: PlayerId, x: f32| Remote {
        player: p,
        cell: CELL,
        pos: [x, 0.0, 0.0],
        heading: 0.0,
        vel: [0.0; 3],
        flags: 0,
    };
    b.sync_remotes(&mut gs, PlayerId::HOST, &[at(G1, 400.0), at(G2, 4000.0)]);
    gs.coop.live.scaling = Some(coop_core::rules::Scaling::for_party(3, Mode::CoopBalanced));
    // 1500 vs 400 units: the closer player wins once the hold time passed
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 0);
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 10_000);
    let t = gs.combat.get(&gecko).copied();
    assert!(t == Some(puppet_of(G1)) || t == Some(PLAYER_REF), "{t:?}");
    // the near one goes down: the gecko turns to someone standing
    b.sync_remotes(
        &mut gs,
        PlayerId::HOST,
        &[
            Remote {
                flags: coop_core::avatar::FLAG_DOWNED,
                ..at(G1, 400.0)
            },
            at(G2, 4000.0),
        ],
    );
    gs.coop.live.downed = false;
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 20_000);
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 30_000);
    assert_eq!(
        gs.combat.get(&gecko),
        Some(&PLAYER_REF),
        "the host player is the nearest standing player"
    );
    gs.coop.live.downed = true;
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 40_000);
    b.retarget(&order, &mut gs, &[gecko], PlayerId::HOST, 50_000);
    assert_ne!(gs.combat.get(&gecko), Some(&puppet_of(G1)));
}

#[test]
fn area_blasts_hit_players_and_npcs_and_show_on_the_other_machines() {
    let (_d, order) = order("cc-blast");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 7);
    // the thrower is the host at x=900; guest 1 stands beside it, guest 2 far away
    r.stand([900.0, 960.0, 9000.0]);
    r.settled();
    r.host
        .positions
        .insert(FormId(GUARD_REF), ([880.0, 1000.0, 0.0], 0.0));
    r.host
        .coop
        .live
        .combat
        .set_next(coop_core::combat::DamageKind::Explosion, 1);
    let f = |w: &str| form(&order, w);
    let words = ["coop", "explode", "B03", "400", "20"];
    assert!(bot_line(&order, &mut r.host, &words, &f));
    r.run_ms(3_000);
    assert!(lost(&r.host) > 0.0, "the thrower takes their own blast");
    assert!(
        r.host
            .damage
            .get(&FormId(GUARD_REF))
            .copied()
            .unwrap_or(0.0)
            > 0.0,
        "the guard standing there is hurt"
    );
    assert_eq!(
        lost(&r.guests[0]),
        0.0,
        "friendly fire is off: guest 1 beside the thrower is spared"
    );
    assert_eq!(r.sim.host.live.combat.vitals.stats.friendly_blocked, 1);
    // with friendly fire on, the near guest is hit and the far one is not
    r.sim.host.live.combat.vitals.friendly = FriendlyFire::On;
    assert!(bot_line(&order, &mut r.host, &words, &f));
    assert!(
        r.until(30_000, |r| lost(&r.guests[0]) > 0.0),
        "guest 1 is in the blast"
    );
    r.run_ms(3_000);
    assert_eq!(lost(&r.guests[1]), 0.0, "guest 2 is far away");
    assert!(r.host.coop.live.combat.stats.blast_hits >= 4);
    assert!(
        r.until(10_000, |r| !r.guests[0]
            .coop
            .live
            .combat
            .shown_blasts
            .is_empty()
            && !r.guests[1].coop.live.combat.shown_blasts.is_empty()),
        "both guests are shown it"
    );
    // thrown by a guest the same way
    r.sim.host.live.combat.vitals.friendly = FriendlyFire::On;
    r.guests[0].damage.clear();
    let before = lost(&r.host);
    assert!(bot_line(&order, &mut r.guests[0], &words, &f));
    assert!(
        r.until(30_000, |r| lost(&r.host) > before),
        "the host player feels a guest's blast"
    );
}

#[test]
fn vats_is_real_time_for_everyone_else_and_resolves_on_the_host() {
    let (_d, order) = order("cc-vats");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 8);
    r.settled();
    // a shared world never changes its clock for VATS
    assert!(!world_clock_may_change(&r.host) && !world_clock_may_change(&r.guests[0]));
    assert!(world_clock_may_change(&GameState::new(&order)));
    let f = |w: &str| form(&order, w);
    let gecko = FormId(GECKO_REF);
    r.run_ms(2_000);
    assert!(bot_line(
        &order,
        &mut r.guests[0],
        &["coop", "vatsmark", "1"],
        &f
    ));
    assert!(bot_line(
        &order,
        &mut r.guests[0],
        &["coop", "vats", "B11", "0", "1000", "4"],
        &f
    ));
    assert!(bot_line(
        &order,
        &mut r.guests[0],
        &["coop", "vats", "B11", "0", "1000", "4"],
        &f
    ));
    assert!(
        r.until(30_000, |r| !r.guests[0]
            .coop
            .live
            .combat
            .vats_results
            .is_empty()),
        "the verdict came back"
    );
    let (_, results, spent) = r.guests[0].coop.live.combat.vats_results[0].clone();
    assert_eq!(results.len(), 2);
    assert_eq!(spent, 40);
    assert!(
        r.until(30_000, |r| world::combat::health(&order, &r.host, gecko)
            .unwrap()
            < 20.0),
        "damage reached the gecko on the host"
    );
    assert!(
        r.until(10_000, |r| r.sim.guests[1]
            .sync
            .live
            .combat
            .vats_active
            .contains(&G1)),
        "the others see only a mark"
    );
    assert!(
        r.guests[1].coop.live.combat.stats.vats_clock_kept == 0,
        "nothing paused on the bystander"
    );
    assert!(
        r.guests[0].coop.live.combat.stats.vats_clock_kept >= 1,
        "the clock request was refused on the actor's side"
    );
    assert!(bot_line(
        &order,
        &mut r.guests[0],
        &["coop", "vatsmark", "0"],
        &f
    ));
    assert!(r.until(10_000, |r| !r.sim.guests[1]
        .sync
        .live
        .combat
        .vats_active
        .contains(&G1)));
}

#[test]
fn shots_are_relayed_with_the_shooters_own_noise() {
    let (_d, order) = order("cc-fire");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 9);
    r.settled();
    let f = |w: &str| form(&order, w);
    assert!(bot_line(
        &order,
        &mut r.guests[0],
        &["coop", "fire", "B04", "100", "200", "0"],
        &f
    ));
    assert!(
        r.until(30_000, |r| !r.host.coop.live.combat.shown_fires.is_empty()
            && !r.guests[1].coop.live.combat.shown_fires.is_empty())
    );
    assert!(
        r.guests[0].coop.live.combat.shown_fires.is_empty(),
        "no echo to the shooter"
    );
    let shot = r.guests[1].coop.live.combat.shown_fires[0];
    assert_eq!(shot.shooter, G1);
    assert_eq!(shot.weapon.0, PISTOL);
    assert!(
        world::noise::value(&r.guests[1], puppet_of(G1)) != 0,
        "heard as player 1's"
    );
    assert_eq!(world::noise::value(&r.guests[1], PLAYER_REF), 0);
    assert_eq!(world::noise::value(&r.host, puppet_of(G2)), 0);
}

#[test]
fn bigger_parties_bring_extras_to_dynamic_encounters_and_clear_them_when_left() {
    let (_d, order) = order("cc-extras");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 10);
    r.settled();
    r.run_ms(3_000);
    // guest 1 rolls an encounter: forty geckos at forty sites
    for i in 0..40u32 {
        let (_, m) = r.sim.guests[0].sync.live.actors.request_spawn(
            coop_core::ids::FormId(0xFC00_0000 + i),
            coop_core::ids::FormId(GECKO),
            coop_core::ids::CellId(CELL),
            [16 * 900, 16 * 1000, 0],
            20,
        );
        r.sim.guest_emit(0, vec![coop_core::proto::Msg::Actor(m)]);
    }
    assert!(
        r.until(60_000, |r| r.host.coop.live.combat.stats.extras_made > 0),
        "some extras come"
    );
    r.run_ms(5_000);
    let made = r.host.coop.live.combat.stats.extras_made;
    let cap = coop_core::rules::Scaling::for_party(3, Mode::CoopBalanced).max_extras_alive;
    assert!(made <= cap, "{made} extras but at most {cap} alive");
    let spawned: Vec<FormId> = r
        .host
        .coop
        .live
        .spawns
        .iter()
        .filter(|s| s.base.is_some())
        .map(|s| s.id)
        .collect();
    assert!(spawned.len() as u32 >= made);
    // replicated: a guest hears about every extra
    let extras = r.host.coop.live.combat.extras.clone();
    assert!(r.until(30_000, |r| extras.iter().all(|e| r.sim.guests[1]
        .sync
        .live
        .actors
        .spawned
        .contains_key(&coop_core::ids::FormId(e.0)))));
    // everyone walks far away: the extras are cleared
    r.stand([40_000.0, 40_000.0, 40_000.0]);
    for (i, g) in std::iter::once(&mut r.host)
        .chain(r.guests.iter_mut())
        .enumerate()
    {
        g.player_position = Some([40_000.0 + 100.0 * i as f32, 0.0, 0.0]);
    }
    assert!(
        r.until(60_000, |r| r.host.coop.live.combat.stats.extras_cleared
            >= 1)
    );
    // solo: no extras at all
    let mut solo = coop_core::rules::Scaling::for_party(1, Mode::CoopBalanced);
    solo.extras = 0.0;
    assert!((0..100u32).all(|i| !solo.spawns_extra(coop_core::ids::FormId(i))));
}

#[test]
fn radiation_and_crippled_limbs_land_on_the_right_player() {
    let (_d, order) = order("cc-effects");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 11);
    r.settled();
    let cx = r.cx();
    let _ = cx;
    let m = r.sim.host.live.combat.vitals.effect(
        0,
        G2,
        coop_core::combat::EffectKind::Radiation,
        30,
        0,
        0,
    );
    let m2 = r.sim.host.live.combat.vitals.effect(
        0,
        G2,
        coop_core::combat::EffectKind::Cripple,
        0,
        4,
        0,
    );
    let outs = r
        .sim
        .host
        .live
        .route_combat(m.into_iter().chain(m2).collect());
    r.sim.host_emit(outs);
    assert!(r.until(30_000, |r| r.guests[1]
        .coop
        .live
        .combat
        .stats
        .effects_applied
        == 2));
    assert_eq!(r.guests[0].coop.live.combat.stats.effects_applied, 0);
    assert_eq!(r.host.coop.live.combat.stats.effects_applied, 0);
    // the crippled leg is the 29th value, damaged to nothing
    let d = r.guests[1]
        .value_damage
        .get(&(PLAYER_REF, 29))
        .copied()
        .unwrap_or(0.0);
    assert!(d > 0.0, "leg condition damaged: {d}");
    let rads = r.guests[1]
        .value_damage
        .get(&(PLAYER_REF, 54))
        .copied()
        .unwrap_or(0.0);
    assert!(rads > 0.0, "radiation counter rose: {rads}");
}

#[test]
fn single_player_pays_nothing() {
    let (_d, order) = order("cc-single");
    let mut g = GameState::new(&order);
    assert!(world_clock_may_change(&g));
    assert!(!bot_line(&order, &mut g, &["coop", "unknown"], &|_| None));
    world::combat::hurt(&order, &mut g, FormId(GECKO_REF), 5.0, PLAYER_REF);
    let c = &g.coop.live.combat;
    assert!(c.npc_hits.is_empty() && c.pvp_hits.is_empty() && c.puppets.is_empty());
}

#[test]
fn a_health_raise_on_the_host_stays_and_reaches_the_guests() {
    let (_d, order) = order("cc-heal");
    let scripts = ScriptCache::default();
    let mut r = Rig::new(&order, &scripts, "perfect", 5);
    r.settled();
    let gecko = FormId(GECKO_REF);
    let hp = |g: &GameState| world::combat::health(&order, g, gecko).unwrap();
    let ledger = |r: &Rig, i: usize| {
        r.sim.guests[i]
            .sync
            .live
            .actors
            .iter()
            .find(|(id, _)| id.0 == GECKO_REF)
            .and_then(|(_, rec)| rec.state)
            .map(|s| f64::from(s.health))
    };
    let full = hp(&r.host);
    // a guard bites the gecko on the host: everyone loses 10
    world::combat::hurt(&order, &mut r.host, gecko, 10.0, FormId(GUARD_REF));
    r.run_ms(2_000);
    for (i, g) in std::iter::once(&r.host).chain(r.guests.iter()).enumerate() {
        assert!(
            (hp(g) - (full - 10.0)).abs() < 1.0,
            "machine {i} after the bite: {}",
            hp(g)
        );
    }
    // a host script gives the 10 back (RestoreActorValue Health 10)
    world::magic::change(&order, &mut r.host, gecko, 16, 10.0, gecko);
    r.run_ms(2_000);
    for (i, g) in std::iter::once(&r.host).chain(r.guests.iter()).enumerate() {
        assert!(
            (hp(g) - full).abs() < 1.0,
            "machine {i} after the heal: {} of {full}",
            hp(g)
        );
    }
    // a host script raises it to 5000 (ForceActorValue Health 5000)
    r.host.actor_values.insert((gecko, 16), 5000.0);
    r.run_ms(2_000);
    assert!((hp(&r.host) - 5000.0).abs() < 1.0, "host: {}", hp(&r.host));
    for i in 0..2 {
        let l = ledger(&r, i).unwrap();
        assert!((l - 5000.0).abs() < 1.0, "guest {i} ledger: {l}");
    }
    // a guest's hit starts from the raise and does not kill it
    world::combat::hurt(&order, &mut r.guests[0], gecko, 100.0, PLAYER_REF);
    r.run_ms(2_000);
    let after = hp(&r.host);
    assert!(
        after < 4990.0 && after > 4800.0,
        "host after the guest's hit: {after}"
    );
    assert!(!r.host.dead.contains(&gecko));
    for i in 0..2 {
        assert!(
            !r.guests[i].dead.contains(&gecko),
            "guest {i} keeps it alive"
        );
        let l = ledger(&r, i).unwrap();
        assert!(
            (l - after).abs() < 1.0,
            "guest {i} ledger {l}, host {after}"
        );
    }
}
