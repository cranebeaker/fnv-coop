//! `(unplaced)/run_00c64c30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c6ba30`: the jump target of `00c657f0`.
const JUMP_TARGET_00C657F0: u32 = 0x00c6_ba30;

// Translated from 00c657f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00c6ba30`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00c657f0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00C657F0, &args![this, first, second]);
}

/// `00a7b3b0` (named `NiExtraData::LinkObject` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A7B3B0: u32 = 0x00a7_b3b0;

// Translated from 00c65160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable word 010c39bc (unit `run_00c64c30`). Forwards `this` and
/// the stream to `00a7b3b0` (`NiExtraData::LinkObject`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c65160(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A7B3B0, &args![this, stream]);
}

// Translated from 00c657e0 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 012043f0 without reading the receiver. The map gives this body no name; it takes only `this` (plain `RET`).
pub fn fn_00c657e0(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_43f0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_57e0, fn_00c657e0(Ptr) -> u32),
        entry!(0x00c6_57f0, fn_00c657f0(Ptr, Ptr, Ptr)),
        entry!(0x00c6_5160, fn_00c65160(Ptr, Ptr)),
        entry!(0x00c6_5530, fn_00c65530()),
        entry!(0x00c6_55d0, fn_00c655d0()),
        entry!(0x00c6_4c30, fn_00c64c30(Ptr, u32) -> bool),
        entry!(0x00c6_4c40, fn_00c64c40(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00c6_4d10, attach_point_pair_ctor(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00c6_4da0, attach_point_pair_set_first(Ptr, Ptr) -> u32),
        entry!(0x00c6_4dc0, attach_point_pair_set_second(Ptr, Ptr) -> u32),
        entry!(0x00c6_4de0, bs_attach_points_ctor(Ptr) -> Ptr),
        entry!(0x00c6_4e70, bs_attach_points_dtor(Ptr)),
        entry!(0x00c6_4fc0, fn_00c64fc0(Ptr, u32) -> Ptr),
        entry!(0x00c6_4fd0, bs_attach_points_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c6_5040, bs_attach_points_load_binary(Ptr, Ptr)),
        entry!(0x00c6_5180, bs_attach_points_register_streamables(Ptr, Ptr) -> u8),
        entry!(0x00c6_5210, bs_attach_points_save_binary(Ptr, Ptr)),
        entry!(0x00c6_5290, bs_attach_points_is_equal(Ptr, Ptr) -> u8),
        entry!(0x00c6_52c0, bs_attach_points_create_object() -> Ptr),
        entry!(0x00c6_5350, bs_attach_points_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c6_5490, bs_attach_points_add_pair(Ptr, Ptr, Ptr)),
        entry!(0x00c6_5650, bs_attach_points_deleting_dtor(Ptr, u32) -> Ptr),
        entry!(0x00c6_5680, attach_point_array_dtor(Ptr)),
        entry!(0x00c6_56b0, attach_point_pair_dtor(Ptr)),
        entry!(0x00c6_5710, attach_point_array_deleting_dtor(Ptr, u32) -> Ptr),
        entry!(0x00c6_5760, attach_point_array_ctor(Ptr, u16, u16) -> Ptr),
    ]
}

/// The `MemoryManager` singleton.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager` allocation (`00aa3e40`, thiscall on the singleton: size).
const ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager` deallocation (`00aa4060`, thiscall on the singleton: the block).
const DEALLOCATE: u32 = 0x00aa_4060;
/// The `NiFixedString` maker (`00a5b690`, cdecl: the text; returns the handle).
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// The empty `NiFixedString` handle.
const FIXED_STRING_EMPTY: u32 = 0x0109_b220;
/// `"BSAttachPoints"`.
const ATTACH_POINTS_NAME: u32 = 0x010c_3a18;
/// The global holding the allocated `NiFixedString`.
const ATTACH_POINTS_STRING: u32 = 0x0120_43ec;
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c65530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65530 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f306a5, state variable) is not translated
/// A start-up initializer: allocates a 4-byte `NiFixedString` from the memory manager and
/// sets it to `"BSAttachPoints"` (`00a5b690`); the compiler kept a test of the constant
/// text's address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) instead never runs (translated anyway). The string (or null when the
/// allocation failed) goes to `012043ec`.
pub fn fn_00c65530(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(ATTACH_POINTS_NAME);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![ATTACH_POINTS_NAME])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(ATTACH_POINTS_STRING, stored);
}

// Translated from 00c655d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the string at `012043ec` exists, releases its handle (unless
/// it is the empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the
/// text), frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always
/// true), and clears the global.
pub fn fn_00c655d0(e: &mut Engine) {
    let string = e.global::<u32>(ATTACH_POINTS_STRING);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(ATTACH_POINTS_STRING, 0);
}

// ---- n081: `BSAttachPoints` and its pair and array (Xbox PDB class names) ----

/// `NiAVObject::ms_RTTI` (`011f4280`, the record `NiAVObject::GetRTTI` `00a5a420` answers).
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
/// The lookup on the stream's object at +0x5c4 (`00853130`, unit
/// `bgssaveloadreferencesmap.cpp`; thiscall: name handle, out word).
const REFERENCES_MAP_LOOKUP: u32 = 0x0085_3130;
/// The pointer-array append (`00c4c8d0`, unit BSMain; thiscall: address of the element).
const ARRAY_ADD: u32 = 0x00c4_c8d0;
/// The pointer-array resize (`00c4c920`, unit BSMain; thiscall: new capacity).
const ARRAY_SET_SIZE: u32 = 0x00c4_c920;
/// The `NiFixedString` assignment (`0043ba10`, thiscall: the source string; returns `this`).
const FIXED_STRING_ASSIGN: u32 = 0x0043_ba10;
/// `NiExtraData`'s constructor taking a name (`00a7b440`, unit `niextradata.cpp`; thiscall:
/// the name's address).
const EXTRA_DATA_CTOR: u32 = 0x00a7_b440;
/// `NiExtraData`'s destructor body (`00a7b300`, unit `niextradata.cpp`).
const EXTRA_DATA_DTOR: u32 = 0x00a7_b300;
/// `NiExtraData`'s `CopyMembers` body (`00a7b330`; thiscall: destination, cloning process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
/// `NiExtraData::LoadBinary` (Xbox PDB, `00a7b4a0`).
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
/// `NiExtraData::RegisterStreamables` (Xbox PDB, `00a7b290`).
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a7_b290;
/// `NiExtraData::SaveBinary` (Xbox PDB, `00a7b2c0`).
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
/// `NiExtraData::IsEqual` (Xbox PDB, `00a7b3d0`).
const EXTRA_DATA_IS_EQUAL: u32 = 0x00a7_b3d0;
/// `NiExtraData::GetViewerStrings` (Xbox PDB, `00a7b580`).
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// The stream's 4-byte count reader (`00a26df0`, cdecl: stream, destination).
const STREAM_LOAD_U32: u32 = 0x00a2_6df0;
/// `NiStreamSaveBinary<unsigned int>` (Xbox PDB, `0044adf0`, cdecl: stream, source).
const STREAM_SAVE_U32: u32 = 0x0044_adf0;
/// `NiStream::LoadFixedString` (Xbox PDB, `00a64df0`; thiscall: destination).
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// `NiStream::SaveFixedString` (Xbox PDB, `00a641f0`; thiscall: the string's address).
const STREAM_SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiGetViewerString` (Xbox PDB, `00a7d330`, cdecl: the text).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// The viewer-string list append (`0096a610`; thiscall: address of the text pointer).
const VIEWER_LIST_ADD: u32 = 0x0096_a610;
/// The `sprintf_s`-style formatter (`00406d00`, cdecl: buffer, size, format, ...).
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// The pair line's format text (`010c3a0c`).
const PAIR_FORMAT: u32 = 0x010c_3a0c;
/// The word at `012043f0` (the address `fn_00c657e0` answers), passed to
/// `NiGetViewerString` as the class's line text.
const CLASS_NAME_WORD: u32 = 0x0120_43f0;
/// The object allocator `00aa13e0` (cdecl: size), through the heap at `011f6080`.
const OBJECT_ALLOCATE: u32 = 0x00aa_13e0;
/// The object deallocator `00aa1460` (cdecl: block, size).
const OBJECT_DEALLOCATE: u32 = 0x00aa_1460;
/// The array-buffer allocator `00aa1070` (cdecl: size).
const BUFFER_ALLOCATE: u32 = 0x00aa_1070;
/// The array-buffer release `00aa10f0` (cdecl: block; nothing for null).
const BUFFER_FREE: u32 = 0x00aa_10f0;
/// The object registration `00a5b950` (unit `niobjectnet.cpp`; thiscall: `011f49e0`).
const OBJECT_REGISTER: u32 = 0x00a5_b950;
/// The record `bs_attach_points_create_object` passes to `00a5b950`.
const CREATE_OBJECT_RECORD: u32 = 0x011f_49e0;
/// `BSAttachPoints`'s vtable.
const ATTACH_POINTS_VTABLE: u32 = 0x010c_396c;
/// The `NiTArray` vtable `00c65760` installs.
const ARRAY_BASE_VTABLE: u32 = 0x010c_3a08;
/// The `NiTPrimitiveArray<BSAttachPointPair *>` vtable the constructor installs after it.
const ARRAY_VTABLE: u32 = 0x010c_3960;

/// `BSAttachPoints` (Xbox PDB, 0x1c bytes): an `NiExtraData` (name at +8) with the
/// `NiTPrimitiveArray<BSAttachPointPair *>` `pAttachPointsA` at +0xc.
const ATTACH_POINTS_ARRAY: u32 = 0x0c;

/// Releases one `NiFixedString` handle the way the inlined destructor does: nothing for the
/// empty string, else `InterlockedDecrement` of the count 8 bytes before the text.
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Element `index` of the `NiTArray` at `array` (`m_pBase[index]`).
fn array_element(e: &Engine, array: u32, index: u32) -> u32 {
    let base = e.mem.u32(array + 4);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00c64c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word `010c1450`): answers false (`XOR AL,AL`) without
/// reading its argument (`RET 4`).
pub fn fn_00c64c30(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> bool {
    false
}

// Translated from 00c64c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; its one caller is `BSStream::RegisterSaveObject` (`00c3b14d`),
/// with `this` the stream's object at +0x5c8. When `object` is an `NiAVObject` (its RTTI
/// chain, vtable +8 then each record's base at +4, reaches `NiAVObject::ms_RTTI`), takes
/// the word at +0x18 of it and, when that is not null, the handle at +8 of that; when the
/// stream's object at +0x5c4 and the handle are both set, looks the handle up there
/// (`00853130`, out word starting at 0). A found entry appends `object` to the array at
/// `this + 8` (`00c4c8d0`, by address) and answers true; every other path answers false.
pub fn fn_00c64c40(e: &mut Engine, this: Ptr, stream: Ptr, object: Ptr) -> bool {
    let object = object.addr();
    let candidate = if object == 0 {
        0
    } else {
        let mut rtti = e.vcall(object, 0x8, &args![]).u32();
        let is_kind = loop {
            if rtti == 0 {
                break false;
            }
            if rtti == NI_AV_OBJECT_RTTI {
                break true;
            }
            rtti = e.mem.u32(rtti + 4);
        };
        if is_kind {
            object
        } else {
            0
        }
    };
    if candidate == 0 {
        return false;
    }
    let owner = e.mem.u32(candidate + 0x18);
    if owner == 0 {
        return false;
    }
    let handle = e.mem.u32(owner + 8);
    e.with_stack(8, |e, locals| {
        // +0 the cast object (passed by address), +4 the lookup's out word.
        let cast = locals.addr();
        let found = cast + 4;
        e.mem.set_u32(cast, candidate);
        e.mem.set_u32(found, 0);
        let map = e.mem.u32(stream.addr() + 0x5c4);
        if map != 0 && handle != 0 {
            e.call(REFERENCES_MAP_LOOKUP, &args![map, handle, found]);
        }
        if e.mem.u32(found) != 0 {
            e.call(ARRAY_ADD, &args![this.addr() + 8, cast]);
            true
        } else {
            false
        }
    })
}

// Translated from 00c64d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64d10 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f305b3, state variable) is not translated
/// `BSAttachPointPair`'s constructor (class from the Xbox PDB: two `NiFixedString`s, 8 bytes;
/// unnamed in the engine map): both strings start as the empty handle (`0109b220`), then the
/// first is assigned from `first` (`00c64da0`) and the second from `second` (`00c64dc0`).
/// Returns `this`.
pub fn attach_point_pair_ctor(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    let empty = e.global::<u32>(FIXED_STRING_EMPTY);
    e.mem.set_u32(this.addr(), empty);
    let empty = e.global::<u32>(FIXED_STRING_EMPTY);
    e.mem.set_u32(this.addr() + 4, empty);
    attach_point_pair_set_first(e, this, first);
    attach_point_pair_set_second(e, this, second);
    this
}

// Translated from 00c64da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns the pair's first string (+0) from `source` through
/// the `NiFixedString` assignment `0043ba10`, passing on its result.
pub fn attach_point_pair_set_first(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    e.call(FIXED_STRING_ASSIGN, &args![this, source]).u32()
}

// Translated from 00c64dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: assigns the pair's second string (+4) from `source` through
/// `0043ba10`, passing on its result.
pub fn attach_point_pair_set_second(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    e.call(FIXED_STRING_ASSIGN, &args![this.addr() + 4, source])
        .u32()
}

// Translated from 00c64de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64de0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f305d6, state variable) is not translated
/// `BSAttachPoints`'s constructor (unnamed in the engine map): `NiExtraData`'s constructor
/// with the word at `012043ec` (the address of the class's name string, `fn_00c65530`;
/// `00a7b440`), the class's vtable `010c396c`, then the array at +0xc built with capacity 0
/// and grow-by 1 (`00c65760`) and given the vtable `010c3960`. Returns `this`.
pub fn bs_attach_points_ctor(e: &mut Engine, this: Ptr) -> Ptr {
    let name = e.global::<u32>(ATTACH_POINTS_STRING);
    e.call(EXTRA_DATA_CTOR, &args![this, name]);
    e.mem.set_u32(this.addr(), ATTACH_POINTS_VTABLE);
    let array = this.addr() + ATTACH_POINTS_ARRAY;
    attach_point_array_ctor(e, Ptr::new(array), 0, 1);
    e.mem.set_u32(array, ARRAY_VTABLE);
    this
}

// Translated from 00c64e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c64e70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30604, state variable) is not translated
/// `BSAttachPoints`'s destructor (unnamed in the engine map): sets the class's vtable, then
/// for each of the array's first `m_usESize` (+0x18, read once) slots destroys a non-null
/// pair (`00c656b0`) and frees its 8 bytes through the memory manager (behind a folded
/// `1 & 1` test, always true); then the inlined array clean-up: every slot below
/// `m_usSize` (+0x16) cleared, both counts zeroed, the `NiTArray` vtable `010c3a08`, its
/// buffer released (`00aa10f0`); last `NiExtraData`'s destructor (`00a7b300`).
pub fn bs_attach_points_dtor(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, ATTACH_POINTS_VTABLE);
    let array = this + ATTACH_POINTS_ARRAY;
    let count = u32::from(e.mem.u16(this + 0x18));
    for index in 0..count {
        let pair = array_element(e, array, index);
        if pair != 0 {
            attach_point_pair_dtor(e, Ptr::new(pair));
            if std::hint::black_box(1u32) & 1 != 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, pair]);
            }
        }
    }
    let mut index: u16 = 0;
    while index < e.mem.u16(array + 0xa) {
        let base = e.mem.u32(array + 4);
        e.mem.set_u32(base.wrapping_add(u32::from(index) * 4), 0);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(array + 0xa, 0);
    e.mem.set_u16(array + 0xc, 0);
    e.mem.set_u32(array, ARRAY_BASE_VTABLE);
    let base = e.mem.u32(array + 4);
    e.call(BUFFER_FREE, &args![base]);
    e.call(EXTRA_DATA_DTOR, &args![this]);
}

// Translated from 00c64fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word `010c39b4`): answers `this` without reading its
/// argument (`RET 4`).
pub fn fn_00c64fc0(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 00c64fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPoints::CopyMembers` (vtable +0x94 in the Xbox PDB; unnamed in the engine map):
/// `NiExtraData`'s part (`00a7b330` with both words), then for each of the first
/// `m_usESize` (+0x18, read once) pairs adds a copy to `destination` (`00c65490`, the
/// pair's two strings by address).
pub fn bs_attach_points_copy_members(e: &mut Engine, this: Ptr, destination: Ptr, process: Ptr) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, destination, process]);
    let count = u32::from(e.mem.u16(this.addr() + 0x18));
    for index in 0..count {
        let pair = array_element(e, this.addr() + ATTACH_POINTS_ARRAY, index);
        bs_attach_points_add_pair(
            e,
            destination,
            Ptr::new(pair),
            Ptr::new(pair.wrapping_add(4)),
        );
    }
}

// Translated from 00c65040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65040 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3062f, state variable) is not translated
/// `BSAttachPoints::LoadBinary` (unnamed in the engine map): `NiExtraData::LoadBinary`, a
/// 4-byte count from the stream into a local starting at 0 (`00a26df0`), the array sized
/// to it (`00c4c920`), then per entry (the bound read from that local each pass) two
/// strings read (`NiStream::LoadFixedString`, each local starting as the empty handle), a
/// pair added from them (`00c65490`) and both temporaries released (second, then first).
pub fn bs_attach_points_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    e.with_stack(12, |e, locals| {
        // +0 the count, +4 the first string, +8 the second.
        let count = locals.addr();
        let first = count + 4;
        let second = count + 8;
        e.mem.set_u32(count, 0);
        e.call(STREAM_LOAD_U32, &args![stream, count]);
        let total = e.mem.u32(count);
        e.call(
            ARRAY_SET_SIZE,
            &args![this.addr() + ATTACH_POINTS_ARRAY, total],
        );
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            let empty = e.global::<u32>(FIXED_STRING_EMPTY);
            e.mem.set_u32(first, empty);
            let empty = e.global::<u32>(FIXED_STRING_EMPTY);
            e.mem.set_u32(second, empty);
            e.call(STREAM_LOAD_FIXED_STRING, &args![stream, first]);
            e.call(STREAM_LOAD_FIXED_STRING, &args![stream, second]);
            bs_attach_points_add_pair(e, this, Ptr::new(first), Ptr::new(second));
            let handle = e.mem.u32(second);
            release_fixed_string(e, handle);
            let handle = e.mem.u32(first);
            release_fixed_string(e, handle);
            index += 1;
        }
    });
}

// Translated from 00c65180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPoints::RegisterStreamables` (unnamed in the engine map): `NiExtraData`'s part
/// first; when it answers true, registers both strings of each of the first `m_usESize`
/// (+0x18, read once) pairs through the stream's vtable slot +0x20 (by address). Passes on
/// the base's byte answer.
pub fn bs_attach_points_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> u8 {
    let registered = e
        .call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u8();
    if registered != 0 {
        let count = u32::from(e.mem.u16(this.addr() + 0x18));
        for index in 0..count {
            let pair = array_element(e, this.addr() + ATTACH_POINTS_ARRAY, index);
            e.vcall(stream.addr(), 0x20, &args![pair]);
            e.vcall(stream.addr(), 0x20, &args![pair.wrapping_add(4)]);
        }
    }
    registered
}

// Translated from 00c65210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPoints::SaveBinary` (unnamed in the engine map): `NiExtraData::SaveBinary`, the
/// count `m_usESize` (+0x18) written as 4 bytes from a local (`0044adf0`), then both strings
/// of each pair (`NiStream::SaveFixedString`, by address); the loop bound is that local,
/// read again each pass.
pub fn bs_attach_points_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        let total = u32::from(e.mem.u16(this.addr() + 0x18));
        e.mem.set_u32(count, total);
        e.call(STREAM_SAVE_U32, &args![stream, count]);
        let mut index = 0u32;
        while index < e.mem.u32(count) {
            let pair = array_element(e, this.addr() + ATTACH_POINTS_ARRAY, index);
            e.call(STREAM_SAVE_FIXED_STRING, &args![stream, pair]);
            e.call(
                STREAM_SAVE_FIXED_STRING,
                &args![stream, pair.wrapping_add(4)],
            );
            index += 1;
        }
    });
}

// Translated from 00c65290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPoints::IsEqual` (unnamed in the engine map): passes on `NiExtraData::IsEqual`'s
/// byte answer.
pub fn bs_attach_points_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u8 {
    e.call(EXTRA_DATA_IS_EQUAL, &args![this, other]).u8()
}

// Translated from 00c652c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c652c0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f30659, state variable) is not translated
/// The class's create function (unnamed in the engine map): 0x1c bytes from `00aa13e0`,
/// constructed (`00c64de0`) unless the allocation failed, then `00a5b950` on the result
/// (null included) with `011f49e0`. Returns the object.
pub fn bs_attach_points_create_object(e: &mut Engine) -> Ptr {
    let block = e.call(OBJECT_ALLOCATE, &args![0x1cu32]).u32();
    let object = if block != 0 {
        bs_attach_points_ctor(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.call(OBJECT_REGISTER, &args![object, CREATE_OBJECT_RECORD]);
    Ptr::new(object)
}

// Translated from 00c65350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65350 - the /GS cookie check (00ec408c on the cookie at 011c16bc XOR the frame) is not translated
/// `BSAttachPoints::GetViewerStrings` (unnamed in the engine map): `NiExtraData`'s part, the
/// class's line (`NiGetViewerString` of the word at `012043f0`) added to `list` (`0096a610`,
/// by address), then per pair (the first `m_usESize`, read once) one line formatted into a
/// 0x104-byte local buffer (`00406d00`: format `010c3a0c`, the index, both string handles)
/// and the buffer's address added the same way.
pub fn bs_attach_points_get_viewer_strings(e: &mut Engine, this: Ptr, list: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, list]);
    e.with_stack(0x10c, |e, locals| {
        // +0 the text pointer handed to the list, +8 the line buffer.
        let text = locals.addr();
        let buffer = text + 8;
        let class_name = e.global::<u32>(CLASS_NAME_WORD);
        let line = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(text, line);
        e.call(VIEWER_LIST_ADD, &args![list, text]);
        let count = u32::from(e.mem.u16(this.addr() + 0x18));
        for index in 0..count {
            let pair = array_element(e, this.addr() + ATTACH_POINTS_ARRAY, index);
            let first = e.mem.u32(pair);
            let second = e.mem.u32(pair.wrapping_add(4));
            e.call(
                FORMAT_TEXT,
                &args![buffer, 0x104u32, PAIR_FORMAT, index, first, second],
            );
            e.mem.set_u32(text, buffer);
            e.call(VIEWER_LIST_ADD, &args![list, text]);
        }
    });
}

// Translated from 00c65490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c65490 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f3067f, state variable) is not translated
/// Unnamed in the engine map: adds a pair to the array at +0xc: 8 bytes from the memory
/// manager, constructed from `first` and `second` (`00c64d10`) unless the allocation
/// failed (then null), appended by address (`00c4c8d0`).
pub fn bs_attach_points_add_pair(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 8u32]).u32();
    let pair = if block != 0 {
        attach_point_pair_ctor(e, Ptr::new(block), first, second).addr()
    } else {
        0
    };
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), pair);
        e.call(ARRAY_ADD, &args![this.addr() + ATTACH_POINTS_ARRAY, local]);
    });
}

// Translated from 00c65650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPoints`'s deleting destructor (unnamed in the engine map): the destructor
/// (`00c64e70`), then with bit 0 of `flags` its 0x1c bytes freed (`00aa1460`). Returns `this`.
pub fn bs_attach_points_deleting_dtor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    bs_attach_points_dtor(e, this);
    if flags & 1 != 0 {
        e.call(OBJECT_DEALLOCATE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00c65680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTPrimitiveArray<BSAttachPointPair *>` destructor (unnamed in the engine map): the
/// `NiTArray` vtable `010c3a08`, then the buffer (+4) released (`00aa10f0`).
pub fn attach_point_array_dtor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_BASE_VTABLE);
    let base = e.mem.u32(this.addr() + 4);
    e.call(BUFFER_FREE, &args![base]);
}

// Translated from 00c656b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAttachPointPair`'s destructor (unnamed in the engine map): releases the second string
/// (+4), then the first (+0); the empty handle `0109b220` is left alone, any other's count
/// (8 bytes before the text) goes through `InterlockedDecrement`.
pub fn attach_point_pair_dtor(e: &mut Engine, this: Ptr) {
    let second = e.mem.u32(this.addr() + 4);
    release_fixed_string(e, second);
    let first = e.mem.u32(this.addr());
    release_fixed_string(e, first);
}

// Translated from 00c65710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray` deleting destructor (unnamed in the engine map): the vtable `010c3a08`,
/// the buffer released (`00aa10f0`), then with bit 0 of `flags` the object freed through
/// the memory manager. Returns `this`.
pub fn attach_point_array_deleting_dtor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_BASE_VTABLE);
    let base = e.mem.u32(this.addr() + 4);
    e.call(BUFFER_FREE, &args![base]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c65760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray` constructor (unnamed in the engine map): vtable `010c3a08`, `m_usMaxSize`
/// and `m_usGrowBy` from the two words' low halves, both counts 0, and a buffer of
/// `max_size * 4` bytes (`00aa1070`) when the capacity is not 0, else null. Returns `this`.
pub fn attach_point_array_ctor(e: &mut Engine, this: Ptr, max_size: u16, grow_by: u16) -> Ptr {
    let this = this.addr();
    e.mem.set_u32(this, ARRAY_BASE_VTABLE);
    e.mem.set_u16(this + 8, max_size);
    e.mem.set_u16(this + 0xe, grow_by);
    e.mem.set_u16(this + 0xa, 0);
    e.mem.set_u16(this + 0xc, 0);
    let capacity = u32::from(e.mem.u16(this + 8));
    let base = if capacity > 0 {
        e.call(BUFFER_ALLOCATE, &args![capacity << 2]).u32()
    } else {
        0
    };
    e.mem.set_u32(this + 4, base);
    Ptr::new(this)
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn log_of(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != top)
            .collect()
    }

    #[test]
    fn initializer_makes_the_string_or_stores_null() {
        for allocated in [false, true] {
            let mut e = Engine::new();
            e.map(ATTACH_POINTS_STRING, 4);
            e.mem.set_u32(ATTACH_POINTS_STRING, 0xffff);
            let block = if allocated { e.mem.alloc(4) } else { 0 };
            e.register_double(ALLOCATE, move |_, _| block.into_ret());
            e.register(FIXED_STRING_CREATE, |_, _| 0x7777u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x00c6_5530, &args![]);
            let mut want = vec![(ALLOCATE, vec![MEMORY_MANAGER, 4])];
            if block != 0 {
                want.push((FIXED_STRING_CREATE, vec![ATTACH_POINTS_NAME]));
                assert_eq!(e.mem.u32(block), 0x7777);
            }
            assert_eq!(log_of(&mut e, 0x00c6_5530), want);
            assert_eq!(e.mem.u32(ATTACH_POINTS_STRING), block);
        }
    }

    #[test]
    fn destructor_releases_the_handle_and_frees_the_string() {
        for (handle, released) in [(0x8008u32, true), (0x4444, false)] {
            let mut e = Engine::new();
            e.map(ATTACH_POINTS_STRING, 4);
            e.map(FIXED_STRING_EMPTY, 4);
            e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
            let string = e.mem.alloc(4);
            e.mem.set_u32(string, handle);
            e.mem.set_u32(ATTACH_POINTS_STRING, string);
            e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x00c6_55d0, &args![]);
            let mut want = vec![];
            if released {
                want.push((INTERLOCKED_DECREMENT, vec![0x8000]));
            }
            want.push((DEALLOCATE, vec![MEMORY_MANAGER, string]));
            assert_eq!(log_of(&mut e, 0x00c6_55d0), want);
            assert_eq!(e.mem.u32(ATTACH_POINTS_STRING), 0);
        }
        // No string: only the global is cleared.
        let mut e = Engine::new();
        e.map(ATTACH_POINTS_STRING, 4);
        e.call_log = Some(vec![]);
        e.call(0x00c6_55d0, &args![]);
        assert!(log_of(&mut e, 0x00c6_55d0).is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c657f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_57f0;
        const TARGET: u32 = 0x00c6_ba30;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }

    #[test]
    fn forwarder_fn_00c65160_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A7B3B0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5160, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A7B3B0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c657e0_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_57e0, &args![0x1000u32]).u32(), 0x0120_43f0);
        assert_eq!(e.call(0x00c6_57e0, &args![0u32]).u32(), 0x0120_43f0);
    }
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(e: &Engine, addr: u32) -> Log {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .cloned()
            .collect()
    }

    fn empty_string(e: &mut Engine, handle: u32) {
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, handle);
    }

    /// An attach-points object whose array holds `pairs` (count +0x18 = their number).
    fn attach_points(e: &mut Engine, pairs: &[u32]) -> u32 {
        let this = e.mem.alloc(0x1c);
        let base = e.mem.alloc(4 * pairs.len().max(1) as u32);
        for (i, p) in pairs.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *p);
        }
        e.mem.set_u32(this + 0x10, base);
        e.mem.set_u16(this + 0x16, pairs.len() as u16);
        e.mem.set_u16(this + 0x18, pairs.len() as u16);
        this
    }

    #[test]
    fn fn_00c64c30_answers_false() {
        let mut e = Engine::new();
        assert!(!e.call(0x00c6_4c30, &args![0x1000u32, 5u32]).bool());
    }

    #[test]
    fn fn_00c64fc0_answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_4fc0, &args![0x1234u32, 5u32]).u32(), 0x1234);
    }

    #[test]
    fn fn_00c64c40_adds_av_objects_found_in_the_map() {
        // (rtti chain reaches NiAVObject, map present, lookup finds) -> added
        for (kind, map_set, finds, added) in [
            (true, true, true, true),
            (true, true, false, false),
            (true, false, true, false),
            (false, true, true, false),
        ] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x10);
            let leaf = e.mem.alloc(8);
            let root = e.mem.alloc(8);
            e.mem.set_u32(leaf + 4, root);
            e.mem
                .set_u32(root + 4, if kind { NI_AV_OBJECT_RTTI } else { 0 });
            e.register_double(0x0077_0000, move |_, _| leaf.into_ret());
            e.mem.set_u32(vtable + 8, 0x0077_0000);
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object, vtable);
            let owner = e.mem.alloc(0x10);
            e.mem.set_u32(owner + 8, 0x5555);
            e.mem.set_u32(object + 0x18, owner);
            let stream = e.mem.alloc(0x600);
            if map_set {
                e.mem.set_u32(stream + 0x5c4, 0x9000);
            }
            e.register_double(REFERENCES_MAP_LOOKUP, move |e, a| {
                if finds {
                    e.mem.set_u32(a[2], 1);
                }
                Ret::default()
            });
            e.register_double(ARRAY_ADD, move |e, a| {
                assert_eq!(e.mem.u32(a[1]), object);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            let r = e
                .call(0x00c6_4c40, &args![0x4000u32, stream, object])
                .bool();
            assert_eq!(r, added);
            let lookups = calls_to(&e, REFERENCES_MAP_LOOKUP);
            assert_eq!(lookups.len(), usize::from(kind && map_set));
            if let Some(l) = lookups.first() {
                assert_eq!(&l.1[..2], &[0x9000, 0x5555]);
            }
            let adds = calls_to(&e, ARRAY_ADD);
            assert_eq!(adds.len(), usize::from(added));
            if let Some(a) = adds.first() {
                assert_eq!(a.1[0], 0x4008);
            }
        }
        // A null object answers false without calls.
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c6_4c40, &args![0x4000u32, 0u32, 0u32]).bool());
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn pair_ctor_sets_empty_strings_then_assigns_both() {
        let mut e = Engine::new();
        empty_string(&mut e, 0x4444);
        e.register_double(FIXED_STRING_ASSIGN, |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x4444);
            a[0].into_ret()
        });
        let pair = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let r = e.call(0x00c6_4d10, &args![pair, 0x100u32, 0x200u32]).u32();
        assert_eq!(r, pair);
        assert_eq!(
            calls_to(&e, FIXED_STRING_ASSIGN),
            vec![
                (FIXED_STRING_ASSIGN, vec![pair, 0x100]),
                (FIXED_STRING_ASSIGN, vec![pair + 4, 0x200])
            ]
        );
    }

    #[test]
    fn pair_setters_pass_on_the_assignment_result() {
        let mut e = Engine::new();
        e.register(FIXED_STRING_ASSIGN, |_, a| (a[0] ^ a[1]).into_ret());
        assert_eq!(e.call(0x00c6_4da0, &args![0x10u32, 0x3u32]).u32(), 0x13);
        assert_eq!(e.call(0x00c6_4dc0, &args![0x10u32, 0x3u32]).u32(), 0x17);
    }

    #[test]
    fn ctor_builds_extra_data_and_the_array() {
        let mut e = Engine::new();
        e.map(ATTACH_POINTS_STRING, 4);
        e.mem.set_u32(ATTACH_POINTS_STRING, 0x6000);
        e.register(EXTRA_DATA_CTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x10, 0xdead);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_4de0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_CTOR),
            vec![(EXTRA_DATA_CTOR, vec![this, 0x6000])]
        );
        assert!(calls_to(&e, BUFFER_ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this), ATTACH_POINTS_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 0x14), 0);
        assert_eq!(e.mem.u16(this + 0x1a), 1);
    }

    #[test]
    fn array_ctor_allocates_only_for_a_capacity() {
        for max in [0u16, 3] {
            let mut e = Engine::new();
            e.register(BUFFER_ALLOCATE, |_, a| (0x8000 + a[0]).into_ret());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 4, 0xdead);
            e.mem.set_u16(this + 0xa, 9);
            e.mem.set_u16(this + 0xc, 9);
            e.call_log = Some(vec![]);
            let r = e.call(
                0x00c6_5760,
                &args![this, 0x0001_0000 | u32::from(max), 2u32],
            );
            assert_eq!(r.u32(), this);
            assert_eq!(e.mem.u32(this), ARRAY_BASE_VTABLE);
            assert_eq!(e.mem.u16(this + 8), max);
            assert_eq!(e.mem.u16(this + 0xa), 0);
            assert_eq!(e.mem.u16(this + 0xc), 0);
            assert_eq!(e.mem.u16(this + 0xe), 2);
            let want = if max == 0 { 0 } else { 0x800c };
            assert_eq!(e.mem.u32(this + 4), want);
            assert_eq!(calls_to(&e, BUFFER_ALLOCATE).len(), usize::from(max != 0));
        }
    }

    #[test]
    fn dtor_destroys_pairs_and_empties_the_array() {
        let mut e = Engine::new();
        empty_string(&mut e, 0x4444);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x4444);
        e.mem.set_u32(pair + 4, 0x7008);
        let this = attach_points(&mut e, &[pair, 0]);
        let base = e.mem.u32(this + 0x10);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.register(BUFFER_FREE, |_, _| Ret::default());
        e.register(EXTRA_DATA_DTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_4e70, &args![this]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![(INTERLOCKED_DECREMENT, vec![0x7000])]
        );
        assert_eq!(
            calls_to(&e, DEALLOCATE),
            vec![(DEALLOCATE, vec![MEMORY_MANAGER, pair])]
        );
        assert_eq!(calls_to(&e, BUFFER_FREE), vec![(BUFFER_FREE, vec![base])]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_DTOR),
            vec![(EXTRA_DATA_DTOR, vec![this])]
        );
        assert_eq!(e.mem.u32(base), 0);
        assert_eq!(e.mem.u16(this + 0x16), 0);
        assert_eq!(e.mem.u16(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0xc), ARRAY_BASE_VTABLE);
        assert_eq!(e.mem.u32(this), ATTACH_POINTS_VTABLE);
    }

    #[test]
    fn copy_members_adds_each_pair_to_the_destination() {
        let mut e = Engine::new();
        let this = attach_points(&mut e, &[0x3000, 0x3100]);
        let destination = attach_points(&mut e, &[]);
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(ARRAY_ADD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_4fd0, &args![this, destination, 0x77u32]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![(EXTRA_DATA_COPY_MEMBERS, vec![this, destination, 0x77])]
        );
        let adds = calls_to(&e, ARRAY_ADD);
        assert_eq!(adds.len(), 2);
        assert!(adds.iter().all(|a| a.1[0] == destination + 0xc));
    }

    #[test]
    fn add_pair_constructs_or_appends_null() {
        for allocated in [false, true] {
            let mut e = Engine::new();
            empty_string(&mut e, 0x4444);
            let block = if allocated { e.mem.alloc(8) } else { 0 };
            e.register_double(ALLOCATE, move |_, _| block.into_ret());
            e.register(FIXED_STRING_ASSIGN, |_, a| a[0].into_ret());
            e.register_double(ARRAY_ADD, move |e, a| {
                assert_eq!(e.mem.u32(a[1]), block);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00c6_5490, &args![0x5000u32, 0x10u32, 0x20u32]);
            assert_eq!(
                calls_to(&e, ALLOCATE),
                vec![(ALLOCATE, vec![MEMORY_MANAGER, 8])]
            );
            assert_eq!(
                calls_to(&e, FIXED_STRING_ASSIGN).len(),
                if allocated { 2 } else { 0 }
            );
            assert_eq!(calls_to(&e, ARRAY_ADD)[0].1[0], 0x500c);
        }
    }

    #[test]
    fn load_binary_reads_count_and_pairs() {
        let mut e = Engine::new();
        empty_string(&mut e, 0x4444);
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_LOAD_U32, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], 2);
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STREAM_LOAD_FIXED_STRING, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x4444);
            e.mem.set_u32(a[1], 0x7008);
            Ret::default()
        });
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.register(ARRAY_ADD, |_, _| Ret::default());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5040, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            calls_to(&e, ARRAY_SET_SIZE),
            vec![(ARRAY_SET_SIZE, vec![0x500c, 2])]
        );
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING).len(), 4);
        assert_eq!(calls_to(&e, ALLOCATE).len(), 2);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 4);
    }

    #[test]
    fn register_streamables_registers_strings_only_when_the_base_does() {
        for base in [0u8, 1] {
            let mut e = Engine::new();
            let this = attach_points(&mut e, &[0x3000]);
            e.register_double(EXTRA_DATA_REGISTER_STREAMABLES, move |_, _| {
                (0x100 | u32::from(base)).into_ret()
            });
            e.register(0x0078_0000, |_, _| Ret::default());
            let vtable = e.mem.alloc(0x24);
            e.mem.set_u32(vtable + 0x20, 0x0078_0000);
            let stream = e.mem.alloc(4);
            e.mem.set_u32(stream, vtable);
            e.call_log = Some(vec![]);
            let r = e.call(0x00c6_5180, &args![this, stream]).u32();
            assert_eq!(r, u32::from(base));
            let regs = calls_to(&e, 0x0078_0000);
            if base == 0 {
                assert!(regs.is_empty());
            } else {
                assert_eq!(
                    regs,
                    vec![
                        (0x0078_0000, vec![stream, 0x3000]),
                        (0x0078_0000, vec![stream, 0x3004])
                    ]
                );
            }
        }
    }

    #[test]
    fn save_binary_writes_count_then_strings() {
        let mut e = Engine::new();
        let this = attach_points(&mut e, &[0x3000, 0x3100]);
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_U32, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 2);
            Ret::default()
        });
        e.register(STREAM_SAVE_FIXED_STRING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5210, &args![this, 0x6000u32]);
        let saved: Vec<u32> = calls_to(&e, STREAM_SAVE_FIXED_STRING)
            .iter()
            .map(|c| c.1[1])
            .collect();
        assert_eq!(saved, vec![0x3000, 0x3004, 0x3100, 0x3104]);
    }

    #[test]
    fn is_equal_passes_on_the_base_byte() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_IS_EQUAL, |_, _| 0x1202u32.into_ret());
        assert_eq!(e.call(0x00c6_5290, &args![1u32, 2u32]).u32(), 2);
    }

    #[test]
    fn create_object_registers_even_a_failed_allocation() {
        let mut e = Engine::new();
        e.register(OBJECT_ALLOCATE, |_, _| 0u32.into_ret());
        e.register(OBJECT_REGISTER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c6_52c0, &args![]).u32(), 0);
        assert_eq!(
            calls_to(&e, OBJECT_REGISTER),
            vec![(OBJECT_REGISTER, vec![0, CREATE_OBJECT_RECORD])]
        );
        assert_eq!(calls_to(&e, OBJECT_ALLOCATE)[0].1, vec![0x1c]);

        let mut e = Engine::new();
        let block = e.mem.alloc(0x1c);
        e.map(ATTACH_POINTS_STRING, 4);
        e.register_double(OBJECT_ALLOCATE, move |_, _| block.into_ret());
        e.register(OBJECT_REGISTER, |_, _| Ret::default());
        e.register(EXTRA_DATA_CTOR, |_, _| Ret::default());
        assert_eq!(e.call(0x00c6_52c0, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), ATTACH_POINTS_VTABLE);
    }

    #[test]
    fn viewer_strings_add_the_class_line_and_one_per_pair() {
        let mut e = Engine::new();
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x11);
        e.mem.set_u32(pair + 4, 0x22);
        let this = attach_points(&mut e, &[pair]);
        e.map(CLASS_NAME_WORD, 4);
        e.mem.set_u32(CLASS_NAME_WORD, 0x0099_0000);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x0098_0000u32.into_ret());
        e.register(FORMAT_TEXT, |_, _| Ret::default());
        e.register_double(VIEWER_LIST_ADD, |e, a| e.mem.u32(a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c6_5350, &args![this, 0x6000u32]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING),
            vec![(GET_VIEWER_STRING, vec![0x0099_0000])]
        );
        let format = calls_to(&e, FORMAT_TEXT);
        assert_eq!(format.len(), 1);
        assert_eq!(&format[0].1[1..], &[0x104, PAIR_FORMAT, 0, 0x11, 0x22]);
        assert_eq!(calls_to(&e, VIEWER_LIST_ADD).len(), 2);
    }

    #[test]
    fn deleting_dtor_frees_with_bit_0() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            let this = attach_points(&mut e, &[]);
            e.register(BUFFER_FREE, |_, _| Ret::default());
            e.register(EXTRA_DATA_DTOR, |_, _| Ret::default());
            e.register(OBJECT_DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c6_5650, &args![this, flags]).u32(), this);
            let frees = calls_to(&e, OBJECT_DEALLOCATE);
            if flags == 1 {
                assert_eq!(frees, vec![(OBJECT_DEALLOCATE, vec![this, 0x1c])]);
            } else {
                assert!(frees.is_empty());
            }
        }
    }

    #[test]
    fn array_dtors_release_the_buffer() {
        let mut e = Engine::new();
        e.register(BUFFER_FREE, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x00c6_5680, &args![this]);
        assert_eq!(e.mem.u32(this), ARRAY_BASE_VTABLE);
        assert_eq!(calls_to(&e, BUFFER_FREE), vec![(BUFFER_FREE, vec![0x7000])]);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c6_5710, &args![this, flags]).u32(), this);
            assert_eq!(calls_to(&e, BUFFER_FREE).len(), 1);
            assert_eq!(calls_to(&e, DEALLOCATE).len(), flags as usize);
        }
    }

    #[test]
    fn pair_dtor_releases_second_then_first() {
        let mut e = Engine::new();
        empty_string(&mut e, 0x4444);
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, 0x1008);
        e.mem.set_u32(pair + 4, 0x2008);
        e.call_log = Some(vec![]);
        e.call(0x00c6_56b0, &args![pair]);
        assert_eq!(
            calls_to(&e, INTERLOCKED_DECREMENT),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x2000]),
                (INTERLOCKED_DECREMENT, vec![0x1000])
            ]
        );
        e.mem.set_u32(pair + 4, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00c6_56b0, &args![pair]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }
}
