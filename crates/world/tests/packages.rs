//! Package types the Goodsprings gunfight (VMS16) uses and the package
//! actions, on generated records (`testdata::packages`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::packages::ids::*;
use world::ai::data::TypeData;
use world::ai::flee::{self, FleeStep};
use world::ai::guard::{self, Post};
use world::ai::{actions, kinds, procedures, Location, Package};
use world::scripting::{Event, GameState, PackageActionKind, Runner, ScriptCache};

fn order() -> (testdata::TempData, LoadOrder) {
    let data = testdata::packages::world("packages");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn load(order: &LoadOrder, id: u32) -> Package {
    Package::load(order, FormId(id)).expect("package")
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

#[test]
fn flee_types_run_the_non_combat_flee_procedure() {
    let (_data, order) = order();
    let p = load(&order, FLEE_NOWHERE);
    assert_eq!(p.kind, kinds::FLEE);
    assert_eq!(p.procedures(), Some(&[procedures::FLEE_NON_COMBAT][..]));
}

#[test]
fn a_flee_package_with_no_target_and_no_place_ends_at_once() {
    let (_data, order) = order();
    let state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    for id in [FLEE_NOWHERE, FLEE_IN_CELL] {
        let p = load(&order, id);
        assert_eq!(flee::flee_from(&order, &state, me, &p), None);
        assert_eq!(flee::flee_to(&order, me, &p), None);
        assert_eq!(
            flee::step(None, None, None, 0.0, p.flags, false),
            FleeStep::Done
        );
    }
    // "In a cell" keeps no radius (the loader's 0067f060).
    let cell = load(&order, FLEE_IN_CELL);
    assert_eq!(
        cell.location,
        Some(Location {
            kind: 1,
            form: FormId(CELL),
            radius: 0
        })
    );
}

#[test]
fn a_flee_package_runs_from_its_target_to_its_place() {
    let (_data, order) = order();
    let mut state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    let p = load(&order, FLEE_TO_MARKER);
    let from = flee::flee_from(&order, &state, me, &p);
    let to = flee::flee_to(&order, me, &p);
    assert_eq!(from, Some(FormId(THREAT_REF)));
    assert_eq!(to, Some(FormId(FLEE_MARKER_REF)));
    let at = |state: &GameState, r: u32| state.place(&order, FormId(r)).unwrap().2;
    let here = at(&state, GUARD_REF);
    let step = flee::step(
        Some((FormId(THREAT_REF), distance(here, at(&state, THREAT_REF)))),
        Some((
            FormId(FLEE_MARKER_REF),
            distance(here, at(&state, FLEE_MARKER_REF)),
        )),
        p.target.map(|t| t.2),
        world::ai::location_radius_of(&order, &p.location.unwrap()) as f32,
        p.flags,
        false,
    );
    assert_eq!(
        step,
        FleeStep::Run {
            from: Some(FormId(THREAT_REF)),
            to: Some(FormId(FLEE_MARKER_REF)),
            distance: 1000.0,
            radius: 256.0
        }
    );
    // A dead target is no longer fled from.
    state.dead.insert(FormId(THREAT_REF));
    assert_eq!(flee::flee_from(&order, &state, me, &p), None);
    // Near the linked reference.
    let linked = load(&order, FLEE_TO_LINKED);
    assert_eq!(flee::flee_to(&order, me, &linked), Some(FormId(LINKED_REF)));
}

#[test]
fn a_guard_near_its_editor_location_goes_back_there_after_a_move() {
    let (_data, order) = order();
    let mut state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    let p = load(&order, GUARD_EDITOR);
    assert_eq!(p.procedures(), Some(&[procedures::GUARD][..]));
    // A script moved them (`MoveTo`): the editor location stays.
    state.positions.insert(me, ([700.0, 700.0, 0.0], 0.0));
    let plan = guard::plan(&order, me, &p).unwrap();
    assert_eq!(
        plan.post,
        Post::Editor {
            position: [100.0, 100.0, 0.0]
        }
    );
    assert_eq!(plan.guarded, FormId(POST_REF));
    assert_eq!(plan.radius, 0.0);
    assert_eq!(plan.path_radius, 15.0);
    assert!(plan.has_location);
    // No reference to turn to, no radius to wander in.
    assert_eq!(guard::facing(&order, &state, &p), None);
    assert_eq!(guard::wanders(&order, &p), None);
}

#[test]
fn a_guard_at_a_heading_marker_turns_to_its_heading() {
    let (_data, order) = order();
    let state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    let p = load(&order, GUARD_POST);
    let plan = guard::plan(&order, me, &p).unwrap();
    assert_eq!(plan.post, Post::Reference(FormId(POST_REF)));
    assert_eq!(plan.path_radius, 15.0);
    let h = guard::facing(&order, &state, &p).unwrap();
    assert!((h - std::f32::consts::FRAC_PI_2).abs() < 1e-4, "{h}");
}

#[test]
fn a_guard_without_a_location_keeps_to_its_target() {
    let (_data, order) = order();
    let me = FormId(GUARD_REF);
    let p = load(&order, GUARD_AROUND);
    let plan = guard::plan(&order, me, &p).unwrap();
    assert_eq!(plan.post, Post::Reference(FormId(POST_REF)));
    assert_eq!(plan.radius, 200.0);
    assert_eq!(plan.path_radius, 100.0);
    assert!(!plan.has_location);
    assert_eq!(guard::wanders(&order, &p), Some(200.0));
    // Target the linked reference.
    let linked = load(&order, GUARD_LINKED);
    let plan = guard::plan(&order, me, &linked).unwrap();
    assert_eq!(plan.guarded, FormId(LINKED_REF));
    assert_eq!(plan.post, Post::Reference(FormId(LINKED_REF)));
    // A target gives its value (0 here); 30 only without one.
    assert_eq!(plan.radius, 0.0);
    assert_eq!(plan.path_radius, 15.0);
    let mut bare = linked.clone();
    bare.target = None;
    let plan = guard::plan(&order, me, &bare).unwrap();
    assert_eq!(plan.guarded, me);
    assert_eq!(plan.post, Post::Reference(me));
    assert_eq!((plan.radius, plan.path_radius), (30.0, 15.0));
}

#[test]
fn type_data_is_read_as_the_loader_files_it() {
    let (_data, order) = order();
    let TypeData::UseWeapon(w) = load(&order, USE_WEAPON).data else {
        panic!("use weapon data");
    };
    assert!(w.always_hit && !w.do_no_damage && w.crouch && !w.hold_fire);
    assert!(w.volley_fire && !w.repeat_fire);
    assert_eq!(
        (w.burst_count, w.volley_shots_min, w.volley_shots_max),
        (3, 2, 4)
    );
    assert_eq!((w.volley_wait_min, w.volley_wait_max), (0.5, 1.5));
    assert_eq!(w.weapon, Some(FormId(WEAPON)));
    assert_eq!(w.attack_target, Some((0, FormId(THREAT_REF), 5)));
    assert_eq!(
        w.target_location,
        Some(Location {
            kind: 0,
            form: FormId(FLEE_MARKER_REF),
            radius: 64
        })
    );
    assert_eq!(
        load(&order, AMBUSH).data,
        TypeData::Ambush {
            location: Some(Location {
                kind: 3,
                form: FormId(0),
                radius: 512
            })
        }
    );
    assert_eq!(
        load(&order, AMBUSH).procedures(),
        Some(&[0, procedures::AMBUSH_WAIT][..])
    );
    let TypeData::Patrol(patrol) = load(&order, PATROL).data else {
        panic!("patrol data");
    };
    assert!(patrol.repeatable && !patrol.start_at_linked_ref);
    assert_eq!(load(&order, FLEE_NOWHERE).data, TypeData::None);
}

#[test]
fn begin_actions_are_asked_once_per_start() {
    let (_data, order) = order();
    let mut state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    let begin = |p: u32| Event::PackageAction {
        who: me,
        package: FormId(p),
        kind: PackageActionKind::Begin,
    };
    actions::begin(&mut state, me, FormId(GUARD_POST), false);
    assert_eq!(state.events, [begin(GUARD_POST)]);
    // Taken up again unchanged: nothing more; restarted: again.
    actions::begin(&mut state, me, FormId(GUARD_POST), false);
    assert_eq!(state.events.len(), 1);
    actions::begin(&mut state, me, FormId(GUARD_POST), true);
    assert_eq!(state.events.len(), 2);
    state.events.clear();
    // A script package `AddScriptPackage` began isn't begun again by the
    // AI taking it up.
    Runner::new(&order, &ScriptCache::default(), &mut state).run_source(
        "TestPackGuardRef.AddScriptPackage TestWithActions",
        None,
        None,
    );
    assert_eq!(state.events, [begin(WITH_ACTIONS)]);
    actions::begin(&mut state, me, FormId(WITH_ACTIONS), false);
    assert_eq!(state.events.len(), 1);
    // The end action, asked by the AI: once per start.
    let end = Event::PackageAction {
        who: me,
        package: FormId(WITH_ACTIONS),
        kind: PackageActionKind::End,
    };
    actions::end(&mut state, me, FormId(WITH_ACTIONS));
    actions::end(&mut state, me, FormId(WITH_ACTIONS));
    assert_eq!(state.events, [begin(WITH_ACTIONS), end.clone()]);
    actions::begin(&mut state, me, FormId(WITH_ACTIONS), true);
    actions::end(&mut state, me, FormId(WITH_ACTIONS));
    assert_eq!(
        state.events,
        [begin(WITH_ACTIONS), end.clone(), begin(WITH_ACTIONS), end]
    );
}

#[test]
fn a_travel_arriving_out_of_sight_ends_once() {
    let (_data, order) = order();
    let mut state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    state.script_packages.insert(me, FormId(WITH_ACTIONS));
    let mut navs = world::ai::NavCache::default();
    let mut ends = 0;
    for _ in 0..3 {
        world::ai::move_offstage(&order, &mut state, me, 2000.0, &mut navs);
        ends = state
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::PackageAction {
                        kind: PackageActionKind::End,
                        ..
                    }
                )
            })
            .count();
    }
    assert_eq!(state.place(&order, me).unwrap().2[0].round(), 800.0);
    assert_eq!(ends, 1);
}

#[test]
fn performing_an_action_runs_its_script_says_its_topic_and_gives_its_idle() {
    let (_data, order) = order();
    let mut state = GameState::new(&order);
    let me = FormId(GUARD_REF);
    let p = load(&order, WITH_ACTIONS);
    let cache = ScriptCache::default();
    let idle = actions::perform(
        &mut Runner::new(&order, &cache, &mut state),
        me,
        &p,
        PackageActionKind::Begin,
    );
    assert_eq!(idle, Some(FormId(IDLE)));
    assert_eq!(state.global(&order, "TestPackGlobal"), Some(5.0));
    assert_eq!(
        state.events,
        [Event::Talk {
            speaker: me,
            to: FormId(0),
            topic: Some(FormId(TOPIC)),
            conversation: false,
        }]
    );
    // The end action's `RemoveScriptPackage` runs on them.
    state.script_packages.insert(me, FormId(WITH_ACTIONS));
    state.events.clear();
    let idle = actions::perform(
        &mut Runner::new(&order, &cache, &mut state),
        me,
        &p,
        PackageActionKind::End,
    );
    assert_eq!(idle, None);
    assert!(!state.script_packages.contains_key(&me));
    assert!(state.evaluate.contains(&me));
    // The empty change action does nothing.
    state.events.clear();
    let idle = actions::perform(
        &mut Runner::new(&order, &cache, &mut state),
        me,
        &p,
        PackageActionKind::Change,
    );
    assert_eq!(idle, None);
    assert!(state.events.is_empty());
}
