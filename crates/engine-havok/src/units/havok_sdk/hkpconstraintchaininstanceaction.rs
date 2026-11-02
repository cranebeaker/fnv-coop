//! `Havok SDK/hkpconstraintchaininstanceaction.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The action (`hkpConstraintChainInstanceAction`, 0x1c bytes) points at its chain instance in
//! the word at +0x18; the instance keeps its chained entities in a pointer array at +0x38 (data),
//! +0x3c (size) and +0x40 (capacity and flags).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e063c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstanceAction::applyAction` (Xbox PDB): takes `this` and one word and
/// returns without doing anything.
pub fn hkp_constraint_chain_instance_action_apply_action(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
) {
}

// Translated from 00e063d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstanceAction::clone` (Xbox PDB): takes `this` and two words and
/// returns null.
pub fn hkp_constraint_chain_instance_action_clone(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
) -> Ptr {
    Ptr::NULL
}

// Translated from 00e063e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstanceAction::entityRemovedCallback` (Xbox PDB): jumps (without
/// popping the caller's argument) to the virtual function at `+0x10` of the chain instance
/// kept at `this + 0x18`, with the instance as `this`; the argument is the removed entity
/// (the chain instance's own `entityRemovedCallback`, `00dd0460`, reads it).
pub fn hkp_constraint_chain_instance_action_entity_removed_callback(
    e: &mut Engine,
    this: Ptr,
    entity: Ptr,
) {
    let instance = e.mem.u32(this.addr() + 0x18);
    e.vcall(instance, 0x10, &args![entity]);
}

// Translated from 00e063f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstanceAction::getEntities` (Xbox PDB): `out` is an `hkArray` of
/// pointers (data +0, size +4, capacity and flags +8). When its capacity (`& 0x3fffffff`) is
/// below the chain's size it frees its storage through the allocator's slot `0x10` (pointer,
/// `capacity * 4`) unless bit 31 is set, asks slot `0xc` for `size * 4` bytes (the allocator
/// answers the block and writes back the size it gave) and stores the new capacity, the
/// bytes divided by 4 (truncating). Then it takes the chain's size and copies the pointers.
pub fn hkp_constraint_chain_instance_action_get_entities(e: &mut Engine, this: Ptr, out: Ptr) {
    let chain = e.mem.u32(this.addr() + 0x18);
    let source = chain + 0x38;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let out = out.addr();
    let flags = e.mem.u32(out + 8);
    let capacity = flags & 0x3fff_ffff;
    if (capacity as i32) < e.mem.i32(source + 4) {
        if (flags as i32) >= 0 {
            let data = e.mem.u32(out);
            e.vcall(allocator, 0x10, &args![data, capacity.wrapping_mul(4)]);
        }
        let wanted = e.mem.u32(source + 4).wrapping_mul(4);
        let (block, bytes) = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), wanted);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        e.mem.set_u32(out, block);
        // CDQ / AND 3 / ADD / SAR 2: signed division by 4.
        e.mem
            .set_i32(out + 8, bytes.wrapping_add((bytes >> 31) & 3) >> 2);
    }
    let count = e.mem.i32(source + 4);
    let data = e.mem.u32(out);
    e.mem.set_i32(out + 4, count);
    let from = e.mem.u32(source);
    for index in 0..count.max(0) as u32 {
        let value = e.mem.u32(from + 4 * index);
        e.mem.set_u32(data + 4 * index, value);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e0_63c0,
            hkp_constraint_chain_instance_action_apply_action(Ptr, u32)
        ),
        entry!(0x00e0_63d0, hkp_constraint_chain_instance_action_clone(Ptr, u32, u32) -> Ptr),
        entry!(
            0x00e0_63e0,
            hkp_constraint_chain_instance_action_entity_removed_callback(Ptr, Ptr)
        ),
        entry!(
            0x00e0_63f0,
            hkp_constraint_chain_instance_action_get_entities(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const BUFFER_ALLOCATE_SLOT: u32 = 0x0900_000c;
    const BUFFER_FREE_SLOT: u32 = 0x0900_0010;

    type FreeLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// A memory router whose allocator hands out buffers rounded up by 8 bytes (slot 0xc) and
    /// logs the `(pointer, size)` pairs it frees (slot 0x10).
    fn install_allocator(e: &mut Engine) -> FreeLog {
        let freed: FreeLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, BUFFER_ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0x10, BUFFER_FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(BUFFER_ALLOCATE_SLOT, |e, a| {
            let wanted = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], wanted + 8);
            Ret {
                eax: e.mem.alloc(wanted + 8),
                ..Ret::default()
            }
        });
        let log = Rc::clone(&freed);
        e.register_double(BUFFER_FREE_SLOT, move |_, a| {
            log.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        freed
    }

    /// An action whose chain instance holds `entities`; an output array with the given data
    /// block, size 0 and capacity word.
    fn make_action(e: &mut Engine, entities: &[u32]) -> u32 {
        let chain = e.mem.alloc(0x50);
        let data = e.mem.alloc(4 * entities.len() as u32 + 4);
        for (index, entity) in entities.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *entity);
        }
        e.mem.set_u32(chain + 0x38, data);
        e.mem.set_u32(chain + 0x3c, entities.len() as u32);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0x18, chain);
        action
    }

    fn make_output(e: &mut Engine, capacity_word: u32) -> u32 {
        let out = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(out, data);
        e.mem.set_u32(out + 8, capacity_word);
        out
    }

    fn entities_of(e: &Engine, out: u32) -> Vec<u32> {
        let data = e.mem.u32(out);
        (0..e.mem.u32(out + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn apply_action_and_clone_do_nothing() {
        let mut e = Engine::new();
        let ret = e.call(0x00e0_63c0, &args![0x1000u32, 7u32]);
        assert_eq!(ret.u32(), 0);
        let ret = e.call(0x00e0_63d0, &args![0x1000u32, 7u32, 8u32]);
        assert_eq!(ret.u32(), 0);
    }

    #[test]
    fn the_removal_callback_forwards_the_entity_to_the_instance() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x0900_0010, move |_, a| {
            sink.borrow_mut().extend_from_slice(a);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let instance = e.mem.alloc(8);
        e.mem.set_u32(instance, vtable);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 0x18, instance);
        e.call(0x00e0_63e0, &args![action, 0xe771u32]);
        assert_eq!(*seen.borrow(), [instance, 0xe771]);
    }

    #[test]
    fn get_entities_copies_into_a_big_enough_array_without_allocating() {
        let mut e = Engine::new();
        let freed = install_allocator(&mut e);
        let action = make_action(&mut e, &[11, 22, 33]);
        let out = make_output(&mut e, 5);
        let data = e.mem.u32(out);
        e.call(0x00e0_63f0, &args![action, out]);
        assert_eq!(e.mem.u32(out), data);
        assert_eq!(entities_of(&e, out), [11, 22, 33]);
        assert_eq!(e.mem.u32(out + 8), 5);
        assert!(freed.borrow().is_empty());
    }

    #[test]
    fn get_entities_frees_owned_storage_and_takes_a_larger_block() {
        let mut e = Engine::new();
        let freed = install_allocator(&mut e);
        let action = make_action(&mut e, &[11, 22, 33]);
        let out = make_output(&mut e, 2);
        let old = e.mem.u32(out);
        e.call(0x00e0_63f0, &args![action, out]);
        assert_eq!(*freed.borrow(), [(old, 8)]);
        assert_ne!(e.mem.u32(out), old);
        assert_eq!(entities_of(&e, out), [11, 22, 33]);
        // 12 bytes asked, 20 given: capacity 5.
        assert_eq!(e.mem.u32(out + 8), 5);
    }

    #[test]
    fn get_entities_keeps_storage_it_does_not_own() {
        let mut e = Engine::new();
        let freed = install_allocator(&mut e);
        let action = make_action(&mut e, &[7, 8]);
        let out = make_output(&mut e, 0x8000_0001);
        e.call(0x00e0_63f0, &args![action, out]);
        assert!(freed.borrow().is_empty());
        assert_eq!(entities_of(&e, out), [7, 8]);
        assert_eq!(e.mem.u32(out + 8), 4);
        // An empty chain copies nothing and needs no new block.
        let empty = make_action(&mut e, &[]);
        let out = make_output(&mut e, 0);
        let data = e.mem.u32(out);
        e.call(0x00e0_63f0, &args![empty, out]);
        assert_eq!(e.mem.u32(out), data);
        assert_eq!(e.mem.u32(out + 4), 0);
    }
}
