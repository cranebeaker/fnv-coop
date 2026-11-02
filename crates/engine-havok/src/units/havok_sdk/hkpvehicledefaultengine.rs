//! `Havok SDK/hkpvehicledefaultengine.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layout and its names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultEngine`, unconfirmed).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpVehicleDefaultEngine` (names unconfirmed).
    pub struct HkpVehicleDefaultEngine: 0x30 {
        /// `m_minRPM`.
        0x08 m_min_rpm: f32,
        /// `m_optRPM`.
        0x0c m_opt_rpm: f32,
        /// `m_maxRPM`.
        0x10 m_max_rpm: f32,
        /// `m_maxTorque`.
        0x14 m_max_torque: f32,
        /// `m_torqueFactorAtMinRPM`.
        0x18 m_torque_factor_at_min_rpm: f32,
        /// `m_torqueFactorAtMaxRPM`.
        0x1c m_torque_factor_at_max_rpm: f32,
        /// `m_resistanceFactorAtMinRPM`.
        0x20 m_resistance_factor_at_min_rpm: f32,
        /// `m_resistanceFactorAtOptRPM`.
        0x24 m_resistance_factor_at_opt_rpm: f32,
        /// `m_resistanceFactorAtMaxRPM`.
        0x28 m_resistance_factor_at_max_rpm: f32,
        /// `m_clutchSlipRPM`.
        0x2c m_clutch_slip_rpm: f32,
    }
}

/// The double `0.5` at `01011588`.
const HALF: f64 = 0.5;

// Translated from 00e440a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultEngine::calcEngineInfo` (Xbox PDB), `(this, _, _, throttle, rpm, output)`;
/// the first two stack words are not read. `throttle` and `rpm` point to floats, `output`
/// receives the torque at `+0` and the corrected rpm at `+4`. Every `float` store rounds
/// (the intermediate values are kept in double precision, which is what the x87 does here);
/// `T` = the throttle, `r` = the rpm, `min`, `opt`, `max`, `maxTorque`, `fMin`, `fMax`,
/// `rMin`, `rOpt`, `rMax` and `slip` the fields of `this`:
/// 1. `output[1] = r`. When `min > r`: `g = float(slip * T)`, `h = float(min * 0.5)`, and
///    `output[1] = float(g + min)` if `h > r`, else `float(min + (g * (r - h)) / (min - h))`;
/// 2. `D = float(output[1] - opt)`. For `D` not below zero (also for a NaN): when `max <=
///    output[1]` (or a NaN) the rpm is limited to `max` and the torque is `float(T * 0.0 -
///    float(rMax * maxTorque))`; otherwise, with `k = float(1 / (max - opt))`, `u = float((rMax -
///    rOpt) * k)`, `V = float((u * D + rOpt) * maxTorque)`, `W = float(k * ((fMax - 1) * k))` and
///    `Z = float(((D * D) * W + 1) * maxTorque)` the torque is `float(T * Z - V)`;
/// 3. for `D` below zero: `k = float(1 / (min - opt))`, `u = float((rMin - rOpt) * k)`, `P4 =
///    float((u * D + rOpt) * maxTorque)`, `W = float(k * ((fMin - 1) * k))`, `P1 = float(((D * D)
///    * W + 1) * maxTorque)`, the rpm is read again and with `r2` it, the torque is `float(T *
///    P1 - float((r2 / min) * P4))` when `min > r2` and `float(T * P1 - P4)` otherwise (also for
///    a NaN).
pub fn calc_engine_info(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultEngine>,
    _unused_1: u32,
    _unused_2: u32,
    throttle: Ptr,
    rpm: Ptr,
    output: Ptr,
) {
    // The x87 stack code with the x87 choice of NaN: `ld` is an `FLD` of a field (quiet), `mem`
    // a field as the memory operand of `FADD`/`FSUB`/`FMUL`/`FDIV` (a signalling NaN stays
    // one there) and `rounded` a store to a scratch float and the load back.
    let (fadd, fsub, fmul, fdiv) = (x87_add, x87_sub, x87_mul, x87_div);
    let me = this.addr();
    let ld = |e: &Engine, offset: u32| x87_load32(e.mem.u32(me + offset));
    let mem = |e: &Engine, offset: u32| x87_m32(e.mem.u32(me + offset));
    let rounded = |value: f64| x87_load32(x87_store32(value));
    let (min_rpm, opt_rpm, max_rpm, max_torque) = (0x08, 0x0c, 0x10, 0x14);
    let (torque_min, torque_max, resistance_min, resistance_opt, resistance_max, slip) =
        (0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c);
    let out = output.addr();
    let r = x87_quiet(e.mem.f32(rpm.addr()));
    e.mem.set_f32(out + 4, r);
    let t = x87_load32(e.mem.u32(throttle.addr()));
    let min = e.get(this, HkpVehicleDefaultEngine::m_min_rpm);
    let max = e.get(this, HkpVehicleDefaultEngine::m_max_rpm);
    if min > r {
        let g = rounded(fmul(ld(e, slip), t));
        let h = rounded(fmul(ld(e, min_rpm), HALF));
        if h > f64::from(r) {
            let sum = fadd(g, mem(e, min_rpm));
            e.mem.set_u32(out + 4, x87_store32(sum));
        } else {
            let lift = fmul(fsub(f64::from(r), h), g);
            let corrected = fdiv(lift, fsub(mem(e, min_rpm), h));
            let sum = fadd(corrected, ld(e, min_rpm));
            e.mem.set_u32(out + 4, x87_store32(sum));
        }
    }
    let rpm_out = x87_load32(e.mem.u32(out + 4));
    let d = rounded(fsub(rpm_out, mem(e, opt_rpm)));
    if !(d < 0.0) {
        let rpm_out = x87_quiet(e.mem.f32(out + 4));
        if !(max > rpm_out) {
            e.mem.set_f32(out + 4, x87_quiet(max));
            let drag = rounded(fmul(ld(e, resistance_max), mem(e, max_torque)));
            let torque = fsub(fmul(t, 0.0), drag);
            e.mem.set_u32(out, x87_store32(torque));
            return;
        }
        let k = rounded(fdiv(1.0, fsub(ld(e, max_rpm), mem(e, opt_rpm))));
        let u = rounded(fmul(fsub(ld(e, resistance_max), mem(e, resistance_opt)), k));
        let v = fmul(fadd(fmul(u, d), mem(e, resistance_opt)), mem(e, max_torque));
        let v = rounded(v);
        let w = rounded(fmul(k, fmul(fsub(ld(e, torque_max), 1.0), k)));
        let z = fmul(fadd(fmul(fmul(d, d), w), 1.0), mem(e, max_torque));
        let z = rounded(z);
        e.mem.set_u32(out, x87_store32(fsub(fmul(t, z), v)));
        return;
    }
    let k = rounded(fdiv(1.0, fsub(ld(e, min_rpm), mem(e, opt_rpm))));
    let u = rounded(fmul(fsub(ld(e, resistance_min), mem(e, resistance_opt)), k));
    let p4 = fmul(fadd(fmul(u, d), mem(e, resistance_opt)), mem(e, max_torque));
    let p4 = rounded(p4);
    let w = rounded(fmul(k, fmul(fsub(ld(e, torque_min), 1.0), k)));
    let p1 = fmul(fadd(fmul(fmul(d, d), w), 1.0), mem(e, max_torque));
    let p1 = rounded(p1);
    let r2 = x87_quiet(e.mem.f32(rpm.addr()));
    let torque = if min > r2 {
        let y = rounded(fmul(fdiv(f64::from(r2), mem(e, min_rpm)), p4));
        fsub(fmul(t, p1), y)
    } else {
        fsub(fmul(t, p1), p4)
    };
    e.mem.set_u32(out, x87_store32(torque));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e4_40a0,
        calc_engine_info(Ptr<HkpVehicleDefaultEngine>, u32, u32, Ptr, Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;

    /// An engine with min 8, opt 16, max 32, max torque 2, torque factors 3 (min) and 0.5 (max),
    /// resistance factors 2.5 (min), 0.5 (opt) and 1.5 (max) and clutch slip 0.5. Runs
    /// `calcEngineInfo` with the throttle and rpm and returns `(torque, output rpm)`.
    fn run(throttle: f32, rpm: f32) -> (f32, f32) {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        for (offset, value) in [
            (0x08, 8.0f32),
            (0x0c, 16.0),
            (0x10, 32.0),
            (0x14, 2.0),
            (0x18, 3.0),
            (0x1c, 0.5),
            (0x20, 2.5),
            (0x24, 0.5),
            (0x28, 1.5),
            (0x2c, 0.5),
        ] {
            e.mem.set_f32(this + offset, value);
        }
        let throttle_at = e.mem.alloc(4);
        e.mem.set_f32(throttle_at, throttle);
        let rpm_at = e.mem.alloc(4);
        e.mem.set_f32(rpm_at, rpm);
        let output = e.mem.alloc(8);
        e.call(
            0x00e4_40a0,
            &args![this, 1u32, 2u32, throttle_at, rpm_at, output],
        );
        (e.mem.f32(output), e.mem.f32(output + 4))
    }

    #[test]
    fn above_the_maximum_the_rpm_is_limited_and_the_engine_drags() {
        // The torque is T * 0 - 1.5 * 2 and the rpm becomes the maximum.
        assert_eq!(run(1.0, 40.0), (-3.0, 32.0));
        // Exactly the maximum counts as above.
        assert_eq!(run(1.0, 32.0), (-3.0, 32.0));
    }

    #[test]
    fn between_the_optimum_and_the_maximum_the_curve_interpolates() {
        // D = 8, k = 1/16, V = ((1/16 * 8) + 0.5) * 2 = 2, W = -1/512, Z = 1.75.
        assert_eq!(run(2.0, 24.0), (3.5 - 2.0, 24.0));
    }

    #[test]
    fn below_the_optimum_the_other_curve_is_used() {
        // D = -4, k = -1/8, P4 = 3, W = 1/32, P1 = 3; the rpm is not below the minimum.
        assert_eq!(run(2.0, 12.0), (6.0 - 3.0, 12.0));
    }

    #[test]
    fn below_the_minimum_the_rpm_is_raised_by_the_throttle() {
        // g = 0.5 * 2 = 1, h = 4. At h the raised rpm is the minimum; D = -8, P4 = 5, P1 = 6
        // and the rpm 4 is below the minimum: y = (4 / 8) * 5 = 2.5.
        assert_eq!(run(2.0, 4.0), (12.0 - 2.5, 8.0));
        // Between h and the minimum: 8 + (1 * (6 - 4)) / (8 - 4) = 8.5.
        assert_eq!(run(2.0, 6.0).1, 8.5);
        // Below h: g + min = 9.
        assert_eq!(run(2.0, 2.0).1, 9.0);
    }

    #[test]
    fn a_nan_rpm_counts_as_above_the_maximum() {
        // Every comparison with a NaN is unordered, which the exe treats as "not below".
        assert_eq!(run(1.0, f32::NAN), (-3.0, 32.0));
    }
}
