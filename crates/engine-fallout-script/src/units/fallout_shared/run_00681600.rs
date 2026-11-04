//! `fallout shared/run_00681600` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The evaluation of one condition item (`TESConditionItem`, laid out in
//! `tesconditionitem.rs`) through the script function table's condition
//! functions.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesconditionitem::TesConditionItem;

/// `00825c00` (`ECX` = item): the word at +0x14 (`eObject`).
const RUN_ON: u32 = 0x0082_5c00;
/// `00569b80` (`ECX` = reference): the reference the case-4 item runs on
/// (the item data names it the linked reference; unconfirmed).
const LINKED_REFERENCE: u32 = 0x0056_9b80;
/// `00413f40` (`ECX` = item): the address of +8 (the function data).
const FUNCTION_DATA: u32 = 0x0041_3f40;
/// `00682320` (cdecl: function data, parameter index): whether that
/// parameter of the function takes the target in place of a missing form
/// (its parameter type is 4 or 6).
const PARAMETER_TAKES_TARGET: u32 = 0x0068_2320;
/// The script function table (`01190910`, `SCRIPT_FUNCTION` (Xbox PDB),
/// 0x28 bytes, indexed by the function number at +8).
const FUNCTION_TABLE: u32 = 0x0119_0910;
const FUNCTION_ENTRY_SIZE: u32 = 0x28;
/// `SCRIPT_FUNCTION::bReferenceFunction` (Xbox PDB, +0x10).
const ENTRY_REFERENCE_FUNCTION: u32 = 0x10;
/// `SCRIPT_FUNCTION::pConditionFunction` (Xbox PDB, +0x20; cdecl: subject,
/// first parameter, second parameter, the result double's address).
const ENTRY_CONDITION_FUNCTION: u32 = 0x20;
/// `00526ac0` (`ECX` = global): the global's value, in `ST0`.
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `006814e0` (`ECX` = item): whether the comparison value is a global.
const VALUE_IS_GLOBAL: u32 = 0x0068_14e0;
/// `005ec060` (`ECX` = item): the comparison operator (flag bits 5-7).
const OPERATOR: u32 = 0x005e_c060;
/// `00681050` (cdecl: operator, value, comparison value; floats as their
/// bits): whether the comparison holds, in `AL`.
const COMPARE: u32 = 0x0068_1050;
/// `005eb680` (`ECX` = item): the function number at +8, zero-extended.
const FUNCTION_NUMBER: u32 = 0x005e_b680;
/// The opcode (number + 0x1000) whose failure with operator 2 or 3 sets
/// the out byte.
const OPCODE_0X104C: u32 = 0x104c;
/// The opcode whose failure always sets the out byte.
const OPCODE_0X1193: u32 = 0x1193;

/// The bits an x87 `FLD qword` / `FSTP dword` pair leaves for `bits`: the
/// value rounded to the nearest float; a NaN keeps its sign and the top of
/// its payload and comes back quiet.
fn x87_double_to_float_bits(bits: u64) -> u32 {
    let value = f64::from_bits(bits);
    if value.is_nan() {
        let sign = ((bits >> 63) as u32) << 31;
        let payload = ((bits >> 29) as u32) & 0x007f_ffff;
        sign | 0x7fc0_0000 | payload
    } else {
        (value as f32).to_bits()
    }
}

// Translated from 00681600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Evaluates the condition item (`ECX`; three words: the subject
/// reference, the target, the address of an out byte; `RET 0xc`). The out
/// byte is cleared first. The reference the function runs on and the
/// target, by the item's run-on word (`00825c00`):
/// - 1: swapped (the target runs it, the subject is the target);
/// - 2: the item's reference (+0x18), the target unchanged;
/// - 3: none, the target the subject; with a subject whose slot 0x100 says
///   so, the result of its slot 0x42c;
/// - 4: none, no target; with a subject, `00569b80` of it;
/// - otherwise the subject and the target.
///
/// The function's first parameter (function data +4, `00413f40`), when 0
/// and `00682320` says parameter 0 takes it, becomes the target. Then, if
/// the function's table entry (`01190910` + 0x28 × number) has a condition
/// function, and either it is not a reference function or there is a
/// reference, the function is called (reference, first parameter, second
/// parameter (+8 of the data), the address of a double starting at 0.0);
/// if it returns nonzero, the double, as a float, is compared (`00681050`
/// with the operator from `005ec060`) with the comparison value: the
/// global's (`00526ac0`) when the item has a global (+4 nonzero and
/// `006814e0`), else the float at +4. The comparison's byte is returned.
/// When it fails, function number 0x4c with operator 2 or 3, or 0x193,
/// sets the out byte. Returns 0 when the function is not called or
/// returns 0. The engine map gives it no name.
pub fn fn_00681600(
    e: &mut Engine,
    this: Ptr<TesConditionItem>,
    subject: u32,
    target: u32,
    out: Ptr,
) -> u8 {
    let mut result = 0u8;
    let mut runs_on = subject;
    let mut target = target;
    e.mem.set_u8(out.addr(), 0);
    match e.call(RUN_ON, &args![this]).u32() {
        1 => {
            runs_on = target;
            target = subject;
        }
        2 => runs_on = e.get(this, TesConditionItem::pRunOnRef),
        3 => {
            runs_on = 0;
            target = subject;
            if subject != 0 && e.vcall(subject, 0x100, &args![]).u8() != 0 {
                runs_on = e.vcall(subject, 0x42c, &args![]).u32();
            }
        }
        4 => {
            runs_on = 0;
            target = 0;
            if subject != 0 {
                runs_on = e.call(LINKED_REFERENCE, &args![subject]).u32();
            }
        }
        _ => {}
    }
    let data = e.call(FUNCTION_DATA, &args![this]).u32();
    let mut first = e.mem.u32(data + 4);
    if first == 0 && e.call(PARAMETER_TAKES_TARGET, &args![data, 0u32]).u8() != 0 {
        first = target;
    }
    let entry = FUNCTION_TABLE + u32::from(e.mem.u16(data)) * FUNCTION_ENTRY_SIZE;
    let condition = e.mem.u32(entry + ENTRY_CONDITION_FUNCTION);
    if condition == 0 {
        return result;
    }
    if e.mem.u8(entry + ENTRY_REFERENCE_FUNCTION) != 0 && runs_on == 0 {
        return result;
    }
    let second = e.mem.u32(data + 8);
    let (called, value) = e.with_stack(8, |e, double| {
        e.mem.set_u64(double.addr(), 0);
        let called = e
            .call(condition, &args![runs_on, first, second, double])
            .u8();
        (called, e.mem.u64(double.addr()))
    });
    if called == 0 {
        return result;
    }
    let global = e.get(this, TesConditionItem::pGlobal);
    let comparison = if global != 0 && e.call(VALUE_IS_GLOBAL, &args![this]).u8() != 0 {
        let st0 = e.call(GLOBAL_VALUE, &args![global]).f64();
        x87_double_to_float_bits(st0.to_bits())
    } else {
        x87_quiet_bits(global)
    };
    let value = x87_double_to_float_bits(value);
    let operator = e.call(OPERATOR, &args![this]).u32();
    result = e.call(COMPARE, &args![operator, value, comparison]).u8();
    if result == 0 {
        let opcode = e
            .call(FUNCTION_NUMBER, &args![this])
            .u32()
            .wrapping_add(0x1000);
        if opcode == OPCODE_0X104C {
            if e.call(OPERATOR, &args![this]).u32() == 2
                || e.call(OPERATOR, &args![this]).u32() == 3
            {
                e.mem.set_u8(out.addr(), 1);
            }
        } else if opcode == OPCODE_0X1193 {
            e.mem.set_u8(out.addr(), 1);
        }
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00681600,
        fn_00681600(Ptr<TesConditionItem>, u32, u32, Ptr) -> u8
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONDITION: u32 = 0x00ab_c000;
    const NUMBER: u32 = 0x4c;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The condition function double writes the double at 0x0b00_0010 and
    /// returns the word at 0x0b00_0018; the comparison returns the word at
    /// 0x0b00_001c; the operator is the word at 0x0b00_0020.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.map(0x0b00_0000, 0x40);
        e.map(FUNCTION_TABLE, 0x280 * FUNCTION_ENTRY_SIZE);
        e.register(RUN_ON, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(FUNCTION_DATA, |_, a| ret(a[0] + 8));
        e.register(PARAMETER_TAKES_TARGET, |e, _| ret(e.mem.u32(0x0b00_0000)));
        e.register(LINKED_REFERENCE, |_, a| ret(a[0] + 0x40));
        e.register(CONDITION, |e, a| {
            let v = e.mem.u64(0x0b00_0010);
            e.mem.set_u64(a[3], v);
            ret(e.mem.u32(0x0b00_0018))
        });
        e.register(VALUE_IS_GLOBAL, |e, a| {
            ret(u32::from(e.mem.u8(a[0]) & 4 != 0))
        });
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: 0.1,
            ..Ret::default()
        });
        e.register(OPERATOR, |e, _| ret(e.mem.u32(0x0b00_0020)));
        e.register(COMPARE, |e, _| ret(e.mem.u32(0x0b00_001c)));
        e.register(FUNCTION_NUMBER, |e, a| ret(u32::from(e.mem.u16(a[0] + 8))));
        e.call_log = Some(vec![]);
        e
    }

    /// An item for function `number` with a condition function, run-on
    /// word `run_on`, comparison value 2.0.
    fn item(e: &mut Engine, number: u32, run_on: u32) -> u32 {
        let p = e.mem.alloc(0x1c);
        e.mem.set_u8(p, 0);
        e.mem.set_f32(p + 4, 2.0);
        e.mem.set_u32(p + 8, number);
        e.mem.set_u32(p + 0xc, 0x0c0c);
        e.mem.set_u32(p + 0x10, 0x1010);
        e.mem.set_u32(p + 0x14, run_on);
        e.mem.set_u32(p + 0x18, 0x1818);
        let entry = FUNCTION_TABLE + number * FUNCTION_ENTRY_SIZE;
        e.mem.set_u32(entry + ENTRY_CONDITION_FUNCTION, CONDITION);
        p
    }

    fn evaluate(e: &mut Engine, p: u32, subject: u32, target: u32) -> (u8, u8) {
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xee);
        let r = e.call(0x0068_1600, &args![p, subject, target, out]).u8();
        (r, e.mem.u8(out))
    }

    #[test]
    fn the_run_on_word_picks_reference_and_target() {
        let mut e = setup();
        e.mem.set_u32(0x0b00_0018, 0);
        // Parameter 0 takes the target in place of the missing form.
        e.mem.set_u32(0x0b00_0000, 1);
        let subject = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x430);
        e.mem.set_u32(subject, vtable);
        e.mem.set_u32(vtable + 0x100, 0x00ab_c100);
        e.mem.set_u32(vtable + 0x42c, 0x00ab_c42c);
        e.register(0x00ab_c100, |_, _| ret(1));
        e.register(0x00ab_c42c, |_, _| ret(0x4242));
        let expected = [
            (0, subject, 0x7777),
            (1, 0x7777, subject),
            (2, 0x1818, 0x7777),
            (3, 0x4242, subject),
            (4, subject + 0x40, 0),
            (5, subject, 0x7777),
        ];
        for (run_on, runs_on, target) in expected {
            let p = item(&mut e, 7, run_on);
            e.mem.set_u32(p + 0xc, 0);
            e.call_log = Some(vec![]);
            assert_eq!(evaluate(&mut e, p, subject, 0x7777), (0, 0));
            let call = &calls(&e, CONDITION)[0];
            assert_eq!(call[..3], [runs_on, target, 0x1010], "{run_on}");
        }
        // Case 3 without the slot's yes: no reference.
        e.register(0x00ab_c100, |_, _| ret(0));
        let p = item(&mut e, 7, 3);
        e.call_log = Some(vec![]);
        evaluate(&mut e, p, subject, 0x7777);
        assert_eq!(calls(&e, CONDITION)[0][..2], [0, 0x0c0c]);
        // Case 4 without a subject: no call to 00569b80.
        let p = item(&mut e, 7, 4);
        e.call_log = Some(vec![]);
        evaluate(&mut e, p, 0, 0x7777);
        assert!(calls(&e, LINKED_REFERENCE).is_empty());
        assert_eq!(calls(&e, CONDITION)[0][..2], [0, 0x0c0c]);
    }

    #[test]
    fn no_call_without_a_function_or_a_needed_reference() {
        let mut e = setup();
        let p = item(&mut e, 9, 0);
        let entry = FUNCTION_TABLE + 9 * FUNCTION_ENTRY_SIZE;
        e.mem.set_u32(entry + ENTRY_CONDITION_FUNCTION, 0);
        assert_eq!(evaluate(&mut e, p, 0x10, 0x20), (0, 0));
        e.mem.set_u32(entry + ENTRY_CONDITION_FUNCTION, CONDITION);
        e.mem.set_u8(entry + ENTRY_REFERENCE_FUNCTION, 1);
        assert_eq!(evaluate(&mut e, p, 0, 0x20), (0, 0));
        assert!(calls(&e, CONDITION).is_empty());
        // A reference function with a reference is called; returning 0
        // ends there.
        e.mem.set_u32(0x0b00_0018, 0x100);
        assert_eq!(evaluate(&mut e, p, 0x10, 0x20), (0, 0));
        assert_eq!(calls(&e, CONDITION).len(), 1);
        assert!(calls(&e, COMPARE).is_empty());
    }

    #[test]
    fn the_result_is_compared_with_the_value_or_the_global() {
        let mut e = setup();
        e.mem.set_u32(0x0b00_0018, 1);
        e.mem.set_u32(0x0b00_001c, 0x0101);
        e.mem.set_u32(0x0b00_0020, 4);
        e.mem.set_f64(0x0b00_0010, 0.1);
        let p = item(&mut e, 9, 0);
        assert_eq!(evaluate(&mut e, p, 0x10, 0x20), (1, 0));
        assert_eq!(
            calls(&e, COMPARE),
            vec![vec![4, 0.1f32.to_bits(), 2.0f32.to_bits()]]
        );
        // The global's value, rounded to a float.
        e.mem.set_u8(p, 4);
        e.call_log = Some(vec![]);
        evaluate(&mut e, p, 0x10, 0x20);
        assert_eq!(calls(&e, COMPARE)[0][2], 0.1f32.to_bits());
        // Flag bit 2 with a zero word: the word itself (0.0).
        e.mem.set_u32(p + 4, 0);
        e.call_log = Some(vec![]);
        evaluate(&mut e, p, 0x10, 0x20);
        assert!(calls(&e, GLOBAL_VALUE).is_empty());
        assert_eq!(calls(&e, COMPARE)[0][2], 0);
        // A signalling NaN result and value come back quiet.
        e.mem.set_u8(p, 0);
        e.mem.set_u32(p + 4, 0x7f80_0001);
        e.mem.set_u64(0x0b00_0010, 0xfff0_0000_2000_0000);
        e.call_log = Some(vec![]);
        evaluate(&mut e, p, 0x10, 0x20);
        assert_eq!(calls(&e, COMPARE)[0][1..], [0xffc0_0001, 0x7fc0_0001]);
    }

    #[test]
    fn a_failed_comparison_sets_the_out_byte_for_two_functions() {
        let mut e = setup();
        e.mem.set_u32(0x0b00_0018, 1);
        e.mem.set_u32(0x0b00_001c, 0);
        for (number, operator, out) in [
            (NUMBER, 2, 1),
            (NUMBER, 3, 1),
            (NUMBER, 1, 0),
            (0x193, 0, 1),
            (0x4d, 2, 0),
        ] {
            e.mem.set_u32(0x0b00_0020, operator);
            let p = item(&mut e, number, 0);
            assert_eq!(evaluate(&mut e, p, 0x10, 0x20), (0, out), "{number:#x}");
        }
        // A holding comparison leaves it cleared.
        e.mem.set_u32(0x0b00_001c, 1);
        let p = item(&mut e, 0x193, 0);
        assert_eq!(evaluate(&mut e, p, 0x10, 0x20), (1, 0));
    }

    #[test]
    fn doubles_round_to_floats_as_the_x87_does() {
        assert_eq!(x87_double_to_float_bits(0.1f64.to_bits()), 0.1f32.to_bits());
        assert_eq!(x87_double_to_float_bits(1e300f64.to_bits()), 0x7f80_0000);
        assert_eq!(x87_double_to_float_bits(0x7ff8_0000_0000_0000), 0x7fc0_0000);
    }
}
