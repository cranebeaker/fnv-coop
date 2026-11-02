//! `BSMain/run_00c5edc0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `007cb2e0` (`BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized` in the engine map;
/// `thiscall`, `RET 4`): appends the word at the address given.
const ARRAY_APPEND: u32 = 0x007c_b2e0;

// Translated from 00c5edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): appends its word to the array at +0xC
/// (`007cb2e0` with the address of the argument).
pub fn fn_00c5edc0(e: &mut Engine, this: Ptr, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_APPEND, &args![this.addr() + 0xc, slot]);
    });
}

// Translated from 00c5ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the word at `index` of the array whose
/// buffer is at +0x10.
pub fn fn_00c5ede0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let buffer = e.mem.u32(this.addr() + 0x10);
    e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_edc0, fn_00c5edc0(Ptr, u32)),
        entry!(0x00c5_ede0, fn_00c5ede0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn append_and_read_back() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(ARRAY_APPEND, move |e, a| {
            inner.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this = e.mem.alloc(0x1c);
        e.call(0x00c5_edc0, &args![this, 0x77u32]);
        assert_eq!(*seen.borrow(), vec![(this + 0xc, 0x77)]);
        let buffer = e.mem.alloc(12);
        e.mem.set_u32(buffer + 8, 0x99);
        e.mem.set_u32(this + 0x10, buffer);
        assert_eq!(e.call(0x00c5_ede0, &args![this, 2u32]).u32(), 0x99);
    }
}
