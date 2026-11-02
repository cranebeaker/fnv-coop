//! `Havok SDK/run_00dfebe0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5b4c.
pub fn return_constant_00dfebe0(_e: &mut Engine) -> u32 {
    0x010e_5b4c
}

// Translated from 00dfec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5bfc.
pub fn return_constant_00dfec20(_e: &mut Engine) -> u32 {
    0x010e_5bfc
}

// Translated from 00dfec80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5d94.
pub fn return_constant_00dfec80(_e: &mut Engine) -> u32 {
    0x010e_5d94
}

// Translated from 00dfeca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5da8.
pub fn return_constant_00dfeca0(_e: &mut Engine) -> u32 {
    0x010e_5da8
}

// Translated from 00dfece0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5e9c.
pub fn return_constant_00dfece0(_e: &mut Engine) -> u32 {
    0x010e_5e9c
}

// Translated from 00dfed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_5ff4.
pub fn return_constant_00dfed20(_e: &mut Engine) -> u32 {
    0x010e_5ff4
}

// Translated from 00dfebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc4229). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfebb0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfebd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc421f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5b4c` (RTTI `hkpVehicleManager`) at `+0`.
pub fn fn_00dfebd0(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5b4c);
    }
}

// Translated from 00dfebf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc4299). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfebf0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc428f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5bfc` (RTTI `hkpVehicleLinearCastBatchingManager`) at `+0`.
pub fn fn_00dfec10(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5bfc);
    }
}

// Translated from 00dfec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc4309). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfec30(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfec50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc4379). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfec50(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc42ff). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5d94` (RTTI `hkpVehicleDriverInputAnalogStatus`) at `+0`.
pub fn fn_00dfec70(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5d94);
    }
}

// Translated from 00dfec90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc436f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5da8` (RTTI `hkpVehicleDefaultAnalogDriverInput`) at `+0`.
pub fn fn_00dfec90(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5da8);
    }
}

// Translated from 00dfecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc43e9). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfecb0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc43df). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5e9c` (RTTI `hkpVehicleDefaultVelocityDamper`) at `+0`.
pub fn fn_00dfecd0(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5e9c);
    }
}

// Translated from 00dfecf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc4459). Cdecl `(object)`: calls the first
/// virtual slot of `object` (`[[object]]`, the destructor entry in the Xbox PDB's class listings)
/// with the one stack word 0 and answers that call's result.
pub fn fn_00dfecf0(e: &mut Engine, object: Ptr) -> u32 {
    e.vcall(object.addr(), 0, &args![0u32]).u32()
}

// Translated from 00dfed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (Ghidra never created it; ghidra_gaps.tsv: imm 00fc444f). Cdecl `(object)`: does nothing for a
/// null `object`; otherwise stores the vtable word `0x010e_5ff4` (RTTI `hkpVehicleDefaultTransmission`) at `+0`.
pub fn fn_00dfed10(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.mem.set_u32(object.addr(), 0x010e_5ff4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfebe0, return_constant_00dfebe0() -> u32),
        entry!(0x00dfec20, return_constant_00dfec20() -> u32),
        entry!(0x00dfec80, return_constant_00dfec80() -> u32),
        entry!(0x00dfeca0, return_constant_00dfeca0() -> u32),
        entry!(0x00dfece0, return_constant_00dfece0() -> u32),
        entry!(0x00dfed20, return_constant_00dfed20() -> u32),
        entry!(0x00df_ebb0, fn_00dfebb0(Ptr) -> u32),
        entry!(0x00df_ebd0, fn_00dfebd0(Ptr)),
        entry!(0x00df_ebf0, fn_00dfebf0(Ptr) -> u32),
        entry!(0x00df_ec10, fn_00dfec10(Ptr)),
        entry!(0x00df_ec30, fn_00dfec30(Ptr) -> u32),
        entry!(0x00df_ec50, fn_00dfec50(Ptr) -> u32),
        entry!(0x00df_ec70, fn_00dfec70(Ptr)),
        entry!(0x00df_ec90, fn_00dfec90(Ptr)),
        entry!(0x00df_ecb0, fn_00dfecb0(Ptr) -> u32),
        entry!(0x00df_ecd0, fn_00dfecd0(Ptr)),
        entry!(0x00df_ecf0, fn_00dfecf0(Ptr) -> u32),
        entry!(0x00df_ed10, fn_00dfed10(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfebe0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ebe0, &args![]).u32(), 0x010e_5b4c);
    }

    #[test]
    fn return_constant_00dfec20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ec20, &args![]).u32(), 0x010e_5bfc);
    }

    #[test]
    fn return_constant_00dfec80_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ec80, &args![]).u32(), 0x010e_5d94);
    }

    #[test]
    fn return_constant_00dfeca0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_eca0, &args![]).u32(), 0x010e_5da8);
    }

    #[test]
    fn return_constant_00dfece0_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ece0, &args![]).u32(), 0x010e_5e9c);
    }

    #[test]
    fn return_constant_00dfed20_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_ed20, &args![]).u32(), 0x010e_5ff4);
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

    cleanup_test!(fn_00dfebb0_calls_virtual_slot_0_with_zero, 0x00df_ebb0);
    cleanup_test!(fn_00dfebf0_calls_virtual_slot_0_with_zero, 0x00df_ebf0);
    cleanup_test!(fn_00dfec30_calls_virtual_slot_0_with_zero, 0x00df_ec30);
    cleanup_test!(fn_00dfec50_calls_virtual_slot_0_with_zero, 0x00df_ec50);
    cleanup_test!(fn_00dfecb0_calls_virtual_slot_0_with_zero, 0x00df_ecb0);
    cleanup_test!(fn_00dfecf0_calls_virtual_slot_0_with_zero, 0x00df_ecf0);
    vtable_test!(fn_00dfebd0_stores_the_vtable_word, 0x00df_ebd0, 0x010e_5b4c);
    vtable_test!(fn_00dfec10_stores_the_vtable_word, 0x00df_ec10, 0x010e_5bfc);
    vtable_test!(fn_00dfec70_stores_the_vtable_word, 0x00df_ec70, 0x010e_5d94);
    vtable_test!(fn_00dfec90_stores_the_vtable_word, 0x00df_ec90, 0x010e_5da8);
    vtable_test!(fn_00dfecd0_stores_the_vtable_word, 0x00df_ecd0, 0x010e_5e9c);
    vtable_test!(fn_00dfed10_stores_the_vtable_word, 0x00df_ed10, 0x010e_5ff4);
}
