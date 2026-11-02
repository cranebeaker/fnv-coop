//! `Havok SDK/hkpspheremotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpSphereMotion` (the class of the vtable these functions fill; names from the engine map):
//! the centre of mass is the vector at `+0x60`, the inverse inertia is one value in lanes 0 to
//! 2 of the vector at `+0xC0` with the inverse mass in lane 3 (`+0xCC`), the linear velocity is
//! at `+0xD0` and the angular velocity at `+0xE0`. A 3x3 inertia matrix is three vectors of
//! sixteen bytes. SSE code computes lane by lane in `f32`, in the exe's operand order (`MULPS
//! a, b` is `mul(a, b)`); the x87 code computes in `f64` and rounds at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpboxmotion::{add, mul, splat};
use super::hkpsphereshape::{load_vec, shufps, store_vec, Vec4};

/// The `double` the local and world inertia compare the inverse mass against (`FCOM`) and the
/// `float` that replaces it when it is smaller.
const INERTIA_FLOOR: u32 = 0x010c_9300;
const INERTIA_FLOOR_REPLACEMENT: u32 = 0x0104_da4c;

const CENTRE_OF_MASS: u32 = 0x60;
const INVERSE_INERTIA: u32 = 0xc0;
const INVERSE_MASS: u32 = 0xcc;
const LINEAR_VELOCITY: u32 = 0xd0;
const ANGULAR_VELOCITY: u32 = 0xe0;

/// Vtable slot of the point-force function `applyForce_ov2` calls, and of the torque function
/// `applyTorque` calls (byte offsets, `this` in ECX).
const APPLY_FORCE_SLOT: u32 = 0x50;
const APPLY_TORQUE_SLOT: u32 = 0x54;

/// Clears the twelve floats of a 3x3 matrix (three `MOVAPS` of zero) and writes `diagonal`
/// at the three diagonal places (`+0`, `+0x14`, `+0x28`).
fn write_diagonal(e: &mut Engine, out: u32, diagonal: u32) {
    for lane in 0..12 {
        e.mem.set_u32(out + 4 * lane, 0);
    }
    for place in [0, 0x14, 0x28] {
        e.mem.set_u32(out + place, diagonal);
    }
}

/// The inertia of a sphere motion as the local and world getters build it: the reciprocal of
/// the inverse inertia at `+0xC0` (a value below the floor replaced by the `float` at
/// `0104da4c`), through the x87 stack and rounded to a `float`.
fn inertia_diagonal(e: &Engine, this: u32) -> u32 {
    let mut value = x87_quiet(e.mem.f32(this + INVERSE_INERTIA));
    if f64::from(value) < e.mem.f64(INERTIA_FLOOR) {
        value = x87_quiet(e.mem.f32(INERTIA_FLOOR_REPLACEMENT));
    }
    x87_quiet((1.0 / f64::from(value)) as f32).to_bits()
}

// Translated from 00d1e1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::getInertiaLocal` (Xbox PDB): writes a diagonal matrix into `out` whose
/// diagonal is `1 / max(inverse inertia, floor)` (`inertia_diagonal`; a NaN stays).
pub fn hkp_sphere_motion_get_inertia_local(e: &mut Engine, this: Ptr, out: Ptr) {
    let diagonal = inertia_diagonal(e, this.addr());
    write_diagonal(e, out.addr(), diagonal);
}

// Translated from 00d1e230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::getInertiaWorld` (Xbox PDB): the same instructions as
/// `getInertiaLocal` (a sphere's inertia is the same in every frame).
pub fn hkp_sphere_motion_get_inertia_world(e: &mut Engine, this: Ptr, out: Ptr) {
    let diagonal = inertia_diagonal(e, this.addr());
    write_diagonal(e, out.addr(), diagonal);
}

// Translated from 00d1e290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::setInertiaLocal` (Xbox PDB): takes the largest of the matrix diagonal
/// (`+0`, `+0x14`, `+0x28`; each compare keeps the second value unless it is below the
/// first, NaN included) and stores its reciprocal in the three inverse inertia lanes, or `0`
/// when that largest value is not above zero (a NaN too).
pub fn hkp_sphere_motion_set_inertia_local(e: &mut Engine, this: Ptr, inertia: Ptr) {
    let first = x87_quiet(e.mem.f32(inertia.addr()));
    let second = x87_quiet(e.mem.f32(inertia.addr() + 0x14));
    let larger = if second < first { first } else { second };
    let third = x87_quiet(e.mem.f32(inertia.addr() + 0x28));
    let largest = if third < larger { larger } else { third };
    let inverse = if largest > 0.0 {
        (1.0 / f64::from(largest)) as f32
    } else {
        0.0
    };
    for lane in 0..3 {
        e.mem
            .set_f32(this.addr() + INVERSE_INERTIA + 4 * lane, inverse);
    }
}

// Translated from 00d1e320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::setInertiaInvLocal` (Xbox PDB): stores the smallest of the matrix
/// diagonal (`+0`, `+0x14`, `+0x28`; each compare keeps the second value unless it is above
/// the first, NaN included) in the three inverse inertia lanes.
pub fn hkp_sphere_motion_set_inertia_inv_local(e: &mut Engine, this: Ptr, inertia: Ptr) {
    let first = x87_quiet(e.mem.f32(inertia.addr()));
    let second = x87_quiet(e.mem.f32(inertia.addr() + 0x14));
    let smaller = if second > first { first } else { second };
    let third = x87_quiet(e.mem.f32(inertia.addr() + 0x28));
    let smallest = if third > smaller { smaller } else { third };
    for lane in 0..3 {
        e.mem
            .set_f32(this.addr() + INVERSE_INERTIA + 4 * lane, smallest);
    }
}

/// The diagonal matrix of the inverse inertia lane at `+0xC0` that both inverse getters write.
fn write_inverse_inertia(e: &mut Engine, this: u32, out: u32) {
    let diagonal = x87_quiet_bits(e.mem.u32(this + INVERSE_INERTIA));
    write_diagonal(e, out, diagonal);
}

// Translated from 00d1e390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::getInertiaInvLocal` (Xbox PDB): writes a diagonal matrix into `out`
/// holding the inverse inertia at `+0xC0` (through the x87, so a signalling NaN comes out
/// quiet).
pub fn hkp_sphere_motion_get_inertia_inv_local(e: &mut Engine, this: Ptr, out: Ptr) {
    write_inverse_inertia(e, this.addr(), out.addr());
}

// Translated from 00d1e3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::getInertiaInvWorld` (Xbox PDB): the same instructions as
/// `getInertiaInvLocal`.
pub fn hkp_sphere_motion_get_inertia_inv_world(e: &mut Engine, this: Ptr, out: Ptr) {
    write_inverse_inertia(e, this.addr(), out.addr());
}

// Translated from 00d1e410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::applyPointImpulse` (Xbox PDB): adds `inverse mass * impulse` to the
/// linear velocity and `((impulse.zxy * r.yzx) - (impulse.yzx * r.zxy)) * inverse inertia`
/// to the angular velocity, where `r` is `point` minus the centre of mass. All four lanes.
pub fn hkp_sphere_motion_apply_point_impulse(e: &mut Engine, this: Ptr, impulse: Ptr, point: Ptr) {
    let base = this.addr();
    let inverse = load_vec(e, base + INVERSE_INERTIA);
    let force = load_vec(e, impulse.addr());
    let linear = crate::sse::addps(
        crate::sse::mulps(splat(inverse, 3), force),
        load_vec(e, base + LINEAR_VELOCITY),
    );
    store_vec(e, base + LINEAR_VELOCITY, linear);
    let centre = load_vec(e, base + CENTRE_OF_MASS);
    let arm = crate::sse::subps(load_vec(e, point.addr()), centre);
    let force = load_vec(e, impulse.addr());
    let first = crate::sse::mulps(shufps(force, force, 0xc9), shufps(arm, arm, 0xd2));
    let second = crate::sse::mulps(shufps(force, force, 0xd2), shufps(arm, arm, 0xc9));
    let torque = crate::sse::mulps(
        crate::sse::subps(second, first),
        load_vec(e, base + INVERSE_INERTIA),
    );
    let angular = crate::sse::addps(torque, load_vec(e, base + ANGULAR_VELOCITY));
    store_vec(e, base + ANGULAR_VELOCITY, angular);
}

// Translated from 00d1e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::applyAngularImpulse` (Xbox PDB): adds `inverse inertia * impulse` to the
/// angular velocity, per lane (lane 3 uses the inverse mass).
pub fn hkp_sphere_motion_apply_angular_impulse(e: &mut Engine, this: Ptr, impulse: Ptr) {
    let base = this.addr();
    let inverse = load_vec(e, base + INVERSE_INERTIA);
    let change = mul(inverse, load_vec(e, impulse.addr()));
    let angular = add(change, load_vec(e, base + ANGULAR_VELOCITY));
    store_vec(e, base + ANGULAR_VELOCITY, angular);
}

// Translated from 00d1e4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::applyForce` (Xbox PDB): adds `inverse mass * (delta time * force)` to
/// the linear velocity, all four lanes.
pub fn hkp_sphere_motion_apply_force(e: &mut Engine, this: Ptr, delta_time: f32, force: Ptr) {
    let base = this.addr();
    let scaled = crate::sse::mulps([delta_time; 4], load_vec(e, force.addr()));
    let inverse_mass = splat(load_vec(e, base + INVERSE_INERTIA), 3);
    let linear = crate::sse::addps(
        crate::sse::mulps(inverse_mass, scaled),
        load_vec(e, base + LINEAR_VELOCITY),
    );
    store_vec(e, base + LINEAR_VELOCITY, linear);
}

// Translated from 00d1e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::applyForce_ov2` (engine map): multiplies `force` by `delta_time` into a
/// sixteen-byte local and calls the motion's slot `0x50` with that local and `point`.
pub fn hkp_sphere_motion_apply_force_ov2(
    e: &mut Engine,
    this: Ptr,
    delta_time: f32,
    force: Ptr,
    point: Ptr,
) {
    let scaled: Vec4 = mul([delta_time; 4], load_vec(e, force.addr()));
    e.with_stack(0x10, |e, local| {
        store_vec(e, local.addr(), scaled);
        e.vcall(this.addr(), APPLY_FORCE_SLOT, &args![local, point]);
    });
}

// Translated from 00d1e560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::applyTorque` (Xbox PDB): multiplies `torque` by `delta_time` into a
/// sixteen-byte local and calls the motion's slot `0x54` with it.
pub fn hkp_sphere_motion_apply_torque(e: &mut Engine, this: Ptr, delta_time: f32, torque: Ptr) {
    let scaled: Vec4 = mul([delta_time; 4], load_vec(e, torque.addr()));
    e.with_stack(0x10, |e, local| {
        store_vec(e, local.addr(), scaled);
        e.vcall(this.addr(), APPLY_TORQUE_SLOT, &args![local]);
    });
}

// Translated from 00d1e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereMotion::getProjectedPointVelocity` (Xbox PDB): with `r` = `point` minus the
/// centre of mass and `c = normal x r`, stores to `out_velocity` the quantity
/// `(angular velocity * c)` summed over x, y, z plus `(linear velocity * normal)` summed over
/// x, y, z (each sum `z + (y + x)`, SSE, `float`), through the x87 (a signalling NaN comes
/// out quiet); and to `out_mass` `inverse inertia lane 0 * (c.x^2 + c.y^2 + c.z^2)` plus the
/// inverse mass, computed in `f64` on the x87 stack and rounded to a `float`.
pub fn hkp_sphere_motion_get_projected_point_velocity(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    normal: Ptr,
    out_velocity: Ptr,
    out_mass: Ptr,
) {
    let base = this.addr();
    let centre = load_vec(e, base + CENTRE_OF_MASS);
    let arm = crate::sse::subps(load_vec(e, point.addr()), centre);
    let direction = load_vec(e, normal.addr());
    let first = crate::sse::mulps(shufps(direction, direction, 0xc9), shufps(arm, arm, 0xd2));
    let second = crate::sse::mulps(shufps(direction, direction, 0xd2), shufps(arm, arm, 0xc9));
    let linear = crate::sse::mulps(load_vec(e, base + LINEAR_VELOCITY), direction);
    let cross = crate::sse::subps(first, second);
    let angular = crate::sse::mulps(load_vec(e, base + ANGULAR_VELOCITY), cross);
    // `ADDSS` with the higher lane as the destination (the SSE NaN rules).
    let sum = |v: Vec4| crate::sse::add(v[2], crate::sse::add(v[1], v[0]));
    let angular_sum = sum(angular);
    let linear_sum = sum(linear);
    let squares = crate::sse::mulps(cross, cross);
    let velocity = crate::sse::add(angular_sum, linear_sum);
    e.mem
        .set_u32(out_velocity.addr(), x87_quiet_bits(velocity.to_bits()));
    // `FLD [+0xC0]`, `FMUL` the scratch float, `FADD [+0xCC]` (a memory operand): the x87
    // choice of NaN.
    let inertia = x87_load32(e.mem.u32(base + INVERSE_INERTIA));
    let square_sum = sum(squares);
    let mass = x87_add(
        x87_mul(inertia, x87_m32(square_sum.to_bits())),
        x87_m32(e.mem.u32(base + INVERSE_MASS)),
    );
    e.mem.set_u32(out_mass.addr(), x87_store32(mass));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_e1d0, hkp_sphere_motion_get_inertia_local(Ptr, Ptr)),
        entry!(0x00d1_e230, hkp_sphere_motion_get_inertia_world(Ptr, Ptr)),
        entry!(0x00d1_e290, hkp_sphere_motion_set_inertia_local(Ptr, Ptr)),
        entry!(
            0x00d1_e320,
            hkp_sphere_motion_set_inertia_inv_local(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e390,
            hkp_sphere_motion_get_inertia_inv_local(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e3d0,
            hkp_sphere_motion_get_inertia_inv_world(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e410,
            hkp_sphere_motion_apply_point_impulse(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d1_e4a0,
            hkp_sphere_motion_apply_angular_impulse(Ptr, Ptr)
        ),
        entry!(0x00d1_e4d0, hkp_sphere_motion_apply_force(Ptr, f32, Ptr)),
        entry!(
            0x00d1_e520,
            hkp_sphere_motion_apply_force_ov2(Ptr, f32, Ptr, Ptr)
        ),
        entry!(0x00d1_e560, hkp_sphere_motion_apply_torque(Ptr, f32, Ptr)),
        entry!(
            0x00d1_e5a0,
            hkp_sphere_motion_get_projected_point_velocity(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(INERTIA_FLOOR, f64::from(f32::EPSILON));
        e.set_global(INERTIA_FLOOR_REPLACEMENT, f32::EPSILON);
        e
    }

    /// A motion with the given inverse inertia (lane 3: inverse mass), centre of mass, linear
    /// and angular velocity.
    fn motion(e: &mut Engine, inverse: Vec4, centre: Vec4, linear: Vec4, angular: Vec4) -> u32 {
        let this = e.mem.alloc(0x100);
        store_vec(e, this + INVERSE_INERTIA, inverse);
        store_vec(e, this + CENTRE_OF_MASS, centre);
        store_vec(e, this + LINEAR_VELOCITY, linear);
        store_vec(e, this + ANGULAR_VELOCITY, angular);
        this
    }

    fn plain_motion(e: &mut Engine, inverse: Vec4) -> u32 {
        motion(e, inverse, [0.0; 4], [0.0; 4], [0.0; 4])
    }

    /// A matrix buffer filled with a pattern the functions must clear.
    fn dirty_matrix(e: &mut Engine) -> u32 {
        let out = e.mem.alloc(0x30);
        for lane in 0..12 {
            e.mem.set_u32(out + 4 * lane, 0xdead_beef);
        }
        out
    }

    fn matrix(e: &Engine, at: u32) -> [u32; 12] {
        std::array::from_fn(|lane| e.mem.u32(at + 4 * lane as u32))
    }

    fn diagonal_matrix(value: f32) -> [u32; 12] {
        let mut expected = [0; 12];
        for place in [0, 5, 10] {
            expected[place] = value.to_bits();
        }
        expected
    }

    fn diagonal_input(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x30);
        for (lane, value) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 0x14 * lane as u32, value);
        }
        at
    }

    fn inverse_lanes(e: &Engine, this: u32) -> [u32; 3] {
        [0, 4, 8].map(|offset| e.mem.u32(this + INVERSE_INERTIA + offset))
    }

    #[test]
    fn get_inertia_local_is_the_reciprocal_with_a_floor() {
        let mut e = engine();
        let out = dirty_matrix(&mut e);
        let this = plain_motion(&mut e, [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00d1_e1d0, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(0.25));
        // A value below the floor is replaced by it.
        let this = plain_motion(&mut e, [0.0; 4]);
        e.call(0x00d1_e1d0, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(8_388_608.0));
        // A NaN is not below the floor and comes out quiet.
        let this = plain_motion(&mut e, [0.0; 4]);
        e.mem.set_u32(this + INVERSE_INERTIA, 0x7f80_0001);
        e.call(0x00d1_e1d0, &args![this, out]);
        assert_eq!(matrix(&e, out)[0] & 0xffc0_0000, 0x7fc0_0000);
    }

    #[test]
    fn get_inertia_world_is_the_reciprocal_with_a_floor() {
        let mut e = engine();
        let out = dirty_matrix(&mut e);
        let this = plain_motion(&mut e, [0.5, 0.0, 0.0, 0.0]);
        e.call(0x00d1_e230, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(2.0));
        let this = plain_motion(&mut e, [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d1_e230, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(8_388_608.0));
    }

    #[test]
    fn set_inertia_local_stores_the_reciprocal_of_the_largest_diagonal() {
        let mut e = engine();
        let this = plain_motion(&mut e, [9.0; 4]);
        let inertia = diagonal_input(&mut e, [1.0, 4.0, 2.0]);
        e.call(0x00d1_e290, &args![this, inertia]);
        assert_eq!(inverse_lanes(&e, this), [0.25f32.to_bits(); 3]);
        // Nothing above zero: zero.
        let inertia = diagonal_input(&mut e, [-1.0, -2.0, -3.0]);
        e.call(0x00d1_e290, &args![this, inertia]);
        assert_eq!(inverse_lanes(&e, this), [0; 3]);
        // A NaN in the third place stays the largest and is not above zero.
        let inertia = diagonal_input(&mut e, [1.0, 2.0, f32::NAN]);
        e.call(0x00d1_e290, &args![this, inertia]);
        assert_eq!(inverse_lanes(&e, this), [0; 3]);
        // A NaN in the first place loses to the second value.
        let inertia = diagonal_input(&mut e, [f32::NAN, 2.0, 1.0]);
        e.call(0x00d1_e290, &args![this, inertia]);
        assert_eq!(inverse_lanes(&e, this), [0.5f32.to_bits(); 3]);
    }

    #[test]
    fn set_inertia_inv_local_stores_the_smallest_diagonal() {
        let mut e = engine();
        let this = plain_motion(&mut e, [9.0; 4]);
        for (values, smallest) in [
            ([3.0, 1.0, 2.0], 1.0f32),
            ([1.0, 3.0, 2.0], 1.0),
            ([5.0, 6.0, 0.5], 0.5),
        ] {
            let inertia = diagonal_input(&mut e, values);
            e.call(0x00d1_e320, &args![this, inertia]);
            assert_eq!(inverse_lanes(&e, this), [smallest.to_bits(); 3]);
        }
        // A NaN in the third place is kept (the compare is false).
        let inertia = diagonal_input(&mut e, [1.0, 2.0, f32::from_bits(0x7fc0_0005)]);
        e.call(0x00d1_e320, &args![this, inertia]);
        assert_eq!(inverse_lanes(&e, this), [0x7fc0_0005; 3]);
    }

    #[test]
    fn get_inertia_inv_local_writes_the_inverse_inertia() {
        let mut e = engine();
        let out = dirty_matrix(&mut e);
        let this = plain_motion(&mut e, [2.5, 0.0, 0.0, 0.0]);
        e.call(0x00d1_e390, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(2.5));
        // A signalling NaN comes out quiet.
        e.mem.set_u32(this + INVERSE_INERTIA, 0x7f80_0001);
        e.call(0x00d1_e390, &args![this, out]);
        assert_eq!(matrix(&e, out)[5], 0x7fc0_0001);
    }

    #[test]
    fn get_inertia_inv_world_writes_the_inverse_inertia() {
        let mut e = engine();
        let out = dirty_matrix(&mut e);
        let this = plain_motion(&mut e, [-3.0, 7.0, 7.0, 7.0]);
        e.call(0x00d1_e3d0, &args![this, out]);
        assert_eq!(matrix(&e, out), diagonal_matrix(-3.0));
    }

    #[test]
    fn apply_point_impulse_changes_both_velocities() {
        let mut e = engine();
        let this = motion(
            &mut e,
            [2.0, 3.0, 4.0, 0.5],
            [1.0, 2.0, 3.0, 0.0],
            [10.0, 20.0, 30.0, 40.0],
            [1.0, 1.0, 1.0, 1.0],
        );
        let impulse = e.mem.alloc(0x10);
        store_vec(&mut e, impulse, [1.0, 2.0, 3.0, 4.0]);
        let point = e.mem.alloc(0x10);
        store_vec(&mut e, point, [4.0, 6.0, 8.0, 9.0]);
        e.call(0x00d1_e410, &args![this, impulse, point]);
        assert_eq!(
            load_vec(&e, this + LINEAR_VELOCITY),
            [10.5, 21.0, 31.5, 42.0]
        );
        // r = (3, 4, 5, 9); r x impulse = (2, -4, 2, 0), times the inverse inertia.
        assert_eq!(
            load_vec(&e, this + ANGULAR_VELOCITY),
            [5.0, -11.0, 9.0, 1.0]
        );
    }

    #[test]
    fn apply_angular_impulse_adds_the_scaled_impulse() {
        let mut e = engine();
        let this = motion(
            &mut e,
            [2.0, 3.0, 4.0, 0.5],
            [0.0; 4],
            [0.0; 4],
            [1.0, 1.0, 1.0, 1.0],
        );
        let impulse = e.mem.alloc(0x10);
        store_vec(&mut e, impulse, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00d1_e4a0, &args![this, impulse]);
        assert_eq!(load_vec(&e, this + ANGULAR_VELOCITY), [3.0, 7.0, 13.0, 3.0]);
    }

    #[test]
    fn apply_force_scales_by_time_and_inverse_mass() {
        let mut e = engine();
        let this = motion(&mut e, [9.0, 9.0, 9.0, 0.25], [0.0; 4], [1.0; 4], [0.0; 4]);
        let force = e.mem.alloc(0x10);
        store_vec(&mut e, force, [2.0, 4.0, 6.0, 8.0]);
        e.call(0x00d1_e4d0, &args![this, 0.5f32, force]);
        assert_eq!(load_vec(&e, this + LINEAR_VELOCITY), [1.25, 1.5, 1.75, 2.0]);
    }

    type Seen = Rc<RefCell<Vec<(Vec<u32>, Vec4)>>>;

    /// Installs a vtable on `this` whose `slot` points at a double that records its words and
    /// the sixteen bytes its second word points to.
    fn recording_slot(e: &mut Engine, this: u32, slot: u32) -> Seen {
        let target = 0x0900_0021;
        let mut slots = vec![0; 0x20];
        slots[(slot / 4) as usize] = target;
        e.put_vtable(0x0110_0000, &slots);
        e.mem.set_u32(this, 0x0110_0000);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(target, move |e, a| {
            let local = load_vec(e, a[1]);
            log.borrow_mut().push((a.to_vec(), local));
            Ret::default()
        });
        seen
    }

    #[test]
    fn apply_force_ov2_passes_the_scaled_force_and_the_point() {
        let mut e = engine();
        let this = plain_motion(&mut e, [1.0; 4]);
        let seen = recording_slot(&mut e, this, APPLY_FORCE_SLOT);
        let force = e.mem.alloc(0x10);
        store_vec(&mut e, force, [2.0, 4.0, 6.0, 8.0]);
        let point = e.mem.alloc(0x10);
        e.call(0x00d1_e520, &args![this, 0.5f32, force, point]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], this);
        assert_eq!(seen[0].0[2], point);
        assert_eq!(seen[0].1, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn apply_torque_passes_the_scaled_torque() {
        let mut e = engine();
        let this = plain_motion(&mut e, [1.0; 4]);
        let seen = recording_slot(&mut e, this, APPLY_TORQUE_SLOT);
        let torque = e.mem.alloc(0x10);
        store_vec(&mut e, torque, [1.0, -2.0, 4.0, 0.0]);
        e.call(0x00d1_e560, &args![this, 2.0f32, torque]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0.len(), 2);
        assert_eq!(seen[0].0[0], this);
        assert_eq!(seen[0].1, [2.0, -4.0, 8.0, 0.0]);
    }

    #[test]
    fn get_projected_point_velocity_sums_both_velocities() {
        let mut e = engine();
        let this = motion(
            &mut e,
            [2.0, 9.0, 9.0, 0.5],
            [0.0, 1.0, 0.0, 0.0],
            [5.0, 6.0, 7.0, 0.0],
            [2.0, 3.0, 4.0, 0.0],
        );
        let point = e.mem.alloc(0x10);
        store_vec(&mut e, point, [1.0, 2.0, 3.0, 0.0]);
        let normal = e.mem.alloc(0x10);
        store_vec(&mut e, normal, [0.0, 0.0, 1.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d1_e5a0, &args![this, point, normal, out, out + 4]);
        // r = (1, 1, 3); normal x r = (-1, 1, 0): 2 * -1 + 3 * 1 + 7 = 8; 2 * 2 + 0.5 = 4.5.
        assert_eq!(e.mem.f32(out), 8.0);
        assert_eq!(e.mem.f32(out + 4), 4.5);
    }
}
