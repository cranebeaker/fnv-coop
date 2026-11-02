//! A world for the long way (`world::ai::navinfo`): an outdoor worldspace
//! `LongWorld` with five squares in a row, (0,0) to (4,0), each holding one
//! navmesh covering it (two triangles, joined to its neighbours through
//! their external connections), a `NAVI` record with each navmesh's entry
//! (`NVMI`) and connections (`NVCI`), a walker in square 0 whose travel
//! package leads to a marker in square 4, and a second, empty `NAVI`
//! version in `DeadMoney.esm` (as the DLCs override `FalloutNV.esm`'s).

use crate::{f32s, group, placed, record, sub, zstr, TempData};

/// The forms, by the names the tests use.
pub mod ids {
    pub const X_MARKER: u32 = 0x3B;
    pub const WORLD: u32 = 0x5000;
    pub const NAVI: u32 = 0x5001;
    /// Squares (i, 0), i = 0..5: cells `CELL + i`, navmeshes
    /// `NAVMESH + i`.
    pub const CELL: u32 = 0x5010;
    pub const NAVMESH: u32 = 0x5020;
    pub const WALKER: u32 = 0x5030;
    /// Type 6 to `MARKER_REF`; its end action sets `DONE_GLOBAL` to 1.
    pub const TRAVEL: u32 = 0x5031;
    pub const DONE_GLOBAL: u32 = 0x5032;
    /// In square 0 at (1000, 1000, 0).
    pub const WALKER_REF: u32 = 0x5040;
    /// An `XMarker` in square 4 at (19000, 3000, 0).
    pub const MARKER_REF: u32 = 0x5041;
    /// The squares in the row.
    pub const SQUARES: i32 = 5;
}

/// One square's navmesh: the square as two counterclockwise triangles
/// sharing the diagonal; triangle 0's east edge and triangle 1's west edge
/// lead to the neighbours (`NVEX`).
fn navmesh(i: i32) -> Vec<u8> {
    use ids::*;
    let (x0, x1) = (i as f32 * 4096.0, (i + 1) as f32 * 4096.0);
    let mut d = sub(b"NVER", &11u32.to_le_bytes());
    d.extend(sub(
        b"NVVX",
        &f32s(&[x0, 0.0, 0.0, x1, 0.0, 0.0, x1, 4096.0, 0.0, x0, 4096.0, 0.0]),
    ));
    let tri = |v: [u16; 3], n: [u16; 3], flags: u16| {
        let mut t = Vec::new();
        for x in v.into_iter().chain(n) {
            t.extend(x.to_le_bytes());
        }
        t.extend(flags.to_le_bytes());
        t.extend([0; 2]);
        t
    };
    let mut externals = Vec::new();
    let mut east_link = 0xFFFF;
    let mut west_link = 0xFFFF;
    if i + 1 < SQUARES {
        east_link = (externals.len() / 10) as u16;
        externals.extend([0; 4]);
        externals.extend((NAVMESH + i as u32 + 1).to_le_bytes());
        externals.extend(1u16.to_le_bytes());
    }
    if i > 0 {
        west_link = (externals.len() / 10) as u16;
        externals.extend([0; 4]);
        externals.extend((NAVMESH + i as u32 - 1).to_le_bytes());
        externals.extend(0u16.to_le_bytes());
    }
    let mut triangles = tri(
        [0, 1, 2],
        [0xFFFF, east_link, 1],
        if east_link == 0xFFFF { 0 } else { 0x2 },
    );
    triangles.extend(tri(
        [0, 2, 3],
        [0, 0xFFFF, west_link],
        if west_link == 0xFFFF { 0 } else { 0x4 },
    ));
    d.extend(sub(b"NVTR", &triangles));
    if !externals.is_empty() {
        d.extend(sub(b"NVEX", &externals));
    }
    record(b"NAVM", NAVMESH + i as u32, &d)
}

/// The navmesh info map: each square's entry at its middle, joined to its
/// neighbours (first list), NVER 11.
fn navi() -> Vec<u8> {
    use ids::*;
    let mut d = sub(b"NVER", &11u32.to_le_bytes());
    for i in 0..SQUARES {
        let mut nvmi = 0u32.to_le_bytes().to_vec();
        nvmi.extend((NAVMESH + i as u32).to_le_bytes());
        nvmi.extend(WORLD.to_le_bytes());
        // The grid word: y, then x.
        nvmi.extend(0i16.to_le_bytes());
        nvmi.extend((i as i16).to_le_bytes());
        nvmi.extend(f32s(&[i as f32 * 4096.0 + 2048.0, 2048.0, 0.0]));
        nvmi.extend(0.0f32.to_le_bytes());
        d.extend(sub(b"NVMI", &nvmi));
    }
    for i in 0..SQUARES {
        let mut nvci = (NAVMESH + i as u32).to_le_bytes().to_vec();
        let mut links = Vec::new();
        if i > 0 {
            links.push(NAVMESH + i as u32 - 1);
        }
        if i + 1 < SQUARES {
            links.push(NAVMESH + i as u32 + 1);
        }
        nvci.extend((links.len() as u32).to_le_bytes());
        for l in links {
            nvci.extend(l.to_le_bytes());
        }
        nvci.extend(0u32.to_le_bytes());
        nvci.extend(0u32.to_le_bytes());
        d.extend(sub(b"NVCI", &nvci));
    }
    record(b"NAVI", NAVI, &d)
}

/// The world, written as `FalloutNV.esm` (and `DeadMoney.esm`, whose
/// `NAVI` version holds no entries) into a temporary Data folder.
pub fn long_paths(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));

    let mut global = edid("TestLongDone");
    global.extend(sub(b"FNAM", b"s"));
    global.extend(sub(b"FLTV", &0.0f32.to_le_bytes()));

    let mut pkdt = 0u32.to_le_bytes().to_vec();
    pkdt.extend([6, 0, 0, 0, 0, 0, 0, 0]);
    let mut pldt = 0u32.to_le_bytes().to_vec();
    pldt.extend(MARKER_REF.to_le_bytes());
    pldt.extend(0u32.to_le_bytes());
    let mut package = edid("TestLongTravel");
    package.extend(sub(b"PKDT", &pkdt));
    package.extend(sub(b"PLDT", &pldt));
    package.extend(sub(b"PSDT", &[0xFF, 0xFF, 0, 0xFF, 0, 0, 0, 0]));
    package.extend(sub(b"POEA", &[]));
    package.extend(sub(b"INAM", &0u32.to_le_bytes()));
    let mut schr = vec![0u8; 20];
    schr[18] = 1;
    package.extend(sub(b"SCHR", &schr));
    package.extend(sub(b"SCTX", b"set TestLongDone to 1"));
    package.extend(sub(b"TNAM", &0u32.to_le_bytes()));

    let mut walker = edid("TestLongWalker");
    walker.extend(sub(b"ACBS", &[0; 24]));
    walker.extend(sub(b"PKID", &TRAVEL.to_le_bytes()));

    let children = |cell: u32, contents: &[u8]| {
        group(
            cell.to_le_bytes(),
            6,
            &group(cell.to_le_bytes(), 9, contents),
        )
    };
    let mut squares = Vec::new();
    for i in 0..SQUARES {
        let cell = CELL + i as u32;
        let mut d = sub(b"DATA", &[0]);
        let mut xclc = i.to_le_bytes().to_vec();
        xclc.extend(0i32.to_le_bytes());
        xclc.extend([0u8; 4]);
        d.extend(sub(b"XCLC", &xclc));
        squares.extend(record(b"CELL", cell, &d));
        let mut refs = navmesh(i);
        if i == 0 {
            let mut r = placed(WALKER_REF, WALKER, [1000.0, 1000.0, 0.0], [0.0; 3], &[]);
            r[..4].copy_from_slice(b"ACHR");
            refs.extend(r);
        }
        if i == SQUARES - 1 {
            refs.extend(placed(
                MARKER_REF,
                X_MARKER,
                [19000.0, 3000.0, 0.0],
                [0.0; 3],
                &[],
            ));
        }
        squares.extend(children(cell, &refs));
    }
    let world_children = group([0; 4], 4, &group([0; 4], 5, &squares));
    let mut world = edid("LongWorld");
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", WORLD, &world);
    worlds.extend(group(WORLD.to_le_bytes(), 1, &world_children));

    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"GLOB", 0, &record(b"GLOB", DONE_GLOBAL, &global)));
    plugin.extend(group(
        *b"STAT",
        0,
        &record(b"STAT", X_MARKER, &edid("XMarker")),
    ));
    plugin.extend(group(*b"PACK", 0, &record(b"PACK", TRAVEL, &package)));
    plugin.extend(group(*b"NPC_", 0, &record(b"NPC_", WALKER, &walker)));
    plugin.extend(group(*b"NAVI", 0, &navi()));
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);

    // A DLC overriding the map record with its own (here empty) version.
    let mut tes4 = sub(b"HEDR", &hedr);
    tes4.extend(sub(b"MAST", &zstr("FalloutNV.esm")));
    tes4.extend(sub(b"DATA", &[0; 8]));
    let mut dlc = record(b"TES4", 0, &tes4);
    dlc.extend(group(
        *b"NAVI",
        0,
        &record(b"NAVI", NAVI, &sub(b"NVER", &11u32.to_le_bytes())),
    ));
    data.write("DeadMoney.esm", &dlc);
    data
}
