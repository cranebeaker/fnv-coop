//! `Havok SDK/run_00d47750` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d47750 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reference-counted pointer's copy constructor (thiscall, `RET 4`: one stack word `source`, the
/// address of another such pointer; no map name): reads the pointer `p` in `*source`; when it is not
/// null calls `00c90510` on it (thiscall, no stack words: the exe's reference increment); stores
/// `p` at `this + 0`. Returns `this`.
pub fn fn_00d47750(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let pointer = e.mem.u32(source.addr());
    if pointer != 0 {
        e.call(0x00c9_0510, &args![pointer]);
    }
    e.mem.set_u32(this.addr(), pointer);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_7750, fn_00d47750(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d47750_copies_the_pointer_and_adds_a_reference() {
        let mut e = Engine::new();
        e.register(0x00c9_0510, |_, _| Ret::default());
        let source = e.mem.alloc(8);
        let this = e.mem.alloc(8);
        e.mem.set_u32(source, 0x5000);
        e.call_log = Some(vec![]);
        let result = e.call(0x00d4_7750, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x00c9_0510, vec![0x5000])]);
        // A null pointer is copied without a call.
        e.mem.set_u32(source, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d4_7750, &args![this, source]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this), 0);
    }
}
