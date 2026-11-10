//! Shared fights on two games in one process (docs/coop/SPECS.md S12): the host's game is the
//! authority for actors that are not players, and the story channel is untouched by it.

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

struct Pair {
    host: Story,
    hd: HostDriver,
    hnet: LoopbackEndpoint,
    guest: Story,
    gd: GuestDriver,
    gnet: LoopbackEndpoint,
    t: u64,
}

impl Pair {
    /// Two games with the whole ladder (or `ladder` on the guest) and fights up to `cap` (`None`:
    /// the guest runs without fights).
    fn with(ladder: Cap, cap: Option<fight::Cap>) -> Pair {
        let net = LoopbackNet::new();
        let mut host = Story::new(&spec());
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        hd.ladder = Some(Ladder::new(false, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
        hd.fight = Some(Fight::new(false, cap.unwrap_or(fight::Cap::parse(""))).quiet());
        hd.start(&mut host.e);
        let mut guest = Story::new(&spec());
        gate_guest(&mut guest.e);
        let mut gd = GuestDriver::new(PlayerId(1), hello());
        gd.ladder = Some(Ladder::new(true, ladder, DEFAULT_AVATAR_BASE).quiet());
        gd.fight = cap.map(|c| Fight::new(true, c).quiet());
        let mut p = Pair {
            hnet: net.endpoint(PlayerId::HOST),
            gnet: net.endpoint(PlayerId(1)),
            host,
            hd,
            guest,
            gd,
            t: 0,
        };
        p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
        p.guest.move_player(TOWN, [5.0, 5.0, 5.0], 0.0);
        p
    }

    fn new() -> Pair {
        Pair::with(Cap::Avatar, Some(fight::Cap::parse("")))
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            self.guest.run_quest_script();
            self.gd.step(&mut self.guest.e, &mut self.gnet, self.t);
        }
    }

    /// The story channel is as it must be whatever the fights did.
    fn story_is_whole(&mut self) {
        assert_eq!(echo(&mut self.guest.e, &self.gd.sync.store), vec![]);
        assert_eq!(self.gd.sync.desyncs_detected, 0);
        assert_eq!(self.gd.refused, 0);
        assert_eq!(
            self.guest.scripts_run(),
            0,
            "no story script ran on the guest"
        );
    }

    fn guest_fight(&self) -> &Fight {
        self.gd.fight.as_ref().unwrap()
    }
}

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    (0..3).all(|i| (a[i] - b[i]).abs() <= 0.1)
}

#[test]
fn the_hosts_actors_are_held_at_the_hosts_state_on_a_guest() {
    let mut p = Pair::new();
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.host.set_health(RAIDER, 62.0);
    p.frames(40);
    // The guest came to the host, and its copy of the actor is where the host's is, its AI off.
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert!((g.heading - 0.5).abs() < 0.01);
    assert_eq!((g.health, g.ai, h.ai), (62.0, false, true));
    assert!(p.guest_fight().held().contains(&RAIDER));
    // It follows: place, heading, health, and whether it fights.
    p.host.move_actor(RAIDER, [300.0, -40.0, 12.0], 1.5);
    p.host.set_health(RAIDER, 30.0);
    p.host.set_combat(RAIDER, 0x14);
    p.frames(5);
    let g = p.guest.actor_view(RAIDER);
    assert!(close(g.pos, [300.0, -40.0, 12.0]), "{g:?}");
    assert_eq!(g.health, 30.0);
    let seen = p.gd.sync.live.actors.get(FormId(RAIDER)).unwrap();
    let state = seen.state.unwrap();
    assert_eq!(seen.owner, Some(PlayerId::HOST));
    assert_ne!(state.flags & fight::FLAG_COMBAT, 0);
    assert_eq!(fight::target_player(state.target), Some(PlayerId::HOST));
    // Its own game switching its AI back on (a load does) is undone and counted.
    p.guest.set_near(RAIDER, true);
    assert_eq!(p.guest_fight().woke, 0);
    coop_engine::fight::release(&mut p.guest.e, RAIDER);
    p.frames(2);
    assert!(!p.guest.actor_view(RAIDER).ai);
    assert_eq!(p.guest_fight().woke, 1);
    // An actor only the guest's game has near is not the host's: it runs itself.
    p.guest.place_actor(GECKO, HOUSE, [1.0, 1.0, 1.0], 0.0);
    p.frames(5);
    assert!(p.guest.actor_view(GECKO).ai);
    assert_eq!(p.guest.actor_view(GECKO).pos, [1.0, 1.0, 1.0]);
    p.story_is_whole();
}

#[test]
fn without_fights_a_guest_runs_its_own_actors() {
    let mut p = Pair::with(Cap::Avatar, None);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    let g = p.guest.actor_view(RAIDER);
    assert_eq!((g.pos, g.ai, g.health), ([900.0, 900.0, 0.0], true, 100.0));
    p.story_is_whole();
}

#[test]
fn a_death_in_the_hosts_fight_is_a_death_on_the_guest_without_its_scripts() {
    let mut p = Pair::new();
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    p.host.kill(RAIDER);
    p.frames(5);
    assert_eq!(p.guest.dead(RAIDER), Some(true));
    // The corpse is not moved or held any more.
    assert!(!p.guest_fight().held().contains(&RAIDER));
    let at = p.guest.actor_view(RAIDER).pos;
    p.host.move_actor(RAIDER, [0.0, 0.0, 500.0], 0.0);
    p.frames(5);
    assert_eq!(p.guest.actor_view(RAIDER).pos, at);
    // Nothing went back to the host as a claim of a death it did not know.
    assert_eq!(p.hd.claims_enacted, 0);
    p.story_is_whole();
}

#[test]
fn an_actor_the_host_stops_running_goes_back_to_the_guests_game() {
    let mut p = Pair::new();
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    assert!(!p.guest.actor_view(RAIDER).ai);
    // It leaves the host's list of actors near its player (the host walked away from it).
    p.host.set_near(RAIDER, false);
    p.frames(5);
    assert!(p.guest.actor_view(RAIDER).ai);
    assert!(p.guest_fight().held().is_empty());
    assert_eq!(p.guest_fight().released, 1);
    // And is held again when the host runs it again.
    p.host.set_near(RAIDER, true);
    p.host.move_actor(RAIDER, [55.0, 66.0, 0.0], 0.0);
    p.frames(5);
    let g = p.guest.actor_view(RAIDER);
    assert!(!g.ai && close(g.pos, [55.0, 66.0, 0.0]), "{g:?}");
    // The session's end gives everything back.
    let mut f = p.gd.fight.take().unwrap();
    f.close(&mut p.guest.e);
    assert!(p.guest.actor_view(RAIDER).ai);
    p.story_is_whole();
}

#[test]
fn a_guest_in_another_space_keeps_its_own_actors() {
    // The ladder stops before the warp: the guest stays in the town, the host in the house.
    let mut p = Pair::with(Cap::Log, Some(fight::Cap::parse("")));
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    let g = p.guest.actor_view(RAIDER);
    assert_eq!((g.pos, g.ai), ([900.0, 900.0, 0.0], true));
    assert!(p.guest_fight().held().is_empty());
    p.story_is_whole();
}
