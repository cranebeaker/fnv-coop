//! `Havok SDK/hkpvehicledefaultsuspension.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultSuspension`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultSuspension` (names unconfirmed): two arrays, one entry per wheel.
    pub struct HkpVehicleDefaultSuspension: 0x20 {
        /// `m_wheelParams` data: 0x30 bytes per wheel, the suspension length at `+0x20`.
        0x08 m_wheel_params: Ptr,
        /// `m_wheelSpringParams` data: three floats per wheel (strength, damping when
        /// compressing, damping when relaxing).
        0x14 m_wheel_spring_params: Ptr,
    }
}

/// `hkpMotion::getMass` (Xbox PDB, `this` = the motion, the mass comes back in `ST0`).
const MOTION_GET_MASS: u32 = 0x00c8_6430;

// Translated from 00e397f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultSuspension::calcSuspension` (Xbox PDB), `(this, _, instance, records,
/// forces)`; the first stack word is not read. `body` = the word at `instance + 0x18` and
/// `data` = the word at `instance + 0x1C` (a signed byte at `data + 0x20` is the wheel count,
/// read again after every wheel). The mass comes from `hkpMotion::getMass` of `body + 0xE0`
/// (stored as a `float`). For wheel `j` the record is the 0x60 bytes at `records + 0x48 +
/// 0x60 * j - 0x24` (so `+0x24` is a word, `+0x48` the compression `c`, `+0x4C` the speed `v`,
/// `+0x50` a factor `f`); a record whose word at `+0x24` is zero gives a force of `0.0`, any
/// other gives (every `-` and `*` in double precision, stored as `float` where marked):
/// `t = float(length - c)`, `a = float(t * strength * f)`, with `d` = the relaxing damping
/// when `v` is not below zero (also for a NaN) and the compressing damping otherwise,
/// `r = float(a - v * d)` and the force `float(r * mass)` written to `forces[j]`.
pub fn calc_suspension(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultSuspension>,
    _unused_1: u32,
    instance: Ptr,
    records: Ptr,
    forces: Ptr,
) {
    let body = e.mem.u32(instance.addr() + 0x18);
    let mass = e.call(MOTION_GET_MASS, &args![body + 0xe0]).f32();
    let data = e.mem.u32(instance.addr() + 0x1c);
    let wheel_count = |e: &Engine| i32::from(e.mem.u8(data + 0x20) as i8);
    if wheel_count(e) <= 0 {
        return;
    }
    let mass = f64::from(x87_quiet(mass));
    let mut wheel = 0i32;
    loop {
        let record = records
            .addr()
            .wrapping_add(0x48)
            .wrapping_add((wheel as u32).wrapping_mul(0x60));
        let force_at = forces.addr().wrapping_add((wheel as u32).wrapping_mul(4));
        if e.mem.u32(record.wrapping_sub(0x24)) == 0 {
            e.mem.set_f32(force_at, 0.0);
        } else {
            let springs = e
                .get(this, HkpVehicleDefaultSuspension::m_wheel_spring_params)
                .addr()
                .wrapping_add((wheel as u32).wrapping_mul(0xc));
            let params = e
                .get(this, HkpVehicleDefaultSuspension::m_wheel_params)
                .addr();
            let length = f64::from(x87_quiet(
                e.mem
                    .f32(params.wrapping_add((wheel as u32).wrapping_mul(0x30)) + 0x20),
            ));
            let compression = f64::from(e.mem.f32(record));
            let stretch = x87_quiet((length - compression) as f32);
            let strength = f64::from(e.mem.f32(springs));
            let factor = f64::from(e.mem.f32(record + 8));
            let spring = (f64::from(stretch) * strength * factor) as f32;
            let speed = x87_quiet(e.mem.f32(record + 4));
            let damping_at = if speed < 0.0 {
                springs + 4
            } else {
                springs + 8
            };
            let damping = x87_quiet(e.mem.f32(damping_at));
            let net = (f64::from(spring) - f64::from(speed) * f64::from(damping)) as f32;
            let force = (f64::from(x87_quiet(net)) * mass) as f32;
            e.mem.set_f32(force_at, force);
        }
        wheel += 1;
        if wheel >= wheel_count(e) {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_97f0,
        calc_suspension(Ptr<HkpVehicleDefaultSuspension>, u32, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;

    /// Two wheels. Wheel 0: length 1, strength 100, dampings 10 (compressing) and 20
    /// (relaxing); wheel 1 has a zero word in its record. The record of wheel 0 has
    /// compression 0.25, factor 2 and the given speed; the mass is 2.
    /// Returns `(this, instance, records, forces, body)`.
    fn rig(e: &mut Engine, speed: f32) -> (u32, u32, u32, u32, u32) {
        let this = e.mem.alloc(0x20);
        let params = e.mem.alloc(0x60);
        e.mem.set_f32(params + 0x20, 1.0);
        e.mem.set_f32(params + 0x30 + 0x20, 5.0);
        let springs = e.mem.alloc(0x18);
        e.mem.set_f32(springs, 100.0);
        e.mem.set_f32(springs + 4, 10.0);
        e.mem.set_f32(springs + 8, 20.0);
        e.mem.set_u32(this + 8, params);
        e.mem.set_u32(this + 0x14, springs);
        let body = e.mem.alloc(0x200);
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, 2);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x18, body);
        e.mem.set_u32(instance + 0x1c, data);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(records + 0x24, 1);
        e.mem.set_f32(records + 0x48, 0.25);
        e.mem.set_f32(records + 0x4c, speed);
        e.mem.set_f32(records + 0x50, 2.0);
        let forces = e.mem.alloc(0x10);
        e.mem.set_f32(forces + 4, 7.0);
        e.register_double(MOTION_GET_MASS, move |_, a| {
            assert_eq!(a, [body + 0xe0]);
            Ret {
                st0: 2.0,
                ..Ret::default()
            }
        });
        (this, instance, records, forces, body)
    }

    #[test]
    fn a_compressing_wheel_uses_the_first_damping() {
        let mut e = engine();
        let (this, instance, records, forces, _) = rig(&mut e, -4.0);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        // t = 0.75, a = 0.75 * 100 * 2 = 150, r = 150 - (-4 * 10) = 190, force 190 * 2.
        assert_eq!(e.mem.f32(forces), 380.0);
        // The second wheel's record has a zero word: 0.0 replaces the 7.0.
        assert_eq!(e.mem.f32(forces + 4), 0.0);
    }

    #[test]
    fn a_relaxing_or_resting_wheel_uses_the_second_damping() {
        let mut e = engine();
        let (this, instance, records, forces, _) = rig(&mut e, 4.0);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        // r = 150 - 4 * 20 = 70.
        assert_eq!(e.mem.f32(forces), 140.0);
        e.mem.set_f32(records + 0x4c, 0.0);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        assert_eq!(e.mem.f32(forces), 300.0);
        // A NaN speed is not below zero either.
        e.mem.set_f32(records + 0x4c, f32::NAN);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        assert!(e.mem.f32(forces).is_nan());
    }

    #[test]
    fn no_wheels_leave_the_forces_alone() {
        let mut e = engine();
        let (this, instance, records, forces, _) = rig(&mut e, 4.0);
        let data = e.mem.u32(instance + 0x1c);
        e.mem.set_u8(data + 0x20, 0);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        assert_eq!(e.mem.f32(forces + 4), 7.0);
        // A negative count (signed byte) is the same.
        e.mem.set_u8(data + 0x20, 0xff);
        e.call(0x00e3_97f0, &args![this, 0u32, instance, records, forces]);
        assert_eq!(e.mem.f32(forces + 4), 7.0);
    }
}
