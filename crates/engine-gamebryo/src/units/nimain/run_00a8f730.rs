//! `NiMain/run_00a8f730` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `011f5e34`: the object `00a8f870` walks (an `NiTFixedStringMap` by its use there).
const HELD_OBJECT_SLOT: u32 = 0x011f_5e34;

// Translated from 00a8f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): when `011f5e34` holds an object, calls its vtable slot 0 with 1 (`thiscall`)
/// and clears the pointer.
pub fn fn_00a8f730(e: &mut Engine) {
    let object = e.mem.u32(HELD_OBJECT_SLOT);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    e.mem.set_u32(HELD_OBJECT_SLOT, 0);
}

/// `011f5d30`: the byte (and object) `00a8f750` answers.
const FLAG_OBJECT: u32 = 0x011f_5d30;

// Translated from 00a8f750 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; a function Ghidra did not create): the address
/// `011f5d30` when its first byte is not 0 (`MOVSX`, `NEG`, `SBB`, `AND`), else 0.
pub fn fn_00a8f750(e: &mut Engine) -> u32 {
    if e.mem.u8(FLAG_OBJECT) != 0 {
        FLAG_OBJECT
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_f730, fn_00a8f730()),
        entry!(0x00a8_f750, fn_00a8f750() -> u32),
    ]
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a8f730_destroys_the_held_object_and_clears_the_pointer() {
        let mut e = Engine::new();
        e.map(HELD_OBJECT_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        e.mem.set_u32(HELD_OBJECT_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f730, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0150_1000, vec![object, 1])]);
        assert_eq!(e.mem.u32(HELD_OBJECT_SLOT), 0);
        e.call_log = Some(vec![]);
        e.call(0x00a8_f730, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(HELD_OBJECT_SLOT), 0);
    }
}

#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn answers_the_address_while_the_byte_is_set() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        assert_eq!(e.call(0x00a8_f750, &args![]).u32(), 0);
        e.mem.set_u8(FLAG_OBJECT, 0x80);
        assert_eq!(e.call(0x00a8_f750, &args![]).u32(), FLAG_OBJECT);
    }
}
