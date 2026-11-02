//! `NiMain/run_00a88c60` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Three functions of a texture transform. The class has no vtable to name it; its fields as
//! the PC code uses them match the Xbox PDB's `NiTextureTransform` up to the matrix
//! (translate +0, rotate +8, scale +0xc, center +0x14, matrix-dirty flag +0x1c, a 3x3 float
//! matrix at +0x20); the method is at +0x44 on PC (+0x50 in the PDB, whose matrix is
//! padded). The class name comes from that match only.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// The texture transform as the PC code uses it (field names from the Xbox PDB's
    /// `NiTextureTransform`, see the module comment).
    pub struct TextureTransform: 0x48 {
        /// `m_kTranslate` (Xbox PDB), x then y.
        0x00 m_kTranslate: u32,
        /// `m_fRotate` (Xbox PDB).
        0x08 m_fRotate: u32,
        /// `m_kScale` (Xbox PDB), x then y.
        0x0C m_kScale: u32,
        /// `m_kCenter` (Xbox PDB), x then y.
        0x14 m_kCenter: u32,
        /// `m_bMatrixDirty` (Xbox PDB).
        0x1C m_bMatrixDirty: u8,
        /// `m_kMatrix` (Xbox PDB): nine floats, row by row.
        0x20 m_kMatrix: u32,
        /// `m_eMethod` (Xbox PDB; PC offset).
        0x44 m_eMethod: u32,
    }
}

/// `_ftol2` (`00ec62c0`): the value in ST0 as an integer in EAX (a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// The `float` 2π (`6.2831855`, `.data`).
const TWO_PI: u32 = 0x011a_b590;
/// The `double` 512.0: entries per turn of the two angle tables.
const TABLE_ENTRIES_PER_TURN: u32 = 0x0102_8338;
/// The 512-entry `float` table read for the first value (`A` below).
const TABLE_A: u32 = 0x011f_52e0;
/// The 512-entry `float` table read for the second value (`B` below).
const TABLE_B: u32 = 0x011f_4ae0;
/// The point loader (`00a68c80`, thiscall on the point, the stream on the stack).
const POINT_LOAD: u32 = 0x00a6_8c80;

fn field(e: &Engine, base: u32, offset: u32) -> f64 {
    x87_m32(e.mem.u32(base + offset))
}

fn load(e: &Engine, base: u32, offset: u32) -> f64 {
    x87_load32(e.mem.u32(base + offset))
}

fn store(e: &mut Engine, base: u32, offset: u32, value: f64) {
    e.mem.set_u32(base + offset, x87_store32(value));
}

// Translated from 00a88cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the transform's matrix. The angle index is
/// `_ftol2(512.0 / 2π * m_fRotate) & 0x1ff` (`FDIVR` of the double by the float, `FMUL` by
/// the rotation); `A` and `B` are the entries of the tables at `011f52e0` and `011f4ae0` (kept
/// as floats). With `d`, `e` as below, by method (+0x44, an integer):
/// - 0: rows `(sx·B, -A·sy, (e·B + -A·d) + cx)`, `(sx·A, sy·B, ((B·d) + A·e) + cy)`, with
///   `d = ty - cy`, `e = tx - cx`;
/// - 1: rows `(sx·B, sx·A, ((A·d + B·f)·sx) + cx)`, `(-A·sy, sy·B, ((B·d - A·f)·sy) + cy)`,
///   with `d = ty - cy`, `f = -cx - tx`;
/// - 2: rows `(sx·B, -A·sy, (A·g + B·(tx - cx)) + cx)`, `(-A·sx, -B·sy, ((cx - tx)·A + B·g) +
///   cy)`, with `g = 1 - (ty + cy)`;
///
/// and the third row `(0, 0, 1)`. Any other method leaves the matrix alone. Each product and
/// sum is one x87 instruction in the exe's operand order; every stored element is rounded to
/// a float, the intermediates are not. Clears `m_bMatrixDirty` in every case.
pub fn fn_00a88cd0(e: &mut Engine, this: Ptr<TextureTransform>) {
    let base = this.addr();
    let per_turn = x87_div(
        e.global::<f64>(TABLE_ENTRIES_PER_TURN),
        x87_load32(e.mem.u32(TWO_PI)),
    );
    let angle = x87_mul(per_turn, field(e, base, 0x08));
    let index = e.call(FTOL, &args![angle]).u32() & 0x1ff;
    let a_bits = x87_quiet_bits(e.mem.u32(TABLE_A + index * 4));
    let b_bits = x87_quiet_bits(e.mem.u32(TABLE_B + index * 4));
    let a = x87_load32(a_bits);
    let b = x87_load32(b_bits);
    let method = e.get(this, TextureTransform::m_eMethod);
    let (m02, m12) = match method {
        0 => {
            let sx = load(e, base, 0x0c);
            store(e, base, 0x2c, x87_mul(sx, a));
            store(e, base, 0x20, x87_mul(sx, b));
            store(e, base, 0x38, 0.0);
            let sy = load(e, base, 0x10);
            let m11 = x87_mul(sy, b);
            let minus_a = -a;
            store(e, base, 0x24, x87_mul(minus_a, field(e, base, 0x10)));
            store(e, base, 0x30, m11);
            store(e, base, 0x3c, 0.0);
            let d = x87_sub(load(e, base, 0x04), field(e, base, 0x18));
            let ex = x87_sub(load(e, base, 0x00), field(e, base, 0x14));
            let b_d = x87_mul(b, d);
            let a_e = x87_mul(a, ex);
            let sum = x87_add(b_d, a_e);
            let m12 = x87_add(sum, load(e, base, 0x18));
            let minus_a_d = x87_mul(minus_a, d);
            let e_b = x87_mul(ex, b);
            (x87_add(e_b, minus_a_d), m12)
        }
        1 => {
            let m10 = x87_mul(-a, field(e, base, 0x10));
            let sx = load(e, base, 0x0c);
            store(e, base, 0x20, x87_mul(sx, b));
            store(e, base, 0x2c, m10);
            store(e, base, 0x38, 0.0);
            let m11 = x87_mul(load(e, base, 0x10), b);
            store(e, base, 0x24, x87_mul(load(e, base, 0x0c), a));
            store(e, base, 0x30, m11);
            store(e, base, 0x3c, 0.0);
            let d = x87_sub(load(e, base, 0x04), field(e, base, 0x18));
            let f = x87_sub(-load(e, base, 0x14), field(e, base, 0x00));
            let b_d = x87_mul(b, d);
            let a_f = x87_mul(a, f);
            let difference = x87_sub(b_d, a_f);
            let scaled = x87_mul(difference, field(e, base, 0x10));
            let m12 = x87_add(scaled, field(e, base, 0x18));
            let b_f = x87_mul(b, f);
            let a_d = x87_mul(a, d);
            let sum = x87_add(a_d, b_f);
            (x87_mul(sum, field(e, base, 0x0c)), m12)
        }
        2 => {
            let minus_a = -a;
            let m10 = x87_mul(minus_a, field(e, base, 0x0c));
            let sx = load(e, base, 0x0c);
            store(e, base, 0x20, x87_mul(sx, b));
            store(e, base, 0x2c, m10);
            store(e, base, 0x38, 0.0);
            let m11 = x87_mul(-b, field(e, base, 0x10));
            let m01 = x87_mul(minus_a, load(e, base, 0x10));
            store(e, base, 0x24, m01);
            store(e, base, 0x30, m11);
            store(e, base, 0x3c, 0.0);
            let ty_cy = x87_add(load(e, base, 0x04), field(e, base, 0x18));
            let g = x87_sub(1.0, ty_cy);
            let h = x87_sub(load(e, base, 0x14), field(e, base, 0x00));
            let h_a = x87_mul(h, a);
            let b_g = x87_mul(b, g);
            let sum = x87_add(h_a, b_g);
            let m12 = x87_add(sum, field(e, base, 0x18));
            let tx_cx = x87_sub(load(e, base, 0x00), field(e, base, 0x14));
            let b_t = x87_mul(b, tx_cx);
            let a_g = x87_mul(a, g);
            (x87_add(a_g, b_t), m12)
        }
        _ => {
            e.set(this, TextureTransform::m_bMatrixDirty, 0);
            return;
        }
    };
    let m02 = x87_add(m02, field(e, base, 0x14));
    store(e, base, 0x28, m02);
    store(e, base, 0x34, m12);
    store(e, base, 0x40, 1.0);
    e.set(this, TextureTransform::m_bMatrixDirty, 0);
}

// Translated from 00a88eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor from a translation, a rotation, a scale, a center (the points by pointer, two
/// words each, copied as words; the rotation through `FLD`/`FSTP`) and the method; then
/// builds the matrix (`00a88cd0`). Returns `this`.
pub fn fn_00a88eb0(
    e: &mut Engine,
    this: Ptr<TextureTransform>,
    translate: Ptr,
    rotate: f32,
    scale: Ptr,
    center: Ptr,
    method: u32,
) -> Ptr<TextureTransform> {
    let base = this.addr();
    let word = e.mem.u32(translate.addr());
    e.mem.set_u32(base, word);
    let second = e.mem.u32(translate.addr() + 4);
    e.mem.set_u32(base + 8, x87_quiet_bits(rotate.to_bits()));
    e.mem.set_u32(base + 4, second);
    for (point, offset) in [(scale, 0x0cu32), (center, 0x14)] {
        let first = e.mem.u32(point.addr());
        e.mem.set_u32(base + offset, first);
        let second = e.mem.u32(point.addr() + 4);
        e.mem.set_u32(base + offset + 4, second);
    }
    e.set(this, TextureTransform::m_eMethod, method);
    fn_00a88cd0(e, this);
    this
}

// Translated from 00a88f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the transform from a stream (the `LoadBinary` at `00a6b81c` calls it): the
/// translation and scale through the point loader (`00a68c80`), then two 4-byte reads through
/// the function pointer at +8 of the stream's file object (`stream` +0x24c), each called as
/// `(file, buffer, 4, &size, 1)` with `size` a word set to 4 and its result added to the
/// file's word at +4: the rotation into +8, then the method into a local whose first value is
/// `this` (the pushed `ECX` it occupies), stored at +0x44; then the center through the point
/// loader, and sets `m_bMatrixDirty`.
pub fn fn_00a88f00(e: &mut Engine, this: Ptr<TextureTransform>, stream: Ptr) {
    let base = this.addr();
    e.call(POINT_LOAD, &args![base, stream]);
    e.call(POINT_LOAD, &args![base + 0x0c, stream]);
    e.with_stack(8, |e, locals| {
        let size = locals.addr();
        let method = locals.addr() + 4;
        e.mem.set_u32(method, base);
        for buffer in [base + 8, method] {
            let file = e.mem.u32(stream.addr() + 0x24c);
            let read = e.mem.u32(file + 8);
            e.mem.set_u32(size, 4);
            let count = e.call(read, &args![file, buffer, 4u32, size, 1u32]).u32();
            let position = e.mem.u32(file + 4);
            e.mem.set_u32(file + 4, position.wrapping_add(count));
        }
        let value = e.mem.u32(method);
        e.set(this, TextureTransform::m_eMethod, value);
    });
    e.call(POINT_LOAD, &args![base + 0x14, stream]);
    e.set(this, TextureTransform::m_bMatrixDirty, 1);
}

/// The two-float point comparison (`0051c330`, `thiscall`, the other point; the engine map
/// places it in `fallout shared/tesobjecttree.cpp`, folded code): true when they differ.
const POINT_DIFFERS: u32 = 0x0051_c330;
/// The point saver (`00a68ce0`, `thiscall` on the point, the stream on the stack).
const POINT_SAVE: u32 = 0x00a6_8ce0;

// Translated from 00a88c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The transforms' inequality (`thiscall`, the other transform; `RET 4`): 1 as soon as the
/// rotation, the translation's x or its y differ (`FUCOMPP`: a NaN differs from
/// everything), the scale points differ (`0051c330`), the methods differ, or the center
/// points differ (`0051c330`); otherwise 0.
pub fn fn_00a88c60(
    e: &mut Engine,
    this: Ptr<TextureTransform>,
    other: Ptr<TextureTransform>,
) -> u32 {
    let base = this.addr();
    let other_base = other.addr();
    for offset in [8u32, 0, 4] {
        if load(e, base, offset) != load(e, other_base, offset) {
            return 1;
        }
    }
    if e.call(POINT_DIFFERS, &args![base + 0xc, other_base + 0xc])
        .u8()
        != 0
    {
        return 1;
    }
    if e.get(this, TextureTransform::m_eMethod) != e.get(other, TextureTransform::m_eMethod) {
        return 1;
    }
    if e.call(POINT_DIFFERS, &args![base + 0x14, other_base + 0x14])
        .u8()
        != 0
    {
        return 1;
    }
    0
}

// Translated from 00a88f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the transform to a stream (the counterpart of [`fn_00a88f00`]; `thiscall`, the
/// stream; `RET 4`): the translation and scale through the point saver (`00a68ce0`), then
/// the rotation (+8) and the method (+0x44), 4 bytes each, through the stream output's write
/// function (`NiStream` +0x250, the function +0xc, the position +4 advanced by its answer;
/// the byte count in the stream argument's own word), then the center through the point
/// saver.
pub fn fn_00a88f80(e: &mut Engine, this: Ptr<TextureTransform>, stream: Ptr) {
    let base = this.addr();
    e.call(POINT_SAVE, &args![base, stream]);
    e.call(POINT_SAVE, &args![base + 0x0c, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        e.mem.set_u32(count, stream.addr());
        for source in [base + 8, base + 0x44] {
            let output = e.mem.u32(stream.addr() + 0x250);
            let write = e.mem.u32(output + 0xc);
            e.mem.set_u32(count, 4);
            let done = e
                .call(write, &args![output, source, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(output + 4);
            e.mem.set_u32(output + 4, position.wrapping_add(done));
        }
    });
    e.call(POINT_SAVE, &args![base + 0x14, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_8cd0, fn_00a88cd0(Ptr<TextureTransform>)),
        entry!(
            0x00a8_8eb0,
            fn_00a88eb0(Ptr<TextureTransform>, Ptr, f32, Ptr, Ptr, u32) -> Ptr<TextureTransform>
        ),
        entry!(0x00a8_8f00, fn_00a88f00(Ptr<TextureTransform>, Ptr)),
        entry!(
            0x00a8_8c60,
            fn_00a88c60(Ptr<TextureTransform>, Ptr<TextureTransform>) -> u32
        ),
        entry!(0x00a8_8f80, fn_00a88f80(Ptr<TextureTransform>, Ptr)),
    ]
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    const INDEX: u32 = 0x123;
    const A: f32 = 0.5;
    const B: f32 = 0.25;

    /// The constants and tables the matrix update reads, and an `_ftol2` double returning
    /// `INDEX + 0x200` (the mask keeps the low 9 bits).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(TWO_PI, 4);
        e.mem.set_u32(TWO_PI, 0x40c9_0fdb);
        e.map(TABLE_ENTRIES_PER_TURN, 8);
        e.set_global::<f64>(TABLE_ENTRIES_PER_TURN, 512.0);
        e.map(TABLE_B, 0x1000);
        e.mem.set_u32(TABLE_A + INDEX * 4, A.to_bits());
        e.mem.set_u32(TABLE_B + INDEX * 4, B.to_bits());
        e.register(FTOL, |_, _| (INDEX + 0x200).into_ret());
        e
    }

    /// tx, ty, rotate, sx, sy, cx, cy.
    fn transform(e: &mut Engine, method: u32) -> u32 {
        let this = e.mem.alloc(0x48);
        let values = [3.0f32, 5.0, 1.0, 2.0, 4.0, 1.0, 2.0];
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(this + 4 * i as u32, v.to_bits());
        }
        e.mem.set_u8(this + 0x1c, 1);
        e.mem.set_u32(this + 0x44, method);
        this
    }

    fn matrix(e: &Engine, this: u32) -> Vec<f32> {
        (0..9)
            .map(|i| f32::from_bits(e.mem.u32(this + 0x20 + 4 * i)))
            .collect()
    }

    #[test]
    fn matrix_update_passes_the_scaled_rotation_to_ftol() {
        let mut e = engine();
        let this = transform(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_8cd0, &args![this]);
        let ftol: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == FTOL)
            .collect();
        let angle = x87_mul(x87_div(512.0, x87_load32(0x40c9_0fdb)), 1.0);
        assert_eq!(ftol, vec![(FTOL, args![angle].to_vec())]);
    }

    #[test]
    fn matrix_update_for_each_method() {
        // tx 3, ty 5, sx 2, sy 4, cx 1, cy 2, A 0.5, B 0.25.
        let (tx, ty, sx, sy, cx, cy) = (3.0f32, 5.0f32, 2.0f32, 4.0f32, 1.0f32, 2.0f32);
        let (a, b) = (A, B);
        let cases = [
            (
                0u32,
                vec![
                    sx * b,
                    -a * sy,
                    (tx - cx) * b + -a * (ty - cy) + cx,
                    sx * a,
                    sy * b,
                    b * (ty - cy) + a * (tx - cx) + cy,
                ],
            ),
            (
                1,
                vec![
                    sx * b,
                    sx * a,
                    (a * (ty - cy) + b * (-cx - tx)) * sx + cx,
                    -a * sy,
                    sy * b,
                    (b * (ty - cy) - a * (-cx - tx)) * sy + cy,
                ],
            ),
            (
                2,
                vec![
                    sx * b,
                    -a * sy,
                    a * (1.0 - (ty + cy)) + b * (tx - cx) + cx,
                    -a * sx,
                    -b * sy,
                    (cx - tx) * a + b * (1.0 - (ty + cy)) + cy,
                ],
            ),
        ];
        for (method, rows) in cases {
            let mut e = engine();
            let this = transform(&mut e, method);
            e.call(0x00a8_8cd0, &args![this]);
            let mut want = rows;
            want.extend([0.0, 0.0, 1.0]);
            assert_eq!(matrix(&e, this), want, "method {method}");
            assert_eq!(e.mem.u8(this + 0x1c), 0);
        }
    }

    #[test]
    fn matrix_update_with_another_method_only_clears_the_flag() {
        let mut e = engine();
        let this = transform(&mut e, 3);
        e.mem.set_u32(this + 0x20, 0x1234_5678);
        e.call(0x00a8_8cd0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x20), 0x1234_5678);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u8(this + 0x1c), 0);
    }

    #[test]
    fn constructor_copies_the_values_and_builds_the_matrix() {
        let mut e = engine();
        let points = e.mem.alloc(0x18);
        for (i, v) in [3.0f32, 5.0, 2.0, 4.0, 1.0, 2.0].iter().enumerate() {
            e.mem.set_u32(points + 4 * i as u32, v.to_bits());
        }
        let this = e.mem.alloc(0x48);
        let r = e.call(
            0x00a8_8eb0,
            &[this, points, 0x7f80_0001, points + 8, points + 0x10, 0],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this), 3.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x18), 2.0f32.to_bits());
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(matrix(&e, this)[0], 2.0 * B);
        assert_eq!(e.mem.u8(this + 0x1c), 0);
    }

    #[test]
    fn load_reads_the_points_rotation_and_method() {
        let mut e = Engine::new();
        e.register(POINT_LOAD, |_, _| Ret::default());
        e.register(0x00aa_0005, |e, a| {
            assert_eq!(a[2], 4);
            assert_eq!(e.mem.u32(a[3]), 4);
            assert_eq!(a[4], 1);
            // The method's local starts out holding `this`.
            let previous = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], previous ^ 0x0f0f);
            3u32.into_ret()
        });
        let stream = e.mem.alloc(0x250);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(file + 4, 100);
        e.mem.set_u32(file + 8, 0x00aa_0005);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 8, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x00a8_8f00, &args![this, stream]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == POINT_LOAD)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            log,
            vec![
                vec![this, stream],
                vec![this + 0x0c, stream],
                vec![this + 0x14, stream]
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 0x1000 ^ 0x0f0f);
        assert_eq!(e.mem.u32(this + 0x44), this ^ 0x0f0f);
        assert_eq!(e.mem.u32(file + 4), 106);
        assert_eq!(e.mem.u8(this + 0x1c), 1);
    }
}

#[cfg(test)]
mod compare_and_save_tests {
    use super::*;

    fn transform(e: &mut Engine) -> u32 {
        let t = e.mem.alloc(0x48);
        for (offset, value) in [(0u32, 1.0f32), (4, 2.0), (8, 0.5)] {
            e.mem.set_f32(t + offset, value);
        }
        e.mem.set_u32(t + 0x44, 1);
        t
    }

    #[test]
    fn differs_on_any_field() {
        let mut e = Engine::new();
        e.register(POINT_DIFFERS, |_, _| 0x100u32.into_ret());
        let a = transform(&mut e);
        let b = transform(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_8c60, &args![a, b]).u32(), 0);
        let points: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(f, _)| *f == POINT_DIFFERS)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            points,
            vec![vec![a + 0xc, b + 0xc], vec![a + 0x14, b + 0x14]]
        );
        for offset in [0u32, 4, 8] {
            let b = transform(&mut e);
            e.mem.set_f32(b + offset, 3.0);
            assert_eq!(e.call(0x00a8_8c60, &args![a, b]).u32(), 1);
            e.mem.set_f32(b + offset, f32::NAN);
            e.mem.set_f32(a + offset, f32::NAN);
            assert_eq!(e.call(0x00a8_8c60, &args![a, b]).u32(), 1);
            let a2 = transform(&mut e);
            e.mem.set_u32(a + offset, e.mem.u32(a2 + offset));
        }
        let c = transform(&mut e);
        e.mem.set_u32(c + 0x44, 2);
        assert_eq!(e.call(0x00a8_8c60, &args![a, c]).u32(), 1);
        e.register(POINT_DIFFERS, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00a8_8c60, &args![a, b]).u32(), 1);
    }

    #[test]
    fn save_writes_points_rotation_and_method() {
        let mut e = Engine::new();
        e.register(POINT_SAVE, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        let this = transform(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a8_8f80, &args![this, stream]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .skip(1)
            .map(|(f, w)| (f, w[1]))
            .collect();
        assert_eq!(
            log,
            vec![
                (POINT_SAVE, stream),
                (POINT_SAVE, stream),
                (0x0300_0000, this + 8),
                (0x0300_0000, this + 0x44),
                (POINT_SAVE, stream),
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 8);
    }
}
