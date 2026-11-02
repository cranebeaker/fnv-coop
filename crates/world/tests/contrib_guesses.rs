//! The Dead Money contributor's untraced rules ([G]), which run only with
//! `world::guesses` on (the viewer's `NV_GUESSES=1`). One test, so turning
//! the process-wide switch on can't race another test in this binary.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::more::ids::*;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn ask(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, expr: &str) -> f32 {
    state.globals.insert(FormId(VALUE), -12345.0);
    Runner::new(order, scripts, state).run_source(&format!("set TestValue to {expr}"), None, None);
    state.globals[&FormId(VALUE)]
}

#[test]
fn with_guesses_on_teammates_follow_and_people_on_their_feet_animate() {
    let data = testdata::more::more("contrib-guesses");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(ROOM));
    state.player_position = Some([0.0, 100.0, 0.0]);
    let who = FormId(PERSON_REF);
    world::guesses::set(true);

    // A teammate with nothing to do follows the player at 200 units.
    let before = world::ai::current_package(&order, &state, who);
    assert!(before
        .as_ref()
        .is_none_or(|p| p.kind != world::ai::kinds::FOLLOW));
    state.teammates.insert(who);
    let package = world::ai::current_package(&order, &state, who).unwrap();
    assert_eq!(package.kind, world::ai::kinds::FOLLOW);
    assert_eq!(
        world::ai::followed(&package),
        Some((PLAYER_REF, world::ai::TEAMMATE_DISTANCE as f32))
    );
    state.teammates.remove(&who);

    // A person on their feet is playing an animation; one who is down isn't.
    assert_eq!(
        ask(&order, &scripts, &mut state, "PersonRef.IsAnimPlaying"),
        1.0
    );
    Runner::new(&order, &scripts, &mut state).run_source(
        "PersonRef.SetActorRefEssential 1",
        None,
        None,
    );
    world::combat::hurt(&order, &mut state, who, 100000.0, PLAYER_REF);
    assert!(state.more.down.contains_key(&who));
    assert_eq!(
        ask(&order, &scripts, &mut state, "PersonRef.IsAnimPlaying"),
        0.0
    );

    world::guesses::set(false);
}
