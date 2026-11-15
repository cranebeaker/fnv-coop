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
    if dead(e, id) != Some(false) {
        return false;
    }
    let Some(a) = actor(e, id) else {
        return false;
    };
    e.call(addr::ACTOR_KILL, &args![a, 0u32, 0.0f32.to_bits()]);
    true
}

/// Whether `kind` is a placed reference's form type (an object, an NPC or a creature).
pub fn is_ref_type(kind: u8) -> bool {
    kind == addr::FORM_TYPE_REFR || is_actor_type(kind)
}

/// The placed reference with id `id` in this game now, or `None`.
pub fn reference(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    is_ref_type(e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET))).then_some(f)
}

/// Whether reference `id` is disabled; `None` when it is not in this game now.
pub fn disabled(e: &mut Engine, id: u32) -> Option<bool> {
    let r = reference(e, id)?;
    Some(e.mem.u32(r.wrapping_add(addr::FORM_FLAGS_OFFSET)) & addr::FORM_FLAG_DISABLED != 0)
}

/// Enables or disables reference `id` through the game's own `Enable` and `Disable`. False when
/// it is not in this game now.
pub fn set_disabled(e: &mut Engine, id: u32, off: bool) -> bool {
    let Some(r) = reference(e, id) else {
        return false;
    };
    let f = if off {
        addr::REF_DISABLE
    } else {
        addr::REF_ENABLE
    };
    e.call(f, &args![r]);
    true
}

/// Whether reference `id` is a trigger volume: it has a primitive (a box or a sphere no one sees,
/// which a script's `OnTriggerEnter` block watches). False when it is not in this game now or
/// the program has no such function.
pub fn trigger_volume(e: &mut Engine, id: u32) -> bool {
    let Some(r) = reference(e, id) else {
        return false;
    };
    if !e.is_registered(addr::REF_EXTRA_LIST) || !e.is_registered(addr::EXTRA_GET_PRIMITIVE) {
        return false;
    }
    let list = e.call(addr::REF_EXTRA_LIST, &args![r]).u32();
    list != 0 && e.call(addr::EXTRA_GET_PRIMITIVE, &args![list]).u32() != 0
}

/// Whether `c` is a trigger volume switched on or off by its own object script: a trigger that
/// spends itself when a player walks into it (`Disable`, `MarkForDelete` in its `OnTriggerEnter`
/// block). That is each player's own: the host's player walking through one must not spend the
/// guest's, whose script gives that player what it gives (PC run 13, 2026-11-11: the trigger
/// outside Doc Mitchell's house gives the player their weapons back; the host's spent the
/// guest's, and the guest could draw none).
pub fn own_trigger_change(e: &mut Engine, c: &crate::Change) -> bool {
    is_ref_type(c.kind)
        && c.flags & addr::CHANGE_FORM_FLAGS != 0
        && c.cause
            .is_some_and(|k| k.reference == c.form && !k.quest_script && k.owner_quest == 0)
        && trigger_volume(e, c.form)
}
