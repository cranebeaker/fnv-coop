//! `fallout shared/region system/run_004f15a0` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The vtable of `TESRegionDataGrass` (RTTI `.?AVTESRegionDataGrass@@`, read at the word before it).
const TES_REGION_DATA_GRASS_VTABLE: u32 = 0x0102_3b24;
/// `MemoryManager` allocation (`00401000`, cdecl, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `004f5db0`: the constructor of the 0x14-byte object (RTTI vtable
/// `.?AVTESRegionGrassObjectList@@`) with its byte argument.
const GRASS_OBJECT_LIST_CTOR: u32 = 0x004f_5db0;

// Translated from 004f15c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the `TESRegionData`
/// priority setter. A `value` above 100 (compared as the zero-extended byte)
/// is refused (returns false); otherwise it is stored at +6 (`cPriority`,
/// Xbox PDB) and true is returned.
pub fn fn_004f15c0(e: &mut Engine, this: Ptr, value: u8) -> bool {
    if i32::from(value) > 0x64 {
        return false;
    }
    e.mem.set_u8(this.addr() + 6, value);
    true
}

// Translated from 004f15f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f15f0 - its FS:[0] exception frame (the unwind states that destroy the partly built object when a constructor throws) is not translated
/// `TESRegionDataGrass` constructor (unnamed in the engine map; the vtable's
/// RTTI names the class): runs the `TESRegionData` constructor, stores the
/// `TESRegionDataGrass` vtable, allocates 0x14 bytes and, when that
/// succeeds, constructs the grass object list in them with the argument 1
/// (`004f5db0`); the result (or null) goes to +8 (`pObjects`, Xbox PDB).
/// Returns `this`.
pub fn fn_004f15f0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_GRASS_VTABLE);
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let list = if block != 0 {
        e.call(GRASS_OBJECT_LIST_CTOR, &args![block, 1u32]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr() + 8, list);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_15c0, fn_004f15c0(Ptr, u8) -> bool),
        entry!(0x004f_15f0, fn_004f15f0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f15c0_stores_priorities_up_to_100() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert!(e.call(0x004f_15c0, &args![this, 0xffff_ff64u32]).bool());
        assert_eq!(e.mem.u8(this + 6), 100);
        assert!(!e.call(0x004f_15c0, &args![this, 0x65u32]).bool());
        assert!(!e.call(0x004f_15c0, &args![this, 0xffu32]).bool());
        assert_eq!(e.mem.u8(this + 6), 100);
        assert!(e.call(0x004f_15c0, &args![this, 0x100u32]).bool());
        assert_eq!(e.mem.u8(this + 6), 0);
        assert_eq!(e.mem.u8(this + 5), 0);
    }

    #[test]
    fn at_004f15f0_builds_the_object_list_when_allocated() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_CTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |e, _| e.mem.alloc(0x14).into_ret());
        e.register(GRASS_OBJECT_LIST_CTOR, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_15f0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_GRASS_VTABLE);
        let list = e.mem.u32(this + 8);
        assert_ne!(list, 0);
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| *a != 0x004f_15f0)
            .collect();
        assert_eq!(
            order,
            vec![TES_REGION_DATA_CTOR, ALLOCATE, GRASS_OBJECT_LIST_CTOR]
        );
        let ctor: Vec<_> = log
            .iter()
            .filter(|c| c.0 == GRASS_OBJECT_LIST_CTOR)
            .collect();
        assert_eq!(ctor[0].1, vec![list, 1]);
    }

    #[test]
    fn at_004f15f0_stores_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_CTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xdead);
        e.call_log = Some(vec![]);
        e.call(0x004f_15f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(log.iter().all(|c| c.0 != GRASS_OBJECT_LIST_CTOR));
    }
}
