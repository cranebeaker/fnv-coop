//! `Havok SDK/hkpmultirayshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMultiRayShape` (field names describe what the code does and are unconfirmed): an
//! array of rays at +0x10 (data pointer, count at +0x14, capacity and flags word at +0x18),
//! each ray two 16-byte vectors (start, end), and a float at +0x1c the constructor-like
//! function `00e3c680` stores.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// The memory router (`00c85750`, no arguments); its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB): cdecl `(allocator, array, capacity, element size)`.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkReferencedObject::calcContentStatistics` (`00c90360`, `(this, collector, class record)`).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The class record that call is given (`01268910`) and the name of the rays array in the
/// report (`010eba30`).
const CLASS_RECORD: u32 = 0x0126_8910;
const NAME_RAYS: u32 = 0x010e_ba30;
/// The vtable `00e3c680` stores (`010e222c`).
const VTABLE: u32 = 0x010e_222c;
/// Float constants in the exe image: zero, three and one half for the normalizing step of
/// `00e3c680`, and the start values of the box minimum and maximum of `getAabbImpl`.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const BOX_MINIMUM_START: u32 = 0x0104_da48;
const BOX_MAXIMUM_START: u32 = 0x0104_e148;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, at: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, at: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// `MINPS a, b`: each lane `a` when `a < b`, else `b` (NaN and equal lanes give `b`).
fn minps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] < b[i] { a[i] } else { b[i] })
}

/// `MAXPS a, b`: each lane `a` when `a > b`, else `b`.
fn maxps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if a[i] > b[i] { a[i] } else { b[i] })
}

// Translated from 00e3c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayShape::castRayWithCollectorImpl` (Xbox PDB), thiscall with three stack words,
/// `RET 0xC`: does nothing.
pub fn hkp_multi_ray_shape_cast_ray_with_collector_impl(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

// Translated from 00e3c610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayShape::castRayImpl` (Xbox PDB), thiscall with three stack words, `RET 0xC`:
/// clears the byte the first one points to (no hit).
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_multi_ray_shape_cast_ray_impl(
    e: &mut Engine,
    _this: Ptr,
    result: Ptr,
    _unused_2: u32,
    _unused_3: u32,
) -> Ptr {
    e.mem.set_u8(result.addr(), 0);
    result
}

// Translated from 00e3c620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayShape::calcContentStatistics` (Xbox PDB), thiscall `(this, collector)`,
/// `RET 8`: when the rays array owns its storage (capacity word non-negative) or the
/// collector's byte at +0x10 is set, reports through the collector's virtual function at
/// +0x14 `(3, "name", data, size * 32, allocated size)` with the allocated size from
/// `hkStatisticsCollector::getAllocatedSize` of `(data, capacity * 32)`; then the base
/// `hkReferencedObject::calcContentStatistics` (`00c90360`) gets `(collector, 01268910)`.
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hkp_multi_ray_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let t = this.addr();
    let capacity_word = e.mem.u32(t + 0x18);
    if (capacity_word as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
        let data = e.mem.u32(t + 0x10);
        let bytes = (capacity_word & 0x3fff_ffff) << 5;
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(t + 0x10);
        let used = e.mem.u32(t + 0x14) << 5;
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, NAME_RAYS, data, used, allocated],
        );
    }
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, CLASS_RECORD],
    );
}

// Translated from 00e3c680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map; builds the rays array from `count` rays at `rays` (thiscall `(this,
/// rays, count, scale)`, `RET 0xC`, returns `this`): sets the word at +6 to 1, +8 to 0, +0xC
/// to 0x18, the vtable (`010e222c`), an empty array at +0x10 (no data, size 0, capacity word
/// `0x80000000`) and the scale at +0x1c (an `FLD`/`FSTP` copy), grows the array to hold the
/// new rays (`hkArrayUtil::_reserve` of the memory router's allocator to the larger of the
/// needed count and twice the capacity, element size 0x20) and appends them. Each ray's start
/// is copied, its end becomes `end + normalize(end - start) * scale` in the first three lanes
/// and the same with the difference of the fourth lanes in the fourth, and its start's fourth
/// lane becomes the length of the first three lanes of the difference (`SQRTSS`). The
/// direction is normalised the way the Havok code does (`RSQRTSS` and one Newton step,
/// zero for a zero length). It is probably the `hkpMultiRayShape` constructor (unconfirmed).
pub fn fn_00e3c680(e: &mut Engine, this: Ptr, rays: Ptr, count: i32, scale: f32) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, 0x18);
    e.mem.set_u32(t, VTABLE);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x14, 0);
    e.mem.set_u32(t + 0x18, 0x8000_0000);
    e.mem.set_u32(t + 0x1c, x87_quiet_bits(scale.to_bits()));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let old_size = e.mem.u32(t + 0x14);
    let capacity = (e.mem.u32(t + 0x18) & 0x3fff_ffff) as i32;
    let needed = (old_size as i32).wrapping_add(count);
    if capacity < needed {
        let doubled = capacity.wrapping_mul(2);
        let wanted = if needed < doubled { doubled } else { needed };
        e.call(ARRAY_RESERVE, &args![allocator, t + 0x10, wanted, 0x20u32]);
    }
    let new_size = e.mem.u32(t + 0x14).wrapping_add(count as u32);
    e.mem.set_u32(t + 0x14, new_size);
    let mut out = old_size.wrapping_shl(5).wrapping_add(e.mem.u32(t + 0x10));
    let mut src = rays.addr();
    for _ in 0..count.max(0) {
        let zero = e.mem.f32(ZERO);
        let three = e.mem.f32(THREE);
        let half = e.mem.f32(HALF);
        copy_vec(e, src, out);
        copy_vec(e, src + 0x10, out + 0x10);
        let start = load_vec(e, out);
        let end = load_vec(e, out + 0x10);
        let diff: Vec4 = std::array::from_fn(|lane| end[lane] - start[lane]);
        let squares = diff.map(|d| d * d);
        let length_squared = squares[2] + (squares[1] + squares[0]);
        e.mem.set_f32(out + 0xc, x87_quiet(length_squared.sqrt()));
        let estimate = rsqrt_estimate(length_squared);
        let step = half * estimate * (three - length_squared * estimate * estimate);
        let factor = if length_squared == zero { 0.0 } else { step };
        let stored_scale = e.mem.f32(t + 0x1c);
        let moved: Vec4 =
            std::array::from_fn(|lane| factor * diff[lane] * stored_scale + end[lane]);
        store_vec(e, out + 0x10, moved);
        out = out.wrapping_add(0x20);
        src = src.wrapping_add(0x20);
    }
    this
}

// Translated from 00e3c800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayShape::getAabbImpl` (Xbox PDB), thiscall `(this, transform, tolerance,
/// aabb)` with the third word unread, `RET 0xC`: transforms the start and end vectors of every
/// ray (the 16 floats at `transform` are four columns; each point becomes `x * column0 + y *
/// column1 + z * column2 + column3`, summed as `(y*c1 + x*c0) + (z*c2 + c3)`) into a local
/// array of 16 elements of 0x20 bytes (grown through `hkArrayUtil::_reserve` when the rays
/// need more) and stores the lane-wise minimum at `aabb` and maximum at `aabb + 0x10` (`MINPS`
/// and `MAXPS` from the constants `0104da48` and `0104e148`, so lanes nothing beats keep
/// them). The local array is freed again through the allocator when it owns its storage.
///
/// The transform loop is a do-while from the last vector down: with no rays it works on
/// index -1, one vector before the local array, which overwrites the array's own data pointer,
/// size and capacity words (the local frame is modelled with the exe's layout: array at +0x50,
/// data at +0x5C), and the later steps read those words back.
pub fn hkp_multi_ray_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    _unused_2: u32,
    aabb: Ptr,
) {
    let t = this.addr();
    let count = e.mem.u32(t + 0x14) as i32;
    e.with_stack(0x25c, |e, frame| {
        let array = frame.addr() + 0x50;
        e.mem.set_u32(array, frame.addr() + 0x5c);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0010);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_mul(2);
            let wanted = if count >= doubled { count } else { doubled };
            e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 0x20u32]);
        }
        let m = transform.addr();
        let columns = [0, 4, 8, 12].map(|first| load_vec(e, m + 4 * first));
        let mut index = count.wrapping_mul(2).wrapping_sub(1);
        e.mem.set_u32(array + 4, count as u32);
        let mut to = e.mem.u32(array).wrapping_add(index.wrapping_shl(4) as u32);
        let mut from = e
            .mem
            .u32(t + 0x10)
            .wrapping_add(index.wrapping_shl(4) as u32);
        loop {
            let [x, y, z, _] = load_vec(e, from);
            let point: Vec4 = std::array::from_fn(|lane| {
                (y * columns[1][lane] + x * columns[0][lane])
                    + (z * columns[2][lane] + columns[3][lane])
            });
            store_vec(e, to, point);
            index = index.wrapping_sub(1);
            from = from.wrapping_sub(0x10);
            to = to.wrapping_sub(0x10);
            if index < 0 {
                break;
            }
        }
        let start_of = |e: &Engine, at: u32| [f32::from_bits(e.mem.u32(at)); 4];
        let mut minimum = start_of(e, BOX_MINIMUM_START);
        let mut maximum = start_of(e, BOX_MAXIMUM_START);
        let mut remaining = e.mem.u32(array + 4) as i32;
        let mut at = e.mem.u32(array);
        if remaining > 0 {
            loop {
                let second = load_vec(e, at + 0x10);
                let first = load_vec(e, at);
                at = at.wrapping_add(0x20);
                remaining -= 1;
                minimum = minps(minimum, second);
                maximum = maxps(maximum, second);
                minimum = minps(minimum, first);
                maximum = maxps(maximum, first);
                if remaining == 0 {
                    break;
                }
            }
        }
        store_vec(e, aabb.addr(), minimum);
        store_vec(e, aabb.addr() + 0x10, maximum);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity_word = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity_word & 0x3fff_ffff) << 5],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e3_c600,
            hkp_multi_ray_shape_cast_ray_with_collector_impl(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00e3_c610,
            hkp_multi_ray_shape_cast_ray_impl(Ptr, Ptr, u32, u32) -> Ptr
        ),
        entry!(
            0x00e3_c620,
            hkp_multi_ray_shape_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00e3_c680, fn_00e3c680(Ptr, Ptr, i32, f32) -> Ptr),
        entry!(
            0x00e3_c800,
            hkp_multi_ray_shape_get_aabb_impl(Ptr, Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_DOUBLE: u32 = 0x0900_0010;
    const REPORT_DOUBLE: u32 = 0x0900_0020;

    /// The argument words of each call a double received.
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Calls logged with the address of the double.
    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        allocator: u32,
        frees: Calls,
        reserves: Calls,
    }

    /// An engine with the exe's float constants, a memory router that hands out an allocator
    /// whose free function (slot 0x10) is logged and an `_reserve` that gives the array a
    /// block of the capacity asked for and logs the call.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_u32(BOX_MINIMUM_START, 0x7f7f_ffee);
        e.mem.set_u32(BOX_MAXIMUM_START, 0xff7f_ffee);
        let frees: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&frees);
        e.register_double(FREE_DOUBLE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let reserves: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reserves);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let block = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        Rig {
            e,
            allocator,
            frees,
            reserves,
        }
    }

    /// Writes floats one after another and returns the address.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32 + 0x20);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// The estimate behind the normalisation is the hardware's, so results are only expected
    /// to agree within a few units in the last place.
    fn assert_close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len());
        for (a, x) in actual.iter().zip(expected) {
            assert!(
                (a - x).abs() <= 8.0 * f32::EPSILON * x.abs().max(1.0),
                "{actual:?} vs {expected:?}"
            );
        }
    }

    #[test]
    fn cast_ray_with_collector_changes_nothing() {
        let mut r = rig();
        let block = r.e.mem.alloc(0x20);
        r.e.mem.write(block, &[0xaa; 0x20]);
        r.e.call(0x00e3_c600, &args![block, 1u32, 2u32, 3u32]);
        assert_eq!(r.e.mem.bytes(block, 0x20), [0xaa; 0x20]);
    }

    #[test]
    fn cast_ray_clears_the_result_byte() {
        let mut r = rig();
        let block = r.e.mem.alloc(0x10);
        r.e.mem.write(block, &[0xff; 4]);
        r.e.call(0x00e3_c610, &args![0x1234u32, block + 1, 5u32, 6u32]);
        assert_eq!(r.e.mem.bytes(block, 4), [0xff, 0, 0xff, 0xff]);
    }

    #[test]
    fn fn_00e3c610_answers_its_result_pointer() {
        let mut r = rig();
        let block = r.e.mem.alloc(0x10);
        r.e.mem.write(block, &[0xff; 4]);
        // The exe leaves the result pointer in EAX at every RET; callers read through it.
        assert_eq!(
            r.e.call(0x00e3_c610, &args![0x1234u32, block + 1, 5u32, 6u32])
                .u32(),
            args![block + 1][0]
        );
    }

    /// Doubles for the statistics callees that log `(address, arguments)`.
    fn statistics_rig() -> (Rig, Log, u32, u32) {
        let mut r = rig();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            GET_ALLOCATED_SIZE,
            REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
            REPORT_DOUBLE,
        ] {
            let sink = Rc::clone(&log);
            r.e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: 77,
                    ..Ret::default()
                }
            });
        }
        let vtable = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(vtable + 0x14, REPORT_DOUBLE);
        let collector = r.e.mem.alloc(0x20);
        r.e.mem.set_u32(collector, vtable);
        let this = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(this + 0x10, 0x5000);
        r.e.mem.set_u32(this + 0x14, 5);
        (r, log, this, collector)
    }

    #[test]
    fn statistics_report_an_array_that_owns_its_storage() {
        let (mut r, log, this, collector) = statistics_rig();
        r.e.mem.set_u32(this + 0x18, 8);
        r.e.call(0x00e3_c620, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            [
                (GET_ALLOCATED_SIZE, vec![0x5000, 8 << 5]),
                (
                    REPORT_DOUBLE,
                    vec![collector, 3, NAME_RAYS, 0x5000, 5 << 5, 77]
                ),
                (
                    REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                    vec![this, collector, CLASS_RECORD]
                ),
            ]
        );
    }

    #[test]
    fn statistics_skip_borrowed_storage_unless_the_collector_wants_it() {
        let (mut r, log, this, collector) = statistics_rig();
        r.e.mem.set_u32(this + 0x18, 0x8000_0008);
        r.e.call(0x00e3_c620, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            [(
                REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                vec![this, collector, CLASS_RECORD]
            )]
        );
        log.borrow_mut().clear();
        r.e.mem.set_u8(collector + 0x10, 1);
        r.e.call(0x00e3_c620, &args![this, collector, 0u32]);
        let log = log.borrow();
        assert_eq!(log.len(), 3);
        assert_eq!(log[0], (GET_ALLOCATED_SIZE, vec![0x5000, 8 << 5]));
        assert_eq!(
            log[1],
            (
                REPORT_DOUBLE,
                vec![collector, 3, NAME_RAYS, 0x5000, 5 << 5, 77]
            )
        );
    }

    fn construct(r: &mut Rig, this: u32, rays: u32, count: i32, scale: f32) -> u32 {
        r.e.call(0x00e3_c680, &args![this, rays, count, scale.to_bits()])
            .u32()
    }

    #[test]
    fn the_constructor_sets_the_header_and_an_empty_array() {
        let mut r = rig();
        let this = r.e.mem.alloc(0x40);
        r.e.mem.write(this, &[0xaa; 0x40]);
        let returned = construct(&mut r, this, 0x1000, 0, f32::from_bits(0x7f80_0001));
        assert_eq!(returned, this);
        assert_eq!(r.e.mem.u16(this + 6), 1);
        assert_eq!(r.e.mem.u32(this + 8), 0);
        assert_eq!(r.e.mem.u32(this + 0xc), 0x18);
        assert_eq!(r.e.mem.u32(this), VTABLE);
        assert_eq!(r.e.mem.u32(this + 0x10), 0);
        assert_eq!(r.e.mem.u32(this + 0x14), 0);
        assert_eq!(r.e.mem.u32(this + 0x18), 0x8000_0000);
        // The scale passed through an `FLD`/`FSTP` pair is quieted.
        assert_eq!(r.e.mem.u32(this + 0x1c), 0x7fc0_0001);
        assert_eq!(r.e.mem.bytes(this + 4, 2), [0xaa, 0xaa]);
        assert!(r.reserves.borrow().is_empty());
    }

    #[test]
    fn a_negative_count_only_changes_the_size() {
        let mut r = rig();
        let this = r.e.mem.alloc(0x40);
        construct(&mut r, this, 0x1000, -1, 1.0);
        assert_eq!(r.e.mem.u32(this + 0x14), 0xffff_ffff);
        assert!(r.reserves.borrow().is_empty());
    }

    #[test]
    fn the_constructor_appends_rays_with_their_length_and_a_longer_end() {
        let mut r = rig();
        let rays = floats(
            &mut r.e,
            &[
                1.0, 2.0, 3.0, 10.0, 1.0, 2.0, 5.0, 14.0, // length 2
                4.0, 5.0, 6.0, 7.0, 4.0, 5.0, 6.0, 7.0, // no length
            ],
        );
        let this = r.e.mem.alloc(0x40);
        construct(&mut r, this, rays, 2, 3.0);
        assert_eq!(
            *r.reserves.borrow(),
            [vec![r.allocator, this + 0x10, 2, 0x20]]
        );
        assert_eq!(r.e.mem.u32(this + 0x14), 2);
        let data = r.e.mem.u32(this + 0x10);
        // Start (1, 2, 3) with the length 2 in the fourth lane; the end is moved 0.5 * 2 * 3
        // further along z and 0.5 * 4 * 3 in the fourth lane.
        assert_eq!(read(&r.e, data, 3), [1.0, 2.0, 3.0]);
        assert_close(&read(&r.e, data + 0xc, 1), &[2.0]);
        assert_close(&read(&r.e, data + 0x10, 4), &[1.0, 2.0, 8.0, 20.0]);
        // The second ray has no length: the end stays and the length is zero.
        assert_eq!(read(&r.e, data + 0x20, 3), [4.0, 5.0, 6.0]);
        assert_eq!(r.e.mem.f32(data + 0x2c), 0.0);
        assert_eq!(read(&r.e, data + 0x30, 4), [4.0, 5.0, 6.0, 7.0]);
    }

    #[test]
    fn the_constructor_takes_the_direction_from_all_three_axes() {
        let mut r = rig();
        let rays = floats(&mut r.e, &[0.0, 0.0, 0.0, 0.0, 3.0, 4.0, 12.0, 0.0]);
        let this = r.e.mem.alloc(0x40);
        construct(&mut r, this, rays, 1, 2.0);
        let data = r.e.mem.u32(this + 0x10);
        // Length 13; the end moves 2 along (3, 4, 12) / 13.
        assert_close(&read(&r.e, data + 0xc, 1), &[13.0]);
        assert_close(
            &read(&r.e, data + 0x10, 4),
            &[3.0 + 6.0 / 13.0, 4.0 + 8.0 / 13.0, 12.0 + 24.0 / 13.0, 0.0],
        );
    }

    /// The transform: four columns of four floats.
    fn transform(e: &mut Engine, columns: [[f32; 4]; 4]) -> u32 {
        floats(e, &columns.concat())
    }

    /// A shape with `count` rays (two vectors each, from `vectors`) and the box it gives.
    fn aabb_of(r: &mut Rig, columns: [[f32; 4]; 4], vectors: &[[f32; 4]]) -> (Vec<f32>, Vec<f32>) {
        let matrix = transform(&mut r.e, columns);
        let data = floats(&mut r.e, &vectors.concat());
        let this = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(this + 0x10, data);
        r.e.mem.set_u32(this + 0x14, vectors.len() as u32 / 2);
        let aabb = r.e.mem.alloc(0x20);
        r.e.call(0x00e3_c800, &args![this, matrix, 0u32, aabb]);
        (read(&r.e, aabb, 4), read(&r.e, aabb + 0x10, 4))
    }

    const IDENTITY: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    #[test]
    fn the_box_surrounds_the_moved_points() {
        let mut r = rig();
        let mut columns = IDENTITY;
        columns[3] = [1.0, 2.0, 3.0, 1.0];
        let (minimum, maximum) = aabb_of(
            &mut r,
            columns,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [1.0, 1.0, 1.0, 0.0],
                [-2.0, 3.0, 0.5, 0.0],
                [4.0, -1.0, 2.0, 0.0],
            ],
        );
        // The points are (1, 2, 3), (2, 3, 4), (-1, 5, 3.5) and (5, 1, 5), all with 1 in the
        // fourth lane.
        assert_eq!(minimum, [-1.0, 1.0, 3.0, 1.0]);
        assert_eq!(maximum, [5.0, 5.0, 5.0, 1.0]);
        // The array is the local one, which is not freed.
        assert!(r.frees.borrow().is_empty());
        assert!(r.reserves.borrow().is_empty());
    }

    #[test]
    fn the_points_are_rotated_by_the_columns() {
        let mut r = rig();
        // (x, y, z) -> (-y, x, z) + (10, 20, 30).
        let columns = [
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 20.0, 30.0, 1.0],
        ];
        let (minimum, maximum) = aabb_of(
            &mut r,
            columns,
            &[[2.0, 3.0, 4.0, 9.0], [1.0, 0.0, -1.0, 9.0]],
        );
        assert_eq!(minimum, [7.0, 21.0, 29.0, 1.0]);
        assert_eq!(maximum, [10.0, 22.0, 34.0, 1.0]);
    }

    #[test]
    fn a_nan_point_is_dropped_by_a_later_point_and_kept_over_an_earlier_one() {
        let mut r = rig();
        // MINPS and MAXPS give the second operand for an unordered pair. In a ray the end
        // vector is taken into the box before the start vector: a NaN point (every lane is NaN
        // as it multiplies every column) at the end is replaced by the start, one at the start
        // stays.
        let (minimum, maximum) = aabb_of(
            &mut r,
            IDENTITY,
            &[[3.0, 4.0, 5.0, 0.0], [f32::NAN, 1.0, 2.0, 0.0]],
        );
        assert_eq!(minimum, [3.0, 4.0, 5.0, 1.0]);
        assert_eq!(maximum, [3.0, 4.0, 5.0, 1.0]);
        let (minimum, maximum) = aabb_of(
            &mut r,
            IDENTITY,
            &[[f32::NAN, 1.0, 2.0, 0.0], [3.0, 4.0, 5.0, 0.0]],
        );
        assert!(minimum.iter().chain(&maximum).all(|lane| lane.is_nan()));
    }

    #[test]
    fn many_rays_grow_the_local_array_and_free_it_after() {
        let mut r = rig();
        let vectors: Vec<[f32; 4]> = (0..20)
            .flat_map(|i| [[i as f32, 0.0, 0.0, 0.0], [i as f32, 1.0, 0.0, 0.0]])
            .collect();
        let (minimum, maximum) = aabb_of(&mut r, IDENTITY, &vectors);
        assert_eq!(minimum, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(maximum, [19.0, 1.0, 0.0, 1.0]);
        // 20 rays do not fit 16: twice the capacity is 32.
        let reserves = r.reserves.borrow();
        assert_eq!(reserves.len(), 1);
        assert_eq!(
            (reserves[0][0], reserves[0][2], reserves[0][3]),
            (r.allocator, 32, 0x20)
        );
        let frees = r.frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!((frees[0][0], frees[0][2]), (r.allocator, 32 << 5));
    }

    #[test]
    fn the_array_is_grown_to_the_larger_of_the_count_and_twice_the_capacity() {
        let mut r = rig();
        let vectors: Vec<[f32; 4]> = (0..17)
            .flat_map(|i| [[i as f32, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]])
            .collect();
        aabb_of(&mut r, IDENTITY, &vectors);
        assert_eq!(r.reserves.borrow()[0][2], 32);
        let mut r = rig();
        let vectors: Vec<[f32; 4]> = (0..32).map(|i| [i as f32, 0.0, 0.0, 0.0]).collect();
        aabb_of(&mut r, IDENTITY, &vectors);
        assert!(r.reserves.borrow().is_empty());
        let mut r = rig();
        let vectors: Vec<[f32; 4]> = (0..66).map(|i| [i as f32, 0.0, 0.0, 0.0]).collect();
        aabb_of(&mut r, IDENTITY, &vectors);
        assert_eq!(r.reserves.borrow()[0][2], 33);
    }

    #[test]
    fn no_rays_work_on_the_vector_before_the_data_and_the_frame_words_it_overwrites() {
        let mut r = rig();
        // With no rays the do-while loop transforms index -1, 16 bytes before the local
        // array, over its own data pointer, size and capacity words. The vector read is the
        // zeros before the shape's data, so the point is the last column (1, 2, 0, 0): the
        // data pointer becomes the bits of 2.0 and size and capacity zero.
        let matrix = transform(
            &mut r.e,
            [[0.0; 4], [0.0; 4], [0.0; 4], [1.0, 2.0, 0.0, 0.0]],
        );
        let block = r.e.mem.alloc(0x40);
        let this = r.e.mem.alloc(0x40);
        r.e.mem.set_u32(this + 0x10, block + 0x10);
        let aabb = r.e.mem.alloc(0x20);
        r.e.call(0x00e3_c800, &args![this, matrix, 0u32, aabb]);
        // Nothing is taken into the box and the "storage" at 2.0 is freed with zero bytes.
        assert_eq!(r.e.mem.u32(aabb), 0x7f7f_ffee);
        assert_eq!(r.e.mem.u32(aabb + 0x1c), 0xff7f_ffee);
        assert_eq!(*r.frees.borrow(), [vec![r.allocator, 0x4000_0000, 0]]);
    }
}
