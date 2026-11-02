//! `Havok SDK/hkpmeshweldingutility.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMeshWeldingUtility` (Xbox PDB) computes the welding information of a triangle mesh:
//! for every triangle and each of its three edges, a 5-bit code naming the angle between the
//! triangle and the neighbour it shares the edge with. The helpers below (names are
//! `fn_<address>`, the engine map has none) work on `hkVector4` values read lane by lane;
//! the SSE code is computed in `f32`, the x87 code in `f64` and rounded to `f32` where the exe
//! stores a float. The member and parameter names are guesses from the Havok SDK and not
//! confirmed in the exe.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `double` `0.20..` (`010cd5a8`) and `double` `3.14159..` (`0101ff40`): the quantizer maps
/// an angle to the first index `i` with `i * ANGLE_STEP - ANGLE_BASE >= angle`.
const ANGLE_STEP: u32 = 0x010c_d5a8;
const ANGLE_BASE: u32 = 0x0101_ff40;
/// `float` `0.001` (`01017d00`), the tolerance of the vertex comparisons.
const VERTEX_TOLERANCE: u32 = 0x0101_7d00;
/// `double` `0.05` (`0103c7c8`): the tolerance of the perpendicularity test.
const PERPENDICULAR_LIMIT: u32 = 0x0103_c7c8;
/// `double` `0.6` (`010290b8`): the smallest edge span `getNearestEdgeAngle` accepts.
const SPAN_LIMIT: u32 = 0x0102_90b8;
/// `float` `pi` (`0102b3c8`): the default angle.
const PI: u32 = 0x0102_b3c8;
/// `float` `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) of the vector
/// normalizer: `0.5 * r * (3 - x * r * r)` after `RSQRTSS`.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The four-lane vector `1.0` (`010ec7f0`).
const ONES: u32 = 0x010e_c7f0;
/// Sign mask of the first three lanes (`010c4c00`), flips a normal.
const NORMAL_SIGN_MASK: u32 = 0x010c_4c00;
/// `float` maximum (`0104da48`, `0x7f7fffee`) and minimum (`0104e148`, `0xff7fffee`): the
/// starting values of a bounding box.
const BOX_MINIMUM_START: u32 = 0x0104_da48;
const BOX_MAXIMUM_START: u32 = 0x0104_e148;
/// `float` `0.05` (`010181bc`): the margin of the prism around a neighbour triangle.
const PRISM_MARGIN: u32 = 0x0101_81bc;
/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c90940`, cdecl `(allocator, array, element size)`): grows the array.
const ARRAY_RESERVE: u32 = 0x00c9_0940;
/// `hkMath_atan2fApproximation` (`00cbff80`, cdecl `(y, x)`, result in ST0).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// `00ddecd0` (thiscall, `RET 0xC`, `(allocator, index, record)`): inserts a 12-byte record
/// into the array at its first argument.
const ARRAY_INSERT: u32 = 0x00dd_ecd0;
/// `00ddeb70` (cdecl `(data, low, high, word)`): sorts 12-byte records by their third float.
const SORT_RECORDS: u32 = 0x00dd_eb70;
/// `00ddce80` (cdecl `(result byte, a, b)`): whether the third floats of two records differ.
const RECORDS_DIFFER: u32 = 0x00dd_ce80;
/// `00ddd620` (cdecl `(v0, v1, v2, out)`): the normal of a triangle.
const TRIANGLE_NORMAL: u32 = 0x00dd_d620;
/// `00ddcdd0` (cdecl `(word, index, value)`): replaces the 5-bit field `index` of `word`.
const SET_FIELD: u32 = 0x00dd_cdd0;
/// `hkTransform::setMulInverseMul` (`00cb22f0`, thiscall, `RET 8`): `this = inverse(a) * b`.
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `00cb1600` (thiscall, `RET 8`), `00cc01c0` (thiscall, `RET 4`) and `00cc00e0` (thiscall,
/// no arguments): the message stream the assertion text is built with.
const STREAM_BEGIN: u32 = 0x00cb_1600;
const STREAM_TEXT: u32 = 0x00cc_01c0;
const STREAM_END: u32 = 0x00cc_00e0;
/// The pointer to the error handler object (`01268360`); its vtable slot `0xC` reports an
/// assertion `(1, id, message, file, line)`.
const ERROR_HANDLER: u32 = 0x0126_8360;
/// The message, the file name and the line of the assertion.
const ASSERT_MESSAGE: u32 = 0x010d_7f80;
const ASSERT_FILE: u32 = 0x010d_7f50;
const ASSERT_ID: u32 = 0x3f50_8c6a;
const ASSERT_LINE: u32 = 0x10f;

type Lanes = [f32; 4];

fn load(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at.wrapping_add(4 * lane as u32)))
}

fn store(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(at.wrapping_add(4 * lane as u32), *value);
    }
}

// The lane arithmetic keeps the SSE NaN rules (`crate::sse`): the first operand is the
// instruction's destination (`ADDPS a, b` is `add(a, b)`), whose NaN wins over the source's.
// Host `+`/`*` leave the choice of NaN to LLVM, which may swap the operands of a commutative
// operation depending on how the surrounding code inlines (605bfeb5's inlined memory accessors
// turned 00ddd5a0's NaN results around).

fn add(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::addps(a, b)
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::subps(a, b)
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::mulps(a, b)
}

fn splat(value: f32) -> Lanes {
    [value; 4]
}

/// `SHUFPS v, v, 0xD2`: `(z, x, y, w)`.
fn rotate_up(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

/// `SHUFPS v, v, 0xC9`: `(y, z, x, w)`.
fn rotate_down(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

/// The cross product `a x b` as the exe computes it:
/// `rotate_down(a) * rotate_up(b) - rotate_up(a) * rotate_down(b)`.
fn cross(a: Lanes, b: Lanes) -> Lanes {
    sub(
        mul(rotate_down(a), rotate_up(b)),
        mul(rotate_up(a), rotate_down(b)),
    )
}

/// The sum of the three lowest lanes of `v`, `v[2] + (v[1] + v[0])` as the SSE shuffles
/// and `ADDSS` add them.
fn sum3(v: Lanes) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

fn dot3(a: Lanes, b: Lanes) -> f32 {
    sum3(mul(a, b))
}

/// The scale of the exe's vector normalizer for a squared length `squared`:
/// `RSQRTSS`, then `0.5 * r * (3 - squared * r * r)`, and `0.0` when `squared == 0.0`.
fn normalizing_scale(e: &Engine, squared: f32) -> f32 {
    let r = rsqrt_estimate(squared);
    let zero = e.global::<f32>(ZERO);
    let three = e.global::<f32>(THREE);
    let half = e.global::<f32>(HALF);
    let value = (half * r) * (three - (squared * r) * r);
    if squared == zero {
        0.0
    } else {
        value
    }
}

/// `|a - b| <= tolerance` in each of the first three lanes (`SUBPS`, `ANDPS` with the abs
/// mask, `CMPLEPS`, `MOVMSKPS & 7 == 7`).
fn close3(a: Lanes, b: Lanes, tolerance: f32) -> bool {
    (0..3).all(|lane| (a[lane] - b[lane]).abs() <= tolerance)
}

// Translated from 00ddce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quantizes an angle (the engine map has no name): the first `i` in `0..31` whose float
/// `(i * 0.2087.. - 3.14159..) as f32` is not below `value` (a NaN `value` never is, so the
/// result is then 31). From there: `i` when `value` is not above 0 or equals that float
/// exactly, otherwise `i - 1`. Returns it in EAX.
pub fn fn_00ddce10(e: &mut Engine, value: f32) -> u32 {
    let step = e.global::<f64>(ANGLE_STEP);
    let base = e.global::<f64>(ANGLE_BASE);
    let value = x87_quiet(value);
    let mut index: i32 = 0;
    let candidate = loop {
        let candidate = ((index as f64) * step - base) as f32;
        // FCOM sets C0 for "below" and for "unordered".
        if candidate >= value {
            break candidate;
        }
        index += 1;
        if index >= 0x1f {
            return 0x1f;
        }
    };
    if !(0.0 < value) || candidate == value {
        index as u32
    } else {
        (index - 1) as u32
    }
}

// Translated from 00ddced0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two triangles have the same vertices in the same order (the engine map has no
/// name): writes 1 to the byte at `result` when the vectors at +0x20, +0x30 and +0x40 of
/// `a` and `b` agree within 0.001 in each of their first three lanes, otherwise 0. Returns
/// `result`.
pub fn fn_00ddced0(e: &mut Engine, result: Ptr, a: Ptr, b: Ptr) -> Ptr {
    let tolerance = e.global::<f32>(VERTEX_TOLERANCE);
    let same = [0x20, 0x30, 0x40].into_iter().all(|offset| {
        close3(
            load(e, a.addr() + offset),
            load(e, b.addr() + offset),
            tolerance,
        )
    });
    e.mem.set_u8(result.addr(), u8::from(same));
    result
}

// Translated from 00ddcf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether both points lie on a vertex of a triangle (the engine map has no name): writes 1
/// to the byte at `result` when the point at `p` and the point at `q` each agree within
/// 0.001 (three lanes) with one of the vertices at +0x20, +0x30, +0x40 of `triangle`,
/// otherwise 0. Returns `result`.
pub fn fn_00ddcf60(e: &mut Engine, result: Ptr, p: Ptr, q: Ptr, triangle: Ptr) -> Ptr {
    let tolerance = e.global::<f32>(VERTEX_TOLERANCE);
    let on_vertex = |e: &Engine, point: Ptr| {
        let point = load(e, point.addr());
        [0x20, 0x30, 0x40]
            .into_iter()
            .any(|offset| close3(load(e, triangle.addr() + offset), point, tolerance))
    };
    let both = on_vertex(e, p) && on_vertex(e, q);
    e.mem.set_u8(result.addr(), u8::from(both));
    result
}

// Translated from 00ddd040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two vectors are nearly perpendicular (the engine map has no name): writes 1 to the
/// byte at `result` when `|a . b|` (three lanes, `l2 + (l1 + l0)`) is below 0.05, otherwise
/// 0. Returns `result`.
pub fn fn_00ddd040(e: &mut Engine, result: Ptr, a: Ptr, b: Ptr) -> Ptr {
    let dot = dot3(load(e, a.addr()), load(e, b.addr()));
    let limit = e.global::<f64>(PERPENDICULAR_LIMIT);
    let perpendicular = (x87_quiet(dot).abs() as f64) < limit;
    e.mem.set_u8(result.addr(), u8::from(perpendicular));
    result
}

// Translated from 00ddd0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clips the segment from `a` to `b` against `count` planes (the engine map has no name),
/// cdecl with seven words: `(result, a, b, count, planes, out_a, out_b)`. For each plane
/// `(normal, w)` (16 bytes, the distance of a point is `normal . p + w`) with the distances
/// `da` of the current start and `db` of the current end: nothing changes unless
/// `da * db < 0`, but when both are on the outer side (`da >= 0` and `db >= 0`) the segment
/// is rejected (byte 0 at `result`, nothing else written). Otherwise the end (when `da <= 0`
/// or NaN) or the start is replaced by the point at `t = da / (da - db)`. After the planes
/// the byte at `result` is 1 and the floats at `out_a` and `out_b` receive the parameters of
/// the clipped start and end along the original segment (`(p - a) . d / (d . d)`). Returns
/// `result`.
pub fn fn_00ddd0b0(
    e: &mut Engine,
    result: Ptr,
    a: Ptr,
    b: Ptr,
    count: i32,
    planes: Ptr,
    out_a: Ptr,
    out_b: Ptr,
) -> Ptr {
    let mut start = load(e, a.addr());
    let mut end = load(e, b.addr());
    if count > 0 {
        let ones = load(e, ONES);
        for index in 0..count {
            let plane = load(e, planes.addr().wrapping_add(16 * index as u32));
            let w = x87_quiet(plane[3]);
            let start_distance = ((w as f64) + (x87_quiet(dot3(plane, start)) as f64)) as f32;
            let end_distance = ((w as f64) + (x87_quiet(dot3(end, plane)) as f64)) as f32;
            // C0 of FCOM: below zero or unordered.
            let start_below = !(start_distance >= 0.0);
            if !start_below {
                let end_below = !(end_distance >= 0.0);
                if !end_below {
                    e.mem.set_u8(result.addr(), 0);
                    return result;
                }
            }
            let product = (end_distance as f64) * (start_distance as f64);
            if !(product < 0.0) {
                continue;
            }
            let t = ((start_distance as f64) / ((start_distance as f64) - (end_distance as f64)))
                as f32;
            let t = splat(t);
            let point = add(mul(sub(ones, t), start), mul(t, end));
            if !(start_distance > 0.0) {
                end = point;
            } else {
                start = point;
            }
        }
    }
    let original = load(e, a.addr());
    let direction = sub(load(e, b.addr()), original);
    let length_squared = sum3(mul(direction, direction));
    let start_numerator = sum3(mul(sub(start, original), direction));
    let end_numerator = sum3(mul(sub(end, original), direction));
    e.mem.set_u8(result.addr(), 1);
    let start_parameter =
        ((x87_quiet(start_numerator) as f64) / (x87_quiet(length_squared) as f64)) as f32;
    e.mem.set_f32(out_a.addr(), start_parameter);
    let end_parameter =
        ((x87_quiet(end_numerator) as f64) / (x87_quiet(length_squared) as f64)) as f32;
    e.mem.set_f32(out_b.addr(), end_parameter);
    result
}

// Translated from 00ddd5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angle from three vectors (the engine map has no name), cdecl `(x, y, z)`:
/// `hkMath_atan2fApproximation(y . z, x . z)` with the dot products over three lanes
/// (`l2 + (l1 + l0)`), both passed through the x87 stack. Returns the result of `00cbff80`
/// in ST0.
pub fn fn_00ddd5a0(e: &mut Engine, x: Ptr, y: Ptr, z: Ptr) -> f32 {
    let z = load(e, z.addr());
    let along_y = dot3(load(e, y.addr()), z);
    let along_x = dot3(load(e, x.addr()), z);
    e.call(
        ATAN2_APPROXIMATION,
        &args![x87_quiet(along_y), x87_quiet(along_x)],
    )
    .f32()
}

// Translated from 00ddd940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The five planes of a prism around a triangle (the engine map has no name), cdecl
/// `(out, triangle, margin)`; `out` receives five 16-byte planes `(normal, w)`:
/// 0 `(n, -margin - n . v0)` with `n` the triangle normal (`00ddd620` of the vertices at
/// +0x20, +0x30, +0x40), 1 `(-n, -margin + n . v0)`, and for the edges `v1-v0`, `v2-v1`,
/// `v0-v2` the normalized `edge x n` with `w = -margin - plane . v` for `v0`, `v1`, `v2`
/// respectively. Returns `out`.
pub fn fn_00ddd940(e: &mut Engine, out: Ptr, triangle: Ptr, margin: f32) -> Ptr {
    let t = triangle.addr();
    let (v0_at, v1_at, v2_at) = (t + 0x20, t + 0x30, t + 0x40);
    let o = out.addr();
    e.with_stack(0x10, |e, normal_buffer| {
        e.call(TRIANGLE_NORMAL, &args![v0_at, v1_at, v2_at, normal_buffer]);
        let margin = -x87_quiet(margin);
        let (v0, v1, v2) = (load(e, v0_at), load(e, v1_at), load(e, v2_at));
        let n = load(e, normal_buffer.addr());
        let edge1 = sub(v1, v0);
        let edge0 = sub(v0, v2);
        let edge2 = sub(v2, v1);
        store(e, o, n);
        let distance = dot3(v0, n);
        e.mem.set_f32(
            o + 0xc,
            ((margin as f64) - (x87_quiet(distance) as f64)) as f32,
        );
        let mask = load(e, NORMAL_SIGN_MASK);
        let flipped: Lanes =
            std::array::from_fn(|lane| f32::from_bits(n[lane].to_bits() ^ mask[lane].to_bits()));
        store(e, o + 0x10, flipped);
        let flipped_distance = dot3(flipped, v0);
        e.mem.set_f32(
            o + 0x1c,
            ((margin as f64) - (x87_quiet(flipped_distance) as f64)) as f32,
        );
        let sides = [(edge1, v0, 0x20u32), (edge2, v1, 0x30), (edge0, v2, 0x40)];
        for (edge, vertex, offset) in sides {
            let side = cross(edge, n);
            let scale = normalizing_scale(e, sum3(mul(side, side)));
            let scaled = mul(splat(scale), side);
            store(e, o + offset, scaled);
            let side_distance = dot3(scaled, vertex);
            e.mem.set_f32(
                o + offset + 0xc,
                ((margin as f64) - (x87_quiet(side_distance) as f64)) as f32,
            );
        }
    });
    out
}

// Translated from 00ddecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts a record into a sorted array (the engine map has no name), `RET 0xC`
/// `(a, b, c)`: the array is at `this + 4` (data +4, size +8) with 12-byte records. The
/// insertion index is one past the first record whose first float is below `a` (the size
/// when there is none). The record `(a, b, c)` (each through `FLD`/`FSTP`) is inserted
/// there through `00ddecd0(array, allocator, index, record)`.
pub fn fn_00ddecf0(e: &mut Engine, this: Ptr, a: f32, b: f32, c: f32) {
    let me = this.addr();
    let size = e.mem.u32(me + 8) as i32;
    let a = x87_quiet(a);
    let mut index = size;
    if size > 0 {
        let mut at = e.mem.u32(me + 4);
        for position in 0..size {
            if x87_quiet(e.mem.f32(at)) < a {
                index = position + 1;
                break;
            }
            at = at.wrapping_add(0xc);
        }
    }
    e.with_stack(0xc, |e, record| {
        e.mem.set_f32(record.addr(), a);
        e.mem.set_f32(record.addr() + 4, x87_quiet(b));
        e.mem.set_f32(record.addr() + 8, x87_quiet(c));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.call(
            ARRAY_INSERT,
            &args![me + 4, allocator, index as u32, record],
        );
    });
}

// Translated from 00dded60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reduces a list of records to one angle (the engine map has no name), `__fastcall` with
/// `this` = `{default float, data, size}`; the records are 12 bytes, `(f0, f1, key)`. With no
/// records, returns the default float. Otherwise sorts them by `key` (`00ddeb70`), merges
/// runs of records whose keys agree (`00ddce80`) into one `(0, clamp(sum(f1 - f0), 0, 1),
/// key of the run's first)`, then returns the smallest key among the merged records whose
/// span is at least 0.6, or 0 when there is none. The fourth word of the sort call is stack
/// garbage in the exe and is only passed on by the sort; 0 here. Returns it in ST0.
pub fn fn_00dded60(e: &mut Engine, this: Ptr) -> f32 {
    let me = this.addr();
    if e.mem.u32(me + 8) == 0 {
        return x87_quiet(e.mem.f32(me));
    }
    e.with_stack(0x10, |e, block| {
        let array = block.addr();
        let flag = block.addr() + 0xc;
        let count = e.mem.u32(me + 8) as i32;
        if count > 1 {
            let data = e.mem.u32(me + 4);
            e.call(SORT_RECORDS, &args![data, 0u32, (count - 1) as u32, 0u32]);
        }
        let count = e.mem.u32(me + 8) as i32;
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let mut start: i32 = 0;
        let mut start_offset: u32 = 0;
        let mut index: i32 = 1;
        if count >= 1 {
            let mut offset: u32 = 0xc;
            loop {
                let group_ends = if index < count {
                    let data = e.mem.u32(me + 4);
                    let differs = e
                        .call(
                            RECORDS_DIFFER,
                            &args![
                                flag,
                                data.wrapping_add(start_offset),
                                data.wrapping_add(offset)
                            ],
                        )
                        .u32();
                    e.mem.u8(differs) != 0
                } else {
                    true
                };
                if group_ends {
                    let data = e.mem.u32(me + 4);
                    let mut sum = 0.0f32;
                    for member in 0..(index - start) {
                        let record = data
                            .wrapping_add(start_offset)
                            .wrapping_add(12 * member as u32);
                        let low = x87_quiet(e.mem.f32(record)) as f64;
                        let high = x87_quiet(e.mem.f32(record + 4)) as f64;
                        sum = ((high - low) + (sum as f64)) as f32;
                    }
                    let key = x87_quiet(e.mem.f32(data.wrapping_add(start_offset) + 8));
                    #[allow(clippy::manual_clamp)] // the exe's compare order, NaN passes through
                    let clamped = if sum < 0.0 {
                        0.0
                    } else if sum > 1.0 {
                        1.0
                    } else {
                        sum
                    };
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let mut size = e.mem.u32(array + 4);
                    if size == e.mem.u32(array + 8) & 0x3fff_ffff {
                        let allocator = e.mem.u32(router + 0x10);
                        e.call(ARRAY_RESERVE, &args![allocator, array, 0xcu32]);
                        size = e.mem.u32(array + 4);
                    }
                    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(12));
                    if slot != 0 {
                        e.mem.set_f32(slot, 0.0);
                        e.mem.set_f32(slot + 4, clamped);
                        e.mem.set_f32(slot + 8, key);
                    }
                    let size = e.mem.u32(array + 4);
                    e.mem.set_u32(array + 4, size.wrapping_add(1));
                    start = index;
                    start_offset = offset;
                }
                let count = e.mem.u32(me + 8) as i32;
                offset = offset.wrapping_add(0xc);
                index += 1;
                if index > count {
                    break;
                }
            }
        }
        let mut smallest = x87_quiet(e.global::<f32>(PI));
        let mut found = false;
        let size = e.mem.u32(array + 4) as i32;
        if size > 0 {
            let limit = e.global::<f64>(SPAN_LIMIT);
            let data = e.mem.u32(array);
            for record in 0..size {
                let at = data.wrapping_add(12 * record as u32);
                let span =
                    (x87_quiet(e.mem.f32(at + 4)) as f64) - (x87_quiet(e.mem.f32(at)) as f64);
                if !(span < limit) {
                    let key = x87_quiet(e.mem.f32(at + 8));
                    if !(key > smallest) {
                        smallest = key;
                    }
                    found = true;
                }
            }
        }
        let result = if found { smallest } else { 0.0 };
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
            );
        }
        result
    })
}

/// Offsets inside the scratch frame of `computeWeldingInfoMultiShape`: the exe keeps all of
/// these on its stack; the addresses passed to callees only need to be distinct.
const FRAME_SIZE: u32 = 0x880;
const FRAME_NORMAL: u32 = 0x000;
const FRAME_A: u32 = 0x040;
const FRAME_B: u32 = 0x050;
const FRAME_C: u32 = 0x060;
const FRAME_ROTATED_NORMAL: u32 = 0x070;
const FRAME_DIRECTION: u32 = 0x080;
const FRAME_OTHER_NORMAL: u32 = 0x090;
const FRAME_SIDE: u32 = 0x0a0;
const FRAME_OUT_A: u32 = 0x0b0;
const FRAME_OUT_B: u32 = 0x0b4;
const FRAME_RESULT: u32 = 0x0c0;
const FRAME_BOX: u32 = 0x0e0;
const FRAME_PLANES: u32 = 0x100;
const FRAME_KEYS: u32 = 0x150;
const FRAME_KEY_STORAGE: u32 = 0x160;
const FRAME_RECORDS: u32 = 0x370;
const FRAME_OUTER_SHAPE: u32 = 0x400;
const FRAME_INNER_SHAPE: u32 = 0x600;
const FRAME_TRANSFORM: u32 = 0x800;

/// `MINPS dst, src` lane: `dst` when `dst < src`, otherwise `src`.
fn minimum(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| {
        if dst[lane] < src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `MAXPS dst, src` lane: `dst` when `dst > src`, otherwise `src`.
fn maximum(dst: Lanes, src: Lanes) -> Lanes {
    std::array::from_fn(|lane| {
        if dst[lane] > src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

// Translated from 00dddcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMeshWeldingUtility::computeWeldingInfoMultiShape` (Xbox PDB), cdecl with five words
/// `(transform, collection, mode, instances, flag)`. The collection's byte at +0x15 must be
/// one of 1, 2, 4, 5, 6, otherwise the assertion "hkpShapeCollection must be a triangle
/// collection that supports welding." (`.\Util\Welding\hkpMeshWeldingUtility.cpp`, line
/// 0x10F) is reported through the error handler at `[01268360]` and the function returns.
/// Otherwise it calls slot `0x2C` of the collection with `mode` and, unless `mode` is 6,
/// sets the byte at +0x14 and walks the collection's keys (slots `8` and `0xC` of the
/// container at +0x10, child shapes through slot `0x14`). For every child with 3 at +0xC
/// (a triangle), and each of its three edges `j`, the other triangles that may touch it are
/// found: every instance in `instances` (`{data, count}` of 0x50-byte entries whose +0x40 is
/// a shape) gets the triangle's vertices moved with `inverse(entry) * transform`, the box of
/// the edge is queried through the instance shape's slot `0x30`, and each other triangle
/// that is not the same one, does not share the edge `B-C` or `C-A`, and whose normal is
/// nearly perpendicular to the edge direction contributes a record `(t_a, t_b, angle)` to a
/// sorted list when the edge segment passes through its prism. The list is reduced with
/// `00dded60` (default angle `pi`, or 0 when `flag` is not 0), quantized with `00ddce10` and
/// stored as the 5-bit field `j` of a word that is finally handed to slot `0x30` of the
/// collection with the key. The names of the roles are guesses; the exe has none.
pub fn hkp_mesh_welding_utility_compute_welding_info_multi_shape(
    e: &mut Engine,
    transform: Ptr,
    collection: Ptr,
    mode: u32,
    instances: Ptr,
    flag: u32,
) {
    let s = collection.addr();
    if !matches!(e.mem.u8(s + 0x15), 1 | 2 | 4 | 5 | 6) {
        e.with_stack(0x240, |e, scratch| {
            let object = scratch.addr();
            let buffer = scratch.addr() + 0x40;
            e.call(STREAM_BEGIN, &args![object, buffer, 0x200u32]);
            e.call(STREAM_TEXT, &args![object, ASSERT_MESSAGE]);
            let handler = e.global::<u32>(ERROR_HANDLER);
            e.vcall(
                handler,
                0xc,
                &args![1u32, ASSERT_ID, buffer, ASSERT_FILE, ASSERT_LINE],
            );
            e.call(STREAM_END, &args![object]);
        });
        return;
    }
    e.vcall(s, 0x2c, &args![mode]);
    if mode as i32 == 6 {
        return;
    }
    let container = s + 0x10;
    e.mem.set_u8(s + 0x14, 1);
    let mut key = e.vcall(container, 8, &args![]).u32();
    if key == 0xffff_ffff {
        return;
    }
    e.with_stack(FRAME_SIZE, |e, frame| {
        let f = frame.addr();
        loop {
            let triangle = e
                .vcall(container, 0x14, &args![key, f + FRAME_OUTER_SHAPE])
                .u32();
            if e.mem.u32(triangle + 0xc) == 3 {
                let mut welding = 0u32;
                e.call(
                    TRIANGLE_NORMAL,
                    &args![
                        triangle + 0x20,
                        triangle + 0x30,
                        triangle + 0x40,
                        f + FRAME_NORMAL
                    ],
                );
                let normal = load(e, f + FRAME_NORMAL);
                for edge in 0..3u32 {
                    let records_default = if flag as u8 == 0 {
                        x87_quiet(e.global::<f32>(PI))
                    } else {
                        0.0
                    };
                    e.mem.set_u32(f + FRAME_RECORDS + 4, 0);
                    e.mem.set_u32(f + FRAME_RECORDS + 8, 0);
                    e.mem.set_u32(f + FRAME_RECORDS + 0xc, 0x8000_0000);
                    e.mem.set_f32(f + FRAME_RECORDS, records_default);
                    let vertex = |e: &Engine, index: u32| load(e, triangle + 0x20 + 0x10 * index);
                    let a = vertex(e, edge);
                    let b = vertex(e, (edge + 1) % 3);
                    let c = vertex(e, (edge + 2) % 3);
                    let mut instance_count = e.mem.u32(instances.addr() + 4) as i32;
                    let mut instance = 0u32;
                    while (instance as i32) < instance_count {
                        let entry = e.mem.u32(instances.addr()) + 0x50 * instance;
                        let shape = e.mem.u32(entry + 0x40);
                        e.call(
                            SET_MUL_INVERSE_MUL,
                            &args![f + FRAME_TRANSFORM, entry, transform],
                        );
                        let columns =
                            [0x00, 0x10, 0x20, 0x30].map(|at| load(e, f + FRAME_TRANSFORM + at));
                        let [column0, column1, column2, translation] = columns;
                        let rotated = |p: Lanes| {
                            add(
                                add(mul(column0, splat(p[0])), mul(column1, splat(p[1]))),
                                mul(column2, splat(p[2])),
                            )
                        };
                        let moved = |p: Lanes| {
                            add(
                                add(mul(column0, splat(p[0])), mul(column1, splat(p[1]))),
                                add(mul(column2, splat(p[2])), translation),
                            )
                        };
                        let rotated_normal = rotated(normal);
                        let moved_a = moved(a);
                        let moved_b = moved(b);
                        let moved_c = moved(c);
                        let along = sub(moved_b, moved_a);
                        let scale = normalizing_scale(e, sum3(mul(along, along)));
                        let direction = mul(splat(scale), along);
                        store(e, f + FRAME_ROTATED_NORMAL, rotated_normal);
                        store(e, f + FRAME_A, moved_a);
                        store(e, f + FRAME_B, moved_b);
                        store(e, f + FRAME_C, moved_c);
                        store(e, f + FRAME_DIRECTION, direction);
                        e.mem.set_u32(f + FRAME_KEYS, f + FRAME_KEY_STORAGE);
                        e.mem.set_u32(f + FRAME_KEYS + 4, 0);
                        e.mem.set_u32(f + FRAME_KEYS + 8, 0x8000_0080);
                        let minimum_start = splat(e.global::<f32>(BOX_MINIMUM_START));
                        let maximum_start = splat(e.global::<f32>(BOX_MAXIMUM_START));
                        let box_minimum = minimum(minimum(minimum_start, moved_a), moved_b);
                        let box_maximum = maximum(maximum(maximum_start, moved_a), moved_b);
                        store(e, f + FRAME_BOX, box_minimum);
                        store(e, f + FRAME_BOX + 0x10, box_maximum);
                        e.vcall(shape, 0x30, &args![f + FRAME_BOX, f + FRAME_KEYS]);
                        let mut index = 0u32;
                        while (index as i32) < e.mem.u32(f + FRAME_KEYS + 4) as i32 {
                            let other_key = e.mem.u32(e.mem.u32(f + FRAME_KEYS) + 4 * index);
                            let other_container = e.vcall(shape, 0x10, &args![]).u32();
                            let other = e
                                .vcall(
                                    other_container,
                                    0x14,
                                    &args![other_key, f + FRAME_INNER_SHAPE],
                                )
                                .u32();
                            index += 1;
                            if e.mem.u32(other + 0xc) != 3 {
                                continue;
                            }
                            let test = f + FRAME_RESULT;
                            fn_00ddced0(e, Ptr::new(test), Ptr::new(triangle), Ptr::new(other));
                            if e.mem.u8(test) != 0 {
                                continue;
                            }
                            fn_00ddcf60(
                                e,
                                Ptr::new(test),
                                Ptr::new(f + FRAME_B),
                                Ptr::new(f + FRAME_C),
                                Ptr::new(other),
                            );
                            if e.mem.u8(test) != 0 {
                                continue;
                            }
                            fn_00ddcf60(
                                e,
                                Ptr::new(test),
                                Ptr::new(f + FRAME_C),
                                Ptr::new(f + FRAME_A),
                                Ptr::new(other),
                            );
                            if e.mem.u8(test) != 0 {
                                continue;
                            }
                            e.call(
                                TRIANGLE_NORMAL,
                                &args![
                                    other + 0x20,
                                    other + 0x30,
                                    other + 0x40,
                                    f + FRAME_OTHER_NORMAL
                                ],
                            );
                            fn_00ddd040(
                                e,
                                Ptr::new(test),
                                Ptr::new(f + FRAME_DIRECTION),
                                Ptr::new(f + FRAME_OTHER_NORMAL),
                            );
                            if e.mem.u8(test) == 0 {
                                continue;
                            }
                            let perpendicular = cross(direction, rotated_normal);
                            let scale =
                                normalizing_scale(e, sum3(mul(perpendicular, perpendicular)));
                            store(e, f + FRAME_SIDE, mul(splat(scale), perpendicular));
                            let angle = fn_00ddd5a0(
                                e,
                                Ptr::new(f + FRAME_ROTATED_NORMAL),
                                Ptr::new(f + FRAME_SIDE),
                                Ptr::new(f + FRAME_OTHER_NORMAL),
                            );
                            let margin = x87_quiet(e.global::<f32>(PRISM_MARGIN));
                            fn_00ddd940(e, Ptr::new(f + FRAME_PLANES), Ptr::new(other), margin);
                            fn_00ddd0b0(
                                e,
                                Ptr::new(test),
                                Ptr::new(f + FRAME_A),
                                Ptr::new(f + FRAME_B),
                                5,
                                Ptr::new(f + FRAME_PLANES),
                                Ptr::new(f + FRAME_OUT_A),
                                Ptr::new(f + FRAME_OUT_B),
                            );
                            if e.mem.u8(test) != 0 {
                                let from = e.mem.f32(f + FRAME_OUT_A);
                                let to = e.mem.f32(f + FRAME_OUT_B);
                                fn_00ddecf0(e, Ptr::new(f + FRAME_RECORDS), from, to, angle);
                            }
                        }
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let allocator = e.mem.u32(router + 0x10);
                        let capacity = e.mem.u32(f + FRAME_KEYS + 8);
                        e.mem.set_u32(f + FRAME_KEYS + 4, 0);
                        if (capacity as i32) >= 0 {
                            let data = e.mem.u32(f + FRAME_KEYS);
                            e.vcall(
                                allocator,
                                0x10,
                                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
                            );
                        }
                        instance += 1;
                        instance_count = e.mem.u32(instances.addr() + 4) as i32;
                    }
                    let angle = fn_00dded60(e, Ptr::new(f + FRAME_RECORDS));
                    let bucket = fn_00ddce10(e, angle);
                    welding = e.call(SET_FIELD, &args![welding, edge, bucket]).u32() & 0xffff;
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let allocator = e.mem.u32(router + 0x10);
                    let capacity = e.mem.u32(f + FRAME_RECORDS + 0xc);
                    e.mem.set_u32(f + FRAME_RECORDS + 8, 0);
                    if (capacity as i32) >= 0 {
                        let data = e.mem.u32(f + FRAME_RECORDS + 4);
                        e.vcall(
                            allocator,
                            0x10,
                            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
                        );
                    }
                }
                e.vcall(s, 0x30, &args![key, welding]);
            }
            key = e.vcall(container, 0xc, &args![key]).u32();
            if key == 0xffff_ffff {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ddce10, fn_00ddce10(f32) -> u32),
        entry!(0x00ddced0, fn_00ddced0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ddcf60, fn_00ddcf60(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ddd040, fn_00ddd040(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00ddd0b0,
            fn_00ddd0b0(Ptr, Ptr, Ptr, i32, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00ddd5a0, fn_00ddd5a0(Ptr, Ptr, Ptr) -> f32),
        entry!(0x00ddd940, fn_00ddd940(Ptr, Ptr, f32) -> Ptr),
        entry!(
            0x00dddcd0,
            hkp_mesh_welding_utility_compute_welding_info_multi_shape(Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(0x00ddecf0, fn_00ddecf0(Ptr, f32, f32, f32)),
        entry!(0x00dded60, fn_00dded60(Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const SLOT_A: u32 = 0x0900_0011;
    const SLOT_B: u32 = 0x0900_0012;
    const SLOT_C: u32 = 0x0900_0013;
    const SLOT_D: u32 = 0x0900_0014;
    const SLOT_E: u32 = 0x0900_0015;
    const SLOT_F: u32 = 0x0900_0016;
    const SLOT_G: u32 = 0x0900_0017;
    const SLOT_H: u32 = 0x0900_0018;
    const SLOT_I: u32 = 0x0900_0019;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn bits(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn log_calls(e: &mut Engine, addr: u32, log: &Log, tag: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            sink.borrow_mut().push(row);
            Ret::default()
        });
    }

    /// Registers the memory router with an allocator whose free slot (`0x10`) is `free_slot`.
    fn router_with_allocator(e: &mut Engine, free_slot: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, free_slot);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// The array reserve: gives the array room for 8 records of 12 bytes.
    fn register_reserve(e: &mut Engine) {
        e.register_double(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(8 * 12);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-5 * b.abs().max(1.0)
    }

    fn assert_lanes(got: Lanes, want: Lanes) {
        for lane in 0..4 {
            assert!(
                close(got[lane], want[lane]),
                "lane {lane}: {got:?} vs {want:?}"
            );
        }
    }

    fn set_angle_constants(e: &mut Engine) {
        e.set_global(ANGLE_STEP, f64::from_bits(0x3fca_ceea_0000_0000));
        e.set_global(ANGLE_BASE, f64::from_bits(0x4009_21fb_6000_0000));
    }

    #[test]
    fn the_angle_quantizer_finds_the_first_index_not_below_the_value() {
        let mut e = engine();
        set_angle_constants(&mut e);
        let quantize = |e: &mut Engine, value: f32| e.call(0x00ddce10, &args![value]).u32();
        assert_eq!(quantize(&mut e, -4.0), 0);
        assert_eq!(quantize(&mut e, 0.0), 15);
        // 20 * step - base is 1.0471976 as a float: an exact match keeps the index...
        assert_eq!(quantize(&mut e, f32::from_bits(0x3f86_0a92)), 20);
        // ...anything between two steps takes the lower one.
        assert_eq!(quantize(&mut e, 1.0), 19);
        assert_eq!(quantize(&mut e, 1.1), 20);
        assert_eq!(quantize(&mut e, 0.5), 17);
        assert_eq!(quantize(&mut e, 5.0), 31);
        assert_eq!(quantize(&mut e, f32::NAN), 31);
    }

    fn new_triangle(e: &mut Engine, v0: Lanes, v1: Lanes, v2: Lanes) -> u32 {
        let t = e.mem.alloc(0x60);
        e.mem.set_u32(t + 0xc, 3);
        store(e, t + 0x20, v0);
        store(e, t + 0x30, v1);
        store(e, t + 0x40, v2);
        t
    }

    #[test]
    fn same_triangle_compares_the_three_vertices_within_a_tolerance() {
        let mut e = engine();
        e.set_global(VERTEX_TOLERANCE, 0.001f32);
        let a = new_triangle(&mut e, [0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]);
        let b = new_triangle(
            &mut e,
            [0.0, 0.0, 0.0005, 9.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
        );
        let result = e.mem.alloc(8);
        assert_eq!(e.call(0x00ddced0, &args![result, a, b]).u32(), result);
        assert_eq!(e.mem.u8(result), 1);
        store(&mut e, b + 0x40, [0.0, 1.01, 0.0, 0.0]);
        e.call(0x00ddced0, &args![result, a, b]);
        assert_eq!(e.mem.u8(result), 0);
        store(&mut e, b + 0x40, [0.0, f32::NAN, 0.0, 0.0]);
        e.call(0x00ddced0, &args![result, a, b]);
        assert_eq!(e.mem.u8(result), 0);
    }

    #[test]
    fn both_points_on_a_vertex_needs_each_to_match_one() {
        let mut e = engine();
        e.set_global(VERTEX_TOLERANCE, 0.001f32);
        let t = new_triangle(&mut e, [0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]);
        let p = e.mem.alloc(16);
        let q = e.mem.alloc(16);
        let result = e.mem.alloc(8);
        store(&mut e, p, [1.0, 0.0, 0.0005, 3.0]);
        store(&mut e, q, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00ddcf60, &args![result, p, q, t]).u32(), result);
        assert_eq!(e.mem.u8(result), 1);
        // Both on the same vertex also counts.
        store(&mut e, q, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00ddcf60, &args![result, p, q, t]);
        assert_eq!(e.mem.u8(result), 1);
        store(&mut e, q, [5.0, 5.0, 5.0, 0.0]);
        e.call(0x00ddcf60, &args![result, p, q, t]);
        assert_eq!(e.mem.u8(result), 0);
        store(&mut e, p, [0.5, 0.5, 0.0, 0.0]);
        store(&mut e, q, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00ddcf60, &args![result, p, q, t]);
        assert_eq!(e.mem.u8(result), 0);
    }

    #[test]
    fn nearly_perpendicular_is_an_absolute_dot_below_the_limit() {
        let mut e = engine();
        e.set_global(PERPENDICULAR_LIMIT, 0.05f64);
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        let result = e.mem.alloc(8);
        store(&mut e, a, [1.0, 0.0, 0.0, 9.0]);
        for (dot, want) in [(0.049f32, 1), (-0.049, 1), (0.06, 0), (-0.06, 0)] {
            store(&mut e, b, [dot, 1.0, 0.0, 9.0]);
            assert_eq!(e.call(0x00ddd040, &args![result, a, b]).u32(), result);
            assert_eq!(e.mem.u8(result), want, "dot {dot}");
        }
        store(&mut e, b, [f32::NAN, 1.0, 0.0, 0.0]);
        e.call(0x00ddd040, &args![result, a, b]);
        assert_eq!(e.mem.u8(result), 0);
    }

    struct Clip {
        e: Engine,
        planes: u32,
        a: u32,
        b: u32,
        result: u32,
        out_a: u32,
        out_b: u32,
    }

    fn clip_rig(a: Lanes, b: Lanes, planes: &[Lanes]) -> Clip {
        let mut e = engine();
        store(&mut e, ONES, [1.0; 4]);
        let plane_block = e.mem.alloc(0x40);
        for (i, plane) in planes.iter().enumerate() {
            store(&mut e, plane_block + 16 * i as u32, *plane);
        }
        let (a_at, b_at) = (e.mem.alloc(16), e.mem.alloc(16));
        store(&mut e, a_at, a);
        store(&mut e, b_at, b);
        let result = e.mem.alloc(8);
        let out_a = e.mem.alloc(8);
        let out_b = e.mem.alloc(8);
        e.mem.set_f32(out_a, -7.0);
        e.mem.set_f32(out_b, -7.0);
        Clip {
            e,
            planes: plane_block,
            a: a_at,
            b: b_at,
            result,
            out_a,
            out_b,
        }
    }

    fn run_clip(c: &mut Clip, count: u32) -> (u8, f32, f32) {
        // A leftover value shows that a rejection writes the byte.
        c.e.mem.set_u8(c.result, 0xee);
        let result =
            c.e.call(
                0x00ddd0b0,
                &args![c.result, c.a, c.b, count, c.planes, c.out_a, c.out_b],
            )
            .u32();
        assert_eq!(result, c.result);
        (
            c.e.mem.u8(c.result),
            c.e.mem.f32(c.out_a),
            c.e.mem.f32(c.out_b),
        )
    }

    #[test]
    fn clipping_a_segment_against_a_plane_moves_the_crossing_end() {
        // The plane x - 1 = 0: the start inside, the end outside.
        let plane = [1.0, 0.0, 0.0, -1.0];
        let mut c = clip_rig([0.0; 4], [2.0, 0.0, 0.0, 0.0], &[plane]);
        assert_eq!(run_clip(&mut c, 1), (1, 0.0, 0.5));
        // Reversed: the start (outside) moves.
        let mut c = clip_rig([2.0, 0.0, 0.0, 0.0], [0.0; 4], &[plane]);
        assert_eq!(run_clip(&mut c, 1), (1, 0.5, 1.0));
        // Both outside: rejected, outputs untouched.
        let mut c = clip_rig([2.0, 0.0, 0.0, 0.0], [3.0, 0.0, 0.0, 0.0], &[plane]);
        assert_eq!(run_clip(&mut c, 1), (0, -7.0, -7.0));
        // Both inside: nothing to clip.
        let mut c = clip_rig([0.0; 4], [0.5, 0.0, 0.0, 0.0], &[plane]);
        assert_eq!(run_clip(&mut c, 1), (1, 0.0, 1.0));
        // A start exactly on the plane with the end inside is not a crossing.
        let mut c = clip_rig([1.0, 0.0, 0.0, 0.0], [0.0; 4], &[plane]);
        assert_eq!(run_clip(&mut c, 1), (1, 0.0, 1.0));
        // No planes at all.
        let mut c = clip_rig([0.0; 4], [2.0, 0.0, 0.0, 0.0], &[]);
        assert_eq!(run_clip(&mut c, 0), (1, 0.0, 1.0));
    }

    #[test]
    fn clipping_applies_the_planes_in_turn_to_the_shortened_segment() {
        // x <= 1 cuts the end (2, 2) to (1, 1); y <= 0.5 then cuts that to (0.5, 0.5), which
        // is a quarter of the way along the original direction (2, 2).
        let mut c = clip_rig(
            [0.0; 4],
            [2.0, 2.0, 0.0, 0.0],
            &[[1.0, 0.0, 0.0, -1.0], [0.0, 1.0, 0.0, -0.5]],
        );
        let (result, from, to) = run_clip(&mut c, 2);
        assert_eq!((result, from), (1, 0.0));
        assert!(close(to, 0.25), "{to}");
    }

    #[test]
    fn the_angle_helper_passes_the_two_dot_products_to_the_atan2_approximation() {
        let mut e = Engine::new();
        let seen: Log = Rc::default();
        let sink = Rc::clone(&seen);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let (x, y, z) = (e.mem.alloc(16), e.mem.alloc(16), e.mem.alloc(16));
        store(&mut e, x, [1.0, 0.0, 0.0, 5.0]);
        store(&mut e, y, [0.0, 2.0, 0.0, 5.0]);
        store(&mut e, z, [1.0, 1.0, 0.0, 5.0]);
        let got = e.call(0x00ddd5a0, &args![x, y, z]).f32();
        assert_eq!(got, 0.75);
        // atan2(y . z, x . z) = (2, 1).
        assert_eq!(
            *seen.borrow(),
            vec![vec![2.0f32.to_bits(), 1.0f32.to_bits()]]
        );
    }

    /// The dot products keep the SSE choice of NaN (00ddd5a0: `MULPS v, z`, `ADDSS l1, l0`,
    /// `ADDSS l2, (l1 + l0)`): of two NaNs the destination's wins. The inputs are the oracle's
    /// vectors 00ddd5a0_7 and _9, which went wrong when LLVM swapped the host additions.
    #[test]
    fn the_angle_helper_keeps_the_sse_choice_of_nan_in_the_dot_products() {
        let bits = |v: [u32; 4]| v.map(f32::from_bits);
        let cases = [
            // _7: y . z is NaN(7fc00000) + (0.5 + NaN(7fffffff)), x . z is
            // NaN(7fc00000) + (NaN(ffffffff) + tiny).
            (
                [0x0000_0003, 0xffff_ffff, 0x8000_0000, 0],
                [0x7fff_ffff, 0x3f80_0000, 0xbf80_0000, 0],
                [0x3fc0_0000, 0x3f00_0000, 0x7fc0_0000, 0],
                [0x7fc0_0000, 0x7fc0_0000],
            ),
            // _9: y . z is tiny + (inf + NaN(7fc00000)), x . z is
            // tiny + (NaN(7fffffff) + NaN(7fc00000)).
            (
                [0x8000_0000, 0x7fff_ffff, 0x3f80_0000, 0],
                [0xbf80_0000, 0x3fc0_0000, 0x3f00_0000, 0],
                [0x7fc0_0000, 0x7f80_0000, 0x0000_0001, 0],
                [0x7fc0_0000, 0x7fff_ffff],
            ),
        ];
        for (x_bits, y_bits, z_bits, want) in cases {
            let mut e = Engine::new();
            let seen: Log = Rc::default();
            let sink = Rc::clone(&seen);
            e.register_double(ATAN2_APPROXIMATION, move |_, a| {
                sink.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            let (x, y, z) = (e.mem.alloc(16), e.mem.alloc(16), e.mem.alloc(16));
            store(&mut e, x, bits(x_bits));
            store(&mut e, y, bits(y_bits));
            store(&mut e, z, bits(z_bits));
            e.call(0x00ddd5a0, &args![x, y, z]);
            assert_eq!(*seen.borrow(), vec![want.to_vec()], "{x_bits:x?}");
        }
    }

    fn prism_rig() -> Engine {
        let mut e = engine();
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        for (lane, sign) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(NORMAL_SIGN_MASK + 4 * lane as u32, *sign);
        }
        e.register_double(TRIANGLE_NORMAL, |e, a| {
            store(e, a[3], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e
    }

    #[test]
    fn the_prism_has_the_triangle_planes_and_three_side_planes() {
        let mut e = prism_rig();
        let t = new_triangle(&mut e, [0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x50);
        let got = e.call(0x00ddd940, &args![out, t, 0.1f32]).u32();
        assert_eq!(got, out);
        let plane = |e: &Engine, i: u32| load(e, out + 0x10 * i);
        assert_lanes(plane(&e, 0), [0.0, 0.0, 1.0, -0.1]);
        assert_lanes(plane(&e, 1), [0.0, 0.0, -1.0, -0.1]);
        assert_eq!(e.mem.u32(out + 0x10), 0x8000_0000);
        assert_lanes(plane(&e, 2), [0.0, -1.0, 0.0, -0.1]);
        let side = std::f32::consts::FRAC_1_SQRT_2;
        assert_lanes(plane(&e, 3), [side, side, 0.0, -0.1 - side]);
        assert_lanes(plane(&e, 4), [-1.0, 0.0, 0.0, -0.1]);
    }

    #[test]
    fn a_degenerate_triangle_gets_zero_side_planes() {
        let mut e = prism_rig();
        let corner = [1.0, 2.0, 3.0, 0.0];
        let t = new_triangle(&mut e, corner, corner, corner);
        let out = e.mem.alloc(0x50);
        e.call(0x00ddd940, &args![out, t, 0.25f32]);
        for plane in 2..5 {
            let lanes = load(&e, out + 0x10 * plane);
            assert_eq!(lanes[..3], [0.0, 0.0, 0.0]);
        }
    }

    #[test]
    fn insertion_index_is_one_past_the_first_smaller_first_float() {
        let mut e = Engine::new();
        let inserts: Log = Rc::default();
        let sink = Rc::clone(&inserts);
        e.register_double(ARRAY_INSERT, move |e, a| {
            let mut row = a.to_vec();
            row.extend(bits(e, a[3], 3));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        let allocator = router_with_allocator(&mut e, SLOT_A);
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(0x40);
        for (i, value) in [5.0f32, 3.0, 1.0].iter().enumerate() {
            e.mem.set_f32(data + 12 * i as u32, *value);
        }
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, 3);
        let signalling = f32::from_bits(0x7fa0_0000);
        for first in [2.0f32, 9.0, 0.5, 3.0] {
            e.call(0x00ddecf0, &args![this, first, signalling, 7.0f32]);
        }
        {
            let rows = inserts.borrow();
            let indexes: Vec<u32> = rows.iter().map(|r| r[2]).collect();
            // 2.0: after the 1.0 at index 2; 9.0: the first record is already smaller; 0.5:
            // none is smaller; 3.0: an equal value is not smaller, the 1.0 is.
            assert_eq!(indexes, vec![3, 1, 3, 3]);
            assert_eq!(rows[0][0], this + 4);
            assert_eq!(rows[0][1], allocator);
            // The record is quieted by the x87 load and store.
            assert_eq!(
                rows[0][4..],
                [2.0f32.to_bits(), 0x7fe0_0000, 7.0f32.to_bits()]
            );
        }
        // With nothing in the array the index is the size, 0.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00ddecf0, &args![this, 1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(inserts.borrow().len(), 5);
        assert_eq!(inserts.borrow()[4][2], 0);
    }

    fn records_rig(records: &[[f32; 3]]) -> (Engine, u32, Log, Log) {
        let mut e = engine();
        e.set_global(PI, std::f32::consts::PI);
        e.set_global(SPAN_LIMIT, 0.6f64);
        let frees: Log = Rc::default();
        let sink = Rc::clone(&frees);
        e.register_double(SLOT_A, move |e, a| {
            let mut row = vec![a[1], a[2]];
            row.extend(bits(e, a[1], a[2] / 4));
            sink.borrow_mut().push(row);
            Ret::default()
        });
        router_with_allocator(&mut e, SLOT_A);
        let sorts: Log = Rc::default();
        log_calls(&mut e, SORT_RECORDS, &sorts, 1);
        register_reserve(&mut e);
        e.register_double(RECORDS_DIFFER, |e, a| {
            let different = (e.mem.f32(a[1] + 8) - e.mem.f32(a[2] + 8)).abs() >= 0.001;
            e.mem.set_u8(a[0], u8::from(different));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(12 * records.len().max(1) as u32);
        for (i, record) in records.iter().enumerate() {
            for (j, value) in record.iter().enumerate() {
                e.mem.set_f32(data + 12 * i as u32 + 4 * j as u32, *value);
            }
        }
        e.mem.set_u32(this + 4, data);
        e.mem.set_u32(this + 8, records.len() as u32);
        (e, this, frees, sorts)
    }

    #[test]
    fn with_no_records_the_default_float_is_returned() {
        let (mut e, this, frees, sorts) = records_rig(&[]);
        e.mem.set_f32(this, 2.5);
        let got = e.call(0x00dded60, &args![this]).f32();
        assert_eq!(got, 2.5);
        assert!(frees.borrow().is_empty() && sorts.borrow().is_empty());
    }

    #[test]
    fn runs_with_equal_keys_are_summed_and_the_smallest_wide_key_wins() {
        let (mut e, this, frees, sorts) = records_rig(&[
            [0.0, 0.5, 1.0],
            [0.0, 0.3, 1.0],
            [0.1, 0.5, 2.0],
            [0.0, 0.7, 4.0],
            [0.0, 0.7, 4.0],
        ]);
        let got = e.call(0x00dded60, &args![this]).f32();
        // Runs: key 1 sums to 0.8 (wide enough), key 2 to 0.4, key 4 to 1.4 clamped to 1.
        assert_eq!(got, 1.0);
        let data = e.mem.u32(this + 4);
        assert_eq!(*sorts.borrow(), vec![vec![1, data, 0, 4, 0]]);
        let frees = frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1], 8 * 12);
        let kept: Vec<f32> = frees[0][2..11].iter().map(|b| f32::from_bits(*b)).collect();
        assert_eq!(kept[0], 0.0);
        assert!(close(kept[1], 0.8), "{kept:?}");
        assert_eq!(kept[2], 1.0);
        assert_eq!(kept[3], 0.0);
        assert!(close(kept[4], 0.4), "{kept:?}");
        assert_eq!(kept[5], 2.0);
        assert_eq!(kept[6..], [0.0, 1.0, 4.0]);
    }

    #[test]
    fn without_a_wide_run_the_result_is_zero() {
        let (mut e, this, _frees, sorts) = records_rig(&[[0.0, 0.2, 1.0]]);
        let got = e.call(0x00dded60, &args![this]).f32();
        assert_eq!(got, 0.0);
        // One record needs no sort.
        assert!(sorts.borrow().is_empty());
    }

    struct Welding {
        e: Engine,
        collection: u32,
        instances: u32,
        transform: u32,
    }

    /// A collection holding one triangle (key 5) above the x axis, and one instance whose
    /// shape answers every box query with the key 7: a triangle below the axis that shares
    /// the first edge of the first. The shapes' virtual calls and the outside helpers are
    /// doubles; the helpers of this file run for real.
    fn welding_rig(log: &Log) -> Welding {
        let mut e = engine();
        set_angle_constants(&mut e);
        e.set_global(PI, std::f32::consts::PI);
        e.set_global(BOX_MINIMUM_START, f32::from_bits(0x7f7f_ffee));
        e.set_global(BOX_MAXIMUM_START, f32::from_bits(0xff7f_ffee));
        e.set_global(PRISM_MARGIN, 0.05f32);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(VERTEX_TOLERANCE, 0.001f32);
        e.set_global(PERPENDICULAR_LIMIT, 0.05f64);
        e.set_global(SPAN_LIMIT, 0.6f64);
        store(&mut e, ONES, [1.0; 4]);
        for (lane, sign) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(NORMAL_SIGN_MASK + 4 * lane as u32, *sign);
        }
        let own = new_triangle(&mut e, [0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]);
        let other = new_triangle(
            &mut e,
            [1.0, 0.0, 0.0, 0.0],
            [0.0; 4],
            [0.0, -1.0, 0.0, 0.0],
        );
        // The collection: vtable slots 0x2C and 0x30, the container at +0x10.
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x2c, SLOT_A);
        e.mem.set_u32(vtable + 0x30, SLOT_B);
        let collection = e.mem.alloc(0x40);
        e.mem.set_u32(collection, vtable);
        e.mem.set_u8(collection + 0x15, 4);
        let container_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(container_vtable + 8, SLOT_C);
        e.mem.set_u32(container_vtable + 0xc, SLOT_D);
        e.mem.set_u32(container_vtable + 0x14, SLOT_E);
        e.mem.set_u32(collection + 0x10, container_vtable);
        log_calls(&mut e, SLOT_A, log, 1);
        log_calls(&mut e, SLOT_B, log, 2);
        e.register_double(SLOT_C, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register_double(SLOT_D, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.register_double(SLOT_E, move |_, _| Ret {
            eax: own,
            ..Ret::default()
        });
        // The instance shape: slot 0x30 answers the box query, slot 0x10 is its collection.
        let shape_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(shape_vtable + 0x30, SLOT_F);
        e.mem.set_u32(shape_vtable + 0x10, SLOT_G);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, shape_vtable);
        let other_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(other_vtable + 0x14, SLOT_H);
        let other_container = e.mem.alloc(0x10);
        e.mem.set_u32(other_container, other_vtable);
        e.register_double(SLOT_G, move |_, _| Ret {
            eax: other_container,
            ..Ret::default()
        });
        e.register_double(SLOT_H, move |_, _| Ret {
            eax: other,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(SLOT_F, move |e, a| {
            let mut row = vec![3];
            row.extend(bits(e, a[1], 8));
            sink.borrow_mut().push(row);
            let storage = e.mem.u32(a[2]);
            e.mem.set_u32(storage, 7);
            e.mem.set_u32(a[2] + 4, 1);
            Ret::default()
        });
        let instances = e.mem.alloc(8);
        let entries = e.mem.alloc(0x50);
        e.mem.set_u32(entries + 0x40, shape);
        e.mem.set_u32(instances, entries);
        e.mem.set_u32(instances + 4, 1);
        let transform = e.mem.alloc(0x40);
        // The relative transform is the identity.
        e.register_double(SET_MUL_INVERSE_MUL, |e, a| {
            store(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            store(e, a[0] + 0x30, [0.0; 4]);
            Ret::default()
        });
        e.register_double(TRIANGLE_NORMAL, |e, a| {
            store(e, a[3], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e.register_double(SET_FIELD, |_, a| Ret {
            eax: (a[0] & !(0x1f << (5 * a[1]))) | (a[2] << (5 * a[1])),
            ..Ret::default()
        });
        register_reserve(&mut e);
        router_with_allocator(&mut e, SLOT_I);
        log_calls(&mut e, SLOT_I, log, 20);
        let sink = Rc::clone(log);
        e.register_double(ARRAY_INSERT, move |e, a| {
            let array = a[0];
            let mut row = vec![40, a[0], a[1], a[2]];
            let record = bits(e, a[3], 3);
            row.extend(&record);
            sink.borrow_mut().push(row);
            if e.mem.u32(array) == 0 {
                let data = e.mem.alloc(8 * 12);
                e.mem.set_u32(array, data);
                e.mem.set_u32(array + 8, 8);
            }
            let size = e.mem.u32(array + 4);
            let slot = e.mem.u32(array) + 12 * size;
            for (i, word) in record.iter().enumerate() {
                e.mem.set_u32(slot + 4 * i as u32, *word);
            }
            e.mem.set_u32(array + 4, size + 1);
            Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            let mut row = vec![30];
            row.extend_from_slice(a);
            sink.borrow_mut().push(row);
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        Welding {
            e,
            collection,
            instances,
            transform,
        }
    }

    fn run_welding(w: &mut Welding, mode: u32, flag: u32) {
        w.e.call(
            0x00dddcd0,
            &args![w.transform, w.collection, mode, w.instances, flag],
        );
    }

    #[test]
    fn a_wrong_collection_type_reports_the_assertion() {
        let log: Log = Rc::default();
        let mut w = welding_rig(&log);
        w.e.mem.set_u8(w.collection + 0x15, 3);
        let handler_vtable = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(handler_vtable + 0xc, SLOT_A + 0x20);
        let handler = w.e.mem.alloc(8);
        w.e.mem.set_u32(handler, handler_vtable);
        w.e.mem.set_u32(ERROR_HANDLER, handler);
        log_calls(&mut w.e, SLOT_A + 0x20, &log, 9);
        log_calls(&mut w.e, STREAM_BEGIN, &log, 10);
        log_calls(&mut w.e, STREAM_TEXT, &log, 11);
        log_calls(&mut w.e, STREAM_END, &log, 12);
        run_welding(&mut w, 1, 0);
        let rows = log.borrow();
        assert_eq!(rows.len(), 4);
        let object = rows[0][1];
        let buffer = rows[0][2];
        assert_eq!(rows[0], vec![10, object, buffer, 0x200]);
        assert_eq!(rows[1], vec![11, object, ASSERT_MESSAGE]);
        assert_eq!(
            rows[2],
            vec![9, handler, 1, ASSERT_ID, buffer, ASSERT_FILE, ASSERT_LINE]
        );
        assert_eq!(rows[3], vec![12, object]);
    }

    #[test]
    fn mode_6_stops_after_telling_the_collection() {
        let log: Log = Rc::default();
        let mut w = welding_rig(&log);
        run_welding(&mut w, 6, 0);
        assert_eq!(*log.borrow(), vec![vec![1, w.collection, 6]]);
        assert_eq!(w.e.mem.u8(w.collection + 0x14), 0);
    }

    #[test]
    fn a_neighbour_across_the_first_edge_decides_its_angle() {
        let log: Log = Rc::default();
        let mut w = welding_rig(&log);
        run_welding(&mut w, 1, 0);
        assert_eq!(w.e.mem.u8(w.collection + 0x14), 1);
        let rows = log.borrow();
        let tag = |t: u32| rows.iter().filter(|r| r[0] == t).collect::<Vec<_>>();
        assert_eq!(rows[0], vec![1, w.collection, 1]);
        // Each of the three edges queries the instance shape with the box of the edge.
        let boxes = tag(3);
        assert_eq!(boxes.len(), 3);
        let first_box = [0.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0].map(f32::to_bits);
        assert_eq!(boxes[0][1..], first_box);
        // The other triangle shares the first edge only: edges 1 and 2 each run into a shared
        // edge and are skipped, leaving the default angle.
        let angles = tag(30);
        assert_eq!(angles.len(), 1);
        assert_eq!(angles[0][1..], [0.0f32.to_bits(), 1.0f32.to_bits()]);
        // The clipped segment spans the whole edge: the record is (0, 1, angle).
        let inserted = tag(40);
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][3], 0);
        assert_eq!(
            inserted[0][4..],
            [0.0f32.to_bits(), 1.0f32.to_bits(), 0.5f32.to_bits()]
        );
        // The record list of the first edge and the reduction's list are both freed.
        let freed: Vec<u32> = tag(20).iter().map(|r| r[3]).collect();
        assert_eq!(freed, vec![96, 96]);
        // Edge 0 quantizes 0.5 to 17, the other two quantize the default pi to 30.
        let welding = 17 | (30 << 5) | (30 << 10);
        let told = tag(2);
        assert_eq!(told.len(), 1);
        assert_eq!(told[0][1..], [w.collection, 5, welding]);
    }

    #[test]
    fn a_nonzero_flag_starts_the_angle_list_at_zero() {
        let log: Log = Rc::default();
        let mut w = welding_rig(&log);
        run_welding(&mut w, 1, 1);
        let rows = log.borrow();
        let told: Vec<&Vec<u32>> = rows.iter().filter(|r| r[0] == 2).collect();
        assert_eq!(told.len(), 1);
        // The default 0.0 quantizes to 15 on the edges without a neighbour.
        assert_eq!(told[0][3], 17 | (15 << 5) | (15 << 10));
    }
}
