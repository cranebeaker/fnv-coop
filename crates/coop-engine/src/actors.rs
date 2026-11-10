//! Placed actors' life and death (docs/coop/SPECS.md S11): whether a placed NPC or creature is
//! dead, and the game's own kill.
//!
//! Only actors both games know by the same id are shared: placed ones from the plugins. An actor
//! made while playing (ids from `ff000000`) has another id, or none, in the other game.

use engine_core::prelude::*;

use crate::{addr, forms};

/// Ids from here up are made while playing and differ between two games.
pub const FIRST_DYNAMIC_ID: u32 = 0xff00_0000;
/// The player's reference.
pub const PLAYER_REF_ID: u32 = 0x14;

/// Whether `kind` is a placed actor's form type.
pub fn is_actor_type(kind: u8) -> bool {
    kind == addr::FORM_TYPE_ACHR || kind == addr::FORM_TYPE_ACRE
}

/// Whether the two games can mean the same actor by `id`: a placed actor, not the player.
pub fn shareable(id: u32) -> bool {
    id != 0 && id != PLAYER_REF_ID && id < FIRST_DYNAMIC_ID
}

/// The placed actor with id `id` in this game now, or `None` (no such form here, or not an actor).
pub fn actor(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    is_actor_type(e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET))).then_some(f)
}

/// Whether actor `id` is dying or dead; `None` when it is not in this game now.
pub fn dead(e: &mut Engine, id: u32) -> Option<bool> {
    let a = actor(e, id)?;
    Some(matches!(
        e.mem.u32(a.wrapping_add(addr::ACTOR_LIFE_STATE_OFFSET)),
        1 | 2
    ))
}

/// Kills actor `id` through the game's `Actor::Kill`, with no attacker (nobody here is credited).
/// False when the actor is not in this game now. The game may keep it alive (essential) or do the
/// kill later in the frame: read [`dead`] afterwards.
pub fn kill(e: &mut Engine, id: u32) -> bool {
    let Some(a) = actor(e, id) else {
        return false;
    };
    e.call(addr::ACTOR_KILL, &args![a, 0u32, 0.0f32.to_bits()]);
    true
}
