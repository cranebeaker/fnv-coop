//! `NiMain/nifogproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f5f3c`: the pointer `00a9a090` stores the object it builds in.
const FOG_PROPERTY_SLOT: u32 = 0x011f_5f3c;
/// Three words at `011f4998` that the object takes at `+0x20`.
const FOG_COLOR_SOURCE: u32 = 0x011f_4998;

// Translated from 00a9a090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nifogproperty.cpp`; run by the start-up `00a89cc0`, no
/// arguments): builds a 0x2c-byte object (allocation `00aa13e0`, base constructor
/// `00a5c200`, vtable `010a0e0c`, the word at `+0x18` set to 0, the float at `+0x1c` to
/// 1.0, the three floats at `+0x20` first to 0.0 and then replaced by the three words at
/// `011f4998`) and stores it, counted, in `011f5f3c`, releasing the object held there
/// before. A failed allocation stores null.
pub fn fn_00a9a090(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x010a_0e0c);
        // `FLDZ; FST; FST; FSTP`: 0.0 into the three words at +0x20.
        e.mem.set_u32(block + 0x20, 0.0f32.to_bits());
        e.mem.set_u32(block + 0x24, 0.0f32.to_bits());
        e.mem.set_u32(block + 0x28, 0.0f32.to_bits());
        e.mem.set_u16(block + 0x18, 0);
        e.mem.set_u32(block + 0x1c, 1.0f32.to_bits());
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(FOG_COLOR_SOURCE + offset);
            e.mem.set_u32(block + 0x20 + offset, word);
        }
        block
    };
    assign_ref_counted(e, FOG_PROPERTY_SLOT, object);
}

/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

// Translated from 00a9a120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; one of the `NiMain` shutdown steps of
/// `00a8a280`): releases the object that `00a9a090` stores in `011f5f3c`. When it is not null, drops one reference
/// (`InterlockedDecrement` of the count at +4; at zero it
/// calls the object's vtable slot +4) and clears the pointer.
pub fn fn_00a9a120(e: &mut Engine) {
    let object = e.mem.u32(FOG_PROPERTY_SLOT);
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
        e.mem.set_u32(FOG_PROPERTY_SLOT, 0);
    }
}

/// `NiFogProperty::ms_RTTI` (the class's `NiRTTI` record; its first word is the class
/// name).
const NI_FOG_PROPERTY_RTTI: u32 = 0x011f_5f34;
/// `NiFogProperty`'s vtable.
const NI_FOG_PROPERTY_VTABLE: u32 = 0x010a_0e0c;
/// The array allocation (`00aa1070`, `cdecl`, one word; unnamed in the engine map) and
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, arguments).
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// The formats `"%s = FOG_Z_LINEAR"`, `"%s = FOG_RANGE_SQ"` and `"%s = FOG_VERTEX_ALPHA"`.
const FORMAT_Z_LINEAR: u32 = 0x010a_0dc4;
const FORMAT_RANGE_SQ: u32 = 0x010a_0dd8;
const FORMAT_VERTEX_ALPHA: u32 = 0x010a_0dec;
/// `00a5d7a0` (unnamed in the engine map; `thiscall`: the clone and the cloning process):
/// the property base's member copy that `CreateClone` runs.
const PROPERTY_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a5d450` (unnamed, unit `niobject.cpp`; `thiscall` with two stack words: the object
/// and the cloning process): what `CreateClone` answers through when the word at +0xc is 0.
const SHARED_CLONE_00A5D450: u32 = 0x00a5_d450;
/// `00a62140` (`thiscall`, the other object; named `NiSequenceStreamHelper::IsEqual` in the
/// engine map, a body shared by identical-code folding): the base comparison `IsEqual`
/// starts with.
const BASE_IS_EQUAL_00A62140: u32 = 0x00a6_2140;
/// `00439090` (`thiscall` on a colour, the other colour; unit `multiboundmarkerdata.cpp` in
/// the engine map): nonzero when the two colours differ (by its body the negation of
/// `004390c0`).
const COLORS_DIFFER_00439090: u32 = 0x0043_9090;
/// `NiProperty::LoadBinary` (`00a5d7f0`) and the base save (`00a5d7b0`, named
/// `NiSequenceStreamHelper::SaveBinary` in the engine map, a folded body), both `thiscall`
/// with the stream.
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;
const PROPERTY_SAVE_BINARY_00A5D7B0: u32 = 0x00a5_d7b0;
/// `NiColor::LoadBinary` (`00a59be0`), `NiColor::SaveBinary` (`00a69780`), both `thiscall`
/// with the stream, and `NiColor::GetViewerString` (`00a696c0`, `thiscall` with a name,
/// answering a new string).
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
const COLOR_SAVE_BINARY: u32 = 0x00a6_9780;
const COLOR_GET_VIEWER_STRING: u32 = 0x00a6_96c0;
/// `NiProperty::GetViewerStrings` (`00a5d850`, `thiscall`, the string array).
const PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiGetViewerString` (`00a7d330`, the class name), `NiGetViewerString_ov2` (`00a7d370`, a
/// name and a bool word, of which it reads the low byte) and `NiGetViewerString_ov9`
/// (`00a7d550`, a name and a float), all `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literals `"Enable"`, `"Depth"`, `"Function"` and `"Color"`.
const TEXT_ENABLE: u32 = 0x0104_153c;
const TEXT_DEPTH: u32 = 0x010a_0eb4;
const TEXT_FUNCTION: u32 = 0x010a_0ea8;
const TEXT_COLOR: u32 = 0x010a_0ea0;
/// `NiStream::m_uiNifFileVersion` (+0xd8), `m_pkIstr` (+0x24c), `m_pkOstr` (+0x250) and
/// `m_usNiPropertyFlags` (+0x288) (Xbox PDB).
const STREAM_FILE_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const STREAM_PROPERTY_FLAGS: u32 = 0x288;
/// `NiBinaryStream::m_uiAbsoluteCurrentPos` (+4), `m_pfnRead` (+8) and `m_pfnWrite` (+0xc)
/// (Xbox PDB); the functions are `cdecl` (stream, buffer, bytes, address of the component
/// size, component count), answering the bytes moved.
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// The first file version whose fog property stores its own flags, and the first whose
/// property flags hold them unconverted.
const OWN_FLAGS_VERSION: u32 = 0x0a00_0102;
const NEW_FLAGS_VERSION: u32 = 0x0500_0016;
/// The fog property's 16-bit flags (+0x18; bit 0 enable, bits 1-2 the fog function), its
/// depth float (+0x1c) and its colour (three floats at +0x20).
const FOG_FLAGS: u32 = 0x18;
const FOG_DEPTH: u32 = 0x1c;
const FOG_COLOR: u32 = 0x20;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// One transfer through the stream's input or output (`function_offset` 8 for `m_pfnRead`,
/// 0xc for `m_pfnWrite` of the binary stream at `stream_offset`): `bytes` at `data`, the
/// component size word set to `bytes`, one component; the position advanced by the result.
fn transfer(
    e: &mut Engine,
    stream: Ptr,
    stream_offset: u32,
    function_offset: u32,
    data: u32,
    bytes: u32,
) {
    let binary = e.mem.u32(stream.addr().wrapping_add(stream_offset));
    let function = e.mem.u32(binary.wrapping_add(function_offset));
    let done = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), bytes);
        e.call(function, &args![binary, data, bytes, size, 1u32])
            .u32()
    });
    let position = binary.wrapping_add(BINARY_STREAM_POSITION);
    let advanced = e.mem.u32(position).wrapping_add(done);
    e.mem.set_u32(position, advanced);
}

/// The constructor's body (inlined by the creation function): the base constructor
/// `00a5c200`, the vtable, the colour cleared (`FLDZ`, three stores), flags 0, depth 1.0,
/// then the colour replaced by the three words at `011f4998`.
fn construct(e: &mut Engine, object: u32) {
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![object]);
    e.mem.set_u32(object, NI_FOG_PROPERTY_VTABLE);
    for offset in [0u32, 4, 8] {
        e.mem.set_u32(object + FOG_COLOR + offset, 0.0f32.to_bits());
    }
    e.mem.set_u16(object + FOG_FLAGS, 0);
    e.mem.set_u32(object + FOG_DEPTH, 1.0f32.to_bits());
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(FOG_COLOR_SOURCE + offset);
        e.mem.set_u32(object + FOG_COLOR + offset, word);
    }
}

// Translated from 00a99e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::GetViewerString` (Xbox PDB; `cdecl`: a name and a fog function): a new
/// buffer of the name's length plus 0x14 (`00aa1070`), into which `NiSprintf` writes
/// `"<name> = FOG_Z_LINEAR"`, `"... FOG_RANGE_SQ"` or `"... FOG_VERTEX_ALPHA"` for the
/// functions 0, 1 and 2; any other function leaves the buffer as allocated. Answers the
/// buffer.
pub fn ni_fog_property_get_viewer_string(e: &mut Engine, name: Ptr, function: u32) -> Ptr {
    let mut length = 0u32;
    while e.mem.u8(name.addr().wrapping_add(length)) != 0 {
        length += 1;
    }
    let size = length.wrapping_add(0x14);
    let buffer = e.call(ALLOCATE_ARRAY, &args![size]).u32();
    let format = match function {
        0 => FORMAT_Z_LINEAR,
        1 => FORMAT_RANGE_SQ,
        2 => FORMAT_VERTEX_ALPHA,
        _ => return Ptr::new(buffer),
    };
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    Ptr::new(buffer)
}

// Translated from 00a99ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nifogproperty.cpp`); by its body the `NiFogProperty`
/// constructor (`thiscall`, no stack words): the base constructor `00a5c200`, the vtable
/// `010a0e0c`, flags 0, depth 1.0 and the colour from the three words at `011f4998` (after
/// clearing it to 0.0). Answers `this`.
pub fn fn_00a99ed0(e: &mut Engine, this: Ptr) -> Ptr {
    construct(e, this.addr());
    this
}

// Translated from 00a99f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::CreateClone` (Xbox PDB): when the word at +0xc is 0, answers what
/// `00a5d450` answers for this object (EAX: `this`); otherwise a new fog property (0x2c
/// bytes, the constructor `00a99ed0`; null when the allocation fails), the property member
/// copy (`00a5d7a0`), then the flags, depth (`FLD`/`FSTP`) and colour copied. Answers the
/// clone.
pub fn ni_fog_property_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let base = this.addr();
    if e.mem.u32(base + 0xc) == 0 {
        e.call(SHARED_CLONE_00A5D450, &args![this, this, cloning_process]);
        return this;
    }
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    let clone = if block == 0 {
        0
    } else {
        fn_00a99ed0(e, Ptr::new(block)).addr()
    };
    e.call(PROPERTY_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let flags = e.mem.u16(base + FOG_FLAGS);
    e.mem.set_u16(clone.wrapping_add(FOG_FLAGS), flags);
    let depth = x87_quiet_bits(e.mem.u32(base + FOG_DEPTH));
    e.mem.set_u32(clone.wrapping_add(FOG_DEPTH), depth);
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(base + FOG_COLOR + offset);
        e.mem.set_u32(clone.wrapping_add(FOG_COLOR + offset), word);
    }
    Ptr::new(clone)
}

// Translated from 00a99fd0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiFogProperty`): 0x2c bytes with the constructor's body inlined, or null.
pub fn fn_00a99fd0(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x2cu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00a9a030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::IsEqual` (Xbox PDB): true when the base comparison (`00a62140`) holds,
/// bit 0 of the flags matches, the depths compare equal (`FUCOMPP`, a NaN is not equal),
/// bits 1-2 of the flags match and the colours do not differ (`00439090`).
pub fn ni_fog_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL_00A62140, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.u8(this.addr() + FOG_FLAGS);
    if (mine ^ e.mem.u8(other.addr().wrapping_add(FOG_FLAGS))) & 1 != 0 {
        return false;
    }
    let a = x87_load32(e.mem.u32(this.addr() + FOG_DEPTH));
    let b = x87_load32(e.mem.u32(other.addr().wrapping_add(FOG_DEPTH)));
    if b != a {
        return false;
    }
    let mine = e.mem.u8(this.addr() + FOG_FLAGS);
    if (mine ^ e.mem.u8(other.addr().wrapping_add(FOG_FLAGS))) & 6 != 0 {
        return false;
    }
    !e.call(
        COLORS_DIFFER_00439090,
        &args![
            this.addr() + FOG_COLOR,
            other.addr().wrapping_add(FOG_COLOR)
        ],
    )
    .bool()
}

// Translated from 00a9a150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`, then the flags. Files
/// before version `0x0a000102` take the stream's property flags (+0x288): below `0x05000016`
/// converted (bit 1 becomes the enable bit 0, bits 3-4 the function bits 1-2), from it bits
/// 0-2 as they are; later files read 2 bytes. Then the depth (4 bytes) is read and the
/// colour loads itself (`NiColor::LoadBinary`).
pub fn ni_fog_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let base = this.addr();
    let version = e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION));
    if version < OWN_FLAGS_VERSION {
        let flags = u32::from(e.mem.u16(stream.addr().wrapping_add(STREAM_PROPERTY_FLAGS)));
        e.mem.set_u16(base + FOG_FLAGS, flags as u16);
        if e.mem.u32(stream.addr().wrapping_add(STREAM_FILE_VERSION)) < NEW_FLAGS_VERSION {
            let enable = (flags >> 1) & 1;
            let function = (flags >> 3) & 3;
            e.mem.set_u16(base + FOG_FLAGS, 0);
            e.mem.set_u16(base + FOG_FLAGS, enable as u16);
            let kept = e.mem.u16(base + FOG_FLAGS) & 0xfff9;
            e.mem
                .set_u16(base + FOG_FLAGS, kept | (function * 2) as u16);
        } else {
            e.mem.set_u16(base + FOG_FLAGS, (flags & 7) as u16);
        }
    } else {
        transfer(
            e,
            stream,
            STREAM_INPUT,
            BINARY_STREAM_READ,
            base + FOG_FLAGS,
            2,
        );
    }
    transfer(
        e,
        stream,
        STREAM_INPUT,
        BINARY_STREAM_READ,
        base + FOG_DEPTH,
        4,
    );
    e.call(COLOR_LOAD_BINARY, &args![base + FOG_COLOR, stream]);
}

// Translated from 00a9a230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::SaveBinary` (Xbox PDB): the base save (`00a5d7b0`), the flags (2 bytes)
/// and the depth (4 bytes) through the stream's output, then the colour saves itself
/// (`NiColor::SaveBinary`).
pub fn ni_fog_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_SAVE_BINARY_00A5D7B0, &args![this, stream]);
    let base = this.addr();
    transfer(
        e,
        stream,
        STREAM_OUTPUT,
        BINARY_STREAM_WRITE,
        base + FOG_FLAGS,
        2,
    );
    transfer(
        e,
        stream,
        STREAM_OUTPUT,
        BINARY_STREAM_WRITE,
        base + FOG_DEPTH,
        4,
    );
    e.call(COLOR_SAVE_BINARY, &args![base + FOG_COLOR, stream]);
}

// Translated from 00a9a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFogProperty::GetViewerStrings` (Xbox PDB): `NiProperty::GetViewerStrings`, then the
/// class name, `"Enable"` with bit 0 of the flags (a bool word built in the stack word still
/// holding the class name's string, whose upper three bytes it keeps), `"Depth"` with the
/// depth, `"Function"` with bits 1-2 (`GetViewerString`) and `"Color"` with the colour
/// (`NiColor::GetViewerString`), each appended to the array.
pub fn ni_fog_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let base = this.addr();
    let name = e.mem.u32(NI_FOG_PROPERTY_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let enable = u32::from(e.mem.u8(base + FOG_FLAGS) & 1);
    let word = (text & 0xffff_ff00) | enable;
    let text = e.call(VIEWER_STRING_BOOL, &args![TEXT_ENABLE, word]).u32();
    push_viewer_string(e, strings, text);
    let depth = x87_quiet_bits(e.mem.u32(base + FOG_DEPTH));
    let text = e.call(VIEWER_STRING_FLOAT, &args![TEXT_DEPTH, depth]).u32();
    push_viewer_string(e, strings, text);
    let function = (u32::from(e.mem.u16(base + FOG_FLAGS)) >> 1) & 3;
    let text = ni_fog_property_get_viewer_string(e, Ptr::new(TEXT_FUNCTION), function).addr();
    push_viewer_string(e, strings, text);
    let text = e
        .call(
            COLOR_GET_VIEWER_STRING,
            &args![base + FOG_COLOR, TEXT_COLOR],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9_a090, fn_00a9a090()),
        entry!(0x00a9_a120, fn_00a9a120()),
        entry!(
            0x00a9_9e50,
            ni_fog_property_get_viewer_string(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_9ed0, fn_00a99ed0(Ptr) -> Ptr),
        entry!(0x00a9_9f60, ni_fog_property_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_9fd0, fn_00a99fd0() -> Ptr),
        entry!(0x00a9_a030, ni_fog_property_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_a150, ni_fog_property_load_binary(Ptr, Ptr)),
        entry!(0x00a9_a230, ni_fog_property_save_binary(Ptr, Ptr)),
        entry!(0x00a9_a2a0, ni_fog_property_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_with_the_three_source_words() {
        let run = run_default_instance(0x00a9_a090, FOG_PROPERTY_SLOT, |e| {
            e.mem.set_u32(FOG_COLOR_SOURCE, 0x1111_1111);
            e.mem.set_u32(FOG_COLOR_SOURCE + 4, 0x2222_2222);
            e.mem.set_u32(FOG_COLOR_SOURCE + 8, 0x3333_3333);
        });
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x010a_0e0c);
        assert_eq!(e.mem.u16(run.object + 0x18), 0);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(run.object + 0x20), 0x1111_1111);
        assert_eq!(e.mem.u32(run.object + 0x24), 0x2222_2222);
        assert_eq!(e.mem.u32(run.object + 0x28), 0x3333_3333);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a9_a090, FOG_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a9a120_releases_and_clears_the_held_object() {
        let mut e = Engine::new();
        e.map(FOG_PROPERTY_SLOT & !0xfff, 0x1000);
        e.put_vtable(0x0150_0000, &[0, 0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0150_0000);
        let remaining = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let counter = remaining.clone();
        e.register_double(INTERLOCKED_DECREMENT, move |_, _| {
            counter.set(counter.get() - 1);
            counter.get().into_ret()
        });
        // Still referenced elsewhere: no destructor.
        remaining.set(2);
        e.mem.set_u32(FOG_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a9_a120, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(INTERLOCKED_DECREMENT, vec![object + 4])));
        assert!(log.iter().all(|call| call.0 != 0x0150_1000));
        assert_eq!(e.mem.u32(FOG_PROPERTY_SLOT), 0);
        // Last reference: vtable slot +4 runs on the object.
        e.mem.set_u32(FOG_PROPERTY_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a9_a120, &args![]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0150_1000, vec![object])));
        assert_eq!(e.mem.u32(FOG_PROPERTY_SLOT), 0);
        // Null: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a9_a120, &args![]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        for (i, v) in [0.25f32, 0.5, 0.75].iter().enumerate() {
            e.mem.set_f32(FOG_COLOR_SOURCE + 4 * i as u32, *v);
        }
        e.register(ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x2c]);
            block.into_ret()
        });
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            OBJECT_BASE_CONSTRUCTOR,
            PROPERTY_COPY_MEMBERS,
            SHARED_CLONE_00A5D450,
            PROPERTY_LOAD_BINARY,
            PROPERTY_SAVE_BINARY_00A5D7B0,
            COLOR_LOAD_BINARY,
            COLOR_SAVE_BINARY,
            PROPERTY_GET_VIEWER_STRINGS,
            NI_SPRINTF,
            STRINGS_GROW,
            STRINGS_SET_AT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    fn assert_built(e: &Engine, object: u32) {
        assert_eq!(e.mem.u32(object), NI_FOG_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(object + FOG_FLAGS), 0);
        assert_eq!(e.mem.f32(object + FOG_DEPTH), 1.0);
        assert_eq!(e.mem.f32(object + FOG_COLOR), 0.25);
        assert_eq!(e.mem.f32(object + FOG_COLOR + 8), 0.75);
    }

    #[test]
    fn get_viewer_string_names_the_function() {
        let mut e = engine();
        let name = e.mem.alloc(8);
        e.mem.write(name, b"Func\0");
        for (function, format) in [
            (0u32, FORMAT_Z_LINEAR),
            (1, FORMAT_RANGE_SQ),
            (2, FORMAT_VERTEX_ALPHA),
        ] {
            let buffer = e.call(0x00a9_9e50, &args![name, function]).u32();
            let printed = calls_to(&e, NI_SPRINTF);
            assert_eq!(printed.last().unwrap(), &vec![buffer, 0x18, format, name]);
        }
        assert_eq!(calls_to(&e, ALLOCATE_ARRAY), vec![vec![0x18]; 3]);
        let buffer = e.call(0x00a9_9e50, &args![name, 3u32]).u32();
        assert_ne!(buffer, 0);
        assert_eq!(calls_to(&e, NI_SPRINTF).len(), 3);
        assert_eq!(calls_to(&e, ALLOCATE_ARRAY).len(), 4);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x00a9_9ed0, &args![this]).u32(), this);
        assert_built(&e, this);
        assert_eq!(calls_to(&e, OBJECT_BASE_CONSTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn create_object_builds_or_answers_null() {
        let mut e = engine();
        let object = e.call(0x00a9_9fd0, &args![]).u32();
        assert_built(&e, object);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_9fd0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_fog_or_shares() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u16(this + FOG_FLAGS, 5);
        e.mem.set_f32(this + FOG_DEPTH, 3.0);
        e.mem.set_f32(this + FOG_COLOR + 4, 9.0);
        let clone = e.call(0x00a9_9f60, &args![this, 0x77u32]).u32();
        assert_ne!(clone, this);
        assert_eq!(
            calls_to(&e, PROPERTY_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        assert_eq!(e.mem.u32(clone), NI_FOG_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(clone + FOG_FLAGS), 5);
        assert_eq!(e.mem.f32(clone + FOG_DEPTH), 3.0);
        assert_eq!(e.mem.f32(clone + FOG_COLOR + 4), 9.0);
        e.mem.set_u32(this + 0xc, 0);
        assert_eq!(e.call(0x00a9_9f60, &args![this, 0x77u32]).u32(), this);
        assert_eq!(
            calls_to(&e, SHARED_CLONE_00A5D450),
            vec![vec![this, this, 0x77]]
        );
        assert_eq!(calls_to(&e, ALLOCATE).len(), 1);
    }

    #[test]
    fn is_equal_checks_base_flags_depth_and_colour() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL_00A62140, |_, _| 1u32.into_ret());
        e.register(COLORS_DIFFER_00439090, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0x2c);
        let other = e.mem.alloc(0x2c);
        for object in [this, other] {
            e.mem.set_u16(object + FOG_FLAGS, 3);
            e.mem.set_f32(object + FOG_DEPTH, 2.0);
        }
        assert!(e.call(0x00a9_a030, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, COLORS_DIFFER_00439090),
            vec![vec![this + FOG_COLOR, other + FOG_COLOR]]
        );
        e.mem.set_u16(other + FOG_FLAGS, 2);
        assert!(!e.call(0x00a9_a030, &args![this, other]).bool());
        e.mem.set_u16(other + FOG_FLAGS, 7);
        assert!(!e.call(0x00a9_a030, &args![this, other]).bool());
        // Bits above 2 are not compared.
        e.mem.set_u16(other + FOG_FLAGS, 0x0b);
        assert!(e.call(0x00a9_a030, &args![this, other]).bool());
        e.mem.set_f32(other + FOG_DEPTH, 2.5);
        assert!(!e.call(0x00a9_a030, &args![this, other]).bool());
        e.mem.set_f32(other + FOG_DEPTH, 2.0);
        e.mem.set_f32(other + FOG_COLOR, 1.0);
        assert!(!e.call(0x00a9_a030, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL_00A62140, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_a030, &args![this, this]).bool());
    }

    /// A stream with version `version`, property flags `flags` and a binary stream whose
    /// read and write double (`0x0300_0000`) fills a read with 0x5a bytes.
    fn stream(e: &mut Engine, version: u32, flags: u16) -> (u32, u32) {
        let stream = e.mem.alloc(0x2a0);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        e.mem.set_u16(stream + STREAM_PROPERTY_FLAGS, flags);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(binary + BINARY_STREAM_READ, 0x0300_0000);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            a[2].into_ret()
        });
        (stream, binary)
    }

    #[test]
    fn load_binary_converts_old_flags_or_reads_them() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        // Before 0x05000016: bit 1 -> bit 0, bits 3-4 -> bits 1-2.
        let (old, binary) = stream(&mut e, 0x0400_0002, 0x1a);
        e.call(0x00a9_a150, &args![this, old]);
        assert_eq!(e.mem.u16(this + FOG_FLAGS), 0x7);
        let reads: Vec<(u32, u32)> = calls_to(&e, 0x0300_0000)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(reads, vec![(binary, this + FOG_DEPTH)]);
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 4);
        assert_eq!(
            calls_to(&e, COLOR_LOAD_BINARY),
            vec![vec![this + FOG_COLOR, old]]
        );
        let (old, _) = stream(&mut e, 0x0400_0002, 0x08);
        e.call(0x00a9_a150, &args![this, old]);
        assert_eq!(e.mem.u16(this + FOG_FLAGS), 0x2);
        // From 0x05000016: bits 0-2 kept.
        let (middle, _) = stream(&mut e, 0x0500_0016, 0xfe);
        e.call(0x00a9_a150, &args![this, middle]);
        assert_eq!(e.mem.u16(this + FOG_FLAGS), 0x6);
        // From 0x0a000102: two bytes read, then the depth.
        let (new, binary) = stream(&mut e, 0x0a00_0102, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x00a9_a150, &args![this, new]);
        let reads: Vec<(u32, u32)> = calls_to(&e, 0x0300_0000)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        assert_eq!(reads, vec![(this + FOG_FLAGS, 2), (this + FOG_DEPTH, 4)]);
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 6);
        assert_eq!(calls_to(&e, PROPERTY_LOAD_BINARY), vec![vec![this, new]]);
    }

    #[test]
    fn save_binary_writes_flags_depth_and_colour() {
        let mut e = engine();
        let this = e.mem.alloc(0x2c);
        let (stream, binary) = stream(&mut e, 0, 0);
        e.call(0x00a9_a230, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PROPERTY_SAVE_BINARY_00A5D7B0),
            vec![vec![this, stream]]
        );
        let writes: Vec<(u32, u32)> = calls_to(&e, 0x0300_0000)
            .iter()
            .map(|c| (c[1], c[2]))
            .collect();
        assert_eq!(writes, vec![(this + FOG_FLAGS, 2), (this + FOG_DEPTH, 4)]);
        assert_eq!(e.mem.u32(binary + BINARY_STREAM_POSITION), 6);
        assert_eq!(
            calls_to(&e, COLOR_SAVE_BINARY),
            vec![vec![this + FOG_COLOR, stream]]
        );
    }

    #[test]
    fn get_viewer_strings_lists_the_fog() {
        let mut e = engine();
        e.map(0x010a_0000, 0x1000);
        e.mem.set_u32(NI_FOG_PROPERTY_RTTI, 0x0123_0000);
        e.mem.write(TEXT_FUNCTION, b"Function\0");
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_1234u32.into_ret());
        e.register(VIEWER_STRING_BOOL, |_, _| 0x0e00_0002u32.into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, _| 0x0e00_0003u32.into_ret());
        e.register(COLOR_GET_VIEWER_STRING, |_, _| 0x0e00_0005u32.into_ret());
        let this = e.mem.alloc(0x2c);
        e.mem.set_u16(this + FOG_FLAGS, 0x5);
        e.mem.set_f32(this + FOG_DEPTH, 0.5);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a9_a2a0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_BOOL),
            vec![vec![TEXT_ENABLE, 0x0e00_1201]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![vec![TEXT_DEPTH, 0.5f32.to_bits()]]
        );
        // Function 2: "Function = FOG_VERTEX_ALPHA".
        let printed = calls_to(&e, NI_SPRINTF);
        assert_eq!(
            printed[0][1..],
            [8 + 0x14, FORMAT_VERTEX_ALPHA, TEXT_FUNCTION]
        );
        assert_eq!(
            calls_to(&e, COLOR_GET_VIEWER_STRING),
            vec![vec![this + FOG_COLOR, TEXT_COLOR]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 5);
    }
}
