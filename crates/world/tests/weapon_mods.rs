//! Weapon mods: their slots, fitting one (`00783af0`), and what they change
//! (`004fe160`, `00644ce0`, `004be380`, `004bd400`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;
use world::weapon_mods::{self, effect};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn fitting_mods_changes_the_gun() {
    let (_data, order) = order("weapon-mods");
    let mut state = GameState::new(&order);
    let gun = FormId(MOD_GUN);
    let slots = weapon_mods::slots(&order, gun).unwrap();
    assert_eq!(slots[0].item, Some(FormId(EXT_MAG)));
    assert_eq!((slots[1].effect, slots[1].value), (effect::DAMAGE, 5.0));
    state.items.insert((PLAYER_REF, gun), 1);
    state.equipped.entry(PLAYER_REF).or_default().push(gun);
    let base = world::combat::weapon_in_hand(&order, &state, PLAYER_REF).unwrap();
    assert_eq!(base.clip, 8);
    let damage = |state: &GameState| {
        let w = world::combat::weapon_in_hand(&order, state, PLAYER_REF);
        world::combat::weapon_damage(&order, state, PLAYER_REF, w.as_ref(), false)
    };
    let before = damage(&state);
    let weight_before = state.inventory_weight(&order, PLAYER_REF);
    let worth_before = world::barter::item_value(&order, &state, PLAYER_REF, gun);
    // None to fit yet.
    assert!(!weapon_mods::attach(
        &order,
        &mut state,
        PLAYER_REF,
        gun,
        FormId(EXT_MAG)
    ));
    for m in [EXT_MAG, BARREL, LIGHT_FRAME] {
        state.items.insert((PLAYER_REF, FormId(m)), 1);
    }
    assert_eq!(
        weapon_mods::fitting(&order, &state, PLAYER_REF, gun),
        vec![FormId(EXT_MAG), FormId(BARREL), FormId(LIGHT_FRAME)]
    );
    for m in [EXT_MAG, BARREL, LIGHT_FRAME] {
        assert!(weapon_mods::attach(
            &order,
            &mut state,
            PLAYER_REF,
            gun,
            FormId(m)
        ));
        assert_eq!(state.item_count(&order, PLAYER_REF, FormId(m)), 0);
    }
    assert_eq!(weapon_mods::flags(&state, PLAYER_REF, gun), 7);
    // Fitted once only.
    state.items.insert((PLAYER_REF, FormId(EXT_MAG)), 1);
    assert!(!weapon_mods::attach(
        &order,
        &mut state,
        PLAYER_REF,
        gun,
        FormId(EXT_MAG)
    ));
    assert_eq!(
        state.misc_stats.get(&weapon_mods::WEAPON_MODIFICATIONS),
        Some(&3)
    );
    // Clip 8 + 7; damage + 5 (before the skill's share and condition, as
    // the game adds it); weight 3 − 1; worth + 3 × 20.
    let modded = world::combat::weapon_in_hand(&order, &state, PLAYER_REF).unwrap();
    assert_eq!(modded.clip, 15);
    assert!(damage(&state) > before);
    let weight_after = state.inventory_weight(&order, PLAYER_REF);
    // The extra mag still carried weighs 0.5.
    assert!(
        (weight_before - (weight_after - 0.5) - 1.0).abs() < 1e-4,
        "{weight_before} {weight_after}"
    );
    let worth_after = world::barter::item_value(&order, &state, PLAYER_REF, gun);
    assert!(
        (worth_after - worth_before - 60.0).abs() < 1e-3,
        "{worth_before} {worth_after}"
    );
    // Kept in a save.
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert_eq!(weapon_mods::flags(&back, PLAYER_REF, gun), 7);
}

/// The models (`00522df0`, `004ab400`, `004ab500`; Xbox PDB
/// `GetModTESModel`, `Get1stPersonModObject`).
#[test]
fn modded_models() {
    let (_data, order) = order("weapon-mods-models");
    let gun = FormId(MOD_GUN);
    let model = |flags| weapon_mods::model(&order, gun, flags);
    let player = |flags| weapon_mods::player_model(&order, gun, flags);
    // Held by anyone: `MWD{flags}`, the plain model when that's missing.
    assert_eq!(model(0).as_deref(), Some(r"Weapons\ModGun.NIF"));
    assert_eq!(model(1).as_deref(), Some(r"Weapons\ModGunExt.NIF"));
    assert_eq!(model(2).as_deref(), Some(r"Weapons\ModGun.NIF"));
    assert_eq!(model(3).as_deref(), Some(r"Weapons\ModGunExtBarrel.NIF"));
    // The player: the first-person object for the flags, else `WNAM`'s.
    assert_eq!(player(0).as_deref(), Some(r"Weapons\1stModGun.NIF"));
    assert_eq!(player(1).as_deref(), Some(r"Weapons\1stModGunExt.NIF"));
    assert_eq!(player(3).as_deref(), Some(r"Weapons\1stModGun.NIF"));
    // A weapon without first-person objects: the same as anyone's.
    let pistol = FormId(PISTOL);
    assert_eq!(
        weapon_mods::player_model(&order, pistol, 0),
        weapon_mods::model(&order, pistol, 0)
    );
}

/// A split beam (`00525b20`, `00523150`, `006450f0`): more projectiles, the
/// cone × its second value, the shown damage × 1.3.
#[test]
fn split_beam() {
    let (_data, order) = order("weapon-mods-split");
    let mut state = GameState::new(&order);
    let gun = FormId(LOUD_GUN);
    state.items.insert((PLAYER_REF, gun), 1);
    state.items.insert((PLAYER_REF, FormId(BEAM_SPLITTER)), 1);
    let plain = world::combat::Weapon::load(&order, gun).unwrap();
    let (count, cone) = plain.shot(&order, None);
    assert!(cone > 0.0);
    assert_eq!(
        weapon_mods::shown_damage_mult(&order, &state, PLAYER_REF, gun),
        1.0
    );
    assert!(weapon_mods::attach(
        &order,
        &mut state,
        PLAYER_REF,
        gun,
        FormId(BEAM_SPLITTER)
    ));
    let split = weapon_mods::modded(&order, &state, PLAYER_REF, plain);
    let (more, narrower) = split.shot(&order, None);
    assert_eq!(more, count + 2);
    assert_eq!(narrower, cone * 0.5);
    assert_eq!(
        weapon_mods::shown_damage_mult(&order, &state, PLAYER_REF, gun),
        1.3
    );
}
