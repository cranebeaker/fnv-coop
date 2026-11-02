//! Putting people together from their records: race parts, clothes that
//! hide what they cover, hair, eyes and head parts, and templates.

use esm::{FormId, LoadOrder, Plugin};
use testdata::{group, record, sub, zstr};
use world::actor_look;

fn race() -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr("TestRace"));
    // Head parts (male): head with its skin, an eye.
    d.extend(sub(b"NAM0", &[]));
    d.extend(sub(b"MNAM", &[]));
    d.extend(sub(b"INDX", &0u32.to_le_bytes()));
    d.extend(sub(b"MODL", &zstr("Characters\\Head\\Head.nif")));
    d.extend(sub(b"ICON", &zstr("Characters\\Male\\HeadSkin.dds")));
    d.extend(sub(b"INDX", &6u32.to_le_bytes()));
    d.extend(sub(b"MODL", &zstr("Characters\\Head\\EyeLeft.nif")));
    d.extend(sub(b"FNAM", &[]));
    d.extend(sub(b"INDX", &0u32.to_le_bytes()));
    d.extend(sub(b"MODL", &zstr("Characters\\Head\\HeadFemale.nif")));
    // Body parts (male): upper body and hands, with skins.
    d.extend(sub(b"NAM1", &[]));
    d.extend(sub(b"MNAM", &[]));
    for (i, model, skin) in [
        (
            0u32,
            "characters\\_male\\UpperBody.nif",
            "Characters\\Male\\Body.dds",
        ),
        (
            1,
            "characters\\_male\\LeftHand.nif",
            "Characters\\Male\\Hand.dds",
        ),
        (
            2,
            "characters\\_male\\RightHand.nif",
            "Characters\\Male\\Hand.dds",
        ),
    ] {
        d.extend(sub(b"INDX", &i.to_le_bytes()));
        d.extend(sub(b"ICON", &zstr(skin)));
        d.extend(sub(b"MODL", &zstr(model)));
    }
    d.extend(sub(b"FNAM", &[]));
    d.extend(sub(b"HNAM", &0x910u32.to_le_bytes()));
    // The race's own faces, after its hair list: men's, then women's.
    d.extend(sub(b"MNAM", &[]));
    d.extend(sub(b"FGGS", &testdata::f32s(&[1.0, 0.25, 0.5])));
    d.extend(sub(b"FGGA", &testdata::f32s(&[-0.5])));
    d.extend(sub(b"FNAM", &[]));
    d.extend(sub(b"FGGS", &testdata::f32s(&[3.0, 3.0, 3.0])));
    record(b"RACE", 0x900, &d)
}

fn npc(id: u32, editor_id: &str, extra: &[u8], acbs_template_flags: u16) -> Vec<u8> {
    let mut d = sub(b"EDID", &zstr(editor_id));
    d.extend(sub(b"MODL", &zstr("Characters\\_Male\\Skeleton.NIF")));
    let mut acbs = vec![0u8; 24];
    acbs[22..24].copy_from_slice(&acbs_template_flags.to_le_bytes());
    d.extend(sub(b"ACBS", &acbs));
    d.extend(extra);
    record(b"NPC_", id, &d)
}

fn order() -> LoadOrder {
    let mut hedr = 1.34f32.to_le_bytes().to_vec();
    hedr.extend([0; 8]);
    let mut plugin = record(b"TES4", 0, &sub(b"HEDR", &hedr));
    plugin.extend(group(*b"RACE", 0, &race()));

    // An outfit covering the upper body, and a hat.
    let armor_with = |id: u32, name: &str, slots: u32, model: &str, extra: &[u8]| {
        let mut d = sub(b"EDID", &zstr(name));
        let mut bmdt = slots.to_le_bytes().to_vec();
        bmdt.extend([0u8; 4]);
        d.extend(sub(b"BMDT", &bmdt));
        d.extend(sub(b"MODL", &zstr(model)));
        d.extend(extra);
        record(b"ARMO", id, &d)
    };
    let armor =
        |id: u32, name: &str, slots: u32, model: &str| armor_with(id, name, slots, model, &[]);
    let mut armors = armor(0x920, "Outfit", 0x04, "Armor\\Outfit.nif");
    armors.extend(armor(0x921, "Hat", 0x400, "Armor\\Hat.nif"));
    // A second outfit for the same slot: only the first is worn.
    armors.extend(armor(0x922, "Outfit2", 0x04, "Armor\\Outfit2.nif"));
    // A jacket worn with a right glove: its biped model list (`BIPL`, a
    // form list) names the glove (an armour addon).
    armors.extend(armor_with(
        0x924,
        "Jacket",
        0x04,
        "Armor\\Jacket.nif",
        &sub(b"BIPL", &0x926u32.to_le_bytes()),
    ));
    plugin.extend(group(*b"ARMO", 0, &armors));
    let mut glove = sub(b"EDID", &zstr("GloveR"));
    let mut bmdt = 0x10u32.to_le_bytes().to_vec();
    bmdt.extend([0u8; 4]);
    glove.extend(sub(b"BMDT", &bmdt));
    glove.extend(sub(b"MODL", &zstr("Armor\\GloveR.nif")));
    plugin.extend(group(*b"ARMA", 0, &record(b"ARMA", 0x925, &glove)));
    let mut list = sub(b"EDID", &zstr("JacketList"));
    list.extend(sub(b"LNAM", &0x925u32.to_le_bytes()));
    plugin.extend(group(*b"FLST", 0, &record(b"FLST", 0x926, &list)));

    let mut hair = sub(b"EDID", &zstr("Hair"));
    hair.extend(sub(b"MODL", &zstr("Characters\\Hair\\Hair.nif")));
    hair.extend(sub(b"ICON", &zstr("Characters\\Hair\\Hair.dds")));
    plugin.extend(group(*b"HAIR", 0, &record(b"HAIR", 0x910, &hair)));
    let mut eyes = sub(b"EDID", &zstr("Eyes"));
    eyes.extend(sub(b"ICON", &zstr("Characters\\Eyes\\Blue.dds")));
    plugin.extend(group(*b"EYES", 0, &record(b"EYES", 0x930, &eyes)));
    let mut beard = sub(b"EDID", &zstr("Beard"));
    beard.extend(sub(b"MODL", &zstr("Characters\\Hair\\Beard.nif")));
    plugin.extend(group(*b"HDPT", 0, &record(b"HDPT", 0x940, &beard)));

    let item = |id: u32| {
        let mut c = id.to_le_bytes().to_vec();
        c.extend(1u32.to_le_bytes());
        sub(b"CNTO", &c)
    };
    let mut traits = sub(b"RNAM", &0x900u32.to_le_bytes());
    traits.extend(sub(b"HNAM", &0x910u32.to_le_bytes()));
    traits.extend(sub(b"ENAM", &0x930u32.to_le_bytes()));
    traits.extend(sub(b"PNAM", &0x940u32.to_le_bytes()));
    traits.extend(sub(b"HCLR", &[200, 100, 50, 0]));
    traits.extend(sub(b"NAM6", &1.1f32.to_le_bytes()));
    traits.extend(sub(b"FGGS", &testdata::f32s(&[0.5, -1.0])));
    traits.extend(sub(b"FGGA", &testdata::f32s(&[2.0])));
    let mut dressed = traits.clone();
    dressed.extend(item(0x920));
    dressed.extend(item(0x922));
    let mut hatted = traits.clone();
    hatted.extend(item(0x921));
    let mut npcs = npc(0xA00, "Dressed", &dressed, 0);
    npcs.extend(npc(0xA01, "Hatted", &hatted, 0));
    // Takes its traits from Dressed, and has no clothes of its own.
    let mut templated = sub(b"TPLT", &0xA00u32.to_le_bytes());
    templated.extend(sub(b"RNAM", &0x999u32.to_le_bytes()));
    npcs.extend(npc(0xA02, "Templated", &templated, 0x01));
    // Weapons: a knife (melee, damage 20), a pistol (damage 10, takes the
    // rounds) and a rifle (damage 30, takes rounds nobody has). "Armed"
    // carries the knife and a "use all" list of the pistol and 12 rounds;
    // "Unloaded" the knife and the rifle.
    let weapon = |id: u32, name: &str, model: &str, damage: i16, animation: u32, ammo: u32| {
        let mut d = sub(b"EDID", &zstr(name));
        d.extend(sub(b"MODL", &zstr(model)));
        let mut data = vec![0u8; 12];
        data.extend(damage.to_le_bytes());
        data.push(6);
        d.extend(sub(b"DATA", &data));
        if ammo != 0 {
            d.extend(sub(b"NAM0", &ammo.to_le_bytes()));
        }
        let mut dnam = animation.to_le_bytes().to_vec();
        dnam.resize(42, 0);
        dnam[41] = 32;
        d.extend(sub(b"DNAM", &dnam));
        record(b"WEAP", id, &d)
    };
    let mut weapons = weapon(0x950, "Knife", "Weapons\\Knife.nif", 20, 1, 0);
    weapons.extend(weapon(0x951, "Pistol", "Weapons\\Pistol.nif", 10, 3, 0x960));
    weapons.extend(weapon(0x952, "Rifle", "Weapons\\Rifle.nif", 30, 5, 0x961));
    plugin.extend(group(*b"WEAP", 0, &weapons));
    let mut ammo = record(b"AMMO", 0x960, &sub(b"EDID", &zstr("Rounds")));
    ammo.extend(record(b"AMMO", 0x961, &sub(b"EDID", &zstr("OtherRounds"))));
    plugin.extend(group(*b"AMMO", 0, &ammo));
    let mut list = sub(b"EDID", &zstr("WithAmmoPistol"));
    list.extend(sub(b"LVLF", &[0x04]));
    for (form, count) in [(0x951u32, 1u16), (0x960, 12)] {
        let mut entry = 1u16.to_le_bytes().to_vec();
        entry.extend([0; 2]);
        entry.extend(form.to_le_bytes());
        entry.extend(count.to_le_bytes());
        entry.extend([0; 2]);
        list.extend(sub(b"LVLO", &entry));
    }
    plugin.extend(group(*b"LVLI", 0, &record(b"LVLI", 0x970, &list)));
    let mut armed = traits.clone();
    armed.extend(item(0x950));
    armed.extend(item(0x970));
    npcs.extend(npc(0xA03, "Armed", &armed, 0));
    let mut unloaded = traits.clone();
    unloaded.extend(item(0x950));
    unloaded.extend(item(0x952));
    npcs.extend(npc(0xA04, "Unloaded", &unloaded, 0));
    let mut gloved = traits.clone();
    gloved.extend(item(0x924));
    npcs.extend(npc(0xA05, "Gloved", &gloved, 0));
    plugin.extend(group(*b"NPC_", 0, &npcs));

    LoadOrder::single("Test.esm", None, Plugin::from_bytes(plugin).unwrap()).unwrap()
}

fn models(look: &world::ActorLook) -> Vec<&str> {
    look.parts.iter().map(|p| p.model.as_str()).collect()
}

#[test]
fn clothes_hide_the_body_they_cover_and_take_the_race_skin() {
    let order = order();
    let look = actor_look(&order, FormId(0xA00)).unwrap();
    assert!(!look.creature && !look.female);
    assert_eq!(look.idle, "Characters\\_Male\\locomotion\\mtidle.kf");
    assert_eq!(look.scale, 1.1);
    let m = models(&look);
    // The outfit replaces the upper body; the second outfit for the same
    // slot isn't worn; the hands stay.
    assert!(m.contains(&"Armor\\Outfit.nif"));
    assert!(!m.contains(&"Armor\\Outfit2.nif"));
    assert!(!m.contains(&"characters\\_male\\UpperBody.nif"));
    assert!(m.contains(&"characters\\_male\\LeftHand.nif"));
    let outfit = &look.parts[m.iter().position(|p| *p == "Armor\\Outfit.nif").unwrap()];
    assert_eq!(
        outfit.skin_texture.as_deref(),
        Some("Characters\\Male\\Body.dds")
    );
    // The head takes its skin; the eye the eye colour.
    let head = &look.parts[m
        .iter()
        .position(|p| *p == "Characters\\Head\\Head.nif")
        .unwrap()];
    assert_eq!(
        head.skin_texture.as_deref(),
        Some("Characters\\Male\\HeadSkin.dds")
    );
    let eye = &look.parts[m
        .iter()
        .position(|p| *p == "Characters\\Head\\EyeLeft.nif")
        .unwrap()];
    assert_eq!(eye.texture.as_deref(), Some("Characters\\Eyes\\Blue.dds"));
    // Hair: its texture, its colour, and the version without a hat.
    let hair = &look.parts[m
        .iter()
        .position(|p| *p == "Characters\\Hair\\Hair.nif")
        .unwrap()];
    assert_eq!(hair.texture.as_deref(), Some("Characters\\Hair\\Hair.dds"));
    assert_eq!(hair.hair_tint, Some([200, 100, 50]));
    assert_eq!(hair.hide_mesh.as_deref(), Some("Hat"));
    let beard = &look.parts[m
        .iter()
        .position(|p| *p == "Characters\\Hair\\Beard.nif")
        .unwrap()];
    assert_eq!(beard.hair_tint, Some([200, 100, 50]));
}

#[test]
fn people_carry_their_best_loaded_weapon_put_away_by_its_kind() {
    let order = order();
    // A "use all" list gives the pistol and its rounds: loaded and ranged,
    // it beats the stronger knife.
    let look = actor_look(&order, FormId(0xA03)).unwrap();
    let weapon = look.fighting.as_ref().unwrap();
    assert_eq!(weapon.weapon, Some(FormId(0x951)));
    let held = look.parts.iter().find(|p| p.bone.is_some()).unwrap();
    assert_eq!(
        (held.model.as_str(), held.bone.as_deref()),
        ("Weapons\\Pistol.nif", Some("Weapon"))
    );
    // Its kind's animations beside the skeleton.
    assert_eq!(
        weapon.holster.as_deref(),
        Some("Characters\\_Male\\1hpholster.kf")
    );
    assert_eq!(weapon.aim, "Characters\\_Male\\1hpaim.kf");
    assert_eq!(
        weapon.run,
        "Characters\\_Male\\locomotion\\1hpfastforward.kf"
    );
    assert_eq!(weapon.attack, "Characters\\_Male\\1hpattackright.kf");
    assert_eq!(
        world::actor::record_inventory(&order, FormId(0xA03)),
        vec![(FormId(0x950), 1), (FormId(0x951), 1), (FormId(0x960), 12)]
    );
    // A rifle with no rounds loses to the knife.
    let look = actor_look(&order, FormId(0xA04)).unwrap();
    assert_eq!(look.fighting.unwrap().weapon, Some(FormId(0x950)));
    // No weapon: fists, with nothing held or put away.
    let look = actor_look(&order, FormId(0xA00)).unwrap();
    let fists = look.fighting.unwrap();
    assert_eq!((fists.weapon, fists.holster), (None, None));
    assert_eq!(fists.attack, "Characters\\_Male\\h2hattackright.kf");
    assert!(look.parts.iter().all(|p| p.bone.is_none()));
}

#[test]
fn a_hat_switches_the_hair_to_its_hat_version() {
    let order = order();
    let look = actor_look(&order, FormId(0xA01)).unwrap();
    let m = models(&look);
    assert!(m.contains(&"Armor\\Hat.nif"));
    // Nothing covers the upper body here.
    assert!(m.contains(&"characters\\_male\\UpperBody.nif"));
    let hair = &look.parts[m
        .iter()
        .position(|p| *p == "Characters\\Hair\\Hair.nif")
        .unwrap()];
    assert_eq!(hair.hide_mesh.as_deref(), Some("NoHat"));
}

#[test]
fn templates_can_supply_the_traits() {
    let order = order();
    let look = actor_look(&order, FormId(0xA02)).unwrap();
    let m = models(&look);
    // The template's race (its own names a race that doesn't exist).
    assert!(m.contains(&"Characters\\Head\\Head.nif"));
    // Its inventory is its own (none): the bare upper body shows.
    assert!(m.contains(&"characters\\_male\\UpperBody.nif"));
    assert_eq!(look.base, FormId(0xA02));
}

#[test]
fn a_face_is_the_races_face_plus_the_npcs_own() {
    let order = order();
    let look = actor_look(&order, FormId(0xA00)).unwrap();
    let face = look.face.unwrap();
    // Men's race values (1, 0.25, 0.5) plus the NPC's (0.5, -1): the NPC's
    // list is shorter, so the race's third value stands alone.
    assert_eq!(face.symmetric, vec![1.5, -0.75, 0.5]);
    assert_eq!(face.asymmetric, vec![1.5]);
}
#[test]
fn bodies_and_hands_carry_the_npcs_body_tint() {
    let order = order();
    let look = actor_look(&order, FormId(0xA00)).unwrap();
    let tint = |model: &str| {
        look.parts
            .iter()
            .find(|p| p.model == model)
            .unwrap()
            .face_tint
            .clone()
    };
    let body =
        Some("textures\\characters\\bodymods\\test.esm\\00000a00modbodymale.dds".to_string());
    assert_eq!(tint("Armor\\Outfit.nif"), body);
    assert_eq!(tint("characters\\_male\\LeftHand.nif"), body);
    // The head keeps its face tint; hair and eyes have none.
    assert_eq!(
        tint("Characters\\Head\\Head.nif").as_deref(),
        Some("textures\\characters\\facemods\\test.esm\\00000a00_0.dds")
    );
    assert_eq!(tint("Characters\\Hair\\Hair.nif"), None);
}
#[test]
fn armour_addons_are_worn_with_their_armour_and_cover_their_slots() {
    let order = order();
    let look = actor_look(&order, FormId(0xA05)).unwrap();
    let m = models(&look);
    assert!(m.contains(&"Armor\\Jacket.nif"));
    // The glove on the right hand replaces the bare right hand; the left
    // hand stays.
    assert!(m.contains(&"Armor\\GloveR.nif"));
    assert!(!m.contains(&"characters\\_male\\RightHand.nif"));
    assert!(m.contains(&"characters\\_male\\LeftHand.nif"));
    // Its bare fingers take the race's hand skin, the jacket the body's.
    let skin = |model: &str| {
        look.parts
            .iter()
            .find(|p| p.model == model)
            .unwrap()
            .skin_texture
            .clone()
    };
    assert_eq!(
        skin("Armor\\GloveR.nif").as_deref(),
        Some("Characters\\Male\\Hand.dds")
    );
    assert_eq!(
        skin("Armor\\Jacket.nif").as_deref(),
        Some("Characters\\Male\\Body.dds")
    );
}
