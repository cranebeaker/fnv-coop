//! `Havok SDK/hkbinarypackfilewriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dbd7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00db_c6e0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dbd7f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00db_c6e0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `memcpy` (`00cb1260`), `memset` (`00cb1280`), `strncpy` (`00cb10b0`) and `strlen`
/// (`00cb10d0`) of the CRT; all cdecl. The padding code calls the CRT `_memset` (`00ec61c0`,
/// cdecl `(destination, value, bytes)`) directly.
const MEM_COPY: u32 = 0x00cb_1260;
const MEM_SET: u32 = 0x00cb_1280;
const STRING_COPY_N: u32 = 0x00cb_10b0;
const STRING_LENGTH: u32 = 0x00cb_10d0;
const CRT_MEM_SET: u32 = 0x00ec_61c0;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, capacity, element size)`) and
/// `_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The pointer map's get-with-default (`00cb1930`, `this` = the map, `(key, default)`).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// The string map functions (`this` = the string map): constructor (`00d87790`, one dword it
/// ignores), get with default (`00d87800`, `(key, default)`), insert (`00d874d0`, `(key,
/// value)`), find key (`00d87580`, `(key)`, answers the index, one past the mask when absent),
/// is-valid (`00d874b0`, `(&out byte, index)`, stores `index <= mask` signed) and destructor
/// (`00d877d0`).
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
const STRING_MAP_FIND_KEY: u32 = 0x00d8_7580;
const STRING_MAP_IS_VALID: u32 = 0x00d8_74b0;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
/// Class functions (`this` = the class): name (`00d46d40`) and signature (`00d47690`, `(flags)`).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const CLASS_GET_SIGNATURE: u32 = 0x00d4_7690;
/// The archive writer the pack file bytes go through (`this` = the archive object whose vtable
/// is `0x010d14f8`): constructor (`00d60830`, `(stream, swap byte)`), `write32` (`00d60940`),
/// `writeRaw` (`00d607b0`, `(data, size)`), the generic array writer (`00d60600`, `(data,
/// element size, count)`), the 32-bit array writer (`00d60aa0`, `(data, count)`) and the
/// destructor (`00d60590`).
const ARCHIVE_CONSTRUCT: u32 = 0x00d6_0830;
const ARCHIVE_WRITE_32: u32 = 0x00d6_0940;
const ARCHIVE_WRITE_RAW: u32 = 0x00d6_07b0;
const ARCHIVE_WRITE_ARRAY: u32 = 0x00d6_0600;
const ARCHIVE_WRITE_ARRAY_32: u32 = 0x00d6_0aa0;
const ARCHIVE_DESTRUCT: u32 = 0x00d6_0590;
/// Swaps the three words of two arrays (`00d9ec10`, `this` = one array, `(other array)`).
const ARRAY_SWAP: u32 = 0x00d9_ec10;
/// The platform object writer: constructor (`00e02bb0`, `this` = the writer, `(layout rules)`),
/// destructor (`00e02c70`) and get layout (`00e02a70`, answers a pointer to the layout rules);
/// its virtual slot `0xc` writes an object (`(stream, object, class, relocation info)`). The
/// relocation info destructor is `00e028e0`.
const PLATFORM_WRITER_CONSTRUCT: u32 = 0x00e0_2bb0;
const PLATFORM_WRITER_DESTRUCT: u32 = 0x00e0_2c70;
const PLATFORM_WRITER_GET_LAYOUT: u32 = 0x00e0_2a70;
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
/// The packfile writer base class: constructor (`00dbe4d0`), `addSection` (`00dbdef0`, `(tag)`),
/// `setSectionForClass` (`00dbdf30`, `(class, tag)`), destructor (`00dbdf60`) and the section
/// index lookup (`00dbdd40`, `(tag)`).
const PACKFILE_WRITER_CONSTRUCT: u32 = 0x00db_e4d0;
const ADD_SECTION: u32 = 0x00db_def0;
const SET_SECTION_FOR_CLASS: u32 = 0x00db_df30;
const PACKFILE_WRITER_DESTRUCT: u32 = 0x00db_df60;
const SECTION_INDEX: u32 = 0x00db_dd40;
/// Writes the default version text into a buffer (`00dbda20`, cdecl `(buffer, size)`).
const VERSION_TEXT: u32 = 0x00db_da20;
/// The vtable of the binary packfile writer and of the sub stream `save` builds on its stack.
const WRITER_VTABLE: u32 = 0x010d_65bc;
const SUB_STREAM_VTABLE: u32 = 0x010d_65ec;
/// The base vtable the sub stream is left with.
const BASE_VTABLE: u32 = 0x0102_e388;
/// Exe globals: the pointer to the section tag "__classnames__", the pointer to the section tag
/// "__types__" and the byte holding the host's endianness flag.
const CLASS_NAMES_TAG_POINTER: u32 = 0x011b_7a7c;
const TYPES_TAG_POINTER: u32 = 0x011b_7aec;
const HOST_ENDIAN_FLAG: u32 = 0x011b_7b89;
/// The four class objects (exe data) the writer registers with the "__types__" section and
/// whose class names it writes first (identities unconfirmed).
const CLASS_OBJECTS: [u32; 4] = [0x0126_cae8, 0x0126_ca58, 0x0126_cab8, 0x0126_ca88];
/// The class records used to write the file header (`0x0126c6c8`) and the section headers
/// (`0x0126c698`).
const FILE_HEADER_CLASS: u32 = 0x0126_c6c8;
const SECTION_HEADER_CLASS: u32 = 0x0126_c698;
/// The "not owned" bit of a capacity word, and the capacity bits.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The inlined "reserve at least": after asking the memory router for its allocator, grows
/// the array (data, size, capacity word) to the larger of `wanted` and twice its capacity
/// (element size `element`) when its capacity is below `wanted` (signed).
fn reserve_at_least(e: &mut Engine, array: u32, wanted: i32, element: u32) {
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let target = capacity.wrapping_add(capacity).max(wanted);
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, target as u32, element],
        );
    }
}

/// The inlined array release: gives `data` back through slot `0x10` of the allocator with the
/// byte size `capacity * element` unless the capacity word marks the data as not owned.
fn release_owned(e: &mut Engine, allocator: u32, data: u32, capacity: u32, element: u32) {
    if capacity & NOT_OWNED == 0 {
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Stores an empty relocation info at `at`: four empty arrays (`data` 0, `size` 0, capacity
/// word `0x80000000`) at `+0`, `+0xc`, `+0x18` and `+0x24`, and a zero word at `+0x30`.
fn store_empty_relocation_info(e: &mut Engine, at: u32) {
    for array in [0u32, 0xc, 0x18, 0x24] {
        e.mem.set_u32(at + array, 0);
        e.mem.set_u32(at + array + 4, 0);
        e.mem.set_u32(at + array + 8, NOT_OWNED);
    }
    e.mem.set_u32(at + 0x30, 0);
}

// Translated from 00dbc670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::hkBinaryPackfileWriter` (Xbox PDB): `this` is ECX. Runs the base
/// constructor (`00dbe4d0`), stores the vtable `0x010d65bc`, adds the section named by the
/// pointer at `0x011b7a7c` ("__classnames__") and assigns the four class objects
/// ([`CLASS_OBJECTS`]) to the section named by the pointer at `0x011b7aec` ("__types__"),
/// reading that global before each call. Returns `this`.
pub fn hk_binary_packfile_writer_hk_binary_packfile_writer(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PACKFILE_WRITER_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), WRITER_VTABLE);
    let tag = e.mem.u32(CLASS_NAMES_TAG_POINTER);
    e.call(ADD_SECTION, &args![this, tag]);
    for class in CLASS_OBJECTS {
        let tag = e.mem.u32(TYPES_TAG_POINTER);
        e.call(SET_SECTION_FOR_CLASS, &args![this, class, tag]);
    }
    this
}

// Translated from 00dbc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the binary packfile writer (unnamed in the exe): `this` is ECX. Stores the
/// vtable `0x010d65bc` and jumps to the base destructor (`00dbdf60`), which returns nothing.
pub fn destructor_00dbc6e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), WRITER_VTABLE);
    e.call(PACKFILE_WRITER_DESTRUCT, &args![this]);
}

// Translated from 00dbc6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::writeClassName` (Xbox PDB): ECX (word 0, `_unused_0`) is not read.
/// `context` is the save context, `class` the class record and `base` the stream position the
/// recorded offset is relative to. Writes the class signature (`00d47690(class, 0)`, with its
/// bytes reversed when the context byte at `+0x4c` is non-zero) followed by the byte 9 as a
/// five-byte record through stream slot `0x10`, then inserts the class name (`00d46d40`) into
/// the string map at `context + 0x30` with the stream position (slot `0x20`) minus `base`,
/// and writes the name with its terminating zero through slot `0x10`.
pub fn write_class_name(e: &mut Engine, _unused_0: u32, context: u32, class: u32, base: u32) {
    let signature = e.call(CLASS_GET_SIGNATURE, &args![class, 0u32]).u32();
    let signature = if e.mem.u8(context + 0x4c) != 0 {
        signature.swap_bytes()
    } else {
        signature
    };
    let stream = e.mem.u32(context + 0x1c);
    e.with_stack(8, |e, record| {
        let record = record.addr();
        e.mem.set_u32(record, signature);
        e.mem.set_u8(record + 4, 9);
        e.vcall(stream, 0x10, &args![record, 5u32]);
    });
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let position = e.vcall(stream, 0x20, &args![]).u32();
    e.call(
        STRING_MAP_INSERT,
        &args![context + 0x30, name, position.wrapping_sub(base)],
    );
    let length = e.call(STRING_LENGTH, &args![name]).u32();
    let stream = e.mem.u32(context + 0x1c);
    e.vcall(stream, 0x10, &args![name, length.wrapping_add(1)]);
}

// Translated from 00dbd720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl): `array` points at an array (data, size, ...) of 48-byte elements; answers
/// `size * 0x30`, the byte size of its elements.
pub fn section_array_bytes_00dbd720(e: &mut Engine, array: u32) -> u32 {
    e.mem.u32(array + 4).wrapping_mul(3) << 4
}

// Translated from 00dbc7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::fillSectionTags` (Xbox PDB): `this` is ECX, `context` the save
/// context. With `n` = the number of section names at `this + 0x68` (names array at `+0x64`):
/// grows the context's section array (at `context + 4`, 48-byte elements) to `n` elements,
/// zeroes the first four bytes of each new element, sets its size to `n`, fills the whole array
/// with `0xff` bytes and copies each section name into its element with `strncpy(.., 0x13)`.
pub fn fill_section_tags(e: &mut Engine, this: Ptr, context: u32) {
    let t = this.addr();
    let wanted = e.mem.i32(t + 0x68);
    let array = context + 4;
    reserve_at_least(e, array, wanted, 0x30);
    let size = e.mem.i32(array + 4);
    let count = wanted.wrapping_sub(size);
    let mut slot = size.wrapping_mul(3).wrapping_shl(4) as u32;
    slot = slot.wrapping_add(e.mem.u32(array));
    if count > 0 {
        for _ in 0..count {
            // The exe skips a null element (placement new of a null pointer).
            if slot != 0 {
                e.call(MEM_SET, &args![slot, 0u32, 4u32]);
            }
            slot = slot.wrapping_add(0x30);
        }
    }
    e.mem.set_u32(array + 4, wanted as u32);
    let bytes = section_array_bytes_00dbd720(e, array);
    let data = e.mem.u32(array);
    e.call(MEM_SET, &args![data, 0xffff_ffffu32, bytes]);
    let mut i = 0i32;
    while i < e.mem.i32(t + 0x68) {
        let name = e.mem.u32(e.mem.u32(t + 0x64).wrapping_add((i as u32) << 2));
        let data = e.mem.u32(array);
        let element = data.wrapping_add((i as u32).wrapping_mul(0x30));
        e.call(STRING_COPY_N, &args![element, name, 0x13u32]);
        i += 1;
    }
}

// Translated from 00dbc860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::saveFileHeader` (Xbox PDB), a register-input function: `EAX` =
/// `options` (word 0), `EDI` = `writer` (word 1, the platform object writer at the context's
/// `+0x20`), then the stack words `stream` and `section_count` (the caller removes them).
/// Builds the 0x40-byte header on its stack, filled with `0xff`: `0x57e0e057`, `0x10c0c010`,
/// `options[0]`, 7, the four bytes at `options + 4`, `section_count` at `+0x14`, at `+0x28` the
/// version text (`options[0xc]` copied with `strncpy(.., 0x10)`, or `00dbda20(buffer, 0x10)`
/// when that pointer is null) and an `0xff` tail at `+0x38`. Then calls slot `0xc` of `writer`
/// with `(stream, header, 0x0126c6c8, an empty relocation info)` and destroys the info.
pub fn save_file_header(
    e: &mut Engine,
    options: u32,
    writer: u32,
    stream: u32,
    section_count: u32,
) {
    e.with_stack(0x74, |e, frame| {
        let info = frame.addr();
        let header = info + 0x34;
        e.call(MEM_SET, &args![header, 0xffff_ffffu32, 0x40u32]);
        e.mem.set_u32(header, 0x57e0_e057);
        e.mem.set_u32(header + 4, 0x10c0_c010);
        e.mem.set_u8(header + 0x28, 0);
        let first = e.mem.u32(options);
        e.mem.set_u32(header + 8, first);
        e.mem.set_u32(header + 0xc, 7);
        e.call(MEM_COPY, &args![header + 0x10, options + 4, 4u32]);
        e.mem.set_u32(header + 0x14, section_count);
        e.call(MEM_SET, &args![header + 0x38, 0xffff_ffffu32, 8u32]);
        let version = e.mem.u32(options + 0xc);
        if version == 0 {
            e.call(VERSION_TEXT, &args![header + 0x28, 0x10u32]);
        } else {
            e.call(STRING_COPY_N, &args![header + 0x28, version, 0x10u32]);
        }
        store_empty_relocation_info(e, info);
        e.vcall(writer, 0xc, &args![stream, header, FILE_HEADER_CLASS, info]);
        e.call(RELOCATION_INFO_DESTRUCT, &args![info]);
    });
}

// Translated from 00dbcdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::padUp` (Xbox PDB), a register-input function: `EAX` = `alignment`
/// (word 0), `EDI` = `stream` (word 1). Reads the stream position (slot `0x20`), builds a
/// 32-byte-buffer array on its stack (capacity word `0x80000020`), reserves it to `alignment`
/// bytes when that exceeds 32, sets its size to `alignment`, fills the bytes that are new with
/// `0xff` through `_memset`, and when `(alignment - 1) & position` is non-zero writes the array
/// through slot `0x10` with `alignment` minus that remainder bytes. Finally releases the array
/// when it is owned.
pub fn pad_up(e: &mut Engine, alignment: u32, stream: u32) {
    let position = e.vcall(stream, 0x20, &args![]).u32();
    e.with_stack(0x2c, |e, local| {
        let array = local.addr();
        e.mem.set_u32(array, array + 0xc);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0020);
        reserve_at_least(e, array, alignment as i32, 1);
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        let count = (alignment as i32).wrapping_sub(size as i32);
        if count > 0 {
            e.call(
                CRT_MEM_SET,
                &args![size.wrapping_add(data), 0xffff_ffffu32, count as u32],
            );
        }
        let remainder = alignment.wrapping_sub(1) & position;
        e.mem.set_u32(array + 4, alignment);
        if remainder != 0 {
            let data = e.mem.u32(array);
            e.vcall(
                stream,
                0x10,
                &args![data, alignment.wrapping_sub(remainder)],
            );
        }
        let allocator = router_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        let data = e.mem.u32(array);
        release_owned(e, allocator, data, capacity, 1);
    });
}

// Translated from 00dbc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::doDeferredWrites` (Xbox PDB): `this` is ECX, `context` the save
/// context and `position` the stream offset to seek back to. Builds an archive writer on the
/// stream (swap byte `context[0x4c]`). For each section `i` of the context (count at `+8`,
/// 48-byte records at `+4`, relocation infos of 52 bytes at `+0x40`): turns the info's second
/// array (16-byte items) into 12-byte `(offset, entry word, entry word)` triples for the
/// items whose key the map at `this + 0x14` resolves (default -3) to a non-negative index into
/// the context's 8-byte entries (at `+0x10`) whose first word is not negative, seeks the stream
/// to the section's `+0x14` plus `+0x1c` and writes the triples as a 32-bit array; then turns
/// the info's third array (8-byte items) into `(offset, 0, string map value)` triples
/// (the string map at `context + 0x30`, default -1), seeks to the section's `+0x14` plus
/// `+0x20` and writes them. Then seeks to `position` and, for each section, writes the section
/// record with the platform writer (slot `0xc`, class `0x0126c698`, empty relocation info).
/// Finally seeks to `0x18`, builds a second archive writer swapping when the second byte of
/// the writer's layout rules differs from the host flag, and writes the entry at index
/// `this[0x8c]` (both words), a zero, and the string map value for the name of the
/// first word of the 24-byte element at `this[0x90]` of the array at `this + 8`.
pub fn do_deferred_writes(e: &mut Engine, this: Ptr, context: u32, position: u32) {
    let t = this.addr();
    e.with_stack(0xa0, |e, frame| {
        let f = frame.addr();
        let array_a = f + 0x20;
        let array_b = f + 0x2c;
        let archive = f + 0x8c;
        let swap = e.mem.u8(context + 0x4c);
        let stream = e.mem.u32(context + 0x1c);
        e.call(ARCHIVE_CONSTRUCT, &args![archive, stream, u32::from(swap)]);
        let mut section_index = 0i32;
        if e.mem.i32(context + 8) > 0 {
            let mut section_offset = 0u32;
            let mut info_offset = 0u32;
            loop {
                let item_array = e.mem.u32(context + 0x40).wrapping_add(info_offset) + 0xc;
                let section = e.mem.u32(context + 4).wrapping_add(section_offset);
                e.mem.set_u32(array_a, 0);
                e.mem.set_u32(array_a + 4, 0);
                e.mem.set_u32(array_a + 8, NOT_OWNED);
                let wanted = e.mem.i32(item_array + 4).wrapping_mul(3);
                reserve_at_least(e, array_a, wanted, 4);
                let size = e.mem.i32(array_a + 4);
                let data = e.mem.u32(array_a);
                let fill = wanted.wrapping_sub(size);
                if fill > 0 {
                    let first = data.wrapping_add((size as u32) << 2);
                    for k in 0..fill as u32 {
                        e.mem.set_u32(first + (k << 2), 0xffff_ffff);
                    }
                }
                e.mem.set_u32(array_a + 4, wanted as u32);
                if e.mem.i32(item_array + 4) > 0 {
                    let mut written = 0u32;
                    let mut offset = 0u32;
                    let mut j = 0i32;
                    loop {
                        let items = e.mem.u32(item_array);
                        let key = e.mem.u32(items.wrapping_add(offset) + 4);
                        let index = e
                            .call(MAP_GET_WITH_DEFAULT, &args![t + 0x14, key, -3i32])
                            .i32();
                        if index >= 0 {
                            let entries = e.mem.u32(context + 0x10);
                            let entry = entries.wrapping_add((index as u32) << 3);
                            if e.mem.i32(entry) >= 0 {
                                let items = e.mem.u32(item_array);
                                let relative = e
                                    .mem
                                    .u32(items.wrapping_add(offset))
                                    .wrapping_sub(e.mem.u32(section + 0x14));
                                let out = e.mem.u32(array_a).wrapping_add(written);
                                e.mem.set_u32(out, relative);
                                let entries = e.mem.u32(context + 0x10);
                                let entry = entries.wrapping_add((index as u32) << 3);
                                let first = e.mem.u32(entry);
                                e.mem.set_u32(out + 4, first);
                                let entries = e.mem.u32(context + 0x10);
                                let entry = entries.wrapping_add((index as u32) << 3);
                                let second = e.mem.u32(entry + 4);
                                e.mem.set_u32(out + 8, second);
                                written = written.wrapping_add(0xc);
                            }
                        }
                        j += 1;
                        offset = offset.wrapping_add(0x10);
                        if j >= e.mem.i32(item_array + 4) {
                            break;
                        }
                    }
                }
                let target = e
                    .mem
                    .u32(section + 0x1c)
                    .wrapping_add(e.mem.u32(section + 0x14));
                let stream = e.mem.u32(context + 0x1c);
                e.vcall(stream, 0x1c, &args![target, 0u32]);
                let data = e.mem.u32(array_a);
                let size = e.mem.u32(array_a + 4);
                e.call(ARCHIVE_WRITE_ARRAY_32, &args![archive, data, size]);
                let allocator = router_allocator(e);
                let capacity = e.mem.u32(array_a + 8);
                e.mem.set_u32(array_a + 4, 0);
                let data = e.mem.u32(array_a);
                release_owned(e, allocator, data, capacity, 4);

                let item_array = e.mem.u32(context + 0x40).wrapping_add(info_offset) + 0x18;
                let section = e.mem.u32(context + 4).wrapping_add(section_offset);
                e.mem.set_u32(array_b, 0);
                e.mem.set_u32(array_b + 4, 0);
                e.mem.set_u32(array_b + 8, NOT_OWNED);
                let wanted = e.mem.i32(item_array + 4).wrapping_mul(3);
                e.mem.set_u32(array_a, 0);
                e.mem.set_u32(array_a + 8, NOT_OWNED);
                reserve_at_least(e, array_b, wanted, 4);
                e.mem.set_u32(array_b + 4, wanted as u32);
                if e.mem.i32(item_array + 4) > 0 {
                    let mut offset = 0u32;
                    let mut j = 0i32;
                    loop {
                        let items = e.mem.u32(item_array);
                        let relative = e
                            .mem
                            .u32(items.wrapping_add(j as u32 * 8))
                            .wrapping_sub(e.mem.u32(section + 0x14));
                        let out = e.mem.u32(array_b).wrapping_add(offset);
                        e.mem.set_u32(out, relative);
                        e.mem.set_u32(out + 4, 0);
                        let items = e.mem.u32(item_array);
                        let name = e.mem.u32(items.wrapping_add(j as u32 * 8) + 4);
                        let value = e
                            .call(
                                STRING_MAP_GET_WITH_DEFAULT,
                                &args![context + 0x30, name, -1i32],
                            )
                            .u32();
                        let out = e.mem.u32(array_b).wrapping_add(offset);
                        e.mem.set_u32(out + 8, value);
                        j += 1;
                        offset = offset.wrapping_add(0xc);
                        if j >= e.mem.i32(item_array + 4) {
                            break;
                        }
                    }
                }
                let target = e
                    .mem
                    .u32(section + 0x20)
                    .wrapping_add(e.mem.u32(section + 0x14));
                let stream = e.mem.u32(context + 0x1c);
                e.vcall(stream, 0x1c, &args![target, 0u32]);
                let data = e.mem.u32(array_b);
                let size = e.mem.u32(array_b + 4);
                e.call(ARCHIVE_WRITE_ARRAY_32, &args![archive, data, size]);
                let allocator = router_allocator(e);
                let capacity = e.mem.u32(array_b + 8);
                e.mem.set_u32(array_b + 4, 0);
                let data = e.mem.u32(array_b);
                release_owned(e, allocator, data, capacity, 4);

                section_index += 1;
                section_offset = section_offset.wrapping_add(0x30);
                info_offset = info_offset.wrapping_add(0x34);
                e.mem.set_u32(array_b, 0);
                e.mem.set_u32(array_b + 8, NOT_OWNED);
                if section_index >= e.mem.i32(context + 8) {
                    break;
                }
            }
        }
        let stream = e.mem.u32(context + 0x1c);
        e.vcall(stream, 0x1c, &args![position, 0u32]);
        section_index = 0;
        if e.mem.i32(context + 8) > 0 {
            let mut offset = 0u32;
            let writer = context + 0x20;
            loop {
                let section = e.mem.u32(context + 4).wrapping_add(offset);
                let info = f + 0x58;
                store_empty_relocation_info(e, info);
                let stream = e.mem.u32(context + 0x1c);
                e.vcall(
                    writer,
                    0xc,
                    &args![stream, section, SECTION_HEADER_CLASS, info],
                );
                e.call(RELOCATION_INFO_DESTRUCT, &args![info]);
                section_index += 1;
                offset = offset.wrapping_add(0x30);
                if section_index >= e.mem.i32(context + 8) {
                    break;
                }
            }
        }
        let stream = e.mem.u32(context + 0x1c);
        e.vcall(stream, 0x1c, &args![0x18u32, 0u32]);
        let layout = e
            .call(PLATFORM_WRITER_GET_LAYOUT, &args![context + 0x20])
            .u32();
        let flag = (e.mem.u32(layout) >> 8) as u8;
        let swap = u32::from(flag != e.mem.u8(HOST_ENDIAN_FLAG));
        let stream = e.mem.u32(context + 0x1c);
        let second = f + 0x48;
        e.call(ARCHIVE_CONSTRUCT, &args![second, stream, swap]);
        let entry_index = e.mem.u32(t + 0x8c);
        let entries = e.mem.u32(context + 0x10);
        let word = e.mem.u32(entries.wrapping_add(entry_index << 3));
        e.call(ARCHIVE_WRITE_32, &args![second, word]);
        let entry_index = e.mem.u32(t + 0x8c);
        let entries = e.mem.u32(context + 0x10);
        let word = e.mem.u32(entries.wrapping_add(entry_index << 3) + 4);
        e.call(ARCHIVE_WRITE_32, &args![second, word]);
        let pending_index = e.mem.u32(t + 0x90);
        let pending = e.mem.u32(t + 8);
        let first = e
            .mem
            .u32(pending.wrapping_add(pending_index.wrapping_mul(3) << 3));
        e.call(ARCHIVE_WRITE_32, &args![second, 0u32]);
        let name = e.call(CLASS_GET_NAME, &args![first]).u32();
        let value = e
            .call(
                STRING_MAP_GET_WITH_DEFAULT,
                &args![context + 0x30, name, -1i32],
            )
            .u32();
        e.call(ARCHIVE_WRITE_32, &args![second, value]);
        e.call(ARCHIVE_DESTRUCT, &args![second]);
        e.call(ARCHIVE_DESTRUCT, &args![archive]);
    });
}

// Translated from 00dbcea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::writeAllObjects` (Xbox PDB): `this` is ECX, `context` the save
/// context. Records the stream position in section 0's `+0x14`, writes the names of the four
/// class objects and of every class of the pending writes (24-byte elements at `this + 8`,
/// count at `+0xc`; class at `+4`) once each (a string map remembers the names), pads to 16,
/// and stores the position relative to section 0 in its `+0x18..+0x2c`. Resizes the context's
/// entry array (8 bytes each) and relocation info array (52 bytes each) to the pending count
/// and section count (destroying or constructing the infos). Then, for each section `s` from 1:
/// records the position, and (when the option byte `options[8]` is zero and `s` equals the
/// index of the "__types__" section, the entries of the pending writes in that section are set
/// to `(-1, -1)`; otherwise) writes every pending write whose section (`+0x10`) is `s`: its
/// entry becomes `(s, position)`, it is written with the platform writer (slot `0xc`) when it
/// has a class and as raw bytes (`+0` and size `+0x14`) when not. Then rebases the info's
/// first array by the section start, moves items of its second array that the map at
/// `this + 0x20` resolves (default 0) to the fourth array (as `(offset, value)`), writes the
/// arrays with an archive writer (padding to 16), seeking past the second and third array
/// items, swaps the arrays into the context's info, writes the exported names (map at
/// `this + 0x2c`; an offset, the name, padded to 4) of the section and the imported names,
/// each group ended by `-1` and padded to 16, recording a position after each part.
pub fn write_all_objects(e: &mut Engine, this: Ptr, context: u32) {
    let t = this.addr();
    let section0 = e.mem.u32(context + 4);
    let stream = e.mem.u32(context + 0x1c);
    let tell = e.vcall(stream, 0x20, &args![]).u32();
    e.mem.set_u32(section0 + 0x14, tell);
    e.with_stack(0x90, |e, frame| {
        let f = frame.addr();
        let names = f + 0x48;
        let info = f + 0x58;
        let archive = f + 0x38;
        let flag = f + 0x13;
        e.call(STRING_MAP_CONSTRUCT, &args![names, 0u32]);
        for class in CLASS_OBJECTS {
            let name = e.call(CLASS_GET_NAME, &args![class]).u32();
            e.call(STRING_MAP_INSERT, &args![names, name, 1u32]);
            let base = e.mem.u32(section0 + 0x14);
            write_class_name(e, t, context, class, base);
        }
        if e.mem.i32(t + 0xc) > 0 {
            let mut j = 0i32;
            let mut offset = 0u32;
            loop {
                let pending = e.mem.u32(t + 8);
                let class = e.mem.u32(pending.wrapping_add(offset) + 4);
                let name = e.call(CLASS_GET_NAME, &args![class]).u32();
                let found = e.call(STRING_MAP_FIND_KEY, &args![names, name]).u32();
                e.call(STRING_MAP_IS_VALID, &args![names, flag, found]);
                if e.mem.u8(flag) == 0 {
                    let pending = e.mem.u32(t + 8);
                    let class = e.mem.u32(pending.wrapping_add(offset) + 4);
                    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
                    e.call(STRING_MAP_INSERT, &args![names, name, 1u32]);
                    let pending = e.mem.u32(t + 8);
                    let base = e.mem.u32(section0 + 0x14);
                    let class = e.mem.u32(pending.wrapping_add(offset) + 4);
                    write_class_name(e, t, context, class, base);
                }
                j += 1;
                offset = offset.wrapping_add(0x18);
                if j >= e.mem.i32(t + 0xc) {
                    break;
                }
            }
        }
        let stream = e.mem.u32(context + 0x1c);
        pad_up(e, 0x10, stream);
        let stream = e.mem.u32(context + 0x1c);
        let tell = e.vcall(stream, 0x20, &args![]).u32();
        let relative = tell.wrapping_sub(e.mem.u32(section0 + 0x14));
        for field in [0x18u32, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            e.mem.set_u32(section0 + field, relative);
        }
        e.call(STRING_MAP_DESTRUCT, &args![names]);

        let pending_count = e.mem.i32(t + 0xc);
        reserve_at_least(e, context + 0x10, pending_count, 8);
        e.mem.set_u32(context + 0x14, pending_count as u32);
        let section_count = e.mem.i32(context + 8);
        reserve_at_least(e, context + 0x40, section_count, 0x34);
        // Destroy the infos past the new count, then construct the missing ones.
        let size = e.mem.i32(context + 0x44);
        let mut surplus = size.wrapping_sub(section_count);
        let mut cursor = e
            .mem
            .u32(context + 0x40)
            .wrapping_add((section_count as u32).wrapping_mul(0x34));
        if surplus > 0 {
            loop {
                e.call(RELOCATION_INFO_DESTRUCT, &args![cursor]);
                cursor = cursor.wrapping_add(0x34);
                surplus -= 1;
                if surplus == 0 {
                    break;
                }
            }
        }
        let size = e.mem.i32(context + 0x44);
        let mut missing = section_count.wrapping_sub(size);
        let mut element = e
            .mem
            .u32(context + 0x40)
            .wrapping_add((size as u32).wrapping_mul(0x34));
        if missing > 0 {
            loop {
                // The exe skips a null element (placement new of a null pointer).
                if element != 0 {
                    store_empty_relocation_info(e, element);
                }
                element = element.wrapping_add(0x34);
                missing -= 1;
                if missing == 0 {
                    break;
                }
            }
        }
        e.mem.set_u32(context + 0x44, section_count as u32);
        let tag = e.mem.u32(TYPES_TAG_POINTER);
        let types_index = e.call(SECTION_INDEX, &args![t, tag]).i32();
        let mut s = 1i32;
        if e.mem.i32(context + 8) > 1 {
            let mut info_offset = 0x34u32;
            let mut section_offset = 0x30u32;
            loop {
                let section = e.mem.u32(context + 4).wrapping_add(section_offset);
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                e.mem.set_u32(section + 0x14, tell);
                let options = e.mem.u32(context);
                store_empty_relocation_info(e, info);
                if e.mem.u8(options + 8) == 0 && s == types_index {
                    // The pending writes of the types section get the entry (-1, -1).
                    let mut k = 0i32;
                    if e.mem.i32(t + 0xc) > 0 {
                        let mut offset = 0u32;
                        loop {
                            let pending = e.mem.u32(t + 8);
                            if e.mem.i32(pending.wrapping_add(offset) + 0x10) == types_index {
                                let entries = e.mem.u32(context + 0x10);
                                e.mem
                                    .set_u32(entries.wrapping_add((k as u32) << 3), 0xffff_ffff);
                                let entries = e.mem.u32(context + 0x10);
                                e.mem.set_u32(
                                    entries.wrapping_add((k as u32) << 3) + 4,
                                    0xffff_ffff,
                                );
                            }
                            k += 1;
                            offset = offset.wrapping_add(0x18);
                            if k >= e.mem.i32(t + 0xc) {
                                break;
                            }
                        }
                    }
                } else if e.mem.i32(t + 0xc) > 0 {
                    let mut k = 0i32;
                    let mut offset = 0u32;
                    loop {
                        let pending = e.mem.u32(t + 8).wrapping_add(offset);
                        if e.mem.i32(pending + 0x10) == s {
                            let class = e.mem.u32(pending + 4);
                            let object = e.mem.u32(pending);
                            let entries = e.mem.u32(context + 0x10);
                            let entry_offset = (k as u32) << 3;
                            e.mem.set_u32(entries.wrapping_add(entry_offset), s as u32);
                            let entry = e.mem.u32(context + 0x10).wrapping_add(entry_offset);
                            let stream = e.mem.u32(context + 0x1c);
                            let tell = e.vcall(stream, 0x20, &args![]).u32();
                            let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                            e.mem.set_u32(entry + 4, relative);
                            let stream = e.mem.u32(context + 0x1c);
                            if class != 0 {
                                e.vcall(context + 0x20, 0xc, &args![stream, object, class, info]);
                            } else {
                                let size = e.mem.u32(pending + 0x14);
                                let data = e.mem.u32(pending);
                                e.vcall(stream, 0x10, &args![data, size]);
                            }
                        }
                        k += 1;
                        offset = offset.wrapping_add(0x18);
                        if k >= e.mem.i32(t + 0xc) {
                            break;
                        }
                    }
                }
                // Rebase the first array by the section start.
                let mut index = 0i32;
                while index < e.mem.i32(info + 4) {
                    let data = e.mem.u32(info);
                    let at = data.wrapping_add((index as u32) << 3);
                    let rebased = e.mem.u32(at).wrapping_sub(e.mem.u32(section + 0x14));
                    e.mem.set_u32(at, rebased);
                    let data = e.mem.u32(info);
                    let at = data.wrapping_add((index as u32) << 3) + 4;
                    let rebased = e.mem.u32(at).wrapping_sub(e.mem.u32(section + 0x14));
                    e.mem.set_u32(at, rebased);
                    index += 1;
                }
                // Move the second array's items that the map resolves into the fourth array.
                let mut i = e.mem.i32(info + 0x10).wrapping_sub(1);
                if i >= 0 {
                    let mut offset = (i as u32) << 4;
                    loop {
                        let data = e.mem.u32(info + 0xc);
                        let key = e.mem.u32(data.wrapping_add(offset) + 4);
                        let value = e
                            .call(MAP_GET_WITH_DEFAULT, &args![t + 0x20, key, 0u32])
                            .u32();
                        if value != 0 {
                            let item = e.mem.u32(info + 0xc).wrapping_add(offset);
                            let allocator = router_allocator(e);
                            let capacity = e.mem.u32(info + 0x2c) & CAPACITY_MASK;
                            if e.mem.u32(info + 0x28) == capacity {
                                e.call(ARRAY_RESERVE_MORE, &args![allocator, info + 0x24, 8u32]);
                            }
                            let count = e.mem.u32(info + 0x28);
                            let slot = e.mem.u32(info + 0x24).wrapping_add(count << 3);
                            e.mem.set_u32(info + 0x28, count.wrapping_add(1));
                            let relative = e.mem.u32(item).wrapping_sub(e.mem.u32(section + 0x14));
                            e.mem.set_u32(slot + 4, value);
                            e.mem.set_u32(slot, relative);
                            let remaining = e.mem.i32(info + 0x10).wrapping_sub(1);
                            e.mem.set_u32(info + 0x10, remaining as u32);
                            if remaining != i {
                                // Overwrite the item with the last one (four words).
                                let data = e.mem.u32(info + 0xc);
                                let source = ((remaining as u32) << 4).wrapping_add(data);
                                let target = data.wrapping_add(offset);
                                for word in 0..4u32 {
                                    let moved = e.mem.u32(source + (word << 2));
                                    e.mem.set_u32(target + (word << 2), moved);
                                }
                            }
                        }
                        i -= 1;
                        offset = offset.wrapping_sub(0x10);
                        if i < 0 {
                            break;
                        }
                    }
                }
                let swap = e.mem.u8(context + 0x4c);
                let stream = e.mem.u32(context + 0x1c);
                e.call(ARCHIVE_CONSTRUCT, &args![archive, stream, u32::from(swap)]);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x18, relative);
                let data = e.mem.u32(info);
                let count = e.mem.u32(info + 4).wrapping_add(e.mem.u32(info + 4));
                e.call(ARCHIVE_WRITE_ARRAY, &args![archive, data, 4u32, count]);
                let stream = e.mem.u32(context + 0x1c);
                pad_up(e, 0x10, stream);
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x1c, relative);
                let skip = e.mem.u32(info + 0x10).wrapping_mul(12);
                e.vcall(stream, 0x1c, &args![skip, 1u32]);
                let infos = e.mem.u32(context + 0x40);
                e.call(
                    ARRAY_SWAP,
                    &args![infos.wrapping_add(info_offset) + 0xc, info + 0xc],
                );
                let stream = e.mem.u32(context + 0x1c);
                pad_up(e, 0x10, stream);
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x20, relative);
                let skip = e.mem.u32(info + 0x1c).wrapping_mul(12);
                e.vcall(stream, 0x1c, &args![skip, 1u32]);
                let infos = e.mem.u32(context + 0x40);
                e.call(
                    ARRAY_SWAP,
                    &args![infos.wrapping_add(info_offset) + 0x18, info + 0x18],
                );
                let stream = e.mem.u32(context + 0x1c);
                pad_up(e, 0x10, stream);
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x24, relative);
                // The exported names: the offset of each exported object of this section, then
                // its name, padded to 4.
                let mask = e.mem.i32(t + 0x34);
                let mut slot = 0i32;
                if mask >= 0 {
                    let mut at = e.mem.u32(t + 0x2c);
                    loop {
                        if e.mem.u32(at) != 0xffff_ffff {
                            break;
                        }
                        slot += 1;
                        at = at.wrapping_add(8);
                        if slot > mask {
                            break;
                        }
                    }
                }
                let mut exported = false;
                if slot <= mask {
                    loop {
                        let exports = e.mem.u32(t + 0x2c);
                        let key = e.mem.u32(exports.wrapping_add((slot as u32) << 3));
                        let index = e
                            .call(MAP_GET_WITH_DEFAULT, &args![t + 0x14, key, -1i32])
                            .i32();
                        if index >= 0 {
                            let pending = e.mem.u32(t + 8);
                            let at = pending.wrapping_add((index as u32).wrapping_mul(0x18));
                            if e.mem.i32(at + 0x10) == s {
                                let exports = e.mem.u32(t + 0x2c);
                                let name = e.mem.u32(exports.wrapping_add((slot as u32) << 3) + 4);
                                let entries = e.mem.u32(context + 0x10);
                                let offset =
                                    e.mem.u32(entries.wrapping_add((index as u32) << 3) + 4);
                                exported = true;
                                e.call(ARCHIVE_WRITE_32, &args![archive, offset]);
                                let length = e.call(STRING_LENGTH, &args![name]).u32();
                                e.call(
                                    ARCHIVE_WRITE_RAW,
                                    &args![archive, name, length.wrapping_add(1)],
                                );
                                let stream = e.mem.u32(context + 0x1c);
                                pad_up(e, 4, stream);
                            }
                        }
                        let mask = e.mem.i32(t + 0x34);
                        let mut next = slot.wrapping_add(1);
                        if next <= mask {
                            let mut at = e.mem.u32(t + 0x2c).wrapping_add((next as u32) << 3);
                            loop {
                                if e.mem.u32(at) != 0xffff_ffff {
                                    break;
                                }
                                next += 1;
                                at = at.wrapping_add(8);
                                if next > mask {
                                    break;
                                }
                            }
                        }
                        slot = next;
                        if slot > mask {
                            break;
                        }
                    }
                    if exported {
                        e.call(ARCHIVE_WRITE_32, &args![archive, 0xffff_ffffu32]);
                        let stream = e.mem.u32(context + 0x1c);
                        pad_up(e, 0x10, stream);
                    }
                }
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x28, relative);
                // The imported names collected in the fourth array.
                let mut count = e.mem.i32(info + 0x28);
                if count > 0 {
                    let mut b = 0u32;
                    loop {
                        let data = e.mem.u32(info + 0x24);
                        let item = data.wrapping_add(b << 3);
                        let offset = e.mem.u32(item);
                        e.call(ARCHIVE_WRITE_32, &args![archive, offset]);
                        let name = e.mem.u32(item + 4);
                        let length = e.call(STRING_LENGTH, &args![name]).u32();
                        e.call(
                            ARCHIVE_WRITE_RAW,
                            &args![archive, name, length.wrapping_add(1)],
                        );
                        let stream = e.mem.u32(context + 0x1c);
                        pad_up(e, 4, stream);
                        count = e.mem.i32(info + 0x28);
                        b += 1;
                        if b as i32 >= count {
                            break;
                        }
                    }
                }
                if count != 0 {
                    e.call(ARCHIVE_WRITE_32, &args![archive, 0xffff_ffffu32]);
                    let infos = e.mem.u32(context + 0x40);
                    e.call(
                        ARRAY_SWAP,
                        &args![infos.wrapping_add(info_offset) + 0x24, info + 0x24],
                    );
                    let stream = e.mem.u32(context + 0x1c);
                    pad_up(e, 0x10, stream);
                }
                let stream = e.mem.u32(context + 0x1c);
                let tell = e.vcall(stream, 0x20, &args![]).u32();
                let relative = tell.wrapping_sub(e.mem.u32(section + 0x14));
                e.mem.set_u32(section + 0x2c, relative);
                e.call(ARCHIVE_DESTRUCT, &args![archive]);
                e.call(RELOCATION_INFO_DESTRUCT, &args![info]);
                s += 1;
                section_offset = section_offset.wrapping_add(0x30);
                info_offset = info_offset.wrapping_add(0x34);
                if s >= e.mem.i32(context + 8) {
                    break;
                }
            }
        }
    });
}

// Translated from 00dbd620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryPackfileWriter::save` (Xbox PDB; virtual slot `0x1c`): `this` is ECX. Builds a save
/// context for `options` and a sub stream on `stream` (vtable `0x010d65ec`, the inner stream at
/// `+8`, its position at the start at `+0xc`), stores the sub stream as the context's stream,
/// fills the section tags, writes the file header (with the context's section count), seeks
/// the sub stream forward (mode 1) by the size of the section records, writes all objects,
/// notes the position, performs the deferred writes with the position taken before the seek,
/// seeks back to the noted position (mode 0), flushes the sub stream, sets the sub stream's
/// vtable to the base vtable and destroys the context. Answers 0.
pub fn save(e: &mut Engine, this: Ptr, stream: u32, options: u32) -> u32 {
    e.with_stack(0x70, |e, frame| {
        let f = frame.addr();
        let sub = f + 0xc;
        let context = f + 0x1c;
        save_context_save_context(e, Ptr::new(context), options);
        e.mem.set_u16(sub + 6, 1);
        e.mem.set_u32(sub, SUB_STREAM_VTABLE);
        e.mem.set_u32(sub + 8, stream);
        let start = e.vcall(stream, 0x20, &args![]).u32();
        e.mem.set_u32(context + 0x1c, sub);
        e.mem.set_u32(sub + 0xc, start);
        fill_section_tags(e, this, context);
        let section_count = e.mem.u32(context + 8);
        save_file_header(e, options, context + 0x20, sub, section_count);
        let after_header = e.vcall(sub, 0x20, &args![]).u32();
        let bytes = section_array_bytes_00dbd720(e, context + 4);
        e.vcall(sub, 0x1c, &args![bytes, 1u32]);
        write_all_objects(e, this, context);
        let end = e.vcall(sub, 0x20, &args![]).u32();
        do_deferred_writes(e, this, context, after_header);
        e.vcall(sub, 0x1c, &args![end, 0u32]);
        e.vcall(sub, 0x14, &args![]);
        e.mem.set_u32(sub, BASE_VTABLE);
        save_context_destructor(e, Ptr::new(context));
        0
    })
}

// Translated from 00dbd730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `isOk` (Xbox PDB name, class unconfirmed): `this` is ECX, `out` a byte pointer.
/// Calls slot `0xc` of the inner stream at `this + 8` with `out` and answers `out`.
pub fn sub_stream_is_ok(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 8);
    e.vcall(inner, 0xc, &args![out]);
    out
}

// Translated from 00dbd750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `write` (unnamed in the exe): `this` is ECX. Jumps to slot `0x10` of the inner
/// stream at `this + 8` with the stack words `data` and `size` and answers what it answers.
pub fn sub_stream_write(e: &mut Engine, this: Ptr, data: u32, size: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 8);
    e.vcall(inner, 0x10, &args![data, size]).u32()
}

// Translated from 00dbd760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `flush` (Xbox PDB name): `this` is ECX. Jumps to slot `0x14` of the inner stream
/// at `this + 8` and answers what it answers.
pub fn sub_stream_flush(e: &mut Engine, this: Ptr) -> u32 {
    let inner = e.mem.u32(this.addr() + 8);
    e.vcall(inner, 0x14, &args![]).u32()
}

// Translated from 00dbd770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `seekTellSupported` (Xbox PDB name): `this` is ECX, `out` a byte pointer. Calls
/// slot `0x18` of the inner stream at `this + 8` with `out` and answers `out`.
pub fn sub_stream_seek_tell_supported(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 8);
    e.vcall(inner, 0x18, &args![out]);
    out
}

// Translated from 00dbd790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `seek` (Xbox PDB name): `this` is ECX. Calls slot `0x1c` of the inner stream at
/// `this + 8` and answers what it answers: with `mode` 0 the offset is `this[0xc] + offset`
/// (relative to the start recorded in the sub stream) and the mode stays 0, otherwise `offset`
/// and `mode` pass unchanged.
pub fn sub_stream_seek(e: &mut Engine, this: Ptr, offset: u32, mode: u32) -> u32 {
    let t = this.addr();
    let inner = e.mem.u32(t + 8);
    if mode == 0 {
        let target = e.mem.u32(t + 0xc).wrapping_add(offset);
        e.vcall(inner, 0x1c, &args![target, mode]).u32()
    } else {
        e.vcall(inner, 0x1c, &args![offset, mode]).u32()
    }
}

// Translated from 00dbd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sub stream `tell` (Xbox PDB name): `this` is ECX. Answers slot `0x20` of the inner stream at
/// `this + 8` minus the start recorded at `this + 0xc`.
pub fn sub_stream_tell(e: &mut Engine, this: Ptr) -> u32 {
    let t = this.addr();
    let inner = e.mem.u32(t + 8);
    let position = e.vcall(inner, 0x20, &args![]).u32();
    position.wrapping_sub(e.mem.u32(t + 0xc))
}

// Translated from 00dbd820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `addLocal` (Xbox PDB name): `this` is ECX and points at an array of 8-byte pairs (data,
/// size, capacity word). Appends the pair `(from, to)`, growing the array through
/// `_reserveMore` (element size 8) when the size equals the capacity.
pub fn add_local(e: &mut Engine, this: Ptr, from: u32, to: u32) {
    let array = this.addr();
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size << 3);
    e.mem.set_u32(slot, from);
    e.mem.set_u32(slot + 4, to);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00dbd870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Relocation info constructor (unnamed in the exe): `this` is ECX. Stores four empty arrays
/// (data 0, size 0, capacity word `0x80000000`) at `+0`, `+0xc`, `+0x18` and `+0x24` and a zero
/// word at `+0x30`. Returns `this`.
pub fn relocation_info_construct_00dbd870(e: &mut Engine, this: Ptr) -> Ptr {
    store_empty_relocation_info(e, this.addr());
    this
}

// Translated from 00dbd8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveContext::SaveContext` (Xbox PDB): `this` is ECX, `options` the stack word. Stores
/// `options` at `+0`, an empty array at `+4` and at `+0x10` (capacity word `0x80000000`),
/// constructs the platform writer at `+0x20` with `options + 4`, the string map at `+0x30`
/// (with the dword `options & 0xffffff00`, whose low byte the exe sets to 0), an empty array at
/// `+0x40`, and the byte at `+0x4c`: 1 when the host endianness byte (`0x011b7b89`) differs
/// from the byte at `options + 5`. Returns `this`.
pub fn save_context_save_context(e: &mut Engine, this: Ptr, options: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, options);
    for array in [4u32, 0x10] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, NOT_OWNED);
    }
    e.call(PLATFORM_WRITER_CONSTRUCT, &args![t + 0x20, options + 4]);
    e.call(
        STRING_MAP_CONSTRUCT,
        &args![t + 0x30, options & 0xffff_ff00],
    );
    e.mem.set_u32(t + 0x40, 0);
    e.mem.set_u32(t + 0x44, 0);
    e.mem.set_u32(t + 0x48, NOT_OWNED);
    let host = e.mem.u8(HOST_ENDIAN_FLAG);
    let file = e.mem.u8(options + 5);
    e.mem.set_u8(t + 0x4c, u8::from(host != file));
    this
}

// Translated from 00dbd910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveContext::~SaveContext` (Xbox PDB): `this` is ECX. Destroys each relocation info of the
/// array at `+0x40` (52-byte elements, count at `+0x44`), sets its size to 0 and releases its
/// data through the allocator when owned, resets it to empty; destroys the string map at
/// `+0x30` and the platform writer at `+0x20`; then releases and resets the 8-byte entry array
/// at `+0x10` and the 48-byte section array at `+4` the same way.
pub fn save_context_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let allocator = router_allocator(e);
    let count = e.mem.i32(t + 0x44);
    let mut element = e.mem.u32(t + 0x40);
    if count > 0 {
        for _ in 0..count {
            e.call(RELOCATION_INFO_DESTRUCT, &args![element]);
            element = element.wrapping_add(0x34);
        }
    }
    let capacity = e.mem.u32(t + 0x48);
    e.mem.set_u32(t + 0x44, 0);
    let data = e.mem.u32(t + 0x40);
    release_owned(e, allocator, data, capacity, 0x34);
    e.call(STRING_MAP_DESTRUCT, &args![t + 0x30]);
    e.mem.set_u32(t + 0x40, 0);
    e.mem.set_u32(t + 0x48, NOT_OWNED);
    e.call(PLATFORM_WRITER_DESTRUCT, &args![t + 0x20]);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(t + 0x18);
    e.mem.set_u32(t + 0x14, 0);
    let data = e.mem.u32(t + 0x10);
    release_owned(e, allocator, data, capacity, 8);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x18, NOT_OWNED);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(t + 0xc);
    e.mem.set_u32(t + 8, 0);
    let data = e.mem.u32(t + 4);
    release_owned(e, allocator, data, capacity, 0x30);
    e.mem.set_u32(t + 0xc, NOT_OWNED);
    e.mem.set_u32(t + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00db_c670,
            hk_binary_packfile_writer_hk_binary_packfile_writer(Ptr) -> Ptr
        ),
        entry!(0x00db_c6e0, destructor_00dbc6e0(Ptr)),
        entry!(0x00db_c6f0, write_class_name(u32, u32, u32, u32)),
        entry!(0x00db_c7a0, fill_section_tags(Ptr, u32)),
        entry!(0x00db_c860, save_file_header(u32, u32, u32, u32)),
        entry!(0x00db_c960, do_deferred_writes(Ptr, u32, u32)),
        entry!(0x00db_cdd0, pad_up(u32, u32)),
        entry!(0x00db_cea0, write_all_objects(Ptr, u32)),
        entry!(0x00db_d620, save(Ptr, u32, u32) -> u32),
        entry!(0x00db_d720, section_array_bytes_00dbd720(u32) -> u32),
        entry!(0x00db_d730, sub_stream_is_ok(Ptr, u32) -> u32),
        entry!(0x00db_d750, sub_stream_write(Ptr, u32, u32) -> u32),
        entry!(0x00db_d760, sub_stream_flush(Ptr) -> u32),
        entry!(0x00db_d770, sub_stream_seek_tell_supported(Ptr, u32) -> u32),
        entry!(0x00db_d790, sub_stream_seek(Ptr, u32, u32) -> u32),
        entry!(0x00db_d7d0, sub_stream_tell(Ptr) -> u32),
        entry!(0x00db_d7f0, deleting_destructor_00dbd7f0(Ptr, u8) -> Ptr),
        entry!(0x00db_d820, add_local(Ptr, u32, u32)),
        entry!(0x00db_d870, relocation_info_construct_00dbd870(Ptr) -> Ptr),
        entry!(0x00db_d8a0, save_context_save_context(Ptr, u32) -> Ptr),
        entry!(0x00db_d910, save_context_destructor(Ptr)),
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

    #[test]
    fn deleting_destructor_00dbd7f0_frees_only_when_bit_zero_is_set() {
        check(0x00db_d7f0, 0x00db_c6e0);
    }
}

#[cfg(test)]
mod translated_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles behind the slots of a stream (`0xc` isOk, `0x10` write, `0x14` flush, `0x18`
    /// seekTellSupported, `0x1c` seek, `0x20` tell), of the platform object writer (`0xc`) and
    /// of the allocator (`0x10`, free).
    const SLOT_IS_OK: u32 = 0x0900_000c;
    const SLOT_WRITE: u32 = 0x0900_0010;
    const SLOT_FLUSH: u32 = 0x0900_0014;
    const SLOT_SUPPORTED: u32 = 0x0900_0018;
    const SLOT_SEEK: u32 = 0x0900_001c;
    const SLOT_TELL: u32 = 0x0900_0020;
    const SLOT_WRITE_OBJECT: u32 = 0x0900_0030;
    const ALLOCATOR_FREE: u32 = 0x0900_0110;

    type Writes = Rc<RefCell<Vec<Vec<u8>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
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

    /// A stream whose slots do nothing and answer 0, except `tell`, which answers `position`.
    fn stream(e: &mut Engine, position: u32) -> u32 {
        nothing(
            e,
            &[
                SLOT_IS_OK,
                SLOT_WRITE,
                SLOT_FLUSH,
                SLOT_SUPPORTED,
                SLOT_SEEK,
            ],
        );
        answer(e, SLOT_TELL, position);
        object_with_slots(
            e,
            &[
                (0xc, SLOT_IS_OK),
                (0x10, SLOT_WRITE),
                (0x14, SLOT_FLUSH),
                (0x18, SLOT_SUPPORTED),
                (0x1c, SLOT_SEEK),
                (0x20, SLOT_TELL),
            ],
        )
    }

    /// Replaces the write slot by a double that records the bytes it is given.
    fn record_writes(e: &mut Engine) -> Writes {
        let writes: Writes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&writes);
        e.register_double(SLOT_WRITE, move |e, a| {
            log.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        writes
    }

    /// The memory router with an allocator whose free slot is a double; answers the allocator.
    fn router(e: &mut Engine) -> u32 {
        let allocator = object_with_slots(e, &[(0x10, ALLOCATOR_FREE)]);
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        nothing(e, &[ALLOCATOR_FREE]);
        allocator
    }

    /// Doubles of the CRT functions that do what the CRT does.
    fn crt_doubles(e: &mut Engine) {
        for address in [MEM_SET, CRT_MEM_SET] {
            e.register_double(address, |e, a| {
                for i in 0..a[2] {
                    e.mem.set_u8(a[0] + i, a[1] as u8);
                }
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        e.register_double(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register_double(STRING_COPY_N, |e, a| {
            let text = e.mem.cstr(a[1]);
            for i in 0..a[2] {
                let byte = text.get(i as usize).copied().unwrap_or(0);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register_double(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
    }

    /// Doubles of `_reserve` and `_reserveMore` that move the data to a fresh block of the
    /// requested capacity (four more elements for `_reserveMore`).
    fn reserve_doubles(e: &mut Engine) {
        fn grow(e: &mut Engine, array: u32, capacity: u32, element: u32) {
            let block = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            let used = e.mem.u32(array + 4) * element;
            for i in 0..used {
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

    #[test]
    fn constructor_builds_on_the_base_and_registers_sections() {
        let mut e = engine();
        nothing(
            &mut e,
            &[
                PACKFILE_WRITER_CONSTRUCT,
                ADD_SECTION,
                SET_SECTION_FOR_CLASS,
            ],
        );
        e.mem.set_u32(CLASS_NAMES_TAG_POINTER, 0xa1a1);
        e.mem.set_u32(TYPES_TAG_POINTER, 0xb2b2);
        let this = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00db_c670, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010d_65bc);
        let order = ordered(
            &e,
            &[
                PACKFILE_WRITER_CONSTRUCT,
                ADD_SECTION,
                SET_SECTION_FOR_CLASS,
            ],
        );
        let mut expected = vec![
            (PACKFILE_WRITER_CONSTRUCT, vec![this]),
            (ADD_SECTION, vec![this, 0xa1a1]),
        ];
        for class in [0x0126_cae8, 0x0126_ca58, 0x0126_cab8, 0x0126_ca88] {
            expected.push((SET_SECTION_FOR_CLASS, vec![this, class, 0xb2b2]));
        }
        assert_eq!(order, expected);
    }

    #[test]
    fn destructor_stores_the_vtable_and_runs_the_base_destructor() {
        let mut e = engine();
        nothing(&mut e, &[PACKFILE_WRITER_DESTRUCT]);
        let this = e.mem.alloc(0x40);
        e.call(0x00db_c6e0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x010d_65bc);
        assert_eq!(calls_to(&e, PACKFILE_WRITER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn write_class_name_writes_the_signature_record_and_the_name() {
        for swap in [0u8, 1] {
            let mut e = engine();
            crt_doubles(&mut e);
            let stream = stream(&mut e, 100);
            let writes = record_writes(&mut e);
            answer(&mut e, CLASS_GET_SIGNATURE, 0x1122_3344);
            let name = e.mem.alloc(8);
            e.mem.set_cstr(name, b"hkFoo");
            answer(&mut e, CLASS_GET_NAME, name);
            nothing(&mut e, &[STRING_MAP_INSERT]);
            let context = e.mem.alloc(0x50);
            e.mem.set_u32(context + 0x1c, stream);
            e.mem.set_u8(context + 0x4c, swap);
            e.call(0x00db_c6f0, &args![0u32, context, 0x5000u32, 40u32]);
            let record = if swap == 0 {
                vec![0x44, 0x33, 0x22, 0x11, 9]
            } else {
                vec![0x11, 0x22, 0x33, 0x44, 9]
            };
            assert_eq!(*writes.borrow(), vec![record, b"hkFoo\0".to_vec()]);
            assert_eq!(calls_to(&e, CLASS_GET_SIGNATURE), vec![vec![0x5000, 0]]);
            assert_eq!(calls_to(&e, CLASS_GET_NAME), vec![vec![0x5000]]);
            assert_eq!(
                calls_to(&e, STRING_MAP_INSERT),
                vec![vec![context + 0x30, name, 60]]
            );
        }
    }

    #[test]
    fn section_array_bytes_multiplies_the_size_by_48() {
        let mut e = engine();
        let array = words(&mut e, &[0, 5, 0]);
        assert_eq!(e.call(0x00db_d720, &args![array]).u32(), 0xf0);
    }

    #[test]
    fn fill_section_tags_sizes_the_array_and_copies_the_names() {
        let mut e = engine();
        crt_doubles(&mut e);
        reserve_doubles(&mut e);
        let allocator = router(&mut e);
        let alpha = e.mem.alloc(8);
        e.mem.set_cstr(alpha, b"alpha");
        let long = e.mem.alloc(32);
        e.mem.set_cstr(long, b"a-name-longer-than-nineteen");
        let names = words(&mut e, &[alpha, long]);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x64, names);
        e.mem.set_u32(this + 0x68, 2);
        let context = e.mem.alloc(0x50);
        set_array(&mut e, context + 4, 0, 0, NOT_OWNED);
        e.call(0x00db_c7a0, &args![this, context]);
        assert_eq!(e.mem.u32(context + 8), 2);
        let data = e.mem.u32(context + 4);
        let mut first = b"alpha".to_vec();
        first.resize(0x13, 0);
        first.resize(0x30, 0xff);
        assert_eq!(e.mem.bytes(data, 0x30), first);
        let mut second = b"a-name-longer-than-".to_vec();
        second.resize(0x30, 0xff);
        assert_eq!(e.mem.bytes(data + 0x30, 0x30), second);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE),
            vec![vec![allocator, context + 4, 2, 0x30]]
        );
        assert_eq!(
            calls_to(&e, MEM_SET),
            vec![
                vec![data, 0, 4],
                vec![data + 0x30, 0, 4],
                vec![data, 0xffff_ffff, 0x60],
            ]
        );
    }

    #[test]
    fn save_file_header_builds_the_header_and_writes_it() {
        for version in [0u32, 1] {
            let mut e = engine();
            crt_doubles(&mut e);
            let seen: Writes = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&seen);
            let writer = object_with_slots(&mut e, &[(0xc, SLOT_WRITE_OBJECT)]);
            e.register_double(SLOT_WRITE_OBJECT, move |e, a| {
                log.borrow_mut().push(e.mem.bytes(a[2], 0x40));
                log.borrow_mut().push(e.mem.bytes(a[4], 0x34));
                Ret::default()
            });
            nothing(&mut e, &[RELOCATION_INFO_DESTRUCT]);
            e.register_double(VERSION_TEXT, |e, a| {
                e.mem.set_cstr(a[0], b"default");
                Ret::default()
            });
            let options = e.mem.alloc(0x20);
            e.mem.set_u32(options, 0x1234_5678);
            e.mem.write(options + 4, &[4, 1, 0, 1]);
            let text = e.mem.alloc(32);
            e.mem.set_cstr(text, b"abcdefghijklmnopqrst");
            e.mem
                .set_u32(options + 0xc, if version == 0 { 0 } else { text });
            e.call(0x00db_c860, &args![options, writer, 0x7777u32, 5u32]);
            let mut expected = Vec::new();
            expected.extend_from_slice(&0x57e0_e057u32.to_le_bytes());
            expected.extend_from_slice(&0x10c0_c010u32.to_le_bytes());
            expected.extend_from_slice(&0x1234_5678u32.to_le_bytes());
            expected.extend_from_slice(&7u32.to_le_bytes());
            expected.extend_from_slice(&[4, 1, 0, 1]);
            expected.extend_from_slice(&5u32.to_le_bytes());
            expected.resize(0x28, 0xff);
            if version == 0 {
                expected.extend_from_slice(b"default\0");
                expected.resize(0x40, 0xff);
            } else {
                expected.extend_from_slice(b"abcdefghijklmnop");
                expected.resize(0x40, 0xff);
            }
            let seen = seen.borrow();
            assert_eq!(seen[0], expected);
            let mut info = Vec::new();
            for _ in 0..4 {
                info.extend_from_slice(&[0u8; 8]);
                info.extend_from_slice(&0x8000_0000u32.to_le_bytes());
            }
            info.extend_from_slice(&[0u8; 4]);
            assert_eq!(seen[1], info);
            let write = &calls_to(&e, SLOT_WRITE_OBJECT)[0];
            assert_eq!(
                (write[0], write[1], write[3]),
                (writer, 0x7777, 0x0126_c6c8)
            );
            assert_eq!(calls_to(&e, RELOCATION_INFO_DESTRUCT), vec![vec![write[4]]]);
            if version == 0 {
                let call = &calls_to(&e, VERSION_TEXT)[0];
                assert_eq!(call[0], write[2] + 0x28);
                assert_eq!(call[1], 0x10);
                assert!(calls_to(&e, STRING_COPY_N).is_empty());
            } else {
                assert!(calls_to(&e, VERSION_TEXT).is_empty());
                assert_eq!(
                    calls_to(&e, STRING_COPY_N),
                    vec![vec![write[2] + 0x28, text, 0x10]]
                );
            }
        }
    }

    #[test]
    fn pad_up_writes_the_missing_bytes_only_when_misaligned() {
        let mut e = engine();
        crt_doubles(&mut e);
        reserve_doubles(&mut e);
        let allocator = router(&mut e);
        let stream = stream(&mut e, 5);
        let writes = record_writes(&mut e);
        // Misaligned by 5 within 16: writes the remaining 11 bytes of 0xff; the 32-byte
        // buffer is not owned, so nothing is released.
        e.call(0x00db_cdd0, &args![16u32, stream]);
        assert_eq!(*writes.borrow(), vec![vec![0xffu8; 11]]);
        assert!(calls_to(&e, ALLOCATOR_FREE).is_empty());
        assert!(calls_to(&e, ARRAY_RESERVE).is_empty());
        // Aligned: no write.
        answer(&mut e, SLOT_TELL, 32);
        e.call(0x00db_cdd0, &args![16u32, stream]);
        assert_eq!(writes.borrow().len(), 1);
        // Alignment 64 exceeds the buffer: the array is reserved to 64 bytes and released.
        answer(&mut e, SLOT_TELL, 3);
        e.call(0x00db_cdd0, &args![64u32, stream]);
        assert_eq!(writes.borrow()[1], vec![0xffu8; 61]);
        let reserve = &calls_to(&e, ARRAY_RESERVE)[0];
        assert_eq!((reserve[0], reserve[2], reserve[3]), (allocator, 64, 1));
        let free = &calls_to(&e, ALLOCATOR_FREE)[0];
        assert_eq!((free[0], free[2]), (allocator, 64));
    }

    type Dumps = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A sub stream over `inner` that started at `base`.
    fn sub_stream(e: &mut Engine, inner: u32, base: u32) -> u32 {
        let sub = e.mem.alloc(0x20);
        e.mem.set_u32(sub + 8, inner);
        e.mem.set_u32(sub + 0xc, base);
        sub
    }

    /// A double storing `value` into the byte whose address it gets and answering 0x55.
    fn store_byte(e: &mut Engine, address: u32, value: u8) {
        e.register_double(address, move |e, a| {
            e.mem.set_u8(a[1], value);
            Ret {
                eax: 0x55,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn sub_stream_is_ok_asks_the_inner_stream_and_answers_the_pointer() {
        let mut e = engine();
        let inner = stream(&mut e, 0);
        store_byte(&mut e, SLOT_IS_OK, 1);
        let sub = sub_stream(&mut e, inner, 0);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00db_d730, &args![sub, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, SLOT_IS_OK), vec![vec![inner, out]]);
    }

    #[test]
    fn sub_stream_write_forwards_and_answers_the_inner_answer() {
        let mut e = engine();
        let inner = stream(&mut e, 0);
        answer(&mut e, SLOT_WRITE, 9);
        let sub = sub_stream(&mut e, inner, 0);
        assert_eq!(e.call(0x00db_d750, &args![sub, 0x1000u32, 12u32]).u32(), 9);
        assert_eq!(calls_to(&e, SLOT_WRITE), vec![vec![inner, 0x1000, 12]]);
    }

    #[test]
    fn sub_stream_flush_forwards_and_answers_the_inner_answer() {
        let mut e = engine();
        let inner = stream(&mut e, 0);
        answer(&mut e, SLOT_FLUSH, 5);
        let sub = sub_stream(&mut e, inner, 0);
        assert_eq!(e.call(0x00db_d760, &args![sub]).u32(), 5);
        assert_eq!(calls_to(&e, SLOT_FLUSH), vec![vec![inner]]);
    }

    #[test]
    fn sub_stream_seek_tell_supported_asks_the_inner_stream_and_answers_the_pointer() {
        let mut e = engine();
        let inner = stream(&mut e, 0);
        store_byte(&mut e, SLOT_SUPPORTED, 1);
        let sub = sub_stream(&mut e, inner, 0);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00db_d770, &args![sub, out]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(calls_to(&e, SLOT_SUPPORTED), vec![vec![inner, out]]);
    }

    #[test]
    fn sub_stream_seek_adds_the_start_only_for_mode_zero() {
        let mut e = engine();
        let inner = stream(&mut e, 0);
        answer(&mut e, SLOT_SEEK, 3);
        let sub = sub_stream(&mut e, inner, 0x100);
        assert_eq!(e.call(0x00db_d790, &args![sub, 0x20u32, 0u32]).u32(), 3);
        assert_eq!(e.call(0x00db_d790, &args![sub, 0x20u32, 2u32]).u32(), 3);
        assert_eq!(
            calls_to(&e, SLOT_SEEK),
            vec![vec![inner, 0x120, 0], vec![inner, 0x20, 2]]
        );
    }

    #[test]
    fn sub_stream_tell_subtracts_the_start() {
        let mut e = engine();
        let inner = stream(&mut e, 0x300);
        let sub = sub_stream(&mut e, inner, 0x100);
        assert_eq!(e.call(0x00db_d7d0, &args![sub]).u32(), 0x200);
        assert_eq!(calls_to(&e, SLOT_TELL), vec![vec![inner]]);
    }

    #[test]
    fn add_local_stores_a_pair_and_grows_a_full_array() {
        let mut e = engine();
        reserve_doubles(&mut e);
        let allocator = router(&mut e);
        let data = e.mem.alloc(32);
        let array = e.mem.alloc(16);
        set_array(&mut e, array, data, 1, 4);
        e.call(0x00db_d820, &args![array, 0xaau32, 0xbbu32]);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert_eq!(dwords(&e, data + 8, 2), vec![0xaa, 0xbb]);
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
        // A full array grows through `_reserveMore` first.
        set_array(&mut e, array, data, 4, 4);
        e.call(0x00db_d820, &args![array, 0xccu32, 0xddu32]);
        assert_eq!(
            calls_to(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, array, 8]]
        );
        assert_eq!(e.mem.u32(array + 4), 5);
        let grown = e.mem.u32(array);
        assert_ne!(grown, data);
        assert_eq!(dwords(&e, grown + 32, 2), vec![0xcc, 0xdd]);
    }

    #[test]
    fn relocation_info_constructor_stores_four_empty_arrays() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        for i in 0..13 {
            e.mem.set_u32(this + i * 4, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00db_d870, &args![this]).u32(), this);
        let empty = [0, 0, 0x8000_0000];
        let mut expected = Vec::new();
        for _ in 0..4 {
            expected.extend_from_slice(&empty);
        }
        expected.push(0);
        assert_eq!(dwords(&e, this, 13), expected);
    }

    #[test]
    fn save_context_constructor_sets_up_the_empty_arrays() {
        let mut e = engine();
        nothing(&mut e, &[PLATFORM_WRITER_CONSTRUCT, STRING_MAP_CONSTRUCT]);
        let options = e.mem.alloc(0x20);
        e.mem.set_u8(options + 5, 1);
        e.mem.set_u8(HOST_ENDIAN_FLAG, 1);
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00db_d8a0, &args![this, options]).u32(), this);
        assert_eq!(e.mem.u32(this), options);
        for array in [4u32, 0x10, 0x40] {
            assert_eq!(dwords(&e, this + array, 3), vec![0, 0, 0x8000_0000]);
        }
        assert_eq!(e.mem.u8(this + 0x4c), 0);
        assert_eq!(
            calls_to(&e, PLATFORM_WRITER_CONSTRUCT),
            vec![vec![this + 0x20, options + 4]]
        );
        assert_eq!(
            calls_to(&e, STRING_MAP_CONSTRUCT),
            vec![vec![this + 0x30, options & 0xffff_ff00]]
        );
        // A different endianness byte sets the swap flag.
        e.mem.set_u8(options + 5, 0);
        e.call(0x00db_d8a0, &args![this, options]);
        assert_eq!(e.mem.u8(this + 0x4c), 1);
    }

    #[test]
    fn save_context_destructor_destroys_and_releases_the_arrays() {
        let mut e = engine();
        let allocator = router(&mut e);
        nothing(
            &mut e,
            &[
                RELOCATION_INFO_DESTRUCT,
                STRING_MAP_DESTRUCT,
                PLATFORM_WRITER_DESTRUCT,
            ],
        );
        let this = e.mem.alloc(0x60);
        let infos = e.mem.alloc(0x68);
        let entries = e.mem.alloc(16);
        let sections = e.mem.alloc(0x60);
        set_array(&mut e, this + 0x40, infos, 2, 2);
        set_array(&mut e, this + 0x10, entries, 2, 2);
        set_array(&mut e, this + 4, sections, 2, 2);
        e.call(0x00db_d910, &args![this]);
        assert_eq!(
            calls_to(&e, RELOCATION_INFO_DESTRUCT),
            vec![vec![infos], vec![infos + 0x34]]
        );
        assert_eq!(
            calls_to(&e, ALLOCATOR_FREE),
            vec![
                vec![allocator, infos, 0x68],
                vec![allocator, entries, 16],
                vec![allocator, sections, 0x60],
            ]
        );
        assert_eq!(calls_to(&e, STRING_MAP_DESTRUCT), vec![vec![this + 0x30]]);
        assert_eq!(
            calls_to(&e, PLATFORM_WRITER_DESTRUCT),
            vec![vec![this + 0x20]]
        );
        for array in [4u32, 0x10, 0x40] {
            assert_eq!(dwords(&e, this + array, 3), vec![0, 0, 0x8000_0000]);
        }
        // Data that is not owned is only forgotten.
        let mut e = engine();
        router(&mut e);
        nothing(
            &mut e,
            &[
                RELOCATION_INFO_DESTRUCT,
                STRING_MAP_DESTRUCT,
                PLATFORM_WRITER_DESTRUCT,
            ],
        );
        let this = e.mem.alloc(0x60);
        let infos = e.mem.alloc(0x34);
        set_array(&mut e, this + 0x40, infos, 1, NOT_OWNED | 1);
        set_array(&mut e, this + 0x10, 0, 0, NOT_OWNED);
        set_array(&mut e, this + 4, 0, 0, NOT_OWNED);
        e.call(0x00db_d910, &args![this]);
        assert_eq!(calls_to(&e, RELOCATION_INFO_DESTRUCT), vec![vec![infos]]);
        assert!(calls_to(&e, ALLOCATOR_FREE).is_empty());
        assert_eq!(dwords(&e, this + 0x40, 3), vec![0, 0, 0x8000_0000]);
    }

    #[test]
    fn do_deferred_writes_rewrites_the_relocations_and_the_section_records() {
        let mut e = engine();
        reserve_doubles(&mut e);
        let allocator = router(&mut e);
        let stream = stream(&mut e, 0x40);
        let dumps: Dumps = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&dumps);
        e.register_double(ARCHIVE_WRITE_ARRAY_32, move |e, a| {
            log.borrow_mut()
                .push((0..a[2]).map(|i| e.mem.u32(a[1] + i * 4)).collect());
            Ret::default()
        });
        nothing(
            &mut e,
            &[
                ARCHIVE_CONSTRUCT,
                ARCHIVE_WRITE_32,
                ARCHIVE_DESTRUCT,
                RELOCATION_INFO_DESTRUCT,
                SLOT_WRITE_OBJECT,
            ],
        );
        e.register_double(MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: match a[1] {
                0xa1 => 0,
                0xa2 => 5,
                _ => a[2],
            },
            ..Ret::default()
        });
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: match a[1] {
                0xb1 => 11,
                0xb2 => 12,
                0x7000 => 21,
                _ => a[2],
            },
            ..Ret::default()
        });
        answer(&mut e, CLASS_GET_NAME, 0x7000);
        let layout = words(&mut e, &[0x0000_0100]);
        answer(&mut e, PLATFORM_WRITER_GET_LAYOUT, layout);
        e.mem.set_u8(HOST_ENDIAN_FLAG, 0);
        let pending = words(&mut e, &[0x6100, 0, 0, 0, 0, 0]);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, pending);
        e.mem.set_u32(this + 0x8c, 1);
        e.mem.set_u32(this + 0x90, 0);
        let section = e.mem.alloc(0x30);
        e.mem.set_u32(section + 0x14, 0x1000);
        e.mem.set_u32(section + 0x1c, 0x20);
        e.mem.set_u32(section + 0x20, 0x40);
        let entries = e.mem.alloc(0x40);
        e.mem.set_u32(entries, 0x500);
        e.mem.set_u32(entries + 4, 0x600);
        e.mem.set_u32(entries + 8, 0x700);
        e.mem.set_u32(entries + 12, 0x800);
        e.mem.set_u32(entries + 40, 0xffff_ffff);
        e.mem.set_u32(entries + 44, 7);
        let first = words(
            &mut e,
            &[0x1030, 0xa1, 0, 0, 0x1040, 0xa2, 0, 0, 0x1050, 0xa3, 0, 0],
        );
        let second = words(&mut e, &[0x1050, 0xb1, 0x1060, 0xb2]);
        let info = e.mem.alloc(0x34);
        set_array(&mut e, info + 0xc, first, 3, 3);
        set_array(&mut e, info + 0x18, second, 2, 2);
        let context = e.mem.alloc(0x50);
        e.mem.set_u32(context + 4, section);
        e.mem.set_u32(context + 8, 1);
        e.mem.set_u32(context + 0x10, entries);
        e.mem.set_u32(context + 0x1c, stream);
        e.mem.set_u32(context + 0x40, info);
        e.mem.set_u32(context + 0x44, 1);
        let writer = object_with_slots(&mut e, &[(0xc, SLOT_WRITE_OBJECT)]);
        let writer_vtable = e.mem.u32(writer);
        e.mem.set_u32(context + 0x20, writer_vtable);
        e.call(0x00db_c960, &args![this, context, 0x77u32]);
        // The first array keeps only the item the map resolves to a usable entry; the
        // other slots stay -1. The second array gets the name values.
        let ff = 0xffff_ffffu32;
        assert_eq!(
            *dumps.borrow(),
            vec![
                vec![0x30, 0x500, 0x600, ff, ff, ff, ff, ff, ff],
                vec![0x50, 0, 11, 0x60, 0, 12],
            ]
        );
        let frees = calls_to(&e, ALLOCATOR_FREE);
        assert_eq!(frees.len(), 2);
        assert_eq!((frees[0][0], frees[0][2]), (allocator, 36));
        assert_eq!((frees[1][0], frees[1][2]), (allocator, 24));
        assert_eq!(
            calls_to(&e, SLOT_SEEK),
            vec![
                vec![stream, 0x1020, 0],
                vec![stream, 0x1040, 0],
                vec![stream, 0x77, 0],
                vec![stream, 0x18, 0],
            ]
        );
        let constructs = calls_to(&e, ARCHIVE_CONSTRUCT);
        assert_eq!((constructs[0][1], constructs[0][2]), (stream, 0));
        assert_eq!((constructs[1][1], constructs[1][2]), (stream, 1));
        let archive = constructs[1][0];
        assert_eq!(
            calls_to(&e, ARCHIVE_WRITE_32),
            vec![
                vec![archive, 0x700],
                vec![archive, 0x800],
                vec![archive, 0],
                vec![archive, 21],
            ]
        );
        let object = calls_to(&e, SLOT_WRITE_OBJECT);
        assert_eq!(object.len(), 1);
        assert_eq!(
            (object[0][0], object[0][1], object[0][2], object[0][3]),
            (context + 0x20, stream, section, 0x0126_c698)
        );
        assert_eq!(
            calls_to(&e, RELOCATION_INFO_DESTRUCT),
            vec![vec![object[0][4]]]
        );
        assert_eq!(
            calls_to(&e, ARCHIVE_DESTRUCT),
            vec![vec![archive], vec![constructs[0][0]]]
        );
        assert_eq!(calls_to(&e, PLATFORM_WRITER_GET_LAYOUT).len(), 1);
    }

    /// Everything `write_all_objects` and `save` call outside the unit as doubles, except the
    /// stream: a class name is the class pointer itself (a mapped block for the null class), the
    /// string map remembers the inserted names and the pointer map resolves the key 0xa1 to 0x77
    /// and 0xe1 to 1.
    fn object_doubles(e: &mut Engine) {
        crt_doubles(e);
        reserve_doubles(e);
        router(e);
        answer(e, STRING_LENGTH, 3);
        answer(e, CLASS_GET_SIGNATURE, 0);
        e.register_double(CLASS_GET_NAME, |_, a| Ret {
            eax: if a[0] == 0 { 0x0112_0000 } else { a[0] },
            ..Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::<u32>::new()));
        let log = Rc::clone(&seen);
        e.register_double(STRING_MAP_INSERT, move |_, a| {
            log.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.register_double(STRING_MAP_FIND_KEY, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register_double(STRING_MAP_IS_VALID, move |e, a| {
            e.mem.set_u8(a[1], u8::from(seen.borrow().contains(&a[2])));
            Ret::default()
        });
        nothing(
            e,
            &[
                STRING_MAP_CONSTRUCT,
                STRING_MAP_DESTRUCT,
                ARCHIVE_CONSTRUCT,
                ARCHIVE_WRITE_32,
                ARCHIVE_WRITE_RAW,
                ARCHIVE_WRITE_ARRAY,
                ARCHIVE_DESTRUCT,
                RELOCATION_INFO_DESTRUCT,
            ],
        );
        e.register_double(ARRAY_SWAP, |e, a| {
            for i in 0..3 {
                let mine = e.mem.u32(a[0] + i * 4);
                let other = e.mem.u32(a[1] + i * 4);
                e.mem.set_u32(a[0] + i * 4, other);
                e.mem.set_u32(a[1] + i * 4, mine);
            }
            Ret::default()
        });
        e.register_double(MAP_GET_WITH_DEFAULT, |_, a| Ret {
            eax: match a[1] {
                0xa1 => 0x77,
                0xe1 => 1,
                _ => a[2],
            },
            ..Ret::default()
        });
    }

    /// The object-writer slot double: gives the relocation info it is handed one relocation
    /// (0x1010 to 0x1020) and two imports (keys 0xa1 and 0xa2 at 0x2000 and 0x2010).
    fn info_writer(e: &mut Engine) {
        e.register_double(SLOT_WRITE_OBJECT, |e, a| {
            let info = a[4];
            let relocations = words(e, &[0x1010, 0x1020]);
            set_array(e, info, relocations, 1, 1);
            let imports = words(e, &[0x2000, 0xa1, 0, 0, 0x2010, 0xa2, 0, 0]);
            set_array(e, info + 0xc, imports, 2, 2);
            Ret::default()
        });
    }

    /// A writer with three pending writes (classes `0x01100000`, null and `0x01100000`, in
    /// sections 1, 1 and 0) and one exported object, a context with two sections whose option
    /// byte `options[8]` is `option_byte`, and a types section index of `types_index`. Answers
    /// `(this, context, stream)`.
    fn objects_fixture(e: &mut Engine, option_byte: u8, types_index: u32) -> (u32, u32, u32) {
        object_doubles(e);
        info_writer(e);
        answer(e, SECTION_INDEX, types_index);
        // The position counts up by 16 on every call, so every padding is a no-op.
        let stream = stream(e, 0);
        let tick = Rc::new(RefCell::new(0x100u32));
        e.register_double(SLOT_TELL, move |_, _| {
            let now = *tick.borrow();
            *tick.borrow_mut() += 0x10;
            Ret {
                eax: now,
                ..Ret::default()
            }
        });
        e.mem.write(0x0111_1000, &[1, 2, 3, 4]);
        let pending = words(
            e,
            &[
                0x0111_0000,
                0x0110_0000,
                0,
                0,
                1,
                8,
                0x0111_1000,
                0,
                0,
                0,
                1,
                4,
                0x0111_2000,
                0x0110_0000,
                0,
                0,
                0,
                2,
            ],
        );
        let exports = words(e, &[0xffff_ffff, 0, 0xe1, 0x0113_0000]);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, pending);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x2c, exports);
        e.mem.set_u32(this + 0x34, 1);
        let options = e.mem.alloc(0x10);
        e.mem.set_u8(options + 8, option_byte);
        let sections = e.mem.alloc(0x60);
        let context = e.mem.alloc(0x50);
        e.mem.set_u32(context, options);
        e.mem.set_u32(context + 4, sections);
        e.mem.set_u32(context + 8, 2);
        e.mem.set_u32(context + 0x1c, stream);
        set_array(e, context + 0x10, 0, 0, NOT_OWNED);
        set_array(e, context + 0x40, 0, 0, NOT_OWNED);
        let writer = object_with_slots(e, &[(0xc, SLOT_WRITE_OBJECT)]);
        let writer_vtable = e.mem.u32(writer);
        e.mem.set_u32(context + 0x20, writer_vtable);
        (this, context, stream)
    }

    #[test]
    fn write_all_objects_writes_the_class_names_and_the_objects_of_each_section() {
        let mut e = engine();
        let (this, context, stream) = objects_fixture(&mut e, 1, 7);
        let writes = record_writes(&mut e);
        let dumps: Dumps = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&dumps);
        e.register_double(ARCHIVE_WRITE_ARRAY, move |e, a| {
            log.borrow_mut()
                .push((0..a[3]).map(|i| e.mem.u32(a[1] + i * 4)).collect());
            Ret::default()
        });
        e.call(0x00db_cea0, &args![this, context]);
        let sections = e.mem.u32(context + 4);
        let entries = e.mem.u32(context + 0x10);
        assert_eq!(e.mem.u32(sections + 0x14), 0x100);
        let rest = e.mem.u32(sections + 0x18);
        for field in [0x1cu32, 0x20, 0x24, 0x28, 0x2c] {
            assert_eq!(e.mem.u32(sections + field), rest);
        }
        let start = e.mem.u32(sections + 0x30 + 0x14);
        assert_eq!(e.mem.u32(context + 0x14), 3);
        assert_eq!(e.mem.u32(context + 0x44), 2);
        // The entries of the two writes in section 1 are (1, position) one tick apart.
        assert_eq!(dwords(&e, entries, 2), vec![1, 0x10]);
        assert_eq!(dwords(&e, entries + 8, 2), vec![1, 0x20]);
        assert_eq!(dwords(&e, entries + 16, 2), vec![0, 0]);
        // Six distinct class names (the four fixed ones, the class and the null class) go into
        // the local name map and, as each is written as a record and a name, into the map of
        // the context; then the raw write of the second object follows.
        let inserts = calls_to(&e, STRING_MAP_INSERT);
        assert_eq!(inserts.len(), 12);
        assert_eq!(inserts.iter().filter(|w| w[0] == context + 0x30).count(), 6);
        let written = writes.borrow();
        assert_eq!(written.len(), 13);
        assert_eq!(written[12], vec![1, 2, 3, 4]);
        let object = calls_to(&e, SLOT_WRITE_OBJECT);
        assert_eq!(object.len(), 1);
        assert_eq!(
            (object[0][0], object[0][1], object[0][2], object[0][3]),
            (context + 0x20, stream, 0x0111_0000, 0x0110_0000)
        );
        // The relocations are rebased by the section start.
        assert_eq!(*dumps.borrow(), vec![vec![0x1010 - start, 0x1020 - start]]);
        // The import with key 0xa1 moved to the fourth array; the other stays in the second
        // array that the context's info now owns.
        let info = e.mem.u32(context + 0x40) + 0x34;
        assert_eq!(dwords(&e, info + 0x10, 1), vec![1]);
        let kept = e.mem.u32(info + 0xc);
        assert_eq!(dwords(&e, kept, 2), vec![0x2010, 0xa2]);
        let swaps = calls_to(&e, ARRAY_SWAP);
        assert_eq!(
            swaps.iter().map(|w| w[0]).collect::<Vec<_>>(),
            vec![info + 0xc, info + 0x18, info + 0x24]
        );
        // The exported object (offset of its entry, name) and the imports, each list closed
        // by -1.
        let archive = calls_to(&e, ARCHIVE_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, ARCHIVE_WRITE_32),
            vec![
                vec![archive, 0x20],
                vec![archive, 0xffff_ffff],
                vec![archive, 0x2000 - start],
                vec![archive, 0xffff_ffff],
            ]
        );
        assert_eq!(
            calls_to(&e, ARCHIVE_WRITE_RAW),
            vec![vec![archive, 0x0113_0000, 4], vec![archive, 0x77, 4]]
        );
        assert_eq!(
            calls_to(&e, SLOT_SEEK),
            vec![vec![stream, 12, 1], vec![stream, 0, 1]]
        );
        // With the types section and a zero option byte the entries of its writes are (-1, -1).
        let mut e = engine();
        let (this, context, _) = objects_fixture(&mut e, 0, 1);
        let writes = record_writes(&mut e);
        e.call(0x00db_cea0, &args![this, context]);
        let entries = e.mem.u32(context + 0x10);
        assert_eq!(
            dwords(&e, entries, 6),
            vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0, 0]
        );
        assert_eq!(writes.borrow().len(), 12);
        assert!(calls_to(&e, SLOT_WRITE_OBJECT).is_empty());
    }

    #[test]
    fn save_writes_the_header_the_objects_and_the_deferred_writes() {
        let mut e = engine();
        object_doubles(&mut e);
        // The stream's position is constant, so the sub stream always reports 0.
        let inner = stream(&mut e, 0x40);
        answer(&mut e, SLOT_TELL, 0x40);
        let slots = [
            (0xc, 0x00db_d730),
            (0x10, 0x00db_d750),
            (0x14, 0x00db_d760),
            (0x18, 0x00db_d770),
            (0x1c, 0x00db_d790),
            (0x20, 0x00db_d7d0),
        ];
        for (offset, target) in slots {
            e.mem.set_u32(SUB_STREAM_VTABLE + offset, target);
        }
        let writer = object_with_slots(&mut e, &[(0xc, SLOT_WRITE_OBJECT)]);
        let writer_vtable = e.mem.u32(writer);
        e.register_double(PLATFORM_WRITER_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], writer_vtable);
            Ret::default()
        });
        nothing(
            &mut e,
            &[
                SLOT_WRITE_OBJECT,
                VERSION_TEXT,
                PLATFORM_WRITER_DESTRUCT,
                ARCHIVE_WRITE_ARRAY_32,
            ],
        );
        answer(&mut e, STRING_MAP_GET_WITH_DEFAULT, 0);
        answer(&mut e, SECTION_INDEX, 0);
        let layout = words(&mut e, &[0]);
        answer(&mut e, PLATFORM_WRITER_GET_LAYOUT, layout);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"__data__");
        let names = words(&mut e, &[name]);
        let pending = e.mem.alloc(0x20);
        e.mem.set_u32(pending, 0x0111_0000);
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, pending);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(this + 0x64, names);
        e.mem.set_u32(this + 0x68, 1);
        let options = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00db_d620, &args![this, inner, options]).u32(), 0);
        // The header and the section record go through the object writer on the sub stream.
        let objects = calls_to(&e, SLOT_WRITE_OBJECT);
        assert_eq!(objects.len(), 2);
        assert_eq!((objects[0][3], objects[1][3]), (0x0126_c6c8, 0x0126_c698));
        let sub = objects[0][1];
        assert_eq!(objects[1][1], sub);
        assert_eq!(objects[0][0], objects[1][0]);
        // Seeks reach the inner stream relative to its start (0x40): past the one section
        // record, then back to the section data, the noted position and the end.
        assert_eq!(
            calls_to(&e, SLOT_SEEK),
            vec![
                vec![inner, 0x30, 1],
                vec![inner, 0x40, 0],
                vec![inner, 0x40, 0],
                vec![inner, 0x40, 0],
                vec![inner, 0x58, 0],
                vec![inner, 0x40, 0],
            ]
        );
        assert_eq!(calls_to(&e, SLOT_FLUSH), vec![vec![inner]]);
    }
}
