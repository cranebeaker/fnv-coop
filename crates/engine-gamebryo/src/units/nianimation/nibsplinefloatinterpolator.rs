//! `NiAnimation/nibsplinefloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3ffc;

// Translated from 00a50b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI)
}

// Translated from 00a50b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x24 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_float_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x24u32]);
    }
    this
}

/// `NiBSplineInterpolator::LoadBinary` (`00a58080`, `thiscall (this,
/// stream)`).
const BASE_LOAD_BINARY: u32 = 0x00a5_8080;

/// One read through the stream reader (`stream + 0x24c`): its function at
/// +8, `cdecl (reader, destination, size, &4, 1)` (the component size 4 in
/// the caller's argument slot), the answer added to the reader's +4.
fn read_through_stream(e: &mut Engine, stream: u32, destination: u32, size: u32) {
    let reader = e.mem.u32(stream.wrapping_add(0x24c));
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let read = e.with_stack(4, |e, component| {
        e.mem.set_u32(component.addr(), 4);
        e.call(
            read_function,
            &args![reader, destination, size, component, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

// Translated from 00a50cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary`, then two 4-byte reads, into +0x1C (the value) and +0x20
/// (the control-point handle).
pub fn ni_b_spline_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    read_through_stream(e, stream.addr(), this.addr() + 0x1c, 4);
    read_through_stream(e, stream.addr(), this.addr() + 0x20, 4);
}

/// The vtable of `NiBSplineFloatInterpolator` (RTTI
/// `.?AVNiBSplineFloatInterpolator@@`) and the object's size.
const VTABLE: u32 = 0x0109_97f4;
const OBJECT_SIZE: u32 = 0x24;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplineInterpolator(NiBSplineData*, NiBSplineBasisData*)`
/// constructor (`00a57f20`, `thiscall (this; data, basis data)`), its
/// `CopyMembers` (`00a57fd0`, `thiscall (this; dest, cloning)`), `IsEqual`
/// (`00a57e90`), `SaveBinary` (`00a58160`) and `GetViewerStrings`
/// (`00a58200`).
const BASE_CONSTRUCTOR: u32 = 0x00a5_7f20;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const BASE_IS_EQUAL: u32 = 0x00a5_7e90;
const BASE_SAVE_BINARY: u32 = 0x00a5_8160;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a5_8200;
/// `NiBSplineData::GetValueDegree3` (`00a52550`, Xbox PDB; `thiscall
/// (data; time, &result, dimension, basis data, handle)`).
const GET_VALUE_DEGREE3: u32 = 0x00a5_2550;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`), its float
/// (`00a7d550`) and unsigned (`00a7d510`) overloads (`cdecl (name,
/// value)`); the viewer-string array's grow (`0096ad30`, `thiscall (array;
/// new capacity)`) and store (`0096ae90`, `thiscall (array; index,
/// &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_fFloatValue"` and `"m_kFloatCPHandle"`.
const NAME_FLOAT_VALUE: u32 = 0x0109_703c;
const NAME_HANDLE: u32 = 0x0109_98f4;
/// `NiBSplineInterpolator` fields (Xbox PDB): `m_fLastTime` +8,
/// `m_fStartTime` +0xC, `m_fEndTime` +0x10, `m_spData` +0x14,
/// `m_spBasisData` +0x18; this class's float value +0x1C and control-point
/// handle +0x20.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const FLOAT_VALUE: u32 = 0x1c;
const FLOAT_HANDLE: u32 = 0x20;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;

/// One 4-byte write from `source` through the stream writer (`stream +
/// 0x250`): its function at +0xC, `cdecl (writer, source, 4, &4, 1)`, the
/// answer added to the writer's +4.
fn write_through_stream(e: &mut Engine, stream: u32, source: u32) {
    let writer = e.mem.u32(stream.wrapping_add(0x250));
    let write_function = e.mem.u32(writer.wrapping_add(0xc));
    let written = e.with_stack(4, |e, component| {
        e.mem.set_u32(component.addr(), 4);
        e.call(
            write_function,
            &args![writer, source, 4u32, component, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(writer.wrapping_add(4)).wrapping_add(written);
    e.mem.set_u32(writer.wrapping_add(4), position);
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

// Translated from 00a50a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::GetDimension` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits): 1
/// for channel 0, 0 for any other.
pub fn ni_b_spline_float_interpolator_get_dimension(
    _e: &mut Engine,
    _this: Ptr,
    channel: u32,
) -> u32 {
    u32::from(channel as u16 == 0)
}

// Translated from 00a50a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::GetControlHandle` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the channel, compared as 16 bits): the
/// handle for channel 0, 0xFFFF for any other.
pub fn ni_b_spline_float_interpolator_get_control_handle(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> u32 {
    if channel as u16 != 0 {
        return INVALID_HANDLE;
    }
    e.mem.u32(this.addr() + FLOAT_HANDLE)
}

// Translated from 00a50a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::SetControlHandle` (Xbox PDB, matched by the
/// engine map; `thiscall (this; handle, channel)`): stores the whole handle
/// word for channel 0 (16-bit compare), nothing for another.
pub fn ni_b_spline_float_interpolator_set_control_handle(
    e: &mut Engine,
    this: Ptr,
    handle: u32,
    channel: u32,
) {
    if channel as u16 == 0 {
        e.mem.set_u32(this.addr() + FLOAT_HANDLE, handle);
    }
}

// Translated from 00a50b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::IsEqual` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the other object): the base `IsEqual`
/// (`00a57e90`), the same value (`FUCOMPP`; a NaN is unequal) and the same
/// handle.
pub fn ni_b_spline_float_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let value = x87_load32(e.mem.u32(this.addr() + FLOAT_VALUE));
    let other_value = x87_load32(e.mem.u32(other.addr() + FLOAT_VALUE));
    if other_value != value {
        return false;
    }
    e.mem.u32(other.addr() + FLOAT_HANDLE) == e.mem.u32(this.addr() + FLOAT_HANDLE)
}

// Translated from 00a50ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::Update` (Xbox PDB, matched by the engine
/// map; `thiscall (this; time, target, &value)`, `RET 0xC`): at the last
/// time (`FUCOMP` equal) the value is copied out (through the FPU).
/// Otherwise, unless the handle is 0xFFFF, the spline data's degree-3 value
/// (`00a52550`, dimension 1) at the normalized time `(time - start) / (end -
/// start)` (x87, stored as a `float`) is written straight into the value;
/// then the value is copied out and the time (through the FPU) becomes the
/// last time. Always true.
pub fn ni_b_spline_float_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: u32,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let last = x87_load32(e.mem.u32(object + LAST_TIME));
    let now = x87_load32(time.to_bits());
    if now == last {
        let current = x87_quiet_bits(e.mem.u32(object + FLOAT_VALUE));
        e.mem.set_u32(value.addr(), current);
        return true;
    }
    let handle = e.mem.u32(object + FLOAT_HANDLE);
    if handle != INVALID_HANDLE {
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(now, x87_m32(start));
        let basis = e.mem.u32(object + BASIS_DATA);
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let normalized = x87_quiet_bits(x87_store32(x87_div(from_start, span)));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.call(
            GET_VALUE_DEGREE3,
            &args![data, normalized, object + FLOAT_VALUE, 1u32, basis, handle],
        );
    }
    let current = x87_quiet_bits(e.mem.u32(object + FLOAT_VALUE));
    e.mem.set_u32(value.addr(), current);
    e.mem
        .set_u32(object + LAST_TIME, x87_quiet_bits(time.to_bits()));
    true
}

// Translated from 00a50c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::CreateClone` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the cloning process): a 0x24-byte
/// block built by the base constructor with no data (`00a57f20`), the
/// vtable and the handle 0xFFFF (null when the allocation fails), the base
/// `CopyMembers` (`00a57fd0`) into it, then the value (through the FPU)
/// and the handle copied (into address 0 when the allocation failed, as the
/// exe does). Answers the new object.
pub fn ni_b_spline_float_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object != 0 {
        e.call(BASE_CONSTRUCTOR, &args![object, 0u32, 0u32]);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + FLOAT_HANDLE, INVALID_HANDLE);
    }
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + FLOAT_VALUE));
    e.mem.set_u32(object.wrapping_add(FLOAT_VALUE), value);
    let handle = e.mem.u32(this.addr() + FLOAT_HANDLE);
    e.mem.set_u32(object.wrapping_add(FLOAT_HANDLE), handle);
    Ptr::new(object)
}

// Translated from 00a50c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the base `SaveBinary`
/// (`00a58160`), then two 4-byte writes, from +0x1C (the value) and +0x20
/// (the control-point handle).
pub fn ni_b_spline_float_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    write_through_stream(e, stream.addr(), this.addr() + FLOAT_VALUE);
    write_through_stream(e, stream.addr(), this.addr() + FLOAT_HANDLE);
}

// Translated from 00a50d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineFloatInterpolator::GetViewerStrings` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the strings array): the base's
/// strings (`00a58200`), then the class name (the first word of the
/// `NiRTTI` record), `m_fFloatValue` (through the FPU) and
/// `m_kFloatCPHandle`, each appended.
pub fn ni_b_spline_float_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + FLOAT_VALUE));
    let text = e
        .call(NI_GET_VIEWER_STRING_FLOAT, &args![NAME_FLOAT_VALUE, value])
        .u32();
    push_viewer_string(e, strings, text);
    let handle = e.mem.u32(this.addr() + FLOAT_HANDLE);
    let text = e
        .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![NAME_HANDLE, handle])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_0a20,
            ni_b_spline_float_interpolator_get_dimension(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_0a60,
            ni_b_spline_float_interpolator_get_control_handle(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_0a80,
            ni_b_spline_float_interpolator_set_control_handle(Ptr, u32, u32)
        ),
        entry!(
            0x00a5_0b30,
            ni_b_spline_float_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_0ba0,
            ni_b_spline_float_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_0c20,
            ni_b_spline_float_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a5_0c70,
            ni_b_spline_float_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_0d30,
            ni_b_spline_float_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a50b70, ni_b_spline_float_interpolator_get_rtti() -> Ptr),
        entry!(0x00a50b80, ni_b_spline_float_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a5_0cd0,
            ni_b_spline_float_interpolator_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a50b70, &args![]).u32(),
            NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_float_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a50b80, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a50b80, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x24]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }

    #[test]
    fn b_spline_float_load_binary_reads_value_then_handle() {
        const READ: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            4u32.into_ret()
        });
        e.call_log = Some(vec![]);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 4, 0x10);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x24);
        e.call(0x00a5_0cd0, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let reads: Vec<_> = log.iter().filter(|call| call.0 == READ).collect();
        assert_eq!(reads.len(), 2);
        assert_eq!((reads[0].1[1], reads[0].1[2]), (this + 0x1c, 4));
        assert_eq!((reads[1].1[1], reads[1].1[2]), (this + 0x20, 4));
        assert_eq!(e.mem.u32(reader + 4), 0x18);
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_LOAD_BINARY && call.1 == vec![this, stream]));
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

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
            e.mem.set_f32(a[2], 4.5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn dimension_and_handle_are_for_channel_zero_only() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        assert_eq!(e.call(0x00a5_0a20, &args![this, 0x0001_0000u32]).u32(), 1);
        assert_eq!(e.call(0x00a5_0a20, &args![this, 2u32]).u32(), 0);
        e.call(0x00a5_0a80, &args![this, 0x0009_0003u32, 0x0005_0000u32]);
        assert_eq!(e.mem.u32(this + FLOAT_HANDLE), 0x0009_0003);
        e.call(0x00a5_0a80, &args![this, 1u32, 1u32]);
        assert_eq!(e.call(0x00a5_0a60, &args![this, 0u32]).u32(), 0x0009_0003);
        assert_eq!(e.call(0x00a5_0a60, &args![this, 3u32]).u32(), 0xffff);
    }

    #[test]
    fn is_equal_compares_value_and_handle() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        let this = e.mem.alloc(OBJECT_SIZE);
        let other = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_f32(this + FLOAT_VALUE, 1.5);
        e.mem.set_f32(other + FLOAT_VALUE, 1.5);
        assert!(e.call(0x00a5_0b30, &args![this, other]).bool());
        e.mem.set_u32(other + FLOAT_HANDLE, 2);
        assert!(!e.call(0x00a5_0b30, &args![this, other]).bool());
        e.mem.set_u32(other + FLOAT_HANDLE, 0);
        e.mem.set_f32(other + FLOAT_VALUE, f32::NAN);
        e.mem.set_f32(this + FLOAT_VALUE, f32::NAN);
        assert!(!e.call(0x00a5_0b30, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        e.mem.set_f32(this + FLOAT_VALUE, 1.5);
        e.mem.set_f32(other + FLOAT_VALUE, 1.5);
        assert!(!e.call(0x00a5_0b30, &args![this, other]).bool());
    }

    #[test]
    fn update_evaluates_into_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        let value = e.mem.alloc(4);
        e.mem.set_f32(this + START_TIME, 2.0);
        e.mem.set_f32(this + END_TIME, 6.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + FLOAT_HANDLE, 3);
        assert!(e
            .call(0x00a5_0ba0, &args![this, 5.0f32, 0u32, value])
            .bool());
        assert_eq!(
            calls(&e, GET_VALUE_DEGREE3),
            vec![vec![
                0x4000,
                0.75f32.to_bits(),
                this + FLOAT_VALUE,
                1,
                0x5000,
                3
            ]]
        );
        assert_eq!(e.mem.f32(value), 4.5);
        assert_eq!(e.mem.f32(this + LAST_TIME), 5.0);
        // The same time: only the copy.
        e.mem.set_f32(this + FLOAT_VALUE, 8.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_0ba0, &args![this, 5.0f32, 0u32, value])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(value), 8.0);
        // No control points: no evaluation, the time recorded.
        e.mem.set_u32(this + FLOAT_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_0ba0, &args![this, 7.0f32, 0u32, value])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 7.0);
    }

    #[test]
    fn create_clone_copies_value_and_handle() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_f32(this + FLOAT_VALUE, 2.5);
        e.mem.set_u32(this + FLOAT_HANDLE, 4);
        let clone = e.call(0x00a5_0c20, &args![this, 0x900u32]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x24]]);
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![clone, 0, 0]]);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![this, clone, 0x900]]);
        assert_eq!(e.mem.f32(clone + FLOAT_VALUE), 2.5);
        assert_eq!(e.mem.u32(clone + FLOAT_HANDLE), 4);
        e.map(0, 0x40);
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_0c20, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.mem.u32(FLOAT_HANDLE), 4);
    }

    #[test]
    fn save_binary_writes_value_and_handle() {
        const WRITE: u32 = 0x0300_0400;
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            word(4)
        });
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a5_0c70, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls(&e, WRITE);
        assert_eq!((writes[0][1], writes[1][1]), (this + 0x1c, this + 0x20));
        assert_eq!(e.mem.u32(writer + 4), 8);
    }

    #[test]
    fn get_viewer_strings_appends_name_value_and_handle() {
        let mut e = engine();
        e.map(NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_FLOAT_INTERPOLATOR_RTTI, 0x0109_aaaau32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x41));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| word(0x42));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0x43));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_f32(this + FLOAT_VALUE, 0.5);
        e.mem.set_u32(this + FLOAT_HANDLE, 2);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a5_0d30, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_aaaa]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![NAME_FLOAT_VALUE, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![NAME_HANDLE, 2]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 4]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 3);
    }
}
