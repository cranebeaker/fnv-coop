//! `Havok SDK/run_00de87c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Operations on 4x4 float matrices (the engine map has no names; `hkMatrix4`'s operations by
//! their shape) and two plane intersection helpers. A matrix is 16 floats, four rows of four
//! (`hkVector4`, 16 bytes), element `(row, column)` at `row * 16 + column * 4`. The SSE parts
//! compute lane by lane in `f32`; the x87 parts (the copies, the transposition, the inversion)
//! pass every float through the x87 stack: `FLD`/`FSTP` quiets a signalling NaN, products and
//! sums are `f64` results rounded to `float` at each store.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMatrix3::invert` (`00cd4f10`, `this` = the matrix of three rows, one float word:
/// the tolerance; answers 0 on success).
const MATRIX3_INVERT: u32 = 0x00cd_4f10;
/// The float `1.19e-7` (`0104da4c`) the inverting functions pass as their tolerance, the zero
/// (`01011d78`) and the zero vector (`010c7160`) of the plane helpers.
const TOLERANCE: u32 = 0x0104_da4c;
const ZERO: u32 = 0x0101_1d78;
const CONSTANT_VECTOR: u32 = 0x010c_7160;

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, at: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(at.wrapping_add(4 * lane as u32)))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, at: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(at.wrapping_add(4 * lane as u32), *value);
    }
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// `SHUFPS v, v, 0`: lane 0 in all four.
fn broadcast(value: f32) -> Lanes {
    [value; 4]
}

/// `FLD dword` then `FSTP dword` from `from` to `to`: the float's bits, a signalling NaN made
/// quiet.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// The sum of the three lowest lanes as two `ADDSS` over shuffled copies form it: `v2 + (v1 +
/// v0)`.
fn lane_sum(v: Lanes) -> f32 {
    v[2] + (v[1] + v[0])
}

// Translated from 00de87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transposes the 4x4 matrix at `this` in place (the engine map has no name), thiscall: swaps
/// the elements `(0, 1)` with `(1, 0)`, `(0, 2)` with `(2, 0)`, `(0, 3)` with `(3, 0)`, `(1, 2)`
/// with `(2, 1)`, `(1, 3)` with `(3, 1)` and `(2, 3)` with `(3, 2)`, each through the x87
/// stack (so both come out quiet) and a stack word.
pub fn fn_00de87c0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    for (low, high) in [
        (0x04, 0x10),
        (0x08, 0x20),
        (0x0c, 0x30),
        (0x18, 0x24),
        (0x1c, 0x34),
        (0x2c, 0x38),
    ] {
        // the exe swaps `[me + high]` with `[me + low]` through a stack word:
        // `tmp = high; high = low; low = tmp`
        let temporary = x87_quiet_bits(e.mem.u32(me.wrapping_add(high)));
        copy_float(e, me.wrapping_add(low), me.wrapping_add(high));
        e.mem
            .set_u32(me.wrapping_add(low), x87_quiet_bits(temporary));
    }
}

// Translated from 00de88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales a matrix by a scalar (the engine map has no name), thiscall with two words
/// `(scalar, matrix)`: row by row, the first lane of the vector at `scalar` (re-read for every
/// row, so a `scalar` inside `this` changes under it) broadcast and multiplied by the row of
/// `matrix`, the result stored in the row of `this`.
pub fn fn_00de88a0(e: &mut Engine, this: Ptr, scalar: Ptr, matrix: Ptr) {
    for row in 0..4u32 {
        let factor = load(e, scalar.addr())[0];
        let values = load(e, matrix.addr().wrapping_add(0x10 * row));
        store(
            e,
            this.addr().wrapping_add(0x10 * row),
            crate::sse::mulps(broadcast(factor), values),
        );
    }
}

// Translated from 00de8910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inverts the 4x4 matrix at `this` in place (the engine map has no name; the shape of
/// `hkMatrix4::invert`), thiscall with the tolerance as one float word. The twelve 2x2
/// minors of the matrix are formed in the x87 (each `float` stored), the determinant `det` as
/// their sum of products. When `|det| <= tolerance` (a NaN determinant or tolerance goes the
/// other way) the matrix is left alone and 1 returned. Otherwise the sixteen cofactors are
/// formed in the x87 (each stored as a float) and the result is `float(1 / det)` times the
/// cofactor matrix, stored through `00de88a0`; the answer is 0.
pub fn fn_00de8910(e: &mut Engine, this: Ptr, tolerance: f32) -> u32 {
    let base = this.addr();
    let m = |e: &Engine, index: u32| e.mem.u32(base.wrapping_add(4 * index));
    // the 2x2 minors, as stored floats
    let minor = |e: &Engine, a: u32, b: u32, c: u32, d: u32| -> u32 {
        x87_store32(x87_sub(
            x87_mul(x87_load32(m(e, a)), x87_m32(m(e, b))),
            x87_mul(x87_load32(m(e, c)), x87_m32(m(e, d))),
        ))
    };
    let t20 = minor(e, 0, 5, 1, 4);
    let t28 = minor(e, 9, 0, 1, 8);
    let t18 = minor(e, 0, 13, 1, 12);
    let t14 = minor(e, 9, 4, 8, 5);
    let t10 = minor(e, 13, 4, 12, 5);
    let t0c = minor(e, 8, 13, 12, 9);
    let t08 = minor(e, 7, 2, 6, 3);
    let t00 = minor(e, 11, 2, 3, 10);
    let t04 = minor(e, 15, 2, 14, 3);
    let t2c = minor(e, 11, 6, 7, 10);
    let t24 = minor(e, 15, 6, 14, 7);
    let t1c = minor(e, 15, 10, 14, 11);
    // the names of the x87 stack values and stored temporaries the exe uses
    let (a, b, c, d, ee) = (t1c, t20, t24, t28, t2c);
    let (f, g, h, i, j, k, l) = (t18, t04, t14, t00, t10, t08, t0c);
    let fl = x87_load32;
    let qm = x87_m32;
    // the determinant
    let ab = x87_mul(fl(a), fl(b));
    let cd = x87_mul(fl(c), fl(d));
    let x1 = x87_sub(ab, cd);
    let x2 = x87_add(x1, x87_mul(fl(ee), qm(f)));
    let x3 = x87_add(x2, x87_mul(fl(g), qm(h)));
    let x4 = x87_sub(x3, x87_mul(fl(i), qm(j)));
    let x5 = x87_add(x4, x87_mul(fl(k), qm(l)));
    let det_bits = x87_store32(x5);
    let det = fl(det_bits);
    let reciprocal_bits = x87_store32(x87_div(1.0, det));
    let magnitude = fl(x87_store32(det.abs()));
    let epsilon = fl(tolerance.to_bits());
    if epsilon >= magnitude {
        return 1;
    }
    let n = |index: u32| m(e, index);
    let out: [u32; 16] = [
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(a), qm(n(5))), x87_mul(fl(n(9)), fl(c))),
            x87_mul(fl(ee), qm(n(13))),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(n(9)), qm(g)), x87_mul(fl(n(1)), fl(a))),
            x87_mul(fl(i), qm(n(13))),
        )),
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(n(1)), fl(c)), x87_mul(fl(g), qm(n(5)))),
            x87_mul(fl(k), qm(n(13))),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(i), qm(n(5))), x87_mul(fl(n(1)), fl(ee))),
            x87_mul(fl(n(9)), qm(k)),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(n(8)), fl(c)), x87_mul(fl(a), qm(n(4)))),
            x87_mul(fl(n(12)), fl(ee)),
        )),
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(a), fl(n(0))), x87_mul(fl(n(8)), fl(g))),
            x87_mul(fl(n(12)), qm(i)),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(g), fl(n(4))), x87_mul(fl(c), fl(n(0)))),
            x87_mul(fl(n(12)), fl(k)),
        )),
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(ee), fl(n(0))), x87_mul(fl(i), qm(n(4)))),
            x87_mul(fl(k), qm(n(8))),
        )),
        x87_store32(x87_add(
            x87_mul(fl(n(15)), fl(h)),
            x87_sub(x87_mul(fl(n(7)), fl(l)), x87_mul(fl(n(11)), fl(j))),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(n(11)), fl(f)), x87_mul(fl(l), fl(n(3)))),
            x87_mul(fl(n(15)), fl(d)),
        )),
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(n(3)), fl(j)), x87_mul(fl(n(7)), fl(f))),
            x87_mul(fl(n(15)), fl(b)),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(n(7)), fl(d)), x87_mul(fl(n(3)), fl(h))),
            x87_mul(fl(n(11)), fl(b)),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(j), qm(n(10))), x87_mul(fl(n(6)), qm(l))),
            x87_mul(fl(n(14)), fl(h)),
        )),
        x87_store32(x87_add(
            x87_sub(x87_mul(fl(n(2)), qm(l)), x87_mul(fl(f), qm(n(10)))),
            x87_mul(fl(n(14)), fl(d)),
        )),
        x87_store32(x87_sub(
            x87_sub(x87_mul(fl(f), fl(n(6))), x87_mul(fl(j), fl(n(2)))),
            x87_mul(fl(n(14)), fl(b)),
        )),
        x87_store32(x87_add(
            x87_mul(fl(b), fl(n(10))),
            x87_sub(x87_mul(fl(h), qm(n(2))), x87_mul(fl(d), fl(n(6)))),
        )),
    ];
    // the cofactors and the reciprocal go to stack memory, then `00de88a0` scales them into
    // the matrix
    e.with_stack(0x80, |e, frame| {
        let scalar = frame.addr() + 0x30;
        let cofactors = frame.addr() + 0x40;
        store(e, scalar, [f32::from_bits(reciprocal_bits), 0.0, 0.0, 0.0]);
        for (index, bits) in out.iter().enumerate() {
            e.mem.set_u32(cofactors + 4 * index as u32, *bits);
        }
        fn_00de88a0(e, this, Ptr::new(scalar), Ptr::new(cofactors));
    });
    0
}

// Translated from 00de8c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 4x4 matrix and inverts the copy (the engine map has no name; the shape of
/// `hkMatrix4::setInverse`), thiscall with two words `(source, tolerance)`: the four rows of
/// `source` go to `this`, then `00de8910` runs on `this` with the tolerance (a float through
/// the x87 stack). Answers its result.
pub fn fn_00de8c50(e: &mut Engine, this: Ptr, source: Ptr, tolerance: f32) -> u32 {
    for row in 0..4u32 {
        let values = load(e, source.addr().wrapping_add(0x10 * row));
        store(e, this.addr().wrapping_add(0x10 * row), values);
    }
    fn_00de8910(e, this, x87_quiet(tolerance))
}

// Translated from 00de8c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a matrix (the engine map has no name), thiscall with one word `other`: every row of
/// `this` becomes the row of `other` plus the row of `this` (lane by lane `SSE` addition, the
/// operand of `other` first).
pub fn fn_00de8c90(e: &mut Engine, this: Ptr, other: Ptr) {
    for row in 0..4u32 {
        let mine = load(e, this.addr().wrapping_add(0x10 * row));
        let theirs = load(e, other.addr().wrapping_add(0x10 * row));
        store(e, this.addr().wrapping_add(0x10 * row), add(theirs, mine));
    }
}

// Translated from 00de8ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Subtracts a matrix (the engine map has no name), thiscall with one word `other`: every row
/// of `this` becomes its row minus the row of `other`.
pub fn fn_00de8ce0(e: &mut Engine, this: Ptr, other: Ptr) {
    for row in 0..4u32 {
        let theirs = load(e, other.addr().wrapping_add(0x10 * row));
        let mine = load(e, this.addr().wrapping_add(0x10 * row));
        store(e, this.addr().wrapping_add(0x10 * row), sub(mine, theirs));
    }
}

// Translated from 00de8d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the matrix in place by a scalar (the engine map has no name), thiscall with one word
/// `scalar`: every row of `this` becomes the first lane of the vector at `scalar` (re-read
/// for every row) times the row.
pub fn fn_00de8d30(e: &mut Engine, this: Ptr, scalar: Ptr) {
    for row in 0..4u32 {
        let factor = load(e, scalar.addr())[0];
        let values = load(e, this.addr().wrapping_add(0x10 * row));
        store(
            e,
            this.addr().wrapping_add(0x10 * row),
            crate::sse::mulps(broadcast(factor), values),
        );
    }
}

// Translated from 00de8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Matrix product (the engine map has no name; the shape of `hkMatrix4::setMul`), thiscall
/// with two words `(first, second)`: copies `first` to a stack matrix and transposes it
/// (`00de87c0`); element `(r, c)` of `this` is then the dot product of row `r` of `second` and
/// row `c` of the transposed copy, so row `r` of `this` is the sum over `k` of
/// `second[r][k]` times row `k` of `first`. A dot product is `(p1 + p3) + (p0 + p2)` of the
/// four lane products `p`; the sum goes through the x87 stack (a signalling NaN comes out
/// quiet) into `this`. The row of `second` is read again for every element, so an output
/// that is also `second` changes under the loop.
pub fn fn_00de8da0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.with_stack(0x40, |e, frame| {
        let transposed = frame.addr();
        for row in 0..4u32 {
            let values = load(e, first.addr().wrapping_add(0x10 * row));
            store(e, transposed + 0x10 * row, values);
        }
        fn_00de87c0(e, Ptr::new(transposed));
        for row in 0..4u32 {
            let row_address = second.addr().wrapping_add(0x10 * row);
            for column in 0..4u32 {
                let theirs = load(e, row_address);
                let mine = load(e, transposed + 0x10 * column);
                let products = mul(theirs, mine);
                let swapped = [products[2], products[3], products[0], products[1]];
                let pairs = add(swapped, products);
                let crossed = [pairs[1], pairs[0], pairs[3], pairs[2]];
                let sum = add(crossed, pairs)[0];
                e.mem.set_u32(
                    this.addr().wrapping_add(0x10 * row + 4 * column),
                    x87_quiet_bits(sum.to_bits()),
                );
            }
        }
    });
}

// Translated from 00de8e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies in place (the engine map has no name), thiscall with one word `other`:
/// `00de8da0` into a stack matrix with `(this, other)` as its two words, then the four rows
/// of that matrix are copied into `this`.
pub fn fn_00de8e50(e: &mut Engine, this: Ptr, other: Ptr) {
    e.with_stack(0x40, |e, frame| {
        let temporary = frame.addr();
        fn_00de8da0(e, Ptr::new(temporary), this, other);
        for row in 0..4u32 {
            let values = load(e, temporary + 0x10 * row);
            store(e, this.addr().wrapping_add(0x10 * row), values);
        }
    });
}

// Translated from 00de8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the last lane of each row of the matrix (the engine map has no name), thiscall:
/// `0.0` at `+0x2c`, `+0x1c` and `+0xc`, `1.0` at `+0x3c`.
pub fn fn_00de8f10(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_f32(me.wrapping_add(0x2c), 0.0);
    e.mem.set_f32(me.wrapping_add(0x1c), 0.0);
    e.mem.set_f32(me.wrapping_add(0xc), 0.0);
    e.mem.set_f32(me.wrapping_add(0x3c), 1.0);
}

// Translated from 00de9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 4x4 matrix element by element (the engine map has no name), thiscall with one word
/// `source`: sixteen `FLD`/`FSTP` pairs, so a signalling NaN is stored quiet.
pub fn fn_00de9080(e: &mut Engine, this: Ptr, source: Ptr) {
    for element in 0..16u32 {
        copy_float(
            e,
            source.addr().wrapping_add(4 * element),
            this.addr().wrapping_add(4 * element),
        );
    }
}

// Translated from 00de90f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a matrix and makes it affine (the engine map has no name), thiscall with one word
/// `source`: the four rows are copied with `MOVAPS`, then `0.0` is stored at `+0xc`, `+0x1c`
/// and `+0x2c` and `1.0` at `+0x3c`.
pub fn fn_00de90f0(e: &mut Engine, this: Ptr, source: Ptr) {
    for row in 0..4u32 {
        let values = load(e, source.addr().wrapping_add(0x10 * row));
        store(e, this.addr().wrapping_add(0x10 * row), values);
    }
    let me = this.addr();
    e.mem.set_f32(me.wrapping_add(0xc), 0.0);
    e.mem.set_f32(me.wrapping_add(0x1c), 0.0);
    e.mem.set_f32(me.wrapping_add(0x2c), 0.0);
    e.mem.set_f32(me.wrapping_add(0x3c), 1.0);
}

/// The stack matrix and the inversion both `00de9140` and `00de9270` start with: the four rows
/// of `source` are copied into the matrix at `temporary`, which `00de8910` then inverts with
/// the tolerance at `0104da4c` (a float through the x87 stack).
fn inverted_copy(e: &mut Engine, temporary: u32, source: u32) {
    for row in 0..4u32 {
        let values = load(e, source.wrapping_add(0x10 * row));
        store(e, temporary + 0x10 * row, values);
    }
    let tolerance = x87_quiet(e.global::<f32>(TOLERANCE));
    fn_00de8910(e, Ptr::new(temporary), tolerance);
}

// Translated from 00de9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Composes a matrix with the inverse of another (the engine map has no name), thiscall with
/// two words `(transform, other)`: `other` is copied to a stack matrix `T` and inverted
/// (`00de8910` with the tolerance at `0104da4c`, its answer ignored); with `A` the rows of
/// `transform`, row `i` of `this` for `i` in `0..3` is `(T[i].x * A0 + T[i].y * A1) + T[i].z *
/// A2` and row 3 is `((T[3].x * A0 + T[3].y * A1) + T[3].z * A2) + A3`.
pub fn fn_00de9140(e: &mut Engine, this: Ptr, transform: Ptr, other: Ptr) {
    e.with_stack(0x60, |e, frame| {
        let t = frame.addr();
        inverted_copy(e, t, other.addr());
        let rows: [Lanes; 4] =
            std::array::from_fn(|row| load(e, transform.addr().wrapping_add(0x10 * row as u32)));
        let t_rows: [Lanes; 4] = std::array::from_fn(|row| load(e, t + 0x10 * row as u32));
        let xy = |row: usize| {
            add(
                mul(broadcast(t_rows[row][0]), rows[0]),
                mul(broadcast(t_rows[row][1]), rows[1]),
            )
        };
        let z = |row: usize| mul(broadcast(t_rows[row][2]), rows[2]);
        let first = add(xy(0), z(0));
        let second = add(z(1), xy(1));
        let third = add(z(2), xy(2));
        let fourth = add(add(xy(3), z(3)), rows[3]);
        store(e, this.addr(), first);
        store(e, this.addr().wrapping_add(0x10), second);
        store(e, this.addr().wrapping_add(0x20), third);
        store(e, this.addr().wrapping_add(0x30), fourth);
    });
}

// Translated from 00de9270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies a matrix with the inverse of another (the engine map has no name), thiscall with
/// two words `(first, other)`: `other` is copied to a stack matrix and inverted as in
/// `00de9140`, then `00de8da0` runs with `this`, `first` and the inverse.
pub fn fn_00de9270(e: &mut Engine, this: Ptr, first: Ptr, other: Ptr) {
    e.with_stack(0x60, |e, frame| {
        let t = frame.addr();
        inverted_copy(e, t, other.addr());
        fn_00de8da0(e, this, first, Ptr::new(t));
    });
}

// Translated from 00de92d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Intersection point of three planes (the engine map has no name), cdecl with four words
/// `(a, b, c, out)`; each plane is a vector whose `w` lane is the offset. Builds the matrix
/// `M` whose row `i` is `(a[i], b[i], c[i], 0)` (the zero from `01011d78`) and the vector
/// `v = (k - a.w, k - b.w, k - c.w, 0)` with `k` the first lane of the vector at `010c7160`
/// (`k - b.w` and `k - c.w` go through the x87 stack and come out quiet), inverts `M` with
/// `hkMatrix3::invert` (`00cd4f10`, the tolerance at `0104da4c`) and, when that succeeds,
/// stores `(v.x * R0 + v.y * R1) + v.z * R2` (`R` the rows of the inverse) at `out` and
/// answers true; otherwise answers false and writes nothing.
pub fn fn_00de92d0(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, out: Ptr) -> bool {
    e.with_stack(0x60, |e, frame| {
        let f = frame.addr();
        let constant = load(e, CONSTANT_VECTOR)[0];
        let zero = e.global::<f32>(ZERO);
        let c_w = load(e, c.addr())[3];
        let for_c = f32::from_bits(x87_quiet_bits((constant - c_w).to_bits()));
        let b_w = load(e, b.addr())[3];
        let for_b = f32::from_bits(x87_quiet_bits((constant - b_w).to_bits()));
        let a_w = load(e, a.addr())[3];
        let for_a = constant - a_w;
        let tolerance = x87_quiet_bits(e.mem.u32(TOLERANCE));
        store(e, f + 0x10, [for_a, for_b, for_c, zero]);
        let rows = [
            [
                load(e, a.addr())[0],
                load(e, b.addr())[0],
                load(e, c.addr())[0],
                zero,
            ],
            [
                load(e, a.addr())[1],
                load(e, b.addr())[1],
                load(e, c.addr())[1],
                zero,
            ],
            [
                load(e, a.addr())[2],
                load(e, b.addr())[2],
                load(e, c.addr())[2],
                zero,
            ],
        ];
        for (index, row) in rows.iter().enumerate() {
            store(e, f + 0x20 + 0x10 * index as u32, *row);
        }
        let status = e.call(MATRIX3_INVERT, &args![f + 0x20, tolerance]).u32();
        if status != 0 {
            return false;
        }
        let v = load(e, f + 0x10);
        let r0 = load(e, f + 0x20);
        let r1 = load(e, f + 0x30);
        let r2 = load(e, f + 0x40);
        let point = add(
            add(mul(broadcast(v[0]), r0), mul(broadcast(v[1]), r1)),
            mul(broadcast(v[2]), r2),
        );
        store(e, out.addr(), point);
        true
    })
}

// Translated from 00de9410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Where a segment crosses a plane (the engine map has no name), cdecl with four words `(a,
/// b, c, out)`: the segment runs from the point `a` to the point `b`, `c` is the plane (normal
/// in the first three lanes, offset in `w`). `d = dot3((b - a) * c)` (lane sum `v2 + (v1 + v0)`)
/// must satisfy `tolerance < d * d` (the float at `0104da4c`; the product rounded to a float,
/// compared on the x87 stack; a NaN fails) or the answer is false and nothing is written.
/// Otherwise `(k - (dot3(c * a) + c.w)) / d` (`k` the first lane of the vector at
/// `010c7160`, the dot as above) goes through the x87 stack to `out`, and the answer is
/// true.
pub fn fn_00de9410(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, out: Ptr) -> bool {
    let tolerance = x87_load32(e.mem.u32(TOLERANCE));
    let start = load(e, a.addr());
    let end = load(e, b.addr());
    let plane = load(e, c.addr());
    let along = mul(sub(end, start), plane);
    let d = lane_sum(along);
    let squared = d * d;
    // `FCOMP` then `JP`: false unless the tolerance is below the square (a NaN is not).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(tolerance < x87_m32(squared.to_bits())) {
        return false;
    }
    let weighted = mul(plane, start);
    let dot = lane_sum(weighted) + plane[3];
    let constant = load(e, CONSTANT_VECTOR)[0];
    let parameter = (constant - dot) / d;
    e.mem
        .set_u32(out.addr(), x87_quiet_bits(parameter.to_bits()));
    true
}

// Translated from 00de8540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpConstraintAtom` (the record at `011b882c` of the data table, name pointer `010d9590`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8540(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpConstraintAtom` (the record at `011b882c` of the data table, name pointer `010d9590`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8140(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpBridgeConstraintAtom` (the record at `011b8844` of the data table, name pointer `010d95a4`). When `object` is not null and `finishing` is not 0, runs `00d21ce0` (`hkpBridgeConstraintAtom::init` in the engine map; thiscall, one stack word, `RET 4`) with `this` = the object and the word at `object + 8` as its stack word. Otherwise nothing happens.
pub fn fn_00de8550(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 && finishing != 0 {
        let word = e.mem.u32(object.addr() + 8);
        e.call(0x00d2_1ce0, &args![object.addr(), word]);
    }
}

// Translated from 00de8160 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpBridgeConstraintAtom` (the record at `011b8844` of the data table, name pointer `010d95a4`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8160(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8570 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpBridgeAtoms` (the record at `011b885c` of the data table, name pointer `010d95bc`). When `object` is not null and `finishing` is not 0, runs `00d21ce0` (`hkpBridgeConstraintAtom::init` in the engine map; thiscall, one stack word, `RET 4`) with `this` = the object and the word at `object + 8` as its stack word. Otherwise nothing happens.
pub fn fn_00de8570(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 && finishing != 0 {
        let word = e.mem.u32(object.addr() + 8);
        e.call(0x00d2_1ce0, &args![object.addr(), word]);
    }
}

// Translated from 00de8180 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpBridgeAtoms` (the record at `011b885c` of the data table, name pointer `010d95bc`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8180(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de81a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpSimpleContactConstraintAtom` (the record at `011b8874` of the data table, name pointer `010d95cc`). When `object` is not null, runs `00d21de0` (thiscall, one stack word, `RET 4`) with `this` = the object and the second stack word `finishing`; a null object does nothing.
pub fn fn_00de81a0(e: &mut Engine, object: Ptr, finishing: u32) {
    if object.addr() != 0 {
        e.call(0x00d2_1de0, &args![object.addr(), finishing]);
    }
}

// Translated from 00de81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpSimpleContactConstraintAtom` (the record at `011b8874` of the data table, name pointer `010d95cc`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de81c0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpBallSocketConstraintAtom` (the record at `011b88a0` of the data table, name pointer `010d95ec`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8590(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de81e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpBallSocketConstraintAtom` (the record at `011b88a0` of the data table, name pointer `010d95ec`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de81e0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpStiffSpringConstraintAtom` (the record at `011b88b8` of the data table, name pointer `010d9608`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85a0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpStiffSpringConstraintAtom` (the record at `011b88b8` of the data table, name pointer `010d9608`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8200(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpSetLocalTransformsConstraintAtom` (the record at `011b88d0` of the data table, name pointer `010d9628`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8730(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpSetLocalTransformsConstraintAtom` (the record at `011b88d0` of the data table, name pointer `010d9628`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8220(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpSetLocalTranslationsConstraintAtom` (the record at `011b88e8` of the data table, name pointer `010d964c`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85b0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpSetLocalTranslationsConstraintAtom` (the record at `011b88e8` of the data table, name pointer `010d964c`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8240(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpSetLocalRotationsConstraintAtom` (the record at `011b8900` of the data table, name pointer `010d9674`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85c0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpSetLocalRotationsConstraintAtom` (the record at `011b8900` of the data table, name pointer `010d9674`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8260(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpOverwritePivotConstraintAtom` (the record at `011b8918` of the data table, name pointer `010d9698`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85d0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8280 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpOverwritePivotConstraintAtom` (the record at `011b8918` of the data table, name pointer `010d9698`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8280(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpLinConstraintAtom` (the record at `011b8930` of the data table, name pointer `010d96b8`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85e0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpLinConstraintAtom` (the record at `011b8930` of the data table, name pointer `010d96b8`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de82a0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de85f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpLinSoftConstraintAtom` (the record at `011b8948` of the data table, name pointer `010d96d0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de85f0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de82c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpLinSoftConstraintAtom` (the record at `011b8948` of the data table, name pointer `010d96d0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de82c0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpLinLimitConstraintAtom` (the record at `011b8960` of the data table, name pointer `010d96ec`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8600(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de82e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpLinLimitConstraintAtom` (the record at `011b8960` of the data table, name pointer `010d96ec`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de82e0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkp2dAngConstraintAtom` (the record at `011b8978` of the data table, name pointer `010d9708`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8610(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkp2dAngConstraintAtom` (the record at `011b8978` of the data table, name pointer `010d9708`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8300(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpAngConstraintAtom` (the record at `011b8990` of the data table, name pointer `010d9720`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8620(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8320 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpAngConstraintAtom` (the record at `011b8990` of the data table, name pointer `010d9720`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8320(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpAngLimitConstraintAtom` (the record at `011b89a8` of the data table, name pointer `010d9738`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8630(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpAngLimitConstraintAtom` (the record at `011b89a8` of the data table, name pointer `010d9738`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8340(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8640 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpTwistLimitConstraintAtom` (the record at `011b89c0` of the data table, name pointer `010d9754`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8640(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpTwistLimitConstraintAtom` (the record at `011b89c0` of the data table, name pointer `010d9754`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8360(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8650 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpConeLimitConstraintAtom` (the record at `011b89d8` of the data table, name pointer `010d9788`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8650(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8380 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpConeLimitConstraintAtom` (the record at `011b89d8` of the data table, name pointer `010d9788`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8380(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpAngFrictionConstraintAtom` (the record at `011b89f0` of the data table, name pointer `010d97a4`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8660(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de83a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpAngFrictionConstraintAtom` (the record at `011b89f0` of the data table, name pointer `010d97a4`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de83a0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpAngMotorConstraintAtom` (the record at `011b8a08` of the data table, name pointer `010d97c4`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8670(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de83c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpAngMotorConstraintAtom` (the record at `011b8a08` of the data table, name pointer `010d97c4`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de83c0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpRagdollMotorConstraintAtom` (the record at `011b8a20` of the data table, name pointer `010d97e0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8680(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de83e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpRagdollMotorConstraintAtom` (the record at `011b8a20` of the data table, name pointer `010d97e0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de83e0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8690 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpLinFrictionConstraintAtom` (the record at `011b8a38` of the data table, name pointer `010d9800`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8690(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpLinFrictionConstraintAtom` (the record at `011b8a38` of the data table, name pointer `010d9800`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8400(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpLinMotorConstraintAtom` (the record at `011b8a50` of the data table, name pointer `010d9820`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86a0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8420 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpLinMotorConstraintAtom` (the record at `011b8a50` of the data table, name pointer `010d9820`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8420(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpPulleyConstraintAtom` (the record at `011b8a68` of the data table, name pointer `010d983c`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86b0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8440 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpPulleyConstraintAtom` (the record at `011b8a68` of the data table, name pointer `010d983c`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8440(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpModifierConstraintAtom` (the record at `011b8a80` of the data table, name pointer `010d9854`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86c0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpModifierConstraintAtom` (the record at `011b8a80` of the data table, name pointer `010d9854`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8460(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpSoftContactModifierConstraintAtom` (the record at `011b8a98` of the data table, name pointer `010d9870`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86d0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpSoftContactModifierConstraintAtom` (the record at `011b8a98` of the data table, name pointer `010d9870`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8480(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpMassChangerModifierConstraintAtom` (the record at `011b8ab0` of the data table, name pointer `010d9898`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86e0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de84a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpMassChangerModifierConstraintAtom` (the record at `011b8ab0` of the data table, name pointer `010d9898`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de84a0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de86f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpCenterOfMassChangerModifierConstraintAtom` (the record at `011b8ac8` of the data table, name pointer `010d98c0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de86f0(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de84c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpCenterOfMassChangerModifierConstraintAtom` (the record at `011b8ac8` of the data table, name pointer `010d98c0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de84c0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpViscousSurfaceModifierConstraintAtom` (the record at `011b8ae0` of the data table, name pointer `010d98f0`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8700(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpViscousSurfaceModifierConstraintAtom` (the record at `011b8ae0` of the data table, name pointer `010d98f0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de84e0(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpMovingSurfaceModifierConstraintAtom` (the record at `011b8af8` of the data table, name pointer `010d9918`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8710(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpMovingSurfaceModifierConstraintAtom` (the record at `011b8af8` of the data table, name pointer `010d9918`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8500(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8720 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The finish function (record word `+4`; `hkTypeInfoRegistry::finishLoadedObject` `00d47900` calls it cdecl with the object and a word of the registry) of the type-info record of `hkpIgnoreModifierConstraintAtom` (the record at `011b8b10` of the data table, name pointer `010d9940`). A bare `RET`: it takes the two stack words, pops none and does nothing.
pub fn fn_00de8720(_e: &mut Engine, _unused_0: Ptr, _unused_1: u32) {}

// Translated from 00de8520 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkpIgnoreModifierConstraintAtom` (the record at `011b8b10` of the data table, name pointer `010d9940`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8520(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkAabb` (the record at `011b91c0` of the data table, name pointer `010d9fa8`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8740(_e: &mut Engine, _unused_0: Ptr) {}

// Translated from 00de8760 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. The cleanup function (record word `+8`; `hkTypeInfoRegistry::cleanupLoadedObject` `00d47940` calls it cdecl with the object) of the type-info record of `hkAabbUint32` (the record at `011b91d8` of the data table, name pointer `010d9fb0`). A bare `RET`: it takes the one stack word, pops none and does nothing.
pub fn fn_00de8760(_e: &mut Engine, _unused_0: Ptr) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_87c0, fn_00de87c0(Ptr)),
        entry!(0x00de_88a0, fn_00de88a0(Ptr, Ptr, Ptr)),
        entry!(0x00de_8910, fn_00de8910(Ptr, f32) -> u32),
        entry!(0x00de_8c50, fn_00de8c50(Ptr, Ptr, f32) -> u32),
        entry!(0x00de_8c90, fn_00de8c90(Ptr, Ptr)),
        entry!(0x00de_8ce0, fn_00de8ce0(Ptr, Ptr)),
        entry!(0x00de_8d30, fn_00de8d30(Ptr, Ptr)),
        entry!(0x00de_8da0, fn_00de8da0(Ptr, Ptr, Ptr)),
        entry!(0x00de_8e50, fn_00de8e50(Ptr, Ptr)),
        entry!(0x00de_8f10, fn_00de8f10(Ptr)),
        entry!(0x00de_9080, fn_00de9080(Ptr, Ptr)),
        entry!(0x00de_90f0, fn_00de90f0(Ptr, Ptr)),
        entry!(0x00de_9140, fn_00de9140(Ptr, Ptr, Ptr)),
        entry!(0x00de_9270, fn_00de9270(Ptr, Ptr, Ptr)),
        entry!(0x00de_92d0, fn_00de92d0(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00de_9410, fn_00de9410(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00de_8540, fn_00de8540(Ptr, u32)),
        entry!(0x00de_8140, fn_00de8140(Ptr)),
        entry!(0x00de_8550, fn_00de8550(Ptr, u32)),
        entry!(0x00de_8160, fn_00de8160(Ptr)),
        entry!(0x00de_8570, fn_00de8570(Ptr, u32)),
        entry!(0x00de_8180, fn_00de8180(Ptr)),
        entry!(0x00de_81a0, fn_00de81a0(Ptr, u32)),
        entry!(0x00de_81c0, fn_00de81c0(Ptr)),
        entry!(0x00de_8590, fn_00de8590(Ptr, u32)),
        entry!(0x00de_81e0, fn_00de81e0(Ptr)),
        entry!(0x00de_85a0, fn_00de85a0(Ptr, u32)),
        entry!(0x00de_8200, fn_00de8200(Ptr)),
        entry!(0x00de_8730, fn_00de8730(Ptr, u32)),
        entry!(0x00de_8220, fn_00de8220(Ptr)),
        entry!(0x00de_85b0, fn_00de85b0(Ptr, u32)),
        entry!(0x00de_8240, fn_00de8240(Ptr)),
        entry!(0x00de_85c0, fn_00de85c0(Ptr, u32)),
        entry!(0x00de_8260, fn_00de8260(Ptr)),
        entry!(0x00de_85d0, fn_00de85d0(Ptr, u32)),
        entry!(0x00de_8280, fn_00de8280(Ptr)),
        entry!(0x00de_85e0, fn_00de85e0(Ptr, u32)),
        entry!(0x00de_82a0, fn_00de82a0(Ptr)),
        entry!(0x00de_85f0, fn_00de85f0(Ptr, u32)),
        entry!(0x00de_82c0, fn_00de82c0(Ptr)),
        entry!(0x00de_8600, fn_00de8600(Ptr, u32)),
        entry!(0x00de_82e0, fn_00de82e0(Ptr)),
        entry!(0x00de_8610, fn_00de8610(Ptr, u32)),
        entry!(0x00de_8300, fn_00de8300(Ptr)),
        entry!(0x00de_8620, fn_00de8620(Ptr, u32)),
        entry!(0x00de_8320, fn_00de8320(Ptr)),
        entry!(0x00de_8630, fn_00de8630(Ptr, u32)),
        entry!(0x00de_8340, fn_00de8340(Ptr)),
        entry!(0x00de_8640, fn_00de8640(Ptr, u32)),
        entry!(0x00de_8360, fn_00de8360(Ptr)),
        entry!(0x00de_8650, fn_00de8650(Ptr, u32)),
        entry!(0x00de_8380, fn_00de8380(Ptr)),
        entry!(0x00de_8660, fn_00de8660(Ptr, u32)),
        entry!(0x00de_83a0, fn_00de83a0(Ptr)),
        entry!(0x00de_8670, fn_00de8670(Ptr, u32)),
        entry!(0x00de_83c0, fn_00de83c0(Ptr)),
        entry!(0x00de_8680, fn_00de8680(Ptr, u32)),
        entry!(0x00de_83e0, fn_00de83e0(Ptr)),
        entry!(0x00de_8690, fn_00de8690(Ptr, u32)),
        entry!(0x00de_8400, fn_00de8400(Ptr)),
        entry!(0x00de_86a0, fn_00de86a0(Ptr, u32)),
        entry!(0x00de_8420, fn_00de8420(Ptr)),
        entry!(0x00de_86b0, fn_00de86b0(Ptr, u32)),
        entry!(0x00de_8440, fn_00de8440(Ptr)),
        entry!(0x00de_86c0, fn_00de86c0(Ptr, u32)),
        entry!(0x00de_8460, fn_00de8460(Ptr)),
        entry!(0x00de_86d0, fn_00de86d0(Ptr, u32)),
        entry!(0x00de_8480, fn_00de8480(Ptr)),
        entry!(0x00de_86e0, fn_00de86e0(Ptr, u32)),
        entry!(0x00de_84a0, fn_00de84a0(Ptr)),
        entry!(0x00de_86f0, fn_00de86f0(Ptr, u32)),
        entry!(0x00de_84c0, fn_00de84c0(Ptr)),
        entry!(0x00de_8700, fn_00de8700(Ptr, u32)),
        entry!(0x00de_84e0, fn_00de84e0(Ptr)),
        entry!(0x00de_8710, fn_00de8710(Ptr, u32)),
        entry!(0x00de_8500, fn_00de8500(Ptr)),
        entry!(0x00de_8720, fn_00de8720(Ptr, u32)),
        entry!(0x00de_8520, fn_00de8520(Ptr)),
        entry!(0x00de_8740, fn_00de8740(Ptr)),
        entry!(0x00de_8760, fn_00de8760(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A matrix of 16 floats in memory, row by row.
    fn matrix(e: &mut Engine, values: [f32; 16]) -> u32 {
        let at = e.mem.alloc(0x40);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
        at
    }

    fn read(e: &Engine, at: u32) -> [f32; 16] {
        std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32))
    }

    fn sequence() -> [f32; 16] {
        std::array::from_fn(|i| i as f32 + 1.0)
    }

    fn diagonal(a: f32, b: f32, c: f32, d: f32) -> [f32; 16] {
        let mut values = [0.0; 16];
        values[0] = a;
        values[5] = b;
        values[10] = c;
        values[15] = d;
        values
    }

    /// A matrix whose first row starts with `first` and whose other elements are 0.
    fn leading(first: [f32; 4]) -> [f32; 16] {
        let mut values = [0.0; 16];
        values[..4].copy_from_slice(&first);
        values
    }

    #[test]
    fn the_transposition_swaps_the_elements_across_the_diagonal() {
        let mut e = Engine::new();
        let m = matrix(&mut e, sequence());
        e.call(0x00de_87c0, &args![m]);
        let got = read(&e, m);
        for row in 0..4 {
            for column in 0..4 {
                assert_eq!(got[row * 4 + column], (column * 4 + row) as f32 + 1.0);
            }
        }
    }

    #[test]
    fn the_transposition_quiets_signalling_nans_it_moves() {
        let mut e = Engine::new();
        let m = matrix(&mut e, sequence());
        e.mem.set_u32(m + 4, 0x7f80_0001);
        e.call(0x00de_87c0, &args![m]);
        assert_eq!(e.mem.u32(m + 0x10), 0x7fc0_0001);
        // the diagonal is not touched
        assert_eq!(e.mem.f32(m), 1.0);
    }

    #[test]
    fn the_scalar_product_scales_every_row() {
        let mut e = Engine::new();
        let source = matrix(&mut e, sequence());
        let scalar = matrix(&mut e, leading([3.0, 9.0, 9.0, 9.0]));
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_88a0, &args![this, scalar, source]);
        let want: Vec<f32> = sequence().iter().map(|v| 3.0 * v).collect();
        assert_eq!(read(&e, this).to_vec(), want);
    }

    #[test]
    fn the_scalar_is_read_again_for_every_row_when_it_is_in_the_result() {
        let mut e = Engine::new();
        let source = matrix(&mut e, sequence());
        // the result holds the scalar 2 in its first lane: after row 0 that lane is 2 * 1 = 2
        let this = matrix(&mut e, leading([2.0, 0.0, 0.0, 0.0]));
        e.call(0x00de_88a0, &args![this, this, source]);
        let got = read(&e, this);
        assert_eq!(got[0..4], [2.0, 4.0, 6.0, 8.0]);
        assert_eq!(got[4..8], [10.0, 12.0, 14.0, 16.0]);
    }

    fn rows_equal(got: [f32; 16], want: [f32; 16]) {
        assert_eq!(got.to_vec(), want.to_vec());
    }

    #[test]
    fn the_inversion_of_a_diagonal_matrix_scales_the_cofactors_by_the_reciprocal() {
        let mut e = Engine::new();
        let m = matrix(&mut e, diagonal(2.0, 3.0, 4.0, 5.0));
        let result = e.call(0x00de_8910, &args![m, 1.0e-6f32]).u32();
        assert_eq!(result, 0);
        // det = 120; the cofactors of the diagonal are 60, 40, 30, 24
        let reciprocal = (1.0f64 / 120.0) as f32;
        rows_equal(
            read(&e, m),
            diagonal(
                reciprocal * 60.0,
                reciprocal * 40.0,
                reciprocal * 30.0,
                reciprocal * 24.0,
            ),
        );
    }

    #[test]
    fn the_inversion_of_an_affine_matrix_gives_its_inverse() {
        let mut e = Engine::new();
        // rows (2, 0, 0, 0), (0, 4, 0, 0), (0, 0, 8, 0), (1, 2, 3, 1)
        let mut values = diagonal(2.0, 4.0, 8.0, 1.0);
        values[12] = 1.0;
        values[13] = 2.0;
        values[14] = 3.0;
        let m = matrix(&mut e, values);
        assert_eq!(e.call(0x00de_8910, &args![m, 1.0e-6f32]).u32(), 0);
        let inverse = read(&e, m);
        // diag(0.5, 0.25, 0.125, 1) with the last row (-0.5, -0.5, -0.375, 1)
        let want = [
            0.5, 0.0, 0.0, 0.0, 0.0, 0.25, 0.0, 0.0, 0.0, 0.0, 0.125, 0.0, -0.5, -0.5, -0.375, 1.0,
        ];
        for (got, want) in inverse.iter().zip(want) {
            assert!((got - want).abs() < 1.0e-6, "{inverse:?}");
        }
    }

    #[test]
    fn a_singular_matrix_is_left_alone_and_the_answer_is_one() {
        let mut e = Engine::new();
        let values = diagonal(2.0, 0.0, 4.0, 5.0);
        let m = matrix(&mut e, values);
        assert_eq!(e.call(0x00de_8910, &args![m, 1.0e-6f32]).u32(), 1);
        rows_equal(read(&e, m), values);
        // a tolerance above the determinant fails the same way; a NaN tolerance does not
        let values = diagonal(2.0, 3.0, 4.0, 5.0);
        let m = matrix(&mut e, values);
        assert_eq!(e.call(0x00de_8910, &args![m, 1000.0f32]).u32(), 1);
        rows_equal(read(&e, m), values);
        let m = matrix(&mut e, values);
        assert_eq!(e.call(0x00de_8910, &args![m, f32::NAN]).u32(), 0);
    }

    #[test]
    fn the_inverse_copy_copies_then_inverts() {
        let mut e = Engine::new();
        let source = matrix(&mut e, diagonal(2.0, 4.0, 8.0, 16.0));
        let this = matrix(&mut e, [0.0; 16]);
        assert_eq!(
            e.call(0x00de_8c50, &args![this, source, 1.0e-6f32]).u32(),
            0
        );
        let want = diagonal(0.5, 0.25, 0.125, 0.0625);
        for (got, want) in read(&e, this).iter().zip(want) {
            assert!((got - want).abs() < 1.0e-7);
        }
        // the source is untouched
        rows_equal(read(&e, source), diagonal(2.0, 4.0, 8.0, 16.0));
        // a failing inversion leaves the copy
        let this = matrix(&mut e, [0.0; 16]);
        assert_eq!(e.call(0x00de_8c50, &args![this, source, 1.0e9f32]).u32(), 1);
        rows_equal(read(&e, this), diagonal(2.0, 4.0, 8.0, 16.0));
    }

    #[test]
    fn the_sum_adds_the_other_matrix() {
        let mut e = Engine::new();
        let this = matrix(&mut e, sequence());
        let other = matrix(&mut e, sequence().map(|v| v * 10.0));
        e.call(0x00de_8c90, &args![this, other]);
        let want: Vec<f32> = sequence().iter().map(|v| v * 11.0).collect();
        assert_eq!(read(&e, this).to_vec(), want);
    }

    #[test]
    fn the_difference_subtracts_the_other_matrix() {
        let mut e = Engine::new();
        let this = matrix(&mut e, sequence().map(|v| v * 10.0));
        let other = matrix(&mut e, sequence());
        e.call(0x00de_8ce0, &args![this, other]);
        let want: Vec<f32> = sequence().iter().map(|v| v * 9.0).collect();
        assert_eq!(read(&e, this).to_vec(), want);
    }

    #[test]
    fn the_in_place_scaling_multiplies_every_row() {
        let mut e = Engine::new();
        let this = matrix(&mut e, sequence());
        let scalar = matrix(&mut e, leading([0.5, 7.0, 7.0, 7.0]));
        e.call(0x00de_8d30, &args![this, scalar]);
        let want: Vec<f32> = sequence().iter().map(|v| 0.5 * v).collect();
        assert_eq!(read(&e, this).to_vec(), want);
    }

    /// Row `r` of the product is the sum over `k` of `second[r][k]` times row `k` of `first`.
    fn product(first: [f32; 16], second: [f32; 16]) -> [f32; 16] {
        std::array::from_fn(|index| {
            let (row, column) = (index / 4, index % 4);
            (0..4)
                .map(|k| second[row * 4 + k] * first[k * 4 + column])
                .sum()
        })
    }

    fn second_matrix() -> [f32; 16] {
        [
            1.0, 0.0, 2.0, 0.0, 0.0, 1.0, 0.0, 3.0, 4.0, 0.0, 1.0, 0.0, 0.0, 5.0, 0.0, 1.0,
        ]
    }

    #[test]
    fn the_matrix_product_multiplies_rows_of_the_second_with_the_first() {
        let mut e = Engine::new();
        let first = matrix(&mut e, sequence());
        let second = matrix(&mut e, second_matrix());
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_8da0, &args![this, first, second]);
        rows_equal(read(&e, this), product(sequence(), second_matrix()));
    }

    #[test]
    fn the_in_place_product_uses_a_temporary() {
        let mut e = Engine::new();
        let this = matrix(&mut e, sequence());
        let other = matrix(&mut e, second_matrix());
        e.call(0x00de_8e50, &args![this, other]);
        // this is the first and other the second matrix of the product
        rows_equal(read(&e, this), product(sequence(), second_matrix()));
    }

    #[test]
    fn the_last_lanes_are_set_to_an_affine_matrix() {
        let mut e = Engine::new();
        let m = matrix(&mut e, sequence());
        e.call(0x00de_8f10, &args![m]);
        let got = read(&e, m);
        assert_eq!([got[3], got[7], got[11], got[15]], [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(got[0], 1.0);
        assert_eq!(got[14], 15.0);
    }

    #[test]
    fn the_element_copy_quiets_signalling_nans() {
        let mut e = Engine::new();
        let source = matrix(&mut e, sequence());
        e.mem.set_u32(source + 0x20, 0xff80_0001);
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_9080, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0x20), 0xffc0_0001);
        assert_eq!(e.mem.f32(this + 0x3c), 16.0);
    }

    #[test]
    fn the_affine_copy_copies_the_rows_and_fixes_the_last_lanes() {
        let mut e = Engine::new();
        let source = matrix(&mut e, sequence());
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_90f0, &args![this, source]);
        let got = read(&e, this);
        assert_eq!([got[3], got[7], got[11], got[15]], [0.0, 0.0, 0.0, 1.0]);
        assert_eq!([got[0], got[5], got[10], got[14]], [1.0, 6.0, 11.0, 15.0]);
    }

    /// The inverse of `diag(2, 4, 8, 1)` is `diag(0.5, 0.25, 0.125, 1)`.
    fn invertible() -> [f32; 16] {
        diagonal(2.0, 4.0, 8.0, 1.0)
    }

    fn tolerance_engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x0104_d000, 0x1000);
        e.mem.set_f32(TOLERANCE, 1.1920929e-7);
        e
    }

    #[test]
    fn composing_with_an_inverse_combines_the_rows() {
        let mut e = tolerance_engine();
        let transform = matrix(&mut e, sequence());
        let other = matrix(&mut e, invertible());
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_9140, &args![this, transform, other]);
        let a = sequence();
        let got = read(&e, this);
        // T = diag(0.5, 0.25, 0.125, 1): rows 0..3 are scaled rows of the transform, the
        // last is the last row of the transform (T[3] has only its w lane set)
        let scaled = [0.5f32, 0.25, 0.125];
        for row in 0..3 {
            for lane in 0..4 {
                assert_eq!(got[row * 4 + lane], scaled[row] * a[row * 4 + lane]);
            }
        }
        assert_eq!(got[12..16], a[12..16]);
    }

    #[test]
    fn multiplying_with_an_inverse_uses_the_inverse_as_the_second_matrix() {
        let mut e = tolerance_engine();
        let first = matrix(&mut e, sequence());
        let other = matrix(&mut e, invertible());
        let this = matrix(&mut e, [0.0; 16]);
        e.call(0x00de_9270, &args![this, first, other]);
        let inverse = diagonal(0.5, 0.25, 0.125, 1.0);
        rows_equal(read(&e, this), product(sequence(), inverse));
    }

    // ----- the plane helpers -----

    fn plane_engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x0104_d000, 0x1000);
        e.mem.map(0x0101_1000, 0x1000);
        e.mem.map(0x010c_7000, 0x1000);
        e.mem.set_f32(TOLERANCE, 1.1920929e-7);
        e.mem.set_f32(ZERO, 0.0);
        for lane in 0..4 {
            e.mem.set_f32(CONSTANT_VECTOR + 4 * lane, 0.0);
        }
        e
    }

    fn vector(e: &mut Engine, v: [f32; 4]) -> u32 {
        let at = e.mem.alloc(0x10);
        for (i, value) in v.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
        at
    }

    fn read_vector(e: &Engine, at: u32) -> [f32; 4] {
        std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32))
    }

    #[test]
    fn three_planes_meet_where_the_inverse_of_their_normals_puts_the_offsets() {
        let mut e = plane_engine();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        {
            let seen = seen.clone();
            e.register_double(MATRIX3_INVERT, move |e, words| {
                let mut row = words.to_vec();
                row.extend((0..12).map(|i| e.mem.u32(words[0] + 4 * i)));
                seen.borrow_mut().push(row);
                Ret::default()
            });
        }
        let a = vector(&mut e, [1.0, 0.0, 0.0, -2.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, -3.0]);
        let c = vector(&mut e, [0.0, 0.0, 1.0, -4.0]);
        let out = vector(&mut e, [9.0; 4]);
        assert!(e.call(0x00de_92d0, &args![a, b, c, out]).bool());
        let rows = seen.borrow();
        assert_eq!(rows.len(), 1);
        // the tolerance, then the identity rows (the double leaves them as they are)
        assert_eq!(rows[0][1], 1.1920929e-7f32.to_bits());
        let floats: Vec<f32> = rows[0][2..].iter().map(|b| f32::from_bits(*b)).collect();
        assert_eq!(
            floats,
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        // v = (0 - a.w, 0 - b.w, 0 - c.w) = (2, 3, 4) against the identity
        assert_eq!(read_vector(&e, out), [2.0, 3.0, 4.0, 0.0]);
    }

    #[test]
    fn the_plane_intersection_uses_the_rows_of_the_inverse() {
        let mut e = plane_engine();
        // the double replaces the matrix by rows that double the identity
        e.register(MATRIX3_INVERT, |e, words| {
            for row in 0..3u32 {
                for lane in 0..4u32 {
                    let value = if lane == row { 2.0 } else { 0.0 };
                    e.mem.set_f32(words[0] + 0x10 * row + 4 * lane, value);
                }
            }
            Ret::default()
        });
        let a = vector(&mut e, [1.0, 0.0, 0.0, -2.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, -3.0]);
        let c = vector(&mut e, [0.0, 0.0, 1.0, -4.0]);
        let out = vector(&mut e, [9.0; 4]);
        assert!(e.call(0x00de_92d0, &args![a, b, c, out]).bool());
        assert_eq!(read_vector(&e, out), [4.0, 6.0, 8.0, 0.0]);
    }

    #[test]
    fn a_failing_inversion_reports_failure_and_writes_nothing() {
        let mut e = plane_engine();
        e.register(MATRIX3_INVERT, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let a = vector(&mut e, [1.0, 0.0, 0.0, -2.0]);
        let out = vector(&mut e, [9.0; 4]);
        assert!(!e.call(0x00de_92d0, &args![a, a, a, out]).bool());
        assert_eq!(read_vector(&e, out), [9.0; 4]);
    }

    #[test]
    fn the_segment_meets_the_plane_at_the_parameter() {
        let mut e = plane_engine();
        let start = vector(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let end = vector(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let plane = vector(&mut e, [0.0, 0.0, 1.0, -1.0]);
        let out = vector(&mut e, [9.0; 4]);
        assert!(e.call(0x00de_9410, &args![start, end, plane, out]).bool());
        // d = 2, t = (0 - (0 + -1)) / 2
        assert_eq!(e.mem.f32(out), 0.5);
        // the other lanes of the output are not written
        assert_eq!(e.mem.f32(out + 4), 9.0);
    }

    #[test]
    fn a_segment_parallel_to_the_plane_does_not_meet_it() {
        let mut e = plane_engine();
        let start = vector(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let end = vector(&mut e, [5.0, 0.0, 0.0, 0.0]);
        let plane = vector(&mut e, [0.0, 0.0, 1.0, -1.0]);
        let out = vector(&mut e, [9.0; 4]);
        assert!(!e.call(0x00de_9410, &args![start, end, plane, out]).bool());
        assert_eq!(e.mem.f32(out), 9.0);
        // a NaN direction fails too
        let end = vector(&mut e, [0.0, 0.0, f32::NAN, 0.0]);
        assert!(!e.call(0x00de_9410, &args![start, end, plane, out]).bool());
    }
}

#[cfg(test)]
mod tests_a03_type_info {
    use crate::prelude::*;

    /// Calls `function` with an object holding a pattern (and, for two words, a finishing word):
    /// nothing else is called and the object is unchanged.
    fn check_nothing(function: u32, words: usize) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u32(object + 4 * i, 0x1000_0000 + i);
        }
        e.call_log = Some(vec![]);
        if words == 2 {
            e.call(function, &args![object, 1u32]);
        } else {
            e.call(function, &args![object]);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        for i in 0..16 {
            assert_eq!(e.mem.u32(object + 4 * i), 0x1000_0000 + i);
        }
    }

    macro_rules! nothing_test {
        ($name:ident, $function:expr, $words:expr) => {
            #[test]
            fn $name() {
                check_nothing($function, $words);
            }
        };
    }

    /// A non-null object: the callee double runs once with `this` = the object and the finishing
    /// word; a null object calls nothing.
    fn check_calls(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(function, &args![0u32, 0x55u32]);
        e.call(function, &args![object, 0x66u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, 0x66]);
    }

    macro_rules! calls_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check_calls($function, $callee);
            }
        };
    }

    /// `00d21ce0` runs (with the word at `+8` of the object) only for a non-null object and a
    /// non-zero finishing word.
    fn check_bridge(function: u32) {
        let mut e = Engine::new();
        e.register(0x00d2_1ce0, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 8, 0xfeed_0001);
        e.call_log = Some(vec![]);
        e.call(function, &args![0u32, 1u32]);
        e.call(function, &args![object, 0u32]);
        e.call(function, &args![object, 1u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00d2_1ce0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![object, 0xfeed_0001]);
    }

    macro_rules! bridge_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_bridge($function);
            }
        };
    }

    nothing_test!(fn_00de8540_does_nothing, 0x00de_8540, 2);
    nothing_test!(fn_00de8140_does_nothing, 0x00de_8140, 1);
    bridge_test!(fn_00de8550_runs_init_only_when_finishing, 0x00de_8550);
    nothing_test!(fn_00de8160_does_nothing, 0x00de_8160, 1);
    bridge_test!(fn_00de8570_runs_init_only_when_finishing, 0x00de_8570);
    nothing_test!(fn_00de8180_does_nothing, 0x00de_8180, 1);
    calls_test!(
        fn_00de81a0_runs_its_callee_on_a_non_null_object,
        0x00de_81a0,
        0x00d2_1de0
    );
    nothing_test!(fn_00de81c0_does_nothing, 0x00de_81c0, 1);
    nothing_test!(fn_00de8590_does_nothing, 0x00de_8590, 2);
    nothing_test!(fn_00de81e0_does_nothing, 0x00de_81e0, 1);
    nothing_test!(fn_00de85a0_does_nothing, 0x00de_85a0, 2);
    nothing_test!(fn_00de8200_does_nothing, 0x00de_8200, 1);
    nothing_test!(fn_00de8730_does_nothing, 0x00de_8730, 2);
    nothing_test!(fn_00de8220_does_nothing, 0x00de_8220, 1);
    nothing_test!(fn_00de85b0_does_nothing, 0x00de_85b0, 2);
    nothing_test!(fn_00de8240_does_nothing, 0x00de_8240, 1);
    nothing_test!(fn_00de85c0_does_nothing, 0x00de_85c0, 2);
    nothing_test!(fn_00de8260_does_nothing, 0x00de_8260, 1);
    nothing_test!(fn_00de85d0_does_nothing, 0x00de_85d0, 2);
    nothing_test!(fn_00de8280_does_nothing, 0x00de_8280, 1);
    nothing_test!(fn_00de85e0_does_nothing, 0x00de_85e0, 2);
    nothing_test!(fn_00de82a0_does_nothing, 0x00de_82a0, 1);
    nothing_test!(fn_00de85f0_does_nothing, 0x00de_85f0, 2);
    nothing_test!(fn_00de82c0_does_nothing, 0x00de_82c0, 1);
    nothing_test!(fn_00de8600_does_nothing, 0x00de_8600, 2);
    nothing_test!(fn_00de82e0_does_nothing, 0x00de_82e0, 1);
    nothing_test!(fn_00de8610_does_nothing, 0x00de_8610, 2);
    nothing_test!(fn_00de8300_does_nothing, 0x00de_8300, 1);
    nothing_test!(fn_00de8620_does_nothing, 0x00de_8620, 2);
    nothing_test!(fn_00de8320_does_nothing, 0x00de_8320, 1);
    nothing_test!(fn_00de8630_does_nothing, 0x00de_8630, 2);
    nothing_test!(fn_00de8340_does_nothing, 0x00de_8340, 1);
    nothing_test!(fn_00de8640_does_nothing, 0x00de_8640, 2);
    nothing_test!(fn_00de8360_does_nothing, 0x00de_8360, 1);
    nothing_test!(fn_00de8650_does_nothing, 0x00de_8650, 2);
    nothing_test!(fn_00de8380_does_nothing, 0x00de_8380, 1);
    nothing_test!(fn_00de8660_does_nothing, 0x00de_8660, 2);
    nothing_test!(fn_00de83a0_does_nothing, 0x00de_83a0, 1);
    nothing_test!(fn_00de8670_does_nothing, 0x00de_8670, 2);
    nothing_test!(fn_00de83c0_does_nothing, 0x00de_83c0, 1);
    nothing_test!(fn_00de8680_does_nothing, 0x00de_8680, 2);
    nothing_test!(fn_00de83e0_does_nothing, 0x00de_83e0, 1);
    nothing_test!(fn_00de8690_does_nothing, 0x00de_8690, 2);
    nothing_test!(fn_00de8400_does_nothing, 0x00de_8400, 1);
    nothing_test!(fn_00de86a0_does_nothing, 0x00de_86a0, 2);
    nothing_test!(fn_00de8420_does_nothing, 0x00de_8420, 1);
    nothing_test!(fn_00de86b0_does_nothing, 0x00de_86b0, 2);
    nothing_test!(fn_00de8440_does_nothing, 0x00de_8440, 1);
    nothing_test!(fn_00de86c0_does_nothing, 0x00de_86c0, 2);
    nothing_test!(fn_00de8460_does_nothing, 0x00de_8460, 1);
    nothing_test!(fn_00de86d0_does_nothing, 0x00de_86d0, 2);
    nothing_test!(fn_00de8480_does_nothing, 0x00de_8480, 1);
    nothing_test!(fn_00de86e0_does_nothing, 0x00de_86e0, 2);
    nothing_test!(fn_00de84a0_does_nothing, 0x00de_84a0, 1);
    nothing_test!(fn_00de86f0_does_nothing, 0x00de_86f0, 2);
    nothing_test!(fn_00de84c0_does_nothing, 0x00de_84c0, 1);
    nothing_test!(fn_00de8700_does_nothing, 0x00de_8700, 2);
    nothing_test!(fn_00de84e0_does_nothing, 0x00de_84e0, 1);
    nothing_test!(fn_00de8710_does_nothing, 0x00de_8710, 2);
    nothing_test!(fn_00de8500_does_nothing, 0x00de_8500, 1);
    nothing_test!(fn_00de8720_does_nothing, 0x00de_8720, 2);
    nothing_test!(fn_00de8520_does_nothing, 0x00de_8520, 1);
    nothing_test!(fn_00de8740_does_nothing, 0x00de_8740, 1);
    nothing_test!(fn_00de8760_does_nothing, 0x00de_8760, 1);
}
