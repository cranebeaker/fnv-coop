//! Hits between people (`Actor::AttackedBy`, Xbox PDB, `008987f0`;
//! `world::combat_ai::attacked_by`) on the generated combat world
//! (`testdata::fighting`): the guards are allies of the town, the town has
//! no relation to the guards, the raiders are the town's enemies. Allies
//! tolerate each other's hits; a stray hit doesn't start a fight; a hit
//! from someone aiming at you does; someone already fighting takes a
//! non-friend who hurts them on as another target.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fighting::ids::*;
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn new_state(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(CELL));
    state
}

/// `attacker` punches `victim` (fists: never enough to kill here).
fn punch(order: &LoadOrder, state: &mut GameState, attacker: u32, victim: u32) {
    let scripts = ScriptCache::default();
    let hit = Runner::new(order, &scripts, state).hit(FormId(attacker), FormId(victim), None);
    assert!(hit.is_some());
    assert!(!state.dead.contains(&FormId(victim)));
}

#[test]
fn an_ally_tolerates_hits_even_from_someone_aiming_at_them() {
    let (_data, order) = order("ff-ally");
    let (guard, town) = (FormId(GUARD_REF), FormId(TOWN_REF));
    // Not fighting; the townsperson is fighting the guard: still an ally's
    // hit (`GetShouldAttackActor` with the attacked flag: reaction 2).
    let mut state = new_state(&order);
    state.combat.insert(town, guard);
    punch(&order, &mut state, TOWN_REF, GUARD_REF);
    assert_eq!(state.combat.get(&guard), None);
    assert!(state.hit_targets.is_empty());
    // Fighting the raider, a stray from the townsperson: no new target.
    let mut state = new_state(&order);
    state.combat.insert(guard, FormId(RAIDER_REF));
    state.combat.insert(town, FormId(RAIDER_REF));
    punch(&order, &mut state, TOWN_REF, GUARD_REF);
    assert_eq!(state.combat.get(&guard), Some(&FormId(RAIDER_REF)));
    assert!(state.hit_targets.is_empty());
}

#[test]
fn a_stray_hit_starts_no_fight_but_an_aimed_one_does() {
    let (_data, order) = order("ff-stray");
    let (guard, town) = (FormId(GUARD_REF), FormId(TOWN_REF));
    // The guard, fighting the raider, hits the townsperson (neutral to the
    // guards): the townsperson isn't fighting and wasn't aimed at.
    let mut state = new_state(&order);
    state.combat.insert(guard, FormId(RAIDER_REF));
    punch(&order, &mut state, GUARD_REF, TOWN_REF);
    assert_eq!(state.combat.get(&town), None);
    assert!(state.hit_targets.is_empty());
    // The guard fighting the townsperson: the townsperson fights back
    // (unaggressive, but hit by someone neither friend nor ally).
    let mut state = new_state(&order);
    state.combat.insert(guard, town);
    punch(&order, &mut state, GUARD_REF, TOWN_REF);
    assert_eq!(state.combat.get(&town), Some(&guard));
}

#[test]
fn someone_fighting_takes_on_whoever_else_hurts_them() {
    let (_data, order) = order("ff-target");
    let (raider, town, guard) = (FormId(RAIDER_REF), FormId(TOWN_REF), FormId(GUARD_REF));
    // The raider fights the guard; a stray from the townsperson (an enemy
    // of the raiders) makes the townsperson another target.
    let mut state = new_state(&order);
    state.combat.insert(raider, guard);
    state.combat.insert(town, guard);
    punch(&order, &mut state, TOWN_REF, RAIDER_REF);
    assert_eq!(state.combat.get(&raider), Some(&guard));
    assert_eq!(state.hit_targets.get(&raider), Some(&vec![town]));
    // Hurt by their own target: nothing more.
    let mut state = new_state(&order);
    state.combat.insert(raider, guard);
    state.combat.insert(guard, raider);
    punch(&order, &mut state, GUARD_REF, RAIDER_REF);
    assert!(state.hit_targets.is_empty());
}

#[test]
fn frenzy_overrules_alliance() {
    let (_data, order) = order("ff-frenzy");
    let (guard, town) = (FormId(GUARD_REF), FormId(TOWN_REF));
    // A frenzied attacker: the guard fights back against its ally.
    let mut state = new_state(&order);
    state.actor_values.insert((town, 0), 3.0);
    state.combat.insert(town, guard);
    punch(&order, &mut state, TOWN_REF, GUARD_REF);
    assert_eq!(state.combat.get(&guard), Some(&town));
    // A frenzied victim fights anyone aiming at them.
    let mut state = new_state(&order);
    state.actor_values.insert((guard, 0), 3.0);
    state.combat.insert(town, guard);
    punch(&order, &mut state, TOWN_REF, GUARD_REF);
    assert_eq!(state.combat.get(&guard), Some(&town));
}
