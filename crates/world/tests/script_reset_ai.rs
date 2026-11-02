//! ResetAI is a deferred actor reset, not an alias of EvaluatePackage.
use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::functions::ids::{ADULT_REF, CUP_REF};
use world::scripting::{GameState, Runner, ScriptCache};

fn run(order: &LoadOrder, state: &mut GameState, text: &str) {
    Runner::new(order, &ScriptCache::default(), state).run_source(text, None, None);
}

#[test]
fn only_reset_ai_releases_furniture_at_the_deferred_update() {
    let data = testdata::functions::functions("reset-ai-furniture");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let mut state = GameState::new(&order);
    let who = FormId(ADULT_REF);
    // Furniture identity is opaque to reset; loading/claiming its marker is
    // tested by furniture tests. Reset must forget both ownership and process.
    let chair = FormId(0x123456);
    let place = ([12.0, 34.0, 56.0], 0.7);
    state.sit(who, chair);
    let mut sitter = world::furniture::Sitter::new(
        chair,
        world::furniture::PlacedMarker {
            number: 14,
            ..Default::default()
        },
        world::furniture::MarkerSettings {
            delta: [0.0; 3],
            heading_delta: 0.0,
        },
        place.0,
        place.1,
    );
    sitter.state = world::furniture::SitState::Sitting;
    state.sitters.insert(who, sitter);
    state.positions.insert(who, place);
    run(&order, &mut state, "AdultRef.EvaluatePackage");
    assert!(state.evaluate.contains(&who));
    assert!(!state.take_ai_reset(who));
    assert_eq!(state.furniture.get(&who), Some(&chair));
    state.evaluate.clear();
    run(&order, &mut state, "AdultRef.ResetAI");
    assert_eq!(state.furniture.get(&who), Some(&chair));
    assert!(state.reset_ai.contains(&who));
    assert!(state.take_ai_reset(who));
    assert!(!state.furniture.contains_key(&who));
    assert!(!state.sitters.contains_key(&who));
    assert!(state.evaluate.contains(&who));
    assert_eq!(state.positions.get(&who), Some(&place));
    assert!(!state.take_ai_reset(who));
}

#[test]
fn invalid_disabled_and_dead_targets_do_not_request_reset() {
    let data = testdata::functions::functions("reset-ai-targets");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let mut state = GameState::new(&order);
    let who = FormId(ADULT_REF);
    run(&order, &mut state, "CupRef.ResetAI");
    assert!(!state.reset_ai.contains(&FormId(CUP_REF)));
    state.disabled.insert(who, true);
    run(&order, &mut state, "AdultRef.ResetAI");
    assert!(state.reset_ai.is_empty());
    state.disabled.insert(who, false);
    state.dead.insert(who);
    run(&order, &mut state, "AdultRef.ResetAI");
    assert!(state.reset_ai.is_empty());
}
