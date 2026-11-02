//! Sleeping, hardcore needs, pickpocketing and trespass warnings
//! (`world::living`) on `testdata::living`'s world.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::living::ids::*;
use world::dialogue::PLAYER_REF;
use world::living::{needs, pickpocket, sleep, trespass};
use world::scripting::{Event, Facts, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::living::living(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A new game with the player in `TestHome`, north of its people.
fn at_home(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(HOME));
    state.player_world = None;
    state.player_position = Some([0.0, 100.0, 0.0]);
    state
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, source: &str) {
    Runner::new(order, scripts, state).run_source(source, None, None);
}

/// An expression's value, as a script sets a global to it.
fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(VALUE), -12345.0);
    run(order, scripts, state, &format!("set TestValue to {expr}"));
    state.globals[&FormId(VALUE)]
}

/// The notices queued since last asked (messages without buttons).
fn notices(state: &mut GameState) -> Vec<String> {
    let mut out = Vec::new();
    state.events.retain(|e| match e {
        Event::Message { text, buttons, .. } if buttons.is_empty() => {
            out.push(text.clone());
            false
        }
        _ => true,
    });
    out
}

fn av(order: &LoadOrder, state: &GameState, who: FormId, value: u16) -> f64 {
    Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(who, value)
    .unwrap()
}

fn has_spell(state: &GameState, spell: u32) -> bool {
    world::magic::is_target_of(state, PLAYER_REF, FormId(spell))
}

#[test]
fn stages_are_kept_highest_first_and_reached_at_their_thresholds() {
    let (_data, order) = order("living-stages");
    let list = needs::stages(&order, needs::DEHYDRATION);
    let thresholds: Vec<u32> = list.iter().map(|s| s.threshold).collect();
    // Written 400, 200, 1000: kept highest first.
    assert_eq!(thresholds, [1000, 400, 200]);
    let at = |v: f64| needs::stage_at(&list, v).map(|s| s.record.0);
    assert_eq!(at(199.0), None);
    assert_eq!(at(200.0), Some(WATER_200));
    assert_eq!(at(450.0), Some(WATER_400));
    assert_eq!(at(1000.0), Some(WATER_1000));
    assert_eq!(list[1].spell, FormId(DEHYDRATION_400));
    // Radiation's stages are read the same way.
    assert_eq!(needs::stages(&order, needs::RADS).len(), 1);
    assert!(needs::stages(&order, 7).is_empty());
}

#[test]
fn game_days_passed_moves_with_the_clock() {
    let (_data, order) = order("living-days");
    let mut state = at_home(&order);
    // 120 real seconds at TimeScale 30: an hour, a 24th of a day.
    state.advance_clock(&order, 120.0);
    let days = state.global(&order, "GameDaysPassed").unwrap();
    assert!((days - (5.0 + 1.0 / 24.0)).abs() < 1e-5, "{days}");
    assert!((state.global(&order, "GameHour").unwrap() - 11.0).abs() < 1e-4);
}

#[test]
fn needs_grow_a_point_per_rate_seconds_of_game_time_in_hardcore() {
    let (_data, order) = order("living-grow");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    let need = |state: &GameState, v: u16| av(&order, state, PLAYER_REF, v);
    // Not in hardcore: nothing grows.
    Runner::new(&order, &scripts, &mut state).update(60.0);
    assert_eq!(need(&state, needs::DEHYDRATION), 0.0);
    run(&order, &scripts, &mut state, "SetHardcore 1");
    assert_eq!(ask(&order, &scripts, &mut state, "player.IsHardcore"), 1.0);
    // The first frame sets the clocks.
    Runner::new(&order, &scripts, &mut state).update(0.0);
    // 32 real seconds = 16 game minutes: dehydration a point per 5 game
    // minutes (`fHCDehydrationRate` 10, the exe's) = 3, hunger per 12.5
    // (25) = 1, sleep per 25 (50) = 0.
    Runner::new(&order, &scripts, &mut state).update(32.0);
    assert_eq!(need(&state, needs::DEHYDRATION), 3.0);
    assert_eq!(need(&state, needs::HUNGER), 1.0);
    assert_eq!(need(&state, needs::SLEEP_DEPRIVATION), 0.0);
    // The fraction is dropped: 1 + 4.5 minutes would be another point.
    Runner::new(&order, &scripts, &mut state).update(9.0);
    assert_eq!(need(&state, needs::DEHYDRATION), 3.0);
    // With the Pip-Boy turned off nothing grows, and the clocks wait.
    run(
        &order,
        &scripts,
        &mut state,
        "DisablePlayerControls 0 1 0 0 0 0 0",
    );
    Runner::new(&order, &scripts, &mut state).update(60.0);
    assert_eq!(need(&state, needs::DEHYDRATION), 3.0);
    run(&order, &scripts, &mut state, "EnablePlayerControls");
    Runner::new(&order, &scripts, &mut state).update(0.0);
    // 4.5 + 30 game minutes since the clock: 6 points.
    assert_eq!(need(&state, needs::DEHYDRATION), 9.0);
}

#[test]
fn a_stage_reached_gives_its_spell_and_says_so() {
    let (_data, order) = order("living-stage-change");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    run(&order, &scripts, &mut state, "SetHardcore 1");
    notices(&mut state);
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue Dehydration 250",
    );
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 250.0);
    assert!(has_spell(&state, DEHYDRATION_200));
    // Minor Dehydration: Endurance −1.
    assert_eq!(av(&order, &state, PLAYER_REF, 7), 4.0);
    assert_eq!(
        notices(&mut state),
        [
            "Your dehydration level has increased.",
            "You are now sick with Minor Dehydration"
        ]
    );
    // The next stage replaces it.
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue Dehydration 200",
    );
    assert!(has_spell(&state, DEHYDRATION_400) && !has_spell(&state, DEHYDRATION_200));
    assert_eq!(av(&order, &state, PLAYER_REF, 7), 3.0);
    notices(&mut state);
    // Drinking lowers it (Restore through the value's flag 0x200): 400 is
    // still the second stage, so nothing's said; 350 is the first.
    state.items.insert((PLAYER_REF, FormId(WATER)), 2);
    world::items::use_item(&order, &mut state, PLAYER_REF, FormId(WATER)).unwrap();
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 400.0);
    assert!(notices(&mut state).is_empty());
    world::items::use_item(&order, &mut state, PLAYER_REF, FormId(WATER)).unwrap();
    assert!(has_spell(&state, DEHYDRATION_200) && !has_spell(&state, DEHYDRATION_400));
    assert_eq!(
        notices(&mut state),
        [
            "Your dehydration level has decreased.",
            "You are now sick with Minor Dehydration"
        ]
    );
    run(
        &order,
        &scripts,
        &mut state,
        "player.RestoreActorValue Dehydration 1000",
    );
    assert!(!has_spell(&state, DEHYDRATION_200));
    assert_eq!(
        notices(&mut state),
        [
            "Your dehydration level has decreased.",
            "You no longer have dehydration sickness."
        ]
    );
    // Hunger's texts aren't in the data: the exe's own.
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue Hunger 200",
    );
    assert_eq!(
        notices(&mut state),
        [
            "Your Hunger level has increased",
            "You are now sick with Minor Starvation"
        ]
    );
    // Rads have their stages too.
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue RadiationRads 300",
    );
    assert!(has_spell(&state, RADS_200));
    // The last stage kills.
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue Dehydration 1000",
    );
    assert!(has_spell(&state, DEHYDRATION_1000));
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert!(state.dead.contains(&PLAYER_REF));
}

#[test]
fn hardcore_off_takes_the_needs_away() {
    let (_data, order) = order("living-hardcore-off");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.IsAlwaysHardcore"),
        1.0
    );
    run(&order, &scripts, &mut state, "SetHardcore 1");
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue Dehydration 300",
    );
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue SleepDeprevation 250",
    );
    assert!(has_spell(&state, DEHYDRATION_200) && has_spell(&state, SLEEP_200));
    // Only 0 and 1 are taken.
    run(&order, &scripts, &mut state, "SetHardcore 2");
    assert!(state.living.hardcore);
    run(&order, &scripts, &mut state, "SetHardcore 0");
    assert!(!state.living.hardcore);
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 0.0);
    assert_eq!(
        av(&order, &state, PLAYER_REF, needs::SLEEP_DEPRIVATION),
        0.0
    );
    assert!(!has_spell(&state, DEHYDRATION_200) && !has_spell(&state, SLEEP_200));
    assert_eq!(
        ask(&order, &scripts, &mut state, "player.IsAlwaysHardcore"),
        0.0
    );
}

#[test]
fn beds_refuse_as_the_game_does() {
    let (_data, order) = order("living-beds");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    let may = |state: &GameState, bed: u32| sleep::may_sleep_in(&order, state, FormId(bed));
    assert!(sleep::is_bed(&order, FormId(FREE_BED_REF)));
    assert!(!sleep::is_bed(&order, FormId(CHAIR_REF)));
    assert_eq!(may(&state, FREE_BED_REF), Ok(()));
    assert_eq!(
        may(&state, OWNED_BED_REF),
        Err("You cannot sleep in an owned bed.".to_string())
    );
    // A faction's bed: any member may, whatever rank it asks (`XRNK`).
    assert!(may(&state, TOWN_BED_REF).is_err());
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddToFaction TestTown 0",
    );
    assert_eq!(may(&state, TOWN_BED_REF), Ok(()));
    // Someone fighting the player: the data's words.
    state.combat.insert(FormId(MARK_REF), PLAYER_REF);
    assert_eq!(
        may(&state, FREE_BED_REF),
        Err("You cannot sleep when enemies are nearby.".to_string())
    );
    state.combat.clear();
    // An effect hurting Health.
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestPoison",
    );
    assert_eq!(
        may(&state, FREE_BED_REF),
        Err("You cannot sleep while taking health damage.".to_string())
    );
    state.active_effects.clear();
    // The barracks forbid waiting (record flag 0x80000).
    state.player_cell = Some(FormId(BARRACKS));
    assert_eq!(
        may(&state, BARRACKS_BED_REF),
        Err("You cannot wait in this location.".to_string())
    );
    // In someone's inn, in the player's own bed: trespassing.
    state.player_cell = Some(FormId(INN));
    assert_eq!(
        may(&state, INN_BED_REF),
        Err("You cannot sleep while trespassing.".to_string())
    );
}

#[test]
fn waiting_is_refused_as_the_game_refuses_it() {
    let (_data, order) = order("living-wait");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    assert_eq!(sleep::may_wait(&order, &state), Ok(()));
    state.player_cell = Some(FormId(INN));
    assert_eq!(
        sleep::may_wait(&order, &state),
        Err("You cannot wait while trespassing.".to_string())
    );
    state.player_cell = Some(FormId(SALOON));
    assert_eq!(sleep::may_wait(&order, &state), Ok(()));
    state.player_cell = Some(FormId(BARRACKS));
    assert_eq!(
        sleep::may_wait(&order, &state),
        Err("You cannot wait in this location.".to_string())
    );
    state.player_cell = Some(FormId(HOME));
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestPoison",
    );
    assert!(state.wait(&order, 1.0).is_err());
}

#[test]
fn a_sleep_heals_in_full_outside_hardcore() {
    let (_data, order) = order("living-sleep");
    let mut state = at_home(&order);
    // Endurance 9: Heal Rate 10 (`fAVDHealRateEndurance9Bonus`), 10 ÷ 30 a
    // game hour.
    state.actor_values.insert((PLAYER_REF, 7), 9.0);
    state.damage.insert(PLAYER_REF, 50.0);
    state.value_damage.insert((PLAYER_REF, 25), 30.0);
    sleep::begin(&mut state, 3, true);
    assert_eq!(world::stats::get(&state, world::stats::TIMES_SLEPT), 1);
    assert!(!sleep::pass_hour(&order, &mut state));
    assert!((state.damage[&PLAYER_REF] - (50.0 - 1.0 / 3.0)).abs() < 1e-4);
    assert!((state.global(&order, "GameHour").unwrap() - 11.0).abs() < 1e-4);
    // The menu's clock reads the same globals (`007c0000`).
    let clock = sleep::Clock::now(&order, &state);
    assert_eq!(clock.twelve_hour(), (11, 0, false));
    assert_eq!(
        clock.weekday(),
        (clock.days_passed.round() as i64).rem_euclid(7) as usize
    );
    sleep::pass_hour(&order, &mut state);
    assert!(sleep::pass_hour(&order, &mut state));
    // Rested: Health in full, the body part whole.
    assert!(!state.damage.contains_key(&PLAYER_REF));
    assert!(!state.value_damage.contains_key(&(PLAYER_REF, 25)));
    assert!(!state.living.sleeping);

    // Cancelled half way: no rest.
    state.damage.insert(PLAYER_REF, 50.0);
    sleep::begin(&mut state, 5, true);
    sleep::pass_hour(&order, &mut state);
    sleep::cancel(&mut state);
    assert!(state.damage[&PLAYER_REF] > 49.0);
    // Waiting isn't sleeping.
    state.wait(&order, 3.0).unwrap();
    assert!(state.damage[&PLAYER_REF] > 48.0);
    assert_eq!(world::stats::get(&state, world::stats::TIMES_SLEPT), 2);
}

#[test]
fn in_hardcore_sleep_only_lowers_sleep_deprivation() {
    let (_data, order) = order("living-sleep-hardcore");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    run(&order, &scripts, &mut state, "SetHardcore 1");
    state.damage.insert(PLAYER_REF, 50.0);
    run(
        &order,
        &scripts,
        &mut state,
        "player.DamageActorValue SleepDeprevation 100",
    );
    // A frame of play sets the needs' clocks.
    needs::grow(&order, &mut state);
    sleep::begin(&mut state, 2, true);
    sleep::pass_hour(&order, &mut state);
    // 100 − min(100, 60); nothing added while asleep.
    assert_eq!(
        av(&order, &state, PLAYER_REF, needs::SLEEP_DEPRIVATION),
        40.0
    );
    // The needs grow before each hour passes, so an hour's points come
    // with the next: none yet, then 12 dehydration an hour.
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 0.0);
    sleep::pass_hour(&order, &mut state);
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 12.0);
    assert_eq!(
        av(&order, &state, PLAYER_REF, needs::SLEEP_DEPRIVATION),
        0.0
    );
    // No rest at the end in hardcore.
    assert_eq!(state.damage[&PLAYER_REF], 50.0);
    // The last hour's needs come with the next frame of play: 11, as the
    // clock is a float (`GameDaysPassed` 5.0833330 × 1440 = 7319.9995
    // minutes, 59.9995 since the clock: 11.9999 points, cut to 11).
    needs::grow(&order, &mut state);
    assert_eq!(av(&order, &state, PLAYER_REF, needs::DEHYDRATION), 23.0);
    assert_eq!(
        av(&order, &state, PLAYER_REF, needs::SLEEP_DEPRIVATION),
        2.0
    );
}

#[test]
fn the_bed_script_makes_the_player_well_rested() {
    let (_data, order) = order("living-well-rested");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    run(&order, &scripts, &mut state, "SetHardcore 1");
    let bed = FormId(SCRIPTED_BED_REF);
    let use_bed = |state: &mut GameState| {
        let mut r = Runner::new(&order, &scripts, state);
        r.run_event(bed, "onactivate", PLAYER_REF);
    };
    // Opening the menu and leaving gives nothing.
    use_bed(&mut state);
    sleep::menu_mode(&mut Runner::new(&order, &scripts, &mut state), &[bed]);
    Runner::new(&order, &scripts, &mut state).run_blocks(bed, Some(bed), "gamemode", |_| true);
    assert!(!has_spell(&state, WELL_RESTED));
    // A sleep in it: Well Rested, which heals in full even in hardcore.
    state.damage.insert(PLAYER_REF, 60.0);
    use_bed(&mut state);
    sleep::begin(&mut state, 1, true);
    assert_eq!(ask(&order, &scripts, &mut state, "IsPCSleeping"), 1.0);
    assert_eq!(ask(&order, &scripts, &mut state, "GetPCSleepHours"), 1.0);
    sleep::menu_mode(&mut Runner::new(&order, &scripts, &mut state), &[bed]);
    assert!(sleep::pass_hour(&order, &mut state));
    assert_eq!(ask(&order, &scripts, &mut state, "IsPCSleeping"), 0.0);
    Runner::new(&order, &scripts, &mut state).run_blocks(bed, Some(bed), "gamemode", |_| true);
    assert!(has_spell(&state, WELL_RESTED));
    // The effect starts as it's cast (`MagicTarget::CheckAddEffect` starts
    // what it adds): its `ScriptEffectStart` has run.
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.damage.get(&PLAYER_REF).copied().unwrap_or(0.0), 0.0);
    assert!(state.perks.contains(&FormId(WELL_RESTED_PERK)));
    assert_eq!(notices(&mut state), ["You are now Well Rested!"]);
}

#[test]
fn scripts_can_set_the_hours_left() {
    let (_data, order) = order("living-sleep-hours");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    sleep::begin(&mut state, 8, false);
    // `SetPCSleepHours` sets the hours and the sleeping flag
    // (`005c1a00(n, 1)`), as the timer scripts cut a sleep short.
    run(&order, &scripts, &mut state, "SetPCSleepHours 1");
    assert_eq!(state.living.hours_left, 1);
    assert!(state.living.sleeping);
    run(&order, &scripts, &mut state, "ShowSleepWaitMenu 1 0");
    assert!(state.events.contains(&Event::SleepWaitMenu { sleep: true }));
}

#[test]
fn using_a_person_picks_their_pockets_while_sneaking() {
    let (_data, order) = order("living-use-person");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    let mark = FormId(MARK_REF);
    let used = |state: &GameState, who: FormId| pickpocket::use_person(&order, state, who, false);
    assert_eq!(used(&state, mark), pickpocket::Use::Talk);
    state.player_sneaking = true;
    assert_eq!(used(&state, mark), pickpocket::Use::Pickpocket);
    // Not the player's companions: their wheel of orders (`005fa330`).
    run(&order, &scripts, &mut state, "PalRef.SetPlayerTeammate 1");
    assert_eq!(used(&state, FormId(PAL_REF)), pickpocket::Use::Wheel);
    assert_eq!(
        pickpocket::use_person(&order, &state, mark, true),
        pickpocket::Use::Refused("Owner is fleeing.".into(), world::message_icon::SAD)
    );
    state.living.caught_by.insert(mark);
    assert_eq!(
        used(&state, mark),
        pickpocket::Use::Refused(
            "Owner has already caught you.".into(),
            world::message_icon::SAD
        )
    );
    run(&order, &scripts, &mut state, "MarkRef.SetUnconscious 1");
    assert_eq!(
        used(&state, mark),
        pickpocket::Use::Refused(
            "Owner is unconscious.".into(),
            world::message_icon::SURPRISED
        )
    );
    state.dead.insert(FormId(VILLAIN_REF));
    assert_eq!(used(&state, FormId(VILLAIN_REF)), pickpocket::Use::Search);
}

#[test]
fn the_chance_follows_the_games_settings() {
    let (_data, order) = order("living-chance");
    let mut state = at_home(&order);
    let mark = FormId(MARK_REF);
    state.actor_values.insert((PLAYER_REF, 42), 50.0);
    // 40 + 0.6 × 50 − 0.6 × 20 − 0.5 × V.
    let money = pickpocket::stack_value(&order, &state, FormId(MONEY), 1);
    assert_eq!(money, 10.0);
    assert_eq!(pickpocket::chance(&order, &state, mark, money), 53);
    assert_eq!(
        pickpocket::stack_value(&order, &state, FormId(MONEY), 3),
        30.0
    );
    // Something dear: at least 5.
    let gold = pickpocket::stack_value(&order, &state, FormId(GOLD), 1);
    assert_eq!(pickpocket::chance(&order, &state, mark, gold), 5);
    // Placing anything is V = 1: 57.5, cut to 57.
    assert_eq!(pickpocket::chance(&order, &state, mark, 1.0), 57);
    // At most 85.
    state.actor_values.insert((PLAYER_REF, 42), 100.0);
    assert_eq!(pickpocket::chance(&order, &state, mark, 1.0), 85);
}

#[test]
fn a_theft_costs_karma_and_getting_caught_is_a_crime() {
    let (_data, order) = order("living-pickpocket");
    let mut state = at_home(&order);
    state.player_sneaking = true;
    state.actor_values.insert((PLAYER_REF, 42), 50.0);
    let mark = FormId(MARK_REF);
    let karma = |state: &GameState| world::reputation::karma(&order, state);
    let mut visit = pickpocket::Visit::new(mark);
    // Chance 53: a roll of 52 succeeds.
    let moved = pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, true, 52);
    assert_eq!(moved.count, 1);
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(MONEY)), 1);
    assert_eq!(karma(&state), -5.0);
    assert_eq!(world::stats::get(&state, world::stats::POCKETS_PICKED), 1);
    pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, true, 0);
    // Counted once a visit.
    assert_eq!(world::stats::get(&state, world::stats::POCKETS_PICKED), 1);
    // Nothing of value: no roll, no karma.
    let pencil = pickpocket::attempt(&order, &mut state, &mut visit, FormId(PENCIL), 1, true, 99);
    assert_eq!(pencil.count, 1);
    assert_eq!(karma(&state), -10.0);
    // Putting something back (V = 1, chance 57), then taking it back: no
    // roll for the player's own.
    let placed = pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, false, 56);
    assert_eq!(placed.count, 1);
    assert_eq!(karma(&state), -15.0);
    let back = pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, true, 99);
    assert_eq!(back.count, 1);
    assert_eq!(karma(&state), -15.0);
    // Caught: nothing moves, the crime, and no more tries.
    notices(&mut state);
    let before = state.item_count(&order, mark, FormId(MONEY));
    let caught = pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, true, 53);
    assert!(caught.caught && caught.count == 0);
    assert_eq!(state.item_count(&order, mark, FormId(MONEY)), before);
    assert!(state.living.caught_by.contains(&mark));
    assert!(notices(&mut state).contains(&"You've been caught pickpocketing.".to_string()));
    assert_eq!(state.faction_crimes[&FormId(TOWN)], (1, 0));
    assert_eq!(world::reputation::get(&state, FormId(TOWN_REP), 0), 2.0);

    let after = pickpocket::attempt(&order, &mut state, &mut visit, FormId(MONEY), 1, true, 0);
    assert_eq!(after.count, 0);
    // A second crime the same day: they attack.
    let mut again = pickpocket::Visit::new(mark);
    pickpocket::attempt(&order, &mut state, &mut again, FormId(MONEY), 1, true, 99);
    assert_eq!(state.combat.get(&mark), Some(&PLAYER_REF));
    // An evil mark costs no karma.
    let k = karma(&state);
    let mut villain = pickpocket::Visit::new(FormId(VILLAIN_REF));
    pickpocket::attempt(&order, &mut state, &mut villain, FormId(MONEY), 1, true, 0);
    assert_eq!(karma(&state), k);
    // Saved and loaded.
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert!(loaded.living.caught_by.contains(&mark));

    assert!(loaded.living.always_hardcore);
}

#[test]
fn thrown_weapons_with_lobbed_projectiles_would_go_off() {
    let (_data, order) = order("living-grenade");
    // Animation type 10–13 and a lobber (`PROJ` type 2): the game arms
    // these when slipped into a pocket (not done here).
    assert!(pickpocket::is_live_grenade(&order, FormId(GRENADE)));
    assert!(!pickpocket::is_live_grenade(&order, FormId(PISTOL)));
    assert!(!pickpocket::is_live_grenade(&order, FormId(MONEY)));
}

/// The player in the inn, north of its owner (150 away), the guard and a
/// stranger (180 away).
fn in_the_inn(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(INN));
    state.player_world = None;
    state.player_position = Some([0.0, 150.0, 0.0]);
    state
}

fn warned(state: &GameState, by: u32) -> usize {
    state
        .events
        .iter()
        .filter(|e| {
            matches!(e, Event::Talk { speaker, topic: Some(t), .. }
                if *speaker == FormId(by) && *t == trespass::GUARD_TRESPASS)
        })
        .count()
}

#[test]
fn trespassers_are_warned_three_times_then_attacked() {
    let (_data, order) = order("living-trespass");
    let scripts = ScriptCache::default();
    let mut state = in_the_inn(&order);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    // The nearest who sees comes (already near), and warns at once.
    let w = state.living.trespass.clone().unwrap();
    assert_eq!(w.warner, FormId(INN_OWNER_REF));
    assert_eq!(w.allowed, 3);
    assert_eq!(warned(&state, INN_OWNER_REF), 1);
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "InnOwnerRef.GetTrespassWarningLevel"
        ),
        1.0
    );
    assert_eq!(
        ask(
            &order,
            &scripts,
            &mut state,
            "InnGuardRef.GetTrespassWarningLevel"
        ),
        -1.0
    );
    // Their package takes them to the player.
    let p = world::ai::current_package(&order, &state, FormId(INN_OWNER_REF)).unwrap();
    assert_eq!(p.kind, trespass::PACKAGE_TYPE);
    assert_eq!(p.location.unwrap().form, PLAYER_REF);
    // Every 10 s (`fAITrespassWarningTimer`) another, four in all, then
    // at 40 s the alarm.
    for _ in 0..390 {
        Runner::new(&order, &scripts, &mut state).update(0.1);
    }
    assert_eq!(warned(&state, INN_OWNER_REF), 4);
    assert!(state.combat.is_empty());
    for _ in 0..15 {
        Runner::new(&order, &scripts, &mut state).update(0.1);
    }
    assert!(state.living.trespass.is_none());
    // Those who care turn on the player (the owner, the guard of the
    // owner's town); the stranger doesn't.
    assert_eq!(state.combat.get(&FormId(INN_OWNER_REF)), Some(&PLAYER_REF));
    assert_eq!(state.combat.get(&FormId(INN_GUARD_REF)), Some(&PLAYER_REF));
    assert!(!state.combat.contains_key(&FormId(INN_STRANGER_REF)));
    assert_eq!(state.faction_crimes[&FormId(TOWN)], (1, 0));
}

#[test]
fn leaving_ends_the_warning_and_off_limits_places_warn_no_one() {
    let (_data, order) = order("living-trespass-leave");
    let scripts = ScriptCache::default();
    let mut state = in_the_inn(&order);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert!(state.living.trespass.is_some());
    state.player_cell = Some(FormId(HOME));
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert!(state.living.trespass.is_none());
    // Too far to be seen: nobody comes.
    let mut far = in_the_inn(&order);
    far.player_position = Some([0.0, 2400.0, 0.0]);
    Runner::new(&order, &scripts, &mut far).update(0.1);
    assert!(far.living.trespass.is_none());
    // A public place isn't trespassing.
    let mut saloon = in_the_inn(&order);
    saloon.player_cell = Some(FormId(SALOON));
    Runner::new(&order, &scripts, &mut saloon).update(0.1);
    assert!(saloon.living.trespass.is_none());
    // Off limits: the alarm at once.
    let mut off = in_the_inn(&order);
    off.player_cell = Some(FormId(OFF_LIMITS));
    Runner::new(&order, &scripts, &mut off).update(0.1);
    assert_eq!(
        off.combat.get(&FormId(OFF_LIMITS_GUARD_REF)),
        Some(&PLAYER_REF)
    );
    assert_eq!(warned(&off, OFF_LIMITS_GUARD_REF), 0);
}

#[test]
fn scripts_can_sound_the_trespass_alarm() {
    let (_data, order) = order("living-trespass-alarm");
    let scripts = ScriptCache::default();
    let mut state = in_the_inn(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "InnGuardRef.SendTrespassAlarm player",
    );
    assert_eq!(state.combat.get(&FormId(INN_GUARD_REF)), Some(&PLAYER_REF));
    assert_eq!(state.combat.get(&FormId(INN_OWNER_REF)), Some(&PLAYER_REF));
    assert!(!state.combat.contains_key(&FormId(INN_STRANGER_REF)));
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
}

#[test]
fn activate_with_its_flag_runs_the_on_activate_block() {
    let (_data, order) = order("living-activate");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    let sleep_var =
        |state: &mut GameState| ask(&order, &scripts, state, "ScriptedBedRef.playerSleep");
    // Without the flag the usual thing happens, the block doesn't run.
    run(
        &order,
        &scripts,
        &mut state,
        "ScriptedBedRef.Activate player",
    );
    assert_eq!(sleep_var(&mut state), 0.0);
    // With it the block runs (as `VCG01` asks about hardcore mode), and
    // the block's own `Activate` does the usual thing.
    state.events.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "ScriptedBedRef.Activate player 1",
    );
    assert_eq!(sleep_var(&mut state), 1.0);
    assert!(state.events.contains(&Event::Activate {
        what: FormId(SCRIPTED_BED_REF),
        by: None,
    }));
}

#[test]
fn the_crosshair_is_red_for_what_the_player_may_not_take() {
    let (_data, order) = order("living-red");
    let mut state = at_home(&order);
    let red =
        |state: &GameState, r: u32| world::living::crosshair_red(&order, state, FormId(r)).unwrap();
    assert!(red(&state, OWNED_BED_REF));
    assert!(!red(&state, FREE_BED_REF));
    // Someone else's chair too (sitting in it is never refused, but it
    // falls under the owned rule all the same).
    assert!(red(&state, CHAIR_REF));
    // People while the player sneaks.
    assert!(!red(&state, MARK_REF));
    state.player_sneaking = true;
    assert!(red(&state, MARK_REF));
    state.dead.insert(FormId(MARK_REF));
    assert!(!red(&state, MARK_REF));
    // Nothing while the player is dead.
    state.dead.insert(PLAYER_REF);
    assert!(!red(&state, OWNED_BED_REF));
}

#[test]
fn the_warner_walks_up_before_warning() {
    let (_data, order) = order("living-trespass-walk");
    let scripts = ScriptCache::default();
    let mut state = in_the_inn(&order);
    // Seen from 600 units: the owner comes, but says nothing yet.
    state.player_position = Some([0.0, 600.0, 0.0]);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(
        state.living.trespass.as_ref().map(|w| w.warner),
        Some(FormId(INN_OWNER_REF))
    );
    for _ in 0..50 {
        Runner::new(&order, &scripts, &mut state).update(0.1);
    }
    assert_eq!(warned(&state, INN_OWNER_REF), 0);
    // Once near (here the owner is moved, as the viewer walks them), the
    // first warning.
    state
        .positions
        .insert(FormId(INN_OWNER_REF), ([0.0, 450.0, 0.0], 0.0));
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(warned(&state, INN_OWNER_REF), 1);
}

#[test]
fn hardcore_is_saved() {
    let (_data, order) = order("living-save");
    let scripts = ScriptCache::default();
    let mut state = at_home(&order);
    run(&order, &scripts, &mut state, "SetHardcore 1");
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert!(loaded.living.hardcore);
    assert!(loaded.living.always_hardcore);
    run(&order, &scripts, &mut state, "DisableHardcoreTracking");
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert!(loaded.living.hardcore && !loaded.living.always_hardcore);
}
