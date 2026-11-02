//! The import on synthetic saves (`fos::write`) and a synthetic plugin.

use super::*;
use fos::write::{Form, PipeWriter, SaveWriter};
use testdata::{f32s, group, record, sub, zstr};

const HOUR: u32 = 0x100;
const SCRIPT: u32 = 0x200;
const QUEST: u32 = 0x300;
const ITEM: u32 = 0x400;
const PICKED: u32 = 0x401;
const LIST: u32 = 0x402;
const GUN: u32 = 0x410;
const CHEST: u32 = 0x500;
const PERK: u32 = 0x600;
const ROOM: u32 = 0x900;
const CHEST_REF: u32 = 0x901;
const MARKER_REF: u32 = 0x902;
const LAMP_REF: u32 = 0x903;
const LAMP: u32 = 0x910;

fn edid(name: &str) -> Vec<u8> {
    sub(b"EDID", &zstr(name))
}

/// `SLSD` for variable `index` (whole number or not) and its `SCVR`.
fn variable(index: u32, integer: bool, name: &str) -> Vec<u8> {
    let mut slsd = index.to_le_bytes().to_vec();
    slsd.extend([0; 12]);
    slsd.push(u8::from(integer));
    slsd.extend([0; 7]);
    let mut v = sub(b"SLSD", &slsd);
    v.extend(sub(b"SCVR", &zstr(name)));
    v
}

fn cnto(item: u32, n: u32) -> Vec<u8> {
    let mut d = item.to_le_bytes().to_vec();
    d.extend(n.to_le_bytes());
    sub(b"CNTO", &d)
}

/// A FalloutNV.esm with a global, a quest and its script, items, a leveled
/// list, a weapon, a container, a perk and a room holding a container, a
/// map marker and a lamp.
fn order() -> LoadOrder {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &hedr));

    let mut glob = edid("GameHour");
    glob.extend(sub(b"FNAM", b"f"));
    glob.extend(sub(b"FLTV", &f32s(&[8.0])));
    bytes.extend(group(*b"GLOB", 0, &record(b"GLOB", HOUR, &glob)));

    let mut scpt = edid("TestQuestScript");
    scpt.extend(variable(1, true, "count"));
    scpt.extend(variable(2, false, "timer"));
    scpt.extend(variable(3, false, "target"));
    bytes.extend(group(*b"SCPT", 0, &record(b"SCPT", SCRIPT, &scpt)));

    let mut qust = edid("TestQuest");
    qust.extend(sub(b"SCRI", &SCRIPT.to_le_bytes()));
    qust.extend(sub(b"DATA", &[0; 8]));
    bytes.extend(group(*b"QUST", 0, &record(b"QUST", QUEST, &qust)));

    let mut misc = record(b"MISC", ITEM, &edid("Thing"));
    misc.extend(record(b"MISC", PICKED, &edid("Picked")));
    bytes.extend(group(*b"MISC", 0, &misc));

    let mut lvli = edid("Pick");
    let mut lvlo = 1u16.to_le_bytes().to_vec();
    lvlo.extend([0; 2]);
    lvlo.extend(PICKED.to_le_bytes());
    lvlo.extend(1u16.to_le_bytes());
    lvlo.extend([0; 2]);
    lvli.extend(sub(b"LVLD", &[0]));
    lvli.extend(sub(b"LVLO", &lvlo));
    bytes.extend(group(*b"LVLI", 0, &record(b"LVLI", LIST, &lvli)));

    let mut weap = edid("Gun");
    let mut data = vec![0; 15];
    data[4..8].copy_from_slice(&200u32.to_le_bytes());
    weap.extend(sub(b"DATA", &data));
    bytes.extend(group(*b"WEAP", 0, &record(b"WEAP", GUN, &weap)));

    let mut cont = edid("Chest");
    cont.extend(cnto(ITEM, 3));
    cont.extend(cnto(LIST, 1));
    bytes.extend(group(*b"CONT", 0, &record(b"CONT", CHEST, &cont)));
    bytes.extend(group(*b"STAT", 0, &record(b"STAT", LAMP, &edid("Lamp"))));
    bytes.extend(group(*b"PERK", 0, &record(b"PERK", PERK, &edid("Perky"))));

    let mut refs = testdata::placed(CHEST_REF, CHEST, [0.0; 3], [0.0; 3], &[]);
    let mut marker = sub(b"XMRK", &[]);
    marker.extend(sub(b"FNAM", &[0]));
    refs.extend(testdata::placed(
        MARKER_REF, LAMP, [0.0; 3], [0.0; 3], &marker,
    ));
    refs.extend(testdata::placed(LAMP_REF, LAMP, [0.0; 3], [0.0; 3], &[]));
    let mut cell = edid("Room");
    cell.extend(sub(b"DATA", &[1]));
    let mut contents = record(b"CELL", ROOM, &cell);
    contents.extend(group(
        ROOM.to_le_bytes(),
        6,
        &group(ROOM.to_le_bytes(), 9, &refs),
    ));
    bytes.extend(group(
        *b"CELL",
        0,
        &group([0; 4], 2, &group([0; 4], 3, &contents)),
    ));
    let plugin = esm::Plugin::from_bytes(bytes).unwrap();
    LoadOrder::from_plugins(vec![("FalloutNV.esm".into(), None, plugin)]).unwrap()
}

/// A moved reference's 27 bytes of initial data.
fn place(w: &mut PipeWriter, space: u32, at: [f32; 3], heading: f32) {
    let mut b = vec![(space >> 16) as u8, (space >> 8) as u8, space as u8];
    for v in at.iter().chain(&[0.0, 0.0, heading]) {
        b.extend_from_slice(&v.to_le_bytes());
    }
    w.bytes(&b);
}

/// The form ids the saves below use, by refID (from 1).
const FORM_IDS: [u32; 12] = [
    QUEST, CHEST_REF, ITEM, GUN, MARKER_REF, LAMP_REF, 0x14, ROOM, PERK, 0x7, HOUR, PICKED,
];

fn refid(form: u32) -> u32 {
    FORM_IDS.iter().position(|&f| f == form).unwrap() as u32 + 1
}

fn save(forms: Vec<Form>, global_data: Vec<(u32, Vec<u8>)>) -> Vec<u8> {
    SaveWriter {
        global_data_1: global_data,
        forms,
        form_ids: FORM_IDS.to_vec(),
        ..SaveWriter::default()
    }
    .build()
    .0
}

fn run(forms: Vec<Form>, global_data: Vec<(u32, Vec<u8>)>) -> Import {
    let import = import(&order(), &save(forms, global_data)).unwrap();
    assert!(import.report.failures.is_empty(), "{:?}", import.report);
    import
}

#[test]
fn plugins_are_matched_by_name_and_unknown_ones_dropped() {
    let mut w = SaveWriter {
        plugins: vec!["DeadMoney.esm".into(), "falloutnv.ESM".into()],
        form_ids: vec![0x0100_0000 | QUEST, 0x0000_1234, 0xFF00_0007],
        ..SaveWriter::default()
    };
    let flags = PipeWriter::new().u8(0x01).finish();
    w.forms = vec![
        Form::new(1, 0x2, fos::save_type::QUST, flags.clone()),
        Form::new(2, 0x2, fos::save_type::QUST, flags),
    ];
    let (bytes, _) = w.build();
    let order = order();
    let save = Save::parse(&bytes).unwrap();
    let (ids, missing) = Ids::new(&order, &save);
    assert_eq!(missing, ["DeadMoney.esm"]);
    assert_eq!(ids.of(RefId(1)), Some(FormId(QUEST)));
    assert_eq!(ids.of(RefId(2)), None);
    assert_eq!(ids.of(RefId(3)), Some(FormId(0xFF00_0007)));
    let import = import(&order, &bytes).unwrap();
    assert!(import.state.running.contains(&FormId(QUEST)));
    assert_eq!(import.report.dropped, 1);
}

#[test]
fn quests_take_their_flags_stages_objectives_and_variables() {
    let mut q = PipeWriter::new();
    q.u8(0x01 | 0x02).f32(1.5);
    // Stages 10 (done) and 20 (not).
    q.vsval(2).u8(10).u8(1).vsval(0).u8(20).u8(0).vsval(0);
    // Variables: count = 3, timer = 2.5, target = the chest.
    q.vsval(3);
    q.u32(1).f64(3.0).u32(2).f64(2.5);
    q.u32(0x8000_0003).ref_id(refid(CHEST_REF));
    q.u8(0).u8(0);
    // Objectives: 10 shown, 20 completed and shown, 30 completed hidden.
    q.vsval(3).u32(10).u32(1).u32(20).u32(3).u32(30).u32(2);
    let flags = 0x2 | 0x4 | 0x8000_0000 | 0x4000_0000 | 0x2000_0000;
    let i = run(
        vec![Form::new(
            refid(QUEST),
            flags,
            fos::save_type::QUST,
            q.finish(),
        )],
        Vec::new(),
    );
    let s = &i.state;
    let quest = FormId(QUEST);
    assert!(s.running.contains(&quest) && s.completed.contains(&quest));
    assert_eq!(s.stages.get(&quest), Some(&10));
    assert!(s.stages_done.contains(&(quest, 10)) && !s.stages_done.contains(&(quest, 20)));
    assert_eq!(s.quest_delays.get(&quest), Some(&1.5));
    assert_eq!(s.objectives.get(&(quest, 10)), Some(&false));
    assert_eq!(s.objectives.get(&(quest, 20)), Some(&true));
    assert!(s.set_by_scripts.hidden_completed.contains(&(quest, 30)));
    let vars = &s.variables[&quest];
    assert_eq!(vars.get("count"), Some(3.0));
    assert_eq!(vars.get("timer"), Some(2.5));
    assert_eq!(vars.get("target"), Some(f64::from(CHEST_REF)));
    let kinds: Vec<_> = vars.iter().map(|(n, k, _)| (n.to_string(), k)).collect();
    assert!(kinds.contains(&("count".into(), VarKind::Integer)));
    assert!(kinds.contains(&("target".into(), VarKind::Ref)));
}

#[test]
fn globals_stats_weather_and_radio_are_taken() {
    let globals = PipeWriter::new()
        .vsval(1)
        .ref_id(refid(HOUR))
        .f32(13.25)
        .finish();
    let stats = PipeWriter::new().u32(2).u32(0).u32(5).finish();
    let mut sky = PipeWriter::new();
    sky.ref_id(refid(ITEM))
        .ref_id(0)
        .ref_id(refid(GUN))
        .ref_id(0);
    sky.f32(13.0).f32(12.5).f32(1.0).u32(0).f32(0.0);
    sky.f32(0.0).f32(0.0).f32(0.0).f32(0.0).f32(0.5).u32(0);
    let mut radio = PipeWriter::new();
    radio.u32(0).u8(1).wstr("").vsval(0).vsval(0);
    radio
        .ref_id(refid(LAMP_REF))
        .ref_id(0)
        .vsval(1)
        .ref_id(refid(LAMP_REF));
    let i = run(
        Vec::new(),
        vec![
            (0, stats),
            (3, globals),
            (8, sky.finish()),
            (10, radio.finish()),
        ],
    );
    let s = &i.state;
    assert_eq!(s.globals.get(&FormId(HOUR)), Some(&13.25));
    assert_eq!(s.misc_stats.get(&1), Some(&5));
    assert_eq!(s.weather.current, Some(FormId(ITEM)));
    assert_eq!(s.weather.picked, Some(FormId(GUN)));
    assert_eq!((s.weather.started, s.weather.fade), (12.5, 1.0));
    assert!(s.radio.on);
    assert_eq!(s.radio.active, Some(FormId(LAMP_REF)));
    assert_eq!(s.radio.discovered, [FormId(LAMP_REF)]);
}

#[test]
fn the_player_is_placed_with_perks_values_and_keys() {
    let mut w = PipeWriter::new();
    place(&mut w, refid(ROOM), [10.0, 20.0, 30.0], 1.5);
    w.player_head(&[(24, 40.0), (5, 1.0)]);
    w.u8(0xFF)
        .mobile_tail()
        .actor_fixed()
        .still_mover(true)
        .u8(0)
        .u8(0);
    let mut keys = [0; 8];
    keys[0] = GUN;
    w.player_tail(&[(refid(PERK), 2)], refid(QUEST), keys);
    let player = Form::new(refid(0x14), 0x2, fos::save_type::ACHR, w.finish());
    // The player's base: S.P.E.C.I.A.L. and the name.
    let mut b = PipeWriter::new();
    b.bytes(&[6, 5, 4, 3, 2, 1, 7]).wstr("Tester");
    let base = Form::new(refid(0x7), 0x4 | 0x20, fos::save_type::NPC_, b.finish());
    let i = run(vec![player, base], Vec::new());
    let p = i.place.unwrap();
    assert_eq!((p.cell, p.world), (FormId(ROOM), None));
    assert_eq!((p.position, p.heading), ([10.0, 20.0, 30.0], 1.5));
    let s = &i.state;
    assert_eq!(s.player_cell, Some(FormId(ROOM)));
    assert_eq!(s.player_name.as_deref(), Some("Tester"));
    assert_eq!(s.actor_values.get(&(PLAYER_REF, 5)), Some(&7.0));
    assert_eq!(s.actor_values.get(&(PLAYER_REF, 11)), Some(&7.0));
    assert_eq!(s.actor_values.get(&(PLAYER_REF, 24)), Some(&40.0));
    assert!(s.perks.contains(&FormId(PERK)));
    assert_eq!(crate::perks::rank(s, FormId(PERK)), 2);
    assert_eq!(s.active_quest, Some(FormId(QUEST)));
    assert_eq!(s.hotkeys[0], Some(FormId(GUN)));
}

#[test]
fn references_take_their_state_and_holders_their_items() {
    // The chest: moved, its inventory changed (one Thing fewer, the
    // leveled list's pick already made by the game, a gun at half health
    // and worn), its lock locked at 50.
    let mut c = PipeWriter::new();
    place(&mut c, refid(ROOM), [1.0, 2.0, 3.0], 0.5);
    c.vsval(1).u8(fos::decode::extra_kind::LOCK);
    c.u8(50).u8(1).ref_id(0).u32(0).u32(0);
    c.vsval(3);
    c.ref_id(refid(ITEM)).i32(-1).vsval(0);
    c.ref_id(refid(PICKED)).i32(1).vsval(0);
    c.ref_id(refid(GUN)).i32(1).vsval(1);
    c.vsval(2).u8(fos::decode::extra_kind::HEALTH).f32(100.0);
    c.u8(fos::decode::extra_kind::WORN);
    let chest_flags = 0x2 | 0x1000 | 0x20 | 0x0800_0000;
    let chest = Form::new(
        refid(CHEST_REF),
        chest_flags,
        fos::save_type::REFR,
        c.finish(),
    );
    // The marker found; the lamp disabled.
    let mut m = PipeWriter::new();
    m.vsval(1).u8(fos::decode::extra_kind::MAP_MARKER).u8(3);
    let marker = Form::new(
        refid(MARKER_REF),
        0x8000_0000,
        fos::save_type::REFR,
        m.finish(),
    );
    let lamp = Form::new(
        refid(LAMP_REF),
        0x1,
        fos::save_type::REFR,
        PipeWriter::new().u32(0x800).finish(),
    );
    // A Thing dropped in game, two of them.
    let mut d = vec![0, 0, refid(ROOM) as u8];
    for v in [5.0f32, 6.0, 7.0, 0.0, 0.0, 0.0] {
        d.extend_from_slice(&v.to_le_bytes());
    }
    d.push(0);
    d.extend([0, 0, refid(ITEM) as u8]);
    let mut dw = PipeWriter::new();
    dw.bytes(&d)
        .vsval(1)
        .u8(fos::decode::extra_kind::COUNT)
        .u16(2);
    let dropped = Form::new(0x80_0003, 0x8000_0400, fos::save_type::REFR, dw.finish());

    let i = run(vec![chest, marker, lamp, dropped], Vec::new());
    let s = &i.state;
    let chest = FormId(CHEST_REF);
    assert_eq!(s.positions.get(&chest), Some(&([1.0, 2.0, 3.0], 0.5)));
    assert_eq!(s.locks.get(&chest), Some(&Some(50)));
    assert_eq!(s.items.get(&(chest, FormId(ITEM))), Some(&2));
    assert_eq!(s.items.get(&(chest, FormId(PICKED))), Some(&1));
    assert_eq!(s.items.get(&(chest, FormId(GUN))), Some(&1));
    assert_eq!(s.weapon_health.get(&(chest, FormId(GUN))), Some(&0.5));
    assert_eq!(s.equipped.get(&chest), Some(&vec![FormId(GUN)]));
    assert!(s.stocked.contains(&chest));
    assert!(s.discovered.contains(&FormId(MARKER_REF)));
    assert_eq!(s.disabled.get(&FormId(LAMP_REF)), Some(&true));
    let made = &s.more.placed.refs[&FormId(0xFF00_0003)];
    assert_eq!(
        (made.base, made.space, made.count),
        (FormId(ITEM), FormId(ROOM), 2)
    );
    assert_eq!(made.position, [5.0, 6.0, 7.0]);
    assert_eq!(s.more.placed.next, 3);
}

#[test]
fn people_take_their_life_state_damage_and_values() {
    // An actor (not in the plugin) dead, with health lost, a value set by
    // a script and one added to it, and well disposed to the player.
    let mut w = PipeWriter::new();
    w.u8(3).mobile_tail();
    // A low process with no package, then 25 health lost.
    w.zeros(&[4, 4, 4]).ref_id(0);
    w.zeros(&[1, 4]).ref_id(0).zeros(&[4, 4, 4, 1, 2, 4, 4]);
    for _ in 0..5 {
        w.ref_id(0);
    }
    w.vsval(0).vsval(1).u8(16).f32(-25.0);
    w.actor_fixed().u8(2);
    w.vsval(1).ref_id(refid(0x14)).u32(15);
    w.vsval(1).u8(40).f32(5.0);
    w.vsval(1).u8(40).f32(50.0);
    w.still_mover(false).u8(0).u8(0);
    let flags = 0x400 | 0x8_0000 | 0x80_0000 | 0x40_0000 | 0x20_0000;
    let mut save_ids = FORM_IDS.to_vec();
    save_ids.push(0x0000_0ABC);
    let w = SaveWriter {
        forms: vec![Form::new(13, flags, fos::save_type::ACHR, w.finish())],
        form_ids: save_ids,
        ..SaveWriter::default()
    };
    let i = import(&order(), &w.build().0).unwrap();
    assert!(i.report.failures.is_empty(), "{:?}", i.report);
    let s = &i.state;
    let who = FormId(0xABC);
    assert!(s.dead.contains(&who));
    assert_eq!(s.damage.get(&who), Some(&25.0));
    assert_eq!(s.actor_values.get(&(who, 40)), Some(&55.0));
    assert_eq!(s.more.dispositions.0.get(&who), Some(&15));
}

#[test]
fn factions_challenges_reputations_and_seen_cells_are_taken() {
    // Faction: crimes major 2, minor 5.
    let fact = Form::new(
        refid(ITEM),
        0x8000_0000,
        fos::save_type::FACT,
        PipeWriter::new().i32(2).i32(5).finish(),
    );
    let chal = Form::new(
        refid(GUN),
        0,
        fos::save_type::CHAL,
        PipeWriter::new().u32(7).u32(1).finish(),
    );
    let repu = Form::new(
        refid(LAMP_REF),
        0,
        fos::save_type::REPU,
        PipeWriter::new().f32(10.0).f32(2.0).finish(),
    );
    let mut seen = PipeWriter::new();
    let mut bits = [0u8; 32];
    bits[0] = 0x81;
    seen.vsval(1).u8(0xFF).u8(1).bytes(&bits);
    let cell = Form::new(
        refid(ROOM),
        0x8000_0000,
        fos::save_type::CELL,
        seen.finish(),
    );
    let i = run(vec![fact, chal, repu, cell], Vec::new());
    let s = &i.state;
    assert_eq!(s.faction_crimes.get(&FormId(ITEM)), Some(&(5, 2)));
    assert_eq!(s.more.challenges.progress.get(&FormId(GUN)), Some(&(7, 1)));
    assert_eq!(s.reputations.get(&FormId(LAMP_REF)), Some(&(10.0, 2.0)));
    let key = crate::local_map::SeenKey::Interior(FormId(ROOM), -1, 1);
    assert_eq!(s.seen.bits.get(&key).map(|b| b[0]), Some(0x81));
}

use crate::inventory_stacks::{SavedExtra as X, SavedInventory, SavedItem, SavedResolution};
use fos::decode::extra_kind as k;

/// The gun's stacks: an explicit count of 0, a signaling NaN's health and
/// two extras nv-rs doesn't read (a poison and a hot key); an empty
/// stack; two at health 100, worn.
fn gun_change(w: &mut PipeWriter) {
    w.ref_id(refid(GUN)).i32(1).vsval(3);
    w.vsval(4)
        .u8(k::COUNT)
        .u16(0)
        .u8(k::HEALTH)
        .u32(0x7F80_0001);
    w.u8(k::POISON).ref_id(refid(ITEM)).u8(k::HOT_KEY).u8(3);
    w.vsval(0);
    w.vsval(3).u8(k::COUNT).u16(2).u8(k::HEALTH).f32(100.0);
    w.u8(k::WORN);
}

fn gun_stacks() -> SavedItem {
    let poison = PipeWriter::new().u8(k::POISON).ref_id(refid(ITEM)).finish();
    let hot_key = PipeWriter::new().u8(k::HOT_KEY).u8(3).finish();
    SavedItem {
        saved: refid(GUN),
        resolution: SavedResolution::InSave(GUN),
        form: Some(FormId(GUN)),
        count_delta: 1,
        stacks: vec![
            crate::inventory_stacks::SavedStack {
                extras: vec![
                    X::Count(0),
                    X::Health(0x7F80_0001),
                    X::Opaque {
                        kind: k::POISON,
                        bytes: poison,
                    },
                    X::Opaque {
                        kind: k::HOT_KEY,
                        bytes: hot_key,
                    },
                ],
            },
            crate::inventory_stacks::SavedStack::default(),
            crate::inventory_stacks::SavedStack {
                extras: vec![X::Count(2), X::Health(100f32.to_bits()), X::Worn],
            },
        ],
    }
}

/// What a save of the state and loading it back keeps of the inventory.
fn through_own_save(s: &GameState) -> GameState {
    let text = crate::save::save(s, None);
    crate::save::load(&text).unwrap().0
}

#[test]
fn holders_keep_their_stacks_raw_health_and_worn_state() {
    // The chest's inventory: the gun's three stacks, and one Thing fewer
    // in a stack that can't be worn.
    let mut c = PipeWriter::new();
    c.vsval(2);
    gun_change(&mut c);
    c.ref_id(refid(ITEM)).i32(-1).vsval(1);
    c.vsval(1).u8(k::CANNOT_WEAR);
    let chest = Form::new(refid(CHEST_REF), 0x20, fos::save_type::REFR, c.finish());
    let version = chest.version;
    let i = run(vec![chest], Vec::new());
    let s = &i.state;
    let holder = FormId(CHEST_REF);
    let expected = SavedInventory {
        version,
        items: vec![
            gun_stacks(),
            SavedItem {
                saved: refid(ITEM),
                resolution: SavedResolution::InSave(ITEM),
                form: Some(FormId(ITEM)),
                count_delta: -1,
                stacks: vec![crate::inventory_stacks::SavedStack {
                    extras: vec![X::Opaque {
                        kind: k::CANNOT_WEAR,
                        bytes: vec![k::CANNOT_WEAR, b'|'],
                    }],
                }],
            },
        ],
    };
    assert_eq!(s.inventory_stacks.get(&holder), Some(&expected));
    // The views nv-rs plays with, as before: the record's three Things
    // one fewer, the leveled list rolled, the gun's delta; the last
    // stack's health is its condition and the worn stack equips it.
    assert_eq!(s.items.get(&(holder, FormId(ITEM))), Some(&2));
    assert_eq!(s.items.get(&(holder, FormId(PICKED))), Some(&1));
    assert_eq!(s.items.get(&(holder, FormId(GUN))), Some(&1));
    assert_eq!(s.weapon_health.get(&(holder, FormId(GUN))), Some(&0.5));
    assert_eq!(s.equipped.get(&holder), Some(&vec![FormId(GUN)]));

    let back = through_own_save(s);
    assert_eq!(back.inventory_stacks, s.inventory_stacks);
    assert_eq!(back.items, s.items);
    assert_eq!(back.weapon_health, s.weapon_health);
    assert_eq!(back.equipped, s.equipped);
    let gun = &back.inventory_stacks[&holder].items[0];
    assert_eq!(gun.stacks[0].health_bits(), Some(0x7F80_0001));
    assert_eq!(gun.stacks[0].count(), Some(0));
    assert_eq!(gun.stacks[1].count(), None);
    assert!(gun.stacks[2].worn());
}

#[test]
fn the_players_stacks_keep_unresolved_items_out_of_the_counts() {
    // The player's inventory: the gun, an item whose refID is past the
    // save's form id array, and one saved as refID 0.
    let mut inv = PipeWriter::new();
    inv.vsval(3);
    gun_change(&mut inv);
    inv.ref_id(0x50).i32(4).vsval(0);
    inv.ref_id(0).i32(1).vsval(1).vsval(1).u8(k::WORN);
    let mut head = PipeWriter::new();
    head.player_head(&[]).u8(0xFF);
    let mut data = head.finish();
    data.extend(inv.finish());
    let mut tail = PipeWriter::new();
    tail.mobile_tail()
        .actor_fixed()
        .still_mover(true)
        .u8(0)
        .u8(0)
        .player_tail(&[], 0, [0; 8]);
    data.extend(tail.finish());
    let player = Form::new(refid(0x14), 0x20, fos::save_type::ACHR, data);
    let i = run(vec![player], Vec::new());
    let s = &i.state;
    let saved = &s.inventory_stacks[&PLAYER_REF];
    assert_eq!(saved.items.len(), 3);
    assert_eq!(saved.items[0], gun_stacks());
    let missing = &saved.items[1];
    assert_eq!(
        (
            missing.saved,
            missing.resolution,
            missing.form,
            missing.count_delta
        ),
        (0x50, SavedResolution::Missing(0x50), None, 4)
    );
    let null = &saved.items[2];
    assert_eq!((null.resolution, null.form), (SavedResolution::Null, None));
    assert!(null.stacks[0].worn());
    // Only the gun reaches the counts and the worn items.
    assert_eq!(s.items.get(&(PLAYER_REF, FormId(GUN))), Some(&1));
    assert_eq!(s.equipped.get(&PLAYER_REF), Some(&vec![FormId(GUN)]));
    assert_eq!(s.weapon_health.get(&(PLAYER_REF, FormId(GUN))), Some(&0.5));
    let back = through_own_save(s);
    assert_eq!(back.inventory_stacks, s.inventory_stacks);
    assert_eq!(back.items, s.items);
}
