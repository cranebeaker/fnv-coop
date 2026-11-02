//! `NiXenonRenderer/nivbset.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A vertex-buffer set (`NiVBSet`, Xbox PDB, 0x34 bytes) keeps its blocks (`NiVBBlock`, 0x44
//! bytes) in an array (`m_apkBlocks`, an `NiTPrimitiveArray` at +0x1C whose data pointer is at
//! +0x20) and in a list ordered by free space (`m_pkBlockList`, +0x2C), linked through each
//! block's `m_pkNextBlock` (+0x3C) and `m_pkPrevBlock` (+0x40). The offsets here are the PC
//! code's and match the PDB.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiVBSet::m_apkBlocks` (Xbox PDB) +0x1C, its data pointer at +0x20, and
/// `NiVBSet::m_pkBlockList` +0x2C.
const SET_BLOCKS: u32 = 0x1c;
const SET_BLOCKS_DATA: u32 = 0x20;
const SET_BLOCK_LIST: u32 = 0x2c;
/// `NiVBBlock` (Xbox PDB): `m_pkVB` +0x08, `m_uiSize` +0x0C, `m_uiFreeBytes` +0x24,
/// `m_uiTotalFreeBytes` +0x28, `m_pkNextBlock` +0x3C, `m_pkPrevBlock` +0x40.
const BLOCK_VB: u32 = 0x08;
const BLOCK_SIZE: u32 = 0x0c;
const BLOCK_FREE_BYTES: u32 = 0x24;
const BLOCK_TOTAL_FREE_BYTES: u32 = 0x28;
const BLOCK_NEXT: u32 = 0x3c;
const BLOCK_PREVIOUS: u32 = 0x40;
/// The head of the list released blocks are kept on (`0126fa30`; the Xbox PDB's
/// `NiVBSet::ms_pkBlockDump` is the class's one static block pointer, the address is not
/// confirmed by a name).
const BLOCK_DUMP: u32 = 0x0126_fa30;
/// `00e942d0` (`thiscall` on the block, no stack words; unnamed in the engine map): run on a
/// block before it goes to the dump.
const BLOCK_RELEASE_00E942D0: u32 = 0x00e9_42d0;
/// `00e98500` (`thiscall` on the block, one word: the chip's index; unnamed in the engine map):
/// gives the chip back to its block.
const BLOCK_FREE_CHIP_00E98500: u32 = 0x00e9_8500;
/// `00e94220` (`thiscall` on the set, one word: the block; unnamed in the engine map): puts the
/// block back into the set's list by its free bytes.
const SET_INSERT_BLOCK_00E94220: u32 = 0x00e9_4220;
/// `NiTArray<NiVBDynamicSet*>::RemoveAt` (`00a93c50`, engine map, a folded body; `thiscall` on
/// the array, one word: the index).
const ARRAY_REMOVE_AT: u32 = 0x00a9_3c50;

// Translated from 00e94150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the set, one word): when `block` is in the set's
/// block list (+0x2C, followed through `m_pkNextBlock`), unlinks it: its neighbours are
/// joined, and the list head moves to its next block when it was the head. A block not found
/// leaves everything as it was.
pub fn fn_00e94150(e: &mut Engine, this: Ptr, block: u32) {
    let set = this.addr();
    let mut current = e.mem.u32(set + SET_BLOCK_LIST);
    if current == 0 {
        return;
    }
    while current != block {
        current = e.mem.u32(current + BLOCK_NEXT);
        if current == 0 {
            return;
        }
    }
    let next = e.mem.u32(current + BLOCK_NEXT);
    if next != 0 {
        let previous = e.mem.u32(current + BLOCK_PREVIOUS);
        e.mem.set_u32(next + BLOCK_PREVIOUS, previous);
    }
    let previous = e.mem.u32(current + BLOCK_PREVIOUS);
    if previous != 0 {
        let next = e.mem.u32(current + BLOCK_NEXT);
        e.mem.set_u32(previous + BLOCK_NEXT, next);
    }
    if current == e.mem.u32(set + SET_BLOCK_LIST) {
        let next = e.mem.u32(current + BLOCK_NEXT);
        e.mem.set_u32(set + SET_BLOCK_LIST, next);
    }
}

// Translated from 00e94330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; `ECX` is not read): for a block that is
/// not null, releases its vertex buffer (+0x08, slot +8, `stdcall`) and clears it, runs
/// `00e942d0` on it, unlinks it from its neighbours and pushes it on the front of the
/// released-block list (`0126fa30`).
pub fn fn_00e94330(e: &mut Engine, _this: Ptr, block: u32) {
    if block == 0 {
        return;
    }
    let vertex_buffer = e.mem.u32(block + BLOCK_VB);
    if vertex_buffer != 0 {
        e.vcall(vertex_buffer, 8, &[]);
        e.mem.set_u32(block + BLOCK_VB, 0);
    }
    e.call(BLOCK_RELEASE_00E942D0, &[block]);
    let next = e.mem.u32(block + BLOCK_NEXT);
    let previous = e.mem.u32(block + BLOCK_PREVIOUS);
    if next != 0 {
        e.mem.set_u32(next + BLOCK_PREVIOUS, previous);
    }
    if previous != 0 {
        e.mem.set_u32(previous + BLOCK_NEXT, next);
    }
    let dump = e.global::<u32>(BLOCK_DUMP);
    if dump != 0 {
        e.mem.set_u32(dump + BLOCK_PREVIOUS, block);
    }
    let dump = e.global::<u32>(BLOCK_DUMP);
    e.mem.set_u32(block + BLOCK_NEXT, dump);
    e.mem.set_u32(block + BLOCK_PREVIOUS, 0);
    e.set_global(BLOCK_DUMP, block);
}

// Translated from 00e94490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiVBSet::FreeChip` (Xbox PDB, engine map), `thiscall`, two words (the block index and the
/// chip's index, `m_uiIndex`). With a block at that index, gives the chip back to it (`00e98500`); when that
/// changed the block's free bytes (+0x24), the block leaves the set's list (`00e94150`) and
/// then, when its total free bytes (+0x28) equal its size (+0x0C), is removed from the block
/// array (`RemoveAt` on +0x1C) and released (`00e94330`); otherwise it goes back into the list
/// (`00e94220`, which gets the set in `ECX` as `00e94150` left it).
pub fn ni_vb_set_free_chip(e: &mut Engine, this: Ptr, index: u32, chip_index: u32) {
    let set = this.addr();
    let blocks = e.mem.u32(set + SET_BLOCKS_DATA);
    let block = e.mem.u32(blocks.wrapping_add(index.wrapping_mul(4)));
    if block == 0 {
        return;
    }
    let free_before = e.mem.u32(block + BLOCK_FREE_BYTES);
    e.call(BLOCK_FREE_CHIP_00E98500, &[block, chip_index]);
    if free_before == e.mem.u32(block + BLOCK_FREE_BYTES) {
        return;
    }
    fn_00e94150(e, this, block);
    if e.mem.u32(block + BLOCK_TOTAL_FREE_BYTES) == e.mem.u32(block + BLOCK_SIZE) {
        e.call(ARRAY_REMOVE_AT, &[set + SET_BLOCKS, index]);
        fn_00e94330(e, this, block);
        return;
    }
    e.call(SET_INSERT_BLOCK_00E94220, &[set, block]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e9_4150, fn_00e94150(Ptr, u32)),
        entry!(0x00e9_4330, fn_00e94330(Ptr, u32)),
        entry!(0x00e9_4490, ni_vb_set_free_chip(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Blocks linked in order (next and previous), each 0x44 bytes.
    fn chain(e: &mut Engine, count: usize) -> Vec<u32> {
        let blocks: Vec<u32> = (0..count).map(|_| e.mem.alloc(0x44)).collect();
        for pair in blocks.windows(2) {
            e.mem.set_u32(pair[0] + BLOCK_NEXT, pair[1]);
            e.mem.set_u32(pair[1] + BLOCK_PREVIOUS, pair[0]);
        }
        blocks
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn unlink_removes_a_listed_block() {
        let mut e = Engine::new();
        let set = e.mem.alloc(0x34);
        let b = chain(&mut e, 3);
        e.mem.set_u32(set + SET_BLOCK_LIST, b[0]);
        // A block not in the list: nothing changes.
        let stranger = e.mem.alloc(0x44);
        e.call(0x00e9_4150, &[set, stranger]);
        assert_eq!(e.mem.u32(b[0] + BLOCK_NEXT), b[1]);
        // The middle one.
        e.call(0x00e9_4150, &[set, b[1]]);
        assert_eq!(e.mem.u32(b[0] + BLOCK_NEXT), b[2]);
        assert_eq!(e.mem.u32(b[2] + BLOCK_PREVIOUS), b[0]);
        // The head moves.
        e.call(0x00e9_4150, &[set, b[0]]);
        assert_eq!(e.mem.u32(set + SET_BLOCK_LIST), b[2]);
        assert_eq!(e.mem.u32(b[2] + BLOCK_PREVIOUS), 0);
        // An empty list.
        e.mem.set_u32(set + SET_BLOCK_LIST, 0);
        e.call(0x00e9_4150, &[set, b[2]]);
        assert_eq!(e.mem.u32(set + SET_BLOCK_LIST), 0);
    }

    #[test]
    fn release_block_moves_it_to_the_dump() {
        let mut e = Engine::new();
        e.register(BLOCK_RELEASE_00E942D0, |_, _| Ret::default());
        e.register(0x0066_0008, |_, _| Ret::default());
        e.map(0x0126_f000, 0x1000);
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, 0x0066_0008);
        let vertex_buffer = e.mem.alloc(4);
        e.mem.set_u32(vertex_buffer, vtable);
        let b = chain(&mut e, 3);
        e.mem.set_u32(b[1] + BLOCK_VB, vertex_buffer);
        let old_dump = e.mem.alloc(0x44);
        e.set_global(BLOCK_DUMP, old_dump);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4330, &[0, b[1]]);
        assert_eq!(calls(&e, 0x0066_0008), vec![vec![vertex_buffer]]);
        assert_eq!(calls(&e, BLOCK_RELEASE_00E942D0), vec![vec![b[1]]]);
        assert_eq!(e.mem.u32(b[1] + BLOCK_VB), 0);
        assert_eq!(e.mem.u32(b[0] + BLOCK_NEXT), b[2]);
        assert_eq!(e.mem.u32(b[2] + BLOCK_PREVIOUS), b[0]);
        assert_eq!(e.global::<u32>(BLOCK_DUMP), b[1]);
        assert_eq!(e.mem.u32(b[1] + BLOCK_NEXT), old_dump);
        assert_eq!(e.mem.u32(b[1] + BLOCK_PREVIOUS), 0);
        assert_eq!(e.mem.u32(old_dump + BLOCK_PREVIOUS), b[1]);
        // An empty dump and no vertex buffer; a null block does nothing.
        e.set_global(BLOCK_DUMP, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4330, &[0, b[0]]);
        assert!(calls(&e, 0x0066_0008).is_empty());
        assert_eq!(e.mem.u32(b[0] + BLOCK_NEXT), 0);
        e.call(0x00e9_4330, &[0, 0]);
        assert_eq!(e.global::<u32>(BLOCK_DUMP), b[0]);
    }

    /// A set with one block at index 1 whose chip release frees `freed` bytes.
    fn set_with_block(e: &mut Engine, freed: u32) -> (u32, u32) {
        e.register_double(BLOCK_FREE_CHIP_00E98500, move |e, a| {
            let free = e.mem.u32(a[0] + BLOCK_FREE_BYTES) + freed;
            e.mem.set_u32(a[0] + BLOCK_FREE_BYTES, free);
            Ret::default()
        });
        e.register(ARRAY_REMOVE_AT, |_, _| Ret::default());
        e.register(SET_INSERT_BLOCK_00E94220, |_, _| Ret::default());
        e.register(BLOCK_RELEASE_00E942D0, |_, _| Ret::default());
        e.map(0x0126_f000, 0x1000);
        let set = e.mem.alloc(0x34);
        let array = e.mem.alloc(8);
        let block = e.mem.alloc(0x44);
        e.mem.set_u32(array + 4, block);
        e.mem.set_u32(set + SET_BLOCKS_DATA, array);
        e.mem.set_u32(set + SET_BLOCK_LIST, block);
        e.mem.set_u32(block + BLOCK_SIZE, 0x100);
        (set, block)
    }

    #[test]
    fn free_chip_reinserts_a_partly_free_block() {
        let mut e = Engine::new();
        let (set, block) = set_with_block(&mut e, 0x10);
        e.mem.set_u32(block + BLOCK_TOTAL_FREE_BYTES, 0x80);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4490, &[set, 1, 0x5000]);
        assert_eq!(
            calls(&e, BLOCK_FREE_CHIP_00E98500),
            vec![vec![block, 0x5000]]
        );
        assert_eq!(e.mem.u32(set + SET_BLOCK_LIST), 0);
        assert_eq!(calls(&e, SET_INSERT_BLOCK_00E94220), vec![vec![set, block]]);
        assert!(calls(&e, ARRAY_REMOVE_AT).is_empty());
    }

    #[test]
    fn free_chip_releases_an_empty_block() {
        let mut e = Engine::new();
        let (set, block) = set_with_block(&mut e, 0x10);
        e.mem.set_u32(block + BLOCK_TOTAL_FREE_BYTES, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4490, &[set, 1, 0x5000]);
        assert_eq!(calls(&e, ARRAY_REMOVE_AT), vec![vec![set + SET_BLOCKS, 1]]);
        assert_eq!(calls(&e, BLOCK_RELEASE_00E942D0), vec![vec![block]]);
        assert_eq!(e.global::<u32>(BLOCK_DUMP), block);
        assert!(calls(&e, SET_INSERT_BLOCK_00E94220).is_empty());
    }

    #[test]
    fn free_chip_without_a_change_or_a_block_stops() {
        let mut e = Engine::new();
        let (set, block) = set_with_block(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e9_4490, &[set, 1, 0x5000]);
        assert_eq!(e.mem.u32(set + SET_BLOCK_LIST), block);
        assert!(calls(&e, SET_INSERT_BLOCK_00E94220).is_empty());
        e.call(0x00e9_4490, &[set, 0, 0x5000]);
        assert_eq!(calls(&e, BLOCK_FREE_CHIP_00E98500).len(), 1);
    }
}
