//! The perks' entry points, applied as the game's `005e58f0` applies them
//! (see `world::perks`): each test's perk is laid out as a real one
//! (`testdata::quests`: Cowboy, Stonewall, Piercing Strike, Laser
//! Commander, Better Criticals, Dream Crusher, Chemist, Lead Belly, Silent
//! Running, Travel Light, Pack Rat, Hand Loader, Rapid Reload, Long Haul,
//! Fast Shot, Built to Destroy, Adamantium Skeleton) and fails to apply
//! without the rule.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{Facts, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn run(order: &LoadOrder, state: &mut GameState, line: &str) {
    let scripts = ScriptCache::default();
    Runner::new(order, &scripts, state).run_source(line, None, None);
}

fn near(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

/// The player holding the pistol, and the gecko.
fn armed(order: &LoadOrder) -> (GameState, Weapon, FormId) {
    let mut state = GameState::new(order);
    state.items.insert((PLAYER_REF, FormId(PISTOL)), 1);
    run(order, &mut state, "player.EquipItem TestPistol");
    let pistol = combat::weapon_in_hand(order, &state, PLAYER_REF).unwrap();
    (state, pistol, FormId(GECKO_REF))
}

#[test]
fn weapon_damage_perks_multiply_what_gets_through_armour() {
    let (_data, order) = order("perk-damage");
    let (mut state, _pistol, gecko) = armed(&order);
    let through = |state: &GameState, weapon: Option<u32>| {
        combat::hit_through_armour(
            &order,
            state,
            9.2,
            Some((PLAYER_REF, weapon.map(FormId))),
            gecko,
            None,
        )
    };
    assert!(near(through(&state, Some(PISTOL)), 9.2));
    // Cowhand (as Cowboy): × 1.25 with the weapons in its list — the
    // pistol, not the fists (the weapon tab, `IsInList`).
    run(&order, &mut state, "player.AddPerk Cowhand");
    assert!(near(through(&state, Some(PISTOL)), 11.5));
    assert!(near(through(&state, None), 9.2));
    // Beast Slayer: × 1.5 against creatures (the target tab,
    // `GetIsCreature`), whatever the weapon.
    run(&order, &mut state, "player.AddPerk BeastSlayer");
    assert!(near(through(&state, Some(PISTOL)), 11.5 * 1.5));
    assert!(near(through(&state, None), 9.2 * 1.5));
    // The gecko holds no perks: its hits on the player are unchanged.
    let back =
        combat::hit_through_armour(&order, &state, 8.0, Some((gecko, None)), PLAYER_REF, None);
    assert!(near(back, 8.0));
    // Nobody's perks when the hit has no attacker.
    assert!(near(
        combat::through_armour(&order, &state, 9.2, gecko, None),
        9.2
    ));
}

#[test]
fn threshold_perks_come_after_the_ammunitions_effects() {
    let (_data, order) = order("perk-threshold");
    let (mut state, _pistol, gecko) = armed(&order);
    state.actor_values.insert((gecko, 76), 10.0);
    let through = |state: &GameState, weapon: Option<u32>| {
        combat::hit_through_armour(
            &order,
            state,
            20.0,
            Some((PLAYER_REF, weapon.map(FormId))),
            gecko,
            None,
        )
    };
    assert!(near(through(&state, Some(PISTOL)), 10.0));
    // Piercer (as Piercing Strike): 15 off the target's threshold with
    // melee and unarmed weapons; the fists count as unarmed, the pistol
    // doesn't.
    run(&order, &mut state, "player.AddPerk Piercer");
    assert!(near(through(&state, None), 20.0));
    assert!(near(through(&state, Some(PISTOL)), 10.0));
    // Bulwark (as Stonewall): + 5 to the player's threshold against melee
    // and unarmed attackers — the gecko's bite, not a gun.
    run(&order, &mut state, "player.AddPerk Bulwark");
    let on_player = |state: &GameState, weapon: Option<u32>, ammo: Option<u32>| {
        combat::hit_through_armour(
            &order,
            state,
            20.0,
            Some((gecko, weapon.map(FormId))),
            PLAYER_REF,
            ammo.map(FormId),
        )
    };
    assert!(near(on_player(&state, None, None), 15.0));
    assert!(near(on_player(&state, Some(PISTOL), None), 20.0));
    // The Pip-Boy's figure asks with the player's own weapon as the
    // attacker's: nothing while the pistol is carried (the weapon in
    // hand), + 5 with the fists.
    assert!(near(
        combat::damage_threshold(&order, &state, PLAYER_REF),
        0.0
    ));
    let mut unarmed = GameState::new(&order);
    unarmed.perks.insert(FormId(BULWARK));
    assert!(near(
        combat::damage_threshold(&order, &unarmed, PLAYER_REF),
        5.0
    ));
    // The ammunition's threshold effects come first, the perks after:
    // hollow points triple a threshold of 2 to 6, then + 5 = 11; what's
    // left × their 1.75.
    state.actor_values.insert((PLAYER_REF, 76), 2.0);
    assert!(near(
        on_player(&state, None, Some(HOLLOW_POINT)),
        (20.0 - 11.0) * 1.75
    ));
}

#[test]
fn critical_perks_change_the_chance_and_the_damage() {
    let (_data, order) = order("perk-crits");
    let (mut state, pistol, gecko) = armed(&order);
    state.actor_values.insert((PLAYER_REF, 11), 5.0);
    state.actor_values.insert((gecko, 11), 5.0);
    let crit = |state: &GameState, roll: u64| {
        combat::critical(&order, state, PLAYER_REF, Some(&pistol), gecko, false, roll)
    };
    // Luck 5: 5%, per mille.
    assert!(crit(&state, 49) && !crit(&state, 50));
    // Marksman (as Laser Commander): + 10 with the list's weapons.
    run(&order, &mut state, "player.AddPerk Marksman");
    assert!(crit(&state, 149) && !crit(&state, 150));
    // Dream Crusher, held by the one hit: the gecko's chance against the
    // player is halved (its own 5% → 2.5%).
    let against_player = |state: &GameState, roll: u64| {
        combat::critical(&order, state, gecko, None, PLAYER_REF, false, roll)
    };
    assert!(against_player(&state, 49) && !against_player(&state, 50));
    run(&order, &mut state, "player.AddPerk DreamCrusher");
    assert!(against_player(&state, 24) && !against_player(&state, 25));
    // Better Crits: the critical damage × 1.5 (the fists' is the hit
    // itself).
    assert!(near(
        combat::critical_damage(&order, &state, PLAYER_REF, None, gecko, 9.2),
        9.2
    ));
    run(&order, &mut state, "player.AddPerk BetterCrits");
    assert!(near(
        combat::critical_damage(&order, &state, PLAYER_REF, None, gecko, 9.2),
        13.8
    ));
}

#[test]
fn food_and_medicine_follow_the_skills_and_the_perks() {
    let (_data, order) = order("perk-food");
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    let skill = |state: &GameState, av: u16| {
        Facts {
            order: &order,
            state,
            speaker: None,
        }
        .current_actor_value(p, av)
        .unwrap_or(0.0) as f32
    };
    // Medicine 50 and Survival 25: 1 + skill ÷ 100 × 2.
    state.actor_values.insert((p, 37), 50.0);
    state.actor_values.insert((p, 44), 25.0);
    let medicine = 1.0 + skill(&state, 37) / 100.0 * 2.0;
    let survival = 1.0 + skill(&state, 44) / 100.0 * 2.0;
    assert!(near(medicine, 2.0) && near(survival, 1.5));
    // A medicine: every effect × the Medicine skill's factor.
    state.damage.insert(p, 100.0);
    state.items.insert((p, FormId(CHEM)), 3);
    world::items::use_item(&order, &mut state, p, FormId(CHEM)).unwrap();
    assert!(near(state.damage[&p] as f32, 100.0 - 30.0 * medicine));
    let strength = state
        .active_effects
        .iter()
        .find(|e| e.source == FormId(CHEM) && e.actor_value == 5)
        .unwrap();
    assert!(near(strength.magnitude, 2.0 * medicine));
    assert!(near(strength.remaining, 10.0));
    // Better Healing: the health × 1.2 as well, the Strength unchanged.
    run(&order, &mut state, "player.AddPerk BetterHealing");
    state.damage.insert(p, 100.0);
    state.active_effects.clear();
    world::items::use_item(&order, &mut state, p, FormId(CHEM)).unwrap();
    assert!(near(state.damage[&p] as f32, 100.0 - 30.0 * medicine * 1.2));
    // A food: the Survival skill's factor on every effect but radiation,
    // which Lead Belly halves; Chemist doubles the benefits' durations
    // but not a hostile effect's.
    run(&order, &mut state, "player.AddPerk LeadBelly");
    run(&order, &mut state, "player.AddPerk Chemist");
    state.active_effects.clear();
    state.items.insert((p, FormId(FOOD)), 1);
    world::items::use_item(&order, &mut state, p, FormId(FOOD)).unwrap();
    let effect = |state: &GameState, av: i32| {
        state
            .active_effects
            .iter()
            .find(|e| e.source == FormId(FOOD) && e.actor_value == av)
            .cloned()
            .unwrap()
    };
    let heal = effect(&state, 16);
    assert!(near(heal.magnitude, 3.0 * survival * 1.2));
    assert!(near(heal.remaining, 8.0));
    let hostile = effect(&state, 5);
    assert!(near(hostile.magnitude, 1.0 * survival));
    assert!(near(hostile.remaining, 10.0));
    // 10 rads × 0.5; at once, so counted already.
    assert!(near(skill(&state, 54), 5.0));
}

#[test]
fn silent_running_is_never_heard_running() {
    let (_data, order) = order("perk-silent");
    let mut state = GameState::new(&order);
    run(&order, &mut state, "player.MoveTo GeckoRef");
    state.player_moving = true;
    state.player_running = true;
    let inputs = |state: &GameState| {
        Facts {
            order: &order,
            state,
            speaker: None,
        }
        .detection_inputs(FormId(GECKO_REF), PLAYER_REF, true, None)
        .unwrap()
    };
    assert!(inputs(&state).running && inputs(&state).moving);
    run(&order, &mut state, "player.AddPerk SilentRunning");
    assert!(!inputs(&state).running && inputs(&state).moving);
    state.player_sneaking = true;
    assert!(!inputs(&state).running && !inputs(&state).moving);
}

#[test]
fn travel_light_pack_rat_and_long_haul() {
    let (_data, order) = order("perk-carry");
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    // Travel Light: × 1.1 unless the coat is worn (its holder condition).
    assert!(near(combat::movement_speed_mult(&order, &state, p), 1.0));
    run(&order, &mut state, "player.AddPerk TravelLight");
    assert!(near(combat::movement_speed_mult(&order, &state, p), 1.1));
    state.items.insert((p, FormId(COAT)), 1);
    run(&order, &mut state, "player.EquipItem TestCoat");
    assert!(near(combat::movement_speed_mult(&order, &state, p), 1.0));
    // Pack Rat: items of 2 or less weigh half (ten cups of 0.5; the
    // pistol's 1.5), heavier ones as before (the rifle's 6).
    state.items.insert((p, FormId(CUP)), 10);
    state.items.insert((p, FormId(PISTOL)), 1);
    state.items.insert((p, FormId(RIFLE)), 1);
    assert!(near(state.inventory_weight(&order, p), 5.0 + 1.5 + 6.0));
    run(&order, &mut state, "player.AddPerk PackRat");
    assert!(near(state.inventory_weight(&order, p), 2.5 + 0.75 + 6.0));
    // Long Haul: fast travel while carrying too much (a carry weight of 5
    // against 9.25).
    run(&order, &mut state, "player.MoveTo GeckoRef");
    state.player_world = Some(FormId(CELL));
    state.actor_values.insert((p, 13), 5.0);
    assert!(state.over_encumbered(&order, p));
    assert_eq!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some("You cannot fast travel while overencumbered.")
    );
    run(&order, &mut state, "player.AddPerk LongHaul");
    assert_ne!(
        world::map::travel_refused(&order, &state).as_deref(),
        Some("You cannot fast travel while overencumbered.")
    );
    state.actor_values.insert((p, 13), 50.0);
    assert!(!state.over_encumbered(&order, p));
}

#[test]
fn hand_loader_doubles_the_chance_of_a_case() {
    let (_data, order) = order("perk-cases");
    let (mut state, pistol, _gecko) = armed(&order);
    let case = |state: &GameState, roll: u64| {
        combat::ammo_item_recovered(&order, state, PLAYER_REF, &pistol, FormId(CASED_AMMO), roll)
    };
    // 25% from the round's own data: a roll of 0..100 at most the chance.
    assert_eq!(case(&state, 25), Some(FormId(CASE)));
    assert_eq!(case(&state, 26), None);
    run(&order, &mut state, "player.AddPerk HandLoader");
    assert_eq!(case(&state, 50), Some(FormId(CASE)));
    assert_eq!(case(&state, 51), None);
    // Only the player gets cases back.
    assert_eq!(
        combat::ammo_item_recovered(
            &order,
            &state,
            FormId(GECKO_REF),
            &pistol,
            FormId(CASED_AMMO),
            0
        ),
        None
    );
}

#[test]
fn reload_and_attack_rates_follow_agility_and_the_perks() {
    let (_data, order) = order("perk-rates");
    let (mut state, pistol, _gecko) = armed(&order);
    let p = PLAYER_REF;
    // Agility 5: 1 + (5 − 5) × 0.1 = 1; Agility 10: 1.5.
    state.actor_values.insert((p, 10), 5.0);
    assert!(near(
        combat::reload_rate(&order, &state, p, Some(&pistol)),
        1.0
    ));
    state.actor_values.insert((p, 10), 10.0);
    assert!(near(
        combat::reload_rate(&order, &state, p, Some(&pistol)),
        1.5
    ));
    run(&order, &mut state, "player.AddPerk RapidReload");
    assert!(near(
        combat::reload_rate(&order, &state, p, Some(&pistol)),
        1.875
    ));
    // The equip rate takes its own entry point (none held here).
    assert!(near(
        combat::equip_rate(&order, &state, p, Some(&pistol)),
        1.5
    ));
    // Attacks: the weapon's speed × its attack multiplier (1 × 1), Fast
    // Shot × 1.2 with guns only.
    assert!(near(
        combat::attack_rate(&order, &state, p, Some(&pistol)),
        1.0
    ));
    run(&order, &mut state, "player.AddPerk FastShot");
    assert!(near(
        combat::attack_rate(&order, &state, p, Some(&pistol)),
        1.2
    ));
    assert!(near(combat::attack_rate(&order, &state, p, None), 1.0));
    // Nobody else holds perks.
    assert!(near(
        combat::reload_rate(&order, &state, FormId(GECKO_REF), Some(&pistol)),
        1.0
    ));
}

#[test]
fn built_to_destroy_wears_the_weapon_faster() {
    let (_data, order) = order("perk-wear");
    let (mut state, pistol, _gecko) = armed(&order);
    let p = PLAYER_REF;
    // Each attack takes `fDamageToWeaponValue` 0.2 of the pistol's 150
    // health points.
    assert!(near(combat::attack_wear(&order, None), 0.2));
    combat::damage_weapon(
        &order,
        &mut state,
        p,
        &pistol,
        combat::attack_wear(&order, None),
    );
    assert!(near(
        combat::weapon_condition(&state, p, pistol.form_id),
        1.0 - 0.2 / 150.0
    ));
    run(&order, &mut state, "player.AddPerk BuiltToDestroy");
    combat::damage_weapon(
        &order,
        &mut state,
        p,
        &pistol,
        combat::attack_wear(&order, None),
    );
    assert!(near(
        combat::weapon_condition(&state, p, pistol.form_id),
        1.0 - (0.2 + 0.2 * 1.15) / 150.0
    ));
}

#[test]
fn adamantium_halves_limb_damage_from_the_attacks_it_names() {
    let (_data, order) = order("perk-limbs");
    let mut state = GameState::new(&order);
    let p = PLAYER_REF;
    let hit = world::body_parts::PartHit {
        part: Some(7),
        multiplier: 1.0,
        health_damage: 10.0,
        limb_damage: 10.0,
        weapon_damage: 0.0,
    };
    let lost = |state: &mut GameState, weapon: Option<u32>| {
        state.value_damage.remove(&(p, 29));
        world::body_parts::hurt_part(
            &order,
            state,
            p,
            &hit,
            false,
            (FormId(GECKO_REF), weapon.map(FormId)),
            world::melee::Special::None,
        );
        state.value_damage.get(&(p, 29)).copied().unwrap_or(0.0) as f32
    };
    assert!(near(lost(&mut state, Some(PISTOL)), 10.0));
    // × 0.5 against gun attacks (the attacker's weapon tab), not bites.
    run(&order, &mut state, "player.AddPerk Adamantium");
    assert!(near(lost(&mut state, Some(PISTOL)), 5.0));
    assert!(near(lost(&mut state, None), 10.0));
}

#[test]
fn an_entry_with_the_wrong_tab_count_is_not_applied() {
    let (_data, order) = order("perk-tabs");
    let state = GameState::new(&order);
    // Every entry point's parameters, as the game's table has them.
    use world::perks::{tab_kinds, TabKind};
    assert_eq!(tab_kinds(0), &[TabKind::Weapon, TabKind::Actor]);
    assert_eq!(tab_kinds(56), &[TabKind::Actor, TabKind::Weapon]);
    assert_eq!(tab_kinds(40), &[TabKind::Weapon]);
    assert_eq!(tab_kinds(17), &[TabKind::Item]);
    assert_eq!(tab_kinds(9), &[]);
    // A call with the wrong number of parameters applies nothing.
    let mut held = state.clone();
    held.perks.insert(FormId(COWHAND));
    assert!(near(
        world::perks::apply_for(&order, &held, PLAYER_REF, 0, 10.0, &[]),
        10.0
    ));
    assert!(near(
        world::perks::apply_for(
            &order,
            &held,
            PLAYER_REF,
            0,
            10.0,
            &[
                world::perks::Tab::Weapon(FormId(PISTOL)),
                world::perks::Tab::Target(FormId(GECKO_REF))
            ]
        ),
        12.5
    ));
    // Only the player's perks count.
    assert!(near(
        world::perks::apply_for(
            &order,
            &held,
            FormId(GECKO_REF),
            0,
            10.0,
            &[
                world::perks::Tab::Weapon(FormId(PISTOL)),
                world::perks::Tab::Target(PLAYER_REF)
            ]
        ),
        10.0
    ));
}
