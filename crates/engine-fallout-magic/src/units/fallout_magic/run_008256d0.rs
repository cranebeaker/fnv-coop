//! `fallout/magic/run_008256d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008256d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether both words at `+0` and `+4` of `this` are zero.
pub fn fn_008256d0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 4) == 0 && e.mem.u32(this.addr()) == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008256d0, fn_008256d0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_only_when_both_words_are_zero() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        assert!(e.call(0x0082_56d0, &args![object]).bool());
        e.mem.set_u32(object.addr(), 1);
        assert!(!e.call(0x0082_56d0, &args![object]).bool());
        e.mem.set_u32(object.addr() + 4, 1);
        assert!(!e.call(0x0082_56d0, &args![object]).bool());
        e.mem.set_u32(object.addr(), 0);
        assert!(!e.call(0x0082_56d0, &args![object]).bool());
    }
}
