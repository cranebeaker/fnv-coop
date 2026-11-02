//! `Havok SDK/hkpdynamicsjobqueueutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its `+0x10` is the allocator whose slot `4` allocates.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

/// Allocates `size` bytes through the memory router's allocator.
fn allocate(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

// Translated from 00d96530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAgentSectorHeader::allocate` (Ghidra name), a `cdecl` function of two words: a count and
/// a sector size in units of 16 bytes. Allocates a block of `4 * (count as a signed 16-bit
/// value) + 12` bytes whose words are the count (twice) and `16 * size + 0x20`, followed by
/// the pointers to `count` sectors, each `16 * size + 0x20` bytes. A sector that was allocated
/// gets the header `{0x10, 0, 16 * size}` at its `+0x10`. A sector pointer is 0 when its
/// allocation failed. Returns the block.
pub fn hkp_agent_sector_header_allocate(e: &mut Engine, count: i32, size: u32) -> Ptr {
    let block = allocate(e, (i32::from(count as i16) * 4 + 0xc) as u32);
    let sector_size = size << 4;
    e.mem.set_u32(block + 4, count as u32);
    e.mem.set_u32(block, count as u32);
    e.mem.set_u32(block + 8, sector_size + 0x20);
    for index in 0..count.max(0) as u32 {
        let sector = allocate(e, sector_size + 0x20);
        e.mem.set_u32(block + 0xc + 4 * index, sector);
        if sector != 0 {
            let header = sector + 0x10;
            e.mem.set_u32(header, 0x10);
            e.mem.set_u32(header + 4, 0);
            e.mem.set_u32(header + 8, sector_size);
        }
    }
    Ptr::new(block)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d96530,
        hkp_agent_sector_header_allocate(i32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A router whose allocator hands out zeroed blocks and records each size; the
    /// `refuse`-th request (counting from 0) fails with 0.
    fn allocator(e: &mut Engine, refuse: usize) -> Rc<RefCell<Vec<u32>>> {
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, object);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&sizes);
        e.register_double(0x0900_0004, move |e, a| {
            let mut sizes = sink.borrow_mut();
            let refused = sizes.len() == refuse;
            sizes.push(a[1]);
            Ret {
                eax: if refused { 0 } else { e.mem.alloc(a[1]) },
                ..Ret::default()
            }
        });
        sizes
    }

    #[test]
    fn allocate_builds_the_header_block_and_the_sectors() {
        let mut e = engine();
        let sizes = allocator(&mut e, usize::MAX);
        let block = e.call(0x00d9_6530, &args![3u32, 2u32]).u32();
        // 3 sectors of 2 * 16 + 0x20 bytes after the 3 * 4 + 12 byte block.
        assert_eq!(*sizes.borrow(), vec![24, 64, 64, 64]);
        assert_eq!(e.mem.u32(block), 3);
        assert_eq!(e.mem.u32(block + 4), 3);
        assert_eq!(e.mem.u32(block + 8), 64);
        for index in 0..3 {
            let sector = e.mem.u32(block + 0xc + 4 * index);
            assert_ne!(sector, 0);
            assert_eq!(e.mem.u32(sector + 0x10), 0x10);
            assert_eq!(e.mem.u32(sector + 0x14), 0);
            assert_eq!(e.mem.u32(sector + 0x18), 32);
        }
    }

    #[test]
    fn a_failed_sector_allocation_leaves_a_null_pointer() {
        let mut e = engine();
        // Request 0 is the block, request 2 is the second sector.
        let sizes = allocator(&mut e, 2);
        let block = e.call(0x00d9_6530, &args![3u32, 1u32]).u32();
        assert_eq!(sizes.borrow().len(), 4);
        assert_ne!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_ne!(e.mem.u32(block + 0x14), 0);
    }

    #[test]
    fn a_count_of_zero_or_less_allocates_only_the_block() {
        let mut e = engine();
        let sizes = allocator(&mut e, usize::MAX);
        let block = e.call(0x00d9_6530, &args![0u32, 1u32]).u32();
        assert_eq!(*sizes.borrow(), vec![12]);
        assert_eq!(e.mem.u32(block), 0);
        // The block size comes from the low 16 bits of the count, sign-extended: -1 gives 8.
        let negative = e.call(0x00d9_6530, &args![-1i32, 1u32]);
        assert_eq!(sizes.borrow()[1], 8);
        assert_eq!(e.mem.u32(negative.u32()), 0xffff_ffff);
    }
}
