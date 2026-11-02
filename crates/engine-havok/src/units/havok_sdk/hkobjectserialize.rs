//! `Havok SDK/hkobjectserialize.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The stream the functions read or write is an object with a vtable: slot `0xc` is `isOk`
//! (given the address of a byte, answers the address of the byte it wrote) and slot `0x10` the
//! read or write of `(buffer, size)`. An `hkArray` is three words: the data, the size and the
//! capacity word (the top bit `0x80000000` marks memory that is not owned, the rest
//! `0x3fffffff` is the capacity).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator (slot `0x10` frees
/// `(pointer, bytes)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyAlloc` (`00c85690`, cdecl `(allocator, size)`) and `easyFree`
/// (`00c856c0`, cdecl `(allocator, block)`).
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// the function at `00c90940` (cdecl `(allocator, array, element size)`) that makes room for one
/// more element.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The `hkOArchive` the data is written through (`this` is a 0x10-byte local): constructor
/// (`00d60830`, `(stream, byte flag)`), `write32` (`00d60940`, `(value)`), `write32u`
/// (`00d60960`, `(value)`), `write64u` (`00d609a0`, `(low word, high word)`) and the destructor
/// (`00d60590`).
const ARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
const ARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE_32U: u32 = 0x00d6_0960;
const ARCHIVE_WRITE_64U: u32 = 0x00d6_09a0;
const ARCHIVE_DESTRUCT: u32 = 0x00d6_0590;
/// The platform object writer: constructor (`00e02bb0`, `this` = a 0x10-byte object, `(layout
/// rules)`), `writeObject` (`00e02a80`, `(stream, object, class, relocation info)`) and
/// destructor (`00e02c70`); the relocation info destructor (`00e028e0`); the array stream writer
/// constructor (`00cc06f0`, a 0x14-byte object, `(array, memory type)`, the engine map calls it
/// `nullTerminate`) and its destructor (`00cc0780`).
const PLATFORM_WRITER_CONSTRUCT: u32 = 0x00e0_2bb0;
const PLATFORM_WRITE_OBJECT: u32 = 0x00e0_2a80;
const PLATFORM_WRITER_DESTRUCT: u32 = 0x00e0_2c70;
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
const STREAM_WRITER_CONSTRUCT: u32 = 0x00cc_06f0;
const STREAM_WRITER_DESTRUCT: u32 = 0x00cc_0780;
/// The byte holding the host's endianness flag.
const HOST_ENDIAN_FLAG: u32 = 0x011b_7b89;
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// Whether the stream says it is ok: slot `0xc` is given the address of a byte and answers the
/// address of the byte it wrote.
fn stream_is_ok(e: &mut Engine, stream: u32, cell: u32) -> bool {
    let answer = e.vcall(stream, 0xc, &args![cell]).u32();
    e.mem.u8(answer) != 0
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The inlined "make room for `wanted`": when the array's capacity (signed) is below `wanted`,
/// `_reserve` (`00c908c0`) grows it to the larger of `wanted` and twice the capacity, element
/// size `element`.
fn reserve_at_least(e: &mut Engine, allocator: u32, array: u32, wanted: u32, element: u32) {
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if (capacity as i32) < wanted as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let target = if (wanted as i32) < doubled as i32 {
            doubled
        } else {
            wanted
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, target, element]);
    }
}

// Translated from 00e0b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_writeData` (engine map name; a static function). Takes its inputs in registers:
/// `stream` in `ESI`, `tables` in `EBX`; then the seven stack words `first_low`, `first_high`,
/// `second_low`, `second_high`, `data`, `size`, `swap_flag` (cdecl). Writes through an
/// `hkOArchive` over `stream` (constructed with `swap_flag` as its byte flag): the 64-bit values
/// `first` and `second`; the count of the first table (`tables + 4`, data at `+0`) with its
/// entries as two `write32`s; the count of the second table (`+0x10`, data at `+0xc`, 16-byte
/// entries) with each entry as a `write32` and two `write64u`s of its words 1 and 2 (high word
/// 0); `size`, and the `size` bytes at `data` through stream slot `0x10`. After each group
/// the stream's `isOk` (slot `0xc`) is asked; when it says no the archive is destroyed and the
/// result is -1. Otherwise the result is the 20 bytes of the header and the counts plus 8 for
/// every entry of the first table, 0x14 for every entry of the second, plus `size` and 4.
pub fn write_data(
    e: &mut Engine,
    stream: u32,
    tables: Ptr,
    first_low: u32,
    first_high: u32,
    second_low: u32,
    second_high: u32,
    data: u32,
    size: u32,
    swap_flag: u32,
) -> i32 {
    let tables = tables.addr();
    e.with_stack(0x14, |e, locals| {
        let archive = locals.addr();
        let cell = locals.addr() + 0x10;
        e.call(ARCHIVE_CONSTRUCT, &args![archive, stream, swap_flag]);
        e.call(ARCHIVE_WRITE_64U, &args![archive, first_low, first_high]);
        let fail = |e: &mut Engine| -> i32 {
            e.call(ARCHIVE_DESTRUCT, &args![archive]);
            -1
        };
        if !stream_is_ok(e, stream, cell) {
            return fail(e);
        }
        e.call(ARCHIVE_WRITE_64U, &args![archive, second_low, second_high]);
        if !stream_is_ok(e, stream, cell) {
            return fail(e);
        }
        let first_count = e.mem.u32(tables + 4);
        e.call(ARCHIVE_WRITE_32U, &args![archive, first_count]);
        let mut written = 0x14u32;
        for index in 0..first_count {
            let entries = e.mem.u32(tables);
            let word = e.mem.u32(entries.wrapping_add(index.wrapping_mul(8)));
            e.call(ARCHIVE_WRITE_32, &args![archive, word]);
            let entries = e.mem.u32(tables);
            let word = e.mem.u32(entries.wrapping_add(index.wrapping_mul(8)) + 4);
            e.call(ARCHIVE_WRITE_32, &args![archive, word]);
            if !stream_is_ok(e, stream, cell) {
                return fail(e);
            }
            written = written.wrapping_add(8);
        }
        let second_count = e.mem.u32(tables + 0x10);
        e.call(ARCHIVE_WRITE_32U, &args![archive, second_count]);
        written = written.wrapping_add(4);
        for index in 0..second_count {
            let entries = e.mem.u32(tables + 0xc);
            let at = entries.wrapping_add(index.wrapping_mul(0x10));
            let word = e.mem.u32(at);
            e.call(ARCHIVE_WRITE_32, &args![archive, word]);
            let entries = e.mem.u32(tables + 0xc);
            let at = entries.wrapping_add(index.wrapping_mul(0x10));
            let word = e.mem.u32(at + 4);
            e.call(ARCHIVE_WRITE_64U, &args![archive, word, 0u32]);
            let entries = e.mem.u32(tables + 0xc);
            let at = entries.wrapping_add(index.wrapping_mul(0x10));
            let word = e.mem.u32(at + 8);
            e.call(ARCHIVE_WRITE_64U, &args![archive, word, 0u32]);
            if !stream_is_ok(e, stream, cell) {
                return fail(e);
            }
            written = written.wrapping_add(0x14);
        }
        e.call(ARCHIVE_WRITE_32U, &args![archive, size]);
        if !stream_is_ok(e, stream, cell) {
            return fail(e);
        }
        e.vcall(stream, 0x10, &args![data, size]);
        if !stream_is_ok(e, stream, cell) {
            return fail(e);
        }
        e.call(ARCHIVE_DESTRUCT, &args![archive]);
        written.wrapping_add(size).wrapping_add(4) as i32
    })
}

// Translated from 00e0ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectSerialize::readObject` (Xbox PDB), cdecl `(stream, block_out, size_out, relocations,
/// second_out)`: reads from `stream` (slot `0x10`, `(buffer, bytes)`; after each group slot
/// `0xc` `isOk` is asked and a failure ends with 0 after the cleanup below): a first 8-byte
/// value (kept for the result) and 8 bytes into `second_out`; a count of fixups and that many
/// `(offset, target)` pairs of words into a local array (`_reserve`, element size 8); a count of
/// relocation entries into the `hkArray` `relocations` (element size `0x18`; each entry has its
/// word at `+0`, 8 bytes at `+8` and 8 bytes at `+0x10`); the size into `size_out`; and that many
/// bytes into a block from `hkMemoryRouter::easyAlloc` (the memory router's allocator,
/// `00c85750`). When the block cannot be read it is freed (`easyFree`). Then each fixup whose
/// offset is not negative stores `block + target` at `block + offset`, `block` goes to
/// `block_out` and the first value is returned. The local array is emptied at every end (its
/// size set to 0 and its data freed with `capacity * 8` bytes unless its capacity word says it
/// is not owned).
pub fn read_object(
    e: &mut Engine,
    stream: Ptr,
    block_out: Ptr,
    size_out: Ptr,
    relocations: Ptr,
    second_out: Ptr,
) -> u64 {
    let stream = stream.addr();
    e.with_stack(0x30, |e, frame| {
        let value = frame.addr();
        let count_cell = frame.addr() + 8;
        let relocation_count_cell = frame.addr() + 0xc;
        let cell = frame.addr() + 0x10;
        let array = frame.addr() + 0x14;
        e.vcall(stream, 0x10, &args![value, 8u32]);
        if !stream_is_ok(e, stream, cell) {
            return 0;
        }
        e.vcall(stream, 0x10, &args![second_out, 8u32]);
        if !stream_is_ok(e, stream, cell) {
            return 0;
        }
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, NOT_OWNED);
        e.vcall(stream, 0x10, &args![count_cell, 4u32]);
        // Empties the local array: the size to 0, the data freed unless it is not owned.
        let empty_array = |e: &mut Engine| {
            let allocator = router_allocator(e);
            let capacity = e.mem.u32(array + 8);
            e.mem.set_u32(array + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(array);
                let bytes = (capacity & CAPACITY_MASK).wrapping_mul(8);
                e.vcall(allocator, 0x10, &args![data, bytes]);
            }
        };
        if !stream_is_ok(e, stream, cell) {
            empty_array(e);
            return 0;
        }
        let fixups = e.mem.u32(count_cell);
        let allocator = router_allocator(e);
        reserve_at_least(e, allocator, array, fixups, 8);
        e.mem.set_u32(array + 4, fixups);
        for index in 0..fixups {
            let entries = e.mem.u32(array);
            let entry = entries.wrapping_add(index.wrapping_mul(8));
            e.vcall(stream, 0x10, &args![entry, 4u32]);
            e.vcall(stream, 0x10, &args![entry + 4, 4u32]);
            if !stream_is_ok(e, stream, cell) {
                empty_array(e);
                return 0;
            }
        }
        e.vcall(stream, 0x10, &args![relocation_count_cell, 4u32]);
        if !stream_is_ok(e, stream, cell) {
            empty_array(e);
            return 0;
        }
        let relocation_count = e.mem.u32(relocation_count_cell);
        let allocator = router_allocator(e);
        reserve_at_least(e, allocator, relocations.addr(), relocation_count, 0x18);
        e.mem.set_u32(relocations.addr() + 4, relocation_count);
        for index in 0..relocation_count {
            let entries = e.mem.u32(relocations.addr());
            let entry = entries.wrapping_add(index.wrapping_mul(0x18));
            e.vcall(stream, 0x10, &args![entry, 4u32]);
            e.vcall(stream, 0x10, &args![entry + 8, 8u32]);
            e.vcall(stream, 0x10, &args![entry + 0x10, 8u32]);
            if !stream_is_ok(e, stream, cell) {
                empty_array(e);
                return 0;
            }
        }
        e.vcall(stream, 0x10, &args![size_out, 4u32]);
        if !stream_is_ok(e, stream, cell) {
            empty_array(e);
            return 0;
        }
        let size = e.mem.u32(size_out.addr());
        let allocator = router_allocator(e);
        let block = e.call(EASY_ALLOC, &args![allocator, size]).u32();
        let size = e.mem.u32(size_out.addr());
        e.vcall(stream, 0x10, &args![block, size]);
        if !stream_is_ok(e, stream, cell) {
            let allocator = router_allocator(e);
            e.call(EASY_FREE, &args![allocator, block]);
            empty_array(e);
            return 0;
        }
        for index in 0..fixups {
            let entries = e.mem.u32(array);
            let at = entries.wrapping_add(index.wrapping_mul(8));
            let offset = e.mem.u32(at);
            if (offset as i32) >= 0 {
                let target = e.mem.u32(at + 4).wrapping_add(block);
                e.mem.set_u32(offset.wrapping_add(block), target);
            }
        }
        e.mem.set_u32(block_out.addr(), block);
        let low = e.mem.u32(value);
        let high = e.mem.u32(value + 4);
        empty_array(e);
        u64::from(low) | u64::from(high) << 32
    })
}

// Translated from 00e0bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkObjectSerialize::writeObject` (Xbox PDB), cdecl `(stream, rules, first_low, first_high,
/// object, class, relocations)`: builds a platform object writer from `rules`
/// (`00e02bb0`) and an array stream writer over a local `hkArray` of bytes (`00cc06f0`), and
/// has the platform writer write `object` of `class` into it (`00e02a80`, with a local
/// relocation record of four empty arrays and a word). The record's second array (16-byte
/// entries `(a, b, c, _)`) is appended to the `hkArray` `relocations` as 0x18-byte entries with
/// words `a` at `+0`, `b` at `+8`, `c` at `+0x10` and 0 at `+0xc`, `+0x14` (the words at
/// `+4` are not written); room is made first (`_reserve`, element size `0x18`, the larger of
/// the needed count and twice the capacity) and again when an append finds the array full
/// (`00c90940`). Then `_writeData` (`00e0b850`, `stream` in `ESI` and the record in `EBX`) writes
/// `(first_low, first_high)`, `(class, 0)`, the local array's bytes and, as the flag, whether the
/// byte at `rules + 1` differs from the host endianness byte (`011b7b89`) (the word the exe
/// passes also has leftover upper bytes; the archive constructor reads only the low byte). The
/// record, the stream writer and the local array (its data freed through the memory router's
/// allocator, `00c85750`) and the platform writer are destroyed and `_writeData`'s result is
/// returned.
pub fn write_object(
    e: &mut Engine,
    stream: u32,
    rules: u32,
    first_low: u32,
    first_high: u32,
    object: u32,
    class: u32,
    relocations: Ptr,
) -> i32 {
    e.with_stack(0x64, |e, frame| {
        let base = frame.addr();
        // The exe's frame: the local array at +0, the platform writer at +0xc, the array stream
        // writer at +0x1c and the relocation record (0x34 bytes) at +0x30.
        let (bytes, writer, stream_writer, record) = (base, base + 0xc, base + 0x1c, base + 0x30);
        e.call(PLATFORM_WRITER_CONSTRUCT, &args![writer, rules]);
        e.mem.set_u32(bytes, 0);
        e.mem.set_u32(bytes + 4, 0);
        e.mem.set_u32(bytes + 8, NOT_OWNED);
        e.call(STREAM_WRITER_CONSTRUCT, &args![stream_writer, bytes, 1u32]);
        for array in 0..4 {
            e.mem.set_u32(record + 12 * array, 0);
            e.mem.set_u32(record + 12 * array + 4, 0);
            e.mem.set_u32(record + 12 * array + 8, NOT_OWNED);
        }
        e.mem.set_u32(record + 0x30, 0);
        e.call(
            PLATFORM_WRITE_OBJECT,
            &args![writer, stream_writer, object, class, record],
        );
        let relocations = relocations.addr();
        let appended = e.mem.u32(record + 0x10);
        let needed = e.mem.u32(relocations + 4).wrapping_add(appended);
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(relocations + 8) & CAPACITY_MASK;
        if (capacity as i32) < needed as i32 {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if (needed as i32) >= doubled as i32 {
                needed
            } else {
                doubled
            };
            e.call(
                ARRAY_RESERVE,
                &args![allocator, relocations, wanted, 0x18u32],
            );
        }
        if (e.mem.u32(record + 0x10) as i32) > 0 {
            let mut index = 0i32;
            let mut offset = 0u32;
            loop {
                let allocator = router_allocator(e);
                let capacity = e.mem.u32(relocations + 8) & CAPACITY_MASK;
                if e.mem.u32(relocations + 4) == capacity {
                    e.call(ARRAY_RESERVE_MORE, &args![allocator, relocations, 0x18u32]);
                }
                let size = e.mem.u32(relocations + 4);
                let data = e.mem.u32(relocations);
                let entry = data.wrapping_add(size.wrapping_mul(0x18));
                e.mem.set_u32(relocations + 4, size.wrapping_add(1));
                let source = e.mem.u32(record + 0xc);
                let word = e.mem.u32(source.wrapping_add(offset));
                e.mem.set_u32(entry, word);
                let source = e.mem.u32(record + 0xc);
                let word = e.mem.u32(source.wrapping_add(offset) + 4);
                e.mem.set_u32(entry + 8, word);
                e.mem.set_u32(entry + 0xc, 0);
                let source = e.mem.u32(record + 0xc);
                let word = e.mem.u32(source.wrapping_add(offset) + 8);
                e.mem.set_u32(entry + 0x10, word);
                e.mem.set_u32(entry + 0x14, 0);
                index += 1;
                offset = offset.wrapping_add(0x10);
                if index >= e.mem.i32(record + 0x10) {
                    break;
                }
            }
        }
        let swap = e.mem.u8(rules + 1) != e.mem.u8(HOST_ENDIAN_FLAG);
        let data = e.mem.u32(bytes);
        let size = e.mem.u32(bytes + 4);
        let result = write_data(
            e,
            stream,
            Ptr::new(record),
            first_low,
            first_high,
            class,
            0,
            data,
            size,
            u32::from(swap),
        );
        e.call(RELOCATION_INFO_DESTRUCT, &args![record]);
        e.call(STREAM_WRITER_DESTRUCT, &args![stream_writer]);
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(bytes + 8);
        e.mem.set_u32(bytes + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(bytes);
            e.vcall(allocator, 0x10, &args![data, capacity & CAPACITY_MASK]);
        }
        e.mem.set_u32(bytes, 0);
        e.mem.set_u32(bytes + 8, NOT_OWNED);
        e.call(PLATFORM_WRITER_DESTRUCT, &args![writer]);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0b850, write_data(u32, Ptr, u32, u32, u32, u32, u32, u32, u32) -> i32),
        entry!(0x00e0ba10, read_object(Ptr, Ptr, Ptr, Ptr, Ptr) -> u64),
        entry!(0x00e0bd40, write_object(u32, u32, u32, u32, u32, u32, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1)
            .collect()
    }

    /// A router whose allocator allocates in the heap (slot 4, unused here) and records frees
    /// (slot `0x10`). Returns the allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.put_vtable(0x4400_0000, &[0, 0, 0, 0, 0x4500_0010]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_0000);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x4500_0010, |_, _| Ret::default());
        allocator
    }

    /// A stream that says "not ok" from its `bad`-th `isOk` question on (0 is never), and whose
    /// slot `0x10` records what it is given.
    fn install_stream(e: &mut Engine, bad: u32) -> u32 {
        e.put_vtable(0x4400_1000, &[0, 0, 0, 0x4500_0020, 0x4500_0021]);
        let stream = e.mem.alloc(8);
        e.mem.set_u32(stream, 0x4400_1000);
        let asked = Rc::new(RefCell::new(0u32));
        e.register_double(0x4500_0020, move |e, a| {
            *asked.borrow_mut() += 1;
            let ok = bad == 0 || *asked.borrow() < bad;
            e.mem.set_u8(a[1], u8::from(ok));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(0x4500_0021, |_, _| Ret::default());
        stream
    }

    fn archive_doubles(e: &mut Engine) {
        for address in [
            ARCHIVE_CONSTRUCT,
            ARCHIVE_WRITE_32,
            ARCHIVE_WRITE_32U,
            ARCHIVE_WRITE_64U,
            ARCHIVE_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// Tables with two first entries `(0x10, 0x11)`, `(0x20, 0x21)` and one second entry
    /// `(0x30, 0x31, 0x32, 0x33)`.
    fn tables(e: &mut Engine) -> u32 {
        let tables = e.mem.alloc(0x14);
        let first = e.mem.alloc(16);
        for (i, word) in [0x10u32, 0x11, 0x20, 0x21].into_iter().enumerate() {
            e.mem.set_u32(first + 4 * i as u32, word);
        }
        let second = e.mem.alloc(16);
        for (i, word) in [0x30u32, 0x31, 0x32, 0x33].into_iter().enumerate() {
            e.mem.set_u32(second + 4 * i as u32, word);
        }
        e.mem.set_u32(tables, first);
        e.mem.set_u32(tables + 4, 2);
        e.mem.set_u32(tables + 0xc, second);
        e.mem.set_u32(tables + 0x10, 1);
        tables
    }

    fn archive_calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        let log = e.call_log.clone().unwrap();
        log.into_iter()
            .filter(|call| {
                [
                    ARCHIVE_CONSTRUCT,
                    ARCHIVE_WRITE_32,
                    ARCHIVE_WRITE_32U,
                    ARCHIVE_WRITE_64U,
                    ARCHIVE_DESTRUCT,
                ]
                .contains(&call.0)
            })
            .map(|(address, words)| (address, words[1..].to_vec()))
            .collect()
    }

    #[test]
    fn write_data_writes_the_header_the_tables_and_the_data() {
        let mut e = Engine::new();
        archive_doubles(&mut e);
        let stream = install_stream(&mut e, 0);
        let tables = tables(&mut e);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00e0_b850,
                &args![stream, tables, 1u32, 2u32, 3u32, 4u32, 0x7000u32, 0x40u32, 1u32],
            )
            .i32();
        assert_eq!(
            archive_calls(&e),
            vec![
                (ARCHIVE_CONSTRUCT, vec![stream, 1]),
                (ARCHIVE_WRITE_64U, vec![1, 2]),
                (ARCHIVE_WRITE_64U, vec![3, 4]),
                (ARCHIVE_WRITE_32U, vec![2]),
                (ARCHIVE_WRITE_32, vec![0x10]),
                (ARCHIVE_WRITE_32, vec![0x11]),
                (ARCHIVE_WRITE_32, vec![0x20]),
                (ARCHIVE_WRITE_32, vec![0x21]),
                (ARCHIVE_WRITE_32U, vec![1]),
                (ARCHIVE_WRITE_32, vec![0x30]),
                (ARCHIVE_WRITE_64U, vec![0x31, 0]),
                (ARCHIVE_WRITE_64U, vec![0x32, 0]),
                (ARCHIVE_WRITE_32U, vec![0x40]),
                (ARCHIVE_DESTRUCT, vec![]),
            ]
        );
        // the data goes through the stream's slot 0x10
        assert_eq!(calls_to(&e, 0x4500_0021), vec![vec![stream, 0x7000, 0x40]]);
        // 0x14 + 2 * 8 + 4 + 0x14 + 0x40 + 4
        assert_eq!(result, 0x14 + 16 + 4 + 0x14 + 0x40 + 4);
    }

    #[test]
    fn write_data_stops_with_minus_one_when_the_stream_fails() {
        // the n-th `isOk` question is the first one to say no
        for (bad, asks_before_the_end) in [(1u32, 1usize), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)] {
            let mut e = Engine::new();
            archive_doubles(&mut e);
            let stream = install_stream(&mut e, bad);
            let tables = tables(&mut e);
            e.call_log = Some(vec![]);
            let result = e
                .call(
                    0x00e0_b850,
                    &args![stream, tables, 1u32, 2u32, 3u32, 4u32, 0x7000u32, 0x40u32, 0u32],
                )
                .i32();
            assert_eq!(result, -1, "bad {bad}");
            assert_eq!(calls_to(&e, 0x4500_0020).len(), asks_before_the_end);
            let calls = archive_calls(&e);
            assert_eq!(calls.last().unwrap().0, ARCHIVE_DESTRUCT);
            // nothing is written to the stream before the data
            assert!(calls_to(&e, 0x4500_0021).is_empty());
        }
    }

    #[test]
    fn write_data_also_fails_on_the_last_question() {
        // the seven questions of a successful run: header, second value, one per first entry
        // (2), one per second entry (1), the size, the data
        let mut e = Engine::new();
        archive_doubles(&mut e);
        let stream = install_stream(&mut e, 7);
        let tables = tables(&mut e);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00e0_b850,
                &args![stream, tables, 1u32, 2u32, 3u32, 4u32, 0x7000u32, 0x40u32, 0u32],
            )
            .i32();
        assert_eq!(result, -1);
        // the data was written, and then the check failed
        assert_eq!(calls_to(&e, 0x4500_0021).len(), 1);
    }

    /// The bytes `read_object` reads, in order: the first value, the second value, 2 fixups
    /// (offset 0 pointing at 8, offset -1 skipped), 1 relocation entry (word `0x55`, 8 bytes,
    /// 8 bytes), the size 0x10 and 16 bytes of data.
    fn object_bytes() -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(0x1111_2222_3333_4444u64.to_le_bytes());
        bytes.extend(0x5555_6666_7777_8888u64.to_le_bytes());
        bytes.extend(2u32.to_le_bytes());
        for word in [0u32, 8, 0xffff_ffff, 4] {
            bytes.extend(word.to_le_bytes());
        }
        bytes.extend(1u32.to_le_bytes());
        bytes.extend(0x55u32.to_le_bytes());
        bytes.extend(0x0102_0304_0506_0708u64.to_le_bytes());
        bytes.extend(0x1112_1314_1516_1718u64.to_le_bytes());
        bytes.extend(0x10u32.to_le_bytes());
        bytes.extend((0..16u8).map(|i| 0xa0 + i));
        bytes
    }

    /// A stream that serves `bytes` to slot `0x10`, whose `isOk` says no once fewer bytes were
    /// available than asked for.
    fn install_reader(e: &mut Engine, bytes: Vec<u8>) -> u32 {
        e.put_vtable(0x4400_2000, &[0, 0, 0, 0x4500_0030, 0x4500_0031]);
        let reader = e.mem.alloc(8);
        e.mem.set_u32(reader, 0x4400_2000);
        let state = Rc::new(RefCell::new((bytes, 0usize, true)));
        let reading = Rc::clone(&state);
        e.register_double(0x4500_0031, move |e, a| {
            let mut state = reading.borrow_mut();
            let (bytes, cursor, ok) = &mut *state;
            let wanted = a[2] as usize;
            if *cursor + wanted > bytes.len() {
                *ok = false;
            } else {
                e.mem.write(a[1], &bytes[*cursor..*cursor + wanted]);
                *cursor += wanted;
            }
            Ret::default()
        });
        e.register_double(0x4500_0030, move |e, a| {
            let ok = state.borrow().2;
            e.mem.set_u8(a[1], u8::from(ok));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        reader
    }

    fn reading_engine() -> Engine {
        let mut e = Engine::new();
        install_allocator(&mut e);
        e.register(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(EASY_FREE, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            // (allocator, array, capacity, element size)
            let block = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e
    }

    #[test]
    fn read_object_reads_the_block_applies_the_fixups_and_returns_the_first_value() {
        let mut e = reading_engine();
        let reader = install_reader(&mut e, object_bytes());
        let (block_out, size_out, second_out) = (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(8));
        let relocations = e.mem.alloc(12);
        e.mem.set_u32(relocations + 8, NOT_OWNED);
        e.call_log = Some(vec![]);
        let value = e
            .call(
                0x00e0_ba10,
                &args![reader, block_out, size_out, relocations, second_out],
            )
            .u64();
        assert_eq!(value, 0x1111_2222_3333_4444);
        assert_eq!(e.mem.u64(second_out), 0x5555_6666_7777_8888);
        assert_eq!(e.mem.u32(size_out), 0x10);
        let block = e.mem.u32(block_out);
        assert_eq!(e.mem.block_size(block), Some(0x10));
        // the data, with the first fixup applied: offset 0 now holds block + 8
        assert_eq!(e.mem.u32(block), block + 8);
        assert_eq!(e.mem.u8(block + 4), 0xa4);
        assert_eq!(e.mem.u8(block + 15), 0xaf);
        // the second fixup (offset -1) was skipped
        // the relocation entry: word +0, 8 bytes at +8 and +0x10
        assert_eq!(e.mem.u32(relocations + 4), 1);
        let entry = e.mem.u32(relocations);
        assert_eq!(e.mem.u32(entry), 0x55);
        assert_eq!(e.mem.u64(entry + 8), 0x0102_0304_0506_0708);
        assert_eq!(e.mem.u64(entry + 0x10), 0x1112_1314_1516_1718);
        // the local array (capacity 2, owned) was freed with 2 * 8 bytes
        let frees = calls_to(&e, 0x4500_0010);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][2], 16);
    }

    #[test]
    fn read_object_gives_up_when_the_stream_runs_dry() {
        let bytes = object_bytes();
        // every prefix that cuts a group short: no block is stored and the result is 0
        for length in [0usize, 7, 12, 22, 26, 33, 40, 48, 56, 63, 70] {
            let mut e = reading_engine();
            let reader = install_reader(&mut e, bytes[..length].to_vec());
            let (block_out, size_out, second_out) =
                (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(8));
            let relocations = e.mem.alloc(12);
            e.mem.set_u32(relocations + 8, NOT_OWNED);
            e.call_log = Some(vec![]);
            let value = e
                .call(
                    0x00e0_ba10,
                    &args![reader, block_out, size_out, relocations, second_out],
                )
                .u64();
            assert_eq!(value, 0, "{length} bytes");
            assert_eq!(e.mem.u32(block_out), 0, "{length} bytes");
        }
    }

    #[test]
    fn read_object_frees_the_block_it_could_not_fill() {
        let mut e = reading_engine();
        let bytes = object_bytes();
        // everything but the last data byte
        let reader = install_reader(&mut e, bytes[..bytes.len() - 1].to_vec());
        let (block_out, size_out, second_out) = (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(8));
        let relocations = e.mem.alloc(12);
        e.mem.set_u32(relocations + 8, NOT_OWNED);
        e.call_log = Some(vec![]);
        let value = e
            .call(
                0x00e0_ba10,
                &args![reader, block_out, size_out, relocations, second_out],
            )
            .u64();
        assert_eq!(value, 0);
        assert_eq!(calls_to(&e, EASY_FREE).len(), 1);
        assert_eq!(e.mem.u32(block_out), 0);
    }

    /// Everything `write_object` calls, with the relocation record that `00e02a80` fills with
    /// one entry `(7, 8, 9, 10)` in its second array.
    fn writing_engine() -> Engine {
        let mut e = Engine::new();
        install_allocator(&mut e);
        archive_doubles(&mut e);
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u8(HOST_ENDIAN_FLAG, 1);
        e.register(PLATFORM_WRITER_CONSTRUCT, |_, _| Ret::default());
        e.register(PLATFORM_WRITER_DESTRUCT, |_, _| Ret::default());
        e.register(RELOCATION_INFO_DESTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITER_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITER_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_RESERVE, |e, a| {
            let block = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            // (allocator, array, element size): twice the capacity, at least 4
            let capacity = (e.mem.u32(a[1] + 8) & CAPACITY_MASK).max(2) * 2;
            let block = e.mem.alloc(capacity * a[2]);
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let bytes = e.mem.bytes(old, size * a[2]);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, capacity);
            Ret::default()
        });
        e.register(PLATFORM_WRITE_OBJECT, |e, a| {
            // (writer, stream writer, object, class, record): the record's second array gets
            // one entry (7, 8, 9, 10)
            let record = a[4];
            let entry = e.mem.alloc(16);
            for (i, word) in [7u32, 8, 9, 10].into_iter().enumerate() {
                e.mem.set_u32(entry + 4 * i as u32, word);
            }
            e.mem.set_u32(record + 0xc, entry);
            e.mem.set_u32(record + 0x10, 1);
            e.mem.set_u32(record + 0x14, 1);
            Ret::default()
        });
        e
    }

    #[test]
    fn write_object_appends_the_relocations_and_writes_the_data() {
        let mut e = writing_engine();
        let stream = install_stream(&mut e, 0);
        let rules = e.mem.alloc(8);
        e.mem.set_u8(rules + 1, 1);
        let relocations = e.mem.alloc(12);
        e.mem.set_u32(relocations + 8, NOT_OWNED);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00e0_bd40,
                &args![stream, rules, 5u32, 6u32, 0x1000u32, 0x2000u32, relocations],
            )
            .i32();
        // (1) the platform writer is built from the rules and asked to write the object
        assert_eq!(calls_to(&e, PLATFORM_WRITER_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, PLATFORM_WRITER_CONSTRUCT)[0][1], rules);
        let write = &calls_to(&e, PLATFORM_WRITE_OBJECT)[0];
        assert_eq!(write[2..4], [0x1000, 0x2000]);
        // (2) the relocations array holds one 0x18-byte entry: words 7, -, 8, 0, 9, 0
        assert_eq!(e.mem.u32(relocations + 4), 1);
        let entry = e.mem.u32(relocations);
        assert_eq!(e.mem.u32(entry), 7);
        assert_eq!(e.mem.u32(entry + 8), 8);
        assert_eq!(e.mem.u32(entry + 0xc), 0);
        assert_eq!(e.mem.u32(entry + 0x10), 9);
        assert_eq!(e.mem.u32(entry + 0x14), 0);
        // (3) `_writeData` got (5, 6), (class, 0) and the swap flag 0 (the byte equals the host's)
        let calls = archive_calls(&e);
        assert_eq!(calls[0], (ARCHIVE_CONSTRUCT, vec![stream, 0]));
        assert_eq!(calls[1], (ARCHIVE_WRITE_64U, vec![5, 6]));
        assert_eq!(calls[2], (ARCHIVE_WRITE_64U, vec![0x2000, 0]));
        // the record, the stream writer and the platform writer are destroyed
        assert_eq!(calls_to(&e, RELOCATION_INFO_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, STREAM_WRITER_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, PLATFORM_WRITER_DESTRUCT).len(), 1);
        // the result is `_writeData`'s: header 0x14, no first entries, one second entry 0x14 +
        // its count 4, then the size (0 bytes of local array) and 4
        assert_eq!(result, 0x14 + 4 + 0x14 + 4);
    }

    #[test]
    fn write_object_passes_a_set_swap_flag_when_the_endianness_differs() {
        let mut e = writing_engine();
        let stream = install_stream(&mut e, 0);
        let rules = e.mem.alloc(8);
        e.mem.set_u8(rules + 1, 0);
        let relocations = e.mem.alloc(12);
        e.mem.set_u32(relocations + 8, NOT_OWNED);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_bd40,
            &args![stream, rules, 5u32, 6u32, 0x1000u32, 0x2000u32, relocations],
        );
        assert_eq!(archive_calls(&e)[0], (ARCHIVE_CONSTRUCT, vec![stream, 1]));
    }

    #[test]
    fn write_object_makes_room_for_the_relocations_it_appends() {
        let mut e = writing_engine();
        let stream = install_stream(&mut e, 0);
        let rules = e.mem.alloc(8);
        e.mem.set_u8(rules + 1, 1);
        // an owned array that already holds one entry in a capacity of 1
        let relocations = e.mem.alloc(12);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(relocations, data);
        e.mem.set_u32(relocations + 4, 1);
        e.mem.set_u32(relocations + 8, 1);
        e.mem.set_u32(data, 0x99);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_bd40,
            &args![stream, rules, 5u32, 6u32, 0x1000u32, 0x2000u32, relocations],
        );
        // 1 + 1 needed against a capacity of 1: `_reserve` with twice the capacity (2)
        let reserve = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserve.len(), 1);
        assert_eq!(reserve[0][2..], [2, 0x18]);
        assert_eq!(e.mem.u32(relocations + 4), 2);
        // the array had room: no second reservation
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn write_object_reserves_more_when_an_append_finds_the_array_full() {
        let mut e = writing_engine();
        let stream = install_stream(&mut e, 0);
        // a reservation that does not grow anything: the append finds size == capacity
        e.register(ARRAY_RESERVE, |_, _| Ret::default());
        let rules = e.mem.alloc(8);
        e.mem.set_u8(rules + 1, 1);
        let relocations = e.mem.alloc(12);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(relocations, data);
        e.mem.set_u32(relocations + 4, 1);
        e.mem.set_u32(relocations + 8, 1);
        e.mem.set_u32(data, 0x99);
        e.call_log = Some(vec![]);
        e.call(
            0x00e0_bd40,
            &args![stream, rules, 5u32, 6u32, 0x1000u32, 0x2000u32, relocations],
        );
        let more = calls_to(&e, ARRAY_RESERVE_MORE);
        assert_eq!(more.len(), 1);
        assert_eq!(more[0][1..], [relocations, 0x18]);
        assert_eq!(e.mem.u32(relocations + 4), 2);
        // the old entry was kept by the growth, the new one follows it
        let entries = e.mem.u32(relocations);
        assert_eq!(e.mem.u32(entries), 0x99);
        assert_eq!(e.mem.u32(entries + 0x18), 7);
    }
}
