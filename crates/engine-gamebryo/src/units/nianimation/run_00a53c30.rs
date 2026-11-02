//! `NiAnimation/run_00a53c30` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;

/// Virtual slot (byte offset) of the map function that gives the bucket index of
/// a key, and of the one that compares a key with the key of an item.
const SLOT_HASH: u32 = 0x04;
const SLOT_IS_KEY_EQUAL: u32 = 0x08;

// Translated from 00a53c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `GetAt` of an `NiTMapBase`-shaped hash map
/// (layout [`NiTPointerMap`]) whose value is a 4-byte word (`thiscall`, two stack
/// words). The map's `HashFunction` (slot 4, `key`) gives the bucket; the chain is
/// walked (items: next at +0, key at +4, value at +8) and for each item the map's
/// `IsKeysEqual` (slot 8, `key`, the item key) decides. On a match the value is
/// stored in the word at `value` and the result is true; otherwise false and
/// `value` is not written. (The bucket index is used as the hash function returns
/// it.)
pub fn fn_00a53c30(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u32, value: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), SLOT_HASH, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut item = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while item != 0 {
        let item_key = e.mem.u32(item.wrapping_add(4));
        if e.vcall(this.addr(), SLOT_IS_KEY_EQUAL, &args![key, item_key])
            .bool()
        {
            let found = e.mem.u32(item.wrapping_add(8));
            e.mem.set_u32(value.addr(), found);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 00a53c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `GetNext` of the same kind of map whose value is a byte
/// (`thiscall`, (&position, &key, &value), `RET 0xc`): the item at the position gives its key
/// (+4) and value byte (+8); the position moves to the next item of its chain or, at the end
/// of a chain, to the first item of the next non-empty bucket after the item's own (the
/// `HashFunction`, slot 4, of its key), or to null after the last bucket.
pub fn fn_00a53c90(e: &mut Engine, this: Ptr<NiTPointerMap>, position: Ptr, key: Ptr, value: Ptr) {
    let item = e.mem.u32(position.addr());
    let item_key = e.mem.u32(item.wrapping_add(4));
    e.mem.set_u32(key.addr(), item_key);
    let item_value = e.mem.u8(item.wrapping_add(8));
    e.mem.set_u8(value.addr(), item_value);
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let item_key = e.mem.u32(item.wrapping_add(4));
    let mut bucket = e
        .vcall(this.addr(), SLOT_HASH, &args![item_key])
        .u32()
        .wrapping_add(1);
    let size = e.get(this, NiTPointerMap::m_uiHashSize);
    if bucket < size {
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let mut slot = table.wrapping_add(bucket.wrapping_mul(4));
        while bucket < size {
            let first = e.mem.u32(slot);
            if first != 0 {
                e.mem.set_u32(position.addr(), first);
                return;
            }
            bucket += 1;
            slot = slot.wrapping_add(4);
        }
    }
    e.mem.set_u32(position.addr(), 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_3c30,
            fn_00a53c30(Ptr<NiTPointerMap>, u32, Ptr) -> bool
        ),
        entry!(0x00a5_3c90, fn_00a53c90(Ptr<NiTPointerMap>, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: u32 = 0x0040_0010;
    const EQUAL: u32 = 0x0040_0020;

    /// A map of two buckets: bucket 1 holds items (key 5, value 50) and
    /// (key 7, value 70); bucket 0 is empty. The hash function is `key % 2`.
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
        e.mem.set_u32(first + 8, 50);
        e.mem.set_u32(table + 4, first);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 4, 2);
        e.mem.set_u32(map + 8, table);
        (e, map)
    }

    #[test]
    fn a_key_in_the_chain_gives_its_value() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        for (key, value) in [(5u32, 50u32), (7, 70)] {
            e.mem.set_u32(out, 0);
            assert!(e.call(0x00a5_3c30, &args![map, key, out]).bool());
            assert_eq!(e.mem.u32(out), value);
        }
    }

    #[test]
    fn a_missing_key_leaves_the_result_alone() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        // Same bucket as 5 and 7 but not in it, and an empty bucket.
        assert!(!e.call(0x00a5_3c30, &args![map, 9u32, out]).bool());
        assert!(!e.call(0x00a5_3c30, &args![map, 4u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x1234);
    }

    #[test]
    fn the_hash_and_compare_functions_get_the_key_and_the_item_key() {
        let (mut e, map) = map();
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3c30, &args![map, 7u32, out]);
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

#[cfg(test)]
mod n007_tests {
    use super::*;

    #[test]
    fn get_next_walks_chains_then_buckets() {
        let mut e = Engine::new();
        e.register(0x0040_0010, |_, a| (a[1] % 4).into_ret());
        let vtable = e.mem.alloc(12);
        e.mem.set_u32(vtable + 4, 0x0040_0010);
        // Four buckets: 1 holds keys 5 then 9, 3 holds key 7.
        let table = e.mem.alloc(16);
        let item = |e: &mut Engine, next: u32, key: u32, value: u8| {
            let item = e.mem.alloc(12);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 4, key);
            e.mem.set_u8(item + 8, value);
            item
        };
        let nine = item(&mut e, 0, 9, 0x19);
        let five = item(&mut e, nine, 5, 0x15);
        let seven = item(&mut e, 0, 7, 0x17);
        e.mem.set_u32(table + 4, five);
        e.mem.set_u32(table + 12, seven);
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 4, 4);
        e.mem.set_u32(map + 8, table);
        let (position, key, value) = (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(position, five);
        let mut seen = vec![];
        while e.mem.u32(position) != 0 {
            e.call(0x00a5_3c90, &args![map, position, key, value]);
            seen.push((e.mem.u32(key), e.mem.u8(value)));
        }
        assert_eq!(seen, vec![(5, 0x15), (9, 0x19), (7, 0x17)]);
    }
}
