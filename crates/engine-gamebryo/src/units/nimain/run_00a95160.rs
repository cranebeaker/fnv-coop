//! `NiMain/run_00a95160` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, one word: the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The base constructor the object gets (`00a5c200`, `thiscall`, no stack words).
const BASE_CONSTRUCT: u32 = 0x00a5_c200;
/// The vtable stored at `+0` (RTTI class `NiSpecularProperty`, read through its complete object locator).
const VTABLE: u32 = 0x010a_02e4;
/// The object's size.
const SIZE: u32 = 0x1c;

// Translated from 00a95160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The create function of `NiSpecularProperty` (Ghidra made no function of it; a data reference from
/// `00a8a076` points at it; `cdecl`, no words): allocates 0x1c bytes (`00aa13e0`), null
/// when that fails. Otherwise runs the base constructor `00a5c200` on the block, clears the 16-bit word at
/// `+0x18`, stores
/// the `NiSpecularProperty` vtable and returns the block.
pub fn fn_00a95160(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BASE_CONSTRUCT, &args![object]);
    e.mem.set_u16(object + 0x18, 0);
    e.mem.set_u32(object, VTABLE);
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_5160, fn_00a95160() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a95160_constructs_the_object_or_returns_null() {
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
        let object = e.call(0x00a9_5160, &args![]).u32();
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|c| c.0 == NI_ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, vec![SIZE]);
        let constructs: Vec<_> = log.iter().filter(|c| c.0 == BASE_CONSTRUCT).collect();
        assert_eq!(constructs.len(), 1);
        assert_eq!(constructs[0].1, vec![object]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u16(object + 0x18), 0);
        assert_eq!(e.mem.u16(object + 0x1a), 0xdead);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_5160, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != BASE_CONSTRUCT));
    }
}
