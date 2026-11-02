//! `Havok SDK/hkpmotion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMotion` (RTTI `.?AVhkpMotion@@` confirmed at the vtable `010c59f4`): its motion state
//! (`hkMotionState`, 0xAF bytes) sits at +0x10 and the two velocity vectors at +0xD0 and
//! +0xE0; the inverse mass is the fourth lane of the vector at +0xC0 (+0xCC).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkpMotion` (RTTI-confirmed class).
const MOTION_VTABLE: u32 = 0x010c_59f4;
/// Offset of the `hkMotionState` inside the motion.
const MOTION_STATE: u32 = 0x10;
/// The constructor of the motion state built at +0x10 (`00cf2410`, `this`, `(first, second)`).
const MOTION_STATE_CONSTRUCT: u32 = 0x00cf_2410;

/// `FISTP dword` in the truncating rounding mode: the 32-bit "integer indefinite" for NaN
/// and out-of-range values.
fn truncate_to_i32(value: f32) -> i32 {
    let truncated = f64::from(value).trunc();
    if value.is_nan() || !(-2147483648.0..2147483648.0).contains(&truncated) {
        i32::MIN
    } else {
        truncated as i32
    }
}

// Translated from 00c86430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::getMass` (Xbox PDB): the reciprocal of the inverse mass at +0xCC, rounded to
/// `float`; an inverse mass equal to zero gives `0` (a NaN is not equal, so it divides).
pub fn hkp_motion_get_mass(e: &mut Engine, this: Ptr) -> f32 {
    let inverse_mass = e.mem.f32(this.addr() + 0xcc);
    if inverse_mass == 0.0 {
        return 0.0;
    }
    if inverse_mass.is_nan() {
        return x87_quiet(inverse_mass);
    }
    (1.0 / f64::from(inverse_mass)) as f32
}

// Translated from 00c86660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMotionState::operator=` (Xbox PDB): copies the first 0xA0 bytes as sixteen-byte
/// blocks, the three floats at +0xA0, +0xA4 and +0xA8 through the x87 (a signalling NaN
/// comes out quiet) and the bytes at +0xAC, +0xAD and +0xAE.
pub fn hk_motion_state_assign(e: &mut Engine, this: Ptr, source: Ptr) {
    let (to, from) = (this.addr(), source.addr());
    for offset in (0..0xa0).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xa0, 0xa4, 0xa8] {
        let bits = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, bits);
    }
    for offset in [0xac, 0xad, 0xae] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
}

// Translated from 00c86610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::getMotionStateAndVelocitiesAndDeactivationType` (Xbox PDB): copies this
/// motion's state (+0x10, `operator=`), both velocity vectors (+0xD0, +0xE0) and the byte
/// at +9 into `other`.
pub fn hkp_motion_get_motion_state_and_velocities_and_deactivation_type(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) {
    let from = Ptr::new(this.addr() + MOTION_STATE);
    let to = Ptr::new(other.addr() + MOTION_STATE);
    hk_motion_state_assign(e, to, from);
    for offset in (0..0x20).step_by(4) {
        let word = e.mem.u32(this.addr() + 0xd0 + offset);
        e.mem.set_u32(other.addr() + 0xd0 + offset, word);
    }
    let byte = e.mem.u8(this.addr() + 9);
    e.mem.set_u8(other.addr() + 9, byte);
}

// Translated from 00c86530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion` constructor, named by its body (the map gives it no name; the vtable it
/// stores is the RTTI-confirmed `hkpMotion` one): sets the word at +6 to 1, the vtable and
/// zero in +0xD0..0xEF, builds the motion state at +0x10 from `(first, second)`
/// (`00cf2410`), clears +0xB4, +0xB8 and the byte at +8. The byte at +9 is `0xFF` when
/// `use_class` is `0`, else the first float of `first` truncated, low four bits. Then clears
/// +0xA, +0xC, +0xF0..0x10F, +0x110 and +0x114 and stores `0x3F80` (the high half of 1.0f)
/// at +0x11E. Returns `this`.
pub fn fn_00c86530(e: &mut Engine, this: Ptr, first: Ptr, second: u32, use_class: u8) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base, MOTION_VTABLE);
    for offset in (0..0x20).step_by(4) {
        e.mem.set_u32(base + 0xd0 + offset, 0);
    }
    e.call(
        MOTION_STATE_CONSTRUCT,
        &args![base + MOTION_STATE, first, second],
    );
    e.mem.set_f32(base + 0xb4, 0.0);
    e.mem.set_f32(base + 0xb8, 0.0);
    e.mem.set_u8(base + 8, 0);
    if use_class == 0 {
        e.mem.set_u8(base + 9, 0xff);
    } else {
        let truncated = truncate_to_i32(e.mem.f32(first.addr()));
        e.mem.set_u8(base + 9, (truncated & 0xf) as u8);
    }
    e.mem.set_u16(base + 0xa, 0);
    e.mem.set_u16(base + 0xc, 0);
    for offset in (0..0x20).step_by(4) {
        e.mem.set_u32(base + 0xf0 + offset, 0);
    }
    e.mem.set_u32(base + 0x110, 0);
    e.mem.set_u32(base + 0x114, 0);
    e.mem.set_u16(base + 0x11e, 0x3f80);
    this
}

/// `hkSweptTransformUtil::setCentreOfRotationLocal` (cdecl, `(centre, motion state)`).
const SET_CENTRE_OF_ROTATION_LOCAL: u32 = 0x00cf_14a0;
/// `hkSweptTransformUtil::warpToPosition` (cdecl, `(position, motion state)`).
const WARP_TO_POSITION: u32 = 0x00cf_20d0;
/// `hkSweptTransformUtil::warpToRotation` (cdecl, `(rotation, motion state)`).
const WARP_TO_ROTATION: u32 = 0x00cf_2160;
/// `hkSweptTransformUtil::warpTo` (cdecl, `(position, rotation, motion state)`).
const WARP_TO: u32 = 0x00cf_1f80;
/// The transform overload of `hkSweptTransformUtil::warpTo` (cdecl, `(transform, motion
/// state)`).
const WARP_TO_TRANSFORM: u32 = 0x00cf_2020;
/// `hkSweptTransform::approxTransformAt` (`this` is the swept transform at +0x50 of the
/// motion, `(time, transform)`).
const APPROX_TRANSFORM_AT: u32 = 0x00cf_22d0;

// Translated from 00c86330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setMass` (Xbox PDB): calls the virtual at vtable offset 0x10 (the inverse-mass
/// setter) with `0.0` when `mass` equals zero (`-0.0` too) and with `1 / mass` otherwise
/// (a NaN divides, and stays NaN).
pub fn hkp_motion_set_mass(e: &mut Engine, this: Ptr, mass: f32) {
    let inverse_mass = if mass == 0.0 {
        0.0
    } else if mass.is_nan() {
        x87_quiet(mass)
    } else {
        (1.0 / f64::from(mass)) as f32
    };
    e.vcall(this.addr(), 0x10, &args![inverse_mass.to_bits()]);
}

// Translated from 00c86380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setCenterOfMassInLocal` (Xbox PDB): `setCentreOfRotationLocal(centre, motion
/// state at +0x10)`.
pub fn hkp_motion_set_center_of_mass_in_local(e: &mut Engine, this: Ptr, centre: u32) {
    e.call(
        SET_CENTRE_OF_ROTATION_LOCAL,
        &args![centre, this.addr() + MOTION_STATE],
    );
}

// Translated from 00c863a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setPosition` (Xbox PDB): `warpToPosition(position, motion state at +0x10)`.
pub fn hkp_motion_set_position(e: &mut Engine, this: Ptr, position: u32) {
    e.call(
        WARP_TO_POSITION,
        &args![position, this.addr() + MOTION_STATE],
    );
}

// Translated from 00c863c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setRotation` (Xbox PDB): `warpToRotation(rotation, motion state at +0x10)`.
pub fn hkp_motion_set_rotation(e: &mut Engine, this: Ptr, rotation: u32) {
    e.call(
        WARP_TO_ROTATION,
        &args![rotation, this.addr() + MOTION_STATE],
    );
}

// Translated from 00c863e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setPositionAndRotation` (Xbox PDB): `warpTo(position, rotation, motion state
/// at +0x10)`.
pub fn hkp_motion_set_position_and_rotation(
    e: &mut Engine,
    this: Ptr,
    position: u32,
    rotation: u32,
) {
    e.call(
        WARP_TO,
        &args![position, rotation, this.addr() + MOTION_STATE],
    );
}

// Translated from 00c86400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setTransform` (Xbox PDB): the transform overload of `warpTo(transform, motion
/// state at +0x10)`.
pub fn hkp_motion_set_transform(e: &mut Engine, this: Ptr, transform: u32) {
    e.call(
        WARP_TO_TRANSFORM,
        &args![transform, this.addr() + MOTION_STATE],
    );
}

// Translated from 00c86420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setDeactivationClass` (Xbox PDB): stores the byte at +0xBE.
pub fn hkp_motion_set_deactivation_class(e: &mut Engine, this: Ptr, class: u8) {
    e.mem.set_u8(this.addr() + 0xbe, class);
}

// Translated from 00c86480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setMassInv` (Xbox PDB): stores the float at +0xCC through the x87 (a
/// signalling NaN comes out quiet).
pub fn hkp_motion_set_mass_inv(e: &mut Engine, this: Ptr, inverse_mass: u32) {
    e.mem
        .set_u32(this.addr() + 0xcc, x87_quiet_bits(inverse_mass));
}

// Translated from 00c86490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::approxTransformAt` (Xbox PDB): `this` in `ECX`; calls
/// `hkSweptTransform::approxTransformAt` on the swept transform at +0x50 with the time (a
/// float, copied through the x87) and `transform`.
pub fn hkp_motion_approx_transform_at(e: &mut Engine, this: Ptr, time: u32, transform: u32) {
    e.call(
        APPROX_TRANSFORM_AT,
        &args![this.addr() + 0x50, x87_quiet_bits(time), transform],
    );
}

// Translated from 00c864b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setLinearVelocity` (Xbox PDB): copies the sixteen-byte vector at `velocity`
/// (aligned) to +0xD0.
pub fn hkp_motion_set_linear_velocity(e: &mut Engine, this: Ptr, velocity: Ptr) {
    for lane in 0..4 {
        let word = e.mem.u32(velocity.addr() + 4 * lane);
        e.mem.set_u32(this.addr() + 0xd0 + 4 * lane, word);
    }
}

// Translated from 00c864d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::setAngularVelocity` (Xbox PDB): copies the sixteen-byte vector at `velocity`
/// (aligned) to +0xE0.
pub fn hkp_motion_set_angular_velocity(e: &mut Engine, this: Ptr, velocity: Ptr) {
    for lane in 0..4 {
        let word = e.mem.u32(velocity.addr() + 4 * lane);
        e.mem.set_u32(this.addr() + 0xe0 + 4 * lane, word);
    }
}

// Translated from 00c864f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMotion::applyLinearImpulse` (Xbox PDB): adds `impulse * inverse mass` (the fourth lane
/// of the vector at +0xC0, +0xCC) to the linear velocity at +0xD0, four lanes of `MULPS` and
/// `ADDPS`.
pub fn hkp_motion_apply_linear_impulse(e: &mut Engine, this: Ptr, impulse: Ptr) {
    // `MULPS` with the splatted inverse mass as the destination, `ADDPS` onto the
    // velocity: the SSE NaN rules.
    let inverse_mass = e.mem.f32(this.addr() + 0xcc);
    for lane in 0..4 {
        let scaled = crate::sse::mul(inverse_mass, e.mem.f32(impulse.addr() + 4 * lane));
        let velocity = e.mem.f32(this.addr() + 0xd0 + 4 * lane);
        e.mem.set_f32(
            this.addr() + 0xd0 + 4 * lane,
            crate::sse::add(scaled, velocity),
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8_6430, hkp_motion_get_mass(Ptr) -> f32),
        entry!(0x00c8_6660, hk_motion_state_assign(Ptr, Ptr)),
        entry!(
            0x00c8_6610,
            hkp_motion_get_motion_state_and_velocities_and_deactivation_type(Ptr, Ptr)
        ),
        entry!(0x00c8_6530, fn_00c86530(Ptr, Ptr, u32, u8) -> Ptr),
        entry!(0x00c8_6330, hkp_motion_set_mass(Ptr, f32)),
        entry!(
            0x00c8_6380,
            hkp_motion_set_center_of_mass_in_local(Ptr, u32)
        ),
        entry!(0x00c8_63a0, hkp_motion_set_position(Ptr, u32)),
        entry!(0x00c8_63c0, hkp_motion_set_rotation(Ptr, u32)),
        entry!(
            0x00c8_63e0,
            hkp_motion_set_position_and_rotation(Ptr, u32, u32)
        ),
        entry!(0x00c8_6400, hkp_motion_set_transform(Ptr, u32)),
        entry!(0x00c8_6420, hkp_motion_set_deactivation_class(Ptr, u8)),
        entry!(0x00c8_6480, hkp_motion_set_mass_inv(Ptr, u32)),
        entry!(0x00c8_6490, hkp_motion_approx_transform_at(Ptr, u32, u32)),
        entry!(0x00c8_64b0, hkp_motion_set_linear_velocity(Ptr, Ptr)),
        entry!(0x00c8_64d0, hkp_motion_set_angular_velocity(Ptr, Ptr)),
        entry!(0x00c8_64f0, hkp_motion_apply_linear_impulse(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mass_is_the_reciprocal_unless_the_inverse_mass_is_zero() {
        let mut e = Engine::new();
        let motion = e.mem.alloc(0x130);
        e.mem.set_f32(motion + 0xcc, 4.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32(), 0.25);
        e.mem.set_f32(motion + 0xcc, 0.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32(), 0.0);
        e.mem.set_f32(motion + 0xcc, -0.0);
        assert_eq!(e.call(0x00c8_6430, &args![motion]).f32().to_bits(), 0);
        e.mem.set_f32(motion + 0xcc, f32::NAN);
        assert!(e.call(0x00c8_6430, &args![motion]).f32().is_nan());
    }

    #[test]
    fn state_assign_copies_the_block_and_quiets_the_three_floats() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0xb0);
        let dest = e.mem.alloc(0xb0);
        for offset in (0..0xb0).step_by(4) {
            e.mem.set_u32(source + offset, 0x1000 + offset);
        }
        e.mem.set_u32(source + 0xa0, 0x7f80_0001);
        e.mem.set_u32(source + 0xa4, 0x3f80_0000);
        e.mem.set_u32(source + 0xa8, 0xff80_1234);
        e.mem.set_u8(source + 0xac, 1);
        e.mem.set_u8(source + 0xad, 2);
        e.mem.set_u8(source + 0xae, 3);
        e.call(0x00c8_6660, &args![dest, source]);
        for offset in (0..0xa0).step_by(4) {
            assert_eq!(e.mem.u32(dest + offset), 0x1000 + offset);
        }
        assert_eq!(e.mem.u32(dest + 0xa0), 0x7fc0_0001);
        assert_eq!(e.mem.u32(dest + 0xa4), 0x3f80_0000);
        assert_eq!(e.mem.u32(dest + 0xa8), 0xffc0_1234);
        let tail = [
            e.mem.u8(dest + 0xac),
            e.mem.u8(dest + 0xad),
            e.mem.u8(dest + 0xae),
        ];
        assert_eq!(tail, [1, 2, 3]);
    }

    #[test]
    fn get_motion_state_copies_state_velocities_and_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x130);
        let other = e.mem.alloc(0x130);
        for offset in (0x10..0xc0).step_by(4) {
            e.mem.set_u32(this + offset, 0x2000 + offset);
        }
        for offset in (0xd0..0xf0).step_by(4) {
            e.mem.set_u32(this + offset, 0x3000 + offset);
        }
        e.mem.set_u8(this + 9, 7);
        e.mem.set_u32(other + 0xcc, 0x55);
        e.call(0x00c8_6610, &args![this, other]);
        assert_eq!(e.mem.u32(other + 0x10), 0x2010);
        assert_eq!(e.mem.u32(other + 0xa0), 0x2000 + 0xa0);
        assert_eq!(e.mem.u32(other + 0xd0), 0x30d0);
        assert_eq!(e.mem.u32(other + 0xec), 0x30ec);
        assert_eq!(e.mem.u8(other + 9), 7);
        assert_eq!(e.mem.u32(other + 0xcc), 0x55);
    }

    fn construct(first_value: f32, use_class: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x130);
        for offset in (0..0x130).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let first = e.mem.alloc(0x10);
        e.mem.set_f32(first, first_value);
        e.register(MOTION_STATE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c8_6530, &args![this, first, 0x1234u32, use_class]);
        (e, this)
    }

    #[test]
    fn constructor_sets_the_fields_and_the_class_byte() {
        let (mut e, this) = construct(37.9, 1);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), MOTION_VTABLE);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 37 & 0xf);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 0);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        for offset in (0xd0..0xf0).step_by(4).chain((0xf0..0x118).step_by(4)) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u16(this + 0x11e), 0x3f80);
        assert_eq!(e.mem.u32(this + 0x118), 0xdead_beef);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == MOTION_STATE_CONSTRUCT)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this + MOTION_STATE);
    }

    #[test]
    fn constructor_class_byte_is_ff_without_the_flag_and_indefinite_gives_zero() {
        let (e, this) = construct(5.0, 0);
        assert_eq!(e.mem.u8(this + 9), 0xff);
        let (e, this) = construct(f32::NAN, 1);
        assert_eq!(e.mem.u8(this + 9), 0);
        let (e, this) = construct(1.0e20, 1);
        assert_eq!(e.mem.u8(this + 9), 0);
    }

    /// A motion whose vtable has a double at slot +0x10 (the inverse-mass setter).
    fn motion_with_setter() -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0001);
        e.register_double(0x0900_0001, |_, _| Ret::default());
        let motion = e.mem.alloc(0x130);
        e.mem.set_u32(motion, vtable);
        e.call_log = Some(vec![]);
        (e, motion)
    }

    fn setter_calls(e: &mut Engine) -> Vec<Vec<u32>> {
        let log = e.call_log.take().unwrap();
        log.into_iter()
            .filter(|c| c.0 == 0x0900_0001)
            .map(|c| c.1)
            .collect()
    }

    #[test]
    fn set_mass_passes_the_reciprocal_or_zero_to_the_virtual() {
        let (mut e, motion) = motion_with_setter();
        e.call(0x00c8_6330, &args![motion, 4.0f32.to_bits()]);
        e.call(0x00c8_6330, &args![motion, 0.0f32.to_bits()]);
        e.call(0x00c8_6330, &args![motion, (-0.0f32).to_bits()]);
        e.call(0x00c8_6330, &args![motion, 3.0f32.to_bits()]);
        e.call(0x00c8_6330, &args![motion, 0x7f80_0001u32]);
        let calls = setter_calls(&mut e);
        assert_eq!(
            calls,
            [
                vec![motion, 0.25f32.to_bits()],
                vec![motion, 0],
                vec![motion, 0],
                vec![motion, (1.0f32 / 3.0).to_bits()],
                vec![motion, 0x7fc0_0001],
            ]
        );
    }

    /// Registers a recording double for each address.
    fn record_calls(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register_double(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
    }

    fn logged(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn the_warp_setters_pass_their_argument_and_the_motion_state() {
        let mut e = Engine::new();
        record_calls(
            &mut e,
            &[
                SET_CENTRE_OF_ROTATION_LOCAL,
                WARP_TO_POSITION,
                WARP_TO_ROTATION,
                WARP_TO,
                WARP_TO_TRANSFORM,
            ],
        );
        let motion = e.mem.alloc(0x130);
        e.call(0x00c8_6380, &args![motion, 0x11u32]);
        e.call(0x00c8_63a0, &args![motion, 0x22u32]);
        e.call(0x00c8_63c0, &args![motion, 0x33u32]);
        e.call(0x00c8_63e0, &args![motion, 0x44u32, 0x55u32]);
        e.call(0x00c8_6400, &args![motion, 0x66u32]);
        let state = motion + 0x10;
        assert_eq!(
            logged(&e, SET_CENTRE_OF_ROTATION_LOCAL),
            [vec![0x11, state]]
        );
        assert_eq!(logged(&e, WARP_TO_POSITION), [vec![0x22, state]]);
        assert_eq!(logged(&e, WARP_TO_ROTATION), [vec![0x33, state]]);
        assert_eq!(logged(&e, WARP_TO), [vec![0x44, 0x55, state]]);
        assert_eq!(logged(&e, WARP_TO_TRANSFORM), [vec![0x66, state]]);
    }

    #[test]
    fn set_deactivation_class_and_mass_inv_store_their_fields() {
        let mut e = Engine::new();
        let motion = e.mem.alloc(0x130);
        e.call(0x00c8_6420, &args![motion, 7u8]);
        assert_eq!(e.mem.u8(motion + 0xbe), 7);
        e.call(0x00c8_6480, &args![motion, 0.5f32.to_bits()]);
        assert_eq!(e.mem.f32(motion + 0xcc), 0.5);
        e.call(0x00c8_6480, &args![motion, 0xff80_0001u32]);
        assert_eq!(e.mem.u32(motion + 0xcc), 0xffc0_0001);
    }

    #[test]
    fn approx_transform_at_calls_the_swept_transform_with_the_time() {
        let mut e = Engine::new();
        record_calls(&mut e, &[APPROX_TRANSFORM_AT]);
        let motion = e.mem.alloc(0x130);
        e.call(0x00c8_6490, &args![motion, 1.5f32.to_bits(), 0x77u32]);
        e.call(0x00c8_6490, &args![motion, 0x7f80_0001u32, 0x78u32]);
        assert_eq!(
            logged(&e, APPROX_TRANSFORM_AT),
            [
                vec![motion + 0x50, 1.5f32.to_bits(), 0x77],
                vec![motion + 0x50, 0x7fc0_0001, 0x78],
            ]
        );
    }

    #[test]
    fn velocity_setters_copy_four_lanes() {
        let mut e = Engine::new();
        let motion = e.mem.alloc(0x130);
        let vector = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(vector + 4 * lane, 0x7f80_0001 + lane);
        }
        e.call(0x00c8_64b0, &args![motion, vector]);
        e.call(0x00c8_64d0, &args![motion, vector]);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(motion + 0xd0 + 4 * lane), 0x7f80_0001 + lane);
            assert_eq!(e.mem.u32(motion + 0xe0 + 4 * lane), 0x7f80_0001 + lane);
        }
        assert_eq!(e.mem.u32(motion + 0xcc), 0);
        assert_eq!(e.mem.u32(motion + 0xf0), 0);
    }

    #[test]
    fn apply_linear_impulse_adds_the_scaled_impulse_to_the_velocity() {
        let mut e = Engine::new();
        let motion = e.mem.alloc(0x130);
        let impulse = e.mem.alloc(0x10);
        e.mem.set_f32(motion + 0xcc, 0.5);
        for (lane, value) in [2.0f32, -4.0, 6.0, 8.0].into_iter().enumerate() {
            e.mem.set_f32(impulse + 4 * lane as u32, value);
            e.mem.set_f32(motion + 0xd0 + 4 * lane as u32, 1.0);
        }
        e.call(0x00c8_64f0, &args![motion, impulse]);
        let velocity: Vec<f32> = (0..4)
            .map(|lane| e.mem.f32(motion + 0xd0 + 4 * lane))
            .collect();
        assert_eq!(velocity, [2.0, -1.0, 4.0, 5.0]);
    }
}
