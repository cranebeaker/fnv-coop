//! `Havok SDK/run_00d49c40` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d49c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`, the stack word is never read): writes the vtable word
/// `0x010c_fd10` at `+0` and returns `this`.
pub fn fn_00d49c40(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010c_fd10);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_9c40, fn_00d49c40(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d49c40_writes_the_vtable_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d4_9c40, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_fd10);
    }
}
