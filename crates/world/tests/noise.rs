//! An attack's noise (`008ba600`, `00525620`, `005fbeb0`, `008bc240`,
//! `00886360`) and detection hearing it (`00472380`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::combat::Weapon;
use world::dialogue::PLAYER_REF;
use world::noise;
use world::scripting::GameState;

#[test]
fn attacks_are_heard_for_a_while() {
    let data = testdata::quests("noise");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let mut state = GameState::new(&order);
    let weapon = |id| Weapon::load(&order, FormId(id)).unwrap();
    let (loud, knife, pistol) = (weapon(LOUD_GUN), weapon(KNIFE), weapon(PISTOL));
    // Guns: their own level (`VNAM`; none is normal).
    assert_eq!(noise::attack_value(&order, &state, PLAYER_REF, &loud), 100);
    assert_eq!(noise::attack_value(&order, &state, PLAYER_REF, &pistol), 50);
    // Melee: a person's is silent, a creature's its own (`NAM5`).
    assert_eq!(noise::attack_value(&order, &state, PLAYER_REF, &knife), 0);
    assert_eq!(
        noise::attack_value(&order, &state, FormId(GECKO_REF), &knife),
        100
    );
    // A silencer makes the gun silent.
    state.items.insert((PLAYER_REF, FormId(LOUD_GUN)), 1);
    state.items.insert((PLAYER_REF, FormId(SILENCER)), 1);
    assert!(world::weapon_mods::attach(
        &order,
        &mut state,
        PLAYER_REF,
        FormId(LOUD_GUN),
        FormId(SILENCER)
    ));
    assert_eq!(noise::attack_value(&order, &state, PLAYER_REF, &loud), 0);
    // Heard for `fActorAlertSoundTimer` (5) seconds.
    noise::attacked(&order, &mut state, FormId(GECKO_REF), &knife);
    assert_eq!(noise::value(&state, FormId(GECKO_REF)), 100);
    noise::update(&mut state, 4.0);
    noise::update(&mut state, 1.0);
    assert_eq!(noise::value(&state, FormId(GECKO_REF)), 100);
    noise::update(&mut state, 0.1);
    assert_eq!(noise::value(&state, FormId(GECKO_REF)), 0);
}
