//! PC allocation helpers reached by the form map's virtual allocator.
//!
//! The map associates `0043a010` with a renderer template name (Xbox PDB),
//! but its PC body takes a twelve-byte node from a shared free list. That
//! name is not used to infer behavior. `0043a080` has no mapped source unit.

use crate::prelude::*;

const FREE_HEAD: u32 = 0x011c_5f58;
const BLOCK_HEAD: u32 = 0x011c_5f54;
const BLOCK_COUNT: u32 = 0x0118_8f7c;
const LOCK: u32 = 0x011c_5f80;

// Translated from 0043a010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes one free node under the native lock and clears its three words.
/// The receiver is unused; there are no stack arguments (plain RET).
pub fn allocate_node(e: &mut Engine, _this: Ptr) -> Ptr {
    e.call(0x0040_fbf0, &args![LOCK, 0u32]);
    if e.mem.u32(FREE_HEAD) == 0 {
        e.call(0x0043_a080, &args![]);
    }
    let node = e.mem.u32(FREE_HEAD);
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): a node still in a watched list.
    crate::units::fallout_shared_facegen::bsfacegenkeyframe::speech_check::node_allocated(e, node);
    let next = e.mem.u32(node);
    e.mem.set_u32(FREE_HEAD, next);
    e.mem.set_u32(node.wrapping_add(8), 0);
    e.mem.set_u32(node, 0);
    e.mem.set_u32(node.wrapping_add(4), 0);
    e.call(0x0040_fba0, &args![LOCK]);
    Ptr::new(node)
}

// Translated from 0043a080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a block of twelve-byte entries, reserves its first entry for
/// the block chain and links the rest as free nodes. Native code assumes a
/// valid configured count and successful allocation; it adds no guards.
pub fn refill(e: &mut Engine) {
    let bytes = e.mem.u32(BLOCK_COUNT).saturating_mul(12);
    let block = e.call(0x0040_1000, &args![bytes]).u32();
    e.mem.set_u32(FREE_HEAD, block);
    let span = e.mem.u32(BLOCK_COUNT).wrapping_mul(12);
    let last = e.mem.u32(FREE_HEAD).wrapping_add(span).wrapping_sub(12);
    e.mem.set_u32(last, 0);
    let mut index = 1u32;
    loop {
        let node = e.mem.u32(FREE_HEAD).wrapping_add(index.wrapping_mul(12));
        e.mem.set_u32(node, node.wrapping_add(12));
        let bound = e.mem.u32(BLOCK_COUNT).wrapping_sub(2);
        let previous = index;
        index = index.wrapping_add(1);
        if previous >= bound {
            break;
        }
    }
    if e.mem.u32(BLOCK_HEAD) != 0 {
        let head = e.mem.u32(FREE_HEAD);
        let old = e.mem.u32(BLOCK_HEAD);
        e.mem.set_u32(head, old);
        e.mem.set_u32(BLOCK_HEAD, e.mem.u32(FREE_HEAD));
    } else {
        e.mem.set_u32(BLOCK_HEAD, e.mem.u32(FREE_HEAD));
        let head = e.mem.u32(BLOCK_HEAD);
        e.mem.set_u32(head, 0);
    }
    e.mem
        .set_u32(FREE_HEAD, e.mem.u32(FREE_HEAD).wrapping_add(12));
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0043a010, allocate_node(Ptr) -> Ptr),
        entry!(0x0043a080, refill()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(count: u32) -> Engine {
        let mut e = Engine::new();
        e.map(BLOCK_COUNT, 4);
        e.map(BLOCK_HEAD, 0x40);
        e.mem.set_u32(BLOCK_COUNT, count);
        let imports = [
            (0x00fdf1d0, "GetCurrentThreadId"),
            (0x00fdf1bc, "InterlockedCompareExchange"),
        ]
        .map(|(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".into(),
            name: name.into(),
        });
        assert_eq!(e.register_imports(&imports), 2);
        e
    }

    #[test]
    fn allocator_reloads_head_after_lock_and_keeps_return_across_unlock() {
        let mut e = fixture(4);
        let node = e.mem.alloc(16);
        e.mem.write(node, &[0xcc; 16]);
        e.mem.set_u32(node, 0x12345678);
        e.register_double(0x0040_fbf0, move |e, a| {
            assert_eq!(a, &[LOCK, 0]);
            e.mem.set_u32(FREE_HEAD, node);
            Ret::default()
        });
        e.register_double(0x0040_fba0, move |e, a| {
            assert_eq!(a, &[LOCK]);
            assert_eq!(e.mem.u32(FREE_HEAD), 0x12345678);
            assert_eq!(e.mem.bytes(node, 12), vec![0; 12]);
            e.mem.set_u32(FREE_HEAD, 0x87654321);
            Ret::default()
        });
        assert_eq!(e.call(0x0043_a010, &[0]).u32(), node);
        assert_eq!(e.mem.u32(node + 12), 0xcccccccc);
        assert_eq!(e.mem.u32(FREE_HEAD), 0x87654321);
    }

    #[test]
    fn real_registry_refills_exhausted_pool_and_chains_blocks() {
        let mut e = fixture(4);
        let first = e.call(0x0043_a010, &[0]).u32();
        let first_block = e.mem.u32(BLOCK_HEAD);
        assert_eq!(first, first_block + 12);
        assert_eq!(e.mem.u32(first_block), 0);
        assert_eq!(e.call(0x0043_a010, &[0]).u32(), first_block + 24);
        assert_eq!(e.call(0x0043_a010, &[0]).u32(), first_block + 36);
        assert_eq!(e.mem.u32(FREE_HEAD), 0);
        let fourth = e.call(0x0043_a010, &[0]).u32();
        let second_block = e.mem.u32(BLOCK_HEAD);
        assert_ne!(first_block, second_block);
        assert_eq!(fourth, second_block + 12);
        assert_eq!(e.mem.u32(second_block), first_block);
        assert_eq!(e.mem.bytes(fourth, 12), vec![0; 12]);
    }

    #[test]
    fn refill_saturates_allocation_and_reloads_count_after_callback() {
        let mut e = fixture(u32::MAX);
        let block = e.mem.alloc(40);
        e.mem.write(block, &[0xcc; 40]);
        e.mem.set_u32(BLOCK_HEAD, 0x12345678);
        e.register_double(0x0040_1000, move |e, a| {
            assert_eq!(a, &[u32::MAX]);
            e.mem.set_u32(BLOCK_COUNT, 3);
            block.into_ret()
        });
        e.call(0x0043_a080, &[]);
        assert_eq!(e.mem.u32(FREE_HEAD), block + 12);
        assert_eq!(e.mem.u32(BLOCK_HEAD), block);
        assert_eq!(e.mem.u32(block), 0x12345678);
        assert_eq!(e.mem.u32(block + 12), block + 24);
        assert_eq!(e.mem.u32(block + 24), 0);
        for offset in [4, 8, 16, 20, 28, 32, 36] {
            assert_eq!(e.mem.u32(block + offset), 0xcccccccc);
        }
    }
}
