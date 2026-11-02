//! `NiXenonRenderer/run_00e7ef80` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

/// Allocates a 0x18-byte block (four zero words, then `size` at `+0x10` and `+0x14`) and
/// keeps it in `slot` (null when the allocation fails).
fn make_array_header(e: &mut Engine, slot: u32, size: u32) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(slot, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, size);
    e.mem.set_u32(block + 0x14, size);
    e.mem.set_u32(slot, block);
}

// Translated from 00e7f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiXenonRenderer subsystem; no arguments):
/// builds two 0x18-byte blocks from the memory manager, kept in `0126f718` (sizes 8) and
/// `0126f71c` (sizes 0x10); null in a slot whose allocation fails.
pub fn fn_00e7f2b0(e: &mut Engine) {
    make_array_header(e, 0x0126_f718, 8);
    make_array_header(e, 0x0126_f71c, 0x10);
}

/// The render-state block pool (`0126f718`) and the render-state entry pool (`0126f71c`), the
/// two blocks `fn_00e7f2b0` makes (element array at +0, count at +8, growth size at +0x10).
const BLOCK_POOL_0126F718: u32 = 0x0126_f718;
const ENTRY_POOL_0126F71C: u32 = 0x0126_f71c;
/// The pools' grows: `00e7f160` for the block pool and `00e7f210` for the entry pool (unnamed;
/// `thiscall` on the pool, one word: the growth size).
const BLOCK_POOL_GROW_00E7F160: u32 = 0x00e7_f160;
const ENTRY_POOL_GROW_00E7F210: u32 = 0x00e7_f210;
/// The block's entry lookup `00e7f000` (unnamed; `thiscall` on the block: the state and the
/// address of a byte it sets to the list the entry is in; answers the entry or 0) and the
/// unlinks from the second list (`00e7efc0`) and from the first (`00e7ef80`) (unnamed;
/// `thiscall` on the block, one word: the entry).
const FIND_ENTRY_00E7F000: u32 = 0x00e7_f000;
const UNLINK_SECOND_00E7EFC0: u32 = 0x00e7_efc0;
const UNLINK_FIRST_00E7EF80: u32 = 0x00e7_ef80;
/// `NiTPrimitiveSet<..>::AddUnique` (`00e7ec00`, `thiscall` on the set, the address of the
/// element; the engine map names it `NiControllerSequence::SetState`), which gives a block or
/// an entry back to its pool, the block's clean-up `00e7f320` (unnamed; `thiscall`, no stack
/// words) and the Ni free `00aa1460` (`cdecl`: block, size).
const POOL_ADD_UNIQUE_00E7EC00: u32 = 0x00e7_ec00;
const BLOCK_CLEAN_UP_00E7F320: u32 = 0x00e7_f320;
const NI_FREE_00AA1460: u32 = 0x00aa_1460;

/// The first element taken from a pool: when the pool is empty it is grown (`grow` with the
/// growth size at +0x10) and that size doubled; then the element at +0 of the array is taken,
/// the count at +8 goes down and the last element moves into the first place.
fn pool_take(e: &mut Engine, pool: u32, grow: u32) -> u32 {
    if e.mem.u32(pool.wrapping_add(8)) == 0 {
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        e.call(grow, &args![pool, growth]);
        let growth = e.mem.u32(pool.wrapping_add(0x10));
        e.mem
            .set_u32(pool.wrapping_add(0x10), growth.wrapping_add(growth));
    }
    let elements = e.mem.u32(pool);
    let element = e.mem.u32(elements);
    let count = e.mem.u32(pool.wrapping_add(8)).wrapping_sub(1);
    e.mem.set_u32(pool.wrapping_add(8), count);
    let last = e.mem.u32(elements.wrapping_add(count.wrapping_mul(4)));
    e.mem.set_u32(elements, last);
    element
}

/// An entry appended to one of the block's lists (count at `count`, last entry at `last`):
/// linked after the last entry when there is one (the last's +0xc and the entry's +8), the
/// entry's +0xc cleared, the count up by one and the entry made the last.
fn append_entry(e: &mut Engine, block: u32, entry: u32, count: u32, last: u32) {
    let tail = e.mem.u32(block + last);
    if tail != 0 {
        e.mem.set_u32(tail.wrapping_add(0xc), entry);
        let tail = e.mem.u32(block + last);
        e.mem.set_u32(entry.wrapping_add(8), tail);
    }
    e.mem.set_u32(entry.wrapping_add(0xc), 0);
    let entries = e.mem.u32(block + count).wrapping_add(1);
    e.mem.set_u32(block + count, entries);
    e.mem.set_u32(block + last, entry);
}

// Translated from 00e7f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): a render state set in a render-state block,
/// `thiscall` with three stack words: the state, the value and a byte that picks the list.
/// Looks the state up (`00e7f000`, with a cleared byte it sets to the entry's list). When
/// found: the entry's value (+4) is replaced; when its list differs from the byte, it is
/// unlinked (`00e7efc0` from the second list, `00e7ef80` from the first) and appended to the
/// other one. When not found: a new entry is taken from the entry pool (`0126f71c`, grown by
/// `00e7f210` when empty), its +8 and +0xc cleared, the state and value stored, and appended to
/// the second list (count +0xc, last +0x10) when the byte is set, else to the first (count +4,
/// last +8).
pub fn fn_00e7f430(e: &mut Engine, this: Ptr, state: u32, value: u32, second_list: u8) {
    let base = this.addr();
    let (entry, in_second) = e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u8(local, 0);
        let entry = e
            .call(FIND_ENTRY_00E7F000, &args![base, state, local])
            .u32();
        (entry, e.mem.u8(local))
    });
    if entry != 0 {
        e.mem.set_u32(entry + 4, value);
        if in_second == second_list {
            return;
        }
        if in_second != 0 {
            e.call(UNLINK_SECOND_00E7EFC0, &args![base, entry]);
            append_entry(e, base, entry, 4, 8);
        } else {
            e.call(UNLINK_FIRST_00E7EF80, &args![base, entry]);
            append_entry(e, base, entry, 0xc, 0x10);
        }
        return;
    }
    let pool = e.global::<u32>(ENTRY_POOL_0126F71C);
    let entry = pool_take(e, pool, ENTRY_POOL_GROW_00E7F210);
    e.mem.set_u32(entry.wrapping_add(8), 0);
    e.mem.set_u32(entry.wrapping_add(0xc), 0);
    e.mem.set_u32(entry, state);
    e.mem.set_u32(entry.wrapping_add(4), value);
    if second_list != 0 {
        append_entry(e, base, entry, 0xc, 0x10);
    } else {
        append_entry(e, base, entry, 4, 8);
    }
}

// Translated from 00e7f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): a new render-state block, no arguments. Takes
/// the first block of the block pool (`0126f718`; grown by `00e7f160` with its growth size,
/// which then doubles, when empty), sets the block's first byte when it is clear and answers
/// the block.
pub fn fn_00e7f5d0(e: &mut Engine) -> u32 {
    let pool = e.global::<u32>(BLOCK_POOL_0126F718);
    let block = pool_take(e, pool, BLOCK_POOL_GROW_00E7F160);
    if e.mem.u8(block) == 0 {
        e.mem.set_u8(block, 1);
    }
    block
}

/// Every entry of one of the block's lists, from the last one (at `last`) back through each
/// entry's +8, given back to the entry pool (`00e7ec00` on the pool read from `0126f71c` for
/// each, with `slot` holding the entry).
fn release_entries(e: &mut Engine, block: u32, last: u32, slot: u32) {
    let mut entry = e.mem.u32(block + last);
    while entry != 0 {
        let previous = e.mem.u32(entry.wrapping_add(8));
        let pool = e.global::<u32>(ENTRY_POOL_0126F71C);
        e.mem.set_u32(slot, entry);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
        entry = previous;
    }
}

// Translated from 00e7f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): a render-state block's release, `cdecl` with one
/// word (the block). Nothing for a null block. When the block's first byte is set (a pool
/// block): the entries of the first list (last at +8) go back to the entry pool, +8 and +4 are
/// cleared, the same for the second list (last at +0x10, cleared with +0xc), and the block goes
/// back to the block pool (`00e7ec00` on `0126f718`; the argument's own stack word holds the
/// element each time). Otherwise the block is cleaned up (`00e7f320`) and freed (`00aa1460`
/// with 0x14 bytes).
pub fn fn_00e7f6b0(e: &mut Engine, block: Ptr) {
    let block = block.addr();
    if block == 0 {
        return;
    }
    if e.mem.u8(block) == 0 {
        e.call(BLOCK_CLEAN_UP_00E7F320, &args![block]);
        e.call(NI_FREE_00AA1460, &args![block, 0x14u32]);
        return;
    }
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        release_entries(e, block, 8, slot);
        e.mem.set_u32(block + 8, 0);
        e.mem.set_u32(block + 4, 0);
        release_entries(e, block, 0x10, slot);
        e.mem.set_u32(block + 0x10, 0);
        e.mem.set_u32(block + 0xc, 0);
        let pool = e.global::<u32>(BLOCK_POOL_0126F718);
        e.mem.set_u32(slot, block);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

/// `NiTSet<..>::Realloc` (`00a311e0`, `thiscall` on the set, one word: the new capacity) the
/// pools' grows call when their set is full.
const SET_REALLOC_00A311E0: u32 = 0x00a3_11e0;

/// A pool grow's block made inline (0xc bytes from the memory manager; null when that fails):
/// +4 the growth size, +8 cleared, +0 the elements `maker` (`cdecl`, the count) makes, or 0 for
/// a growth of 0.
pub(crate) fn new_pool_block(e: &mut Engine, growth: u32, maker: u32) -> u32 {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0xcu32],
        )
        .u32();
    if block != 0 {
        e.mem.set_u32(block + 4, growth);
        e.mem.set_u32(block + 8, 0);
        if growth != 0 {
            let elements = e.call(maker, &args![growth]).u32();
            e.mem.set_u32(block, elements);
        } else {
            e.mem.set_u32(block, 0);
        }
    }
    block
}

/// The rest of a pool grow (pool: element set at +0, capacity +4, count +8, block chain +0xc):
/// for each of the `growth` new elements (its address in the block's array while the index is
/// below the block's +4, else 0; the block's words read even when it is null) the set grows
/// when full (`00a311e0` to twice the capacity, 1 when it is 0) and the element is appended;
/// then the block is linked in front of the chain (its +8 takes the old head, written even
/// through a null block).
pub(crate) fn pool_add_block(e: &mut Engine, pool: u32, growth: u32, block: u32, size: u32) {
    let mut index = 0u32;
    let mut offset = 0u32;
    while index < growth {
        let element = if index < e.mem.u32(block.wrapping_add(4)) {
            e.mem.u32(block).wrapping_add(offset)
        } else {
            0
        };
        let capacity = e.mem.u32(pool + 4);
        if e.mem.u32(pool + 8) == capacity {
            let new = if capacity != 0 {
                capacity.wrapping_add(capacity)
            } else {
                1
            };
            e.call(SET_REALLOC_00A311E0, &args![pool, new]);
        }
        let used = e.mem.u32(pool + 8);
        let elements = e.mem.u32(pool);
        e.mem
            .set_u32(elements.wrapping_add(used.wrapping_mul(4)), element);
        e.mem.set_u32(pool + 8, used.wrapping_add(1));
        index += 1;
        offset = offset.wrapping_add(size);
    }
    let chain = e.mem.u32(pool + 0xc);
    e.mem.set_u32(block.wrapping_add(8), chain);
    e.mem.set_u32(pool + 0xc, block);
}

// Translated from 00e7f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): a render-state block's entry lookup, `thiscall`
/// with two stack words: the state and the address of a byte. Walks the first list (from its
/// last entry at +8 back through each entry's +8) and then the second (from +0x10) for an entry
/// whose +0 is the state; the byte becomes 0 or 1 for the list it was found in and the entry is
/// answered; 0 when there is none (the byte not written).
pub fn fn_00e7f000(e: &mut Engine, this: Ptr, state: u32, list: Ptr) -> u32 {
    let base = this.addr();
    for (last, which) in [(8u32, 0u8), (0x10, 1)] {
        let mut entry = e.mem.u32(base + last);
        while entry != 0 {
            if e.mem.u32(entry) == state {
                e.mem.set_u8(list.addr(), which);
                return entry;
            }
            entry = e.mem.u32(entry + 8);
        }
    }
    0
}

/// The element makers of the block pool (`00e7f0a0`, 0x14-byte blocks) and the entry pool's
/// block constructor (`00e7f0f0`, `thiscall` on the 0xc-byte block, one word: the count; 0x10-byte
/// entries) (unnamed; `cdecl` with the count for the first).
const NEW_BLOCKS_00E7F0A0: u32 = 0x00e7_f0a0;
const ENTRY_BLOCK_CONSTRUCT_00E7F0F0: u32 = 0x00e7_f0f0;

// Translated from 00e7f160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the render-state block pool's grow, `thiscall`
/// on the pool with one stack word (the growth size): a new pool block of that many 0x14-byte
/// blocks (`00e7f0a0`) whose blocks are appended to the pool's set and the block chained (see
/// [`pool_add_block`]).
pub fn fn_00e7f160(e: &mut Engine, this: Ptr, growth: u32) {
    let block = new_pool_block(e, growth, NEW_BLOCKS_00E7F0A0);
    pool_add_block(e, this.addr(), growth, block, 0x14);
}

// Translated from 00e7f210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the render-state entry pool's grow, `thiscall` on
/// the pool with one stack word (the growth size): 0xc bytes from the memory manager built by
/// `00e7f0f0` with the growth size (null when the allocation fails), whose 0x10-byte entries are
/// appended to the pool's set and the block chained (see [`pool_add_block`]).
pub fn fn_00e7f210(e: &mut Engine, this: Ptr, growth: u32) {
    let memory = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0xcu32],
        )
        .u32();
    let block = if memory != 0 {
        e.call(ENTRY_BLOCK_CONSTRUCT_00E7F0F0, &args![memory, growth])
            .u32()
    } else {
        0
    };
    pool_add_block(e, this.addr(), growth, block, 0x10);
}

/// An element added to a pool's set unless it is already there, as `00e7f320` expands it: the
/// set's elements are searched (an index of -1 counts as not found); when absent the set grows
/// when full (`00a311e0`) and the element is appended, and the pool is read again from
/// `0126f71c`. Answers the pool the next search uses.
fn add_unique(e: &mut Engine, pool: u32, element: u32) -> u32 {
    let used = e.mem.u32(pool + 8);
    let elements = e.mem.u32(pool);
    let mut found = None;
    for index in 0..used {
        if e.mem.u32(elements.wrapping_add(4 * index)) == element {
            found = Some(index);
            break;
        }
    }
    if let Some(index) = found {
        if index != 0xffff_ffff {
            return pool;
        }
    }
    let capacity = e.mem.u32(pool + 4);
    if used == capacity {
        let new = if capacity != 0 {
            capacity.wrapping_add(capacity)
        } else {
            1
        };
        e.call(SET_REALLOC_00A311E0, &args![pool, new]);
    }
    let used = e.mem.u32(pool + 8);
    let elements = e.mem.u32(pool);
    e.mem
        .set_u32(elements.wrapping_add(used.wrapping_mul(4)), element);
    e.mem.set_u32(pool + 8, used.wrapping_add(1));
    e.global::<u32>(ENTRY_POOL_0126F71C)
}

// Translated from 00e7f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): a render-state block's clean-up, `thiscall`, no
/// stack words. Every entry of the first list (from +8 back through each entry's +8) and then
/// of the second (from +0x10) goes back to the entry pool (`0126f71c`, read once and again
/// after each append) unless it is already in its set, then +8 and +0x10 are cleared (the
/// counts +4 and +0xc are kept).
pub fn fn_00e7f320(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut pool = e.global::<u32>(ENTRY_POOL_0126F71C);
    for last in [8u32, 0x10] {
        let mut entry = e.mem.u32(base + last);
        while entry != 0 {
            let previous = e.mem.u32(entry + 8);
            pool = add_unique(e, pool, entry);
            entry = previous;
        }
    }
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0);
}

/// The Ni array allocation `00aa1420` (`cdecl`, the byte count), the compiler's `vector
/// constructor iterator` (`00401050`, `stdcall`: array, element size, count, constructor) and
/// the element constructors it is given: `00e7ee80` for the 0x14-byte render-state blocks and
/// `00e7ee70` for the 0x10-byte entries (code Ghidra made no functions of).
const NI_ARRAY_ALLOCATE_00AA1420: u32 = 0x00aa_1420;
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const BLOCK_CONSTRUCT_00E7EE80: u32 = 0x00e7_ee80;
const ENTRY_CONSTRUCT_00E7EE70: u32 = 0x00e7_ee70;

/// `count * size` as the code computes it (`MUL`, `SETO`, `NEG`, `OR`): all ones on overflow.
fn product_or_all_ones(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    }
}

// Translated from 00e7f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): `operator new[]` of `count` 0x14-byte
/// render-state blocks after a count word, `cdecl` with one word (the count): `count * 0x14 + 4`
/// bytes from `00aa1420` (all ones when the product or the sum overflows); null when that fails,
/// otherwise the count is stored, the blocks are built (`vector constructor iterator` with
/// `00e7ee80`) and the address after the count is answered.
pub fn fn_00e7f0a0(e: &mut Engine, count: u32) -> u32 {
    let total = product_or_all_ones(count, 0x14)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(NI_ARRAY_ALLOCATE_00AA1420, &args![total]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![block + 4, 0x14u32, count, BLOCK_CONSTRUCT_00E7EE80],
    );
    block + 4
}

// Translated from 00e7f0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the entry pool's block constructor, `thiscall`
/// with one stack word (the count): +4 the count, +8 cleared, +0 an array of `count` 0x10-byte
/// entries (`00aa1420` with `count * 0x10`, all ones on overflow; built by the `vector
/// constructor iterator` with `00e7ee70`), null for a count of 0 or a failed allocation.
/// Answers `this`.
pub fn fn_00e7f0f0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base + 4, count);
    e.mem.set_u32(base + 8, 0);
    if count == 0 {
        e.mem.set_u32(base, 0);
        return this;
    }
    let entries = e
        .call(
            NI_ARRAY_ALLOCATE_00AA1420,
            &args![product_or_all_ones(count, 0x10)],
        )
        .u32();
    if entries == 0 {
        e.mem.set_u32(base, 0);
        return this;
    }
    e.call(
        VECTOR_CONSTRUCT_00401050,
        &args![entries, 0x10u32, count, ENTRY_CONSTRUCT_00E7EE70],
    );
    e.mem.set_u32(base, entries);
    this
}

/// An entry unlinked from one of the block's lists (count at `count`, last entry at `last`):
/// its neighbours (+8 the one before, +0xc the one after) linked to each other, the last
/// entry moved back when it is the entry, the count down by one and the entry's links
/// cleared.
fn unlink_entry(e: &mut Engine, block: u32, entry: u32, count: u32, last: u32) {
    let before = e.mem.u32(entry.wrapping_add(8));
    let after = e.mem.u32(entry.wrapping_add(0xc));
    if before != 0 {
        e.mem.set_u32(before.wrapping_add(0xc), after);
    }
    if after != 0 {
        e.mem.set_u32(after.wrapping_add(8), before);
    }
    if entry == e.mem.u32(block.wrapping_add(last)) {
        e.mem.set_u32(block.wrapping_add(last), before);
    }
    let entries = e.mem.u32(block.wrapping_add(count)).wrapping_sub(1);
    e.mem.set_u32(block.wrapping_add(count), entries);
    e.mem.set_u32(entry.wrapping_add(8), 0);
    e.mem.set_u32(entry.wrapping_add(0xc), 0);
}

// Translated from 00e7ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a render-state block's unlink of `entry` from its first list
/// (`thiscall`, one word; count +4, last entry +8): the neighbours linked to each other, the
/// last entry moved back when it is this one, the count down and the entry's links (+8,
/// +0xc) cleared.
pub fn fn_00e7ef80(e: &mut Engine, this: Ptr, entry: u32) {
    unlink_entry(e, this.addr(), entry, 4, 8);
}

// Translated from 00e7efc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: as [`fn_00e7ef80`] for the block's second list (count +0xc,
/// last entry +0x10).
pub fn fn_00e7efc0(e: &mut Engine, this: Ptr, entry: u32) {
    unlink_entry(e, this.addr(), entry, 0xc, 0x10);
}

// Translated from 00e7f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a state; called by the fragment shader's
/// alpha set-up): looks the state up in the render-state block (`00e7f000`, with a cleared
/// byte it sets to the entry's list); when found the entry is unlinked from its list (the
/// exe inlines [`fn_00e7efc0`] for the second list and [`fn_00e7ef80`] for the first) and
/// given back to the entry pool (`0126f71c`, `AddUnique` with the address of a word holding
/// it, the argument's own slot in the exe).
pub fn fn_00e7f550(e: &mut Engine, this: Ptr, state: u32) {
    let base = this.addr();
    let (entry, in_second) = e.with_stack(4, |e, local| {
        let flag = local.addr() + 3;
        e.mem.set_u8(flag, 0);
        let entry = e.call(FIND_ENTRY_00E7F000, &args![base, state, flag]).u32();
        (entry, e.mem.u8(flag))
    });
    if entry == 0 {
        return;
    }
    if in_second != 0 {
        unlink_entry(e, base, entry, 0xc, 0x10);
    } else {
        unlink_entry(e, base, entry, 4, 8);
    }
    let pool = e.mem.u32(ENTRY_POOL_0126F71C);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(POOL_ADD_UNIQUE_00E7EC00, &args![pool, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e7_ef80, fn_00e7ef80(Ptr, u32)),
        entry!(0x00e7_efc0, fn_00e7efc0(Ptr, u32)),
        entry!(0x00e7_f550, fn_00e7f550(Ptr, u32)),
        entry!(0x00e7_f2b0, fn_00e7f2b0()),
        entry!(0x00e7_f430, fn_00e7f430(Ptr, u32, u32, u8)),
        entry!(0x00e7_f5d0, fn_00e7f5d0() -> u32),
        entry!(0x00e7_f6b0, fn_00e7f6b0(Ptr)),
        entry!(0x00e7_f000, fn_00e7f000(Ptr, u32, Ptr) -> u32),
        entry!(0x00e7_f160, fn_00e7f160(Ptr, u32)),
        entry!(0x00e7_f210, fn_00e7f210(Ptr, u32)),
        entry!(0x00e7_f320, fn_00e7f320(Ptr)),
        entry!(0x00e7_f0a0, fn_00e7f0a0(u32) -> u32),
        entry!(0x00e7_f0f0, fn_00e7f0f0(Ptr, u32) -> Ptr),
        entry!(0x00e7_f050, fn_00e7f050(Ptr, u32) -> Ptr),
        entry!(0x00e7_f070, fn_00e7f070(Ptr)),
        entry!(0x00e7_f610, fn_00e7f610()),
        entry!(0x00e7_f750, fn_00e7f750(u32)),
        entry!(0x00e7_f790, fn_00e7f790(Ptr)),
        entry!(0x00e7_f7c0, fn_00e7f7c0(Ptr) -> u32),
    ]
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block), the Ni array
/// free `00aa1490` (`cdecl`: block, element size) and the free `00aa10f0` (`cdecl`, one word).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const NI_ARRAY_FREE_00AA1490: u32 = 0x00aa_1490;
const FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00e7f050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the scalar deleting destructor of the entry
/// pool's 0xc-byte block, `thiscall` with the flags: the destructor `00e7f070`, then with bit
/// 0 set the block given back to the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00e7f050(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00e7f070(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e7f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the destructor of the entry pool's 0xc-byte
/// block, `thiscall`, no stack words: its 0x10-byte entries freed (`00aa1490` with the array
/// at +0), then the next block of the chain (+8), when there is one, destroyed the same way
/// and given back to the memory manager.
pub fn fn_00e7f070(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let entries = e.mem.u32(base);
    e.call(NI_ARRAY_FREE_00AA1490, &args![entries, 0x10u32]);
    let next = e.mem.u32(base.wrapping_add(8));
    if next != 0 {
        fn_00e7f070(e, Ptr::new(next));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, next]);
    }
}

// Translated from 00e7f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`), no arguments: frees the two render-state pools
/// `fn_00e7f2b0` made. The block pool (`0126f718`): its count (+8) cleared, its chain (+0xc)
/// destroyed (`00e7f790`) and given back to the memory manager, its set's array (+0) freed
/// (`00aa10f0`) and the pool given back. The entry pool (`0126f71c`): the same, its chain's
/// first block destroyed as `00e7f070` inlines it (its entries freed, a next block deleted
/// through `00e7f050` with 1).
pub fn fn_00e7f610(e: &mut Engine) {
    let pool = e.mem.u32(BLOCK_POOL_0126F718);
    if pool != 0 {
        let chain = e.mem.u32(pool.wrapping_add(0xc));
        e.mem.set_u32(pool.wrapping_add(8), 0);
        if chain != 0 {
            fn_00e7f790(e, Ptr::new(chain));
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, chain]);
        }
        let elements = e.mem.u32(pool);
        e.call(FREE_00AA10F0, &args![elements]);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, pool]);
    }
    let pool = e.mem.u32(ENTRY_POOL_0126F71C);
    if pool != 0 {
        let chain = e.mem.u32(pool.wrapping_add(0xc));
        e.mem.set_u32(pool.wrapping_add(8), 0);
        if chain != 0 {
            let entries = e.mem.u32(chain);
            e.call(NI_ARRAY_FREE_00AA1490, &args![entries, 0x10u32]);
            let next = e.mem.u32(chain.wrapping_add(8));
            if next != 0 {
                fn_00e7f050(e, Ptr::new(next), 1);
            }
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, chain]);
        }
        let elements = e.mem.u32(pool);
        e.call(FREE_00AA10F0, &args![elements]);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, pool]);
    }
}

// Translated from 00e7f750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): `operator delete[]` of an array of 0x14-byte
/// render-state blocks made by `00e7f0a0`, `cdecl` with the array (the count is the word
/// before it). Nothing for a null array; otherwise each block from the last to the first is
/// cleaned up (`00e7f320`; the count runs down as a signed number, so a count of 0 runs none)
/// and the allocation, from the count word, freed (`00aa1490` with 0x14).
pub fn fn_00e7f750(e: &mut Engine, array: u32) {
    if array == 0 {
        return;
    }
    let count = e.mem.u32(array.wrapping_sub(4));
    let mut block = array.wrapping_add(count.wrapping_mul(0x14));
    let mut left = count.wrapping_sub(1);
    while (left as i32) >= 0 {
        block = block.wrapping_sub(0x14);
        fn_00e7f320(e, Ptr::new(block));
        left = left.wrapping_sub(1);
    }
    e.call(
        NI_ARRAY_FREE_00AA1490,
        &args![array.wrapping_sub(4), 0x14u32],
    );
}

// Translated from 00e7f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`): the destructor of the block pool's 0xc-byte
/// block, `thiscall`, no stack words: its blocks deleted (`00e7f750` with the array at +0),
/// then the next block of the chain (+8), when there is one, destroyed the same way and given
/// back to the memory manager.
pub fn fn_00e7f790(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let blocks = e.mem.u32(base);
    fn_00e7f750(e, blocks);
    let next = e.mem.u32(base.wrapping_add(8));
    if next != 0 {
        fn_00e7f790(e, Ptr::new(next));
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, next]);
    }
}

// Translated from 00e7f7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e7ef80`), no arguments: answers the word at `0126f728`.
pub fn fn_00e7f7c0(e: &mut Engine, _this: Ptr) -> u32 {
    e.mem.u32(0x0126_f728)
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_both_blocks() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            let block = e.mem.alloc(0x18);
            e.mem.write(block, &[0xaa; 0x18]);
            block.into_ret()
        });
        e.call(0x00e7_f2b0, &args![]);
        let first = e.mem.u32(0x0126_f718);
        let second = e.mem.u32(0x0126_f71c);
        assert_ne!(first, second);
        assert_eq!(e.mem.bytes(first, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(first + 0x10), 8);
        assert_eq!(e.mem.u32(first + 0x14), 8);
        assert_eq!(e.mem.u32(second + 0x10), 0x10);
        assert_eq!(e.mem.u32(second + 0x14), 0x10);
    }

    #[test]
    fn failed_allocations_leave_null() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(0x0126_f718, 1);
        e.mem.set_u32(0x0126_f71c, 1);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00e7_f2b0, &args![]);
        assert_eq!(e.mem.u32(0x0126_f718), 0);
        assert_eq!(e.mem.u32(0x0126_f71c), 0);
    }
}

#[cfg(test)]
mod render_state_tests {
    use super::*;

    /// An engine with both pools mapped: each pool holds `held` fresh elements (0x14 bytes,
    /// cleared) in a 16-element array, growth size 2; the grow doubles put one new element in.
    fn engine(held: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for global in [BLOCK_POOL_0126F718, ENTRY_POOL_0126F71C] {
            let pool = e.mem.alloc(0x18);
            let elements = e.mem.alloc(0x40);
            for index in 0..held {
                let element = e.mem.alloc(0x14);
                e.mem.set_u32(elements + 4 * index, element);
            }
            e.mem.set_u32(pool, elements);
            e.mem.set_u32(pool + 8, held);
            e.mem.set_u32(pool + 0x10, 2);
            e.set_global(global, pool);
        }
        fn grow(e: &mut Engine, a: &[u32]) -> Ret {
            let element = e.mem.alloc(0x14);
            let elements = e.mem.u32(a[0]);
            e.mem.set_u32(elements, element);
            e.mem.set_u32(a[0] + 8, 1);
            Ret::default()
        }
        e.register(BLOCK_POOL_GROW_00E7F160, grow);
        e.register(ENTRY_POOL_GROW_00E7F210, grow);
        for target in [
            UNLINK_SECOND_00E7EFC0,
            UNLINK_FIRST_00E7EF80,
            BLOCK_CLEAN_UP_00E7F320,
            NI_FREE_00AA1460,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        // The add-unique double records the element its slot holds at +0x3c of the pool.
        e.register(POOL_ADD_UNIQUE_00E7EC00, |e, a| {
            let element = e.mem.u32(a[1]);
            let seen = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, seen + 1);
            let elements = e.mem.u32(a[0]);
            e.mem.set_u32(elements + 0x3c - 4 * seen, element);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A lookup double that answers `entry` with the list byte `list`.
    fn find(e: &mut Engine, entry: u32, list: u8) {
        e.register_double(FIND_ENTRY_00E7F000, move |e, a| {
            assert_eq!(e.mem.u8(a[2]), 0);
            e.mem.set_u8(a[2], list);
            entry.into_ret()
        });
    }

    #[test]
    fn fn_00e7f5d0_takes_a_block_and_marks_it() {
        let mut e = engine(2);
        let pool = e.global::<u32>(BLOCK_POOL_0126F718);
        let elements = e.mem.u32(pool);
        let first = e.mem.u32(elements);
        let second = e.mem.u32(elements + 4);
        assert_eq!(e.call(0x00e7_f5d0, &args![]).u32(), first);
        assert_eq!(e.mem.u8(first), 1);
        assert_eq!(e.mem.u32(pool + 8), 1);
        assert_eq!(e.mem.u32(elements), second);
        // An empty pool is grown first and its growth size doubled.
        let mut e = engine(0);
        let pool = e.global::<u32>(BLOCK_POOL_0126F718);
        let block = e.call(0x00e7_f5d0, &args![]).u32();
        assert_eq!(calls(&e, BLOCK_POOL_GROW_00E7F160), vec![vec![pool, 2]]);
        assert_eq!(e.mem.u32(pool + 0x10), 4);
        assert_eq!(e.mem.u8(block), 1);
    }

    #[test]
    fn fn_00e7f430_appends_a_new_entry_to_the_chosen_list() {
        let mut e = engine(2);
        find(&mut e, 0, 0);
        let block = e.mem.alloc(0x14);
        e.call(0x00e7_f430, &args![block, 0x13u32, 5u32, 1u32]);
        let first = e.mem.u32(block + 0x10);
        assert_eq!(e.mem.u32(first), 0x13);
        assert_eq!(e.mem.u32(first + 4), 5);
        assert_eq!(e.mem.u32(first + 8), 0);
        assert_eq!(e.mem.u32(block + 0xc), 1);
        e.call(0x00e7_f430, &args![block, 0x14u32, 6u32, 1u32]);
        let second = e.mem.u32(block + 0x10);
        assert_eq!(e.mem.u32(first + 0xc), second);
        assert_eq!(e.mem.u32(second + 8), first);
        assert_eq!(e.mem.u32(block + 0xc), 2);
        e.call(0x00e7_f430, &args![block, 0x15u32, 7u32, 0u32]);
        let third = e.mem.u32(block + 8);
        assert_eq!(e.mem.u32(third), 0x15);
        assert_eq!(e.mem.u32(block + 4), 1);
        // The third entry came from a grown pool.
        let pool = e.global::<u32>(ENTRY_POOL_0126F71C);
        assert_eq!(calls(&e, ENTRY_POOL_GROW_00E7F210), vec![vec![pool, 2]]);
        assert_eq!(
            calls(&e, FIND_ENTRY_00E7F000)
                .iter()
                .map(|a| a[1])
                .collect::<Vec<_>>(),
            vec![0x13, 0x14, 0x15]
        );
    }

    #[test]
    fn fn_00e7f430_moves_a_found_entry_between_the_lists() {
        let mut e = engine(0);
        let block = e.mem.alloc(0x14);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry + 8, 0x77);
        // Found in the second list, asked for the first: unlinked and appended there.
        find(&mut e, entry, 1);
        e.call(0x00e7_f430, &args![block, 0x13u32, 9u32, 0u32]);
        assert_eq!(e.mem.u32(entry + 4), 9);
        assert_eq!(calls(&e, UNLINK_SECOND_00E7EFC0), vec![vec![block, entry]]);
        assert_eq!(e.mem.u32(block + 8), entry);
        assert_eq!(e.mem.u32(block + 4), 1);
        // No last entry: the entry's +8 is left as it was.
        assert_eq!(e.mem.u32(entry + 8), 0x77);
        // Found in the first list, asked for the second.
        find(&mut e, entry, 0);
        e.call(0x00e7_f430, &args![block, 0x13u32, 3u32, 1u32]);
        assert_eq!(calls(&e, UNLINK_FIRST_00E7EF80), vec![vec![block, entry]]);
        assert_eq!(e.mem.u32(block + 0x10), entry);
        assert_eq!(e.mem.u32(block + 0xc), 1);
        // Found in the list asked for: only the value changes.
        find(&mut e, entry, 1);
        e.call(0x00e7_f430, &args![block, 0x13u32, 4u32, 1u32]);
        assert_eq!(e.mem.u32(entry + 4), 4);
        assert_eq!(e.mem.u32(block + 0xc), 1);
        assert!(calls(&e, ENTRY_POOL_GROW_00E7F210).is_empty());
    }

    #[test]
    fn fn_00e7f6b0_gives_the_entries_and_the_block_back() {
        let mut e = engine(0);
        let block = e.mem.alloc(0x14);
        e.mem.set_u8(block, 1);
        let entries: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        // First list: entries 0 <- 1 (last); second list: entry 2.
        e.mem.set_u32(entries[1] + 8, entries[0]);
        e.mem.set_u32(block + 4, 2);
        e.mem.set_u32(block + 8, entries[1]);
        e.mem.set_u32(block + 0xc, 1);
        e.mem.set_u32(block + 0x10, entries[2]);
        e.call(0x00e7_f6b0, &args![block]);
        let entry_pool = e.global::<u32>(ENTRY_POOL_0126F71C);
        let block_pool = e.global::<u32>(BLOCK_POOL_0126F718);
        let given = |e: &Engine, pool: u32, count: u32| -> Vec<u32> {
            let elements = e.mem.u32(pool);
            (0..count)
                .map(|i| e.mem.u32(elements + 0x3c - 4 * i))
                .collect()
        };
        assert_eq!(
            given(&e, entry_pool, 3),
            vec![entries[1], entries[0], entries[2]]
        );
        assert_eq!(given(&e, block_pool, 1), vec![block]);
        assert_eq!(e.mem.bytes(block + 4, 0x10), vec![0; 0x10]);
        assert!(calls(&e, NI_FREE_00AA1460).is_empty());
    }

    #[test]
    fn fn_00e7f6b0_frees_a_block_not_from_the_pool() {
        let mut e = engine(0);
        let block = e.mem.alloc(0x14);
        e.call(0x00e7_f6b0, &args![block]);
        assert_eq!(calls(&e, BLOCK_CLEAN_UP_00E7F320), vec![vec![block]]);
        assert_eq!(calls(&e, NI_FREE_00AA1460), vec![vec![block, 0x14]]);
        e.call(0x00e7_f6b0, &args![0u32]);
        assert_eq!(calls(&e, NI_FREE_00AA1460).len(), 1);
        assert!(calls(&e, POOL_ADD_UNIQUE_00E7EC00).is_empty());
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine whose realloc double moves a set's elements into a new array of the asked
    /// capacity, the memory manager allocating, the makers answering arrays.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.register(SET_REALLOC_00A311E0, |e, a| {
            let used = e.mem.u32(a[0] + 8);
            let old = e.mem.u32(a[0]);
            let new = e.mem.alloc(4 * a[1]);
            for index in 0..used {
                let element = e.mem.u32(old + 4 * index);
                e.mem.set_u32(new + 4 * index, element);
            }
            e.mem.set_u32(a[0], new);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NEW_BLOCKS_00E7F0A0, |e, a| {
            e.mem.alloc(0x14 * a[0]).into_ret()
        });
        e.register(ENTRY_BLOCK_CONSTRUCT_00E7F0F0, |e, a| {
            let entries = e.mem.alloc(0x10 * a[1]);
            e.mem.set_u32(a[0], entries);
            e.mem.set_u32(a[0] + 4, a[1]);
            e.mem.set_u32(a[0] + 8, 0);
            a[0].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// A pool with an empty set (capacity 0) and an old block in its chain.
    fn pool(e: &mut Engine) -> u32 {
        let pool = e.mem.alloc(0x18);
        e.mem.set_u32(pool + 0xc, 0x0e00_0000);
        pool
    }

    #[test]
    fn fn_00e7f160_adds_the_new_blocks_to_the_pool() {
        let mut e = engine();
        let pool = pool(&mut e);
        e.call(0x00e7_f160, &args![pool, 3u32]);
        let block = e.mem.u32(pool + 0xc);
        assert_eq!(e.mem.u32(block + 8), 0x0e00_0000);
        let blocks = e.mem.u32(block);
        assert_eq!(e.mem.u32(pool + 8), 3);
        let elements = e.mem.u32(pool);
        let held: Vec<u32> = (0..3).map(|i| e.mem.u32(elements + 4 * i)).collect();
        assert_eq!(held, vec![blocks, blocks + 0x14, blocks + 0x28]);
        assert_eq!(calls(&e, NEW_BLOCKS_00E7F0A0), vec![vec![3]]);
        assert_eq!(
            calls(&e, SET_REALLOC_00A311E0),
            vec![vec![pool, 1], vec![pool, 2], vec![pool, 4]]
        );
    }

    #[test]
    fn fn_00e7f210_builds_the_entry_block_and_adds_its_entries() {
        let mut e = engine();
        let pool = pool(&mut e);
        e.call(0x00e7_f210, &args![pool, 2u32]);
        let block = e.mem.u32(pool + 0xc);
        assert_eq!(e.mem.u32(block + 8), 0x0e00_0000);
        let entries = e.mem.u32(block);
        let elements = e.mem.u32(pool);
        assert_eq!(e.mem.u32(elements), entries);
        assert_eq!(e.mem.u32(elements + 4), entries + 0x10);
        assert_eq!(
            calls(&e, ENTRY_BLOCK_CONSTRUCT_00E7F0F0),
            vec![vec![block, 2]]
        );
        // No growth: only the chain changes.
        let pool = self::pool(&mut e);
        e.call(0x00e7_f160, &args![pool, 0u32]);
        let block = e.mem.u32(pool + 0xc);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(pool + 8), 0);
    }

    #[test]
    fn fn_00e7f000_finds_an_entry_in_either_list() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x14);
        let first = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(first[0], 0x13);
        e.mem.set_u32(first[1], 0x14);
        e.mem.set_u32(first[1] + 8, first[0]);
        e.mem.set_u32(second, 0x15);
        e.mem.set_u32(block + 8, first[1]);
        e.mem.set_u32(block + 0x10, second);
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, 7);
        assert_eq!(
            e.call(0x00e7_f000, &args![block, 0x13u32, byte]).u32(),
            first[0]
        );
        assert_eq!(e.mem.u8(byte), 0);
        assert_eq!(
            e.call(0x00e7_f000, &args![block, 0x15u32, byte]).u32(),
            second
        );
        assert_eq!(e.mem.u8(byte), 1);
        e.mem.set_u8(byte, 7);
        assert_eq!(e.call(0x00e7_f000, &args![block, 0x16u32, byte]).u32(), 0);
        assert_eq!(e.mem.u8(byte), 7);
    }

    #[test]
    fn fn_00e7f320_gives_the_entries_back_once() {
        let mut e = engine();
        let pool = self::pool(&mut e);
        e.set_global(ENTRY_POOL_0126F71C, pool);
        let block = e.mem.alloc(0x14);
        let entries: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        // First list: 0 <- 1; second list: 2 <- 0 (the entry 0 again).
        e.mem.set_u32(entries[1] + 8, entries[0]);
        e.mem.set_u32(block + 4, 2);
        e.mem.set_u32(block + 8, entries[1]);
        e.mem.set_u32(block + 0xc, 1);
        e.mem.set_u32(block + 0x10, entries[2]);
        e.mem.set_u32(entries[2] + 8, entries[0]);
        e.call(0x00e7_f320, &args![block]);
        let elements = e.mem.u32(pool);
        assert_eq!(e.mem.u32(pool + 8), 3);
        let held: Vec<u32> = (0..3).map(|i| e.mem.u32(elements + 4 * i)).collect();
        assert_eq!(held, vec![entries[1], entries[0], entries[2]]);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(block + 4), 2);
        assert_eq!(e.mem.u32(block + 0xc), 1);
    }
}

#[cfg(test)]
mod b000p_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_ARRAY_ALLOCATE_00AA1420, |e, a| {
            if a[0] <= 0x1000 {
                e.mem.alloc(a[0].max(1)).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(VECTOR_CONSTRUCT_00401050, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00e7f0a0_makes_the_counted_block_array() {
        let mut e = engine();
        let blocks = e.call(0x00e7_f0a0, &args![3u32]).u32();
        assert_eq!(e.mem.u32(blocks - 4), 3);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420), vec![vec![0x40]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![blocks, 0x14, 3, BLOCK_CONSTRUCT_00E7EE80]]
        );
        assert_eq!(e.call(0x00e7_f0a0, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(calls(&e, NI_ARRAY_ALLOCATE_00AA1420)[1], vec![u32::MAX]);
    }

    #[test]
    fn fn_00e7f0f0_builds_the_entry_block() {
        let mut e = engine();
        let block = e.mem.alloc(0xc);
        e.mem.write(block, &[0xcc; 0xc]);
        assert_eq!(e.call(0x00e7_f0f0, &args![block, 2u32]).u32(), block);
        let entries = e.mem.u32(block);
        assert_eq!(e.mem.u32(block + 4), 2);
        assert_eq!(e.mem.u32(block + 8), 0);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![entries, 0x10, 2, ENTRY_CONSTRUCT_00E7EE70]]
        );
        e.call(0x00e7_f0f0, &args![block, 0u32]);
        assert_eq!(e.mem.u32(block), 0);
        e.call(0x00e7_f0f0, &args![block, 0x1000_0000u32]);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u32(block + 4), 0x1000_0000);
        assert_eq!(
            calls(&e, NI_ARRAY_ALLOCATE_00AA1420).last().unwrap(),
            &vec![u32::MAX]
        );
    }
}

#[cfg(test)]
mod b052_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        for no_op in [
            MEMORY_MANAGER_FREE,
            NI_ARRAY_FREE_00AA1490,
            FREE_00AA10F0,
            POOL_ADD_UNIQUE_00E7EC00,
            SET_REALLOC_00A311E0,
        ] {
            e.register(no_op, |_, _| Ret::default());
        }
        e
    }

    /// A chain of two 0xc-byte pool blocks with arrays 0xa100 and 0xa200.
    fn chain(e: &mut Engine) -> (u32, u32) {
        let (first, second) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.mem.set_u32(first, 0xa100);
        e.mem.set_u32(first + 8, second);
        e.mem.set_u32(second, 0xa200);
        (first, second)
    }

    #[test]
    fn entry_block_destructors_free_the_chain() {
        let mut e = engine();
        let (first, second) = chain(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e7_f050, &args![first, 1u32]).u32(), first);
        assert_eq!(
            calls(&e, NI_ARRAY_FREE_00AA1490),
            vec![vec![0xa100, 0x10], vec![0xa200, 0x10]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, second],
                vec![MEMORY_MANAGER_OBJECT, first]
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00e7_f050, &args![second, 0u32]);
        assert!(calls(&e, MEMORY_MANAGER_FREE).is_empty());
        assert_eq!(calls(&e, NI_ARRAY_FREE_00AA1490).len(), 1);
    }

    #[test]
    fn fn_00e7f750_cleans_up_each_block_last_first() {
        let mut e = engine();
        let pool = e.mem.alloc(0x18);
        e.mem.set_u32(ENTRY_POOL_0126F71C, pool);
        let allocation = e.mem.alloc(4 + 2 * 0x14);
        e.mem.set_u32(allocation, 2);
        let array = allocation + 4;
        // Each block holds one entry in its first list, so its clean-up gives it back.
        let mut entries = vec![];
        for index in 0..2u32 {
            let entry = e.mem.alloc(0x10);
            e.mem.set_u32(array + 0x14 * index + 8, entry);
            entries.push(entry);
        }
        let elements = e.mem.alloc(0x10);
        e.mem.set_u32(pool, elements);
        e.mem.set_u32(pool + 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x00e7_f750, &args![array]);
        assert_eq!(e.mem.u32(elements), entries[1]);
        assert_eq!(e.mem.u32(elements + 4), entries[0]);
        assert_eq!(
            calls(&e, NI_ARRAY_FREE_00AA1490),
            vec![vec![allocation, 0x14]]
        );
        // A count of 0: nothing cleaned up; a null array: nothing at all.
        e.mem.set_u32(allocation, 0);
        e.call(0x00e7_f750, &args![array]);
        assert_eq!(e.mem.u32(pool + 8), 2);
        e.call_log = Some(vec![]);
        e.call(0x00e7_f750, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e7f790_deletes_each_blocks_array() {
        let mut e = engine();
        let (first, second) = chain(&mut e);
        // Empty arrays (count 0) behind each block's array pointer.
        for block in [first, second] {
            let allocation = e.mem.alloc(4);
            e.mem.set_u32(block, allocation + 4);
        }
        e.call_log = Some(vec![]);
        e.call(0x00e7_f790, &args![first]);
        assert_eq!(calls(&e, NI_ARRAY_FREE_00AA1490).len(), 2);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, second]]
        );
    }

    #[test]
    fn fn_00e7f610_frees_both_pools() {
        let mut e = engine();
        let (block_pool, entry_pool) = (e.mem.alloc(0x18), e.mem.alloc(0x18));
        e.mem.set_u32(BLOCK_POOL_0126F718, block_pool);
        e.mem.set_u32(ENTRY_POOL_0126F71C, entry_pool);
        let block_chain = e.mem.alloc(0xc);
        e.mem.set_u32(block_chain, 0);
        e.mem.set_u32(block_pool, 0xb100);
        e.mem.set_u32(block_pool + 8, 3);
        e.mem.set_u32(block_pool + 0xc, block_chain);
        let (first, second) = chain(&mut e);
        e.mem.set_u32(entry_pool, 0xe100);
        e.mem.set_u32(entry_pool + 8, 5);
        e.mem.set_u32(entry_pool + 0xc, first);
        e.call_log = Some(vec![]);
        e.call(0x00e7_f610, &args![]);
        assert_eq!(e.mem.u32(block_pool + 8), 0);
        assert_eq!(e.mem.u32(entry_pool + 8), 0);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0xb100], vec![0xe100]]);
        assert_eq!(
            calls(&e, NI_ARRAY_FREE_00AA1490),
            vec![vec![0xa100, 0x10], vec![0xa200, 0x10]]
        );
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_FREE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(
            freed,
            vec![block_chain, block_pool, second, first, entry_pool]
        );
        // Without pools: nothing.
        e.mem.set_u32(BLOCK_POOL_0126F718, 0);
        e.mem.set_u32(ENTRY_POOL_0126F71C, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e7_f610, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e7f7c0_reads_the_word() {
        let mut e = engine();
        e.mem.set_u32(0x0126_f728, 0x1234);
        assert_eq!(e.call(0x00e7_f7c0, &args![0u32]).u32(), 0x1234);
    }
}

#[cfg(test)]
mod n010_tests {
    use super::*;

    /// A block whose first list is a, b, c (c last) and whose second list is d (d last).
    fn block(e: &mut Engine) -> (u32, [u32; 4]) {
        let mut entries = [0u32; 4];
        for (i, entry) in entries.iter_mut().enumerate() {
            *entry = e.mem.alloc(0x10);
            e.mem.set_u32(*entry, i as u32 + 1);
        }
        let [a, b, c, d] = entries;
        e.mem.set_u32(b + 8, a);
        e.mem.set_u32(a + 0xc, b);
        e.mem.set_u32(c + 8, b);
        e.mem.set_u32(b + 0xc, c);
        let block = e.mem.alloc(0x14);
        e.mem.set_u32(block + 4, 3);
        e.mem.set_u32(block + 8, c);
        e.mem.set_u32(block + 0xc, 1);
        e.mem.set_u32(block + 0x10, d);
        (block, entries)
    }

    #[test]
    fn fn_00e7ef80_unlinks_from_the_first_list() {
        let mut e = Engine::new();
        let (block, [a, b, c, _]) = block(&mut e);
        e.call(0x00e7_ef80, &args![block, b]);
        assert_eq!(e.mem.u32(a + 0xc), c);
        assert_eq!(e.mem.u32(c + 8), a);
        assert_eq!(e.mem.u32(block + 4), 2);
        assert_eq!(e.mem.u32(block + 8), c);
        assert_eq!((e.mem.u32(b + 8), e.mem.u32(b + 0xc)), (0, 0));
        // The last entry: the block's last moves back.
        e.call(0x00e7_ef80, &args![block, c]);
        assert_eq!(e.mem.u32(block + 8), a);
        assert_eq!(e.mem.u32(a + 0xc), 0);
        assert_eq!(e.mem.u32(block + 4), 1);
    }

    #[test]
    fn fn_00e7efc0_unlinks_from_the_second_list() {
        let mut e = Engine::new();
        let (block, [_, _, _, d]) = block(&mut e);
        e.call(0x00e7_efc0, &args![block, d]);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 4), 3);
    }

    #[test]
    fn fn_00e7f550_removes_a_found_state_and_pools_its_entry() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(ENTRY_POOL_0126F71C, 0x0e70_0000);
        let (block, [_, b, _, d]) = block(&mut e);
        // The lookup finds state 2 (b) in the first list and state 4 (d) in the second.
        e.register_double(FIND_ENTRY_00E7F000, move |e, a| match a[1] {
            2 => b.into_ret(),
            4 => {
                e.mem.set_u8(a[2], 1);
                d.into_ret()
            }
            _ => Ret::default(),
        });
        e.register(POOL_ADD_UNIQUE_00E7EC00, |e, a| {
            assert_eq!(a[0], 0x0e70_0000);
            e.mem.u32(a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00e7_f550, &args![block, 2u32]);
        assert_eq!(e.mem.u32(block + 4), 2);
        e.call(0x00e7_f550, &args![block, 4u32]);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
        e.call(0x00e7_f550, &args![block, 9u32]);
        let log = e.call_log.take().unwrap();
        let finds: Vec<_> = log.iter().filter(|c| c.0 == FIND_ENTRY_00E7F000).collect();
        assert_eq!(finds.len(), 3);
        assert_eq!(&finds[0].1[..2], &[block, 2]);
        let pooled: Vec<_> = log
            .iter()
            .filter(|c| c.0 == POOL_ADD_UNIQUE_00E7EC00)
            .collect();
        assert_eq!(pooled.len(), 2);
    }
}
