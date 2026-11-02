//! `Havok SDK/hkptyremarksinfo.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The Xbox PDB has no type records for the tyre-mark classes, so the layouts below are read
//! off the code and their names are Havok's public ones, unconfirmed. An `hkpTyremarksInfo`
//! (vtable names in the map) holds `m_minTyremarkEnergy` (`+8`) and `m_maxTyremarkEnergy`
//! (`+0xC`) and an array of wheel pointers (`+0x10` data, `+0x14` size). A wheel
//! (`hkpTyremarksWheel`) keeps a ring buffer of tyre-mark points: the current position (`+8`),
//! the number of points (`+0xC`), and the points as an `hkArray` (`+0x10` data, `+0x14` size,
//! `+0x18` capacity and flags). A point is 32 bytes: the left and the right edge of the strip,
//! each a vector whose fourth lane is the strength (`+0xC` and `+0x1C`).
//!
//! The ring buffer functions divide with `CDQ`/`IDIV`: the PC raises a divide error (the game
//! stops) for a zero divisor, which these translations panic on. Float arithmetic on the x87
//! stack is done in `f64` and stored as `f32`, as the other translations do.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpTyremarksInfo` (unconfirmed name), the fields the code reads.
    pub struct HkpTyremarksInfo: 0x18 {
        /// `m_minTyremarkEnergy`.
        0x08 m_min_tyremark_energy: f32,
        /// `m_maxTyremarkEnergy`.
        0x0c m_max_tyremark_energy: f32,
        /// The data pointer of the wheel array.
        0x10 m_tyremarks_wheel: Ptr,
        /// The wheel array's size.
        0x14 m_tyremarks_wheel_size: i32,
    }
}

layout! {
    /// `hkpTyremarksWheel` (unconfirmed name), the fields the code reads.
    pub struct HkpTyremarksWheel: 0x1c {
        /// `m_currentPosition`: the next point to write.
        0x08 m_current_position: i32,
        /// `m_numPoints`: the ring buffer's length.
        0x0c m_num_points: i32,
        /// The point array's data pointer.
        0x10 m_tyremark_points: Ptr,
        /// The point array's size.
        0x14 m_tyremark_points_size: u32,
        /// The point array's capacity and flags (bit 31: the memory is not owned).
        0x18 m_tyremark_points_capacity: u32,
    }
}

/// `hkpTyremarkPoint` is 32 bytes.
const POINT_SIZE: u32 = 0x20;
/// `hkVector4::setRotatedDir` (Xbox PDB, `this` = result, then the rotation and the vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkpTyremarkPoint`'s constructor (`00e39e50`, `this` = the point): zeroes its 32 bytes.
const POINT_CONSTRUCTOR: u32 = 0x00e3_9e50;
/// `hkStatisticsCollector::getAllocatedSize` (Xbox PDB, cdecl `(pointer, size)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpTyremarksWheel::getTyremarkPoint` and the function adding a point (see below).
const GET_TYREMARK_POINT: u32 = 0x00e3_9e60;
const GET_POINT_STRENGTH: u32 = 0x00e3_9e80;
const ADD_TYREMARK_POINT: u32 = 0x00e3_9f50;
/// The string `calcContentStatistics` hands to the collector (`010eb878`).
const POINTS_NAME: u32 = 0x010e_b878;
/// The constants `updateTyremarksInfo` reads: the double 255.0 (`0101e568`), the float 255.0
/// (`01023cd8`), the float 0.05 (`010181bc`), the doubles -0.5 (`01011598`) and 0.5
/// (`01011588`).
const STRENGTH_RANGE: f64 = 255.0;
const STRENGTH_MAX: f32 = 255.0;
const BASE_SCALE: f32 = 0.05;
const NEGATIVE_HALF: f64 = -0.5;
const HALF: f64 = 0.5;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// `CDQ`/`IDIV` remainder: a zero divisor (and `i32::MIN / -1`) raises a divide error on the
/// PC, which stops the game.
fn remainder(dividend: i32, divisor: i32, address: u32) -> i32 {
    dividend.checked_rem(divisor).unwrap_or_else(|| {
        panic!("integer divide error (#DE) in {address:08x}: {dividend} / {divisor}")
    })
}

// Translated from 00e39e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The wheel's point `index` places after the current position, wrapped round the ring (the
/// map gives it no name; that it is `hkpTyremarksWheel::getTyremarkPoint` is unconfirmed):
/// `((current + index) % number of points) * 32 + data`, a signed remainder.
pub fn fn_00e39e60(e: &mut Engine, this: Ptr<HkpTyremarksWheel>, index: i32) -> Ptr {
    let current = e.get(this, HkpTyremarksWheel::m_current_position);
    let count = e.get(this, HkpTyremarksWheel::m_num_points);
    let slot = remainder(current.wrapping_add(index), count, 0x00e3_9e60);
    let data = e.get(this, HkpTyremarksWheel::m_tyremark_points);
    Ptr::new(((slot as u32) << 5).wrapping_add(data.addr()))
}

// Translated from 00e39e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The strength of a tyre-mark point (the map gives it no name): the float at `+0x1C` (the
/// right edge's fourth lane), returned on the x87 stack (an `FLD`, which quiets a signalling
/// NaN).
pub fn fn_00e39e80(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x1c))
}

// Translated from 00e39e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTyremarksInfo::getWheelTyremarksStrips` (Xbox PDB), `(this, vehicle, wheel, strips)`:
/// copies the points of wheel number `wheel` (32 bytes each) to `strips`, point `i` being the
/// one `i` places after the wheel's current position (`00e39e60`), for every point of the
/// ring (the count is signed). The vehicle word is not read.
pub fn get_wheel_tyremarks_strips(
    e: &mut Engine,
    this: Ptr<HkpTyremarksInfo>,
    _unused_1: u32,
    wheel: i32,
    strips: Ptr,
) {
    let wheels = e.get(this, HkpTyremarksInfo::m_tyremarks_wheel);
    let wheel = Ptr::<HkpTyremarksWheel>::new(
        e.mem
            .u32(wheels.addr().wrapping_add((wheel as u32).wrapping_mul(4))),
    );
    let count = e.get(wheel, HkpTyremarksWheel::m_num_points);
    let mut out = strips.addr();
    let mut i = 0;
    while i < count {
        let point = e.call(GET_TYREMARK_POINT, &args![wheel, i]).u32();
        let bytes = e.mem.bytes(point, POINT_SIZE);
        e.mem.write(out, &bytes);
        i += 1;
        out = out.wrapping_add(POINT_SIZE);
    }
}

// Translated from 00e39ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTyremarksInfo::calcContentStatistics` (Xbox PDB), `(this, collector)` (the function ends
/// `RET 8`: the second stack word is not read): for every wheel, when its point array owns its
/// memory (bit 31 of the capacity word clear) or the collector's byte at `+0x10` is set, asks
/// `getAllocatedSize` (`00d06f80`, cdecl) for the size of the array's memory (capacity masked
/// with `3FFFFFFF`, times 32) and tells the collector (slot `0x14`) `(3, "...", data, size * 32,
/// allocated)`.
pub fn calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpTyremarksInfo>,
    collector: Ptr,
    _unused_1: u32,
) {
    let wheels = e.get(this, HkpTyremarksInfo::m_tyremarks_wheel);
    let count = e.get(this, HkpTyremarksInfo::m_tyremarks_wheel_size);
    let mut i = 0;
    while i < count {
        let wheel = Ptr::<HkpTyremarksWheel>::new(
            e.mem
                .u32(wheels.addr().wrapping_add((i as u32).wrapping_mul(4))),
        );
        let capacity = e.get(wheel, HkpTyremarksWheel::m_tyremark_points_capacity);
        if (capacity as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
            let data = e.get(wheel, HkpTyremarksWheel::m_tyremark_points);
            let allocated = e
                .call(
                    GET_ALLOCATED_SIZE,
                    &args![data, (capacity & 0x3fff_ffff) << 5],
                )
                .u32();
            let size = e.get(wheel, HkpTyremarksWheel::m_tyremark_points_size);
            e.vcall(
                collector.addr(),
                0x14,
                &args![3u32, POINTS_NAME, data, size << 5, allocated],
            );
        }
        i += 1;
    }
}

// Translated from 00e39f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a point to the wheel's ring buffer (the map gives it no name; that it is
/// `hkpTyremarksWheel::addTyremarkPoint` is unconfirmed): nothing happens when both the new
/// point's strength (`00e39e80`) and the previous point's (the one before the current
/// position, `(current + count - 1) % count`) compare equal to zero (`FUCOM`: a NaN is not
/// equal). Otherwise the 32 bytes are copied to the current position and the position moves
/// on, `(current + 1) % count`.
pub fn fn_00e39f50(e: &mut Engine, this: Ptr<HkpTyremarksWheel>, point: Ptr) {
    let strength = e.call(GET_POINT_STRENGTH, &args![point]).f32();
    let skip = if strength == 0.0 {
        let count = e.get(this, HkpTyremarksWheel::m_num_points);
        let current = e.get(this, HkpTyremarksWheel::m_current_position);
        let previous = remainder(
            current.wrapping_add(count).wrapping_sub(1),
            count,
            0x00e3_9f50,
        );
        let data = e.get(this, HkpTyremarksWheel::m_tyremark_points);
        let before = (previous as u32) << 5;
        let before = before.wrapping_add(data.addr());
        e.call(GET_POINT_STRENGTH, &args![before]).f32() == 0.0
    } else {
        false
    };
    if skip {
        return;
    }
    let current = e.get(this, HkpTyremarksWheel::m_current_position);
    let data = e.get(this, HkpTyremarksWheel::m_tyremark_points);
    let bytes = e.mem.bytes(point.addr(), POINT_SIZE);
    e.mem
        .write(((current as u32) << 5).wrapping_add(data.addr()), &bytes);
    let count = e.get(this, HkpTyremarksWheel::m_num_points);
    let next = remainder(
        e.get(this, HkpTyremarksWheel::m_current_position)
            .wrapping_add(1),
        count,
        0x00e3_9f50,
    );
    e.set(this, HkpTyremarksWheel::m_current_position, next);
}

// Translated from 00e39fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTyremarksInfo::updateTyremarksInfo` (Xbox PDB), `(this, time, vehicle)`. The vehicle
/// keeps its chassis body at `+0x18` (linear velocity at `+0x1B0`, rotation at `+0xF0`), its
/// data at `+0x1C` (wheel count, a signed byte, at `+0x20`, a vector at `+0x50`, the per-wheel
/// table of 0x28-byte entries at `+0x8C` with the wheel radius at `+8`) and its wheel states
/// at `+0x48` (0xE0 bytes each: a vector at `+0`, one at `+0x10`, the float at `+0xC8`).
///
/// With `rotated` = the chassis rotation applied to the vector at `data + 0x50`
/// (`hkVector4::setRotatedDir`, `00c7f700`) and `step` = velocity * `time` (lane by lane), every
/// wheel `i` gets one new point, added through `00e39f50`:
/// * strength `s`: `t` = state float - `m_minTyremarkEnergy` (x87, stored as `f32`); `s` = 0.0
///   unless `t > 0`; otherwise `t * (255.0 / (max - min))` (the divisor not rounded to `f32`)
///   stored as `f32`, and 255.0 when that exceeds 255;
/// * `radius` = the wheel's table float (an `FLD`/`FSTP` copy); `edge` = -0.5 * radius and
///   `side` = radius * 0.5, both stored as `f32`; `base` = 0.05 * (state vector at `+0x10`) +
///   state vector at `+0`;
/// * the left edge is `edge * rotated + base + step` with the fourth lane `s`;
/// * the right edge starts as `p = side * rotated + base` with lanes 0 and 3 plus `step`'s (x87
///   adds): `(p.x, p.y + step.y, p.z + step.z, s)` (SSE adds for y and z).
///
/// The point is built on the stack after zeroing it through `00e39e50`.
pub fn update_tyremarks_info(e: &mut Engine, this: Ptr<HkpTyremarksInfo>, time: f32, vehicle: Ptr) {
    let body = e.mem.u32(vehicle.addr() + 0x18);
    let velocity = load_vec(e, body + 0x1b0);
    let data = e.mem.u32(vehicle.addr() + 0x1c);
    let step: Vec4 = velocity.map(|lane| time * lane);
    let rotated = e.with_stack(0x10, |e, result| {
        e.call(SET_ROTATED_DIR, &args![result, body + 0xf0, data + 0x50]);
        load_vec(e, result.addr())
    });
    let mut count = i32::from(e.mem.u8(data + 0x20) as i8);
    let mut i = 0i32;
    while i < count {
        update_wheel(e, this, vehicle, i as u32, step, rotated);
        // The count is read again after every wheel.
        let data = e.mem.u32(vehicle.addr() + 0x1c);
        count = i32::from(e.mem.u8(data + 0x20) as i8);
        i += 1;
    }
}

/// One wheel of [`update_tyremarks_info`].
fn update_wheel(
    e: &mut Engine,
    this: Ptr<HkpTyremarksInfo>,
    vehicle: Ptr,
    index: u32,
    step: Vec4,
    rotated: Vec4,
) {
    let data = e.mem.u32(vehicle.addr() + 0x1c);
    let state = e
        .mem
        .u32(vehicle.addr() + 0x48)
        .wrapping_add(index.wrapping_mul(0xe0));
    let min = e.get(this, HkpTyremarksInfo::m_min_tyremark_energy);
    let max = e.get(this, HkpTyremarksInfo::m_max_tyremark_energy);
    let above = (f64::from(x87_quiet(e.mem.f32(state + 0xc8))) - f64::from(x87_quiet(min))) as f32;
    let strength = if above > 0.0 {
        let range = f64::from(x87_quiet(max)) - f64::from(x87_quiet(min));
        let scaled = (f64::from(above) * (STRENGTH_RANGE / range)) as f32;
        if f64::from(scaled) > STRENGTH_RANGE {
            STRENGTH_MAX
        } else {
            scaled
        }
    } else {
        0.0
    };
    let table = e.mem.u32(data + 0x8c);
    let radius = x87_quiet(e.mem.f32(table.wrapping_add(index.wrapping_mul(0x28)) + 8));
    let near_vector = load_vec(e, state + 0x10);
    let origin = load_vec(e, state);
    let base: Vec4 = std::array::from_fn(|lane| BASE_SCALE * near_vector[lane] + origin[lane]);
    let edge = (NEGATIVE_HALF * f64::from(radius)) as f32;
    let side = (f64::from(radius) * HALF) as f32;
    let mut centre: Vec4 = std::array::from_fn(|lane| side * rotated[lane] + base[lane]);
    centre[0] = (f64::from(centre[0]) + f64::from(step[0])) as f32;
    centre[3] = (f64::from(centre[3]) + f64::from(step[3])) as f32;
    let mut left: Vec4 = std::array::from_fn(|lane| edge * rotated[lane] + base[lane] + step[lane]);
    left[3] = strength;
    let right = [
        centre[0],
        x87_quiet(centre[1] + step[1]),
        x87_quiet(centre[2] + step[2]),
        strength,
    ];
    e.with_stack(POINT_SIZE, |e, point| {
        e.call(POINT_CONSTRUCTOR, &args![point]);
        store_vec(e, point.addr(), left);
        store_vec(e, point.addr() + 0x10, right);
        let wheels = e.get(this, HkpTyremarksInfo::m_tyremarks_wheel);
        let wheel = e.mem.u32(wheels.addr().wrapping_add(index.wrapping_mul(4)));
        e.call(ADD_TYREMARK_POINT, &args![wheel, point]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_9e60, fn_00e39e60(Ptr<HkpTyremarksWheel>, i32) -> Ptr),
        entry!(0x00e3_9e80, fn_00e39e80(Ptr) -> f32),
        entry!(
            0x00e3_9e90,
            get_wheel_tyremarks_strips(Ptr<HkpTyremarksInfo>, u32, i32, Ptr)
        ),
        entry!(
            0x00e3_9ee0,
            calc_content_statistics(Ptr<HkpTyremarksInfo>, Ptr, u32)
        ),
        entry!(0x00e3_9f50, fn_00e39f50(Ptr<HkpTyremarksWheel>, Ptr)),
        entry!(
            0x00e3_9fd0,
            update_tyremarks_info(Ptr<HkpTyremarksInfo>, f32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};

    /// A wheel with `count` points (each filled with its index in every float), the current
    /// position `current` and the capacity word `capacity`. Returns `(wheel, data)`.
    fn wheel(e: &mut Engine, count: i32, current: i32, capacity: u32) -> (u32, u32) {
        let wheel = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x20 * count.max(1) as u32);
        for point in 0..count.max(0) as u32 {
            for lane in 0..8 {
                e.mem.set_f32(data + 0x20 * point + 4 * lane, point as f32);
            }
        }
        e.mem.set_i32(wheel + 8, current);
        e.mem.set_i32(wheel + 0xc, count);
        e.mem.set_u32(wheel + 0x10, data);
        e.mem.set_u32(wheel + 0x14, count as u32);
        e.mem.set_u32(wheel + 0x18, capacity);
        (wheel, data)
    }

    #[test]
    fn point_lookup_wraps_round_the_ring() {
        let mut e = engine();
        let (wheel, data) = wheel(&mut e, 5, 3, 5);
        let at = |e: &mut Engine, index: i32| e.call(0x00e3_9e60, &args![wheel, index]).u32();
        assert_eq!(at(&mut e, 0), data + 3 * 32);
        assert_eq!(at(&mut e, 1), data + 4 * 32);
        assert_eq!(at(&mut e, 2), data);
        // A negative sum has a negative remainder (signed IDIV): -2 % 5 = -2.
        assert_eq!(at(&mut e, -5), (data as i32 - 2 * 32) as u32);
    }

    #[test]
    #[should_panic(expected = "integer divide error")]
    fn point_lookup_stops_on_an_empty_ring() {
        let mut e = engine();
        let (wheel, _) = wheel(&mut e, 0, 0, 0);
        e.call(0x00e3_9e60, &args![wheel, 1i32]);
    }

    #[test]
    fn point_strength_is_the_float_at_0x1c() {
        let mut e = engine();
        let point = e.mem.alloc(0x20);
        e.mem.set_f32(point + 0x1c, 2.5);
        assert_eq!(e.call(0x00e3_9e80, &args![point]).f32(), 2.5);
    }

    #[test]
    fn strips_copy_every_point_in_ring_order() {
        let mut e = engine();
        let (first, _) = wheel(&mut e, 2, 0, 2);
        let (second, _) = wheel(&mut e, 3, 1, 3);
        let wheels = e.mem.alloc(8);
        e.mem.set_u32(wheels, first);
        e.mem.set_u32(wheels + 4, second);
        let info = e.mem.alloc(0x18);
        e.mem.set_u32(info + 0x10, wheels);
        let strips = e.mem.alloc(0x60);
        e.call(0x00e3_9e90, &args![info, 0x1234u32, 1i32, strips]);
        // Points 1, 2, 0 of the second wheel.
        for (i, point) in [1.0f32, 2.0, 0.0].into_iter().enumerate() {
            assert_eq!(e.mem.f32(strips + 0x20 * i as u32), point);
            assert_eq!(e.mem.f32(strips + 0x20 * i as u32 + 0x1c), point);
        }
    }

    #[test]
    fn strips_of_an_empty_wheel_copy_nothing() {
        let mut e = engine();
        let (wheel, _) = wheel(&mut e, 0, 0, 0);
        let wheels = e.mem.alloc(4);
        e.mem.set_u32(wheels, wheel);
        let info = e.mem.alloc(0x18);
        e.mem.set_u32(info + 0x10, wheels);
        let strips = e.mem.alloc(0x20);
        e.mem.write(strips, &[0xaa; 0x20]);
        e.call(0x00e3_9e90, &args![info, 0u32, 0i32, strips]);
        assert_eq!(e.mem.u32(strips), 0xaaaa_aaaa);
    }

    #[test]
    fn statistics_skip_wheels_whose_memory_is_not_owned() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, GET_ALLOCATED_SIZE, 0x99);
        let (owned, owned_data) = wheel(&mut e, 4, 0, 6);
        let (borrowed, borrowed_data) = wheel(&mut e, 2, 0, 0x8000_0003);
        let wheels = e.mem.alloc(8);
        e.mem.set_u32(wheels, owned);
        e.mem.set_u32(wheels + 4, borrowed);
        let info = e.mem.alloc(0x18);
        e.mem.set_u32(info + 0x10, wheels);
        e.mem.set_i32(info + 0x14, 2);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, 0x0900_0014);
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, vtable);
        log_double(&mut e, &log, 0x0900_0014, 0);
        e.call(0x00e3_9ee0, &args![info, collector, 0x55u32]);
        assert_eq!(
            calls(&log, GET_ALLOCATED_SIZE),
            vec![vec![owned_data, 6 << 5]]
        );
        assert_eq!(
            calls(&log, 0x0900_0014),
            vec![vec![collector, 3, POINTS_NAME, owned_data, 4 << 5, 0x99]]
        );
        // With the collector's byte at +0x10 set the borrowed array is reported too, with the
        // capacity masked.
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00e3_9ee0, &args![info, collector, 0x55u32]);
        assert_eq!(
            calls(&log, GET_ALLOCATED_SIZE)[2],
            vec![borrowed_data, 3 << 5]
        );
        assert_eq!(calls(&log, 0x0900_0014).len(), 3);
    }

    /// A point whose strength (`+0x1C`) is `strength`, with `marker` in the first float.
    fn point(e: &mut Engine, marker: f32, strength: f32) -> u32 {
        let point = e.mem.alloc(0x20);
        e.mem.set_f32(point, marker);
        e.mem.set_f32(point + 0x1c, strength);
        point
    }

    #[test]
    fn adding_a_point_stores_it_and_advances_the_ring() {
        let mut e = engine();
        let (wheel, data) = wheel(&mut e, 3, 2, 3);
        let new = point(&mut e, 42.0, 5.0);
        e.call(0x00e3_9f50, &args![wheel, new]);
        assert_eq!(e.mem.f32(data + 2 * 32), 42.0);
        assert_eq!(e.mem.f32(data + 2 * 32 + 0x1c), 5.0);
        // The position wraps: (2 + 1) % 3.
        assert_eq!(e.mem.i32(wheel + 8), 0);
    }

    #[test]
    fn adding_a_zero_point_after_a_zero_point_is_skipped() {
        let mut e = engine();
        let (wheel, data) = wheel(&mut e, 3, 1, 3);
        // The previous point (index 0) has strength 0 (its marker value).
        let new = point(&mut e, 42.0, 0.0);
        e.call(0x00e3_9f50, &args![wheel, new]);
        assert_eq!(e.mem.f32(data + 32), 1.0);
        assert_eq!(e.mem.i32(wheel + 8), 1);
        // Negative zero compares equal to zero too.
        let negative = point(&mut e, 42.0, -0.0);
        e.call(0x00e3_9f50, &args![wheel, negative]);
        assert_eq!(e.mem.i32(wheel + 8), 1);
    }

    #[test]
    fn adding_a_zero_point_after_a_non_zero_point_is_stored() {
        let mut e = engine();
        let (wheel, data) = wheel(&mut e, 3, 2, 3);
        // The previous point (index 1) has strength 1.0.
        let new = point(&mut e, 42.0, 0.0);
        e.call(0x00e3_9f50, &args![wheel, new]);
        assert_eq!(e.mem.f32(data + 2 * 32), 42.0);
        assert_eq!(e.mem.i32(wheel + 8), 0);
    }

    #[test]
    fn adding_a_nan_point_is_stored() {
        let mut e = engine();
        let (wheel, data) = wheel(&mut e, 3, 1, 3);
        let new = point(&mut e, 42.0, f32::NAN);
        e.call(0x00e3_9f50, &args![wheel, new]);
        assert_eq!(e.mem.f32(data + 32), 42.0);
    }

    #[test]
    #[should_panic(expected = "integer divide error")]
    fn adding_a_point_to_an_empty_ring_stops() {
        let mut e = engine();
        let (wheel, _) = wheel(&mut e, 0, 0, 0);
        let new = point(&mut e, 1.0, 1.0);
        e.call(0x00e3_9f50, &args![wheel, new]);
    }

    /// A vehicle with two wheels: returns `(info, vehicle, wheels' data)` for `update`.
    /// Chassis velocity `(1, 2, 3, 4)`, rotated vector answered `(10, 20, 30, 40)`, energy
    /// range 3.0 to 5.0, wheel radius 0.25; wheel states: energy 7.0 / 3.5 / 1.0.
    struct Rig {
        e: Engine,
        info: u32,
        vehicle: u32,
        datas: Vec<u32>,
        wheels: Vec<u32>,
    }

    fn rig(energies: &[f32]) -> Rig {
        let mut e = engine();
        let body = e.mem.alloc(0x200);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(body + 0x1b0 + 4 * lane as u32, value);
        }
        let data = e.mem.alloc(0x100);
        e.mem.set_u8(data + 0x20, energies.len() as u8);
        let table = e.mem.alloc(0x28 * energies.len().max(1) as u32);
        for i in 0..energies.len() as u32 {
            e.mem.set_f32(table + 0x28 * i + 8, 0.25);
        }
        e.mem.set_u32(data + 0x8c, table);
        let states = e.mem.alloc(0xe0 * energies.len().max(1) as u32);
        for (i, energy) in energies.iter().enumerate() {
            let state = states + 0xe0 * i as u32;
            e.mem.set_f32(state + 0xc8, *energy);
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(state + 4 * lane as u32, value);
                e.mem.set_f32(state + 0x10 + 4 * lane as u32, 8.0 * value);
            }
        }
        let vehicle = e.mem.alloc(0x60);
        e.mem.set_u32(vehicle + 0x18, body);
        e.mem.set_u32(vehicle + 0x1c, data);
        e.mem.set_u32(vehicle + 0x48, states);
        let info = e.mem.alloc(0x18);
        e.mem.set_f32(info + 8, 3.0);
        e.mem.set_f32(info + 0xc, 5.0);
        let list = e.mem.alloc(4 * energies.len().max(1) as u32);
        let mut datas = Vec::new();
        let mut wheels = Vec::new();
        for i in 0..energies.len() as u32 {
            let (w, d) = wheel(&mut e, 2, 0, 2);
            // Clear the points so that the added ones are easy to see.
            e.mem.write(d, &[0; 0x40]);
            e.mem.set_u32(list + 4 * i, w);
            datas.push(d);
            wheels.push(w);
        }
        e.mem.set_u32(info + 0x10, list);
        e.mem.set_i32(info + 0x14, energies.len() as i32);
        e.register_double(SET_ROTATED_DIR, |e, a| {
            for (lane, value) in [10.0f32, 20.0, 30.0, 40.0].into_iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register_double(POINT_CONSTRUCTOR, |e, a| {
            e.mem.write(a[0], &[0; 0x20]);
            Ret::default()
        });
        Rig {
            e,
            info,
            vehicle,
            datas,
            wheels,
        }
    }

    fn point_words(e: &Engine, address: u32) -> Vec<f32> {
        (0..8).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    #[test]
    fn update_adds_a_point_for_every_wheel() {
        let mut rig = rig(&[7.0, 3.5, 1.0]);
        let log = new_log();
        log_double(&mut rig.e, &log, SET_ROTATED_DIR, 0);
        rig.e
            .call(0x00e3_9fd0, &args![rig.info, 2.0f32, rig.vehicle]);
        // The rotation is the chassis' (+0xF0) and the vector the data's (+0x50).
        let body = rig.e.mem.u32(rig.vehicle + 0x18);
        let data = rig.e.mem.u32(rig.vehicle + 0x1c);
        let rotation = calls(&log, SET_ROTATED_DIR);
        assert_eq!(rotation.len(), 1);
        assert_eq!(rotation[0][1..], [body + 0xf0, data + 0x50]);
        // The first two wheels got a point (the ring moved on); the third has strength 0 and
        // so does the previous point, so nothing was added.
        let positions: Vec<i32> = rig
            .wheels
            .iter()
            .map(|wheel| rig.e.mem.i32(wheel + 8))
            .collect();
        assert_eq!(positions, vec![1, 1, 0]);
    }

    #[test]
    fn update_builds_the_edges_from_the_wheel_state() {
        let mut rig = rig(&[7.0]);
        rig.e
            .call(0x00e3_9fd0, &args![rig.info, 2.0f32, rig.vehicle]);
        // Strength: (7 - 3) * 255 / 2 = 510, above 255, so 255. Radius 0.25: edge -0.125,
        // side 0.125. base = 0.05 * (8, 16, 24, 32) + (1, 2, 3, 4); step = 2 * (1, 2, 3, 4).
        let rotated = [10.0f32, 20.0, 30.0, 40.0];
        let near = [8.0f32, 16.0, 24.0, 32.0];
        let origin = [1.0f32, 2.0, 3.0, 4.0];
        let step = [2.0f32, 4.0, 6.0, 8.0];
        let base: Vec<f32> = (0..4).map(|i| 0.05f32 * near[i] + origin[i]).collect();
        let mut left: Vec<f32> = (0..4)
            .map(|i| -0.125f32 * rotated[i] + base[i] + step[i])
            .collect();
        left[3] = 255.0;
        let centre: Vec<f32> = (0..4).map(|i| 0.125f32 * rotated[i] + base[i]).collect();
        let right = [
            centre[0] + step[0],
            centre[1] + step[1],
            centre[2] + step[2],
            255.0,
        ];
        let written = point_words(&rig.e, rig.datas[0]);
        assert_eq!(written[..4], left[..4]);
        assert_eq!(written[4..], right);
    }

    #[test]
    fn update_scales_the_strength_between_zero_and_255() {
        // 3.5 - 3.0 = 0.5, times 255 / 2 = 63.75; 1.0 is below the minimum: 0.0.
        let rig_energies = [3.5, 1.0, 3.0, 5.0];
        let mut rig = rig(&rig_energies);
        rig.e
            .call(0x00e3_9fd0, &args![rig.info, 2.0f32, rig.vehicle]);
        let strengths: Vec<f32> = rig
            .datas
            .iter()
            .map(|data| rig.e.mem.f32(data + 0x1c))
            .collect();
        assert_eq!(strengths, vec![63.75, 0.0, 0.0, 255.0]);
        // The left edge's fourth lane is the strength as well.
        assert_eq!(rig.e.mem.f32(rig.datas[0] + 0xc), 63.75);
    }

    #[test]
    fn update_with_a_nan_energy_gives_zero_strength() {
        let mut rig = rig(&[f32::NAN]);
        rig.e
            .call(0x00e3_9fd0, &args![rig.info, 2.0f32, rig.vehicle]);
        assert_eq!(rig.e.mem.f32(rig.datas[0] + 0x1c), 0.0);
    }

    #[test]
    fn update_with_no_wheels_adds_nothing() {
        let mut rig = rig(&[]);
        rig.e
            .call(0x00e3_9fd0, &args![rig.info, 2.0f32, rig.vehicle]);
        assert!(rig.wheels.is_empty());
    }
}
