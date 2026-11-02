//! The combat AI read from a plugin built from scratch
//! (`testdata::fighting`): combat styles from their records, projectiles'
//! reach and gunmen's bands, the new weapon fields, who starts a fight on
//! noticing whom, allies helping, the weak running, melee skill and reach.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fighting::ids::*;
use world::combat::{self, Weapon};
use world::combat_ai::{self, CombatStyle, SettingCache};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// The player standing in the test cell.
fn state_with_player(order: &LoadOrder, at: [f32; 3]) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some(at);
    state
}

#[test]
fn combat_styles_come_from_the_record_or_the_default() {
    let (_data, order) = order("cai-styles");
    let gecko = CombatStyle::of(&order, FormId(GECKO_REF));
    assert_eq!(gecko.form_id, Some(FormId(GECKO_STYLE)));
    assert_eq!((gecko.attack_chance, gecko.block_chance), (60, 40));
    assert_eq!(gecko.hold_timer, (0.1, 0.35));
    assert_eq!((gecko.range_mult_min, gecko.range_mult_max), (0.5, 2.0));
    assert_eq!(gecko.semi_auto_delay_mult, (1.0, 2.0));
    // No ZNAM: the engine's default style (form 0000003D).
    let guard = CombatStyle::of(&order, FormId(GUARD_REF));
    assert_eq!(guard.form_id, Some(combat_ai::DEFAULT_STYLE));
    assert_eq!((guard.attack_chance, guard.hold_timer), (40, (0.5, 1.5)));
    // A record older than form version 12: both range multipliers 1.
    let old = CombatStyle::load(&order, FormId(OLD_STYLE)).unwrap();
    assert_eq!((old.range_mult_min, old.range_mult_max), (1.0, 1.0));
    assert_eq!(old.attack_chance, 60);
}

#[test]
fn weapons_and_projectiles_give_the_bands() {
    let (_data, order) = order("cai-bands");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let pistol = Weapon::load(&order, FormId(PISTOL)).unwrap();
    assert_eq!(pistol.semi_auto_delay, (0.0, 0.3));
    assert!(!pistol.is_automatic());
    let smg = Weapon::load(&order, FormId(SMG)).unwrap();
    assert!(smg.is_automatic());
    assert_eq!(smg.fire_rate, 11.0);
    // A hitscan bullet reaches its range; a lobbed grenade speed² ÷ g.
    assert_eq!(
        combat::projectile_reach(&order, FormId(BULLET)),
        Some(10000.0)
    );
    let lobbed = combat::projectile_reach(&order, FormId(GRENADE)).unwrap();
    assert!((lobbed - 1_000_000.0 / 686.61).abs() < 0.1, "{lobbed}");
    // The pistol with the default style and the data's absolute range
    // multiplier 4: 256 / 768 / 3072.
    let default = CombatStyle::of(&order, FormId(GUARD_REF));
    let reach = pistol
        .projectile
        .and_then(|p| combat::projectile_reach(&order, p));
    let band = combat_ai::ranged_band(Some((&pistol, reach)), &default, &s);
    assert_eq!(
        (band.min, band.optimal, band.absolute_max),
        (256.0, 768.0, 3072.0)
    );
    // The launcher's range is fixed (the gecko style's multipliers don't
    // apply) and its grenade caps it.
    let launcher = Weapon::load(&order, FormId(LAUNCHER)).unwrap();
    let gecko = CombatStyle::of(&order, FormId(GECKO_REF));
    let reach = launcher
        .projectile
        .and_then(|p| combat::projectile_reach(&order, p));
    let band = combat_ai::ranged_band(Some((&launcher, reach)), &gecko, &s);
    assert!((band.optimal - lobbed * 0.85).abs() < 0.1, "{band:?}");
    assert!((band.absolute_max - lobbed).abs() < 0.1);
    assert_eq!(band.min, 0.0);
}

#[test]
fn noticing_starts_fights_by_aggression_and_reaction() {
    let (_data, order) = order("cai-notice");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let (gecko, guard, raider, town) = (
        FormId(GECKO_REF),
        FormId(GUARD_REF),
        FormId(RAIDER_REF),
        FormId(TOWN_REF),
    );
    // The gecko (very aggressive) attacks the neutral player once it
    // notices them, above −20; not at −20.
    assert!(combat_ai::starts_combat(
        &order, &state, gecko, PLAYER_REF, -19, &s
    ));
    assert!(!combat_ai::starts_combat(
        &order, &state, gecko, PLAYER_REF, -20, &s
    ));
    // The guard (aggressive) doesn't attack a neutral player; the raider,
    // whose faction is the player's enemy, does, and the town's too.
    assert!(!combat_ai::starts_combat(
        &order, &state, guard, PLAYER_REF, 50, &s
    ));
    assert!(combat_ai::starts_combat(
        &order, &state, raider, PLAYER_REF, 5, &s
    ));
    assert!(combat_ai::starts_combat(
        &order, &state, raider, town, 5, &s
    ));
    // Fifteen already on the player: only a seen one joins.
    let mut crowded = state_with_player(&order, [0.0, 1500.0, 0.0]);
    for i in 0..15 {
        crowded.combat.insert(FormId(0x5000 + i), PLAYER_REF);
    }
    assert!(!combat_ai::starts_combat(
        &order, &crowded, gecko, PLAYER_REF, -5, &s
    ));
    assert!(combat_ai::starts_combat(
        &order, &crowded, gecko, PLAYER_REF, 1, &s
    ));
}

#[test]
fn allies_help_by_their_assistance() {
    let (_data, order) = order("cai-assist");
    let mut state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let (gecko, guard, raider, town) = (
        FormId(GECKO_REF),
        FormId(GUARD_REF),
        FormId(RAIDER_REF),
        FormId(TOWN_REF),
    );
    state.combat.insert(raider, town);
    state.combat.insert(town, raider);
    // The guard (helps allies) sees the townsperson, an ally, fighting the
    // raider, and detects the raider: it joins against the raider.
    assert_eq!(
        combat_ai::assists_against(&order, &state, guard, town, 10, |_| Some(5)),
        Some(raider)
    );
    // Not without seeing the friend, or detecting the enemy at 1.
    assert_eq!(
        combat_ai::assists_against(&order, &state, guard, town, 0, |_| Some(5)),
        None
    );
    assert_eq!(
        combat_ai::assists_against(&order, &state, guard, town, 10, |_| Some(0)),
        None
    );
    // The gecko isn't the town's friend; the townsperson helps nobody.
    assert_eq!(
        combat_ai::assists_against(&order, &state, gecko, town, 10, |_| Some(5)),
        None
    );
    assert_eq!(combat_ai::assistance(&order, &state, guard), 1);
    assert_eq!(combat_ai::assistance(&order, &state, gecko), 2);
    // A script's value wins.
    state.actor_values.insert((guard, 57), 0.0);
    assert_eq!(
        combat_ai::assists_against(&order, &state, guard, town, 10, |_| Some(5)),
        None
    );
}

#[test]
fn the_weak_and_unaggressive_run() {
    let (_data, order) = order("cai-flee");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let (gecko, guard, raider, town) = (
        FormId(GECKO_REF),
        FormId(GUARD_REF),
        FormId(RAIDER_REF),
        FormId(TOWN_REF),
    );
    // Fists (1.5 × 1.57 a second) against a machete (10 a second), both
    // with 100 health: 0.24 of the raider's strength, under the cautious
    // 0.375.
    assert!(combat_ai::flees_on_sight(&order, &state, town, raider, &s));
    // The gecko bites for 5 with 20 health: the townsperson is stronger.
    assert!(!combat_ai::flees_on_sight(&order, &state, town, gecko, &s));
    // The guard isn't unaggressive; the raider doesn't threaten the guard
    // (neutral, aggressive only to enemies).
    assert!(!combat_ai::flees_on_sight(
        &order, &state, guard, raider, &s
    ));
}

#[test]
fn melee_skill_and_reach_come_from_the_records() {
    let (_data, order) = order("cai-melee");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let gecko = FormId(GECKO_REF);
    assert_eq!(combat_ai::melee_skill(&order, &state, gecko, None), 40.0);
    assert_eq!(combat::creature_reach(&order, gecko), Some((25.0, 1)));
    let reach = combat_ai::melee_reach(None, combat::creature_reach(&order, gecko), 1.0, &s);
    assert_eq!(reach, 25.0);
    let giant = combat::creature_reach(&order, FormId(GIANT_REF));
    assert_eq!(combat_ai::melee_reach(None, giant, 1.0, &s), 60.0);
    // The raider's machete: Melee Weapons skill (0 on the record), reach
    // 0.5 × 128.
    let machete = combat::weapon_in_hand(&order, &state, FormId(RAIDER_REF)).unwrap();
    assert_eq!(machete.form_id, FormId(MACHETE));
    assert_eq!(combat_ai::melee_reach(Some(&machete), None, 1.0, &s), 64.0);
    assert_eq!(
        combat_ai::melee_skill(&order, &state, FormId(RAIDER_REF), Some(&machete)),
        0.0
    );
}

#[test]
fn creatures_made_from_templates_fight_with_the_templates_values() {
    let (_data, order) = order("cai-template");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let spawned = FormId(SPAWNED_REF);
    // Its own record has zeros and a reach of 32: everything comes from the
    // gecko behind the leveled list.
    assert_eq!(combat::creature_damage(&order, spawned), Some(5.0));
    assert_eq!(combat::max_health(&order, &state, spawned), Some(20.0));
    assert_eq!(combat::creature_reach(&order, spawned), Some((25.0, 1)));
    assert_eq!(combat_ai::melee_skill(&order, &state, spawned, None), 40.0);
    assert_eq!(combat_ai::aggression(&order, &state, spawned), 2);
    assert_eq!(combat_ai::assistance(&order, &state, spawned), 2);
    let facts = Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    // Perception 3 and Assistance as actor values too.
    assert_eq!(facts.base_actor_value(spawned, 6), Some(3.0));
    assert_eq!(facts.base_actor_value(spawned, 57), Some(2.0));
    assert_eq!(
        CombatStyle::of(&order, spawned).form_id,
        Some(FormId(GECKO_STYLE))
    );
    assert!(combat_ai::starts_combat(
        &order, &state, spawned, PLAYER_REF, -10, &s
    ));
}

#[test]
fn detection_takes_the_line_of_sight_and_movement_given() {
    let (_data, order) = order("cai-detect");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    // The player 500 north of the gecko (which faces north).
    let mut state = state_with_player(&order, [0.0, 1500.0, 0.0]);
    let facts = Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    let gecko = FormId(GECKO_REF);
    let seen = combat_ai::detection_value(&facts, gecko, PLAYER_REF, true, None, &s).unwrap();
    let hidden = combat_ai::detection_value(&facts, gecko, PLAYER_REF, false, None, &s).unwrap();
    assert!(seen > hidden, "{seen} {hidden}");
    // Someone walking is easier to hear than someone standing still.
    let raider = FormId(RAIDER_REF);
    let still = combat_ai::detection_value(&facts, gecko, raider, false, None, &s).unwrap();
    let walking =
        combat_ai::detection_value(&facts, gecko, raider, false, Some((true, false)), &s).unwrap();
    assert!(walking > still, "{walking} {still}");
    // Next to each other: 100.
    state.positions.insert(raider, ([0.0, 1001.0, 0.0], 0.0));
    let facts = Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    assert_eq!(
        combat_ai::detection_value(&facts, gecko, raider, false, None, &s),
        Some(100)
    );
}
