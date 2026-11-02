//! `Havok SDK/hksolverallocator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkSolverAllocator`'s virtual table (`010d1110`; the class name is confirmed
/// by the RTTI record before it).
const VTABLE: u32 = 0x010d_1110;
/// `hkCriticalSection::hkCriticalSection` (`00cf6da0`, `thiscall`, spin count).
const CRITICAL_SECTION_CONSTRUCTOR: u32 = 0x00cf_6da0;

// Translated from 00d5e900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::hkSolverAllocator` (Xbox PDB), `thiscall`, no stack
/// arguments: stores the virtual table, clears the words at `+4` to `+0x14`,
/// points `+0x18` at `this + 0x24`, clears `+0x1c`, stores `0x80000020` at `+0x20`
/// and constructs the critical section at `+0x124` with spin count 0. Returns
/// `this`.
pub fn hk_solver_allocator_hk_solver_allocator(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    for offset in [0x04, 0x08, 0x0c, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x18, base + 0x24);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0x20, 0x8000_0020);
    e.call(CRITICAL_SECTION_CONSTRUCTOR, &args![base + 0x124, 0u32]);
    this
}

/// `EnterCriticalSection` and `LeaveCriticalSection` (`kernel32` import slots,
/// `stdcall`, one word).
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

// Translated from 00d5e960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::setBuffer` (Xbox PDB), `thiscall`, `RET 8`: inside the
/// critical section at `+0x124`, points `+0x18` at `this + 0x24`, stores
/// `0x80000020` at `+0x20`, `buffer` at `+4` and `+0xc`, `buffer + size` at `+8` and
/// clears `+0x10`.
pub fn hk_solver_allocator_set_buffer(e: &mut Engine, this: Ptr, buffer: u32, size: u32) {
    let base = this.addr();
    e.call(ENTER_CRITICAL_SECTION, &args![base + 0x124]);
    e.mem.set_u32(base + 0x18, base + 0x24);
    e.mem.set_u32(base + 0x20, 0x8000_0020);
    e.mem.set_u32(base + 4, buffer);
    e.mem.set_u32(base + 0xc, buffer);
    e.mem.set_u32(base + 8, buffer.wrapping_add(size));
    e.mem.set_u32(base + 0x10, 0);
    e.call(LEAVE_CRITICAL_SECTION, &args![base + 0x124]);
}

/// `DeleteCriticalSection` (`kernel32` import slot, `stdcall`, one word).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The base allocator part of the destructor (`00cbf1a0`, no map name, `thiscall`): stores the
/// base allocator's vtable (`010c3b30`) at `this`.
const BASE_ALLOCATOR_DESTRUCTOR: u32 = 0x00cb_f1a0;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, count, element size) and
/// `hkMemUtil::memMove` (cdecl: destination, source, byte count).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `00d5ed80` (no map name; `thiscall`, four stack words): inserts `count` elements, calling
/// `00d5ecc0` with `(allocator, index, 0, source, count)`.
const ARRAY_INSERT_AT: u32 = 0x00d5_ed80;

/// Mask of the capacity bits of an array's capacity-and-flags word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d5e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::blockAlloc` (Xbox PDB), virtual, `RET 4` (one word, not read): answers 0
/// (the solver allocator hands out no blocks).
pub fn hk_solver_allocator_block_alloc(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

// Translated from 00d5e890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::blockFree` (Xbox PDB), virtual, `RET 8` (two words, not read): does
/// nothing.
pub fn hk_solver_allocator_block_free(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32) {
}

// Translated from 00d5e8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::canAllocSingleBlock` (Xbox PDB), `thiscall`, `RET 4`: whether `size` is at
/// most the bytes between the buffer's end (`+8`) and its start (`+4`) (signed compare).
pub fn hk_solver_allocator_can_alloc_single_block(e: &mut Engine, this: Ptr, size: i32) -> bool {
    let base = this.addr();
    let capacity = e.mem.i32(base + 8).wrapping_sub(e.mem.i32(base + 4));
    size <= capacity
}

// Translated from 00d5e8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::getMemoryStatistics` (Xbox PDB), `thiscall`, `RET 4`: writes four words to
/// `statistics`: `[0]` = buffer end - buffer start (`+8` - `+4`), `[3]` = buffer end - top
/// (`+8` - `+0xc`), `[1]` = top - buffer start (`+0xc` - `+4`), `[2]` = the peak (`+0x14`).
pub fn hk_solver_allocator_get_memory_statistics(e: &mut Engine, this: Ptr, statistics: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8).wrapping_sub(e.mem.u32(base + 4));
    e.mem.set_u32(statistics, size);
    let free = e.mem.u32(base + 8).wrapping_sub(e.mem.u32(base + 0xc));
    e.mem.set_u32(statistics + 0xc, free);
    let used = e.mem.u32(base + 0xc).wrapping_sub(e.mem.u32(base + 4));
    e.mem.set_u32(statistics + 4, used);
    let peak = e.mem.u32(base + 0x14);
    e.mem.set_u32(statistics + 8, peak);
}

// Translated from 00d5e8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::resetPeakMemoryStatistics` (Xbox PDB), `ECX` = `this`: the peak (`+0x14`)
/// becomes the bytes in use, top (`+0xc`) - buffer start (`+4`).
pub fn hk_solver_allocator_reset_peak_memory_statistics(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let used = e.mem.u32(base + 0xc).wrapping_sub(e.mem.u32(base + 4));
    e.mem.set_u32(base + 0x14, used);
}

// Translated from 00d5e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the destructor body of `hkSolverAllocator`, which `00d5ed70` calls). `ECX` =
/// `this`, no stack words: stores the solver allocator's vtable (`010d1110`), deletes the critical
/// section at `+0x124` and then runs the base allocator's part (`00cbf1a0`, which stores the base
/// vtable). Nothing is answered.
pub fn fn_00d5e940(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE);
    e.call(DELETE_CRITICAL_SECTION, &args![base + 0x124]);
    e.call(BASE_ALLOCATOR_DESTRUCTOR, &args![this]);
}

/// Reads the used bytes (top - start) and raises the peak (`+0x14`) when it is higher (unsigned
/// compare), after counting one more live allocation (`+0x10`), as every successful `bufAlloc`
/// ends.
fn count_allocation(e: &mut Engine, base: u32) {
    let used = e.mem.u32(base + 0xc).wrapping_sub(e.mem.u32(base + 4));
    let live = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0x10, live.wrapping_add(1));
    if used > e.mem.u32(base + 0x14) {
        e.mem.set_u32(base + 0x14, used);
    }
}

/// Moves `bytes` bytes down by 8 bytes starting at `entry + 8` (the dword loop both functions
/// use to close a gap in the free list): `((bytes - 1) >> 2) + 1` dwords are copied forward
/// from `entry + 8` to `entry`. Does nothing for 0 or fewer bytes.
fn close_gap(e: &mut Engine, entry: u32, bytes: i32) {
    if bytes > 0 {
        let dwords = ((bytes - 1) as u32 >> 2) + 1;
        for i in 0..dwords {
            let value = e.mem.u32(entry.wrapping_add(8).wrapping_add(4 * i));
            e.mem.set_u32(entry.wrapping_add(4 * i), value);
        }
    }
}

// Translated from 00d5e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::bufAlloc` (Xbox PDB), `thiscall`, `RET 4`: `size_ptr` points to the
/// requested size and receives the size granted. Inside the critical section (`+0x124`): fails
/// (answers 0) when the live-allocation count (`+0x10`) plus one is not below twice the free-list
/// capacity minus its size (`+0x1c`, array at `+0x18` of 8-byte `(address, size)` entries, capacity
/// in `+0x20`). Otherwise looks through the free list from the last entry to the first for the
/// smallest block at least as large as the request (a strictly smaller one wins; blocks above
/// `0x7ffffff` are never taken). A block found is split from the front when it is more than
/// 1.5 times the request and the request is above `0x400` (the granted size stays the request); else it is
/// taken whole (the granted size is the block's) and removed from the list. With no such block
/// the request is carved from the top of the buffer (`+0xc`, up to `+8`). Every success counts one
/// more live allocation and raises the peak (`+0x14`). Answers the block's address.
pub fn hk_solver_allocator_buf_alloc(e: &mut Engine, this: Ptr, size_ptr: u32) -> u32 {
    let base = this.addr();
    let section = base + 0x124;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let flags = e.mem.u32(base + 0x20);
    let list_count = e.mem.i32(base + 0x1c);
    let live = e.mem.i32(base + 0x10);
    let wanted = e.mem.i32(size_ptr);
    let room = ((flags & CAPACITY_MASK) as i32)
        .wrapping_add((flags & CAPACITY_MASK) as i32)
        .wrapping_sub(list_count);
    if live.wrapping_add(1) >= room {
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return 0;
    }
    let mut best: i32 = -1;
    let mut best_size: i32 = 0x7ff_ffff;
    let mut index = list_count.wrapping_sub(1);
    let list = e.mem.u32(base + 0x18);
    if index >= 0 {
        let mut at = list
            .wrapping_add((index as u32).wrapping_mul(8))
            .wrapping_add(4);
        loop {
            let entry_size = e.mem.i32(at);
            if (wanted..best_size).contains(&entry_size) {
                best_size = entry_size;
                best = index;
            }
            at = at.wrapping_sub(8);
            index = index.wrapping_sub(1);
            if index < 0 {
                break;
            }
        }
    }
    if best >= 0 {
        let entry = (best as u32).wrapping_mul(8);
        let block = e.mem.u32(entry.wrapping_add(list));
        if best_size.wrapping_mul(2) > wanted.wrapping_mul(3) && wanted > 0x400 {
            let at = entry.wrapping_add(list);
            let remaining = e.mem.i32(at + 4).wrapping_sub(wanted);
            e.mem.set_i32(at + 4, remaining);
            e.mem.set_u32(at, block.wrapping_add(wanted as u32));
        } else {
            let granted = e.mem.u32(entry.wrapping_add(list) + 4);
            e.mem.set_u32(size_ptr, granted);
            let count = e.mem.i32(base + 0x1c).wrapping_sub(1);
            e.mem.set_i32(base + 0x1c, count);
            let moved = count.wrapping_sub(best).wrapping_mul(8);
            let list = e.mem.u32(base + 0x18);
            close_gap(e, entry.wrapping_add(list), moved);
        }
        count_allocation(e, base);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return block;
    }
    let top = e.mem.u32(base + 0xc);
    let remaining = e.mem.i32(base + 8).wrapping_sub(top as i32);
    if remaining < wanted {
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return 0;
    }
    e.mem.set_u32(base + 0xc, top.wrapping_add(wanted as u32));
    e.mem.set_i32(size_ptr, wanted);
    count_allocation(e, base);
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
    top
}

// Translated from 00d5eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::bufFree` (Xbox PDB), `thiscall`, `RET 8`: gives the block at `block` of
/// `size` bytes back, inside the critical section (`+0x124`), counting one live allocation
/// less (`+0x10`). A block ending at the top of the buffer (`+0xc`) lowers the top by `size`, and
/// every free-list entry (array at `+0x18`, 8-byte `(address, size)` entries kept in address
/// order, count at `+0x1c`) that then ends exactly where the block started (compared with
/// the block's address, as the code does) lowers the top to its address and is dropped. Any other block
/// is entered in the list: the first one alone; otherwise the list is searched from the end
/// for the last entry below the block's address, which is extended when it ends at the block
/// (and merged with the next entry when that starts where the block ends); else, if the next
/// entry starts where the block ends, that entry is extended to start at the block; else a
/// new entry is inserted after it (`00d5eda0`).
pub fn hk_solver_allocator_buf_free(e: &mut Engine, this: Ptr, block: u32, size: u32) {
    let base = this.addr();
    let section = base + 0x124;
    e.call(ENTER_CRITICAL_SECTION, &args![section]);
    let top = e.mem.u32(base + 0xc);
    let live = e.mem.u32(base + 0x10);
    e.mem.set_u32(base + 0x10, live.wrapping_add(0xffff_ffff));
    let block_end = block.wrapping_add(size);
    if block_end == top {
        e.mem.set_u32(base + 0xc, top.wrapping_sub(size));
        if e.mem.u32(base + 0x1c) != 0 {
            loop {
                let list = e.mem.u32(base + 0x18);
                let count = e.mem.u32(base + 0x1c);
                let last = list.wrapping_add(count.wrapping_mul(8)).wrapping_sub(8);
                let end = e.mem.u32(last + 4).wrapping_add(e.mem.u32(last));
                if end != block {
                    break;
                }
                let list = e.mem.u32(base + 0x18);
                let count = e.mem.u32(base + 0x1c);
                let address = e
                    .mem
                    .u32(list.wrapping_add(count.wrapping_mul(8)).wrapping_sub(8));
                e.mem.set_u32(base + 0xc, address);
                let count = e.mem.u32(base + 0x1c).wrapping_add(0xffff_ffff);
                e.mem.set_u32(base + 0x1c, count);
                if count == 0 {
                    break;
                }
            }
        }
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return;
    }
    let count = e.mem.i32(base + 0x1c);
    if count == 0 {
        let list = e.mem.u32(base + 0x18);
        e.mem.set_u32(base + 0x1c, 1);
        e.mem.set_u32(list, block);
        e.mem.set_u32(list + 4, size);
        e.call(LEAVE_CRITICAL_SECTION, &args![section]);
        return;
    }
    // `last_index` is also kept (in the stack slot of `size`) for the later comparisons.
    let last_index = count.wrapping_sub(1);
    let mut index = last_index;
    let mut found_previous = false;
    if index >= 0 {
        let list = e.mem.u32(base + 0x18);
        let mut entry = list.wrapping_add((index as u32).wrapping_mul(8));
        loop {
            if e.mem.u32(entry) < block {
                found_previous = true;
                break;
            }
            index = index.wrapping_sub(1);
            entry = entry.wrapping_sub(8);
            if index < 0 {
                break;
            }
        }
        if found_previous {
            let slot = list.wrapping_add((index as u32).wrapping_mul(8));
            let previous_size = e.mem.u32(slot + 4);
            let previous_end = e.mem.u32(slot).wrapping_add(previous_size);
            if previous_end == block {
                let merged = previous_size.wrapping_add(size);
                e.mem.set_u32(slot + 4, merged);
                let last = e.mem.i32(base + 0x1c).wrapping_sub(1);
                if index < last {
                    let list = e.mem.u32(base + 0x18);
                    let next = list.wrapping_add((index as u32).wrapping_mul(8));
                    if block_end == e.mem.u32(next + 8) {
                        let total = e.mem.u32(next + 0xc).wrapping_add(merged);
                        e.mem.set_u32(slot + 4, total);
                        let count = e.mem.i32(base + 0x1c).wrapping_sub(1);
                        e.mem.set_i32(base + 0x1c, count);
                        let list = e.mem.u32(base + 0x18);
                        let first_moved = index.wrapping_add(1);
                        let moved = count.wrapping_sub(first_moved).wrapping_mul(8);
                        let entry = list.wrapping_add((first_moved as u32).wrapping_mul(8));
                        close_gap(e, entry, moved);
                    }
                }
                e.call(LEAVE_CRITICAL_SECTION, &args![section]);
                return;
            }
        }
    }
    // Not next to an earlier block. `index` is the entry below the block (-1 when none).
    if index < last_index {
        let list = e.mem.u32(base + 0x18);
        let next = list
            .wrapping_add((index as u32).wrapping_mul(8))
            .wrapping_add(8);
        if e.mem.u32(next) == block_end {
            let next_size = e.mem.u32(next + 4);
            e.mem.set_u32(next + 4, next_size.wrapping_add(size));
            e.mem.set_u32(next, block);
            e.call(LEAVE_CRITICAL_SECTION, &args![section]);
            return;
        }
    }
    // The exe's frame (`SUB ESP,0x8`, four pushes): 0x18 bytes, the inserted pair at +0x10
    // (00d5ec96).
    e.with_stack(0x18, |e, frame| {
        let pair = frame.addr() + 0x10;
        e.mem.set_u32(pair, block);
        e.mem.set_u32(pair + 4, size);
        fn_00d5eda0(
            e,
            Ptr::new(base + 0x18),
            0,
            index.wrapping_add(1) as u32,
            pair,
        );
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![section]);
}

// Translated from 00d5ecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the array splice of the free list of 8-byte entries). `thiscall`, `this` = the
/// array (data, size, capacity and flags), `RET 0x14`: replaces `remove` entries starting at
/// `index` by `insert` entries copied from `source` (an entry is copied only if its destination is
/// not null). The array first grows when its capacity is below the new size
/// (`hkArrayUtil::_reserve(allocator, array, max(2 * capacity, new size), 8)`); the entries after
/// the removed ones move to follow the inserted ones (`hkMemUtil::memMove`); the size becomes
/// `size - remove + insert`.
pub fn fn_00d5ecc0(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove: i32,
    source: u32,
    insert: i32,
) {
    let array = this.addr();
    let size = e.mem.i32(array + 4);
    let tail = size.wrapping_sub(index).wrapping_sub(remove);
    let new_size = size.wrapping_sub(remove).wrapping_add(insert);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let data = e.mem.u32(array);
    let bytes = tail.wrapping_mul(8);
    let from = data.wrapping_add((remove.wrapping_add(index) as u32).wrapping_mul(8));
    let to = data.wrapping_add((index.wrapping_add(insert) as u32).wrapping_mul(8));
    e.call(MEM_MOVE, &args![to, from, bytes]);
    let data = e.mem.u32(array);
    let mut at = data.wrapping_add((index as u32).wrapping_mul(8));
    if insert > 0 {
        let offset = source.wrapping_sub(at);
        for _ in 0..insert {
            if at != 0 {
                let first = e.mem.u32(at.wrapping_add(offset));
                e.mem.set_u32(at, first);
                let second = e.mem.u32(at.wrapping_add(offset).wrapping_add(4));
                e.mem.set_u32(at + 4, second);
            }
            at = at.wrapping_add(8);
        }
    }
    e.mem.set_i32(array + 4, new_size);
}

// Translated from 00d5ed70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSolverAllocator::_scalar_deleting_destructor_` (Xbox PDB), `thiscall`, `RET 4` (the flag
/// word is not read, nothing is freed): runs the destructor body (`00d5e940`) and returns `this`.
pub fn hk_solver_allocator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> Ptr {
    fn_00d5e940(e, this);
    this
}

// Translated from 00d5eda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `thiscall` (`ECX` = `this`, the array, passed on), `RET 0xc`: inserts one entry:
/// `00d5ed80(this, allocator, index, source, 1)` with the three stack words as
/// `(allocator, index, source)`.
pub fn fn_00d5eda0(e: &mut Engine, this: Ptr, allocator: u32, index: u32, source: u32) {
    e.call(
        ARRAY_INSERT_AT,
        &args![this, allocator, index, source, 1u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5e900,
            hk_solver_allocator_hk_solver_allocator(Ptr) -> Ptr
        ),
        entry!(0x00d5e960, hk_solver_allocator_set_buffer(Ptr, u32, u32)),
        entry!(0x00d5e880, hk_solver_allocator_block_alloc(Ptr, u32) -> u32),
        entry!(0x00d5e890, hk_solver_allocator_block_free(Ptr, u32, u32)),
        entry!(0x00d5e8a0, hk_solver_allocator_can_alloc_single_block(Ptr, i32) -> bool),
        entry!(
            0x00d5e8c0,
            hk_solver_allocator_get_memory_statistics(Ptr, u32)
        ),
        entry!(
            0x00d5e8f0,
            hk_solver_allocator_reset_peak_memory_statistics(Ptr)
        ),
        entry!(0x00d5e940, fn_00d5e940(Ptr)),
        entry!(0x00d5e9b0, hk_solver_allocator_buf_alloc(Ptr, u32) -> u32),
        entry!(0x00d5eb10, hk_solver_allocator_buf_free(Ptr, u32, u32)),
        entry!(0x00d5ecc0, fn_00d5ecc0(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00d5ed70, hk_solver_allocator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00d5eda0, fn_00d5eda0(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_sets_the_fields_and_builds_the_critical_section() {
        let mut e = Engine::new();
        e.register(CRITICAL_SECTION_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let object = e.mem.alloc(0x130);
        e.mem.write(object, &[0xcd; 0x130]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d5_e900, &args![object]).u32(), object);
        let log = e.call_log.take().unwrap();
        let sections: Vec<_> = log
            .iter()
            .filter(|call| call.0 == CRITICAL_SECTION_CONSTRUCTOR)
            .collect();
        assert_eq!(sections.len(), 1);
        assert_eq!(sections[0].1, vec![object + 0x124, 0]);
        assert_eq!(e.mem.u32(object), VTABLE);
        for offset in [4, 8, 0xc, 0x10, 0x14, 0x1c] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u32(object + 0x18), object + 0x24);
        assert_eq!(e.mem.u32(object + 0x20), 0x8000_0020);
        // Untouched: the bytes between the named words and the section.
        assert_eq!(e.mem.u32(object + 0x24), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(object + 0x120), 0xcdcd_cdcd);
    }

    #[test]
    fn set_buffer_describes_the_buffer_inside_the_critical_section() {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        let object = e.mem.alloc(0x130);
        e.mem.write(object, &[0xcd; 0x130]);
        e.call_log = Some(vec![]);
        e.call(0x00d5_e960, &args![object, 0x7000u32, 0x300u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![0x00d5_e960, ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]
        );
        assert_eq!(log[1].1, vec![object + 0x124]);
        assert_eq!(e.mem.u32(object + 4), 0x7000);
        assert_eq!(e.mem.u32(object + 0xc), 0x7000);
        assert_eq!(e.mem.u32(object + 8), 0x7300);
        assert_eq!(e.mem.u32(object + 0x10), 0);
        assert_eq!(e.mem.u32(object + 0x18), object + 0x24);
        assert_eq!(e.mem.u32(object + 0x20), 0x8000_0020);
    }

    /// The solver allocator's fields: buffer start `+4`, end `+8`, top `+0xc`, live count `+0x10`,
    /// peak `+0x14`, the free list (data `+0x18`, size `+0x1c`, capacity and flags `+0x20`).
    struct Fields {
        start: u32,
        end: u32,
        top: u32,
        live: u32,
        peak: u32,
    }

    /// An engine with the critical section functions and `memMove` as doubles and an allocator
    /// whose free list (capacity 8, not owned) holds `list`; returns the allocator.
    fn allocator(fields: &Fields, list: &[(u32, u32)]) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ENTER_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(LEAVE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        // `00d5ed80` inserts through `00d5ecc0` with nothing removed.
        e.register_double(ARRAY_INSERT_AT, |e, a| {
            e.call(0x00d5_ecc0, &args![a[0], a[1], a[2], 0u32, a[3], a[4]]);
            Ret::default()
        });
        let this = e.mem.alloc(0x130);
        let storage = e.mem.alloc(0x100);
        e.mem.set_u32(this + 4, fields.start);
        e.mem.set_u32(this + 8, fields.end);
        e.mem.set_u32(this + 0xc, fields.top);
        e.mem.set_u32(this + 0x10, fields.live);
        e.mem.set_u32(this + 0x14, fields.peak);
        e.mem.set_u32(this + 0x18, storage);
        e.mem.set_u32(this + 0x1c, list.len() as u32);
        e.mem.set_u32(this + 0x20, 0x8000_0008);
        for (i, (address, size)) in list.iter().enumerate() {
            e.mem.set_u32(storage + 8 * i as u32, *address);
            e.mem.set_u32(storage + 8 * i as u32 + 4, *size);
        }
        (e, this)
    }

    fn free_list(e: &Engine, this: u32) -> Vec<(u32, u32)> {
        let data = e.mem.u32(this + 0x18);
        (0..e.mem.u32(this + 0x1c))
            .map(|i| (e.mem.u32(data + 8 * i), e.mem.u32(data + 8 * i + 4)))
            .collect()
    }

    const FIELDS: Fields = Fields {
        start: 0x1_0000,
        end: 0x2_0000,
        top: 0x1_8000,
        live: 3,
        peak: 0x100,
    };

    fn alloc(e: &mut Engine, this: u32, size: i32) -> (u32, i32) {
        let cell = e.mem.alloc(4);
        e.mem.set_i32(cell, size);
        let block = e.call(0x00d5_e9b0, &args![this, cell]).u32();
        (block, e.mem.i32(cell))
    }

    #[test]
    fn the_stub_and_statistics_functions() {
        let (mut e, this) = allocator(&FIELDS, &[]);
        assert_eq!(e.call(0x00d5_e880, &args![this, 0x40u32]).u32(), 0);
        e.call(0x00d5_e890, &args![this, 1u32, 2u32]);
        // The buffer holds 0x10000 bytes: 0x10000 fits, 0x10001 does not (signed compare).
        assert!(e.call(0x00d5_e8a0, &args![this, 0x1_0000u32]).bool());
        assert!(!e.call(0x00d5_e8a0, &args![this, 0x1_0001u32]).bool());
        assert!(e.call(0x00d5_e8a0, &args![this, -5i32]).bool());
        let out = e.mem.alloc(16);
        e.call(0x00d5_e8c0, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x1_0000);
        assert_eq!(e.mem.u32(out + 4), 0x8000);
        assert_eq!(e.mem.u32(out + 8), 0x100);
        assert_eq!(e.mem.u32(out + 0xc), 0x8000);
        e.call(0x00d5_e8f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000);
    }

    #[test]
    fn the_destructor_body_deletes_the_section_and_runs_the_base_part() {
        let mut e = Engine::new();
        e.register(DELETE_CRITICAL_SECTION, |_, _| Ret::default());
        e.register(BASE_ALLOCATOR_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x130);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d5_ed70, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), VTABLE);
        let calls: Vec<_> = log.iter().skip(1).collect();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, DELETE_CRITICAL_SECTION);
        assert_eq!(calls[0].1, vec![this + 0x124]);
        assert_eq!(calls[1].0, BASE_ALLOCATOR_DESTRUCTOR);
        assert_eq!(calls[1].1, vec![this]);
        e.call(0x00d5_e940, &args![this]);
    }

    #[test]
    fn buf_alloc_carves_from_the_top_and_counts() {
        let (mut e, this) = allocator(&FIELDS, &[]);
        let (block, granted) = alloc(&mut e, this, 0x200);
        assert_eq!((block, granted), (0x1_8000, 0x200));
        assert_eq!(e.mem.u32(this + 0xc), 0x1_8200);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        // Used 0x8200 is above the old peak 0x100.
        assert_eq!(e.mem.u32(this + 0x14), 0x8200);
        // A request for more than what is left fails and changes nothing.
        let (block, _) = alloc(&mut e, this, 0x8000);
        assert_eq!(block, 0);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(e.mem.u32(this + 0xc), 0x1_8200);
        // Exactly what is left is granted.
        let (block, _) = alloc(&mut e, this, 0x7e00);
        assert_eq!(block, 0x1_8200);
        assert_eq!(e.mem.u32(this + 0xc), 0x2_0000);
    }

    #[test]
    fn buf_alloc_refuses_when_the_live_count_reaches_the_limit() {
        // Capacity 8 gives 16 slots; with 2 list entries the limit is 14: 13 live + 1 is not below it.
        let list = [(0x1_1000, 0x100), (0x1_2000, 0x100)];
        let (mut e, this) = allocator(&Fields { live: 13, ..FIELDS }, &list);
        let (block, _) = alloc(&mut e, this, 0x10);
        assert_eq!(block, 0);
        assert_eq!(e.mem.u32(this + 0x10), 13);
        // One live less: allowed.
        e.mem.set_u32(this + 0x10, 12);
        let (block, _) = alloc(&mut e, this, 0x10);
        assert_ne!(block, 0);
    }

    #[test]
    fn buf_alloc_takes_the_smallest_block_that_fits_whole_and_closes_the_gap() {
        let list = [
            (0x1_1000, 0x300),
            (0x1_2000, 0x180),
            (0x1_3000, 0x100),
            (0x1_4000, 0x180),
        ];
        let (mut e, this) = allocator(&FIELDS, &list);
        // 0x150 fits in 0x300, 0x180 and 0x180: the last 0x180 is found first and kept (a strictly
        // smaller block is needed to replace it). 0x180 * 2 <= 0x150 * 3: taken whole.
        let (block, granted) = alloc(&mut e, this, 0x150);
        assert_eq!((block, granted), (0x1_4000, 0x180));
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x300), (0x1_2000, 0x180), (0x1_3000, 0x100)]
        );
        // The top was not touched; the live count and peak were updated.
        assert_eq!(e.mem.u32(this + 0xc), 0x1_8000);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(e.mem.u32(this + 0x14), 0x8000);
        // The middle entry goes: the entries after it move down.
        let (block, granted) = alloc(&mut e, this, 0x110);
        assert_eq!((block, granted), (0x1_2000, 0x180));
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x300), (0x1_3000, 0x100)]
        );
    }

    #[test]
    fn buf_alloc_splits_a_much_larger_block_when_the_request_is_big() {
        let list = [(0x1_1000, 0x2000)];
        let (mut e, this) = allocator(&FIELDS, &list);
        // 0x2000 * 2 > 0x800 * 3 and 0x800 > 0x400: the front 0x800 is taken.
        let (block, granted) = alloc(&mut e, this, 0x800);
        assert_eq!((block, granted), (0x1_1000, 0x800));
        assert_eq!(free_list(&e, this), vec![(0x1_1800, 0x1800)]);
        // A request of 0x400 or less takes a larger block whole, whatever its size.
        let (block, granted) = alloc(&mut e, this, 0x400);
        assert_eq!((block, granted), (0x1_1800, 0x1800));
        assert!(free_list(&e, this).is_empty());
    }

    #[test]
    fn buf_alloc_ignores_free_blocks_that_are_too_small() {
        let list = [(0x1_1000, 0x40)];
        let (mut e, this) = allocator(&FIELDS, &list);
        let (block, _) = alloc(&mut e, this, 0x100);
        assert_eq!(block, 0x1_8000);
        assert_eq!(free_list(&e, this), vec![(0x1_1000, 0x40)]);
    }

    fn free(e: &mut Engine, this: u32, block: u32, size: u32) {
        e.call(0x00d5_eb10, &args![this, block, size]);
    }

    #[test]
    fn buf_free_of_the_top_block_lowers_the_top_and_absorbs_adjacent_entries() {
        // The list holds an entry ending at 0x1_7f00 (the block's start) and an earlier one.
        let list = [(0x1_1000, 0x100), (0x1_7e00, 0x100)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_7f00, 0x100);
        // The block started at 0x1_7f00 and ended at 0x1_8000 (the top).
        assert_eq!(e.mem.u32(this + 0x10), 2);
        // The top first drops by the size, then to the adjacent entry's address.
        assert_eq!(e.mem.u32(this + 0xc), 0x1_7e00);
        assert_eq!(free_list(&e, this), vec![(0x1_1000, 0x100)]);
        // Without an adjacent entry only the top changes.
        let (mut e, this) = allocator(&FIELDS, &[(0x1_1000, 0x100)]);
        free(&mut e, this, 0x1_7f00, 0x100);
        assert_eq!(e.mem.u32(this + 0xc), 0x1_7f00);
        assert_eq!(free_list(&e, this), vec![(0x1_1000, 0x100)]);
        // Entries below the top block's start are all absorbed only when each ends at it:
        // a single entry ending there is removed and the list becomes empty.
        let (mut e, this) = allocator(&FIELDS, &[(0x1_7e00, 0x100)]);
        free(&mut e, this, 0x1_7f00, 0x100);
        assert!(free_list(&e, this).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0x1_7e00);
    }

    #[test]
    fn buf_free_of_the_first_inner_block_starts_the_list() {
        let (mut e, this) = allocator(&FIELDS, &[]);
        free(&mut e, this, 0x1_2000, 0x100);
        assert_eq!(free_list(&e, this), vec![(0x1_2000, 0x100)]);
        assert_eq!(e.mem.u32(this + 0xc), 0x1_8000);
        assert_eq!(e.mem.u32(this + 0x10), 2);
    }

    #[test]
    fn buf_free_extends_the_previous_entry_and_merges_with_the_next() {
        // Entry A ends where the block starts; entry B starts where the block ends.
        let list = [(0x1_1000, 0x100), (0x1_1200, 0x80), (0x1_5000, 0x10)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_1100, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x280), (0x1_5000, 0x10)]
        );
        // Extending the previous entry only (the next does not start at the block's end).
        let list = [(0x1_1000, 0x100), (0x1_1300, 0x80)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_1100, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x200), (0x1_1300, 0x80)]
        );
        // The previous entry is the last one: nothing to merge with.
        let list = [(0x1_1000, 0x100)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_1100, 0x40);
        assert_eq!(free_list(&e, this), vec![(0x1_1000, 0x140)]);
    }

    #[test]
    fn buf_free_extends_the_next_entry_downwards_or_inserts_a_new_one() {
        // The block ends where the next entry starts: that entry now starts at the block.
        let list = [(0x1_1000, 0x40), (0x1_3000, 0x80)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_2f00, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x40), (0x1_2f00, 0x180)]
        );
        // Neither neighbour touches: a new entry is inserted in address order through
        // `00d5ed80` (allocator argument 0, one entry, after the entry below the block).
        let list = [(0x1_1000, 0x40), (0x1_5000, 0x80)];
        let (mut e, this) = allocator(&FIELDS, &list);
        e.call_log = Some(vec![]);
        // The inserted pair sits at +0x10 of the exe's 0x18-byte frame (a stack local, whose
        // bytes stay as they were left until another local takes its place).
        let frame = e.next_stack_local(0x18);
        free(&mut e, this, 0x1_3000, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x40), (0x1_3000, 0x100), (0x1_5000, 0x80)]
        );
        assert_eq!(
            (e.mem.u32(frame + 0x10), e.mem.u32(frame + 0x14)),
            (0x1_3000, 0x100)
        );
        let inserted: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_INSERT_AT)
            .collect();
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0].1[..3], [this + 0x18, 0, 1]);
        assert_eq!(inserted[0].1[4], 1);
    }

    #[test]
    fn buf_free_below_every_entry_inserts_at_the_front_or_extends_the_first() {
        let list = [(0x1_5000, 0x80)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_1000, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x100), (0x1_5000, 0x80)]
        );
        // The first entry starts where the block ends: it grows downwards.
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_4f00, 0x100);
        assert_eq!(free_list(&e, this), vec![(0x1_4f00, 0x180)]);
    }

    #[test]
    fn buf_free_after_the_last_entry_appends() {
        let list = [(0x1_1000, 0x40)];
        let (mut e, this) = allocator(&FIELDS, &list);
        free(&mut e, this, 0x1_2000, 0x100);
        assert_eq!(
            free_list(&e, this),
            vec![(0x1_1000, 0x40), (0x1_2000, 0x100)]
        );
    }

    #[test]
    fn the_splice_inserts_removes_and_grows() {
        let (mut e, this) = allocator(&FIELDS, &[(1, 10), (2, 20), (3, 30)]);
        let array = this + 0x18;
        let source = e.mem.alloc(16);
        e.mem.set_u32(source, 7);
        e.mem.set_u32(source + 4, 70);
        e.mem.set_u32(source + 8, 8);
        e.mem.set_u32(source + 12, 80);
        // Insert two entries at index 1, removing none.
        e.call(0x00d5_ecc0, &args![array, 0u32, 1i32, 0i32, source, 2i32]);
        assert_eq!(
            free_list(&e, this),
            vec![(1, 10), (7, 70), (8, 80), (2, 20), (3, 30)]
        );
        // Replace the two at index 1 by one.
        e.call(0x00d5_ecc0, &args![array, 0u32, 1i32, 2i32, source, 1i32]);
        assert_eq!(
            free_list(&e, this),
            vec![(1, 10), (7, 70), (2, 20), (3, 30)]
        );
        // Remove one without inserting.
        e.call(0x00d5_ecc0, &args![array, 0u32, 0i32, 1i32, source, 0i32]);
        assert_eq!(free_list(&e, this), vec![(7, 70), (2, 20), (3, 30)]);
        // Growing: capacity 4 is below the new size 5: `_reserve` is asked for max(8, 5) = 8 entries.
        e.register_double(ARRAY_RESERVE, |e, a| {
            let (array, count) = (a[1], a[2]);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let data = e.mem.alloc(count * 8);
            let kept = e.mem.bytes(old, size * 8);
            e.mem.write(data, &kept);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(array + 8, 4);
        e.call(
            0x00d5_ecc0,
            &args![array, 0x99u32, 3i32, 0i32, source, 2i32],
        );
        let reserve: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_RESERVE)
            .collect();
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0].1, vec![0x99, array, 8, 8]);
        assert_eq!(
            free_list(&e, this),
            vec![(7, 70), (2, 20), (3, 30), (7, 70), (8, 80)]
        );
    }

    #[test]
    fn the_single_insert_passes_one_entry_to_the_insert_function() {
        let mut e = Engine::new();
        e.register(ARRAY_INSERT_AT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d5_eda0, &args![0x1000u32, 0x11u32, 0x22u32, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ARRAY_INSERT_AT, vec![0x1000, 0x11, 0x22, 0x33, 1]));
    }
}
