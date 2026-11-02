//! `Havok SDK/run_00e02490` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e02490 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_9fa4.
pub fn return_constant_00e02490(_e: &mut Engine) -> u32 {
    0x010e_9fa4
}

// Translated from 00e024d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a0a4.
pub fn return_constant_00e024d0(_e: &mut Engine) -> u32 {
    0x010e_a0a4
}

// Translated from 00e02510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_310c.
pub fn return_constant_00e02510(_e: &mut Engine) -> u32 {
    0x010e_310c
}

// Translated from 00e02550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a10c.
pub fn return_constant_00e02550(_e: &mut Engine) -> u32 {
    0x010e_a10c
}

// Translated from 00e02590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_a154.
pub fn return_constant_00e02590(_e: &mut Engine) -> u32 {
    0x010e_a154
}

// Translated from 00e02460 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6db9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e02460(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6daf). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_9fa4` (RTTI `hkxEdgeSelectionChannel`) at `+0`.
pub fn fn_00e02480(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_9fa4);
    }
}

// Translated from 00e024a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6e29). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e024a0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e024c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6e1f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_a0a4` (RTTI `hkxCamera`) at `+0`.
pub fn fn_00e024c0(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_a0a4);
    }
}

// Translated from 00e024e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6e99). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e024e0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6e8f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_310c` (RTTI `hkxAttributeHolder`) at `+0`.
pub fn fn_00e02500(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_310c);
    }
}

// Translated from 00e02520 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6f19). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e02520(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02540 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6f0f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_a10c` (RTTI `hkxAnimatedVector`) at `+0`.
pub fn fn_00e02540(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_a10c);
    }
}

// Translated from 00e02560 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6f89). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00e02560(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00e02580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc6f7f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_a154` (RTTI `hkxAnimatedQuaternion`) at `+0`.
pub fn fn_00e02580(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_a154);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e02490, return_constant_00e02490() -> u32),
        entry!(0x00e024d0, return_constant_00e024d0() -> u32),
        entry!(0x00e02510, return_constant_00e02510() -> u32),
        entry!(0x00e02550, return_constant_00e02550() -> u32),
        entry!(0x00e02590, return_constant_00e02590() -> u32),
        entry!(0x00e0_2460, fn_00e02460(Ptr) -> u32),
        entry!(0x00e0_2480, fn_00e02480(Ptr)),
        entry!(0x00e0_24a0, fn_00e024a0(Ptr) -> u32),
        entry!(0x00e0_24c0, fn_00e024c0(Ptr)),
        entry!(0x00e0_24e0, fn_00e024e0(Ptr) -> u32),
        entry!(0x00e0_2500, fn_00e02500(Ptr)),
        entry!(0x00e0_2520, fn_00e02520(Ptr) -> u32),
        entry!(0x00e0_2540, fn_00e02540(Ptr)),
        entry!(0x00e0_2560, fn_00e02560(Ptr) -> u32),
        entry!(0x00e0_2580, fn_00e02580(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00e02490_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2490, &args![]).u32(), 0x010e_9fa4);
    }

    #[test]
    fn return_constant_00e024d0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_24d0, &args![]).u32(), 0x010e_a0a4);
    }

    #[test]
    fn return_constant_00e02510_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2510, &args![]).u32(), 0x010e_310c);
    }

    #[test]
    fn return_constant_00e02550_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2550, &args![]).u32(), 0x010e_a10c);
    }

    #[test]
    fn return_constant_00e02590_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e0_2590, &args![]).u32(), 0x010e_a154);
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

    cleanup_test!(fn_00e02460_calls_virtual_slot_0_with_zero, 0x00e0_2460);
    cleanup_test!(fn_00e024a0_calls_virtual_slot_0_with_zero, 0x00e0_24a0);
    cleanup_test!(fn_00e024e0_calls_virtual_slot_0_with_zero, 0x00e0_24e0);
    cleanup_test!(fn_00e02520_calls_virtual_slot_0_with_zero, 0x00e0_2520);
    cleanup_test!(fn_00e02560_calls_virtual_slot_0_with_zero, 0x00e0_2560);
    vtable_test!(fn_00e02480_stores_the_vtable_word, 0x00e0_2480, 0x010e_9fa4);
    vtable_test!(fn_00e024c0_stores_the_vtable_word, 0x00e0_24c0, 0x010e_a0a4);
    vtable_test!(fn_00e02500_stores_the_vtable_word, 0x00e0_2500, 0x010e_310c);
    vtable_test!(fn_00e02540_stores_the_vtable_word, 0x00e0_2540, 0x010e_a10c);
    vtable_test!(fn_00e02580_stores_the_vtable_word, 0x00e0_2580, 0x010e_a154);
}
