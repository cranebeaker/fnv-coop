//! A world for the AI's rules (`world::movement`, `world::social`,
//! `world::ai`'s dialogue packages): `TestAiCell`, an interior with one
//! square navmesh 2000 across; people who greet, chatter and talk to each
//! other (with `HELLO`, `GOODBYE`, `IdleChatter` and a follow-up topic);
//! three dialogue packages (talk to the player, "Say To" the player, talk
//! once the player is in a trigger box, `PLD2`); two creatures with and
//! without their own turning speed (`TNAM`); an `XMarker`, an
//! `XMarkerHeading`, a chair, food on the floor and in a pocket; travel and
//! wander packages to them.

use crate::{condition, f32s, group, placed, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const PLAYER: u32 = 0x7;
    pub const PLAYER_REF: u32 = 0x14;
    /// The hard-coded topics.
    pub const HELLO: u32 = 0xD2;
    pub const GOODBYE: u32 = 0xD4;
    pub const IDLE_CHATTER: u32 = 0xD5;
    /// The game's markers.
    pub const X_MARKER_HEADING: u32 = 0x34;
    pub const X_MARKER: u32 = 0x3B;
    pub const GLOBALS: u32 = 0x1100;
    /// Running from the start; every line here is its.
    pub const QUEST: u32 = 0x1110;
    pub const VOICE: u32 = 0x1111;
    /// "What's new?" (Chatty's `HELLO` line leads to it) and a topic the
    /// talker's package names.
    pub const FOLLOW_UP: u32 = 0x1112;
    pub const TALK_TOPIC: u32 = 0x1113;
    // Lines.
    /// Chatty to Chatty Two only (`GetIsID` asked of the target): "Hello,
    /// neighbour.", follow-up `FOLLOW_UP`, the other speaks next.
    pub const CHATTY_HELLO: u32 = 0x1120;
    /// Chatty Two: "Not much."
    pub const CHATTY2_FOLLOW_UP: u32 = 0x1121;
    /// Chatty: "Bye now."
    pub const CHATTY_GOODBYE: u32 = 0x1122;
    /// The greeter to anyone: "Hi there."
    pub const GREETER_HELLO: u32 = 0x1123;
    /// The greeter: "Nice day."
    pub const GREETER_CHATTER: u32 = 0x1124;
    /// The talker on its package's topic.
    pub const TALKER_LINE: u32 = 0x1125;
    // People and creatures (bases).
    pub const GREETER: u32 = 0x1130;
    pub const CHATTY: u32 = 0x1131;
    pub const CHATTY2: u32 = 0x1132;
    pub const TALKER: u32 = 0x1133;
    pub const SAYER: u32 = 0x1134;
    pub const WAITER: u32 = 0x1135;
    /// `TNAM` 200 and 0.
    pub const FAST_CREATURE: u32 = 0x1136;
    pub const SLOW_CREATURE: u32 = 0x1137;
    // Packages.
    /// Type 15: the player, distance 0, about `TALK_TOPIC`.
    pub const TALK_PACKAGE: u32 = 0x1140;
    /// Type 15, "Say To": the player, distance 0.
    pub const SAY_PACKAGE: u32 = 0x1141;
    /// Type 15: the player, distance 256, once the player is in the
    /// trigger (`PLD2`).
    pub const WAIT_PACKAGE: u32 = 0x1142;
    /// Travel to the marker, to the chair, and to the trigger (radius 0).
    pub const TO_MARKER: u32 = 0x1143;
    pub const TO_CHAIR: u32 = 0x1144;
    pub const TO_TRIGGER: u32 = 0x1145;
    /// Travel to the `XMarkerHeading` (radius 0).
    pub const TO_HEADING_MARKER: u32 = 0x1146;
    /// Wander packages (type 5): near the `XMarkerHeading` within 40 (too
    /// small: they stand), near the `XMarker` within 400, near the trigger
    /// with radius 0 (an activator: half its bounds' diagonal), and "in a
    /// cell" within 300.
    pub const WANDER_SMALL: u32 = 0x1147;
    pub const WANDER_WIDE: u32 = 0x1148;
    pub const WANDER_TRIGGER: u32 = 0x1149;
    pub const WANDER_IN_CELL: u32 = 0x114A;
    /// Type 15: the player, distance 256, while in `CELL` (`PLD2` kind 1).
    pub const CELL_WAIT_PACKAGE: u32 = 0x114B;
    // Objects (bases).
    pub const CHAIR: u32 = 0x1150;
    /// An activator, bounds 40 × 40 × 40.
    pub const TRIGGER: u32 = 0x1151;
    pub const FOOD: u32 = 0x1152;
    // The cell and what's in it.
    pub const CELL: u32 = 0x1160;
    pub const NAVMESH: u32 = 0x1161;
    pub const GREETER_REF: u32 = 0x1170;
    pub const CHATTY_REF: u32 = 0x1171;
    pub const CHATTY2_REF: u32 = 0x1172;
    pub const TALKER_REF: u32 = 0x1173;
    pub const SAYER_REF: u32 = 0x1174;
    pub const WAITER_REF: u32 = 0x1175;
    pub const FAST_REF: u32 = 0x1176;
    pub const SLOW_REF: u32 = 0x1177;
    /// At (300, 300).
    pub const MARKER_REF: u32 = 0x1178;
    /// At (-300, 300).
    pub const CHAIR_REF: u32 = 0x1179;
    /// At (600, 0), a box 100 each way (`XPRM`).
    pub const TRIGGER_REF: u32 = 0x117A;
    /// At (100, -100).
    pub const FOOD_REF: u32 = 0x117B;
    /// An `XMarkerHeading` at (-300, -300), turned 90° (z: facing east).
    pub const HEADING_MARKER_REF: u32 = 0x117C;
}

/// The world (see the module notes). The people stand at: the greeter
/// (0, 0), Chatty (0, 200), Chatty Two (0, 300), the talker (-500, 0),
/// the sayer (-500, -500), the waiter (600, -200); the creatures at
/// (800, 800) and (-800, 800). The greeter carries a piece of food.
pub fn world(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    let mut globals = Vec::new();
    for (i, (name, value)) in [
        ("GameHour", 12.0f32),
        ("TimeScale", 30.0),
        ("GameDaysPassed", 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut d = edid(name);
        d.extend(sub(b"FNAM", b"f"));
        d.extend(sub(b"FLTV", &value.to_le_bytes()));
        globals.extend(record(b"GLOB", GLOBALS + i as u32, &d));
    }

    let mut quest = edid("TestTalkQuest");
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(1.0f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    let mut voice = edid("TestVoice");
    voice.extend(sub(b"DNAM", &[0]));

    // Topics and lines.
    let topic = |id: u32, name: &str, full: &str| {
        let mut d = edid(name);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"FULL", &zstr(full)));
        d.extend(sub(b"DATA", &[0, 0]));
        record(b"DIAL", id, &d)
    };
    // A line: next speaker, text, said by `by` (`GetIsID`, function 72),
    // to `to` if given (`GetIsID` asked of the target), follow-ups.
    let line = |id: u32, next: u8, text: &str, by: u32, to: Option<u32>, follow: &[u32]| {
        let mut d = sub(b"DATA", &[0, next, 0, 0]);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"TRDT", &[0; 24]));
        d.extend(sub(b"NAM1", &zstr(text)));
        d.extend(condition(72, [by, 0], 1.0));
        if let Some(t) = to {
            let mut c = condition(72, [t, 0], 1.0);
            c[6 + 20] = 1;
            d.extend(c);
        }
        for f in follow {
            d.extend(sub(b"TCLT", &f.to_le_bytes()));
        }
        record(b"INFO", id, &d)
    };
    let mut dialogue = topic(HELLO, "HELLO", "Hello");
    let mut hello = line(
        CHATTY_HELLO,
        0,
        "Hello, neighbour.",
        CHATTY,
        Some(CHATTY2),
        &[FOLLOW_UP],
    );
    hello.extend(line(GREETER_HELLO, 0, "Hi there.", GREETER, None, &[]));
    dialogue.extend(group(HELLO.to_le_bytes(), 7, &hello));
    dialogue.extend(topic(GOODBYE, "GOODBYE", "Goodbye."));
    dialogue.extend(group(
        GOODBYE.to_le_bytes(),
        7,
        &line(CHATTY_GOODBYE, 0, "Bye now.", CHATTY, None, &[]),
    ));
    dialogue.extend(topic(IDLE_CHATTER, "IdleChatter", "Idle Chatter"));
    dialogue.extend(group(
        IDLE_CHATTER.to_le_bytes(),
        7,
        &line(GREETER_CHATTER, 0, "Nice day.", GREETER, None, &[]),
    ));
    dialogue.extend(topic(FOLLOW_UP, "TestWhatsNew", "What's new?"));
    dialogue.extend(group(
        FOLLOW_UP.to_le_bytes(),
        7,
        &line(CHATTY2_FOLLOW_UP, 0, "Not much.", CHATTY2, None, &[]),
    ));
    dialogue.extend(topic(TALK_TOPIC, "TestTalkTopic", "Listen."));
    dialogue.extend(group(
        TALK_TOPIC.to_le_bytes(),
        7,
        &line(TALKER_LINE, 0, "Listen to me.", TALKER, None, &[]),
    ));

    // Packages: PKDT flags, type; PLDT; PSDT any time; PTDT; PKDD.
    let pkdt = |kind: u8| sub(b"PKDT", &[0, 0, 0, 0, kind, 0, 0, 0, 0, 0, 0, 0]);
    let pldt = |kind: u32, form: u32, radius: u32| {
        let mut d = kind.to_le_bytes().to_vec();
        d.extend(form.to_le_bytes());
        d.extend(radius.to_le_bytes());
        d
    };
    let any_time = sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]);
    let ptdt = |form: u32, distance: u32| {
        let mut d = 0u32.to_le_bytes().to_vec();
        d.extend(form.to_le_bytes());
        d.extend(distance.to_le_bytes());
        d.extend([0; 4]);
        sub(b"PTDT", &d)
    };
    let pkdd = |topic: u32, say_to: bool| {
        let mut d = vec![0u8; 24];
        d[0..4].copy_from_slice(&100.0f32.to_le_bytes());
        d[4..8].copy_from_slice(&topic.to_le_bytes());
        d[16] = u8::from(say_to);
        sub(b"PKDD", &d)
    };
    let package = |id: u32, name: &str, parts: &[Vec<u8>]| {
        let mut d = edid(name);
        for p in parts {
            d.extend(p);
        }
        record(b"PACK", id, &d)
    };
    let mut packages = package(
        TALK_PACKAGE,
        "TestTalkToPlayer",
        &[
            pkdt(15),
            sub(b"PLDT", &pldt(3, 0, 0)),
            any_time.clone(),
            ptdt(PLAYER_REF, 0),
            pkdd(TALK_TOPIC, false),
        ],
    );
    packages.extend(package(
        SAY_PACKAGE,
        "TestSayToPlayer",
        &[
            pkdt(15),
            any_time.clone(),
            ptdt(PLAYER_REF, 0),
            pkdd(0, true),
        ],
    ));
    packages.extend(package(
        WAIT_PACKAGE,
        "TestTalkInTrigger",
        &[
            pkdt(15),
            sub(b"PLDT", &pldt(3, 0, 0)),
            sub(b"PLD2", &pldt(0, TRIGGER_REF, 0)),
            any_time.clone(),
            ptdt(PLAYER_REF, 256),
            pkdd(0, false),
        ],
    ));
    packages.extend(package(
        CELL_WAIT_PACKAGE,
        "TestTalkInCell",
        &[
            pkdt(15),
            sub(b"PLDT", &pldt(3, 0, 0)),
            sub(b"PLD2", &pldt(1, CELL, 0)),
            any_time.clone(),
            ptdt(PLAYER_REF, 256),
            pkdd(0, false),
        ],
    ));
    packages.extend(package(
        TO_MARKER,
        "TestToMarker",
        &[
            pkdt(6),
            sub(b"PLDT", &pldt(0, MARKER_REF, 0)),
            any_time.clone(),
        ],
    ));
    packages.extend(package(
        TO_CHAIR,
        "TestToChair",
        &[
            pkdt(6),
            sub(b"PLDT", &pldt(0, CHAIR_REF, 0)),
            any_time.clone(),
        ],
    ));
    packages.extend(package(
        TO_TRIGGER,
        "TestToTrigger",
        &[
            pkdt(6),
            sub(b"PLDT", &pldt(0, TRIGGER_REF, 0)),
            any_time.clone(),
        ],
    ));
    for (id, name, kind, location) in [
        (
            TO_HEADING_MARKER,
            "TestToHeadingMarker",
            6,
            pldt(0, HEADING_MARKER_REF, 0),
        ),
        (
            WANDER_SMALL,
            "TestWanderSmall",
            5,
            pldt(0, HEADING_MARKER_REF, 40),
        ),
        (WANDER_WIDE, "TestWanderWide", 5, pldt(0, MARKER_REF, 400)),
        (
            WANDER_TRIGGER,
            "TestWanderTrigger",
            5,
            pldt(0, TRIGGER_REF, 0),
        ),
        (WANDER_IN_CELL, "TestWanderInCell", 5, pldt(1, CELL, 300)),
    ] {
        packages.extend(package(
            id,
            name,
            &[pkdt(kind), sub(b"PLDT", &location), any_time.clone()],
        ));
    }

    // People: ACBS (female flag 0), voice, bounds 40 × 30 × 128.
    let bounds = |lo: [i16; 3], hi: [i16; 3]| {
        let mut d = Vec::new();
        for v in lo.iter().chain(&hi) {
            d.extend(v.to_le_bytes());
        }
        sub(b"OBND", &d)
    };
    let npc = |id: u32, name: &str, package: Option<u32>, items: &[(u32, i32)]| {
        let mut d = edid(name);
        d.extend(bounds([-20, -15, 0], [20, 15, 128]));
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"ACBS", &[0; 24]));
        d.extend(sub(b"VTCK", &VOICE.to_le_bytes()));
        if let Some(p) = package {
            d.extend(sub(b"PKID", &p.to_le_bytes()));
        }
        for (item, n) in items {
            let mut c = item.to_le_bytes().to_vec();
            c.extend(n.to_le_bytes());
            d.extend(sub(b"CNTO", &c));
        }
        record(b"NPC_", id, &d)
    };
    let mut player = edid("Player");
    player.extend(sub(b"ACBS", &[0; 24]));
    let mut npcs = record(b"NPC_", PLAYER, &player);
    npcs.extend(npc(GREETER, "TestGreeter", None, &[(FOOD, 1)]));
    npcs.extend(npc(CHATTY, "TestChatty", None, &[]));
    npcs.extend(npc(CHATTY2, "TestChattyTwo", None, &[]));
    npcs.extend(npc(TALKER, "TestTalker", Some(TALK_PACKAGE), &[]));
    npcs.extend(npc(SAYER, "TestSayer", Some(SAY_PACKAGE), &[]));
    npcs.extend(npc(WAITER, "TestWaiter", Some(WAIT_PACKAGE), &[]));
    let creature = |id: u32, name: &str, turning: f32| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        d.extend(sub(b"TNAM", &turning.to_le_bytes()));
        record(b"CREA", id, &d)
    };
    let mut creatures = creature(FAST_CREATURE, "TestFastCreature", 200.0);
    creatures.extend(creature(SLOW_CREATURE, "TestSlowCreature", 0.0));

    let mut statics = record(b"STAT", X_MARKER_HEADING, &edid("XMarkerHeading"));
    statics.extend(record(b"STAT", X_MARKER, &edid("XMarker")));
    let mut chair = edid("TestAiChair");
    chair.extend(bounds([-20, -20, 0], [20, 20, 40]));
    chair.extend(sub(b"MNAM", &0x4000_0001u32.to_le_bytes()));
    let mut trigger = edid("TestAiTrigger");
    trigger.extend(bounds([-20, -20, -20], [20, 20, 20]));
    let mut food = edid("TestFoodItem");
    food.extend(sub(b"FULL", &zstr("Food")));
    food.extend(bounds([-5, -5, 0], [5, 5, 10]));

    // The cell: the navmesh square (-1000, -1000)–(1000, 1000) as two
    // triangles, and everyone.
    let mut refs = Vec::new();
    let mut navmesh = sub(b"NVER", &11u32.to_le_bytes());
    navmesh.extend(sub(
        b"NVVX",
        &f32s(&[
            -1000.0, -1000.0, 0.0, 1000.0, -1000.0, 0.0, 1000.0, 1000.0, 0.0, -1000.0, 1000.0, 0.0,
        ]),
    ));
    let tri = |v: [u16; 3], n: [u16; 3]| {
        let mut d = Vec::new();
        for x in v.into_iter().chain(n) {
            d.extend(x.to_le_bytes());
        }
        d.extend([0; 4]);
        d
    };
    let mut triangles = tri([0, 1, 2], [0xFFFF, 0xFFFF, 1]);
    triangles.extend(tri([0, 2, 3], [0, 0xFFFF, 0xFFFF]));
    navmesh.extend(sub(b"NVTR", &triangles));
    refs.extend(record(b"NAVM", NAVMESH, &navmesh));
    let actor = |id: u32, base: u32, pos: [f32; 3], kind: &[u8; 4]| {
        let mut r = placed(id, base, pos, [0.0; 3], &[]);
        r[..4].copy_from_slice(kind);
        r
    };
    for (id, base, pos) in [
        (GREETER_REF, GREETER, [0.0, 0.0, 0.0]),
        (CHATTY_REF, CHATTY, [0.0, 200.0, 0.0]),
        (CHATTY2_REF, CHATTY2, [0.0, 300.0, 0.0]),
        (TALKER_REF, TALKER, [-500.0, 0.0, 0.0]),
        (SAYER_REF, SAYER, [-500.0, -500.0, 0.0]),
        (WAITER_REF, WAITER, [600.0, -200.0, 0.0]),
    ] {
        refs.extend(actor(id, base, pos, b"ACHR"));
    }
    refs.extend(actor(FAST_REF, FAST_CREATURE, [800.0, 800.0, 0.0], b"ACRE"));
    refs.extend(actor(
        SLOW_REF,
        SLOW_CREATURE,
        [-800.0, 800.0, 0.0],
        b"ACRE",
    ));
    refs.extend(placed(
        MARKER_REF,
        X_MARKER,
        [300.0, 300.0, 0.0],
        [0.0; 3],
        &[],
    ));
    refs.extend(placed(
        CHAIR_REF,
        CHAIR,
        [-300.0, 300.0, 0.0],
        [0.0; 3],
        &[],
    ));
    let mut xprm = f32s(&[100.0, 100.0, 100.0, 0.5, 0.5, 0.5, 0.0]);
    xprm.extend(1u32.to_le_bytes());
    refs.extend(placed(
        TRIGGER_REF,
        TRIGGER,
        [600.0, 0.0, 0.0],
        [0.0; 3],
        &sub(b"XPRM", &xprm),
    ));
    refs.extend(placed(FOOD_REF, FOOD, [100.0, -100.0, 0.0], [0.0; 3], &[]));
    refs.extend(placed(
        HEADING_MARKER_REF,
        X_MARKER_HEADING,
        [-300.0, -300.0, 0.0],
        [0.0, 0.0, 90.0],
        &[],
    ));
    let mut cell = edid("TestAiCell");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", CELL, &cell);
    contents.extend(group(
        CELL.to_le_bytes(),
        6,
        &group(CELL.to_le_bytes(), 9, &refs),
    ));
    let cells = group(*b"CELL", 0, &group([0; 4], 2, &group([0; 4], 3, &contents)));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GLOB", 0, &globals));
    plugin.extend(group(*b"STAT", 0, &statics));
    plugin.extend(group(*b"FURN", 0, &record(b"FURN", CHAIR, &chair)));
    plugin.extend(group(*b"ACTI", 0, &record(b"ACTI", TRIGGER, &trigger)));
    plugin.extend(group(*b"INGR", 0, &record(b"INGR", FOOD, &food)));
    plugin.extend(group(*b"PACK", 0, &packages));
    plugin.extend(group(*b"QUST", 0, &record(b"QUST", QUEST, &quest)));
    plugin.extend(group(*b"VTYP", 0, &record(b"VTYP", VOICE, &voice)));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(group(*b"CREA", 0, &creatures));
    plugin.extend(group(*b"DIAL", 0, &dialogue));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}
