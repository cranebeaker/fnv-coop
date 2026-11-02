//! `NiMain/nistencilproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{assign_ref_counted, ALLOCATE, OBJECT_BASE_CONSTRUCTOR};

/// `011f4a14`: the pointer `00a6f450` stores the object it builds in.
const STENCIL_PROPERTY_SLOT: u32 = 0x011f_4a14;

// Translated from 00a6f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nistencilproperty.cpp`; run by the start-up
/// `00a89cc0`, no arguments): builds a 0x24-byte object (allocation `00aa13e0`, base
/// constructor `00a5c200`, vtable `0101e07c`, the word at `+0x1c` set to 0, the word at
/// `+0x20` to -1, the word at `+0x18` to 0x4180) and stores it, counted, in `011f4a14`,
/// releasing the object held there before. A failed allocation stores null.
pub fn fn_00a6f450(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x24u32]).u32();
    let object = if block == 0 {
        0
    } else {
        e.call(OBJECT_BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, 0x0101_e07c);
        e.mem.set_u32(block + 0x1c, 0);
        e.mem.set_u32(block + 0x20, 0xffff_ffff);
        e.mem.set_u16(block + 0x18, 0x4180);
        block
    };
    assign_ref_counted(e, STENCIL_PROPERTY_SLOT, object);
}

use crate::units::nimain::nimainsdm::interlocked_decrement;

/// The vtable of `NiStencilProperty` (RTTI `.?AVNiStencilProperty@@`).
const NI_STENCIL_PROPERTY_VTABLE: u32 = 0x0101_e07c;
/// `NiStencilProperty` size (Xbox PDB and the allocations).
const NI_STENCIL_PROPERTY_SIZE: u32 = 0x24;
/// `NiObjectNET::m_spControllers` +0x0c, then `NiStencilProperty`'s
/// `m_uFlags` (16 bits) +0x18, `m_uiRef` +0x1c and `m_uiMask` +0x20 (Xbox
/// PDB names; PC offsets as the code uses them).
const CONTROLLERS: u32 = 0x0c;
const STENCIL_FLAGS: u32 = 0x18;
const STENCIL_REF: u32 = 0x1c;
const STENCIL_MASK: u32 = 0x20;
/// The flags, reference and mask a new property starts with.
const DEFAULT_FLAGS: u16 = 0x4180;
const DEFAULT_MASK: u32 = 0xffff_ffff;

/// `00a5d7a0` (a jump to `00a5c290`; `thiscall`, `(destination,
/// cloning)`): the member copy `CreateClone` runs.
const OBJECT_NET_COPY_MEMBERS: u32 = 0x00a5_d7a0;
/// `00a5d450` (unit `niobject.cpp`, `thiscall`, `(object, cloning)`): what
/// `CreateClone` runs for a property without controllers, on the property
/// itself.
const OBJECT_SHARE_CLONE: u32 = 0x00a5_d450;
/// `NiProperty::LoadBinary` (`00a5d7f0`).
const PROPERTY_LOAD_BINARY: u32 = 0x00a5_d7f0;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; PC offsets), and the input stream's position (+4) and read
/// function (+8, `cdecl` `(stream, buffer, size, &component size, component
/// count)`, returns the bytes read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
/// The first file version that stores the flags word whole.
const VERSION_WITH_PACKED_FLAGS: u32 = 0x1401_0002;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The constructor body the property's `CreateClone` and `CreateObject`
/// run on a new block: the `NiObjectNET` base constructor, the flags, the
/// vtable, reference 0 and mask -1.
fn construct_in(e: &mut Engine, object: u32) {
    e.call(OBJECT_BASE_CONSTRUCTOR, &args![object]);
    e.mem
        .set_u16(object.wrapping_add(STENCIL_FLAGS), DEFAULT_FLAGS);
    e.mem.set_u32(object, NI_STENCIL_PROPERTY_VTABLE);
    e.mem.set_u32(object.wrapping_add(STENCIL_REF), 0);
    e.mem
        .set_u32(object.wrapping_add(STENCIL_MASK), DEFAULT_MASK);
}

// Translated from 00a6f390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::CreateClone` (Xbox PDB). With controllers: a new
/// 0x24-byte property (null when the allocation fails), the member copy
/// (`00a5d7a0`) into it, then the flags, reference and mask copied; returns
/// it. Without controllers the property is its own clone: `00a5d450` runs
/// with the property and `cloning`, and the property is returned.
pub fn ni_stencil_property_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(CONTROLLERS)) == 0 {
        e.call(OBJECT_SHARE_CLONE, &args![this, this, cloning]);
        return Ptr::new(this);
    }
    let block = e.call(ALLOCATE, &args![NI_STENCIL_PROPERTY_SIZE]).u32();
    let object = if block == 0 {
        0
    } else {
        construct_in(e, block);
        block
    };
    e.call(OBJECT_NET_COPY_MEMBERS, &args![this, object, cloning]);
    let flags = e.mem.u16(this.wrapping_add(STENCIL_FLAGS));
    e.mem.set_u16(object.wrapping_add(STENCIL_FLAGS), flags);
    let reference = e.mem.u32(this.wrapping_add(STENCIL_REF));
    e.mem.set_u32(object.wrapping_add(STENCIL_REF), reference);
    let mask = e.mem.u32(this.wrapping_add(STENCIL_MASK));
    e.mem.set_u32(object.wrapping_add(STENCIL_MASK), mask);
    Ptr::new(object)
}

// Translated from 00a6f410 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiStencilProperty"` at `00a8a094`;
/// Ghidra has no function here): allocates 0x24 bytes and, when that
/// succeeds, constructs a default property in it (base constructor, flags
/// 0x4180, the vtable, reference 0, mask -1). Returns it, or 0.
pub fn ni_stencil_property_create_object(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![NI_STENCIL_PROPERTY_SIZE]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct_in(e, block);
    Ptr::new(block)
}

// Translated from 00a6f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nistencilproperty.cpp`): releases the
/// default property `00a6f450` stores in `011f4a14` (`DeleteThis` when the
/// count reaches 0) and clears the pointer; nothing when it is null.
pub fn fn_00a6f4d0(e: &mut Engine) {
    let object = e.mem.u32(STENCIL_PROPERTY_SLOT);
    if object == 0 {
        return;
    }
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
    e.mem.set_u32(STENCIL_PROPERTY_SLOT, 0);
}

// Translated from 00a6f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::LoadBinary` (Xbox PDB): `NiProperty::LoadBinary`,
/// then from file version `0x14010002` on the flags word (2 bytes), the
/// reference and the mask (4 bytes each). Older files store the fields
/// one by one and the flags word is packed from them: an enable byte (bit
/// 0), the function (bits 12-15), the reference and mask, the fail action
/// (bits 1-3), the z-fail action (bits 4-6), the pass action (bits 7-9) and
/// the draw mode (bits 10-11), each a 4-byte value shifted into place and
/// masked only by the 16-bit word.
///
/// The component sizes and buffers are the exe's stack words: the
/// argument's own word and two locals (`L0` at -8 and `L1` at -4 from the
/// return address), as the disassembly uses them.
pub fn ni_stencil_property_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PROPERTY_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    // Frame: L0 +0, L1 +4, the argument word +8.
    e.with_stack(12, |e, frame| {
        let l0 = frame.addr();
        let l1 = l0 + 4;
        let argument = l0 + 8;
        e.mem.set_u32(argument, stream);
        let flags_at = this.wrapping_add(STENCIL_FLAGS);
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= VERSION_WITH_PACKED_FLAGS {
            stream_read(e, stream, flags_at, 2, 2, argument);
            stream_read(e, stream, this.wrapping_add(STENCIL_REF), 4, 4, argument);
            stream_read(e, stream, this.wrapping_add(STENCIL_MASK), 4, 4, argument);
            return;
        }
        stream_read(e, stream, argument, 1, 1, l0);
        let flags = e.mem.u16(flags_at);
        if e.mem.u8(argument) != 0 {
            e.mem.set_u16(flags_at, flags | 1);
        } else {
            e.mem.set_u16(flags_at, flags & 0xfffe);
        }
        stream_read(e, stream, l1, 4, 4, l0);
        let packed = (e.mem.u16(flags_at) & 0x0fff) | (e.mem.u32(l1) << 12) as u16;
        e.mem.set_u16(flags_at, packed);
        stream_read(e, stream, this.wrapping_add(STENCIL_REF), 4, 4, l1);
        stream_read(e, stream, this.wrapping_add(STENCIL_MASK), 4, 4, l1);
        for (shift, keep) in [(1u32, 0xfff1u16), (4, 0xff8f), (7, 0xfc7f), (10, 0xf3ff)] {
            stream_read(e, stream, l0, 4, 4, l1);
            let value = e.mem.u32(l0).wrapping_shl(shift) as u16;
            let packed = (e.mem.u16(flags_at) & keep) | value;
            e.mem.set_u16(flags_at, packed);
        }
    });
}

/// `NiStencilProperty::ms_RTTI` (`011f4a0c`, what the class's `GetRTTI` `0049ed30` answers);
/// its first word is the class name.
const NI_STENCIL_PROPERTY_RTTI: u32 = 0x011f_4a0c;
/// The base class's `IsEqual` (`00a62140`) and `SaveBinary` (`00a5d7b0`), `__thiscall` with
/// one word (the map names the folded bodies after `NiSequenceStreamHelper`), and
/// `NiProperty::GetViewerStrings` (`00a5d850`).
const BASE_IS_EQUAL: u32 = 0x00a6_2140;
const BASE_SAVE_BINARY: u32 = 0x00a5_d7b0;
const NI_PROPERTY_GET_VIEWER_STRINGS: u32 = 0x00a5_d850;
/// `NiStream`'s output stream (+0x250) and its write function (+0xc: cdecl, stream, buffer,
/// bytes, component sizes, component count; the bytes written are added to its +0x4).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// The allocation `00aa1070` (cdecl: size) and `NiSprintf` (`00a305b0`, cdecl: buffer, size,
/// format, then the format's arguments).
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `"%s = UNKNOWN!!!"`, the format of a value outside every list.
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// `NiGetViewerString` (`00a7d330`, cdecl: a name), `NiGetViewerString_ov2` (`00a7d370`,
/// cdecl: label, a `bool` word) and `NiGetViewerString_ov8` (`00a7d510`, cdecl: label, an
/// unsigned word); the string array's `SetSize` (`0096ad30`, `__thiscall`: the new size) and
/// `SetAt` (`0096ae90`, `__thiscall`: index, address of the element).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The labels `GetViewerStrings` passes: `"Enabled"`, `"m_uiRef"`, `"m_uiMask"`,
/// `"Test Funtion"` (sic), `"Test Fail Action"`, `"Depth Fail Action"`, `"Test Pass Action"`
/// and `"Draw Mode"`.
const LABEL_ENABLED: u32 = 0x0109_c7d8;
const LABEL_REF: u32 = 0x0109_cb7c;
const LABEL_MASK: u32 = 0x0109_cb70;
const LABEL_TEST_FUNCTION: u32 = 0x0109_cb60;
const LABEL_TEST_FAIL_ACTION: u32 = 0x0109_cb4c;
const LABEL_DEPTH_FAIL_ACTION: u32 = 0x0109_cb38;
const LABEL_TEST_PASS_ACTION: u32 = 0x0109_cb24;
const LABEL_DRAW_MODE: u32 = 0x0109_cb18;

// Translated from 00a6f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::IsEqual` (Xbox PDB): the base class's comparison (`00a62140`), then the
/// same flags word (+0x18), reference (+0x1c) and mask (+0x20).
pub fn ni_stencil_property_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (a, b) = (this.addr(), other.addr());
    e.mem.u16(b + STENCIL_FLAGS) == e.mem.u16(a + STENCIL_FLAGS)
        && e.mem.u32(b + STENCIL_REF) == e.mem.u32(a + STENCIL_REF)
        && e.mem.u32(b + STENCIL_MASK) == e.mem.u32(a + STENCIL_MASK)
}

/// The shared body of the three static `GetViewerString`s: a block of the name's length plus
/// 0x16 bytes (`00aa1070`), `NiSprintf` into it with the case's format (or
/// `"%s = UNKNOWN!!!"` for a value outside the cases) and the name; answers the block.
fn viewer_string(e: &mut Engine, name: Ptr, format: Option<u32>) -> Ptr {
    let size = (e.mem.cstr(name.addr()).len() as u32).wrapping_add(0x16);
    let text = e.call(ALLOCATE_00AA1070, &args![size]).u32();
    let format = format.unwrap_or(FORMAT_UNKNOWN);
    e.call(NI_SPRINTF, &args![text, size, format, name]);
    Ptr::new(text)
}

// Translated from 00a6f0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::GetViewerString` (Xbox PDB; static, cdecl: name, test function): a new
/// string `"<name> = TEST_<function>"` for functions 0..7 (NEVER, LESS, EQUAL, LESSEQUAL,
/// GREATER, NOTEQUAL, GREATEREQUAL, ALWAYS; the compiler's jump table), or
/// `"<name> = UNKNOWN!!!"`.
pub fn ni_stencil_property_get_viewer_string(e: &mut Engine, name: Ptr, function: u32) -> Ptr {
    let format = match function {
        0 => Some(0x0109_b3ec),
        1 => Some(0x0109_b460),
        2 => Some(0x0109_b450),
        3 => Some(0x0109_b43c),
        4 => Some(0x0109_b428),
        5 => Some(0x0109_b414),
        6 => Some(0x0109_b3fc),
        7 => Some(0x0109_b470),
        _ => None,
    };
    viewer_string(e, name, format)
}

// Translated from 00a6f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::GetViewerString_ov2` (Xbox PDB; static, cdecl: name, action): a new
/// string `"<name> = ACTION_<action>"` for actions 0..5 (KEEP, ZERO, REPLACE, INCREMENT,
/// DECREMENT, INVERT; the compiler's jump table), or `"<name> = UNKNOWN!!!"`.
pub fn ni_stencil_property_get_viewer_string_ov2(e: &mut Engine, name: Ptr, action: u32) -> Ptr {
    let format = match action {
        0 => Some(0x0109_cabc),
        1 => Some(0x0109_caa8),
        2 => Some(0x0109_ca94),
        3 => Some(0x0109_ca7c),
        4 => Some(0x0109_ca64),
        5 => Some(0x0109_ca50),
        _ => None,
    };
    viewer_string(e, name, format)
}

// Translated from 00a6f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::GetViewerString_ov3` (Xbox PDB; static, cdecl: name, draw mode): a new
/// string `"<name> = DRAW_<mode>"` for modes 0..3 (CCW_OR_BOTH, CCW, CW, BOTH; the compiler's
/// jump table), or `"<name> = UNKNOWN!!!"`.
pub fn ni_stencil_property_get_viewer_string_ov3(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let format = match mode {
        0 => Some(0x0109_cb00),
        1 => Some(0x0109_caf0),
        2 => Some(0x0109_cae0),
        3 => Some(0x0109_cad0),
        _ => None,
    };
    viewer_string(e, name, format)
}

/// One write through the stream's output: `size` bytes from `buffer` as one component of
/// `size` bytes (the size word on the stack); the position moves on by what was written.
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        let bytes = e
            .call(write, &args![output, buffer, size, slot, 1u32])
            .u32();
        let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
        e.mem.set_u32(
            output.wrapping_add(BINARY_STREAM_POSITION),
            position.wrapping_add(bytes),
        );
    });
}

// Translated from 00a6f740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::SaveBinary` (Xbox PDB): the base class's save (`00a5d7b0`), then the
/// flags word (2 bytes), the reference and the mask (4 bytes each).
pub fn ni_stencil_property_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let (this, stream) = (this.addr(), stream.addr());
    stream_write(e, stream, this.wrapping_add(STENCIL_FLAGS), 2);
    stream_write(e, stream, this.wrapping_add(STENCIL_REF), 4);
    stream_write(e, stream, this.wrapping_add(STENCIL_MASK), 4);
}

/// The inlined `NiTArray<char *>::Add` through the stack word at `slot` (the code's own
/// argument word): `value` stored there, the array grown to its size plus `m_usGrowBy`
/// (+0xe) when the size (+0xa) has reached the capacity (+0x8), then `SetAt` at the old size.
fn append(e: &mut Engine, strings: u32, slot: u32, value: u32) {
    e.mem.set_u32(slot, value);
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    let capacity = u32::from(e.mem.u16(strings.wrapping_add(0x8)));
    if size >= capacity {
        let grow_by = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow_by + size]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a6f7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiStencilProperty::GetViewerStrings` (Xbox PDB): `NiProperty`'s strings (`00a5d850`), then
/// appended in order: the class name (`NiGetViewerString` of `ms_RTTI`'s first word),
/// `"Enabled"` (flag bit 0), `"m_uiRef"`, `"m_uiMask"`, the test function (flag bits 12-15),
/// the fail, depth-fail and pass actions (bits 1-3, 4-6, 7-9) and the draw mode (bits 10-11).
/// Every string goes through the argument's stack word; the `bool` word for `"Enabled"` is
/// that word with its low byte replaced (the upper bytes are the class name string's), as the
/// code builds it.
pub fn ni_stencil_property_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_PROPERTY_GET_VIEWER_STRINGS, &args![this, strings]);
    let (this, strings) = (this.addr(), strings.addr());
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.mem.u32(NI_STENCIL_PROPERTY_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        append(e, strings, slot, text);
        let enabled = (e.mem.u32(slot) & 0xffff_ff00)
            | u32::from(e.mem.u8(this.wrapping_add(STENCIL_FLAGS)) & 1);
        let text = e
            .call(NI_GET_VIEWER_STRING_BOOL, &args![LABEL_ENABLED, enabled])
            .u32();
        append(e, strings, slot, text);
        let reference = e.mem.u32(this.wrapping_add(STENCIL_REF));
        let text = e
            .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![LABEL_REF, reference])
            .u32();
        append(e, strings, slot, text);
        let mask = e.mem.u32(this.wrapping_add(STENCIL_MASK));
        let text = e
            .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![LABEL_MASK, mask])
            .u32();
        append(e, strings, slot, text);
        let flags = |e: &Engine| u32::from(e.mem.u16(this.wrapping_add(STENCIL_FLAGS)));
        let function = flags(e) >> 12;
        let text =
            ni_stencil_property_get_viewer_string(e, Ptr::new(LABEL_TEST_FUNCTION), function);
        append(e, strings, slot, text.addr());
        for (label, shift) in [
            (LABEL_TEST_FAIL_ACTION, 1u32),
            (LABEL_DEPTH_FAIL_ACTION, 4),
            (LABEL_TEST_PASS_ACTION, 7),
        ] {
            let action = (flags(e) >> shift) & 7;
            let text = ni_stencil_property_get_viewer_string_ov2(e, Ptr::new(label), action);
            append(e, strings, slot, text.addr());
        }
        let mode = (flags(e) >> 10) & 3;
        let text = ni_stencil_property_get_viewer_string_ov3(e, Ptr::new(LABEL_DRAW_MODE), mode);
        append(e, strings, slot, text.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a6_f450, fn_00a6f450()),
        entry!(0x00a6_f390, ni_stencil_property_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a6_f410, ni_stencil_property_create_object() -> Ptr),
        entry!(0x00a6_f4d0, fn_00a6f4d0()),
        entry!(0x00a6_f500, ni_stencil_property_load_binary(Ptr, Ptr)),
        entry!(0x00a6_f070, ni_stencil_property_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a6_f0b0, ni_stencil_property_get_viewer_string(Ptr, u32) -> Ptr),
        entry!(0x00a6_f1e0, ni_stencil_property_get_viewer_string_ov2(Ptr, u32) -> Ptr),
        entry!(0x00a6_f2d0, ni_stencil_property_get_viewer_string_ov3(Ptr, u32) -> Ptr),
        entry!(0x00a6_f740, ni_stencil_property_save_binary(Ptr, Ptr)),
        entry!(
            0x00a6_f7d0,
            ni_stencil_property_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn builds_the_object_and_replaces_the_held_one() {
        let run = run_default_instance(0x00a6_f450, STENCIL_PROPERTY_SLOT, |_| {});
        let e = &run.engine;
        assert_eq!(e.mem.u32(run.object), 0x0101_e07c);
        assert_eq!(e.mem.u32(run.object + 0x1c), 0);
        assert_eq!(e.mem.u32(run.object + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u16(run.object + 0x18), 0x4180);
        assert_eq!(e.mem.u32(run.object + 4), 1);
        assert_eq!(*run.freed.borrow(), vec![run.previous]);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let run = run_default_instance(0x00a6_f450, STENCIL_PROPERTY_SLOT, |e| {
            e.register(ALLOCATE, |_, _| 0u32.into_ret());
        });
        assert_eq!(run.object, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const READ: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    #[test]
    fn create_object_builds_a_default_property() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x24);
        logged(&mut e, &log, ALLOCATE, block);
        logged(&mut e, &log, OBJECT_BASE_CONSTRUCTOR, block);
        assert_eq!(e.call(0x00a6_f410, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_STENCIL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(block + STENCIL_FLAGS), 0x4180);
        assert_eq!(e.mem.u32(block + STENCIL_REF), 0);
        assert_eq!(e.mem.u32(block + STENCIL_MASK), 0xffff_ffff);
        logged(&mut e, &log, ALLOCATE, 0);
        assert_eq!(e.call(0x00a6_f410, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_a_controlled_property_and_shares_a_plain_one() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x24);
        logged(&mut e, &log, ALLOCATE, block);
        logged(&mut e, &log, OBJECT_BASE_CONSTRUCTOR, block);
        logged(&mut e, &log, OBJECT_NET_COPY_MEMBERS, 0);
        logged(&mut e, &log, OBJECT_SHARE_CLONE, 0);
        let this = e.mem.alloc(0x24);
        e.mem.set_u32(this + CONTROLLERS, 0x7000);
        e.mem.set_u16(this + STENCIL_FLAGS, 0x1234);
        e.mem.set_u32(this + STENCIL_REF, 5);
        e.mem.set_u32(this + STENCIL_MASK, 0xff);
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(
            log.borrow()[2],
            (OBJECT_NET_COPY_MEMBERS, vec![this, block, 0x9000])
        );
        assert_eq!(e.mem.u32(block), NI_STENCIL_PROPERTY_VTABLE);
        assert_eq!(e.mem.u16(block + STENCIL_FLAGS), 0x1234);
        assert_eq!(e.mem.u32(block + STENCIL_REF), 5);
        assert_eq!(e.mem.u32(block + STENCIL_MASK), 0xff);
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        logged(&mut e, &log, ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(
            log.borrow()[1],
            (OBJECT_NET_COPY_MEMBERS, vec![this, 0, 0x9000])
        );
        assert_eq!(e.mem.u32(STENCIL_MASK), 0xff);
        // No controllers: the property is its own clone.
        e.mem.set_u32(this + CONTROLLERS, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a6_f390, &args![this, 0x9000u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            [(OBJECT_SHARE_CLONE, vec![this, this, 0x9000])]
        );
    }

    #[test]
    fn fn_00a6f4d0_releases_the_default_property() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, DELETE_THIS, 0);
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.map(STENCIL_PROPERTY_SLOT, 4);
        e.call(0x00a6_f4d0, &args![]);
        assert!(log.borrow().is_empty());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, 2);
        e.mem.set_u32(STENCIL_PROPERTY_SLOT, object);
        e.call(0x00a6_f4d0, &args![]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(object + 4), 1);
        assert_eq!(e.mem.u32(STENCIL_PROPERTY_SLOT), 0);
        e.mem.set_u32(STENCIL_PROPERTY_SLOT, object);
        e.call(0x00a6_f4d0, &args![]);
        assert_eq!(*log.borrow(), [(DELETE_THIS, vec![object])]);
    }

    /// `LoadBinary` on a stream of file `version` whose reads hand out
    /// `words` in turn (each read writes its size in bytes of the word).
    fn load(version: u32, words: &'static [u32], flags: u16) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, PROPERTY_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[1], a[2], e.mem.u32(a[3]), a[4]]));
            let word = *next.next().unwrap();
            e.mem.write(a[1], &word.to_le_bytes()[..a[2] as usize]);
            ret(a[2])
        });
        let this = e.mem.alloc(0x24);
        e.mem.set_u16(this + STENCIL_FLAGS, flags);
        e.call(0x00a6_f500, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_packed_flags_of_new_files() {
        let (e, log, this, stream) = load(0x1401_0002, &[0xbeef, 3, 0x7f], 0);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), 0xbeef);
        assert_eq!(e.mem.u32(this + STENCIL_REF), 3);
        assert_eq!(e.mem.u32(this + STENCIL_MASK), 0x7f);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 10);
        let reads: Vec<Vec<u32>> = log.borrow()[1..].iter().map(|c| c.1.clone()).collect();
        assert_eq!(
            reads,
            [
                vec![this + STENCIL_FLAGS, 2, 2, 1],
                vec![this + STENCIL_REF, 4, 4, 1],
                vec![this + STENCIL_MASK, 4, 4, 1],
            ]
        );
    }

    #[test]
    fn load_binary_packs_the_fields_of_old_files() {
        // enable, function 5, ref 2, mask 0xff, fail 3, z-fail 6, pass 1,
        // draw mode 2.
        let (e, log, this, _) = load(0x1401_0001, &[1, 5, 2, 0xff, 3, 6, 1, 2], 0x0000);
        let expected = 1 | (5 << 12) | (3 << 1) | (6 << 4) | (1 << 7) | (2 << 10);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), expected);
        assert_eq!(e.mem.u32(this + STENCIL_REF), 2);
        assert_eq!(e.mem.u32(this + STENCIL_MASK), 0xff);
        assert_eq!(log.borrow().len(), 9);
        assert_eq!(log.borrow()[1].1[1..], [1, 1, 1]);
        // A zero enable byte clears bit 0; values wider than their field
        // spill into the bits above, as the exe's shifts and ORs do.
        let (e, _, this, _) = load(0x0400_0002, &[0, 0, 0, 0, 0, 0, 0, 0x3f], 0xffff);
        assert_eq!(e.mem.u16(this + STENCIL_FLAGS), 0xfc00);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn name(e: &mut Engine, text: &[u8]) -> u32 {
        let p = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(p, text);
        p
    }

    #[test]
    fn is_equal_compares_flags_reference_and_mask() {
        let mut e = Engine::new();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        assert!(e.call(0x00a6_f070, &args![a, b]).bool());
        for (offset, wide) in [
            (STENCIL_FLAGS, false),
            (STENCIL_REF, true),
            (STENCIL_MASK, true),
        ] {
            if wide {
                e.mem.set_u32(b + offset, 0x100);
            } else {
                e.mem.set_u16(b + offset, 0x100);
            }
            assert!(!e.call(0x00a6_f070, &args![a, b]).bool());
            e.mem.set_u32(b + offset, 0);
        }
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a6_f070, &args![a, b]).bool());
    }

    #[test]
    fn the_static_viewer_strings_pick_the_case_format() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x40);
        e.register_double(ALLOCATE_00AA1070, move |_, _| word(block));
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let label = name(&mut e, b"Mode");
        e.call_log = Some(vec![]);
        let cases: [(u32, u32, u32); 6] = [
            (0x00a6_f0b0, 0, 0x0109_b3ec),
            (0x00a6_f0b0, 7, 0x0109_b470),
            (0x00a6_f0b0, 8, FORMAT_UNKNOWN),
            (0x00a6_f1e0, 5, 0x0109_ca50),
            (0x00a6_f1e0, 6, FORMAT_UNKNOWN),
            (0x00a6_f2d0, 2, 0x0109_cae0),
        ];
        for (function, value, format) in cases {
            assert_eq!(e.call(function, &args![label, value]).u32(), block);
            let printed = calls_to(&e, NI_SPRINTF);
            assert_eq!(
                printed[printed.len() - 1],
                vec![block, 4 + 0x16, format, label]
            );
        }
        assert_eq!(calls_to(&e, ALLOCATE_00AA1070)[0], vec![4 + 0x16]);
        e.call(0x00a6_f2d0, &args![label, 4u32]);
        assert_eq!(calls_to(&e, NI_SPRINTF)[6][2], FORMAT_UNKNOWN);
        e.call(0x00a6_f1e0, &args![label, 1u32]);
        assert_eq!(calls_to(&e, NI_SPRINTF)[7][2], 0x0109_caa8);
        e.call(0x00a6_f0b0, &args![label, 3u32]);
        assert_eq!(calls_to(&e, NI_SPRINTF)[8][2], 0x0109_b43c);
    }

    #[test]
    fn save_binary_writes_flags_reference_and_mask() {
        let mut e = Engine::new();
        const WRITE_DOUBLE: u32 = 0x0070_a000;
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        let writes: Rc<RefCell<Vec<(Vec<u8>, u32)>>> = Rc::new(RefCell::new(vec![]));
        let log = writes.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            log.borrow_mut()
                .push((e.mem.bytes(a[1], a[2]), e.mem.u32(a[3])));
            word(a[2])
        });
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE_DOUBLE);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let p = e.mem.alloc(0x24);
        e.mem.set_u16(p + STENCIL_FLAGS, 0x4180);
        e.mem.set_u32(p + STENCIL_REF, 3);
        e.mem.set_u32(p + STENCIL_MASK, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x00a6_f740, &args![p, stream]);
        assert_eq!(calls_to(&e, BASE_SAVE_BINARY), vec![vec![p, stream]]);
        let expected = vec![
            (vec![0x80, 0x41], 2),
            (vec![3, 0, 0, 0], 4),
            (vec![0xff, 0, 0, 0], 4),
        ];
        assert_eq!(*writes.borrow(), expected);
        assert_eq!(e.mem.u32(output + 4), 10);
    }

    #[test]
    fn get_viewer_strings_lists_every_field() {
        let mut e = Engine::new();
        e.map(NI_STENCIL_PROPERTY_RTTI, 8);
        e.mem.set_u32(NI_STENCIL_PROPERTY_RTTI, 0x0109_cb90);
        e.map(0x0109_cb00, 0x80);
        for (label, text) in [
            (LABEL_TEST_FUNCTION, &b"Test Funtion"[..]),
            (LABEL_TEST_FAIL_ACTION, b"Test Fail Action"),
            (LABEL_DEPTH_FAIL_ACTION, b"Depth Fail Action"),
            (LABEL_TEST_PASS_ACTION, b"Test Pass Action"),
            (LABEL_DRAW_MODE, b"Draw Mode"),
        ] {
            e.mem.set_cstr(label, text);
        }
        e.register(NI_PROPERTY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x1234_5600));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| word(a[1]));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, a| word(a[1]));
        // The static strings run for real: the block is the value the format was chosen for.
        e.register(ALLOCATE_00AA1070, |e, a| word(e.mem.alloc(a[0])));
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let log = stored.clone();
        e.register_double(STRING_ARRAY_SET_AT, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[2]));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        let p = e.mem.alloc(0x24);
        // Function 5, fail 2, depth fail 3, pass 4, draw mode 1, enabled.
        let flags: u16 = (5 << 12) | (1 << 10) | (4 << 7) | (3 << 4) | (2 << 1) | 1;
        e.mem.set_u16(p + STENCIL_FLAGS, flags);
        e.mem.set_u32(p + STENCIL_REF, 7);
        e.mem.set_u32(p + STENCIL_MASK, 0xf0);
        e.call_log = Some(vec![]);
        e.call(0x00a6_f7d0, &args![p, strings]);
        assert_eq!(
            calls_to(&e, NI_PROPERTY_GET_VIEWER_STRINGS),
            vec![vec![p, strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_cb90]]);
        // The bool word keeps the class name string's upper bytes.
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_BOOL),
            vec![vec![LABEL_ENABLED, 0x1234_5601]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![LABEL_REF, 7], vec![LABEL_MASK, 0xf0]]
        );
        let formats: Vec<(u32, u32)> = calls_to(&e, NI_SPRINTF)
            .iter()
            .map(|c| (c[3], c[2]))
            .collect();
        assert_eq!(
            formats,
            vec![
                (LABEL_TEST_FUNCTION, 0x0109_b414),
                (LABEL_TEST_FAIL_ACTION, 0x0109_ca94),
                (LABEL_DEPTH_FAIL_ACTION, 0x0109_ca7c),
                (LABEL_TEST_PASS_ACTION, 0x0109_ca64),
                (LABEL_DRAW_MODE, 0x0109_caf0),
            ]
        );
        assert_eq!(stored.borrow().len(), 9);
        assert_eq!(stored.borrow()[0], 0x1234_5600);
        assert_eq!(stored.borrow()[2], 7);
        // Grown at 0, 4 and 8 by `m_usGrowBy` 4.
        assert_eq!(
            calls_to(&e, STRING_ARRAY_SET_SIZE),
            vec![vec![strings, 4], vec![strings, 8], vec![strings, 12]]
        );
    }
}
