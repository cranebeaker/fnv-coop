//! The other player's avatar as that player (docs/coop/SPECS.md S35), on two games in one
//! process: it walks with its player's gait, holds, draws and fires its weapon, wears its items
//! and has its face and name; the story channel is untouched.

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::addr;
use coop_engine::look::{self, Face};
use coop_engine::testkit::{look_kit, Story, StorySpec};
use coop_engine_session::ladder::{Cap as LiveCap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::look::{Cap, Look};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const RIFLE: u32 = 0x0010_7001;
const PISTOL: u32 = 0x0010_7002;
const ROUNDS: u32 = 0x0010_7003;
const SUIT: u32 = 0x0010_7004;
const HAT: u32 = 0x0010_7005;
const RACE: u32 = 0x0010_7100;
const OTHER_RACE: u32 = 0x0010_7106;
const HAIR: u32 = 0x0010_7101;
const EYES: u32 = 0x0010_7102;
const NOSE: u32 = 0x0010_7103;
const BROW: u32 = 0x0010_7104;
const PLAYER: u32 = 0x14;
const RUN_FORWARD: u16 = 0x201;
const SNEAK_FORWARD: u16 = 0x501;

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

/// A story with the look kit and the same items and look forms in every game.
fn story() -> Story {
    let mut s = Story::new(&spec());
    look_kit::install(&mut s);
    for (id, kind) in [
        (RIFLE, addr::FORM_TYPE_WEAPON),
        (PISTOL, addr::FORM_TYPE_WEAPON),
        (ROUNDS, 0x29),
        (SUIT, 0x18),
        (HAT, 0x18),
    ] {
        s.add_look_item(id, kind);
    }
    for id in [RACE, OTHER_RACE, HAIR, EYES, NOSE, BROW] {
        s.add_look_form(id, 0x0c);
    }
    s.set_look_ammo(RIFLE, ROUNDS);
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

    fn host_look(&self) -> &Look {
        self.hd.ladder.as_ref().unwrap().look.as_ref().unwrap()
    }

    fn guest_look(&self) -> &Look {
        self.gd.ladder.as_ref().unwrap().look.as_ref().unwrap()
    }

    /// The story channel saw none of it.
    fn story_untouched(&mut self) {
        assert!(echo(&mut self.guest.e, &self.gd.sync.store).is_empty());
        assert_eq!(self.gd.sync.desyncs_detected, 0);
        assert_eq!(self.gd.refused, 0);
    }
}

#[test]
fn an_avatar_walks_runs_sneaks_and_stands_with_its_players_flags() {
    let mut p = Pair::new(Some(Cap::Walk));
    p.frames(20);
    let (on_guest, on_host) = p.avatars();
    // Its AI on (the game animates it), standing.
    let g = p.guest.placed();
    assert!(g[0].3, "the guest's avatar of the host has its AI on");
    assert_eq!(p.guest.forced_moves(on_guest), Some(0));
    // The host runs: the guest's avatar of it runs, along its path.
    p.host.set_moves(PLAYER, RUN_FORWARD);
    p.host.move_player(HOUSE, [300.0, 200.0, 0.0], 1.0);
    p.frames(10);
    assert_eq!(p.guest.forced_moves(on_guest), Some(RUN_FORWARD));
    let at = p.guest.placed()[0].2;
    assert!((at[0] - 300.0).abs() < 0.1, "{at:?}");
    // It sneaks, then stands: the idle.
    p.host.set_moves(PLAYER, SNEAK_FORWARD);
    p.frames(10);
    assert_eq!(p.guest.forced_moves(on_guest), Some(SNEAK_FORWARD));
    p.host.set_moves(PLAYER, 0);
    p.frames(10);
    assert_eq!(p.guest.forced_moves(on_guest), Some(0));
    // And the other way.
    p.guest.set_moves(PLAYER, RUN_FORWARD);
    p.frames(10);
    assert_eq!(p.host.forced_moves(on_host), Some(RUN_FORWARD));
    // The game is asked only when the gait changes.
    let gaits = p.guest_look().counts.gaits;
    p.frames(10);
    assert_eq!(p.guest_look().counts.gaits, gaits);
    // A fight its own AI starts is ended.
    p.guest.set_combat(on_guest, PLAYER);
    p.frames(1);
    assert!(!p.guest.actor_view(on_guest).in_combat);
    assert_eq!(p.guest_look().counts.calmed, 1);
    // Its AI was never put back off.
    let status = p.gd.ladder.as_ref().unwrap().status();
    assert!(status.contains("woke 0"), "{status}");
    p.story_untouched();
}

#[test]
fn without_the_look_an_avatar_slides_with_its_ai_off_as_before() {
    let mut p = Pair::new(None);
    p.host.set_moves(PLAYER, RUN_FORWARD);
    p.frames(20);
    let (on_guest, _) = p.avatars();
    assert!(!p.guest.placed()[0].3);
    assert_eq!(p.guest.forced_moves(on_guest), None);
    assert_eq!(look::base_of(&mut p.guest.e, on_guest), Some(7));
}

#[test]
fn the_weapon_is_in_hand_drawn_as_the_players_and_its_attacks_and_reloads_played() {
    let mut p = Pair::new(Some(Cap::Pose));
    p.host.look_give(PLAYER, RIFLE, 1, true);
    p.host.look_give(PLAYER, ROUNDS, 30, true);
    p.frames(20);
    let (on_guest, on_host) = p.avatars();
    // The rifle in hand (and rounds for it), put away as the host's is.
    assert_eq!(p.guest.look_wearing(on_guest), [RIFLE]);
    assert_eq!(p.guest.look_count(on_guest, ROUNDS), look::AMMO_FILL);
    assert!(!p.guest.look_gear(on_guest).drawn);
    // Drawn.
    p.host.set_look_drawn(PLAYER, true);
    p.frames(10);
    assert!(p.guest.look_gear(on_guest).drawn);
    // Each attack the host's player starts is played once, with its group.
    assert!(p.host.player_attack(0x20));
    p.frames(10);
    assert_eq!(p.guest.attacks_queued(on_guest), 1);
    assert_eq!(p.guest.queued_group(on_guest), 0x20);
    p.guest.finish_attack(on_guest);
    p.frames(10);
    assert_eq!(p.guest.attacks_queued(on_guest), 1, "once");
    // An attack the game did not start is no attack.
    p.host.refuse_player_attacks(true);
    assert!(!p.host.player_attack(0x21));
    p.frames(10);
    assert_eq!(p.guest.attacks_queued(on_guest), 1);
    // A reload, once.
    p.host.set_look_action(PLAYER, addr::ANIM_ACTION_RELOAD);
    p.frames(10);
    assert_eq!(p.guest.look_gear(on_guest).reloads, 1);
    p.host.set_look_action(PLAYER, 0);
    // Another weapon: in the avatar's hand.
    p.host.look_give(PLAYER, PISTOL, 1, true);
    p.host.look_take_off(PLAYER, RIFLE);
    p.frames(10);
    assert_eq!(p.guest.look_wearing(on_guest), [PISTOL]);
    // Put away.
    p.host.set_look_drawn(PLAYER, false);
    p.frames(10);
    assert!(!p.guest.look_gear(on_guest).drawn);
    // The guest's attacks reach the host's avatar of the guest too.
    p.guest.look_give(PLAYER, RIFLE, 1, true);
    p.guest.set_look_drawn(PLAYER, true);
    p.frames(10);
    assert!(p.guest.player_attack(0x22));
    p.frames(10);
    assert_eq!(p.host.attacks_queued(on_host), 1);
    assert_eq!(p.host.queued_group(on_host), 0x22);
    assert_eq!(
        p.host_look().counts.actions_sent,
        2,
        "the host's attack and reload"
    );
    p.story_untouched();
}

#[test]
fn an_attack_from_before_the_guest_was_in_is_not_played() {
    let mut p = Pair::new(Some(Cap::Pose));
    p.guest.look_give(PLAYER, RIFLE, 1, true);
    p.guest.set_look_drawn(PLAYER, true);
    // Before the host took the guest in (its first frame), and once in, before its avatar is.
    p.frames(1);
    assert!(p.guest.player_attack(0x20));
    p.frames(40);
    let (_, on_host) = p.avatars();
    assert_eq!(p.host.attacks_queued(on_host), 0);
    assert_eq!(p.guest_look().counts.actions_sent, 0);
}

#[test]
fn the_avatar_wears_what_its_player_wears_and_changes_with_it() {
    let mut p = Pair::new(Some(Cap::Gear));
    p.host.look_give(PLAYER, SUIT, 1, true);
    p.host.look_give(PLAYER, HAT, 1, true);
    p.host.look_give(PLAYER, RIFLE, 1, true);
    p.frames(40);
    let (on_guest, _) = p.avatars();
    assert_eq!(p.guest.look_wearing(on_guest), [SUIT, HAT, RIFLE]);
    // The host takes the hat off: so does its avatar, within a second or two.
    p.host.look_take_off(PLAYER, HAT);
    p.frames(40);
    assert_eq!(p.guest.look_wearing(on_guest), [SUIT, RIFLE]);
    // Its parts were sent only when they changed.
    assert_eq!(p.host_look().counts.parts_sent, 2);
    // Something the avatar's own base puts on it is taken off too.
    p.guest.look_give(on_guest, PISTOL, 1, true);
    p.frames(40);
    assert_eq!(p.guest.look_wearing(on_guest), [SUIT, RIFLE]);
    p.story_untouched();
}

fn face(name: &str) -> Face {
    Face {
        female: true,
        race: RACE,
        hair: HAIR,
        hair_length: 0.25,
        eyes: EYES,
        hair_color: 0x0040_2010,
        height: 1.0,
        weight: 0.5,
        head_parts: vec![NOSE, BROW],
        coord: [
            (3, 1, vec![0.5, -1.25, 2.0]),
            (2, 1, vec![0.125, -0.75]),
            (1, 2, vec![3.5, -2.5]),
            (0, 0, Vec::new()),
        ],
        name: name.into(),
    }
}

#[test]
fn the_avatar_has_its_players_face_body_and_name() {
    let mut p = Pair::new(Some(Cap::Face));
    let mine = Face {
        female: false,
        race: OTHER_RACE,
        head_parts: vec![BROW],
        name: "Host Courier".into(),
        ..face("")
    };
    assert!(look::set_face(&mut p.host.e, 7, &mine));
    let theirs = face("Guest Courier");
    assert!(look::set_face(&mut p.guest.e, 7, &theirs));
    p.frames(40);
    let (on_guest, on_host) = p.avatars();
    // The host's avatar of the guest is placed from a base of its own with the guest's look.
    let base = look::base_of(&mut p.host.e, on_host).unwrap();
    assert_ne!(base, 7);
    assert_eq!(look::face_of_id(&mut p.host.e, base), Some(theirs.clone()));
    assert_eq!(p.host.face_npc(base), 0);
    // And the guest's of the host with the host's.
    let base = look::base_of(&mut p.guest.e, on_guest).unwrap();
    assert_eq!(look::face_of_id(&mut p.guest.e, base), Some(mine.clone()));
    // Each game's own player is as it was.
    assert_eq!(look::player_face(&mut p.host.e), Some(mine));
    assert_eq!(look::player_face(&mut p.guest.e), Some(theirs));
    assert_eq!((p.host.npcs_made(), p.guest.npcs_made()), (1, 1));
    // The guest changes its name: the host places its avatar anew, from a new base.
    assert!(look::set_face(&mut p.guest.e, 7, &face("Second Name")));
    p.frames(40);
    let (_, again) = p.avatars();
    assert_ne!(again, on_host);
    let base = look::base_of(&mut p.host.e, again).unwrap();
    assert_eq!(
        look::face_of_id(&mut p.host.e, base).map(|f| f.name),
        Some("Second Name".into())
    );
    assert_eq!(p.host.npcs_made(), 2);
    // Nothing more is made while the look stays.
    p.frames(40);
    assert_eq!(p.host.npcs_made(), 2);
    assert_eq!(p.host_look().counts.bases, 2);
    p.story_untouched();
}

#[test]
fn below_the_face_cap_no_base_is_made() {
    let mut p = Pair::new(Some(Cap::Gear));
    assert!(look::set_face(&mut p.guest.e, 7, &face("Guest Courier")));
    p.frames(40);
    let (_, on_host) = p.avatars();
    assert_eq!(look::base_of(&mut p.host.e, on_host), Some(7));
    assert_eq!(p.host.npcs_made(), 0);
}
