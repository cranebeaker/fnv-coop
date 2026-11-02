//! A world for quest targets (`world::quest_targets`): a worldspace whose
//! persistent cell holds load doors into a house and a shop; the house has
//! a door down into a basement; the shop has a front door and a locked back
//! door (with a key). Two quests: `QTQuest` (objective 10 targets the
//! basement's crate; objective 20 the shop's crate and, when stage 5 is
//! done, its till) and `QTOther` (objective 10 targets the house's chair).

use crate::{f32s, group, placed, record, sub, zstr, TempData};

pub mod ids {
    pub const DOOR: u32 = 0xB00;
    /// A door base with `FNAM` 0x08 (minimal use).
    pub const MINIMAL_DOOR: u32 = 0xB01;
    pub const CRATE: u32 = 0xB02;
    pub const KEY: u32 = 0xB03;
    pub const QUEST: u32 = 0xB10;
    pub const OTHER_QUEST: u32 = 0xB11;
    pub const WORLD: u32 = 0xC00;
    pub const PERSISTENT: u32 = 0xC01;
    /// Outside: to the house at (1000, 0, 0); the shop's front at
    /// (5000, 0, 0) and its locked back door at (200, 0, 0).
    pub const HOUSE_OUTSIDE: u32 = 0xC03;
    pub const SHOP_FRONT_OUTSIDE: u32 = 0xC05;
    pub const SHOP_BACK_OUTSIDE: u32 = 0xC06;
    pub const HOUSE: u32 = 0xD00;
    pub const HOUSE_DOOR: u32 = 0xD01;
    pub const HOUSE_TO_BASEMENT: u32 = 0xD02;
    pub const HOUSE_CHAIR: u32 = 0xD10;
    pub const SHOP: u32 = 0xE00;
    pub const SHOP_FRONT: u32 = 0xE01;
    pub const SHOP_BACK: u32 = 0xE02;
    pub const SHOP_CRATE: u32 = 0xE10;
    pub const SHOP_TILL: u32 = 0xE11;
    pub const BASEMENT: u32 = 0xF00;
    pub const BASEMENT_DOOR: u32 = 0xF01;
    pub const BASEMENT_CRATE: u32 = 0xF10;
}

/// An objective's index and its targets, each with its `CTDA` bytes.
type Objective<'a> = (i32, &'a [(u32, &'a [u8])]);

pub fn quest_targets(tag: &str) -> TempData {
    use ids::*;
    let data = TempData::new(tag);
    let edid = |s: &str| sub(b"EDID", &zstr(s));
    let named = |kind: &[u8; 4], id: u32, name: &str, rest: &[u8]| {
        let mut d = edid(name);
        d.extend(rest);
        record(kind, id, &d)
    };
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));

    let mut doors = named(b"DOOR", DOOR, "QTDoor", &sub(b"FNAM", &[0]));
    doors.extend(named(
        b"DOOR",
        MINIMAL_DOOR,
        "QTMinimalDoor",
        &sub(b"FNAM", &[0x08]),
    ));
    plugin.extend(group(*b"DOOR", 0, &doors));
    plugin.extend(group(*b"STAT", 0, &named(b"STAT", CRATE, "QTCrate", &[])));
    plugin.extend(group(*b"KEYM", 0, &named(b"KEYM", KEY, "QTKey", &[])));

    // Quests: objective, then its targets each with their conditions.
    let quest = |id: u32, name: &str, objectives: &[Objective]| {
        let mut d = sub(b"FULL", &zstr(name));
        let mut qdata = vec![0u8, 50, 0, 0];
        qdata.extend(1.0f32.to_le_bytes());
        d.extend(sub(b"DATA", &qdata));
        d.extend(sub(b"INDX", &5i16.to_le_bytes()));
        d.extend(sub(b"QSDT", &[0]));
        for (index, targets) in objectives {
            d.extend(sub(b"QOBJ", &index.to_le_bytes()));
            d.extend(sub(b"NNAM", &zstr(&format!("Objective {index}"))));
            for (target, conditions) in *targets {
                let mut qsta = target.to_le_bytes().to_vec();
                // Flags 0, then three bytes the game leaves unread.
                qsta.extend([0, 0x41, 0xE4, 0x1A]);
                d.extend(sub(b"QSTA", &qsta));
                d.extend(*conditions);
            }
        }
        named(b"QUST", id, name, &d)
    };
    // GetStageDone QTQuest 5 == 1.
    let stage_done = crate::condition(59, [QUEST, 5], 1.0);
    let mut quests = quest(
        QUEST,
        "QTQuest",
        &[
            (10, &[(BASEMENT_CRATE, &[])]),
            (20, &[(SHOP_CRATE, &[]), (SHOP_TILL, &stage_done)]),
        ],
    );
    quests.extend(quest(
        OTHER_QUEST,
        "QTOther",
        &[(10, &[(HOUSE_CHAIR, &[])])],
    ));
    plugin.extend(group(*b"QUST", 0, &quests));

    let load_door = |id: u32, base: u32, pos: [f32; 3], to: u32, arrive: [f32; 3], extra: &[u8]| {
        let mut xtel = to.to_le_bytes().to_vec();
        xtel.extend(f32s(&[arrive[0], arrive[1], arrive[2], 0.0, 0.0, 0.0]));
        xtel.extend(0u32.to_le_bytes());
        let mut more = sub(b"XTEL", &xtel);
        more.extend(extra);
        let mut r = placed(id, base, pos, [0.0; 3], &more);
        // Persistent, as the editor keeps load doors.
        r[8..12].copy_from_slice(&0x400u32.to_le_bytes());
        r
    };
    let crate_at = |id: u32, pos: [f32; 3]| placed(id, CRATE, pos, [0.0; 3], &[]);
    let mut xloc = vec![50u8, 0, 0, 0];
    xloc.extend(KEY.to_le_bytes());
    xloc.extend([0; 12]);

    // The worldspace: its persistent cell with the doors outside.
    let mut outside = load_door(
        HOUSE_OUTSIDE,
        DOOR,
        [1000.0, 0.0, 0.0],
        HOUSE_DOOR,
        [0.0, 100.0, 0.0],
        &[],
    );
    outside.extend(load_door(
        SHOP_FRONT_OUTSIDE,
        DOOR,
        [5000.0, 0.0, 0.0],
        SHOP_FRONT,
        [0.0, 100.0, 0.0],
        &[],
    ));
    outside.extend(load_door(
        SHOP_BACK_OUTSIDE,
        DOOR,
        [200.0, 0.0, 0.0],
        SHOP_BACK,
        [0.0, -100.0, 0.0],
        &sub(b"XLOC", &xloc),
    ));
    let mut persistent_cell = sub(b"DATA", &[0x02]);
    persistent_cell.extend(sub(b"XCLC", &[0u8; 12]));
    let mut persistent = record(b"CELL", PERSISTENT, &persistent_cell);
    persistent[8..12].copy_from_slice(&0x400u32.to_le_bytes());
    let mut world_children = persistent;
    world_children.extend(group(
        PERSISTENT.to_le_bytes(),
        6,
        &group(PERSISTENT.to_le_bytes(), 8, &outside),
    ));
    let mut world = edid("QTWorld");
    world.extend(sub(b"DATA", &[0]));
    let mut worlds = record(b"WRLD", WORLD, &world);
    worlds.extend(group(WORLD.to_le_bytes(), 1, &world_children));

    // The interiors.
    let cell = |id: u32, name: &str, refs: &[u8]| {
        let mut c = named(b"CELL", id, name, &sub(b"DATA", &[1]));
        c.extend(group(
            id.to_le_bytes(),
            6,
            &group(id.to_le_bytes(), 8, refs),
        ));
        c
    };
    let mut house = load_door(
        HOUSE_DOOR,
        DOOR,
        [0.0, 0.0, 0.0],
        HOUSE_OUTSIDE,
        [1000.0, -100.0, 0.0],
        &[],
    );
    house.extend(load_door(
        HOUSE_TO_BASEMENT,
        DOOR,
        [300.0, 0.0, 0.0],
        BASEMENT_DOOR,
        [0.0, 0.0, 0.0],
        &[],
    ));
    house.extend(crate_at(HOUSE_CHAIR, [100.0, 100.0, 0.0]));
    let mut shop = load_door(
        SHOP_FRONT,
        DOOR,
        [0.0, 0.0, 0.0],
        SHOP_FRONT_OUTSIDE,
        [5000.0, -100.0, 0.0],
        &[],
    );
    shop.extend(load_door(
        SHOP_BACK,
        DOOR,
        [0.0, -200.0, 0.0],
        SHOP_BACK_OUTSIDE,
        [200.0, 100.0, 0.0],
        &[],
    ));
    shop.extend(crate_at(SHOP_CRATE, [50.0, 50.0, 0.0]));
    shop.extend(crate_at(SHOP_TILL, [-50.0, 50.0, 0.0]));
    let mut basement = load_door(
        BASEMENT_DOOR,
        DOOR,
        [0.0, 0.0, 0.0],
        HOUSE_TO_BASEMENT,
        [300.0, -100.0, 0.0],
        &[],
    );
    basement.extend(crate_at(BASEMENT_CRATE, [50.0, 50.0, 0.0]));
    let mut interiors = cell(HOUSE, "QTHouse", &house);
    interiors.extend(cell(SHOP, "QTShop", &shop));
    interiors.extend(cell(BASEMENT, "QTBasement", &basement));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &interiors)),
    ));
    plugin.extend(group(*b"WRLD", 0, &worlds));
    data.write("FalloutNV.esm", &plugin);
    data
}
