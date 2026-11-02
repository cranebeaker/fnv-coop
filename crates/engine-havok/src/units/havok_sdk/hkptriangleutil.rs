//! `Havok SDK/hkptriangleutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` the final difference is compared with (`0.0`, in `.rdata`).
const ZERO_01011D78: u32 = 0x0101_1d78;

/// Three lanes of `float` at `addr` (the code loads four; the fourth never
/// reaches the result).
fn read_vector_00ceb8d0(e: &Engine, addr: u32) -> [f32; 3] {
    [0, 4, 8].map(|offset| e.mem.f32(addr + offset))
}

fn sub_00ceb8d0(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// `(v1, v2, v0)` times `(u2, u0, u1)` minus `(v2, v0, v1)` times
/// `(u1, u2, u0)`: the cross product `v x u` as the code's `MULPS`/`SUBPS`
/// sequence computes it.
fn cross_00ceb8d0(v: [f32; 3], u: [f32; 3]) -> [f32; 3] {
    [
        v[2] * u[1] - v[1] * u[2],
        v[0] * u[2] - v[2] * u[0],
        v[1] * u[0] - v[0] * u[1],
    ]
}

/// The squared length as the `ADDSS` chain adds it: `s2 + (s1 + s0)` of the
/// squared lanes.
fn squared_length_00ceb8d0(v: [f32; 3]) -> f32 {
    let s = [v[0] * v[0], v[1] * v[1], v[2] * v[2]];
    s[2] + (s[1] + s[0])
}

// Translated from 00ceb8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleUtil::isDegenerate` (map name). cdecl, five words: writes 1
/// to the byte `result` when the triangle `a`, `b`, `c` (three floats each)
/// is degenerate for `tolerance`, otherwise 0. SSE (`f32`) with the
/// compares done on the x87 stack, so NaN is handled as the flags say:
/// - degenerate when `tolerance > |(a - c) x (a - b)|^2` (ordered compare);
/// - degenerate when `|(b - c) x (b - a)|^2 < tolerance` (ordered compare);
/// - otherwise with `u = a - b`, `g = c - b` it is degenerate exactly when
///   `f32(|u|^2 * |g|^2) - f32(dot(g, u)^2)` equals 0.0, where the product
///   and the square are each rounded to `float` and the subtraction is
///   rounded once more (all three are `FSTP float` in the code).
///
/// Answers `result` (`EAX` at both `RET`s, 00ceba4f and 00ceba5b: `MOV EAX,[EBP+8]` at 00ceba44
/// and 00ceba52); its callers read the answer through it (`CMP byte ptr [EAX],0` at 00d55bb9,
/// 00d561d2, 00dde8d4).
pub fn hkp_triangle_util_is_degenerate(
    e: &mut Engine,
    result: Ptr,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    tolerance: f32,
) -> Ptr {
    let a = read_vector_00ceb8d0(e, a.addr());
    let b = read_vector_00ceb8d0(e, b.addr());
    let c = read_vector_00ceb8d0(e, c.addr());
    let tolerance = x87_quiet(tolerance);
    let u = sub_00ceb8d0(a, b);
    let v = sub_00ceb8d0(a, c);
    let first = squared_length_00ceb8d0(cross_00ceb8d0(v, u));
    // `FCOM` then `TEST AH,0x41`: leave when the tolerance is above the sum.
    let degenerate = if tolerance > first {
        true
    } else {
        let w = sub_00ceb8d0(b, a);
        let r = sub_00ceb8d0(b, c);
        let second = squared_length_00ceb8d0(cross_00ceb8d0(r, w));
        // `FCOMPP` then `TEST AH,5`: leave when the sum is below the tolerance.
        if second < tolerance {
            true
        } else {
            let length_ab = squared_length_00ceb8d0(u);
            let g = sub_00ceb8d0(c, b);
            let length_bc = squared_length_00ceb8d0(g);
            let dot = dot_00ceb8d0(g, u);
            let product = (length_ab as f64 * length_bc as f64) as f32;
            let dot_squared = (dot as f64 * dot as f64) as f32;
            let difference = (product as f64 - dot_squared as f64) as f32;
            difference == e.global::<f32>(ZERO_01011D78)
        }
    };
    e.mem.set_u8(result.addr(), degenerate as u8);
    result // 00ceba44 / 00ceba52: MOV EAX,[EBP+8], the byte written through it, then RET
}

/// The dot product as the `ADDSS` chain adds it: `p2 + (p1 + p0)`.
fn dot_00ceb8d0(g: [f32; 3], u: [f32; 3]) -> f32 {
    let p = [g[0] * u[0], g[1] * u[1], g[2] * u[2]];
    p[2] + (p[1] + p[0])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ceb8d0,
        hkp_triangle_util_is_degenerate(Ptr, Ptr, Ptr, Ptr, f32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_degenerate_00ceb8d0(points: [[f32; 3]; 3], tolerance: f32) -> u8 {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(ZERO_01011D78, 0.0f32);
        let mut pointers = [Ptr::<()>::NULL; 3];
        for (slot, point) in pointers.iter_mut().zip(points) {
            let block = e.mem.alloc(16);
            for (i, value) in point.iter().enumerate() {
                e.mem.set_f32(block + 4 * i as u32, *value);
            }
            *slot = Ptr::<()>::new(block);
        }
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xee);
        e.call(
            0x00ce_b8d0,
            &args![
                Ptr::<()>::new(result),
                pointers[0],
                pointers[1],
                pointers[2],
                tolerance
            ],
        );
        e.mem.u8(result)
    }

    #[test]
    fn hkp_triangle_util_is_degenerate_accepts_a_proper_triangle() {
        let triangle = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_eq!(is_degenerate_00ceb8d0(triangle, 1.0e-6), 0);
    }

    #[test]
    fn hkp_triangle_util_is_degenerate_rejects_collinear_points_by_the_cross_products() {
        let line = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]];
        assert_eq!(is_degenerate_00ceb8d0(line, 1.0e-6), 1);
        // A tolerance above the squared area (1 here) also rejects it.
        let triangle = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_eq!(is_degenerate_00ceb8d0(triangle, 2.0), 1);
    }

    #[test]
    fn hkp_triangle_util_is_degenerate_answers_its_result_pointer() {
        // EAX is `result` at both RETs (00ceba44 and 00ceba52 load it); the callers read the
        // answer through it (00d55bb9 CMP byte ptr [EAX],0). One exit through each load.
        let cases = [
            ([[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]], 1u8),
            ([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], 0),
        ];
        for (points, expected) in cases {
            let mut e = Engine::new();
            e.map(0x0101_1000, 0x1000);
            e.set_global(ZERO_01011D78, 0.0f32);
            let mut words = [0u32; 3];
            for (word, point) in words.iter_mut().zip(points) {
                let block = e.mem.alloc(16);
                for (i, value) in point.iter().enumerate() {
                    e.mem.set_f32(block + 4 * i as u32, *value);
                }
                *word = block;
            }
            let result = e.mem.alloc(4);
            e.mem.set_u8(result, 0xee);
            let answered = e
                .call(
                    0x00ce_b8d0,
                    &args![result, words[0], words[1], words[2], 1.0e-6f32],
                )
                .u32();
            assert_eq!(answered, result);
            assert_eq!(e.mem.u8(answered), expected);
        }
    }

    #[test]
    fn hkp_triangle_util_is_degenerate_a_nan_tolerance_passes_both_compares() {
        let triangle = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_eq!(is_degenerate_00ceb8d0(triangle, f32::NAN), 0);
    }
}
