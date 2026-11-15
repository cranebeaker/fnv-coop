//! The game, as the program's loop sees it.

use coop_engine::gate::{Counters, Intent};
use coop_engine::Program;

/// What the game shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    /// The player plays.
    Play,
    /// A menu is open.
    Menu,
    /// A loading screen.
    Loading,
}

/// One running game: the program by address, its frames, and the guest gate.
///
/// Each call but [`Machine::frame`] happens between two frames, on the thread that runs
/// them.
pub trait Machine: Program {
    /// Runs one frame of the game's own code. False when the game's main does not run any
    /// more.
    fn frame(&mut self) -> bool;

    /// What the game shows now.
    fn screen(&self) -> Screen;

    /// Starts (`true`) or ends the adapter's own bracket. What the program runs inside the
    /// bracket is the session's work, and the guest gate lets it pass.
    fn own(&mut self, on: bool);

    /// Tells the gate one frame of play of `play_ms` milliseconds. True when the gate saw no
    /// script for too long: it is not in the script path any more.
    fn gate_frame(&mut self, play_ms: u32) -> bool;

    /// The changes of shared quests that this guest's game wanted to make since the last
    /// call. Empty on a game with no gate.
    fn take_intents(&mut self) -> Vec<Intent>;

    /// What the gate counted. `None` on a game with no gate.
    fn counters(&self) -> Option<Counters>;
}
