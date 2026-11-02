//! `BSHavok/bhkstream.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions move Havok data through an `NiStream`: its reader is the object at
//! `stream + 0x24C` and its writer the object at `stream + 0x250`; both keep the stream
//! position at `+4` and call a function pointer (`+8` reads, `+0xC` writes) as
//! `(io, buffer, size, count_slot, counted)`; the position advances by what the function
//! answers. `stream + 0x1E4` and `+0x1E5` are 1 when the stream is in PC byte order for
//! writing and reading, and `stream + 4` is the stream version. An `hkArray` is three
//! words: data at `+0`, size at `+4` and capacity (low 30 bits, the top bit set when the
//! array does not own its storage) at `+8`.
//!
//! The loaders that allocate raise the per-thread allocation word at `TLS + 0x2b4` to 0x10
//! for their duration and put the old value back. The exception-unwinding frames (`FS:[0]`
//! registrations) and the `/GS` cookie check of the functions marked `Translation
//! incomplete` are not modeled.

use super::bhklimitedhingeconstraint::{allocator, enter_scope, stream_transfer};
#[allow(unused_imports)]
use crate::prelude::*;

/// Byte swap of `n` words in place (`00a62150`, cdecl `(pointer, n)`).
const SWAP_WORDS: u32 = 0x00a6_2150;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, n, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The `float` the vector readers put in the fourth lane (`01011d78`).
const FOURTH_LANE: u32 = 0x0101_1d78;
/// Vtable of `hkpMoppCode` (`0102e368`, RTTI-confirmed), whose 0x30-byte object
/// `LoadBinary_ov3` builds.
const VTABLE_MOPP_CODE: u32 = 0x0102_e368;

/// One transfer from the stream's reader (`stream + 0x24C`).
fn read_from_stream(e: &mut Engine, stream: u32, buffer: u32, size: u32, counted: bool) {
    let io = e.mem.u32(stream + 0x24c);
    stream_transfer(e, io, 8, buffer, size, counted);
}

/// One transfer to the stream's writer (`stream + 0x250`).
fn write_to_stream(e: &mut Engine, stream: u32, buffer: u32, size: u32, counted: bool) {
    let io = e.mem.u32(stream + 0x250);
    stream_transfer(e, io, 0xc, buffer, size, counted);
}

/// Byte-swaps the four words at `address` in place.
fn swap_four_words(e: &mut Engine, address: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(address + 4 * lane);
        e.mem.set_u32(address + 4 * lane, word.swap_bytes());
    }
}

// Translated from 00cf0930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf0930 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkStream::SwapHKArray` (Xbox PDB), cdecl `(data, byte_count, array)`: with `n` =
/// `byte_count / 4` (truncating toward zero), runs the word swap `00a62150` on `(data, n)`,
/// keeps the three words of `array`, runs the same swap again and puts the three words
/// back.
pub fn bhk_stream_swap_hk_array(e: &mut Engine, data: u32, byte_count: i32, array: Ptr) {
    let n = byte_count.wrapping_add((byte_count >> 31) & 3) >> 2;
    e.call(SWAP_WORDS, &args![data, n]);
    let size = e.mem.u32(array.addr() + 4);
    let capacity = e.mem.u32(array.addr() + 8);
    let first = e.mem.u32(array.addr());
    e.call(SWAP_WORDS, &args![data, n]);
    e.mem.set_u32(array.addr(), first);
    e.mem.set_u32(array.addr() + 4, size);
    e.mem.set_u32(array.addr() + 8, capacity);
}

// Translated from 00cf09a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, cdecl `(stream, data)`: writes ten 4-byte words of `data` to the stream's
/// writer, in the order `+0`, `+4`, `+8`, `+0x10`, `+0x14`, `+0x18`, `+0x1C`, `+0x20`,
/// `+0x24`, `+0x28` (`+0xC` is not written).
pub fn fn_00cf09a0(e: &mut Engine, stream: u32, data: u32) {
    for offset in [0, 4, 8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28] {
        write_to_stream(e, stream, data + offset, 4, true);
    }
}

// Translated from 00cf0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, cdecl `(stream, dest)`: the reading counterpart of `00cf09a0`. Reads ten
/// 4-byte words from the stream's reader. The first three become the words `+0`, `+4`
/// and `+8` of `dest`, with the `float` at `01011d78` as the word at `+0xC`; the next
/// four become the words at `+0x10..+0x1C`; the last three become `+0x20`, `+0x24` and
/// `+0x28`, again with the `float` at `01011d78` at `+0x2C`. Each vector is stored as soon
/// as its words are read.
pub fn fn_00cf0b00(e: &mut Engine, stream: u32, dest: u32) {
    e.with_stack(0x10, |e, cells| {
        let cell = cells.addr();
        for (target, words) in [(dest, 3), (dest + 0x10, 4), (dest + 0x20, 3)] {
            for lane in 0..words {
                read_from_stream(e, stream, cell + 4 * lane, 4, true);
            }
            for lane in 0..words {
                let word = e.mem.u32(cell + 4 * lane);
                e.mem.set_u32(target + 4 * lane, word);
            }
            if words == 3 {
                let fourth = e.mem.u32(FOURTH_LANE);
                e.mem.set_u32(target + 0xc, fourth);
            }
        }
    });
}

// Translated from 00cf0cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf0cf0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkStream::SaveBinary` (Xbox PDB), cdecl `(stream, array)`: writes the array's size
/// (`+4`) as four bytes, then, for each of that many 16-byte elements (the size is read
/// back from the buffer it was written from, as a signed number), writes the element at
/// the array's data pointer (read again each time) plus 0x10 times the index. Answers the
/// size times 16.
pub fn bhk_stream_save_binary(e: &mut Engine, stream: u32, array: Ptr) -> u32 {
    e.with_stack(0x20, |e, cells| {
        let count_cell = cells.addr();
        let element = cells.addr() + 0x10;
        let count = e.mem.u32(array.addr() + 4);
        e.mem.set_u32(count_cell, count);
        write_to_stream(e, stream, count_cell, 4, true);
        let result = e.mem.u32(count_cell) << 4;
        let mut index = 0i32;
        while index < e.mem.i32(count_cell) {
            let data = e.mem.u32(array.addr());
            let bytes = e.mem.bytes(data + 0x10 * index as u32, 0x10);
            e.mem.write(element, &bytes);
            write_to_stream(e, stream, element, 0x10, true);
            index += 1;
        }
        result
    })
}

// Translated from 00cf0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf0dc0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkStream::SaveBinary_ov2` (Xbox PDB), cdecl `(stream, object)`, the writer of the
/// `hkpMoppCode` (`+0x10` its 16-byte offset, `+0x20` its data pointer and `+0x24` the
/// data size, as `LoadBinary_ov3` builds it): nothing for a null object (answers 0).
/// Otherwise writes the size as four bytes, the 16 bytes at `+0x10`, and, when the size
/// (read back from the buffer it was written from) is not 0, that many bytes from the
/// data pointer, with the count slot holding 1. Answers the size.
pub fn bhk_stream_save_binary_ov2(e: &mut Engine, stream: u32, object: u32) -> u32 {
    if object == 0 {
        return 0;
    }
    e.with_stack(0x20, |e, cells| {
        let size_cell = cells.addr();
        let offset_buffer = cells.addr() + 0x10;
        let size = e.mem.u32(object + 0x24);
        e.mem.set_u32(size_cell, size);
        write_to_stream(e, stream, size_cell, 4, true);
        let bytes = e.mem.bytes(object + 0x10, 0x10);
        e.mem.write(offset_buffer, &bytes);
        write_to_stream(e, stream, offset_buffer, 0x10, true);
        let size = e.mem.u32(size_cell);
        if size != 0 {
            let data = e.mem.u32(object + 0x20);
            let io = e.mem.u32(stream + 0x250);
            let function = e.mem.u32(io + 0xc);
            let done = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), 1);
                e.call(function, &args![io, data, size, slot, 1u32]).u32()
            });
            let position = e.mem.u32(io + 4);
            e.mem.set_u32(io + 4, position.wrapping_add(done));
        }
        e.mem.u32(size_cell)
    })
}

// Translated from 00cf0e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkStream::SaveBinary_ov3` (Xbox PDB), cdecl `(stream, array)`: writes the array's
/// size (`+4`) as four bytes, then, for each of that many words (the size is read back
/// from the buffer it was written from, as a signed number; the data pointer is read again
/// each time), writes the word, byte-swapped when `stream + 0x1E4` is not 1. Answers the
/// size times 4.
pub fn bhk_stream_save_binary_ov3(e: &mut Engine, stream: u32, array: Ptr) -> u32 {
    e.with_stack(0x10, |e, cells| {
        let count_cell = cells.addr();
        let word_cell = cells.addr() + 4;
        let count = e.mem.u32(array.addr() + 4);
        e.mem.set_u32(count_cell, count);
        write_to_stream(e, stream, count_cell, 4, true);
        let result = e.mem.u32(count_cell) << 2;
        let mut index = 0u32;
        while (index as i32) < e.mem.i32(count_cell) {
            let data = e.mem.u32(array.addr());
            let mut word = e.mem.u32(data + 4 * index);
            if e.mem.u8(stream + 0x1e4) != 1 {
                word = word.swap_bytes();
            }
            e.mem.set_u32(word_cell, word);
            write_to_stream(e, stream, word_cell, 4, true);
            index += 1;
        }
        result
    })
}

/// Frees what `array` owns (when its capacity word has the top bit clear, with `size_shift`
/// giving the element size as a shift), clears its size and storage, makes it hold no
/// storage of its own (capacity word `0x80000000`) and reserves room for `count` elements
/// of `element_size` bytes (the capacity doubling is irrelevant for an empty array but is
/// kept as the code computes it); finally sets its size to `count`. This is the part the
/// array loaders share.
fn reset_array(e: &mut Engine, array: u32, count: i32, size_shift: u32, element_size: u32) {
    let heap = allocator(e);
    let capacity_word = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity_word & 0x3fff_ffff) << size_shift;
        e.vcall(heap, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let heap = allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![heap, array, wanted, element_size]);
    }
    e.mem.set_u32(array + 4, count as u32);
}

// Translated from 00cf0f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf0f40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStream::LoadBinary` (Xbox PDB), cdecl `(stream, array)`: the reading counterpart of
/// `SaveBinary`. With the allocation-scope word at 0x10, reads the element count (four
/// bytes), frees the array's storage when it owns it (the allocator's slot `0x10`,
/// `(data, capacity * 16)`), empties it, reserves room for the count (`00c908c0`, element
/// size 0x10) and sets its size. Then reads each 16-byte element to the data pointer plus
/// 0x10 times the index (the count is unsigned here; the read passes 0 for the count slot
/// and the flag) and byte-swaps its four words unless `stream + 0x1E5` is 1. Answers the
/// count times 16.
pub fn bhk_stream_load_binary(e: &mut Engine, stream: u32, array: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let count = e.with_stack(4, |e, cell| {
        read_from_stream(e, stream, cell.addr(), 4, true);
        e.mem.u32(cell.addr())
    });
    reset_array(e, array, count as i32, 4, 0x10);
    let result = count << 4;
    let mut offset = 0u32;
    for _ in 0..count {
        let data = e.mem.u32(array);
        read_from_stream(e, stream, data + offset, 0x10, false);
        if e.mem.u8(stream + 0x1e5) != 1 {
            let data = e.mem.u32(array);
            swap_four_words(e, data + offset);
        }
        offset = offset.wrapping_add(0x10);
    }
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cf10f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf10f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStream::LoadBinary_ov2` (Xbox PDB), cdecl `(stream, array)`: like `LoadBinary` for
/// 4-byte elements (the storage freed is `capacity * 4` bytes, the reserve element size
/// is 4) with a signed count, and each element is read to the data pointer plus 4 times
/// the index with the count slot holding 4 and the flag 1; no byte swap. Answers the count
/// times 4.
pub fn bhk_stream_load_binary_ov2(e: &mut Engine, stream: u32, array: u32) -> u32 {
    let (word, saved) = enter_scope(e);
    let count = e.with_stack(4, |e, cell| {
        read_from_stream(e, stream, cell.addr(), 4, true);
        e.mem.i32(cell.addr())
    });
    reset_array(e, array, count, 2, 4);
    let result = (count as u32) << 2;
    let mut index = 0u32;
    while (index as i32) < count {
        let data = e.mem.u32(array);
        read_from_stream(e, stream, data + 4 * index, 4, true);
        index += 1;
    }
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cf1270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cf1270 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkStream::LoadBinary_ov3` (Xbox PDB), cdecl `(stream, out)`: builds an `hkpMoppCode`
/// and reads it. With the allocation-scope word at 0x10, takes 0x30 bytes from the
/// allocator (slot 4, the result is not checked) and makes them an `hkpMoppCode` (vtable
/// `0102e368`; `+6` = 1, `+4` = 0x30, the 16 bytes at `+0x10` and the data array at
/// `+0x20` empty with capacity word `0x80000000`, the byte at `+0x2C` = 2); the object
/// goes to `*out`. Reads a count (four bytes) and the 16 bytes at `+0x10` (count slot 0,
/// flag 0). For a stream version below 6 and a nonzero count, the count loses 0x30 and
/// 0x20 bytes are read and dropped (count slot 4, flag 1: eight 4-byte words). For a
/// nonzero count the data array is reserved (`00c908c0`, element size 1) and set to that
/// size, and that many bytes are read to its data pointer. A stream not in PC byte order
/// (`stream + 0x1E5` not 1) swaps the four words at `+0x10`. Answers the count.
pub fn bhk_stream_load_binary_ov3(e: &mut Engine, stream: u32, out: Ptr) -> u32 {
    let (word, saved) = enter_scope(e);
    let heap = allocator(e);
    let object = e.vcall(heap, 4, &args![0x30u32]).u32();
    e.mem.set_u32(object, VTABLE_MOPP_CODE);
    e.mem.set_u16(object + 6, 1);
    e.mem.set_u16(object + 4, 0x30);
    e.mem.set_u32(object + 0x20, 0);
    e.mem.set_u32(object + 0x24, 0);
    e.mem.set_u32(object + 0x28, 0x8000_0000);
    e.mem.set_u8(object + 0x2c, 2);
    for lane in 0..4 {
        e.mem.set_u32(object + 0x10 + 4 * lane, 0);
    }
    e.mem.set_u32(out.addr(), object);
    let result = e.with_stack(4, |e, cell| {
        let cell = cell.addr();
        read_from_stream(e, stream, cell, 4, true);
        let object = e.mem.u32(out.addr());
        read_from_stream(e, stream, object + 0x10, 0x10, false);
        if e.mem.u32(stream + 4) < 6 && e.mem.u32(cell) != 0 {
            let reduced = e.mem.u32(cell).wrapping_sub(0x30);
            e.mem.set_u32(cell, reduced);
            // exe 00cf136a-00cf1396: the count slot holds 4, not the size (00cf1389), so the
            // reader takes the 0x20 bytes as eight 4-byte words; flag 1.
            e.with_stack(0x20, |e, dropped| {
                let io = e.mem.u32(stream + 0x24c);
                let function = e.mem.u32(io + 8);
                let done = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), 4);
                    e.call(function, &args![io, dropped.addr(), 0x20u32, slot, 1u32])
                        .u32()
                });
                let position = e.mem.u32(io + 4);
                e.mem.set_u32(io + 4, position.wrapping_add(done));
            });
        }
        let count = e.mem.u32(cell);
        if count != 0 {
            let object = e.mem.u32(out.addr());
            let array = object + 0x20;
            let heap = allocator(e);
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
            if capacity < count as i32 {
                let doubled = capacity.wrapping_add(capacity);
                let wanted = if count as i32 >= doubled {
                    count as i32
                } else {
                    doubled
                };
                e.call(ARRAY_RESERVE, &args![heap, array, wanted, 1u32]);
            }
            e.mem.set_u32(array + 4, count);
            let object = e.mem.u32(out.addr());
            let data = e.mem.u32(object + 0x20);
            let count = e.mem.u32(cell);
            read_from_stream(e, stream, data, count, false);
        }
        if e.mem.u8(stream + 0x1e5) != 1 {
            let object = e.mem.u32(out.addr());
            swap_four_words(e, object + 0x10);
        }
        e.mem.u32(cell)
    });
    e.mem.set_u32(word, saved);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf0930, bhk_stream_swap_hk_array(u32, i32, Ptr)),
        entry!(0x00cf09a0, fn_00cf09a0(u32, u32)),
        entry!(0x00cf0b00, fn_00cf0b00(u32, u32)),
        entry!(0x00cf0cf0, bhk_stream_save_binary(u32, Ptr) -> u32),
        entry!(0x00cf0dc0, bhk_stream_save_binary_ov2(u32, u32) -> u32),
        entry!(0x00cf0e90, bhk_stream_save_binary_ov3(u32, Ptr) -> u32),
        entry!(0x00cf0f40, bhk_stream_load_binary(u32, u32) -> u32),
        entry!(0x00cf10f0, bhk_stream_load_binary_ov2(u32, u32) -> u32),
        entry!(0x00cf1270, bhk_stream_load_binary_ov3(u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhklimitedhingeconstraint::support::*;
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    const READER: u32 = 0x0900_0008;
    const WRITER: u32 = 0x0900_000c;

    type Written = Rc<RefCell<Vec<Vec<u8>>>>;

    /// A stream whose reader hands out `reads` in turn (each must be exactly the size
    /// asked for) and whose writer collects the bytes it is given; both record their
    /// words in `log`. The stream is in PC byte order for both directions.
    fn scripted_stream(e: &mut Engine, reads: Vec<Vec<u8>>, log: &Calls) -> (u32, u32, Written) {
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(io + 8, READER);
        e.mem.set_u32(io + 0xc, WRITER);
        let queue = Rc::new(RefCell::new(VecDeque::from(reads)));
        let reader_log = Rc::clone(log);
        e.register_double(READER, move |e, a| {
            reader_log.borrow_mut().push((READER, a.to_vec()));
            let chunk = queue.borrow_mut().pop_front().expect("unexpected read");
            assert_eq!(chunk.len() as u32, a[2]);
            e.mem.write(a[1], &chunk);
            ret(a[2])
        });
        let written = Written::default();
        let sink = Rc::clone(&written);
        let writer_log = Rc::clone(log);
        e.register_double(WRITER, move |e, a| {
            writer_log.borrow_mut().push((WRITER, a.to_vec()));
            sink.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            ret(a[2])
        });
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u8(stream + 0x1e4, 1);
        e.mem.set_u8(stream + 0x1e5, 1);
        (stream, io, written)
    }

    #[test]
    fn swap_hk_array_swaps_twice_and_restores_the_header_words() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let array = e.mem.alloc(0x10);
        for (offset, value) in [(0, 0x11u32), (4, 0x22), (8, 0x33)] {
            e.mem.set_u32(array + offset, value);
        }
        // The first swap spoils the header; the second one is the point where it is read.
        let spoil = Rc::clone(&log);
        e.register_double(SWAP_WORDS, move |e, a| {
            spoil.borrow_mut().push((SWAP_WORDS, a.to_vec()));
            let calls = spoil.borrow().iter().filter(|c| c.0 == SWAP_WORDS).count();
            if calls == 1 {
                e.mem.set_u32(array + 4, 0x99);
            } else {
                e.mem.set_u32(array, 0xaa);
                e.mem.set_u32(array + 8, 0xbb);
            }
            Ret::default()
        });
        e.call(0x00cf_0930, &args![0x5000u32, 14u32, array]);
        assert_eq!(calls_to(&log, SWAP_WORDS), [[0x5000, 3], [0x5000, 3]]);
        // The words read between the two swaps come back: +4 as the first swap left it.
        assert_eq!(e.mem.u32(array), 0x11);
        assert_eq!(e.mem.u32(array + 4), 0x99);
        assert_eq!(e.mem.u32(array + 8), 0x33);
        // A negative byte count rounds toward zero, and a count under 4 gives 0 words.
        e.call(0x00cf_0930, &args![0x5000u32, -14i32, array]);
        assert_eq!(calls_to(&log, SWAP_WORDS)[2], [0x5000, (-3i32) as u32]);
        e.call(0x00cf_0930, &args![0x5000u32, 3u32, array]);
        assert_eq!(calls_to(&log, SWAP_WORDS)[4], [0x5000, 0]);
    }

    #[test]
    fn fn_00cf09a0_writes_ten_words_skipping_the_fourth() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let (stream, io, written) = scripted_stream(&mut e, vec![], &log);
        let data = e.mem.alloc(0x30);
        for index in 0..12 {
            e.mem.set_u32(data + 4 * index, 0x100 + index);
        }
        e.call(0x00cf_09a0, &args![stream, data]);
        let words: Vec<u32> = written
            .borrow()
            .iter()
            .map(|bytes| u32::from_le_bytes(bytes[..4].try_into().unwrap()))
            .collect();
        assert_eq!(
            words,
            [0x100, 0x101, 0x102, 0x104, 0x105, 0x106, 0x107, 0x108, 0x109, 0x10a]
        );
        let calls = calls_to(&log, WRITER);
        assert_eq!(calls.len(), 10);
        assert!(calls.iter().all(|c| c[0] == io && c[2] == 4 && c[4] == 1));
        assert_eq!(e.mem.u32(io + 4), 40);
    }

    #[test]
    fn fn_00cf0b00_reads_three_vectors_and_stores_each_after_its_words() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(FOURTH_LANE, 0x3f80_0000u32);
        let log: Calls = Rc::default();
        let reads: Vec<Vec<u8>> = (1..=10u32)
            .map(|n| (n * 0x10).to_le_bytes().to_vec())
            .collect();
        let (stream, io, _) = scripted_stream(&mut e, reads, &log);
        let dest = e.mem.alloc(0x30);
        e.call(0x00cf_0b00, &args![stream, dest]);
        let words: Vec<u32> = (0..12).map(|i| e.mem.u32(dest + 4 * i)).collect();
        assert_eq!(
            words,
            [
                0x10,
                0x20,
                0x30,
                0x3f80_0000,
                0x40,
                0x50,
                0x60,
                0x70,
                0x80,
                0x90,
                0xa0,
                0x3f80_0000
            ]
        );
        assert_eq!(calls_to(&log, READER).len(), 10);
        assert_eq!(e.mem.u32(io + 4), 40);
    }

    #[test]
    fn save_binary_writes_the_count_and_each_sixteen_byte_element() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let (stream, io, written) = scripted_stream(&mut e, vec![], &log);
        let data = e.mem.alloc(0x40);
        for index in 0..8 {
            e.mem.set_u32(data + 4 * index, 0x200 + index);
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        assert_eq!(e.call(0x00cf_0cf0, &args![stream, array]).u32(), 32);
        let chunks = written.borrow().clone();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], 2u32.to_le_bytes());
        assert_eq!(chunks[1], e.mem.bytes(data, 16));
        assert_eq!(chunks[2], e.mem.bytes(data + 16, 16));
        assert_eq!(e.mem.u32(io + 4), 36);
        // A zero (or negative) count writes only the count.
        written.borrow_mut().clear();
        e.mem.set_u32(array + 4, 0xffff_ffff);
        assert_eq!(
            e.call(0x00cf_0cf0, &args![stream, array]).u32(),
            0xffff_fff0
        );
        assert_eq!(written.borrow().len(), 1);
    }

    #[test]
    fn save_binary_ov2_writes_the_size_the_offset_and_the_data() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let (stream, io, written) = scripted_stream(&mut e, vec![], &log);
        assert_eq!(e.call(0x00cf_0dc0, &args![stream, 0u32]).u32(), 0);
        assert!(written.borrow().is_empty());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0403_0201);
        e.mem.set_u32(data + 4, 0x0807_0605);
        let object = e.mem.alloc(0x30);
        for index in 0..4 {
            e.mem.set_u32(object + 0x10 + 4 * index, 0x500 + index);
        }
        e.mem.set_u32(object + 0x20, data);
        e.mem.set_u32(object + 0x24, 6);
        assert_eq!(e.call(0x00cf_0dc0, &args![stream, object]).u32(), 6);
        let chunks = written.borrow().clone();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], 6u32.to_le_bytes());
        assert_eq!(chunks[1], e.mem.bytes(object + 0x10, 16));
        assert_eq!(chunks[2], [1, 2, 3, 4, 5, 6]);
        // The data write passes the size and a count slot of 1.
        let data_write = &calls_to(&log, WRITER)[2];
        assert_eq!((data_write[0], data_write[1], data_write[2]), (io, data, 6));
        assert_eq!(data_write[4], 1);
        assert_eq!(e.mem.u32(io + 4), 4 + 16 + 6);
        // No data bytes for a size of 0.
        written.borrow_mut().clear();
        e.mem.set_u32(object + 0x24, 0);
        assert_eq!(e.call(0x00cf_0dc0, &args![stream, object]).u32(), 0);
        assert_eq!(written.borrow().len(), 2);
    }

    #[test]
    fn save_binary_ov3_swaps_each_word_for_a_foreign_stream() {
        let mut e = Engine::new();
        let log: Calls = Rc::default();
        let (stream, _, written) = scripted_stream(&mut e, vec![], &log);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x0102_0304);
        e.mem.set_u32(data + 4, 0x0a0b_0c0d);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        assert_eq!(e.call(0x00cf_0e90, &args![stream, array]).u32(), 8);
        let words = |written: &Written| -> Vec<u32> {
            written
                .borrow()
                .iter()
                .map(|bytes| u32::from_le_bytes(bytes[..4].try_into().unwrap()))
                .collect()
        };
        assert_eq!(words(&written), [2, 0x0102_0304, 0x0a0b_0c0d]);
        written.borrow_mut().clear();
        e.mem.set_u8(stream + 0x1e4, 0);
        e.call(0x00cf_0e90, &args![stream, array]);
        assert_eq!(words(&written), [2, 0x0403_0201, 0x0d0c_0b0a]);
        // A zero count writes only the count.
        written.borrow_mut().clear();
        e.mem.set_u32(array + 4, 0);
        assert_eq!(e.call(0x00cf_0e90, &args![stream, array]).u32(), 0);
        assert_eq!(words(&written), [0]);
    }

    /// Doubles for `_reserve`: it gives the array a 0x80-byte block and capacity `n`.
    fn reserve_double(e: &mut Engine, log: &Calls) -> u32 {
        let block = e.mem.alloc(0x80);
        let reserved = Rc::clone(log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserved.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        block
    }

    #[test]
    fn load_binary_frees_empties_reserves_and_reads_swapped_elements() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, FREE_TARGET, 0, &log);
        let block = reserve_double(&mut e, &log);
        let mut reads = vec![2u32.to_le_bytes().to_vec()];
        reads.push((0..16).collect());
        reads.push((16..32).collect());
        let (stream, io, _) = scripted_stream(&mut e, reads, &log);
        e.mem.set_u8(stream + 0x1e5, 0);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x7000);
        e.mem.set_u32(array + 4, 5);
        e.mem.set_u32(array + 8, 3);
        assert_eq!(e.call(0x00cf_0f40, &args![stream, array]).u32(), 32);
        let frees = calls_to(&log, FREE_TARGET);
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [0x7000, 3 * 16]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[0][1..], [array, 2, 0x10]);
        assert_eq!(e.mem.u32(array), block);
        assert_eq!(e.mem.u32(array + 4), 2);
        // The element reads pass 0 for the slot and the flag, and the words come out swapped.
        let element_reads = &calls_to(&log, READER)[1..];
        assert_eq!(element_reads.len(), 2);
        assert_eq!(element_reads[1][1..], [block + 16, 16, 0, 0]);
        assert_eq!(e.mem.u32(block), 0x0001_0203);
        assert_eq!(e.mem.u32(block + 12), 0x0c0d_0e0f);
        assert_eq!(e.mem.u32(block + 16), 0x1011_1213);
        assert_eq!(e.mem.u32(io + 4), 4 + 32);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn load_binary_keeps_storage_it_does_not_own_and_reads_nothing_for_zero() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, FREE_TARGET, 0, &log);
        reserve_double(&mut e, &log);
        let (stream, _, _) = scripted_stream(&mut e, vec![0u32.to_le_bytes().to_vec()], &log);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x7000);
        e.mem.set_u32(array + 4, 5);
        e.mem.set_u32(array + 8, 0x8000_0003);
        assert_eq!(e.call(0x00cf_0f40, &args![stream, array]).u32(), 0);
        assert!(calls_to(&log, FREE_TARGET).is_empty());
        assert!(calls_to(&log, ARRAY_RESERVE).is_empty());
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn load_binary_ov2_reads_signed_counted_words_without_swapping() {
        let (mut e, word, _) = engine_with_allocator();
        let log: Calls = Rc::default();
        record(&mut e, FREE_TARGET, 0, &log);
        let block = reserve_double(&mut e, &log);
        let reads = vec![
            3u32.to_le_bytes().to_vec(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
        ];
        let (stream, io, _) = scripted_stream(&mut e, reads, &log);
        e.mem.set_u8(stream + 0x1e5, 0);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x7000);
        e.mem.set_u32(array + 8, 5);
        assert_eq!(e.call(0x00cf_10f0, &args![stream, array]).u32(), 12);
        assert_eq!(calls_to(&log, FREE_TARGET)[0][1..], [0x7000, 20]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[0][1..], [array, 3, 4]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(e.mem.u32(block), 0x0403_0201);
        assert_eq!(e.mem.u32(block + 8), 0x0c0b_0a09);
        let element_reads = &calls_to(&log, READER)[1..];
        assert_eq!(element_reads[2][1..3], [block + 8, 4]);
        assert_eq!(element_reads[2][4], 1);
        assert_eq!(e.mem.u32(io + 4), 16);
        assert_eq!(e.mem.u32(word), 0x77);
        // A negative count reads no elements and answers count * 4.
        let (stream, _, _) = scripted_stream(&mut e, vec![(-2i32).to_le_bytes().to_vec()], &log);
        assert_eq!(
            e.call(0x00cf_10f0, &args![stream, array]).u32(),
            (-8i32) as u32
        );
    }

    #[test]
    fn load_binary_ov3_builds_the_mopp_code_and_reads_its_data() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        let data_block = reserve_double(&mut e, &log);
        let mut reads = vec![4u32.to_le_bytes().to_vec()];
        reads.push((0x40..0x50).collect());
        reads.push(vec![0xa1, 0xa2, 0xa3, 0xa4]);
        let (stream, io, _) = scripted_stream(&mut e, reads, &log);
        e.mem.set_u32(stream + 4, 6);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00cf_1270, &args![stream, out]).u32(), 4);
        assert_eq!(e.mem.u32(out), block);
        assert_eq!(e.mem.u32(block), VTABLE_MOPP_CODE);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u16(block + 4), 0x30);
        assert_eq!(e.mem.u8(block + 0x2c), 2);
        assert_eq!(e.mem.u32(block + 0x10), 0x4342_4140);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[0][1..], [block + 0x20, 4, 1]);
        assert_eq!(e.mem.u32(block + 0x24), 4);
        assert_eq!(e.mem.u32(data_block), 0xa4a3_a2a1);
        let reads = calls_to(&log, READER);
        assert_eq!(reads[1][1..], [block + 0x10, 0x10, 0, 0]);
        assert_eq!(reads[2][1..], [data_block, 4, 0, 0]);
        assert_eq!(e.mem.u32(io + 4), 4 + 16 + 4);
        assert_eq!(e.mem.u32(word), 0x77);
    }

    #[test]
    fn load_binary_ov3_below_version_6_drops_0x20_bytes_as_four_byte_words() {
        let (mut e, word, block) = engine_with_allocator();
        let log: Calls = Rc::default();
        let data_block = reserve_double(&mut e, &log);
        let mut reads = vec![0x34u32.to_le_bytes().to_vec()];
        reads.push((0x40..0x50).collect());
        reads.push(vec![0xee; 0x20]);
        reads.push(vec![0xa1, 0xa2, 0xa3, 0xa4]);
        let (stream, io, _) = scripted_stream(&mut e, reads, &log);
        e.mem.set_u32(stream + 4, 5);
        // The word each read finds in its count slot (0 when it has none).
        const SLOT_READER: u32 = 0x0900_0020;
        let slots: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = Rc::clone(&slots);
        e.register_double(SLOT_READER, move |e, a| {
            let slot = if a[3] == 0 { 0 } else { e.mem.u32(a[3]) };
            seen.borrow_mut().push(slot);
            e.call(READER, a)
        });
        e.mem.set_u32(io + 8, SLOT_READER);
        let out = e.mem.alloc(4);
        // The count loses 0x30 and is what the function answers.
        assert_eq!(e.call(0x00cf_1270, &args![stream, out]).u32(), 4);
        let reads = calls_to(&log, READER);
        assert_eq!(reads.len(), 4);
        // exe 00cf1389: the dropped 0x20 bytes go with the slot holding 4 and the flag 1.
        assert_eq!((reads[2][0], reads[2][2], reads[2][4]), (io, 0x20, 1));
        assert_eq!(*slots.borrow(), [4, 0, 4, 0]);
        assert_eq!(calls_to(&log, ARRAY_RESERVE)[0][1..], [block + 0x20, 4, 1]);
        assert_eq!(e.mem.u32(block + 0x24), 4);
        assert_eq!(reads[3][1..], [data_block, 4, 0, 0]);
        assert_eq!(e.mem.u32(data_block), 0xa4a3_a2a1);
        assert_eq!(e.mem.u32(io + 4), 4 + 16 + 0x20 + 4);
        assert_eq!(e.mem.u32(word), 0x77);
    }
}
