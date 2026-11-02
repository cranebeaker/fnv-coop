//! Ready-made test characters (`world::character`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::more::ids::*;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, ScriptCache};

#[test]
fn a_character_file_sets_up_the_game() {
    let data = testdata::more::more("character");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let file = "\
; a comment, then a blank line

level 12
set TestValue to 7
player.AddItem TestGun 1
NoSuchThing.Enable
AddItemToLeveledList TestGun TestGun 1 1 100
level twelve
";
    let problems = world::character::apply(&order, &scripts, &mut state, file);
    // The level, with its experience and no level-up waiting.
    assert_eq!(state.player_level, 12);
    assert!(!state.level_up_pending);
    let xp = state.actor_values[&(PLAYER_REF, world::experience::XP)];
    assert_eq!(xp, world::experience::xp_for_level(&order, 12));
    // Script lines ran.
    assert_eq!(state.globals[&FormId(VALUE)], 7.0);
    assert_eq!(state.items.get(&(PLAYER_REF, FormId(GUN))), Some(&1));
    // What didn't take, by line.
    let lines: Vec<usize> = problems.iter().map(|p| p.line).collect();
    assert_eq!(lines, [6, 7, 8], "{problems:?}");
    assert!(problems[0].why.contains("NoSuchThing"));
    assert!(problems[1].why.contains("isn't carried out"));
    assert!(problems[2].why.contains("level N"));
}
