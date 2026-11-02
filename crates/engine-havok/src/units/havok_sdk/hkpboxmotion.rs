//! `Havok SDK/hkpboxmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`).
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// `hkpBoxMotion`'s vtable (the vtable `010c6dc4` is that class; RTTI read at the vtable)
/// and the `float` constant `1.0` stored in four lanes.
const BOX_MOTION_VTABLE: u32 = 0x010c_6dc4;
const FLOAT_ONE: u32 = 0x0101_2068;

// Translated from 00d1e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the box motion (type 3): the motion base constructor with a trailing 0,
/// the vtable, motion type 3 at +8 and the float at `01012068` (1.0) in all four lanes of
/// the vector at +0xC0. Returns `this`.
pub fn fn_00d1e690(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    let one = e.mem.u32(FLOAT_ONE);
    e.mem.set_u32(this.addr(), BOX_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 3);
    for lane in 0..4 {
        e.mem.set_u32(this.addr() + 0xc0 + 4 * lane, one);
    }
    this
}

// `hkpBoxMotion` (the Xbox PDB has no record of the Havok classes, so the field names below
// are unconfirmed): the rotation is three vectors at `+0x10`, `+0x20` and `+0x30`, the centre
// of mass at `+0x60`, the inverse inertia diagonal in lanes 0 to 2 of the vector at `+0xC0`
// with the inverse mass in lane 3 (`+0xCC`), the linear velocity at `+0xD0` and the angular
// velocity at `+0xE0`. SSE code computes lane by lane in `f32`; the x87 code (`FDIV`) divides
// in double precision and rounds at each `float` store.
use super::hkpsphereshape::{
    load_vec, maxps, movhlps, movlhps, shufps, store_vec, unpckhps, unpcklps, zip_with, Vec4,
};

const INVERSE_INERTIA: u32 = 0xc0;
const LINEAR_VELOCITY: u32 = 0xd0;
const ANGULAR_VELOCITY: u32 = 0xe0;
const ROTATION: u32 = 0x10;
const CENTRE_OF_MASS: u32 = 0x60;
/// The `float` the local inertia clamps the inverse inertia to (`MOVSS`, splat), the 16-byte
/// vector the world inertia clamps to, `hkMatrix3::setMulInverse` (`00cd5610`, `this` the
/// output, two words) and `hkMatrix3::changeBasis` (`00cd5880`, `this` the matrix, one word).
const LOCAL_CLAMP: u32 = 0x0104_da4c;
const WORLD_CLAMP: u32 = 0x010c_7200;
const SET_MUL_INVERSE: u32 = 0x00cd_5610;
const CHANGE_BASIS: u32 = 0x00cd_5880;

pub(crate) fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

pub(crate) fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

pub(crate) fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

/// One lane in all four (`SHUFPS v, v, 0x55 * lane`).
pub(crate) fn splat(v: Vec4, lane: u8) -> Vec4 {
    shufps(v, v, 0x55 * lane)
}

/// `1.0 / x` on the x87 stack: `x` is loaded (a signalling NaN is quieted), divided in double
/// precision and rounded to a `float`.
fn reciprocal(x: f32) -> f32 {
    (1.0f64 / f64::from(x87_quiet(x))) as f32
}

/// The cross product `r x v` the way the code forms it: `v.zxy * r.yzx - v.yzx * r.zxy`.
pub(crate) fn cross(r: Vec4, v: Vec4) -> Vec4 {
    let minus = mul(shufps(v, v, 0xc9), shufps(r, r, 0xd2));
    let plus = mul(shufps(v, v, 0xd2), shufps(r, r, 0xc9));
    sub(plus, minus)
}

/// `v` times the matrix whose rows are `r0`, `r1`, `r2`, the `UNPCK`/`MOVLHPS`/`SHUFPS`
/// chain: `(v.y * column_y + v.x * column_x) + column_z * v.z`.
pub(crate) fn rows_times(v: Vec4, r0: Vec4, r1: Vec4, r2: Vec4) -> Vec4 {
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let column_x = movlhps(low, r2);
    let column_y = shufps(movhlps(column_x, low), r2, 0xd4);
    let column_z = shufps(high, r2, 0xe4);
    let part_y = mul(splat(v, 1), column_y);
    let part_x = mul(splat(v, 0), column_x);
    let part_z = mul(column_z, splat(v, 2));
    add(add(part_y, part_x), part_z)
}

/// `s.y * r1 + s.x * r0 + s.z * r2` (the weighted sum of the rows), `+ old`, with the SSE
/// NaN rules (each `MULPS` with the splatted lane as the destination).
fn rows_weighted(s: Vec4, r0: Vec4, r1: Vec4, r2: Vec4, old: Vec4) -> Vec4 {
    use crate::sse::{addps, mulps};
    let part_y = mulps(splat(s, 1), r1);
    let part_x = mulps(splat(s, 0), r0);
    let part_z = mulps(splat(s, 2), r2);
    addps(addps(addps(part_y, part_x), part_z), old)
}

/// [`cross`] with the SSE NaN rules (`MULPS`, `SUBPS`, the destination first).
fn cross_sse(r: Vec4, v: Vec4) -> Vec4 {
    use crate::sse::{mulps, subps};
    let minus = mulps(shufps(v, v, 0xc9), shufps(r, r, 0xd2));
    let plus = mulps(shufps(v, v, 0xd2), shufps(r, r, 0xc9));
    subps(plus, minus)
}

/// [`rows_times`] with the SSE NaN rules. `z_lane_first`: the `MULPS` of the `z` part has the
/// splatted `v.z` as its destination (else the column, as in [`rows_times`]).
fn rows_times_sse(v: Vec4, r0: Vec4, r1: Vec4, r2: Vec4, z_lane_first: bool) -> Vec4 {
    use crate::sse::{addps, mulps};
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let column_x = movlhps(low, r2);
    let column_y = shufps(movhlps(column_x, low), r2, 0xd4);
    let column_z = shufps(high, r2, 0xe4);
    let part_y = mulps(splat(v, 1), column_y);
    let part_x = mulps(splat(v, 0), column_x);
    let part_z = if z_lane_first {
        mulps(splat(v, 2), column_z)
    } else {
        mulps(column_z, splat(v, 2))
    };
    addps(addps(part_y, part_x), part_z)
}

/// `z + (y + x)` with `ADDSS` (the higher lane the destination, the SSE NaN rules).
fn sum3_sse(v: Vec4) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

/// The three rotation vectors of the motion.
fn rotation(e: &Engine, motion: u32) -> (Vec4, Vec4, Vec4) {
    (
        load_vec(e, motion + ROTATION),
        load_vec(e, motion + ROTATION + 0x10),
        load_vec(e, motion + ROTATION + 0x20),
    )
}

// Translated from 00d1e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::setMass` (engine map): calls the motion's slot `0x10` (the inverse-mass
/// setter) with `1.0 / mass`, divided on the x87 stack and rounded to a `float`.
pub fn hkp_box_motion_set_mass(e: &mut Engine, this: Ptr, mass: f32) {
    let inverse = reciprocal(mass);
    e.vcall(this.addr(), 0x10, &args![inverse]);
}

// Translated from 00d1e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::getInertiaLocal` (engine map): the diagonal matrix (three vectors at
/// `out`, zeroed first) of `1.0 / max(clamp, inverse inertia)` per axis, the clamp being the
/// `float` at `0104da4c` (`MAXPS` keeps the inverse inertia lane when the clamp is not
/// greater, and when either is a NaN).
pub fn hkp_box_motion_get_inertia_local(e: &mut Engine, this: Ptr, out: Ptr) {
    let motion = this.addr();
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let clamp: f32 = e.global(LOCAL_CLAMP);
    let clamped = zip_with([clamp; 4], inverse, maxps);
    write_diagonal(e, out.addr(), [clamped[0], clamped[1], clamped[2]]);
}

/// Zeroes the matrix at `out` (three vectors) and stores `1 / lane` on its diagonal
/// (`+0`, `+0x14`, `+0x28`).
fn write_diagonal(e: &mut Engine, out: u32, lanes: [f32; 3]) {
    for row in 0..3 {
        store_vec(e, out + 0x10 * row, [0.0; 4]);
    }
    e.mem.set_f32(out, reciprocal(lanes[0]));
    e.mem.set_f32(out + 0x14, reciprocal(lanes[1]));
    e.mem.set_f32(out + 0x28, reciprocal(lanes[2]));
}

// Translated from 00d1e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::setInertiaLocal` (engine map): stores `1.0 / m[0]`, `1.0 / m[5]` and
/// `1.0 / m[10]` (the diagonal of the matrix at `inertia`) in lanes 0 to 2 of the inverse
/// inertia vector; lane 3 (the inverse mass) is kept.
pub fn hkp_box_motion_set_inertia_local(e: &mut Engine, this: Ptr, inertia: Ptr) {
    let motion = this.addr();
    let matrix = inertia.addr();
    let inverse_mass = e.mem.u32(motion + INVERSE_INERTIA + 0xc);
    let x = reciprocal(e.mem.f32(matrix));
    let y = reciprocal(e.mem.f32(matrix + 0x14));
    let z = reciprocal(e.mem.f32(matrix + 0x28));
    e.mem.set_f32(motion + INVERSE_INERTIA, x);
    e.mem.set_f32(motion + INVERSE_INERTIA + 4, y);
    e.mem.set_f32(motion + INVERSE_INERTIA + 8, z);
    e.mem.set_u32(motion + INVERSE_INERTIA + 0xc, inverse_mass);
}

// Translated from 00d1e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::getInertiaInvLocal` (engine map): the matrix at `out` is zero except its
/// diagonal (`+0`, `+0x14`, `+0x28`), which copies lanes 0 to 2 of the inverse inertia
/// (through the x87 stack: a signalling NaN is quieted).
pub fn hkp_box_motion_get_inertia_inv_local(e: &mut Engine, this: Ptr, out: Ptr) {
    let motion = this.addr();
    let out = out.addr();
    let diagonal =
        [0u32, 4, 8].map(|lane| x87_quiet_bits(e.mem.u32(motion + INVERSE_INERTIA + lane)));
    for row in 0..3 {
        store_vec(e, out + 0x10 * row, [0.0; 4]);
    }
    e.mem.set_u32(out, diagonal[0]);
    e.mem.set_u32(out + 0x14, diagonal[1]);
    e.mem.set_u32(out + 0x28, diagonal[2]);
}

// Translated from 00d1e810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::setInertiaInvLocal` (engine map): copies the diagonal of the matrix at
/// `inertia` (`+0`, `+0x14`, `+0x28`) into lanes 0 to 2 of the inverse inertia, bit for bit;
/// lane 3 (the inverse mass) is kept.
pub fn hkp_box_motion_set_inertia_inv_local(e: &mut Engine, this: Ptr, inertia: Ptr) {
    let motion = this.addr();
    let matrix = inertia.addr();
    let x = e.mem.u32(matrix);
    let y = e.mem.u32(matrix + 0x14);
    let z = e.mem.u32(matrix + 0x28);
    let inverse_mass = e.mem.u32(motion + INVERSE_INERTIA + 0xc);
    e.mem.set_u32(motion + INVERSE_INERTIA, x);
    e.mem.set_u32(motion + INVERSE_INERTIA + 4, y);
    e.mem.set_u32(motion + INVERSE_INERTIA + 8, z);
    e.mem.set_u32(motion + INVERSE_INERTIA + 0xc, inverse_mass);
}

// Translated from 00d1e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::getInertiaInvWorld` (engine map): scales the three rotation vectors by
/// the inverse inertia lanes 0 to 2 (a local matrix of `splat(inverse[i]) * rotation[i]`) and
/// gives `hkMatrix3::setMulInverse` (`00cd5610`, `this` = `out`) that matrix and the
/// rotation.
pub fn hkp_box_motion_get_inertia_inv_world(e: &mut Engine, this: Ptr, out: Ptr) {
    let motion = this.addr();
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let (r0, r1, r2) = rotation(e, motion);
    let scaled = [
        mul(splat(inverse, 0), r0),
        mul(splat(inverse, 1), r1),
        mul(splat(inverse, 2), r2),
    ];
    e.with_stack(0x30, |e, local| {
        for (row, vector) in scaled.into_iter().enumerate() {
            store_vec(e, local.addr() + 0x10 * row as u32, vector);
        }
        e.call(SET_MUL_INVERSE, &args![out, local, motion + ROTATION]);
    });
}

// Translated from 00d1e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::getInertiaWorld` (engine map): like `getInertiaLocal` but clamping with
/// the vector at `010c7200` as the second `MAXPS` operand, then `hkMatrix3::changeBasis`
/// (`00cd5880`, `this` = `out`) with the rotation.
pub fn hkp_box_motion_get_inertia_world(e: &mut Engine, this: Ptr, out: Ptr) {
    let motion = this.addr();
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let clamp = load_vec(e, WORLD_CLAMP);
    let clamped = zip_with(inverse, clamp, maxps);
    write_diagonal(e, out.addr(), [clamped[0], clamped[1], clamped[2]]);
    e.call(CHANGE_BASIS, &args![out, motion + ROTATION]);
}

// Translated from 00d1e920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::applyForce` (engine map): `delta_time` and `force` (a vector); the linear
/// velocity (`+0xD0`) grows by `inverse_mass * (delta_time * force)` per lane.
pub fn hkp_box_motion_apply_force(e: &mut Engine, this: Ptr, delta_time: f32, force: Ptr) {
    let motion = this.addr();
    let force = load_vec(e, force.addr());
    let scaled = crate::sse::mulps([delta_time; 4], force);
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let change = crate::sse::mulps(splat(inverse, 3), scaled);
    let velocity = load_vec(e, motion + LINEAR_VELOCITY);
    store_vec(
        e,
        motion + LINEAR_VELOCITY,
        crate::sse::addps(change, velocity),
    );
}

// Translated from 00d1e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::applyForce_ov2` (engine map): calls the motion's slot `0x50` (apply a
/// force at a point) with a local vector `delta_time * force` and `point`.
pub fn hkp_box_motion_apply_force_ov2(
    e: &mut Engine,
    this: Ptr,
    delta_time: f32,
    force: Ptr,
    point: Ptr,
) {
    let force = load_vec(e, force.addr());
    let scaled = mul([delta_time; 4], force);
    e.with_stack(0x10, |e, local| {
        store_vec(e, local.addr(), scaled);
        e.vcall(this.addr(), 0x50, &args![local, point]);
    });
}

// Translated from 00d1e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::applyTorque` (engine map): calls the motion's slot `0x54` with a local
/// vector `delta_time * torque`.
pub fn hkp_box_motion_apply_torque(e: &mut Engine, this: Ptr, delta_time: f32, torque: Ptr) {
    let torque = load_vec(e, torque.addr());
    let scaled = mul([delta_time; 4], torque);
    e.with_stack(0x10, |e, local| {
        store_vec(e, local.addr(), scaled);
        e.vcall(this.addr(), 0x54, &args![local]);
    });
}

// Translated from 00d1e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::applyPointImpulse` (engine map): the linear velocity grows by
/// `inverse_mass * impulse`; with `r = point - centre of mass` the angular impulse
/// `r x impulse` is carried through the rotation, scaled by the inverse inertia and carried
/// back, and added to the angular velocity (`+0xE0`).
pub fn hkp_box_motion_apply_point_impulse(e: &mut Engine, this: Ptr, impulse: Ptr, point: Ptr) {
    use crate::sse::{addps, mulps, subps};
    let motion = this.addr();
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let impulse = load_vec(e, impulse.addr());
    let linear = addps(
        mulps(splat(inverse, 3), impulse),
        load_vec(e, motion + LINEAR_VELOCITY),
    );
    store_vec(e, motion + LINEAR_VELOCITY, linear);
    let arm = subps(
        load_vec(e, point.addr()),
        load_vec(e, motion + CENTRE_OF_MASS),
    );
    let torque = cross_sse(arm, impulse);
    let (r0, r1, r2) = rotation(e, motion);
    let local = mulps(rows_times_sse(torque, r0, r1, r2, false), inverse);
    let angular = rows_weighted(local, r0, r1, r2, load_vec(e, motion + ANGULAR_VELOCITY));
    store_vec(e, motion + ANGULAR_VELOCITY, angular);
}

// Translated from 00d1eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::applyAngularImpulse` (engine map): the angular impulse is carried through
/// the rotation, scaled by the inverse inertia, carried back and added to the angular
/// velocity (`+0xE0`).
pub fn hkp_box_motion_apply_angular_impulse(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let motion = this.addr();
    let (r0, r1, r2) = rotation(e, motion);
    let impulse = load_vec(e, impulse.addr());
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let local = crate::sse::mulps(rows_times_sse(impulse, r0, r1, r2, true), inverse);
    let angular = rows_weighted(local, r0, r1, r2, load_vec(e, motion + ANGULAR_VELOCITY));
    store_vec(e, motion + ANGULAR_VELOCITY, angular);
}

// Translated from 00d1eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpBoxMotion::getProjectedPointVelocity` (engine map): with `r = point - centre of mass`
/// and `c = r x normal`, writes to `velocity` the sum of `linear velocity . normal` and
/// `angular velocity . c` (each summed `z + (y + x)`, the two sums added, through the x87
/// stack: a signalling NaN is quieted) and to `mass_factor` the float at `+0xCC` plus the
/// sum of `inverse inertia * c' * c'` (`c'` being `c` carried through the rotation), added in
/// double precision and rounded to a `float`.
pub fn hkp_box_motion_get_projected_point_velocity(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    normal: Ptr,
    velocity: Ptr,
    mass_factor: Ptr,
) {
    use crate::sse::{mulps, subps};
    let motion = this.addr();
    let centre = load_vec(e, motion + CENTRE_OF_MASS);
    let arm = subps(load_vec(e, point.addr()), centre);
    let normal = load_vec(e, normal.addr());
    let moment = cross_sse(arm, normal);
    let linear = mulps(load_vec(e, motion + LINEAR_VELOCITY), normal);
    let angular = mulps(load_vec(e, motion + ANGULAR_VELOCITY), moment);
    let total = crate::sse::add(sum3_sse(angular), sum3_sse(linear));
    e.mem
        .set_u32(velocity.addr(), x87_quiet_bits(total.to_bits()));
    let (r0, r1, r2) = rotation(e, motion);
    let carried = rows_times_sse(moment, r0, r1, r2, false);
    let inverse = load_vec(e, motion + INVERSE_INERTIA);
    let weighted = mulps(mulps(inverse, carried), carried);
    let sum = sum3_sse(weighted);
    // `FLD [+0xCC]`, `FADD` the scratch float (the x87 choice of NaN).
    let base = x87_load32(e.mem.u32(motion + INVERSE_INERTIA + 0xc));
    let mass = x87_add(base, x87_m32(sum.to_bits()));
    e.mem.set_u32(mass_factor.addr(), x87_store32(mass));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_e670, hkp_box_motion_set_mass(Ptr, f32)),
        entry!(0x00d1_e690, fn_00d1e690(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d1_e6f0, hkp_box_motion_get_inertia_local(Ptr, Ptr)),
        entry!(0x00d1_e750, hkp_box_motion_set_inertia_local(Ptr, Ptr)),
        entry!(0x00d1_e7b0, hkp_box_motion_get_inertia_inv_local(Ptr, Ptr)),
        entry!(0x00d1_e810, hkp_box_motion_set_inertia_inv_local(Ptr, Ptr)),
        entry!(0x00d1_e850, hkp_box_motion_get_inertia_inv_world(Ptr, Ptr)),
        entry!(0x00d1_e8c0, hkp_box_motion_get_inertia_world(Ptr, Ptr)),
        entry!(0x00d1_e920, hkp_box_motion_apply_force(Ptr, f32, Ptr)),
        entry!(
            0x00d1_e970,
            hkp_box_motion_apply_force_ov2(Ptr, f32, Ptr, Ptr)
        ),
        entry!(0x00d1_e9b0, hkp_box_motion_apply_torque(Ptr, f32, Ptr)),
        entry!(
            0x00d1_e9f0,
            hkp_box_motion_apply_point_impulse(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d1_eaf0, hkp_box_motion_apply_angular_impulse(Ptr, Ptr)),
        entry!(
            0x00d1_eba0,
            hkp_box_motion_get_projected_point_velocity(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn construct_fills_the_box_motion_fields() {
        let mut e = test_engine();
        e.set_global(FLOAT_ONE, 1.0f32);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00d1_e690, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6, 0]]);
        assert_eq!(e.mem.u32(this), BOX_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 3);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0xc0 + 4 * lane), 1.0);
        }
    }
}

#[cfg(test)]
mod box_motion_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_INVERSE_MASS: u32 = 0x0900_0001;
    const SLOT_FORCE_AT_POINT: u32 = 0x0900_0002;
    const SLOT_TORQUE: u32 = 0x0900_0003;

    fn bits(v: Vec4) -> [u32; 4] {
        v.map(f32::to_bits)
    }

    /// A motion with the vtable slots 0x10, 0x50 and 0x54 pointing at logging doubles:
    /// rotation rows `(1, 2, 0, 1)`, `(0, 1, 3, 2)`, `(2, 0, 1, 3)`, centre of mass
    /// `(1, 1, 1, 0)`, inverse inertia `(2, 3, 4)` with inverse mass 5, linear velocity
    /// `(1, 2, 3, 4)` and angular velocity `(5, 6, 7, 8)`.
    fn motion(e: &mut Engine, log: &Log) -> u32 {
        e.map(0x0100_0000, 0x0030_0000);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(table + 0x10, SLOT_INVERSE_MASS);
        e.mem.set_u32(table + 0x50, SLOT_FORCE_AT_POINT);
        e.mem.set_u32(table + 0x54, SLOT_TORQUE);
        for address in [SLOT_INVERSE_MASS, SLOT_FORCE_AT_POINT, SLOT_TORQUE] {
            let sink = Rc::clone(log);
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                if address != SLOT_INVERSE_MASS {
                    words.extend(bits(load_vec(e, a[1])));
                }
                sink.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        let motion = e.mem.alloc(0x100);
        e.mem.set_u32(motion, table);
        store_vec(e, motion + 0x10, [1.0, 2.0, 0.0, 1.0]);
        store_vec(e, motion + 0x20, [0.0, 1.0, 3.0, 2.0]);
        store_vec(e, motion + 0x30, [2.0, 0.0, 1.0, 3.0]);
        store_vec(e, motion + 0x60, [1.0, 1.0, 1.0, 0.0]);
        store_vec(e, motion + 0xc0, [2.0, 3.0, 4.0, 5.0]);
        store_vec(e, motion + 0xd0, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, motion + 0xe0, [5.0, 6.0, 7.0, 8.0]);
        motion
    }

    fn vector(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, v);
        address
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn set_mass_hands_the_reciprocal_to_slot_0x10() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        e.call(0x00d1_e670, &args![m, 4.0f32]);
        e.call(0x00d1_e670, &args![m, 0.0f32]);
        e.call(0x00d1_e670, &args![m, 3.0f32]);
        assert_eq!(
            calls_to(&log, SLOT_INVERSE_MASS),
            vec![
                vec![m, 0.25f32.to_bits()],
                vec![m, f32::INFINITY.to_bits()],
                vec![m, ((1.0f64 / 3.0) as f32).to_bits()],
            ]
        );
    }

    #[test]
    fn get_inertia_local_inverts_the_clamped_diagonal() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        e.set_global(LOCAL_CLAMP, 0.001f32);
        store_vec(&mut e, m + 0xc0, [0.5, 0.25, 0.0, 9.0]);
        let out = e.mem.alloc(0x30);
        for offset in 0..12 {
            e.mem.set_u32(out + 4 * offset, 0x1234_5678);
        }
        e.call(0x00d1_e6f0, &args![m, out]);
        let diagonal = [e.mem.f32(out), e.mem.f32(out + 0x14), e.mem.f32(out + 0x28)];
        assert_eq!(
            diagonal,
            [2.0, 4.0, ((1.0f64 / f64::from(0.001f32)) as f32)]
        );
        for offset in [4u32, 8, 0xc, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x2c] {
            assert_eq!(e.mem.u32(out + offset), 0);
        }
    }

    #[test]
    fn get_inertia_local_keeps_a_nan_lane_of_the_inverse_inertia() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        e.set_global(LOCAL_CLAMP, 0.001f32);
        // MAXPS returns its second operand (the inverse inertia) when it is a NaN.
        e.mem.set_u32(m + 0xc0, 0x7f80_0001);
        let out = e.mem.alloc(0x30);
        e.call(0x00d1_e6f0, &args![m, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
    }

    #[test]
    fn set_inertia_local_stores_reciprocals_and_keeps_the_inverse_mass() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let inertia = e.mem.alloc(0x30);
        e.mem.set_f32(inertia, 2.0);
        e.mem.set_f32(inertia + 0x14, 4.0);
        e.mem.set_f32(inertia + 0x28, 8.0);
        e.mem.set_u32(m + 0xcc, 0x7f80_0001);
        e.call(0x00d1_e750, &args![m, inertia]);
        assert_eq!(load_vec(&e, m + 0xc0)[..3], [0.5, 0.25, 0.125]);
        assert_eq!(e.mem.u32(m + 0xcc), 0x7f80_0001);
    }

    #[test]
    fn get_inertia_inv_local_copies_the_diagonal_and_quiets_nans() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        e.mem.set_u32(m + 0xc4, 0x7f80_0001);
        let out = e.mem.alloc(0x30);
        for offset in 0..12 {
            e.mem.set_u32(out + 4 * offset, 0x1234_5678);
        }
        e.call(0x00d1_e7b0, &args![m, out]);
        assert_eq!(e.mem.f32(out), 2.0);
        assert_eq!(e.mem.u32(out + 0x14), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 0x28), 4.0);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 0x2c), 0);
    }

    #[test]
    fn set_inertia_inv_local_copies_bits_and_keeps_the_inverse_mass() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let inertia = e.mem.alloc(0x30);
        e.mem.set_u32(inertia, 0x7f80_0001);
        e.mem.set_f32(inertia + 0x14, 6.0);
        e.mem.set_f32(inertia + 0x28, 7.0);
        e.call(0x00d1_e810, &args![m, inertia]);
        assert_eq!(e.mem.u32(m + 0xc0), 0x7f80_0001);
        assert_eq!(e.mem.f32(m + 0xc4), 6.0);
        assert_eq!(e.mem.f32(m + 0xc8), 7.0);
        assert_eq!(e.mem.f32(m + 0xcc), 5.0);
    }

    #[test]
    fn get_inertia_inv_world_scales_the_rotation_rows_for_set_mul_inverse() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let sink = Rc::clone(&log);
        e.register_double(SET_MUL_INVERSE, move |e, a| {
            let mut words = a.to_vec();
            for row in 0..3 {
                words.extend(bits(load_vec(e, a[1] + 0x10 * row)));
            }
            sink.borrow_mut().push((SET_MUL_INVERSE, words));
            Ret::default()
        });
        let out = e.mem.alloc(0x30);
        e.call(0x00d1_e850, &args![m, out]);
        let call = &calls_to(&log, SET_MUL_INVERSE)[0];
        assert_eq!(call[0], out);
        assert_eq!(call[2], m + 0x10);
        assert_eq!(call[3..7], bits([2.0, 4.0, 0.0, 2.0]));
        assert_eq!(call[7..11], bits([0.0, 3.0, 9.0, 6.0]));
        assert_eq!(call[11..15], bits([8.0, 0.0, 4.0, 12.0]));
    }

    #[test]
    fn get_inertia_world_inverts_the_clamped_diagonal_then_changes_basis() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        store_vec(&mut e, WORLD_CLAMP, [0.5, 0.5, 8.0, 0.5]);
        let sink = Rc::clone(&log);
        e.register_double(CHANGE_BASIS, move |_, a| {
            sink.borrow_mut().push((CHANGE_BASIS, a.to_vec()));
            Ret::default()
        });
        let out = e.mem.alloc(0x30);
        e.call(0x00d1_e8c0, &args![m, out]);
        // Inverse inertia (2, 3, 4): the first two stay, the third is raised to 8.
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 0x14), e.mem.f32(out + 0x28)],
            [0.5, (1.0f64 / 3.0) as f32, 0.125]
        );
        assert_eq!(calls_to(&log, CHANGE_BASIS), vec![vec![out, m + 0x10]]);
    }

    #[test]
    fn apply_force_adds_inverse_mass_times_the_scaled_force() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let force = vector(&mut e, [2.0, 4.0, 6.0, 8.0]);
        e.call(0x00d1_e920, &args![m, 0.5f32, force]);
        assert_eq!(load_vec(&e, m + 0xd0), [6.0, 12.0, 18.0, 24.0]);
    }

    #[test]
    fn apply_force_at_a_point_passes_the_scaled_force_to_slot_0x50() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let force = vector(&mut e, [2.0, 4.0, 6.0, 8.0]);
        e.call(0x00d1_e970, &args![m, 0.5f32, force, 0x1234u32]);
        let calls = calls_to(&log, SLOT_FORCE_AT_POINT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], m);
        assert_eq!(calls[0][2], 0x1234);
        assert_eq!(calls[0][3..7], bits([1.0, 2.0, 3.0, 4.0]));
    }

    #[test]
    fn apply_torque_passes_the_scaled_torque_to_slot_0x54() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let torque = vector(&mut e, [2.0, 4.0, 6.0, 8.0]);
        e.call(0x00d1_e9b0, &args![m, 2.0f32, torque]);
        let calls = calls_to(&log, SLOT_TORQUE);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][2..6], bits([4.0, 8.0, 12.0, 16.0]));
    }

    /// The rows of the test rotation.
    const ROWS: [Vec4; 3] = [
        [1.0, 2.0, 0.0, 1.0],
        [0.0, 1.0, 3.0, 2.0],
        [2.0, 0.0, 1.0, 3.0],
    ];

    /// The angular velocity change the decompiled arithmetic gives for the torque `c`:
    /// each inverse-inertia-scaled row dot, carried back by the rows.
    fn expected_angular(c: Vec4, old: Vec4) -> Vec4 {
        let scaled = [0usize, 1, 2].map(|row| {
            let r = ROWS[row];
            (c[1] * r[1] + c[0] * r[0] + r[2] * c[2]) * [2.0, 3.0, 4.0][row]
        });
        std::array::from_fn(|lane| {
            scaled[1] * ROWS[1][lane]
                + scaled[0] * ROWS[0][lane]
                + scaled[2] * ROWS[2][lane]
                + old[lane]
        })
    }

    #[test]
    fn apply_point_impulse_changes_both_velocities() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let impulse = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let point = vector(&mut e, [2.0, 3.0, 1.0, 0.0]);
        e.call(0x00d1_e9f0, &args![m, impulse, point]);
        assert_eq!(load_vec(&e, m + 0xd0), [6.0, 12.0, 18.0, 4.0]);
        // r = point - centre = (1, 2, 0); r x impulse = (6, -3, 0).
        let torque = [6.0, -3.0, 0.0, 0.0];
        assert_eq!(
            load_vec(&e, m + 0xe0),
            expected_angular(torque, [5.0, 6.0, 7.0, 8.0])
        );
    }

    #[test]
    fn apply_angular_impulse_changes_the_angular_velocity_only() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let impulse = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00d1_eaf0, &args![m, impulse]);
        assert_eq!(load_vec(&e, m + 0xd0), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            load_vec(&e, m + 0xe0),
            expected_angular([1.0, 2.0, 3.0, 0.0], [5.0, 6.0, 7.0, 8.0])
        );
    }

    #[test]
    fn get_projected_point_velocity_sums_the_velocities_and_the_mass_factor() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let m = motion(&mut e, &log);
        let point = vector(&mut e, [2.0, 3.0, 1.0, 0.0]);
        let normal = vector(&mut e, [0.0, 1.0, 2.0, 0.0]);
        let velocity = e.mem.alloc(8);
        let factor = e.mem.alloc(8);
        e.call(0x00d1_eba0, &args![m, point, normal, velocity, factor]);
        // r = (1, 2, 0); c = r x normal = (4, -2, 1).
        let c = [4.0f32, -2.0, 1.0];
        let angular = 5.0 * c[0] + 6.0 * c[1] + 7.0 * c[2];
        let linear = 1.0 * 0.0 + 2.0 * 1.0 + 3.0 * 2.0;
        assert_eq!(e.mem.f32(velocity), angular + linear);
        // c carried through the rows, then inverse inertia times its square, plus lane 3.
        let carried: Vec<f32> = (0..3)
            .map(|row| c[1] * ROWS[row][1] + c[0] * ROWS[row][0] + ROWS[row][2] * c[2])
            .collect();
        let inertia = [2.0, 3.0, 4.0];
        let factor_value = 5.0
            + inertia[2] * carried[2] * carried[2]
            + inertia[1] * carried[1] * carried[1]
            + inertia[0] * carried[0] * carried[0];
        assert_eq!(e.mem.f32(factor), factor_value);
    }
}
