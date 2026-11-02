//! `NiMain/run_00a693e0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCullingProcess`'s vtable (RTTI `.?AVNiCullingProcess@@`).
const NI_CULLING_PROCESS_VTABLE: u32 = 0x0109_c28c;

// Translated from 00a693e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: puts `NiCullingProcess`'s vtable at `this`
/// (the body the destructors of culling processes, such as
/// `~BSCullingProcess`, call last for their `NiCullingProcess` base).
pub fn fn_00a693e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_CULLING_PROCESS_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a693e0, fn_00a693e0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_the_culling_process_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0xdead_beef);
        e.call(0x00a693e0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_CULLING_PROCESS_VTABLE);
    }
}
