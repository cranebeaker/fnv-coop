//! `BSCore/bsblockmemheap.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa4860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<void *,BlockMemPage *>::SetAt` (Xbox PDB), `RET 8`. The bucket is the map's
/// vtable slot `0x4` (hash) of `key`; a node of the bucket's chain (bucket table at `+8`, each
/// node's next at `+0` and key at `+4`) whose key the slot `0x8` (equality) matches is cleared
/// (slot `0x10`) and set again (slot `0xc` with the node, `key`, `value`). Without one, a new
/// node from slot `0x14` is set (slot `0xc`), linked at the head of the bucket, and the count
/// (`+0xc`) grows.
pub fn bs_map_base_block_mem_page_set_at(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let base = this.addr();
    let bucket = e.vcall(base, 0x4, &args![key]).u32();
    let table = e.mem.u32(base.wrapping_add(8));
    let mut node = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node.wrapping_add(4));
        if e.vcall(base, 0x8, &args![key, node_key]).bool() {
            e.vcall(base, 0x10, &args![node]);
            e.vcall(base, 0xc, &args![node, key, value]);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.vcall(base, 0x14, &args![]).u32();
    e.vcall(base, 0xc, &args![node, key, value]);
    let table = e.mem.u32(base.wrapping_add(8));
    let head = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    e.mem.set_u32(node, head);
    let table = e.mem.u32(base.wrapping_add(8));
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), node);
    let count = e.mem.u32(base.wrapping_add(0xc));
    e.mem.set_u32(base.wrapping_add(0xc), count.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00aa_4860,
        bs_map_base_block_mem_page_set_at(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    const HASH: u32 = 0x7000_0004;
    const EQUAL: u32 = 0x7000_0008;
    const SET: u32 = 0x7000_000c;
    const CLEAR: u32 = 0x7000_0010;
    const NEW: u32 = 0x7000_0014;

    fn map(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x18);
        for slot in (4..0x18).step_by(4) {
            e.mem.set_u32(vtable + slot, 0x7000_0000 + slot);
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        let table = e.mem.alloc(8);
        e.mem.set_u32(this + 8, table);
        e.register(HASH, |_, a| (a[1] & 1).into_ret());
        e.register(EQUAL, |_, a| (a[1] == a[2]).into_ret());
        e.register(SET, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(CLEAR, |_, _| Ret::default());
        e.register(NEW, |e, _| e.mem.alloc(0xc).into_ret());
        this
    }

    #[test]
    fn set_at_adds_then_replaces() {
        let mut e = Engine::new();
        let this = map(&mut e);
        let table = e.mem.u32(this + 8);
        e.call(0x00aa_4860, &args![this, 3u32, 0x30u32]);
        e.call(0x00aa_4860, &args![this, 5u32, 0x50u32]);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let head = e.mem.u32(table + 4);
        assert_eq!((e.mem.u32(head + 4), e.mem.u32(head + 8)), (5, 0x50));
        let second = e.mem.u32(head);
        assert_eq!((e.mem.u32(second + 4), e.mem.u32(second + 8)), (3, 0x30));
        assert_eq!(e.mem.u32(table), 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_4860, &args![this, 3u32, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(second + 8), 0x33);
        let clears: Vec<_> = log.iter().filter(|(a, _)| *a == CLEAR).collect();
        assert_eq!(clears, vec![&(CLEAR, vec![this, second])]);
        assert!(log.iter().all(|(a, _)| *a != NEW));
    }
}
