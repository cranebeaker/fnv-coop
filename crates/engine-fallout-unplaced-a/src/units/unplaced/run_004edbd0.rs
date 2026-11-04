//! `(unplaced)/run_004edbd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtables of `NiTPointerMap<TESObjectREFR *, WadingWaterData *>`, its base
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, TESObjectREFR *, ...>` and
/// `NiTArray<BSTextureManager::eTextureType, NiTMallocInterface<...>>` (classes confirmed by
/// the RTTI before them), the map's `00438af0` (`thiscall`, no arguments), the table free
/// `00aa10f0` and the array free `004ede70` (`cdecl`, `(block)`).
const POINTER_MAP_VTABLE: u32 = 0x0102_3234;
const MAP_BASE_VTABLE: u32 = 0x0102_3294;
const TEXTURE_TYPE_ARRAY_VTABLE: u32 = 0x0102_32b4;
const MAP_CLEAR_00438AF0: u32 = 0x0043_8af0;
const TABLE_FREE: u32 = 0x00aa_10f0;
const ARRAY_FREE_004EDE70: u32 = 0x004e_de70;

// Translated from 004edc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<NiTPointerAllocator<unsigned int>, TESObjectREFR *, ...>` destructor body
/// (unnamed in the map): the class's vtable, `00438af0` on the map, and its bucket table
/// (`+8`) freed (`00aa10f0`).
pub fn fn_004edc30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.call(MAP_CLEAR_00438AF0, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(TABLE_FREE, &args![table]);
}

// Translated from 004edbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPointerMap<TESObjectREFR *, WadingWaterData *>` destructor body (unnamed in the
/// map): the class's vtable, `00438af0` on the map, then the base's destructor body
/// (`fn_004edc30`).
// Translation incomplete: 004edbd0 - SEH exception unwinding is unsupported.
pub fn fn_004edbd0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_CLEAR_00438AF0, &args![this]);
    fn_004edc30(e, this);
}

// Translated from 004edc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<BSTextureManager::eTextureType, NiTMallocInterface<...>>` destructor body
/// (unnamed in the map): the class's vtable and the element block (`+4`) freed
/// (`004ede70`).
pub fn fn_004edc60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TEXTURE_TYPE_ARRAY_VTABLE);
    let elements = e.mem.u32(this.addr() + 4);
    e.call(ARRAY_FREE_004EDE70, &args![elements]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004edc30, fn_004edc30(Ptr)),
        entry!(0x004edbd0, fn_004edbd0(Ptr)),
        entry!(0x004edc60, fn_004edc60(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn map_destructor_bodies_clear_and_free() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MAP_CLEAR_00438AF0, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call(0x004edc30, &args![this]);
        assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        assert_eq!(calls_at(&e, TABLE_FREE), vec![vec![0x7000]]);
        e.call(0x004edbd0, &args![this]);
        assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        assert_eq!(calls_at(&e, MAP_CLEAR_00438AF0), vec![vec![this]; 3]);
        assert_eq!(calls_at(&e, TABLE_FREE).len(), 2);
    }

    #[test]
    fn texture_type_array_destructor_frees_the_elements() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_FREE_004EDE70, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7100);
        e.call(0x004edc60, &args![this]);
        assert_eq!(e.mem.u32(this), TEXTURE_TYPE_ARRAY_VTABLE);
        assert_eq!(calls_at(&e, ARRAY_FREE_004EDE70), vec![vec![0x7100]]);
    }
}
