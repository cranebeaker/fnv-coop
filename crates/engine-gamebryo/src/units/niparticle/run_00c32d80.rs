//! `NiParticle/run_00c32d80` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31260` (unnamed, `thiscall`, no words): the base part of the constructor.
const BASE_CONSTRUCT: u32 = 0x00c3_1260;
/// The vtable this constructor stores (its RTTI names `NiPSysModifierFloatCtlr`).
const VTABLE_010C0E44: u32 = 0x010c_0e44;

// Translated from 00c32d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX`, no words): the `NiPSysModifierFloatCtlr`
/// constructor (by its vtable's RTTI): the base part
/// (`00c31260`), then the vtable `010c0e44`. Answers `this`. (`00c2b330`, the
/// creation function that calls it, stores the `NiPSysEmitterSpeedCtlr`
/// vtable over it.)
pub fn fn_00c32d80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_010C0E44);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_2d80, fn_00c32d80(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c32d80_runs_the_base_and_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_2d80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010C0E44);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT, vec![this])));
    }
}
