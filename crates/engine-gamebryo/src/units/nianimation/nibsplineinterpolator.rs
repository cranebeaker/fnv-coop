//! `NiAnimation/nibsplineinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_INTERPOLATOR_RTTI: u32 = 0x011f_4240;

// Translated from 00a57f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_INTERPOLATOR_RTTI)
}

// Translated from 00a581e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x1c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

/// `NiInterpolator`'s constructor (`00a56ff0`) and destructor (`00a57010`); `thiscall`,
/// no stack word.
const INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_6ff0;
const INTERPOLATOR_DESTRUCTOR: u32 = 0x00a5_7010;
/// `NiInterpolator::LoadBinary` (`00a88a40`; `thiscall`: stream).
const INTERPOLATOR_LOAD_BINARY: u32 = 0x00a8_8a40;
/// The base member copy `CopyMembers` runs first (`00aa1000`; `thiscall` on the source:
/// clone, cloning process).
const BASE_COPY_MEMBERS: u32 = 0x00aa_1000;
/// `NiObject::Clone` (`00a5d3f0`; `thiscall`: cloning process), answers the copy.
const OBJECT_CLONE: u32 = 0x00a5_d3f0;
/// `NiStream::ResolveLinkID` (`00a64620`; `thiscall`, no stack word): the next linked
/// object.
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The `NiBSplineInterpolator` virtual table (`0109ae8c`).
const B_SPLINE_INTERPOLATOR_VTABLE: u32 = 0x0109_ae8c;
/// The floats the constructor copies into the start and end times (`0109ae80`,
/// `01015f5c`).
const START_TIME_DEFAULT: u32 = 0x0109_ae80;
const END_TIME_DEFAULT: u32 = 0x0101_5f5c;
/// The doubles `GetActiveTimeRange` compares the times with (`010231b0`, `010241b0`).
const UNSET_START: u32 = 0x0102_31b0;
const UNSET_END: u32 = 0x0102_41b0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis`, virtual slot 4.
const SLOT_DELETE_THIS: u32 = 0x04;

layout! {
    /// `NiBSplineInterpolator` (Xbox PDB), 0x1C bytes.
    pub struct NiBSplineInterpolator: 0x1C {
        /// `m_fLastTime` (Xbox PDB, `NiInterpolator`).
        0x08 m_fLastTime: u32,
        /// `m_fStartTime` (Xbox PDB).
        0x0C m_fStartTime: u32,
        /// `m_fEndTime` (Xbox PDB).
        0x10 m_fEndTime: u32,
        /// `m_spData` (Xbox PDB): `NiBSplineData*`.
        0x14 m_spData: u32,
        /// `m_spBasisData` (Xbox PDB): `NiBSplineBasisData*`.
        0x18 m_spBasisData: u32,
    }
}

/// `InterlockedIncrement` of an object's reference count at `+4`.
fn add_ref(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at `+4` and, when it
/// reaches zero, the object's `DeleteThis` (virtual slot 4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer stored at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            add_ref(e, new);
        }
    }
}

/// One 4-byte read into `destination` through the stream's input object (`NiStream`
/// `+0x24C`; read function at `+8`, `cdecl` (input, destination, 4, address of the
/// component size 4, 1)); the answer added to the input's word at `+4`.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, destination, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

// Translated from 00a57f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::NiBSplineInterpolator(NiBSplineData*, NiBSplineBasisData*)`
/// (unnamed in the engine map; `thiscall`, `RET 8`): the `NiInterpolator` constructor
/// (`00a56ff0`), the start and end times from the floats at `0109ae80` and `01015f5c`
/// (through the x87 stack), the class's virtual table, then both pointers stored and
/// counted (when not null). Answers `this`.
pub fn ni_b_spline_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    data: u32,
    basis_data: u32,
) -> Ptr<NiBSplineInterpolator> {
    e.call(INTERPOLATOR_CONSTRUCTOR, &args![this]);
    let start = x87_quiet_bits(e.global::<u32>(START_TIME_DEFAULT));
    e.set(this, NiBSplineInterpolator::m_fStartTime, start);
    let end = x87_quiet_bits(e.global::<u32>(END_TIME_DEFAULT));
    e.set(this, NiBSplineInterpolator::m_fEndTime, end);
    e.mem.set_u32(this.addr(), B_SPLINE_INTERPOLATOR_VTABLE);
    e.set(this, NiBSplineInterpolator::m_spData, data);
    if data != 0 {
        add_ref(e, data);
    }
    e.set(this, NiBSplineInterpolator::m_spBasisData, basis_data);
    if basis_data != 0 {
        add_ref(e, basis_data);
    }
    this
}

// Translated from 00a57fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::CopyMembers` (unnamed in the engine map; `thiscall` on the
/// source, `RET 8`: clone, cloning process). The base copy (`00aa1000`), the end and
/// start times (x87 copies), the data pointer assigned to the clone's (counted), and, when
/// the source has basis data, its clone (`NiObject::Clone`, `00a5d3f0`) assigned.
pub fn ni_b_spline_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    clone: Ptr<NiBSplineInterpolator>,
    cloning: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let end = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fEndTime));
    e.set(clone, NiBSplineInterpolator::m_fEndTime, end);
    let start = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fStartTime));
    e.set(clone, NiBSplineInterpolator::m_fStartTime, start);
    let slot = clone.addr() + 0x14;
    let old = e.mem.u32(slot);
    if old != e.get(this, NiBSplineInterpolator::m_spData) {
        if old != 0 {
            release(e, old);
        }
        let data = e.get(this, NiBSplineInterpolator::m_spData);
        e.mem.set_u32(slot, data);
        if data != 0 {
            add_ref(e, data);
        }
    }
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        let copy = e.call(OBJECT_CLONE, &args![basis_data, cloning]).u32();
        assign_pointer(e, clone.addr() + 0x18, copy);
    }
}

// Translated from 00a57f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::~NiBSplineInterpolator` (Xbox PDB; `thiscall`): stores the
/// class's virtual table, releases the basis data and the data (when set), then the
/// `NiInterpolator` destructor (`00a57010`, a jump).
pub fn ni_b_spline_interpolator_destructor(e: &mut Engine, this: Ptr<NiBSplineInterpolator>) {
    e.mem.set_u32(this.addr(), B_SPLINE_INTERPOLATOR_VTABLE);
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        release(e, basis_data);
    }
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    if data != 0 {
        release(e, data);
    }
    e.call(INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a57d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetActiveTimeRange` (Xbox PDB; `thiscall`, `RET 8`: addresses of
/// the begin and end times). When the begin time equals the double at `010231b0` and the
/// end time the one at `010241b0` (the unset range; a NaN is unequal) both become 0;
/// otherwise they become the interpolator's start and end times (x87 copies).
pub fn ni_b_spline_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let unset_start = f64::from_bits(e.mem.u64(UNSET_START));
    if x87_load32(e.mem.u32(begin.addr())) == unset_start {
        let unset_end = f64::from_bits(e.mem.u64(UNSET_END));
        if x87_load32(e.mem.u32(end.addr())) == unset_end {
            e.mem.set_u32(begin.addr(), 0);
            e.mem.set_u32(end.addr(), 0);
            return;
        }
    }
    let start = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fStartTime));
    e.mem.set_u32(begin.addr(), start);
    let finish = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fEndTime));
    e.mem.set_u32(end.addr(), finish);
}

// Translated from 00a58080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream).
/// `NiInterpolator::LoadBinary` (`00a88a40`), the start and end times (one 4-byte read
/// each), then the data and the basis data as the next two linked objects
/// (`NiStream::ResolveLinkID`, `00a64620`), each assigned and counted.
pub fn ni_b_spline_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    stream: Ptr,
) {
    e.call(INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    read_word(e, stream, this.addr() + 0xc);
    read_word(e, stream, this.addr() + 0x10);
    let data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x14, data);
    let basis_data = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + 0x18, basis_data);
}

/// `NiBSplineInterpolator` virtual slots (Xbox PDB names): `GetChannelCount`
/// +0xDC, `GetControlPointCount` +0xE0, `GetDimension` +0xE4,
/// `UsesCompressedControlPoints` +0xF0, `GetControlHandle` +0xF8.
const SLOT_GET_CHANNEL_COUNT: u32 = 0xdc;
const SLOT_GET_CONTROL_POINT_COUNT: u32 = 0xe0;
const SLOT_GET_DIMENSION: u32 = 0xe4;
const SLOT_USES_COMPRESSED_CONTROL_POINTS: u32 = 0xf0;
const SLOT_GET_CONTROL_HANDLE: u32 = 0xf8;
/// `NiObject` virtual slots: `RegisterStreamables` +0x54, `IsEqual` +0x5C,
/// `GetViewerStrings` +0x60.
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_GET_VIEWER_STRINGS: u32 = 0x60;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;
/// `NiBSplineBasisData::GetDegree3Basis` (`00a62660`, engine map;
/// `thiscall`, answers the basis, whose first word is its control point
/// count).
const GET_DEGREE3_BASIS: u32 = 0x00a6_2660;
/// The base `RegisterStreamables` (`00a81fe0`, a 5-byte jump to
/// `00a5d2f0`), `IsEqual` (`00ba87d0`, a 5-byte jump to `NiObject::IsEqual`
/// `00a5d470`) and `SaveBinary` (`00a724d0`, a 5-byte jump to `00a89af0`);
/// `thiscall`, one word each.
const BASE_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
const BASE_SAVE_BINARY: u32 = 0x00a7_24d0;
/// `NiInterpolator::GetViewerStrings` (`00a570b0`, engine map; `thiscall`,
/// one word).
const INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_70b0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`) and its float overload
/// (`00a7d550`, `cdecl (name, value)`); the viewer-string array's grow
/// (`0096ad30`, `thiscall (array; new capacity)`) and store (`0096ae90`,
/// `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_fStartTime"` and `"m_fEndTime"`.
const NAME_START_TIME: u32 = 0x0109_af98;
const NAME_END_TIME: u32 = 0x0109_af8c;

/// One 4-byte write from `source` through the stream's output object
/// (`NiStream` +0x250; write function at +0xC, `cdecl (output, source, 4,
/// address of the component size 4, 1)`); the answer added to the output's
/// word at +4.
fn write_word(e: &mut Engine, stream: Ptr, source: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write, &args![output, source, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(output + 4).wrapping_add(answer);
    e.mem.set_u32(output + 4, total);
}

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
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

// Translated from 00a57da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetAllocatedSize` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel): the channel's dimension
/// times its control point count (both virtual, given the channel word)
/// times 2 with compressed control points (vtable +0xF0), 4 without.
pub fn ni_b_spline_interpolator_get_allocated_size(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    channel: u32,
) -> u32 {
    let object = this.addr();
    let compressed = e
        .vcall(object, SLOT_USES_COMPRESSED_CONTROL_POINTS, &args![])
        .bool();
    let dimension = e.vcall(object, SLOT_GET_DIMENSION, &args![channel]).u32();
    let count = e
        .vcall(object, SLOT_GET_CONTROL_POINT_COUNT, &args![channel])
        .u32();
    let size = dimension.wrapping_mul(count);
    if compressed {
        size.wrapping_add(size)
    } else {
        let size = size.wrapping_add(size);
        size.wrapping_add(size)
    }
}

// Translated from 00a57e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetControlPointCount` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel): for a channel below the
/// channel count (vtable +0xDC, 16-bit compare), with data and a control
/// handle (vtable +0xF8, given the whole channel word) other than 0xFFFF,
/// the first word of the basis data's degree-3 basis (`00a62660`);
/// otherwise 0.
pub fn ni_b_spline_interpolator_get_control_point_count(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    channel: u32,
) -> u32 {
    let object = this.addr();
    let channels = e.vcall(object, SLOT_GET_CHANNEL_COUNT, &args![]).u16();
    if (channel as u16) < channels && e.get(this, NiBSplineInterpolator::m_spData) != 0 {
        let handle = e
            .vcall(object, SLOT_GET_CONTROL_HANDLE, &args![channel])
            .u32();
        if handle != INVALID_HANDLE {
            let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
            let basis = e.call(GET_DEGREE3_BASIS, &args![basis_data]).u32();
            return e.mem.u32(basis);
        }
    }
    0
}

// Translated from 00a57e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::RegisterStreamables` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): false when the base
/// registration (`00a81fe0`) answers false; otherwise the data and the
/// basis data (when set) register themselves (vtable +0x54) and the answer
/// is true.
pub fn ni_b_spline_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    if data != 0 {
        e.vcall(data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        e.vcall(basis_data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a57e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::IsEqual` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the other object), as the exe has it: after the
/// base `IsEqual` (`00ba87d0`), equal start and end times (`FUCOMPP`; a NaN
/// is unequal) answer true. Otherwise the data and then the basis data are
/// compared the usual way (both null, or both set and equal by the first's
/// `IsEqual`, vtable +0x5C; a mismatch stops), but every way through that
/// answers false.
pub fn ni_b_spline_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    other: Ptr<NiBSplineInterpolator>,
) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let start = x87_load32(e.get(this, NiBSplineInterpolator::m_fStartTime));
    let other_start = x87_load32(e.get(other, NiBSplineInterpolator::m_fStartTime));
    if other_start == start {
        let end = x87_load32(e.get(this, NiBSplineInterpolator::m_fEndTime));
        let other_end = x87_load32(e.get(other, NiBSplineInterpolator::m_fEndTime));
        if other_end == end {
            return true;
        }
    }
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    let other_data = e.get(other, NiBSplineInterpolator::m_spData);
    if data != 0 {
        if other_data == 0 {
            return false;
        }
        if !e.vcall(data, SLOT_IS_EQUAL, &args![other_data]).bool() {
            return false;
        }
    } else if other_data != 0 {
        return false;
    }
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    let other_basis_data = e.get(other, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 && other_basis_data != 0 {
        // The exe drops this answer: the code after the call returns false.
        e.vcall(basis_data, SLOT_IS_EQUAL, &args![other_basis_data]);
    }
    false
}

// Translated from 00a58160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::SaveBinary` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the stream): the base `SaveBinary`
/// (`00a724d0`), the start and end times (one 4-byte write each), then the
/// data's and the basis data's link ids (the stream's vtable +0x30).
pub fn ni_b_spline_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    stream: Ptr,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    write_word(e, stream, this.addr() + 0xc);
    write_word(e, stream, this.addr() + 0x10);
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    e.vcall(stream.addr(), 0x30, &args![data]);
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    e.vcall(stream.addr(), 0x30, &args![basis_data]);
}

// Translated from 00a58200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineInterpolator::GetViewerStrings` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the strings array):
/// `NiInterpolator::GetViewerStrings` (`00a570b0`), then the class name
/// (the first word of the `NiRTTI` record), `m_fStartTime` and
/// `m_fEndTime` (the floats through the FPU) appended, then the data's and
/// the basis data's own strings (vtable +0x60, when set).
pub fn ni_b_spline_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiBSplineInterpolator>,
    strings: Ptr,
) {
    e.call(INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let start = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fStartTime));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![NAME_START_TIME, start])
        .u32();
    push_viewer_string(e, strings, text);
    let end = x87_quiet_bits(e.get(this, NiBSplineInterpolator::m_fEndTime));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![NAME_END_TIME, end])
        .u32();
    push_viewer_string(e, strings, text);
    let data = e.get(this, NiBSplineInterpolator::m_spData);
    if data != 0 {
        e.vcall(data, SLOT_GET_VIEWER_STRINGS, &args![strings]);
    }
    let basis_data = e.get(this, NiBSplineInterpolator::m_spBasisData);
    if basis_data != 0 {
        e.vcall(basis_data, SLOT_GET_VIEWER_STRINGS, &args![strings]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_7da0,
            ni_b_spline_interpolator_get_allocated_size(Ptr<NiBSplineInterpolator>, u32) -> u32
        ),
        entry!(
            0x00a5_7e00,
            ni_b_spline_interpolator_get_control_point_count(
                Ptr<NiBSplineInterpolator>,
                u32,
            ) -> u32
        ),
        entry!(
            0x00a5_7e50,
            ni_b_spline_interpolator_register_streamables(Ptr<NiBSplineInterpolator>, Ptr) -> bool
        ),
        entry!(
            0x00a5_7e90,
            ni_b_spline_interpolator_is_equal(
                Ptr<NiBSplineInterpolator>,
                Ptr<NiBSplineInterpolator>,
            ) -> bool
        ),
        entry!(
            0x00a5_8160,
            ni_b_spline_interpolator_save_binary(Ptr<NiBSplineInterpolator>, Ptr)
        ),
        entry!(
            0x00a5_8200,
            ni_b_spline_interpolator_get_viewer_strings(Ptr<NiBSplineInterpolator>, Ptr)
        ),
        entry!(0x00a57f70, ni_b_spline_interpolator_get_rtti() -> Ptr),
        entry!(0x00a581e0, ni_b_spline_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_7f20,
            ni_b_spline_interpolator_constructor(
                Ptr<NiBSplineInterpolator>,
                u32,
                u32,
            ) -> Ptr<NiBSplineInterpolator>
        ),
        entry!(
            0x00a5_7fd0,
            ni_b_spline_interpolator_copy_members(
                Ptr<NiBSplineInterpolator>,
                Ptr<NiBSplineInterpolator>,
                Ptr,
            )
        ),
        entry!(
            0x00a5_7f80,
            ni_b_spline_interpolator_destructor(Ptr<NiBSplineInterpolator>)
        ),
        entry!(
            0x00a5_7d60,
            ni_b_spline_interpolator_get_active_time_range(Ptr<NiBSplineInterpolator>, Ptr, Ptr)
        ),
        entry!(
            0x00a5_8080,
            ni_b_spline_interpolator_load_binary(Ptr<NiBSplineInterpolator>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57f70, &args![]).u32(),
            NI_B_SPLINE_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a5_7f80, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a581e0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a581e0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x1c]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const DELETE_THIS: u32 = 0x0310_0000;

    /// An engine with counting doubles and a virtual table whose `DeleteThis` is recorded.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        e.call_log = Some(vec![]);
        (e, vtable)
    }

    fn object(e: &mut Engine, vtable: u32, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn constructor_counts_both_pointers() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(START_TIME_DEFAULT, 4);
        e.set_global(START_TIME_DEFAULT, 0x7f80_0001u32);
        e.map(END_TIME_DEFAULT, 4);
        e.set_global(END_TIME_DEFAULT, 0xff7f_ffffu32);
        let data = object(&mut e, vtable, 0);
        let this = e.mem.alloc(0x1c);
        assert_eq!(e.call(0x00a5_7f20, &args![this, data, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), B_SPLINE_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0xff7f_ffff);
        assert_eq!((e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)), (data, 0));
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT), vec![vec![data + 4]]);
    }

    #[test]
    fn copy_members_shares_the_data_and_clones_the_basis() {
        let (mut e, vtable) = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let data = object(&mut e, vtable, 1);
        let old = object(&mut e, vtable, 1);
        let basis = object(&mut e, vtable, 1);
        let copy = object(&mut e, vtable, 0);
        e.register_double(OBJECT_CLONE, move |_, _| copy.into_ret());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0xc, 1.5f32.to_bits());
        e.mem.set_u32(this + 0x10, 2.5f32.to_bits());
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, basis);
        let clone = e.mem.alloc(0x1c);
        e.mem.set_u32(clone + 0x14, old);
        e.call(0x00a5_7fd0, &args![this, clone, 0x4000u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this, clone, 0x4000]]
        );
        assert_eq!(e.mem.f32(clone + 0xc), 1.5);
        assert_eq!(e.mem.f32(clone + 0x10), 2.5);
        assert_eq!(e.mem.u32(clone + 0x14), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![old]]);
        assert_eq!(calls(&e, OBJECT_CLONE), vec![vec![basis, 0x4000]]);
        assert_eq!(e.mem.u32(clone + 0x18), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
    }

    #[test]
    fn destructor_releases_both_pointers() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        let data = object(&mut e, vtable, 2);
        let basis = object(&mut e, vtable, 1);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, basis);
        e.call(0x00a5_7f80, &args![this]);
        assert_eq!(e.mem.u32(this), B_SPLINE_INTERPOLATOR_VTABLE);
        assert_eq!(
            calls(&e, INTERLOCKED_DECREMENT),
            vec![vec![basis + 4], vec![data + 4]]
        );
        assert_eq!(calls(&e, DELETE_THIS), vec![vec![basis]]);
        assert_eq!(calls(&e, INTERPOLATOR_DESTRUCTOR), vec![vec![this]]);
    }

    #[test]
    fn active_time_range_is_zero_only_when_unset() {
        let mut e = Engine::new();
        e.map(UNSET_START, 8);
        e.mem.set_u64(UNSET_START, f64::from(f32::MAX).to_bits());
        e.map(UNSET_END, 8);
        e.mem.set_u64(UNSET_END, f64::from(f32::MIN).to_bits());
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0xc, 1.0f32.to_bits());
        e.mem.set_u32(this + 0x10, 3.0f32.to_bits());
        let range = e.mem.alloc(8);
        e.mem.set_u32(range, f32::MAX.to_bits());
        e.mem.set_u32(range + 4, f32::MIN.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.u32(range), e.mem.u32(range + 4)), (0, 0));
        e.mem.set_u32(range, f32::MAX.to_bits());
        e.mem.set_u32(range + 4, 0.0f32.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.f32(range), e.mem.f32(range + 4)), (1.0, 3.0));
        e.mem.set_u32(range, f32::NAN.to_bits());
        e.mem.set_u32(range + 4, f32::MIN.to_bits());
        e.call(0x00a5_7d60, &args![this, range, range + 4]);
        assert_eq!((e.mem.f32(range), e.mem.f32(range + 4)), (1.0, 3.0));
    }

    #[test]
    fn load_binary_reads_the_times_and_links_both_objects() {
        let (mut e, vtable) = engine();
        e.register(INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        let data = object(&mut e, vtable, 0);
        let basis = object(&mut e, vtable, 0);
        let mut linked = vec![basis, data];
        e.register_double(RESOLVE_LINK_ID, move |_, _| {
            linked.pop().unwrap().into_ret()
        });
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], 0x3f80_0000);
            4u32.into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x1c);
        e.call(0x00a5_8080, &args![this, stream]);
        assert_eq!(
            calls(&e, INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(input + 4), 8);
        let reads = calls(&e, READ);
        assert_eq!((reads[0][1], reads[1][1]), (this + 0xc, this + 0x10));
        assert_eq!(
            (e.mem.u32(this + 0x14), e.mem.u32(this + 0x18)),
            (data, basis)
        );
        assert_eq!((e.mem.u32(data + 4), e.mem.u32(basis + 4)), (1, 1));
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    type I = NiBSplineInterpolator;

    const VTABLE: u32 = 0x0002_0000;
    const CHANNELS: u32 = 0x0001_0300;
    const POINTS: u32 = 0x0001_0310;
    const DIMENSION: u32 = 0x0001_0320;
    const COMPRESSED: u32 = 0x0001_0330;
    const HANDLE: u32 = 0x0001_0340;
    const CHILD: u32 = 0x0001_0350;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with one vtable for the interpolator and its children:
    /// the slots answer the object's own words (+0x1C compressed flag,
    /// +0x20 dimension, +0x24 control points, +0x28 channels, +0x2C
    /// handle); the children's register / equal / strings slots answer
    /// whether their argument is 0x4444.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(COMPRESSED, |e, a| word(e.mem.u32(a[0] + 0x1c)));
        e.register(DIMENSION, |e, a| word(e.mem.u32(a[0] + 0x20)));
        e.register(POINTS, |e, a| word(e.mem.u32(a[0] + 0x24)));
        e.register(CHANNELS, |e, a| word(e.mem.u32(a[0] + 0x28)));
        e.register(HANDLE, |e, a| word(e.mem.u32(a[0] + 0x2c)));
        e.register(CHILD, |_, a| word(u32::from(a[1] == 0x4444)));
        let mut slots = vec![0u32; 0x40];
        slots[(SLOT_GET_CHANNEL_COUNT / 4) as usize] = CHANNELS;
        slots[(SLOT_GET_CONTROL_POINT_COUNT / 4) as usize] = POINTS;
        slots[(SLOT_GET_DIMENSION / 4) as usize] = DIMENSION;
        slots[(SLOT_USES_COMPRESSED_CONTROL_POINTS / 4) as usize] = COMPRESSED;
        slots[(SLOT_GET_CONTROL_HANDLE / 4) as usize] = HANDLE;
        slots[(SLOT_REGISTER_STREAMABLES / 4) as usize] = CHILD;
        slots[(SLOT_IS_EQUAL / 4) as usize] = CHILD;
        slots[(SLOT_GET_VIEWER_STRINGS / 4) as usize] = CHILD;
        slots[0x30 / 4] = CHILD;
        e.put_vtable(VTABLE, &slots);
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object, VTABLE);
        object
    }

    #[test]
    fn allocated_size_is_dimension_times_points_times_the_point_size() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u32(this + 0x20, 3);
        e.mem.set_u32(this + 0x24, 5);
        assert_eq!(e.call(0x00a5_7da0, &args![this, 7u32]).u32(), 60);
        assert_eq!(calls(&e, DIMENSION), vec![vec![this, 7]]);
        assert_eq!(calls(&e, POINTS), vec![vec![this, 7]]);
        e.mem.set_u32(this + 0x1c, 1);
        assert_eq!(e.call(0x00a5_7da0, &args![this, 7u32]).u32(), 30);
    }

    #[test]
    fn control_point_count_comes_from_the_basis() {
        let mut e = engine();
        e.register(GET_DEGREE3_BASIS, |_, a| word(a[0] + 8));
        let this = object(&mut e);
        let basis_data = e.mem.alloc(0x10);
        e.mem.set_u32(basis_data + 8, 12);
        e.mem.set_u32(this + 0x28, 2);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x18, basis_data);
        let this = Ptr::<I>::new(this);
        assert_eq!(e.call(0x00a5_7e00, &args![this, 0x0001_0001u32]).u32(), 12);
        assert_eq!(calls(&e, HANDLE), vec![vec![this.addr(), 0x0001_0001]]);
        assert_eq!(calls(&e, GET_DEGREE3_BASIS), vec![vec![basis_data]]);
        // Channel out of range (16-bit compare), no data, no handle.
        assert_eq!(e.call(0x00a5_7e00, &args![this, 2u32]).u32(), 0);
        e.mem.set_u32(this.addr() + 0x2c, 0xffff);
        assert_eq!(e.call(0x00a5_7e00, &args![this, 0u32]).u32(), 0);
        e.mem.set_u32(this.addr() + 0x2c, 0);
        e.set(this, I::m_spData, 0);
        assert_eq!(e.call(0x00a5_7e00, &args![this, 0u32]).u32(), 0);
        assert_eq!(calls(&e, GET_DEGREE3_BASIS).len(), 1);
    }

    #[test]
    fn register_streamables_registers_both_children() {
        let mut e = engine();
        let this = object(&mut e);
        let data = object(&mut e);
        let basis_data = object(&mut e);
        e.mem.set_u32(this + 0x14, data);
        e.mem.set_u32(this + 0x18, basis_data);
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(0));
        assert!(!e.call(0x00a5_7e50, &args![this, 0x700u32]).bool());
        assert!(calls(&e, CHILD).is_empty());
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(1));
        assert!(e.call(0x00a5_7e50, &args![this, 0x700u32]).bool());
        assert_eq!(
            calls(&e, CHILD),
            vec![vec![data, 0x700], vec![basis_data, 0x700]]
        );
        e.mem.set_u32(this + 0x14, 0);
        e.mem.set_u32(this + 0x18, 0);
        assert!(e.call(0x00a5_7e50, &args![this, 0x700u32]).bool());
        assert_eq!(calls(&e, CHILD).len(), 2);
    }

    #[test]
    fn is_equal_answers_true_only_for_equal_times() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        let this = Ptr::<I>::new(object(&mut e));
        let other = Ptr::<I>::new(object(&mut e));
        e.set(this, I::m_fStartTime, 1.0f32.to_bits());
        e.set(other, I::m_fStartTime, 1.0f32.to_bits());
        e.set(this, I::m_fEndTime, 2.0f32.to_bits());
        e.set(other, I::m_fEndTime, 2.0f32.to_bits());
        assert!(e.call(0x00a5_7e90, &args![this, other]).bool());
        // Different times: the children are compared, the answer is false.
        let data = object(&mut e);
        let basis_data = object(&mut e);
        e.set(other, I::m_fEndTime, 3.0f32.to_bits());
        e.set(this, I::m_spData, data);
        e.set(other, I::m_spData, 0x4444);
        e.set(this, I::m_spBasisData, basis_data);
        e.set(other, I::m_spBasisData, 0x4444);
        assert!(!e.call(0x00a5_7e90, &args![this, other]).bool());
        assert_eq!(
            calls(&e, CHILD),
            vec![vec![data, 0x4444], vec![basis_data, 0x4444]]
        );
        // A NaN time is unequal; unequal data stops before the basis data.
        e.set(other, I::m_fEndTime, 2.0f32.to_bits());
        e.set(this, I::m_fStartTime, f32::NAN.to_bits());
        e.set(other, I::m_spData, 0x4445);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_7e90, &args![this, other]).bool());
        assert_eq!(calls(&e, CHILD), vec![vec![data, 0x4445]]);
        // One side without data: no compare at all.
        e.set(other, I::m_spData, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_7e90, &args![this, other]).bool());
        assert!(calls(&e, CHILD).is_empty());
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        e.set(this, I::m_fStartTime, 1.0f32.to_bits());
        assert!(!e.call(0x00a5_7e90, &args![this, this]).bool());
    }

    #[test]
    fn save_binary_writes_times_and_links() {
        const WRITE: u32 = 0x0001_0360;
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            word(4)
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream, VTABLE);
        e.mem.set_u32(stream + 0x250, output);
        let this = object(&mut e);
        e.mem.set_u32(this + 0x14, 0x5000);
        e.mem.set_u32(this + 0x18, 0x6000);
        e.call(0x00a5_8160, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls(&e, WRITE);
        assert_eq!((writes[0][1], writes[1][1]), (this + 0xc, this + 0x10));
        assert_eq!(e.mem.u32(output + 4), 8);
        assert_eq!(
            calls(&e, CHILD),
            vec![vec![stream, 0x5000], vec![stream, 0x6000]]
        );
    }

    #[test]
    fn get_viewer_strings_appends_name_times_and_children() {
        let mut e = engine();
        e.map(NI_B_SPLINE_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_INTERPOLATOR_RTTI, 0x0109_7777u32);
        e.register(INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x11));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| word(0x12));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = object(&mut e);
        e.mem.set_f32(this + 0xc, 1.5);
        e.mem.set_f32(this + 0x10, 2.5);
        let data = object(&mut e);
        e.mem.set_u32(this + 0x14, data);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00a5_8200, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_7777]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![
                vec![NAME_START_TIME, 1.5f32.to_bits()],
                vec![NAME_END_TIME, 2.5f32.to_bits()]
            ]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 3);
        assert_eq!(calls(&e, CHILD), vec![vec![data, strings]]);
    }
}
