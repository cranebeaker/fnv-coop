//! `BSHavok/bhkentitylistener.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The listener keeps a mask word at `+4` and a pointer list (`NiTPointerList`, handled by
//! `00631540` add / `00631620` remove) at `+0xC`.
//!
//! The exception-unwinding frames (`FS:[0]` registrations) of the functions marked
//! `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, release_ni_object, INTERLOCKED_INCREMENT};

/// Dynamic cast of `00653290` (`ECX` the object, one stack argument: the `NiRTTI` global).
const IS_KIND_OF: u32 = 0x0065_3290;
/// The `NiRTTI` global `GetRB` casts to.
const RIGID_BODY_RTTI: u32 = 0x0126_81c0;
/// `NiTPointerList` add (`00631540`) and remove (`00631620`) on the list at `this + 0xc`;
/// the argument is the address of a cell holding the pointer.
const LIST_ADD: u32 = 0x0063_1540;
const LIST_REMOVE: u32 = 0x0063_1620;
/// `bhkEntityListener::RemoveEntity` and `AddEntity`.
const REMOVE_ENTITY: u32 = 0x00c9_a520;
const ADD_ENTITY: u32 = 0x00c9_a460;

// Translated from 00c9a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntityListener::GetRB` (Xbox PDB): for a Havok entity whose word at `+0xC` is set,
/// casts that word with `00653290` to the `NiRTTI` at `012681c0` and answers the result
/// when its flags word (`+0x10`) shares a bit with the listener's mask (`this + 4`);
/// otherwise 0.
pub fn bhk_entity_listener_get_rb(e: &mut Engine, this: Ptr, entity: u32) -> u32 {
    if entity != 0 {
        let owner = e.mem.u32(entity + 0xc);
        if owner != 0 {
            let cast = e.call(IS_KIND_OF, &args![owner, RIGID_BODY_RTTI]).u32();
            if cast != 0 {
                let flags = e.mem.u32(cast + 0x10);
                if e.mem.u32(this.addr() + 4) & flags != 0 {
                    return cast;
                }
            }
        }
    }
    0
}

// Translated from 00c9a460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9a460 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkEntityListener::AddEntity` (Xbox PDB): with the allocation-scope word at 0x10, takes
/// a reference on `entity` (when set), adds it to the pointer list at `this + 0xc` through a
/// cell holding it, and drops the reference again (deleting the entity when that was the
/// last one).
pub fn bhk_entity_listener_add_entity(e: &mut Engine, this: Ptr, entity: u32) {
    let (word, saved) = enter_scope(e);
    if entity != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![entity + 4]);
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), entity);
            e.call(LIST_ADD, &args![this.addr() + 0xc, cell]);
        });
        release_ni_object(e, entity);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c9a520 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c9a520 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkEntityListener::RemoveEntity` (Xbox PDB): like `AddEntity`, but only for an entity
/// whose count (`+4`) is above 0, and removing it from the list (`00631620`).
pub fn bhk_entity_listener_remove_entity(e: &mut Engine, this: Ptr, entity: u32) {
    let (word, saved) = enter_scope(e);
    if entity != 0 && e.mem.u32(entity + 4) > 0 {
        e.call(INTERLOCKED_INCREMENT, &args![entity + 4]);
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), entity);
            e.call(LIST_REMOVE, &args![this.addr() + 0xc, cell]);
        });
        release_ni_object(e, entity);
    }
    e.mem.set_u32(word, saved);
}

// Translated from 00c9a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntityListener::entityRemovedCallback` (Xbox PDB): asks virtual slot `0x24` for the
/// entity of `event`, then `RemoveEntity` (`00c9a520`) on it.
pub fn bhk_entity_listener_entity_removed_callback(e: &mut Engine, this: Ptr, event: u32) {
    let entity = e.vcall(this.addr(), 0x24, &args![event]).u32();
    e.call(REMOVE_ENTITY, &args![this, entity]);
}

// Translated from 00c9a600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkEntityListener::entityAddedCallback` (Xbox PDB): asks virtual slot `0x24` for the
/// entity of `event`, then `AddEntity` (`00c9a460`) on it.
pub fn bhk_entity_listener_entity_added_callback(e: &mut Engine, this: Ptr, event: u32) {
    let entity = e.vcall(this.addr(), 0x24, &args![event]).u32();
    e.call(ADD_ENTITY, &args![this, entity]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_a430, bhk_entity_listener_get_rb(Ptr, u32) -> u32),
        entry!(0x00c9_a460, bhk_entity_listener_add_entity(Ptr, u32)),
        entry!(0x00c9_a520, bhk_entity_listener_remove_entity(Ptr, u32)),
        entry!(
            0x00c9_a5e0,
            bhk_entity_listener_entity_removed_callback(Ptr, u32)
        ),
        entry!(
            0x00c9_a600,
            bhk_entity_listener_entity_added_callback(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::SCOPE_WORD;
    use super::*;

    #[test]
    fn get_rb_needs_a_cast_object_sharing_a_mask_bit() {
        let mut e = logging_engine();
        e.register(IS_KIND_OF, |e, a| Ret {
            eax: if a[0] == 0x6000 {
                e.mem.u32(0x0900_1000)
            } else {
                0
            },
            ..Ret::default()
        });
        let body = e.mem.alloc(0x20);
        e.map(0x0900_1000, 4);
        e.mem.set_u32(0x0900_1000, body);
        e.mem.set_u32(body + 0x10, 0b0110);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 4, 0b0100);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 0xc, 0x6000);
        assert_eq!(e.call(0x00c9_a430, &args![this, entity]).u32(), body);
        assert_eq!(
            calls_to(&e, IS_KIND_OF),
            vec![vec![0x6000, RIGID_BODY_RTTI]]
        );
        e.mem.set_u32(this + 4, 0b1001);
        assert_eq!(e.call(0x00c9_a430, &args![this, entity]).u32(), 0);
        e.mem.set_u32(this + 4, 0b0100);
        e.mem.set_u32(entity + 0xc, 0x6001);
        assert_eq!(e.call(0x00c9_a430, &args![this, entity]).u32(), 0);
        e.mem.set_u32(entity + 0xc, 0);
        assert_eq!(e.call(0x00c9_a430, &args![this, entity]).u32(), 0);
        assert_eq!(e.call(0x00c9_a430, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls_to(&e, IS_KIND_OF).len(), 3);
    }

    #[test]
    fn add_entity_holds_a_reference_around_the_list_add() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(LIST_ADD, |e, a| {
            // The cell holds the entity, and the reference has been taken.
            let entity = e.mem.u32(a[1]);
            assert_eq!(e.mem.u32(entity + 4), 2);
            Ret::default()
        });
        e.register(0x0900_0004, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let entity = ni_object(&mut e, 1, 0x0900_0004);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 3);
        e.call(0x00c9_a460, &args![this, entity]);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 3);
        assert_eq!(e.mem.u32(entity + 4), 1);
        let adds = calls_to(&e, LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], this + 0xc);
        assert!(calls_to(&e, 0x0900_0004).is_empty());
        // A null entity only sets and restores the scope word.
        e.call(0x00c9_a460, &args![this, 0u32]);
        assert_eq!(calls_to(&e, LIST_ADD).len(), 1);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 3);
    }

    #[test]
    fn add_entity_deletes_an_entity_whose_last_reference_was_the_temporary() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(0x0900_0004, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let entity = ni_object(&mut e, 0, 0x0900_0004);
        e.call(0x00c9_a460, &args![this, entity]);
        assert_eq!(calls_to(&e, 0x0900_0004), vec![vec![entity]]);
    }

    #[test]
    fn remove_entity_skips_an_entity_without_references() {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(0x0900_0004, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let dead = ni_object(&mut e, 0, 0x0900_0004);
        e.call(0x00c9_a520, &args![this, dead]);
        e.call(0x00c9_a520, &args![this, 0u32]);
        assert!(calls_to(&e, LIST_REMOVE).is_empty());
        let live = ni_object(&mut e, 3, 0x0900_0004);
        e.call(0x00c9_a520, &args![this, live]);
        let removes = calls_to(&e, LIST_REMOVE);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][0], this + 0xc);
        assert_eq!(e.mem.u32(live + 4), 3);
    }

    #[test]
    fn callbacks_resolve_the_entity_through_slot_0x24() {
        let mut e = logging_engine();
        e.register(0x0900_0024, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(ADD_ENTITY, |_, _| Ret::default());
        e.register(REMOVE_ENTITY, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(vtable + 0x24, 0x0900_0024);
        e.call(0x00c9_a5e0, &args![this, 0x10u32]);
        e.call(0x00c9_a600, &args![this, 0x20u32]);
        assert_eq!(calls_to(&e, REMOVE_ENTITY), vec![vec![this, 0x11]]);
        assert_eq!(calls_to(&e, ADD_ENTITY), vec![vec![this, 0x21]]);
        assert_eq!(
            calls_to(&e, 0x0900_0024),
            vec![vec![this, 0x10], vec![this, 0x20]]
        );
    }
}
