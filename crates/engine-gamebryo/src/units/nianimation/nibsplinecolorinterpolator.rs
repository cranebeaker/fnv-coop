//! `NiAnimation/nibsplinecolorinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineColorInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_402c;

// Translated from 00a51aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_color_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI)
}

// Translated from 00a51af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_color_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
}

/// The vtable of `NiBSplineColorInterpolator` (RTTI
/// `.?AVNiBSplineColorInterpolator@@`) and the object's size.
const VTABLE: u32 = 0x0109_9d3c;
const OBJECT_SIZE: u32 = 0x34;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplineInterpolator(NiBSplineData*, NiBSplineBasisData*)`
/// constructor (`00a57f20`, `thiscall (this; data, basis data)`), its
/// `CopyMembers` (`00a57fd0`, `thiscall (this; dest, cloning)`),
/// `RegisterStreamables` (`00a57e50`), `IsEqual` (`00a57e90`),
/// `LoadBinary` (`00a58080`), `SaveBinary` (`00a58160`) and
/// `GetViewerStrings` (`00a58200`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7f20;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a5_7e50;
const BASE_IS_EQUAL: u32 = 0x00a5_7e90;
const BASE_LOAD_BINARY: u32 = 0x00a5_8080;
const BASE_SAVE_BINARY: u32 = 0x00a5_8160;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a5_8200;
/// `NiBSplineData::GetValueDegree3` (`00a52550`, Xbox PDB; `thiscall
/// (data; time, &result, dimension, basis data, handle)`).
const GET_VALUE_DEGREE3: u32 = 0x00a5_2550;
/// `NiColorA::LoadBinary` (`00a69800`), `NiColorA::SaveBinary`
/// (`00a698a0`) and `NiColorA::operator!=` (`004a4c90`, the engine map's
/// `NiColorA::operator_`); engine map names, `thiscall (color; stream or
/// other color)`.
const COLOR_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_NOT_EQUAL: u32 = 0x004a_4c90;
/// `NiBSplineInterpolator::GetControlPointCount` (vtable +0xE0, Xbox PDB
/// slot name).
const SLOT_GET_CONTROL_POINT_COUNT: u32 = 0xe0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`) and its unsigned
/// overload (`00a7d510`, `cdecl (name, value)`); the viewer-string array's
/// grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_kColorACPHandle"`.
const NAME_HANDLE: u32 = 0x0109_9e3c;
/// `NiBSplineInterpolator` fields (Xbox PDB): `m_fLastTime` +8,
/// `m_fStartTime` +0xC, `m_fEndTime` +0x10, `m_spData` +0x14,
/// `m_spBasisData` +0x18; this class's color value +0x1C (four floats) and
/// control-point handle +0x2C.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const COLOR_VALUE: u32 = 0x1c;
const COLOR_HANDLE: u32 = 0x2c;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;

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

/// The color and handle the factory and `CreateClone` give a new object:
/// the vtable, the four floats 0 (`FLDZ`, stored four times) and the
/// handle.
fn set_up(e: &mut Engine, object: u32, handle: u32) {
    e.mem.set_u32(object, VTABLE);
    let zero = x87_store32(0.0);
    for i in 0..4 {
        e.mem.set_u32(object + COLOR_VALUE + 4 * i, zero);
    }
    e.mem.set_u32(object + COLOR_HANDLE, handle);
}

/// A new color interpolator in a block from `operator new`: the base
/// constructor with no data and no basis data, then [`set_up`] with the
/// handle 0xFFFF. Null when the allocation fails.
fn new_color(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCTOR, &args![object, 0u32, 0u32]);
    set_up(e, object, INVALID_HANDLE);
    object
}

/// Copies the color value and the handle (five words) between objects.
fn copy_color_and_handle(e: &mut Engine, from: u32, to: u32) {
    for offset in [0x1c, 0x20, 0x24, 0x28, 0x2c] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 00a50610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetChannelPosed` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits):
/// for channel 0, true when `GetControlPointCount(0)` (vtable +0xE0) is 0;
/// false for any other channel.
pub fn ni_b_spline_color_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> bool {
    if channel as u16 != 0 {
        return false;
    }
    e.vcall(this.addr(), SLOT_GET_CONTROL_POINT_COUNT, &args![0u32])
        .u32()
        == 0
}

// Translated from 00a50a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetDegree` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits): 3
/// for channel 0, 0 for any other.
pub fn ni_b_spline_color_interpolator_get_degree(_e: &mut Engine, _this: Ptr, channel: u32) -> u32 {
    (u32::from(channel as u16 != 0).wrapping_sub(1)) & 3
}

// Translated from 00a50b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::RegisterStreamables` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the stream):
/// `NiBSplineInterpolator::RegisterStreamables`'s answer (`00a57e50`).
pub fn ni_b_spline_color_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00a51a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetDimension` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits): 4
/// for channel 0, 0 for any other.
pub fn ni_b_spline_color_interpolator_get_dimension(
    _e: &mut Engine,
    _this: Ptr,
    channel: u32,
) -> u32 {
    (u32::from(channel as u16 != 0).wrapping_sub(1)) & 4
}

// Translated from 00a51a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetControlHandle` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits): the
/// handle for channel 0, 0xFFFF for any other.
pub fn ni_b_spline_color_interpolator_get_control_handle(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> u32 {
    if channel as u16 != 0 {
        return INVALID_HANDLE;
    }
    e.mem.u32(this.addr() + COLOR_HANDLE)
}

// Translated from 00a51a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::SetControlHandle` (Xbox PDB, matched by the
/// engine map; `thiscall (this; handle, channel)`): stores the whole handle
/// word for channel 0 (16-bit compare), nothing for another.
pub fn ni_b_spline_color_interpolator_set_control_handle(
    e: &mut Engine,
    this: Ptr,
    handle: u32,
    channel: u32,
) {
    if channel as u16 == 0 {
        e.mem.set_u32(this.addr() + COLOR_HANDLE, handle);
    }
}

// Translated from 00a51a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::CopyMembers` (body; the engine map has no
/// name; `thiscall (this; dest, cloning)`): the base `CopyMembers`
/// (`00a57fd0`), then the color value and the handle copied word for word.
pub fn fn_00a51a60(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    copy_color_and_handle(e, this.addr(), dest.addr());
}

// Translated from 00a51ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBSplineColorInterpolator(NiBSplineData*, unsigned int,
/// NiBSplineBasisData*)` constructor (the engine map has no name;
/// `thiscall (this; data, handle, basis data)`, `RET 0xC`): the base
/// constructor on the data and the basis data (`00a57f20`), the vtable, the
/// color 0 and the handle. Returns `this`.
pub fn fn_00a51ab0(e: &mut Engine, this: Ptr, data: u32, handle: u32, basis_data: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, data, basis_data]);
    set_up(e, this.addr(), handle);
    this
}

// Translated from 00a51b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::Update` (Xbox PDB, matched by the engine
/// map; `thiscall (this; time, target, &color)`, `RET 0xC`): at the last
/// time (`FUCOMP` equal) the color is copied out. Otherwise, unless the
/// handle is 0xFFFF, the color becomes the spline data's degree-3 value
/// (`00a52550`, dimension 4, into a stack buffer) at the normalized time
/// `(time - start) / (end - start)` (x87, stored as a `float`), its four
/// floats copied in through the FPU; then the color is copied out and the
/// time (through the FPU) becomes the last time. Always true.
pub fn ni_b_spline_color_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: u32,
    color: Ptr,
) -> bool {
    let object = this.addr();
    let copy_out = |e: &mut Engine| {
        for i in 0..4 {
            let value = e.mem.u32(object + COLOR_VALUE + 4 * i);
            e.mem.set_u32(color.addr().wrapping_add(4 * i), value);
        }
    };
    let last = x87_load32(e.mem.u32(object + LAST_TIME));
    let now = x87_load32(time.to_bits());
    if now == last {
        copy_out(e);
        return true;
    }
    let handle = e.mem.u32(object + COLOR_HANDLE);
    if handle != INVALID_HANDLE {
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(now, x87_m32(start));
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let basis = e.mem.u32(object + BASIS_DATA);
        let normalized = x87_quiet_bits(x87_store32(x87_div(from_start, span)));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.with_stack(0x10, |e, result| {
            e.call(
                GET_VALUE_DEGREE3,
                &args![data, normalized, result, 4u32, basis, handle],
            );
            for i in 0..4 {
                let value = x87_quiet_bits(e.mem.u32(result.addr() + 4 * i));
                e.mem.set_u32(object + COLOR_VALUE + 4 * i, value);
            }
        });
    }
    copy_out(e);
    e.mem
        .set_u32(object + LAST_TIME, x87_quiet_bits(time.to_bits()));
    true
}

// Translated from 00a51bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::CreateClone` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the cloning process): a new object as
/// the factory builds it, the base `CopyMembers` (`00a57fd0`) into it, then
/// the color and the handle copied as words (into address 0 when the
/// allocation failed, as the exe does). Answers the new object.
pub fn ni_b_spline_color_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = new_color(e);
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    copy_color_and_handle(e, this.addr(), object);
    Ptr::new(object)
}

// Translated from 00a51c60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; registered for
/// `"NiBSplineColorInterpolator"` by `00a2d8a0`; Ghidra has no function
/// here and the engine map no name): a new color interpolator (no data, no
/// basis data, color 0, handle 0xFFFF), null when the allocation fails.
pub fn fn_00a51c60(e: &mut Engine) -> Ptr {
    Ptr::new(new_color(e))
}

// Translated from 00a51ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the base `SaveBinary`
/// (`00a58160`), the color (`NiColorA::SaveBinary` on +0x1C), then the
/// handle (4 bytes from +0x2C through the stream writer's function at its
/// +0xC, `cdecl (writer, this + 0x2c, 4, &4, 1)`, the answer added to the
/// writer's +4).
pub fn ni_b_spline_color_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(COLOR_SAVE_BINARY, &args![this.addr() + COLOR_VALUE, stream]);
    let writer = e.mem.u32(stream.addr() + 0x250);
    let write_function = e.mem.u32(writer.wrapping_add(0xc));
    let source = this.addr() + COLOR_HANDLE;
    let written = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write_function, &args![writer, source, 4u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(writer.wrapping_add(4)).wrapping_add(written);
    e.mem.set_u32(writer.wrapping_add(4), position);
}

// Translated from 00a51cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::IsEqual` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the other object): the base `IsEqual`
/// (`00a57e90`), the same color (`NiColorA::operator!=` answering false)
/// and the same handle.
pub fn ni_b_spline_color_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.call(
        COLOR_NOT_EQUAL,
        &args![this.addr() + COLOR_VALUE, other.addr() + COLOR_VALUE],
    )
    .bool()
    {
        return false;
    }
    e.mem.u32(other.addr() + COLOR_HANDLE) == e.mem.u32(this.addr() + COLOR_HANDLE)
}

// Translated from 00a51d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::LoadBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the base `LoadBinary`
/// (`00a58080`), the color (`NiColorA::LoadBinary` on +0x1C), then the
/// handle (4 bytes into +0x2C through the stream reader's function at its
/// +8, `cdecl (reader, this + 0x2c, 4, &4, 1)`, the answer added to the
/// reader's +4).
pub fn ni_b_spline_color_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + COLOR_VALUE, stream]);
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let destination = this.addr() + COLOR_HANDLE;
    let read = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read_function, &args![reader, destination, 4u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

// Translated from 00a51d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineColorInterpolator::GetViewerStrings` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the strings array): the base's
/// strings (`00a58200`), then the class name (the first word of the
/// `NiRTTI` record) and `m_kColorACPHandle`, each appended.
pub fn ni_b_spline_color_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let handle = e.mem.u32(this.addr() + COLOR_HANDLE);
    let text = e
        .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![NAME_HANDLE, handle])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_0610,
            ni_b_spline_color_interpolator_get_channel_posed(Ptr, u32) -> bool
        ),
        entry!(
            0x00a5_0a40,
            ni_b_spline_color_interpolator_get_degree(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_0b10,
            ni_b_spline_color_interpolator_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1a00,
            ni_b_spline_color_interpolator_get_dimension(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_1a20,
            ni_b_spline_color_interpolator_get_control_handle(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_1a40,
            ni_b_spline_color_interpolator_set_control_handle(Ptr, u32, u32)
        ),
        entry!(0x00a5_1a60, fn_00a51a60(Ptr, Ptr, u32)),
        entry!(0x00a5_1ab0, fn_00a51ab0(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00a5_1b10,
            ni_b_spline_color_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_1bf0,
            ni_b_spline_color_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_1c60, fn_00a51c60() -> Ptr),
        entry!(
            0x00a5_1ca0,
            ni_b_spline_color_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_1cf0,
            ni_b_spline_color_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1d30,
            ni_b_spline_color_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_1d80,
            ni_b_spline_color_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a51aa0, ni_b_spline_color_interpolator_get_rtti() -> Ptr),
        entry!(0x00a51af0, ni_b_spline_color_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_color_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a51aa0, &args![]).u32(),
            NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_color_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a51af0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a51af0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    const POINTS: u32 = 0x0001_0400;

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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        e.register(BASE_CONSTRUCTOR, |_, a| word(a[0]));
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(GET_VALUE_DEGREE3, |e, a| {
            for i in 0..4 {
                e.mem.set_f32(a[2] + 4 * i, 0.5 + i as f32);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn channel_queries_answer_for_channel_zero() {
        let mut e = engine();
        e.register(POINTS, |e, a| word(e.mem.u32(a[0] + 0x30)));
        let mut slots = vec![0u32; 0x39];
        slots[(SLOT_GET_CONTROL_POINT_COUNT / 4) as usize] = POINTS;
        e.put_vtable(0x0002_0000, &slots);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this, 0x0002_0000);
        assert!(e.call(0x00a5_0610, &args![this, 0x0001_0000u32]).bool());
        assert_eq!(calls(&e, POINTS), vec![vec![this, 0]]);
        e.mem.set_u32(this + 0x30, 8);
        assert!(!e.call(0x00a5_0610, &args![this, 0u32]).bool());
        assert!(!e.call(0x00a5_0610, &args![this, 1u32]).bool());
        assert_eq!(calls(&e, POINTS).len(), 2);
        assert_eq!(e.call(0x00a5_0a40, &args![this, 0u32]).u32(), 3);
        assert_eq!(e.call(0x00a5_0a40, &args![this, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00a5_1a00, &args![this, 0x0003_0000u32]).u32(), 4);
        assert_eq!(e.call(0x00a5_1a00, &args![this, 2u32]).u32(), 0);
        e.call(0x00a5_1a40, &args![this, 0x0002_0004u32, 0u32]);
        e.call(0x00a5_1a40, &args![this, 7u32, 1u32]);
        assert_eq!(e.call(0x00a5_1a20, &args![this, 0u32]).u32(), 0x0002_0004);
        assert_eq!(e.call(0x00a5_1a20, &args![this, 1u32]).u32(), 0xffff);
    }

    #[test]
    fn register_streamables_answers_the_base() {
        let mut e = engine();
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(0x200));
        assert!(!e.call(0x00a5_0b10, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(
            calls(&e, BASE_REGISTER_STREAMABLES),
            vec![vec![0x1000, 0x2000]]
        );
        e.register(BASE_REGISTER_STREAMABLES, |_, _| word(1));
        assert!(e.call(0x00a5_0b10, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn constructors_and_copies_set_color_and_handle() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.write(this, &[0xcd; 0x34]);
        assert_eq!(
            e.call(0x00a5_1ab0, &args![this, 0x4000u32, 9u32, 0x5000u32])
                .u32(),
            this
        );
        assert_eq!(
            calls(&e, BASE_CONSTRUCTOR),
            vec![vec![this, 0x4000, 0x5000]]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + COLOR_VALUE + 4 * i), 0);
        }
        assert_eq!(e.mem.u32(this + COLOR_HANDLE), 9);
        let object = e.call(0x00a5_1c60, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + COLOR_HANDLE), 0xffff);
        for i in 0..4 {
            e.mem.set_f32(this + COLOR_VALUE + 4 * i, i as f32);
        }
        e.call(0x00a5_1a60, &args![this, object, 0x900u32]);
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this, object, 0x900]]
        );
        assert_eq!(e.mem.f32(object + COLOR_VALUE + 12), 3.0);
        assert_eq!(e.mem.u32(object + COLOR_HANDLE), 9);
        let clone = e.call(0x00a5_1bf0, &args![this, 0x900u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(e.mem.f32(clone + COLOR_VALUE + 8), 2.0);
        assert_eq!(e.mem.u32(clone + COLOR_HANDLE), 9);
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_1c60, &args![]).u32(), 0);
        e.map(0, 0x40);
        assert_eq!(e.call(0x00a5_1bf0, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.mem.u32(COLOR_HANDLE), 9);
    }

    #[test]
    fn update_evaluates_four_floats() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        let color = e.mem.alloc(0x10);
        e.mem.set_f32(this + START_TIME, 0.0);
        e.mem.set_f32(this + END_TIME, 4.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + COLOR_HANDLE, 2);
        assert!(e
            .call(0x00a5_1b10, &args![this, 1.0f32, 0u32, color])
            .bool());
        let call = &calls(&e, GET_VALUE_DEGREE3)[0];
        assert_eq!((call[0], call[1]), (0x4000, 0.25f32.to_bits()));
        assert_eq!(call[3..], [4, 0x5000, 2]);
        for i in 0..4 {
            assert_eq!(e.mem.f32(color + 4 * i), 0.5 + i as f32);
        }
        assert_eq!(e.mem.f32(this + LAST_TIME), 1.0);
        e.mem.set_f32(this + COLOR_VALUE, 9.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_1b10, &args![this, 1.0f32, 0u32, color])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(color), 9.0);
        e.mem.set_u32(this + COLOR_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_1b10, &args![this, 2.0f32, 0u32, color])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
    }

    #[test]
    fn save_and_load_binary_handle_color_and_handle() {
        const IO: u32 = 0x0300_0500;
        let mut e = engine();
        for addr in [
            BASE_SAVE_BINARY,
            BASE_LOAD_BINARY,
            COLOR_SAVE_BINARY,
            COLOR_LOAD_BINARY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(IO, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            word(4)
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, IO);
        e.mem.set_u32(file + 0xc, IO);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a5_1ca0, &args![this, stream]);
        e.call(0x00a5_1d30, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(
            calls(&e, COLOR_SAVE_BINARY),
            vec![vec![this + 0x1c, stream]]
        );
        assert_eq!(calls(&e, BASE_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(
            calls(&e, COLOR_LOAD_BINARY),
            vec![vec![this + 0x1c, stream]]
        );
        let io = calls(&e, IO);
        assert_eq!((io[0][1], io[1][1]), (this + 0x2c, this + 0x2c));
        assert_eq!(e.mem.u32(file + 4), 8);
    }

    #[test]
    fn is_equal_compares_base_color_and_handle() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        e.register(COLOR_NOT_EQUAL, |_, _| word(0));
        let this = e.mem.alloc(OBJECT_SIZE);
        let other = e.mem.alloc(OBJECT_SIZE);
        assert!(e.call(0x00a5_1cf0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, COLOR_NOT_EQUAL),
            vec![vec![this + 0x1c, other + 0x1c]]
        );
        e.mem.set_u32(other + COLOR_HANDLE, 3);
        assert!(!e.call(0x00a5_1cf0, &args![this, other]).bool());
        e.mem.set_u32(other + COLOR_HANDLE, 0);
        e.register(COLOR_NOT_EQUAL, |_, _| word(1));
        assert!(!e.call(0x00a5_1cf0, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_1cf0, &args![this, other]).bool());
    }

    #[test]
    fn get_viewer_strings_appends_name_and_handle() {
        let mut e = engine();
        e.map(NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_COLOR_INTERPOLATOR_RTTI, 0x0109_bbbbu32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x51));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0x52));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this + COLOR_HANDLE, 6);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call(0x00a5_1d80, &args![this, strings]);
        assert_eq!(
            calls(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_bbbb]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![NAME_HANDLE, 6]]
        );
        assert!(calls(&e, STRINGS_GROW).is_empty());
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 2);
    }
}
