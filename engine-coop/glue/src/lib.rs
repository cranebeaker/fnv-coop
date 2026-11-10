//! The glue between the co-op adapter and the upstream engine.
//!
//! This is the one crate of the co-op layer that names upstream types. The adapter
//! (`coop-engine`) sees the game program through [`coop_engine::Program`]. This crate
//! implements that trait on the upstream engine, and starts a game for a test run.
//!
//! When upstream changes its interface, this crate is the place that changes.

use std::path::Path;

use coop_engine::{Program, Ret};

pub mod avatar_guard;
pub mod gate;
use engine::Engine;

/// A running game: the upstream host.
pub use host::Host;

/// The upstream engine as the adapter's [`Program`]. Use it between the game's frames.
pub struct EngineProgram<'a>(pub &'a mut Engine);

impl Program for EngineProgram<'_> {
    fn read(&self, addr: u32, out: &mut [u8]) {
        self.0.mem.read(addr, out);
    }

    fn write(&mut self, addr: u32, bytes: &[u8]) {
        self.0.mem.write(addr, bytes);
    }

    fn call(&mut self, addr: u32, args: &[u32]) -> Ret {
        let ret = self.0.call(addr, args);
        Ret {
            eax: ret.eax,
            st0: ret.st0,
        }
    }

    fn alloc(&mut self, len: u32) -> u32 {
        self.0.mem.alloc(len)
    }

    fn free(&mut self, addr: u32) {
        self.0.mem.free(addr);
    }

    fn install_gate(
        &mut self,
        rules: &coop_engine::gate::Rules,
        shared_quest: Option<u32>,
    ) -> Result<(), String> {
        gate::install(self.0, rules.clone())?;
        gate::self_check(self.0, shared_quest)
    }
}

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

/// What the game shows now.
pub fn screen(game: &host::Host) -> Screen {
    match host::uncap::screen(&game.engine) {
        host::uncap::Screen::Play => Screen::Play,
        host::uncap::Screen::Menu => Screen::Menu,
        host::uncap::Screen::Loading => Screen::Loading,
    }
}

/// How many wraps the engine has at `addr`. An override by address removes them, so the
/// adapter overrides only an address with none.
pub fn wraps_at(e: &Engine, addr: u32) -> usize {
    e.wraps_at(addr).len()
}

/// Starts the game on the installation whose Data folder is `data`. `test`: a test run
/// (stepped threads, the player's Fallout.ini options not read). The game's end goes to
/// `at_end` with its status.
pub fn start(
    data: &Path,
    test: bool,
    at_end: impl FnOnce(i32) + 'static,
) -> Result<host::Host, String> {
    let choice = host::fixes::Choice::read_with(test)?;
    let mut game = host::Host::start_with(data, &choice)?;
    game.at_main_end = Some(Box::new(
        move |_: &host::Host, end: host::player_move::MainEnd| at_end(end.status()),
    ));
    game.end_if_main_ended();
    Ok(game)
}

/// One frame of the game with no input. False when the game's main does not run.
pub fn frame(game: &mut host::Host) -> bool {
    game.set_keys_down(&[]);
    game.mouse([0; 3], [false; 8]);
    game.main_frame()
}

/// How many frames the main menu takes to build, from its first frame.
const MENU_FRAMES: u32 = 120;

/// Runs the game to its main menu, built. The count of frames. `Err` when the menu is not
/// reached in `limit` frames.
pub fn reach_menu(game: &mut host::Host, limit: u32) -> Result<u32, String> {
    let mut menu_at = None;
    for frames in 1..=limit {
        if !frame(game) {
            return Err("the game's main does not run".into());
        }
        if menu_at.is_none() && screen(game) == Screen::Menu {
            menu_at = Some(frames);
        }
        if menu_at.is_some_and(|at| frames >= at + MENU_FRAMES) {
            return Ok(frames);
        }
    }
    Err(format!("the main menu was not reached in {limit} frames"))
}

/// Goes from the built main menu into `cell` (a console line `coc`, the adapter's own), and
/// runs `settle` frames of play more. The count of frames. `Err` when play is not reached
/// in `limit` frames.
pub fn enter_cell(
    game: &mut host::Host,
    cell: &str,
    settle: u32,
    limit: u32,
) -> Result<u32, String> {
    let line = format!("coc {cell}");
    let mut run = || coop_engine::console::run_line(&mut EngineProgram(&mut game.engine), &line, 0);
    if !gate::own(&mut run) {
        return Err("the game has no console manager".into());
    }
    let mut play_at = None;
    for frames in 1..=limit {
        if !frame(game) {
            return Err("the game's main does not run".into());
        }
        if play_at.is_none() && screen(game) == Screen::Play {
            play_at = Some(frames);
        }
        if play_at.is_some_and(|at| frames >= at + settle) {
            return Ok(frames);
        }
    }
    Err(format!("play in {cell} was not reached in {limit} frames"))
}

/// Runs the game from its start into `cell`, and `settle` frames more. `Err` when play is
/// not reached in `limit` frames.
pub fn reach_play(
    game: &mut host::Host,
    cell: &str,
    settle: u32,
    limit: u32,
) -> Result<u32, String> {
    let menu = reach_menu(game, limit)?;
    Ok(menu + enter_cell(game, cell, settle, limit.saturating_sub(menu))?)
}
