//! Dialogue the way the opening's farewell is written (Doc Mitchell's
//! chain at his front door, `VCG01DocMitchellTopic076`, rebuilt from
//! scratch): a greeting line that goes straight on (`TCFU`) to a gift
//! line, which goes on to one of two lines by a global's value; those
//! offer a topic (`TCLT`) whose answer goes on to a Goodbye line with both
//! result scripts. Besides: a run of random lines, lines for low and high
//! Intelligence, a `GOODBYE` topic line without the Goodbye flag, and a
//! "run immediately" line whose follow-up the menu doesn't take.

use crate::{condition, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const PLAYER: u32 = 0x7;
    pub const GREETING: u32 = 0xC8;
    pub const GOODBYE: u32 = 0xD4;
    /// Runs from the start; every line here is its.
    pub const QUEST: u32 = 0x1200;
    /// Chooses which of the two middle lines follows (0: `MIDDLE_B`).
    pub const SWITCH: u32 = 0x1201;
    /// Set to 1 by the Goodbye line's first script, 2 by its second.
    pub const DOOR_OPEN: u32 = 0x1202;
    /// `iDialogueDummySpeakThisIntOrBelow` 3, as the game's data sets it.
    pub const DUMMY_SETTING: u32 = 0x1203;
    pub const DOC: u32 = 0x1210;
    /// Topics.
    pub const OUTRO: u32 = 0x1220;
    pub const THANKS: u32 = 0x1221;
    pub const RANDOM_TOPIC: u32 = 0x1222;
    pub const SMART_TOPIC: u32 = 0x1223;
    pub const IMMEDIATE_TOPIC: u32 = 0x1224;
    /// Lines.
    /// The greeting: said once, goes on to `GIFT`.
    pub const WELCOME: u32 = 0x1230;
    /// Under `OUTRO`: goes on to `MIDDLE_A` or `MIDDLE_B`.
    pub const GIFT: u32 = 0x1231;
    /// `SWITCH` 1 / 0; both offer `THANKS`.
    pub const MIDDLE_A: u32 = 0x1232;
    pub const MIDDLE_B: u32 = 0x1233;
    /// Under `OUTRO`, flagged Goodbye: sets `DOOR_OPEN` 1, then 2.
    pub const BYE: u32 = 0x1234;
    /// The answer to `THANKS`: goes on to `BYE`.
    pub const THANKS_LINE: u32 = 0x1235;
    /// Under `RANDOM_TOPIC`: random, random-end, then plain.
    pub const RANDOM_1: u32 = 0x1236;
    pub const RANDOM_2: u32 = 0x1237;
    pub const PLAIN: u32 = 0x1238;
    /// Under `SMART_TOPIC`: low then high Intelligence.
    pub const DUMB_LINE: u32 = 0x1239;
    pub const SMART_LINE: u32 = 0x123A;
    /// Under `GOODBYE`, without the Goodbye flag.
    pub const SEE_YOU: u32 = 0x123B;
    /// Under `IMMEDIATE_TOPIC`: flagged 0x08, goes on to `BYE`.
    pub const IMMEDIATE_LINE: u32 = 0x123C;
    /// The player's main list: top-level topics the doctor answers only
    /// while `LIST_ON` is 1. The hard-coded refusal topics (top-level here,
    /// to show they're left out anyway), a conversation-kind one (as the
    /// game's `GOODBYE` is) of priority 5, an ordinary one of 60, and ones
    /// for low (0x10, 70) and high (0x20, 80) Intelligence players.
    pub const LIST_ON: u32 = 0x1204;
    pub const SPEECH_FAILURE: u32 = 0xFD;
    pub const REFUSAL: u32 = 0x118;
    pub const CHAT: u32 = 0x1225;
    pub const ASK: u32 = 0x1226;
    pub const DUMB_ASK: u32 = 0x1227;
    pub const SMART_ASK: u32 = 0x1228;
    pub const FAILURE_LINE: u32 = 0x123D;
    pub const REFUSAL_LINE: u32 = 0x123E;
    pub const CHAT_LINE: u32 = 0x123F;
    pub const ASK_LINE: u32 = 0x1240;
    pub const DUMB_ASK_LINE: u32 = 0x1241;
    pub const SMART_ASK_LINE: u32 = 0x1242;
}

/// A line: its `DATA` flags (byte 2 and 3), text, said by the doctor,
/// with extra subrecords.
fn line(id: u32, flags: u8, flags2: u8, text: &str, extra: &[u8]) -> Vec<u8> {
    use ids::*;
    let mut d = sub(b"DATA", &[0, 0, flags, flags2]);
    d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
    d.extend(sub(b"TRDT", &[0; 24]));
    d.extend(sub(b"NAM1", &zstr(text)));
    d.extend(condition(72, [DOC, 0], 1.0));
    d.extend(extra);
    record(b"INFO", id, &d)
}

fn follow(ids: &[u32]) -> Vec<u8> {
    ids.iter()
        .flat_map(|i| sub(b"TCFU", &i.to_le_bytes()))
        .collect()
}

/// The data (see the module notes).
pub fn world(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let topic = |id: u32, name: &str, full: &str| {
        let mut d = edid(name);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"FULL", &zstr(full)));
        d.extend(sub(b"DATA", &[0, 0]));
        record(b"DIAL", id, &d)
    };

    let mut globals = Vec::new();
    for (id, name) in [
        (SWITCH, "TestSwitch"),
        (DOOR_OPEN, "TestDoorOpen"),
        (LIST_ON, "TestListOn"),
    ] {
        let mut d = edid(name);
        d.extend(sub(b"FNAM", b"s"));
        d.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
        globals.extend(record(b"GLOB", id, &d));
    }
    let mut setting = edid("iDialogueDummySpeakThisIntOrBelow");
    setting.extend(sub(b"DATA", &3i32.to_le_bytes()));

    let mut quest = edid("TestOutroQuest");
    let mut qdata = vec![0x01, 50, 0, 0];
    qdata.extend(1.0f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));

    let npc = |id: u32, name: &str| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        record(b"NPC_", id, &d)
    };
    let mut npcs = npc(PLAYER, "Player");
    npcs.extend(npc(DOC, "TestDoc"));

    // The greeting goes on to the gift.
    let mut dialogue = topic(GREETING, "GREETING", "GREETING");
    dialogue.extend(crate::group(
        GREETING.to_le_bytes(),
        7,
        &line(WELCOME, 0x04, 0, "Here. These are yours.", &follow(&[GIFT])),
    ));
    // The outro: gift, the two middles, goodbye.
    dialogue.extend(topic(OUTRO, "TestOutro", "<Outro>"));
    let mut outro = line(
        GIFT,
        0,
        0,
        "You ought to have this.",
        &follow(&[MIDDLE_A, MIDDLE_B]),
    );
    for (id, value, text) in [
        (MIDDLE_A, 1.0, "Was my wife's."),
        (MIDDLE_B, 0.0, "Never was my style."),
    ] {
        let mut extra = condition(74, [SWITCH, 0], value);
        extra.extend(sub(b"TCLT", &THANKS.to_le_bytes()));
        outro.extend(line(id, 0, 0, text, &extra));
    }
    let mut scripts = sub(b"SCHR", &[0; 20]);
    scripts.extend(sub(b"SCTX", b"set TestDoorOpen to 1"));
    scripts.extend(sub(b"NEXT", &[]));
    scripts.extend(sub(b"SCHR", &[0; 20]));
    scripts.extend(sub(b"SCTX", b"set TestDoorOpen to 2"));
    outro.extend(line(BYE, 0x01, 0, "Try not to get killed.", &scripts));
    dialogue.extend(crate::group(OUTRO.to_le_bytes(), 7, &outro));
    dialogue.extend(topic(THANKS, "TestThanks", "Thanks, Doc."));
    dialogue.extend(crate::group(
        THANKS.to_le_bytes(),
        7,
        &line(THANKS_LINE, 0, 0, "Don't mention it.", &follow(&[BYE])),
    ));
    // Random lines.
    dialogue.extend(topic(RANDOM_TOPIC, "TestRandom", "Random"));
    let mut random = line(RANDOM_1, 0x02, 0, "One.", &[]);
    random.extend(line(RANDOM_2, 0x02 | 0x20, 0, "Two.", &[]));
    random.extend(line(PLAIN, 0, 0, "Three.", &[]));
    dialogue.extend(crate::group(RANDOM_TOPIC.to_le_bytes(), 7, &random));
    // Intelligence classes.
    dialogue.extend(topic(SMART_TOPIC, "TestSmart", "Smart"));
    let mut smart = line(DUMB_LINE, 0, 0x10, "Me no think.", &[]);
    smart.extend(line(SMART_LINE, 0, 0x20, "Indubitably.", &[]));
    dialogue.extend(crate::group(SMART_TOPIC.to_le_bytes(), 7, &smart));
    // `GOODBYE` without the flag.
    dialogue.extend(topic(GOODBYE, "GOODBYE", "Goodbye."));
    dialogue.extend(crate::group(
        GOODBYE.to_le_bytes(),
        7,
        &line(SEE_YOU, 0, 0, "See you.", &[]),
    ));
    // Run immediately: the menu takes no follow-up.
    dialogue.extend(topic(IMMEDIATE_TOPIC, "TestImmediate", "Now"));
    dialogue.extend(crate::group(
        IMMEDIATE_TOPIC.to_le_bytes(),
        7,
        &line(IMMEDIATE_LINE, 0x08, 0, "At once.", &follow(&[BYE])),
    ));
    // The player's main list.
    for (id, name, kind, flags, priority, line_id, text) in [
        (
            SPEECH_FAILURE,
            "SpeechChallengeFailure",
            0u8,
            0x02u8,
            90.0f32,
            FAILURE_LINE,
            "No.",
        ),
        (
            REFUSAL,
            "InfoRefusal",
            0,
            0x02,
            90.0,
            REFUSAL_LINE,
            "Not telling.",
        ),
        (CHAT, "TestChat", 1, 0x02, 5.0, CHAT_LINE, "Chat."),
        (ASK, "TestAsk", 0, 0x02, 60.0, ASK_LINE, "Ask."),
        (
            DUMB_ASK,
            "TestDumbAsk",
            0,
            0x02 | 0x10,
            70.0,
            DUMB_ASK_LINE,
            "Dumb.",
        ),
        (
            SMART_ASK,
            "TestSmartAsk",
            0,
            0x02 | 0x20,
            80.0,
            SMART_ASK_LINE,
            "Smart.",
        ),
    ] {
        let mut d = edid(name);
        d.extend(sub(b"QSTI", &QUEST.to_le_bytes()));
        d.extend(sub(b"FULL", &zstr(name)));
        d.extend(sub(b"PNAM", &priority.to_le_bytes()));
        d.extend(sub(b"DATA", &[kind, flags]));
        dialogue.extend(record(b"DIAL", id, &d));
        dialogue.extend(crate::group(
            id.to_le_bytes(),
            7,
            &line(line_id, 0, 0, text, &condition(74, [LIST_ON, 0], 1.0)),
        ));
    }

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(crate::group(
        *b"GMST",
        0,
        &record(b"GMST", DUMMY_SETTING, &setting),
    ));
    plugin.extend(crate::group(*b"GLOB", 0, &globals));
    plugin.extend(crate::group(*b"QUST", 0, &record(b"QUST", QUEST, &quest)));
    plugin.extend(crate::group(*b"NPC_", 0, &npcs));
    plugin.extend(crate::group(*b"DIAL", 0, &dialogue));
    data.write("FalloutNV.esm", &plugin);
    data
}
