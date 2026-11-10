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
}

#[test]
fn a_guests_hit_is_applied_by_the_hosts_game_and_comes_back() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    p.guest.hit(RAIDER, PLAYER, 30.0);
    // The guest's own game did not apply it.
    assert_eq!(p.guest.hits_run(), 0);
    assert_eq!(p.guest.actor_view(RAIDER).health, 100.0);
    p.frames(3);
    let h = p.host.actor_view(RAIDER);
    assert_eq!((h.health, h.attacker), (70.0, avatar));
    assert_eq!(p.guest.actor_view(RAIDER).health, 70.0);
    assert_eq!(
        (p.host_fight().hits_applied, p.guest_fight().hits_sent),
        (1, 1)
    );
    // What the guest's held actors do to each other is not the guest's game's to apply.
    p.guest.hit(RAIDER, GECKO, 20.0);
    p.frames(3);
    assert_eq!(p.guest.hits_run(), 0);
    assert_eq!(p.host.actor_view(RAIDER).health, 70.0);
    // A lethal hit: the host's game kills, credited to the guest's avatar, and the death
    // reaches the guest without its scripts.
    let ran = p.host.scripts_run();
    p.guest.hit(RAIDER, PLAYER, 500.0);
    p.frames(5);
    assert_eq!(p.host.dead(RAIDER), Some(true));
    assert_eq!(p.guest.dead(RAIDER), Some(true));
    assert_eq!(p.host_fight().kills, 1);
    assert_eq!(
        p.host.scripts_run(),
        ran + 1,
        "the death's script ran on the host, once"
    );
    // A hit on the corpse is refused by the ledger.
    p.guest.hit(RAIDER, PLAYER, 10.0);
    p.frames(3);
    assert_eq!(p.host_fight().hits_applied, 2);
    p.story_is_whole();
}

#[test]
fn an_essential_actor_survives_a_guests_lethal_hit() {
    let mut p = in_a_fight("");
    p.host.set_essential(RAIDER, true);
    p.frames(2);
    p.guest.hit(RAIDER, PLAYER, 500.0);
    p.frames(5);
    assert_eq!(p.host.dead(RAIDER), Some(false));
    assert_eq!(p.guest.dead(RAIDER), Some(false));
    assert_eq!(p.host_fight().kills, 0);
    p.story_is_whole();
}

#[test]
fn the_hosts_actors_hurt_a_guest_through_the_host() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    // The raider hits the guest's avatar in the host's game: kept from the avatar.
    p.host.hit(avatar, RAIDER, 25.0);
    assert_eq!(p.host.hits_run(), 0);
    p.frames(3);
    // The guest's own player took it (with whatever the party's scaling adds).
    let health = p.guest.actor_view(PLAYER).health;
    assert!((70.0..=75.0).contains(&health), "{health}");
    assert_eq!(p.guest.actor_view(PLAYER).attacker, RAIDER);
    assert_eq!(
        (p.host_fight().hurts_sent, p.guest_fight().hurts_taken),
        (1, 1)
    );
    // The host knows the guest's health from then on.
    let vitals = p.hd.sync.live.combat.vitals.get(PlayerId(1)).unwrap();
    assert_eq!(f32::from(vitals.hp), health.ceil());
    // The held raider's own swing at the guest's player, in the guest's game, does nothing.
    p.guest.hit(PLAYER, RAIDER, 50.0);
    assert_eq!(p.guest.hits_run(), 0);
    assert_eq!(p.guest.actor_view(PLAYER).health, health);
    // Anything else that hits the guest's player is the guest's game's own (a trap, a fall).
    p.guest.hit(PLAYER, 0, 5.0);
    assert_eq!(p.guest.hits_run(), 1);
    // The host's player hitting the avatar: friendly fire is off, nothing arrives.
    p.host.hit(avatar, PLAYER, 40.0);
    p.frames(3);
    assert_eq!(p.guest_fight().hurts_taken, 1);
    // The host's own fights are its game's.
    p.host.hit(RAIDER, PLAYER, 10.0);
    assert_eq!(p.host.hits_run(), 1);
    p.story_is_whole();
}

#[test]
fn with_state_only_every_game_applies_its_own_hits() {
    let mut p = in_a_fight("state");
    p.guest.hit(RAIDER, PLAYER, 30.0);
    assert_eq!(p.guest.hits_run(), 1);
    p.frames(3);
    assert_eq!(p.host.actor_view(RAIDER).health, 100.0);
    assert_eq!(p.host_fight().hits_applied, 0);
    p.story_is_whole();
}

#[test]
fn an_enemy_turns_on_the_guest_that_hurts_it() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    // Fights on: the avatar can be fought.
    assert!(!p.host.ghost(avatar));
    p.host.set_health(RAIDER, 2000.0);
    p.host.set_combat(RAIDER, PLAYER);
    // The gecko fights the raider, not a player: nobody redirects it.
    p.host.place_actor(GECKO, HOUSE, [50.0, 20.0, 0.0], 0.0);
    p.host.set_combat(GECKO, RAIDER);
    p.frames(20);
    assert_eq!(p.host.actor_view(RAIDER).target, PLAYER);
    for _ in 0..6 {
        p.guest.hit(RAIDER, PLAYER, 200.0);
        p.frames(2);
    }
    // The director holds a target for a while before it switches.
    p.frames(80);
    assert_eq!(p.host.actor_view(RAIDER).target, avatar);
    assert_eq!(p.host_fight().retargets, 1);
    assert_eq!(p.host.combats_started(), 1);
    assert_eq!(p.host.actor_view(GECKO).target, RAIDER);
    // The guest is told who the raider is after.
    let state = p.gd.sync.live.actors.get(FormId(RAIDER)).unwrap().state;
    assert_eq!(
        fight::target_player(state.unwrap().target),
        Some(PlayerId(1))
    );
    p.story_is_whole();
}

#[test]
fn without_the_threat_slice_avatars_stay_ghosts_and_nobody_is_redirected() {
    let mut p = in_a_fight("damage");
    let avatar = p.guest_avatar();
    assert!(p.host.ghost(avatar));
    p.host.set_health(RAIDER, 2000.0);
    p.host.set_combat(RAIDER, PLAYER);
    for _ in 0..6 {
        p.guest.hit(RAIDER, PLAYER, 200.0);
        p.frames(2);
    }
    p.frames(80);
    assert_eq!(p.host.actor_view(RAIDER).target, PLAYER);
    assert_eq!(p.host.combats_started(), 0);
    p.story_is_whole();
}

/// Every control of the player disabled.
const NO_CONTROLS: u8 = 0x7f;

#[test]
fn a_guest_who_would_die_is_down_and_the_host_next_to_them_revives_them() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    p.host.hit(avatar, RAIDER, 500.0);
    p.frames(3);
    // Not dead: down, with 1 health and no controls, and everyone knows.
    assert!(!p.guest.player_dead());
    assert_eq!(p.guest.actor_view(PLAYER).health, 1.0);
    assert_eq!(p.guest.controls(), NO_CONTROLS);
    assert!(p.guest_fight().down());
    assert!(p.hd.sync.live.rules.revive.is_downed(PlayerId(1)));
    // A downed player is out of the fight: more hits on the avatar do nothing.
    let taken = p.guest_fight().hurts_taken;
    p.host.hit(avatar, RAIDER, 50.0);
    p.frames(3);
    assert_eq!(p.guest_fight().hurts_taken, taken);
    // The host stands where the guest came to: after the reviver's hold the guest is up, at its
    // share of the health it had.
    p.frames(70);
    assert!(!p.guest_fight().down());
    assert_eq!(p.guest.actor_view(PLAYER).health, 30.0);
    assert_eq!(p.guest.controls(), 0);
    assert_eq!((p.guest_fight().downs, p.guest_fight().ups), (1, 1));
    assert!(!p.hd.sync.live.rules.revive.is_downed(PlayerId(1)));
    p.story_is_whole();
}

#[test]
fn a_host_who_would_die_is_revived_by_the_guest_next_to_them() {
    let mut p = in_a_fight("");
    // The raider kills the host's player in the host's own game: held.
    p.host.hit(PLAYER, RAIDER, 500.0);
    p.frames(3);
    assert!(!p.host.player_dead());
    assert!(p.host_fight().down());
    assert_eq!(p.host.controls(), NO_CONTROLS);
    assert!(p.gd.sync.live.rules.downed.contains(&PlayerId::HOST));
    p.frames(70);
    assert!(!p.host_fight().down());
    assert_eq!(p.host.actor_view(PLAYER).health, 30.0);
    assert_eq!(p.host.controls(), 0);
    p.story_is_whole();
}

#[test]
fn a_downed_player_nobody_comes_to_is_up_again_when_the_window_ends() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    // The host walks far off in the same house.
    p.host.move_player(HOUSE, [9000.0, 9000.0, 0.0], 0.0);
    p.frames(5);
    p.host.hit(avatar, RAIDER, 500.0);
    p.frames(100);
    assert!(p.guest_fight().down());
    assert_eq!(p.guest.controls(), NO_CONTROLS);
    // The reviver's window is a minute.
    p.frames(1200);
    assert!(!p.guest_fight().down());
    assert_eq!(p.guest.actor_view(PLAYER).health, 100.0);
    assert_eq!(p.guest.controls(), 0);
    assert!(!p.guest.player_dead());
    p.story_is_whole();
}

#[test]
fn with_everyone_down_every_player_dies_as_the_game_has_it() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    p.host.hit(avatar, RAIDER, 500.0);
    p.frames(3);
    assert!(p.guest_fight().down());
    p.host.hit(PLAYER, RAIDER, 500.0);
    p.frames(5);
    assert!(p.host.player_dead());
    assert!(p.guest.player_dead());
    assert_eq!((p.host_fight().wipes, p.guest_fight().wipes), (1, 1));
    assert_eq!((p.host.controls(), p.guest.controls()), (0, 0));
}

#[test]
fn without_the_revive_slice_a_player_at_no_health_dies() {
    let mut p = in_a_fight("threat");
    let avatar = p.guest_avatar();
    p.host.hit(avatar, RAIDER, 500.0);
    p.frames(3);
    assert!(p.guest.player_dead());
    assert!(!p.guest_fight().down());
    p.host.hit(PLAYER, RAIDER, 500.0);
    assert!(p.host.player_dead());
}

#[test]
fn the_sessions_end_gives_a_downed_player_its_controls_and_its_death_back() {
    let mut p = in_a_fight("");
    let avatar = p.guest_avatar();
    p.host.move_player(HOUSE, [9000.0, 9000.0, 0.0], 0.0);
    p.frames(5);
    p.host.hit(avatar, RAIDER, 500.0);
    p.frames(3);
    assert_eq!(p.guest.controls(), NO_CONTROLS);
    let mut f = p.gd.fight.take().unwrap();
    f.close(&mut p.guest.e);
    assert_eq!(p.guest.controls(), 0);
    // No session: the guest's game applies hits and kills its player itself again.
    p.guest.hit(PLAYER, RAIDER, 500.0);
    assert!(p.guest.player_dead());
}
