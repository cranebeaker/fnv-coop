//! `Havok SDK/hkpmotorbuildjacobian.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The motor data computation (`hkCalcMotorData`) and the two steps of the one-dimensional
//! linear velocity motor Jacobian. The x87 code keeps its intermediate values in extended
//! precision (the process runs with a 53-bit mantissa), so each chain of x87 operations
//! without a store is computed in `f64` and rounded to `float` only where the exe stores.

use crate::prelude::*;

/// The `double` 1.1920929e-07 (`FLT_EPSILON`) the Jacobian steps add.
const EPSILON: u32 = 0x010c_9300;

type Lanes = [f32; 4];

/// A `MOVAPS` load of four lanes.
fn load(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

/// A `MOVAPS` store of four lanes.
fn store(e: &mut Engine, addr: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

fn mul(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

fn sub(a: Lanes, b: Lanes) -> Lanes {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// `SHUFPS first, second, imm`: lanes 0 and 1 from `first`, lanes 2 and 3 from `second`.
fn shuffle(first: Lanes, second: Lanes, imm: u8) -> Lanes {
    let pick = |shift: u8| usize::from((imm >> shift) & 3);
    [
        first[pick(0)],
        first[pick(2)],
        second[pick(4)],
        second[pick(6)],
    ]
}

/// One lane broadcast to all four (`SHUFPS v, v, 0x00 / 0x55 / 0xaa`).
fn splat(lanes: Lanes, lane: usize) -> Lanes {
    [lanes[lane]; 4]
}

/// `(y * row1 + x * row0) + z * row2` over the three rows at `body + 0x50`, `+0x60`, `+0x70`.
fn through_rows(e: &Engine, body: u32, vector: Lanes) -> Lanes {
    let by_y = mul(splat(vector, 1), load(e, body + 0x60));
    let by_x = mul(splat(vector, 0), load(e, body + 0x50));
    let by_z = mul(splat(vector, 2), load(e, body + 0x70));
    add(add(by_y, by_x), by_z)
}

/// `FLD` of a `float`: the value as the x87 holds it (a signalling NaN comes out quiet).
fn fld(e: &Engine, addr: u32) -> f64 {
    f64::from(x87_quiet(e.mem.f32(addr)))
}

/// A float copied through the x87 stack (`FLD` / `FSTP`), as the bits stored.
fn copy_bits(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// `0` below zero, `1` above one, else the value itself (a NaN stays).
// The exe tests below zero first, then above one; a NaN falls through both.
#[allow(clippy::manual_clamp)]
fn clamp_unit(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if 1.0 < value {
        1.0
    } else {
        value
    }
}

// Translated from 00dcfff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCalcMotorData` (Xbox PDB; cdecl): computes the six words of `output` from the motor
/// (`motor`: the kind byte at +8 selects the case; floats at +0xC..+0x20, a flag byte at
/// +0x1C for kind 2) and the `input` block (floats at +0, +0xC, +0x10 and +0x14 and the
/// pointer at +4 to a block with floats at +0, +0xC and +0x10), returning without a write for
/// any other kind. All the loads and copies pass through the x87 stack.
///
/// Kind 1: `output[4]` etc. are the motor floats `+0x10`, `-(+0x10)`, `+0x14`, `+0x18`;
/// `output[1]` is `(a + input[4]) * block[0x10]` with `a` the speed `input[5]` moved towards
/// `input[5] * motor[0x1C] * block[0xC]` by at most `motor[0x20] * block[0xC]` and clamped to
/// `±|input[5]| - input[3]`. Kind 2: `output[1]` is `motor[0x18]`, or
/// `(input[5] + input[4]) * block[0x10]` when the byte at motor +0x1C is nonzero; the others
/// are `input[3]`, motor `+0x10`, `+0xC`, `+0x14`, `+0x14`. Kind 3: `output[0]` is
/// `input[5] + input[4]` rounded, `output[1]` is zero, `output[2..4]` are motor `+0x10` and
/// `+0xC`, and `output[4]` and `output[5]` are two quantities clamped to 0..1
/// (`motor[0x14] * block[0]² / input[0]` and `motor[0x18] * block[0] / input[0]`). Kind 4 jumps
/// to the function pointer at motor +0x14 with the same three arguments.
pub fn hk_calc_motor_data(e: &mut Engine, motor: Ptr, input: Ptr, output: Ptr) {
    let (motor, input, output) = (motor.addr(), input.addr(), output.addr());
    let kind = i32::from(e.mem.u8(motor + 8) as i8);
    let block = e.mem.u32(input + 4);
    let words: [u32; 6] = match kind {
        1 => {
            let speed = fld(e, input + 0x14);
            let block_factor = fld(e, block + 0xc);
            let reached = ((fld(e, motor + 0x1c) * speed) * block_factor) as f32;
            let difference = (speed - f64::from(reached)) as f32;
            let step = (fld(e, motor + 0x20) * block_factor) as f32;
            let moved = if step < difference.abs() {
                let step = if 0.0 < difference { step } else { -step };
                (f64::from(reached) + f64::from(step)) as f32
            } else {
                speed as f32
            };
            let magnitude = f64::from((speed as f32).abs());
            let margin = fld(e, input + 0xc);
            let lower = ((-magnitude) - margin) as f32;
            let upper = (magnitude - margin) as f32;
            let limited = if moved < lower {
                lower
            } else if upper < moved {
                upper
            } else {
                moved
            };
            let shifted =
                ((f64::from(limited) + fld(e, input + 0x10)) * fld(e, block + 0x10)) as f32;
            let axis = copy_bits(e, motor + 0x10);
            [
                copy_bits(e, input + 0xc),
                shifted.to_bits(),
                axis,
                axis ^ 0x8000_0000,
                copy_bits(e, motor + 0x14),
                copy_bits(e, motor + 0x18),
            ]
        }
        2 => {
            let shifted = if e.mem.u8(motor + 0x1c) == 0 {
                copy_bits(e, motor + 0x18)
            } else {
                (((fld(e, input + 0x14) + fld(e, input + 0x10)) * fld(e, block + 0x10)) as f32)
                    .to_bits()
            };
            [
                copy_bits(e, input + 0xc),
                shifted,
                copy_bits(e, motor + 0x10),
                copy_bits(e, motor + 0xc),
                copy_bits(e, motor + 0x14),
                copy_bits(e, motor + 0x14),
            ]
        }
        3 => {
            let total = (fld(e, input + 0x14) + fld(e, input + 0x10)) as f32;
            let inverse = (1.0 / fld(e, input)) as f32;
            let scale = f64::from(x87_quiet(e.mem.f32(block)));
            let first = ((fld(e, motor + 0x14) * scale) * scale) * f64::from(inverse);
            let second = f64::from(inverse) * (scale * fld(e, motor + 0x18));
            [
                total.to_bits(),
                0,
                copy_bits(e, motor + 0x10),
                copy_bits(e, motor + 0xc),
                clamp_unit(first as f32).to_bits(),
                clamp_unit(second as f32).to_bits(),
            ]
        }
        4 => {
            let target = e.mem.u32(motor + 0x14);
            e.call(target, &args![motor, input, output]);
            return;
        }
        _ => return,
    };
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(output + 4 * index as u32, word);
    }
}

// Translated from 00dd0260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearVelocityMotorBeginJacobian` (Xbox PDB; cdecl): `direction` and `point` are
/// 16-byte vectors; `info` has the two body blocks as the words at +0x30 and +0x34 (each
/// with a weight vector at +0x30, the position at +0x40 and three rows at +0x50, +0x60 and
/// +0x70). Writes `direction` to `jacobian + 0`; the rows of body A applied to
/// `(point - positionA) x direction` to `jacobian + 0x10`; the rows of body B applied to
/// `direction x (point - positionB)` to `jacobian + 0x20`; then
/// `total = (weightA3 + weightB3 + sum over the lanes of rotated² * weight) + FLT_EPSILON`
/// (the lane sum in the order of the `SHUFPS` / `ADDPS` pairs), rounded to `float`, goes to
/// `jacobian + 0x2c`, and its reciprocal (rounded to `float`) to `jacobian + 0x1c` and
/// `*result`.
pub fn hk1d_linear_velocity_motor_begin_jacobian(
    e: &mut Engine,
    direction: Ptr,
    point: Ptr,
    info: Ptr,
    jacobian: Ptr,
    result: Ptr,
) {
    let jacobian = jacobian.addr();
    let body_a = e.mem.u32(info.addr() + 0x30);
    let body_b = e.mem.u32(info.addr() + 0x34);
    for word in 0..4u32 {
        let value = e.mem.u32(direction.addr() + 4 * word);
        e.mem.set_u32(jacobian + 4 * word, value);
    }
    let axis = load(e, direction.addr());
    let world = load(e, point.addr());
    let from_a = sub(world, load(e, body_a + 0x40));
    let from_b = sub(world, load(e, body_b + 0x40));
    let axis_zxy = shuffle(axis, axis, 0xd2);
    let axis_yzx = shuffle(axis, axis, 0xc9);
    let cross_a = sub(
        mul(shuffle(from_a, from_a, 0xc9), axis_zxy),
        mul(shuffle(from_a, from_a, 0xd2), axis_yzx),
    );
    let cross_b = sub(
        mul(shuffle(from_b, from_b, 0xd2), axis_yzx),
        mul(shuffle(from_b, from_b, 0xc9), axis_zxy),
    );
    let rotated_a = through_rows(e, body_a, cross_a);
    store(e, jacobian + 0x10, rotated_a);
    let rotated_b = through_rows(e, body_b, cross_b);
    store(e, jacobian + 0x20, rotated_b);
    let weight_a = load(e, body_a + 0x30);
    let weight_b = load(e, body_b + 0x30);
    let weighted = add(
        mul(mul(rotated_a, rotated_a), weight_a),
        mul(mul(rotated_b, rotated_b), weight_b),
    );
    let combined = [
        weighted[0],
        weighted[1],
        weighted[2],
        weight_a[3] + weight_b[3],
    ];
    let pairs = add(shuffle(combined, combined, 0x4e), combined);
    let total = add(shuffle(pairs, pairs, 0xb1), pairs)[0];
    let epsilon: f64 = e.global(EPSILON);
    let value = (f64::from(total) + epsilon) as f32;
    e.mem.set_f32(jacobian + 0x2c, value);
    let inverse = (1.0 / f64::from(value)) as f32;
    e.mem.set_f32(jacobian + 0x1c, inverse);
    e.mem.set_f32(result.addr(), inverse);
}

// Translated from 00dd03c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dLinearVelocityMotorCommitJacobian` (Xbox PDB; cdecl): `begin + 0xc` gets
/// `state[2] * block[0]`. At the cursor `*cursor` a record of 0x50 bytes is started: its first
/// float gets `FLT_EPSILON` added (through the x87 stack) and its low byte is then
/// overwritten with the tag 0x0a; `+0x30` is `block[2] * state[1]`, `+0x34` is
/// `block[3] * state[1]`, and `+0x38`, `+0x3c` and `+0x40` are the floats `block[1]`,
/// `block[4]` and `block[5]` (copied through the x87 stack). The cursor moves on by 0x50.
pub fn hk1d_linear_velocity_motor_commit_jacobian(
    e: &mut Engine,
    block: Ptr,
    state: Ptr,
    begin: Ptr,
    cursor: Ptr,
) {
    let (block, state) = (block.addr(), state.addr());
    let product = fld(e, state + 8) * fld(e, block);
    e.mem.set_f32(begin.addr() + 0xc, product as f32);
    let record = e.mem.u32(cursor.addr());
    let epsilon: f64 = e.global(EPSILON);
    let first = (fld(e, record) + epsilon) as f32;
    e.mem.set_f32(record, first);
    e.mem.set_u8(record, 0xa);
    e.mem.set_u32(cursor.addr(), record + 0x50);
    let scale = fld(e, state + 4);
    e.mem
        .set_f32(record + 0x30, (fld(e, block + 8) * scale) as f32);
    e.mem
        .set_f32(record + 0x34, (fld(e, block + 0xc) * scale) as f32);
    for (to, from) in [(0x38, 4), (0x3c, 0x10), (0x40, 0x14)] {
        let bits = copy_bits(e, block + from);
        e.mem.set_u32(record + to, bits);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_fff0, hk_calc_motor_data(Ptr, Ptr, Ptr)),
        entry!(
            0x00dd_0260,
            hk1d_linear_velocity_motor_begin_jacobian(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00dd_03c0,
            hk1d_linear_velocity_motor_commit_jacobian(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_9000, 0x1000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e
    }

    fn set_floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * index as u32, *value);
        }
    }

    fn floats(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|index| e.mem.f32(address + 4 * index))
            .collect()
    }

    /// A motor of the given kind with the floats at +0xC..+0x20 and the byte at +0x1C.
    fn motor_of(e: &mut Engine, kind: u8, floats: [f32; 5], flag: u8) -> u32 {
        let motor = e.mem.alloc(0x40);
        e.mem.set_u8(motor + 8, kind);
        set_floats(e, motor + 0xc, &floats[..4]);
        e.mem.set_f32(motor + 0x20, floats[4]);
        if flag != 0 {
            e.mem.set_u8(motor + 0x1c, flag);
        }
        motor
    }

    /// An input block (floats 0..6) and its info block (floats at +0, +0xC, +0x10).
    fn input_of(e: &mut Engine, values: [f32; 6], block: [f32; 3]) -> u32 {
        let input = e.mem.alloc(0x20);
        set_floats(e, input, &values);
        let info = e.mem.alloc(0x20);
        e.mem.set_f32(info, block[0]);
        e.mem.set_f32(info + 0xc, block[1]);
        e.mem.set_f32(info + 0x10, block[2]);
        e.mem.set_u32(input + 4, info);
        input
    }

    #[test]
    fn kind_one_moves_the_speed_towards_the_target_within_the_margin() {
        let mut e = engine();
        // Motor +0xC..+0x1C: unused, axis 0.5, 2, 8, 4 (the reached ratio); +0x20: step ratio 2.
        let motor = motor_of(&mut e, 1, [0.0, 0.5, 2.0, 0.25, 2.0], 0);
        e.mem.set_f32(motor + 0x1c, 0.25);
        // Speed 8, margin 1, offset 3, block factors 2 (+0xC) and 10 (+0x10).
        let input = input_of(&mut e, [0.0, 0.0, 0.0, 1.0, 3.0, 8.0], [0.0, 2.0, 10.0]);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        // reached = 0.25 * 8 * 2 = 4; difference 4; step = 2 * 2 = 4: not below, so the speed
        // stays 8; limits +-8 - 1 = -9 / 7: 8 > 7 gives 7; (7 + 3) * 10 = 100.
        assert_eq!(floats(&e, output, 6), [1.0, 100.0, 0.5, -0.5, 2.0, 0.25]);
    }

    #[test]
    fn kind_one_steps_by_the_limited_amount_when_far_from_the_target() {
        let mut e = engine();
        let motor = motor_of(&mut e, 1, [0.0, 1.0, 2.0, 3.0, 1.0], 0);
        e.mem.set_f32(motor + 0x1c, 0.5);
        // Speed 8, block factor 1: reached 4, difference 4, step 1 < 4: 4 + 1 = 5; margin 0:
        // limits -8 / 8; (5 + 0) * 1 = 5.
        let input = input_of(&mut e, [0.0, 0.0, 0.0, 0.0, 0.0, 8.0], [0.0, 1.0, 1.0]);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(floats(&e, output, 2), [0.0, 5.0]);
        // A negative difference steps down: speed -8 reaches -4, difference -4, so -4 - 1.
        e.mem.set_f32(input + 0x14, -8.0);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(e.mem.f32(output + 4), -5.0);
        // Speed 8 with the margin 6: the upper limit 8 - 6 = 2 cuts the stepped 5.
        e.mem.set_f32(input + 0x14, 8.0);
        e.mem.set_f32(input + 0xc, 6.0);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(e.mem.f32(output + 4), 2.0);
        // A very low reached value is cut by the lower limit -8.
        e.mem.set_f32(input + 0xc, 0.0);
        e.mem.set_f32(motor + 0x1c, -5.0);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(e.mem.f32(output + 4), -8.0);
    }

    #[test]
    fn kind_two_uses_the_stored_speed_or_the_input_speed() {
        let mut e = engine();
        let motor = motor_of(&mut e, 2, [0.5, 1.5, 2.5, 3.5, 4.5], 0);
        e.mem.set_u8(motor + 0x1c, 0);
        e.mem.set_f32(motor + 0x18, 9.0);
        let input = input_of(&mut e, [0.0, 0.0, 0.0, 7.0, 2.0, 3.0], [0.0, 0.0, 4.0]);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(floats(&e, output, 6), [7.0, 9.0, 1.5, 0.5, 2.5, 2.5]);
        e.mem.set_u8(motor + 0x1c, 1);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(e.mem.f32(output + 4), 20.0);
    }

    #[test]
    fn kind_three_clamps_the_two_ratios() {
        let mut e = engine();
        // motor +0xC: 11, +0x10: 12, +0x14: 0.5, +0x18: 0.5; input[0] = 2, block[0] = 2.
        let motor = motor_of(&mut e, 3, [11.0, 12.0, 0.5, 0.5, 0.0], 0);
        let input = input_of(&mut e, [2.0, 0.0, 0.0, 0.0, 1.0, 2.0], [2.0, 0.0, 0.0]);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        // first = 0.5 * 2 * 2 * (1/2) = 1 (not above one); second = (1/2) * (2 * 0.5) = 0.5.
        assert_eq!(floats(&e, output, 6), [3.0, 0.0, 12.0, 11.0, 1.0, 0.5]);
        e.mem.set_f32(motor + 0x14, 2.0);
        e.mem.set_f32(motor + 0x18, -1.0);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(floats(&e, output + 16, 2), [1.0, 0.0]);
    }

    #[test]
    fn kind_four_jumps_to_the_function_in_the_motor() {
        let mut e = engine();
        let motor = motor_of(&mut e, 4, [0.0; 5], 0);
        e.mem.set_u32(motor + 0x14, 0x0900_0005);
        e.register_double(0x0900_0005, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let input = e.mem.alloc(0x20);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|call| call.0 == 0x0900_0005)
            .map(|call| call.1)
            .collect();
        assert_eq!(calls, [vec![motor, input, output]]);
        assert_eq!(e.mem.u32(output), 0);
    }

    #[test]
    fn an_unknown_kind_writes_nothing() {
        let mut e = engine();
        let output = e.mem.alloc(0x20);
        for kind in [0u8, 5, 0x80] {
            let motor = motor_of(&mut e, kind, [1.0; 5], 0);
            let input = input_of(&mut e, [1.0; 6], [1.0; 3]);
            e.call(0x00dc_fff0, &args![motor, input, output]);
        }
        assert_eq!(e.mem.bytes(output, 0x20), vec![0; 0x20]);
    }

    #[test]
    fn copies_pass_through_the_x87_stack() {
        let mut e = engine();
        let motor = motor_of(&mut e, 2, [0.0; 5], 0);
        e.mem.set_u32(motor + 0x10, 0x7f80_0001);
        e.mem.set_u32(motor + 0x18, 0xff80_0002);
        let input = input_of(&mut e, [0.0; 6], [0.0; 3]);
        e.mem.set_u32(input + 0xc, 0x7f80_0003);
        let output = e.mem.alloc(0x20);
        e.call(0x00dc_fff0, &args![motor, input, output]);
        assert_eq!(e.mem.u32(output), 0x7fc0_0003);
        assert_eq!(e.mem.u32(output + 4), 0xffc0_0002);
        assert_eq!(e.mem.u32(output + 8), 0x7fc0_0001);
    }

    /// A body block with the weight vector, position and three rows.
    fn body_of(e: &mut Engine, weight: [f32; 4], position: [f32; 4], rows: [f32; 3]) -> u32 {
        let body = e.mem.alloc(0x80);
        set_floats(e, body + 0x30, &weight);
        set_floats(e, body + 0x40, &position);
        for (index, scale) in rows.into_iter().enumerate() {
            e.mem
                .set_f32(body + 0x50 + 0x10 * index as u32 + 4 * index as u32, scale);
        }
        body
    }

    #[test]
    fn begin_jacobian_builds_the_rows_and_the_inverse_effective_mass() {
        let mut e = engine();
        let body_a = body_of(&mut e, [1.0, 2.0, 3.0, 10.0], [0.0; 4], [1.0, 2.0, 3.0]);
        let body_b = body_of(
            &mut e,
            [4.0, 5.0, 6.0, 20.0],
            [1.0, -1.0, 0.0, 0.0],
            [4.0, 5.0, 6.0],
        );
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        let direction = e.mem.alloc(0x10);
        set_floats(&mut e, direction, &[0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(0x10);
        set_floats(&mut e, point, &[1.0, 0.0, 0.0, 0.0]);
        let jacobian = e.mem.alloc(0x30);
        let result = e.mem.alloc(8);
        e.call(
            0x00dd_0260,
            &args![direction, point, info, jacobian, result],
        );
        // From A: (1, 0, 0) x (0, 0, 1) = (0, -1, 0), rows (1, 2, 3) give (0, -2, 0); from B:
        // (0, 1, 0) gives (-1, 0, 0) and the rows (4, 5, 6) give (-4, 0, 0). The total is
        // (30 + (0 + 8)) + (0 + 64) = 102 (plus FLT_EPSILON, absorbed by rounding).
        assert_eq!(floats(&e, jacobian, 4), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(floats(&e, jacobian + 0x10, 3), [0.0, -2.0, 0.0]);
        assert_eq!(floats(&e, jacobian + 0x20, 3), [-4.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(jacobian + 0x2c), 102.0);
        let inverse = (1.0f64 / 102.0) as f32;
        assert_eq!(e.mem.f32(jacobian + 0x1c), inverse);
        assert_eq!(e.mem.f32(result), inverse);
    }

    #[test]
    fn commit_jacobian_starts_a_record_at_the_cursor() {
        let mut e = engine();
        let block = e.mem.alloc(0x20);
        set_floats(&mut e, block, &[2.0, 3.0, 5.0, 7.0, 11.0, 13.0]);
        let state = e.mem.alloc(0x10);
        set_floats(&mut e, state, &[0.0, 0.5, 4.0]);
        let begin = e.mem.alloc(0x20);
        let record = e.mem.alloc(0x60);
        e.mem.set_f32(record, 1.0);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, record);
        e.call(0x00dd_03c0, &args![block, state, begin, cursor]);
        assert_eq!(e.mem.f32(begin + 0xc), 8.0);
        assert_eq!(e.mem.u32(record), 0x3f80_000a);
        assert_eq!(e.mem.u32(cursor), record + 0x50);
        assert_eq!(floats(&e, record + 0x30, 5), [2.5, 3.5, 3.0, 11.0, 13.0]);
    }
}
