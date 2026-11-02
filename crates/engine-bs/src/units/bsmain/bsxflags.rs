//! `BSMain/bsxflags.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegerExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTEGER_EXTRA_DATA_RTTI: u32 = 0x011f_5f2c;

// Translated from 00a99c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_integer_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTEGER_EXTRA_DATA_RTTI)
}

/// `00a99d30` (named `NiFloatExtraData::LoadBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LOAD_BINARY_TARGET_00A99D30: u32 = 0x00a9_9d30;

// Translated from 00c430d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::LoadBinary` (engine map vtable name, vtable word 010c18a0): forwards `this` and
/// the stream to `00a99d30` (named `NiFloatExtraData::LoadBinary`, a body folded with the
/// integer extra data's). Returns nothing: Xbox PDB
/// `?LoadBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn bsx_flags_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LOAD_BINARY_TARGET_00A99D30, &args![this, stream]);
}

/// `00a9af00` (named `NiTextKeyExtraData::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00A9AF00: u32 = 0x00a9_af00;

// Translated from 00c430f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSXFlags` vtable word 010c18a4, between `LoadBinary`
/// (010c18a0) and the word 010c18a8. Forwards `this` and the stream to `00a9af00`, a
/// one-instruction jump to `NiExtraData::LinkObject` (`00a7b3b0`). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c430f0(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00A9AF00, &args![this, stream]);
}

/// `00a9a4e0` (named `NiFloatExtraData::SaveBinary` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const SAVE_BINARY_TARGET_00A9A4E0: u32 = 0x00a9_a4e0;

// Translated from 00c43140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::SaveBinary` (engine map vtable name, vtable word 010c18ac): forwards `this` and
/// the stream to `00a9a4e0` (named `NiFloatExtraData::SaveBinary`, a body folded with the
/// integer extra data's). Returns nothing: Xbox PDB
/// `?SaveBinary@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn bsx_flags_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SAVE_BINARY_TARGET_00A9A4E0, &args![this, stream]);
}

// Translated from 00c43650 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Answers the constant address 01202de0 without reading the receiver. The map names this body `BSXFlags::GetRTTI`; it takes only `this` (plain `RET`).
pub fn fn_00c43650(_e: &mut Engine, _this: Ptr) -> u32 {
    0x0120_2de0
}

/// `NiObjectNET::GetExtraData` (Xbox PDB, `00a5bdd0`): `thiscall`, `RET 4`; the extra data
/// whose name the given pointer (to a `NiFixedString`) names, or null.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// The global holding the pointer to the `NiFixedString` that names the `BSXFlags` extra data
/// (unconfirmed: set by a static initializer).
const BSX_FLAGS_NAME: u32 = 0x0120_2ddc;
/// `BSXFlags::ms_RTTI`, the address `BSXFlags::GetRTTI` (`00c43650`) answers.
const BSX_FLAGS_RTTI: u32 = 0x0120_2de0;
/// The virtual slot (byte offset) that answers the object's `NiRTTI`.
const SLOT_GET_RTTI: u32 = 0x08;

// Translated from 00c43490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`bsxflags.obj` range). `cdecl`, one stack word. For a non-null
/// object, looks up its extra data under the name the global at `01202ddc` points to
/// (`NiObjectNET::GetExtraData`) and answers it when it is a `BSXFlags` (its virtual slot +8
/// answers the `NiRTTI` at `01202de0`); null otherwise.
pub fn fn_00c43490(e: &mut Engine, object: Ptr) -> Ptr {
    if object.is_null() {
        return Ptr::NULL;
    }
    let name = e.global::<u32>(BSX_FLAGS_NAME);
    let data = e.call(GET_EXTRA_DATA, &args![object, name]).u32();
    if data != 0 && e.vcall(data, SLOT_GET_RTTI, &args![]).u32() == BSX_FLAGS_RTTI {
        Ptr::new(data)
    } else {
        Ptr::NULL
    }
}

/// `NiIntegerExtraData` vtable (RTTI).
const VTABLE_INTEGER_EXTRA_DATA: u32 = 0x010a_0d24;
/// `BSXFlags` vtable (RTTI).
const VTABLE_BSX_FLAGS: u32 = 0x010c_1854;
/// The `NiExtraData` default constructor (`00a7b2e0`, thiscall).
const EXTRA_DATA_DEFAULT_CONSTRUCTOR: u32 = 0x00a7_b2e0;
/// The `NiExtraData` destructor body (`00a7b300`, thiscall).
const EXTRA_DATA_DESTRUCTOR: u32 = 0x00a7_b300;
/// `NiDelete` of a sized block (`00aa1460`, cdecl: the block, its size).
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiNew` allocation (`00aa13e0`, cdecl: size).
const NI_NEW: u32 = 0x00aa_13e0;
/// The name setter `00a5b950` (unit `NiMain/niobjectnet.cpp`; thiscall, `RET 4`).
const SET_NAME: u32 = 0x00a5_b950;
/// The word whose address the factory passes to `00a5b950`.
const CREATED_NAME: u32 = 0x011f_49e0;
/// `00448a60` (unit `fallout shared/modelloader.cpp` in the engine map; thiscall, `RET 4`).
const FLAGS_TEST: u32 = 0x0044_8a60;
/// `"BSX"`.
const BSX_FLAGS_NAME_TEXT: u32 = 0x010c_1954;
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
/// `InterlockedDecrement` import slot (`kernel32`, stdcall: address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00c43470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Names the flags with `BSXFlags`'s extra-data name (`00a5b950` with the global at
/// `01202ddc`).
pub fn fn_00c43470(e: &mut Engine, this: Ptr) {
    let name = e.global::<u32>(BSX_FLAGS_NAME);
    e.call(SET_NAME, &args![this, name]);
}

/// The common body of the two constructors: `NiExtraData` (`00a7b2e0`), the
/// `NiIntegerExtraData` vtable and value (+0xc), the `BSXFlags` vtable, the name.
fn construct(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    e.call(EXTRA_DATA_DEFAULT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_INTEGER_EXTRA_DATA);
    e.mem.set_u32(this.addr() + 0xc, value);
    e.mem.set_u32(this.addr(), VTABLE_BSX_FLAGS);
    fn_00c43470(e, this);
    this
}

// Translated from 00c42f70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42f70 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e378, state variable) is not translated
/// `BSXFlags::BSXFlags` (engine map): an `NiIntegerExtraData` with value 0 given the
/// `BSXFlags` vtable and name (`00c43470`). Returns `this`.
pub fn bsx_flags_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    construct(e, this, 0)
}

// Translated from 00c42ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c42ff0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e39b, state variable) is not translated
/// `BSXFlags` constructor with the flags value: as `00c42f70` with `value` at +0xc. Returns
/// `this`.
pub fn bsx_flags_constructor_with_value(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    construct(e, this, value)
}

// Translated from 00c43070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags` destructor: the `BSXFlags` vtable, then the `NiIntegerExtraData` vtable, then
/// the `NiExtraData` destructor (`00a7b300`).
pub fn bsx_flags_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BSX_FLAGS);
    e.mem.set_u32(this.addr(), VTABLE_INTEGER_EXTRA_DATA);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00c430a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `this` (thiscall, `RET 4`: the stack word is not read).
pub fn fn_00c430a0(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 00c43620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::_scalar_deleting_destructor_` (engine map): the destructor (`00c43070`), then,
/// for bit 0 of `flags`, `00aa1460(this, 0x10)`. Returns `this`.
pub fn bsx_flags_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    bsx_flags_destructor(e, this);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00a99d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegerExtraData::_scalar_deleting_destructor_` (engine map): the
/// `NiIntegerExtraData` vtable, the `NiExtraData` destructor (`00a7b300`), then, for bit 0
/// of `flags` (`TEST byte`), `00aa1460(this, 0x10)`. Returns `this`.
pub fn ni_integer_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_INTEGER_EXTRA_DATA);
    e.call(EXTRA_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00c43190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c43190 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e3c5, state variable) is not translated
/// The `BSXFlags` factory (no arguments): a 0x10-byte `NiNew` block constructed
/// (`00c42f70`), or null when the allocation fails; then `00a5b950` runs on the result (also
/// when it is null) with `011f49e0`. Returns the result.
pub fn fn_00c43190(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x10u32]).u32();
    let object = if block == 0 {
        0
    } else {
        bsx_flags_constructor(e, Ptr::new(block)).addr()
    };
    e.call(SET_NAME, &args![object, CREATED_NAME]);
    object
}

// Translated from 00624b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::Find` (engine map; cdecl): the object's `BSXFlags` (`00c43490`) and, when there
/// is one, `00448a60` on it with `argument`; 0 otherwise.
pub fn fn_00624b70(e: &mut Engine, object: Ptr, argument: u32) -> u32 {
    let flags = fn_00c43490(e, object);
    if flags.addr() == 0 {
        return 0;
    }
    e.call(FLAGS_TEST, &args![flags, argument]).u32()
}

// Translated from 00c43500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c43500 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f2e3eb, state variable) is not translated
/// The start-up initializer of the `BSXFlags` name: a 4-byte `NiFixedString` from the memory
/// manager set to `"BSX"` (`00a5b690`); the compiler kept a test of the constant text's
/// address (`MOV EAX,imm; TEST; JNZ`), so the branch that stores the empty string
/// (`0109b220`) never runs (translated anyway). The string (or null when the allocation
/// failed) goes to `01202ddc`.
pub fn fn_00c43500(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32();
    let stored = if block == 0 {
        0
    } else {
        let text = std::hint::black_box(BSX_FLAGS_NAME_TEXT);
        let handle = if text != 0 {
            e.call(FIXED_STRING_CREATE, &args![BSX_FLAGS_NAME_TEXT])
                .u32()
        } else {
            e.global::<u32>(FIXED_STRING_EMPTY)
        };
        e.mem.set_u32(block, handle);
        block
    };
    e.set_global::<u32>(BSX_FLAGS_NAME, stored);
}

// Translated from 00c435a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Its `atexit` destructor: when the name exists, releases its handle (unless it is the
/// empty string `0109b220`: `InterlockedDecrement` of the count 8 bytes before the text),
/// frees the 4 bytes through the memory manager (behind a folded `1 & 1` test, always true),
/// and clears the global.
pub fn fn_00c435a0(e: &mut Engine) {
    let string = e.global::<u32>(BSX_FLAGS_NAME);
    if string != 0 {
        let handle = e.mem.u32(string);
        if handle != e.global::<u32>(FIXED_STRING_EMPTY) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
        if std::hint::black_box(1u32) & 1 != 0 {
            e.call(DEALLOCATE, &args![MEMORY_MANAGER, string]);
        }
    }
    e.set_global::<u32>(BSX_FLAGS_NAME, 0);
}

/// `NiIntegerExtraData`'s member copy (`00a99c40`; `thiscall` on the source, `RET 8`: clone,
/// cloning process), `NiBinaryExtraData::RegisterStreamables` (`00a89ae0`, engine map; the
/// shared extra-data body, `thiscall`, `RET 4`), the integer comparison (`00a742a0`, unit
/// `nistringextradata.cpp`; `thiscall`, `RET 4`: whether the other object is non-null with an
/// equal word at +0xC) and `NiIntegerExtraData::GetViewerStrings` (`00a99d70`).
const INTEGER_COPY_MEMBERS: u32 = 0x00a9_9c40;
const EXTRA_DATA_REGISTER_STREAMABLES: u32 = 0x00a8_9ae0;
const INTEGER_IS_EQUAL: u32 = 0x00a7_42a0;
const INTEGER_GET_VIEWER_STRINGS: u32 = 0x00a9_9d70;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: text) and its `bool` overload (`00a7d370`,
/// `cdecl`: name, byte); the viewer string array's `SetAtGrow` (`00470000`, `thiscall`,
/// `RET 8`: index, address of the string) and add (`0096a610`, `thiscall`, `RET 4`).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRINGS_SET_AT_GROW: u32 = 0x0047_0000;
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The flag names (`bAnimated` `010c1948`, `bHavok` `010c1940`, `bRagDoll` `010c1934`,
/// `bComplex` `010c1928`, `bAddon` `010c1920`, `bEditorMarker` `010c1910`, `bDynamic`
/// `010c1904`, `bArticulated` `010c18f4`, `bIKTarget` `010c18e8`) by flag bit, from bit 0.
const FLAG_NAMES: [u32; 9] = [
    0x010c_1948,
    0x010c_1940,
    0x010c_1934,
    0x010c_1928,
    0x010c_1920,
    0x010c_1910,
    0x010c_1904,
    0x010c_18f4,
    0x010c_18e8,
];

// Translated from 00c430b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSXFlags` member copy (unnamed in the engine map; `thiscall` on the source, `RET 8`:
/// clone, cloning process): forwards to `NiIntegerExtraData`'s (`00a99c40`).
pub fn fn_00c430b0(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(INTEGER_COPY_MEMBERS, &args![this, clone, cloning]);
}

// Translated from 00c43110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: stream): answers 1 when `00a89ae0`
/// answers a zero byte and 0 otherwise (`NEG`/`SBB`/`ADD 1` on the byte).
pub fn fn_00c43110(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(EXTRA_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .u8()
        == 0
}

// Translated from 00c43160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags`'s `IsEqual` (unnamed in the engine map; `thiscall`, `RET 4`: the other
/// object): the byte `00a742a0` answers.
pub fn fn_00c43160(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(INTEGER_IS_EQUAL, &args![this, other]).u8() != 0
}

/// Passes the viewer string `text` (by address, as the exe does) to `function` on
/// `strings`, after `leading` (the array's size for `SetAtGrow`, nothing for the add).
fn store_viewer_string(
    e: &mut Engine,
    function: u32,
    strings: Ptr,
    leading: Option<u32>,
    text: u32,
) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        match leading {
            Some(index) => e.call(function, &args![strings, index, slot]),
            None => e.call(function, &args![strings, slot]),
        };
    });
}

// Translated from 00c43220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSXFlags::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the viewer string array):
/// `NiIntegerExtraData::GetViewerStrings` (`00a99d70`), then the class name (the first word
/// of `01202de0`, `NiGetViewerString`) and the `bAnimated` flag (bit 0 of the flags at +0xC,
/// `00a7d370`), each stored with `SetAtGrow` at the array's size (its 16-bit `m_usSize`,
/// +0xA); then one string per further flag bit 1 to 8 (`bHavok`, `bRagDoll`, `bComplex`,
/// `bAddon`, `bEditorMarker`, `bDynamic`, `bArticulated`, `bIKTarget`), each added
/// (`0096a610`).
pub fn bsx_flags_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(INTEGER_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(BSX_FLAGS_RTTI);
    let text = e.call(VIEWER_STRING, &args![name]).u32();
    let size = u32::from(e.mem.u16(strings.addr().wrapping_add(0xa)));
    store_viewer_string(e, VIEWER_STRINGS_SET_AT_GROW, strings, Some(size), text);
    for (bit, flag_name) in FLAG_NAMES.iter().enumerate() {
        let flags = e.mem.u32(this.addr().wrapping_add(0xc));
        let set = u32::from(flags & (1 << bit) != 0);
        let text = e.call(VIEWER_STRING_BOOL, &args![*flag_name, set]).u32();
        if bit == 0 {
            let size = u32::from(e.mem.u16(strings.addr().wrapping_add(0xa)));
            store_viewer_string(e, VIEWER_STRINGS_SET_AT_GROW, strings, Some(size), text);
        } else {
            store_viewer_string(e, VIEWER_STRINGS_ADD, strings, None, text);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_30b0, fn_00c430b0(Ptr, Ptr, Ptr)),
        entry!(0x00c4_3110, fn_00c43110(Ptr, Ptr) -> bool),
        entry!(0x00c4_3160, fn_00c43160(Ptr, Ptr) -> bool),
        entry!(0x00c4_3220, bsx_flags_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c4_3470, fn_00c43470(Ptr)),
        entry!(0x00c4_2f70, bsx_flags_constructor(Ptr) -> Ptr),
        entry!(0x00c4_2ff0, bsx_flags_constructor_with_value(Ptr, u32) -> Ptr),
        entry!(0x00c4_3070, bsx_flags_destructor(Ptr)),
        entry!(0x00c4_30a0, fn_00c430a0(Ptr, u32) -> Ptr),
        entry!(0x00c4_3620, bsx_flags_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a9_9d00,
            ni_integer_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c4_3190, fn_00c43190() -> u32),
        entry!(0x0062_4b70, fn_00624b70(Ptr, u32) -> u32),
        entry!(0x00c4_3500, fn_00c43500()),
        entry!(0x00c4_35a0, fn_00c435a0()),
        entry!(0x00c4_3490, fn_00c43490(Ptr) -> Ptr),
        entry!(0x00c4_3650, fn_00c43650(Ptr) -> u32),
        entry!(0x00a99c60, ni_integer_extra_data_get_rtti() -> Ptr),
        entry!(0x00c4_30d0, bsx_flags_load_binary(Ptr, Ptr)),
        entry!(0x00c4_30f0, fn_00c430f0(Ptr, Ptr)),
        entry!(0x00c4_3140, bsx_flags_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_integer_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a99c60, &args![]).u32(),
            NI_INTEGER_EXTRA_DATA_RTTI
        );
    }

    #[test]
    fn forwarder_bsx_flags_load_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LOAD_BINARY_TARGET_00A99D30, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_30d0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LOAD_BINARY_TARGET_00A99D30)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c430f0_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00A9AF00, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_30f0, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00A9AF00)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_bsx_flags_save_binary_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(SAVE_BINARY_TARGET_00A9A4E0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3140, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == SAVE_BINARY_TARGET_00A9A4E0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod constant_getter_tests {
    use super::*;

    #[test]
    fn fn_00c43650_answers_its_constant_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c4_3650, &args![0x1000u32]).u32(), 0x0120_2de0);
        assert_eq!(e.call(0x00c4_3650, &args![0u32]).u32(), 0x0120_2de0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn rig(rtti_of_data: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(BSX_FLAGS_NAME, 4);
        e.set_global(BSX_FLAGS_NAME, 0x0120_3000);
        let mut slots = [0u32; 4];
        slots[2] = 0x0f00_c000;
        e.put_vtable(0x0f00_c100, &slots);
        e.register_double(0x0f00_c000, move |_, _| rtti_of_data.into_ret());
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0x0f00_c100);
        e.register_double(GET_EXTRA_DATA, move |_, _| data.into_ret());
        (e, data)
    }

    #[test]
    fn at_00c43490_answers_the_extra_data_when_it_is_a_bsx_flags() {
        let (mut e, data) = rig(BSX_FLAGS_RTTI);
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), data);
        let lookups: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GET_EXTRA_DATA)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(lookups, vec![vec![0x4000, 0x0120_3000]]);
    }

    #[test]
    fn at_00c43490_answers_null_for_other_extra_data_nothing_or_a_null_object() {
        let (mut e, _) = rig(0x0120_2de4);
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), 0);
        e.register(GET_EXTRA_DATA, |_, _| Ret::default());
        assert_eq!(e.call(0x00c4_3490, &args![0x4000u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3490, &args![0u32]).u32(), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 != GET_EXTRA_DATA));
    }
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(BSX_FLAGS_NAME, 4);
        e.mem.set_u32(BSX_FLAGS_NAME, 0x4321);
        e.register(EXTRA_DATA_DEFAULT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(EXTRA_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(NI_DELETE, |_, _| Ret::default());
        e
    }

    #[test]
    fn constructors_set_the_value_vtable_and_name() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_2f70, &args![this]).u32(), this);
        assert_eq!(
            log_of(&mut e, 0x00c4_2f70),
            vec![
                (EXTRA_DATA_DEFAULT_CONSTRUCTOR, vec![this]),
                (SET_NAME, vec![this, 0x4321]),
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE_BSX_FLAGS);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.call(0x00c4_2ff0, &args![this, 0x82u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0x82);
        e.call_log = Some(vec![]);
        e.call(0x00c4_3470, &args![this]);
        assert_eq!(
            log_of(&mut e, 0x00c4_3470),
            vec![(SET_NAME, vec![this, 0x4321])]
        );
    }

    #[test]
    fn destructors_restore_the_base_vtable_and_free_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, VTABLE_BSX_FLAGS);
        for (addr, flags) in [(0x00c4_3620u32, 1u32), (0x00a9_9d00, 1), (0x00c4_3620, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this, flags]).u32(), this);
            let mut want = vec![(EXTRA_DATA_DESTRUCTOR, vec![this])];
            if flags & 1 != 0 {
                want.push((NI_DELETE, vec![this, 0x10]));
            }
            assert_eq!(log_of(&mut e, addr), want);
            assert_eq!(e.mem.u32(this), VTABLE_INTEGER_EXTRA_DATA);
        }
        e.call(0x00c4_3070, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_INTEGER_EXTRA_DATA);
        assert_eq!(e.call(0x00c4_30a0, &args![this, 5u32]).u32(), this);
    }

    #[test]
    fn factory_builds_flags_or_passes_null_on() {
        let mut e = engine();
        let block = e.mem.alloc(0x10);
        e.register_double(NI_NEW, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3190, &args![]).u32(), block);
        let log = log_of(&mut e, 0x00c4_3190);
        assert_eq!(log[0], (NI_NEW, vec![0x10]));
        assert_eq!(log.last().unwrap(), &(SET_NAME, vec![block, CREATED_NAME]));
        assert_eq!(e.mem.u32(block), VTABLE_BSX_FLAGS);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3190, &args![]).u32(), 0);
        assert_eq!(
            log_of(&mut e, 0x00c4_3190),
            vec![(NI_NEW, vec![0x10]), (SET_NAME, vec![0, CREATED_NAME])]
        );
    }

    #[test]
    fn find_tests_the_flags_of_an_object_that_has_them() {
        let mut e = engine();
        let data = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x00aa_000a);
        e.mem.set_u32(data, vtable);
        e.register(0x00aa_000a, |_, _| BSX_FLAGS_RTTI.into_ret());
        e.register_double(GET_EXTRA_DATA, move |_, _| data.into_ret());
        e.register(FLAGS_TEST, |_, a| (a[1] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_4b70, &args![0x900u32, 0x20u32]).u32(), 0x21);
        let tests: Vec<_> = log_of(&mut e, 0x0062_4b70)
            .into_iter()
            .filter(|(a, _)| *a == FLAGS_TEST)
            .collect();
        assert_eq!(tests, vec![(FLAGS_TEST, vec![data, 0x20])]);
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0062_4b70, &args![0x900u32, 0x20u32]).u32(), 0);
    }

    #[test]
    fn name_initializer_and_destructor() {
        let mut e = Engine::new();
        e.map(BSX_FLAGS_NAME, 4);
        e.map(FIXED_STRING_EMPTY, 4);
        e.mem.set_u32(FIXED_STRING_EMPTY, 0x4444);
        let block = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, _| block.into_ret());
        e.register(FIXED_STRING_CREATE, |_, _| 0x8008u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_3500, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c4_3500),
            vec![
                (ALLOCATE, vec![MEMORY_MANAGER, 4]),
                (FIXED_STRING_CREATE, vec![BSX_FLAGS_NAME_TEXT]),
            ]
        );
        assert_eq!(e.mem.u32(BSX_FLAGS_NAME), block);
        assert_eq!(e.mem.u32(block), 0x8008);
        e.call_log = Some(vec![]);
        e.call(0x00c4_35a0, &args![]);
        assert_eq!(
            log_of(&mut e, 0x00c4_35a0),
            vec![
                (INTERLOCKED_DECREMENT, vec![0x8000]),
                (DEALLOCATE, vec![MEMORY_MANAGER, block]),
            ]
        );
        assert_eq!(e.mem.u32(BSX_FLAGS_NAME), 0);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00c4_3500, &args![]);
        assert_eq!(e.mem.u32(BSX_FLAGS_NAME), 0);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn copy_register_and_compare_forward_to_the_integer_bodies() {
        let mut e = Engine::new();
        e.register(INTEGER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(EXTRA_DATA_REGISTER_STREAMABLES, |_, a| a[1].into_ret());
        e.register(INTEGER_IS_EQUAL, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c4_30b0, &args![0x10u32, 0x20u32, 0x30u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(INTEGER_COPY_MEMBERS, vec![0x10, 0x20, 0x30])));
        assert!(e.call(0x00c4_3110, &args![0x10u32, 0x100u32]).bool());
        assert!(!e.call(0x00c4_3110, &args![0x10u32, 0x101u32]).bool());
        assert_eq!(e.call(0x00c4_3110, &args![0x10u32, 0x100u32]).u32(), 1);
        assert!(e.call(0x00c4_3160, &args![0x10u32, 0x201u32]).bool());
        assert!(!e.call(0x00c4_3160, &args![0x10u32, 0x200u32]).bool());
    }

    #[test]
    fn viewer_strings_list_every_flag() {
        let mut e = Engine::new();
        e.map(BSX_FLAGS_RTTI, 8);
        e.mem.set_u32(BSX_FLAGS_RTTI, 0x0400_0000);
        e.register(INTEGER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, _| 0x51u32.into_ret());
        e.register(VIEWER_STRING_BOOL, |_, a| {
            (a[0] & 0xfff).wrapping_mul(2).wrapping_add(a[1]).into_ret()
        });
        let stored = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = stored.clone();
        e.register_double(VIEWER_STRINGS_SET_AT_GROW, move |e, a| {
            inner.borrow_mut().push((a[1], e.mem.u32(a[2])));
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            Ret::default()
        });
        let inner = stored.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            inner.borrow_mut().push((0xffff, e.mem.u32(a[1])));
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0x0000_0105);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.call(0x00c4_3220, &args![this, strings]);
        let expected: Vec<(u32, u32)> = std::iter::once((3, 0x51))
            .chain(FLAG_NAMES.iter().enumerate().map(|(bit, name)| {
                let set = u32::from(0x105u32 & (1 << bit) != 0);
                let index = if bit == 0 { 4 } else { 0xffff };
                (index, (name & 0xfff) * 2 + set)
            }))
            .collect();
        assert_eq!(*stored.borrow(), expected);
    }
}
