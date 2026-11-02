//! `fallout shared/run_00540c20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable slot `00540c20` calls (+0x10).
const RELEASE_SLOT: u32 = 0x10;

// Translated from 00540c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `this`'s virtual slot +0x10 with `first` and `second` (thiscall,
/// `RET 8`; unnamed in the engine map). The code also keeps a constant 1
/// in a stack local it never passes.
pub fn fn_00540c20(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.vcall(this.addr(), RELEASE_SLOT, &args![first, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0054_0c20, fn_00540c20(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calls_virtual_slot_0x10_with_both_words() {
        let mut e = Engine::new();
        let vtable = 0x0360_0000;
        e.put_vtable(vtable, &[0, 0, 0, 0, 0x0350_0000]);
        e.register(0x0350_0000, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.call_log = Some(vec![]);
        e.call(0x0054_0c20, &args![this, 0x11u32, 0x22u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0350_0000).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![this, 0x11, 0x22]);
    }
}
