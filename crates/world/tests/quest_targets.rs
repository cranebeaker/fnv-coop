//! Quest targets (`world::quest_targets`): `QSTA` read per objective, the
//! active quest chosen as the game chooses it, the targets shown, and the
//! door the compass points at for a target in another place.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_targets::ids::*;
use world::dialogue::PLAYER_REF;
use world::quest::Quest;
use world::quest_targets::{current_targets, door_path, DoorGraph, Tracker};
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quest_targets::quest_targets(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn put_player(state: &mut GameState, space: u32, cell: u32, at: [f32; 3]) {
    state.player_world = (space != cell).then_some(FormId(space));
    state.player_cell = Some(FormId(cell));
    state.player_position = Some(at);
}

#[test]
fn objectives_keep_their_targets_and_the_conditions_after_each() {
    let (_data, order) = order("qt-parse");
    let quest = Quest::load(&order, FormId(QUEST)).unwrap();
    let first = quest.objective(10).unwrap();
    assert_eq!(first.text, "Objective 10");
    assert_eq!(first.targets.len(), 1);
    assert_eq!(first.targets[0].reference, FormId(BASEMENT_CRATE));
    assert_eq!(first.targets[0].flags, 0);
    assert!(first.targets[0].conditions.is_empty());
    let second = quest.objective(20).unwrap();
    let refs: Vec<FormId> = second.targets.iter().map(|t| t.reference).collect();
    assert_eq!(refs, [FormId(SHOP_CRATE), FormId(SHOP_TILL)]);
    // The condition belongs to the till only, not to the stage or quest.
    assert!(second.targets[0].conditions.is_empty());
    assert_eq!(second.targets[1].conditions.len(), 1);
    assert_eq!(second.targets[1].conditions[0].function, 59);
    assert!(quest.conditions.is_empty());
    assert!(quest.stages[0].entries[0].conditions.is_empty());
}

#[test]
fn the_first_quest_shown_becomes_active_until_it_completes() {
    let (_data, order) = order("qt-active");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    assert_eq!(state.active_quest, None);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source("SetObjectiveDisplayed QTQuest 10 1", None, None);
    runner.run_source("SetObjectiveDisplayed QTOther 10 1", None, None);
    assert_eq!(state.active_quest, Some(FormId(QUEST)));
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source("CompleteQuest QTQuest", None, None);
    assert_eq!(state.active_quest, None);
    // A completed quest's objectives don't make it active again.
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source("SetObjectiveDisplayed QTQuest 20 1", None, None);
    assert_eq!(state.active_quest, None);
    let mut runner = Runner::new(&order, &scripts, &mut state);
    runner.run_source("ForceActiveQuest QTOther", None, None);
    assert_eq!(state.active_quest, Some(FormId(OTHER_QUEST)));
}

#[test]
fn only_the_active_quests_shown_uncompleted_objectives_with_passing_conditions() {
    let (_data, order) = order("qt-current");
    let quest = Quest::load(&order, FormId(QUEST)).unwrap();
    let mut state = GameState::new(&order);
    let refs = |state: &GameState| -> Vec<FormId> {
        current_targets(&order, &quest, state)
            .iter()
            .map(|t| t.reference)
            .collect()
    };
    // Shown but not the active quest: nothing.
    state.objectives.insert((FormId(QUEST), 10), false);
    assert!(refs(&state).is_empty());
    state.active_quest = Some(FormId(QUEST));
    assert_eq!(refs(&state), [FormId(BASEMENT_CRATE)]);
    // Completed objectives drop out; the till waits for stage 5.
    state.objectives.insert((FormId(QUEST), 10), true);
    state.objectives.insert((FormId(QUEST), 20), false);
    assert_eq!(refs(&state), [FormId(SHOP_CRATE)]);
    state.stages_done.insert((FormId(QUEST), 5));
    assert_eq!(refs(&state), [FormId(SHOP_CRATE), FormId(SHOP_TILL)]);
    // A completed quest shows none.
    state.completed.insert(FormId(QUEST));
    assert!(refs(&state).is_empty());
}

#[test]
fn the_compass_follows_the_first_door_toward_another_place() {
    let (_data, order) = order("qt-path");
    let mut state = GameState::new(&order);
    let mut graph = DoorGraph::default();
    // Outdoors: into the house, then down to the basement.
    let path = door_path(
        &order,
        &state,
        &mut graph,
        FormId(WORLD),
        [0.0; 3],
        FormId(BASEMENT),
    );
    assert_eq!(
        path,
        Some(vec![FormId(HOUSE_OUTSIDE), FormId(HOUSE_TO_BASEMENT)])
    );
    // From the basement out to the world: up, then out of the house.
    let path = door_path(
        &order,
        &state,
        &mut graph,
        FormId(BASEMENT),
        [0.0; 3],
        FormId(WORLD),
    );
    assert_eq!(path, Some(vec![FormId(BASEMENT_DOOR), FormId(HOUSE_DOOR)]));
    // Already there: no doors.
    let path = door_path(
        &order,
        &state,
        &mut graph,
        FormId(BASEMENT),
        [0.0; 3],
        FormId(BASEMENT),
    );
    assert_eq!(path, Some(Vec::new()));

    // The tracker: the active quest's target in the basement.
    state.active_quest = Some(FormId(QUEST));
    state.objectives.insert((FormId(QUEST), 10), false);
    put_player(&mut state, WORLD, PERSISTENT, [0.0; 3]);
    let mut tracker = Tracker::default();
    let shown = tracker.shown(&order, &state);
    assert_eq!(shown.len(), 1);
    assert_eq!(shown[0].target.reference, FormId(BASEMENT_CRATE));
    assert_eq!(shown[0].follow, FormId(HOUSE_OUTSIDE));
    put_player(&mut state, HOUSE, HOUSE, [0.0; 3]);
    assert_eq!(
        tracker.shown(&order, &state)[0].follow,
        FormId(HOUSE_TO_BASEMENT)
    );
    // In the same place the target itself.
    put_player(&mut state, BASEMENT, BASEMENT, [0.0; 3]);
    assert_eq!(
        tracker.shown(&order, &state)[0].follow,
        FormId(BASEMENT_CRATE)
    );
}

#[test]
fn a_locked_door_costs_more_unless_the_player_has_its_key() {
    let (_data, order) = order("qt-locks");
    let mut state = GameState::new(&order);
    let mut graph = DoorGraph::default();
    let first = |state: &GameState, graph: &mut DoorGraph| {
        door_path(&order, state, graph, FormId(WORLD), [0.0; 3], FormId(SHOP))
            .and_then(|p| p.first().copied())
    };
    // The back door is 200 away but locked: the front, 5000 away.
    assert_eq!(first(&state, &mut graph), Some(FormId(SHOP_FRONT_OUTSIDE)));
    // With the key, the back door.
    state.items.insert((PLAYER_REF, FormId(KEY)), 1);
    state.stocked.insert(PLAYER_REF);
    assert_eq!(first(&state, &mut graph), Some(FormId(SHOP_BACK_OUTSIDE)));
    // Unlocked by a script: the back door without a key too.
    state.items.clear();
    state.locks.insert(FormId(SHOP_BACK_OUTSIDE), None);
    assert_eq!(first(&state, &mut graph), Some(FormId(SHOP_BACK_OUTSIDE)));
    // A disabled door isn't used.
    state.disabled.insert(FormId(SHOP_BACK_OUTSIDE), true);
    assert_eq!(first(&state, &mut graph), Some(FormId(SHOP_FRONT_OUTSIDE)));
}
