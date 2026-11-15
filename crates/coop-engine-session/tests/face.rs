//! A guest's face, hair, race, sex and name in a party save (docs/coop/SPECS.md S41, a part of
//! S32's character section), between games in one process joined over coop-core's loopback
//! transport: the guest's game writes the saved look onto its player through the race and sex
//! menu's calls and builds its head again. Each game is the story kit's with the look, anim and
//! stat kits' stand-ins.

use std::path::{Path, PathBuf};

use coop_core::compat::plugin_list_hash;
use coop_core::ids::PlayerId;
use coop_core::proto::{Hello, PROTOCOL_VERSION};
use coop_core::sync::HostConfig;
use coop_core::transport::{LoopbackEndpoint, LoopbackNet};
use coop_engine::look::{self, Face};
use coop_engine::statkit as kit;
use coop_engine::testkit::{anim_kit, look_kit, Story, StorySpec};
use coop_engine::{addr, stats};
use coop_engine_session::party::{self, Part};
use coop_engine_session::{gate_guest, GuestDriver, HostDriver};

engine_core::assemble!();

const MQ: u32 = 0x0010_4c0c;
const CELL: u32 = 0x0010_5a00;
const RACE: u32 = 0x0010_7100;
const OTHER_RACE: u32 = 0x0010_7106;
const HAIR: u32 = 0x0010_7101;
const OTHER_HAIR: u32 = 0x0010_7107;
const EYES: u32 = 0x0010_7102;
const NOSE: u32 = 0x0010_7103;
const PLAYER: u32 = 0x14;

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
    look_kit::install(&mut s);
    anim_kit::install(&mut s);
    kit::install(&mut s, &[]);
    for id in [RACE, OTHER_RACE, HAIR, OTHER_HAIR, EYES, NOSE] {
        s.add_look_form(id, 0x0c);
    }
    s.move_player(CELL, [0.0; 3], 0.0);
    s
}

/// Sunny's look, as character creation made it.
fn sunny() -> Face {
    Face {
        female: true,
        race: OTHER_RACE,
        hair: HAIR,
        hair_length: 0.25,
        eyes: EYES,
        hair_color: 0x0040_2010,
        height: 1.02,
        weight: 0.5,
        head_parts: vec![NOSE],
        coord: [
            (3, 1, vec![0.5, -1.25, 2.0]),
            (2, 1, vec![0.125, -0.75]),
            (0, 0, Vec::new()),
            (0, 0, Vec::new()),
        ],
        name: "Sunny".into(),
    }
}

/// The look a game started anew gives its player.
fn new_character() -> Face {
    Face {
        female: false,
        race: RACE,
        hair: OTHER_HAIR,
        name: "Courier".into(),
        ..sunny()
    }
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

    fn join(&mut self) {
        let mut game = story();
        assert!(look::set_face(&mut game.e, 7, &new_character()));
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

#[test]
fn a_guest_started_anew_gets_its_face_race_sex_and_name_back() {
    let dir = party_dir("party-face-anew");
    let mut w = World::new(&dir);
    w.run(20);
    assert!(look::set_face(&mut w.game().e, 7, &sunny()));
    w.run(120);
    let key = party::key_of("Sunny");
    let held =
        w.hd.social
            .party
            .held(key)
            .expect("the host holds the part");
    assert_eq!(held.sheet.and_then(|s| s.look), Some(sunny()));
    // Sunny's game ends; a new game joins as Sunny with a new character's look.
    w.hd.sync.disconnect(PlayerId(1));
    w.net.flush(PlayerId(1));
    w.net.flush(PlayerId::HOST);
    w.t += 50;
    w.hd.step(&mut w.host.e, &mut w.hostnet, w.t);
    w.net.flush(PlayerId(1));
    w.guests.clear();
    w.join();
    assert_eq!(look::player_face(&mut w.game().e), Some(new_character()));
    w.run(60);
    assert_eq!(w.guests[0].driver.social.party.restored, 1);
    assert_eq!(w.guests[0].driver.social.party.last.character.look, 1);
    // Its own player, in the world: Sunny's look, through the menu's calls, its head rebuilt.
    assert_eq!(look::player_face(&mut w.game().e), Some(sunny()));
    let calls = w.game().menu_calls();
    for c in [
        addr::NPC_SET_RACE,
        addr::ACTOR_BASE_SET_FLAG_BIT,
        addr::NPC_SET_HAIR,
        addr::UPDATE_PLAYER_NAME,
        addr::PLAYER_REFRESH_3D,
    ] {
        assert!(calls.contains(&c), "{c:08x} called");
    }
    let shape = w.game().shape(PLAYER);
    assert_eq!((shape.heads_built, shape.faces_applied), (1, 2));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_load_gives_the_saved_look_back_and_a_part_without_it_leaves_the_look_alone() {
    let dir = party_dir("party-face-load");
    let mut w = World::new(&dir);
    w.run(20);
    assert!(look::set_face(&mut w.game().e, 7, &sunny()));
    w.run(120);
    assert_eq!(w.host.save("Save 4.fos"), None);
    w.run(2);
    let key = party::key_of("Sunny");
    let file = dir.join(format!("Save 4.{key:016x}.nvcoop"));
    let on_disk = Part::decode(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(
        on_disk.sheet.as_ref().and_then(|s| s.look.clone()),
        Some(sunny())
    );
    // Sunny changes its hair at a barber, then the host loads the save.
    let mut barber = sunny();
    barber.hair = OTHER_HAIR;
    assert!(look::set_face(&mut w.game().e, 7, &barber));
    w.run(120);
    assert_eq!(w.host.load("Save 4.fos"), None);
    w.run(20);
    assert_eq!(look::player_face(&mut w.game().e), Some(sunny()));
    // The same sex and race: neither is asked of the game.
    let calls = w.game().menu_calls();
    assert!(!calls.contains(&addr::NPC_SET_RACE));
    assert!(!calls.contains(&addr::ACTOR_BASE_SET_FLAG_BIT));
    // A part without the look (a build before it) leaves the look alone.
    let mut old = w.hd.social.party.held(key).unwrap();
    if let Some(s) = old.sheet.as_mut() {
        s.look = None;
    }
    let old = Part::decode(&old.encode()).unwrap();
    assert!(old.sheet.as_ref().is_some_and(|s| s.look.is_none()));
    assert!(look::set_face(&mut w.game().e, 7, &barber));
    let applied = old.apply(&mut w.game().e);
    assert_eq!(applied.character.look, 0);
    assert_eq!(look::player_face(&mut w.game().e), Some(barber.clone()));
    // The sheet read now holds the look the game has.
    assert_eq!(
        stats::read(&mut w.game().e).and_then(|s| s.look),
        Some(barber)
    );
    let _ = std::fs::remove_dir_all(&dir);
}
