//! `Havok SDK/run_00dfe510` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dfe510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: takes no arguments and returns the constant 0x010e_57a4.
pub fn return_constant_00dfe510(_e: &mut Engine) -> u32 {
    0x010e_57a4
}

// Translated from 00dfe4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: one stack word `object` (cdecl, nothing popped). Calls the virtual
/// function in slot 0 of `object`'s vtable on it (`this` in `ECX`) with the word 0, which the callee
/// pops, and returns what it returns (`EAX`). Ghidra made no function here; the address is an immediate operand at 00fc3ef9.
pub fn fn_00dfe4e0(e: &mut Engine, object: u32) -> u32 {
    e.vcall(object, 0, &args![0u32]).u32()
}
// Translated from 00dfe500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: when `object` is not null, writes the vtable word `010e57a4` at its
/// start. It takes a second stack word, `_unused_1`, that it never reads (cdecl, nothing popped);
/// `hkTypeInfoRegistry::finishLoadedObject` (`00d47900`) calls an entry's `+4` function with two words.
/// Ghidra made no function here; the address is an immediate operand at 00fc3eef.
pub fn fn_00dfe500(e: &mut Engine, object: u32, _unused_1: u32) {
    if object != 0 {
        e.mem.set_u32(object, 0x010e_57a4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dfe510, return_constant_00dfe510() -> u32),
        entry!(0x00df_e4e0, fn_00dfe4e0(u32) -> u32),
        entry!(0x00df_e500, fn_00dfe500(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_constant_00dfe510_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00df_e510, &args![]).u32(), 0x010e_57a4);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The virtual function in slot 0 gets the object and the word 0; its answer comes back.
    #[test]
    fn fn_00dfe4e0_calls_slot_zero_with_zero() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable, 0x0900_0002);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0002, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00df_e4e0, &args![object]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![vec![object, 0]]);
    }
    /// A null object is left alone; any other gets the vtable word.
    #[test]
    fn fn_00dfe500_writes_the_vtable_word_for_a_non_null_object() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        e.call(0x00df_e500, &args![0u32, 5u32]);
        e.call(0x00df_e500, &args![object, 5u32]);
        assert_eq!(e.mem.u32(object), 0x010e_57a4);
    }
}
