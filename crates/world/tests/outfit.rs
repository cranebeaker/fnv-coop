//! What a person wears now (`world::outfit`), as the redraw follows it,
//! from a plugin built from scratch: their record's clothes from the
//! start, then equipping, taking off and taking away.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{group, record, sub, zstr};
use world::outfit::{start_worn, worn_armour};
use world::scripting::GameState;

const SHIRT: u32 = 0x900;
const DRESS: u32 = 0x901;
const HAT: u32 = 0x902;
const GLOVES_A: u32 = 0x903;
const GLOVES_B: u32 = 0x904;
const GLOVES: u32 = 0x950;
const NPC: u32 = 0x700;
const NPC_REF: u32 = 0x800;

fn plugin() -> Vec<u8> {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    let clothes = |id: u32, name: &str, slots: u32| {
        let mut d = sub(b"EDID", &zstr(name));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        d.extend(sub(b"MODL", &zstr(&format!("Armor\\{name}.nif"))));
        record(b"ARMO", id, &d)
    };
    // A shirt (upper body), a dress over the upper body and the left hand,
    // a hat (the hat slot) and two pairs of gloves (both hands).
    let mut armour = clothes(SHIRT, "Shirt", 0x04);
    armour.extend(clothes(DRESS, "Dress", 0x04 | 0x08));
    armour.extend(clothes(HAT, "Hat", 0x400));
    armour.extend(clothes(GLOVES_A, "GlovesA", 0x08 | 0x10));
    armour.extend(clothes(GLOVES_B, "GlovesB", 0x08 | 0x10));
    plugin.extend(group(*b"ARMO", 0, &armour));
    // Gloves from a leveled list: either pair, at level 1.
    let mut list = sub(b"EDID", &zstr("LLGloves"));
    list.extend(sub(b"LVLD", &[0]));
    list.extend(sub(b"LVLF", &[0]));
    for gloves in [GLOVES_A, GLOVES_B] {
        let mut entry = 1u16.to_le_bytes().to_vec();
        entry.extend([0; 2]);
        entry.extend(gloves.to_le_bytes());
        entry.extend(1u16.to_le_bytes());
        entry.extend([0; 2]);
        list.extend(sub(b"LVLO", &entry));
    }
    plugin.extend(group(*b"LVLI", 0, &record(b"LVLI", GLOVES, &list)));
    // The person: carries the shirt, the hat and the gloves' list.
    let mut npc = sub(b"EDID", &zstr("TestPerson"));
    npc.extend(sub(b"MODL", &zstr("Characters\\_Male\\Skeleton.NIF")));
    npc.extend(sub(b"ACBS", &[0; 24]));
    for item in [SHIRT, HAT, GLOVES] {
        let mut cnto = item.to_le_bytes().to_vec();
        cnto.extend(1u32.to_le_bytes());
        npc.extend(sub(b"CNTO", &cnto));
    }
    plugin.extend(group(*b"NPC_", 0, &record(b"NPC_", NPC, &npc)));
    let placed = testdata::placed(NPC_REF, NPC, [0.0; 3], [0.0; 3], &[]);
    plugin.extend(group(*b"CELL", 0, &placed));
    plugin
}

fn ids(list: &[u32]) -> Vec<FormId> {
    list.iter().map(|&i| FormId(i)).collect()
}

#[test]
fn what_is_worn_follows_equipping_taking_off_and_taking_away() {
    let data = testdata::quests("outfit");
    data.write("FalloutNV.esm", &plugin());
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let who = FormId(NPC_REF);
    let start = start_worn(&order, FormId(NPC));
    // From the start, put on slot by slot (006047c0): the shirt (slot 2),
    // the list's first gloves (slot 3), the hat (slot 10); the list's gloves
    // are known to come from the list.
    assert_eq!(
        start.pieces.iter().map(|p| p.item).collect::<Vec<_>>(),
        ids(&[SHIRT, GLOVES_A, HAT])
    );
    assert_eq!(start.pieces[1].from, ids(&[GLOVES_A, GLOVES_B]));
    let mut state = GameState::new(&order);
    let worn = |state: &GameState| worn_armour(&order, state, who, &start);
    assert_eq!(worn(&state), ids(&[SHIRT, GLOVES_A, HAT]));
    // Untouched, the look made from what's worn is the record's own.
    let look = world::actor::npc_look_wearing(&order, FormId(NPC), &worn(&state), None);
    assert_eq!(look, world::actor_look(&order, FormId(NPC)));

    // Equipping what they don't carry does nothing (`0088c830`).
    state.equip(&order, who, FormId(DRESS));
    assert_eq!(worn(&state), ids(&[SHIRT, GLOVES_A, HAT]));
    // Given the dress, it's worn, and what it covers comes off.
    state.stock(&order, who);
    // The list's pick, whichever it was, keeps the gloves drawn.
    state.items.remove(&(who, FormId(GLOVES_B)));
    state.items.insert((who, FormId(GLOVES_A)), 1);
    state.items.insert((who, FormId(DRESS)), 1);
    state.equip(&order, who, FormId(DRESS));
    assert_eq!(worn(&state), ids(&[HAT, DRESS]));
    let models: Vec<String> =
        world::actor::npc_look_wearing(&order, FormId(NPC), &worn(&state), None)
            .unwrap()
            .parts
            .into_iter()
            .map(|p| p.model)
            .collect();
    assert_eq!(models, ["Armor\\Hat.nif", "Armor\\Dress.nif"]);
    // Taken off: what it replaced stays off while they still carry it.
    state.unequip_item(&order, who, FormId(DRESS));
    assert_eq!(worn(&state), ids(&[HAT]));
    // Taken away: they pick again, and wear the shirt and gloves again.
    state.items.remove(&(who, FormId(DRESS)));
    assert_eq!(worn(&state), ids(&[SHIRT, GLOVES_A, HAT]));

    // The hat taken off, then put on again.
    state.unequip_item(&order, who, FormId(HAT));
    assert_eq!(worn(&state), ids(&[SHIRT, GLOVES_A]));
    state.equip(&order, who, FormId(HAT));
    assert_eq!(worn(&state), ids(&[SHIRT, GLOVES_A, HAT]));

    // Taken away from the start's: the shirt; the gloves once neither
    // pair the list could give is left.
    state.items.remove(&(who, FormId(SHIRT)));
    assert_eq!(worn(&state), ids(&[GLOVES_A, HAT]));
    state.items.insert((who, FormId(GLOVES_A)), 0);
    state.items.insert((who, FormId(GLOVES_B)), 1);
    assert_eq!(worn(&state), ids(&[GLOVES_A, HAT]));
    state.items.remove(&(who, FormId(GLOVES_B)));
    assert_eq!(worn(&state), ids(&[HAT]));
}

#[test]
fn the_best_armour_for_each_slot_is_put_on() {
    // 004c8220: the highest DR (DNAM u16 / 100, truncated) + DT wins its
    // slot, the first of equals; 006047c0 puts on slot 2's first, and a
    // piece over the upper body only for slot 2.
    let data = testdata::quests("outfit-best");
    let mut plugin = plugin();
    let armour = |id: u32, name: &str, slots: u32, dr: u16, dt: f32| {
        let mut d = sub(b"EDID", &zstr(name));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        let mut dnam = dr.to_le_bytes().to_vec();
        dnam.extend([0; 2]);
        dnam.extend(dt.to_le_bytes());
        d.extend(sub(b"DNAM", &dnam));
        let mut fields = 10i32.to_le_bytes().to_vec();
        fields.extend(100i32.to_le_bytes());
        fields.extend(1f32.to_le_bytes());
        d.extend(sub(b"DATA", &fields));
        record(b"ARMO", id, &d)
    };
    let mut more = armour(0x910, "Vest", 0x04, 100, 0.0);
    more.extend(armour(0x911, "Plate", 0x04, 1000, 2.0));
    more.extend(armour(0x912, "Helmet", 0x400 | 0x04, 2000, 0.0));
    more.extend(armour(0x913, "Mitts", 0x08 | 0x10, 0, 1.0));
    more.extend(armour(0x914, "MittsToo", 0x08 | 0x10, 0, 1.0));
    plugin.extend(group(*b"ARMO", 0, &more));
    data.write("FalloutNV.esm", &plugin);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let carried = |items: &[u32]| -> Vec<(FormId, Vec<(FormId, i32)>)> {
        items
            .iter()
            .map(|&i| (FormId(i), vec![(FormId(i), 1)]))
            .collect()
    };
    // The plate (10 + 2) beats the vest (1) on slot 2; the helmet covers
    // the upper body too (20), so it wins slot 2 and the hat slot comes
    // with it; the first mitts of equals.
    let pick = |items: &[u32]| world::actor::pick_worn(&order, &carried(items));
    assert_eq!(pick(&[0x910, 0x911]), ids(&[0x911]));
    assert_eq!(pick(&[0x910, 0x911, 0x912]), ids(&[0x912]));
    assert_eq!(pick(&[0x913, 0x914]), ids(&[0x913]));
    assert_eq!(
        world::actor::best_armour(&order, &ids(&[0x910, 0x911]), 2),
        Some(FormId(0x911))
    );
}

#[test]
fn what_came_off_is_saved() {
    let data = testdata::quests("outfit-save");
    data.write("FalloutNV.esm", &plugin());
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let who = FormId(NPC_REF);
    let mut state = GameState::new(&order);
    state.stock(&order, who);
    state.items.insert((who, FormId(DRESS)), 1);
    state.equip(&order, who, FormId(DRESS));
    state.unequip_item(&order, who, FormId(HAT));
    let off = state.taken_off.get(&who).cloned().unwrap();
    assert!(off.contains(&(FormId(SHIRT), Some(FormId(DRESS)))));
    assert!(off.contains(&(FormId(HAT), None)));
    let text = world::save::save(&state, None);
    let (loaded, _) = world::save::load(&text).unwrap();
    assert_eq!(loaded.taken_off.get(&who), Some(&off));
}
