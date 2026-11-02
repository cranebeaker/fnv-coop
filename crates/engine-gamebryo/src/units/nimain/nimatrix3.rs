//! `NiMain/nimatrix3.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `NiMatrix3` (Xbox PDB) is nine floats, row by row: m00 +0, m01 +4, m02 +8, m10 +0xc, m11
// +0x10, m12 +0x14, m20 +0x18, m21 +0x1c, m22 +0x20.

/// `NiMatrix3::operator*` (`thiscall` on the left matrix, (result, right); answers the
/// result).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiFastATan2` (`cdecl`, two floats; answers in `ST0`).
const FAST_ATAN2: u32 = 0x00a8_11f0;
/// `00ec9650` and `00eca1d0` (the CRT's `asin` and `acos`; the argument and the result in
/// `ST0`).
const ASIN: u32 = 0x00ec_9650;
const ACOS: u32 = 0x00ec_a1d0;
/// The double -1 (`0101a6b0`), the float pi (`0109afc0`), the float pi/2 (`011a9444`) and the
/// double at `01012060` the angle extraction subtracts.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const PI_FLOAT: u32 = 0x0109_afc0;
const HALF_PI_FLOAT: u32 = 0x011a_9444;
const ANGLE_OFFSET_DOUBLE: u32 = 0x0101_2060;

/// `sin` and `cos` of an angle as `FSINCOS` leaves them, each stored to `float` (computed in
/// `f64`, which can differ from the extended-precision result in the last bit of a float).
fn sin_cos(angle: f64) -> (u32, u32) {
    (x87_store32(angle.sin()), x87_store32(angle.cos()))
}

/// Writes nine words at `at`.
fn put9(e: &mut Engine, at: u32, words: [u32; 9]) {
    for (i, w) in words.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *w);
    }
}

/// The rotation matrices the Euler constructors build from an angle's `float` sine and
/// cosine: about x `1 0 0 / 0 c s / 0 -s c`, about y `c 0 -s / 0 1 0 / s 0 c`, about z
/// `c s 0 / -s c 0 / 0 0 1`.
fn about_x(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [one, zero, zero, zero, cos, sin, zero, minus_sin, cos]
}

fn about_y(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [cos, zero, minus_sin, zero, one, zero, sin, zero, cos]
}

fn about_z(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [cos, sin, zero, minus_sin, cos, zero, zero, zero, one]
}

/// `this = outer * (middle * inner)` through `NiMatrix3::operator*`, the middle matrix's
/// buffer taking the second product as the code does, then copied into `this`.
fn compose(e: &mut Engine, this: Ptr, outer: [u32; 9], middle: [u32; 9], inner: [u32; 9]) {
    e.with_stack(0x90, |e, locals| {
        let (o, m, i, r) = (
            locals.addr(),
            locals.addr() + 0x24,
            locals.addr() + 0x48,
            locals.addr() + 0x6c,
        );
        put9(e, o, outer);
        put9(e, m, middle);
        put9(e, i, inner);
        let first = e.call(MATRIX_MULTIPLY, &args![m, r, i]).u32();
        let second = e.call(MATRIX_MULTIPLY, &args![o, m, first]).u32();
        for word in 0..9 {
            let value = e.mem.u32(second + 4 * word);
            e.mem.set_u32(this.addr() + 4 * word, value);
        }
    });
}

// Translated from 00a582f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::TransformVertices` (Xbox PDB; `cdecl`, (matrix, translation, count, source,
/// destination)): each destination vertex is the matrix times the source vertex plus the
/// translation, `x = m00 sx + tx + m01 sy + sz m02`, `y = m10 sx + ty + m11 sy + m12 sz`,
/// `z = m20 sx + tz + sy m21 + m22 sz` in the exe's order, each component read from memory
/// when used (so a transform in place sees the components already written). The exe does
/// four vertices per loop pass while at least four remain (a signed count test), the rest
/// one by one; the order of reads and writes is the same.
pub fn ni_matrix3_transform_vertices(
    e: &mut Engine,
    matrix: Ptr,
    translation: Ptr,
    count: u32,
    source: Ptr,
    destination: Ptr,
) {
    let (m, t) = (matrix.addr(), translation.addr());
    let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let md = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    // The unrolled passes (the largest multiple of four vertices, when the count is at least
    // four as a signed number) and the tail (up to the count, unsigned) visit every vertex in
    // order: one loop here.
    for index in 0..count {
        let s = source.addr().wrapping_add(index.wrapping_mul(12));
        let d = destination.addr().wrapping_add(index.wrapping_mul(12));
        let x = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m), md(e, s)), md(e, t)),
                x87_mul(ld(e, m + 4), md(e, s + 4)),
            ),
            x87_mul(ld(e, s + 8), md(e, m + 8)),
        );
        e.mem.set_u32(d, x87_store32(x));
        let y = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m + 0xc), md(e, s)), md(e, t + 4)),
                x87_mul(ld(e, m + 0x10), md(e, s + 4)),
            ),
            x87_mul(ld(e, m + 0x14), md(e, s + 8)),
        );
        e.mem.set_u32(d + 4, x87_store32(y));
        let z = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m + 0x18), md(e, s)), md(e, t + 8)),
                x87_mul(ld(e, s + 4), md(e, m + 0x1c)),
            ),
            x87_mul(ld(e, m + 0x20), md(e, s + 8)),
        );
        e.mem.set_u32(d + 8, x87_store32(z));
    }
}

// Translated from 00a58510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `cdecl`, one float; answers in `ST0`):
/// the arc cosine clamped to its domain: pi (the float at `0109afc0`) for a value at or
/// below -1 (the double at `0101a6b0`) or NaN, 0 at or above 1, otherwise `acos`
/// (`00eca1d0`) stored as a `float`.
pub fn fn_00a58510(e: &mut Engine, value: f32) -> f32 {
    let v = x87_load32(value.to_bits());
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    if !(v > minus_one) {
        return f32::from_bits(x87_quiet_bits(e.mem.u32(PI_FLOAT)));
    }
    if !(1.0 > v) {
        return 0.0;
    }
    let angle = e.call(ACOS, &args![v]).f64();
    f32::from_bits(x87_store32(angle))
}

// Translated from 00a592c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::ToEulerAnglesXYZ` (Xbox PDB): the y angle `-asin(m02)` (the arc sine through
/// `00ec9650`, `m02` clamped: pi/2, the float at `011a9444`, at or below -1 or NaN, -pi/2 at
/// or above 1); strictly between -pi/2 and pi/2 the x angle `-atan2(-m12, m22)` and the z
/// angle `-atan2(-m01, m00)` (`NiFastATan2`) and the answer 1; at -pi/2 or below the z angle
/// 0 and the x angle `atan2(m10, m11)` minus the double at `01012060`; at pi/2 or above (or
/// NaN) the z angle 0 and the x angle `0 - atan2(m10, m11)`; those answer 0.
pub fn ni_matrix3_to_euler_angles_xyz(
    e: &mut Engine,
    this: Ptr,
    x_angle: Ptr,
    y_angle: Ptr,
    z_angle: Ptr,
) -> bool {
    let m = this.addr();
    let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let a = x87_load32(x87_store32(ld(e, m + 8)));
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    let half_pi_bits = e.mem.u32(HALF_PI_FLOAT);
    let local = if !(a > minus_one) {
        x87_store32(-x87_load32(half_pi_bits))
    } else if !(1.0 > a) {
        x87_store32(x87_load32(half_pi_bits))
    } else {
        let angle = e.call(ASIN, &args![a]).f64();
        x87_store32(x87_load32(x87_store32(angle)))
    };
    let half_pi = x87_load32(e.mem.u32(HALF_PI_FLOAT));
    let y = x87_load32(x87_store32(-x87_load32(local)));
    e.mem.set_u32(y_angle.addr(), x87_store32(y));
    let atan2 =
        |e: &mut Engine, first: u32, second: u32| e.call(FAST_ATAN2, &args![first, second]).f64();
    let negated = |e: &Engine, at: u32| x87_store32(-x87_load32(e.mem.u32(at)));
    let copied = |e: &Engine, at: u32| x87_store32(x87_load32(e.mem.u32(at)));
    if y < half_pi {
        if -half_pi < y {
            let (first, second) = (negated(e, m + 0x14), copied(e, m + 0x20));
            let x = atan2(e, first, second);
            e.mem.set_u32(x_angle.addr(), x87_store32(-x));
            let (first, second) = (negated(e, m + 4), copied(e, m));
            let z = atan2(e, first, second);
            e.mem.set_u32(z_angle.addr(), x87_store32(-z));
            return true;
        }
        let (first, second) = (copied(e, m + 0xc), copied(e, m + 0x10));
        let r = x87_store32(atan2(e, first, second));
        e.mem.set_u32(z_angle.addr(), x87_store32(0.0));
        let offset = f64::from_bits(e.mem.u64(ANGLE_OFFSET_DOUBLE));
        e.mem
            .set_u32(x_angle.addr(), x87_store32(x87_sub(x87_load32(r), offset)));
        return false;
    }
    let (first, second) = (copied(e, m + 0xc), copied(e, m + 0x10));
    let r = x87_store32(atan2(e, first, second));
    e.mem.set_u32(z_angle.addr(), x87_store32(0.0));
    e.mem
        .set_u32(x_angle.addr(), x87_store32(x87_sub(0.0, x87_load32(r))));
    false
}

// Translated from 00a59540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB): the rotations about x, y and z by the three
/// angles (sines and cosines by `FSINCOS`, stored as floats), combined as `X * (Y * Z)`
/// through `NiMatrix3::operator*`, copied into `this`.
pub fn ni_matrix3_from_euler_angles_xyz(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    let (sx, cx) = sin_cos(x87_load32(x.to_bits()));
    let (sy, cy) = sin_cos(x87_load32(y.to_bits()));
    let (sz, cz) = sin_cos(x87_load32(z.to_bits()));
    compose(e, this, about_x(sx, cx), about_y(sy, cy), about_z(sz, cz));
}

// Translated from 00a59780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::FromEulerAnglesZYX` (Xbox PDB; the angles about z, y and x in that order):
/// the three rotations combined as `Z * (Y * X)` through `NiMatrix3::operator*`, copied into
/// `this`.
pub fn ni_matrix3_from_euler_angles_zyx(e: &mut Engine, this: Ptr, z: f32, y: f32, x: f32) {
    let (sx, cx) = sin_cos(x87_load32(x.to_bits()));
    let (sy, cy) = sin_cos(x87_load32(y.to_bits()));
    let (sz, cz) = sin_cos(x87_load32(z.to_bits()));
    compose(e, this, about_z(sz, cz), about_y(sy, cy), about_x(sx, cx));
}

// Translated from 00a59a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::LoadBinary` (Xbox PDB): the nine 4-byte entries, row by row, from the
/// stream's input (`NiStream` +0x24c, its read function +8 and position +4), the
/// byte-count word a local set to 4.
pub fn ni_matrix3_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.with_stack(4, |e, count| {
        for word in 0..9 {
            let input = e.mem.u32(stream.addr() + 0x24c);
            let function = e.mem.u32(input + 8);
            e.mem.set_u32(count.addr(), 4);
            let destination = this.addr() + 4 * word;
            let done = e
                .call(function, &args![input, destination, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(input + 4).wrapping_add(done);
            e.mem.set_u32(input + 4, position);
        }
    });
}

/// The CRT square root helper (`00ec6040`; the argument and the result in `ST0`).
const SQUARE_ROOT: u32 = 0x00ec_6040;
/// The doubles 1.0 (`01012070`), 0.5 (`01011588`) and 4.0 (`0101db80`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const HALF_DOUBLE: u32 = 0x0101_1588;
const FOUR_DOUBLE: u32 = 0x0101_db80;
/// The float 1e-6 (`010959f4`) the axis extraction compares lengths and errors with.
const AXIS_EPSILON: u32 = 0x0109_59f4;
/// The float 1e-8 (`0109afc4`) below which entries count as zero.
const ZERO_EPSILON: u32 = 0x0109_afc4;
/// `"%s = ((%g,%g,%g),(%g,%g,%g),(%g,%g,%g))"` (`0109afc8`).
const VIEWER_FORMAT: u32 = 0x0109_afc8;
/// The Ni allocator (`00aa1070`, `cdecl`, the size) and `NiSprintf` (`00a305b0`, `cdecl`:
/// buffer, size, format, arguments).
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiStream::m_pkOstr` (+0x250, the `NiBinaryStream*` written to) and the stream's write
/// function (+0xc) and position (+4).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
const BINARY_STREAM_POSITION: u32 = 0x4;

/// `FLD dword` of the float at `at`.
fn ld(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// The float at `at` as a memory operand of an arithmetic instruction.
fn op(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// `FSTP dword` of `value` at `at`.
fn st(e: &mut Engine, at: u32, value: f64) {
    e.mem.set_u32(at, x87_store32(value));
}

/// An `FSTP dword` into a local and the `FLD dword` back.
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FLD dword [at]; FABS; FSTP dword`: the magnitude of the float at `at`, as float bits.
fn magnitude_bits(e: &Engine, at: u32) -> u32 {
    x87_store32(ld(e, at).abs())
}

/// The square root as the code takes it: the value stored to a float local and loaded,
/// the CRT helper, the result stored to a float local and loaded.
fn float_square_root(e: &mut Engine, value: f64) -> f64 {
    let root = e.call(SQUARE_ROOT, &args![through_float(value)]).f64();
    through_float(root)
}

// Translated from 00a58550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `thiscall`, (angle, x, y, z)): the
/// rotation angle and axis of the matrix. The angle is the arc cosine of `(trace - 1) / 2`
/// (pi, the float at `0109afc0`, at or below -1 or NaN; 0 at or above 1; otherwise `acos`,
/// `00eca1d0`); the axis is `(m12 - m21, m20 - m02, m01 - m10)` divided by its length
/// (square root `00ec6040`) when that length is above 1e-6 (`010959f4`). Otherwise an angle
/// at most 1 gives the axis `(1, 0, 0)`; a larger one (a half turn) the axis
/// `sqrt((m00 + 1) / 2)`, `sqrt((m11 + 1) / 2)`, `sqrt((m22 + 1) / 2)`, whose z is negated
/// when the matrix does not map the axis onto itself (squared error of `M v - v` at least
/// 1e-6), and then y as well when the error stays that large. After negating y the code
/// computes the three rows of `M v - v` once more into its stack locals (each stored as a
/// float, then as a double into one slot) and never reads them; translated, with no effect.
///
/// The errors only decide comparisons, so the NaN a product picks there does not matter:
/// they are computed with every matrix entry as a memory operand.
pub fn fn_00a58550(e: &mut Engine, this: Ptr, angle: Ptr, x: Ptr, y: Ptr, z: Ptr) {
    let m = this.addr();
    let (x, y, z) = (x.addr(), y.addr(), z.addr());
    let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
    let half = f64::from_bits(e.mem.u64(HALF_DOUBLE));
    let trace = x87_store32(x87_add(x87_add(ld(e, m), op(e, m + 0x10)), op(e, m + 0x20)));
    let cosine = through_float(x87_mul(x87_sub(x87_load32(trace), one), half));
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    let value = if !(cosine > minus_one) {
        ld(e, PI_FLOAT)
    } else if !(1.0 > cosine) {
        0.0
    } else {
        through_float(e.call(ACOS, &args![cosine]).f64())
    };
    st(e, angle.addr(), through_float(value));

    st(e, x, x87_sub(ld(e, m + 0x14), op(e, m + 0x1c)));
    st(e, y, x87_sub(ld(e, m + 0x18), op(e, m + 8)));
    let z_value = through_float(x87_sub(ld(e, m + 4), op(e, m + 0xc)));
    st(e, z, z_value);
    let (xv, yv) = (ld(e, x), ld(e, y));
    let squares = x87_add(
        x87_mul(z_value, z_value),
        x87_add(x87_mul(xv, xv), x87_mul(yv, yv)),
    );
    let length = float_square_root(e, squares);
    let epsilon = ld(e, AXIS_EPSILON);
    if length > epsilon {
        let inverse = through_float(x87_div(1.0, length));
        st(e, x, x87_mul(ld(e, x), inverse));
        st(e, y, x87_mul(ld(e, y), inverse));
        st(e, z, x87_mul(inverse, op(e, z)));
        return;
    }
    if !(1.0 < op(e, angle.addr())) {
        st(e, x, 1.0);
        st(e, y, 0.0);
        st(e, z, 0.0);
        return;
    }
    for (row, out) in [(m, x), (m + 0x10, y)] {
        let half_sum = x87_mul(x87_add(ld(e, row), one), half);
        let root = float_square_root(e, half_sum);
        st(e, out, root);
    }
    let half_sum = x87_mul(x87_add(ld(e, m + 0x20), one), half);
    let z_value = float_square_root(e, half_sum);
    st(e, z, z_value);
    if !(epsilon > axis_error(e, m, ld(e, x), ld(e, y), z_value)) {
        let negated = through_float(-z_value);
        st(e, z, negated);
        if !(epsilon > axis_error(e, m, ld(e, x), ld(e, y), negated)) {
            let negated = through_float(-ld(e, y));
            st(e, y, negated);
            // The unused rows (`00a58803`..`00a5884c`), in the exe's operand order.
            let x_row = x87_add(
                x87_add(x87_mul(ld(e, x), op(e, m)), x87_mul(negated, op(e, m + 4))),
                x87_mul(ld(e, m + 8), op(e, z)),
            );
            let _x_error = through_float(x87_sub(x_row, op(e, x)));
            let y_row = x87_add(
                x87_add(
                    x87_mul(negated, op(e, m + 0x10)),
                    x87_mul(ld(e, m + 0xc), op(e, x)),
                ),
                x87_mul(ld(e, z), op(e, m + 0x14)),
            );
            let _y_error = through_float(x87_sub(y_row, negated));
            let z_row = x87_add(
                x87_add(
                    x87_mul(negated, op(e, m + 0x1c)),
                    x87_mul(ld(e, m + 0x18), op(e, x)),
                ),
                x87_mul(ld(e, z), op(e, m + 0x20)),
            );
            let _z_error = through_float(x87_sub(z_row, op(e, z)));
        }
    }
}

/// The squared length of `M v - v` for `v = (x, y, z)`, each component stored to a float
/// and the sum too (`00a58550`).
fn axis_error(e: &Engine, m: u32, x: f64, y: f64, z: f64) -> f64 {
    let row = |row: u32, own: f64| {
        let sum = x87_add(
            x87_add(x87_mul(x, op(e, row)), x87_mul(y, op(e, row + 4))),
            x87_mul(z, op(e, row + 8)),
        );
        through_float(x87_sub(sum, own))
    };
    let (ex, ey, ez) = (row(m, x), row(m + 0xc, y), row(m + 0x18, z));
    through_float(x87_add(
        x87_add(x87_mul(ey, ey), x87_mul(ex, ex)),
        x87_mul(ez, ez),
    ))
}

// Translated from 00a588a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::TransposeTimes` (Xbox PDB; (result, other)): `result = this^T * other`, each
/// entry `this[0][c] * other[0][k] + this[1][c] * other[1][k] + this[2][c] * other[2][k]`
/// in the exe's order and stored when computed (so a result that overlaps an operand sees
/// the entries already written). Answers `result`.
pub fn ni_matrix3_transpose_times(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let (a, b, r) = (this.addr(), other.addr(), result.addr());
    // Per result entry, its three products as (loaded float, memory operand), summed in this
    // order.
    type Product = (u32, u32);
    let entries: [(u32, [Product; 3]); 9] = [
        (0x00, [(b, a), (b + 0xc, a + 0xc), (b + 0x18, a + 0x18)]),
        (
            0x0c,
            [(a + 0x10, b + 0xc), (a + 4, b), (b + 0x18, a + 0x1c)],
        ),
        (
            0x18,
            [(a + 0x14, b + 0xc), (a + 8, b), (b + 0x18, a + 0x20)],
        ),
        (
            0x04,
            [(b + 4, a), (b + 0x10, a + 0xc), (a + 0x18, b + 0x1c)],
        ),
        (
            0x10,
            [(b + 0x10, a + 0x10), (b + 4, a + 4), (b + 0x1c, a + 0x1c)],
        ),
        (
            0x1c,
            [(b + 0x10, a + 0x14), (b + 4, a + 8), (b + 0x1c, a + 0x20)],
        ),
        (
            0x08,
            [(b + 8, a), (b + 0x14, a + 0xc), (a + 0x18, b + 0x20)],
        ),
        (
            0x14,
            [(b + 0x14, a + 0x10), (b + 8, a + 4), (b + 0x20, a + 0x1c)],
        ),
        (
            0x20,
            [(b + 0x14, a + 0x14), (b + 8, a + 8), (b + 0x20, a + 0x20)],
        ),
    ];
    for (offset, products) in entries {
        let term = |(loaded, operand): Product| x87_mul(ld(e, loaded), op(e, operand));
        let sum = x87_add(
            x87_add(term(products[0]), term(products[1])),
            term(products[2]),
        );
        st(e, r + offset, sum);
    }
    result
}

// Translated from 00a58990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `thiscall`, (diagonal, sub-diagonal),
/// answers in `AL`): reduces the symmetric matrix to tridiagonal form with one Householder
/// rotation of rows and columns 1 and 2. The diagonal gets `m00`; when `|m02|` is at least
/// 1e-8 (`0109afc4`) the rotation by `(m01, m02) / L`, `L = sqrt(m01^2 + m02^2)` (`00ec6040`)
/// gives the diagonal `m11 + q b`, `m22 - q b` and the sub-diagonal `L`, `m12 - q a` (`a`,
/// `b` the normalized pair, `q = 2 a m12 + (m22 - m11) b`), leaves the rotation in the
/// matrix (`1 0 0 / 0 a b / 0 b -a`) and answers true. Otherwise (NaN too) the diagonal gets
/// `m11`, `m22`, the sub-diagonal `m01`, `m12`, the matrix becomes the identity and the
/// answer is false.
pub fn fn_00a58990(e: &mut Engine, this: Ptr, diagonal: Ptr, sub_diagonal: Ptr) -> bool {
    let (m, d, s) = (this.addr(), diagonal.addr(), sub_diagonal.addr());
    let quiet = |e: &Engine, at: u32| x87_quiet_bits(e.mem.u32(at));
    let (a, b) = (quiet(e, m + 4), quiet(e, m + 8));
    let (c11, c12, c22) = (quiet(e, m + 0x10), quiet(e, m + 0x14), quiet(e, m + 0x20));
    let first = quiet(e, m);
    e.mem.set_u32(d, first);
    let bv = x87_load32(b);
    let magnitude = through_float(bv.abs());
    let one = x87_store32(1.0);
    let zero = x87_store32(0.0);
    if !(magnitude >= op(e, ZERO_EPSILON)) {
        e.mem.set_u32(d + 4, c11);
        e.mem.set_u32(d + 8, c22);
        e.mem.set_u32(s, a);
        e.mem.set_u32(s + 4, c12);
        put9(e, m, [one, zero, zero, zero, one, zero, zero, zero, one]);
        return false;
    }
    let av = x87_load32(a);
    let length = float_square_root(e, x87_add(x87_mul(bv, bv), x87_mul(av, av)));
    st(e, s, length);
    let inverse = through_float(x87_div(1.0, length));
    let a = x87_store32(x87_mul(inverse, x87_m32(a)));
    let b = x87_store32(x87_mul(inverse, x87_m32(b)));
    let (av, bv) = (x87_load32(a), x87_load32(b));
    let (c11, c12, c22) = (x87_load32(c11), x87_load32(c12), x87_load32(c22));
    let difference_b = x87_mul(x87_sub(c22, c11), bv);
    let q = through_float(x87_add(x87_mul(x87_add(av, av), c12), difference_b));
    let qb = x87_mul(q, bv);
    st(e, d + 4, x87_add(c11, qb));
    st(e, d + 8, x87_sub(c22, qb));
    st(e, s + 4, x87_sub(c12, x87_mul(q, av)));
    let minus_a = x87_store32(-av);
    put9(e, m, [one, zero, zero, zero, a, b, zero, b, minus_a]);
    true
}

/// How the code reads the first entry `a` of a rotated pair in its two products: loaded by
/// `FLD` in both, as a memory operand in both, or as a memory operand in the first and
/// loaded in the second (`00a58b00`; this only decides which NaN a product gives).
#[derive(Clone, Copy)]
enum PairRead {
    Loaded,
    Operand,
    OperandThenLoaded,
}

/// One row of a plane rotation of two matrix columns (`00a58b00`): with `b` the entry at
/// `b_at` as it was, `b = a s + b c` and then `a = a c - b s`, each stored as a float.
fn rotate_pair(e: &mut Engine, a_at: u32, b_at: u32, cosine: f64, sine: f64, read: PairRead) {
    let old_b = ld(e, b_at);
    let a_first = match read {
        PairRead::Loaded => ld(e, a_at),
        PairRead::Operand | PairRead::OperandThenLoaded => op(e, a_at),
    };
    st(
        e,
        b_at,
        x87_add(x87_mul(a_first, sine), x87_mul(old_b, cosine)),
    );
    let a_second = match read {
        PairRead::Loaded | PairRead::OperandThenLoaded => ld(e, a_at),
        PairRead::Operand => op(e, a_at),
    };
    st(
        e,
        a_at,
        x87_sub(x87_mul(a_second, cosine), x87_mul(old_b, sine)),
    );
}

/// The cosine and sine of the rotation that zeroes `bottom` against `top` (`00a58b00`):
/// with `|top| <= |bottom|` (as floats) `t = top / bottom`, sine `1 / r`, cosine `t sine`;
/// otherwise (NaN too) `t = bottom / top`, cosine `1 / r`, sine `t cosine`; `r` is the
/// square root of `t^2 + one`. Answers (cosine, sine, r, whether `|top| <= |bottom|`).
fn plane_rotation(e: &mut Engine, top: f64, bottom: f64, one: f64) -> (f64, f64, f64, bool) {
    let bottom_size = through_float(bottom.abs());
    let top_size = through_float(top.abs());
    let top_smaller = top_size <= bottom_size;
    let ratio = if top_smaller {
        through_float(x87_div(top, bottom))
    } else {
        through_float(x87_div(bottom, top))
    };
    let root = float_square_root(e, x87_add(x87_mul(ratio, ratio), one));
    let first = through_float(x87_div(1.0, root));
    let second = through_float(x87_mul(ratio, first));
    if top_smaller {
        (second, first, root, true)
    } else {
        (first, second, root, false)
    }
}

/// The 2x2 block `00a58b00` finishes on: the diagonal entries `low` and `high` (offsets
/// into the diagonal), the sub-diagonal entry between them, the other sub-diagonal entry
/// (zeroed first), how the sum of the two diagonal entries is read (loaded entry, operand
/// entry), and the three rows of the rotated column pair.
struct FinalBlock {
    low: u32,
    high: u32,
    between: u32,
    other: u32,
    sum: (u32, u32),
    rows: [(u32, u32, PairRead); 3],
}

/// `00a58b00`'s exit once one sub-diagonal entry has vanished: that entry becomes 0; unless
/// the other vanishes too, the 2x2 block's eigenvalues `(d0 + d1 -/+ sqrt((d0 - d1)^2 + 4
/// e^2)) / 2` go to the diagonal and the matching rotation to the matrix; then the other
/// entry becomes 0 too. Answers true.
fn finish_block(e: &mut Engine, m: u32, d: u32, s: u32, block: &FinalBlock) -> bool {
    let (low, high, between) = (d + block.low, d + block.high, s + block.between);
    st(e, s + block.other, 0.0);
    let low_size = x87_load32(magnitude_bits(e, low));
    let sizes = x87_load32(x87_store32(x87_add(
        low_size,
        x87_m32(magnitude_bits(e, high)),
    )));
    let between_size = x87_load32(magnitude_bits(e, between));
    if sizes == x87_add(between_size, sizes) {
        st(e, between, 0.0);
        return true;
    }
    let sum = through_float(x87_add(ld(e, d + block.sum.0), op(e, d + block.sum.1)));
    let difference = through_float(x87_sub(ld(e, low), op(e, high)));
    let four = f64::from_bits(e.mem.u64(FOUR_DOUBLE));
    let quadruple = x87_mul(x87_mul(ld(e, between), four), op(e, between));
    let discriminant = x87_add(x87_mul(difference, difference), quadruple);
    let root = float_square_root(e, discriminant);
    let half = x87_load64(e.mem.u64(HALF_DOUBLE));
    let low_value = x87_store32(x87_mul(x87_sub(sum, root), half));
    let high_value = x87_store32(x87_mul(x87_add(sum, root), half));
    let (x, y) = if difference >= 0.0 {
        let x = x87_quiet_bits(e.mem.u32(between));
        (x, x87_store32(x87_sub(ld(e, low), x87_m32(low_value))))
    } else {
        let x = x87_store32(x87_sub(ld(e, high), x87_m32(low_value)));
        (x, x87_quiet_bits(e.mem.u32(between)))
    };
    let (xv, yv) = (x87_load32(x), x87_load32(y));
    let length = float_square_root(e, x87_add(x87_mul(yv, yv), x87_mul(xv, xv)));
    let inverse = through_float(x87_div(1.0, length));
    let cosine = through_float(x87_mul(inverse, x87_m32(x)));
    let sine = through_float(x87_mul(inverse, x87_m32(y)));
    for (a_at, b_at, read) in block.rows {
        rotate_pair(e, m + a_at, m + b_at, cosine, sine, read);
    }
    e.mem.set_u32(low, x87_quiet_bits(low_value));
    e.mem.set_u32(high, x87_quiet_bits(high_value));
    st(e, between, 0.0);
    true
}

// Translated from 00a58b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `thiscall`, (diagonal, sub-diagonal),
/// answers in `AL`): the QL iteration with implicit shifts on the tridiagonal matrix (three
/// diagonal and two sub-diagonal floats), accumulating the rotations in the matrix's
/// columns. Each of at most 32 passes first tests whether `e0` or `e1` has become
/// negligible (`|d0| + |d1| + |e0| == |d0| + |d1|`, the same with `d1`, `d2`, `e1`); then it
/// solves the remaining 2x2 block and answers true. Otherwise one pass: the shift from
/// `g = (d1 - d0) / 2 e0` and `r = sqrt(g^2 + 1)`, `f = e0 / (g +/- r) + d2 - d0` (minus
/// when `g` is negative or NaN), a rotation of columns 1 and 2 by `(f, e1)`, the new `p`,
/// `e0 c`, `e0 s`, then a rotation of columns 0 and 1 and the new `e1`, `e0`, `d0`, `d1`,
/// `d2`. After 32 passes the answer is false. The constant 1 the code keeps on the x87
/// stack is plain 1.0 here; the one it adds from memory (`01012070`) is read.
pub fn fn_00a58b00(e: &mut Engine, this: Ptr, diagonal: Ptr, sub_diagonal: Ptr) -> bool {
    let (m, d, s) = (this.addr(), diagonal.addr(), sub_diagonal.addr());
    let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
    for _ in 0..32 {
        let size1 = magnitude_bits(e, d + 4);
        let size0 = magnitude_bits(e, d);
        let upper = through_float(x87_add(x87_load32(size0), x87_load32(size1)));
        let e0_size = x87_load32(magnitude_bits(e, s));
        if upper == x87_add(e0_size, upper) {
            let block = FinalBlock {
                low: 4,
                high: 8,
                between: 4,
                other: 0,
                sum: (8, 4),
                rows: [
                    (4, 8, PairRead::Loaded),
                    (0x10, 0x14, PairRead::Loaded),
                    (0x1c, 0x20, PairRead::OperandThenLoaded),
                ],
            };
            return finish_block(e, m, d, s, &block);
        }
        let lower = through_float(x87_add(
            x87_load32(size1),
            x87_m32(magnitude_bits(e, d + 8)),
        ));
        let e1_size = x87_load32(magnitude_bits(e, s + 4));
        if lower == x87_add(e1_size, lower) {
            let block = FinalBlock {
                low: 0,
                high: 4,
                between: 0,
                other: 4,
                sum: (0, 4),
                rows: [
                    (0, 4, PairRead::Loaded),
                    (0xc, 0x10, PairRead::Operand),
                    (0x18, 0x1c, PairRead::Loaded),
                ],
            };
            return finish_block(e, m, d, s, &block);
        }

        // The shift.
        let e0 = ld(e, s);
        let g = through_float(x87_div(x87_sub(ld(e, d + 4), op(e, d)), x87_add(e0, e0)));
        let r = float_square_root(e, x87_add(1.0, x87_mul(g, g)));
        let e1 = through_float(ld(e, s + 4));
        let d2_minus_d0 = x87_store32(x87_sub(ld(e, d + 8), op(e, d)));
        let denominator = if g >= 0.0 {
            x87_add(g, r)
        } else {
            x87_sub(g, r)
        };
        let f = through_float(x87_add(
            x87_div(op(e, s), denominator),
            x87_m32(d2_minus_d0),
        ));

        // The rotation of columns 1 and 2.
        let (cosine, sine, _, _) = plane_rotation(e, f, e1, one);
        rotate_pair(e, m + 4, m + 8, cosine, sine, PairRead::Loaded);
        rotate_pair(e, m + 0x10, m + 0x14, cosine, sine, PairRead::Loaded);
        rotate_pair(e, m + 0x1c, m + 0x20, cosine, sine, PairRead::Operand);
        let e1 = ld(e, s + 4);
        let p = through_float(x87_add(
            x87_mul(x87_sub(ld(e, d + 4), op(e, d + 8)), sine),
            x87_mul(x87_add(e1, e1), cosine),
        ));
        let e0_cosine = through_float(x87_mul(ld(e, s), cosine));
        let e0_sine = through_float(x87_mul(ld(e, s), sine));
        let w = through_float(x87_sub(x87_mul(cosine, p), ld(e, s + 4)));
        let p = through_float(x87_mul(sine, p));

        // The rotation of columns 0 and 1.
        let (cosine, sine, root, top_smaller) = plane_rotation(e, w, e0_sine, 1.0);
        let scaled = if top_smaller { e0_sine } else { w };
        st(e, s + 4, x87_mul(scaled, root));
        rotate_pair(e, m, m + 4, cosine, sine, PairRead::Loaded);
        rotate_pair(e, m + 0xc, m + 0x10, cosine, sine, PairRead::Operand);
        rotate_pair(e, m + 0x18, m + 0x1c, cosine, sine, PairRead::Loaded);

        let g = through_float(x87_sub(ld(e, d + 4), p));
        st(e, d + 8, x87_add(p, op(e, d + 8)));
        let k = through_float(x87_add(
            x87_mul(x87_sub(ld(e, d), g), sine),
            x87_mul(x87_add(e0_cosine, e0_cosine), cosine),
        ));
        st(e, s, x87_sub(x87_mul(cosine, k), e0_cosine));
        let shift = through_float(x87_mul(sine, k));
        st(e, d + 4, x87_add(g, shift));
        st(e, d, x87_sub(op(e, d), shift));
    }
    false
}

// Translated from 00a59400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::ToEulerAnglesZXY` (Xbox PDB; (z angle, x angle, y angle)): the x angle
/// `-asin(m21)` (`00ec9650`; `m21` at or below -1 or NaN gives pi/2, the float at `011a9444`,
/// at or above 1 -pi/2); strictly between -pi/2 and pi/2 the z angle `-atan2(-m01, m11)` and
/// the y angle `-atan2(-m20, m22)` (`NiFastATan2`) and the answer 1; at -pi/2 or below the
/// y angle 0 and the z angle `atan2(m02, m00)` minus the double at `01012060`; at pi/2 or
/// above (or NaN) the y angle 0 and the z angle `0 - atan2(m02, m00)`; those answer 0.
pub fn ni_matrix3_to_euler_angles_zxy(
    e: &mut Engine,
    this: Ptr,
    z_angle: Ptr,
    x_angle: Ptr,
    y_angle: Ptr,
) -> bool {
    let m = this.addr();
    let value = through_float(ld(e, m + 0x1c));
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    let half_pi = ld(e, HALF_PI_FLOAT);
    let local = if !(value > minus_one) {
        x87_store32(-half_pi)
    } else if !(1.0 > value) {
        x87_store32(half_pi)
    } else {
        let angle = e.call(ASIN, &args![value]).f64();
        x87_store32(through_float(angle))
    };
    let x = through_float(-x87_load32(local));
    st(e, x_angle.addr(), x);
    let atan2 =
        |e: &mut Engine, first: u32, second: u32| e.call(FAST_ATAN2, &args![first, second]).f64();
    let negated = |e: &Engine, at: u32| x87_store32(-ld(e, at));
    let copied = |e: &Engine, at: u32| x87_quiet_bits(e.mem.u32(at));
    if x < half_pi {
        if -half_pi < x {
            let (first, second) = (negated(e, m + 4), copied(e, m + 0x10));
            let z = atan2(e, first, second);
            st(e, z_angle.addr(), -z);
            let (first, second) = (negated(e, m + 0x18), copied(e, m + 0x20));
            let y = atan2(e, first, second);
            st(e, y_angle.addr(), -y);
            return true;
        }
        let (first, second) = (copied(e, m + 8), copied(e, m));
        let r = x87_store32(atan2(e, first, second));
        st(e, y_angle.addr(), 0.0);
        let offset = f64::from_bits(e.mem.u64(ANGLE_OFFSET_DOUBLE));
        st(e, z_angle.addr(), x87_sub(x87_load32(r), offset));
        return false;
    }
    let (first, second) = (copied(e, m + 8), copied(e, m));
    let r = x87_store32(atan2(e, first, second));
    st(e, y_angle.addr(), 0.0);
    st(e, z_angle.addr(), x87_sub(0.0, x87_load32(r)));
    false
}

/// `FSINCOS` of a float angle: the sine and cosine stored as floats.
fn float_sin_cos(angle: f32) -> (u32, u32) {
    let (sine, cosine) = x87_fsincos(X87Ext::load32(angle.to_bits()));
    (sine.store32(), cosine.store32())
}

// Translated from 00a59660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; three float angles about z, x and y in
/// that order): the three rotations (sines and cosines by `FSINCOS`, stored as floats)
/// combined as `Z * (X * Y)` through `NiMatrix3::operator*`, copied into `this`.
pub fn fn_00a59660(e: &mut Engine, this: Ptr, z: f32, x: f32, y: f32) {
    let (sx, cx) = float_sin_cos(x);
    let (sy, cy) = float_sin_cos(y);
    let (sz, cz) = float_sin_cos(z);
    compose(e, this, about_z(sz, cz), about_x(sx, cx), about_y(sy, cy));
}

// Translated from 00a598a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `thiscall`, no arguments): sets every
/// entry whose magnitude (stored as a float) is at most 1e-8 (`0109afc4`) and that is not
/// already zero (either sign) to +0.
pub fn fn_00a598a0(e: &mut Engine, this: Ptr) {
    let limit = ld(e, ZERO_EPSILON);
    for entry in 0..9 {
        let at = this.addr() + 4 * entry;
        let size = through_float(ld(e, at).abs());
        if size <= limit && op(e, at) != 0.0 {
            st(e, at, 0.0);
        }
    }
}

// Translated from 00a598f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::GetViewerString` (Xbox PDB; the prefix): allocates the prefix's length plus
/// 0x80 bytes (`00aa1070`) and prints `"%s = ((%g,%g,%g),(%g,%g,%g),(%g,%g,%g))"` into it
/// through `NiSprintf` (`00a305b0`, buffer, size, format, prefix, the nine entries as
/// doubles); answers the buffer.
pub fn ni_matrix3_get_viewer_string(e: &mut Engine, this: Ptr, prefix: Ptr) -> Ptr {
    let mut length = 0u32;
    while e.mem.u8(prefix.addr().wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    let size = length.wrapping_add(0x80);
    let buffer = e.call(NI_ALLOC, &args![size]).u32();
    let mut words = args![buffer, size, VIEWER_FORMAT, prefix];
    for entry in 0..9 {
        let value = ld(e, this.addr() + 4 * entry);
        words.extend(args![value]);
    }
    e.call(NI_SPRINTF, &words);
    Ptr::new(buffer)
}

// Translated from 00a59980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; (eigenvalues, eigenvectors)): the
/// eigen-decomposition of the symmetric matrix. A copy of the matrix is reduced to
/// tridiagonal form (`00a58990`, diagonal into `eigenvalues`, sub-diagonal into a local) and
/// solved (`00a58b00`); `eigenvectors` gets the copy transposed, its last row negated when
/// the reduction answered true.
pub fn fn_00a59980(e: &mut Engine, this: Ptr, eigenvalues: Ptr, eigenvectors: Ptr) {
    e.with_stack(0x2c, |e, frame| {
        let sub_diagonal = frame;
        let copy = frame.addr() + 8;
        for word in 0..9 {
            let value = e.mem.u32(this.addr() + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        let flipped = fn_00a58990(e, Ptr::new(copy), eigenvalues, sub_diagonal);
        fn_00a58b00(e, Ptr::new(copy), eigenvalues, sub_diagonal);
        let out = eigenvectors.addr();
        for (index, source) in [0u32, 3, 6, 1, 4, 7].into_iter().enumerate() {
            let value = x87_quiet_bits(e.mem.u32(copy + 4 * source));
            e.mem.set_u32(out + 4 * index as u32, value);
        }
        for (index, source) in [(6u32, 2u32), (7, 5), (8, 8)] {
            let value = ld(e, copy + 4 * source);
            let value = if flipped { -value } else { value };
            st(e, out + 4 * index, value);
        }
    });
}

// Translated from 00a59a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::SaveBinary` (Xbox PDB): first sets the near-zero entries to 0 (`00a598a0`),
/// then writes the nine 4-byte entries, row by row, to the stream's output (`NiStream`
/// +0x250, its write function +0xc and position +4), the byte-count word a local set to 4.
pub fn ni_matrix3_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    fn_00a598a0(e, this);
    e.with_stack(4, |e, count| {
        for word in 0..9 {
            let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
            let function = e.mem.u32(output + BINARY_STREAM_WRITE);
            e.mem.set_u32(count.addr(), 4);
            let source = this.addr() + 4 * word;
            let done = e
                .call(function, &args![output, source, 4u32, count, 1u32])
                .u32();
            let position = e
                .mem
                .u32(output + BINARY_STREAM_POSITION)
                .wrapping_add(done);
            e.mem.set_u32(output + BINARY_STREAM_POSITION, position);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_82f0,
            ni_matrix3_transform_vertices(Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(0x00a5_8510, fn_00a58510(f32) -> f32),
        entry!(
            0x00a5_92c0,
            ni_matrix3_to_euler_angles_xyz(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_9540,
            ni_matrix3_from_euler_angles_xyz(Ptr, f32, f32, f32)
        ),
        entry!(
            0x00a5_9780,
            ni_matrix3_from_euler_angles_zyx(Ptr, f32, f32, f32)
        ),
        entry!(0x00a5_9a30, ni_matrix3_load_binary(Ptr, Ptr)),
        entry!(0x00a5_8550, fn_00a58550(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00a5_88a0,
            ni_matrix3_transpose_times(Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a5_8990, fn_00a58990(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00a5_8b00, fn_00a58b00(Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00a5_9400,
            ni_matrix3_to_euler_angles_zxy(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a5_9660, fn_00a59660(Ptr, f32, f32, f32)),
        entry!(0x00a5_98a0, fn_00a598a0(Ptr)),
        entry!(
            0x00a5_98f0,
            ni_matrix3_get_viewer_string(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a5_9980, fn_00a59980(Ptr, Ptr, Ptr)),
        entry!(0x00a5_9a90, ni_matrix3_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// An engine with the constants and a matrix product double (row by row, in `f32`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_a000, 0x1000);
        e.map(0x0109_a000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.mem.set_f64(MINUS_ONE_DOUBLE, -1.0);
        e.mem.set_f32(PI_FLOAT, std::f32::consts::PI);
        e.mem.set_f32(HALF_PI_FLOAT, std::f32::consts::FRAC_PI_2);
        e.mem.set_f64(ANGLE_OFFSET_DOUBLE, 0.0);
        e.register(MATRIX_MULTIPLY, |e, a| {
            let get = |e: &Engine, m: u32, r: u32, c: u32| e.mem.f32(m + 12 * r + 4 * c);
            let mut out = [0.0f32; 9];
            for r in 0..3 {
                for c in 0..3 {
                    out[(r * 3 + c) as usize] = (0..3)
                        .map(|k| get(e, a[0], r, k) * get(e, a[2], k, c))
                        .sum();
                }
            }
            for (i, v) in out.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            a[1].into_ret()
        });
        e
    }

    #[test]
    fn transform_vertices_applies_the_matrix_and_translation() {
        let mut e = engine();
        let m = e.mem.alloc(0x24);
        put(&mut e, m, &[0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0]);
        let t = e.mem.alloc(0xc);
        put(&mut e, t, &[10.0, 20.0, 30.0]);
        let count = 5u32;
        let src = e.mem.alloc(12 * count);
        for i in 0..count {
            put(&mut e, src + 12 * i, &[i as f32, 1.0, 1.0]);
        }
        let dst = e.mem.alloc(12 * count);
        e.call(0x00a5_82f0, &args![m, t, count, src, dst]);
        for i in 0..count {
            assert_eq!(
                floats(&e, dst + 12 * i, 3),
                vec![9.0, 20.0 + i as f32, 32.0]
            );
        }
        // In place, the later components see the earlier ones already written.
        let v = e.mem.alloc(12);
        put(&mut e, v, &[3.0, 1.0, 1.0]);
        e.call(0x00a5_82f0, &args![m, t, 1u32, v, v]);
        assert_eq!(floats(&e, v, 3), vec![9.0, 29.0, 32.0]);
    }

    #[test]
    fn clamped_arc_cosine() {
        let mut e = engine();
        e.register(ACOS, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: v.acos(),
                ..Ret::default()
            }
        });
        let acos = |e: &mut Engine, v: f32| e.call(0x00a5_8510, &args![v]).f32();
        assert_eq!(acos(&mut e, -1.0), std::f32::consts::PI);
        assert_eq!(acos(&mut e, f32::NAN), std::f32::consts::PI);
        assert_eq!(acos(&mut e, 1.0), 0.0);
        assert_eq!(acos(&mut e, 2.0), 0.0);
        assert_eq!(acos(&mut e, 0.5), (0.5f64.acos()) as f32);
    }

    #[test]
    fn euler_angles_round_trip_and_the_gimbal_cases() {
        let mut e = engine();
        e.register(ASIN, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: v.asin(),
                ..Ret::default()
            }
        });
        e.register(FAST_ATAN2, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        let m = e.mem.alloc(0x24);
        let out = e.mem.alloc(0xc);
        e.call(0x00a5_9540, &args![m, 0.25f32, 0.5f32, -0.75f32]);
        let ok = e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]).bool();
        assert!(ok);
        let angles = floats(&e, out, 3);
        for (got, want) in angles.iter().zip([0.25f32, 0.5, -0.75]) {
            assert!((got - want).abs() < 1e-5, "{got} {want}");
        }
        // m02 = -1: y = pi/2, z 0, x = 0 - atan2(m10, m11).
        put(&mut e, m, &[0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]);
        let ok = e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]).bool();
        assert!(!ok);
        assert_eq!(
            floats(&e, out, 3),
            vec![0.0, std::f32::consts::FRAC_PI_2, 0.0]
        );
        // m02 = 1: y = -pi/2, x = atan2(m10, m11) - the offset (0 here).
        put(&mut e, m, &[0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]);
        assert_eq!(
            floats(&e, out, 3),
            vec![
                std::f32::consts::FRAC_PI_2,
                -std::f32::consts::FRAC_PI_2,
                0.0
            ]
        );
    }

    #[test]
    fn from_euler_angles_zyx_composes_in_reverse() {
        let mut e = engine();
        let m = e.mem.alloc(0x24);
        // A quarter turn about z only.
        let quarter = std::f32::consts::FRAC_PI_2;
        e.call(0x00a5_9780, &args![m, quarter, 0.0f32, 0.0f32]);
        let (s, c) = (
            (f64::from(quarter)).sin() as f32,
            (f64::from(quarter)).cos() as f32,
        );
        assert_eq!(floats(&e, m, 9), vec![c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0]);
        // A quarter turn about x only (the third angle).
        e.call(0x00a5_9780, &args![m, 0.0f32, 0.0f32, quarter]);
        assert_eq!(floats(&e, m, 9), vec![1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c]);
        // XYZ with only y.
        e.call(0x00a5_9540, &args![m, 0.0f32, quarter, 0.0f32]);
        assert_eq!(floats(&e, m, 9), vec![c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c]);
    }

    #[test]
    fn load_binary_reads_nine_words() {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = 1u32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        let m = e.mem.alloc(0x24);
        e.call(0x00a5_9a30, &args![m, stream]);
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(m + 4 * i)).collect();
        assert_eq!(words, (1..=9).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(input + 4), 36);
    }

    fn st0_argument(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
    }

    /// [`engine`] with the other constants, the square root, `acos`, `asin` and `atan2`.
    fn solver_engine() -> Engine {
        let mut e = engine();
        for page in [0x0101_1000, 0x0101_d000, 0x0109_5000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(FOUR_DOUBLE, 4.0);
        e.mem.set_f32(AXIS_EPSILON, 1e-6);
        e.mem.set_f32(ZERO_EPSILON, 1e-8);
        e.register(SQUARE_ROOT, |_, a| Ret {
            st0: st0_argument(a).sqrt(),
            ..Ret::default()
        });
        e.register(ACOS, |_, a| Ret {
            st0: st0_argument(a).acos(),
            ..Ret::default()
        });
        e.register(ASIN, |_, a| Ret {
            st0: st0_argument(a).asin(),
            ..Ret::default()
        });
        e.register(FAST_ATAN2, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        e
    }

    fn close(got: &[f32], want: &[f32]) {
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-5, "{got:?} {want:?}");
        }
    }

    #[test]
    fn angle_and_axis_of_rotations_and_half_turns() {
        let mut e = solver_engine();
        let m = e.mem.alloc(0x24);
        let out = e.mem.alloc(0x10);
        let run = |e: &mut Engine| {
            e.call(0x00a5_8550, &args![m, out, out + 4, out + 8, out + 12]);
            floats(e, out, 4)
        };
        // A quarter turn about z: angle acos(0), axis (0, 0, 1).
        put(&mut e, m, &[0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        let quarter = (0.0f64.acos()) as f32;
        assert_eq!(run(&mut e), vec![quarter, 0.0, 0.0, 1.0]);
        // The identity: angle 0 and the axis (1, 0, 0).
        put(&mut e, m, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert_eq!(run(&mut e), vec![0.0, 1.0, 0.0, 0.0]);
        // A half turn about x: pi, the axis from the diagonal, which the matrix keeps.
        put(&mut e, m, &[1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0]);
        assert_eq!(run(&mut e), vec![std::f32::consts::PI, 1.0, 0.0, 0.0]);
        // A half turn about (1, -1, 0): the diagonal gives (h, h, 0), which the matrix does
        // not keep, nor with z negated, so y is negated too.
        put(&mut e, m, &[0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0]);
        let h = (0.5f64.sqrt()) as f32;
        let got = run(&mut e);
        assert_eq!(got[..3], [std::f32::consts::PI, h, -h]);
        assert_eq!(got[3].to_bits(), 0x8000_0000);
        // A half turn about (1, 1, 0) keeps (h, h, 0).
        put(&mut e, m, &[0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0]);
        assert_eq!(run(&mut e), vec![std::f32::consts::PI, h, h, 0.0]);
    }

    #[test]
    fn transpose_times_multiplies_the_transpose() {
        let mut e = solver_engine();
        let (a, b, r) = (e.mem.alloc(0x24), e.mem.alloc(0x24), e.mem.alloc(0x24));
        let av = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let bv = [2.0f32, -1.0, 0.5, 3.0, 0.0, 1.0, -2.0, 4.0, 1.5];
        put(&mut e, a, &av);
        put(&mut e, b, &bv);
        assert_eq!(e.call(0x00a5_88a0, &args![a, r, b]).u32(), r);
        let mut want = [0.0f32; 9];
        for i in 0..3 {
            for j in 0..3 {
                want[i * 3 + j] = (0..3).map(|k| av[k * 3 + i] * bv[k * 3 + j]).sum();
            }
        }
        assert_eq!(floats(&e, r, 9), want.to_vec());
    }

    #[test]
    fn tridiagonal_reduction_rotates_rows_one_and_two() {
        let mut e = solver_engine();
        let (m, d, s) = (e.mem.alloc(0x24), e.mem.alloc(0xc), e.mem.alloc(8));
        // m02 = 4, m01 = 3: L = 5, (a, b) = (0.6, 0.8), q = 2 a m12 + (m22 - m11) b = 4.
        put(&mut e, m, &[7.0, 3.0, 4.0, 3.0, 1.0, 2.0, 4.0, 2.0, 3.0]);
        assert!(e.call(0x00a5_8990, &args![m, d, s]).bool());
        close(&floats(&e, d, 3), &[7.0, 4.2, -0.2]);
        close(&floats(&e, s, 2), &[5.0, -0.4]);
        close(
            &floats(&e, m, 9),
            &[1.0, 0.0, 0.0, 0.0, 0.6, 0.8, 0.0, 0.8, -0.6],
        );
        // m02 below 1e-8: the identity, the diagonal and sub-diagonal as they are.
        put(&mut e, m, &[7.0, 3.0, 1e-9, 3.0, 1.0, 2.0, 1e-9, 2.0, 5.0]);
        assert!(!e.call(0x00a5_8990, &args![m, d, s]).bool());
        assert_eq!(floats(&e, d, 3), vec![7.0, 1.0, 5.0]);
        assert_eq!(floats(&e, s, 2), vec![3.0, 2.0]);
        assert_eq!(
            floats(&e, m, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn ql_iteration_finishes_blocks_and_gives_up_after_32_passes() {
        let mut e = solver_engine();
        let (m, d, s) = (e.mem.alloc(0x24), e.mem.alloc(0xc), e.mem.alloc(8));
        let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let h = (0.5f64.sqrt()) as f32;
        // e1 = 0: the upper block [[2, 1], [1, 2]] gives 1 and 3, the 45-degree rotation of
        // columns 0 and 1.
        put(&mut e, m, &identity);
        put(&mut e, d, &[2.0, 2.0, 3.0]);
        put(&mut e, s, &[1.0, 0.0]);
        assert!(e.call(0x00a5_8b00, &args![m, d, s]).bool());
        assert_eq!(floats(&e, d, 3), vec![1.0, 3.0, 3.0]);
        assert_eq!(floats(&e, s, 2), vec![0.0, 0.0]);
        close(&floats(&e, m, 9), &[h, h, 0.0, -h, h, 0.0, 0.0, 0.0, 1.0]);
        // e0 = 0: the lower block [[2, 1], [1, 3]] gives (5 -/+ sqrt 5) / 2.
        put(&mut e, m, &identity);
        put(&mut e, d, &[1.0, 2.0, 3.0]);
        put(&mut e, s, &[0.0, 1.0]);
        assert!(e.call(0x00a5_8b00, &args![m, d, s]).bool());
        let root5 = 5.0f32.sqrt();
        close(
            &floats(&e, d, 3),
            &[1.0, (5.0 - root5) / 2.0, (5.0 + root5) / 2.0],
        );
        assert_eq!(floats(&e, s, 2), vec![0.0, 0.0]);
        // Both already zero: nothing changes.
        put(&mut e, m, &identity);
        put(&mut e, d, &[1.0, 2.0, 3.0]);
        put(&mut e, s, &[0.0, 0.0]);
        assert!(e.call(0x00a5_8b00, &args![m, d, s]).bool());
        assert_eq!(floats(&e, d, 3), vec![1.0, 2.0, 3.0]);
        assert_eq!(floats(&e, m, 9), identity.to_vec());
        // A NaN never converges: 32 passes, then false.
        put(&mut e, m, &identity);
        put(&mut e, d, &[f32::NAN, 2.0, 3.0]);
        put(&mut e, s, &[1.0, 1.0]);
        assert!(!e.call(0x00a5_8b00, &args![m, d, s]).bool());
    }

    #[test]
    fn eigen_decomposition_of_a_symmetric_matrix() {
        let mut e = solver_engine();
        let a = [4.0f32, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 2.0];
        let (m, values, vectors) = (e.mem.alloc(0x24), e.mem.alloc(0xc), e.mem.alloc(0x24));
        put(&mut e, m, &a);
        e.call(0x00a5_9980, &args![m, values, vectors]);
        // The matrix itself is left as it was.
        assert_eq!(floats(&e, m, 9), a.to_vec());
        let lambda = floats(&e, values, 3);
        let v = floats(&e, vectors, 9);
        // Each row of the result is an eigenvector of its eigenvalue.
        for k in 0..3 {
            let row = &v[k * 3..k * 3 + 3];
            for i in 0..3 {
                let av: f32 = (0..3).map(|j| a[i * 3 + j] * row[j]).sum();
                assert!((av - lambda[k] * row[i]).abs() < 1e-4, "{lambda:?} {v:?}");
            }
        }
        let trace: f32 = lambda.iter().sum();
        assert!((trace - 9.0).abs() < 1e-4);
    }

    #[test]
    fn to_euler_angles_zxy_round_trip_and_the_gimbal_cases() {
        let mut e = solver_engine();
        let m = e.mem.alloc(0x24);
        let out = e.mem.alloc(0xc);
        e.call(0x00a5_9660, &args![m, 0.25f32, 0.5f32, -0.75f32]);
        let ok = e.call(0x00a5_9400, &args![m, out, out + 4, out + 8]).bool();
        assert!(ok);
        close(&floats(&e, out, 3), &[0.25, 0.5, -0.75]);
        // m21 = -1: x = pi/2, y = 0, z = 0 - atan2(m02, m00).
        put(&mut e, m, &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0]);
        assert!(!e.call(0x00a5_9400, &args![m, out, out + 4, out + 8]).bool());
        assert_eq!(
            floats(&e, out, 3),
            vec![0.0, std::f32::consts::FRAC_PI_2, 0.0]
        );
        // m21 = 1: x = -pi/2, z = atan2(m02, m00) minus the offset (0 here).
        put(&mut e, m, &[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        assert!(!e.call(0x00a5_9400, &args![m, out, out + 4, out + 8]).bool());
        assert_eq!(
            floats(&e, out, 3),
            vec![
                std::f32::consts::FRAC_PI_2,
                -std::f32::consts::FRAC_PI_2,
                0.0
            ]
        );
    }

    #[test]
    fn from_euler_angles_zxy_composes_z_x_y() {
        let mut e = solver_engine();
        let m = e.mem.alloc(0x24);
        let quarter = std::f32::consts::FRAC_PI_2;
        let (s, c) = (
            f32::from_bits(float_sin_cos(quarter).0),
            f32::from_bits(float_sin_cos(quarter).1),
        );
        // Only the first angle: about z.
        e.call(0x00a5_9660, &args![m, quarter, 0.0f32, 0.0f32]);
        assert_eq!(floats(&e, m, 9), vec![c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0]);
        // Only the second: about x.
        e.call(0x00a5_9660, &args![m, 0.0f32, quarter, 0.0f32]);
        assert_eq!(floats(&e, m, 9), vec![1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c]);
        // Only the third: about y.
        e.call(0x00a5_9660, &args![m, 0.0f32, 0.0f32, quarter]);
        assert_eq!(floats(&e, m, 9), vec![c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c]);
    }

    #[test]
    fn snap_zeroes_tiny_nonzero_entries() {
        let mut e = solver_engine();
        let m = e.mem.alloc(0x24);
        put(
            &mut e,
            m,
            &[1e-9, -1e-9, 0.0, -0.0, 1e-8, 2e-8, f32::NAN, 1.0, -5e-9],
        );
        e.call(0x00a5_98a0, &args![m]);
        let bits: Vec<u32> = (0..9).map(|i| e.mem.u32(m + 4 * i)).collect();
        assert_eq!(
            bits,
            vec![
                0,
                0,
                0,
                0x8000_0000,
                0,
                2e-8f32.to_bits(),
                f32::NAN.to_bits(),
                1.0f32.to_bits(),
                0
            ]
        );
    }

    #[test]
    fn viewer_string_allocates_and_prints_the_nine_entries() {
        let mut e = solver_engine();
        let m = e.mem.alloc(0x24);
        put(&mut e, m, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.5]);
        let prefix = e.mem.alloc(8);
        e.mem.write(prefix, b"rot\0");
        e.register(NI_ALLOC, |_, _| 0x0300_0000u32.into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_98f0, &args![m, prefix]).u32(), 0x0300_0000);
        let log = e.call_log.take().unwrap();
        let alloc = log.iter().find(|c| c.0 == NI_ALLOC).unwrap();
        assert_eq!(alloc.1, vec![0x83]);
        let print = log.iter().find(|c| c.0 == NI_SPRINTF).unwrap();
        let mut want = args![0x0300_0000u32, 0x83u32, VIEWER_FORMAT, prefix];
        for v in [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.5] {
            want.extend(args![v]);
        }
        assert_eq!(print.1, want);
    }

    #[test]
    fn save_binary_snaps_then_writes_nine_words() {
        let mut e = solver_engine();
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(output + 4, 8);
        let written = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let sink = written.clone();
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            sink.borrow_mut().push(e.mem.u32(a[1]));
            4u32.into_ret()
        });
        let m = e.mem.alloc(0x24);
        put(&mut e, m, &[1.0, 1e-9, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        e.call(0x00a5_9a90, &args![m, stream]);
        let want: Vec<u32> = [1.0f32, 0.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(*written.borrow(), want);
        assert_eq!(e.mem.u32(output + 4), 8 + 36);
    }
}
