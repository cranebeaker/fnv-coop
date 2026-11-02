//! `Havok SDK/run_00dddc40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    compare_flags, test_parity_even, x87_to_float,
};

/// The routine `00ddd7e0` (cdecl: the triangle points and two pointers to `float`s; answers a
/// `float` in `ST0`).
const ANGLE: u32 = 0x00dd_d7e0;
/// The two `double`s the angle table is built from: the step (`010cd5a8`) and the offset
/// (`0101ff40`).
const STEP: u32 = 0x010c_d5a8;
const OFFSET: u32 = 0x0101_ff40;

// Translated from 00dddc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `points`). Asks `00ddd7e0` for the angle `a` (stored
/// through a `float` slot, so rounded to a `float` and a signalling NaN quieted), then finds the
/// first `i` from 0 whose table value `v = float(i * step - offset)` is not below `a` (a NaN
/// compares as below, so it never stops early) and answers:
///
/// * 31 when `i` reaches 31 without that;
/// * `i - 1` when `0.0 < a` and `v != a` (x87 compares; `a` is positive here so not a NaN);
/// * `i` otherwise.
pub fn fn_00dddc40(e: &mut Engine, points: u32) -> u32 {
    // The exe's frame (`SUB ESP,0x8`): the two out words at +0 and +0x4 (00dddc47).
    let angle = e.with_stack(8, |e, scratch| {
        let scratch = scratch.addr();
        e.call(ANGLE, &args![points, scratch, scratch + 4]).f64()
    });
    let angle = x87_load32(x87_to_float(angle).to_bits());
    let step = e.mem.f64(STEP);
    let offset = e.mem.f64(OFFSET);
    let mut index = 0u32;
    let value = loop {
        let product = x87_mul(f64::from(index), step);
        let value = x87_load32(x87_store32(x87_sub(product, offset)));
        // `FCOM` then `TEST AH, 1`: the loop goes on while the value is below the angle or
        // unordered with it.
        if compare_flags(value, angle) & 1 == 0 {
            break value;
        }
        index += 1;
        if index >= 0x1f {
            return index;
        }
    };
    // `FLDZ`, `FCOMP`, `TEST AH, 5`, `JP`: only a positive angle goes on to the next check.
    if test_parity_even(compare_flags(0.0, angle), 5) {
        return index;
    }
    // `FUCOMPP`, `TEST AH, 0x44`, `JNP`: equal values answer the index itself.
    if !test_parity_even(compare_flags(value, angle), 0x44) {
        return index;
    }
    index - 1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00dddc40, fn_00dddc40(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const STEP_VALUE: f64 = 0.209_439_510_239_319_53;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STEP, 8);
        e.map(OFFSET, 8);
        e.mem.set_f64(STEP, STEP_VALUE);
        e.mem.set_f64(OFFSET, std::f64::consts::PI);
        e
    }

    /// The table value for index `i`, as the code forms it.
    fn table(i: u32) -> f64 {
        f64::from((f64::from(i) * STEP_VALUE - std::f64::consts::PI) as f32)
    }

    fn run(angle: f64) -> u32 {
        let mut e = engine();
        e.register_double(ANGLE, move |_, _| Ret {
            st0: angle,
            ..Ret::default()
        });
        e.call(0x00dd_dc40, &args![0x1000u32]).u32()
    }

    #[test]
    fn the_index_is_the_last_table_value_below_a_positive_angle() {
        // table(18) = 0.628 is the first value not below 0.5.
        assert_eq!(run(0.5), 17);
        assert_eq!(run(0.01), 15);
    }

    #[test]
    fn an_angle_equal_to_a_table_value_answers_its_index() {
        assert_eq!(run(table(18)), 18);
        assert_eq!(run(table(30)), 30);
    }

    #[test]
    fn a_zero_or_negative_angle_answers_the_first_value_not_below_it() {
        // table(11) = -0.838 is the first value not below -1.0.
        assert_eq!(run(-1.0), 11);
        assert_eq!(run(-10.0), 0);
        assert_eq!(run(table(5)), 5);
    }

    #[test]
    fn an_angle_above_the_table_or_a_nan_answers_31() {
        assert_eq!(run(4.0), 31);
        assert_eq!(run(f64::NAN), 31);
        assert_eq!(run(f64::INFINITY), 31);
        assert_eq!(run(f64::NEG_INFINITY), 0);
    }

    #[test]
    fn the_angle_routine_gets_the_points_and_two_adjacent_float_slots() {
        let mut e = engine();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(ANGLE, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        // The two out words are the exe's whole 8-byte frame.
        let frame = e.next_stack_local(8);
        e.call(0x00dd_dc40, &args![0x1234u32]);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][0], 0x1234);
        assert_eq!(log[0][2], log[0][1] + 4);
        assert_eq!(log[0][1], frame);
    }
}
