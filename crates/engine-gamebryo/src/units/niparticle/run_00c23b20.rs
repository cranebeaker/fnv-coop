//! `NiParticle/run_00c23b20` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c23b20` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysSpawnModifier` (the RTTI name of the
//! vtable `010be124` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysspawnmodifier::NiPSysSpawnModifier;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiPSysModifier` constructor `00c31720` (`thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00c3_1720;
/// `NiPSysSpawnModifier`'s vtable (`010be124`, RTTI `.?AVNiPSysSpawnModifier@@`).
const VTABLE: u32 = 0x010b_e124;
/// The bits of `1.0f` (`FLD1`, `FSTP dword`).
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00c23b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x34 bytes, and when the
/// allocation is not null runs the `NiPSysModifier` constructor on it, then sets
/// `m_fPercentageSpawned` to 1.0, `m_fSpawnSpeedChaos`, `m_fSpawnDirChaos`, `m_fLifeSpan`
/// and `m_fLifeSpanVar` to 0.0, `m_usNumSpawnGenerations`, `m_usMinNumToSpawn` and
/// `m_usMaxNumToSpawn` to 1, and stores `NiPSysSpawnModifier`'s vtable. Answers the object,
/// or 0 when the allocation failed.
pub fn fn_00c23b20(e: &mut Engine) -> Ptr<NiPSysSpawnModifier> {
    let object = Ptr::<NiPSysSpawnModifier>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.set(object, NiPSysSpawnModifier::m_fPercentageSpawned, ONE_BITS);
        e.set(object, NiPSysSpawnModifier::m_fSpawnSpeedChaos, 0);
        e.set(object, NiPSysSpawnModifier::m_fSpawnDirChaos, 0);
        e.set(object, NiPSysSpawnModifier::m_usNumSpawnGenerations, 1);
        e.set(object, NiPSysSpawnModifier::m_fLifeSpan, 0);
        e.mem.set_u32(object.addr(), VTABLE);
        e.set(object, NiPSysSpawnModifier::m_fLifeSpanVar, 0);
        e.set(object, NiPSysSpawnModifier::m_usMinNumToSpawn, 1);
        e.set(object, NiPSysSpawnModifier::m_usMaxNumToSpawn, 1);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_3b20, fn_00c23b20() -> Ptr<NiPSysSpawnModifier>)]
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
    fn fn_00c23b20_builds_the_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x34]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_3b20, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x18), 0xcdcd_0001);
        assert_eq!(e.mem.u32(object + 0x1c), ONE_BITS);
        assert_eq!(e.mem.u32(object + 0x20), 0x0001_0001);
        for offset in [0x24u32, 0x28, 0x2c, 0x30] {
            assert_eq!(e.mem.u32(object + offset), 0, "{offset:#x}");
        }

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_3b20, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
