//! `NiMain/run_00a9b120` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a80540` (`thiscall`; no Xbox PDB name).
const BASE_CONSTRUCTOR_00A80540: u32 = 0x00a8_0540;
/// The vtable of `NiParticles` (RTTI `.?AVNiParticles@@`).
const VTABLE: u32 = 0x010a_11d4;

// Translated from 00a9b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiParticles` constructor without arguments (`thiscall`; no Xbox PDB
/// name): the base constructor `00a80540`, then the `NiParticles` vtable.
/// Returns `this`.
pub fn fn_00a9b120(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00A80540, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_b120, fn_00a9b120(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_then_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xf0);
        e.register(BASE_CONSTRUCTOR_00A80540, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b120, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (BASE_CONSTRUCTOR_00A80540, vec![this])
        );
    }
}
