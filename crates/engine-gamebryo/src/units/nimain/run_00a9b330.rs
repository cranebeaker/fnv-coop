//! `NiMain/run_00a9b330` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00a9b330` is a function Ghidra never created (research/engine-map/ghidra_gaps.tsv),
//! read from its bytes: the object factory of `NiAmbientLight` (the RTTI name of the vtable
//! `010a12cc` it stores).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The base constructor `00a71f60` (unit `nilight.cpp`, unnamed in the engine map; it
/// stores `NiLight`'s vtable; `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a7_1f60;
/// `NiAmbientLight`'s vtable (`010a12cc`, RTTI `.?AVNiAmbientLight@@`).
const VTABLE: u32 = 0x010a_12cc;

// Translated from 00a9b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `cdecl`, no arguments): allocates 0xf0 bytes, and when the
/// allocation is not null runs the base constructor `00a71f60` on it, stores
/// `NiAmbientLight`'s vtable and clears the byte at +0x9d (a base-class field on PC, not
/// identified here). Answers the object, or 0 when the allocation failed.
pub fn fn_00a9b330(e: &mut Engine) -> Ptr {
    let object = Ptr::new(e.call(NI_NEW, &args![0xf0u32]).u32());
    if !object.is_null() {
        e.call(BASE_CONSTRUCT, &args![object]);
        e.mem.set_u32(object.addr(), VTABLE);
        e.mem.set_u8(object.addr() + 0x9d, 0);
    }
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_b330, fn_00a9b330() -> Ptr)]
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
    fn fn_00a9b330_builds_the_light_or_answers_null() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xcd; 0xf0]);
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_b330, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0xf0]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 0x9c), 0xcdcd_00cd);

        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b330, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }
}
