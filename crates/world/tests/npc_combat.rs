//! People's weapons in a fight (`world::npc_combat`) on the generated
//! combat world (`testdata::fighting`): which weapon the arsenal and the
//! attack costs pick, ammunition and reloads, a thrown weapon used up,
//! `OnStartCombat` and `GetShouldAttack`, and `SetUnconscious` ending the
//! fight.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::fighting::ids::*;
use world::combat::{self, Weapon};
use world::combat_ai::{CombatStyle, SettingCache};
use world::dialogue::PLAYER_REF;
use world::npc_combat::{self, kind, AfterShot};
use world::scripting::{Facts, GameState, Runner, ScriptCache, Value};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::fighting::fighting(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// The guard carrying `items` besides their pistol.
fn armed(order: &LoadOrder, items: &[(u32, i32)]) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(CELL));
    let guard = FormId(GUARD_REF);
    state.stock(order, guard);
    for &(item, n) in items {
        state.items.insert((guard, FormId(item)), n);
    }
    state
}

#[test]
fn the_guard_fights_with_the_best_weapon_that_reaches() {
    let (_data, order) = order("npcc-choice");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let guard = FormId(GUARD_REF);
    let style = CombatStyle::of(&order, guard);
    let mut state = armed(
        &order,
        &[(RIFLE, 1), (ROUND, 20), (MACHETE, 1), (DYNAMITE, 3)],
    );
    // Each kind's best: the pistol (3.125 attacks a second) beats the
    // rifle (1.25) among guns; the machete; the dynamite as a grenade.
    let a = npc_combat::arsenal(&order, &state, guard, &style, Some(600.0), &s);
    assert_eq!(a.weapons[kind::RANGED], Some(FormId(PISTOL)));
    assert_eq!(a.weapons[kind::MELEE], Some(FormId(MACHETE)));
    assert_eq!(a.weapons[kind::GRENADE], Some(FormId(DYNAMITE)));
    assert!(a.scores[kind::HAND_TO_HAND] > 0.0);
    assert_eq!(npc_combat::cheapest_kind(&a), Some(kind::RANGED));
    // The fight starts: the pistol is drawn.
    assert!(npc_combat::choose_weapon(
        &order,
        &mut state,
        guard,
        &style,
        Some(600.0),
        &s
    ));
    assert_eq!(
        combat::weapon_in_hand(&order, &state, guard).map(|w| w.form_id),
        Some(FormId(PISTOL))
    );
    // Not again until `fCombatInventoryUpdateTimer` (5 s) has passed.
    state.items.remove(&(guard, FormId(PISTOL)));
    assert!(!npc_combat::choose_weapon(
        &order,
        &mut state,
        guard,
        &style,
        Some(600.0),
        &s
    ));
    state.seconds += 5.0;
    // Without the pistol the rifle is the gun.
    npc_combat::choose_weapon(&order, &mut state, guard, &style, Some(600.0), &s);
    assert_eq!(
        combat::weapon_in_hand(&order, &state, guard).map(|w| w.form_id),
        Some(FormId(RIFLE))
    );
}

#[test]
fn a_gunman_fights_with_the_gun_and_rounds_his_leveled_list_gives() {
    // Ghost Town Gunfight: the Powder Gangers carry their guns only
    // through `WithAmmoNVâ€¦Loot` lists (e.g. `GSPGAAM2`'s
    // `WithAmmoNVSingleShotgunLoot`). Untouched (never stocked), the
    // fight must still see the gun and its rounds, not leave them fists.
    let (_data, order) = order("npcc-leveled");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let gunman = FormId(GUNMAN_REF);
    let style = CombatStyle::of(&order, gunman);
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(CELL));
    npc_combat::choose_weapon(&order, &mut state, gunman, &style, Some(600.0), &s);
    assert_eq!(
        combat::weapon_in_hand(&order, &state, gunman).map(|w| w.form_id),
        Some(FormId(RIFLE))
    );
    assert_eq!(state.item_count(&order, gunman, FormId(ROUND)), 10);
    assert!(!state.npc_combat.unarmed.contains(&gunman));
}

#[test]
fn guns_need_their_ammunition_and_far_targets_keep_the_best_gun() {
    let (_data, order) = order("npcc-ammo");
    let settings = SettingCache::default();
    let s = |n: &str, d: f32| settings.get(&order, n, d);
    let guard = FormId(GUARD_REF);
    let style = CombatStyle::of(&order, guard);
    // A rifle without rounds isn't counted.
    let mut state = armed(&order, &[(RIFLE, 1), (MACHETE, 1)]);
    state.items.remove(&(guard, FormId(PISTOL)));
    let a = npc_combat::arsenal(&order, &state, guard, &style, Some(600.0), &s);
    assert_eq!(a.weapons[kind::RANGED], None);
    assert_eq!(npc_combat::cheapest_kind(&a), Some(kind::MELEE));
    // Beyond every gun's absolute maximum (3000 for the rifle, 3072 for
    // the pistol): the best of them is kept.
    let state = armed(&order, &[(RIFLE, 1), (ROUND, 5)]);
    let a = npc_combat::arsenal(&order, &state, guard, &style, Some(5000.0), &s);
    assert_eq!(a.weapons[kind::RANGED], Some(FormId(PISTOL)));
    assert_eq!(a.scores[kind::RANGED], a.best_score);
    // The rifle, set aside, would need to be within 3000.
    assert_eq!(a.recheck_range, 3000.0);
    // At 3050 only the pistol (3072) reaches; the rifle (3000) is set
    // aside, its score Ã· 10000.
    let a = npc_combat::arsenal(&order, &state, guard, &style, Some(3050.0), &s);
    assert_eq!(a.weapons[kind::RANGED], Some(FormId(PISTOL)));
    // A broken pistol (condition 0) isn't rated at all: the rifle, the
    // only gun left, is kept though it doesn't reach.
    let mut worn = state.clone();
    worn.weapon_health.insert((guard, FormId(PISTOL)), 0.0);
    let a = npc_combat::arsenal(&order, &worn, guard, &style, Some(3050.0), &s);
    assert_eq!(a.weapons[kind::RANGED], Some(FormId(RIFLE)));
    assert_eq!(a.recheck_range, 0.0);
}

#[test]
fn clips_empty_and_reload_and_teammates_use_rounds_up() {
    let (_data, order) = order("npcc-reload");
    let guard = FormId(GUARD_REF);
    let rifle = Weapon::load(&order, FormId(RIFLE)).unwrap();
    // Not a teammate, a weapon without "NPCs use ammo": the clip runs
    // down but the rounds stay.
    let mut state = armed(&order, &[(RIFLE, 1), (ROUND, 7)]);
    assert!(!npc_combat::should_use_ammo(&state, guard, &rifle));
    for _ in 0..4 {
        assert_eq!(
            npc_combat::fired(&order, &mut state, guard, &rifle),
            AfterShot::Ready
        );
    }
    // The fifth empties it: a reload of 2 s (Agility 5, rate 1).
    assert_eq!(
        npc_combat::fired(&order, &mut state, guard, &rifle),
        AfterShot::Reloading(2.0)
    );
    assert!(npc_combat::reloading(&state, guard));
    assert_eq!(state.item_count(&order, guard, FormId(ROUND)), 7);
    state.seconds += 2.0;
    assert!(!npc_combat::reloading(&state, guard));
    // A teammate uses rounds up: a first clip of 5 of 7, then the 2 left,
    // then nothing.
    let mut state = armed(&order, &[(RIFLE, 1), (ROUND, 7)]);
    state.teammates.insert(guard);
    assert!(npc_combat::should_use_ammo(&state, guard, &rifle));
    for _ in 0..4 {
        npc_combat::fired(&order, &mut state, guard, &rifle);
    }
    assert_eq!(
        npc_combat::fired(&order, &mut state, guard, &rifle),
        AfterShot::Reloading(2.0)
    );
    assert_eq!(state.item_count(&order, guard, FormId(ROUND)), 2);
    state.seconds += 2.0;
    npc_combat::fired(&order, &mut state, guard, &rifle);
    assert_eq!(
        npc_combat::fired(&order, &mut state, guard, &rifle),
        AfterShot::Dry
    );
    assert_eq!(state.item_count(&order, guard, FormId(ROUND)), 0);
}

#[test]
fn dynamite_is_used_up_throw_by_throw() {
    let (_data, order) = order("npcc-dynamite");
    let guard = FormId(GUARD_REF);
    let dynamite = Weapon::load(&order, FormId(DYNAMITE)).unwrap();
    let mut state = armed(&order, &[(DYNAMITE, 2)]);
    state.equip(&order, guard, FormId(DYNAMITE));
    // "NPCs use ammo" (flags2 0x2): each throw takes a stick.
    assert!(npc_combat::should_use_ammo(&state, guard, &dynamite));
    assert_eq!(
        npc_combat::fired(&order, &mut state, guard, &dynamite),
        AfterShot::Ready
    );
    assert_eq!(state.item_count(&order, guard, FormId(DYNAMITE)), 1);
    assert_eq!(
        npc_combat::fired(&order, &mut state, guard, &dynamite),
        AfterShot::Dry
    );
    assert!(!state.is_equipped(guard, FormId(DYNAMITE)));
}

#[test]
fn a_fight_runs_on_start_combat_once_the_target_is_detected() {
    let (_data, order) = order("npcc-start");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let raider = FormId(RAIDER_REF);
    let global = FormId(FIGHT_GLOBAL);
    state.combat.insert(raider, PLAYER_REF);
    // Not yet detected: nothing.
    npc_combat::start_combat_event(
        &mut Runner::new(&order, &scripts, &mut state),
        raider,
        false,
    );
    assert_eq!(state.globals.get(&global).copied().unwrap_or(0.0), 0.0);
    // Detected: the block naming the player runs, once in the fight.
    for _ in 0..2 {
        npc_combat::start_combat_event(
            &mut Runner::new(&order, &scripts, &mut state),
            raider,
            true,
        );
    }
    assert_eq!(state.globals.get(&global), Some(&1.0));
    // A new fight runs it again; one against someone else doesn't match
    // `OnStartCombat player`.
    npc_combat::combat_over(&mut state, raider);
    state.combat.insert(raider, FormId(TOWN_REF));
    npc_combat::start_combat_event(&mut Runner::new(&order, &scripts, &mut state), raider, true);
    assert_eq!(state.globals.get(&global), Some(&1.0));
    npc_combat::combat_over(&mut state, raider);
    state.combat.insert(raider, PLAYER_REF);
    npc_combat::start_combat_event(&mut Runner::new(&order, &scripts, &mut state), raider, true);
    assert_eq!(state.globals.get(&global), Some(&2.0));
}

#[test]
fn get_should_attack_asks_the_attack_rule() {
    let (_data, order) = order("npcc-should");
    let state = GameState::new(&order);
    let facts = Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    let f = script::functions::FUNCTIONS
        .iter()
        .position(|f| f.name == "GetShouldAttack")
        .unwrap() as u16;
    let player = [Value::Form(PLAYER_REF)];
    // The raider's faction is the player's enemy: 100. The guard is
    // aggressive but neutral to the player: 0.
    assert_eq!(
        facts.value(f, Some(FormId(RAIDER_REF)), &player),
        Some(100.0)
    );
    assert_eq!(facts.value(f, Some(FormId(GUARD_REF)), &player), Some(0.0));
    // Not an actor: 0.
    let cell = [Value::Form(FormId(CELL))];
    assert_eq!(facts.value(f, Some(FormId(RAIDER_REF)), &cell), Some(0.0));
}

#[test]
fn set_unconscious_ends_their_fight() {
    let (_data, order) = order("npcc-unconscious");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let raider = FormId(RAIDER_REF);
    state.combat.insert(raider, PLAYER_REF);
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestRaiderRef.SetUnconscious 1",
        None,
        None,
    );
    assert!(state.unconscious.contains(&raider));
    assert!(!state.combat.contains_key(&raider));
    Runner::new(&order, &scripts, &mut state).run_source(
        "TestRaiderRef.SetUnconscious 0",
        None,
        None,
    );
    assert!(!state.unconscious.contains(&raider));
}
