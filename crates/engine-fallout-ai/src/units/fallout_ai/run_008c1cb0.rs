//! `fallout/ai/run_008c1cb0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c1cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor body (the engine map has no name): the vtable `01084fec`,
/// then `008454f0` with 1; its `EAX` is answered as it is.
pub fn fn_008c1cb0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), 0x0108_4fec);
    e.call(0x0084_54f0, &args![this, 1u32]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c_1cb0, fn_008c1cb0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_then_calls_with_one() {
        let mut e = Engine::new();
        e.register(0x0084_54f0, |_, a| (a[0] + a[1]).into_ret());
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x008c_1cb0, &args![this]).u32(), this + 1);
        assert_eq!(e.mem.u32(this), 0x0108_4fec);
    }
}
