//! Every animation the other player plays, on its avatar (docs/coop/SPECS.md S39), on two games
//! in one process: groups, idles, jumps, speeds, sights, knock-downs, furniture and turns on the
//! spot travel both ways and are played with the game's own calls; the story channel is
//! untouched.

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::addr;
use coop_engine::testkit::{anim_kit, look_kit, Story, StorySpec};
use coop_engine_session::ladder::{Cap as LiveCap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::look::{Cap, Look};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const CHAIR: u32 = 0x0010_7200;
const WAVE: u32 = 0x0010_7201;
const PLAYER: u32 = 0x14;
/// Group types of the anim kit's own choosing: a hit reaction, an attack, an aim.
const HIT: u16 = 0x0045;
const ATTACK: u16 = 0x0025;
const AIM: u16 = 0x0035;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE],
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

fn story() -> Story {
    let mut s = Story::new(&spec());
    look_kit::install(&mut s);
    anim_kit::install(&mut s);
    for id in [CHAIR, WAVE] {
        s.add_look_form(id, 0x0c);
    }
    s.group_section(HIT as u8, 2);
    s
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
    fn new(cap: Option<Cap>) -> Pair {
        let net = LoopbackNet::new();
        let mut host = story();
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        let mut ladder = Ladder::new(false, LiveCap::Avatar, DEFAULT_AVATAR_BASE).quiet();
        ladder.look = cap.map(Look::new);
        hd.ladder = Some(ladder);
        hd.start(&mut host.e);
        let mut guest = story();
        gate_guest(&mut guest.e);
        let mut gd = GuestDriver::new(PlayerId(1), hello());
        let mut ladder = Ladder::new(true, LiveCap::Avatar, DEFAULT_AVATAR_BASE).quiet();
        ladder.look = cap.map(Look::new);
        gd.ladder = Some(ladder);
        host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.0);
        guest.move_player(HOUSE, [10.0, 20.0, 0.0], 0.5);
        Pair {
            hnet: net.endpoint(PlayerId::HOST),
            gnet: net.endpoint(PlayerId(1)),
            host,
            hd,
            guest,
            gd,
            t: 0,
        }
    }

    fn frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
            self.guest.run_quest_script();
            self.gd.step(&mut self.guest.e, &mut self.gnet, self.t);
        }
    }

    /// The guest's avatar of the host, and the host's of the guest.
    fn avatars(&self) -> (u32, u32) {
        let one = |l: &Ladder| match l.avatar_ids().as_slice() {
            [(_, id)] => *id,
            other => panic!("one avatar, not {other:?}"),
        };
        (
            one(self.gd.ladder.as_ref().unwrap()),
            one(self.hd.ladder.as_ref().unwrap()),
        )
    }

    fn guest_look(&self) -> &Look {
        self.gd.ladder.as_ref().unwrap().look.as_ref().unwrap()
    }

    fn story_untouched(&mut self) {
        assert!(echo(&mut self.guest.e, &self.gd.sync.store).is_empty());
        assert_eq!(self.gd.sync.desyncs_detected, 0);
        assert_eq!(self.gd.refused, 0);
    }
}

#[test]
fn the_groups_idles_and_jumps_a_player_starts_play_on_its_avatar_both_ways() {
    let mut p = Pair::new(Some(Cap::Face));
    p.frames(20);
    let (on_guest, on_host) = p.avatars();
    assert!(p.guest.shape(on_guest).plays.is_empty());
    // A hit reaction in a section of its own: played once, as the avatar's own group.
    p.host.start_group(PLAYER, 2, HIT);
    p.frames(10);
    assert_eq!(p.guest.shape(on_guest).plays, [HIT | anim_kit::WEAPON_BITS]);
    assert_eq!(p.guest.group_in(on_guest, 2), HIT | anim_kit::WEAPON_BITS);
    // Attacks, aims, draws and movement have their own ways: not sent as groups.
    p.host.start_group(PLAYER, 4, ATTACK);
    p.host.start_group(PLAYER, 5, AIM);
    p.host
        .start_group(PLAYER, 6, u16::from(addr::ANIM_GROUP_TYPE_DRAW));
    p.host
        .start_group(PLAYER, addr::ANIMATION_MOVEMENT_SECTION, HIT);
    p.frames(10);
    assert_eq!(p.guest.shape(on_guest).plays.len(), 1);
    // An idle: played once.
    p.host.start_idle(PLAYER, WAVE);
    p.frames(10);
    assert_eq!(p.guest.shape(on_guest).idles, [WAVE]);
    // A jump: the avatar's own jump.
    p.host.set_controller(PLAYER, addr::CONTROLLER_JUMPING);
    p.frames(5);
    p.host.set_controller(PLAYER, 2);
    p.frames(5);
    p.host.set_controller(PLAYER, 0);
    p.frames(5);
    assert_eq!(p.guest.shape(on_guest).jumps, 1);
    // And the other way.
    p.guest.start_group(PLAYER, 2, HIT);
    p.guest.start_idle(PLAYER, WAVE);
    p.guest.set_controller(PLAYER, addr::CONTROLLER_JUMPING);
    p.frames(10);
    let shape = p.host.shape(on_host);
    assert_eq!(shape.plays, [HIT | anim_kit::WEAPON_BITS]);
    assert_eq!((shape.idles.as_slice(), shape.jumps), (&[WAVE][..], 1));
    let c = p.guest_look().anim.counts;
    assert_eq!((c.groups, c.idles, c.jumps), (1, 1, 1));
    p.story_untouched();
}

#[test]
fn the_avatar_walks_at_its_players_speed_aims_and_is_knocked_down_with_it() {
    let mut p = Pair::new(Some(Cap::Face));
    p.frames(20);
    let (on_guest, _) = p.avatars();
    // The host's own speeds, from the start.
    let s = p.guest.shape(on_guest);
    assert_eq!((s.walk, s.run), (anim_kit::WALK, anim_kit::RUN));
    p.host.set_body(PLAYER, true, 95.0, 410.0, 0);
    p.frames(10);
    let s = p.guest.shape(on_guest);
    assert_eq!((s.walk, s.run), (95.0, 410.0));
    assert!(s.sights);
    // Knocked down: the avatar's fatigue goes below zero; up again: given back.
    p.host.set_body(PLAYER, false, 95.0, 410.0, 3);
    p.frames(10);
    let s = p.guest.shape(on_guest);
    assert!(!s.sights);
    assert_eq!(s.knock, anim_kit::KNOCKED);
    p.host.set_body(PLAYER, false, 95.0, 410.0, 0);
    p.frames(10);
    let s = p.guest.shape(on_guest);
    assert_eq!((s.knock, s.fatigue), (0, anim_kit::FATIGUE));
    assert_eq!(p.guest_look().anim.counts.knocks, 2);
    p.story_untouched();
}

#[test]
fn the_avatar_sits_on_its_players_furniture_and_gets_up_with_it() {
    let mut p = Pair::new(Some(Cap::Face));
    p.frames(20);
    let (on_guest, _) = p.avatars();
    p.host.set_furniture(PLAYER, CHAIR);
    p.frames(10);
    let s = p.guest.shape(on_guest);
    assert_eq!((s.sits.as_slice(), s.furniture), (&[CHAIR][..], CHAIR));
    // Once, while it sits.
    p.frames(10);
    assert_eq!(p.guest.shape(on_guest).sits.len(), 1);
    p.host.set_furniture(PLAYER, 0);
    p.frames(10);
    let s = p.guest.shape(on_guest);
    assert_eq!((s.get_ups, s.furniture), (1, 0));
    // Standing again, its gait is forced again.
    assert_eq!(p.guest.forced_moves(on_guest), Some(0));
    p.story_untouched();
}

#[test]
fn a_player_turning_while_standing_turns_its_avatar_on_the_spot() {
    let mut p = Pair::new(Some(Cap::Walk));
    p.frames(20);
    let (on_guest, _) = p.avatars();
    assert_eq!(p.guest.forced_moves(on_guest), Some(0));
    p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 1.6);
    p.frames(3);
    assert_eq!(
        p.guest.forced_moves(on_guest),
        Some(addr::MOVE_TURN_LEFT),
        "a rising heading turns left"
    );
    p.frames(20);
    assert_eq!(p.guest.forced_moves(on_guest), Some(0));
    p.host.move_player(HOUSE, [100.0, 200.0, 0.0], 0.6);
    p.frames(3);
    assert_eq!(p.guest.forced_moves(on_guest), Some(addr::MOVE_TURN_RIGHT));
    assert!(p.guest_look().anim.counts.turns >= 2);
    p.story_untouched();
}

#[test]
fn without_the_look_no_animation_is_sent_or_played() {
    let mut p = Pair::new(None);
    p.frames(20);
    let (on_guest, _) = p.avatars();
    p.host.start_group(PLAYER, 2, HIT);
    p.host.start_idle(PLAYER, WAVE);
    p.host.set_controller(PLAYER, addr::CONTROLLER_JUMPING);
    p.host.set_furniture(PLAYER, CHAIR);
    p.frames(20);
    let s = p.guest.shape(on_guest);
    assert!(s.plays.is_empty() && s.idles.is_empty() && s.sits.is_empty());
    assert_eq!(s.jumps, 0);
    assert_eq!((s.walk, s.run), (anim_kit::WALK, anim_kit::RUN));
}

#[test]
fn the_look_is_on_unless_asked_off() {
    assert_eq!(Cap::parse(""), Some(Cap::Face));
    assert_eq!(Cap::parse("off"), None);
}
