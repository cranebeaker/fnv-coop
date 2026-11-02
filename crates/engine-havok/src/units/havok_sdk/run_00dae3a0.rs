//! `Havok SDK/run_00dae3a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, andnps, cmpeqss, cmpltps, divss, fcom_status, load_vec, movhlps, movlhps,
    movmskps, mulps, mulss, rsqrtss, shufps, store_vec, subps, subss, unpckhps, unpcklps, xorps,
    Vec4,
};

/// The `3.4028e38` float (`0104da48`) the minimum distance starts at.
const START_MINIMUM: u32 = 0x0104_da48;
/// The double `1.05` (`010d5288`), the double `526.0` (`010d5280`), the float `0.030461745`
/// (`010d5278`), the double `1e-9` (`010d5270`) and the double `0.030461745` (`010d5268`).
const GROWTH: u32 = 0x010d_5288;
const SQUARE_LIMIT: u32 = 0x010d_5280;
const START_RATIO: u32 = 0x010d_5278;
const EPSILON: u32 = 0x010d_5270;
const RATIO: u32 = 0x010d_5268;
/// The `hkGskManifold` point remover `00dae220` (the exe passes the manifold in `EDX`).
const MANIFOLD_REMOVE_POINT: u32 = 0x00da_e220;

/// The squared length `z + (y + x)` of the vector `v` (lane 0 of the result), as the exe sums it.
fn squared_length(v: Vec4) -> Vec4 {
    let squares = mulps(v, v);
    let sum = addss(shufps(squares, squares, 0x55), squares);
    addss(shufps(squares, squares, 0xaa), sum)
}

/// A value stored as a float and loaded again (`FSTP dword` then `FLD dword`).
fn rounded(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FNSTSW AX; TEST AH, 5; JP skip` after an `FCOM`/`FCOMP` of `st0` with `operand`: the jump is
/// not taken exactly when `st0 < operand` (C0 alone; equal and unordered both set an even number
/// of C0/C2, and C2 alone never comes out of a compare).
fn st0_below(st0: f64, operand: f64) -> bool {
    fcom_status(st0, operand) & 5 == 1
}

/// `FNSTSW AX; TEST AH, 0x41; JNZ skip`: the jump is not taken exactly when `st0 > operand`.
fn st0_above(st0: f64, operand: f64) -> bool {
    fcom_status(st0, operand) & 0x41 == 0
}

// Translated from 00daee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register input `ecx` (`points`, five 0x30-byte point records: the first four are the
/// manifold's, the fifth at `+0xc0` is the new one), cdecl otherwise, answers the index (0 to 4)
/// of the point to drop. Per record: the vector at `+0`, a vector at `+0x10` whose `w` (`+0x1c`)
/// is a depth, and a vector at `+0x20`; the new point's depth is the word at `+0xdc`. The exe keeps
/// the running values as floats on its stack frame (minimum at `+0`, maximum at `+4`, a scratch
/// word at `+0xc`, the four squared distances from `+0x10`, five used flags from `+0x20`), which
/// is the frame laid out below.
///
/// The squared distances `d0..d3` of the old points to the new one (`z + (y + x)` of the squared
/// difference) give the farthest `F` (strictly greater, first wins) and the nearest (strictly
/// less), whose depth difference `D = new depth - nearest depth` is kept. `F` is flagged and the
/// maximum is scaled by 1.05. Then three more points are flagged, each the first strictly
/// greatest of the unflagged: the one farthest from `F` (taken only above the scaled maximum,
/// else 4, the new point), the one with the largest squared cross product `(P - F) x (F - S)` for
/// the point `S` just flagged, and the one farthest from that third point. When the new point
/// (4) is still unflagged, `D < 0` and `minimum < D * (D * 526.0)`, the depths decide: with
/// `T_k = (depth_k - new depth)^2 + 1e-9` the answer becomes `k` (0 to 3, the last one that
/// does) while `d_k * ratio < T_k`, the ratio first `0.030461745` and then the last `T_k / (d_k +
/// 1e-9)`; it is 4 when none does. Otherwise the answer is the first unflagged index (0 when all
/// five are flagged). The exe's name for it is unknown (the choice of the manifold point to
/// replace).
pub fn fn_00daee00(e: &mut Engine, points: u32) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let (minimum_at, maximum_at, scratch_at) = (f, f + 4, f + 0xc);
        let distance_at = |k: u32| f + 0x10 + 4 * k;
        let flag_at = |k: u32| f + 0x20 + 4 * k;
        let point = |k: u32| points.wrapping_add(0x30 * k);
        let query = load_vec(e, points + 0xc0);
        e.mem.set_u32(maximum_at, x87_store32(0.0));
        let start = x87_load32(e.mem.u32(START_MINIMUM));
        e.mem.set_u32(minimum_at, x87_store32(start));
        for k in 0..5 {
            e.mem.set_u32(flag_at(k), 0);
        }
        let mut farthest = 0u32;
        let mut nearest = 0u32;
        for k in 0..4 {
            let difference = subps(query, load_vec(e, point(k)));
            let total = squared_length(difference);
            let distance = x87_load32(total[0].to_bits());
            e.mem.set_u32(distance_at(k), x87_store32(distance));
            if st0_below(x87_load32(e.mem.u32(maximum_at)), distance) {
                e.mem.set_u32(maximum_at, x87_store32(distance));
                farthest = k;
            }
            if st0_above(x87_load32(e.mem.u32(minimum_at)), distance) {
                e.mem.set_u32(minimum_at, x87_store32(distance));
                nearest = k;
            }
        }
        let depth = x87_load32(e.mem.u32(points + 0xdc));
        let depth_difference = x87_sub(depth, x87_m32(e.mem.u32(point(nearest) + 0x1c)));
        e.mem.set_u32(flag_at(farthest), 1);
        e.mem.set_u32(scratch_at, x87_store32(depth_difference));
        let anchor = point(farthest);
        let scaled = x87_mul(x87_load32(e.mem.u32(maximum_at)), e.mem.f64(GROWTH));
        e.mem.set_u32(maximum_at, x87_store32(scaled));
        let mut second = 4u32;
        for k in 0..5 {
            if e.mem.u32(flag_at(k)) == 0 {
                let difference = subps(load_vec(e, anchor), load_vec(e, point(k)));
                let distance = x87_load32(squared_length(difference)[0].to_bits());
                if st0_below(x87_load32(e.mem.u32(maximum_at)), distance) {
                    e.mem.set_u32(maximum_at, x87_store32(distance));
                    second = k;
                }
            }
        }
        let anchor_vec = load_vec(e, anchor);
        e.mem.set_u32(maximum_at, x87_store32(0.0));
        let second_vec = load_vec(e, point(second));
        e.mem.set_u32(flag_at(second), 1);
        let edge = subps(anchor_vec, second_vec);
        let mut third = 0u32;
        for k in 0..5 {
            if e.mem.u32(flag_at(k)) == 0 {
                let offset = subps(load_vec(e, point(k)), anchor_vec);
                let first_product = mulps(shufps(offset, offset, 0xc9), shufps(edge, edge, 0xd2));
                let second_product = mulps(shufps(offset, offset, 0xd2), shufps(edge, edge, 0xc9));
                let cross = subps(first_product, second_product);
                let distance = x87_load32(squared_length(cross)[0].to_bits());
                if st0_below(x87_load32(e.mem.u32(maximum_at)), distance) {
                    e.mem.set_u32(maximum_at, x87_store32(distance));
                    third = k;
                }
            }
        }
        e.mem.set_u32(flag_at(third), 1);
        e.mem.set_u32(maximum_at, x87_store32(0.0));
        let third_at = point(third);
        let mut fourth = 0u32;
        for k in 0..5 {
            if e.mem.u32(flag_at(k)) == 0 {
                let difference = subps(load_vec(e, third_at), load_vec(e, point(k)));
                let distance = x87_load32(squared_length(difference)[0].to_bits());
                if st0_below(x87_load32(e.mem.u32(maximum_at)), distance) {
                    e.mem.set_u32(maximum_at, x87_store32(distance));
                    fourth = k;
                }
            }
        }
        e.mem.set_u32(flag_at(fourth), 1);
        let first_unflagged =
            |e: &Engine| (0..5).find(|k| e.mem.u32(flag_at(*k)) == 0).unwrap_or(0);
        if e.mem.u32(flag_at(4)) != 0 {
            return first_unflagged(e);
        }
        let depth_gap = x87_load32(e.mem.u32(scratch_at));
        if !st0_below(depth_gap, 0.0) {
            return first_unflagged(e);
        }
        let minimum = x87_load32(e.mem.u32(minimum_at));
        let limit = x87_mul(depth_gap, x87_mul(depth_gap, e.mem.f64(SQUARE_LIMIT)));
        if !st0_below(minimum, limit) {
            return first_unflagged(e);
        }
        e.mem
            .set_u32(maximum_at, x87_quiet_bits(e.mem.u32(START_RATIO)));
        let mut answer = 4u32;
        let reference = rounded(depth);
        let epsilon = e.mem.f64(EPSILON);
        for k in 0..4u32 {
            let other = x87_load32(e.mem.u32(point(k) + 0x1c));
            let gap = rounded(x87_sub(other, reference));
            let square = rounded(x87_mul(gap, gap));
            let bound = if k == 3 {
                rounded(x87_add(epsilon, x87_m32(x87_store32(square))))
            } else {
                rounded(x87_add(square, epsilon))
            };
            let distance = x87_load32(e.mem.u32(distance_at(k)));
            let product = if k == 0 {
                x87_mul(e.mem.f64(RATIO), distance)
            } else {
                x87_mul(distance, x87_m32(e.mem.u32(maximum_at)))
            };
            if st0_below(product, bound) {
                answer = k;
                if k != 3 {
                    let quotient = x87_div(bound, x87_add(distance, epsilon));
                    e.mem.set_u32(maximum_at, x87_store32(quotient));
                }
            }
        }
        answer
    })
}

/// The state of the point lists after an entry is added (the common tail of `00daf1e0`): the
/// entry at `record + 4` gets the counts `info[8]` and `info[9]`, the id `word` and, per vertex
/// word of `info` (the first `info[8]` are group A, the next `info[9]` group B), the byte `index
/// << 4` where the vertex sits in the record's word table (the table after the `record[2]` 8-byte
/// entries, group A words first). A vertex that is not in its group yet is appended to the group;
/// a new group A word shifts the group B words up and, when any was added, the group B index
/// bytes of every other entry are raised by 0x10 per added word.
fn rewrite_entry_and_tables(e: &mut Engine, record: u32, info: u32, word: u32) {
    let count_a = e.mem.u8(info + 8);
    e.mem.set_u8(record + 4, count_a);
    let count_b = e.mem.u8(info + 9);
    e.mem.set_u8(record + 5, count_b);
    e.mem.set_u16(record + 6, word as u16);
    e.mem.set_u32(record + 8, 0);
    let table = record + 4 + 8 * u32::from(e.mem.u8(record + 2));
    let mut added = 0u32;
    if e.mem.u8(info + 8) > 0 {
        let mut j = 0u32;
        loop {
            let value = e.mem.u16(info + 2 * j);
            let mut index = 0u32;
            let mut found = false;
            if e.mem.u8(record) != 0 {
                loop {
                    if e.mem.u16(table + 2 * index) == value {
                        found = true;
                        break;
                    }
                    index += 1;
                    if index >= u32::from(e.mem.u8(record)) {
                        break;
                    }
                }
            }
            if !found {
                let mut k = u32::from(e.mem.u8(record + 1)) + u32::from(e.mem.u8(record));
                if k > index {
                    loop {
                        let moved = e.mem.u16(table + 2 * k - 2);
                        e.mem.set_u16(table + 2 * k, moved);
                        k -= 1;
                        if k <= index {
                            break;
                        }
                    }
                }
                e.mem.set_u16(table + 2 * index, value);
                let count = e.mem.u8(record);
                e.mem.set_u8(record, count.wrapping_add(1));
                added += 0x10;
            }
            e.mem.set_u8(record + j + 8, (index << 4) as u8);
            j += 1;
            if j >= u32::from(e.mem.u8(info + 8)) {
                break;
            }
        }
    }
    if added != 0 && e.mem.u8(record + 2) > 1 {
        let mut entry = record + 0xc;
        let mut number = 1u32;
        loop {
            let mut c = u32::from(e.mem.u8(entry));
            if e.mem.u8(entry + 1) != 0 {
                loop {
                    let at = entry + c + 4;
                    let byte = e.mem.u8(at);
                    e.mem.set_u8(at, byte.wrapping_add(added as u8));
                    c += 1;
                    if c >= u32::from(e.mem.u8(entry)) + u32::from(e.mem.u8(entry + 1)) {
                        break;
                    }
                }
            }
            number += 1;
            entry += 8;
            if number >= u32::from(e.mem.u8(record + 2)) {
                break;
            }
        }
    }
    if e.mem.u8(info + 9) > 0 {
        let mut j = 0u32;
        loop {
            let first = u32::from(e.mem.u8(info + 8));
            let value = e.mem.u16(info + 2 * (first + j));
            let start = u32::from(e.mem.u8(record));
            let end = u32::from(e.mem.u8(record + 1)) + start;
            let mut index = start;
            let mut found = false;
            if (index as i32) < (end as i32) {
                loop {
                    if e.mem.u16(table + 2 * index) == value {
                        found = true;
                        break;
                    }
                    index += 1;
                    if (index as i32) >= (end as i32) {
                        break;
                    }
                }
            }
            if !found {
                e.mem.set_u16(table + 2 * index, value);
                let count = e.mem.u8(record + 1);
                e.mem.set_u8(record + 1, count.wrapping_add(1));
            }
            let first = u32::from(e.mem.u8(info + 8));
            e.mem.set_u8(record + first + j + 8, (index << 4) as u8);
            j += 1;
            if (j as i32) >= i32::from(e.mem.u8(info + 9)) {
                break;
            }
        }
    }
}

// Translated from 00daf1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an entry to a manifold record: cdecl, ten stack words `(record, a1, a2, a3, a4, info,
/// out, points, object, flag)`, answers 4 (added), 5 (rejected), 6 (replaced a point and the
/// record lost its last entry) or the index of the point it replaced. `record` has the group
/// counts at `+0` and `+1`, the entry count `n` at `+2`, `n` 8-byte entries from `+4` (two group
/// counts, a 16-bit id, four index bytes) and then the table of 16-bit vertex words (see
/// `rewrite_entry_and_tables`). `object` is a callback object: its slot `+0xc` is asked `(a1,
/// a2, a3, a4, info, out)` and answers a 16-bit id (0xffff for none), its slot `+0x14` is told
/// `(id, a4[1])`.
///
/// With 4 entries: `00daee00(points)` picks the point to drop (4: the answer is 5). The id of
/// its entry (the word at `+2`) goes to slot `+0x14`, its entry takes the first entry's two data
/// words (the first entry itself takes the second entry's when it is the one dropped), `00dae220`
/// removes the point and slot `+0xc` is asked. With 0xffff there the first entry takes the last
/// entry's data words, the entry count drops, the vertex words after the entries move down by
/// one entry, the 0x24 bytes of `points + 0x90` are copied to the dropped point's slot (`+0x30`
/// per index) and the answer is 6. Otherwise the id is stored at `out + 0x20`, `out` (0x24
/// bytes) is copied to the slot and the answer will be the index. With fewer entries: slot
/// `+0xc` is asked unless `flag` is set and `info[9]` is not 3 (the id is then taken to be
/// 0xffff); 0xffff from the callback means the answer 5. The id goes to `out + 0x20`, the vertex
/// words move up by one entry and the first entry moves to the end. Then the new entry is
/// written at the front by `rewrite_entry_and_tables`. The exe's name for it is unknown (part of
/// the manifold's point list, see `00dae3a0`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00daf1e0(
    e: &mut Engine,
    record: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    info: u32,
    out: u32,
    points: u32,
    object: u32,
    flag: u32,
) -> u32 {
    let (result, word) = if e.mem.u8(record + 2) == 4 {
        let index = fn_00daee00(e, points);
        if index == 4 {
            return 5;
        }
        let id = u32::from(e.mem.u16(record + index * 8 + 6));
        let extra = e.mem.u32(a4 + 4);
        e.vcall(object, 0x14, &args![id, extra]);
        if index != 0 {
            let first = e.mem.u32(record + 4);
            e.mem.set_u32(record + index * 8 + 4, first);
            let second = e.mem.u32(record + 8);
            e.mem.set_u32(record + index * 8 + 8, second);
        } else {
            let first = e.mem.u32(record + 0xc);
            e.mem.set_u32(record + 4, first);
            let second = e.mem.u32(record + 0x10);
            e.mem.set_u32(record + 8, second);
        }
        e.call(MANIFOLD_REMOVE_POINT, &args![record]);
        let answered = e
            .vcall(object, 0xc, &args![a1, a2, a3, a4, info, out])
            .u16();
        if answered == 0xffff {
            let first = e.mem.u32(record + 0x1c);
            let second = e.mem.u32(record + 0x20);
            e.mem.set_u32(record + 4, first);
            let count = u32::from(e.mem.u8(record + 2));
            e.mem.set_u32(record + 8, second);
            let mut destination = record + (count - 1) * 8 + 4;
            let mut remaining =
                (i32::from(e.mem.u8(record + 1)) + i32::from(e.mem.u8(record)) - 1) >> 1;
            e.mem.set_u8(record + 2, (count - 1) as u8);
            if remaining >= 0 {
                loop {
                    let moved = e.mem.u32(destination + 8);
                    e.mem.set_u32(destination, moved);
                    destination += 4;
                    remaining -= 1;
                    if remaining < 0 {
                        break;
                    }
                }
            }
            let slot = points + index * 0x30;
            let vector = load_vec(e, points + 0x90);
            store_vec(e, slot, vector);
            let vector = load_vec(e, points + 0xa0);
            store_vec(e, slot + 0x10, vector);
            let last = e.mem.u32(points + 0xb0);
            e.mem.set_u32(slot + 0x20, last);
            return 6;
        }
        let vector = load_vec(e, out);
        e.mem.set_u32(out + 0x20, u32::from(answered));
        let slot = points + index * 0x30;
        store_vec(e, slot, vector);
        let vector = load_vec(e, out + 0x10);
        store_vec(e, slot + 0x10, vector);
        let last = e.mem.u32(out + 0x20);
        e.mem.set_u32(slot + 0x20, last);
        (index, u32::from(answered))
    } else {
        let word = if flag == 0 || e.mem.u8(info + 9) == 3 {
            let answered = u32::from(
                e.vcall(object, 0xc, &args![a1, a2, a3, a4, info, out])
                    .u16(),
            );
            if answered == 0xffff {
                return 5;
            }
            answered
        } else {
            0xffff
        };
        e.mem.set_u32(out + 0x20, word);
        let count = u32::from(e.mem.u8(record + 2));
        let mut remaining =
            (i32::from(e.mem.u8(record + 1)) + i32::from(e.mem.u8(record)) - 1) >> 1;
        let mut source = record.wrapping_add(((remaining + 2 * count as i32) * 4 + 4) as u32);
        let mut destination =
            record.wrapping_add(((remaining + 2 * (count as i32 + 1)) * 4 + 4) as u32);
        if remaining >= 0 {
            loop {
                let moved = e.mem.u32(source);
                source = source.wrapping_sub(4);
                e.mem.set_u32(destination, moved);
                destination = destination.wrapping_sub(4);
                remaining -= 1;
                if remaining < 0 {
                    break;
                }
            }
        }
        let first = e.mem.u32(record + 4);
        e.mem.set_u32(record + count * 8 + 4, first);
        let second = e.mem.u32(record + 8);
        e.mem.set_u32(record + count * 8 + 8, second);
        e.mem.set_u8(record + 2, (count + 1) as u8);
        (4, word)
    };
    rewrite_entry_and_tables(e, record, info, word);
    result
}

/// The four sign bits (`010c44d0`) the four-vertex cases negate their normal with.
const SIGN_BITS: u32 = 0x010c_44d0;

/// A float of the image as `MOVSS xmm, [address]` loads it: lane 0, the other lanes zero.
fn scalar_vec(e: &Engine, address: u32) -> Vec4 {
    [f32::from_bits(e.mem.u32(address)), 0.0, 0.0, 0.0]
}

/// What a case of `00dae3a0` leaves for the common tail: `XMM0`, `XMM2`, `XMM7` and `EAX`
/// (non-zero: the entry goes).
struct Closest {
    x0: Vec4,
    x2: Vec4,
    x7: Vec4,
    eax: u32,
}

impl Closest {
    fn rejected() -> Self {
        Closest {
            x0: [0.0; 4],
            x2: [0.0; 4],
            x7: [0.0; 4],
            eax: 1,
        }
    }
}

/// The end the two four-vertex cases `00dae5dd` and `00daea68` share (`00daec44`): from the
/// gathered row sums in `x1` and `x4`, the two slots `v_30` and `v_60` and the scaled normal `x3`
/// it makes `MOVMSKPS(sums < 0)`; `XMM7` is the normal, negated when bit 3 is set, and `EAX` is
/// `!mask & 7`.
fn quad_tail(e: &Engine, x1: Vec4, x3: Vec4, x4: Vec4, v_30: Vec4, v_60: Vec4) -> (Vec4, u32) {
    let x5 = v_60;
    let x1 = shufps(x1, x4, 0x88);
    let x4 = v_30;
    let x6 = x4;
    let x4 = shufps(x4, x5, 0x88);
    let x6 = shufps(x6, x5, 0xdd);
    let x6 = addps(x6, x4);
    let x1 = addps(x1, x6);
    let x1 = cmpltps(x1, [0.0; 4]);
    let mask = movmskps(x1);
    let mut x7 = x3;
    if mask & 8 != 0 {
        let signs = load_vec(e, SIGN_BITS);
        x7 = xorps(x3, signs);
    }
    (x7, !mask & 7)
}

/// `00dae3a0` case (1, 1) (`00dae438`): the entry's two vertices `P0` and `P1`. The squared
/// length `L` of `P0 - P1` (`z + (y + x)`) is compared with the float at `points + 0x11c`: when
/// that is below `L` the entry is dropped, otherwise `XMM0` is `P0`, `XMM2` is `P0 - P1` and `XMM7`
/// the vector at `points + 0x100`.
fn case_one_one(e: &Engine, edi: u32, esi: u32) -> Closest {
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00dae438: MOVZX EAX,byte ptr [EDI + 0x4]
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00dae43c: MOVZX ECX,byte ptr [EDI + 0x5]
    let x4 = load_vec(e, eax.wrapping_add(esi)); // 00dae440: MOVAPS XMM4,xmmword ptr [EAX + ESI*0x1]
    let x1 = load_vec(e, ecx.wrapping_add(esi)); // 00dae444: MOVAPS XMM1,xmmword ptr [ECX + ESI*0x1]
    let x3 = x4; // 00dae44a: MOVAPS XMM3,XMM4
    let x3 = subps(x3, x1); // 00dae44d: SUBPS XMM3,XMM1
    let x1 = x3; // 00dae450: MOVAPS XMM1,XMM3
    let x1 = mulps(x1, x3); // 00dae453: MULPS XMM1,XMM3
    let x5 = x1; // 00dae456: MOVAPS XMM5,XMM1
    let x5 = shufps(x5, x1, 0x55); // 00dae459: SHUFPS XMM5,XMM1,0x55
    let x6 = x1; // 00dae45d: MOVAPS XMM6,XMM1
    let x5 = addss(x5, x1); // 00dae460: ADDSS XMM5,XMM1
    let x6 = shufps(x6, x1, 0xaa); // 00dae464: SHUFPS XMM6,XMM1,0xaa
    let x6 = addss(x6, x5); // 00dae468: ADDSS XMM6,XMM5
    let d_78 = x6[0].to_bits(); // 00dae46c: MOVSS dword ptr [ESP + 0x78],XMM6
    let length = x87_load32(d_78);
    let limit = x87_load32(e.mem.u32(esi.wrapping_add(0x11c)));
    if st0_below(limit, length) {
        return Closest::rejected();
    }
    let x7 = load_vec(e, esi.wrapping_add(0x100)); // 00dae48f: MOVAPS XMM7,xmmword ptr [ESI + 0x100]
    let x2 = x3; // 00dae496: MOVAPS XMM2,XMM3
    let x0 = x4; // 00dae499: MOVAPS XMM0,XMM4
                 // 00dae49c: XOR EAX,EAX
                 // 00dae49e: JMP 0x00daec8c
    Closest { x0, x2, x7, eax: 0 }
}

/// `00dae3a0` case (1, 2) (`00dae4a3`): the entry's three vertices `P0`, `P1`, `P2` (bytes `+4`
/// to `+6`). The exe's shuffles gather four sums in `XMM4`: lane 0 is `(P2 - P0) . (P2 - P1)`,
/// lane 1 is `(P0 - P1) . (P2 - P1)` (each as `z + (y + x)`), lane 2 the squared length of the
/// cross product and lane 3 what its `SHUFPS` ladder leaves. The entry goes when `M * |P2 - P1|^2`
/// is below lane 3 (`M` the float at `points + 0x11c`) or when lane 0 or 1 is negative.
/// Otherwise `XMM0 = P0`, `XMM2 = P0 - Q` for `Q = (1 - t) * P1 + t * P2`, `t = lane0 / (lane0 +
/// lane1)`, and `XMM7` is the vector at `points + 0x100`.
fn case_one_two(e: &Engine, edi: u32, esi: u32) -> Closest {
    let edx = u32::from(e.mem.u8(edi.wrapping_add(0x6))); // 00dae4a3: MOVZX EDX,byte ptr [EDI + 0x6]
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00dae4a7: MOVZX EAX,byte ptr [EDI + 0x4]
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00dae4ab: MOVZX ECX,byte ptr [EDI + 0x5]
    let x4 = load_vec(e, edx.wrapping_add(esi)); // 00dae4af: MOVAPS XMM4,xmmword ptr [EDX + ESI*0x1]
    let x0 = load_vec(e, eax.wrapping_add(esi)); // 00dae4b3: MOVAPS XMM0,xmmword ptr [EAX + ESI*0x1]
    let x3 = load_vec(e, ecx.wrapping_add(esi)); // 00dae4b7: MOVAPS XMM3,xmmword ptr [ECX + ESI*0x1]
    let v_20 = x3; // 00dae4bb: MOVAPS xmmword ptr [ESP + 0x20],XMM3
    let x1 = x4; // 00dae4c0: MOVAPS XMM1,XMM4
    let x1 = subps(x1, x3); // 00dae4c3: SUBPS XMM1,XMM3
    let v_40 = x4; // 00dae4ca: MOVAPS xmmword ptr [ESP + 0x40],XMM4
    let x4 = subps(x4, x0); // 00dae4cf: SUBPS XMM4,XMM0
    let x3 = x4; // 00dae4d4: MOVAPS XMM3,XMM4
    let x3 = shufps(x3, x4, 0xc9); // 00dae4d7: SHUFPS XMM3,XMM4,0xc9
    let x5 = x1; // 00dae4db: MOVAPS XMM5,XMM1
    let x5 = shufps(x5, x1, 0xd2); // 00dae4de: SHUFPS XMM5,XMM1,0xd2
    let x3 = mulps(x3, x5); // 00dae4e2: MULPS XMM3,XMM5
    let x5 = x4; // 00dae4e5: MOVAPS XMM5,XMM4
    let x5 = shufps(x5, x4, 0xd2); // 00dae4e8: SHUFPS XMM5,XMM4,0xd2
    let x6 = x1; // 00dae4ec: MOVAPS XMM6,XMM1
    let x6 = shufps(x6, x1, 0xc9); // 00dae4ef: SHUFPS XMM6,XMM1,0xc9
    let x5 = mulps(x5, x6); // 00dae4f3: MULPS XMM5,XMM6
    let x5 = subps(x5, x3); // 00dae4f6: SUBPS XMM5,XMM3
    let x5 = mulps(x5, x5); // 00dae4f9: MULPS XMM5,XMM5
    let v_50 = x5; // 00dae4fc: MOVAPS xmmword ptr [ESP + 0x50],XMM5
    let x4 = mulps(x4, x1); // 00dae501: MULPS XMM4,XMM1
    let x7 = load_vec(e, esi.wrapping_add(0x100)); // 00dae504: MOVAPS XMM7,xmmword ptr [ESI + 0x100]
    let x5 = x4; // 00dae50b: MOVAPS XMM5,XMM4
    let x3 = x0; // 00dae50e: MOVAPS XMM3,XMM0
    let x3 = subps(x3, v_20); // 00dae511: SUBPS XMM3,xmmword ptr [ESP + 0x20]
    let x3 = mulps(x3, x1); // 00dae516: MULPS XMM3,XMM1
    let x5 = unpcklps(x5, x3); // 00dae519: UNPCKLPS XMM5,XMM3
    let x4 = unpckhps(x4, x3); // 00dae51c: UNPCKHPS XMM4,XMM3
    let x3 = v_50; // 00dae51f: MOVAPS XMM3,xmmword ptr [ESP + 0x50]
    let x6 = x5; // 00dae524: MOVAPS XMM6,XMM5
    let x6 = movlhps(x6, x3); // 00dae527: MOVLHPS XMM6,XMM3
    let x3 = x6; // 00dae52a: MOVAPS XMM3,XMM6
    let x3 = movhlps(x3, x5); // 00dae52d: MOVHLPS XMM3,XMM5
    let x5 = v_50; // 00dae530: MOVAPS XMM5,xmmword ptr [ESP + 0x50]
    let x3 = shufps(x3, x5, 0xd4); // 00dae535: SHUFPS XMM3,XMM5,0xd4
    let x4 = shufps(x4, x5, 0xe4); // 00dae539: SHUFPS XMM4,XMM5,0xe4
    let x3 = addps(x3, x6); // 00dae53d: ADDPS XMM3,XMM6
    let x1 = mulps(x1, x1); // 00dae540: MULPS XMM1,XMM1
    let x4 = addps(x4, x3); // 00dae543: ADDPS XMM4,XMM3
    let x3 = x1; // 00dae546: MOVAPS XMM3,XMM1
    let v_100 = x4; // 00dae549: MOVAPS xmmword ptr [ESP + 0x100],XMM4
    let l4 = x87_load32(v_100[3].to_bits());
    let limit = x87_load32(e.mem.u32(esi.wrapping_add(0x11c)));
    let x3 = shufps(x3, x1, 0x55); // 00dae55e: SHUFPS XMM3,XMM1,0x55
    let x5 = x1; // 00dae562: MOVAPS XMM5,XMM1
    let x3 = addss(x3, x1); // 00dae565: ADDSS XMM3,XMM1
    let x5 = shufps(x5, x1, 0xaa); // 00dae569: SHUFPS XMM5,XMM1,0xaa
    let x5 = addss(x5, x3); // 00dae56d: ADDSS XMM5,XMM3
    let d_7c = x5[0].to_bits(); // 00dae571: MOVSS dword ptr [ESP + 0x7c],XMM5
    let product = x87_mul(limit, x87_m32(d_7c));
    if st0_below(product, l4) {
        return Closest::rejected();
    }
    let x1 = x4; // 00dae588: MOVAPS XMM1,XMM4
    let x1 = cmpltps(x1, [0.0; 4]); // 00dae58b: CMPLTPS XMM1,xmmword ptr [ESP + 0x90]
    let edx = movmskps(x1); // 00dae594: MOVMSKPS EDX,XMM1
    if edx & 3 != 0 {
        return Closest::rejected();
    }
    let x3 = load_vec(e, 0x010ec7f0); // 00dae5a0: MOVAPS XMM3,xmmword ptr [0x010ec7f0]
    let x1 = x4; // 00dae5a7: MOVAPS XMM1,XMM4
    let x1 = shufps(x1, x4, 0x55); // 00dae5aa: SHUFPS XMM1,XMM4,0x55
    let x2 = x4; // 00dae5ae: MOVAPS XMM2,XMM4
    let x2 = addss(x2, x1); // 00dae5b1: ADDSS XMM2,XMM1
    let x1 = v_40; // 00dae5b5: MOVAPS XMM1,xmmword ptr [ESP + 0x40]
    let x4 = divss(x4, x2); // 00dae5ba: DIVSS XMM4,XMM2
    let x4 = shufps(x4, x4, 0x0); // 00dae5be: SHUFPS XMM4,XMM4,0x0
    let x3 = subps(x3, x4); // 00dae5c2: SUBPS XMM3,XMM4
    let x3 = mulps(x3, v_20); // 00dae5c5: MULPS XMM3,xmmword ptr [ESP + 0x20]
    let x1 = mulps(x1, x4); // 00dae5ca: MULPS XMM1,XMM4
    let x3 = addps(x3, x1); // 00dae5cd: ADDPS XMM3,XMM1
    let x2 = x0; // 00dae5d0: MOVAPS XMM2,XMM0
    let x2 = subps(x2, x3); // 00dae5d3: SUBPS XMM2,XMM3
                            // 00dae5d6: XOR EAX,EAX
                            // 00dae5d8: JMP 0x00daec8c
    Closest { x0, x2, x7, eax: 0 }
}

/// `00dae3a0` case (1, 3) (`00dae5dd`): the entry's four vertices `P0..P3` (bytes `+4` to
/// `+7`). `XMM0` is `P0`, `XMM2` is `P0 - P1`, and the sign test of `quad_tail` (on the normal of
/// `(P3 - P2) x (P1 - P3)` scaled to unit length with the `RSQRTSS` estimate) gives `XMM7` and
/// `EAX`.
fn case_one_three(e: &Engine, edi: u32, esi: u32) -> Closest {
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00dae5dd: MOVZX EAX,byte ptr [EDI + 0x4]
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00dae5e1: MOVZX ECX,byte ptr [EDI + 0x5]
    let edx = u32::from(e.mem.u8(edi.wrapping_add(0x6))); // 00dae5e5: MOVZX EDX,byte ptr [EDI + 0x6]
    let ebx = u32::from(e.mem.u8(edi.wrapping_add(0x7))); // 00dae5e9: MOVZX EBX,byte ptr [EDI + 0x7]
    let x6 = load_vec(e, ebx.wrapping_add(esi)); // 00dae5ed: MOVAPS XMM6,xmmword ptr [EBX + ESI*0x1]
    let x4 = load_vec(e, ecx.wrapping_add(esi)); // 00dae5f1: MOVAPS XMM4,xmmword ptr [ECX + ESI*0x1]
    let x5 = load_vec(e, edx.wrapping_add(esi)); // 00dae5f5: MOVAPS XMM5,xmmword ptr [EDX + ESI*0x1]
    let x0 = load_vec(e, eax.wrapping_add(esi)); // 00dae5f9: MOVAPS XMM0,xmmword ptr [EAX + ESI*0x1]
    let x1 = x6; // 00dae5fd: MOVAPS XMM1,XMM6
    let x1 = subps(x1, x5); // 00dae600: SUBPS XMM1,XMM5
    let x7 = x1; // 00dae603: MOVAPS XMM7,XMM1
    let x7 = shufps(x7, x1, 0xd2); // 00dae606: SHUFPS XMM7,XMM1,0xd2
    let x1 = shufps(x1, x1, 0xc9); // 00dae60a: SHUFPS XMM1,XMM1,0xc9
    let v_50 = x6; // 00dae60e: MOVAPS xmmword ptr [ESP + 0x50],XMM6
    let v_a0 = x1; // 00dae613: MOVAPS xmmword ptr [ESP + 0xa0],XMM1
    let x3 = x4; // 00dae61b: MOVAPS XMM3,XMM4
    let x3 = subps(x3, x6); // 00dae61e: SUBPS XMM3,XMM6
    let x6 = x3; // 00dae621: MOVAPS XMM6,XMM3
    let x6 = shufps(x6, x3, 0xc9); // 00dae624: SHUFPS XMM6,XMM3,0xc9
    let x3 = shufps(x3, x3, 0xd2); // 00dae628: SHUFPS XMM3,XMM3,0xd2
    let v_60 = x3; // 00dae634: MOVAPS xmmword ptr [ESP + 0x60],XMM3
    let v_b0 = x7; // 00dae639: MOVAPS xmmword ptr [ESP + 0xb0],XMM7
    let x7 = mulps(x7, x6); // 00dae641: MULPS XMM7,XMM6
    let v_20 = x5; // 00dae644: MOVAPS xmmword ptr [ESP + 0x20],XMM5
    let x5 = subps(x5, x4); // 00dae649: SUBPS XMM5,XMM4
    let x2 = x0; // 00dae64c: MOVAPS XMM2,XMM0
    let x2 = subps(x2, x4); // 00dae64f: SUBPS XMM2,XMM4
    let x4 = x1; // 00dae652: MOVAPS XMM4,XMM1
    let x4 = mulps(x4, x3); // 00dae655: MULPS XMM4,XMM3
    let x4 = subps(x4, x7); // 00dae658: SUBPS XMM4,XMM7
    let x1 = x4; // 00dae65b: MOVAPS XMM1,XMM4
    let x1 = mulps(x1, x4); // 00dae65e: MULPS XMM1,XMM4
    let x3 = x1; // 00dae661: MOVAPS XMM3,XMM1
    let x3 = shufps(x3, x1, 0x55); // 00dae664: SHUFPS XMM3,XMM1,0x55
    let x3 = addss(x3, x1); // 00dae668: ADDSS XMM3,XMM1
    let x1 = shufps(x1, x1, 0xaa); // 00dae66c: SHUFPS XMM1,XMM1,0xaa
    let x1 = addss(x1, x3); // 00dae670: ADDSS XMM1,XMM3
    let x3 = x1; // 00dae674: MOVAPS XMM3,XMM1
    let v_30 = x6; // 00dae677: MOVAPS xmmword ptr [ESP + 0x30],XMM6
    let x3 = rsqrtss(x3, x3); // 00dae67c: RSQRTSS XMM3,XMM3
    let x6 = x3; // 00dae680: MOVAPS XMM6,XMM3
    let x3 = scalar_vec(e, 0x01011d78); // 00dae683: MOVSS XMM3,dword ptr [0x01011d78]
    let v_40 = x6; // 00dae68b: MOVAPS xmmword ptr [ESP + 0x40],XMM6
    let x7 = x3; // 00dae690: MOVAPS XMM7,XMM3
    let x3 = x1; // 00dae693: MOVAPS XMM3,XMM1
    let x1 = mulss(x1, x6); // 00dae696: MULSS XMM1,XMM6
    let x1 = mulss(x1, x6); // 00dae69a: MULSS XMM1,XMM6
    let x6 = scalar_vec(e, 0x01017718); // 00dae69e: MOVSS XMM6,dword ptr [0x01017718]
    let x6 = subss(x6, x1); // 00dae6a6: SUBSS XMM6,XMM1
    let x1 = scalar_vec(e, 0x01016248); // 00dae6aa: MOVSS XMM1,dword ptr [0x01016248]
    let x1 = mulss(x1, [v_40[0], 0.0, 0.0, 0.0]); // 00dae6b2: MULSS XMM1,dword ptr [ESP + 0x40]
    let x1 = mulss(x1, x6); // 00dae6b8: MULSS XMM1,XMM6
    let x3 = cmpeqss(x3, x7); // 00dae6bc: CMPEQSS XMM3,XMM7
    let x3 = andnps(x3, x1); // 00dae6c1: ANDNPS XMM3,XMM1
    let x1 = x0; // 00dae6c4: MOVAPS XMM1,XMM0
    let x1 = subps(x1, v_20); // 00dae6c7: SUBPS XMM1,xmmword ptr [ESP + 0x20]
    let x6 = x1; // 00dae6cc: MOVAPS XMM6,XMM1
    let x6 = shufps(x6, x1, 0xd2); // 00dae6cf: SHUFPS XMM6,XMM1,0xd2
    let x6 = mulps(x6, v_a0); // 00dae6d3: MULPS XMM6,xmmword ptr [ESP + 0xa0]
    let x1 = shufps(x1, x1, 0xc9); // 00dae6db: SHUFPS XMM1,XMM1,0xc9
    let x1 = mulps(x1, v_b0); // 00dae6df: MULPS XMM1,xmmword ptr [ESP + 0xb0]
    let x1 = subps(x1, x6); // 00dae6e7: SUBPS XMM1,XMM6
    let x3 = shufps(x3, x3, 0x0); // 00dae6ea: SHUFPS XMM3,XMM3,0x0
    let x3 = mulps(x3, x4); // 00dae6ee: MULPS XMM3,XMM4
    let x4 = x0; // 00dae6f1: MOVAPS XMM4,XMM0
    let x4 = subps(x4, v_50); // 00dae6f4: SUBPS XMM4,xmmword ptr [ESP + 0x50]
    let x6 = x4; // 00dae6f9: MOVAPS XMM6,XMM4
    let x6 = shufps(x6, x4, 0xc9); // 00dae6fc: SHUFPS XMM6,XMM4,0xc9
    let x6 = mulps(x6, v_60); // 00dae700: MULPS XMM6,xmmword ptr [ESP + 0x60]
    let x7 = x4; // 00dae705: MOVAPS XMM7,XMM4
    let x7 = shufps(x7, x4, 0xd2); // 00dae708: SHUFPS XMM7,XMM4,0xd2
    let x7 = mulps(x7, v_30); // 00dae70c: MULPS XMM7,xmmword ptr [ESP + 0x30]
    let x6 = subps(x6, x7); // 00dae711: SUBPS XMM6,XMM7
    let x4 = x2; // 00dae714: MOVAPS XMM4,XMM2
    let x1 = mulps(x1, x3); // 00dae717: MULPS XMM1,XMM3
    let x6 = mulps(x6, x3); // 00dae71a: MULPS XMM6,XMM3
    let x4 = shufps(x4, x2, 0xc9); // 00dae71d: SHUFPS XMM4,XMM2,0xc9
    let x7 = x5; // 00dae721: MOVAPS XMM7,XMM5
    let x7 = shufps(x7, x5, 0xd2); // 00dae724: SHUFPS XMM7,XMM5,0xd2
    let x4 = mulps(x4, x7); // 00dae728: MULPS XMM4,XMM7
    let x5 = shufps(x5, x5, 0xc9); // 00dae72b: SHUFPS XMM5,XMM5,0xc9
    let x7 = x2; // 00dae72f: MOVAPS XMM7,XMM2
    let x7 = shufps(x7, x2, 0xd2); // 00dae732: SHUFPS XMM7,XMM2,0xd2
    let x7 = mulps(x7, x5); // 00dae736: MULPS XMM7,XMM5
    let x5 = load_vec(e, esi.wrapping_add(0x100)); // 00dae739: MOVAPS XMM5,xmmword ptr [ESI + 0x100]
    let x4 = subps(x4, x7); // 00dae740: SUBPS XMM4,XMM7
    let x7 = x1; // 00dae743: MOVAPS XMM7,XMM1
    let x7 = shufps(x7, x6, 0x44); // 00dae746: SHUFPS XMM7,XMM6,0x44
    let x4 = mulps(x4, x3); // 00dae74a: MULPS XMM4,XMM3
    let v_30 = x7; // 00dae74d: MOVAPS xmmword ptr [ESP + 0x30],XMM7
    let x5 = mulps(x5, x3); // 00dae752: MULPS XMM5,XMM3
    let x7 = x4; // 00dae755: MOVAPS XMM7,XMM4
    let x7 = shufps(x7, x5, 0x44); // 00dae758: SHUFPS XMM7,XMM5,0x44
    let v_60 = x7; // 00dae75c: MOVAPS xmmword ptr [ESP + 0x60],XMM7
    let x1 = shufps(x1, x6, 0xee); // 00dae761: SHUFPS XMM1,XMM6,0xee
    let x4 = shufps(x4, x5, 0xee); // 00dae765: SHUFPS XMM4,XMM5,0xee
                                   // 00dae769: JMP 0x00daec44
    let (x7, eax) = quad_tail(e, x1, x3, x4, v_30, v_60);
    Closest { x0, x2, x7, eax }
}

/// `00dae3a0` case (2, 1) (`00dae76e`): the entry's three vertices `P0`, `P1`, `P2`. Like case
/// (1, 2) with lane 0 `(P1 - P2) . (P1 - P0)` and lane 1 `(P2 - P0) . (P1 - P0)`; the entry goes
/// when `M * |P1 - P0|^2` is below lane 3 or lane 0 or 1 is negative. Otherwise `XMM0 = Q = (1 -
/// t) * P1 + t * P0` for `t = lane0 / (lane0 + lane1)`, `XMM2 = Q - P2` and `XMM7` is the vector
/// at `points + 0x100`.
fn case_two_one(e: &Engine, edi: u32, esi: u32) -> Closest {
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00dae76e: MOVZX ECX,byte ptr [EDI + 0x5]
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00dae772: MOVZX EAX,byte ptr [EDI + 0x4]
    let edx = u32::from(e.mem.u8(edi.wrapping_add(0x6))); // 00dae776: MOVZX EDX,byte ptr [EDI + 0x6]
    let x3 = load_vec(e, eax.wrapping_add(esi)); // 00dae77a: MOVAPS XMM3,xmmword ptr [EAX + ESI*0x1]
    let x4 = load_vec(e, ecx.wrapping_add(esi)); // 00dae77e: MOVAPS XMM4,xmmword ptr [ECX + ESI*0x1]
    let v_50 = x3; // 00dae782: MOVAPS xmmword ptr [ESP + 0x50],XMM3
    let x1 = x4; // 00dae787: MOVAPS XMM1,XMM4
    let x1 = subps(x1, x3); // 00dae78a: SUBPS XMM1,XMM3
    let x3 = load_vec(e, edx.wrapping_add(esi)); // 00dae78d: MOVAPS XMM3,xmmword ptr [EDX + ESI*0x1]
    let v_40 = x3; // 00dae791: MOVAPS xmmword ptr [ESP + 0x40],XMM3
    let v_30 = x4; // 00dae79a: MOVAPS xmmword ptr [ESP + 0x30],XMM4
    let x4 = subps(x4, x3); // 00dae79f: SUBPS XMM4,XMM3
    let x3 = x4; // 00dae7a4: MOVAPS XMM3,XMM4
    let x3 = shufps(x3, x4, 0xc9); // 00dae7a7: SHUFPS XMM3,XMM4,0xc9
    let x5 = x1; // 00dae7ab: MOVAPS XMM5,XMM1
    let x5 = shufps(x5, x1, 0xd2); // 00dae7ae: SHUFPS XMM5,XMM1,0xd2
    let x3 = mulps(x3, x5); // 00dae7b2: MULPS XMM3,XMM5
    let x5 = x4; // 00dae7b5: MOVAPS XMM5,XMM4
    let x5 = shufps(x5, x4, 0xd2); // 00dae7b8: SHUFPS XMM5,XMM4,0xd2
    let x6 = x1; // 00dae7bc: MOVAPS XMM6,XMM1
    let x6 = shufps(x6, x1, 0xc9); // 00dae7bf: SHUFPS XMM6,XMM1,0xc9
    let x5 = mulps(x5, x6); // 00dae7c3: MULPS XMM5,XMM6
    let x5 = subps(x5, x3); // 00dae7c6: SUBPS XMM5,XMM3
    let x3 = v_40; // 00dae7c9: MOVAPS XMM3,xmmword ptr [ESP + 0x40]
    let x3 = subps(x3, v_50); // 00dae7ce: SUBPS XMM3,xmmword ptr [ESP + 0x50]
    let x3 = mulps(x3, x1); // 00dae7d3: MULPS XMM3,XMM1
    let x5 = mulps(x5, x5); // 00dae7d6: MULPS XMM5,XMM5
    let v_20 = x5; // 00dae7d9: MOVAPS xmmword ptr [ESP + 0x20],XMM5
    let x4 = mulps(x4, x1); // 00dae7de: MULPS XMM4,XMM1
    let x7 = load_vec(e, esi.wrapping_add(0x100)); // 00dae7e1: MOVAPS XMM7,xmmword ptr [ESI + 0x100]
    let x5 = x4; // 00dae7e8: MOVAPS XMM5,XMM4
    let x5 = unpcklps(x5, x3); // 00dae7eb: UNPCKLPS XMM5,XMM3
    let x4 = unpckhps(x4, x3); // 00dae7ee: UNPCKHPS XMM4,XMM3
    let x3 = v_20; // 00dae7f1: MOVAPS XMM3,xmmword ptr [ESP + 0x20]
    let x6 = x5; // 00dae7f6: MOVAPS XMM6,XMM5
    let x6 = movlhps(x6, x3); // 00dae7f9: MOVLHPS XMM6,XMM3
    let x3 = x6; // 00dae7fc: MOVAPS XMM3,XMM6
    let x3 = movhlps(x3, x5); // 00dae7ff: MOVHLPS XMM3,XMM5
    let x5 = v_20; // 00dae802: MOVAPS XMM5,xmmword ptr [ESP + 0x20]
    let x3 = shufps(x3, x5, 0xd4); // 00dae807: SHUFPS XMM3,XMM5,0xd4
    let x4 = shufps(x4, x5, 0xe4); // 00dae80b: SHUFPS XMM4,XMM5,0xe4
    let x3 = addps(x3, x6); // 00dae80f: ADDPS XMM3,XMM6
    let x1 = mulps(x1, x1); // 00dae812: MULPS XMM1,XMM1
    let x4 = addps(x4, x3); // 00dae815: ADDPS XMM4,XMM3
    let x3 = x1; // 00dae818: MOVAPS XMM3,XMM1
    let v_e0 = x4; // 00dae81b: MOVAPS xmmword ptr [ESP + 0xe0],XMM4
    let l4 = x87_load32(v_e0[3].to_bits());
    let limit = x87_load32(e.mem.u32(esi.wrapping_add(0x11c)));
    let x3 = shufps(x3, x1, 0x55); // 00dae830: SHUFPS XMM3,XMM1,0x55
    let x5 = x1; // 00dae834: MOVAPS XMM5,XMM1
    let x3 = addss(x3, x1); // 00dae837: ADDSS XMM3,XMM1
    let x5 = shufps(x5, x1, 0xaa); // 00dae83b: SHUFPS XMM5,XMM1,0xaa
    let x5 = addss(x5, x3); // 00dae83f: ADDSS XMM5,XMM3
    let d_80 = x5[0].to_bits(); // 00dae843: MOVSS dword ptr [ESP + 0x80],XMM5
    let product = x87_mul(limit, x87_m32(d_80));
    if st0_below(product, l4) {
        return Closest::rejected();
    }
    let x1 = x4; // 00dae860: MOVAPS XMM1,XMM4
    let x1 = cmpltps(x1, [0.0; 4]); // 00dae863: CMPLTPS XMM1,xmmword ptr [ESP + 0x90]
    let eax = movmskps(x1); // 00dae86c: MOVMSKPS EAX,XMM1
    if eax & 3 != 0 {
        return Closest::rejected();
    }
    let x0 = x4; // 00dae877: MOVAPS XMM0,XMM4
    let x0 = shufps(x0, x4, 0x55); // 00dae87a: SHUFPS XMM0,XMM4,0x55
    let x1 = x4; // 00dae87e: MOVAPS XMM1,XMM4
    let x1 = addss(x1, x0); // 00dae881: ADDSS XMM1,XMM0
    let x0 = load_vec(e, 0x010ec7f0); // 00dae885: MOVAPS XMM0,xmmword ptr [0x010ec7f0]
    let x4 = divss(x4, x1); // 00dae88c: DIVSS XMM4,XMM1
    let x4 = shufps(x4, x4, 0x0); // 00dae890: SHUFPS XMM4,XMM4,0x0
    let x0 = subps(x0, x4); // 00dae894: SUBPS XMM0,XMM4
    let x0 = mulps(x0, v_30); // 00dae897: MULPS XMM0,xmmword ptr [ESP + 0x30]
    let x4 = mulps(x4, v_50); // 00dae89c: MULPS XMM4,xmmword ptr [ESP + 0x50]
    let x0 = addps(x0, x4); // 00dae8a1: ADDPS XMM0,XMM4
    let x2 = x0; // 00dae8a4: MOVAPS XMM2,XMM0
    let x2 = subps(x2, v_40); // 00dae8a7: SUBPS XMM2,xmmword ptr [ESP + 0x40]
                              // 00dae8ac: XOR EAX,EAX
                              // 00dae8ae: JMP 0x00daec8c
    Closest { x0, x2, x7, eax: 0 }
}

/// `00dae3a0` case (2, 2) (`00dae8b3`): the entry's four vertices as two segments `P0 P1` and
/// `P2 P3`. With `N = (P1 - P0) x (P3 - P2)`, `M0 = (P3 - P2) x N` and `M2 = (P1 - P0) x N`, the
/// four sums `M0 . (P0 - P2)`, `M2 . (P0 - P2)`, `M0 . (P1 - P3)` and `M2 . (P1 - P3)` must have
/// the sign mask 3 or 0xc, else the entry goes. Then `XMM0` is the closest point `Q0` of the
/// first segment, `XMM2` is `Q0 - Q1` for the closest point `Q1` of the second (as in `00dad340`)
/// and `XMM7` the vector at `points + 0x100`.
fn case_two_two(e: &Engine, edi: u32, esi: u32) -> Closest {
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00dae8b3: MOVZX ECX,byte ptr [EDI + 0x5]
    let ebx = u32::from(e.mem.u8(edi.wrapping_add(0x7))); // 00dae8b7: MOVZX EBX,byte ptr [EDI + 0x7]
    let edx = u32::from(e.mem.u8(edi.wrapping_add(0x6))); // 00dae8bb: MOVZX EDX,byte ptr [EDI + 0x6]
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00dae8bf: MOVZX EAX,byte ptr [EDI + 0x4]
    let x4 = load_vec(e, ecx.wrapping_add(esi)); // 00dae8c3: MOVAPS XMM4,xmmword ptr [ECX + ESI*0x1]
    let x7 = load_vec(e, ebx.wrapping_add(esi)); // 00dae8c7: MOVAPS XMM7,xmmword ptr [EBX + ESI*0x1]
    let x5 = load_vec(e, edx.wrapping_add(esi)); // 00dae8cb: MOVAPS XMM5,xmmword ptr [EDX + ESI*0x1]
    let x6 = load_vec(e, eax.wrapping_add(esi)); // 00dae8cf: MOVAPS XMM6,xmmword ptr [EAX + ESI*0x1]
    let x1 = x4; // 00dae8d3: MOVAPS XMM1,XMM4
    let x1 = subps(x1, x6); // 00dae8d6: SUBPS XMM1,XMM6
    let v_a0 = x4; // 00dae8d9: MOVAPS xmmword ptr [ESP + 0xa0],XMM4
    let x4 = subps(x4, x7); // 00dae8e1: SUBPS XMM4,XMM7
    let v_d0 = x4; // 00dae8e4: MOVAPS xmmword ptr [ESP + 0xd0],XMM4
    let v_40 = x5; // 00dae8ec: MOVAPS xmmword ptr [ESP + 0x40],XMM5
    let x3 = x7; // 00dae8f5: MOVAPS XMM3,XMM7
    let x3 = subps(x3, x5); // 00dae8f8: SUBPS XMM3,XMM5
    let x4 = x3; // 00dae8fb: MOVAPS XMM4,XMM3
    let x4 = shufps(x4, x3, 0xc9); // 00dae8fe: SHUFPS XMM4,XMM3,0xc9
    let x3 = shufps(x3, x3, 0xd2); // 00dae902: SHUFPS XMM3,XMM3,0xd2
    let v_20 = x7; // 00dae90a: MOVAPS xmmword ptr [ESP + 0x20],XMM7
    let x7 = x1; // 00dae90f: MOVAPS XMM7,XMM1
    let x7 = shufps(x7, x1, 0xd2); // 00dae912: SHUFPS XMM7,XMM1,0xd2
    let x1 = shufps(x1, x1, 0xc9); // 00dae916: SHUFPS XMM1,XMM1,0xc9
    let v_b0 = x6; // 00dae91a: MOVAPS xmmword ptr [ESP + 0xb0],XMM6
    let x6 = subps(x6, x5); // 00dae922: SUBPS XMM6,XMM5
    let x5 = x3; // 00dae925: MOVAPS XMM5,XMM3
    let x3 = x1; // 00dae928: MOVAPS XMM3,XMM1
    let x1 = mulps(x1, x5); // 00dae92b: MULPS XMM1,XMM5
    let v_30 = x5; // 00dae92e: MOVAPS xmmword ptr [ESP + 0x30],XMM5
    let v_60 = x4; // 00dae933: MOVAPS xmmword ptr [ESP + 0x60],XMM4
    let x5 = x7; // 00dae938: MOVAPS XMM5,XMM7
    let x5 = mulps(x5, x4); // 00dae93b: MULPS XMM5,XMM4
    let x1 = subps(x1, x5); // 00dae93e: SUBPS XMM1,XMM5
    let x5 = x1; // 00dae941: MOVAPS XMM5,XMM1
    let x5 = shufps(x5, x1, 0xc9); // 00dae944: SHUFPS XMM5,XMM1,0xc9
    let x1 = shufps(x1, x1, 0xd2); // 00dae948: SHUFPS XMM1,XMM1,0xd2
    let x4 = x1; // 00dae94c: MOVAPS XMM4,XMM1
    let x1 = mulps(x1, v_60); // 00dae94f: MULPS XMM1,xmmword ptr [ESP + 0x60]
    let x4 = mulps(x4, x3); // 00dae954: MULPS XMM4,XMM3
    let x3 = x5; // 00dae957: MOVAPS XMM3,XMM5
    let x5 = mulps(x5, v_30); // 00dae95a: MULPS XMM5,xmmword ptr [ESP + 0x30]
    let x1 = subps(x1, x5); // 00dae95f: SUBPS XMM1,XMM5
    let x3 = mulps(x3, x7); // 00dae962: MULPS XMM3,XMM7
    let x4 = subps(x4, x3); // 00dae965: SUBPS XMM4,XMM3
    let x3 = v_d0; // 00dae968: MOVAPS XMM3,xmmword ptr [ESP + 0xd0]
    let x5 = x1; // 00dae970: MOVAPS XMM5,XMM1
    let x1 = load_vec(e, esi.wrapping_add(0x100)); // 00dae973: MOVAPS XMM1,xmmword ptr [ESI + 0x100]
    let v_130 = x1; // 00dae97a: MOVAPS xmmword ptr [ESP + 0x130],XMM1
    let x1 = x5; // 00dae982: MOVAPS XMM1,XMM5
    let x5 = mulps(x5, x3); // 00dae985: MULPS XMM5,XMM3
    let x1 = mulps(x1, x6); // 00dae988: MULPS XMM1,XMM6
    let x7 = x4; // 00dae98b: MOVAPS XMM7,XMM4
    let x4 = mulps(x4, x3); // 00dae98e: MULPS XMM4,XMM3
    let x7 = mulps(x7, x6); // 00dae991: MULPS XMM7,XMM6
    let x6 = x5; // 00dae994: MOVAPS XMM6,XMM5
    let x5 = x1; // 00dae997: MOVAPS XMM5,XMM1
    let x3 = x6; // 00dae99a: MOVAPS XMM3,XMM6
    let x3 = shufps(x3, x4, 0x44); // 00dae99d: SHUFPS XMM3,XMM4,0x44
    let x6 = shufps(x6, x4, 0xee); // 00dae9a1: SHUFPS XMM6,XMM4,0xee
    let x5 = shufps(x5, x7, 0x44); // 00dae9a5: SHUFPS XMM5,XMM7,0x44
    let x4 = x5; // 00dae9a9: MOVAPS XMM4,XMM5
    let x4 = shufps(x4, x3, 0xdd); // 00dae9ac: SHUFPS XMM4,XMM3,0xdd
    let x5 = shufps(x5, x3, 0x88); // 00dae9b0: SHUFPS XMM5,XMM3,0x88
    let x1 = shufps(x1, x7, 0xee); // 00dae9b4: SHUFPS XMM1,XMM7,0xee
    let x1 = shufps(x1, x6, 0x88); // 00dae9b8: SHUFPS XMM1,XMM6,0x88
    let x4 = addps(x4, x5); // 00dae9bc: ADDPS XMM4,XMM5
    let x1 = addps(x1, x4); // 00dae9bf: ADDPS XMM1,XMM4
    let x3 = x1; // 00dae9c2: MOVAPS XMM3,XMM1
    let x3 = cmpltps(x3, [0.0; 4]); // 00dae9c5: CMPLTPS XMM3,xmmword ptr [ESP + 0x90]
    let eax = movmskps(x3); // 00dae9ce: MOVMSKPS EAX,XMM3
    if eax != 3 && eax != 0xc {
        return Closest::rejected();
    }
    let x3 = load_vec(e, 0x010ec7f0); // 00dae9ed: MOVAPS XMM3,xmmword ptr [0x010ec7f0]
    let x7 = v_130; // 00dae9f4: MOVAPS XMM7,xmmword ptr [ESP + 0x130]
    let x0 = x1; // 00dae9fc: MOVAPS XMM0,XMM1
    let x0 = shufps(x0, x1, 0xaa); // 00dae9ff: SHUFPS XMM0,XMM1,0xaa
    let x2 = x1; // 00daea03: MOVAPS XMM2,XMM1
    let x2 = subss(x2, x0); // 00daea06: SUBSS XMM2,XMM0
    let x0 = x1; // 00daea0a: MOVAPS XMM0,XMM1
    let x0 = divss(x0, x2); // 00daea0d: DIVSS XMM0,XMM2
    let x0 = shufps(x0, x0, 0x0); // 00daea11: SHUFPS XMM0,XMM0,0x0
    let x2 = x0; // 00daea15: MOVAPS XMM2,XMM0
    let x0 = x3; // 00daea18: MOVAPS XMM0,XMM3
    let x0 = subps(x0, x2); // 00daea1b: SUBPS XMM0,XMM2
    let x2 = mulps(x2, v_a0); // 00daea1e: MULPS XMM2,xmmword ptr [ESP + 0xa0]
    let x0 = mulps(x0, v_b0); // 00daea26: MULPS XMM0,xmmword ptr [ESP + 0xb0]
    let x0 = addps(x0, x2); // 00daea2e: ADDPS XMM0,XMM2
    let x2 = x1; // 00daea31: MOVAPS XMM2,XMM1
    let x2 = shufps(x2, x1, 0x55); // 00daea34: SHUFPS XMM2,XMM1,0x55
    let x4 = x2; // 00daea38: MOVAPS XMM4,XMM2
    let x1 = shufps(x1, x1, 0xff); // 00daea3b: SHUFPS XMM1,XMM1,0xff
    let x4 = subss(x4, x1); // 00daea3f: SUBSS XMM4,XMM1
    let x2 = divss(x2, x4); // 00daea43: DIVSS XMM2,XMM4
    let x2 = shufps(x2, x2, 0x0); // 00daea47: SHUFPS XMM2,XMM2,0x0
    let x3 = subps(x3, x2); // 00daea4b: SUBPS XMM3,XMM2
    let x2 = mulps(x2, v_20); // 00daea4e: MULPS XMM2,xmmword ptr [ESP + 0x20]
    let x3 = mulps(x3, v_40); // 00daea53: MULPS XMM3,xmmword ptr [ESP + 0x40]
    let x3 = addps(x3, x2); // 00daea58: ADDPS XMM3,XMM2
    let x2 = x0; // 00daea5b: MOVAPS XMM2,XMM0
    let x2 = subps(x2, x3); // 00daea5e: SUBPS XMM2,XMM3
                            // 00daea61: XOR EAX,EAX
                            // 00daea63: JMP 0x00daec88
    Closest { x0, x2, x7, eax: 0 }
}

/// `00dae3a0` case (3, 1) (`00daea68`): the entry's four vertices `P0..P3`. With `u` the unit
/// normal (the `RSQRTSS` estimate refined as in `00dad110`) of `(P2 - P1) x (P0 - P2)`,
/// `XMM0 = ((P0 - P3) . u) * u + P3` and `XMM2 = P0 - P3`; `quad_tail` gives `XMM7` and `EAX`.
fn case_three_one(e: &Engine, edi: u32, esi: u32) -> Closest {
    let edx = u32::from(e.mem.u8(edi.wrapping_add(0x6))); // 00daea68: MOVZX EDX,byte ptr [EDI + 0x6]
    let eax = u32::from(e.mem.u8(edi.wrapping_add(0x4))); // 00daea6c: MOVZX EAX,byte ptr [EDI + 0x4]
    let ecx = u32::from(e.mem.u8(edi.wrapping_add(0x5))); // 00daea70: MOVZX ECX,byte ptr [EDI + 0x5]
    let x3 = load_vec(e, eax.wrapping_add(esi)); // 00daea74: MOVAPS XMM3,xmmword ptr [EAX + ESI*0x1]
    let x6 = load_vec(e, edx.wrapping_add(esi)); // 00daea78: MOVAPS XMM6,xmmword ptr [EDX + ESI*0x1]
    let x4 = load_vec(e, ecx.wrapping_add(esi)); // 00daea7c: MOVAPS XMM4,xmmword ptr [ECX + ESI*0x1]
    let x1 = x3; // 00daea80: MOVAPS XMM1,XMM3
    let x1 = subps(x1, x6); // 00daea83: SUBPS XMM1,XMM6
    let v_a0 = x3; // 00daea86: MOVAPS xmmword ptr [ESP + 0xa0],XMM3
    let x2 = x3; // 00daea8e: MOVAPS XMM2,XMM3
    let x0 = x6; // 00daea91: MOVAPS XMM0,XMM6
    let x0 = subps(x0, x4); // 00daea94: SUBPS XMM0,XMM4
    let ebx = u32::from(e.mem.u8(edi.wrapping_add(0x7))); // 00daea97: MOVZX EBX,byte ptr [EDI + 0x7]
    let x5 = load_vec(e, ebx.wrapping_add(esi)); // 00daea9b: MOVAPS XMM5,xmmword ptr [EBX + ESI*0x1]
    let v_b0 = x6; // 00daeaa1: MOVAPS xmmword ptr [ESP + 0xb0],XMM6
    let x6 = x0; // 00daeaa9: MOVAPS XMM6,XMM0
    let x6 = shufps(x6, x0, 0xd2); // 00daeaac: SHUFPS XMM6,XMM0,0xd2
    let x0 = shufps(x0, x0, 0xc9); // 00daeab0: SHUFPS XMM0,XMM0,0xc9
    let v_40 = x0; // 00daeab8: MOVAPS xmmword ptr [ESP + 0x40],XMM0
    let v_60 = x4; // 00daeabd: MOVAPS xmmword ptr [ESP + 0x60],XMM4
    let x4 = subps(x4, x3); // 00daeac2: SUBPS XMM4,XMM3
    let x3 = x1; // 00daeac5: MOVAPS XMM3,XMM1
    let x3 = shufps(x3, x1, 0xc9); // 00daeac8: SHUFPS XMM3,XMM1,0xc9
    let x1 = shufps(x1, x1, 0xd2); // 00daeacc: SHUFPS XMM1,XMM1,0xd2
    let x7 = x1; // 00daead0: MOVAPS XMM7,XMM1
    let x1 = x0; // 00daead3: MOVAPS XMM1,XMM0
    let x1 = mulps(x1, x7); // 00daead6: MULPS XMM1,XMM7
    let v_c0 = x3; // 00daead9: MOVAPS xmmword ptr [ESP + 0xc0],XMM3
    let v_20 = x6; // 00daeae1: MOVAPS xmmword ptr [ESP + 0x20],XMM6
    let x6 = mulps(x6, x3); // 00daeae6: MULPS XMM6,XMM3
    let x1 = subps(x1, x6); // 00daeae9: SUBPS XMM1,XMM6
    let x0 = x1; // 00daeaec: MOVAPS XMM0,XMM1
    let x0 = mulps(x0, x1); // 00daeaef: MULPS XMM0,XMM1
    let x3 = x0; // 00daeaf2: MOVAPS XMM3,XMM0
    let x3 = shufps(x3, x0, 0x55); // 00daeaf5: SHUFPS XMM3,XMM0,0x55
    let x3 = addss(x3, x0); // 00daeaf9: ADDSS XMM3,XMM0
    let x0 = shufps(x0, x0, 0xaa); // 00daeafd: SHUFPS XMM0,XMM0,0xaa
    let x0 = addss(x0, x3); // 00daeb01: ADDSS XMM0,XMM3
    let x3 = x0; // 00daeb05: MOVAPS XMM3,XMM0
    let x3 = rsqrtss(x3, x3); // 00daeb08: RSQRTSS XMM3,XMM3
    let x6 = x3; // 00daeb0c: MOVAPS XMM6,XMM3
    let x3 = scalar_vec(e, 0x01011d78); // 00daeb0f: MOVSS XMM3,dword ptr [0x01011d78]
    let v_50 = x7; // 00daeb17: MOVAPS xmmword ptr [ESP + 0x50],XMM7
    let x7 = x3; // 00daeb1c: MOVAPS XMM7,XMM3
    let x3 = x0; // 00daeb1f: MOVAPS XMM3,XMM0
    let x0 = mulss(x0, x6); // 00daeb22: MULSS XMM0,XMM6
    let x0 = mulss(x0, x6); // 00daeb26: MULSS XMM0,XMM6
    let v_30 = x6; // 00daeb2a: MOVAPS xmmword ptr [ESP + 0x30],XMM6
    let x6 = scalar_vec(e, 0x01017718); // 00daeb2f: MOVSS XMM6,dword ptr [0x01017718]
    let x6 = subss(x6, x0); // 00daeb37: SUBSS XMM6,XMM0
    let x0 = scalar_vec(e, 0x01016248); // 00daeb3b: MOVSS XMM0,dword ptr [0x01016248]
    let x0 = mulss(x0, [v_30[0], 0.0, 0.0, 0.0]); // 00daeb43: MULSS XMM0,dword ptr [ESP + 0x30]
    let x3 = cmpeqss(x3, x7); // 00daeb49: CMPEQSS XMM3,XMM7
    let x0 = mulss(x0, x6); // 00daeb4e: MULSS XMM0,XMM6
    let x3 = andnps(x3, x0); // 00daeb52: ANDNPS XMM3,XMM0
    let x6 = x5; // 00daeb55: MOVAPS XMM6,XMM5
    let x6 = subps(x6, v_b0); // 00daeb58: SUBPS XMM6,xmmword ptr [ESP + 0xb0]
    let x0 = x5; // 00daeb60: MOVAPS XMM0,XMM5
    let x0 = subps(x0, v_a0); // 00daeb63: SUBPS XMM0,xmmword ptr [ESP + 0xa0]
    let v_f0 = x5; // 00daeb6b: MOVAPS xmmword ptr [ESP + 0xf0],XMM5
    let x2 = subps(x2, x5); // 00daeb73: SUBPS XMM2,XMM5
    let x3 = shufps(x3, x3, 0x0); // 00daeb78: SHUFPS XMM3,XMM3,0x0
    let x3 = mulps(x3, x1); // 00daeb7c: MULPS XMM3,XMM1
    let x1 = x5; // 00daeb7f: MOVAPS XMM1,XMM5
    let x1 = subps(x1, v_60); // 00daeb82: SUBPS XMM1,xmmword ptr [ESP + 0x60]
    let x5 = x1; // 00daeb87: MOVAPS XMM5,XMM1
    let x5 = shufps(x5, x1, 0xd2); // 00daeb8a: SHUFPS XMM5,XMM1,0xd2
    let x5 = mulps(x5, v_40); // 00daeb8e: MULPS XMM5,xmmword ptr [ESP + 0x40]
    let x1 = shufps(x1, x1, 0xc9); // 00daeb93: SHUFPS XMM1,XMM1,0xc9
    let x1 = mulps(x1, v_20); // 00daeb97: MULPS XMM1,xmmword ptr [ESP + 0x20]
    let x1 = subps(x1, x5); // 00daeb9c: SUBPS XMM1,XMM5
    let x5 = x6; // 00daeb9f: MOVAPS XMM5,XMM6
    let x5 = shufps(x5, x6, 0xc9); // 00daeba2: SHUFPS XMM5,XMM6,0xc9
    let x5 = mulps(x5, v_50); // 00daeba6: MULPS XMM5,xmmword ptr [ESP + 0x50]
    let x7 = x6; // 00daebab: MOVAPS XMM7,XMM6
    let x1 = mulps(x1, x3); // 00daebae: MULPS XMM1,XMM3
    let x7 = shufps(x7, x6, 0xd2); // 00daebb1: SHUFPS XMM7,XMM6,0xd2
    let x7 = mulps(x7, v_c0); // 00daebb5: MULPS XMM7,xmmword ptr [ESP + 0xc0]
    let x6 = x4; // 00daebbd: MOVAPS XMM6,XMM4
    let x6 = shufps(x6, x4, 0xc9); // 00daebc0: SHUFPS XMM6,XMM4,0xc9
    let x5 = subps(x5, x7); // 00daebc4: SUBPS XMM5,XMM7
    let x7 = x0; // 00daebc7: MOVAPS XMM7,XMM0
    let x7 = shufps(x7, x0, 0xd2); // 00daebca: SHUFPS XMM7,XMM0,0xd2
    let x0 = shufps(x0, x0, 0xc9); // 00daebce: SHUFPS XMM0,XMM0,0xc9
    let x6 = mulps(x6, x7); // 00daebd2: MULPS XMM6,XMM7
    let x4 = shufps(x4, x4, 0xd2); // 00daebd5: SHUFPS XMM4,XMM4,0xd2
    let x4 = mulps(x4, x0); // 00daebd9: MULPS XMM4,XMM0
    let x0 = load_vec(e, esi.wrapping_add(0x100)); // 00daebdc: MOVAPS XMM0,xmmword ptr [ESI + 0x100]
    let x4 = subps(x4, x6); // 00daebe3: SUBPS XMM4,XMM6
    let x0 = mulps(x0, x3); // 00daebe6: MULPS XMM0,XMM3
    let v_c0 = x0; // 00daebe9: MOVAPS xmmword ptr [ESP + 0xc0],XMM0
    let x5 = mulps(x5, x3); // 00daebf1: MULPS XMM5,XMM3
    let x6 = x1; // 00daebf4: MOVAPS XMM6,XMM1
    let x6 = shufps(x6, x5, 0x44); // 00daebf7: SHUFPS XMM6,XMM5,0x44
    let v_30 = x6; // 00daebfb: MOVAPS xmmword ptr [ESP + 0x30],XMM6
    let x4 = mulps(x4, x3); // 00daec00: MULPS XMM4,XMM3
    let x6 = x4; // 00daec03: MOVAPS XMM6,XMM4
    let x4 = shufps(x4, v_c0, 0xee); // 00daec06: SHUFPS XMM4,xmmword ptr [ESP + 0xc0],0xee
    let x6 = shufps(x6, x0, 0x44); // 00daec0f: SHUFPS XMM6,XMM0,0x44
    let v_60 = x6; // 00daec13: MOVAPS xmmword ptr [ESP + 0x60],XMM6
    let x0 = x3; // 00daec18: MOVAPS XMM0,XMM3
    let x0 = mulps(x0, x2); // 00daec1b: MULPS XMM0,XMM2
    let x6 = x0; // 00daec1e: MOVAPS XMM6,XMM0
    let x6 = shufps(x6, x0, 0x55); // 00daec21: SHUFPS XMM6,XMM0,0x55
    let x6 = addss(x6, x0); // 00daec25: ADDSS XMM6,XMM0
    let x0 = shufps(x0, x0, 0xaa); // 00daec29: SHUFPS XMM0,XMM0,0xaa
    let x0 = addss(x0, x6); // 00daec2d: ADDSS XMM0,XMM6
    let x0 = shufps(x0, x0, 0x0); // 00daec31: SHUFPS XMM0,XMM0,0x0
    let x0 = mulps(x0, x3); // 00daec35: MULPS XMM0,XMM3
    let x0 = addps(x0, v_f0); // 00daec38: ADDPS XMM0,xmmword ptr [ESP + 0xf0]
    let x1 = shufps(x1, x5, 0xee); // 00daec40: SHUFPS XMM1,XMM5,0xee
    let (x7, eax) = quad_tail(e, x1, x3, x4, v_30, v_60);
    Closest { x0, x2, x7, eax }
}

// Translated from 00dae3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(record, points, start, cell, object)`: goes through the manifold record's entries
/// from number `start` on (see `00daf1e0` for the record's layout: group counts `a` and `b` at
/// `+0` and `+1`, the entry count at `+2`, 8-byte entries from `+4`, then the table of vertex
/// words) and keeps those whose vertices still matter. The case of an entry is picked by `b + 4
/// * a` through the exe's byte table `00daedf0` and jump table `00daedd0` (only `(1, 1)`,
/// `(1, 2)`, `(1, 3)`, `(2, 1)`, `(2, 2)` and `(3, 1)` have code: an entry with another count
/// pair goes). The case works on the vertices at `points + [entry + 4..=7]` (16-byte vectors) and
/// leaves `XMM0`, `XMM2`, `XMM7` and `EAX`; a non-zero `EAX` drops the entry. Otherwise, with `s
/// = XMM7 . XMM2` (`z + (y + x)`), `A`, `B` and `C` the floats at `points + 0x114`, `+0x110` and
/// `+0x118`, and `t = s - (B + A)` (each step rounded to a float): the entry stays only when `C
/// > t`; the 0x30-byte output record at `*cell` gets `(A - s) * XMM7 + XMM0` at `+0`, `XMM7` at
/// `+0x10` (its lane 3 replaced by `t`) and the entry's id word at `+0x20`, and `*cell` moves on
/// by 0x30. A dropped entry (its id, unless 0xffff, and `cell[1]` go to the object's virtual slot
/// `+0x14`) is replaced by the last entry. When entries were dropped the entry count is
/// stored, the vertex words move down to follow the entries and `00dae220` runs on the record.
/// The exe's name for it is unknown (the manifold's point list, sibling of `00daf1e0`).
pub fn fn_00dae3a0(e: &mut Engine, record: u32, points: u32, start: u32, cell: u32, object: u32) {
    let original = u32::from(e.mem.u8(record + 2));
    let mut count = original;
    let mut index = start;
    if index >= original {
        return;
    }
    let table_start = record + 4 + count * 8;
    let mut table = table_start;
    let mut entry = record + 4 + index * 8;
    loop {
        let out = e.mem.u32(cell);
        let first = u32::from(e.mem.u8(entry));
        let second = u32::from(e.mem.u8(entry + 1));
        let slot = second + 4 * first;
        let case = if slot > 0xd {
            0
        } else {
            [0u8, 7, 7, 7, 7, 1, 2, 3, 7, 4, 5, 7, 7, 6][slot as usize]
        };
        let closest = match case {
            1 => case_one_one(e, entry, points),
            2 => case_one_two(e, entry, points),
            3 => case_one_three(e, entry, points),
            4 => case_two_one(e, entry, points),
            5 => case_two_two(e, entry, points),
            6 => case_three_one(e, entry, points),
            _ => Closest::rejected(),
        };
        let mut kept = false;
        if closest.eax == 0 {
            let depth = x87_load32(e.mem.u32(points + 0x114));
            let product = mulps(closest.x7, closest.x2);
            let sum = addss(shufps(product, product, 0x55), product);
            let total = addss(shufps(product, product, 0xaa), sum);
            let s = x87_load32(total[0].to_bits());
            let gap = x87_store32(x87_sub(depth, s));
            let base = x87_load32(e.mem.u32(points + 0x110));
            let gap_vec = [f32::from_bits(gap), 0.0, 0.0, 0.0];
            let both = x87_store32(x87_add(base, x87_m32(e.mem.u32(points + 0x114))));
            let rest = x87_load32(x87_store32(x87_sub(s, x87_m32(both))));
            let limit = x87_load32(e.mem.u32(points + 0x118));
            if st0_above(limit, rest) {
                let x2 = mulps(shufps(gap_vec, gap_vec, 0), closest.x7);
                store_vec(e, out + 0x10, closest.x7);
                let x2 = addps(x2, closest.x0);
                store_vec(e, out, x2);
                e.mem.set_u32(out + 0x1c, x87_store32(rest));
                let id = u32::from(e.mem.u16(entry + 2));
                e.mem.set_u32(out + 0x20, id);
                let moved = e.mem.u32(cell);
                e.mem.set_u32(cell, moved.wrapping_add(0x30));
                kept = true;
            }
        }
        if !kept {
            let id = u32::from(e.mem.u16(entry + 2));
            if id != 0xffff {
                let extra = e.mem.u32(cell + 4);
                e.vcall(object, 0x14, &args![id, extra]);
            }
            let last = e.mem.u32(table - 8);
            index = index.wrapping_sub(1);
            table -= 8;
            e.mem.set_u32(entry, last);
            let last_second = e.mem.u32(table + 4);
            count -= 1;
            e.mem.set_u32(entry + 4, last_second);
            // `SUB EDI,0x8` at 00daed70: the entry pointer steps back with the
            // index, so the entry moved in from the end is looked at next.
            entry = entry.wrapping_sub(8);
        }
        index = index.wrapping_add(1);
        entry += 8;
        if index >= count {
            break;
        }
    }
    if original > count {
        e.mem.set_u8(record + 2, count as u8);
        let mut destination = record + count * 8 + 4;
        let mut remaining =
            (i32::from(e.mem.u8(record + 1)) + i32::from(e.mem.u8(record)) - 1) >> 1;
        if remaining >= 0 {
            let distance = table_start - destination;
            loop {
                let moved = e.mem.u32(destination + distance);
                e.mem.set_u32(destination, moved);
                destination += 4;
                remaining -= 1;
                if remaining < 0 {
                    break;
                }
            }
        }
        e.call(MANIFOLD_REMOVE_POINT, &args![record]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_ee00, fn_00daee00(u32) -> u32),
        entry!(
            0x00da_f1e0,
            fn_00daf1e0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00da_e3a0, fn_00dae3a0(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e.mem.set_u32(START_MINIMUM, 0x7f7f_ffee);
        e.mem.set_u64(GROWTH, 0x3ff0_cccc_c000_0000);
        e.mem.set_u64(SQUARE_LIMIT, 526.0f64.to_bits());
        e.mem.set_u32(START_RATIO, 0x3cf9_8ae9);
        e.mem.set_u64(EPSILON, 0x3e11_2e0b_e000_0000);
        e.mem.set_u64(RATIO, 0x3f9f_315d_2000_0000);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// Five point records: position at `+0` and depth at `+0x1c`.
    fn make_points(e: &mut Engine, positions: [[f32; 3]; 5], depths: [f32; 5]) -> u32 {
        let points = e.mem.alloc(0x120);
        for (k, (position, depth)) in positions.iter().zip(depths).enumerate() {
            let at = points + 0x30 * k as u32;
            store_vec(e, at, [position[0], position[1], position[2], 0.0]);
            e.mem.set_f32(at + 0x1c, depth);
        }
        points
    }

    /// The choice of `00daee00` in plain `f64` arithmetic, following its description.
    fn expected_drop(positions: [[f32; 3]; 5], depths: [f32; 5]) -> u32 {
        let pos: Vec<[f64; 3]> = positions.iter().map(|p| p.map(f64::from)).collect();
        let squared =
            |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| (a[i] - b[i]) * (a[i] - b[i])).sum::<f64>();
        let dist: Vec<f64> = pos.iter().take(4).map(|p| squared(pos[4], *p)).collect();
        let (mut farthest, mut nearest) = (0usize, 0usize);
        let (mut max, mut min) = (0.0f64, f64::from(f32::MAX));
        for (k, d) in dist.iter().enumerate() {
            if max < *d {
                max = *d;
                farthest = k;
            }
            if min > *d {
                min = *d;
                nearest = k;
            }
        }
        let gap = f64::from(depths[4]) - f64::from(depths[nearest]);
        let mut flags = [false; 5];
        flags[farthest] = true;
        max *= f64::from(1.05f32);
        let mut second = 4;
        for (k, p) in pos.iter().enumerate() {
            if !flags[k] && max < squared(pos[farthest], *p) {
                max = squared(pos[farthest], *p);
                second = k;
            }
        }
        flags[second] = true;
        let edge: [f64; 3] = std::array::from_fn(|i| pos[farthest][i] - pos[second][i]);
        let (mut third, mut max) = (0usize, 0.0f64);
        for (k, p) in pos.iter().enumerate() {
            if flags[k] {
                continue;
            }
            let o: [f64; 3] = std::array::from_fn(|i| p[i] - pos[farthest][i]);
            let c = [
                o[1] * edge[2] - o[2] * edge[1],
                o[2] * edge[0] - o[0] * edge[2],
                o[0] * edge[1] - o[1] * edge[0],
            ];
            let length = c[0] * c[0] + c[1] * c[1] + c[2] * c[2];
            if max < length {
                max = length;
                third = k;
            }
        }
        flags[third] = true;
        let (mut fourth, mut max) = (0usize, 0.0f64);
        for (k, p) in pos.iter().enumerate() {
            if !flags[k] && max < squared(pos[third], *p) {
                max = squared(pos[third], *p);
                fourth = k;
            }
        }
        flags[fourth] = true;
        let first_free = flags.iter().position(|f| !f).unwrap_or(0) as u32;
        if flags[4] || gap >= 0.0 || min >= gap * (gap * 526.0) {
            return first_free;
        }
        let epsilon = f64::from(1e-9f32);
        let mut answer = 4u32;
        let mut ratio = f64::from(f32::from_bits(0x3cf9_8ae9));
        for (k, d) in dist.iter().enumerate() {
            let t = (f64::from(depths[k]) - f64::from(depths[4])).powi(2) + epsilon;
            if ratio * *d < t {
                answer = k as u32;
                if k != 3 {
                    ratio = t / (*d + epsilon);
                }
            }
        }
        answer
    }

    fn drop_index(positions: [[f32; 3]; 5], depths: [f32; 5]) -> u32 {
        let mut e = engine();
        let points = make_points(&mut e, positions, depths);
        e.call(0x00da_ee00, &args![points]).u32()
    }

    const SPREAD: [[f32; 3]; 5] = [
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [0.0, 12.0, 0.0],
        [0.0, 0.0, 14.0],
        [0.1, 0.1, 0.1],
    ];

    const FAR: [[f32; 3]; 5] = [
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [0.0, 10.0, 0.0],
        [0.0, 0.0, 10.0],
        [20.0, 20.0, 20.0],
    ];

    #[test]
    fn a_new_point_outside_pushes_out_the_unflagged_old_one() {
        // The farthest is 0, the new point is flagged second, then 1 and 2: point 3 goes.
        assert_eq!(drop_index(FAR, [0.0; 5]), 3);
        assert_eq!(expected_drop(FAR, [0.0; 5]), 3);
    }

    #[test]
    fn a_redundant_new_point_lets_the_depths_choose() {
        // Point 0 sits next to the new point: the new one is not flagged and the depth
        // differences decide which old point is replaced.
        let cases: [([f32; 5], u32); 5] = [
            ([1.0, 0.6, 0.6, 0.6, 0.5], 0),
            ([1.0, 100.0, 0.6, 0.6, 0.5], 1),
            ([1.0, 0.6, 0.6, 50.0, 0.5], 3),
            // The depth gap is small: nothing is preferred and the new point itself goes.
            ([0.52, 0.5, 0.5, 0.5, 0.5], 4),
            // A new depth above the nearest one's: the first unflagged point (the new one).
            ([0.5, 0.5, 0.5, 0.5, 1.0], 4),
        ];
        for (depths, want) in cases {
            assert_eq!(drop_index(SPREAD, depths), want);
            assert_eq!(expected_drop(SPREAD, depths), want);
        }
    }

    #[test]
    fn other_layouts_agree_with_the_plain_description() {
        let layouts: [[[f32; 3]; 5]; 4] = [
            [
                [1.0, 2.0, 3.0],
                [4.0, -2.0, 1.0],
                [-3.0, 5.0, 2.0],
                [2.0, 2.0, -6.0],
                [1.5, 2.5, 2.0],
            ],
            [
                [0.0, 0.0, 0.0],
                [3.0, 0.0, 0.0],
                [0.0, 4.0, 0.0],
                [0.0, 0.0, 5.0],
                [0.2, 0.1, 0.2],
            ],
            [
                [0.0, 0.0, 0.0],
                [3.0, 0.0, 0.0],
                [0.0, 4.0, 0.0],
                [0.0, 0.0, 5.0],
                [3.1, 0.2, 0.1],
            ],
            [
                [5.0, 5.0, 5.0],
                [-5.0, 5.0, 5.0],
                [5.0, -5.0, 5.0],
                [5.0, 5.0, -5.0],
                [-4.0, -4.0, -4.0],
            ],
        ];
        for positions in layouts {
            for depths in [
                [0.0f32; 5],
                [1.0, 2.0, 3.0, 4.0, 0.0],
                [0.5, 0.6, 0.7, 0.8, 0.4],
                [9.0, 1.0, 1.0, 1.0, 0.9],
            ] {
                assert_eq!(
                    drop_index(positions, depths),
                    expected_drop(positions, depths),
                    "{positions:?} {depths:?}"
                );
            }
        }
    }

    const OBJECT_SLOT_C: u32 = 0x0900_0001;
    const OBJECT_SLOT_14: u32 = 0x0900_0002;

    /// A callback object whose slot `+0xc` answers `answer` and slot `+0x14` answers 0.
    fn make_object(e: &mut Engine, answer: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, OBJECT_SLOT_C);
        e.mem.set_u32(vtable + 0x14, OBJECT_SLOT_14);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.register_double(OBJECT_SLOT_C, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        e.register_double(OBJECT_SLOT_14, |_, _| Ret::default());
        object
    }

    fn make_record(e: &mut Engine, counts: [u8; 2], entries: &[[u8; 8]], words: &[u16]) -> u32 {
        let record = e.mem.alloc(0x80);
        e.mem.set_u8(record, counts[0]);
        e.mem.set_u8(record + 1, counts[1]);
        e.mem.set_u8(record + 2, entries.len() as u8);
        for (i, entry) in entries.iter().enumerate() {
            for (k, byte) in entry.iter().enumerate() {
                e.mem.set_u8(record + 4 + 8 * i as u32 + k as u32, *byte);
            }
        }
        let table = record + 4 + 8 * entries.len() as u32;
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u16(table + 2 * i as u32, *word);
        }
        record
    }

    fn make_info(e: &mut Engine, counts: [u8; 2], words: &[u16]) -> u32 {
        let info = e.mem.alloc(0x20);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u16(info + 2 * i as u32, *word);
        }
        e.mem.set_u8(info + 8, counts[0]);
        e.mem.set_u8(info + 9, counts[1]);
        info
    }

    fn entry_of(e: &Engine, record: u32, index: u32) -> Vec<u8> {
        (0..8)
            .map(|k| e.mem.u8(record + 4 + 8 * index + k))
            .collect()
    }

    fn table_of(e: &Engine, record: u32, count: u32) -> Vec<u16> {
        let table = record + 4 + 8 * u32::from(e.mem.u8(record + 2));
        (0..count).map(|i| e.mem.u16(table + 2 * i)).collect()
    }

    /// Calls `00daf1e0` with the record, info, object and flag; the other words are 1, 2, 3, `a4`
    /// (a block with 0xa4a4 at `+4`) and `out`. Returns the answer, `a4` and `out`.
    fn add(
        e: &mut Engine,
        (record, info, object): (u32, u32, u32),
        points: u32,
        flag: u32,
    ) -> (u32, u32, u32) {
        let a4 = e.mem.alloc(8);
        e.mem.set_u32(a4 + 4, 0xa4a4);
        let out = e.mem.alloc(0x30);
        let result = e
            .call(
                0x00da_f1e0,
                &args![record, 1u32, 2u32, 3u32, a4, info, out, points, object, flag],
            )
            .u32();
        (result, a4, out)
    }

    #[test]
    fn the_first_entry_is_written_with_its_vertices_in_the_tables() {
        let mut e = engine();
        let object = make_object(&mut e, 0x1234);
        let record = make_record(&mut e, [0, 0], &[], &[]);
        let info = make_info(&mut e, [2, 1], &[0x100, 0x200, 0x300]);
        let (result, a4, out) = add(&mut e, (record, info, object), 0, 0);
        assert_eq!(result, 4);
        assert_eq!(
            calls(&e, OBJECT_SLOT_C),
            vec![vec![object, 1, 2, 3, a4, info, out]]
        );
        assert_eq!(e.mem.u32(out + 0x20), 0x1234);
        assert_eq!(e.mem.u8(record), 2);
        assert_eq!(e.mem.u8(record + 1), 1);
        assert_eq!(e.mem.u8(record + 2), 1);
        assert_eq!(
            entry_of(&e, record, 0),
            [2, 1, 0x34, 0x12, 0x00, 0x10, 0x20, 0x00]
        );
        assert_eq!(table_of(&e, record, 3), [0x100, 0x200, 0x300]);
    }

    #[test]
    fn a_second_entry_shares_vertices_and_shifts_the_older_one() {
        let mut e = engine();
        let object = make_object(&mut e, 0x1234);
        let record = make_record(&mut e, [0, 0], &[], &[]);
        let info = make_info(&mut e, [2, 1], &[0x100, 0x200, 0x300]);
        add(&mut e, (record, info, object), 0, 0);
        let object = make_object(&mut e, 0x4321);
        // Vertex 0x200 is known, 0x500 is a new group A vertex, 0x300 is known (group B).
        let info = make_info(&mut e, [2, 1], &[0x200, 0x500, 0x300]);
        let (result, _, _) = add(&mut e, (record, info, object), 0, 0);
        assert_eq!(result, 4);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 1), 1);
        assert_eq!(e.mem.u8(record + 2), 2);
        assert_eq!(
            entry_of(&e, record, 0),
            [2, 1, 0x21, 0x43, 0x10, 0x20, 0x30, 0x00]
        );
        // The older entry moved to the end; its group B index moved up by one vertex.
        assert_eq!(
            entry_of(&e, record, 1),
            [2, 1, 0x34, 0x12, 0x00, 0x10, 0x30, 0x00]
        );
        assert_eq!(table_of(&e, record, 4), [0x100, 0x200, 0x500, 0x300]);
    }

    #[test]
    fn a_new_group_b_vertex_is_appended() {
        let mut e = engine();
        let object = make_object(&mut e, 0x1234);
        let record = make_record(&mut e, [0, 0], &[], &[]);
        let info = make_info(&mut e, [1, 1], &[0x100, 0x300]);
        add(&mut e, (record, info, object), 0, 0);
        let info = make_info(&mut e, [1, 1], &[0x100, 0x400]);
        add(&mut e, (record, info, object), 0, 0);
        assert_eq!(e.mem.u8(record), 1);
        assert_eq!(e.mem.u8(record + 1), 2);
        assert_eq!(table_of(&e, record, 3), [0x100, 0x300, 0x400]);
        assert_eq!(entry_of(&e, record, 0)[4..6], [0x00, 0x20]);
        assert_eq!(entry_of(&e, record, 1)[4..6], [0x00, 0x10]);
    }

    #[test]
    fn with_the_flag_set_the_id_is_none_without_asking() {
        let mut e = engine();
        let object = make_object(&mut e, 0x1234);
        let record = make_record(&mut e, [0, 0], &[], &[]);
        let info = make_info(&mut e, [1, 1], &[0x100, 0x200]);
        let (result, _, out) = add(&mut e, (record, info, object), 0, 1);
        assert_eq!(result, 4);
        assert!(calls(&e, OBJECT_SLOT_C).is_empty());
        assert_eq!(e.mem.u32(out + 0x20), 0xffff);
        assert_eq!(e.mem.u16(record + 6), 0xffff);
        // A group B count of 3 asks anyway.
        let mut e = engine();
        let object = make_object(&mut e, 0x77);
        let record = make_record(&mut e, [0, 0], &[], &[]);
        let info = make_info(&mut e, [0, 3], &[0x100, 0x200, 0x300]);
        let (result, _, out) = add(&mut e, (record, info, object), 0, 1);
        assert_eq!(result, 4);
        assert_eq!(calls(&e, OBJECT_SLOT_C).len(), 1);
        assert_eq!(e.mem.u32(out + 0x20), 0x77);
    }

    #[test]
    fn an_answer_of_none_rejects_the_entry() {
        let mut e = engine();
        let object = make_object(&mut e, 0xffff);
        let record = make_record(&mut e, [1, 0], &[[1, 0, 9, 0, 0, 0, 0, 0]], &[0x100]);
        let info = make_info(&mut e, [1, 0], &[0x200]);
        let before = entry_of(&e, record, 0);
        let (result, _, _) = add(&mut e, (record, info, object), 0, 0);
        assert_eq!(result, 5);
        assert_eq!(entry_of(&e, record, 0), before);
        assert_eq!(e.mem.u8(record + 2), 1);
    }

    /// A full record (four entries) with its words: group A `0x100, 0x200`, group B `0x300`.
    fn full_record(e: &mut Engine) -> u32 {
        make_record(
            e,
            [2, 1],
            &[
                [2, 1, 0x10, 0x01, 0x00, 0x10, 0x20, 0],
                [2, 1, 0x20, 0x02, 0x10, 0x00, 0x20, 0],
                [2, 1, 0x30, 0x03, 0x00, 0x10, 0x20, 0],
                [2, 1, 0x40, 0x04, 0x10, 0x00, 0x20, 0],
            ],
            &[0x100, 0x200, 0x300],
        )
    }

    #[test]
    fn a_full_record_whose_choice_is_the_new_point_rejects_the_entry() {
        let mut e = engine();
        let object = make_object(&mut e, 0x55);
        let record = full_record(&mut e);
        let points = make_points(&mut e, SPREAD, [0.52, 0.5, 0.5, 0.5, 0.5]);
        let info = make_info(&mut e, [1, 1], &[0x100, 0x300]);
        let (result, _, _) = add(&mut e, (record, info, object), points, 0);
        assert_eq!(result, 5);
        assert!(calls(&e, OBJECT_SLOT_C).is_empty());
        assert!(calls(&e, OBJECT_SLOT_14).is_empty());
    }

    #[test]
    fn a_full_record_replaces_the_chosen_point_with_the_new_one() {
        let mut e = engine();
        let object = make_object(&mut e, 0x55);
        e.register(MANIFOLD_REMOVE_POINT, |_, _| Ret::default());
        let record = full_record(&mut e);
        let points = make_points(&mut e, FAR, [0.0; 5]);
        let info = make_info(&mut e, [1, 1], &[0x200, 0x300]);
        let (result, _, out) = add(&mut e, (record, info, object), points, 0);
        // Point 3 goes: the id of its entry and a4[1] go to slot 0x14, the remover runs, and
        // the callback's id 0x55 is stored with the output at the point's slot.
        assert_eq!(result, 3);
        assert_eq!(
            calls(&e, OBJECT_SLOT_14),
            vec![vec![object, 0x0440, 0xa4a4]]
        );
        assert_eq!(calls(&e, MANIFOLD_REMOVE_POINT), vec![vec![record]]);
        assert_eq!(e.mem.u32(out + 0x20), 0x55);
        assert_eq!(e.mem.u32(points + 3 * 0x30 + 0x20), 0x55);
        // The new entry is written at the front.
        assert_eq!(e.mem.u16(record + 6), 0x55);
        assert_eq!(e.mem.u8(record + 2), 4);
    }

    #[test]
    fn a_full_record_whose_new_point_cannot_be_placed_loses_its_last_entry() {
        let mut e = engine();
        let object = make_object(&mut e, 0xffff);
        e.register(MANIFOLD_REMOVE_POINT, |_, _| Ret::default());
        let record = full_record(&mut e);
        let points = make_points(&mut e, FAR, [0.0; 5]);
        // The vectors at points + 0x90 and + 0xa0 and the word at + 0xb0 are copied in.
        store_vec(&mut e, points + 0x90, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, points + 0xa0, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(points + 0xb0, 0xb0b0);
        let old_first = entry_of(&e, record, 0);
        let info = make_info(&mut e, [1, 1], &[0x200, 0x300]);
        let (result, _, _) = add(&mut e, (record, info, object), points, 0);
        assert_eq!(result, 6);
        assert_eq!(e.mem.u8(record + 2), 3);
        // Entry 3 got the first entry's data words before the remover, and the first entry
        // then took the last entry's, which are the old first entry's: no change.
        assert_eq!(entry_of(&e, record, 0), old_first);
        assert_eq!(load_vec(&e, points + 3 * 0x30), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, points + 3 * 0x30 + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(points + 3 * 0x30 + 0x20), 0xb0b0);
        // The vertex words moved down by one entry (8 bytes).
        assert_eq!(table_of(&e, record, 3), [0x100, 0x200, 0x300]);
    }
}

#[cfg(test)]
mod a09_tests_b {
    use super::*;

    const ONES: u32 = 0x010e_c7f0;
    const OBJECT_SLOT_14: u32 = 0x0900_0014;
    type Vec3 = [f32; 3];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e.mem.set_u32(0x0101_1d78, 0);
        e.mem.set_u32(0x0101_7718, 3.0f32.to_bits());
        e.mem.set_u32(0x0101_6248, 0.5f32.to_bits());
        for lane in 0..4 {
            e.mem.set_u32(SIGN_BITS + 4 * lane, 0x8000_0000);
            e.mem.set_u32(ONES + 4 * lane, 1.0f32.to_bits());
        }
        e.register(MANIFOLD_REMOVE_POINT, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn cross(a: Vec3, b: Vec3) -> Vec3 {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn dot(a: Vec3, b: Vec3) -> f32 {
        a[2] * b[2] + (a[1] * b[1] + a[0] * b[0])
    }

    fn sub(a: Vec3, b: Vec3) -> Vec3 {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    fn xyz(v: Vec4) -> Vec3 {
        [v[0], v[1], v[2]]
    }

    fn unit(v: Vec3) -> Vec3 {
        let length = dot(v, v).sqrt();
        [v[0] / length, v[1] / length, v[2] / length]
    }

    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= 1e-4 * (1.0 + expected.abs()),
            "{actual} is not near {expected}"
        );
    }

    fn near_vec(actual: Vec4, expected: Vec3) {
        for lane in 0..3 {
            near(actual[lane], expected[lane]);
        }
    }

    /// The vertex offsets of the vertices: vertex `k` is at `points + OFFSETS[k]`.
    const OFFSETS: [u32; 6] = [0x10, 0x20, 0x30, 0x40, 0x50, 0x60];

    /// A points block: the vertices at `OFFSETS`, the vector `extra` at `+0x100`, and the floats
    /// at `+0x110` (B), `+0x114` (A), `+0x118` (C) and `+0x11c` (M).
    fn make_points(e: &mut Engine, vertices: &[Vec4], extra: Vec4, floats: [f32; 4]) -> u32 {
        let points = e.mem.alloc(0x130);
        for (offset, vertex) in OFFSETS.iter().zip(vertices) {
            store_vec(e, points + offset, *vertex);
        }
        store_vec(e, points + 0x100, extra);
        for (i, value) in floats.iter().enumerate() {
            e.mem.set_f32(points + 0x110 + 4 * i as u32, *value);
        }
        points
    }

    /// An entry: counts, id and the four vertex byte offsets (as indexes into `OFFSETS`).
    type Entry = (u8, u8, u16, [usize; 4]);

    fn make_record(e: &mut Engine, counts: [u8; 2], entries: &[Entry], words: &[u16]) -> u32 {
        let record = e.mem.alloc(0x100);
        e.mem.set_u8(record, counts[0]);
        e.mem.set_u8(record + 1, counts[1]);
        e.mem.set_u8(record + 2, entries.len() as u8);
        for (i, (a, b, id, vertices)) in entries.iter().enumerate() {
            let at = record + 4 + 8 * i as u32;
            e.mem.set_u8(at, *a);
            e.mem.set_u8(at + 1, *b);
            e.mem.set_u16(at + 2, *id);
            for (k, vertex) in vertices.iter().enumerate() {
                e.mem.set_u8(at + 4 + k as u32, OFFSETS[*vertex] as u8);
            }
        }
        let table = record + 4 + 8 * entries.len() as u32;
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u16(table + 2 * i as u32, *word);
        }
        record
    }

    /// An object whose slot `+0x14` is a double, and a cell whose first word points at a block
    /// for `records` output records and whose second word is `0xe7a`. Returns (object, cell, out).
    fn make_object_and_cell(e: &mut Engine, records: u32) -> (u32, u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, OBJECT_SLOT_14);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.register(OBJECT_SLOT_14, |_, _| Ret::default());
        let out = e.mem.alloc(0x30 * records);
        let cell = e.mem.alloc(8);
        e.mem.set_u32(cell, out);
        e.mem.set_u32(cell + 4, 0xe7a);
        (object, cell, out)
    }

    fn entry_bytes(e: &Engine, record: u32, index: u32) -> Vec<u8> {
        (0..8)
            .map(|k| e.mem.u8(record + 4 + 8 * index + k))
            .collect()
    }

    fn vertex(e: &Engine, points: u32, k: usize) -> Vec4 {
        load_vec(e, points + OFFSETS[k])
    }

    const SEGMENT: [Vec4; 2] = [[1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]];

    #[test]
    fn a_segment_entry_that_passes_writes_its_output_record() {
        let mut e = engine();
        let points = make_points(
            &mut e,
            &SEGMENT,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 5.0],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[(1, 1, 0x11, [0, 1, 0, 0])],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 2);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        // s = 0.5, A - s = 1.5, t = 0.5 - (0.25 + 2.0) = -1.75: the output is 1.5 * V + P0.
        assert_eq!(load_vec(&e, out), [1.75, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.5, 0.0, 0.0, -1.75]);
        assert_eq!(e.mem.u32(out + 0x20), 0x11);
        assert_eq!(e.mem.u32(cell), out + 0x30);
        assert!(calls(&e, OBJECT_SLOT_14).is_empty());
        assert_eq!(e.mem.u8(record + 2), 1);
        assert!(calls(&e, MANIFOLD_REMOVE_POINT).is_empty());
    }

    #[test]
    fn an_entry_beyond_the_limit_or_not_below_the_depth_bound_goes() {
        // The squared length 1 is above the float at +0x11c (0.5).
        let mut e = engine();
        let points = make_points(
            &mut e,
            &SEGMENT,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 0.5],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[(1, 1, 0x11, [0, 1, 0, 0])],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 1);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        assert_eq!(e.mem.u32(cell), out);
        assert_eq!(calls(&e, OBJECT_SLOT_14), vec![vec![object, 0x11, 0xe7a]]);
        assert_eq!(e.mem.u8(record + 2), 0);
        assert_eq!(calls(&e, MANIFOLD_REMOVE_POINT), vec![vec![record]]);
        // The bound C = -2 is not above t = -1.75.
        let mut e = engine();
        let points = make_points(
            &mut e,
            &SEGMENT,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, -2.0, 5.0],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[(1, 1, 0x11, [0, 1, 0, 0])],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 1);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        assert_eq!(e.mem.u32(cell), out);
        assert_eq!(calls(&e, OBJECT_SLOT_14).len(), 1);
    }

    #[test]
    fn dropped_entries_are_replaced_by_the_last_one_and_the_words_follow() {
        let mut e = engine();
        let vertices: [Vec4; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        let points = make_points(
            &mut e,
            &vertices,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 5.0],
        );
        // Entry 1 uses the far vertices (squared length 100 > 5) and goes.
        let record = make_record(
            &mut e,
            [1, 1],
            &[
                (1, 1, 0x11, [0, 1, 0, 0]),
                (1, 1, 0x22, [2, 3, 0, 0]),
                (1, 1, 0x33, [0, 1, 0, 0]),
            ],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 3);
        let last = entry_bytes(&e, record, 2);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        assert_eq!(e.mem.u8(record + 2), 2);
        assert_eq!(calls(&e, OBJECT_SLOT_14), vec![vec![object, 0x22, 0xe7a]]);
        // Entry 1 is now the old last entry.
        assert_eq!(entry_bytes(&e, record, 1), last);
        assert_eq!(e.mem.u32(out + 0x20), 0x11);
        assert_eq!(e.mem.u32(out + 0x30 + 0x20), 0x33);
        assert_eq!(e.mem.u32(cell), out + 0x60);
        // The words follow the entries: they start at +4 + 2 * 8 now.
        let table = record + 4 + 16;
        assert_eq!([e.mem.u16(table), e.mem.u16(table + 2)], [0xaaa1, 0xaaa2]);
        assert_eq!(calls(&e, MANIFOLD_REMOVE_POINT), vec![vec![record]]);
    }

    /// p181: the entry moved in from the end goes through the same check at the dropped
    /// entry's place (`SUB EDI,0x8` at 00daed70). Entries 0 and 2 go, entry 1 stays: the old
    /// last entry (0x33) moves to place 0 and goes too, then 0x22 moves there and stays. Before
    /// the fix the pointer moved on, 0x33 stayed in the record with its id already freed (the
    /// next step wrote it to the output or freed it a second time) and 0x22 was lost.
    #[test]
    fn an_entry_moved_in_from_the_end_is_checked_at_the_dropped_place() {
        let mut e = engine();
        let vertices: [Vec4; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        let points = make_points(
            &mut e,
            &vertices,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 5.0],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[
                (1, 1, 0x11, [2, 3, 0, 0]),
                (1, 1, 0x22, [0, 1, 0, 0]),
                (1, 1, 0x33, [2, 3, 0, 0]),
            ],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 3);
        let kept = entry_bytes(&e, record, 1);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        assert_eq!(
            calls(&e, OBJECT_SLOT_14),
            vec![vec![object, 0x11, 0xe7a], vec![object, 0x33, 0xe7a]]
        );
        assert_eq!(e.mem.u8(record + 2), 1);
        assert_eq!(entry_bytes(&e, record, 0), kept);
        assert_eq!(e.mem.u32(out + 0x20), 0x22);
        assert_eq!(e.mem.u32(cell), out + 0x30);
        let table = record + 4 + 8;
        assert_eq!([e.mem.u16(table), e.mem.u16(table + 2)], [0xaaa1, 0xaaa2]);
    }

    #[test]
    fn entries_before_the_start_and_entries_with_other_counts_are_handled() {
        let mut e = engine();
        let points = make_points(
            &mut e,
            &SEGMENT,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 5.0],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[
                (1, 1, 0x11, [0, 1, 0, 0]),
                (2, 3, 0xffff, [0, 1, 0, 0]),
                (1, 1, 0x33, [0, 1, 0, 0]),
            ],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 3);
        // Start at entry 1: the unknown count pair goes without a call (id 0xffff), the entry
        // before the start stays as it is.
        e.call(0x00da_e3a0, &args![record, points, 1u32, cell, object]);
        assert!(calls(&e, OBJECT_SLOT_14).is_empty());
        assert_eq!(e.mem.u8(record + 2), 2);
        assert_eq!(e.mem.u16(record + 4 + 2), 0x11);
        assert_eq!(e.mem.u16(record + 4 + 8 + 2), 0x33);
        assert_eq!(e.mem.u32(out + 0x20), 0x33);
        assert_eq!(e.mem.u32(cell), out + 0x30);
        // A start at or beyond the count does nothing.
        let mut e = engine();
        let points = make_points(
            &mut e,
            &SEGMENT,
            [0.5, 0.0, 0.0, 0.0],
            [0.25, 2.0, 1.0, 5.0],
        );
        let record = make_record(
            &mut e,
            [1, 1],
            &[(1, 1, 0x11, [0, 1, 0, 0])],
            &[0xaaa1, 0xaaa2],
        );
        let (object, cell, out) = make_object_and_cell(&mut e, 1);
        e.call(0x00da_e3a0, &args![record, points, 1u32, cell, object]);
        assert_eq!(e.mem.u32(cell), out);
        assert_eq!(e.mem.u8(record + 2), 1);
    }

    #[test]
    fn two_counts_that_have_no_case_go_with_their_id() {
        for counts in [(0u8, 0u8), (0, 1), (1, 0), (2, 0), (2, 3), (3, 3), (4, 4)] {
            let mut e = engine();
            let points = make_points(
                &mut e,
                &SEGMENT,
                [0.5, 0.0, 0.0, 0.0],
                [0.25, 2.0, 1.0, 5.0],
            );
            let record = make_record(
                &mut e,
                [1, 1],
                &[(counts.0, counts.1, 0x44, [0, 1, 0, 0])],
                &[0xaaa1, 0xaaa2],
            );
            let (object, cell, _) = make_object_and_cell(&mut e, 1);
            e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
            assert_eq!(e.mem.u8(record + 2), 0, "{counts:?}");
            assert_eq!(calls(&e, OBJECT_SLOT_14), vec![vec![object, 0x44, 0xe7a]]);
        }
    }

    fn case_inputs(e: &mut Engine, vertices: &[Vec4], extra: Vec4) -> (u32, u32) {
        let points = make_points(e, vertices, extra, [0.0, 0.0, 0.0, 5.0]);
        let entry = make_record(e, [0, 0], &[(0, 0, 0, [0, 1, 2, 3])], &[]) + 4;
        (entry, points)
    }

    #[test]
    fn the_triangle_case_with_one_and_two_vertices_finds_the_closest_point() {
        let mut e = engine();
        let vertices = [
            [1.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        let (entry, points) = case_inputs(&mut e, &vertices, [0.5, 0.0, 0.0, 0.0]);
        let result = case_one_two(&e, entry, points);
        assert_eq!(result.eax, 0);
        // Q = (1 - 0.5) * P1 + 0.5 * P2 = (1, 0, 0): XMM2 is P0 - Q.
        assert_eq!(result.x0, vertex(&e, points, 0));
        near_vec(result.x2, [0.0, 1.0, 0.0]);
        assert_eq!(result.x7, [0.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn the_triangle_case_with_two_and_one_vertices_finds_the_closest_point() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
        ];
        let (entry, points) = case_inputs(&mut e, &vertices, [0.5, 0.0, 0.0, 0.0]);
        let result = case_two_one(&e, entry, points);
        assert_eq!(result.eax, 0);
        // Q = (1 - 0.5) * P1 + 0.5 * P0 = (1, 0, 0): XMM2 is Q - P2.
        near_vec(result.x0, [1.0, 0.0, 0.0]);
        near_vec(result.x2, [0.0, -1.0, 0.0]);
        assert_eq!(result.x7, [0.5, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn the_triangle_cases_drop_a_negative_projection() {
        let mut e = engine();
        // P0 is behind P1 along the edge: (P0 - P1) . d21 is negative.
        let vertices = [
            [-1.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        let (entry, points) = case_inputs(&mut e, &vertices, [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(case_one_two(&e, entry, points).eax, 1);
    }

    /// `00dacf10`'s reading of a four-vertex set (the entry's vertices are `P0..P3`): the
    /// normal of `(P3 - P2) x (P1 - P3)`, the tests `(P0 - P2) x (P3 - P2)`, `(P0 - P3) x
    /// (P1 - P3)`, `(P0 - P1) x (P2 - P1)` and `V`, all against the unit normal.
    fn first_four_vertex_reading(p: &[Vec4], extra: Vec4) -> (Vec3, u32) {
        let [p0, p1, p2, p3] = [xyz(p[0]), xyz(p[1]), xyz(p[2]), xyz(p[3])];
        let d32 = sub(p3, p2);
        let d13 = sub(p1, p3);
        let u = unit(cross(d32, d13));
        let sums = [
            dot(cross(sub(p0, p2), d32), u),
            dot(cross(sub(p0, p3), d13), u),
            dot(cross(sub(p0, p1), sub(p2, p1)), u),
            dot(xyz(extra), u),
        ];
        let mask: u32 = (0..4).map(|k| u32::from(sums[k] < 0.0) << k).sum();
        let sign = if mask & 8 != 0 { -1.0 } else { 1.0 };
        ([sign * u[0], sign * u[1], sign * u[2]], !mask & 7)
    }

    /// `00dad110`'s reading: the normal of `(P2 - P1) x (P0 - P2)`, the tests `(P3 - P1) x
    /// (P2 - P1)`, `(P3 - P2) x (P0 - P2)`, `(P3 - P0) x (P1 - P0)` and `V`; the point
    /// `((P0 - P3) . u) u + P3`.
    fn second_four_vertex_reading(p: &[Vec4], extra: Vec4) -> (Vec3, Vec3, u32) {
        let [p0, p1, p2, p3] = [xyz(p[0]), xyz(p[1]), xyz(p[2]), xyz(p[3])];
        let d21 = sub(p2, p1);
        let d02 = sub(p0, p2);
        let u = unit(cross(d21, d02));
        let sums = [
            dot(cross(sub(p3, p1), d21), u),
            dot(cross(sub(p3, p2), d02), u),
            dot(cross(sub(p3, p0), sub(p1, p0)), u),
            dot(xyz(extra), u),
        ];
        let mask: u32 = (0..4).map(|k| u32::from(sums[k] < 0.0) << k).sum();
        let along = dot(sub(p0, p3), u);
        let point = [
            along * u[0] + p3[0],
            along * u[1] + p3[1],
            along * u[2] + p3[2],
        ];
        let sign = if mask & 8 != 0 { -1.0 } else { 1.0 };
        ([sign * u[0], sign * u[1], sign * u[2]], point, !mask & 7)
    }

    const QUADS: [[Vec4; 4]; 3] = [
        [
            [1.0, 1.0, 0.0, 0.5],
            [0.0, 0.0, 0.0, 0.25],
            [4.0, 0.0, 0.0, 0.75],
            [0.0, 4.0, 0.0, 1.5],
        ],
        [
            [5.0, 5.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.25],
            [4.0, 0.0, 0.0, 0.75],
            [0.0, 4.0, 0.0, 1.5],
        ],
        [
            [1.0, 1.0, 3.0, 0.5],
            [0.0, 0.0, 0.0, 0.25],
            [4.0, 0.0, 0.0, 0.75],
            [5.0, 5.0, 1.0, 2.0],
        ],
    ];

    #[test]
    fn the_four_vertex_case_with_one_and_three_matches_the_first_reading() {
        for quad in QUADS {
            for extra in [[0.0, 0.0, 2.0, 0.0], [0.0, 0.0, -2.0, 0.0]] {
                let mut e = engine();
                let (entry, points) = case_inputs(&mut e, &quad, extra);
                let result = case_one_three(&e, entry, points);
                let (normal, eax) = first_four_vertex_reading(&quad, extra);
                assert_eq!(result.eax, eax);
                assert_eq!(result.x0, quad[0]);
                let wanted: Vec4 = std::array::from_fn(|k| quad[0][k] - quad[1][k]);
                assert_eq!(result.x2, wanted);
                near_vec(result.x7, normal);
            }
        }
    }

    #[test]
    fn the_four_vertex_case_with_three_and_one_matches_the_second_reading() {
        for quad in QUADS {
            for extra in [[0.0, 0.0, 2.0, 0.0], [0.0, 0.0, -2.0, 0.0]] {
                let mut e = engine();
                let (entry, points) = case_inputs(&mut e, &quad, extra);
                let result = case_three_one(&e, entry, points);
                let (normal, point, eax) = second_four_vertex_reading(&quad, extra);
                assert_eq!(result.eax, eax);
                near_vec(result.x0, point);
                let wanted: Vec4 = std::array::from_fn(|k| quad[0][k] - quad[3][k]);
                assert_eq!(result.x2, wanted);
                near_vec(result.x7, normal);
            }
        }
    }

    /// The closest points of the segments `P0 P1` and `P2 P3` (see `00dad340`): the four sums
    /// and `(Q0, Q0 - Q1)`.
    fn closest_points(p: &[Vec4]) -> ([f32; 4], Vec3, Vec3) {
        let [p0, p1, p2, p3] = [xyz(p[0]), xyz(p[1]), xyz(p[2]), xyz(p[3])];
        let d10 = sub(p1, p0);
        let d32 = sub(p3, p2);
        let n = cross(d10, d32);
        let m0 = cross(d32, n);
        let m2 = cross(d10, n);
        let sums = [
            dot(m0, sub(p0, p2)),
            dot(m2, sub(p0, p2)),
            dot(m0, sub(p1, p3)),
            dot(m2, sub(p1, p3)),
        ];
        let t = sums[0] / (sums[0] - sums[2]);
        let u = sums[1] / (sums[1] - sums[3]);
        let q0: Vec3 = std::array::from_fn(|k| (1.0 - t) * p0[k] + t * p1[k]);
        let q1: Vec3 = std::array::from_fn(|k| (1.0 - u) * p2[k] + u * p3[k]);
        (sums, q0, sub(q0, q1))
    }

    #[test]
    fn the_two_segment_case_finds_the_closest_points_or_drops_the_entry() {
        let crossing = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, -1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
        ];
        let apart = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [5.0, -1.0, 1.0, 0.0],
            [5.0, 1.0, 1.0, 0.0],
        ];
        let extra = [7.0, 8.0, 9.0, 10.0];
        let mut e = engine();
        let (entry, points) = case_inputs(&mut e, &crossing, extra);
        let result = case_two_two(&e, entry, points);
        let (sums, q0, difference) = closest_points(&crossing);
        assert_eq!(sums, [-8.0, -8.0, 8.0, 8.0]);
        assert_eq!(result.eax, 0);
        near_vec(result.x0, q0);
        near_vec(result.x2, difference);
        assert_eq!(result.x7, extra);
        let mut e = engine();
        let (entry, points) = case_inputs(&mut e, &apart, extra);
        assert_eq!(case_two_two(&e, entry, points).eax, 1);
    }

    #[test]
    fn the_segment_case_through_the_whole_function_keeps_the_output_shape() {
        // A (1, 3) entry through the main loop: its output record is written from the result.
        let mut e = engine();
        let quad = QUADS[0];
        let points = make_points(&mut e, &quad, [0.0, 0.0, 2.0, 0.0], [0.25, 2.0, 1.0, 5.0]);
        let record = make_record(&mut e, [1, 3], &[(1, 3, 0x55, [0, 1, 2, 3])], &[1, 2, 3, 4]);
        let (object, cell, out) = make_object_and_cell(&mut e, 1);
        e.call(0x00da_e3a0, &args![record, points, 0u32, cell, object]);
        let (normal, eax) = first_four_vertex_reading(&quad, [0.0, 0.0, 2.0, 0.0]);
        // The four-vertex sign test answers `!mask & 7` and only 0 keeps the entry.
        if eax == 0 {
            assert_eq!(e.mem.u32(cell), out + 0x30);
            assert_eq!(e.mem.u32(out + 0x20), 0x55);
            near_vec(load_vec(&e, out + 0x10), normal);
        } else {
            assert_eq!(e.mem.u32(cell), out);
            assert_eq!(calls(&e, OBJECT_SLOT_14), vec![vec![object, 0x55, 0xe7a]]);
        }
    }
}
