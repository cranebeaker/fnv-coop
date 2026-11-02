//! `Havok SDK/run_00dfda80` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfda80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x20-byte stack buffer by calling `00e3f510` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00dfda80(e: &mut Engine) -> u32 {
    e.with_stack(0x20, |e, buffer| {
        e.call(0x00e3_f510, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00dfdb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_351c.
pub fn return_constant_00dfdb60(_e: &mut Engine) -> u32 {
    0x010e_351c
}

// Translated from 00dfdb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_352c.
pub fn return_constant_00dfdb80(_e: &mut Engine) -> u32 {
    0x010e_352c
}

// Translated from 00dfda10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011bbb50). Cdecl
/// `(object, flag)`. For a non-null `object`: runs the constructor `00cb0a70` (thiscall, `RET 4`)
/// with `flag` on the member at `+0`, then again on the member at `+4`.
pub fn fn_00dfda10(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    e.call(0x00cb_0a70, &args![object, flag]);
    e.call(0x00cb_0a70, &args![object.byte_add(4), flag]);
}

// Translated from 00dfda50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc355f). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00e3f510` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00dfda50(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e3_f510, &args![object, flag]);
    }
}

// Translated from 00dfda70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc3569). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfda70(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfdab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011bbb54). Cdecl
/// `(object)`: runs `00cb0c00` (thiscall, no stack words) on the member at `+4`, then tail-jumps
/// to it again for the member at `+0`, so it answers the second call's result.
pub fn fn_00dfdab0(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00cb_0c00, &args![object.byte_add(4)]);
    e.call(0x00cb_0c00, &args![object]).u32()
}

// Translated from 00dfdad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011bbc18). Cdecl
/// `(object, flag)`. For a non-null `object`: runs the constructor `00cb0a70` (thiscall, `RET 4`)
/// with `flag` on the member at `+4`.
pub fn fn_00dfdad0(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    e.call(0x00cb_0a70, &args![object.byte_add(4), flag]);
}

// Translated from 00dfdb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc3609). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfdb00(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfdb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc3679). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfdb20(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfdb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: data 011bbc1c). Cdecl
/// `(object)`: tail-jumps to `00cb0c00` (thiscall, no stack words) with `ECX = object + 4`, so it
/// answers that function's result.
pub fn fn_00dfdb40(e: &mut Engine, object: Ptr) -> u32 {
    e.call(0x00cb_0c00, &args![object.byte_add(4)]).u32()
}

// Translated from 00dfdb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc366f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_351c` (RTTI `hkxSparselyAnimatedEnum`) at `+0`.
pub fn fn_00dfdb50(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_351c);
    }
}

// Translated from 00dfdb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc35ff). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_352c` (RTTI `hkxEnum`) at `+0`.
pub fn fn_00dfdb70(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_352c);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_da80, fn_00dfda80() -> u32),
        entry!(0x00dfdb60, return_constant_00dfdb60() -> u32),
        entry!(0x00dfdb80, return_constant_00dfdb80() -> u32),
        entry!(0x00df_da10, fn_00dfda10(Ptr, u32)),
        entry!(0x00df_da50, fn_00dfda50(Ptr, u32)),
        entry!(0x00df_da70, fn_00dfda70(Ptr) -> u32),
        entry!(0x00df_dab0, fn_00dfdab0(Ptr) -> u32),
        entry!(0x00df_dad0, fn_00dfdad0(Ptr, u32)),
        entry!(0x00df_db00, fn_00dfdb00(Ptr) -> u32),
        entry!(0x00df_db20, fn_00dfdb20(Ptr) -> u32),
        entry!(0x00df_db40, fn_00dfdb40(Ptr) -> u32),
        entry!(0x00df_db50, fn_00dfdb50(Ptr)),
        entry!(0x00df_db70, fn_00dfdb70(Ptr)),
    ]
}

#[cfg(test)]
mod a07_first_word_tests {
    use crate::prelude::*;

    const BUILT_WORD: u32 = 0x0abc_def0;

    /// A double over `callee` writes `BUILT_WORD` at the buffer it is given; `function`
    /// must call it once with that buffer and a zero flag word, and return the word.
    fn check(function: u32, callee: u32) {
        let mut e = Engine::new();
        e.register(callee, |e, a| {
            e.mem.set_u32(a[0], BUILT_WORD);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(function, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == callee).collect();
        assert_eq!(calls.len(), 1);
        assert_ne!(calls[0].1[0], 0);
        assert_eq!(calls[0].1[1], 0);
        assert_eq!(result, BUILT_WORD);
    }

    macro_rules! first_word_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check($function, $callee);
            }
        };
    }

    first_word_test!(
        fn_00dfda80_returns_first_word_of_built_object,
        0x00df_da80,
        0x00e3_f510
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type ConstructorLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Double for a constructor: logs its argument words and writes `vtable` at the start of `this`.
    fn constructor_double(e: &mut Engine, address: u32, vtable: u32) -> ConstructorLog {
        let log: ConstructorLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn fn_00dfda80_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e3_f510, 0x0a0b_0c0d);
        let result = e.call(0x00df_da80, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00dfdb60_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_db60, &args![]).u32(), 0x010e_351c);
    }

    #[test]
    fn return_constant_00dfdb80_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_db80, &args![]).u32(), 0x010e_352c);
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A double at `address` that records the argument words of each call and answers `answer`.
    fn recording_double(e: &mut Engine, address: u32, answer: u32) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        log
    }

    /// `function(object, flag)` calls `callee(object, flag)` for a non-null object and nothing for
    /// a null one.
    fn check_finish(function: u32, callee: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, callee, 0);
        e.call(function, &args![0u32, 1u32]);
        assert!(log.borrow().is_empty());
        e.call(function, &args![0x4000u32, 1u32]);
        e.call(function, &args![0x4000u32, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 1], vec![0x4000, 0]]);
    }

    macro_rules! finish_test {
        ($name:ident, $function:expr, $callee:expr) => {
            #[test]
            fn $name() {
                check_finish($function, $callee);
            }
        };
    }

    /// `function(object)` calls slot 0 of the object's vtable with `(object, 0)` and answers
    /// what the slot answers.
    fn check_cleanup(function: u32) {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x7000_0000, 0x1234_5678);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 0x7000_0000);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, table);
        assert_eq!(e.call(function, &args![object]).u32(), 0x1234_5678);
        assert_eq!(*log.borrow(), vec![vec![object, 0]]);
    }

    macro_rules! cleanup_test {
        ($name:ident, $function:expr) => {
            #[test]
            fn $name() {
                check_cleanup($function);
            }
        };
    }

    /// `function(object)` stores `vtable` at the start of a non-null object and leaves a null one.
    fn check_vtable(function: u32, vtable: u32) {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(function, &args![object]);
        assert_eq!(e.mem.u32(object), vtable);
        e.call(function, &args![0u32]);
    }

    macro_rules! vtable_test {
        ($name:ident, $function:expr, $vtable:expr) => {
            #[test]
            fn $name() {
                check_vtable($function, $vtable);
            }
        };
    }

    finish_test!(
        fn_00dfda50_calls_00e3f510_for_a_non_null_object,
        0x00df_da50,
        0x00e3_f510
    );
    cleanup_test!(fn_00dfda70_calls_virtual_slot_0_with_zero, 0x00df_da70);
    cleanup_test!(fn_00dfdb00_calls_virtual_slot_0_with_zero, 0x00df_db00);
    cleanup_test!(fn_00dfdb20_calls_virtual_slot_0_with_zero, 0x00df_db20);
    vtable_test!(fn_00dfdb50_stores_the_vtable_word, 0x00df_db50, 0x010e_351c);
    vtable_test!(fn_00dfdb70_stores_the_vtable_word, 0x00df_db70, 0x010e_352c);

    #[test]
    fn fn_00dfda10_constructs_the_members_at_0_and_4() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00cb_0a70, 0);
        e.call(0x00df_da10, &args![0x4000u32, 1u32]);
        assert_eq!(*log.borrow(), vec![vec![0x4000, 1], vec![0x4004, 1]]);
        e.call(0x00df_da10, &args![0u32, 1u32]);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn fn_00dfdab0_runs_00cb0c00_on_the_member_at_4_then_the_one_at_0() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00cb_0c00, 0x31);
        assert_eq!(e.call(0x00df_dab0, &args![0x4000u32]).u32(), 0x31);
        assert_eq!(*log.borrow(), vec![vec![0x4004], vec![0x4000]]);
    }

    #[test]
    fn fn_00dfdad0_constructs_the_member_at_4() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00cb_0a70, 0);
        e.call(0x00df_dad0, &args![0x4000u32, 1u32]);
        assert_eq!(*log.borrow(), vec![vec![0x4004, 1]]);
        e.call(0x00df_dad0, &args![0u32, 1u32]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn fn_00dfdb40_runs_00cb0c00_on_the_member_at_4() {
        let mut e = Engine::new();
        let log = recording_double(&mut e, 0x00cb_0c00, 0x32);
        assert_eq!(e.call(0x00df_db40, &args![0x4000u32]).u32(), 0x32);
        assert_eq!(*log.borrow(), vec![vec![0x4004]]);
    }
}
