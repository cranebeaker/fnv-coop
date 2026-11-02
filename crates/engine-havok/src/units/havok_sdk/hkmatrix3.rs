//! `Havok SDK/hkmatrix3.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkMatrix3` is three 16-byte columns (`x`, `y`, `z` lanes used, the fourth lane is
//! padding). The SSE code multiplies with `MULPS` of a broadcast lane by a column and adds
//! the three products in the order the instructions add them (`f32`, all four lanes).

#[allow(unused_imports)]
use crate::prelude::*;

/// The SSE sign-clearing mask (four words, `0x7fffffff`) `isApproximatelyEqual` loads.
const ABS_MASK: u32 = 0x0101_7d10;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// `FLD dword [from]; FSTP dword [to]`: a copy that quiets a signalling NaN.
fn copy_through_x87(e: &mut Engine, to: u32, from: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00cd4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setTranspose` (Xbox PDB): copies the diagonal (+0, +0x14, +0x28), zeroes the
/// padding lanes (+0xC, +0x1C, +0x2C) and swaps the off-diagonal pairs, element by element
/// in the exe's order (so `this == source` gives the exe's partial result), every copy
/// through the x87.
pub fn hk_matrix3_set_transpose(e: &mut Engine, this: Ptr, source: Ptr) {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 0x14, 0x28] {
        copy_through_x87(e, to + offset, from + offset);
    }
    for offset in [0xc, 0x1c, 0x2c] {
        e.mem.set_u32(to + offset, 0);
    }
    for (destination, origin) in [
        (0x4, 0x10),
        (0x10, 0x4),
        (0x8, 0x20),
        (0x20, 0x8),
        (0x18, 0x24),
        (0x24, 0x18),
    ] {
        copy_through_x87(e, to + destination, from + origin);
    }
}

// Translated from 00cd4e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::isApproximatelyEqual` (Xbox PDB): writes 1 to the byte `out` when, for each of
/// the three columns, `|this - other| <= epsilon` in lanes 0..2 (`CMPLEPS`: false for a NaN;
/// the absolute value is the `ANDPS` with the mask at `01017d10`), else 0. Stops at the
/// first column that differs. Returns `out`.
pub fn hk_matrix3_is_approximately_equal(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    other: Ptr,
    epsilon: f32,
) -> Ptr {
    let mut equal = true;
    for column in 0..3u32 {
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        let theirs = load_lanes(e, other.addr() + 0x10 * column);
        let within = (0..3).all(|lane| {
            let mask = e.mem.u32(ABS_MASK + 4 * lane as u32);
            let difference = mine[lane] - theirs[lane];
            f32::from_bits(difference.to_bits() & mask) <= epsilon
        });
        if !within {
            equal = false;
            break;
        }
    }
    e.mem.set_u8(out.addr(), u8::from(equal));
    out
}

// Translated from 00cd55b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setMul` (second overload, Xbox PDB): `this = a * b`. The columns of `a` are
/// loaded first; then for the columns of `b` from the last to the first,
/// `this.column = (x * a0 + y * a1) + z * a2` (`x`, `y`, `z` the lanes of that column of
/// `b`), four lanes.
pub fn hk_matrix3_set_mul_ov2(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let columns = [
        load_lanes(e, a.addr()),
        load_lanes(e, a.addr() + 0x10),
        load_lanes(e, a.addr() + 0x20),
    ];
    for column in (0..3u32).rev() {
        let v = load_lanes(e, b.addr() + 0x10 * column);
        let result: Lanes = std::array::from_fn(|lane| {
            (v[0] * columns[0][lane] + v[1] * columns[1][lane]) + v[2] * columns[2][lane]
        });
        store_lanes(e, this.addr() + 0x10 * column, result);
    }
}

// Translated from 00cd5680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::mul` (Xbox PDB): `this = this * other`, through a temporary
/// (`setMul_ov2(temp, this, other)`, then the three columns copied back).
pub fn hk_matrix3_mul(e: &mut Engine, this: Ptr, other: Ptr) {
    e.with_stack(0x30, |e, temp| {
        hk_matrix3_set_mul_ov2(e, temp, this, other);
        for offset in (0..0x30).step_by(4) {
            let word = e.mem.u32(temp.addr() + offset);
            e.mem.set_u32(this.addr() + offset, word);
        }
    });
}

// Translated from 00cd4b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::getIdentity` (Xbox PDB): takes no arguments and returns the constant 0x0126_8370.
pub fn hk_matrix3_get_identity(_e: &mut Engine) -> u32 {
    0x0126_8370
}

/// `hkVector4::isOk3` (`00c7f4e0`, thiscall with the address of a result byte; returns that
/// address).
const VECTOR4_IS_OK3: u32 = 0x00c7_f4e0;
/// The CRT square root (`00ec6040`, argument in `ST0`, result in `ST0`).
const X87_SQRT: u32 = 0x00ec_6040;
/// Constants of the exe's data the matrix code loads: a float 0.0, a float whose bits are
/// `0x80000000` (the `0 - x` of a negation), a float 1.0 and a double 1.0, then the float and the
/// double the determinant of `invertSymmetric` is kept above (both read as `2^-69` in the exe).
const ZERO_F32: u32 = 0x0101_1d78;
const NEGATIVE_ZERO_F32: u32 = 0x010c_4990;
const ONE_F32: u32 = 0x0101_2068;
const ONE_F64: u32 = 0x0101_2070;
const DETERMINANT_FLOOR_F32: u32 = 0x010c_b414;
const DETERMINANT_FLOOR_F64: u32 = 0x010c_b418;

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: Lanes, src: Lanes, imm: u8) -> Lanes {
    let pick = |v: Lanes, shift: u8| v[usize::from((imm >> shift) & 3)];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: Lanes, src: Lanes) -> Lanes {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: Lanes, src: Lanes) -> Lanes {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`.
fn movlhps(dst: Lanes, src: Lanes) -> Lanes {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`.
fn movhlps(dst: Lanes, src: Lanes) -> Lanes {
    [src[2], src[3], dst[2], dst[3]]
}

/// `MULPS dst, src`.
fn mulps(dst: Lanes, src: Lanes) -> Lanes {
    crate::sse::mulps(dst, src)
}

/// `ADDPS dst, src`.
fn addps(dst: Lanes, src: Lanes) -> Lanes {
    crate::sse::addps(dst, src)
}

/// `SUBPS dst, src`.
fn subps(dst: Lanes, src: Lanes) -> Lanes {
    crate::sse::subps(dst, src)
}

/// Lane `lane` of `v` in all four lanes (`SHUFPS v, v, lane * 0x55`).
fn splat(v: Lanes, lane: usize) -> Lanes {
    [v[lane]; 4]
}

/// The three columns of the matrix at `addr`.
fn load_columns(e: &Engine, addr: u32) -> [Lanes; 3] {
    [
        load_lanes(e, addr),
        load_lanes(e, addr + 0x10),
        load_lanes(e, addr + 0x20),
    ]
}

/// The sum the exe forms from the three products `p` of a determinant: lane 1 plus lane 0
/// (`ADDSS`), then lane 2 plus that.
fn sum_of_products(p: Lanes) -> f32 {
    crate::sse::add(p[2], crate::sse::add(p[1], p[0]))
}

/// The four lanes of a vector loaded from memory by a scalar `MOVSS`: the value, then zeros.
fn scalar_lanes(value: f32) -> Lanes {
    [value, 0.0, 0.0, 0.0]
}

// Translated from 00cd4af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::isOk` (Xbox PDB): writes 1 to the byte `out` when `hkVector4::isOk3` (`00c7f4e0`,
/// given the address of one of three local result bytes) answers true for each of the three
/// columns in turn, else 0 (it stops at the first column that fails). Returns `out`.
pub fn hk_matrix3_is_ok(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.with_stack(4, |e, locals| {
        for column in 0..3u32 {
            let answer = e
                .call(
                    VECTOR4_IS_OK3,
                    &args![this.addr() + 0x10 * column, locals.addr() + 1 + column],
                )
                .u32();
            if e.mem.u8(answer) == 0 {
                e.mem.set_u8(out.addr(), 0);
                return out;
            }
        }
        e.mem.set_u8(out.addr(), 1);
        out
    })
}

// Translated from 00cd4b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::transpose` (Xbox PDB): swaps the off-diagonal pairs (+4/+0x10, +8/+0x20,
/// +0x18/+0x24) in place through a float temporary on the stack, each value through the x87
/// (a signalling NaN comes out quiet).
pub fn hk_matrix3_transpose(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (first, second) in [(0x10, 0x4), (0x20, 0x8), (0x24, 0x18)] {
        let held = x87_quiet_bits(e.mem.u32(base + first));
        copy_through_x87(e, base + first, base + second);
        e.mem.set_u32(base + second, x87_quiet_bits(held));
    }
}

// Translated from 00cd4bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `this = transpose(a) * b`: the columns of `a` are turned into
/// rows (`UNPCKLPS`, `UNPCKHPS`, `MOVLHPS`, `MOVHLPS`, `SHUFPS`; lane 3 of the rows comes from
/// the padding lanes of the third column), then for each column `v` of `b`, first to last,
/// `this.column = (v.y * row_y + v.x * row_x) + v.z * row_z` in four lanes. `a` is read before
/// `this` is written; `b` is read column by column.
pub fn fn_00cd4bf0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let [first, second, third] = load_columns(e, a.addr());
    let low = unpcklps(first, second);
    let high = unpckhps(first, second);
    let row_x = movlhps(low, third);
    let row_y = shufps(movhlps(row_x, low), third, 0xd4);
    let row_z = shufps(high, third, 0xe4);
    for column in 0..3u32 {
        let v = load_lanes(e, b.addr() + 0x10 * column);
        let from_x = mulps(splat(v, 0), row_x);
        let from_y = mulps(splat(v, 1), row_y);
        let from_z = mulps(splat(v, 2), row_z);
        let result = addps(addps(from_y, from_x), from_z);
        store_lanes(e, this.addr() + 0x10 * column, result);
    }
}

// Translated from 00cd4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Multiplies each of the three columns of `this` by lane 0 of the
/// vector at `scale` (all four lanes of the column), reading the vector again before each
/// column.
pub fn fn_00cd4c70(e: &mut Engine, this: Ptr, scale: Ptr) {
    for column in 0..3u32 {
        let factor = splat(load_lanes(e, scale.addr()), 0);
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        store_lanes(e, this.addr() + 0x10 * column, mulps(factor, mine));
    }
}

// Translated from 00cd4cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setCrossSkewSymmetric` (Xbox PDB): the matrix of the cross product with the
/// vector `(x, y, z)` at `vector`. With `zero` the float at `0101_1d78` and `negative` the float
/// at `010c4990` (the exe computes `negative - v` for the negations), the columns are
/// `(zero, z, negative - y, zero)`, `(negative - z, zero, x, zero)` and
/// `(y, negative - x, zero, zero)`.
pub fn hk_matrix3_set_cross_skew_symmetric(e: &mut Engine, this: Ptr, vector: Ptr) {
    let zero = e.mem.f32(ZERO_F32);
    let negative = e.mem.f32(NEGATIVE_ZERO_F32);
    let x = e.mem.f32(vector.addr());
    let y = e.mem.f32(vector.addr() + 4);
    let z = e.mem.f32(vector.addr() + 8);
    store_lanes(e, this.addr(), [zero, z, negative - y, zero]);
    store_lanes(e, this.addr() + 0x10, [negative - z, zero, x, zero]);
    store_lanes(e, this.addr() + 0x20, [y, negative - x, zero, zero]);
}

// Translated from 00cd4d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setMul` (Xbox PDB, the overload taking a vector and a matrix): for each column,
/// `this.column = lane 0 of the vector at scale * the same column of b` (four lanes), the
/// vector read again before each column.
pub fn hk_matrix3_set_mul(e: &mut Engine, this: Ptr, scale: Ptr, b: Ptr) {
    for column in 0..3u32 {
        let factor = splat(load_lanes(e, scale.addr()), 0);
        let theirs = load_lanes(e, b.addr() + 0x10 * column);
        store_lanes(e, this.addr() + 0x10 * column, mulps(factor, theirs));
    }
}

// Translated from 00cd4d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. For each column, `this.column = (lane 0 of the vector at scale *
/// the same column of b) + this.column` (four lanes), the vector read again before each column.
pub fn fn_00cd4d90(e: &mut Engine, this: Ptr, scale: Ptr, b: Ptr) {
    for column in 0..3u32 {
        let factor = splat(load_lanes(e, scale.addr()), 0);
        let theirs = load_lanes(e, b.addr() + 0x10 * column);
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        store_lanes(
            e,
            this.addr() + 0x10 * column,
            addps(mulps(factor, theirs), mine),
        );
    }
}

// Translated from 00cd4e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Whether the matrix equals its transpose within `epsilon`: builds
/// the transpose in a local matrix (every element through the x87, the padding lanes zero) and
/// answers the byte `hkMatrix3::isApproximatelyEqual` (`00cd4e00`, called with `this`, the
/// address of a local result byte, the local matrix and `epsilon` through the x87) writes.
pub fn fn_00cd4e90(e: &mut Engine, this: Ptr, epsilon: f32) -> bool {
    e.with_stack(0x30, |e, local| {
        let (to, from) = (local.addr(), this.addr());
        for offset in [0, 0x14, 0x28] {
            copy_through_x87(e, to + offset, from + offset);
        }
        for offset in [0xc, 0x1c, 0x2c] {
            e.mem.set_u32(to + offset, 0);
        }
        for (destination, origin) in [
            (0x4, 0x10),
            (0x10, 0x4),
            (0x8, 0x20),
            (0x20, 0x8),
            (0x18, 0x24),
            (0x24, 0x18),
        ] {
            copy_through_x87(e, to + destination, from + origin);
        }
        e.with_stack(4, |e, result| {
            let answer =
                hk_matrix3_is_approximately_equal(e, this, result, local, x87_quiet(epsilon));
            e.mem.u8(answer.addr()) != 0
        })
    })
}

// Translated from 00cd4f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::invert` (Xbox PDB): inverts the matrix in place and returns `hkResult` 0, or
/// returns 1 and leaves it unchanged when the determinant is too small. The three cofactor
/// columns come from `SHUFPS` (`0xd2`, `0xc9`) and `MULPS`/`SUBPS`; the determinant is the
/// products of the first column with the first cofactor column summed as
/// `p2 + (p1 + p0)` (`ADDSS`), and it fails when `|determinant|` (through the x87, stored as a
/// float) is not above `epsilon^3` (the x87 product, NaN included). Otherwise the cofactor
/// columns are scaled by `one / determinant` (`DIVSS` with the float at `01012068`) and stored,
/// then the matrix is transposed element by element through the x87.
pub fn hk_matrix3_invert(e: &mut Engine, this: Ptr, epsilon: f32) -> u32 {
    let base = this.addr();
    let [c0, c1, c2] = load_columns(e, base);
    let eps = x87_load32(epsilon.to_bits());
    let c1_d2 = shufps(c1, c1, 0xd2);
    let c1_c9 = shufps(c1, c1, 0xc9);
    let c2_c9 = shufps(c2, c2, 0xc9);
    let c2_d2 = shufps(c2, c2, 0xd2);
    let c0_d2 = shufps(c0, c0, 0xd2);
    let c0_c9 = shufps(c0, c0, 0xc9);
    let cofactor0 = subps(mulps(c2_d2, c1_c9), mulps(c2_c9, c1_d2));
    let determinant = sum_of_products(mulps(c0, cofactor0));
    let absolute = x87_load32(x87_quiet_bits(determinant.to_bits()) & 0x7fff_ffff);
    let limit = x87_mul(eps, x87_mul(eps, eps));
    if !(absolute > limit) {
        return 1;
    }
    let inverse = splat(scalar_lanes(e.mem.f32(ONE_F32) / determinant), 0);
    let m1 = mulps(c2_d2, c0_c9);
    let m3 = mulps(c2_c9, c0_d2);
    let m4 = mulps(c0_d2, c1_c9);
    let m5 = mulps(c0_c9, c1_d2);
    let cofactor1 = subps(m3, m1);
    let cofactor2 = subps(m5, m4);
    store_lanes(e, base, mulps(inverse, cofactor0));
    store_lanes(e, base + 0x10, mulps(cofactor1, inverse));
    store_lanes(e, base + 0x20, mulps(cofactor2, inverse));
    hk_matrix3_transpose(e, this);
    0
}

// Translated from 00cd5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::invertSymmetric` (Xbox PDB): inverts the matrix in place without testing it.
/// The cofactor columns are formed as in `invert` (`00cd4f10`) from `SHUFPS` (`0xd2`, `0xc9`)
/// products (the exe multiplies in the opposite operand order here), the determinant is
/// `p2 + (p1 + p0)` of the first column times the first cofactor column; it goes through the x87
/// (stored as a float); when it is below the double at `010cb418` (ordered) it is replaced by
/// the float at `010cb414`. Each column is the cofactor column times the float `1.0 / determinant`
/// (x87 division, rounded to a float).
pub fn hk_matrix3_invert_symmetric(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let [c0, c1, c2] = load_columns(e, base);
    let c1_d2 = shufps(c1, c1, 0xd2);
    let c1_c9 = shufps(c1, c1, 0xc9);
    let c2_c9 = shufps(c2, c2, 0xc9);
    let c2_d2 = shufps(c2, c2, 0xd2);
    let c0_d2 = shufps(c0, c0, 0xd2);
    let c0_c9 = shufps(c0, c0, 0xc9);
    let cofactor0 = subps(mulps(c1_c9, c2_d2), mulps(c1_d2, c2_c9));
    let cofactor1 = subps(mulps(c2_c9, c0_d2), mulps(c2_d2, c0_c9));
    let cofactor2 = subps(mulps(c1_d2, c0_c9), mulps(c1_c9, c0_d2));
    let sum = sum_of_products(mulps(c0, cofactor0));
    let mut stored = x87_quiet_bits(sum.to_bits());
    if x87_load32(stored) < e.mem.f64(DETERMINANT_FLOOR_F64) {
        stored = x87_quiet_bits(e.mem.u32(DETERMINANT_FLOOR_F32));
    }
    let reciprocal = x87_store32(x87_div(1.0, x87_load32(stored)));
    let factor = [f32::from_bits(reciprocal); 4];
    store_lanes(e, base, mulps(factor, cofactor0));
    store_lanes(e, base + 0x10, mulps(factor, cofactor1));
    store_lanes(e, base + 0x20, mulps(factor, cofactor2));
}

// Translated from 00cd5140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::add` (Xbox PDB): `this.column = other.column + this.column` for the three
/// columns (four lanes; the other matrix is the first operand of `ADDPS`).
pub fn hk_matrix3_add(e: &mut Engine, this: Ptr, other: Ptr) {
    for column in 0..3u32 {
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        let theirs = load_lanes(e, other.addr() + 0x10 * column);
        store_lanes(e, this.addr() + 0x10 * column, addps(theirs, mine));
    }
}

// Translated from 00cd5180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::sub` (Xbox PDB): `this.column = this.column - other.column` for the three
/// columns (four lanes).
pub fn hk_matrix3_sub(e: &mut Engine, this: Ptr, other: Ptr) {
    for column in 0..3u32 {
        let theirs = load_lanes(e, other.addr() + 0x10 * column);
        let mine = load_lanes(e, this.addr() + 0x10 * column);
        store_lanes(e, this.addr() + 0x10 * column, subps(mine, theirs));
    }
}

// Translated from 00cd51c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::getDeterminant` (Xbox PDB): writes the four lanes `(d, p2, p2, p2)` to the
/// vector `out`, with `(p0, p1, p2)` the lane-wise product of the first matrix row
/// (`m00, m10, m20`) with the cofactor row built from the second and third rows (`SHUFPS` `0xd2`,
/// `0xc9`), and `d = p2 + (p1 + p0)`. The rows are gathered with `MOVSS` and `UNPCKLPS` from the
/// float elements (lane 3 of each row is the float at `01011d78`). Answers `out` (its callers
/// read the determinant through `EAX`).
pub fn hk_matrix3_get_determinant(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let base = this.addr();
    let zero = e.mem.f32(ZERO_F32);
    let element = |e: &Engine, offset: u32| e.mem.f32(base + offset);
    let (m04, m14, m24) = (element(e, 0x4), element(e, 0x14), element(e, 0x24));
    let (m08, m18, m28) = (element(e, 0x8), element(e, 0x18), element(e, 0x28));
    let (m00, m10, m20) = (element(e, 0x0), element(e, 0x10), element(e, 0x20));
    let row_y = [m04, m14, m24, zero];
    let row_z = [m08, m18, m28, zero];
    let row_x = [m00, m10, m20, zero];
    let left = mulps(shufps(row_z, row_z, 0xc9), shufps(row_y, row_y, 0xd2));
    let right = mulps(shufps(row_z, row_z, 0xd2), shufps(row_y, row_y, 0xc9));
    let products = mulps(subps(right, left), row_x);
    let sum = sum_of_products(products);
    store_lanes(e, out.addr(), [sum, products[2], products[2], products[2]]);
    // 00cd51e1 MOV EAX,[EBP+8] (out), 00cd5258 MOVAPS [EAX],XMM2: EAX is still out at the RET 4.
    out
}

// Translated from 00cd5270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a Jacobi rotation of a symmetric matrix (the helper
/// `00cd56c0` uses). Takes its inputs in registers: `col` in `ECX`, `row` in `EDX`, `matrix` in
/// `EDI`; then the stack word `out`. Writes `row` to `out + 8` and `col` to `out + 0xc`. With
/// `a` the float of `matrix` at byte `col * 16 + row * 4` (column `col`, row `row`): when `a`
/// equals 0.0 (ordered) it writes the floats 1.0 and 0.0 to `out + 0` and `out + 4` and
/// stops. Otherwise, all in x87 arithmetic with a float store between the steps,
/// `theta = (m[col][col] - m[row][row]) / (a + a)`; `r = sqrt(theta * theta + 1.0)` (the
/// double at `01012070`, CRT `00ec6040`); `x = theta - r` when `theta < 0.0` or unordered,
/// else `theta + r` (the compare is `FCOM` against 0.0, bit 0 of the status word);
/// `t = 1.0 / x`; `c = 1.0 / sqrt(1.0 + t * t)`; it writes `c` to `out + 0` and `c * t` to
/// `out + 4`.
pub fn fn_00cd5270(e: &mut Engine, col: u32, row: u32, matrix: Ptr, out: Ptr) {
    let element = matrix
        .addr()
        .wrapping_add(col.wrapping_mul(16).wrapping_add(row.wrapping_mul(4)));
    e.mem.set_u32(out.addr() + 8, row);
    e.mem.set_u32(out.addr() + 0xc, col);
    let a_bits = e.mem.u32(element);
    // FCOM float [element] against 0.0: equal only when ordered and equal.
    if x87_m32(a_bits) == 0.0 {
        e.mem.set_u32(out.addr(), 1.0f32.to_bits());
        e.mem.set_u32(out.addr() + 4, 0);
        return;
    }
    let diagonal = x87_load32(
        e.mem
            .u32(matrix.addr().wrapping_add(col.wrapping_mul(0x14))),
    );
    let other_diagonal = x87_m32(
        e.mem
            .u32(matrix.addr().wrapping_add(row.wrapping_mul(0x14))),
    );
    let difference = x87_sub(diagonal, other_diagonal);
    let twice = x87_add(x87_load32(a_bits), x87_load32(a_bits));
    let theta_bits = x87_store32(x87_div(difference, twice));
    let theta = x87_load32(theta_bits);
    let square = x87_add(x87_mul(theta, theta), e.mem.f64(ONE_F64));
    let root_input = x87_load32(x87_store32(square));
    let root_bits = x87_quiet_bits(x87_store32(e.call(X87_SQRT, &args![root_input]).f64()));
    let root = x87_m32(root_bits);
    // FCOM sets bit 0 of the status word for "below" and for unordered.
    let x = if !(theta >= 0.0) {
        x87_sub(theta, root)
    } else {
        x87_add(theta, root)
    };
    let t_bits = x87_store32(x87_div(1.0, x));
    let t = x87_load32(t_bits);
    let sum = x87_add(1.0, x87_mul(t, t));
    let sum_root_input = x87_load32(x87_store32(sum));
    let sum_root_bits = x87_store32(e.call(X87_SQRT, &args![sum_root_input]).f64());
    let c_bits = x87_store32(x87_div(1.0, x87_load32(sum_root_bits)));
    let c = x87_load32(c_bits);
    e.mem.set_u32(out.addr(), x87_quiet_bits(c_bits));
    let s_bits = x87_store32(x87_mul(c, x87_m32(t_bits)));
    e.mem.set_u32(out.addr() + 4, s_bits);
}

// Translated from 00cd5340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the Frobenius norm of a 3x3 matrix, result in `ST0` (here the
/// return value). Takes `matrix` in `EAX`. For each row `i` in turn it adds the squares of
/// `m[0][i]`, `m[1][i]`, `m[2][i]` (the floats at `i*4`, `0x10 + i*4`, `0x20 + i*4`) to a float
/// accumulator, rounding to a float after each addition (x87 arithmetic), then takes the CRT
/// square root (`00ec6040`) and rounds to a float.
pub fn fn_00cd5340(e: &mut Engine, matrix: Ptr) -> f64 {
    let mut accumulator = 0.0f32.to_bits();
    for row in 0..3u32 {
        let base = matrix.addr() + 4 * row;
        let x0 = x87_load32(e.mem.u32(base));
        let x1 = x87_load32(e.mem.u32(base + 0x10));
        let x2 = x87_load32(e.mem.u32(base + 0x20));
        accumulator = x87_store32(x87_add(x87_mul(x0, x0), x87_load32(accumulator)));
        accumulator = x87_store32(x87_add(x87_mul(x1, x1), x87_m32(accumulator)));
        accumulator = x87_store32(x87_add(x87_mul(x2, x2), x87_m32(accumulator)));
    }
    let root = e.call(X87_SQRT, &args![x87_load32(accumulator)]).f64();
    x87_load32(x87_store32(root))
}

// Translated from 00cd53a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the norm of the off-diagonal elements of a 3x3 matrix, result
/// in `ST0` (here the return value). Takes `matrix` in `ECX`. For each row `j` and each column
/// `c` other than `j`, in that order, adds the square of the float at `c * 0x10 + j * 4` to a
/// float accumulator (x87 arithmetic, rounded to a float after each addition), then takes the
/// CRT square root (`00ec6040`) and rounds to a float.
pub fn fn_00cd53a0(e: &mut Engine, matrix: Ptr) -> f64 {
    let mut accumulator = 0.0f32.to_bits();
    for row in 0..3u32 {
        for column in 0..3u32 {
            if column == row {
                continue;
            }
            let value = x87_load32(e.mem.u32(matrix.addr() + 0x10 * column + 4 * row));
            accumulator = x87_store32(x87_add(x87_mul(value, value), x87_m32(accumulator)));
        }
    }
    let root = e.call(X87_SQRT, &args![x87_load32(accumulator)]).f64();
    x87_load32(x87_store32(root))
}

// Translated from 00cd5400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: finds the off-diagonal element of largest magnitude above the
/// diagonal. Takes its inputs in registers: `matrix` in `EAX`, `column_out` in `EDX`,
/// `row_out` in `ESI` (no stack words). The running maximum starts as the float 0.0. For each
/// row `j` of 0..3 it considers, in this order, the float at `0x00 + j*4` (column 0) when
/// `j < 0` (never true: the exe's loop starts at 0, the branch is translated for fidelity),
/// the float at `0x10 + j*4` (column 1) when `j < 1` and the float at `0x20 + j*4` (column 2)
/// when `j < 2`. An element replaces the maximum when `maximum <= |element|` (the absolute
/// value through the x87 and a float store; false for a NaN): then `*row_out = j` and
/// `*column_out = ` its column. The values left are those of the last replacement; nothing is
/// written when no element replaces the initial 0.0.
pub fn fn_00cd5400(e: &mut Engine, matrix: Ptr, column_out: Ptr, row_out: Ptr) {
    let mut maximum = 0.0f32.to_bits();
    for row in 0..3i32 {
        let mut consider = |e: &mut Engine, column: u32| {
            let element = e.mem.u32(matrix.addr() + 0x10 * column + 4 * row as u32);
            let absolute_bits = x87_quiet_bits(element) & 0x7fff_ffff;
            let absolute = x87_load32(absolute_bits);
            if x87_load32(maximum) <= absolute {
                e.mem.set_u32(row_out.addr(), row as u32);
                maximum = absolute_bits;
                e.mem.set_u32(column_out.addr(), column);
            }
        };
        if row < 0 {
            consider(e, 0);
        }
        if row < 1 {
            consider(e, 1);
        }
        if row < 2 {
            consider(e, 2);
        }
    }
}

// Translated from 00cd54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: builds the rotation matrix of a Jacobi step. Takes its inputs in
/// registers: `record` in `EAX`, `out` in `ECX` (no stack words). The record (the output of
/// `00cd5270`) is `c` (+0), `s` (+4), `p` (+8, a word) and `q` (+0xc, a word). The matrix at
/// `out` is the identity, then `m[p][p] = c`, element `p + q * 4` (column `q`, row `p`) `= s`,
/// element `q + p * 4` (column `p`, row `q`) `= -s` and `m[q][q] = c`, every float through the
/// x87 (a signalling NaN comes out quiet; the negation flips the sign of the quiet value).
/// The record is read again for every store, after the matrix was cleared.
pub fn fn_00cd54d0(e: &mut Engine, record: Ptr, out: Ptr) {
    let (base, record) = (out.addr(), record.addr());
    for column in 0..3u32 {
        store_lanes(e, base + 0x10 * column, [0.0; 4]);
    }
    for offset in [0, 0x14, 0x28] {
        e.mem.set_u32(base + offset, 1.0f32.to_bits());
    }
    let p = e.mem.u32(record + 8);
    let c_bits = x87_quiet_bits(e.mem.u32(record));
    let diagonal = p.wrapping_mul(5).wrapping_mul(4);
    e.mem.set_u32(base.wrapping_add(diagonal), c_bits);
    let p = e.mem.u32(record + 8);
    let q = e.mem.u32(record + 0xc);
    let s_bits = x87_quiet_bits(e.mem.u32(record + 4));
    let above = p.wrapping_add(q.wrapping_mul(4)).wrapping_mul(4);
    e.mem.set_u32(base.wrapping_add(above), s_bits);
    let q = e.mem.u32(record + 0xc);
    let negated = x87_quiet_bits(e.mem.u32(record + 4)) ^ 0x8000_0000;
    let p = e.mem.u32(record + 8);
    let below = q.wrapping_add(p.wrapping_mul(4)).wrapping_mul(4);
    e.mem.set_u32(base.wrapping_add(below), negated);
    let q = e.mem.u32(record + 0xc);
    let c_bits = x87_quiet_bits(e.mem.u32(record));
    let diagonal = q.wrapping_mul(5).wrapping_mul(4);
    e.mem.set_u32(base.wrapping_add(diagonal), c_bits);
}

// Translated from 00cd5530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3_setMulMat3Mat3` (engine map name): cdecl `(destination, a, b)`, the three stack
/// words, no `this`. The same instruction sequence as `setMul` second overload (`00cd55b0`),
/// which takes the destination in `ECX`: `destination = a * b`.
pub fn hk_matrix3_set_mul_mat3_mat3(e: &mut Engine, destination: Ptr, a: Ptr, b: Ptr) {
    hk_matrix3_set_mul_ov2(e, destination, a, b);
}

// Translated from 00cd55a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Engine map name `hkMatrix3_invertSymmetric`: the stack-argument entry of
/// `hkMatrix3::invertSymmetric`; loads the one stack word into `ECX` and jumps to `00cd5050`
/// (which ends in a plain `RET`: the caller removes the word).
pub fn fn_00cd55a0(e: &mut Engine, matrix: Ptr) {
    hk_matrix3_invert_symmetric(e, matrix);
}

// Translated from 00cd5610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::setMulInverse` (Xbox PDB): `this = a * transpose(b)`. Transposes `b` into a
/// local matrix with `setTranspose` (`00cd4ba0`) and multiplies it by `a` column by column as
/// `setMul` second overload does (`00cd55b0`: `a` is read before `this` is written).
pub fn hk_matrix3_set_mul_inverse(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    e.with_stack(0x30, |e, transposed| {
        hk_matrix3_set_transpose(e, transposed, b);
        hk_matrix3_set_mul_ov2(e, this, a, transposed);
    });
}

// Translated from 00cd56c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: diagonalises a symmetric matrix by Jacobi rotations (the
/// functions at `00cd5340`, `00cd53a0`, `00cd5400`, `00cd5270` and `00cd54d0` are its helpers);
/// thiscall with four stack words. Returns 1 when `this` is not symmetric within `epsilon`
/// (`00cd4e90`), touching nothing. Otherwise: a working copy `w` of the matrix; `eigenvectors`
/// is cleared to the identity; the threshold is the float `norm(this) * epsilon` (`00cd5340`,
/// x87 multiplication); while the off-diagonal norm of `w` (`00cd53a0`, as a float) is not
/// below the threshold and fewer than `max_iterations` steps ran (signed compare), it finds the
/// largest element above the diagonal (`00cd5400`), makes the rotation `r` (`00cd5270`,
/// `00cd54d0`), sets `w = transpose(r) * (w * r)` and `eigenvectors = eigenvectors * r`.
/// The vector at `eigenvalues` is then `(w00, w11, w22, 0)`; the result is 1 when
/// `threshold < off-diagonal norm` (ordered) and 0 otherwise (a NaN counts as converged).
pub fn fn_00cd56c0(
    e: &mut Engine,
    this: Ptr,
    eigenvectors: Ptr,
    eigenvalues: Ptr,
    max_iterations: i32,
    epsilon: f32,
) -> u32 {
    if !fn_00cd4e90(e, this, epsilon) {
        return 1;
    }
    e.with_stack(0xd0, |e, frame| {
        // The exe's frame: +0x10 and +0x18 receive the indices of the largest element, +0x20 is
        // the working matrix, +0x58 the threshold as a double, +0x60 the rotation matrix,
        // +0x90 the rotation record and +0xa0 the transposed rotation matrix.
        let (cells_column, cells_row) = (frame.addr() + 0x10, frame.addr() + 0x18);
        let (work, threshold_double) = (frame.addr() + 0x20, frame.addr() + 0x58);
        let (rotation, record, transposed) = (
            frame.addr() + 0x60,
            frame.addr() + 0x90,
            frame.addr() + 0xa0,
        );
        for offset in (0..0x30).step_by(0x10) {
            let columns = load_lanes(e, this.addr() + offset);
            store_lanes(e, work + offset, columns);
        }
        for column in 0..3u32 {
            store_lanes(e, eigenvectors.addr() + 0x10 * column, [0.0; 4]);
        }
        for offset in [0, 0x14, 0x28] {
            e.mem
                .set_u32(eigenvectors.addr() + offset, 1.0f32.to_bits());
        }
        let norm = fn_00cd5340(e, this);
        let threshold_bits = x87_store32(x87_mul(norm, x87_m32(epsilon.to_bits())));
        let off_diagonal = fn_00cd53a0(e, Ptr::new(work));
        let mut off = x87_load32(x87_store32(off_diagonal));
        let threshold = x87_load32(threshold_bits);
        e.mem.set_f64(threshold_double, threshold);
        let mut iterations = 0i32;
        let mut converged = off < threshold;
        while !converged {
            if iterations >= max_iterations {
                break;
            }
            e.mem.set_u32(cells_row, 0);
            e.mem.set_u32(cells_column, 0);
            fn_00cd5400(
                e,
                Ptr::new(work),
                Ptr::new(cells_column),
                Ptr::new(cells_row),
            );
            let row = e.mem.u32(cells_row);
            let column = e.mem.u32(cells_column);
            fn_00cd5270(e, column, row, Ptr::new(work), Ptr::new(record));
            fn_00cd54d0(e, Ptr::new(record), Ptr::new(rotation));
            for offset in (0..0x30).step_by(0x10) {
                let columns = load_lanes(e, rotation + offset);
                store_lanes(e, transposed + offset, columns);
            }
            hk_matrix3_transpose(e, Ptr::new(transposed));
            hk_matrix3_mul(e, Ptr::new(work), Ptr::new(rotation));
            hk_matrix3_set_mul_ov2(e, Ptr::new(work), Ptr::new(transposed), Ptr::new(work));
            hk_matrix3_mul(e, eigenvectors, Ptr::new(rotation));
            iterations += 1;
            off = x87_load32(x87_store32(fn_00cd53a0(e, Ptr::new(work))));
            converged = off < e.mem.f64(threshold_double);
        }
        let diagonal = |e: &Engine, offset: u32| e.mem.u32(work + offset);
        let values = [diagonal(e, 0), diagonal(e, 0x14), diagonal(e, 0x28), 0];
        for (lane, bits) in values.into_iter().enumerate() {
            e.mem.set_u32(eigenvalues.addr() + 4 * lane as u32, bits);
        }
        u32::from(x87_load32(threshold_bits) < off)
    })
}

// Translated from 00cd5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMatrix3::changeBasis` (Xbox PDB): `this = basis * (this * transpose(basis))`. A local
/// matrix gets `setMulInverse(local, this, basis)` (`00cd5610`), then `this` gets
/// `setMul(basis, local)` (`00cd55b0`).
pub fn hk_matrix3_change_basis(e: &mut Engine, this: Ptr, basis: Ptr) {
    e.with_stack(0x30, |e, local| {
        hk_matrix3_set_mul_inverse(e, local, this, basis);
        hk_matrix3_set_mul_ov2(e, this, basis, local);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd4b50, hk_matrix3_get_identity() -> u32),
        entry!(0x00cd4af0, hk_matrix3_is_ok(Ptr, Ptr) -> Ptr),
        entry!(0x00cd4b60, hk_matrix3_transpose(Ptr)),
        entry!(0x00cd4bf0, fn_00cd4bf0(Ptr, Ptr, Ptr)),
        entry!(0x00cd4c70, fn_00cd4c70(Ptr, Ptr)),
        entry!(0x00cd4cc0, hk_matrix3_set_cross_skew_symmetric(Ptr, Ptr)),
        entry!(0x00cd4d40, hk_matrix3_set_mul(Ptr, Ptr, Ptr)),
        entry!(0x00cd4d90, fn_00cd4d90(Ptr, Ptr, Ptr)),
        entry!(0x00cd4e90, fn_00cd4e90(Ptr, f32) -> bool),
        entry!(0x00cd4f10, hk_matrix3_invert(Ptr, f32) -> u32),
        entry!(0x00cd5050, hk_matrix3_invert_symmetric(Ptr)),
        entry!(0x00cd5140, hk_matrix3_add(Ptr, Ptr)),
        entry!(0x00cd5180, hk_matrix3_sub(Ptr, Ptr)),
        // 00cd525e RET 4 with EAX = out (loaded at 00cd51e1).
        entry!(0x00cd51c0, hk_matrix3_get_determinant(Ptr, Ptr) -> Ptr),
        entry!(0x00cd5270, fn_00cd5270(u32, u32, Ptr, Ptr)),
        entry!(0x00cd5340, fn_00cd5340(Ptr) -> f64),
        entry!(0x00cd53a0, fn_00cd53a0(Ptr) -> f64),
        entry!(0x00cd5400, fn_00cd5400(Ptr, Ptr, Ptr)),
        entry!(0x00cd54d0, fn_00cd54d0(Ptr, Ptr)),
        entry!(0x00cd5530, hk_matrix3_set_mul_mat3_mat3(Ptr, Ptr, Ptr)),
        entry!(0x00cd55a0, fn_00cd55a0(Ptr)),
        entry!(0x00cd5610, hk_matrix3_set_mul_inverse(Ptr, Ptr, Ptr)),
        entry!(0x00cd56c0, fn_00cd56c0(Ptr, Ptr, Ptr, i32, f32) -> u32),
        entry!(0x00cd5880, hk_matrix3_change_basis(Ptr, Ptr)),
        entry!(0x00cd_4ba0, hk_matrix3_set_transpose(Ptr, Ptr)),
        entry!(
            0x00cd_4e00,
            hk_matrix3_is_approximately_equal(Ptr, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x00cd_55b0, hk_matrix3_set_mul_ov2(Ptr, Ptr, Ptr)),
        entry!(0x00cd_5680, hk_matrix3_mul(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(e: &mut Engine, columns: [[f32; 3]; 3]) -> u32 {
        let m = e.mem.alloc(0x30);
        for (i, c) in columns.iter().enumerate() {
            store_lanes(e, m + 16 * i as u32, [c[0], c[1], c[2], 9.0]);
        }
        m
    }

    fn elements(e: &Engine, m: u32) -> [[f32; 3]; 3] {
        std::array::from_fn(|c| {
            let lanes = load_lanes(e, m + 16 * c as u32);
            [lanes[0], lanes[1], lanes[2]]
        })
    }

    #[test]
    fn transpose_swaps_pairs_and_zeroes_the_padding() {
        let mut e = Engine::new();
        let source = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let dest = e.mem.alloc(0x30);
        e.mem.set_u32(dest + 0xc, 0xffff_ffff);
        e.call(0x00cd_4ba0, &args![dest, source]);
        assert_eq!(
            elements(&e, dest),
            [[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]]
        );
        for padding in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(dest + padding), 0);
        }
    }

    #[test]
    fn transpose_quiets_signalling_nans() {
        let mut e = Engine::new();
        let source = matrix(&mut e, [[0.0; 3]; 3]);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 0x10, 0xff80_0002);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cd_4ba0, &args![dest, source]);
        assert_eq!(e.mem.u32(dest), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 4), 0xffc0_0002);
    }

    fn setup_mask(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
    }

    #[test]
    fn approximate_equality_checks_each_column_and_ignores_the_fourth_lane() {
        let mut e = Engine::new();
        setup_mask(&mut e);
        let a = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = matrix(&mut e, [[1.1, 1.9, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let out = e.mem.alloc(4);
        let got = e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00cd_4e00, &args![a, out, b, 0.05f32]);
        assert_eq!(e.mem.u8(out), 0);
        // only the third column differs
        store_lanes(&mut e, b, [1.0, 2.0, 3.0, 50.0]);
        store_lanes(&mut e, b + 0x20, [7.0, 8.0, 9.5, 9.0]);
        e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]);
        assert_eq!(e.mem.u8(out), 0);
        // the padding lane of every column differs; still equal
        store_lanes(&mut e, b + 0x20, [7.0, 8.0, 9.0, 123.0]);
        e.call(0x00cd_4e00, &args![a, out, b, 0.2f32]);
        assert_eq!(e.mem.u8(out), 1);
        // NaN compares false
        e.mem.set_f32(b + 4, f32::NAN);
        e.call(0x00cd_4e00, &args![a, out, b, 100.0f32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn set_mul_ov2_multiplies_column_by_column() {
        let mut e = Engine::new();
        let a = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]);
        let b = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cd_55b0, &args![dest, a, b]);
        // column of b = (x, y, z): x * a0 + y * a1 + z * a2
        assert_eq!(
            elements(&e, dest),
            [[1.0, -3.0, 2.0], [4.0, -6.0, 5.0], [7.0, -9.0, 8.0]]
        );
        // the fourth lane: (1 + 2) * 9 + 3 * 9 from the padding lanes of a
        assert_eq!(e.mem.f32(dest + 12), 54.0);
    }

    #[test]
    fn mul_replaces_this_with_this_times_other() {
        let mut e = Engine::new();
        let this = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]);
        let other = matrix(&mut e, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        e.call(0x00cd_5680, &args![this, other]);
        assert_eq!(
            elements(&e, this),
            [[1.0, -3.0, 2.0], [4.0, -6.0, 5.0], [7.0, -9.0, 8.0]]
        );
    }

    #[test]
    fn hk_matrix3_get_identity_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cd_4b50, &args![]).u32(), 0x0126_8370);
    }

    /// The exe's constants this unit reads, and the sign mask.
    fn setup_constants(e: &mut Engine) {
        setup_mask(e);
        e.mem.set_u32(NEGATIVE_ZERO_F32, 0x8000_0000);
        e.mem.set_f32(ONE_F32, 1.0);
        e.mem.set_f64(ONE_F64, 1.0);
        e.mem.set_u32(DETERMINANT_FLOOR_F32, 0x1d00_0000);
        e.mem.set_u64(DETERMINANT_FLOOR_F64, 0x3ba0_0000_0000_0000);
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() <= 1e-5 * (1.0 + b.abs())
    }

    /// `a * b` of two column-major matrices, in `f64`.
    fn product(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
        std::array::from_fn(|j| {
            std::array::from_fn(|i| {
                (0..3)
                    .map(|k| f64::from(b[j][k]) * f64::from(a[k][i]))
                    .sum::<f64>() as f32
            })
        })
    }

    const SAMPLE: [[f32; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
    const IDENTITY: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    fn is_ok3_double(e: &mut Engine, a: &[u32]) -> Ret {
        let ok = e.mem.f32(a[0]) >= 0.0;
        e.mem.set_u8(a[1], u8::from(ok));
        Ret {
            eax: a[1],
            ..Ret::default()
        }
    }

    #[test]
    fn is_ok_asks_each_column_and_stops_at_the_first_failure() {
        let mut e = Engine::new();
        e.register(VECTOR4_IS_OK3, is_ok3_double);
        let m = matrix(&mut e, SAMPLE);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let got = e.call(0x00cd_4af0, &args![m, out]).u32();
        assert_eq!(got, out);
        assert_eq!(e.mem.u8(out), 1);
        let log = e.call_log.take().unwrap();
        let asked: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == VECTOR4_IS_OK3)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(asked, [m, m + 0x10, m + 0x20]);
        // the second column fails: the third is not asked
        e.mem.set_f32(m + 0x10, -1.0);
        e.call_log = Some(vec![]);
        e.call(0x00cd_4af0, &args![m, out]);
        assert_eq!(e.mem.u8(out), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == VECTOR4_IS_OK3).count(), 2);
    }

    #[test]
    fn transpose_in_place_swaps_pairs_and_quiets_nans() {
        let mut e = Engine::new();
        let m = matrix(&mut e, SAMPLE);
        e.mem.set_u32(m + 0x10, 0x7f80_0001);
        e.call(0x00cd_4b60, &args![m]);
        assert_eq!(e.mem.u32(m + 4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(m + 0x10), 2.0);
        assert_eq!(e.mem.f32(m + 8), 7.0);
        assert_eq!(e.mem.f32(m + 0x20), 3.0);
        assert_eq!(e.mem.f32(m + 0x18), 8.0);
        assert_eq!(e.mem.f32(m + 0x24), 6.0);
        for diagonal in [0, 0x14, 0x28] {
            assert!(e.mem.f32(m + diagonal) >= 1.0);
        }
        // the padding lanes are untouched
        assert_eq!(e.mem.f32(m + 0xc), 9.0);
    }

    #[test]
    fn matrix_transpose_times_matrix() {
        let mut e = Engine::new();
        let a = matrix(&mut e, SAMPLE);
        let b = matrix(&mut e, IDENTITY);
        let this = e.mem.alloc(0x30);
        e.call(0x00cd_4bf0, &args![this, a, b]);
        assert_eq!(
            elements(&e, this),
            [[1.0, 4.0, 7.0], [2.0, 5.0, 8.0], [3.0, 6.0, 9.0]]
        );
        // lane 3 of the first row is the second element of the third column
        assert_eq!(e.mem.f32(this + 0xc), 8.0);
        let doubled = matrix(&mut e, [[2.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]]);
        e.call(0x00cd_4bf0, &args![this, a, doubled]);
        // column of b = (x, y, z): x * row_x + y * row_y + z * row_z, rows of a being (1,4,7), (2,5,8), (3,6,9)
        assert_eq!(
            elements(&e, this),
            [[2.0, 8.0, 14.0], [5.0, 11.0, 17.0], [3.0, 6.0, 9.0]]
        );
    }

    #[test]
    fn columns_scaled_by_lane_zero_of_a_vector() {
        let mut e = Engine::new();
        let m = matrix(&mut e, SAMPLE);
        let scale = e.mem.alloc(16);
        e.mem.set_f32(scale, 2.0);
        e.mem.set_f32(scale + 4, 100.0);
        e.call(0x00cd_4c70, &args![m, scale]);
        assert_eq!(
            elements(&e, m),
            [[2.0, 4.0, 6.0], [8.0, 10.0, 12.0], [14.0, 16.0, 18.0]]
        );
        assert_eq!(e.mem.f32(m + 0xc), 18.0);
    }

    #[test]
    fn cross_skew_symmetric_matrix() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let vector = e.mem.alloc(16);
        for (lane, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * lane as u32, v);
        }
        let m = e.mem.alloc(0x30);
        e.call(0x00cd_4cc0, &args![m, vector]);
        assert_eq!(
            elements(&e, m),
            [[0.0, 3.0, -2.0], [-3.0, 0.0, 1.0], [2.0, -1.0, 0.0]]
        );
        for padding in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(m + padding), 0);
        }
        // the negations are `-0.0 - v`: a zero component gives a negative zero
        for lane in 0..3 {
            e.mem.set_f32(vector + 4 * lane, 0.0);
        }
        e.call(0x00cd_4cc0, &args![m, vector]);
        assert_eq!(e.mem.u32(m + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(m + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(m + 0x24), 0x8000_0000);
        assert_eq!(e.mem.u32(m + 4), 0);
    }

    #[test]
    fn set_mul_by_a_vector_scales_the_columns_of_the_matrix() {
        let mut e = Engine::new();
        let b = matrix(&mut e, SAMPLE);
        let scale = e.mem.alloc(16);
        e.mem.set_f32(scale, 3.0);
        let this = e.mem.alloc(0x30);
        e.call(0x00cd_4d40, &args![this, scale, b]);
        assert_eq!(
            elements(&e, this),
            [[3.0, 6.0, 9.0], [12.0, 15.0, 18.0], [21.0, 24.0, 27.0]]
        );
        assert_eq!(e.mem.f32(this + 0x2c), 27.0);
    }

    #[test]
    fn add_mul_adds_the_scaled_columns() {
        let mut e = Engine::new();
        let b = matrix(&mut e, SAMPLE);
        let scale = e.mem.alloc(16);
        e.mem.set_f32(scale, 3.0);
        let this = matrix(&mut e, IDENTITY);
        e.call(0x00cd_4d90, &args![this, scale, b]);
        assert_eq!(
            elements(&e, this),
            [[4.0, 6.0, 9.0], [12.0, 16.0, 18.0], [21.0, 24.0, 28.0]]
        );
        assert_eq!(e.mem.f32(this + 0xc), 36.0);
    }

    #[test]
    fn symmetry_test_through_the_transpose_and_approximate_equality() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let symmetric = matrix(&mut e, [[1.0, 2.0, 3.0], [2.0, 5.0, 6.0], [3.0, 6.0, 9.0]]);
        assert!(e.call(0x00cd_4e90, &args![symmetric, 0.001f32]).bool());
        let off_by_a_little = matrix(&mut e, [[1.0, 2.0, 3.0], [2.01, 5.0, 6.0], [3.0, 6.0, 9.0]]);
        assert!(e.call(0x00cd_4e90, &args![off_by_a_little, 0.1f32]).bool());
        assert!(!e
            .call(0x00cd_4e90, &args![off_by_a_little, 0.001f32])
            .bool());
        // the matrix itself is not touched
        assert_eq!(e.mem.f32(off_by_a_little + 0x10), 2.01);
        // a NaN epsilon: nothing is within it
        assert!(!e.call(0x00cd_4e90, &args![symmetric, f32::NAN]).bool());
    }

    #[test]
    fn invert_gives_the_inverse_and_leaves_a_singular_matrix_alone() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let columns = [[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]];
        let m = matrix(&mut e, columns);
        assert_eq!(e.call(0x00cd_4f10, &args![m, 0.001f32]).u32(), 0);
        let inverse = elements(&e, m);
        let check = product(inverse, columns);
        for c in 0..3 {
            for r in 0..3 {
                assert!(close(check[c][r], IDENTITY[c][r]), "{check:?}");
            }
        }
        // a diagonal matrix inverts exactly
        let diagonal = matrix(&mut e, [[2.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 8.0]]);
        assert_eq!(e.call(0x00cd_4f10, &args![diagonal, 0.001f32]).u32(), 0);
        assert_eq!(
            elements(&e, diagonal),
            [[0.5, 0.0, 0.0], [0.0, 0.25, 0.0], [0.0, 0.0, 0.125]]
        );
        // singular: failure, unchanged
        let singular = matrix(&mut e, SAMPLE);
        assert_eq!(e.call(0x00cd_4f10, &args![singular, 0.001f32]).u32(), 1);
        assert_eq!(elements(&e, singular), SAMPLE);
        // the determinant must be above epsilon cubed: 64 is not above 5^3 = 125
        let diagonal = matrix(&mut e, [[2.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 8.0]]);
        assert_eq!(e.call(0x00cd_4f10, &args![diagonal, 5.0f32]).u32(), 1);
        assert_eq!(e.mem.f32(diagonal), 2.0);
        // a NaN epsilon fails
        assert_eq!(e.call(0x00cd_4f10, &args![diagonal, f32::NAN]).u32(), 1);
    }

    #[test]
    fn invert_symmetric_inverts_and_keeps_the_determinant_above_the_floor() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let columns = [[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let m = matrix(&mut e, columns);
        e.call(0x00cd_5050, &args![m]);
        let inverse = elements(&e, m);
        let check = product(inverse, columns);
        for c in 0..3 {
            for r in 0..3 {
                assert!(close(check[c][r], IDENTITY[c][r]), "{check:?}");
            }
        }
        // a determinant below 2^-69 is replaced by it
        let tiny = 1e-8f32;
        let small = matrix(
            &mut e,
            [[tiny, 0.0, 0.0], [0.0, tiny, 0.0], [0.0, 0.0, tiny]],
        );
        e.call(0x00cd_5050, &args![small]);
        let expected = (tiny * tiny) * f32::from_bits(0x6200_0000);
        assert_eq!(e.mem.f32(small), expected);
        assert_eq!(e.mem.f32(small + 0x14), expected);
        assert_eq!(e.mem.f32(small + 0x28), expected);
        // the stack-argument entry does the same
        let again = matrix(&mut e, columns);
        e.call(0x00cd_55a0, &args![again]);
        assert_eq!(elements(&e, again), inverse);
    }

    #[test]
    fn add_and_sub_work_on_all_four_lanes() {
        let mut e = Engine::new();
        let a = matrix(&mut e, SAMPLE);
        let b = matrix(&mut e, IDENTITY);
        e.call(0x00cd_5140, &args![a, b]);
        assert_eq!(
            elements(&e, a),
            [[2.0, 2.0, 3.0], [4.0, 6.0, 6.0], [7.0, 8.0, 10.0]]
        );
        assert_eq!(e.mem.f32(a + 0xc), 18.0);
        e.call(0x00cd_5180, &args![a, b]);
        assert_eq!(elements(&e, a), SAMPLE);
        assert_eq!(e.mem.f32(a + 0xc), 9.0);
    }

    #[test]
    fn determinant_lanes() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let m = matrix(&mut e, [[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);
        let out = e.mem.alloc(16);
        e.call(0x00cd_51c0, &args![m, out]);
        assert_eq!(load_lanes(&e, out), [1.0, 25.0, 25.0, 25.0]);
    }

    #[test]
    fn get_determinant_answers_the_out_vector() {
        // 00cd51e1 / 00cd525e: EAX holds `out` at the RET; the callers (00d7b807, 00e13714 ...)
        // read the determinant through it.
        let mut e = Engine::new();
        setup_constants(&mut e);
        let m = matrix(&mut e, [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]]);
        let out = e.mem.alloc(16);
        let answer = e.call(0x00cd_51c0, &args![m, out]).u32();
        assert_eq!(answer, out);
        assert_eq!(e.mem.f32(answer), 24.0);
    }

    #[test]
    fn jacobi_rotation_of_a_nonzero_element() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        // column 0 row 1 is the element at byte 4
        let m = matrix(&mut e, [[3.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let out = e.mem.alloc(16);
        e.call(0x00cd_5270, &args![0u32, 1u32, m, out]);
        assert!(close(e.mem.f32(out), 0.923_879_5));
        assert!(close(e.mem.f32(out + 4), 0.382_683_43));
        assert_eq!(e.mem.u32(out + 8), 1);
        assert_eq!(e.mem.u32(out + 0xc), 0);
        // a negative theta takes the other branch
        let m = matrix(&mut e, [[1.0, 1.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]]);
        e.call(0x00cd_5270, &args![0u32, 1u32, m, out]);
        assert!(close(e.mem.f32(out), 0.923_879_5));
        assert!(close(e.mem.f32(out + 4), -0.382_683_43));
    }

    #[test]
    fn jacobi_rotation_of_a_zero_element_is_the_identity() {
        let mut e = Engine::new();
        // The exe's sqrt/acos error path (a NaN or negative argument) sets errno, a word of
        // the thread's data, which the bare engine does not have: doubled.
        let errno = e.mem.alloc(4);
        e.register_double(0x00ec_85e3, move |_, _| errno.into_ret());
        setup_constants(&mut e);
        let m = matrix(&mut e, [[3.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let out = e.mem.alloc(16);
        e.mem.set_u32(out + 4, 0x1234_5678);
        e.call(0x00cd_5270, &args![0u32, 1u32, m, out]);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 8), 1);
        assert_eq!(e.mem.u32(out + 0xc), 0);
        // a NaN element is not zero: the rotation is computed (and is NaN)
        e.mem.set_f32(m + 4, f32::NAN);
        e.call(0x00cd_5270, &args![0u32, 1u32, m, out]);
        assert!(e.mem.f32(out).is_nan());
    }

    #[test]
    fn frobenius_norm() {
        let mut e = Engine::new();
        // The exe's sqrt/acos error path (a NaN or negative argument) sets errno, a word of
        // the thread's data, which the bare engine does not have: doubled.
        let errno = e.mem.alloc(4);
        e.register_double(0x00ec_85e3, move |_, _| errno.into_ret());
        let m = matrix(&mut e, [[1.0, 2.0, 2.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
        assert_eq!(e.call(0x00cd_5340, &args![m]).f64(), 3.0);
        // a NaN element gives a NaN
        e.mem.set_f32(m + 0x28, f32::NAN);
        assert!(e.call(0x00cd_5340, &args![m]).f64().is_nan());
    }

    #[test]
    fn off_diagonal_norm_ignores_the_diagonal() {
        let mut e = Engine::new();
        let m = matrix(
            &mut e,
            [[100.0, 4.0, 0.0], [3.0, 100.0, 0.0], [0.0, 0.0, 100.0]],
        );
        assert_eq!(e.call(0x00cd_53a0, &args![m]).f64(), 5.0);
    }

    #[test]
    fn largest_element_above_the_diagonal() {
        let mut e = Engine::new();
        let m = matrix(
            &mut e,
            [[0.0, 100.0, 0.0], [1.0, 0.0, 0.0], [-5.0, 2.0, 0.0]],
        );
        let (column, row) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(column, 0xaaaa);
        e.mem.set_u32(row, 0xbbbb);
        e.call(0x00cd_5400, &args![m, column, row]);
        // the element in column 2, row 0 (the lower element of column 0 is not considered)
        assert_eq!((e.mem.u32(column), e.mem.u32(row)), (2, 0));
        // equal magnitudes: the later element wins
        let m = matrix(&mut e, [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, -2.0, 0.0]]);
        e.call(0x00cd_5400, &args![m, column, row]);
        assert_eq!((e.mem.u32(column), e.mem.u32(row)), (2, 1));
        // an all-zero matrix replaces the zero maximum by every element
        let m = matrix(&mut e, [[0.0; 3]; 3]);
        e.call(0x00cd_5400, &args![m, column, row]);
        assert_eq!((e.mem.u32(column), e.mem.u32(row)), (2, 1));
        // NaN elements never replace it
        let m = matrix(&mut e, [[f32::NAN; 3]; 3]);
        e.mem.set_u32(column, 0xaaaa);
        e.mem.set_u32(row, 0xbbbb);
        e.call(0x00cd_5400, &args![m, column, row]);
        assert_eq!((e.mem.u32(column), e.mem.u32(row)), (0xaaaa, 0xbbbb));
    }

    #[test]
    fn rotation_matrix_from_a_record() {
        let mut e = Engine::new();
        let record = e.mem.alloc(16);
        e.mem.set_f32(record, 0.8);
        e.mem.set_f32(record + 4, 0.6);
        e.mem.set_u32(record + 8, 0);
        e.mem.set_u32(record + 0xc, 1);
        let out = matrix(&mut e, [[7.0; 3]; 3]);
        e.call(0x00cd_54d0, &args![record, out]);
        assert_eq!(
            elements(&e, out),
            [[0.8, -0.6, 0.0], [0.6, 0.8, 0.0], [0.0, 0.0, 1.0]]
        );
        for padding in [0xc, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(out + padding), 0);
        }
        // indices 2 and 0, with a signalling NaN for the sine
        e.mem.set_u32(record + 8, 2);
        e.mem.set_u32(record + 0xc, 0);
        e.mem.set_u32(record + 4, 0x7f80_0001);
        e.call(0x00cd_54d0, &args![record, out]);
        // p + q * 4 = 2: column 0 row 2 is s; q + p * 4 = 8: column 2 row 0 is -s
        assert_eq!(e.mem.u32(out + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(out + 0x20), 0xffc0_0001);
        assert_eq!(e.mem.f32(out), 0.8);
        assert_eq!(e.mem.f32(out + 0x28), 0.8);
        assert_eq!(e.mem.f32(out + 0x14), 1.0);
    }

    #[test]
    fn set_mul_mat3_mat3_is_a_times_b_into_the_destination() {
        let mut e = Engine::new();
        let a = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]]);
        let b = matrix(&mut e, SAMPLE);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cd_5530, &args![dest, a, b]);
        assert_eq!(
            elements(&e, dest),
            [[1.0, -3.0, 2.0], [4.0, -6.0, 5.0], [7.0, -9.0, 8.0]]
        );
    }

    #[test]
    fn set_mul_inverse_multiplies_by_the_transpose() {
        let mut e = Engine::new();
        let a = matrix(&mut e, SAMPLE);
        let b = matrix(&mut e, [[1.0, 2.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        let this = e.mem.alloc(0x30);
        e.call(0x00cd_5610, &args![this, a, b]);
        assert_eq!(
            elements(&e, this),
            [[1.0, 2.0, 3.0], [6.0, 9.0, 12.0], [7.0, 8.0, 9.0]]
        );
    }

    #[test]
    fn change_basis_rotates_a_diagonal_matrix() {
        let mut e = Engine::new();
        let this = matrix(&mut e, [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
        let basis = matrix(&mut e, [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        e.call(0x00cd_5880, &args![this, basis]);
        assert_eq!(
            elements(&e, this),
            [[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 3.0]]
        );
    }

    #[test]
    fn diagonalising_a_symmetric_matrix() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let columns = [[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 3.0]];
        let this = matrix(&mut e, columns);
        let vectors = e.mem.alloc(0x30);
        let values = e.mem.alloc(16);
        let result = e
            .call(0x00cd_56c0, &args![this, vectors, values, 20i32, 1e-6f32])
            .u32();
        assert_eq!(result, 0);
        let found = load_lanes(&e, values);
        let mut sorted = [found[0], found[1], found[2]];
        sorted.sort_by(f32::total_cmp);
        assert!(close(sorted[0], 1.0) && close(sorted[1], 3.0) && close(sorted[2], 3.0));
        assert_eq!(found[3], 0.0);
        // M * V = V * D: for each eigenvector column
        let v = elements(&e, vectors);
        let mv = product(columns, v);
        for i in 0..3 {
            for r in 0..3 {
                assert!(
                    close(mv[i][r], found[i] * v[i][r]),
                    "{mv:?} {v:?} {found:?}"
                );
            }
        }
        // the matrix itself is unchanged
        assert_eq!(elements(&e, this), columns);
    }

    #[test]
    fn diagonalising_stops_at_the_iteration_limit_and_rejects_asymmetry() {
        let mut e = Engine::new();
        setup_constants(&mut e);
        let this = matrix(&mut e, [[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 3.0]]);
        let vectors = e.mem.alloc(0x30);
        let values = e.mem.alloc(16);
        // no iteration allowed: not converged (1), the diagonal of the copy, identity vectors
        let result = e
            .call(0x00cd_56c0, &args![this, vectors, values, 0i32, 1e-6f32])
            .u32();
        assert_eq!(result, 1);
        assert_eq!(load_lanes(&e, values), [2.0, 2.0, 3.0, 0.0]);
        assert_eq!(elements(&e, vectors), IDENTITY);
        // a diagonal matrix is converged at once
        let diagonal = matrix(&mut e, [[2.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 0.0, 3.0]]);
        let result = e
            .call(
                0x00cd_56c0,
                &args![diagonal, vectors, values, 0i32, 1e-6f32],
            )
            .u32();
        assert_eq!(result, 0);
        assert_eq!(load_lanes(&e, values), [2.0, 5.0, 3.0, 0.0]);
        // not symmetric: 1, outputs untouched
        let skewed = matrix(&mut e, SAMPLE);
        e.mem.set_f32(vectors, 77.0);
        e.mem.set_f32(values, 88.0);
        let result = e
            .call(0x00cd_56c0, &args![skewed, vectors, values, 20i32, 1e-6f32])
            .u32();
        assert_eq!(result, 1);
        assert_eq!(e.mem.f32(vectors), 77.0);
        assert_eq!(e.mem.f32(values), 88.0);
    }
}
