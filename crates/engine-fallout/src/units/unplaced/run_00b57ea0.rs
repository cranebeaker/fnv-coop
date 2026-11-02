//! `(unplaced)/run_00b57ea0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slots of the map object that `00b57ea0` calls (byte offsets).
const SLOT_HASH: u32 = 0x04;
const SLOT_EQUAL: u32 = 0x08;
const SLOT_ASSIGN: u32 = 0x0c;
const SLOT_NEW_NODE: u32 = 0x14;

/// Offsets of the map object: the bucket array (`+8`) and the item count
/// (`+0xc`); the node layout is next (`+0`), key (`+4`), value (`+8`).
const BUCKETS: u32 = 0x08;
const COUNT: u32 = 0x0c;

// Translated from 00b57ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: key, value): the `SetAt` of a
/// chained hash map whose hashing, comparison, node assignment and node
/// allocation are virtual slots (`+4` hash of the key, `+8` keys equal, `+0xc`
/// assign a node its key and value, `+0x14` new node). The bucket is the hash
/// result as an index. A node with an equal key gets the new value (at `+8`);
/// otherwise a new node is assigned, linked at the head of the bucket and
/// counted. (The decompiler drops the value argument; the disassembly passes
/// it to slot `+0xc`.)
pub fn fn_00b57ea0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, SLOT_HASH, &args![key]).u32();
    let table = e.mem.u32(map + BUCKETS);
    let mut node = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, SLOT_EQUAL, &args![key, node_key]).bool() {
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(map, SLOT_NEW_NODE, &args![]).u32();
    e.vcall(map, SLOT_ASSIGN, &args![node, key, value]);
    let table = e.mem.u32(map + BUCKETS);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let table = e.mem.u32(map + BUCKETS);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), node);
    let count = e.mem.u32(map + COUNT);
    e.mem.set_u32(map + COUNT, count.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b57ea0, fn_00b57ea0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0700_0000;
    const HASH: u32 = 0x0500_0001;
    const EQUAL: u32 = 0x0500_0002;
    const ASSIGN: u32 = 0x0500_0003;
    const NEW_NODE: u32 = 0x0500_0005;

    /// A map of 4 buckets whose hash is `key % 4`.
    fn map(e: &mut Engine) -> (u32, u32) {
        e.put_vtable(VTABLE, &[0, HASH, EQUAL, ASSIGN, 0, NEW_NODE, 0]);
        e.register(HASH, |_, a| Ret {
            eax: a[1] % 4,
            ..Ret::default()
        });
        e.register(EQUAL, |_, a| Ret {
            eax: u32::from(a[1] == a[2]),
            ..Ret::default()
        });
        e.register(ASSIGN, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(NEW_NODE, |e, _| Ret {
            eax: e.mem.alloc(12),
            ..Ret::default()
        });
        let object = e.mem.alloc(0x10);
        let table = e.mem.alloc(16);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + BUCKETS, table);
        (object, table)
    }

    #[test]
    fn a_new_key_gets_a_node_at_the_head_of_its_bucket() {
        let mut e = Engine::new();
        let (object, table) = map(&mut e);
        e.call(0x00b57ea0, &args![object, 5u32, 50u32]);
        let first = e.mem.u32(table + 4);
        assert_eq!((e.mem.u32(first + 4), e.mem.u32(first + 8)), (5, 50));
        assert_eq!(e.mem.u32(first), 0);
        // 9 hashes to the same bucket and goes in front.
        e.call(0x00b57ea0, &args![object, 9u32, 90u32]);
        let second = e.mem.u32(table + 4);
        assert_ne!(second, first);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(object + COUNT), 2);
    }

    #[test]
    fn an_existing_key_only_gets_a_new_value() {
        let mut e = Engine::new();
        let (object, table) = map(&mut e);
        e.call(0x00b57ea0, &args![object, 5u32, 50u32]);
        e.call(0x00b57ea0, &args![object, 9u32, 90u32]);
        e.call(0x00b57ea0, &args![object, 5u32, 55u32]);
        assert_eq!(e.mem.u32(object + COUNT), 2);
        let head = e.mem.u32(table + 4);
        let tail = e.mem.u32(head);
        assert_eq!((e.mem.u32(head + 4), e.mem.u32(head + 8)), (9, 90));
        assert_eq!((e.mem.u32(tail + 4), e.mem.u32(tail + 8)), (5, 55));
    }
}
