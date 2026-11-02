//! `fallout shared/sky/run_00633c90` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Adds a reference to a counted object (`0040f6e0`: increments the count at
/// `+4`).
const ADD_REFERENCE: u32 = 0x0040_f6e0;

// Translated from 00633c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Smart-pointer constructor from a raw pointer (`__thiscall`, `RET 4`):
/// stores `pointer` at `+0` of `this`, adds a reference to the object when
/// it is not null, and returns `this`.
pub fn fn_00633c90(e: &mut Engine, this: Ptr, pointer: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), pointer.addr());
    if !pointer.is_null() {
        e.call(ADD_REFERENCE, &args![pointer]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00633c90, fn_00633c90(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_pointer_and_adds_a_reference() {
        let mut e = Engine::new();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        let holder: Ptr = Ptr::new(e.mem.alloc(4));
        e.call_log = Some(vec![]);
        let back = e.call(0x0063_3c90, &args![holder, 0x5000u32]);
        assert_eq!(back.ptr::<()>(), holder);
        assert_eq!(e.mem.u32(holder.addr()), 0x5000);
        assert_eq!(e.call_log.take().unwrap()[1], (ADD_REFERENCE, vec![0x5000]));
    }

    #[test]
    fn a_null_pointer_adds_no_reference() {
        let mut e = Engine::new();
        let holder: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(holder.addr(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0063_3c90, &args![holder, Ptr::<()>::NULL]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(holder.addr()), 0);
    }
}
