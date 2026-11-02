//! `NiParticle/run_00c270f0` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c270f0` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysUpdateCtlr` (the RTTI name of the
//! vtable `010be6b4` it stores).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nitimecontroller::NiTimeController;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The `NiTimeController` constructor `00a6d220` (`thiscall`, no stack words).
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
/// `NiPSysUpdateCtlr`'s vtable (`010be6b4`, RTTI `.?AVNiPSysUpdateCtlr@@`).
const VTABLE: u32 = 0x010b_e6b4;

// Translated from 00c270f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x34 bytes, and when the
/// allocation is not null runs the `NiTimeController` constructor on it, stores
/// `NiPSysUpdateCtlr`'s vtable and 0.0 in `m_fHiKeyTime` (+0x18) and `m_fLoKeyTime`
/// (+0x14). Answers the object, or 0 when the allocation failed.
pub fn fn_00c270f0(e: &mut Engine) -> Ptr<NiTimeController> {
    let object = Ptr::<NiTimeController>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if !object.is_null() {
        e.call(TIME_CONTROLLER_CONSTRUCT, &args![object]);
        e.set(object, NiTimeController::m_fHiKeyTime, 0);
        e.mem.set_u32(object.addr(), VTABLE);
        e.set(object, NiTimeController::m_fLoKeyTime, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_70f0, fn_00c270f0() -> Ptr<NiTimeController>)]
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
    fn fn_00c270f0_builds_the_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x34]);
            block.into_ret()
        });
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_70f0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x14), 0);
        assert_eq!(e.mem.u32(object + 0x18), 0);
        assert_eq!(e.mem.u32(object + 0x1c), 0xcdcd_cdcd);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_70f0, &args![]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT).is_empty());
    }
}
