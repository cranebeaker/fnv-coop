//! `NiParticle/run_00c29d00` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00c29d00` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiPSysModifierActiveCtlr` (the RTTI name of
//! the vtable `010bebbc` it stores).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base constructor `00c32b60` (unnamed in the engine map; `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00c3_2b60;
/// `NiPSysModifierActiveCtlr`'s vtable (`010bebbc`, RTTI `.?AVNiPSysModifierActiveCtlr@@`).
const VTABLE: u32 = 0x010b_ebbc;

// Translated from 00c29d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x40 bytes, and when the
/// allocation is not null runs the base constructor `00c32b60` on it and stores
/// `NiPSysModifierActiveCtlr`'s vtable. Answers the object, or 0 when the allocation failed.
pub fn fn_00c29d00(e: &mut Engine) -> Ptr {
    let object = Ptr::new(e.call(NI_NEW, &args![0x40u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.mem.set_u32(object.addr(), VTABLE);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c2_9d00, fn_00c29d00() -> Ptr)]
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
    fn fn_00c29d00_builds_the_controller_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x40]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00c2_9d00, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x3c), 0xcdcd_cdcd);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_9d00, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
