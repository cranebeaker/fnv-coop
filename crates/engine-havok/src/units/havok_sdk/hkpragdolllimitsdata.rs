//! `Havok SDK/hkpragdolllimitsdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpRagdollLimitsData` (Xbox PDB; vtable `010d0af4`) keeps its atoms from `+0x10` (`0xAC`
//! bytes of them). What the code does with the rest: two rotation matrices (`+0x20` and
//! `+0x50`, four rows of four floats each), the cone-limit byte at `+0x9A`, the angular
//! limit floats from `+0x88`, and the motor floats at `+0xB0`, `+0xB4` and `+0xB8`.
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkphingeconstraintdata::{
    CONSTRAINT_INFO_UTIL, IS_ORTHONORMAL, ORTHONORMAL_TOLERANCE,
};
use crate::units::havok_sdk::hkpsphereshape::store_vec;

/// Vtable of the constraint data.
const VTABLE: u32 = 0x010d_0af4;
/// `-100.0` as a `double`: the float at `+0x9C` must equal it for `isValid`.
const MINUS_HUNDRED: u32 = 0x0102_4108;
/// The `float` pi, widened to a `double`: the float at `+0xA0` must be below it.
const PI_AS_DOUBLE: u32 = 0x0101_ff40;
/// The floats the constructor loads (all `float`).
const LIMIT_LOW: u32 = 0x010c_b764;
const LIMIT_HIGH: u32 = 0x010c_b760;
const DAMPING: u32 = 0x0103_0ff0;
const MINUS_HUNDRED_FLOAT: u32 = 0x0102_4100;
const MOTOR_LOW: u32 = 0x010c_b75c;
const MOTOR_HIGH: u32 = 0x0103_13d4;

// Translated from 00d4f130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollLimitsData::getConstraintInfo` (Xbox PDB): `getConstraintInfoUtil` over the atoms
/// at `+0x10`, `0xAC` bytes of them.
pub fn hkp_ragdoll_limits_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    e.call(
        CONSTRAINT_INFO_UTIL,
        &args![this.addr() + 0x10, 0xacu32, info],
    );
}

// Translated from 00d4f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollLimitsData::getRuntimeInfo` (Xbox PDB): the first word is not read; `info` gets
/// `{0x1C, 3}` (`+0` the size in bytes, `+4` the number of solver results).
pub fn hkp_ragdoll_limits_data_get_runtime_info(
    e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    info: Ptr,
) {
    e.mem.set_u32(info.addr() + 4, 3);
    e.mem.set_u32(info.addr(), 0x1c);
}

// Translated from 00d4f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollLimitsData::isValid` (Xbox PDB): the byte at `result` is 1 while every check
/// holds and 0 from the first one that fails. The checks, in order: `isOrthonormal` (tolerance
/// from `01017d00`) of the rotations at `+0x20` and at `+0x50`; the float at `+0x9C` equals
/// -100.0 (a `double`); the float at `+0xA0` is not negative and below pi; the float at `+0xB0`
/// is not above the one at `+0xB4`; the float at `+0x88` is not above the one at `+0x8C`.
/// Every float compare is false for NaN. Answers `result` (`EAX`).
///
/// `isOrthonormal` is handed the byte of the caller's argument slot as its flag: here a scratch
/// word.
pub fn hkp_ragdoll_limits_data_is_valid(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let me = this.addr();
    let tolerance = x87_quiet_bits(e.global::<u32>(ORTHONORMAL_TOLERANCE));
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(result.addr(), 1);
        let first = e
            .call(IS_ORTHONORMAL, &args![me + 0x20, flag, tolerance])
            .u32();
        let mut valid = e.mem.u8(first) != 0;
        e.mem.set_u8(result.addr(), u8::from(valid));
        if valid {
            let second = e
                .call(IS_ORTHONORMAL, &args![me + 0x50, flag, tolerance])
                .u32();
            valid = e.mem.u8(second) != 0;
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        // `FLD float; FCOMP double; TEST AH,0x44; JP`: true only when equal.
        if valid {
            let value = f64::from(e.mem.f32(me + 0x9c));
            valid = value == e.global::<f64>(MINUS_HUNDRED);
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        // `FLDZ; FCOMP float; TEST AH,0x41; JP`: true when zero is not above the value.
        if valid {
            valid = 0.0 <= f64::from(e.mem.f32(me + 0xa0));
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        // `FCOMP double; TEST AH,5; JP`: true when the value is below the constant.
        if valid {
            valid = f64::from(e.mem.f32(me + 0xa0)) < e.global::<f64>(PI_AS_DOUBLE);
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        // `FCOMPP; TEST AH,1; JNZ`: false when [+0xB4] is below [+0xB0] or unordered.
        if valid {
            valid = e.mem.f32(me + 0xb0) <= e.mem.f32(me + 0xb4);
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        if valid {
            valid = e.mem.f32(me + 0x88) <= e.mem.f32(me + 0x8c);
        }
        e.mem.set_u8(result.addr(), u8::from(valid));
        result
    })
}

// Translated from 00d4f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x9A` to 0x10 when `flag` (a byte) is non-zero, else to 0. Unnamed in the
/// map; the setter's meaning is unconfirmed.
pub fn fn_00d4f270(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr() + 0x9a, u8::from(flag != 0) << 4);
}

// Translated from 00d4f290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollLimitsData::getType` (Xbox PDB): 0x13.
pub fn hkp_ragdoll_limits_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x13
}

/// A `float` the constructor loads and stores through the x87 stack (a signalling NaN is
/// quieted).
fn load_float_bits(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(address))
}

// Translated from 00d4f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRagdollLimitsData::hkpRagdollLimitsData` (Xbox PDB): the vtable, the reference count
/// (`+6` = 1), the atom headers (types at `+0x10`, `+0x80`, `+0x94` and `+0xA8`), the two
/// identity rotations (`+0x20`, `+0x50`), the default angular limits and the motor defaults
/// (read from the exe's float constants). Answers `this` (`ECX`).
pub fn hkp_ragdoll_limits_data_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u16(me + 0x10, 4);
    e.mem.set_u8(me + 0x82, 1);
    e.mem.set_u16(me + 0x80, 0xf);
    e.mem.set_u8(me + 0x96, 1);
    e.mem.set_u8(me + 0x9a, 0);
    e.mem.set_u16(me + 0x94, 0x10);
    e.mem.set_u16(me + 0xa8, 0x10);
    e.mem.set_u8(me + 0xaa, 1);
    e.mem.set_u8(me + 0xae, 0);
    for base in [0x20, 0x50] {
        for row in [0, 0x10, 0x20] {
            store_vec(e, me + base + row, [0.0; 4]);
        }
        for diagonal in [0, 0x14, 0x28] {
            e.mem.set_f32(me + base + diagonal, 1.0);
        }
    }
    e.mem.set_u8(me + 0x83, 0);
    let low = load_float_bits(e, LIMIT_LOW);
    e.mem.set_u8(me + 0x84, 1);
    e.mem.set_u32(me + 0x88, low);
    e.mem.set_u8(me + 0x97, 0);
    let high = load_float_bits(e, LIMIT_HIGH);
    e.mem.set_u8(me + 0x98, 0);
    e.mem.set_u32(me + 0x8c, high);
    e.mem.set_u8(me + 0x99, 0);
    let damping = load_float_bits(e, DAMPING);
    e.mem.set_u32(me + 0x90, damping);
    let minus_hundred = load_float_bits(e, MINUS_HUNDRED_FLOAT);
    e.mem.set_u32(me + 0x9c, minus_hundred);
    e.mem.set_f32(me + 0xa0, 1.0);
    e.mem.set_u32(me + 0xa4, damping);
    fn_00d4f270(e, this, 1);
    let motor_low = load_float_bits(e, MOTOR_LOW);
    e.mem.set_u32(me + 0xb0, motor_low);
    e.mem.set_u8(me + 0xac, 1);
    let motor_high = load_float_bits(e, MOTOR_HIGH);
    e.mem.set_u8(me + 0xad, 1);
    e.mem.set_u32(me + 0xb4, motor_high);
    e.mem.set_u8(me + 0xab, 0);
    e.mem.set_u32(me + 0xb8, damping);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d4_f130,
            hkp_ragdoll_limits_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00d4_f150,
            hkp_ragdoll_limits_data_get_runtime_info(Ptr, u32, Ptr)
        ),
        entry!(0x00d4_f170, hkp_ragdoll_limits_data_is_valid(Ptr, Ptr) -> Ptr),
        entry!(0x00d4_f270, fn_00d4f270(Ptr, u8)),
        entry!(0x00d4_f290, hkp_ragdoll_limits_data_get_type(Ptr) -> u32),
        entry!(0x00d4_f2a0, hkp_ragdoll_limits_data_construct(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use std::cell::RefCell;
    use std::rc::Rc;

    const TOLERANCE_BITS: u32 = 0x3a83_126f;

    #[test]
    fn constraint_info_goes_through_the_util_over_the_atoms_00d4f130() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, CONSTRAINT_INFO_UTIL, 0);
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(0x20);
        e.call(0x00d4_f130, &args![this, info]);
        assert_eq!(
            calls(&log, CONSTRAINT_INFO_UTIL),
            vec![vec![this + 0x10, 0xac, info]]
        );
    }

    #[test]
    fn runtime_info_is_size_0x1c_and_three_results_00d4f150() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let info = e.mem.alloc(8);
        e.mem.write(info, &[0xff; 8]);
        e.call(0x00d4_f150, &args![this, 7u32, info]);
        assert_eq!((e.mem.u32(info), e.mem.u32(info + 4)), (0x1c, 3));
    }

    /// Valid data for `isValid`: -100.0 at `+0x9C`, 0.5 at `+0xA0`, ordered limits.
    fn valid_limits(e: &mut Engine) -> u32 {
        e.set_global(MINUS_HUNDRED, -100.0f64);
        e.set_global(PI_AS_DOUBLE, f64::from(std::f32::consts::PI));
        e.set_global(ORTHONORMAL_TOLERANCE, TOLERANCE_BITS);
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0x9c, -100.0);
        e.mem.set_f32(this + 0xa0, 0.5);
        e.mem.set_f32(this + 0xb0, -0.1);
        e.mem.set_f32(this + 0xb4, 0.1);
        e.mem.set_f32(this + 0x88, -1.0);
        e.mem.set_f32(this + 0x8c, 1.0);
        this
    }

    /// Registers an `isOrthonormal` double that answers `answers` in turn and logs its arguments.
    fn orthonormal_double(e: &mut Engine, answers: Vec<u8>) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        let mut answers = answers.into_iter();
        e.register_double(IS_ORTHONORMAL, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u8(a[1], answers.next().unwrap());
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        seen
    }

    #[test]
    fn valid_when_every_check_holds_00d4f170() {
        let mut e = engine();
        let this = valid_limits(&mut e);
        let seen = orthonormal_double(&mut e, vec![1, 1]);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0x55);
        assert_eq!(e.call(0x00d4_f170, &args![this, result]).u32(), result);
        assert_eq!(e.mem.u8(result), 1);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!((seen[0][0], seen[0][2]), (this + 0x20, TOLERANCE_BITS));
        assert_eq!((seen[1][0], seen[1][2]), (this + 0x50, TOLERANCE_BITS));
    }

    #[test]
    fn each_failing_check_makes_it_invalid_00d4f170() {
        // (what to break, the orthonormal answers, how many times it is asked)
        let cases: [(&str, Vec<u8>, usize); 10] = [
            ("first rotation", vec![0], 1),
            ("second rotation", vec![1, 0], 2),
            ("minus hundred", vec![1, 1], 2),
            ("minus hundred nan", vec![1, 1], 2),
            ("negative angle", vec![1, 1], 2),
            ("angle nan", vec![1, 1], 2),
            ("angle at pi", vec![1, 1], 2),
            ("motor limits", vec![1, 1], 2),
            ("motor limits nan", vec![1, 1], 2),
            ("angular limits", vec![1, 1], 2),
        ];
        for (broken, answers, asked) in cases {
            let mut e = engine();
            let this = valid_limits(&mut e);
            match broken {
                "minus hundred" => e.mem.set_f32(this + 0x9c, -99.0),
                "minus hundred nan" => e.mem.set_f32(this + 0x9c, f32::NAN),
                "negative angle" => e.mem.set_f32(this + 0xa0, -0.5),
                "angle nan" => e.mem.set_f32(this + 0xa0, f32::NAN),
                "angle at pi" => e.mem.set_f32(this + 0xa0, std::f32::consts::PI),
                "motor limits" => e.mem.set_f32(this + 0xb0, 0.2),
                "motor limits nan" => e.mem.set_f32(this + 0xb4, f32::NAN),
                "angular limits" => e.mem.set_f32(this + 0x88, 2.0),
                _ => {}
            }
            let seen = orthonormal_double(&mut e, answers);
            let result = e.mem.alloc(4);
            assert_eq!(e.call(0x00d4_f170, &args![this, result]).u32(), result);
            assert_eq!(e.mem.u8(result), 0, "{broken}");
            assert_eq!(seen.borrow().len(), asked, "{broken}");
        }
    }

    #[test]
    fn zero_angle_and_equal_limits_are_still_valid_00d4f170() {
        let mut e = engine();
        let this = valid_limits(&mut e);
        e.mem.set_f32(this + 0xa0, 0.0);
        e.mem.set_f32(this + 0xb0, 0.1);
        e.mem.set_f32(this + 0x88, 1.0);
        orthonormal_double(&mut e, vec![1, 1]);
        let result = e.mem.alloc(4);
        e.call(0x00d4_f170, &args![this, result]);
        assert_eq!(e.mem.u8(result), 1);
    }

    #[test]
    fn the_flag_setter_writes_zero_or_0x10_00d4f270() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        e.call(0x00d4_f270, &args![this, 0xffff_ff01u32]);
        assert_eq!(e.mem.u8(this + 0x9a), 0x10);
        e.call(0x00d4_f270, &args![this, 0xffff_ff00u32]);
        assert_eq!(e.mem.u8(this + 0x9a), 0);
        e.call(0x00d4_f270, &args![this, 0x80u32]);
        assert_eq!(e.mem.u8(this + 0x9a), 0x10);
    }

    #[test]
    fn the_type_is_0x13_00d4f290() {
        let mut e = engine();
        assert_eq!(e.call(0x00d4_f290, &args![0u32]).u32(), 0x13);
    }

    #[test]
    fn construct_sets_the_defaults_00d4f2a0() {
        let mut e = engine();
        e.set_global(LIMIT_LOW, -0.5f32);
        e.set_global(LIMIT_HIGH, 0.5f32);
        e.set_global(DAMPING, 0.8f32);
        e.set_global(MINUS_HUNDRED_FLOAT, -100.0f32);
        e.set_global(MOTOR_LOW, -0.025f32);
        e.set_global(MOTOR_HIGH, 0x7f80_0001u32);
        let this = e.mem.alloc(0xc0);
        e.mem.write(this, &[0xcd; 0xc0]);
        assert_eq!(e.call(0x00d4_f2a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 0x10), 4);
        assert_eq!(e.mem.u8(this + 0x82), 1);
        assert_eq!(e.mem.u16(this + 0x80), 0xf);
        assert_eq!(e.mem.u8(this + 0x96), 1);
        assert_eq!(e.mem.u8(this + 0x9a), 0x10);
        assert_eq!(e.mem.u16(this + 0x94), 0x10);
        assert_eq!(e.mem.u16(this + 0xa8), 0x10);
        assert_eq!(e.mem.u8(this + 0xaa), 1);
        assert_eq!(e.mem.u8(this + 0xae), 0);
        for base in [0x20, 0x50] {
            for row in 0..3 {
                for lane in 0..4 {
                    let expected = if row == lane { 1.0 } else { 0.0 };
                    let at = this + base + 0x10 * row + 4 * lane;
                    assert_eq!(e.mem.f32(at), expected);
                }
            }
        }
        assert_eq!(e.mem.u8(this + 0x83), 0);
        assert_eq!(e.mem.u8(this + 0x84), 1);
        assert_eq!(e.mem.f32(this + 0x88), -0.5);
        assert_eq!(e.mem.u8(this + 0x97), 0);
        assert_eq!(e.mem.u8(this + 0x98), 0);
        assert_eq!(e.mem.u8(this + 0x99), 0);
        assert_eq!(e.mem.f32(this + 0x8c), 0.5);
        assert_eq!(e.mem.f32(this + 0x90), 0.8);
        assert_eq!(e.mem.f32(this + 0x9c), -100.0);
        assert_eq!(e.mem.f32(this + 0xa0), 1.0);
        assert_eq!(e.mem.f32(this + 0xa4), 0.8);
        assert_eq!(e.mem.f32(this + 0xb0), -0.025);
        assert_eq!(e.mem.u8(this + 0xac), 1);
        assert_eq!(e.mem.u8(this + 0xad), 1);
        assert_eq!(e.mem.u32(this + 0xb4), 0x7fc0_0001);
        assert_eq!(e.mem.u8(this + 0xab), 0);
        assert_eq!(e.mem.f32(this + 0xb8), 0.8);
        // Bytes the constructor never writes keep their value.
        assert_eq!(e.mem.u16(this + 0x12), 0xcdcd);
        assert_eq!(e.mem.u8(this + 0xaf), 0xcd);
    }
}
