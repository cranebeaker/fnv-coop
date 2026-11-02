//! A world for the script functions in `world::more_functions`: two people
//! (one essential), a creature and a leveled list of it, a combat style,
//! two talking activators (one a radio station broadcasting from the
//! start), a cup, two destructible activators, challenges (scripted, a
//! recurring one, one starting disabled, one counting completed
//! challenges) with a script, an idle, a marker, and the interior
//! `TestRoom` holding one of each.

use crate::{group, placed, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const VALUE: u32 = 0xE00;
    /// Raised by one each time `TestChallengeScript` runs.
    pub const DONE: u32 = 0xE01;
    pub const PERSON: u32 = 0xE02;
    /// `ACBS` flag 0x02.
    pub const HERO: u32 = 0xE03;
    pub const DOG: u32 = 0xE04;
    /// Gives `TestDog` at level 1.
    pub const DOGS: u32 = 0xE05;
    pub const STYLE: u32 = 0xE06;
    /// A talking activator whose record has form flag 0x40000000 (a
    /// station broadcasting).
    pub const RADIO: u32 = 0xE07;
    pub const TALKER: u32 = 0xE08;
    pub const CUP: u32 = 0xE09;
    /// Health 100; stages at 50 % (model `test\barreldamaged.nif`, damage
    /// stage 1) and 0 % (destroys it, damage stage 2).
    pub const BARREL: u32 = 0xE0A;
    /// Health 100; stages at 60 % (caps the damage) and 20 % (disables it).
    pub const CRATE: u32 = 0xE0B;
    pub const CHALLENGE_SCRIPT: u32 = 0xE0C;
    /// Scripted, threshold 3, interval 1, `TestChallengeScript`.
    pub const SCRIPTED: u32 = 0xE0D;
    /// Scripted and recurring, threshold 2.
    pub const RECURRING: u32 = 0xE0E;
    /// Scripted, starts disabled, threshold 1.
    pub const LOCKED: u32 = 0xE0F;
    /// Counts the statistic "Challenges Completed" (27), threshold 2.
    pub const COUNTING: u32 = 0xE10;
    pub const IDLE: u32 = 0xE11;
    /// A marker base (a `STAT` below 0x800 in the master, as `XMarker`).
    pub const MARKER: u32 = 0x3B;
    pub const ROOM: u32 = 0xE20;
    /// At 0,0,0 facing east (90 degrees).
    pub const PERSON_REF: u32 = 0xE30;
    pub const HERO_REF: u32 = 0xE31;
    pub const DOG_REF: u32 = 0xE32;
    pub const RADIO_REF: u32 = 0xE33;
    pub const TALKER_REF: u32 = 0xE34;
    pub const BARREL_REF: u32 = 0xE35;
    pub const CRATE_REF: u32 = 0xE36;
    /// At 500,0,0.
    pub const MARKER_REF: u32 = 0xE37;
    /// A cup whose enable parent is the barrel.
    pub const CHILD_REF: u32 = 0xE38;
    /// The person is in it at rank 2, the hero at rank 0.
    pub const GANG: u32 = 0xE12;
    /// A spell of one script effect (10 s) whose `ScriptEffectUpdate` block
    /// sets `TestElapsed` to `ScriptEffectElapsedSeconds`.
    pub const TICK: u32 = 0xE13;
    pub const TICK_EFFECT: u32 = 0xE14;
    pub const TICK_SCRIPT: u32 = 0xE15;
    pub const ELAPSED: u32 = 0xE16;
    /// A radio topic (`DIAL`, type 7) for `StartRadioConversation`.
    pub const RADIO_TOPIC: u32 = 0xE17;
    /// An effect shader (`EFSH`) for `PlayMagicShaderVisuals`.
    pub const SHADER: u32 = 0xE18;
    /// A Caravan card (`CCRD`) and a cup (`MISC`) with the Caravan cards'
    /// `OnAdd` script (`TestCardScript`), and one of each placed.
    pub const CARD_SCRIPT: u32 = 0xE19;
    pub const CARD: u32 = 0xE1A;
    pub const CARD_CUP: u32 = 0xE1B;
    pub const CARD_REF: u32 = 0xE3A;
    pub const CARD_CUP_REF: u32 = 0xE3B;
    /// `TestTick`'s effect as an ability (`SPEL` type 4) and as a poison
    /// (5), which `DispelAllSpells` leaves.
    pub const TICK_ABILITY: u32 = 0xE1C;
    pub const TICK_POISON: u32 = 0xE1D;
    /// A weapon (`WEAP`, no data) for `FireWeapon`.
    pub const GUN: u32 = 0xE1E;
    /// A casino (`CSNO`, no data) for the casino games' menus.
    pub const CASINO: u32 = 0xE1F;
    /// A placed object whose script (`TestSayerScript`) has `SayToDone`
    /// blocks: `TestRadioTopic` adds 1 to `TestValue`, `TestTick` sets it
    /// to 100.
    pub const SAYER_SCRIPT: u32 = 0xE21;
    pub const SAYER: u32 = 0xE22;
    pub const SAYER_REF: u32 = 0xE3C;
    /// A quest running from the start, whose line of `TestRadioTopic`
    /// (`RADIO_LINE`) is a song (`TestSong`, `radio\testsong.mp3`).
    pub const RADIO_QUEST: u32 = 0xE23;
    pub const RADIO_LINE: u32 = 0xE24;
    pub const SONG: u32 = 0xE25;
    /// `AMLRadio`: largest attenuation distance 20 (× 100 units).
    pub const RADIO_MODEL: u32 = 0xE26;
    /// An item whose `OnAdd` script (`TestMoverScript`) gives itself to
    /// the crate (`RemoveMe CrateRef`).
    pub const MOVER_SCRIPT: u32 = 0xE27;
    pub const MOVER: u32 = 0xE28;
}

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn more(tag: &str) -> TempData {
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

    let global = |id: u32, name: &str| {
        let mut g = sub(b"FNAM", b"f");
        g.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
        named(b"GLOB", id, name, &g)
    };
    let mut globals = global(VALUE, "TestValue");
    globals.extend(global(DONE, "TestDone"));
    globals.extend(global(ELAPSED, "TestElapsed"));
    plugin.extend(group(*b"GLOB", 0, &globals));

    // People: ACBS flags at 0 (0x02 essential).
    let person = |id: u32, name: &str, flags: u32, rank: u8| {
        let mut acbs = flags.to_le_bytes().to_vec();
        acbs.extend([0; 20]);
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"ACBS", &acbs));
        let mut snam = GANG.to_le_bytes().to_vec();
        snam.extend([rank, 0, 0, 0]);
        d.extend(sub(b"SNAM", &snam));
        let mut stats = 100i32.to_le_bytes().to_vec();
        stats.extend([5; 7]);
        d.extend(sub(b"DATA", &stats));
        named(b"NPC_", id, &name.replace(' ', ""), &d)
    };
    let mut npcs = person(PERSON, "Test Person", 0, 2);
    npcs.extend(person(HERO, "Test Hero", 0x02, 0));
    plugin.extend(group(
        *b"FACT",
        0,
        &named(b"FACT", GANG, "TestGang", &sub(b"DATA", &[0; 4])),
    ));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    let mut dog = sub(b"ACBS", &[0; 24]);
    let mut dog_data = vec![0u8, 0, 0, 0];
    dog_data.extend(20i16.to_le_bytes());
    dog_data.extend([0, 0]);
    dog_data.extend(4i16.to_le_bytes());
    dog_data.extend([5; 7]);
    dog.extend(sub(b"DATA", &dog_data));
    plugin.extend(group(*b"CREA", 0, &named(b"CREA", DOG, "TestDog", &dog)));
    let mut entry = 1u16.to_le_bytes().to_vec();
    entry.extend([0, 0]);
    entry.extend(DOG.to_le_bytes());
    entry.extend(1u16.to_le_bytes());
    entry.extend([0, 0]);
    let mut dogs = sub(b"LVLD", &[0]);
    dogs.extend(sub(b"LVLF", &[0]));
    dogs.extend(sub(b"LVLO", &entry));
    plugin.extend(group(*b"LVLC", 0, &named(b"LVLC", DOGS, "TestDogs", &dogs)));
    plugin.extend(group(
        *b"CSTY",
        0,
        &named(b"CSTY", STYLE, "TestStyle", &sub(b"CSTD", &[0; 92])),
    ));

    // Talking activators: the radio's record flags 0x40000000
    // (broadcasting) and 0x20000 (a radio station; placed with `XRDO`).
    let mut radio = named(b"TACT", RADIO, "TestRadio", &sub(b"FULL", &zstr("Radio")));
    radio[8..12].copy_from_slice(&0x4002_0000u32.to_le_bytes());
    radio.extend(named(
        b"TACT",
        TALKER,
        "TestTalker",
        &sub(b"FULL", &zstr("Talker")),
    ));
    plugin.extend(group(*b"TACT", 0, &radio));
    let mut topic_data = sub(b"FULL", &zstr("Radio topic"));
    topic_data.extend(sub(b"DATA", &[7, 0]));
    // Caravan cards: the card script's `OnAdd` (as Dead Money's
    // `NVDLC01CardAddToPlayerScript`), also noting the container.
    let mut card_script = sub(b"SCHR", &[0; 20]);
    card_script.extend(sub(
        b"SCTX",
        b"scn TestCardScript\nbegin OnAdd\n\tset TestValue to GetContainer\n\tif GetContainer != player\n\t\treturn\n\telse\n\t\tAddCardToPlayer\n\t\tRemoveMe\n\tendif\nend",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", CARD_SCRIPT, "TestCardScript", &card_script),
    ));
    let mut sayer_script = sub(b"SCHR", &[0; 20]);
    sayer_script.extend(sub(
        b"SCTX",
        b"scn TestSayerScript\nbegin SayToDone TestRadioTopic\n\tset TestValue to TestValue + 1\nend\nbegin SayToDone TestTick\n\tset TestValue to 100\nend",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", SAYER_SCRIPT, "TestSayerScript", &sayer_script),
    ));
    let mut sayer = sub(b"FULL", &zstr("Sayer"));
    sayer.extend(sub(b"SCRI", &SAYER_SCRIPT.to_le_bytes()));
    sayer.extend(sub(b"DATA", &[0; 8]));
    plugin.extend(group(
        *b"MISC",
        0,
        &named(b"MISC", SAYER, "TestSayer", &sayer),
    ));
    let mut mover_script = sub(b"SCHR", &[0; 20]);
    mover_script.extend(sub(
        b"SCTX",
        b"scn TestMoverScript\nbegin OnAdd player\n\tRemoveMe CrateRef\nend",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", MOVER_SCRIPT, "TestMoverScript", &mover_script),
    ));
    let mut mover = sub(b"FULL", &zstr("Mover"));
    mover.extend(sub(b"SCRI", &MOVER_SCRIPT.to_le_bytes()));
    mover.extend(sub(b"DATA", &[0; 8]));
    plugin.extend(group(
        *b"MISC",
        0,
        &named(b"MISC", MOVER, "TestMover", &mover),
    ));
    let mut card = sub(b"FULL", &zstr("Ace of Clubs"));
    card.extend(sub(b"SCRI", &CARD_SCRIPT.to_le_bytes()));
    // Its suit and value (`INTV` twice, as the game's cards have them).
    card.extend(sub(b"INTV", &1i32.to_le_bytes()));
    card.extend(sub(b"INTV", &1i32.to_le_bytes()));
    card.extend(sub(b"DATA", &1i32.to_le_bytes()));
    plugin.extend(group(*b"CCRD", 0, &named(b"CCRD", CARD, "TestCard", &card)));
    let mut card_cup = sub(b"FULL", &zstr("Lucky cup"));
    card_cup.extend(sub(b"SCRI", &CARD_SCRIPT.to_le_bytes()));
    card_cup.extend(sub(b"DATA", &[0; 8]));
    plugin.extend(group(
        *b"MISC",
        0,
        &named(b"MISC", CARD_CUP, "TestCardCup", &card_cup),
    ));
    plugin.extend(group(
        *b"EFSH",
        0,
        &named(b"EFSH", SHADER, "TestShader", &sub(b"DATA", &[0; 4])),
    ));
    let mut quest = edid("TestRadioQuest");
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(1.0f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", RADIO_QUEST, &quest)));
    let mut sounds = named(
        b"SOUN",
        SONG,
        "TestSong",
        &sub(b"FNAM", &zstr("radio\\testsong.mp3")),
    );
    let mut model = sub(b"FNAM", &zstr("fx\\radio.wav"));
    model.extend(sub(b"SNDD", &[1, 20, 0, 0, 0, 0, 0, 0]));
    sounds.extend(named(b"SOUN", RADIO_MODEL, "AMLRadio", &model));
    plugin.extend(group(*b"SOUN", 0, &sounds));
    // The topic's one line: a song (`TRDT` bytes 16..20).
    let mut trdt = [0u8; 24];
    trdt[16..20].copy_from_slice(&SONG.to_le_bytes());
    let mut line = sub(b"DATA", &[0, 0, 0, 0]);
    line.extend(sub(b"QSTI", &RADIO_QUEST.to_le_bytes()));
    line.extend(sub(b"TRDT", &trdt));
    line.extend(sub(b"NAM1", &zstr("A song.")));
    let mut topic = named(b"DIAL", RADIO_TOPIC, "TestRadioTopic", &topic_data);
    topic.extend(group(
        RADIO_TOPIC.to_le_bytes(),
        7,
        &record(b"INFO", RADIO_LINE, &line),
    ));
    plugin.extend(group(*b"DIAL", 0, &topic));

    let mut cup = sub(b"FULL", &zstr("Cup"));
    let mut v = 1i32.to_le_bytes().to_vec();
    v.extend(1.0f32.to_le_bytes());
    cup.extend(sub(b"DATA", &v));
    plugin.extend(group(*b"MISC", 0, &named(b"MISC", CUP, "TestCup", &cup)));
    plugin.extend(group(
        *b"STAT",
        0,
        &named(
            b"STAT",
            MARKER,
            "XMarker",
            &sub(b"MODL", &zstr("marker_x.nif")),
        ),
    ));

    // Destructible activators: DEST (health, count, flags), then each
    // stage's DSTD (health %, number, damage stage, flags, self damage,
    // explosion, debris, debris count), a model, DSTF.
    let stage = |percent: u8, n: u8, damage_stage: u8, flags: u8, model: Option<&str>| {
        let mut d = vec![percent, n, damage_stage, flags];
        d.extend(0i32.to_le_bytes());
        d.extend([0; 12]);
        let mut s = sub(b"DSTD", &d);
        if let Some(m) = model {
            s.extend(sub(b"DMDL", &zstr(m)));
        }
        s.extend(sub(b"DSTF", &[]));
        s
    };
    let destructible = |health: i32, stages: &[Vec<u8>]| {
        let mut dest = health.to_le_bytes().to_vec();
        dest.extend([stages.len() as u8, 0, 0, 0]);
        let mut d = sub(b"MODL", &zstr("test\\barrel.nif"));
        d.extend(sub(b"DEST", &dest));
        for s in stages {
            d.extend(s);
        }
        d
    };
    let mut acti = named(
        b"ACTI",
        BARREL,
        "TestBarrel",
        &destructible(
            100,
            &[
                stage(50, 0, 1, 0, Some("test\\barreldamaged.nif")),
                stage(0, 1, 2, 0x04, None),
            ],
        ),
    );
    acti.extend(named(
        b"ACTI",
        CRATE,
        "TestCrate",
        &destructible(
            100,
            &[stage(60, 0, 1, 0x01, None), stage(20, 1, 2, 0x02, None)],
        ),
    ));
    plugin.extend(group(*b"ACTI", 0, &acti));

    // Challenges: DATA (kind, threshold, flags, interval, three values).
    let mut script = sub(b"SCHR", &[0; 20]);
    script.extend(sub(
        b"SCTX",
        b"scn TestChallengeScript\nbegin ScriptEffectStart\n\tset TestDone to TestDone + 1\nend",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", CHALLENGE_SCRIPT, "TestChallengeScript", &script),
    ));
    let challenge = |id: u32, name: &str, kind: u32, threshold: u32, flags: u32, v1: u16| {
        let mut d = sub(b"FULL", &zstr(name));
        d.extend(sub(b"SCRI", &CHALLENGE_SCRIPT.to_le_bytes()));
        d.extend(sub(b"DESC", &zstr("Do it.")));
        let mut data = kind.to_le_bytes().to_vec();
        data.extend(threshold.to_le_bytes());
        data.extend(flags.to_le_bytes());
        data.extend(1u32.to_le_bytes());
        data.extend(v1.to_le_bytes());
        data.extend([0; 6]);
        d.extend(sub(b"DATA", &data));
        named(b"CHAL", id, &format!("Test{}", name.replace(' ', "")), &d)
    };
    let mut chals = challenge(SCRIPTED, "Scripted", 13, 3, 0, 0);
    chals.extend(challenge(RECURRING, "Recurring", 13, 2, 0x02, 0));
    chals.extend(challenge(LOCKED, "Locked", 13, 1, 0x01, 0));
    chals.extend(challenge(COUNTING, "Counting", 11, 2, 0, 27));
    plugin.extend(group(*b"CHAL", 0, &chals));
    plugin.extend(group(*b"IDLE", 0, &named(b"IDLE", IDLE, "TestIdle", &[])));

    // A script effect (archetype 1 at 64, its script at 8) in a spell.
    let mut tick = sub(b"SCHR", &[0; 20]);
    tick.extend(sub(
        b"SCTX",
        b"scn TestTickScript\nbegin ScriptEffectUpdate\n\tset TestElapsed to ScriptEffectElapsedSeconds\nend",
    ));
    plugin.extend(group(
        *b"SCPT",
        0,
        &named(b"SCPT", TICK_SCRIPT, "TestTickScript", &tick),
    ));
    let mut mgef = vec![0u8; 72];
    mgef[8..12].copy_from_slice(&TICK_SCRIPT.to_le_bytes());
    mgef[16..20].copy_from_slice(&(-1i32).to_le_bytes());
    mgef[64..68].copy_from_slice(&1u32.to_le_bytes());
    mgef[68..72].copy_from_slice(&(-1i32).to_le_bytes());
    let mut effect = sub(b"FULL", &zstr("Tick"));
    effect.extend(sub(b"DATA", &mgef));
    plugin.extend(group(
        *b"MGEF",
        0,
        &named(b"MGEF", TICK_EFFECT, "TestTickEffect", &effect),
    ));
    let mut spell = sub(b"SPIT", &[0; 16]);
    spell.extend(sub(b"EFID", &TICK_EFFECT.to_le_bytes()));
    let mut efit = 0u32.to_le_bytes().to_vec();
    efit.extend([0; 4]);
    efit.extend(10u32.to_le_bytes());
    efit.extend([0; 4]);
    efit.extend((-1i32).to_le_bytes());
    spell.extend(sub(b"EFIT", &efit));
    let typed = |kind: u32| {
        let mut spit = [0u8; 16];
        spit[..4].copy_from_slice(&kind.to_le_bytes());
        let mut s = sub(b"SPIT", &spit);
        s.extend_from_slice(&spell[sub(b"SPIT", &[0; 16]).len()..]);
        s
    };
    let mut spells = named(b"SPEL", TICK, "TestTick", &spell);
    spells.extend(named(b"SPEL", TICK_ABILITY, "TestTickAbility", &typed(4)));
    spells.extend(named(b"SPEL", TICK_POISON, "TestTickPoison", &typed(5)));
    plugin.extend(group(*b"SPEL", 0, &spells));
    plugin.extend(group(*b"WEAP", 0, &named(b"WEAP", GUN, "TestGun", &[])));
    plugin.extend(group(
        *b"CSNO",
        0,
        &named(b"CSNO", CASINO, "TestCasino", &[]),
    ));

    // The room.
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
        PERSON_REF,
        PERSON,
        [0.0; 3],
        [0.0, 0.0, 90.0],
        "PersonRef",
        &[],
    );
    refs.extend(thing(
        b"ACHR",
        HERO_REF,
        HERO,
        [0.0, 200.0, 0.0],
        [0.0; 3],
        "HeroRef",
        &[],
    ));
    refs.extend(thing(
        b"ACRE",
        DOG_REF,
        DOG,
        [0.0, -200.0, 0.0],
        [0.0; 3],
        "DogRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        RADIO_REF,
        RADIO,
        [200.0, 0.0, 0.0],
        [0.0; 3],
        "RadioRef",
        // Radio data: heard everywhere (range type 1).
        &sub(b"XRDO", &{
            let mut x = 0.0f32.to_le_bytes().to_vec();
            x.extend(1u32.to_le_bytes());
            x.extend(0.0f32.to_le_bytes());
            x.extend(0u32.to_le_bytes());
            x
        }),
    ));
    refs.extend(thing(
        b"REFR",
        TALKER_REF,
        TALKER,
        [200.0, 100.0, 0.0],
        [0.0; 3],
        "TalkerRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        BARREL_REF,
        BARREL,
        [300.0, 0.0, 0.0],
        [0.0; 3],
        "BarrelRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        CRATE_REF,
        CRATE,
        [300.0, 100.0, 0.0],
        [0.0; 3],
        "CrateRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        MARKER_REF,
        MARKER,
        [500.0, 0.0, 0.0],
        [0.0; 3],
        "MarkerRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        CARD_REF,
        CARD,
        [400.0, 0.0, 0.0],
        [0.0; 3],
        "CardRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        CARD_CUP_REF,
        CARD_CUP,
        [400.0, 100.0, 0.0],
        [0.0; 3],
        "CardCupRef",
        &[],
    ));
    refs.extend(thing(
        b"REFR",
        SAYER_REF,
        SAYER,
        [400.0, 200.0, 0.0],
        [0.0; 3],
        "SayerRef",
        &[],
    ));
    let mut xesp = BARREL_REF.to_le_bytes().to_vec();
    xesp.push(0);
    refs.extend(thing(
        b"REFR",
        CHILD_REF,
        CUP,
        [300.0, 0.0, 50.0],
        [0.0; 3],
        "ChildRef",
        &sub(b"XESP", &xesp),
    ));
    let mut contents = named(b"CELL", ROOM, "TestRoom", &sub(b"DATA", &[1]));
    contents.extend(group(
        ROOM.to_le_bytes(),
        6,
        &group(ROOM.to_le_bytes(), 9, &refs),
    ));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &contents)),
    ));
    data.write("FalloutNV.esm", &plugin);
    data
}
