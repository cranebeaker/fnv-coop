//! A world for package types and actions (`world::ai::flee`,
//! `world::ai::guard`, `world::ai::data`, `world::ai::actions`): an
//! interior with a guard, a threat, markers and one package of each case
//! the Goodsprings gunfight uses, plus use-weapon, ambush and patrol data
//! and a travel package with begin and end actions.

use crate::{group, placed, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const PLAYER: u32 = 0x7;
    pub const X_MARKER_HEADING: u32 = 0x34;
    pub const X_MARKER: u32 = 0x3B;
    /// A short global the begin action's script sets to 5.
    pub const GLOBAL: u32 = 0x2100;
    pub const IDLE: u32 = 0x2101;
    pub const TOPIC: u32 = 0x2102;
    pub const WEAPON: u32 = 0x2103;
    // Packages.
    /// Type 10, no location, no target (`GoodspringsFleePackage`), flags
    /// 0x0C203002.
    pub const FLEE_NOWHERE: u32 = 0x2110;
    /// Type 10, "in a cell" with radius 300 on disk
    /// (`GSSettlerAAMHidePackage`).
    pub const FLEE_IN_CELL: u32 = 0x2111;
    /// Type 10, to `FLEE_MARKER_REF` within 256, from `THREAT_REF` (1000).
    pub const FLEE_TO_MARKER: u32 = 0x2112;
    /// Type 10, near the linked reference.
    pub const FLEE_TO_LINKED: u32 = 0x2113;
    /// Type 14, near the editor location, target `POST_REF` with value 0
    /// (`GSSettlerCFGunfightPackage`).
    pub const GUARD_EDITOR: u32 = 0x2114;
    /// Type 14, near `POST_REF`, target `POST_REF`
    /// (`GSSettlerAAMGunfightPackage`).
    pub const GUARD_POST: u32 = 0x2115;
    /// Type 14, no location, target `POST_REF` with value 200.
    pub const GUARD_AROUND: u32 = 0x2116;
    /// Type 14, no location, target the linked reference.
    pub const GUARD_LINKED: u32 = 0x2117;
    /// Type 16: `PKW3`, `PTD2` (`THREAT_REF`), `PLD2` (`FLEE_MARKER_REF`).
    pub const USE_WEAPON: u32 = 0x2118;
    /// Type 9: editor location, `PLD2` editor location 512, `PKAM`, the
    /// `PLD2` again (`GSSettlerAmbushPackage`).
    pub const AMBUSH: u32 = 0x2119;
    /// Type 13, `PKPT` repeatable.
    pub const PATROL: u32 = 0x211A;
    /// Type 6 to `POST_REF`; begins setting `GLOBAL` to 5 with `IDLE` and
    /// `TOPIC`; ends with `RemoveScriptPackage` (`GSJoeCobbLeavePackage`).
    pub const WITH_ACTIONS: u32 = 0x211B;
    // People and places.
    pub const GUARD: u32 = 0x2120;
    pub const THREAT: u32 = 0x2121;
    pub const CELL: u32 = 0x2130;
    /// The cell's navmesh, a square 2000 across.
    pub const NAVMESH: u32 = 0x2136;
    /// At (100, 100), heading 0, linked to `LINKED_REF`.
    pub const GUARD_REF: u32 = 0x2131;
    /// At (300, 0).
    pub const THREAT_REF: u32 = 0x2132;
    /// An `XMarkerHeading` at (800, 800) turned 90°.
    pub const POST_REF: u32 = 0x2133;
    /// An `XMarker` at (-800, -800).
    pub const FLEE_MARKER_REF: u32 = 0x2134;
    /// An `XMarker` at (0, 600).
    pub const LINKED_REF: u32 = 0x2135;
}

/// The world (see the module notes).
pub fn world(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    let mut global = edid("TestPackGlobal");
    global.extend(sub(b"FNAM", b"s"));
    global.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));
    let mut topic = edid("TestPackTopic");
    topic.extend(sub(b"DATA", &[0, 0]));

    let pkdt = |flags: u32, kind: u8| {
        let mut d = flags.to_le_bytes().to_vec();
        d.extend([kind, 0, 0, 0, 0, 0, 0, 0]);
        sub(b"PKDT", &d)
    };
    let loc = |sig: &[u8; 4], kind: u32, form: u32, radius: u32| {
        let mut d = kind.to_le_bytes().to_vec();
        d.extend(form.to_le_bytes());
        d.extend(radius.to_le_bytes());
        sub(sig, &d)
    };
    let target = |sig: &[u8; 4], kind: u32, form: u32, value: u32| {
        let mut d = kind.to_le_bytes().to_vec();
        d.extend(form.to_le_bytes());
        d.extend(value.to_le_bytes());
        d.extend([0; 4]);
        sub(sig, &d)
    };
    let any_time = sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]);
    // An action: marker, idle, script header and source, topic.
    let action = |marker: &[u8; 4], idle: u32, source: &str, topic: u32| {
        let mut d = sub(marker, &[]);
        d.extend(sub(b"INAM", &idle.to_le_bytes()));
        let mut schr = vec![0u8; 20];
        schr[18] = 1;
        d.extend(sub(b"SCHR", &schr));
        if !source.is_empty() {
            d.extend(sub(b"SCTX", source.as_bytes()));
        }
        d.extend(sub(b"TNAM", &topic.to_le_bytes()));
        d
    };
    let package = |id: u32, name: &str, parts: &[Vec<u8>]| {
        let mut d = edid(name);
        for p in parts {
            d.extend(p);
        }
        record(b"PACK", id, &d)
    };
    let mut pkw3 = vec![1, 0, 1, 0, 1, 0];
    pkw3.extend(3u16.to_le_bytes());
    pkw3.extend(2u16.to_le_bytes());
    pkw3.extend(4u16.to_le_bytes());
    pkw3.extend(0.5f32.to_le_bytes());
    pkw3.extend(1.5f32.to_le_bytes());
    pkw3.extend(WEAPON.to_le_bytes());
    let mut packages = Vec::new();
    for (id, name, parts) in [
        (
            FLEE_NOWHERE,
            "TestFleeNowhere",
            vec![pkdt(0x0C20_3002, 10), any_time.clone()],
        ),
        (
            FLEE_IN_CELL,
            "TestFleeInCell",
            vec![pkdt(0, 10), loc(b"PLDT", 1, CELL, 300), any_time.clone()],
        ),
        (
            FLEE_TO_MARKER,
            "TestFleeToMarker",
            vec![
                pkdt(0, 10),
                loc(b"PLDT", 0, FLEE_MARKER_REF, 256),
                any_time.clone(),
                target(b"PTDT", 0, THREAT_REF, 1000),
            ],
        ),
        (
            FLEE_TO_LINKED,
            "TestFleeToLinked",
            vec![pkdt(0, 10), loc(b"PLDT", 6, 0, 0), any_time.clone()],
        ),
        (
            GUARD_EDITOR,
            "TestGuardEditor",
            vec![
                pkdt(0x1C80_3202, 14),
                loc(b"PLDT", 3, 0, 0),
                any_time.clone(),
                target(b"PTDT", 0, POST_REF, 0),
            ],
        ),
        (
            GUARD_POST,
            "TestGuardPost",
            vec![
                pkdt(0x1C80_3202, 14),
                loc(b"PLDT", 0, POST_REF, 0),
                any_time.clone(),
                target(b"PTDT", 0, POST_REF, 0),
            ],
        ),
        (
            GUARD_AROUND,
            "TestGuardAround",
            vec![
                pkdt(0, 14),
                any_time.clone(),
                target(b"PTDT", 0, POST_REF, 200),
            ],
        ),
        (
            GUARD_LINKED,
            "TestGuardLinked",
            vec![pkdt(0, 14), any_time.clone(), target(b"PTDT", 3, 0, 0)],
        ),
        (
            USE_WEAPON,
            "TestUseWeapon",
            vec![
                pkdt(0, 16),
                loc(b"PLDT", 3, 0, 0),
                any_time.clone(),
                sub(b"PKW3", &pkw3),
                loc(b"PLD2", 0, FLEE_MARKER_REF, 64),
                target(b"PTD2", 0, THREAT_REF, 5),
            ],
        ),
        (
            AMBUSH,
            "TestAmbush",
            vec![
                pkdt(0x1080_1200, 9),
                loc(b"PLDT", 3, 0, 0),
                loc(b"PLD2", 3, 0, 512),
                any_time.clone(),
                sub(b"PKAM", &[]),
                loc(b"PLD2", 3, 0, 512),
            ],
        ),
        (
            PATROL,
            "TestPatrol",
            vec![pkdt(0, 13), any_time.clone(), sub(b"PKPT", &[1, 0])],
        ),
        (
            WITH_ACTIONS,
            "TestWithActions",
            vec![
                pkdt(0, 6),
                loc(b"PLDT", 0, POST_REF, 0),
                any_time.clone(),
                action(b"POBA", IDLE, "set TestPackGlobal to 5", TOPIC),
                action(b"POEA", 0, "RemoveScriptPackage", 0),
                action(b"POCA", 0, "", 0),
            ],
        ),
    ] {
        packages.extend(package(id, name, &parts));
    }

    let npc = |id: u32, name: &str| {
        let mut d = edid(name);
        d.extend(sub(b"ACBS", &[0; 24]));
        record(b"NPC_", id, &d)
    };
    let mut player = edid("Player");
    player.extend(sub(b"ACBS", &[0; 24]));
    let mut npcs = record(b"NPC_", PLAYER, &player);
    npcs.extend(npc(GUARD, "TestPackGuard"));
    npcs.extend(npc(THREAT, "TestPackThreat"));

    let mut statics = record(b"STAT", X_MARKER_HEADING, &edid("XMarkerHeading"));
    statics.extend(record(b"STAT", X_MARKER, &edid("XMarker")));

    let actor = |id: u32, base: u32, pos: [f32; 3], extra: &[u8]| {
        let mut r = placed(id, base, pos, [0.0; 3], extra);
        r[..4].copy_from_slice(b"ACHR");
        r
    };
    // A navmesh square (-1000, -1000)–(1000, 1000), two triangles.
    let mut navmesh = sub(b"NVER", &11u32.to_le_bytes());
    navmesh.extend(sub(
        b"NVVX",
        &crate::f32s(&[
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
    let mut guard_extra = edid("TestPackGuardRef");
    guard_extra.extend(sub(b"XLKR", &LINKED_REF.to_le_bytes()));
    let mut refs = actor(GUARD_REF, GUARD, [100.0, 100.0, 0.0], &guard_extra);
    refs.extend(record(b"NAVM", NAVMESH, &navmesh));
    refs.extend(actor(THREAT_REF, THREAT, [300.0, 0.0, 0.0], &[]));
    refs.extend(placed(
        POST_REF,
        X_MARKER_HEADING,
        [800.0, 800.0, 0.0],
        [0.0, 0.0, 90.0],
        &[],
    ));
    refs.extend(placed(
        FLEE_MARKER_REF,
        X_MARKER,
        [-800.0, -800.0, 0.0],
        [0.0; 3],
        &[],
    ));
    refs.extend(placed(
        LINKED_REF,
        X_MARKER,
        [0.0, 600.0, 0.0],
        [0.0; 3],
        &[],
    ));
    let mut cell = edid("TestPackCell");
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
    plugin.extend(group(*b"GLOB", 0, &record(b"GLOB", GLOBAL, &global)));
    plugin.extend(group(*b"STAT", 0, &statics));
    plugin.extend(group(
        *b"IDLE",
        0,
        &record(b"IDLE", IDLE, &edid("TestPackIdle")),
    ));
    plugin.extend(group(*b"DIAL", 0, &record(b"DIAL", TOPIC, &topic)));
    plugin.extend(group(
        *b"WEAP",
        0,
        &record(b"WEAP", WEAPON, &edid("TestPackWeapon")),
    ));
    plugin.extend(group(*b"PACK", 0, &packages));
    plugin.extend(group(*b"NPC_", 0, &npcs));
    plugin.extend(cells);
    data.write("FalloutNV.esm", &plugin);
    data
}
