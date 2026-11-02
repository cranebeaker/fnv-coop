//! Fatigue and knock-outs (`world::fatigue`), `ForceFlee`
//! (`world::ai::flee::force`) and combat groups (`world::combat_groups`),
//! on generated records carrying `FalloutNV.esm`'s values.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fatigue::ids::*;
use world::combat::{self, Weapon};
use world::dialogue::PLAYER_REF;
use world::fatigue::{self, knock};
use world::scripting::{Event, Facts, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fatigue::fatigue(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn facts<'a>(order: &'a LoadOrder, state: &'a GameState) -> Facts<'a> {
    Facts {
        order,
        state,
        speaker: None,
    }
}

fn full(order: &LoadOrder, state: &GameState, who: u32) -> f64 {
    facts(order, state)
        .permanent_actor_value(FormId(who), fatigue::FATIGUE)
        .unwrap()
}

fn now(order: &LoadOrder, state: &GameState, who: u32) -> f64 {
    fatigue::fatigue(order, state, FormId(who)).unwrap()
}

/// A player with Luck 0 (no criticals) and Unarmed 50.
fn player(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.actor_values.insert((PLAYER_REF, 11), 0.0);
    state
        .actor_values
        .insert((PLAYER_REF, combat::av::UNARMED), 50.0);
    state
}

#[test]
fn full_fatigue_is_the_record_and_the_derived_part() {
    let (_data, order) = order("fatigue-full");
    let mut state = GameState::new(&order);
    // `00643870`: the player 200 + 90 + 20 × 5 + 10 × (1 − 1).
    assert_eq!(full(&order, &state, 0x14), 390.0);
    state.player_level = 4;
    assert_eq!(full(&order, &state, 0x14), 420.0);
    // People: 50 + 90 + 20 × 5, no level term.
    assert_eq!(full(&order, &state, PERSON_REF), 240.0);
    // Worked out by the game (`006040d0`): 50 + 20 × 5 + 10 × 3, added to
    // the record's 50 (`008803a0`, the value's flag 0x40).
    assert_eq!(full(&order, &state, AUTO_REF), 230.0);
    // A creature: its record's.
    assert_eq!(full(&order, &state, CREATURE_REF), 100.0);
    // A full value reads 1 (`GetFatiguePercentage`).
    assert_eq!(
        fatigue::percentage(&order, &state, FormId(PERSON_REF)),
        Some(1.0)
    );
}

#[test]
fn fists_tire_by_half_their_damage_until_the_minimum() {
    let (_data, order) = order("fatigue-fists");
    let scripts = ScriptCache::default();
    let mut state = player(&order);
    let person = FormId(PERSON_REF);
    // Fists with Unarmed 50: 1 + 0.5 + 0.05 × 50 = 4 health, 2 fatigue.
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, person, None, None)
        .unwrap();
    assert!((hit.dealt - 4.0).abs() < 1e-4, "{}", hit.dealt);
    assert!((hit.fatigue - 2.0).abs() < 1e-4, "{}", hit.fatigue);
    assert!((now(&order, &state, PERSON_REF) - 238.0).abs() < 1e-4);
    // At the minimum (−25) or below, hits take no more (`0089d6f0`).
    state.value_damage.insert((person, fatigue::FATIGUE), 265.0);
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, person, None, None)
        .unwrap();
    assert_eq!(hit.fatigue, 0.0);
    assert_eq!(now(&order, &state, PERSON_REF), -25.0);
    // Just above it, one more.
    state.value_damage.insert((person, fatigue::FATIGUE), 264.0);
    Runner::new(&order, &scripts, &mut state).hit_at(PLAYER_REF, person, None, None);
    assert!((now(&order, &state, PERSON_REF) + 26.0).abs() < 1e-4);
}

#[test]
fn creatures_and_the_knocked_down_take_no_fist_fatigue() {
    let (_data, order) = order("fatigue-none");
    let mut state = player(&order);
    // A creature striking without a weapon (`00646310` is only for people).
    assert_eq!(
        fatigue::fists_fatigue(&order, &state, FormId(CREATURE_REF), PLAYER_REF, 10.0),
        0.0
    );
    assert_eq!(
        fatigue::fists_fatigue(&order, &state, PLAYER_REF, FormId(PERSON_REF), 10.0),
        5.0
    );
    // Someone already knocked down (vtable +0x230).
    state
        .knocks
        .state
        .insert(FormId(PERSON_REF), knock::KNOCKED_OUT);
    assert_eq!(
        fatigue::fists_fatigue(&order, &state, PLAYER_REF, FormId(PERSON_REF), 10.0),
        0.0
    );
}

#[test]
fn a_bean_bag_adds_its_fatigue_through_the_armour() {
    let (_data, order) = order("fatigue-beanbag");
    let gun = Weapon::load(&order, FormId(SHOTGUN)).unwrap();
    // `009b5a30`: the round's kind-5 effect adds 250 to the hit's 0.
    assert_eq!(
        fatigue::through_armour(&order, 0.0, Some(FormId(BEAN_BAG)), true, 10.0, 10.0),
        250.0
    );
    // In the share the armour let through.
    assert_eq!(
        fatigue::through_armour(&order, 0.0, Some(FormId(BEAN_BAG)), true, 10.0, 5.0),
        125.0
    );
    // Fists' fatigue isn't given the ammunition's.
    assert_eq!(
        fatigue::through_armour(&order, 2.0, Some(FormId(BEAN_BAG)), false, 4.0, 4.0),
        2.0
    );
    let scripts = ScriptCache::default();
    let mut state = player(&order);
    state.items.insert((PLAYER_REF, FormId(BEAN_BAG)), 10);
    // The round's damage × 0.05 is held at the 20% floor (`009b5a30`), so
    // a fifth of the 250 gets through: 50.
    let hit = Runner::new(&order, &scripts, &mut state)
        .hit_at(PLAYER_REF, FormId(PERSON_REF), Some(&gun), None)
        .unwrap();
    assert!((hit.fatigue - 50.0).abs() < 1e-3, "{}", hit.fatigue);
    assert!((now(&order, &state, PERSON_REF) - 190.0).abs() < 1e-3);
    // Five hits: 240 − 250, below 0, knocked out on the next update.
    for _ in 0..4 {
        Runner::new(&order, &scripts, &mut state).hit_at(
            PLAYER_REF,
            FormId(PERSON_REF),
            Some(&gun),
            None,
        );
    }
    assert!((now(&order, &state, PERSON_REF) + 10.0).abs() < 1e-3);
    fatigue::advance(&order, &mut state, 0.0);
    assert_eq!(
        fatigue::knock_state(&state, FormId(PERSON_REF)),
        knock::KNOCK_OUT_LEAD_IN
    );
}

#[test]
fn knocked_out_below_zero_then_all_back_at_zero() {
    let (_data, order) = order("fatigue-knockout");
    let mut state = GameState::new(&order);
    let person = FormId(PERSON_REF);
    // `DamageAV Fatigue 250` (the cattle prod's and boxing gloves' hit
    // scripts damage it so): 240 − 250 = −10.
    world::magic::change(&order, &mut state, person, fatigue::FATIGUE, -250.0, person);
    assert_eq!(now(&order, &state, PERSON_REF), -10.0);
    state.events.clear();
    fatigue::advance(&order, &mut state, 0.0);
    assert_eq!(
        fatigue::knock_state(&state, person),
        knock::KNOCK_OUT_LEAD_IN
    );
    assert_eq!(state.events, vec![Event::KnockedOut { who: person }]);
    // `GetKnockedState` is 1 while falling and lying; using them says
    // they're unconscious.
    assert_eq!(fatigue::knocked_state_value(&state, person), 1.0);
    fatigue::advance(&order, &mut state, 1.0);
    assert_eq!(fatigue::knock_state(&state, person), knock::KNOCKED_OUT);
    assert_eq!(now(&order, &state, PERSON_REF), -9.0);
    match world::living::pickpocket::use_person(&order, &state, person, false) {
        world::living::pickpocket::Use::Refused(text, _) => {
            assert!(text.ends_with("is unconscious."), "{text}")
        }
        other => panic!("{other:?}"),
    }
    // 1 a second (`fFatigueReturnBase`); still down at −1.
    for _ in 0..8 {
        fatigue::advance(&order, &mut state, 1.0);
    }
    assert_eq!(now(&order, &state, PERSON_REF), -1.0);
    assert_eq!(fatigue::knock_state(&state, person), knock::KNOCKED_OUT);
    // Reaching 0 brings all of it back (`0088b5a0`), and they get up.
    state.events.clear();
    fatigue::advance(&order, &mut state, 1.0);
    assert_eq!(now(&order, &state, PERSON_REF), 240.0);
    assert_eq!(fatigue::knock_state(&state, person), knock::GETTING_UP);
    assert_eq!(state.events, vec![Event::GotUp { who: person }]);
    assert_eq!(fatigue::knocked_state_value(&state, person), 0.0);
    fatigue::advance(&order, &mut state, 1.0);
    assert_eq!(fatigue::knock_state(&state, person), knock::NORMAL);
}

#[test]
fn some_are_never_knocked_down() {
    let (_data, order) = order("fatigue-sturdy");
    let mut state = GameState::new(&order);
    let sturdy = FormId(STURDY_REF);
    world::magic::change(
        &order,
        &mut state,
        sturdy,
        fatigue::FATIGUE,
        -1000.0,
        sturdy,
    );
    fatigue::advance(&order, &mut state, 0.0);
    assert_eq!(fatigue::knock_state(&state, sturdy), knock::NORMAL);
}

#[test]
fn essential_people_down_count_as_knocked_out() {
    let (_data, order) = order("fatigue-essential");
    let mut state = GameState::new(&order);
    let person = FormId(PERSON_REF);
    state.more.down.insert(person, 10.0);
    assert_eq!(fatigue::knocked_state_value(&state, person), 1.0);
    assert!(fatigue::lies_down(&state, person));
    // Their going down and up are `world::combat`'s: no events here.
    fatigue::advance(&order, &mut state, 0.0);
    assert!(state.events.is_empty());
    // `GetKnockedState` is function 107 (opcode 0x106b).
    assert_eq!(
        facts(&order, &state).value(107, Some(person), &[]),
        Some(1.0)
    );
}

#[test]
fn force_flee_starts_the_engines_flee_out_of_a_fight() {
    let (_data, order) = order("fatigue-flee");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let person = FormId(PERSON_REF);
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestFatiguePersonRef.ForceFlee",
        None,
        None,
    );
    assert!(world::ai::flee::forced(&state, person));
    assert!(state.events.contains(&Event::Flees {
        who: person,
        to: None
    }));
    // With a reference to run to.
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestFatigueAutoRef.ForceFlee TestFatigueCell TestFatigueMarkerRef",
        None,
        None,
    );
    let f = state.forced_flee[&FormId(AUTO_REF)];
    assert_eq!(f.to, Some(FormId(MARKER_REF)));
    assert_eq!(f.cell, Some(FormId(CELL)));
    // Picking their package again ends it (`EvaluatePackage`).
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestFatigueAutoRef.EvaluatePackage",
        None,
        None,
    );
    assert!(!world::ai::flee::forced(&state, FormId(AUTO_REF)));
    // Fighting: the combat controller's ForceFlee does nothing in this
    // build (`008d0600`).
    state.combat.insert(FormId(FRIEND_REF), PLAYER_REF);
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestFatigueFriendRef.ForceFlee",
        None,
        None,
    );
    assert!(!world::ai::flee::forced(&state, FormId(FRIEND_REF)));
    // Nor on the player, the unconscious or the dead.
    assert!(!world::ai::flee::force(&mut state, PLAYER_REF, None, None));
    state.unconscious.insert(FormId(STURDY_REF));
    assert!(!world::ai::flee::force(
        &mut state,
        FormId(STURDY_REF),
        None,
        None
    ));
    // They're fleeing: using them says so.
    match world::living::pickpocket::use_person(&order, &state, person, false) {
        world::living::pickpocket::Use::Refused(text, _) => {
            assert!(text.contains("fleeing"), "{text}")
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn combat_groups_count_their_members_and_targets() {
    use world::combat_groups as groups;
    let (_data, order) = order("fatigue-groups");
    let mut state = GameState::new(&order);
    let (a, b) = (FormId(PERSON_REF), FormId(FRIEND_REF));
    // Out of a fight: 0.
    assert_eq!(groups::member_count(&state, a), 0);
    // Starting a fight alone: a group of one, with its target.
    state.combat.insert(a, PLAYER_REF);
    assert_eq!(groups::member_count(&state, a), 1);
    assert_eq!(groups::target_count(&state, a), 1);
    // The player's group: made by being attacked, the attacker its target.
    assert_eq!(groups::member_count(&state, PLAYER_REF), 1);
    assert_eq!(groups::target_count(&state, PLAYER_REF), 1);
    // The friend sees `a` fighting someone it detects, and helps: it joins
    // `a`'s group (both in a faction allied with itself, assistance 2).
    assert!(groups::helps_group(&order, &state, b, a, 50, |_| Some(10)));
    // Not without detecting the target.
    assert!(!groups::helps_group(&order, &state, b, a, 50, |_| Some(0)));
    state.combat.insert(b, PLAYER_REF);
    groups::join(&mut state, b, a);
    assert_eq!(groups::member_count(&state, a), 2);
    assert_eq!(groups::member_count(&state, b), 2);
    assert!(!groups::helps_group(&order, &state, b, a, 50, |_| Some(10)));
    // The conditions' function (416, opcode 0x11a0), on either.
    assert_eq!(facts(&order, &state).value(416, Some(b), &[]), Some(2.0));
    assert_eq!(facts(&order, &state).value(417, Some(b), &[]), Some(1.0));
    // Leaving the fight leaves the group.
    state.combat.remove(&b);
    groups::tidy(&mut state);
    assert_eq!(groups::member_count(&state, a), 1);
    assert!(state.combat_groups.joined.is_empty());
}

#[test]
fn an_explosions_danger_counts_the_combat_managers_actors() {
    use world::combat_groups as groups;
    let (_data, order) = order("fatigue-danger");
    let mut state = GameState::new(&order);
    let (a, b, c) = (FormId(PERSON_REF), FormId(FRIEND_REF), FormId(0x0100_0999));
    let point = [0.0; 3];
    let all = |state: &GameState| {
        groups::explosion_nearby(
            state,
            a,
            [
                (a, [10.0, 0.0, 0.0]),
                (b, [20.0, 0.0, 0.0]),
                (c, [30.0, 0.0, 0.0]),
                (PLAYER_REF, [0.0; 3]),
            ],
            point,
            100.0,
        )
    };
    // 00992ba0: `a` fights the player alone. The player is its group's
    // target (not counted); `b` and `c` aren't in any fight, so the
    // manager doesn't hold them: not spectators.
    state.combat.insert(a, PLAYER_REF);
    let n = all(&state);
    assert_eq!((n.own, n.group, n.spectators), (1, 0, 0));
    // `b` joins `a`'s group: its group.
    state.combat.insert(b, PLAYER_REF);
    groups::join(&mut state, b, a);
    let n = all(&state);
    assert_eq!((n.own, n.group, n.spectators), (1, 1, 0));
    // `c` fights the player in a group of its own: held, not `a`'s group's
    // target, so a spectator.
    state.combat.insert(c, PLAYER_REF);
    let n = all(&state);
    assert_eq!((n.own, n.group, n.spectators), (1, 1, 1));
    // Out of reach: not counted.
    let far = groups::explosion_nearby(&state, a, [(c, [300.0, 0.0, 0.0])], point, 100.0);
    assert_eq!(far.spectators, 0);
    // The dead aren't counted.
    state.dead.insert(c);
    assert_eq!(all(&state).spectators, 0);
}
