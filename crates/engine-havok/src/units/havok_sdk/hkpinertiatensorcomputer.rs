//! `Havok SDK/hkpinertiatensorcomputer.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The mass-properties record these functions fill (its class name is unconfirmed; the layout
//! is read from this unit's code): the volume at `+0`, the mass at `+4`, the centre of mass at
//! `+0x10` (a 16-byte vector), and the inertia tensor as three 16-byte rows at `+0x20`, `+0x30`
//! and `+0x40` (diagonal at `+0x20`, `+0x34`, `+0x48`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_GROW, ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, cross, mul, sub};
use crate::units::havok_sdk::hkphingeconstraintdata::{normalised, scaled};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, maxps, minps, store_bits, store_vec, sum3, Vec4, HALF_F32,
    SIGN_MASK,
};

/// `hkMatrix3::setMul` (second overload, `00cd55b0`): `this = a * b`.
const MATRIX_SET_MUL: u32 = 0x00cd_55b0;
/// `00cd4c70` (unnamed in the engine map): `this` is a matrix of three 16-byte rows, the one
/// word the address of a vector whose first lane multiplies every lane of every row.
const MATRIX_SCALE: u32 = 0x00cd_4c70;
/// The CRT square root (`00ec6040`, the argument in `ST0`, so a leading double).
const X87_SQRT: u32 = 0x00ec_6040;

/// Fields of the mass-properties record.
const VOLUME: u32 = 0;
const MASS: u32 = 4;
const CENTRE: u32 = 0x10;
const INERTIA: u32 = 0x20;

/// Doubles the code multiplies by: 0.4 (sphere moment factor), 4.18879 (4 pi / 3 as a
/// widened float), 1/3 (widened float), 8.0.
const SPHERE_MOMENT_FACTOR: u32 = 0x0102_90c0;
const SPHERE_VOLUME_FACTOR: u32 = 0x010c_bf40;
const THIRD: u32 = 0x0109_9ee0;
const EIGHT: u32 = 0x0102_40b0;
/// Doubles 1.0, 2.0, 3.0, 4.0 and 0.5.
const ONE_D: u32 = 0x0101_2070;
const TWO_D: u32 = 0x0101_1590;
const THREE_D: u32 = 0x0102_1928;
const FOUR_D: u32 = 0x0101_db80;
const HALF_D: u32 = 0x0101_1588;

/// A float as the x87 holds it after `FLD dword`.
fn load(e: &Engine, addr: u32) -> f64 {
    f64::from(e.mem.f32(addr))
}

/// `FSTP dword`: the value rounded to a float.
fn store(e: &mut Engine, addr: u32, value: f64) {
    e.mem.set_f32(addr, value as f32);
}

/// What a value is after a store to a float slot and a load back.
fn rounded(value: f64) -> f64 {
    f64::from(value as f32)
}

/// `[addr] = (term + [addr])` rounded to a float; the stored value.
fn accumulate(e: &mut Engine, addr: u32, term: f64) -> f64 {
    let sum = rounded(term + load(e, addr));
    store(e, addr, sum);
    sum
}

fn zero_vector(e: &mut Engine, addr: u32) {
    store_vec(e, addr, [0.0; 4]);
}

/// `MATRIX_SCALE` with the factor in the first lane of a temporary vector.
fn scale_rows(e: &mut Engine, matrix: u32, factor: f32) {
    e.with_stack(0x10, |e, vector| {
        store_vec(e, vector.addr(), [factor, 0.0, 0.0, 0.0]);
        e.call(MATRIX_SCALE, &args![matrix, vector]);
    });
}

/// Functions outside this unit the code calls (all by address): `00c6cd00` copies three rows
/// (`this` = destination, the one word the source), `hkMatrix3::sub` `00cd5180` and
/// `hkMatrix3::add` `00cd5140` (`this` op= the one word), `hkMatrix3::changeBasis` `00cd5880`
/// (`this` = matrix, the one word the transform) and `hkVector4::setTransformedPos`
/// `00c7f640` (`this` = result, `(transform, vector)`).
const COPY_ROWS: u32 = 0x00c6_cd00;
const MATRIX_SUB: u32 = 0x00cd_5180;
const MATRIX_ADD: u32 = 0x00cd_5140;
const CHANGE_BASIS: u32 = 0x00cd_5880;
const TRANSFORM_POINT: u32 = 0x00c7_f640;
/// The same function under the name the recursive shape code uses.
const SET_TRANSFORMED_POS: u32 = TRANSFORM_POINT;
/// `hkTransform::setMul` (Xbox PDB, `00cb2240`; `this` = result, `(a, b)`), `00c98210` (no
/// map name: cdecl, two floats, answers the larger in `ST0`; the second when equal),
/// `hkpCylinderShape::getCylinderRadius` (`00d571a0`, `this` = shape, answer in `ST0`),
/// the `hkpSphereShape` constructor (`00cd8160`, `this` = object, radius) and
/// `hkpConvexVerticesShape::getOriginalVertices` (`00d59200`, `this` = shape, the one word an
/// array to fill).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
const TRIANGLE_THICKNESS: u32 = 0x00c9_8210;
const CYLINDER_RADIUS: u32 = 0x00d5_71a0;
const SPHERE_CONSTRUCT: u32 = 0x00cd_8160;
const GET_ORIGINAL_VERTICES: u32 = 0x00d5_9200;
/// Floats: 5e-6 (`010cbf78`, a face weight) and the one `computeShapeVolumeMassProperties`
/// retries with (`01013ea4`).
const TINY_AREA: u32 = 0x010c_bf78;
const RETRY_PARAMETER: u32 = 0x0101_3ea4;

// Translated from 00ceba60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeSphereVolumeMassProperties` (Xbox PDB): answers 1 (and
/// writes nothing) when `mass` or `radius` is not above zero (a NaN passes the tests).
/// Otherwise the record at `out` gets the volume `r * 4.18879 * r * r` (`+0`), the mass
/// (`+4`, through the x87 so a signalling NaN is quieted), a zero centre of mass and the
/// diagonal inertia `r * mass * r * 0.4` with zero off-diagonal lanes; answers 0.
pub fn hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(
    e: &mut Engine,
    radius: f32,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || radius <= 0.0 {
        return 1;
    }
    let out = out.addr();
    let (r, m) = (f64::from(radius), f64::from(mass));
    let moment = ((r * m) * r) * e.mem.f64(SPHERE_MOMENT_FACTOR);
    let volume = ((r * e.mem.f64(SPHERE_VOLUME_FACTOR)) * r) * r;
    for row in 0..3 {
        zero_vector(e, out + INERTIA + 0x10 * row);
    }
    zero_vector(e, out + CENTRE);
    for diagonal in [0x20, 0x34, 0x48] {
        store(e, out + diagonal, moment);
    }
    store(e, out + VOLUME, volume);
    e.mem.set_f32(out + MASS, x87_quiet(mass));
    0
}

// Translated from 00cebaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeBoxVolumeMassProperties` (Xbox PDB): `half_extents`
/// points at the three half extents `x`, `y`, `z`. Answers 1 (and writes nothing) when `mass`
/// is not above zero (a NaN passes). Otherwise the record at `out` gets the volume
/// `y * x * z * 8` (`+0`), the mass (`+4`), a zero centre of mass and the diagonal inertia
/// `(y^2 + z^2) * m`, `(z^2 + x^2) * m`, `(y^2 + x^2) * m` with `m = mass / 3` (the double at
/// `01099ee0` times the mass, rounded to a float); answers 0.
pub fn hkp_inertia_tensor_computer_compute_box_volume_mass_properties(
    e: &mut Engine,
    half_extents: Ptr,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 {
        return 1;
    }
    let (h, out) = (half_extents.addr(), out.addr());
    let [x, y, z] = [0, 4, 8].map(|offset| f64::from(x87_quiet(e.mem.f32(h + offset))));
    let m = rounded(x87_mul(e.mem.f64(THIRD), f64::from(mass)));
    for row in 0..3 {
        zero_vector(e, out + INERTIA + 0x10 * row);
    }
    zero_vector(e, out + CENTRE);
    // The x87 stack code, with the x87 choice of NaN.
    let (fadd, fmul) = (x87_add, x87_mul);
    store(e, out + 0x20, fmul(fadd(fmul(y, y), fmul(z, z)), m));
    store(e, out + 0x34, fmul(fadd(fmul(z, z), fmul(x, x)), m));
    store(e, out + 0x48, fmul(fadd(fmul(y, y), fmul(x, x)), m));
    let eight = e.mem.f64(EIGHT);
    store(e, out + VOLUME, fmul(fmul(z, fmul(y, x)), eight));
    e.mem.set_f32(out + MASS, x87_quiet(mass));
    0
}

// Translated from 00cebdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies from the mass-properties record `source` into the
/// block `destination`: the float at `+4` (through the x87) to `+0x90`, and the 16-byte
/// vectors at `+0x10`, `+0x20`, `+0x30`, `+0x40` to `+0x80`, `+0x50`, `+0x60`, `+0x70`.
pub fn fn_00cebdb0(e: &mut Engine, source: Ptr, destination: Ptr) {
    let (s, d) = (source.addr(), destination.addr());
    e.mem.set_f32(d + 0x90, x87_quiet(e.mem.f32(s + 4)));
    for (from, to) in [(0x10, 0x80), (0x20, 0x50), (0x30, 0x60), (0x40, 0x70)] {
        copy_vec(e, s + from, d + to);
    }
}

// Translated from 00cebdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: like `00cebdb0` but into the block at `destination + 0x50`
/// (`base`), with `mass` as its float at `base + 0x40`; the vectors at `source + 0x10`,
/// `+0x20`, `+0x30`, `+0x40` go to `base + 0x30`, `+0`, `+0x10`, `+0x20`. The three rows at
/// `base` are then scaled (`00cd4c70`) by `mass / [source + 4]`, a double division rounded to
/// a float.
pub fn fn_00cebdf0(e: &mut Engine, source: Ptr, mass: f32, destination: Ptr) {
    let (s, base) = (source.addr(), destination.addr() + 0x50);
    let scale = (f64::from(mass) / load(e, s + 4)) as f32;
    e.mem.set_f32(base + 0x40, x87_quiet(mass));
    for (from, to) in [(0x10, 0x30), (0x20, 0), (0x30, 0x10), (0x40, 0x20)] {
        copy_vec(e, s + from, base + to);
    }
    scale_rows(e, base, scale);
}

/// The shift `00cebe80` makes (the same instructions are inline in `00cec6b0`): with the
/// offset vector `a` and the mass `p`, the tensor `m` loses `p * (a_y^2 + a_z^2)` and its two
/// analogues on the diagonal and each off-diagonal pair gets `+ (a_i * p) * a_j`.
fn shift_to_centre(e: &mut Engine, a: u32, p: f64, m: u32) {
    // The x87 stack code with the x87 choice of NaN: `FLD` loads (quiet) and the memory
    // operands of `FSUBR`/`FMUL`/`FADD` (a signalling NaN stays one there), each read when
    // the code reads it.
    let (fadd, fsub, fmul) = (x87_add, x87_sub, x87_mul);
    let ld = |e: &Engine, addr: u32| x87_load32(e.mem.u32(addr));
    let mem = |e: &Engine, addr: u32| x87_m32(e.mem.u32(addr));
    let square = |e: &Engine, addr: u32| fmul(ld(e, addr), ld(e, addr));
    let sum = fadd(square(e, a + 8), square(e, a + 4));
    store(e, m, fsub(ld(e, m), fmul(sum, p)));
    let sum = fadd(square(e, a + 8), square(e, a));
    store(e, m + 0x14, fsub(mem(e, m + 0x14), fmul(sum, p)));
    let sum = fadd(square(e, a + 4), square(e, a));
    store(e, m + 0x28, fsub(mem(e, m + 0x28), fmul(sum, p)));
    let product = fmul(fmul(ld(e, a), p), mem(e, a + 4));
    let xy = rounded(fadd(product, mem(e, m + 4)));
    store(e, m + 4, xy);
    store(e, m + 0x10, xy);
    let product = fmul(fmul(p, mem(e, a + 4)), mem(e, a + 8));
    let yz = rounded(fadd(product, mem(e, m + 0x18)));
    store(e, m + 0x18, yz);
    store(e, m + 0x24, yz);
    let product = fmul(fmul(p, mem(e, a + 8)), mem(e, a));
    let zx = rounded(fadd(product, mem(e, m + 0x20)));
    store(e, m + 0x20, zx);
    store(e, m + 8, zx);
}

// Translated from 00cebe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::shiftInertiaToCom` (Xbox PDB): `offset` points at a vector
/// (three floats), `inertia` at a 3 x 4 float tensor. With `p = mass` the diagonal drops by
/// `p * (offset_j^2 + offset_k^2)` and the pairs `(1, 4)`, `(6, 9)`, `(8, 2)` of the tensor's
/// floats get `offset_i * p * offset_j` added (each pair holds one rounded value).
pub fn hkp_inertia_tensor_computer_shift_inertia_to_com(
    e: &mut Engine,
    offset: Ptr,
    mass: f32,
    inertia: Ptr,
) {
    shift_to_centre(e, offset.addr(), x87_load32(mass.to_bits()), inertia.addr());
}

// Translated from 00cebf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the opposite shift of `00cebe80` (the diagonal gains
/// `p * (offset_j^2 + offset_k^2)`, the off-diagonal pairs lose `offset_i * p * offset_j`;
/// the third pair uses `(p * offset_z) * offset_x`).
pub fn fn_00cebf30(e: &mut Engine, offset: Ptr, mass: f32, inertia: Ptr) {
    // The x87 stack code with the x87 choice of NaN: `FLD` loads (quiet) and the memory
    // operands of `FADD`/`FMUL`/`FSUBR` (a signalling NaN stays one there), each read when
    // the code reads it.
    let (a, m) = (offset.addr(), inertia.addr());
    let (fadd, fsub, fmul) = (x87_add, x87_sub, x87_mul);
    let ld = |e: &Engine, addr: u32| x87_load32(e.mem.u32(addr));
    let mem = |e: &Engine, addr: u32| x87_m32(e.mem.u32(addr));
    let square = |e: &Engine, addr: u32| fmul(ld(e, addr), ld(e, addr));
    let p = x87_load32(mass.to_bits());
    let sum = fadd(square(e, a + 8), square(e, a + 4));
    store(e, m, fadd(fmul(sum, p), ld(e, m)));
    let sum = fadd(square(e, a + 8), square(e, a));
    store(e, m + 0x14, fadd(fmul(sum, p), mem(e, m + 0x14)));
    let sum = fadd(square(e, a + 4), square(e, a));
    store(e, m + 0x28, fadd(fmul(sum, p), mem(e, m + 0x28)));
    let product = fmul(fmul(ld(e, a), p), mem(e, a + 4));
    let xy = rounded(fsub(ld(e, m + 4), product));
    store(e, m + 4, xy);
    store(e, m + 0x10, xy);
    let product = fmul(fmul(ld(e, a + 4), p), mem(e, a + 8));
    let yz = rounded(fsub(ld(e, m + 0x18), product));
    store(e, m + 0x18, yz);
    store(e, m + 0x24, yz);
    let product = fmul(fmul(p, ld(e, a + 8)), ld(e, a));
    let zx = rounded(fsub(mem(e, m + 0x20), product));
    store(e, m + 0x20, zx);
    store(e, m + 8, zx);
}

// Translated from 00cebfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `props` (the mass-properties record) arrives in EAX, `scale`
/// on the stack. The mass at `+4` becomes `scale * mass` (a double product rounded to a
/// float) and the rows at `props + 0x20` are scaled by `scale` (`00cd4c70`).
pub fn fn_00cebfd0(e: &mut Engine, props: Ptr, scale: f32) {
    let p = props.addr();
    let scaled = f64::from(scale) * load(e, p + MASS);
    store(e, p + MASS, scaled);
    scale_rows(e, p + INERTIA, scale);
}

/// Divisors the projection integrals end with, with the offset of the float each divides
/// (`+0xC` is multiplied by the double 0.5 instead).
const PROJECTION_DIVISORS: [(u32, u32); 9] = [
    (0x10, 0x0103_4208),
    (0x18, 0x0103_5710),
    (0x24, 0x0102_fc70),
    (0x14, 0x010a_57b8),
    (0x20, 0x010a_57b0),
    (0x30, 0x0107_4f30),
    (0x1c, 0x0103_56d8),
    (0x28, 0x0101_2638),
    (0x2c, 0x010c_bf58),
];

// Translated from 00cec000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InternalInertiaTensorComputer::compProjectionIntegrals` (Xbox PDB). `this` holds the
/// indices `A` (`+0`) and `B` (`+4`) of the two axes projected on; `vertices` is the
/// polygon's first three vertices, 16 bytes each. Walks the three edges and leaves, at
/// `this + 0xC ..= 0x30`, the integrals over the projected polygon of `1`, `a`, `b`, `a^2`,
/// `a*b`, `b^2`, `a^3`, `a^2*b`, `a*b^2` and `b^3` in that order of the float slots
/// `+0xC, +0x10, +0x14, +0x18, +0x1C, +0x20, +0x24, +0x28, +0x2C, +0x30` (the
/// accumulations follow the exe's order and round to a float at each store; the slot's
/// names are unconfirmed).
pub fn internal_inertia_tensor_computer_comp_projection_integrals(
    e: &mut Engine,
    this: Ptr,
    vertices: Ptr,
) {
    let (t, v) = (this.addr(), vertices.addr());
    for offset in (0xc..=0x30).step_by(4) {
        store(e, t + offset, 0.0);
    }
    let (index_a, index_b) = (e.mem.u32(t), e.mem.u32(t + 4));
    let (mut at_a, mut at_b) = (
        v.wrapping_add(index_a.wrapping_mul(4)),
        v.wrapping_add(index_b.wrapping_mul(4)),
    );
    let three = e.mem.f64(THREE_D);
    let four = e.mem.f64(FOUR_D);
    for corner in 1..=3u32 {
        let (a0, b0) = (load(e, at_a), load(e, at_b));
        let next = (corner % 3) * 4;
        let a1 = load(
            e,
            v.wrapping_add(index_a.wrapping_add(next).wrapping_mul(4)),
        );
        let b1 = load(
            e,
            v.wrapping_add(index_b.wrapping_add(next).wrapping_mul(4)),
        );
        let da = rounded(a1 - a0);
        let db = rounded(b1 - b0);
        let a_sq = rounded(a0 * a0);
        let a_cu = rounded(a_sq * a0);
        let b_sq = rounded(b0 * b0);
        let b_cu = rounded(b_sq * b0);
        let a1_sq = rounded(a1 * a1);
        let b1_sq = rounded(b1 * b1);
        let b1_cu = rounded(b1_sq * b1);
        let sum_a = rounded(a1 + a0);
        let ca = rounded(sum_a * a1 + a_sq);
        let ka = rounded(ca * a1 + a_cu);
        let cb = rounded((b1 + b0) * b1 + b_sq);
        let kb = rounded(cb * b1 + b_cu);
        let two = (a1 + a1) * a0;
        let pa = rounded((two + a1_sq * three) + a_sq);
        let pb = rounded(three * a_sq + (a1_sq + two));
        accumulate(e, t + 0xc, sum_a * db);
        accumulate(e, t + 0x10, ca * db);
        accumulate(e, t + 0x18, ka * db);
        let a_4 = rounded(a_cu * a0);
        let la = rounded(ka * a1 + a_4);
        accumulate(e, t + 0x24, la * db);
        accumulate(e, t + 0x14, cb * da);
        accumulate(e, t + 0x20, kb * da);
        let b_4 = rounded(b_cu * b0);
        let lb = rounded(kb * b1 + b_4);
        accumulate(e, t + 0x30, lb * da);
        accumulate(e, t + 0x1c, db * (pa * b1 + pb * b0));
        let a1_cu = rounded(a1_sq * a1);
        let qa = rounded(four * a1_cu + pa * a0);
        let qb = rounded(pb * a1 + a_cu * four);
        accumulate(e, t + 0x28, (qa * b1 + qb * b0) * db);
        let x = (b1_sq * three) * b0;
        let s = four * b1_cu + x;
        let s2 = s + (b1 + b1) * b_sq;
        let sb = rounded(s2 + b_cu);
        let w = (b1_sq + b1_sq) * b0 + b1_cu;
        let z = ((three * b1) * b_sq + w) + b_cu * four;
        let tb = rounded(z);
        accumulate(e, t + 0x2c, (a1 * sb + a0 * tb) * da);
        at_a = at_a.wrapping_add(0x10);
        at_b = at_b.wrapping_add(0x10);
    }
    let half = e.mem.f64(HALF_D);
    store(e, t + 0xc, load(e, t + 0xc) * half);
    for (offset, divisor) in PROJECTION_DIVISORS {
        let divisor = e.mem.f64(divisor);
        store(e, t + offset, load(e, t + offset) / divisor);
    }
}

// Translated from 00cec3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InternalInertiaTensorComputer::compFaceIntegrals` (Xbox PDB): `this` holds the axis
/// indices `A` (`+0`), `B` (`+4`) and `C` (`+8`), the face `vertices` (first three, 16 bytes
/// each) and the face `normal`. First runs `compProjectionIntegrals(this, vertices)`; then,
/// with `w = ((-n0 * v0) - v1 * n1) - v2 * n2` and `k = 1 / n[C]`, writes the twelve floats
/// `this + 0x34 ..= 0x60` (in steps of 4): `Pa*k`, `Pb*k`, `-k^2 (nB*Pb + nA*Pa + w*P1)`,
/// `Paa*k`, `Pbb*k`, `k^3 (nA^2 Paa + 2 nA nB Pab + nB^2 Pbb + w (2 (nB Pb + nA Pa) + w P1))`,
/// `k*Paaa`, `k*Pbbb`, `-k^4 (...)` (the cubic terms), `k*Paab`, `-k^2 (nB Pbbb + nA Pabb +
/// Pbb w)` and `k^3 (...)` (`P..` the projection integrals at `+0xC ..= 0x30`, `nA` and `nB`
/// the normal's components `A` and `B`), each product rounded as the exe's temporaries are.
pub fn internal_inertia_tensor_computer_comp_face_integrals(
    e: &mut Engine,
    this: Ptr,
    vertices: Ptr,
    normal: Ptr,
) {
    internal_inertia_tensor_computer_comp_projection_integrals(e, this, vertices);
    let (t, v, n) = (this.addr(), vertices.addr(), normal.addr());
    let [n0, n1, n2] = [0, 4, 8].map(|offset| load(e, n + offset));
    let [v0, v1, v2] = [0, 4, 8].map(|offset| load(e, v + offset));
    let w = rounded(((-n0 * v0) - (v1 * n1)) - (v2 * n2));
    let (index_a, index_b, index_c) = (e.mem.u32(t), e.mem.u32(t + 4), e.mem.u32(t + 8));
    let component = |e: &Engine, index: u32| load(e, n.wrapping_add(index.wrapping_mul(4)));
    let n_c = component(e, index_c);
    let n_b = component(e, index_b);
    let n_a = component(e, index_a);
    let k = rounded(1.0 / n_c);
    let k2 = rounded(k * k);
    let k3 = rounded(k2 * k);
    let [p1, pa, pb, paa, pab, pbb, paaa, paab, pabb, pbbb] =
        [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30].map(|o| load(e, t + o));
    let c2 = e.mem.f64(TWO_D);
    let c3 = e.mem.f64(THREE_D);
    store(e, t + 0x34, pa * k);
    store(e, t + 0x38, pb * k);
    let p1w = w * p1;
    let negative_k2 = -k2;
    store(e, t + 0x3c, ((n_b * pb + n_a * pa) + p1w) * negative_k2);
    store(e, t + 0x40, paa * k);
    store(e, t + 0x44, pbb * k);
    let a_sq = rounded(n_a * n_a);
    let b_sq = rounded(n_b * n_b);
    let quadratic = (((n_a * c2) * n_b) * pab + a_sq * paa) + b_sq * pbb;
    let linear = n_b * pb + n_a * pa;
    store(e, t + 0x48, (quadratic + ((linear * c2 + p1w) * w)) * k3);
    store(e, t + 0x4c, k * paaa);
    store(e, t + 0x50, k * pbbb);
    let a_cu = rounded((n_a * n_a) * n_a);
    let b_cu = rounded((n_b * n_b) * n_b);
    let mut cubic = ((a_sq * c3) * n_b) * paab + a_cu * paaa;
    cubic += ((n_a * c3) * b_sq) * pabb;
    cubic += b_cu * pbbb;
    cubic += (((((n_a * c2) * n_b) * pab) + a_sq * paa) + b_sq * pbb) * (w * c3);
    cubic += (p1w + linear * c3) * (w * w);
    let k4 = rounded(k3 * k);
    store(e, t + 0x54, cubic * -k4);
    store(e, t + 0x58, k * paab);
    store(
        e,
        t + 0x5c,
        ((n_b * pbbb + n_a * pabb) + pbb * w) * negative_k2,
    );
    let mixed = (a_sq * paaa + ((n_a * c2) * n_b) * paab) + b_sq * pabb;
    let mixed = mixed + (((n_a * paa + pab * n_b) * c2 + pa * w) * w);
    store(e, t + 0x60, k3 * mixed);
}

// Translated from 00cec6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InternalInertiaTensorComputer::computeInertialTensorInternal` (Xbox PDB): `this` holds
/// the volume integrals the walk over a closed surface leaves (the sums at `+0x64` (the
/// divisor), `+0x68 ..= 0x70`, `+0x74 ..= 0x7C` and `+0x80 ..= 0x88`; their identity is
/// unconfirmed). `center_of_mass` gets `[+0x68 + 4 i] / [+0x64]` for the three lanes.
/// `inertia` gets the diagonal `([+0x7C] + [+0x78]) * density`, `([+0x7C] + [+0x74]) *
/// density`, `([+0x78] + [+0x74]) * density` and the off-diagonal pairs `-density * [+0x80]`,
/// `[+0x84] * -density`, `-density * [+0x88]` (pair slots `(1, 4)`, `(6, 9)`, `(8, 2)`), and
/// is then shifted to the centre of mass by `mass` exactly as `shiftInertiaToCom` does.
pub fn internal_inertia_tensor_computer_compute_inertial_tensor_internal(
    e: &mut Engine,
    this: Ptr,
    mass: f32,
    density: f32,
    center_of_mass: Ptr,
    inertia: Ptr,
) {
    // The x87 stack code with the x87 choice of NaN: `FLD` loads (quiet) and the memory
    // operands of `FDIV`/`FADD`/`FMUL` (a signalling NaN stays one there).
    let (t, com, m) = (this.addr(), center_of_mass.addr(), inertia.addr());
    let (fadd, fmul) = (x87_add, x87_mul);
    let ld = |e: &Engine, addr: u32| x87_load32(e.mem.u32(addr));
    let mem = |e: &Engine, addr: u32| x87_m32(e.mem.u32(addr));
    for lane in 0..3 {
        let quotient = x87_div(ld(e, t + 0x68 + 4 * lane), mem(e, t + 0x64));
        store(e, com + 4 * lane, quotient);
    }
    let rho = x87_load32(density.to_bits());
    store(e, m, fmul(fadd(ld(e, t + 0x7c), mem(e, t + 0x78)), rho));
    store(
        e,
        m + 0x14,
        fmul(fadd(ld(e, t + 0x7c), mem(e, t + 0x74)), rho),
    );
    store(
        e,
        m + 0x28,
        fmul(fadd(ld(e, t + 0x78), mem(e, t + 0x74)), rho),
    );
    // `FCHS` of the density, stored to a scratch float and loaded back.
    let negative = rounded(-rho);
    let xy = rounded(fmul(ld(e, t + 0x80), negative));
    store(e, m + 4, xy);
    store(e, m + 0x10, xy);
    let yz = rounded(fmul(ld(e, t + 0x84), negative));
    store(e, m + 0x18, yz);
    store(e, m + 0x24, yz);
    let zx = rounded(fmul(negative, mem(e, t + 0x88)));
    store(e, m + 0x20, zx);
    store(e, m + 8, zx);
    shift_to_centre(e, com, x87_load32(mass.to_bits()), m);
}

/// The float at `0x0104da48` (a large value) the bounding box search starts from, and the
/// vector of ones (`010ec7f0`) it combines with.
const LARGE_FLOAT: u32 = 0x0104_da48;
const AABB_ONES: u32 = 0x010e_c7f0;

// Translated from 00cec7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InternalInertiaTensorComputer::compGeometryAabb` (Xbox PDB): `vertices` is an array of
/// `count` 16-byte vectors. Both outputs are zeroed; nothing more happens for `count == 0`.
/// Otherwise the running minimum and maximum (starting at the large float and its negation,
/// the sign mask XORed in) take each vector with `MINPS`/`MAXPS` (vector as the destination
/// operand; none for a negative count), `center` gets `(1 - 0.5) * min + 0.5 * max` and
/// `extent` gets `max - min` (the full size of the box), in all four lanes.
pub fn internal_inertia_tensor_computer_comp_geometry_aabb(
    e: &mut Engine,
    vertices: Ptr,
    count: i32,
    center: Ptr,
    extent: Ptr,
) {
    zero_vector(e, center.addr());
    zero_vector(e, extent.addr());
    let large = e.mem.u32(LARGE_FLOAT);
    let sign = load_bits(e, SIGN_MASK);
    let mut minimum: Vec4 = [f32::from_bits(large); 4];
    let mut maximum: Vec4 = std::array::from_fn(|lane| f32::from_bits(sign[lane] ^ large));
    for index in 0..count.max(0) as u32 {
        let vector = load_vec(e, vertices.addr().wrapping_add(0x10 * index));
        minimum = std::array::from_fn(|lane| minps(vector[lane], minimum[lane]));
        maximum = std::array::from_fn(|lane| maxps(vector[lane], maximum[lane]));
    }
    if count == 0 {
        return;
    }
    let half: f32 = e.global(HALF_F32);
    let ones = load_vec(e, AABB_ONES);
    let middle: Vec4 =
        std::array::from_fn(|lane| (ones[lane] - half) * minimum[lane] + half * maximum[lane]);
    store_vec(e, center.addr(), middle);
    let size: Vec4 = std::array::from_fn(|lane| maximum[lane] - minimum[lane]);
    store_vec(e, extent.addr(), size);
}

/// The float at `0x010cb428`: the Jacobi sweeps stop once the largest squared off-diagonal
/// element drops below it.
const JACOBI_THRESHOLD: u32 = 0x010c_b428;
/// The float -1.0 (`01012054`).
const MINUS_ONE_F32: u32 = 0x0101_2054;

// Translated from 00cec880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a Jacobi eigen-decomposition of the symmetric 3 x 4 float
/// matrix `matrix` (three 16-byte rows), at most five rotations. `out` starts as the identity
/// and each rotation zeroes the largest squared off-diagonal element of `(0,1)`, `(0,2)`,
/// `(1,2)` (the first of equal ones; the loop ends once it is below the float at
/// `010cb428`): with `theta = (m[q][q] - m[p][p]) / (2 m[q][p])`, `t = sign / (|theta| +
/// sqrt(theta^2 + 1))` (sign +1 only for `theta > 0`), `c = 1 / sqrt(t^2 + 1)` and `s = c t`
/// (all rounded to floats, the square roots through the CRT's `00ec6040`), the rotation
/// matrix `G` has `c` at `(p,p)` and `(q,q)` and `s`, `-s` at `(p,q)`, `(q,p)`. `H = G *
/// matrix`, then `matrix = H * G^T` and `out = out * G^T` (`hkMatrix3::setMul`, `00cd55b0`;
/// the input matrix is overwritten). Finally the rows of `out` are normalised, row 1 is
/// rebuilt as `row2 x row0` and normalised, row 2 as `row0 x row1` and normalised
/// (one refined `RSQRTSS` each).
pub fn fn_00cec880(e: &mut Engine, matrix: Ptr, out: Ptr) {
    let (m, o) = (matrix.addr(), out.addr());
    for row in 0..3 {
        zero_vector(e, o + 0x10 * row);
    }
    for diagonal in [0, 0x14, 0x28] {
        e.mem.set_f32(o + diagonal, 1.0);
    }
    let one = e.mem.f64(ONE_D);
    for _ in 0..5 {
        let square = |e: &Engine, offset: u32| rounded(load(e, m + offset) * load(e, m + offset));
        let (mut best, mut p, mut q) = (square(e, 4), 0u32, 1u32);
        let second = square(e, 8);
        if best < second {
            best = second;
            q = 2;
        }
        let third = square(e, 0x18);
        if best < third {
            best = third;
            p = 1;
            q = 2;
        }
        let threshold = f64::from(e.global::<f32>(JACOBI_THRESHOLD));
        if threshold > best {
            break;
        }
        let diagonal_q = load(e, m + q * 20);
        let diagonal_p = load(e, m + p * 20);
        let off_diagonal = load(e, m + (p + 4 * q) * 4);
        let theta = rounded((diagonal_q - diagonal_p) / (off_diagonal + off_diagonal));
        let sign = if theta > 0.0 {
            1.0
        } else {
            f64::from(e.global::<f32>(MINUS_ONE_F32))
        };
        let root = rounded(e.call(X87_SQRT, &args![rounded(theta * theta + one)]).f64());
        let tangent = rounded(sign / (root + rounded(theta.abs())));
        let hypotenuse = rounded(
            e.call(X87_SQRT, &args![rounded(tangent * tangent + one)])
                .f64(),
        );
        let cosine = rounded(1.0 / hypotenuse);
        let sine = rounded(cosine * tangent);
        e.with_stack(0x60, |e, locals| {
            let (g, h) = (locals.addr(), locals.addr() + 0x30);
            for row in 0..3 {
                zero_vector(e, g + 0x10 * row);
            }
            for diagonal in [0, 0x14, 0x28] {
                e.mem.set_f32(g + diagonal, 1.0);
            }
            store(e, g + p * 20, cosine);
            store(e, g + q * 20, cosine);
            store(e, g + (q * 4 + p) * 4, -sine);
            store(e, g + (p * 4 + q) * 4, sine);
            e.call(MATRIX_SET_MUL, &args![h, g, m]);
            store(e, g + (q * 4 + p) * 4, sine);
            store(e, g + (p * 4 + q) * 4, -sine);
            e.call(MATRIX_SET_MUL, &args![m, h, g]);
            e.call(MATRIX_SET_MUL, &args![h, o, g]);
            for row in 0..3 {
                copy_vec(e, h + 0x10 * row, o + 0x10 * row);
            }
        });
    }
    let row0 = normalised(e, load_vec(e, o));
    store_vec(e, o, row0);
    let row1 = normalised(e, cross(load_vec(e, o + 0x20), row0));
    store_vec(e, o + 0x10, row1);
    let row2 = normalised(e, cross(row0, row1));
    store_vec(e, o + 0x20, row2);
}

/// The convex hull builder the mass-properties function drives (class and member names
/// unconfirmed; the roles are read from the code that calls them): the constructor of the
/// hull object (`00d76ca0`, `this` = the object) and of its configuration (`00d74d40`), the
/// build (`00d77200`: `this` = hull, `(source, configuration)`), the dimension count
/// (`00d74e10`, -1 when it failed, 3 for a solid), the mass-property pass (`00d762f0`), the
/// surface area and the volume (`00d75390`, `00d752e0`: floats in `ST0`), the expansion by a
/// margin (`00d77be0`, one float), the centre (`00d75600`: a pointer to a 16-byte vector),
/// the three rows (`00d75560`: a pointer to three 16-byte vectors) and the destructor
/// (`00d76940`).
const HULL_CONSTRUCT: u32 = 0x00d7_6ca0;
const HULL_CONFIG_CONSTRUCT: u32 = 0x00d7_4d40;
const HULL_BUILD: u32 = 0x00d7_7200;
const HULL_DIMENSIONS: u32 = 0x00d7_4e10;
const HULL_BUILD_MASS: u32 = 0x00d7_62f0;
const HULL_SURFACE_AREA: u32 = 0x00d7_5390;
const HULL_VOLUME: u32 = 0x00d7_52e0;
const HULL_EXPAND: u32 = 0x00d7_7be0;
const HULL_CENTRE: u32 = 0x00d7_5600;
const HULL_ROWS: u32 = 0x00d7_5560;
const HULL_DESTRUCT: u32 = 0x00d7_6940;
/// The float the first margin comes from (divided by the double 5.0 at `01020998`), the
/// double 12.566370964050293 (4 pi as a float) and the double 1.1920928955078125e-05.
const MARGIN_SEED: u32 = 0x011b_153c;
const FIVE_D: u32 = 0x0102_0998;
const FOUR_PI_D: u32 = 0x010c_bf50;
const MINIMUM_VOLUME_D: u32 = 0x010c_bf48;

// Translated from 00cebbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeConvexHullMassProperties` (Xbox PDB): builds the hull of
/// `source` (the configuration byte at `+0` set to 1) and answers 1 when that fails
/// (dimension count -1). The first margin is `[011b153c] / 5.0` as a float. With three
/// dimensions the mass pass runs and, for `radius` above zero, the "surplus" volume
/// `area * radius + radius * ((radius * 4 pi) * radius) / 3` is formed (a float, kept
/// separately; zero otherwise). Then, while the hull is not three-dimensional or its volume is
/// below 1.19e-5 (a NaN volume ends the loop), it is rebuilt, expanded by the margin and the
/// margin doubled. Finally `out + 0x10` gets the centre, `out + 0x20 ..= 0x40` the three rows,
/// `out + 0` the volume plus the surplus and `out + 4` gets 1.0; when both that sum and the
/// surplus are above zero the three rows are scaled (`00cd4c70`) by `surplus / sum + 1`.
/// The hull is destroyed on every path; answers 0 on success.
pub fn hkp_inertia_tensor_computer_compute_convex_hull_mass_properties(
    e: &mut Engine,
    source: Ptr,
    radius: f32,
    out: Ptr,
) -> u32 {
    let (source, out) = (source.addr(), out.addr());
    e.with_stack(0x90, |e, frame| {
        let hull = frame.addr();
        let config = hull + 0x40;
        let vector = hull + 0x70;
        e.call(HULL_CONSTRUCT, &args![hull]);
        e.call(HULL_CONFIG_CONSTRUCT, &args![config]);
        e.mem.set_u8(config, 1);
        e.call(HULL_BUILD, &args![hull, source, config]);
        if e.call(HULL_DIMENSIONS, &args![hull]).u32() == u32::MAX {
            e.call(HULL_DESTRUCT, &args![hull]);
            return 1;
        }
        let mut margin = (f64::from(e.global::<f32>(MARGIN_SEED)) / e.mem.f64(FIVE_D)) as f32;
        let mut surplus = 0.0f32;
        if e.call(HULL_DIMENSIONS, &args![hull]).u32() == 3 {
            e.call(HULL_BUILD_MASS, &args![hull]);
            if 0.0 < radius {
                let area = e.call(HULL_SURFACE_AREA, &args![hull]).f64();
                let r = f64::from(radius);
                let shell = (area * r) as f32;
                let ball = r * ((r * e.mem.f64(FOUR_PI_D)) * r);
                surplus = (f64::from(shell) + ball / e.mem.f64(THREE_D)) as f32;
            }
        }
        loop {
            let rebuild = if e.call(HULL_DIMENSIONS, &args![hull]).u32() == 3 {
                e.call(HULL_VOLUME, &args![hull]).f64() < e.mem.f64(MINIMUM_VOLUME_D)
            } else {
                true
            };
            if !rebuild {
                break;
            }
            e.call(HULL_BUILD, &args![hull, source, config]);
            e.call(HULL_EXPAND, &args![hull, margin]);
            if e.call(HULL_DIMENSIONS, &args![hull]).u32() == 3 {
                e.call(HULL_BUILD_MASS, &args![hull]);
            }
            margin = (f64::from(margin) + f64::from(margin)) as f32;
        }
        let centre = e.call(HULL_CENTRE, &args![hull]).u32();
        copy_vec(e, centre, out + CENTRE);
        let rows = e.call(HULL_ROWS, &args![hull]).u32();
        for row in 0..3 {
            copy_vec(e, rows + 0x10 * row, out + INERTIA + 0x10 * row);
        }
        let volume = e.call(HULL_VOLUME, &args![hull]).f64();
        let total = (volume + f64::from(surplus)) as f32;
        e.mem.set_f32(out + VOLUME, total);
        e.mem.set_f32(out + MASS, 1.0);
        if 0.0 < total && 0.0 < surplus {
            let factor = (f64::from(surplus) / f64::from(total) + e.mem.f64(ONE_D)) as f32;
            store_vec(e, vector, [factor, 0.0, 0.0, 0.0]);
            e.call(MATRIX_SCALE, &args![out + INERTIA, vector]);
        }
        e.call(HULL_DESTRUCT, &args![hull]);
        0
    })
}

// Translated from 00cecc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the mass of the record at `this` to `mass` and scales the
/// three rows at `this + 0x20` (`00cd4c70`) by `mass / old mass`, a double division rounded to
/// a float.
pub fn fn_00cecc90(e: &mut Engine, this: Ptr, mass: f32) {
    let t = this.addr();
    let quiet = x87_quiet(mass);
    let scale = (f64::from(quiet) / load(e, t + MASS)) as f32;
    e.mem.set_f32(t + MASS, quiet);
    scale_rows(e, t + INERTIA, scale);
}

// Translated from 00ceccf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeSphereSurfaceMassProperties` (Xbox PDB): a hollow sphere
/// of outer `radius` and wall `thickness`. Answers 1 (nothing written) when `mass` is not
/// above zero, `thickness` is not below `radius` or `thickness` is not above zero (a NaN passes
/// the second and third tests). The record at `out` becomes the outer solid sphere (volume
/// `((radius * 4.18879) * radius) * radius`, mass the same number) less the inner one (of
/// radius `radius - thickness`, built the same way), both through
/// `computeSphereVolumeMassProperties`: the centre is zero, the rows are the outer rows less
/// the inner rows (`00cd5180`), the mass and volume are the differences, and everything is then
/// scaled (`00cebfd0`) by `mass` over the difference of the two volume numbers. Answers 0.
pub fn hkp_inertia_tensor_computer_compute_sphere_surface_mass_properties(
    e: &mut Engine,
    radius: f32,
    mass: f32,
    thickness: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || thickness >= radius || thickness <= 0.0 {
        return 1;
    }
    let out = out.addr();
    let inner = (f64::from(radius) - f64::from(thickness)) as f32;
    let factor = e.mem.f64(SPHERE_VOLUME_FACTOR);
    let r = f64::from(radius);
    let t = f64::from(inner);
    let outer_volume = (((r * factor) * r) * r) as f32;
    let inner_volume = (((factor * t) * t) * t) as f32;
    e.with_stack(0xa0, |e, frame| {
        let (outer, hollow) = (frame.addr(), frame.addr() + 0x50);
        hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(
            e,
            radius,
            outer_volume,
            Ptr::new(outer),
        );
        hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(
            e,
            inner,
            inner_volume,
            Ptr::new(hollow),
        );
        subtract_record(e, outer, hollow, out);
        let scale = (f64::from(mass) / (f64::from(outer_volume) - f64::from(inner_volume))) as f32;
        fn_00cebfd0(e, Ptr::new(out), scale);
    });
    0
}

/// What the two surface functions do with the solid they carved out: `out` gets a zero
/// centre, the rows of `outer` less the rows of `hollow` (`00c6cd00` copies, `00cd5180`
/// subtracts) and the differences of mass and volume.
fn subtract_record(e: &mut Engine, outer: u32, hollow: u32, out: u32) {
    zero_vector(e, out + CENTRE);
    e.call(COPY_ROWS, &args![out + INERTIA, outer + INERTIA]);
    e.call(MATRIX_SUB, &args![out + INERTIA, hollow + INERTIA]);
    store(
        e,
        out + MASS,
        load(e, outer + MASS) - load(e, hollow + MASS),
    );
    store(
        e,
        out + VOLUME,
        load(e, outer + VOLUME) - load(e, hollow + VOLUME),
    );
}

// Translated from 00cece50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeBoxSurfaceMassProperties` (Xbox PDB): a hollow box of
/// the three half extents at `half_extents` and wall `thickness`. Answers 1 (nothing written)
/// when `mass` is not above zero, `thickness` is not below each half extent (a NaN fails) or
/// `thickness` is not above zero. Otherwise like `00ceccf0`: the outer solid box (volume
/// `(((x * 8) * y) * z)`, built by `computeBoxVolumeMassProperties`) less the inner one, whose
/// half extents are each reduced by `thickness`, scaled by `mass` over the difference of the
/// two volumes. Answers 0.
pub fn hkp_inertia_tensor_computer_compute_box_surface_mass_properties(
    e: &mut Engine,
    half_extents: Ptr,
    mass: f32,
    thickness: f32,
    out: Ptr,
) -> u32 {
    let h = half_extents.addr();
    let [x, y, z] = [0, 4, 8].map(|offset| e.mem.f32(h + offset));
    let fits = thickness < x && y > thickness && z > thickness;
    if mass <= 0.0 || !fits || thickness <= 0.0 {
        return 1;
    }
    let out = out.addr();
    let reduce = |half: f32| (f64::from(half) - f64::from(thickness)) as f32;
    let inner = [reduce(x), reduce(y), reduce(z)];
    let eight = e.mem.f64(EIGHT);
    let outer_volume = (((f64::from(x) * eight) * f64::from(y)) * f64::from(z)) as f32;
    let inner_volume =
        (((eight * f64::from(inner[0])) * f64::from(inner[1])) * f64::from(inner[2])) as f32;
    e.with_stack(0xd0, |e, frame| {
        let (inner_extents, outer, hollow) =
            (frame.addr(), frame.addr() + 0x10, frame.addr() + 0x60);
        store_vec(e, inner_extents, [inner[0], inner[1], inner[2], 0.0]);
        hkp_inertia_tensor_computer_compute_box_volume_mass_properties(
            e,
            half_extents,
            outer_volume,
            Ptr::new(outer),
        );
        hkp_inertia_tensor_computer_compute_box_volume_mass_properties(
            e,
            Ptr::new(inner_extents),
            inner_volume,
            Ptr::new(hollow),
        );
        subtract_record(e, outer, hollow, out);
        let scale = (f64::from(mass) / (f64::from(outer_volume) - f64::from(inner_volume))) as f32;
        fn_00cebfd0(e, Ptr::new(out), scale);
    });
    0
}

// Translated from 00ced090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::combineMassProperties` (Xbox PDB): `list` holds a pointer to
/// `count` (the word at `+4`) 144-byte elements, each a mass-properties record followed by a
/// transform (three rows at `+0x50`, translation at `+0x80`). Answers 1 (nothing written
/// but the zeroed accumulators) when `count` is not above zero or the summed mass is not
/// above zero (a NaN passes). Otherwise `out` gets the summed volume and mass, the mass
/// weighted mean of the transformed centres (`00c7f640`) as its centre, and the sum of the
/// element rows brought into the common frame (`00cd5880`), shifted by the distance of the
/// element centre from that mean (`00cebf30`) and added (`00cd5140`). Answers 0.
pub fn hkp_inertia_tensor_computer_combine_mass_properties(
    e: &mut Engine,
    list: Ptr,
    out: Ptr,
) -> u32 {
    let (list, out) = (list.addr(), out.addr());
    let count = e.mem.i32(list + 4);
    if count <= 0 {
        return 1;
    }
    e.with_stack(0x50, |e, frame| {
        let (position, rows, offset) = (frame.addr(), frame.addr() + 0x10, frame.addr() + 0x40);
        let element = |e: &Engine, index: i32| e.mem.u32(list).wrapping_add(0x90 * index as u32);
        let mut mass_sum = 0.0f32;
        let mut volume_sum = 0.0f32;
        let mut weighted: Vec4 = [0.0; 4];
        for index in 0..count {
            let item = element(e, index);
            e.call(TRANSFORM_POINT, &args![position, item + 0x50, item + 0x10]);
            let mass = e.mem.f32(item + MASS);
            let moved = load_vec(e, position);
            mass_sum = (f64::from(mass_sum) + f64::from(mass)) as f32;
            volume_sum = (load(e, item + VOLUME) + f64::from(volume_sum)) as f32;
            weighted = std::array::from_fn(|lane| mass * moved[lane] + weighted[lane]);
        }
        if mass_sum <= 0.0 {
            return 1;
        }
        let inverse = (1.0 / f64::from(mass_sum)) as f32;
        let centre = scaled(weighted, inverse);
        e.mem.set_f32(out + MASS, mass_sum);
        store_vec(e, out + CENTRE, centre);
        e.mem.set_f32(out + VOLUME, volume_sum);
        for row in 0..3 {
            zero_vector(e, out + INERTIA + 0x10 * row);
        }
        for index in 0..count {
            let item = element(e, index);
            for row in 0..3 {
                copy_vec(e, item + INERTIA + 0x10 * row, rows + 0x10 * row);
            }
            e.call(CHANGE_BASIS, &args![rows, item + 0x50]);
            e.call(TRANSFORM_POINT, &args![position, item + 0x50, item + 0x10]);
            let moved = load_vec(e, position);
            store_vec(
                e,
                offset,
                std::array::from_fn(|lane| moved[lane] - centre[lane]),
            );
            fn_00cebf30(
                e,
                Ptr::new(offset),
                x87_quiet(e.mem.f32(item + MASS)),
                Ptr::new(rows),
            );
            e.call(MATRIX_ADD, &args![out + INERTIA, rows]);
        }
        0
    })
}

// Translated from 00ced310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the largest of the three diagonal floats of the 3 x 4 matrix
/// `matrix` (taking the later one only when the earlier is not below it) written back as
/// the diagonal of an otherwise zero matrix.
pub fn fn_00ced310(e: &mut Engine, matrix: Ptr) {
    let m = matrix.addr();
    let first = x87_quiet(e.mem.f32(m));
    let second = x87_quiet(e.mem.f32(m + 0x14));
    let larger = if second < first { first } else { second };
    let third = x87_quiet(e.mem.f32(m + 0x28));
    let largest = if third < larger { larger } else { third };
    for row in 0..3 {
        zero_vector(e, m + 0x10 * row);
    }
    for diagonal in [0, 0x14, 0x28] {
        e.mem.set_f32(m + diagonal, largest);
    }
}

// Translated from 00ced390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `InternalInertiaTensorComputer::compVolumeIntegrals` (Xbox PDB): `mesh` holds the vertex
/// array (`+0`, 16-byte vectors), the face array (`+0xC`, 16 bytes each, the three vertex
/// indices first) and the face count (`+0x10`). The ten floats at `this + 0x64 ..= 0x88` are
/// cleared; then every face whose normal (the cross product of two edges of the vertices
/// moved by `offset`) has a length above zero adds its terms. The normal is made unit length
/// with a double division, the axis `C` of its largest component (the third axis on a tie
/// between the second and third) goes to `+8`, `A = (C + 1) mod 3` to `+0` and `B = (A + 1)
/// mod 3` to `+4`, `compFaceIntegrals` runs, and the unit normal's lanes multiply the face
/// integrals: `+0x34` (when `A` is 0, else `+0x38` when `B` is 0, else `+0x3C`)
/// times lane 0 goes to `+0x64`; lanes `A`, `B`, `C` times the floats at `+0x40`,
/// `+0x44`, `+0x48` go to the triple at `+0x68`, times `+0x4C`, `+0x50`, `+0x54` to the triple
/// at `+0x74` and times `+0x58`, `+0x5C`, `+0x60` to the triple at `+0x80` (each into its
/// lane's slot). At the end the triples at `+0x68` and `+0x80` are halved and the one at
/// `+0x74` divided by 3 (the identity of the sums is unconfirmed).
pub fn internal_inertia_tensor_computer_comp_volume_integrals(
    e: &mut Engine,
    this: Ptr,
    mesh: Ptr,
    offset: Ptr,
) {
    let (t, mesh) = (this.addr(), mesh.addr());
    for slot in 0..10 {
        store(e, t + 0x64 + 4 * slot, 0.0);
    }
    let count = e.mem.i32(mesh + 0x10);
    if count > 0 {
        let shift = load_vec(e, offset.addr());
        e.with_stack(0x40, |e, frame| {
            let (corners, normal) = (frame.addr(), frame.addr() + 0x30);
            for face in 0..count as u32 {
                let entry = e.mem.u32(mesh + 0xc).wrapping_add(0x10 * face);
                let vertices = e.mem.u32(mesh);
                let [first, second, third] = [0, 4, 8].map(|offset| e.mem.u32(entry + offset));
                let moved = |e: &Engine, index: u32| {
                    let vertex = load_vec(e, vertices.wrapping_add(index << 4));
                    std::array::from_fn::<f32, 4, _>(|lane| vertex[lane] + shift[lane])
                };
                let (v0, v1, v2) = (moved(e, first), moved(e, second), moved(e, third));
                store_vec(e, corners, v0);
                store_vec(e, corners + 0x10, v1);
                store_vec(e, corners + 0x20, v2);
                let edge_1: Vec4 = std::array::from_fn(|lane| v1[lane] - v0[lane]);
                let edge_2: Vec4 = std::array::from_fn(|lane| v2[lane] - v0[lane]);
                let raw = cross(edge_1, edge_2);
                let length = sum3(mul(raw, raw)).sqrt();
                if length > 0.0 {
                    let unit = scaled(raw, (1.0 / f64::from(length)) as f32);
                    store_vec(e, normal, unit);
                    let [ax, ay, az] = [unit[0].abs(), unit[1].abs(), unit[2].abs()];
                    let axis_c = if ay < ax && az < ax {
                        0
                    } else if ay > az {
                        1
                    } else {
                        2
                    };
                    let axis_a = (axis_c + 1) % 3;
                    let axis_b = (axis_a + 1) % 3;
                    e.mem.set_u32(t + 8, axis_c);
                    e.mem.set_u32(t, axis_a);
                    e.mem.set_u32(t + 4, axis_b);
                    internal_inertia_tensor_computer_comp_face_integrals(
                        e,
                        this,
                        Ptr::new(corners),
                        Ptr::new(normal),
                    );
                    let selected = if axis_a == 0 {
                        load(e, t + 0x34)
                    } else if axis_b == 0 {
                        load(e, t + 0x38)
                    } else {
                        load(e, t + 0x3c)
                    };
                    accumulate(e, t + 0x64, f64::from(unit[0]) * selected);
                    let lanes = [axis_a, axis_b, axis_c];
                    for (group, base) in [0x68u32, 0x74, 0x80].into_iter().enumerate() {
                        for (position, lane) in lanes.into_iter().enumerate() {
                            let factor = t + 0x40 + 0xc * group as u32 + 4 * position as u32;
                            accumulate(
                                e,
                                t + base + 4 * lane,
                                f64::from(unit[lane as usize]) * load(e, factor),
                            );
                        }
                    }
                }
            }
        });
    }
    let half = e.mem.f64(HALF_D);
    for offset in [0x68, 0x6c, 0x70, 0x80, 0x84, 0x88] {
        store(e, t + offset, load(e, t + offset) * half);
    }
    let three = e.mem.f64(THREE_D);
    for offset in [0x74, 0x78, 0x7c] {
        store(e, t + offset, load(e, t + offset) / three);
    }
}

/// The double 0.1 (`0101ffa0`) the smallest diagonal entries are compared with.
const TENTH_D: u32 = 0x0101_ffa0;

/// What `00ced6b0` and `00ced8e0` share. The bounding box of the vertices gives a centre and
/// an extent; the box mass properties of that extent (`computeBoxVolumeMassProperties`) are
/// kept for a fallback. The volume integrals (`compVolumeIntegrals`, vertices moved by the
/// negated centre) give the volume in the float at `+0x64`. Answers `false` when it is not
/// above zero (a NaN counts as above), after copying the box properties into `out` when
/// `fallback` is set. Otherwise `out` gets that volume, `mass` and the tensor of
/// `computeInertialTensorInternal` for the density `mass / volume`, the centre moved back by
/// the bounding box centre and every diagonal entry below a tenth of the box tensor's replaced
/// by that tenth (the centre lane by the bounding box centre's and the off-diagonal entries of
/// its row and column by the larger of themselves and zero).
fn geometry_volume_mass_properties(
    e: &mut Engine,
    geometry: Ptr,
    mass: f32,
    out: Ptr,
    fallback: bool,
) -> bool {
    let (geometry, out) = (geometry.addr(), out.addr());
    e.with_stack(0x120, |e, frame| {
        let base = frame.addr();
        let (extent, centre, boxed, integrals) =
            (base + 0x20, base + 0x30, base + 0x40, base + 0x90);
        let vertices = e.mem.u32(geometry);
        let count = e.mem.i32(geometry + 4);
        internal_inertia_tensor_computer_comp_geometry_aabb(
            e,
            Ptr::new(vertices),
            count,
            Ptr::new(centre),
            Ptr::new(extent),
        );
        hkp_inertia_tensor_computer_compute_box_volume_mass_properties(
            e,
            Ptr::new(extent),
            mass,
            Ptr::new(boxed),
        );
        let middle = load_vec(e, centre);
        store_vec(e, boxed + CENTRE, middle);
        let sign = load_bits(e, SIGN_MASK);
        store_bits(
            e,
            extent,
            std::array::from_fn(|lane| sign[lane] ^ middle[lane].to_bits()),
        );
        internal_inertia_tensor_computer_comp_volume_integrals(
            e,
            Ptr::new(integrals),
            Ptr::new(geometry),
            Ptr::new(extent),
        );
        let volume = e.mem.f32(integrals + 0x64);
        if volume <= 0.0 {
            if fallback {
                e.mem
                    .set_f32(out + VOLUME, x87_quiet(e.mem.f32(boxed + VOLUME)));
                e.mem
                    .set_f32(out + MASS, x87_quiet(e.mem.f32(boxed + MASS)));
                store_vec(e, out + CENTRE, middle);
                for row in 0..3 {
                    copy_vec(e, boxed + INERTIA + 0x10 * row, out + INERTIA + 0x10 * row);
                }
            }
            return false;
        }
        let mass = x87_quiet(mass);
        e.mem.set_f32(out + VOLUME, x87_quiet(volume));
        e.mem.set_f32(out + MASS, mass);
        let density = (f64::from(mass) / f64::from(volume)) as f32;
        internal_inertia_tensor_computer_compute_inertial_tensor_internal(
            e,
            Ptr::new(integrals),
            mass,
            density,
            Ptr::new(out + CENTRE),
            Ptr::new(out + INERTIA),
        );
        let moved = load_vec(e, out + CENTRE);
        store_vec(
            e,
            out + CENTRE,
            std::array::from_fn(|lane| moved[lane] + middle[lane]),
        );
        let tenth = e.mem.f64(TENTH_D);
        for index in 0..3u32 {
            let diagonal = out + INERTIA + 0x14 * index;
            let threshold = load(e, boxed + INERTIA + 0x14 * index) * tenth;
            if load(e, diagonal) < threshold {
                store(e, diagonal, threshold);
                e.mem
                    .set_f32(out + CENTRE + 4 * index, middle[index as usize]);
                for other in (0..3u32).filter(|other| *other != index) {
                    let entry = out + INERTIA + 0x10 * index + 4 * other;
                    let value = x87_quiet(e.mem.f32(entry));
                    let value = if value < 0.0 { 0.0 } else { value };
                    e.mem.set_f32(entry, value);
                    e.mem
                        .set_f32(out + INERTIA + 0x10 * other + 4 * index, value);
                }
            }
        }
        true
    })
}

// Translated from 00ced6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeGeometryVolumeMassProperties` (Xbox PDB): `geometry`
/// holds the vertex array (`+0`) and the triangle array (`+0xC`). A `mass` that is not above
/// zero (a NaN stays) is replaced by 1.0. See `geometry_volume_mass_properties`; when the
/// volume integral is not above zero `out` takes the bounding box's properties.
pub fn hkp_inertia_tensor_computer_compute_geometry_volume_mass_properties(
    e: &mut Engine,
    geometry: Ptr,
    mass: f32,
    out: Ptr,
) {
    let mass = if mass <= 0.0 { 1.0 } else { mass };
    geometry_volume_mass_properties(e, geometry, mass, out, true);
}

// Translated from 00ced8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeGeometryVolumeMassPropertiesChecked` (Xbox PDB): answers
/// 1 and writes nothing when `mass` is not above zero (a NaN passes) or when the volume
/// integral is not above zero (a NaN passes); otherwise like `00ced6b0` and answers 0.
pub fn hkp_inertia_tensor_computer_compute_geometry_volume_mass_properties_checked(
    e: &mut Engine,
    geometry: Ptr,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 {
        return 1;
    }
    u32::from(!geometry_volume_mass_properties(
        e, geometry, mass, out, false,
    ))
}

/// `00d5ac50` (`hkgeometryutility.cpp`, unnamed in the engine map): a pointer to a byte the
/// check `00d5a6e0` fills, the rotation rows and three output vectors; the vectors are the
/// rows (the identity when the byte stays 0, the third negated when the triple product of the
/// rows is negative).
const ROTATION_BASIS: u32 = 0x00d5_ac50;
/// The vector of `0x7fffffff` words (`01017d10`) that clears sign bits.
const ABS_MASK: u32 = 0x0101_7d10;

/// The axis of the largest of the first three lanes of `abs` (the later axis only when
/// strictly larger).
fn largest_axis(abs: Vec4) -> usize {
    if abs[1] > abs[0] {
        if abs[2] > abs[1] {
            2
        } else {
            1
        }
    } else if abs[2] > abs[0] {
        2
    } else {
        0
    }
}

// Translated from 00cedae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: finds the two longest axes of the bounds of `shape` in the
/// rotated frame of `rotation` (three rows). `00d5ac50` makes three vectors `a`, `b`, `c` from
/// the rotation; the matrix `M` with the rows `(a_i, b_i, c_i, 0)` (and a zero fourth row) is
/// passed to the shape's virtual function at `+0x1c` with the float 0.0 and a buffer for a
/// bounding box (minimum, maximum). The extent `maximum - minimum` loses its largest lane
/// (by absolute value; a later lane has to be strictly larger), then its largest remaining
/// lane picks the second axis; `out_1` and `out_2` receive the rows of `M` of the two axes
/// (a 16-byte copy each).
pub fn fn_00cedae0(e: &mut Engine, shape: Ptr, rotation: Ptr, out_1: Ptr, out_2: Ptr) {
    let rotation = rotation.addr();
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr();
        let (flag, column_a, column_c, column_b) =
            (base + 0xf, base + 0x10, base + 0x20, base + 0x30);
        let (matrix, bounds, copy) = (base + 0x40, base + 0x80, base + 0xa0);
        for row in 0..3 {
            copy_vec(e, rotation + 0x10 * row, copy + 0x10 * row);
        }
        e.call(
            ROTATION_BASIS,
            &args![flag, copy, column_a, column_b, column_c],
        );
        let [a, b, c] = [column_a, column_b, column_c].map(|column| load_vec(e, column));
        for row in 0..3 {
            store_vec(e, matrix + 0x10 * row as u32, [a[row], b[row], c[row], 0.0]);
        }
        store_vec(e, matrix + 0x30, [0.0; 4]);
        e.vcall(shape.addr(), 0x1c, &args![matrix, 0.0f32, bounds]);
        let (minimum, maximum) = (load_vec(e, bounds), load_vec(e, bounds + 0x10));
        let mask = load_bits(e, ABS_MASK);
        let absolute = |extent: Vec4| -> Vec4 {
            std::array::from_fn(|lane| f32::from_bits(extent[lane].to_bits() & mask[lane]))
        };
        let mut extent: Vec4 = std::array::from_fn(|lane| maximum[lane] - minimum[lane]);
        let first = largest_axis(absolute(extent));
        extent[first] = 0.0;
        let second = largest_axis(absolute(extent));
        copy_vec(e, matrix + 0x10 * first as u32, out_1.addr());
        copy_vec(e, matrix + 0x10 * second as u32, out_2.addr());
    });
}

// Translated from 00cedcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the mass properties of `count` equal point masses. `points` is
/// the first of `count` positions (three floats each, `stride` bytes apart); answers 1 when
/// `count` is not above zero. The positions are copied into an array of 16-byte vectors (a
/// fourth lane of 0.0, the array reserved through the memory router and released again), the
/// record at `out` gets `mass` (`+4`; the volume is not written), a zero tensor, the mean
/// position as its centre (`sum * (1 / count)`) and, for each point's offset `d` from that
/// centre, `(d_j^2 + d_k^2) * (mass / count)` added to the diagonal and `(d_i * d_j) * (mass /
/// count)` subtracted from the off-diagonal pairs. Answers 0.
pub fn fn_00cedcb0(
    e: &mut Engine,
    points: Ptr,
    stride: i32,
    count: i32,
    mass: f32,
    out: Ptr,
) -> u32 {
    if count <= 0 {
        return 1;
    }
    let (points, out) = (points.addr(), out.addr());
    e.with_stack(12, |e, array| {
        let a = array.addr();
        e.mem.set_u32(a + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(a + 8) & 0x3fff_ffff;
        if (capacity as i32) < count {
            let doubled = capacity.wrapping_mul(2);
            let wanted = if count < doubled as i32 {
                doubled
            } else {
                count as u32
            };
            e.call(ARRAY_RESERVE, &args![allocator, a, wanted, 0x10u32]);
        }
        e.mem.set_u32(a + 4, count as u32);
        let data = e.mem.u32(a);
        for index in 0..count {
            let point = points.wrapping_add((stride as u32).wrapping_mul(index as u32));
            let slot = data.wrapping_add(0x10 * index as u32);
            for lane in 0..3 {
                let coordinate = x87_quiet(e.mem.f32(point + 4 * lane));
                e.mem.set_f32(slot + 4 * lane, coordinate);
            }
            e.mem.set_f32(slot + 12, 0.0);
        }
        e.mem.set_f32(out + MASS, x87_quiet(mass));
        for offset in [CENTRE, INERTIA, INERTIA + 0x10, INERTIA + 0x20] {
            zero_vector(e, out + offset);
        }
        let points_f32 = count as f32;
        let per_point = (f64::from(x87_quiet(mass)) / f64::from(points_f32)) as f32;
        let mut sum: Vec4 = [0.0; 4];
        for index in 0..count {
            let vector = load_vec(e, data.wrapping_add(0x10 * index as u32));
            sum = std::array::from_fn(|lane| vector[lane] + sum[lane]);
        }
        let inverse = (1.0 / f64::from(points_f32)) as f32;
        let centre = scaled(sum, inverse);
        store_vec(e, out + CENTRE, centre);
        let weight = f64::from(per_point);
        for index in 0..count {
            let slot = data.wrapping_add(0x10 * index as u32);
            let delta = |e: &Engine, lane: u32| {
                f64::from((load(e, slot + 4 * lane) - f64::from(centre[lane as usize])) as f32)
            };
            let (dx, dy, dz) = (delta(e, 0), delta(e, 1), delta(e, 2));
            let (dx2, dy2, dz2) = (dx * dx, dy * dy, dz * dz);
            accumulate(e, out + 0x20, (dy2 + dz2) * weight);
            accumulate(e, out + 0x34, (dz2 + dx2) * weight);
            accumulate(e, out + 0x48, (dy2 + dx2) * weight);
            let xy = (dy * dx) * weight;
            let xz = (dx * dz) * weight;
            let yz = (dz * dy) * weight;
            for (offset, product) in [
                (0x30, xy),
                (0x24, xy),
                (0x40, xz),
                (0x28, xz),
                (0x38, yz),
                (0x44, yz),
            ] {
                store(e, out + offset, load(e, out + offset) - product);
            }
        }
        release_array(e, a, 4);
    });
    0
}

/// What the code that owns a `hkArray` does when it is done with it: the memory router is
/// asked for the heap allocator (`+0x10` of the router), the size (`+4`) becomes 0 and, when
/// the capacity word (`+8`) has its top bit clear (the buffer is not inline), the allocator's
/// function at `+0x10` frees the buffer (`+0`) with `(capacity & 0x3fffffff) << element_shift`
/// bytes.
fn release_array(e: &mut Engine, array: u32, element_shift: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(array + 8);
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, 0);
    if (flags as i32) >= 0 {
        e.vcall(
            allocator,
            0x10,
            &args![data, (flags & 0x3fff_ffff) << element_shift],
        );
    }
}

/// True unless `a < b` holds (an unordered pair counts as not less).
fn not_less(a: f32, b: f32) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Less)
}

/// `00ca8920` (`hkprigidbody.cpp`, unnamed in the engine map): `this` is a node, the one word a
/// pointer to the three-row float matrix `00cedec0` just edited; it forwards both to the
/// function at `+0x1c` of the object embedded at `this + 0xE0` (purpose unconfirmed).
const SET_INERTIA: u32 = 0x00ca_8920;

// Translated from 00cedec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `list` is a `hkArray` of constraint-like entries (each holding
/// two node pointers at `+0x14` and `+0x18`); the entries with `node` on one side and a
/// non-null node on the other are removed from `list` (taking the last entry's place) into a
/// local array, then the function recurses (with the same `factor`) into the node on the
/// other side of each. The recursion results are summed (`sum`, a float) and the largest is
/// kept (`max`, starting at 0.0). A node whose byte at `+0xE8` is 4 or 5 answers 0.0. For
/// any other node the object embedded at `node + 0xE0` is asked (virtual function `+0x14`,
/// one pointer) for a three-row matrix, whose diagonal `d0`, `d1`, `d2` gives `a`, the largest
/// of the three (a later one only when strictly larger). With `sum` exactly 0.0 the answer is
/// `a`. Otherwise `c = a * factor`, `m = min(c, sum)`, `m2 = max(max, m)`, every diagonal entry
/// that is not below `m2` becomes `m2`, the matrix is stored back (`00ca8920`) and the answer
/// is `max(m2, min(a + sum, c))`. The meaning of the numbers is unconfirmed.
pub fn fn_00cedec0(e: &mut Engine, list: Ptr, node: Ptr, factor: f32) -> f32 {
    let (list, node) = (list.addr(), node.addr());
    let factor = x87_quiet(factor);
    e.with_stack(0x80, |e, frame| {
        let array = frame.addr();
        let (buffer, out) = (array + 0x10, array + 0x50);
        e.mem.set_u32(array, buffer);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0010);
        for index in (0..e.mem.i32(list + 4)).rev() {
            let entries = e.mem.u32(list);
            let entry = e.mem.u32(entries.wrapping_add(4 * index as u32));
            let first = e.mem.u32(entry + 0x14);
            let other = if first == node {
                e.mem.u32(entry + 0x18)
            } else if e.mem.u32(entry + 0x18) == node {
                first
            } else {
                continue;
            };
            if other == 0 {
                continue;
            }
            let remaining = e.mem.i32(list + 4) - 1;
            e.mem.set_i32(list + 4, remaining);
            if remaining != index {
                let last = e.mem.u32(entries.wrapping_add(4 * remaining as u32));
                e.mem.set_u32(entries.wrapping_add(4 * index as u32), last);
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            if e.mem.u32(array + 4) == capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
            }
            let size = e.mem.u32(array + 4);
            let data = e.mem.u32(array);
            e.mem.set_u32(data.wrapping_add(4 * size), entry);
            e.mem.set_u32(array + 4, size + 1);
        }
        let mut sum = 0.0f32;
        let mut largest = 0.0f32;
        for index in (0..e.mem.i32(array + 4)).rev() {
            let entry = e.mem.u32(e.mem.u32(array).wrapping_add(4 * index as u32));
            let first = e.mem.u32(entry + 0x14);
            let child = if first == node {
                e.mem.u32(entry + 0x18)
            } else {
                first
            };
            let result = fn_00cedec0(e, Ptr::new(list), Ptr::new(child), factor);
            sum = (f64::from(result) + f64::from(sum)) as f32;
            if largest < result {
                largest = result;
            }
        }
        if matches!(e.mem.u8(node + 0xe8), 4 | 5) {
            release_array(e, array, 2);
            return 0.0;
        }
        e.vcall(node + 0xe0, 0x14, &args![out]);
        let [d0, d1, d2] = [out, out + 0x14, out + 0x28].map(|slot| e.mem.f32(slot));
        let upper = if d2 < d1 { d1 } else { d2 };
        let a = if upper < d0 { d0 } else { upper };
        if sum == 0.0 {
            release_array(e, array, 2);
            return a;
        }
        let capped = (f64::from(a) * f64::from(factor)) as f32;
        let limited = if capped > sum { sum } else { capped };
        let floor = if largest < limited { limited } else { largest };
        for slot in [out, out + 0x14, out + 0x28] {
            if not_less(floor, e.mem.f32(slot)) {
                e.mem.set_f32(slot, floor);
            }
        }
        e.call(SET_INERTIA, &args![node, out]);
        let total = (f64::from(a) + f64::from(sum)) as f32;
        let bounded = if total > capped { capped } else { total };
        let answer = if floor < bounded { bounded } else { floor };
        release_array(e, array, 2);
        answer
    })
}

// Translated from 00cee1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the `count` pointers at `entries` into an array whose
/// buffer is taken from the memory router's linear allocator (its top pointer, `[router]`,
/// moves up by `(count * 4 + 0x7f) & ~0x7f` for the duration and is put back at the end),
/// runs `00cedec0` on it with `node` and `factor` (the result is discarded) and releases the
/// array (a buffer still equal to the saved top is left alone: its size is only cleared).
pub fn fn_00cee1c0(e: &mut Engine, entries: Ptr, count: i32, node: Ptr, factor: f32) {
    let entries = entries.addr();
    e.with_stack(0x10, |e, frame| {
        let array = frame.addr();
        e.mem.set_u32(array + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let top = e.mem.u32(router);
        let reserved = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, reserved.wrapping_add(top));
        e.mem.set_u32(array + 8, count as u32 | 0x8000_0000);
        e.mem.set_u32(array, top);
        for index in 0..count {
            let entry = e.mem.u32(entries.wrapping_add(4 * index as u32));
            let size = e.mem.u32(array + 4);
            e.mem
                .set_u32(e.mem.u32(array).wrapping_add(4 * size), entry);
            e.mem.set_u32(array + 4, size + 1);
        }
        fn_00cedec0(e, Ptr::new(array), node, x87_quiet(factor));
        if top == e.mem.u32(array) {
            e.mem.set_u32(array + 4, 0);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, top);
        release_array(e, array, 2);
    });
}

/// Floats the triangle function compares with and multiplies by: the limit `1e-5` (`010718c0`)
/// and the third `0.33333334` (`0106c2e0`), and the doubles 9.0 (`0102e438`) and 12.0
/// (`01035710`).
const FLAT_LIMIT: u32 = 0x0107_18c0;
const THIRD_F32: u32 = 0x0106_c2e0;
const NINE_D: u32 = 0x0102_e438;
const TWELVE_D: u32 = 0x0103_5710;
/// `hkGeometry::~hkGeometry` (Xbox PDB, `00cce530`, `this` is the geometry).
const GEOMETRY_DESTRUCT: u32 = 0x00cc_e530;
/// The offsets of the corner triples (into the six vertices) of the eight triangles of the
/// slab `computeTriangleSurfaceMassProperties` builds around a triangle.
const SLAB_TRIANGLES: [[u32; 3]; 8] = [
    [0, 2, 4],
    [1, 5, 3],
    [0, 3, 2],
    [0, 1, 3],
    [1, 0, 4],
    [1, 4, 5],
    [2, 5, 4],
    [2, 3, 5],
];

/// Appends one triangle `{a, b, c, -1}` (16 bytes) to the `hkArray` at `array`: the router's
/// allocator is asked to reserve (`00c908c0`, element size 0x10) when the capacity is below the
/// new size (the larger of the new size and twice the capacity).
fn push_triangle(e: &mut Engine, array: u32, corners: [u32; 3]) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.i32(array + 4);
    let flags = e.mem.u32(array + 8);
    let wanted = size + 1;
    let capacity = (flags & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_mul(2);
        let reserve = if wanted >= doubled { wanted } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, reserve as u32, 0x10u32],
        );
    }
    e.mem.set_i32(array + 4, wanted);
    let slot = e.mem.u32(array).wrapping_add((size as u32) << 4);
    for (lane, corner) in corners.into_iter().enumerate() {
        e.mem.set_u32(slot + 4 * lane as u32, corner);
    }
    e.mem.set_u32(slot + 12, u32::MAX);
}

// Translated from 00cee290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeTriangleSurfaceMassProperties` (Xbox PDB): the mass
/// properties of the triangle `a`, `b`, `c` as a slab of `thickness` and total `mass`. Answers
/// 1 (nothing written) when `mass` is not above zero or `thickness` is below zero. With the
/// cross product `n` of the edges (`c - b`, `a - b`) and its length `len`, the centre `g` is the
/// mean of the corners (`((a + b) + c)` times the float at `0106c2e0`). Three cases follow.
/// Flat (the float at `010718c0` above `thickness`): a thin plate; the second moments about the
/// origin are `(a_i * a_j + b_i * b_j + c_i * c_j + 9 * g_i * g_j) * mass / 12` (summed in
/// double precision, each result rounded to a float), made into a tensor and shifted
/// (`shiftInertiaToCom`) to `g`. Degenerate (that
/// float above `len`): a point mass at `g`, `(g_j^2 + g_k^2) * mass` on the diagonal and
/// `-(g_i * g_j) * mass` off it, not shifted. Otherwise a slab of the six corners moved by
/// `+-thickness / 2` along the unit normal (eight triangles, an `hkGeometry` built through the
/// router's allocator) goes through `computeGeometryVolumeMassProperties` and takes the
/// result's centre and rows. In all three the record gets `mass` (`+4`), the rows (`+0x20 ..`),
/// the centre (`+0x10`) and the volume `len * 0.5 * thickness`. Answers 0.
pub fn hkp_inertia_tensor_computer_compute_triangle_surface_mass_properties(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    mass: f32,
    thickness: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || thickness < 0.0 {
        return 1;
    }
    let (pa, pb, pc, out) = (a.addr(), b.addr(), c.addr(), out.addr());
    let [va, vb, vc] = [pa, pb, pc].map(|corner| load_vec(e, corner));
    let normal = cross(sub(vc, vb), sub(va, vb));
    let length = sum3(mul(normal, normal)).sqrt();
    let limit: f32 = e.global(FLAT_LIMIT);
    let third: f32 = e.global(THIRD_F32);
    let centre = scaled(add(add(va, vb), vc), third);
    let m = f64::from(x87_quiet(mass));
    e.with_stack(0x140, |e, frame| {
        let base = frame.addr();
        let (g, rows, record) = (base, base + 0x10, base + 0x40);
        let geometry = base + 0xa0;
        store_vec(e, g, centre);
        if limit > thickness {
            let nine = e.mem.f64(NINE_D);
            let f = m / e.mem.f64(TWELVE_D);
            let at = |corner: Vec4, lane: usize| f64::from(corner[lane]);
            let g64 = [0, 1, 2].map(|lane| at(centre, lane));
            let diagonal = |lane: usize| {
                let c9 = g64[lane] * nine;
                let sum = at(va, lane) * at(va, lane) + g64[lane] * c9;
                let sum = sum + at(vb, lane) * at(vb, lane);
                let sum = sum + at(vc, lane) * at(vc, lane);
                f64::from((sum * f) as f32)
            };
            let (ixx, iyy, izz) = (diagonal(0), diagonal(1), diagonal(2));
            let c9x = g64[0] * nine;
            let c9y = g64[1] * nine;
            let xy = (at(va, 0) * at(va, 1) + g64[1] * c9x) + at(vb, 0) * at(vb, 1);
            let xy = xy + at(vc, 1) * at(vc, 0);
            let xz = (at(va, 0) * at(va, 2) + c9x * g64[2]) + at(vb, 0) * at(vb, 2);
            let xz = xz + at(vc, 2) * at(vc, 0);
            let yz = (at(va, 2) * at(va, 1) + g64[2] * c9y) + at(vb, 2) * at(vb, 1);
            let yz = yz + at(vc, 2) * at(vc, 1);
            let xy = -f64::from((xy * f) as f32);
            let xz = -f64::from((xz * f) as f32);
            let yz = -f64::from((yz * f) as f32);
            let xx = izz + iyy;
            let yy = izz + ixx;
            let zz = ixx + iyy;
            write_symmetric(e, rows, [xx, yy, zz], [xy, xz, yz]);
            hkp_inertia_tensor_computer_shift_inertia_to_com(
                e,
                Ptr::new(g),
                x87_quiet(mass),
                Ptr::new(rows),
            );
            finish_surface(e, out, mass, thickness, length, g, rows);
        } else if limit > length {
            let g64 = [0, 1, 2].map(|lane| f64::from(centre[lane]));
            let [gx2, gy2, gz2] = g64.map(|value| value * value);
            let xx = (gy2 + gz2) * m;
            let yy = (gz2 + gx2) * m;
            let zz = (gy2 + gx2) * m;
            let negative = f64::from((-m) as f32);
            let xy = (g64[1] * g64[0]) * negative;
            let xz = (g64[0] * g64[2]) * negative;
            let yz = (g64[2] * g64[1]) * negative;
            let round = |value: f64| f64::from(value as f32);
            write_symmetric(
                e,
                rows,
                [round(xx), round(yy), round(zz)],
                [round(xy), round(xz), round(yz)],
            );
            finish_surface(e, out, mass, thickness, length, g, rows);
        } else {
            let direction = normalised(e, normal);
            let half = (f64::from(thickness) * e.mem.f64(HALF_D)) as f32;
            let lower = (-e.mem.f64(HALF_D) * f64::from(thickness)) as f32;
            let corners = [va, vb, vc];
            let scratch = base + 0xc0;
            let vertices = base + 0xd0;
            e.call(MEMORY_ROUTER, &args![]);
            for (index, corner) in corners.into_iter().enumerate() {
                for (step, shift) in [half, lower].into_iter().enumerate() {
                    let moved: Vec4 = std::array::from_fn(|lane| {
                        let offset = shift * direction[lane];
                        offset + corner[lane]
                    });
                    store_vec(
                        e,
                        vertices + 0x20 * index as u32 + 0x10 * step as u32,
                        moved,
                    );
                }
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.mem.set_u32(geometry + 8, 0x8000_0000);
            e.mem.set_u32(geometry + 0x14, 0x8000_0000);
            e.mem.set_i32(scratch, 6 << 4);
            let data = e.vcall(allocator, 0xc, &args![scratch]).u32();
            e.mem.set_u32(geometry, data);
            let granted = e.mem.i32(scratch);
            let capacity = (granted + ((granted >> 31) & 0xf)) >> 4;
            e.mem.set_i32(geometry + 8, capacity);
            e.mem.set_u32(geometry + 4, 6);
            for index in 0..6u32 {
                copy_vec(e, vertices + 0x10 * index, data + 0x10 * index);
            }
            for corners in SLAB_TRIANGLES {
                push_triangle(e, geometry + 0xc, corners);
            }
            hkp_inertia_tensor_computer_compute_geometry_volume_mass_properties(
                e,
                Ptr::new(geometry),
                x87_quiet(mass),
                Ptr::new(record),
            );
            e.call(GEOMETRY_DESTRUCT, &args![geometry]);
            copy_vec(e, record + CENTRE, g);
            for row in 0..3 {
                copy_vec(e, record + INERTIA + 0x10 * row, rows + 0x10 * row);
            }
            e.call(MEMORY_ROUTER, &args![]);
            finish_surface(e, out, mass, thickness, length, g, rows);
        }
    });
    0
}

/// The symmetric tensor rows `00cee290` builds: the diagonal `xx`, `yy`, `zz` and the pairs
/// `xy`, `xz`, `yz`, with the fourth lane of each row left at zero (the exe leaves it
/// unwritten).
fn write_symmetric(e: &mut Engine, rows: u32, diagonal: [f64; 3], pairs: [f64; 3]) {
    let [xx, yy, zz] = diagonal;
    let [xy, xz, yz] = pairs;
    for (row, values) in [[xx, xy, xz], [xy, yy, yz], [xz, yz, zz]]
        .into_iter()
        .enumerate()
    {
        store_vec(e, rows + 0x10 * row as u32, [0.0; 4]);
        for (lane, value) in values.into_iter().enumerate() {
            store(e, rows + 0x10 * row as u32 + 4 * lane as u32, value);
        }
    }
}

/// The end all three cases of `00cee290` share: `mass` (through the x87), the three rows, the
/// volume `(len * 0.5) * thickness` and the centre go to the record at `out`.
fn finish_surface(
    e: &mut Engine,
    out: u32,
    mass: f32,
    thickness: f32,
    length: f32,
    centre: u32,
    rows: u32,
) {
    e.mem.set_f32(out + MASS, x87_quiet(mass));
    for row in 0..3 {
        copy_vec(e, rows + 0x10 * row, out + INERTIA + 0x10 * row);
    }
    let half = e.mem.f64(HALF_D);
    store(
        e,
        out + VOLUME,
        (f64::from(length) * half) * f64::from(thickness),
    );
    copy_vec(e, centre, out + CENTRE);
}

/// `hkMath::acos` (Xbox PDB, `00c74a70`, cdecl, one float; the result in `ST0`),
/// `hkQuaternion::setAxisAngle` (Xbox PDB, `00cb2450`, `this` = quaternion, `(axis, angle)`),
/// `00cb2d90` (`hkrotation.cpp`, unnamed in the engine map: `this` = the three rows of a
/// matrix, the one word a quaternion) and `hkVector4::setRotatedDir` (`00c7f700`).
const ACOS: u32 = 0x00c7_4a70;
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const ROTATION_FROM_QUATERNION: u32 = 0x00cb_2d90;
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The doubles 4.1887903 (`010cbf40`, in the constants above), pi (`0101ff40`), 0.25
/// (`010290b0`), 0.375 (`010cbf68`), -0.375 (`010cbf60`) and 0.99999 (`010cbf70`).
const PI_D: u32 = 0x0101_ff40;
const QUARTER_D: u32 = 0x0102_90b0;
const THREE_EIGHTHS_D: u32 = 0x010c_bf68;
const MINUS_THREE_EIGHTHS_D: u32 = 0x010c_bf60;
const NEARLY_ONE_D: u32 = 0x010c_bf70;
/// The 16-byte vector at `01268390` the cylinder function compares its direction with (zero
/// in the file; the program sets it at run time).
const CYLINDER_UP: u32 = 0x0126_8390;
/// Size of an element `combineMassProperties` takes.
const ELEMENT: u32 = 0x90;

/// Reserves the next element of the `hkArray` at `array` (elements of 0x90 bytes): the router
/// is asked for its allocator and, when the size equals the capacity, `00c90940` grows the
/// array by that element size. Answers the address of the new element; the size has grown.
fn push_element(e: &mut Engine, array: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, ELEMENT]);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size + 1);
    e.mem.u32(array).wrapping_add(size.wrapping_mul(ELEMENT))
}

/// Writes one element of the list `combineMassProperties` takes: the volume and mass (through
/// the x87), the centre, the three rows read from `rows`, and the transform (three rows
/// `rotation` and the translation).
fn fill_element(
    e: &mut Engine,
    slot: u32,
    (volume, mass): (f32, f32),
    centre: Vec4,
    rows: u32,
    (rotation, translation): ([Vec4; 3], Vec4),
) {
    e.mem.set_f32(slot + VOLUME, x87_quiet(volume));
    e.mem.set_f32(slot + MASS, x87_quiet(mass));
    store_vec(e, slot + CENTRE, centre);
    for row in 0..3 {
        copy_vec(e, rows + 0x10 * row, slot + INERTIA + 0x10 * row);
        store_vec(e, slot + 0x50 + 0x10 * row, rotation[row as usize]);
    }
    store_vec(e, slot + 0x80, translation);
}

/// The identity rotation the cylinder and capsule functions use when the direction is
/// (nearly) the reference axis.
const IDENTITY_ROWS: [Vec4; 3] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
];

/// The rotation of the cylinder and capsule functions: `dot` is the sum of the first three
/// lanes of `up * direction`. Unless its absolute value (through the x87) reaches the double
/// at `010cbf70` (or is a NaN) the axis is the normalised cross product of `up` and `direction`,
/// `hkMath::acos(dot)` the angle, `setAxisAngle` builds a quaternion at `scratch` and `00cb2d90`
/// the three rows at `scratch + 0x20`, which are answered; otherwise the identity.
fn rotation_from(e: &mut Engine, scratch: u32, up: Vec4, direction: Vec4) -> [Vec4; 3] {
    let dot = sum3(mul(up, direction));
    let limit = e.mem.f64(NEARLY_ONE_D);
    if f64::from(x87_quiet(dot).abs()) < limit {
        let axis = normalised(e, cross(up, direction));
        store_vec(e, scratch + 0x10, axis);
        let angle = e.call(ACOS, &args![dot]).f64() as f32;
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![scratch, scratch + 0x10, angle],
        );
        e.call(ROTATION_FROM_QUATERNION, &args![scratch + 0x20, scratch]);
        [0, 1, 2].map(|row| load_vec(e, scratch + 0x20 + 0x10 * row))
    } else {
        IDENTITY_ROWS
    }
}

/// The inertia of a cylinder of radius `r` and length `length` about its axis: the rows `diag`
/// holds `(L^2 / 12 + r^2 / 4)` twice and `r^2 / 2` (each rounded to a float, the first as the
/// sum of `((L * (L / 2)) * 0.5) / 3.0` and `r^2 * 0.25`).
fn cylinder_diagonal(e: &mut Engine, r: f32, length: f32) -> (f64, f64) {
    let r2 = f64::from(r) * f64::from(r);
    let l = f64::from(length);
    let thick = ((l * (l * e.mem.f64(HALF_D))) * e.mem.f64(HALF_D)) / e.mem.f64(THREE_D);
    (
        f64::from((thick + r2 * e.mem.f64(QUARTER_D)) as f32),
        f64::from((r2 * e.mem.f64(HALF_D)) as f32),
    )
}

// Translated from 00ceeca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeCapsuleVolumeMassProperties` (Xbox PDB): a capsule of
/// the segment `a`-`b` and `radius`, total `mass`. Answers 1 (nothing written) when `mass` or
/// `radius` is not above zero. The length is `L = |b - a|`; for `L > 0` and a direction whose
/// dot product with the z axis stays below 0.99999 in absolute value the rotation is made
/// from the z axis to the direction (axis, `acos`, quaternion), otherwise it is the identity.
/// The volumes are `Vs = ((r * 4.18879) * r) * r` (the sphere) and `Vc = ((r * pi) * r) * L`
/// (the cylinder), their sum `T`, and the masses `Vs * mass / T` and `Vc * mass / T`. Three
/// elements go to `combineMassProperties` (`00ced090`, which fills `out`): the cylinder
/// (diagonal tensor scaled by its mass) at the midpoint and two hemispheres, each the sphere's
/// tensor halved, shifted (`shiftInertiaToCom`) to the hemisphere's centre `0.375 r` along
/// the axis and placed `L / 2` away along the rotated axis. Answers 0.
pub fn hkp_inertia_tensor_computer_compute_capsule_volume_mass_properties(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    radius: f32,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || radius <= 0.0 {
        return 1;
    }
    let (pa, pb, out) = (a.addr(), b.addr(), out.addr());
    let (va, vb) = (load_vec(e, pa), load_vec(e, pb));
    let direction = sub(vb, va);
    let length = sum3(mul(direction, direction)).sqrt();
    e.with_stack(0x300, |e, frame| {
        let base = frame.addr();
        let (array, scratch, rows, midpoint) = (base, base + 0x10, base + 0x30, base + 0x60);
        let (offset, record, temp, buffer) = (base + 0x70, base + 0x80, base + 0xd0, base + 0x110);
        let up: Vec4 = [0.0, 0.0, 1.0, 0.0];
        let rotation = if length > 0.0 {
            rotation_from(e, scratch, up, normalised(e, direction))
        } else {
            IDENTITY_ROWS
        };
        for (index, row) in rotation.into_iter().enumerate() {
            store_vec(e, rows + 0x10 * index as u32, row);
        }
        let half: f32 = e.global(HALF_F32);
        let sum = add(va, vb);
        let mid: Vec4 = std::array::from_fn(|lane| half * sum[lane]);
        store_vec(e, midpoint, mid);
        e.mem.set_u32(array, buffer);
        e.mem.set_u32(array + 8, 0x8000_0003);
        let (r, l, m) = (f64::from(radius), f64::from(length), f64::from(mass));
        let sphere_volume = (((r * e.mem.f64(SPHERE_VOLUME_FACTOR)) * r) * r) as f32;
        let cylinder_volume = (((r * e.mem.f64(PI_D)) * r) * l) as f32;
        let total = (f64::from(cylinder_volume) + f64::from(sphere_volume)) as f32;
        let sphere_mass = ((f64::from(sphere_volume) * m) / f64::from(total)) as f32;
        let cylinder_mass = ((f64::from(cylinder_volume) * m) / f64::from(total)) as f32;
        // The cylinder.
        let slot = push_element(e, array);
        let (ip, ip_z) = cylinder_diagonal(e, radius, length);
        for row in 0..3 {
            zero_vector(e, temp + 0x10 * row);
        }
        store(e, temp, ip);
        store(e, temp + 0x14, ip);
        store(e, temp + 0x28, ip_z);
        scale_rows(e, temp, cylinder_mass);
        fill_element(
            e,
            slot,
            (cylinder_volume, cylinder_mass),
            [0.0; 4],
            temp,
            (rotation, mid),
        );
        // The two hemispheres.
        let half_mass = (f64::from(sphere_mass) * e.mem.f64(HALF_D)) as f32;
        let half_volume = (f64::from(sphere_volume) * e.mem.f64(HALF_D)) as f32;
        let half_length = l * e.mem.f64(HALF_D);
        // The signed half length along the axis and the double giving the centre of mass.
        let hemispheres = [
            (half_length as f32, THREE_EIGHTHS_D),
            ((-half_length) as f32, MINUS_THREE_EIGHTHS_D),
        ];
        for (half, centre_factor) in hemispheres {
            store_vec(e, offset, [0.0, 0.0, half, 0.0]);
            e.call(SET_ROTATED_DIR, &args![offset, rows, offset]);
            let moved = load_vec(e, offset);
            let centre_z = (f64::from(radius) * e.mem.f64(centre_factor)) as f32;
            let translation: Vec4 = std::array::from_fn(|lane| mid[lane] + moved[lane]);
            for row in 0..3 {
                zero_vector(e, record + INERTIA + 0x10 * row);
            }
            hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(
                e,
                radius,
                sphere_mass,
                Ptr::new(record),
            );
            for row in 0..3 {
                copy_vec(e, record + INERTIA + 0x10 * row, temp + 0x10 * row);
            }
            scale_rows(e, temp, 0.5);
            let centre: Vec4 = [0.0, 0.0, centre_z, 0.0];
            store_vec(e, scratch, centre);
            hkp_inertia_tensor_computer_shift_inertia_to_com(
                e,
                Ptr::new(scratch),
                half_mass,
                Ptr::new(temp),
            );
            let slot = push_element(e, array);
            fill_element(
                e,
                slot,
                (half_volume, half_mass),
                centre,
                temp,
                (rotation, translation),
            );
        }
        hkp_inertia_tensor_computer_combine_mass_properties(e, Ptr::new(array), Ptr::new(out));
        release_array(e, array, 0);
    });
    0
}

// Translated from 00cef6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeCylinderVolumeMassProperties` (Xbox PDB): a cylinder of
/// the segment `a`-`b` and `radius`, total `mass`. Answers 1 (nothing written) when `mass` or
/// `radius` is not above zero or the length `L = |b - a|` is not above zero. The rotation is
/// made from the vector at `01268390` to the direction as in `00ceeca0`; the volume is
/// `((r * pi) * r) * L`, the single element (centre zero, the diagonal of `(L^2 / 12 + r^2 / 4,
/// L^2 / 12 + r^2 / 4, r^2 / 2)` scaled by `mass`, the rotation and the midpoint) goes to
/// `combineMassProperties`, which fills `out`. Answers 0.
pub fn hkp_inertia_tensor_computer_compute_cylinder_volume_mass_properties(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    radius: f32,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || radius <= 0.0 {
        return 1;
    }
    let (pa, pb, out) = (a.addr(), b.addr(), out.addr());
    let (va, vb) = (load_vec(e, pa), load_vec(e, pb));
    let direction = sub(vb, va);
    let length = sum3(mul(direction, direction)).sqrt();
    if length <= 0.0 || length.is_nan() {
        return 1;
    }
    e.with_stack(0x1c0, |e, frame| {
        let base = frame.addr();
        let (array, scratch, rows, buffer) = (base, base + 0x10, base + 0x30, base + 0x60);
        let up = load_vec(e, CYLINDER_UP);
        let rotation = rotation_from(e, scratch, up, normalised(e, direction));
        let half: f32 = e.global(HALF_F32);
        let sum = add(va, vb);
        let mid: Vec4 = std::array::from_fn(|lane| half * sum[lane]);
        let (r, l) = (f64::from(radius), f64::from(length));
        let volume = (((r * e.mem.f64(PI_D)) * r) * l) as f32;
        let mass_q = x87_quiet(mass);
        e.mem.set_u32(array, buffer);
        e.mem.set_u32(array + 8, 0x8000_0001);
        let slot = push_element(e, array);
        for row in 0..3 {
            zero_vector(e, rows + 0x10 * row);
        }
        let (ip, ip_z) = cylinder_diagonal(e, radius, length);
        store(e, rows, ip);
        store(e, rows + 0x14, ip);
        store(e, rows + 0x28, ip_z);
        scale_rows(e, rows, mass_q);
        fill_element(e, slot, (volume, mass_q), [0.0; 4], rows, (rotation, mid));
        hkp_inertia_tensor_computer_combine_mass_properties(e, Ptr::new(array), Ptr::new(out));
        release_array(e, array, 0);
    });
    0
}

// Translated from 00cefb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the surface mass properties of a triangle mesh. `mesh` holds the
/// vertex pointer (`+0`, 16-byte vectors), the face pointer (`+0xC`, faces of 16 bytes whose first
/// three words are vertex indices) and the face count (`+0x10`). Answers 1 (nothing written)
/// when `mass` or `thickness` is not above zero (a NaN passes). The record at `out` gets a
/// zero centre and zero rows first. Each face has a weight: 1.0 when `area_weighted` is zero,
/// otherwise half the length of the cross product of two edges (the float at `010cbf78`, 5e-6,
/// when that length is below the float at `010718c0`). The weights are stored in a float array
/// reserved through the memory router and summed. For every face `computeTriangleSurfaceMassProperties`
/// runs with the mass `weight * mass / sum`; its centre, scaled by that mass, accumulates into
/// the centre of `out`, and its rows, shifted (`00cebf30`) by that centre, are added
/// (`00cd5140`) to the rows of `out`. At the end `out` gets `mass`, its centre divided by `mass`
/// and its rows shifted (`shiftInertiaToCom`) by that centre. The volume is not written.
/// Answers 0.
pub fn fn_00cefb90(
    e: &mut Engine,
    mesh: Ptr,
    thickness: f32,
    area_weighted: u8,
    mass: f32,
    out: Ptr,
) -> u32 {
    if mass <= 0.0 || thickness <= 0.0 {
        return 1;
    }
    let (mesh, out) = (mesh.addr(), out.addr());
    for offset in [CENTRE, INERTIA, INERTIA + 0x10, INERTIA + 0x20] {
        zero_vector(e, out + offset);
    }
    let count = e.mem.i32(mesh + 0x10);
    e.with_stack(0xd0, |e, frame| {
        let base = frame.addr();
        let (areas, corners, record) = (base, base + 0x10, base + 0x40);
        let (rows, offset) = (base + 0x90, base + 0xc0);
        e.mem.set_u32(areas + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(areas + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_mul(2);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, areas, wanted as u32, 4u32]);
        }
        e.mem.set_i32(areas + 4, count);
        let vertices = e.mem.u32(mesh);
        let faces = e.mem.u32(mesh + 0xc);
        let vertex = |e: &Engine, face: u32, word: u32| {
            let index = e.mem.u32(faces.wrapping_add(0x10 * face) + 4 * word);
            load_vec(e, vertices.wrapping_add(index << 4))
        };
        let weights = e.mem.u32(areas);
        let mut total = 0.0f32;
        for face in 0..count as u32 {
            let pivot = vertex(e, face, 1);
            let first_edge = sub(vertex(e, face, 2), pivot);
            let second_edge = sub(vertex(e, face, 0), pivot);
            let normal = cross(first_edge, second_edge);
            let length = sum3(mul(normal, normal)).sqrt();
            let limit: f32 = e.global(FLAT_LIMIT);
            let weight = if area_weighted == 0 {
                1.0
            } else if length < limit {
                e.global::<f32>(TINY_AREA)
            } else {
                (f64::from(length) * e.mem.f64(HALF_D)) as f32
            };
            e.mem.set_f32(weights.wrapping_add(4 * face), weight);
            total = (f64::from(weight) + f64::from(total)) as f32;
        }
        let mass_q = x87_quiet(mass);
        for face in 0..count as u32 {
            for word in 0..3 {
                let corner = vertex(e, face, word);
                store_vec(e, corners + 0x10 * word, corner);
            }
            let weight = e.mem.f32(weights.wrapping_add(4 * face));
            let part = ((f64::from(weight) * f64::from(mass_q)) / f64::from(total)) as f32;
            for step in 0..5 {
                zero_vector(e, record + 0x10 * step);
            }
            hkp_inertia_tensor_computer_compute_triangle_surface_mass_properties(
                e,
                Ptr::new(corners),
                Ptr::new(corners + 0x10),
                Ptr::new(corners + 0x20),
                part,
                thickness,
                Ptr::new(record),
            );
            for row in 0..3 {
                copy_vec(e, record + INERTIA + 0x10 * row, rows + 0x10 * row);
            }
            let centre = load_vec(e, record + CENTRE);
            let accumulated = load_vec(e, out + CENTRE);
            store_vec(
                e,
                out + CENTRE,
                std::array::from_fn(|lane| part * centre[lane] + accumulated[lane]),
            );
            copy_vec(e, record + CENTRE, offset);
            fn_00cebf30(e, Ptr::new(offset), part, Ptr::new(rows));
            e.call(MATRIX_ADD, &args![out + INERTIA, rows]);
        }
        e.mem.set_f32(out + MASS, mass_q);
        let inverse = (1.0 / f64::from(mass_q)) as f32;
        let centre = load_vec(e, out + CENTRE);
        store_vec(e, out + CENTRE, scaled(centre, inverse));
        hkp_inertia_tensor_computer_shift_inertia_to_com(
            e,
            Ptr::new(out + CENTRE),
            mass_q,
            Ptr::new(out + INERTIA),
        );
        release_array(e, areas, 2);
    });
    0
}

/// The identity transform of the exe's elements: three rows and a zero translation.
const IDENTITY_TRANSFORM: ([Vec4; 3], Vec4) = (IDENTITY_ROWS, [0.0; 4]);

// Translated from 00cf0820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies a mass-properties record (`this` is the destination,
/// `source` the one word): the volume (`+0`) and mass (`+4`) through the x87, then the centre
/// and the three rows (16 bytes each, `+0x10 ..= +0x40`).
pub fn fn_00cf0820(e: &mut Engine, this: Ptr, source: Ptr) {
    let (this, source) = (this.addr(), source.addr());
    e.mem
        .set_f32(this + VOLUME, x87_quiet(e.mem.f32(source + VOLUME)));
    e.mem
        .set_f32(this + MASS, x87_quiet(e.mem.f32(source + MASS)));
    for offset in [CENTRE, INERTIA, INERTIA + 0x10, INERTIA + 0x20] {
        copy_vec(e, source + offset, this + offset);
    }
}

/// What the end of the leaf cases of `00ceff00` does with the record the leaf filled: a volume
/// of exactly zero (a NaN is not) ends it; otherwise the record is scaled by its own volume
/// (`00cebfd0`) and, with the record already in `out` (the identity transform) and this one (the
/// three rows and the translation at `transform`), goes to `combineMassProperties`, which
/// fills `out`.
fn merge_leaf(e: &mut Engine, leaf: u32, transform: u32, out: u32) {
    let volume = e.mem.f32(leaf + VOLUME);
    if volume == 0.0 {
        return;
    }
    fn_00cebfd0(e, Ptr::new(leaf), x87_quiet(volume));
    e.with_stack(0x130, |e, frame| {
        let array = frame.addr();
        e.mem.set_u32(array, array + 0x10);
        e.mem.set_u32(array + 8, 0x8000_0002);
        let slot = push_element(e, array);
        fn_00cf0820(e, Ptr::new(slot), Ptr::new(out));
        for row in 0..3 {
            store_vec(
                e,
                slot + 0x50 + 0x10 * row,
                IDENTITY_TRANSFORM.0[row as usize],
            );
        }
        store_vec(e, slot + 0x80, IDENTITY_TRANSFORM.1);
        let slot = push_element(e, array);
        fn_00cf0820(e, Ptr::new(slot), Ptr::new(leaf));
        for row in 0..4 {
            copy_vec(e, transform + 0x10 * row, slot + 0x50 + 0x10 * row);
        }
        hkp_inertia_tensor_computer_combine_mass_properties(e, Ptr::new(array), Ptr::new(out));
        release_array(e, array, 0);
    });
}

// Translated from 00ceff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeRecursiveShapeVolumeMassProperties` (Xbox PDB name as the
/// engine map lists it, without the class prefix): adds the mass properties of `shape`,
/// placed by the transform at `transform` (three rows and a translation), to the record at
/// `out`. `param3` (the third stack word, a float; its meaning is unconfirmed) is only passed
/// on to `00c98210` for triangles. The shape type is the word at `+0xC`; types 0 and above
/// 28 do nothing. The leaf types make unit-mass properties of their own (each in a zeroed record
/// that `merge_leaf` combines into `out`): 1 sphere (radius at `+0x10`), 2 cylinder (the end
/// points at `+0x20` and `+0x30` moved outwards by the radius), 3 triangle (corners at
/// `+0x20`, `+0x30`, `+0x40`; thickness from `00c98210`), 4 box (half extents at `+0x20` plus
/// the radius), 5 capsule, 6 convex vertices (`00d59200` gathers them). Container types (7 to
/// 10, 14, 22 and 23) are iterated through virtual functions `+0x10`, `+0x8`, `+0x14`, `+0xC`
/// of the container the shape's virtual function `+0x10` answers; type 11 (child at `+0x18`,
/// translation at `+0x20`), 12 (child at `+0x18`, transform at `+0x20`), 15 (child at `+0x14`,
/// transform at `+0x30`), 21 (an array of spheres from `+0x20` on, `+0x10` of them) and 27
/// (child at `+0x18`) recurse with the composed transform.
pub fn compute_recursive_shape_volume_mass_properties(
    e: &mut Engine,
    shape: Ptr,
    transform: Ptr,
    param3: f32,
    out: Ptr,
) {
    let (shape, transform, out) = (shape.addr(), transform.addr(), out.addr());
    let kind = e.mem.u32(shape + 0xc);
    if kind == 0 || kind > 28 {
        return;
    }
    // The compiler's switch on the shape type: the case of each type 1 to 28; the cases 12 and 13
    // do nothing.
    let case = match kind {
        1 => 0,
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 4,
        6 => 5,
        7..=10 | 14 | 22..=23 => 6,
        11 => 7,
        12 => 8,
        13 | 16..=20 | 24..=26 => 13,
        15 => 9,
        21 => 10,
        27 => 11,
        28 => 12,
        _ => unreachable!(),
    };
    e.with_stack(0x1c0, |e, frame| {
        let base = frame.addr();
        let (leaf, first, second, third) = (base, base + 0x50, base + 0x60, base + 0x70);
        let (array, source, sphere) = (base + 0x90, base + 0xa0, base + 0xb0);
        let (buffer, local, result, copy) = (base + 0xd0, base + 0x100, base + 0x140, base + 0x180);
        let rho = e.mem.f32(shape + 0x10);
        match case {
            0 => {
                hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(
                    e,
                    rho,
                    1.0,
                    Ptr::new(leaf),
                );
            }
            1 => {
                let radius = x87_quiet(rho);
                let (a, b) = (load_vec(e, shape + 0x20), load_vec(e, shape + 0x30));
                let unit = normalised(e, sub(b, a));
                let along = scaled(unit, radius);
                store_vec(e, first, sub(a, along));
                store_vec(e, second, add(along, b));
                let core = e.call(CYLINDER_RADIUS, &args![shape]).f64();
                let grown = (core + f64::from(radius)) as f32;
                hkp_inertia_tensor_computer_compute_cylinder_volume_mass_properties(
                    e,
                    Ptr::new(first),
                    Ptr::new(second),
                    grown,
                    1.0,
                    Ptr::new(leaf),
                );
            }
            2 => {
                for (slot, offset) in [(first, 0x20), (second, 0x30), (third, 0x40)] {
                    copy_vec(e, shape + offset, slot);
                }
                let thickness = e
                    .call(
                        TRIANGLE_THICKNESS,
                        &args![x87_quiet(rho), x87_quiet(param3)],
                    )
                    .f64() as f32;
                hkp_inertia_tensor_computer_compute_triangle_surface_mass_properties(
                    e,
                    Ptr::new(first),
                    Ptr::new(second),
                    Ptr::new(third),
                    1.0,
                    thickness,
                    Ptr::new(leaf),
                );
            }
            3 => {
                let half = load_vec(e, shape + 0x20);
                store_vec(e, first, std::array::from_fn(|lane| rho + half[lane]));
                hkp_inertia_tensor_computer_compute_box_volume_mass_properties(
                    e,
                    Ptr::new(first),
                    1.0,
                    Ptr::new(leaf),
                );
            }
            4 => {
                hkp_inertia_tensor_computer_compute_capsule_volume_mass_properties(
                    e,
                    Ptr::new(shape + 0x20),
                    Ptr::new(shape + 0x30),
                    rho,
                    1.0,
                    Ptr::new(leaf),
                );
            }
            5 => {
                e.mem.set_u32(array + 8, 0x8000_0000);
                e.call(GET_ORIGINAL_VERTICES, &args![shape, array]);
                e.mem.set_u32(source, e.mem.u32(array));
                e.mem.set_u32(source + 4, e.mem.u32(array + 4));
                e.mem.set_u32(source + 8, 0x10);
                hkp_inertia_tensor_computer_compute_convex_hull_mass_properties(
                    e,
                    Ptr::new(source),
                    rho,
                    Ptr::new(leaf),
                );
                release_array(e, array, 4);
            }
            6 => {
                let container = e.vcall(shape, 0x10, &args![]).u32();
                let mut key = e.vcall(container, 8, &args![]).u32();
                while key != u32::MAX {
                    let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
                    if child != 0 {
                        compute_recursive_shape_volume_mass_properties(
                            e,
                            Ptr::new(child),
                            Ptr::new(transform),
                            param3,
                            Ptr::new(out),
                        );
                    }
                    key = e.vcall(container, 0xc, &args![key]).u32();
                }
                return;
            }
            7 => {
                for (row, vector) in IDENTITY_ROWS.into_iter().enumerate() {
                    store_vec(e, local + 0x10 * row as u32, vector);
                }
                copy_vec(e, shape + 0x20, local + 0x30);
                e.call(TRANSFORM_SET_MUL, &args![result, transform, local]);
                let child = e.mem.u32(shape + 0x18);
                compute_recursive_shape_volume_mass_properties(
                    e,
                    Ptr::new(child),
                    Ptr::new(result),
                    param3,
                    Ptr::new(out),
                );
                return;
            }
            8 | 9 => {
                let (placement, child) = if case == 8 {
                    (0x20, 0x18)
                } else {
                    (0x30, 0x14)
                };
                e.call(
                    TRANSFORM_SET_MUL,
                    &args![result, transform, shape + placement],
                );
                let child = e.mem.u32(shape + child);
                compute_recursive_shape_volume_mass_properties(
                    e,
                    Ptr::new(child),
                    Ptr::new(result),
                    param3,
                    Ptr::new(out),
                );
                return;
            }
            10 => {
                for index in 0..e.mem.i32(shape + 0x10) {
                    let entry = shape + 0x20 + 0x10 * index as u32;
                    for row in 0..4 {
                        copy_vec(e, transform + 0x10 * row, copy + 0x10 * row);
                    }
                    e.call(SET_TRANSFORMED_POS, &args![copy + 0x30, copy, entry]);
                    let radius = e.mem.f32(entry + 0xc);
                    e.call(SPHERE_CONSTRUCT, &args![sphere, radius]);
                    compute_recursive_shape_volume_mass_properties(
                        e,
                        Ptr::new(sphere),
                        Ptr::new(copy),
                        param3,
                        Ptr::new(out),
                    );
                }
                return;
            }
            11 => {
                let child = e.mem.u32(shape + 0x18);
                compute_recursive_shape_volume_mass_properties(
                    e,
                    Ptr::new(child),
                    Ptr::new(transform),
                    param3,
                    Ptr::new(out),
                );
                return;
            }
            _ => return,
        }
        merge_leaf(e, leaf, transform, out);
    });
}

// Translated from 00cf0690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::computeShapeVolumeMassProperties` (Xbox PDB): the mass
/// properties of `shape` for the total `mass`, written to `out` (volume at `+0`, mass at
/// `+4`, centre at `+0x10`, rows from `+0x20`). The recursive function runs with the identity
/// transform and 0.0 as its third word; when the volume it gathers is exactly zero (a NaN is
/// not) it runs again from scratch with the float at `01013ea4` instead, and when the
/// volume is still zero nothing is written to `out`. Otherwise the mass and rows are scaled
/// by `mass / volume` (`00cebfd0`) and copied to `out`.
pub fn hkp_inertia_tensor_computer_compute_shape_volume_mass_properties(
    e: &mut Engine,
    shape: Ptr,
    mass: f32,
    out: Ptr,
) {
    let out = out.addr();
    e.with_stack(0xa0, |e, frame| {
        let record = frame.addr();
        let transform = record + 0x50;
        for (row, vector) in IDENTITY_ROWS.into_iter().enumerate() {
            store_vec(e, transform + 0x10 * row as u32, vector);
        }
        compute_recursive_shape_volume_mass_properties(
            e,
            shape,
            Ptr::new(transform),
            0.0,
            Ptr::new(record),
        );
        if e.mem.f32(record + VOLUME) == 0.0 {
            let retry: f32 = e.global(RETRY_PARAMETER);
            compute_recursive_shape_volume_mass_properties(
                e,
                shape,
                Ptr::new(transform),
                retry,
                Ptr::new(record),
            );
            if e.mem.f32(record + VOLUME) == 0.0 {
                return;
            }
        }
        let scale = (f64::from(mass) / f64::from(e.mem.f32(record + VOLUME))) as f32;
        fn_00cebfd0(e, Ptr::new(record), scale);
        fn_00cf0820(e, Ptr::new(out), Ptr::new(record));
    });
}

// Translated from 00cf07a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpInertiaTensorComputer::setShapeVolumeMassProperties` (Xbox PDB): runs
/// `computeShapeVolumeMassProperties` for `shape` and `mass` into a zeroed record and copies
/// the result into the construction info at `cinfo`: the mass (through the x87) to `+0x90`, the
/// centre to `+0x80` and the three rows to `+0x50`, `+0x60` and `+0x70`. The volume is not
/// copied.
pub fn hkp_inertia_tensor_computer_set_shape_volume_mass_properties(
    e: &mut Engine,
    shape: Ptr,
    mass: f32,
    cinfo: Ptr,
) {
    let cinfo = cinfo.addr();
    e.with_stack(0x50, |e, frame| {
        let record = frame.addr();
        hkp_inertia_tensor_computer_compute_shape_volume_mass_properties(
            e,
            shape,
            x87_quiet(mass),
            Ptr::new(record),
        );
        e.mem
            .set_f32(cinfo + 0x90, x87_quiet(e.mem.f32(record + MASS)));
        copy_vec(e, record + CENTRE, cinfo + 0x80);
        for row in 0..3 {
            copy_vec(e, record + INERTIA + 0x10 * row, cinfo + 0x50 + 0x10 * row);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ce_ba60,
            hkp_inertia_tensor_computer_compute_sphere_volume_mass_properties(f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_baf0,
            hkp_inertia_tensor_computer_compute_box_volume_mass_properties(Ptr, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_bbb0,
            hkp_inertia_tensor_computer_compute_convex_hull_mass_properties(Ptr, f32, Ptr) -> u32
        ),
        entry!(0x00ce_bdb0, fn_00cebdb0(Ptr, Ptr)),
        entry!(0x00ce_bdf0, fn_00cebdf0(Ptr, f32, Ptr)),
        entry!(
            0x00ce_be80,
            hkp_inertia_tensor_computer_shift_inertia_to_com(Ptr, f32, Ptr)
        ),
        entry!(0x00ce_bf30, fn_00cebf30(Ptr, f32, Ptr)),
        entry!(0x00ce_bfd0, fn_00cebfd0(Ptr, f32)),
        entry!(
            0x00ce_c000,
            internal_inertia_tensor_computer_comp_projection_integrals(Ptr, Ptr)
        ),
        entry!(
            0x00ce_c3d0,
            internal_inertia_tensor_computer_comp_face_integrals(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00ce_c6b0,
            internal_inertia_tensor_computer_compute_inertial_tensor_internal(
                Ptr, f32, f32, Ptr, Ptr
            )
        ),
        entry!(
            0x00ce_c7f0,
            internal_inertia_tensor_computer_comp_geometry_aabb(Ptr, i32, Ptr, Ptr)
        ),
        entry!(0x00ce_c880, fn_00cec880(Ptr, Ptr)),
        entry!(0x00ce_cc90, fn_00cecc90(Ptr, f32)),
        entry!(
            0x00ce_ccf0,
            hkp_inertia_tensor_computer_compute_sphere_surface_mass_properties(f32, f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_ce50,
            hkp_inertia_tensor_computer_compute_box_surface_mass_properties(Ptr, f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_d090,
            hkp_inertia_tensor_computer_combine_mass_properties(Ptr, Ptr) -> u32
        ),
        entry!(0x00ce_d310, fn_00ced310(Ptr)),
        entry!(
            0x00ce_d390,
            internal_inertia_tensor_computer_comp_volume_integrals(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00ce_d6b0,
            hkp_inertia_tensor_computer_compute_geometry_volume_mass_properties(Ptr, f32, Ptr)
        ),
        entry!(
            0x00ce_d8e0,
            hkp_inertia_tensor_computer_compute_geometry_volume_mass_properties_checked(Ptr, f32, Ptr) -> u32
        ),
        entry!(0x00ce_dae0, fn_00cedae0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00ce_dcb0, fn_00cedcb0(Ptr, i32, i32, f32, Ptr) -> u32),
        entry!(0x00ce_dec0, fn_00cedec0(Ptr, Ptr, f32) -> f32),
        entry!(0x00ce_e1c0, fn_00cee1c0(Ptr, i32, Ptr, f32)),
        entry!(
            0x00ce_e290,
            hkp_inertia_tensor_computer_compute_triangle_surface_mass_properties(Ptr, Ptr, Ptr, f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_eca0,
            hkp_inertia_tensor_computer_compute_capsule_volume_mass_properties(Ptr, Ptr, f32, f32, Ptr) -> u32
        ),
        entry!(
            0x00ce_f6d0,
            hkp_inertia_tensor_computer_compute_cylinder_volume_mass_properties(Ptr, Ptr, f32, f32, Ptr) -> u32
        ),
        entry!(0x00ce_fb90, fn_00cefb90(Ptr, f32, u8, f32, Ptr) -> u32),
        entry!(
            0x00ce_ff00,
            compute_recursive_shape_volume_mass_properties(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00cf_0690,
            hkp_inertia_tensor_computer_compute_shape_volume_mass_properties(Ptr, f32, Ptr)
        ),
        entry!(
            0x00cf_07a0,
            hkp_inertia_tensor_computer_set_shape_volume_mass_properties(Ptr, f32, Ptr)
        ),
        entry!(0x00cf_0820, fn_00cf0820(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use crate::units::havok_sdk::hkpsphereshape::{THREE_F32, ZERO_F32};
    use std::rc::Rc;

    /// Fake addresses of the doubles the tests hang objects on.
    const ALLOCATOR: u32 = 0x0900_0000;
    const NODE: u32 = 0x0910_0000;
    const SHAPE: u32 = 0x0920_0000;
    const CONTAINER: u32 = 0x0930_0000;

    /// The float, vector and double constants of the exe's data that the unit reads.
    fn exe_data(e: &mut Engine) {
        for (address, bits) in [
            (ZERO_F32, 0),
            (THREE_F32, 3.0f32.to_bits()),
            (HALF_F32, 0.5f32.to_bits()),
            (MINUS_ONE_F32, (-1.0f32).to_bits()),
            (LARGE_FLOAT, 0x7f7f_ffee),
            (JACOBI_THRESHOLD, 1.4210855e-14f32.to_bits()),
            (THIRD_F32, 0x3eaa_aaab),
            (FLAT_LIMIT, 0x3727_c5ac),
            (TINY_AREA, 0x36a7_c5ac),
            (RETRY_PARAMETER, 0x3c23_d70a),
            (MARGIN_SEED, 0.05f32.to_bits()),
        ] {
            e.mem.set_u32(address, bits);
        }
        for (address, words) in [
            (SIGN_MASK, [0x8000_0000; 4]),
            (ABS_MASK, [0x7fff_ffff; 4]),
            (AABB_ONES, [1.0f32.to_bits(); 4]),
            (CYLINDER_UP, [0, 0, 1.0f32.to_bits(), 0]),
        ] {
            for (lane, word) in words.into_iter().enumerate() {
                e.mem.set_u32(address + 4 * lane as u32, word);
            }
        }
        for (address, value) in [
            (SPHERE_MOMENT_FACTOR, 0.4),
            (SPHERE_VOLUME_FACTOR, f64::from(4.1887903f32)),
            (THIRD, f64::from(0.33333334f32)),
            (EIGHT, 8.0),
            (ONE_D, 1.0),
            (TWO_D, 2.0),
            (THREE_D, 3.0),
            (FOUR_D, 4.0),
            (HALF_D, 0.5),
            (FIVE_D, 5.0),
            (FOUR_PI_D, f64::from(12.566371f32)),
            (MINIMUM_VOLUME_D, 1.1920928955078125e-5),
            (NINE_D, 9.0),
            (TWELVE_D, 12.0),
            (TENTH_D, 0.1),
            (PI_D, f64::from(std::f32::consts::PI)),
            (QUARTER_D, 0.25),
            (THREE_EIGHTHS_D, 0.375),
            (MINUS_THREE_EIGHTHS_D, -0.375),
            (NEARLY_ONE_D, f64::from(0.99999f32)),
        ] {
            e.mem.set_f64(address, value);
        }
        let divisors = [6.0, 12.0, 20.0, -6.0, -12.0, -20.0, 24.0, 60.0, -60.0];
        for ((_, address), value) in PROJECTION_DIVISORS.into_iter().zip(divisors) {
            e.mem.set_f64(address, value);
        }
    }

    /// An engine with the exe's data range mapped and the constants set.
    fn setup() -> Engine {
        let mut e = engine();
        exe_data(&mut e);
        e
    }

    fn put_vec(e: &mut Engine, v: Vec4) -> u32 {
        let at = e.mem.alloc(0x10);
        store_vec(e, at, v);
        at
    }

    /// An array of 16-byte vectors.
    fn put_vecs(e: &mut Engine, vectors: &[Vec4]) -> u32 {
        let at = e.mem.alloc(0x10 * vectors.len() as u32);
        for (index, v) in vectors.iter().enumerate() {
            store_vec(e, at + 0x10 * index as u32, *v);
        }
        at
    }

    fn rows_at(e: &Engine, at: u32) -> [Vec4; 3] {
        [0, 1, 2].map(|row| load_vec(e, at + 0x10 * row))
    }

    /// A tensor of three rows from the diagonal and the three off-diagonal pairs (xy, yz, zx).
    fn tensor(diagonal: [f32; 3], pairs: [f32; 3]) -> [Vec4; 3] {
        let [xx, yy, zz] = diagonal;
        let [xy, yz, zx] = pairs;
        [[xx, xy, zx, 0.0], [xy, yy, yz, 0.0], [zx, yz, zz, 0.0]]
    }

    fn put_rows(e: &mut Engine, rows: [Vec4; 3]) -> u32 {
        put_vecs(e, &rows)
    }

    /// `actual` against `expected` to a relative 1e-4 (an absolute 1e-4 below 1.0): the
    /// square roots go through the host's reciprocal estimate.
    fn near(actual: f32, expected: f64) {
        let allowed = 1e-4 * expected.abs().max(1.0);
        assert!(
            (f64::from(actual) - expected).abs() <= allowed,
            "{actual} against {expected}"
        );
    }

    fn near_rows(actual: [Vec4; 3], expected: [[f64; 3]; 3]) {
        for row in 0..3 {
            for lane in 0..3 {
                near(actual[row][lane], expected[row][lane]);
            }
        }
    }

    /// A mass-properties record: volume, mass, centre and the three tensor rows.
    fn put_record(e: &mut Engine, volume: f32, mass: f32, centre: Vec4, rows: [Vec4; 3]) -> u32 {
        let at = e.mem.alloc(0x50);
        e.mem.set_f32(at + VOLUME, volume);
        e.mem.set_f32(at + MASS, mass);
        store_vec(e, at + CENTRE, centre);
        for (index, row) in rows.into_iter().enumerate() {
            store_vec(e, at + INERTIA + 0x10 * index as u32, row);
        }
        at
    }

    /// `actual` and `expected` must agree on the volume, mass, centre and diagonal.
    fn near_record(
        e: &Engine,
        at: u32,
        (volume, mass): (f64, f64),
        centre: [f64; 3],
        diagonal: [f64; 3],
    ) {
        near(e.mem.f32(at + VOLUME), volume);
        near(e.mem.f32(at + MASS), mass);
        let found = load_vec(e, at + CENTRE);
        for lane in 0..3 {
            near(found[lane], centre[lane]);
        }
        let rows = rows_at(e, at + INERTIA);
        for axis in 0..3 {
            near(rows[axis][axis], diagonal[axis]);
        }
    }

    /// `00cd4c70` (every lane of the three rows times the first lane of the vector),
    /// `00cd5180` (rows less rows), `00cd5140` (rows plus rows) and `00cd5880` (the change of
    /// basis: logged, and the tests only use the identity rotation).
    fn matrix_doubles(e: &mut Engine, log: &Log) {
        e.register_double(MATRIX_SCALE, |e, a| {
            let factor = e.mem.f32(a[1]);
            for lane in 0..12 {
                let at = a[0] + 0x10 * (lane / 4) + 4 * (lane % 4);
                e.mem.set_f32(at, e.mem.f32(at) * factor);
            }
            Ret::default()
        });
        for (address, combine) in [
            (MATRIX_SUB, (|a, b| a - b) as fn(f32, f32) -> f32),
            (MATRIX_ADD, |a, b| a + b),
        ] {
            e.register_double(address, move |e, a| {
                for lane in 0..12 {
                    let offset = 0x10 * (lane / 4) + 4 * (lane % 4);
                    let value = combine(e.mem.f32(a[0] + offset), e.mem.f32(a[1] + offset));
                    e.mem.set_f32(a[0] + offset, value);
                }
                Ret::default()
            });
        }
        log_double(e, log, CHANGE_BASIS, 0);
    }

    /// The memory router (`00c85750`) with a stack top in its first word and a heap allocator
    /// at `+0x10` whose `bufferAlloc` (slot `0xC`) hands out the requested bytes; the array
    /// helpers `00c908c0` (reserve to a capacity) and `00c90940` (double the capacity) move the
    /// elements into a new buffer and log their arguments. Answers `(router, stack top)`.
    fn heap(e: &mut Engine, log: &Log) -> (u32, u32) {
        let stack = e.mem.alloc(0x1000);
        let allocator = logging_object(e, log, ALLOCATOR);
        e.register_double(ALLOCATOR + 0xc, |e, a| {
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size),
                ..Ret::default()
            }
        });
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for (address, grow) in [(ARRAY_RESERVE, false), (ARRAY_GROW, true)] {
            let log = Rc::clone(log);
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let array = a[1];
                let (data, size) = (e.mem.u32(array), e.mem.u32(array + 4));
                let (capacity, element) = if grow {
                    ((e.mem.u32(array + 8) & 0x3fff_ffff).max(1) * 2, a[2])
                } else {
                    (a[2], a[3])
                };
                let moved = if size > 0 {
                    e.mem.bytes(data, size * element)
                } else {
                    Vec::new()
                };
                let buffer = e.mem.alloc(capacity * element);
                e.mem.write(buffer, &moved);
                e.mem.set_u32(array, buffer);
                e.mem.set_u32(array + 8, capacity);
                Ret::default()
            });
        }
        (router, stack)
    }

    /// The CRT square root `00ec6040` (argument and result in `ST0`).
    fn sqrt_double(e: &mut Engine) {
        e.register_double(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
    }

    // --- computeSphereVolumeMassProperties and computeBoxVolumeMassProperties ---------

    #[test]
    fn sphere_volume_mass_properties_follow_the_solid_sphere_formulas_00ceba60() {
        let mut e = setup();
        let out = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ce_ba60, &args![2.0f32, 3.0f32, out]).u32(), 0);
        near_record(
            &e,
            out,
            (f64::from(4.1887903f32) * 8.0, 3.0),
            [0.0; 3],
            [4.8; 3],
        );
        let rows = rows_at(&e, out + INERTIA);
        assert_eq!(rows[0][1], 0.0);
        assert_eq!(rows[2][0], 0.0);
        // A signalling mass is quieted by the x87 copy; a mass or radius at zero is refused.
        e.call(0x00ce_ba60, &args![1.0f32, 0x7f80_0001u32, out]);
        assert_eq!(e.mem.u32(out + MASS), 0x7fc0_0001);
        let untouched = put_record(&mut e, 7.0, 7.0, [7.0; 4], [[7.0; 4]; 3]);
        assert_eq!(
            e.call(0x00ce_ba60, &args![2.0f32, 0.0f32, untouched]).u32(),
            1
        );
        assert_eq!(
            e.call(0x00ce_ba60, &args![0.0f32, 1.0f32, untouched]).u32(),
            1
        );
        assert_eq!(e.mem.f32(untouched + MASS), 7.0);
    }

    #[test]
    fn box_volume_mass_properties_follow_the_solid_box_formulas_00cebaf0() {
        let mut e = setup();
        let half = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let out = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ce_baf0, &args![half, 6.0f32, out]).u32(), 0);
        near_record(&e, out, (48.0, 6.0), [0.0; 3], [26.0, 20.0, 10.0]);
        assert_eq!(e.call(0x00ce_baf0, &args![half, 0.0f32, out]).u32(), 1);
        e.call(0x00ce_baf0, &args![half, 0x7f80_0001u32, out]);
        assert_eq!(e.mem.u32(out + MASS), 0x7fc0_0001);
    }

    // --- copies and shifts of mass properties -----------------------------------------

    #[test]
    fn mass_and_vectors_are_copied_into_the_constraint_block_00cebdb0() {
        let mut e = setup();
        let rows = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ];
        let source = put_record(&mut e, 1.5, 5.0, [13.0, 14.0, 15.0, 16.0], rows);
        let block = e.mem.alloc(0xa0);
        e.call(0x00ce_bdb0, &args![source, block]);
        assert_eq!(e.mem.f32(block + 0x90), 5.0);
        assert_eq!(load_vec(&e, block + 0x80), [13.0, 14.0, 15.0, 16.0]);
        assert_eq!(rows_at(&e, block + 0x50), rows);
        assert_eq!(e.mem.f32(block), 0.0);
        e.mem.set_u32(source + MASS, 0x7f80_0001);
        e.call(0x00ce_bdb0, &args![source, block]);
        assert_eq!(e.mem.u32(block + 0x90), 0x7fc0_0001);
    }

    #[test]
    fn rows_are_copied_and_scaled_by_the_mass_ratio_00cebdf0() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let rows = tensor([1.0, 2.0, 3.0], [0.5, 0.25, 0.125]);
        let source = put_record(&mut e, 1.0, 2.0, [4.0, 5.0, 6.0, 7.0], rows);
        let block = e.mem.alloc(0xa0);
        e.call(0x00ce_bdf0, &args![source, 6.0f32, block]);
        let base = block + 0x50;
        assert_eq!(e.mem.f32(base + 0x40), 6.0);
        assert_eq!(load_vec(&e, base + 0x30), [4.0, 5.0, 6.0, 7.0]);
        let scaled_rows = rows.map(|row| row.map(|value| value * 3.0));
        assert_eq!(rows_at(&e, base), scaled_rows);
    }

    #[test]
    fn shifting_to_the_centre_of_mass_subtracts_the_parallel_axis_terms_00cebe80() {
        let mut e = setup();
        let offset = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let inertia = put_rows(&mut e, tensor([10.0, 20.0, 30.0], [0.0; 3]));
        e.call(0x00ce_be80, &args![offset, 2.0f32, inertia]);
        assert_eq!(
            rows_at(&e, inertia),
            tensor([-16.0, 0.0, 20.0], [4.0, 12.0, 6.0])
        );
    }

    #[test]
    fn the_opposite_shift_adds_the_parallel_axis_terms_00cebf30() {
        let mut e = setup();
        let offset = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let inertia = put_rows(&mut e, tensor([10.0, 20.0, 30.0], [0.0; 3]));
        e.call(0x00ce_bf30, &args![offset, 2.0f32, inertia]);
        assert_eq!(
            rows_at(&e, inertia),
            tensor([36.0, 40.0, 40.0], [-4.0, -12.0, -6.0])
        );
    }

    #[test]
    fn scaling_a_record_scales_its_mass_and_rows_00cebfd0() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let rows = tensor([1.0, 2.0, 3.0], [0.5, 0.25, 0.125]);
        let record = put_record(&mut e, 1.0, 3.0, [0.0; 4], rows);
        e.call(0x00ce_bfd0, &args![record, 2.0f32]);
        assert_eq!(e.mem.f32(record + MASS), 6.0);
        assert_eq!(
            rows_at(&e, record + INERTIA),
            rows.map(|row| row.map(|v| v * 2.0))
        );
        assert_eq!(e.mem.f32(record + VOLUME), 1.0);
    }

    // --- the integrals over a polygon, a face and a closed surface --------------------

    #[test]
    fn projection_integrals_of_a_right_triangle_match_the_closed_forms_00cec000() {
        let mut e = setup();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 1);
        let unit = put_vecs(
            &mut e,
            &[[0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]],
        );
        e.call(0x00ce_c000, &args![this, unit]);
        let found = |offset: u32| e.mem.f32(this + offset);
        near(found(0xc), 1.0 / 2.0);
        near(found(0x10), 1.0 / 6.0);
        near(found(0x14), 1.0 / 6.0);
        near(found(0x18), 1.0 / 12.0);
        near(found(0x1c), 1.0 / 24.0);
        near(found(0x20), 1.0 / 12.0);
        near(found(0x24), 1.0 / 20.0);
        near(found(0x28), 1.0 / 60.0);
        near(found(0x2c), 1.0 / 60.0);
        near(found(0x30), 1.0 / 20.0);
        // Another triangle: (1,1), (3,1), (1,2) of area 1.
        let shifted = put_vecs(
            &mut e,
            &[
                [1.0, 1.0, 0.0, 0.0],
                [3.0, 1.0, 0.0, 0.0],
                [1.0, 2.0, 0.0, 0.0],
            ],
        );
        e.call(0x00ce_c000, &args![this, shifted]);
        let found = |offset: u32| e.mem.f32(this + offset);
        near(found(0xc), 1.0);
        near(found(0x10), 5.0 / 3.0);
        near(found(0x14), 4.0 / 3.0);
        near(found(0x18), 3.0);
        near(found(0x1c), 26.0 / 12.0);
        near(found(0x20), 11.0 / 6.0);
    }

    #[test]
    fn face_integrals_of_a_slanted_triangle_match_the_surface_integrals_00cec3d0() {
        let mut e = setup();
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 8, 2);
        let corners = put_vecs(
            &mut e,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 3.0, 0.0],
            ],
        );
        let normal = put_vec(&mut e, [6.0 / 7.0, 3.0 / 7.0, 2.0 / 7.0, 0.0]);
        e.call(0x00ce_c3d0, &args![this, corners, normal]);
        // The face is the plane x + y / 2 + z / 3 = 1 with the area 3.5; the integral of a
        // monomial is the area times the mean over the barycentric coordinates.
        let area = 3.5;
        let expected = [
            (0x34, area / 3.0),
            (0x38, 2.0 * area / 3.0),
            (0x3c, area),
            (0x40, area / 6.0),
            (0x44, 4.0 * area / 6.0),
            (0x48, 9.0 * area / 6.0),
            (0x4c, area / 10.0),
            (0x50, 8.0 * area / 10.0),
            (0x54, 27.0 * area / 10.0),
            (0x58, 2.0 * area / 30.0),
            (0x5c, 12.0 * area / 30.0),
            (0x60, 9.0 * area / 30.0),
        ];
        for (offset, value) in expected {
            near(e.mem.f32(this + offset), value);
        }
        // The projection integrals are left in place.
        near(e.mem.f32(this + 0xc), 1.0);
    }

    #[test]
    fn the_inertial_tensor_of_a_cube_comes_out_as_mass_over_six_00cec6b0() {
        let mut e = setup();
        let this = e.mem.alloc(0x90);
        e.mem.set_f32(this + 0x64, 1.0);
        for (offset, value) in [
            (0x68, 0.5),
            (0x6c, 0.5),
            (0x70, 0.5),
            (0x74, 1.0 / 3.0),
            (0x78, 1.0 / 3.0),
            (0x7c, 1.0 / 3.0),
            (0x80, 0.25),
            (0x84, 0.25),
            (0x88, 0.25),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        let centre = e.mem.alloc(0x10);
        let inertia = e.mem.alloc(0x30);
        e.call(0x00ce_c6b0, &args![this, 2.0f32, 2.0f32, centre, inertia]);
        for lane in 0..3 {
            near(e.mem.f32(centre + 4 * lane), 0.5);
        }
        let rows = rows_at(&e, inertia);
        near_rows(
            rows,
            [
                [1.0 / 3.0, 0.0, 0.0],
                [0.0, 1.0 / 3.0, 0.0],
                [0.0, 0.0, 1.0 / 3.0],
            ],
        );
    }

    #[test]
    fn the_bounding_box_takes_the_minimum_and_maximum_of_every_lane_00cec7f0() {
        let mut e = setup();
        let vertices = put_vecs(
            &mut e,
            &[
                [-1.0, 2.0, 3.0, 9.0],
                [4.0, -5.0, 6.0, 9.0],
                [0.0, 0.0, 0.0, 9.0],
            ],
        );
        let centre = e.mem.alloc(0x10);
        let extent = e.mem.alloc(0x10);
        e.call(0x00ce_c7f0, &args![vertices, 3i32, centre, extent]);
        assert_eq!(load_vec(&e, centre), [1.5, -1.5, 3.0, 9.0]);
        assert_eq!(load_vec(&e, extent), [5.0, 7.0, 6.0, 0.0]);
        store_vec(&mut e, centre, [7.0; 4]);
        store_vec(&mut e, extent, [7.0; 4]);
        e.call(0x00ce_c7f0, &args![vertices, 0i32, centre, extent]);
        assert_eq!(load_vec(&e, centre), [0.0; 4]);
        assert_eq!(load_vec(&e, extent), [0.0; 4]);
    }

    #[test]
    fn a_symmetric_matrix_is_diagonalised_and_its_axes_are_orthonormal_00cec880() {
        let mut e = setup();
        sqrt_double(&mut e);
        let original = tensor([2.0, 2.0, 3.0], [1.0, 0.0, 0.0]);
        let matrix = put_rows(&mut e, original);
        let axes = e.mem.alloc(0x30);
        e.call(0x00ce_c880, &args![matrix, axes]);
        let diagonal = rows_at(&e, matrix);
        let found = rows_at(&e, axes);
        // The off-diagonal entries are gone and the diagonal holds the eigenvalues 1, 3, 3.
        let mut values = [diagonal[0][0], diagonal[1][1], diagonal[2][2]];
        values.sort_by(f32::total_cmp);
        near(values[0], 1.0);
        near(values[1], 3.0);
        near(values[2], 3.0);
        near(diagonal[0][1], 0.0);
        near(diagonal[0][2], 0.0);
        near(diagonal[1][2], 0.0);
        for first in 0..3 {
            for second in 0..3 {
                let dot: f32 = (0..3)
                    .map(|lane| found[first][lane] * found[second][lane])
                    .sum();
                near(dot, if first == second { 1.0 } else { 0.0 });
            }
            // Each row of the result is an eigenvector of the original matrix (the rows are the
            // columns of the hkMatrix3) with the eigenvalue found on the diagonal.
            let axis = found[first];
            for row in 0..3 {
                let product: f32 = (0..3).map(|lane| original[row][lane] * axis[lane]).sum();
                near(product, f64::from(diagonal[first][first] * axis[row]));
            }
        }
    }

    // --- the convex hull, rescaling and the hollow shapes -----------------------------

    /// The convex hull builder as doubles: a hull of three dimensions whose first volume is
    /// below the limit (so it is rebuilt once) and then 2.0, with the surface area 10, the
    /// given centre and rows. The build logs its arguments and the three words of the
    /// source it is given.
    fn hull_doubles(e: &mut Engine, log: &Log, centre: u32, rows: u32) {
        for address in [
            HULL_CONSTRUCT,
            HULL_CONFIG_CONSTRUCT,
            HULL_BUILD_MASS,
            HULL_EXPAND,
            HULL_DESTRUCT,
        ] {
            log_double(e, log, address, 0);
        }
        let sink = Rc::clone(log);
        e.register_double(HULL_BUILD, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..3).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((HULL_BUILD, words));
            Ret::default()
        });
        log_double(e, log, HULL_DIMENSIONS, 3);
        log_double(e, log, HULL_CENTRE, centre);
        log_double(e, log, HULL_ROWS, rows);
        e.register_double(HULL_SURFACE_AREA, |_, _| Ret {
            st0: 10.0,
            ..Ret::default()
        });
        let mut volumes = 0;
        e.register_double(HULL_VOLUME, move |_, _| {
            volumes += 1;
            Ret {
                st0: if volumes == 1 { 1e-6 } else { 2.0 },
                ..Ret::default()
            }
        });
    }

    #[test]
    fn convex_hull_properties_come_from_the_hull_builder_and_the_margin_loop_00cebbb0() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let centre = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let hull_rows = put_rows(&mut e, tensor([1.0, 2.0, 3.0], [0.0; 3]));
        hull_doubles(&mut e, &log, centre, hull_rows);
        let source = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ce_bbb0, &args![source, 0.0f32, out]).u32(), 0);
        assert_eq!(calls(&log, HULL_BUILD).len(), 2);
        let margin = (f64::from(0.05f32) / 5.0) as f32;
        let expansions = calls(&log, HULL_EXPAND);
        assert_eq!(expansions.len(), 1);
        assert_eq!(expansions[0][1], margin.to_bits());
        assert_eq!(calls(&log, HULL_DESTRUCT).len(), 1);
        assert_eq!(e.mem.f32(out + VOLUME), 2.0);
        assert_eq!(e.mem.f32(out + MASS), 1.0);
        assert_eq!(load_vec(&e, out + CENTRE), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(rows_at(&e, out + INERTIA), rows_at(&e, hull_rows));
        // A radius adds the shell: area * r + r * (r * 4 pi * r) / 3 to the volume, and the
        // rows are scaled by that surplus over the total plus one.
        e.call(0x00ce_bbb0, &args![source, 0.5f32, out]);
        let surplus = 10.0 * 0.5 + 0.5 * (0.5 * 12.566371 * 0.5) / 3.0;
        near(e.mem.f32(out + VOLUME), 2.0 + surplus);
        let factor = surplus / (2.0 + surplus) + 1.0;
        near(e.mem.f32(out + INERTIA), factor);
        near(e.mem.f32(out + INERTIA + 0x14), 2.0 * factor);
        // A hull that cannot be built answers 1 and is still destroyed.
        log_double(&mut e, &log, HULL_DIMENSIONS, u32::MAX);
        assert_eq!(e.call(0x00ce_bbb0, &args![source, 0.0f32, out]).u32(), 1);
        assert_eq!(calls(&log, HULL_DESTRUCT).len(), 3);
    }

    #[test]
    fn a_record_is_given_a_new_mass_and_its_rows_follow_00cecc90() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let rows = tensor([1.0, 2.0, 3.0], [0.5, 0.25, 0.125]);
        let record = put_record(&mut e, 1.0, 2.0, [0.0; 4], rows);
        e.call(0x00ce_cc90, &args![record, 6.0f32]);
        assert_eq!(e.mem.f32(record + MASS), 6.0);
        assert_eq!(
            rows_at(&e, record + INERTIA),
            rows.map(|row| row.map(|v| v * 3.0))
        );
    }

    #[test]
    fn a_hollow_sphere_is_the_outer_sphere_less_the_inner_one_00ceccf0() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let out = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x00ce_ccf0, &args![2.0f32, 10.0f32, 0.5f32, out])
                .u32(),
            0
        );
        let factor = f64::from(4.1887903f32);
        let shell = factor * (8.0 - 3.375);
        let moment = 0.4 * 10.0 * (32.0 - 7.59375) / (8.0 - 3.375);
        near_record(&e, out, (shell, 10.0), [0.0; 3], [moment; 3]);
        // The wall must be thinner than the radius and above zero; the mass above zero.
        for (radius, mass, thickness) in
            [(2.0f32, 10.0f32, 2.0f32), (2.0, 10.0, 0.0), (2.0, 0.0, 0.5)]
        {
            let refused = e
                .call(0x00ce_ccf0, &args![radius, mass, thickness, out])
                .u32();
            assert_eq!(refused, 1);
        }
    }

    #[test]
    fn a_hollow_box_is_the_outer_box_less_the_inner_one_00cece50() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let half = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let out = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x00ce_ce50, &args![half, 12.0f32, 0.5f32, out])
                .u32(),
            0
        );
        // Outer box 2 x 4 x 6 (volume 48) less the box of the half extents 0.5, 1.5, 2.5
        // (volume 15), both at unit density; the tensors are scaled to the mass 12.
        let scale = 12.0 / 33.0;
        near_record(
            &e,
            out,
            (33.0, 12.0),
            [0.0; 3],
            [
                (48.0 * 13.0 / 3.0 - 15.0 * 8.5 / 3.0) * scale,
                (48.0 * 10.0 / 3.0 - 15.0 * 6.5 / 3.0) * scale,
                (48.0 * 5.0 / 3.0 - 15.0 * 2.5 / 3.0) * scale,
            ],
        );
        // A wall as thick as the smallest half extent, or none, is refused.
        for thickness in [1.0f32, 0.0] {
            assert_eq!(
                e.call(0x00ce_ce50, &args![half, 12.0f32, thickness, out])
                    .u32(),
                1
            );
        }
        assert_eq!(
            e.call(0x00ce_ce50, &args![half, 0.0f32, 0.5f32, out]).u32(),
            1
        );
    }

    // --- combining several bodies ------------------------------------------------------

    /// A list of mass-properties elements (a record, then the transform with the rotation
    /// rows at `+0x50` and the translation at `+0x80`): unit spheres of mass 1 whose
    /// translations are the given vectors.
    fn element_list(e: &mut Engine, translations: &[Vec4]) -> u32 {
        let data = e.mem.alloc(0x90 * translations.len() as u32);
        for (index, translation) in translations.iter().enumerate() {
            let element = data + 0x90 * index as u32;
            e.mem.set_f32(element + VOLUME, 1.0);
            e.mem.set_f32(element + MASS, 1.0);
            for (row, vector) in IDENTITY_ROWS.into_iter().enumerate() {
                store_vec(
                    e,
                    element + INERTIA + 0x10 * row as u32,
                    tensor([0.4; 3], [0.0; 3])[row],
                );
                store_vec(e, element + 0x50 + 0x10 * row as u32, vector);
            }
            store_vec(e, element + 0x80, *translation);
        }
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, translations.len() as u32);
        list
    }

    #[test]
    fn two_spheres_are_combined_with_the_parallel_axis_shift_00ced090() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let list = element_list(&mut e, &[[1.0, 0.0, 0.0, 0.0], [-1.0, 0.0, 0.0, 0.0]]);
        let out = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ce_d090, &args![list, out]).u32(), 0);
        near_record(&e, out, (2.0, 2.0), [0.0; 3], [0.8, 2.8, 2.8]);
        assert_eq!(calls(&log, CHANGE_BASIS).len(), 2);
        // An empty list, or one whose masses add up to nothing, answers 1.
        e.mem.set_u32(list + 4, 0);
        assert_eq!(e.call(0x00ce_d090, &args![list, out]).u32(), 1);
        let massless = element_list(&mut e, &[[0.0; 4]]);
        let data = e.mem.u32(massless);
        e.mem.set_f32(data + MASS, 0.0);
        assert_eq!(e.call(0x00ce_d090, &args![massless, out]).u32(), 1);
    }

    #[test]
    fn the_diagonal_becomes_its_largest_entry_00ced310() {
        let mut e = setup();
        let matrix = put_rows(&mut e, tensor([1.0, 5.0, 3.0], [0.5, 0.25, 0.125]));
        e.call(0x00ce_d310, &args![matrix]);
        assert_eq!(rows_at(&e, matrix), tensor([5.0; 3], [0.0; 3]));
        // A signalling NaN as the second entry is not below the first, so it is taken, and
        // then the third is not below it either: the third entry wins.
        e.mem.set_f32(matrix, 9.0);
        e.mem.set_u32(matrix + 0x14, 0x7f80_0001);
        e.mem.set_f32(matrix + 0x28, 4.0);
        e.call(0x00ce_d310, &args![matrix]);
        assert_eq!(rows_at(&e, matrix), tensor([4.0; 3], [0.0; 3]));
    }

    // --- closed triangle meshes --------------------------------------------------------

    /// The vertices of the unit cube and its twelve triangles, wound outwards.
    const CUBE_CORNERS: [Vec4; 8] = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [1.0, 1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0, 0.0],
        [0.0, 1.0, 1.0, 0.0],
    ];
    const CUBE_FACES: [[u32; 3]; 12] = [
        [0, 2, 1],
        [0, 3, 2],
        [4, 5, 6],
        [4, 6, 7],
        [0, 1, 5],
        [0, 5, 4],
        [3, 6, 2],
        [3, 7, 6],
        [0, 4, 7],
        [0, 7, 3],
        [1, 2, 6],
        [1, 6, 5],
    ];

    /// A mesh of the vertices and the first `faces` triangles: vertex pointer `+0`, vertex
    /// count `+4`, face pointer `+0xC` (16 bytes per face) and face count `+0x10`.
    fn mesh(e: &mut Engine, corners: &[Vec4], faces: &[[u32; 3]]) -> u32 {
        let vertices = put_vecs(e, corners);
        let triangles = e.mem.alloc(0x10 * faces.len().max(1) as u32);
        for (index, face) in faces.iter().enumerate() {
            for (corner, vertex) in face.iter().enumerate() {
                e.mem
                    .set_u32(triangles + 0x10 * index as u32 + 4 * corner as u32, *vertex);
            }
        }
        let mesh = e.mem.alloc(0x20);
        e.mem.set_u32(mesh, vertices);
        e.mem.set_u32(mesh + 4, corners.len() as u32);
        e.mem.set_u32(mesh + 0xc, triangles);
        e.mem.set_u32(mesh + 0x10, faces.len() as u32);
        mesh
    }

    #[test]
    fn the_volume_integrals_of_the_unit_cube_match_the_closed_forms_00ced390() {
        let mut e = setup();
        let cube = mesh(&mut e, &CUBE_CORNERS, &CUBE_FACES);
        let this = e.mem.alloc(0x90);
        let offset = put_vec(&mut e, [0.0; 4]);
        e.call(0x00ce_d390, &args![this, cube, offset]);
        near(e.mem.f32(this + 0x64), 1.0);
        for lane in 0..3 {
            near(e.mem.f32(this + 0x68 + 4 * lane), 0.5);
            near(e.mem.f32(this + 0x74 + 4 * lane), 1.0 / 3.0);
            near(e.mem.f32(this + 0x80 + 4 * lane), 0.25);
        }
        // Moved so that the cube is centred, the first moments vanish and the second ones
        // are 1/12.
        store_vec(&mut e, offset, [-0.5, -0.5, -0.5, 0.0]);
        e.call(0x00ce_d390, &args![this, cube, offset]);
        near(e.mem.f32(this + 0x64), 1.0);
        for lane in 0..3 {
            near(e.mem.f32(this + 0x68 + 4 * lane), 0.0);
            near(e.mem.f32(this + 0x74 + 4 * lane), 1.0 / 12.0);
            near(e.mem.f32(this + 0x80 + 4 * lane), 0.0);
        }
        // A mesh without faces leaves the sums at zero.
        let empty = mesh(&mut e, &CUBE_CORNERS, &[]);
        e.call(0x00ce_d390, &args![this, empty, offset]);
        assert_eq!(e.mem.f32(this + 0x64), 0.0);
        assert_eq!(e.mem.f32(this + 0x74), 0.0);
    }

    #[test]
    fn the_geometry_of_a_cube_has_the_inertia_of_a_cube_00ced6b0() {
        let mut e = setup();
        let cube = mesh(&mut e, &CUBE_CORNERS, &CUBE_FACES);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_d6b0, &args![cube, 2.0f32, out]);
        near_record(&e, out, (1.0, 2.0), [0.5; 3], [1.0 / 3.0; 3]);
        near_rows(
            rows_at(&e, out + INERTIA),
            [
                [1.0 / 3.0, 0.0, 0.0],
                [0.0, 1.0 / 3.0, 0.0],
                [0.0, 0.0, 1.0 / 3.0],
            ],
        );
        // A mass that is not above zero becomes 1.0.
        e.call(0x00ce_d6b0, &args![cube, 0.0f32, out]);
        near(e.mem.f32(out + MASS), 1.0);
        // Without faces the volume integral is zero and the bounding box's properties are
        // copied: its volume is 8 times the product of its full sizes.
        let empty = mesh(&mut e, &CUBE_CORNERS, &[]);
        e.call(0x00ce_d6b0, &args![empty, 3.0f32, out]);
        near_record(&e, out, (8.0, 3.0), [0.5; 3], [2.0; 3]);
        // A far vertex that no face uses makes the box tensor much larger than the cube's,
        // so the diagonal is raised to a tenth of the box's and the centre lanes replaced.
        let mut corners = CUBE_CORNERS.to_vec();
        corners.push([10.0, 10.0, 10.0, 0.0]);
        let stretched = mesh(&mut e, &corners, &CUBE_FACES);
        e.call(0x00ce_d6b0, &args![stretched, 2.0f32, out]);
        let box_diagonal = (100.0 + 100.0) * (f64::from(0.33333334f32) * 2.0);
        near_record(&e, out, (1.0, 2.0), [5.0; 3], [box_diagonal * 0.1; 3]);
    }

    #[test]
    fn the_checked_geometry_function_refuses_what_has_no_volume_00ced8e0() {
        let mut e = setup();
        let cube = mesh(&mut e, &CUBE_CORNERS, &CUBE_FACES);
        let out = e.mem.alloc(0x50);
        assert_eq!(e.call(0x00ce_d8e0, &args![cube, 2.0f32, out]).u32(), 0);
        near_record(&e, out, (1.0, 2.0), [0.5; 3], [1.0 / 3.0; 3]);
        let empty = mesh(&mut e, &CUBE_CORNERS, &[]);
        let untouched = put_record(&mut e, 7.0, 7.0, [7.0; 4], [[7.0; 4]; 3]);
        assert_eq!(
            e.call(0x00ce_d8e0, &args![empty, 2.0f32, untouched]).u32(),
            1
        );
        assert_eq!(e.mem.f32(untouched + MASS), 7.0);
        assert_eq!(
            e.call(0x00ce_d8e0, &args![cube, 0.0f32, untouched]).u32(),
            1
        );
        assert_eq!(e.mem.f32(untouched + MASS), 7.0);
    }

    // --- axes of a shape, point masses and constraint graphs ----------------------------

    #[test]
    fn the_two_longest_axes_of_the_bounds_are_picked_00cedae0() {
        let mut e = setup();
        let log = new_log();
        e.register_double(ROTATION_BASIS, |e, a| {
            for (column, vector) in [
                [1.0, 2.0, 3.0, 0.0],
                [4.0, 5.0, 6.0, 0.0],
                [7.0, 8.0, 9.0, 0.0],
            ]
            .into_iter()
            .enumerate()
            {
                store_vec(e, a[2 + column], vector);
            }
            Ret::default()
        });
        let shape = logging_object(&mut e, &log, SHAPE);
        let sink = Rc::clone(&log);
        e.register_double(SHAPE + 0x1c, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..12).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((SHAPE + 0x1c, words));
            store_vec(e, a[3], [0.0; 4]);
            store_vec(e, a[3] + 0x10, [1.0, 3.0, 2.0, 0.0]);
            Ret::default()
        });
        let rotation = put_rows(&mut e, tensor([1.0, 1.0, 1.0], [0.0; 3]));
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        e.call(0x00ce_dae0, &args![shape, rotation, first, second]);
        let seen = calls(&log, SHAPE + 0x1c);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][2], 0.0f32.to_bits());
        let matrix: Vec<f32> = seen[0][4..16]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        assert_eq!(
            matrix,
            [1.0, 4.0, 7.0, 0.0, 2.0, 5.0, 8.0, 0.0, 3.0, 6.0, 9.0, 0.0]
        );
        assert_eq!(load_vec(&e, first), [2.0, 5.0, 8.0, 0.0]);
        assert_eq!(load_vec(&e, second), [3.0, 6.0, 9.0, 0.0]);
    }

    #[test]
    fn point_masses_give_the_summed_squares_about_their_mean_00cedcb0() {
        let mut e = setup();
        let log = new_log();
        let (router, _) = heap(&mut e, &log);
        let allocator = e.mem.u32(router + 0x10);
        let points = e.mem.alloc(0x30);
        for (index, point) in [
            [1.0f32, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            for (lane, value) in point.into_iter().enumerate() {
                e.mem
                    .set_f32(points + 12 * index as u32 + 4 * lane as u32, value);
            }
        }
        let out = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x00ce_dcb0, &args![points, 12i32, 4i32, 4.0f32, out])
                .u32(),
            0
        );
        near_record(&e, out, (0.0, 4.0), [0.0; 3], [2.0, 2.0, 4.0]);
        let rows = rows_at(&e, out + INERTIA);
        assert_eq!((rows[0][1], rows[0][2], rows[1][2]), (0.0, 0.0, 0.0));
        let reserved = calls(&log, ARRAY_RESERVE);
        assert_eq!(reserved.len(), 1);
        assert_eq!(
            (reserved[0][0], reserved[0][2], reserved[0][3]),
            (allocator, 4, 0x10)
        );
        let freed = calls(&log, ALLOCATOR + 0x10);
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][2], 0x40);
        assert_eq!(
            e.call(0x00ce_dcb0, &args![points, 12i32, 0i32, 4.0f32, out])
                .u32(),
            1
        );
    }

    /// `00ca8920` (set the inertia of a node) as a double that logs its arguments, the three
    /// diagonal floats of the matrix it is given and the router's stack top at that moment.
    fn inertia_double(e: &mut Engine, log: &Log, router: u32) {
        let sink = Rc::clone(log);
        e.register_double(SET_INERTIA, move |e, a| {
            let mut words = a.to_vec();
            words.extend([0, 0x14, 0x28].map(|diagonal| e.mem.u32(a[1] + diagonal)));
            words.push(e.mem.u32(router));
            sink.borrow_mut().push((SET_INERTIA, words));
            Ret::default()
        });
    }

    /// A node `a` linked to a node `b` by one entry of the list `list`. The embedded object
    /// of a node (at `+0xE0`) answers its matrix (virtual function `+0x14`) with the three
    /// diagonal floats stored at `node + 0x20`. Answers `(list, a)`.
    fn graph(e: &mut Engine, diagonal_a: [f32; 3], diagonal_b: [f32; 3]) -> (u32, u32) {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, NODE + 0x14);
        e.register_double(NODE + 0x14, |e, a| {
            for axis in 0..3 {
                let diagonal = e.mem.f32(a[0] - 0xe0 + 0x20 + 4 * axis);
                e.mem.set_f32(a[1] + 0x14 * axis, diagonal);
            }
            Ret::default()
        });
        let [a, b] = [diagonal_a, diagonal_b].map(|diagonal| {
            let node = e.mem.alloc(0x100);
            e.mem.set_u32(node + 0xe0, vtable);
            for (axis, value) in diagonal.into_iter().enumerate() {
                e.mem.set_f32(node + 0x20 + 4 * axis as u32, value);
            }
            node
        });
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 0x14, a);
        e.mem.set_u32(entry + 0x18, b);
        let entries = e.mem.alloc(4);
        e.mem.set_u32(entries, entry);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, entries);
        e.mem.set_u32(list + 4, 1);
        e.mem.set_u32(list + 8, 0x8000_0001);
        (list, a)
    }

    #[test]
    fn inertia_is_raised_to_the_largest_of_the_linked_nodes_00cedec0() {
        let mut e = setup();
        let log = new_log();
        let (router, _) = heap(&mut e, &log);
        inertia_double(&mut e, &log, router);
        // The linked node has the diagonal 2, 3, 1 (largest 3); the node itself 1, 1, 1 and
        // the factor 2, so the diagonal becomes 3 and the answer is 3.
        let (list, a) = graph(&mut e, [1.0; 3], [2.0, 3.0, 1.0]);
        assert_eq!(e.call(0x00ce_dec0, &args![list, a, 2.0f32]).f32(), 3.0);
        assert_eq!(e.mem.u32(list + 4), 0);
        let seen = calls(&log, SET_INERTIA);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], a);
        let diagonal: Vec<f32> = seen[0][2..5]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        assert_eq!(diagonal, [3.0; 3]);
        // A node whose byte at +0xE8 is 4 or 5 answers 0 and its inertia is not stored.
        let (list, a) = graph(&mut e, [1.0; 3], [2.0, 3.0, 1.0]);
        e.mem.set_u8(a + 0xe8, 4);
        assert_eq!(e.call(0x00ce_dec0, &args![list, a, 2.0f32]).f32(), 0.0);
        assert_eq!(calls(&log, SET_INERTIA).len(), 1);
        // With nothing linked the answer is the node's own largest diagonal entry.
        let (list, a) = graph(&mut e, [1.0, 7.0, 2.0], [0.0; 3]);
        e.mem.set_u32(list + 4, 0);
        assert_eq!(e.call(0x00ce_dec0, &args![list, a, 2.0f32]).f32(), 7.0);
        assert_eq!(calls(&log, SET_INERTIA).len(), 1);
    }

    #[test]
    fn the_entries_are_copied_onto_the_router_stack_for_the_walk_00cee1c0() {
        let mut e = setup();
        let log = new_log();
        let (router, stack) = heap(&mut e, &log);
        inertia_double(&mut e, &log, router);
        let (list, a) = graph(&mut e, [1.0; 3], [2.0, 3.0, 1.0]);
        let entries = e.mem.u32(list);
        e.call(0x00ce_e1c0, &args![entries, 1i32, a, 2.0f32]);
        let seen = calls(&log, SET_INERTIA);
        assert_eq!(seen.len(), 1);
        // While the walk ran the router's stack top was moved up by the rounded size.
        assert_eq!(seen[0][5], stack + 0x80);
        assert_eq!(e.mem.u32(router), stack);
        assert!(calls(&log, ALLOCATOR + 0x10).is_empty());
    }

    // --- triangles, capsules, cylinders and meshes --------------------------------------

    #[test]
    fn triangle_properties_cover_the_flat_the_degenerate_and_the_slab_case_00cee290() {
        let mut e = setup();
        let log = new_log();
        heap(&mut e, &log);
        log_double(&mut e, &log, GEOMETRY_DESTRUCT, 0);
        let corners = put_vecs(
            &mut e,
            &[
                [0.0; 4],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 1.0, 0.0, 0.0],
                [2.0, 2.0, 0.0, 0.0],
            ],
        );
        let at = |index: u32| corners + 0x10 * index;
        let out = e.mem.alloc(0x50);
        // A flat triangle: the tensor of the plate about its centroid (1/3, 1/3, 0).
        let flat = e.call(
            0x00ce_e290,
            &args![at(0), at(1), at(2), 12.0f32, 0.0f32, out],
        );
        assert_eq!(flat.u32(), 0);
        near_record(
            &e,
            out,
            (0.0, 12.0),
            [1.0 / 3.0, 1.0 / 3.0, 0.0],
            [2.0 / 3.0, 2.0 / 3.0, 4.0 / 3.0],
        );
        near(e.mem.f32(out + INERTIA + 4), 1.0 / 3.0);
        near(e.mem.f32(out + INERTIA + 0x10), 1.0 / 3.0);
        // Three points on a line are a point mass at their mean (1, 1, 0).
        let line = e.call(
            0x00ce_e290,
            &args![at(0), at(3), at(4), 6.0f32, 1.0f32, out],
        );
        assert_eq!(line.u32(), 0);
        near_record(&e, out, (0.0, 6.0), [1.0, 1.0, 0.0], [6.0, 6.0, 12.0]);
        near(e.mem.f32(out + INERTIA + 4), -6.0);
        // A slab: the plate's tensor plus mass * thickness^2 / 12 about the normal's axes.
        let slab = e.call(
            0x00ce_e290,
            &args![at(0), at(1), at(2), 12.0f32, 0.2f32, out],
        );
        assert_eq!(slab.u32(), 0);
        near_record(
            &e,
            out,
            (0.1, 12.0),
            [1.0 / 3.0, 1.0 / 3.0, 0.0],
            [2.0 / 3.0 + 0.04, 2.0 / 3.0 + 0.04, 4.0 / 3.0],
        );
        near(e.mem.f32(out + INERTIA + 4), 1.0 / 3.0);
        assert_eq!(calls(&log, GEOMETRY_DESTRUCT).len(), 1);
        // The mass has to be above zero and the thickness not below it.
        assert_eq!(
            e.call(
                0x00ce_e290,
                &args![at(0), at(1), at(2), 0.0f32, 0.2f32, out]
            )
            .u32(),
            1
        );
        assert_eq!(
            e.call(
                0x00ce_e290,
                &args![at(0), at(1), at(2), 1.0f32, -0.2f32, out]
            )
            .u32(),
            1
        );
    }

    /// The scene of the capsule and cylinder functions: the matrix operations, the router and
    /// the rotation functions as doubles; the quaternion double logs the axis it is given
    /// and the rotation from it is the identity.
    fn scene() -> (Engine, Log) {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        heap(&mut e, &log);
        let sink = Rc::clone(&log);
        e.register_double(ACOS, move |_, a| {
            sink.borrow_mut().push((ACOS, a.to_vec()));
            Ret {
                st0: f64::from(f32::from_bits(a[0])).acos(),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(QUATERNION_SET_AXIS_ANGLE, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..3).map(|lane| e.mem.u32(a[1] + 4 * lane)));
            sink.borrow_mut().push((QUATERNION_SET_AXIS_ANGLE, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ROTATION_FROM_QUATERNION, move |e, a| {
            sink.borrow_mut()
                .push((ROTATION_FROM_QUATERNION, a.to_vec()));
            for (row, vector) in IDENTITY_ROWS.into_iter().enumerate() {
                store_vec(e, a[0] + 0x10 * row as u32, vector);
            }
            Ret::default()
        });
        (e, log)
    }

    /// The rotation functions' single calls for a segment along x: the angle `acos(0)`, a
    /// quaternion about the axis (0, 1, 0) and the rotation made from it.
    fn assert_quarter_turn_about_y(log: &Log) {
        assert_eq!(calls(log, ACOS), [[0.0f32.to_bits()]]);
        let quaternions = calls(log, QUATERNION_SET_AXIS_ANGLE);
        assert_eq!(quaternions.len(), 1);
        assert_eq!(quaternions[0][2], std::f32::consts::FRAC_PI_2.to_bits());
        let axis: Vec<f32> = quaternions[0][3..6]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        near(axis[0], 0.0);
        near(axis[1], 1.0);
        near(axis[2], 0.0);
        assert_eq!(calls(log, ROTATION_FROM_QUATERNION).len(), 1);
    }

    #[test]
    fn a_capsule_is_a_cylinder_and_two_hemispheres_00ceeca0() {
        let (mut e, log) = scene();
        let ends = put_vecs(&mut e, &[[0.0, 0.0, -1.0, 0.0], [0.0, 0.0, 1.0, 0.0]]);
        let out = e.mem.alloc(0x50);
        let status = e.call(0x00ce_eca0, &args![ends, ends + 0x10, 1.0f32, 10.0f32, out]);
        assert_eq!(status.u32(), 0);
        // Radius 1, length 2, mass 10: the cylinder takes 6 and each hemisphere 2.
        let volume = f64::from(4.1887903f32) + 2.0 * f64::from(std::f32::consts::PI);
        near_record(&e, out, (volume, 10.0), [0.0; 3], [12.1, 12.1, 4.6]);
        assert!(calls(&log, ACOS).is_empty());
        // The segment from (-1, 0, 0) to (1, 0, 0) is turned with the quaternion functions.
        let sideways = put_vecs(&mut e, &[[-1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]]);
        e.call(
            0x00ce_eca0,
            &args![sideways, sideways + 0x10, 1.0f32, 10.0f32, out],
        );
        assert_quarter_turn_about_y(&log);
        // A mass or radius at zero is refused.
        assert_eq!(
            e.call(0x00ce_eca0, &args![ends, ends + 0x10, 1.0f32, 0.0f32, out])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ce_eca0, &args![ends, ends + 0x10, 0.0f32, 1.0f32, out])
                .u32(),
            1
        );
    }

    #[test]
    fn a_cylinder_has_the_tensor_of_a_solid_cylinder_00cef6d0() {
        let (mut e, log) = scene();
        let ends = put_vecs(&mut e, &[[0.0, 0.0, -1.0, 0.0], [0.0, 0.0, 1.0, 0.0]]);
        let out = e.mem.alloc(0x50);
        let status = e.call(0x00ce_f6d0, &args![ends, ends + 0x10, 1.0f32, 5.0f32, out]);
        assert_eq!(status.u32(), 0);
        let pi = f64::from(std::f32::consts::PI);
        near_record(
            &e,
            out,
            (2.0 * pi, 5.0),
            [0.0; 3],
            [5.0 * (4.0 / 12.0 + 0.25), 5.0 * (4.0 / 12.0 + 0.25), 2.5],
        );
        assert!(calls(&log, ACOS).is_empty());
        let sideways = put_vecs(&mut e, &[[-1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]]);
        e.call(
            0x00ce_f6d0,
            &args![sideways, sideways + 0x10, 1.0f32, 5.0f32, out],
        );
        assert_quarter_turn_about_y(&log);
        // A mass or radius at zero, or a segment of no length, is refused.
        assert_eq!(
            e.call(0x00ce_f6d0, &args![ends, ends + 0x10, 1.0f32, 0.0f32, out])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ce_f6d0, &args![ends, ends + 0x10, 0.0f32, 1.0f32, out])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ce_f6d0, &args![ends, ends, 1.0f32, 1.0f32, out])
                .u32(),
            1
        );
    }

    #[test]
    fn the_faces_of_a_plate_share_the_mass_by_area_00cefb90() {
        let mut e = setup();
        let log = new_log();
        matrix_doubles(&mut e, &log);
        let (router, _) = heap(&mut e, &log);
        let allocator = e.mem.u32(router + 0x10);
        let square = mesh(
            &mut e,
            &[
                [0.0; 4],
                [1.0, 0.0, 0.0, 0.0],
                [1.0, 1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );
        let out = put_record(&mut e, 7.0, 7.0, [7.0; 4], [[7.0; 4]; 3]);
        let status = e.call(0x00ce_fb90, &args![square, 1e-6f32, 1u8, 12.0f32, out]);
        assert_eq!(status.u32(), 0);
        // A unit square of mass 12: 1 about each in-plane axis and 2 about the normal; the
        // volume is not written.
        near_record(&e, out, (7.0, 12.0), [0.5, 0.5, 0.0], [1.0, 1.0, 2.0]);
        assert_eq!(e.mem.f32(out + VOLUME), 7.0);
        let rows = rows_at(&e, out + INERTIA);
        near(rows[0][1], 0.0);
        near(rows[0][2], 0.0);
        near(rows[1][2], 0.0);
        let reserved = calls(&log, ARRAY_RESERVE);
        assert_eq!(
            (reserved[0][0], reserved[0][2], reserved[0][3]),
            (allocator, 2, 4)
        );
        assert_eq!(calls(&log, ALLOCATOR + 0x10)[0][2], 8);
        // Equal weights give the same result for two equal faces.
        let other = put_record(&mut e, 7.0, 7.0, [7.0; 4], [[7.0; 4]; 3]);
        e.call(0x00ce_fb90, &args![square, 1e-6f32, 0u8, 12.0f32, other]);
        near_record(&e, other, (7.0, 12.0), [0.5, 0.5, 0.0], [1.0, 1.0, 2.0]);
        // Mass and thickness have to be above zero.
        assert_eq!(
            e.call(0x00ce_fb90, &args![square, 1e-6f32, 1u8, 0.0f32, out])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ce_fb90, &args![square, 0.0f32, 1u8, 12.0f32, out])
                .u32(),
            1
        );
    }

    // --- shapes ----------------------------------------------------------------------

    /// A shape record of 0x80 bytes: the type at `+0xC` and the float at `+0x10`.
    fn shape_of(e: &mut Engine, kind: u32, radius: f32) -> u32 {
        let shape = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc, kind);
        e.mem.set_f32(shape + 0x10, radius);
        shape
    }

    /// A transform of identity rows and the given translation.
    fn transform_at(e: &mut Engine, translation: Vec4) -> u32 {
        put_vecs(
            e,
            &[
                IDENTITY_ROWS[0],
                IDENTITY_ROWS[1],
                IDENTITY_ROWS[2],
                translation,
            ],
        )
    }

    /// The volume the sphere function computes for a radius (`4.18879` as a float).
    fn sphere_volume(radius: f64) -> f64 {
        f64::from(4.1887903f32) * radius * radius * radius
    }

    #[test]
    fn a_sphere_adds_its_properties_at_the_transform_and_spheres_accumulate_00ceff00() {
        let (mut e, _log) = scene();
        let sphere = shape_of(&mut e, 1, 2.0);
        let above = transform_at(&mut e, [0.0, 0.0, 5.0, 0.0]);
        let below = transform_at(&mut e, [0.0, 0.0, -5.0, 0.0]);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![sphere, above, 0.0f32, out]);
        let volume = sphere_volume(2.0);
        near_record(
            &e,
            out,
            (volume, volume),
            [0.0, 0.0, 5.0],
            [1.6 * volume; 3],
        );
        // A second sphere below it: twice the mass, the centre in the middle and the
        // parallel axis terms (5^2 per unit of mass) on the diagonal entries x and y.
        e.call(0x00ce_ff00, &args![sphere, below, 0.0f32, out]);
        near_record(
            &e,
            out,
            (2.0 * volume, 2.0 * volume),
            [0.0; 3],
            [53.2 * volume, 53.2 * volume, 3.2 * volume],
        );
        // A type of 0, or above 28, or one of the types that do nothing, leaves `out` alone.
        let before = e.mem.bytes(out, 0x50);
        for kind in [0, 13, 29, 0x1234] {
            let other = shape_of(&mut e, kind, 2.0);
            e.call(0x00ce_ff00, &args![other, above, 0.0f32, out]);
        }
        assert_eq!(e.mem.bytes(out, 0x50), before);
    }

    /// `00cb2240` (`hkTransform::setMul`) as a double that copies the first transform and adds
    /// the second one's translation; it logs its arguments.
    fn transform_double(e: &mut Engine, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(TRANSFORM_SET_MUL, move |e, a| {
            sink.borrow_mut().push((TRANSFORM_SET_MUL, a.to_vec()));
            for row in 0..4 {
                copy_vec(e, a[1] + 0x10 * row, a[0] + 0x10 * row);
            }
            let (base, local) = (load_vec(e, a[0] + 0x30), load_vec(e, a[2] + 0x30));
            store_vec(
                e,
                a[0] + 0x30,
                std::array::from_fn(|lane| base[lane] + local[lane]),
            );
            Ret::default()
        });
    }

    #[test]
    fn wrapper_shapes_recurse_with_the_composed_transform_00ceff00() {
        let (mut e, log) = scene();
        transform_double(&mut e, &log);
        let sphere = shape_of(&mut e, 1, 1.0);
        let base = transform_at(&mut e, [0.0, 0.0, 10.0, 0.0]);
        let volume = sphere_volume(1.0);
        // Types 11 (a translation at +0x20), 12 (a transform at +0x20, translation at +0x50),
        // 15 (a transform at +0x30, translation at +0x60) and 27 (no change).
        for (kind, child, translation_at, local) in [
            (11, 0x18, 0x20, [1.0, 0.0, 0.0]),
            (12, 0x18, 0x50, [0.0, 2.0, 0.0]),
            (15, 0x14, 0x60, [0.0, 0.0, 3.0]),
            (27, 0x18, 0x20, [0.0; 3]),
        ] {
            let wrapper = shape_of(&mut e, kind, 0.0);
            e.mem.set_u32(wrapper + child, sphere);
            store_vec(
                &mut e,
                wrapper + translation_at,
                [local[0], local[1], local[2], 0.0],
            );
            let out = e.mem.alloc(0x50);
            e.call(0x00ce_ff00, &args![wrapper, base, 0.0f32, out]);
            let centre = [local[0], local[1], local[2] + 10.0].map(f64::from);
            near_record(&e, out, (volume, volume), centre, [0.4 * volume; 3]);
            let composed = calls(&log, TRANSFORM_SET_MUL);
            match kind {
                11 => assert_eq!(composed.last().map(|words| words[1]), Some(base)),
                12 => assert_eq!(composed.last().map(|words| words[2]), Some(wrapper + 0x20)),
                15 => assert_eq!(composed.last().map(|words| words[2]), Some(wrapper + 0x30)),
                _ => {}
            }
        }
        assert_eq!(calls(&log, TRANSFORM_SET_MUL).len(), 3);
    }

    #[test]
    fn an_array_of_spheres_is_added_sphere_by_sphere_00ceff00() {
        let (mut e, _log) = scene();
        // The sphere constructor (`hkpSphereShape`, `00cd8160`): the type word and the radius.
        e.register_double(SPHERE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 1);
            e.mem.set_u32(a[0] + 0x10, a[1]);
            Ret::default()
        });
        let shape = shape_of(&mut e, 21, 0.0);
        e.mem.set_u32(shape + 0x10, 2);
        store_vec(&mut e, shape + 0x20, [3.0, 0.0, 0.0, 1.0]);
        store_vec(&mut e, shape + 0x30, [-3.0, 0.0, 0.0, 1.0]);
        let origin = transform_at(&mut e, [0.0; 4]);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![shape, origin, 0.0f32, out]);
        let volume = sphere_volume(1.0);
        near_record(
            &e,
            out,
            (2.0 * volume, 2.0 * volume),
            [0.0; 3],
            [0.8 * volume, 18.8 * volume, 18.8 * volume],
        );
    }

    /// A container shape (type 7) whose virtual function `+0x10` answers a container with
    /// the keys `0 ..` `children.len()` and the children behind them. Answers the shape.
    fn container_shape(e: &mut Engine, log: &Log, children: Vec<u32>) -> u32 {
        let container = logging_object(e, log, CONTAINER);
        let keys = children.len() as u32;
        e.register_double(CONTAINER + 8, move |_, _| Ret {
            eax: if keys == 0 { u32::MAX } else { 0 },
            ..Ret::default()
        });
        e.register_double(CONTAINER + 0xc, move |_, a| Ret {
            eax: if a[1] + 1 < keys { a[1] + 1 } else { u32::MAX },
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(CONTAINER + 0x14, move |_, a| {
            sink.borrow_mut().push((CONTAINER + 0x14, a.to_vec()));
            Ret {
                eax: children[a[1] as usize],
                ..Ret::default()
            }
        });
        let shape = logging_object(e, log, SHAPE);
        e.mem.set_u32(shape + 0xc, 7);
        let sink = Rc::clone(log);
        e.register_double(SHAPE + 0x10, move |_, a| {
            sink.borrow_mut().push((SHAPE + 0x10, a.to_vec()));
            Ret {
                eax: container,
                ..Ret::default()
            }
        });
        shape
    }

    #[test]
    fn the_children_of_a_container_are_added_in_key_order_00ceff00() {
        let (mut e, log) = scene();
        let small = shape_of(&mut e, 1, 1.0);
        let large = shape_of(&mut e, 1, 2.0);
        let shape = container_shape(&mut e, &log, vec![small, large]);
        let origin = transform_at(&mut e, [0.0; 4]);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![shape, origin, 0.0f32, out]);
        let (small_volume, large_volume) = (sphere_volume(1.0), sphere_volume(2.0));
        let total = small_volume + large_volume;
        near_record(
            &e,
            out,
            (total, total),
            [0.0; 3],
            [0.4 * (small_volume + 4.0 * large_volume); 3],
        );
        assert_eq!(calls(&log, CONTAINER + 0x14).len(), 2);
    }

    #[test]
    fn boxes_capsules_and_cylinders_are_built_from_the_shape_record_00ceff00() {
        let (mut e, log) = scene();
        let origin = transform_at(&mut e, [0.0; 4]);
        // A box: half extents (0.5, 1, 1.5) plus the radius 0.5.
        let boxed = shape_of(&mut e, 4, 0.5);
        store_vec(&mut e, boxed + 0x20, [0.5, 1.0, 1.5, 0.0]);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![boxed, origin, 0.0f32, out]);
        near_record(&e, out, (24.0, 24.0), [0.0; 3], [50.0, 40.0, 26.0]);
        // A capsule of radius 1 between (0, 0, -1) and (0, 0, 1).
        let capsule = shape_of(&mut e, 5, 1.0);
        store_vec(&mut e, capsule + 0x20, [0.0, 0.0, -1.0, 0.0]);
        store_vec(&mut e, capsule + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let capsule_out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![capsule, origin, 0.0f32, capsule_out]);
        let volume = sphere_volume(1.0) + 2.0 * f64::from(std::f32::consts::PI);
        near_record(
            &e,
            capsule_out,
            (volume, volume),
            [0.0; 3],
            [1.21 * volume, 1.21 * volume, 0.46 * volume],
        );
        // A cylinder of radius 0.5 whose core radius (`00d571a0`) is 1.0: the end points
        // move outwards by 0.5 and the radius is 1.5.
        let sink = Rc::clone(&log);
        e.register_double(CYLINDER_RADIUS, move |_, a| {
            sink.borrow_mut().push((CYLINDER_RADIUS, a.to_vec()));
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        let cylinder = shape_of(&mut e, 2, 0.5);
        store_vec(&mut e, cylinder + 0x20, [0.0, 0.0, -1.0, 0.0]);
        store_vec(&mut e, cylinder + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let cylinder_out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![cylinder, origin, 0.0f32, cylinder_out]);
        assert_eq!(calls(&log, CYLINDER_RADIUS), [[cylinder]]);
        let volume = f64::from(std::f32::consts::PI) * 2.25 * 3.0;
        near_record(
            &e,
            cylinder_out,
            (volume, volume),
            [0.0; 3],
            [1.3125 * volume, 1.3125 * volume, 1.125 * volume],
        );
    }

    #[test]
    fn a_triangle_asks_00c98210_for_its_thickness_and_a_hull_gathers_its_vertices_00ceff00() {
        let (mut e, log) = scene();
        let origin = transform_at(&mut e, [0.0; 4]);
        let sink = Rc::clone(&log);
        e.register_double(TRIANGLE_THICKNESS, move |_, a| {
            sink.borrow_mut().push((TRIANGLE_THICKNESS, a.to_vec()));
            Ret {
                st0: 1e-6,
                ..Ret::default()
            }
        });
        let triangle = shape_of(&mut e, 3, 0.25);
        store_vec(&mut e, triangle + 0x20, [0.0; 4]);
        store_vec(&mut e, triangle + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, triangle + 0x40, [0.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![triangle, origin, 0.75f32, out]);
        assert_eq!(
            calls(&log, TRIANGLE_THICKNESS),
            [[0.25f32.to_bits(), 0.75f32.to_bits()]]
        );
        // The thickness 1e-6 makes a flat triangle of volume (1 * 0.5) * 1e-6.
        assert!((e.mem.f32(out + VOLUME) - 5e-7).abs() < 1e-9);
        assert!((e.mem.f32(out + MASS) - 5e-7).abs() < 1e-9);
        let centre = load_vec(&e, out + CENTRE);
        near(centre[0], 1.0 / 3.0);
        near(centre[1], 1.0 / 3.0);
        // A convex vertices shape: `00d59200` fills an array of its points which the hull
        // builder takes as `{data, size, stride 0x10}` and which is freed afterwards.
        let points = put_vecs(
            &mut e,
            &[
                [0.0; 4],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        e.register_double(GET_ORIGINAL_VERTICES, move |e, a| {
            e.mem.set_u32(a[1], points);
            e.mem.set_u32(a[1] + 4, 4);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let centre_at = put_vec(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let rows_at_hull = put_rows(&mut e, tensor([1.0, 2.0, 3.0], [0.0; 3]));
        hull_doubles(&mut e, &log, centre_at, rows_at_hull);
        let convex = shape_of(&mut e, 6, 0.0);
        let hull_out = e.mem.alloc(0x50);
        e.call(0x00ce_ff00, &args![convex, origin, 0.0f32, hull_out]);
        let builds = calls(&log, HULL_BUILD);
        assert_eq!(builds[0][3..6], [points, 4, 0x10]);
        let freed = calls(&log, ALLOCATOR + 0x10);
        let freed = freed.last().expect("the array is given back");
        assert_eq!(freed[1..], [points, 0x40]);
        near_record(&e, hull_out, (2.0, 2.0), [1.0, 2.0, 3.0], [2.0, 4.0, 6.0]);
    }

    #[test]
    fn the_shape_properties_are_scaled_to_the_mass_and_retried_when_empty_00cf0690() {
        let (mut e, log) = scene();
        let sphere = shape_of(&mut e, 1, 1.0);
        let out = e.mem.alloc(0x50);
        e.call(0x00cf_0690, &args![sphere, 3.0f32, out]);
        let volume = sphere_volume(1.0);
        near_record(&e, out, (volume, 3.0), [0.0; 3], [1.2; 3]);
        // A triangle whose thickness comes out as zero has no volume: the recursion runs a
        // second time with the float at 01013ea4 as its third word, and `out` stays as it was.
        let sink = Rc::clone(&log);
        e.register_double(TRIANGLE_THICKNESS, move |_, a| {
            sink.borrow_mut().push((TRIANGLE_THICKNESS, a.to_vec()));
            Ret::default()
        });
        let triangle = shape_of(&mut e, 3, 0.25);
        store_vec(&mut e, triangle + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, triangle + 0x40, [0.0, 1.0, 0.0, 0.0]);
        let untouched = put_record(&mut e, 7.0, 7.0, [7.0; 4], [[7.0; 4]; 3]);
        e.call(0x00cf_0690, &args![triangle, 3.0f32, untouched]);
        assert_eq!(
            calls(&log, TRIANGLE_THICKNESS),
            [[0.25f32.to_bits(), 0], [0.25f32.to_bits(), 0x3c23_d70a]]
        );
        assert_eq!(e.mem.f32(untouched + MASS), 7.0);
        assert_eq!(e.mem.f32(untouched + VOLUME), 7.0);
        // An empty container is asked twice as well.
        let empty = container_shape(&mut e, &log, Vec::new());
        e.call(0x00cf_0690, &args![empty, 3.0f32, untouched]);
        assert_eq!(calls(&log, SHAPE + 0x10).len(), 2);
        assert_eq!(e.mem.f32(untouched + MASS), 7.0);
    }

    #[test]
    fn the_construction_info_takes_the_mass_centre_and_rows_00cf07a0() {
        let (mut e, _log) = scene();
        let sphere = shape_of(&mut e, 1, 1.0);
        let cinfo = e.mem.alloc(0xa0);
        for offset in [0x50, 0x60, 0x70, 0x80] {
            store_vec(&mut e, cinfo + offset, [7.0; 4]);
        }
        e.call(0x00cf_07a0, &args![sphere, 3.0f32, cinfo]);
        assert_eq!(e.mem.f32(cinfo + 0x90), 3.0);
        assert_eq!(load_vec(&e, cinfo + 0x80), [0.0; 4]);
        near_rows(
            rows_at(&e, cinfo + 0x50),
            [[1.2, 0.0, 0.0], [0.0, 1.2, 0.0], [0.0, 0.0, 1.2]],
        );
    }

    #[test]
    fn a_record_is_copied_with_signalling_floats_quieted_00cf0820() {
        let mut e = setup();
        let rows = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ];
        let source = put_record(&mut e, 1.5, 2.5, [13.0, 14.0, 15.0, 16.0], rows);
        let copy = e.mem.alloc(0x50);
        e.call(0x00cf_0820, &args![copy, source]);
        assert_eq!(e.mem.bytes(copy, 0x50), e.mem.bytes(source, 0x50));
        e.mem.set_u32(source + VOLUME, 0x7f80_0001);
        e.mem.set_u32(source + MASS, 0xff80_0002);
        e.call(0x00cf_0820, &args![copy, source]);
        assert_eq!(e.mem.u32(copy + VOLUME), 0x7fc0_0001);
        assert_eq!(e.mem.u32(copy + MASS), 0xffc0_0002);
    }
}
