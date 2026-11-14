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
/// A quest each player plays in its own game (not shared), and its last stage.
const OPENING: u32 = 0x0010_4c1c;
const HOUSE: u32 = 0x0010_5a00;
const TOWN: u32 = 0x0010_5a01;
/// A third place: the yard outside the opening's room.
const YARD: u32 = 0x0010_5a02;
/// An exterior cell of the world space `MOJAVE`.
const DESERT: u32 = 0x0010_5a10;
const MOJAVE: u32 = 0x0010_5a20;
const RAIDER: u32 = 0x0010_4f25;
const GECKO: u32 = 0x0010_4f26;
const PLAYER: u32 = 0x14;
/// Two packages: what the host's game picks for an actor, and what a guest's own would.
const LEAD_OUT: u32 = 0x0010_6a01;
const STAY_IN: u32 = 0x0010_6a02;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20]), (OPENING, vec![200])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE, TOWN, YARD],
        exteriors: vec![(DESERT, MOJAVE)],
        actors: vec![RAIDER, GECKO],
        packages: vec![LEAD_OUT, STAY_IN],
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

/// The same with the host's actors followed (their AI on here, as the PC runs since run 10 have
/// them): the hit goes through the host, the copy's health is the host's, and the copy is not
/// moved or frozen for it.
#[test]
fn a_guests_hit_on_a_followed_actor_goes_through_the_host() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(40);
    assert!(
        p.guest.actor_view(RAIDER).ai,
        "a followed actor keeps its AI"
    );
    let avatar = p.guest_avatar();
    p.guest.hit(RAIDER, PLAYER, 30.0);
    assert_eq!(p.guest.hits_run(), 0);
    p.frames(3);
    let h = p.host.actor_view(RAIDER);
    assert_eq!((h.health, h.attacker), (70.0, avatar));
    let g = p.guest.actor_view(RAIDER);
    assert_eq!((g.health, g.ai), (70.0, true));
    // While it fights on the host its copy is kept at the host's place every frame, however
    // little it has strayed.
    p.host.set_combat(RAIDER, PLAYER);
    p.host.move_actor(RAIDER, [40.0, 20.0, 0.0], 0.5);
    p.frames(3);
    assert!(close(p.guest.actor_view(RAIDER).pos, [40.0, 20.0, 0.0]));
    assert!(p.guest_fight().fight_leads > 0);
    // A copy still in a conversation here when the host's fights is led into the fight all the
    // same (the fight comes first; a conversation is left alone only out of one).
    p.guest.set_in_dialogue(RAIDER, true);
    p.host.move_actor(RAIDER, [60.0, 30.0, 0.0], 0.5);
    p.frames(3);
    assert!(close(p.guest.actor_view(RAIDER).pos, [60.0, 30.0, 0.0]));
    p.guest.set_in_dialogue(RAIDER, false);
    // What the copy's own AI does to this player is the host's to say.
    p.guest.hit(PLAYER, RAIDER, 20.0);
    assert_eq!(p.guest.hits_run(), 0);
    p.guest.hit(RAIDER, PLAYER, 500.0);
    p.frames(5);
    assert_eq!(p.host.dead(RAIDER), Some(true));
    assert_eq!(p.guest.dead(RAIDER), Some(true));
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
    let host_avatar = p.host_avatar();
    assert!(p.guest.ghost(host_avatar));
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
    // (the host's game keeps the raider: what a guest's game runs hurts that guest there)
    p.hd.fight.as_mut().unwrap().handoff = false;
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
    // (the host's game keeps the raider: what a guest's game runs hurts that guest there)
    p.hd.fight.as_mut().unwrap().handoff = false;
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

/// PC run 8, 2026-11-10: the guest lay looking at the ceiling. Each player plays the opening in
/// its own game, and the guest's Doc Mitchell, held at the host's place with his AI off, never
/// started it. Until a guest's own opening is done it holds none of the host's actors.
#[test]
fn a_guest_holds_no_actor_until_its_own_opening_is_done() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().hold_after = Some((OPENING, 200));
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    let g = p.guest.actor_view(RAIDER);
    assert!(close(g.pos, [900.0, 900.0, 0.0]) && g.ai, "{g:?}");
    assert!(p.guest_fight().held().is_empty());
    assert_eq!(p.guest_fight().moves, 0);
    // The guest's own game ends its opening: from then on the actor is the host's.
    assert!(p.guest.set_stage(OPENING, 200));
    p.frames(5);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos) && !g.ai, "{g:?} {h:?}");
    assert!(p.guest_fight().held().contains(&RAIDER));
    let lines = p.guest_fight().lines.join("\n");
    assert!(
        lines.contains("no actor is held while") && lines.contains("opening is done"),
        "{lines}"
    );
}

/// PC run 26, 2026-11-12: the guest, in its own opening in its own copy of Doc Mitchell's house,
/// was handed the host's Doc Mitchell (the same room by the numbers); the host's copy stood still
/// with its AI off, the host's player never got up, and the guest's opening skipped lines. A
/// guest whose own opening runs is handed no actor.
#[test]
fn a_guest_in_its_own_opening_is_handed_no_actor() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.gd.fight.as_mut().unwrap().hold_after = Some((OPENING, 200));
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(!p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    assert!(p.host.actor_view(RAIDER).ai);
    // Its opening done, the guest is the nearest player and its game runs the actor.
    assert!(p.guest.set_stage(OPENING, 200));
    p.frames(80);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    assert!(p.host.actor_view(RAIDER).ai && p.host_fight().theirs().contains(&RAIDER));
}

/// The same for the move to the host: a host that leaves first does not pull a guest out of its
/// own opening.
#[test]
fn a_guest_is_not_moved_to_the_host_until_its_own_opening_is_done() {
    let mut p = Pair::new();
    p.gd.ladder.as_mut().unwrap().follow_after = Some((OPENING, 200));
    p.frames(40);
    assert_eq!(p.guest.warps(), 0);
    assert!(p
        .gd
        .ladder
        .as_ref()
        .unwrap()
        .lines
        .iter()
        .any(|l| l.contains("follows once its own opening is done")));
    // PC run 19: the stage is set indoors; the guest is not moved until it has walked out
    // itself (the trigger at the door gives the player its weapons back) and a little after.
    assert!(p.guest.set_stage(OPENING, 200));
    p.frames(40);
    assert_eq!(
        p.guest.warps(),
        0,
        "moved before walking out of the opening's room"
    );
    p.guest.move_player(YARD, [5.0, 5.0, 0.0], 0.0);
    p.frames(10);
    assert_eq!(
        p.guest.warps(),
        0,
        "moved at once, before the door's trigger ran"
    );
    p.frames(60);
    assert_eq!(p.guest.warps(), 1);
}

/// PC run 10, 2026-11-11: Sunny had walked outdoors on the host; the guest's copy of her was
/// still in the saloon and was moved to the outdoor coordinates inside it. A move changes a
/// reference's place, not its cell: a copy in another space than the host's actor is not held.
#[test]
fn a_copy_in_another_space_than_the_hosts_actor_is_not_held() {
    let mut p = Pair::new();
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    // The guest's copy is in the other space.
    p.guest.place_actor(RAIDER, TOWN, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    let g = p.guest.actor_view(RAIDER);
    assert!(close(g.pos, [900.0, 900.0, 0.0]) && g.ai, "{g:?}");
    assert!(p.guest_fight().held().is_empty());
    assert!(p
        .guest_fight()
        .lines
        .iter()
        .any(|l| l.contains("not held") && l.contains(&format!("{RAIDER:08x}"))));
    // Its own game brings it into the host's space: held from then on.
    p.guest.place_actor(RAIDER, HOUSE, [700.0, 700.0, 0.0], 2.0);
    p.frames(5);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos) && !g.ai, "{g:?} {h:?}");
}

/// PC run 7, 2026-11-10: Joe Cobb's confrontation with Trudy (a trigger's script says
/// `GSJoeCobbRef.StartConversation TrudyRef ...`) played in the host's game only. A conversation a
/// script starts between two actors is the host's: the host's game runs it and tells the guests,
/// whose games play it once, with both where the host had them. A guest's own script asking for
/// one is not obeyed (docs/coop/DECISIONS.md D5, condition 5: no double scenes), and its script
/// still reads past the command's parameters.
#[test]
fn a_conversation_between_actors_is_the_hosts_and_plays_once_on_the_guest() {
    let mut p = Pair::new();
    // Shared fights are not needed for scenes.
    p.hd.fight = None;
    p.gd.fight = None;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.guest.place_actor(GECKO, HOUSE, [-300.0, 40.0, 0.0], 2.0);
    p.frames(30);
    // The host's script starts it: once there, once on the guest, both moved to the host's places.
    assert_eq!(p.host.script_start_conversation(RAIDER, GECKO, MQ), 6);
    assert_eq!(p.host.conversations(RAIDER), (1, GECKO, MQ));
    p.frames(60);
    assert_eq!(p.guest.conversations(RAIDER), (1, GECKO, MQ));
    assert_eq!((p.gd.scenes_played, p.gd.scenes_missed), (1, 0));
    assert!(close(p.guest.actor_view(RAIDER).pos, [10.0, 20.0, 0.0]));
    assert!(close(p.guest.actor_view(GECKO).pos, [60.0, 20.0, 0.0]));
    assert_eq!(
        (p.gd.refused, p.gd.healed, p.gd.sync.desyncs_detected),
        (0, 0, 0)
    );
    assert!(p
        .hd
        .stage_notes
        .iter()
        .any(|l| l.contains("scene:") && l.contains(&format!("{RAIDER:08x}"))));
    // A conversation with the player is that player's own: a guest's runs in its own game and
    // the host's is not sent.
    assert_eq!(p.guest.script_start_conversation(RAIDER, PLAYER, MQ), 6);
    assert_eq!(p.guest.conversations(RAIDER), (2, PLAYER, MQ));
    assert_eq!(p.host.script_start_conversation(GECKO, PLAYER, MQ), 6);
    p.frames(60);
    assert_eq!(p.guest.conversations(GECKO).0, 0);
    assert_eq!(p.gd.scenes_played, 1);
}

/// PC run 23, 2026-11-12: the guest walked into the trigger by Trudy and Joe Cobb and nothing
/// happened in either game. A scene a guest's own script starts is asked of the host, whose game
/// plays it and so has every game play it once; the host's own trigger firing after does not
/// play it again.
#[test]
fn a_scene_a_guests_own_script_starts_is_played_by_the_host_for_everyone() {
    let mut p = Pair::new();
    p.hd.fight = None;
    p.gd.fight = None;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.frames(30);
    // The guest's trigger: not played there at once, and its script goes on.
    assert_eq!(p.guest.script_start_conversation(RAIDER, GECKO, MQ), 6);
    assert_eq!(p.guest.conversations(RAIDER).0, 0);
    p.frames(60);
    assert_eq!(p.host.conversations(RAIDER), (1, GECKO, MQ));
    assert_eq!(p.guest.conversations(RAIDER), (1, GECKO, MQ));
    assert_eq!(p.gd.scenes_asked, 1);
    // The host's player walks into its own trigger: not a second time.
    assert_eq!(p.host.script_start_conversation(RAIDER, GECKO, MQ), 6);
    p.frames(60);
    assert_eq!(p.host.conversations(RAIDER).0, 1);
    assert_eq!(p.guest.conversations(RAIDER).0, 1);
}

/// PC run 31: the host walked up to Joe Cobb and Trudy and their scene played; the guest walked
/// into its own trigger later, the host's game played the scene again, and Joe began his line again
/// as he ran off. A scene a guest's game asks for that played on the host a little while ago is
/// not played again; after a while it is.
#[test]
fn a_scene_the_host_just_played_is_not_played_again_for_a_guest() {
    let mut p = Pair::new();
    p.hd.fight = None;
    p.gd.fight = None;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.frames(30);
    assert_eq!(p.host.script_start_conversation(RAIDER, GECKO, MQ), 6);
    p.frames(60);
    assert_eq!(p.guest.conversations(RAIDER).0, 1);
    // The guest's own trigger, later: not played again anywhere.
    assert_eq!(p.guest.script_start_conversation(RAIDER, GECKO, MQ), 6);
    p.frames(60);
    assert_eq!(p.host.conversations(RAIDER).0, 1);
    assert_eq!(p.guest.conversations(RAIDER).0, 1);
    assert_eq!(p.hd.scenes_refused, 1);
    // Long after, not while one of them is in a conversation on the host; then it is.
    p.frames((coop_engine_session::SCENE_AGAIN_MS / 50) as u32);
    p.host.set_in_dialogue(GECKO, true);
    assert_eq!(p.guest.script_start_conversation(RAIDER, GECKO, MQ), 6);
    p.frames(60);
    assert_eq!(p.hd.scenes_refused, 2);
    p.host.set_in_dialogue(GECKO, false);
    assert_eq!(p.guest.script_start_conversation(RAIDER, GECKO, MQ), 6);
    p.frames(60);
    assert_eq!(p.host.conversations(RAIDER).0, 2);
    assert_eq!(p.guest.conversations(RAIDER).0, 2);
}

/// A scene that happened before a guest joined is over: it comes with the join and is not played.
#[test]
fn a_scene_from_before_the_join_is_not_played() {
    let mut p = Pair::new();
    p.hd.fight = None;
    p.gd.fight = None;
    for s in [&mut p.host, &mut p.guest] {
        s.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
        s.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    }
    p.host.script_start_conversation(RAIDER, GECKO, MQ);
    p.frames(60);
    assert_eq!(p.guest.conversations(RAIDER).0, 0);
    assert_eq!((p.gd.scenes_played, p.gd.scenes_missed), (0, 0));
}

/// PC run 10, 2026-11-11: an actor held with its AI off kept the pose it had and stood in a
/// floor. Followed instead, it keeps its own AI. D6 (the owner, 2026-11-14: no teleports): it is
/// put on the host's actor's path while that one moves or has only just stopped. Without D7's
/// keeping it there, once that one has stood a while it is left alone near it; further off for a
/// while it walks back (no jump), and only one too far to walk is put there at once.
#[test]
fn a_followed_actor_keeps_its_ai_and_walks_back_to_the_hosts_place_when_it_strays() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    // (with the copy kept where the host's stands switched off: D7's fallback)
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [60.0, 20.0, 0.0], 2.0);
    p.frames(60);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos) && g.ai, "{g:?} {h:?}");
    // The host's actor has stood a while: the copy is its own AI's, its movement its own.
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    // Near the host's place: untouched.
    p.guest.move_actor(RAIDER, [110.0, 20.0, 0.0], 2.0);
    p.frames(40);
    assert!(close(p.guest.actor_view(RAIDER).pos, [110.0, 20.0, 0.0]));
    assert_eq!(p.guest_fight().nudges, 0);
    // Strayed, but not for long enough yet.
    p.guest.move_actor(RAIDER, [610.0, 20.0, 0.0], 2.0);
    p.frames(10);
    assert!(close(p.guest.actor_view(RAIDER).pos, [610.0, 20.0, 0.0]));
    // Long enough: it walks back, a step at a time, its AI on.
    p.frames(30);
    let g = p.guest.actor_view(RAIDER);
    assert!(g.pos[0] < 600.0 && g.pos[0] > 300.0 && g.ai, "{g:?}");
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x101));
    assert_eq!(p.guest_fight().nudges, 1);
    // Taken far off on its way (its own game moved it): put there at once.
    p.guest.move_actor(RAIDER, [2000.0, 20.0, 0.0], 2.0);
    p.frames(1);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert_eq!(p.guest_fight().following.snaps, 1);
    p.frames(80);
    let g = p.guest.actor_view(RAIDER);
    assert!(g.pos[0] <= 110.0, "{g:?}");
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    p.frames(60);
    assert_eq!(p.guest_fight().nudges, 1);
    assert!(p.guest_fight().held().is_empty());
    // Too far to walk: put there at once.
    p.guest.move_actor(RAIDER, [2000.0, 20.0, 0.0], 2.0);
    p.frames(2);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert_eq!(p.guest_fight().following.snaps, 2);
}

/// D6: while the host's actor walks, the copy goes along its path a little behind, smoothly,
/// with its gait; off the path, it catches up rather than jumping.
#[test]
fn a_copy_goes_along_the_hosts_actors_path_with_its_gait() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [0.0, 0.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [400.0, 0.0, 0.0], 0.0);
    p.frames(80);
    // The host's raider runs north, 20 units a frame.
    p.host.set_moves(RAIDER, 0x201);
    let mut xs = Vec::new();
    for i in 1..=40 {
        p.host.move_actor(RAIDER, [0.0, 20.0 * i as f32, 0.0], 0.0);
        p.frames(1);
        xs.push(p.guest.actor_view(RAIDER).pos[1]);
    }
    // Running too, behind the host's by a little, and never by a jump.
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x201));
    let behind = 800.0 - xs[39];
    assert!((20.0..=120.0).contains(&behind), "{behind}");
    assert!(xs.windows(2).all(|w| (w[1] - w[0]).abs() <= 45.0), "{xs:?}");
    // It stops: the copy stops where it did, and stands there (D7).
    p.host.set_moves(RAIDER, 0);
    p.frames(10);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0));
    p.frames((fight::SETTLE_MS / 50) as u32 + 5);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0));
    // Taken off its path by its own AI, it catches up when the host's moves again.
    p.guest.move_actor(RAIDER, [0.0, 1300.0, 0.0], 0.0);
    p.host.move_actor(RAIDER, [0.0, 820.0, 0.0], 0.0);
    p.host.set_moves(RAIDER, 0x101);
    p.frames(3);
    let g = p.guest.actor_view(RAIDER);
    assert!(g.pos[1] > 900.0 && g.pos[1] < 1300.0, "{g:?}");
    p.frames(20);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert_eq!(p.guest_fight().following.snaps, 0);
}

/// PC run 30: Joe Cobb's scene played in both games; the guest's copy, still in its conversation
/// when the host's was done, was put at the host's place and asked to take its leaving package,
/// and cut his line to run off. A followed copy in a conversation here is left to it; once it is
/// over it is followed again.
#[test]
fn a_followed_actor_in_a_conversation_here_is_left_to_it() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [610.0, 20.0, 0.0], 2.0);
    p.guest.set_in_dialogue(RAIDER, true);
    p.frames(80);
    assert!(close(p.guest.actor_view(RAIDER).pos, [610.0, 20.0, 0.0]));
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    assert_eq!(p.guest_fight().nudges, 0);
    assert!(p.guest_fight().talks_left > 0);
    // Over: followed again (back where the host's stands).
    p.guest.set_in_dialogue(RAIDER, false);
    p.frames(20);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    // A conversation that never moves on is left alone only so long: then followed again.
    p.guest.move_actor(RAIDER, [610.0, 20.0, 0.0], 2.0);
    p.guest.set_in_dialogue(RAIDER, true);
    let frames = (fight::TALK_GRACE_MS / 50) as u32;
    p.frames(frames - 20);
    assert!(close(p.guest.actor_view(RAIDER).pos, [610.0, 20.0, 0.0]));
    p.frames(40);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
}

/// D7 (PC run 31: Sunny stood by her campfire in the host's game and further off in the guest's,
/// where her copy, left to its own AI, had picked its own spot; committee 3-0): a copy stands where
/// the host's actor stands, however long that one stays, standing; taken off it by its own game,
/// it is back there at once (catching up, no jump), and it goes where the host's goes.
#[test]
fn a_copy_stands_where_the_hosts_actor_stands() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [210.0, 20.0, 0.0], 2.0);
    p.frames(5 * (fight::SETTLE_MS / 50) as u32);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos) && g.ai, "{g:?} {h:?}");
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0));
    // Its own game takes it a little way off: back where the host's stands within a frame or two.
    p.guest.move_actor(RAIDER, [110.0, 20.0, 0.0], 2.0);
    p.frames(3);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    // The host's moves on: so does the copy.
    p.host.move_actor(RAIDER, [300.0, 20.0, 0.0], 0.5);
    p.frames(10);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert_eq!(
        (p.guest_fight().nudges, p.guest_fight().following.snaps),
        (0, 0)
    );
}

/// PC run 12, 2026-11-11: the host's Sunny walked out of the saloon; the guest's copy stood at
/// the door and walked back. A followed copy that is in another space than the host's actor is
/// brought into the guest's player's space, at the host's place, by the game's own `MoveTo`.
#[test]
fn a_followed_copy_is_brought_through_the_door_after_the_hosts_actor() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().follow = true;
    // The host and its actor are in the house; so is the guest's player, but its copy is not.
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [300.0, 300.0, 0.0], 0.0);
    p.guest.place_actor(RAIDER, TOWN, [900.0, 900.0, 0.0], 2.0);
    p.frames(40);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos) && g.ai, "{g:?} {h:?}");
    assert_eq!(p.guest_fight().brings, 1);
    assert!(p
        .guest_fight()
        .lines
        .iter()
        .any(|l| l.contains("brought to") && l.contains(&format!("{RAIDER:08x}"))));
    // In the same space from then on: followed as any other, not brought again.
    p.frames(200);
    assert_eq!(p.guest_fight().brings, 1);
}

/// PC run 12, 2026-11-11: Sunny led the host outdoors and, on the guest, walked back to where she
/// had been; Chet sat down on the host and walked on the spot on the guest. A package's
/// conditions are each game's own ("the player" is that game's player), so the two copies of an
/// actor picked different packages. A followed actor's copy is given the package the host's runs.
#[test]
fn a_followed_actor_runs_the_package_the_hosts_copy_runs() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().follow = true;
    // The host's game runs it throughout (with hand-off, one out of the host's player's sight
    // near the guest is the guest's game's, which picks its own package: see
    // `the_nearest_players_game_runs_an_actor`).
    p.hd.fight.as_mut().unwrap().handoff = false;
    p.gd.fight.as_mut().unwrap().hold_after = Some((OPENING, 200));
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.guest.set_own_package(RAIDER, STAY_IN);
    // While the guest's own opening runs, its actors act for it.
    p.frames(40);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    p.guest.set_own_package(RAIDER, STAY_IN);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    // The opening done: the copy is asked again and runs the host's package, and keeps it
    // when its own game evaluates.
    assert!(p.guest.set_stage(OPENING, 200));
    p.frames(80);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    p.guest.set_own_package(RAIDER, STAY_IN);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    assert!(coop_engine::commands::packages_given(&mut p.guest.e) >= 2);
    // The host's changes: the guest's follows.
    p.host.set_own_package(RAIDER, STAY_IN);
    p.frames(20);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    // Given as the game's own pick is: the procedure type computed, the old package ended and
    // the new one begun.
    let (computed, begun, _) = p.guest.package_marks(LEAD_OUT);
    assert!(computed && begun >= 1, "{computed} {begun}");
    assert!(p.guest.package_marks(STAY_IN).2 >= 1);
    assert!(coop_engine::commands::drain_notes(&mut p.guest.e)
        .iter()
        .any(|l| l.contains(&format!("{RAIDER:08x}")) && l.contains(&format!("{LEAD_OUT:08x}"))));
    // The host's player goes away from the actor: the host's game runs it on, and its copy here
    // keeps the host's package (PC run 14: Sunny's copy came back indoors).
    p.host.set_near(RAIDER, false);
    p.frames(20);
    p.guest.set_own_package(RAIDER, LEAD_OUT);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    // Out of the host's player's sight it takes another package: the copy is given that.
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.frames(80);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    p.host.set_own_package(RAIDER, STAY_IN);
    p.host.set_near(RAIDER, true);
    p.frames(80);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    // It dies out of sight: the host has no more to say of it.
    p.host.set_near(RAIDER, false);
    p.frames(20);
    let a = coop_engine::actors::actor(&mut p.host.e, RAIDER).unwrap();
    p.host
        .e
        .mem
        .set_u32(a + coop_engine::addr::ACTOR_LIFE_STATE_OFFSET, 1);
    p.frames(40);
    assert_eq!(
        coop_engine::commands::wanted_package(&mut p.guest.e, RAIDER),
        None
    );
    // The other actor, of which the host has said nothing, is this game's own.
    p.guest.place_actor(GECKO, HOUSE, [0.0; 3], 0.0);
    p.guest.set_own_package(GECKO, LEAD_OUT);
    assert_eq!(p.guest.package(GECKO), LEAD_OUT);
    assert_eq!(
        (p.gd.refused, p.gd.healed, p.gd.sync.desyncs_detected),
        (0, 0, 0)
    );
}

/// PC run 13, 2026-11-11: Joe Cobb left the saloon on the host and stood still on the guest;
/// Sunny's copy walked out after the host's and came straight back in. The host's package was
/// only kept on copies in the guest's player's space with the host's actor in it too. It is kept
/// on every actor the host runs: a copy the host's has left behind is asked to take it (and so
/// walks out as the host's did), and one that is already where the host's is keeps it.
#[test]
fn a_copy_keeps_the_hosts_package_when_the_hosts_actor_is_in_another_space() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().follow = true;
    // Both players and the actor are in the house.
    p.host.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.host.set_own_package(RAIDER, STAY_IN);
    p.guest.set_own_package(RAIDER, STAY_IN);
    p.frames(40);
    // The host's walks to the door and out into the town; the guest's copy is still where it
    // was, 500 units from the last place the host's had in the house.
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.host.place_actor(RAIDER, HOUSE, [400.0, 900.0, 0.0], 2.0);
    p.frames(4);
    p.host.place_actor(RAIDER, TOWN, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    let before = p.guest_fight().nudges;
    p.frames(120);
    assert_eq!(p.hd.fight.as_ref().unwrap().left_told, 1);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    // It is not put back at the place the host's had in the house: its own legs take it out.
    assert_eq!(p.guest.actor_view(RAIDER).pos, [900.0, 900.0, 0.0]);
    assert_eq!(p.guest_fight().nudges, before);
    assert!(p
        .guest_fight()
        .lines
        .iter()
        .any(|l| l.contains("asked to follow it out") && l.contains(&format!("{RAIDER:08x}"))));
    // It has walked out, into the town where the host's is and the guest's player is not: its
    // own game's choice there is still the host's.
    p.guest.place_actor(RAIDER, TOWN, [10.0, 20.0, 0.0], 0.5);
    p.frames(20);
    p.guest.set_own_package(RAIDER, STAY_IN);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    // With packages off, neither.
    p.gd.fight.as_mut().unwrap().packages_on = false;
    p.frames(20);
    p.guest.set_own_package(RAIDER, STAY_IN);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    assert_eq!(
        (p.gd.refused, p.gd.healed, p.gd.sync.desyncs_detected),
        (0, 0, 0)
    );
}

/// PC run 18, 2026-11-12: Sunny waits for "the player" and talks to whoever that is in each game;
/// her copy on the guest was not given that package, chose its own, walked off and was dragged
/// back, and only the host's player could be spoken to. A package aimed at the player is given
/// to the copy like any other, and the copy is then given room: it is not led, and is put back
/// only when far from the host's place.
#[test]
fn a_copy_whose_package_is_aimed_at_the_player_is_given_it_and_room() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    for g in [&mut p.host, &mut p.guest] {
        let pkg = coop_engine::forms::form(&mut g.e, LEAD_OUT).unwrap();
        let player = g.e.mem.u32(coop_engine::addr::PLAYER_SINGLETON);
        g.e.mem.set_u32(pkg + 0x60, 0);
        g.e.mem.set_u32(pkg + 0x64, player);
        g.e.mem
            .set_u32(pkg + coop_engine::addr::PACKAGE_TARGET_OFFSET, pkg + 0x60);
    }
    assert!(coop_engine::commands::aimed_at_player(
        &mut p.guest.e,
        LEAD_OUT
    ));
    assert!(!coop_engine::commands::aimed_at_player(
        &mut p.guest.e,
        STAY_IN
    ));
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.guest.set_own_package(RAIDER, STAY_IN);
    p.frames(80);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    // It has turned to this game's player, 500 units from the host's: left where it is.
    p.guest.move_actor(RAIDER, [510.0, 20.0, 0.0], 0.5);
    p.frames(200);
    assert!(close(p.guest.actor_view(RAIDER).pos, [510.0, 20.0, 0.0]));
    assert_eq!(p.guest_fight().nudges, 0);
    // The host's walks off: this one is not taken along (it is about this game's player).
    p.host.set_moves(RAIDER, 0x101);
    p.host.move_actor(RAIDER, [10.0, 60.0, 0.0], 0.5);
    p.frames(5);
    assert!(close(p.guest.actor_view(RAIDER).pos, [510.0, 20.0, 0.0]));
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    p.host.set_moves(RAIDER, 0);
    // Further than its room, again and again: walked back each time, never kept there.
    for _ in 0..3 {
        p.guest.move_actor(RAIDER, [900.0, 60.0, 0.0], 0.5);
        p.frames(160);
    }
    assert_eq!(p.guest_fight().nudges, 3);
    // Far off it is still put back.
    p.guest.move_actor(RAIDER, [2000.0, 20.0, 0.0], 0.5);
    p.frames(60);
    assert!(close(p.guest.actor_view(RAIDER).pos, [10.0, 60.0, 0.0]));
}

/// PC run 21, 2026-11-12: with the host's player at the saloon door, Sunny and the geckos near
/// the guest stood still on the host (its game ran them at a low level, far from its player)
/// and jumped about on the guest. The nearest player's game runs an actor: here the guest's.
/// The host's copy is put where the guest's game has it, the host's player's hits on it go
/// through the ledger and are done in the guest's game, and the host takes it back when its
/// player comes nearer.
#[test]
fn the_nearest_players_game_runs_an_actor() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(
        p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)),
        "{:?}",
        p.host_fight().lines
    );
    // PC run 22: the host's own lists leave it out now and then; it stays the guest's.
    p.host.set_near(RAIDER, false);
    p.frames(30);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    // PC run 24: the host's player is nearer by the numbers, but the host's game does not run
    // it at full detail: it stays the guest's.
    p.host.move_player(HOUSE, [12.0, 20.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [900.0, 20.0, 0.0], 0.0);
    p.frames(60);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    // PC run 25: at the edge of the host's list it is in and out from one moment to the next:
    // it stays the guest's until the host's list has had it CLOSE_SETTLE_MS without a break.
    for _ in 0..4 {
        p.host.set_near(RAIDER, true);
        p.frames(40);
        p.host.set_near(RAIDER, false);
        p.frames(2);
    }
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    p.host.set_near(RAIDER, true);
    p.frames(140);
    assert!(!p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    p.host.set_near(RAIDER, false);
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.frames(20);
    p.host.set_near(RAIDER, true);
    // The guest's game moves it: the host's copy follows.
    p.guest.move_actor(RAIDER, [200.0, 120.0, 0.0], 1.0);
    p.frames(20);
    assert!(close(p.host.actor_view(RAIDER).pos, [200.0, 120.0, 0.0]));
    // In a conversation in the host's game (a scene it plays, PC run 30), the host's copy is not
    // led: it plays the scene where it is.
    p.host.set_in_dialogue(RAIDER, true);
    p.guest.move_actor(RAIDER, [260.0, 160.0, 0.0], 1.0);
    p.frames(20);
    assert!(close(p.host.actor_view(RAIDER).pos, [200.0, 120.0, 0.0]));
    p.host.set_in_dialogue(RAIDER, false);
    p.frames(20);
    assert!(close(p.host.actor_view(RAIDER).pos, [260.0, 160.0, 0.0]));
    p.guest.move_actor(RAIDER, [200.0, 120.0, 0.0], 1.0);
    p.frames(20);
    assert!(p.gd.fight.as_ref().unwrap().owned_published > 0);
    // Its AI runs on the guest: it is not put back at the host's place.
    assert!(close(p.guest.actor_view(RAIDER).pos, [200.0, 120.0, 0.0]));
    // PC run 25: led with its AI on, the host's copy far from its player finished Sunny's sneak
    // package and her conversation at once and set the stages past them. The host's copy is kept
    // still, its AI off, and its package says nothing: the guest's copy picks its own, as the
    // guest's game has the story.
    assert!(p.host.actor_view(RAIDER).ai && p.host_fight().theirs().contains(&RAIDER));
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.guest.set_own_package(RAIDER, STAY_IN);
    p.frames(80);
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    assert_eq!(
        coop_engine::commands::wanted_package(&mut p.guest.e, RAIDER),
        None
    );
    p.guest.set_own_package(RAIDER, LEAD_OUT);
    p.frames(20);
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    // A word of the host's about its package that comes late changes nothing here, nor does a
    // host's script telling it to evaluate its package (its own pick stands).
    coop_engine::commands::set_wanted_package(&mut p.guest.e, RAIDER, STAY_IN);
    p.frames(5);
    assert!(coop_engine::commands::evaluate_package(
        &mut p.guest.e,
        RAIDER
    ));
    assert_eq!(p.guest.package(RAIDER), LEAD_OUT);
    // The host's player shoots it: done in the guest's game, through the ledger.
    p.host.hit(RAIDER, PLAYER, 30.0);
    assert_eq!(p.host.hits_run(), 0, "kept from the host's copy");
    p.frames(10);
    assert_eq!(p.guest.actor_view(RAIDER).health, 70.0);
    assert_eq!(p.host.actor_view(RAIDER).health, 70.0);
    // The guest's player shoots it: through the ledger too, done once.
    let run = p.guest.hits_run();
    p.guest.hit(RAIDER, PLAYER, 20.0);
    assert_eq!(p.guest.hits_run(), run, "kept until the ledger says");
    p.frames(10);
    assert_eq!(p.guest.actor_view(RAIDER).health, 50.0);
    // It hits the guest's player: that runs in the guest's game, where it is run.
    let run = p.guest.hits_run();
    p.guest.hit(PLAYER, RAIDER, 10.0);
    assert_eq!(p.guest.hits_run(), run + 1);
    // The host's player walks up to it: the host's game runs it again.
    p.host.move_player(HOUSE, [210.0, 120.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.frames(80);
    assert!(!p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    // Its AI on again in the host's game.
    assert!(p.host.actor_view(RAIDER).ai);
    // With hand-off off the host keeps it whoever is nearer.
    let mut q = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    q.hd.fight.as_mut().unwrap().handoff = false;
    q.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    q.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    q.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    q.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    q.frames(80);
    assert!(!q.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
}

/// PC run 27, 2026-11-12: the guest started Sunny's tutorial; the host's player stood nearer, at
/// the saloon, so the host's game took her back, and her package waiting for "the player" had her
/// walk to the host's player and wait for it to talk. An actor is run by the game of the player
/// it last spoke with while that player is near it.
#[test]
fn an_actor_is_run_by_the_game_of_the_player_it_spoke_with() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    let guest = p.gd.sync.player;
    let owned = |p: &Pair| p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER));
    p.host.move_player(HOUSE, [12.0, 20.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [900.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(60);
    assert!(!owned(&p), "the host's player is nearer");
    p.hd.fight.as_mut().unwrap().engage(RAIDER, guest);
    p.frames(60);
    assert!(owned(&p));
    assert!(p.host.actor_view(RAIDER).ai && p.host_fight().theirs().contains(&RAIDER));
    // Spoken with by the host's player out of its game's full detail: it stays the guest's,
    // the nearer.
    p.host.set_near(RAIDER, false);
    p.hd.fight
        .as_mut()
        .unwrap()
        .engage(RAIDER, coop_core::ids::PlayerId::HOST);
    p.frames(60);
    assert!(owned(&p));
    p.host.set_near(RAIDER, true);
    p.frames(120);
    // The host's player speaks with it: the host's game runs it again.
    p.hd.fight
        .as_mut()
        .unwrap()
        .engage(RAIDER, coop_core::ids::PlayerId::HOST);
    p.frames(60);
    assert!(!owned(&p));
    assert!(p.host.actor_view(RAIDER).ai);
    // The guest speaks with it again and then goes far off: the nearest player's game again.
    p.hd.fight.as_mut().unwrap().engage(RAIDER, guest);
    p.frames(60);
    assert!(owned(&p));
    p.guest.move_player(HOUSE, [9000.0, 20.0, 0.0], 0.0);
    p.frames(60);
    assert!(!owned(&p));
}

/// PC run 25 and 29: the host's copy of an actor a guest's game runs walks with its AI on (kept
/// still with its AI off it sank into the floor and slid), and what it would do on its own in
/// the host's game is skipped: the guest's game does it and proposes the outcome. A scene the
/// host plays, and a death, are the host's still.
#[test]
fn the_hosts_copy_of_an_actor_a_guest_runs_does_nothing_of_its_own() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    assert!(p.host.actor_view(RAIDER).ai);
    let done = |p: &mut Pair, quest: u32, stage: u8| {
        coop_engine::quest::record(&mut p.host.e, quest)
            .unwrap()
            .done
            .contains(&stage)
    };
    // Its own script (a package's end, its object script): skipped here.
    p.host.run_object_script_setting_stage(RAIDER, MQ, 20);
    assert!(!done(&mut p, MQ, 20));
    assert_eq!(coop_engine::gate::puppet_skips(&mut p.host.e), 1);
    // In a scene this game plays: its lines run here.
    coop_engine::gate::set_scene_actors(&mut p.host.e, [RAIDER]);
    p.host.run_dialogue_result_setting_stage(RAIDER, MQ, 10);
    assert!(done(&mut p, MQ, 10));
    coop_engine::gate::set_scene_actors(&mut p.host.e, []);
    // Another actor's script runs as ever.
    p.host.place_actor(GECKO, HOUSE, [5000.0, 5000.0, 0.0], 0.5);
    p.host.run_object_script_setting_stage(GECKO, MQ, 20);
    assert!(done(&mut p, MQ, 20));
    // Dead, its scripts run here (its death is the host's to run).
    p.host.kill(RAIDER);
    p.host.run_object_script_setting_stage(RAIDER, OPENING, 200);
    assert!(done(&mut p, OPENING, 200));
}

/// Harness, 2026-11-13: Sunny at the first well, far from the host's player in a part of the
/// world its game had not loaded, had no space in the host's game (her cell let go), was never
/// handed to the guest standing by her, and the host's game ran her sneak and her talk at once
/// (stages 35, 40 and 45 together, as in PC run 25). Such an actor near a guest out in a world
/// space is in that world space, and the guest's game runs it.
#[test]
fn an_actor_in_a_part_of_the_world_the_host_has_not_loaded_goes_to_the_guest_near_it() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(DESERT, [-60000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(DESERT, [-67000.0, -9000.0, 0.0], 0.0);
    p.host
        .place_actor(RAIDER, DESERT, [-67100.0, -9000.0, 0.0], 0.5);
    p.guest
        .place_actor(RAIDER, DESERT, [-67100.0, -9000.0, 0.0], 0.5);
    // The host's game let its cell go.
    let a = coop_engine::actors::actor(&mut p.host.e, RAIDER).unwrap();
    p.host
        .e
        .mem
        .set_u32(a + coop_engine::addr::REF_CELL_OFFSET, 0);
    p.frames(80);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    assert!(p.host_fight().theirs().contains(&RAIDER));
    // Its own doing in the host's game is skipped.
    p.host.run_object_script_setting_stage(RAIDER, MQ, 20);
    assert!(!coop_engine::quest::record(&mut p.host.e, MQ)
        .unwrap()
        .done
        .contains(&20));
}

/// Harness, 2026-11-13: while no player was near Sunny, the guest's copy stood at the first well
/// on a stale package and the host's walked on to the third; when the guest came near, its game
/// took her over where its own copy stood. A copy taken over is first put where the host's game
/// had the actor.
#[test]
fn an_actor_a_guest_takes_over_goes_on_from_the_hosts_place() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    // Only the takeover moves the copy here (no following before it).
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [1500.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    assert!(close(p.guest.actor_view(RAIDER).pos, [10.0, 20.0, 0.0]));
}

/// Harness, 2026-11-13: Sunny walked from the saloon to her campfire in the host's game and was
/// handed to the guest there, out of the host's player's close list; the guest's copy, near no
/// player all along, was still in the saloon, so neither game ran her and both copies stood. A
/// copy taken over in another space than the host's actor (which is in the guest's player's) is
/// brought out to the host's place.
#[test]
fn an_actor_a_guest_takes_over_in_another_space_is_brought_to_the_hosts_place() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    // Only the takeover moves the copy here (no following before it).
    p.gd.fight.as_mut().unwrap().bring_on = false;
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.move_player(DESERT, [-60000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(DESERT, [-67000.0, -9000.0, 0.0], 0.0);
    p.host
        .place_actor(RAIDER, DESERT, [-67100.0, -9000.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    // The host's game let its cell go: it is the guest's to run.
    let a = coop_engine::actors::actor(&mut p.host.e, RAIDER).unwrap();
    p.host
        .e
        .mem
        .set_u32(a + coop_engine::addr::REF_CELL_OFFSET, 0);
    p.frames(80);
    assert!(p.gd.sync.live.actors.owns(coop_core::ids::FormId(RAIDER)));
    let g = p.guest.actor_view(RAIDER);
    assert!(close(g.pos, [-67100.0, -9000.0, 0.0]), "{g:?}");
    let copy = coop_engine::actors::actor(&mut p.guest.e, RAIDER).unwrap();
    assert_eq!(coop_engine::fight::space_of(&mut p.guest.e, copy), MOJAVE);
    assert_eq!(p.guest_fight().takeover_brings, 1);
    // Not brought again once there.
    p.frames(200);
    assert_eq!(p.guest_fight().takeover_brings, 1);
}

/// A copy is brought out only into the space this player is in: one taken over while this
/// player has gone into another space than the host's actor is not moved to that actor's
/// coordinates in this player's space (PC run 10's fault: Sunny put at outdoor coordinates in
/// the saloon).
#[test]
fn a_copy_taken_over_is_not_brought_into_a_space_the_hosts_actor_is_not_in() {
    let mut p = Pair::with(Cap::Log, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.gd.fight.as_mut().unwrap().bring_on = false;
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.move_player(DESERT, [-60000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(DESERT, [-67000.0, -9000.0, 0.0], 0.0);
    p.host
        .place_actor(RAIDER, DESERT, [-67100.0, -9000.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, TOWN, [10.0, 20.0, 0.0], 0.5);
    let a = coop_engine::actors::actor(&mut p.host.e, RAIDER).unwrap();
    p.host
        .e
        .mem
        .set_u32(a + coop_engine::addr::REF_CELL_OFFSET, 0);
    // The host hands it over; before the guest's game takes it, its player goes indoors (the
    // ladder stops before rung 3, which would move it back to the host).
    let id = coop_core::ids::FormId(RAIDER);
    let mut frames = 0;
    loop {
        p.t += 50;
        p.hd.step(&mut p.host.e, &mut p.hnet, p.t);
        let handed =
            p.hd.sync
                .live
                .actors
                .own
                .get(id)
                .is_some_and(|r| r.owner.is_some_and(|o| !o.is_host()));
        if handed || frames > 200 {
            break;
        }
        p.guest.run_quest_script();
        p.gd.step(&mut p.guest.e, &mut p.gnet, p.t);
        frames += 1;
    }
    assert!(!p.gd.sync.live.actors.owns(id));
    p.guest.move_player(HOUSE, [300.0, 300.0, 0.0], 0.0);
    p.guest.run_quest_script();
    p.gd.step(&mut p.guest.e, &mut p.gnet, p.t);
    assert!(p.gd.sync.live.actors.owns(id));
    p.frames(1);
    assert_eq!(p.guest_fight().takeover_brings, 0);
    let copy = coop_engine::actors::actor(&mut p.guest.e, RAIDER).unwrap();
    assert_eq!(coop_engine::fight::space_of(&mut p.guest.e, copy), TOWN);
}

/// PC run 23: a gecko's death script ran twice in the guest's game, and Sunny's quest counted two
/// geckos with one left. A dead actor is never hurt or killed again.
#[test]
fn a_dead_actor_is_not_hurt_or_killed_again() {
    let mut p = Pair::new();
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    let a = coop_engine::actors::actor(&mut p.guest.e, RAIDER).unwrap();
    p.guest
        .e
        .mem
        .set_u32(a + coop_engine::addr::ACTOR_LIFE_STATE_OFFSET, 2);
    let health = p.guest.actor_view(RAIDER).health;
    assert_eq!(
        coop_engine::fight::hurt_actor(&mut p.guest.e, RAIDER, 30.0, 0),
        None
    );
    assert_eq!(p.guest.actor_view(RAIDER).health, health);
    assert!(!coop_engine::fight::kill_by(&mut p.guest.e, RAIDER, 0));
    assert!(!coop_engine::actors::kill(&mut p.guest.e, RAIDER));
}

/// Peer review, 2026-11-11: a scene was asked of actors the game's own command would refuse. An
/// actor without its 3D here (its cell not loaded), or dead, is not made to talk; the log says so.
#[test]
fn a_scene_is_not_played_for_an_actor_the_game_would_refuse() {
    let mut p = Pair::new();
    p.hd.fight = None;
    p.gd.fight = None;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    // On the guest the raider is placed; the gecko has no 3D.
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.frames(30);
    p.host.script_start_conversation(RAIDER, GECKO, MQ);
    p.frames(60);
    assert_eq!(p.guest.conversations(RAIDER).0, 0);
    assert_eq!((p.gd.scenes_played, p.gd.scenes_missed), (0, 1));
    // A dead one likewise; and each new scene of the same actor is its own (the count).
    p.guest.place_actor(GECKO, HOUSE, [0.0; 3], 0.0);
    p.guest.kill(GECKO);
    p.host.script_start_conversation(RAIDER, GECKO, MQ);
    p.frames(60);
    assert_eq!((p.gd.scenes_played, p.gd.scenes_missed), (0, 2));
    assert!(p.gd.healed_notes.iter().any(|l| l.contains("Refused")));
}

/// D5's condition 6: with scenes switched off every game runs its own, as before.
#[test]
fn with_scenes_off_every_game_runs_its_own_conversations() {
    let mut p = Pair::new();
    p.hd.fight = None;
    p.gd.fight = None;
    for s in [&mut p.host, &mut p.guest] {
        coop_engine::commands::set_scenes_on(&mut s.e, false);
        s.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
        s.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    }
    p.frames(30);
    p.guest.script_start_conversation(RAIDER, GECKO, MQ);
    assert_eq!(p.guest.conversations(RAIDER).0, 1);
    p.host.script_start_conversation(RAIDER, GECKO, MQ);
    p.frames(60);
    assert_eq!(p.guest.conversations(RAIDER).0, 1);
    assert_eq!((p.gd.scenes_played, p.gd.scenes_missed), (0, 0));
}

/// D5's condition 6 for following: each part has its switch. Plain following walks a strayed
/// actor back and does nothing else: no keeping it there, no bringing, no packages.
#[test]
fn plain_following_neither_leads_nor_brings_nor_gives_packages() {
    let mut p = Pair::new();
    {
        let f = p.gd.fight.as_mut().unwrap();
        f.follow = true;
        f.lead_on = false;
        f.bring_on = false;
        f.packages_on = false;
    }
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.host.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [900.0, 900.0, 0.0], 2.0);
    p.guest.place_actor(GECKO, TOWN, [5.0, 5.0, 0.0], 0.0);
    p.host.set_own_package(RAIDER, LEAD_OUT);
    p.guest.set_own_package(RAIDER, STAY_IN);
    p.frames(60);
    for _ in 0..3 {
        p.guest.move_actor(RAIDER, [610.0, 20.0, 0.0], 2.0);
        p.frames(120);
    }
    let f = p.guest_fight();
    assert!(f.nudges >= 3, "{}", f.nudges);
    assert_eq!((f.brings, f.package_asks), (0, 0));
    assert_eq!(p.guest.package(RAIDER), STAY_IN);
    assert_eq!(p.guest.actor_view(GECKO).pos, [5.0, 5.0, 0.0]);
}

/// Peer review, 2026-11-11: a copy that leaves again each time it is brought is not brought for
/// ever, and a dead copy is not brought at all.
#[test]
fn a_copy_is_brought_a_few_times_at_most() {
    let mut p = Pair::new();
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [300.0, 300.0, 0.0], 0.0);
    for _ in 0..6 {
        p.guest.place_actor(RAIDER, TOWN, [900.0, 900.0, 0.0], 2.0);
        p.frames(80);
    }
    assert_eq!(p.guest_fight().brings, fight::BRING_AT_MOST);
    assert!(p
        .guest_fight()
        .lines
        .iter()
        .any(|l| l.contains("left to this game from now")));
}

impl Pair {
    /// The host's avatar in the guest's game.
    fn host_avatar(&self) -> u32 {
        self.gd.ladder.as_ref().unwrap().avatar_ids()[0].1
    }

    /// A followed raider near the guest, run by the host's game (fights shared up to damage: no
    /// threat director turning the host's raider).
    fn followed() -> Pair {
        let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
        p.gd.fight.as_mut().unwrap().follow = true;
        p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
        p.guest.move_player(HOUSE, [0.0; 3], 0.0);
        p.guest.place_actor(RAIDER, HOUSE, [30.0, 20.0, 0.0], 0.5);
        p.frames(40);
        p
    }
}

/// PC run 30 (the owner, 2026-11-14): an enemy faced one player while it shot the other. The
/// guest's copy of an actor the host runs fights the target the host's actor fights (the host's
/// avatar here when the host's actor fights the host), never its own choice; its own attacks are
/// refused and the host's actor's attacks are replayed on it.
#[test]
fn a_guests_copy_fights_whom_the_hosts_actor_fights_and_attacks_when_it_attacks() {
    let mut p = Pair::followed();
    let (host_avatar, guest_avatar) = (p.host_avatar(), p.guest_avatar());
    // Both games' raiders start a fight: the host's with the host, the copy with the guest.
    p.host.set_combat(RAIDER, PLAYER);
    p.guest.set_combat(RAIDER, PLAYER);
    p.frames(3);
    // The copy's group did not know the host's avatar: it was started on a fight with it.
    assert_eq!(p.guest.actor_view(RAIDER).target, host_avatar);
    assert_eq!(p.guest_fight().puppets.starts, 1);
    // Its own choice of the guest is not taken.
    p.guest.choose_target(RAIDER, PLAYER);
    assert_eq!(p.guest.actor_view(RAIDER).target, host_avatar);
    // Its own attack is refused; the host's raider's attack is replayed on it.
    assert!(!p.guest.queue_attack(RAIDER, 0x20));
    assert_eq!(p.guest.attacks_queued(RAIDER), 0);
    assert!(p.host.queue_attack(RAIDER, 0x21));
    p.frames(3);
    assert_eq!(p.guest.attacks_queued(RAIDER), 1);
    assert_eq!(p.guest.queued_group(RAIDER), 0x21);
    assert_eq!(p.guest_fight().puppets.replays, 1);
    // Once, however many states carry it.
    p.frames(10);
    assert_eq!(p.guest.attacks_queued(RAIDER), 1);
    // The host's raider turns on the guest: so does the copy, on this game's own player.
    p.host.finish_attack(RAIDER);
    p.guest.finish_attack(RAIDER);
    p.host.set_combat(RAIDER, guest_avatar);
    p.frames(3);
    assert_eq!(p.guest.actor_view(RAIDER).target, PLAYER);
    assert_eq!(p.guest_fight().puppets.turns, 1);
    assert!(p.host.queue_attack(RAIDER, 0x20));
    p.frames(3);
    assert_eq!(p.guest.attacks_queued(RAIDER), 2);
    // The host's own raider is its own: its attacks and target are never touched.
    assert_eq!(p.host.attacks_queued(RAIDER), 2);
    assert_eq!(p.host.actor_view(RAIDER).target, guest_avatar);
    p.story_is_whole();
}

/// D6, condition 2: a copy whose owner's actor fights nobody does not fight on here; it is given
/// a moment, since the host's game takes up a fight a player begins here a little later.
#[test]
fn a_copy_whose_owner_stops_fighting_stops_too() {
    let mut p = Pair::followed();
    p.guest.set_combat(RAIDER, PLAYER);
    let frames = (fight::CALM_AFTER_MS / 50) as u32;
    p.frames(frames - 5);
    assert!(p.guest.actor_view(RAIDER).in_combat);
    // The host's raider takes the fight up: it goes on.
    p.host.set_combat(RAIDER, p.guest_avatar());
    p.frames(frames + 10);
    assert!(p.guest.actor_view(RAIDER).in_combat);
    assert_eq!(p.guest_fight().puppets.stops, 0);
    // The host's raider is done: so is the copy, a moment later.
    p.host.set_combat(RAIDER, 0);
    p.frames(frames - 5);
    assert!(p.guest.actor_view(RAIDER).in_combat);
    p.frames(10);
    assert!(!p.guest.actor_view(RAIDER).in_combat);
    assert_eq!(p.guest_fight().puppets.stops, 1);
    p.story_is_whole();
}

/// D6, condition 3: a copy in a conversation here is its own game's: its attacks are not
/// refused; nor is a copy given back to its game when the host no longer runs the actor.
#[test]
fn a_copy_left_to_a_conversation_attacks_as_its_game_has_it() {
    let mut p = Pair::followed();
    assert!(!p.guest.queue_attack(RAIDER, 0x20));
    p.guest.set_in_dialogue(RAIDER, true);
    p.frames(2);
    assert!(p.guest.queue_attack(RAIDER, 0x20));
    assert_eq!(p.guest.attacks_queued(RAIDER), 1);
    // Back to following: refused again.
    p.guest.set_in_dialogue(RAIDER, false);
    p.guest.finish_attack(RAIDER);
    p.frames(2);
    assert!(!p.guest.queue_attack(RAIDER, 0x20));
    // Without shared fights a copy is its game's own.
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [30.0, 20.0, 0.0], 0.5);
    p.frames(40);
    assert!(p.guest.queue_attack(RAIDER, 0x20));
}

/// After a hand-off the host's copy of the actor a guest runs fights whom the guest's actor
/// fights and replays its attacks, and what it does to the guest's avatar is not sent to the
/// guest as well (the guest's game did that blow to its own player).
#[test]
fn the_hosts_copy_of_an_actor_a_guest_runs_fights_as_the_guests_actor_does() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    let guest_avatar = p.guest_avatar();
    // The guest's raider fights the guest; the host's copy chose the host.
    p.guest.set_combat(RAIDER, PLAYER);
    p.host.set_combat(RAIDER, PLAYER);
    p.frames(3);
    assert_eq!(p.host.actor_view(RAIDER).target, guest_avatar);
    assert!(!p.host.queue_attack(RAIDER, 0x20));
    assert!(p.guest.queue_attack(RAIDER, 0x22));
    p.frames(3);
    assert_eq!(p.host.queued_group(RAIDER), 0x22);
    assert_eq!(p.host_fight().puppets.replays, 1);
    // Its replayed blow on the guest's avatar is not sent: the guest's raider hit the guest.
    let before = p.guest.actor_view(PLAYER).health;
    p.host.hit(guest_avatar, RAIDER, 25.0);
    p.frames(3);
    assert_eq!(p.guest.actor_view(PLAYER).health, before);
    assert_eq!(p.host_fight().hurts_sent, 0);
    assert_eq!(p.host_fight().theirs_hits_left, 1);
    // The guest's game runs its raider as it has it: its target and attacks are its own.
    assert_eq!(p.guest.actor_view(RAIDER).target, PLAYER);
    assert_eq!(p.guest.attacks_queued(RAIDER), 1);
}

/// A copy whose game does not take up the fight it is asked to start (its group still does not
/// know the owner's target) is asked again, but not every frame.
#[test]
fn a_copy_is_asked_to_take_up_its_owners_fight_at_most_every_second() {
    let mut p = Pair::followed();
    p.guest.set_deaf(RAIDER, true);
    p.host.set_combat(RAIDER, PLAYER);
    p.frames(3);
    assert_eq!(p.guest_fight().puppets.starts, 1);
    let frames = (fight::AIM_START_EVERY_MS / 50) as u32;
    p.frames(frames - 5);
    assert_eq!(p.guest_fight().puppets.starts, 1);
    p.frames(6);
    assert_eq!(p.guest_fight().puppets.starts, 2);
    // Taken up: no more asking.
    p.guest.set_deaf(RAIDER, false);
    p.frames(2 * frames);
    assert_eq!(p.guest.actor_view(RAIDER).target, p.host_avatar());
    assert_eq!(p.guest_fight().puppets.starts, 3);
}

/// A copy whose owner's actor is in combat but has no target now (it lost the one it fought)
/// fights on: only an owner out of combat ends the copy's fight.
#[test]
fn a_copy_whose_owner_searches_for_its_target_fights_on() {
    let mut p = Pair::followed();
    p.guest.set_combat(RAIDER, PLAYER);
    p.host.set_searching(RAIDER);
    p.frames(3 * (fight::CALM_AFTER_MS / 50) as u32);
    assert!(p.guest.actor_view(RAIDER).in_combat);
    assert_eq!(p.guest_fight().puppets.stops, 0);
}

/// With fights shared only as states, every game runs its own fights: the host's copy of an
/// actor a guest runs attacks as the host's game has it.
#[test]
fn without_shared_damage_the_hosts_copy_of_a_guests_actor_is_its_own() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("state")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    p.host.set_combat(RAIDER, PLAYER);
    p.frames(2);
    assert!(p.host.queue_attack(RAIDER, 0x20));
}

/// The session's end gives every copy back to its game: its own attacks and its own movement.
#[test]
fn the_sessions_end_gives_the_copies_back_to_their_games() {
    let mut p = Pair::followed();
    p.host.set_moves(RAIDER, 0x101);
    p.host.move_actor(RAIDER, [10.0, 60.0, 0.0], 0.5);
    p.frames(3);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x101));
    assert!(!p.guest.queue_attack(RAIDER, 0x20));
    let mut f = p.gd.fight.take().unwrap();
    f.close(&mut p.guest.e);
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    assert!(p.guest.queue_attack(RAIDER, 0x20));
}

/// After a hand-off the host's copy goes along the guest's actor's path with its gait; when the
/// guest's game takes an actor over, its copy there has its own movement back.
#[test]
fn the_hosts_copy_goes_along_the_guests_actors_path_and_a_taken_over_copy_is_its_own() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    // Before the hand-off the guest's copy walks with the host's raider.
    p.frames(2);
    p.host.set_moves(RAIDER, 0x101);
    p.host.move_actor(RAIDER, [10.0, 40.0, 0.0], 0.5);
    p.frames(3);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x101));
    p.host.set_moves(RAIDER, 0);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    // The guest's game runs it now: its movement is its own there.
    assert_eq!(p.guest.forced_moves(RAIDER), None);
    // The guest's raider runs: the host's copy runs along behind it.
    p.guest.set_moves(RAIDER, 0x201);
    for i in 1..=10 {
        p.guest
            .move_actor(RAIDER, [10.0, 40.0 + 20.0 * i as f32, 0.0], 0.5);
        p.frames(1);
    }
    assert_eq!(p.host.forced_moves(RAIDER), Some(0x201));
    let behind = p.guest.actor_view(RAIDER).pos[1] - p.host.actor_view(RAIDER).pos[1];
    assert!((20.0..=120.0).contains(&behind), "{behind}");
}

/// In a fight the copy stays where the host's actor is, however long that one stands; its gait is
/// asked of the game once, not every frame.
#[test]
fn a_fighting_copy_stays_where_the_hosts_actor_is() {
    let mut p = Pair::followed();
    // (in a fight even without D7's keeping)
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.set_combat(RAIDER, PLAYER);
    p.host.set_moves(RAIDER, 0x201);
    p.host.move_actor(RAIDER, [10.0, 60.0, 0.0], 0.5);
    p.frames(10);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x201));
    let asked = p.guest.forces(RAIDER);
    p.frames(20);
    assert_eq!(p.guest.forces(RAIDER), asked);
    p.host.set_moves(RAIDER, 0);
    p.frames((fight::SETTLE_MS / 50) as u32 + 20);
    assert_eq!(p.guest.forces(RAIDER), asked + 1);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0));
    // Its own combat moves take it aside: it is back where the host's stands.
    p.guest.move_actor(RAIDER, [110.0, 60.0, 0.0], 0.5);
    p.frames(2);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
}

/// The host's threat director chooses targets for the host's own actors only: one a guest's game
/// runs fights whom that game has it fight.
#[test]
fn the_threat_director_leaves_an_actor_a_guest_runs_alone() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    // The guest's raider goes for the host's avatar there, far off; the guest is nearer.
    let host_avatar = p.host_avatar();
    p.guest.set_combat(RAIDER, host_avatar);
    p.host.set_combat(RAIDER, PLAYER);
    p.frames(60);
    assert_eq!(p.host.actor_view(RAIDER).target, PLAYER);
    assert_eq!(p.host_fight().retargets, 0);
}

/// With the threat slice every game's actors may fight every player: the host's avatar in a
/// guest's game is not a ghost either (that game runs the actors near its player after a
/// hand-off).
#[test]
fn with_the_threat_slice_every_avatar_can_be_fought() {
    let p = in_a_fight("");
    let (host_avatar, guest_avatar) = (p.host_avatar(), p.guest_avatar());
    let mut p = p;
    assert!(!p.host.ghost(guest_avatar));
    assert!(!p.guest.ghost(host_avatar));
}

/// Each game's actors notice only its own player (the others are avatars there). A guest's copy
/// of an actor the host runs that goes for the guest has the host's actor fight the guest's
/// avatar: in both games it fights the guest.
#[test]
fn a_guests_copy_that_goes_for_the_guest_has_the_hosts_actor_fight_the_guest() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.move_player(HOUSE, [0.0; 3], 0.0);
    p.guest.place_actor(RAIDER, HOUSE, [30.0, 20.0, 0.0], 0.5);
    p.frames(40);
    // Going for another actor asks nothing.
    p.guest.place_actor(GECKO, HOUSE, [60.0, 20.0, 0.0], 0.5);
    p.guest.set_combat(RAIDER, GECKO);
    p.frames(6);
    assert_eq!(p.guest_fight().puppets.asks, 0);
    p.guest.set_combat(RAIDER, PLAYER);
    p.frames(6);
    assert_eq!(p.host.actor_view(RAIDER).target, p.guest_avatar());
    assert_eq!(p.guest.actor_view(RAIDER).target, PLAYER);
    assert_eq!(
        (
            p.guest_fight().puppets.asks,
            p.host_fight().puppets.taken_up
        ),
        (1, 1)
    );
    p.frames(3 * (fight::CALM_AFTER_MS / 50) as u32);
    assert!(p.guest.actor_view(RAIDER).in_combat);
    assert_eq!(p.guest_fight().puppets.stops, 0);
    p.story_is_whole();
}

/// After a hand-off, the host's copy of the guest's actor that goes for the host has the guest's
/// actor fight the host's avatar (harness, 2026-11-14: the guest's geckos stood while their copies
/// went for the host and were stopped again and again).
#[test]
fn the_hosts_copy_that_goes_for_the_host_has_the_guests_actor_fight_the_host() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    p.host.set_combat(RAIDER, PLAYER);
    p.frames(6);
    assert_eq!(p.guest.actor_view(RAIDER).target, p.host_avatar());
    assert_eq!(p.host.actor_view(RAIDER).target, PLAYER);
    assert_eq!(
        (
            p.host_fight().puppets.asks,
            p.guest_fight().puppets.taken_up
        ),
        (1, 1)
    );
    p.frames(3 * (fight::CALM_AFTER_MS / 50) as u32);
    assert!(p.host.actor_view(RAIDER).in_combat);
    assert_eq!(p.host_fight().puppets.stops, 0);
}

/// Without the threat slice no game's actor is turned on a player for another game's copy: the
/// copy stops.
#[test]
fn without_the_threat_slice_a_copys_own_fight_is_not_taken_up() {
    let mut p = Pair::followed();
    p.guest.set_combat(RAIDER, PLAYER);
    p.frames(3 * (fight::CALM_AFTER_MS / 50) as u32);
    assert!(!p.host.actor_view(RAIDER).in_combat);
    assert!(!p.guest.actor_view(RAIDER).in_combat);
    assert_eq!(
        (
            p.guest_fight().puppets.asks,
            p.host_fight().puppets.taken_up
        ),
        (0, 0)
    );
}

/// PC run 31: the host's copy of Sunny, at the wells while the host stood at the saloon, was in a
/// cell the host's game had let go: put on her path every frame, she never moved. A copy in a cell
/// this game let go is brought to its place the way an avatar is (`MoveTo`), when that place is
/// within this game's loaded cells, and left alone otherwise; it is not counted as put on its
/// path.
#[test]
fn a_copy_in_a_cell_this_game_let_go_is_brought_or_left_alone() {
    let mut p = Pair::followed();
    let was = p.guest.actor_view(RAIDER).pos;
    p.guest.set_cell_loaded(HOUSE, false);
    // Its owner's place, out of reach of this game's cells: left alone.
    p.host.move_actor(RAIDER, [10.0, 20_000.0, 0.0], 0.5);
    p.frames(10);
    assert!(close(p.guest.actor_view(RAIDER).pos, was));
    // Within reach: brought there.
    p.host.move_actor(RAIDER, [10.0, 400.0, 0.0], 0.5);
    p.frames(10);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    assert!(p.guest_fight().following.brought >= 1);
    assert_eq!(p.guest_fight().following.snaps, 0);
    assert_eq!(p.guest.forced_moves(RAIDER), None);
}

/// PC run 31: Victor walked past the guest; the host's copy was elsewhere (in another space) and
/// never showed, and the host's game took him back from the guest by its copy's place. The host's
/// copy of an actor a guest runs, in another space than that actor while the host's player is in
/// that actor's space and within reach of it, is brought to its place (not more often than every
/// few seconds), and the actor stays the guest's to run.
#[test]
fn the_hosts_copy_in_another_space_is_brought_to_the_guests_actor() {
    let mut p = Pair::with(Cap::Avatar, Some(fight::Cap::parse("damage")));
    p.gd.fight.as_mut().unwrap().follow = true;
    p.host.move_player(HOUSE, [3000.0, 3000.0, 0.0], 0.0);
    p.guest.move_player(HOUSE, [20.0, 20.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.guest.place_actor(RAIDER, HOUSE, [10.0, 20.0, 0.0], 0.5);
    p.frames(80);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    let space = |p: &mut Pair| {
        let copy = coop_engine::actors::actor(&mut p.host.e, RAIDER).unwrap();
        coop_engine::fight::space_of(&mut p.host.e, copy)
    };
    // The host's copy is taken elsewhere: brought back, and the guest still runs it.
    p.host.place_actor(RAIDER, TOWN, [500.0, 500.0, 0.0], 0.5);
    p.frames(10);
    assert_eq!(space(&mut p), HOUSE);
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(h.pos, g.pos), "{h:?} {g:?}");
    assert_eq!(p.host_fight().brings, 1);
    assert!(p.host_fight().theirs().contains(&RAIDER));
    // Again at once: brought only once the few seconds are over.
    p.host.place_actor(RAIDER, TOWN, [500.0, 500.0, 0.0], 0.5);
    p.frames(10);
    assert_eq!(space(&mut p), TOWN);
    p.frames((fight::BRING_EVERY_MS / 50) as u32);
    assert_eq!(space(&mut p), HOUSE);
    assert_eq!(p.host_fight().brings, 2);
    // The host's player far off (out of the cells its game keeps): not brought.
    p.host.move_player(HOUSE, [30_000.0, 30_000.0, 0.0], 0.0);
    p.host.place_actor(RAIDER, TOWN, [500.0, 500.0, 0.0], 0.5);
    p.frames(2 * (fight::BRING_EVERY_MS / 50) as u32);
    assert_eq!(space(&mut p), TOWN);
    assert_eq!(p.host_fight().brings, 2);
}

/// PC run 31: Sunny crouched at the well in the guest's game and stood in the host's once she had
/// stood still a moment (the copy was left to its own AI). A crouched actor's copy stays crouched
/// where it is, however long it stays, even with D7's keeping switched off.
#[test]
fn a_crouched_actors_copy_stays_crouched() {
    let mut p = Pair::followed();
    p.gd.fight.as_mut().unwrap().lead_on = false;
    p.host.set_moves(RAIDER, 0x400);
    p.host.move_actor(RAIDER, [10.0, 40.0, 0.0], 0.5);
    p.frames(3 * (fight::SETTLE_MS / 50) as u32);
    assert_eq!(p.guest.forced_moves(RAIDER), Some(0x400));
    let (h, g) = (p.host.actor_view(RAIDER), p.guest.actor_view(RAIDER));
    assert!(close(g.pos, h.pos), "{g:?} {h:?}");
    // It stands up: a moment later the copy is its own AI's again.
    p.host.set_moves(RAIDER, 0);
    p.host.move_actor(RAIDER, [10.0, 41.0, 0.0], 0.5);
    p.frames(2 * (fight::SETTLE_MS / 50) as u32);
    assert_eq!(p.guest.forced_moves(RAIDER), None);
}
