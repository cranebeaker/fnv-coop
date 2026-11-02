//! `Havok SDK/run_00d05db0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d05db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing (an empty constructor body; its caller `buildAndAddCastJobs` sets `ECX` to a
/// local object first and pushes nothing). The class is unconfirmed.
pub fn fn_00d05db0(_e: &mut Engine, _this: Ptr) {}

// Translated from 00d05de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing (an empty constructor body, called with `ECX` set to a local object and no
/// stack arguments). The class is unconfirmed.
pub fn fn_00d05de0(_e: &mut Engine, _this: Ptr) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_5db0, fn_00d05db0(Ptr)),
        entry!(0x00d0_5de0, fn_00d05de0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Neither function touches memory.
    #[test]
    fn both_do_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        e.call(0x00d0_5db0, &args![this]);
        e.call(0x00d0_5de0, &args![this]);
        assert_eq!(e.mem.bytes(this, 0x10), vec![0xaa; 0x10]);
    }
}
