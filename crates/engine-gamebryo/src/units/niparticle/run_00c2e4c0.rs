//! `NiParticle/run_00c2e4c0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::niparticle::nipsysbombmodifier::NiPSysBombModifier;

/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`thiscall`, no words).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
/// The vtable stored over the constructed object (its RTTI names `NiPSysBombModifier`).
const BOMB_MODIFIER_VTABLE: u32 = 0x010c_01ec;
/// The three words at `011a946c` the bomb axis is copied from (`MOV`).
const DEFAULT_AXIS: u32 = 0x011a_946c;

// Translated from 00c2e4c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiPSysBombModifier`: a 0x38-byte object from `00aa13e0`, constructed
/// by `NiPSysModifier`'s constructor (`00c31720`), given the vtable `010c01ec`, no bomb object
/// (+0x18), the axis (+0x1c) copied word by word from `011a946c`, decay (+0x28) and delta-v
/// (+0x2c) 0.0 (`FLDZ`, `FST`, `FSTP`) and both types (+0x30, +0x34) 0; null when the
/// allocation fails.
pub fn fn_00c2e4c0(e: &mut Engine) -> Ptr<NiPSysBombModifier> {
    let block = e.call(NI_NEW, &args![0x38u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(MODIFIER_CONSTRUCTOR, &args![block]);
    let this = Ptr::<NiPSysBombModifier>::new(block);
    e.mem.set_u32(block, BOMB_MODIFIER_VTABLE);
    e.set(this, NiPSysBombModifier::m_pkBombObj, 0);
    let axis_x = e.global::<u32>(DEFAULT_AXIS);
    e.mem.set_u32(block + 0x1c, axis_x);
    let axis_y = e.global::<u32>(DEFAULT_AXIS + 4);
    e.mem.set_u32(block + 0x20, axis_y);
    let axis_z = e.global::<u32>(DEFAULT_AXIS + 8);
    e.set(this, NiPSysBombModifier::m_fDecay, 0.0f32.to_bits());
    e.set(this, NiPSysBombModifier::m_fDeltaV, 0.0f32.to_bits());
    e.mem.set_u32(block + 0x24, axis_z);
    e.set(this, NiPSysBombModifier::m_eDecayType, 0);
    e.set(this, NiPSysBombModifier::m_eSymmType, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_e4c0, fn_00c2e4c0() -> Ptr<NiPSysBombModifier>)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c2e4c0_builds_the_modifier_or_answers_null() {
        let mut e = Engine::new();
        e.map(DEFAULT_AXIS, 12);
        e.mem.set_f32(DEFAULT_AXIS, 1.0);
        e.mem.set_f32(DEFAULT_AXIS + 4, 2.0);
        e.mem.set_f32(DEFAULT_AXIS + 8, 3.0);
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            block.into_ret()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_e4c0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), BOMB_MODIFIER_VTABLE);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.f32(object + 0x1c), 1.0);
        assert_eq!(e.mem.f32(object + 0x20), 2.0);
        assert_eq!(e.mem.f32(object + 0x24), 3.0);
        assert_eq!(e.mem.u32(object + 0x28), 0);
        assert_eq!(e.mem.u32(object + 0x2c), 0);
        assert_eq!(e.mem.u32(object + 0x30), 0);
        assert_eq!(e.mem.u32(object + 0x34), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_NEW, vec![0x38])));
        assert!(log.contains(&(MODIFIER_CONSTRUCTOR, vec![object])));

        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c2_e4c0, &args![]).u32(), 0);
    }
}
