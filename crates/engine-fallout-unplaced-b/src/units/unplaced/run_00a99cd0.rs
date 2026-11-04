//! `(unplaced)/run_00a99cd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00a99cd0` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiIntegerExtraData` (the RTTI name of the
//! vtable `010a0d24` it stores).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base constructor `00a7b2e0` (unit `niextradata.cpp`, unnamed in the engine map;
/// `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a7_b2e0;
/// `NiIntegerExtraData`'s vtable (`010a0d24`, RTTI `.?AVNiIntegerExtraData@@`).
const VTABLE: u32 = 0x010a_0d24;

// Translated from 00a99cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0x10 bytes, and when the
/// allocation is not null runs the base constructor `00a7b2e0` on it, stores
/// `NiIntegerExtraData`'s vtable and clears `m_iValue` (+0x0c). Answers the object, or 0
/// when the allocation failed.
pub fn fn_00a99cd0(e: &mut Engine) -> Ptr {
    let object = Ptr::new(e.call(NI_NEW, &args![0x10u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.mem.set_u32(object.addr(), VTABLE);
        // NiIntegerExtraData::m_iValue (Xbox PDB) +0x0C
        e.mem.set_u32(object.addr() + 0x0c, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9cd0, fn_00a99cd0() -> Ptr)]
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
    fn fn_00a99cd0_builds_the_extra_data_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0x10]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_9cd0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 8), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x0c), 0);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_9cd0, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
