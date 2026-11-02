//! `Havok SDK/run_00d21cc0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d21cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: three stack words (cdecl, nothing popped): `object`, `a` and `b`.
/// Calls the virtual function at +0x30 of `object` (`this` in `ECX`) with `a` and `b`, which the
/// callee pops, and returns its answer. `hkpConstraintAtom`'s build function word holds this
/// address. Ghidra made no function here; the address is an immediate operand at 00d21ce7.
pub fn fn_00d21cc0(e: &mut Engine, object: u32, a: u32, b: u32) -> u32 {
    e.vcall(object, 0x30, &args![a, b]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_1cc0, fn_00d21cc0(u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d21cc0_calls_slot_0x30_with_both_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 0x30, 0x0900_0005);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0005, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 9,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x00d2_1cc0, &args![object, 7u32, 8u32]).u32(), 9);
        assert_eq!(*seen.borrow(), vec![vec![object, 7, 8]]);
    }
}
