//! Placed objects' doors and locks, read and set through the game's own functions
//! (docs/coop/SPECS.md S12): whether a reference that opens is open, and its lock.
//!
//! These are the per-feature calls beside the changed-form stream ([`crate::changes`]). They
//! exist because the change journal cannot follow a door: a door opened gets its change flag
//! through `AddChange`, but a door shut again loses it through `RemoveChange`, which does not pass
//! the journal's wrap. Only objects both games know by the same id are shared (placed ones from
//! the plugins).

use engine_core::prelude::*;

use crate::{actors, addr, forms};

/// Whether the two games can mean the same placed object by `id`.
pub fn shareable(id: u32) -> bool {
    id != 0 && id < actors::FIRST_DYNAMIC_ID
}

/// The placed object (a `REFR`, not an actor) with id `id` in this game now.
pub fn reference(e: &mut Engine, id: u32) -> Option<u32> {
    let f = forms::form(e, id)?;
    (e.mem.u8(f.wrapping_add(addr::FORM_TYPE_OFFSET)) == addr::FORM_TYPE_REFR).then_some(f)
}

/// Whether reference `id` is open or opening (`BGSOpenCloseForm::GetOpenState`). `None` when it
/// is not in this game now or is not something that opens.
pub fn open(e: &mut Engine, id: u32) -> Option<bool> {
    let r = reference(e, id)?;
    match e.call(addr::GET_OPEN_STATE, &args![r]).u32() {
        1 | 2 => Some(true),
        3 | 4 => Some(false),
        _ => None,
    }
}

/// Opens or shuts reference `id` as the game does when it is activated
/// (`BGSOpenCloseForm::SetOpenState`, not immediate: the open or close sequence plays). False
/// when the reference is not in this game now or does not open.
pub fn set_open(e: &mut Engine, id: u32, to: bool) -> bool {
    if open(e, id).is_none() {
        return false;
    }
    let Some(r) = reference(e, id) else {
        return false;
    };
    e.call(addr::SET_OPEN_STATE, &args![r, u32::from(to), 0u32]);
    true
}

/// A reference's lock as the game keeps it (`REFR_LOCK`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lock {
    /// `cBaseLevel`: the lock's level.
    pub level: u8,
    /// `cFlags`: bit 0 locked.
    pub flags: u8,
    /// `uiNumTries`: failed forcings; the game calls a lock broken once they reach its limit.
    pub tries: u32,
}

impl Lock {
    /// Whether the lock is locked.
    pub fn locked(&self) -> bool {
        self.flags & addr::LOCK_FLAG_LOCKED != 0
    }
}

/// Reference `id`'s own lock, or `None` when it is not in this game now or has no lock.
pub fn lock(e: &mut Engine, id: u32) -> Option<Lock> {
    let r = reference(e, id)?;
    let l = match e.call(addr::REF_OWN_LOCK, &args![r]).u32() {
        0 => return None,
        l => l,
    };
    Some(Lock {
        level: e.mem.u8(l + addr::LOCK_LEVEL_OFFSET),
        flags: e.mem.u8(l + addr::LOCK_FLAGS_OFFSET),
        tries: e.mem.u32(l + addr::LOCK_TRIES_OFFSET),
    })
}

/// Makes reference `id`'s lock `to`: locked or unlocked through the game's own
/// `TESObjectREFR::Lock` and `UnLock` (which record the change), then the level, the other flags
/// and the tries written in place and the change recorded (`AddLockChange`). False when the
/// reference is not in this game now or has no lock (a lock is never made here: both games have
/// the same placed locks).
pub fn set_lock(e: &mut Engine, id: u32, to: Lock) -> bool {
    let Some(now) = lock(e, id) else {
        return false;
    };
    let Some(r) = reference(e, id) else {
        return false;
    };
    if now == to {
        return true;
    }
    if now.locked() != to.locked() {
        let f = if to.locked() {
            addr::REF_LOCK
        } else {
            addr::REF_UNLOCK
        };
        e.call(f, &args![r]);
    }
    if lock(e, id) != Some(to) {
        let l = e.call(addr::REF_OWN_LOCK, &args![r]).u32();
        if l == 0 {
            return false;
        }
        e.mem.set_u8(l + addr::LOCK_LEVEL_OFFSET, to.level);
        e.mem.set_u8(l + addr::LOCK_FLAGS_OFFSET, to.flags);
        e.mem.set_u32(l + addr::LOCK_TRIES_OFFSET, to.tries);
        e.call(addr::REF_ADD_LOCK_CHANGE, &args![r]);
    }
    true
}
