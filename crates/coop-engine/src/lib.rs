//! Co-op's bridge to the translated engine (docs/coop/PLAN.md, layer 2; docs/coop/SPECS.md S2).
//!
//! The only crate of the co-op layer that touches the engine. It takes part in the game as the
//! fourth extension owner of ADR-0016 ([`engine_core::extend::Owner::Coop`], after the fix rows,
//! `nvse` and `jip`) and calls the game's functions by address, as `nvse` does. With co-op not
//! installed nothing here runs and the game is unchanged.
//!
//! - [`journal`]: every form the game marks as changed for the save game, recorded while armed.
//! - [`forms`]: forms by id, quest stages through the game's own `SetStageDone`, global values,
//!   and the comparison that replicates globals.
//! - [`gate`]: on a guest, the story's scripts do not run (the host runs them).
//! - [`quest`]: a quest's story state as the host sends it and a guest applies it.
//! - [`vars`]: a quest script's variables, read on the host and written on a guest.
//! - [`live`]: the local player's place, a guest's warp to the host, a remote player's avatar.
//! - [`changes`]: one form's change as the game's own save record, taken on the host and loaded
//!   on a guest; [`world`]: doors and locks through the game's own functions.
//! - [`fight`]: actors as the host's game has them, held on a guest; hits, threat and the
//!   player's fall (shared fights).
//! - [`addr`]: every exe address and offset used, in one place.
//! - [`diag`]: `NV_COOP=journal`, the change journal printed on a PC run.

pub mod actors;
pub mod addr;
pub mod diag;
// fights
pub mod fight;
pub mod forms;
pub mod gate;
pub mod journal;
pub mod live;
pub mod quest;
#[cfg(any(test, feature = "testkit"))]
pub mod testkit;
pub mod vars;
// world
pub mod changes;
pub mod world;
#[cfg(test)]
mod world_tests;
#[cfg(any(test, feature = "testkit"))]
pub mod worldkit;
// world: end

// social: conversations, party saves and the rest vote (docs/coop/SPECS.md S12 to S15).
pub mod rest;
pub mod saves;
pub mod social;
#[cfg(any(test, feature = "testkit"))]
pub mod social_kit;
pub mod talk;

// vats: V.A.T.S. with several players, and damage outside `Actor::HitMe` (docs/coop/SPECS.md
// S20, S21).
pub mod harm;
pub mod vats;

use std::sync::atomic::Ordering;

use engine_core::prelude::*;

pub use journal::{arm, armed, drain, Change, Journal};

/// Whether co-op is asked for in this run: `NV_COOP` set and not `0`.
pub fn wanted() -> bool {
    std::env::var_os("NV_COOP").is_some_and(|v| v != "0")
}

/// Installs co-op's wraps on `e` (once; a second call does nothing). Must run before the game's
/// real threads start, as every wrap must (`Engine::wrap` checks it): `Host::start_with` calls it
/// after `nvse::install`. The addresses it wraps (`BGSSaveLoadGame::AddChange`,
/// `ScriptRunner::Run`) must already be registered (the translations, or a test's stand-ins).
pub fn install(e: &mut Engine) {
    // The journal's slot is made here, before the threads start, as a lock-free shared slot
    // must be: real threads' engines share it from then on.
    if e.ext_free::<Journal>()
        .installed
        .swap(true, Ordering::SeqCst)
    {
        return;
    }
    e.wrap(journal::ADD_CHANGE, journal::add_change);
    e.wrap(gate::RUN, gate::run);
    // The gate's slot too, before the threads start.
    let _ = e.ext_free::<gate::Gate>();
    // fights: the hit wrap and its slot.
    fight::install(e);
    // social
    social::install(e);
    // vats: the menu-mode, time multiplier and value-change wraps.
    vats::install(e);
    harm::install(e);
}

// The unit tests' own program: no other crate's functions are linked in.
#[cfg(test)]
engine_core::assemble!();

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::extend::Owner;

    #[test]
    fn install_wraps_once_as_coop() {
        let mut e = Engine::new();
        e.register(addr::SAVE_LOAD_ADD_CHANGE, |_, _| Ret::default());
        e.register(addr::SCRIPT_RUNNER_RUN, |_, _| Ret::default());
        install(&mut e);
        install(&mut e);
        assert_eq!(
            e.wrapped(),
            [
                (addr::SCRIPT_RUNNER_RUN, vec![Owner::Coop]),
                (addr::SAVE_LOAD_ADD_CHANGE, vec![Owner::Coop]),
            ]
        );
    }
}
