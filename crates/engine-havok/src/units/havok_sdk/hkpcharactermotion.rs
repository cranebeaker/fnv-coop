//! `Havok SDK/hkpcharactermotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCharacterMotion` is an `hkpMotion` (Xbox PDB layout: `m_motionState`
//! +0x10, whose `m_sweptTransform.m_centerOfMass1` is at +0x60,
//! `m_inertiaAndMassInv` +0xC0, `m_linearVelocity` +0xD0,
//! `m_angularVelocity` +0xE0) whose inertia is a single scalar kept in the
//! three inverse-inertia lanes of `m_inertiaAndMassInv`; its `w` lane is the
//! inverse mass. SSE code (`MULPS`, `SUBPS`, `ADDSS`, `SHUFPS`) computes
//! lane by lane in `f32`; a dot product is `z + (y + x)`. x87 code compares
//! the way its `FNSTSW`/`TEST AH` pairs read the flags (NaN included) and
//! rounds at each `float` store; a float copied through `FLD`/`FSTP` goes
//! through [`x87_quiet`].

#[allow(unused_imports)]
use crate::prelude::*;
use crate::sse::{add, addps, mulps, subps};

layout! {
    /// `hkpCharacterMotion` (Xbox PDB `hkpMotion` layout): only the fields
    /// the unit reads or writes; the size is that of `hkpMotion`.
    pub struct HkpCharacterMotion: 0x120 {
        /// The vtable.
        0x00 vtable: u32,
    }
}

/// `m_motionState.m_sweptTransform.m_centerOfMass1` (Xbox PDB), a vector.
const CENTER_OF_MASS: u32 = 0x60;
/// `m_inertiaAndMassInv` (Xbox PDB): inverse inertia in x, y, z and the
/// inverse mass in `w`.
const INERTIA_AND_MASS_INV: u32 = 0xc0;
/// `m_linearVelocity` and `m_angularVelocity` (Xbox PDB), vectors.
const LINEAR_VELOCITY: u32 = 0xd0;
const ANGULAR_VELOCITY: u32 = 0xe0;
/// `hkpMotion` vtable slots (Xbox PDB): `applyPointImpulse` and
/// `applyAngularImpulse`.
const APPLY_POINT_IMPULSE_SLOT: u32 = 0x50;
const APPLY_ANGULAR_IMPULSE_SLOT: u32 = 0x54;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `SHUFPS v, v, 0xc9`: lanes (1, 2, 0, 3).
fn shuffle_c9(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: lanes (2, 0, 1, 3).
fn shuffle_d2(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

/// The inverse mass, broadcast to every lane (`SHUFPS v, v, 0xff` of the
/// vector at +0xC0).
fn inverse_mass(e: &Engine, this: Ptr<HkpCharacterMotion>) -> f32 {
    e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 0xc)
}

/// Writes a 3x4 matrix of zeros with `value` on the diagonal (three
/// four-lane rows), as the four inertia getters do.
fn write_diagonal(e: &mut Engine, out: Ptr, value: f32) {
    for row in 0..3 {
        store_vec(e, out.addr() + 0x10 * row, [0.0; 4]);
    }
    for row in 0..3 {
        e.mem.set_f32(out.addr() + 0x14 * row, value);
    }
}

/// `1 / inertia-inverse lane`, the value `getInertiaLocal` and
/// `getInertiaWorld` put on the diagonal: `FLD1 / m` in x87, stored as a
/// `float`.
fn reciprocal_of_first_lane(e: &Engine, this: Ptr<HkpCharacterMotion>) -> f32 {
    let m = x87_quiet(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV));
    (1.0f64 / m as f64) as f32
}

// Translated from 00d1ecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::getInertiaLocal` (Xbox PDB): writes the 3x4 matrix
/// with `1 / m_inertiaAndMassInv.x` on its diagonal and zeros elsewhere.
pub fn hkp_character_motion_get_inertia_local(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    out: Ptr,
) {
    let inverse = reciprocal_of_first_lane(e, this);
    write_diagonal(e, out, x87_quiet(inverse));
}

// Translated from 00d1ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::getInertiaWorld` (Xbox PDB): the same matrix as
/// `getInertiaLocal` (the inertia is a scalar, so it is the same in the
/// world frame).
pub fn hkp_character_motion_get_inertia_world(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    out: Ptr,
) {
    let inverse = reciprocal_of_first_lane(e, this);
    write_diagonal(e, out, x87_quiet(inverse));
}

// Translated from 00d1ed40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::setInertiaLocal` (Xbox PDB): takes the largest of
/// the matrix's diagonal entries (`in[0]`, `in[5]`, `in[10]`; a NaN or
/// equal entry is replaced by the later one) and stores its reciprocal, or
/// 0 when it is not above 0, in the three inverse-inertia lanes.
pub fn hkp_character_motion_set_inertia_local(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    matrix: Ptr,
) {
    let x = x87_quiet(e.mem.f32(matrix.addr()));
    let y = x87_quiet(e.mem.f32(matrix.addr() + 0x14));
    // `FCOM; TEST AH,5; JP`: the later value unless it is smaller.
    let larger = if y < x { x } else { y };
    let z = x87_quiet(e.mem.f32(matrix.addr() + 0x28));
    let largest = if z < larger { larger } else { z };
    // `FCOM` against 0: not above 0 (NaN included) gives 0.
    let inverse = if largest > 0.0 {
        (1.0f64 / largest as f64) as f32
    } else {
        0.0
    };
    let inverse = x87_quiet(inverse);
    for lane in 0..3 {
        e.mem
            .set_f32(this.addr() + INERTIA_AND_MASS_INV + 4 * lane, inverse);
    }
}

// Translated from 00d1edd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::setInertiaInvLocal` (Xbox PDB): stores the smallest
/// of the matrix's diagonal entries (`in[0]`, `in[5]`, `in[10]`; a NaN or
/// equal entry is replaced by the later one) in the three inverse-inertia
/// lanes.
pub fn hkp_character_motion_set_inertia_inv_local(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    matrix: Ptr,
) {
    let x = x87_quiet(e.mem.f32(matrix.addr()));
    let y = x87_quiet(e.mem.f32(matrix.addr() + 0x14));
    // `FCOM; TEST AH,0x41; JNZ`: the later value unless it is larger.
    let smaller = if y > x { x } else { y };
    let z = x87_quiet(e.mem.f32(matrix.addr() + 0x28));
    let smallest = if z > smaller { smaller } else { z };
    let smallest = x87_quiet(smallest);
    for lane in 0..3 {
        e.mem
            .set_f32(this.addr() + INERTIA_AND_MASS_INV + 4 * lane, smallest);
    }
}

// Translated from 00d1ee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::getInertiaInvLocal` (Xbox PDB): writes the 3x4
/// matrix with `m_inertiaAndMassInv.x` on its diagonal and zeros elsewhere.
pub fn hkp_character_motion_get_inertia_inv_local(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    out: Ptr,
) {
    let value = x87_quiet(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV));
    write_diagonal(e, out, value);
}

// Translated from 00d1ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::getInertiaInvWorld` (Xbox PDB): the same matrix as
/// `getInertiaInvLocal`.
pub fn hkp_character_motion_get_inertia_inv_world(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    out: Ptr,
) {
    let value = x87_quiet(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV));
    write_diagonal(e, out, value);
}

// Translated from 00d1eec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::applyPointImpulse` (Xbox PDB): adds `impulse *
/// inverse mass` to the linear velocity and `((impulse x (point - centre of
/// mass)) with its sign as the code forms it) * inverse inertia` to the
/// angular velocity; every lane, `w` included, is computed.
pub fn hkp_character_motion_apply_point_impulse(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    impulse: Ptr,
    point: Ptr,
) {
    let me = this.addr();
    let mass_inv = inverse_mass(e, this);
    let j = load_vec(e, impulse.addr());
    let linear = load_vec(e, me + LINEAR_VELOCITY);
    store_vec(
        e,
        me + LINEAR_VELOCITY,
        addps(mulps([mass_inv; 4], j), linear),
    );

    let centre = load_vec(e, me + CENTER_OF_MASS);
    let r = subps(load_vec(e, point.addr()), centre);
    let j = load_vec(e, impulse.addr());
    let first = mulps(shuffle_c9(j), shuffle_d2(r));
    let second = mulps(shuffle_d2(j), shuffle_c9(r));
    let inertia = load_vec(e, me + INERTIA_AND_MASS_INV);
    let torque = mulps(subps(second, first), inertia);
    let angular = load_vec(e, me + ANGULAR_VELOCITY);
    store_vec(e, me + ANGULAR_VELOCITY, addps(torque, angular));
}

// Translated from 00d1ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::applyAngularImpulse` (Xbox PDB): adds `impulse *
/// m_inertiaAndMassInv` (lane by lane) to the angular velocity.
pub fn hkp_character_motion_apply_angular_impulse(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    impulse: Ptr,
) {
    let me = this.addr();
    let inertia = load_vec(e, me + INERTIA_AND_MASS_INV);
    let j = load_vec(e, impulse.addr());
    let angular = load_vec(e, me + ANGULAR_VELOCITY);
    store_vec(
        e,
        me + ANGULAR_VELOCITY,
        zip_with(zip_with(inertia, j, |p, q| p * q), angular, |p, q| p + q),
    );
}

// Translated from 00d1ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::applyForce` (Xbox PDB, the overload without a
/// point): adds `inverse mass * (delta_time * force)` to the linear
/// velocity.
pub fn hkp_character_motion_apply_force(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    delta_time: f32,
    force: Ptr,
) {
    let me = this.addr();
    let f = load_vec(e, force.addr());
    let mass_inv = inverse_mass(e, this);
    let linear = load_vec(e, me + LINEAR_VELOCITY);
    let impulse = mulps([mass_inv; 4], mulps([delta_time; 4], f));
    store_vec(e, me + LINEAR_VELOCITY, addps(impulse, linear));
}

// Translated from 00d1efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::applyForce` (Xbox PDB, the overload with a point):
/// builds `delta_time * force` and applies it as an impulse at `point`
/// through the vtable's `applyPointImpulse` (slot +0x50).
pub fn hkp_character_motion_apply_force_ov2(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    delta_time: f32,
    force: Ptr,
    point: Ptr,
) {
    let f = load_vec(e, force.addr());
    e.with_stack(0x10, |e, impulse| {
        store_vec(e, impulse.addr(), f.map(|lane| delta_time * lane));
        e.vcall(
            this.addr(),
            APPLY_POINT_IMPULSE_SLOT,
            &args![impulse, point],
        );
    });
}

// Translated from 00d1f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::applyTorque` (Xbox PDB): applies `delta_time *
/// torque` as an angular impulse through the vtable's `applyAngularImpulse`
/// (slot +0x54).
pub fn hkp_character_motion_apply_torque(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    delta_time: f32,
    torque: Ptr,
) {
    let t = load_vec(e, torque.addr());
    e.with_stack(0x10, |e, impulse| {
        store_vec(e, impulse.addr(), t.map(|lane| delta_time * lane));
        e.vcall(this.addr(), APPLY_ANGULAR_IMPULSE_SLOT, &args![impulse]);
    });
}

// Translated from 00d1f050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterMotion::getProjectedPointVelocity` (Xbox PDB): writes to
/// `out_velocity` the velocity of `point` along `normal`, `(n x r) .
/// angular velocity + linear velocity . n` with `r = point - centre of
/// mass`, and the inverse mass to `out_inverse_mass`.
pub fn hkp_character_motion_get_projected_point_velocity(
    e: &mut Engine,
    this: Ptr<HkpCharacterMotion>,
    point: Ptr,
    normal: Ptr,
    out_velocity: Ptr,
    out_inverse_mass: Ptr,
) {
    let me = this.addr();
    let centre = load_vec(e, me + CENTER_OF_MASS);
    let r = subps(load_vec(e, point.addr()), centre);
    let n = load_vec(e, normal.addr());
    let linear = load_vec(e, me + LINEAR_VELOCITY);
    let along = mulps(linear, n);
    let first = mulps(shuffle_c9(n), shuffle_d2(r));
    let second = mulps(shuffle_d2(n), shuffle_c9(r));
    let angular = load_vec(e, me + ANGULAR_VELOCITY);
    let turning = mulps(subps(first, second), angular);
    let turning_sum = add(turning[2], add(turning[1], turning[0]));
    let along_sum = add(along[2], add(along[1], along[0]));
    e.mem
        .set_f32(out_velocity.addr(), x87_quiet(add(turning_sum, along_sum)));
    let mass_inv = x87_quiet(inverse_mass(e, this));
    e.mem.set_f32(out_inverse_mass.addr(), mass_inv);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d1ecc0,
            hkp_character_motion_get_inertia_local(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1ed00,
            hkp_character_motion_get_inertia_world(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1ed40,
            hkp_character_motion_set_inertia_local(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1edd0,
            hkp_character_motion_set_inertia_inv_local(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1ee40,
            hkp_character_motion_get_inertia_inv_local(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1ee80,
            hkp_character_motion_get_inertia_inv_world(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1eec0,
            hkp_character_motion_apply_point_impulse(Ptr<HkpCharacterMotion>, Ptr, Ptr)
        ),
        entry!(
            0x00d1ef50,
            hkp_character_motion_apply_angular_impulse(Ptr<HkpCharacterMotion>, Ptr)
        ),
        entry!(
            0x00d1ef80,
            hkp_character_motion_apply_force(Ptr<HkpCharacterMotion>, f32, Ptr)
        ),
        entry!(
            0x00d1efd0,
            hkp_character_motion_apply_force_ov2(Ptr<HkpCharacterMotion>, f32, Ptr, Ptr)
        ),
        entry!(
            0x00d1f010,
            hkp_character_motion_apply_torque(Ptr<HkpCharacterMotion>, f32, Ptr)
        ),
        entry!(
            0x00d1f050,
            hkp_character_motion_get_projected_point_velocity(
                Ptr<HkpCharacterMotion>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            )
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
        e
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> Ptr {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        Ptr::new(address)
    }

    /// A motion with inverse inertia (0.5, 0.25, 0.125) and inverse mass 2,
    /// centre of mass (1, 0, 0), linear velocity (1, 2, 3, 4) and angular
    /// velocity (0.5, 0.5, 0.5, 0.5).
    fn motion(e: &mut Engine) -> Ptr<HkpCharacterMotion> {
        let this = e.new_object::<HkpCharacterMotion>();
        let me = this.addr();
        store_vec(e, me + CENTER_OF_MASS, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, me + INERTIA_AND_MASS_INV, [0.5, 0.25, 0.125, 2.0]);
        store_vec(e, me + LINEAR_VELOCITY, [1.0, 2.0, 3.0, 4.0]);
        store_vec(e, me + ANGULAR_VELOCITY, [0.5, 0.5, 0.5, 0.5]);
        this
    }

    fn matrix(e: &mut Engine, diagonal: [f32; 3]) -> Ptr {
        let address = e.mem.alloc(0x30);
        for (row, value) in diagonal.into_iter().enumerate() {
            e.mem.set_f32(address + 0x14 * row as u32, value);
        }
        Ptr::new(address)
    }

    fn read_matrix(e: &Engine, address: u32) -> [f32; 12] {
        std::array::from_fn(|index| e.mem.f32(address + 4 * index as u32))
    }

    fn diagonal_matrix(value: f32) -> [f32; 12] {
        let mut expected = [0.0; 12];
        for index in [0, 5, 10] {
            expected[index] = value;
        }
        expected
    }

    #[test]
    fn inertia_getters_put_the_reciprocal_on_the_diagonal() {
        let mut e = engine();
        let this = motion(&mut e);
        for address in [0x00d1ecc0u32, 0x00d1ed00] {
            let out = e.mem.alloc(0x30);
            for index in 0..12 {
                e.mem.set_f32(out + 4 * index, 9.0);
            }
            e.call(address, &args![this, out]);
            assert_eq!(read_matrix(&e, out), diagonal_matrix(2.0));
        }
    }

    #[test]
    fn inertia_inverse_getters_put_the_stored_value_on_the_diagonal() {
        let mut e = engine();
        let this = motion(&mut e);
        for address in [0x00d1ee40u32, 0x00d1ee80] {
            let out = e.mem.alloc(0x30);
            for index in 0..12 {
                e.mem.set_f32(out + 4 * index, 9.0);
            }
            e.call(address, &args![this, out]);
            assert_eq!(read_matrix(&e, out), diagonal_matrix(0.5));
        }
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.mem
            .set_u32(this.addr() + INERTIA_AND_MASS_INV, 0x7f80_0001);
        let out = e.mem.alloc(0x30);
        e.call(0x00d1ee40, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
    }

    #[test]
    fn set_inertia_local_stores_the_reciprocal_of_the_largest_entry() {
        let mut e = engine();
        let this = motion(&mut e);
        let input = matrix(&mut e, [2.0, 8.0, 4.0]);
        e.call(0x00d1ed40, &args![this, input]);
        for lane in 0..3 {
            assert_eq!(
                e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 4 * lane),
                0.125
            );
        }
        // The inverse mass is untouched.
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 0xc), 2.0);
        // Largest entry last, and the zero / negative / NaN cases give 0.
        let last = matrix(&mut e, [1.0, 2.0, 4.0]);
        e.call(0x00d1ed40, &args![this, last]);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV), 0.25);
        let zero = matrix(&mut e, [0.0, -1.0, 0.0]);
        e.call(0x00d1ed40, &args![this, zero]);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 4), 0.0);
        let nan = matrix(&mut e, [f32::NAN, f32::NAN, f32::NAN]);
        e.call(0x00d1ed40, &args![this, nan]);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 8), 0.0);
    }

    #[test]
    fn set_inertia_inv_local_stores_the_smallest_entry() {
        let mut e = engine();
        let this = motion(&mut e);
        let input = matrix(&mut e, [2.0, 0.5, 4.0]);
        e.call(0x00d1edd0, &args![this, input]);
        for lane in 0..3 {
            assert_eq!(
                e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 4 * lane),
                0.5
            );
        }
        let first = matrix(&mut e, [0.25, 0.5, 4.0]);
        e.call(0x00d1edd0, &args![this, first]);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV), 0.25);
        let last = matrix(&mut e, [4.0, 3.0, 1.0]);
        e.call(0x00d1edd0, &args![this, last]);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 8), 1.0);
        assert_eq!(e.mem.f32(this.addr() + INERTIA_AND_MASS_INV + 0xc), 2.0);
    }

    #[test]
    fn point_impulse_changes_both_velocities() {
        let mut e = engine();
        let this = motion(&mut e);
        let impulse = vector_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        // r = point - centre = (0, 1, 0).
        let point = vector_at(&mut e, [1.0, 1.0, 0.0, 0.0]);
        e.call(0x00d1eec0, &args![this, impulse, point]);
        // Linear: (1,2,3,4) + 2 * (0,0,1,0).
        assert_eq!(
            load_vec(&e, this.addr() + LINEAR_VELOCITY),
            [1.0, 2.0, 5.0, 4.0]
        );
        // The code forms j x r' = (j2*r1 - j1*r2, j0*r2 - j2*r0, j1*r0 -
        // j0*r1) = (1, 0, 0); times (0.5, 0.25, 0.125, 2) plus 0.5.
        assert_eq!(
            load_vec(&e, this.addr() + ANGULAR_VELOCITY),
            [1.0, 0.5, 0.5, 0.5]
        );
    }

    #[test]
    fn angular_impulse_scales_by_the_inverse_inertia() {
        let mut e = engine();
        let this = motion(&mut e);
        let impulse = vector_at(&mut e, [2.0, 4.0, 8.0, 1.0]);
        e.call(0x00d1ef50, &args![this, impulse]);
        assert_eq!(
            load_vec(&e, this.addr() + ANGULAR_VELOCITY),
            [1.5, 1.5, 1.5, 2.5]
        );
    }

    #[test]
    fn force_changes_the_linear_velocity() {
        let mut e = engine();
        let this = motion(&mut e);
        let force = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d1ef80, &args![this, 0.5f32, force]);
        // 2 * (0.5 * 1) added to x.
        assert_eq!(
            load_vec(&e, this.addr() + LINEAR_VELOCITY),
            [2.0, 2.0, 3.0, 4.0]
        );
    }

    /// A vtable whose slots 0x50 and 0x54 are recorded by doubles.
    fn recording_vtable(
        e: &mut Engine,
        this: Ptr<HkpCharacterMotion>,
    ) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut slots = vec![0u32; 0x58 / 4];
        slots[0x50 / 4] = 0x0100_2000;
        slots[0x54 / 4] = 0x0100_2004;
        e.put_vtable(0x0100_1000, &slots);
        e.set(this, HkpCharacterMotion::vtable, 0x0100_1000);
        for address in [0x0100_2000u32, 0x0100_2004] {
            let shared = Rc::clone(&calls);
            e.register_double(address, move |e, a| {
                let mut words = vec![address];
                words.extend_from_slice(a);
                // The impulse is a stack local: record its lanes too.
                words.extend((0..4).map(|lane| e.mem.u32(a[1] + 4 * lane)));
                shared.borrow_mut().push(words);
                Ret::default()
            });
        }
        calls
    }

    #[test]
    fn force_at_a_point_applies_a_point_impulse() {
        let mut e = engine();
        let this = motion(&mut e);
        let calls = recording_vtable(&mut e, this);
        let force = vector_at(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let point = vector_at(&mut e, [9.0, 9.0, 9.0, 9.0]);
        e.call(0x00d1efd0, &args![this, 0.5f32, force, point]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], 0x0100_2000);
        assert_eq!(calls[0][1], this.addr());
        assert_eq!(calls[0][3], point.addr());
        let lanes: Vec<f32> = calls[0][4..8].iter().map(|b| f32::from_bits(*b)).collect();
        assert_eq!(lanes, [0.5, 1.0, 1.5, 2.0]);
    }

    #[test]
    fn torque_applies_an_angular_impulse() {
        let mut e = engine();
        let this = motion(&mut e);
        let calls = recording_vtable(&mut e, this);
        let torque = vector_at(&mut e, [2.0, 4.0, 6.0, 8.0]);
        e.call(0x00d1f010, &args![this, 0.5f32, torque]);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], 0x0100_2004);
        assert_eq!(calls[0][1], this.addr());
        let lanes: Vec<f32> = calls[0][3..7].iter().map(|b| f32::from_bits(*b)).collect();
        assert_eq!(lanes, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn projected_point_velocity_adds_the_spin_and_the_linear_part() {
        let mut e = engine();
        let this = motion(&mut e);
        // r = (0, 1, 0), n = (0, 0, 1): the code forms n x r = (n1*r2 -
        // n2*r1, n2*r0 - n0*r2, n0*r1 - n1*r0) = (-1, 0, 0); times the
        // angular velocity (0.5, ...) = -0.5; plus linear . n = 3.
        let point = vector_at(&mut e, [1.0, 1.0, 0.0, 0.0]);
        let normal = vector_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let out = e.mem.alloc(8);
        e.call(0x00d1f050, &args![this, point, normal, out, out + 4]);
        assert_eq!(e.mem.f32(out), 2.5);
        assert_eq!(e.mem.f32(out + 4), 2.0);
    }
}
