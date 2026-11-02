//! A switch for behaviour that isn't traced yet ([G] in the code and the
//! topic files), off by default so the base-game routes run only what was
//! read from the game; the viewer turns it on with `NV_GUESSES=1`. What
//! it gates now: a teammate with nothing to do following the player
//! (`ai::current_package`), `IsAnimPlaying` on a person, the dying
//! body taking the actor's velocity (viewer `ai`), billboards turning
//! about their up axis (`cellview::impacts`) and the falloff-flag reading
//! of impact shaders (`preview::cell`). Teammates coming along to another
//! place is traced (`companions::come_along`, `00973de0`) and always
//! on. See `docs/CONTRIB_PLAYCON.md`.

use std::sync::atomic::{AtomicBool, Ordering};

static ON: AtomicBool = AtomicBool::new(false);

/// Whether untraced behaviour runs.
pub fn enabled() -> bool {
    ON.load(Ordering::Relaxed)
}

/// Turns untraced behaviour on or off (process-wide).
pub fn set(on: bool) {
    ON.store(on, Ordering::Relaxed);
}
