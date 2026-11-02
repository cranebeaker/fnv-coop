//! `NiParticle/run_00c31260` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysmodifier::FIXED_STRING_NULL;
use crate::units::niparticle::nipsysmodifierctlr::NiPSysModifierCtlr;

/// `00a38e00`: the `NiSingleInterpController` constructor (`thiscall`, no
/// stack words; the engine map's name for it is folded code).
const SINGLE_INTERP_CONTROLLER_CONSTRUCTOR: u32 = 0x00a3_8e00;
/// The vtable of `NiPSysModifierCtlr` (RTTI `.?AVNiPSysModifierCtlr@@`).
const VTABLE: u32 = 0x010c_08ac;

// Translated from 00c31260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifierCtlr` constructor (by the
/// vtable it stores, `010c08ac`). The `NiSingleInterpController`
/// constructor, the vtable, the null modifier name handle (the word at
/// `0109b220`) and no modifier. Returns `this`.
pub fn fn_00c31260(e: &mut Engine, this: Ptr<NiPSysModifierCtlr>) -> Ptr<NiPSysModifierCtlr> {
    e.call(SINGLE_INTERP_CONTROLLER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let null = e.global::<u32>(FIXED_STRING_NULL);
    e.set(this, NiPSysModifierCtlr::m_kModifierName, null);
    e.set(this, NiPSysModifierCtlr::m_pkModifier, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c3_1260,
        fn_00c31260(Ptr<NiPSysModifierCtlr>) -> Ptr<NiPSysModifierCtlr>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtable_and_the_null_name() {
        let mut e = Engine::new();
        e.map(FIXED_STRING_NULL & !0xfff, 0x1000);
        e.set_global(FIXED_STRING_NULL, 0x77u32);
        e.register(SINGLE_INTERP_CONTROLLER_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifierCtlr>();
        e.set(this, NiPSysModifierCtlr::m_pkModifier, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_1260, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == SINGLE_INTERP_CONTROLLER_CONSTRUCTOR && c.1 == vec![this.addr()]));
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_kModifierName), 0x77);
        assert_eq!(e.get(this, NiPSysModifierCtlr::m_pkModifier), 0);
    }
}
