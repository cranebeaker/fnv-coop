//! Dropping things into the world (`Drop`, `DropMe`, `OnDrop`) and picking
//! them up again.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::quest_ids::*;
use world::dialogue::PLAYER_REF;
use world::scripting::{GameState, Runner, ScriptCache};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::quests(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

fn run(order: &LoadOrder, scripts: &ScriptCache, state: &mut GameState, line: &str) {
    Runner::new(order, scripts, state).run_source(line, None, None);
    assert!(state.unhandled.is_empty(), "{:?}", state.unhandled_first);
}

fn global(state: &GameState) -> f32 {
    state.globals.get(&FormId(GLOBAL)).copied().unwrap_or(0.0)
}

/// `ref.Drop item count` (`005b58d0` → `004c6dd0`): the things leave and
/// lie 50 units in front of the holder, 30 up, as one reference standing
/// for them all; picked up, they come back.
#[test]
fn dropping_and_picking_up() {
    let (_data, order) = order("drop-basic");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some([100.0, 200.0, 0.0]);
    run(&order, &scripts, &mut state, "player.AddItem TestCup 5");
    run(&order, &scripts, &mut state, "player.Drop TestCup 3");
    let cup = FormId(CUP);
    assert_eq!(state.item_count(&order, PLAYER_REF, cup), 2);
    let (&id, made) = state.more.placed.refs.iter().next().unwrap();
    let space = made.space;
    assert_eq!((made.base, made.count), (cup, 3));
    // Facing north (heading 0): 50 along y.
    assert_eq!(made.position, [100.0, 250.0, 30.0]);
    // Facing east (heading 90 degrees clockwise): 50 along x.
    let mut east = state.clone();
    east.player_heading = std::f32::consts::FRAC_PI_2;
    east.more.placed.refs.clear();
    run(&order, &scripts, &mut east, "player.Drop TestCup 1");
    let m = east.more.placed.refs.values().next().unwrap();
    assert!(
        (m.position[0] - 150.0).abs() < 1e-3 && (m.position[1] - 200.0).abs() < 1e-3,
        "{:?}",
        m.position
    );
    let here = world::more_functions::placed::dropped_items(&order, &state, space);
    assert_eq!(here.len(), 1);
    assert_eq!((here[0].reference, here[0].count), (id, 3));
    state.pick_up(&order, id, cup, 3);
    assert_eq!(state.item_count(&order, PLAYER_REF, cup), 5);
    assert!(world::more_functions::placed::dropped_items(&order, &state, space).is_empty());
    // A save keeps a dropped thing's count.
    run(&order, &scripts, &mut state, "player.Drop TestCup 2");
    let text = world::save::save(&state, None);
    let (back, _) = world::save::load(&text).unwrap();
    assert!(back
        .more
        .placed
        .refs
        .values()
        .any(|m| m.base == cup && m.count == 2));
}

/// `OnDrop Player` (`005750a0` flags it on any `RemoveItem`): dropped into
/// the world, or put in a container.
#[test]
fn ondrop_runs_when_the_player_lets_go() {
    let (_data, order) = order("drop-ondrop");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    state.player_cell = Some(FormId(CELL));
    state.player_position = Some([0.0, 0.0, 0.0]);
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddItem TestKeepsake 2",
    );
    Runner::new(&order, &scripts, &mut state).update(0.1);
    state.globals.insert(FormId(GLOBAL), 0.0);
    run(&order, &scripts, &mut state, "player.Drop TestKeepsake 1");
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(global(&state), 42.0);
    // Into a container too.
    state.globals.insert(FormId(GLOBAL), 0.0);
    state.move_item(&order, PLAYER_REF, FormId(CHEST_REF), FormId(KEEPSAKE), 1);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(global(&state), 42.0);
}

/// `DropMe` (`005b5860`): a gift a companion won't keep lands at their
/// feet, its script with it.
#[test]
fn dropme_lands_in_the_world() {
    let (_data, order) = order("drop-dropme");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let doc = FormId(DOC_REF);
    let gift = FormId(REFUSED_GIFT);
    run(
        &order,
        &scripts,
        &mut state,
        "player.AddItem TestRefusedGift 1",
    );
    state.move_item(&order, PLAYER_REF, doc, gift, 1);
    assert_eq!(state.item_count(&order, doc, gift), 1);
    Runner::new(&order, &scripts, &mut state).update(0.1);
    assert_eq!(state.item_count(&order, doc, gift), 0);
    let made: Vec<_> = state
        .more
        .placed
        .refs
        .iter()
        .filter(|(_, m)| m.base == gift)
        .map(|(&r, _)| r)
        .collect();
    assert_eq!(made.len(), 1);
    assert!(state.item_scripts.iter().any(|s| s.holder == made[0]));
}

/// The Pip-Boy's Drop (`00780140` case 7): its refusals in the game's
/// order (a quest item, something equipped mid-action, in the air), then
/// "How many?" for more than `iInventoryAskQuantityAt` (5).
#[test]
fn the_pipboys_drop_refusals_and_how_many() {
    let (_data, order) = order("drop-refusals");
    let scripts = ScriptCache::default();
    let mut state = GameState::new(&order);
    let cup = FormId(CUP);
    run(&order, &scripts, &mut state, "player.AddItem TestCup 6");
    let refusal = |state: &GameState, acting, in_air| {
        world::items::drop_refusal(&order, state, cup, acting, in_air)
    };
    assert_eq!(refusal(&state, true, false), None);
    assert_eq!(refusal(&state, false, true), Some("sNoJumpWarning"));
    state.equipped.entry(PLAYER_REF).or_default().push(cup);
    assert_eq!(refusal(&state, false, false), None);
    assert_eq!(
        refusal(&state, true, true),
        Some("sDropEquippedItemWarning")
    );
    run(&order, &scripts, &mut state, "SetQuestObject TestCup 1");
    assert_eq!(refusal(&state, true, true), Some("sDropQuestItemWarning"));
    assert!(!world::items::drop_asks(&order, 5));
    assert!(world::items::drop_asks(&order, 6));
}
