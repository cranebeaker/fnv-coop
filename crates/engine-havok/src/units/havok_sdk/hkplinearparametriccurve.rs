//! `Havok SDK/hkplinearparametriccurve.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpLinearParametricCurve` (the class name is the engine map's Xbox PDB method name) is a
//! polyline: an `hkArray` of 16-byte points at +0x20 and an `hkArray<float>` at +0x2C that
//! holds, for each point, the length of the path up to it. The curve's parameter is the
//! point index plus the fraction along the segment. The member names are the Havok SDK's and
//! are unconfirmed in the exe; the offsets are the exe's.
//!
//! The vector code is SSE: lanes are `f32`, the sum of the three lowest lanes of a product is
//! `l2 + (l1 + l0)` (the `SHUFPS`/`ADDSS` order), and a unit length is the hardware
//! `RSQRTSS` estimate (host CPU) with one Newton step and the `CMPEQSS`/`ANDNPS` zero guard.
//! Values the exe moves through the x87 stack (`FLD`/`FSTP`, `FILD`, `FISUB`) are computed in
//! `f64` and rounded to `f32` where the exe stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkarrayutil::HkArrayBase;
use crate::x87::rsqrt_estimate;

/// `hkMath::hkFloatToInt` (`00cbfef0`, cdecl, one `float` word): the integer part, in EAX.
const FLOAT_TO_INT: u32 = 0x00cb_fef0;
/// The thread's memory router (`00c85750`); word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90940` (cdecl `(allocator, array, element size)`): makes room for more elements.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkVector4::setTransformedPos` (`00c7f640`, thiscall, `RET 8`): writes the transform of the
/// point (second word) by the transform (first word) to `this`.
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `0.0f`.
const ZERO: u32 = 0x0101_1d78;
/// `1.0f`.
const ONE: u32 = 0x0101_2068;
/// `3.0f`.
const THREE: u32 = 0x0101_7718;
/// `0.5f`.
const HALF: u32 = 0x0101_6248;
/// `0.5` (`double`).
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `0.01f` and `0.99f`: the fractions `getNearestPoint` picks at a vertex.
const JUST_PAST: u32 = 0x0101_3ea4;
const JUST_BEFORE: u32 = 0x0106_682c;
/// `0.25f` and `0.25` (`double`): the part of the loop's ends that wraps around.
const QUARTER: u32 = 0x0101_622c;
const QUARTER_DOUBLE: u32 = 0x0102_90b0;
/// Four lanes of `1.0f`.
const ONES: u32 = 0x010e_c7f0;
/// A vector of weights (three lanes used) the binormal dots the tangent with.
const BINORMAL_WEIGHTS: u32 = 0x0126_8380;
/// `0.98` as a `double`: the dot product above which the tangent is taken as parallel.
const PARALLEL_LIMIT: u32 = 0x010d_5378;
/// The vtable of a curve made by the constructor `00e48030`.
const CURVE_VTABLE: u32 = 0x010e_80b4;
/// Size of a curve object.
const CURVE_SIZE: u32 = 0x40;
/// The capacity field's size bits; the top two bits are flags.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

layout! {
    /// `hkpLinearParametricCurve` as far as these functions read it.
    pub struct HkpLinearParametricCurve: 0x40 {
        /// `m_smoothingFactor`: the part of a segment, in parameter units, over which the
        /// tangent blends into the neighbouring segment's.
        0x08 m_smoothingFactor: f32,
        /// `m_closedLoop` (a `hkBool`, one byte).
        0x0C m_closedLoop: u8,
        /// `m_dirNotParallelToTangentAlongWholePath`: a vector (four lanes).
        0x10 m_dirNotParallelToTangentAlongWholePath: Inline<[f32; 4]>,
        /// `m_points`: `hkArray<hkVector4>`.
        0x20 m_points: Inline<HkArrayBase>,
        /// `m_distance`: `hkArray<hkReal>`, the path length up to each point.
        0x2C m_distance: Inline<HkArrayBase>,
    }
}

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr.wrapping_add(4 * lane as u32)))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr.wrapping_add(4 * lane as u32), *value);
    }
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

/// `SHUFPS v, v, 0` on a scalar: the value in all four lanes.
fn splat(value: f32) -> Lanes {
    [value; 4]
}

/// The sum of the three lowest lanes the way the `SHUFPS`/`ADDSS` pairs add them.
fn sum_xyz(v: Lanes) -> f32 {
    v[2] + (v[1] + v[0])
}

/// `1 / sqrt(sum)` the way the code computes it: `RSQRTSS`, `(0.5 * r) * (3 - (sum * r) * r)`,
/// and 0 when `sum == 0` (`CMPEQSS` then `ANDNPS`).
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// `(1 - t) * from + to * t`, the interpolation `MULPS`/`ADDPS` sequence.
fn lerp(e: &Engine, from: Lanes, to: Lanes, t: f32) -> Lanes {
    let ones = load(e, ONES);
    add(mul(sub(ones, splat(t)), from), mul(to, splat(t)))
}

fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FLOAT_TO_INT, &args![x87_quiet(value)]).i32()
}

fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

fn count(e: &Engine, this: Ptr<HkpLinearParametricCurve>) -> i32 {
    e.mem.i32(this.addr() + 0x24)
}

/// Address of point `index` (the index is shifted left by four in 32 bits).
fn point_address(e: &Engine, this: Ptr<HkpLinearParametricCurve>, index: i32) -> u32 {
    e.mem
        .u32(this.addr() + 0x20)
        .wrapping_add((index as u32).wrapping_shl(4))
}

fn point(e: &Engine, this: Ptr<HkpLinearParametricCurve>, index: i32) -> Lanes {
    load(e, point_address(e, this, index))
}

// Translated from 00e47400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::isClosedLoop` (Xbox PDB): the `hkBool` result goes through the
/// hidden result pointer (the byte at +0x0C is copied there); the pointer is returned.
pub fn hkp_linear_parametric_curve_is_closed_loop(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    result: Ptr,
) -> Ptr {
    let closed = e.get(this, HkpLinearParametricCurve::m_closedLoop);
    e.mem.set_u8(result.addr(), closed);
    result
}

// Translated from 00e47410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getStart` (Xbox PDB): the first parameter, `0.0` (`FLDZ`).
pub fn hkp_linear_parametric_curve_get_start(
    _e: &mut Engine,
    _this: Ptr<HkpLinearParametricCurve>,
) -> f32 {
    0.0
}

// Translated from 00e47440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getPoint` (Xbox PDB): the point at `parameter` into `result`.
/// The segment is the integer part of the parameter (0 for a negative one, the parameter itself
/// is not clamped); at the last point the point itself is copied.
pub fn hkp_linear_parametric_curve_get_point(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    parameter: f32,
    result: Ptr,
) {
    let index = if parameter < 0.0 {
        0
    } else {
        float_to_int(e, parameter)
    };
    let fraction = ((parameter as f64) - (index as f64)) as f32;
    let next = index.wrapping_add(1);
    if next >= count(e, this) {
        let from = point(e, this, index);
        store(e, result.addr(), from);
    } else {
        let from = point(e, this, index);
        let to = point(e, this, next);
        let blended = lerp(e, from, to, fraction);
        store(e, result.addr(), blended);
    }
}

// Translated from 00e474e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getNearestPoint` (Xbox PDB): the parameter of the point of the
/// curve nearest to `point` (returned in ST0), and that point in `result`. The search starts at
/// the segment of `parameter` (0 for a negative one) and walks backwards while the projection
/// falls before a segment and forwards while it falls past one; at a vertex it picks the
/// segment the point lies more along (fraction `0.99` of the first, or `0.01` of the next).
/// A closed loop that ends within `0.25` of its start or end asks itself (vtable +0x10) again
/// with the parameter wrapped around.
pub fn hkp_linear_parametric_curve_get_nearest_point(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    parameter: f32,
    query: Ptr,
    result: Ptr,
) -> f32 {
    let parameter = if parameter < 0.0 { 0.0 } else { parameter };
    let mut i = float_to_int(e, parameter);
    let total = count(e, this);
    let mut j = i.wrapping_add(1);
    if j >= total {
        j = total.wrapping_sub(1);
        i = j.wrapping_sub(1);
    }
    let q = load(e, query.addr());
    let mut fraction;
    loop {
        let from = point(e, this, i);
        let to = point(e, this, j);
        let d = sub(to, from);
        let along = sum_xyz(mul(sub(q, from), d));
        let length_squared = sum_xyz(mul(d, d));
        fraction = ((along as f64) / (length_squared as f64)) as f32;
        if fraction < 0.0 {
            if i == 0 {
                break;
            }
            i = i.wrapping_sub(1);
            j = j.wrapping_sub(1);
            continue;
        }
        let r = sub(q, to);
        let past = sum_xyz(mul(r, d));
        if past <= 0.0 || j.wrapping_add(1) >= total {
            break;
        }
        let after = point(e, this, j.wrapping_add(1));
        let d_next = sub(after, to);
        let along_next = sum_xyz(mul(d_next, r));
        if 0.0 < along_next {
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
            continue;
        }
        // The point is past the vertex but behind the next segment: take the segment it lies
        // more along.
        let inverse = inverse_length(e, sum_xyz(mul(d, d)));
        let inverse_next = inverse_length(e, sum_xyz(mul(d_next, d_next)));
        let a = sum_xyz(mul(mul(splat(inverse), d), r));
        let b = sum_xyz(mul(mul(splat(inverse_next), d_next), r));
        if a < -b {
            fraction = e.global(JUST_BEFORE);
        } else {
            fraction = e.global(JUST_PAST);
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        break;
    }
    let value = ((i as f64) + (fraction as f64)) as f32;
    let from = point(e, this, i);
    let to = point(e, this, j);
    let nearest = lerp(e, from, to, fraction);
    store(e, result.addr(), nearest);
    if e.get(this, HkpLinearParametricCurve::m_closedLoop) == 0 {
        return value;
    }
    let last = total.wrapping_sub(1) as f32 as f64;
    let quarter: f32 = e.global(QUARTER);
    let wrapped = if value < quarter {
        Some(last - (1.0 - value as f64))
    } else {
        let limit: f64 = e.global(QUARTER_DOUBLE);
        if last - limit < value as f64 {
            Some(1.0 - (last - value as f64))
        } else {
            None
        }
    };
    match wrapped {
        Some(wrapped) => e
            .vcall(this.addr(), 0x10, &args![wrapped as f32, query, result])
            .f32(),
        None => value,
    }
}

/// The unit tangent of a segment blended into the one next to it: `result` becomes
/// `normalize(unit(d) * weight + (1 - weight) * result)`.
fn blend_tangent(e: &mut Engine, result: Ptr, d: Lanes, weight: f32) {
    let inverse = inverse_length(e, sum_xyz(mul(d, d)));
    let ones = load(e, ONES);
    let unit = mul(splat(inverse), d);
    let current = load(e, result.addr());
    let blended = add(
        mul(unit, splat(weight)),
        mul(sub(ones, splat(weight)), current),
    );
    let inverse = inverse_length(e, sum_xyz(mul(blended, blended)));
    store(e, result.addr(), mul(splat(inverse), blended));
}

// Translated from 00e478d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getTangent` (Xbox PDB): the unit tangent of the segment at
/// `parameter` into `result` (lane 3 is the unnormalized difference of the points' lane 3). Within
/// `m_smoothingFactor` of the segment's start (and not on the first segment) or end (and not on
/// the last) it is blended with the neighbouring segment's tangent, weighted by
/// `0.5 * (smoothing - distance) / smoothing`, and renormalized.
pub fn hkp_linear_parametric_curve_get_tangent(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    parameter: f32,
    result: Ptr,
) {
    let width = x87_quiet(e.get(this, HkpLinearParametricCurve::m_smoothingFactor));
    let inverse_width = (1.0f64 / width as f64) as f32;
    let mut i = float_to_int(e, parameter);
    let total = count(e, this);
    let mut j = i.wrapping_add(1);
    if j < total {
        if i < 0 {
            i = 0;
            j = 1;
        }
    } else {
        j = total.wrapping_sub(1);
        i = j.wrapping_sub(1);
    }
    let fraction = ((parameter as f64) - (i as f64)) as f32;
    let d = sub(point(e, this, j), point(e, this, i));
    let inverse = inverse_length(e, sum_xyz(mul(d, d)));
    store(e, result.addr(), mul(splat(inverse), d));
    let half: f64 = e.global(HALF_DOUBLE);
    if width > fraction && i > 0 {
        let distance = (width as f64) - (fraction as f64);
        let weight = ((distance * half) * inverse_width as f64) as f32;
        j = j.wrapping_sub(1);
        i = i.wrapping_sub(1);
        let d = sub(point(e, this, j), point(e, this, i));
        blend_tangent(e, result, d, weight);
    }
    let behind = ((j as f64) - (parameter as f64)) as f32;
    if behind < width && !(j >= count(e, this).wrapping_sub(1)) {
        let distance = (width as f64) - (behind as f64);
        let weight = ((distance * half) * inverse_width as f64) as f32;
        let d = sub(
            point(e, this, j.wrapping_add(1)),
            point(e, this, i.wrapping_add(1)),
        );
        blend_tangent(e, result, d, weight);
    }
}

// Translated from 00e47c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getEnd` (Xbox PDB): the last parameter, the number of points
/// minus one (`FILD`, returned in ST0 exactly).
pub fn hkp_linear_parametric_curve_get_end(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
) -> f64 {
    count(e, this).wrapping_sub(1) as f64
}

// Translated from 00e47c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getLengthFromStart` (Xbox PDB): the path length at `parameter`,
/// by linear interpolation of `m_distance` over the segment (the last one for a parameter past
/// the end, the first for a negative one), returned in ST0 rounded to a float.
pub fn hkp_linear_parametric_curve_get_length_from_start(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    parameter: f32,
) -> f32 {
    let mut index = float_to_int(e, parameter);
    let last = count(e, this).wrapping_sub(1);
    let distances = e.mem.u32(this.addr() + 0x2c);
    let distance = |e: &Engine, index: i32| e.mem.f32(distances.wrapping_add((index as u32) << 2));
    let segment;
    if index >= last {
        index = last;
        segment =
            ((distance(e, index) as f64) - (distance(e, index.wrapping_sub(1)) as f64)) as f32;
    } else {
        if index < 0 {
            index = 0;
        }
        segment =
            ((distance(e, index.wrapping_add(1)) as f64) - (distance(e, index) as f64)) as f32;
    }
    ((((parameter as f64) - (index as f64)) * segment as f64) + distance(e, index) as f64) as f32
}

// Translated from 00e47cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getBinormal` (Xbox PDB): takes the tangent at `parameter`
/// (vtable +0x14). If its dot product with the weight vector `01268380` is below 0.98 the
/// result is the cross product of the stored direction (+0x10) and the tangent; otherwise it
/// is a vector with the tangent's components swapped and one negated along the two axes
/// where the tangent is largest (the axis it is smallest along is left out). The result is
/// normalized.
pub fn hkp_linear_parametric_curve_get_binormal(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    parameter: f32,
    result: Ptr,
) {
    e.with_stack(0x10, |e, tangent_ptr| {
        e.vcall(this.addr(), 0x14, &args![x87_quiet(parameter), tangent_ptr]);
        let tangent = load(e, tangent_ptr.addr());
        let weights = load(e, BINORMAL_WEIGHTS);
        let dot = sum_xyz(mul(weights, tangent));
        let limit: f64 = e.global(PARALLEL_LIMIT);
        let parallel = !((dot as f64) < limit);
        if !parallel {
            let direction = load(e, this.addr() + 0x10);
            let first = mul(
                [direction[1], direction[2], direction[0], direction[3]],
                [tangent[2], tangent[0], tangent[1], tangent[3]],
            );
            let second = mul(
                [direction[2], direction[0], direction[1], direction[3]],
                [tangent[1], tangent[2], tangent[0], tangent[3]],
            );
            store(e, result.addr(), sub(second, first));
        } else {
            let magnitude = |v: f32| f32::from_bits(x87_quiet_bits(v.to_bits()) & 0x7fff_ffff);
            let a0 = magnitude(tangent[0]);
            let a1 = magnitude(tangent[1]);
            let a2 = magnitude(tangent[2]);
            // `larger` is the index of the larger of the first two (ties and NaN: 1).
            let (larger, smaller, smallest_of_two) = if a0 > a1 {
                (0usize, 1usize, a1)
            } else {
                (1usize, 0usize, a0)
            };
            let other = if smallest_of_two > a2 { smaller } else { 2 };
            store(e, result.addr(), [0.0; 4]);
            let kept = x87_quiet(tangent[other]);
            e.mem.set_f32(result.addr() + 4 * larger as u32, kept);
            let negated = f32::from_bits(x87_quiet_bits(tangent[larger].to_bits()) ^ 0x8000_0000);
            e.mem.set_f32(result.addr() + 4 * other as u32, negated);
        }
        let v = load(e, result.addr());
        let inverse = inverse_length(e, sum_xyz(mul(v, v)));
        store(e, result.addr(), mul(splat(inverse), v));
    });
}

// Translated from 00e47e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::transformPoints` (Xbox PDB): transforms every point in place by
/// `transform` with `hkVector4::setTransformedPos` (`00c7f640`).
pub fn hkp_linear_parametric_curve_transform_points(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    transform: Ptr,
) {
    let mut remaining = count(e, this);
    let mut offset = 0u32;
    while remaining > 0 {
        let target = e.mem.u32(this.addr() + 0x20).wrapping_add(offset);
        e.call(SET_TRANSFORMED_POS, &args![target, transform, target]);
        offset = offset.wrapping_add(0x10);
        remaining -= 1;
    }
}

/// Asks the allocator for more room when the array is full (size equals capacity): the
/// memory router's allocator and `00c90940`.
fn grow_if_full(e: &mut Engine, array: Ptr<HkArrayBase>, element_size: i32) {
    let allocator = allocator(e);
    let capacity = e.get(array, HkArrayBase::m_capacityAndFlags) & CAPACITY_MASK;
    if e.get(array, HkArrayBase::m_size) as u32 == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

// Translated from 00e47e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the point at `point` (copying all four lanes) to the curve and appends the path
/// length up to it to `m_distance`: 0 for the first point, otherwise the previous length plus
/// the distance between the two points (`SQRTSS` of the squared differences of the three
/// lowest lanes). Returns the address of the new length. The engine map has no name for it.
pub fn fn_00e47e80(e: &mut Engine, this: Ptr<HkpLinearParametricCurve>, point: Ptr) -> u32 {
    let points = this.at(HkpLinearParametricCurve::m_points);
    let distances = this.at(HkpLinearParametricCurve::m_distance);
    grow_if_full(e, points, 0x10);
    let size = e.get(points, HkArrayBase::m_size);
    let data = e.get(points, HkArrayBase::m_data).addr();
    let copy = load(e, point.addr());
    store(e, data.wrapping_add((size as u32) << 4), copy);
    e.set(points, HkArrayBase::m_size, size.wrapping_add(1));
    let total = count(e, this);
    let value = if total == 1 {
        grow_if_full(e, distances, 4);
        0.0
    } else {
        let current = load(e, point.addr());
        let data = e.get(points, HkArrayBase::m_data).addr();
        let previous = load(e, data.wrapping_add((total.wrapping_sub(2) as u32) << 4));
        let difference = sub(current, previous);
        let length = sum_xyz(mul(difference, difference)).sqrt();
        let distance_count = e.get(distances, HkArrayBase::m_size);
        let distance_data = e.get(distances, HkArrayBase::m_data).addr();
        let last = e.mem.f32(
            distance_data
                .wrapping_add((distance_count as u32) << 2)
                .wrapping_sub(4),
        );
        let value = length + last;
        grow_if_full(e, distances, 4);
        x87_quiet(value)
    };
    let size = e.get(distances, HkArrayBase::m_size);
    let data = e.get(distances, HkArrayBase::m_data).addr();
    let slot = data.wrapping_add((size as u32) << 2);
    e.mem.set_f32(slot, value);
    e.set(distances, HkArrayBase::m_size, size.wrapping_add(1));
    slot
}

// Translated from 00e47fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::getPointsToDraw` (Xbox PDB): copies the points into the
/// `hkArray<hkVector4>` at `points`. An array too small is freed (allocator slot `0x10`, unless
/// it does not own its storage) and replaced by a block from slot `0x0C`; the capacity becomes
/// the byte count the allocator reports divided by 16 (signed). Its size becomes the number of
/// points.
pub fn hkp_linear_parametric_curve_get_points_to_draw(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
    points: Ptr<HkArrayBase>,
) {
    let allocator = allocator(e);
    let flags = e.get(points, HkArrayBase::m_capacityAndFlags);
    let capacity = flags & CAPACITY_MASK;
    if (capacity as i32) < count(e, this) {
        if (flags as i32) >= 0 {
            let data = e.get(points, HkArrayBase::m_data);
            e.vcall(allocator, 0x10, &args![data, capacity << 4]);
        }
        let bytes = (count(e, this) as u32) << 4;
        e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), bytes);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            e.set(points, HkArrayBase::m_data, Ptr::new(block));
            let got = e.mem.i32(size.addr());
            let capacity = (got.wrapping_add((got >> 31) & 0xf)) >> 4;
            e.set(points, HkArrayBase::m_capacityAndFlags, capacity as u32);
        });
    }
    let total = count(e, this);
    let destination = e.get(points, HkArrayBase::m_data).addr();
    e.set(points, HkArrayBase::m_size, total);
    let source = e.mem.u32(this.addr() + 0x20);
    let mut index = 0u32;
    while (index as i32) < total {
        let lanes = load(e, source.wrapping_add(index << 4));
        store(e, destination.wrapping_add(index << 4), lanes);
        index += 1;
    }
}

// Translated from 00e48030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (`__fastcall`; the engine map has no name for it): smoothing factor
/// `0.01`, the vtable `010e80b4`, the word at +6 set to 1, not closed, the direction vector
/// `(0, 1, 0, 0)`, and both arrays empty without owned storage (capacity `0x80000000`).
/// Returns `this`.
pub fn fn_00e48030(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
) -> Ptr<HkpLinearParametricCurve> {
    let zero: f32 = e.global(ZERO);
    let smoothing: f32 = e.global(JUST_PAST);
    let one: f32 = e.global(ONE);
    let base = this.addr();
    e.mem.set_f32(base + 8, x87_quiet(smoothing));
    e.mem.set_u32(base, CURVE_VTABLE);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u8(base + 0xc, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0x8000_0000);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x34, 0x8000_0000);
    store(e, base + 0x10, [zero, one, zero, zero]);
    this
}

// Translated from 00e48090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLinearParametricCurve::clone` (Xbox PDB, `__fastcall`): a new curve from the heap
/// allocator (slot 4, `0x40` bytes, its size recorded in the 16-bit word at +4), built with
/// `00e48030`, every point added with `fn_00e47e80`, the closed flag and the direction vector
/// copied (lanes 1 and 2 through the x87 stack, lane 3 reset to `0.0`), and the distance array
/// copied element by element (through the x87 stack) after sizing it like `getPointsToDraw`.
/// Then the smoothing factor is copied. Returns the copy.
pub fn hkp_linear_parametric_curve_clone(
    e: &mut Engine,
    this: Ptr<HkpLinearParametricCurve>,
) -> Ptr<HkpLinearParametricCurve> {
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![CURVE_SIZE]).u32();
    e.mem.set_u16(block + 4, CURVE_SIZE as u16);
    let copy = fn_00e48030(e, Ptr::new(block));
    let mut remaining = count(e, this);
    let mut offset = 0u32;
    while remaining > 0 {
        let source = e.mem.u32(this.addr() + 0x20).wrapping_add(offset);
        fn_00e47e80(e, copy, Ptr::new(source));
        offset = offset.wrapping_add(0x10);
        remaining -= 1;
    }
    let closed = e.get(this, HkpLinearParametricCurve::m_closedLoop);
    e.set(copy, HkpLinearParametricCurve::m_closedLoop, closed);
    let zero: f32 = e.global(ZERO);
    let direction = load(e, this.addr() + 0x10);
    store(
        e,
        copy.addr() + 0x10,
        [
            direction[0],
            x87_quiet(direction[1]),
            x87_quiet(direction[2]),
            zero,
        ],
    );
    let heap = allocator(e);
    let distances = copy.at(HkpLinearParametricCurve::m_distance);
    let wanted = e.mem.i32(this.addr() + 0x30);
    let flags = e.get(distances, HkArrayBase::m_capacityAndFlags);
    let capacity = flags & CAPACITY_MASK;
    if (capacity as i32) < wanted {
        if (flags as i32) >= 0 {
            let data = e.get(distances, HkArrayBase::m_data);
            e.vcall(heap, 0x10, &args![data, capacity << 2]);
        }
        let bytes = (wanted as u32) << 2;
        e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), bytes);
            let block = e.vcall(heap, 0xc, &args![size]).u32();
            e.set(distances, HkArrayBase::m_data, Ptr::new(block));
            let got = e.mem.i32(size.addr());
            let capacity = (got.wrapping_add((got >> 31) & 3)) >> 2;
            e.set(distances, HkArrayBase::m_capacityAndFlags, capacity as u32);
        });
    }
    let size = e.mem.i32(this.addr() + 0x30);
    let destination = e.get(distances, HkArrayBase::m_data).addr();
    e.set(distances, HkArrayBase::m_size, size);
    let source = e.mem.u32(this.addr() + 0x2c);
    let mut index = 0u32;
    while (index as i32) < size {
        let value = e.mem.f32(source.wrapping_add(index << 2));
        e.mem
            .set_f32(destination.wrapping_add(index << 2), x87_quiet(value));
        index += 1;
    }
    let smoothing = e.get(this, HkpLinearParametricCurve::m_smoothingFactor);
    e.set(
        copy,
        HkpLinearParametricCurve::m_smoothingFactor,
        x87_quiet(smoothing),
    );
    copy
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e47400,
            hkp_linear_parametric_curve_is_closed_loop(Ptr<HkpLinearParametricCurve>, Ptr) -> Ptr
        ),
        entry!(
            0x00e47410,
            hkp_linear_parametric_curve_get_start(Ptr<HkpLinearParametricCurve>) -> f32
        ),
        entry!(
            0x00e47440,
            hkp_linear_parametric_curve_get_point(Ptr<HkpLinearParametricCurve>, f32, Ptr)
        ),
        entry!(
            0x00e474e0,
            hkp_linear_parametric_curve_get_nearest_point(
                Ptr<HkpLinearParametricCurve>,
                f32,
                Ptr,
                Ptr,
            ) -> f32
        ),
        entry!(
            0x00e478d0,
            hkp_linear_parametric_curve_get_tangent(Ptr<HkpLinearParametricCurve>, f32, Ptr)
        ),
        entry!(
            0x00e47c20,
            hkp_linear_parametric_curve_get_end(Ptr<HkpLinearParametricCurve>) -> f64
        ),
        entry!(
            0x00e47c30,
            hkp_linear_parametric_curve_get_length_from_start(
                Ptr<HkpLinearParametricCurve>,
                f32,
            ) -> f32
        ),
        entry!(
            0x00e47cb0,
            hkp_linear_parametric_curve_get_binormal(Ptr<HkpLinearParametricCurve>, f32, Ptr)
        ),
        entry!(
            0x00e47e50,
            hkp_linear_parametric_curve_transform_points(Ptr<HkpLinearParametricCurve>, Ptr)
        ),
        entry!(
            0x00e47e80,
            fn_00e47e80(Ptr<HkpLinearParametricCurve>, Ptr) -> u32
        ),
        entry!(
            0x00e47fa0,
            hkp_linear_parametric_curve_get_points_to_draw(
                Ptr<HkpLinearParametricCurve>,
                Ptr<HkArrayBase>,
            )
        ),
        entry!(
            0x00e48030,
            fn_00e48030(Ptr<HkpLinearParametricCurve>) -> Ptr<HkpLinearParametricCurve>
        ),
        entry!(
            0x00e48090,
            hkp_linear_parametric_curve_clone(
                Ptr<HkpLinearParametricCurve>,
            ) -> Ptr<HkpLinearParametricCurve>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATOR_VTABLE: u32 = 0x0900_0000;
    const ALLOCATE_BY_VALUE: u32 = 0x0900_0101;
    const ALLOCATE_BY_POINTER: u32 = 0x0900_0102;
    const FREE: u32 = 0x0900_0103;
    const CURVE_TEST_VTABLE: u32 = 0x0900_0200;
    const SLOT_DOUBLE: u32 = 0x0900_0201;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn close(got: f32, want: f32) -> bool {
        (got - want).abs() <= 1e-5 * want.abs().max(1.0)
    }

    fn assert_close(got: Lanes, want: Lanes) {
        for lane in 0..4 {
            assert!(
                close(got[lane], want[lane]),
                "lane {lane}: {got:?} vs {want:?}"
            );
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(ONE, 1.0f32);
        e.set_global(JUST_PAST, 0.01f32);
        e.set_global(JUST_BEFORE, 0.99f32);
        e.set_global(QUARTER, 0.25f32);
        e.set_global(QUARTER_DOUBLE, 0.25f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(PARALLEL_LIMIT, 0.98f32 as f64);
        store(&mut e, ONES, [1.0; 4]);
        e.register(FLOAT_TO_INT, |_, a| Ret {
            eax: f32::from_bits(a[0]).floor() as i32 as u32,
            ..Ret::default()
        });
        e
    }

    /// A curve over `points`, with the point array exactly full.
    fn curve(e: &mut Engine, points: &[Lanes]) -> Ptr<HkpLinearParametricCurve> {
        let this = e.new_object::<HkpLinearParametricCurve>();
        let data = e.mem.alloc(16 * points.len() as u32);
        for (index, lanes) in points.iter().enumerate() {
            store(e, data + 16 * index as u32, *lanes);
        }
        e.mem.set_u32(this.addr() + 0x20, data);
        e.mem.set_u32(this.addr() + 0x24, points.len() as u32);
        e.mem.set_u32(this.addr() + 0x28, points.len() as u32);
        this
    }

    /// Gives the curve the distance array `values`.
    fn set_distances(e: &mut Engine, this: Ptr<HkpLinearParametricCurve>, values: &[f32]) {
        let data = e.mem.alloc(4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(data + 4 * index as u32, *value);
        }
        e.mem.set_u32(this.addr() + 0x2c, data);
        e.mem.set_u32(this.addr() + 0x30, values.len() as u32);
        e.mem.set_u32(this.addr() + 0x34, values.len() as u32);
    }

    /// The vtable slots at +0x10 and +0x14 of a curve go to a double that records its words
    /// and returns `answer` in ST0; `fill` is written over the third word (a result vector)
    /// when given.
    fn install_slots(
        e: &mut Engine,
        this: Ptr<HkpLinearParametricCurve>,
        answer: f64,
        fill: Option<Lanes>,
    ) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(SLOT_DOUBLE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            if let Some(lanes) = fill {
                store(e, a[2], lanes);
            }
            Ret {
                st0: answer,
                ..Ret::default()
            }
        });
        e.put_vtable(CURVE_TEST_VTABLE, &[0, 0, 0, 0, SLOT_DOUBLE, SLOT_DOUBLE]);
        e.mem.set_u32(this.addr(), CURVE_TEST_VTABLE);
        log
    }

    /// Installs the memory router whose allocator hands out heap blocks: slot 4 takes the size
    /// by value, slot `0xC` takes a pointer to the size and rounds it up by 8, slot `0x10`
    /// frees (recorded as `[address, bytes]`).
    fn install_allocator(e: &mut Engine) -> Log {
        let frees: Log = Rc::new(RefCell::new(vec![]));
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, ALLOCATE_BY_VALUE, 0, ALLOCATE_BY_POINTER, FREE],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATE_BY_VALUE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register_double(ALLOCATE_BY_POINTER, |e, a| {
            let size = e.mem.u32(a[1]) + 8;
            e.mem.set_u32(a[1], size);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        let sink = frees.clone();
        e.register_double(FREE, move |_, a| {
            sink.borrow_mut().push(vec![a[1], a[2]]);
            Ret::default()
        });
        frees
    }

    /// `00c90940` made to grow the array: a fresh block four elements larger, old elements
    /// copied, capacity set. Records `[array, element size]`.
    fn install_reserve(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push(vec![a[1], a[2]]);
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(element * (size + 4));
            let old = e.mem.u32(array);
            for byte in 0..element * size {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, size + 4);
            Ret::default()
        });
        log
    }

    #[test]
    fn is_closed_loop_copies_the_flag_byte_and_returns_the_pointer() {
        let mut e = engine();
        let this = curve(&mut e, &[]);
        let out = e.mem.alloc(8);
        e.mem.set_u8(this.addr() + 0xc, 1);
        assert_eq!(e.call(0x00e47400, &args![this, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u8(this.addr() + 0xc, 0);
        e.call(0x00e47400, &args![this, out]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn get_start_is_zero() {
        let mut e = engine();
        let this = curve(&mut e, &[]);
        assert_eq!(e.call(0x00e47410, &args![this]).f32(), 0.0);
    }

    #[test]
    fn get_point_interpolates_and_copies_at_the_last_point() {
        let mut e = engine();
        let this = curve(
            &mut e,
            &[[0.0; 4], [10.0, 20.0, 30.0, 40.0], [1.0, 2.0, 3.0, 4.0]],
        );
        let out = e.mem.alloc(16);
        e.call(0x00e47440, &args![this, 0.25f32, out]);
        assert_eq!(load(&e, out), [2.5, 5.0, 7.5, 10.0]);
        e.call(0x00e47440, &args![this, 1.5f32, out]);
        assert_eq!(load(&e, out), [5.5, 11.0, 16.5, 22.0]);
        e.call(0x00e47440, &args![this, 2.0f32, out]);
        assert_eq!(load(&e, out), [1.0, 2.0, 3.0, 4.0]);
        // A negative parameter uses segment 0 with the negative fraction.
        e.call(0x00e47440, &args![this, -3.0f32, out]);
        assert_eq!(load(&e, out), [-30.0, -60.0, -90.0, -120.0]);
    }

    fn corner_curve(e: &mut Engine) -> Ptr<HkpLinearParametricCurve> {
        curve(
            e,
            &[[0.0; 4], [10.0, 0.0, 0.0, 0.0], [10.0, 10.0, 0.0, 0.0]],
        )
    }

    fn nearest(
        e: &mut Engine,
        this: Ptr<HkpLinearParametricCurve>,
        start: f32,
        at: Lanes,
    ) -> (f32, Lanes) {
        let query = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        store(e, query, at);
        let value = e.call(0x00e474e0, &args![this, start, query, out]).f32();
        (value, load(e, out))
    }

    #[test]
    fn get_nearest_point_projects_onto_the_first_segment() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        let (value, point) = nearest(&mut e, this, 0.0, [4.0, 5.0, 0.0, 0.0]);
        assert!(close(value, 0.4));
        assert_close(point, [4.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn get_nearest_point_walks_forward_to_the_next_segment() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        let (value, point) = nearest(&mut e, this, 0.0, [12.0, 5.0, 0.0, 0.0]);
        assert!(close(value, 1.5));
        assert_close(point, [10.0, 5.0, 0.0, 0.0]);
    }

    #[test]
    fn get_nearest_point_before_the_start_extrapolates_the_first_segment() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        let (value, point) = nearest(&mut e, this, -1.0, [-5.0, 0.0, 0.0, 0.0]);
        assert!(close(value, -0.5));
        assert_close(point, [-5.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn get_nearest_point_walks_back_when_the_start_is_too_far() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        // Starting at segment 1 the projection is before it: step back to segment 0.
        let (value, point) = nearest(&mut e, this, 1.0, [4.0, -3.0, 0.0, 0.0]);
        assert!(close(value, 0.4));
        assert_close(point, [4.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn get_nearest_point_at_a_vertex_picks_the_segment_it_lies_more_along() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        // Beyond the corner and behind the second segment, more along the first.
        let (value, point) = nearest(&mut e, this, 0.0, [11.0, -3.0, 0.0, 0.0]);
        assert!(close(value, 0.99));
        assert_close(point, [9.9, 0.0, 0.0, 0.0]);
        // More along the second segment: 0.01 into it.
        let (value, point) = nearest(&mut e, this, 0.0, [13.0, -2.0, 0.0, 0.0]);
        assert!(close(value, 1.01));
        assert_close(point, [10.0, 0.1, 0.0, 0.0]);
    }

    #[test]
    fn get_nearest_point_of_a_closed_loop_wraps_around_near_the_ends() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        e.mem.set_u8(this.addr() + 0xc, 1);
        let log = install_slots(&mut e, this, 7.5, None);
        // Near the start (0.1 < 0.25): asks again with (end - (1 - 0.1)).
        let query = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        store(&mut e, query, [1.0, 3.0, 0.0, 0.0]);
        let value = e.call(0x00e474e0, &args![this, 0.0f32, query, out]).f32();
        assert_eq!(value, 7.5);
        let words = log.borrow()[0].clone();
        assert_eq!(words[0], this.addr());
        assert!(close(f32::from_bits(words[1]), 1.1));
        assert_eq!(&words[2..], &[query, out]);
        // Near the end (1.9 > 2 - 0.25): asks again with (1 - (end - 1.9)).
        store(&mut e, query, [10.0, 9.0, 0.0, 0.0]);
        let value = e.call(0x00e474e0, &args![this, 1.0f32, query, out]).f32();
        assert_eq!(value, 7.5);
        let words = log.borrow()[1].clone();
        assert!(close(f32::from_bits(words[1]), 0.9));
        // In the middle the parameter is returned as it is.
        store(&mut e, query, [4.0, 5.0, 0.0, 0.0]);
        let value = e.call(0x00e474e0, &args![this, 0.0f32, query, out]).f32();
        assert!(close(value, 0.4));
        assert_eq!(log.borrow().len(), 2);
    }

    fn tangent_curve(e: &mut Engine) -> Ptr<HkpLinearParametricCurve> {
        let this = curve(
            e,
            &[
                [0.0; 4],
                [10.0, 0.0, 0.0, 0.0],
                [10.0, 10.0, 0.0, 0.0],
                [10.0, 10.0, 10.0, 0.0],
            ],
        );
        e.mem.set_f32(this.addr() + 8, 0.5);
        this
    }

    fn tangent(e: &mut Engine, this: Ptr<HkpLinearParametricCurve>, parameter: f32) -> Lanes {
        let out = e.mem.alloc(16);
        e.call(0x00e478d0, &args![this, parameter, out]);
        load(e, out)
    }

    #[test]
    fn get_tangent_is_the_unit_direction_of_the_segment() {
        let mut e = engine();
        let this = tangent_curve(&mut e);
        assert_close(tangent(&mut e, this, 0.25), [1.0, 0.0, 0.0, 0.0]);
        // Past the last point the last segment is used.
        assert_close(tangent(&mut e, this, 5.0), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn get_tangent_blends_at_a_vertex_with_both_neighbours() {
        let mut e = engine();
        let this = tangent_curve(&mut e);
        // Exactly at the vertex of segments 0 and 1: weight 0.5 of the previous direction
        // (1,0,0), then (the exe reuses the lowered index) weight 0.5 of segment 1 again.
        assert_close(tangent(&mut e, this, 1.0), [0.3826834, 0.9238795, 0.0, 0.0]);
    }

    #[test]
    fn get_tangent_blends_with_the_next_segment_near_its_end() {
        let mut e = engine();
        let this = tangent_curve(&mut e);
        // 0.1 before the end of segment 1: weight 0.4 of the next direction (0,0,1).
        assert_close(
            tangent(&mut e, this, 1.9),
            [0.0, 0.83205029, 0.5547002, 0.0],
        );
    }

    #[test]
    fn get_end_is_the_point_count_minus_one() {
        let mut e = engine();
        let this = tangent_curve(&mut e);
        assert_eq!(e.call(0x00e47c20, &args![this]).f64(), 3.0);
    }

    #[test]
    fn get_length_from_start_interpolates_the_distances() {
        let mut e = engine();
        let this = tangent_curve(&mut e);
        set_distances(&mut e, this, &[0.0, 10.0, 25.0, 30.0]);
        let length =
            |e: &mut Engine, parameter: f32| e.call(0x00e47c30, &args![this, parameter]).f32();
        assert_eq!(length(&mut e, 1.5), 17.5);
        assert_eq!(length(&mut e, 0.0), 0.0);
        // Past the end the last segment is extended.
        assert_eq!(length(&mut e, 5.0), 40.0);
        // Before the start the first segment is extended.
        assert_eq!(length(&mut e, -2.0), -20.0);
    }

    #[test]
    fn get_binormal_is_the_cross_product_with_the_stored_direction() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        store(&mut e, BINORMAL_WEIGHTS, [1.0, 0.0, 0.0, 0.0]);
        store(&mut e, this.addr() + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let log = install_slots(&mut e, this, 0.0, Some([0.0, 1.0, 0.0, 0.0]));
        let out = e.mem.alloc(16);
        e.call(0x00e47cb0, &args![this, 0.75f32, out]);
        assert_close(load(&e, out), [1.0, 0.0, 0.0, 0.0]);
        let words = log.borrow()[0].clone();
        assert_eq!(words[0], this.addr());
        assert_eq!(f32::from_bits(words[1]), 0.75);
    }

    #[test]
    fn get_binormal_of_a_tangent_along_the_weights_swaps_two_components() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        store(&mut e, BINORMAL_WEIGHTS, [1.0, 1.0, 1.0, 0.0]);
        let out = e.mem.alloc(16);
        // The smallest component is the third: the other two swap, one negated.
        install_slots(&mut e, this, 0.0, Some([0.5, 0.6, 0.1, 0.0]));
        e.call(0x00e47cb0, &args![this, 0.0f32, out]);
        assert_close(load(&e, out), [-0.6 / 0.781025, 0.5 / 0.781025, 0.0, 0.0]);
        // The smallest component is the first: the third takes its place.
        install_slots(&mut e, this, 0.0, Some([0.1, 0.6, 0.5, 0.0]));
        e.call(0x00e47cb0, &args![this, 0.0f32, out]);
        assert_close(load(&e, out), [0.0, 0.5 / 0.781025, -0.6 / 0.781025, 0.0]);
    }

    #[test]
    fn transform_points_calls_the_vector_transform_on_every_point_in_place() {
        let mut e = engine();
        let this = corner_curve(&mut e);
        let log: Log = Rc::new(RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(SET_TRANSFORMED_POS, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        e.call(0x00e47e50, &args![this, transform]);
        let data = e.mem.u32(this.addr() + 0x20);
        let want: Vec<Vec<u32>> = (0..3)
            .map(|k| vec![data + 16 * k, transform, data + 16 * k])
            .collect();
        assert_eq!(*log.borrow(), want);
        // No points, no calls.
        log.borrow_mut().clear();
        let empty = curve(&mut e, &[]);
        e.call(0x00e47e50, &args![empty, transform]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn add_point_appends_the_point_and_the_path_length() {
        let mut e = engine();
        install_allocator(&mut e);
        let reserves = install_reserve(&mut e);
        let this = e.new_object::<HkpLinearParametricCurve>();
        fn_00e48030(&mut e, this);
        let point = e.mem.alloc(16);
        let mut slots = vec![];
        for lanes in [[0.0; 4], [3.0, 4.0, 0.0, 9.0], [3.0, 4.0, 12.0, 0.0]] {
            store(&mut e, point, lanes);
            slots.push(e.call(0x00e47e80, &args![this, point]).u32());
        }
        assert_eq!(e.mem.i32(this.addr() + 0x24), 3);
        assert_eq!(e.mem.i32(this.addr() + 0x30), 3);
        let data = e.mem.u32(this.addr() + 0x20);
        assert_eq!(load(&e, data + 16), [3.0, 4.0, 0.0, 9.0]);
        assert_eq!(load(&e, data + 32), [3.0, 4.0, 12.0, 0.0]);
        let distances = e.mem.u32(this.addr() + 0x2c);
        let lengths: Vec<f32> = (0..3).map(|k| e.mem.f32(distances + 4 * k)).collect();
        assert_eq!(lengths, vec![0.0, 5.0, 17.0]);
        assert_eq!(slots, vec![distances, distances + 4, distances + 8]);
        // Both arrays were empty and unowned: the first add asked for room for the points and
        // then for the distances; the room made covers the next two without asking again.
        let asked: Vec<u32> = reserves.borrow().iter().map(|words| words[0]).collect();
        assert_eq!(asked, vec![this.addr() + 0x20, this.addr() + 0x2c]);
    }

    #[test]
    fn get_points_to_draw_replaces_a_small_array_and_copies_the_points() {
        let mut e = engine();
        let frees = install_allocator(&mut e);
        let this = corner_curve(&mut e);
        let out = e.mem.alloc(12);
        // Owned storage, too small: freed with its size, replaced by a block of 3 points
        // (48 bytes plus the allocator's 8 rounds down to 3 elements).
        let old = e.mem.alloc(16);
        e.mem.set_u32(out, old);
        e.mem.set_u32(out + 8, 1);
        e.call(0x00e47fa0, &args![this, out]);
        assert_eq!(*frees.borrow(), vec![vec![old, 16]]);
        assert_eq!(e.mem.i32(out + 4), 3);
        assert_eq!(e.mem.u32(out + 8), 3);
        let data = e.mem.u32(out);
        assert_eq!(load(&e, data + 16), [10.0, 0.0, 0.0, 0.0]);
        assert_eq!(load(&e, data + 32), [10.0, 10.0, 0.0, 0.0]);
        // Large enough already: only the copy.
        frees.borrow_mut().clear();
        e.mem.set_u32(this.addr() + 0x24, 2);
        e.call(0x00e47fa0, &args![this, out]);
        assert_eq!(e.mem.i32(out + 4), 2);
        assert_eq!(e.mem.u32(out), data);
        assert!(frees.borrow().is_empty());
        // Storage it does not own is not freed.
        let borrowed = e.mem.alloc(12);
        e.mem.set_u32(borrowed + 8, 0x8000_0000);
        e.call(0x00e47fa0, &args![this, borrowed]);
        assert!(frees.borrow().is_empty());
        assert_eq!(e.mem.i32(borrowed + 4), 2);
    }

    #[test]
    fn the_constructor_sets_the_defaults() {
        let mut e = engine();
        let this = e.new_object::<HkpLinearParametricCurve>();
        e.mem.set_u8(this.addr() + 0xc, 7);
        assert_eq!(e.call(0x00e48030, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), CURVE_VTABLE);
        assert_eq!(e.mem.f32(this.addr() + 8), 0.01);
        assert_eq!(e.mem.u16(this.addr() + 6), 1);
        assert_eq!(e.mem.u8(this.addr() + 0xc), 0);
        assert_eq!(load(&e, this.addr() + 0x10), [0.0, 1.0, 0.0, 0.0]);
        for offset in [0x20, 0x24, 0x2c, 0x30] {
            assert_eq!(e.mem.u32(this.addr() + offset), 0);
        }
        assert_eq!(e.mem.u32(this.addr() + 0x28), 0x8000_0000);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 0x8000_0000);
    }

    #[test]
    fn clone_copies_the_points_flag_direction_distances_and_smoothing() {
        let mut e = engine();
        let frees = install_allocator(&mut e);
        install_reserve(&mut e);
        let this = corner_curve(&mut e);
        // More distances than the copy's own, so its distance block is replaced.
        set_distances(&mut e, this, &[0.0, 11.0, 22.0, 33.0, 44.0, 55.0]);
        e.mem.set_u8(this.addr() + 0xc, 1);
        e.mem.set_f32(this.addr() + 8, 0.25);
        store(&mut e, this.addr() + 0x10, [0.5, 0.25, 2.0, 7.0]);
        let copy = e
            .call(0x00e48090, &args![this])
            .ptr::<HkpLinearParametricCurve>();
        assert_ne!(copy.addr(), this.addr());
        assert_eq!(e.mem.u16(copy.addr() + 4), 0x40);
        assert_eq!(e.mem.u32(copy.addr()), CURVE_VTABLE);
        assert_eq!(e.mem.i32(copy.addr() + 0x24), 3);
        let data = e.mem.u32(copy.addr() + 0x20);
        assert_eq!(load(&e, data + 16), [10.0, 0.0, 0.0, 0.0]);
        assert_eq!(load(&e, data + 32), [10.0, 10.0, 0.0, 0.0]);
        assert_eq!(e.mem.u8(copy.addr() + 0xc), 1);
        assert_eq!(load(&e, copy.addr() + 0x10), [0.5, 0.25, 2.0, 0.0]);
        assert_eq!(e.mem.i32(copy.addr() + 0x30), 6);
        let distances = e.mem.u32(copy.addr() + 0x2c);
        let lengths: Vec<f32> = (0..6).map(|k| e.mem.f32(distances + 4 * k)).collect();
        assert_eq!(lengths, vec![0.0, 11.0, 22.0, 33.0, 44.0, 55.0]);
        // The copy's block (room for 4 after the three added points) was freed first: 4 * 4 bytes.
        assert_eq!(frees.borrow().len(), 1);
        assert_eq!(frees.borrow()[0][1], 16);
        assert_eq!(e.mem.u32(copy.addr() + 0x34), 8);
        assert_eq!(e.mem.f32(copy.addr() + 8), 0.25);
    }
}
