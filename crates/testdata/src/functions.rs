//! A world for the script functions in `world::script_functions`: people of
//! an adult race, a child race and a race whose body models are a child's,
//! a creature, a class, a faction that tracks crime and is the player's
//! friend, a rifle, items (one a quest item), form lists, doors (plain,
//! open by default, locked), a quest with three objectives, a note, a
//! challenge, a message, an idle, a topic, three game settings, and the
//! interior `TestHouse` (owned by the faction) with `TestHouseCellar` and
//! `Elsewhere` beside it.

use crate::{f32s, group, placed, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const VALUE: u32 = 0xF00;
    pub const FLOAT_SETTING: u32 = 0xF01;
    pub const INT_SETTING: u32 = 0xF02;
    pub const TEXT_SETTING: u32 = 0xF03;
    pub const ADULT_RACE: u32 = 0xF04;
    /// `DATA` flag 0x04.
    pub const CHILD_RACE: u32 = 0xF05;
    /// No flag, but its male upper body is `Characters\Child\UpperBody.nif`.
    pub const SMALL_RACE: u32 = 0xF06;
    pub const CLASS: u32 = 0xF07;
    pub const PLAYER: u32 = 0x7;
    /// In `TestTown` (rank 0) and `OtherFaction` (rank -1), of `TestClass`.
    pub const ADULT: u32 = 0xF08;
    pub const KID: u32 = 0xF09;
    pub const SMALL: u32 = 0xF0A;
    /// Type 2.
    pub const DOG: u32 = 0xF0B;
    /// Tracks crime (`DATA` 0x100) and is the player's friend.
    pub const TOWN: u32 = 0xF0C;
    pub const OTHER_FACTION: u32 = 0xF0D;
    /// Animation type 5 (two-handed rifle), Guns (41).
    pub const RIFLE: u32 = 0xF0E;
    pub const CUP: u32 = 0xF0F;
    pub const SHIRT: u32 = 0xF10;
    pub const DRINK: u32 = 0xF11;
    /// A key with the record's quest-item flag (0x400).
    pub const QUEST_KEY: u32 = 0xF12;
    /// `TestCup` and `TestAdult`.
    pub const LIST: u32 = 0xF13;
    /// `TestShirt`.
    pub const HOLDOUT: u32 = 0xF14;
    /// `TestRifle`.
    pub const WEAPONS: u32 = 0xF15;
    pub const DOOR: u32 = 0xF16;
    pub const CHEST: u32 = 0xF17;
    pub const FLASH: u32 = 0xF18;
    /// The menu backgrounds' modifiers, at their `FalloutNV.esm` form IDs
    /// (`world::menu_background::form`).
    pub const MENU_POPUP_FX: u32 = 0x0003_2B38;
    pub const MENU_PAUSE_FX: u32 = 0x0004_EEE8;
    /// Starts with the game; objectives 10, 20, 30; stage 10.
    pub const QUEST: u32 = 0xF19;
    /// VCG01 activation regression fixture: stage 60/objective 30, then 65.
    pub const VIGOR_QUEST: u32 = 0xF28;
    pub const VIGOR_TESTER_SCRIPT: u32 = 0xF29;
    pub const VIGOR_TESTER: u32 = 0xF2A;
    pub const VIGOR_TESTER_REF: u32 = 0xF2B;
    pub const VIGOR_TRIGGER_SCRIPT: u32 = 0xF2C;
    pub const VIGOR_TRIGGER: u32 = 0xF2D;
    pub const VIGOR_TRIGGER_REF: u32 = 0xF2E;
    /// Doesn't start with the game; stage 5.
    pub const OTHER_QUEST: u32 = 0xF1A;
    pub const NOTE: u32 = 0xF1B;
    pub const CHALLENGE: u32 = 0xF1C;
    /// `DESC` "Mr. Renamed", `FULL` "A Title".
    pub const NAME_MESSAGE: u32 = 0xF1D;
    pub const WAVE: u32 = 0xF1E;
    pub const TOPIC: u32 = 0xF1F;
    /// A region `TestHouse` lists (`XCLR`).
    pub const REGION: u32 = 0xF23;
    /// A trigger (`TestTrigger`, placed as `TriggerRef`) whose script
    /// counts the player's `OnTrigger` in `iInside` and sets `iEntered`
    /// when the adult enters.
    pub const TRIGGER_SCRIPT: u32 = 0xF24;
    pub const TRIGGER: u32 = 0xF25;
    pub const TRIGGER_REF: u32 = 0xF3B;
    /// A travel package (`PKDT` type 6) and a sandbox one (12).
    pub const TRAVEL: u32 = 0xF26;
    pub const SANDBOX: u32 = 0xF27;
    pub const HOUSE: u32 = 0xF20;
    pub const CELLAR: u32 = 0xF21;
    pub const ELSEWHERE: u32 = 0xF22;
    /// At 100,0,0 facing north.
    pub const ADULT_REF: u32 = 0xF30;
    pub const KID_REF: u32 = 0xF31;
    pub const SMALL_REF: u32 = 0xF32;
    pub const DOG_REF: u32 = 0xF33;
    pub const DOOR_REF: u32 = 0xF34;
    /// Open by default (`ONAM`).
    pub const OPEN_DOOR_REF: u32 = 0xF35;
    /// Locked at level 50.
    pub const LOCKED_DOOR_REF: u32 = 0xF36;
    /// Owned by `TestAdult`.
    pub const CHEST_REF: u32 = 0xF37;
    /// At 100,100,0, turned 10, 20, 30 degrees, scale 2.
    pub const CUP_REF: u32 = 0xF38;
    /// In the cellar.
    pub const CELLAR_CUP_REF: u32 = 0xF39;
    /// The rifle lying at 200,0,0.
    pub const RIFLE_REF: u32 = 0xF3A;
}

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn functions(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };
    let mut plugin = record(
        b"TES4",
        0,
        &sub(b"HEDR", &{
            let mut h = 1.34f32.to_le_bytes().to_vec();
            h.extend([0; 8]);
            h
        }),
    );

    // Game settings.
    let mut settings = named(
        b"GMST",
        FLOAT_SETTING,
        "fTestSetting",
        &sub(b"DATA", &2.5f32.to_le_bytes()),
    );
    settings.extend(named(
        b"GMST",
        INT_SETTING,
        "iTestCount",
        &sub(b"DATA", &7i32.to_le_bytes()),
    ));
    settings.extend(named(
        b"GMST",
        TEXT_SETTING,
        "sTestText",
        &sub(b"DATA", &zstr("hi")),
    ));
    plugin.extend(group(*b"GMST", 0, &settings));

    let mut glob = sub(b"FNAM", b"f");
    glob.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    plugin.extend(group(
        *b"GLOB",
        0,
        &named(b"GLOB", VALUE, "TestValue", &glob),
    ));

    // Races: DATA (skill boosts 14, 2 unused, heights and weights, flags
    // at 32), then the body parts after NAM1.
    let race = |id: u32, name: &str, flags: u32, body: Option<&str>| {
        let mut d = vec![0u8; 16];
        d.extend(f32s(&[1.0, 1.0, 1.0, 1.0]));
        d.extend(flags.to_le_bytes());
        let mut r = sub(b"DATA", &d);
        r.extend(sub(b"NAM1", &[]));
        r.extend(sub(b"MNAM", &[]));
        r.extend(sub(b"INDX", &0u32.to_le_bytes()));
        r.extend(sub(
            b"MODL",
            &zstr(body.unwrap_or("Characters\\_Male\\UpperBody.nif")),
        ));
        named(b"RACE", id, name, &r)
    };
    let mut races = race(ADULT_RACE, "TestAdultRace", 0, None);
    races.extend(race(CHILD_RACE, "TestChildRace", 0x04, None));
    races.extend(race(
        SMALL_RACE,
        "TestSmallRace",
        0,
        Some("Characters\\Child\\UpperBody.nif"),
    ));
    plugin.extend(group(*b"RACE", 0, &races));
    plugin.extend(group(*b"CLAS", 0, &named(b"CLAS", CLASS, "TestClass", &[])));

    // Factions: the town tracks crime and is the player's friend.
    let mut town = sub(b"FULL", &zstr("Test Town"));
    let mut xnam = 0x1B2A4u32.to_le_bytes().to_vec();
    xnam.extend(0i32.to_le_bytes());
    xnam.extend(3u32.to_le_bytes());
    town.extend(sub(b"XNAM", &xnam));
    town.extend(sub(b"DATA", &0x100u32.to_le_bytes()));
    let mut factions = named(b"FACT", TOWN, "TestTown", &town);
    factions.extend(named(
        b"FACT",
        OTHER_FACTION,
        "OtherFaction",
        &sub(b"DATA", &[0; 4]),
    ));
    plugin.extend(group(*b"FACT", 0, &factions));

    // People.
    let person = |id: u32, name: &str, race: u32, extra: &[u8]| {
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"ACBS", &[0; 24]));
        d.extend(sub(b"RNAM", &race.to_le_bytes()));
        let mut stats = 100i32.to_le_bytes().to_vec();
        stats.extend([5; 7]);
        d.extend(sub(b"DATA", &stats));
        d.extend(extra);
        named(b"NPC_", id, &name.replace(' ', ""), &d)
    };
    let membership = |faction: u32, rank: i8| {
        let mut m = faction.to_le_bytes().to_vec();
        m.extend([rank as u8, 0, 0, 0]);
        sub(b"SNAM", &m)
    };
    let mut adult_extra = sub(b"CNAM", &CLASS.to_le_bytes());
    adult_extra.extend(membership(TOWN, 0));
    adult_extra.extend(membership(OTHER_FACTION, -1));
    let mut npcs = person(PLAYER, "Player", ADULT_RACE, &[]);
    npcs.extend(person(ADULT, "Test Adult", ADULT_RACE, &adult_extra));
    npcs.extend(person(KID, "Test Kid", CHILD_RACE, &[]));
    npcs.extend(person(SMALL, "Test Small", SMALL_RACE, &[]));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    let mut dog = sub(b"ACBS", &[0; 24]);
    let mut dog_data = vec![2u8, 0, 0, 0];
    dog_data.extend(20i16.to_le_bytes());
    dog_data.extend([0, 0]);
    dog_data.extend(4i16.to_le_bytes());
    dog_data.extend([5; 7]);
    dog.extend(sub(b"DATA", &dog_data));
    plugin.extend(group(*b"CREA", 0, &named(b"CREA", DOG, "TestDog", &dog)));

    // Items.
    let mut rifle = sub(b"FULL", &zstr("Rifle"));
    let mut rifle_data = 100i32.to_le_bytes().to_vec();
    rifle_data.extend(100i32.to_le_bytes());
    rifle_data.extend(5.0f32.to_le_bytes());
    rifle_data.extend(20i16.to_le_bytes());
    rifle_data.push(5);
    rifle.extend(sub(b"DATA", &rifle_data));
    let mut dnam = vec![0u8; 204];
    // No resist type (-1 at 120), as the game's guns.
    dnam[120..124].copy_from_slice(&(-1i32).to_le_bytes());
    dnam[0] = 5;
    dnam[104..108].copy_from_slice(&41u32.to_le_bytes());
    rifle.extend(sub(b"DNAM", &dnam));
    plugin.extend(group(
        *b"WEAP",
        0,
        &named(b"WEAP", RIFLE, "TestRifle", &rifle),
    ));
    let misc = |id: u32, name: &str| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut v = 1i32.to_le_bytes().to_vec();
        v.extend(1.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &v));
        named(b"MISC", id, &format!("Test{name}"), &d)
    };
    plugin.extend(group(*b"MISC", 0, &misc(CUP, "Cup")));
    let mut key = sub(b"FULL", &zstr("Quest Key"));
    key.extend(sub(b"DATA", &[0; 8]));
    let mut key = named(b"KEYM", QUEST_KEY, "TestQuestKey", &key);
    key[8..12].copy_from_slice(&0x400u32.to_le_bytes());
    plugin.extend(group(*b"KEYM", 0, &key));
    let mut shirt = sub(b"FULL", &zstr("Shirt"));
    shirt.extend(sub(b"BMDT", &[4, 0, 0, 0, 0, 0, 0, 0]));
    plugin.extend(group(
        *b"ARMO",
        0,
        &named(b"ARMO", SHIRT, "TestShirt", &shirt),
    ));
    let mut drink = sub(b"FULL", &zstr("Drink"));
    drink.extend(sub(b"DATA", &0.5f32.to_le_bytes()));
    drink.extend(sub(b"ENIT", &[0; 20]));
    plugin.extend(group(
        *b"ALCH",
        0,
        &named(b"ALCH", DRINK, "TestDrink", &drink),
    ));

    // Form lists.
    let list = |id: u32, name: &str, forms: &[u32]| {
        let mut d = Vec::new();
        for f in forms {
            d.extend(sub(b"LNAM", &f.to_le_bytes()));
        }
        named(b"FLST", id, name, &d)
    };
    let mut lists = list(LIST, "TestList", &[CUP, ADULT]);
    lists.extend(list(HOLDOUT, "TestHoldout", &[SHIRT]));
    lists.extend(list(WEAPONS, "TestWeapons", &[RIFLE]));
    plugin.extend(group(*b"FLST", 0, &lists));

    plugin.extend(group(
        *b"DOOR",
        0,
        &named(
            b"DOOR",
            DOOR,
            "TestDoor",
            &sub(b"MODL", &zstr("test\\door.nif")),
        ),
    ));
    plugin.extend(group(
        *b"CONT",
        0,
        &named(b"CONT", CHEST, "TestChest", &sub(b"FULL", &zstr("Chest"))),
    ));
    let mut flash = 0u32.to_le_bytes().to_vec();
    flash.extend(1.0f32.to_le_bytes());

    // The menu backgrounds' modifiers, laid out as `FalloutNV.esm`'s (flags
    // 0, a 1 s duration, keys at 0 and 1): `PopupBackgroundFX` a blur of 3;
    // `PauseBackgroundFX` a blur of 3, saturation × 0 and a tint.
    let keys = |pairs: &[(f32, f32)]| -> Vec<u8> {
        pairs
            .iter()
            .flat_map(|(t, v)| t.to_le_bytes().into_iter().chain(v.to_le_bytes()))
            .collect()
    };
    let background = |id: u32, name: &str, saturation: f32, tint: [f32; 4]| {
        let mut dnam = 0u32.to_le_bytes().to_vec();
        dnam.extend(1.0f32.to_le_bytes());
        let mut d = sub(b"DNAM", &dnam);
        d.extend(sub(b"BNAM", &keys(&[(0.0, 3.0), (1.0, 0.0)])));
        let mut t = Vec::new();
        for (time, c) in [(0.0f32, tint), (1.0, [1.0, 1.0, 1.0, 0.0])] {
            t.extend(time.to_le_bytes());
            t.extend(c.iter().flat_map(|v| v.to_le_bytes()));
        }
        d.extend(sub(b"TNAM", &t));
        // Track 17 (saturation): multiply, then add.
        d.extend(sub(
            &[17, b'I', b'A', b'D'],
            &keys(&[(0.0, saturation), (1.0, 1.0)]),
        ));
        d.extend(sub(
            &[17 | 0x40, b'I', b'A', b'D'],
            &keys(&[(0.0, 0.0), (1.0, 0.0)]),
        ));
        named(b"IMAD", id, name, &d)
    };
    let mut imads = named(b"IMAD", FLASH, "TestFlash", &sub(b"DNAM", &flash));
    imads.extend(background(
        MENU_POPUP_FX,
        "PopupBackgroundFX",
        1.0,
        [1.0, 1.0, 1.0, 0.0],
    ));
    imads.extend(background(
        MENU_PAUSE_FX,
        "PauseBackgroundFX",
        0.0,
        [0.671, 0.659, 0.239, 0.588],
    ));
    plugin.extend(group(*b"IMAD", 0, &imads));

    // Quests.
    let quest = |id: u32, name: &str, flags: u8, stage: i16, objectives: &[i32]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut qdata = vec![flags, 50, 0, 0];
        qdata.extend(1.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &qdata));
        d.extend(sub(b"INDX", &stage.to_le_bytes()));
        d.extend(sub(b"QSDT", &[0]));
        d.extend(sub(b"CNAM", &zstr("Something happened.")));
        if id == VIGOR_QUEST {
            d.extend(sub(b"SCHR", &[0; 20]));
            // The installed stage-60 source completes objective 10 after
            // Doc's intro. Objective 30 is displayed by INFO 001074A2.
            d.extend(sub(b"SCTX", b"SetObjectiveCompleted VCG01 10 1"));
            d.extend(sub(b"INDX", &65i16.to_le_bytes()));
            d.extend(sub(b"QSDT", &[0]));
            d.extend(sub(b"CNAM", &zstr("Vigor tester complete.")));
        }
        for o in objectives {
            d.extend(sub(b"QOBJ", &o.to_le_bytes()));
            d.extend(sub(b"NNAM", &zstr(&format!("Objective {o}"))));
        }
        named(b"QUST", id, &name.replace(' ', ""), &d)
    };
    let mut quests = quest(QUEST, "Test Quest", 0x01, 10, &[10, 20, 30]);
    let vigor_quest = quest(VIGOR_QUEST, "VCG01", 0x01, 60, &[10, 30]);
    quests.extend(vigor_quest);
    quests.extend(quest(OTHER_QUEST, "Other Quest", 0, 5, &[]));
    plugin.extend(group(*b"QUST", 0, &quests));

    let mut note = sub(b"FULL", &zstr("Note"));
    note.extend(sub(b"DATA", &[1]));
    plugin.extend(group(*b"NOTE", 0, &named(b"NOTE", NOTE, "TestNote", &note)));
    plugin.extend(group(
        *b"CHAL",
        0,
        &named(b"CHAL", CHALLENGE, "TestChallenge", &[]),
    ));
    let mut message = sub(b"DESC", &zstr("Mr. Renamed"));
    message.extend(sub(b"FULL", &zstr("A Title")));
    plugin.extend(group(
        *b"MESG",
        0,
        &named(b"MESG", NAME_MESSAGE, "TestNameMessage", &message),
    ));
    plugin.extend(group(*b"IDLE", 0, &named(b"IDLE", WAVE, "TestWave", &[])));
    plugin.extend(group(*b"DIAL", 0, &named(b"DIAL", TOPIC, "TestTopic", &[])));

    // The cells and what's in them.
    let thing = |kind: &[u8; 4],
                 id: u32,
                 base: u32,
                 pos: [f32; 3],
                 degrees: [f32; 3],
                 name: &str,
                 extra: &[u8]| {
        let r = placed(id, base, pos, degrees, extra);
        let mut d = edid(name);
        d.extend(&r[24..]);
        record(kind, id, &d)
    };
    let mut refs = thing(
        b"ACHR",
        ADULT_REF,
        ADULT,
        [100.0, 0.0, 0.0],
        [0.0; 3],
        "AdultRef",
        &[],
    );
    refs.extend(thing(
        b"ACHR",
        KID_REF,
        KID,
        [0.0, -100.0, 0.0],
        [0.0; 3],
        "KidRef",
        &[],
    ));
    refs.extend(thing(
        b"ACHR",
        SMALL_REF,
        SMALL,
        [0.0, -200.0, 0.0],
        [0.0; 3],
        "SmallRef",
        &[],
    ));
    refs.extend(thing(
        b"ACRE",
        DOG_REF,
        DOG,
        [-100.0, 0.0, 0.0],
        [0.0; 3],
        "DogRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        DOOR_REF,
        DOOR,
        [300.0, 0.0, 0.0],
        [0.0; 3],
        "DoorRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        OPEN_DOOR_REF,
        DOOR,
        [300.0, 100.0, 0.0],
        [0.0; 3],
        "OpenDoorRef",
        &sub(b"ONAM", &[]),
    ));
    let mut xloc = vec![50u8, 0, 0, 0];
    xloc.extend([0; 16]);
    refs.extend(thing(
        b"REFR",
        LOCKED_DOOR_REF,
        DOOR,
        [300.0, 200.0, 0.0],
        [0.0; 3],
        "LockedDoorRef",
        &sub(b"XLOC", &xloc),
    ));
    refs.extend(thing(
        b"REFR",
        CHEST_REF,
        CHEST,
        [0.0, 100.0, 0.0],
        [0.0; 3],
        "ChestRef",
        &sub(b"XOWN", &ADULT.to_le_bytes()),
    ));
    refs.extend(thing(
        b"REFR",
        CUP_REF,
        CUP,
        [100.0, 100.0, 0.0],
        [10.0, 20.0, 30.0],
        "CupRef",
        &sub(b"XSCL", &2.0f32.to_le_bytes()),
    ));
    refs.extend(thing(
        b"REFR",
        RIFLE_REF,
        RIFLE,
        [200.0, 0.0, 0.0],
        [0.0; 3],
        "RifleRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        TRIGGER_REF,
        TRIGGER,
        [0.0, 0.0, 0.0],
        [0.0; 3],
        "TriggerRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        VIGOR_TESTER_REF,
        VIGOR_TESTER,
        [500.0, 0.0, 0.0],
        [0.0; 3],
        "VCG01VigorTesterREF",
        &[],
    ));
    let mut trigger_data = Vec::new();
    for half in [216.0f32, 93.0, 100.0] {
        trigger_data.extend(half.to_le_bytes());
    }
    trigger_data.extend([0u8; 16]);
    trigger_data.extend(1u32.to_le_bytes());
    refs.extend(thing(
        b"REFR",
        VIGOR_TRIGGER_REF,
        VIGOR_TRIGGER,
        [1888.0, 1835.0, 7460.0],
        [0.0; 3],
        "VCG01VigorTesterTriggerREF",
        &sub(b"XPRM", &trigger_data),
    ));
    let mut script = sub(b"SCHR", &[0; 20]);
    script.extend(sub(
        b"SCTX",
        b"scn TestTriggerScript\nshort iEntered\nshort iInside\n\
          Begin OnTrigger player\n\tset iInside to iInside + 1\nEnd\n\
          Begin OnTriggerEnter\n\tif IsActionRef AdultRef\n\t\tset iEntered to 1\n\tendif\nEnd",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", TRIGGER_SCRIPT, "TestTriggerScript", &script),
    ));
    plugin.extend(group(
        *b"ACTI",
        0,
        &named(
            b"ACTI",
            TRIGGER,
            "TestTrigger",
            &sub(b"SCRI", &TRIGGER_SCRIPT.to_le_bytes()),
        ),
    ));
    let mut vigor_trigger_script = sub(b"SCHR", &[0; 20]);
    vigor_trigger_script.extend(sub(
        b"SCTX",
        b"scn VCG01VigorTesterTriggerSCRIPT\n\
          Begin OnTriggerEnter Player\n\
          \tif GetStageDone VCG01 60 == 0\n\
          \t\tif IsActionRef Player == 1\n\
          \t\t\tSetStage VCG01 60\n\
          \t\tendif\n\
          \tendif\n\
          End",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(
            b"SCPT",
            VIGOR_TRIGGER_SCRIPT,
            "VCG01VigorTesterTriggerSCRIPT",
            &vigor_trigger_script,
        ),
    ));
    plugin.extend(group(
        *b"ACTI",
        0,
        &named(
            b"ACTI",
            VIGOR_TRIGGER,
            "VCG01VigorTesterTrigger",
            &sub(b"SCRI", &VIGOR_TRIGGER_SCRIPT.to_le_bytes()),
        ),
    ));
    let mut vigor_script = sub(b"SCHR", &[0; 20]);
    vigor_script.extend(sub(
        b"SCTX",
        b"scn VCG01VigorTesterSCRIPT\n\
          Begin OnActivate\n\
          \tif GetStage VCG01 == 60 && GetObjectiveDisplayed VCG01 30 == 1\n\
          \t\tShowLoveTesterMenuParams 40\n\
          \t\tSetStage VCG01 65\n\
          \tendif\n\
          End",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(
            b"SCPT",
            VIGOR_TESTER_SCRIPT,
            "VCG01VigorTesterSCRIPT",
            &vigor_script,
        ),
    ));
    let mut vigor_tester = sub(b"FULL", &zstr("Vit-o-matic Vigor Tester"));
    vigor_tester.extend(sub(b"SCRI", &VIGOR_TESTER_SCRIPT.to_le_bytes()));
    plugin.extend(group(
        *b"ACTI",
        0,
        &named(b"ACTI", VIGOR_TESTER, "VCG01VigorTester", &vigor_tester),
    ));
    let package = |id: u32, name: &str, kind: u8| {
        let mut d = sub(b"PKDT", &[0, 0, 0, 0, kind, 0, 0, 0, 0, 0, 0, 0]);
        d.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
        named(b"PACK", id, name, &d)
    };
    let mut packages = package(TRAVEL, "TestTravel", 6);
    packages.extend(package(SANDBOX, "TestSandbox", 12));
    plugin.extend(group(*b"PACK", 0, &packages));
    // The house is owned by the town and lies in the test region.
    let cell = |id: u32, name: &str, owner: Option<u32>| {
        let mut d = sub(b"DATA", &[1]);
        if let Some(o) = owner {
            d.extend(sub(b"XOWN", &o.to_le_bytes()));
            d.extend(sub(b"XCLR", &REGION.to_le_bytes()));
        }
        named(b"CELL", id, name, &d)
    };
    plugin.extend(group(
        *b"REGN",
        0,
        &named(b"REGN", REGION, "TestRegion", &[]),
    ));
    let cell_group =
        |id: u32, refs: &[u8]| group(id.to_le_bytes(), 6, &group(id.to_le_bytes(), 9, refs));
    let mut contents = cell(HOUSE, "TestHouse", Some(TOWN));
    contents.extend(cell_group(HOUSE, &refs));
    contents.extend(cell(CELLAR, "TestHouseCellar", None));
    contents.extend(cell_group(
        CELLAR,
        &thing(
            b"REFR",
            CELLAR_CUP_REF,
            CUP,
            [0.0; 3],
            [0.0; 3],
            "CellarCupRef",
            &[],
        ),
    ));
    contents.extend(cell(ELSEWHERE, "Elsewhere", None));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &contents)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
}
