//! `Havok SDK/run_00dacf10` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addps, addss, andnps, cmpeqss, cmpltps, divss, load_vec, movmskps, mulps, mulss, rsqrtss,
    shufps, store_vec, subps, subss, xorps, Vec4,
};

/// The `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) floats of the normalising
/// step, the four sign bits `0x80000000` (`010c44d0`) and the four `1.0` (`010ec7f0`).
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_THREE: u32 = 0x0101_7718;
const FLOAT_HALF: u32 = 0x0101_6248;
const SIGN_BITS: u32 = 0x010c_44d0;
const ONES: u32 = 0x010e_c7f0;

/// A float of the image as `MOVSS xmm, [address]` loads it: lane 0, the other lanes zero.
fn scalar_vec(e: &Engine, address: u32) -> Vec4 {
    [f32::from_bits(e.mem.u32(address)), 0.0, 0.0, 0.0]
}

/// The four vertex byte offsets of a triangle record (`+4` to `+7`).
fn vertex_offsets(e: &Engine, record: u32) -> [u32; 4] {
    std::array::from_fn(|i| u32::from(e.mem.u8(record + 4 + i as u32)))
}

/// The normal `normal` scaled to unit length the way the exe does it: the squared length
/// `z + (y + x)` of `normal * normal`, its `RSQRTSS` estimate `r` (the host's), and the factor
/// `(0.5 * r) * (3.0 - (length * r) * r)`, which is 0 when the squared length equals the image's
/// `0.0` (`CMPEQSS`, `ANDNPS`; a NaN is not equal). The result is that factor times `normal`.
fn scaled_normal(e: &Engine, normal: Vec4) -> Vec4 {
    let squares = mulps(normal, normal);
    let sum = addss(shufps(squares, squares, 0x55), squares);
    let length_squared = addss(shufps(squares, squares, 0xaa), sum);
    let estimate = rsqrtss(length_squared, length_squared);
    let is_zero = cmpeqss(length_squared, scalar_vec(e, FLOAT_ZERO));
    let product = mulss(mulss(length_squared, estimate), estimate);
    let difference = subss(scalar_vec(e, FLOAT_THREE), product);
    let factor = mulss(mulss(scalar_vec(e, FLOAT_HALF), estimate), difference);
    let kept = andnps(is_zero, factor);
    mulps(shufps(kept, kept, 0), normal)
}

/// The horizontal gather the exe does with its eight `SHUFPS` before the sign test: lane `k` of
/// the result is `z + (y + x)` of `rows[k]` (`rows[3]`'s lanes 0 to 2; four rows of already
/// multiplied vectors).
fn gather_sums(rows: [Vec4; 4]) -> Vec4 {
    let [r0, r1, r2, r3] = rows;
    let high_a = shufps(r0, r1, 0xee);
    let high_b = shufps(r2, r3, 0xee);
    let high = shufps(high_a, high_b, 0x88);
    let low_a = shufps(r0, r1, 0x44);
    let low_b = shufps(r2, r3, 0x44);
    let odd = shufps(low_a, low_b, 0xdd);
    let even = shufps(low_a, low_b, 0x88);
    addps(high, addps(odd, even))
}

/// The sign test and output of the two triangle functions below: `MOVMSKPS` of `sums < 0` (bit 3
/// is the extra vector's), the normal stored at `out + 0x20` (negated when bit 3 is set) and the
/// answer `!mask & 7`.
fn store_oriented_normal(e: &mut Engine, out: u32, scaled: Vec4, sums: Vec4) -> u32 {
    let mask = movmskps(cmpltps(sums, [0.0; 4]));
    store_vec(e, out + 0x20, scaled);
    if mask & 8 != 0 {
        let sign = load_vec(e, SIGN_BITS);
        store_vec(e, out + 0x20, xorps(scaled, sign));
    }
    !mask & 7
}

// Translated from 00dacf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` (a triangle record: byte offsets `o0..o3` of four vertices from the base
/// at `+4` to `+7`) and `ecx` (the base address of the vertex data), then the stack word `out`;
/// cdecl, answers `!mask & 7`. Vertices `P0..P3` are the 16-byte vectors at `base + o_k`, and
/// `V` the vector at `base + 0x100`. The record at `out` gets `P0` (`+0`), `P0 - P1` (`+0x10`)
/// and the unit normal `u` of the cross product `(P3 - P2) x (P1 - P3)` (`+0x20`, see
/// `scaled_normal`), negated when `V . u < 0`. The mask has the bits `(P0 - P2) x (P3 - P2) .
/// u < 0`, `(P0 - P3) x (P1 - P3) . u < 0`, `(P0 - P1) x (P2 - P1) . u < 0` and `V . u < 0` (every
/// dot product is `z + (y + x)` of the lane products), so the answer has a bit for each of the
/// first three that is not negative. The exe's name for it is unknown (a triangle's plane and
/// edge tests).
pub fn fn_00dacf10(e: &mut Engine, eax: u32, ecx: u32, out: u32) -> u32 {
    let [o0, o1, o2, o3] = vertex_offsets(e, eax);
    let address0 = o0.wrapping_add(ecx);
    let first = load_vec(e, address0);
    store_vec(e, out, first);
    let p1 = load_vec(e, o1.wrapping_add(ecx));
    let again = load_vec(e, address0);
    store_vec(e, out + 0x10, subps(again, p1));
    let p3 = load_vec(e, o3.wrapping_add(ecx));
    let p2 = load_vec(e, o2.wrapping_add(ecx));
    let p1 = load_vec(e, o1.wrapping_add(ecx));
    let d32 = subps(p3, p2);
    let a = shufps(d32, d32, 0xd2);
    let bc = shufps(d32, d32, 0xc9);
    let d13 = subps(p1, p3);
    let c = shufps(d13, d13, 0xc9);
    let d = shufps(d13, d13, 0xd2);
    let d21 = subps(p2, p1);
    let normal = subps(mulps(bc, d), mulps(a, c));
    let scaled = scaled_normal(e, normal);
    let p0 = load_vec(e, address0);
    let e02 = subps(p0, p2);
    let e01 = subps(p0, p1);
    let e03 = subps(p0, p3);
    let x0 = subps(
        mulps(shufps(e02, e02, 0xc9), a),
        mulps(shufps(e02, e02, 0xd2), bc),
    );
    let x1 = subps(
        mulps(shufps(e03, e03, 0xc9), d),
        mulps(shufps(e03, e03, 0xd2), c),
    );
    let x2 = subps(
        mulps(shufps(e01, e01, 0xc9), shufps(d21, d21, 0xd2)),
        mulps(shufps(e01, e01, 0xd2), shufps(d21, d21, 0xc9)),
    );
    let extra = load_vec(e, ecx.wrapping_add(0x100));
    let rows = [
        mulps(x0, scaled),
        mulps(x1, scaled),
        mulps(x2, scaled),
        mulps(extra, scaled),
    ];
    store_oriented_normal(e, out, scaled, gather_sums(rows))
}

// Translated from 00dad110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` (a triangle record, see `00dacf10`), `ecx` (the vertex base) and `esi`
/// (the output record); cdecl, answers `!mask & 7`. `P3` goes to `out + 0`, `P0 - P3` to
/// `out + 0x10`, and the unit normal `u` of `(P2 - P1) x (P0 - P2)` (`scaled_normal`) to `out +
/// 0x20`, negated when `V . u < 0`. The mask has the bits `(P3 - P1) x (P2 - P1) . u < 0`, `(P3 -
/// P2) x (P0 - P2) . u < 0`, `(P3 - P0) x (P1 - P0) . u < 0` and `V . u < 0`. Afterwards `out + 0`
/// is overwritten with `((P0 - P3) . u) * u + P3`, the projection of `P3` on the plane through
/// `P0`. The exe's name for it is unknown (the sibling of `00dacf10`).
pub fn fn_00dad110(e: &mut Engine, eax: u32, ecx: u32, esi: u32) -> u32 {
    let out = esi;
    let [o0, o1, o2, o3] = vertex_offsets(e, eax);
    let (address0, address1) = (o0.wrapping_add(ecx), o1.wrapping_add(ecx));
    let (address2, address3) = (o2.wrapping_add(ecx), o3.wrapping_add(ecx));
    let first = load_vec(e, address3);
    store_vec(e, out, first);
    let p3 = load_vec(e, address3);
    let p0 = load_vec(e, address0);
    let g = subps(p0, p3);
    store_vec(e, out + 0x10, g);
    let p2 = load_vec(e, address2);
    let p0 = load_vec(e, address0);
    let p1 = load_vec(e, address1);
    let d02 = subps(p0, p2);
    let c = shufps(d02, d02, 0xc9);
    let d = shufps(d02, d02, 0xd2);
    let d21 = subps(p2, p1);
    let a = shufps(d21, d21, 0xd2);
    let bc = shufps(d21, d21, 0xc9);
    let a_times_c = mulps(a, c);
    let d10 = subps(p1, p0);
    let normal = subps(mulps(bc, d), a_times_c);
    let scaled = scaled_normal(e, normal);
    let p3 = load_vec(e, address3);
    let e31 = subps(p3, p1);
    let e32 = subps(p3, p2);
    let e30 = subps(p3, p0);
    let x0 = subps(
        mulps(shufps(e31, e31, 0xc9), a),
        mulps(shufps(e31, e31, 0xd2), bc),
    );
    let x1 = subps(
        mulps(shufps(e32, e32, 0xc9), d),
        mulps(shufps(e32, e32, 0xd2), c),
    );
    let x2 = subps(
        mulps(shufps(e30, e30, 0xc9), shufps(d10, d10, 0xd2)),
        mulps(shufps(e30, e30, 0xd2), shufps(d10, d10, 0xc9)),
    );
    let extra = load_vec(e, ecx.wrapping_add(0x100));
    let scaled_extra = mulps(extra, scaled);
    let rows = [
        mulps(x0, scaled),
        mulps(x1, scaled),
        mulps(x2, scaled),
        scaled_extra,
    ];
    let projected = mulps(g, scaled);
    let along = addss(shufps(projected, projected, 0x55), projected);
    let along = addss(shufps(projected, projected, 0xaa), along);
    let sums = gather_sums(rows);
    let offset = mulps(shufps(along, along, 0), scaled);
    let mask = movmskps(cmpltps(sums, [0.0; 4]));
    store_vec(e, out, addps(offset, p3));
    store_vec(e, out + 0x20, scaled);
    if mask & 8 != 0 {
        let sign = load_vec(e, SIGN_BITS);
        store_vec(e, out + 0x20, xorps(scaled, sign));
    }
    !mask & 7
}

// Translated from 00dad340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs `eax` (the vertex base) and `ecx` (a triangle record, see `00dacf10`), then
/// the stack word `out`; cdecl, answers 0 or 1. With `N = (P1 - P0) x (P3 - P2)`, `M0 = (P3 - P2)
/// x N` and `M2 = (P1 - P0) x N`, the four sums (`z + (y + x)` of the lane products) `M0 . (P0 -
/// P2)`, `M2 . (P0 - P2)`, `M0 . (P1 - P3)` and `M2 . (P1 - P3)` are tested for `< 0`; unless
/// the mask is 3 or 0xc the answer is 1 and only `V` (the vector at `base + 0x100`) is stored at
/// `out + 0x20`. Otherwise (answer 0) the closest points of the segments `P0 P1` and `P2 P3` are
/// stored: `out + 0` the point `(1 - t) * P0 + t * P1` for `t = s0 / (s0 - s2)` and `out + 0x10`
/// its difference to `(1 - u) * P2 + u * P3` for `u = s1 / (s1 - s3)` (`s0..s3` the four sums; the
/// division `DIVSS`, the `1.0` vector from `010ec7f0`). The exe's name for it is unknown.
pub fn fn_00dad340(e: &mut Engine, eax: u32, ecx: u32, out: u32) -> u32 {
    let base = eax;
    let [o0, o1, o2, o3] = vertex_offsets(e, ecx);
    let p3 = load_vec(e, o3.wrapping_add(base));
    let p1 = load_vec(e, o1.wrapping_add(base));
    let p2 = load_vec(e, o2.wrapping_add(base));
    let p0 = load_vec(e, o0.wrapping_add(base));
    let d10 = subps(p1, p0);
    let d13 = subps(p1, p3);
    let d02 = subps(p0, p2);
    let d32 = subps(p3, p2);
    let bc = shufps(d32, d32, 0xc9);
    let a = shufps(d10, d10, 0xd2);
    let f = shufps(d32, d32, 0xd2);
    let g = shufps(d10, d10, 0xc9);
    let normal = subps(mulps(g, f), mulps(a, bc));
    let n_yzx = shufps(normal, normal, 0xc9);
    let n_zxy = shufps(normal, normal, 0xd2);
    let m2 = subps(mulps(n_zxy, g), mulps(n_yzx, a));
    let m0 = subps(mulps(n_zxy, bc), mulps(n_yzx, f));
    let extra = load_vec(e, base.wrapping_add(0x100));
    store_vec(e, out + 0x20, extra);
    let rows = [
        mulps(m0, d02),
        mulps(m2, d02),
        mulps(m0, d13),
        mulps(m2, d13),
    ];
    let sums = gather_sums(rows);
    let mask = movmskps(cmpltps(sums, [0.0; 4]));
    if mask != 3 && mask != 0xc {
        return 1;
    }
    let first = load_vec(e, o0.wrapping_add(base));
    let z = shufps(sums, sums, 0xaa);
    let along = divss(sums, subss(sums, z));
    let t = shufps(along, along, 0);
    let ones = load_vec(e, ONES);
    let q0 = addps(
        mulps(subps(ones, t), first),
        mulps(load_vec(e, o1.wrapping_add(base)), t),
    );
    let y = shufps(sums, sums, 0x55);
    let w = shufps(sums, sums, 0xff);
    store_vec(e, out, q0);
    let across = divss(y, subss(y, w));
    let second = load_vec(e, o2.wrapping_add(base));
    let u = shufps(across, across, 0);
    let q1 = addps(
        mulps(subps(load_vec(e, ONES), u), second),
        mulps(load_vec(e, o3.wrapping_add(base)), u),
    );
    store_vec(e, out + 0x10, subps(q0, q1));
    0
}

/// A word of the table that follows the `n` 8-byte entries of a `00dad4d0` record: the 16-bit word
/// at `record + 4 + n * 8 + (byte >> 3)`, where `byte` is the byte at `at`.
fn table_word(e: &Engine, record: u32, count: u32, at: u32) -> u16 {
    let index = u32::from(e.mem.u8(at)) >> 3;
    e.mem.u16(record + 4 + count.wrapping_mul(8) + index)
}

// Translated from 00dad4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl `(record, key)`, answers a bool (`AL`). `record` has the entry count `n` at `+2`, then
/// `n` 8-byte entries from `+4` (two match bytes, two unused, then four index bytes) and after
/// them a table of 16-bit words; the index bytes shifted right by 3 are byte offsets into it.
/// `key` holds four words at `+0` and the byte counts `k8` and `k9` at `+8` and `+9`.
///
/// First pass: the first entry whose match bytes equal `(k8, k9)` and whose first `k8 + k9`
/// (at most 4) indexed words equal the key's words has the entry swapped with entry 0 (when it
/// is not entry 0 itself) and the answer is true. Second pass (only when `k8` or `k9` is 1, and
/// not reached after a match): the first entry in whose first match byte `k8 == 1` and the word
/// of its first index byte equals the key's first word, or whose second match byte and `k9` are
/// both 1 and the word of the index byte stored `entry[0]` bytes after the entry's own first
/// index byte equals the key's word number `k8`, has both its match bytes cleared; the answer
/// is false either way. The exe's name for it is unknown.
pub fn fn_00dad4d0(e: &mut Engine, record: u32, key: u32) -> bool {
    let count = u32::from(e.mem.u8(record + 2));
    let key_a = e.mem.u8(key + 8);
    let key_b = e.mem.u8(key + 9);
    if count != 0 {
        let mut index = 0u32;
        let mut entry = record + 4;
        loop {
            let mut matched = e.mem.u8(entry) == key_a && e.mem.u8(entry + 1) == key_b;
            if matched {
                let words = u32::from(key_a) + u32::from(key_b);
                matched = e.mem.u16(key) == table_word(e, record, count, entry + 4)
                    && e.mem.u16(key + 2) == table_word(e, record, count, entry + 5);
                if matched && words > 2 {
                    matched = e.mem.u16(key + 4) == table_word(e, record, count, entry + 6);
                }
                if matched && words > 3 {
                    matched = e.mem.u16(key + 6) == table_word(e, record, count, entry + 7);
                }
            }
            if matched {
                if index != 0 {
                    let slot = record + index * 8;
                    let first = e.mem.u32(record + 4);
                    let moved = e.mem.u32(slot + 4);
                    let moved_second = e.mem.u32(slot + 8);
                    e.mem.set_u32(slot + 4, first);
                    let second = e.mem.u32(record + 8);
                    e.mem.set_u32(slot + 8, second);
                    e.mem.set_u32(record + 4, moved);
                    e.mem.set_u32(record + 8, moved_second);
                }
                return true;
            }
            index += 1;
            entry += 8;
            if index >= u32::from(e.mem.u8(record + 2)) {
                break;
            }
        }
    }
    let count = u32::from(e.mem.u8(record + 2));
    if (key_a != 1 && key_b != 1) || count == 0 {
        return false;
    }
    let mut entry = record + 4;
    for _ in 0..count {
        let mut hit = false;
        if key_a == 1 && e.mem.u8(entry) == 1 {
            hit = table_word(e, record, count, entry + 4) == e.mem.u16(key);
        }
        if !hit && key_b == 1 && e.mem.u8(entry + 1) == 1 {
            let skip = u32::from(e.mem.u8(entry));
            let indexed = table_word(e, record, count, entry + skip + 4);
            hit = indexed == e.mem.u16(key + 2 * u32::from(key_a));
        }
        if hit {
            e.mem.set_u8(entry + 1, 0);
            e.mem.set_u8(entry, 0);
            return false;
        }
        entry += 8;
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_cf10, fn_00dacf10(u32, u32, u32) -> u32),
        entry!(0x00da_d110, fn_00dad110(u32, u32, u32) -> u32),
        entry!(0x00da_d340, fn_00dad340(u32, u32, u32) -> u32),
        entry!(0x00da_d4d0, fn_00dad4d0(u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    type Vec3 = [f32; 3];

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

    /// The vertex offsets of the record: vertex `k` is stored at `base + OFFSETS[k]`.
    const OFFSETS: [u8; 4] = [0x10, 0x20, 0x30, 0x40];

    struct Rig {
        e: Engine,
        base: u32,
        record: u32,
        out: u32,
    }

    /// A vertex base with the four points at the offsets `order` picks (point `k` is vertex
    /// `order[k]`'s), `extra` at `+0x100`, the image constants, a record and an output block
    /// filled with `0xdead` words.
    fn rig(points: [Vec4; 4], extra: Vec4, order: [usize; 4]) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(FLOAT_ZERO, 0);
        e.mem.set_u32(FLOAT_THREE, 3.0f32.to_bits());
        e.mem.set_u32(FLOAT_HALF, 0.5f32.to_bits());
        for lane in 0..4 {
            e.mem.set_u32(SIGN_BITS + 4 * lane, 0x8000_0000);
            e.mem.set_u32(ONES + 4 * lane, 1.0f32.to_bits());
        }
        let base = e.mem.alloc(0x120);
        for (k, point) in points.iter().enumerate() {
            store_vec(&mut e, base + u32::from(OFFSETS[order[k]]), *point);
        }
        store_vec(&mut e, base + 0x100, extra);
        let record = e.mem.alloc(8);
        for (k, slot) in order.iter().enumerate() {
            e.mem.set_u8(record + 4 + k as u32, OFFSETS[*slot]);
        }
        let out = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(out + 4 * word, 0xdead);
        }
        Rig {
            e,
            base,
            record,
            out,
        }
    }

    fn points_a() -> [Vec4; 4] {
        [
            [1.0, 1.0, 0.0, 0.5],
            [0.0, 0.0, 0.0, 0.25],
            [4.0, 0.0, 0.0, 0.75],
            [0.0, 4.0, 0.0, 1.5],
        ]
    }

    fn check_00dacf10(points: [Vec4; 4], extra: Vec4, order: [usize; 4]) -> u32 {
        let mut r = rig(points, extra, order);
        let answer = r.e.call(0x00da_cf10, &args![r.record, r.base, r.out]).u32();
        let [p0, p1, p2, p3] = points.map(xyz);
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
        assert_eq!(answer, !mask & 7);
        assert_eq!(load_vec(&r.e, r.out), points[0]);
        let first_minus_second: Vec4 = std::array::from_fn(|k| points[0][k] - points[1][k]);
        assert_eq!(load_vec(&r.e, r.out + 0x10), first_minus_second);
        let sign = if mask & 8 != 0 { -1.0 } else { 1.0 };
        near_vec(
            load_vec(&r.e, r.out + 0x20),
            [sign * u[0], sign * u[1], sign * u[2]],
        );
        assert_eq!(load_vec(&r.e, r.out + 0x20)[3].abs(), 0.0);
        answer
    }

    #[test]
    fn the_triangle_tests_give_one_bit_per_edge_that_is_not_negative() {
        let up = [0.0, 0.0, 2.0, 0.0];
        let down = [0.0, 0.0, -2.0, 0.0];
        // Inside: every cross product points the same way.
        assert_eq!(check_00dacf10(points_a(), up, [0, 1, 2, 3]), 0);
        // Outside across the second and third edge only.
        let mut outside = points_a();
        outside[0] = [5.0, 5.0, 0.0, 0.0];
        assert_eq!(check_00dacf10(outside, up, [0, 1, 2, 3]), 1);
        // The extra vector on the other side flips the stored normal, not the answer.
        assert_eq!(check_00dacf10(outside, down, [0, 1, 2, 3]), 1);
        // Another corner: positive for the first and third.
        outside[0] = [-2.0, 1.0, 0.0, 0.0];
        check_00dacf10(outside, up, [0, 1, 2, 3]);
    }

    #[test]
    fn the_record_picks_the_vertices_by_their_offsets() {
        let up = [0.0, 0.0, 2.0, 0.0];
        for order in [[3, 2, 1, 0], [1, 3, 0, 2], [2, 0, 3, 1]] {
            check_00dacf10(points_a(), up, order);
        }
    }

    #[test]
    fn a_degenerate_triangle_gets_a_zero_normal() {
        let mut r = rig(
            [[1.0, 1.0, 1.0, 0.0]; 4],
            [0.0, 0.0, 1.0, 0.0],
            [0, 1, 2, 3],
        );
        r.e.call(0x00da_cf10, &args![r.record, r.base, r.out]);
        let normal = load_vec(&r.e, r.out + 0x20);
        assert_eq!(&normal[..3], &[0.0, 0.0, 0.0]);
    }

    fn check_00dad110(points: [Vec4; 4], extra: Vec4, order: [usize; 4]) -> u32 {
        let mut r = rig(points, extra, order);
        let answer = r.e.call(0x00da_d110, &args![r.record, r.base, r.out]).u32();
        let [p0, p1, p2, p3] = points.map(xyz);
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
        assert_eq!(answer, !mask & 7);
        let along = dot(sub(p0, p3), u);
        let projected = load_vec(&r.e, r.out);
        near_vec(
            projected,
            [
                along * u[0] + p3[0],
                along * u[1] + p3[1],
                along * u[2] + p3[2],
            ],
        );
        assert_eq!(projected[3], points[3][3]);
        let p0_minus_p3: Vec4 = std::array::from_fn(|k| points[0][k] - points[3][k]);
        assert_eq!(load_vec(&r.e, r.out + 0x10), p0_minus_p3);
        let sign = if mask & 8 != 0 { -1.0 } else { 1.0 };
        near_vec(
            load_vec(&r.e, r.out + 0x20),
            [sign * u[0], sign * u[1], sign * u[2]],
        );
        answer
    }

    #[test]
    fn the_second_triangle_function_projects_the_last_vertex_on_the_plane() {
        let up = [0.0, 0.0, 2.0, 0.0];
        let down = [0.0, 0.0, -2.0, 0.0];
        let lifted = [
            [1.0, 1.0, 3.0, 0.5],
            [0.0, 0.0, 0.0, 0.25],
            [4.0, 0.0, 0.0, 0.75],
            [0.0, 4.0, 0.0, 1.5],
        ];
        check_00dad110(lifted, up, [0, 1, 2, 3]);
        check_00dad110(lifted, down, [0, 1, 2, 3]);
        let mut moved = lifted;
        moved[3] = [5.0, 5.0, 1.0, 2.0];
        check_00dad110(moved, up, [0, 1, 2, 3]);
        for order in [[3, 2, 1, 0], [1, 3, 0, 2]] {
            check_00dad110(moved, up, order);
        }
    }

    /// The reference of `00dad340`: the four sums and the closest points.
    fn closest_points(points: [Vec4; 4]) -> ([f32; 4], Vec3) {
        let [p0, p1, p2, p3] = points.map(xyz);
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
        (sums, sub(q0, q1))
    }

    fn run_00dad340(points: [Vec4; 4], order: [usize; 4]) -> (u32, Rig) {
        let mut r = rig(points, [7.0, 8.0, 9.0, 10.0], order);
        let answer = r.e.call(0x00da_d340, &args![r.base, r.record, r.out]).u32();
        (answer, r)
    }

    #[test]
    fn crossing_segments_give_their_closest_points() {
        let points = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, -1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
        ];
        let (answer, r) = run_00dad340(points, [0, 1, 2, 3]);
        assert_eq!(answer, 0);
        near_vec(load_vec(&r.e, r.out), [1.0, 0.0, 0.0]);
        near_vec(load_vec(&r.e, r.out + 0x10), [0.0, 0.0, -1.0]);
        assert_eq!(load_vec(&r.e, r.out + 0x20), [7.0, 8.0, 9.0, 10.0]);
        let (sums, difference) = closest_points(points);
        assert_eq!(sums, [-8.0, -8.0, 8.0, 8.0]);
        near_vec(load_vec(&r.e, r.out + 0x10), difference);
    }

    #[test]
    fn segments_that_do_not_cross_leave_only_the_extra_vector() {
        let points = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [5.0, -1.0, 1.0, 0.0],
            [5.0, 1.0, 1.0, 0.0],
        ];
        let (answer, r) = run_00dad340(points, [0, 1, 2, 3]);
        assert_eq!(answer, 1);
        assert_eq!(load_vec(&r.e, r.out), [f32::from_bits(0xdead); 4]);
        assert_eq!(load_vec(&r.e, r.out + 0x10), [f32::from_bits(0xdead); 4]);
        assert_eq!(load_vec(&r.e, r.out + 0x20), [7.0, 8.0, 9.0, 10.0]);
    }

    #[test]
    fn the_vertex_offsets_pick_the_segment_ends() {
        let points = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
            [1.0, -1.0, 1.0, 0.0],
        ];
        let (sums, expected) = closest_points(points);
        let mask: u32 = (0..4).map(|k| u32::from(sums[k] < 0.0) << k).sum();
        assert_eq!(mask, 3);
        for order in [[0, 1, 2, 3], [3, 2, 1, 0], [2, 3, 0, 1]] {
            let (answer, r) = run_00dad340(points, order);
            assert_eq!(answer, 0);
            near_vec(load_vec(&r.e, r.out), [1.0, 0.0, 0.0]);
            near_vec(load_vec(&r.e, r.out + 0x10), expected);
        }
    }

    /// A `00dad4d0` record with the entries (match bytes and index bytes) and the table words
    /// that follow; `key` has the words and the two counts.
    fn dad4d0_rig(
        entries: &[([u8; 2], [u8; 4])],
        table: [u16; 4],
        key_words: [u16; 4],
        counts: [u8; 2],
    ) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x80);
        e.mem.set_u8(record + 2, entries.len() as u8);
        for (i, (matches, indexes)) in entries.iter().enumerate() {
            let at = record + 4 + 8 * i as u32;
            e.mem.set_u8(at, matches[0]);
            e.mem.set_u8(at + 1, matches[1]);
            e.mem.set_u8(at + 2, 0xaa);
            e.mem.set_u8(at + 3, 0xbb);
            for (k, byte) in indexes.iter().enumerate() {
                e.mem.set_u8(at + 4 + k as u32, *byte);
            }
        }
        let table_at = record + 4 + 8 * entries.len() as u32;
        for (k, word) in table.iter().enumerate() {
            e.mem.set_u16(table_at + 2 * k as u32, *word);
        }
        let key = e.mem.alloc(0x10);
        for (k, word) in key_words.iter().enumerate() {
            e.mem.set_u16(key + 2 * k as u32, *word);
        }
        e.mem.set_u8(key + 8, counts[0]);
        e.mem.set_u8(key + 9, counts[1]);
        (e, record, key)
    }

    fn entry_bytes(e: &Engine, record: u32, index: u32) -> Vec<u8> {
        (0..8)
            .map(|k| e.mem.u8(record + 4 + 8 * index + k))
            .collect()
    }

    const TABLE: [u16; 4] = [0x1111, 0x2222, 0x3333, 0x4444];

    #[test]
    fn a_matching_entry_is_swapped_to_the_front() {
        let entries = [
            ([2, 2], [0, 16, 32, 48]),
            ([1, 0], [0, 16, 32, 48]),
            ([1, 1], [0, 16, 32, 48]),
        ];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x1111, 0x2222, 0, 0], [1, 1]);
        let third = entry_bytes(&e, record, 2);
        let first = entry_bytes(&e, record, 0);
        assert!(e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(entry_bytes(&e, record, 0), third);
        assert_eq!(entry_bytes(&e, record, 2), first);
    }

    #[test]
    fn a_first_entry_match_changes_nothing() {
        let entries = [([1, 1], [0, 16, 32, 48]), ([1, 1], [0, 16, 32, 48])];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x1111, 0x2222, 0, 0], [1, 1]);
        let before = entry_bytes(&e, record, 0);
        assert!(e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(entry_bytes(&e, record, 0), before);
    }

    #[test]
    fn more_counted_words_compare_more_of_the_key() {
        let indexes = [0, 16, 32, 48];
        let entries = [([2, 2], indexes)];
        let all = [0x1111, 0x2222, 0x3333, 0x4444];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, all, [2, 2]);
        assert!(e.call(0x00da_d4d0, &args![record, key]).bool());
        for wrong in 0..4 {
            let mut words = all;
            words[wrong] ^= 1;
            let (mut e, record, key) = dad4d0_rig(&entries, TABLE, words, [2, 2]);
            assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
        }
        // Three counted words ignore the fourth.
        let entries = [([2, 1], indexes)];
        let mut words = all;
        words[3] = 0;
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, words, [2, 1]);
        assert!(e.call(0x00da_d4d0, &args![record, key]).bool());
    }

    #[test]
    fn the_second_pass_clears_the_match_bytes_of_the_entry_it_finds() {
        // Key counts (1, 0): the entry's first match byte is 1 but the second differs, so the
        // first pass does not match; the word of its first index byte equals the key's word.
        let entries = [([3, 3], [0, 16, 32, 48]), ([1, 1], [48, 16, 32, 0])];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x4444, 0, 0, 0], [1, 0]);
        assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(&entry_bytes(&e, record, 1)[..2], &[0, 0]);
        assert_eq!(&entry_bytes(&e, record, 0)[..2], &[3, 3]);
    }

    #[test]
    fn the_second_pass_can_match_through_the_second_byte() {
        // Key counts (0, 1): the entry bytes are (3, 1): its first byte 3 names the index byte
        // at entry + 3 + 4 = the entry's last one, which indexes the table word 0x4444.
        let entries = [([3, 1], [0, 16, 32, 48])];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x4444, 0, 0, 0], [0, 1]);
        assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(&entry_bytes(&e, record, 0)[..2], &[0, 0]);
        // A word that is not the key's leaves the entry alone.
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x3333, 0, 0, 0], [0, 1]);
        assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(&entry_bytes(&e, record, 0)[..2], &[3, 1]);
    }

    #[test]
    fn without_a_one_in_the_key_or_any_entry_nothing_is_found() {
        let entries = [([5, 5], [0, 16, 32, 48])];
        let (mut e, record, key) = dad4d0_rig(&entries, TABLE, [0x1111, 0, 0, 0], [2, 2]);
        assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
        assert_eq!(&entry_bytes(&e, record, 0)[..2], &[5, 5]);
        // No entries at all.
        let (mut e, record, key) = dad4d0_rig(&[], TABLE, [0; 4], [1, 1]);
        assert!(!e.call(0x00da_d4d0, &args![record, key]).bool());
    }
}
