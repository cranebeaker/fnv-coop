//! Conversations, party saves, rejoining and the rest vote between games in one process, joined
//! over coop-core's loopback transport (docs/coop/SPECS.md S14 to S17). Each game is the story
//! kit's: a small story in engine memory with stand-ins for the game's functions.

use std::path::PathBuf;

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::testkit::{Story, StorySpec};
use coop_engine::{forms, gate, live, talk};
use coop_engine_session::party::{self, Part};
use coop_engine_session::{gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

/// A quest of the shared story, and one each player runs for themself.
const MQ: u32 = 0x0010_4c0c;
const OWN: u32 = 0x0010_4c1c;
/// The game clock's hour.
const HOUR: u32 = 0x38;
const SUNNY: u32 = 0x0010_4e85;
const CHET: u32 = 0x0010_4f25;
const SALOON: u32 = 0x0010_5a00;
const STORE: u32 = 0x0010_5a01;
/// Sunny's line of the shared story: said, it sets MQ's stage 10; ended, stage 20.
const SHARED_LINE: u32 = 0x0010_9001;
/// A line of the player's own quest: said, it sets OWN's stage 5.
const OWN_LINE: u32 = 0x0010_9002;
/// A second line of the shared story, with no result.
const PLAIN_LINE: u32 = 0x0010_9003;

fn hello() -> Hello {
    let plugins = vec!["FalloutNV.esm".to_string()];
    Hello {
        proto: PROTOCOL_VERSION,
        build_hash: 1,
        rules_hash: 2,
        mods_hash: plugin_list_hash(&plugins),
        exe: "1.4.0.525".into(),
        volatile_hash: 3,
        plugins,
    }
}

/// A game: everyone in the saloon with Sunny; Chet in the store.
fn story() -> Story {
    let mut s = Story::new(&StorySpec {
        quests: vec![(MQ, vec![5, 10, 20, 30]), (OWN, vec![5, 9])],
        globals: vec![(HOUR, 8.0)],
        vars: vec![(OWN, vec![1])],
        cells: vec![SALOON, STORE],
        actors: vec![SUNNY, CHET],
        ..StorySpec::default()
    });
    s.add_info(
        SHARED_LINE,
        MQ,
        "Head on up to the saloon. | I will meet you there.",
        "Where is Sunny?",
    );
    s.set_info_result(SHARED_LINE, false, MQ, 10);
    s.set_info_result(SHARED_LINE, true, MQ, 20);
    s.add_info(OWN_LINE, OWN, "You have been out cold.", "");
    s.set_info_result(OWN_LINE, false, OWN, 5);
    s.add_info(PLAIN_LINE, MQ, "Howdy.", "");
    s.place(SUNNY, SALOON);
    s.place(CHET, STORE);
    s.move_player(SALOON, [0.0; 3], 0.0);
    s.set_hour_global(HOUR);
    s
}

fn guest_story() -> Story {
    let mut g = story();
    gate_guest(&mut g.e);
    gate::set_own_quests(&mut g.e, [OWN]);
    g
}

struct Guest {
    game: Story,
    driver: GuestDriver,
    net: LoopbackEndpoint,
}

struct World {
    net: LoopbackNet,
    host: Story,
    hd: HostDriver,
    hostnet: LoopbackEndpoint,
    guests: Vec<Guest>,
    t: u64,
}

impl World {
    fn new(guests: &[&str]) -> World {
        let net = LoopbackNet::new();
        let hostnet = net.endpoint(PlayerId::HOST);
        let mut host = story();
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], []);
        hd.set_clock([HOUR]);
        hd.start(&mut host.e);
        let mut w = World {
            net,
            host,
            hd,
            hostnet,
            guests: Vec::new(),
            t: 0,
        };
        for name in guests {
            w.join(guest_story(), name);
        }
        w
    }

    /// A game joins as the next player, its player known by `name`.
    fn join(&mut self, game: Story, name: &str) -> usize {
        let player = PlayerId(self.guests.len() as u8 + 1);
        let mut driver = GuestDriver::new(player, hello());
        driver.social.party.key = party::key_of(name);
        driver.social.party.own_quests = vec![OWN];
        self.guests.push(Guest {
            game,
            driver,
            net: self.net.endpoint(player),
        });
        self.guests.len() - 1
    }

    fn run_every(&mut self, frames: u32, ms: u64) {
        for _ in 0..frames {
            self.t += ms;
            self.hd.step(&mut self.host.e, &mut self.hostnet, self.t);
            for g in &mut self.guests {
                g.driver.step(&mut g.game.e, &mut g.net, self.t);
            }
        }
    }

    fn run(&mut self, frames: u32) {
        self.run_every(frames, 50);
    }

    /// Every guest holds the host's story: the shared quest's stage, the replica's hash.
    fn assert_in_step(&mut self) {
        let stage = forms::quest_stage(&mut self.host.e, MQ);
        for (i, g) in self.guests.iter_mut().enumerate() {
            assert_eq!(forms::quest_stage(&mut g.game.e, MQ), stage, "guest {i}");
            assert_eq!(g.driver.sync.desyncs_detected, 0, "guest {i}");
            assert!(g.driver.sync.hash_checks > 0, "guest {i}");
            assert_eq!(
                g.driver.sync.store.rev(),
                self.hd.sync.store.rev(),
                "guest {i}"
            );
        }
    }
}

/// A folder of this test's own under the build's temporary folder, empty.
fn party_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

// ---- Conversations ----

#[test]
fn a_guests_line_runs_once_on_the_host_and_the_others_see_it() {
    let mut w = World::new(&["Sunny", "Chet"]);
    w.run(20);
    // Guest 1 talks to Sunny; guest 2 stands by; the host is in the store.
    w.host.move_player(STORE, [0.0; 3], 0.0);
    assert!(w.guests[0].game.talk_to(SUNNY));
    w.guests[0].game.say(SHARED_LINE, false);
    w.run(10);
    // The result ran in the host's game, not in the talker's, and came back as the story.
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(10));
    assert_eq!(w.hd.social.enactor.enacted, 1);
    assert_eq!(talk::counts(&mut w.guests[0].game.e), (1, 0, 0));
    assert_eq!(w.guests[0].game.said(SHARED_LINE), 1);
    assert_eq!(w.guests[0].driver.social.talker.opened, 1);
    w.assert_in_step();
    // The guest standing by saw what was chosen and each response; the host, elsewhere, did not.
    assert_eq!(
        w.guests[1].game.messages(),
        [
            "Player 2: Where is Sunny?",
            "Head on up to the saloon.",
            "I will meet you there."
        ]
    );
    assert_eq!(w.guests[1].driver.social.talker.lines_shown, 1);
    assert!(w.host.messages().is_empty());
    assert_eq!(w.hd.social.talker.lines_far, 1);
    assert!(w.guests[0].game.messages().is_empty());
    // The line ends, then the conversation: the end result runs once, the NPC is free again.
    w.guests[0].game.say(SHARED_LINE, true);
    w.run(10);
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(20));
    assert_eq!(w.hd.social.enactor.enacted, 2);
    assert_eq!(w.hd.sync.live.dialogue.open_sessions(), 1);
    w.guests[0].game.leave();
    w.run(10);
    assert_eq!(w.hd.sync.live.dialogue.open_sessions(), 0);
    assert_eq!(w.hd.social.enactor.left, 0);
    w.assert_in_step();
    // Nothing ran twice: the stage's own result script ran on the host only.
    assert_eq!(w.guests[0].game.scripts_run(), 0);
    assert_eq!(w.guests[1].game.scripts_run(), 0);
}

#[test]
fn a_line_whose_end_never_comes_is_finished_by_the_next_line_and_by_the_goodbye() {
    let mut w = World::new(&["Sunny"]);
    w.run(20);
    assert!(w.guests[0].game.talk_to(SUNNY));
    w.guests[0].game.say(PLAIN_LINE, false);
    w.guests[0].game.say(SHARED_LINE, false);
    w.guests[0].game.leave();
    w.run(30);
    // Both lines were picked in order; the second's end result ran when the conversation ended.
    assert_eq!(w.guests[0].driver.social.talker.picks, 2);
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(20));
    assert_eq!(w.hd.sync.live.dialogue.open_sessions(), 0);
    assert_eq!(
        w.host.messages(),
        [
            "Howdy.",
            "Player 2: Where is Sunny?",
            "Head on up to the saloon.",
            "I will meet you there."
        ]
    );
    w.assert_in_step();
}

#[test]
fn the_hosts_line_is_seen_by_a_guest_and_runs_in_the_hosts_game_only() {
    let mut w = World::new(&["Sunny"]);
    w.run(20);
    assert!(w.host.talk_to(SUNNY));
    w.host.say(SHARED_LINE, false);
    w.host.say(SHARED_LINE, true);
    w.host.leave();
    w.run(20);
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(20));
    assert_eq!(
        w.hd.social.enactor.enacted, 0,
        "the host's game ran it itself"
    );
    assert_eq!(w.hd.social.talker.opened, 1);
    assert_eq!(
        w.guests[0].game.messages(),
        [
            "Player 1: Where is Sunny?",
            "Head on up to the saloon.",
            "I will meet you there."
        ]
    );
    assert_eq!(w.hd.sync.live.dialogue.open_sessions(), 0);
    w.assert_in_step();
    assert_eq!(w.guests[0].game.scripts_run(), 0);
}

#[test]
fn a_line_of_the_players_own_quest_stays_in_that_players_game() {
    let mut w = World::new(&["Sunny"]);
    w.run(20);
    assert!(w.guests[0].game.talk_to(SUNNY));
    w.guests[0].game.say(OWN_LINE, false);
    w.guests[0].game.say(OWN_LINE, true);
    w.guests[0].game.leave();
    w.run(20);
    assert_eq!(forms::quest_stage(&mut w.guests[0].game.e, OWN), Some(5));
    assert_eq!(forms::quest_stage(&mut w.host.e, OWN), Some(0));
    assert_eq!(w.guests[0].driver.social.talker.opened, 0);
    assert_eq!(w.hd.social.enactor.enacted + w.hd.social.enactor.left, 0);
    assert!(w.host.messages().is_empty());
}

#[test]
fn an_npc_is_in_one_conversation_at_a_time() {
    let mut w = World::new(&["Sunny", "Chet"]);
    w.run(20);
    assert!(w.guests[0].game.talk_to(SUNNY));
    w.guests[0].game.say(PLAIN_LINE, false);
    w.run(10);
    // The others' games know: no menu opens on Sunny, and the player is told why.
    assert!(!w.guests[1].game.talk_to(SUNNY));
    assert!(!w.host.talk_to(SUNNY));
    w.run(2);
    assert_eq!(w.guests[1].driver.social.talker.blocked, 1);
    assert_eq!(w.hd.social.talker.blocked, 1);
    assert_eq!(
        w.guests[1].game.messages().last().map(String::as_str),
        Some("Another player is talking to them.")
    );
    // Another NPC is free.
    w.guests[1].game.place(CHET, SALOON);
    assert!(w.guests[1].game.talk_to(CHET));
    w.guests[1].game.leave();
    // Once the first conversation is over, Sunny is free again.
    w.guests[0].game.say(PLAIN_LINE, true);
    w.guests[0].game.leave();
    w.run(10);
    assert!(w.guests[1].game.talk_to(SUNNY));
}

#[test]
fn two_players_who_start_at_once_the_host_lets_one_talk_and_closes_the_others_menu() {
    let mut w = World::new(&["Sunny", "Chet"]);
    w.run(20);
    // The same frame: neither game knows of the other's conversation yet.
    assert!(w.guests[0].game.talk_to(SUNNY));
    assert!(w.guests[1].game.talk_to(SUNNY));
    w.guests[0].game.say(SHARED_LINE, false);
    w.guests[1].game.say(SHARED_LINE, false);
    w.run(10);
    assert_eq!(w.guests[0].driver.social.talker.opened, 1);
    assert_eq!(w.guests[1].driver.social.talker.denied, 1);
    assert_eq!(talk::menu_npc(&mut w.guests[1].game.e), None);
    assert_eq!(talk::menu_npc(&mut w.guests[0].game.e), Some(SUNNY));
    // One result on the host, for the one conversation.
    assert_eq!(w.hd.social.enactor.enacted, 1);
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(10));
    w.assert_in_step();
}

#[test]
fn a_pick_the_host_cannot_run_is_left_alone_and_named() {
    let mut w = World::new(&["Sunny"]);
    w.run(20);
    // Sunny is not in the host's game now (her cell is not loaded there).
    w.host.set_present(SUNNY, false);
    assert!(w.guests[0].game.talk_to(SUNNY));
    w.guests[0].game.say(SHARED_LINE, false);
    w.run(10);
    assert_eq!(w.hd.social.enactor.enacted, 0);
    assert_eq!(w.hd.social.enactor.left, 1);
    assert!(w.hd.social.enactor.notes[0].contains("could not be run here"));
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(0));
    w.assert_in_step();
}

// ---- Party saves, rejoining, joining late ----

#[test]
fn a_party_save_holds_each_guests_part_and_a_load_gives_it_back() {
    let dir = party_dir("party-save");
    let mut w = World::new(&["Sunny", "Chet"]);
    w.hd.social.party.dir = Some(dir.clone());
    // Guest 1 has played its own opening and stands at a place of its own.
    w.guests[0].game.set_stage(OWN, 5);
    w.guests[0].game.set_quest_var(OWN, 1, 2.5);
    w.guests[0]
        .game
        .move_player(SALOON, [10.0, 20.0, 30.0], 1.0);
    w.run(20);
    let key = party::key_of("Sunny");
    let held =
        w.hd.social
            .party
            .held(key)
            .expect("the host holds the part");
    assert_eq!(
        held.place.map(|p| (p.space, p.pos)),
        Some((SALOON, [10.0, 20.0, 30.0]))
    );
    assert_eq!(held.quests.len(), 1);
    assert_eq!(held.quests[0].vars, [(1, 2.5f64.to_bits())]);
    // No save yet: no file. The host saves: a file per guest, named after the save.
    assert!(!dir.exists());
    assert_eq!(w.host.save("Save 3 - Goodsprings.fos"), None);
    w.run(2);
    let file = dir.join(format!("Save 3 - Goodsprings.{key:016x}.nvcoop"));
    assert_eq!(
        Part::decode(&std::fs::read(&file).unwrap()),
        Some(held.clone())
    );
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
    assert_eq!(
        (w.hd.social.party.saved, w.hd.social.party.files_written),
        (1, 2)
    );
    // The guest plays on: the host's copy follows, the saved file does not.
    w.guests[0].game.set_stage(OWN, 9);
    w.guests[0].game.move_player(STORE, [1.0, 2.0, 3.0], 0.0);
    w.run(120);
    assert_eq!(
        w.hd.social.party.held(key).unwrap().place.map(|p| p.space),
        Some(STORE)
    );
    assert_eq!(
        Part::decode(&std::fs::read(&file).unwrap()),
        Some(held.clone())
    );
    assert_eq!(w.guests[0].driver.social.party.restored, 0);
    // The host loads that save: each guest gets its part back, once.
    assert_eq!(w.host.load("Save 3 - Goodsprings.fos"), None);
    w.run(20);
    assert_eq!(w.hd.social.party.given_back, 2);
    for g in &w.guests {
        assert_eq!(g.driver.social.party.restored, 1);
    }
    let l = live::local(&mut w.guests[0].game.e).unwrap();
    assert_eq!(
        (l.cell, l.pos, l.heading),
        (SALOON, [10.0, 20.0, 30.0], 1.0)
    );
    assert_eq!(w.guests[0].game.warps(), 1);
    assert_eq!(w.guests[0].driver.social.party.last.quests, 1);
    w.run(200);
    for g in &w.guests {
        assert_eq!(g.driver.social.party.restored, 1, "given back once");
    }
    w.assert_in_step();
    // A save with no party files gives nothing back.
    assert_eq!(w.host.load("Save 1"), None);
    w.run(20);
    assert_eq!(w.hd.social.party.given_back, 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_player_who_joins_late_gets_the_snapshot_and_its_own_saved_part() {
    let dir = party_dir("late-join");
    // An earlier session: Sunny played and the host saved.
    {
        let mut w = World::new(&["Sunny"]);
        w.hd.social.party.dir = Some(dir.clone());
        w.guests[0].game.set_stage(OWN, 5);
        w.guests[0].game.move_player(STORE, [7.0, 8.0, 9.0], 0.5);
        w.run(20);
        w.host.save("autosave");
        w.run(2);
    }
    // A new session: the host loads the save and plays on, a conversation among it.
    let mut w = World::new(&[]);
    w.hd.social.party.dir = Some(dir.clone());
    w.host.load("autosave");
    assert!(w.host.talk_to(SUNNY));
    w.host.say(SHARED_LINE, false);
    w.run(20);
    // Sunny joins the running game from a game of its own that just started, and a stranger too.
    let sunny = w.join(guest_story(), "Sunny");
    let other = w.join(guest_story(), "Stranger");
    w.run(60);
    w.assert_in_step();
    assert_eq!(
        forms::quest_stage(&mut w.guests[sunny].game.e, MQ),
        Some(10)
    );
    assert_eq!(w.guests[sunny].driver.social.party.restored, 1);
    assert_eq!(
        forms::quest_stage(&mut w.guests[sunny].game.e, OWN),
        Some(5)
    );
    let l = live::local(&mut w.guests[sunny].game.e).unwrap();
    assert_eq!((l.cell, l.pos), (STORE, [7.0, 8.0, 9.0]));
    assert_eq!(w.guests[other].driver.social.party.restored, 0);
    assert_eq!(
        forms::quest_stage(&mut w.guests[other].game.e, OWN),
        Some(0)
    );
    // The story's scripts never ran on either.
    assert_eq!(w.guests[sunny].game.scripts_run(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_guest_that_drops_keeps_its_game_and_one_that_starts_anew_gets_its_part_back() {
    let mut w = World::new(&["Sunny"]);
    w.guests[0].game.set_stage(OWN, 5);
    w.guests[0].game.move_player(STORE, [7.0, 8.0, 9.0], 0.5);
    w.run(20);
    // The connection drops; the host plays on; the guest walks on in its own game.
    w.hd.sync.disconnect(PlayerId(1));
    w.net.flush(PlayerId(1));
    w.net.flush(PlayerId::HOST);
    assert!(w.host.set_stage(MQ, 10));
    for _ in 0..10 {
        w.t += 50;
        w.hd.step(&mut w.host.e, &mut w.hostnet, w.t);
    }
    assert_eq!(w.hd.social.party.left_behind, 1);
    w.net.flush(PlayerId(1));
    w.guests[0].game.move_player(SALOON, [1.0, 1.0, 1.0], 0.0);
    // It comes back with its game still running: the story catches up, its own state stays.
    {
        let g = &mut w.guests[0];
        g.driver.rejoin(PlayerId(1), &mut g.net);
    }
    w.run(60);
    w.assert_in_step();
    assert_eq!(w.guests[0].driver.social.party.restored, 0);
    assert_eq!(live::local(&mut w.guests[0].game.e).unwrap().cell, SALOON);
    // It drops again, and this time its game is started anew: the part the host kept comes back.
    w.run(120);
    w.hd.sync.disconnect(PlayerId(1));
    w.net.flush(PlayerId(1));
    w.net.flush(PlayerId::HOST);
    w.t += 50;
    w.hd.step(&mut w.host.e, &mut w.hostnet, w.t);
    w.net.flush(PlayerId(1));
    w.guests.clear();
    w.join(guest_story(), "Sunny");
    w.run(60);
    w.assert_in_step();
    assert_eq!(w.guests[0].driver.social.party.restored, 1);
    assert_eq!(forms::quest_stage(&mut w.guests[0].game.e, OWN), Some(5));
    let l = live::local(&mut w.guests[0].game.e).unwrap();
    assert_eq!((l.cell, l.pos), (SALOON, [1.0, 1.0, 1.0]));
}

#[test]
fn a_guest_still_at_its_menu_gets_its_part_once_it_is_in_the_world() {
    let dir = party_dir("menu-join");
    {
        let mut w = World::new(&["Sunny"]);
        w.hd.social.party.dir = Some(dir.clone());
        w.guests[0].game.set_stage(OWN, 5);
        w.run(20);
        w.host.save("quicksave");
        w.run(2);
    }
    let mut w = World::new(&[]);
    w.hd.social.party.dir = Some(dir.clone());
    w.host.load("quicksave");
    // A game at its main menu: a player in no cell.
    let mut game = Story::new(&StorySpec {
        quests: vec![(MQ, vec![5, 10, 20, 30]), (OWN, vec![5, 9])],
        globals: vec![(HOUR, 8.0)],
        vars: vec![(OWN, vec![1])],
        cells: vec![SALOON, STORE],
        ..StorySpec::default()
    });
    gate_guest(&mut game.e);
    w.join(game, "Sunny");
    w.run(40);
    assert_eq!(w.guests[0].driver.social.party.restored, 0);
    assert_eq!(w.guests[0].driver.social.party.uploads, 0);
    w.guests[0].game.move_player(STORE, [0.0; 3], 0.0);
    w.run(5);
    assert_eq!(w.guests[0].driver.social.party.restored, 1);
    assert_eq!(forms::quest_stage(&mut w.guests[0].game.e, OWN), Some(5));
    assert_eq!(live::local(&mut w.guests[0].game.e).unwrap().cell, SALOON);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---- Sleep and wait ----

fn hour(s: &mut Story) -> f32 {
    forms::global_value(&mut s.e, HOUR).unwrap()
}

#[test]
fn a_rest_everyone_agrees_to_passes_for_everyone() {
    let mut w = World::new(&["Sunny", "Chet"]);
    w.run(30);
    // Guest 1 asks to wait three hours: its own rest does not start; the others are told.
    w.guests[0].game.click_rest(3, false);
    w.run(5);
    assert_eq!(w.guests[0].game.rest_clicks(), (0, 1));
    assert_eq!(w.guests[0].driver.social.rester.proposed, 1);
    for s in [&mut w.host, &mut w.guests[1].game] {
        assert_eq!(
            s.messages().last().map(String::as_str),
            Some("Player 2 wants to wait 3 hour(s). Rest yourself to agree.")
        );
    }
    assert_eq!(hour(&mut w.host), 8.0);
    // The host agrees by resting itself (whatever hours its own menu shows); still one to go.
    w.host.click_rest(1, false);
    w.run(5);
    assert_eq!(w.hd.social.rester.agreed, 1);
    assert_eq!(w.host.hours_passed().0, 0);
    // The last player agrees: three hours pass in every game, through the game's hour step.
    w.guests[1].game.click_rest(8, true);
    w.run(40);
    assert_eq!(w.host.hours_passed().0, 3);
    assert_eq!(hour(&mut w.host), 11.0);
    assert_eq!(w.hd.social.rester.passed, 1);
    for g in &mut w.guests {
        assert_eq!(g.game.hours_passed().0, 3);
        assert_eq!(hour(&mut g.game), 11.0, "the host's clock");
        assert_eq!(g.driver.social.rester.passed, 1);
        assert_eq!(g.game.rest_clicks().0, 0, "no game rested by itself");
    }
    w.assert_in_step();
}

#[test]
fn a_rest_one_player_does_not_agree_to_passes_for_no_one() {
    let mut w = World::new(&["Sunny"]);
    w.run(30);
    w.guests[0].game.click_rest(5, true);
    w.run(5);
    assert_eq!(
        w.host.messages().last().map(String::as_str),
        Some("Player 2 wants to sleep 5 hour(s). Rest yourself to agree.")
    );
    // The host never answers: the vote runs out.
    w.run_every(40, 1000);
    assert_eq!(w.host.hours_passed().0, 0);
    assert_eq!(w.guests[0].game.hours_passed().0, 0);
    assert_eq!(hour(&mut w.host), 8.0);
    assert_eq!(w.guests[0].driver.social.rester.refused, 1);
    assert_eq!(
        w.guests[0].game.messages().last().map(String::as_str),
        Some("The party did not agree to rest.")
    );
    w.assert_in_step();
}

#[test]
fn a_host_alone_rests_as_the_game_does() {
    let mut w = World::new(&[]);
    w.run(5);
    w.host.click_rest(2, false);
    w.run(5);
    assert_eq!(w.host.rest_clicks(), (1, 0), "the game's own rest started");
    assert_eq!(w.hd.social.rester.proposed, 0);
    // And its conversations are its own too.
    assert!(w.host.talk_to(SUNNY));
    w.host.say(SHARED_LINE, false);
    w.run(5);
    assert_eq!(w.hd.social.talker.opened, 0);
    assert_eq!(forms::quest_stage(&mut w.host.e, MQ), Some(10));
}
