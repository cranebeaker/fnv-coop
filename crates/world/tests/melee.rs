//! Melee and unarmed fighting (`docs/MELEE_UNARMED.md`): reach by the
//! records, V.A.T.S. specials and their spells, Super Slam's knockdowns,
//! fists' power attacks, the unarmed cross and uppercut on the limbs, on
//! generated records carrying `FalloutNV.esm`'s values.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::melee::ids::*;
use world::body_parts::part;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::melee::{self, Blow, Special};
use world::scripting::{GameState, Runner, ScriptCache};
use world::vats;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::melee::melee(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn weapon(order: &LoadOrder, id: u32) -> Weapon {
    Weapon::load(order, FormId(id)).unwrap()
}

/// A player with Luck 0 (no criticals) and Unarmed and Melee Weapons 50.
fn player(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.actor_values.insert((PLAYER_REF, 11), 0.0);
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::UNARMED), 50.0);
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::MELEE_WEAPONS), 50.0);
    state
}

#[test]
fn reach_comes_from_the_weapon_hand_to_hand_ones_too() {
    let (_data, order) = order("melee-reach");
    let knuckles = weapon(&order, BRASS_KNUCKLES);
    let mantis = weapon(&order, MANTIS_GAUNTLET);
    let machete = weapon(&order, MACHETE);
    let sledge = weapon(&order, SUPER_SLEDGE);
    assert_eq!(knuckles.animation, 0);
    // `009a69c0`: the brass knuckles' reach 1 is 128 units, not the 64 of
    // bare hands; the mantis gauntlet's 1.2 is 153.6.
    assert_eq!(Weapon::melee_reach(Some(&knuckles)), 128.0);
    assert!((Weapon::melee_reach(Some(&mantis)) - 153.6).abs() < 1e-3);
    assert_eq!(Weapon::melee_reach(Some(&machete)), 64.0);
    assert_eq!(Weapon::melee_reach(Some(&sledge)), 128.0);
    assert_eq!(Weapon::melee_reach(None), 64.0);
    // `008990f0`: × scale, × 2 for the player in V.A.T.S.
    assert_eq!(
        melee::swing_reach(&order, Some(&knuckles), 1.0, false),
        128.0
    );
    assert_eq!(melee::swing_reach(&order, None, 1.0, true), 128.0);
    assert_eq!(melee::swing_reach(&order, Some(&machete), 1.5, false), 96.0);
}

#[test]
fn vats_specials_are_named_by_the_weapon_and_need_whole_skill_points() {
    let (_data, order) = order("melee-specials");
    let s = vats::Settings::load(&order);
    let mut state = player(&order);
    let sledge = weapon(&order, SUPER_SLEDGE);
    let names = |state: &GameState, w: Option<&Weapon>, down: bool| {
        vats::specials(&order, state, &s, w, down)
            .into_iter()
            .map(|sp| (sp.name, sp.cost))
            .collect::<Vec<_>>()
    };
    // The super sledge's Mauler: Melee Weapons 50 asked for, 48 points.
    assert_eq!(
        names(&state, Some(&sledge), false),
        vec![("Mauler".into(), 48.0)]
    );
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::MELEE_WEAPONS), 49.9);
    assert!(names(&state, Some(&sledge), false).is_empty());
    // The machete's Back Slash asks for nothing.
    let machete = weapon(&order, MACHETE);
    assert_eq!(
        names(&state, Some(&machete), false),
        vec![("Back Slash".into(), 16.0)]
    );
    // Unarmed 50 (at the threshold) gives Uppercut, 75 Cross too, and
    // Stomp instead against someone down.
    let uppercut = ("Uppercut".to_string(), 20.0);
    assert_eq!(names(&state, None, false), vec![uppercut.clone()]);
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::UNARMED), 74.9);
    assert_eq!(names(&state, None, false), vec![uppercut.clone()]);
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::UNARMED), 75.0);
    assert_eq!(
        names(&state, None, false),
        vec![uppercut, ("Cross".into(), 20.0)]
    );
    assert_eq!(names(&state, None, true), vec![("Stomp".into(), 20.0)]);
    // What they play.
    assert_eq!(
        vats::attack_group(vats::kind::SPECIAL, false),
        Some(melee::group::ATTACK_FORWARD_POWER)
    );
    assert_eq!(
        vats::attack_group(vats::kind::UPPERCUT, false),
        Some(melee::group::ATTACK6)
    );
    assert_eq!(vats::attack_group(vats::kind::UPPERCUT, true), None);
    assert_eq!(
        vats::attack_group(vats::kind::CROSS, false),
        Some(melee::group::ATTACK7)
    );
    assert_eq!(
        vats::attack_group(vats::kind::STOMP, false),
        Some(melee::group::STOMP)
    );
}

fn playing(state: &mut GameState, kind: u8, weapon: Option<u32>) {
    vats::begin_attack(
        state,
        vats::AttackFacts {
            weapon: weapon.map(FormId),
            target: FormId(PERSON_REF),
            part_av: 26,
            kind,
            hit: true,
            distance: 50.0,
            paralyzing_palm: false,
        },
    );
}

#[test]
fn the_maulers_hit_casts_its_knockdown_spell() {
    let (_data, order) = order("melee-mauler");
    let scripts = ScriptCache::default();
    let mut state = player(&order);
    let sledge = weapon(&order, SUPER_SLEDGE);
    let t = FormId(PERSON_REF);
    // An ordinary blow casts nothing.
    Runner::new(&order, &scripts, &mut state).hit_at(PLAYER_REF, t, Some(&sledge), None);
    assert!(!world::magic::is_target_of(&state, t, FormId(MAULER_SPELL)));
    // The queued special does, on the one struck.
    playing(&mut state, vats::kind::SPECIAL, Some(SUPER_SLEDGE));
    assert_eq!(
        vats::special_effect(&order, &state, PLAYER_REF, Some(&sledge)),
        Some(FormId(MAULER_SPELL))
    );
    Runner::new(&order, &scripts, &mut state).hit_at(PLAYER_REF, t, Some(&sledge), None);
    assert!(world::magic::is_target_of(&state, t, FormId(MAULER_SPELL)));
    world::magic::tick(&mut Runner::new(&order, &scripts, &mut state), 0.1);
    assert_eq!(state.globals.get(&FormId(KNOCKDOWNS)), Some(&1.0));
    // The machete's special has no spell; a plain V.A.T.S. attack none.
    let machete = weapon(&order, MACHETE);
    assert_eq!(
        vats::special_effect(&order, &state, PLAYER_REF, Some(&machete)),
        None
    );
    playing(&mut state, vats::kind::TWO_HAND_MELEE, Some(SUPER_SLEDGE));
    assert_eq!(
        vats::special_effect(&order, &state, PLAYER_REF, Some(&sledge)),
        None
    );
}

#[test]
fn super_slam_knocks_down_more_with_two_handed_weapons() {
    let (_data, order) = order("melee-super-slam");
    let mut state = player(&order);
    let chance = |state: &GameState, w: Option<u32>| {
        melee::knockdown_chance(&order, state, PLAYER_REF, w.map(FormId))
    };
    assert_eq!(chance(&state, None), 0.0);
    world::perks::add(&order, &mut state, FormId(SUPER_SLAM));
    // The perk's conditions ask about the weapon in the player's hands.
    assert!((chance(&state, None) - 0.15).abs() < 1e-6);
    state.equip(&order, PLAYER_REF, FormId(MACHETE));
    assert!((chance(&state, Some(MACHETE)) - 0.15).abs() < 1e-6);
    state.unequip(PLAYER_REF, FormId(MACHETE));
    state.equip(&order, PLAYER_REF, FormId(SUPER_SLEDGE));
    assert!((chance(&state, Some(SUPER_SLEDGE)) - 0.30).abs() < 1e-6);
}

#[test]
fn fists_power_attacks_multiply_after_the_armour() {
    let (_data, order) = order("melee-fists");
    let scripts = ScriptCache::default();
    let t = FormId(PERSON_REF);
    let dealt = |power: bool, w: Option<&Weapon>| {
        let mut state = player(&order);
        Runner::new(&order, &scripts, &mut state)
            .strike_at(PLAYER_REF, t, w, None, power)
            .unwrap()
            .dealt
    };
    // Fists, Unarmed 50: 1 + Unarmed Damage (0.5 + 0.05 × 50) = 4; a power
    // attack's × 2 comes after (8), not inside (1 × 2 + 3 = 5).
    assert!((dealt(false, None) - 4.0).abs() < 1e-4);
    assert!((dealt(true, None) - 8.0).abs() < 1e-4);
    // Brass knuckles: the power inside the weapon's damage, 18 × 2 + 3.
    let knuckles = weapon(&order, BRASS_KNUCKLES);
    assert!((dealt(false, Some(&knuckles)) - 21.0).abs() < 1e-4);
    assert!((dealt(true, Some(&knuckles)) - 39.0).abs() < 1e-4);
}

#[test]
fn a_cross_hurts_limbs_more_and_an_uppercut_disarms() {
    let (_data, order) = order("melee-limbs");
    let scripts = ScriptCache::default();
    let lost = |special: Special| {
        let mut state = player(&order);
        Runner::new(&order, &scripts, &mut state)
            .blow_at(
                PLAYER_REF,
                FormId(PERSON_REF),
                None,
                Some(part::RIGHT_ARM),
                Blow {
                    power: false,
                    special,
                },
            )
            .unwrap()
            .hurt
            .unwrap()
            .lost
    };
    let plain = lost(Special::None);
    assert!(plain > 0.0);
    assert!((lost(Special::Cross) - plain * 2.5).abs() < 1e-4);
    // The sledger holds the two-handed super sledge: a punch on the left
    // arm that doesn't cripple it leaves it in their hands; an uppercut
    // there makes them drop it.
    let blow = |special: Special| {
        let mut state = player(&order);
        let hit = Runner::new(&order, &scripts, &mut state)
            .blow_at(
                PLAYER_REF,
                FormId(SLEDGER_REF),
                None,
                Some(part::LEFT_ARM),
                Blow {
                    power: false,
                    special,
                },
            )
            .unwrap();
        hit.hurt.unwrap()
    };
    let punch = blow(Special::None);
    assert!(!punch.crippled && !punch.staggered && punch.dropped.is_none());
    let uppercut = blow(Special::Stagger);
    assert!(!uppercut.crippled && uppercut.staggered);
    assert_eq!(uppercut.dropped, Some(FormId(SUPER_SLEDGE)));
}
