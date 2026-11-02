//! `Havok SDK/hkpfixedrigidmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpKeyframedRigidMotion::hkpKeyframedRigidMotion` (Xbox PDB, `00d1dfa0`, `this`,
/// `(position, rotation)`), the base constructor of the fixed motion.
const KEYFRAMED_MOTION_CONSTRUCT: u32 = 0x00d1_dfa0;
/// `hkpFixedRigidMotion`'s vtable (RTTI confirmed).
const FIXED_MOTION_VTABLE: u32 = 0x010c_6cdc;

// Translated from 00d1e160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::hkpFixedRigidMotion` (Xbox PDB): the keyframed motion constructor
/// on `this`, then the fixed motion's vtable and motion type 5 at +8. Returns `this`.
pub fn hkp_fixed_rigid_motion_construct(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    rotation: Ptr,
) -> Ptr {
    e.call(KEYFRAMED_MOTION_CONSTRUCT, &args![this, position, rotation]);
    e.mem.set_u32(this.addr(), FIXED_MOTION_VTABLE);
    e.mem.set_u8(this.addr() + 8, 5);
    this
}

/// `hkMotionState::operator=` (Xbox PDB, `00c86660`, `this` = the destination, one argument =
/// the source).
const MOTION_STATE_ASSIGN: u32 = 0x00c8_6660;

// Translated from 00d1e130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::setStepPosition` (Xbox PDB): a fixed body does not move, so this
/// does nothing. `this` and two arguments are ignored.
pub fn hkp_fixed_rigid_motion_set_step_position(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) {
}

// Translated from 00d1e140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::setLinearVelocity` (Xbox PDB): does nothing. `this` and one argument
/// are ignored.
pub fn hkp_fixed_rigid_motion_set_linear_velocity(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d1e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::setAngularVelocity` (Xbox PDB): does nothing. `this` and one argument
/// are ignored.
pub fn hkp_fixed_rigid_motion_set_angular_velocity(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d1e190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedRigidMotion::getPositionAndVelocities` (Xbox PDB): copies the motion state (`this +
/// 0x10`) into the one in `out` (`out + 0x10`, `hkMotionState::operator=`) and zeroes the
/// two velocity vectors at `out + 0xd0` and `out + 0xe0`.
pub fn hkp_fixed_rigid_motion_get_position_and_velocities(e: &mut Engine, this: Ptr, out: Ptr) {
    let out = out.addr();
    e.call(MOTION_STATE_ASSIGN, &args![out + 0x10, this.addr() + 0x10]);
    for word in 0..8 {
        e.mem.set_u32(out + 0xd0 + 4 * word, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d1_e160, hkp_fixed_rigid_motion_construct(Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00d1_e130,
            hkp_fixed_rigid_motion_set_step_position(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_e140,
            hkp_fixed_rigid_motion_set_linear_velocity(Ptr, u32)
        ),
        entry!(
            0x00d1_e150,
            hkp_fixed_rigid_motion_set_angular_velocity(Ptr, u32)
        ),
        entry!(
            0x00d1_e190,
            hkp_fixed_rigid_motion_get_position_and_velocities(Ptr, Ptr)
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
    fn construct_runs_the_keyframed_constructor_then_marks_the_motion_fixed() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(KEYFRAMED_MOTION_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d1_e160, &args![this, 5u32, 6u32]).u32(), this);
        assert_eq!(*seen.borrow(), vec![vec![this, 5, 6]]);
        assert_eq!(e.mem.u32(this), FIXED_MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 5);
    }

    /// Calls `address` on a block with the given argument words and checks nothing changed.
    fn assert_no_effect(address: u32, extra: &[u32]) {
        let mut e = test_engine();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x10, 0x1234);
        let before = e.mem.bytes(this, 0x20);
        let mut words = vec![this];
        words.extend_from_slice(extra);
        e.call(address, &words);
        assert_eq!(e.mem.bytes(this, 0x20), before);
    }

    #[test]
    fn set_step_position_does_nothing() {
        assert_no_effect(0x00d1_e130, &[1, 2]);
    }

    #[test]
    fn set_linear_velocity_does_nothing() {
        assert_no_effect(0x00d1_e140, &[1]);
    }

    #[test]
    fn set_angular_velocity_does_nothing() {
        assert_no_effect(0x00d1_e150, &[1]);
    }

    #[test]
    fn get_position_and_velocities_copies_the_motion_state_and_clears_the_velocities() {
        let mut e = test_engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(MOTION_STATE_ASSIGN, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x100);
        for word in 0..8 {
            e.mem.set_u32(out + 0xd0 + 4 * word, 0xdead_beef);
        }
        e.mem.set_u32(out + 0xf0, 0x55);
        e.call(0x00d1_e190, &args![this, out]);
        assert_eq!(*seen.borrow(), vec![vec![out + 0x10, this + 0x10]]);
        assert_eq!(e.mem.bytes(out + 0xd0, 32), vec![0; 32]);
        assert_eq!(e.mem.u32(out + 0xf0), 0x55);
    }
}
