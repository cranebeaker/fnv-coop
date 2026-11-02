//! `Havok SDK/hkbinarytagfilereader.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0` is the pointer temporary blocks are carved
/// from, `+0xc` is the temporary allocator and `+0x10` the allocator long-lived blocks and array
/// storage come from.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemory` helper `(allocator, size)` that answers a block (`00c85690`) and the one that
/// frees `(allocator, block)` (`00c856c0`); both cdecl.
const EASY_ALLOC: u32 = 0x00c8_5690;
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// `_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The CRT's 64-bit left shift (`00ec7810`, `(value as 64-bit, shift)`).
const SHIFT_LEFT_64: u32 = 0x00ec_7810;
/// The input archive the reader is built on (`hkIArchive`): `readArrayGeneric` (`00dd8fe0`,
/// `this` = the archive, `(buffer, element size, count)`), `readRaw` (`00dd90d0`, `(buffer,
/// size)`), the constructor (`00dd9130`, `(stream, a dword whose low byte is zero)`) and the
/// destructor (`00dd91f0`).
const READ_ARRAY: u32 = 0x00dd_8fe0;
const READ_RAW: u32 = 0x00dd_90d0;
const ARCHIVE_CONSTRUCT: u32 = 0x00dd_9130;
const ARCHIVE_DESTRUCT: u32 = 0x00dd_91f0;
/// `hkDataClass` helpers (`this` = the class handle): constructor (`00dba650`, `(class pointer)`),
/// destructor (`00dba660`), member count (`00dba730`) and all-member-info (`00dbaed0`, `(array
/// of 0x14-byte records)`).
const DATA_CLASS_CONSTRUCT: u32 = 0x00db_a650;
const DATA_CLASS_DESTRUCT: u32 = 0x00db_a660;
const DATA_CLASS_NUM_MEMBERS: u32 = 0x00db_a730;
const DATA_CLASS_ALL_MEMBER_INFO: u32 = 0x00db_aed0;
/// `hkDataObject` helpers (`this` = the object handle): constructor (`00dba760`, `(object
/// pointer)`), copy (`00dba780`, `(other handle)`), destructor (`00dba7f0`) and `operator[]`
/// (`00dba870`, `(out value, member key)`, answers `out`).
const DATA_OBJECT_CONSTRUCT: u32 = 0x00db_a760;
const DATA_OBJECT_COPY: u32 = 0x00db_a780;
const DATA_OBJECT_DESTRUCT: u32 = 0x00db_a7f0;
const DATA_OBJECT_INDEX: u32 = 0x00db_a870;
/// `hkDataObject::Value` setters (`this` = the value, a pair of words): type (`00dba920`),
/// byte (`00dbaa30`), 64-bit integer (`00dbaa90`, `(low, high)`), float (`00dbaab0`), vector
/// (`00dbab10`, `(floats, count)`), object (`00dbaa50`, `(object handle)`) and string
/// (`00dbab30`).
const VALUE_GET_TYPE: u32 = 0x00db_a920;
const VALUE_SET_BYTE: u32 = 0x00db_aa30;
const VALUE_SET_INT_64: u32 = 0x00db_aa90;
const VALUE_SET_FLOAT: u32 = 0x00db_aab0;
const VALUE_SET_VECTOR: u32 = 0x00db_ab10;
const VALUE_SET_OBJECT: u32 = 0x00db_aa50;
const VALUE_SET_STRING: u32 = 0x00db_ab30;
/// A handle's set-pointer (`00dbab50`, `this` = the handle, `(pointer)`, adds a reference) and
/// release (`00dbab70`).
const HANDLE_SET: u32 = 0x00db_ab50;
const HANDLE_RELEASE: u32 = 0x00db_ab70;
/// `hkDataArray` helpers (`this` = the array handle): type (`00dbac10`), set size (`00dbabe0`,
/// `(count)`), class (`00dbac20`, `(out class handle)`), element (`00dbab90`, `(out proxy,
/// index)`, answers `out`), swizzle object member (`00dbaea0`, `(out array handle, member
/// key)`), set bytes (`00dbae10`, `(bytes, count)`) and set floats (`00dbae80`, `(floats,
/// count)`). The element proxy has its own setters (`this` = the proxy): integer (`00dbada0`),
/// vector (`00dbad60`, `(floats, count)`), object (`00dbad00`, `(object handle)`), string
/// (`00dbadc0`) and as-array (`00dbacd0`, `(out array handle)`).
const ARRAY_GET_TYPE: u32 = 0x00db_ac10;
const ARRAY_SET_SIZE: u32 = 0x00db_abe0;
const ARRAY_GET_CLASS: u32 = 0x00db_ac20;
const ARRAY_ELEMENT: u32 = 0x00db_ab90;
const ARRAY_SWIZZLE_MEMBER: u32 = 0x00db_aea0;
const ARRAY_SET_BYTES: u32 = 0x00db_ae10;
const ARRAY_SET_FLOATS: u32 = 0x00db_ae80;
const ELEMENT_SET_INT: u32 = 0x00db_ada0;
const ELEMENT_SET_VECTOR: u32 = 0x00db_ad60;
const ELEMENT_SET_OBJECT: u32 = 0x00db_ad00;
const ELEMENT_SET_STRING: u32 = 0x00db_adc0;
const ELEMENT_AS_ARRAY: u32 = 0x00db_acd0;
/// The error text builder: constructor (`00cb1600`, `this` = the builder, `(buffer, size)`),
/// append (`00cc01c0`, `(text)`) and destructor (`00cc00e0`). The sink is the object whose
/// pointer is in the global `0x01268360`; its slot `0xc` takes `(level, id, text, file, line)`.
const ERROR_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const ERROR_BUILDER_APPEND: u32 = 0x00cc_01c0;
const ERROR_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const ERROR_SINK: u32 = 0x0126_8360;
/// The source file name (".\Tagfile\Binary\hkBinaryTagfileReader.cpp"), the id every parse
/// error but "corrupt file" is reported with, and the vtable the reader class constructor
/// stores.
const SOURCE_FILE: u32 = 0x010d_6cf8;
const PARSE_ERROR_ID: u32 = 0x2ab6_036f;
const TAGFILE_READER_VTABLE: u32 = 0x010d_6ce8;
/// The first string table entry (the address of a zero byte in the exe's data).
const EMPTY_STRING: u32 = 0x0101_1584;
/// The "not owned" bit of a capacity word, and the capacity bits.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator at `router + offset` (`0x10` or `0xc`).
fn router_allocator_at(e: &mut Engine, offset: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + offset)
}

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    router_allocator_at(e, 0x10)
}

/// The inlined "reserve at least", once `allocator` is known: grows the array (data, size,
/// capacity word) to the larger of `wanted` and twice its capacity (element size `element`)
/// when its capacity is below `wanted` (signed).
fn grow_to(e: &mut Engine, allocator: u32, array: u32, wanted: i32, element: u32) {
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let target = capacity.wrapping_add(capacity).max(wanted);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target as u32, element],
        );
    }
}

/// The inlined "reserve at least" with the long-lived allocator (the router is asked first).
fn reserve_at_least(e: &mut Engine, array: u32, wanted: i32, element: u32) {
    let allocator = router_allocator(e);
    grow_to(e, allocator, array, wanted, element);
}

/// The inlined array push of one word: asks the router for its allocator, grows the array by
/// `_reserveMore` (element size 4) when it is full, stores `value` at the end and bumps the
/// size.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let allocator = router_allocator(e);
    let size = e.mem.u32(array + 4);
    if size == e.mem.u32(array + 8) & CAPACITY_MASK {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The inlined release of a local array: asks the router for the allocator at `router +
/// allocator_offset`, resets the size to zero and, when the capacity word does not mark the
/// data as not owned, frees it through the allocator's slot `0x10` with `capacity * element`
/// bytes. The data and capacity words are left as they are.
fn release_scratch_array(e: &mut Engine, allocator_offset: u32, array: u32, element: u32) {
    let allocator = router_allocator_at(e, allocator_offset);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// The inlined release of one of the reader's own arrays: like [`release_scratch_array`] with
/// the long-lived allocator and 4-byte elements, then the array is left empty (data 0,
/// capacity word `0x80000000`).
fn release_array(e: &mut Engine, array: u32) {
    release_scratch_array(e, 0x10, array, 4);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// Reads one byte from the archive through `readArrayGeneric(buffer, 1, 1)`.
fn read_byte(e: &mut Engine, this: u32) -> u8 {
    e.with_stack(4, |e, buffer| {
        e.call(READ_ARRAY, &args![this, buffer, 1u32, 1u32]);
        e.mem.u8(buffer.addr())
    })
}

/// The variable-length integer both integer readers share. The first byte holds the sign in
/// bit 0, the low six value bits in bits 1 to 6 and a continuation flag in bit 7; each
/// following byte holds seven more value bits (shifted left by 6, 13, 20, ... through the
/// CRT's 64-bit shift) and a continuation flag in bit 7. A negative number is negated as a
/// 64-bit value.
fn read_variable_integer(e: &mut Engine, this: u32) -> u64 {
    let first = read_byte(e, this);
    let negative = first & 1 != 0;
    let mut value = u64::from((first >> 1) & 0x3f);
    let mut shift = 6u32;
    let mut byte = first;
    while byte & 0x80 != 0 {
        byte = read_byte(e, this);
        let piece = e
            .call(SHIFT_LEFT_64, &args![u64::from(byte & 0x7f), shift])
            .u64();
        value |= piece;
        shift = shift.wrapping_add(7);
    }
    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

// Translated from 00dc5bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A reader member that reads one 32-bit float (`this` = the reader): `readArrayGeneric(buffer,
/// 4, 1)` into a local, answered in `ST0` after the `FLD` that quiets a signalling NaN.
pub fn read_float(e: &mut Engine, this: u32) -> f32 {
    e.with_stack(4, |e, buffer| {
        e.call(READ_ARRAY, &args![this, buffer, 4u32, 1u32]);
        x87_quiet(e.mem.f32(buffer.addr()))
    })
}

// Translated from 00dc5bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readInt<int>` (Xbox PDB): reads a variable-length integer (see
/// [`read_variable_integer`]) from the reader `this` and answers its low 32 bits.
pub fn read_int_32(e: &mut Engine, this: u32) -> u32 {
    read_variable_integer(e, this) as u32
}

// Translated from 00dc5c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 64-bit form of the reader's variable-length integer: the same bytes as
/// [`read_int_32`], answered whole in `EDX:EAX`.
pub fn read_int_64(e: &mut Engine, this: u32) -> u64 {
    read_variable_integer(e, this)
}

// Translated from 00dc5d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryTagfileReader::hkBinaryTagfileReader` (Xbox PDB): stores the 16-bit word 1 at `this
/// + 6` and the vtable `0x010d6ce8`; answers `this`.
pub fn tagfile_reader_construct(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this + 6, 1);
    e.mem.set_u32(this, TAGFILE_READER_VTABLE);
    this
}

// Translated from 00dc5d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryTagfileReader::load` (Xbox PDB), virtual slot `0xc` of the vtable `0x010d6ce8`:
/// `ECX` is not read (`_unused_0`); the stack words are `out` (a data object handle), the input
/// `stream` and the `builder` object. Builds a 0x3c-byte reader on its stack (constructor,
/// `readObjectTree(out)`, destructor) and answers `out`.
pub fn tagfile_load(e: &mut Engine, _unused_0: u32, out: u32, stream: u32, builder: u32) -> u32 {
    e.with_stack(0x3c, |e, reader| {
        let reader = reader.addr();
        reader_construct(e, reader, stream, builder);
        read_object_tree(e, reader, out);
        reader_destruct(e, reader);
    });
    out
}

// Translated from 00dc5d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readString` (Xbox PDB), `this` = the reader. Reads an integer; when it is positive
/// it is a length: a block of length + 1 bytes comes from `easyAlloc` with the router's
/// allocator, the text is read into it, a zero byte ends it, the pointer is pushed on the
/// string table (the array at `this + 0x24`) and answered. Otherwise the negated integer is an
/// index into the string table and its entry is answered.
pub fn read_string(e: &mut Engine, this: u32) -> u32 {
    let length = read_int_32(e, this) as i32;
    if length > 0 {
        let allocator = router_allocator(e);
        let size = (length as u32).wrapping_add(1);
        let text = e.call(EASY_ALLOC, &args![allocator, size]).u32();
        e.call(READ_RAW, &args![this, text, length as u32]);
        e.mem.set_u8(text.wrapping_add(length as u32), 0);
        push_word(e, this + 0x24, text);
        text
    } else {
        let table = e.mem.u32(this + 0x24);
        e.mem
            .u32(table.wrapping_sub((length as u32).wrapping_mul(4)))
    }
}

// Translated from 00dc5e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readBitfield` (Xbox PDB), `this` = the reader, stack words `bit_count` and `array`
/// (a byte array). Rounds the bit count up to a multiple of 8 (`padded`), reserves the array
/// to `padded` bytes and sets its size to `padded`, reads `padded / 8` bytes into a local
/// buffer and expands them, lowest bit first, to one 0/1 byte per bit; then reserves to
/// `bit_count` and sets the size to `bit_count`. The exe's buffer holds 0x3c bytes; more is
/// not caught.
pub fn read_bitfield(e: &mut Engine, this: u32, bit_count: i32, array: u32) {
    let padded = bit_count.wrapping_add(7) & !7;
    let byte_count = padded.wrapping_add((padded >> 31) & 7) >> 3;
    reserve_at_least(e, array, padded, 1);
    e.mem.set_u32(array + 4, padded as u32);
    e.with_stack(0x44, |e, frame| {
        let buffer = frame.addr() + 4;
        e.call(READ_ARRAY, &args![this, buffer, 1u32, byte_count as u32]);
        for index in 0..byte_count {
            let index = index as u32;
            let byte = e.mem.u8(buffer + index);
            let data = e.mem.u32(array);
            for bit in 0..8u32 {
                e.mem
                    .set_u8(data.wrapping_add(index * 8 + bit), (byte >> bit) & 1);
            }
        }
    });
    reserve_at_least(e, array, bit_count, 1);
    e.mem.set_u32(array + 4, bit_count as u32);
}

// Translated from 00dc5f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::Reader` (Xbox PDB), `this` = the reader (0x3c bytes), stack words `stream` and
/// `builder`. Constructs the archive base (`stream`, a dword that is `this` with its low byte
/// cleared), stores `builder` at `+0x10` and zero at `+0x14`, makes the three empty arrays at
/// `+0x18` (classes), `+0x24` (strings) and `+0x30` (objects), pushes a null onto the classes,
/// and the empty string `0x01011584` and a null onto the strings. Answers `this`.
pub fn reader_construct(e: &mut Engine, this: u32, stream: u32, builder: u32) -> u32 {
    e.call(ARCHIVE_CONSTRUCT, &args![this, stream, this & 0xffff_ff00]);
    e.mem.set_u32(this + 0x10, builder);
    e.mem.set_u32(this + 0x14, 0);
    for array in [0x18u32, 0x24, 0x30] {
        e.mem.set_u32(this + array, 0);
        e.mem.set_u32(this + array + 4, 0);
        e.mem.set_u32(this + array + 8, NOT_OWNED);
    }
    push_word(e, this + 0x18, 0);
    push_word(e, this + 0x24, EMPTY_STRING);
    push_word(e, this + 0x24, 0);
    this
}

// Translated from 00dc6020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::~Reader` (Xbox PDB), `this` = the reader. Deletes the object at `+0x14` through
/// its virtual slot 0 with the flag 1 when there is one; frees the strings from index 2 on
/// with `easyFree`; releases the object array (`+0x30`), the string table (`+0x24`) and the
/// class array (`+0x18`); then runs the archive base destructor.
pub fn reader_destruct(e: &mut Engine, this: u32) {
    let object = e.mem.u32(this + 0x14);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
    if e.mem.u32(this + 0x28) as i32 > 2 {
        let mut index = 2u32;
        loop {
            let table = e.mem.u32(this + 0x24);
            let text = e.mem.u32(table.wrapping_add(index * 4));
            let allocator = router_allocator(e);
            e.call(EASY_FREE, &args![allocator, text]);
            index += 1;
            if index as i32 >= e.mem.u32(this + 0x28) as i32 {
                break;
            }
        }
    }
    release_array(e, this + 0x30);
    release_array(e, this + 0x24);
    release_array(e, this + 0x18);
    e.call(ARCHIVE_DESTRUCT, &args![this]);
}

/// The inlined start shared by `readObject` and the compound-array case of `_readArrayItems`:
/// builds the two scratch arrays at `bits` (bytes) and `members` (0x14-byte records), each
/// 12 bytes followed by a word that remembers the router's bump pointer. The bits array takes
/// 0x80 bytes carved from the router's bump pointer (`router + 0`), capacity word
/// `0x80000080`, and `readBitfield(total)` fills it; the members array takes `total * 0x14`
/// bytes rounded up to 0x80, with its records zeroed (a null address is skipped), size `total`
/// and capacity word `total | 0x80000000`; then the class's all-member-info fills it.
fn begin_member_arrays(e: &mut Engine, this: u32, class: u32, total: u32, bits: u32, members: u32) {
    e.mem.set_u32(bits, 0);
    e.mem.set_u32(bits + 4, 0);
    e.mem.set_u32(bits + 8, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let block = e.mem.u32(router);
    e.mem.set_u32(router, block.wrapping_add(0x80));
    e.mem.set_u32(bits, block);
    e.mem.set_u32(bits + 0xc, block);
    e.mem.set_u32(bits + 8, 0x8000_0080);
    read_bitfield(e, this, total as i32, bits);
    e.mem.set_u32(members, 0);
    e.mem.set_u32(members + 4, 0);
    e.mem.set_u32(members + 8, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let block = e.mem.u32(router);
    let rounded = total.wrapping_mul(20).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(block));
    let used = e.mem.u32(members + 4);
    e.mem.set_u32(members + 8, total | NOT_OWNED);
    let remaining = (total as i32).wrapping_sub(used as i32);
    e.mem.set_u32(members, block);
    e.mem.set_u32(members + 0xc, block);
    let mut record = block.wrapping_add(used.wrapping_mul(20));
    for _ in 0..remaining {
        if record != 0 {
            for word in 0..5u32 {
                e.mem.set_u32(record + word * 4, 0);
            }
        }
        record = record.wrapping_add(20);
    }
    e.mem.set_u32(members + 4, total);
    e.call(DATA_CLASS_ALL_MEMBER_INFO, &args![class, members]);
}

/// The inlined end shared by the same two functions, for one scratch array (`array`, with its
/// remembered bump pointer at `array + 0xc`): when the array's data is still at that pointer
/// the size is reset to 0, the router's bump pointer is set back to it, the array is released
/// through the router's allocator (see [`release_scratch_array`]) and left empty.
fn end_member_array(e: &mut Engine, array: u32, element: u32) {
    let saved = e.mem.u32(array + 0xc);
    if saved == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    release_scratch_array(e, 0x10, array, element);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00dc6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::_readArrayItems` (Xbox PDB), `this` = the reader, stack words `array` (the address
/// of a `hkDataArray` handle) and `count`; reads `count` items of the array's type
/// (`hkDataArray::getType`, 1 to 10; other values do nothing) into it:
/// 1 bytes: a temporary block from the router's `+0xc` allocator (slot `0xc` takes the
///   requested size in a local and writes back the granted size), one `readArrayGeneric`, then
///   the bytes go to the array's set-bytes; the block is released again.
/// 2 integers: one 32-bit integer per element.
/// 3 floats: the same with 4-byte elements and the array's set-floats.
/// 4 vectors: an integer `n` first, then per element `n` floats read into a zeroed
///   four-float local and set as a 4-vector.
/// 6 and 7 vectors of `getType * 4 - 12` floats, one per element.
/// 8 objects: `readObject(-1)` per element.
/// 9 structs: sets the array size, takes the array's class, builds the scratch arrays with
///   [`begin_member_arrays`] and, for every member whose bit is set, swizzles the member out
///   of the array and reads its items (nested arrays when the member's record has a non-zero
///   dimension word), then tears the scratch arrays down.
/// 10 strings: one `readString` per element.
/// The exe keeps its locals in a 0xac-byte frame; the frame here has the same layout.
pub fn read_array_items(e: &mut Engine, this: u32, array: u32, count: i32) {
    let kind = e.call(ARRAY_GET_TYPE, &args![array]).u32();
    let case = kind.wrapping_sub(1);
    if case > 9 {
        return;
    }
    e.with_stack(0xac, |e, frame| {
        let f = frame.addr();
        match case {
            0 => {
                e.mem.set_u32(f, 0);
                e.mem.set_u32(f + 4, 0);
                e.mem.set_u32(f + 8, NOT_OWNED);
                let temporary = router_allocator_at(e, 0xc);
                let (data, size, capacity) = if count == 0 {
                    (0, count as u32, NOT_OWNED)
                } else {
                    e.mem.set_u32(f + 0x10, count as u32);
                    let data = e.vcall(temporary, 0xc, &args![f + 0x10]).u32();
                    let granted = e.mem.u32(f + 0x10);
                    (
                        data,
                        granted,
                        if granted == 0 { NOT_OWNED } else { granted },
                    )
                };
                e.mem.set_u32(f, data);
                e.mem.set_u32(f + 4, size);
                e.mem.set_u32(f + 8, capacity);
                let allocator = router_allocator_at(e, 0xc);
                grow_to(e, allocator, f, count, 1);
                let data = e.mem.u32(f);
                e.call(READ_ARRAY, &args![this, data, 1u32, count as u32]);
                let data = e.mem.u32(f);
                e.call(ARRAY_SET_BYTES, &args![array, data, count as u32]);
                release_scratch_array(e, 0xc, f, 1);
            }
            1 => {
                for index in 0..count {
                    let value = read_int_32(e, this);
                    let element = e
                        .call(ARRAY_ELEMENT, &args![array, f + 0x3c, index as u32])
                        .u32();
                    e.call(ELEMENT_SET_INT, &args![element, value]);
                }
            }
            2 => {
                e.mem.set_u32(f, 0);
                e.mem.set_u32(f + 4, 0);
                e.mem.set_u32(f + 8, NOT_OWNED);
                let temporary = router_allocator_at(e, 0xc);
                let (data, size, capacity) = if count == 0 {
                    (0, count as u32, NOT_OWNED)
                } else {
                    e.mem.set_u32(f + 0x10, (count as u32).wrapping_mul(4));
                    let data = e.vcall(temporary, 0xc, &args![f + 0x10]).u32();
                    let granted = e.mem.u32(f + 0x10) as i32;
                    let elements = granted.wrapping_add((granted >> 31) & 3) >> 2;
                    (
                        data,
                        elements as u32,
                        if elements == 0 {
                            NOT_OWNED
                        } else {
                            elements as u32
                        },
                    )
                };
                e.mem.set_u32(f, data);
                e.mem.set_u32(f + 4, size);
                e.mem.set_u32(f + 8, capacity);
                let allocator = router_allocator_at(e, 0xc);
                grow_to(e, allocator, f, count, 4);
                e.mem.set_u32(f + 4, count as u32);
                let data = e.mem.u32(f);
                e.call(READ_ARRAY, &args![this, data, 4u32, count as u32]);
                let data = e.mem.u32(f);
                let size = e.mem.u32(f + 4);
                e.call(ARRAY_SET_FLOATS, &args![array, data, size]);
                release_scratch_array(e, 0xc, f, 4);
            }
            3 => {
                let width = read_int_32(e, this);
                e.mem.set_u32(f + 0x14, width);
                for word in 0..4u32 {
                    e.mem.set_u32(f + 0x1c + word * 4, 0);
                }
                for index in 0..count {
                    e.call(READ_ARRAY, &args![this, f + 0x1c, 4u32, width]);
                    let element = e
                        .call(ARRAY_ELEMENT, &args![array, f + 0x4c, index as u32])
                        .u32();
                    e.call(ELEMENT_SET_VECTOR, &args![element, f + 0x1c, 4u32]);
                }
            }
            5 | 6 => {
                let floats = e
                    .call(ARRAY_GET_TYPE, &args![array])
                    .u32()
                    .wrapping_mul(4)
                    .wrapping_sub(12) as i32;
                for index in 0..count {
                    for float in 0..floats {
                        e.call(READ_ARRAY, &args![this, f + 0x14, 4u32, 1u32]);
                        let bits = x87_quiet_bits(e.mem.u32(f + 0x14));
                        e.mem.set_u32(f + 0x6c + float as u32 * 4, bits);
                    }
                    let element = e
                        .call(ARRAY_ELEMENT, &args![array, f + 0x5c, index as u32])
                        .u32();
                    e.call(ELEMENT_SET_VECTOR, &args![element, f + 0x6c, floats as u32]);
                }
            }
            7 => {
                for index in 0..count {
                    let object = read_object(e, this, f + 0x38, -1);
                    let element = e
                        .call(ARRAY_ELEMENT, &args![array, f + 0x64, index as u32])
                        .u32();
                    e.call(ELEMENT_SET_OBJECT, &args![element, object]);
                    e.call(DATA_OBJECT_DESTRUCT, &args![f + 0x38]);
                }
            }
            8 => {
                e.call(ARRAY_SET_SIZE, &args![array, count as u32]);
                e.call(ARRAY_GET_CLASS, &args![array, f + 0x10]);
                let total = e.call(DATA_CLASS_NUM_MEMBERS, &args![f + 0x10]).u32();
                begin_member_arrays(e, this, f + 0x10, total, f + 0x1c, f);
                for member in 0..total as i32 {
                    let info = e.mem.u32(f).wrapping_add(member as u32 * 20);
                    let flags = e.mem.u32(f + 0x1c);
                    if e.mem.u8(flags.wrapping_add(member as u32)) != 0 {
                        let key = e.mem.u32(info);
                        e.call(ARRAY_SWIZZLE_MEMBER, &args![array, f + 0x18, key]);
                        let dimension = e.mem.u32(info + 0x10);
                        if dimension == 0 {
                            read_array_items(e, this, f + 0x18, count);
                        } else {
                            for index in 0..count {
                                let element = e
                                    .call(ARRAY_ELEMENT, &args![f + 0x18, f + 0x44, index as u32])
                                    .u32();
                                e.call(ELEMENT_AS_ARRAY, &args![element, f + 0x34]);
                                read_array_items(e, this, f + 0x34, dimension as i32);
                                e.call(HANDLE_RELEASE, &args![f + 0x34]);
                            }
                        }
                        e.call(HANDLE_RELEASE, &args![f + 0x18]);
                    }
                }
                end_member_array(e, f, 0x14);
                end_member_array(e, f + 0x1c, 1);
                e.call(DATA_CLASS_DESTRUCT, &args![f + 0x10]);
            }
            9 => {
                for index in 0..count {
                    let text = read_string(e, this);
                    let element = e
                        .call(ARRAY_ELEMENT, &args![array, f + 0x54, index as u32])
                        .u32();
                    e.call(ELEMENT_SET_STRING, &args![element, text]);
                }
            }
            _ => {}
        }
    });
}

// Translated from 00dc6720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readBinaryValue` (Xbox PDB), `this` = the reader, stack words `value_first`,
/// `value_second` (the two words of a `hkDataObject::Value` passed by value) and `info` (a
/// 0x14-byte member record: word 2 holds the type flags, word 3 the class, word 4 the array
/// dimension). By the flags:
/// bit 5 (`0x20`): a `hkDataObject` handle for `value_first`, the builder's slot `0x14` made
///   an array handle from `(object, info)`, `_readArrayItems(array, info[4])`;
/// bit 4 (`0x10`): a copy of the record, an integer `n` read, the value's type (`0x19` with a
///   zero class word reads one more integer and takes that entry of the class array as the
///   class), the same array handle made, its size set to `n` and `_readArrayItems(array, n)`;
/// otherwise by the type 1 to 10: a byte, a 64-bit integer, a float, a vector of
///   `type * 4 - 12` floats (types 4, 6, 7), nothing (5), an object (`readObject(-1)`, types 8
///   and 9) or a string (10) read and put into the value.
/// The exe keeps its locals (and the value words, which it stores over its own argument slots)
/// in the frame; the frame here is a fresh 0x68-byte block with the same layout.
pub fn read_binary_value(
    e: &mut Engine,
    this: u32,
    value_first: u32,
    value_second: u32,
    info: u32,
) {
    let flags = e.mem.u32(info + 8);
    e.with_stack(0x68, |e, frame| {
        let f = frame.addr();
        let value = f + 0x5c;
        let slot = f + 0x64;
        e.mem.set_u32(value, value_first);
        e.mem.set_u32(value + 4, value_second);
        if flags & 0x20 != 0 {
            e.call(DATA_OBJECT_CONSTRUCT, &args![f, value_first]);
            let builder = e.mem.u32(this + 0x10);
            let array = e.vcall(builder, 0x14, &args![f, info]).u32();
            e.call(HANDLE_SET, &args![slot, array]);
            let dimension = e.mem.u32(info + 0x10);
            read_array_items(e, this, slot, dimension as i32);
            e.call(HANDLE_RELEASE, &args![slot]);
            e.call(DATA_OBJECT_DESTRUCT, &args![f]);
            return;
        }
        if flags & 0x10 != 0 {
            for word in 0..5u32 {
                let copied = e.mem.u32(info + word * 4);
                e.mem.set_u32(f + 4 + word * 4, copied);
            }
            let items = read_int_32(e, this);
            let kind = e.call(VALUE_GET_TYPE, &args![value]).u32();
            if kind == 0x19 && e.mem.u32(info + 0xc) == 0 {
                let index = read_int_32(e, this);
                let classes = e.mem.u32(this + 0x18);
                let class = e.mem.u32(classes.wrapping_add(index.wrapping_mul(4)));
                e.mem.set_u32(f + 0x10, class);
            }
            e.call(DATA_OBJECT_CONSTRUCT, &args![f, value_first]);
            let builder = e.mem.u32(this + 0x10);
            let array = e.vcall(builder, 0x14, &args![f, f + 4]).u32();
            e.call(HANDLE_SET, &args![slot, array]);
            e.call(ARRAY_SET_SIZE, &args![slot, items]);
            read_array_items(e, this, slot, items as i32);
            e.call(HANDLE_RELEASE, &args![slot]);
            e.call(DATA_OBJECT_DESTRUCT, &args![f]);
            return;
        }
        match flags.wrapping_sub(1) {
            0 => {
                e.call(READ_ARRAY, &args![this, slot, 1u32, 1u32]);
                let byte = e.mem.u8(slot);
                e.call(VALUE_SET_BYTE, &args![value, u32::from(byte)]);
            }
            1 => {
                let number = read_int_64(e, this);
                e.call(VALUE_SET_INT_64, &args![value, number]);
            }
            2 => {
                let number = read_float(e, this);
                e.mem.set_f32(slot, number);
                let bits = x87_quiet_bits(e.mem.u32(slot));
                e.call(VALUE_SET_FLOAT, &args![value, bits]);
            }
            3 | 5 | 6 => {
                let floats = flags.wrapping_mul(4).wrapping_sub(12) as i32;
                for float in 0..floats {
                    e.call(READ_ARRAY, &args![this, slot, 4u32, 1u32]);
                    let bits = x87_quiet_bits(e.mem.u32(slot));
                    e.mem.set_u32(f + 0x18 + float as u32 * 4, bits);
                }
                e.call(VALUE_SET_VECTOR, &args![value, f + 0x18, floats as u32]);
            }
            7 | 8 => {
                read_object(e, this, slot, -1);
                e.call(VALUE_SET_OBJECT, &args![value, slot]);
                e.call(DATA_OBJECT_DESTRUCT, &args![slot]);
            }
            9 => {
                let text = read_string(e, this);
                e.call(VALUE_SET_STRING, &args![value, text]);
            }
            _ => {}
        }
    });
}

/// Builds the error text in `builder` (12 bytes) over `buffer` (0x200 bytes), appends `text`
/// and reports `(level, id, text buffer, source file, line)` to the error sink (slot `0xc` of
/// the object in the global `0x01268360`); answers what the sink answers. The caller destroys
/// the builder.
fn report_error(
    e: &mut Engine,
    builder: u32,
    buffer: u32,
    text: u32,
    level: u32,
    id: u32,
    line: u32,
) -> u32 {
    e.call(ERROR_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
    e.call(ERROR_BUILDER_APPEND, &args![builder, text]);
    let sink = e.mem.u32(ERROR_SINK);
    e.vcall(sink, 0xc, &args![level, id, buffer, SOURCE_FILE, line])
        .u32()
}

// Translated from 00dc6970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readObject` (Xbox PDB), `this` = the reader, stack words `out` (a data object
/// handle) and `tag` (-1: read the tag as an integer first); answers `out`.
/// Tag 5: an integer indexes the object array (`+0x30`) and `out` is constructed from that
/// entry. Tag 6: `out` is constructed null. Otherwise an integer indexes the class array
/// (`+0x18`), a class handle is made, the builder's slot `0x10` creates the object from it
/// and a handle is made for it; tag 4 pushes the object onto the object array, tag 3 does
/// nothing more, any other tag reports "corrupt file" (level 3, id `0x484994d5`, line `0x178`;
/// a non-zero answer from the sink is an `INT3` in the exe, a panic here). Then the member
/// bits and records are built ([`begin_member_arrays`]) and for every member whose bit is set
/// the object's `operator[](record key)` value is filled by `readBinaryValue`; `out` is copied
/// from the object handle and the scratch arrays are torn down.
/// The frame (0x238 bytes) has the exe's layout.
pub fn read_object(e: &mut Engine, this: u32, out: u32, tag: i32) -> u32 {
    let tag = if tag == -1 {
        read_int_32(e, this) as i32
    } else {
        tag
    };
    if tag == 5 {
        let index = read_int_32(e, this);
        let objects = e.mem.u32(this + 0x30);
        let object = e.mem.u32(objects.wrapping_add(index.wrapping_mul(4)));
        e.call(DATA_OBJECT_CONSTRUCT, &args![out, object]);
        return out;
    }
    if tag == 6 {
        e.call(DATA_OBJECT_CONSTRUCT, &args![out, 0u32]);
        return out;
    }
    e.with_stack(0x238, |e, frame| {
        let f = frame.addr();
        let index = read_int_32(e, this);
        let classes = e.mem.u32(this + 0x18);
        let class = e.mem.u32(classes.wrapping_add(index.wrapping_mul(4)));
        e.call(DATA_CLASS_CONSTRUCT, &args![f, class]);
        let total = e.call(DATA_CLASS_NUM_MEMBERS, &args![f]).u32();
        let builder = e.mem.u32(this + 0x10);
        let created = e.vcall(builder, 0x10, &args![f]).u32();
        e.call(DATA_OBJECT_CONSTRUCT, &args![f + 4, created]);
        match tag {
            3 => {}
            4 => {
                let object = e.mem.u32(f + 4);
                push_word(e, this + 0x30, object);
            }
            _ => {
                let answer =
                    report_error(e, f + 0x2c, f + 0x38, 0x010d_6d24, 3, 0x4849_94d5, 0x178);
                if answer != 0 {
                    panic!("INT3 at 00dc6a74");
                }
                e.call(ERROR_BUILDER_DESTRUCT, &args![f + 0x2c]);
            }
        }
        begin_member_arrays(e, this, f, total, f + 0x18, f + 8);
        for member in 0..total as i32 {
            let info = e.mem.u32(f + 8).wrapping_add(member as u32 * 20);
            let flags = e.mem.u32(f + 0x18);
            if e.mem.u8(flags.wrapping_add(member as u32)) != 0 {
                let key = e.mem.u32(info);
                let proxy = e
                    .call(DATA_OBJECT_INDEX, &args![f + 4, f + 0x2c, key])
                    .u32();
                let first = e.mem.u32(proxy);
                let second = e.mem.u32(proxy + 4);
                read_binary_value(e, this, first, second, info);
            }
        }
        e.call(DATA_OBJECT_COPY, &args![out, f + 4]);
        end_member_array(e, f + 8, 0x14);
        end_member_array(e, f + 0x18, 1);
        e.call(DATA_OBJECT_DESTRUCT, &args![f + 4]);
        e.call(DATA_CLASS_DESTRUCT, &args![f]);
    });
    out
}

// Translated from 00dc6c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readClass` (Xbox PDB), `this` = the reader; answers 1 when the builder's status
/// check says to stop and 0 otherwise. Reads the class name (a string) and version (an
/// integer) into a description record, asks the builder's slot `0x18` for a status pointer
/// (the pointed-to word 0: release the member list and answer 1); reads an integer parent
/// index (negative or a null class entry give a null parent, otherwise the parent is the
/// result of the class entry's virtual slot 8); reads the member count and, per member, name
/// (string), flags (integer), an integer when flag `0x20` is set and a string when the low
/// four flag bits are 8 or 9 (given to the builder's slot `0x24` when non-null); the builder's
/// slot `0xc` makes the class from the description and it is pushed onto the class array
/// (`+0x18`). The member list is a local array of 0x10-byte records.
pub fn read_class(e: &mut Engine, this: u32) -> u32 {
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        let members = f + 0x10;
        e.mem.set_u32(members, 0);
        e.mem.set_u32(members + 4, 0);
        e.mem.set_u32(members + 8, NOT_OWNED);
        let name = read_string(e, this);
        e.mem.set_u32(f + 4, name);
        let version = read_int_32(e, this);
        e.mem.set_u32(f + 8, version);
        let builder = e.mem.u32(this + 0x10);
        let status = e.vcall(builder, 0x18, &args![f]).u32();
        if e.mem.u32(status) == 0 {
            release_scratch_array(e, 0x10, members, 0x10);
            return 1;
        }
        let parent_index = read_int_32(e, this) as i32;
        let parent = if parent_index < 0 {
            0
        } else {
            let classes = e.mem.u32(this + 0x18);
            let parent_class = e
                .mem
                .u32(classes.wrapping_add((parent_index as u32).wrapping_mul(4)));
            if parent_class == 0 {
                0
            } else {
                e.vcall(parent_class, 8, &args![]).u32()
            }
        };
        e.mem.set_u32(f + 0xc, parent);
        let count = read_int_32(e, this) as i32;
        reserve_at_least(e, members, count, 0x10);
        e.mem.set_u32(members + 4, count as u32);
        if count > 0 {
            let mut index = 0i32;
            let mut offset = 0u32;
            loop {
                let record = e.mem.u32(members).wrapping_add(offset);
                let member_name = read_string(e, this);
                e.mem.set_u32(record, member_name);
                let member_flags = read_int_32(e, this);
                e.mem.set_u32(record + 4, member_flags);
                let extra = if member_flags & 0x20 != 0 {
                    read_int_32(e, this)
                } else {
                    0
                };
                e.mem.set_u32(record + 8, extra);
                let kind = e.mem.u32(record + 4) & 0xf;
                let text = if kind == 9 || kind == 8 {
                    read_string(e, this)
                } else {
                    0
                };
                e.mem.set_u32(record + 0xc, text);
                if text != 0 {
                    let builder = e.mem.u32(this + 0x10);
                    e.vcall(builder, 0x24, &args![text]);
                }
                index += 1;
                offset = offset.wrapping_add(0x10);
                if index >= e.mem.u32(members + 4) as i32 {
                    break;
                }
            }
        }
        let builder = e.mem.u32(this + 0x10);
        let class = e.vcall(builder, 0xc, &args![f + 4]).u32();
        push_word(e, this + 0x18, class);
        release_scratch_array(e, 0x10, members, 0x10);
        0
    })
}

// Translated from 00dc6e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Reader::readObjectTree` (Xbox PDB), `this` = the reader, stack word `out` (a data object
/// handle); answers `out`. Reads two 32-bit words and requires `0xcab00d1e`, then
/// `0xd011face` ("This does not look like a binary tagfile (magic number mismatch)" is
/// reported otherwise, level 1, id `0x2ab6036f`, line `0x1c5`, and `out` is constructed null).
/// Then reads tags until -1 or an unknown one: tag 1 reads a version integer (non-zero is
/// reported as "Unrecognised tagfile version." and ends, line `0x1d9`), tag 2 reads a class
/// (answer 1 ends), tags 3, 4 and 6 read an object into `out` and end with it, tag 5 ends.
/// Every other end constructs `out` null.
pub fn read_object_tree(e: &mut Engine, this: u32, out: u32) -> u32 {
    e.with_stack(0x214, |e, frame| {
        let f = frame.addr();
        e.call(READ_ARRAY, &args![this, f + 0x10, 4u32, 1u32]);
        e.call(READ_ARRAY, &args![this, f, 4u32, 1u32]);
        if e.mem.u32(f + 0x10) == 0xcab0_0d1e && e.mem.u32(f) == 0xd011_face {
            let mut tag = read_int_32(e, this) as i32;
            loop {
                if tag == -1 {
                    break;
                }
                let case = (tag as u32).wrapping_sub(1);
                if case > 5 {
                    break;
                }
                match case {
                    0 => {
                        let version = read_int_32(e, this);
                        if version != 0 {
                            report_error(e, f + 4, f + 0x14, 0x010d_6d7c, 1, PARSE_ERROR_ID, 0x1d9);
                            e.call(ERROR_BUILDER_DESTRUCT, &args![f + 4]);
                            break;
                        }
                    }
                    1 => {
                        if read_class(e, this) == 1 {
                            break;
                        }
                    }
                    2 | 3 | 5 => {
                        read_object(e, this, out, tag);
                        return out;
                    }
                    _ => break,
                }
                tag = read_int_32(e, this) as i32;
            }
        } else {
            report_error(e, f + 4, f + 0x14, 0x010d_6d38, 1, PARSE_ERROR_ID, 0x1c5);
            e.call(ERROR_BUILDER_DESTRUCT, &args![f + 4]);
        }
        e.call(DATA_OBJECT_CONSTRUCT, &args![out, 0u32]);
        out
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_5bb0, read_float(u32) -> f32),
        entry!(0x00dc_5bd0, read_int_32(u32) -> u32),
        entry!(0x00dc_5c80, read_int_64(u32) -> u64),
        entry!(0x00dc_5d30, tagfile_reader_construct(u32) -> u32),
        entry!(0x00dc_5d50, tagfile_load(u32, u32, u32, u32) -> u32),
        entry!(0x00dc_5d90, read_string(u32) -> u32),
        entry!(0x00dc_5e10, read_bitfield(u32, i32, u32)),
        entry!(0x00dc_5f30, reader_construct(u32, u32, u32) -> u32),
        entry!(0x00dc_6020, reader_destruct(u32)),
        entry!(0x00dc_6100, read_array_items(u32, u32, i32)),
        entry!(0x00dc_6720, read_binary_value(u32, u32, u32, u32)),
        entry!(0x00dc_6970, read_object(u32, u32, i32) -> u32),
        entry!(0x00dc_6c90, read_class(u32) -> u32),
        entry!(0x00dc_6e50, read_object_tree(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// Doubles behind the slots of the allocators (`0xc` of the temporary allocator allocates a
    /// block, `0x10` frees one), of the builder object at `reader + 0x10` and of the error
    /// sink, and the other objects' slots.
    const TEMP_ALLOC: u32 = 0x0900_000c;
    const TEMP_FREE: u32 = 0x0900_0010;
    const HEAP_FREE: u32 = 0x0900_0110;
    const BUILD_CLASS: u32 = 0x0900_020c;
    const CREATE_OBJECT: u32 = 0x0900_0210;
    const MAKE_ARRAY: u32 = 0x0900_0214;
    const CHECK_STATUS: u32 = 0x0900_0218;
    const ADD_MEMBER: u32 = 0x0900_0224;
    const SINK: u32 = 0x0900_030c;
    const CLASS_PARENT: u32 = 0x0900_0408;
    const OBJECT_DELETE: u32 = 0x0900_0500;

    type Input = Rc<RefCell<VecDeque<u8>>>;
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// The memory router's block and the objects it points at.
    struct Router {
        block: u32,
        pool: u32,
        heap: u32,
        temporary: u32,
    }

    /// The bytes of a tagfile, built up piece by piece.
    #[derive(Default)]
    struct Stream(Vec<u8>);

    impl Stream {
        /// A variable-length integer: the sign in bit 0 and six value bits in bits 1 to 6 of
        /// the first byte, seven more value bits in each following byte, bit 7 of a byte
        /// saying that another byte follows.
        fn int(mut self, value: i64) -> Self {
            let mut magnitude = value.unsigned_abs();
            let mut bytes = vec![(((magnitude & 0x3f) << 1) as u8) | u8::from(value < 0)];
            magnitude >>= 6;
            while magnitude != 0 {
                let last = bytes.len() - 1;
                bytes[last] |= 0x80;
                bytes.push((magnitude & 0x7f) as u8);
                magnitude >>= 7;
            }
            self.0.extend(bytes);
            self
        }

        fn word(mut self, value: u32) -> Self {
            self.0.extend(value.to_le_bytes());
            self
        }

        fn float(self, value: f32) -> Self {
            self.word(value.to_bits())
        }

        fn byte(mut self, value: u8) -> Self {
            self.0.push(value);
            self
        }

        fn text(mut self, value: &str) -> Self {
            self.0.extend(value.bytes());
            self
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// `__allshl` (`00ec7810`) is an unplaced unit of `engine-fallout`, which this
    /// crate's tests do not link since the engine split: a double with its result
    /// (the 64-bit value shifted left by the count; 0 from a count of 64 on).
    fn allshl_double(e: &mut Engine) {
        e.register(SHIFT_LEFT_64, |_, a| {
            let value = u64::from(a[0]) | (u64::from(a[1]) << 32);
            let count = a[2] as u8;
            (if count >= 0x40 { 0 } else { value << count }).into_ret()
        });
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The calls to any of `addresses`, in order, as `(address, words)`.
    fn ordered(e: &Engine, addresses: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addresses.contains(a))
            .cloned()
            .collect()
    }

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn nothing(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register_double(*address, |_, _| Ret::default());
        }
    }

    /// A double that answers the given values one after the other.
    fn sequence(e: &mut Engine, address: u32, values: &[u32]) {
        let mut queue: VecDeque<u32> = values.iter().copied().collect();
        e.register_double(address, move |_, _| Ret {
            eax: queue.pop_front().expect("the sequence has no more answers"),
            ..Ret::default()
        });
    }

    /// A double that records `count` dwords at argument `arg` of every call and answers
    /// `value`.
    fn watch(e: &mut Engine, address: u32, arg: usize, count: u32, value: u32) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push(dwords(e, a[arg], count));
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        seen
    }

    /// An object whose vtable holds `(offset, target)` slots.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Stores `words` in a fresh block.
    fn words(e: &mut Engine, words: &[u32]) -> u32 {
        let block = e.mem.alloc(words.len() as u32 * 4 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(block + i as u32 * 4, *word);
        }
        block
    }

    fn set_array(e: &mut Engine, array: u32, data: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
    }

    fn dwords(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(address + i * 4)).collect()
    }

    /// Makes the archive reads come from `stream`: `readArrayGeneric` and `readRaw` double.
    fn feed(e: &mut Engine, stream: Stream) -> Input {
        let input: Input = Rc::new(RefCell::new(stream.0.into_iter().collect()));
        let reads = Rc::clone(&input);
        e.register_double(READ_ARRAY, move |e, a| {
            for i in 0..a[2].wrapping_mul(a[3]) {
                let byte = reads.borrow_mut().pop_front().expect("the input is empty");
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        });
        let raw = Rc::clone(&input);
        e.register_double(READ_RAW, move |e, a| {
            for i in 0..a[2] {
                let byte = raw.borrow_mut().pop_front().expect("the input is empty");
                e.mem.set_u8(a[1] + i, byte);
            }
            Ret::default()
        });
        input
    }

    /// Makes `getAllMemberInfo` write the given 0x14-byte records into the array it is given.
    fn member_info(e: &mut Engine, records: Vec<[u32; 5]>) {
        e.register_double(DATA_CLASS_ALL_MEMBER_INFO, move |e, a| {
            let data = e.mem.u32(a[1]);
            for (i, record) in records.iter().enumerate() {
                for (j, word) in record.iter().enumerate() {
                    e.mem.set_u32(data + (i * 5 + j) as u32 * 4, *word);
                }
            }
            Ret::default()
        });
    }

    /// The doubles of everything the unit calls outside itself, doing nothing and answering
    /// 0 unless said otherwise: the router (a bump pool of 0x4000 bytes, a temporary allocator
    /// whose allocation slot hands out a block of the requested size and a heap allocator),
    /// `_reserve`/`_reserveMore` (move the data to a fresh block of the requested size, four
    /// elements more for `_reserveMore`), `easyAlloc`, the builder slots, the error sink
    /// (answering 0) and the data accessors. Reading from the archive is not set up here
    /// ([`feed`]).
    fn externals(e: &mut Engine) -> Router {
        let heap = object_with_slots(e, &[(0x10, HEAP_FREE)]);
        let temporary = object_with_slots(e, &[(0xc, TEMP_ALLOC), (0x10, TEMP_FREE)]);
        let pool = e.mem.alloc(0x4000);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, pool);
        e.mem.set_u32(block + 0xc, temporary);
        e.mem.set_u32(block + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register_double(TEMP_ALLOC, |e, a| {
            let size = e.mem.u32(a[1]);
            Ret {
                eax: e.mem.alloc(size.max(4)),
                ..Ret::default()
            }
        });
        e.register_double(EASY_ALLOC, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        fn grow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
            let block = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for i in 0..e.mem.u32(array + 4) * element {
                let byte = e.mem.u8(old + i);
                e.mem.set_u8(block + i, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
        }
        e.register_double(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & CAPACITY_MASK) + 4;
            grow(e, a[1], capacity, a[2]);
            Ret::default()
        });
        for address in [ARRAY_ELEMENT, DATA_OBJECT_INDEX, ARRAY_SWIZZLE_MEMBER] {
            e.register_double(address, |_, a| Ret {
                eax: a[1],
                ..Ret::default()
            });
        }
        let go_on = e.mem.alloc(4);
        e.mem.set_u32(go_on, 1);
        answer(e, CHECK_STATUS, go_on);
        answer(e, BUILD_CLASS, 0x0c1a_5500);
        answer(e, CREATE_OBJECT, 0x0b0b_0001);
        answer(e, MAKE_ARRAY, 0x0a0a_0001);
        let sink = object_with_slots(e, &[(0xc, SINK)]);
        e.mem.set_u32(ERROR_SINK, sink);
        nothing(
            e,
            &[
                TEMP_FREE,
                HEAP_FREE,
                ADD_MEMBER,
                SINK,
                EASY_FREE,
                ARCHIVE_CONSTRUCT,
                ARCHIVE_DESTRUCT,
                DATA_CLASS_CONSTRUCT,
                DATA_CLASS_DESTRUCT,
                DATA_CLASS_NUM_MEMBERS,
                DATA_CLASS_ALL_MEMBER_INFO,
                DATA_OBJECT_CONSTRUCT,
                DATA_OBJECT_COPY,
                DATA_OBJECT_DESTRUCT,
                VALUE_GET_TYPE,
                VALUE_SET_BYTE,
                VALUE_SET_INT_64,
                VALUE_SET_FLOAT,
                VALUE_SET_VECTOR,
                VALUE_SET_OBJECT,
                VALUE_SET_STRING,
                HANDLE_SET,
                HANDLE_RELEASE,
                ARRAY_GET_TYPE,
                ARRAY_SET_SIZE,
                ARRAY_GET_CLASS,
                ARRAY_SET_BYTES,
                ARRAY_SET_FLOATS,
                ELEMENT_SET_INT,
                ELEMENT_SET_VECTOR,
                ELEMENT_SET_OBJECT,
                ELEMENT_SET_STRING,
                ELEMENT_AS_ARRAY,
                ERROR_BUILDER_CONSTRUCT,
                ERROR_BUILDER_APPEND,
                ERROR_BUILDER_DESTRUCT,
            ],
        );
        Router {
            block,
            pool,
            heap,
            temporary,
        }
    }

    /// A reader object (0x40 bytes) with a builder object at `+0x10` (slots `0xc` build class,
    /// `0x10` create object, `0x14` make array, `0x18` check status, `0x24` add member) and
    /// three empty arrays. Answers `(reader, builder)`.
    fn reader(e: &mut Engine) -> (u32, u32) {
        let builder = object_with_slots(
            e,
            &[
                (0xc, BUILD_CLASS),
                (0x10, CREATE_OBJECT),
                (0x14, MAKE_ARRAY),
                (0x18, CHECK_STATUS),
                (0x24, ADD_MEMBER),
            ],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, builder);
        for array in [0x18u32, 0x24, 0x30] {
            set_array(e, this + array, 0, 0, NOT_OWNED);
        }
        (this, builder)
    }

    #[test]
    fn read_float_reads_four_bytes_and_quiets_a_signalling_nan() {
        let mut e = engine();
        let input = feed(&mut e, Stream::default().float(1.5).word(0x7f80_0001));
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00dc_5bb0, &args![this]).f32(), 1.5);
        assert_eq!(
            e.call(0x00dc_5bb0, &args![this]).f32().to_bits(),
            0x7fc0_0001
        );
        let reads = calls_to(&e, READ_ARRAY);
        assert_eq!(reads.len(), 2);
        assert_eq!((reads[0][0], reads[0][2], reads[0][3]), (this, 4, 1));
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_int_32_reads_a_variable_length_integer() {
        let mut e = engine();
        allshl_double(&mut e);
        let this = e.mem.alloc(0x40);
        let values = [
            0i64,
            1,
            -1,
            31,
            32,
            -32,
            63,
            64,
            -64,
            1000,
            -1000,
            1 << 20,
            0x7fff_ffff,
            -0x8000_0000,
            0x1_2345_6789,
        ];
        for value in values {
            let input = feed(&mut e, Stream::default().int(value));
            assert_eq!(e.call(0x00dc_5bd0, &args![this]).u32(), value as u32);
            assert!(input.borrow().is_empty());
        }
        // The first byte alone: bit 0 is the sign, bits 1 to 6 the value, bit 7 more bytes.
        let input = feed(&mut e, Stream::default().byte(0b0001_0111));
        assert_eq!(e.call(0x00dc_5bd0, &args![this]).i32(), -0b1011);
        assert!(input.borrow().is_empty());
        let reads = calls_to(&e, READ_ARRAY);
        assert_eq!((reads[0][0], reads[0][2], reads[0][3]), (this, 1, 1));
    }

    #[test]
    fn read_int_64_answers_the_whole_value() {
        let mut e = engine();
        allshl_double(&mut e);
        let this = e.mem.alloc(0x40);
        let values = [
            0i64,
            5,
            -5,
            1 << 31,
            1 << 40,
            -(1 << 40),
            0x1234_5678_9abc_def0,
            i64::MAX,
            i64::MIN,
            -1,
        ];
        for value in values {
            let input = feed(&mut e, Stream::default().int(value));
            assert_eq!(e.call(0x00dc_5c80, &args![this]).u64(), value as u64);
            assert!(input.borrow().is_empty());
        }
    }

    #[test]
    fn tagfile_reader_construct_stores_the_halfword_and_the_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xdead_beef);
        assert_eq!(e.call(0x00dc_5d30, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_6ce8);
        assert_eq!(e.mem.u16(this + 4), 0xbeef);
        assert_eq!(e.mem.u16(this + 6), 1);
    }

    #[test]
    fn tagfile_load_builds_a_reader_reads_the_tree_and_destroys_it() {
        let mut e = engine();
        let router = externals(&mut e);
        // A bad magic number: the tree reader reports it and makes an empty object.
        let input = feed(&mut e, Stream::default().word(0xcab0_0d1e).word(0));
        let builders = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&builders);
        e.register_double(ARCHIVE_DESTRUCT, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[0] + 0x10)));
            Ret::default()
        });
        let out = e.mem.alloc(8);
        let result = e.call(
            0x00dc_5d50,
            &args![0xdeadu32, out, 0x1234_5000u32, 0x0b0b_1000u32],
        );
        assert_eq!(result.u32(), out);
        let constructs = calls_to(&e, ARCHIVE_CONSTRUCT);
        assert_eq!(constructs.len(), 1);
        let reader = constructs[0][0];
        assert_eq!(
            constructs[0],
            vec![reader, 0x1234_5000, reader & 0xffff_ff00]
        );
        assert_eq!(*builders.borrow(), vec![(reader, 0x0b0b_1000)]);
        assert!(input.borrow().is_empty());
        assert_eq!(calls_to(&e, SINK).len(), 1);
        assert_eq!(
            ordered(
                &e,
                &[ARCHIVE_CONSTRUCT, DATA_OBJECT_CONSTRUCT, ARCHIVE_DESTRUCT]
            ),
            vec![
                (ARCHIVE_CONSTRUCT, constructs[0].clone()),
                (DATA_OBJECT_CONSTRUCT, vec![out, 0]),
                (ARCHIVE_DESTRUCT, vec![reader]),
            ]
        );
        // The destructor released the string and class arrays (one push grew each, to 4 words).
        let frees = calls_to(&e, HEAP_FREE);
        assert_eq!(frees.len(), 2);
        assert_eq!((frees[0][0], frees[0][2]), (router.heap, 16));
        assert_eq!((frees[1][0], frees[1][2]), (router.heap, 16));
    }

    #[test]
    fn read_string_reads_new_text_or_looks_up_the_string_table() {
        let mut e = engine();
        let router = externals(&mut e);
        let (this, _) = reader(&mut e);
        let table = words(&mut e, &[0x1111, 0x2222, 0x3333]);
        set_array(&mut e, this + 0x24, table, 3, 3);
        let input = feed(&mut e, Stream::default().int(3).text("abc").int(-2).int(0));
        let text = e.call(0x00dc_5d90, &args![this]).u32();
        assert_eq!(e.mem.cstr(text), b"abc");
        assert_eq!(calls_to(&e, EASY_ALLOC), vec![vec![router.heap, 4]]);
        assert_eq!(calls_to(&e, READ_RAW), vec![vec![this, text, 3]]);
        // The table was full, so it grew before the pointer was pushed.
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![router.heap, this + 0x24, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x28), 4);
        let table = e.mem.u32(this + 0x24);
        assert_eq!(dwords(&e, table, 4), vec![0x1111, 0x2222, 0x3333, text]);
        // A negated index (and zero) look up the table.
        assert_eq!(e.call(0x00dc_5d90, &args![this]).u32(), 0x3333);
        assert_eq!(e.call(0x00dc_5d90, &args![this]).u32(), 0x1111);
        assert!(input.borrow().is_empty());
        assert_eq!(calls_to(&e, EASY_ALLOC).len(), 1);
    }

    #[test]
    fn read_bitfield_expands_bytes_to_one_byte_per_bit() {
        let mut e = engine();
        let router = externals(&mut e);
        let (this, _) = reader(&mut e);
        let input = feed(
            &mut e,
            Stream::default().byte(0b1011_0001).byte(0b0000_0011),
        );
        let array = e.mem.alloc(0x10);
        set_array(&mut e, array, 0, 0, NOT_OWNED);
        e.call(0x00dc_5e10, &args![this, 10u32, array]);
        // Reserved to the padded 16 bytes; the second reserve (10) finds enough room.
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![router.heap, array, 16, 1]]
        );
        assert_eq!(e.mem.u32(array + 4), 10);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.bytes(data, 10), vec![1, 0, 0, 0, 1, 1, 0, 1, 1, 1]);
        assert_eq!(e.mem.bytes(data + 10, 6), vec![0; 6]);
        let reads = calls_to(&e, READ_ARRAY);
        assert_eq!((reads[0][0], reads[0][2], reads[0][3]), (this, 1, 2));
        assert!(input.borrow().is_empty());
        // No bits: nothing to expand, and the size becomes 0.
        set_array(&mut e, array, data, 7, 16);
        e.call(0x00dc_5e10, &args![this, 0u32, array]);
        assert_eq!(e.mem.u32(array + 4), 0);
        let reads = calls_to(&e, READ_ARRAY);
        assert_eq!((reads[1][2], reads[1][3]), (1, 0));
        assert_eq!(calls_to(&e, ARRAY_RESERVE).len(), 1);
    }

    #[test]
    fn reader_construct_makes_the_arrays_and_pushes_the_first_entries() {
        let mut e = engine();
        let router = externals(&mut e);
        let this = e.mem.alloc(0x40);
        let result = e.call(0x00dc_5f30, &args![this, 0x1234_5600u32, 0x0b0b_0000u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(
            calls_to(&e, ARCHIVE_CONSTRUCT),
            vec![vec![this, 0x1234_5600, this & 0xffff_ff00]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x0b0b_0000);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        let classes = e.mem.u32(this + 0x18);
        assert_eq!(dwords(&e, classes, 1), vec![0]);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        assert_eq!(e.mem.u32(this + 0x20), 4);
        let strings = e.mem.u32(this + 0x24);
        assert_eq!(dwords(&e, strings, 2), vec![0x0101_1584, 0]);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert_eq!(e.mem.u32(this + 0x2c), 4);
        assert_eq!(dwords(&e, this + 0x30, 3), vec![0, 0, NOT_OWNED]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![
                vec![router.heap, this + 0x18, 4],
                vec![router.heap, this + 0x24, 4]
            ]
        );
    }

    #[test]
    fn reader_destruct_frees_the_strings_from_index_two_and_releases_the_arrays() {
        let mut e = engine();
        let router = externals(&mut e);
        let (this, _) = reader(&mut e);
        let object = object_with_slots(&mut e, &[(0, OBJECT_DELETE)]);
        nothing(&mut e, &[OBJECT_DELETE]);
        e.mem.set_u32(this + 0x14, object);
        let strings = words(&mut e, &[0x10, 0x20, 0x30, 0x40]);
        set_array(&mut e, this + 0x24, strings, 4, 6);
        let objects = words(&mut e, &[1, 2]);
        set_array(&mut e, this + 0x30, objects, 2, 2);
        let classes = words(&mut e, &[0, 0, 0]);
        set_array(&mut e, this + 0x18, classes, 3, 0x8000_0003);
        e.call(0x00dc_6020, &args![this]);
        assert_eq!(
            ordered(&e, &[OBJECT_DELETE, EASY_FREE, HEAP_FREE, ARCHIVE_DESTRUCT]),
            vec![
                (OBJECT_DELETE, vec![object, 1]),
                (EASY_FREE, vec![router.heap, 0x30]),
                (EASY_FREE, vec![router.heap, 0x40]),
                (HEAP_FREE, vec![router.heap, objects, 8]),
                (HEAP_FREE, vec![router.heap, strings, 24]),
                (ARCHIVE_DESTRUCT, vec![this]),
            ]
        );
        assert_eq!(
            dwords(&e, this + 0x18, 9),
            vec![0, 0, NOT_OWNED, 0, 0, NOT_OWNED, 0, 0, NOT_OWNED]
        );
        // Without an object and with only two strings nothing is deleted or freed one by one.
        let (this, _) = reader(&mut e);
        let strings = words(&mut e, &[0x10, 0x20]);
        set_array(&mut e, this + 0x24, strings, 2, 0x8000_0002);
        e.call(0x00dc_6020, &args![this]);
        assert_eq!(calls_to(&e, OBJECT_DELETE).len(), 1);
        assert_eq!(calls_to(&e, EASY_FREE).len(), 2);
        assert_eq!(calls_to(&e, HEAP_FREE).len(), 2);
        assert_eq!(calls_to(&e, ARCHIVE_DESTRUCT).len(), 2);
    }

    /// An engine with the externals, a reader and an array handle address.
    fn setup() -> (Engine, Router, u32, u32) {
        let mut e = engine();
        let router = externals(&mut e);
        let (this, _) = reader(&mut e);
        let array = e.mem.alloc(8);
        (e, router, this, array)
    }

    #[test]
    fn read_array_items_reads_bytes_through_a_temporary_block() {
        let (mut e, router, this, array) = setup();
        sequence(&mut e, ARRAY_GET_TYPE, &[1, 1]);
        let input = feed(&mut e, Stream::default().byte(7).byte(8).byte(9));
        e.call(0x00dc_6100, &args![this, array, 3u32]);
        let set = calls_to(&e, ARRAY_SET_BYTES);
        assert_eq!(set.len(), 1);
        let block = set[0][1];
        assert_eq!(set[0], vec![array, block, 3]);
        assert_eq!(e.mem.bytes(block, 3), vec![7, 8, 9]);
        assert_eq!(calls_to(&e, READ_ARRAY), vec![vec![this, block, 1, 3]]);
        let allocs = calls_to(&e, TEMP_ALLOC);
        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0][0], router.temporary);
        assert_eq!(
            calls_to(&e, TEMP_FREE),
            vec![vec![router.temporary, block, 3]]
        );
        assert!(input.borrow().is_empty());
        // No bytes: no block is asked for and nothing is freed.
        e.call(0x00dc_6100, &args![this, array, 0u32]);
        assert_eq!(calls_to(&e, TEMP_ALLOC).len(), 1);
        assert_eq!(calls_to(&e, ARRAY_SET_BYTES)[1], vec![array, 0, 0]);
        assert_eq!(calls_to(&e, TEMP_FREE).len(), 1);
    }

    #[test]
    fn read_array_items_reads_one_integer_per_element() {
        let (mut e, _, this, array) = setup();
        sequence(&mut e, ARRAY_GET_TYPE, &[2, 2]);
        let input = feed(&mut e, Stream::default().int(5).int(-7));
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let elements = calls_to(&e, ARRAY_ELEMENT);
        assert_eq!(elements.len(), 2);
        assert_eq!(
            (elements[0][0], elements[0][2], elements[1][2]),
            (array, 0, 1)
        );
        assert_eq!(
            calls_to(&e, ELEMENT_SET_INT),
            vec![
                vec![elements[0][1], 5],
                vec![elements[1][1], (-7i32) as u32]
            ]
        );
        assert!(input.borrow().is_empty());
        // A negative count does nothing.
        e.call(0x00dc_6100, &args![this, array, -1i32]);
        assert_eq!(calls_to(&e, ELEMENT_SET_INT).len(), 2);
    }

    #[test]
    fn read_array_items_reads_floats_through_a_temporary_block() {
        let (mut e, router, this, array) = setup();
        sequence(&mut e, ARRAY_GET_TYPE, &[3, 3]);
        let input = feed(
            &mut e,
            Stream::default()
                .float(1.5)
                .float(2.5)
                .float(3.5)
                .float(4.5),
        );
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let set = calls_to(&e, ARRAY_SET_FLOATS);
        let block = set[0][1];
        assert_eq!(set, vec![vec![array, block, 2]]);
        assert_eq!(
            dwords(&e, block, 2),
            vec![1.5f32.to_bits(), 2.5f32.to_bits()]
        );
        assert_eq!(calls_to(&e, READ_ARRAY), vec![vec![this, block, 4, 2]]);
        assert_eq!(
            calls_to(&e, TEMP_FREE),
            vec![vec![router.temporary, block, 8]]
        );
        // A block granted too small is grown by _reserve first, to the larger of twice its
        // capacity and the count.
        e.register_double(TEMP_ALLOC, |e, a| {
            let wanted = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], wanted.min(4));
            Ret {
                eax: e.mem.alloc(8),
                ..Ret::default()
            }
        });
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(
            (reserves[0][0], reserves[0][2], reserves[0][3]),
            (router.temporary, 2, 4)
        );
        let grown = calls_to(&e, ARRAY_SET_FLOATS)[1][1];
        assert_eq!(
            dwords(&e, grown, 2),
            vec![3.5f32.to_bits(), 4.5f32.to_bits()]
        );
        assert_eq!(calls_to(&e, TEMP_FREE)[1], vec![router.temporary, grown, 8]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_array_items_reads_vectors_of_the_width_given_first() {
        let (mut e, _, this, array) = setup();
        sequence(&mut e, ARRAY_GET_TYPE, &[4]);
        let vectors: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&vectors);
        e.register_double(ELEMENT_SET_VECTOR, move |e, a| {
            log.borrow_mut().push(dwords(e, a[1], a[2]));
            Ret::default()
        });
        let input = feed(
            &mut e,
            Stream::default()
                .int(2)
                .float(1.0)
                .float(2.0)
                .float(3.0)
                .float(4.0),
        );
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let bits = |value: f32| value.to_bits();
        assert_eq!(
            *vectors.borrow(),
            vec![
                vec![bits(1.0), bits(2.0), 0, 0],
                vec![bits(3.0), bits(4.0), 0, 0]
            ]
        );
        let reads = calls_to(&e, READ_ARRAY);
        assert_eq!(reads.len(), 3);
        assert_eq!((reads[1][2], reads[1][3]), (4, 2));
        assert_eq!(reads[1][1], reads[2][1]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_array_items_reads_vectors_of_twelve_or_sixteen_floats() {
        for (kind, floats) in [(6u32, 12u32), (7, 16)] {
            let (mut e, _, this, array) = setup();
            sequence(&mut e, ARRAY_GET_TYPE, &[kind, kind]);
            let vectors: Seen = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&vectors);
            e.register_double(ELEMENT_SET_VECTOR, move |e, a| {
                log.borrow_mut().push(dwords(e, a[1], a[2]));
                Ret::default()
            });
            // The second float is a signalling NaN, which the copy quiets.
            let mut stream = Stream::default();
            let mut expected = Vec::new();
            for i in 0..floats {
                if i == 1 {
                    stream = stream.word(0x7f80_0001);
                    expected.push(0x7fc0_0001);
                } else {
                    stream = stream.float(i as f32 + 1.0);
                    expected.push((i as f32 + 1.0).to_bits());
                }
            }
            let input = feed(&mut e, stream);
            e.call(0x00dc_6100, &args![this, array, 1u32]);
            assert_eq!(*vectors.borrow(), vec![expected]);
            let elements = calls_to(&e, ARRAY_ELEMENT);
            assert_eq!(elements.len(), 1);
            assert_eq!(calls_to(&e, ARRAY_GET_TYPE).len(), 2);
            assert_eq!(calls_to(&e, READ_ARRAY).len(), floats as usize);
            assert!(input.borrow().is_empty());
        }
    }

    #[test]
    fn read_array_items_reads_an_object_per_element() {
        let (mut e, _, this, array) = setup();
        sequence(&mut e, ARRAY_GET_TYPE, &[8]);
        let input = feed(&mut e, Stream::default().int(6).int(6));
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let constructs = calls_to(&e, DATA_OBJECT_CONSTRUCT);
        assert_eq!(constructs.len(), 2);
        let object = constructs[0][0];
        assert_eq!(constructs, vec![vec![object, 0]; 2]);
        let elements = calls_to(&e, ELEMENT_SET_OBJECT);
        assert_eq!(elements.len(), 2);
        assert_eq!((elements[0][1], elements[1][1]), (object, object));
        assert_eq!(calls_to(&e, DATA_OBJECT_DESTRUCT), vec![vec![object]; 2]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_array_items_reads_a_string_per_element() {
        let (mut e, _, this, array) = setup();
        let table = words(&mut e, &[0xa0, 0xa1, 0xa2]);
        set_array(&mut e, this + 0x24, table, 3, 3);
        sequence(&mut e, ARRAY_GET_TYPE, &[10]);
        let input = feed(&mut e, Stream::default().int(-2).int(0));
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        let strings: Vec<u32> = calls_to(&e, ELEMENT_SET_STRING)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(strings, vec![0xa2, 0xa0]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_array_items_ignores_the_types_without_items() {
        for kind in [0u32, 5, 11, 0x19] {
            let (mut e, _, this, array) = setup();
            sequence(&mut e, ARRAY_GET_TYPE, &[kind]);
            let input = feed(&mut e, Stream::default());
            e.call(0x00dc_6100, &args![this, array, 3u32]);
            // Only the call itself and the type query were made.
            assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
            assert!(input.borrow().is_empty());
        }
    }

    #[test]
    fn read_array_items_reads_the_members_of_compound_items() {
        let (mut e, router, this, array) = setup();
        // The array's type, then the type of the swizzled array of each read: integers.
        sequence(&mut e, ARRAY_GET_TYPE, &[9, 2, 2, 2]);
        answer(&mut e, DATA_CLASS_NUM_MEMBERS, 3);
        member_info(
            &mut e,
            vec![[0xa0, 0, 2, 0, 0], [0xa1, 0, 2, 0, 3], [0xa2, 0, 2, 0, 0]],
        );
        e.register_double(ARRAY_SWIZZLE_MEMBER, |e, a| {
            e.mem.set_u32(a[1], 0x5000 + a[2]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The member bits (members 0 and 1), member 0's two integers and member 1's three
        // integers for each of the two items.
        let input = feed(
            &mut e,
            Stream::default()
                .byte(0b011)
                .int(10)
                .int(-3)
                .int(1)
                .int(2)
                .int(3)
                .int(4)
                .int(5)
                .int(6),
        );
        e.call(0x00dc_6100, &args![this, array, 2u32]);
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![array, 2]]);
        let class = calls_to(&e, ARRAY_GET_CLASS);
        assert_eq!(class.len(), 1);
        assert_eq!(class[0][0], array);
        assert_eq!(
            calls_to(&e, DATA_CLASS_NUM_MEMBERS),
            vec![vec![class[0][1]]]
        );
        let info = calls_to(&e, DATA_CLASS_ALL_MEMBER_INFO);
        assert_eq!(info.len(), 1);
        assert_eq!(info[0][0], class[0][1]);
        let ints: Vec<u32> = calls_to(&e, ELEMENT_SET_INT).iter().map(|w| w[1]).collect();
        assert_eq!(ints, vec![10, (-3i32) as u32, 1, 2, 3, 4, 5, 6]);
        let swizzles = calls_to(&e, ARRAY_SWIZZLE_MEMBER);
        assert_eq!(
            swizzles.iter().map(|w| (w[0], w[2])).collect::<Vec<_>>(),
            vec![(array, 0xa0), (array, 0xa1)]
        );
        assert_eq!(calls_to(&e, ELEMENT_AS_ARRAY).len(), 2);
        assert_eq!(calls_to(&e, HANDLE_RELEASE).len(), 4);
        assert_eq!(calls_to(&e, DATA_CLASS_DESTRUCT), vec![vec![class[0][1]]]);
        // The scratch arrays are not owned: the router's bump pointer is back and nothing is
        // freed.
        assert_eq!(e.mem.u32(router.block), router.pool);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert!(input.borrow().is_empty());
    }

    /// Runs `readBinaryValue` for a value (`0x1000`, `0x2000`) and a member record whose flags
    /// are `flags`, on a reader whose string table is `[0xa0, 0xa1, 0xa2]`. Every value setter
    /// double records the two value words (first answer), the vector setter also the floats it
    /// is given (second answer).
    fn value_run(flags: u32, stream: Stream) -> (Engine, Seen, Seen) {
        let mut e = engine();
        externals(&mut e);
        let (this, _) = reader(&mut e);
        let table = words(&mut e, &[0xa0, 0xa1, 0xa2]);
        set_array(&mut e, this + 0x24, table, 3, 3);
        let values: Seen = Rc::new(RefCell::new(Vec::new()));
        let vectors: Seen = Rc::new(RefCell::new(Vec::new()));
        for address in [
            VALUE_SET_BYTE,
            VALUE_SET_INT_64,
            VALUE_SET_FLOAT,
            VALUE_SET_OBJECT,
            VALUE_SET_STRING,
        ] {
            let log = Rc::clone(&values);
            e.register_double(address, move |e, a| {
                log.borrow_mut().push(dwords(e, a[0], 2));
                Ret::default()
            });
        }
        let log = Rc::clone(&values);
        let floats = Rc::clone(&vectors);
        e.register_double(VALUE_SET_VECTOR, move |e, a| {
            log.borrow_mut().push(dwords(e, a[0], 2));
            floats.borrow_mut().push(dwords(e, a[1], a[2]));
            Ret::default()
        });
        let info = words(&mut e, &[0xaa, 0, flags, 0, 0]);
        let input = feed(&mut e, stream);
        e.call(0x00dc_6720, &args![this, 0x1000u32, 0x2000u32, info]);
        assert!(input.borrow().is_empty());
        (e, values, vectors)
    }

    #[test]
    fn read_binary_value_reads_a_simple_value_into_the_value_words() {
        let value_words = vec![vec![0x1000, 0x2000]];
        // 1: a byte.
        let (e, values, _) = value_run(1, Stream::default().byte(0xab));
        let sets = calls_to(&e, VALUE_SET_BYTE);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0][1], 0xab);
        assert_eq!(*values.borrow(), value_words);
        // 2: a 64-bit integer, low word first.
        let (e, _, _) = value_run(2, Stream::default().int(-5));
        let sets = calls_to(&e, VALUE_SET_INT_64);
        assert_eq!(sets[0][1..].to_vec(), vec![0xffff_fffb, 0xffff_ffff]);
        // 3: a float, quieted.
        let (e, _, _) = value_run(3, Stream::default().float(2.5));
        assert_eq!(calls_to(&e, VALUE_SET_FLOAT)[0][1], 2.5f32.to_bits());
        let (e, _, _) = value_run(3, Stream::default().word(0x7f80_0001));
        assert_eq!(calls_to(&e, VALUE_SET_FLOAT)[0][1], 0x7fc0_0001);
        // 4, 6 and 7: vectors of 4, 12 and 16 floats.
        for (flags, count) in [(4u32, 4u32), (6, 12), (7, 16)] {
            let mut stream = Stream::default();
            let mut expected = Vec::new();
            for i in 0..count {
                if i == 2 {
                    stream = stream.word(0xff80_0003);
                    expected.push(0xffc0_0003);
                } else {
                    stream = stream.float(i as f32 * 0.5);
                    expected.push((i as f32 * 0.5).to_bits());
                }
            }
            let (e, values, vectors) = value_run(flags, stream);
            assert_eq!(*vectors.borrow(), vec![expected]);
            assert_eq!(*values.borrow(), value_words);
            assert_eq!(calls_to(&e, VALUE_SET_VECTOR)[0][2], count);
        }
        // 8 and 9: an object (tag 6 is read, which makes a null object) is set and released.
        for flags in [8u32, 9] {
            let (e, values, _) = value_run(flags, Stream::default().int(6));
            let constructs = calls_to(&e, DATA_OBJECT_CONSTRUCT);
            let object = constructs[0][0];
            assert_eq!(constructs, vec![vec![object, 0]]);
            assert_eq!(calls_to(&e, VALUE_SET_OBJECT)[0][1], object);
            assert_eq!(calls_to(&e, DATA_OBJECT_DESTRUCT), vec![vec![object]]);
            assert_eq!(*values.borrow(), value_words);
        }
        // 10: a string, here string table entry 1.
        let (e, values, _) = value_run(10, Stream::default().int(-1));
        assert_eq!(calls_to(&e, VALUE_SET_STRING)[0][1], 0xa1);
        assert_eq!(*values.borrow(), value_words);
        // 5, 11, 0 and an unknown bit: nothing is read or set.
        for flags in [5u32, 11, 0, 0x40] {
            let (e, values, _) = value_run(flags, Stream::default());
            assert!(values.borrow().is_empty());
            assert!(calls_to(&e, READ_ARRAY).is_empty());
        }
    }

    #[test]
    fn read_binary_value_reads_the_items_of_an_array_value() {
        let mut e = engine();
        externals(&mut e);
        let (this, builder) = reader(&mut e);
        let info = words(&mut e, &[0xaa, 0, 0x22, 0, 4]);
        sequence(&mut e, ARRAY_GET_TYPE, &[2]);
        let made = watch(&mut e, MAKE_ARRAY, 2, 5, 0x0a0a_0007);
        let input = feed(&mut e, Stream::default().int(1).int(2).int(3).int(4));
        e.call(0x00dc_6720, &args![this, 0x1000u32, 0x2000u32, info]);
        let constructs = calls_to(&e, DATA_OBJECT_CONSTRUCT);
        let object = constructs[0][0];
        assert_eq!(constructs, vec![vec![object, 0x1000]]);
        assert_eq!(calls_to(&e, MAKE_ARRAY), vec![vec![builder, object, info]]);
        assert_eq!(*made.borrow(), vec![vec![0xaa, 0, 0x22, 0, 4]]);
        let handle = calls_to(&e, HANDLE_SET)[0][0];
        assert_eq!(
            ordered(
                &e,
                &[
                    HANDLE_SET,
                    ARRAY_GET_TYPE,
                    HANDLE_RELEASE,
                    DATA_OBJECT_DESTRUCT
                ]
            ),
            vec![
                (HANDLE_SET, vec![handle, 0x0a0a_0007]),
                (ARRAY_GET_TYPE, vec![handle]),
                (HANDLE_RELEASE, vec![handle]),
                (DATA_OBJECT_DESTRUCT, vec![object]),
            ]
        );
        assert_eq!(calls_to(&e, ELEMENT_SET_INT).len(), 4);
        assert!(input.borrow().is_empty());
    }

    /// A reader whose class array is `[0, 0xc1a5]` and whose object array is `[0x0b0c_0001,
    /// 0x0b0c_0002]`.
    fn object_setup() -> (Engine, Router, u32, u32) {
        let mut e = engine();
        let router = externals(&mut e);
        let (this, builder) = reader(&mut e);
        let classes = words(&mut e, &[0, 0xc1a5]);
        set_array(&mut e, this + 0x18, classes, 2, 2);
        let objects = words(&mut e, &[0x0b0c_0001, 0x0b0c_0002]);
        set_array(&mut e, this + 0x30, objects, 2, 2);
        (e, router, this, builder)
    }

    #[test]
    fn read_object_answers_a_known_object_or_null_for_tags_five_and_six() {
        let (mut e, _, this, _) = object_setup();
        let input = feed(&mut e, Stream::default().int(5).int(1).int(0));
        let out = e.mem.alloc(8);
        // The tag is read from the stream when it is -1.
        assert_eq!(e.call(0x00dc_6970, &args![this, out, -1i32]).u32(), out);
        assert_eq!(
            calls_to(&e, DATA_OBJECT_CONSTRUCT),
            vec![vec![out, 0x0b0c_0002]]
        );
        // Given as 5, the index is read: object 0.
        assert_eq!(e.call(0x00dc_6970, &args![this, out, 5u32]).u32(), out);
        assert_eq!(
            calls_to(&e, DATA_OBJECT_CONSTRUCT)[1],
            vec![out, 0x0b0c_0001]
        );
        // Tag 6 reads nothing and makes a null object.
        assert_eq!(e.call(0x00dc_6970, &args![this, out, 6u32]).u32(), out);
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT)[2], vec![out, 0]);
        assert!(input.borrow().is_empty());
        assert_eq!(calls_to(&e, DATA_CLASS_CONSTRUCT).len(), 0);
        assert_eq!(calls_to(&e, ARRAY_RESERVE_MORE).len(), 0);
    }

    #[test]
    fn read_object_creates_the_object_and_reads_the_members_whose_bit_is_set() {
        let (mut e, router, this, builder) = object_setup();
        answer(&mut e, DATA_CLASS_NUM_MEMBERS, 2);
        member_info(&mut e, vec![[0xaa, 0, 2, 0, 0], [0xbb, 0, 2, 0, 0]]);
        let proxy = words(&mut e, &[0x1111, 0x2222]);
        answer(&mut e, DATA_OBJECT_INDEX, proxy);
        let values = watch(&mut e, VALUE_SET_INT_64, 0, 2, 0);
        // The class index, the member bits (member 0 only) and member 0's integer.
        let input = feed(&mut e, Stream::default().int(1).byte(0b01).int(42));
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_6970, &args![this, out, 3u32]).u32(), out);
        let class = calls_to(&e, DATA_CLASS_CONSTRUCT);
        let f = class[0][0];
        assert_eq!(class, vec![vec![f, 0xc1a5]]);
        assert_eq!(
            ordered(
                &e,
                &[
                    DATA_CLASS_NUM_MEMBERS,
                    CREATE_OBJECT,
                    DATA_OBJECT_CONSTRUCT,
                    DATA_CLASS_ALL_MEMBER_INFO,
                    DATA_OBJECT_INDEX,
                    VALUE_SET_INT_64,
                    DATA_OBJECT_COPY,
                    DATA_OBJECT_DESTRUCT,
                    DATA_CLASS_DESTRUCT,
                ]
            ),
            vec![
                (DATA_CLASS_NUM_MEMBERS, vec![f]),
                (CREATE_OBJECT, vec![builder, f]),
                (DATA_OBJECT_CONSTRUCT, vec![f + 4, 0x0b0b_0001]),
                (DATA_CLASS_ALL_MEMBER_INFO, vec![f, f + 8]),
                (DATA_OBJECT_INDEX, vec![f + 4, f + 0x2c, 0xaa]),
                (VALUE_SET_INT_64, calls_to(&e, VALUE_SET_INT_64)[0].clone()),
                (DATA_OBJECT_COPY, vec![out, f + 4]),
                (DATA_OBJECT_DESTRUCT, vec![f + 4]),
                (DATA_CLASS_DESTRUCT, vec![f]),
            ]
        );
        // The value words come from the operator[] answer.
        assert_eq!(*values.borrow(), vec![vec![0x1111, 0x2222]]);
        assert_eq!(calls_to(&e, VALUE_SET_INT_64)[0][1..].to_vec(), vec![42, 0]);
        assert_eq!(e.mem.u32(router.block), router.pool);
        assert_eq!(e.mem.u32(this + 0x34), 2);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_object_pushes_a_tag_four_object_on_the_object_array() {
        let (mut e, router, this, _) = object_setup();
        e.register_double(DATA_OBJECT_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let input = feed(&mut e, Stream::default().int(1).int(1));
        let out = e.mem.alloc(8);
        e.call(0x00dc_6970, &args![this, out, 4u32]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![router.heap, this + 0x30, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x34), 3);
        let objects = e.mem.u32(this + 0x30);
        assert_eq!(
            dwords(&e, objects, 3),
            vec![0x0b0c_0001, 0x0b0c_0002, 0x0b0b_0001]
        );
        // Tag 3 does not push.
        e.call(0x00dc_6970, &args![this, out, 3u32]);
        assert_eq!(e.mem.u32(this + 0x34), 3);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_object_reports_an_unknown_tag_as_a_corrupt_file() {
        let (mut e, _, this, _) = object_setup();
        let input = feed(&mut e, Stream::default().int(1));
        let out = e.mem.alloc(8);
        e.call(0x00dc_6970, &args![this, out, 9u32]);
        let class = calls_to(&e, DATA_CLASS_CONSTRUCT);
        let f = class[0][0];
        let sink = e.mem.u32(ERROR_SINK);
        assert_eq!(
            ordered(
                &e,
                &[
                    ERROR_BUILDER_CONSTRUCT,
                    ERROR_BUILDER_APPEND,
                    SINK,
                    ERROR_BUILDER_DESTRUCT
                ]
            ),
            vec![
                (ERROR_BUILDER_CONSTRUCT, vec![f + 0x2c, f + 0x38, 0x200]),
                (ERROR_BUILDER_APPEND, vec![f + 0x2c, 0x010d_6d24]),
                (
                    SINK,
                    vec![sink, 3, 0x4849_94d5, f + 0x38, 0x010d_6cf8, 0x178]
                ),
                (ERROR_BUILDER_DESTRUCT, vec![f + 0x2c]),
            ]
        );
        // The object is still made and its (zero) members read.
        assert_eq!(calls_to(&e, DATA_OBJECT_COPY), vec![vec![out, f + 4]]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    #[should_panic(expected = "INT3 at 00dc6a74")]
    fn read_object_stops_when_the_sink_answers_non_zero() {
        let (mut e, _, this, _) = object_setup();
        answer(&mut e, SINK, 1);
        let _input = feed(&mut e, Stream::default().int(1));
        let out = e.mem.alloc(8);
        e.call(0x00dc_6970, &args![this, out, 9u32]);
    }

    #[test]
    fn read_class_stops_when_the_builder_says_so() {
        let (mut e, _, this, builder) = object_setup();
        let zero = e.mem.alloc(4);
        answer(&mut e, CHECK_STATUS, zero);
        let input = feed(&mut e, Stream::default().int(1).text("A").int(2));
        assert_eq!(e.call(0x00dc_6c90, &args![this]).u32(), 1);
        let checks = calls_to(&e, CHECK_STATUS);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][0], builder);
        assert!(calls_to(&e, BUILD_CLASS).is_empty());
        // The name was read and kept in the string table; no class was added.
        assert_eq!(e.mem.u32(this + 0x28), 1);
        let strings = e.mem.u32(this + 0x24);
        assert_eq!(e.mem.cstr(e.mem.u32(strings)), b"A");
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_class_reads_the_members_and_has_the_builder_make_the_class() {
        let (mut e, router, this, builder) = object_setup();
        let built: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&built);
        e.register_double(BUILD_CLASS, move |e, a| {
            let description = dwords(e, a[1], 6);
            let records = dwords(e, description[3], 8);
            log.borrow_mut().push(description);
            log.borrow_mut().push(records);
            Ret {
                eax: 0xc1a5_5500,
                ..Ret::default()
            }
        });
        // Name "N", version 7, no parent, two members: "x" (flags 0x21 with the extra
        // integer 5) and "y" (flags 9 with the extra string "T").
        let input = feed(
            &mut e,
            Stream::default()
                .int(1)
                .text("N")
                .int(7)
                .int(-1)
                .int(2)
                .int(1)
                .text("x")
                .int(0x21)
                .int(5)
                .int(1)
                .text("y")
                .int(9)
                .int(1)
                .text("T"),
        );
        assert_eq!(e.call(0x00dc_6c90, &args![this]).u32(), 0);
        let strings = e.mem.u32(this + 0x24);
        let names = dwords(&e, strings, 4);
        assert_eq!(e.mem.cstr(names[0]), b"N");
        assert_eq!(e.mem.cstr(names[1]), b"x");
        assert_eq!(e.mem.cstr(names[2]), b"y");
        assert_eq!(e.mem.cstr(names[3]), b"T");
        let built = built.borrow();
        let data = built[0][3];
        assert_eq!(built[0], vec![names[0], 7, 0, data, 2, 2]);
        assert_eq!(
            built[1],
            vec![names[1], 0x21, 5, 0, names[2], 9, 0, names[3]]
        );
        assert_eq!(calls_to(&e, ADD_MEMBER), vec![vec![builder, names[3]]]);
        let reserves = calls_to(&e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 1);
        assert_eq!(
            (reserves[0][0], reserves[0][2], reserves[0][3]),
            (router.heap, 2, 0x10)
        );
        // The member list is released: two 16-byte records.
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![router.heap, data, 32]]);
        // The class went onto the class array after the two that were there.
        assert_eq!(e.mem.u32(this + 0x1c), 3);
        let classes = e.mem.u32(this + 0x18);
        assert_eq!(dwords(&e, classes, 3), vec![0, 0xc1a5, 0xc1a5_5500]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_class_takes_the_parent_from_the_class_array() {
        let (mut e, _, this, _) = object_setup();
        let parent_class = object_with_slots(&mut e, &[(8, CLASS_PARENT)]);
        answer(&mut e, CLASS_PARENT, 0x7a7a);
        let classes = words(&mut e, &[0, parent_class]);
        set_array(&mut e, this + 0x18, classes, 2, 2);
        let built = watch(&mut e, BUILD_CLASS, 1, 3, 0xc1a5_5500);
        let input = feed(
            &mut e,
            Stream::default()
                .int(1)
                .text("N")
                .int(0)
                // Parent index 1 (an object that answers 0x7a7a), 0 (null), -3 (negative).
                .int(1)
                .int(0)
                .int(1)
                .text("M")
                .int(0)
                .int(0)
                .int(0)
                .int(1)
                .text("L")
                .int(0)
                .int(-3)
                .int(0),
        );
        for _ in 0..3 {
            assert_eq!(e.call(0x00dc_6c90, &args![this]).u32(), 0);
        }
        let parents: Vec<u32> = built.borrow().iter().map(|d| d[2]).collect();
        assert_eq!(parents, vec![0x7a7a, 0, 0]);
        assert_eq!(calls_to(&e, CLASS_PARENT), vec![vec![parent_class]]);
        assert!(input.borrow().is_empty());
    }

    /// Runs `readObjectTree` on a stream with `configure` applied first; answers the engine,
    /// the reader, the out handle and what is left of the stream.
    fn tree_run(stream: Stream, configure: impl FnOnce(&mut Engine)) -> (Engine, u32, u32, Input) {
        let mut e = engine();
        externals(&mut e);
        let (this, _) = reader(&mut e);
        configure(&mut e);
        let input = feed(&mut e, stream);
        let out = e.mem.alloc(8);
        assert_eq!(e.call(0x00dc_6e50, &args![this, out]).u32(), out);
        (e, this, out, input)
    }

    const MAGIC: u32 = 0xcab0_0d1e;
    const MAGIC_SECOND: u32 = 0xd011_face;

    #[test]
    fn read_object_tree_reports_a_bad_magic_number() {
        for stream in [
            Stream::default().word(0).word(MAGIC_SECOND),
            Stream::default().word(MAGIC).word(0xdead),
        ] {
            let (e, _, out, input) = tree_run(stream, |_| {});
            let sink = e.mem.u32(ERROR_SINK);
            let sinks = calls_to(&e, SINK);
            assert_eq!(sinks.len(), 1);
            let buffer = sinks[0][3];
            assert_eq!(
                sinks[0],
                vec![sink, 1, 0x2ab6_036f, buffer, 0x010d_6cf8, 0x1c5]
            );
            let made = calls_to(&e, ERROR_BUILDER_CONSTRUCT);
            let builder = made[0][0];
            assert_eq!(made, vec![vec![builder, buffer, 0x200]]);
            assert_eq!(
                calls_to(&e, ERROR_BUILDER_APPEND),
                vec![vec![builder, 0x010d_6d38]]
            );
            assert_eq!(calls_to(&e, ERROR_BUILDER_DESTRUCT), vec![vec![builder]]);
            assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0]]);
            assert!(input.borrow().is_empty());
        }
    }

    #[test]
    fn read_object_tree_reports_an_unknown_version() {
        let stream = Stream::default()
            .word(MAGIC)
            .word(MAGIC_SECOND)
            .int(1)
            .int(1);
        let (e, _, out, input) = tree_run(stream, |_| {});
        let sinks = calls_to(&e, SINK);
        assert_eq!(sinks.len(), 1);
        assert_eq!(
            (sinks[0][1], sinks[0][2], sinks[0][5]),
            (1, 0x2ab6_036f, 0x1d9)
        );
        let made = calls_to(&e, ERROR_BUILDER_CONSTRUCT);
        assert_eq!(
            calls_to(&e, ERROR_BUILDER_APPEND),
            vec![vec![made[0][0], 0x010d_6d7c]]
        );
        assert_eq!(calls_to(&e, ERROR_BUILDER_DESTRUCT), vec![vec![made[0][0]]]);
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0]]);
        assert!(input.borrow().is_empty());
    }

    #[test]
    fn read_object_tree_skips_a_version_zero_and_stops_at_the_end_tags() {
        // Version 0 is fine; the next tag then ends the tree: -1, 5, a tag past 6, 0 and -2.
        for tag in [-1i64, 5, 7, 0, -2] {
            let stream = Stream::default()
                .word(MAGIC)
                .word(MAGIC_SECOND)
                .int(1)
                .int(0)
                .int(tag);
            let (e, _, out, input) = tree_run(stream, |_| {});
            assert!(calls_to(&e, SINK).is_empty());
            assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0]]);
            assert!(input.borrow().is_empty());
        }
    }

    #[test]
    fn read_object_tree_reads_classes_and_then_an_object() {
        // A class the builder says to stop at ends the tree without a class.
        let stream = Stream::default()
            .word(MAGIC)
            .word(MAGIC_SECOND)
            .int(2)
            .int(1)
            .text("C")
            .int(0);
        let (e, _, out, input) = tree_run(stream, |e| {
            let zero = e.mem.alloc(4);
            answer(e, CHECK_STATUS, zero);
        });
        assert!(calls_to(&e, BUILD_CLASS).is_empty());
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0]]);
        assert!(input.borrow().is_empty());
        // A class without members, then tag 6: the object is read (a null one) and answered
        // without a second null object.
        let stream = Stream::default()
            .word(MAGIC)
            .word(MAGIC_SECOND)
            .int(2)
            .int(1)
            .text("C")
            .int(0)
            .int(-1)
            .int(0)
            .int(6);
        let (e, _, out, input) = tree_run(stream, |_| {});
        assert_eq!(calls_to(&e, BUILD_CLASS).len(), 1);
        assert_eq!(calls_to(&e, DATA_OBJECT_CONSTRUCT), vec![vec![out, 0]]);
        assert!(input.borrow().is_empty());
    }
}
