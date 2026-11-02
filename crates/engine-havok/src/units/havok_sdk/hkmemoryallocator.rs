//! `Havok SDK/hkmemoryallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions
//! in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkMemoryAllocator` is the base class of Havok's allocators; these are its default
//! methods, all `thiscall` and all going through the object's own virtual table (slot `4`
//! allocates a block of `(bytes)`, `8` frees `(block, bytes)`, `0xc` allocates a buffer through
//! the address of the byte count, `0x10` frees a buffer, `0x24` answers whether a total can be
//! allocated). The ones that end in a `JMP` into a virtual slot are tail calls: they answer the
//! callee's `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMemUtil::memCpy` (`00d2caa0`, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00d2_caa0;

// Translated from 00cbf1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::bufAlloc` (Xbox PDB), `RET 4`: replaces its stack word (the address of
/// the requested byte count) with the byte count itself and jumps to the object's slot `4`
/// (block allocation). Answers that call's `EAX`.
pub fn hk_memory_allocator_buf_alloc(e: &mut Engine, this: Ptr, size_cell: Ptr) -> u32 {
    let size = e.mem.u32(size_cell.addr());
    e.vcall(this.addr(), 4, &args![size]).u32()
}

// Translated from 00cbf1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::bufFree` (Xbox PDB), `RET 8` (the callee's): jumps to the object's slot
/// `8` (block free) with its own stack words, `(block, bytes)`.
pub fn hk_memory_allocator_buf_free(e: &mut Engine, this: Ptr, block: u32, bytes: u32) {
    e.vcall(this.addr(), 8, &args![block, bytes]);
}

// Translated from 00cbf1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::blockAllocBatch` (Xbox PDB), `RET 0xc` `(blocks, count, bytes)`: for
/// each of the `count` entries allocates `bytes` through the object's slot `4` and stores the
/// block in the array at `blocks`.
pub fn hk_memory_allocator_block_alloc_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: Ptr,
    count: i32,
    bytes: u32,
) {
    let mut index = 0i32;
    if count > 0 {
        loop {
            let block = e.vcall(this.addr(), 4, &args![bytes]).u32();
            e.mem.set_u32(
                blocks.addr().wrapping_add((index as u32).wrapping_mul(4)),
                block,
            );
            index += 1;
            if index >= count {
                break;
            }
        }
    }
}

// Translated from 00cbf220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::blockFreeBatch` (Xbox PDB), `RET 0xc` `(blocks, count, bytes)`: frees
/// each non-null block of the array at `blocks` (`bytes` each) through the object's slot `8`.
pub fn hk_memory_allocator_block_free_batch(
    e: &mut Engine,
    this: Ptr,
    blocks: Ptr,
    count: i32,
    bytes: u32,
) {
    let mut index = 0i32;
    if count > 0 {
        loop {
            let block = e
                .mem
                .u32(blocks.addr().wrapping_add((index as u32).wrapping_mul(4)));
            if block != 0 {
                e.vcall(this.addr(), 8, &args![block, bytes]);
            }
            index += 1;
            if index >= count {
                break;
            }
        }
    }
}

// Translated from 00cbf260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::canAllocSingleBlock` (Xbox PDB), `RET 4` (the callee's): jumps to the
/// object's slot `0x24` (`canAllocTotal`) with its own stack word. Answers that call's `EAX`.
pub fn hk_memory_allocator_can_alloc_single_block(e: &mut Engine, this: Ptr, bytes: u32) -> u32 {
    e.vcall(this.addr(), 0x24, &args![bytes]).u32()
}

// Translated from 00cbf270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::canAllocTotal` (Xbox PDB), `RET 4` (the word is never read): always true.
pub fn hk_memory_allocator_can_alloc_total(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    true
}

// Translated from 00cbf280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryAllocator::bufRealloc` (Xbox PDB), `RET 0xc` `(block, old_bytes, size_cell)`:
/// allocates a buffer through the object's slot `0xc` with the address of the requested byte
/// count, copies the smaller of `old_bytes` and the (possibly changed) byte count from the old
/// block (`hkMemUtil::memCpy`), frees the old block through slot `0x10` and answers the new one.
pub fn hk_memory_allocator_buf_realloc(
    e: &mut Engine,
    this: Ptr,
    block: Ptr,
    old_bytes: i32,
    size_cell: Ptr,
) -> u32 {
    let new_block = e.vcall(this.addr(), 0xc, &args![size_cell]).u32();
    let mut copied = e.mem.i32(size_cell.addr());
    if old_bytes < copied {
        copied = old_bytes;
    }
    e.call(MEM_COPY, &args![new_block, block, copied]);
    e.vcall(this.addr(), 0x10, &args![block, old_bytes]);
    new_block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbf1b0, hk_memory_allocator_buf_alloc(Ptr, Ptr) -> u32),
        entry!(0x00cbf1d0, hk_memory_allocator_buf_free(Ptr, u32, u32)),
        entry!(
            0x00cbf1e0,
            hk_memory_allocator_block_alloc_batch(Ptr, Ptr, i32, u32)
        ),
        entry!(
            0x00cbf220,
            hk_memory_allocator_block_free_batch(Ptr, Ptr, i32, u32)
        ),
        entry!(
            0x00cbf260,
            hk_memory_allocator_can_alloc_single_block(Ptr, u32) -> u32
        ),
        entry!(
            0x00cbf270,
            hk_memory_allocator_can_alloc_total(Ptr, u32) -> bool
        ),
        entry!(
            0x00cbf280,
            hk_memory_allocator_buf_realloc(Ptr, Ptr, i32, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const VTABLE: u32 = 0x0128_3000;
    const ALLOC: u32 = 0x0900_4001;
    const FREE: u32 = 0x0900_4002;
    const BUF_ALLOC: u32 = 0x0900_4003;
    const BUF_FREE: u32 = 0x0900_4004;
    const CAN_ALLOC_TOTAL: u32 = 0x0900_4005;

    /// An allocator object whose slots are doubles: slot 4 hands out 0x1000, 0x2000, ...;
    /// slot 0xc allocates `a[1]`'s bytes; the others answer fixed values; every call is logged.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.put_vtable(
            VTABLE,
            &[
                0,
                ALLOC,
                FREE,
                BUF_ALLOC,
                BUF_FREE,
                0,
                0,
                0,
                0,
                CAN_ALLOC_TOTAL,
            ],
        );
        e.register(ALLOC, |e, _| {
            let next = e.mem.u32(0x0127_0000) + 0x1000;
            e.mem.set_u32(0x0127_0000, next);
            next.into_ret()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(BUF_ALLOC, |e, a| {
            // Grants the request (the byte count) plus 8.
            let bytes = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], bytes + 8);
            0x7000u32.into_ret()
        });
        e.register(BUF_FREE, |_, _| Ret::default());
        e.register(CAN_ALLOC_TOTAL, |_, a| (a[1] + 1).into_ret());
        e.register(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, VTABLE);
        e.call_log = Some(vec![]);
        (e, object)
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn buf_alloc_passes_the_requested_size_to_slot_4() {
        let (mut e, object) = engine();
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x123);
        assert_eq!(e.call(0x00cb_f1b0, &args![object, cell]).u32(), 0x1000);
        assert_eq!(calls(&e, ALLOC), vec![vec![object, 0x123]]);
    }

    #[test]
    fn buf_free_passes_its_words_to_slot_8() {
        let (mut e, object) = engine();
        e.call(0x00cb_f1d0, &args![object, 0x5000u32, 0x40u32]);
        assert_eq!(calls(&e, FREE), vec![vec![object, 0x5000, 0x40]]);
    }

    #[test]
    fn block_alloc_batch_fills_the_array() {
        let (mut e, object) = engine();
        let blocks = e.mem.alloc(16);
        e.call(0x00cb_f1e0, &args![object, blocks, 3u32, 0x20u32]);
        let stored: Vec<u32> = (0..4).map(|i| e.mem.u32(blocks + 4 * i)).collect();
        assert_eq!(stored, vec![0x1000, 0x2000, 0x3000, 0]);
        assert_eq!(calls(&e, ALLOC).len(), 3);
        assert_eq!(calls(&e, ALLOC)[0], vec![object, 0x20]);
        // Nothing for a count that is not positive.
        e.call(0x00cb_f1e0, &args![object, blocks, 0u32, 0x20u32]);
        e.call(0x00cb_f1e0, &args![object, blocks, (-2i32) as u32, 0x20u32]);
        assert_eq!(calls(&e, ALLOC).len(), 3);
    }

    #[test]
    fn block_free_batch_frees_the_non_null_blocks() {
        let (mut e, object) = engine();
        let blocks = e.mem.alloc(16);
        for (i, block) in [0x1000u32, 0, 0x3000].into_iter().enumerate() {
            e.mem.set_u32(blocks + 4 * i as u32, block);
        }
        e.call(0x00cb_f220, &args![object, blocks, 3u32, 0x20u32]);
        assert_eq!(
            calls(&e, FREE),
            vec![vec![object, 0x1000, 0x20], vec![object, 0x3000, 0x20]]
        );
        e.call(0x00cb_f220, &args![object, blocks, 0u32, 0x20u32]);
        assert_eq!(calls(&e, FREE).len(), 2);
    }

    #[test]
    fn can_alloc_single_block_jumps_to_can_alloc_total() {
        let (mut e, object) = engine();
        assert_eq!(e.call(0x00cb_f260, &args![object, 41u32]).u32(), 42);
        assert_eq!(calls(&e, CAN_ALLOC_TOTAL), vec![vec![object, 41]]);
    }

    #[test]
    fn can_alloc_total_is_always_true() {
        let (mut e, object) = engine();
        assert!(e.call(0x00cb_f270, &args![object, 12345u32]).bool());
    }

    #[test]
    fn buf_realloc_copies_the_smaller_size_and_frees_the_old_block() {
        let (mut e, object) = engine();
        let old = e.mem.alloc(0x40);
        e.mem.write(old, &[7u8; 0x40]);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x20);
        // The new block (0x7000 from the double) is mapped for the copy.
        e.map(0x7000, 0x1000);
        // The request is 0x20 and the grant 0x28; the old block is 0x40: copy 0x28.
        let result = e
            .call(0x00cb_f280, &args![object, old, 0x40u32, cell])
            .u32();
        assert_eq!(result, 0x7000);
        assert_eq!(calls(&e, BUF_ALLOC), vec![vec![object, cell]]);
        assert_eq!(calls(&e, MEM_COPY), vec![vec![0x7000, old, 0x28]]);
        assert_eq!(e.mem.u8(0x7000 + 0x27), 7);
        assert_eq!(e.mem.u8(0x7000 + 0x28), 0);
        assert_eq!(calls(&e, BUF_FREE), vec![vec![object, old, 0x40]]);
        // An old block smaller than the grant copies only itself.
        e.mem.set_u32(cell, 0x20);
        e.call(0x00cb_f280, &args![object, old, 0x10u32, cell]);
        assert_eq!(calls(&e, MEM_COPY)[1], vec![0x7000, old, 0x10]);
    }
}
