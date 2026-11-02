//! `Havok SDK/hkpvehicledefaultvelocitydamper.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultVelocityDamper`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultVelocityDamper` (names unconfirmed).
    pub struct HkpVehicleDefaultVelocityDamper: 0x14 {
        /// `m_normalSpinDamping`.
        0x08 m_normal_spin_damping: f32,
        /// `m_collisionSpinDamping`.
        0x0c m_collision_spin_damping: f32,
        /// `m_collisionThreshold`.
        0x10 m_collision_threshold: f32,
    }
}

/// `hkpRigidBody::setAngularVelocity` (Xbox PDB, `this` = the body, one pointer to the vector).
const SET_ANGULAR_VELOCITY: u32 = 0x0056_1800;

// Translated from 00e43ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultVelocityDamper::applyVelocityDamping` (Xbox PDB), `(this, delta time,
/// instance)`. `body` = the word at `instance + 0x18`; its angular velocity `a` is the four
/// floats at `body + 0x1C0`. With `s = a.z² + (a.y² + a.x²)` (four-lane `float` products) and
/// `c` = `collisionThreshold` squared (rounded to `float`), the damping is
/// `collisionSpinDamping` when `c < s` and `normalSpinDamping` otherwise (also for a NaN);
/// `f = float(1 - damping * delta time)`, `0.0` when `f < 0.0`. The body's angular velocity is
/// set (`setAngularVelocity` with a pointer to a stack vector) to `f * a` in all four lanes.
pub fn apply_velocity_damping(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultVelocityDamper>,
    delta_time: f32,
    instance: Ptr,
) {
    let body = e.mem.u32(instance.addr() + 0x18);
    let threshold = x87_quiet(e.get(this, HkpVehicleDefaultVelocityDamper::m_collision_threshold));
    let spin: [f32; 4] =
        std::array::from_fn(|i| f32::from_bits(e.mem.u32(body + 0x1c0 + 4 * i as u32)));
    let squares: [f32; 4] = std::array::from_fn(|i| spin[i] * spin[i]);
    let spin_squared = squares[2] + (squares[1] + squares[0]);
    let threshold_squared = (f64::from(threshold) * f64::from(threshold)) as f32;
    let damping = if threshold_squared < spin_squared {
        e.get(
            this,
            HkpVehicleDefaultVelocityDamper::m_collision_spin_damping,
        )
    } else {
        e.get(this, HkpVehicleDefaultVelocityDamper::m_normal_spin_damping)
    };
    let mut factor = (1.0 - f64::from(x87_quiet(damping)) * f64::from(delta_time)) as f32;
    if factor < 0.0 {
        factor = 0.0;
    }
    e.with_stack(0x10, |e, vector| {
        for (i, lane) in spin.iter().enumerate() {
            e.mem.set_f32(vector.addr() + 4 * i as u32, factor * lane);
        }
        e.call(SET_ANGULAR_VELOCITY, &args![body, vector]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e4_3ae0,
        apply_velocity_damping(Ptr<HkpVehicleDefaultVelocityDamper>, f32, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Runs the damper with normal damping `normal`, collision damping 2, threshold 2 (so 4
    /// squared) and delta time `dt` on the angular velocity `spin`; returns the body, the
    /// call's `this` and the vector it was given.
    fn run(normal: f32, dt: f32, spin: [f32; 4]) -> (u32, u32, [f32; 4]) {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        e.mem.set_f32(this + 8, normal);
        e.mem.set_f32(this + 0xc, 2.0);
        e.mem.set_f32(this + 0x10, 2.0);
        let body = e.mem.alloc(0x200);
        for (i, lane) in spin.iter().enumerate() {
            e.mem.set_f32(body + 0x1c0 + 4 * i as u32, *lane);
        }
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x18, body);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SET_ANGULAR_VELOCITY, move |e, a| {
            let vector: [f32; 4] = std::array::from_fn(|i| e.mem.f32(a[1] + 4 * i as u32));
            sink.borrow_mut().push((a[0], vector));
            Ret::default()
        });
        e.call(0x00e4_3ae0, &args![this, dt, instance]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        (body, seen[0].0, seen[0].1)
    }

    #[test]
    fn a_fast_spin_gets_the_collision_damping() {
        // s = 14 > 4: damping 2, f = 1 - 2 * 0.25 = 0.5.
        let (body, this, vector) = run(0.5, 0.25, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(this, body);
        assert_eq!(vector, [0.5, 1.0, 1.5, 2.0]);
    }

    #[test]
    fn a_slow_spin_gets_the_normal_damping() {
        // s = 0.25 <= 4: damping 0.5, f = 1 - 0.5 * 0.25 = 0.875; the fourth lane is scaled too.
        let (_, _, vector) = run(0.5, 0.25, [0.5, 0.0, 0.0, 1.0]);
        assert_eq!(vector, [0.4375, 0.0, 0.0, 0.875]);
        // A NaN spin takes the normal damping as well.
        let (_, _, vector) = run(0.5, 0.25, [f32::NAN, 0.0, 0.0, 1.0]);
        assert!(vector[0].is_nan());
        assert_eq!(vector[3], 0.875);
    }

    #[test]
    fn the_factor_never_goes_below_zero() {
        let (_, _, vector) = run(8.0, 1.0, [0.5, 0.0, 0.0, 1.0]);
        assert_eq!(vector, [0.0, 0.0, 0.0, 0.0]);
    }
}
