//! `NiParticle/run_00c32b60` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c31260`: the `NiPSysModifierCtlr` constructor (`thiscall`).
const MODIFIER_CTLR_CONSTRUCTOR: u32 = 0x00c3_1260;
/// The vtable of `NiPSysModifierBoolCtlr` (RTTI
/// `.?AVNiPSysModifierBoolCtlr@@`).
const VTABLE: u32 = 0x010c_0d2c;

// Translated from 00c32b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifierBoolCtlr` constructor (by
/// the vtable it stores, `010c0d2c`): the `NiPSysModifierCtlr` constructor,
/// then the vtable. Returns `this`.
pub fn fn_00c32b60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MODIFIER_CTLR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_2b60, fn_00c32b60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_runs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(MODIFIER_CTLR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_2b60, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == MODIFIER_CTLR_CONSTRUCTOR && c.1 == vec![this]));
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
