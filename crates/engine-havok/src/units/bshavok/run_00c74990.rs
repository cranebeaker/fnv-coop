//! `BSHavok/run_00c74990` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;

/// Virtual slot (byte offset) of the map function that gives the bucket index of
/// a key, and of the one that compares a key with the key of an item.
const SLOT_HASH: u32 = 0x04;
const SLOT_IS_KEY_EQUAL: u32 = 0x08;

// Translated from 00c74990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `GetAt` of an `NiTMapBase`-shaped hash map
/// (layout [`NiTPointerMap`]) whose value is a 16-bit word (`thiscall`, two stack
/// words); the same walk as `00a53c30`. The map's `HashFunction` (slot 4, `key`)
/// gives the bucket; the chain is walked (items: next at +0, key at +4, value at
/// +8) and for each item the map's `IsKeysEqual` (slot 8, `key`, the item key)
/// decides. On a match the 16-bit value is stored in the word at `value` and the
/// result is true; otherwise false and `value` is not written.
pub fn fn_00c74990(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), SLOT_HASH, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(this.addr(), SLOT_IS_KEY_EQUAL, &args![key, item_key])
            .bool()
        {
            let found = e.mem.u16(item.wrapping_add(8));
            e.mem.set_u16(value.addr(), found);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c7_4990,
        fn_00c74990(Ptr<NiTPointerMap>, u32, Ptr) -> bool
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: u32 = 0x0040_0010;
    const EQUAL: u32 = 0x0040_0020;

    /// A map of two buckets: bucket 1 holds items (key 5, value 0x1234 with
    /// junk above it) and (key 7, value 70); bucket 0 is empty. The hash
    /// function is `key % 2`.
    fn map() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(HASH, |_, a| (a[1] % 2).into_ret());
        e.register(EQUAL, |_, a| (a[1] == a[2]).into_ret());
        let vtable = e.mem.alloc(12);
        e.mem.set_u32(vtable + 4, HASH);
        e.mem.set_u32(vtable + 8, EQUAL);
        let table = e.mem.alloc(8);
        let second = e.mem.alloc(12);
        e.mem.set_u32(second + 4, 7);
        e.mem.set_u32(second + 8, 70);
        let first = e.mem.alloc(12);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u32(first + 8, 0xffff_1234);
        e.mem.set_u32(table + 4, first);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, table);
        (e, map)
    }

    #[test]
    fn a_key_in_the_chain_gives_its_16_bit_value_only() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xaaaa_aaaa);
        assert!(e.call(0x00c7_4990, &args![map, 5u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0xaaaa_1234);
        assert!(e.call(0x00c7_4990, &args![map, 7u32, out]).bool());
        assert_eq!(e.mem.u16(out), 70);
    }

    #[test]
    fn a_missing_key_leaves_the_result_alone() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        assert!(!e.call(0x00c7_4990, &args![map, 9u32, out]).bool());
        assert!(!e.call(0x00c7_4990, &args![map, 4u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    #[test]
    fn the_hash_and_compare_functions_get_the_key_and_the_item_key() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00c7_4990, &args![map, 7u32, out]);
        let log = e.call_log.take().unwrap();
        let hashes: Vec<_> = log.iter().filter(|call| call.0 == HASH).collect();
        assert_eq!(hashes.len(), 1);
        assert_eq!(hashes[0].1, args![map, 7u32]);
        let compares: Vec<_> = log.iter().filter(|call| call.0 == EQUAL).collect();
        assert_eq!(compares.len(), 2);
        assert_eq!(compares[0].1, args![map, 7u32, 5u32]);
        assert_eq!(compares[1].1, args![map, 7u32, 7u32]);
    }
}
