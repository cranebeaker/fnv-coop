//! `NiMain/run_00a89af0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a89af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an empty `thiscall` body (`RET 4`), called with `this` in
/// `ECX` and one stack word (the stream, from the `LinkObject`/`SaveBinary` callers, e.g.
/// `00a326e0`); it does nothing (identical-code folding shares it between such callers).
pub fn fn_00a89af0(_e: &mut Engine, _this: Ptr, _unused_0: u32) {}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_9af0, fn_00a89af0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_nothing() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234_5678);
        e.call(0x00a8_9af0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this), 0x1234_5678);
    }
}
