//! Companions in a party, on two games in one process (docs/coop/SPECS.md S32): a companion
//! follows the player who hired it, through doors, on that player's side, with that player's
//! wheel, and a party save keeps who hired whom.

use std::path::PathBuf;

use coop_core::ids::{FormId, PlayerId};
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine::{addr, companion, forms};
use coop_engine_session::companion::{kept_of, NO_KEEPER};
use coop_engine_session::fight::{self, Fight};
use coop_engine_session::ladder::{Cap, Ladder, DEFAULT_AVATAR_BASE};
use coop_engine_session::party::{self, Part};
use coop_engine_session::{echo, gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const HOUSE: u32 = 0x0010_5a00;
const TOWN: u32 = 0x0010_5a01;
/// Two companions and an enemy.
const CASS: u32 = 0x0010_4f30;
const BOONE: u32 = 0x0010_4f31;
const RAIDER: u32 = 0x0010_4f25;
/// A follow package aimed at "the player".
const FOLLOW: u32 = 0x0010_6a10;
const PLAYER: u32 = 0x14;

fn spec() -> StorySpec {
    StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        globals: vec![(0x39, 0.0)],
        cells: vec![HOUSE, TOWN],
        actors: vec![CASS, BOONE, RAIDER],
        packages: vec![FOLLOW],
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

/// A game with the two companions near its player, each running the follow package.
fn game() -> Story {
    let mut s = Story::new(&spec());
    s.move_player(HOUSE, [0.0, 0.0, 0.0], 0.0);
    let player = s.e.mem.u32(addr::PLAYER_SINGLETON);
    s.aim_package_at(FOLLOW, player);
    s.set_package_type(FOLLOW, 1);
    for (id, x) in [(CASS, 100.0), (BOONE, -100.0)] {
        s.place_actor(id, HOUSE, [x, 0.0, 0.0], 0.0);
        s.set_own_package(id, FOLLOW);
    }
    s
}

struct Pair {
    net: LoopbackNet,
    host: Story,
    hd: HostDriver,
    hnet: LoopbackEndpoint,
    guest: Story,
    gd: GuestDriver,
    gnet: LoopbackEndpoint,
    t: u64,
}

impl Pair {
    /// Two games, the host's ladder whole, the guest's up to `ladder`, fights shared as the
    /// viewer's default.
    fn with(ladder: Cap) -> Pair {
        let net = LoopbackNet::new();
        let mut host = game();
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], [0x39]);
        hd.ladder = Some(Ladder::new(false, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
        hd.fight = Some(Fight::new(false, fight::Cap::parse("")).quiet());
        hd.start(&mut host.e);
        let mut guest = game();
        gate_guest(&mut guest.e);
        let mut gd = GuestDriver::new(PlayerId(1), hello());
        gd.ladder = Some(Ladder::new(true, ladder, DEFAULT_AVATAR_BASE).quiet());
        gd.fight = Some(Fight::new(true, fight::Cap::parse("")).quiet());
        Pair {
            hnet: net.endpoint(PlayerId::HOST),
            gnet: net.endpoint(PlayerId(1)),
            net,
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
            self.gd.step(&mut self.guest.e, &mut self.gnet, self.t);
        }
    }

    /// The guest hires `id`: its line runs in the host's game (as the host's game runs a guest's
    /// line), and that line's `SetPlayerTeammate 1` is the guest's doing.
    fn guest_hires(&mut self, id: u32) {
        let a = coop_engine::actors::actor(&mut self.host.e, id).unwrap();
        companion::with_hirer(&mut self.host.e, 1, |e| {
            e.call(addr::ACTOR_SET_PLAYER_TEAMMATE, &[a, 1]);
        });
    }

    /// The avatar of `p` in `game`'s ladder.
    fn avatar(ladder: &Ladder, p: PlayerId) -> u32 {
        ladder
            .avatar_ids()
            .into_iter()
            .find(|(q, _)| *q == p)
            .map(|(_, a)| a)
            .expect("an avatar")
    }

    fn story_is_whole(&mut self) {
        assert_eq!(echo(&mut self.guest.e, &self.gd.sync.store), vec![]);
        assert_eq!(self.gd.sync.desyncs_detected, 0);
        assert_eq!(self.gd.refused, 0);
    }

    /// The host's record of companion `id`.
    fn record(&self, id: u32) -> Option<coop_engine_session::companion::Kept> {
        self.gd
            .sync
            .store
            .iter()
            .find(|(k, _)| k.form.0 == id)
            .and_then(|(k, _)| {
                self.gd
                    .sync
                    .store
                    .iter()
                    .filter(|(k2, _)| k2.form == k.form)
                    .find_map(|(_, en)| kept_of(&en.field))
            })
    }
}

fn ptr(s: &mut Story, id: u32) -> u32 {
    forms::form(&mut s.e, id).unwrap()
}

#[test]
fn a_companion_a_guest_hires_follows_that_guest_and_the_hosts_follows_the_host() {
    let mut p = Pair::with(Cap::Avatar);
    p.frames(40);
    // The host hires Boone in its own game; the guest hires Cass.
    p.host.script_set_teammate(BOONE, true);
    p.guest_hires(CASS);
    p.frames(10);
    assert_eq!(p.hd.companions.keeper(CASS), Some(PlayerId(1)));
    assert_eq!(p.hd.companions.keeper(BOONE), Some(PlayerId::HOST));
    assert_eq!(p.record(CASS).map(|k| k.slot), Some(1));
    // On the host: Cass's follow step follows the guest's avatar, Boone's the host's player.
    let guest_avatar = Pair::avatar(p.hd.ladder.as_ref().unwrap(), PlayerId(1));
    let (av, host_player) = (ptr(&mut p.host, guest_avatar), ptr(&mut p.host, PLAYER));
    for id in [CASS, BOONE] {
        p.host.set_follow_target(id, host_player);
        p.host.process_follow(addr::HIGH_PROCESS_FOLLOW, id);
    }
    assert_eq!(p.host.followed(CASS), av);
    assert_eq!(p.host.followed(BOONE), host_player);
    // On the guest: Cass is its player's teammate and follows it; Boone follows the host's
    // avatar there.
    assert!(p.gd.companions.mine().contains(&CASS));
    assert_eq!(companion::teammate(&mut p.guest.e, CASS), Some(true));
    assert_eq!(companion::teammate(&mut p.guest.e, BOONE), Some(false));
    let host_avatar = Pair::avatar(p.gd.ladder.as_ref().unwrap(), PlayerId::HOST);
    let (hav, guest_player) = (ptr(&mut p.guest, host_avatar), ptr(&mut p.guest, PLAYER));
    for id in [CASS, BOONE] {
        p.guest.set_follow_target(id, guest_player);
        p.guest.process_follow(addr::MIDDLE_HIGH_PROCESS_FOLLOW, id);
    }
    assert_eq!(p.guest.followed(CASS), guest_player);
    assert_eq!(p.guest.followed(BOONE), hav);
    // With hand-off, Cass is the guest's game's to run; Boone, keeping company with the host's
    // player, the host's.
    p.frames(20);
    let owner = |p: &Pair, id: u32| {
        p.hd.sync
            .live
            .actors
            .own
            .get(FormId(id))
            .and_then(|r| r.owner)
    };
    assert_eq!(owner(&p, CASS), Some(PlayerId(1)));
    assert_eq!(owner(&p, BOONE), Some(PlayerId::HOST));
    p.story_is_whole();
}

#[test]
fn a_hire_outside_a_line_is_the_player_who_last_spoke_with_the_actor() {
    let mut p = Pair::with(Cap::Avatar);
    p.frames(40);
    // A script hires Cass a moment after the guest spoke with her; Boone after nobody did.
    p.hd.companions.on_talk(CASS, PlayerId(1));
    p.host.script_set_teammate(CASS, true);
    p.host.script_set_teammate(BOONE, true);
    p.frames(5);
    assert_eq!(p.hd.companions.keeper(CASS), Some(PlayerId(1)));
    assert_eq!(p.hd.companions.keeper(BOONE), Some(PlayerId::HOST));
    // Let go: nobody's, and the guest's game takes back the flag it set.
    p.host.script_set_teammate(CASS, false);
    p.frames(5);
    assert_eq!(p.hd.companions.keeper(CASS), None);
    assert_eq!(p.record(CASS).map(|k| k.slot), Some(NO_KEEPER));
    assert!(!p.gd.companions.mine().contains(&CASS));
    assert_eq!(companion::teammate(&mut p.guest.e, CASS), Some(false));
    p.story_is_whole();
}

#[test]
fn a_following_companion_is_brought_through_a_door_and_a_waiting_one_is_not() {
    // The guest's ladder stops before the warp: its player walks about its own game.
    let mut p = Pair::with(Cap::Log);
    p.frames(40);
    p.guest_hires(CASS);
    p.guest_hires(BOONE);
    p.frames(10);
    assert!(p.gd.companions.mine().contains(&CASS));
    // Boone is told to wait (the wheel).
    let mut s = companion::switches(&mut p.guest.e, BOONE).unwrap();
    s[addr::WHEEL_WAITING] = 1.0;
    companion::set_switches(&mut p.guest.e, BOONE, s);
    // The guest's player goes through a door into the town.
    p.guest.move_player(TOWN, [5000.0, 0.0, 0.0], 0.0);
    p.frames(10);
    let town = forms::form(&mut p.guest.e, TOWN).unwrap();
    let cell = |s: &mut Story, id: u32| {
        let a = forms::form(&mut s.e, id).unwrap();
        s.e.mem.u32(a + addr::REF_CELL_OFFSET)
    };
    assert_eq!(cell(&mut p.guest, CASS), town, "Cass came along");
    assert_ne!(cell(&mut p.guest, BOONE), town, "Boone waits");
    let at = p.guest.actor_view(CASS).pos;
    assert!((at[0] - 5000.0).abs() < 1.0 && at[1] < 0.0, "{at:?}");
    assert_eq!(p.gd.companions.brought, 1);
    // A fast travel far within the same space: brought again, once the wait is over.
    p.guest.move_player(TOWN, [40000.0, 0.0, 0.0], 0.0);
    p.frames(80);
    assert!((p.guest.actor_view(CASS).pos[0] - 40000.0).abs() < 1.0);
    assert_eq!(p.gd.companions.brought, 2);
    // Let go by the game's own command: stays where it is.
    p.host.script_set_teammate(CASS, false);
    p.frames(5);
    p.guest.move_player(HOUSE, [0.0, 0.0, 0.0], 0.0);
    p.frames(80);
    assert_eq!(cell(&mut p.guest, CASS), town);
    assert_eq!(p.gd.companions.brought, 2);
}

#[test]
fn a_companion_does_not_hurt_a_player_and_an_enemy_still_does() {
    let mut p = Pair::with(Cap::Avatar);
    p.frames(40);
    p.guest_hires(CASS);
    p.host.script_set_teammate(BOONE, true);
    p.frames(10);
    p.host.place_actor(RAIDER, HOUSE, [50.0, 50.0, 0.0], 0.0);
    // On the host, the host's own companion and the guest's swing at the host's player.
    let before = p.host.actor_view(PLAYER).health;
    p.host.hit(PLAYER, CASS, 10.0);
    p.host.hit(PLAYER, BOONE, 10.0);
    assert_eq!(p.host.actor_view(PLAYER).health, before);
    // An enemy's blow lands.
    p.host.hit(PLAYER, RAIDER, 10.0);
    assert_eq!(p.host.actor_view(PLAYER).health, before - 10.0);
    // On the guest, a companion's blow on its own player is dropped too.
    let before = p.guest.actor_view(PLAYER).health;
    p.guest.hit(PLAYER, CASS, 10.0);
    assert_eq!(p.guest.actor_view(PLAYER).health, before);
    p.story_is_whole();
}

#[test]
fn the_wheel_of_the_guest_who_hired_it_switches_the_companion_in_every_game() {
    let mut p = Pair::with(Cap::Avatar);
    p.frames(40);
    p.guest_hires(CASS);
    p.host.script_set_teammate(BOONE, true);
    p.frames(10);
    let before = p.host.evaluations(CASS);
    // The guest's wheel: aggressive, keep distance, waiting.
    companion::set_switches(&mut p.guest.e, CASS, [1.0, 1.0, 0.0, 1.0]);
    p.frames(20);
    assert_eq!(
        companion::switches(&mut p.host.e, CASS),
        Some([1.0, 1.0, 0.0, 1.0])
    );
    assert!(p.host.evaluations(CASS) > before, "it thought again");
    assert_eq!(
        p.record(CASS).map(|k| k.switches),
        Some([1.0, 1.0, 0.0, 1.0])
    );
    assert!(p.hd.companions.claims_taken >= 1);
    // The guest's own switching of the host's companion is not proposed (not its to switch).
    companion::set_switches(&mut p.guest.e, BOONE, [1.0, 0.0, 0.0, 0.0]);
    p.frames(20);
    assert_eq!(
        companion::switches(&mut p.host.e, BOONE),
        Some([0.0, 0.0, 0.0, 0.0])
    );
    assert_eq!(p.hd.companions.claims_refused, 0);
    p.story_is_whole();
}

// ---- Party saves ----

fn party_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn who_hired_a_companion_survives_a_party_save_and_a_load() {
    let dir = party_dir("companions-save");
    let mut p = Pair::with(Cap::Avatar);
    p.hd.social.party.dir = Some(dir.clone());
    p.gd.social.party.key = party::key_of("Sunny");
    p.frames(40);
    p.guest_hires(CASS);
    p.frames(10);
    companion::set_switches(&mut p.guest.e, CASS, [0.0, 1.0, 1.0, 0.0]);
    p.frames(140);
    assert_eq!(p.host.save("Save 4"), None);
    p.frames(2);
    let key = party::key_of("Sunny");
    let file = dir.join(format!("Save 4.{key:016x}.nvcoop"));
    let part = Part::decode(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(part.companions, vec![(CASS, [0.0, 1.0, 1.0, 0.0])]);
    // The load: the save's teammates are the host's until each guest's part claims its own.
    assert_eq!(p.host.load("Save 4"), None);
    p.t += 50;
    p.hd.step(&mut p.host.e, &mut p.hnet, p.t);
    assert_eq!(p.hd.companions.keeper(CASS), Some(PlayerId::HOST));
    p.frames(40);
    assert_eq!(p.gd.social.party.restored, 1);
    assert_eq!(p.hd.companions.keeper(CASS), Some(PlayerId(1)));
    assert!(p.gd.companions.mine().contains(&CASS));
    assert_eq!(
        companion::switches(&mut p.host.e, CASS),
        Some([0.0, 1.0, 1.0, 0.0])
    );
    p.story_is_whole();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_guest_that_comes_back_with_a_new_game_gets_its_companion_back() {
    let mut p = Pair::with(Cap::Avatar);
    p.gd.social.party.key = party::key_of("Sunny");
    p.frames(40);
    p.guest_hires(CASS);
    p.frames(140);
    // The guest's game is closed; a new one joins in its place.
    p.hd.sync.disconnect(PlayerId(1));
    p.net.flush(PlayerId(1));
    p.net.flush(PlayerId::HOST);
    p.t += 50;
    p.hd.step(&mut p.host.e, &mut p.hnet, p.t);
    p.net.flush(PlayerId(1));
    p.guest = game();
    gate_guest(&mut p.guest.e);
    p.gd = GuestDriver::new(PlayerId(1), hello());
    p.gd.social.party.key = party::key_of("Sunny");
    p.gd.ladder = Some(Ladder::new(true, Cap::Avatar, DEFAULT_AVATAR_BASE).quiet());
    p.gd.fight = Some(Fight::new(true, fight::Cap::parse("")).quiet());
    assert_eq!(companion::teammate(&mut p.guest.e, CASS), Some(false));
    p.frames(60);
    assert_eq!(p.hd.companions.keeper(CASS), Some(PlayerId(1)));
    assert!(p.gd.companions.mine().contains(&CASS));
    assert_eq!(companion::teammate(&mut p.guest.e, CASS), Some(true));
    p.story_is_whole();
}
