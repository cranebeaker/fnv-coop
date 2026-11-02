//! A world for the character-revision prompt (B29), shaped like the
//! game's `VCG04` (`VCG04QuestScript`, `VCG04ActivatorScript`,
//! `VCG04Message`): an outdoor worldspace `ReviseWorld` with the squares
//! -1,0 (`MAIL`), 0,0 (`HOME`) and 3,0 (`BORDER`), each its own cell.
//!
//! - `ReviseQuest` (starts with the game): once the player is in
//!   `ReviseWorld` east of x 10000, moves `ReviseActivatorRef` to the
//!   player and activates it, once (`Done`).
//! - `ReviseActivatorRef` (persistent, standing in 0,0): `OnActivate` shows
//!   `ReviseMessage` ("Edit Name", "Rebuild Character", "Finished");
//!   `GameMode` asks again when `MessageBoxPending` (as after an edit),
//!   then reads `GetButtonPressed`: 0 counts a rename and 1 a rebuild,
//!   each asking again, 2
//!   counts the end and stops the quest. `iAsked` counts the boxes shown.
//! - `MailboxRef` (in -1,0): reads `GetButtonPressed` every frame, as the
//!   Mojave Express box (`vMojaveExpressBoxSCRIPT`) does once used, and
//!   counts what it got (`iTaken`).

use crate::{group, record, record_flagged, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const WORLD: u32 = 0x3000;
    pub const PERSISTENT_CELL: u32 = 0x3001;
    /// Square -1,0.
    pub const MAIL: u32 = 0x3010;
    /// Square 0,0.
    pub const HOME: u32 = 0x3011;
    /// Square 3,0.
    pub const BORDER: u32 = 0x3012;
    pub const QUEST_SCRIPT: u32 = 0x3100;
    pub const ACTIVATOR_SCRIPT: u32 = 0x3101;
    pub const MAILBOX_SCRIPT: u32 = 0x3102;
    pub const QUEST: u32 = 0x3200;
    pub const MESSAGE: u32 = 0x3201;
    pub const ACTIVATOR: u32 = 0x3202;
    pub const MAILBOX: u32 = 0x3203;
    /// Persistent, standing in 0,0 at 100,100.
    pub const ACTIVATOR_REF: u32 = 0x3300;
    /// In -1,0 at -2000,100.
    pub const MAILBOX_REF: u32 = 0x3301;
}

fn script(id: u32, name: &str, source: &str) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(name));
    d.extend(sub(b"SCHR", &[0; 20]));
    d.extend(sub(b"SCTX", source.as_bytes()));
    record(b"SCPT", id, &d)
}

fn activator(id: u32, name: &str, script: u32) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(name));
    d.extend(sub(b"SCRI", &script.to_le_bytes()));
    record(b"ACTI", id, &d)
}

/// A placed reference with an editor ID.
fn named(id: u32, name: &str, base: u32, pos: [f32; 3], flags: u32) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(name));
    d.extend(sub(b"NAME", &base.to_le_bytes()));
    d.extend(sub(
        b"DATA",
        &crate::f32s(&[pos[0], pos[1], pos[2], 0.0, 0.0, 0.0]),
    ));
    record_flagged(b"REFR", id, flags, &d)
}

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn revise(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let mut scripts = script(
        QUEST_SCRIPT,
        "ReviseQuestScript",
        "scn ReviseQuestScript\nshort Done\n\
         Begin GameMode\n\
         \tif Done == 0\n\
         \t\tif player.GetInWorldSpace ReviseWorld\n\
         \t\t\tif (player.GetPos X > 10000)\n\
         \t\t\t\tReviseActivatorRef.MoveTo player\n\
         \t\t\t\tReviseActivatorRef.Activate player 1\n\
         \t\t\t\tset Done to 1\n\
         \t\t\tendif\n\
         \t\tendif\n\
         \tendif\n\
         End",
    );
    scripts.extend(script(
        ACTIVATOR_SCRIPT,
        "ReviseActivatorScript",
        "scn ReviseActivatorScript\nint Button\nshort MessageBoxPending\n\
         short iAsked\nshort iRenamed\nshort iRebuilt\nshort iFinished\n\
         Begin OnActivate\n\
         \tShowMessage ReviseMessage\n\
         \tset iAsked to iAsked + 1\n\
         End\n\
         Begin GameMode\n\
         \tif MenuMode == 0 && MessageBoxPending\n\
         \t\tShowMessage ReviseMessage\n\
         \t\tset iAsked to iAsked + 1\n\
         \t\tset MessageBoxPending to 0\n\
         \tendif\n\
         \tset Button to GetButtonPressed\n\
         \tif ( Button == 0 )\n\
         \t\tset iRenamed to iRenamed + 1\n\
         \t\tset MessageBoxPending to 1\n\
         \tendif\n\
         \tif ( Button == 1 )\n\
         \t\tset iRebuilt to iRebuilt + 1\n\
         \t\tset MessageBoxPending to 1\n\
         \tendif\n\
         \tif ( Button == 2 )\n\
         \t\tset iFinished to iFinished + 1\n\
         \t\tStopQuest ReviseQuest\n\
         \tendif\n\
         End",
    ));
    scripts.extend(script(
        MAILBOX_SCRIPT,
        "ReviseMailboxScript",
        "scn ReviseMailboxScript\nshort iButton\nshort iTaken\n\
         Begin GameMode\n\
         \tset iButton to GetButtonPressed\n\
         \tif iButton > -1\n\
         \t\tset iTaken to iTaken + 1\n\
         \tendif\n\
         End",
    ));

    let mut quest = sub(b"EDID", &zstr("ReviseQuest"));
    quest.extend(sub(b"SCRI", &QUEST_SCRIPT.to_le_bytes()));
    quest.extend(sub(b"FULL", &zstr("Player Character Revision")));
    // Starts with the game; its own delay 0.1 s.
    let mut qdata = vec![0x01, 0, 0, 0];
    qdata.extend(0.1f32.to_le_bytes());
    quest.extend(sub(b"DATA", &qdata));
    let quests = record(b"QUST", QUEST, &quest);

    let mut message = sub(b"EDID", &zstr("ReviseMessage"));
    message.extend(sub(b"DESC", &zstr("You may revise your character.")));
    message.extend(sub(b"DNAM", &1u32.to_le_bytes()));
    message.extend(sub(b"ITXT", &zstr("Edit Name")));
    message.extend(sub(b"ITXT", &zstr("Rebuild Character")));
    message.extend(sub(b"ITXT", &zstr("Finished")));
    let messages = record(b"MESG", MESSAGE, &message);

    let mut activators = activator(ACTIVATOR, "ReviseActivator", ACTIVATOR_SCRIPT);
    activators.extend(activator(MAILBOX, "ReviseMailbox", MAILBOX_SCRIPT));

    let children = |cell: u32, kind: i32, contents: &[u8]| {
        group(
            cell.to_le_bytes(),
            6,
            &group(cell.to_le_bytes(), kind, contents),
        )
    };
    let exterior_cell = |id: u32, x: i32, y: i32| {
        let mut d = sub(b"DATA", &[0]);
        let mut xclc = x.to_le_bytes().to_vec();
        xclc.extend(y.to_le_bytes());
        xclc.extend([0u8; 4]);
        d.extend(sub(b"XCLC", &xclc));
        record(b"CELL", id, &d)
    };
    let mut world_children = {
        let mut d = sub(b"DATA", &[0x02]);
        d.extend(sub(b"XCLC", &[0u8; 12]));
        record_flagged(b"CELL", PERSISTENT_CELL, 0x400, &d)
    };
    world_children.extend(children(
        PERSISTENT_CELL,
        8,
        &named(
            ACTIVATOR_REF,
            "ReviseActivatorRef",
            ACTIVATOR,
            [100.0, 100.0, 0.0],
            0x400,
        ),
    ));
    let mut squares = exterior_cell(MAIL, -1, 0);
    squares.extend(children(
        MAIL,
        9,
        &named(MAILBOX_REF, "MailboxRef", MAILBOX, [-2000.0, 100.0, 0.0], 0),
    ));
    squares.extend(exterior_cell(HOME, 0, 0));
    squares.extend(exterior_cell(BORDER, 3, 0));
    world_children.extend(group([0; 4], 4, &group([0; 4], 5, &squares)));
    let mut world = sub(b"EDID", &zstr("ReviseWorld"));
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", WORLD, &world);
    worlds.extend(group(WORLD.to_le_bytes(), 1, &world_children));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"SCPT", 0, &scripts));
    plugin.extend(group(*b"MESG", 0, &messages));
    plugin.extend(group(*b"ACTI", 0, &activators));
    plugin.extend(group(*b"QUST", 0, &quests));
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}
