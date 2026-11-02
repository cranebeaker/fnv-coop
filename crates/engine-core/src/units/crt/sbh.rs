//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the small-block heap of LIBCMT/sbheap.c, `___sbh_free_block`
//! (`00ede263`), `___sbh_alloc_new_region` (`00ede579`), `___sbh_resize_block` (`00ede731`) and
//! `___sbh_alloc_block` (`00edea12`), and the wide stream opener Ghidra names
//! `_tmpfile_helper` (`00ecef9e`, the shape of `_wfsopen`).
//!
//! The small-block heap's structures (as `__sbh_find_block` and `__sbh_alloc_new_group` in
//! `misc7.rs` use them): the region headers (`__sbh_pHeaderList`, `__sbh_cntHeaderList` of
//! them, room for `__sbh_sizeHeaderList`), 0x14 bytes each: `+0` and `+4` the masks of the
//! free lists any group of the region has blocks on (lists 0 to 31 from bit 31 of `+0`, lists
//! 32 to 63 from bit 31 of `+4`), `+8` the mask of the region's uncommitted groups, `+0xC` the
//! region's base (1 MB reserved, 32 groups of 0x8000 bytes) and `+0x10` its data (0x41C4
//! bytes: `+0` the last group used, `+4` 64 counts of groups with blocks on each list, `+0x43`
//! the count of committed groups, `+0x44` and `+0xC4` each group's list masks as in the header,
//! `+0x144` the groups' data, 0x204 bytes each: a count of used blocks, then 64 list heads of
//! 8 bytes `{forward, back}` at `+4` and `+8`). A block has its size at `-4` (bit 0 set when in
//! use) and again in its last word; a free block links into the list of its size
//! (`size / 16 - 1`, at most 63) through `+4` (forward) and `+8` (back).
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__sbh_cntHeaderList` (`01270930`), `__sbh_pHeaderList` (`01270934`), `__sbh_pHeaderScan`
/// (`0127093c`, where `___sbh_alloc_block`'s search starts), `__sbh_sizeHeaderList`
/// (`01270940`), `__sbh_pHeaderDefer` (`01270734`) and `__sbh_indGroupDefer` (`01270944`, the
/// emptied group kept committed until another empties) and `_crtheap` (`012705bc`).
const HEADER_COUNT: u32 = 0x0127_0930;
const HEADER_LIST: u32 = 0x0127_0934;
const HEADER_SCAN: u32 = 0x0127_093c;
const HEADER_SIZE: u32 = 0x0127_0940;
const DEFER_HEADER: u32 = 0x0127_0734;
const DEFER_GROUP: u32 = 0x0127_0944;
const CRT_HEAP: u32 = 0x0127_05bc;
/// The KERNEL32 import slots (stdcall): `VirtualFree` (`00fdf10c`: address, size, type),
/// `VirtualAlloc` (`00fdf114`: address, size, type, protection), `HeapReAlloc` (`00fdf19c`:
/// heap, flags, block, size), `HeapAlloc` (`00fdf1a0`: heap, flags, size) and `HeapFree`
/// (`00fdf1a4`: heap, flags, block).
const VIRTUAL_FREE: u32 = 0x00fd_f10c;
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
const HEAP_REALLOC: u32 = 0x00fd_f19c;
const HEAP_ALLOC: u32 = 0x00fd_f1a0;
const HEAP_FREE: u32 = 0x00fd_f1a4;
/// `memmove` (`00ec7230`: destination, source, count), `___sbh_alloc_new_region` (`00ede579`,
/// no arguments) and `__sbh_alloc_new_group` (`00ede629`: the header), cdecl.
const MEMMOVE: u32 = 0x00ec_7230;
const SBH_ALLOC_NEW_REGION: u32 = 0x00ed_e579;
const SBH_ALLOC_NEW_GROUP: u32 = 0x00ed_e629;
/// `_errno` (`00ec85e3`), `_invalid_parameter` (`00ec7c30`, five words), `_getstream`
/// (`00ed8501`, a free stream, locked, or 0), `_wopenfile` (`00edfbfa`: name, mode, share flag,
/// stream) and the `__finally` block of `_tmpfile_helper` (`00ecf05a`: the stream), cdecl.
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const GETSTREAM: u32 = 0x00ed_8501;
const WOPENFILE: u32 = 0x00ed_fbfa;
const TMPFILE_HELPER_FINALLY: u32 = 0x00ec_f05a;

/// The free list of a block size, as the free and resize code computes it: `size / 16 - 1`
/// (arithmetic shift), at most 63 compared unsigned (a size below 16 gives 63).
fn list_of(size: u32) -> u32 {
    let list = ((size as i32) >> 4).wrapping_sub(1) as u32;
    if list > 0x3f {
        0x3f
    } else {
        list
    }
}

/// The bit of `list` in its mask word: `0x80000000 >> list` for lists below 32, else `>> (list
/// - 32)` (`SHR` takes the count's low five bits).
fn list_bit(list: u32) -> u32 {
    0x8000_0000u32 >> (list.wrapping_sub(if list < 0x20 { 0 } else { 0x20 }) & 0x1f)
}

/// The mask word of `list` at `base`: `+0` for lists below 32, `+4` (or `+0x80` from a group's
/// `+0x44`, the `+0xC4` word) for the others.
fn mask_word(base: u32, list: u32, high_offset: u32) -> u32 {
    if list < 0x20 {
        base
    } else {
        base.wrapping_add(high_offset)
    }
}

/// A free block about to leave the last place on its list: the list's bit is cleared in the
/// group's mask (`region + 0x44 + group * 4`, or `+0xC4`), the region's count of groups with
/// blocks on it (the byte at `region + 4 + list`) counted down, and when that reaches 0 the
/// bit is cleared in the header's mask (`+0` or `+4`).
fn clear_list_bit(e: &mut Engine, header: u32, region: u32, group: u32, list: u32) {
    let bits = !list_bit(list);
    let word = mask_word(
        region
            .wrapping_add(group.wrapping_mul(4))
            .wrapping_add(0x44),
        list,
        0x80,
    );
    let masked = e.mem.u32(word) & bits;
    e.mem.set_u32(word, masked);
    let count_at = region.wrapping_add(list).wrapping_add(4);
    let count = e.mem.u8(count_at).wrapping_sub(1);
    e.mem.set_u8(count_at, count);
    if count == 0 {
        let word = mask_word(header, list, 4);
        let masked = e.mem.u32(word) & bits;
        e.mem.set_u32(word, masked);
    }
}

/// A free block taken off its list: its back neighbour's forward link becomes its forward
/// link, then its forward neighbour's back link its back link.
fn unlink(e: &mut Engine, block: u32) {
    let back = e.mem.u32(block.wrapping_add(8));
    let forward = e.mem.u32(block.wrapping_add(4));
    e.mem.set_u32(back.wrapping_add(4), forward);
    let forward = e.mem.u32(block.wrapping_add(4));
    let back = e.mem.u32(block.wrapping_add(8));
    e.mem.set_u32(forward.wrapping_add(8), back);
}

/// A free block put at the end of `list` (the head at `group_data + list * 8`): back link the
/// head, forward link the head's old last block, the head's last link the block, the old
/// last block's back link the block. When it is the list's only block the region's count byte
/// is counted up (setting the header's bit when it was 0) and the group's bit set.
fn link_free(
    e: &mut Engine,
    header: u32,
    region: u32,
    group: u32,
    group_data: u32,
    block: u32,
    list: u32,
) {
    let head = group_data.wrapping_add(list.wrapping_mul(8));
    let last = e.mem.u32(head.wrapping_add(4));
    e.mem.set_u32(block.wrapping_add(8), head);
    e.mem.set_u32(block.wrapping_add(4), last);
    e.mem.set_u32(head.wrapping_add(4), block);
    let last = e.mem.u32(block.wrapping_add(4));
    e.mem.set_u32(last.wrapping_add(8), block);
    if e.mem.u32(block.wrapping_add(4)) != e.mem.u32(block.wrapping_add(8)) {
        return;
    }
    let count_at = region.wrapping_add(list).wrapping_add(4);
    let count = e.mem.u8(count_at);
    e.mem.set_u8(count_at, count.wrapping_add(1));
    let bit = list_bit(list);
    if count == 0 {
        let word = mask_word(header, list, 4);
        let set = e.mem.u32(word) | bit;
        e.mem.set_u32(word, set);
    }
    let word = mask_word(
        region
            .wrapping_add(group.wrapping_mul(4))
            .wrapping_add(0x44),
        list,
        0x80,
    );
    let set = e.mem.u32(word) | bit;
    e.mem.set_u32(word, set);
}

/// The size word at the start (`-4` of the user's address) and in the last word of a block.
fn set_sizes(e: &mut Engine, entry: u32, size: u32) {
    e.mem.set_u32(entry, size);
    e.mem
        .set_u32(size.wrapping_add(entry).wrapping_sub(4), size);
}

// Translated from 00ede263 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___sbh_free_block` (LIBCMT/sbheap.c; cdecl: the region header, the block). A block whose
/// size word (minus 1) has bit 0 set is not in use and is left alone. A free block after it is
/// taken off its list (clearing the list's bits when it was the only one) and joined; a free
/// block before it (the word below the size) likewise, but taken off its list only when the
/// joined size belongs on another list. The block is linked on its list unless it joined a
/// free block that stays on the same one, and gets its size at both ends. When the group's
/// count of used blocks reaches 0: a group kept from before (`__sbh_pHeaderDefer`, its index
/// in `__sbh_indGroupDefer`) is decommitted (`VirtualFree(base + index * 0x8000, 0x8000,
/// MEM_DECOMMIT)`, its bit set in the header's uncommitted mask, its region's list masks
/// (`+0xC4`) cleared and committed count counted down, clearing bit 0 of the header's `+4` at
/// 0); a region with every group uncommitted is released (`VirtualFree(base, 0, MEM_RELEASE)`,
/// `HeapFree` of its data), its header removed from the list (`memmove` of the ones after it,
/// the count down, the given header moving down with them when it was after it) and the scan
/// start reset to the list. This group is then the kept one.
pub fn sbh_free_block(e: &mut Engine, header: u32, block: u32) {
    let region = e.mem.u32(header.wrapping_add(0x10));
    let group = block.wrapping_sub(e.mem.u32(header.wrapping_add(0xc))) >> 15;
    let group_data = group
        .wrapping_mul(0x204)
        .wrapping_add(region)
        .wrapping_add(0x144);
    let mut entry = block.wrapping_sub(4);
    let mut size = e.mem.u32(entry).wrapping_sub(1);
    if size & 1 != 0 {
        return;
    }
    let next = size.wrapping_add(entry);
    let next_size = e.mem.u32(next);
    let prev_size = e.mem.u32(entry.wrapping_sub(4));
    if next_size & 1 == 0 {
        let list = list_of(next_size);
        if e.mem.u32(next.wrapping_add(4)) == e.mem.u32(next.wrapping_add(8)) {
            clear_list_bit(e, header, region, group, list);
        }
        unlink(e, next);
        size = size.wrapping_add(next_size);
    }
    let mut list = list_of(size);
    let prev_used = prev_size & 1 != 0;
    let mut old = 0;
    if !prev_used {
        entry = entry.wrapping_sub(prev_size);
        old = list_of(prev_size);
        size = size.wrapping_add(prev_size);
        list = list_of(size);
        if old != list {
            if e.mem.u32(entry.wrapping_add(4)) == e.mem.u32(entry.wrapping_add(8)) {
                clear_list_bit(e, header, region, group, old);
            }
            unlink(e, entry);
        }
    }
    if prev_used || old != list {
        link_free(e, header, region, group, group_data, entry, list);
    }
    set_sizes(e, entry, size);
    let used = e.mem.u32(group_data).wrapping_sub(1);
    e.mem.set_u32(group_data, used);
    if used != 0 {
        return;
    }
    let mut header = header;
    let defer = e.mem.u32(DEFER_HEADER);
    if defer != 0 {
        let index = e.mem.u32(DEFER_GROUP);
        let at = (index << 15).wrapping_add(e.mem.u32(defer.wrapping_add(0xc)));
        e.call(VIRTUAL_FREE, &args![at, 0x8000u32, 0x4000u32]);
        let index = e.mem.u32(DEFER_GROUP);
        let defer = e.mem.u32(DEFER_HEADER);
        let groups = e.mem.u32(defer.wrapping_add(8)) | (0x8000_0000u32 >> (index & 0x1f));
        e.mem.set_u32(defer.wrapping_add(8), groups);
        let data = e.mem.u32(e.mem.u32(DEFER_HEADER).wrapping_add(0x10));
        let index = e.mem.u32(DEFER_GROUP);
        e.mem.set_u32(
            data.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xc4),
            0,
        );
        let data = e.mem.u32(e.mem.u32(DEFER_HEADER).wrapping_add(0x10));
        let committed = e.mem.u8(data.wrapping_add(0x43)).wrapping_sub(1);
        e.mem.set_u8(data.wrapping_add(0x43), committed);
        let defer = e.mem.u32(DEFER_HEADER);
        let data = e.mem.u32(defer.wrapping_add(0x10));
        if e.mem.u8(data.wrapping_add(0x43)) == 0 {
            let word = e.mem.u32(defer.wrapping_add(4)) & 0xffff_fffe;
            e.mem.set_u32(defer.wrapping_add(4), word);
        }
        let defer = e.mem.u32(DEFER_HEADER);
        if e.mem.u32(defer.wrapping_add(8)) == u32::MAX {
            let base = e.mem.u32(defer.wrapping_add(0xc));
            e.call(VIRTUAL_FREE, &args![base, 0u32, 0x8000u32]);
            let defer = e.mem.u32(DEFER_HEADER);
            let heap = e.mem.u32(CRT_HEAP);
            let data = e.mem.u32(defer.wrapping_add(0x10));
            e.call(HEAP_FREE, &args![heap, 0u32, data]);
            let defer = e.mem.u32(DEFER_HEADER);
            let bytes = e
                .mem
                .u32(HEADER_COUNT)
                .wrapping_mul(0x14)
                .wrapping_sub(defer)
                .wrapping_add(e.mem.u32(HEADER_LIST))
                .wrapping_sub(0x14);
            e.call(MEMMOVE, &args![defer, defer.wrapping_add(0x14), bytes]);
            let count = e.mem.u32(HEADER_COUNT).wrapping_sub(1);
            e.mem.set_u32(HEADER_COUNT, count);
            if header > e.mem.u32(DEFER_HEADER) {
                header = header.wrapping_sub(0x14);
            }
            let list = e.mem.u32(HEADER_LIST);
            e.mem.set_u32(HEADER_SCAN, list);
        }
    }
    e.mem.set_u32(DEFER_HEADER, header);
    e.mem.set_u32(DEFER_GROUP, group);
}

// Translated from 00ede579 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___sbh_alloc_new_region` (LIBCMT/sbheap.c; no arguments). A full header list
/// (`__sbh_cntHeaderList` equal to `__sbh_sizeHeaderList`) grows by 16 headers (`HeapReAlloc`
/// of `_crtheap`, flags 0; failing answers 0). The next header gets its data (`HeapAlloc`
/// with `HEAP_ZERO_MEMORY`, 0x41C4 bytes) and its 1 MB of address space (`VirtualAlloc(0,
/// 0x100000, MEM_RESERVE, PAGE_READWRITE)`; failing frees the data); either failing answers 0.
/// The header's uncommitted mask becomes all ones, its list masks 0, the count goes up and the
/// data's last group (`+0`) becomes -1. Answers the header.
pub fn sbh_alloc_new_region(e: &mut Engine) -> u32 {
    let size = e.mem.u32(HEADER_SIZE);
    let mut count = e.mem.u32(HEADER_COUNT);
    if count == size {
        let heap = e.mem.u32(CRT_HEAP);
        let list = e.mem.u32(HEADER_LIST);
        let bytes = size.wrapping_add(0x10).wrapping_mul(0x14);
        let grown = e.call(HEAP_REALLOC, &args![heap, 0u32, list, bytes]).u32();
        if grown == 0 {
            return 0;
        }
        let size = e.mem.u32(HEADER_SIZE).wrapping_add(0x10);
        e.mem.set_u32(HEADER_SIZE, size);
        count = e.mem.u32(HEADER_COUNT);
        e.mem.set_u32(HEADER_LIST, grown);
    }
    let header = count
        .wrapping_mul(0x14)
        .wrapping_add(e.mem.u32(HEADER_LIST));
    let heap = e.mem.u32(CRT_HEAP);
    let data = e.call(HEAP_ALLOC, &args![heap, 8u32, 0x41c4u32]).u32();
    e.mem.set_u32(header.wrapping_add(0x10), data);
    if data == 0 {
        return 0;
    }
    let base = e
        .call(VIRTUAL_ALLOC, &args![0u32, 0x10_0000u32, 0x2000u32, 4u32])
        .u32();
    e.mem.set_u32(header.wrapping_add(0xc), base);
    if base == 0 {
        let heap = e.mem.u32(CRT_HEAP);
        let data = e.mem.u32(header.wrapping_add(0x10));
        e.call(HEAP_FREE, &args![heap, 0u32, data]);
        return 0;
    }
    e.mem.set_u32(header.wrapping_add(8), u32::MAX);
    e.mem.set_u32(header, 0);
    e.mem.set_u32(header.wrapping_add(4), 0);
    let count = e.mem.u32(HEADER_COUNT).wrapping_add(1);
    e.mem.set_u32(HEADER_COUNT, count);
    let data = e.mem.u32(header.wrapping_add(0x10));
    e.mem.set_u32(data, u32::MAX);
    header
}

// Translated from 00ede731 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___sbh_resize_block` (LIBCMT/sbheap.c; cdecl: the region header, the block, the new
/// size). The size needed is the new size plus the size word, rounded up to 16
/// (`(size + 0x17) & ~0xF`). Growing (signed compares) needs the block after to be free and
/// big enough together (else 0): it is taken off its list and what is left over, when above 0,
/// becomes a free block after the grown one, linked on its list. Shrinking makes the rest a
/// free block, joined with a free block after it (taken off its list), and links it. An equal
/// size changes nothing. The block gets its new size (with the used bit) at both ends; answers
/// 1.
pub fn sbh_resize_block(e: &mut Engine, header: u32, block: u32, new_size: u32) -> u32 {
    let region = e.mem.u32(header.wrapping_add(0x10));
    let group = block.wrapping_sub(e.mem.u32(header.wrapping_add(0xc))) >> 15;
    let group_data = group
        .wrapping_mul(0x204)
        .wrapping_add(region)
        .wrapping_add(0x144);
    let old = e.mem.u32(block.wrapping_sub(4)).wrapping_sub(1);
    let need = new_size.wrapping_add(0x17) & 0xffff_fff0;
    let next = old.wrapping_add(block).wrapping_sub(4);
    let next_size = e.mem.u32(next);
    if (need as i32) > (old as i32) {
        if next_size & 1 != 0 {
            return 0;
        }
        if (need as i32) > (next_size.wrapping_add(old) as i32) {
            return 0;
        }
        let list = list_of(next_size);
        if e.mem.u32(next.wrapping_add(4)) == e.mem.u32(next.wrapping_add(8)) {
            clear_list_bit(e, header, region, group, list);
        }
        unlink(e, next);
        let rest = next_size.wrapping_add(old.wrapping_sub(need));
        if (rest as i32) > 0 {
            let free = block.wrapping_add(need).wrapping_sub(4);
            let list = list_of(rest);
            link_free(e, header, region, group, group_data, free, list);
            set_sizes(e, free, rest);
        }
        set_used(e, block, need);
        return 1;
    }
    if need == old {
        return 1;
    }
    let mut rest = old.wrapping_sub(need);
    e.mem.set_u32(block.wrapping_sub(4), need.wrapping_add(1));
    let free = block.wrapping_add(need).wrapping_sub(4);
    e.mem.set_u32(free.wrapping_sub(4), need.wrapping_add(1));
    let mut list = list_of(rest);
    if next_size & 1 == 0 {
        let next_list = list_of(next_size);
        if e.mem.u32(next.wrapping_add(4)) == e.mem.u32(next.wrapping_add(8)) {
            clear_list_bit(e, header, region, group, next_list);
        }
        unlink(e, next);
        rest = rest.wrapping_add(next_size);
        list = list_of(rest);
    }
    link_free(e, header, region, group, group_data, free, list);
    set_sizes(e, free, rest);
    1
}

/// A used block's size (`size + 1`, the used bit) at its start (`block - 4`) and in its last
/// word (`block + size - 8`).
fn set_used(e: &mut Engine, block: u32, size: u32) {
    e.mem.set_u32(block.wrapping_sub(4), size.wrapping_add(1));
    e.mem.set_u32(
        block.wrapping_add(size).wrapping_sub(8),
        size.wrapping_add(1),
    );
}

/// Whether a header (or a group's pair of words) has a block on a list at or above the asked
/// one: `(high & high_mask) | (low & low_mask)`.
fn has_list(e: &Engine, low_at: u32, high_at: u32, low_mask: u32, high_mask: u32) -> bool {
    (e.mem.u32(high_at) & high_mask) | (e.mem.u32(low_at) & low_mask) != 0
}

// Translated from 00edea12 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___sbh_alloc_block` (LIBCMT/sbheap.c; cdecl: the size). The size needed is the size plus
/// the size word, rounded up to 16; its list (`size / 16 - 1`) gives the masks of the lists
/// that hold big enough blocks (`0xFFFFFFFF >> list` for the low lists and all high lists, or
/// none of the low and `0xFFFFFFFF >> (list - 32)` of the high). The headers are searched from
/// the scan start (`__sbh_pHeaderScan`) to the end, then from the list's start to the scan
/// start, for one with such a list; failing that the same two ways for one with an
/// uncommitted group (`+8` not 0), failing that a new region (`___sbh_alloc_new_region`; none
/// answers 0); for these a group is committed (`__sbh_alloc_new_group`, its index kept as the
/// region's last group; -1 answers 0). The found header becomes the scan start. The group is
/// the region's last one when it has such a list, else the first that has (the exe's search
/// never ends when none has, as here). The list is the first set bit of the group's masks
/// (low, else high from 32; the exe's search never ends for an empty mask, as here), the block
/// the first on it. What is left of it stays free at its start (on another list when the size
/// left belongs there, taking the block off its list and clearing its bits when it was the
/// only one); the allocated block is the end part, its size with the used bit at both ends.
/// The group's used count goes up; a group that was empty and kept for decommitting
/// (`__sbh_pHeaderDefer`, `__sbh_indGroupDefer`) is no longer kept. The region's last group
/// becomes this one. Answers the block (after its size word).
pub fn sbh_alloc_block(e: &mut Engine, size: u32) -> u32 {
    let end = e
        .mem
        .u32(HEADER_COUNT)
        .wrapping_mul(0x14)
        .wrapping_add(e.mem.u32(HEADER_LIST));
    let need = size.wrapping_add(0x17) & 0xffff_fff0;
    let wanted = ((need as i32) >> 4).wrapping_sub(1);
    let (low_mask, high_mask) = if wanted < 0x20 {
        (u32::MAX >> (wanted as u32 & 0x1f), u32::MAX)
    } else {
        (0, u32::MAX >> ((wanted as u32).wrapping_sub(0x20) & 0x1f))
    };
    let scan = e.mem.u32(HEADER_SCAN);
    let has_block = |e: &Engine, h: u32| has_list(e, h, h.wrapping_add(4), low_mask, high_mask);
    let mut header = scan;
    while header < end && !has_block(e, header) {
        header = header.wrapping_add(0x14);
    }
    if header == end {
        header = e.mem.u32(HEADER_LIST);
        while header < scan && !has_block(e, header) {
            header = header.wrapping_add(0x14);
        }
        if header == scan {
            while header < end && e.mem.u32(header.wrapping_add(8)) == 0 {
                header = header.wrapping_add(0x14);
            }
            if header == end {
                header = e.mem.u32(HEADER_LIST);
                while header < scan && e.mem.u32(header.wrapping_add(8)) == 0 {
                    header = header.wrapping_add(0x14);
                }
                if header == scan {
                    header = e.call(SBH_ALLOC_NEW_REGION, &args![]).u32();
                    if header == 0 {
                        return 0;
                    }
                }
            }
            let index = e.call(SBH_ALLOC_NEW_GROUP, &args![header]).u32();
            let data = e.mem.u32(header.wrapping_add(0x10));
            e.mem.set_u32(data, index);
            if e.mem.u32(e.mem.u32(header.wrapping_add(0x10))) == u32::MAX {
                return 0;
            }
        }
    }
    e.mem.set_u32(HEADER_SCAN, header);
    let region = e.mem.u32(header.wrapping_add(0x10));
    let mut group = e.mem.u32(region);
    let group_has = |e: &Engine, g: u32| {
        let at = region.wrapping_add(g.wrapping_mul(4));
        has_list(
            e,
            at.wrapping_add(0x44),
            at.wrapping_add(0xc4),
            low_mask,
            high_mask,
        )
    };
    if group == u32::MAX || !group_has(e, group) {
        group = 0;
        while !group_has(e, group) {
            group = group.wrapping_add(1);
        }
    }
    let group_data = group
        .wrapping_mul(0x204)
        .wrapping_add(region)
        .wrapping_add(0x144);
    let at = region.wrapping_add(group.wrapping_mul(4));
    let mut bits = e.mem.u32(at.wrapping_add(0x44)) & low_mask;
    let mut list = 0u32;
    if bits == 0 {
        bits = e.mem.u32(at.wrapping_add(0xc4)) & high_mask;
        list = 0x20;
    }
    while (bits as i32) >= 0 {
        bits = bits.wrapping_add(bits);
        list = list.wrapping_add(1);
    }
    let block = e.mem.u32(
        group_data
            .wrapping_add(list.wrapping_mul(8))
            .wrapping_add(4),
    );
    let rest = e.mem.u32(block).wrapping_sub(need);
    let mut rest_list = ((rest as i32) >> 4).wrapping_sub(1);
    if rest_list > 0x3f {
        rest_list = 0x3f;
    }
    if rest_list != list as i32 {
        if e.mem.u32(block.wrapping_add(4)) == e.mem.u32(block.wrapping_add(8)) {
            clear_list_bit(e, header, region, group, list);
        }
        unlink(e, block);
        if rest != 0 {
            link_free(
                e,
                header,
                region,
                group,
                group_data,
                block,
                rest_list as u32,
            );
        }
    }
    if rest != 0 {
        set_sizes(e, block, rest);
    }
    let used_block = block.wrapping_add(rest);
    e.mem.set_u32(used_block, need.wrapping_add(1));
    e.mem.set_u32(
        used_block.wrapping_add(need).wrapping_sub(4),
        need.wrapping_add(1),
    );
    let used = e.mem.u32(group_data);
    e.mem.set_u32(group_data, used.wrapping_add(1));
    if used == 0 && header == e.mem.u32(DEFER_HEADER) && group == e.mem.u32(DEFER_GROUP) {
        e.mem.set_u32(DEFER_HEADER, 0);
    }
    e.mem.set_u32(region, group);
    used_block.wrapping_add(4)
}

// Translated from 00ecef9e (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecef9e - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180158) and the __local_unwind4 (00ed863c) call on the empty-name path are not translated; its __finally block (00ecf05a) is called inline on both paths.
/// `_tmpfile_helper` (Ghidra's function ID; the body is the shape of LIBCMT's `_wfsopen`; cdecl:
/// wide name, wide mode, share flag). A null name or mode, or an empty mode, sets `errno` to
/// `EINVAL` (0x16), calls the invalid-parameter handler (five zeros) and answers 0. Otherwise a
/// stream comes from `_getstream` (none: `errno` `EMFILE` (0x18), 0). An empty name sets
/// `errno` to `EINVAL` and answers 0; otherwise `_wopenfile(name, mode, share, stream)`
/// answers. Either way the stream is unlocked afterwards (the `__finally` block `00ecf05a`).
pub fn tmpfile_helper(e: &mut Engine, name: u32, mode: u32, share: u32) -> u32 {
    let invalid = |e: &mut Engine| {
        let error = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(error, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    };
    if name == 0 || mode == 0 || e.mem.u16(mode) == 0 {
        invalid(e);
        return 0;
    }
    let stream = e.call(GETSTREAM, &args![]).u32();
    if stream == 0 {
        let error = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(error, 0x18);
        return 0;
    }
    if e.mem.u16(name) == 0 {
        let error = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(error, 0x16);
        e.call(TMPFILE_HELPER_FINALLY, &args![stream]);
        return 0;
    }
    let opened = e.call(WOPENFILE, &args![name, mode, share, stream]).u32();
    e.call(TMPFILE_HELPER_FINALLY, &args![stream]);
    opened
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_e263, sbh_free_block(u32, u32)),
        entry!(0x00ed_e579, sbh_alloc_new_region() -> u32),
        entry!(0x00ed_e731, sbh_resize_block(u32, u32, u32) -> u32),
        entry!(0x00ed_ea12, sbh_alloc_block(u32) -> u32),
        entry!(0x00ec_ef9e, tmpfile_helper(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const BASE: u32 = 0x0400_0000;

    /// An empty small-block heap: the globals mapped, `HeapReAlloc`/`HeapAlloc` handing out
    /// zeroed blocks (the old list copied), `VirtualAlloc` reserving at `BASE` and committing
    /// where asked, `VirtualFree` and `HeapFree` doing nothing.
    fn heap() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.map(BASE, 0x10_0000);
        e.mem.set_u32(CRT_HEAP, 0x5555);
        e.register(HEAP_REALLOC, |e, a| {
            let block = e.mem.alloc(a[3]);
            if a[2] != 0 {
                let bytes = e.mem.bytes(a[2], a[3] - 0x140);
                e.mem.write(block, &bytes);
            }
            block.into_ret()
        });
        e.register(HEAP_ALLOC, |e, a| e.mem.alloc(a[2]).into_ret());
        e.register(VIRTUAL_ALLOC, |_, a| {
            (if a[0] == 0 { BASE } else { a[0] }).into_ret()
        });
        e.register(VIRTUAL_FREE, |_, _| 1u32.into_ret());
        e.register(HEAP_FREE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn a_new_region_reserves_a_megabyte_and_grows_the_list() {
        let mut e = heap();
        let header = e.call(0x00ed_e579, &[]).u32();
        let list = e.mem.u32(HEADER_LIST);
        assert_eq!(header, list);
        assert_eq!(calls(&e, HEAP_REALLOC), vec![vec![0x5555, 0, 0, 0x140]]);
        assert_eq!(calls(&e, HEAP_ALLOC), vec![vec![0x5555, 8, 0x41c4]]);
        assert_eq!(
            calls(&e, VIRTUAL_ALLOC),
            vec![vec![0, 0x10_0000, 0x2000, 4]]
        );
        assert_eq!(e.mem.u32(HEADER_SIZE), 0x10);
        assert_eq!(e.mem.u32(HEADER_COUNT), 1);
        assert_eq!(e.mem.u32(header + 8), u32::MAX);
        assert_eq!(e.mem.u32(header + 0xc), BASE);
        let data = e.mem.u32(header + 0x10);
        assert_eq!(e.mem.u32(data), u32::MAX);
        // A second one fits in the list; a failed reservation frees the data.
        assert_eq!(e.call(0x00ed_e579, &[]).u32(), list + 0x14);
        assert_eq!(calls(&e, HEAP_REALLOC).len(), 1);
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_e579, &[]).u32(), 0);
        let data = e.mem.u32(list + 0x28 + 0x10);
        assert_eq!(calls(&e, HEAP_FREE), vec![vec![0x5555, 0, data]]);
        assert_eq!(e.mem.u32(HEADER_COUNT), 2);
    }

    #[test]
    fn blocks_are_carved_resized_and_joined_again() {
        let mut e = heap();
        // An empty heap: a region, its first group, two blocks from the end of its first page.
        let a = e.call(0x00ed_ea12, &[0x20]).u32();
        let header = e.mem.u32(HEADER_LIST);
        let region = e.mem.u32(header + 0x10);
        let group = region + 0x144;
        assert_eq!(e.mem.u32(HEADER_SCAN), header);
        assert_eq!(a, BASE + 0xfd0);
        assert_eq!(e.mem.u32(BASE + 0xfcc), 0x31);
        assert_eq!(e.mem.u32(BASE + 0xff8), 0x31);
        assert_eq!(e.mem.u32(BASE + 0xc), 0xfc0);
        assert_eq!(e.mem.u32(BASE + 0xfc8), 0xfc0);
        let b = e.call(0x00ed_ea12, &[0x20]).u32();
        assert_eq!(b, BASE + 0xfa0);
        assert_eq!(e.mem.u32(group), 2);
        assert_eq!(e.mem.u32(region), 0);
        // Growing into a block in use fails.
        assert_eq!(e.call(0x00ed_e731, &[header, b, 0x40]).u32(), 0);
        // Freeing a: alone on list 2.
        e.call(0x00ed_e263, &[header, a]);
        assert_eq!(e.mem.u32(BASE + 0xfcc), 0x30);
        assert_eq!(e.mem.u32(BASE + 0xff8), 0x30);
        assert_eq!(e.mem.u32(header), 0x2000_0000);
        assert_eq!(e.mem.u32(region + 0x44), 0x2000_0000);
        assert_eq!(e.mem.u8(region + 6), 1);
        assert_eq!(e.mem.u32(group + 0x14), BASE + 0xfcc);
        assert_eq!(e.mem.u32(group), 1);
        // Growing b into it leaves 0x10 free on list 0.
        assert_eq!(e.call(0x00ed_e731, &[header, b, 0x40]).u32(), 1);
        assert_eq!(e.mem.u32(BASE + 0xf9c), 0x51);
        assert_eq!(e.mem.u32(BASE + 0xfe8), 0x51);
        assert_eq!(e.mem.u32(BASE + 0xfec), 0x10);
        assert_eq!(e.mem.u32(BASE + 0xff8), 0x10);
        assert_eq!(e.mem.u32(header), 0x8000_0000);
        assert_eq!(e.mem.u32(region + 0x44), 0x8000_0000);
        assert_eq!(e.mem.u8(region + 6), 0);
        assert_eq!(e.mem.u8(region + 4), 1);
        // Shrinking it joins the rest with that free block: 0x40 on list 3.
        assert_eq!(e.call(0x00ed_e731, &[header, b, 0x10]).u32(), 1);
        assert_eq!(e.mem.u32(BASE + 0xf9c), 0x21);
        assert_eq!(e.mem.u32(BASE + 0xfb8), 0x21);
        assert_eq!(e.mem.u32(BASE + 0xfbc), 0x40);
        assert_eq!(e.mem.u32(BASE + 0xff8), 0x40);
        assert_eq!(e.mem.u32(header), 0x1000_0000);
        assert_eq!(e.mem.u32(region + 0x44), 0x1000_0000);
        // The same size changes nothing.
        assert_eq!(e.call(0x00ed_e731, &[header, b, 0x10]).u32(), 1);
        assert_eq!(e.mem.u32(BASE + 0xf9c), 0x21);
        // Freeing b joins both neighbours back into the page's one free block; the emptied
        // group is kept for decommitting.
        e.call(0x00ed_e263, &[header, b]);
        assert_eq!(e.mem.u32(BASE + 0xc), 0xff0);
        assert_eq!(e.mem.u32(BASE + 0xff8), 0xff0);
        assert_eq!(e.mem.u32(header), 0);
        assert_eq!(e.mem.u32(region + 0x44), 0);
        assert_eq!(e.mem.u32(group), 0);
        assert_eq!(e.mem.u32(DEFER_HEADER), header);
        assert_eq!(e.mem.u32(DEFER_GROUP), 0);
        // A block that is not in use (the page's free block) is left alone.
        e.call(0x00ed_e263, &[header, BASE + 0x10]);
        assert_eq!(e.mem.u32(group), 0);
        // Allocating from the kept group takes it back.
        let c = e.call(0x00ed_ea12, &[0x20]).u32();
        assert_eq!(c, BASE + 0xfd0);
        assert_eq!(e.mem.u32(DEFER_HEADER), 0);
        assert!(calls(&e, VIRTUAL_FREE).is_empty());
    }

    #[test]
    fn an_emptied_region_is_released_when_another_group_empties() {
        let mut e = heap();
        // Two regions: the first kept with its only group to decommit, the second with one
        // used block (group 1).
        let first = e.call(0x00ed_e579, &[]).u32();
        let second = e.call(0x00ed_e579, &[]).u32();
        let first_data = e.mem.u32(first + 0x10);
        e.mem.set_u32(first + 4, 1);
        e.mem.set_u32(first + 8, 0x7fff_ffff);
        e.mem.set_u8(first_data + 0x43, 1);
        e.mem.set_u32(first_data + 0xc4, 1);
        e.mem.set_u32(DEFER_HEADER, first);
        e.mem.set_u32(DEFER_GROUP, 0);
        let second_data = e.mem.u32(second + 0x10);
        let group = second_data + 0x144 + 0x204;
        for list in 0..64 {
            let head = group + list * 8;
            e.mem.set_u32(head + 4, head);
            e.mem.set_u32(head + 8, head);
        }
        e.mem.set_u32(group, 1);
        let block = BASE + 0x8000 + 0x10;
        e.mem.set_u32(block - 8, 1);
        e.mem.set_u32(block - 4, 0x21);
        e.mem.set_u32(block + 0x1c, 1);
        e.call(0x00ed_e263, &[second, block]);
        assert_eq!(e.mem.u32(block - 4), 0x20);
        assert_eq!(e.mem.u32(second), 0x4000_0000);
        assert_eq!(
            calls(&e, VIRTUAL_FREE),
            vec![vec![BASE, 0x8000, 0x4000], vec![BASE, 0, 0x8000]]
        );
        assert_eq!(calls(&e, HEAP_FREE), vec![vec![0x5555, 0, first_data]]);
        assert_eq!(e.mem.u32(HEADER_COUNT), 1);
        // The second header moved down into the first's place, and is now the kept one.
        let list = e.mem.u32(HEADER_LIST);
        assert_eq!(e.mem.u32(list + 0x10), second_data);
        assert_eq!(e.mem.u32(DEFER_HEADER), list);
        assert_eq!(e.mem.u32(DEFER_GROUP), 1);
        assert_eq!(e.mem.u32(HEADER_SCAN), list);
    }

    #[test]
    fn tmpfile_helper_checks_then_opens_and_unlocks() {
        let mut e = heap();
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e.register(TMPFILE_HELPER_FINALLY, |_, _| Ret::default());
        e.register(WOPENFILE, |_, _| 0x0777u32.into_ret());
        e.register(GETSTREAM, |_, _| 0x0600u32.into_ret());
        let name = e.mem.alloc(8);
        let mode = e.mem.alloc(8);
        e.mem.set_u16(name, u16::from(b'a'));
        e.mem.set_u16(mode, u16::from(b'r'));
        assert_eq!(e.call(0x00ec_ef9e, &[name, mode, 0x40]).u32(), 0x0777);
        assert_eq!(calls(&e, WOPENFILE), vec![vec![name, mode, 0x40, 0x0600]]);
        assert_eq!(calls(&e, TMPFILE_HELPER_FINALLY), vec![vec![0x0600]]);
        // Bad arguments.
        for (n, m) in [(0, mode), (name, 0), (name, name + 2)] {
            e.mem.set_u32(errno, 0);
            assert_eq!(e.call(0x00ec_ef9e, &[n, m, 0x40]).u32(), 0);
            assert_eq!(e.mem.u32(errno), 0x16);
        }
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 3);
        // An empty name still unlocks the stream.
        e.mem.set_u16(name, 0);
        assert_eq!(e.call(0x00ec_ef9e, &[name, mode, 0x40]).u32(), 0);
        assert_eq!(e.mem.u32(errno), 0x16);
        assert_eq!(calls(&e, TMPFILE_HELPER_FINALLY).len(), 2);
        // No stream.
        e.register(GETSTREAM, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_ef9e, &[name, mode, 0x40]).u32(), 0);
        assert_eq!(e.mem.u32(errno), 0x18);
        assert_eq!(calls(&e, WOPENFILE).len(), 1);
    }
}
