//! `Havok SDK/hkpbinaryaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBinaryAction` (the Xbox PDB has no record of the Havok classes, so the field names
//! below are unconfirmed): an action on two entities. `+0x08` is the world the action is in
//! (0 when in none), `+0x10` user data, `+0x14` a string pointer (`hkStringPtr`), `+0x18`
//! and `+0x1C` the two entities (each held with a reference).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable of the action base (`010cc2fc`), the one of `hkpBinaryAction` (`010cc320`)
/// and the base class's (`0102e388`).
const ACTION_VTABLE: u32 = 0x010c_c2fc;
const BINARY_ACTION_VTABLE: u32 = 0x010c_c320;
const BASE_VTABLE: u32 = 0x0102_e388;
/// Takes a reference on `this` (`00c90510`) and drops one (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpWorld::removeActionImmediately` (`00c97510`, `this` the world, one word: the action).
const REMOVE_ACTION_IMMEDIATELY: u32 = 0x00c9_7510;
/// World calls on an action and one of its entities (two words: the action and the entity):
/// detaching it (`00c976a0`, `stdcall`, the world the caller puts in ECX is not read) and
/// attaching it (`00c92bd0`, `this` the world).
const DETACH_ENTITY: u32 = 0x00c9_76a0;
const ATTACH_ENTITY: u32 = 0x00c9_2bd0;
/// `hkStringPtr` constructor (`00cb0a60`) and destructor (`00cb0c00`).
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The array growth helper (`00c90940`, cdecl: allocator, array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;

const WORLD: u32 = 0x08;
const USER_DATA: u32 = 0x10;
const NAME: u32 = 0x14;
const ENTITY_A: u32 = 0x18;
const ENTITY_B: u32 = 0x1c;

// Translated from 00cf4ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a reference (`00c90510`) on each non-null entity of the action, entity A first
/// (no engine-map name; `__fastcall`, `this` only; the second call is a tail call).
pub fn fn_00cf4ca0(e: &mut Engine, this: Ptr) {
    let action = this.addr();
    let entity_a = e.mem.u32(action + ENTITY_A);
    if entity_a != 0 {
        e.call(ADD_REFERENCE, &args![entity_a]);
    }
    let entity_b = e.mem.u32(action + ENTITY_B);
    if entity_b != 0 {
        e.call(ADD_REFERENCE, &args![entity_b]);
    }
}

// Translated from 00cf4cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::entityRemovedCallback` (engine map): when the action is in a world
/// (`+8`), removes it from that world at once (`hkpWorld::removeActionImmediately`,
/// `00c97510`); the stack argument (the entity) is never read.
pub fn hkp_binary_action_entity_removed_callback(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let action = this.addr();
    let world = e.mem.u32(action + WORLD);
    if world != 0 {
        e.call(REMOVE_ACTION_IMMEDIATELY, &args![world, action]);
    }
}

// Translated from 00cf4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::setEntityB` (engine map): takes a reference on `entity`; when entity B
/// is set, detaches it from the world (when the action is in one; `00c976a0`), drops its
/// reference and clears it; then stores `entity` and, when the action is in a world,
/// attaches it (`00c92bd0`).
pub fn hkp_binary_action_set_entity_b(e: &mut Engine, this: Ptr, entity: u32) {
    let action = this.addr();
    e.call(ADD_REFERENCE, &args![entity]);
    let old = e.mem.u32(action + ENTITY_B);
    if old != 0 {
        let world = e.mem.u32(action + WORLD);
        if world != 0 {
            // 00cf4d4f: `00c976a0` is stdcall (RET 8) and does not read ECX.
            e.call(DETACH_ENTITY, &args![action, old]);
        }
        let old = e.mem.u32(action + ENTITY_B);
        e.call(REMOVE_REFERENCE, &args![old]);
        e.mem.set_u32(action + ENTITY_B, 0);
    }
    let world = e.mem.u32(action + WORLD);
    e.mem.set_u32(action + ENTITY_B, entity);
    if world != 0 {
        e.call(ATTACH_ENTITY, &args![world, action, entity]);
    }
}

// Translated from 00cf4ce0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00cf4ce0 to the `RET 4` at 00cf4d26, inside the body Ghidra
/// gives `SetRigidBodyA`); `hkpBinaryAction::setEntityA` by its place before `setEntityB` and
/// its shape (unconfirmed name). `thiscall`, one stack word: as [`hkp_binary_action_set_entity_b`]
/// for entity A (`+0x18`): takes a reference on `entity`; when entity A is set, detaches it from
/// the world (when the action is in one; `00c976a0`), drops its reference and clears it; then
/// stores `entity` and, when the action is in a world, attaches it (`00c92bd0`).
pub fn hkp_binary_action_set_entity_a(e: &mut Engine, this: Ptr, entity: u32) {
    let action = this.addr();
    e.call(ADD_REFERENCE, &args![entity]);
    let old = e.mem.u32(action + ENTITY_A);
    if old != 0 {
        let world = e.mem.u32(action + WORLD);
        if world != 0 {
            // 00cf4cff: `00c976a0` is stdcall (RET 8) and does not read ECX.
            e.call(DETACH_ENTITY, &args![action, old]);
        }
        let old = e.mem.u32(action + ENTITY_A);
        e.call(REMOVE_REFERENCE, &args![old]);
        e.mem.set_u32(action + ENTITY_A, 0);
    }
    let world = e.mem.u32(action + WORLD);
    e.mem.set_u32(action + ENTITY_A, entity);
    if world != 0 {
        e.call(ATTACH_ENTITY, &args![world, action, entity]);
    }
}

// Translated from 00cf4d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::hkpBinaryAction` (engine map): user data `user_data` at `+0x10`,
/// reference count 1 (`+6`), world and `+0x0C` cleared, the string pointer at `+0x14`
/// constructed, the vtable, the entities `entity_a` and `entity_b`; when either is non-null
/// the references are taken (`fn_00cf4ca0`). Returns `this`.
pub fn hkp_binary_action_construct(
    e: &mut Engine,
    this: Ptr,
    entity_a: u32,
    entity_b: u32,
    user_data: u32,
) -> Ptr {
    let action = this.addr();
    e.mem.set_u32(action + USER_DATA, user_data);
    e.mem.set_u16(action + 6, 1);
    e.mem.set_u32(action, ACTION_VTABLE);
    e.mem.set_u32(action + WORLD, 0);
    e.mem.set_u32(action + 0xc, 0);
    e.call(STRING_PTR_CONSTRUCT, &args![action + NAME]);
    e.mem.set_u32(action, BINARY_ACTION_VTABLE);
    e.mem.set_u32(action + ENTITY_A, entity_a);
    e.mem.set_u32(action + ENTITY_B, entity_b);
    if entity_b != 0 || entity_a != 0 {
        fn_00cf4ca0(e, this);
    }
    this
}

// Translated from 00cf4de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::~hkpBinaryAction` (engine map): drops the reference on each non-null
/// entity (A then B) and clears it, destroys the string pointer at `+0x14` and leaves the
/// base class's vtable (the vtable passes through `010cc320` and `010cc2fc` on the way).
pub fn hkp_binary_action_destruct(e: &mut Engine, this: Ptr) {
    let action = this.addr();
    e.mem.set_u32(action, BINARY_ACTION_VTABLE);
    let entity_a = e.mem.u32(action + ENTITY_A);
    if entity_a != 0 {
        e.call(REMOVE_REFERENCE, &args![entity_a]);
        e.mem.set_u32(action + ENTITY_A, 0);
    }
    let entity_b = e.mem.u32(action + ENTITY_B);
    if entity_b != 0 {
        e.call(REMOVE_REFERENCE, &args![entity_b]);
        e.mem.set_u32(action + ENTITY_B, 0);
    }
    e.mem.set_u32(action, ACTION_VTABLE);
    e.call(STRING_PTR_DESTRUCT, &args![action + NAME]);
    e.mem.set_u32(action, BASE_VTABLE);
}

/// Appends `word` to the array at `array` (data, size, capacity word): the memory router
/// is asked first, and when the size equals the capacity (`& 0x3fffffff`) the array grows
/// through `00c90940` (allocator, array, element size 4).
fn array_push(e: &mut Engine, array: u32, word: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), word);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00cf4e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBinaryAction::getEntities` (engine map): appends entity A and then entity B (null
/// ones included) to the array `entities`, growing it when it is full.
pub fn hkp_binary_action_get_entities(e: &mut Engine, this: Ptr, entities: Ptr) {
    let action = this.addr();
    let entity_a = e.mem.u32(action + ENTITY_A);
    array_push(e, entities.addr(), entity_a);
    let entity_b = e.mem.u32(action + ENTITY_B);
    array_push(e, entities.addr(), entity_b);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_4ca0, fn_00cf4ca0(Ptr)),
        entry!(
            0x00cf_4cc0,
            hkp_binary_action_entity_removed_callback(Ptr, u32)
        ),
        entry!(0x00cf_4ce0, hkp_binary_action_set_entity_a(Ptr, u32)),
        entry!(0x00cf_4d30, hkp_binary_action_set_entity_b(Ptr, u32)),
        entry!(0x00cf_4d80, hkp_binary_action_construct(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00cf_4de0, hkp_binary_action_destruct(Ptr)),
        entry!(0x00cf_4e30, hkp_binary_action_get_entities(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range mapped, a memory router (allocator at the
    /// returned address) and doubles for the callees that log `(address, arguments)`.
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            REMOVE_ACTION_IMMEDIATELY,
            DETACH_ENTITY,
            ATTACH_ENTITY,
            STRING_PTR_CONSTRUCT,
            STRING_PTR_DESTRUCT,
            ARRAY_RESERVE_MORE,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (e, log, allocator)
    }

    fn calls(log: &Log) -> Vec<(u32, Vec<u32>)> {
        log.borrow().clone()
    }

    #[test]
    fn references_are_taken_on_non_null_entities_in_order() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4ca0, &args![action]);
        assert!(calls(&log).is_empty());
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        e.call(0x00cf_4ca0, &args![action]);
        assert_eq!(calls(&log), vec![(ADD_REFERENCE, vec![0x2000])]);
        log.borrow_mut().clear();
        e.mem.set_u32(action + ENTITY_A, 0x1000);
        e.call(0x00cf_4ca0, &args![action]);
        assert_eq!(
            calls(&log),
            vec![(ADD_REFERENCE, vec![0x1000]), (ADD_REFERENCE, vec![0x2000])]
        );
    }

    #[test]
    fn entity_removed_removes_the_action_from_its_world_only_when_it_has_one() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4cc0, &args![action, 0x1234u32]);
        assert!(calls(&log).is_empty());
        e.mem.set_u32(action + WORLD, 0x7000);
        e.call(0x00cf_4cc0, &args![action, 0x1234u32]);
        assert_eq!(
            calls(&log),
            vec![(REMOVE_ACTION_IMMEDIATELY, vec![0x7000, action])]
        );
    }

    #[test]
    fn set_entity_a_replaces_an_old_entity_in_a_world() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4ce0, &args![action, 0x2000u32]);
        assert_eq!(e.mem.u32(action + ENTITY_A), 0x2000);
        assert_eq!(calls(&log), vec![(ADD_REFERENCE, vec![0x2000])]);
        e.mem.set_u32(action + WORLD, 0x7000);
        e.call(0x00cf_4ce0, &args![action, 0x3000u32]);
        assert_eq!(e.mem.u32(action + ENTITY_A), 0x3000);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0);
        assert_eq!(
            calls(&log)[1..],
            [
                (ADD_REFERENCE, vec![0x3000]),
                (DETACH_ENTITY, vec![action, 0x2000]),
                (REMOVE_REFERENCE, vec![0x2000]),
                (ATTACH_ENTITY, vec![0x7000, action, 0x3000]),
            ]
        );
    }

    #[test]
    fn set_entity_b_without_an_old_entity_or_world_only_stores() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4d30, &args![action, 0x3000u32]);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0x3000);
        assert_eq!(calls(&log), vec![(ADD_REFERENCE, vec![0x3000])]);
    }

    #[test]
    fn set_entity_b_replaces_an_old_entity_in_a_world() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + WORLD, 0x7000);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        e.call(0x00cf_4d30, &args![action, 0x3000u32]);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0x3000);
        assert_eq!(
            calls(&log),
            vec![
                (ADD_REFERENCE, vec![0x3000]),
                (DETACH_ENTITY, vec![action, 0x2000]),
                (REMOVE_REFERENCE, vec![0x2000]),
                (ATTACH_ENTITY, vec![0x7000, action, 0x3000]),
            ]
        );
    }

    #[test]
    fn set_entity_b_replaces_an_old_entity_outside_a_world_without_world_calls() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        e.call(0x00cf_4d30, &args![action, 0u32]);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0);
        assert_eq!(
            calls(&log),
            vec![(ADD_REFERENCE, vec![0]), (REMOVE_REFERENCE, vec![0x2000])]
        );
    }

    #[test]
    fn construct_fills_the_fields_and_references_the_entities() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + WORLD, 0x99);
        let got = e
            .call(0x00cf_4d80, &args![action, 0x1000u32, 0x2000u32, 0x55u32])
            .u32();
        assert_eq!(got, action);
        assert_eq!(e.mem.u32(action + USER_DATA), 0x55);
        assert_eq!(e.mem.u16(action + 6), 1);
        assert_eq!(e.mem.u32(action), BINARY_ACTION_VTABLE);
        assert_eq!(e.mem.u32(action + WORLD), 0);
        assert_eq!(e.mem.u32(action + 0xc), 0);
        assert_eq!(e.mem.u32(action + ENTITY_A), 0x1000);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0x2000);
        assert_eq!(
            calls(&log),
            vec![
                (STRING_PTR_CONSTRUCT, vec![action + NAME]),
                (ADD_REFERENCE, vec![0x1000]),
                (ADD_REFERENCE, vec![0x2000]),
            ]
        );
    }

    #[test]
    fn construct_with_both_entities_null_takes_no_references() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4d80, &args![action, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls(&log),
            vec![(STRING_PTR_CONSTRUCT, vec![action + NAME])]
        );
    }

    #[test]
    fn construct_with_only_the_second_entity_references_it() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.call(0x00cf_4d80, &args![action, 0u32, 0x2000u32, 0u32]);
        assert_eq!(
            calls(&log),
            vec![
                (STRING_PTR_CONSTRUCT, vec![action + NAME]),
                (ADD_REFERENCE, vec![0x2000]),
            ]
        );
    }

    #[test]
    fn destruct_releases_the_entities_and_destroys_the_name() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + ENTITY_A, 0x1000);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        e.call(0x00cf_4de0, &args![action]);
        assert_eq!(e.mem.u32(action + ENTITY_A), 0);
        assert_eq!(e.mem.u32(action + ENTITY_B), 0);
        assert_eq!(e.mem.u32(action), BASE_VTABLE);
        assert_eq!(
            calls(&log),
            vec![
                (REMOVE_REFERENCE, vec![0x1000]),
                (REMOVE_REFERENCE, vec![0x2000]),
                (STRING_PTR_DESTRUCT, vec![action + NAME]),
            ]
        );
    }

    #[test]
    fn destruct_skips_null_entities() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        e.call(0x00cf_4de0, &args![action]);
        assert_eq!(
            calls(&log),
            vec![
                (REMOVE_REFERENCE, vec![0x2000]),
                (STRING_PTR_DESTRUCT, vec![action + NAME]),
            ]
        );
    }

    #[test]
    fn get_entities_appends_both_entities_including_null_ones() {
        let (mut e, log, _) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 4);
        e.mem.set_u32(data, 0xaaaa);
        e.call(0x00cf_4e30, &args![action, array]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [0xaaaa, 0, 0x2000]
        );
        assert!(calls(&log).is_empty());
    }

    #[test]
    fn get_entities_grows_a_full_array_before_each_append() {
        let (mut e, log, allocator) = setup();
        let action = e.mem.alloc(0x40);
        e.mem.set_u32(action + ENTITY_A, 0x1000);
        e.mem.set_u32(action + ENTITY_B, 0x2000);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        // The flag bits above the 30-bit capacity are ignored in the compare.
        e.mem.set_u32(array + 8, 0x8000_0001);
        let bigger = e.mem.alloc(0x20);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE_MORE, a.to_vec()));
            let word = e.mem.u32(e.mem.u32(a[1]));
            e.mem.set_u32(bigger, word);
            e.mem.set_u32(a[1], bigger);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        e.call(0x00cf_4e30, &args![action, array]);
        assert_eq!(
            calls(&log),
            vec![(ARRAY_RESERVE_MORE, vec![allocator, array, 4])]
        );
        assert_eq!(e.mem.u32(array), bigger);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(
            [e.mem.u32(bigger + 4), e.mem.u32(bigger + 8)],
            [0x1000, 0x2000]
        );
    }
}
