//! `(unplaced)/run_00e91fd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slots of the map object (byte offsets): hash of a key, keys equal,
/// and the two calls made on a removed node.
const SLOT_HASH: u32 = 0x04;
const SLOT_EQUAL: u32 = 0x08;
const SLOT_RELEASE_NODE: u32 = 0x10;
const SLOT_FREE_NODE: u32 = 0x18;

/// Offsets of the map object: the bucket array (`+8`) and the item count
/// (`+0xc`); a node is next (`+0`), key (`+4`).
const BUCKETS: u32 = 0x08;
const COUNT: u32 = 0x0c;

/// Unlinks nothing itself: calls the two node slots and decrements the count.
fn retire(e: &mut Engine, map: u32, node: u32) {
    e.vcall(map, SLOT_RELEASE_NODE, &args![node]);
    e.vcall(map, SLOT_FREE_NODE, &args![node]);
    let count = e.mem.u32(map + COUNT);
    e.mem.set_u32(map + COUNT, count.wrapping_sub(1));
}

// Translated from 00e91fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: key): `RemoveAt` of the
/// chained hash map of `00b57ea0`. Hashes the key (slot `+4`) to a bucket and
/// walks its chain comparing keys (slot `+8`); the first node with an equal key
/// is unlinked, slots `+0x10` and `+0x18` run on it, the count at `+0xc` goes
/// down and the answer is true. Answers false when no node matches.
pub fn fn_00e91fd0(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let map = this.addr();
    let bucket = e.vcall(map, SLOT_HASH, &args![key]).u32();
    let table = e.mem.u32(map + BUCKETS);
    let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    if head == 0 {
        return false;
    }
    let head_key = e.mem.u32(head + 4);
    if e.vcall(map, SLOT_EQUAL, &args![key, head_key]).bool() {
        let table = e.mem.u32(map + BUCKETS);
        let next = e.mem.u32(head);
        e.mem
            .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), next);
        retire(e, map, head);
        return true;
    }
    let mut previous = head;
    let mut node = e.mem.u32(head);
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, SLOT_EQUAL, &args![key, node_key]).bool() {
            let next = e.mem.u32(node);
            e.mem.set_u32(previous, next);
            retire(e, map, node);
            return true;
        }
        previous = node;
        node = e.mem.u32(node);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e91fd0, fn_00e91fd0(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0700_0000;
    const HASH: u32 = 0x0500_0001;
    const EQUAL: u32 = 0x0500_0002;
    const RELEASE: u32 = 0x0500_0004;
    const FREE_NODE: u32 = 0x0500_0006;

    /// A 4-bucket map (hash `key % 4`) holding keys 5, 9 (a chain in bucket 1,
    /// 9 first) and 6.
    fn map(e: &mut Engine) -> (u32, u32, [u32; 3]) {
        e.put_vtable(VTABLE, &[0, HASH, EQUAL, 0, RELEASE, 0, FREE_NODE]);
        e.register(HASH, |_, a| Ret {
            eax: a[1] % 4,
            ..Ret::default()
        });
        e.register(EQUAL, |_, a| Ret {
            eax: u32::from(a[1] == a[2]),
            ..Ret::default()
        });
        e.register(RELEASE, |_, _| Ret::default());
        e.register(FREE_NODE, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        let table = e.mem.alloc(16);
        let nodes = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        for (node, key) in nodes.iter().zip([9u32, 5, 6]) {
            e.mem.set_u32(node + 4, key);
        }
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(table + 4, nodes[0]);
        e.mem.set_u32(table + 8, nodes[2]);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + BUCKETS, table);
        e.mem.set_u32(object + COUNT, 3);
        (object, table, nodes)
    }

    #[test]
    fn the_head_of_a_chain_is_unlinked() {
        let mut e = Engine::new();
        let (object, table, nodes) = map(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00e91fd0, &args![object, 9u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(table + 4), nodes[1]);
        assert_eq!(e.mem.u32(object + COUNT), 2);
        assert_eq!(log[log.len() - 2], (RELEASE, vec![object, nodes[0]]));
        assert_eq!(log[log.len() - 1], (FREE_NODE, vec![object, nodes[0]]));
    }

    #[test]
    fn a_later_node_is_unlinked_from_its_predecessor() {
        let mut e = Engine::new();
        let (object, table, nodes) = map(&mut e);
        assert!(e.call(0x00e91fd0, &args![object, 5u32]).bool());
        assert_eq!(e.mem.u32(table + 4), nodes[0]);
        assert_eq!(e.mem.u32(nodes[0]), 0);
        assert_eq!(e.mem.u32(object + COUNT), 2);
    }

    #[test]
    fn a_missing_key_or_empty_bucket_answers_false() {
        let mut e = Engine::new();
        let (object, _, _) = map(&mut e);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00e91fd0, &args![object, 13u32]).bool());
        assert!(!e.call(0x00e91fd0, &args![object, 7u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != RELEASE && c.0 != FREE_NODE));
        assert_eq!(e.mem.u32(object + COUNT), 3);
    }
}
