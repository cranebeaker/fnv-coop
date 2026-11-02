//! `Havok SDK/run_00e00ed0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e00f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map. Builds a temporary object in a 0x30-byte stack buffer by calling `00e00fb0` on it (flag word 0; the callee pops it) and returns the first word of that buffer, which is what the code reads back after the call. The class is unconfirmed.
pub fn fn_00e00f90(e: &mut Engine) -> u32 {
    e.with_stack(0x30, |e, buffer| {
        e.call(0x00e0_0fb0, &args![buffer, 0u32]);
        e.mem.u32(buffer.addr())
    })
}

// Translated from 00e00ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_0e5c.
pub fn return_constant_00e00ed0(_e: &mut Engine) -> u32 {
    0x010d_0e5c
}

// Translated from 00e00f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_102c.
pub fn return_constant_00e00f40(_e: &mut Engine) -> u32 {
    0x010d_102c
}

// Translated from 00e010b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010d_10c4.
pub fn return_constant_00e010b0(_e: &mut Engine) -> u32 {
    0x010d_10c4
}

// Translated from 00e01130 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_901c.
pub fn return_constant_00e01130(_e: &mut Engine) -> u32 {
    0x010e_901c
}

// Translated from 00e00fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010e8e94`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010e8e74`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, 7) and the byte at `+0x15` (3,
/// once). Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e00fb0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x010c_c364);
    e.mem.set_u32(base + 0x10, 0x010c_c344);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 3);
    }
    e.mem.set_u32(base, 0x010e_8e94);
    e.mem.set_u32(base + 0x10, 0x010e_8e74);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
    }
    this
}

// Translated from 00e00e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc5f59). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e00e70(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e00e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc5f4f). Cdecl
/// `(object, flag)`: does nothing for a null `object`. Otherwise it writes the object's
/// constructor chain: vtable word `0x010305a8` (RTTI `hkpShape`) at `+0` and, when `flag` is not 0,
/// `0` at `+0xc`; `0x010c6fd0` (RTTI `hkpSingleShapeContainer`) at `+0x14`; vtable word
/// `0x010d0e5c` (RTTI `hkpConvexTranslateShape`) at `+0`; and the word `0xb` at `+0xc`.
pub fn fn_00e00e90(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let me = object.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(me.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(me.wrapping_add(0x14), 0x010c_6fd0);
    e.mem.set_u32(me, 0x010d_0e5c);
    e.mem.set_u32(me.wrapping_add(0xc), 0xb);
}

// Translated from 00e00ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc5fc9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e00ee0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e00f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc5fbf). Cdecl
/// `(object, flag)`: does nothing for a null `object`. Otherwise it writes the object's
/// constructor chain: vtable word `0x010305a8` (RTTI `hkpShape`) at `+0` and, when `flag` is not 0,
/// `0` at `+0xc`; `0x010c6fd0` (RTTI `hkpSingleShapeContainer`) at `+0x14`; vtable word
/// `0x010d102c` (RTTI `hkpConvexTransformShape`) at `+0`; and the word `0xc` at `+0xc`.
pub fn fn_00e00f00(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let me = object.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(me.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(me.wrapping_add(0x14), 0x010c_6fd0);
    e.mem.set_u32(me, 0x010d_102c);
    e.mem.set_u32(me.wrapping_add(0xc), 0xc);
}

// Translated from 00e00f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6039). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e00f50(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e00f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc602f). Cdecl `(object, flag)`: does nothing
/// for a null `object`; otherwise calls `00e00fb0` (thiscall on `object` with `flag` as its one stack word).
pub fn fn_00e00f70(e: &mut Engine, object: Ptr, flag: u32) {
    if !object.is_null() {
        e.call(0x00e0_0fb0, &args![object, flag]);
    }
}

// Translated from 00e01050 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc60a9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e01050(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e01070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc609f). Cdecl
/// `(object, flag)`: does nothing for a null `object`. Otherwise it writes the object's
/// constructor chain: vtable word `0x010305a8` (RTTI `hkpShape`) at `+0` and, when `flag` is not 0,
/// `0` at `+0xc`; `0x010c6fb0` (RTTI `hkpShapeContainer`) at `+0x14`; vtable word `0x010d10c4`
/// (RTTI `hkpConvexListShape`) at `+0`; `0x010d10a4` (the same class's second vtable) at `+0x14`;
/// and the word `0x16` at `+0xc`.
pub fn fn_00e01070(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let me = object.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(me.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(me.wrapping_add(0x14), 0x010c_6fb0);
    e.mem.set_u32(me, 0x010d_10c4);
    e.mem.set_u32(me.wrapping_add(0x14), 0x010d_10a4);
    e.mem.set_u32(me.wrapping_add(0xc), 0x16);
}

// Translated from 00e010c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6129). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e010c0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e010e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc611f). Cdecl
/// `(object, flag)`: does nothing for a null `object`. Otherwise it writes the object's
/// constructor chain: vtable word `0x010305a8` (RTTI `hkpShape`) at `+0` and, when `flag` is not 0,
/// `0` at `+0xc`; then `0x19` at `+0xc` and the vtable word `0x010ce31c` (RTTI
/// `hkpSampledHeightFieldShape`) at `+0`; when `flag` is not 0, `0xd` at `+0xc`; the vtable word
/// `0x010e901c` (RTTI `hkpCompressedSampledHeightFieldShape`) at `+0`; and when `flag` is not 0
/// the byte `1` at `+0x1d`.
pub fn fn_00e010e0(e: &mut Engine, object: Ptr, flag: u32) {
    if object.is_null() {
        return;
    }
    let me = object.addr();
    e.mem.set_u32(me, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(me.wrapping_add(0xc), 0);
    }
    e.mem.set_u32(me.wrapping_add(0xc), 0x19);
    e.mem.set_u32(me, 0x010c_e31c);
    if flag != 0 {
        e.mem.set_u32(me.wrapping_add(0xc), 0xd);
    }
    e.mem.set_u32(me, 0x010e_901c);
    if flag != 0 {
        e.mem.set_u8(me.wrapping_add(0x1d), 1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_0f90, fn_00e00f90() -> u32),
        entry!(0x00e00ed0, return_constant_00e00ed0() -> u32),
        entry!(0x00e00f40, return_constant_00e00f40() -> u32),
        entry!(0x00e010b0, return_constant_00e010b0() -> u32),
        entry!(0x00e01130, return_constant_00e01130() -> u32),
        entry!(0x00e0_0fb0, fn_00e00fb0(Ptr, u32) -> Ptr),
        entry!(0x00e0_0e70, fn_00e00e70(Ptr) -> u32),
        entry!(0x00e0_0e90, fn_00e00e90(Ptr, u32)),
        entry!(0x00e0_0ee0, fn_00e00ee0(Ptr) -> u32),
        entry!(0x00e0_0f00, fn_00e00f00(Ptr, u32)),
        entry!(0x00e0_0f50, fn_00e00f50(Ptr) -> u32),
        entry!(0x00e0_0f70, fn_00e00f70(Ptr, u32)),
        entry!(0x00e0_1050, fn_00e01050(Ptr) -> u32),
        entry!(0x00e0_1070, fn_00e01070(Ptr, u32)),
        entry!(0x00e0_10c0, fn_00e010c0(Ptr) -> u32),
        entry!(0x00e0_10e0, fn_00e010e0(Ptr, u32)),
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
        fn_00e00f90_returns_first_word_of_built_object,
        0x00e0_0f90,
        0x00e0_0fb0
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
    fn return_constant_00e00ed0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0ed0, &args![]).u32(), 0x010d_0e5c);
    }

    #[test]
    fn return_constant_00e00f40_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_0f40, &args![]).u32(), 0x010d_102c);
    }

    #[test]
    fn fn_00e00f90_constructs_a_temporary_and_returns_its_vtable_word() {
        let mut e = Engine::new();
        let log = constructor_double(&mut e, 0x00e0_0fb0, 0x0a0b_0c0d);
        let result = e.call(0x00e0_0f90, &args![]).u32();
        assert_eq!(result, 0x0a0b_0c0d);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].len(), 2);
        assert_ne!(calls[0][0], 0);
        assert_eq!(calls[0][1], 0);
    }

    #[test]
    fn return_constant_00e010b0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_10b0, &args![]).u32(), 0x010d_10c4);
    }

    #[test]
    fn return_constant_00e01130_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_1130, &args![]).u32(), 0x010e_901c);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// Mode 0 leaves +0xc and the byte at +0x15 alone; another mode ends with 7 and 3.
    #[test]
    fn fn_00e00fb0_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0x66; 0x20]);
        assert_eq!(e.call(0x00e0_0fb0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_8e94);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_8e74);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.call(0x00e0_0fb0, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0xc), 7);
        assert_eq!(e.mem.u8(this + 0x15), 3);
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

    finish_test!(
        fn_00e00f70_calls_00e00fb0_for_a_non_null_object,
        0x00e0_0f70,
        0x00e0_0fb0
    );
    cleanup_test!(fn_00e00e70_calls_virtual_slot_0_with_zero, 0x00e0_0e70);
    cleanup_test!(fn_00e00ee0_calls_virtual_slot_0_with_zero, 0x00e0_0ee0);
    cleanup_test!(fn_00e00f50_calls_virtual_slot_0_with_zero, 0x00e0_0f50);
    cleanup_test!(fn_00e01050_calls_virtual_slot_0_with_zero, 0x00e0_1050);
    cleanup_test!(fn_00e010c0_calls_virtual_slot_0_with_zero, 0x00e0_10c0);

    #[test]
    fn fn_00e00e90_writes_the_convex_translate_shape_header() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_0e90, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0e5c);
        assert_eq!(e.mem.u32(object + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(object + 0xc), 0xb);
        e.call(0x00e0_0e90, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_0e5c);
        assert_eq!(e.mem.u32(object + 0xc), 0xb);
        e.call(0x00e0_0e90, &args![0u32, 1u32]);
    }

    #[test]
    fn fn_00e00f00_writes_the_convex_transform_shape_header() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_0f00, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_102c);
        assert_eq!(e.mem.u32(object + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(object + 0xc), 0xc);
        e.call(0x00e0_0f00, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_102c);
        assert_eq!(e.mem.u32(object + 0xc), 0xc);
        e.call(0x00e0_0f00, &args![0u32, 1u32]);
    }

    #[test]
    fn fn_00e01070_writes_the_convex_list_shape_header() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_1070, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010d_10c4);
        assert_eq!(e.mem.u32(object + 0x14), 0x010d_10a4);
        assert_eq!(e.mem.u32(object + 0xc), 0x16);
        e.call(0x00e0_1070, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010d_10c4);
        assert_eq!(e.mem.u32(object + 0xc), 0x16);
        e.call(0x00e0_1070, &args![0u32, 1u32]);
    }

    #[test]
    fn fn_00e010e0_writes_the_compressed_height_field_header() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        e.call(0x00e0_10e0, &args![object, 0u32]);
        assert_eq!(e.mem.u32(object), 0x010e_901c);
        assert_eq!(e.mem.u32(object + 0xc), 0x19);
        assert_eq!(e.mem.u8(object + 0x1d), 0);
        e.call(0x00e0_10e0, &args![object, 1u32]);
        assert_eq!(e.mem.u32(object), 0x010e_901c);
        assert_eq!(e.mem.u32(object + 0xc), 0xd);
        assert_eq!(e.mem.u8(object + 0x1d), 1);
        e.call(0x00e0_10e0, &args![0u32, 1u32]);
    }
}
