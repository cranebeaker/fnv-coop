//! `NiParticle/run_00c2a2b0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c2a2b0` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysGrowFadeModifier` (the RTTI name of the
//! vtable `010bf0bc` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysgrowfademodifier::NiPSysGrowFadeModifier;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor `00c31720` (`thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysGrowFadeModifier`'s vtable (`010bf0bc`, RTTI `.?AVNiPSysGrowFadeModifier@@`).
const VTABLE: u32 = 0x010b_f0bc;

// Translated from 00c2a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x2c bytes, and when the
/// allocation is not null runs the `NiPSysModifier` constructor on it, then sets
/// `m_fGrowTime`, `m_fFadeTime` and `m_fBaseScale` to 0.0 and `m_usGrowGeneration` and
/// `m_usFadeGeneration` to 0, and stores `NiPSysGrowFadeModifier`'s vtable. Answers the
/// object, or 0 when the allocation failed.
pub fn fn_00c2a2b0(e: &mut Engine) -> Ptr<NiPSysGrowFadeModifier> {
    let object = Ptr::<NiPSysGrowFadeModifier>::new(e.call(NI_NEW, &args![0x2cu32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.set(object, NiPSysGrowFadeModifier::m_fGrowTime, 0);
        e.set(object, NiPSysGrowFadeModifier::m_fFadeTime, 0);
        e.set(object, NiPSysGrowFadeModifier::m_usGrowGeneration, 0);
        e.set(object, NiPSysGrowFadeModifier::m_fBaseScale, 0);
        e.mem.set_u32(object.addr(), VTABLE);
        e.set(object, NiPSysGrowFadeModifier::m_usFadeGeneration, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_a2b0, fn_00c2a2b0() -> Ptr<NiPSysGrowFadeModifier>)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00c2a2b0_builds_the_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x2c]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_a2b0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x2c]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0xcdcd_0000);
        assert_eq!(e.mem.u32(object + 0x20), 0);
        assert_eq!(e.mem.u32(object + 0x24), 0xcdcd_0000);
        assert_eq!(e.mem.u32(object + 0x28), 0);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_a2b0, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
