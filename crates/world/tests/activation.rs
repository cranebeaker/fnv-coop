//! What the crosshair offers and what the Info panel says
//! (`world::activation`, `00579280`, `00775a00`), on a generated plugin.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{f32s, group, record, sub, zstr};
use world::activation::{self, class};
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

const CHAIR: u32 = 0x800;
const BED: u32 = 0x801;
const CHEST: u32 = 0x802;
const LOCKER: u32 = 0x803;
const CAN: u32 = 0x804;
const RIFLE: u32 = 0x805;
const TESTER: u32 = 0x806;
const NAMELESS: u32 = 0x807;
const SUNNY: u32 = 0x808;
const GECKO: u32 = 0x809;
const TALKER: u32 = 0x80A;
const DOOR: u32 = 0x80B;
const BOOK: u32 = 0x80C;
const LAMP: u32 = 0x80D;
const KEY: u32 = 0x80E;
const WALL: u32 = 0x80F;
const SCONCE: u32 = 0x810;

const ROOM: u32 = 0x900;
const TOWN: u32 = 0x901;

const CHAIR_REF: u32 = 0x910;
const BED_REF: u32 = 0x911;
const CHEST_REF: u32 = 0x912;
const LOCKED_REF: u32 = 0x913;
const CANS_REF: u32 = 0x914;
const RIFLE_REF: u32 = 0x915;
const TESTER_REF: u32 = 0x916;
const NAMELESS_REF: u32 = 0x917;
const SUNNY_REF: u32 = 0x918;
const GECKO_REF: u32 = 0x919;
const TALKER_REF: u32 = 0x91A;
const DOOR_REF: u32 = 0x91B;
const FAR_DOOR_REF: u32 = 0x91C;
const BOOK_REF: u32 = 0x91D;
const LAMP_REF: u32 = 0x91E;
const KEYED_REF: u32 = 0x91F;
const WALL_REF: u32 = 0x920;
const SCONCE_REF: u32 = 0x921;

fn named(kind: &[u8; 4], id: u32, name: Option<&str>, rest: &[u8]) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(&format!("Test{id:X}")));
    if let Some(n) = name {
        d.extend(sub(b"FULL", &zstr(n)));
    }
    d.extend(rest);
    record(kind, id, &d)
}

fn placed(kind: &[u8; 4], id: u32, base: u32, extra: &[u8]) -> Vec<u8> {
    let mut d = sub(b"NAME", &base.to_le_bytes());
    d.extend(extra);
    d.extend(sub(b"DATA", &f32s(&[0.0; 6])));
    record(kind, id, &d)
}

fn cell(id: u32, name: &str, refs: &[u8]) -> Vec<u8> {
    let mut c = sub(b"EDID", &zstr(&format!("Cell{id:X}")));
    c.extend(sub(b"FULL", &zstr(name)));
    c.extend(sub(b"DATA", &[1]));
    let mut out = record(b"CELL", id, &c);
    out.extend(group(
        id.to_le_bytes(),
        6,
        &group(id.to_le_bytes(), 9, refs),
    ));
    out
}

fn xloc(level: u8, key: u32) -> Vec<u8> {
    let mut d = vec![level, 0, 0, 0];
    d.extend(key.to_le_bytes());
    d.extend([0u8; 12]);
    sub(b"XLOC", &d)
}

fn world() -> (testdata::TempData, LoadOrder) {
    let data = testdata::TempData::empty("activation");
    let mut plugin = record(
        b"TES4",
        0,
        &sub(b"HEDR", &{
            let mut h = 1.34f32.to_le_bytes().to_vec();
            h.extend([0; 8]);
            h
        }),
    );
    let acbs = |flags: u32| {
        let mut d = flags.to_le_bytes().to_vec();
        d.extend([0u8; 20]);
        sub(b"ACBS", &d)
    };
    plugin.extend(group(
        *b"NPC_",
        0,
        &named(b"NPC_", SUNNY, Some("Sunny Smiles"), &acbs(0)),
    ));
    let mut creatures = named(b"CREA", GECKO, Some("Gecko"), &acbs(0));
    creatures.extend(named(
        b"CREA",
        TALKER,
        Some("Talking Gecko"),
        &acbs(activation::ALLOW_PC_DIALOGUE),
    ));
    plugin.extend(group(*b"CREA", 0, &creatures));
    let mut acti = named(b"ACTI", TESTER, Some("Vit-o-matic Vigor Tester"), &[]);
    acti.extend(named(b"ACTI", NAMELESS, None, &[]));
    plugin.extend(group(*b"ACTI", 0, &acti));
    plugin.extend(group(*b"BOOK", 0, &named(b"BOOK", BOOK, Some("Book"), &[])));
    let mut chest = named(b"CONT", CHEST, Some("Chest"), &[]);
    let mut cnto = KEY.to_le_bytes().to_vec();
    cnto.extend(1u32.to_le_bytes());
    chest.extend(named(b"CONT", LOCKER, Some("Locker"), &sub(b"CNTO", &cnto)));
    plugin.extend(group(*b"CONT", 0, &chest));
    plugin.extend(group(*b"DOOR", 0, &named(b"DOOR", DOOR, Some("Door"), &[])));
    // A light that can be carried: `DATA` flags (at 12) 0x2.
    let mut light = vec![0u8; 12];
    light.extend(2u32.to_le_bytes());
    light.extend([0u8; 20]);
    let mut lights = named(b"LIGH", LAMP, Some("Lamp"), &sub(b"DATA", &light));
    // One that can't be carried.
    lights.extend(named(b"LIGH", SCONCE, None, &sub(b"DATA", &[0u8; 36])));
    plugin.extend(group(*b"LIGH", 0, &lights));
    plugin.extend(group(*b"STAT", 0, &named(b"STAT", WALL, None, &[])));
    // A tin can worth 2, weighing 0.5; a rifle worth 75, weighing 7.
    let mut misc = named(
        b"MISC",
        CAN,
        Some("Tin Can"),
        &sub(b"DATA", &{
            let mut d = 2i32.to_le_bytes().to_vec();
            d.extend(0.5f32.to_le_bytes());
            d
        }),
    );
    misc.extend(named(
        b"KEYM",
        KEY,
        Some("Locker Key"),
        &sub(b"DATA", &[0u8; 8]),
    ));
    plugin.extend(group(*b"MISC", 0, &misc));
    plugin.extend(group(
        *b"WEAP",
        0,
        &named(
            b"WEAP",
            RIFLE,
            Some("Varmint Rifle"),
            &sub(b"DATA", &{
                let mut d = 75i32.to_le_bytes().to_vec();
                d.extend(100i32.to_le_bytes());
                d.extend(7.0f32.to_le_bytes());
                d.extend([0u8; 3]);
                d
            }),
        ),
    ));
    let mut furniture = named(
        b"FURN",
        CHAIR,
        Some("Chair"),
        &sub(b"MNAM", &0x4000_0004u32.to_le_bytes()),
    );
    furniture.extend(named(
        b"FURN",
        BED,
        Some("Bed"),
        &sub(b"MNAM", &0x8000_0001u32.to_le_bytes()),
    ));
    plugin.extend(group(*b"FURN", 0, &furniture));

    let mut refs = placed(b"REFR", CHAIR_REF, CHAIR, &[]);
    refs.extend(placed(b"REFR", BED_REF, BED, &[]));
    refs.extend(placed(b"REFR", CHEST_REF, CHEST, &[]));
    refs.extend(placed(b"REFR", LOCKED_REF, LOCKER, &xloc(25, 0)));
    refs.extend(placed(b"REFR", KEYED_REF, LOCKER, &xloc(25, KEY)));
    refs.extend(placed(
        b"REFR",
        CANS_REF,
        CAN,
        &sub(b"XCNT", &3u32.to_le_bytes()),
    ));
    refs.extend(placed(b"REFR", RIFLE_REF, RIFLE, &[]));
    refs.extend(placed(b"REFR", TESTER_REF, TESTER, &[]));
    refs.extend(placed(b"REFR", NAMELESS_REF, NAMELESS, &[]));
    refs.extend(placed(b"ACHR", SUNNY_REF, SUNNY, &[]));
    refs.extend(placed(b"ACRE", GECKO_REF, GECKO, &[]));
    refs.extend(placed(b"ACRE", TALKER_REF, TALKER, &[]));
    let mut xtel = FAR_DOOR_REF.to_le_bytes().to_vec();
    xtel.extend(f32s(&[0.0; 6]));
    refs.extend(placed(b"REFR", DOOR_REF, DOOR, &sub(b"XTEL", &xtel)));
    refs.extend(placed(b"REFR", BOOK_REF, BOOK, &[]));
    refs.extend(placed(b"REFR", LAMP_REF, LAMP, &[]));
    refs.extend(placed(b"REFR", WALL_REF, WALL, &[]));
    refs.extend(placed(b"REFR", SCONCE_REF, SCONCE, &[]));
    let mut cells = cell(ROOM, "Doc Mitchell's House", &refs);
    let mut back = DOOR_REF.to_le_bytes().to_vec();
    back.extend(f32s(&[0.0; 6]));
    cells.extend(cell(
        TOWN,
        "Goodsprings",
        &placed(b"REFR", FAR_DOOR_REF, DOOR, &sub(b"XTEL", &back)),
    ));
    plugin.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &cells)),
    ));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn state(order: &LoadOrder) -> GameState {
    let mut s = GameState::new(order);
    s.player_cell = Some(FormId(ROOM));
    s.player_world = None;
    s
}

#[test]
fn the_action_class_follows_the_form_type_and_the_references_state() {
    let (_data, order) = world();
    let mut s = state(&order);
    let c = |s: &GameState, r: u32| activation::action_class(&order, s, FormId(r));
    assert_eq!(c(&s, CHAIR_REF), class::SIT);
    assert_eq!(c(&s, BED_REF), class::SLEEP);
    assert_eq!(c(&s, CHEST_REF), class::OPEN);
    assert_eq!(c(&s, CANS_REF), class::TAKE);
    assert_eq!(c(&s, RIFLE_REF), class::TAKE);
    assert_eq!(c(&s, TESTER_REF), class::ACTIVATE);
    assert_eq!(c(&s, NAMELESS_REF), class::NONE);
    assert_eq!(c(&s, DOOR_REF), class::OPEN_DOOR);
    assert_eq!(c(&s, BOOK_REF), class::READ);
    assert_eq!(c(&s, LAMP_REF), class::TAKE);
    assert_eq!(c(&s, SUNNY_REF), class::TALK);
    // A creature talks only when its record allows it.
    assert_eq!(c(&s, GECKO_REF), class::NONE);
    assert_eq!(c(&s, TALKER_REF), class::TALK);
    assert_eq!(c(&s, PLAYER_REF.0), class::NONE);
    // Sneaking: their pockets.
    s.player_sneaking = true;
    assert_eq!(c(&s, SUNNY_REF), class::TAKE);
    assert_eq!(c(&s, GECKO_REF), class::NONE);
    s.player_sneaking = false;
    // The dead are searched.
    s.dead.insert(FormId(SUNNY_REF));
    s.dead.insert(FormId(GECKO_REF));
    assert_eq!(c(&s, SUNNY_REF), class::OPEN);
    assert_eq!(c(&s, GECKO_REF), class::OPEN);
    // Something set destroyed offers nothing.
    s.destroyed.insert(FormId(TESTER_REF));
    assert_eq!(c(&s, TESTER_REF), class::NONE);
}

#[test]
fn the_info_panel_says_what_the_game_says() {
    let (_data, order) = world();
    let mut s = state(&order);
    let info = |s: &GameState, r: u32| activation::info(&order, s, FormId(r)).unwrap();
    let words = |s: &GameState, r: u32| {
        let i = info(s, r);
        (i.action.unwrap_or_default(), i.target)
    };
    assert_eq!(words(&s, CHAIR_REF), ("Sit".into(), "Chair".into()));
    assert_eq!(words(&s, BED_REF), ("Sleep".into(), "Bed".into()));
    assert_eq!(words(&s, CANS_REF), ("Take".into(), "Tin Can (3)".into()));
    assert_eq!(words(&s, BOOK_REF), ("Take".into(), "Book".into()));
    assert_eq!(
        words(&s, TESTER_REF),
        ("Activate".into(), "Vit-o-matic Vigor Tester".into())
    );
    assert_eq!(words(&s, SUNNY_REF), ("Talk".into(), "Sunny Smiles".into()));
    // A load door: "Door to Goodsprings".
    assert_eq!(
        words(&s, DOOR_REF),
        ("Open".into(), "Door to Goodsprings".into())
    );
    // A gecko: its name, no action.
    let gecko = info(&s, GECKO_REF);
    assert_eq!((gecko.action, gecko.target.as_str()), (None, "Gecko"));
    // Nothing to say about a nameless activator.
    assert!(activation::info(&order, &s, FormId(NAMELESS_REF)).is_none());
    // Weight and value for items, not books.
    let can = info(&s, CANS_REF).weight_value.unwrap();
    assert_eq!((can.weight.as_str(), can.value.as_str()), ("0.5", "2"));
    assert_eq!(
        (can.weight_label.as_str(), can.value_label.as_str()),
        ("WG", "VAL")
    );
    let rifle = info(&s, RIFLE_REF).weight_value.unwrap();
    assert_eq!((rifle.weight.as_str(), rifle.value.as_str()), ("7.0", "75"));
    assert!(info(&s, BOOK_REF).weight_value.is_none());
    // Containers: empty, locked, or locked with the key held.
    assert_eq!(info(&s, CHEST_REF).empty.as_deref(), Some("Empty"));
    let locked = info(&s, LOCKED_REF);
    assert_eq!(locked.lock.as_deref(), Some("[Locked - Easy]"));
    assert_eq!(locked.empty, None);
    s.items.insert((PLAYER_REF, FormId(KEY)), 1);
    s.stocked.insert(PLAYER_REF);
    assert_eq!(
        info(&s, KEYED_REF).lock.as_deref(),
        Some("[Locked - Use Key]")
    );
    // Sneaking at a person: "Pickpocket", in the crime colour.
    s.player_sneaking = true;
    let pick = info(&s, SUNNY_REF);
    assert_eq!(pick.action.as_deref(), Some("Pickpocket"));
    assert!(pick.crime);
    s.player_sneaking = false;
    // The dead: "Search", and "Empty" when they hold nothing.
    s.dead.insert(FormId(SUNNY_REF));
    let body = info(&s, SUNNY_REF);
    assert_eq!(body.action.as_deref(), Some("Search"));
    assert_eq!(body.empty.as_deref(), Some("Empty"));
}

/// `00631d60`: the view caster's fuzzy candidate takes the place of an
/// exact pick on a static, a static collection, a tree, a light that can't
/// be carried, or something with no base; never of furniture, items,
/// containers, doors or people.
#[test]
fn the_fuzzy_pick_replaces_statics_and_fixed_lights_only() {
    let (_data, order) = world();
    let replaces = |r: u32| activation::fuzzy_replaces(&order, FormId(r));
    assert!(replaces(WALL_REF));
    assert!(replaces(SCONCE_REF));
    assert!(replaces(0xABCDE));
    for r in [
        LAMP_REF, CHAIR_REF, CANS_REF, CHEST_REF, DOOR_REF, SUNNY_REF, TESTER_REF,
    ] {
        assert!(!replaces(r), "{r:X}");
    }
}

#[test]
fn a_pickup_announces_the_item_added() {
    let (_data, order) = world();
    assert_eq!(
        activation::pickup_message(&order, FormId(CAN), 1).as_deref(),
        Some("Tin Can added")
    );
    assert_eq!(
        activation::pickup_message(&order, FormId(CAN), 3).as_deref(),
        Some("3 Tin Can(s) added")
    );
    // Notes and furniture aren't announced (`004ce380`'s type switch).
    assert_eq!(activation::pickup_message(&order, FormId(CHAIR), 1), None);
}

/// `00586500`: the place's name is the map region's (`RDAT` type 4 then
/// `RDMP`) whose outline holds the point, the highest priority first.
#[test]
fn a_place_is_named_by_its_map_region() {
    let data = testdata::TempData::empty("activation-region");
    let mut plugin = record(
        b"TES4",
        0,
        &sub(b"HEDR", &{
            let mut h = 1.34f32.to_le_bytes().to_vec();
            h.extend([0; 8]);
            h
        }),
    );
    let square = |lo: f32, hi: f32| f32s(&[lo, lo, hi, lo, hi, hi, lo, hi]);
    let region = |id: u32, name: &str, priority: u8, lo: f32, hi: f32| {
        let mut d = sub(b"EDID", &zstr(&format!("Region{id:X}")));
        d.extend(sub(b"WNAM", &0xC00u32.to_le_bytes()));
        d.extend(sub(b"RPLI", &0u32.to_le_bytes()));
        d.extend(sub(b"RPLD", &square(lo, hi)));
        d.extend(sub(b"RDAT", &[4, 0, 0, 0, 0, priority, 0, 0]));
        d.extend(sub(b"RDMP", &zstr(name)));
        record(b"REGN", id, &d)
    };
    let mut regions = region(0xA00, "Mojave", 10, -10000.0, 10000.0);
    regions.extend(region(0xA01, "Goodsprings", 50, 0.0, 1000.0));
    plugin.extend(group(*b"REGN", 0, &regions));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let at = |x: f32, y: f32| world::region::location_name(&order, FormId(0xC00), None, x, y);
    assert_eq!(at(500.0, 500.0).as_deref(), Some("Goodsprings"));
    assert_eq!(at(5000.0, 500.0).as_deref(), Some("Mojave"));
    assert_eq!(at(50000.0, 0.0), None);
    // Another worldspace's point: none.
    assert_eq!(
        world::region::location_name(&order, FormId(0xC01), None, 500.0, 500.0),
        None
    );
}
