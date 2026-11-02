//! The script functions in `world::more_functions`, each as the game's own
//! handler carries it out (`testdata::more` builds the world). A function
//! not carried out stops the script, so each check fails without it.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::more::ids::*;
use world::dialogue::PLAYER_REF;
use world::more_functions::{self as more, movement, SaveKind, Seen, Shown};
use world::radio::{RadioEvent, RadioFiles};
use world::scripting::{Event, GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::more::more(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// Not a value any function here gives: what `ask` returns when the
/// script stopped (the function wasn't carried out).
const STOPPED: f32 = -12345.0;

fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(VALUE), STOPPED);
    Runner::new(order, scripts, state).run_source(&format!("set TestValue to {expr}"), None, None);
    state.globals[&FormId(VALUE)]
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, source: &str) {
    Runner::new(order, scripts, state).run_source(source, None, None);
}

fn new_game(order: &LoadOrder) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(ROOM));
    state.player_world = None;
    state.player_position = Some([0.0, 100.0, 0.0]);
    state
}

#[test]
fn ghosts_have_no_reaction_to_hits() {
    let (_data, order) = order("more-ghost");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    assert_eq!(q(&mut state, "PersonRef.GetIsGhost"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetGhost 1\nBarrelRef.SetGhost 1",
    );
    assert_eq!(q(&mut state, "PersonRef.GetIsGhost"), 1.0);
    // Not an actor: nothing.
    assert_eq!(q(&mut state, "BarrelRef.GetIsGhost"), 0.0);
    assert!(state.events.contains(&Event::More(Shown::Ghost {
        who: FormId(PERSON_REF),
        on: true
    })));
    // Hit, the ghost takes nothing and doesn't fight back; the dog does.
    let hit = Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, FormId(PERSON_REF), None);
    assert_eq!(hit, None);
    assert!(!state.combat.contains_key(&FormId(PERSON_REF)));
    assert!(!state.damage.contains_key(&FormId(PERSON_REF)));
    Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, FormId(DOG_REF), None);
    assert_eq!(state.combat.get(&FormId(DOG_REF)), Some(&PLAYER_REF));
    run(&order, &scripts, &mut state, "PersonRef.SetGhost 0");
    assert_eq!(q(&mut state, "PersonRef.GetIsGhost"), 0.0);
}

#[test]
fn essential_people_go_down_instead_of_dying_and_get_up() {
    let (_data, order) = order("more-flags");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let who = FormId(PERSON_REF);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorRefEssential 1",
    );
    // A blow far past their health: down, not dead.
    let dead = world::combat::hurt(&order, &mut state, who, 100000.0, PLAYER_REF);
    assert!(!dead);
    assert!(!state.dead.contains(&who));
    assert!(state.more.down.contains_key(&who));
    // Down they lie knocked out: `GetKnockedState` (`005a08c0`) is 1 for
    // knock states 3 and 4 (`world::fatigue`).
    assert_eq!(
        ask(&order, &scripts, &mut state, "PersonRef.GetKnockedState"),
        1.0
    );
    // Down, they take no more harm while essential.
    assert!(!world::combat::hurt(
        &order, &mut state, who, 100000.0, PLAYER_REF
    ));
    // Down, their health is already restored (`008a0960`); the exe's 10 s
    // (`fEssentialDeathTime`) pass and they get up with all of it.
    let full = world::combat::max_health(&order, &state, who).unwrap();
    assert_eq!(world::combat::health(&order, &state, who), Some(full));
    assert_eq!(world::combat::essential_down_time(&order), 10.0);
    world::combat::advance_down(&order, &mut state, 9.9);
    assert!(state.more.down.contains_key(&who));
    world::combat::advance_down(&order, &mut state, 0.2);
    assert!(state.more.down.is_empty());
    assert_eq!(world::combat::health(&order, &state, who), Some(full));
    assert_eq!(
        ask(&order, &scripts, &mut state, "PersonRef.GetKnockedState"),
        0.0
    );
    // Down again; a script takes the flag off: the next blow kills.
    world::combat::hurt(&order, &mut state, who, 100000.0, PLAYER_REF);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorRefEssential 0",
    );
    assert!(world::combat::hurt(
        &order, &mut state, who, 100000.0, PLAYER_REF
    ));
    assert!(state.dead.contains(&who));
    assert!(state.more.down.is_empty());
}

#[test]
fn teammates_follow_only_their_own_packages_without_guesses() {
    // The contributor's untraced follow rule is behind `world::guesses`,
    // off here (`contrib_guesses.rs` checks it on).
    let (_data, order) = order("more-flags");
    let mut state = new_game(&order);
    let who = FormId(PERSON_REF);
    state.teammates.insert(who);
    assert!(world::ai::current_package(&order, &state, who)
        .is_none_or(|p| p.kind != world::ai::kinds::FOLLOW));
}

#[test]
fn alpha_alert_essential_and_subtitles() {
    let (_data, order) = order("more-flags");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorAlpha 1.5\nDogRef.SetActorAlpha -1",
    );
    assert_eq!(state.more.alpha[&FormId(PERSON_REF)], 1.0);
    assert_eq!(state.more.alpha[&FormId(DOG_REF)], 0.0);
    assert_eq!(q(&mut state, "PersonRef.GetIsAlerted"), 0.0);
    run(&order, &scripts, &mut state, "PersonRef.SetAlert 1");
    assert_eq!(q(&mut state, "PersonRef.GetIsAlerted"), 1.0);
    // Essential: the hero's record; the person by base, by reference, as a
    // teammate outside hardcore.
    assert_eq!(q(&mut state, "HeroRef.IsEssential"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 0.0);
    run(&order, &scripts, &mut state, "SetEssential TestPerson 1");
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 1.0);
    run(&order, &scripts, &mut state, "SetEssential TestPerson 0");
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 0.0);
    assert_eq!(q(&mut state, "PersonRef.IsActorRefEssential"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorRefEssential 1",
    );
    assert_eq!(q(&mut state, "PersonRef.IsActorRefEssential"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 1.0);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorRefEssential 0",
    );
    state.teammates.insert(FormId(PERSON_REF));
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 1.0);
    state.living.hardcore = true;
    assert_eq!(q(&mut state, "PersonRef.IsEssential"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.AlwaysShowActorSubtitles 1",
    );
    assert!(state.more.always_subtitles.contains(&FormId(PERSON_REF)));
}

#[test]
fn talking_activators_radios_and_combat_styles() {
    let (_data, order) = order("more-tact");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    run(
        &order,
        &scripts,
        &mut state,
        "TalkerRef.SetTalkingActivatorActor PersonRef",
    );
    assert_eq!(
        q(&mut state, "PersonRef.IsActorTalkingThroughActivator"),
        0.0
    );
    state.speaking.insert(FormId(TALKER_REF));
    assert_eq!(
        q(&mut state, "PersonRef.IsActorTalkingThroughActivator"),
        1.0
    );
    assert_eq!(q(&mut state, "HeroRef.IsActorTalkingThroughActivator"), 0.0);
    // The station's record broadcasts; switched off and on again.
    assert_eq!(q(&mut state, "RadioRef.GetBroadcastState"), 1.0);
    assert_eq!(q(&mut state, "TalkerRef.GetBroadcastState"), 0.0);
    run(&order, &scripts, &mut state, "RadioRef.SetBroadcastState 0");
    assert_eq!(q(&mut state, "RadioRef.GetBroadcastState"), 0.0);
    // A combat style given to someone is the one they fight with.
    assert_ne!(
        more::combat_style(&order, &state, FormId(PERSON_REF)).form_id,
        Some(FormId(STYLE))
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetCombatStyle TestStyle",
    );
    assert_eq!(
        more::combat_style(&order, &state, FormId(PERSON_REF)).form_id,
        Some(FormId(STYLE))
    );
}

#[test]
fn small_questions() {
    let (_data, order) = order("more-small");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let mut q = |e: &str| ask(&order, &scripts, &mut state, e);
    assert_eq!(q("Sqrt 16"), 4.0);
    assert_eq!(q("PersonRef.IsActor"), 1.0);
    assert_eq!(q("BarrelRef.IsActor"), 0.0);
    assert_eq!(q("IsWin32"), 1.0);
    assert_eq!(q("IsXBox"), 0.0);
    // NPC_ is form type 42, ACTI 21.
    assert_eq!(q("PersonRef.GetIsObjectType 42"), 1.0);
    assert_eq!(q("BarrelRef.GetIsObjectType 21"), 1.0);
    assert_eq!(q("BarrelRef.GetIsObjectType 42"), 0.0);
    assert_eq!(q("ChildRef.GetParentRef"), BARREL_REF as f32);
    assert_eq!(q("BarrelRef.GetParentRef"), 0.0);
    assert_eq!(q("PersonRef.IsLimbGone 3"), 0.0);
    assert_eq!(q("PersonRef.IsLimbGone 1 5"), 0.0);
    assert_eq!(q("player.GetSandman"), 0.0);
    assert_eq!(q("IsPlayerGrabbedRef CupRef"), 0.0);
    // Level 1: 200 experience to level 2 (the exe's iXPBase).
    assert_eq!(q("GetXPForNextLevel"), 200.0);
    assert_eq!(q("PersonRef.GetIgnoreCrime"), 0.0);
    assert_eq!(q("PersonRef.GetIgnoreFriendlyHits"), 0.0);
    assert_eq!(q("GetInCharGen"), 0.0);
    assert_eq!(q("PersonRef.IsInCriticalStage 2"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "SetInChargen 1\nPersonRef.SetCriticalStage 2\nPersonRef.IgnoreCrime 1",
    );
    let mut q = |e: &str| ask(&order, &scripts, &mut state, e);
    assert_eq!(q("GetInCharGen"), 1.0);
    assert_eq!(q("PersonRef.IsInCriticalStage 2"), 1.0);
    assert_eq!(q("PersonRef.GetIgnoreCrime"), 1.0);
}

#[test]
fn what_the_viewer_saw() {
    let (_data, order) = order("more-seen");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // Nothing seen: standing still, no idle, no procedure.
    assert_eq!(q(&mut state, "PersonRef.IsMoving"), 0.0);
    assert_eq!(q(&mut state, "PersonRef.IsLastIdlePlayed TestIdle"), 0.0);
    assert_eq!(q(&mut state, "PersonRef.GetCurrentAIProcedure"), 0.0);
    more::report(
        &mut state,
        FormId(PERSON_REF),
        Seen {
            movement: movement::LEFT | movement::RUNNING,
            last_idle: Some(FormId(IDLE)),
            procedure: Some(10),
            swimming: true,
            idle_playing: true,
        },
    );
    assert_eq!(q(&mut state, "PersonRef.IsIdlePlaying"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.IsMoving"), 3.0);
    assert_eq!(q(&mut state, "PersonRef.IsRunning"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.IsSneaking"), 0.0);
    assert_eq!(q(&mut state, "PersonRef.IsSwimming"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.IsLastIdlePlayed TestIdle"), 1.0);
    assert_eq!(q(&mut state, "PersonRef.GetCurrentAIProcedure"), 10.0);
    // Sneaking, unless the flag that cancels it is up.
    more::report(
        &mut state,
        FormId(PERSON_REF),
        Seen {
            movement: movement::SNEAKING,
            ..Seen::default()
        },
    );
    assert_eq!(q(&mut state, "PersonRef.IsSneaking"), 1.0);
    more::report(
        &mut state,
        FormId(PERSON_REF),
        Seen {
            movement: movement::SNEAKING | movement::NOT_SNEAKING,
            ..Seen::default()
        },
    );
    assert_eq!(q(&mut state, "PersonRef.IsSneaking"), 0.0);
    // The player, before the viewer says: from the state.
    state.player_sneaking = true;
    assert_eq!(q(&mut state, "player.IsSneaking"), 1.0);
    // Menus.
    assert_eq!(q(&mut state, "MenuMode 0"), 0.0);
    state.more.menu_open = Some(1002);
    assert_eq!(q(&mut state, "MenuMode 0"), 1.0);
    assert_eq!(q(&mut state, "MenuMode 1"), 1.0);
    assert_eq!(q(&mut state, "MenuMode 1002"), 1.0);
    assert_eq!(q(&mut state, "MenuMode 1036"), 0.0);
}

#[test]
fn challenges_count_complete_and_recur() {
    let (_data, order) = order("more-chal");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    let done = |state: &GameState| state.globals[&FormId(DONE)];
    for _ in 0..2 {
        run(
            &order,
            &scripts,
            &mut state,
            "IncrementScriptedChallenge TestScripted",
        );
    }
    assert_eq!(q(&mut state, "GetChallengeCompleted TestScripted"), 0.0);
    assert!(state.events.iter().any(|e| matches!(e,
        Event::Message { text, .. } if text == "Scripted   2\\3\nDo it.")));
    run(
        &order,
        &scripts,
        &mut state,
        "IncrementScriptedChallenge TestScripted",
    );
    // Completed: its script ran, the statistic went up, the notice.
    assert_eq!(done(&state), 1.0);
    assert_eq!(q(&mut state, "GetChallengeCompleted TestScripted"), 1.0);
    assert_eq!(world::stats::get(&state, 27), 1);
    assert!(state.events.iter().any(|e| matches!(e,
        Event::Message { text, .. } if text == "Scripted   3\\3\nDo it.")));
    // No more counting once completed.
    run(
        &order,
        &scripts,
        &mut state,
        "IncrementScriptedChallenge TestScripted",
    );
    assert_eq!(done(&state), 1.0);
    // The statistic counted for the challenge about it when scripts next
    // ran.
    assert_eq!(state.more.challenges.progress[&FormId(COUNTING)].0, 1);

    // Recurring: done twice over, counting again each time.
    for _ in 0..2 {
        run(
            &order,
            &scripts,
            &mut state,
            "IncrementScriptedChallenge TestRecurring",
        );
    }
    assert_eq!(done(&state), 2.0);
    assert_eq!(q(&mut state, "GetChallengeCompleted TestRecurring"), 1.0);
    assert_eq!(state.more.challenges.progress[&FormId(RECURRING)].0, 0);
    // That was the second challenge completed: the one counting them
    // completes when scripts next run (its own completion doesn't count
    // for it).
    assert_eq!(done(&state), 3.0);
    assert_eq!(q(&mut state, "GetChallengeCompleted TestCounting"), 1.0);
    // The condition asks the completed flag itself: not set while it recurs.
    let facts = world::scripting::Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    let get_challenge_completed = script::functions::FUNCTIONS
        .iter()
        .position(|f| f.name == "GetChallengeCompleted")
        .unwrap() as u16;
    let arg = [world::scripting::Value::Form(FormId(RECURRING))];
    assert_eq!(facts.value(get_challenge_completed, None, &arg), Some(0.0));
    run(
        &order,
        &scripts,
        &mut state,
        "RemoveRecurringFromChallenge TestRecurring",
    );
    for _ in 0..2 {
        run(
            &order,
            &scripts,
            &mut state,
            "IncrementScriptedChallenge TestRecurring",
        );
    }
    assert_eq!(done(&state), 4.0);
    let facts = world::scripting::Facts {
        order: &order,
        state: &state,
        speaker: None,
    };
    assert_eq!(facts.value(get_challenge_completed, None, &arg), Some(2.0));
    // The counting one, completed, counts no more.
    run(&order, &scripts, &mut state, "set TestValue to 0");
    assert_eq!(done(&state), 4.0);

    // Starting disabled: nothing until unlocked.
    run(
        &order,
        &scripts,
        &mut state,
        "IncrementScriptedChallenge TestLocked",
    );
    assert_eq!(q(&mut state, "GetChallengeCompleted TestLocked"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "UnlockChallenge TestLocked\nIncrementScriptedChallenge TestLocked",
    );
    assert_eq!(q(&mut state, "GetChallengeCompleted TestLocked"), 1.0);

    // Kept in a save.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(
        back.more.challenges.progress,
        state.more.challenges.progress
    );
}

#[test]
fn place_at_me_makes_references() {
    let (_data, order) = order("more-place");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "ref r\nset r to PersonRef.PlaceAtMe TestCup 3\nset TestValue to r.GetIsID TestCup",
    );
    assert_eq!(state.globals[&FormId(VALUE)], 1.0);
    let made: Vec<_> = state
        .more
        .placed
        .refs
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();
    assert_eq!(made.len(), 3);
    let at = |i: usize| made[i].1.position;
    // The first on the caller's spot, then a ring of 100 units, 45 degrees
    // apart, at the caller's height; all in its room, turned as it is.
    assert_eq!(at(0), [0.0, 0.0, 0.0]);
    let d = |p: [f32; 3]| (p[0] * p[0] + p[1] * p[1]).sqrt();
    assert!((d(at(1)) - 100.0).abs() < 1e-3 && (d(at(2)) - 100.0).abs() < 1e-3);
    let a = |p: [f32; 3]| p[0].atan2(p[1]);
    let step = (a(at(2)) - a(at(1))).rem_euclid(std::f32::consts::TAU);
    assert!((step - std::f32::consts::FRAC_PI_4).abs() < 1e-4, "{step}");
    assert!(made.iter().all(|(_, m)| m.base == FormId(CUP)
        && m.cell == FormId(ROOM)
        && (m.rotation[2] - 90f32.to_radians()).abs() < 1e-5));
    let events = state
        .events
        .iter()
        .filter(|e| matches!(e, Event::More(Shown::Placed { .. })))
        .count();
    assert_eq!(events, 3);
    // Where they are, for other functions.
    let third = made[2].0;
    assert_eq!(state.place(&order, third).unwrap().2, at(2));

    // A leveled list: on one spot, 100 units in front of the person (who
    // faces east); from a marker (no model) on the marker's own spot.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.PlaceAtMe TestDogs 1 100 0",
    );
    let dog = state.more.placed.refs.values().last().copied().unwrap();
    assert_eq!(dog.base, FormId(DOG));
    assert!((dog.position[0] - 100.0).abs() < 1e-3 && dog.position[1].abs() < 1e-3);
    run(
        &order,
        &scripts,
        &mut state,
        "MarkerRef.PlaceAtMe TestDogs 1 100 0",
    );
    let dog = state.more.placed.refs.values().last().copied().unwrap();
    assert_eq!(dog.position, [500.0, 0.0, 0.0]);
    // An actor's leveled form, on the caller.
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.PlaceLeveledActorAtMe TestDog",
    );
    let dog = state.more.placed.refs.values().last().copied().unwrap();
    assert_eq!((dog.base, dog.position), (FormId(DOG), [0.0, 200.0, 0.0]));
    // Kept in a save.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.more.placed.refs, state.more.placed.refs);
    assert_eq!(back.more.placed.next, state.more.placed.next);
}

/// The Pip-Boy's Drop (`00780c50`, `009614b0`): the items leave the
/// inventory and lie `fPlayerDropDistance` (plus their size) in front of
/// the player as one reference keeping the count, kept in a save.
#[test]
fn the_player_drops_items_in_front() {
    let (_data, order) = order("more-drop");
    let mut state = new_game(&order);
    state.stock(&order, PLAYER_REF);
    state.items.insert((PLAYER_REF, FormId(CUP)), 5);
    let r = more::placed::drop_item(&order, &mut state, FormId(CUP), 3, 0.5).unwrap();
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CUP)), 2);
    let made = state.more.placed.refs[&r];
    assert_eq!((made.base, made.count), (FormId(CUP), 3));
    let heading = 0.5f32;
    let (dx, dy) = (made.position[0], made.position[1] - 100.0);
    assert!((dx * heading.cos() - dy * heading.sin()).abs() < 1e-3);
    assert!((dx.hypot(dy) - more::placed::DROP_DISTANCE).abs() < 50.0);
    assert!(state
        .events
        .contains(&Event::More(Shown::Placed { reference: r })));
    // All the rest: none left; nothing more to drop.
    more::placed::drop_item(&order, &mut state, FormId(CUP), 10, 0.0).unwrap();
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CUP)), 0);
    assert!(more::placed::drop_item(&order, &mut state, FormId(CUP), 1, 0.0).is_none());
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.more.placed.refs, state.more.placed.refs);
    // Dropped items lie where they fell as things E can take; taken, they
    // go back into the inventory and the reference is gone.
    let space = state.place(&order, PLAYER_REF).unwrap().0;
    let lying = more::placed::dropped_items(&order, &state, space);
    assert_eq!(lying.len(), 2);
    let first = lying.iter().find(|i| i.reference == r).unwrap();
    assert!(first.is_item());
    assert_eq!((first.base, first.count), (FormId(CUP), 3));
    state.pick_up(&order, first.reference, first.base, first.count);
    more::placed::taken(&mut state, first.reference);
    assert_eq!(state.item_count(&order, PLAYER_REF, FormId(CUP)), 3);
    assert_eq!(more::placed::dropped_items(&order, &state, space).len(), 1);
    // Elsewhere: none.
    assert!(more::placed::dropped_items(&order, &state, FormId(0x1234)).is_empty());
}

#[test]
fn objects_break_in_stages() {
    let (_data, order) = order("more-dest");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    assert_eq!(q(&mut state, "BarrelRef.GetDestructionStage"), -1.0);
    run(&order, &scripts, &mut state, "BarrelRef.DamageObject 30");
    // 70 %: no stage passed yet.
    assert_eq!(q(&mut state, "BarrelRef.GetDestructionStage"), 0.0);
    run(&order, &scripts, &mut state, "BarrelRef.DamageObject 25");
    // 45 %: the first stage (50 %), with its model.
    assert_eq!(q(&mut state, "BarrelRef.GetDestructionStage"), 1.0);
    assert!(state.events.iter().any(|e| matches!(e,
        Event::More(Shown::Destruction { stage: 1, model: Some(m), .. })
            if m == "test\\barreldamaged.nif")));
    assert_eq!(q(&mut state, "BarrelRef.GetDestroyed"), 0.0);
    run(&order, &scripts, &mut state, "BarrelRef.DamageObject 100");
    assert_eq!(q(&mut state, "BarrelRef.GetDestructionStage"), 2.0);
    assert_eq!(q(&mut state, "BarrelRef.GetDestroyed"), 1.0);
    run(&order, &scripts, &mut state, "BarrelRef.ClearDestruction");
    assert_eq!(q(&mut state, "BarrelRef.GetDestructionStage"), -1.0);
    assert_eq!(q(&mut state, "BarrelRef.GetDestroyed"), 0.0);
    // The crate's first stage caps the damage at 60 %; the next disables it.
    run(&order, &scripts, &mut state, "CrateRef.DamageObject 90");
    assert_eq!(state.more.damaged.health[&FormId(CRATE_REF)], 60.0);
    assert_eq!(q(&mut state, "CrateRef.GetDisabled"), 0.0);
    run(&order, &scripts, &mut state, "CrateRef.DamageObject 50");
    assert_eq!(q(&mut state, "CrateRef.GetDisabled"), 1.0);
    // Nothing to destroy on a person (another path in the game).
    assert_eq!(q(&mut state, "PersonRef.DamageObject 10"), STOPPED);
}

#[test]
fn saves_time_owners_and_waking() {
    let (_data, order) = order("more-misc");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "Autosave\nForceSave\nSystemSave",
    );
    for kind in [SaveKind::Autosave, SaveKind::Force, SaveKind::System] {
        assert!(state.events.contains(&Event::More(Shown::Save(kind))));
    }
    run(&order, &scripts, &mut state, "SetGlobalTimeMultiplier 0.5");
    assert_eq!(state.more.time_multiplier, Some(0.5));
    run(&order, &scripts, &mut state, "CupRef.SetOwnership");
    run(
        &order,
        &scripts,
        &mut state,
        "BarrelRef.SetOwnership\nBarrelRef.ClearOwnership",
    );
    assert!(!state
        .set_by_scripts
        .owners
        .contains_key(&FormId(BARREL_REF)));
    // Waking: only while sleeping.
    run(&order, &scripts, &mut state, "WakeUpPC 3");
    assert_eq!(state.living.hours_left, 0);
    state.living.sleeping = true;
    state.living.hours_left = 8;
    run(&order, &scripts, &mut state, "WakeUpPC 0");
    assert_eq!(state.living.hours_left, 0);
    // What's kept comes back from a save.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetGhost 1\nPersonRef.SetActorAlpha 0.25\nSetEssential TestPerson 1",
    );
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert!(back.more.ghosts.contains(&FormId(PERSON_REF)));
    assert_eq!(back.more.alpha[&FormId(PERSON_REF)], 0.25);
    assert!(back.more.essential_bases[&FormId(PERSON)]);
    assert_eq!(back.more.time_multiplier, Some(0.5));
}

#[test]
fn fights_ranks_and_effect_seconds() {
    let (_data, order) = order("more-fights");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // Ranks: the person's 2 against the hero's 0; the dog isn't in the gang.
    assert_eq!(
        q(
            &mut state,
            "PersonRef.GetFactionRankDifference TestGang HeroRef"
        ),
        2.0
    );
    assert_eq!(
        q(
            &mut state,
            "HeroRef.GetFactionRankDifference TestGang PersonRef"
        ),
        -2.0
    );
    assert_eq!(
        q(
            &mut state,
            "PersonRef.GetFactionRankDifference TestGang DogRef"
        ),
        0.0
    );
    // Fights: who targets whom; all of it stopped for the hero.
    state.combat.insert(FormId(PERSON_REF), FormId(HERO_REF));
    state.combat.insert(FormId(DOG_REF), FormId(HERO_REF));
    state.combat.insert(FormId(HERO_REF), FormId(PERSON_REF));
    assert_eq!(q(&mut state, "PersonRef.IsCombatTarget HeroRef"), 1.0);
    assert_eq!(q(&mut state, "HeroRef.IsCombatTarget DogRef"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.StopCombatAlarmOnActor",
    );
    assert!(state.combat.is_empty());
    // For the player: the factions of the people around forgive crimes.
    state.crime_enemies.insert(FormId(GANG));
    state.combat.insert(FormId(DOG_REF), PLAYER_REF);
    run(
        &order,
        &scripts,
        &mut state,
        "player.StopCombatAlarmOnActor",
    );
    assert!(state.crime_enemies.is_empty() && state.combat.is_empty());
    // Damage resistance now.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetActorValue DamageResist 15",
    );
    assert_eq!(q(&mut state, "PersonRef.GetArmorRating"), 15.0);
    // Turning, as the viewer saw it.
    more::report(
        &mut state,
        FormId(PERSON_REF),
        Seen {
            movement: movement::TURNING_RIGHT,
            ..Seen::default()
        },
    );
    assert_eq!(q(&mut state, "PersonRef.IsTurning"), 2.0);
    // An effect script sees its update's seconds; outside one, 0.
    assert_eq!(q(&mut state, "ScriptEffectElapsedSeconds"), 0.0);
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestTick",
    );
    Runner::new(&order, &scripts, &mut state).update(0.5);
    assert_eq!(state.globals[&FormId(ELAPSED)], 0.5);
    // Menus' points, a Securitron's face, rumble.
    run(
        &order,
        &scripts,
        &mut state,
        "AddSPECIALPoints 1\nAddSPECIALPoints 1\nAddTagSkills 2\nSetRumble 0.5 0.5 1",
    );
    assert_eq!((state.more.special_points, state.more.tag_points), (2, 2));
    run(&order, &scripts, &mut state, "SetSPECIALPoints 5");
    assert_eq!(state.more.special_points, 5);
    run(
        &order,
        &scripts,
        &mut state,
        "SetSecuritronExpression PersonRef Infantry Neutral",
    );
    assert_eq!(
        state.more.securitron_faces[&FormId(PERSON_REF)],
        ("Infantry".to_string(), "Neutral".to_string())
    );
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled);
}

#[test]
fn the_pipboy_radio_and_its_stations() {
    let (_data, order) = order("more-radio");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    // The script functions act on the Pip-Boy's own radio
    // (`GameState::radio`, what DATA › Radio lists and plays).
    let tuned = |state: &GameState| (state.radio.on, state.radio.active);
    let id = |r: u32| Some(FormId(r));
    // Tuning while off does nothing; on with a station tunes to it.
    run(&order, &scripts, &mut state, "PipboyRadio Tune RadioRef");
    assert_eq!(tuned(&state), (false, None));
    run(&order, &scripts, &mut state, "PipboyRadio on RadioRef");
    assert_eq!(tuned(&state), (true, id(RADIO_REF)));
    // On as a click turns it on: the decks cleared, the music held, and
    // a range pass asked for now.
    let asked = std::mem::take(&mut state.radio.pending);
    assert!(asked.contains(&RadioEvent::ClearDecks));
    assert!(asked.contains(&RadioEvent::HoldMusic(true)));
    assert!(state.radio.force_update);
    assert!(state
        .radio
        .stations
        .iter()
        .any(|s| s.reference == FormId(RADIO_REF)));
    // Dead Money's words: `Tune` with a capital (compared without case).
    run(&order, &scripts, &mut state, "PipboyRadio Tune TalkerRef");
    assert_eq!(tuned(&state), (true, id(TALKER_REF)));
    // Something that can't be a station switches the radio off: the
    // music let go.
    state.radio.pending.clear();
    run(&order, &scripts, &mut state, "PipboyRadio tune BarrelRef");
    assert_eq!(tuned(&state), (false, None));
    assert!(state.radio.pending.contains(&RadioEvent::HoldMusic(false)));
    // A number starting with 1 is on; off forgets the station.
    run(&order, &scripts, &mut state, "PipboyRadio 1 RadioRef");
    assert_eq!(tuned(&state), (true, id(RADIO_REF)));
    run(&order, &scripts, &mut state, "PipBoyRadioOff");
    assert_eq!(tuned(&state), (false, None));
    // On without a station: the first station in range (RadioRef is heard
    // everywhere).
    run(&order, &scripts, &mut state, "PipboyRadio on");
    assert_eq!(tuned(&state), (true, id(RADIO_REF)));

    // A station's conversation: the topic given (a programme of its song,
    // starting in 50 ms), or the default one; not a station: nothing.
    state.radio.clock = 5_000;
    run(
        &order,
        &scripts,
        &mut state,
        "RadioRef.StartRadioConversation TestRadioTopic\nTalkerRef.StartRadioConversation\n\
         BarrelRef.StartRadioConversation TestRadioTopic",
    );
    let station = |state: &GameState, r: u32| {
        state
            .radio
            .stations
            .iter()
            .find(|s| s.reference == FormId(r))
            .cloned()
    };
    let s = station(&state, RADIO_REF).unwrap();
    assert_eq!(s.started, Some(id(RADIO_TOPIC)));
    assert_eq!((s.current, s.start, s.duration), (Some(0), 5_050, 0));
    assert_eq!(s.items[0].sound(), id(SONG));
    assert_eq!(station(&state, TALKER_REF).unwrap().started, Some(None));
    assert!(station(&state, BARREL_REF).is_none());

    // A person plays a station and stops; 2 and things that aren't
    // people do nothing.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetNPCRadio 1 RadioRef\nBarrelRef.SetNPCRadio 1 RadioRef\n\
         HeroRef.SetNPCRadio 1 RadioRef\nHeroRef.SetNPCRadio 2 RadioRef",
    );
    let n: Vec<(FormId, FormId)> = state.radio.receivers().collect();
    assert!(n.contains(&(FormId(PERSON_REF), FormId(RADIO_REF))));
    assert!(n.contains(&(FormId(HERO_REF), FormId(RADIO_REF))));
    assert!(!n.iter().any(|&(w, _)| w == FormId(BARREL_REF)));
    state.radio.pending.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.SetNPCRadio 0 RadioRef",
    );
    assert!(!state.radio.receivers().any(|(w, _)| w == FormId(HERO_REF)));
    assert!(state
        .radio
        .pending
        .contains(&RadioEvent::ReceiverStop(FormId(HERO_REF))));

    state.radio.force_update = false;
    run(
        &order,
        &scripts,
        &mut state,
        "ForceRadioStationUpdate\nResetPipboyManager\nPipboyRadio enable TalkerRef",
    );
    assert!(state.radio.force_update && state.more.pipboy_reset);
    assert_eq!(tuned(&state), (true, id(TALKER_REF)));

    // Kept in a save: the Pip-Boy radio, the conversation still playing,
    // who plays which station.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(tuned(&back), (true, id(TALKER_REF)));
    assert_eq!(
        back.radio.receivers().collect::<Vec<_>>(),
        state.radio.receivers().collect::<Vec<_>>()
    );
    assert_eq!(
        station(&back, RADIO_REF).map(|s| (s.started, s.pending_start)),
        Some((Some(id(RADIO_TOPIC)), true))
    );
    assert!(back.more.pipboy_reset);
    // Saves from before: the scripts' own radio line is the Pip-Boy's.
    let old = saved.replace("pipboyreset", "scriptradio on 00000E33");
    let (back, _) = world::save::load(&old).unwrap();
    assert_eq!(back.radio.active, id(RADIO_REF));
}

/// `SetNPCRadio`'s receiver plays the station's line through the person
/// while the Pip-Boy isn't tuned to it (`00834260`): its song's mono file,
/// in step with the line's start, when the player is within `AMLRadio`'s
/// largest attenuation distance × 1.1 (2200); a receiver in another
/// place than the player stops for good.
#[test]
fn a_person_plays_a_station_near_the_player() {
    struct Files;
    impl RadioFiles for Files {
        fn voice_ms(&mut self, _: &str) -> Option<u32> {
            None
        }
        fn song_ms(&mut self, _: &str) -> Option<u32> {
            Some(60_000)
        }
    }
    let (_data, order) = order("more-npcradio");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetNPCRadio 1 RadioRef\nRadioRef.StartRadioConversation TestRadioTopic",
    );
    let frame = |state: &mut GameState, now: u64| {
        let mut radio = std::mem::take(&mut state.radio);
        let place = world::radio::place_of(state);
        let ev = radio.update(&order, &scripts, state, &place, now, 75, 0.8, &mut Files);
        state.radio = radio;
        ev
    };
    // Too far (the person is at 0,0,0): nothing starts.
    state.player_position = Some([3000.0, 0.0, 0.0]);
    let ev = frame(&mut state, 1_000);
    assert!(!ev.iter().any(|e| matches!(e, RadioEvent::Receiver { .. })));
    // Near: the song, 950 ms into the line (it started at 50).
    state.player_position = Some([2000.0, 0.0, 0.0]);
    let ev = frame(&mut state, 1_000);
    assert!(ev.contains(&RadioEvent::Receiver {
        reference: FormId(PERSON_REF),
        path: "sound\\radio\\testsong_mono.ogg".into(),
        song: true,
        volume: 0.8,
        offset: 950,
    }));
    // Started once; the Pip-Boy (off) plays nothing.
    let ev = frame(&mut state, 1_100);
    assert!(!ev
        .iter()
        .any(|e| matches!(e, RadioEvent::Receiver { .. } | RadioEvent::Song { .. })));
    // The player elsewhere: the receiver stops for good.
    state.player_cell = Some(FormId(0xE99));
    let ev = frame(&mut state, 1_200);
    assert!(ev.contains(&RadioEvent::ReceiverStop(FormId(PERSON_REF))));
    assert!(state
        .radio
        .stations
        .iter()
        .all(|s| s.users.iter().all(|u| !u.playing)));
}

#[test]
fn objects_animations_playing() {
    let (_data, order) = order("more-anim");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // Before the viewer reports anything, nothing has 3D: nothing plays.
    assert_eq!(q(&mut state, "BarrelRef.IsAnimPlaying"), 0.0);
    more::report_sequences(
        &mut state,
        [
            (FormId(BARREL_REF), vec!["SpecialIdle".to_string()]),
            (FormId(RADIO_REF), vec!["Forward".to_string()]),
            (FormId(CRATE_REF), Vec::new()),
        ]
        .into_iter()
        .collect(),
    );
    assert_eq!(q(&mut state, "BarrelRef.IsAnimPlaying"), 1.0);
    assert_eq!(q(&mut state, "BarrelRef.IsAnimPlaying Forward"), 0.0);
    // Group names compare without case (Dead Money writes `Forward`).
    assert_eq!(q(&mut state, "RadioRef.IsAnimPlaying forward"), 1.0);
    assert_eq!(q(&mut state, "RadioRef.IsAnimPlaying Backward"), 0.0);
    assert_eq!(q(&mut state, "CrateRef.IsAnimPlaying"), 0.0);
    // People's animation data isn't carried out: asked about a group the
    // script stops; asked about any, a person on their feet is playing
    // their idle (one who is down isn't: `essential_people_go_down…`).
    assert_eq!(q(&mut state, "PersonRef.IsAnimPlaying Forward"), STOPPED);
    // Asked about any: untraced, only with `world::guesses` on
    // (`contrib_guesses.rs`); off, the script stops.
    assert_eq!(q(&mut state, "PersonRef.IsAnimPlaying"), STOPPED);
    // The viewer stops reporting a one-shot group once it has ended
    // (`preview::cell::sequence_playing`): it reads 0 from then on.
    more::report_sequences(
        &mut state,
        [(FormId(RADIO_REF), Vec::new())].into_iter().collect(),
    );
    assert_eq!(q(&mut state, "RadioRef.IsAnimPlaying Forward"), 0.0);
    assert_eq!(q(&mut state, "RadioRef.IsAnimPlaying"), 0.0);
}

/// A camera and collision for `GetLineOfSight`: boxes for the people and
/// the barrel, everything in view or nothing, and every ray stopped at
/// the same distance (or none).
struct TestSight {
    in_view: bool,
    hit: Option<f32>,
}

impl world::sight::Sight for TestSight {
    fn bound(&self, r: FormId) -> Option<([f32; 3], [f32; 3])> {
        let at = match r.0 {
            PERSON_REF => [0.0, 0.0, 0.0],
            HERO_REF => [0.0, 200.0, 0.0],
            BARREL_REF => [100.0, 0.0, 0.0],
            _ => return None,
        };
        Some((
            [at[0] - 20.0, at[1] - 20.0, at[2]],
            [at[0] + 20.0, at[1] + 20.0, at[2] + 120.0],
        ))
    }
    fn camera(&self) -> Option<[f32; 3]> {
        Some([0.0, 100.0, 120.0])
    }
    fn in_view(&self, _lo: [f32; 3], _hi: [f32; 3]) -> bool {
        self.in_view
    }
    fn ray(&self, _from: [f32; 3], _to: [f32; 3]) -> Option<f32> {
        self.hit
    }
}

fn ask_seeing(
    order: &LoadOrder,
    scripts: &ScriptCache,
    state: &mut GameState,
    sight: &TestSight,
    expr: &str,
) -> f32 {
    state.globals.insert(FormId(VALUE), STOPPED);
    Runner::new(order, scripts, state)
        .with_sight(sight)
        .run_source(&format!("set TestValue to {expr}"), None, None);
    state.globals[&FormId(VALUE)]
}

#[test]
fn line_of_sight() {
    let (_data, order) = order("more-sight");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let clear = TestSight {
        in_view: true,
        hit: None,
    };
    let walled = TestSight {
        in_view: true,
        hit: Some(10.0),
    };
    let q =
        |state: &mut GameState, s: &TestSight, e: &str| ask_seeing(&order, &scripts, state, s, e);
    // The player: in view and a ray gets through.
    assert_eq!(q(&mut state, &clear, "Player.GetLineOfSight HeroRef"), 1.0);
    // Out of view: no, however clear.
    let away = TestSight {
        in_view: false,
        hit: None,
    };
    assert_eq!(q(&mut state, &away, "Player.GetLOS HeroRef"), 0.0);
    // A ray stopped where it reaches the hero's box hit the hero (its
    // 0.75 ray from (0, 100, 120) to (0, 200, 90) enters the box 83.5
    // units along); stopped well short, a wall.
    let at_box = TestSight {
        in_view: true,
        hit: Some(84.0),
    };
    assert_eq!(q(&mut state, &at_box, "Player.GetLOS HeroRef"), 1.0);
    let short = TestSight {
        in_view: true,
        hit: Some(50.0),
    };
    assert_eq!(q(&mut state, &short, "Player.GetLOS HeroRef"), 0.0);
    // Walled off, the player's own detection data decides.
    assert_eq!(q(&mut state, &walled, "Player.GetLOS HeroRef"), 0.0);
    more::report_detection_sight(&mut state, PLAYER_REF, FormId(HERO_REF), true);
    assert_eq!(q(&mut state, &walled, "Player.GetLOS HeroRef"), 1.0);
    // No 3D: not seen.
    assert_eq!(q(&mut state, &clear, "Player.GetLOS DogRef"), 0.0);

    // Someone else: their last detection run's line of sight.
    assert_eq!(q(&mut state, &clear, "HeroRef.GetLOS PersonRef"), 0.0);
    more::report_detection_sight(&mut state, FormId(HERO_REF), FormId(PERSON_REF), true);
    assert_eq!(q(&mut state, &clear, "HeroRef.GetLOS PersonRef"), 1.0);
    // The caller must be an actor.
    assert_eq!(q(&mut state, &clear, "BarrelRef.GetLOS Player"), 0.0);
    // An object target for someone else: no detection entry, so 0.
    assert_eq!(q(&mut state, &clear, "HeroRef.GetLOS BarrelRef"), 0.0);
    // Within 2 units of the caller the answer is 1 whatever detection
    // found (`0088b880` returns early), further it's detection's again.
    let hero_at = state.place(&order, FormId(HERO_REF)).unwrap().2;
    let beside = |d: f32| (hero_at.map(|c| c + d / 3f32.sqrt()), 0.0);
    assert_eq!(q(&mut state, &clear, "PersonRef.GetLOS HeroRef"), 0.0);
    state.positions.insert(FormId(PERSON_REF), beside(1.0));
    assert_eq!(q(&mut state, &clear, "PersonRef.GetLOS HeroRef"), 1.0);
    state.positions.insert(FormId(PERSON_REF), beside(3.0));
    assert_eq!(q(&mut state, &clear, "PersonRef.GetLOS HeroRef"), 0.0);
    state.positions.remove(&FormId(PERSON_REF));
    // The player headless (no camera) isn't carried out.
    assert_eq!(
        ask(&order, &scripts, &mut state, "Player.GetLOS HeroRef"),
        STOPPED
    );
    // Headless, someone else's test still answers from detection.
    assert_eq!(
        ask(&order, &scripts, &mut state, "HeroRef.GetLOS PersonRef"),
        1.0
    );
}

#[test]
fn effect_shaders_on_references() {
    use world::more_functions::shaders;
    let (_data, order) = order("more-shaders");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    more::report_loaded(
        &mut state,
        [PLAYER_REF, FormId(PERSON_REF), FormId(BARREL_REF)]
            .into_iter()
            .collect(),
    );
    let running = |state: &GameState| -> Vec<(u32, Option<f64>)> {
        shaders::active(state)
            .map(|v| {
                assert_eq!(v.shader, FormId(SHADER));
                (v.reference.0, v.until)
            })
            .collect()
    };
    // Until stopped; again, a second one (they stack); for 2 s; with no
    // 3D, nothing; no reference, the player.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.PlayMagicShaderVisuals TestShader\nPersonRef.pms TestShader\n\
         BarrelRef.pms TestShader 2\nHeroRef.pms TestShader\npms TestShader",
    );
    let now = state.seconds;
    assert_eq!(
        running(&state),
        vec![
            (PERSON_REF, None),
            (PERSON_REF, None),
            (BARREL_REF, Some(now + 2.0)),
            (PLAYER_REF.0, None),
        ]
    );
    assert!(state.events.contains(&Event::More(Shown::ShaderVisual {
        reference: FormId(BARREL_REF),
        shader: FormId(SHADER),
        seconds: Some(2.0),
    })));
    // Its seconds over, the barrel's ends.
    state.seconds += 3.0;
    assert_eq!(running(&state).len(), 3);
    // Stopping ends every one with that shader on the reference.
    state.events.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.StopMagicShaderVisuals TestShader\nHeroRef.sms TestShader",
    );
    assert_eq!(running(&state), vec![(PLAYER_REF.0, None)]);
    assert_eq!(
        state.events,
        vec![Event::More(Shown::ShaderVisualStopped {
            reference: FormId(PERSON_REF),
            shader: FormId(SHADER),
        })]
    );
}

#[test]
fn terminals_go_back_only_while_open() {
    let (_data, order) = order("more-terminal-back");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let back = Event::TerminalBack;
    // No terminal open: nothing.
    run(&order, &scripts, &mut state, "ForceTerminalBack");
    assert!(!state.events.contains(&back));
    // Another menu open: nothing.
    state.more.menu_open = Some(1036);
    run(&order, &scripts, &mut state, "ForceTerminalBack");
    assert!(!state.events.contains(&back));
    // The terminal menu: back a screen, once per call.
    state.more.menu_open = Some(world::terminal::TERMINAL_MENU);
    run(
        &order,
        &scripts,
        &mut state,
        "ForceTerminalBack\nForceTerminalBack",
    );
    assert_eq!(state.events.iter().filter(|e| **e == back).count(), 2);
}

#[test]
fn caravan_cards_picked_up() {
    let (_data, order) = order("more-cards");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let held = |state: &GameState, holder: FormId, item: u32| {
        state
            .items
            .get(&(holder, FormId(item)))
            .copied()
            .unwrap_or(0)
    };
    // Outside an item's script there's no container.
    assert_eq!(
        ask(&order, &scripts, &mut state, "CardRef.GetContainer"),
        0.0
    );
    // The player picks up a card: its `OnAdd` (run with the scripted
    // items, `Runner::run_item_scripts`) sees the player as the
    // container, the card joins their cards and leaves the inventory.
    state.pick_up(&order, FormId(CARD_REF), FormId(CARD), 1);
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert_eq!(state.globals[&FormId(VALUE)], PLAYER_REF.0 as f32);
    assert!(state.caravan.owns(FormId(CARD)));
    assert_eq!(held(&state, PLAYER_REF, CARD), 0);
    // Not a card: it isn't added to the cards, but `RemoveMe` still takes
    // it out.
    state.pick_up(&order, FormId(CARD_CUP_REF), FormId(CARD_CUP), 1);
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert!(!state.caravan.owns(FormId(CARD_CUP)));
    assert_eq!(held(&state, PLAYER_REF, CARD_CUP), 0);
    // Into another container: the script returns before anything.
    state.stock(&order, FormId(CRATE_REF));
    state.items.insert((FormId(CRATE_REF), FormId(CARD_CUP)), 1);
    state.added(&order, FormId(CRATE_REF), FormId(CARD_CUP), 1);
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert_eq!(state.globals[&FormId(VALUE)], CRATE_REF as f32);
    assert_eq!(held(&state, FormId(CRATE_REF), CARD_CUP), 1);
    // `RemoveMe` with a container (`005b53d0`): one moves there, its
    // script with it.
    state.items.insert((PLAYER_REF, FormId(MOVER)), 2);
    state.added(&order, PLAYER_REF, FormId(MOVER), 1);
    Runner::new(&order, &scripts, &mut state).run_item_scripts();
    assert_eq!(held(&state, PLAYER_REF, MOVER), 1);
    assert_eq!(held(&state, FormId(CRATE_REF), MOVER), 1);
    assert!(state
        .item_scripts
        .iter()
        .any(|s| s.holder == FormId(CRATE_REF) && s.item == FormId(MOVER)));

    // The cards are kept in a save.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.caravan.inactive, state.caravan.inactive);
}

#[test]
fn companions_pushes_dispositions_and_causes_of_death() {
    let (_data, order) = order("more-actors");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    let opened = |state: &GameState, who: u32| {
        state
            .events
            .contains(&Event::TeammateContainer(FormId(who)))
    };

    // A companion's things open; someone else's only when forced, and
    // never a barrel's.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.OpenTeammateContainer",
    );
    assert!(!opened(&state, PERSON_REF));
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetPlayerTeammate 1",
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.OpenTeammateContainer",
    );
    assert!(opened(&state, PERSON_REF));
    run(
        &order,
        &scripts,
        &mut state,
        "DogRef.OpenTeammateContainer 1",
    );
    assert!(opened(&state, DOG_REF));
    run(
        &order,
        &scripts,
        &mut state,
        "BarrelRef.OpenTeammateContainer 1",
    );
    assert!(!opened(&state, BARREL_REF));

    // A push: only someone with 3D loaded, at the force from their
    // Agility and the number.
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.PushActorAway PersonRef 5",
    );
    let pushed = |state: &GameState| {
        state
            .events
            .iter()
            .filter(|e| matches!(e, Event::More(Shown::PushedAway { .. })))
            .count()
    };
    assert_eq!(pushed(&state), 0);
    more::report_loaded(&mut state, [FormId(PERSON_REF)].into_iter().collect());
    let agility = q(&mut state, "PersonRef.GetActorValue Agility");
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.PushActorAway PersonRef 5",
    );
    assert!(state.events.contains(&Event::More(Shown::PushedAway {
        who: FormId(PERSON_REF),
        from: FormId(HERO_REF),
        force: more::actors::push_force(&order, f64::from(agility), 5),
    })));
    // Agility 5, 5: (1 − 0.008 × 50) × (5 × 10 + 50).
    assert!((more::actors::push_force(&order, 5.0, 5) - 60.0).abs() < 1e-4);
    // Not an actor: nothing pushed (the game only reports it).
    run(
        &order,
        &scripts,
        &mut state,
        "HeroRef.PushActorAway BarrelRef 5",
    );
    assert_eq!(pushed(&state), 1);

    // A disposition toward the player moves to the number, within 0–100;
    // toward anyone else nothing is kept.
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetDisposition player 100",
    );
    assert_eq!(
        more::actors::disposition(&state, FormId(PERSON_REF), PLAYER_REF),
        100
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetDisposition player 40",
    );
    assert_eq!(
        more::actors::disposition(&state, FormId(PERSON_REF), PLAYER_REF),
        40
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetDisposition player 250",
    );
    assert_eq!(
        more::actors::disposition(&state, FormId(PERSON_REF), PLAYER_REF),
        100
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetDisposition player -5",
    );
    assert_eq!(
        more::actors::disposition(&state, FormId(PERSON_REF), PLAYER_REF),
        0
    );
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.SetDisposition DogRef 80",
    );
    assert_eq!(state.more.dispositions.0.len(), 1);

    // Causes of death: none kept is −1; fists kill hand to hand; `Kill`
    // with a limb keeps the cause given.
    assert_eq!(q(&mut state, "DogRef.GetCauseofDeath"), -1.0);
    for _ in 0..1000 {
        if state.dead.contains(&FormId(DOG_REF)) {
            break;
        }
        Runner::new(&order, &scripts, &mut state).hit(PLAYER_REF, FormId(DOG_REF), None);
    }
    assert!(state.dead.contains(&FormId(DOG_REF)));
    assert_eq!(q(&mut state, "DogRef.GetCauseofDeath"), 3.0);
    run(&order, &scripts, &mut state, "PersonRef.Kill player");
    assert_eq!(q(&mut state, "PersonRef.GetCauseofDeath"), -1.0);
    run(&order, &scripts, &mut state, "HeroRef.Kill player 0 0");
    assert_eq!(q(&mut state, "HeroRef.GetCauseofDeath"), 0.0);
    assert_eq!(q(&mut state, "BarrelRef.GetCauseofDeath"), -1.0);

    // Both are kept in a save.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.more.dispositions, state.more.dispositions);
    assert_eq!(back.more.cause_of_death, state.more.cause_of_death);
}

#[test]
fn a_recast_replaces_its_effect_and_a_poison_adds_its_duration() {
    // 00823210 (CheckAddEffect): an actor effect's identical effect from
    // the same caster is dispelled (00824400); another caster's stays; a
    // poison's identical effect gets the new duration added (00824c00).
    let (_data, order) = order("more-recast");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let from = |state: &GameState, source: u32| -> Vec<(Option<FormId>, f32)> {
        state
            .active_effects
            .iter()
            .filter(|e| e.target == PLAYER_REF && e.source == FormId(source))
            .map(|e| (e.caster, e.remaining))
            .collect()
    };
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestTick\nplayer.CastImmediateOnSelf TestTick",
    );
    assert_eq!(from(&state, TICK), vec![(Some(PLAYER_REF), 10.0)]);
    run(
        &order,
        &scripts,
        &mut state,
        "PersonRef.Cast TestTick player",
    );
    assert_eq!(
        from(&state, TICK),
        vec![(Some(PLAYER_REF), 10.0), (Some(FormId(PERSON_REF)), 10.0)]
    );
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestTickPoison\nplayer.CastImmediateOnSelf TestTickPoison",
    );
    assert_eq!(from(&state, TICK_POISON), vec![(Some(PLAYER_REF), 20.0)]);
    // The caster and the effect item are kept in a save.
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.active_effects, state.active_effects);
}

#[test]
fn dispel_all_spells_leaves_abilities_and_poisons() {
    let (_data, order) = order("more-dispel");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let on = |state: &GameState, who: u32, source: u32| {
        state
            .active_effects
            .iter()
            .any(|e| e.target == FormId(who) && e.source == FormId(source))
    };
    run(
        &order,
        &scripts,
        &mut state,
        "player.CastImmediateOnSelf TestTick\nplayer.AddSpell TestTickAbility\n\
         player.CastImmediateOnSelf TestTickPoison\nPersonRef.CastImmediateOnSelf TestTick",
    );
    assert!(on(&state, PLAYER_REF.0, TICK));
    assert!(on(&state, PLAYER_REF.0, TICK_ABILITY));
    assert!(on(&state, PLAYER_REF.0, TICK_POISON));
    // Not an actor: nothing, and the script goes on.
    assert_eq!(
        ask(&order, &scripts, &mut state, "BarrelRef.DispelAllSpells"),
        1.0
    );
    run(&order, &scripts, &mut state, "player.DispelAllSpells");
    assert!(!on(&state, PLAYER_REF.0, TICK));
    assert!(on(&state, PLAYER_REF.0, TICK_ABILITY));
    assert!(on(&state, PLAYER_REF.0, TICK_POISON));
    // Only the caller's effects end.
    assert!(on(&state, PERSON_REF, TICK));
    assert!(more::actors::dispelled_by_all(&order, FormId(TICK)));
    assert!(!more::actors::dispelled_by_all(
        &order,
        FormId(TICK_ABILITY)
    ));
}

#[test]
fn traps_vats_targets_and_weapons_fired() {
    let (_data, order) = order("more-traps");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let targetable =
        |state: &GameState, r: u32| more::traps::vats_targetable(&order, state, FormId(r));

    // The barrel is destructible, its base not targetable: SetVATSTarget
    // turns it each way; the same as the base clears the override.
    assert!(!targetable(&state, BARREL_REF));
    run(&order, &scripts, &mut state, "BarrelRef.SetVATSTarget 1");
    assert!(targetable(&state, BARREL_REF));
    assert!(state.more.vats_overrides.0.contains(&FormId(BARREL_REF)));
    run(&order, &scripts, &mut state, "BarrelRef.SetVATSTarget 0");
    assert!(!targetable(&state, BARREL_REF));
    assert!(state.more.vats_overrides.0.is_empty());
    // Not destructible: nothing.
    assert_eq!(
        ask(&order, &scripts, &mut state, "RadioRef.SetVATSTarget 1"),
        1.0
    );
    assert!(!targetable(&state, RADIO_REF));

    // FireWeapon: a weapon is fired; anything else only reported.
    run(&order, &scripts, &mut state, "BarrelRef.FireWeapon TestGun");
    assert!(state.events.contains(&Event::More(Shown::WeaponFired {
        from: FormId(BARREL_REF),
        weapon: FormId(GUN),
    })));
    run(
        &order,
        &scripts,
        &mut state,
        "BarrelRef.FireWeapon TestTick",
    );
    let fired = state
        .events
        .iter()
        .filter(|e| matches!(e, Event::More(Shown::WeaponFired { .. })))
        .count();
    assert_eq!(fired, 1);

    // The override is kept in a save.
    run(&order, &scripts, &mut state, "BarrelRef.SetVATSTarget 1");
    let saved = world::save::save(&state, None);
    let (back, _) = world::save::load(&saved).unwrap();
    assert_eq!(back.more.vats_overrides, state.more.vats_overrides);
}

#[test]
fn shots_leave_along_the_objects_facing() {
    let close = |a: [f32; 3], b: [f32; 3]| (0..3).all(|i| (a[i] - b[i]).abs() < 1e-4);
    let q = std::f32::consts::FRAC_PI_2;
    // Facing north, east (a quarter turn clockwise), and tipped nose down.
    let (o, d) = more::traps::shot_from([1.0, 2.0, 3.0], [0.0, 0.0, 0.0], 1.0, None);
    assert!(close(o, [1.0, 2.0, 3.0]) && close(d, [0.0, 1.0, 0.0]));
    let (_, d) = more::traps::shot_from([0.0; 3], [0.0, 0.0, q], 1.0, None);
    assert!(close(d, [1.0, 0.0, 0.0]), "{d:?}");
    let (_, d) = more::traps::shot_from([0.0; 3], [q, 0.0, 0.0], 1.0, None);
    assert!(close(d, [0.0, 0.0, -1.0]), "{d:?}");
    // From a node 10 units ahead in the model, the object turned east.
    let node = nif::math::Transform {
        translation: [0.0, 10.0, 0.0],
        ..nif::math::Transform::IDENTITY
    };
    let (o, d) = more::traps::shot_from([0.0; 3], [0.0, 0.0, q], 1.0, Some(node));
    assert!(
        close(o, [10.0, 0.0, 0.0]) && close(d, [1.0, 0.0, 0.0]),
        "{o:?} {d:?}"
    );
}

#[test]
fn facing_up_as_the_viewer_reports_it() {
    let (_data, order) = order("more-facing");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // No 3D reported: 1; an object: 0.
    assert_eq!(q(&mut state, "PersonRef.IsFacingUp"), 1.0);
    assert_eq!(q(&mut state, "BarrelRef.IsFacingUp"), 0.0);
    more::report_facing_up(
        &mut state,
        [(FormId(PERSON_REF), false), (FormId(DOG_REF), true)]
            .into_iter()
            .collect(),
    );
    assert_eq!(q(&mut state, "PersonRef.IsFacingUp"), 0.0);
    assert_eq!(q(&mut state, "DogRef.IsFacingUp"), 1.0);
}

#[test]
fn recipe_and_casino_menus_open_with_their_data() {
    use world::casino::Game;
    let recipe = world::crafting::RECIPE_MENU;
    let (_data, order) = order("more-menus");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let has = |state: &GameState, e: Event| state.events.contains(&e);

    // The recipe menu, sold by the player, then by a talking activator's
    // speaker; an ordinary object sells nothing.
    run(&order, &scripts, &mut state, "player.ShowRecipeMenu");
    assert!(has(
        &state,
        Event::RecipeMenu {
            actor: PLAYER_REF,
            category: FormId(0),
        }
    ));
    assert!(has(&state, Event::Menu(recipe)));
    state.events.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "TalkerRef.SetTalkingActivatorActor PersonRef\nTalkerRef.ShowRecipeMenu",
    );
    assert!(has(
        &state,
        Event::RecipeMenu {
            actor: FormId(PERSON_REF),
            category: FormId(0),
        }
    ));
    state.events.clear();
    assert_eq!(
        ask(&order, &scripts, &mut state, "BarrelRef.ShowRecipeMenu"),
        1.0
    );
    assert!(!has(&state, Event::Menu(recipe)));

    // A casino game with its casino and numbers; without a casino, nothing.
    run(
        &order,
        &scripts,
        &mut state,
        "ShowSlotMachineMenuParams TestCasino 1 25 0",
    );
    assert!(has(
        &state,
        Event::Casino {
            game: Game::Slots,
            casino: FormId(CASINO),
            min_bet: 1,
            max_bet: 25,
            min_winnings: 0,
        }
    ));
    assert!(has(&state, Event::Menu(1080)));
    run(
        &order,
        &scripts,
        &mut state,
        "ShowBlackJackMenuParams TestCasino 1 200 0",
    );
    assert!(has(&state, Event::Menu(1081)));
    run(
        &order,
        &scripts,
        &mut state,
        "ShowRouletteMenuParams TestCasino 1 100 0",
    );
    assert!(has(&state, Event::Menu(1082)));
    state.events.clear();
    run(
        &order,
        &scripts,
        &mut state,
        "ShowRouletteMenuParams TestGun 1 100 0",
    );
    assert!(state.events.is_empty());
}

#[test]
fn say_to_done_runs_the_blocks_for_the_topic_said() {
    let (_data, order) = order("more-saytodone");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    state.globals.insert(FormId(VALUE), 0.0);
    // The line of `TestRadioTopic` is said: only its block runs.
    Runner::new(&order, &scripts, &mut state).say_to_done(FormId(SAYER_REF), FormId(RADIO_TOPIC));
    assert_eq!(state.globals[&FormId(VALUE)], 1.0);
    Runner::new(&order, &scripts, &mut state).say_to_done(FormId(SAYER_REF), FormId(RADIO_TOPIC));
    assert_eq!(state.globals[&FormId(VALUE)], 2.0);
    // Another topic: its own block.
    Runner::new(&order, &scripts, &mut state).say_to_done(FormId(SAYER_REF), FormId(TICK));
    assert_eq!(state.globals[&FormId(VALUE)], 100.0);
    // Someone without such blocks: nothing.
    Runner::new(&order, &scripts, &mut state).say_to_done(FormId(PERSON_REF), FormId(RADIO_TOPIC));
    assert_eq!(state.globals[&FormId(VALUE)], 100.0);
}

#[test]
fn a_teammate_told_to_wait_stays_and_one_with_nothing_to_do_follows() {
    use world::ai::{kinds, teammate_follows, Package};
    let package = |kind: u8| Package {
        form_id: FormId(0x0100_0001),
        editor_id: None,
        kind,
        flags: 0,
        location: None,
        schedule: Default::default(),
        conditions: Vec::new(),
        target: None,
        topic: None,
        actions: Default::default(),
        data: Default::default(),
    };
    // Nothing, or a package that only fills time: they come along.
    assert!(teammate_follows(None));
    for kind in [kinds::SANDBOX, kinds::WANDER, kinds::PATROL, kinds::FIND] {
        assert!(teammate_follows(Some(&package(kind))), "kind {kind}");
    }
    // The wait order is a guard package: they stay. So do a travel, a
    // follow and the rest, which are the game's own.
    for kind in [kinds::GUARD, kinds::TRAVEL, kinds::FOLLOW, kinds::DIALOGUE] {
        assert!(!teammate_follows(Some(&package(kind))), "kind {kind}");
    }
}

/// `GetShouldAttack` (`0059ed30`: 100 or 0, both people), `GetIsAlignment`
/// (`005a4dd0`, `0047e040`'s bands), `SetItemValue` (`005d3e30`: the
/// base's value, for every one; saved), `GetContainer` (`005ce5c0`: an
/// item script's holder).
#[test]
fn attacks_alignment_values_and_holders() {
    let (_data, order) = order("more-attack-align-value");
    let scripts = ScriptCache::default();
    let mut state = new_game(&order);
    let q = |state: &mut GameState, e: &str| ask(&order, &scripts, state, e);
    // Frenzied (aggression 3) attacks anyone; unaggressive, no one.
    state.actor_values.insert((FormId(PERSON_REF), 0), 3.0);
    assert_eq!(q(&mut state, "PersonRef.GetShouldAttack HeroRef"), 100.0);
    state.actor_values.insert((FormId(PERSON_REF), 0), 0.0);
    assert_eq!(q(&mut state, "PersonRef.GetShouldAttack HeroRef"), 0.0);
    // Not a person: 0.
    state.actor_values.insert((FormId(PERSON_REF), 0), 3.0);
    assert_eq!(q(&mut state, "PersonRef.GetShouldAttack ChildRef"), 0.0);

    // Karma 0 is neutral (1); -300 evil (2); 800 very good (3).
    let karma = |state: &mut GameState, k: f64| {
        state.actor_values.insert((PLAYER_REF, 23), k);
    };
    karma(&mut state, 0.0);
    assert_eq!(q(&mut state, "player.GetIsAlignment 1"), 1.0);
    assert_eq!(q(&mut state, "player.GetIsAlignment 0"), 0.0);
    karma(&mut state, -300.0);
    assert_eq!(q(&mut state, "player.GetIsAlignment 2"), 1.0);
    karma(&mut state, 800.0);
    assert_eq!(q(&mut state, "player.GetIsAlignment 3"), 1.0);

    // The cup is worth 1; set on the placed one, every cup is worth 7.
    let cup = FormId(CUP);
    assert_eq!(world::barter::value_now(&order, &state, cup), 1.0);
    run(&order, &scripts, &mut state, "ChildRef.SetItemValue 7");
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
    assert_eq!(world::barter::value_now(&order, &state, cup), 7.0);
    assert_eq!(world::items::value(&order, &state, cup), 7);
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert_eq!(back.more.item_values.get(&cup), Some(&7));

    // Outside an item's own script there's no holder.
    assert_eq!(q(&mut state, "GetContainer"), 0.0);
}
