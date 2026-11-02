//! The play loop's patch crate.
//!
//! A stop in a play-build run is usually one function translated wrong or not at all. Fixing it
//! in its home file rebuilds that engine crate and everything after it (engine-fallout at
//! opt-level 3 takes many minutes). A fix written here instead rebuilds only this crate, the
//! host and the link: [`funcs`] is registered over the engine's own table at start-up
//! (`host::Host::start`, right after the exe is mapped), so an entry here replaces the one with
//! the same address.
//!
//! Rules: the same as any engine file (translated from the exe, address marked, tests in this
//! file). A patch reaches only calls made through `Engine::call`: a call made directly within
//! its home unit file still runs the old code, so patch that caller too when it matters. The
//! next gated collection moves every entry to its home file (replacing the old translation)
//! and empties [`funcs`]; main never carries patches.

#[allow(unused_imports)]
use engine::prelude::*;

/// The patched functions, by exe address. Empty between play loops (and always on main).
pub fn funcs() -> Vec<(u32, AbiFn)> {
    Vec::new()
}
