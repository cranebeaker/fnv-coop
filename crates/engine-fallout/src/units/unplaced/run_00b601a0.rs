//! `(unplaced)/run_00b601a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slots of the map object called on each node (byte offsets).
const SLOT_RELEASE_NODE: u32 = 0x10;
const SLOT_FREE_NODE: u32 = 0x18;

/// Offsets of the map object: the bucket count (`+4`), the bucket array (`+8`)
/// and the item count (`+0xc`).
const BUCKET_COUNT: u32 = 0x04;
const BUCKETS: u32 = 0x08;
const COUNT: u32 = 0x0c;

// Translated from 00b601a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `RemoveAll` of the
/// chained hash map `00b57ea0` belongs to. Walks the buckets in order; for every
/// node of a bucket it unlinks the head, calls virtual slot `+0x10` and then
/// slot `+0x18` with the node, then sets the item count to 0. The bucket array
/// and the bucket count are read again at each step, as the code does.
pub fn fn_00b601a0(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    let mut bucket = 0u32;
    if e.mem.u32(map + BUCKET_COUNT) != 0 {
        loop {
            let table = e.mem.u32(map + BUCKETS);
            if e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4))) != 0 {
                loop {
                    let table = e.mem.u32(map + BUCKETS);
                    let slot = table.wrapping_add(bucket.wrapping_mul(4));
                    let node = e.mem.u32(slot);
                    let next = e.mem.u32(node);
                    e.mem.set_u32(slot, next);
                    e.vcall(map, SLOT_RELEASE_NODE, &args![node]);
                    e.vcall(map, SLOT_FREE_NODE, &args![node]);
                    let table = e.mem.u32(map + BUCKETS);
                    if e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4))) == 0 {
                        break;
                    }
                }
            }
            bucket = bucket.wrapping_add(1);
            if bucket >= e.mem.u32(map + BUCKET_COUNT) {
                break;
            }
        }
    }
    e.mem.set_u32(map + COUNT, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b601a0, fn_00b601a0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0700_0000;
    const RELEASE: u32 = 0x0500_0004;
    const FREE_NODE: u32 = 0x0500_0006;

    /// A map of `buckets` buckets; `chains[i]` lists the nodes of bucket i.
    fn map(e: &mut Engine, chains: &[&[u32]]) -> (u32, u32) {
        e.put_vtable(VTABLE, &[0, 0, 0, 0, RELEASE, 0, FREE_NODE]);
        e.register(RELEASE, |_, _| Ret::default());
        e.register(FREE_NODE, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        let table = e.mem.alloc(4 * chains.len() as u32);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + BUCKET_COUNT, chains.len() as u32);
        e.mem.set_u32(object + BUCKETS, table);
        e.mem.set_u32(object + COUNT, 99);
        for (i, chain) in chains.iter().enumerate() {
            let mut next = 0;
            for node in chain.iter().rev() {
                e.mem.set_u32(*node, next);
                next = *node;
            }
            e.mem.set_u32(table + 4 * i as u32, next);
        }
        (object, table)
    }

    #[test]
    fn every_node_is_released_then_freed_in_bucket_order() {
        let mut e = Engine::new();
        let nodes: Vec<u32> = (0..4).map(|_| e.mem.alloc(12)).collect();
        let (object, table) = map(
            &mut e,
            &[&[nodes[0], nodes[1]], &[], &[nodes[2]], &[nodes[3]]],
        );
        e.call_log = Some(vec![]);
        e.call(0x00b601a0, &args![object]);
        let log = e.call_log.take().unwrap();
        let mut want = vec![];
        for node in &nodes {
            want.push((RELEASE, vec![object, *node]));
            want.push((FREE_NODE, vec![object, *node]));
        }
        assert_eq!(log[1..], want[..]);
        assert_eq!(e.mem.u32(object + COUNT), 0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(table + 4 * i), 0);
        }
    }

    #[test]
    fn a_map_without_buckets_only_clears_the_count() {
        let mut e = Engine::new();
        let (object, _) = map(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00b601a0, &args![object]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(object + COUNT), 0);
    }
}
