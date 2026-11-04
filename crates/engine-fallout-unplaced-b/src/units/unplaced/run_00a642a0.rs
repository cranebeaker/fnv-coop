//! `(unplaced)/run_00a642a0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Virtual slots of the map object (byte offsets): hash of a key, keys equal,
/// assign a node its key and value.
const SLOT_HASH: u32 = 0x04;
const SLOT_EQUAL: u32 = 0x08;
const SLOT_ASSIGN: u32 = 0x0c;

/// Offsets of the map object: the bucket array (`+8`), the node allocator whose
/// first word is the item count (`+0xc`) and the byte that says the keys are
/// kept as they are (`+0x10`).
const BUCKETS: u32 = 0x08;
const ALLOCATOR: u32 = 0x0c;
const KEEP_KEYS: u32 = 0x10;
/// `0043a010` (`thiscall`, no stack word, `ECX` = the allocator at `+0xc`):
/// takes a node from the free list (the engine map names the callee of the
/// decompiled call `NiTPointerListBase<...>::AddHead`).
const ALLOCATE_NODE: u32 = 0x0043_a010;

// Translated from 00a642a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTPointerMap<char const *, NiShader *>, NiShader *>::SetAt`
/// (engine map name; `thiscall`, `RET 8`: key, value). The `SetAt` of a chained
/// hash map: hashes the key (slot `+4`) to a bucket and walks its chain comparing
/// keys (slot `+8`). A node with an equal key gets the value at `+8`, and its key
/// word at `+4` is also replaced by the new key unless the byte at `+0x10` is set.
/// Otherwise a node is taken with `0043a010` on `this + 0xc`, assigned the key and
/// value (slot `+0xc`), linked at the head of its bucket and counted in the word
/// at `+0xc`. (The decompiler shows a stale register for the bucket index; the
/// disassembly keeps it in a stack slot.)
pub fn fn_00a642a0(e: &mut Engine, this: Ptr, key: u32, value: u32) {
    let map = this.addr();
    let bucket = e.vcall(map, SLOT_HASH, &args![key]).u32();
    let table = e.mem.u32(map + BUCKETS);
    let mut node = e.mem.u32(table.wrapping_add(bucket.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u32(node + 4);
        if e.vcall(map, SLOT_EQUAL, &args![key, node_key]).bool() {
            if e.mem.u8(map + KEEP_KEYS) == 0 {
                e.mem.set_u32(node + 4, key);
            }
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let node = e.call(ALLOCATE_NODE, &args![map + ALLOCATOR]).u32();
    e.vcall(map, SLOT_ASSIGN, &args![node, key, value]);
    let table = e.mem.u32(map + BUCKETS);
    let slot = table.wrapping_add(bucket.wrapping_mul(4));
    let head = e.mem.u32(slot);
    e.mem.set_u32(node, head);
    let table = e.mem.u32(map + BUCKETS);
    e.mem
        .set_u32(table.wrapping_add(bucket.wrapping_mul(4)), node);
    let count = e.mem.u32(map + ALLOCATOR);
    e.mem.set_u32(map + ALLOCATOR, count.wrapping_add(1));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a642a0, fn_00a642a0(Ptr, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0700_0000;
    const HASH: u32 = 0x0500_0001;
    const EQUAL: u32 = 0x0500_0002;
    const ASSIGN: u32 = 0x0500_0003;

    /// A map of 4 buckets with hash `key % 4`; keys "equal" when they differ by
    /// a multiple of 100 (so a stored key can differ from the lookup key).
    fn map(e: &mut Engine, keep_keys: u8) -> (u32, u32) {
        e.put_vtable(VTABLE, &[0, HASH, EQUAL, ASSIGN]);
        e.register(HASH, |_, a| Ret {
            eax: a[1] % 4,
            ..Ret::default()
        });
        e.register(EQUAL, |_, a| Ret {
            eax: u32::from(a[1] % 100 == a[2] % 100),
            ..Ret::default()
        });
        e.register(ASSIGN, |e, a| {
            e.mem.set_u32(a[1] + 4, a[2]);
            e.mem.set_u32(a[1] + 8, a[3]);
            Ret::default()
        });
        e.register(ALLOCATE_NODE, |e, _| Ret {
            eax: e.mem.alloc(12),
            ..Ret::default()
        });
        let object = e.mem.alloc(0x14);
        let table = e.mem.alloc(16);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + BUCKETS, table);
        e.mem.set_u8(object + KEEP_KEYS, keep_keys);
        (object, table)
    }

    #[test]
    fn a_new_key_gets_a_node_from_the_allocator() {
        let mut e = Engine::new();
        let (object, table) = map(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a642a0, &args![object, 5u32, 50u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (HASH, vec![object, 5]));
        assert_eq!(log[2].0, ALLOCATE_NODE);
        assert_eq!(log[2].1, vec![object + 0xc]);
        let node = e.mem.u32(table + 4);
        assert_eq!((e.mem.u32(node + 4), e.mem.u32(node + 8)), (5, 50));
        assert_eq!(e.mem.u32(object + 0xc), 1);
        e.call(0x00a642a0, &args![object, 9u32, 90u32]);
        let head = e.mem.u32(table + 4);
        assert_eq!(e.mem.u32(head), node);
        assert_eq!(e.mem.u32(object + 0xc), 2);
    }

    #[test]
    fn an_equal_key_is_replaced_unless_the_map_keeps_its_keys() {
        for (keep, stored_key) in [(0u8, 105u32), (1, 5)] {
            let mut e = Engine::new();
            let (object, table) = map(&mut e, keep);
            e.call(0x00a642a0, &args![object, 5u32, 50u32]);
            e.call(0x00a642a0, &args![object, 105u32, 60u32]);
            let node = e.mem.u32(table + 4);
            assert_eq!(e.mem.u32(node + 4), stored_key);
            assert_eq!(e.mem.u32(node + 8), 60);
            assert_eq!(e.mem.u32(object + 0xc), 1);
        }
    }
}
