//! `(unplaced)/run_00aa0250` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTMapBase` (Xbox PDB): bucket count +4, bucket array +8, item count
/// +0xC; items are next +0, key +4; the map frees an item through vtable
/// +8.
const HASH_SIZE: u32 = 0x4;
const HASH_TABLE: u32 = 0x8;
const COUNT: u32 = 0xc;
const DELETE_ITEM_SLOT: u32 = 0x8;

// Translated from 00aa0250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (an `NiTMapBase::RemoveAt` shape): finds the
/// item whose key is `*key` in its bucket (`key % bucket count`), unlinks
/// it, frees it (vtable +8) and counts it off; answers whether it was
/// there.
pub fn fn_00aa0250(e: &mut Engine, this: Ptr, key: Ptr) -> bool {
    let wanted = e.mem.u32(key.addr());
    let buckets = e.mem.u32(this.addr().wrapping_add(HASH_SIZE));
    // `DIV`: a bucket count of 0 is a divide fault in the exe.
    let index = wanted % buckets;
    let table = e.mem.u32(this.addr().wrapping_add(HASH_TABLE));
    let mut link = table.wrapping_add(index * 4);
    let mut item = e.mem.u32(link);
    while item != 0 {
        if e.mem.u32(item + 4) == wanted {
            let next = e.mem.u32(item);
            e.mem.set_u32(link, next);
            e.vcall(this.addr(), DELETE_ITEM_SLOT, &args![item]);
            let count = e.mem.u32(this.addr() + COUNT).wrapping_sub(1);
            e.mem.set_u32(this.addr() + COUNT, count);
            return true;
        }
        link = item;
        item = e.mem.u32(item);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa0250, fn_00aa0250(Ptr, Ptr) -> bool),
        entry!(0x00aa_02b0, fn_00aa02b0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELETE_ITEM: u32 = 0x7200_0008;

    #[test]
    fn removes_the_item_with_the_key() {
        let mut e = Engine::new();
        e.put_vtable(0x7000_0000, &[0, 0, DELETE_ITEM]);
        e.register(DELETE_ITEM, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        let table = e.mem.alloc(0xc);
        e.mem.set_u32(map, 0x7000_0000);
        e.mem.set_u32(map + 4, 3);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u32(map + 0xc, 2);
        // Bucket 1 holds keys 4 then 7.
        let second = e.mem.alloc(8);
        e.mem.set_u32(second + 4, 7);
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 4);
        e.mem.set_u32(table + 4, first);
        let key = e.mem.alloc(4);
        e.mem.set_u32(key, 7);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa0250, &args![map, key]).bool());
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().find(|c| c.0 == DELETE_ITEM).unwrap().1,
            [map, second]
        );
        // The head, then a key not there.
        e.mem.set_u32(key, 4);
        assert!(e.call(0x00aa0250, &args![map, key]).bool());
        assert_eq!(e.mem.u32(table + 4), 0);
        e.mem.set_u32(key, 1);
        assert!(!e.call(0x00aa0250, &args![map, key]).bool());
        assert_eq!(e.mem.u32(map + 0xc), 0);
    }
}

/// The vtable of `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString,
/// NiGlobalConstantEntry*>>, NiGlobalConstantEntry*>` (its RTTI names the
/// class).
const GLOBAL_CONSTANT_MAP_VTABLE: u32 = 0x010a_2138;
/// The map's `RemoveAll` (`00aa01f0`; the engine map names it for the
/// `NiAVObject*` map, whose code it shares).
const STRING_MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00aa02b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `NiTFixedStringMapBase<DFALL<NiTMapItem<
/// NiFixedString, NiGlobalConstantEntry*>>, NiGlobalConstantEntry*>` (slot 0
/// of its vtable `010a2138`, whose RTTI names the class): sets that vtable,
/// empties the map (`00aa01f0`), frees the bucket array (+8) through
/// `00aa10f0`, and with bit 0 of `flags` frees the object through the
/// memory manager. Returns `this`.
pub fn fn_00aa02b0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, GLOBAL_CONSTANT_MAP_VTABLE);
    e.call(STRING_MAP_REMOVE_ALL, &[this]);
    let buckets = e.mem.u32(this + HASH_TABLE);
    e.call(NI_FREE, &[buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, this]);
    }
    this
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn global_constant_map_destructor() {
        for (flags, deleted) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            for addr in [STRING_MAP_REMOVE_ALL, NI_FREE, MEMORY_FREE] {
                e.register(addr, |_, _| 0u32.into_ret());
            }
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00aa_02b0, &[this, flags]).u32(), this);
            let calls: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![(STRING_MAP_REMOVE_ALL, vec![this]), (NI_FREE, vec![0x7000])];
            if deleted {
                expected.push((MEMORY_FREE, vec![MEMORY_MANAGER, this]));
            }
            assert_eq!(calls, expected);
            assert_eq!(e.mem.u32(this), GLOBAL_CONSTANT_MAP_VTABLE);
        }
    }
}
