//! `NiParticle/nipsysmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `NiPSysModifier` base: its layout and the stream and string helpers
//! the other particle units of this crate share.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPSysModifier` (Xbox PDB), 0x18 bytes; the PC code uses the same
    /// offsets.
    pub struct NiPSysModifier: 0x18 {
        /// `m_kName` (Xbox PDB): an `NiFixedString` handle.
        0x08 m_kName: u32,
        /// `m_uiOrder` (Xbox PDB).
        0x0C m_uiOrder: u32,
        /// `m_pkTarget` (Xbox PDB): `NiParticleSystem*`.
        0x10 m_pkTarget: u32,
        /// `m_bActive` (Xbox PDB).
        0x14 m_bActive: bool,
    }
}

/// The vtable of `NiPSysModifier` (RTTI `.?AVNiPSysModifier@@`).
const VTABLE: u32 = 0x010c_09cc;

/// The global holding the null `NiFixedString` handle (`0109b220`): the
/// code compares and stores it rather than a literal 0.
pub(crate) const FIXED_STRING_NULL: u32 = 0x0109_b220;

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
pub(crate) const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
pub(crate) const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; offsets as the PC code reads them), and the input stream's
/// position (+4) and read function (+8, `cdecl` `(stream, buffer, size,
/// &component size, component count)`, returns the bytes read).
pub(crate) const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// `NiObject::NiObject` (`00a5d3a0`, `thiscall`, no stack words).
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// `NiObject::~NiObject` body (`00a5d3d0`, `thiscall`).
const NI_OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `NiObject::CopyMembers` (`00a5d450`, `thiscall`, `(dest, process)`).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `NiObject::ProcessClone` (`00a5d430`, `thiscall`, one word).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiObject::LoadBinary` (`00a5d610`, `thiscall`, one word).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `00a89af0` (unnamed in the engine map): `thiscall`, one word, the base
/// link step `NiPSysModifier::LinkObject` runs first.
const BASE_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, `thiscall` on the stream).
pub(crate) const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00a53c30` (unnamed): the cloning process's map lookup, `thiscall` on
/// the process's first word, `(key, &value)`.
pub(crate) const CLONE_MAP_LOOKUP: u32 = 0x00a5_3c30;
/// `NiStream::LoadCStringAsFixedString` (`00a64ea0`) and
/// `NiStream::LoadFixedString` (`00a64df0`): `thiscall` on the stream, one
/// word (the string's address).
pub(crate) const LOAD_CSTRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
pub(crate) const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
/// `00a645b0` (unnamed, `NiMain/nistream.cpp`): `thiscall` on the stream,
/// no stack words; it reads a link id into the stream's list.
pub(crate) const READ_LINK_ID: u32 = 0x00a6_45b0;
/// The first file version whose names are fixed strings
/// (`CMP [stream+0xd8], 0x14010001`).
pub(crate) const VERSION_FIXED_STRINGS: u32 = 0x1401_0001;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
pub(crate) fn stream_read(
    e: &mut Engine,
    stream: u32,
    buffer: u32,
    size: u32,
    component: u32,
    slot: u32,
) {
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

/// The `NiFixedString` assignment the code inlines: when `dest`'s handle
/// differs from `source`, `source` is held (`InterlockedIncrement` on its
/// word at -8) unless it is the null handle, the old handle released
/// (`InterlockedDecrement`) unless null, and `source` stored.
pub(crate) fn fixed_string_assign(e: &mut Engine, dest: u32, source: u32) {
    if e.mem.u32(dest) == source {
        return;
    }
    if source != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_INCREMENT, &args![source.wrapping_sub(8)]);
    }
    let old = e.mem.u32(dest);
    if old != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(dest, source);
}

/// The `NiFixedString` destructor the code inlines: the handle at `string`
/// is released unless it is the null handle.
pub(crate) fn fixed_string_release(e: &mut Engine, string: u32) {
    let handle = e.mem.u32(string);
    if handle != e.global::<u32>(FIXED_STRING_NULL) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The name read the modifier and controller `LoadBinary`s share: files
/// before 20.1.0.1 store a C string (`LoadCStringAsFixedString`), later ones
/// a fixed string (`LoadFixedString`), into the string at `string`.
pub(crate) fn load_name(e: &mut Engine, stream: u32, string: u32) {
    let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
    if version < VERSION_FIXED_STRINGS {
        e.call(LOAD_CSTRING_AS_FIXED_STRING, &args![stream, string]);
    } else {
        e.call(LOAD_FIXED_STRING, &args![stream, string]);
    }
}

// Translated from 00c31650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::LinkObject` (Xbox PDB): the base link step `00a89af0`
/// on `this` with the stream, then `NiStream::GetObjectFromLinkID` on the
/// stream; its result is not stored (the exe drops `EAX`).
pub fn ni_p_sys_modifier_link_object(e: &mut Engine, this: Ptr<NiPSysModifier>, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]);
}

// Translated from 00c31720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifier` constructor (it stores
/// the vtable `010c09cc`, RTTI `.?AVNiPSysModifier@@`). `NiObject`'s
/// constructor, then the null name handle (the word at `0109b220`), order
/// and target 0, active. Returns `this`.
pub fn fn_00c31720(e: &mut Engine, this: Ptr<NiPSysModifier>) -> Ptr<NiPSysModifier> {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let null = e.global::<u32>(FIXED_STRING_NULL);
    e.set(this, NiPSysModifier::m_kName, null);
    e.set(this, NiPSysModifier::m_uiOrder, 0);
    e.set(this, NiPSysModifier::m_pkTarget, 0);
    e.set(this, NiPSysModifier::m_bActive, true);
    this
}

// Translated from 00c31750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::~NiPSysModifier` (Xbox PDB): the vtable back to
/// `NiPSysModifier`'s, the name released unless it is the null handle, then
/// `NiObject`'s destructor body (`00a5d3d0`, a tail jump).
pub fn ni_p_sys_modifier_destructor(e: &mut Engine, this: Ptr<NiPSysModifier>) {
    e.mem.set_u32(this.addr(), VTABLE);
    fixed_string_release(e, this.addr() + NiPSysModifier::m_kName.off);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
}

// Translated from 00c31780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifier` member copy into `dest`
/// (`thiscall`, `RET 8`: `dest`, cloning process). `NiObject::CopyMembers`,
/// then the name (fixed-string assignment), the order and the active flag;
/// the target is not copied.
pub fn fn_00c31780(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    dest: Ptr<NiPSysModifier>,
    process: Ptr,
) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, dest, process]);
    let name = e.get(this, NiPSysModifier::m_kName);
    fixed_string_assign(e, dest.addr() + NiPSysModifier::m_kName.off, name);
    let order = e.get(this, NiPSysModifier::m_uiOrder);
    e.set(dest, NiPSysModifier::m_uiOrder, order);
    let active = e.mem.u8(this.addr() + NiPSysModifier::m_bActive.off);
    e.mem
        .set_u8(dest.addr() + NiPSysModifier::m_bActive.off, active);
}

// Translated from 00c317e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`, then
/// the clone of `this` and the clone of its target from the cloning
/// process's map (`00a53c30` on the process's first word, both into one
/// stack word cleared once before the first lookup); the target's clone
/// becomes the clone's target.
pub fn ni_p_sys_modifier_process_clone(e: &mut Engine, this: Ptr<NiPSysModifier>, process: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = e.mem.u32(slot.addr());
        let target = e.get(this, NiPSysModifier::m_pkTarget);
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_LOOKUP, &args![map, target, slot]);
        let target_clone = e.mem.u32(slot.addr());
        e.mem.set_u32(
            clone.wrapping_add(NiPSysModifier::m_pkTarget.off),
            target_clone,
        );
    });
}

// Translated from 00c318b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the
/// name (a C string before file version 20.1.0.1, a fixed string from it),
/// the order (4 bytes), a link id (`00a645b0`, the target), then one byte
/// read into the low byte of the stack word that holds the stream argument;
/// the active flag is whether that byte is non-zero.
pub fn ni_p_sys_modifier_load_binary(e: &mut Engine, this: Ptr<NiPSysModifier>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    load_name(e, stream.addr(), this.addr() + NiPSysModifier::m_kName.off);
    e.with_stack(8, |e, frame| {
        let size_slot = frame.addr();
        let argument_slot = frame.addr() + 4;
        stream_read(
            e,
            stream.addr(),
            this.addr() + NiPSysModifier::m_uiOrder.off,
            4,
            4,
            size_slot,
        );
        e.call(READ_LINK_ID, &args![stream]);
        e.mem.set_u32(argument_slot, stream.addr());
        stream_read(e, stream.addr(), argument_slot, 1, 1, size_slot);
        let active = e.mem.u8(argument_slot) != 0;
        e.set(this, NiPSysModifier::m_bActive, active);
    });
}

// Translated from 00c31710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27d0.
pub fn ni_p_sys_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27d0
}

/// `NiStream::m_pkOstr` (Xbox PDB name; +0x250 as the PC code reads it) and
/// the output stream's write function (+0xc, `cdecl` `(stream, buffer, size,
/// &component size, component count)`, returns the bytes written).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiObject::RegisterStreamables` (`00a5d2f0`, `thiscall`, one word).
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
/// `NiObject::IsEqual` (`00a5d470`, `thiscall`, one word).
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
/// `NiObject::GetViewerStrings` (`00a5d6e0`, `thiscall`, one word).
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `00a89af0` (unnamed, 3 bytes): the base save step `SaveBinary` runs
/// first (`thiscall`, one word); the same body `LinkObject` runs.
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `NiStream::SaveFixedString` (`00a641f0`, `thiscall` on the stream, one
/// word: the string's address).
pub(crate) const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
/// `NiStream` vtable slots: register a fixed string (+0x20, one word: the
/// string's address) and save a link id (+0x30, one word: the object).
pub(crate) const STREAM_REGISTER_FIXED_STRING: u32 = 0x20;
pub(crate) const STREAM_SAVE_LINK_ID: u32 = 0x30;
/// `00aa1460` (`cdecl`, `(block, size)`): the sized Gamebryo delete.
pub(crate) const NI_DELETE: u32 = 0x00aa_1460;
/// The `NiRTTI` record of `NiPSysModifier` (`012027d0`): its first word is
/// the class name.
const RTTI: u32 = 0x0120_27d0;
/// `NiGetViewerString` overloads (`cdecl`): the plain one (one word: the
/// text), `_ov2` (label, `bool` byte zero-extended), `_ov6` (label, `u16`
/// zero-extended), `_ov8` (label, `u32`), `_ov9` (label, `float`) and
/// `_ov11` (label, `NiFixedString` handle). Each returns the new string.
pub(crate) const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
pub(crate) const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
pub(crate) const NI_GET_VIEWER_STRING_U16: u32 = 0x00a7_d490;
pub(crate) const NI_GET_VIEWER_STRING_U32: u32 = 0x00a7_d510;
pub(crate) const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
pub(crate) const NI_GET_VIEWER_STRING_FIXED: u32 = 0x00a7_d5e0;
/// The viewer-string array's grow (`0096ad30`, `thiscall`, one word: the new
/// capacity) and store (`0096ae90`, `thiscall`, `(index, &string)`).
pub(crate) const STRINGS_GROW: u32 = 0x0096_ad30;
pub(crate) const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Labels of `NiPSysModifier::GetViewerStrings`: `"Name"`, `"Order"`,
/// `"Active"`.
const LABEL_NAME: u32 = 0x0105_ac4c;
const LABEL_ORDER: u32 = 0x010c_0a70;
const LABEL_ACTIVE: u32 = 0x0109_c938;

/// One write through the stream's write function from `buffer`: the word
/// at `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes written.
pub(crate) fn stream_write(
    e: &mut Engine,
    stream: u32,
    buffer: u32,
    size: u32,
    component: u32,
    slot: u32,
) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.mem.set_u32(slot, component);
    let bytes = e
        .call(write, &args![output, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The append to a viewer-string array the `GetViewerStrings` methods
/// inline: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then `text`, kept in a
/// stack word, is stored at the old size.
pub(crate) fn add_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// `NiGetViewerString` on the class name of the `NiRTTI` record at `rtti`
/// (its first word), appended to `strings`.
pub(crate) fn add_class_name(e: &mut Engine, strings: u32, rtti: u32) {
    let name = e.global::<u32>(rtti);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
}

/// One labelled value through the `NiGetViewerString` overload at
/// `overload`, appended to `strings`.
pub(crate) fn add_labelled(e: &mut Engine, strings: u32, overload: u32, label: u32, value: u32) {
    let text = e.call(overload, &args![label, value]).u32();
    add_viewer_string(e, strings, text);
}

// Translated from 00c31670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::RegisterStreamables` (Xbox PDB): `NiObject`'s
/// `RegisterStreamables`; when it answers false, false. Otherwise the name
/// is registered with the stream (slot +0x20, the name's address) and the
/// answer is true.
pub fn ni_p_sys_modifier_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    stream: Ptr,
) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    e.vcall(
        stream.addr(),
        STREAM_REGISTER_FIXED_STRING,
        &args![this.addr() + NiPSysModifier::m_kName.off],
    );
    true
}

// Translated from 00c316a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysModifier` constructor with a name
/// and an order (`thiscall`, `RET 8`: the address of the name's handle, the
/// order). `NiObject`'s constructor, the vtable `010c09cc`, the null name
/// handle, the order, target 0, active; then the name is assigned (held
/// unless null, the old one released unless null). Returns `this`.
pub fn fn_00c316a0(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    name: Ptr,
    order: u32,
) -> Ptr<NiPSysModifier> {
    e.call(NI_OBJECT_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let null = e.global::<u32>(FIXED_STRING_NULL);
    e.set(this, NiPSysModifier::m_kName, null);
    e.set(this, NiPSysModifier::m_uiOrder, order);
    e.set(this, NiPSysModifier::m_pkTarget, 0);
    e.set(this, NiPSysModifier::m_bActive, true);
    let handle = e.mem.u32(name.addr());
    fixed_string_assign(e, this.addr() + NiPSysModifier::m_kName.off, handle);
    this
}

// Translated from 00c31830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::IsEqual` (Xbox PDB): `NiObject::IsEqual`, then the same
/// name handle, order and active byte.
pub fn ni_p_sys_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    other: Ptr<NiPSysModifier>,
) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.get(this, NiPSysModifier::m_kName) == e.get(other, NiPSysModifier::m_kName)
        && e.get(this, NiPSysModifier::m_uiOrder) == e.get(other, NiPSysModifier::m_uiOrder)
        && e.mem.u8(this.addr() + NiPSysModifier::m_bActive.off)
            == e.mem.u8(other.addr() + NiPSysModifier::m_bActive.off)
}

// Translated from 00c31870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPSysModifier` deleting destructor (unnamed in the engine map;
/// `RET 4`): the destructor inlined (the vtable back to `NiPSysModifier`'s,
/// the name released unless null, `NiObject`'s destructor body `00a5d3d0`),
/// then, with bit 0 of `flags`, the sized delete of 0x18 bytes. Returns
/// `this`.
pub fn fn_00c31870(e: &mut Engine, this: Ptr<NiPSysModifier>, flags: u32) -> Ptr<NiPSysModifier> {
    e.mem.set_u32(this.addr(), VTABLE);
    fixed_string_release(e, this.addr() + NiPSysModifier::m_kName.off);
    e.call(NI_OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00c31950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::SaveBinary` (Xbox PDB): the base save step
/// (`00a89af0`), the name (`NiStream::SaveFixedString`), the order (4
/// bytes; its component size kept in the stack word that held the stream
/// argument), the target's link id (stream slot +0x30), then the active
/// flag: one byte written from the low byte of that argument word (its
/// component size in the function's own stack word).
pub fn ni_p_sys_modifier_save_binary(e: &mut Engine, this: Ptr<NiPSysModifier>, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(
        SAVE_FIXED_STRING,
        &args![stream, this.addr() + NiPSysModifier::m_kName.off],
    );
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, stream.addr());
        stream_write(
            e,
            stream.addr(),
            this.addr() + NiPSysModifier::m_uiOrder.off,
            4,
            4,
            argument_slot,
        );
        let target = e.get(this, NiPSysModifier::m_pkTarget);
        e.vcall(stream.addr(), STREAM_SAVE_LINK_ID, &args![target]);
        let active = e.mem.u8(this.addr() + NiPSysModifier::m_bActive.off);
        e.mem.set_u8(argument_slot, active);
        stream_write(e, stream.addr(), argument_slot, 1, 1, local);
    });
}

// Translated from 00c319e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysModifier::GetViewerStrings` (Xbox PDB): `NiObject`'s
/// `GetViewerStrings`, then the class name (`NiRTTI` `012027d0`),
/// `"Name"` (the name handle), `"Order"` and `"Active"` appended to the
/// viewer-string array.
pub fn ni_p_sys_modifier_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPSysModifier>,
    strings: Ptr,
) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    add_class_name(e, strings, RTTI);
    let name = e.get(this, NiPSysModifier::m_kName);
    add_labelled(e, strings, NI_GET_VIEWER_STRING_FIXED, LABEL_NAME, name);
    let order = e.get(this, NiPSysModifier::m_uiOrder);
    add_labelled(e, strings, NI_GET_VIEWER_STRING_U32, LABEL_ORDER, order);
    let active = u32::from(e.mem.u8(this.addr() + NiPSysModifier::m_bActive.off));
    add_labelled(e, strings, NI_GET_VIEWER_STRING_BOOL, LABEL_ACTIVE, active);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_1670,
            ni_p_sys_modifier_register_streamables(Ptr<NiPSysModifier>, Ptr) -> bool
        ),
        entry!(
            0x00c3_16a0,
            fn_00c316a0(Ptr<NiPSysModifier>, Ptr, u32) -> Ptr<NiPSysModifier>
        ),
        entry!(
            0x00c3_1830,
            ni_p_sys_modifier_is_equal(Ptr<NiPSysModifier>, Ptr<NiPSysModifier>) -> bool
        ),
        entry!(
            0x00c3_1870,
            fn_00c31870(Ptr<NiPSysModifier>, u32) -> Ptr<NiPSysModifier>
        ),
        entry!(
            0x00c3_1950,
            ni_p_sys_modifier_save_binary(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_19e0,
            ni_p_sys_modifier_get_viewer_strings(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(0x00c31710, ni_p_sys_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c3_1650,
            ni_p_sys_modifier_link_object(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_1720,
            fn_00c31720(Ptr<NiPSysModifier>) -> Ptr<NiPSysModifier>
        ),
        entry!(
            0x00c3_1750,
            ni_p_sys_modifier_destructor(Ptr<NiPSysModifier>)
        ),
        entry!(
            0x00c3_1780,
            fn_00c31780(Ptr<NiPSysModifier>, Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_17e0,
            ni_p_sys_modifier_process_clone(Ptr<NiPSysModifier>, Ptr)
        ),
        entry!(
            0x00c3_18b0,
            ni_p_sys_modifier_load_binary(Ptr<NiPSysModifier>, Ptr)
        ),
    ]
}

/// Test support shared by this crate's particle units: a stream whose read
/// function hands out the bytes queued for it, in order.
#[cfg(test)]
pub(crate) mod test_stream {
    use super::*;

    /// The read function's address in tests.
    pub const READ_FN: u32 = 0x0f7f_0000;

    /// Builds an `NiStream` with file version `version` and an input stream
    /// reading from the bytes in `data`; returns the stream's address. The
    /// read double copies `size` bytes from a cursor kept before the data.
    pub fn new(e: &mut Engine, version: u32, data: &[u8]) -> u32 {
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(data.len() as u32 + 8);
        e.mem.write(buffer + 4, data);
        e.mem.set_u32(buffer, buffer + 4);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ_FN);
        e.mem.set_u32(input + 0xc, buffer);
        e.register(READ_FN, |e, a| {
            let (input, dest, size) = (a[0], a[1], a[2]);
            let buffer = e.mem.u32(input + 0xc);
            let cursor = e.mem.u32(buffer);
            let bytes = e.mem.bytes(cursor, size);
            e.mem.write(dest, &bytes);
            e.mem.set_u32(buffer, cursor + size);
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        stream
    }

    /// Doubles for the imported `InterlockedIncrement` and
    /// `InterlockedDecrement` (the platform registers them only with the
    /// executable): they change the word and answer its new value.
    pub fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
    }

    /// The input stream's position word.
    pub fn position(e: &Engine, stream: u32) -> u32 {
        let input = e.mem.u32(stream + STREAM_INPUT);
        e.mem.u32(input + BINARY_STREAM_POSITION)
    }

    /// The write function's address in tests.
    pub const WRITE_FN: u32 = 0x0f7f_0010;

    /// Builds an `NiStream` with an output stream whose write double keeps
    /// every write as `(bytes, component size)`; returns the stream's
    /// address. The vtable (for `SaveLinkID` and the like) is left to the
    /// test.
    pub fn new_output(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE_FN);
        stream
    }

    /// The writes an output stream received, `(bytes, component size)`.
    pub type Writes = std::rc::Rc<std::cell::RefCell<Vec<(Vec<u8>, u32)>>>;

    /// The writes the output stream received, `(bytes, component size)`.
    pub fn writes(e: &mut Engine) -> Writes {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = writes.clone();
        e.register_double(WRITE_FN, move |e, a| {
            let (buffer, size, component) = (a[1], a[2], a[3]);
            seen.borrow_mut()
                .push((e.mem.bytes(buffer, size), e.mem.u32(component)));
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        writes
    }

    /// The output stream's position word.
    pub fn output_position(e: &Engine, stream: u32) -> u32 {
        let output = e.mem.u32(stream + STREAM_OUTPUT);
        e.mem.u32(output + BINARY_STREAM_POSITION)
    }

    /// Doubles for the viewer-string calls: every `NiGetViewerString`
    /// overload answers its first word (the class name or the label), the
    /// array's grow does nothing and its store records the string. Returns
    /// the stored strings in order.
    pub fn viewer_strings(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        for overload in [
            NI_GET_VIEWER_STRING,
            NI_GET_VIEWER_STRING_BOOL,
            NI_GET_VIEWER_STRING_U16,
            NI_GET_VIEWER_STRING_U32,
            NI_GET_VIEWER_STRING_FLOAT,
            NI_GET_VIEWER_STRING_FIXED,
        ] {
            e.register(overload, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            Ret::default()
        });
        stored
    }

    /// A viewer-string array with room left (capacity 64, size 0, step 4).
    pub fn strings(e: &mut Engine) -> u32 {
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 64);
        e.mem.set_u16(strings + 0xe, 4);
        strings
    }

    /// The words of every call to `addr` in the call log.
    pub fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Maps the global holding the null fixed-string handle and sets it.
    pub fn null_handle(e: &mut Engine, value: u32) {
        e.map(FIXED_STRING_NULL & !0xfff, 0x1000);
        e.set_global(FIXED_STRING_NULL, value);
    }

    /// Maps the page of a global (an `NiRTTI` record's name word, a float
    /// constant) and sets its word.
    pub fn global_word(e: &mut Engine, addr: u32, value: u32) {
        e.map(addr & !0xfff, 0x1000);
        e.set_global(addr, value);
    }
}

#[cfg(test)]
mod tests {
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

    fn null_handle(e: &mut Engine, value: u32) {
        e.map(FIXED_STRING_NULL & !0xfff, 0x1000);
        e.set_global(FIXED_STRING_NULL, value);
    }

    #[test]
    fn ni_p_sys_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1710, &args![]).u32(), 0x0120_27d0);
    }

    #[test]
    fn link_object_runs_the_base_step_then_reads_a_link() {
        let mut e = Engine::new();
        e.register(BASE_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c3_1650, &args![this, 0x7000u32]);
        assert_eq!(
            calls_to(&e, BASE_LINK_OBJECT),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(calls_to(&e, GET_OBJECT_FROM_LINK_ID), vec![vec![0x7000]]);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0);
    }

    #[test]
    fn constructor_sets_the_vtable_null_name_and_active() {
        let mut e = Engine::new();
        null_handle(&mut e, 0x55);
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_uiOrder, 9);
        e.set(this, NiPSysModifier::m_pkTarget, 9);
        assert_eq!(e.call(0x00c3_1720, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, NiPSysModifier::m_kName), 0x55);
        assert_eq!(e.get(this, NiPSysModifier::m_uiOrder), 0);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0);
        assert!(e.get(this, NiPSysModifier::m_bActive));
    }

    #[test]
    fn destructor_releases_a_name_but_not_the_null_handle() {
        let mut e = Engine::new();
        null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        let string = e.mem.alloc(16);
        e.mem.set_u32(string, 2);
        e.set(this, NiPSysModifier::m_kName, string + 8);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1750, &args![this]);
        assert_eq!(e.mem.u32(string), 1);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
        e.set(this, NiPSysModifier::m_kName, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1750, &args![this]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn copy_members_assigns_the_name_order_and_active_flag() {
        let mut e = Engine::new();
        null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(NI_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysModifier>();
        let dest = e.new_object::<NiPSysModifier>();
        let strings = e.mem.alloc(32);
        e.mem.set_u32(strings, 1);
        e.mem.set_u32(strings + 16, 1);
        e.set(this, NiPSysModifier::m_kName, strings + 8);
        e.set(dest, NiPSysModifier::m_kName, strings + 24);
        e.set(this, NiPSysModifier::m_uiOrder, 7);
        e.set(this, NiPSysModifier::m_pkTarget, 0x99);
        e.set(this, NiPSysModifier::m_bActive, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1780, &args![this, dest, 0x4000u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x4000]]
        );
        assert_eq!(e.get(dest, NiPSysModifier::m_kName), strings + 8);
        assert_eq!(e.mem.u32(strings), 2);
        assert_eq!(e.mem.u32(strings + 16), 0);
        assert_eq!(e.get(dest, NiPSysModifier::m_uiOrder), 7);
        assert_eq!(e.get(dest, NiPSysModifier::m_pkTarget), 0);
        assert!(e.get(dest, NiPSysModifier::m_bActive));
        // The same handle: nothing is held or released.
        e.call_log = Some(vec![]);
        e.call(0x00c3_1780, &args![this, dest, 0x4000u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn process_clone_sets_the_clone_target() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        let clone = e.new_object::<NiPSysModifier>();
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_pkTarget, 0x6000);
        let map = e.mem.alloc(4);
        let process = e.mem.alloc(4);
        e.mem.set_u32(process, map);
        e.mem.set_u32(map, clone.addr());
        e.register(CLONE_MAP_LOOKUP, |e, a| {
            let value = if a[1] == 0x6000 {
                0x6100
            } else {
                e.mem.u32(a[0])
            };
            e.mem.set_u32(a[2], value);
            Ret::default()
        });
        e.call(0x00c3_17e0, &args![this, process]);
        assert_eq!(e.get(clone, NiPSysModifier::m_pkTarget), 0x6100);
    }

    #[test]
    fn load_binary_reads_name_order_link_and_active() {
        for (version, loader) in [
            (0x1401_0000u32, LOAD_CSTRING_AS_FIXED_STRING),
            (0x1401_0001, LOAD_FIXED_STRING),
        ] {
            let mut e = Engine::new();
            e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
            e.register(LOAD_CSTRING_AS_FIXED_STRING, |_, _| Ret::default());
            e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
            e.register(READ_LINK_ID, |_, _| Ret::default());
            let stream = test_stream::new(&mut e, version, &[5, 0, 0, 0, 0]);
            let this = e.new_object::<NiPSysModifier>();
            e.set(this, NiPSysModifier::m_bActive, true);
            e.call_log = Some(vec![]);
            e.call(0x00c3_18b0, &args![this, stream]);
            assert_eq!(calls_to(&e, loader), vec![vec![stream, this.addr() + 8]]);
            assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
            assert_eq!(e.get(this, NiPSysModifier::m_uiOrder), 5);
            assert!(!e.get(this, NiPSysModifier::m_bActive));
            assert_eq!(test_stream::position(&e, stream), 5);
        }
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(LOAD_FIXED_STRING, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x1402_0000, &[1, 0, 0, 0, 2]);
        let this = e.new_object::<NiPSysModifier>();
        e.call(0x00c3_18b0, &args![this, stream]);
        assert!(e.get(this, NiPSysModifier::m_bActive));
    }

    #[test]
    fn register_streamables_registers_the_name_after_the_base() {
        const REGISTER: u32 = 0x0f6f_0000;
        for base in [false, true] {
            let mut e = Engine::new();
            let eax = 0x1234_5600 | u32::from(base);
            e.register_double(NI_OBJECT_REGISTER_STREAMABLES, move |_, _| Ret {
                eax,
                ..Ret::default()
            });
            e.register(REGISTER, |_, _| Ret::default());
            let stream = e.mem.alloc(0x40);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(stream, vtable);
            e.mem
                .set_u32(vtable + STREAM_REGISTER_FIXED_STRING, REGISTER);
            let this = e.new_object::<NiPSysModifier>();
            e.call_log = Some(vec![]);
            let answer = e.call(0x00c3_1670, &args![this, stream]).bool();
            assert_eq!(answer, base);
            assert_eq!(
                calls_to(&e, NI_OBJECT_REGISTER_STREAMABLES),
                vec![vec![this.addr(), stream]]
            );
            let registered = calls_to(&e, REGISTER);
            if base {
                assert_eq!(registered, vec![vec![stream, this.addr() + 8]]);
            } else {
                assert!(registered.is_empty());
            }
        }
    }

    #[test]
    fn named_constructor_sets_fields_and_holds_the_name() {
        let mut e = Engine::new();
        null_handle(&mut e, 0);
        test_stream::interlocked(&mut e);
        e.register(NI_OBJECT_CONSTRUCTOR, |_, _| Ret::default());
        let string = e.mem.alloc(16);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, string + 8);
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_pkTarget, 9);
        let result = e.call(0x00c3_16a0, &args![this, name, 6u32]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, NiPSysModifier::m_kName), string + 8);
        assert_eq!(e.mem.u32(string), 1);
        assert_eq!(e.get(this, NiPSysModifier::m_uiOrder), 6);
        assert_eq!(e.get(this, NiPSysModifier::m_pkTarget), 0);
        assert!(e.get(this, NiPSysModifier::m_bActive));
        // A null name: nothing held or released.
        e.mem.set_u32(name, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c3_16a0, &args![this, name, 1u32]);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
        assert_eq!(e.get(this, NiPSysModifier::m_kName), 0);
    }

    #[test]
    fn is_equal_compares_name_order_and_active_after_the_base() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let a = e.new_object::<NiPSysModifier>();
        let b = e.new_object::<NiPSysModifier>();
        for p in [a, b] {
            e.set(p, NiPSysModifier::m_kName, 0x50);
            e.set(p, NiPSysModifier::m_uiOrder, 3);
            e.set(p, NiPSysModifier::m_bActive, true);
        }
        assert!(e.call(0x00c3_1830, &args![a, b]).bool());
        e.set(b, NiPSysModifier::m_bActive, false);
        assert!(!e.call(0x00c3_1830, &args![a, b]).bool());
        e.set(b, NiPSysModifier::m_bActive, true);
        e.set(b, NiPSysModifier::m_uiOrder, 4);
        assert!(!e.call(0x00c3_1830, &args![a, b]).bool());
        e.set(b, NiPSysModifier::m_uiOrder, 3);
        e.set(b, NiPSysModifier::m_kName, 0x51);
        assert!(!e.call(0x00c3_1830, &args![a, b]).bool());
        e.set(b, NiPSysModifier::m_kName, 0x50);
        e.register(NI_OBJECT_IS_EQUAL, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        assert!(!e.call(0x00c3_1830, &args![a, b]).bool());
    }

    #[test]
    fn deleting_destructor_releases_the_name_and_deletes_with_bit_0() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            null_handle(&mut e, 0);
            test_stream::interlocked(&mut e);
            e.register(NI_OBJECT_DESTRUCTOR, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysModifier>();
            let string = e.mem.alloc(16);
            e.mem.set_u32(string, 2);
            e.set(this, NiPSysModifier::m_kName, string + 8);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c3_1870, &args![this, flags]).u32(), this.addr());
            assert_eq!(e.mem.u32(this.addr()), VTABLE);
            assert_eq!(e.mem.u32(string), 1);
            assert_eq!(calls_to(&e, NI_OBJECT_DESTRUCTOR), vec![vec![this.addr()]]);
            let deletes = calls_to(&e, NI_DELETE);
            if flags == 1 {
                assert_eq!(deletes, vec![vec![this.addr(), 0x18]]);
            } else {
                assert!(deletes.is_empty());
            }
        }
    }

    #[test]
    fn save_binary_writes_name_order_link_and_active() {
        const SAVE_LINK: u32 = 0x0f6f_0010;
        let mut e = Engine::new();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_FIXED_STRING, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + STREAM_SAVE_LINK_ID, SAVE_LINK);
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_uiOrder, 0x0102_0304);
        e.set(this, NiPSysModifier::m_pkTarget, 0x7700);
        e.set(this, NiPSysModifier::m_bActive, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1950, &args![this, stream]);
        assert_eq!(
            calls_to(&e, BASE_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, SAVE_FIXED_STRING),
            vec![vec![stream, this.addr() + 8]]
        );
        assert_eq!(calls_to(&e, SAVE_LINK), vec![vec![stream, 0x7700]]);
        assert_eq!(*writes.borrow(), vec![(vec![4, 3, 2, 1], 4), (vec![1], 1)]);
        assert_eq!(test_stream::output_position(&e, stream), 5);
    }

    #[test]
    fn get_viewer_strings_appends_class_name_and_fields() {
        let mut e = Engine::new();
        test_stream::global_word(&mut e, RTTI, 0x0abc_0000);
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let stored = test_stream::viewer_strings(&mut e);
        let strings = test_stream::strings(&mut e);
        let this = e.new_object::<NiPSysModifier>();
        e.set(this, NiPSysModifier::m_kName, 0x55);
        e.set(this, NiPSysModifier::m_uiOrder, 7);
        e.set(this, NiPSysModifier::m_bActive, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_19e0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(
            *stored.borrow(),
            vec![0x0abc_0000, LABEL_NAME, LABEL_ORDER, LABEL_ACTIVE]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FIXED),
            vec![vec![LABEL_NAME, 0x55]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_U32),
            vec![vec![LABEL_ORDER, 7]]
        );
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_BOOL),
            vec![vec![LABEL_ACTIVE, 1]]
        );
    }

    #[test]
    fn add_viewer_string_grows_a_full_array_first() {
        let mut e = Engine::new();
        let stored = test_stream::viewer_strings(&mut e);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 0xe, 5);
        e.call_log = Some(vec![]);
        add_viewer_string(&mut e, strings, 0x99);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 8]]);
        let set = calls_to(&e, STRINGS_SET_AT);
        assert_eq!(&set[0][..2], [strings, 3]);
        assert_eq!(*stored.borrow(), vec![0x99]);
    }
}
