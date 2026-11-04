//! `fallout shared/run_00493830` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `thiscall` test `008256d0` applied to `this + 0x104` (answers a
/// `bool` in `AL`; the engine map has no name).
const MEMBER_TEST: u32 = 0x0082_56d0;

// Translated from 00493830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The negation of `008256d0` on the member at +0x104 (the engine map has
/// no name): 1 when its `AL` is zero, else 0.
pub fn fn_00493830(e: &mut Engine, this: Ptr) -> bool {
    !e.call(MEMBER_TEST, &args![this.addr() + 0x104]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0049_3830, fn_00493830(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_negated_member_test() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x108);
        e.register(MEMBER_TEST, |_, _| 0x100u32.into_ret());
        assert_eq!(e.call(0x0049_3830, &args![this]).u32(), 1);
        e.register(MEMBER_TEST, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x0049_3830, &args![this]).u32(), 0);
    }
}
