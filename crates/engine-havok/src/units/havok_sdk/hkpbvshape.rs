//! `Havok SDK/hkpbvshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use super::hkpsphereshape::{profile_record, PROFILE_END};
#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00c9db30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00c9_d930`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00c9db30(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00c9_d930, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkReferencedObject` reference count increment (`00c90510`) and release (`00c905b0`), both
/// with `this` in `ECX` and nothing on the stack.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkReferencedObject::calcContentStatistics` (`this` in `ECX`, collector, class).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The `hkClass` `hkpBvShape::calcContentStatistics` passes to it, and the string the
/// collector's slot `0xC` is given first.
const BV_SHAPE_CLASS: u32 = 0x0126_8910;
const CONTAINER_LABEL: u32 = 0x010c_6fec;
/// Vtables: the shape container sub-object (`+0x14` of the shape) while it is alive and
/// after it is destroyed, the shape itself while it is alive and once its destructor is done.
const CONTAINER_VTABLE: u32 = 0x010c_6fd0;
const CONTAINER_VTABLE_DESTROYED: u32 = 0x010c_6fb0;
const BV_SHAPE_VTABLE: u32 = 0x010c_6ff8;
const BASE_VTABLE: u32 = 0x0102_e388;
/// The shape type the constructor stores in `+0xC`.
const BV_SHAPE_TYPE: u32 = 0x1b;
/// The profiling label `"TtrcBvShape"`.
const PROFILE_BEGIN_BV_SHAPE: u32 = 0x010c_7024;
/// `hkpShapeRayCastOutput::m_shapeKeyIndex` (`+0x40`) and the start of its `m_shapeKeys`
/// (`+0x20`).
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;
const OUT_SHAPE_KEYS: u32 = 0x20;

// Translated from 00c9d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSingleShapeContainer`'s scalar deleting destructor (engine map): `ECX` = container,
/// one stack word (the flags, which the code never reads and which free nothing). Sets the
/// live vtable `010c6fd0`, releases the shape at `+4` when there is one, then sets the
/// destroyed vtable `010c6fb0`. Returns `this`.
pub fn hkp_single_shape_container_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _flags: u32,
) -> Ptr {
    let shape = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), CONTAINER_VTABLE);
    if shape != 0 {
        e.call(REMOVE_REFERENCE, &args![shape]);
    }
    e.mem.set_u32(this.addr(), CONTAINER_VTABLE_DESTROYED);
    this
}

// Translated from 00c9d8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvShape::calcContentStatistics` (engine map): `ECX` = shape, the collector and one
/// more word (never read) on the stack. Calls the collector's slot `0xC` with the label at
/// `010c6fec`, the shape at `+0x18` and 0, then `hkReferencedObject::calcContentStatistics`
/// with the class at `01268910`.
pub fn hkp_bv_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_1: u32,
) {
    let child = e.mem.u32(this.addr() + 0x18);
    e.vcall(collector, 0xc, &args![CONTAINER_LABEL, child, 0u32]);
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, BV_SHAPE_CLASS],
    );
}

// Translated from 00c9d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvShape::getContainer` (engine map): the shape container sub-object, `this + 0x14`.
pub fn hkp_bv_shape_get_container(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr() + 0x14
}

// Translated from 00c9d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the bounding-volume shape (no engine-map name; `ECX` = shape): sets the
/// vtable `010c6ff8`, releases the shape at `+0x10` (the code does not test it for null),
/// sets the container vtable at `+0x14` to `010c6fd0`, releases the shape at `+0x18` when
/// there is one, then sets the container vtable `010c6fb0` and the shape vtable `0102e388`.
pub fn fn_00c9d930(e: &mut Engine, this: Ptr) {
    let shape = this.addr();
    let bounding = e.mem.u32(shape + 0x10);
    e.mem.set_u32(shape, BV_SHAPE_VTABLE);
    e.call(REMOVE_REFERENCE, &args![bounding]);
    let child = e.mem.u32(shape + 0x18);
    e.mem.set_u32(shape + 0x14, CONTAINER_VTABLE);
    if child != 0 {
        e.call(REMOVE_REFERENCE, &args![child]);
    }
    e.mem.set_u32(shape + 0x14, CONTAINER_VTABLE_DESTROYED);
    e.mem.set_u32(shape, BASE_VTABLE);
}

// Translated from 00c9d970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvShape::getAabbImpl` (engine map): `ECX` = shape, three stack words (the transform,
/// the tolerance as a `float` loaded and stored on the x87 stack, the output). Calls the
/// bounding shape's (the one at `+0x10`) slot `0x1C` with them.
pub fn hkp_bv_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: u32,
    tolerance: f32,
    out: u32,
) {
    let bounding = e.mem.u32(this.addr() + 0x10);
    e.vcall(bounding, 0x1c, &args![transform, x87_quiet(tolerance), out]);
}

// Translated from 00c9d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the bounding-volume shape (no engine-map name; `ECX` = shape, the
/// bounding shape and the contained shape on the stack): stores the bounding shape at
/// `+0x10`, the reference count word `+6` = 1, user data 0, type `0x1b`, the vtable, the
/// container vtable `010c6fd0` at `+0x14` and the contained shape at `+0x18`, then takes
/// a reference on each (the contained shape first). Returns `this`.
pub fn fn_00c9d990(e: &mut Engine, this: Ptr, bounding: u32, child: u32) -> Ptr {
    let shape = this.addr();
    e.mem.set_u32(shape + 0x10, bounding);
    e.mem.set_u16(shape + 6, 1);
    e.mem.set_u32(shape + 8, 0);
    e.mem.set_u32(shape + 0xc, BV_SHAPE_TYPE);
    e.mem.set_u32(shape, BV_SHAPE_VTABLE);
    e.mem.set_u32(shape + 0x14, CONTAINER_VTABLE);
    e.mem.set_u32(shape + 0x18, child);
    e.call(ADD_REFERENCE, &args![child]);
    e.call(ADD_REFERENCE, &args![bounding]);
    this
}

// Translated from 00c9d9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvShape::castRayImpl` (engine map): `ECX` = shape, three stack words (the hit flag
/// byte pointer, the input, the output). Between profiling records `"TtrcBvShape"` and `"Et"`
/// it counts a level in the output (`+0x40`), calls the contained shape's (`+0x18`) slot
/// `0x20` with the three words, drops the level and, when the flag byte is set, clears the
/// shape key at the new level (`+0x20 + level * 4`). Returns the flag byte pointer.
pub fn hkp_bv_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    hit: Ptr,
    input: u32,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_BV_SHAPE);
    let output_address = output.addr();
    let level = e
        .mem
        .u32(output_address + OUT_SHAPE_KEY_INDEX)
        .wrapping_add(1);
    e.mem.set_u32(output_address + OUT_SHAPE_KEY_INDEX, level);
    let child = e.mem.u32(this.addr() + 0x18);
    e.vcall(child, 0x20, &args![hit, input, output]);
    let level = e
        .mem
        .u32(output_address + OUT_SHAPE_KEY_INDEX)
        .wrapping_sub(1);
    e.mem.set_u32(output_address + OUT_SHAPE_KEY_INDEX, level);
    if e.mem.u8(hit.addr()) != 0 {
        let key = output_address
            .wrapping_add(OUT_SHAPE_KEYS)
            .wrapping_add(level.wrapping_mul(4));
        e.mem.set_u32(key, 0);
    }
    profile_record(e, PROFILE_END);
    hit
}

// Translated from 00c9da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBvShape::castRayWithCollectorImpl` (engine map): `ECX` = shape, three stack words
/// (`result`, `collector`, `third`). Between profiling records `"TtrcBvShape"` and `"Et"` it
/// calls the contained shape's (`+0x18`) slot `0x24` with `result`, the address of a 16-byte
/// record on the stack (the contained shape, 0, the word at `+8` of `collector`, `collector`)
/// and `third`.
pub fn hkp_bv_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr,
    result: u32,
    collector: u32,
    third: u32,
) {
    profile_record(e, PROFILE_BEGIN_BV_SHAPE);
    let child = e.mem.u32(this.addr() + 0x18);
    let collector_word = e.mem.u32(collector + 8);
    e.with_stack(0x10, |e, record| {
        e.mem.set_u32(record.addr(), child);
        e.mem.set_u32(record.addr() + 4, 0);
        e.mem.set_u32(record.addr() + 8, collector_word);
        e.mem.set_u32(record.addr() + 0xc, collector);
        e.vcall(child, 0x24, &args![result, record, third]);
    });
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_d8c0, hkp_single_shape_container_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c9_d8f0,
            hkp_bv_shape_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(0x00c9_d920, hkp_bv_shape_get_container(Ptr) -> u32),
        entry!(0x00c9_d930, fn_00c9d930(Ptr)),
        entry!(0x00c9_d970, hkp_bv_shape_get_aabb_impl(Ptr, u32, f32, u32)),
        entry!(0x00c9_d990, fn_00c9d990(Ptr, u32, u32) -> Ptr),
        entry!(0x00c9_d9e0, hkp_bv_shape_cast_ray_impl(Ptr, Ptr, u32, Ptr) -> Ptr),
        entry!(
            0x00c9_da80,
            hkp_bv_shape_cast_ray_with_collector_impl(Ptr, u32, u32, u32)
        ),
        entry!(0x00c9_db30, deleting_destructor_00c9db30(Ptr, u8) -> Ptr),
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
    fn deleting_destructor_00c9db30_frees_only_when_bit_zero_is_set() {
        check(0x00c9_db30, 0x00c9_d930);
    }
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_AABB: u32 = 0x0900_0101;
    const SLOT_CAST_RAY: u32 = 0x0900_0102;
    const SLOT_COLLECTOR: u32 = 0x0900_0103;
    const SLOT_STATISTICS: u32 = 0x0900_0104;

    /// An engine with the exe's data range mapped and a profiling stream (room for 4
    /// records, cursor at `+4`, limit at `+0xC`).
    fn setup() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        let records = e.mem.alloc(0x30);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + 0x30);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, log, stream)
    }

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

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32) -> Vec<u32> {
        let first = e.mem.u32(stream + 0xc) - 0x30;
        let cursor = e.mem.u32(stream + 4);
        (first..cursor).step_by(12).map(|a| e.mem.u32(a)).collect()
    }

    /// An object whose vtable (at `+0`) holds `slots` at the given byte offsets.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for &(offset, target) in slots {
            e.mem.set_u32(table + offset, target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn container_destructor_releases_the_shape_under_the_live_vtable() {
        let (mut e, _, _) = setup();
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let sink = Rc::clone(&seen);
        let container = e.mem.alloc(0x10);
        e.register_double(REMOVE_REFERENCE, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(container)));
            Ret::default()
        });
        e.mem.set_u32(container + 4, 0x4444);
        let got = e.call(0x00c9_d8c0, &args![container, 1u32]).u32();
        assert_eq!(got, container);
        assert_eq!(*seen.borrow(), vec![(0x4444, 0x010c_6fd0)]);
        assert_eq!(e.mem.u32(container), 0x010c_6fb0);
    }

    #[test]
    fn container_destructor_without_a_shape_releases_nothing() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[REMOVE_REFERENCE]);
        let container = e.mem.alloc(0x10);
        e.call(0x00c9_d8c0, &args![container, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(container), 0x010c_6fb0);
    }

    #[test]
    fn calc_content_statistics_reports_the_child_then_the_base() {
        let (mut e, log, _) = setup();
        log_callees(
            &mut e,
            &log,
            &[SLOT_STATISTICS, REFERENCED_OBJECT_CALC_CONTENT_STATISTICS],
        );
        let collector = object_with_slots(&mut e, &[(0xc, SLOT_STATISTICS)]);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x18, 0x5555);
        e.call(0x00c9_d8f0, &args![shape, collector, 0x9999u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (SLOT_STATISTICS, vec![collector, 0x010c_6fec, 0x5555, 0]),
                (
                    REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                    vec![shape, collector, 0x0126_8910]
                ),
            ]
        );
    }

    #[test]
    fn get_container_is_the_sub_object_at_0x14() {
        let (mut e, _, _) = setup();
        assert_eq!(e.call(0x00c9_d920, &args![0x1000u32]).u32(), 0x1014);
    }

    #[test]
    fn destructor_releases_both_shapes_and_resets_the_vtables() {
        let (mut e, _, _) = setup();
        let seen: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::default();
        let sink = Rc::clone(&seen);
        let shape = e.mem.alloc(0x20);
        e.register_double(REMOVE_REFERENCE, move |e, a| {
            sink.borrow_mut()
                .push((a[0], e.mem.u32(shape), e.mem.u32(shape + 0x14)));
            Ret::default()
        });
        e.mem.set_u32(shape + 0x10, 0x3333);
        e.mem.set_u32(shape + 0x18, 0x4444);
        e.call(0x00c9_d930, &args![shape]);
        assert_eq!(
            *seen.borrow(),
            vec![(0x3333, 0x010c_6ff8, 0), (0x4444, 0x010c_6ff8, 0x010c_6fd0)]
        );
        assert_eq!(e.mem.u32(shape), 0x0102_e388);
        assert_eq!(e.mem.u32(shape + 0x14), 0x010c_6fb0);
    }

    #[test]
    fn destructor_skips_a_missing_contained_shape_but_not_a_missing_bounding_shape() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[REMOVE_REFERENCE]);
        let shape = e.mem.alloc(0x20);
        e.call(0x00c9_d930, &args![shape]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), vec![vec![0]]);
    }

    #[test]
    fn get_aabb_forwards_to_the_bounding_shape_with_a_quiet_tolerance() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[SLOT_AABB]);
        let bounding = object_with_slots(&mut e, &[(0x1c, SLOT_AABB)]);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x10, bounding);
        e.call(0x00c9_d970, &args![shape, 0x1111u32, 0.5f32, 0x2222u32]);
        e.call(
            0x00c9_d970,
            &args![shape, 0x1111u32, f32::from_bits(0x7f80_0001), 0x2222u32],
        );
        assert_eq!(
            calls_to(&log, SLOT_AABB),
            vec![
                vec![bounding, 0x1111, 0.5f32.to_bits(), 0x2222],
                vec![bounding, 0x1111, 0x7fc0_0001, 0x2222],
            ]
        );
    }

    #[test]
    fn construct_fills_the_shape_and_takes_both_references() {
        let (mut e, log, _) = setup();
        log_callees(&mut e, &log, &[ADD_REFERENCE]);
        let shape = e.mem.alloc(0x20);
        let got = e
            .call(0x00c9_d990, &args![shape, 0x3333u32, 0x4444u32])
            .u32();
        assert_eq!(got, shape);
        assert_eq!(e.mem.u32(shape + 0x10), 0x3333);
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.u32(shape + 8), 0);
        assert_eq!(e.mem.u32(shape + 0xc), 0x1b);
        assert_eq!(e.mem.u32(shape), 0x010c_6ff8);
        assert_eq!(e.mem.u32(shape + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(shape + 0x18), 0x4444);
        assert_eq!(
            *log.borrow(),
            vec![(ADD_REFERENCE, vec![0x4444]), (ADD_REFERENCE, vec![0x3333])]
        );
    }

    /// A shape whose contained shape's slot `0x20` sets the hit byte to `hit` and logs the
    /// level counter it sees.
    fn ray_shape(e: &mut Engine, log: &Log, hit: u8) -> (u32, u32, u32) {
        let sink = Rc::clone(log);
        e.register_double(SLOT_CAST_RAY, move |e, a| {
            let level = e.mem.u32(a[3] + 0x40);
            sink.borrow_mut()
                .push((SLOT_CAST_RAY, vec![a[0], a[1], a[2], a[3], level]));
            e.mem.set_u8(a[1], hit);
            Ret::default()
        });
        let child = object_with_slots(e, &[(0x20, SLOT_CAST_RAY)]);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x18, child);
        let output = e.mem.alloc(0x50);
        for key in 0..4 {
            e.mem.set_u32(output + 0x20 + 4 * key, 0xaaaa_0000 + key);
        }
        e.mem.set_u32(output + 0x40, 2);
        (shape, child, output)
    }

    #[test]
    fn cast_ray_counts_a_level_and_clears_the_key_of_a_hit() {
        let (mut e, log, stream) = setup();
        let (shape, child, output) = ray_shape(&mut e, &log, 1);
        let hit = e.mem.alloc(4);
        let got = e
            .call(0x00c9_d9e0, &args![shape, hit, 0x7777u32, output])
            .u32();
        assert_eq!(got, hit);
        assert_eq!(
            *log.borrow(),
            vec![(SLOT_CAST_RAY, vec![child, hit, 0x7777, output, 3])]
        );
        assert_eq!(e.mem.u32(output + 0x40), 2);
        assert_eq!(e.mem.u32(output + 0x28), 0);
        assert_eq!(e.mem.u32(output + 0x24), 0xaaaa_0001);
        assert_eq!(e.mem.u32(output + 0x2c), 0xaaaa_0003);
        assert_eq!(labels(&e, stream), vec![0x010c_7024, 0x010c_41f8]);
    }

    #[test]
    fn cast_ray_keeps_the_key_of_a_miss() {
        let (mut e, log, stream) = setup();
        let (shape, _, output) = ray_shape(&mut e, &log, 0);
        let hit = e.mem.alloc(4);
        e.call(0x00c9_d9e0, &args![shape, hit, 0x7777u32, output]);
        assert_eq!(e.mem.u32(output + 0x28), 0xaaaa_0002);
        assert_eq!(e.mem.u32(output + 0x40), 2);
        assert_eq!(labels(&e, stream), vec![0x010c_7024, 0x010c_41f8]);
    }

    #[test]
    fn cast_ray_with_collector_hands_the_child_a_four_word_record() {
        let (mut e, log, stream) = setup();
        let sink = Rc::clone(&log);
        e.register_double(SLOT_COLLECTOR, move |e, a| {
            let record: Vec<u32> = (0..4).map(|word| e.mem.u32(a[2] + 4 * word)).collect();
            sink.borrow_mut()
                .push((SLOT_COLLECTOR, [a.to_vec(), record].concat()));
            Ret::default()
        });
        let child = object_with_slots(&mut e, &[(0x24, SLOT_COLLECTOR)]);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x18, child);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector + 8, 0x6666);
        e.call(0x00c9_da80, &args![shape, 0x1111u32, collector, 0x2222u32]);
        let calls = calls_to(&log, SLOT_COLLECTOR);
        assert_eq!(calls.len(), 1);
        let call = &calls[0];
        assert_eq!([call[0], call[1], call[3]], [child, 0x1111, 0x2222]);
        assert_eq!(call[4..8], [child, 0, 0x6666, collector]);
        assert_eq!(labels(&e, stream), vec![0x010c_7024, 0x010c_41f8]);
    }
}
