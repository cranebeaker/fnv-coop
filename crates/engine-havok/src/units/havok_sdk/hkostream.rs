//! `Havok SDK/hkostream.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cc07f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cc_0780`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cc07f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cc_0780, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkString::strLen` (`00cb10d0`, cdecl, one argument).
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// `"(null)"`.
const NULL_TEXT: u32 = 0x010c_9308;

// Translated from 00cc0080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a C string to a stream through the stream's vtable slot 0x10 (`write(buffer,
/// length)`), or `"(null)"` (6 bytes) when `text` is null. The exe takes the stream in EDI and
/// the text in ESI, so the parameters are in that order; the result is whatever the write
/// call left in EAX. The identity of the stream class is unconfirmed.
pub fn fn_00cc0080(e: &mut Engine, stream: Ptr, text: Ptr) -> u32 {
    if text.addr() != 0 {
        let length = e.call(STRING_LENGTH, &args![text]).u32();
        e.vcall(stream.addr(), 0x10, &args![text, length]).u32()
    } else {
        e.vcall(stream.addr(), 0x10, &args![NULL_TEXT, 6u32]).u32()
    }
}

// Translated from 00cc01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes `text` through the writer held at `this + 8` (`00cc0080`) and returns `this`. The
/// class is unconfirmed (an `hkOstream` has its `hkStreamWriter*` at +8).
pub fn fn_00cc01c0(e: &mut Engine, this: Ptr, text: Ptr) -> Ptr {
    let writer = e.mem.u32(this.addr() + 8);
    fn_00cc0080(e, Ptr::new(writer), text);
    this
}

// Translated from 00cc04c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8560.
pub fn hk_ostream_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8560
}

// Translated from 00cc0770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::getClassType` (Xbox PDB): takes no arguments and returns the constant 0x0126_8ad8.
pub fn hk_array_stream_writer_get_class_type(_e: &mut Engine) -> u32 {
    0x0126_8ad8
}

/// `hkString::snprintf` (`00cb0f60`, cdecl `(buffer, size, format, ...)`), `hkString::vsnprintf`
/// (`00cb0f50`, cdecl `(buffer, size, format, argument list)`) and `hkStringPtr::getLength`
/// (`00cb0a80`, `this` = the string pointer record).
const SNPRINTF: u32 = 0x00cb_0f60;
const VSNPRINTF: u32 = 0x00cb_0f50;
const STRING_PTR_LENGTH: u32 = 0x00cb_0a80;
/// The size of the stack buffer the `<<` operators format into (their `alloca_probe` of
/// `0x2728` bytes leaves 10020 bytes of it; the probe itself is left out here and the buffer is
/// a [`Engine::with_stack`] block).
const FORMAT_BUFFER_SIZE: u32 = 0x2728;
/// The format strings: `0x010a5824` (`operator<<` taking an `int`), `0x01020774` (`short` and
/// `int`), `0x010c1a98` (`unsigned short` and `unsigned int`), `0x01094cf0` (`float`),
/// `"%I64i"` at `0x010c9320` and `"%I64u"` at `0x010c9328`.
const FORMAT_INT: u32 = 0x010a_5824;
const FORMAT_SIGNED: u32 = 0x0102_0774;
const FORMAT_UNSIGNED: u32 = 0x010c_1a98;
const FORMAT_FLOAT: u32 = 0x0109_4cf0;
const FORMAT_INT64: u32 = 0x010c_9320;
const FORMAT_UINT64: u32 = 0x010c_9328;
/// The global holding the file-system object whose virtual slot `0x10` opens a writer.
const FILE_SYSTEM_GLOBAL: u32 = 0x0126_8364;
/// The buffered stream writer's constructor (`this`, three words).
const BUFFERED_STREAM_WRITER_CONSTRUCTOR: u32 = 0x00d2_74c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl: allocator, array, capacity, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `0x00d46be0` (`this` = the array writer's array record): its destructor.
const ARRAY_DESTRUCTOR: u32 = 0x00d4_6be0;
/// The text `"writer"` the content statistics are reported under.
const WRITER_NAME: u32 = 0x010c_9330;
/// Vtables: the `hkOstream` (`0x010c9314`), the memory track stream writer (`0x010c933c`), the
/// array stream writer (`0x010c9368`) and the base (`0x0102e388`).
const OSTREAM_VTABLE: u32 = 0x010c_9314;
const TRACK_WRITER_VTABLE: u32 = 0x010c_933c;
const ARRAY_WRITER_VTABLE: u32 = 0x010c_9368;
const BASE_VTABLE: u32 = 0x0102_e388;

/// The allocator object: the word at `+0x10` of the memory router's structure.
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Formats `words` with `format` into a stack buffer through `snprintf` (the exe's
/// `alloca_probe` of the buffer is left out) and writes the result to the writer at
/// `this + 8` through `00cc0080`. Returns `this`.
fn format_and_write(e: &mut Engine, this: Ptr, format: u32, words: &[u32]) -> Ptr {
    e.with_stack(FORMAT_BUFFER_SIZE, |e, buffer| {
        let mut arguments = vec![buffer.addr(), FORMAT_BUFFER_SIZE, format];
        arguments.extend_from_slice(words);
        e.call(SNPRINTF, &arguments);
        let writer = e.mem.u32(this.addr() + 8);
        fn_00cc0080(e, Ptr::new(writer), buffer);
    });
    this
}

// Translated from 00cc0120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, the `int` form the map names), `(value)`: formats the
/// word with the format at `0x010a5824` into a buffer (`snprintf`, size `0x2728`) and writes
/// the text ([`fn_00cc0080`]). Returns `this`. The exe's `alloca_probe` of the buffer is left
/// out.
pub fn hk_ostream_shl(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    format_and_write(e, this, FORMAT_INT, &[value])
}

// Translated from 00cc01a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, the `char` form): writes one byte, the argument word's
/// low byte, through the writer's slot `0x10` with the address of that word and length 1.
/// Returns `this`.
pub fn hk_ostream_shl_ov2(e: &mut Engine, this: Ptr, value: u8) -> Ptr {
    let writer = e.mem.u32(this.addr() + 8);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), u32::from(value));
        e.vcall(writer, 0x10, &args![slot, 1u32]);
    });
    this
}

// Translated from 00cc01e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `operator<<` for a `short`: the word sign-extended from 16 bits,
/// formatted with `0x01020774` and written ([`fn_00cc0080`]). Returns `this`.
pub fn fn_00cc01e0(e: &mut Engine, this: Ptr, value: i16) -> Ptr {
    format_and_write(e, this, FORMAT_SIGNED, &[i32::from(value) as u32])
}

// Translated from 00cc0230 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `operator<<` for an `unsigned short`: the word zero-extended from
/// 16 bits, formatted with `0x010c1a98` and written. Returns `this`.
pub fn fn_00cc0230(e: &mut Engine, this: Ptr, value: u16) -> Ptr {
    format_and_write(e, this, FORMAT_UNSIGNED, &[u32::from(value)])
}

// Translated from 00cc0280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, `operator_shl_ov3`): the signed word formatted with
/// `0x01020774` and written. Returns `this`.
pub fn hk_ostream_shl_ov3(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    format_and_write(e, this, FORMAT_SIGNED, &[value])
}

// Translated from 00cc02d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, `operator_shl_ov4`): the unsigned word formatted with
/// `0x010c1a98` and written. Returns `this`.
pub fn hk_ostream_shl_ov4(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    format_and_write(e, this, FORMAT_UNSIGNED, &[value])
}

// Translated from 00cc0320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, `operator_shl_ov5`, `float`): the float is loaded on the
/// x87 stack (`FLD`, a signalling NaN quieted) and passed to `snprintf` as a double (two
/// words) with the format at `0x01094cf0`; the text is written. `value` is the float's bits.
/// Returns `this`.
pub fn hk_ostream_shl_ov5(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    let double = x87_load32(value).to_bits();
    format_and_write(
        e,
        this,
        FORMAT_FLOAT,
        &[double as u32, (double >> 32) as u32],
    )
}

// Translated from 00cc0370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `operator<<` for a signed 64-bit number (two words, low
/// first): formatted with `"%I64i"` (`0x010c9320`) and written. Returns `this`.
pub fn fn_00cc0370(e: &mut Engine, this: Ptr, value: u64) -> Ptr {
    format_and_write(e, this, FORMAT_INT64, &[value as u32, (value >> 32) as u32])
}

// Translated from 00cc03c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `operator<<` for an unsigned 64-bit number (two words, low
/// first): formatted with `"%I64u"` (`0x010c9328`) and written. Returns `this`.
pub fn fn_00cc03c0(e: &mut Engine, this: Ptr, value: u64) -> Ptr {
    format_and_write(
        e,
        this,
        FORMAT_UINT64,
        &[value as u32, (value >> 32) as u32],
    )
}

// Translated from 00cc0410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::printf` (Xbox PDB), cdecl `(stream, format, ...)`: `arguments` are the words
/// after the format. They are copied into memory and `vsnprintf` (`00cb0f50`) gets the address
/// of the first as its argument list; the result (buffer of `0x2728` bytes, the exe's
/// `alloca_probe` of it left out) is written to the writer at `stream + 8` ([`fn_00cc0080`]).
pub fn hk_ostream_printf(e: &mut Engine, stream: Ptr, format: u32, arguments: &[u32]) {
    e.with_stack(FORMAT_BUFFER_SIZE, |e, buffer| {
        e.with_stack((4 * arguments.len() as u32).max(4), |e, list| {
            for (i, word) in arguments.iter().enumerate() {
                e.mem.set_u32(list.addr() + 4 * i as u32, *word);
            }
            e.call(VSNPRINTF, &args![buffer, FORMAT_BUFFER_SIZE, format, list]);
        });
        let writer = e.mem.u32(stream.addr() + 8);
        fn_00cc0080(e, Ptr::new(writer), buffer);
    });
}

/// `hkOstream::printf` in the uniform form: `(stream, format, ...)`.
fn printf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    hk_ostream_printf(e, Ptr::new(words[0]), words[1], &words[2..]);
    Ret::default()
}

// Translated from 00cc0470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a tail jump to the writer's (`+8`) virtual slot `0x10` with the
/// same two stack words `(buffer, length)`; returns the writer's `EAX`.
pub fn fn_00cc0470(e: &mut Engine, this: Ptr, buffer: Ptr, length: u32) -> u32 {
    let writer = e.mem.u32(this.addr() + 8);
    e.vcall(writer, 0x10, &args![buffer, length]).u32()
}

// Translated from 00cc04a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::calcContentStatistics` (Xbox PDB), `(collector, _unused)`: calls the
/// collector's virtual slot `0xC` with `("writer", writer, 0)` where the writer is the one at
/// `this + 8`. The second stack word (the exe's `RET 8`) is not read.
pub fn hk_ostream_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let writer = e.mem.u32(this.addr() + 8);
    e.vcall(collector.addr(), 0xc, &args![WRITER_NAME, writer, 0u32]);
}

// Translated from 00cc04d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the `hkOstream` constructor that opens its writer through the
/// file-system object held in the global `0x01268364` (its virtual slot `0x10`, one word):
/// reference count 1 (`+6`), vtable `0x010c9314`, the writer at `+8`. Returns `this`.
pub fn fn_00cc04d0(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OSTREAM_VTABLE);
    let file_system = e.mem.u32(FILE_SYSTEM_GLOBAL);
    let writer = e.vcall(file_system, 0x10, &args![name]).u32();
    e.mem.set_u32(this.addr() + 8, writer);
    this
}

// Translated from 00cc0500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::hkOstream` (Xbox PDB), `(a, b, c)`: reference count 1 (`+6`), vtable
/// `0x010c9314`, a 0x1c-byte block from the memory router's allocator (slot 4) with its size
/// word set, built by the buffered stream writer's constructor (`00d274c0`, the three words),
/// kept at `+8`. Returns `this`.
pub fn hk_ostream_constructor(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OSTREAM_VTABLE);
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![0x1cu32]).u32();
    e.mem.set_u16(block + 4, 0x1c);
    let writer = e
        .call(BUFFERED_STREAM_WRITER_CONSTRUCTOR, &args![block, a, b, c])
        .u32();
    e.mem.set_u32(this.addr() + 8, writer);
    this
}

// Translated from 00cc0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the `hkOstream` constructor that makes a memory track stream
/// writer: reference count 1 (`+6`), vtable `0x010c9314`, a 0x10-byte block from the
/// allocator (slot 4) with size word `0x10`, count 1, vtable `0x010c933c`, `value` at `+8` and 1
/// at `+0xC`, kept at `+8`. Returns `this`.
pub fn fn_00cc0550(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OSTREAM_VTABLE);
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![0x10u32]).u32();
    e.mem.set_u16(block + 4, 0x10);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block, TRACK_WRITER_VTABLE);
    e.mem.set_u32(block + 8, value);
    e.mem.set_u32(block + 0xc, 1);
    e.mem.set_u32(this.addr() + 8, block);
    this
}

// Translated from 00cc05a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::operator<<` (Xbox PDB, `operator_shl_ov6`, an `hkStringPtr`), `(string)`: when
/// the pointer word of the string record has text (the word without its low flag bit),
/// writes it with its length (`hkStringPtr::getLength`, `00cb0a80`) through the writer's
/// slot `0x10`; otherwise writes `"(null)"`, 6 bytes. Returns `this`.
pub fn hk_ostream_shl_ov6(e: &mut Engine, this: Ptr, string: Ptr) -> Ptr {
    let text = e.mem.u32(string.addr()) & 0xffff_fffe;
    let writer = e.mem.u32(this.addr() + 8);
    if text != 0 {
        let length = e.call(STRING_PTR_LENGTH, &args![string]).u32();
        let writer = e.mem.u32(this.addr() + 8);
        e.vcall(writer, 0x10, &args![text, length]);
    } else {
        e.vcall(writer, 0x10, &args![NULL_TEXT, 6u32]);
    }
    this
}

// Translated from 00cc05f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `operator<<` for a string record `(data, size including the
/// terminator)`: writes `size - 1` bytes of `data` through the writer's slot `0x10`, or
/// `"(null)"`, 6 bytes, when `data` is null. Returns `this`.
pub fn fn_00cc05f0(e: &mut Engine, this: Ptr, string: Ptr) -> Ptr {
    let data = e.mem.u32(string.addr());
    let writer = e.mem.u32(this.addr() + 8);
    if data != 0 {
        let size = e.mem.u32(string.addr() + 4);
        e.vcall(writer, 0x10, &args![data, size.wrapping_sub(1)]);
    } else {
        e.vcall(writer, 0x10, &args![NULL_TEXT, 6u32]);
    }
    this
}

// Translated from 00cc0630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkOstream::hkOstream` (Xbox PDB, `hkOstream_ov2`), `(array)`: reference count 1 (`+6`),
/// vtable `0x010c9314`, a 0x14-byte block from the allocator (slot 4) with its size word set,
/// made an array stream writer over `array` with flag 1 ([`hk_array_stream_writer_constructor`])
/// and kept at `+8`. Returns `this`.
pub fn hk_ostream_constructor_ov2(e: &mut Engine, this: Ptr, array: Ptr) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), OSTREAM_VTABLE);
    let heap = allocator(e);
    let block = e.vcall(heap, 4, &args![0x14u32]).u32();
    e.mem.set_u16(block + 4, 0x14);
    let writer = hk_array_stream_writer_constructor(e, Ptr::new(block), array, 1);
    e.mem.set_u32(this.addr() + 8, writer.addr());
    this
}

// Translated from 00cc0680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamWriter::tell` (Xbox PDB): with the track record at `this + 8`, `(count
/// at +0xC - 1) * (word at +0) + (word at +4)`.
pub fn hk_memory_track_stream_writer_tell(e: &mut Engine, this: Ptr) -> u32 {
    let record = e.mem.u32(this.addr() + 8);
    e.mem
        .u32(record + 0xc)
        .wrapping_sub(1)
        .wrapping_mul(e.mem.u32(record))
        .wrapping_add(e.mem.u32(record + 4))
}

// Translated from 00cc0690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMemoryTrackStreamWriter::_scalar_deleting_destructor_` (Xbox PDB), `(flags)`: installs the
/// track writer's vtable (`0x010c933c`); when `+0xC` is 0 and the record at `+8` exists runs
/// the record's destructor (`00d46be0`) and gives its 0x14 bytes back to the allocator (slot
/// 8); leaves the base vtable and, with bit 0 of `flags`, gives the object's block back with
/// its size word. Returns `this`.
pub fn hk_memory_track_stream_writer_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), TRACK_WRITER_VTABLE);
    if e.mem.u32(this.addr() + 0xc) == 0 {
        let record = e.mem.u32(this.addr() + 8);
        if record != 0 {
            e.call(ARRAY_DESTRUCTOR, &args![record]);
            let heap = allocator(e);
            e.vcall(heap, 8, &args![record, 0x14u32]);
        }
    }
    e.mem.set_u32(this.addr(), BASE_VTABLE);
    if flags & 1 != 0 {
        let heap = allocator(e);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(heap, 8, &args![this, size]);
    }
    this
}

// Translated from 00cc06f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayStreamWriter::nullTerminate` (Xbox PDB name; the body is the constructor), `(array,
/// flag)`: reference count 1 (`+6`), vtable `0x010c9368`, the array at `+8`, its size at `+0xC`
/// and `flag` at `+0x10`; then room for one more byte is made in the array (`_reserve` with
/// the larger of twice the capacity and size + 1 when the capacity is below size + 1) and a 0
/// byte is stored after the data. Returns `this`.
pub fn hk_array_stream_writer_constructor(e: &mut Engine, this: Ptr, array: Ptr, flag: u32) -> Ptr {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr(), ARRAY_WRITER_VTABLE);
    e.mem.set_u32(this.addr() + 8, array.addr());
    let size = e.mem.u32(array.addr() + 4);
    e.mem.set_u32(this.addr() + 0xc, size);
    e.mem.set_u32(this.addr() + 0x10, flag);
    let wanted = e.mem.u32(array.addr() + 4).wrapping_add(1) as i32;
    let heap = allocator(e);
    let capacity = (e.mem.u32(array.addr() + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(ARRAY_RESERVE, &args![heap, array, new_capacity, 1u32]);
    }
    let held = e.mem.u32(this.addr() + 8);
    let end = e.mem.u32(held).wrapping_add(e.mem.u32(held + 4));
    e.mem.set_u8(end, 0);
    this
}

// Translated from 00cc0780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArray<char,hkContainerHeapAllocator>::_scalar_deleting_destructor_` (Xbox PDB name; the
/// body is the array stream writer's destructor): installs the array writer's vtable
/// (`0x010c9368`); when the flag at `+0x10` is 0 and the array record at `+8` exists, empties
/// it (size 0, storage given back through the allocator's slot `0x10` unless the capacity word
/// is negative, data 0, capacity `0x80000000`) and gives the record's 0xC bytes back (slot 8);
/// leaves the base vtable.
pub fn hk_array_stream_writer_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_WRITER_VTABLE);
    if e.mem.u32(this.addr() + 0x10) == 0 {
        let record = e.mem.u32(this.addr() + 8);
        if record != 0 {
            let heap = allocator(e);
            let capacity = e.mem.u32(record + 8);
            e.mem.set_u32(record + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(record);
                e.vcall(heap, 0x10, &args![data, capacity & 0x3fff_ffff]);
            }
            e.mem.set_u32(record, 0);
            e.mem.set_u32(record + 8, 0x8000_0000);
            let heap = allocator(e);
            e.vcall(heap, 8, &args![record, 0xcu32]);
        }
    }
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc_0080, fn_00cc0080(Ptr, Ptr) -> u32),
        entry!(0x00cc_01c0, fn_00cc01c0(Ptr, Ptr) -> Ptr),
        entry!(0x00cc_07f0, deleting_destructor_00cc07f0(Ptr, u8) -> Ptr),
        entry!(0x00cc04c0, hk_ostream_get_class_type() -> u32),
        entry!(0x00cc0770, hk_array_stream_writer_get_class_type() -> u32),
        entry!(0x00cc_0120, hk_ostream_shl(Ptr, u32) -> Ptr),
        entry!(0x00cc_01a0, hk_ostream_shl_ov2(Ptr, u8) -> Ptr),
        entry!(0x00cc_01e0, fn_00cc01e0(Ptr, i16) -> Ptr),
        entry!(0x00cc_0230, fn_00cc0230(Ptr, u16) -> Ptr),
        entry!(0x00cc_0280, hk_ostream_shl_ov3(Ptr, u32) -> Ptr),
        entry!(0x00cc_02d0, hk_ostream_shl_ov4(Ptr, u32) -> Ptr),
        entry!(0x00cc_0320, hk_ostream_shl_ov5(Ptr, u32) -> Ptr),
        entry!(0x00cc_0370, fn_00cc0370(Ptr, u64) -> Ptr),
        entry!(0x00cc_03c0, fn_00cc03c0(Ptr, u64) -> Ptr),
        (0x00cc_0410, printf_entry as AbiFn),
        entry!(0x00cc_0470, fn_00cc0470(Ptr, Ptr, u32) -> u32),
        entry!(
            0x00cc_04a0,
            hk_ostream_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00cc_04d0, fn_00cc04d0(Ptr, u32) -> Ptr),
        entry!(0x00cc_0500, hk_ostream_constructor(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00cc_0550, fn_00cc0550(Ptr, u32) -> Ptr),
        entry!(0x00cc_05a0, hk_ostream_shl_ov6(Ptr, Ptr) -> Ptr),
        entry!(0x00cc_05f0, fn_00cc05f0(Ptr, Ptr) -> Ptr),
        entry!(0x00cc_0630, hk_ostream_constructor_ov2(Ptr, Ptr) -> Ptr),
        entry!(0x00cc_0680, hk_memory_track_stream_writer_tell(Ptr) -> u32),
        entry!(0x00cc_0690, hk_memory_track_stream_writer_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00cc_06f0, hk_array_stream_writer_constructor(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cc_0780, hk_array_stream_writer_destructor(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    type WriteLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// A stream whose vtable slot 0x10 is a double logging `(stream, buffer, length)`, and a
    /// `strLen` double answering 5.
    fn stream_rig(e: &mut Engine) -> (u32, WriteLog) {
        const WRITE: u32 = 0x0900_0022;
        let log: WriteLog = Rc::new(RefCell::new(Vec::new()));
        let write_log = Rc::clone(&log);
        e.register_double(WRITE, move |_, a| {
            write_log.borrow_mut().push((a[0], a[1], a[2]));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        e.register(STRING_LENGTH, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, WRITE);
        let stream = e.mem.alloc(0x10);
        e.mem.set_u32(stream, vtable);
        (stream, log)
    }

    #[test]
    fn fn_00cc0080_writes_the_text_or_the_null_marker() {
        let mut e = engine();
        let (stream, log) = stream_rig(&mut e);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00cc_0080, &args![stream, text]).u32(), 0x77);
        assert_eq!(e.call(0x00cc_0080, &args![stream, 0u32]).u32(), 0x77);
        assert_eq!(
            *log.borrow(),
            vec![(stream, text, 5), (stream, NULL_TEXT, 6)]
        );
    }

    #[test]
    fn fn_00cc01c0_writes_through_the_writer_and_returns_this() {
        let mut e = engine();
        let (stream, log) = stream_rig(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, stream);
        let text = e.mem.alloc(8);
        assert_eq!(e.call(0x00cc_01c0, &args![this, text]).u32(), this);
        assert_eq!(*log.borrow(), vec![(stream, text, 5)]);
    }

    #[test]
    fn deleting_destructor_00cc07f0_frees_only_when_bit_zero_is_set() {
        check(0x00cc_07f0, 0x00cc_0780);
    }

    #[test]
    fn hk_ostream_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_04c0, &args![]).u32(), 0x0126_8560);
    }

    #[test]
    fn hk_array_stream_writer_get_class_type_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_0770, &args![]).u32(), 0x0126_8ad8);
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const WRITE_SLOT: u32 = 0x0900_0030;
    const ALLOCATE_SLOT: u32 = 0x0900_0031;
    const FREE_BLOCK_SLOT: u32 = 0x0900_0032;
    const FREE_ARRAY_SLOT: u32 = 0x0900_0033;
    const OPEN_SLOT: u32 = 0x0900_0034;
    const COLLECTOR_SLOT: u32 = 0x0900_0035;

    fn rig() -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.map(0x010c_9000, 0x1000);
        e.mem.set_cstr(NULL_TEXT, b"(null)");
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        // The writer: slot 0x10 logs the text and its length.
        let seen = Rc::clone(&log);
        e.register_double(WRITE_SLOT, move |e, a| {
            let text = String::from_utf8_lossy(&e.mem.bytes(a[1], a[2])).into_owned();
            seen.borrow_mut().push(format!("write:{}:{}", a[2], text));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, WRITE_SLOT);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer, vtable);
        // The stream object holding the writer at +8.
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, writer);
        // The allocator: slot 4 hands out blocks, slot 8 and 0x10 log.
        let seen = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            seen.borrow_mut().push(format!("allocate:{:x}", a[1]));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_BLOCK_SLOT, move |_, a| {
            seen.borrow_mut().push(format!("free-block:{:x}", a[2]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_ARRAY_SLOT, move |_, a| {
            seen.borrow_mut().push(format!("free-array:{:x}", a[2]));
            Ret::default()
        });
        let heap_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(heap_vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(heap_vtable + 8, FREE_BLOCK_SLOT);
        e.mem.set_u32(heap_vtable + 0x10, FREE_ARRAY_SLOT);
        let heap = e.mem.alloc(0x10);
        e.mem.set_u32(heap, heap_vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, log, this, writer)
    }

    /// An `snprintf` double that writes `fmt<format>:<words>` into the buffer.
    fn install_snprintf(e: &mut Engine, log: &Log) {
        let seen = Rc::clone(log);
        e.register_double(SNPRINTF, move |e, a| {
            let words: Vec<String> = a[3..].iter().map(|w| format!("{w:x}")).collect();
            seen.borrow_mut()
                .push(format!("snprintf:{:x}:{:x}", a[1], a[2]));
            let text = format!("[{}]", words.join(","));
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
    }

    #[test]
    fn shl_formats_the_word_and_writes_it() {
        let (mut e, log, this, _) = rig();
        install_snprintf(&mut e, &log);
        let got = e.call(0x00cc_0120, &args![this, 0x2au32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![
                format!("snprintf:2728:{FORMAT_INT:x}"),
                "write:4:[2a]".to_string()
            ]
        );
    }

    #[test]
    fn shl_variants_use_their_formats_and_extensions() {
        let cases: [(u32, Vec<u32>, u32, &str); 5] = [
            (0x00cc_0280, vec![0xffff_fff0], FORMAT_SIGNED, "[fffffff0]"),
            (
                0x00cc_02d0,
                vec![0xffff_fff0],
                FORMAT_UNSIGNED,
                "[fffffff0]",
            ),
            // `short`: sign-extended; `unsigned short`: zero-extended.
            (0x00cc_01e0, vec![0xfffe], FORMAT_SIGNED, "[fffffffe]"),
            (0x00cc_0230, vec![0xfffe], FORMAT_UNSIGNED, "[fffe]"),
            // A float is passed as a double: 1.5 is 0x3ff8000000000000.
            (0x00cc_0320, vec![0x3fc0_0000], FORMAT_FLOAT, "[0,3ff80000]"),
        ];
        for (address, args_in, format, text) in cases {
            let (mut e, log, this, _) = rig();
            install_snprintf(&mut e, &log);
            let mut words = vec![this];
            words.extend(args_in);
            let got = e.call(address, &words).u32();
            assert_eq!(got, this);
            let log = log.borrow();
            assert_eq!(log[0], format!("snprintf:2728:{format:x}"));
            assert_eq!(log[1], format!("write:{}:{}", text.len(), text));
        }
    }

    #[test]
    fn shl_of_64_bit_numbers_passes_both_words() {
        for (address, format) in [(0x00cc_0370, FORMAT_INT64), (0x00cc_03c0, FORMAT_UINT64)] {
            let (mut e, log, this, _) = rig();
            install_snprintf(&mut e, &log);
            let got = e
                .call(address, &args![this, 0x1122_3344_5566_7788u64])
                .u32();
            assert_eq!(got, this);
            let log = log.borrow();
            assert_eq!(log[0], format!("snprintf:2728:{format:x}"));
            assert_eq!(log[1], "write:19:[55667788,11223344]");
        }
    }

    #[test]
    fn shl_of_a_char_writes_the_low_byte_through_a_pointer() {
        let (mut e, log, this, writer) = rig();
        let seen = Rc::clone(&log);
        e.register_double(WRITE_SLOT, move |e, a| {
            assert_eq!(a[0], writer);
            seen.borrow_mut()
                .push(format!("byte:{:x}:{}", e.mem.u8(a[1]), a[2]));
            Ret::default()
        });
        let got = e.call(0x00cc_01a0, &args![this, 0x1234_5641u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec!["byte:41:1".to_string()]);
    }

    #[test]
    fn printf_formats_with_the_argument_list_and_writes() {
        let (mut e, log, this, _) = rig();
        let seen = Rc::clone(&log);
        e.register_double(VSNPRINTF, move |e, a| {
            let first = e.mem.u32(a[3]);
            let second = e.mem.u32(a[3] + 4);
            seen.borrow_mut().push(format!(
                "vsnprintf:{:x}:{:x}:{first:x}:{second:x}",
                a[1], a[2]
            ));
            e.mem.set_cstr(a[0], b"hello");
            Ret::default()
        });
        e.call(0x00cc_0410, &[this, 0x4444, 7, 8]);
        assert_eq!(
            *log.borrow(),
            vec![
                "vsnprintf:2728:4444:7:8".to_string(),
                "write:5:hello".to_string()
            ]
        );
    }

    #[test]
    fn fn_00cc0470_forwards_to_the_writer() {
        let (mut e, log, this, _) = rig();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abc");
        assert_eq!(e.call(0x00cc_0470, &args![this, text, 3u32]).u32(), 0x77);
        assert_eq!(*log.borrow(), vec!["write:3:abc".to_string()]);
    }

    #[test]
    fn calc_content_statistics_reports_the_writer_under_its_name() {
        let (mut e, log, this, writer) = rig();
        let seen = Rc::clone(&log);
        e.register_double(COLLECTOR_SLOT, move |_, a| {
            seen.borrow_mut()
                .push(format!("collect:{:x}:{:x}:{:x}", a[1], a[2], a[3]));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, COLLECTOR_SLOT);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.call(0x00cc_04a0, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![format!("collect:{WRITER_NAME:x}:{writer:x}:0")]
        );
    }

    #[test]
    fn fn_00cc04d0_opens_its_writer_through_the_file_system() {
        let (mut e, _, _, _) = rig();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, OPEN_SLOT);
        let file_system = e.mem.alloc(0x10);
        e.mem.set_u32(file_system, vtable);
        e.mem.set_u32(FILE_SYSTEM_GLOBAL, file_system);
        e.register_double(OPEN_SLOT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cc_04d0, &args![this, 0x700u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OSTREAM_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0x701);
    }

    #[test]
    fn hk_ostream_constructor_builds_a_buffered_writer() {
        let (mut e, log, _, _) = rig();
        let seen = Rc::clone(&log);
        e.register_double(BUFFERED_STREAM_WRITER_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut()
                .push(format!("buffered:{:x}:{:x}:{:x}", a[1], a[2], a[3]));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cc_0500, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(got, this);
        let writer = e.mem.u32(this + 8);
        assert_eq!(e.mem.u16(writer + 4), 0x1c);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), OSTREAM_VTABLE);
        assert_eq!(
            *log.borrow(),
            vec!["allocate:1c".to_string(), "buffered:1:2:3".to_string()]
        );
    }

    #[test]
    fn fn_00cc0550_builds_a_track_writer() {
        let (mut e, _, _, _) = rig();
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cc_0550, &args![this, 0x9999u32]).u32();
        assert_eq!(got, this);
        let block = e.mem.u32(this + 8);
        assert_eq!((e.mem.u16(block + 4), e.mem.u16(block + 6)), (0x10, 1));
        assert_eq!(e.mem.u32(block), TRACK_WRITER_VTABLE);
        assert_eq!((e.mem.u32(block + 8), e.mem.u32(block + 0xc)), (0x9999, 1));
        assert_eq!(e.mem.u32(this), OSTREAM_VTABLE);
    }

    #[test]
    fn shl_of_a_string_ptr_writes_text_or_null() {
        let (mut e, log, this, _) = rig();
        e.register(STRING_PTR_LENGTH, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"xyz");
        let record = e.mem.alloc(4);
        // The low bit is a flag.
        e.mem.set_u32(record, text | 1);
        assert_eq!(e.call(0x00cc_05a0, &args![this, record]).u32(), this);
        e.mem.set_u32(record, 1);
        e.call(0x00cc_05a0, &args![this, record]);
        assert_eq!(
            *log.borrow(),
            vec!["write:3:xyz".to_string(), "write:6:(null)".to_string()]
        );
    }

    #[test]
    fn shl_of_a_sized_string_writes_size_minus_one_or_null() {
        let (mut e, log, this, _) = rig();
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"abcd");
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, text);
        e.mem.set_u32(record + 4, 5);
        assert_eq!(e.call(0x00cc_05f0, &args![this, record]).u32(), this);
        e.mem.set_u32(record, 0);
        e.call(0x00cc_05f0, &args![this, record]);
        assert_eq!(
            *log.borrow(),
            vec!["write:4:abcd".to_string(), "write:6:(null)".to_string()]
        );
    }

    #[test]
    fn array_stream_writer_constructor_reserves_room_and_terminates() {
        let (mut e, log, _, _) = rig();
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut()
                .push(format!("reserve:{:x}:{:x}", a[2], a[3]));
            let block = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(4);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 3);
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cc_06f0, &args![this, array, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), ARRAY_WRITER_VTABLE);
        assert_eq!(e.mem.u32(this + 8), array);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        // Capacity 3 < size + 1: twice the capacity (6) is larger than 4.
        assert_eq!(*log.borrow(), vec!["reserve:6:1".to_string()]);
        let new_data = e.mem.u32(array);
        assert_eq!(e.mem.u8(new_data + 3), 0);
        // With enough capacity no reserve is made.
        log.borrow_mut().clear();
        e.mem.set_u32(array + 8, 0x20);
        e.call(0x00cc_06f0, &args![this, array, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }

    #[test]
    fn hk_ostream_constructor_ov2_makes_an_array_writer() {
        let (mut e, log, _, _) = rig();
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(8);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        e.mem.set_u32(array + 8, 8);
        let this = e.mem.alloc(0x20);
        let got = e.call(0x00cc_0630, &args![this, array]).u32();
        assert_eq!(got, this);
        let writer = e.mem.u32(this + 8);
        assert_eq!(e.mem.u16(writer + 4), 0x14);
        assert_eq!(e.mem.u32(writer), ARRAY_WRITER_VTABLE);
        assert_eq!(e.mem.u32(writer + 0x10), 1);
        assert_eq!(*log.borrow(), vec!["allocate:14".to_string()]);
        assert_eq!(e.mem.u8(data + 2), 0);
    }

    #[test]
    fn track_writer_tell_is_the_count_times_the_chunk_plus_the_offset() {
        let (mut e, _, this, _) = rig();
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 100);
        e.mem.set_u32(record + 4, 7);
        e.mem.set_u32(record + 0xc, 3);
        e.mem.set_u32(this + 8, record);
        assert_eq!(e.call(0x00cc_0680, &args![this]).u32(), 207);
    }

    #[test]
    fn track_writer_deleting_destructor_frees_by_flags() {
        let (mut e, log, _, _) = rig();
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_DESTRUCTOR, move |_, a| {
            seen.borrow_mut().push(format!("destroy:{:x}", a[0]));
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let record = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, record);
        e.mem.set_u16(this + 4, 0x30);
        let got = e.call(0x00cc_0690, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(
            *log.borrow(),
            vec![format!("destroy:{record:x}"), "free-block:14".to_string()]
        );
        log.borrow_mut().clear();
        // Bit 0 also frees the object itself; a nonzero +0xC keeps the record.
        e.mem.set_u32(this + 0xc, 1);
        e.call(0x00cc_0690, &args![this, 1u32]);
        assert_eq!(*log.borrow(), vec!["free-block:30".to_string()]);
    }

    #[test]
    fn array_stream_writer_destructor_empties_an_owned_array() {
        let (mut e, log, _, _) = rig();
        let this = e.mem.alloc(0x20);
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, 0x5000);
        e.mem.set_u32(record + 4, 4);
        e.mem.set_u32(record + 8, 6);
        e.mem.set_u32(this + 8, record);
        e.call(0x00cc_0780, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec!["free-array:6".to_string(), "free-block:c".to_string()]
        );
        assert_eq!(
            (
                e.mem.u32(record),
                e.mem.u32(record + 4),
                e.mem.u32(record + 8)
            ),
            (0, 0, 0x8000_0000)
        );
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        // A nonzero flag at +0x10 leaves the array alone.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x10, 1);
        e.call(0x00cc_0780, &args![this]);
        assert!(log.borrow().is_empty());
    }
}
