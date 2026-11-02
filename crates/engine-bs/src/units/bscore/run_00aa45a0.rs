//! `BSCore/run_00aa45a0` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `MemoryManager` (the singleton at `011f6238`) heap lookups and the
//! constructor of its `BSMapBase<unsigned long, ScrapHeap*>`. The manager's
//! fields are read at their PC offsets; their order matches the Xbox PDB's
//! `iNumHeaps` (+0x02, `u16`), `iNumPhysicalHeaps` (+0x04, `u16`),
//! `ppHeaps` (+0x08) and `ppPhysicalHeaps` (+0x124), names not confirmed
//! for the PC build. Heaps are `IMemoryHeap` objects called through their
//! vtables by slot offset.

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager` (PC) +0x02: number of entries of the first heap array.
const HEAP_COUNT: u32 = 0x02;
/// `MemoryManager` (PC) +0x04: number of entries of the second heap array.
const SECOND_HEAP_COUNT: u32 = 0x04;
/// `MemoryManager` (PC) +0x08: the first heap array (`IMemoryHeap**`).
const HEAPS: u32 = 0x08;
/// `MemoryManager` (PC) +0x124: the second heap array (`IMemoryHeap**`).
const SECOND_HEAPS: u32 = 0x124;

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager`'s allocator (`this`, size).
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `_memset` (cdecl: destination, value, size).
const MEMSET: u32 = 0x00ec_61c0;
/// The vtable of `BSMapBase<unsigned long, ScrapHeap*>` (`.?AV?$BSMapBase@IPAVScrapHeap@@@@`
/// through its RTTI locator).
const SCRAP_HEAP_MAP_VTABLE: u32 = 0x010a_2618;

layout! {
    /// `BSMapBase<unsigned long,ScrapHeap *>` (Xbox PDB), 0x10 bytes.
    pub struct ScrapHeapMap: 0x10 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_uiHashSize` (Xbox PDB).
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB): `BSMapItem**`.
        0x08 m_ppkHashTable: u32,
        /// `m_uiCount` (Xbox PDB).
        0x0C m_uiCount: u32,
    }
}

fn count(e: &Engine, manager: Ptr, offset: u32) -> u32 {
    e.mem.u16(manager.addr().wrapping_add(offset)) as u32
}

fn array_entry(e: &Engine, manager: Ptr, offset: u32, index: u32) -> u32 {
    let array = e.mem.u32(manager.addr().wrapping_add(offset));
    e.mem.u32(array.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00aa45a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first heap of the first array whose vtable slot +0x10 (Xbox
/// `IMemoryHeap::PointerInHeap`) answers true for `pointer`, or null.
pub fn fn_00aa45a0(e: &mut Engine, this: Ptr, pointer: u32) -> Ptr {
    let mut index: u16 = 0;
    while (index as u32) < count(e, this, HEAP_COUNT) {
        let heap = array_entry(e, this, HEAPS, index as u32);
        if e.vcall(heap, 0x10, &args![pointer]).bool() {
            return Ptr::new(heap);
        }
        index = index.wrapping_add(1);
    }
    Ptr::NULL
}

// Translated from 00aa4610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first heap of the second array whose vtable slot +0x10
/// answers true for `pointer`, or null.
pub fn fn_00aa4610(e: &mut Engine, this: Ptr, pointer: u32) -> Ptr {
    let mut found = 0u32;
    let mut index: u16 = 0;
    while (index as u32) < count(e, this, SECOND_HEAP_COUNT) && found == 0 {
        let heap = array_entry(e, this, SECOND_HEAPS, index as u32);
        if e.vcall(heap, 0x10, &args![pointer]).bool() {
            found = array_entry(e, this, SECOND_HEAPS, index as u32);
        }
        index = index.wrapping_add(1);
    }
    Ptr::new(found)
}

// Translated from 00aa46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Heap by index over both arrays: indices below the first count select
/// the first array, the next ones the second; beyond both, null.
pub fn fn_00aa46a0(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    let first = count(e, this, HEAP_COUNT);
    if index < first {
        Ptr::new(array_entry(e, this, HEAPS, index))
    } else if index.wrapping_sub(first) < count(e, this, SECOND_HEAP_COUNT) {
        Ptr::new(array_entry(
            e,
            this,
            SECOND_HEAPS,
            index.wrapping_sub(first),
        ))
    } else {
        Ptr::NULL
    }
}

// Translated from 00aa4710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls vtable slot +0x18 of the heap at `index` (`00aa46a0`) with
/// (`stats`, `flag`); true when that heap exists.
pub fn fn_00aa4710(e: &mut Engine, this: Ptr, index: u32, flag: u8, stats: u32) -> bool {
    let heap = fn_00aa46a0(e, this, index);
    if heap.is_null() {
        return false;
    }
    e.vcall(heap.addr(), 0x18, &args![stats, flag as u32]);
    true
}

// Translated from 00aa4760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00aa4710` for index `index` of the second array (offset by the first
/// count). The decompiler shows no result, but `00aa4710`'s result stays in
/// `AL` and is returned.
pub fn fn_00aa4760(e: &mut Engine, this: Ptr, index: u32, flag: u8, stats: u32) -> bool {
    let first = count(e, this, HEAP_COUNT);
    fn_00aa4710(e, this, first.wrapping_add(index), flag, stats)
}

// Translated from 00aa4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00aa4710` with the same arguments (its result is returned in `AL`).
pub fn fn_00aa4790(e: &mut Engine, this: Ptr, index: u32, flag: u8, stats: u32) -> bool {
    fn_00aa4710(e, this, index, flag, stats)
}

// Translated from 00aa47c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0; its two stack words are never read.
pub fn fn_00aa47c0(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) -> u32 {
    0
}

// Translated from 00aa47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase<unsigned long, ScrapHeap*>` constructor (the vtable it
/// stores; no name in the engine map): sets the hash size, clears the
/// count and, for a non-zero size, allocates the bucket table from the
/// `MemoryManager` and zeroes it.
pub fn fn_00aa47e0(e: &mut Engine, this: Ptr<ScrapHeapMap>, hash_size: u32) -> Ptr<ScrapHeapMap> {
    e.set(this, ScrapHeapMap::vtable, SCRAP_HEAP_MAP_VTABLE);
    e.set(this, ScrapHeapMap::m_uiHashSize, hash_size);
    e.set(this, ScrapHeapMap::m_uiCount, 0);
    if hash_size != 0 {
        let bytes = e.get(this, ScrapHeapMap::m_uiHashSize).wrapping_shl(2);
        let table = e
            .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
            .u32();
        e.set(this, ScrapHeapMap::m_ppkHashTable, table);
        let bytes = e.get(this, ScrapHeapMap::m_uiHashSize).wrapping_shl(2);
        let table = e.get(this, ScrapHeapMap::m_ppkHashTable);
        e.call(MEMSET, &args![table, 0u32, bytes]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa45a0, fn_00aa45a0(Ptr, u32) -> Ptr),
        entry!(0x00aa4610, fn_00aa4610(Ptr, u32) -> Ptr),
        entry!(0x00aa46a0, fn_00aa46a0(Ptr, u32) -> Ptr),
        entry!(0x00aa4710, fn_00aa4710(Ptr, u32, u8, u32) -> bool),
        entry!(0x00aa4760, fn_00aa4760(Ptr, u32, u8, u32) -> bool),
        entry!(0x00aa4790, fn_00aa4790(Ptr, u32, u8, u32) -> bool),
        entry!(0x00aa47c0, fn_00aa47c0(Ptr, u32, u32) -> u32),
        entry!(
            0x00aa47e0,
            fn_00aa47e0(Ptr<ScrapHeapMap>, u32) -> Ptr<ScrapHeapMap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_VTABLE: u32 = 0x0300_0000;
    const IN_HEAP: u32 = 0x0300_1000;
    const STATS: u32 = 0x0300_1100;

    /// A manager with `first` heaps in the first array and `second` in the
    /// second; heap `n` is at `0x0400_0000 + n * 0x10`, holds `n` at +4 and
    /// claims pointers equal to `n`.
    fn manager(e: &mut Engine, first: u16, second: u16) -> Ptr {
        let m = Ptr::new(e.mem.alloc(0x130));
        e.mem.set_u16(m.addr() + HEAP_COUNT, first);
        e.mem.set_u16(m.addr() + SECOND_HEAP_COUNT, second);
        e.map(0x0400_0000, 0x1000);
        let a = e.mem.alloc(4 * first as u32 + 4);
        let b = e.mem.alloc(4 * second as u32 + 4);
        e.mem.set_u32(m.addr() + HEAPS, a);
        e.mem.set_u32(m.addr() + SECOND_HEAPS, b);
        for n in 0..(first as u32 + second as u32) {
            let heap = 0x0400_0000 + n * 0x10;
            e.mem.set_u32(heap, HEAP_VTABLE);
            e.mem.set_u32(heap + 4, n);
            let slot = if n < first as u32 {
                a + 4 * n
            } else {
                b + 4 * (n - first as u32)
            };
            e.mem.set_u32(slot, heap);
        }
        let mut slots = vec![0u32; 8];
        slots[4] = IN_HEAP;
        slots[6] = STATS;
        e.put_vtable(HEAP_VTABLE, &slots);
        e.register(IN_HEAP, |e, a| (e.mem.u32(a[0] + 4) == a[1]).into_ret());
        e.register(STATS, |_, _| Ret::default());
        m
    }

    #[test]
    fn find_in_first_array() {
        let mut e = Engine::new();
        let m = manager(&mut e, 3, 2);
        assert_eq!(e.call(0x00aa45a0, &args![m, 1u32]).u32(), 0x0400_0010);
        // Heap 3 is in the second array: not searched.
        assert_eq!(e.call(0x00aa45a0, &args![m, 3u32]).u32(), 0);
    }

    #[test]
    fn find_in_second_array() {
        let mut e = Engine::new();
        let m = manager(&mut e, 3, 2);
        assert_eq!(e.call(0x00aa4610, &args![m, 4u32]).u32(), 0x0400_0040);
        assert_eq!(e.call(0x00aa4610, &args![m, 1u32]).u32(), 0);
    }

    #[test]
    fn heap_by_index_spans_both_arrays() {
        let mut e = Engine::new();
        let m = manager(&mut e, 3, 2);
        assert_eq!(e.call(0x00aa46a0, &args![m, 2u32]).u32(), 0x0400_0020);
        assert_eq!(e.call(0x00aa46a0, &args![m, 4u32]).u32(), 0x0400_0040);
        assert_eq!(e.call(0x00aa46a0, &args![m, 5u32]).u32(), 0);
    }

    #[test]
    fn stats_call_passes_stats_then_flag() {
        let mut e = Engine::new();
        let m = manager(&mut e, 3, 2);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00aa4710, &args![m, 1u32, 0x1FFu32, 0xABCu32])
            .bool());
        assert!(!e.call(0x00aa4710, &args![m, 9u32, 1u32, 0xABCu32]).bool());
        let log = e.call_log.take().unwrap();
        let stats: Vec<_> = log.iter().filter(|(a, _)| *a == STATS).collect();
        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].1, vec![0x0400_0010, 0xABC, 0xFF]);
    }

    #[test]
    fn second_array_stats_offsets_the_index() {
        let mut e = Engine::new();
        let m = manager(&mut e, 3, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa4760, &args![m, 1u32, 0u32, 7u32]).bool());
        assert!(!e.call(0x00aa4760, &args![m, 2u32, 0u32, 7u32]).bool());
        assert!(e.call(0x00aa4790, &args![m, 0u32, 1u32, 8u32]).bool());
        let log = e.call_log.take().unwrap();
        let stats: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == STATS)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(
            stats,
            vec![vec![0x0400_0040, 7, 0], vec![0x0400_0000, 8, 1]]
        );
    }

    #[test]
    fn returns_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa47c0, &args![0u32, 1u32, 2u32]).u32(), 0);
    }

    #[test]
    fn map_constructor_allocates_a_zeroed_table() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0x0500_0000u32.into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let map: Ptr<ScrapHeapMap> = e.new_object();
        e.set(map, ScrapHeapMap::m_uiCount, 9);
        assert_eq!(e.call(0x00aa47e0, &args![map, 37u32]).u32(), map.addr());
        assert_eq!(e.get(map, ScrapHeapMap::vtable), SCRAP_HEAP_MAP_VTABLE);
        assert_eq!(e.get(map, ScrapHeapMap::m_uiHashSize), 37);
        assert_eq!(e.get(map, ScrapHeapMap::m_ppkHashTable), 0x0500_0000);
        assert_eq!(e.get(map, ScrapHeapMap::m_uiCount), 0);
        let log = e.call_log.take().unwrap();
        let find = |addr: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|(a, _)| *a == addr)
                .map(|(_, w)| w.clone())
                .collect()
        };
        assert_eq!(
            find(MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 148]]
        );
        assert_eq!(find(MEMSET), vec![vec![0x0500_0000, 0, 148]]);
    }

    #[test]
    fn map_constructor_with_zero_size_allocates_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let map: Ptr<ScrapHeapMap> = e.new_object();
        e.set(map, ScrapHeapMap::m_ppkHashTable, 0x77);
        e.call(0x00aa47e0, &args![map, 0u32]);
        assert_eq!(e.get(map, ScrapHeapMap::m_ppkHashTable), 0x77);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
