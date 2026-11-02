//! `Havok SDK/hkpbroadphaseborder.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpBroadPhaseBorder` (the Xbox PDB has no record of the Havok classes, so the field names
//! below are unconfirmed): an object with three listener sub-objects (a world deletion
//! listener at `+8`, a phantom overlap listener at `+0xC`, a world post-simulation listener
//! at `+0x10`) that keeps six thin phantoms (`+0x18` to `+0x2C`) on the faces of the world's
//! bounds. `+0x14` is the world (0 when deactivated), `+0x30` what to do to an entity that
//! leaves the bounds (0 or 1: change its motion type, 2: remove it), `+0x34` a byte and
//! `+0x38` an `hkArray` of the entities collected during a step (data, size, capacity).

use super::hkpsphereshape::ZERO_F32;
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The array growth helper (`00c90940`, cdecl: allocator, array, element size) and the array
/// free helper (`00c90740`, cdecl: data, size, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
const ARRAY_FREE: u32 = 0x00c9_0740;
/// Reference count increment (`00c90510`) and release (`00c905b0`), `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpWorld::removePhantomBatch` (`00c95800`, `this` the world, the phantom array and the
/// count), `hkpPhantom::removePhantomOverlapListener` (`00d07c80`, `this` the phantom, one word),
/// `hkpPhantom::addPhantomOverlapListener` (`00d07de0`, same), `hkpWorld::addPhantom`
/// (`00c936a0`, one word), the world deletion listener registration (`00c93a60`, one word),
/// `hkpWorld::addWorldPostSimulationListener` (`00c93920`, one word),
/// `hkpAabbPhantom::hkpAabbPhantom` (`00cf2cb0`, the box and 0), `hkpRigidBody::setMotionType`
/// (`00ca8420`, three words) and `hkpWorld::removeEntity` (`00c91620`, the result slot and the
/// entity).
const REMOVE_PHANTOM_BATCH: u32 = 0x00c9_5800;
const REMOVE_OVERLAP_LISTENER: u32 = 0x00d0_7c80;
const ADD_OVERLAP_LISTENER: u32 = 0x00d0_7de0;
const ADD_PHANTOM: u32 = 0x00c9_36a0;
const ADD_WORLD_DELETION_LISTENER: u32 = 0x00c9_3a60;
const ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const AABB_PHANTOM_CONSTRUCT: u32 = 0x00cf_2cb0;
const SET_MOTION_TYPE: u32 = 0x00ca_8420;
const REMOVE_ENTITY: u32 = 0x00c9_1620;
/// The entity ordering function the sort of `fn_00d0b630` is given (`00d0b5d0`, cdecl, two
/// entities, a boolean in `AL`).
const ENTITY_LESS: u32 = 0x00d0_b5d0;

const WORLD: u32 = 0x14;
const PHANTOMS: u32 = 0x18;
const ON_EXIT_ACTION: u32 = 0x30;
const COLLECT_FLAG: u32 = 0x34;
const ENTITIES: u32 = 0x38;
/// Fields of the world the bounds are read from: the minimum (`+0x310`) and the maximum
/// (`+0x320`), each a vector of four words.
const WORLD_BOUNDS_MIN: u32 = 0x310;
const WORLD_BOUNDS_MAX: u32 = 0x320;

/// The vtables of the three listener sub-objects while the border is built (`+0`, `+8`, `+0xC`
/// and `+0x10`).
const VTABLE_BORDER: u32 = 0x010c_d3c4;
const VTABLE_DELETION_LISTENER: u32 = 0x010c_d3b4;
const VTABLE_OVERLAP_LISTENER: u32 = 0x010c_d3a4;
const VTABLE_POST_SIMULATION_LISTENER: u32 = 0x010c_d394;
/// The vtables the constructor stores first and then replaces.
const EARLY_DELETION_LISTENER: u32 = 0x010c_b1c4;
const EARLY_OVERLAP_LISTENER: u32 = 0x010c_925c;
const EARLY_POST_SIMULATION_LISTENER: u32 = 0x010c_cf7c;
/// The size of an `hkpAabbPhantom` and the phantom type the constructor stores at `+0x28`.
const PHANTOM_SIZE: u32 = 0xe0;
const PHANTOM_KIND: u8 = 3;

// Translated from 00d0b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhaseBorder::deactivate` (engine map): when the border has a world (`+0x14`),
/// removes the six phantoms from it in one batch, then for each phantom removes the overlap
/// listener (`this + 0xC`), releases it and clears its slot, calls slot 4 of the deletion
/// listener sub-object (`this + 8`) with the world and clears `+0x14`.
pub fn hkp_broad_phase_border_deactivate(e: &mut Engine, this: Ptr) {
    let border = this.addr();
    let world = e.mem.u32(border + WORLD);
    if world == 0 {
        return;
    }
    e.call(REMOVE_PHANTOM_BATCH, &args![world, border + PHANTOMS, 6u32]);
    for slot in 0..6 {
        let at = border + PHANTOMS + 4 * slot;
        let phantom = e.mem.u32(at);
        e.call(REMOVE_OVERLAP_LISTENER, &args![phantom, border + 0xc]);
        let phantom = e.mem.u32(at);
        e.call(REMOVE_REFERENCE, &args![phantom]);
        e.mem.set_u32(at, 0);
    }
    let world = e.mem.u32(border + WORLD);
    e.vcall(border + 8, 4, &args![world]);
    e.mem.set_u32(border + WORLD, 0);
}

// Translated from 00d0b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhaseBorder::maxPositionExceededCallback` (engine map): `ECX` = border, the
/// entity on the stack. With action 0 or 1 (`+0x30`) it calls `hkpRigidBody::setMotionType`
/// on the entity with 5, 1 and 0; with action 2 and the word at `+8` of the entity equal
/// to the border's world (`+0x14`) it has that world remove the entity (the result slot being
/// a stack word holding the entity); other actions do nothing.
pub fn hkp_broad_phase_border_max_position_exceeded_callback(
    e: &mut Engine,
    this: Ptr,
    entity: Ptr,
) {
    let border = this.addr();
    match e.mem.u32(border + ON_EXIT_ACTION) {
        0 | 1 => {
            e.call(SET_MOTION_TYPE, &args![entity, 5u32, 1u32, 0u32]);
        }
        2 => {
            let world = e.mem.u32(border + WORLD);
            if e.mem.u32(entity.addr() + 8) == world {
                e.with_stack(4, |e, result| {
                    e.mem.set_u32(result.addr(), entity.addr());
                    e.call(REMOVE_ENTITY, &args![world, result, entity]);
                });
            }
        }
        _ => {}
    }
}

// Translated from 00d0b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles and frees the collected entities (no engine-map name; `ECX` = the post-simulation
/// listener sub-object, `border + 0x10`, and one stack word never read). When the array
/// (`border + 0x38`: data, size) is not empty: sorts it with `00e10760` and `00d0b5d0` when it
/// holds more than one entity, calls slot `0xC` of the border (`this - 0x10`) with each entity
/// (the size is read again after each call), frees the array with `00c90740` (data, size, 4)
/// and sets the size to 0. The data pointer is kept.
pub fn fn_00d0b630(e: &mut Engine, this: Ptr, _unused: u32) {
    let listener = this.addr();
    let size = e.mem.u32(listener + 0x2c) as i32;
    if size == 0 {
        return;
    }
    if size > 1 {
        let data = e.mem.u32(listener + 0x28);
        quick_sort_recursive_entities(e, data, 0, size - 1, ENTITY_LESS);
    }
    let mut index = 0u32;
    if (e.mem.u32(listener + 0x2c) as i32) > 0 {
        loop {
            let data = e.mem.u32(listener + 0x28);
            let entity = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            e.vcall(listener - 0x10, 0xc, &args![entity]);
            index += 1;
            if (index as i32) >= (e.mem.u32(listener + 0x2c) as i32) {
                break;
            }
        }
    }
    let size = e.mem.u32(listener + 0x2c);
    let data = e.mem.u32(listener + 0x28);
    e.call(ARRAY_FREE, &args![data, size, 4u32]);
    e.mem.set_u32(listener + 0x2c, 0);
}

// Translated from 00d0b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The overlap listener's callback (no engine-map name; `ECX` = the overlap listener
/// sub-object, `border + 0xC`, the overlap event on the stack). When the collidable at `+4` of
/// the event has the byte 1 at `+0x18` and an owner (the collidable plus the signed byte at
/// `+0x10`; skipped when that is 0): if the border's collect byte (`+0x34`) is set, appends the owner
/// to the entity array (growing it through `00c90940` when full) and takes a reference on
/// it; otherwise calls slot `0xC` of the border with the owner.
pub fn fn_00d0b6a0(e: &mut Engine, this: Ptr, event: Ptr) {
    let listener = this.addr();
    let collidable = e.mem.u32(event.addr() + 4);
    if e.mem.u8(collidable + 0x18) != 1 {
        return;
    }
    let owner = i32::from(e.mem.u8(collidable + 0x10) as i8) as u32;
    let owner = owner.wrapping_add(collidable);
    if owner == 0 {
        return;
    }
    if e.mem.u8(listener + 0x28) != 0 {
        let array = listener + 0x2c;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), owner);
        e.mem
            .set_u32(array + 4, e.mem.u32(array + 4).wrapping_add(1));
        e.call(ADD_REFERENCE, &args![owner]);
        return;
    }
    e.vcall(listener - 0xc, 0xc, &args![owner]);
}

/// One face of the world's bounds as the `hkAabb` the phantom is built from (two vectors,
/// minimum then maximum): the `x`, `y`, `z` words of both corners with the `float` at
/// `01011d78` in the fourth lane.
fn face(zero: u32, low: [u32; 3], high: [u32; 3]) -> [u32; 8] {
    [
        low[0], low[1], low[2], zero, high[0], high[1], high[2], zero,
    ]
}

// Translated from 00d0b710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBroadPhaseBorder::hkpBroadPhaseBorder` (engine map): `ECX` = border; the world, the
/// action for entities that leave the bounds and a byte on the stack. Sets the reference count
/// word `+6` to 1, the vtables, the empty entity array (capacity word `0x80000000`), the three
/// fields and a reference on the border, registers the deletion (`+8`) and post-simulation
/// (`+0x10`) listeners with the world, then makes six `hkpAabbPhantom`s (0xE0 bytes from the
/// memory router's allocator, slot 4, with their size word at `+4`) of the thin boxes on the
/// faces of the world's bounds (`+0x310` minimum, `+0x320` maximum; the order is +x, -x, +y,
/// -y, +z, -z), each with the byte 3 at `+0x28`, the overlap listener (`this + 0xC`) added and
/// stored at `+0x18` to `+0x2C` before it is added to the world. Returns `this`.
pub fn hkp_broad_phase_border_construct(
    e: &mut Engine,
    this: Ptr,
    world: u32,
    action: u32,
    collect: u8,
) -> Ptr {
    let border = this.addr();
    e.mem.set_u16(border + 6, 1);
    e.mem.set_u32(border + 8, EARLY_DELETION_LISTENER);
    e.mem.set_u32(border + 0xc, EARLY_OVERLAP_LISTENER);
    e.mem.set_u32(border + 0x10, EARLY_POST_SIMULATION_LISTENER);
    e.mem.set_u32(border, VTABLE_BORDER);
    e.mem.set_u32(border + 8, VTABLE_DELETION_LISTENER);
    e.mem.set_u32(border + 0xc, VTABLE_OVERLAP_LISTENER);
    e.mem
        .set_u32(border + 0x10, VTABLE_POST_SIMULATION_LISTENER);
    e.mem.set_u32(border + ENTITIES, 0);
    e.mem.set_u32(border + ENTITIES + 4, 0);
    e.mem.set_u32(border + ENTITIES + 8, 0x8000_0000);
    e.mem.set_u32(border + ON_EXIT_ACTION, action);
    e.mem.set_u32(border + WORLD, world);
    e.mem.set_u8(border + COLLECT_FLAG, collect);
    e.call(ADD_REFERENCE, &args![this]);
    e.call(ADD_WORLD_DELETION_LISTENER, &args![world, border + 8]);
    e.call(ADD_POST_SIMULATION_LISTENER, &args![world, border + 0x10]);
    let low = [0, 4, 8].map(|offset| e.mem.u32(world + WORLD_BOUNDS_MIN + offset));
    let high = [0, 4, 8].map(|offset| e.mem.u32(world + WORLD_BOUNDS_MAX + offset));
    let zero = e.mem.u32(ZERO_F32);
    let faces = [
        face(zero, [high[0], low[1], low[2]], high),
        face(zero, low, [low[0], high[1], high[2]]),
        face(zero, [low[0], high[1], low[2]], high),
        face(zero, low, [high[0], low[1], high[2]]),
        face(zero, [low[0], low[1], high[2]], high),
        face(zero, low, [high[0], high[1], low[2]]),
    ];
    for (slot, words) in faces.into_iter().enumerate() {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let memory = e.vcall(allocator, 4, &args![PHANTOM_SIZE]).u32();
        e.mem.set_u16(memory + 4, PHANTOM_SIZE as u16);
        let phantom = e.with_stack(0x20, |e, aabb| {
            for (index, word) in words.into_iter().enumerate() {
                e.mem.set_u32(aabb.addr() + 4 * index as u32, word);
            }
            e.call(AABB_PHANTOM_CONSTRUCT, &args![memory, aabb, 0u32])
                .u32()
        });
        e.mem.set_u8(phantom + 0x28, PHANTOM_KIND);
        e.call(ADD_OVERLAP_LISTENER, &args![phantom, border + 0xc]);
        e.mem.set_u32(border + PHANTOMS + 4 * slot as u32, phantom);
        e.call(ADD_PHANTOM, &args![world, phantom]);
    }
    this
}

// Translated from 00e10760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive` for entity pointers (engine map): cdecl; the array of
/// pointers, the first and last index (signed) and the ordering function (`less(a, b)`,
/// cdecl, the result in `AL`). Hoare partition around the middle element (`(first + last) >> 1`),
/// recursing into the left part and iterating on the right one.
pub fn quick_sort_recursive_entities(e: &mut Engine, array: u32, first: i32, last: i32, less: u32) {
    let item = |e: &Engine, index: i32| {
        e.mem
            .u32(array.wrapping_add((index as u32).wrapping_mul(4)))
    };
    let precedes = |e: &mut Engine, a: u32, b: u32| e.call(less, &args![a, b]).eax & 0xff != 0;
    let mut first = first;
    loop {
        let mut low = first;
        let mut high = last;
        let pivot = item(e, low.wrapping_add(high) >> 1);
        loop {
            loop {
                let candidate = item(e, low);
                if !precedes(e, candidate, pivot) {
                    break;
                }
                low += 1;
            }
            loop {
                let candidate = item(e, high);
                if !precedes(e, pivot, candidate) {
                    break;
                }
                high -= 1;
            }
            if high < low {
                break;
            }
            if high != low {
                let at_low = item(e, low);
                let at_high = item(e, high);
                e.mem
                    .set_u32(array.wrapping_add((high as u32).wrapping_mul(4)), at_low);
                e.mem
                    .set_u32(array.wrapping_add((low as u32).wrapping_mul(4)), at_high);
            }
            high -= 1;
            low += 1;
            if low > high {
                break;
            }
        }
        if first < high {
            quick_sort_recursive_entities(e, array, first, high, less);
        }
        if low < last {
            first = low;
        } else {
            return;
        }
    }
}

// Translated from 00d0b5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a "less than" comparison for a sort, cdecl, two stack words (nothing
/// popped): answers 1 when the word at +0xd4 of `a` is below (unsigned) the one of `b`, else 0
/// (`CMP` / `SBB` / `NEG`). Ghidra made no function here; the address is pushed at 00d0b63f.
pub fn fn_00d0b5d0(e: &mut Engine, a: u32, b: u32) -> u32 {
    u32::from(e.mem.u32(a + 0xd4) < e.mem.u32(b + 0xd4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_b570, hkp_broad_phase_border_deactivate(Ptr)),
        entry!(
            0x00d0_b5f0,
            hkp_broad_phase_border_max_position_exceeded_callback(Ptr, Ptr)
        ),
        entry!(0x00d0_b630, fn_00d0b630(Ptr, u32)),
        entry!(0x00d0_b6a0, fn_00d0b6a0(Ptr, Ptr)),
        entry!(0x00d0_b710, hkp_broad_phase_border_construct(Ptr, u32, u32, u8) -> Ptr),
        entry!(
            0x00e1_0760,
            quick_sort_recursive_entities(u32, i32, i32, u32)
        ),
        entry!(0x00d0_b5d0, fn_00d0b5d0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_DELETION: u32 = 0x0900_0301;
    const SLOT_ENTITY: u32 = 0x0900_0302;
    const SLOT_ALLOCATE: u32 = 0x0900_0303;

    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        (e, Rc::default())
    }

    /// Logs every call to the given addresses with its argument words.
    fn log_callees(e: &mut Engine, log: &Log, addresses: &[u32]) {
        for &address in addresses {
            let sink = Rc::clone(log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable (at `+0`) holds `slots` at the given byte offsets.
    fn object_with_slots(e: &mut Engine, base: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x80);
        for &(offset, target) in slots {
            e.mem.set_u32(table + offset, target);
        }
        let object = if base == 0 { e.mem.alloc(0x80) } else { base };
        e.mem.set_u32(object, table);
        object
    }

    /// Doubles for the memory router, whose allocator's slot 4 hands out zeroed 0x100-byte
    /// blocks.
    fn memory(e: &mut Engine, log: &Log) -> u32 {
        let sink = Rc::clone(log);
        e.register_double(SLOT_ALLOCATE, move |e, a| {
            sink.borrow_mut().push((SLOT_ALLOCATE, a.to_vec()));
            Ret {
                eax: e.mem.alloc(0x100),
                ..Ret::default()
            }
        });
        let allocator = object_with_slots(e, 0, &[(4, SLOT_ALLOCATE)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    #[test]
    fn deactivate_without_a_world_does_nothing() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[REMOVE_PHANTOM_BATCH]);
        let border = e.mem.alloc(0x80);
        e.call(0x00d0_b570, &args![border]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn deactivate_removes_the_phantoms_listener_and_world() {
        let (mut e, log) = engine();
        log_callees(
            &mut e,
            &log,
            &[
                REMOVE_PHANTOM_BATCH,
                REMOVE_OVERLAP_LISTENER,
                REMOVE_REFERENCE,
                SLOT_DELETION,
            ],
        );
        let border = e.mem.alloc(0x80);
        object_with_slots(&mut e, border + 8, &[(4, SLOT_DELETION)]);
        e.mem.set_u32(border + WORLD, 0x7000);
        for slot in 0..6 {
            e.mem
                .set_u32(border + PHANTOMS + 4 * slot, 0x100 * (slot + 1));
        }
        e.call(0x00d0_b570, &args![border]);
        let mut expected = vec![(REMOVE_PHANTOM_BATCH, vec![0x7000, border + PHANTOMS, 6])];
        for slot in 1..=6 {
            expected.push((REMOVE_OVERLAP_LISTENER, vec![0x100 * slot, border + 0xc]));
            expected.push((REMOVE_REFERENCE, vec![0x100 * slot]));
        }
        expected.push((SLOT_DELETION, vec![border + 8, 0x7000]));
        assert_eq!(*log.borrow(), expected);
        assert_eq!(e.mem.u32(border + WORLD), 0);
        for slot in 0..6 {
            assert_eq!(e.mem.u32(border + PHANTOMS + 4 * slot), 0);
        }
    }

    #[test]
    fn exceeding_the_bounds_changes_the_motion_type_for_actions_0_and_1() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SET_MOTION_TYPE, REMOVE_ENTITY]);
        let border = e.mem.alloc(0x80);
        for action in [0, 1] {
            e.mem.set_u32(border + ON_EXIT_ACTION, action);
            e.call(0x00d0_b5f0, &args![border, 0x4444u32]);
        }
        assert_eq!(
            calls_to(&log, SET_MOTION_TYPE),
            vec![vec![0x4444, 5, 1, 0]; 2]
        );
        assert!(calls_to(&log, REMOVE_ENTITY).is_empty());
    }

    #[test]
    fn exceeding_the_bounds_removes_an_entity_of_this_world_for_action_2() {
        let (mut e, log) = engine();
        let sink = Rc::clone(&log);
        e.register_double(REMOVE_ENTITY, move |e, a| {
            let slot = e.mem.u32(a[1]);
            sink.borrow_mut()
                .push((REMOVE_ENTITY, vec![a[0], slot, a[2]]));
            Ret::default()
        });
        log_callees(&mut e, &log, &[SET_MOTION_TYPE]);
        let border = e.mem.alloc(0x80);
        e.mem.set_u32(border + ON_EXIT_ACTION, 2);
        e.mem.set_u32(border + WORLD, 0x7000);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, 0x7000);
        e.call(0x00d0_b5f0, &args![border, entity]);
        assert_eq!(
            *log.borrow(),
            vec![(REMOVE_ENTITY, vec![0x7000, entity, entity])]
        );
    }

    #[test]
    fn exceeding_the_bounds_ignores_other_worlds_and_other_actions() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SET_MOTION_TYPE, REMOVE_ENTITY]);
        let border = e.mem.alloc(0x80);
        e.mem.set_u32(border + WORLD, 0x7000);
        let entity = e.mem.alloc(0x20);
        e.mem.set_u32(entity + 8, 0x7001);
        e.mem.set_u32(border + ON_EXIT_ACTION, 2);
        e.call(0x00d0_b5f0, &args![border, entity]);
        e.mem.set_u32(entity + 8, 0x7000);
        e.mem.set_u32(border + ON_EXIT_ACTION, 3);
        e.call(0x00d0_b5f0, &args![border, entity]);
        assert!(log.borrow().is_empty());
    }

    /// Registers `less` as the ordering function: ascending by value.
    fn ascending(e: &mut Engine) {
        e.register_double(ENTITY_LESS, |_, a| Ret {
            eax: u32::from(a[0] < a[1]),
            ..Ret::default()
        });
    }

    fn array_of(e: &mut Engine, values: &[u32]) -> u32 {
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *value);
        }
        array
    }

    fn contents(e: &Engine, array: u32, count: usize) -> Vec<u32> {
        (0..count)
            .map(|i| e.mem.u32(array + 4 * i as u32))
            .collect()
    }

    #[test]
    fn quick_sort_orders_the_whole_range() {
        let (mut e, _) = engine();
        ascending(&mut e);
        let values = [9, 3, 7, 3, 1, 8, 2, 6, 5, 4, 0, 7];
        let array = array_of(&mut e, &values);
        e.call(0x00e1_0760, &args![array, 0u32, 11u32, ENTITY_LESS]);
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        assert_eq!(contents(&e, array, 12), sorted);
    }

    #[test]
    fn quick_sort_leaves_elements_outside_the_range_alone() {
        let (mut e, _) = engine();
        ascending(&mut e);
        let array = array_of(&mut e, &[50, 9, 8, 7, 6, 5, 40]);
        e.call(0x00e1_0760, &args![array, 1u32, 5u32, ENTITY_LESS]);
        assert_eq!(contents(&e, array, 7), vec![50, 5, 6, 7, 8, 9, 40]);
    }

    #[test]
    fn quick_sort_handles_sorted_reversed_and_equal_input() {
        let (mut e, _) = engine();
        ascending(&mut e);
        for values in [[1, 2, 3, 4, 5], [5, 4, 3, 2, 1], [2, 2, 2, 2, 2]] {
            let array = array_of(&mut e, &values);
            e.call(0x00e1_0760, &args![array, 0u32, 4u32, ENTITY_LESS]);
            let mut sorted = values.to_vec();
            sorted.sort_unstable();
            assert_eq!(contents(&e, array, 5), sorted);
        }
    }

    #[test]
    fn quick_sort_of_one_element_still_asks_the_ordering_function() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[ENTITY_LESS]);
        let array = array_of(&mut e, &[7]);
        e.call(0x00e1_0760, &args![array, 0u32, 0u32, ENTITY_LESS]);
        assert_eq!(
            *log.borrow(),
            vec![(ENTITY_LESS, vec![7, 7]), (ENTITY_LESS, vec![7, 7])]
        );
    }

    /// A listener sub-object (`border + 0x10`) with `values` as its entity array.
    fn collected(e: &mut Engine, values: &[u32]) -> (u32, u32) {
        let border = object_with_slots(e, 0, &[(0xc, SLOT_ENTITY)]);
        let listener = border + 0x10;
        let array = array_of(e, values);
        e.mem.set_u32(listener + 0x28, array);
        e.mem.set_u32(listener + 0x2c, values.len() as u32);
        (border, array)
    }

    #[test]
    fn collected_entities_are_sorted_handled_and_freed() {
        let (mut e, log) = engine();
        ascending(&mut e);
        log_callees(&mut e, &log, &[SLOT_ENTITY, ARRAY_FREE]);
        let (border, array) = collected(&mut e, &[30, 10, 20]);
        e.call(0x00d0_b630, &args![border + 0x10, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (SLOT_ENTITY, vec![border, 10]),
                (SLOT_ENTITY, vec![border, 20]),
                (SLOT_ENTITY, vec![border, 30]),
                (ARRAY_FREE, vec![array, 3, 4]),
            ]
        );
        assert_eq!(e.mem.u32(border + 0x10 + 0x2c), 0);
        assert_eq!(e.mem.u32(border + 0x10 + 0x28), array);
    }

    #[test]
    fn a_single_collected_entity_is_not_sorted() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ARRAY_FREE, ENTITY_LESS]);
        let (border, array) = collected(&mut e, &[30]);
        e.call(0x00d0_b630, &args![border + 0x10, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (SLOT_ENTITY, vec![border, 30]),
                (ARRAY_FREE, vec![array, 1, 4]),
            ]
        );
    }

    #[test]
    fn no_collected_entities_means_nothing_happens() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ARRAY_FREE]);
        let (border, _) = collected(&mut e, &[]);
        e.call(0x00d0_b630, &args![border + 0x10, 0u32]);
        assert!(log.borrow().is_empty());
    }

    /// An overlap event whose collidable has `kind` at `+0x18` and the owner `delta` bytes
    /// away (`delta` stored as a signed byte at `+0x10`).
    fn overlap_event(e: &mut Engine, kind: u8, delta: i8) -> (u32, u32) {
        let collidable = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u8(collidable + 0x18, kind);
        e.mem.set_u8(collidable + 0x10, delta as u8);
        let event = e.mem.alloc(0x10);
        e.mem.set_u32(event + 4, collidable);
        (event, collidable)
    }

    #[test]
    fn an_overlap_of_a_non_body_collidable_is_ignored() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ADD_REFERENCE]);
        let border = object_with_slots(&mut e, 0, &[(0xc, SLOT_ENTITY)]);
        let (event, _) = overlap_event(&mut e, 2, -8);
        e.call(0x00d0_b6a0, &args![border + 0xc, event]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn an_overlap_without_an_owner_is_ignored() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ADD_REFERENCE]);
        let border = object_with_slots(&mut e, 0, &[(0xc, SLOT_ENTITY)]);
        e.map(0x40, 0x100);
        let collidable = 0x50;
        e.mem.set_u8(collidable + 0x18, 1);
        e.mem.set_u8(collidable + 0x10, (-0x50i8) as u8);
        let event = e.mem.alloc(0x10);
        e.mem.set_u32(event + 4, collidable);
        e.call(0x00d0_b6a0, &args![border + 0xc, event]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn an_overlap_is_passed_on_when_the_border_does_not_collect() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ADD_REFERENCE]);
        let border = object_with_slots(&mut e, 0, &[(0xc, SLOT_ENTITY)]);
        let (event, collidable) = overlap_event(&mut e, 1, -8);
        e.call(0x00d0_b6a0, &args![border + 0xc, event]);
        assert_eq!(
            *log.borrow(),
            vec![(SLOT_ENTITY, vec![border, collidable - 8])]
        );
    }

    #[test]
    fn an_overlap_is_collected_and_referenced_when_the_border_collects() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ADD_REFERENCE, ARRAY_GROW]);
        memory(&mut e, &log);
        let border = object_with_slots(&mut e, 0, &[(0xc, SLOT_ENTITY)]);
        e.mem.set_u8(border + COLLECT_FLAG, 1);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(border + ENTITIES, data);
        e.mem.set_u32(border + ENTITIES + 4, 1);
        e.mem.set_u32(border + ENTITIES + 8, 0x8000_0000 | 4);
        let (event, collidable) = overlap_event(&mut e, 1, 12);
        e.call(0x00d0_b6a0, &args![border + 0xc, event]);
        assert_eq!(e.mem.u32(data + 4), collidable + 12);
        assert_eq!(e.mem.u32(border + ENTITIES + 4), 2);
        assert_eq!(*log.borrow(), vec![(ADD_REFERENCE, vec![collidable + 12])]);
    }

    #[test]
    fn a_full_entity_array_is_grown_through_the_allocator_first() {
        let (mut e, log) = engine();
        log_callees(&mut e, &log, &[SLOT_ENTITY, ADD_REFERENCE]);
        let allocator = memory(&mut e, &log);
        let border = object_with_slots(&mut e, 0, &[(0xc, SLOT_ENTITY)]);
        e.mem.set_u8(border + COLLECT_FLAG, 1);
        let small = e.mem.alloc(0x20);
        let roomy = e.mem.alloc(0x40);
        e.mem.set_u32(border + ENTITIES, small);
        e.mem.set_u32(border + ENTITIES + 4, 2);
        e.mem.set_u32(border + ENTITIES + 8, 0x8000_0000 | 2);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_GROW, move |e, a| {
            sink.borrow_mut().push((ARRAY_GROW, a.to_vec()));
            e.mem.set_u32(a[1], roomy);
            e.mem.set_u32(a[1] + 8, 0x8000_0000 | 8);
            Ret::default()
        });
        let (event, collidable) = overlap_event(&mut e, 1, 4);
        e.call(0x00d0_b6a0, &args![border + 0xc, event]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ARRAY_GROW, vec![allocator, border + ENTITIES, 4]),
                (ADD_REFERENCE, vec![collidable + 4]),
            ]
        );
        assert_eq!(e.mem.u32(roomy + 8), collidable + 4);
        assert_eq!(e.mem.u32(border + ENTITIES + 4), 3);
    }

    #[test]
    fn construct_builds_six_face_phantoms_from_the_world_bounds() {
        let (mut e, log) = engine();
        log_callees(
            &mut e,
            &log,
            &[
                ADD_REFERENCE,
                ADD_WORLD_DELETION_LISTENER,
                ADD_POST_SIMULATION_LISTENER,
                ADD_OVERLAP_LISTENER,
                ADD_PHANTOM,
            ],
        );
        let allocator = memory(&mut e, &log);
        let sink = Rc::clone(&log);
        e.register_double(AABB_PHANTOM_CONSTRUCT, move |e, a| {
            let aabb: Vec<u32> = (0..8).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            sink.borrow_mut()
                .push((AABB_PHANTOM_CONSTRUCT, [vec![a[0], a[2]], aabb].concat()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.mem.set_u32(ZERO_F32, 0x1234_5678);
        let world = e.mem.alloc(0x340);
        // Minimum (1, 2, 3, 4), maximum (11, 12, 13, 14), as raw words.
        for (offset, word) in [(0x310, 1), (0x314, 2), (0x318, 3), (0x31c, 4)] {
            e.mem.set_u32(world + offset, word);
        }
        for (offset, word) in [(0x320, 11), (0x324, 12), (0x328, 13), (0x32c, 14)] {
            e.mem.set_u32(world + offset, word);
        }
        let border = e.mem.alloc(0x80);
        let got = e.call(0x00d0_b710, &args![border, world, 2u32, 1u8]).u32();
        assert_eq!(got, border);
        assert_eq!(e.mem.u16(border + 6), 1);
        assert_eq!(e.mem.u32(border), VTABLE_BORDER);
        assert_eq!(e.mem.u32(border + 8), VTABLE_DELETION_LISTENER);
        assert_eq!(e.mem.u32(border + 0xc), VTABLE_OVERLAP_LISTENER);
        assert_eq!(e.mem.u32(border + 0x10), VTABLE_POST_SIMULATION_LISTENER);
        assert_eq!(e.mem.u32(border + WORLD), world);
        assert_eq!(e.mem.u32(border + ON_EXIT_ACTION), 2);
        assert_eq!(e.mem.u8(border + COLLECT_FLAG), 1);
        assert_eq!(e.mem.u32(border + ENTITIES), 0);
        assert_eq!(e.mem.u32(border + ENTITIES + 4), 0);
        assert_eq!(e.mem.u32(border + ENTITIES + 8), 0x8000_0000);
        let z = 0x1234_5678;
        let faces: [[u32; 8]; 6] = [
            [11, 2, 3, z, 11, 12, 13, z],
            [1, 2, 3, z, 1, 12, 13, z],
            [1, 12, 3, z, 11, 12, 13, z],
            [1, 2, 3, z, 11, 2, 13, z],
            [1, 2, 13, z, 11, 12, 13, z],
            [1, 2, 3, z, 11, 12, 3, z],
        ];
        let constructs = calls_to(&log, AABB_PHANTOM_CONSTRUCT);
        assert_eq!(constructs.len(), 6);
        for (slot, call) in constructs.iter().enumerate() {
            let phantom = e.mem.u32(border + PHANTOMS + 4 * slot as u32);
            assert_eq!(call[..2], [phantom, 0]);
            assert_eq!(call[2..], faces[slot]);
            assert_eq!(e.mem.u16(phantom + 4), 0xe0);
            assert_eq!(e.mem.u8(phantom + 0x28), 3);
        }
        assert_eq!(calls_to(&log, ADD_REFERENCE), vec![vec![border]]);
        assert_eq!(
            calls_to(&log, ADD_WORLD_DELETION_LISTENER),
            vec![vec![world, border + 8]]
        );
        assert_eq!(
            calls_to(&log, ADD_POST_SIMULATION_LISTENER),
            vec![vec![world, border + 0x10]]
        );
        assert_eq!(
            calls_to(&log, SLOT_ALLOCATE),
            vec![vec![allocator, 0xe0]; 6]
        );
        let phantoms: Vec<u32> = (0..6)
            .map(|slot| e.mem.u32(border + PHANTOMS + 4 * slot))
            .collect();
        let listeners: Vec<Vec<u32>> = phantoms
            .iter()
            .map(|phantom| vec![*phantom, border + 0xc])
            .collect();
        assert_eq!(calls_to(&log, ADD_OVERLAP_LISTENER), listeners);
        let added: Vec<Vec<u32>> = phantoms
            .iter()
            .map(|phantom| vec![world, *phantom])
            .collect();
        assert_eq!(calls_to(&log, ADD_PHANTOM), added);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;

    #[test]
    fn fn_00d0b5d0_compares_the_words_unsigned() {
        let mut e = Engine::new();
        for (a, b, expected) in [
            (1, 2, 1),
            (2, 1, 0),
            (2, 2, 0),
            (0xffff_ffff, 1, 0),
            (1, 0xffff_ffff, 1),
        ] {
            let first = e.mem.alloc(0xe0);
            let second = e.mem.alloc(0xe0);
            e.mem.set_u32(first + 0xd4, a);
            e.mem.set_u32(second + 0xd4, b);
            assert_eq!(e.call(0x00d0_b5d0, &args![first, second]).u32(), expected);
        }
    }
}
