//! `NiAnimation/nibsplinepoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplinePoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_3ff4;

// Translated from 00a506b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI)
}

/// `NiBSplineInterpolator::LoadBinary` (`00a58080`) and `CopyMembers`
/// (`00a57fd0`, `thiscall (this, dest, cloning process)`; the engine map
/// has no name); `NiColor::LoadBinary` (`00a59be0`, `thiscall (this,
/// stream)`: three floats).
const BASE_LOAD_BINARY: u32 = 0x00a5_8080;
const BASE_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;

// Translated from 00a50680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::CopyMembers` (body; the engine map has no
/// name): the base class's `CopyMembers`, then the point value (+0x1C, 3
/// words) and the control-point handle (+0x28) copied word for word.
pub fn fn_00a50680(e: &mut Engine, this: Ptr, dest: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, dest, cloning]);
    for offset in [0x1c, 0x20, 0x24, 0x28] {
        let value = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(dest.addr().wrapping_add(offset), value);
    }
}

// Translated from 00a50910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::LoadBinary` (Xbox PDB): the base class's
/// `LoadBinary`, the point value loaded as three floats (`NiColor`'s
/// loader on +0x1C), then the control-point handle read (4 bytes into
/// +0x28 through the stream reader's function at its +8, `cdecl (reader,
/// this + 0x28, 4, &4, 1)`, the answer added to the reader's +4).
pub fn ni_b_spline_point3_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    e.call(COLOR_LOAD_BINARY, &args![this.addr() + 0x1c, stream]);
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let destination = this.addr() + 0x28;
    let read = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read_function, &args![reader, destination, 4u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

/// The vtable of `NiBSplinePoint3Interpolator` (RTTI
/// `.?AVNiBSplinePoint3Interpolator@@`) and the object's size.
const VTABLE: u32 = 0x0109_96dc;
const OBJECT_SIZE: u32 = 0x2c;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplineInterpolator(NiBSplineData*, NiBSplineBasisData*)`
/// constructor (`00a57f20`, `thiscall (this; data, basis data)`), its
/// `IsEqual` (`00a57e90`), `SaveBinary` (`00a58160`) and
/// `GetViewerStrings` (`00a58200`); `thiscall`, one word each.
const BASE_CONSTRUCTOR: u32 = 0x00a5_7f20;
const BASE_IS_EQUAL: u32 = 0x00a5_7e90;
const BASE_SAVE_BINARY: u32 = 0x00a5_8160;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a5_8200;
/// `NiBSplineData::GetValueDegree3` (`00a52550`, Xbox PDB; `thiscall
/// (data; time, &result, dimension, basis data, handle)`).
const GET_VALUE_DEGREE3: u32 = 0x00a5_2550;
/// `NiColor::SaveBinary` (`00a69780`, engine map; `thiscall (floats;
/// stream)`: three floats).
const COLOR_SAVE_BINARY: u32 = 0x00a6_9780;
/// `00439090` (`thiscall (point; other)`, no engine map name): the
/// negation of `004390c0`'s answer on the two points; by its use here, true
/// when they differ.
const POINT3_NOT_EQUAL: u32 = 0x0043_9090;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`) and its unsigned
/// overload (`00a7d510`, `cdecl (name, value)`); the viewer-string array's
/// grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_kPoint3CPHandle"`.
const NAME_HANDLE: u32 = 0x0109_97dc;
/// `NiBSplineInterpolator` fields (Xbox PDB): `m_fLastTime` +8,
/// `m_fStartTime` +0xC, `m_fEndTime` +0x10, `m_spData` +0x14,
/// `m_spBasisData` +0x18; this class's point value +0x1C (three floats) and
/// control-point handle +0x28.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const POINT3_VALUE: u32 = 0x1c;
const POINT3_HANDLE: u32 = 0x28;
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

/// A new point3 interpolator in a block from `operator new`: the base
/// constructor with no data and no basis data, the vtable, the handle
/// 0xFFFF. Null when the allocation fails.
fn new_point3(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(BASE_CONSTRUCTOR, &args![object, 0u32, 0u32]);
    e.mem.set_u32(object, VTABLE);
    e.mem.set_u32(object + POINT3_HANDLE, INVALID_HANDLE);
    object
}

// Translated from 00a50640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::GetControlHandle` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the channel, compared as 16
/// bits): the handle for channel 0, 0xFFFF for any other.
pub fn ni_b_spline_point3_interpolator_get_control_handle(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> u32 {
    if channel as u16 != 0 {
        return INVALID_HANDLE;
    }
    e.mem.u32(this.addr() + POINT3_HANDLE)
}

// Translated from 00a50660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::SetControlHandle` (Xbox PDB, matched by
/// the engine map; `thiscall (this; handle, channel)`): stores the whole
/// handle word for channel 0 (16-bit compare), nothing for another.
pub fn ni_b_spline_point3_interpolator_set_control_handle(
    e: &mut Engine,
    this: Ptr,
    handle: u32,
    channel: u32,
) {
    if channel as u16 == 0 {
        e.mem.set_u32(this.addr() + POINT3_HANDLE, handle);
    }
}

// Translated from 00a50710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::Update` (Xbox PDB, matched by the engine
/// map; `thiscall (this; time, target, &point)`, `RET 0xC`): at the last
/// time (`FUCOMP` equal) the value is copied out. Otherwise, unless the
/// handle is 0xFFFF, the value becomes the spline data's degree-3 value
/// (`00a52550`, dimension 3, into a stack buffer) at the normalized time
/// `(time - start) / (end - start)` (x87, stored as a `float`), its three
/// floats copied in through the FPU; then the value is copied out and the
/// time (through the FPU) becomes the last time. Always true.
pub fn ni_b_spline_point3_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: u32,
    point: Ptr,
) -> bool {
    let object = this.addr();
    let copy_out = |e: &mut Engine| {
        for i in 0..3 {
            let value = e.mem.u32(object + POINT3_VALUE + 4 * i);
            e.mem.set_u32(point.addr().wrapping_add(4 * i), value);
        }
    };
    let last = x87_load32(e.mem.u32(object + LAST_TIME));
    let now = x87_load32(time.to_bits());
    if now == last {
        copy_out(e);
        return true;
    }
    let handle = e.mem.u32(object + POINT3_HANDLE);
    if handle != INVALID_HANDLE {
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(now, x87_m32(start));
        let basis = e.mem.u32(object + BASIS_DATA);
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let normalized = x87_quiet_bits(x87_store32(x87_div(from_start, span)));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.with_stack(0xc, |e, result| {
            e.call(
                GET_VALUE_DEGREE3,
                &args![data, normalized, result, 3u32, basis, handle],
            );
            for i in 0..3 {
                let value = x87_quiet_bits(e.mem.u32(result.addr() + 4 * i));
                e.mem.set_u32(object + POINT3_VALUE + 4 * i, value);
            }
        });
    }
    copy_out(e);
    e.mem
        .set_u32(object + LAST_TIME, x87_quiet_bits(time.to_bits()));
    true
}

// Translated from 00a507e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::CreateClone` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the cloning process): a new object as
/// the factory builds it, the base `CopyMembers` (`00a57fd0`) into it, then
/// the point value and the handle copied as words (into address 0 when the
/// allocation failed, as the exe does). Answers the new object.
pub fn ni_b_spline_point3_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: u32,
) -> Ptr {
    let object = new_point3(e);
    e.call(BASE_COPY_MEMBERS, &args![this, object, cloning]);
    for offset in [0x1c, 0x20, 0x24, 0x28] {
        let value = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(object.wrapping_add(offset), value);
    }
    Ptr::new(object)
}

// Translated from 00a50840 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; registered for
/// `"NiBSplinePoint3Interpolator"` by `00a2d8a0`; Ghidra has no function
/// here and the engine map no name): a new point3 interpolator (no data, no
/// basis data, handle 0xFFFF), null when the allocation fails.
pub fn fn_00a50840(e: &mut Engine) -> Ptr {
    Ptr::new(new_point3(e))
}

// Translated from 00a50880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the base `SaveBinary`
/// (`00a58160`), the point value as three floats (`NiColor::SaveBinary` on
/// +0x1C), then the handle (4 bytes from +0x28 through the stream writer's
/// function at its +0xC, `cdecl (writer, this + 0x28, 4, &4, 1)`, the
/// answer added to the writer's +4).
pub fn ni_b_spline_point3_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.call(
        COLOR_SAVE_BINARY,
        &args![this.addr() + POINT3_VALUE, stream],
    );
    let writer = e.mem.u32(stream.addr() + 0x250);
    let write_function = e.mem.u32(writer.wrapping_add(0xc));
    let source = this.addr() + POINT3_HANDLE;
    let written = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write_function, &args![writer, source, 4u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(writer.wrapping_add(4)).wrapping_add(written);
    e.mem.set_u32(writer.wrapping_add(4), position);
}

// Translated from 00a508d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::IsEqual` (Xbox PDB, matched by the engine
/// map; `thiscall`, one word: the other object): the base `IsEqual`
/// (`00a57e90`), the same point value (`00439090` answering false) and the
/// same handle.
pub fn ni_b_spline_point3_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.call(
        POINT3_NOT_EQUAL,
        &args![this.addr() + POINT3_VALUE, other.addr() + POINT3_VALUE],
    )
    .bool()
    {
        return false;
    }
    e.mem.u32(other.addr() + POINT3_HANDLE) == e.mem.u32(this.addr() + POINT3_HANDLE)
}

// Translated from 00a50960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplinePoint3Interpolator::GetViewerStrings` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the strings array): the base's
/// strings (`00a58200`), then the class name (the first word of the
/// `NiRTTI` record) and `m_kPoint3CPHandle`, each appended.
pub fn ni_b_spline_point3_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let handle = e.mem.u32(this.addr() + POINT3_HANDLE);
    let text = e
        .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![NAME_HANDLE, handle])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_0640,
            ni_b_spline_point3_interpolator_get_control_handle(Ptr, u32) -> u32
        ),
        entry!(
            0x00a5_0660,
            ni_b_spline_point3_interpolator_set_control_handle(Ptr, u32, u32)
        ),
        entry!(
            0x00a5_0710,
            ni_b_spline_point3_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_07e0,
            ni_b_spline_point3_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_0840, fn_00a50840() -> Ptr),
        entry!(
            0x00a5_0880,
            ni_b_spline_point3_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_08d0,
            ni_b_spline_point3_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_0960,
            ni_b_spline_point3_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a506b0, ni_b_spline_point3_interpolator_get_rtti() -> Ptr),
        entry!(0x00a5_0680, fn_00a50680(Ptr, Ptr, Ptr)),
        entry!(
            0x00a5_0910,
            ni_b_spline_point3_interpolator_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a506b0, &args![]).u32(),
            NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn b_spline_point3_copy_members_copies_value_and_handle() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2c);
        let dest = e.mem.alloc(0x2c);
        for (i, offset) in [0x1c, 0x20, 0x24, 0x28].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x7f80_0001 + i as u32);
        }
        e.call(0x00a5_0680, &args![this, dest, 0x7000u32]);
        for (i, offset) in [0x1c, 0x20, 0x24, 0x28].iter().enumerate() {
            assert_eq!(e.mem.u32(dest + offset), 0x7f80_0001 + i as u32);
        }
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_COPY_MEMBERS && call.1 == vec![this, dest, 0x7000]));
    }

    #[test]
    fn b_spline_point3_load_binary_loads_the_point_then_the_handle() {
        const READ: u32 = 0x0300_0000;
        let mut e = Engine::new();
        e.register(BASE_LOAD_BINARY, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |_, _| 4u32.into_ret());
        e.call_log = Some(vec![]);
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x2c);
        e.call(0x00a5_0910, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().skip(1).map(|call| call.0).collect();
        assert_eq!(order, vec![BASE_LOAD_BINARY, COLOR_LOAD_BINARY, READ]);
        assert_eq!(log[2].1, vec![this + 0x1c, stream]);
        assert_eq!((log[3].1[1], log[3].1[2]), (this + 0x28, 4));
        assert_eq!(e.mem.u32(reader + 4), 4);
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
            for i in 0..3 {
                e.mem.set_f32(a[2] + 4 * i, 1.0 + i as f32);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn control_handle_is_for_channel_zero_only() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a5_0660, &args![this, 0x1234_0007u32, 0x0001_0000u32]);
        assert_eq!(e.mem.u32(this + POINT3_HANDLE), 0x1234_0007);
        e.call(0x00a5_0660, &args![this, 9u32, 1u32]);
        assert_eq!(e.mem.u32(this + POINT3_HANDLE), 0x1234_0007);
        assert_eq!(
            e.call(0x00a5_0640, &args![this, 0x0002_0000u32]).u32(),
            0x1234_0007
        );
        assert_eq!(e.call(0x00a5_0640, &args![this, 1u32]).u32(), 0xffff);
    }

    #[test]
    fn update_evaluates_the_spline_at_the_normalized_time() {
        let mut e = engine();
        let this = e.mem.alloc(OBJECT_SIZE);
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(this + START_TIME, 1.0);
        e.mem.set_f32(this + END_TIME, 5.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + POINT3_HANDLE, 6);
        assert!(e
            .call(0x00a5_0710, &args![this, 2.0f32, 0u32, point])
            .bool());
        let call = &calls(&e, GET_VALUE_DEGREE3)[0];
        assert_eq!((call[0], call[1]), (0x4000, 0.25f32.to_bits()));
        assert_eq!(call[3..], [3, 0x5000, 6]);
        for i in 0..3 {
            assert_eq!(e.mem.f32(this + POINT3_VALUE + 4 * i), 1.0 + i as f32);
            assert_eq!(e.mem.f32(point + 4 * i), 1.0 + i as f32);
        }
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        // The same time: only the copy.
        e.mem.set_f32(this + POINT3_VALUE, 7.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_0710, &args![this, 2.0f32, 0u32, point])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(point), 7.0);
        // Handle 0xFFFF: no evaluation, the time recorded.
        e.mem.set_u32(this + POINT3_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_0710, &args![this, 3.0f32, 0u32, point])
            .bool());
        assert!(calls(&e, GET_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 3.0);
        assert_eq!(e.mem.f32(point), 7.0);
    }

    #[test]
    fn factory_and_create_clone_build_point3_interpolators() {
        let mut e = engine();
        let object = e.call(0x00a5_0840, &args![]).u32();
        assert_eq!(calls(&e, BASE_CONSTRUCTOR), vec![vec![object, 0, 0]]);
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.u32(object + POINT3_HANDLE), 0xffff);
        let this = e.mem.alloc(OBJECT_SIZE);
        for (i, offset) in [0x1cu32, 0x20, 0x24, 0x28].iter().enumerate() {
            e.mem.set_u32(this + offset, 0x100 + i as u32);
        }
        let clone = e.call(0x00a5_07e0, &args![this, 0x900u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(calls(&e, BASE_COPY_MEMBERS), vec![vec![this, clone, 0x900]]);
        for (i, offset) in [0x1cu32, 0x20, 0x24, 0x28].iter().enumerate() {
            assert_eq!(e.mem.u32(clone + offset), 0x100 + i as u32);
        }
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_0840, &args![]).u32(), 0);
        e.map(0, 0x40);
        assert_eq!(e.call(0x00a5_07e0, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.mem.u32(0x28), 0x103);
    }

    #[test]
    fn save_binary_writes_value_and_handle() {
        const WRITE: u32 = 0x0300_0300;
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        e.register(COLOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            word(4)
        });
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(OBJECT_SIZE);
        e.call(0x00a5_0880, &args![this, stream]);
        assert_eq!(calls(&e, BASE_SAVE_BINARY), vec![vec![this, stream]]);
        assert_eq!(
            calls(&e, COLOR_SAVE_BINARY),
            vec![vec![this + 0x1c, stream]]
        );
        assert_eq!(calls(&e, WRITE)[0][..2], [writer, this + 0x28]);
        assert_eq!(e.mem.u32(writer + 4), 4);
    }

    #[test]
    fn is_equal_compares_base_value_and_handle() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| word(1));
        e.register(POINT3_NOT_EQUAL, |_, _| word(0));
        let this = e.mem.alloc(OBJECT_SIZE);
        let other = e.mem.alloc(OBJECT_SIZE);
        assert!(e.call(0x00a5_08d0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, POINT3_NOT_EQUAL),
            vec![vec![this + 0x1c, other + 0x1c]]
        );
        e.mem.set_u32(other + POINT3_HANDLE, 1);
        assert!(!e.call(0x00a5_08d0, &args![this, other]).bool());
        e.mem.set_u32(other + POINT3_HANDLE, 0);
        e.register(POINT3_NOT_EQUAL, |_, _| word(1));
        assert!(!e.call(0x00a5_08d0, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| word(0));
        e.register(POINT3_NOT_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_08d0, &args![this, other]).bool());
    }

    #[test]
    fn get_viewer_strings_appends_name_and_handle() {
        let mut e = engine();
        e.map(NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_POINT3_INTERPOLATOR_RTTI, 0x0109_9999u32);
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x31));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0x32));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u32(this + POINT3_HANDLE, 5);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a5_0960, &args![this, strings]);
        assert_eq!(
            calls(&e, BASE_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_9999]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![NAME_HANDLE, 5]]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
    }
}
