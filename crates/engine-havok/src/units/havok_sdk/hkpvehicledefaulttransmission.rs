//! `Havok SDK/hkpvehicledefaulttransmission.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layouts and their names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultTransmission`, unconfirmed). The vtable
//! (`010e5ff4`) has `calcTransmission` in slot `0xC`, `calcMainTransmittedTorque` in `0x10`,
//! `calcTransmissionRPM` in `0x14`, `calcIsReversing` in `0x18`, `updateCurrentGear` in `0x1C`
//! and `getCurrentRatio` in `0x20`.
//!
//! The transmission output record the functions fill: float `+0` (the rpm), float `+4` (the main
//! transmitted torque), data pointer `+8` of the per-wheel torque floats, byte `+0xD` (reversing),
//! signed byte `+0xE` (the gear), byte `+0xF` (shifting) and float `+0x10` (the clutch delay
//! countdown).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultTransmission` (names unconfirmed).
    pub struct HkpVehicleDefaultTransmission: 0x34 {
        /// `m_downshiftRPM`.
        0x08 m_downshift_rpm: f32,
        /// `m_upshiftRPM`.
        0x0c m_upshift_rpm: f32,
        /// `m_primaryTransmissionRatio`.
        0x10 m_primary_transmission_ratio: f32,
        /// `m_clutchDelayTime`.
        0x14 m_clutch_delay_time: f32,
        /// `m_reverseGearRatio`.
        0x18 m_reverse_gear_ratio: f32,
        /// `m_gearsRatio` data.
        0x1c m_gears_ratio: Ptr,
        /// `m_gearsRatio` size.
        0x20 m_gears_ratio_size: i32,
        /// `m_wheelsTorqueRatio` data.
        0x28 m_wheels_torque_ratio: Ptr,
        /// `m_wheelsTorqueRatio` size.
        0x2c m_wheels_torque_ratio_size: i32,
    }
}

/// Slots of the transmission's vtable.
const SLOT_MAIN_TRANSMITTED_TORQUE: u32 = 0x10;
const SLOT_TRANSMISSION_RPM: u32 = 0x14;
const SLOT_IS_REVERSING: u32 = 0x18;
const SLOT_UPDATE_CURRENT_GEAR: u32 = 0x1c;
const SLOT_CURRENT_RATIO: u32 = 0x20;

// Translated from 00e43c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::calcIsReversing` (Xbox PDB), `(this, result, instance,
/// output)` (`this` is not read): writes the byte at `result` and returns `result`. The byte
/// is 1 when the byte at `instance + 0xB0` is not zero and the signed gear byte of `output`
/// (`+0xE`) is not above zero, else 0.
pub fn calc_is_reversing(
    e: &mut Engine,
    _unused_this: Ptr,
    result: Ptr,
    instance: Ptr,
    output: Ptr,
) -> Ptr {
    let reversing =
        e.mem.u8(instance.addr() + 0xb0) != 0 && (e.mem.u8(output.addr() + 0xe) as i8) <= 0;
    e.mem.set_u8(result.addr(), u8::from(reversing));
    result
}

// Translated from 00e43c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::getCurrentRatio` (Xbox PDB), `(this, _, output)`; the first
/// stack word (the instance) is not read. The primary ratio times either the negated reverse
/// ratio (when the reversing byte `output + 0xD` is not zero) or the gear ratio indexed by the
/// signed gear byte `output + 0xE`, rounded to `float`.
pub fn get_current_ratio(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultTransmission>,
    _unused_1: u32,
    output: Ptr,
) -> f32 {
    let ratio = if e.mem.u8(output.addr() + 0xd) != 0 {
        -x87_quiet(e.get(this, HkpVehicleDefaultTransmission::m_reverse_gear_ratio))
    } else {
        let gear = i32::from(e.mem.u8(output.addr() + 0xe) as i8);
        let ratios = e
            .get(this, HkpVehicleDefaultTransmission::m_gears_ratio)
            .addr();
        x87_quiet(
            e.mem
                .f32(ratios.wrapping_add((gear as u32).wrapping_mul(4))),
        )
    };
    let primary = f64::from(x87_quiet(e.get(
        this,
        HkpVehicleDefaultTransmission::m_primary_transmission_ratio,
    )));
    x87_quiet((primary * f64::from(ratio)) as f32)
}

// Translated from 00e43c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::calcTransmission` (Xbox PDB), `(this, delta time, instance,
/// output)`:
/// 1. `output + 0xD` = the byte `calcIsReversing` (slot `0x18`) writes through the pointer it
///    is given (a scratch slot) and returns, called with `(scratch, instance, output)`;
/// 2. `output[1]` = slot `0x10` (`calcMainTransmittedTorque`) and `output[0]` = slot `0x14`
///    (`calcTransmissionRPM`), both with `(instance, output)` and the answer in `ST0`;
/// 3. for every entry `j` of `m_wheelsTorqueRatio` (size read again after every entry):
///    `output.torques[j] = float(ratio[j] * output[1])`, `output.torques` being the data pointer
///    at `output + 8`;
/// 4. slot `0x1C` (`updateCurrentGear`) is called with `(delta time, instance, output)`, the
///    float passed as its bits through the x87.
pub fn calc_transmission(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultTransmission>,
    delta_time: f32,
    instance: Ptr,
    output: Ptr,
) {
    let out = output.addr();
    let flag = e.with_stack(4, |e, scratch| {
        let at = e
            .vcall(
                this.addr(),
                SLOT_IS_REVERSING,
                &args![scratch, instance, output],
            )
            .u32();
        e.mem.u8(at)
    });
    e.mem.set_u8(out + 0xd, flag);
    let torque = e
        .vcall(
            this.addr(),
            SLOT_MAIN_TRANSMITTED_TORQUE,
            &args![instance, output],
        )
        .f32();
    e.mem.set_f32(out + 4, torque);
    let rpm = e
        .vcall(this.addr(), SLOT_TRANSMISSION_RPM, &args![instance, output])
        .f32();
    e.mem.set_f32(out, rpm);
    let mut wheel = 0u32;
    if e.get(
        this,
        HkpVehicleDefaultTransmission::m_wheels_torque_ratio_size,
    ) > 0
    {
        loop {
            let ratios = e
                .get(this, HkpVehicleDefaultTransmission::m_wheels_torque_ratio)
                .addr();
            let ratio = f64::from(x87_quiet(e.mem.f32(ratios.wrapping_add(wheel * 4))));
            let torques = e.mem.u32(out + 8);
            let main = f64::from(e.mem.f32(out + 4));
            e.mem
                .set_f32(torques.wrapping_add(wheel * 4), (ratio * main) as f32);
            wheel += 1;
            if wheel as i32
                >= e.get(
                    this,
                    HkpVehicleDefaultTransmission::m_wheels_torque_ratio_size,
                )
            {
                break;
            }
        }
    }
    let bits = x87_quiet_bits(delta_time.to_bits());
    e.vcall(
        this.addr(),
        SLOT_UPDATE_CURRENT_GEAR,
        &args![bits, instance, output],
    );
}

// Translated from 00e43d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::calcMainTransmittedTorque` (Xbox PDB), `(this, instance,
/// output)`: `0.0` while the shifting byte `output + 0xF` is not zero, else slot `0x20`
/// (`getCurrentRatio`, with `(instance, output)`) times the float at `instance + 0xB4` (read
/// before the call), rounded to `float`.
pub fn calc_main_transmitted_torque(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultTransmission>,
    instance: Ptr,
    output: Ptr,
) -> f32 {
    if e.mem.u8(output.addr() + 0xf) != 0 {
        return 0.0;
    }
    let factor = x87_quiet(e.mem.f32(instance.addr() + 0xb4));
    let ratio = e
        .vcall(this.addr(), SLOT_CURRENT_RATIO, &args![instance, output])
        .f32();
    x87_quiet((f64::from(ratio) * f64::from(factor)) as f32)
}

// Translated from 00e43d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::calcTransmissionRPM` (Xbox PDB), `(this, instance,
/// output)`. `data` = the word at `instance + 0x1C` (a signed byte at `+0x20` is the wheel
/// count `n`) and `infos` = the word at `instance + 0x48` (0xE0 bytes per wheel). The exe
/// unrolls the loop by four; the result is the same as one loop over the wheels `j` in `0..n`
/// (nothing for `n <= 0`) with `a = float((infos[j].float(+0xC0) * 60.0) * c)`, `c` being the
/// double `0x3fc45f3060000000` (1 / 2 pi, only 24 bits long) and
/// `acc = float(a * m_wheelsTorqueRatio[j] + acc)` (double precision between the stores; the
/// ratio array's size is not checked). The answer is `float(getCurrentRatio * acc)` (slot
/// `0x20` with `(instance, output)`), or `0.0` when that is below zero.
pub fn calc_transmission_rpm(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultTransmission>,
    instance: Ptr,
    output: Ptr,
) -> f32 {
    let data = e.mem.u32(instance.addr() + 0x1c);
    let wheels = i32::from(e.mem.u8(data + 0x20) as i8);
    let infos = e.mem.u32(instance.addr() + 0x48);
    let ratios = e
        .get(this, HkpVehicleDefaultTransmission::m_wheels_torque_ratio)
        .addr();
    let scale = f64::from_bits(0x3fc4_5f30_6000_0000);
    let mut acc = 0.0f32;
    for wheel in 0..wheels.max(0) as u32 {
        let value = f64::from(x87_quiet(
            e.mem.f32(
                infos
                    .wrapping_add(wheel.wrapping_mul(0xe0))
                    .wrapping_add(0xc0),
            ),
        ));
        let rotation = x87_quiet(((value * 60.0) * scale) as f32);
        let ratio = f64::from(e.mem.f32(ratios.wrapping_add(wheel.wrapping_mul(4))));
        acc = (f64::from(rotation) * ratio + f64::from(acc)) as f32;
    }
    let ratio = e
        .vcall(this.addr(), SLOT_CURRENT_RATIO, &args![instance, output])
        .f32();
    let rpm = x87_quiet((f64::from(ratio) * f64::from(x87_quiet(acc))) as f32);
    if rpm < 0.0 {
        0.0
    } else {
        rpm
    }
}

// Translated from 00e43ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultTransmission::updateCurrentGear` (Xbox PDB), `(this, delta time, _,
/// output)`; the second stack word is not read. The countdown (`output + 0x10`) loses the delta
/// time (stored as `float`); while the shifting byte (`+0xF`) is set it is cleared once the
/// countdown is at most zero (`FLDZ`, `FCOMPP`, `TEST AH,0x1`: C0 is set for `0 < countdown`
/// and for unordered, so a NaN countdown keeps the byte). Unless the reversing byte (`+0xD`) is set:
/// when the rpm (`+0`) is below `downshiftRPM` and the signed gear byte (`+0xE`) is above
/// zero, the gear goes down by one; and when `upshiftRPM` is below the rpm and the (new) gear
/// plus one is below `m_gearsRatio`'s size, the gear goes up by one. Each gear change sets the
/// countdown to the clutch delay time and the shifting byte to 1.
pub fn update_current_gear(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultTransmission>,
    delta_time: f32,
    _unused_2: u32,
    output: Ptr,
) {
    let out = output.addr();
    let shifting = e.mem.u8(out + 0xf) != 0;
    let countdown = x87_quiet(e.mem.f32(out + 0x10));
    // `FLD` the countdown, `FSUB` the delta time (a memory operand): the x87 choice of NaN.
    let difference = x87_sub(f64::from(countdown), x87_m32(delta_time.to_bits()));
    let remaining = f32::from_bits(x87_store32(difference));
    e.mem.set_f32(out + 0x10, remaining);
    if shifting && remaining <= 0.0 {
        e.mem.set_u8(out + 0xf, 0);
    }
    if e.mem.u8(out + 0xd) != 0 {
        return;
    }
    let rpm = x87_quiet(e.mem.f32(out));
    if e.get(this, HkpVehicleDefaultTransmission::m_downshift_rpm) > rpm {
        let gear = e.mem.u8(out + 0xe);
        if (gear as i8) > 0 {
            e.mem.set_u8(out + 0xe, gear.wrapping_sub(1));
            let delay = x87_quiet(e.get(this, HkpVehicleDefaultTransmission::m_clutch_delay_time));
            e.mem.set_f32(out + 0x10, delay);
            e.mem.set_u8(out + 0xf, 1);
        }
    }
    let rpm = x87_quiet(e.mem.f32(out));
    if e.get(this, HkpVehicleDefaultTransmission::m_upshift_rpm) < rpm {
        let gear = e.mem.u8(out + 0xe);
        let next = i32::from(gear as i8) + 1;
        if next < e.get(this, HkpVehicleDefaultTransmission::m_gears_ratio_size) {
            e.mem.set_u8(out + 0xe, gear.wrapping_add(1));
            let delay = x87_quiet(e.get(this, HkpVehicleDefaultTransmission::m_clutch_delay_time));
            e.mem.set_f32(out + 0x10, delay);
            e.mem.set_u8(out + 0xf, 1);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_3c00, calc_is_reversing(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00e4_3c30,
            get_current_ratio(Ptr<HkpVehicleDefaultTransmission>, u32, Ptr) -> f32
        ),
        entry!(
            0x00e4_3c80,
            calc_transmission(Ptr<HkpVehicleDefaultTransmission>, f32, Ptr, Ptr)
        ),
        entry!(
            0x00e4_3d00,
            calc_main_transmitted_torque(Ptr<HkpVehicleDefaultTransmission>, Ptr, Ptr) -> f32
        ),
        entry!(
            0x00e4_3d40,
            calc_transmission_rpm(Ptr<HkpVehicleDefaultTransmission>, Ptr, Ptr) -> f32
        ),
        entry!(
            0x00e4_3ea0,
            update_current_gear(Ptr<HkpVehicleDefaultTransmission>, f32, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, new_log, Log};

    /// A transmission whose vtable slots `0x10` to `0x20` are doubles at `0x0900_0000 + slot`
    /// (they log and answer `ST0`/`EAX` 0 unless a test replaces them). Primary ratio 2,
    /// reverse ratio 3, gear ratios `[0.5, 1.5, 2.5]` (size 3), clutch delay 0.5, downshift
    /// 1000, upshift 3000, wheel torque ratios `[0.5, 0.25]`.
    fn transmission(e: &mut Engine, log: &Log) -> u32 {
        let this = e.mem.alloc(0x34);
        let vtable = e.mem.alloc(0x30);
        for slot in (0x0c..0x30).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
            let log = std::rc::Rc::clone(log);
            e.register_double(0x0900_0000 + slot, move |_, a| {
                log.borrow_mut().push((0x0900_0000 + slot, a.to_vec()));
                Ret::default()
            });
        }
        e.mem.set_u32(this, vtable);
        e.mem.set_f32(this + 8, 1000.0);
        e.mem.set_f32(this + 0xc, 3000.0);
        e.mem.set_f32(this + 0x10, 2.0);
        e.mem.set_f32(this + 0x14, 0.5);
        e.mem.set_f32(this + 0x18, 3.0);
        let gears = e.mem.alloc(0x10);
        for (i, ratio) in [0.5f32, 1.5, 2.5].iter().enumerate() {
            e.mem.set_f32(gears + 4 * i as u32, *ratio);
        }
        e.mem.set_u32(this + 0x1c, gears);
        e.mem.set_u32(this + 0x20, 3);
        let wheels = e.mem.alloc(8);
        e.mem.set_f32(wheels, 0.5);
        e.mem.set_f32(wheels + 4, 0.25);
        e.mem.set_u32(this + 0x28, wheels);
        e.mem.set_u32(this + 0x2c, 2);
        this
    }

    /// Replaces the double at the vtable `slot` of `this` by one that answers `st0`.
    fn answer(e: &mut Engine, log: &Log, slot: u32, st0: f64) {
        let log = std::rc::Rc::clone(log);
        e.register_double(0x0900_0000 + slot, move |_, a| {
            log.borrow_mut().push((0x0900_0000 + slot, a.to_vec()));
            Ret {
                st0,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn reversing_needs_the_flag_and_a_gear_not_above_zero() {
        let mut e = engine();
        let instance = e.mem.alloc(0xc0);
        let output = e.mem.alloc(0x20);
        let result = e.mem.alloc(4);
        // The instance's byte at +0xB0 is zero: never reversing.
        assert_eq!(
            e.call(0x00e4_3c00, &args![0u32, result, instance, output])
                .u32(),
            result
        );
        assert_eq!(e.mem.u8(result), 0);
        e.mem.set_u8(instance + 0xb0, 1);
        for (gear, expected) in [(0u8, 1u8), (1, 0), (0xff, 1), (0x7f, 0)] {
            e.mem.set_u8(output + 0xe, gear);
            e.call(0x00e4_3c00, &args![0u32, result, instance, output]);
            assert_eq!(e.mem.u8(result), expected, "gear {gear}");
        }
    }

    #[test]
    fn the_current_ratio_follows_the_gear_or_the_reverse() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        let output = e.mem.alloc(0x20);
        e.mem.set_u8(output + 0xe, 1);
        // 2 * 1.5
        assert_eq!(e.call(0x00e4_3c30, &args![this, 0u32, output]).f32(), 3.0);
        e.mem.set_u8(output + 0xd, 1);
        // 2 * -3
        assert_eq!(e.call(0x00e4_3c30, &args![this, 0u32, output]).f32(), -6.0);
    }

    #[test]
    fn calc_transmission_fills_the_output_and_updates_the_gear() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        answer(&mut e, &log, 0x10, 4.0);
        answer(&mut e, &log, 0x14, 7.5);
        let sink = std::rc::Rc::clone(&log);
        e.register_double(0x0900_0018, move |e, a| {
            sink.borrow_mut().push((0x0900_0018, a.to_vec()));
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let torques = e.mem.alloc(8);
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output + 8, torques);
        let instance = 0x1234u32;
        e.call(0x00e4_3c80, &args![this, 0.25f32, instance, output]);
        assert_eq!(e.mem.u8(output + 0xd), 1);
        assert_eq!(e.mem.f32(output + 4), 4.0);
        assert_eq!(e.mem.f32(output), 7.5);
        assert_eq!([e.mem.f32(torques), e.mem.f32(torques + 4)], [2.0, 1.0]);
        let reversing = calls(&log, 0x0900_0018);
        assert_eq!(reversing.len(), 1);
        assert_eq!(reversing[0][0], this);
        assert_eq!(reversing[0][2..], [instance, output]);
        assert_eq!(calls(&log, 0x0900_0010), vec![vec![this, instance, output]]);
        assert_eq!(calls(&log, 0x0900_0014), vec![vec![this, instance, output]]);
        assert_eq!(
            calls(&log, 0x0900_001c),
            vec![vec![this, 0.25f32.to_bits(), instance, output]]
        );
    }

    #[test]
    fn the_main_torque_is_the_ratio_times_the_instance_factor() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        answer(&mut e, &log, 0x20, 3.0);
        let instance = e.mem.alloc(0xc0);
        e.mem.set_f32(instance + 0xb4, 2.0);
        let output = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00e4_3d00, &args![this, instance, output]).f32(),
            6.0
        );
        assert_eq!(calls(&log, 0x0900_0020), vec![vec![this, instance, output]]);
        // While shifting the torque is zero and the ratio is not asked.
        e.mem.set_u8(output + 0xf, 1);
        assert_eq!(
            e.call(0x00e4_3d00, &args![this, instance, output]).f32(),
            0.0
        );
        assert_eq!(calls(&log, 0x0900_0020).len(), 1);
    }

    #[test]
    fn the_rpm_sums_the_wheels_and_never_goes_below_zero() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        answer(&mut e, &log, 0x20, 3.0);
        // Five wheels (one unrolled round of the exe plus one more) with the c0 values 1 to 5;
        // the ratio array has the entries [0.5, 0.25, 1, 1, 1] here.
        let ratios = e.mem.alloc(0x20);
        for (i, ratio) in [0.5f32, 0.25, 1.0, 1.0, 1.0].iter().enumerate() {
            e.mem.set_f32(ratios + 4 * i as u32, *ratio);
        }
        e.mem.set_u32(this + 0x28, ratios);
        let infos = e.mem.alloc(0x500);
        for wheel in 0..5u32 {
            e.mem
                .set_f32(infos + 0xe0 * wheel + 0xc0, (wheel + 1) as f32);
        }
        let data = e.mem.alloc(0x40);
        e.mem.set_u8(data + 0x20, 5);
        let instance = e.mem.alloc(0x60);
        e.mem.set_u32(instance + 0x1c, data);
        e.mem.set_u32(instance + 0x48, infos);
        let output = e.mem.alloc(0x20);
        // (1 * 0.5 + 2 * 0.25 + 3 + 4 + 5) * 60 / (2 pi) * 3 = 13 * 9.549297 * 3.
        let expected = 13.0 * 60.0 / (2.0 * std::f32::consts::PI) * 3.0;
        let got = e.call(0x00e4_3d40, &args![this, instance, output]).f32();
        assert!(
            (got - expected).abs() < expected * 1e-5,
            "{got} vs {expected}"
        );
        // A negative ratio gives zero; so do no wheels.
        answer(&mut e, &log, 0x20, -1.0);
        assert_eq!(
            e.call(0x00e4_3d40, &args![this, instance, output]).f32(),
            0.0
        );
        e.mem.set_u8(data + 0x20, 0);
        answer(&mut e, &log, 0x20, 3.0);
        assert_eq!(
            e.call(0x00e4_3d40, &args![this, instance, output]).f32(),
            0.0
        );
    }

    /// An output record with the rpm, gear, reversing and shifting bytes and the countdown.
    fn record(
        e: &mut Engine,
        rpm: f32,
        gear: u8,
        reversing: u8,
        shifting: u8,
        countdown: f32,
    ) -> u32 {
        let output = e.mem.alloc(0x20);
        e.mem.set_f32(output, rpm);
        e.mem.set_u8(output + 0xe, gear);
        e.mem.set_u8(output + 0xd, reversing);
        e.mem.set_u8(output + 0xf, shifting);
        e.mem.set_f32(output + 0x10, countdown);
        output
    }

    fn gear_state(e: &Engine, output: u32) -> (u8, u8, f32) {
        (
            e.mem.u8(output + 0xe),
            e.mem.u8(output + 0xf),
            e.mem.f32(output + 0x10),
        )
    }

    #[test]
    fn a_low_rpm_shifts_down_and_a_high_one_shifts_up() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        let output = record(&mut e, 500.0, 2, 0, 0, 0.0);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        // Down to gear 1, countdown 0.5 (the clutch delay), shifting.
        assert_eq!(gear_state(&e, output), (1, 1, 0.5));
        // Gear 0 cannot go lower.
        let output = record(&mut e, 500.0, 0, 0, 0, 0.0);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output), (0, 0, -0.1));
        // Up: 2 + 1 < 3 is false, so gear 1 goes to 2 but gear 2 stays.
        let output = record(&mut e, 4000.0, 1, 0, 0, 0.0);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output), (2, 1, 0.5));
        let output = record(&mut e, 4000.0, 2, 0, 0, 0.0);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output), (2, 0, -0.1));
    }

    #[test]
    fn shifting_ends_with_the_countdown_and_reversing_changes_no_gear() {
        let mut e = engine();
        let log = new_log();
        let this = transmission(&mut e, &log);
        // 0.05 - 0.1 <= 0: the shifting byte is cleared (rpm between the limits).
        let output = record(&mut e, 2000.0, 1, 0, 1, 0.05);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output).1, 0);
        // Still counting: stays set.
        let output = record(&mut e, 2000.0, 1, 0, 1, 0.5);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output), (1, 1, 0.4));
        // Reversing: only the countdown moves, whatever the rpm.
        let output = record(&mut e, 500.0, 2, 1, 0, 0.0);
        e.call(0x00e4_3ea0, &args![this, 0.1f32, 0u32, output]);
        assert_eq!(gear_state(&e, output), (2, 0, -0.1));
    }
}
