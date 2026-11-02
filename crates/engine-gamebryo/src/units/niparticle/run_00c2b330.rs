//! `NiParticle/run_00c2b330` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00c32d80` (`thiscall`, no words): the `NiPSysModifierFloatCtlr` constructor.
const FLOAT_CTLR_CONSTRUCTOR: u32 = 0x00c3_2d80;
/// The vtable stored over the constructed object (its RTTI names
/// `NiPSysEmitterSpeedCtlr`).
const EMITTER_SPEED_CTLR_VTABLE: u32 = 0x010b_f654;

// Translated from 00c2b330 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiPSysEmitterSpeedCtlr`: a 0x40-byte object from `00aa13e0`,
/// constructed by `00c32d80` and given the vtable `010bf654`; null when the allocation
/// fails.
pub fn fn_00c2b330(e: &mut Engine) -> Ptr {
    let block = e.call(NI_NEW, &args![0x40u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(FLOAT_CTLR_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, EMITTER_SPEED_CTLR_VTABLE);
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_b330, fn_00c2b330() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c2b330_builds_the_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FLOAT_CTLR_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x0101_0101);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_b330, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), EMITTER_SPEED_CTLR_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(NI_NEW, vec![0x40])));
        assert!(log.contains(&(FLOAT_CTLR_CONSTRUCTOR, vec![object])));

        e.register(NI_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_b330, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|call| call.0 == FLOAT_CTLR_CONSTRUCTOR));
    }
}
