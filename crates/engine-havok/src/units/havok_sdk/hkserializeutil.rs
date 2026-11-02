//! `Havok SDK/hkserializeutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `hkSerializeUtil` (format detection, `load`, `loadOnHeap`, `saveTagfile`) and the
//! helpers of its anonymous namespace. The compiler gave many of those helpers register
//! calling conventions (inputs in `EAX`, `EDX`, `EBX`, `ESI`, `EDI`); their translations take the
//! register values first (in the order `EAX`, `ECX`, `EDX`, `EBX`, `ESI`, `EDI`, as each doc
//! comment lists them) and then the stack words in the order the code reads them.
//!
//! Several of the functions pass the address of one of their own stack-argument slots as an
//! out parameter (a `bool` or a handle written over a word the code no longer needs); the
//! translations give such an out parameter its own four-byte block holding the word the slot held.
//! The functions have no exception frames and no stack-protector cookie.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): the allocators are at `+0xc` and `+0x10`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The singleton whose slot `0xc` hands out the type-info registry and slot `0x10` the class
/// registry (both `thiscall`, no arguments); it is written at run time.
const REGISTRIES_SINGLETON: u32 = 0x0126_8b08;
/// The singleton the loaders forward formats they cannot read to: its slot `0x10` (`isLoadable`,
/// one word), `0x14` (`loadOldPackfile`, three words) and `0x18` (`loadOldPackfileOnHeap`, three
/// words) have the shapes of the stubs in `hkserializedeprecated.cpp`; written at run time.
const DEPRECATED_LOADER_SINGLETON: u32 = 0x0126_9b74;
/// The version patch manager (written at run time).
const PATCH_MANAGER_SINGLETON: u32 = 0x0126_9b70;
/// The four bytes that name the structure layout of the platform ("layout rules").
const PLATFORM_LAYOUT_RULES: u32 = 0x011b_7b88;
/// `"Havok-7.1.0-r1"`.
const CURRENT_CONTENTS_VERSION: u32 = 0x010c_f830;

/// First and second magic word of a packfile.
const PACKFILE_MAGIC_FIRST: u32 = 0x57e0_e057;
const PACKFILE_MAGIC_SECOND: u32 = 0x10c0_c010;
/// First and second magic word of a binary tagfile.
const TAGFILE_MAGIC_FIRST: u32 = 0xcab0_0d1e;
const TAGFILE_MAGIC_SECOND: u32 = 0xd011_face;

/// The vtable of `hkBinaryTagfileWriter` (`010cf9c8`, RTTI `.?AVhkBinaryTagfileWriter@@`).
const TAGFILE_WRITER_VTABLE: u32 = 0x010c_f9c8;
/// The vtable of `ForwardingPackfileListerer` (`010cf9b4`; RTTI `.?AV` + the anonymous-namespace
/// class named `ForwardingPackfileListerer`).
const LISTENER_VTABLE: u32 = 0x010c_f9b4;
/// The vtable of `hkBaseObject` that a destroyed listener is left with (`0102e388`).
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The vtable of the local type-info registry (`010cf7b0`, RTTI `.?AVhkTypeInfoRegistry@@`).
const TYPE_INFO_REGISTRY_VTABLE: u32 = 0x010c_f7b0;

/// `hkStringPtr::hkStringPtr`, `~hkStringPtr`, `operator=` (one string word), `set` (start, length)
/// and `printf` (cdecl: the string pointer, the format, then its arguments).
const STRING_PTR_CONSTRUCTOR: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCTOR: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
const STRING_PTR_SET: u32 = 0x00cb_0c50;
const STRING_PTR_PRINTF: u32 = 0x00cb_0c70;
/// `hkString`: `strStr`, `strLen`, `strChr`, `strCmp`, `memCmp`, `memSet` and `atoi` (all cdecl).
const STRING_STR: u32 = 0x00cb_1130;
const STRING_LEN: u32 = 0x00cb_10d0;
const STRING_CHR: u32 = 0x00cb_1140;
const STRING_COMPARE: u32 = 0x00cb_0fa0;
const MEMORY_COMPARE: u32 = 0x00cb_1290;
const MEMORY_SET: u32 = 0x00cb_1280;
const STRING_TO_INT: u32 = 0x00cb_10f0;
/// `hkMemUtil::memSet` and `hkMemUtil::memCpy` (cdecl, three words) and the C runtime's `memset`.
const MEM_UTIL_SET: u32 = 0x00d2_cab0;
const MEM_UTIL_COPY: u32 = 0x00d2_caa0;
const CRT_MEMSET: u32 = 0x00ec_61c0;
/// `hkStringBuf::hkStringBuf` (`thiscall`, a C string).
const STRING_BUF_CONSTRUCTOR: u32 = 0x00d6_1a50;
/// `hkArrayUtil::_reserve` (cdecl: allocator, array, count, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject::removeReference` (no map name; `thiscall`).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The string map (`hkCachedHashMap<hkStringMapOperations,...>`) constructor (one ignored stack
/// word) and destructor.
const STRING_MAP_CONSTRUCTOR: u32 = 0x00d8_7790;
const STRING_MAP_DESTRUCTOR: u32 = 0x00d8_77d0;
/// `hkTypeInfoRegistry::merge` (`thiscall`, the registry to merge in).
const TYPE_INFO_REGISTRY_MERGE: u32 = 0x00d4_7970;
/// `hkVersionUtil::getCurrentVersion` (no arguments).
const CURRENT_VERSION: u32 = 0x00dc_0fb0;
/// `hkNativePackfileUtils::load_ov2` (cdecl: data, size, type-info registry).
const NATIVE_PACKFILE_LOAD: u32 = 0x00dc_0cd0;
/// `hkBinaryTagfileWriter::save` (thiscall, `RET 0xc`: object, stream, listener; `this` is the
/// writer object the caller builds on its stack), reached the way the code reaches it: through
/// the word `010cf9d4` (slot `0xc` of the writer's vtable).
const TAGFILE_WRITER_SAVE_SLOT: u32 = TAGFILE_WRITER_VTABLE + 0xc;
/// `hkBinaryTagfileReader::hkBinaryTagfileReader` and `load` (`this`, output handle, stream,
/// world).
const TAGFILE_READER_CONSTRUCTOR: u32 = 0x00dc_5d30;
const TAGFILE_READER_LOAD: u32 = 0x00dc_5d50;
/// `hkTagfileReader::detectFormat` (cdecl: output word, stream).
const TAGFILE_READER_DETECT_FORMAT: u32 = 0x00dc_ca60;
/// `hkDataWorldNative`: constructor, destructor, `setContents`, `getContents`, `setClassRegistry`
/// and `wrapObject`.
const WORLD_CONSTRUCTOR: u32 = 0x00db_b360;
const WORLD_DESTRUCTOR: u32 = 0x00db_b3f0;
const WORLD_SET_CONTENTS: u32 = 0x00db_b730;
const WORLD_GET_CONTENTS: u32 = 0x00db_b620;
const WORLD_SET_CLASS_REGISTRY: u32 = 0x00db_b490;
const WORLD_WRAP_OBJECT: u32 = 0x00db_b4f0;
/// `hkDataWorldDict`: constructor, destructor and `getContents` (`thiscall`, output handle).
const DICT_CONSTRUCTOR: u32 = 0x00dc_8030;
const DICT_DESTRUCTOR: u32 = 0x00dc_8080;
const DICT_GET_CONTENTS: u32 = 0x00dc_72a0;
/// `hkDataObject`: constructor from an implementation, copy constructor (`_ov2`), destructor,
/// `isNull`, `getClass` (output class handle) and `getHandle` (output pair).
const DATA_OBJECT_CONSTRUCTOR: u32 = 0x00db_a760;
const DATA_OBJECT_COPY_CONSTRUCTOR: u32 = 0x00db_a780;
const DATA_OBJECT_DESTRUCTOR: u32 = 0x00db_a7f0;
const DATA_OBJECT_IS_NULL: u32 = 0x00db_a980;
const DATA_OBJECT_GET_CLASS: u32 = 0x00db_a850;
const DATA_OBJECT_GET_HANDLE: u32 = 0x00db_a810;
/// `hkDataClass::~hkDataClass` and `hkDataClass::getName`.
const DATA_CLASS_DESTRUCTOR: u32 = 0x00db_a660;
const DATA_CLASS_GET_NAME: u32 = 0x00db_a680;
/// `hkDataObjectUtil::toObjectWithRegistry` (cdecl: handle, class registry, type-info registry) and
/// `toResourceWithRegistry` (cdecl: handle, class registry).
const TO_OBJECT_WITH_REGISTRY: u32 = 0x00dc_32c0;
const TO_RESOURCE_WITH_REGISTRY: u32 = 0x00dc_3300;
/// `hkClass::getDescribedVersion` (`thiscall`).
const CLASS_GET_DESCRIBED_VERSION: u32 = 0x00d4_7120;
/// `hkVersionPatchManager`: `recomputePatchDependencies`, `getUid` (name, version; returns the
/// 64-bit uid in `EDX:EAX`) and `applyPatches` (world, class registry).
const RECOMPUTE_PATCH_DEPENDENCIES: u32 = 0x00db_8670;
const PATCH_MANAGER_GET_UID: u32 = 0x00db_7dd0;
const APPLY_PATCHES: u32 = 0x00db_a0d0;
/// `hkMapBase<uint64,uint64>::getWithDefault` (`thiscall`: key low and high, default low and high).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1cb0;
/// `hkIstream::hkIstream` (`thiscall`: buffer, size; returns `this`) and its destructor.
const ISTREAM_CONSTRUCTOR: u32 = 0x00dc_c6a0;
const ISTREAM_DESTRUCTOR: u32 = 0x00dc_c160;

/// The strings the unit's functions use, as addresses in the exe's data.
const KEY_CLASS_VERSION: u32 = 0x010c_fa58;
const KEY_CONTENTS_VERSION: u32 = 0x010c_fa48;
const TAG_PACKFILE_OPEN: u32 = 0x010c_fa68;
const TAG_PACKFILE_OPEN_SPACE: u32 = 0x010c_fa84;
const TAG_PACKFILE_OPEN_CLOSE: u32 = 0x010c_fa74;
const VERSION_3_0_0: u32 = 0x010c_fa28;
const VERSION_3_1_0: u32 = 0x010c_fa3c;
const VERSION_ERROR: u32 = 0x010c_fa34;
const MESSAGE_UNABLE_TO_DETECT: u32 = 0x010c_fa94;
const MESSAGE_STREAM_NOT_OK: u32 = 0x010c_fab8;
const MESSAGE_STREAM_NULL: u32 = 0x010c_facc;
const MESSAGE_WRONG_PLATFORM: u32 = 0x010c_f9d8;
const FORMAT_UNABLE_TO_VERSION: u32 = 0x010c_f9f4;

/// The formats `detectFormat` reports in the first word of its result (the names are the
/// code's: 2 is checked against the packfile header, 4 is a binary tagfile; 3 is the XML packfile
/// the code reads `<hkpackfile` for).
const FORMAT_PACKFILE: u32 = 2;
const FORMAT_XML_PACKFILE: u32 = 3;
const FORMAT_BINARY_TAGFILE: u32 = 4;

layout! {
    /// `_anonymous_namespace_::ForwardingPackfileListerer` (engine map name), 0x10 bytes as built
    /// on the stack by `00d485a0`: vtable, a 16-bit word 1 at `+6`, the listener the callback
    /// forwards to and the world.
    pub struct ForwardingPackfileListerer: 0x10 {
        /// The listener whose slot `0xc` the callback asks to replace objects.
        0x08 forwarded_listener: Ptr,
        /// The `hkDataWorldNative` objects are wrapped for.
        0x0c world: Ptr,
    }
}

/// Gives an array's storage back the way the callers do: through slot `0x10` of `allocator`, with
/// the data pointer and the byte count (the callers have checked that the array owns it).
fn release_storage(e: &mut Engine, allocator: u32, data: u32, bytes: u32) {
    e.vcall(allocator, 0x10, &args![data, bytes]);
}

// Translated from 00d483a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkBinaryTagfile::isBinaryMagic` (engine map name), cdecl, three words: writes 1 at the byte
/// `out` when `first` and `second` are the tagfile's two magic words (`0xcab00d1e`,
/// `0xd011face`), either as they are or with the bytes of each swapped; else 0. Returns `out`.
/// (The code also stores the swapped words over its argument slots; nothing reads them.)
pub fn hk_binary_tagfile_is_binary_magic(e: &mut Engine, out: u32, first: u32, second: u32) -> u32 {
    let native = first == TAGFILE_MAGIC_FIRST && second == TAGFILE_MAGIC_SECOND;
    let swapped =
        first.swap_bytes() == TAGFILE_MAGIC_FIRST && second.swap_bytes() == TAGFILE_MAGIC_SECOND;
    e.mem.set_u8(out, u8::from(native || swapped));
    out
}

// Translated from 00d48420 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Register input `EAX` = a C string; returns (in `EAX`) the address of its first
/// character that is not a space (`0x20`).
pub fn fn_00d48420(e: &mut Engine, text: u32) -> u32 {
    let mut at = text;
    while e.mem.u8(at) == b' ' {
        at = at.wrapping_add(1);
    }
    at
}

// Translated from 00d48440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findTextValue` (engine map name). Register inputs `EAX` = `haystack`, `EDI` = `needle`; one
/// stack word `destination` (an `hkStringPtr`); cdecl. Finds `needle` in `haystack`, requires
/// `= "` after it (spaces allowed before the `=` and the quote), and stores the text up to
/// the next quote in `destination` (`hkStringPtr::set`). Returns 1 when it did, else 0.
pub fn find_text_value(e: &mut Engine, haystack: u32, needle: u32, destination: u32) -> u32 {
    let found = e.call(STRING_STR, &args![haystack, needle]).u32();
    if found == 0 {
        return 0;
    }
    let length = e.call(STRING_LEN, &args![needle]).u32();
    let after_key = found.wrapping_add(length);
    let equals = fn_00d48420(e, after_key);
    if e.mem.u8(equals) != b'=' {
        return 0;
    }
    let quote = fn_00d48420(e, equals.wrapping_add(1));
    if e.mem.u8(quote) != b'"' {
        return 0;
    }
    let start = quote.wrapping_add(1);
    let end = e.call(STRING_CHR, &args![start, 0x22u32]).u32();
    if end == 0 {
        return 0;
    }
    e.call(
        STRING_PTR_SET,
        &args![destination, start, end.wrapping_sub(start)],
    );
    1
}

// Translated from 00d48560 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Register inputs `EAX` = `stream`, `EDX` = `listener`; one stack word `object`
/// (an `hkDataObject`); cdecl. Without a stream answers 1. Otherwise builds an
/// `hkBinaryTagfileWriter` object on the stack (vtable `010cf9c8`, the word 1 at `+6`) and calls its
/// `save` through the vtable word `010cf9d4` with `(object, stream, listener)`; answers its result.
pub fn fn_00d48560(e: &mut Engine, stream: u32, listener: u32, object: u32) -> u32 {
    if stream == 0 {
        return 1;
    }
    // The exe's frame (`SUB ESP,0x8`): the writer is all of it (00d48578).
    e.with_stack(8, |e, writer| {
        let writer = writer.addr();
        e.mem.set_u16(writer + 6, 1);
        e.mem.set_u32(writer, TAGFILE_WRITER_VTABLE);
        let save = e.mem.u32(TAGFILE_WRITER_SAVE_SLOT);
        e.call(save, &args![writer, object, stream, listener]).u32()
    })
}

// Translated from 00d485a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::saveTagfile` (engine map name), cdecl, five words. Builds an
/// `hkDataWorldNative` on the stack, sets its contents to `object` of `class`, then writes the
/// contents as a tagfile to `stream` (`00d48560`) with a `ForwardingPackfileListerer` made on the
/// stack from `forwarded_listener` and the world. `_unused_3` is never read. The out handle for the
/// contents is the stack slot of the last argument itself; the listener keeps the word it held.
/// Returns the result of the write.
pub fn hk_serialize_util_save_tagfile(
    e: &mut Engine,
    object: u32,
    class: u32,
    stream: u32,
    _unused_3: u32,
    forwarded_listener: u32,
) -> u32 {
    // The exe's frame (`SUB ESP,0x38`, one push): 0x3c bytes, the listener at +0x4 (00d485d7)
    // and the world at +0x14 (00d485a4). The handle is the last argument's own slot, +0x50 past
    // the return address at +0x3c (`LEA EDX,[ESP+0x50]`, 00d485ce).
    e.with_stack(0x54, |e, frame| {
        let frame = frame.addr();
        let listener = frame + 4;
        let world = frame + 0x14;
        let handle = frame + 0x50;
        for (slot, word) in [object, class, stream, _unused_3].into_iter().enumerate() {
            e.mem.set_u32(frame + 0x40 + slot as u32 * 4, word);
        }
        e.call(WORLD_CONSTRUCTOR, &args![world]);
        e.call(WORLD_SET_CONTENTS, &args![world, object, class]);
        e.mem.set_u32(handle, forwarded_listener);
        e.mem.set_u16(listener + 6, 1);
        e.mem.set_u32(listener, LISTENER_VTABLE);
        e.mem.set_u32(listener + 8, forwarded_listener);
        e.mem.set_u32(listener + 0xc, world);
        let contents = e.call(WORLD_GET_CONTENTS, &args![world, handle]).u32();
        let result = fn_00d48560(e, stream, listener, contents);
        e.call(DATA_OBJECT_DESTRUCTOR, &args![handle]);
        e.mem.set_u32(listener, BASE_OBJECT_VTABLE);
        e.call(WORLD_DESTRUCTOR, &args![world]);
        result
    })
}

// Translated from 00d48680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::loadStreamIntoBuffer` (engine map name). Register inputs `EBX` =
/// `stream` (an `hkStreamReader`), `ESI` = `buffer` (an `hkArray<char>`: data, size, capacity
/// and flags); no stack words. Reads the whole stream into the array. A stream that can seek
/// (slot `0x24` writes a `bool` and answers its pointer) is measured (slot `0x28` seeks to the
/// end with `(0, 2)`, slot `0x2c` tells the position), the array is reserved for that size (at
/// least double the capacity), the stream rewound and read in one go (slot `0x10`). Any other
/// stream is read in blocks of `0x20000` bytes (reserving room for each) until a read returns
/// no more than 0.
pub fn load_stream_into_buffer(e: &mut Engine, stream: u32, buffer: u32) {
    // The exe's frame (`PUSH ECX`, `PUSH EDI`): 8 bytes, the flag byte at +0x7 (00d48687).
    let seekable = e.with_stack(8, |e, frame| {
        let flag_at = e.vcall(stream, 0x24, &args![frame.addr() + 7]).u32();
        e.mem.u8(flag_at) != 0
    });
    if seekable {
        e.vcall(stream, 0x28, &args![0u32, 2u32]);
        let size = e.vcall(stream, 0x2c, &args![]).i32();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = (e.mem.u32(buffer + 8) & 0x3fff_ffff) as i32;
        if capacity < size {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if size < doubled { doubled } else { size };
            e.call(ARRAY_RESERVE, &args![allocator, buffer, wanted, 1u32]);
        }
        e.mem.set_i32(buffer + 4, size);
        e.vcall(stream, 0x28, &args![0u32, 0u32]);
        let count = e.mem.u32(buffer + 4);
        let data = e.mem.u32(buffer);
        e.vcall(stream, 0x10, &args![data, count]);
        return;
    }
    loop {
        let mut wanted = e.mem.i32(buffer + 4).wrapping_add(0x20000);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0xc);
        let capacity = (e.mem.u32(buffer + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            let doubled = capacity.wrapping_add(capacity);
            if wanted < doubled {
                wanted = doubled;
            }
            e.call(ARRAY_RESERVE, &args![allocator, buffer, wanted, 1u32]);
        }
        let data = e.mem.u32(buffer);
        let size = e.mem.u32(buffer + 4);
        let read = e
            .vcall(stream, 0x10, &args![size.wrapping_add(data), 0x20000u32])
            .i32();
        let size = e.mem.u32(buffer + 4);
        e.mem.set_u32(buffer + 4, size.wrapping_add(read as u32));
        if read <= 0 {
            break;
        }
    }
}

/// Stores `code` at `error_details` and `message` in the `hkStringPtr` after it, when there is a
/// result structure (the shape of every error the loaders report).
fn report_error(e: &mut Engine, error_details: u32, code: u32, message: u32) {
    if error_details != 0 {
        e.mem.set_u32(error_details, code);
        e.call(STRING_PTR_ASSIGN, &args![error_details + 4, message]);
    }
}

// Translated from 00d48760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::loadPackfile` (engine map name). Register inputs `EAX` =
/// `error_details`, `ECX` = `header` (the format record `detectFormat` made: format at `+0`,
/// contents version string at `+4`, the four layout-rule bytes at `+8`), `EDX` = `stream`; one
/// stack word `type_info_registry`; cdecl. A native packfile (format 2) of this platform and
/// contents version is read into a buffer (`loadStreamIntoBuffer`) and loaded
/// (`hkNativePackfileUtils::load_ov2` with `type_info_registry`), the buffer released; a wrong
/// platform reports error 3 ("Wrong platform for packfile") and answers 0; every other case goes to
/// slot `0x14` of the deprecated-format loader singleton with `(stream, header, error_details)`.
pub fn load_packfile(
    e: &mut Engine,
    error_details: u32,
    header: u32,
    stream: u32,
    type_info_registry: u32,
) -> u32 {
    if e.mem.u32(header) == FORMAT_PACKFILE {
        let differs = e
            .call(
                MEMORY_COMPARE,
                &args![header + 8, PLATFORM_LAYOUT_RULES, 4u32],
            )
            .u32();
        if differs != 0 {
            report_error(e, error_details, 3, MESSAGE_WRONG_PLATFORM);
            return 0;
        }
        let version = e.call(CURRENT_VERSION, &args![]).u32();
        if hk_string_ptr_operator_ov3(e, Ptr::new(header + 4), version) {
            // The exe's frame (`SUB ESP,0xc`, three pushes): 0x18 bytes, the buffer array at
            // +0xc (00d487a4).
            return e.with_stack(0x18, |e, frame| {
                let buffer = frame.addr() + 0xc;
                e.mem.set_u32(buffer, 0);
                e.mem.set_u32(buffer + 4, 0);
                e.mem.set_u32(buffer + 8, 0x8000_0000);
                load_stream_into_buffer(e, stream, buffer);
                let data = e.mem.u32(buffer);
                let size = e.mem.u32(buffer + 4);
                let resource = e
                    .call(NATIVE_PACKFILE_LOAD, &args![data, size, type_info_registry])
                    .u32();
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let flags = e.mem.u32(buffer + 8);
                let allocator = e.mem.u32(router + 0xc);
                e.mem.set_u32(buffer + 4, 0);
                if (flags as i32) >= 0 {
                    let data = e.mem.u32(buffer);
                    release_storage(e, allocator, data, flags & 0x3fff_ffff);
                }
                resource
            });
        }
    }
    let singleton = e.mem.u32(DEPRECATED_LOADER_SINGLETON);
    e.vcall(singleton, 0x14, &args![stream, header, error_details])
        .u32()
}

// Translated from 00d48840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::loadNewPackfileOnHeap` (engine map name). Register input `EAX` =
/// `class_registry`; four stack words `stream`, `header` (see `load_packfile`), `error_details`
/// and `type_info_registry`; cdecl. Builds a local type-info registry (vtable `010cf7b0`, the word 1
/// at `+6`, a string map at `+8` whose constructor ignores its one stack word, `0` at `+0x18`)
/// merged from `type_info_registry`, loads the packfile with it (`load_packfile`), builds an
/// `hkDataWorldNative` on the stack whose class registry is `class_registry`, asks the resource
/// for its contents type (slot `0x1c`), looks the class up in `class_registry` (slot `0x10`) and,
/// when found, gets the resource's contents (slot `0x18`, `(0, type_info_registry)`), wraps them in the
/// world (`wrapObject`) and converts the handle with `toObjectWithRegistry`. Releases the
/// world, the resource and the local registry's map. Answers the converted object, or 0 when
/// the class is not found.
pub fn load_new_packfile_on_heap(
    e: &mut Engine,
    class_registry: u32,
    stream: u32,
    header: u32,
    error_details: u32,
    type_info_registry: u32,
) -> u32 {
    // The exe's frame (`SUB ESP,0x48`, four pushes): 0x58 bytes, the data-object handle at +0x10
    // (00d488e1), the local registry at +0x14 (00d48865) and the world at +0x30 (00d4889d).
    e.with_stack(0x58, |e, frame| {
        let frame = frame.addr();
        load_new_packfile_in_frame(
            e,
            frame,
            class_registry,
            stream,
            header,
            error_details,
            type_info_registry,
        )
    })
}

/// [`load_new_packfile_on_heap`] in its frame.
fn load_new_packfile_in_frame(
    e: &mut Engine,
    frame: u32,
    class_registry: u32,
    stream: u32,
    header: u32,
    error_details: u32,
    type_info_registry: u32,
) -> u32 {
    let local_registry = frame + 0x14;
    e.mem.set_u16(local_registry + 6, 1);
    e.mem.set_u32(local_registry, TYPE_INFO_REGISTRY_VTABLE);
    e.call(STRING_MAP_CONSTRUCTOR, &args![local_registry + 8, 0u32]);
    e.mem.set_u32(local_registry + 0x18, 0);
    e.call(
        TYPE_INFO_REGISTRY_MERGE,
        &args![local_registry, type_info_registry],
    );
    let resource = load_packfile(e, error_details, header, stream, local_registry);
    let world = frame + 0x30;
    e.call(WORLD_CONSTRUCTOR, &args![world]);
    e.call(WORLD_SET_CLASS_REGISTRY, &args![world, class_registry]);
    let registry_vtable = e.mem.u32(class_registry);
    let contents_type = e.vcall(resource, 0x1c, &args![]).u32();
    let lookup = e.mem.u32(registry_vtable + 0x10);
    let class = e.call(lookup, &args![class_registry, contents_type]).u32();
    if class == 0 {
        e.call(WORLD_DESTRUCTOR, &args![world]);
        e.call(REMOVE_REFERENCE, &args![resource]);
        e.call(STRING_MAP_DESTRUCTOR, &args![local_registry + 8]);
        0
    } else {
        let object = e
            .vcall(resource, 0x18, &args![0u32, type_info_registry])
            .u32();
        let wrapped = e
            .call(WORLD_WRAP_OBJECT, &args![world, object, class])
            .u32();
        let handle = frame + 0x10;
        e.call(DATA_OBJECT_CONSTRUCTOR, &args![handle, wrapped]);
        let converted = e
            .call(
                TO_OBJECT_WITH_REGISTRY,
                &args![handle, class_registry, type_info_registry],
            )
            .u32();
        e.call(DATA_OBJECT_DESTRUCTOR, &args![handle]);
        e.call(WORLD_DESTRUCTOR, &args![world]);
        e.call(REMOVE_REFERENCE, &args![resource]);
        e.call(STRING_MAP_DESTRUCTOR, &args![local_registry + 8]);
        converted
    }
}

// Translated from 00d48950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::checkTagfileWorld` (engine map name). Register input `EAX` = `world`;
/// three stack words `error_details`, `patch_manager`, `class_registry`; cdecl. Recomputes the
/// patch dependencies, asks the world for all its classes (slot `0x20`, into a local array)
/// and checks each (name from slot `8`, version from slot `0xc`): the class registry
/// (slot `0x10`) must know the class at that version (`hkClass::getDescribedVersion`) or the
/// patch manager's map (`+0x18`, keyed by `getUid(name, version)`) must have a patch for
/// it. The first class that fails reports error 4 ("Unable to version data of class %s, version
/// 0x%p.") and answers 0; otherwise answers 1. The local array's storage is released either way.
pub fn check_tagfile_world(
    e: &mut Engine,
    world: u32,
    error_details: u32,
    patch_manager: u32,
    class_registry: u32,
) -> u32 {
    e.call(RECOMPUTE_PATCH_DEPENDENCIES, &args![patch_manager]);
    // The exe's frame (`SUB ESP,0xc`, four pushes): 0x1c bytes, the class array at +0x10
    // (00d4896b).
    e.with_stack(0x1c, |e, frame| {
        check_tagfile_world_in_frame(
            e,
            frame.addr() + 0x10,
            world,
            error_details,
            patch_manager,
            class_registry,
        )
    })
}

/// [`check_tagfile_world`] with its class array at `array`.
fn check_tagfile_world_in_frame(
    e: &mut Engine,
    array: u32,
    world: u32,
    error_details: u32,
    patch_manager: u32,
    class_registry: u32,
) -> u32 {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    e.vcall(world, 0x20, &args![array]);
    let mut index: u32 = 0;
    if e.mem.i32(array + 4) > 0 {
        loop {
            let data = e.mem.u32(array);
            let class_object = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            let name = e.vcall(class_object, 8, &args![]).u32();
            let data = e.mem.u32(array);
            let class_object = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
            let version = e.vcall(class_object, 0xc, &args![]).u32();
            let known = e.vcall(class_registry, 0x10, &args![name]).u32();
            let current =
                known != 0 && e.call(CLASS_GET_DESCRIBED_VERSION, &args![known]).u32() == version;
            if !current {
                let uid = e
                    .call(PATCH_MANAGER_GET_UID, &args![patch_manager, name, version])
                    .u64();
                let patch = e
                    .call(
                        MAP_GET_WITH_DEFAULT,
                        &args![
                            patch_manager + 0x18,
                            uid as u32,
                            (uid >> 32) as u32,
                            0xffff_ffffu32,
                            0xffff_ffffu32
                        ],
                    )
                    .u32();
                if patch == 0xffff_ffff {
                    e.mem.set_u32(error_details, 4);
                    e.call(
                        STRING_PTR_PRINTF,
                        &args![error_details + 4, FORMAT_UNABLE_TO_VERSION, name, version],
                    );
                    let router = e.call(MEMORY_ROUTER, &args![]).u32();
                    let flags = e.mem.u32(array + 8);
                    let allocator = e.mem.u32(router + 0x10);
                    e.mem.set_u32(array + 4, 0);
                    if (flags as i32) >= 0 {
                        let data = e.mem.u32(array);
                        release_storage(e, allocator, data, (flags & 0x3fff_ffff) * 4);
                    }
                    return 0;
                }
            }
            index = index.wrapping_add(1);
            if (index as i32) >= e.mem.i32(array + 4) {
                index = 0;
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, index);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(array);
        release_storage(e, allocator, data, (flags & 0x3fff_ffff) * 4);
    }
    1
}

// Translated from 00d48a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::loadBinaryTagfileIntoWorld` (engine map name). Register inputs `ESI` =
/// `world`, `EDI` = `error_details`, `EBX` = `class_registry`; one stack word `stream`; cdecl.
/// Reads the tagfile from `stream` into the world (an `hkBinaryTagfileReader` on the stack,
/// `load` into a throw-away handle), then, when there is an error structure, checks the world
/// (`checkTagfileWorld` with the patch manager singleton) and answers 1 if that fails; otherwise
/// applies the patches (`hkVersionPatchManager::applyPatches`) and answers its result.
pub fn load_binary_tagfile_into_world(
    e: &mut Engine,
    world: u32,
    error_details: u32,
    class_registry: u32,
    stream: u32,
) -> u32 {
    // The exe's frame (`SUB ESP,0xc`): the handle at +0 (00d48aa2) and the reader at +0x4
    // (00d48a93).
    e.with_stack(0xc, |e, frame| {
        let handle = frame.addr();
        let reader = handle + 4;
        e.call(TAGFILE_READER_CONSTRUCTOR, &args![reader]);
        e.call(TAGFILE_READER_LOAD, &args![reader, handle, stream, world]);
        e.call(DATA_OBJECT_DESTRUCTOR, &args![handle]);
    });
    if error_details != 0 {
        let patch_manager = e.mem.u32(PATCH_MANAGER_SINGLETON);
        if check_tagfile_world(e, world, error_details, patch_manager, class_registry) == 0 {
            return 1;
        }
    }
    let patch_manager = e.mem.u32(PATCH_MANAGER_SINGLETON);
    e.call(APPLY_PATCHES, &args![patch_manager, world, class_registry])
        .u32()
}

/// The body of `detectFormat`, with its 0x154-byte frame at `frame` (offsets are from the
/// frame's start): the saved result address at `+0` (also the temporary `hkStringPtr`), the
/// `bool` of `isBinaryMagic` at `+7`, the 0x80-byte read buffer at `+8`, the 0x40-byte tail at
/// `+0x88` and an `hkStringBuf` (data, size, capacity and flags, then its storage) at `+0xc8`.
fn detect_format_in_frame(e: &mut Engine, stream: u32, result: u32, frame: u32) {
    let buffer = frame + 8;
    let tail = frame + 0x88;
    let string_buf = frame + 0xc8;
    e.mem.set_u32(result, 0);
    e.call(STRING_PTR_ASSIGN, &args![result + 4, 0u32]);
    e.mem.set_u32(frame, result + 8);
    e.call(MEM_UTIL_SET, &args![result + 8, 0u32, 4u32]);
    e.call(CRT_MEMSET, &args![buffer, 0u32, 0x80u32]);
    e.vcall(stream, 0x1c, &args![0x80u32]);
    let bytes_read = e.vcall(stream, 0x10, &args![buffer, 0x80u32]).i32();
    e.vcall(stream, 0x20, &args![]);
    e.call(MEMORY_SET, &args![tail, 0xffff_ffffu32, 0x40u32]);
    let first = e.mem.u32(buffer);
    e.mem.set_u32(tail, PACKFILE_MAGIC_FIRST);
    e.mem.set_u32(tail + 4, PACKFILE_MAGIC_SECOND);
    e.mem.set_u8(tail + 0x28, 0);
    if bytes_read >= 8 {
        let second = e.mem.u32(buffer + 4);
        let magic = hk_binary_tagfile_is_binary_magic(e, frame + 7, first, second);
        if e.mem.u8(magic) != 0 {
            e.mem.set_u32(result, FORMAT_BINARY_TAGFILE);
            return;
        }
    }
    if bytes_read >= 0x40
        && first == PACKFILE_MAGIC_FIRST
        && e.mem.u32(buffer + 4) == PACKFILE_MAGIC_SECOND
    {
        e.mem.set_u32(result, FORMAT_PACKFILE);
        e.call(STRING_PTR_ASSIGN, &args![result + 4, buffer + 0x28]);
        let destination = e.mem.u32(frame);
        e.call(MEM_UTIL_COPY, &args![destination, buffer + 0x10, 4u32]);
        return;
    }
    if e.call(STRING_STR, &args![buffer, TAG_PACKFILE_OPEN_SPACE])
        .u32()
        == 0
        && e.call(STRING_STR, &args![buffer, TAG_PACKFILE_OPEN_CLOSE])
            .u32()
            == 0
    {
        return;
    }
    e.mem.set_u32(result, FORMAT_XML_PACKFILE);
    let tag = e.call(STRING_STR, &args![buffer, TAG_PACKFILE_OPEN]).u32();
    e.call(STRING_BUF_CONSTRUCTOR, &args![string_buf, tag]);
    e.call(STRING_PTR_CONSTRUCTOR, &args![frame]);
    let text = e.mem.u32(string_buf);
    let found = find_text_value(e, text, KEY_CLASS_VERSION, frame);
    // The version string to assign to the result, or none when `contentsversion` set it.
    let assign = if found == 0 {
        Some(VERSION_3_0_0)
    } else {
        let class_version_text = e.mem.u32(frame) & 0xffff_fffe;
        let class_version = e
            .call(STRING_TO_INT, &args![class_version_text, 0u32])
            .i32();
        if class_version == 1 {
            Some(VERSION_3_0_0)
        } else if (class_version.wrapping_sub(2) as u32) <= 5 {
            let text = e.mem.u32(string_buf);
            if find_text_value(e, text, KEY_CONTENTS_VERSION, result + 4) != 0 {
                None
            } else {
                Some(VERSION_3_1_0)
            }
        } else {
            Some(VERSION_ERROR)
        }
    };
    if let Some(version) = assign {
        e.call(STRING_PTR_ASSIGN, &args![result + 4, version]);
    }
    e.call(STRING_PTR_DESTRUCTOR, &args![frame]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let flags = e.mem.u32(string_buf + 8);
    e.mem.set_u32(string_buf + 4, 0);
    if (flags as i32) >= 0 {
        let data = e.mem.u32(string_buf);
        release_storage(e, allocator, data, flags & 0x3fff_ffff);
    }
}

// Translated from 00d48af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::detectFormat` (engine map name), cdecl `(stream, result)`: reads the first
/// 0x80 bytes of the stream (mark `0x80` at slot `0x1c`, read at slot `0x10`, rewind at slot `0x20`)
/// and fills the 12-byte `result` (format word, `hkStringPtr` with the contents version, four
/// bytes of layout rules). A binary tagfile is format 4 (`isBinaryMagic` on the first two words,
/// with at least 8 bytes read); a packfile is format 2 (at least 0x40 bytes and the packfile
/// magic): the version string at `+0x28` of the header and the layout rules at `+0x10` are
/// copied; text containing `<hkpackfile` is format 3 whose version comes from its
/// `classversion` and `contentsversion` attributes (`Havok-3.0.0`, `Havok-3.1.0` or `error`).
/// Anything else leaves format 0. The read buffer is followed in the frame by 0x40 bytes
/// filled with `0xff` (then the two packfile magic words and a zero byte), which end the text.
pub fn hk_serialize_util_detect_format(e: &mut Engine, stream: u32, result: u32) {
    // The exe's frame (`SUB ESP,0x154`, four pushes) is 0x164 bytes; the locals start at +0x10
    // (the saved result address, 00d48b1b; the read buffer at +0x18, 00d48b29).
    e.with_stack(0x164, |e, frame| {
        detect_format_in_frame(e, stream, result, frame.addr() + 0x10)
    });
}

/// What `load` does once the stream is known to be good, with its locals: `info` (the 12-byte
/// format record), `world` (an `hkDataWorldDict`, 12 bytes) and `handle` (4 bytes).
fn load_in_frame(
    e: &mut Engine,
    stream: u32,
    error_details: u32,
    class_registry: u32,
    info: u32,
    world: u32,
    handle: u32,
) -> u32 {
    e.call(STRING_PTR_CONSTRUCTOR, &args![info + 4]);
    hk_serialize_util_detect_format(e, stream, info);
    let format = e.mem.i32(info);
    if format >= FORMAT_PACKFILE as i32 {
        if format <= FORMAT_XML_PACKFILE as i32 {
            let singleton = e.mem.u32(REGISTRIES_SINGLETON);
            let type_info_registry = e.vcall(singleton, 0xc, &args![]).u32();
            let result = load_packfile(e, error_details, info, stream, type_info_registry);
            e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
            return result;
        }
        if format == FORMAT_BINARY_TAGFILE as i32 {
            e.call(DICT_CONSTRUCTOR, &args![world]);
            let failed =
                load_binary_tagfile_into_world(e, world, error_details, class_registry, stream);
            if failed != 0 {
                e.call(DICT_DESTRUCTOR, &args![world]);
                e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
                return 0;
            }
            let contents = e.call(DICT_GET_CONTENTS, &args![world, handle]).u32();
            let result = e
                .call(TO_RESOURCE_WITH_REGISTRY, &args![contents, class_registry])
                .u32();
            e.call(DATA_OBJECT_DESTRUCTOR, &args![handle]);
            e.call(DICT_DESTRUCTOR, &args![world]);
            e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
            return result;
        }
    }
    report_error(e, error_details, 2, MESSAGE_UNABLE_TO_DETECT);
    e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
    0
}

// Translated from 00d48d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::load` (engine map name), cdecl, three words `(stream, error_details,
/// class_registry)`; the third is not in the decompiler's signature. A null stream reports error 1
/// ("Stream pointer is null") and a stream whose slot `0xc` reports not-ok error 1 ("Stream is
/// not ok"), both answering 0. A null `class_registry` becomes the registries singleton's class
/// registry (slot `0x10`). The format is detected: packfiles (2 and 3) are loaded by
/// `loadPackfile` with the singleton's type-info registry (slot `0xc`); a binary tagfile (4)
/// is read into a new `hkDataWorldDict` (`loadBinaryTagfileIntoWorld`) and its contents made
/// into a resource (`toResourceWithRegistry`); any other format reports error 2 ("Unable to
/// detect format from stream"). Answers the loaded resource, or 0.
pub fn hk_serialize_util_load(
    e: &mut Engine,
    stream: u32,
    error_details: u32,
    class_registry: u32,
) -> u32 {
    if stream == 0 {
        report_error(e, error_details, 1, MESSAGE_STREAM_NULL);
        return 0;
    }
    // The exe's frame (`SUB ESP,0x18`, three pushes) taken with the arguments: the format
    // record at +0xc (00d48db4) and the dictionary world at +0x18 (00d48dde); the return address
    // at +0x24. The stream's own argument slot (+0x28) is both the word the "is ok" test gets
    // (00d48d63) and the contents handle (00d48dfe).
    e.with_stack(0x34, |e, frame| {
        let frame = frame.addr();
        let slot = frame + 0x28;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(frame + 0x2c, error_details);
        e.mem.set_u32(frame + 0x30, class_registry);
        let ok_at = e.vcall(stream, 0xc, &args![slot]).u32();
        if e.mem.u8(ok_at) == 0 {
            report_error(e, error_details, 1, MESSAGE_STREAM_NOT_OK);
            return 0;
        }
        let mut class_registry = class_registry;
        if class_registry == 0 {
            let singleton = e.mem.u32(REGISTRIES_SINGLETON);
            class_registry = e.vcall(singleton, 0x10, &args![]).u32();
        }
        load_in_frame(
            e,
            stream,
            error_details,
            class_registry,
            frame + 0xc,
            frame + 0x18,
            slot,
        )
    })
}

// Translated from 00d48f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::load_ov2` (engine map name), cdecl, four words `(buffer, size,
/// error_details, class_registry)`. A buffer that starts with the two packfile magic words,
/// has this platform's layout rules at `+0x10` and this version string at `+0x28` is loaded
/// directly (`hkNativePackfileUtils::load_ov2(buffer, size, 0)`); any other buffer is wrapped in
/// an `hkIstream` (on the stack) and loaded as a stream (`load`), the istream destroyed
/// afterwards. (The 0x40-byte local that the code fills with `0xff` and the two magic words is
/// never read, but its `memSet` is a call.)
pub fn hk_serialize_util_load_ov2(
    e: &mut Engine,
    buffer: u32,
    size: u32,
    error_details: u32,
    class_registry: u32,
) -> u32 {
    // The exe's frame (`SUB ESP,0x4c`, one push): 0x50 bytes, the stream object at +0x4
    // (00d48f83) and the 0x40-byte magic block at +0x10 (00d48f06), its magic words at +0x10 and
    // +0x14 and the zero byte at +0x38 (00d48f24..00d48f30).
    e.with_stack(0x50, |e, frame| {
        load_buffer_in_frame(e, frame.addr(), buffer, size, error_details, class_registry)
    })
}

/// [`hk_serialize_util_load_ov2`] in its frame.
fn load_buffer_in_frame(
    e: &mut Engine,
    frame: u32,
    buffer: u32,
    size: u32,
    error_details: u32,
    class_registry: u32,
) -> u32 {
    let local = frame + 0x10;
    e.call(MEMORY_SET, &args![local, 0xffff_ffffu32, 0x40u32]);
    e.mem.set_u32(local, PACKFILE_MAGIC_FIRST);
    e.mem.set_u32(local + 4, PACKFILE_MAGIC_SECOND);
    e.mem.set_u8(local + 0x28, 0);
    if e.mem.u32(buffer) == PACKFILE_MAGIC_FIRST
        && e.mem.u32(buffer + 4) == PACKFILE_MAGIC_SECOND
        && e.call(
            MEMORY_COMPARE,
            &args![buffer + 0x10, PLATFORM_LAYOUT_RULES, 4u32],
        )
        .u32()
            == 0
    {
        let version = e.call(CURRENT_VERSION, &args![]).u32();
        if e.call(STRING_COMPARE, &args![buffer + 0x28, version]).u32() == 0 {
            return e
                .call(NATIVE_PACKFILE_LOAD, &args![buffer, size, 0u32])
                .u32();
        }
    }
    let istream = frame + 4;
    let made = e
        .call(ISTREAM_CONSTRUCTOR, &args![istream, buffer, size])
        .u32();
    let reader = e.mem.u32(made + 8);
    let result = hk_serialize_util_load(e, reader, error_details, class_registry);
    e.call(ISTREAM_DESTRUCTOR, &args![istream]);
    result
}

/// What `loadOnHeap` does once the stream is known to be good and both registries are known,
/// with its locals `info` (12 bytes), `world` (an `hkDataWorldDict`, 12 bytes) and `handle`
/// (4 bytes).
#[allow(clippy::too_many_arguments)]
fn load_on_heap_in_frame(
    e: &mut Engine,
    stream: u32,
    error_details: u32,
    class_registry: u32,
    type_info_registry: u32,
    info: u32,
    world: u32,
    handle: u32,
    stream_slot: u32,
) -> u32 {
    e.call(STRING_PTR_CONSTRUCTOR, &args![info + 4]);
    hk_serialize_util_detect_format(e, stream, info);
    let format = e.mem.u32(info);
    match format {
        FORMAT_PACKFILE => {
            let equal_at = hk_structure_layout_layout_rules_operator(
                e,
                info + 8,
                stream_slot,
                PLATFORM_LAYOUT_RULES,
            );
            let equal = e.mem.u8(equal_at) != 0;
            if equal && hk_string_ptr_operator_ov3(e, Ptr::new(info + 4), CURRENT_CONTENTS_VERSION)
            {
                let result = load_new_packfile_on_heap(
                    e,
                    class_registry,
                    stream,
                    info,
                    error_details,
                    type_info_registry,
                );
                e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
                return result;
            }
        }
        FORMAT_XML_PACKFILE => {}
        FORMAT_BINARY_TAGFILE => {
            e.call(DICT_CONSTRUCTOR, &args![world]);
            let failed =
                load_binary_tagfile_into_world(e, world, error_details, class_registry, stream);
            if failed != 0 {
                e.call(DICT_DESTRUCTOR, &args![world]);
                e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
                return 0;
            }
            let contents = e.call(DICT_GET_CONTENTS, &args![world, handle]).u32();
            let result = e
                .call(
                    TO_OBJECT_WITH_REGISTRY,
                    &args![contents, class_registry, type_info_registry],
                )
                .u32();
            e.call(DATA_OBJECT_DESTRUCTOR, &args![handle]);
            e.call(DICT_DESTRUCTOR, &args![world]);
            e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
            return result;
        }
        _ => {
            report_error(e, error_details, 2, MESSAGE_UNABLE_TO_DETECT);
            e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
            return 0;
        }
    }
    let singleton = e.mem.u32(DEPRECATED_LOADER_SINGLETON);
    let result = e
        .vcall(singleton, 0x18, &args![stream, info, error_details])
        .u32();
    e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
    result
}

// Translated from 00d48fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::loadOnHeap` (engine map name), cdecl, four words `(stream, error_details,
/// class_registry, type_info_registry)`; the last two are not in the decompiler's signature.
/// Checks the stream as `load` does (errors 1), takes the registries singleton's class
/// registry (slot `0x10`) and type-info registry (slot `0xc`) for null arguments, detects the
/// format and: for format 2 with this platform's layout rules and contents version `Havok-7.1.0-r1`
/// answers `loadNewPackfileOnHeap`; for any other packfile (format 3 or a mismatch) forwards to
/// slot `0x18` of the deprecated-format loader singleton with `(stream, info, error_details)`;
/// for format 4 reads the tagfile into a `hkDataWorldDict` and converts its contents
/// (`toObjectWithRegistry`); any other format reports error 2. Answers the loaded object, or 0.
pub fn hk_serialize_util_load_on_heap(
    e: &mut Engine,
    stream: u32,
    error_details: u32,
    class_registry: u32,
    type_info_registry: u32,
) -> u32 {
    if stream == 0 {
        report_error(e, error_details, 1, MESSAGE_STREAM_NULL);
        return 0;
    }
    // The exe's frame (`SUB ESP,0x1c`, four pushes) taken with the arguments: the contents
    // handle at +0x10 (00d490d4), the format record at +0x14 (00d4905c) and the dictionary world
    // at +0x20 (00d490b3); the return address at +0x2c. The stream's own argument slot (+0x30) is
    // the word the "is ok" test gets (00d48ff3) and the layout-rules comparison's (00d4912f).
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        let slot = frame + 0x30;
        e.mem.set_u32(slot, stream);
        e.mem.set_u32(frame + 0x34, error_details);
        e.mem.set_u32(frame + 0x38, class_registry);
        e.mem.set_u32(frame + 0x3c, type_info_registry);
        let ok_at = e.vcall(stream, 0xc, &args![slot]).u32();
        if e.mem.u8(ok_at) == 0 {
            report_error(e, error_details, 1, MESSAGE_STREAM_NOT_OK);
            return 0;
        }
        let mut class_registry = class_registry;
        if class_registry == 0 {
            let singleton = e.mem.u32(REGISTRIES_SINGLETON);
            class_registry = e.vcall(singleton, 0x10, &args![]).u32();
        }
        let mut type_info_registry = type_info_registry;
        if type_info_registry == 0 {
            let singleton = e.mem.u32(REGISTRIES_SINGLETON);
            type_info_registry = e.vcall(singleton, 0xc, &args![]).u32();
        }
        load_on_heap_in_frame(
            e,
            stream,
            error_details,
            class_registry,
            type_info_registry,
            frame + 0x14,
            frame + 0x20,
            frame + 0x10,
            slot,
        )
    })
}

// Translated from 00d49240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::isLoadable` (engine map name), cdecl, one word `stream`. Answers 0 for a null
/// stream or one whose slot `0xc` reports not-ok. A tagfile format (`hkTagfileReader::detectFormat`
/// above 1) answers 1. Otherwise detects the packfile format: a native packfile (2) of this platform
/// and contents version answers 1; format 3, or a format 2 with the right platform but another
/// version, asks slot `0x10` of the deprecated-format loader singleton about the format record
/// and answers 1 if it does; everything else answers 0.
pub fn hk_serialize_util_is_loadable(e: &mut Engine, stream: u32) -> u32 {
    if stream == 0 {
        return 0;
    }
    // The exe's frame (`SUB ESP,0xc`, one push) taken with the argument: the format record at
    // +0x4 (00d49283), the return address at +0x10 and the stream's own slot at +0x14, which
    // the "is ok" test (00d49255) and the tagfile format (00d49267) both use.
    e.with_stack(0x18, |e, frame| {
        is_loadable_in_frame(e, stream, frame.addr())
    })
}

/// [`hk_serialize_util_is_loadable`] in its frame.
fn is_loadable_in_frame(e: &mut Engine, stream: u32, frame: u32) -> u32 {
    let slot = frame + 0x14;
    e.mem.set_u32(slot, stream);
    let ok_at = e.vcall(stream, 0xc, &args![slot]).u32();
    let ok = e.mem.u8(ok_at) != 0;
    if !ok {
        return 0;
    }
    let format_at = e
        .call(TAGFILE_READER_DETECT_FORMAT, &args![slot, stream])
        .u32();
    let tagfile_format = e.mem.i32(format_at);
    if tagfile_format > 1 {
        return 1;
    }
    let info = frame + 4;
    e.call(STRING_PTR_CONSTRUCTOR, &args![info + 4]);
    hk_serialize_util_detect_format(e, stream, info);
    let format = e.mem.u32(info);
    let mut ask_deprecated = false;
    let mut loadable = false;
    if format == FORMAT_PACKFILE
        && e.call(
            MEMORY_COMPARE,
            &args![info + 8, PLATFORM_LAYOUT_RULES, 4u32],
        )
        .u32()
            == 0
    {
        let version = e.call(CURRENT_VERSION, &args![]).u32();
        if hk_string_ptr_operator_ov3(e, Ptr::new(info + 4), version) {
            loadable = true;
        } else {
            ask_deprecated = true;
        }
    } else if format == FORMAT_XML_PACKFILE {
        ask_deprecated = true;
    }
    if ask_deprecated {
        let singleton = e.mem.u32(DEPRECATED_LOADER_SINGLETON);
        loadable = e.vcall(singleton, 0x10, &args![info]).u32() != 0;
    }
    e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
    u32::from(loadable)
}

// Translated from 00d49310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSerializeUtil::detectFormat_ov2` (engine map name), cdecl, `(out, stream)`: detects the
/// format of `stream` into a local record and stores just its format word at `out`. Returns
/// `out`.
pub fn hk_serialize_util_detect_format_ov2(e: &mut Engine, out: u32, stream: u32) -> u32 {
    // The exe's frame (`SUB ESP,0xc`, one push): 0x10 bytes, the format record at +0x4
    // (00d49321).
    e.with_stack(0x10, |e, frame| {
        let info = frame.addr() + 4;
        e.call(STRING_PTR_CONSTRUCTOR, &args![info + 4]);
        hk_serialize_util_detect_format(e, stream, info);
        let format = e.mem.u32(info);
        e.mem.set_u32(out, format);
        e.call(STRING_PTR_DESTRUCTOR, &args![info + 4]);
    });
    out
}

// Translated from 00d49350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStringPtr::operator__ov3` (engine map name; the `operator` of an `hkStringPtr` and a C
/// string, its symbol characters dropped by the map), `thiscall`, `RET 4`: whether the string
/// pointer's text (its word with the flag bit cleared) equals `text`. Two null strings are equal;
/// a null and a non-null are not; otherwise `hkString::strCmp` decides.
pub fn hk_string_ptr_operator_ov3(e: &mut Engine, this: Ptr, text: u32) -> bool {
    let own = e.mem.u32(this.addr()) & 0xffff_fffe;
    if own == 0 {
        return text == 0;
    }
    if text == 0 {
        return false;
    }
    e.call(STRING_COMPARE, &args![own, text]).u32() == 0
}

// Translated from 00d493a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkStructureLayout::LayoutRules::operator_` (engine map name; the equality test, its symbol
/// characters dropped by the map), `thiscall`, `RET 8`: writes 1 at the byte `out` when the four
/// layout-rule bytes at `this` equal the four at `other`, else 0. Returns `out`.
pub fn hk_structure_layout_layout_rules_operator(
    e: &mut Engine,
    this: u32,
    out: u32,
    other: u32,
) -> u32 {
    let equal = (0..4).all(|i| e.mem.u8(this + i) == e.mem.u8(other + i));
    e.mem.set_u8(out, u8::from(equal));
    out
}

// Translated from 00d493e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::ForwardingPackfileListerer::wrapObject` (engine map name), `thiscall`,
/// `RET 0xc`: builds the `hkDataObject` handle `out` for an object of `class`: for a non-null
/// `object` wraps it in the listener's world (`hkDataWorldNative::wrapObject`, `world` at `+0xc`)
/// and constructs the handle from the result; for a null one constructs the null handle. Returns
/// `out`.
pub fn forwarding_packfile_listerer_wrap_object(
    e: &mut Engine,
    this: Ptr<ForwardingPackfileListerer>,
    out: u32,
    object: u32,
    class: u32,
) -> u32 {
    if object != 0 {
        let world = e.get(this, ForwardingPackfileListerer::world).addr();
        let wrapped = e
            .call(WORLD_WRAP_OBJECT, &args![world, object, class])
            .u32();
        e.call(DATA_OBJECT_CONSTRUCTOR, &args![out, wrapped]);
    } else {
        e.call(DATA_OBJECT_CONSTRUCTOR, &args![out, 0u32]);
    }
    out
}

// Translated from 00d49420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_anonymous_namespace_::ForwardingPackfileListerer::addDataObjectCallback` (engine map name),
/// `thiscall`, `RET 8`: `out` receives the `hkDataObject` handle for `input`. When the listener
/// has a listener to forward to (`+8`) and `input` is not null, it looks up the class of the
/// object in the class registry (registries singleton, slot `0x10`, then its slot `0x10` with
/// the class name; the handle's own class replaces it when it has one) and asks the forwarded
/// listener (slot `0xc`) with the `(object, class)` pair as two pointers into a stack pair. If the
/// listener changed either, `out` is built from the pair by `wrapObject` (`00d493e0`);
/// otherwise (or with nothing to forward to) `out` is a copy of `input`. Returns `out`.
pub fn forwarding_packfile_listerer_add_data_object_callback(
    e: &mut Engine,
    this: Ptr<ForwardingPackfileListerer>,
    out: u32,
    input: u32,
) -> u32 {
    let forwarded = e
        .get(this, ForwardingPackfileListerer::forwarded_listener)
        .addr();
    if forwarded != 0 && e.call(DATA_OBJECT_IS_NULL, &args![input]).u32() == 0 {
        // The exe's frame (`SUB ESP,0x10`, four pushes) taken with the arguments: the pair at
        // +0x10 (00d4948c / 00d49478), the return address at +0x20; the class goes into the
        // `input` argument's own slot, +0x28 (00d49446; `input` itself is kept in `EBX`).
        let replaced = e.with_stack(0x2c, |e, frame| {
            let frame = frame.addr();
            e.mem.set_u32(frame + 0x24, out);
            e.mem.set_u32(frame + 0x28, input);
            forward_data_object_in_frame(e, this, out, input, forwarded, frame)
        });
        if replaced {
            return out;
        }
    }
    e.call(DATA_OBJECT_COPY_CONSTRUCTOR, &args![out, input]);
    out
}

/// The forwarding part of [`forwarding_packfile_listerer_add_data_object_callback`] in its
/// frame; true when the listener replaced the object (then `out` holds the new one).
fn forward_data_object_in_frame(
    e: &mut Engine,
    this: Ptr<ForwardingPackfileListerer>,
    out: u32,
    input: u32,
    forwarded: u32,
    frame: u32,
) -> bool {
    {
        let class_slot = frame + 0x28;
        e.call(DATA_OBJECT_GET_CLASS, &args![input, class_slot]);
        let singleton = e.mem.u32(REGISTRIES_SINGLETON);
        let registry = e.vcall(singleton, 0x10, &args![]).u32();
        let lookup_slot = e.mem.u32(registry) + 0x10;
        let name = e.call(DATA_CLASS_GET_NAME, &args![class_slot]).u32();
        let lookup = e.mem.u32(lookup_slot);
        let mut class = e.call(lookup, &args![registry, name]).u32();
        // `pair`: the object and class cells handed to the listener (+0, +4) and the pair
        // `getHandle` fills (+8, +0xc).
        let pair = frame + 0x10;
        e.call(DATA_OBJECT_GET_HANDLE, &args![input, pair + 8]);
        let handle_class = e.mem.u32(pair + 0xc);
        let handle_object = e.mem.u32(pair + 8);
        e.mem.set_u32(pair, handle_object);
        if handle_class != 0 {
            class = handle_class;
        }
        e.mem.set_u32(pair + 4, class);
        e.vcall(forwarded, 0xc, &args![pair, pair + 4]);
        let object_cell = e.mem.u32(pair);
        let class_cell = e.mem.u32(pair + 4);
        let changed = object_cell != e.mem.u32(pair + 8) || class_cell != class;
        if changed {
            forwarding_packfile_listerer_wrap_object(e, this, out, object_cell, class_cell);
            e.call(DATA_CLASS_DESTRUCTOR, &args![class_slot]);
            return true;
        }
        e.call(DATA_CLASS_DESTRUCTOR, &args![class_slot]);
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d483a0, hk_binary_tagfile_is_binary_magic(u32, u32, u32) -> u32),
        entry!(0x00d48420, fn_00d48420(u32) -> u32),
        entry!(0x00d48440, find_text_value(u32, u32, u32) -> u32),
        entry!(0x00d48560, fn_00d48560(u32, u32, u32) -> u32),
        entry!(0x00d485a0, hk_serialize_util_save_tagfile(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d48680, load_stream_into_buffer(u32, u32)),
        entry!(0x00d48760, load_packfile(u32, u32, u32, u32) -> u32),
        entry!(0x00d48840, load_new_packfile_on_heap(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00d48950, check_tagfile_world(u32, u32, u32, u32) -> u32),
        entry!(0x00d48a90, load_binary_tagfile_into_world(u32, u32, u32, u32) -> u32),
        entry!(0x00d48af0, hk_serialize_util_detect_format(u32, u32)),
        entry!(0x00d48d30, hk_serialize_util_load(u32, u32, u32) -> u32),
        entry!(0x00d48f00, hk_serialize_util_load_ov2(u32, u32, u32, u32) -> u32),
        entry!(0x00d48fc0, hk_serialize_util_load_on_heap(u32, u32, u32, u32) -> u32),
        entry!(0x00d49240, hk_serialize_util_is_loadable(u32) -> u32),
        entry!(0x00d49310, hk_serialize_util_detect_format_ov2(u32, u32) -> u32),
        entry!(0x00d49350, hk_string_ptr_operator_ov3(Ptr, u32) -> bool),
        entry!(0x00d493a0, hk_structure_layout_layout_rules_operator(u32, u32, u32) -> u32),
        entry!(
            0x00d493e0,
            forwarding_packfile_listerer_wrap_object(
                Ptr<ForwardingPackfileListerer>,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00d49420,
            forwarding_packfile_listerer_add_data_object_callback(
                Ptr<ForwardingPackfileListerer>,
                u32,
                u32,
            ) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Vec<(u32, Vec<u32>)>;
    type Frees = Rc<RefCell<Vec<[u32; 2]>>>;

    /// Fake code addresses for the doubles of the objects the tests build.
    const STREAM_SLOTS: u32 = 0x0a00_0000;
    const REGISTRIES_SLOTS: u32 = 0x0a20_0000;
    const DEPRECATED_SLOTS: u32 = 0x0a30_0000;
    const CLASS_REGISTRY_LOOKUP: u32 = 0x0a40_0010;
    const SAVE_DOUBLE: u32 = 0x0a50_0000;
    const ALLOCATOR_FREE: u32 = 0x0a60_0010;
    const LISTENER_CALLBACK: u32 = 0x0a70_000c;
    const RESOURCE_SLOTS: u32 = 0x0a80_0000;
    const WORLD_SLOTS: u32 = 0x0a90_0000;
    const CLASS_OBJECT_SLOTS: u32 = 0x0aa0_0000;

    /// Every function the unit calls that is not in it, as a do-nothing double.
    const EXTERNAL: &[u32] = &[
        MEMORY_ROUTER,
        STRING_PTR_CONSTRUCTOR,
        STRING_PTR_DESTRUCTOR,
        STRING_PTR_ASSIGN,
        STRING_PTR_SET,
        STRING_PTR_PRINTF,
        STRING_STR,
        STRING_LEN,
        STRING_CHR,
        STRING_COMPARE,
        MEMORY_COMPARE,
        MEMORY_SET,
        STRING_TO_INT,
        MEM_UTIL_SET,
        MEM_UTIL_COPY,
        CRT_MEMSET,
        STRING_BUF_CONSTRUCTOR,
        ARRAY_RESERVE,
        REMOVE_REFERENCE,
        STRING_MAP_CONSTRUCTOR,
        STRING_MAP_DESTRUCTOR,
        TYPE_INFO_REGISTRY_MERGE,
        CURRENT_VERSION,
        NATIVE_PACKFILE_LOAD,
        TAGFILE_READER_CONSTRUCTOR,
        TAGFILE_READER_LOAD,
        TAGFILE_READER_DETECT_FORMAT,
        WORLD_CONSTRUCTOR,
        WORLD_DESTRUCTOR,
        WORLD_SET_CONTENTS,
        WORLD_GET_CONTENTS,
        WORLD_SET_CLASS_REGISTRY,
        WORLD_WRAP_OBJECT,
        DICT_CONSTRUCTOR,
        DICT_DESTRUCTOR,
        DICT_GET_CONTENTS,
        DATA_OBJECT_CONSTRUCTOR,
        DATA_OBJECT_COPY_CONSTRUCTOR,
        DATA_OBJECT_DESTRUCTOR,
        DATA_OBJECT_IS_NULL,
        DATA_OBJECT_GET_CLASS,
        DATA_OBJECT_GET_HANDLE,
        DATA_CLASS_DESTRUCTOR,
        DATA_CLASS_GET_NAME,
        TO_OBJECT_WITH_REGISTRY,
        TO_RESOURCE_WITH_REGISTRY,
        CLASS_GET_DESCRIBED_VERSION,
        RECOMPUTE_PATCH_DEPENDENCIES,
        PATCH_MANAGER_GET_UID,
        APPLY_PATCHES,
        MAP_GET_WITH_DEFAULT,
        ISTREAM_CONSTRUCTOR,
        ISTREAM_DESTRUCTOR,
    ];

    /// The strings the code finds in the exe's data.
    const STRINGS: &[(u32, &str)] = &[
        (VERSION_3_0_0, "Havok-3.0.0"),
        (VERSION_3_1_0, "Havok-3.1.0"),
        (VERSION_ERROR, "error"),
        (KEY_CLASS_VERSION, "classversion"),
        (KEY_CONTENTS_VERSION, "contentsversion"),
        (TAG_PACKFILE_OPEN, "<hkpackfile"),
        (TAG_PACKFILE_OPEN_SPACE, "<hkpackfile "),
        (TAG_PACKFILE_OPEN_CLOSE, "<hkpackfile>"),
        (CURRENT_CONTENTS_VERSION, "Havok-7.1.0-r1"),
        (
            MESSAGE_UNABLE_TO_DETECT,
            "Unable to detect format from stream",
        ),
        (MESSAGE_STREAM_NOT_OK, "Stream is not ok"),
        (MESSAGE_STREAM_NULL, "Stream pointer is null"),
        (MESSAGE_WRONG_PLATFORM, "Wrong platform for packfile"),
    ];

    fn zero(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn str_str(e: &mut Engine, a: &[u32]) -> Ret {
        let haystack = e.mem.cstr(a[0]);
        let needle = e.mem.cstr(a[1]);
        let at = haystack
            .windows(needle.len())
            .position(|window| window == needle.as_slice());
        answer(at.map_or(0, |i| a[0] + i as u32))
    }

    fn str_len(e: &mut Engine, a: &[u32]) -> Ret {
        answer(e.mem.cstr(a[0]).len() as u32)
    }

    fn str_chr(e: &mut Engine, a: &[u32]) -> Ret {
        let text = e.mem.cstr(a[0]);
        let at = text.iter().position(|c| u32::from(*c) == a[1]);
        answer(at.map_or(0, |i| a[0] + i as u32))
    }

    fn str_cmp(e: &mut Engine, a: &[u32]) -> Ret {
        answer(u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])))
    }

    fn mem_cmp(e: &mut Engine, a: &[u32]) -> Ret {
        let (x, y) = (e.mem.bytes(a[0], a[2]), e.mem.bytes(a[1], a[2]));
        answer(u32::from(x != y))
    }

    fn mem_set(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
        answer(a[0])
    }

    fn mem_copy(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0], &bytes);
        answer(a[0])
    }

    fn to_int(e: &mut Engine, a: &[u32]) -> Ret {
        let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
        answer(text.trim().parse::<i32>().map_or(a[1], |n| n as u32))
    }

    fn store_first_word(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], a[1]);
        answer(a[0])
    }

    fn clear_word(e: &mut Engine, a: &[u32]) -> Ret {
        e.mem.set_u32(a[0], 0);
        answer(a[0])
    }

    /// Sets a string pointer to a fresh copy of the text `set` is given (the owned flag set, as
    /// the real `hkStringPtr` does).
    fn string_ptr_set(e: &mut Engine, a: &[u32]) -> Ret {
        let text = e.mem.bytes(a[1], a[2]);
        let copy = e.mem.alloc(a[2] + 1);
        e.mem.set_cstr(copy, &text);
        e.mem.set_u32(a[0], copy | 1);
        Ret::default()
    }

    /// `hkStringPtr::operator=`: a copy of the string with the owned flag, or null.
    fn string_ptr_assign(e: &mut Engine, a: &[u32]) -> Ret {
        if a[1] == 0 {
            e.mem.set_u32(a[0], 0);
        } else {
            let text = e.mem.cstr(a[1]);
            let copy = e.mem.alloc(text.len() as u32 + 1);
            e.mem.set_cstr(copy, &text);
            e.mem.set_u32(a[0], copy | 1);
        }
        Ret::default()
    }

    /// The text of a string pointer word (the owned flag cleared).
    fn string_of(e: &Engine, word: u32) -> Vec<u8> {
        e.mem.cstr(word & 0xffff_fffe)
    }

    /// An engine with the exe's string data in place, every external function a do-nothing
    /// double and the C string functions real.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_f000, 0x1000);
        e.map(0x011b_7000, 0x1000);
        e.map(0x0126_8000, 0x2000);
        for address in EXTERNAL {
            e.register(*address, zero);
        }
        e.register(STRING_STR, str_str);
        e.register(STRING_LEN, str_len);
        e.register(STRING_CHR, str_chr);
        e.register(STRING_COMPARE, str_cmp);
        e.register(MEMORY_COMPARE, mem_cmp);
        e.register(MEMORY_SET, mem_set);
        e.register(MEM_UTIL_SET, mem_set);
        e.register(CRT_MEMSET, mem_set);
        e.register(MEM_UTIL_COPY, mem_copy);
        e.register(STRING_TO_INT, to_int);
        e.register(STRING_PTR_CONSTRUCTOR, clear_word);
        e.register(STRING_PTR_ASSIGN, string_ptr_assign);
        e.register(STRING_PTR_SET, string_ptr_set);
        e.register(STRING_BUF_CONSTRUCTOR, store_first_word);
        for (address, text) in STRINGS {
            e.mem.set_cstr(*address, text.as_bytes());
        }
        e.mem.write(PLATFORM_LAYOUT_RULES, &[1, 2, 3, 4]);
        e
    }

    fn text(e: &mut Engine, s: &str) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s.as_bytes());
        at
    }

    /// An object whose vtable holds `(slot offset, target)`.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x40);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    struct StreamState {
        data: Vec<u8>,
        position: usize,
        ok: bool,
        seekable: bool,
    }

    /// A stream object over `data`: slots `0xc` (ok), `0x10` (read), `0x1c` (mark), `0x20`
    /// (rewind to the start), `0x24` (seekable), `0x28` (seek), `0x2c` (tell).
    fn stream(e: &mut Engine, data: &[u8], ok: bool, seekable: bool) -> u32 {
        let state = Rc::new(RefCell::new(StreamState {
            data: data.to_vec(),
            position: 0,
            ok,
            seekable,
        }));
        let slots: [u32; 7] = [0xc, 0x10, 0x1c, 0x20, 0x24, 0x28, 0x2c];
        let table: Vec<(u32, u32)> = slots.iter().map(|s| (*s, STREAM_SLOTS + s)).collect();
        let object = object_with_slots(e, &table);
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0xc, move |e, a| {
            e.mem.set_u8(a[1], u8::from(s.borrow().ok));
            answer(a[1])
        });
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0x10, move |e, a| {
            let mut st = s.borrow_mut();
            let count = (a[2] as usize).min(st.data.len() - st.position);
            let chunk = st.data[st.position..st.position + count].to_vec();
            e.mem.write(a[1], &chunk);
            st.position += count;
            answer(count as u32)
        });
        e.register(STREAM_SLOTS + 0x1c, zero);
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0x20, move |_, _| {
            s.borrow_mut().position = 0;
            Ret::default()
        });
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0x24, move |e, a| {
            e.mem.set_u8(a[1], u8::from(s.borrow().seekable));
            answer(a[1])
        });
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0x28, move |_, a| {
            let mut st = s.borrow_mut();
            st.position = if a[2] == 2 {
                st.data.len()
            } else {
                a[1] as usize
            };
            Ret::default()
        });
        let s = Rc::clone(&state);
        e.register_double(STREAM_SLOTS + 0x2c, move |_, _| {
            answer(s.borrow().position as u32)
        });
        object
    }

    /// The registries singleton (slot `0xc` the type-info registry `0x7001`, slot `0x10` the
    /// class registry object), the deprecated-format loader singleton (slots `0x10`, `0x14`,
    /// `0x18` answering `0xd010`, `0xd014`, `0xd018`) and the patch manager (`0x7002`).
    /// Returns the class registry object.
    fn install_singletons(e: &mut Engine) -> u32 {
        let class_registry = object_with_slots(e, &[(0x10, CLASS_REGISTRY_LOOKUP)]);
        e.register_double(CLASS_REGISTRY_LOOKUP, |_, _| answer(0));
        e.register(REGISTRIES_SLOTS + 0xc, |_, _| answer(0x7001));
        e.register_double(REGISTRIES_SLOTS + 0x10, move |_, _| answer(class_registry));
        let registries = object_with_slots(
            e,
            &[
                (0xc, REGISTRIES_SLOTS + 0xc),
                (0x10, REGISTRIES_SLOTS + 0x10),
            ],
        );
        e.mem.set_u32(REGISTRIES_SINGLETON, registries);
        for slot in [0x10, 0x14, 0x18] {
            e.register_double(DEPRECATED_SLOTS + slot, move |_, _| answer(0xd000 + slot));
        }
        let deprecated = object_with_slots(
            e,
            &[
                (0x10, DEPRECATED_SLOTS + 0x10),
                (0x14, DEPRECATED_SLOTS + 0x14),
                (0x18, DEPRECATED_SLOTS + 0x18),
            ],
        );
        e.mem.set_u32(DEPRECATED_LOADER_SINGLETON, deprecated);
        e.mem.set_u32(PATCH_MANAGER_SINGLETON, 0x7002);
        class_registry
    }

    /// A memory router whose allocator (at `+0xc` and `+0x10`) logs what is freed through slot
    /// `0x10`.
    fn install_router(e: &mut Engine) -> Frees {
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&frees);
        e.register_double(ALLOCATOR_FREE, move |_, a| {
            log.borrow_mut().push([a[1], a[2]]);
            Ret::default()
        });
        let allocator = object_with_slots(e, &[(0x10, ALLOCATOR_FREE)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, allocator);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| answer(router));
        frees
    }

    fn calls_to(e: &mut Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn all_calls(e: &mut Engine) -> CallLog {
        e.call_log.take().unwrap()
    }

    #[test]
    fn is_binary_magic_accepts_native_and_swapped_words_only() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        let cases = [
            (TAGFILE_MAGIC_FIRST, TAGFILE_MAGIC_SECOND, 1),
            (
                TAGFILE_MAGIC_FIRST.swap_bytes(),
                TAGFILE_MAGIC_SECOND.swap_bytes(),
                1,
            ),
            (TAGFILE_MAGIC_FIRST, TAGFILE_MAGIC_SECOND.swap_bytes(), 0),
            (TAGFILE_MAGIC_FIRST.swap_bytes(), TAGFILE_MAGIC_SECOND, 0),
            (TAGFILE_MAGIC_FIRST, 0, 0),
            (0, 0, 0),
        ];
        for (first, second, expected) in cases {
            e.mem.set_u8(out, 0xaa);
            let got = e.call(0x00d4_83a0, &args![out, first, second]).u32();
            assert_eq!(got, out);
            assert_eq!(e.mem.u8(out), expected);
        }
    }

    #[test]
    fn fn_00d48420_skips_only_spaces() {
        let mut e = engine();
        let spaced = text(&mut e, "   x  y");
        assert_eq!(e.call(0x00d4_8420, &args![spaced]).u32(), spaced + 3);
        let plain = text(&mut e, "abc");
        assert_eq!(e.call(0x00d4_8420, &args![plain]).u32(), plain);
        let tabbed = text(&mut e, " \tz");
        assert_eq!(e.call(0x00d4_8420, &args![tabbed]).u32(), tabbed + 1);
        let blank = text(&mut e, "  ");
        assert_eq!(e.call(0x00d4_8420, &args![blank]).u32(), blank + 2);
    }

    #[test]
    fn find_text_value_extracts_the_quoted_value() {
        let mut e = engine();
        let needle = text(&mut e, "key");
        let destination = e.mem.alloc(4);
        let ok = text(&mut e, "<a key  =  \"value\" other=\"x\">");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d4_8440, &args![ok, needle, destination]).u32(),
            1
        );
        let set = calls_to(&mut e, STRING_PTR_SET);
        assert_eq!(set.len(), 1);
        assert_eq!(string_of(&e, e.mem.u32(destination)), b"value");
        // A missing key, `=`, opening quote or closing quote each answer 0 and store nothing.
        for bad in [
            "<a other=\"x\">",
            "<a key \"value\">",
            "<a key = value>",
            "<a key = \"value>",
        ] {
            let haystack = text(&mut e, bad);
            e.mem.set_u32(destination, 0);
            assert_eq!(
                e.call(0x00d4_8440, &args![haystack, needle, destination])
                    .u32(),
                0,
                "{bad}"
            );
            assert_eq!(e.mem.u32(destination), 0, "{bad}");
        }
    }

    #[test]
    fn fn_00d48560_saves_through_the_writer_vtable_word() {
        let mut e = engine();
        e.mem.set_u32(TAGFILE_WRITER_SAVE_SLOT, SAVE_DOUBLE);
        let writers: Rc<RefCell<Vec<(u32, u32, u16)>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&writers);
        e.register_double(SAVE_DOUBLE, move |e, a| {
            seen.borrow_mut()
                .push((a[0], e.mem.u32(a[0]), e.mem.u16(a[0] + 6)));
            answer(a[1] + a[2] + a[3])
        });
        e.call_log = Some(vec![]);
        // Without a stream nothing is called and the answer is 1.
        assert_eq!(e.call(0x00d4_8560, &args![0u32, 0x20u32, 0x30u32]).u32(), 1);
        assert!(calls_to(&mut e, SAVE_DOUBLE).is_empty());
        // `this` (the writer object, vtable `010cf9c8`, the word 1 at `+6`) and `(object, stream,
        // listener)` reach `save`, whose answer is returned.
        let got = e.call(0x00d4_8560, &args![0x10u32, 0x20u32, 0x30u32]).u32();
        assert_eq!(got, 0x20 + 0x30 + 0x10);
        let writer = writers.borrow()[0].0;
        assert_eq!(
            calls_to(&mut e, SAVE_DOUBLE),
            vec![vec![writer, 0x30, 0x10, 0x20]]
        );
        assert_eq!(
            (writers.borrow()[0].1, writers.borrow()[0].2),
            (TAGFILE_WRITER_VTABLE, 1)
        );
    }

    #[test]
    fn save_tagfile_writes_the_contents_with_a_listener_over_the_world() {
        let mut e = engine();
        e.mem.set_u32(TAGFILE_WRITER_SAVE_SLOT, SAVE_DOUBLE);
        let seen: Rc<RefCell<Vec<[u32; 6]>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SAVE_DOUBLE, move |e, a| {
            let listener = a[3];
            log.borrow_mut().push([
                a[1],
                a[2],
                e.mem.u32(listener),
                e.mem.u32(listener + 8),
                e.mem.u32(listener + 0xc),
                u32::from(e.mem.u16(listener + 6)),
            ]);
            answer(0x5a5a)
        });
        e.register(WORLD_GET_CONTENTS, |e, a| {
            e.mem.set_u32(a[1], 0xc0de);
            answer(a[1])
        });
        e.call_log = Some(vec![]);
        let got = e
            .call(
                0x00d4_85a0,
                &args![0x1111u32, 0x2222u32, 0x3333u32, 0x4444u32, 0x5555u32],
            )
            .u32();
        assert_eq!(got, 0x5a5a);
        let log = all_calls(&mut e);
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| {
                [
                    WORLD_CONSTRUCTOR,
                    WORLD_SET_CONTENTS,
                    WORLD_GET_CONTENTS,
                    SAVE_DOUBLE,
                    DATA_OBJECT_DESTRUCTOR,
                    WORLD_DESTRUCTOR,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                WORLD_CONSTRUCTOR,
                WORLD_SET_CONTENTS,
                WORLD_GET_CONTENTS,
                SAVE_DOUBLE,
                DATA_OBJECT_DESTRUCTOR,
                WORLD_DESTRUCTOR
            ]
        );
        let world = log.iter().find(|c| c.0 == WORLD_CONSTRUCTOR).unwrap().1[0];
        let set = log.iter().find(|c| c.0 == WORLD_SET_CONTENTS).unwrap();
        assert_eq!(set.1, vec![world, 0x1111, 0x2222]);
        let get = log.iter().find(|c| c.0 == WORLD_GET_CONTENTS).unwrap();
        assert_eq!(get.1[0], world);
        let handle = get.1[1];
        let destroyed = log.iter().find(|c| c.0 == DATA_OBJECT_DESTRUCTOR).unwrap();
        assert_eq!(destroyed.1, vec![handle]);
        // `save(object = the handle, stream, listener)`; the listener holds the vtable, the
        // word 1, the forwarded listener (the last argument) and the world.
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0], [handle, 0x3333, LISTENER_VTABLE, 0x5555, world, 1]);
    }

    #[test]
    fn load_stream_into_buffer_reads_a_seekable_stream_in_one_go() {
        let mut e = engine();
        install_router(&mut e);
        e.register_double(ARRAY_RESERVE, |e, a| {
            // Grows the array's storage to `a[2]` bytes (flags without the not-owned bit).
            let (array, bytes) = (a[1], a[2]);
            let data = e.mem.alloc(bytes);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, bytes);
            Ret::default()
        });
        let payload: Vec<u8> = (0..100u8).collect();
        let source = stream(&mut e, &payload, true, true);
        let buffer = e.mem.alloc(12);
        e.mem.set_u32(buffer + 8, 0x8000_0010);
        e.call_log = Some(vec![]);
        e.call(0x00d4_8680, &args![source, buffer]);
        let reserve = calls_to(&mut e, ARRAY_RESERVE);
        // Capacity 0x10 is below 100 and its double (0x20) is too: the size is asked for.
        assert_eq!(reserve.len(), 1);
        assert_eq!(&reserve[0][1..], &[buffer, 100, 1]);
        assert_eq!(e.mem.u32(buffer + 4), 100);
        assert_eq!(e.mem.bytes(e.mem.u32(buffer), 100), payload);
    }

    #[test]
    fn load_stream_into_buffer_doubles_the_capacity_when_that_is_more() {
        let mut e = engine();
        install_router(&mut e);
        e.register_double(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source = stream(&mut e, &[7; 40], true, true);
        let buffer = e.mem.alloc(12);
        e.mem.set_u32(buffer + 8, 0x8000_0020);
        e.call_log = Some(vec![]);
        e.call(0x00d4_8680, &args![source, buffer]);
        // Capacity 0x20 < 40 and double 0x40 > 40: reserves 0x40.
        assert_eq!(calls_to(&mut e, ARRAY_RESERVE)[0][2], 0x40);
        assert_eq!(e.mem.u32(buffer + 4), 40);
    }

    #[test]
    fn load_stream_into_buffer_reads_other_streams_in_blocks_until_nothing_comes() {
        let mut e = engine();
        install_router(&mut e);
        e.register_double(ARRAY_RESERVE, |e, a| {
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let data = e.mem.alloc(a[2]);
            if size > 0 {
                let kept = e.mem.bytes(old, size);
                e.mem.write(data, &kept);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let payload: Vec<u8> = (0..200u8).collect();
        let source = stream(&mut e, &payload, true, false);
        let buffer = e.mem.alloc(12);
        e.mem.set_u32(buffer + 8, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00d4_8680, &args![source, buffer]);
        // Two reads: the first gets 200 bytes, the second 0 and ends the loop.
        assert_eq!(e.mem.u32(buffer + 4), 200);
        assert_eq!(e.mem.bytes(e.mem.u32(buffer), 200), payload);
        let reserves = calls_to(&mut e, ARRAY_RESERVE);
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[0][2], 0x20000);
        // Capacity 0x20000, size 200: 200 + 0x20000 is below the doubled capacity 0x40000.
        assert_eq!(reserves[1][2], 0x40000);
    }

    /// A packfile format record: `format`, the version string pointer and the layout rules.
    fn header(e: &mut Engine, format: u32, version: Option<&str>, rules: [u8; 4]) -> u32 {
        let record = e.mem.alloc(0xc);
        e.mem.set_u32(record, format);
        if let Some(version) = version {
            let at = text(e, version);
            e.mem.set_u32(record + 4, at);
        }
        e.mem.write(record + 8, &rules);
        record
    }

    #[test]
    fn load_packfile_reads_a_native_packfile_and_releases_the_buffer() {
        let mut e = engine();
        install_singletons(&mut e);
        let frees = install_router(&mut e);
        e.register_double(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.register(NATIVE_PACKFILE_LOAD, |_, a| answer(a[0] ^ a[1] ^ a[2]));
        let record = header(&mut e, 2, Some("Havok-7.1.0-r1"), [1, 2, 3, 4]);
        let source = stream(&mut e, &[9; 64], true, true);
        e.call_log = Some(vec![]);
        // The buffer array sits at +0xc of the 0x18-byte frame; the seekable read flag at +0x7
        // of the stream reader's 8-byte frame, nested below it.
        let frame = next_frame(&mut e, 0x18);
        let flag_frame = e.with_stack(0x18, |e, outer| {
            assert_eq!(outer.addr(), frame);
            e.next_stack_local(8)
        });
        let got = e
            .call(0x00d4_8760, &args![0xe000u32, record, source, 0x7001u32])
            .u32();
        assert_eq!(calls_to(&mut e, ARRAY_RESERVE)[0][1], frame + 0xc);
        assert_eq!(calls_to(&mut e, STREAM_SLOTS + 0x24)[0][1], flag_frame + 7);
        let loads = calls_to(&mut e, NATIVE_PACKFILE_LOAD);
        assert_eq!(loads.len(), 1);
        let data = loads[0][0];
        assert_eq!(&loads[0][1..], &[64, 0x7001]);
        assert_eq!(got, data ^ 64 ^ 0x7001);
        // The buffer's storage (capacity 64, owned) goes back through the allocator.
        assert_eq!(*frees.borrow(), vec![[data, 64]]);
        assert!(calls_to(&mut e, DEPRECATED_SLOTS + 0x14).is_empty());
    }

    #[test]
    fn load_packfile_reports_a_wrong_platform_and_forwards_other_formats() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        let source = stream(&mut e, &[], true, true);
        // Wrong layout rules: error 3 and no result.
        let wrong = header(&mut e, 2, Some("Havok-7.1.0-r1"), [4, 3, 2, 1]);
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d4_8760, &args![err, wrong, source, 0x7001u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(err), 3);
        assert_eq!(
            string_of(&e, e.mem.u32(err + 4)),
            b"Wrong platform for packfile"
        );
        // Without an error structure: just 0.
        assert_eq!(
            e.call(0x00d4_8760, &args![0u32, wrong, source, 0x7001u32])
                .u32(),
            0
        );
        assert!(calls_to(&mut e, DEPRECATED_SLOTS + 0x14).is_empty());
        // Another format, and a packfile of another version, go to the deprecated loader.
        for (format, version) in [(3, "Havok-7.1.0-r1"), (2, "Havok-5.0.0-r1")] {
            let record = header(&mut e, format, Some(version), [1, 2, 3, 4]);
            let got = e
                .call(0x00d4_8760, &args![0xe000u32, record, source, 0x7001u32])
                .u32();
            assert_eq!(got, 0xd014);
            let forwarded = calls_to(&mut e, DEPRECATED_SLOTS + 0x14);
            assert_eq!(forwarded.last().unwrap()[1..], [source, record, 0xe000]);
        }
        assert!(calls_to(&mut e, NATIVE_PACKFILE_LOAD).is_empty());
    }

    #[test]
    fn load_new_packfile_on_heap_converts_the_contents_of_the_resource() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        e.register(WORLD_WRAP_OBJECT, |_, a| answer(a[1] + 1));
        e.register(TO_OBJECT_WITH_REGISTRY, |_, a| answer(a[0] ^ 0xff00));
        // A resource whose contents type is 0x44 and contents 0x99; the class registry answers
        // a class (0xc1a5) for that type.
        e.register(RESOURCE_SLOTS + 0x1c, |_, _| answer(0x44));
        e.register(RESOURCE_SLOTS + 0x18, |_, _| answer(0x99));
        let resource = object_with_slots(
            &mut e,
            &[(0x1c, RESOURCE_SLOTS + 0x1c), (0x18, RESOURCE_SLOTS + 0x18)],
        );
        e.register_double(CLASS_REGISTRY_LOOKUP, |_, a| {
            answer(if a[1] == 0x44 { 0xc1a5 } else { 0 })
        });
        let class_registry = object_with_slots(&mut e, &[(0x10, CLASS_REGISTRY_LOOKUP)]);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.register_double(NATIVE_PACKFILE_LOAD, move |_, _| answer(resource));
        let record = header(&mut e, 2, Some("Havok-7.1.0-r1"), [1, 2, 3, 4]);
        let source = stream(&mut e, &[1; 8], true, true);
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x58);
        let got = e
            .call(
                0x00d4_8840,
                &args![class_registry, source, record, 0xe000u32, 0x7001u32],
            )
            .u32();
        let log = all_calls(&mut e);
        let find = |address: u32| log.iter().filter(|c| c.0 == address).collect::<Vec<_>>();
        // The local registry is merged from the argument and handed to the packfile load.
        let merges = find(TYPE_INFO_REGISTRY_MERGE);
        assert_eq!(merges[0].1[1], 0x7001);
        let local_registry = merges[0].1[0];
        // The exe frame: the registry at +0x14, the world at +0x30.
        assert_eq!(local_registry, frame + 0x14);
        assert_eq!(find(WORLD_CONSTRUCTOR)[0].1[0], frame + 0x30);
        assert_eq!(find(NATIVE_PACKFILE_LOAD)[0].1[2], local_registry);
        let map = log
            .iter()
            .find(|c| c.0 == STRING_MAP_CONSTRUCTOR)
            .unwrap()
            .1[0];
        assert_eq!(map, local_registry + 8);
        // The class registry is set on the world; the object is wrapped with the class found.
        assert_eq!(find(WORLD_SET_CLASS_REGISTRY)[0].1[1], class_registry);
        assert_eq!(find(WORLD_WRAP_OBJECT)[0].1[1..], [0x99, 0xc1a5]);
        let wrapped_to_handle = &find(DATA_OBJECT_CONSTRUCTOR)[0].1;
        assert_eq!(wrapped_to_handle[1], 0x9a);
        let converted = &find(TO_OBJECT_WITH_REGISTRY)[0].1;
        assert_eq!(converted[0], wrapped_to_handle[0]);
        assert_eq!(converted[1..], [class_registry, 0x7001]);
        assert_eq!(got, wrapped_to_handle[0] ^ 0xff00);
        // The resource is released, the local registry's map destroyed.
        assert_eq!(find(REMOVE_REFERENCE)[0].1, vec![resource]);
        assert_eq!(find(STRING_MAP_DESTRUCTOR)[0].1, vec![local_registry + 8]);
    }

    #[test]
    fn load_new_packfile_on_heap_answers_0_for_an_unknown_class() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        e.register(RESOURCE_SLOTS + 0x1c, |_, _| answer(0x44));
        let resource = object_with_slots(&mut e, &[(0x1c, RESOURCE_SLOTS + 0x1c)]);
        let class_registry = object_with_slots(&mut e, &[(0x10, CLASS_REGISTRY_LOOKUP)]);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.register_double(NATIVE_PACKFILE_LOAD, move |_, _| answer(resource));
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let record = header(&mut e, 2, Some("Havok-7.1.0-r1"), [1, 2, 3, 4]);
        let source = stream(&mut e, &[1; 8], true, true);
        e.call_log = Some(vec![]);
        let got = e
            .call(
                0x00d4_8840,
                &args![class_registry, source, record, 0xe000u32, 0x7001u32],
            )
            .u32();
        assert_eq!(got, 0);
        assert!(calls_to(&mut e, WORLD_WRAP_OBJECT).is_empty());
        assert_eq!(calls_to(&mut e, WORLD_DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&mut e, REMOVE_REFERENCE), vec![vec![resource]]);
        assert_eq!(calls_to(&mut e, STRING_MAP_DESTRUCTOR).len(), 1);
    }

    /// A world whose class list (slot `0x20`) is `classes`, each class object answering its name
    /// (slot 8) and version (slot `0xc`).
    fn world_with_classes(e: &mut Engine, classes: &[(u32, u32)]) -> u32 {
        for (i, (name, version)) in classes.iter().enumerate() {
            let base = CLASS_OBJECT_SLOTS + 0x100 * i as u32;
            let (name, version) = (*name, *version);
            e.register_double(base + 8, move |_, _| answer(name));
            e.register_double(base + 0xc, move |_, _| answer(version));
        }
        let objects: Vec<u32> = (0..classes.len() as u32)
            .map(|i| {
                let base = CLASS_OBJECT_SLOTS + 0x100 * i;
                object_with_slots(e, &[(8, base + 8), (0xc, base + 0xc)])
            })
            .collect();
        let count = objects.len() as u32;
        e.register_double(WORLD_SLOTS + 0x20, move |e, a| {
            let data = e.mem.alloc(4 * count.max(1));
            for (i, object) in objects.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *object);
            }
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 4, count);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        object_with_slots(e, &[(0x20, WORLD_SLOTS + 0x20)])
    }

    /// A class registry that knows each `(name, hkClass)` of `known`; the hkClass objects'
    /// described version is the pointer minus 0x1000.
    fn registry_knowing(e: &mut Engine, known: &'static [(u32, u32)]) -> u32 {
        e.register_double(CLASS_REGISTRY_LOOKUP, move |_, a| {
            answer(known.iter().find(|k| k.0 == a[1]).map_or(0, |k| k.1))
        });
        e.register(CLASS_GET_DESCRIBED_VERSION, |_, a| answer(a[0] - 0x1000));
        object_with_slots(e, &[(0x10, CLASS_REGISTRY_LOOKUP)])
    }

    #[test]
    fn check_tagfile_world_accepts_known_and_patchable_classes() {
        let mut e = engine();
        let frees = install_router(&mut e);
        // Class 0x10 at version 5 is known; class 0x20 at version 6 is known at 5 but a patch exists.
        let world = world_with_classes(&mut e, &[(0x10, 5), (0x20, 6)]);
        let registry = registry_knowing(&mut e, &[(0x10, 0x1005), (0x20, 0x1005)]);
        e.register(PATCH_MANAGER_GET_UID, |_, a| Ret {
            eax: a[1],
            edx: a[2],
            ..Ret::default()
        });
        e.register(MAP_GET_WITH_DEFAULT, |_, a| {
            answer(if a[1] == 0x20 { 7 } else { 0xffff_ffff })
        });
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x1c);
        let got = e
            .call(0x00d4_8950, &args![world, err, 0x7002u32, registry])
            .u32();
        assert_eq!(got, 1);
        // The class array sits at +0x10 of the exe frame.
        assert_eq!(calls_to(&mut e, WORLD_SLOTS + 0x20)[0][1], frame + 0x10);
        assert_eq!(
            calls_to(&mut e, RECOMPUTE_PATCH_DEPENDENCIES),
            vec![vec![0x7002]]
        );
        // Only the mismatching class asked the patch map (this = patch manager + 0x18, key lo/hi,
        // default lo/hi).
        let patch = calls_to(&mut e, MAP_GET_WITH_DEFAULT);
        assert_eq!(
            patch,
            vec![vec![0x7002 + 0x18, 0x20, 6, 0xffff_ffff, 0xffff_ffff]]
        );
        assert_eq!(e.mem.u32(err), 0);
        // The array (owned, capacity 4) is released with 16 bytes.
        let freed = frees.borrow();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1], 16);
    }

    #[test]
    fn check_tagfile_world_reports_a_class_it_cannot_version() {
        let mut e = engine();
        let frees = install_router(&mut e);
        let world = world_with_classes(&mut e, &[(0x10, 5), (0x30, 9)]);
        // Class 0x30 is unknown to the registry and the patch map has nothing for it.
        let registry = registry_knowing(&mut e, &[(0x10, 0x1005)]);
        e.register(MAP_GET_WITH_DEFAULT, |_, _| answer(0xffff_ffff));
        e.register(PATCH_MANAGER_GET_UID, |_, a| Ret {
            eax: a[1],
            edx: a[2],
            ..Ret::default()
        });
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00d4_8950, &args![world, err, 0x7002u32, registry])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(err), 4);
        assert_eq!(
            calls_to(&mut e, STRING_PTR_PRINTF),
            vec![vec![err + 4, FORMAT_UNABLE_TO_VERSION, 0x30, 9]]
        );
        assert_eq!(frees.borrow().len(), 1);
    }

    #[test]
    fn check_tagfile_world_with_no_classes_answers_1() {
        let mut e = engine();
        let frees = install_router(&mut e);
        let world = world_with_classes(&mut e, &[]);
        let registry = registry_knowing(&mut e, &[]);
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d4_8950, &args![world, err, 0x7002u32, registry])
                .u32(),
            1
        );
        assert!(calls_to(&mut e, STRING_PTR_PRINTF).is_empty());
        assert_eq!(frees.borrow().len(), 1);
    }

    #[test]
    fn load_binary_tagfile_into_world_checks_then_patches() {
        let mut e = engine();
        install_router(&mut e);
        e.register(APPLY_PATCHES, |_, a| answer(a[1] + a[2]));
        let world = world_with_classes(&mut e, &[]);
        let registry = registry_knowing(&mut e, &[]);
        e.mem.set_u32(PATCH_MANAGER_SINGLETON, 0x7002);
        let source = stream(&mut e, &[], true, true);
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        // With an error structure the world is checked (no classes: fine), then patched.
        let frame = next_frame(&mut e, 0xc);
        let got = e
            .call(0x00d4_8a90, &args![world, err, registry, source])
            .u32();
        assert_eq!(got, world + registry);
        let load = calls_to(&mut e, TAGFILE_READER_LOAD);
        assert_eq!(load.len(), 1);
        // The handle at +0 of the exe frame, the reader at +0x4.
        assert_eq!(load[0][..2], [frame + 4, frame]);
        assert_eq!(load[0][2..], [source, world]);
        assert_eq!(calls_to(&mut e, RECOMPUTE_PATCH_DEPENDENCIES).len(), 1);
        assert_eq!(
            calls_to(&mut e, APPLY_PATCHES),
            vec![vec![0x7002, world, registry]]
        );
        // Without one the check is skipped.
        e.call_log = Some(vec![]);
        e.call(0x00d4_8a90, &args![world, 0u32, registry, source]);
        assert!(calls_to(&mut e, RECOMPUTE_PATCH_DEPENDENCIES).is_empty());
        assert_eq!(calls_to(&mut e, APPLY_PATCHES).len(), 1);
    }

    #[test]
    fn load_binary_tagfile_into_world_answers_1_when_the_check_fails() {
        let mut e = engine();
        install_router(&mut e);
        let world = world_with_classes(&mut e, &[(0x30, 9)]);
        let registry = registry_knowing(&mut e, &[]);
        e.register(PATCH_MANAGER_GET_UID, |_, _| Ret::default());
        e.register(MAP_GET_WITH_DEFAULT, |_, _| answer(0xffff_ffff));
        let source = stream(&mut e, &[], true, true);
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00d4_8a90, &args![world, err, registry, source])
            .u32();
        assert_eq!(got, 1);
        assert!(calls_to(&mut e, APPLY_PATCHES).is_empty());
    }

    /// The first 0x40 bytes of a packfile: both magic words, rules at `+0x10`, a version string
    /// at `+0x28`.
    fn packfile_bytes(version: &str) -> Vec<u8> {
        let mut bytes = vec![0u8; 0x80];
        bytes[0..4].copy_from_slice(&PACKFILE_MAGIC_FIRST.to_le_bytes());
        bytes[4..8].copy_from_slice(&PACKFILE_MAGIC_SECOND.to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&[1, 2, 3, 4]);
        bytes[0x28..0x28 + version.len()].copy_from_slice(version.as_bytes());
        bytes
    }

    fn detect(e: &mut Engine, data: &[u8]) -> (u32, u32) {
        let source = stream(e, data, true, true);
        let result = e.mem.alloc(12);
        e.mem.set_u32(result + 8, 0xdead_beef);
        e.call(0x00d4_8af0, &args![source, result]);
        (result, source)
    }

    #[test]
    fn detect_format_finds_binary_tagfiles_by_their_magic() {
        let mut e = engine();
        let mut data = TAGFILE_MAGIC_FIRST.to_le_bytes().to_vec();
        data.extend(TAGFILE_MAGIC_SECOND.to_le_bytes());
        let (result, _) = detect(&mut e, &data);
        assert_eq!(e.mem.u32(result), 4);
        // Fewer than 8 bytes cannot be a tagfile.
        let (result, _) = detect(&mut e, &data[..7]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn detect_format_finds_packfiles_and_copies_version_and_rules() {
        let mut e = engine();
        let (result, _) = detect(&mut e, &packfile_bytes("Havok-7.1.0-r1"));
        assert_eq!(e.mem.u32(result), 2);
        // The version is the string at +0x28 of the read buffer (the double stores the pointer).
        assert_eq!(string_of(&e, e.mem.u32(result + 4)), b"Havok-7.1.0-r1");
        // The four bytes at +0x10 of the header were copied to +8 of the result.
        assert_eq!(e.mem.bytes(result + 8, 4), vec![1, 2, 3, 4]);
        // Fewer than 0x40 bytes read is not a packfile.
        let short = packfile_bytes("Havok-7.1.0-r1");
        let (result, _) = detect(&mut e, &short[..0x3f]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn detect_format_reads_the_xml_packfile_versions() {
        let cases = [
            // No classversion attribute: 3.0.0.
            ("<hkpackfile>", "Havok-3.0.0"),
            ("<hkpackfile classversion=\"1\">", "Havok-3.0.0"),
            // classversion 2..7 without contentsversion: 3.1.0.
            ("<hkpackfile classversion=\"3\">", "Havok-3.1.0"),
            // ... with it: the attribute's text.
            (
                "<hkpackfile classversion=\"7\" contentsversion=\"hk_2.0\">",
                "hk_2.0",
            ),
            // Anything else: error.
            ("<hkpackfile classversion=\"8\">", "error"),
            ("<hkpackfile classversion=\"0\">", "error"),
        ];
        for (xml, expected) in cases {
            let mut e = engine();
            install_router(&mut e);
            e.register(STRING_PTR_SET, string_ptr_set);
            let mut data = b"junk ".to_vec();
            data.extend(xml.as_bytes());
            let (result, _) = detect(&mut e, &data);
            assert_eq!(e.mem.u32(result), 3, "{xml}");
            assert_eq!(
                string_of(&e, e.mem.u32(result + 4)),
                expected.as_bytes(),
                "{xml}"
            );
        }
    }

    #[test]
    fn detect_format_leaves_unknown_data_at_format_0_and_releases_the_string_buffer() {
        let mut e = engine();
        let (result, _) = detect(&mut e, b"hello world");
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.u32(result + 8), 0);
        // The xml path releases the string buffer's storage unless it is not owned.
        let mut e = engine();
        let frees = install_router(&mut e);
        e.register(STRING_BUF_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 8, 0x20);
            Ret::default()
        });
        let (result, _) = detect(&mut e, b"<hkpackfile>");
        assert_eq!(e.mem.u32(result), 3);
        let freed = frees.borrow();
        assert_eq!(freed.len(), 1);
        assert_eq!(freed[0][1], 0x20);
    }

    #[test]
    fn detect_format_ov2_stores_only_the_format_word() {
        let mut e = engine();
        let mut data = TAGFILE_MAGIC_FIRST.to_le_bytes().to_vec();
        data.extend(TAGFILE_MAGIC_SECOND.to_le_bytes());
        let source = stream(&mut e, &data, true, true);
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00d4_9310, &args![out, source]).u32(), out);
        assert_eq!(e.mem.u32(out), 4);
        let ctor = calls_to(&mut e, STRING_PTR_CONSTRUCTOR);
        let dtor = calls_to(&mut e, STRING_PTR_DESTRUCTOR);
        assert_eq!(ctor.len(), 1);
        assert_eq!(ctor, dtor);
    }

    #[test]
    fn load_reports_a_null_or_broken_stream() {
        let mut e = engine();
        install_singletons(&mut e);
        let err = e.mem.alloc(8);
        assert_eq!(e.call(0x00d4_8d30, &args![0u32, err, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(err), 1);
        assert_eq!(string_of(&e, e.mem.u32(err + 4)), b"Stream pointer is null");
        // No error structure: just 0.
        assert_eq!(e.call(0x00d4_8d30, &args![0u32, 0u32, 0u32]).u32(), 0);
        let broken = stream(&mut e, &[], false, true);
        e.mem.set_u32(err, 0);
        assert_eq!(e.call(0x00d4_8d30, &args![broken, err, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(err), 1);
        assert_eq!(string_of(&e, e.mem.u32(err + 4)), b"Stream is not ok");
    }

    #[test]
    fn load_dispatches_on_the_detected_format() {
        // Format 4: a tagfile is read into a dict world and made into a resource.
        let mut e = engine();
        let class_registry = install_singletons(&mut e);
        install_router(&mut e);
        e.register(DICT_GET_CONTENTS, |e, a| {
            e.mem.set_u32(a[1], 0xc0de);
            answer(a[1])
        });
        e.register(TO_RESOURCE_WITH_REGISTRY, |_, a| answer(a[1] + 1));
        e.register(APPLY_PATCHES, |_, _| answer(0));
        let mut data = TAGFILE_MAGIC_FIRST.to_le_bytes().to_vec();
        data.extend(TAGFILE_MAGIC_SECOND.to_le_bytes());
        let source = stream(&mut e, &data, true, true);
        e.call_log = Some(vec![]);
        // Null registry: the singleton's class registry.
        let got = e.call(0x00d4_8d30, &args![source, 0u32, 0u32]).u32();
        assert_eq!(got, class_registry + 1);
        assert_eq!(calls_to(&mut e, DICT_CONSTRUCTOR).len(), 1);
        assert_eq!(calls_to(&mut e, DICT_DESTRUCTOR).len(), 1);
        // One handle is the tagfile reader's throw-away, the other the contents'.
        assert_eq!(calls_to(&mut e, DATA_OBJECT_DESTRUCTOR).len(), 2);
        // An explicit registry is used as given.
        let got = e.call(0x00d4_8d30, &args![source, 0u32, 0x1234u32]).u32();
        assert_eq!(got, 0x1235);
    }

    #[test]
    fn load_stops_when_the_tagfile_cannot_be_patched() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        e.register(APPLY_PATCHES, |_, _| answer(1));
        let mut data = TAGFILE_MAGIC_FIRST.to_le_bytes().to_vec();
        data.extend(TAGFILE_MAGIC_SECOND.to_le_bytes());
        let source = stream(&mut e, &data, true, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_8d30, &args![source, 0u32, 0u32]).u32(), 0);
        assert!(calls_to(&mut e, TO_RESOURCE_WITH_REGISTRY).is_empty());
        assert_eq!(calls_to(&mut e, DICT_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn load_sends_packfiles_to_load_packfile_and_rejects_unknown_formats() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        // Format 3 (xml): loadPackfile forwards it to the deprecated loader.
        let source = stream(&mut e, b"<hkpackfile>", true, true);
        e.call_log = Some(vec![]);
        let got = e.call(0x00d4_8d30, &args![source, 0u32, 0u32]).u32();
        assert_eq!(got, 0xd014);
        assert_eq!(calls_to(&mut e, DEPRECATED_SLOTS + 0x14).len(), 1);
        // `detectFormat`'s temporary and the format record's string.
        assert_eq!(calls_to(&mut e, STRING_PTR_DESTRUCTOR).len(), 2);
        // Unknown data: error 2.
        let source = stream(&mut e, b"nothing", true, true);
        let err = e.mem.alloc(8);
        assert_eq!(e.call(0x00d4_8d30, &args![source, err, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(err), 2);
        assert_eq!(
            string_of(&e, e.mem.u32(err + 4)),
            b"Unable to detect format from stream"
        );
    }

    #[test]
    fn load_ov2_loads_a_matching_buffer_directly_and_wraps_the_rest_in_a_stream() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.register(NATIVE_PACKFILE_LOAD, |_, a| answer(a[0] + a[1] + a[2]));
        let bytes = packfile_bytes("Havok-7.1.0-r1");
        let buffer = e.mem.alloc(0x80);
        e.mem.write(buffer, &bytes);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00d4_8f00, &args![buffer, 0x80u32, 0u32, 0u32])
            .u32();
        assert_eq!(got, buffer + 0x80);
        assert_eq!(
            calls_to(&mut e, NATIVE_PACKFILE_LOAD),
            vec![vec![buffer, 0x80, 0]]
        );
        assert!(calls_to(&mut e, ISTREAM_CONSTRUCTOR).is_empty());
        // The local the code fills with 0xff is a call.
        assert_eq!(calls_to(&mut e, MEMORY_SET).len(), 1);
        // A different version goes through the istream: its reader (+8) is loaded as a stream.
        let other = packfile_bytes("Havok-5.0.0-r1");
        e.mem.write(buffer, &other);
        let source = stream(&mut e, &[], false, true);
        e.register_double(ISTREAM_CONSTRUCTOR, move |e, a| {
            e.mem.set_u32(a[0] + 8, source);
            answer(a[0])
        });
        let err = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let got = e
            .call(0x00d4_8f00, &args![buffer, 0x80u32, err, 0u32])
            .u32();
        // The reader reports not-ok: `load` reports error 1 and answers 0.
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(err), 1);
        let made = calls_to(&mut e, ISTREAM_CONSTRUCTOR);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1..], [buffer, 0x80]);
        assert_eq!(calls_to(&mut e, ISTREAM_DESTRUCTOR), vec![vec![made[0][0]]]);
        assert!(calls_to(&mut e, NATIVE_PACKFILE_LOAD).is_empty());
    }

    #[test]
    fn load_on_heap_uses_the_new_loader_for_current_packfiles() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        let class_registry = object_with_slots(&mut e, &[(0x10, CLASS_REGISTRY_LOOKUP)]);
        e.register(RESOURCE_SLOTS + 0x1c, |_, _| answer(0x44));
        let resource = object_with_slots(&mut e, &[(0x1c, RESOURCE_SLOTS + 0x1c)]);
        e.register_double(CLASS_REGISTRY_LOOKUP, |_, _| answer(0));
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.register_double(NATIVE_PACKFILE_LOAD, move |_, _| answer(resource));
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source = stream(&mut e, &packfile_bytes("Havok-7.1.0-r1"), true, true);
        e.call_log = Some(vec![]);
        // Format 2 with this platform's rules and the current contents version: the unknown
        // class makes the new loader answer 0 (and nothing is forwarded).
        let got = e
            .call(0x00d4_8fc0, &args![source, 0u32, class_registry, 0x7001u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(calls_to(&mut e, WORLD_CONSTRUCTOR).len(), 1);
        assert!(calls_to(&mut e, DEPRECATED_SLOTS + 0x18).is_empty());
    }

    #[test]
    fn load_on_heap_forwards_other_packfiles_and_converts_tagfiles() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        // A packfile whose contents version is not the current one goes to slot 0x18.
        let source = stream(&mut e, &packfile_bytes("Havok-5.0.0-r1"), true, true);
        e.call_log = Some(vec![]);
        let got = e.call(0x00d4_8fc0, &args![source, 0u32, 0u32, 0u32]).u32();
        assert_eq!(got, 0xd018);
        let forwarded = calls_to(&mut e, DEPRECATED_SLOTS + 0x18);
        assert_eq!(forwarded.len(), 1);
        assert_eq!(forwarded[0][1], source);
        // XML packfiles go there too.
        let source = stream(&mut e, b"<hkpackfile>", true, true);
        assert_eq!(
            e.call(0x00d4_8fc0, &args![source, 0u32, 0u32, 0u32]).u32(),
            0xd018
        );
        // A tagfile is read into a dict and converted with both registries.
        e.register(DICT_GET_CONTENTS, |e, a| {
            e.mem.set_u32(a[1], 0xc0de);
            answer(a[1])
        });
        e.register(TO_OBJECT_WITH_REGISTRY, |_, a| answer(a[1] + a[2]));
        e.register(APPLY_PATCHES, |_, _| answer(0));
        let mut data = TAGFILE_MAGIC_FIRST.to_le_bytes().to_vec();
        data.extend(TAGFILE_MAGIC_SECOND.to_le_bytes());
        let source = stream(&mut e, &data, true, true);
        let got = e
            .call(0x00d4_8fc0, &args![source, 0u32, 0x10u32, 0x20u32])
            .u32();
        assert_eq!(got, 0x30);
        // Unknown data: error 2; broken stream: error 1; null: error 1.
        let err = e.mem.alloc(8);
        let source = stream(&mut e, b"nothing", true, true);
        assert_eq!(
            e.call(0x00d4_8fc0, &args![source, err, 0u32, 0u32]).u32(),
            0
        );
        assert_eq!(e.mem.u32(err), 2);
        assert_eq!(e.call(0x00d4_8fc0, &args![0u32, err, 0u32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(err), 1);
        assert_eq!(string_of(&e, e.mem.u32(err + 4)), b"Stream pointer is null");
        let broken = stream(&mut e, &[], false, true);
        assert_eq!(
            e.call(0x00d4_8fc0, &args![broken, err, 0u32, 0u32]).u32(),
            0
        );
        assert_eq!(string_of(&e, e.mem.u32(err + 4)), b"Stream is not ok");
    }

    /// The stack local the next frame of `size` bytes takes (an outermost one: the called
    /// function's own frame).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    #[test]
    fn the_locals_sit_at_the_exe_frame_offsets() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        e.mem.set_u32(TAGFILE_WRITER_SAVE_SLOT, SAVE_DOUBLE);
        e.register(SAVE_DOUBLE, |_, _| answer(0));
        e.register(WORLD_GET_CONTENTS, |_, a| answer(a[1]));
        e.call_log = Some(vec![]);
        // 00d48560: the writer is its whole 8-byte frame.
        let frame = next_frame(&mut e, 8);
        e.call(0x00d4_8560, &args![0x10u32, 0x20u32, 0x30u32]);
        assert_eq!(calls_to(&mut e, SAVE_DOUBLE)[0][0], frame);
        // 00d485a0: the world at +0x14, the handle in the last argument's slot (+0x50).
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x54);
        e.call(
            0x00d4_85a0,
            &args![0x1111u32, 0x2222u32, 0x3333u32, 0x4444u32, 0x5555u32],
        );
        assert_eq!(
            calls_to(&mut e, WORLD_CONSTRUCTOR),
            vec![vec![frame + 0x14]]
        );
        assert_eq!(
            calls_to(&mut e, WORLD_GET_CONTENTS),
            vec![vec![frame + 0x14, frame + 0x50]]
        );
        // 00d48af0: the read buffer at +0x18 of 0x164 bytes.
        e.call_log = Some(vec![]);
        let source = stream(&mut e, &[0u8; 8], true, true);
        let result = e.mem.alloc(12);
        let frame = next_frame(&mut e, 0x164);
        e.call(0x00d4_8af0, &args![source, result]);
        assert_eq!(calls_to(&mut e, CRT_MEMSET)[0][0], frame + 0x18);
        // 00d49310: the format record at +0x4.
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x10);
        e.call(0x00d4_9310, &args![result, source]);
        assert_eq!(calls_to(&mut e, STRING_PTR_CONSTRUCTOR)[0][0], frame + 8);
        // 00d48d30 / 00d48fc0: an old packfile goes to the deprecated loader with the format
        // record at +0xc / +0x14.
        let bytes = packfile_bytes("Havok-5.0.0-r1");
        let source = stream(&mut e, &bytes, true, true);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x34);
        e.call(0x00d4_8d30, &args![source, 0u32, 0u32]);
        assert_eq!(calls_to(&mut e, DEPRECATED_SLOTS + 0x14)[0][2], frame + 0xc);
        let source = stream(&mut e, &bytes, true, true);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x40);
        e.call(0x00d4_8fc0, &args![source, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&mut e, DEPRECATED_SLOTS + 0x18)[0][2],
            frame + 0x14
        );
        // 00d48f00: the stream object at +0x4 of 0x50 bytes.
        let buffer = e.mem.alloc(0x80);
        e.mem.write(buffer, b"not a packfile");
        e.register(ISTREAM_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            answer(a[0])
        });
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x50);
        e.call(0x00d4_8f00, &args![buffer, 0x80u32, 0u32, 0u32]);
        assert_eq!(calls_to(&mut e, ISTREAM_CONSTRUCTOR)[0][0], frame + 4);
        // 00d49240: the tagfile format goes into the stream's own slot (+0x14).
        e.register(TAGFILE_READER_DETECT_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 2);
            answer(a[0])
        });
        let source = stream(&mut e, &[0u8; 8], true, true);
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x18);
        e.call(0x00d4_9240, &args![source]);
        assert_eq!(
            calls_to(&mut e, TAGFILE_READER_DETECT_FORMAT)[0][0],
            frame + 0x14
        );
    }

    #[test]
    fn is_loadable_checks_each_format() {
        let mut e = engine();
        install_singletons(&mut e);
        install_router(&mut e);
        let current = text(&mut e, "Havok-7.1.0-r1");
        e.register_double(CURRENT_VERSION, move |_, _| answer(current));
        assert_eq!(e.call(0x00d4_9240, &args![0u32]).u32(), 0);
        let broken = stream(&mut e, &[], false, true);
        assert_eq!(e.call(0x00d4_9240, &args![broken]).u32(), 0);
        // A tagfile format (above 1) is loadable at once.
        let tag = stream(&mut e, &[], true, true);
        e.register(TAGFILE_READER_DETECT_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 2);
            answer(a[0])
        });
        assert_eq!(e.call(0x00d4_9240, &args![tag]).u32(), 1);
        e.register(TAGFILE_READER_DETECT_FORMAT, |e, a| {
            e.mem.set_u32(a[0], 1);
            answer(a[0])
        });
        // A current native packfile.
        let ok = stream(&mut e, &packfile_bytes("Havok-7.1.0-r1"), true, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_9240, &args![ok]).u32(), 1);
        assert!(calls_to(&mut e, DEPRECATED_SLOTS + 0x10).is_empty());
        // Another version, and the xml format, ask the deprecated loader (answer 0xd010: yes).
        let old = stream(&mut e, &packfile_bytes("Havok-5.0.0-r1"), true, true);
        assert_eq!(e.call(0x00d4_9240, &args![old]).u32(), 1);
        let xml = stream(&mut e, b"<hkpackfile>", true, true);
        assert_eq!(e.call(0x00d4_9240, &args![xml]).u32(), 1);
        assert_eq!(calls_to(&mut e, DEPRECATED_SLOTS + 0x10).len(), 2);
        // A refusal from the deprecated loader, and unknown data, are not loadable.
        e.register(DEPRECATED_SLOTS + 0x10, |_, _| answer(0));
        assert_eq!(e.call(0x00d4_9240, &args![xml]).u32(), 0);
        let nothing = stream(&mut e, b"zzz", true, true);
        assert_eq!(e.call(0x00d4_9240, &args![nothing]).u32(), 0);
        // A packfile of the wrong platform is not asked about.
        let mut wrong = packfile_bytes("Havok-7.1.0-r1");
        wrong[0x10..0x14].copy_from_slice(&[9, 9, 9, 9]);
        let wrong = stream(&mut e, &wrong, true, true);
        let before = calls_to(&mut e, DEPRECATED_SLOTS + 0x10).len();
        assert_eq!(e.call(0x00d4_9240, &args![wrong]).u32(), 0);
        assert_eq!(calls_to(&mut e, DEPRECATED_SLOTS + 0x10).len(), before);
    }

    #[test]
    fn string_ptr_equals_a_c_string_by_text() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        let abc = text(&mut e, "abc");
        let also_abc = text(&mut e, "abc");
        let other = text(&mut e, "abd");
        let ask =
            |e: &mut Engine, this: u32, text: u32| e.call(0x00d4_9350, &args![this, text]).bool();
        // A null string equals only a null text; the flag bit is not part of the text.
        assert!(ask(&mut e, this, 0));
        assert!(!ask(&mut e, this, abc));
        e.mem.set_u32(this, 1);
        assert!(ask(&mut e, this, 0));
        e.mem.set_u32(this, abc);
        assert!(!ask(&mut e, this, 0));
        assert!(ask(&mut e, this, also_abc));
        assert!(!ask(&mut e, this, other));
        e.mem.set_u32(this, abc | 1);
        assert!(ask(&mut e, this, also_abc));
    }

    #[test]
    fn layout_rules_compare_four_bytes() {
        let mut e = engine();
        let (a, b, out) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(4));
        e.mem.write(a, &[1, 2, 3, 4]);
        e.mem.write(b, &[1, 2, 3, 4]);
        assert_eq!(e.call(0x00d4_93a0, &args![a, out, b]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        for i in 0..4 {
            e.mem.write(b, &[1, 2, 3, 4]);
            e.mem.set_u8(b + i, 9);
            assert_eq!(e.call(0x00d4_93a0, &args![a, out, b]).u32(), out);
            assert_eq!(e.mem.u8(out), 0, "byte {i}");
        }
        // The fifth byte is not compared.
        e.mem.write(b, &[1, 2, 3, 4, 9]);
        e.call(0x00d4_93a0, &args![a, out, b]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn listener_wrap_object_wraps_in_the_world_or_makes_the_null_handle() {
        let mut e = engine();
        e.register(WORLD_WRAP_OBJECT, |_, a| answer(a[1] * 0x100 + a[2]));
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener + 0xc, 0x77);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d4_93e0, &args![listener, out, 5u32, 6u32]).u32(),
            out
        );
        assert_eq!(calls_to(&mut e, WORLD_WRAP_OBJECT), vec![vec![0x77, 5, 6]]);
        assert_eq!(
            calls_to(&mut e, DATA_OBJECT_CONSTRUCTOR),
            vec![vec![out, 0x506]]
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00d4_93e0, &args![listener, out, 0u32, 6u32]).u32(),
            out
        );
        assert!(calls_to(&mut e, WORLD_WRAP_OBJECT).is_empty());
        assert_eq!(
            calls_to(&mut e, DATA_OBJECT_CONSTRUCTOR),
            vec![vec![out, 0]]
        );
    }

    /// A listener whose forwarded listener's callback runs `callback` on the `(object cell,
    /// class cell)` pointers; the registry answers the class `0xc1a5` for every name, the data
    /// object's handle is `(0xabc, 0)` or `(0xabc, handle class)`.
    fn callback_setup(
        handle_class: u32,
        callback: fn(&mut Engine, &[u32]) -> Ret,
    ) -> (Engine, u32) {
        let mut e = engine();
        install_singletons(&mut e);
        let registry = object_with_slots(&mut e, &[(0x10, CLASS_REGISTRY_LOOKUP)]);
        e.register(CLASS_REGISTRY_LOOKUP, |_, _| answer(0xc1a5));
        e.register_double(REGISTRIES_SLOTS + 0x10, move |_, _| answer(registry));
        e.register(DATA_CLASS_GET_NAME, |_, _| answer(0x4e4e));
        e.register_double(DATA_OBJECT_GET_HANDLE, move |e, a| {
            e.mem.set_u32(a[1], 0xabc);
            e.mem.set_u32(a[1] + 4, handle_class);
            Ret::default()
        });
        e.register(LISTENER_CALLBACK, callback);
        let forwarded = object_with_slots(&mut e, &[(0xc, LISTENER_CALLBACK)]);
        let listener = e.mem.alloc(0x10);
        e.mem.set_u32(listener + 8, forwarded);
        e.mem.set_u32(listener + 0xc, 0x77);
        (e, listener)
    }

    #[test]
    fn add_data_object_callback_copies_the_input_when_the_listener_changes_nothing() {
        let (mut e, listener) = callback_setup(0, zero);
        e.register_double(LISTENER_CALLBACK, |e, a| {
            // The pair: object cell, then class cell (the registry's class).
            assert_eq!(a[2], a[1] + 4);
            assert_eq!(e.mem.u32(a[1]), 0xabc);
            assert_eq!(e.mem.u32(a[2]), 0xc1a5);
            Ret::default()
        });
        let (out, input) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        let frame = next_frame(&mut e, 0x2c);
        assert_eq!(e.call(0x00d4_9420, &args![listener, out, input]).u32(), out);
        assert_eq!(
            calls_to(&mut e, DATA_OBJECT_COPY_CONSTRUCTOR),
            vec![vec![out, input]]
        );
        // The class into `input`'s own argument slot (+0x28), the handle pair at +0x18.
        assert_eq!(calls_to(&mut e, DATA_OBJECT_GET_CLASS)[0][1], frame + 0x28);
        assert_eq!(calls_to(&mut e, DATA_OBJECT_GET_HANDLE)[0][1], frame + 0x18);
        assert_eq!(calls_to(&mut e, DATA_CLASS_DESTRUCTOR).len(), 1);
        assert!(calls_to(&mut e, WORLD_WRAP_OBJECT).is_empty());
    }

    #[test]
    fn add_data_object_callback_wraps_what_the_listener_replaced() {
        // The listener replaces the object (cell 0) with 0xbad.
        let (mut e, listener) = callback_setup(0, |e, a| {
            e.mem.set_u32(a[1], 0xbad);
            Ret::default()
        });
        e.register(WORLD_WRAP_OBJECT, |_, a| answer(a[1]));
        let (out, input) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00d4_9420, &args![listener, out, input]).u32(), out);
        assert_eq!(
            calls_to(&mut e, WORLD_WRAP_OBJECT),
            vec![vec![0x77, 0xbad, 0xc1a5]]
        );
        assert!(calls_to(&mut e, DATA_OBJECT_COPY_CONSTRUCTOR).is_empty());
        // A listener that replaces the class instead also counts; the handle's own class wins
        // over the registry's before the listener sees it.
        let (mut e, listener) = callback_setup(0xc1a6, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0xc1a6);
            e.mem.set_u32(a[2], 0xdd);
            Ret::default()
        });
        e.register(WORLD_WRAP_OBJECT, |_, a| answer(a[1]));
        let (out, input) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00d4_9420, &args![listener, out, input]);
        assert_eq!(
            calls_to(&mut e, WORLD_WRAP_OBJECT),
            vec![vec![0x77, 0xabc, 0xdd]]
        );
    }

    #[test]
    fn add_data_object_callback_copies_without_a_listener_or_for_a_null_object() {
        let (mut e, listener) = callback_setup(0, zero);
        let (out, input) = (e.mem.alloc(4), e.mem.alloc(4));
        e.mem.set_u32(listener + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00d4_9420, &args![listener, out, input]);
        assert_eq!(
            calls_to(&mut e, DATA_OBJECT_COPY_CONSTRUCTOR),
            vec![vec![out, input]]
        );
        assert!(calls_to(&mut e, DATA_OBJECT_IS_NULL).is_empty());
        // With a listener but a null input object: copied, and the listener is never asked.
        let (mut e, listener) = callback_setup(0, zero);
        e.register(DATA_OBJECT_IS_NULL, |_, _| answer(1));
        e.register_double(LISTENER_CALLBACK, |_, _| {
            panic!("the listener must not be asked")
        });
        let (out, input) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00d4_9420, &args![listener, out, input]);
        assert_eq!(
            calls_to(&mut e, DATA_OBJECT_COPY_CONSTRUCTOR),
            vec![vec![out, input]]
        );
    }
}
