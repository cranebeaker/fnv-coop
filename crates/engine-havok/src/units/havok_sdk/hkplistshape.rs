//! `Havok SDK/hkplistshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpListShape` (Xbox PDB, RTTI `.?AVhkpListShape@@` confirmed at the vtable `010cc3d8`) is
//! a shape collection of child shapes: `m_childInfo` (an `hkArray` of 0x10-byte
//! `ChildInfo`, the child shape pointer first) at +0x18, `m_aabbHalfExtents` at +0x30 and
//! `m_aabbCenter` at +0x40 (offsets agree with the Xbox PDB). A shape's `getAabbImpl` is
//! vtable slot `0x1c`: `(transform, tolerance, out aabb)`, the aabb being two four-float
//! vectors, minimum then maximum.
//!
//! SSE code computes lane by lane in `f32`; `MINPS`/`MAXPS` answer the second operand when
//! the comparison is false or unordered, as `minimum` and `maximum` here do.

#[allow(unused_imports)]
use crate::prelude::*;

/// The identity transform global `getAabb` of the children is called with when the list
/// shape recomputes its extents (`01268370`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
/// The four-lane vector of `0.5` (`010c71d0`).
const HALF_VECTOR: u32 = 0x010c_71d0;
/// The four-lane vector the extents are compared against (`01267e30`).
const EXTENT_THRESHOLD: u32 = 0x0126_7e30;
/// `float` the minimum corner starts from (`010cc3b0`, `0x7effffee`); the maximum starts
/// from the same value with the sign bits of `SIGN_MASK` flipped.
const START_MINIMUM: u32 = 0x010c_c3b0;
/// Sign mask of four lanes (`010c44d0`).
const SIGN_MASK: u32 = 0x010c_44d0;

layout! {
    /// `hkpListShape` (Xbox PDB), 0x70 bytes: the fields the translations use.
    pub struct HkpListShape: 0x70 {
        /// `m_childInfo.m_data` (Xbox PDB): the `ChildInfo` array, 0x10 bytes per entry.
        0x18 m_childInfo: Ptr,
        /// `m_childInfo.m_size` (Xbox PDB).
        0x1c m_childInfoSize: i32,
        /// `m_childInfo.m_capacityAndFlags` (Xbox PDB): capacity in the low 30 bits, bit 31 set
        /// when the storage is not owned.
        0x20 m_childInfoCapacityAndFlags: u32,
        /// `m_aabbHalfExtents` (Xbox PDB): `hkVector4`.
        0x30 m_aabbHalfExtents: Inline<[f32; 4]>,
        /// `m_aabbCenter` (Xbox PDB): `hkVector4`.
        0x40 m_aabbCenter: Inline<[f32; 4]>,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf5440,
            hkp_list_shape_calc_content_statistics(Ptr<HkpListShape>, Ptr, u32)
        ),
        entry!(
            0x00cf54d0,
            hkp_list_shape_set_shapes(Ptr<HkpListShape>, Ptr, i32, Ptr, u32)
        ),
        entry!(
            0x00cf5640,
            hkp_list_shape_constructor(Ptr<HkpListShape>, Ptr, i32, u32) -> Ptr<HkpListShape>
        ),
        entry!(0x00cf4fb0, fn_00cf4fb0(Ptr<HkpListShape>, Ptr)),
        entry!(
            0x00cf5420,
            hkp_list_shape_recalc_aabb_extents(Ptr<HkpListShape>)
        ),
        entry!(
            0x00cf5590,
            hkp_list_shape_get_aabb_impl(Ptr<HkpListShape>, Ptr, f32, Ptr)
        ),
    ]
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, values: [f32; 4]) {
    for (i, value) in values.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, value);
    }
}

/// `MINPS dst, src` lane: `dst` when `dst < src`, otherwise `src`.
fn minimum(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` lane: `dst` when `dst > src`, otherwise `src`.
fn maximum(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// The child shape of entry `index`: the first word of the 0x10-byte `ChildInfo`.
fn child_shape(e: &Engine, this: Ptr<HkpListShape>, index: i32) -> u32 {
    let data = e.get(this, HkpListShape::m_childInfo);
    e.mem
        .u32(data.addr().wrapping_add((index as u32).wrapping_mul(0x10)))
}

/// Merges the aabb at `from` into the one at `into`: lane-wise minimum of the minimum
/// corners and maximum of the maximum corners (`MINPS`, `MAXPS` with the accumulated one as
/// the first operand).
fn merge_aabb(e: &mut Engine, into: u32, from: u32) {
    let minimum_corner = lanes(e, into);
    let new_minimum = lanes(e, from);
    set_lanes(
        e,
        into,
        [0, 1, 2, 3].map(|i| minimum(minimum_corner[i], new_minimum[i])),
    );
    let maximum_corner = lanes(e, into + 0x10);
    let new_maximum = lanes(e, from + 0x10);
    set_lanes(
        e,
        into + 0x10,
        [0, 1, 2, 3].map(|i| maximum(maximum_corner[i], new_maximum[i])),
    );
}

// Translated from 00cf4fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 4`: the bounding box of all children at the identity
/// transform with tolerance 0 (first child's `getAabbImpl` straight into `aabb`, the others
/// merged in), after which `m_aabbCenter` is `(max + min) * 0.5` and `m_aabbHalfExtents` is
/// `(max - min) * 0.5`. The first child is read even when there are none, as the exe does.
pub fn fn_00cf4fb0(e: &mut Engine, this: Ptr<HkpListShape>, aabb: Ptr) {
    let first = child_shape(e, this, 0);
    e.vcall(first, 0x1c, &args![IDENTITY_TRANSFORM, 0u32, aabb]);
    let count = e.get(this, HkpListShape::m_childInfoSize);
    if count > 1 {
        e.with_stack(0x20, |e, scratch| {
            for index in 1..count {
                let child = child_shape(e, this, index);
                e.vcall(child, 0x1c, &args![IDENTITY_TRANSFORM, 0u32, scratch]);
                merge_aabb(e, aabb.addr(), scratch.addr());
            }
        });
    }
    let minimum_corner = lanes(e, aabb.addr());
    let maximum_corner = lanes(e, aabb.addr() + 0x10);
    let half = lanes(e, HALF_VECTOR);
    let center = [0, 1, 2, 3].map(|i| (maximum_corner[i] + minimum_corner[i]) * half[i]);
    set_lanes(e, this.addr() + 0x40, center);
    let extents = [0, 1, 2, 3].map(|i| (maximum_corner[i] - minimum_corner[i]) * half[i]);
    set_lanes(e, this.addr() + 0x30, extents);
}

// Translated from 00cf5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::recalcAabbExtents` (Xbox PDB): recomputes the center and half extents
/// (`00cf4fb0`) into a scratch aabb that is thrown away.
pub fn hkp_list_shape_recalc_aabb_extents(e: &mut Engine, this: Ptr<HkpListShape>) {
    e.with_stack(0x20, |e, scratch| fn_00cf4fb0(e, this, scratch));
}

// Translated from 00cf5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::getAabbImpl` (the Xbox PDB name of slot `0x1c` in the vtable `010cc3d8`,
/// where this body sits), `RET 0xc`: `(transform, tolerance, out aabb)`. Recomputes the
/// extents first when every half extent is at or below the threshold vector `01267e30`
/// (unordered counts as above), then starts the box inverted (minimum `0x7effffee`, maximum
/// its sign-flipped twin) and merges in the `getAabbImpl` of every child with the given
/// transform and tolerance (the tolerance passes through `FLD`/`FSTP`).
pub fn hkp_list_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpListShape>,
    transform: Ptr,
    tolerance: f32,
    aabb: Ptr,
) {
    let extents = lanes(e, this.addr() + 0x30);
    let threshold = lanes(e, EXTENT_THRESHOLD);
    if (0..4).all(|i| extents[i] <= threshold[i]) {
        hkp_list_shape_recalc_aabb_extents(e, this);
    }
    let start = e.mem.u32(START_MINIMUM);
    for lane in 0..4 {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(aabb.addr() + 4 * lane, start);
        e.mem.set_u32(aabb.addr() + 0x10 + 4 * lane, start ^ mask);
    }
    let count = e.get(this, HkpListShape::m_childInfoSize);
    if count > 0 {
        let tolerance_bits = x87_quiet_bits(tolerance.to_bits());
        e.with_stack(0x20, |e, scratch| {
            for index in 0..count {
                let child = child_shape(e, this, index);
                e.vcall(child, 0x1c, &args![transform, tolerance_bits, scratch]);
                merge_aabb(e, aabb.addr(), scratch.addr());
            }
        });
    }
}

/// The thread's memory router (`00c85750`); word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `00c90670` (cdecl `(data, count, element size)`): run on the child array by `setShapes`
/// when its last argument is 1.
const ARRAY_FINISH: u32 = 0x00c9_0670;
/// `00d06f80` (cdecl `(data, byte size)`): returns a word that `calcContentStatistics`
/// hands on to the collector.
const BLOCK_QUERY: u32 = 0x00d0_6f80;
/// `00d213e0` (thiscall, `RET 8`, `(collector, constant)`): the base shape's statistics.
const BASE_CALC_STATISTICS: u32 = 0x00d2_13e0;
/// `00d21410` (thiscall, `RET 8`, `(shape type, flag)`): the shape base constructor.
const SHAPE_BASE_CONSTRUCTOR: u32 = 0x00d2_1410;
/// Strings and constants handed to the collector (`010cc3a4`, `010c6fec`, `01269198`;
/// unconfirmed).
const CHILD_ARRAY_NAME: u32 = 0x010c_c3a4;
const CHILD_NAME: u32 = 0x010c_6fec;
const BASE_ARGUMENT: u32 = 0x0126_9198;

// Translated from 00cf5440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::calcContentStatistics` (Xbox PDB), `RET 8` with the second word unread:
/// when the child array owns its storage (bit 31 of the capacity word clear) or the byte at
/// `collector + 0x10` is not 0, reports it to the collector's slot `0x14` as `(3, "child
/// array", data, used bytes, 00d06f80(data, capacity bytes))`; then reports every child shape
/// to slot `0x0C` as `(name, child, 0)` and ends with `00d213e0(this, collector, constant)`.
pub fn hkp_list_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpListShape>,
    collector: Ptr,
    _unused_1: u32,
) {
    let capacity = e.get(this, HkpListShape::m_childInfoCapacityAndFlags);
    let reports = (capacity as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0;
    if reports {
        let data = e.get(this, HkpListShape::m_childInfo).addr();
        let word = e
            .call(BLOCK_QUERY, &args![data, (capacity & 0x3fff_ffff) << 4])
            .u32();
        let data = e.get(this, HkpListShape::m_childInfo).addr();
        let used = (e.get(this, HkpListShape::m_childInfoSize) as u32) << 4;
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, CHILD_ARRAY_NAME, data, used, word],
        );
    }
    let count = e.get(this, HkpListShape::m_childInfoSize);
    if count > 0 {
        let mut offset = 0u32;
        let mut index = 0;
        loop {
            let data = e.get(this, HkpListShape::m_childInfo).addr();
            let child = e.mem.u32(data.wrapping_add(offset));
            e.vcall(collector.addr(), 0xc, &args![CHILD_NAME, child, 0u32]);
            index += 1;
            offset = offset.wrapping_add(0x10);
            if index >= e.get(this, HkpListShape::m_childInfoSize) {
                break;
            }
        }
    }
    e.call(BASE_CALC_STATISTICS, &args![this, collector, BASE_ARGUMENT]);
}

// Translated from 00cf54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::setShapes` (Xbox PDB), `RET 0x10`: `(shapes, count, filter infos, flag)`.
/// Grows the child array to at least `count` entries (twice the capacity, or `count` when
/// that is larger; `00c908c0` with element size 0x10), sets its size to `count`, and writes
/// for every non-null shape pointer its `ChildInfo`: the shape at +0, the matching filter
/// info (or 0 when that array is null) at +4, 0 at +8 and `count` at +0xC (entries of null
/// shapes are skipped and keep their old contents). When the flag is exactly 1, runs
/// `00c90670(data, size, 0x10)`; then recomputes the extents (`00cf5420`).
pub fn hkp_list_shape_set_shapes(
    e: &mut Engine,
    this: Ptr<HkpListShape>,
    shapes: Ptr,
    count: i32,
    filter_infos: Ptr,
    flag: u32,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let array = this.addr() + 0x18;
    let capacity = (e.get(this, HkpListShape::m_childInfoCapacityAndFlags) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = doubled.max(count);
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    e.mem.set_u32(array + 4, count as u32);
    if count > 0 {
        let mut source = shapes.addr();
        let mut offset = 0u32;
        for _ in 0..count {
            let shape = e.mem.u32(source);
            if shape != 0 {
                let entry = e.mem.u32(array).wrapping_add(offset);
                e.mem.set_u32(entry, shape);
                let filter = if filter_infos.addr() != 0 {
                    e.mem.u32(
                        filter_infos
                            .addr()
                            .wrapping_add(source.wrapping_sub(shapes.addr())),
                    )
                } else {
                    0
                };
                e.mem.set_u32(entry + 4, filter);
                e.mem.set_u32(entry + 0xc, count as u32);
                e.mem.set_u32(entry + 8, 0);
            }
            source = source.wrapping_add(4);
            offset = offset.wrapping_add(0x10);
        }
    }
    if flag == 1 {
        let size = e.mem.u32(this.addr() + 0x1c);
        let data = e.mem.u32(array);
        e.call(ARRAY_FINISH, &args![data, size, 0x10u32]);
    }
    hkp_list_shape_recalc_aabb_extents(e, this);
}

// Translated from 00cf5640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::hkpListShape` (Xbox PDB), `RET 0xC`, `(shapes, count, flag)`:
/// the shape base constructor (`00d21410` with `(9, 0)`), the vtables `010cc3d8` at +0 and
/// `010cc3b8` at +0x10, the child array at +0x18 empty without owned storage and reserved
/// for 4 entries of 0x10 bytes, `setShapes(shapes, count, 0, flag)`, the eight words
/// at +0x50 set to `-1` and the two 16-bit words at +0x24 and +0x26 cleared. Returns `this`.
pub fn hkp_list_shape_constructor(
    e: &mut Engine,
    this: Ptr<HkpListShape>,
    shapes: Ptr,
    count: i32,
    flag: u32,
) -> Ptr<HkpListShape> {
    let me = this.addr();
    e.call(SHAPE_BASE_CONSTRUCTOR, &args![this, 9u32, 0u32]);
    e.mem.set_u32(me, 0x010c_c3d8);
    e.mem.set_u32(me + 0x10, 0x010c_c3b8);
    let array = me + 0x18;
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < 4 {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = doubled.max(4);
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x10u32]);
    }
    hkp_list_shape_set_shapes(e, this, shapes, count, Ptr::NULL, flag);
    for word in 0..8 {
        e.mem.set_u32(me + 0x50 + 4 * word, 0xffff_ffff);
    }
    e.mem.set_u16(me + 0x26, 0);
    e.mem.set_u16(me + 0x24, 0);
    this
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHILD_VTABLE: u32 = 0x0200_0000;
    const CHILD_GET_AABB: u32 = 0x0200_0100;

    /// Children are objects `[vtable, min x4, max x4]`; their `getAabbImpl` copies that box
    /// out and logs the transform and tolerance words it was given.
    fn rig(boxes: &[([f32; 4], [f32; 4])]) -> (Engine, Ptr<HkpListShape>) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.put_vtable(CHILD_VTABLE, &[0, 0, 0, 0, 0, 0, 0, CHILD_GET_AABB]);
        e.register(CHILD_GET_AABB, |e, a| {
            for lane in 0..8 {
                let value = e.mem.u32(a[0] + 4 + 4 * lane);
                e.mem.set_u32(a[3] + 4 * lane, value);
            }
            Ret::default()
        });
        set_lanes(&mut e, HALF_VECTOR, [0.5; 4]);
        set_lanes(&mut e, EXTENT_THRESHOLD, [0.0; 4]);
        e.mem.set_u32(START_MINIMUM, 0x7eff_ffee);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let list = e.new_object::<HkpListShape>();
        let infos = e.mem.alloc(0x10 * boxes.len() as u32 + 0x10);
        for (i, (low, high)) in boxes.iter().enumerate() {
            let child = e.mem.alloc(0x24);
            e.mem.set_u32(child, CHILD_VTABLE);
            set_lanes(&mut e, child + 4, *low);
            set_lanes(&mut e, child + 0x14, *high);
            e.mem.set_u32(infos + 0x10 * i as u32, child);
        }
        e.set(list, HkpListShape::m_childInfo, Ptr::new(infos));
        e.set(list, HkpListShape::m_childInfoSize, boxes.len() as i32);
        (e, list)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const TWO_BOXES: [([f32; 4], [f32; 4]); 2] = [
        ([0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 0.0]),
        ([-1.0, 1.0, 0.5, 0.0], [2.0, 3.0, 1.0, 0.0]),
    ];

    #[test]
    fn extents_come_from_the_merged_children() {
        let (mut e, list) = rig(&TWO_BOXES);
        let out = e.mem.alloc(0x20);
        e.call(0x00cf_4fb0, &args![list, Ptr::<()>::new(out)]);
        assert_eq!(lanes(&e, out), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [2.0, 3.0, 3.0, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x40), [0.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
        let children = calls(&e, CHILD_GET_AABB);
        assert_eq!(children.len(), 2);
        // Identity transform, tolerance zero.
        assert_eq!(children[1][1..3], [IDENTITY_TRANSFORM, 0]);
    }

    #[test]
    fn recalc_runs_the_same_computation_on_a_scratch_box() {
        let (mut e, list) = rig(&TWO_BOXES);
        e.call(0x00cf_5420, &args![list]);
        assert_eq!(lanes(&e, list.addr() + 0x40), [0.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
    }

    #[test]
    fn get_aabb_recalculates_only_for_empty_extents() {
        let (mut e, list) = rig(&TWO_BOXES);
        let out = e.mem.alloc(0x20);
        let transform = e.mem.alloc(0x40);
        e.call(
            0x00cf_5590,
            &args![
                list,
                Ptr::<()>::new(transform),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        // The extents were all at or below the threshold, so they were recomputed.
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, out), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [2.0, 3.0, 3.0, 0.0]);
        let children = calls(&e, CHILD_GET_AABB);
        // Two calls for the recomputation, two for the box itself, with the given arguments.
        assert_eq!(children.len(), 4);
        assert_eq!(children[2][1..3], [transform, 0.25f32.to_bits()]);

        // With non-empty extents the recomputation is skipped.
        let (mut e, list) = rig(&TWO_BOXES);
        set_lanes(&mut e, list.addr() + 0x30, [1.0, 1.0, 1.0, 1.0]);
        e.call(
            0x00cf_5590,
            &args![
                list,
                Ptr::<()>::new(transform),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        assert_eq!(calls(&e, CHILD_GET_AABB).len(), 2);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.0; 4]);
    }

    #[test]
    fn get_aabb_of_an_empty_list_is_the_inverted_box() {
        let (mut e, list) = rig(&[]);
        set_lanes(&mut e, list.addr() + 0x30, [1.0; 4]);
        let out = e.mem.alloc(0x20);
        e.call(
            0x00cf_5590,
            &args![list, Ptr::<()>::new(0), 0.0f32, Ptr::<()>::new(out)],
        );
        assert_eq!(e.mem.u32(out), 0x7eff_ffee);
        assert_eq!(e.mem.u32(out + 0x1c), 0xfeff_ffee);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const SLOT_A: u32 = 0x0900_0011;
    const SLOT_B: u32 = 0x0900_0012;
    const AABB_SLOT: u32 = 0x0900_0013;

    /// Logs every call to `addr` as `[tag, args..]` and answers `result` in EAX.
    fn log_calls(e: &mut Engine, addr: u32, log: &Log, tag: u32, result: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            sink.borrow_mut().push(row);
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn router_with_allocator(e: &mut Engine, allocator: u32) {
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
    }

    /// A shape whose `getAabbImpl` slot (0x1c) is the logging double.
    fn shape(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, AABB_SLOT);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn calc_content_statistics_reports_the_array_and_every_child() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        log_calls(&mut e, BLOCK_QUERY, &log, 1, 0xabc);
        log_calls(&mut e, SLOT_A, &log, 2, 0);
        log_calls(&mut e, SLOT_B, &log, 3, 0);
        log_calls(&mut e, BASE_CALC_STATISTICS, &log, 4, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x14, SLOT_A);
        e.mem.set_u32(vtable + 0xc, SLOT_B);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        let this = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, 0x111);
        e.mem.set_u32(data + 0x10, 0x222);
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 2);
        e.mem.set_u32(this + 0x20, 4);
        e.call(0x00cf5440, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![1, data, 0x40],
                vec![2, collector, 3, CHILD_ARRAY_NAME, data, 0x20, 0xabc],
                vec![3, collector, CHILD_NAME, 0x111, 0],
                vec![3, collector, CHILD_NAME, 0x222, 0],
                vec![4, this, collector, BASE_ARGUMENT],
            ]
        );
        // Storage that is not owned is reported only when the collector asks for it; an empty
        // array reports no children.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x20, 0x8000_0004);
        e.mem.set_u32(this + 0x1c, 0);
        e.call(0x00cf5440, &args![this, collector, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![4, this, collector, BASE_ARGUMENT]]);
        log.borrow_mut().clear();
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00cf5440, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![1, data, 0x40],
                vec![2, collector, 3, CHILD_ARRAY_NAME, data, 0, 0xabc],
                vec![4, this, collector, BASE_ARGUMENT],
            ]
        );
    }

    #[test]
    fn set_shapes_fills_the_entries_and_skips_null_shapes() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        log_calls(&mut e, ARRAY_RESERVE, &log, 1, 0);
        log_calls(&mut e, ARRAY_FINISH, &log, 2, 0);
        log_calls(&mut e, AABB_SLOT, &log, 3, 0);
        router_with_allocator(&mut e, 0x1234);
        let (first, kept, last) = (shape(&mut e), shape(&mut e), shape(&mut e));
        let this = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x100);
        for entry in 0..9 {
            let at = data + 0x10 * entry;
            e.mem.set_u32(at, kept);
            e.mem.set_u32(at + 4, 0x77);
            e.mem.set_u32(at + 8, 0x88);
            e.mem.set_u32(at + 0xc, 0x99);
        }
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x20, 8);
        let shapes = e.mem.alloc(0x40);
        for (i, word) in [first, 0, last].into_iter().enumerate() {
            e.mem.set_u32(shapes + 4 * i as u32, word);
        }
        let filters = e.mem.alloc(0x10);
        for (i, word) in [1u32, 2, 3].into_iter().enumerate() {
            e.mem.set_u32(filters + 4 * i as u32, word);
        }
        e.call(0x00cf54d0, &args![this, shapes, 3u32, filters, 1u32]);
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        let entry = |e: &Engine, i: u32| {
            (0..4)
                .map(|w| e.mem.u32(data + 0x10 * i + 4 * w))
                .collect::<Vec<_>>()
        };
        assert_eq!(entry(&e, 0), vec![first, 1, 0, 3]);
        assert_eq!(entry(&e, 1), vec![kept, 0x77, 0x88, 0x99]);
        assert_eq!(entry(&e, 2), vec![last, 3, 0, 3]);
        // No growth was needed; the flag 1 ran 00c90670; the extents were recomputed over
        // all three children.
        let rows = log.borrow().clone();
        assert_eq!(
            rows.iter().map(|r| r[0]).collect::<Vec<_>>(),
            vec![2, 3, 3, 3]
        );
        assert_eq!(rows[0][1..], [data, 3, 0x10]);
        assert_eq!(
            rows[1..].iter().map(|r| r[1]).collect::<Vec<_>>(),
            vec![first, kept, last]
        );
        // Without filter infos the filter word is 0; without the flag nothing is finished.
        log.borrow_mut().clear();
        e.call(0x00cf54d0, &args![this, shapes, 1u32, 0u32, 0u32]);
        assert_eq!(entry(&e, 0), vec![first, 0, 0, 1]);
        assert!(log.borrow().iter().all(|r| r[0] == 3));
        // Growing doubles the capacity or takes the count, whichever is larger.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x20, 2);
        e.call(0x00cf54d0, &args![this, shapes, 3u32, 0u32, 0u32]);
        e.mem.set_u32(this + 0x20, 0x8000_0004);
        e.call(0x00cf54d0, &args![this, shapes, 9u32, 0u32, 0u32]);
        let reserves: Vec<Vec<u32>> = log
            .borrow()
            .iter()
            .filter(|r| r[0] == 1)
            .map(|r| r[1..].to_vec())
            .collect();
        assert_eq!(
            reserves,
            vec![
                vec![0x1234, this + 0x18, 4, 0x10],
                vec![0x1234, this + 0x18, 9, 0x10]
            ]
        );
    }

    #[test]
    fn the_constructor_sets_up_the_vtables_the_array_and_the_shapes() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::default();
        log_calls(&mut e, SHAPE_BASE_CONSTRUCTOR, &log, 1, 0);
        log_calls(&mut e, ARRAY_FINISH, &log, 3, 0);
        log_calls(&mut e, AABB_SLOT, &log, 4, 0);
        router_with_allocator(&mut e, 0x1234);
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(vec![2, a[0], a[1], a[2], a[3]]);
            let data = e.mem.alloc(a[2] * 0x10);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let (first, second) = (shape(&mut e), shape(&mut e));
        let shapes = e.mem.alloc(0x10);
        e.mem.set_u32(shapes, first);
        e.mem.set_u32(shapes + 4, second);
        let this = e.mem.alloc(0x70);
        e.mem.set_u16(this + 0x24, 0x5555);
        e.mem.set_u16(this + 0x26, 0x5555);
        let result = e.call(0x00cf5640, &args![this, shapes, 2u32, 1u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_c3d8);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_c3b8);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert_eq!(e.mem.u32(this + 0x20), 4);
        for word in 0..8 {
            assert_eq!(e.mem.u32(this + 0x50 + 4 * word), 0xffff_ffff);
        }
        assert_eq!(e.mem.u16(this + 0x24), 0);
        assert_eq!(e.mem.u16(this + 0x26), 0);
        let data = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(data), first);
        assert_eq!(e.mem.u32(data + 0xc), 2);
        assert_eq!(e.mem.u32(data + 0x10), second);
        let rows = log.borrow().clone();
        assert_eq!(rows[0], vec![1, this, 9, 0]);
        assert_eq!(rows[1], vec![2, 0x1234, this + 0x18, 4, 0x10]);
        assert_eq!(rows[2], vec![3, data, 2, 0x10]);
        assert_eq!(
            rows[3..].iter().map(|r| r[0]).collect::<Vec<_>>(),
            vec![4, 4]
        );
    }
}
