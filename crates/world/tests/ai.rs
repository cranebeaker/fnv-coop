//! The AI's rules read from a plugin built from scratch (`testdata::ai`):
//! turning speeds from the records, dialogue packages walking up to the
//! player and talking (a "Say To" line, a second location the player must
//! be at), travel radii by what's there, conversations between people
//! following their topics and speakers, food taken and eaten, and scripts
//! asking for packages to be looked at again.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::ai::ids::*;
use world::ai::{self, DialogueStep};
use world::dialogue::Speaker;
use world::movement::{self, MoveSettings};
use world::scripting::{GameState, Runner, ScriptCache};
use world::social;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::ai::world(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

/// A new game with the player standing in the test cell at `at`.
fn with_player(order: &LoadOrder, at: [f32; 3]) -> GameState {
    let mut state = GameState::new(order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some(at);
    state
}

#[test]
fn turning_speeds_come_from_the_records() {
    let (_data, order) = order("ai-turning");
    let s = MoveSettings::read(&order, &|_, _| None);
    // People 90°/s; a creature its TNAM, 0 → 45.
    assert_eq!(
        movement::turning_speed(&order, FormId(GREETER_REF), &s),
        90.0
    );
    assert_eq!(movement::turning_speed(&order, FormId(FAST_REF), &s), 200.0);
    assert_eq!(movement::turning_speed(&order, FormId(SLOW_REF), &s), 45.0);
    // In place: × 1.5 (2.5 in combat) for people, × 1.25 for creatures.
    assert!((s.in_place_rate(90.0, false, false).to_degrees() - 135.0).abs() < 0.01);
    assert!((s.in_place_rate(90.0, false, true).to_degrees() - 225.0).abs() < 0.01);
    assert!((s.in_place_rate(200.0, true, true).to_degrees() - 250.0).abs() < 0.01);
    // The INI's [Pathfinding] scale wins when set.
    let faster = MoveSettings::read(&order, &|section, key| {
        (section == "Pathfinding" && key == "fAITurnSpeedScale").then_some(2.0)
    });
    assert!((faster.in_place_rate(90.0, false, false).to_degrees() - 180.0).abs() < 0.01);
}

#[test]
fn dialogue_packages_read_their_data() {
    let (_data, order) = order("ai-pkdd");
    let talk = ai::dialogue_data(&order, FormId(TALK_PACKAGE)).unwrap();
    assert_eq!(talk.topic, Some(FormId(TALK_TOPIC)));
    assert!(!talk.say_to);
    assert_eq!(talk.fov, 100.0);
    let say = ai::dialogue_data(&order, FormId(SAY_PACKAGE)).unwrap();
    assert!(say.say_to);
    assert_eq!(say.topic, None);
    let second = ai::second_location(&order, FormId(WAIT_PACKAGE)).unwrap();
    assert_eq!((second.kind, second.form), (0, FormId(TRIGGER_REF)));
    assert!(ai::second_location(&order, FormId(TALK_PACKAGE)).is_none());
}

#[test]
fn dialogue_packages_walk_up_to_the_player_and_talk() {
    let (_data, order) = order("ai-dialogue");
    let step = |state: &GameState, who: u32, at_place: bool| {
        let p = ai::current_package(&order, state, FormId(who)).unwrap();
        ai::dialogue_step(&order, state, FormId(who), &p, at_place, 20.25).unwrap()
    };
    // Talking to the player: within 100 + the walker's radius (at least
    // 32), measured flat; farther, they walk up. Its place (the editor
    // location) first.
    let state = with_player(&order, [-300.0, 0.0, 0.0]);
    assert_eq!(step(&state, TALKER_REF, false), DialogueStep::Travel);
    assert_eq!(
        step(&state, TALKER_REF, true),
        DialogueStep::Approach { reach: 100.0 }
    );
    let state = with_player(&order, [-370.0, 0.0, 60.0]);
    assert_eq!(step(&state, TALKER_REF, true), DialogueStep::Talk);
    // "Say To": within 120 (3D), a line; no place to go first.
    let state = with_player(&order, [-500.0, -700.0, 0.0]);
    assert_eq!(
        step(&state, SAYER_REF, false),
        DialogueStep::Approach { reach: 120.0 }
    );
    let state = with_player(&order, [-500.0, -610.0, 0.0]);
    assert_eq!(step(&state, SAYER_REF, true), DialogueStep::Say);
    // A second location: nothing until the player is in the trigger box,
    // even within reach.
    let state = with_player(&order, [600.0, -150.0, 0.0]);
    assert_eq!(step(&state, WAITER_REF, true), DialogueStep::Wait);
    // In it but 290 away (more than 256 + 32): walk up.
    let state = with_player(&order, [600.0, 90.0, 0.0]);
    assert_eq!(
        step(&state, WAITER_REF, true),
        DialogueStep::Approach { reach: 256.0 }
    );
    let state = with_player(&order, [600.0, -90.0, 0.0]);
    assert_eq!(step(&state, WAITER_REF, true), DialogueStep::Talk);
    // In another place: wait.
    let mut state = with_player(&order, [600.0, -90.0, 0.0]);
    state.player_cell = Some(FormId(0xFFFF));
    assert_eq!(step(&state, WAITER_REF, true), DialogueStep::Wait);
}

#[test]
fn travel_radii_follow_what_is_there() {
    let (_data, order) = order("ai-radii");
    let mut state = with_player(&order, [0.0; 3]);
    let who = FormId(GREETER_REF);
    let to = |state: &mut GameState, package: u32| {
        state.script_packages.insert(who, FormId(package));
        let p = ai::current_package(&order, state, who).unwrap();
        ai::destination(&order, state, who, &p).unwrap()
    };
    // An XMarker 20, furniture 10, anything else half its bounds' diagonal
    // (40 × 40 × 40: 34.6, rounded) + 20.
    assert_eq!(to(&mut state, TO_MARKER), ([300.0, 300.0, 0.0], 20.0));
    assert_eq!(to(&mut state, TO_CHAIR), ([-300.0, 300.0, 0.0], 10.0));
    assert_eq!(to(&mut state, TO_TRIGGER), ([600.0, 0.0, 0.0], 55.0));
    // Near the editor location: by the person's own bounds (40 × 30 × 128:
    // 137.4 ÷ 2 = 68.7 → 69) + 20.
    state.script_packages.remove(&FormId(TALKER_REF));
    let p = ai::current_package(&order, &state, FormId(TALKER_REF)).unwrap();
    let (at, r) = ai::destination(&order, &state, FormId(TALKER_REF), &p).unwrap();
    assert_eq!((at, r), ([-500.0, 0.0, 0.0], 89.0));
}

#[test]
fn conversations_follow_topics_and_speakers() {
    let (_data, order) = order("ai-conversation");
    let state = with_player(&order, [0.0; 3]);
    let speaker = |r: u32, b: u32| Speaker::load(&order, FormId(r), FormId(b)).unwrap();
    let chatty = speaker(CHATTY_REF, CHATTY);
    let chatty2 = speaker(CHATTY2_REF, CHATTY2);
    let greeter = speaker(GREETER_REF, GREETER);
    let mut dice = 7u64;
    let mut roll = move || {
        dice ^= dice << 13;
        dice ^= dice >> 7;
        dice ^= dice << 17;
        dice
    };
    // Chatty's hello (only to Chatty Two), its follow-up said by the other
    // (next speaker 0), then nothing more: two lines.
    let lines = social::conversation(&order, &state, &chatty, &chatty2, None, &mut roll);
    let said: Vec<(u32, u32, u32)> = lines
        .iter()
        .map(|l| (l.speaker.0, l.listener.0, l.info.form_id.0))
        .collect();
    assert_eq!(
        said,
        vec![
            (CHATTY_REF, CHATTY2_REF, CHATTY_HELLO),
            (CHATTY2_REF, CHATTY_REF, CHATTY2_FOLLOW_UP),
        ]
    );
    // To the greeter Chatty has no hello: GOODBYE instead.
    let lines = social::conversation(&order, &state, &chatty, &greeter, None, &mut roll);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].info.form_id, FormId(CHATTY_GOODBYE));
    // Chatty Two has neither: no conversation.
    assert!(social::conversation(&order, &state, &chatty2, &chatty, None, &mut roll).is_empty());
    // The greeter's hello and chatter, said to the player.
    let hello = social::pick_for(
        &order,
        social::topics::HELLO,
        &greeter,
        world::dialogue::PLAYER_REF,
        &state,
        &[],
    )
    .unwrap();
    assert_eq!(hello.form_id, FormId(GREETER_HELLO));
    let chatter = social::pick_for(
        &order,
        social::topics::IDLE_CHATTER,
        &greeter,
        world::dialogue::PLAYER_REF,
        &state,
        &[],
    )
    .unwrap();
    assert_eq!(chatter.form_id, FormId(GREETER_CHATTER));
    assert_eq!(social::next_speaker(&order, FormId(CHATTY_HELLO)), 0);
}

#[test]
fn food_is_taken_and_eaten() {
    let (_data, order) = order("ai-food");
    let mut state = with_player(&order, [0.0; 3]);
    // Carried food (the greeter's record gives one).
    let greeter = FormId(GREETER_REF);
    assert_eq!(
        world::sandbox::carried_food(&order, &state, greeter),
        Some(FormId(FOOD))
    );
    assert!(world::sandbox::eat(
        &order,
        &mut state,
        greeter,
        FormId(FOOD)
    ));
    assert_eq!(state.item_count(&order, greeter, FormId(FOOD)), 0);
    assert!(world::sandbox::carried_food(&order, &state, greeter).is_none());
    // Food on the floor: taken (gone from the world), then eaten.
    let chatty = FormId(CHATTY_REF);
    assert_eq!(
        world::sandbox::take_food(&order, &mut state, chatty, FormId(FOOD_REF)),
        Some(FormId(FOOD))
    );
    assert!(!world::enabled_now(
        &order,
        FormId(FOOD_REF),
        &state.disabled
    ));
    assert_eq!(state.item_count(&order, chatty, FormId(FOOD)), 1);
    // Not twice.
    assert!(world::sandbox::take_food(&order, &mut state, chatty, FormId(FOOD_REF)).is_none());
    assert!(world::sandbox::eat(
        &order,
        &mut state,
        chatty,
        FormId(FOOD)
    ));
    assert_eq!(state.item_count(&order, chatty, FormId(FOOD)), 0);
    // No sound for the player from someone else's meal.
    assert!(!state
        .events
        .iter()
        .any(|e| matches!(e, world::scripting::Event::Sound(_))));
}

#[test]
fn a_straight_line_on_the_navmesh_is_the_path() {
    let (_data, order) = order("ai-straight");
    let mesh = ai::NavMesh::load(&order, FormId(CELL));
    // Across both triangles: the line stays on the navmesh.
    let path = mesh
        .path([-900.0, -800.0, 0.0], [800.0, 900.0, 0.0])
        .unwrap();
    assert_eq!(path, vec![[-900.0, -800.0, 0.0], [800.0, 900.0, 0.0]]);
}

#[test]
fn scripts_ask_for_packages_to_be_looked_at_again() {
    let data = testdata::quests("ai-evaluate");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(testdata::quest_ids::DOC_REF);
    Runner::new(&order, &scripts, &mut state).run_source("DocRef.EvaluatePackage", None, None);
    assert!(state.evaluate.contains(&doc));
    state.evaluate.clear();
    Runner::new(&order, &scripts, &mut state).run_source(
        "DocRef.AddScriptPackage TestFarTravelPackage",
        None,
        None,
    );
    assert!(state.evaluate.contains(&doc));
}

#[test]
fn travels_end_facing_a_heading_marker() {
    let (_data, order) = order("ai-arrival");
    let state = with_player(&order, [0.0, 0.0, 0.0]);
    let package = |id: u32| ai::Package::load(&order, FormId(id)).unwrap();
    let east = std::f32::consts::FRAC_PI_2;
    // A travel to an XMarkerHeading: its heading; to an XMarker: none.
    let h = ai::arrival_heading(
        &order,
        &state,
        FormId(GREETER_REF),
        &package(TO_HEADING_MARKER),
    );
    assert!((h.unwrap() - east).abs() < 1e-5, "{h:?}");
    assert_eq!(
        ai::arrival_heading(&order, &state, FormId(GREETER_REF), &package(TO_MARKER)),
        None
    );
    // A dialogue package near the editor location: where they were placed
    // facing.
    assert_eq!(
        ai::arrival_heading(&order, &state, FormId(TALKER_REF), &package(TALK_PACKAGE)),
        Some(0.0)
    );
    // A wander package's place isn't the travel procedure's end.
    assert_eq!(
        ai::arrival_heading(&order, &state, FormId(GREETER_REF), &package(WANDER_SMALL)),
        None
    );
    // Out of sight the heading is set as they arrive (`0090ad40`).
    let mut state = with_player(&order, [5000.0, 0.0, 0.0]);
    state
        .script_packages
        .insert(FormId(GREETER_REF), FormId(TO_HEADING_MARKER));
    let mut navs = ai::NavCache::default();
    let done = ai::move_offstage(&order, &mut state, FormId(GREETER_REF), 1000.0, &mut navs);
    assert_eq!(done, ai::Offstage::Arrived);
    let (at, heading) = state.positions[&FormId(GREETER_REF)];
    assert!(
        (at[0] + 300.0).hypot(at[1] + 300.0) <= 20.0 + 1e-3,
        "{at:?}"
    );
    assert!((heading - east).abs() < 1e-5, "{heading}");
}

#[test]
fn wander_packages_stand_wander_and_come_back_by_their_radius() {
    let (_data, order) = order("ai-wander");
    let package = |id: u32| ai::Package::load(&order, FormId(id)).unwrap();
    // The package's radius; an activator with none: half its bounds'
    // diagonal (40 × 40 × 40: 34.6 → 35); "in a cell": around the person,
    // 800 indoors.
    assert_eq!(
        ai::wander_radius(&order, &package(WANDER_SMALL), true),
        (40.0, false)
    );
    assert_eq!(
        ai::wander_radius(&order, &package(WANDER_WIDE), true),
        (400.0, false)
    );
    assert_eq!(
        ai::wander_radius(&order, &package(WANDER_TRIGGER), true),
        (35.0, false)
    );
    assert_eq!(
        ai::wander_radius(&order, &package(WANDER_IN_CELL), true),
        (800.0, true)
    );
    // Outdoors "in a cell" has no radius: the loader drops it (`0067f060`)
    // and the radius getter gives 0 for that kind (`0067f1c0`).
    assert_eq!(
        ai::wander_radius(&order, &package(WANDER_IN_CELL), false),
        (0.0, true)
    );
    // Under 60 they stand (facing an XMarkerHeading), or go back when not
    // at the place.
    use ai::WanderStep::*;
    assert_eq!(ai::wander_step(40.0, false, true, 10.0), Stand);
    assert_eq!(ai::wander_step(40.0, false, false, 10.0), Back);
    let state = with_player(&order, [0.0, 0.0, 0.0]);
    let h = ai::marker_heading(&order, &state, &package(WANDER_SMALL)).unwrap();
    assert!((h - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
    // Otherwise they wander until more than the radius + 250 off.
    assert_eq!(ai::wander_step(400.0, false, true, 650.0), Wander);
    assert_eq!(ai::wander_step(400.0, false, true, 651.0), Back);
    // Around themselves: no radius test.
    assert_eq!(ai::wander_step(30.0, true, false, 0.0), Wander);
    // The spots: 32 to 0.75 × the radius.
    assert_eq!(ai::wander_ring(400.0), (32.0, 300.0));
}
