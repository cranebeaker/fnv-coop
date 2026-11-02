//! The character-revision prompt (B29) on a generated world shaped like
//! the game's `VCG04` (`testdata::revise`): a message box's button belongs
//! to the script that showed it (`005b4630`, `005b4a80`), and a reference
//! a script moved runs its script where it now stands (`005ccb20` →
//! `00573800` → `00548230`). Each frame runs as the viewer's does: the
//! attached cells, the moved references placed, the quests, then the
//! reference-script pass; a message box shown is answered before the next
//! frame (the game stands still while it is up).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::revise::ids::*;
use world::dialogue::PLAYER_REF;
use world::ref_scripts::{moved_references, RefScripts};
use world::scripting::{interactive_in_square, Event, GameState, Interactive, Runner, ScriptCache};
use world::WorldGrid;

struct Fixture {
    _data: testdata::TempData,
    order: LoadOrder,
    cache: ScriptCache,
    grid: WorldGrid,
}

fn fixture(tag: &str) -> Fixture {
    let data = testdata::revise::revise(tag);
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
            .map(|(s, _)| *s)
            .unwrap();
        interactive_in_square(&self.order, &self.grid, square)
    }

    /// The player stands at `x` (y 100) in the world.
    fn stand(&self, state: &mut GameState, x: f32) {
        let at = [x, 100.0, 0.0];
        state.player_world = Some(FormId(WORLD));
        state.player_cell = self.grid.cell_at(world::square_of(at));
        state.player_position = Some(at);
    }

    /// One frame with `cells` attached; the message boxes it showed.
    fn frame(&self, refs: &mut RefScripts, state: &mut GameState, cells: &[u32]) -> usize {
        let cells: Vec<FormId> = cells.iter().map(|&c| FormId(c)).collect();
        let mut runner = Runner::new(&self.order, &self.cache, state);
        refs.attach(&mut runner, &cells, |c| self.load(c));
        let moved: Vec<(FormId, Option<FormId>)> = moved_references(&self.order, state)
            .into_iter()
            .map(|r| {
                let cell = state
                    .place(&self.order, r)
                    .filter(|p| p.0 == FormId(WORLD))
                    .and_then(|p| self.grid.cell_at(world::square_of(p.2)));
                (r, cell)
            })
            .collect();
        let mut runner = Runner::new(&self.order, &self.cache, state);
        refs.place_moved(&mut runner, &moved);
        runner.update(1.0 / 30.0);
        let feet = runner.state.player_position.unwrap();
        refs.frame(&mut runner, &[(PLAYER_REF, feet)]);
        boxes(state)
    }
}

/// The message boxes shown since last asked.
fn boxes(state: &mut GameState) -> usize {
    let n = state
        .events
        .iter()
        .filter(|e| matches!(e, Event::Message { buttons, .. } if !buttons.is_empty()))
        .count();
    state.events.clear();
    n
}

fn var(state: &GameState, owner: u32, name: &str) -> f64 {
    state
        .variables
        .get(&FormId(owner))
        .and_then(|l| l.get(name))
        .unwrap_or(0.0)
}

#[test]
fn a_message_boxs_button_is_for_the_script_that_showed_it() {
    let f = fixture("revise-owner");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.stand(&mut state, 100.0);
    // The mailbox's cell runs before the activator's.
    let home = [MAIL, HOME];
    f.frame(&mut refs, &mut state, &home);
    Runner::new(&f.order, &f.cache, &mut state).run_source(
        "ReviseActivatorRef.Activate player 1",
        None,
        None,
    );
    assert_eq!(boxes(&mut state), 1);
    // "Finished": the mailbox, asking every frame and first in the pass,
    // gets -1 (`005b4a80`); the activator gets the button.
    state.button = Some(2);
    f.frame(&mut refs, &mut state, &home);
    assert_eq!(var(&state, MAILBOX_REF, "iTaken"), 0.0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iFinished"), 1.0);
    assert!(!state.running.contains(&FormId(QUEST)));
    // Given once.
    f.frame(&mut refs, &mut state, &home);
    assert_eq!(var(&state, ACTIVATOR_REF, "iFinished"), 1.0);

    // A button nobody read is dropped when the next message is shown
    // (`005b4940`).
    Runner::new(&f.order, &f.cache, &mut state).run_source(
        "ReviseActivatorRef.Activate player 1",
        None,
        None,
    );
    state.button = Some(1);
    Runner::new(&f.order, &f.cache, &mut state).run_source("ShowMessage ReviseMessage", None, None);
    f.frame(&mut refs, &mut state, &home);
    assert_eq!(var(&state, ACTIVATOR_REF, "iRebuilt"), 0.0);
}

#[test]
fn another_scripts_ok_is_not_read_as_edit_name() {
    // The playtest's loop: another script's pop-up (an add-on's start
    // message) answered "OK", button 0, was taken by the revise activator
    // as "Edit Name", which asks again. In the game only the box's own
    // script gets it (`005b4a80`).
    let f = fixture("revise-other-ok");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.stand(&mut state, 100.0);
    let home = [HOME];
    f.frame(&mut refs, &mut state, &home);
    // Shown by another script (here a console line).
    Runner::new(&f.order, &f.cache, &mut state).run_source("ShowMessage ReviseMessage", None, None);
    assert_eq!(boxes(&mut state), 1);
    state.button = Some(0);
    let mut shown = 0;
    for _ in 0..5 {
        shown += f.frame(&mut refs, &mut state, &home);
    }
    assert_eq!(shown, 0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iRenamed"), 0.0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iAsked"), 0.0);
    // Still there for the console's own script.
    assert_eq!(state.button, Some(0));
}

#[test]
fn the_prompt_far_from_home_is_answered_there_and_asked_no_more() {
    let f = fixture("revise-border");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    // Past the border: only the border's square is attached; the
    // activator's own square (0,0) is out of the grid.
    f.stand(&mut state, 13000.0);
    let border = [BORDER];
    assert_eq!(f.frame(&mut refs, &mut state, &border), 1);
    assert_eq!(var(&state, QUEST, "done"), 1.0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iAsked"), 1.0);
    // "Rebuild Character": the activator, moved to the player, reads it
    // there and asks again (the game's script does, after an edit).
    state.button = Some(1);
    let mut shown = 0;
    for _ in 0..3 {
        shown += f.frame(&mut refs, &mut state, &border);
    }
    assert_eq!(var(&state, ACTIVATOR_REF, "iRebuilt"), 1.0);
    assert_eq!(shown, 1);
    assert_eq!(var(&state, ACTIVATOR_REF, "iAsked"), 2.0);
    // "Finished": done, and the quest stops.
    state.button = Some(2);
    f.frame(&mut refs, &mut state, &border);
    assert_eq!(var(&state, ACTIVATOR_REF, "iFinished"), 1.0);
    assert!(!state.running.contains(&FormId(QUEST)));
    // Back home nothing is asked again and nothing is left to read.
    f.stand(&mut state, 100.0);
    let mut shown = 0;
    for _ in 0..10 {
        shown += f.frame(&mut refs, &mut state, &[MAIL, HOME]);
    }
    assert_eq!(shown, 0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iAsked"), 2.0);
    assert_eq!(var(&state, ACTIVATOR_REF, "iRebuilt"), 1.0);
    assert_eq!(var(&state, MAILBOX_REF, "iTaken"), 0.0);
}

#[test]
fn a_moved_reference_leaves_its_old_cell_and_loads_in_the_new_one() {
    let f = fixture("revise-moved");
    let mut state = GameState::new(&f.order);
    let mut refs = RefScripts::default();
    f.stand(&mut state, 100.0);
    f.frame(&mut refs, &mut state, &[MAIL, HOME]);
    let listed = |refs: &RefScripts| {
        refs.refs()
            .iter()
            .any(|r| r.reference == FormId(ACTIVATOR_REF))
    };
    assert!(listed(&refs));
    // Moved into the mailbox's square: still attached, now with its cell.
    Runner::new(&f.order, &f.cache, &mut state).run_source(
        "ReviseActivatorRef.MoveTo MailboxRef",
        None,
        None,
    );
    f.frame(&mut refs, &mut state, &[MAIL, HOME]);
    assert!(listed(&refs));
    // Moved out of the attached cells: no longer run.
    f.stand(&mut state, 13000.0);
    Runner::new(&f.order, &f.cache, &mut state).run_source(
        "ReviseActivatorRef.MoveTo player",
        None,
        None,
    );
    f.frame(&mut refs, &mut state, &[MAIL, HOME]);
    assert!(!listed(&refs));
    // The border attached: it's there.
    f.frame(&mut refs, &mut state, &[BORDER]);
    assert!(listed(&refs));
}
