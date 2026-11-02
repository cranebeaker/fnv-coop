//! `Havok SDK/hkpvehicledefaultbrake.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultBrake`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultBrake` (names unconfirmed).
    pub struct HkpVehicleDefaultBrake: 0x18 {
        /// `m_wheelBrakingProperties` data: 12 bytes per wheel (a float maximum torque, a float
        /// minimum pedal input to block, a byte "connected to the handbrake").
        0x08 m_wheel_braking_properties: Ptr,
        /// `m_wheelsMinTimeToBlock`.
        0x14 m_wheels_min_time_to_block: f32,
    }
}

// Translated from 00e398c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultBrake::calcBreakingInfo` (Xbox PDB), `(this, delta time, instance, input,
/// output)`. `data` = the word at `instance + 0x1C` (a signed byte at `+0x20` is the wheel
/// count, read again after every wheel; its words `+0x8C` is the wheel parameter array, 0x28
/// bytes per wheel, and `instance + 0x48` the wheel infos, 0xE0 bytes per wheel). `input + 4`
/// is the pedal float `pedal` and `input + 0xC` a flag byte. `output` holds the torque array's
/// data pointer at `+0`, the "wheel blocked" byte array's data pointer at `+0xC` and a float
/// timer at `+0x18`. With `inverse = 1 / delta time` (kept in double precision), for wheel `j`:
/// - the blocked byte is "connected to the handbrake and the flag is not zero";
/// - when `minPedal < pedal` a "some wheel blocks" flag is set;
/// - with `r`, `w` the wheel parameters' first two floats and `a` the wheel info's float at
///   `+0xC0`: `n = float(-((a * r) * (w * inverse)))`, `t = float(r * n)` and `m = float(maxTorque
///   * pedal)`; when `m < |t|` the torque is `m` for `t > 0` and `-m` otherwise (also for 0 and
///   NaN), else `t`; it is written to the torque array.
///
/// Afterwards, without the "some wheel blocks" flag the timer is set to `0.0`. With it, the timer
/// becomes `delta time + timer` when `minTimeToBlock > timer` (or either is a NaN); otherwise
/// every wheel with `minPedal < pedal` gets its blocked byte set to 1.
pub fn calc_breaking_info(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultBrake>,
    delta_time: f32,
    instance: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let pedal = x87_quiet(e.mem.f32(input.addr() + 4));
    let handbrake_flag = e.mem.u8(input.addr() + 0xc);
    let output = output.addr();
    let mut some_wheel_blocks = false;
    let data = e.mem.u32(instance.addr() + 0x1c);
    let wheel_count =
        |e: &Engine| i32::from(e.mem.u8(e.mem.u32(instance.addr() + 0x1c) + 0x20) as i8);
    let delta = f64::from(x87_quiet(delta_time));
    if i32::from(e.mem.u8(data + 0x20) as i8) > 0 {
        let inverse = 1.0 / delta;
        let mut wheel = 0i32;
        loop {
            let properties = e
                .get(this, HkpVehicleDefaultBrake::m_wheel_braking_properties)
                .addr()
                .wrapping_add((wheel as u32).wrapping_mul(0xc));
            let blocked = e.mem.u8(properties + 8) != 0 && handbrake_flag != 0;
            let blocked_bytes = e.mem.u32(output + 0xc);
            e.mem
                .set_u8(blocked_bytes.wrapping_add(wheel as u32), u8::from(blocked));
            if e.mem.f32(properties + 4) < pedal {
                some_wheel_blocks = true;
            }
            let data = e.mem.u32(instance.addr() + 0x1c);
            let parameters = e
                .mem
                .u32(data + 0x8c)
                .wrapping_add((wheel as u32).wrapping_mul(0x28));
            let infos = e.mem.u32(instance.addr() + 0x48);
            let radius = f64::from(x87_quiet(e.mem.f32(parameters)));
            let info = f64::from(x87_quiet(
                e.mem.f32(
                    infos
                        .wrapping_add((wheel as u32).wrapping_mul(0xe0))
                        .wrapping_add(0xc0),
                ),
            ));
            let weight = f64::from(e.mem.f32(parameters + 4));
            let negated = -((info * radius) * (weight * inverse)) as f32;
            let torque = (radius * f64::from(x87_quiet(negated))) as f32;
            let maximum = (f64::from(e.mem.f32(properties)) * f64::from(pedal)) as f32;
            let torque = x87_quiet(torque);
            let torque = if maximum < torque.abs() {
                if torque > 0.0 {
                    maximum
                } else {
                    -maximum
                }
            } else {
                torque
            };
            let torques = e.mem.u32(output);
            e.mem.set_f32(
                torques.wrapping_add((wheel as u32).wrapping_mul(4)),
                x87_quiet(torque),
            );
            wheel += 1;
            if wheel >= wheel_count(e) {
                break;
            }
        }
    }
    if !some_wheel_blocks {
        e.mem.set_f32(output + 0x18, 0.0);
        return;
    }
    let timer = e.mem.f32(output + 0x18);
    if !(e.get(this, HkpVehicleDefaultBrake::m_wheels_min_time_to_block) <= timer) {
        e.mem
            .set_f32(output + 0x18, (delta + f64::from(timer)) as f32);
        return;
    }
    let mut wheel = 0i32;
    if wheel_count(e) > 0 {
        loop {
            let properties = e
                .get(this, HkpVehicleDefaultBrake::m_wheel_braking_properties)
                .addr()
                .wrapping_add((wheel as u32).wrapping_mul(0xc));
            if e.mem.f32(properties + 4) < pedal {
                let blocked_bytes = e.mem.u32(output + 0xc);
                e.mem.set_u8(blocked_bytes.wrapping_add(wheel as u32), 1);
            }
            wheel += 1;
            if wheel >= wheel_count(e) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e3_98c0,
        calc_breaking_info(Ptr<HkpVehicleDefaultBrake>, f32, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;

    struct Rig {
        e: Engine,
        this: u32,
        instance: u32,
        input: u32,
        output: u32,
        properties: u32,
        torques: u32,
        blocked: u32,
    }

    /// Two wheels, delta time 0.5 (so 1 / dt = 2) and pedal 0.5, handbrake flag set.
    /// Wheel 0: radius 2, info 3, weight 5, maximum torque 100, minimum pedal 0.25, connected.
    /// Wheel 1: radius 1, info -1, weight 1, maximum torque 1, minimum pedal 0.4, not connected.
    fn rig(min_time: f32, timer: f32) -> Rig {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let properties = e.mem.alloc(0x18);
        e.mem.set_f32(properties, 100.0);
        e.mem.set_f32(properties + 4, 0.25);
        e.mem.set_u8(properties + 8, 1);
        e.mem.set_f32(properties + 0xc, 1.0);
        e.mem.set_f32(properties + 0xc + 4, 0.4);
        e.mem.set_u32(this + 8, properties);
        e.mem.set_f32(this + 0x14, min_time);
        let data = e.mem.alloc(0xa0);
        e.mem.set_u8(data + 0x20, 2);
        let parameters = e.mem.alloc(0x50);
        e.mem.set_f32(parameters, 2.0);
        e.mem.set_f32(parameters + 4, 5.0);
        e.mem.set_f32(parameters + 0x28, 1.0);
        e.mem.set_f32(parameters + 0x28 + 4, 1.0);
        e.mem.set_u32(data + 0x8c, parameters);
        let infos = e.mem.alloc(0x1c0);
        e.mem.set_f32(infos + 0xc0, 3.0);
        e.mem.set_f32(infos + 0xe0 + 0xc0, -1.0);
        let instance = e.mem.alloc(0x60);
        e.mem.set_u32(instance + 0x1c, data);
        e.mem.set_u32(instance + 0x48, infos);
        let input = e.mem.alloc(0x10);
        e.mem.set_f32(input + 4, 0.5);
        e.mem.set_u8(input + 0xc, 1);
        let torques = e.mem.alloc(8);
        let blocked = e.mem.alloc(8);
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output, torques);
        e.mem.set_u32(output + 0xc, blocked);
        e.mem.set_f32(output + 0x18, timer);
        Rig {
            e,
            this,
            instance,
            input,
            output,
            properties,
            torques,
            blocked,
        }
    }

    fn run(r: &mut Rig) {
        r.e.call(
            0x00e3_98c0,
            &args![r.this, 0.5f32, r.instance, r.input, r.output],
        );
    }

    #[test]
    fn torques_are_limited_by_the_pedal_and_the_maximum() {
        let mut r = rig(1.0, 0.25);
        run(&mut r);
        // Wheel 0: n = -((3 * 2) * (5 * 2)) = -60, t = 2 * -60 = -120, m = 100 * 0.5 = 50 < 120
        // and t is not above zero, so -50. Wheel 1: n = -((-1 * 1) * (1 * 2)) = 2, t = 2,
        // m = 1 * 0.5 = 0.5 < 2 and t > 0, so +0.5.
        assert_eq!(r.e.mem.f32(r.torques), -50.0);
        assert_eq!(r.e.mem.f32(r.torques + 4), 0.5);
        // Wheel 0 is connected and the flag is set; wheel 1 is not connected.
        assert_eq!([r.e.mem.u8(r.blocked), r.e.mem.u8(r.blocked + 1)], [1, 0]);
        // A torque below the maximum is kept: wheel 1 with maximum 100.
        r.e.mem.set_f32(r.properties + 0xc, 100.0);
        run(&mut r);
        assert_eq!(r.e.mem.f32(r.torques + 4), 2.0);
    }

    #[test]
    fn the_timer_runs_until_the_minimum_time_to_block() {
        // minTimeToBlock 1.0 > timer 0.25: the timer gains the delta time, no byte changes.
        let mut r = rig(1.0, 0.25);
        run(&mut r);
        assert_eq!(r.e.mem.f32(r.output + 0x18), 0.75);
        assert_eq!([r.e.mem.u8(r.blocked), r.e.mem.u8(r.blocked + 1)], [1, 0]);
        // A NaN limit also counts on.
        r.e.mem.set_f32(r.this + 0x14, f32::NAN);
        run(&mut r);
        assert_eq!(r.e.mem.f32(r.output + 0x18), 1.25);
    }

    #[test]
    fn once_the_time_has_passed_every_pedal_blocked_wheel_is_fixed() {
        // minTimeToBlock 0.5 <= timer 0.75: wheel 1 (minimum pedal 0.4 < 0.5) is fixed too.
        let mut r = rig(0.5, 0.75);
        run(&mut r);
        assert_eq!([r.e.mem.u8(r.blocked), r.e.mem.u8(r.blocked + 1)], [1, 1]);
        assert_eq!(r.e.mem.f32(r.output + 0x18), 0.75);
    }

    #[test]
    fn without_a_blocking_wheel_the_timer_resets() {
        let mut r = rig(0.5, 0.75);
        r.e.mem.set_f32(r.input + 4, 0.1);
        run(&mut r);
        assert_eq!(r.e.mem.f32(r.output + 0x18), 0.0);
        // No wheels at all: the same.
        let data = r.e.mem.u32(r.instance + 0x1c);
        r.e.mem.set_u8(data + 0x20, 0);
        r.e.mem.set_f32(r.output + 0x18, 3.0);
        run(&mut r);
        assert_eq!(r.e.mem.f32(r.output + 0x18), 0.0);
    }
}
