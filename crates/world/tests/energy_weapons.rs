//! Energy weapons (`docs/ENERGY_WEAPONS.md`): their records, energy cells
//! against armour, the resistance their damage meets, the critical chance
//! of automatic ones, and the critical effects that disintegrate or goo
//! the killed, on generated records carrying `FalloutNV.esm`'s values.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::energy::ids::*;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::energy::energy(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A player with Energy Weapons 100 and the given Luck.
fn player(order: &LoadOrder, luck: f64) -> GameState {
    let mut state = GameState::new(order);
    state.actor_values.insert((PLAYER_REF, 34), 100.0);
    state.actor_values.insert((PLAYER_REF, 11), luck);
    state
}

fn target() -> FormId {
    FormId(TARGET_REF)
}

#[test]
fn energy_weapons_carry_their_critical_effect_resistance_and_projectile() {
    let (_data, order) = order("energy-records");
    let laser = Weapon::load(&order, FormId(LASER_PISTOL)).unwrap();
    assert_eq!(laser.crit_effect, Some(FormId(LASER_SPELL)));
    assert!(laser.crit_on_death);
    assert_eq!(laser.resist, Some(60));
    assert_eq!(laser.skill, 34);
    assert_eq!(laser.ammo, vec![FormId(CELL), FormId(CELL_OVERCHARGE)]);
    let plasma = Weapon::load(&order, FormId(PLASMA_PISTOL)).unwrap();
    assert_eq!(plasma.crit_effect, Some(FormId(PLASMA_SPELL)));
    assert_eq!(plasma.ammo_use, 2);
    // The beam is a beam (type 4), the plasma bolt a missile (type 1)
    // flying at 7500 units a second.
    let beam = world::explosions::ProjectileRecord::load(&order, FormId(BEAM)).unwrap();
    let bolt = world::explosions::ProjectileRecord::load(&order, FormId(PLASMA_BOLT)).unwrap();
    assert_eq!(beam.kind, world::explosions::proj_type::BEAM);
    assert_eq!(bolt.kind, world::explosions::proj_type::MISSILE);
    assert_eq!(bolt.speed, 7500.0);
}

#[test]
fn energy_cells_bypass_threshold_and_energy_resistance_comes_last() {
    let (_data, order) = order("energy-armour");
    let mut state = player(&order, 0.0);
    let t = target();
    let attacker = Some((PLAYER_REF, Some(FormId(LASER_PISTOL))));
    // Damage threshold 10: a cell's "DT − 2" leaves 8 to take off 12.
    state.actor_values.insert((t, 76), 10.0);
    let through = |state: &GameState, ammo: u32| {
        combat::hit_through_armour(&order, state, 12.0, attacker, t, Some(FormId(ammo)))
    };
    assert!((through(&state, CELL) - 4.0).abs() < 1e-4);
    // Over charge: DT − 5 and damage × 1.25 after the threshold: (12 − 5)
    // × 1.25.
    assert!((through(&state, CELL_OVERCHARGE) - 8.75).abs() < 1e-4);
    // Energy Resistance 50 halves what's left after the 20% floor, so it
    // can go below it: 4 × 0.5.
    state.actor_values.insert((t, 60), 50.0);
    assert!((through(&state, CELL) - 2.0).abs() < 1e-4);
    // With a threshold the hit can't beat, the floor (2.4) then the
    // resistance: 1.2.
    state.actor_values.insert((t, 76), 100.0);
    assert!((through(&state, CELL) - 1.2).abs() < 1e-4);
    // Resistance of 100 or more takes it all; below 0 nothing.
    state.actor_values.insert((t, 76), 0.0);
    state.actor_values.insert((t, 60), 150.0);
    assert_eq!(through(&state, CELL), 0.0);
    state.actor_values.insert((t, 60), -20.0);
    assert!((through(&state, CELL) - 12.0).abs() < 1e-4);
    // A weapon without a resist type isn't resisted.
    let gun = Weapon::load(&order, FormId(LASER_PISTOL)).map(|w| Weapon { resist: None, ..w });
    assert_eq!(combat::resisted_share(&order, &state, gun.as_ref(), t), 1.0);
}

#[test]
fn automatic_weapons_share_the_critical_chance_over_their_fire_rate() {
    let (_data, order) = order("energy-crit-chance");
    let state = player(&order, 6.0);
    let rcw = Weapon::load(&order, FormId(LASER_RCW)).unwrap();
    let pistol = Weapon::load(&order, FormId(LASER_PISTOL)).unwrap();
    assert_eq!(combat::critical_divisor(Some(&rcw)), 9.0);
    assert_eq!(combat::critical_divisor(Some(&pistol)), 1.0);
    let crit = |w: &Weapon, roll: u64| {
        combat::critical(&order, &state, PLAYER_REF, Some(w), target(), false, roll)
    };
    // Luck 6 × 1.5: 9%, rolled per mille.
    assert!(crit(&pistol, 89) && !crit(&pistol, 90));
    // 6 ÷ 9 × 0.5: a third of a percent (3.33 per mille, whole rolls
    // below it).
    assert!(crit(&rcw, 2) && !crit(&rcw, 3));
}

/// The laser pistol's critical kills the target (5 health left).
fn laser_kill(state: &mut GameState, order: &LoadOrder, scripts: &ScriptCache) -> FormId {
    let t = target();
    let full = combat::max_health(order, state, t).unwrap();
    state.damage.insert(t, full - 5.0);
    let laser = Weapon::load(order, FormId(LASER_PISTOL)).unwrap();
    let hit = Runner::new(order, scripts, state)
        .strike_at(PLAYER_REF, t, Some(&laser), None, false)
        .unwrap();
    assert!(hit.critical);
    assert!(state.dead.contains(&t));
    t
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, seconds: f32) {
    let mut left = seconds;
    while left > 1e-4 {
        let step = left.min(0.1);
        world::magic::tick(&mut Runner::new(order, scripts, state), step);
        left -= step;
    }
}

#[test]
fn a_laser_critical_that_kills_disintegrates_the_body_into_an_ash_pile() {
    let (_data, order) = order("energy-disintegrate");
    let scripts = ScriptCache::default();
    let mut state = player(&order, 100.0);
    let t = laser_kill(&mut state, &order, &scripts);
    // The spell is on the corpse and survives death.
    assert!(world::magic::is_target_of(&state, t, FormId(LASER_SPELL)));
    assert!(world::magic::survives_death(&order, FormId(LASER_EFFECT)));
    run(&order, &scripts, &mut state, 1.0);
    let stage = |s: &GameState| s.more.critical_stage.get(&t).copied();
    assert_eq!(stage(&state), Some(3));
    assert!(state.more.ash_piles.is_empty());
    assert!(!world::more_functions::body_gone(&state, t));
    // With half a second to go the ash pile is left where they lay.
    run(&order, &scripts, &mut state, 0.5);
    let (&pile, &corpse) = state.more.ash_piles.iter().next().unwrap();
    assert_eq!(corpse, t);
    let made = state.more.placed.refs[&pile];
    assert_eq!(made.base, FormId(ASH_PILE));
    assert_eq!(made.position, [0.0, 500.0, 0.0]);
    // Searching the pile searches the corpse.
    assert_eq!(world::activation::stands_for(&order, &state, pile), t);
    assert_eq!(world::activation::stands_for(&order, &state, t), t);
    // The pile is among the made references E can use in the room, and
    // the crosshair names it as an activator: "Activate Ash Pile", "Empty"
    // while the corpse holds nothing (`00775a00`).
    let space = FormId(ROOM);
    let usable = world::more_functions::placed::dropped_items(&order, &state, space);
    assert!(usable.iter().any(|i| i.reference == pile));
    let info = world::activation::info(&order, &state, pile).unwrap();
    assert_eq!(info.action.as_deref(), Some("Activate"));
    assert_eq!(info.target, "Ash Pile");
    assert_eq!(info.empty.as_deref(), Some("Empty"));
    state.items.insert((t, FormId(CELL)), 3);
    state.stocked.insert(t);
    let info = world::activation::info(&order, &state, pile).unwrap();
    assert_eq!(info.empty, None);
    // Then the body goes, and it counts as the player's disintegration.
    run(&order, &scripts, &mut state, 0.5);
    assert_eq!(stage(&state), Some(4));
    assert!(world::more_functions::body_gone(&state, t));
    assert_eq!(world::stats::get(&state, world::stats::DISINTEGRATIONS), 1);
    // One pile only; the effect ends after its 4 s.
    run(&order, &scripts, &mut state, 3.0);
    assert_eq!(state.more.ash_piles.len(), 1);
    assert!(!world::magic::is_target_of(&state, t, FormId(LASER_SPELL)));
}

#[test]
fn a_plasma_critical_that_kills_leaves_a_goo_pile() {
    let (_data, order) = order("energy-goo");
    let scripts = ScriptCache::default();
    let mut state = player(&order, 100.0);
    let t = target();
    let full = combat::max_health(&order, &state, t).unwrap();
    state.damage.insert(t, full - 5.0);
    let plasma = Weapon::load(&order, FormId(PLASMA_PISTOL)).unwrap();
    let hit = Runner::new(&order, &scripts, &mut state)
        .strike_at(PLAYER_REF, t, Some(&plasma), None, false)
        .unwrap();
    assert!(hit.critical && state.dead.contains(&t));
    run(&order, &scripts, &mut state, 0.5);
    assert_eq!(state.more.critical_stage.get(&t), Some(&1));
    run(&order, &scripts, &mut state, 1.5);
    assert_eq!(state.more.critical_stage.get(&t), Some(&2));
    assert!(world::more_functions::body_gone(&state, t));
    let (&pile, _) = state.more.ash_piles.iter().next().unwrap();
    assert_eq!(state.more.placed.refs[&pile].base, FormId(GOO_PILE));
}

#[test]
fn on_death_effects_wait_for_a_kill_and_spare_sitters() {
    let (_data, order) = order("energy-on-death");
    let scripts = ScriptCache::default();
    let laser = Weapon::load(&order, FormId(LASER_PISTOL)).unwrap();
    // A critical that doesn't kill casts nothing ("on death").
    let mut state = player(&order, 100.0);
    let t = target();
    let hit = Runner::new(&order, &scripts, &mut state)
        .strike_at(PLAYER_REF, t, Some(&laser), None, false)
        .unwrap();
    assert!(hit.critical && !state.dead.contains(&t));
    assert!(state.active_effects.is_empty());
    // A critical without "on death" casts it on the living.
    let anytime = Weapon {
        crit_on_death: false,
        ..laser.clone()
    };
    Runner::new(&order, &scripts, &mut state).strike_at(PLAYER_REF, t, Some(&anytime), None, false);
    assert!(world::magic::is_target_of(&state, t, FormId(LASER_SPELL)));
    // Someone sitting is spared the effects `BannedEffectsOnSitters` holds.
    let mut state = player(&order, 100.0);
    state.furniture.insert(t, FormId(0x123));
    assert_eq!(combat::sit_sleep_state(&state, t), 4);
    let full = combat::max_health(&order, &state, t).unwrap();
    state.damage.insert(t, full - 5.0);
    let hit = Runner::new(&order, &scripts, &mut state)
        .strike_at(PLAYER_REF, t, Some(&laser), None, false)
        .unwrap();
    assert!(hit.critical && state.dead.contains(&t));
    assert!(state.active_effects.is_empty());
    // The already dead get nothing.
    let mut state = player(&order, 100.0);
    let t = laser_kill(&mut state, &order, &scripts);
    state.active_effects.clear();
    Runner::new(&order, &scripts, &mut state).strike_at(PLAYER_REF, t, Some(&laser), None, false);
    assert!(state.active_effects.is_empty());
}

#[test]
fn death_dispels_effects_unless_their_magic_effect_survives_it() {
    let (_data, order) = order("energy-dispel");
    let scripts = ScriptCache::default();
    let mut state = player(&order, 0.0);
    let t = target();
    world::magic::add_spell(&order, &mut state, t, FormId(BURN_SPELL), t, false);
    world::magic::add_spell(&order, &mut state, t, FormId(LASER_SPELL), t, false);
    run(&order, &scripts, &mut state, 0.1);
    assert_eq!(state.active_effects.len(), 2);
    combat::hurt(&order, &mut state, t, 1000.0, PLAYER_REF);
    run(&order, &scripts, &mut state, 0.1);
    // The burn ended, its finish block run; the disintegration goes on.
    assert_eq!(state.globals.get(&FormId(GLOBAL)), Some(&1.0));
    assert!(!world::magic::is_target_of(&state, t, FormId(BURN_SPELL)));
    assert!(world::magic::is_target_of(&state, t, FormId(LASER_SPELL)));
    assert!(!world::magic::survives_death(&order, FormId(BURN_EFFECT)));
}

#[test]
fn ash_piles_are_saved_with_their_corpse() {
    let (_data, order) = order("energy-save");
    let scripts = ScriptCache::default();
    let mut state = player(&order, 100.0);
    laser_kill(&mut state, &order, &scripts);
    run(&order, &scripts, &mut state, 2.0);
    assert_eq!(state.more.ash_piles.len(), 1);
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert_eq!(back.more.ash_piles, state.more.ash_piles);
    assert_eq!(back.more.critical_stage, state.more.critical_stage);
}
