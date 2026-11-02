//! `Havok SDK/hkpendofstepcallbackutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{profile_record, PROFILE_END};

/// The memory router (`00c85750`); its word at `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl): allocator, array, capacity, element size.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `00d9ec10` (`ECX` = first array, one stack word: the second array): swaps the three words
/// (data, size, capacity) of two arrays.
const ARRAY_SWAP: u32 = 0x00d9_ec10;
/// `00d9ec40` (cdecl, four words: the data, first and last index, one more word it only passes
/// on): sorts 16-byte entries by the same two-word key the merge `00d9e8b0` uses.
const ARRAY_SORT: u32 = 0x00d9_ec40;
/// `00d9e2b0` (`ECX` = entry of the first array, two stack words: the address of a byte and an
/// entry of the second array): stores 1 in the byte when the first three words of both entries
/// are equal, 0 otherwise, and returns the byte address.
const ENTRIES_EQUAL: u32 = 0x00d9_e2b0;
/// `00cf3dd0` (`ECX` = object): the object whose byte at `+0x26` holds the callback flags.
const CALLBACK_FLAGS_OWNER: u32 = 0x00cf_3dd0;
/// Profiling label `"TtEndOfStepCbs"`.
const LABEL_TT_END_OF_STEP_CBS: u32 = 0x010d_40ec;
/// The capacity word of an array without storage: bit 31 (not owned) and a capacity of 0.
const EMPTY_CAPACITY: u32 = 0x8000_0000;
/// The capacity bits of a capacity word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The allocator behind the memory router (`00c85750`, its `+0x10`).
fn allocator_of_router(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Makes room for `wanted` entries of `element_size` bytes in the array at `array` (`00c908c0`
/// with the larger of `wanted` and twice the current capacity) when its capacity, compared as
/// signed values, is below `wanted`.
fn reserve(e: &mut Engine, allocator: u32, array: u32, wanted: i32, element_size: u32) {
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, element_size],
        );
    }
}

/// Starts the temporary array at `scratch` (data at `+0`, size at `+4`, capacity word at `+8`)
/// with room for `count` entries of `element_size` bytes. A count of 0 gives no storage;
/// otherwise the allocator (slot `0xC`) is asked for `count * element_size` bytes and may grant
/// a different number, which divided by `element_size` becomes the size word and the capacity
/// (a capacity of 0 is stored as the empty capacity word).
fn start_scratch_array(e: &mut Engine, scratch: u32, count: i32, element_size: i32) {
    let allocator = allocator_of_router(e);
    let (data, granted) = if count == 0 {
        (0, 0)
    } else {
        e.with_stack(4, |e, cell| {
            e.mem.set_i32(cell.addr(), count.wrapping_mul(element_size));
            let data = e.vcall(allocator, 0xc, &args![cell]).u32();
            (data, e.mem.i32(cell.addr()) / element_size)
        })
    };
    let capacity = if granted == 0 {
        EMPTY_CAPACITY
    } else {
        granted as u32
    };
    e.mem.set_u32(scratch, data);
    e.mem.set_i32(scratch + 4, granted);
    e.mem.set_u32(scratch + 8, capacity);
}

/// Gives the array at `array` the contents of the temporary array at `scratch` (`00d9ec10`),
/// which takes over the old storage, then frees that storage (allocator slot `0x10`: data,
/// capacity times `element_size` bytes) unless its capacity word says it is not owned. The
/// old size is dropped.
fn swap_and_free_scratch(e: &mut Engine, array: u32, scratch: u32, element_size: u32) {
    e.call(ARRAY_SWAP, &args![array, scratch]);
    let allocator = allocator_of_router(e);
    let capacity = e.mem.u32(scratch + 8);
    e.mem.set_u32(scratch + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(scratch);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
        );
    }
}

/// Copies `words` 32-bit words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The shared body of `00d9e4a0` and `00d9e680`: `entries` (data at `+0`, size at `+4`) holds
/// 12-byte entries; every entry of `items` (the same layout, entries of `element_size` bytes)
/// whose first three words equal a 12-byte entry (`00d9e2b0`) is dropped from `items` and the
/// matching 12-byte entry is removed from `entries` (the last entry takes its place). The
/// other items are collected in a temporary array that then replaces the storage of `items`.
/// Does nothing when `entries` is empty.
fn drop_matching_items(e: &mut Engine, entries: u32, items: u32, element_size: u32) {
    let entries_before = e.mem.i32(entries + 4);
    let items_before = e.mem.i32(items + 4);
    if entries_before == 0 {
        return;
    }
    e.with_stack(0x20, |e, frame| {
        let scratch = frame.addr();
        let found = frame.addr() + 0x10;
        e.mem.set_u32(scratch, 0);
        e.mem.set_u32(scratch + 4, 0);
        e.mem.set_u32(scratch + 8, EMPTY_CAPACITY);
        start_scratch_array(e, scratch, items_before, element_size as i32);
        let mut out = 0u32;
        for index in 0..items_before.max(0) as u32 {
            let item = e
                .mem
                .u32(items)
                .wrapping_add(index.wrapping_mul(element_size));
            let entry_count = e.mem.i32(entries + 4);
            let mut matched = None;
            let mut entry = e.mem.u32(entries);
            let mut position = 0;
            while position < entry_count {
                let at = e.call(ENTRIES_EQUAL, &args![entry, found, item]).u32();
                if e.mem.u8(at) != 0 {
                    matched = Some(position);
                    break;
                }
                position += 1;
                entry = entry.wrapping_add(12);
            }
            match matched {
                Some(position) => {
                    let last = entry_count - 1;
                    e.mem.set_i32(entries + 4, last);
                    if last != position {
                        let data = e.mem.u32(entries);
                        copy_words(
                            e,
                            data.wrapping_add((last as u32).wrapping_mul(12)),
                            data.wrapping_add((position as u32).wrapping_mul(12)),
                            3,
                        );
                    }
                }
                None => {
                    let to = e.mem.u32(scratch).wrapping_add(out);
                    copy_words(e, item, to, element_size / 4);
                    out = out.wrapping_add(element_size);
                }
            }
        }
        let allocator = allocator_of_router(e);
        let new_size = items_before
            .wrapping_sub(entries_before)
            .wrapping_add(e.mem.i32(entries + 4));
        reserve(e, allocator, scratch, new_size, element_size);
        e.mem.set_i32(scratch + 4, new_size);
        swap_and_free_scratch(e, items, scratch, element_size);
    });
}

// Translated from 00d9e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl, two stack words: an array of 12-byte entries and an array
/// of 16-byte entries, each laid out as data at `+0`, size at `+4`, capacity word at `+8`).
/// Removes from the second array every entry whose first three words equal an entry of the
/// first array and removes that entry from the first array (`drop_matching_items`); nothing
/// happens when the first array is empty. The second array keeps the entries that were not
/// matched and its size becomes its old size minus the old size of the first array plus the
/// new one.
pub fn fn_00d9e4a0(e: &mut Engine, entries: Ptr, items: Ptr) {
    drop_matching_items(e, entries.addr(), items.addr(), 16);
}

// Translated from 00d9e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl, two stack words: two arrays of 12-byte entries). The same
/// as `00d9e4a0`, for a second array of 12-byte entries: every entry of the second array that
/// equals an entry of the first is dropped from both.
pub fn fn_00d9e680(e: &mut Engine, entries: Ptr, items: Ptr) {
    drop_matching_items(e, entries.addr(), items.addr(), 12);
}

/// The two-word key of the entry whose first word is `object`: the words at `+0xD4` of the
/// objects at `object + 0x58` (the high word) and `object + 0x5C` (the low word).
fn entry_key(e: &Engine, object: u32) -> (u32, u32) {
    let high = e.mem.u32(e.mem.u32(object + 0x58) + 0xd4);
    let low = e.mem.u32(e.mem.u32(object + 0x5c) + 0xd4);
    (high, low)
}

// Translated from 00d9e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl, two stack words: an array of 16-byte entries and an array
/// of 12-byte entries). Merges the two arrays into a temporary array of 12-byte entries
/// (three words of each entry), taking the entry of the first array when its key (the pair
/// `entry_key`, compared high word first as unsigned values) is smaller than that of the
/// entry of the second array and the other one otherwise, then the rest of whichever array is
/// left. The second array takes over the temporary array, whose size word is the number of
/// entries the allocator granted, and the size of the first array is set to 0. Nothing happens
/// when the first array is empty (its data pointer equals the end of its entries).
pub fn fn_00d9e8b0(e: &mut Engine, first: Ptr, second: Ptr) {
    let first = first.addr();
    let second = second.addr();
    let mut from_first = e.mem.u32(first);
    let first_end = from_first.wrapping_add(e.mem.u32(first + 4).wrapping_shl(4));
    if from_first == first_end {
        return;
    }
    let total = e.mem.i32(second + 4).wrapping_add(e.mem.i32(first + 4));
    e.with_stack(0x10, |e, scratch| {
        let scratch = scratch.addr();
        e.mem.set_u32(scratch, 0);
        e.mem.set_u32(scratch + 4, 0);
        e.mem.set_u32(scratch + 8, EMPTY_CAPACITY);
        start_scratch_array(e, scratch, total, 12);
        let mut from_second = e.mem.u32(second);
        let second_end = from_second.wrapping_add((e.mem.i32(second + 4) as u32).wrapping_mul(12));
        let mut to = e.mem.u32(scratch);
        while from_first < first_end && from_second < second_end {
            let key_first = entry_key(e, e.mem.u32(from_first));
            let key_second = entry_key(e, e.mem.u32(from_second));
            if key_first < key_second {
                copy_words(e, from_first, to, 3);
                from_first = from_first.wrapping_add(0x10);
            } else {
                copy_words(e, from_second, to, 3);
                from_second = from_second.wrapping_add(12);
            }
            to = to.wrapping_add(12);
        }
        while from_first < first_end {
            copy_words(e, from_first, to, 3);
            from_first = from_first.wrapping_add(0x10);
            to = to.wrapping_add(12);
        }
        while from_second < second_end {
            copy_words(e, from_second, to, 3);
            from_second = from_second.wrapping_add(12);
            to = to.wrapping_add(12);
        }
        e.call(ARRAY_SWAP, &args![second, scratch]);
        e.mem.set_u32(first + 4, 0);
        let allocator = allocator_of_router(e);
        let capacity = e.mem.u32(scratch + 8);
        if capacity as i32 >= 0 {
            let data = e.mem.u32(scratch);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(12)],
            );
        }
    });
}

// Translated from 00d9e2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (cdecl, three stack words: an object, a callback object and a
/// word). Reads two words through the object's virtual function at `+0x40` (the words at
/// `+0x14` and `+0x18` of what it returns, on two separate calls) and then walks the entries of
/// the block at `[object + 0x3C]` (a 16-bit count at `+4`, a 16-bit entry count at `+6`, a stride
/// byte at `+0xA`, the first entry at `+0x30` with 0x20 bytes each and the second list after
/// the first entries with the stride): for each of the `count` positions, from the last index
/// down to 0, when bit 0 of the byte at `+0xF` of the entry of the second list is set, or bit 3
/// of the byte at `+0x12` of `[[virtual function at +0x40 of the object] + 0x30]` is set, it
/// builds a 0x34-byte record on the stack and calls the callback object's virtual function at
/// slot 0 with it. The record holds: the third word, the two words read first, the object, 3,
/// the current entry of the first list, the current entry of the second list, the group flag
/// (byte), whether this is the first and whether it is the last position (bytes), a padding
/// byte left unwritten (zero here), then 0, 0, the second list entry plus 0x14, 0, 0.
pub fn fn_00d9e2e0(e: &mut Engine, object: Ptr, callback: Ptr, third: u32) {
    let object = object.addr();
    let callback = callback.addr();
    let first_word = {
        let view = e.vcall(object, 0x40, &args![]).u32();
        e.mem.u32(view + 0x14)
    };
    let second_word = {
        let view = e.vcall(object, 0x40, &args![]).u32();
        e.mem.u32(view + 0x18)
    };
    let block = e.mem.u32(object + 0x3c);
    let count = u32::from(e.mem.u16(block + 4));
    let stride = u32::from(e.mem.u8(block + 0xa));
    let mut second_entry = block + 0x30 + (u32::from(e.mem.u16(block + 6)) << 5);
    let mut first_entry = block + 0x30;
    let view = e.vcall(object, 0x40, &args![]).u32();
    let flagged = e.mem.u8(e.mem.u32(view + 0x30) + 0x12) & 8 != 0;
    for index in (0..count).rev() {
        if e.mem.u8(second_entry + 0xf) & 1 != 0 || flagged {
            e.with_stack(0x34, |e, record| {
                let record = record.addr();
                e.mem.set_u32(record, third);
                e.mem.set_u32(record + 4, first_word);
                e.mem.set_u32(record + 8, second_word);
                e.mem.set_u32(record + 0xc, object);
                e.mem.set_u32(record + 0x10, 3);
                e.mem.set_u32(record + 0x14, first_entry);
                e.mem.set_u32(record + 0x18, second_entry);
                e.mem.set_u8(record + 0x1c, u8::from(flagged));
                e.mem.set_u8(record + 0x1d, u8::from(index == count - 1));
                e.mem.set_u8(record + 0x1e, u8::from(index == 0));
                e.mem.set_u32(record + 0x28, second_entry + 0x14);
                e.vcall(callback, 0, &args![record]);
            });
        }
        second_entry = second_entry.wrapping_add(stride);
        first_entry = first_entry.wrapping_add(0x20);
    }
}

// Translated from 00d9ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpEndOfStepCallbackUtil::postSimulationCallback` (Xbox PDB name per the engine map):
/// `ECX` = the utility object, one stack word that is not read. Writes a profiling record
/// (`TtEndOfStepCbs`), removes from the 16-byte entries at `+0x18` (size at `+0x1C`) and the
/// 12-byte entries at `+0xC` (size at `+0x10`) the ones that equal an entry of the 12-byte
/// entries at `+0x24` (`00d9e4a0` and `00d9e680`). When the byte at `+8` is set it sorts the
/// 16-byte entries with `00d9ec40` (first to last index; the fourth word is a byte of 0 over
/// unwritten stack that the callee only passes on, 0 here) when there are at least two, and
/// merges them into the 12-byte entries (`00d9e8b0`); otherwise it appends three words of each
/// 16-byte entry to the 12-byte entries (making room with `00c908c0` first) and clears the
/// size of the 16-byte entries. Then, for each of the 12-byte entries, when bit 2 or bit 3 of
/// the byte at `+0x26` of the object `00cf3dd0` gives for the entry's first word (an object;
/// the argument is what its virtual function at `+0x40` returns) is set, it calls `00d9e2e0`
/// with the object and the entry's other two words. Clears the word at `+4` and writes the
/// closing profiling record.
pub fn hkp_end_of_step_callback_util_post_simulation_callback(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) {
    profile_record(e, LABEL_TT_END_OF_STEP_CBS);
    let me = this.addr();
    let sixteen = me + 0x18;
    let twelve = me + 0xc;
    let removals = me + 0x24;
    fn_00d9e4a0(e, Ptr::new(removals), Ptr::new(sixteen));
    fn_00d9e680(e, Ptr::new(removals), Ptr::new(twelve));
    if e.mem.u8(me + 8) != 0 {
        let count = e.mem.i32(me + 0x1c);
        if count > 1 {
            let data = e.mem.u32(sixteen);
            e.call(ARRAY_SORT, &args![data, 0u32, count.wrapping_sub(1), 0u32]);
        }
        fn_00d9e8b0(e, Ptr::new(sixteen), Ptr::new(twelve));
    } else {
        let count = e.mem.i32(me + 0x1c);
        let allocator = allocator_of_router(e);
        let before = e.mem.i32(me + 0x10);
        reserve(e, allocator, twelve, before.wrapping_add(count), 12);
        e.mem
            .set_i32(me + 0x10, e.mem.i32(me + 0x10).wrapping_add(count));
        let mut to = e
            .mem
            .u32(twelve)
            .wrapping_add((before as u32).wrapping_mul(12));
        for index in 0..count.max(0) as u32 {
            let from = e.mem.u32(sixteen).wrapping_add(index.wrapping_mul(0x10));
            copy_words(e, from, to, 3);
            to = to.wrapping_add(12);
        }
        e.mem.set_u32(me + 0x1c, 0);
    }
    let count = e.mem.i32(me + 0x10);
    for index in 0..count.max(0) as u32 {
        let entry = e.mem.u32(twelve).wrapping_add(index.wrapping_mul(12));
        let object = e.mem.u32(entry);
        let view = e.vcall(object, 0x40, &args![]).u32();
        let owner = e.call(CALLBACK_FLAGS_OWNER, &args![view]).u32();
        if e.mem.u8(owner + 0x26) & 0xc != 0 {
            let callback = e.mem.u32(entry + 4);
            let third = e.mem.u32(entry + 8);
            fn_00d9e2e0(e, Ptr::new(object), Ptr::new(callback), third);
        }
    }
    e.mem.set_u32(me + 4, 0);
    profile_record(e, PROFILE_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d9e2e0, fn_00d9e2e0(Ptr, Ptr, u32)),
        entry!(0x00d9e4a0, fn_00d9e4a0(Ptr, Ptr)),
        entry!(0x00d9e680, fn_00d9e680(Ptr, Ptr)),
        entry!(0x00d9e8b0, fn_00d9e8b0(Ptr, Ptr)),
        entry!(
            0x00d9ea80,
            hkp_end_of_step_callback_util_post_simulation_callback(Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };

    const ALLOCATOR: u32 = 0x0900_0000;
    const VIEW_CALL: u32 = 0x0900_1000;
    const CALLBACK: u32 = 0x0900_2000;

    struct Rig {
        log: Log,
        stream: u32,
        records: u32,
        allocator: u32,
        vtable: u32,
    }

    /// An engine with the memory router, an allocator that grants its request shifted right by
    /// `shift` bits (the block always has the requested size), logging doubles for the swap, the
    /// sort and the equality test (which compares three words), and a record stream. Objects made
    /// by `object` have a virtual function at `+0x40` (logged) that returns the object itself,
    /// and `00cf3dd0` (logged) answers the word at `+0x70` of its argument.
    fn rig(shift: u32) -> (Engine, Rig) {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ALLOCATOR + 0xc, move |e, a| {
            let cell = e.mem.u32(a[1]);
            let block = e.mem.alloc(cell.max(4));
            e.mem.set_u32(a[1], cell >> shift);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(ARRAY_SWAP, move |e, a| {
            sink.borrow_mut().push((ARRAY_SWAP, a.to_vec()));
            for word in 0..3 {
                let first = e.mem.u32(a[0] + 4 * word);
                let second = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, second);
                e.mem.set_u32(a[1] + 4 * word, first);
            }
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(ENTRIES_EQUAL, move |e, a| {
            sink.borrow_mut().push((ENTRIES_EQUAL, a.to_vec()));
            let same = (0..3).all(|word| e.mem.u32(a[0] + 4 * word) == e.mem.u32(a[2] + 4 * word));
            e.mem.set_u8(a[1], u8::from(same));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(VIEW_CALL, move |_, a| {
            sink.borrow_mut().push((VIEW_CALL, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(CALLBACK_FLAGS_OWNER, move |e, a| {
            sink.borrow_mut().push((CALLBACK_FLAGS_OWNER, a.to_vec()));
            Ret {
                eax: e.mem.u32(a[0] + 0x70),
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x40, VIEW_CALL);
        (
            e,
            Rig {
                log,
                stream,
                records,
                allocator,
                vtable,
            },
        )
    }

    /// Lays out an array of entries of `element_words` words at `at`: data, size and capacity
    /// both the entry count.
    fn array_at(e: &mut Engine, at: u32, words: &[u32], element_words: u32) -> u32 {
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *word);
        }
        let count = words.len() as u32 / element_words;
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, count);
        e.mem.set_u32(at + 8, count);
        at
    }

    fn array(e: &mut Engine, words: &[u32], element_words: u32) -> u32 {
        let at = e.mem.alloc(0xc);
        array_at(e, at, words, element_words)
    }

    /// The words of the entries of an array.
    fn words(e: &Engine, array: u32, element_words: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        let count = e.mem.u32(array + 4) * element_words;
        (0..count).map(|word| e.mem.u32(data + 4 * word)).collect()
    }

    /// An object whose virtual table has the logged function at `+0x40`, with the words `hi` and
    /// `lo` behind `+0x58` and `+0x5C` (at `+0xD4`), the callback flag byte `flags` behind
    /// `+0x70` (at `+0x26`), a block of no entries at `+0x3C` and an object whose flag byte is 0
    /// behind `+0x30`.
    fn object(e: &mut Engine, rig: &Rig, hi: u32, lo: u32, flags: u8) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, rig.vtable);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x3c, block);
        for (at, value) in [(0x58, hi), (0x5c, lo)] {
            let part = e.mem.alloc(0xe0);
            e.mem.set_u32(part + 0xd4, value);
            e.mem.set_u32(object + at, part);
        }
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(owner + 0x26, flags);
        e.mem.set_u32(object + 0x70, owner);
        let group = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x30, group);
        e.mem.set_u32(object + 0x14, 0x111);
        e.mem.set_u32(object + 0x18, 0x222);
        object
    }

    #[test]
    fn fn_00d9e4a0_drops_the_matching_16_byte_entries() {
        let (mut e, rig) = rig(0);
        log_double(&mut e, &rig.log, ARRAY_RESERVE, 0);
        let entries = array(&mut e, &[1, 1, 1, 2, 2, 2, 3, 3, 3], 3);
        let items = array(
            &mut e,
            &[2, 2, 2, 90, 7, 7, 7, 80, 3, 3, 3, 60, 9, 9, 9, 50],
            4,
        );
        let old_data = e.mem.u32(items);
        e.call(0x00d9_e4a0, &args![entries, items]);
        assert_eq!(words(&e, entries, 3), vec![1, 1, 1]);
        assert_eq!(words(&e, items, 4), vec![7, 7, 7, 80, 9, 9, 9, 50]);
        // The size is 4 - 3 + 1; the capacity is what the allocator granted.
        assert_eq!(e.mem.u32(items + 8), 4);
        assert_eq!(calls(&rig.log, ENTRIES_EQUAL).len(), 7);
        assert!(calls(&rig.log, ARRAY_RESERVE).is_empty());
        let swaps = calls(&rig.log, ARRAY_SWAP);
        assert_eq!(swaps.len(), 1);
        assert_eq!(swaps[0][0], items);
        // The storage the items had is freed: 4 entries of 16 bytes.
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![rig.allocator, old_data, 64]]
        );
    }

    #[test]
    fn fn_00d9e4a0_reserves_when_the_allocator_grants_too_little() {
        let (mut e, rig) = rig(1);
        log_double(&mut e, &rig.log, ARRAY_RESERVE, 0);
        let entries = array(&mut e, &[1, 1, 1], 3);
        let items = array(&mut e, &[2, 2, 2, 1, 3, 3, 3, 2, 4, 4, 4, 3, 5, 5, 5, 4], 4);
        let old_data = e.mem.u32(items);
        e.call(0x00d9_e4a0, &args![entries, items]);
        // Nothing matched; the capacity is 2 of the 4 wanted, so twice it is wanted.
        let reserved = calls(&rig.log, ARRAY_RESERVE);
        assert_eq!(reserved.len(), 1);
        assert_eq!(
            [reserved[0][0], reserved[0][2], reserved[0][3]],
            [rig.allocator, 4, 16]
        );
        assert_eq!(e.mem.u32(items + 4), 4);
        assert_eq!(words(&e, entries, 3), vec![1, 1, 1]);
        assert_eq!(e.mem.u32(items + 8), 2);
        // The old storage, 4 entries of 16 bytes, is freed.
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![rig.allocator, old_data, 64]]
        );
    }

    #[test]
    fn fn_00d9e4a0_does_nothing_without_entries() {
        let (mut e, rig) = rig(0);
        let entries = array(&mut e, &[], 3);
        let items = array(&mut e, &[2, 2, 2, 90], 4);
        e.call(0x00d9_e4a0, &args![entries, items]);
        assert_eq!(words(&e, items, 4), vec![2, 2, 2, 90]);
        assert!(calls(&rig.log, ENTRIES_EQUAL).is_empty());
        assert!(calls(&rig.log, ARRAY_SWAP).is_empty());
        assert!(calls(&rig.log, ALLOCATOR + 0x10).is_empty());
    }

    #[test]
    fn fn_00d9e680_drops_the_matching_12_byte_entries() {
        let (mut e, rig) = rig(0);
        let entries = array(&mut e, &[1, 1, 1, 2, 2, 2], 3);
        let items = array(&mut e, &[2, 2, 2, 5, 5, 5], 3);
        let old_data = e.mem.u32(items);
        e.call(0x00d9_e680, &args![entries, items]);
        assert_eq!(words(&e, entries, 3), vec![1, 1, 1]);
        assert_eq!(words(&e, items, 3), vec![5, 5, 5]);
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![rig.allocator, old_data, 24]]
        );
    }

    #[test]
    fn fn_00d9e8b0_merges_by_key() {
        let (mut e, rig) = rig(0);
        let a1 = object(&mut e, &rig, 1, 5, 0);
        let a2 = object(&mut e, &rig, 3, 0, 0);
        let b1 = object(&mut e, &rig, 1, 2, 0);
        let b2 = object(&mut e, &rig, 1, 5, 0);
        let b3 = object(&mut e, &rig, 4, 0, 0);
        let b4 = object(&mut e, &rig, 0, 9, 0);
        let first = array(&mut e, &[a1, 11, 12, 13, a2, 21, 22, 23], 4);
        let second = array(&mut e, &[b4, 41, 42, b1, 11, 12, b2, 21, 22, b3, 31, 32], 3);
        let old_data = e.mem.u32(second);
        e.call(0x00d9_e8b0, &args![first, second]);
        // A key equal to the other array's goes after it; the fourth word is not copied.
        assert_eq!(
            words(&e, second, 3),
            vec![b4, 41, 42, b1, 11, 12, b2, 21, 22, a1, 11, 12, a2, 21, 22, b3, 31, 32]
        );
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![rig.allocator, old_data, 48]]
        );
    }

    #[test]
    fn fn_00d9e8b0_copies_the_first_array_when_the_second_is_empty() {
        let (mut e, rig) = rig(0);
        let a1 = object(&mut e, &rig, 1, 5, 0);
        let a2 = object(&mut e, &rig, 3, 0, 0);
        let first = array(&mut e, &[a1, 11, 12, 13, a2, 21, 22, 23], 4);
        let second = array(&mut e, &[], 3);
        let old_data = e.mem.u32(second);
        e.call(0x00d9_e8b0, &args![first, second]);
        assert_eq!(words(&e, second, 3), vec![a1, 11, 12, a2, 21, 22]);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(
            calls(&rig.log, ALLOCATOR + 0x10),
            vec![vec![rig.allocator, old_data, 0]]
        );
    }

    #[test]
    fn fn_00d9e8b0_does_nothing_without_entries_in_the_first_array() {
        let (mut e, rig) = rig(0);
        let first = array(&mut e, &[], 4);
        let second = array(&mut e, &[5, 6, 7], 3);
        e.call(0x00d9_e8b0, &args![first, second]);
        assert_eq!(words(&e, second, 3), vec![5, 6, 7]);
        assert!(calls(&rig.log, ARRAY_SWAP).is_empty());
    }

    /// A walker object (virtual function `+0x40` returns the object, whose `+0x14` and
    /// `+0x18` are 0x111 and 0x222) with a block of `entry_flags.len()` positions, a stride of
    /// 0x10 and two list entries of 0x20 bytes before the second list; the object behind
    /// `+0x30` has the byte `group_flag` at `+0x12`. Returns the object, the block and a
    /// callback object whose slot 0 is a double that logs its record's 13 words after the
    /// receiver and the record address.
    fn walker(e: &mut Engine, rig: &Rig, entry_flags: &[u8], group_flag: u8) -> (u32, u32, u32) {
        let object = object(e, rig, 0, 0, 0);
        let block = e.mem.alloc(0x100);
        e.mem.set_u16(block + 4, entry_flags.len() as u16);
        e.mem.set_u16(block + 6, 2);
        e.mem.set_u8(block + 0xa, 0x10);
        for (index, flags) in entry_flags.iter().enumerate() {
            e.mem
                .set_u8(block + 0x70 + 0x10 * index as u32 + 0xf, *flags);
        }
        e.mem.set_u32(object + 0x3c, block);
        let group = e.mem.u32(object + 0x30);
        e.mem.set_u8(group + 0x12, group_flag);
        let callback = e.mem.alloc(0x10);
        let callback_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(callback, callback_vtable);
        e.mem.set_u32(callback_vtable, CALLBACK);
        let sink = rig.log.clone();
        e.register_double(CALLBACK, move |e, a| {
            let mut logged = a.to_vec();
            logged.extend((0..13).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((CALLBACK, logged));
            Ret::default()
        });
        (object, block, callback)
    }

    #[test]
    fn fn_00d9e2e0_calls_back_for_the_flagged_entries() {
        let (mut e, rig) = rig(0);
        let (object, block, callback) = walker(&mut e, &rig, &[1, 0, 1], 0);
        e.call(0x00d9_e2e0, &args![object, callback, 0x77u32]);
        // Positions are visited from the last index down; the byte at +0x1c is the group
        // flag, the one at +0x1d marks the first position visited and +0x1e the last.
        let logged = calls(&rig.log, CALLBACK);
        assert_eq!(logged.len(), 2);
        assert_eq!(logged[0][0], callback);
        assert_eq!(
            logged[0][2..],
            [
                0x77,
                0x111,
                0x222,
                object,
                3,
                block + 0x30,
                block + 0x70,
                0x100,
                0,
                0,
                block + 0x70 + 0x14,
                0,
                0
            ]
        );
        assert_eq!(
            logged[1][2..],
            [
                0x77,
                0x111,
                0x222,
                object,
                3,
                block + 0x70,
                block + 0x90,
                0x1_0000,
                0,
                0,
                block + 0x90 + 0x14,
                0,
                0
            ]
        );
        assert_eq!(calls(&rig.log, VIEW_CALL).len(), 3);
    }

    #[test]
    fn fn_00d9e2e0_calls_back_for_every_entry_with_the_group_flag() {
        let (mut e, rig) = rig(0);
        let (object, block, callback) = walker(&mut e, &rig, &[0, 0], 8);
        e.call(0x00d9_e2e0, &args![object, callback, 0x77u32]);
        let logged = calls(&rig.log, CALLBACK);
        assert_eq!(logged.len(), 2);
        assert_eq!(logged[0][2 + 7], 0x101);
        assert_eq!(logged[1][2 + 7], 0x1_0001);
        assert_eq!(logged[0][2 + 6], block + 0x70);
        assert_eq!(logged[1][2 + 6], block + 0x80);
    }

    #[test]
    fn post_simulation_callback_appends_and_calls_back() {
        let (mut e, rig) = rig(0);
        let callback = e.mem.alloc(0x10);
        let callback_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(callback, callback_vtable);
        e.mem.set_u32(callback_vtable, CALLBACK);
        let sink = rig.log.clone();
        e.register_double(CALLBACK, move |e, a| {
            sink.borrow_mut()
                .push((CALLBACK, vec![a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 0xc)]));
            Ret::default()
        });
        let sink = rig.log.clone();
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let used = e.mem.u32(a[1] + 4) * a[3];
            let block = e.mem.alloc(a[2] * a[3]);
            let old = e.mem.u32(a[1]);
            for word in 0..used / 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(block + 4 * word, value);
            }
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let first = object(&mut e, &rig, 1, 1, 4);
        let second = object(&mut e, &rig, 2, 2, 0);
        let third = object(&mut e, &rig, 3, 3, 8);
        // The first object has one entry that is flagged, so its callbacks run.
        let block = e.mem.alloc(0x100);
        e.mem.set_u16(block + 4, 1);
        e.mem.set_u16(block + 6, 0);
        e.mem.set_u8(block + 0xa, 0x10);
        e.mem.set_u8(block + 0x30 + 0xf, 1);
        e.mem.set_u32(first + 0x3c, block);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u8(this + 8, 0);
        array_at(&mut e, this + 0xc, &[first, callback, 0x32], 3);
        array_at(
            &mut e,
            this + 0x18,
            &[second, 0x41, 0x42, 0xdead, third, 0x51, 0x52, 0xbeef],
            4,
        );
        array_at(&mut e, this + 0x24, &[second, 0x41, 0x42], 3);
        e.call(0x00d9_ea80, &args![this, 0u32]);
        // The entry equal to the one in the removal list is gone before the rest is appended.
        assert_eq!(
            words(&e, this + 0xc, 3),
            vec![first, callback, 0x32, third, 0x51, 0x52]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        let reserved = calls(&rig.log, ARRAY_RESERVE);
        assert_eq!(reserved, vec![vec![rig.allocator, this + 0xc, 2, 12]]);
        // The virtual function runs once per entry and three more times in the walk of an
        // object with callbacks; the callback runs for the one flagged entry.
        let views: Vec<u32> = calls(&rig.log, VIEW_CALL).iter().map(|c| c[0]).collect();
        assert_eq!(
            views,
            vec![first, first, first, first, third, third, third, third]
        );
        assert_eq!(calls(&rig.log, CALLBACK), vec![vec![callback, 0x32, first]]);
        assert_eq!(
            profile_labels(&e, rig.stream, rig.records),
            vec![LABEL_TT_END_OF_STEP_CBS, PROFILE_END]
        );
    }

    #[test]
    fn post_simulation_callback_sorts_and_merges_when_asked() {
        let (mut e, rig) = rig(0);
        log_double(&mut e, &rig.log, ARRAY_SORT, 0);
        let late = object(&mut e, &rig, 3, 0, 0);
        let early = object(&mut e, &rig, 1, 5, 0);
        let middle = object(&mut e, &rig, 2, 2, 0);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 4, 5);
        e.mem.set_u8(this + 8, 1);
        array_at(&mut e, this + 0xc, &[middle, 0x31, 0x32], 3);
        array_at(
            &mut e,
            this + 0x18,
            &[late, 0x41, 0x42, 0xdead, early, 0x51, 0x52, 0xbeef],
            4,
        );
        array_at(&mut e, this + 0x24, &[], 3);
        let sixteen = e.mem.u32(this + 0x18);
        e.call(0x00d9_ea80, &args![this, 0u32]);
        assert_eq!(calls(&rig.log, ARRAY_SORT), vec![vec![sixteen, 0, 1, 0]]);
        // The sort is a double, so the merge takes the entries in their stored order.
        assert_eq!(
            words(&e, this + 0xc, 3),
            vec![middle, 0x31, 0x32, late, 0x41, 0x42, early, 0x51, 0x52]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        let views: Vec<u32> = calls(&rig.log, VIEW_CALL).iter().map(|c| c[0]).collect();
        assert_eq!(views, vec![middle, late, early]);
        assert!(calls(&rig.log, CALLBACK).is_empty());
        assert_eq!(
            profile_labels(&e, rig.stream, rig.records),
            vec![LABEL_TT_END_OF_STEP_CBS, PROFILE_END]
        );
    }
}
