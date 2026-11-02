//! `Havok SDK/hkpshapephantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d0_8130`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9eb00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d0_8130, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkpPhantom::updateBroadPhase` (Xbox PDB, `00d07ec0`, `this`, `(aabb)`).
const PHANTOM_UPDATE_BROAD_PHASE: u32 = 0x00d0_7ec0;
/// The `double` `0.5` the collidable's radius is scaled by (`01011588`).
const HALF_DOUBLE: u32 = 0x0101_1588;

/// The shape's box for the phantom's broad-phase update: `half = collidable radius * 0.5`
/// rounded to `float` (the radius is the `float` at +8 of the structure at +0x78 of the
/// collidable at +8).
fn collidable_half_radius(e: &Engine, collidable: u32) -> f32 {
    let owner = e.mem.u32(collidable + 0x78);
    let radius = f64::from(x87_quiet(e.mem.f32(owner + 8)));
    (radius * e.global::<f64>(HALF_DOUBLE)) as f32
}

// Translated from 00c9e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::getMotionState` (Xbox PDB): the motion state embedded at `this + 0xB0`.
pub fn hkp_shape_phantom_get_motion_state(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr() + 0xb0)
}

// Translated from 00c9e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::setTransform` (Xbox PDB): copies the 4x4 transform (64 bytes) to the
/// motion state at +0xB0; when the phantom has a collidable (+8), asks the shape (+0x10,
/// slot `0x1c`) for its box at `(transform, radius * 0.5, out)` and passes it to
/// `hkpPhantom::updateBroadPhase`.
pub fn hkp_shape_phantom_set_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    for offset in (0..0x40).step_by(4) {
        let word = e.mem.u32(transform.addr() + offset);
        e.mem.set_u32(this.addr() + 0xb0 + offset, word);
    }
    let collidable = e.mem.u32(this.addr() + 8);
    if collidable != 0 {
        let tolerance = x87_quiet(collidable_half_radius(e, collidable));
        let shape = e.mem.u32(this.addr() + 0x10);
        e.with_stack(0x30, |e, aabb| {
            e.vcall(shape, 0x1c, &args![transform, tolerance, aabb]);
            e.call(PHANTOM_UPDATE_BROAD_PHASE, &args![this, aabb]);
        });
    }
}

// Translated from 00c9e990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::setPosition` (Xbox PDB): copies the position vector (16 bytes) to the
/// translation column of the motion state (+0xE0); when the phantom has a collidable (+8),
/// asks the shape (+0x10, slot `0x1c`) for its box at `(motion state, radius * 0.5 +
/// tolerance, out)` (each step rounded to `float`) and passes it to
/// `hkpPhantom::updateBroadPhase`.
pub fn hkp_shape_phantom_set_position(e: &mut Engine, this: Ptr, position: Ptr, tolerance: f32) {
    for offset in (0..0x10).step_by(4) {
        let word = e.mem.u32(position.addr() + offset);
        e.mem.set_u32(this.addr() + 0xe0 + offset, word);
    }
    let collidable = e.mem.u32(this.addr() + 8);
    if collidable != 0 {
        let half = x87_quiet(collidable_half_radius(e, collidable));
        let total = x87_quiet((f64::from(half) + f64::from(x87_quiet(tolerance))) as f32);
        let shape = e.mem.u32(this.addr() + 0x10);
        let motion_state = this.addr() + 0xb0;
        e.with_stack(0x30, |e, aabb| {
            e.vcall(shape, 0x1c, &args![motion_state, total, aabb]);
            e.call(PHANTOM_UPDATE_BROAD_PHASE, &args![this, aabb]);
        });
    }
}

/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The `hkpWorld` function (`00c91160`, `hkpworld.cpp`) that stores a pending-operation
/// record while the world is locked.
const WORLD_QUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpWorldOperationUtil::removePhantomBP` (`00cb5d00`) and `addPhantomBP` (`00cb5b10`),
/// `hkpWorldCallbackUtil::firePhantomShapeSet` (`00d008c0`) and
/// `hkpPhantom::firePhantomShapeSet` (`00d07bd0`).
const REMOVE_PHANTOM_BP: u32 = 0x00cb_5d00;
const ADD_PHANTOM_BP: u32 = 0x00cb_5b10;
const FIRE_WORLD_PHANTOM_SHAPE_SET: u32 = 0x00d0_08c0;
const FIRE_PHANTOM_SHAPE_SET: u32 = 0x00d0_7bd0;
/// The two `hkpWorld` functions (`00c91120`, `00c91140`, `hkpworld.cpp`) `setShape` calls when
/// the world's lock count falls back to zero.
const WORLD_UNLOCK_FIRST: u32 = 0x00c9_1120;
const WORLD_UNLOCK_SECOND: u32 = 0x00c9_1140;
/// `hkpPhantom::deallocateInternalArrays` (Xbox PDB, `00d07e20`).
const PHANTOM_DEALLOCATE_INTERNAL_ARRAYS: u32 = 0x00d0_7e20;
/// `hkpWorldObject::hkpWorldObject` (Xbox PDB, `00cd2ce0`), `hkQuaternion::set` (`00cb26e0`)
/// and the motion-state constructor `00cf2410`.
const WORLD_OBJECT_CONSTRUCT: u32 = 0x00cd_2ce0;
const QUATERNION_SET: u32 = 0x00cb_26e0;
const MOTION_STATE_CONSTRUCT: u32 = 0x00cf_2410;

// Translated from 00c9e820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::deallocateInternalArrays` (Xbox PDB): a jump to
/// `hkpPhantom::deallocateInternalArrays` (`00d07e20`) with the same `this`.
pub fn hkp_shape_phantom_deallocate_internal_arrays(e: &mut Engine, this: Ptr) {
    e.call(PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, &args![this]);
}

// Translated from 00c9e830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::setShape` (Xbox PDB): with a world at +8 whose lock count (+0x94) is not
/// zero, queues a record `{5, this, shape}` through `00c91160` and returns `false`.
/// Otherwise locks the world (+0x94 incremented) and removes the phantom from the broad phase,
/// takes a reference on `shape`, releases the old one (+0x10), stores the new shape, fires
/// the world's and the phantom's shape-set callbacks, adds the phantom back to the broad
/// phase and decrements the lock count; at zero, with the byte at +0x9c clear, it calls
/// `00c91120` when +0x8c is set and `00c91140` when +0xa4 is 1 and +0x90 is set. Returns
/// `true`.
pub fn hkp_shape_phantom_set_shape(e: &mut Engine, this: Ptr, shape: u32) -> bool {
    let base = this.addr();
    let world = e.mem.u32(base + 8);
    if world != 0 {
        if e.mem.u32(world + 0x94) != 0 {
            e.with_stack(0xc, |e, operation| {
                e.mem.set_u8(operation.addr(), 5);
                e.mem.set_u32(operation.addr() + 4, base);
                e.mem.set_u32(operation.addr() + 8, shape);
                e.call(WORLD_QUEUE_OPERATION, &args![world, operation]);
            });
            return false;
        }
        // The exe tests the world pointer a second time here; it is still non-zero.
        let locks = e.mem.u32(world + 0x94).wrapping_add(1);
        e.mem.set_u32(world + 0x94, locks);
        let world = e.mem.u32(base + 8);
        e.call(REMOVE_PHANTOM_BP, &args![world, this]);
    }
    e.call(ADD_REFERENCE, &args![shape]);
    let old_shape = e.mem.u32(base + 0x10);
    if old_shape != 0 {
        e.call(REMOVE_REFERENCE, &args![old_shape]);
    }
    let world = e.mem.u32(base + 8);
    e.mem.set_u32(base + 0x10, shape);
    if world != 0 {
        e.call(FIRE_WORLD_PHANTOM_SHAPE_SET, &args![world, this]);
    }
    e.call(FIRE_PHANTOM_SHAPE_SET, &args![this]);
    let world = e.mem.u32(base + 8);
    if world != 0 {
        e.call(ADD_PHANTOM_BP, &args![world, this]);
        let world = e.mem.u32(base + 8);
        let locks = e.mem.u32(world + 0x94).wrapping_sub(1);
        e.mem.set_u32(world + 0x94, locks);
        if locks == 0 && e.mem.u8(world + 0x9c) == 0 {
            if e.mem.u32(world + 0x8c) != 0 {
                e.call(WORLD_UNLOCK_FIRST, &args![world]);
            }
            if e.mem.u32(world + 0xa4) == 1 && e.mem.u32(world + 0x90) != 0 {
                e.call(WORLD_UNLOCK_SECOND, &args![world]);
            }
        }
    }
    true
}

// Translated from 00c9ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapePhantom::calcAabb` (Xbox PDB): asks the shape (+0x10, slot `0x1c`) for its box at
/// `(motion state at this + 0xB0, radius * 0.5, out)`, the radius being the collidable's
/// (+8) as in `setTransform`.
pub fn hkp_shape_phantom_calc_aabb(e: &mut Engine, this: Ptr, out: Ptr) {
    let collidable = e.mem.u32(this.addr() + 8);
    let tolerance = x87_quiet(collidable_half_radius(e, collidable));
    let shape = e.mem.u32(this.addr() + 0x10);
    e.vcall(shape, 0x1c, &args![this.addr() + 0xb0, tolerance, out]);
}

// Translated from 00c9ea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `hkpShapePhantom`-family base constructor (class name unconfirmed): runs
/// `hkpWorldObject::hkpWorldObject(this, argument, 2)`, sets the vtable `0x010c_7214`, clears
/// the arrays at +0x8c and +0x98 (data, size 0, capacity word `0x8000_0000`) and stores at
/// +0x20 the low byte of `this - (this + 0x10)` (always `0xf0`). Returns `this`.
pub fn fn_00c9ea90(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    let base = this.addr();
    e.call(WORLD_OBJECT_CONSTRUCT, &args![this, argument, 2u32]);
    e.mem.set_u32(base, 0x010c_7214);
    e.mem.set_u32(base + 0x8c, 0);
    e.mem.set_u32(base + 0x90, 0);
    e.mem.set_u32(base + 0x94, 0x8000_0000);
    e.mem.set_u32(base + 0x98, 0);
    e.mem.set_u32(base + 0x9c, 0);
    e.mem.set_u32(base + 0xa0, 0x8000_0000);
    e.mem
        .set_u8(base + 0x20, (base as u8).wrapping_sub((base + 0x10) as u8));
    this
}

// Translated from 00c9ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor derived from `00c9ea90` (class name unconfirmed): runs it with `argument`,
/// sets the vtable `0x010c_7254`, builds a quaternion from `rotation` (`hkQuaternion::set`),
/// constructs the motion state at +0xB0 from `rotation + 0x30` and that quaternion
/// (`00cf2410`) and stores the motion state's address at +0x18. Returns `this`.
pub fn fn_00c9ea40(e: &mut Engine, this: Ptr, argument: u32, rotation: Ptr) -> Ptr {
    let base = this.addr();
    fn_00c9ea90(e, this, argument);
    e.mem.set_u32(base, 0x010c_7254);
    e.with_stack(0x10, |e, quaternion| {
        e.call(QUATERNION_SET, &args![quaternion, rotation]);
        e.call(
            MOTION_STATE_CONSTRUCT,
            &args![base + 0xb0, rotation.addr() + 0x30, quaternion],
        );
    });
    e.mem.set_u32(base + 0x18, base + 0xb0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_e810, hkp_shape_phantom_get_motion_state(Ptr) -> Ptr),
        entry!(0x00c9_e910, hkp_shape_phantom_set_transform(Ptr, Ptr)),
        entry!(0x00c9_e990, hkp_shape_phantom_set_position(Ptr, Ptr, f32)),
        entry!(
            0x00c9_e820,
            hkp_shape_phantom_deallocate_internal_arrays(Ptr)
        ),
        entry!(0x00c9_e830, hkp_shape_phantom_set_shape(Ptr, u32) -> bool),
        entry!(0x00c9_ea00, hkp_shape_phantom_calc_aabb(Ptr, Ptr)),
        entry!(0x00c9_ea40, fn_00c9ea40(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00c9_ea90, fn_00c9ea90(Ptr, u32) -> Ptr),
        entry!(0x00c9_eb00, deleting_destructor_00c9eb00(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00c9eb00_frees_only_when_bit_zero_is_set() {
        check(0x00c9_eb00, 0x00d0_8130);
    }

    const SHAPE_VTABLE: u32 = 0x0a00_0000;
    const SHAPE_AABB: u32 = 0x0a10_0001;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A phantom with a collidable whose radius structure holds `radius`, a shape whose
    /// `getAabb` (slot 0x1c) and the broad-phase update log their arguments.
    fn phantom(e: &mut Engine, radius: f32) -> (u32, CallLog) {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        let mut slots = vec![0u32; 8];
        slots[0x1c / 4] = SHAPE_AABB;
        e.put_vtable(SHAPE_VTABLE, &slots);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, SHAPE_VTABLE);
        let owner = e.mem.alloc(0x20);
        e.mem.set_f32(owner + 8, radius);
        let collidable = e.mem.alloc(0x80);
        e.mem.set_u32(collidable + 0x78, owner);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, collidable);
        e.mem.set_u32(this + 0x10, shape);
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let aabb_log = Rc::clone(&log);
        e.register_double(SHAPE_AABB, move |_, a| {
            aabb_log.borrow_mut().push(vec![1, a[0], a[1], a[2]]);
            Ret::default()
        });
        let update_log = Rc::clone(&log);
        e.register_double(PHANTOM_UPDATE_BROAD_PHASE, move |_, a| {
            update_log.borrow_mut().push(vec![2, a[0], a[1]]);
            Ret::default()
        });
        (this, log)
    }

    #[test]
    fn get_motion_state_is_this_plus_0xb0() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c9_e810, &args![0x1000u32]).u32(), 0x10b0);
    }

    #[test]
    fn set_transform_copies_the_transform_and_updates_the_broad_phase() {
        let mut e = Engine::new();
        let (this, log) = phantom(&mut e, 3.0);
        let transform = e.mem.alloc(0x40);
        for offset in (0..0x40).step_by(4) {
            e.mem.set_u32(transform + offset, 0x100 + offset);
        }
        e.call(0x00c9_e910, &args![this, transform]);
        for offset in (0..0x40).step_by(4) {
            assert_eq!(e.mem.u32(this + 0xb0 + offset), 0x100 + offset);
        }
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        // getAabb(this = shape, transform, radius * 0.5, out)
        let shape = e.mem.u32(this + 0x10);
        assert_eq!(log[0][..3], [1, shape, transform]);
        assert_eq!(f32::from_bits(log[0][3]), 1.5);
        assert_eq!(log[1][..2], [2, this]);
    }

    #[test]
    fn set_position_adds_the_tolerance_and_skips_the_update_without_a_collidable() {
        let mut e = Engine::new();
        let (this, log) = phantom(&mut e, 3.0);
        let position = e.mem.alloc(0x10);
        for offset in (0..0x10).step_by(4) {
            e.mem.set_u32(position + offset, 0x200 + offset);
        }
        e.call(0x00c9_e990, &args![this, position, 0.25f32]);
        for offset in (0..0x10).step_by(4) {
            assert_eq!(e.mem.u32(this + 0xe0 + offset), 0x200 + offset);
        }
        {
            let log = log.borrow();
            assert_eq!(log.len(), 2);
            // getAabb(this = shape, motion state, radius * 0.5 + tolerance, out)
            assert_eq!(log[0][2], this + 0xb0);
            assert_eq!(f32::from_bits(log[0][3]), 1.75);
            assert_eq!(log[1][..2], [2, this]);
        }
        log.borrow_mut().clear();
        e.mem.set_u32(this + 8, 0);
        e.mem.set_u32(position, 0x300);
        e.call(0x00c9_e990, &args![this, position, 0.25f32]);
        assert_eq!(e.mem.u32(this + 0xe0), 0x300);
        assert!(log.borrow().is_empty());
    }

    /// A double at `address` that logs `[address, arguments...]`.
    fn log_calls(e: &mut Engine, address: u32, log: &CallLog) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret::default()
        });
    }

    #[test]
    fn deallocate_internal_arrays_jumps_to_the_phantom_version() {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        log_calls(&mut e, PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, &log);
        e.call(0x00c9_e820, &args![0x1234u32]);
        assert_eq!(*log.borrow(), vec![vec![0x00d0_7e20, 0x1234]]);
    }

    /// A phantom with a world at +8 and an old shape at +0x10, every callee of `setShape`
    /// logging its arguments.
    fn shape_setting(e: &mut Engine) -> (u32, u32, CallLog) {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        for address in [
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            WORLD_QUEUE_OPERATION,
            REMOVE_PHANTOM_BP,
            ADD_PHANTOM_BP,
            FIRE_WORLD_PHANTOM_SHAPE_SET,
            FIRE_PHANTOM_SHAPE_SET,
            WORLD_UNLOCK_FIRST,
            WORLD_UNLOCK_SECOND,
        ] {
            log_calls(e, address, &log);
        }
        let world = e.mem.alloc(0x100);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, world);
        e.mem.set_u32(this + 0x10, 0x4444);
        (this, world, log)
    }

    #[test]
    fn set_shape_queues_an_operation_while_the_world_is_locked() {
        let mut e = Engine::new();
        let (this, world, log) = shape_setting(&mut e);
        e.mem.set_u32(world + 0x94, 2);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let seen_in_double = Rc::clone(&seen);
        e.register_double(WORLD_QUEUE_OPERATION, move |e, a| {
            seen_in_double.borrow_mut().push((
                a[0],
                e.mem.u8(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[1] + 8),
            ));
            Ret::default()
        });
        let done = e.call(0x00c9_e830, &args![this, 0x5555u32]).u32();
        assert_eq!(done, 0);
        assert_eq!(*seen.borrow(), vec![(world, 5, this, 0x5555)]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(world + 0x94), 2);
        assert_eq!(e.mem.u32(this + 0x10), 0x4444);
    }

    #[test]
    fn set_shape_swaps_the_shape_and_unlocks_the_world() {
        let mut e = Engine::new();
        let (this, world, log) = shape_setting(&mut e);
        e.mem.set_u32(world + 0x8c, 1);
        e.mem.set_u32(world + 0xa4, 1);
        e.mem.set_u32(world + 0x90, 7);
        let done = e.call(0x00c9_e830, &args![this, 0x5555u32]).u32();
        assert_eq!(done, 1);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![REMOVE_PHANTOM_BP, world, this],
                vec![ADD_REFERENCE, 0x5555],
                vec![REMOVE_REFERENCE, 0x4444],
                vec![FIRE_WORLD_PHANTOM_SHAPE_SET, world, this],
                vec![FIRE_PHANTOM_SHAPE_SET, this],
                vec![ADD_PHANTOM_BP, world, this],
                vec![WORLD_UNLOCK_FIRST, world],
                vec![WORLD_UNLOCK_SECOND, world],
            ]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x5555);
        assert_eq!(e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn set_shape_skips_the_unlock_calls_when_their_conditions_fail() {
        let mut e = Engine::new();
        let (this, world, log) = shape_setting(&mut e);
        e.mem.set_u32(this + 0x10, 0);
        // The world's byte at +0x9c is set: nothing is called after the broad-phase add.
        e.mem.set_u8(world + 0x9c, 1);
        e.mem.set_u32(world + 0x8c, 1);
        e.call(0x00c9_e830, &args![this, 0x5555u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|entry| entry[0]).collect();
        assert_eq!(
            addresses,
            vec![
                REMOVE_PHANTOM_BP,
                ADD_REFERENCE,
                FIRE_WORLD_PHANTOM_SHAPE_SET,
                FIRE_PHANTOM_SHAPE_SET,
                ADD_PHANTOM_BP
            ]
        );
        // Byte clear, +0x8c clear, +0xa4 not 1: still nothing.
        log.borrow_mut().clear();
        e.mem.set_u8(world + 0x9c, 0);
        e.mem.set_u32(world + 0x8c, 0);
        e.mem.set_u32(world + 0xa4, 2);
        e.mem.set_u32(world + 0x90, 7);
        e.call(0x00c9_e830, &args![this, 0x6666u32]);
        let addresses: Vec<u32> = log.borrow().iter().map(|entry| entry[0]).collect();
        assert_eq!(
            addresses,
            vec![
                REMOVE_PHANTOM_BP,
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                FIRE_WORLD_PHANTOM_SHAPE_SET,
                FIRE_PHANTOM_SHAPE_SET,
                ADD_PHANTOM_BP
            ]
        );
        // Without a world only the reference and phantom callbacks run.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 8, 0);
        let done = e.call(0x00c9_e830, &args![this, 0x7777u32]).u32();
        assert_eq!(done, 1);
        let addresses: Vec<u32> = log.borrow().iter().map(|entry| entry[0]).collect();
        assert_eq!(
            addresses,
            vec![ADD_REFERENCE, REMOVE_REFERENCE, FIRE_PHANTOM_SHAPE_SET]
        );
    }

    #[test]
    fn calc_aabb_asks_the_shape_for_its_box_at_the_motion_state() {
        let mut e = Engine::new();
        let (this, log) = phantom(&mut e, 3.0);
        let seen = Rc::clone(&log);
        e.register_double(SHAPE_AABB, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00c9_ea00, &args![this, 0x9000u32]);
        let shape = e.mem.u32(this + 0x10);
        assert_eq!(log.borrow().len(), 1);
        let call = log.borrow()[0].clone();
        assert_eq!(call[..2], [shape, this + 0xb0]);
        assert_eq!(f32::from_bits(call[2]), 1.5);
        assert_eq!(call[3], 0x9000);
    }

    #[test]
    fn world_object_style_constructors_fill_the_phantom() {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        for address in [
            WORLD_OBJECT_CONSTRUCT,
            QUATERNION_SET,
            MOTION_STATE_CONSTRUCT,
        ] {
            log_calls(&mut e, address, &log);
        }
        let this = e.mem.alloc(0x100);
        let got = e.call(0x00c9_ea90, &args![this, 0x1111u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![vec![WORLD_OBJECT_CONSTRUCT, this, 0x1111, 2]]
        );
        assert_eq!(e.mem.u32(this), 0x010c_7214);
        assert_eq!(e.mem.u32(this + 0x94), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0xa0), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x8c), 0);
        assert_eq!(e.mem.u8(this + 0x20), 0xf0);
        log.borrow_mut().clear();
        let rotation = e.mem.alloc(0x40);
        let got = e.call(0x00c9_ea40, &args![this, 0x2222u32, rotation]).u32();
        assert_eq!(got, this);
        let log = log.borrow();
        assert_eq!(log[0], vec![WORLD_OBJECT_CONSTRUCT, this, 0x2222, 2]);
        assert_eq!(log[1][..1], [QUATERNION_SET]);
        let quaternion = log[1][1];
        assert_eq!(log[1][2], rotation);
        assert_eq!(
            log[2],
            vec![
                MOTION_STATE_CONSTRUCT,
                this + 0xb0,
                rotation + 0x30,
                quaternion
            ]
        );
        assert_eq!(e.mem.u32(this), 0x010c_7254);
        assert_eq!(e.mem.u32(this + 0x18), this + 0xb0);
    }
}
