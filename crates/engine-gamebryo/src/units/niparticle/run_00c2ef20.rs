//! `NiParticle/run_00c2ef20` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c2ef20` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysAgeDeathModifier` (the RTTI name of
//! the vtable `010c0304` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysagedeathmodifier::NiPSysAgeDeathModifier;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor `00c31720` (`thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysAgeDeathModifier`'s vtable (`010c0304`, RTTI `.?AVNiPSysAgeDeathModifier@@`).
const VTABLE: u32 = 0x010c_0304;

// Translated from 00c2ef20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x20 bytes, and when the
/// allocation is not null runs the `NiPSysModifier` constructor on it, stores
/// `NiPSysAgeDeathModifier`'s vtable, clears `m_bSpawnOnDeath` (+0x18) and
/// `m_pkSpawnModifier` (+0x1c). Answers the object, or 0 when the allocation failed.
pub fn fn_00c2ef20(e: &mut Engine) -> Ptr<NiPSysAgeDeathModifier> {
    let object = Ptr::<NiPSysAgeDeathModifier>::new(e.call(NI_NEW, &args![0x20u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.mem.set_u32(object.addr(), VTABLE);
        e.set(object, NiPSysAgeDeathModifier::m_bSpawnOnDeath, 0);
        e.set(object, NiPSysAgeDeathModifier::m_pkSpawnModifier, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_ef20, fn_00c2ef20() -> Ptr<NiPSysAgeDeathModifier>)]
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

    /// An engine whose allocation hands out blocks filled with 0xcd (so every store shows)
    /// and whose base constructor answers its object.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x20]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00c2ef20_builds_the_modifier_or_answers_null() {
        let mut e = engine();
        let object = e.call(0x00c2_ef20, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x20]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x18), 0xcdcd_cd00);
        assert_eq!(e.mem.u32(object + 0x1c), 0);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_ef20, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
