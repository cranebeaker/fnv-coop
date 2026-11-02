//! `(unplaced)/run_008eea60` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008eea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): calls `this`'s vtable slot +0x218
/// and passes its result (`EAX`, `EDX`, `ST0`) back unchanged.
pub fn fn_008eea60(e: &mut Engine, this: Ptr) -> Ret {
    e.vcall(this.addr(), 0x218, &[])
}

/// The registered form of `008eea60`: `this`, the result passed back whole.
fn fn_008eea60_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fn_008eea60(e, Ptr::new(words[0]))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x008e_ea60, fn_008eea60_entry as AbiFn)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_the_slot_result_on() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x220);
        e.mem.set_u32(vtable + 0x218, vtable + 0x218);
        e.register(vtable + 0x218, |_, a| Ret {
            eax: a[0] + 1,
            edx: 2,
            st0: 0.5,
        });
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        let r = e.call(0x008e_ea60, &args![this]);
        assert_eq!((r.eax, r.edx, r.st0), (this + 1, 2, 0.5));
    }
}
