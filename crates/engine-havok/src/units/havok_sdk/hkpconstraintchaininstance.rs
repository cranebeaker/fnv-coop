//! `Havok SDK/hkpconstraintchaininstance.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dd0630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_05c0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dd0630(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_05c0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// The vtable of the chain instance (`0x010d_79e8`), of the action it makes (`0x010d_79c4`),
/// and the two base vtables the action's destructor passes through (`hkpConstraintChainInstanceAction`
/// is `0x010c_c2fc` while its string member is destroyed, then `0x0102_e388`).
const CHAIN_INSTANCE_VTABLE: u32 = 0x010d_79e8;
const CHAIN_ACTION_VTABLE: u32 = 0x010d_79c4;
const ACTION_BASE_VTABLE_A: u32 = 0x010c_c2fc;
const ACTION_BASE_VTABLE_B: u32 = 0x0102_e388;
/// Adds a reference to the object in `ECX` (`00c90510`) / removes one (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

/// The allocator behind the memory router.
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

// Translated from 00cf4eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstanceAction::_scalar_deleting_destructor_` (Xbox PDB): sets the
/// action's first base vtable (`010cc2fc`), destroys the `hkStringPtr` member at +0x14
/// (`00cb0c00`), sets the `hkReferencedObject` vtable (`0102e388`) and, when bit 0 of `flags`
/// is set, returns the block to the allocator (slot 8) with the 16-bit size at +4. Returns
/// `this`.
pub fn hkp_constraint_chain_instance_action_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), ACTION_BASE_VTABLE_A);
    e.call(0x00cb_0c00, &args![this.addr() + 0x14]);
    e.mem.set_u32(this.addr(), ACTION_BASE_VTABLE_B);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dd0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstance::entityRemovedCallback` (Xbox PDB); `this` is the instance
/// (ECX), `entity` the entity being removed. With `world` the word at `entity + 8` (an
/// `hkpWorldObject`'s `m_world`, Xbox PDB): raises the world's counter at +0x94, removes the
/// instance's action (+0x44) with `hkpWorld::removeActionImmediately` (`00c97510`), removes the
/// instance from the critical locked island (`00cb3da0(world, this)`) and lowers the counter;
/// when it reaches zero and the byte at +0x9c is clear, the world's pending operations run
/// (`00c91120` when the word at +0x8c is set, then `00c91140` when the word at +0xa4 is 1 and
/// the one at +0x90 is set).
pub fn hkp_constraint_chain_instance_entity_removed_callback(
    e: &mut Engine,
    this: Ptr,
    entity: Ptr,
) {
    let world = e.mem.u32(entity.addr() + 8);
    let counter = e.mem.u32(world + 0x94);
    e.mem.set_u32(world + 0x94, counter.wrapping_add(1));
    let action = e.mem.u32(this.addr() + 0x44);
    e.call(0x00c9_7510, &args![world, action]);
    e.call(0x00cb_3da0, &args![world, this]);
    let counter = e.mem.u32(world + 0x94).wrapping_sub(1);
    e.mem.set_u32(world + 0x94, counter);
    if counter == 0 && e.mem.u8(world + 0x9c) == 0 {
        if e.mem.u32(world + 0x8c) != 0 {
            e.call(0x00c9_1120, &args![world]);
        }
        if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
            e.call(0x00c9_1140, &args![world]);
        }
    }
}

// Translated from 00dd04d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a member of `hkpConstraintChainInstance`, by its unit): appends
/// `entity` to the chain's pointer array (data +0x38, size +0x3c, capacity and flags +0x40,
/// grown by `00c90940(allocator, array, 4)` when full) and takes a reference on it
/// (`00c90510`). While the array's size is below 2 and the base instance's entity slot of that
/// index (+0x14 + 4 * size) is empty, the entity is also stored in that slot, with a reference
/// of its own.
pub fn fn_00dd04d0(e: &mut Engine, this: Ptr, entity: Ptr) {
    let this = this.addr();
    let size = e.mem.i32(this + 0x3c);
    let slot = this
        .wrapping_add((size as u32).wrapping_mul(4))
        .wrapping_add(0x14);
    if size < 2 && e.mem.u32(slot) == 0 {
        e.mem.set_u32(slot, entity.addr());
        e.call(ADD_REFERENCE, &args![entity]);
    }
    let array = this + 0x38;
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(0x00c9_0940, &args![allocator, array, 4u32]);
    }
    let at = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(4));
    e.mem.set_u32(at, entity.addr());
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.call(ADD_REFERENCE, &args![entity]);
}

// Translated from 00dd0530 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the constructor of `hkpConstraintChainInstance`, by its unit and
/// the vtable it sets): runs the base constructor `00cf3f40(this, 1)`, sets the chain vtable
/// (`010d79e8`), empties the pointer array (data +0x38, size +0x3c, capacity and flags +0x40 =
/// `0x80000000`), stores `data` at +0xc with a reference (`00c90510`), clears +0x14 and +0x18
/// and makes the instance's action: a 0x1c-byte block from the allocator (slot 4) with size
/// 0x1c (+4) and reference count 1 (+6), the vtable `010cc2fc`, three zero words at +8, +0xc and
/// +0x10, the `hkStringPtr` at +0x14 (`00cb0a60`), the instance at +0x18 and finally the vtable
/// `010d79c4`. The action is kept at +0x44. Returns `this`.
pub fn fn_00dd0530(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let at = this.addr();
    e.call(0x00cf_3f40, &args![this, 1u32]);
    e.mem.set_u32(at, CHAIN_INSTANCE_VTABLE);
    e.mem.set_u32(at + 0x38, 0);
    e.mem.set_u32(at + 0x3c, 0);
    e.mem.set_u32(at + 0x40, 0x8000_0000);
    e.mem.set_u32(at + 0xc, data.addr());
    e.call(ADD_REFERENCE, &args![data]);
    e.mem.set_u32(at + 0x14, 0);
    e.mem.set_u32(at + 0x18, 0);
    let allocator = memory_allocator(e);
    let action = e.vcall(allocator, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(action + 6, 1);
    e.mem.set_u16(action + 4, 0x1c);
    e.mem.set_u32(action, ACTION_BASE_VTABLE_A);
    e.mem.set_u32(action + 8, 0);
    e.mem.set_u32(action + 0xc, 0);
    e.mem.set_u32(action + 0x10, 0);
    e.call(0x00cb_0a60, &args![action + 0x14]);
    e.mem.set_u32(action + 0x18, at);
    e.mem.set_u32(action, CHAIN_ACTION_VTABLE);
    e.mem.set_u32(at + 0x44, action);
    this
}

// Translated from 00dd05c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintChainInstance::~hkpConstraintChainInstance` (Xbox PDB), `this` in ECX: sets
/// the chain vtable, removes a reference (`00c905b0`) from each pointer of the array at +0x38
/// (size +0x3c) and from the action at +0x44, empties the array (size 0; the storage is
/// returned through the allocator's slot `0x10` with `(capacity & 0x3fffffff) * 4` bytes unless
/// bit 31 of +0x40 is set; data null, capacity `0x80000000`) and finishes in the base
/// destructor `00cf40b0(this)`.
pub fn hkp_constraint_chain_instance_destructor(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at, CHAIN_INSTANCE_VTABLE);
    let mut index = 0;
    while index < e.mem.i32(at + 0x3c) {
        let element = e.mem.u32(
            e.mem
                .u32(at + 0x38)
                .wrapping_add((index as u32).wrapping_mul(4)),
        );
        e.call(REMOVE_REFERENCE, &args![element]);
        index += 1;
    }
    let action = e.mem.u32(at + 0x44);
    e.call(REMOVE_REFERENCE, &args![action]);
    let allocator = memory_allocator(e);
    let flags = e.mem.u32(at + 0x40);
    e.mem.set_u32(at + 0x3c, 0);
    if flags as i32 >= 0 {
        let data = e.mem.u32(at + 0x38);
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(at + 0x38, 0);
    e.mem.set_u32(at + 0x40, 0x8000_0000);
    e.call(0x00cf_40b0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_4eb0, hkp_constraint_chain_instance_action_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(
            0x00dd_0460,
            hkp_constraint_chain_instance_entity_removed_callback(Ptr, Ptr)
        ),
        entry!(0x00dd_04d0, fn_00dd04d0(Ptr, Ptr)),
        entry!(0x00dd_0530, fn_00dd0530(Ptr, Ptr) -> Ptr),
        entry!(0x00dd_05c0, hkp_constraint_chain_instance_destructor(Ptr)),
        entry!(0x00dd_0630, deleting_destructor_00dd0630(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00dd0630_frees_only_when_bit_zero_is_set() {
        check(0x00dd_0630, 0x00dd_05c0);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_SLOT: u32 = 0x0900_0001;
    const FREE_SLOT: u32 = 0x0900_0002;
    const BUFFER_FREE_SLOT: u32 = 0x0900_0003;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// Doubles at `addresses` that log `(address, arguments)`; a memory router whose allocator
    /// hands out zeroed blocks (slot 4) and logs frees (slots 8 and 0x10).
    fn install(e: &mut Engine, addresses: &[u32]) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 8, FREE_SLOT);
        e.mem.set_u32(vtable + 0x10, BUFFER_FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_SLOT, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        for slot in [FREE_SLOT, BUFFER_FREE_SLOT] {
            let sink = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                sink.borrow_mut().push((slot, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn action_destructor_destroys_the_string_and_frees_on_request() {
        let mut e = engine();
        let log = install(&mut e, &[]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x00cb_0c00, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[0] - 0x14)));
            Ret::default()
        });
        let action = e.mem.alloc(0x20);
        e.mem.set_u16(action + 4, 0x1c);
        let got = e.call(0x00cf_4eb0, &args![action, 0xfeu32]).u32();
        assert_eq!(got, action);
        assert_eq!(*seen.borrow(), [(action + 0x14, 0x010c_c2fc)]);
        assert_eq!(e.mem.u32(action), 0x0102_e388);
        assert!(calls_to(&log, FREE_SLOT).is_empty());
        let got = e.call(0x00cf_4eb0, &args![action, 1u32]).u32();
        assert_eq!(got, action);
        assert_eq!(calls_to(&log, FREE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, FREE_SLOT)[0][1..], [action, 0x1c]);
    }

    /// A world with the counters and flags the removal callback reads.
    fn make_world(
        e: &mut Engine,
        counter: u32,
        locked: u8,
        pending_a: u32,
        mode: u32,
        pending_b: u32,
    ) -> u32 {
        let world = e.mem.alloc(0xb0);
        e.mem.set_u32(world + 0x94, counter);
        e.mem.set_u8(world + 0x9c, locked);
        e.mem.set_u32(world + 0x8c, pending_a);
        e.mem.set_u32(world + 0xa4, mode);
        e.mem.set_u32(world + 0x90, pending_b);
        world
    }

    #[test]
    fn entity_removal_runs_the_pending_world_operations_when_the_counter_returns_to_zero() {
        let mut e = engine();
        let log = install(
            &mut e,
            &[0x00c9_7510, 0x00cb_3da0, 0x00c9_1120, 0x00c9_1140],
        );
        let instance = e.mem.alloc(0x50);
        e.mem.set_u32(instance + 0x44, 0xac71_0000);
        let entity = e.mem.alloc(0x10);
        // The counter the world holds before the call, then the lock byte, the first pending
        // word, the mode word, the second pending word, and which calls are expected.
        for (counter, locked, a, mode, b, expect_a, expect_b) in [
            (0, 0, 1, 1, 1, true, true),
            (0, 0, 0, 1, 1, false, true),
            (0, 0, 1, 2, 1, true, false),
            (0, 0, 1, 1, 0, true, false),
            (0, 1, 1, 1, 1, false, false),
            (4, 0, 1, 1, 1, false, false),
        ] {
            log.borrow_mut().clear();
            let world = make_world(&mut e, counter, locked, a, mode, b);
            e.mem.set_u32(entity + 8, world);
            e.call(0x00dd_0460, &args![instance, entity]);
            assert_eq!(calls_to(&log, 0x00c9_7510), [[world, 0xac71_0000]]);
            assert_eq!(calls_to(&log, 0x00cb_3da0), [[world, instance]]);
            assert_eq!(calls_to(&log, 0x00c9_1120).len(), usize::from(expect_a));
            assert_eq!(calls_to(&log, 0x00c9_1140).len(), usize::from(expect_b));
            assert_eq!(e.mem.u32(world + 0x94), counter);
        }
    }

    #[test]
    fn the_counter_is_raised_while_the_instance_is_removed() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(0));
        let sink = Rc::clone(&seen);
        install(&mut e, &[0x00c9_7510, 0x00c9_1120, 0x00c9_1140]);
        e.register_double(0x00cb_3da0, move |e, a| {
            *sink.borrow_mut() = e.mem.u32(a[0] + 0x94);
            Ret::default()
        });
        let instance = e.mem.alloc(0x50);
        let entity = e.mem.alloc(0x10);
        let world = make_world(&mut e, 6, 1, 0, 0, 0);
        e.mem.set_u32(entity + 8, world);
        e.call(0x00dd_0460, &args![instance, entity]);
        assert_eq!(*seen.borrow(), 7);
    }

    fn make_chain(e: &mut Engine, size: u32, capacity_flags: u32) -> u32 {
        let chain = e.mem.alloc(0x50);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(chain + 0x38, data);
        e.mem.set_u32(chain + 0x3c, size);
        e.mem.set_u32(chain + 0x40, capacity_flags);
        chain
    }

    #[test]
    fn adding_an_entity_also_fills_a_free_base_slot_while_the_chain_is_short() {
        let mut e = engine();
        let log = install(&mut e, &[ADD_REFERENCE, 0x00c9_0940]);
        let chain = make_chain(&mut e, 0, 0x8000_0004);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.call(0x00dd_04d0, &args![chain, first]);
        assert_eq!(e.mem.u32(chain + 0x14), first);
        assert_eq!(e.mem.u32(chain + 0x3c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(chain + 0x38)), first);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[first], [first]]);
        assert!(calls_to(&log, 0x00c9_0940).is_empty());
        // Size 1: slot +0x18 is free too.
        log.borrow_mut().clear();
        e.call(0x00dd_04d0, &args![chain, second]);
        assert_eq!(e.mem.u32(chain + 0x18), second);
        assert_eq!(e.mem.u32(chain + 0x3c), 2);
        assert_eq!(calls_to(&log, ADD_REFERENCE).len(), 2);
        // Size 2: only the array.
        log.borrow_mut().clear();
        let third = e.mem.alloc(8);
        e.call(0x00dd_04d0, &args![chain, third]);
        assert_eq!(e.mem.u32(chain + 0x3c), 3);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[third]]);
    }

    #[test]
    fn adding_an_entity_to_a_full_array_asks_for_more_room_and_respects_a_taken_slot() {
        let mut e = engine();
        let log = install(&mut e, &[ADD_REFERENCE, 0x00c9_0940]);
        let chain = make_chain(&mut e, 0, 0x4000_0000);
        // Capacity 0 with size 0: full. A taken base slot is left alone.
        e.mem.set_u32(chain + 0x14, 0x1234);
        let entity = e.mem.alloc(8);
        e.call(0x00dd_04d0, &args![chain, entity]);
        assert_eq!(e.mem.u32(chain + 0x14), 0x1234);
        let grow = calls_to(&log, 0x00c9_0940);
        assert_eq!(grow.len(), 1);
        assert_eq!(grow[0][1..], [chain + 0x38, 4]);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[entity]]);
        assert_eq!(e.mem.u32(chain + 0x3c), 1);
    }

    #[test]
    fn the_constructor_makes_the_action() {
        let mut e = engine();
        let log = install(&mut e, &[0x00cf_3f40, ADD_REFERENCE, 0x00cb_0a60]);
        let chain = e.mem.alloc(0x50);
        e.mem.set_u32(chain + 0x14, 0x77);
        let data = e.mem.alloc(8);
        let got = e.call(0x00dd_0530, &args![chain, data]).u32();
        assert_eq!(got, chain);
        assert_eq!(calls_to(&log, 0x00cf_3f40), [[chain, 1]]);
        assert_eq!(e.mem.u32(chain), 0x010d_79e8);
        assert_eq!(e.mem.u32(chain + 0x38), 0);
        assert_eq!(e.mem.u32(chain + 0x3c), 0);
        assert_eq!(e.mem.u32(chain + 0x40), 0x8000_0000);
        assert_eq!(e.mem.u32(chain + 0xc), data);
        assert_eq!(calls_to(&log, ADD_REFERENCE), [[data]]);
        assert_eq!(e.mem.u32(chain + 0x14), 0);
        assert_eq!(e.mem.u32(chain + 0x18), 0);
        assert_eq!(calls_to(&log, ALLOCATE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, ALLOCATE_SLOT)[0][1], 0x1c);
        let action = e.mem.u32(chain + 0x44);
        assert_eq!(calls_to(&log, 0x00cb_0a60), [[action + 0x14]]);
        assert_eq!(e.mem.u16(action + 4), 0x1c);
        assert_eq!(e.mem.u16(action + 6), 1);
        assert_eq!(e.mem.u32(action), 0x010d_79c4);
        assert_eq!(e.mem.u32(action + 0x18), chain);
        for offset in [8, 0xc, 0x10] {
            assert_eq!(e.mem.u32(action + offset), 0);
        }
    }

    #[test]
    fn the_destructor_releases_the_members_and_frees_owned_storage() {
        let mut e = engine();
        let log = install(&mut e, &[REMOVE_REFERENCE, 0x00cf_40b0]);
        let chain = make_chain(&mut e, 2, 5);
        let data = e.mem.u32(chain + 0x38);
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, 0xa2);
        e.mem.set_u32(chain + 0x44, 0xac);
        e.call(0x00dd_05c0, &args![chain]);
        assert_eq!(e.mem.u32(chain), 0x010d_79e8);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0xa1], [0xa2], [0xac]]);
        assert_eq!(calls_to(&log, BUFFER_FREE_SLOT).len(), 1);
        assert_eq!(calls_to(&log, BUFFER_FREE_SLOT)[0][1..], [data, 20]);
        assert_eq!(e.mem.u32(chain + 0x3c), 0);
        assert_eq!(e.mem.u32(chain + 0x38), 0);
        assert_eq!(e.mem.u32(chain + 0x40), 0x8000_0000);
        assert_eq!(calls_to(&log, 0x00cf_40b0), [[chain]]);
        // Storage that is not owned (bit 31) is not freed.
        log.borrow_mut().clear();
        let borrowed = make_chain(&mut e, 0, 0x8000_0004);
        e.mem.set_u32(borrowed + 0x44, 0xad);
        e.call(0x00dd_05c0, &args![borrowed]);
        assert!(calls_to(&log, BUFFER_FREE_SLOT).is_empty());
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[0xad]]);
    }
}
