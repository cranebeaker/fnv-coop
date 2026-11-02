//! `NiMain/run_00a9b150` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor the object gets (`00a80540`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a8_0540;
/// The vtable stored at `+0` (RTTI class `NiParticles`, read through its complete object locator).
const VTABLE: u32 = 0x010a_11d4;
/// The object's size.
const SIZE: u32 = 0xc4;

// Translated from 00a9b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The create function of `NiParticles` (Ghidra made no function of it; a data reference from
/// `00a89da3`, `00a89f44` and `00a89f9e` points at it; `cdecl`, no words): allocates 0xc4 bytes (`00aa13e0`), null
/// when that fails. Otherwise runs the base constructor `00a80540` on the block, stores
/// the `NiParticles` vtable and returns the block.
pub fn fn_00a9b150(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_b150, fn_00a9b150() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a9b150_constructs_the_object_or_returns_null() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xdead_beef);
            }
            block.into_ret()
        });
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_b150, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|c| c.0 == NI_ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, vec![SIZE]);
        let constructs: Vec<_> = log.iter().filter(|c| c.0 == BASE_CONSTRUCT).collect();
        assert_eq!(constructs.len(), 1);
        assert_eq!(constructs[0].1, vec![object]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0xdead_beef);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_b150, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != BASE_CONSTRUCT));
    }
}
