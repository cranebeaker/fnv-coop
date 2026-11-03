//! Hardcore needs per player (dehydration, hunger, sleep deprivation).
//!
//! The needs are actor values of the player (`world::living::needs`), so
//! each player's live in their shadow, and the needs' clocks live in the
//! swapped `Living` ([`super::extra`]). The host's own needs grow in its
//! scripts' frame and in each hour of a wait or sleep
//! (`world::living::sleep::pass_hour`). This module grows every other
//! player's needs the same way, inside that player's shadow, so that each
//! player's hunger, thirst and sleep are their own.

use esm::LoadOrder;

use crate::dialogue::PLAYER_REF;
use crate::living::needs::{self, SLEEP_DEPRIVATION};
use crate::scripting::GameState;

/// How often the host grows the other players' needs (the growth reads the
/// game clock, so a later call catches up).
const EVERY_MS: u64 = 1_000;

/// The slots whose rows are parked (every player but the one live now).
fn others(gs: &GameState) -> Vec<u8> {
    gs.coop
        .bank
        .parked
        .keys()
        .copied()
        .filter(|s| *s != 255)
        .collect()
}

/// Host, from the campaign frame: in hardcore, every other player's needs
/// grow on the game clock (`needs::grow` in their shadow).
pub fn frame(order: &LoadOrder, gs: &mut GameState, now_ms: u64) {
    if !gs.living.hardcore || now_ms < gs.coop.campaign.needs_ms {
        return;
    }
    gs.coop.campaign.needs_ms = now_ms + EVERY_MS;
    for s in others(gs) {
        gs.with_player(s, |g| needs::grow(order, g));
    }
}

/// Host, after the host's own hour of an approved wait or sleep: every
/// other player's needs grow for the hour, and a sleep lowers their sleep
/// deprivation by `fHCSleepRestorationMod` (as `pass_hour` does for the
/// host).
pub fn hour(order: &LoadOrder, gs: &mut GameState, sleeping: bool) {
    if !gs.living.hardcore {
        return;
    }
    for s in others(gs) {
        gs.with_player(s, |g| {
            let was = std::mem::replace(&mut g.living.sleeping, sleeping);
            needs::grow(order, g);
            if sleeping {
                let sd = needs::value(order, g, PLAYER_REF, SLEEP_DEPRIVATION);
                let by = f64::from(crate::living::setting(
                    order,
                    "fHCSleepRestorationMod",
                    60.0,
                ))
                .min(sd);
                if by > 0.0 {
                    crate::magic::change(order, g, PLAYER_REF, SLEEP_DEPRIVATION, by, PLAYER_REF);
                }
            }
            g.living.sleeping = was;
        });
    }
}
