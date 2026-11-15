//! A guest's character in a party save (docs/coop/SPECS.md S32), between games in one process
//! joined over coop-core's loopback transport. Each game is the story kit's with the item kit's
//! and the stat kit's stand-ins for the game's inventory and character functions.

use std::path::{Path, PathBuf};

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::itemkit::{self, ItemKitSpec, ItemSpec};
use coop_engine::statkit as kit;
use coop_engine::testkit::{Story, StorySpec};
use coop_engine::{addr, items, live, stats};
use coop_engine_session::party::{self, Part};
use coop_engine_session::{gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const CELL: u32 = 0x0010_5a00;
const TOUGHNESS: u32 = 0x0003_1dd8;
const EDUCATED: u32 = 0x0003_1dd9;
const ARMOR: u32 = 0x0002_0420;
const RIFLE: u32 = 0x0000_4333;
const STRENGTH: u32 = addr::ACTOR_VALUE_SPECIAL_FIRST;
const SPEECH: u32 = addr::ACTOR_VALUE_SKILL_FIRST + 0xb;
const RIGHT_ARM: u32 = addr::ACTOR_VALUE_CONDITION_FIRST + 3;

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

fn story() -> Story {
    let mut s = Story::new(&StorySpec {
        quests: vec![(MQ, vec![10, 20])],
        cells: vec![CELL],
        ..StorySpec::default()
    });
    itemkit::install(
        &mut s,
        &ItemKitSpec {
            items: vec![
                ItemSpec {
                    id: ARMOR,
                    health: 100,
                },
                ItemSpec {
                    id: RIFLE,
                    health: 200,
                },
            ],
            ..ItemKitSpec::default()
        },
    );
    kit::install(&mut s, &[TOUGHNESS, EDUCATED]);
    s.move_player(CELL, [0.0; 3], 0.0);
    s
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
    fn new(dir: &Path) -> World {
        let net = LoopbackNet::new();
        let hostnet = net.endpoint(PlayerId::HOST);
        let mut host = story();
        let mut hd = HostDriver::new(HostConfig::new(hello(), 1), [MQ], []);
        hd.start(&mut host.e);
        hd.social.party.dir = Some(dir.to_path_buf());
        let mut w = World {
            net,
            host,
            hd,
            hostnet,
            guests: Vec::new(),
            t: 0,
        };
        w.join();
        w
    }

    /// A game started anew joins as player 1, known as Sunny.
    fn join(&mut self) {
        let mut game = story();
        gate_guest(&mut game.e);
        let mut driver = GuestDriver::new(PlayerId(1), hello());
        driver.social.party.key = party::key_of("Sunny");
        self.guests.push(Guest {
            game,
            driver,
            net: self.net.endpoint(PlayerId(1)),
        });
    }

    fn run(&mut self, frames: u32) {
        for _ in 0..frames {
            self.t += 50;
            self.hd.step(&mut self.host.e, &mut self.hostnet, self.t);
            for g in &mut self.guests {
                g.driver.step(&mut g.game.e, &mut g.net, self.t);
            }
        }
    }

    fn game(&mut self) -> &mut Story {
        &mut self.guests[0].game
    }
}

fn party_dir(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Sunny's character as it is when the host saves.
fn play_a_character(s: &mut Story) {
    let player = live::player(&mut s.e).unwrap();
    kit::set_base(s, STRENGTH, 8.0);
    kit::set_base(s, SPEECH, 62.0);
    kit::set_level(s, 5, 4300);
    kit::set_tags(s, [SPEECH as i32, -1, -1, -1]);
    stats::set_perk(&mut s.e, player, TOUGHNESS, 2);
    kit::set_current(s, addr::ACTOR_VALUE_HEALTH, 74.0);
    kit::set_current(s, addr::ACTOR_VALUE_RADIATION, 85.0);
    kit::set_current(s, RIGHT_ARM, 0.0);
    kit::set_karma(s, 310);
    items::add(&mut s.e, player, ARMOR, 1, None);
    items::add(&mut s.e, player, RIFLE, 1, Some(0.5));
    stats::equip(&mut s.e, player, ARMOR);
    stats::equip(&mut s.e, player, RIFLE);
}

#[test]
fn a_party_save_holds_a_guests_character_and_a_load_gives_it_back() {
    let dir = party_dir("party-character");
    let mut w = World::new(&dir);
    w.run(20);
    play_a_character(w.game());
    let saved = stats::read(&mut w.game().e).unwrap();
    w.run(120);
    let key = party::key_of("Sunny");
    let held =
        w.hd.social
            .party
            .held(key)
            .expect("the host holds the part");
    assert_eq!(held.sheet.as_ref(), Some(&saved));
    assert_eq!(w.host.save("Save 3.fos"), None);
    w.run(2);
    let file = dir.join(format!("Save 3.{key:016x}.nvcoop"));
    let on_disk = Part::decode(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(on_disk.sheet, Some(saved.clone()));

    // Sunny plays on: levels up, takes a perk, heals, takes the armor off, loses karma.
    {
        let s = w.game();
        let player = live::player(&mut s.e).unwrap();
        kit::set_level(s, 6, 5200);
        kit::set_base(s, SPEECH, 70.0);
        stats::set_perk(&mut s.e, player, EDUCATED, 1);
        kit::set_current(s, addr::ACTOR_VALUE_HEALTH, 100.0);
        kit::set_current(s, RIGHT_ARM, 100.0);
        kit::set_karma(s, 0);
        stats::unequip(&mut s.e, player, ARMOR);
    }
    w.run(120);
    assert_ne!(stats::read(&mut w.game().e), Some(saved.clone()));
    // The host loads the save: Sunny's character is the saved one.
    assert_eq!(w.host.load("Save 3.fos"), None);
    w.run(20);
    let party = &w.guests[0].driver.social.party;
    assert_eq!(party.restored, 1);
    let changed = party.last.character;
    assert!(changed.character > 0 && changed.condition > 0 && changed.worn > 0);
    assert_eq!(stats::read(&mut w.game().e), Some(saved));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_guest_started_anew_gets_its_character_back() {
    let dir = party_dir("party-character-anew");
    let mut w = World::new(&dir);
    w.run(20);
    play_a_character(w.game());
    let saved = stats::read(&mut w.game().e).unwrap();
    w.run(120);
    // Sunny's game ends; the host keeps its last part. A new game joins as Sunny.
    w.hd.sync.disconnect(PlayerId(1));
    w.net.flush(PlayerId(1));
    w.net.flush(PlayerId::HOST);
    w.t += 50;
    w.hd.step(&mut w.host.e, &mut w.hostnet, w.t);
    assert_eq!(w.hd.social.party.left_behind, 1);
    w.net.flush(PlayerId(1));
    w.guests.clear();
    w.join();
    // A new character carries nothing; the part brings the pack back first, then what is worn.
    let player = live::player(&mut w.game().e).unwrap();
    assert_eq!(stats::worn(&mut w.game().e, player), Vec::<u32>::new());
    w.run(60);
    assert_eq!(w.guests[0].driver.social.party.restored, 1);
    assert_eq!(stats::read(&mut w.game().e), Some(saved));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_part_without_the_character_leaves_the_character_alone() {
    let dir = party_dir("party-character-old");
    // A part as a build before this section wrote it: place and pack only.
    let mut w = World::new(&dir);
    w.run(20);
    play_a_character(w.game());
    w.run(120);
    let key = party::key_of("Sunny");
    let mut old = w.hd.social.party.held(key).unwrap();
    old.sheet = None;
    let bytes = old.encode();
    assert_eq!(Part::decode(&bytes).unwrap().sheet, None);
    // The guest's game takes such a part: nothing of the character changes.
    let before = stats::read(&mut w.game().e);
    let applied = old.apply(&mut w.game().e);
    assert_eq!(applied.character, stats::Changed::default());
    assert_eq!(stats::read(&mut w.game().e), before);
    let _ = std::fs::remove_dir_all(&dir);
}
