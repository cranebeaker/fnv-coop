//! `NiParticle/run_00c31640` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c31640` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv; Ghidra
//! holds its bytes in the body of `00c2dc70`), read from its bytes. It is reached from the
//! vtable of `NiPSysMeshEmitter` (`010bd898` + 4, slot 39) and 25 more, and by the jump that
//! ends `NiPSysBoundUpdateModifier::SetSystemPointer` (`00c2dc70`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::niparticle::nipsysmodifier::NiPSysModifier;

// Translated from 00c31640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `thiscall`, one stack word, `RET 4`): the `NiPSysModifier`
/// system-pointer setter: stores `system` in `m_pkTarget` (+0x10).
pub fn fn_00c31640(e: &mut Engine, this: Ptr<NiPSysModifier>, system: u32) {
    e.set(this, NiPSysModifier::m_pkTarget, system);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c3_1640, fn_00c31640(Ptr<NiPSysModifier>, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c31640_stores_the_system_in_the_target() {
        let mut e = Engine::new();
        let this = e.new_object::<NiPSysModifier>();
        e.mem.write(this.addr(), &[0xcd; 0x18]);
        e.call(0x00c3_1640, &args![this, 0x0e00_1000u32]);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0x0e00_1000);
        assert_eq!(e.mem.u32(this.addr() + 0x0c), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this.addr() + 0x14), 0xcdcd_cdcd);
    }
}
