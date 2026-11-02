//! `Havok SDK/hkpvehicledefaultsteering.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultSteering`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultSteering` (names unconfirmed); slot `0x10` of its vtable is `calcMainSteeringAngle`.
    pub struct HkpVehicleDefaultSteering: 0x18 {
        /// `m_maxSteeringAngle`.
        0x08 m_max_steering_angle: f32,
        /// `m_maxSpeedFullSteeringAngle`.
        0x0c m_max_speed_full_steering_angle: f32,
        /// `m_doesWheelSteer` data (one byte per wheel).
        0x10 m_does_wheel_steer: Ptr,
        /// `m_doesWheelSteer` size.
        0x14 m_does_wheel_steer_size: i32,
    }
}

/// `hkVector4::setRotatedDir` (Xbox PDB name of `00c7f700`, `this` = the result vector, then the
/// rotation and the vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;

// Translated from 00e43f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultSteering::calcMainSteeringAngle` (Xbox PDB), `(this, _, instance, input,
/// output)`; the first stack word is not read. `body` = the word at `instance + 0x18` and
/// `data` = the word at `instance + 0x1C`.
/// 1. `output[0] = float(input[2] * maxSteeringAngle)` (the third float of `input`);
/// 2. `d` = the vector `data + 0x40` rotated by the matrix at `body + 0xF0`
///    (`setRotatedDir`), `p` = the four-lane product of the body's velocity (`+0x1B0`, read
///    again from the instance) with `d` and `s = (p.y + p.x) + p.z`;
/// 3. when `maxSpeedFullSteeringAngle < s` (false for a NaN) the angle is scaled by the square
///    of `q = float(maxSpeedFullSteeringAngle / s)`: `output[0] = float((output[0] * q) * q)`.
pub fn calc_main_steering_angle(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultSteering>,
    _unused_1: u32,
    instance: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let body = e.mem.u32(instance.addr() + 0x18);
    let data = e.mem.u32(instance.addr() + 0x1c);
    let steer = f64::from(x87_quiet(e.mem.f32(input.addr() + 8)));
    let max_angle = f64::from(e.get(this, HkpVehicleDefaultSteering::m_max_steering_angle));
    e.mem.set_f32(output.addr(), (steer * max_angle) as f32);
    let direction: [f32; 4] = e.with_stack(0x10, |e, frame| {
        e.call(SET_ROTATED_DIR, &args![frame, body + 0xf0, data + 0x40]);
        std::array::from_fn(|i| e.mem.f32(frame.addr() + 4 * i as u32))
    });
    let body = e.mem.u32(instance.addr() + 0x18);
    let product: [f32; 4] =
        std::array::from_fn(|i| e.mem.f32(body + 0x1b0 + 4 * i as u32) * direction[i]);
    let speed = (product[1] + product[0]) + product[2];
    let max_speed = e.get(
        this,
        HkpVehicleDefaultSteering::m_max_speed_full_steering_angle,
    );
    if max_speed < speed {
        let scale = (f64::from(x87_quiet(max_speed)) / f64::from(x87_quiet(speed))) as f32;
        let angle = f64::from(x87_quiet(e.mem.f32(output.addr())));
        let scaled = ((angle * f64::from(scale)) * f64::from(scale)) as f32;
        e.mem.set_f32(output.addr(), scaled);
    }
}

// Translated from 00e44010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultSteering::calcSteering` (Xbox PDB), `(this, delta time, instance, input,
/// output)`. Calls slot `0x10` of `this` (`calcMainSteeringAngle`) with the same four words
/// (the float passed as its bits, through the x87), then, for every wheel `i` of
/// `m_doesWheelSteer` (the count is read again after every wheel): `output[1][i]` = `output[0]`
/// (through the x87) when its flag byte is non-zero, else `0.0`. `output[1]` is the per-wheel
/// angle array's data pointer.
pub fn calc_steering(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultSteering>,
    delta_time: f32,
    instance: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let bits = x87_quiet_bits(delta_time.to_bits());
    e.vcall(this.addr(), 0x10, &args![bits, instance, input, output]);
    let mut wheel = 0u32;
    if e.get(this, HkpVehicleDefaultSteering::m_does_wheel_steer_size) > 0 {
        loop {
            let flags = e
                .get(this, HkpVehicleDefaultSteering::m_does_wheel_steer)
                .addr();
            let angles = e.mem.u32(output.addr() + 4);
            if e.mem.u8(flags.wrapping_add(wheel)) != 0 {
                let angle = x87_quiet_bits(e.mem.u32(output.addr()));
                e.mem.set_u32(angles.wrapping_add(wheel * 4), angle);
            } else {
                e.mem.set_u32(angles.wrapping_add(wheel * 4), 0);
            }
            wheel += 1;
            if wheel as i32 >= e.get(this, HkpVehicleDefaultSteering::m_does_wheel_steer_size) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_3f70,
            calc_main_steering_angle(Ptr<HkpVehicleDefaultSteering>, u32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e4_4010,
            calc_steering(Ptr<HkpVehicleDefaultSteering>, f32, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, new_log};

    /// An instance whose body (rotation at `+0xF0`, velocity `(2, 3, 4, 5)` at `+0x1B0`) and
    /// data are allocated; `setRotatedDir` answers the vector `(1, 2, 3, 9)`. Returns
    /// `(this, instance, input, output, body, data)` with `this.maxSteeringAngle = 0.5`,
    /// `this.maxSpeedFullSteeringAngle = max_speed` and the input's third float 1.
    fn rig(e: &mut Engine, max_speed: f32) -> (u32, u32, u32, u32, u32, u32) {
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + 8, 0.5);
        e.mem.set_f32(this + 0xc, max_speed);
        let body = e.mem.alloc(0x200);
        for (i, lane) in [2.0f32, 3.0, 4.0, 5.0].iter().enumerate() {
            e.mem.set_f32(body + 0x1b0 + 4 * i as u32, *lane);
        }
        let data = e.mem.alloc(0x80);
        let instance = e.mem.alloc(0x40);
        e.mem.set_u32(instance + 0x18, body);
        e.mem.set_u32(instance + 0x1c, data);
        let input = e.mem.alloc(0x10);
        e.mem.set_f32(input + 8, 1.0);
        let output = e.mem.alloc(0x10);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            assert_eq!(a[1..], [body + 0xf0, data + 0x40]);
            for (i, lane) in [1.0f32, 2.0, 3.0, 9.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, *lane);
            }
            Ret::default()
        });
        (this, instance, input, output, body, data)
    }

    #[test]
    fn the_angle_is_scaled_down_above_the_full_steering_speed() {
        let mut e = engine();
        // s = (6 + 2) + 12 = 20 is above 10: q = 0.5, so 0.5 * 0.5 * 0.5.
        let (this, instance, input, output, _, _) = rig(&mut e, 10.0);
        e.call(0x00e4_3f70, &args![this, 0u32, instance, input, output]);
        assert_eq!(e.mem.f32(output), 0.125);
        // At or above s the angle stays 0.5; so it does for a NaN limit.
        for limit in [20.0f32, 30.0, f32::NAN] {
            let (this, instance, input, output, _, _) = rig(&mut e, limit);
            e.call(0x00e4_3f70, &args![this, 0u32, instance, input, output]);
            assert_eq!(e.mem.f32(output), 0.5);
        }
    }

    #[test]
    fn each_steering_wheel_gets_the_main_angle() {
        let mut e = engine();
        let log = new_log();
        let this = e.mem.alloc(0x18);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        e.mem.set_u32(this, vtable);
        let flags = e.mem.alloc(8);
        e.mem.set_u8(flags, 1);
        e.mem.set_u8(flags + 2, 1);
        e.mem.set_u32(this + 0x10, flags);
        e.mem.set_u32(this + 0x14, 3);
        let angles = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(angles + 4 * i, 9.0);
        }
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 4, angles);
        let sink = std::rc::Rc::clone(&log);
        e.register_double(0x0900_0010, move |e, a| {
            sink.borrow_mut().push((0x0900_0010, a.to_vec()));
            e.mem.set_f32(a[4], 0.75);
            Ret::default()
        });
        e.call(
            0x00e4_4010,
            &args![this, 0.25f32, 0x1111u32, 0x2222u32, output],
        );
        assert_eq!(
            calls(&log, 0x0900_0010),
            vec![vec![this, 0.25f32.to_bits(), 0x1111, 0x2222, output]]
        );
        assert_eq!(
            [
                e.mem.f32(angles),
                e.mem.f32(angles + 4),
                e.mem.f32(angles + 8)
            ],
            [0.75, 0.0, 0.75]
        );
        // No wheels: nothing is written.
        e.mem.set_u32(this + 0x14, 0);
        e.mem.set_f32(angles, 9.0);
        e.call(0x00e4_4010, &args![this, 0.25f32, 1u32, 2u32, output]);
        assert_eq!(e.mem.f32(angles), 9.0);
    }
}
