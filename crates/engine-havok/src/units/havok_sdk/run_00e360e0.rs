//! `Havok SDK/run_00e360e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e360e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a method that does nothing (`RET 4`: it takes one stack word it
/// never reads). The class is unconfirmed.
pub fn fn_00e360e0(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_60e0, fn_00e360e0(Ptr, u32))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The no-op changes nothing.
    #[test]
    fn fn_00e360e0_does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call(0x00e3_60e0, &args![this, 1u32]);
        assert_eq!(e.mem.bytes(this, 0x10), vec![0xaa; 0x10]);
    }
}
