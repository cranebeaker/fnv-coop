//! `Havok SDK/hkpvehicledefaultanalogdriverinput.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PDB has no vehicle types: the layouts and their names are taken from what the code reads
//! (the names follow the Havok SDK's `hkpVehicleDefaultAnalogDriverInput` and
//! `hkpVehicleDriverInputAnalogStatus`, unconfirmed). The default input's vtable (`010e5da8`)
//! has `calcDriverInput` in slot `0xC`, `calcAcceleratorInput` in `0x10`, `calcBrakeInput` in
//! `0x14`, `calcTryingToReverse` in `0x18` and `calcSteeringInput` in `0x1C`.
//!
//! The driver input output record: floats `+0` (accelerator), `+4` (brake) and `+8` (steering),
//! byte `+0xC` (handbrake) and byte `+0xD` (trying to reverse). The vehicle instance's byte at
//! `+0xCC` is its "is reversing" flag.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

layout! {
    /// `hkpVehicleDefaultAnalogDriverInput` (names unconfirmed).
    pub struct HkpVehicleDefaultAnalogDriverInput: 0x18 {
        /// `m_slopeChangePointX`.
        0x08 m_slope_change_point_x: f32,
        /// `m_initialSlope`.
        0x0c m_initial_slope: f32,
        /// `m_deadZone`.
        0x10 m_dead_zone: f32,
        /// `m_autoReverse`.
        0x14 m_auto_reverse: u8,
    }
}

layout! {
    /// `hkpVehicleDriverInputAnalogStatus` (Xbox PDB class; the field names are unconfirmed
    /// and taken from the Havok SDK).
    pub struct HkpVehicleDriverInputAnalogStatus: 0x14 {
        /// `m_memSizeAndFlags` (16 bits at `+4`) and the next 16 bits at `+6`.
        0x04 m_mem_size: u16,
        0x06 m_reference_count: u16,
        /// `m_positionX`.
        0x08 m_position_x: f32,
        /// `m_positionY`.
        0x0c m_position_y: f32,
        /// `m_handbrakeEnabled`.
        0x10 m_handbrake_enabled: u8,
        /// `m_reverseButtonPressed`.
        0x11 m_reverse_button_pressed: u8,
    }
}

/// The memory router (`00c85750`): its field `+0x10` is the allocator; slot `4` of it
/// allocates the number of bytes it is given.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable of an `hkpVehicleDriverInputAnalogStatus` (written by `clone`).
const ANALOG_STATUS_VTABLE: u32 = 0x010e_5d94;
/// The vtable slots of the input object.
const SLOT_ACCELERATOR: u32 = 0x10;
const SLOT_BRAKE: u32 = 0x14;
const SLOT_TRYING_TO_REVERSE: u32 = 0x18;
const SLOT_STEERING: u32 = 0x1c;

type Vec4 = [f32; 4];

fn read_vec4(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|i| f32::from_bits(e.mem.u32(address + 4 * i as u32)))
}

/// `MULPS`.
fn mul(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] * b[i])
}

/// `ADDPS`.
fn add(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|i| a[i] + b[i])
}

/// The scalar sum the exe builds from a four-lane vector: `ADDSS` of lane 1 and lane 0, then
/// lane 2 plus that.
fn sum_xyz(a: Vec4) -> f32 {
    a[2] + (a[1] + a[0])
}

/// One Newton step on `RSQRTSS`: `0.5 * r * (3.0 - (x * r) * r)` with `r` the estimate; zero
/// when `x == 0.0` (`CMPEQSS` and `ANDNPS`). The constants are the floats `3.0` (`01017718`)
/// and `0.5` (`01016248`), the zero at `01011d78`.
fn inverse_length(x: f32) -> f32 {
    let r = rsqrt_estimate(x);
    let refined = (0.5 * r) * (3.0 - (x * r) * r);
    if x == 0.0 {
        0.0
    } else {
        refined
    }
}

/// The accelerator/brake pedal input shared by two functions: the float at `status + 0xC`
/// (through the x87), negated when the instance is reversing and the input has `m_autoReverse`.
fn pedal(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    instance: Ptr,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
) -> f32 {
    let reversing = e.mem.u8(instance.addr() + 0xcc) != 0;
    let auto = e.get(this, HkpVehicleDefaultAnalogDriverInput::m_auto_reverse) != 0;
    let value = x87_quiet(e.get(status, HkpVehicleDriverInputAnalogStatus::m_position_y));
    if reversing && auto {
        -value
    } else {
        value
    }
}

// Translated from 00e43540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAnalogDriverInput::calcAcceleratorInput` (Xbox PDB), `(this, _, instance,
/// status, _)`; the first and last stack words are not read. `p` = `status.positionY`, negated
/// when the instance's reversing byte (`+0xCC`) and `m_autoReverse` are both set. Returns `1.0`
/// when the instance is reversing and `m_autoReverse` is not set; else `0.0` for `p > 0`, and
/// `float(p * -1.0)` otherwise (so `-0.0` for zero).
pub fn calc_accelerator_input(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    _unused_1: u32,
    instance: Ptr,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
    _unused_4: u32,
) -> f32 {
    let reversing = e.mem.u8(instance.addr() + 0xcc) != 0;
    let value = pedal(e, this, instance, status);
    if e.get(this, HkpVehicleDefaultAnalogDriverInput::m_auto_reverse) == 0 && reversing {
        return 1.0;
    }
    if value > 0.0 {
        return 0.0;
    }
    // `black_box` keeps the multiply (00e4358f `FMUL`): LLVM folds `x * -1.0` into a negation
    // in release builds, which would flip a NaN's sign.
    x87_quiet((f64::from(value) * std::hint::black_box(-1.0)) as f32)
}

// Translated from 00e435a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAnalogDriverInput::calcBrakeInput` (Xbox PDB), `(this, _, instance,
/// status, _)`. The pedal value `p` as in `calcAcceleratorInput`; the answer is `0.0` when
/// `p < 0` and `p` otherwise (also for a NaN). (The decompiler shows a function without a
/// result; the exe returns it in `ST0`.)
pub fn calc_brake_input(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    _unused_1: u32,
    instance: Ptr,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
    _unused_4: u32,
) -> f32 {
    let value = pedal(e, this, instance, status);
    if value < 0.0 {
        0.0
    } else {
        value
    }
}

// Translated from 00e435f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAnalogDriverInput::calcDriverInput` (Xbox PDB), `(this, delta time,
/// instance, status, output)`. Calls, each with `(delta time, instance, status, output)` (the
/// float as its bits through the x87) and the answer in `ST0`: slot `0x10` into `output[0]`,
/// slot `0x14` into `output[1]`, slot `0x1C` into `output[2]`; then `output + 0xC` = the byte
/// `status + 0x10`, and slot `0x18` with `(scratch, delta time, instance, status, output)`
/// answers a pointer to a byte that goes to `output + 0xD`.
pub fn calc_driver_input(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    delta_time: f32,
    instance: Ptr,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
    output: Ptr,
) {
    let bits = x87_quiet_bits(delta_time.to_bits());
    let out = output.addr();
    for (slot, offset) in [(SLOT_ACCELERATOR, 0), (SLOT_BRAKE, 4)] {
        let value = e
            .vcall(this.addr(), slot, &args![bits, instance, status, output])
            .f32();
        e.mem.set_f32(out + offset, value);
    }
    let steering = e
        .vcall(
            this.addr(),
            SLOT_STEERING,
            &args![bits, instance, status, output],
        )
        .f32();
    e.mem.set_f32(out + 8, steering);
    let handbrake = e.get(
        status,
        HkpVehicleDriverInputAnalogStatus::m_handbrake_enabled,
    );
    e.mem.set_u8(out + 0xc, handbrake);
    let reverse = e.with_stack(4, |e, scratch| {
        let at = e
            .vcall(
                this.addr(),
                SLOT_TRYING_TO_REVERSE,
                &args![scratch, bits, instance, status, output],
            )
            .u32();
        e.mem.u8(at)
    });
    e.mem.set_u8(out + 0xd, reverse);
}

// Translated from 00e436b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAnalogDriverInput::calcSteeringInput` (Xbox PDB), `(this, _, _, status,
/// _)`; only the third stack word is read. With `x = status.positionX`, `a = |x|` (stored as
/// `float`), the dead zone `d`, the slope change point `p`, the initial slope `m` and `sign` =
/// `1.0` when `0.0 < x` and `-1.0` otherwise (also for a NaN):
/// - `0.0` when `a < d`;
/// - `float(float((a - d) * m) * sign)` when `a < p`;
/// - else with `c = float(m * (p - d))` and `q = float((1 - c) / ((1 - d) - (p - d)))` (double
///   precision between the stores): `float(float((a - p) * q + c) * sign)`.
pub fn calc_steering_input(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    _unused_1: u32,
    _unused_2: u32,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
    _unused_4: u32,
) -> f32 {
    let x = x87_quiet(e.get(status, HkpVehicleDriverInputAnalogStatus::m_position_x));
    let a = x.abs();
    let dead_zone = e.get(this, HkpVehicleDefaultAnalogDriverInput::m_dead_zone);
    if a < dead_zone {
        return 0.0;
    }
    let sign = if 0.0 < x { 1.0f32 } else { -1.0f32 };
    let point = e.get(
        this,
        HkpVehicleDefaultAnalogDriverInput::m_slope_change_point_x,
    );
    let slope = e.get(this, HkpVehicleDefaultAnalogDriverInput::m_initial_slope);
    let wide = f64::from;
    // Arithmetic in the exe's operand order with the x87 NaN choice (#199): `FSUB`/`FMUL
    // dword ptr` read the field from memory (`x87_m32`); `FLD` values are quiet.
    let dead_zone_m = x87_m32(dead_zone.to_bits());
    if point > a {
        // FSUB dword deadZone; FMUL dword initialSlope; FSTP/FLD; FMUL dword sign.
        let y = x87_mul(x87_sub(wide(a), dead_zone_m), x87_m32(slope.to_bits())) as f32;
        return x87_quiet(x87_mul(wide(y), x87_m32(sign.to_bits())) as f32);
    }
    // FLD point; FSUB dword deadZone; FLD initialSlope; FMUL ST0,ST1 (D8 C9).
    let span = x87_sub(wide(x87_quiet(point)), dead_zone_m);
    let c = x87_mul(wide(x87_quiet(slope)), span) as f32;
    // FSUBRP ST2 (1 - c); FSUB dword deadZone; FSUBRP ST3; FDIVRP ST2.
    let one_minus_c = x87_sub(1.0, wide(c));
    let q = x87_div(one_minus_c, x87_sub(x87_sub(1.0, dead_zone_m), span)) as f32;
    // FLD point; FSUBP ST3 (a - point); FMULP ST2; FADDP.
    let z = x87_add(
        x87_mul(x87_sub(wide(a), wide(x87_quiet(point))), wide(q)),
        wide(c),
    ) as f32;
    x87_quiet(x87_mul(wide(z), x87_m32(sign.to_bits())) as f32)
}

// Translated from 00e43780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDefaultAnalogDriverInput::calcTryingToReverse` (Xbox PDB), `(this, result, _,
/// instance, status, _)`; the second and last stack words are not read. Writes the byte at
/// `result` and returns `result`. Without `m_autoReverse` the byte is `status.reverseButtonPressed`.
/// Otherwise (`body` = the word at `instance + 0x18`, `reversing` = the byte at
/// `instance + 0xCC`, `speed` = the square root of the squared length of the velocity at
/// `body + 0x1B0` as a `float`, `slow` = `speed < 1.3888889` (the float at `010ebf10`),
/// `forward` = `u.y * r1 + u.x * r0 + u.z * r2` with `u` the vector at `data + 0x40` (`data`
/// = the word at `instance + 0x1C`) and `r0`, `r1`, `r2` the matrix columns at `body + 0xF0`,
/// `0x100`, `0x110` (all four-lane `float` operations), `ahead` = `positionY > 0.1` (the double
/// at `0101ffa0`, the float 0.1 widened)):
/// 1. when `ahead` and `speed > 1.1920929e-7` (the float at `0104da4c`) the normalised
///    velocity (`inverse_length` steps with the `RSQRTSS` estimate) is dotted with the
///    normalised `forward`, and a dot product below `1.1920929e-7` ends with the byte `1`;
/// 2. otherwise the byte is `ahead ? 1 : 0` when the instance is not reversing and `slow`, `0`
///    when it is not reversing and not slow, `1` when reversing and not slow, and `ahead` when
///    reversing and slow.
pub fn calc_trying_to_reverse(
    e: &mut Engine,
    this: Ptr<HkpVehicleDefaultAnalogDriverInput>,
    result: Ptr,
    _unused_2: u32,
    instance: Ptr,
    status: Ptr<HkpVehicleDriverInputAnalogStatus>,
    _unused_5: u32,
) -> Ptr {
    let out = result.addr();
    if e.get(this, HkpVehicleDefaultAnalogDriverInput::m_auto_reverse) == 0 {
        let pressed = e.get(
            status,
            HkpVehicleDriverInputAnalogStatus::m_reverse_button_pressed,
        );
        e.mem.set_u8(out, pressed);
        return result;
    }
    let body = e.mem.u32(instance.addr() + 0x18);
    let velocity = read_vec4(e, body + 0x1b0);
    let reversing = e.mem.u8(instance.addr() + 0xcc) != 0;
    let squared = sum_xyz(mul(velocity, velocity));
    let speed = squared.sqrt();
    let threshold = f32::from_bits(0x3fb1_c71d);
    let slow = speed < threshold;
    let position_y = f64::from(x87_quiet(
        e.get(status, HkpVehicleDriverInputAnalogStatus::m_position_y),
    ));
    let ahead = position_y > f64::from(0.1f32);
    let epsilon = f32::from_bits(0x3400_0000);
    if ahead && epsilon < speed {
        let data = e.mem.u32(instance.addr() + 0x1c);
        let u = read_vec4(e, data + 0x40);
        let r0 = read_vec4(e, body + 0xf0);
        let r1 = read_vec4(e, body + 0x100);
        let r2 = read_vec4(e, body + 0x110);
        let forward = add(
            add(mul([u[1]; 4], r1), mul([u[0]; 4], r0)),
            mul([u[2]; 4], r2),
        );
        let inverse_speed = inverse_length(squared);
        let inverse_forward = inverse_length(sum_xyz(mul(forward, forward)));
        let product = mul(
            mul([inverse_speed; 4], velocity),
            mul([inverse_forward; 4], forward),
        );
        if epsilon > sum_xyz(product) {
            e.mem.set_u8(out, 1);
            return result;
        }
    }
    let value = if reversing {
        if slow {
            u8::from(ahead)
        } else {
            1
        }
    } else {
        u8::from(slow && ahead)
    };
    e.mem.set_u8(out, value);
    result
}

// Translated from 00e43a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpVehicleDriverInputAnalogStatus::clone` (Xbox PDB), `this` = the status to copy: allocates
/// 0x14 bytes through the memory router's allocator (slot `4`), writes the vtable `010e5d94`
/// and the words `0x14` and `1` at `+4` and `+6` (which the copy overwrites at once with the
/// source's), copies the floats at `+8` and `+0xC` through the x87 and the bytes at `+0x10`
/// and `+0x11`. Returns the new block.
pub fn hkp_vehicle_driver_input_analog_status_clone(
    e: &mut Engine,
    this: Ptr<HkpVehicleDriverInputAnalogStatus>,
) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x14u32]).u32();
    e.mem.set_u32(block, ANALOG_STATUS_VTABLE);
    e.mem.set_u16(block + 4, 0x14);
    e.mem.set_u16(block + 6, 1);
    let size = e.get(this, HkpVehicleDriverInputAnalogStatus::m_mem_size);
    e.mem.set_u16(block + 4, size);
    let count = e.get(this, HkpVehicleDriverInputAnalogStatus::m_reference_count);
    e.mem.set_u16(block + 6, count);
    for offset in [8, 0xc] {
        let bits = x87_quiet_bits(e.mem.u32(this.addr() + offset));
        e.mem.set_u32(block + offset, bits);
    }
    let handbrake = e.get(this, HkpVehicleDriverInputAnalogStatus::m_handbrake_enabled);
    e.mem.set_u8(block + 0x10, handbrake);
    let reverse = e.get(
        this,
        HkpVehicleDriverInputAnalogStatus::m_reverse_button_pressed,
    );
    e.mem.set_u8(block + 0x11, reverse);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e4_3540,
            calc_accelerator_input(
                Ptr<HkpVehicleDefaultAnalogDriverInput>,
                u32,
                Ptr,
                Ptr<HkpVehicleDriverInputAnalogStatus>,
                u32,
            ) -> f32
        ),
        entry!(
            0x00e4_35a0,
            calc_brake_input(
                Ptr<HkpVehicleDefaultAnalogDriverInput>,
                u32,
                Ptr,
                Ptr<HkpVehicleDriverInputAnalogStatus>,
                u32,
            ) -> f32
        ),
        entry!(
            0x00e4_35f0,
            calc_driver_input(
                Ptr<HkpVehicleDefaultAnalogDriverInput>,
                f32,
                Ptr,
                Ptr<HkpVehicleDriverInputAnalogStatus>,
                Ptr,
            )
        ),
        entry!(
            0x00e4_36b0,
            calc_steering_input(
                Ptr<HkpVehicleDefaultAnalogDriverInput>,
                u32,
                u32,
                Ptr<HkpVehicleDriverInputAnalogStatus>,
                u32,
            ) -> f32
        ),
        entry!(
            0x00e4_3780,
            calc_trying_to_reverse(
                Ptr<HkpVehicleDefaultAnalogDriverInput>,
                Ptr,
                u32,
                Ptr,
                Ptr<HkpVehicleDriverInputAnalogStatus>,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x00e4_3a60,
            hkp_vehicle_driver_input_analog_status_clone(
                Ptr<HkpVehicleDriverInputAnalogStatus>,
            ) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, new_log, Log};

    /// An input object: dead zone 0.125, slope change point 0.5, initial slope 2, the given
    /// `m_autoReverse`; the vtable slots `0x10` to `0x1C` are logging doubles answering 0.
    fn input(e: &mut Engine, log: &Log, auto_reverse: u8) -> u32 {
        let this = e.mem.alloc(0x18);
        e.mem.set_f32(this + 8, 0.5);
        e.mem.set_f32(this + 0xc, 2.0);
        e.mem.set_f32(this + 0x10, 0.125);
        e.mem.set_u8(this + 0x14, auto_reverse);
        let vtable = e.mem.alloc(0x20);
        for slot in (0x10..0x20).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x0900_0000 + slot);
            let log = std::rc::Rc::clone(log);
            e.register_double(0x0900_0000 + slot, move |_, a| {
                log.borrow_mut().push((0x0900_0000 + slot, a.to_vec()));
                Ret::default()
            });
        }
        e.mem.set_u32(this, vtable);
        this
    }

    /// A status with the given X and Y and the two flag bytes.
    fn status(e: &mut Engine, x: f32, y: f32, handbrake: u8, reverse: u8) -> u32 {
        let status = e.mem.alloc(0x14);
        e.mem.set_f32(status + 8, x);
        e.mem.set_f32(status + 0xc, y);
        e.mem.set_u8(status + 0x10, handbrake);
        e.mem.set_u8(status + 0x11, reverse);
        status
    }

    fn instance_with(e: &mut Engine, reversing: u8) -> u32 {
        let instance = e.mem.alloc(0xe0);
        e.mem.set_u8(instance + 0xcc, reversing);
        instance
    }

    #[test]
    fn the_accelerator_follows_the_pedal_and_the_reversing_state() {
        let mut e = engine();
        let log = new_log();
        let automatic = input(&mut e, &log, 1);
        let manual = input(&mut e, &log, 0);
        let forward = instance_with(&mut e, 0);
        let backward = instance_with(&mut e, 1);
        let pushed = status(&mut e, 0.0, 0.5, 0, 0);
        let pulled = status(&mut e, 0.0, -0.5, 0, 0);
        let idle = status(&mut e, 0.0, 0.0, 0, 0);
        let run = |e: &mut Engine, this: u32, instance: u32, status: u32| {
            e.call(0x00e4_3540, &args![this, 0u32, instance, status, 0u32])
                .f32()
        };
        // Pushing forward means no accelerator input here (the other pedal is the brake).
        assert_eq!(run(&mut e, automatic, forward, pushed), 0.0);
        assert_eq!(run(&mut e, automatic, forward, pulled), 0.5);
        // Reversing with auto reverse flips the pedal.
        assert_eq!(run(&mut e, automatic, backward, pushed), 0.5);
        assert_eq!(run(&mut e, automatic, backward, pulled), 0.0);
        // Reversing without auto reverse: full input.
        assert_eq!(run(&mut e, manual, backward, pushed), 1.0);
        // Zero times -1.0 is negative zero; a NaN stays one.
        assert_eq!(run(&mut e, automatic, forward, idle).to_bits(), 0x8000_0000);
        let nan = status(&mut e, 0.0, f32::NAN, 0, 0);
        assert!(run(&mut e, automatic, forward, nan).is_nan());
    }

    #[test]
    fn the_brake_is_the_pedal_unless_it_is_negative() {
        let mut e = engine();
        let log = new_log();
        let automatic = input(&mut e, &log, 1);
        let forward = instance_with(&mut e, 0);
        let backward = instance_with(&mut e, 1);
        let pushed = status(&mut e, 0.0, 0.5, 0, 0);
        let pulled = status(&mut e, 0.0, -0.5, 0, 0);
        let run = |e: &mut Engine, instance: u32, status: u32| {
            e.call(0x00e4_35a0, &args![automatic, 0u32, instance, status, 0u32])
                .f32()
        };
        assert_eq!(run(&mut e, forward, pushed), 0.5);
        assert_eq!(run(&mut e, forward, pulled), 0.0);
        assert_eq!(run(&mut e, backward, pulled), 0.5);
        assert_eq!(run(&mut e, backward, pushed), 0.0);
    }

    #[test]
    fn the_driver_input_asks_the_four_slots() {
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 1);
        for (slot, st0) in [(0x10u32, 0.5f64), (0x14, 0.25), (0x1c, -0.75)] {
            let log = std::rc::Rc::clone(&log);
            e.register_double(0x0900_0000 + slot, move |_, a| {
                log.borrow_mut().push((0x0900_0000 + slot, a.to_vec()));
                Ret {
                    st0,
                    ..Ret::default()
                }
            });
        }
        let sink = std::rc::Rc::clone(&log);
        e.register_double(0x0900_0018, move |e, a| {
            sink.borrow_mut().push((0x0900_0018, a.to_vec()));
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let status = status(&mut e, 0.0, 0.0, 1, 0);
        let output = e.mem.alloc(0x10);
        let instance = 0x1234u32;
        e.call(0x00e4_35f0, &args![this, 0.5f32, instance, status, output]);
        assert_eq!(
            [
                e.mem.f32(output),
                e.mem.f32(output + 4),
                e.mem.f32(output + 8)
            ],
            [0.5, 0.25, -0.75]
        );
        assert_eq!([e.mem.u8(output + 0xc), e.mem.u8(output + 0xd)], [1, 1]);
        let words = |slot: u32| calls(&log, 0x0900_0000 + slot);
        let bits = 0.5f32.to_bits();
        assert_eq!(
            words(0x10),
            vec![vec![this, bits, instance, status, output]]
        );
        assert_eq!(
            words(0x14),
            vec![vec![this, bits, instance, status, output]]
        );
        assert_eq!(
            words(0x1c),
            vec![vec![this, bits, instance, status, output]]
        );
        let reverse = words(0x18);
        assert_eq!(reverse.len(), 1);
        assert_eq!(reverse[0][2..], [bits, instance, status, output]);
    }

    #[test]
    fn steering_has_a_dead_zone_a_linear_part_and_a_curved_part() {
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 1);
        let mut run = |x: f32| {
            let status = status(&mut e, x, 0.0, 0, 0);
            e.call(0x00e4_36b0, &args![this, 0u32, 0u32, status, 0u32])
                .f32()
        };
        // Inside the dead zone (and exactly zero).
        assert_eq!(run(0.0625), 0.0);
        assert_eq!(run(0.0), 0.0);
        // Linear part: (0.25 - 0.125) * 2.
        assert_eq!(run(0.25), 0.25);
        assert_eq!(run(-0.25), -0.25);
        // Curved part: c = 0.75, q = (1 - 0.75) / (0.875 - 0.375) = 0.5, (0.75 - 0.5) * 0.5 + 0.75.
        assert_eq!(run(0.75), 0.875);
        assert_eq!(run(-0.75), -0.875);
        // A NaN is not inside the dead zone, is negative, and takes the curved part.
        assert!(run(f32::NAN).is_nan());
    }

    #[test]
    fn steering_keeps_the_x87_nan_when_two_nans_meet() {
        // #199: -qNaN 0xffffffff and +qNaN 0x7fffffff meet; the x87 keeps the positive one.
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 1);
        let status = status(&mut e, 1.0, 0.0, 0, 0);
        // Linear part: (a - deadZone) * initialSlope with both NaNs.
        e.mem.set_f32(this + 8, 2.0);
        e.mem.set_u32(this + 0xc, 0x7fff_ffff);
        e.mem.set_u32(this + 0x10, 0xffff_ffff);
        let linear = e
            .call(0x00e4_36b0, &args![this, 0u32, 0u32, status, 0u32])
            .f32();
        assert_eq!(linear.to_bits(), 0x7fff_ffff);
        // Curved part: point - deadZone with both NaNs.
        e.mem.set_u32(this + 8, 0xffff_ffff);
        e.mem.set_f32(this + 0xc, 2.0);
        e.mem.set_u32(this + 0x10, 0x7fff_ffff);
        let curved = e
            .call(0x00e4_36b0, &args![this, 0u32, 0u32, status, 0u32])
            .f32();
        assert_eq!(curved.to_bits(), 0x7fff_ffff);
    }

    /// A body with the identity matrix columns, the given velocity and a vehicle data whose
    /// vector at `+0x40` is `(0, 1, 0, 0)`; returns `(instance, status)` with the given reversing
    /// byte and `positionY`.
    fn moving(e: &mut Engine, velocity: [f32; 3], reversing: u8, position_y: f32) -> (u32, u32) {
        let body = e.mem.alloc(0x200);
        for (i, lane) in velocity.iter().enumerate() {
            e.mem.set_f32(body + 0x1b0 + 4 * i as u32, *lane);
        }
        for (column, offset) in [(0u32, 0xf0u32), (1, 0x100), (2, 0x110)] {
            e.mem.set_f32(body + offset + 4 * column, 1.0);
        }
        let data = e.mem.alloc(0x80);
        e.mem.set_f32(data + 0x44, 1.0);
        let instance = e.mem.alloc(0xe0);
        e.mem.set_u32(instance + 0x18, body);
        e.mem.set_u32(instance + 0x1c, data);
        e.mem.set_u8(instance + 0xcc, reversing);
        (instance, status(e, 0.0, position_y, 0, 0))
    }

    fn trying(e: &mut Engine, this: u32, instance: u32, status: u32) -> u8 {
        let result = e.mem.alloc(4);
        let got = e
            .call(
                0x00e4_3780,
                &args![this, result, 0u32, instance, status, 0u32],
            )
            .u32();
        assert_eq!(got, result);
        e.mem.u8(result)
    }

    #[test]
    fn without_auto_reverse_the_button_decides() {
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 0);
        let pressed = status(&mut e, 0.0, 0.0, 0, 1);
        let released = status(&mut e, 0.0, 0.0, 0, 0);
        assert_eq!(trying(&mut e, this, 0, pressed), 1);
        assert_eq!(trying(&mut e, this, 0, released), 0);
    }

    #[test]
    fn driving_backwards_while_pushing_forward_is_trying_to_reverse() {
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 1);
        // Forward is +y in these rigs. Moving forward at speed 2: not reversing.
        let (instance, status) = moving(&mut e, [0.0, 2.0, 0.0], 0, 0.5);
        assert_eq!(trying(&mut e, this, instance, status), 0);
        // Moving backwards: the dot product is about -1.
        let (instance, status) = moving(&mut e, [0.0, -2.0, 0.0], 0, 0.5);
        assert_eq!(trying(&mut e, this, instance, status), 1);
        // Sideways: the dot product is zero, which is below 1.19e-7 only by rounding; the
        // estimate makes it exactly zero here (the y lane is zero).
        let (instance, status) = moving(&mut e, [2.0, 0.0, 0.0], 0, 0.5);
        assert_eq!(trying(&mut e, this, instance, status), 1);
    }

    #[test]
    fn standing_still_depends_on_the_pedal_and_the_reversing_state() {
        let mut e = engine();
        let log = new_log();
        let this = input(&mut e, &log, 1);
        // Slow (speed 0), not reversing: the byte is "pedal above 0.1".
        let (instance, status) = moving(&mut e, [0.0; 3], 0, 0.5);
        assert_eq!(trying(&mut e, this, instance, status), 1);
        let (instance, status) = moving(&mut e, [0.0; 3], 0, 0.05);
        assert_eq!(trying(&mut e, this, instance, status), 0);
        // Slow and reversing: the same.
        let (instance, status) = moving(&mut e, [0.0; 3], 1, 0.5);
        assert_eq!(trying(&mut e, this, instance, status), 1);
        let (instance, status) = moving(&mut e, [0.0; 3], 1, 0.05);
        assert_eq!(trying(&mut e, this, instance, status), 0);
        // Fast (2) with the pedal below 0.1: not slow; reversing gives 1, driving gives 0.
        let (instance, status) = moving(&mut e, [0.0, 2.0, 0.0], 1, 0.05);
        assert_eq!(trying(&mut e, this, instance, status), 1);
        let (instance, status) = moving(&mut e, [0.0, 2.0, 0.0], 0, 0.05);
        assert_eq!(trying(&mut e, this, instance, status), 0);
    }

    #[test]
    fn cloning_copies_the_status_into_a_new_block() {
        let mut e = engine();
        let allocated = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let block = e.mem.alloc(0x14);
        let sink = std::rc::Rc::clone(&allocated);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let source = status(&mut e, 0.25, -0.5, 1, 1);
        e.mem.set_u16(source + 4, 0x1234);
        e.mem.set_u16(source + 6, 7);
        let copy = e.call(0x00e4_3a60, &args![source]).u32();
        assert_eq!(copy, block);
        assert_eq!(*allocated.borrow(), vec![vec![allocator, 0x14]]);
        assert_eq!(e.mem.u32(copy), ANALOG_STATUS_VTABLE);
        assert_eq!([e.mem.u16(copy + 4), e.mem.u16(copy + 6)], [0x1234, 7]);
        assert_eq!([e.mem.f32(copy + 8), e.mem.f32(copy + 0xc)], [0.25, -0.5]);
        assert_eq!([e.mem.u8(copy + 0x10), e.mem.u8(copy + 0x11)], [1, 1]);
    }
}
