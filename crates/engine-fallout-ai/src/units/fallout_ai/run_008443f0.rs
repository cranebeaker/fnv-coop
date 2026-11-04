//! `fallout/ai/run_008443f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTMap;

/// The function `fn_00844500` calls on the member (`00845260`).
const CALLEE_00845260: u32 = 0x0084_5260;

// Translated from 00844500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `00845260` with `ECX` = `this + 0xc` and passes on its pointer result.
pub fn fn_00844500(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CALLEE_00845260, &args![this.byte_add(0xc)]).ptr()
}

/// The map's vtable slots `+4` (one stack word, the key: the bucket index) and `+8` (two stack
/// words, two keys: a `bool` in `AL`, whether they are equal).
const KEY_TO_HASH_INDEX_SLOT: u32 = 0x4;
const IS_KEYS_EQUAL_SLOT: u32 = 0x8;
/// `NiTMapItem` (Xbox PDB) words: `m_pkNext` `+0`, `m_key` `+4`, `m_val` from `+8`.
const ITEM_NEXT: u32 = 0x0;
const ITEM_KEY: u32 = 0x4;
const ITEM_VALUE: u32 = 0x8;

// Translated from 008443f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The lookup of an `NiTMapBase` instance with an 8-byte value (unnamed in the engine map;
/// `thiscall`, two stack words: the key and the address the value is copied to, `RET 8`): the
/// bucket from vtable slot `+4`, then along the bucket's chain (`m_pkNext`) the first item
/// whose key slot `+8` calls equal to `key`; its two value words go to `out` and the answer is
/// `true`. `false` when the chain ends without one.
pub fn fn_008443f0(e: &mut Engine, this: Ptr<NiTMap>, key: u32, out: u32) -> bool {
    let map = this.addr();
    let bucket = e.vcall(map, KEY_TO_HASH_INDEX_SLOT, &args![key]).u32();
    let table = e.get(this, NiTMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item + ITEM_KEY);
        if e.vcall(map, IS_KEYS_EQUAL_SLOT, &args![key, item_key])
            .bool()
        {
            let first = e.mem.u32(item + ITEM_VALUE);
            let second = e.mem.u32(item + ITEM_VALUE + 4);
            e.mem.set_u32(out, first);
            e.mem.set_u32(out + 4, second);
            return true;
        }
        item = e.mem.u32(item + ITEM_NEXT);
    }
    false
}

/// The vtables (RTTI names read through their complete object locators):
/// `NiTMap<unsigned int,ReconstructFormData>` (`0107eba8`), its
/// `NiTMapBase<DFALL<..>,unsigned int,ReconstructFormData>` (`0107ec68`),
/// `NiTMap<unsigned int,BGSReconstructFormsMap *>` (`0107ebc8`), its `NiTMapBase` (`0107ec88`)
/// and the `NiTMapBase<..,unsigned int,BGSReconstructCellsMap *>` (`0107eca8`).
const FORM_DATA_MAP_VTABLE_0107EBA8: u32 = 0x0107_eba8;
const FORM_DATA_MAP_BASE_VTABLE_0107EC68: u32 = 0x0107_ec68;
const FORMS_MAP_MAP_VTABLE_0107EBC8: u32 = 0x0107_ebc8;
const FORMS_MAP_MAP_BASE_VTABLE_0107EC88: u32 = 0x0107_ec88;
const CELLS_MAP_MAP_BASE_VTABLE_0107ECA8: u32 = 0x0107_eca8;
/// `00438af0` (unnamed, `thiscall`, no stack words): empties the map (the call both
/// destructors make first).
const MAP_REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// The Ni allocator's allocate and free (`00aa1070`, `00aa10f0`; `cdecl`, one word).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;
/// `00403d30` (unnamed, `cdecl`: destination, byte, count): fills memory.
const MEMORY_SET_00403D30: u32 = 0x0040_3d30;
/// `006b8310` (unnamed, `thiscall` on the map's allocator `+0xc`, one stack word: the item).
const ALLOCATOR_DEALLOCATE_006B8310: u32 = 0x006b_8310;

/// An `NiTMapBase` destructor: the vtable, `00438af0`, then the bucket array freed.
fn destroy_map_base(e: &mut Engine, this: Ptr<NiTMap>, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(NI_FREE_00AA10F0, &args![table]);
}

/// An `NiTMapBase` constructor: the vtable, `m_uiHashSize`, a zero count, and a bucket array of
/// `buckets` words from the Ni allocator, filled with zero bytes.
fn construct_map_base(e: &mut Engine, this: Ptr<NiTMap>, vtable: u32, buckets: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTMap::m_uiHashSize, buckets);
    e.set(this, NiTMap::m_uiCount, 0);
    let bytes = e.get(this, NiTMap::m_uiHashSize).wrapping_shl(2);
    let table = e.call(NI_ALLOCATE_00AA1070, &args![bytes]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    let bytes = e.get(this, NiTMap::m_uiHashSize).wrapping_shl(2);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(MEMORY_SET_00403D30, &args![table, 0u32, bytes]);
    this
}

// Translated from 00844470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMap<unsigned int,ReconstructFormData>` (by its vtable; unnamed in the
/// engine map; `thiscall`, no stack words): the vtable `0107eba8`, `00438af0`, then the
/// `NiTMapBase` destructor [`fn_008444d0`].
// Translation incomplete: 00844470 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_00844470(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), FORM_DATA_MAP_VTABLE_0107EBA8);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_008444d0(e, this);
}

// Translated from 008444d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMapBase<DFALL<..>,unsigned int,ReconstructFormData>` (by its vtable;
/// unnamed in the engine map; `thiscall`, no stack words): the vtable `0107ec68`, `00438af0`,
/// then the bucket array freed (`00aa10f0`).
pub fn fn_008444d0(e: &mut Engine, this: Ptr<NiTMap>) {
    destroy_map_base(e, this, FORM_DATA_MAP_BASE_VTABLE_0107EC68);
}

// Translated from 00844560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one stack word, `RET 4`): an 8-byte value
/// constructor (the value of `NiTMap<unsigned int,ReconstructFormData>`, by its only caller
/// here): word 0 cleared, word 1 set to `second`. Returns `this`.
pub fn fn_00844560(e: &mut Engine, this: Ptr, second: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, second);
    this
}

// Translated from 00844520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `+0x18` of `NiTMap<unsigned int,ReconstructFormData>` (`0107eba8`; also at
/// `0107e948`; unnamed in the engine map; `thiscall`, one stack word: the item, `RET 4`): a
/// local value made by [`fn_00844560`] with 0 is copied into the item's value (`+8`, `+0xc`),
/// then the item goes to `006b8310` on the allocator `+0xc`.
pub fn fn_00844520(e: &mut Engine, this: Ptr<NiTMap>, item: u32) {
    e.with_stack(8, |e, value| {
        fn_00844560(e, value, 0);
        let first = e.mem.u32(value.addr());
        e.mem.set_u32(item + ITEM_VALUE, first);
        let second = e.mem.u32(value.addr() + 4);
        e.mem.set_u32(item + ITEM_VALUE + 4, second);
    });
    e.call(
        ALLOCATOR_DEALLOCATE_006B8310,
        &args![this.addr() + 0xc, item],
    );
}

// Translated from 00844590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMapBase<DFALL<..>,unsigned int,BGSReconstructFormsMap *>` (by its
/// vtable; unnamed in the engine map; `thiscall`, one stack word: the bucket count, `RET 4`):
/// the vtable `0107ec88`, `m_uiHashSize`, a zero item count, and the bucket array (`00aa1070`
/// of four bytes a bucket) filled with zero (`00403d30`). Returns `this`.
pub fn fn_00844590(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, FORMS_MAP_MAP_BASE_VTABLE_0107EC88, buckets)
}

// Translated from 00844600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMap<unsigned int,BGSReconstructFormsMap *>` (by its vtable; unnamed in
/// the engine map; `thiscall`, no stack words): the vtable `0107ebc8`, `00438af0`, then the
/// `NiTMapBase` destructor [`fn_00844660`].
// Translation incomplete: 00844600 - the C++ exception frame (FS:[0] chain, one unwind state for the base) is not translated.
pub fn fn_00844600(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), FORMS_MAP_MAP_VTABLE_0107EBC8);
    e.call(MAP_REMOVE_ALL_00438AF0, &args![this]);
    fn_00844660(e, this);
}

// Translated from 00844660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMapBase<DFALL<..>,unsigned int,BGSReconstructFormsMap *>` (by its vtable;
/// unnamed in the engine map; `thiscall`, no stack words): the vtable `0107ec88`, `00438af0`,
/// then the bucket array freed (`00aa10f0`).
pub fn fn_00844660(e: &mut Engine, this: Ptr<NiTMap>) {
    destroy_map_base(e, this, FORMS_MAP_MAP_BASE_VTABLE_0107EC88);
}

// Translated from 00844690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMapBase<DFALL<..>,unsigned int,BGSReconstructCellsMap *>` (by its
/// vtable; unnamed in the engine map; `thiscall`, one stack word: the bucket count, `RET 4`):
/// as [`fn_00844590`] with the vtable `0107eca8`. Returns `this`.
pub fn fn_00844690(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, CELLS_MAP_MAP_BASE_VTABLE_0107ECA8, buckets)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0084_4500, fn_00844500(Ptr) -> Ptr),
        entry!(0x0084_43f0, fn_008443f0(Ptr<NiTMap>, u32, u32) -> bool),
        entry!(0x0084_4470, fn_00844470(Ptr<NiTMap>)),
        entry!(0x0084_44d0, fn_008444d0(Ptr<NiTMap>)),
        entry!(0x0084_4520, fn_00844520(Ptr<NiTMap>, u32)),
        entry!(0x0084_4560, fn_00844560(Ptr, u32) -> Ptr),
        entry!(0x0084_4590, fn_00844590(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x0084_4600, fn_00844600(Ptr<NiTMap>)),
        entry!(0x0084_4660, fn_00844660(Ptr<NiTMap>)),
        entry!(0x0084_4690, fn_00844690(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00844500_passes_this_plus_c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_00845260, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0084_4500, &args![0x4100u32]).ptr::<()>().addr(),
            0x1459
        );
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_00845260).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0xc]);
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A map with four buckets whose hash is `key % 4` and whose keys compare equal when the
    /// same; bucket 1 chains the items for keys 5 and 9.
    fn map_with_chain(e: &mut Engine) -> Ptr<NiTMap> {
        let map: Ptr<NiTMap> = e.new_object();
        e.put_vtable(0x0070_0000, &[0, 0x0070_1000, 0x0070_1010]);
        e.mem.set_u32(map.addr(), 0x0070_0000);
        e.register(0x0070_1000, |_, a| (a[1] % 4).into_ret());
        e.register(0x0070_1010, |_, a| (a[1] == a[2]).into_ret());
        let table = e.mem.alloc(0x10);
        e.set(map, NiTMap::m_ppkHashTable, table);
        e.set(map, NiTMap::m_uiHashSize, 4);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 4, 9);
        e.mem.set_u32(second + 8, 0x99);
        e.mem.set_u32(second + 12, 0x999);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(first + 8, 0x55);
        e.mem.set_u32(first + 12, 0x555);
        e.mem.set_u32(table + 4, first);
        map
    }

    #[test]
    fn fn_008443f0_finds_the_value_along_the_chain() {
        let mut e = Engine::new();
        let map = map_with_chain(&mut e);
        let out = e.mem.alloc(8);
        assert!(e.call(0x0084_43f0, &args![map, 9u32, out]).bool());
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x99, 0x999));
        assert!(e.call(0x0084_43f0, &args![map, 5u32, out]).bool());
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x55, 0x555));
        // Same bucket, no item; and an empty bucket.
        e.mem.set_u32(out, 0x1234);
        assert!(!e.call(0x0084_43f0, &args![map, 13u32, out]).bool());
        assert!(!e.call(0x0084_43f0, &args![map, 2u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    #[test]
    fn map_destructors_empty_then_free_the_buckets() {
        for (address, vtable, base_vtable) in [
            (
                0x0084_4470u32,
                Some(FORM_DATA_MAP_VTABLE_0107EBA8),
                FORM_DATA_MAP_BASE_VTABLE_0107EC68,
            ),
            (0x0084_44d0, None, FORM_DATA_MAP_BASE_VTABLE_0107EC68),
            (
                0x0084_4600,
                Some(FORMS_MAP_MAP_VTABLE_0107EBC8),
                FORMS_MAP_MAP_BASE_VTABLE_0107EC88,
            ),
            (0x0084_4660, None, FORMS_MAP_MAP_BASE_VTABLE_0107EC88),
        ] {
            let mut e = Engine::new();
            let map: Ptr<NiTMap> = e.new_object();
            e.set(map, NiTMap::m_ppkHashTable, 0x4440);
            let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
            let record = seen.clone();
            e.register_double(MAP_REMOVE_ALL_00438AF0, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[0]));
                Ret::default()
            });
            e.register(NI_FREE_00AA10F0, nothing);
            e.call_log = Some(vec![]);
            e.call(address, &args![map]);
            let log = e.call_log.take().unwrap();
            let expected: Vec<u32> = vtable.into_iter().chain([base_vtable]).collect();
            assert_eq!(*seen.borrow(), expected);
            assert_eq!(calls(&log, NI_FREE_00AA10F0), vec![vec![0x4440]]);
            assert_eq!(e.mem.u32(map.addr()), base_vtable);
        }
    }

    #[test]
    fn fn_00844560_makes_the_value() {
        let mut e = Engine::new();
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 0x77);
        assert_eq!(e.call(0x0084_4560, &args![value, 0x42u32]).u32(), value);
        assert_eq!((e.mem.u32(value), e.mem.u32(value + 4)), (0, 0x42));
    }

    #[test]
    fn fn_00844520_clears_the_value_and_deallocates_the_item() {
        let mut e = Engine::new();
        let map: Ptr<NiTMap> = e.new_object();
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 4, 3);
        e.mem.set_u32(item + 8, 0x11);
        e.mem.set_u32(item + 12, 0x22);
        e.register(ALLOCATOR_DEALLOCATE_006B8310, nothing);
        e.call_log = Some(vec![]);
        e.call(0x0084_4520, &args![map, item]);
        let log = e.call_log.take().unwrap();
        assert_eq!((e.mem.u32(item + 8), e.mem.u32(item + 12)), (0, 0));
        assert_eq!(e.mem.u32(item + 4), 3);
        assert_eq!(
            calls(&log, ALLOCATOR_DEALLOCATE_006B8310),
            vec![vec![map.addr() + 0xc, item]]
        );
    }

    #[test]
    fn map_constructors_allocate_and_clear_the_buckets() {
        for (address, vtable) in [
            (0x0084_4590u32, FORMS_MAP_MAP_BASE_VTABLE_0107EC88),
            (0x0084_4690, CELLS_MAP_MAP_BASE_VTABLE_0107ECA8),
        ] {
            let mut e = Engine::new();
            let map: Ptr<NiTMap> = e.new_object();
            e.set(map, NiTMap::m_uiCount, 9);
            e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(MEMORY_SET_00403D30, nothing);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 37u32]).u32(), map.addr());
            let log = e.call_log.take().unwrap();
            let table = e.get(map, NiTMap::m_ppkHashTable);
            assert_eq!(e.mem.u32(map.addr()), vtable);
            assert_eq!(e.get(map, NiTMap::m_uiHashSize), 37);
            assert_eq!(e.get(map, NiTMap::m_uiCount), 0);
            assert_eq!(calls(&log, NI_ALLOCATE_00AA1070), vec![vec![37 * 4]]);
            assert_eq!(
                calls(&log, MEMORY_SET_00403D30),
                vec![vec![table, 0, 37 * 4]]
            );
        }
    }
}
