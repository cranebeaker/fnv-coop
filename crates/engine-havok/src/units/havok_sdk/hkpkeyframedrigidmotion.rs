//! `Havok SDK/hkpkeyframedrigidmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};

/// The motion base constructor (`00c86530`, `this`, `(position, rotation, 0)`).
const MOTION_CONSTRUCT: u32 = 0x00c8_6530;
/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpKeyframedRigidMotion`'s vtable (RTTI confirmed) and the `hkReferencedObject` base's.
const KEYFRAMED_MOTION_VTABLE: u32 = 0x010c_6bec;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00d1dfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::hkpKeyframedRigidMotion` (Xbox PDB): the motion base
/// constructor (`00c86530` with `(position, rotation, 0)`), the vtable, a null pointer at
/// +0x118, a zero 16-bit word at +0x11C, a zero vector at +0xC0 and motion type 4 at +8.
/// Returns `this`.
pub fn hkp_keyframed_rigid_motion_construct(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) -> Ptr {
    e.call(MOTION_CONSTRUCT, &args![this, position, rotation, 0u32]);
    e.mem.set_u32(this.addr(), KEYFRAMED_MOTION_VTABLE);
    e.mem.set_u32(this.addr() + 0x118, 0);
    e.mem.set_u16(this.addr() + 0x11c, 0);
    e.mem.write(this.addr() + 0xc0, &[0; 16]);
    e.mem.set_u8(this.addr() + 8, 4);
    this
}

// Translated from 00d1dff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::~hkpKeyframedRigidMotion` (Xbox PDB): releases the object held
/// at +0x118 (when there is one) and leaves the `hkReferencedObject` vtable.
pub fn hkp_keyframed_rigid_motion_destruct(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr() + 0x118);
    e.mem.set_u32(this.addr(), KEYFRAMED_MOTION_VTABLE);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(this.addr(), REFERENCED_OBJECT_VTABLE);
}

/// `hkReferencedObject::addReference` (`00c90510`, `this` = the object).
const ADD_REFERENCE: u32 = 0x00c9_0510;

// Translated from 00d1dec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setMass` (Xbox PDB): a keyframed motion ignores it. The word is
/// not read.
pub fn hkp_keyframed_rigid_motion_set_mass(_e: &mut Engine, _this: Ptr, _unused_0: u32) {}

// Translated from 00d1ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setMassInv` (Xbox PDB): ignored; the word is not read.
pub fn hkp_keyframed_rigid_motion_set_mass_inv(_e: &mut Engine, _this: Ptr, _unused_0: u32) {}

// Translated from 00d1dee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setInertiaLocal` (Xbox PDB): ignored; the word is not read.
pub fn hkp_keyframed_rigid_motion_set_inertia_local(_e: &mut Engine, _this: Ptr, _unused_0: u32) {}

// Translated from 00d1def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setInertiaInvLocal` (Xbox PDB): ignored; the word is not read.
pub fn hkp_keyframed_rigid_motion_set_inertia_inv_local(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
) {
}

// Translated from 00d1df00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyLinearImpulse` (Xbox PDB): ignored; the word is not read.
pub fn hkp_keyframed_rigid_motion_apply_linear_impulse(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
) {
}

// Translated from 00d1df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyPointImpulse` (Xbox PDB): ignored; the two words are not
/// read.
pub fn hkp_keyframed_rigid_motion_apply_point_impulse(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00d1df20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyAngularImpulse` (Xbox PDB): ignored; the word is not read.
pub fn hkp_keyframed_rigid_motion_apply_angular_impulse(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
) {
}

// Translated from 00d1df30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyForce` (Xbox PDB): ignored; the two words are not read.
pub fn hkp_keyframed_rigid_motion_apply_force(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00d1df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyForce_ov2` (Xbox PDB): ignored; the three words are not
/// read.
pub fn hkp_keyframed_rigid_motion_apply_force_ov2(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00d1df50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::applyTorque` (Xbox PDB): ignored; the two words are not read.
pub fn hkp_keyframed_rigid_motion_apply_torque(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00d1df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setStepPosition` (Xbox PDB): ignored; the two words are not read.
pub fn hkp_keyframed_rigid_motion_set_step_position(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
) {
}

// Translated from 00d1df70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::setStoredMotion` (Xbox PDB): takes a reference on the new object
/// when there is one, releases the one held at +0x118 when there is one, and stores the
/// new pointer there.
pub fn hkp_keyframed_rigid_motion_set_stored_motion(e: &mut Engine, this: Ptr, stored: Ptr) {
    if stored.addr() != 0 {
        e.call(ADD_REFERENCE, &args![stored]);
    }
    let held = e.mem.u32(this.addr() + 0x118);
    if held != 0 {
        e.call(REMOVE_REFERENCE, &args![held]);
    }
    e.mem.set_u32(this.addr() + 0x118, stored.addr());
}

/// The three zero rows (`MOVAPS` of zero at +0, +0x10 and +0x20) the inertia getters leave in
/// their output matrix.
fn zero_matrix(e: &mut Engine, out: Ptr) {
    for row in [0, 0x10, 0x20] {
        store_vec(e, out.addr() + row, [0.0; 4]);
    }
}

// Translated from 00d1e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::getInertiaLocal` (Xbox PDB): a zero matrix.
pub fn hkp_keyframed_rigid_motion_get_inertia_local(e: &mut Engine, _this: Ptr, out: Ptr) {
    zero_matrix(e, out);
}

// Translated from 00d1e030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::getInertiaWorld` (Xbox PDB): a zero matrix.
pub fn hkp_keyframed_rigid_motion_get_inertia_world(e: &mut Engine, _this: Ptr, out: Ptr) {
    zero_matrix(e, out);
}

// Translated from 00d1e050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::getInertiaInvLocal` (Xbox PDB): a zero matrix.
pub fn hkp_keyframed_rigid_motion_get_inertia_inv_local(e: &mut Engine, _this: Ptr, out: Ptr) {
    zero_matrix(e, out);
}

// Translated from 00d1e070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::getInertiaInvWorld` (Xbox PDB): a zero matrix.
pub fn hkp_keyframed_rigid_motion_get_inertia_inv_world(e: &mut Engine, _this: Ptr, out: Ptr) {
    zero_matrix(e, out);
}

// Translated from 00d1e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpKeyframedRigidMotion::getProjectedPointVelocity` (Xbox PDB): the velocity of `point`
/// along `normal`. With the arm `r = point - (+0x60)`, it is `(normal x r) . (+0xE0) +
/// (+0xD0) . normal` (the sums are `z + (y + x)` per product, the angular one first) and goes
/// through the x87 stack into `velocity`; `distance` becomes 0.0. The meaning of the three
/// fields (the centre of mass, linear and angular velocity) is unconfirmed.
pub fn hkp_keyframed_rigid_motion_get_projected_point_velocity(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    normal: Ptr,
    velocity: Ptr,
    distance: Ptr,
) {
    let me = this.addr();
    let centre = load_vec(e, me + 0x60);
    // `SUBPS`, `MULPS`, `ADDSS` with the SSE NaN rules, the destination operand first:
    // `normal.yzxw * arm.zxyw - normal.zxyw * arm.yzxw`.
    use crate::sse::{add, mulps, subps};
    let sum = |v: [f32; 4]| add(v[2], add(v[1], v[0]));
    let yzxw = |v: [f32; 4]| [v[1], v[2], v[0], v[3]];
    let zxyw = |v: [f32; 4]| [v[2], v[0], v[1], v[3]];
    let arm = subps(load_vec(e, point.addr()), centre);
    let direction = load_vec(e, normal.addr());
    let linear = mulps(load_vec(e, me + 0xd0), direction);
    let moment = subps(
        mulps(yzxw(direction), zxyw(arm)),
        mulps(zxyw(direction), yzxw(arm)),
    );
    let angular = mulps(moment, load_vec(e, me + 0xe0));
    let along = add(sum(angular), sum(linear));
    e.mem.set_f32(velocity.addr(), x87_quiet(along));
    e.mem.set_f32(distance.addr(), 0.0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_dec0, hkp_keyframed_rigid_motion_set_mass(Ptr, u32)),
        entry!(
            0x00d1_ded0,
            hkp_keyframed_rigid_motion_set_mass_inv(Ptr, u32)
        ),
        entry!(
            0x00d1_dee0,
            hkp_keyframed_rigid_motion_set_inertia_local(Ptr, u32)
        ),
        entry!(
            0x00d1_def0,
            hkp_keyframed_rigid_motion_set_inertia_inv_local(Ptr, u32)
        ),
        entry!(
            0x00d1_df00,
            hkp_keyframed_rigid_motion_apply_linear_impulse(Ptr, u32)
        ),
        entry!(
            0x00d1_df10,
            hkp_keyframed_rigid_motion_apply_point_impulse(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_df20,
            hkp_keyframed_rigid_motion_apply_angular_impulse(Ptr, u32)
        ),
        entry!(
            0x00d1_df30,
            hkp_keyframed_rigid_motion_apply_force(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_df40,
            hkp_keyframed_rigid_motion_apply_force_ov2(Ptr, u32, u32, u32)
        ),
        entry!(
            0x00d1_df50,
            hkp_keyframed_rigid_motion_apply_torque(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_df60,
            hkp_keyframed_rigid_motion_set_step_position(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_df70,
            hkp_keyframed_rigid_motion_set_stored_motion(Ptr, Ptr)
        ),
        entry!(0x00d1_dfa0, hkp_keyframed_rigid_motion_construct(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00d1_dff0, hkp_keyframed_rigid_motion_destruct(Ptr)),
        entry!(
            0x00d1_e010,
            hkp_keyframed_rigid_motion_get_inertia_local(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e030,
            hkp_keyframed_rigid_motion_get_inertia_world(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e050,
            hkp_keyframed_rigid_motion_get_inertia_inv_local(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e070,
            hkp_keyframed_rigid_motion_get_inertia_inv_world(Ptr, Ptr)
        ),
        entry!(
            0x00d1_e090,
            hkp_keyframed_rigid_motion_get_projected_point_velocity(Ptr, Ptr, Ptr, Ptr, Ptr)
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
    fn construct_sets_up_the_keyframed_motion() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x120);
        e.mem.write(this + 0xc0, &[0xff; 16]);
        e.mem.set_u32(this + 0x118, 0x1234);
        e.mem.set_u16(this + 0x11c, 9);
        assert_eq!(e.call(0x00d1_dfa0, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6, 0]]);
        assert_eq!(e.mem.u32(this), KEYFRAMED_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 4);
        assert_eq!(e.mem.u32(this + 0x118), 0);
        assert_eq!(e.mem.u16(this + 0x11c), 0);
        assert_eq!(e.mem.bytes(this + 0xc0, 16), vec![0; 16]);
    }

    #[test]
    fn destruct_releases_the_held_object_only_when_there_is_one() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(REMOVE_REFERENCE, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x120);
        e.call(0x00d1_dff0, &args![this]);
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u32(this), REFERENCED_OBJECT_VTABLE);
        e.mem.set_u32(this + 0x118, 0x77);
        e.call(0x00d1_dff0, &args![this]);
        assert_eq!(*seen.borrow(), vec![0x77]);
    }
}

#[cfg(test)]
mod motion_method_tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    /// Calls the stub at `address` with `words` stack words on a motion filled with a
    /// pattern and checks that nothing changed and nothing was called.
    fn assert_ignored(address: u32, words: usize) {
        let mut e = engine();
        let log: Log = new_log();
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x120);
        e.mem.write(this, &[0x5a; 0x120]);
        let mut arguments = vec![this];
        arguments.extend((0..words).map(|n| 0x1000 + n as u32));
        e.call(address, &arguments);
        assert_eq!(e.mem.bytes(this, 0x120), vec![0x5a; 0x120]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn set_mass_is_ignored_00d1dec0() {
        assert_ignored(0x00d1_dec0, 1);
    }

    #[test]
    fn set_mass_inv_is_ignored_00d1ded0() {
        assert_ignored(0x00d1_ded0, 1);
    }

    #[test]
    fn set_inertia_local_is_ignored_00d1dee0() {
        assert_ignored(0x00d1_dee0, 1);
    }

    #[test]
    fn set_inertia_inv_local_is_ignored_00d1def0() {
        assert_ignored(0x00d1_def0, 1);
    }

    #[test]
    fn apply_linear_impulse_is_ignored_00d1df00() {
        assert_ignored(0x00d1_df00, 1);
    }

    #[test]
    fn apply_point_impulse_is_ignored_00d1df10() {
        assert_ignored(0x00d1_df10, 2);
    }

    #[test]
    fn apply_angular_impulse_is_ignored_00d1df20() {
        assert_ignored(0x00d1_df20, 1);
    }

    #[test]
    fn apply_force_is_ignored_00d1df30() {
        assert_ignored(0x00d1_df30, 2);
    }

    #[test]
    fn apply_force_ov2_is_ignored_00d1df40() {
        assert_ignored(0x00d1_df40, 3);
    }

    #[test]
    fn apply_torque_is_ignored_00d1df50() {
        assert_ignored(0x00d1_df50, 2);
    }

    #[test]
    fn set_step_position_is_ignored_00d1df60() {
        assert_ignored(0x00d1_df60, 2);
    }

    #[test]
    fn set_stored_motion_swaps_the_references_00d1df70() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        log_double(&mut e, &log, REMOVE_REFERENCE, 0);
        let this = e.mem.alloc(0x120);
        e.call(0x00d1_df70, &args![this, 0x4000u32]);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x4000]]);
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(e.mem.u32(this + 0x118), 0x4000);
        // The old object is released after the new one is held.
        e.call(0x00d1_df70, &args![this, 0x5000u32]);
        assert_eq!(
            log.borrow()
                .iter()
                .map(|(a, w)| (*a, w[0]))
                .collect::<Vec<_>>(),
            [
                (ADD_REFERENCE, 0x4000),
                (ADD_REFERENCE, 0x5000),
                (REMOVE_REFERENCE, 0x4000)
            ]
        );
        assert_eq!(e.mem.u32(this + 0x118), 0x5000);
        // Clearing releases without adding.
        e.call(0x00d1_df70, &args![this, 0u32]);
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 2);
        assert_eq!(calls(&log, REMOVE_REFERENCE).len(), 2);
        assert_eq!(e.mem.u32(this + 0x118), 0);
    }

    fn assert_zero_matrix(address: u32) {
        let mut e = engine();
        let this = e.mem.alloc(0x120);
        let out = e.mem.alloc(0x40);
        e.mem.write(out, &[0xff; 0x40]);
        e.call(address, &args![this, out]);
        assert_eq!(e.mem.bytes(out, 0x30), vec![0; 0x30]);
        assert_eq!(e.mem.bytes(out + 0x30, 0x10), vec![0xff; 0x10]);
    }

    #[test]
    fn get_inertia_local_is_zero_00d1e010() {
        assert_zero_matrix(0x00d1_e010);
    }

    #[test]
    fn get_inertia_world_is_zero_00d1e030() {
        assert_zero_matrix(0x00d1_e030);
    }

    #[test]
    fn get_inertia_inv_local_is_zero_00d1e050() {
        assert_zero_matrix(0x00d1_e050);
    }

    #[test]
    fn get_inertia_inv_world_is_zero_00d1e070() {
        assert_zero_matrix(0x00d1_e070);
    }

    #[test]
    fn projected_point_velocity_adds_the_angular_and_linear_parts_00d1e090() {
        let mut e = engine();
        let motion = e.mem.alloc(0x120);
        let [point, normal, velocity, distance] = [0; 4].map(|_| e.mem.alloc(0x10));
        store_vec(&mut e, motion + 0x60, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, motion + 0xd0, [4.0, 0.5, 0.0, 0.0]);
        store_vec(&mut e, motion + 0xe0, [0.0, 0.0, 2.0, 0.0]);
        store_vec(&mut e, point, [2.0, 2.0, 3.0, 0.0]);
        e.mem.set_f32(distance, 9.0);
        // normal x arm = (0, 1, 0) x (1, 0, 0) = (0, 0, -1): -2 from the spin, 0.5 from the line.
        store_vec(&mut e, normal, [0.0, 1.0, 0.0, 0.0]);
        e.call(
            0x00d1_e090,
            &args![motion, point, normal, velocity, distance],
        );
        assert_eq!(e.mem.f32(velocity), -1.5);
        assert_eq!(e.mem.f32(distance), 0.0);
        // (1, 0, 0) x (1, 0, 0) = 0: only the line, 4.
        store_vec(&mut e, normal, [1.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00d1_e090,
            &args![motion, point, normal, velocity, distance],
        );
        assert_eq!(e.mem.f32(velocity), 4.0);
    }
}
