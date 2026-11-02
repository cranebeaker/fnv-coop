//! `NiParticle/run_00c2d690` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c2d690` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysBoundUpdateModifier` (the RTTI name of
//! the vtable `010c0134` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysboundupdatemodifier::NiPSysBoundUpdateModifier;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor `00c31720` (`thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysBoundUpdateModifier`'s vtable (`010c0134`, RTTI
/// `.?AVNiPSysBoundUpdateModifier@@`).
const VTABLE: u32 = 0x010c_0134;

// Translated from 00c2d690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x20 bytes, and when the
/// allocation is not null runs the `NiPSysModifier` constructor on it, clears
/// `m_sUpdateSkip` (+0x18), `m_pkSkipBounds` (+0x1c) and `m_usUpdateCount` (+0x1a) and
/// stores `NiPSysBoundUpdateModifier`'s vtable. Answers the object, or 0 when the
/// allocation failed.
pub fn fn_00c2d690(e: &mut Engine) -> Ptr<NiPSysBoundUpdateModifier> {
    let object = Ptr::<NiPSysBoundUpdateModifier>::new(e.call(NI_NEW, &args![0x20u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.set(object, NiPSysBoundUpdateModifier::m_sUpdateSkip, 0);
        e.set(object, NiPSysBoundUpdateModifier::m_pkSkipBounds, 0);
        e.mem.set_u32(object.addr(), VTABLE);
        e.set(object, NiPSysBoundUpdateModifier::m_usUpdateCount, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_d690, fn_00c2d690() -> Ptr<NiPSysBoundUpdateModifier>)]
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
    fn fn_00c2d690_builds_the_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x20]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_d690, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x20]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_d690, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
