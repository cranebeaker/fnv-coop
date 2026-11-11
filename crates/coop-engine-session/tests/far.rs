//! Actors the host's game does not run, and held actors moved in steps, on three games in one
//! process (docs/coop/SPECS.md S21): one authority for every actor, wherever the players are.

use coop_core::ids::{FormId, PlayerId};
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::fight::{self, Fight};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const TOWN: u32 = 0x0010_5a01;
const RAIDER: u32 = 0x0010_4f25;
const GECKO: u32 = 0x0010_4f26;
const PLAYER: u32 = 0x14;
const ANN: PlayerId = PlayerId(1);
const BEN: PlayerId = PlayerId(2);

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE, TOWN],
        actors: vec![RAIDER, GECKO],
        ..StorySpec::default()
    }
}

fn hello() -> Hello {
    let plugins = vec!["FalloutNV.esm".to_string()];
    Hello {
        proto: PROTOCOL_VERSION,
        build_hash: 1,
        rules_hash: 2,
        mods_hash: coop_core::compat::plugin_list_hash(&plugins),
        exe: "1.4.0.525".into(),
        volatile_hash: 3,
        plugins,
    }
}

struct Guest {
    game: Story,
    d: GuestDriver,
    net: LoopbackEndpoint,
}

/// The host in the house and two guests, Ann and Ben, who stay in the town (their ladders stop
/// before the warp to the host).
struct Trio {
    host: Story,
    hd: HostDriver,
    hnet: LoopbackEndpoint,
    ann: Guest,
    ben: Guest,
    t: u64,
}

fn guest(net: &LoopbackNet, p: PlayerId, at: [f32; 3]) -> Guest {
    let mut game = Story::new(&spec());
    gate_guest(&mut game.e);
    let mut d = GuestDriver::new(p, hello());
    d.ladder = Some(Ladder::new(true, Cap::Log, DEFAULT_AVATAR_BASE).quiet());
    d.fight = Some(Fight::new(true, fight::Cap::parse("")).quiet());
    game.move_player(TOWN, at, 0.0);
    Guest {
        game,
        d,
        net: net.endpoint(p),
    }
}

impl Guest {
    fn fight(&self) -> &Fight {
        self.d.fight.as_ref().unwrap()
    }

    fn owns(&self, id: u32) -> bool {
        self.d.sync.live.actors.owns(FormId(id))
    }

    fn story_is_whole(&mut self) {
        assert_eq!(echo(&mut self.game.e, &self.d.sync.store), vec![]);
        assert_eq!(self.d.sync.desyncs_detected, 0);
        assert_eq!(self.d.refused, 0);
        assert_eq!(self.game.scripts_run(), 0);
    }
}

impl Trio {
    fn new() -> Trio {
        let net = LoopbackNet::new();
        let mut host = Story::new(&spec());
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        hd.ladder = Some(Ladder::new(false, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
        hd.fight = Some(Fight::new(false, fight::Cap::parse("")).quiet());
        hd.start(&mut host.e);
        host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
        Trio {
            hnet: net.endpoint(PlayerId::HOST),
            ann: guest(&net, ANN, [5.0, 5.0, 5.0]),
            ben: guest(&net, BEN, [50.0, 5.0, 5.0]),
            host,
            hd,
            t: 0,
        }
    }

    fn frames(&mut self, n: u32) {
        self.frames_of(n, 50);
    }

    /// `n` frames of `ms` milliseconds each.
    fn frames_of(&mut self, n: u32, ms: u64) {
        for _ in 0..n {
            self.t += ms;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            for g in [&mut self.ann, &mut self.ben] {
                g.d.step(&mut g.game.e, &mut g.net, self.t);
            }
        }
    }

    /// The ledger's owner of an actor, and its health there.
    fn ledger(&self, id: u32) -> Option<(Option<PlayerId>, u16)> {
        let rec = self.hd.sync.live.actors.own.get(FormId(id))?;
        Some((rec.owner, rec.state.health))
    }

    /// The gecko in the town, run by Ann's game and by nobody else's, after it was reported.
    fn with_anns_gecko() -> Trio {
        let mut p = Trio::new();
        p.ann.game.place_actor(GECKO, TOWN, [10.0, 20.0, 0.0], 0.5);
        p.ben
            .game
            .place_actor(GECKO, TOWN, [900.0, 900.0, 0.0], 2.0);
        p.ben.game.set_near(GECKO, false);
        p.frames(60);
        p
    }
}

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    (0..3).all(|i| (a[i] - b[i]).abs() <= 0.1)
}

#[test]
fn an_actor_only_a_guest_runs_is_that_guests_and_the_others_hold_it() {
    let mut p = Trio::with_anns_gecko();
    // Ann's game runs the gecko and is its authority; the host's ledger says so.
    assert_eq!(p.ledger(GECKO), Some((Some(ANN), 100)));
    assert!(p.ann.owns(GECKO) && !p.ben.owns(GECKO));
    assert!(p.ann.game.actor_view(GECKO).ai);
    assert!(p.ann.fight().held().is_empty());
    // Ben's copy is held at Ann's state, as the host's actors are.
    let b = p.ben.game.actor_view(GECKO);
    assert!(!b.ai && close(b.pos, [10.0, 20.0, 0.0]), "{b:?}");
    assert!(p.ben.fight().held().contains(&GECKO));
    // It follows Ann's game: place and health.
    p.ann.game.move_actor(GECKO, [300.0, -40.0, 12.0], 1.5);
    p.ann.game.set_health(GECKO, 62.0);
    p.frames(6);
    let b = p.ben.game.actor_view(GECKO);
    assert!(close(b.pos, [300.0, -40.0, 12.0]), "{b:?}");
    assert_eq!(b.health, 62.0);
    assert_eq!(p.ledger(GECKO), Some((Some(ANN), 62)));
    // The host's own copy, which its game does not run, is left alone.
    assert_eq!(p.host.actor_view(GECKO).health, 100.0);
    assert_eq!(p.hd.fight.as_ref().unwrap().far.adopted, 1);
    p.ann.story_is_whole();
    p.ben.story_is_whole();
}

#[test]
fn two_guests_who_run_the_same_actor_get_one_owner() {
    let mut p = Trio::new();
    p.ann.game.place_actor(GECKO, TOWN, [10.0, 20.0, 0.0], 0.5);
    p.ben
        .game
        .place_actor(GECKO, TOWN, [900.0, 900.0, 0.0], 2.0);
    p.frames(60);
    let owner = p.ledger(GECKO).unwrap().0.unwrap();
    assert!(owner == ANN || owner == BEN);
    assert_ne!(p.ann.owns(GECKO), p.ben.owns(GECKO));
    // The one who is not the owner holds it where the owner has it.
    let (a, b) = (p.ann.game.actor_view(GECKO), p.ben.game.actor_view(GECKO));
    assert_ne!(a.ai, b.ai);
    assert!(close(a.pos, b.pos), "{a:?} {b:?}");
}

#[test]
fn another_players_hit_on_a_guests_actor_is_applied_by_its_owners_game() {
    let mut p = Trio::with_anns_gecko();
    // Ben shoots the gecko he holds: kept from his copy, judged by the host's ledger, applied
    // by Ann's game, and back in the state everyone gets.
    p.ben.game.hit(GECKO, PLAYER, 30.0);
    assert_eq!(p.ben.game.hits_run(), 0);
    p.frames(6);
    assert_eq!(p.ann.game.actor_view(GECKO).health, 70.0);
    assert_eq!(p.ben.game.actor_view(GECKO).health, 70.0);
    assert_eq!(p.ledger(GECKO), Some((Some(ANN), 70)));
    assert_eq!(p.ann.fight().far.hurts_applied, 1);
    // Not by the host's game, which does not run it.
    assert_eq!(p.host.actor_view(GECKO).health, 100.0);
    assert_eq!(p.hd.fight.as_ref().unwrap().hits_applied, 0);
    // Ann's own hit is her game's own, and everyone sees its result.
    p.ann.game.hit(GECKO, PLAYER, 20.0);
    assert_eq!(p.ann.game.hits_run(), 1);
    p.frames(6);
    assert_eq!(p.ben.game.actor_view(GECKO).health, 50.0);
    // A lethal hit of Ben's: Ann's game kills, and the death reaches Ben and the host's story.
    p.ben.game.hit(GECKO, PLAYER, 500.0);
    p.frames(10);
    assert_eq!(p.ann.game.dead(GECKO), Some(true));
    assert_eq!(p.ben.game.dead(GECKO), Some(true));
    assert_eq!(p.host.dead(GECKO), Some(true));
    assert!(p.hd.sync.live.actors.is_dead(FormId(GECKO)));
}

#[test]
fn a_death_in_the_owners_game_is_a_death_for_everyone() {
    let mut p = Trio::with_anns_gecko();
    p.ann.game.kill(GECKO);
    p.frames(10);
    assert_eq!(p.ben.game.dead(GECKO), Some(true));
    assert_eq!(p.host.dead(GECKO), Some(true));
    assert!(p.hd.sync.live.actors.is_dead(FormId(GECKO)));
    assert!(!p.ben.fight().held().contains(&GECKO));
}

#[test]
fn the_host_takes_an_actor_over_when_its_game_runs_it_and_gives_it_back() {
    let mut p = Trio::with_anns_gecko();
    // The host comes to the town and its game runs the gecko: the host is its authority now,
    // and both guests hold it at the host's state.
    p.host.move_player(TOWN, [0.0, 0.0, 0.0], 0.0);
    p.host.place_actor(GECKO, TOWN, [70.0, 80.0, 0.0], 1.0);
    p.frames(60);
    assert_eq!(p.ledger(GECKO).unwrap().0, Some(PlayerId::HOST));
    assert!(!p.ann.owns(GECKO));
    for g in [&mut p.ann, &mut p.ben] {
        let v = g.game.actor_view(GECKO);
        assert!(!v.ai && close(v.pos, [70.0, 80.0, 0.0]), "{v:?}");
    }
    // The host's game stops running it: Ann's game, which still does, is its authority again.
    p.host.set_near(GECKO, false);
    p.frames(60);
    assert_eq!(p.ledger(GECKO).unwrap().0, Some(ANN));
    assert!(p.ann.owns(GECKO));
    assert!(p.ann.game.actor_view(GECKO).ai);
    assert!(!p.ben.game.actor_view(GECKO).ai);
}

#[test]
fn an_actor_in_another_space_than_its_reporter_is_not_entered() {
    let mut p = Trio::new();
    // Ann's game has the raider near, but in the house, where Ann is not.
    p.ann.game.place_actor(RAIDER, HOUSE, [1.0, 2.0, 3.0], 0.0);
    p.frames(60);
    assert_eq!(p.ledger(RAIDER), None);
    assert!(p.ann.game.actor_view(RAIDER).ai);
    assert_eq!(p.ann.fight().far.reported, 0);
}

#[test]
fn a_held_actor_moves_between_two_states_in_steps() {
    let mut p = Trio::with_anns_gecko();
    p.ann.game.move_actor(GECKO, [110.0, 20.0, 0.0], 0.5);
    // Frame by frame Ben's copy goes there, never back, and not in one jump.
    let mut seen = Vec::new();
    for _ in 0..40 {
        p.frames_of(1, 10);
        seen.push(p.ben.game.actor_view(GECKO).pos[0]);
    }
    assert!(seen.windows(2).all(|w| w[0] <= w[1]), "{seen:?}");
    assert!(seen.iter().any(|x| *x > 10.5 && *x < 109.5), "{seen:?}");
    assert!((seen.last().unwrap() - 110.0).abs() <= 0.1, "{seen:?}");
    // Twin: with the steps off the copy is at the owner's place as soon as the state is here.
    let mut p = Trio::with_anns_gecko();
    p.ben.d.fight.as_mut().unwrap().smooth_on = false;
    p.ann.game.move_actor(GECKO, [110.0, 20.0, 0.0], 0.5);
    let mut seen = Vec::new();
    for _ in 0..40 {
        p.frames_of(1, 10);
        seen.push(p.ben.game.actor_view(GECKO).pos[0]);
    }
    assert!(
        seen.iter()
            .all(|x| (*x - 10.0).abs() <= 0.1 || (*x - 110.0).abs() <= 0.1),
        "{seen:?}"
    );
}
