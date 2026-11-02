//! `NiSystem/run_00aa24a0` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable both functions store (`010a2554`; its RTTI type descriptor reads
/// `.?AVNiSearchPath@@`).
const NI_SEARCH_PATH_VTABLE: u32 = 0x010a_2554;

// Translated from 00aa24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa24a0`): the `NiSearchPath` constructor body (the vtable
/// `010a2554` is `NiSearchPath`'s by its RTTI), `thiscall`, no stack words. Stores the vtable,
/// clears `m_uiNextPath` (+0x4, Xbox PDB) and the first bytes of `m_acFilePath` (+0x8) and
/// `m_acReferencePath` (+0x10c); answers `this`.
pub fn fn_00aa24d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, NI_SEARCH_PATH_VTABLE);
    e.mem.set_u32(base + 0x4, 0);
    e.mem.set_u8(base + 0x8, 0);
    e.mem.set_u8(base + 0x10c, 0);
    this
}

// Translated from 00aa24f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa24a0`): the `NiSearchPath` destructor body (`thiscall`,
/// no stack words): stores the class's vtable `010a2554` and nothing else.
pub fn fn_00aa24f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_SEARCH_PATH_VTABLE);
}

/// The `NiAllocator` vtable (`010a24d0`; its RTTI type descriptor reads
/// `.?AVNiAllocator@@`).
const NI_ALLOCATOR_VTABLE: u32 = 0x010a_24d0;
/// The memory manager object (`011f6238`), the `this` of its deallocate.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa4060` (`thiscall` on the memory manager, one word): its deallocate.
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00aa24a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00aa24a0`): the `NiAllocator` scalar
/// deleting destructor (the vtable `010a24d0` is `NiAllocator`'s by its
/// RTTI), `thiscall`, `RET 4` (the flags). Stores the vtable and, with bit 0
/// of the flags, gives `this` back to the memory manager (`00aa4060`).
/// Answers `this`.
pub fn fn_00aa24a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_ALLOCATOR_VTABLE);
    if flags & 1 != 0 {
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, this],
        );
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_24d0, fn_00aa24d0(Ptr) -> Ptr),
        entry!(0x00aa_24f0, fn_00aa24f0(Ptr)),
        entry!(0x00aa_24a0, fn_00aa24a0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00aa24d0_sets_the_vtable_and_clears_the_paths() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        for offset in [0x4u32, 0x8, 0x9, 0x10c, 0x10d] {
            e.mem.set_u8(this + offset, 0x55);
        }
        assert_eq!(e.call(0x00aa_24d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SEARCH_PATH_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 8), 0);
        assert_eq!(e.mem.u8(this + 9), 0x55);
        assert_eq!(e.mem.u8(this + 0x10c), 0);
        assert_eq!(e.mem.u8(this + 0x10d), 0x55);
    }

    #[test]
    fn fn_00aa24f0_only_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x210);
        e.mem.set_u32(this, 0x1234);
        e.mem.set_u32(this + 4, 7);
        e.call(0x00aa_24f0, &args![this]);
        assert_eq!(e.mem.u32(this), NI_SEARCH_PATH_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 7);
    }
}

#[cfg(test)]
mod tests_n084 {
    use super::*;

    #[test]
    fn fn_00aa24a0_frees_only_with_bit_0() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_24a0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_ALLOCATOR_VTABLE);
        assert_eq!(e.call(0x00aa_24a0, &args![this, 3u32]).u32(), this);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MEMORY_MANAGER_DEALLOCATE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(frees, vec![vec![MEMORY_MANAGER_OBJECT, this]]);
    }
}
