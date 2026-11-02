//! `fallout/ai/run_008dd860` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `005d43c0`: `thiscall`, returns `this + 0x44`.
const PLUS_44: u32 = 0x005d_43c0;
/// `0042eef0`: `thiscall` on that address, no arguments.
const CALLEE_0042EEF0: u32 = 0x0042_eef0;

// Translated from 008dd860 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `0042eef0` on the address `005d43c0` gives (`this +
/// 0x44`). `EAX` is left as that call's result, which this returns.
pub fn fn_008dd860(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.call(PLUS_44, &args![this]).u32();
    e.call(CALLEE_0042EEF0, &args![inner]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d_d860, fn_008dd860(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_008dd860_calls_on_this_plus_44() {
        let mut e = Engine::new();
        e.register(PLUS_44, |_, a| (a[0] + 0x44).into_ret());
        e.register(CALLEE_0042EEF0, |_, a| (a[0] ^ 1).into_ret());
        assert_eq!(e.call(0x008d_d860, &args![0x1000u32]).u32(), 0x1045);
    }
}
