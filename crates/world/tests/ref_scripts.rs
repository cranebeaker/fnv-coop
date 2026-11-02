//! The reference-script pass (`world::ref_scripts`): which placed objects'
//! scripts run, `OnLoad`, triggers and the pass stopping, on a generated
//! world (`testdata::ref_scripts`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::ref_scripts::ids::*;
use world::dialogue::PLAYER_REF;
use world::ref_scripts::{grid_center, grid_squares, RefScripts};
use world::scripting::{
    interactive_in_square, interactive_references, GameState, Interactive, Runner, ScriptCache,
};
use world::WorldGrid;

struct Fixture {
    _data: testdata::TempData,
    order: LoadOrder,
    cache: ScriptCache,
    grid: WorldGrid,
}

fn fixture(tag: &str) -> Fixture {
    let data = testdata::ref_scripts::ref_scripts(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let grid = WorldGrid::load(&order, FormId(WORLD)).unwrap();
    Fixture {
        _data: data,
        order,
        cache: ScriptCache::default(),
        grid,
    }
}

impl Fixture {
    fn load(&self, cell: FormId) -> Vec<Interactive> {
        let square = self
            .grid
            .cells
            .iter()
            .find(|(_, &c)| c == cell)
            .map(|(s, _)| *s);
        match square {
            Some(square) => interactive_in_square(&self.order, &self.grid, square),
            None => interactive_references(&self.order, cell),
        }
    }

    fn attach(&self, refs: &mut RefScripts, state: &mut GameState, cells: &[u32]) {
        let cells: Vec<FormId> = cells.iter().map(|&c| FormId(c)).collect();
        let mut runner = Runner::new(&self.order, &self.cache, state);
        refs.attach(&mut runner, &cells, |c| self.load(c));
    }

    fn frame(
        &self,
        refs: &mut RefScripts,
        state: &mut GameState,
        people: &[(FormId, [f32; 3])],
    ) -> Vec<FormId> {
        let mut runner = Runner::new(&self.order, &self.cache, state);
        refs.frame(&mut runner, people)
    }
}

fn var(state: &GameState, reference: u32, name: &str) -> f64 {
    state
        .variables
        .get(&FormId(reference))
        .and_then(|l| l.get(name))
        .unwrap_or(0.0)
}

#[test]
fn the_grid_moves_only_past_the_games_margin() {
    // `00452580`: 4096 units from the centre cell's centre (x 2048).
    assert_eq!(grid_center(None, [100.0, 100.0, 0.0], 5), (0, 0));
    // `00406d90` rounds (FISTP, halves to even) before `>> 12`.
    assert_eq!(grid_center(None, [4095.5, -0.4, 0.0], 5), (1, 0));
    assert_eq!(grid_center(None, [4094.5, -0.6, 0.0], 5), (0, -1));
    assert_eq!(grid_center(Some((0, 0)), [4000.0, 100.0, 0.0], 5), (0, 0));
    assert_eq!(grid_center(Some((0, 0)), [6143.0, 100.0, 0.0], 5), (0, 0));
    assert_eq!(grid_center(Some((0, 0)), [6144.0, 100.0, 0.0], 5), (1, 0));
    assert_eq!(grid_center(Some((0, 0)), [100.0, -2048.0, 0.0], 5), (0, -1));
    assert_eq!(grid_center(Some((0, 0)), [100.0, -2047.0, 0.0], 5), (0, 0));
    // Back by one cell: still inside the new centre's margin.
    assert_eq!(grid_center(Some((1, 0)), [3000.0, 100.0, 0.0], 5), (1, 0));
    // With a grid of 3, 3072.
    assert_eq!(grid_center(Some((0, 0)), [5119.0, 100.0, 0.0], 3), (0, 0));
    assert_eq!(grid_center(Some((0, 0)), [5120.0, 100.0, 0.0], 3), (1, 0));
}

#[test]
fn grid_squares_run_x_outer_y_inner() {
    let squares = grid_squares((0, 0), 5);
    assert_eq!(squares.len(), 25);
    assert_eq!(&squares[..3], &[(-2, -2), (-2, -1), (-2, 0)]);
    assert_eq!(squares[5], (-1, -2));
    assert_eq!(squares[24], (2, 2));
    assert!(!squares.contains(&(3, 0)));
}

#[test]
fn a_square_holds_the_persistent_objects_standing_in_it() {
    let f = fixture("refscripts-persistent");
    let refs: Vec<FormId> = f.load(FormId(EAST)).iter().map(|r| r.reference).collect();
    assert_eq!(refs, [FormId(MOVER_REF), FormId(PERSISTENT_REF)]);
}

#[test]
fn neighbouring_attached_cells_run_their_scripts() {
    let f = fixture("refscripts-neighbours");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD, FAR]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, COUNTER_REF, "iFrames"), 1.0);
    assert_eq!(var(&state, FAR_REF, "iFrames"), 1.0);
}

#[test]
fn on_load_runs_once_per_attach_not_per_frame_or_grid_move() {
    let f = fixture("refscripts-onload");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD]);
    f.frame(&mut refs, &mut state, &[]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, COUNTER_REF, "iLoads"), 1.0);
    assert_eq!(var(&state, COUNTER_REF, "iFrames"), 2.0);
    // The grid moves; the field stays attached.
    f.attach(&mut refs, &mut state, &[FAR, FIELD]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, COUNTER_REF, "iLoads"), 1.0);
    assert_eq!(var(&state, FAR_REF, "iLoads"), 1.0);
    // Detached, then attached again: loaded again.
    f.attach(&mut refs, &mut state, &[FAR]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, COUNTER_REF, "iFrames"), 3.0);
    f.attach(&mut refs, &mut state, &[FIELD]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, COUNTER_REF, "iLoads"), 2.0);
}

#[test]
fn disabled_objects_run_and_enabling_loads_them() {
    let f = fixture("refscripts-disabled");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD]);
    f.frame(&mut refs, &mut state, &[]);
    // `0054c740` runs a disabled reference's script (flag 0x800).
    assert_eq!(var(&state, DISABLED_REF, "iFrames"), 1.0);
    assert_eq!(var(&state, DISABLED_REF, "iLoads"), 1.0);
    state.disabled.insert(FormId(DISABLED_REF), false);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, DISABLED_REF, "iLoads"), 2.0);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, DISABLED_REF, "iLoads"), 2.0);
}

#[test]
fn people_left_out_as_disabled_come_in_once_enabled() {
    // Ghost Town Gunfight: the Powder Gangers follow
    // `GoodspringsPowderGangMarker`; a square loaded before the marker is
    // enabled must still bring them in afterwards (`Enable`, 005c43d0).
    let f = fixture("refscripts-enabled-people");
    let mut state = GameState::new(&f.order);
    let left_out = world::ai::disabled_people_in_square(&f.order, &f.grid, (0, 1), &state.disabled);
    assert_eq!(left_out, [FormId(FOLLOWER_REF), FormId(LONE_REF)]);
    let world = FormId(WORLD);
    assert!(world::ai::enabled_since_load(&f.order, &state, world, &left_out).is_empty());
    // The enable parent: its follower comes in.
    state.disabled.insert(FormId(DISABLED_REF), false);
    assert_eq!(
        world::ai::enabled_since_load(&f.order, &state, world, &left_out),
        [FormId(FOLLOWER_REF)]
    );
    // Enabled directly; the dead stay out.
    state.disabled.insert(FormId(LONE_REF), false);
    state.dead.insert(FormId(FOLLOWER_REF));
    assert_eq!(
        world::ai::enabled_since_load(&f.order, &state, world, &left_out),
        [FormId(LONE_REF)]
    );
    // Only for the place they stand in.
    assert!(world::ai::enabled_since_load(&f.order, &state, FormId(ROOM), &left_out).is_empty());
}

#[test]
fn trigger_events_enter_then_trigger_then_leave() {
    let f = fixture("refscripts-trigger");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD]);
    let inside = [(PLAYER_REF, [1000.0, 1000.0, 0.0])];
    f.frame(&mut refs, &mut state, &inside);
    assert_eq!(var(&state, TRIGGER_REF, "iEnter"), 1.0);
    assert_eq!(var(&state, TRIGGER_REF, "iAnyEnter"), 1.0);
    // Someone new isn't flagged `OnTrigger` on the step they enter.
    assert_eq!(var(&state, TRIGGER_REF, "iInside"), 0.0);
    f.frame(&mut refs, &mut state, &inside);
    f.frame(&mut refs, &mut state, &inside);
    assert_eq!(var(&state, TRIGGER_REF, "iEnter"), 1.0);
    assert_eq!(var(&state, TRIGGER_REF, "iInside"), 2.0);
    f.frame(&mut refs, &mut state, &[(PLAYER_REF, [0.0; 3])]);
    assert_eq!(var(&state, TRIGGER_REF, "iLeave"), 1.0);
    assert_eq!(var(&state, TRIGGER_REF, "iInside"), 2.0);
    assert!(refs.inside(FormId(TRIGGER_REF)).is_empty());
    assert_eq!(var(&state, TRIGGER_REF, "iFrames"), 4.0);
}

#[test]
fn a_trigger_takes_one_event_per_step() {
    let f = fixture("refscripts-one-event");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD]);
    let both = [
        (PLAYER_REF, [1000.0, 1000.0, 0.0]),
        (FormId(COUNTER_REF), [1010.0, 1000.0, 0.0]),
    ];
    f.frame(&mut refs, &mut state, &both);
    assert_eq!(var(&state, TRIGGER_REF, "iAnyEnter"), 1.0);
    f.frame(&mut refs, &mut state, &both);
    assert_eq!(var(&state, TRIGGER_REF, "iAnyEnter"), 2.0);
    assert_eq!(var(&state, TRIGGER_REF, "iEnter"), 1.0);
}

#[test]
fn disabling_or_detaching_a_trigger_makes_its_occupants_leave() {
    let f = fixture("refscripts-force-leave");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[FIELD]);
    let inside = [(PLAYER_REF, [1000.0, 1000.0, 0.0])];
    f.frame(&mut refs, &mut state, &inside);
    state.disabled.insert(FormId(TRIGGER_REF), true);
    f.frame(&mut refs, &mut state, &inside);
    assert_eq!(var(&state, TRIGGER_REF, "iLeave"), 1.0);
    assert!(refs.inside(FormId(TRIGGER_REF)).is_empty());
    // A disabled trigger isn't updated: no one enters it again.
    f.frame(&mut refs, &mut state, &inside);
    assert_eq!(var(&state, TRIGGER_REF, "iEnter"), 1.0);

    state.disabled.insert(FormId(TRIGGER_REF), false);
    f.frame(&mut refs, &mut state, &inside);
    assert_eq!(var(&state, TRIGGER_REF, "iEnter"), 2.0);
    f.attach(&mut refs, &mut state, &[EAST]);
    assert_eq!(var(&state, TRIGGER_REF, "iLeave"), 2.0);
}

#[test]
fn a_command_that_changes_references_stops_the_pass() {
    let f = fixture("refscripts-stop");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    // 0,0 before 1,0; in 1,0 the mover before the persistent counter.
    f.attach(&mut refs, &mut state, &[FIELD, EAST]);
    let ran = f.frame(&mut refs, &mut state, &[]);
    assert_eq!(ran.last(), Some(&FormId(MOVER_REF)));
    assert_eq!(var(&state, COUNTER_REF, "iFrames"), 1.0);
    assert_eq!(var(&state, MOVER_REF, "iFrames"), 1.0);
    assert_eq!(var(&state, PERSISTENT_REF, "iFrames"), 0.0);
    // Its `OnLoad` waits for its first run.
    assert_eq!(refs.pending(FormId(PERSISTENT_REF)), &[("onload", None)]);
}

/// B32: a start-game quest's once-only guard (the DLC start scripts'
/// `nEnableDLC`) belongs to the quest, not to any cell, and a reference's
/// variables stay with the reference while its cell is detached: changing
/// cell twice and back neither runs the guarded part again nor starts the
/// reference's variables over.
#[test]
fn once_only_guards_survive_cell_changes() {
    let f = fixture("refscripts-once");
    let mut state = GameState::new(&f.order);
    assert!(state.running.contains(&FormId(ONCE_QUEST)));
    let mut refs = RefScripts::default();
    let step = |cells: &[u32], state: &mut GameState, refs: &mut RefScripts| {
        f.attach(refs, state, cells);
        for _ in 0..3 {
            f.frame(refs, state, &[]);
            Runner::new(&f.order, &f.cache, state).update(6.0);
        }
    };
    // Starting indoors, then out, in again and out again.
    step(&[ROOM], &mut state, &mut refs);
    step(&[FIELD], &mut state, &mut refs);
    assert_eq!(var(&state, ONCE_QUEST, "iShown"), 1.0);
    assert_eq!(var(&state, ONCE_QUEST, "nEnableDLC"), 2.0);
    step(&[ROOM], &mut state, &mut refs);
    step(&[FIELD], &mut state, &mut refs);
    assert_eq!(var(&state, ONCE_QUEST, "iShown"), 1.0);
    assert_eq!(var(&state, ONCE_QUEST, "nEnableDLC"), 2.0);
    // The room's counter kept its count across the detach: loaded twice,
    // run on each of its six frames.
    assert_eq!(var(&state, ROOM_REF, "iLoads"), 2.0);
    assert_eq!(var(&state, ROOM_REF, "iFrames"), 6.0);
}

#[test]
fn an_interior_runs_its_own_references() {
    let f = fixture("refscripts-interior");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.attach(&mut refs, &mut state, &[ROOM]);
    f.frame(&mut refs, &mut state, &[]);
    assert_eq!(var(&state, ROOM_REF, "iLoads"), 1.0);
    assert_eq!(var(&state, ROOM_REF, "iFrames"), 1.0);
    assert_eq!(var(&state, COUNTER_REF, "iFrames"), 0.0);
}
