//! `fallout shared/foscriptfunctions.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The VATS singleton (`thiscall` target of `0044ddc0` and `009c71c0`).
const VATS: u32 = 0x011f_2250;
/// The player character pointer.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `"GetVATSValue %d, %d -> %0.2f"`.
const DEBUG_FORMAT: u32 = 0x0103_4044;

/// `0044ddc0` (`thiscall`, no stack words): the word at `this` (here the
/// VATS mode).
const WORD_AT: u32 = 0x0044_ddc0;
/// `VATS::GetCurrentAction` (engine map), `thiscall`.
const VATS_CURRENT_ACTION: u32 = 0x009c_71c0;
/// `007f5280` (no parameters): the action data of the VATS menu, or null.
const MENU_ACTION: u32 = 0x007f_5280;
/// `00500940` (`thiscall`): the form list member of the form.
const FORM_LIST: u32 = 0x0050_0940;
/// `005f65d0` (`thiscall`, `RET 4`): whether the list holds the form whose
/// word is at the given address (`AL`).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// `007af430` (`thiscall`): the base form of a reference (the word at
/// +0x20).
const BASE_FORM: u32 = 0x007a_f430;
/// `00446390` (`thiscall`): the word at the start of the object.
const FIRST_WORD: u32 = 0x0044_6390;
/// `CombatUtilities::GetDistanceBetweenFacingActors` (engine map), `cdecl`:
/// (actor, other, 0), in `ST0`.
const DISTANCE_BETWEEN_FACING_ACTORS: u32 = 0x009a_64d0;
/// `0058cba0` (`thiscall`, `RET 4`): whether the object has the flag
/// (`AL`).
const HAS_FLAG: u32 = 0x0058_cba0;
/// `Interface::IsConsoleVisible` (engine map), no parameters.
const IS_CONSOLE_VISIBLE: u32 = 0x0070_3d50;
/// The console print (`00703c00`, `cdecl`, format first).
const CONSOLE_PRINT: u32 = 0x0070_3c00;

/// 1.0 or 0.0 from a byte test, as the code builds it (`FLD1`/`FLDZ`, a
/// `float` local, then `FSTP double`).
fn one_or_zero(value: bool) -> f64 {
    if value {
        1.0
    } else {
        0.0
    }
}

/// `005f65d0` on the form's list (`00500940`) with the address of a local
/// holding `item`.
fn form_list_contains(e: &mut Engine, form: u32, item: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        let list = e.call(FORM_LIST, &args![form]).u32();
        e.call(LIST_CONTAINS, &args![list, slot]).bool()
    })
}

// Translated from 00594e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOScriptFunctions::GetVATSValueConditionFunction` (Xbox PDB name from
/// the engine map; `cdecl`, four words, answer in `AL`; the first word is
/// not read). The result starts at 0. The action is VATS's current action
/// (`009c71c0`) in mode 4, the menu's (`007f5280`) in modes 2 and 3, none
/// otherwise; with an action and `value` from 0 to 0x11 (the code's jump
/// table, translated as a `match`), the result is set for that value and the
/// answer is true:
/// - 0: 1.0 when `form` is the action's target (`action[+0x14]` +0x30);
/// - 1: 1.0 when that target is in `form`'s list;
/// - 2: 1.0 when the action's actor (+0xc) has a base form and `form`'s
///   slot +0x10c answers false for it;
/// - 3: 1.0 when the actor's base form is in `form`'s list;
/// - 4: the distance between the player and the actor (`009a64d0`);
/// - 5, 6: 1.0 when `form` is the word at +0x10, at +0;
/// - 7: the byte at +4 as 1.0 or 0.0;
/// - 8, 11, 12, 13, 14: the target data's flag 4, 0x10, 0x40, 0x20, 0x80
///   (`0058cba0`) as 1.0 or 0.0;
/// - 9: 1.0 when the target data's word at +0x50 is set and `form`'s slot
///   +0x10c answers false for it; 10: 1.0 when it is in `form`'s list;
/// - 15: 1.0 when `form` is the first word (`00446390`) of the target, or
///   when there is no target and no `form`;
/// - 16: the byte at +6 as 1.0 or 0.0, then 1.0 when the byte at +0x24 is
///   set;
/// - 17: the byte at +7 as 1.0 or 0.0.
///
/// With the console open the value is printed (format at `01034044`).
pub fn fo_script_functions_get_vats_value_condition_function(
    e: &mut Engine,
    _unused_0: u32,
    value: i32,
    form: u32,
    result: Ptr,
) -> bool {
    let result = result.addr();
    e.mem.set_f64(result, 0.0);
    let mut answered = false;
    let action = if e.call(WORD_AT, &args![VATS]).u32() == 4 {
        e.call(VATS_CURRENT_ACTION, &args![VATS]).u32()
    } else if e.call(WORD_AT, &args![VATS]).u32() == 2 || e.call(WORD_AT, &args![VATS]).u32() == 3 {
        e.call(MENU_ACTION, &[]).u32()
    } else {
        0
    };
    if action != 0 && (0..0x12).contains(&value) {
        let target_data = |e: &Engine| e.mem.u32(action + 0x14);
        match value {
            0 => {
                let target = e.mem.u32(target_data(e) + 0x30);
                if form != 0 && form == target {
                    e.mem.set_f64(result, 1.0);
                }
            }
            1 => {
                let target = e.mem.u32(target_data(e) + 0x30);
                if form != 0 && target != 0 && form_list_contains(e, form, target) {
                    e.mem.set_f64(result, 1.0);
                }
            }
            2 => {
                if form != 0 && e.mem.u32(action + 0xc) != 0 {
                    let actor = e.mem.u32(action + 0xc);
                    if e.call(BASE_FORM, &args![actor]).u32() != 0 {
                        let actor = e.mem.u32(action + 0xc);
                        let base = e.call(BASE_FORM, &args![actor]).u32();
                        if !e.vcall(form, 0x10c, &args![base]).bool() {
                            e.mem.set_f64(result, 1.0);
                        }
                    }
                }
            }
            3 => {
                if e.mem.u32(action + 0xc) != 0 && form != 0 {
                    let actor = e.mem.u32(action + 0xc);
                    let base = e.call(BASE_FORM, &args![actor]).u32();
                    if form_list_contains(e, form, base) {
                        e.mem.set_f64(result, 1.0);
                    }
                }
            }
            4 => {
                let player = e.global::<u32>(PLAYER_POINTER);
                let actor = e.mem.u32(action + 0xc);
                let distance = e
                    .call(DISTANCE_BETWEEN_FACING_ACTORS, &args![player, actor, 0u32])
                    .f64();
                e.mem.set_f64(result, distance);
            }
            5 => {
                if form == e.mem.u32(action + 0x10) {
                    e.mem.set_f64(result, 1.0);
                }
            }
            6 => {
                if form == e.mem.u32(action) {
                    e.mem.set_f64(result, 1.0);
                }
            }
            7 => {
                let set = e.mem.u8(action + 4) != 0;
                e.mem.set_f64(result, one_or_zero(set));
            }
            9 => {
                if form != 0 && e.mem.u32(target_data(e) + 0x50) != 0 {
                    let item = e.mem.u32(target_data(e) + 0x50);
                    if !e.vcall(form, 0x10c, &args![item]).bool() {
                        e.mem.set_f64(result, 1.0);
                    }
                }
            }
            10 => {
                if e.mem.u32(target_data(e) + 0x50) != 0 && form != 0 {
                    let item = e.mem.u32(target_data(e) + 0x50);
                    if form_list_contains(e, form, item) {
                        e.mem.set_f64(result, 1.0);
                    }
                }
            }
            8 | 11 | 12 | 13 | 14 => {
                let flag: u32 = match value {
                    8 => 4,
                    11 => 0x10,
                    12 => 0x40,
                    13 => 0x20,
                    _ => 0x80,
                };
                let data = target_data(e);
                let set = e.call(HAS_FLAG, &args![data, flag]).bool();
                e.mem.set_f64(result, one_or_zero(set));
            }
            15 => {
                if e.mem.u32(target_data(e) + 0x30) != 0 {
                    let target = e.mem.u32(target_data(e) + 0x30);
                    if e.call(FIRST_WORD, &args![target]).u32() == form {
                        e.mem.set_f64(result, 1.0);
                    }
                } else if form == 0 {
                    e.mem.set_f64(result, 1.0);
                }
            }
            16 => {
                let set = e.mem.u8(action + 6) != 0;
                e.mem.set_f64(result, one_or_zero(set));
                let combined = if e.mem.u8(action + 0x24) != 0 {
                    1.0
                } else {
                    x87_load64(e.mem.u64(result))
                };
                e.mem
                    .set_u64(result, x87_load64(combined.to_bits()).to_bits());
            }
            _ => {
                let set = e.mem.u8(action + 7) != 0;
                e.mem.set_f64(result, one_or_zero(set));
            }
        }
        answered = true;
    }
    if e.call(IS_CONSOLE_VISIBLE, &[]).bool() {
        let shown = x87_load64(e.mem.u64(result));
        e.call(CONSOLE_PRINT, &args![DEBUG_FORMAT, value, form, shown]);
    }
    answered
}

// Translated from 00594dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FOScriptFunctions::GetVATSValueFunction` (Xbox PDB name in the engine
/// map; the script command's eight words): parses two word arguments
/// (`Script::ParseParameters`, both starting at 0) and answers
/// `GetVATSValueConditionFunction(thisObj, first, second, result)`
/// (`00594e40`); false when they do not parse.
pub fn fo_script_functions_get_vats_value_function(
    e: &mut Engine,
    a: crate::units::fallout_shared::tesscriptfunctions::ScriptArgs,
) -> bool {
    let Some([value, form]) = a.parse_into(e, [0, 0]) else {
        return false;
    };
    fo_script_functions_get_vats_value_condition_function(
        e,
        a.this_obj.addr(),
        value as i32,
        form,
        a.result,
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0059_4dc0,
            fo_script_functions_get_vats_value_function(
                crate::units::fallout_shared::tesscriptfunctions::ScriptArgs
            ) -> bool
        ),
        entry!(
            0x0059_4e40,
            fo_script_functions_get_vats_value_condition_function(u32, i32, u32, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// VATS in `mode` with an action whose target data is set up; the
    /// console is closed unless `console`.
    fn engine(mode: u32, console: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let action = e.mem.alloc(0x30);
        let data = e.mem.alloc(0x60);
        e.mem.set_u32(action + 0x14, data);
        e.register_double(WORD_AT, move |_, _| word(mode));
        e.register_double(VATS_CURRENT_ACTION, move |_, _| word(action));
        e.register_double(MENU_ACTION, move |_, _| word(action));
        e.register_double(IS_CONSOLE_VISIBLE, move |_, _| word(u32::from(console)));
        e.register(CONSOLE_PRINT, |_, _| Ret::default());
        e.register(FORM_LIST, |_, a| word(a[0] + 0x1000));
        e.register(LIST_CONTAINS, |e, a| {
            word(u32::from(e.mem.u32(a[1]) == 0x55))
        });
        e.register(BASE_FORM, |e, a| word(e.mem.u32(a[0])));
        e.register(HAS_FLAG, |_, a| word(u32::from(a[1] == 0x40)));
        (e, action, data)
    }

    fn run(e: &mut Engine, value: i32, form: u32) -> (bool, f64) {
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 9.0);
        let answered = e
            .call(0x0059_4e40, &args![0u32, value, form, result])
            .bool();
        (answered, e.mem.f64(result))
    }

    #[test]
    fn without_an_action_the_answer_is_false_and_the_result_zero() {
        let (mut e, _, _) = engine(1, false);
        assert_eq!(run(&mut e, 5, 0), (false, 0.0));
        let (mut e, _, _) = engine(4, false);
        assert_eq!(run(&mut e, 0x12, 0), (false, 0.0));
        assert_eq!(run(&mut e, -1, 0), (false, 0.0));
    }

    #[test]
    fn target_values() {
        let (mut e, _, data) = engine(4, false);
        e.mem.set_u32(data + 0x30, 0x77);
        assert_eq!(run(&mut e, 0, 0x77), (true, 1.0));
        assert_eq!(run(&mut e, 0, 0x78), (true, 0.0));
        e.mem.set_u32(data + 0x30, 0x55);
        assert_eq!(run(&mut e, 1, 0x9000), (true, 1.0));
        assert_eq!(run(&mut e, 1, 0), (true, 0.0));
        e.register(FIRST_WORD, |_, _| word(0x9000));
        assert_eq!(run(&mut e, 15, 0x9000), (true, 1.0));
        e.mem.set_u32(data + 0x30, 0);
        assert_eq!(run(&mut e, 15, 0), (true, 1.0));
        assert_eq!(run(&mut e, 15, 1), (true, 0.0));
    }

    #[test]
    fn actor_values_and_the_vtable_slot() {
        let (mut e, action, data) = engine(2, false);
        let actor = e.mem.alloc(4);
        e.mem.set_u32(actor, 0x55);
        e.mem.set_u32(action + 0xc, actor);
        let form = e.mem.alloc(4);
        e.put_vtable(0x0300_0000, &[0; 0x44]);
        e.mem.set_u32(0x0300_0000 + 0x10c, 0x0300_1000);
        e.mem.set_u32(form, 0x0300_0000);
        e.register(0x0300_1000, |_, a| word(u32::from(a[1] == 0x66)));
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 2, form), (true, 1.0));
        assert_eq!(calls_to(&e, 0x0300_1000), vec![vec![form, 0x55]]);
        assert_eq!(run(&mut e, 3, form), (true, 1.0));
        e.mem.set_u32(data + 0x50, 0x66);
        assert_eq!(run(&mut e, 9, form), (true, 0.0));
        e.mem.set_u32(data + 0x50, 0x55);
        assert_eq!(run(&mut e, 9, form), (true, 1.0));
        assert_eq!(run(&mut e, 10, form), (true, 1.0));
        e.mem.set_u32(action + 0xc, 0);
        assert_eq!(run(&mut e, 2, form), (true, 0.0));
    }

    #[test]
    fn words_bytes_flags_and_distance() {
        let (mut e, action, _) = engine(3, false);
        e.mem.set_u32(action, 0x11);
        e.mem.set_u32(action + 0x10, 0x22);
        assert_eq!(run(&mut e, 5, 0x22), (true, 1.0));
        assert_eq!(run(&mut e, 6, 0x11), (true, 1.0));
        assert_eq!(run(&mut e, 6, 0x22), (true, 0.0));
        e.mem.set_u8(action + 4, 1);
        assert_eq!(run(&mut e, 7, 0), (true, 1.0));
        assert_eq!(run(&mut e, 12, 0), (true, 1.0));
        for value in [8, 11, 13, 14] {
            assert_eq!(run(&mut e, value, 0), (true, 0.0));
        }
        e.mem.set_u8(action + 7, 1);
        assert_eq!(run(&mut e, 17, 0), (true, 1.0));
        assert_eq!(run(&mut e, 16, 0), (true, 0.0));
        e.mem.set_u8(action + 0x24, 1);
        assert_eq!(run(&mut e, 16, 0), (true, 1.0));
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLAYER_POINTER, 0x4444u32);
        e.register(DISTANCE_BETWEEN_FACING_ACTORS, |_, _| Ret {
            st0: 123.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 4, 0), (true, 123.5));
        assert_eq!(
            calls_to(&e, DISTANCE_BETWEEN_FACING_ACTORS),
            vec![vec![0x4444, 0, 0]]
        );
    }

    #[test]
    fn the_console_prints_the_value() {
        let (mut e, action, _) = engine(4, true);
        e.mem.set_u8(action + 4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, 7, 0x33), (true, 1.0));
        let bits = 1.0f64.to_bits();
        assert_eq!(
            calls_to(&e, CONSOLE_PRINT),
            vec![vec![
                DEBUG_FORMAT,
                7,
                0x33,
                bits as u32,
                (bits >> 32) as u32
            ]]
        );
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn get_vats_value_parses_then_asks_the_condition() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 4);
        // The parse fills its two outputs with 0x11 and 0x5000 and answers
        // the word at 0x0f00_0000.
        e.register(0x005a_ccb0, |e, a| {
            e.mem.set_u32(a[7], 0x11);
            e.mem.set_u32(a[8], 0x5000);
            Ret {
                eax: e.mem.u32(0x0f00_0000),
                ..Ret::default()
            }
        });
        // VATS mode 0: no action, the condition answers false.
        e.register(WORD_AT, |_, _| Ret::default());
        e.register(IS_CONSOLE_VISIBLE, |_, _| Ret::default());
        let result = e.mem.alloc(8);
        e.mem.set_f64(result, 7.0);
        let words = [1u32, 2, 3, 4, 5, 6, result, 8];
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0059_4dc0, &words).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(&log[1].1[..7], &[1, 2, 8, 3, 4, 5, 6]);
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.f64(result), 7.0);
        // Parsed: the condition runs (and clears the result).
        e.mem.set_u32(0x0f00_0000, 1);
        assert!(!e.call(0x0059_4dc0, &words).bool());
        assert_eq!(e.mem.f64(result), 0.0);
    }
}
