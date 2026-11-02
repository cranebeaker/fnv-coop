//! `Havok SDK/hkpackfilewriter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkPackfileWriter` fields as the translated functions use them (offsets from `this`): the
//! vtable at `+0`; the pending writes array at `+8` (24-byte elements: data `+8`, size `+0xc`,
//! capacity word `+0x10`); an open-addressing map at `+0x14` from object pointer to write index
//! (data `+0x14`, count `+0x18`, mask `+0x1c`; a missing key reads as `-1`); maps at `+0x20`
//! (imports) and `+0x2c` (exports); string maps at `+0x38` (class name to class) and `+0x48`
//! (enum name to enum); a map at `+0x58` (object to the object it was merged with); the array of
//! section names at `+0x64` (data, size `+0x68`, capacity word `+0x6c`) with its string map at
//! `+0x70` (name to index); the array of unregistered objects at `+0x80` (8-byte elements:
//! object, class); the counters at `+0x94` and `+0x98`, the indices at `+0x8c` and `+0x90`; a
//! map at `+0x9c` (object to section index), a string map at `+0xa8` (class name to section
//! index) and a multimap at `+0xb8`, with a map at `+0xc4` torn down by the destructor.
//! A capacity word keeps the capacity in its low 30 bits and sets bit 31 when the data is not
//! owned. Pointers to class objects are `hkClass`-like records (the functions of the class are
//! called by address: name `00d46d40`, parent `00d46d50`, vtable test `00d470f0`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkMemoryRouter::easyFree` (`00c856c0`, cdecl `(allocator, pointer)`).
const EASY_FREE: u32 = 0x00c8_56c0;
/// `hkString::strDup` is not used here; `hkArrayUtil::_reserveMore` (`00c90940`, cdecl
/// `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// The map functions (`this` = the map): insert (`00cb1870`, `(allocator, key, value)`), find
/// key (`00cb18f0`, `(key)`, returns the index, past the mask when absent), get with default
/// (`00cb1930`, `(key, default)`), reserve (`00cb1fe0`, `(allocator, capacity)`) and clear and
/// deallocate (`00cb1a90`, `(allocator)`); the empty function the destructor calls on each map
/// (`00a29680`).
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
const MAP_RESERVE: u32 = 0x00cb_1fe0;
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
const NO_EFFECT: u32 = 0x00a2_9680;
/// The string map functions (`this` = the string map): constructor (`00d87790`, one dword it
/// ignores), get with default (`00d87800`, `(key, default)`), insert (`00d874d0`, `(key,
/// value)`), lookup (`00d87650`, `(key, &out)`, returns 0 when found) and destructor
/// (`00d877d0`).
const STRING_MAP_CONSTRUCT: u32 = 0x00d8_7790;
const STRING_MAP_GET_WITH_DEFAULT: u32 = 0x00d8_7800;
const STRING_MAP_INSERT: u32 = 0x00d8_74d0;
const STRING_MAP_LOOKUP: u32 = 0x00d8_7650;
const STRING_MAP_DESTRUCT: u32 = 0x00d8_77d0;
/// `hkString::strDup` (`00cb1410`, cdecl `(text)`).
const STRING_DUPLICATE: u32 = 0x00cb_1410;
/// `hkString::strCmp` (`00cb0fa0`, cdecl `(a, b)`) and `hkString::snprintf` (`00cb0f60`, cdecl
/// `(buffer, size, format, ...)`).
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const SNPRINTF: u32 = 0x00cb_0f60;
/// Class functions (`this` = the class): name (`00d46d40`), parent (`00d46d50`), `hasVtable`
/// (`00d470f0`, `(&out byte)`, returns the out pointer) and signature (`00d47690`, `(flags)`).
const CLASS_GET_NAME: u32 = 0x00d4_6d40;
const CLASS_GET_PARENT: u32 = 0x00d4_6d50;
const CLASS_HAS_VTABLE: u32 = 0x00d4_70f0;
const CLASS_GET_SIGNATURE: u32 = 0x00d4_7690;
/// The text builder used for the error messages (identity unconfirmed): constructor (`00cb1600`,
/// `this` = the builder, `(buffer, size)`), append (`00cc01c0`, `(text)`, returns the builder)
/// and destructor (`00cc00e0`).
const TEXT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const TEXT_BUILDER_APPEND: u32 = 0x00cc_01c0;
const TEXT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
/// The multimap at `+0xb8`: constructor (`00dc9e70`) and insert (`00dc9ec0`, `(key, &value)`).
const MULTI_MAP_CONSTRUCT: u32 = 0x00dc_9e70;
const MULTI_MAP_INSERT: u32 = 0x00dc_9ec0;
/// The objects `chasePointers` builds on its stack: structure layout constructor (`00dc1670`),
/// platform object writer constructor (`00e02bb0`, `this` = the writer, `(layout)`), offset-only
/// stream writer constructor (`00e02ef0`), the writer's `writeObject` (`00e02a80`, `(stream,
/// object, class, relocation info)`) and its destructor (`00e02c70`).
const STRUCTURE_LAYOUT_CONSTRUCT: u32 = 0x00dc_1670;
const PLATFORM_WRITER_CONSTRUCT: u32 = 0x00e0_2bb0;
const OFFSET_STREAM_WRITER_CONSTRUCT: u32 = 0x00e0_2ef0;
const PLATFORM_WRITE_OBJECT: u32 = 0x00e0_2a80;
const PLATFORM_WRITER_DESTRUCT: u32 = 0x00e0_2c70;
/// The relocation info built by `addObject` (`00dbd870` constructor, `00e028e0` destructor).
const RELOCATION_INFO_CONSTRUCT: u32 = 0x00db_d870;
const RELOCATION_INFO_DESTRUCT: u32 = 0x00e0_28e0;
/// Exe globals: the object whose slot `0x14` gives the class registry (and whose slot `0x10`
/// gives the referenced object of a packfile data), the error reporter (slot `0xc`) and the two
/// pointers to the section tags "__data__" and "__types__".
const SOURCE_OBJECT_GLOBAL: u32 = 0x0126_8b08;
const ERROR_REPORTER_GLOBAL: u32 = 0x0126_8360;
const DATA_TAG_POINTER: u32 = 0x011b_7ae8;
const TYPES_TAG_POINTER: u32 = 0x011b_7aec;
/// Three class objects (exe data) the constructor registers in the map at `+0x14`; the last is
/// the one `addPendingWrite` counts and `addObject` recurses with.
const CLASS_OBJECT_A: u32 = 0x0126_cab8;
const CLASS_OBJECT_B: u32 = 0x0126_ca58;
const CLASS_OBJECT_C: u32 = 0x0126_cae8;
/// The vtable the constructor installs and the one the destructor ends with.
const PACKFILE_WRITER_VTABLE: u32 = 0x010d_6708;
const BASE_VTABLE: u32 = 0x0102_e388;
/// Exe strings: the version format and suffix, "hkClass" and "hkClassEnum", the source file name
/// and the pieces of the messages.
const VERSION_FORMAT: u32 = 0x010d_6628;
const VERSION_SUFFIX: u32 = 0x010d_663c;
const NAME_HK_CLASS: u32 = 0x010d_66fc;
const NAME_HK_CLASS_ENUM: u32 = 0x010d_6640;
const SOURCE_FILE_NAME: u32 = 0x010d_664c;
const TEXT_CONFLICTING: u32 = 0x010d_666c;
const TEXT_PERHAPS: u32 = 0x010d_6698;
const TEXT_UNREGISTERED: u32 = 0x010d_6734;
const TEXT_NOT_SERIALIZED: u32 = 0x010d_6760;
const TEXT_POINTERS_NULL: u32 = 0x010d_67a8;
const TEXT_NO_WARNINGS: u32 = 0x010d_67e0;
/// The "not owned" bit of a capacity word, and the capacity bits.
const NOT_OWNED: u32 = 0x8000_0000;
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// Bytes of the text buffer the messages are built in.
const MESSAGE_BUFFER_SIZE: u32 = 0x200;

/// The memory router's allocator (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Inserts `key` -> `value` into the map at `map` (the router is asked for the allocator
/// first).
fn map_insert(e: &mut Engine, map: u32, key: u32, value: u32) {
    let allocator = router_allocator(e);
    e.call(MAP_INSERT, &args![map, allocator, key, value]);
}

/// Initialises a map at `map` (data 0, count 0, mask `-1`) and reserves it with capacity 0.
fn map_construct(e: &mut Engine, map: u32) {
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = router_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
}

/// Clears and deallocates the map at `map`, then calls the empty function on it.
fn map_destruct(e: &mut Engine, map: u32) {
    let allocator = router_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(NO_EFFECT, &args![map]);
}

/// Whether the map's `findKey` index is past the mask (the key is absent).
fn key_absent(e: &mut Engine, map: u32, key: u32) -> bool {
    let index = e.call(MAP_FIND_KEY, &args![map, key]).i32();
    index > e.mem.i32(map + 8)
}

/// `hkClass::hasVtable`: asks the class (`00d470f0`) and reads the byte at the pointer it
/// returns.
fn class_has_vtable(e: &mut Engine, class: u32) -> bool {
    e.with_stack(4, |e, out| {
        let result = e.call(CLASS_HAS_VTABLE, &args![class, out]).u32();
        e.mem.u8(result) != 0
    })
}

/// Frees an owned array's data through allocator slot `0x10` (`data, capacity * element`).
fn free_owned(e: &mut Engine, allocator: u32, data: u32, capacity_word: u32, element: u32) {
    if capacity_word & NOT_OWNED == 0 {
        let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(element);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Appends a word array element (`element` bytes, filled by `fill`) after growing the array
/// (`_reserveMore`) when it is full; the router is asked before the check. The array is
/// `(data, size, capacity word)` at `array`.
fn array_push(e: &mut Engine, array: u32, element: u32, fill: impl FnOnce(&mut Engine, u32)) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(element));
    fill(e, slot);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00dbd9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getExactClass` (named in the engine map); takes its inputs in registers: the class in EDI
/// (first argument here), the class registry in ESI (second) and the instance on the stack
/// (third; the caller cleans it). A class without a vtable is its own exact class (returned
/// unchanged); with a vtable and no registry the result is 0; otherwise slot `0x10` of the
/// registry is asked for the class of the instance.
pub fn get_exact_class(e: &mut Engine, class: u32, registry: u32, instance: u32) -> u32 {
    if !class_has_vtable(e, class) {
        return class;
    }
    if registry == 0 {
        return 0;
    }
    e.vcall(registry, 0x10, &args![instance]).u32()
}

// Translated from 00dbda20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::getCurrentVersion` (named in the engine map, cdecl): formats the version
/// text "Havok-%i.%i.%i-%s" with 7, 1, 0 and the exe's version suffix string into `buffer`
/// (`size` bytes, `snprintf`). Returns the `snprintf` result as the exe leaves it in EAX.
pub fn hkp_packfile_writer_get_current_version(e: &mut Engine, buffer: u32, size: u32) -> u32 {
    e.call(
        SNPRINTF,
        &args![
            buffer,
            size,
            VERSION_FORMAT,
            7u32,
            1u32,
            0u32,
            VERSION_SUFFIX
        ],
    )
    .u32()
}

// Translated from 00dbda50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::setContents` (named in the engine map): asks slot `0x14` of the exe
/// global object `01268b08` for the class registry (no arguments) and calls this writer's vtable
/// slot `0xc` (`setContentsWithRegistry`) with `(object, class, registry, last)`. The decompiler
/// shows three arguments; the disassembly keeps the third argument of this function on the
/// stack across the registry call and passes it as the fourth.
pub fn hkp_packfile_writer_set_contents(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    last: u32,
) {
    let vtable = e.mem.u32(this.addr());
    let source = e.mem.u32(SOURCE_OBJECT_GLOBAL);
    let registry = e.vcall(source, 0x14, &args![]).u32();
    let target = e.mem.u32(vtable + 0xc);
    e.call(target, &args![this, object, class, registry, last]);
}

// Translated from 00dbda80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::addImport` (named in the engine map): when the map at `+0x14` has a
/// non-negative write index for `object`, that pending write's word at `+0x10` (24-byte
/// elements at `+8`) is set to `-2`; then `object` -> `-2` is inserted into the map at `+0x14`
/// and `object` -> `name` into the map at `+0x20`.
pub fn hkp_packfile_writer_add_import(e: &mut Engine, this: Ptr, object: u32, name: u32) {
    let base = this.addr();
    let index = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0x14, object, 0xffff_ffffu32],
        )
        .i32();
    if index >= 0 {
        let writes = e.mem.u32(base + 8);
        let slot = writes
            .wrapping_add((index as u32).wrapping_mul(0x18))
            .wrapping_add(0x10);
        e.mem.set_u32(slot, 0xffff_fffe);
    }
    map_insert(e, base + 0x14, object, 0xffff_fffe);
    map_insert(e, base + 0x20, object, name);
}

// Translated from 00dbdae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::addExport` (named in the engine map): inserts `object` -> `name` into the
/// map at `+0x2c`.
pub fn hkp_packfile_writer_add_export(e: &mut Engine, this: Ptr, object: u32, name: u32) {
    map_insert(e, this.addr() + 0x2c, object, name);
}

// Translated from 00dbdb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `chasePointers` (named in the engine map, cdecl): builds on the stack a structure layout, an
/// offset-only stream writer and a platform object writer (constructed with the layout), has the
/// writer write `object` of class `class` to the stream with the relocation info `info`, sets
/// the stream writer's vtable to the base vtable (`0102e388`) and destructs the platform
/// object writer. The three objects share a `0x24`-byte frame: layout at `+0`, stream writer
/// at `+4`, platform writer at `+0x14`.
pub fn chase_pointers(e: &mut Engine, object: u32, class: u32, info: u32) {
    e.with_stack(0x24, |e, frame| {
        let layout = frame.addr();
        let stream = layout + 4;
        let writer = layout + 0x14;
        e.call(STRUCTURE_LAYOUT_CONSTRUCT, &args![layout]);
        e.call(PLATFORM_WRITER_CONSTRUCT, &args![writer, layout]);
        e.call(OFFSET_STREAM_WRITER_CONSTRUCT, &args![stream]);
        e.call(
            PLATFORM_WRITE_OBJECT,
            &args![writer, stream, object, class, info],
        );
        e.mem.set_u32(stream, BASE_VTABLE);
        e.call(PLATFORM_WRITER_DESTRUCT, &args![writer]);
    });
}

/// Builds an error message in a stack buffer and hands it to the error reporter (slot `0xc`:
/// `(level, id, text, file, line)`). The text builder (`00cb1600`, `(buffer, size)`) is
/// constructed first, then `parts` supplies the pieces (the class names are read after the
/// construction), each appended through `00cc01c0`; the builder is destructed (`00cc00e0`)
/// after the report. With `int3_on_result` a non-zero result of the reporter is an `INT3` in
/// the exe (which stops before the destructor) and panics here.
fn report_message(
    e: &mut Engine,
    level: u32,
    id: u32,
    line: u32,
    int3_on_result: Option<&str>,
    parts: impl FnOnce(&mut Engine) -> Vec<u32>,
) {
    e.with_stack(MESSAGE_BUFFER_SIZE, |e, buffer| {
        e.with_stack(0x20, |e, builder| {
            let text = buffer.addr();
            let builder = builder.addr();
            e.call(
                TEXT_BUILDER_CONSTRUCT,
                &args![builder, text, MESSAGE_BUFFER_SIZE],
            );
            let mut current = builder;
            for part in parts(e) {
                current = e.call(TEXT_BUILDER_APPEND, &args![current, part]).u32();
            }
            let reporter = e.mem.u32(ERROR_REPORTER_GLOBAL);
            let result = e
                .vcall(
                    reporter,
                    0xc,
                    &args![level, id, text, SOURCE_FILE_NAME, line],
                )
                .u32();
            if let Some(site) = int3_on_result {
                if result != 0 {
                    panic!("INT3 at {site}");
                }
            }
            e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

// Translated from 00dbdb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::notDuplicateMetaData` (named in the engine map): returns 1 when the
/// object's meta data is new and 0 when it duplicates an earlier one. `class` is the object's
/// class. If its name is "hkClass" the object is itself a class: its name is looked up in the
/// string map at `+0x38`; when there is such a class its signatures (flags 0) are compared and a
/// difference is reported ("Conflicting metadata found for class ...", level 3, id
/// `0x2518721c`, line `0xb2`; a non-zero result of the reporter is an `INT3` in the exe and
/// panics here), then `object` gets the write index of the earlier class in the map at `+0x14`
/// and is recorded as merged with it in the map at `+0x58` (result 0); without such a class the
/// name is inserted (result 1). If the class name is "hkClassEnum" the same is done with the
/// enum name (the word at `object + 0`) and the string map at `+0x48`. Any other class gives 1.
pub fn hkp_packfile_writer_not_duplicate_meta_data(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
) -> u32 {
    let base = this.addr();
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    if e.call(STRING_COMPARE, &args![name, NAME_HK_CLASS]).u32() == 0 {
        let name = e.call(CLASS_GET_NAME, &args![object]).u32();
        let existing = e
            .call(STRING_MAP_GET_WITH_DEFAULT, &args![base + 0x38, name, 0u32])
            .u32();
        if existing == 0 {
            let name = e.call(CLASS_GET_NAME, &args![object]).u32();
            e.call(STRING_MAP_INSERT, &args![base + 0x38, name, object]);
            return 1;
        }
        let earlier = e.call(CLASS_GET_SIGNATURE, &args![existing, 0u32]).u32();
        let current = e.call(CLASS_GET_SIGNATURE, &args![object, 0u32]).u32();
        if current != earlier {
            report_message(
                e,
                3,
                0x2518_721c,
                0xb2,
                Some("00dbdc40 (hkPackfileWriter::notDuplicateMetaData)"),
                |e| {
                    let name = e.call(CLASS_GET_NAME, &args![object]).u32();
                    vec![TEXT_CONFLICTING, name, TEXT_PERHAPS]
                },
            );
        }
        merge_into_earlier(e, base, object, existing);
        return 0;
    }
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    if e.call(STRING_COMPARE, &args![name, NAME_HK_CLASS_ENUM])
        .u32()
        != 0
    {
        return 1;
    }
    let enum_name = e.mem.u32(object);
    let existing = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![base + 0x48, enum_name, 0u32],
        )
        .u32();
    if existing == 0 {
        let enum_name = e.mem.u32(object);
        e.call(STRING_MAP_INSERT, &args![base + 0x48, enum_name, object]);
        return 1;
    }
    merge_into_earlier(e, base, object, existing);
    0
}

/// `object` takes the write index `existing` has in the map at `+0x14` and is recorded as
/// merged with `existing` in the map at `+0x58`.
fn merge_into_earlier(e: &mut Engine, base: u32, object: u32, existing: u32) {
    let index = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0x14, existing, 0xffff_ffffu32],
        )
        .u32();
    map_insert(e, base + 0x14, object, index);
    map_insert(e, base + 0x58, object, existing);
}

// Translated from 00dbdd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::sectionTagToIndex` (named in the engine map): the index of the section
/// named `tag` in the string map at `+0x70` when it is there (lookup `00d87650` returns 0).
/// Otherwise the name is copied (`strDup`), inserted with the current section count (`+0x68`)
/// as its index, appended to the array at `+0x64` (4-byte elements, `_reserveMore` when
/// full) and the old count is returned.
pub fn hkp_packfile_writer_section_tag_to_index(e: &mut Engine, this: Ptr, tag: u32) -> u32 {
    let base = this.addr();
    let found = e.with_stack(4, |e, out| {
        let status = e
            .call(STRING_MAP_LOOKUP, &args![base + 0x70, tag, out])
            .u32();
        (status, e.mem.u32(out.addr()))
    });
    if found.0 == 0 {
        return found.1;
    }
    let copy = e.call(STRING_DUPLICATE, &args![tag]).u32();
    let index = e.mem.u32(base + 0x68);
    e.call(STRING_MAP_INSERT, &args![base + 0x70, copy, index]);
    array_push(e, base + 0x64, 4, |e, slot| e.mem.set_u32(slot, copy));
    index
}

// Translated from 00dbddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::findSectionFor` (named in the engine map; the decompiler shows no
/// result, the disassembly returns the index in EAX): the section index of `object` in the map
/// at `+0x9c` when there is one (default `-1`); else that of `class`'s name, then of each
/// parent class's name, in the string map at `+0xa8`; else `sectionTagToIndex(tag)`.
pub fn hkp_packfile_writer_find_section_for(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    tag: u32,
) -> u32 {
    let base = this.addr();
    let section = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0x9c, object, 0xffff_ffffu32],
        )
        .u32();
    if section != 0xffff_ffff {
        return section;
    }
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    let section = e
        .call(
            STRING_MAP_GET_WITH_DEFAULT,
            &args![base + 0xa8, name, 0xffff_ffffu32],
        )
        .u32();
    if section != 0xffff_ffff {
        return section;
    }
    let mut current = class;
    loop {
        current = e.call(CLASS_GET_PARENT, &args![current]).u32();
        if current == 0 {
            return hkp_packfile_writer_section_tag_to_index(e, this, tag);
        }
        let name = e.call(CLASS_GET_NAME, &args![current]).u32();
        let section = e
            .call(
                STRING_MAP_GET_WITH_DEFAULT,
                &args![base + 0xa8, name, 0xffff_ffffu32],
            )
            .u32();
        if section != 0xffff_ffff {
            return section;
        }
    }
}

// Translated from 00dbde40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::addPendingWrite` (named in the engine map): finds the section for
/// `(owner, class, tag)` (`findSectionFor`), appends a 24-byte record to the pending writes (the
/// array at `+8`, `_reserveMore` with element size `0x18` when full; the router is asked before
/// the check) with the words `(object, type, owner, class, section, -1)` (skipped when the
/// element address is null), and bumps the counter at `+0x98` when `type` is the class object
/// `0126cae8` or the one at `+0x94` otherwise.
pub fn hkp_packfile_writer_add_pending_write(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    type_object: u32,
    owner: u32,
    class: u32,
    tag: u32,
) {
    let base = this.addr();
    let section = hkp_packfile_writer_find_section_for(e, this, owner, class, tag);
    array_push(e, base + 8, 0x18, |e, slot| {
        if slot != 0 {
            e.mem.set_u32(slot, object);
            e.mem.set_u32(slot + 4, type_object);
            e.mem.set_u32(slot + 8, owner);
            e.mem.set_u32(slot + 0xc, class);
            e.mem.set_u32(slot + 0x10, section);
            e.mem.set_u32(slot + 0x14, 0xffff_ffff);
        }
    });
    let is_class_object = u32::from(type_object == CLASS_OBJECT_C);
    let total = e.mem.u32(base + 0x98).wrapping_add(is_class_object);
    e.mem.set_u32(base + 0x98, total);
    let others = e.mem.u32(base + 0x94).wrapping_add(1 - is_class_object);
    e.mem.set_u32(base + 0x94, others);
}

// Translated from 00dbdef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::addSection` (named in the engine map): a jump to
/// [`hkp_packfile_writer_section_tag_to_index`].
pub fn hkp_packfile_writer_add_section(e: &mut Engine, this: Ptr, tag: u32) -> u32 {
    hkp_packfile_writer_section_tag_to_index(e, this, tag)
}

// Translated from 00dbdf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::setSectionForPointer` (named in the engine map): inserts `pointer` ->
/// `sectionTagToIndex(tag)` into the map at `+0x9c`.
pub fn hkp_packfile_writer_set_section_for_pointer(
    e: &mut Engine,
    this: Ptr,
    pointer: u32,
    tag: u32,
) {
    let index = hkp_packfile_writer_section_tag_to_index(e, this, tag);
    map_insert(e, this.addr() + 0x9c, pointer, index);
}

// Translated from 00dbdf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::setSectionForClass` (named in the engine map): inserts the class's name ->
/// `sectionTagToIndex(tag)` into the string map at `+0xa8`.
pub fn hkp_packfile_writer_set_section_for_class(e: &mut Engine, this: Ptr, class: u32, tag: u32) {
    let index = hkp_packfile_writer_section_tag_to_index(e, this, tag);
    let name = e.call(CLASS_GET_NAME, &args![class]).u32();
    e.call(STRING_MAP_INSERT, &args![this.addr() + 0xa8, name, index]);
}

// Translated from 00dbdf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed destructor (`__fastcall`; the deleting destructor `00dbe760` calls it): sets the
/// vtable (`010d6708`); frees every section name of the array at `+0x64` (`easyFree`); tears
/// down, in this order, the map at `+0xc4`, the multimap array at `+0xb8` (8-byte elements), the
/// string map at `+0xa8`, the map at `+0x9c`, the array at `+0x80` (8-byte elements), the
/// string map at `+0x70`, the section name array at `+0x64` (4-byte elements), the map at
/// `+0x58`, the string maps at `+0x48` and `+0x38`, the maps at `+0x2c`, `+0x20` and `+0x14`
/// and the pending writes at `+8` (24-byte elements). Maps are cleared and deallocated (and the
/// empty function called); arrays have their size zeroed and, when owned, their data freed
/// through allocator slot `0x10`, and are reset to data 0, capacity word `0x80000000`. The
/// vtable ends as `0102e388`.
pub fn fn_00dbdf60(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PACKFILE_WRITER_VTABLE);
    let mut index = 0u32;
    if e.mem.i32(base + 0x68) > 0 {
        loop {
            let names = e.mem.u32(base + 0x64);
            let name = e.mem.u32(names.wrapping_add(index.wrapping_mul(4)));
            let allocator = router_allocator(e);
            e.call(EASY_FREE, &args![allocator, name]);
            index += 1;
            if index as i32 >= e.mem.i32(base + 0x68) {
                break;
            }
        }
    }
    map_destruct(e, base + 0xc4);
    destruct_array(e, base, (0xb8, 0xbc, 0xc0), 8);
    e.call(STRING_MAP_DESTRUCT, &args![base + 0xa8]);
    map_destruct(e, base + 0x9c);
    destruct_array(e, base, (0x80, 0x84, 0x88), 8);
    e.call(STRING_MAP_DESTRUCT, &args![base + 0x70]);
    destruct_array(e, base, (0x64, 0x68, 0x6c), 4);
    map_destruct(e, base + 0x58);
    e.call(STRING_MAP_DESTRUCT, &args![base + 0x48]);
    e.call(STRING_MAP_DESTRUCT, &args![base + 0x38]);
    map_destruct(e, base + 0x2c);
    map_destruct(e, base + 0x20);
    map_destruct(e, base + 0x14);
    destruct_array(e, base, (8, 0xc, 0x10), 0x18);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// Empties one array of the writer: `(data, size, capacity word)` field offsets; the router is
/// asked first, the size zeroed, owned data freed and the array reset.
fn destruct_array(e: &mut Engine, base: u32, fields: (u32, u32, u32), element: u32) {
    let (data, size, capacity) = fields;
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(base + capacity);
    e.mem.set_u32(base + size, 0);
    let array = e.mem.u32(base + data);
    free_owned(e, allocator, array, capacity_word, element);
    e.mem.set_u32(base + data, 0);
    e.mem.set_u32(base + capacity, NOT_OWNED);
}

// Translated from 00dbe150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::addObject` (named in the engine map): records `object` and everything it
/// points to. Nothing happens when `object` already has an entry in the map at `+0x14`.
/// Otherwise the exact class is [`get_exact_class`]`(class, registry, object)`. If there is none
/// the class is unregistered: an error is reported ("Found an un-registered class derived from
/// ..." and three more pieces, level 1, id `0xabbaabba`, line `0x121`), `object` is entered with
/// index `-1` and `(object, class)` is appended to the array at `+0x80` (8-byte elements). If
/// there is one, a non-zero `rewriter` may replace the object and the class through its slot
/// `0xc` (two cells `(object, class)`); a null replacement enters `-1`. Otherwise
/// [`hkp_packfile_writer_not_duplicate_meta_data`] decides (0 stops); the object also stops
/// when it was replaced by an object that already has an entry. The object (and the
/// replacement) get the next write index (`+0xc`), a replacement is also recorded in the map at
/// `+0x58`, and the write is queued ([`hkp_packfile_writer_add_pending_write`]). Then the
/// object is chased ([`chase_pointers`], into a relocation info): for each relocation
/// `(target, target class, flag)` of the info, with a flag, a target without a section gets
/// the section of the written object (default `sectionTagToIndex(tag)`) in the map at `+0x9c`;
/// the target is added recursively with its class; a target other than the written object that
/// has a write index is entered in the multimap (`written` -> index). Last the class itself is
/// added with the class object `0126cae8` and the "__types__" tag.
pub fn hkp_packfile_writer_add_object(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    registry: u32,
    rewriter: u32,
    tag: u32,
) {
    let base = this.addr();
    let objects = base + 0x14;
    if !key_absent(e, objects, object) {
        return;
    }
    let exact = get_exact_class(e, class, registry, object);
    if exact == 0 {
        report_message(e, 1, 0xabba_abba, 0x121, None, |e| {
            let class_name = e.call(CLASS_GET_NAME, &args![class]).u32();
            vec![
                TEXT_UNREGISTERED,
                class_name,
                TEXT_NOT_SERIALIZED,
                TEXT_POINTERS_NULL,
                TEXT_NO_WARNINGS,
            ]
        });
        map_insert(e, objects, object, 0xffff_ffff);
        array_push(e, base + 0x80, 8, |e, slot| {
            e.mem.set_u32(slot, object);
            e.mem.set_u32(slot + 4, class);
        });
        return;
    }
    e.with_stack(12, |e, cells| {
        let object_cell = cells.addr();
        let class_cell = object_cell + 4;
        let index_cell = object_cell + 8;
        e.mem.set_u32(object_cell, object);
        e.mem.set_u32(class_cell, exact);
        if rewriter != 0 {
            e.vcall(rewriter, 0xc, &args![object_cell, class_cell]);
        }
        let written = e.mem.u32(object_cell);
        if written == 0 {
            map_insert(e, objects, object, 0xffff_ffff);
            return;
        }
        let written_class = e.mem.u32(class_cell);
        if hkp_packfile_writer_not_duplicate_meta_data(e, this, written, written_class) == 0 {
            return;
        }
        if object != written && !key_absent(e, objects, written) {
            return;
        }
        let write_index = e.mem.u32(base + 0xc);
        map_insert(e, objects, object, write_index);
        let written = e.mem.u32(object_cell);
        if written != object {
            let write_index = e.mem.u32(base + 0xc);
            map_insert(e, objects, written, write_index);
            let written = e.mem.u32(object_cell);
            map_insert(e, base + 0x58, written, object);
        }
        let written = e.mem.u32(object_cell);
        let written_class = e.mem.u32(class_cell);
        hkp_packfile_writer_add_pending_write(e, this, written, written_class, object, class, tag);
        e.with_stack(0x40, |e, info| {
            let info = info.addr();
            e.call(RELOCATION_INFO_CONSTRUCT, &args![info]);
            let written = e.mem.u32(object_cell);
            let written_class = e.mem.u32(class_cell);
            chase_pointers(e, written, written_class, info);
            if e.mem.i32(info + 0x10) > 0 {
                let mut position = 0i32;
                let mut offset = 0u32;
                loop {
                    let entries = e.mem.u32(info + 0xc);
                    let entry = entries.wrapping_add(offset);
                    let target_class = e.mem.u32(entry + 8);
                    let target = e.mem.u32(entry + 4);
                    if target_class != 0 && target != 0 {
                        if e.mem.u32(entry + 0xc) != 0 {
                            let sections = base + 0x9c;
                            if key_absent(e, sections, target) {
                                let default =
                                    hkp_packfile_writer_section_tag_to_index(e, this, tag);
                                let written = e.mem.u32(object_cell);
                                let section = e
                                    .call(MAP_GET_WITH_DEFAULT, &args![sections, written, default])
                                    .u32();
                                map_insert(e, sections, target, section);
                            }
                        }
                        hkp_packfile_writer_add_object(
                            e,
                            this,
                            target,
                            target_class,
                            registry,
                            rewriter,
                            tag,
                        );
                        let written = e.mem.u32(object_cell);
                        if written != target {
                            let write_index = e
                                .call(
                                    MAP_GET_WITH_DEFAULT,
                                    &args![objects, target, 0xffff_ffffu32],
                                )
                                .u32();
                            e.mem.set_u32(index_cell, write_index);
                            if write_index != 0xffff_ffff {
                                e.call(MULTI_MAP_INSERT, &args![base + 0xb8, written, index_cell]);
                            }
                        }
                    }
                    position += 1;
                    offset = offset.wrapping_add(0x10);
                    if position >= e.mem.i32(info + 0x10) {
                        break;
                    }
                }
            }
            let types_tag = e.mem.u32(TYPES_TAG_POINTER);
            let written_class = e.mem.u32(class_cell);
            hkp_packfile_writer_add_object(
                e,
                this,
                written_class,
                CLASS_OBJECT_C,
                registry,
                rewriter,
                types_tag,
            );
            e.call(RELOCATION_INFO_DESTRUCT, &args![info]);
        });
    });
}

// Translated from 00dbe4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::hkPackfileWriter` (named in the engine map, `__fastcall`): sets the
/// vtable (`010d6708`), the 16-bit word at `+6` to 1, the pending writes empty (capacity word
/// `0x80000000`), constructs and reserves the maps at `+0x14`, `+0x20`, `+0x2c` and `+0x58`,
/// the string maps at `+0x38` and `+0x48`, the section names (`+0x64`, `+0x70`), the
/// unregistered objects (`+0x80`), the indices (`+0x8c`, `+0x90` = -1) and counters (`+0x94`,
/// `+0x98` = 0), the map at `+0x9c`, the string map at `+0xa8` and the multimap at `+0xb8`;
/// enters the three class objects (`0126cab8`, `0126ca58`, `0126cae8`) in the map at `+0x14` with
/// index `-1`. Returns `this`.
pub fn hkp_packfile_writer_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, PACKFILE_WRITER_VTABLE);
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, NOT_OWNED);
    map_construct(e, base + 0x14);
    map_construct(e, base + 0x20);
    map_construct(e, base + 0x2c);
    e.call(STRING_MAP_CONSTRUCT, &args![base + 0x38, 0u32]);
    e.call(STRING_MAP_CONSTRUCT, &args![base + 0x48, 0u32]);
    map_construct(e, base + 0x58);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base + 0x68, 0);
    e.mem.set_u32(base + 0x6c, NOT_OWNED);
    e.call(STRING_MAP_CONSTRUCT, &args![base + 0x70, 0u32]);
    e.mem.set_u32(base + 0x88, NOT_OWNED);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x8c, 0xffff_ffff);
    e.mem.set_u32(base + 0x90, 0xffff_ffff);
    e.mem.set_u32(base + 0x94, 0);
    e.mem.set_u32(base + 0x98, 0);
    map_construct(e, base + 0x9c);
    e.call(STRING_MAP_CONSTRUCT, &args![base + 0xa8, 0u32]);
    e.call(MULTI_MAP_CONSTRUCT, &args![base + 0xb8]);
    for class_object in [CLASS_OBJECT_A, CLASS_OBJECT_B, CLASS_OBJECT_C] {
        map_insert(e, base + 0x14, class_object, 0xffff_ffff);
    }
    this
}

// Translated from 00dbe680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackfileWriter::setContentsWithRegistry` (named in the engine map): with a null
/// `registry` slot `0x14` of the exe global object `01268b08` supplies one. The class is its
/// own exact class when it has no vtable (`hasVtable`), else slot `0x10` of the registry gives
/// the class of `object`. Then [`hkp_packfile_writer_add_object`] is run with the original
/// `registry`, `rewriter` and the "__data__" tag, and the write indices of the object and of the
/// exact class (default `-1`, map at `+0x14`) are stored at `+0x8c` and `+0x90`.
pub fn hkp_packfile_writer_set_contents_with_registry(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    class: u32,
    registry: u32,
    rewriter: u32,
) {
    let base = this.addr();
    let used = if registry == 0 {
        let source = e.mem.u32(SOURCE_OBJECT_GLOBAL);
        e.vcall(source, 0x14, &args![]).u32()
    } else {
        registry
    };
    let exact = if class_has_vtable(e, class) {
        e.vcall(used, 0x10, &args![object]).u32()
    } else {
        class
    };
    let tag = e.mem.u32(DATA_TAG_POINTER);
    hkp_packfile_writer_add_object(e, this, object, exact, registry, rewriter, tag);
    let object_index = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0x14, object, 0xffff_ffffu32],
        )
        .u32();
    e.mem.set_u32(base + 0x8c, object_index);
    let class_index = e
        .call(
            MAP_GET_WITH_DEFAULT,
            &args![base + 0x14, exact, 0xffff_ffffu32],
        )
        .u32();
    e.mem.set_u32(base + 0x90, class_index);
}

// Translated from 00dbe760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00db_df60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dbe760(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00db_df60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db_d9f0, get_exact_class(u32, u32, u32) -> u32),
        entry!(
            0x00db_da20,
            hkp_packfile_writer_get_current_version(u32, u32) -> u32
        ),
        entry!(
            0x00db_da50,
            hkp_packfile_writer_set_contents(Ptr, u32, u32, u32)
        ),
        entry!(0x00db_da80, hkp_packfile_writer_add_import(Ptr, u32, u32)),
        entry!(0x00db_dae0, hkp_packfile_writer_add_export(Ptr, u32, u32)),
        entry!(0x00db_db10, chase_pointers(u32, u32, u32)),
        entry!(
            0x00db_db70,
            hkp_packfile_writer_not_duplicate_meta_data(Ptr, u32, u32) -> u32
        ),
        entry!(
            0x00db_dd40,
            hkp_packfile_writer_section_tag_to_index(Ptr, u32) -> u32
        ),
        entry!(
            0x00db_ddc0,
            hkp_packfile_writer_find_section_for(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00db_de40,
            hkp_packfile_writer_add_pending_write(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00db_def0,
            hkp_packfile_writer_add_section(Ptr, u32) -> u32
        ),
        entry!(
            0x00db_df00,
            hkp_packfile_writer_set_section_for_pointer(Ptr, u32, u32)
        ),
        entry!(
            0x00db_df30,
            hkp_packfile_writer_set_section_for_class(Ptr, u32, u32)
        ),
        entry!(0x00db_df60, fn_00dbdf60(Ptr)),
        entry!(
            0x00db_e150,
            hkp_packfile_writer_add_object(Ptr, u32, u32, u32, u32, u32)
        ),
        entry!(0x00db_e4d0, hkp_packfile_writer_construct(Ptr) -> Ptr),
        entry!(
            0x00db_e680,
            hkp_packfile_writer_set_contents_with_registry(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00db_e760, deleting_destructor_00dbe760(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0012;
    const FREE_DATA_DOUBLE: u32 = 0x0900_0013;
    const REPORT_DOUBLE: u32 = 0x0900_0020;
    const REGISTRY_DOUBLE: u32 = 0x0900_0021;
    const SOURCE_DOUBLE: u32 = 0x0900_0022;
    const SETTER_DOUBLE: u32 = 0x0900_0023;
    const REWRITER_DOUBLE: u32 = 0x0900_0024;

    /// A map or string map's recorded entries: `(map address, key)` to value.
    type Store = Rc<RefCell<HashMap<(u32, u32), u32>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// An engine that logs its calls.
    fn logged() -> Engine {
        let mut e = engine();
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
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
    fn deleting_destructor_00dbe760_frees_only_when_bit_zero_is_set() {
        check(0x00db_e760, 0x00db_df60);
    }

    /// A router whose allocator has the slot-8, slot-`0x10` and slot-4 doubles; returns the
    /// allocator.
    fn install_allocator(e: &mut Engine) -> u32 {
        e.register_double(FREE_BLOCK_DOUBLE, |_, _| Ret::default());
        e.register_double(FREE_DATA_DOUBLE, |_, _| Ret::default());
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 8, FREE_BLOCK_DOUBLE);
        e.mem.set_u32(table + 0x10, FREE_DATA_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        allocator
    }

    /// Doubles for the map functions that keep their entries in a store. A map's mask
    /// (`+8`) decides what `findKey` returns for an absent key (mask + 1).
    fn install_maps(e: &mut Engine) -> Store {
        let store: Store = Rc::default();
        let inserts = Rc::clone(&store);
        e.register_double(MAP_INSERT, move |_, a| {
            inserts.borrow_mut().insert((a[0], a[2]), a[3]);
            Ret::default()
        });
        let gets = Rc::clone(&store);
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: gets.borrow().get(&(a[0], a[1])).copied().unwrap_or(a[2]),
            ..Ret::default()
        });
        let finds = Rc::clone(&store);
        e.register_double(MAP_FIND_KEY, move |e, a| {
            let present = finds.borrow().contains_key(&(a[0], a[1]));
            Ret {
                eax: if present { 0 } else { e.mem.u32(a[0] + 8) + 1 },
                ..Ret::default()
            }
        });
        let string_inserts = Rc::clone(&store);
        e.register_double(STRING_MAP_INSERT, move |_, a| {
            string_inserts.borrow_mut().insert((a[0], a[1]), a[2]);
            Ret::default()
        });
        let string_gets = Rc::clone(&store);
        e.register_double(STRING_MAP_GET_WITH_DEFAULT, move |_, a| Ret {
            eax: string_gets
                .borrow()
                .get(&(a[0], a[1]))
                .copied()
                .unwrap_or(a[2]),
            ..Ret::default()
        });
        let lookups = Rc::clone(&store);
        e.register_double(STRING_MAP_LOOKUP, move |e, a| {
            let value = lookups.borrow().get(&(a[0], a[1])).copied();
            if let Some(value) = value {
                e.mem.set_u32(a[2], value);
            }
            Ret {
                eax: u32::from(value.is_none()),
                ..Ret::default()
            }
        });
        store
    }

    /// Writes the C string `text` into new memory and returns its address.
    fn string(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text.as_bytes());
        at
    }

    /// A class record: the name pointer at `+0`, the parent at `+4` and the signature at `+8`.
    fn class(e: &mut Engine, name: &str, parent: u32, signature: u32) -> u32 {
        let name = string(e, name);
        let at = e.mem.alloc(0x10);
        e.mem.set_u32(at, name);
        e.mem.set_u32(at + 4, parent);
        e.mem.set_u32(at + 8, signature);
        at
    }

    /// Doubles for the class functions (name, parent, signature, vtable test: a class whose
    /// address is odd has a vtable) and the string compare, and the "hkClass" and
    /// "hkClassEnum" strings of the exe.
    fn install_classes(e: &mut Engine) {
        e.register_double(CLASS_GET_NAME, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register_double(CLASS_GET_PARENT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register_double(CLASS_GET_SIGNATURE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register_double(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], (a[0] & 1) as u8);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(STRING_COMPARE, |e, a| {
            let (left, right) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            Ret {
                eax: match left.cmp(&right) {
                    std::cmp::Ordering::Less => 0xffff_ffff,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => 1,
                },
                ..Ret::default()
            }
        });
        e.mem.set_cstr(NAME_HK_CLASS, b"hkClass");
        e.mem.set_cstr(NAME_HK_CLASS_ENUM, b"hkClassEnum");
    }

    /// The error reporter (slot `0xc` is the report double returning `result`) and the text
    /// builder doubles; the builder append returns its `this`.
    fn install_reporter(e: &mut Engine, result: u32) {
        e.register_double(REPORT_DOUBLE, move |_, _| Ret {
            eax: result,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, REPORT_DOUBLE);
        let reporter = e.mem.alloc(0x10);
        e.mem.set_u32(reporter, table);
        e.mem.set_u32(ERROR_REPORTER_GLOBAL, reporter);
        e.register_double(TEXT_BUILDER_CONSTRUCT, |_, _| Ret::default());
        e.register_double(TEXT_BUILDER_APPEND, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(TEXT_BUILDER_DESTRUCT, |_, _| Ret::default());
    }

    /// A zeroed writer with the maps' masks at 7 (the maps at `+0x14` and `+0x9c`).
    fn writer(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0x1c, 7);
        e.mem.set_u32(this + 0xa4, 7);
        this
    }

    /// The texts the builder was handed, in order.
    fn appended(e: &Engine) -> Vec<u32> {
        calls(e, TEXT_BUILDER_APPEND)
            .into_iter()
            .map(|words| words[1])
            .collect()
    }

    #[test]
    fn get_exact_class_returns_the_class_unless_it_has_a_vtable() {
        let mut e = logged();
        install_classes(&mut e);
        e.register_double(REGISTRY_DOUBLE, |_, a| Ret {
            eax: a[1] + 0x10,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0x10, REGISTRY_DOUBLE);
        let registry = e.mem.alloc(0x10);
        e.mem.set_u32(registry, table);
        // EDI = class, ESI = registry, the instance on the stack.
        assert_eq!(
            e.call(0x00db_d9f0, &args![0x1000u32, registry, 0x77u32])
                .u32(),
            0x1000
        );
        assert_eq!(
            e.call(0x00db_d9f0, &args![0x1001u32, 0u32, 0x77u32]).u32(),
            0
        );
        assert_eq!(
            e.call(0x00db_d9f0, &args![0x1001u32, registry, 0x77u32])
                .u32(),
            0x87
        );
        assert_eq!(calls(&e, REGISTRY_DOUBLE), vec![vec![registry, 0x77]]);
    }

    #[test]
    fn get_current_version_formats_the_version_text() {
        let mut e = logged();
        e.register_double(SNPRINTF, |_, _| Ret {
            eax: 21,
            ..Ret::default()
        });
        assert_eq!(e.call(0x00db_da20, &args![0x4000u32, 64u32]).u32(), 21);
        assert_eq!(
            calls(&e, SNPRINTF),
            vec![vec![0x4000, 64, VERSION_FORMAT, 7, 1, 0, VERSION_SUFFIX]]
        );
    }

    #[test]
    fn set_contents_calls_set_contents_with_registry_with_the_fourth_argument() {
        let mut e = logged();
        e.register_double(SETTER_DOUBLE, |_, _| Ret::default());
        e.register_double(SOURCE_DOUBLE, |_, _| Ret {
            eax: 0x5555,
            ..Ret::default()
        });
        let source_table = e.mem.alloc(0x20);
        e.mem.set_u32(source_table + 0x14, SOURCE_DOUBLE);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, source_table);
        e.mem.set_u32(SOURCE_OBJECT_GLOBAL, source);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, SETTER_DOUBLE);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, table);
        e.call(0x00db_da50, &args![this, 0x11u32, 0x22u32, 0x33u32]);
        assert_eq!(calls(&e, SOURCE_DOUBLE), vec![vec![source]]);
        assert_eq!(
            calls(&e, SETTER_DOUBLE),
            vec![vec![this, 0x11, 0x22, 0x5555, 0x33]]
        );
    }

    #[test]
    fn add_import_marks_the_pending_write_and_enters_the_object_twice() {
        let mut e = logged();
        install_allocator(&mut e);
        let store = install_maps(&mut e);
        let this = writer(&mut e);
        let writes = e.mem.alloc(0x18 * 3);
        e.mem.set_u32(this + 8, writes);
        store.borrow_mut().insert((this + 0x14, 0x500), 2);
        e.call(0x00db_da80, &args![this, 0x500u32, 0x600u32]);
        assert_eq!(e.mem.u32(writes + 0x30 + 0x10), 0xffff_fffe);
        assert_eq!(e.mem.u32(writes + 0x10), 0);
        assert_eq!(store.borrow()[&(this + 0x14, 0x500)], 0xffff_fffe);
        assert_eq!(store.borrow()[&(this + 0x20, 0x500)], 0x600);
        // A key without an index only enters the maps.
        e.call(0x00db_da80, &args![this, 0x501u32, 0x601u32]);
        assert_eq!(store.borrow()[&(this + 0x14, 0x501)], 0xffff_fffe);
        assert_eq!(e.mem.u32(writes + 0x18 * 2 + 0x10), 0xffff_fffe);
        assert_eq!(e.mem.u32(writes + 0x18 + 0x10), 0);
    }

    #[test]
    fn add_export_enters_the_object_in_the_export_map() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        let store = install_maps(&mut e);
        let this = writer(&mut e);
        e.call(0x00db_dae0, &args![this, 0x500u32, 0x600u32]);
        assert_eq!(store.borrow()[&(this + 0x2c, 0x500)], 0x600);
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![vec![this + 0x2c, allocator, 0x500, 0x600]]
        );
    }

    #[test]
    fn chase_pointers_builds_the_writer_objects_and_writes_the_object() {
        let mut e = logged();
        let seen = Rc::new(RefCell::new(0u32));
        for address in [
            STRUCTURE_LAYOUT_CONSTRUCT,
            PLATFORM_WRITER_CONSTRUCT,
            OFFSET_STREAM_WRITER_CONSTRUCT,
            PLATFORM_WRITE_OBJECT,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let stream_word = Rc::clone(&seen);
        e.register_double(PLATFORM_WRITER_DESTRUCT, move |e, a| {
            *stream_word.borrow_mut() = e.mem.u32(a[0] - 0x10);
            Ret::default()
        });
        e.call(0x00db_db10, &args![0x10u32, 0x20u32, 0x30u32]);
        let layout = calls(&e, STRUCTURE_LAYOUT_CONSTRUCT)[0][0];
        let (stream, writer) = (layout + 4, layout + 0x14);
        assert_eq!(
            calls(&e, PLATFORM_WRITER_CONSTRUCT),
            vec![vec![writer, layout]]
        );
        assert_eq!(
            calls(&e, OFFSET_STREAM_WRITER_CONSTRUCT),
            vec![vec![stream]]
        );
        assert_eq!(
            calls(&e, PLATFORM_WRITE_OBJECT),
            vec![vec![writer, stream, 0x10, 0x20, 0x30]]
        );
        assert_eq!(calls(&e, PLATFORM_WRITER_DESTRUCT), vec![vec![writer]]);
        assert_eq!(*seen.borrow(), BASE_VTABLE);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                STRUCTURE_LAYOUT_CONSTRUCT,
                PLATFORM_WRITER_CONSTRUCT,
                OFFSET_STREAM_WRITER_CONSTRUCT,
                PLATFORM_WRITE_OBJECT,
                PLATFORM_WRITER_DESTRUCT
            ]
        );
    }

    /// The classes of the duplicate-metadata tests: a plain class "Foo", the class "hkClass"
    /// and the class "hkClassEnum".
    fn metadata_classes(e: &mut Engine) -> (u32, u32, u32) {
        let plain = class(e, "Foo", 0, 0);
        let hk_class = class(e, "hkClass", 0, 0);
        let hk_enum = class(e, "hkClassEnum", 0, 0);
        (plain, hk_class, hk_enum)
    }

    #[test]
    fn not_duplicate_meta_data_accepts_objects_of_other_classes() {
        let mut e = logged();
        install_classes(&mut e);
        let store = install_maps(&mut e);
        install_allocator(&mut e);
        let this = writer(&mut e);
        let (plain, _, _) = metadata_classes(&mut e);
        let object = class(&mut e, "Bar", 0, 0);
        assert_eq!(e.call(0x00db_db70, &args![this, object, plain]).u32(), 1);
        assert!(store.borrow().is_empty());
    }

    #[test]
    fn not_duplicate_meta_data_enters_a_new_class_by_name() {
        let mut e = logged();
        install_classes(&mut e);
        let store = install_maps(&mut e);
        install_allocator(&mut e);
        let this = writer(&mut e);
        let (_, hk_class, _) = metadata_classes(&mut e);
        let object = class(&mut e, "Bar", 0, 5);
        assert_eq!(e.call(0x00db_db70, &args![this, object, hk_class]).u32(), 1);
        let name = e.mem.u32(object);
        assert_eq!(store.borrow()[&(this + 0x38, name)], object);
        assert_eq!(store.borrow().len(), 1);
    }

    /// A packfile writer that already knows a class called "Bar" (signature `signature`) with
    /// write index 9, and a new object with the same name (signature 5).
    fn known_class(e: &mut Engine, signature: u32) -> (u32, u32, u32, Store) {
        install_classes(e);
        let store = install_maps(e);
        install_allocator(e);
        let this = writer(e);
        let existing = class(e, "Bar", 0, signature);
        let object = class(e, "Bar", 0, 5);
        let name = e.mem.u32(object);
        store.borrow_mut().insert((this + 0x38, name), existing);
        store.borrow_mut().insert((this + 0x14, existing), 9);
        (this, existing, object, store)
    }

    #[test]
    fn not_duplicate_meta_data_merges_a_class_with_the_same_signature() {
        let mut e = logged();
        install_reporter(&mut e, 0);
        let (this, existing, object, store) = known_class(&mut e, 5);
        let (_, hk_class, _) = metadata_classes(&mut e);
        assert_eq!(e.call(0x00db_db70, &args![this, object, hk_class]).u32(), 0);
        assert_eq!(store.borrow()[&(this + 0x14, object)], 9);
        assert_eq!(store.borrow()[&(this + 0x58, object)], existing);
        assert!(calls(&e, REPORT_DOUBLE).is_empty());
        assert_eq!(
            calls(&e, CLASS_GET_SIGNATURE),
            vec![vec![existing, 0], vec![object, 0]]
        );
    }

    #[test]
    fn not_duplicate_meta_data_reports_a_signature_conflict_and_still_merges() {
        let mut e = logged();
        install_reporter(&mut e, 0);
        let (this, existing, object, store) = known_class(&mut e, 6);
        let (_, hk_class, _) = metadata_classes(&mut e);
        assert_eq!(e.call(0x00db_db70, &args![this, object, hk_class]).u32(), 0);
        let reports = calls(&e, REPORT_DOUBLE);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            (reports[0][1], reports[0][2], reports[0][4], reports[0][5]),
            (3, 0x2518_721c, SOURCE_FILE_NAME, 0xb2)
        );
        let name = e.mem.u32(object);
        assert_eq!(appended(&e), vec![TEXT_CONFLICTING, name, TEXT_PERHAPS]);
        assert_eq!(calls(&e, TEXT_BUILDER_DESTRUCT).len(), 1);
        assert_eq!(store.borrow()[&(this + 0x58, object)], existing);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00dbdc40")]
    fn not_duplicate_meta_data_stops_where_the_exe_executes_int3() {
        let mut e = logged();
        install_reporter(&mut e, 1);
        let (this, _, object, _) = known_class(&mut e, 6);
        let (_, hk_class, _) = metadata_classes(&mut e);
        e.call(0x00db_db70, &args![this, object, hk_class]);
    }

    #[test]
    fn not_duplicate_meta_data_treats_enums_like_classes_by_the_name_word() {
        let mut e = logged();
        install_classes(&mut e);
        let store = install_maps(&mut e);
        install_allocator(&mut e);
        let this = writer(&mut e);
        let (_, _, hk_enum) = metadata_classes(&mut e);
        let name = string(&mut e, "Colors");
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, name);
        assert_eq!(e.call(0x00db_db70, &args![this, object, hk_enum]).u32(), 1);
        assert_eq!(store.borrow()[&(this + 0x48, name)], object);
        // An enum of the same name is merged with the earlier one.
        let again = e.mem.alloc(0x10);
        e.mem.set_u32(again, name);
        store.borrow_mut().insert((this + 0x14, object), 4);
        assert_eq!(e.call(0x00db_db70, &args![this, again, hk_enum]).u32(), 0);
        assert_eq!(store.borrow()[&(this + 0x14, again)], 4);
        assert_eq!(store.borrow()[&(this + 0x58, again)], object);
    }

    /// Doubles for `_reserveMore`: the array moves to a block with room for `capacity` more
    /// elements, keeping its elements, and the capacity word grows.
    fn install_reserve_more(e: &mut Engine) {
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) * 2 + 2;
            let block = e.mem.alloc(capacity * element);
            let old = e.mem.u32(array);
            for offset in 0..size * element {
                let byte = e.mem.u8(old + offset);
                e.mem.set_u8(block + offset, byte);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
    }

    #[test]
    fn section_tag_to_index_returns_the_index_of_a_known_tag() {
        let mut e = logged();
        install_allocator(&mut e);
        let store = install_maps(&mut e);
        let this = writer(&mut e);
        store.borrow_mut().insert((this + 0x70, 0x900), 3);
        e.register_double(STRING_DUPLICATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00db_dd40, &args![this, 0x900u32]).u32(), 3);
        assert!(calls(&e, STRING_DUPLICATE).is_empty());
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }

    #[test]
    fn section_tag_to_index_adds_a_new_tag_with_the_next_index() {
        let mut e = logged();
        install_allocator(&mut e);
        let store = install_maps(&mut e);
        install_reserve_more(&mut e);
        let this = writer(&mut e);
        e.register_double(STRING_DUPLICATE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00db_dd40, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.call(0x00db_dd40, &args![this, 0x910u32]).u32(), 1);
        assert_eq!(e.call(0x00db_dd40, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(store.borrow()[&(this + 0x70, 0x900)], 0);
        assert_eq!(store.borrow()[&(this + 0x70, 0x910)], 1);
        assert_eq!(e.mem.u32(this + 0x68), 2);
        let names = e.mem.u32(this + 0x64);
        assert_eq!((e.mem.u32(names), e.mem.u32(names + 4)), (0x900, 0x910));
        assert_eq!(calls(&e, STRING_DUPLICATE), vec![vec![0x900], vec![0x910]]);
    }

    #[test]
    fn add_section_is_the_same_as_the_tag_lookup() {
        let mut e = logged();
        install_allocator(&mut e);
        let store = install_maps(&mut e);
        let this = writer(&mut e);
        store.borrow_mut().insert((this + 0x70, 0x900), 7);
        assert_eq!(e.call(0x00db_def0, &args![this, 0x900u32]).u32(), 7);
    }

    /// A writer, the class maps and a class chain `leaf -> middle -> root` whose names are
    /// "Leaf", "Middle" and "Root".
    fn section_fixture(e: &mut Engine) -> (u32, Store, [u32; 3]) {
        install_allocator(e);
        install_classes(e);
        install_reserve_more(e);
        let store = install_maps(e);
        let this = writer(e);
        e.register_double(STRING_DUPLICATE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let root = class(e, "Root", 0, 0);
        let middle = class(e, "Middle", root, 0);
        let leaf = class(e, "Leaf", middle, 0);
        (this, store, [leaf, middle, root])
    }

    #[test]
    fn find_section_for_prefers_the_object_then_the_class_then_the_parents_then_the_tag() {
        let mut e = logged();
        let (this, store, [leaf, _, root]) = section_fixture(&mut e);
        let leaf_name = e.mem.u32(leaf);
        let root_name = e.mem.u32(root);
        // The tag index is new: 0.
        assert_eq!(
            e.call(0x00db_ddc0, &args![this, 0x500u32, leaf, 0x900u32])
                .u32(),
            0
        );
        // A section for the root class name applies to its descendants.
        store.borrow_mut().insert((this + 0xa8, root_name), 4);
        assert_eq!(
            e.call(0x00db_ddc0, &args![this, 0x500u32, leaf, 0x900u32])
                .u32(),
            4
        );
        // The class name beats the parents'.
        store.borrow_mut().insert((this + 0xa8, leaf_name), 5);
        assert_eq!(
            e.call(0x00db_ddc0, &args![this, 0x500u32, leaf, 0x900u32])
                .u32(),
            5
        );
        // The object beats the class.
        store.borrow_mut().insert((this + 0x9c, 0x500), 6);
        assert_eq!(
            e.call(0x00db_ddc0, &args![this, 0x500u32, leaf, 0x900u32])
                .u32(),
            6
        );
    }

    #[test]
    fn add_pending_write_appends_a_record_and_counts_the_kind() {
        let mut e = logged();
        let (this, store, [leaf, _, _]) = section_fixture(&mut e);
        store.borrow_mut().insert((this + 0x9c, 0x500), 8);
        e.call(
            0x00db_de40,
            &args![this, 0x10u32, 0x20u32, 0x500u32, leaf, 0x900u32],
        );
        e.call(
            0x00db_de40,
            &args![this, 0x11u32, CLASS_OBJECT_C, 0x500u32, leaf, 0x900u32],
        );
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let writes = e.mem.u32(this + 8);
        let record = |i: u32| -> Vec<u32> {
            (0..6)
                .map(|w| e.mem.u32(writes + 0x18 * i + 4 * w))
                .collect()
        };
        assert_eq!(record(0), vec![0x10, 0x20, 0x500, leaf, 8, 0xffff_ffff]);
        assert_eq!(
            record(1),
            vec![0x11, CLASS_OBJECT_C, 0x500, leaf, 8, 0xffff_ffff]
        );
        assert_eq!(e.mem.u32(this + 0x94), 1);
        assert_eq!(e.mem.u32(this + 0x98), 1);
    }

    #[test]
    fn set_section_for_pointer_and_class_enter_the_section_index() {
        let mut e = logged();
        let (this, store, [leaf, _, _]) = section_fixture(&mut e);
        e.call(0x00db_df00, &args![this, 0x500u32, 0x900u32]);
        assert_eq!(store.borrow()[&(this + 0x9c, 0x500)], 0);
        let leaf_name = e.mem.u32(leaf);
        e.call(0x00db_df30, &args![this, leaf, 0x910u32]);
        assert_eq!(store.borrow()[&(this + 0xa8, leaf_name)], 1);
    }

    /// A writer to destroy: names `0x91` and `0x92`, an owned pending-write array and unowned
    /// others.
    fn destroy_fixture(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xd0);
        let names = e.mem.alloc(8);
        e.mem.set_u32(names, 0x91);
        e.mem.set_u32(names + 4, 0x92);
        e.mem.set_u32(this + 0x64, names);
        e.mem.set_u32(this + 0x68, 2);
        e.mem.set_u32(this + 0x6c, 3);
        e.mem.set_u32(this + 8, 0x7000);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 5);
        e.mem.set_u32(this + 0x80, 0x7100);
        e.mem.set_u32(this + 0x84, 1);
        e.mem.set_u32(this + 0x88, 0x8000_0002);
        e.mem.set_u32(this + 0xb8, 0x7200);
        e.mem.set_u32(this + 0xbc, 1);
        e.mem.set_u32(this + 0xc0, 4);
        this
    }

    #[test]
    fn destruct_tears_down_every_member_in_order() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        for address in [
            EASY_FREE,
            MAP_CLEAR_AND_DEALLOCATE,
            NO_EFFECT,
            STRING_MAP_DESTRUCT,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        let this = destroy_fixture(&mut e);
        let names = e.mem.u32(this + 0x64);
        e.call(0x00db_df60, &args![this]);
        assert_eq!(
            calls(&e, EASY_FREE),
            vec![vec![allocator, 0x91], vec![allocator, 0x92]]
        );
        let cleared: Vec<u32> = calls(&e, MAP_CLEAR_AND_DEALLOCATE)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(cleared, vec![0xc4, 0x9c, 0x58, 0x2c, 0x20, 0x14]);
        assert_eq!(calls(&e, NO_EFFECT).len(), 6);
        let strings: Vec<u32> = calls(&e, STRING_MAP_DESTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(strings, vec![0xa8, 0x70, 0x48, 0x38]);
        // Owned data is freed through slot 0x10: the multimap array (4 * 8 bytes), the
        // section names (3 * 4) and the pending writes (5 * 0x18); the array at +0x80 is not
        // owned.
        assert_eq!(
            calls(&e, FREE_DATA_DOUBLE),
            vec![
                vec![allocator, 0x7200, 32],
                vec![allocator, names, 12],
                vec![allocator, 0x7000, 0x78],
            ]
        );
        for (data, size, capacity) in [
            (0xb8, 0xbc, 0xc0),
            (0x80, 0x84, 0x88),
            (0x64, 0x68, 0x6c),
            (8, 0xc, 0x10),
        ] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    /// The construct doubles; returns nothing, the log records the calls.
    fn install_construct_doubles(e: &mut Engine) {
        for address in [
            MAP_RESERVE,
            STRING_MAP_CONSTRUCT,
            MULTI_MAP_CONSTRUCT,
            MAP_INSERT,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn construct_sets_up_the_maps_and_registers_the_class_objects() {
        let mut e = logged();
        let allocator = install_allocator(&mut e);
        install_construct_doubles(&mut e);
        let this = e.mem.alloc(0xd0);
        for offset in (0..0xd0).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let result = e.call(0x00db_e4d0, &args![this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), PACKFILE_WRITER_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        for map in [0x14, 0x20, 0x2c, 0x58, 0x9c] {
            assert_eq!(e.mem.u32(this + map), 0);
            assert_eq!(e.mem.u32(this + map + 4), 0);
            assert_eq!(e.mem.u32(this + map + 8), 0xffff_ffff);
        }
        assert_eq!(e.mem.u32(this + 0x6c), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x88), 0x8000_0000);
        for offset in [0x8, 0xc, 0x64, 0x68, 0x80, 0x84, 0x94, 0x98] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            (e.mem.u32(this + 0x8c), e.mem.u32(this + 0x90)),
            (0xffff_ffff, 0xffff_ffff)
        );
        let reserves: Vec<u32> = calls(&e, MAP_RESERVE).iter().map(|w| w[0] - this).collect();
        assert_eq!(reserves, vec![0x14, 0x20, 0x2c, 0x58, 0x9c]);
        assert!(calls(&e, MAP_RESERVE)
            .iter()
            .all(|w| w[1] == allocator && w[2] == 0));
        let strings: Vec<u32> = calls(&e, STRING_MAP_CONSTRUCT)
            .iter()
            .map(|w| w[0] - this)
            .collect();
        assert_eq!(strings, vec![0x38, 0x48, 0x70, 0xa8]);
        assert_eq!(calls(&e, MULTI_MAP_CONSTRUCT), vec![vec![this + 0xb8]]);
        assert_eq!(
            calls(&e, MAP_INSERT),
            vec![
                vec![this + 0x14, allocator, CLASS_OBJECT_A, 0xffff_ffff],
                vec![this + 0x14, allocator, CLASS_OBJECT_B, 0xffff_ffff],
                vec![this + 0x14, allocator, CLASS_OBJECT_C, 0xffff_ffff],
            ]
        );
    }

    #[test]
    fn set_contents_with_registry_uses_the_global_registry_and_stores_the_indices() {
        let mut e = logged();
        install_allocator(&mut e);
        install_classes(&mut e);
        let store = install_maps(&mut e);
        // The exe global supplies the registry `registry`; its slot 0x10 gives the exact class.
        let registry_table = e.mem.alloc(0x20);
        e.mem.set_u32(registry_table + 0x10, REGISTRY_DOUBLE);
        let registry = e.mem.alloc(0x10);
        e.mem.set_u32(registry, registry_table);
        e.register_double(REGISTRY_DOUBLE, |_, _| Ret {
            eax: 0x2000,
            ..Ret::default()
        });
        e.register_double(SOURCE_DOUBLE, move |_, _| Ret {
            eax: registry,
            ..Ret::default()
        });
        let source_table = e.mem.alloc(0x20);
        e.mem.set_u32(source_table + 0x14, SOURCE_DOUBLE);
        let source = e.mem.alloc(0x10);
        e.mem.set_u32(source, source_table);
        e.mem.set_u32(SOURCE_OBJECT_GLOBAL, source);
        let this = writer(&mut e);
        e.mem.set_u32(DATA_TAG_POINTER, 0x1234);
        // A class with a vtable (odd): exact class 0x2000; the object is already known, so
        // addObject returns at once.
        store.borrow_mut().insert((this + 0x14, 0x500), 3);
        store.borrow_mut().insert((this + 0x14, 0x2000), 4);
        e.call(0x00db_e680, &args![this, 0x500u32, 0x1001u32, 0u32, 0u32]);
        assert_eq!(calls(&e, SOURCE_DOUBLE), vec![vec![source]]);
        assert_eq!(calls(&e, REGISTRY_DOUBLE), vec![vec![registry, 0x500]]);
        assert_eq!(e.mem.u32(this + 0x8c), 3);
        assert_eq!(e.mem.u32(this + 0x90), 4);
        // A class without a vtable is its own exact class and a given registry is not asked.
        e.call_log = Some(Vec::new());
        store.borrow_mut().insert((this + 0x14, 0x1000), 6);
        e.call(
            0x00db_e680,
            &args![this, 0x500u32, 0x1000u32, 0x77u32, 0u32],
        );
        assert!(calls(&e, SOURCE_DOUBLE).is_empty());
        assert!(calls(&e, REGISTRY_DOUBLE).is_empty());
        assert_eq!(e.mem.u32(this + 0x8c), 3);
        assert_eq!(e.mem.u32(this + 0x90), 6);
    }

    /// The doubles `addObject` needs besides the maps, classes and reporter: the relocation
    /// info (empty unless a test fills it), the chase and the headline classes.
    fn install_add_object(e: &mut Engine) -> (u32, Store) {
        let allocator = install_allocator(e);
        install_classes(e);
        install_reserve_more(e);
        install_reporter(e, 0);
        let store = install_maps(e);
        for address in [
            RELOCATION_INFO_CONSTRUCT,
            RELOCATION_INFO_DESTRUCT,
            STRUCTURE_LAYOUT_CONSTRUCT,
            PLATFORM_WRITER_CONSTRUCT,
            OFFSET_STREAM_WRITER_CONSTRUCT,
            PLATFORM_WRITE_OBJECT,
            PLATFORM_WRITER_DESTRUCT,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(STRING_DUPLICATE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        // The class object the recursion ends with has a plain name.
        let name = string(e, "ClassObject");
        e.mem.set_u32(CLASS_OBJECT_C, name);
        e.mem.set_u32(TYPES_TAG_POINTER, 0x7777);
        (allocator, store)
    }

    /// A writer that, like the constructor leaves it, knows the class object `0126cae8`.
    fn add_object_writer(e: &mut Engine, store: &Store) -> u32 {
        let this = writer(e);
        store
            .borrow_mut()
            .insert((this + 0x14, CLASS_OBJECT_C), 0xffff_ffff);
        this
    }

    #[test]
    fn add_object_does_nothing_for_a_known_object() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        store.borrow_mut().insert((this + 0x14, 0x500), 1);
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, 0x1000u32, 0u32, 0u32, 0x900u32],
        );
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn add_object_reports_an_unregistered_class_and_remembers_it() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        // A class with a vtable and no registry has no exact class.
        let class_record = e.mem.alloc(0x20);
        let class_name = string(&mut e, "Derived");
        e.mem.set_u32(class_record, class_name);
        // Make the vtable test see the class as having a vtable.
        e.register_double(CLASS_HAS_VTABLE, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, class_record, 0u32, 0u32, 0x900u32],
        );
        let reports = calls(&e, REPORT_DOUBLE);
        assert_eq!(reports.len(), 1);
        assert_eq!(
            (reports[0][1], reports[0][2], reports[0][4], reports[0][5]),
            (1, 0xabba_abba, SOURCE_FILE_NAME, 0x121)
        );
        assert_eq!(
            appended(&e),
            vec![
                TEXT_UNREGISTERED,
                class_name,
                TEXT_NOT_SERIALIZED,
                TEXT_POINTERS_NULL,
                TEXT_NO_WARNINGS
            ]
        );
        assert_eq!(store.borrow()[&(this + 0x14, 0x500)], 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x84), 1);
        let unregistered = e.mem.u32(this + 0x80);
        assert_eq!(
            (e.mem.u32(unregistered), e.mem.u32(unregistered + 4)),
            (0x500, class_record)
        );
        assert!(calls(&e, PLATFORM_WRITE_OBJECT).is_empty());
    }

    #[test]
    fn add_object_queues_the_object_then_its_class() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        let plain = class(&mut e, "Foo", 0, 0);
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, plain, 0u32, 0u32, 0x900u32],
        );
        // The object got write index 0, its class write index 1.
        assert_eq!(store.borrow()[&(this + 0x14, 0x500)], 0);
        assert_eq!(store.borrow()[&(this + 0x14, plain)], 1);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let writes = e.mem.u32(this + 8);
        let record = |i: u32| -> Vec<u32> {
            (0..4)
                .map(|w| e.mem.u32(writes + 0x18 * i + 4 * w))
                .collect()
        };
        assert_eq!(record(0), vec![0x500, plain, 0x500, plain]);
        assert_eq!(
            record(1),
            vec![plain, CLASS_OBJECT_C, plain, CLASS_OBJECT_C]
        );
        assert_eq!(e.mem.u32(this + 0x94), 1);
        assert_eq!(e.mem.u32(this + 0x98), 1);
        // The object was chased through the platform writer.
        let writes = calls(&e, PLATFORM_WRITE_OBJECT);
        assert_eq!(writes.len(), 2);
        assert_eq!((writes[0][2], writes[0][3]), (0x500, plain));
        assert_eq!((writes[1][2], writes[1][3]), (plain, CLASS_OBJECT_C));
        assert_eq!(calls(&e, RELOCATION_INFO_DESTRUCT).len(), 2);
    }

    #[test]
    fn add_object_lets_the_rewriter_replace_the_object() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        let plain = class(&mut e, "Foo", 0, 0);
        e.register_double(REWRITER_DOUBLE, |e, a| {
            e.mem.set_u32(a[1], 0x600);
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, REWRITER_DOUBLE);
        let rewriter = e.mem.alloc(0x10);
        e.mem.set_u32(rewriter, table);
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, plain, 0u32, rewriter, 0x900u32],
        );
        // Both addresses got the write index; the replacement is merged with the original.
        assert_eq!(store.borrow()[&(this + 0x14, 0x500)], 0);
        assert_eq!(store.borrow()[&(this + 0x14, 0x600)], 0);
        assert_eq!(store.borrow()[&(this + 0x58, 0x600)], 0x500);
        let writes = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(writes), 0x600);
        assert_eq!(e.mem.u32(writes + 8), 0x500);
    }

    #[test]
    fn add_object_enters_minus_one_for_an_object_the_rewriter_drops() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        let plain = class(&mut e, "Foo", 0, 0);
        e.register_double(REWRITER_DOUBLE, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(table + 0xc, REWRITER_DOUBLE);
        let rewriter = e.mem.alloc(0x10);
        e.mem.set_u32(rewriter, table);
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, plain, 0u32, rewriter, 0x900u32],
        );
        assert_eq!(store.borrow()[&(this + 0x14, 0x500)], 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls(&e, PLATFORM_WRITE_OBJECT).is_empty());
    }

    #[test]
    fn add_object_follows_the_relocations_of_the_object() {
        let mut e = logged();
        let (_, store) = install_add_object(&mut e);
        let this = add_object_writer(&mut e, &store);
        let plain = class(&mut e, "Foo", 0, 0);
        let other = class(&mut e, "Other", 0, 0);
        // The first chase fills the relocation info with two entries (target, class, flag):
        // an object with a flag and a null class one that is skipped.
        let info_entries = e.mem.alloc(0x20);
        for (i, word) in [0u32, 0x700, other, 1, 0, 0x701, 0, 1]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(info_entries + 4 * i as u32, word);
        }
        let inserted = Rc::new(RefCell::new(Vec::new()));
        let insert_log = Rc::clone(&inserted);
        e.register_double(MULTI_MAP_INSERT, move |e, a| {
            insert_log.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let first = Rc::new(RefCell::new(true));
        let first_flag = Rc::clone(&first);
        e.register_double(PLATFORM_WRITE_OBJECT, move |e, a| {
            if std::mem::replace(&mut *first_flag.borrow_mut(), false) {
                e.mem.set_u32(a[4] + 0xc, info_entries);
                e.mem.set_u32(a[4] + 0x10, 2);
            }
            Ret::default()
        });
        e.call(
            0x00db_e150,
            &args![this, 0x500u32, plain, 0u32, 0u32, 0x900u32],
        );
        // The flagged target without a section gets the written object's section (default:
        // the new tag index 0).
        assert_eq!(store.borrow()[&(this + 0x9c, 0x700)], 0);
        // The target was added with its class (write indices 1 for it, 2 for its class).
        assert!(store.borrow().contains_key(&(this + 0x14, 0x700)));
        assert!(!store.borrow().contains_key(&(this + 0x14, 0x701)));
        // The multimap learned that the object points at the target's index.
        let inserts = inserted.borrow();
        assert_eq!(inserts.len(), 1);
        assert_eq!(
            inserts[0],
            (this + 0xb8, 0x500, store.borrow()[&(this + 0x14, 0x700)])
        );
    }
}
