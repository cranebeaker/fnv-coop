//! `Havok SDK/hkvector4.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! All four are SSE code on 16-byte aligned `hkVector4` / `hkTransform` memory: the
//! transform is four columns (rotation columns at +0, +0x10, +0x20, translation at +0x30)
//! and every product is a `MULPS` of a broadcast lane with a column, summed in the order
//! the instructions add them (`f32` arithmetic, all four lanes).

#[allow(unused_imports)]
use crate::prelude::*;

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// `MULPS a, b` (the SSE NaN rules: `a` is the destination).
fn mul(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::mulps(a, b)
}

/// `ADDPS a, b` (the SSE NaN rules).
fn add(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::addps(a, b)
}

/// `SHUFPS v, v, 0x00 / 0x55 / 0xaa`: one lane broadcast to all four.
fn splat(v: Lanes, lane: usize) -> Lanes {
    [v[lane]; 4]
}

/// `(y * column1 + x * column0) + z * column2`, the order of `00c7f700` and `00c7f750`.
fn rotate(e: &Engine, matrix: u32, vector: Lanes) -> Lanes {
    let column0 = load(e, matrix);
    let column1 = load(e, matrix + 0x10);
    let column2 = load(e, matrix + 0x20);
    let by_y = mul(splat(vector, 1), column1);
    let by_x = mul(splat(vector, 0), column0);
    let by_z = mul(splat(vector, 2), column2);
    add(add(by_y, by_x), by_z)
}

// Translated from 00c7f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedPos` (Xbox PDB): `this = transform * vector`, the rotation
/// columns weighted by x, y, z plus the translation column.
pub fn set_transformed_pos(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let column0 = load(e, transform.addr());
    let column1 = load(e, transform.addr() + 0x10);
    let column2 = load(e, transform.addr() + 0x20);
    let translation = load(e, transform.addr() + 0x30);
    let by_y = mul(splat(v, 1), column1);
    let by_x = mul(splat(v, 0), column0);
    let by_z = mul(splat(v, 2), column2);
    let rotated = add(by_y, by_x);
    let moved = add(by_z, translation);
    store(e, this.addr(), add(rotated, moved));
}

// Translated from 00c7f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedInversePos` (Xbox PDB): `this = transpose(rotation) *
/// (vector - translation)`. The compiler's transpose of the three columns leaves lane 3 of
/// the first two rows as `column2.y` and of the third as `column2.w` (the `SHUFPS`
/// immediates `0xd4` / `0xe4`), which is what the fourth lane of the result uses.
pub fn set_transformed_inverse_pos(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let translation = load(e, transform.addr() + 0x30);
    let v: Lanes = sub(load(e, vector.addr()), translation);
    let c0 = load(e, transform.addr());
    let c1 = load(e, transform.addr() + 0x10);
    let c2 = load(e, transform.addr() + 0x20);
    let row0 = [c0[0], c1[0], c2[0], c2[1]];
    let row1 = [c0[1], c1[1], c2[1], c2[3]];
    let row2 = [c0[2], c1[2], c2[2], c2[3]];
    let by_y = mul(splat(v, 1), row1);
    let by_x = mul(splat(v, 0), row0);
    // `MULPS` with the row as the destination here.
    let by_z = mul(row2, splat(v, 2));
    store(e, this.addr(), add(add(by_y, by_x), by_z));
}

// Translated from 00c7f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setRotatedDir` (Xbox PDB): `this = rotation * vector` (no translation).
pub fn set_rotated_dir(e: &mut Engine, this: Ptr, rotation: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let result = rotate(e, rotation.addr(), v);
    store(e, this.addr(), result);
}

// Translated from 00c7f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setMul3` (Xbox PDB): the same instructions as `setRotatedDir`
/// (`00c7f700`), a 3x3 matrix times a vector.
pub fn set_mul3(e: &mut Engine, this: Ptr, matrix: Ptr, vector: Ptr) {
    let v = load(e, vector.addr());
    let result = rotate(e, matrix.addr(), v);
    store(e, this.addr(), result);
}

/// `1.0` (`double`) at `01012070`, what the normalisation tests subtract from the squared length.
const ONE: u32 = 0x0101_2070;

// Translated from 00c7f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (inputs in registers): whether the first `count` floats of an
/// array are finite. Registers first: `EAX` the address of the result byte (`out`), `EDX` the
/// number of floats (`count`, signed), `ESI` the array (`floats`). Each float goes through
/// `FLD`/`FSTP` (a signalling NaN is quieted; the exponent is kept) and the byte is set to 0
/// at the first whose exponent bits are all ones (infinity or NaN), else to 1 (also for a count
/// of zero or less). Only the byte at `out` is written.
pub fn fn_00c7f4a0(e: &mut Engine, out: Ptr, count: i32, floats: Ptr) {
    let mut index = 0u32;
    while (index as i32) < count {
        let bits = x87_quiet_bits(e.mem.u32(floats.addr().wrapping_add(4 * index)));
        if bits & 0x7f80_0000 == 0x7f80_0000 {
            e.mem.set_u8(out.addr(), 0);
            return;
        }
        index += 1;
    }
    e.mem.set_u8(out.addr(), 1);
}

// Translated from 00c7f4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::isOk3` (Xbox PDB), `thiscall`, `RET 4` (`out`): `00c7f4a0` over the first three
/// lanes; returns `out`.
pub fn hk_vector4_is_ok3(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    fn_00c7f4a0(e, out, 3, this);
    out
}

// Translated from 00c7f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `out`): `00c7f4a0` over all four lanes (the
/// four-lane form of `isOk3`); returns `out`.
pub fn fn_00c7f500(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    fn_00c7f4a0(e, out, 4, this);
    out
}

/// The test of both normalisation checks, from the squared length `sum` (a `float` the code
/// stores with `MOVSS` and loads with `FLD`): `|sum - 1.0| < epsilon`, the subtraction in
/// `double` (`FSUB` with the constant at `01012070`) rounded to `float` by `FSTP`, `FABS` and
/// another `FSTP`/`FLD`, the compare `FCOMPP` (`TEST AH,0x41`): true only when `epsilon` is
/// above the absolute value, neither being a NaN.
fn close_to_one(e: &Engine, sum: f32, epsilon: f32) -> bool {
    let one: f64 = e.global(ONE);
    let difference = x87_sub(x87_load32(sum.to_bits()), one);
    let magnitude = x87_load32(x87_store32(difference)).abs();
    let kept = x87_load32(x87_store32(magnitude));
    x87_load32(epsilon.to_bits()) > kept
}

// Translated from 00c7f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::isNormalized3` (Xbox PDB), `thiscall`, `RET 8` (`out`, `epsilon`): when
/// `isOk3` finds the three lanes finite, the squared length (`z*z + (y*y + x*x)`) is compared
/// with 1 (`close_to_one`); the byte at `out` is 1 when it is within `epsilon`, else 0 (also
/// when a lane is not finite). Returns `out`.
pub fn hk_vector4_is_normalized3(e: &mut Engine, this: Ptr, out: Ptr, epsilon: f32) -> Ptr {
    let finite = e.with_stack(4, |e, flag| {
        hk_vector4_is_ok3(e, this, flag);
        e.mem.u8(flag.addr()) != 0
    });
    let normalized = if finite {
        let squares = mul(load(e, this.addr()), load(e, this.addr()));
        close_to_one(e, squares[2] + (squares[1] + squares[0]), epsilon)
    } else {
        false
    };
    e.mem.set_u8(out.addr(), u8::from(normalized));
    out
}

// Translated from 00c7f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `out`, `epsilon`): `isNormalized3` over four
/// lanes. When `00c7f500` finds the lanes finite, the squared length is summed as the shuffles
/// add it, `(sq3 + sq1) + (sq2 + sq0)`, and tested like `isNormalized3`. Returns `out`.
pub fn fn_00c7f5b0(e: &mut Engine, this: Ptr, out: Ptr, epsilon: f32) -> Ptr {
    let finite = e.with_stack(4, |e, flag| {
        fn_00c7f500(e, this, flag);
        e.mem.u8(flag.addr()) != 0
    });
    let normalized = if finite {
        let squares = mul(load(e, this.addr()), load(e, this.addr()));
        let sum = (squares[3] + squares[1]) + (squares[2] + squares[0]);
        close_to_one(e, sum, epsilon)
    } else {
        false
    };
    e.mem.set_u8(out.addr(), u8::from(normalized));
    out
}

/// `SHUFPS dest, src, imm`: lanes 0 and 1 of the result are lanes of `dest` picked by bits 0-1 and
/// 2-3 of `imm`, lanes 2 and 3 lanes of `src` picked by bits 4-5 and 6-7.
fn shufps(dest: Lanes, src: Lanes, imm: u8) -> Lanes {
    let pick = |v: Lanes, shift: u8| v[usize::from((imm >> shift) & 3)];
    [pick(dest, 0), pick(dest, 2), pick(src, 4), pick(src, 6)]
}

/// `SUBPS a, b` (the SSE NaN rules).
fn sub(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::subps(a, b)
}

/// The four-lane quaternion rotation both `_ov2` functions end in, in the order of their
/// instructions: `2 * (w * cross + (w * w - 0.5) * v + dot * q)`, where `q` is the quaternion,
/// `w` its fourth lane and `dot` the sum `z + (y + x)` of the products `q * v`; `cross` is the
/// lane-wise `left - right` of the two shuffled products the caller built.
fn rotate_by_quaternion(
    e: &Engine,
    quaternion: Lanes,
    vector: Lanes,
    cross: Lanes,
    products: Lanes,
) -> Lanes {
    // `ADDSS` adds only lane 0; the other lanes keep the broadcast value.
    let mut y_sum = splat(products, 1);
    y_sum[0] = crate::sse::add(y_sum[0], products[0]);
    let mut dot = splat(products, 2);
    dot[0] = crate::sse::add(dot[0], y_sum[0]);
    let w = shufps(quaternion, quaternion, 0xff);
    let cross_w = mul(cross, w);
    let w_squared = mul(w, w);
    let half_offsets = load(e, 0x010c_71e0);
    let biased = add(w_squared, half_offsets);
    let along = mul(biased, vector);
    let dot_q = mul(splat(dot, 0), quaternion);
    let combined = add(along, dot_q);
    let sum = add(cross_w, combined);
    add(sum, sum)
}

// Translated from 00c7f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedPos_ov2` (engine map name; `thiscall`, two stack words
/// `transform`, `vector`, `RET 8`; the transform is an `hkQsTransform` as the offsets read it:
/// translation at `+0`, rotation quaternion at `+0x10`, scale at `+0x20`): `this = translation +
/// rotate(quaternion, scale * vector)`, all four lanes in `f32` in the order of the SSE
/// instructions (the rotation is `rotate_by_quaternion`; its constant vector is at `010c71e0`).
/// `this` is stored once before the translation is read and added, as the exe does.
pub fn fn_00c7f7a0(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let t = transform.addr();
    let quaternion = load(e, t.wrapping_add(0x10));
    let scale = load(e, t.wrapping_add(0x20));
    let v = load(e, vector.addr());
    let scaled = mul(scale, v);
    let products = mul(quaternion, scaled);
    let left = mul(
        shufps(quaternion, quaternion, 0xc9),
        shufps(scaled, scaled, 0xd2),
    );
    let right = mul(
        shufps(quaternion, quaternion, 0xd2),
        shufps(scaled, scaled, 0xc9),
    );
    let cross = sub(left, right);
    let rotated = rotate_by_quaternion(e, quaternion, scaled, cross, products);
    store(e, this.addr(), rotated);
    let translation = load(e, t);
    store(e, this.addr(), add(translation, rotated));
}

// Translated from 00c7f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkVector4::setTransformedInversePos_ov2` (engine map name; `thiscall`, two stack words
/// `transform`, `vector`, `RET 8`; an `hkQsTransform` as for `00c7f7a0`): `this = inverse
/// rotation of (vector - translation), divided by the scale`: the difference is rotated by the
/// conjugate quaternion (`rotate_by_quaternion` with the cross product of the other sign), stored
/// once, and then multiplied lane by lane by `2r - r * scale * r` where `r` is the `RCPPS`
/// estimate of the scale (a Newton step; the estimate is the host CPU's, so results can differ
/// in the last bits from another CPU's).
pub fn fn_00c7f840(e: &mut Engine, this: Ptr, transform: Ptr, vector: Ptr) {
    let t = transform.addr();
    let quaternion = load(e, t.wrapping_add(0x10));
    let translation = load(e, t);
    let difference = sub(load(e, vector.addr()), translation);
    let products = mul(quaternion, difference);
    let left = mul(
        shufps(quaternion, quaternion, 0xc9),
        shufps(difference, difference, 0xd2),
    );
    let right = mul(
        shufps(quaternion, quaternion, 0xd2),
        shufps(difference, difference, 0xc9),
    );
    let cross = sub(right, left);
    let rotated = rotate_by_quaternion(e, quaternion, difference, cross, products);
    store(e, this.addr(), rotated);
    let scale = load(e, t.wrapping_add(0x20));
    let estimate: Lanes =
        std::array::from_fn(|lane| crate::units::havok_sdk::hkaabbtree::rcp_estimate(scale[lane]));
    let estimate_times_scale = mul(estimate, scale);
    let twice = add(estimate, estimate);
    let refined_part = mul(estimate_times_scale, estimate);
    let refined = sub(twice, refined_part);
    store(e, this.addr(), mul(refined, rotated));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c7f7a0, fn_00c7f7a0(Ptr, Ptr, Ptr)),
        entry!(0x00c7f840, fn_00c7f840(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f640, set_transformed_pos(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f690, set_transformed_inverse_pos(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f700, set_rotated_dir(Ptr, Ptr, Ptr)),
        entry!(0x00c7_f750, set_mul3(Ptr, Ptr, Ptr)),
        entry!(0x00c7f4a0, fn_00c7f4a0(Ptr, i32, Ptr)),
        entry!(0x00c7f4e0, hk_vector4_is_ok3(Ptr, Ptr) -> Ptr),
        entry!(0x00c7f500, fn_00c7f500(Ptr, Ptr) -> Ptr),
        entry!(0x00c7f520, hk_vector4_is_normalized3(Ptr, Ptr, f32) -> Ptr),
        entry!(0x00c7f5b0, fn_00c7f5b0(Ptr, Ptr, f32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A transform (translation, quaternion, scale; 16-byte lanes), a vector and a result block,
    /// with the constant vector `010c71e0` set to four -0.5.
    fn qs_setup_00c7f7a0(
        translation: [f32; 4],
        quaternion: [f32; 4],
        scale: [f32; 4],
        vector: [f32; 4],
    ) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x010c_7000, 0x1000);
        for lane in 0..4 {
            e.mem.set_f32(0x010c_71e0 + 4 * lane, -0.5);
        }
        let transform = e.mem.alloc(0x40);
        let vector_at = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        for lane in 0..4usize {
            let at = 4 * lane as u32;
            e.mem.set_f32(transform + at, translation[lane]);
            e.mem.set_f32(transform + 0x10 + at, quaternion[lane]);
            e.mem.set_f32(transform + 0x20 + at, scale[lane]);
            e.mem.set_f32(vector_at + at, vector[lane]);
        }
        (e, transform, vector_at, out)
    }

    fn lanes_at_00c7f7a0(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }

    #[test]
    fn fn_00c7f7a0_with_the_identity_rotation_scales_and_translates() {
        let (mut e, transform, vector, out) = qs_setup_00c7f7a0(
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 2.0, 2.0, 1.0],
            [4.0, 5.0, 6.0, 0.0],
        );
        e.call(0x00c7_f7a0, &args![out, transform, vector]);
        assert_eq!(lanes_at_00c7f7a0(&e, out), [9.0, 12.0, 15.0, 0.0]);
    }

    #[test]
    fn fn_00c7f7a0_rotates_a_quarter_turn_about_z() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let (mut e, transform, vector, out) = qs_setup_00c7f7a0(
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, s, s],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 0.0, 0.0, 0.0],
        );
        e.call(0x00c7_f7a0, &args![out, transform, vector]);
        let got = lanes_at_00c7f7a0(&e, out);
        for (g, w) in got.iter().zip([1.0, 3.0, 3.0, 0.0]) {
            assert!((g - w).abs() < 1e-5, "{got:?}");
        }
    }

    #[test]
    fn fn_00c7f7a0_adds_the_translation_read_after_the_first_store() {
        // The result block is the translation itself: the first store overwrites it, so the
        // translation the exe adds is the rotated vector.
        let (mut e, transform, vector, _) = qs_setup_00c7f7a0(
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [4.0, 5.0, 6.0, 0.0],
        );
        e.call(0x00c7_f7a0, &args![transform, transform, vector]);
        assert_eq!(lanes_at_00c7f7a0(&e, transform), [8.0, 10.0, 12.0, 0.0]);
    }

    #[test]
    fn fn_00c7f840_undoes_the_transform_of_00c7f7a0() {
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let (mut e, transform, vector, out) = qs_setup_00c7f7a0(
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, s, s],
            [2.0, 2.0, 2.0, 1.0],
            [1.0, 4.0, 3.0, 0.0],
        );
        e.call(0x00c7_f840, &args![out, transform, vector]);
        let got = lanes_at_00c7f7a0(&e, out);
        for (g, w) in got.iter().zip([1.0, 0.0, 0.0, 0.0]) {
            assert!((g - w).abs() < 1e-4, "{got:?}");
        }
    }

    #[test]
    fn fn_00c7f840_with_the_identity_rotation_subtracts_and_divides() {
        let (mut e, transform, vector, out) = qs_setup_00c7f7a0(
            [1.0, 2.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [2.0, 4.0, 8.0, 1.0],
            [5.0, 10.0, 19.0, 0.0],
        );
        e.call(0x00c7_f840, &args![out, transform, vector]);
        let got = lanes_at_00c7f7a0(&e, out);
        for (g, w) in got.iter().zip([2.0, 2.0, 2.0, 0.0]) {
            assert!((g - w).abs() < 1e-4, "{got:?}");
        }
    }

    /// A 4x4 transform with distinct columns, a vector and an output block.
    fn setup(e: &mut Engine) -> (u32, u32, u32) {
        let transform = e.mem.alloc(0x40);
        let columns: [Lanes; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [0.5, 0.25, 0.125, 2.0],
        ];
        for (i, c) in columns.iter().enumerate() {
            store(e, transform + 16 * i as u32, *c);
        }
        let vector = e.mem.alloc(0x10);
        store(e, vector, [1.0, 2.0, 3.0, 9.0]);
        let out = e.mem.alloc(0x10);
        (transform, vector, out)
    }

    #[test]
    fn transformed_pos_adds_the_translation_column() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f640, &args![out, t, v]);
        // x*c0 + y*c1 + z*c2 + c3, per lane
        assert_eq!(load(&e, out), [38.5, 44.25, 50.125, 58.0]);
    }

    #[test]
    fn transformed_inverse_pos_uses_the_transposed_rows() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f690, &args![out, t, v]);
        // d = vector - translation; the rows as the shuffles leave them are (1,5,9,10),
        // (2,6,10,12), (3,7,11,12): lane 3 of rows 0 and 1 is column2.y and column2.w.
        let (dx, dy, dz) = (0.5f32, 1.75f32, 2.875f32);
        let want = [
            dy * 2.0 + dx * 1.0 + dz * 3.0,
            dy * 6.0 + dx * 5.0 + dz * 7.0,
            dy * 10.0 + dx * 9.0 + dz * 11.0,
            dy * 12.0 + dx * 10.0 + dz * 12.0,
        ];
        assert_eq!(load(&e, out), want);
    }

    #[test]
    fn rotated_dir_and_mul3_ignore_the_translation() {
        let mut e = Engine::new();
        let (t, v, out) = setup(&mut e);
        e.call(0x00c7_f700, &args![out, t, v]);
        assert_eq!(load(&e, out), [38.0, 44.0, 50.0, 58.0 - 2.0]);
        let out2 = e.mem.alloc(0x10);
        e.call(0x00c7_f750, &args![out2, t, v]);
        assert_eq!(load(&e, out2), load(&e, out));
    }

    #[test]
    fn the_finite_check_stops_at_the_first_infinity_or_nan() {
        let mut e = Engine::new();
        let floats = e.mem.alloc(0x10);
        store(&mut e, floats, [1.0, -2.0, 3.0, 4.0]);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xcdcd_cdcd);
        e.call(0x00c7_f4a0, &args![out, 4i32, floats]);
        assert_eq!(e.mem.u32(out), 0xcdcd_cd01, "only the low byte is written");
        e.mem.set_f32(floats + 8, f32::INFINITY);
        e.call(0x00c7_f4a0, &args![out, 4i32, floats]);
        assert_eq!(e.mem.u8(out), 0);
        // Only the first `count` floats are looked at.
        e.call(0x00c7_f4a0, &args![out, 2i32, floats]);
        assert_eq!(e.mem.u8(out), 1);
        // A signalling NaN is a NaN too; a count of zero or less is fine.
        e.mem.set_u32(floats, 0x7f80_0001);
        e.call(0x00c7_f4a0, &args![out, 1i32, floats]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00c7_f4a0, &args![out, 0i32, floats]);
        assert_eq!(e.mem.u8(out), 1);
        e.mem.set_u8(out, 0);
        e.call(0x00c7_f4a0, &args![out, -3i32, floats]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn is_ok3_and_the_four_lane_form_differ_in_the_last_lane() {
        let mut e = Engine::new();
        let vector = e.mem.alloc(0x10);
        store(&mut e, vector, [1.0, 2.0, 3.0, f32::NAN]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00c7_f4e0, &args![vector, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.call(0x00c7_f500, &args![vector, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn normalized_checks_the_squared_length_against_one() {
        let mut e = Engine::new();
        e.map(0x0101_2070, 8);
        e.mem.set_f64(0x0101_2070, 1.0);
        let vector = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        let check = |e: &mut Engine, addr: u32, v: Lanes, epsilon: f32| {
            store(e, vector, v);
            assert_eq!(e.call(addr, &args![vector, out, epsilon]).u32(), out);
            e.mem.u8(out)
        };
        // Three lanes: the fourth is ignored.
        assert_eq!(check(&mut e, 0x00c7_f520, [1.0, 0.0, 0.0, 5.0], 0.01), 1);
        assert_eq!(check(&mut e, 0x00c7_f520, [2.0, 0.0, 0.0, 0.0], 0.01), 0);
        assert_eq!(check(&mut e, 0x00c7_f520, [0.5, 0.5, 0.5, 0.5], 0.1), 0);
        assert_eq!(check(&mut e, 0x00c7_f520, [0.5, 0.5, 0.5, 0.5], 0.3), 1);
        // Exactly epsilon away is not below it.
        assert_eq!(check(&mut e, 0x00c7_f520, [0.5, 0.5, 0.5, 0.0], 0.25), 0);
        assert_eq!(check(&mut e, 0x00c7_f520, [0.5, 0.5, 0.5, 0.0], 0.3), 1);
        // A lane that is not finite (or a NaN epsilon) gives 0.
        assert_eq!(
            check(&mut e, 0x00c7_f520, [f32::INFINITY, 0.0, 0.0, 0.0], 1.0e9),
            0
        );
        assert_eq!(
            check(&mut e, 0x00c7_f520, [1.0, 0.0, 0.0, 0.0], f32::NAN),
            0
        );
        // Four lanes: (0.5, 0.5, 0.5, 0.5) is a unit vector, a NaN fourth lane is not ok.
        assert_eq!(check(&mut e, 0x00c7_f5b0, [0.5, 0.5, 0.5, 0.5], 0.01), 1);
        assert_eq!(check(&mut e, 0x00c7_f5b0, [1.0, 0.0, 0.0, 1.0], 0.5), 0);
        assert_eq!(
            check(&mut e, 0x00c7_f5b0, [1.0, 0.0, 0.0, f32::NAN], 0.5),
            0
        );
    }
}
