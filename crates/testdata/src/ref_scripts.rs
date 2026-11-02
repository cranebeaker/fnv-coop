//! A world for the reference-script pass (`world::ref_scripts`): an outdoor
//! worldspace `ScriptWorld` with squares 0,0, 1,0 and 3,0 and a persistent
//! object standing in 1,0, people in 0,1 (one following an enable parent, a
//! persistent disabled one), and the interior `ScriptRoom`. Counters count
//! their `OnLoad` and `GameMode` runs; a trigger counts its events; a mover
//! calls `Activate` (a command that stops the pass) every frame; a
//! start-game quest's script does something once, as the DLC start
//! scripts do.

use crate::{f32s, group, placed, record, record_flagged, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const WORLD: u32 = 0x2000;
    pub const PERSISTENT_CELL: u32 = 0x2001;
    /// Square 0,0.
    pub const FIELD: u32 = 0x2010;
    /// Square 1,0.
    pub const EAST: u32 = 0x2020;
    /// Square 3,0.
    pub const FAR: u32 = 0x2030;
    pub const ROOM: u32 = 0x2040;
    /// `iLoads` (`OnLoad`) and `iFrames` (`GameMode`).
    pub const COUNTER_SCRIPT: u32 = 0x2100;
    /// `iEnter`, `iInside`, `iLeave` for the player; `iAnyEnter` for
    /// anyone; `iFrames`.
    pub const TRIGGER_SCRIPT: u32 = 0x2101;
    /// `iFrames`, then `Activate`.
    pub const MOVER_SCRIPT: u32 = 0x2102;
    pub const COUNTER: u32 = 0x2200;
    pub const TRIGGER: u32 = 0x2201;
    pub const MOVER: u32 = 0x2202;
    /// In 0,0 at 100,100.
    pub const COUNTER_REF: u32 = 0x2300;
    /// In 0,0 at 1000,1000: a box 100 each way.
    pub const TRIGGER_REF: u32 = 0x2301;
    /// In 0,0, initially disabled.
    pub const DISABLED_REF: u32 = 0x2302;
    /// In 1,0 at 4200,100.
    pub const MOVER_REF: u32 = 0x2303;
    /// In 3,0.
    pub const FAR_REF: u32 = 0x2304;
    /// Persistent, standing in 1,0 at 5000,100.
    pub const PERSISTENT_REF: u32 = 0x2305;
    /// In `ScriptRoom`.
    pub const ROOM_REF: u32 = 0x2306;
    /// Square 0,1, holding people.
    pub const CAMP: u32 = 0x2050;
    /// A person (`NPC_`) with no script.
    pub const SETTLER: u32 = 0x2203;
    /// In 0,1: follows `DISABLED_REF`'s enable state (`XESP`).
    pub const FOLLOWER_REF: u32 = 0x2307;
    /// In 0,1, enabled.
    pub const AWAKE_REF: u32 = 0x2308;
    /// Persistent and initially disabled, standing in 0,1 at 100,4200.
    pub const LONE_REF: u32 = 0x2309;
    /// A DLC start script's shape (`NVDLC03MQ00SCRIPT`): `nEnableDLC` 0 →
    /// 1 with `fStartTimer` 5, then once it runs out `iShown` + 1 and
    /// `nEnableDLC` 2.
    pub const ONCE_QUEST_SCRIPT: u32 = 0x2103;
    /// Start game enabled, with `ONCE_QUEST_SCRIPT`.
    pub const ONCE_QUEST: u32 = 0x2400;
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

/// The world, written as `FalloutNV.esm` into a temporary Data folder.
pub fn ref_scripts(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let mut scripts = script(
        COUNTER_SCRIPT,
        "CounterScript",
        "scn CounterScript\nshort iLoads\nshort iFrames\n\
         Begin OnLoad\n\tset iLoads to iLoads + 1\nEnd\n\
         Begin GameMode\n\tset iFrames to iFrames + 1\nEnd",
    );
    scripts.extend(script(
        TRIGGER_SCRIPT,
        "TriggerScript",
        "scn TriggerScript\nshort iEnter\nshort iInside\nshort iLeave\n\
         short iAnyEnter\nshort iFrames\n\
         Begin OnTriggerEnter player\n\tset iEnter to iEnter + 1\nEnd\n\
         Begin OnTrigger player\n\tset iInside to iInside + 1\nEnd\n\
         Begin OnTriggerLeave player\n\tset iLeave to iLeave + 1\nEnd\n\
         Begin OnTriggerEnter\n\tset iAnyEnter to iAnyEnter + 1\nEnd\n\
         Begin GameMode\n\tset iFrames to iFrames + 1\nEnd",
    ));
    scripts.extend(script(
        MOVER_SCRIPT,
        "MoverScript",
        "scn MoverScript\nshort iFrames\n\
         Begin GameMode\n\tset iFrames to iFrames + 1\n\tActivate\nEnd",
    ));
    scripts.extend(script(
        ONCE_QUEST_SCRIPT,
        "OnceQuestScript",
        "scn OnceQuestScript\nshort nEnableDLC\nshort iShown\nfloat fStartTimer\n\
         Begin GameMode\n\
         \tif nEnableDLC == 0\n\t\tset nEnableDLC to 1\n\t\tset fStartTimer to 5\n\
         \telseif nEnableDLC == 1\n\
         \t\tif fStartTimer <= 0\n\t\t\tset iShown to iShown + 1\n\t\t\tset nEnableDLC to 2\n\
         \t\telse\n\t\t\tset fStartTimer to fStartTimer - GetSecondsPassed\n\t\tendif\n\
         \tendif\nEnd",
    ));
    let quests = {
        let mut d = sub(b"EDID", &zstr("OnceQuest"));
        d.extend(sub(b"SCRI", &ONCE_QUEST_SCRIPT.to_le_bytes()));
        // Start game enabled (0x01), priority 0, no delay of its own.
        d.extend(sub(b"DATA", &[0x01, 0, 0, 0, 0, 0, 0, 0]));
        record(b"QUST", ONCE_QUEST, &d)
    };
    let mut activators = activator(COUNTER, "Counter", COUNTER_SCRIPT);
    activators.extend(activator(TRIGGER, "Trigger", TRIGGER_SCRIPT));
    activators.extend(activator(MOVER, "Mover", MOVER_SCRIPT));
    let people = record(b"NPC_", SETTLER, &sub(b"EDID", &zstr("Settler")));

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
    let mut xprm = f32s(&[100.0, 100.0, 100.0, 0.5, 0.5, 0.5, 0.0]);
    xprm.extend(1u32.to_le_bytes());

    let mut world_children = {
        let mut d = sub(b"DATA", &[0x02]);
        d.extend(sub(b"XCLC", &[0u8; 12]));
        record_flagged(b"CELL", PERSISTENT_CELL, 0x400, &d)
    };
    let actor = |id: u32, pos: [f32; 3], extra: &[u8]| {
        let mut r = placed(id, SETTLER, pos, [0.0; 3], extra);
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    let mut persistent = placed(PERSISTENT_REF, COUNTER, [5000.0, 100.0, 0.0], [0.0; 3], &[]);
    persistent[8..12].copy_from_slice(&0x400u32.to_le_bytes());
    let mut lone = actor(LONE_REF, [100.0, 4200.0, 0.0], &[]);
    lone[8..12].copy_from_slice(&0xC00u32.to_le_bytes());
    persistent.extend(lone);
    world_children.extend(children(PERSISTENT_CELL, 8, &persistent));

    let mut squares = exterior_cell(FIELD, 0, 0);
    let mut field = placed(COUNTER_REF, COUNTER, [100.0, 100.0, 0.0], [0.0; 3], &[]);
    field.extend(placed(
        TRIGGER_REF,
        TRIGGER,
        [1000.0, 1000.0, 0.0],
        [0.0; 3],
        &sub(b"XPRM", &xprm),
    ));
    let mut disabled = placed(DISABLED_REF, COUNTER, [200.0, 100.0, 0.0], [0.0; 3], &[]);
    disabled[8..12].copy_from_slice(&0x800u32.to_le_bytes());
    field.extend(disabled);
    squares.extend(children(FIELD, 9, &field));
    squares.extend(exterior_cell(EAST, 1, 0));
    squares.extend(children(
        EAST,
        9,
        &placed(MOVER_REF, MOVER, [4200.0, 100.0, 0.0], [0.0; 3], &[]),
    ));
    squares.extend(exterior_cell(CAMP, 0, 1));
    let mut xesp = DISABLED_REF.to_le_bytes().to_vec();
    xesp.extend([0, 0, 0, 0]);
    let mut camp = actor(FOLLOWER_REF, [200.0, 4200.0, 0.0], &sub(b"XESP", &xesp));
    camp.extend(actor(AWAKE_REF, [300.0, 4200.0, 0.0], &[]));
    squares.extend(children(CAMP, 9, &camp));
    squares.extend(exterior_cell(FAR, 3, 0));
    squares.extend(children(
        FAR,
        9,
        &placed(FAR_REF, COUNTER, [12500.0, 100.0, 0.0], [0.0; 3], &[]),
    ));
    world_children.extend(group([0; 4], 4, &group([0; 4], 5, &squares)));
    let mut world = sub(b"EDID", &zstr("ScriptWorld"));
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", WORLD, &world);
    worlds.extend(group(WORLD.to_le_bytes(), 1, &world_children));

    let mut room = sub(b"EDID", &zstr("ScriptRoom"));
    room.extend(sub(b"DATA", &[1]));
    let mut interiors = record(b"CELL", ROOM, &room);
    interiors.extend(children(
        ROOM,
        9,
        &placed(ROOM_REF, COUNTER, [0.0; 3], [0.0; 3], &[]),
    ));
    let interiors = group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &interiors)),
    );

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"SCPT", 0, &scripts));
    plugin.extend(group(*b"ACTI", 0, &activators));
    plugin.extend(group(*b"NPC_", 0, &people));
    plugin.extend(group(*b"QUST", 0, &quests));
    plugin.extend(interiors);
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}
