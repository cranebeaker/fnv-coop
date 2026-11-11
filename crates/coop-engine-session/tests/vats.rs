//! V.A.T.S. with two players and damage outside `Actor::HitMe`, on two games in one process
//! (docs/coop/SPECS.md S20, S21). The harness is the shared fights' (`tests/fights.rs`).

use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine_session::fight::{self, Fight};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::vats::{Mode, Share};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const TOWN: u32 = 0x0010_5a01;
const RAIDER: u32 = 0x0010_4f25;
const GECKO: u32 = 0x0010_4f26;
const PLAYER: u32 = 0x14;

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

/// Two games in the same house, the raider placed and held, after the ladder's first frames.
fn in_a_fight(cap: &str) -> Pair {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse(cap)));
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    assert!(p.guest_fight().held().contains(&RAIDER));
    p
}

impl Pair {
    /// The guest's avatar in the host's game.
    fn guest_avatar(&self) -> u32 {
        self.hd.ladder.as_ref().unwrap().avatar_ids()[0].1
    }

    fn host_fight(&self) -> &Fight {
        self.hd.fight.as_ref().unwrap()
    }

    /// Frames of the host's game only (nobody has joined).
    fn host_frames(&mut self, n: u32) {
        for _ in 0..n {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hnet, self.t);
        }
    }
}

const VATS_MENU: u32 = 0x420;

#[test]
fn alone_a_players_vats_stops_and_slows_the_game_as_it_always_did() {
    let mut p = Pair::new();
    // Nobody has joined: the host's driver runs, the guest's does not.
    p.host_frames(5);
    p.host.open_menu(VATS_MENU);
    p.host.set_vats_mode(2);
    p.host_frames(2);
    assert!(p.host.in_menu_mode());
    p.host.close_menu();
    p.host.set_vats_mode(4);
    p.host.ask_time_multiplier(0.25);
    p.host_frames(2);
    assert_eq!(p.host.time_multiplier(), 0.25);
    assert_eq!(p.host_fight().vats.entered, 0);
    assert!(p.hd.sync.live.combat.vats.active().next().is_none());
}

#[test]
fn with_a_guest_the_hosts_vats_neither_stops_nor_slows_the_world() {
    let mut p = in_a_fight("");
    // The host picks its targets: its game runs on behind the menu, so the actors everyone
    // sees keep moving, and the guest is told.
    p.host.open_menu(VATS_MENU);
    p.host.set_vats_mode(2);
    p.frames(2);
    assert!(!p.host.in_menu_mode());
    assert!(p.gd.sync.live.combat.vats_active.contains(&PlayerId::HOST));
    p.host.move_actor(RAIDER, [300.0, -40.0, 12.0], 1.5);
    p.frames(3);
    assert!(close(p.guest.actor_view(RAIDER).pos, [300.0, -40.0, 12.0]));
    // Any other menu still stops the host's game.
    p.host.open_menu(0x3ea);
    assert!(p.host.in_menu_mode());
    // The shots play: no slow motion for the world.
    p.host.close_menu();
    p.host.set_vats_mode(4);
    p.host.ask_time_multiplier(0.25);
    assert_eq!(p.host.time_multiplier(), 1.0);
    // V.A.T.S. ends: the guest is told.
    p.host.set_vats_mode(0);
    p.host.ask_time_multiplier(1.0);
    p.frames(2);
    assert!(p.gd.sync.live.combat.vats_active.is_empty());
    assert_eq!(p.host_fight().vats.entered, 1);
    p.story_is_whole();
}

#[test]
fn a_guest_in_vats_is_in_real_time_and_its_shots_go_through_the_host() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    p.guest.open_menu(VATS_MENU);
    p.guest.set_vats_mode(2);
    p.frames(2);
    // The guest's own game does not stop either (it may run actors the others see), and the
    // host knows who is in V.A.T.S.
    assert!(!p.guest.in_menu_mode());
    let active: Vec<PlayerId> = p.hd.sync.live.combat.vats.active().copied().collect();
    assert_eq!(active, vec![PlayerId(1)]);
    // While the guest picks, the host's actors go on: the raider moves and hits the guest's
    // avatar, and the guest's player takes it in full, at once.
    p.host.move_actor(RAIDER, [50.0, 60.0, 0.0], 0.0);
    p.host.hit(avatar, RAIDER, 25.0);
    p.frames(3);
    assert!(close(p.guest.actor_view(RAIDER).pos, [50.0, 60.0, 0.0]));
    let health = p.guest.actor_view(PLAYER).health;
    assert!((70.0..=75.0).contains(&health), "{health}");
    // The queued shot is played by the guest's game as an attack: kept from its copy, judged
    // by the host's ledger, applied by the host's game, with no slow motion on the way.
    p.guest.close_menu();
    p.guest.set_vats_mode(4);
    p.guest.ask_time_multiplier(0.125);
    assert_eq!(p.guest.time_multiplier(), 1.0);
    p.guest.hit(RAIDER, PLAYER, 30.0);
    assert_eq!(p.guest.hits_run(), 0);
    p.frames(3);
    let h = p.host.actor_view(RAIDER);
    assert_eq!((h.health, h.attacker), (70.0, avatar));
    assert_eq!(p.guest.actor_view(RAIDER).health, 70.0);
    p.guest.set_vats_mode(0);
    p.frames(2);
    assert!(p.hd.sync.live.combat.vats.active().next().is_none());
    p.story_is_whole();
}

#[test]
fn twin_with_vats_not_shared_the_hosts_menu_stops_its_game() {
    let mut p = Pair::new();
    p.hd.fight.as_mut().unwrap().vats = Share::new(Mode::Off);
    p.frames(40);
    p.host.open_menu(VATS_MENU);
    p.host.set_vats_mode(2);
    p.frames(2);
    assert!(p.host.in_menu_mode());
    assert!(p.gd.sync.live.combat.vats_active.is_empty());
    // Sharing the speed only: no slow motion, but the menu stops the game.
    let mut p = Pair::new();
    p.hd.fight.as_mut().unwrap().vats = Share::new(Mode::parse("slow"));
    p.frames(40);
    p.host.open_menu(VATS_MENU);
    p.host.set_vats_mode(2);
    p.frames(2);
    assert!(p.host.in_menu_mode());
    p.host.set_vats_mode(4);
    p.host.ask_time_multiplier(0.25);
    assert_eq!(p.host.time_multiplier(), 1.0);
}

#[test]
fn the_sessions_end_gives_vats_back_to_the_game() {
    let mut p = in_a_fight("");
    p.guest.open_menu(VATS_MENU);
    p.guest.set_vats_mode(2);
    p.frames(2);
    assert!(!p.guest.in_menu_mode());
    let mut f = p.gd.fight.take().unwrap();
    f.close(&mut p.guest.e);
    assert!(p.guest.in_menu_mode());
    p.guest.ask_time_multiplier(0.25);
    assert_eq!(p.guest.time_multiplier(), 0.25);
}

#[test]
fn a_held_actors_loss_outside_a_hit_is_the_hosts_to_decide() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    // In the guest's game the held raider falls and burns from the gecko's fire: neither
    // touches the guest's copy, and neither is a claim.
    p.guest.fall(RAIDER, 30.0);
    p.guest.burn(RAIDER, GECKO, 20.0);
    p.frames(4);
    assert_eq!(p.guest.actor_view(RAIDER).health, 100.0);
    assert_eq!(p.host.actor_view(RAIDER).health, 100.0);
    // The guest's player's fire on it, a little each frame: the host's game applies it, whole
    // points at a time, credited to the guest's avatar, and it comes back in the actor's state.
    for _ in 0..10 {
        p.guest.burn(RAIDER, PLAYER, 0.4);
        p.frames(1);
    }
    p.frames(4);
    let h = p.host.actor_view(RAIDER);
    assert_eq!((h.health, h.attacker), (96.0, avatar));
    assert_eq!(p.guest.actor_view(RAIDER).health, 96.0);
    assert!(p.guest_fight().harms_sent >= 1);
    // The same in the host's game is the host's game's own.
    p.host.fall(RAIDER, 6.0);
    p.frames(3);
    assert_eq!(p.host.actor_view(RAIDER).health, 90.0);
    assert_eq!(p.guest.actor_view(RAIDER).health, 90.0);
    p.story_is_whole();
}

#[test]
fn an_effect_on_a_guests_avatar_reaches_the_guests_player() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    // The raider's fire burns the guest's avatar in the host's game: kept from the avatar,
    // taken by the guest's own player.
    for _ in 0..5 {
        p.host.burn(avatar, RAIDER, 5.0);
        p.frames(1);
    }
    p.frames(4);
    assert_eq!(p.guest.actor_view(PLAYER).health, 75.0);
    assert_eq!(p.guest.actor_view(PLAYER).attacker, RAIDER);
    // In the guest's game the held raider's fire on the guest's player does nothing (it came
    // from the host), and the player's own fall is the guest's game's own.
    p.guest.burn(PLAYER, RAIDER, 50.0);
    p.guest.fall(PLAYER, 5.0);
    assert_eq!(p.guest.actor_view(PLAYER).health, 70.0);
    p.story_is_whole();
}

#[test]
fn twin_with_state_only_every_game_applies_its_own_losses() {
    let mut p = in_a_fight("state");
    p.guest.fall(RAIDER, 30.0);
    // The guest's game applied it.
    assert_eq!(p.guest.actor_view(RAIDER).health, 70.0);
    p.frames(3);
    assert_eq!(p.host.actor_view(RAIDER).health, 100.0);
    assert_eq!(p.guest_fight().harms_sent, 0);
}
