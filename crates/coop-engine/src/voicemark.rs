//! voicemark: the speaking players' compass ticks (docs/coop/SPECS.md, voice list).
//!
//! The compass wrap (`compass`, C-14) ticks each other player's avatar in purple. While that
//! player's voice frames arrive (the session's `voicelist::Speaking`), the tick is drawn in
//! [`SPEAKING`], a light magenta, and at full alpha, so the speaking player is picked out where the
//! other players' marks are already drawn. No other drawing is touched: the colour goes through the
//! same tile traits the compass wrap sets (`red`, `green`, `blue`, the system colour off).

use std::sync::{PoisonError, RwLock};

use engine_core::prelude::*;

use crate::compass::PURPLE;

/// The tick colour of a player who speaks (red, green, blue, 0..255): lighter than [`PURPLE`] and
/// as far from the HUD's own colours (amber, white, blue, green, the warning red).
pub const SPEAKING: [u8; 3] = [255, 140, 255];

/// The speaking avatars (form ids), a lock-free shared slot as the compass's own.
#[derive(Debug, Default)]
pub struct Speaking {
    ids: RwLock<Vec<u32>>,
}

/// Makes the slot (before the game's threads start; [`crate::install`]).
pub(crate) fn install(e: &mut Engine) {
    let _ = e.ext_free::<Speaking>();
}

/// The avatars (form ids) whose players speak now; every other avatar is ticked in purple.
pub fn set_speaking(e: &mut Engine, ids: impl IntoIterator<Item = u32>) {
    let ids: Vec<u32> = ids.into_iter().collect();
    let slot = e.ext_free::<Speaking>();
    let mut held = slot.ids.write().unwrap_or_else(PoisonError::into_inner);
    if *held != ids {
        *held = ids;
    }
}

/// Whether the avatar `id` is marked as speaking.
pub fn speaking(e: &mut Engine, id: u32) -> bool {
    e.ext_free::<Speaking>()
        .ids
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .contains(&id)
}

/// The colour of avatar `id`'s tick.
pub fn tick_colour(e: &mut Engine, id: u32) -> [u8; 3] {
    if speaking(e, id) {
        SPEAKING
    } else {
        PURPLE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_speaking_avatar_is_ticked_in_the_speaking_colour() {
        let mut e = Engine::new();
        install(&mut e);
        assert_eq!(tick_colour(&mut e, 7), PURPLE);
        set_speaking(&mut e, [7]);
        assert_eq!(tick_colour(&mut e, 7), SPEAKING);
        assert_eq!(tick_colour(&mut e, 8), PURPLE);
        set_speaking(&mut e, []);
        assert_eq!(tick_colour(&mut e, 7), PURPLE);
        assert_ne!(SPEAKING, PURPLE);
    }
}
