//! `NiAnimation/nibsplinecomppoint3interpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineCompPoint3Interpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_COMP_POINT3_INTERPOLATOR_RTTI: u32 = 0x011f_4014;

// Translated from 00a51270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_comp_point3_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_COMP_POINT3_INTERPOLATOR_RTTI)
}

// Translated from 00a51480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00a50960`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `011f4014`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_b_spline_comp_point3_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00a5_0960;
    const CLASS_NAME_POINTER: u32 = 0x011f_4014;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
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

/// `NiBSplineCompPoint3Interpolator`'s vtable (`01099a24`) and size, and
/// the `float` its offset and half range start as (`01099a1c`).
const COMP_POINT3_VTABLE: u32 = 0x0109_9a24;
const COMP_POINT3_SIZE: u32 = 0x34;
const COMP_POINT3_DEFAULT: u32 = 0x0109_9a1c;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplinePoint3Interpolator` constructor (`00a506c0`, `thiscall
/// (this, data, handle, basis data)`), `CopyMembers` (`00a50680`,
/// `thiscall (this, dest, cloning process)`) and `LoadBinary` (`00a50910`,
/// `thiscall (this, stream)`).
const POINT3_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_06c0;
const POINT3_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_0680;
const POINT3_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_0910;
/// `NiBSplineData::GetCompactedValueDegree3` (`00a526b0`, `thiscall
/// (data, t, &result, dimension, basis data, handle, offset, half range)`,
/// `RET 0x1C`).
const GET_COMPACTED_VALUE_DEGREE3: u32 = 0x00a5_26b0;
/// `NiBSplinePoint3Interpolator` fields (Xbox PDB, PC offsets):
/// `m_fLastTime` +8, `m_fStartTime` +0xC, `m_fEndTime` +0x10, `m_spData`
/// +0x14, `m_spBasisData` +0x18, `m_kPoint3Value` +0x1C, the control-point
/// handle +0x28; this class's `m_afCompScalars`: offset +0x2C, half range
/// +0x30.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const POINT3_VALUE: u32 = 0x1c;
const POINT3_HANDLE: u32 = 0x28;
const COMP_OFFSET: u32 = 0x2c;
const COMP_HALF_RANGE: u32 = 0x30;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;

/// Builds a new `NiBSplineCompPoint3Interpolator` in a block from
/// `operator new`: the point3 interpolator constructor with no data, the
/// handle 0xFFFF and no basis data, the class's vtable, both scalars the
/// default (one `FLD` stored twice). Null when the allocation fails.
fn new_comp_point3(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![COMP_POINT3_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(
        POINT3_INTERPOLATOR_CONSTRUCTOR,
        &args![object, 0u32, INVALID_HANDLE, 0u32],
    );
    let default = x87_quiet_bits(e.global::<u32>(COMP_POINT3_DEFAULT));
    e.mem.set_u32(object, COMP_POINT3_VTABLE);
    e.mem.set_u32(object + COMP_OFFSET, default);
    e.mem.set_u32(object + COMP_HALF_RANGE, default);
    object
}

// Translated from 00a513c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (no Ghidra function; the engine map has no name): a
/// new `NiBSplineCompPoint3Interpolator` (no data, handle 0xFFFF, both
/// scalars the default `01099a1c`), null when the allocation fails.
pub fn fn_00a513c0(e: &mut Engine) -> Ptr {
    Ptr::new(new_comp_point3(e))
}

// Translated from 00a51400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::LoadBinary` (Xbox PDB): the point3
/// interpolator's `LoadBinary`, then the two scalars read in one call to
/// the stream reader's function at its +8 (`cdecl (reader, this + 0x2c, 8,
/// &4, 1)`), the answer added to the reader's +4.
pub fn ni_b_spline_comp_point3_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(POINT3_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let reader = e.mem.u32(stream.addr() + 0x24c);
    let read_function = e.mem.u32(reader.wrapping_add(8));
    let destination = this.addr() + COMP_OFFSET;
    let read = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read_function, &args![reader, destination, 8u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(reader.wrapping_add(4)).wrapping_add(read);
    e.mem.set_u32(reader.wrapping_add(4), position);
}

// Translated from 00a51360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::CreateClone` (Xbox PDB): a new object
/// as the factory builds it, the point3 interpolator's members copied into
/// it (`00a50680`), then both scalars (through the FPU); into address 0
/// when the allocation failed, as the exe does. Answers the new object.
pub fn ni_b_spline_comp_point3_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let object = new_comp_point3(e);
    e.call(
        POINT3_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    let offset = x87_quiet_bits(e.mem.u32(this.addr() + COMP_OFFSET));
    e.mem.set_u32(object.wrapping_add(COMP_OFFSET), offset);
    let half_range = x87_quiet_bits(e.mem.u32(this.addr() + COMP_HALF_RANGE));
    e.mem
        .set_u32(object.wrapping_add(COMP_HALF_RANGE), half_range);
    Ptr::new(object)
}

// Translated from 00a51280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::Update` (Xbox PDB; `thiscall (time,
/// target, &point)`, `RET 0xC`): at the last time the current value is
/// copied out. Otherwise, unless the handle is 0xFFFF, the value becomes
/// the spline data's compacted degree-3 value (dimension 3) at the
/// normalized time `(time - start) / (end - start)` (x87, stored as a
/// `float`), its three floats copied in through the FPU; then the value is
/// copied out and the last time set. Always true.
pub fn ni_b_spline_comp_point3_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _target: Ptr,
    point: Ptr,
) -> bool {
    let object = this.addr();
    let point = point.addr();
    let copy_out = |e: &mut Engine| {
        for i in 0..3 {
            let value = e.mem.u32(object + POINT3_VALUE + 4 * i);
            e.mem.set_u32(point.wrapping_add(4 * i), value);
        }
    };
    if time == e.mem.f32(object + LAST_TIME) {
        copy_out(e);
        return true;
    }
    let handle = e.mem.u32(object + POINT3_HANDLE);
    if handle != INVALID_HANDLE {
        let half_range = x87_quiet_bits(e.mem.u32(object + COMP_HALF_RANGE));
        let offset = x87_quiet_bits(e.mem.u32(object + COMP_OFFSET));
        let basis = e.mem.u32(object + BASIS_DATA);
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(x87_load32(time.to_bits()), x87_m32(start));
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let normalized = x87_store32(x87_div(from_start, span));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.with_stack(0xc, |e, result| {
            e.call(
                GET_COMPACTED_VALUE_DEGREE3,
                &args![data, normalized, result, 3u32, basis, handle, offset, half_range],
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

/// `NiBSplinePoint3Interpolator::IsEqual` (`00a508d0`) and `SaveBinary`
/// (`00a50880`); `thiscall`, one word each.
const POINT3_INTERPOLATOR_IS_EQUAL: u32 = 0x00a5_08d0;
const POINT3_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a5_0880;

// Translated from 00a51230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::IsEqual` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the other object): the point3
/// interpolator's `IsEqual` (`00a508d0`) and both scalars equal
/// (`FUCOMPP`; a NaN is unequal).
pub fn ni_b_spline_comp_point3_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    if !e
        .call(POINT3_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    for i in 0..2 {
        let ours = x87_load32(e.mem.u32(this.addr() + COMP_OFFSET + 4 * i));
        let theirs = x87_load32(e.mem.u32(other.addr() + COMP_OFFSET + 4 * i));
        if theirs != ours {
            return false;
        }
    }
    true
}

// Translated from 00a51440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompPoint3Interpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the point3
/// interpolator's `SaveBinary` (`00a50880`), then the two scalars written
/// in one call to the stream writer's function at its +0xC (`cdecl
/// (writer, this + 0x2c, 8, &4, 1)`), the answer added to the writer's +4.
pub fn ni_b_spline_comp_point3_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(POINT3_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream.addr() + 0x250);
    let write_function = e.mem.u32(writer.wrapping_add(0xc));
    let source = this.addr() + COMP_OFFSET;
    let written = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(write_function, &args![writer, source, 8u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(writer.wrapping_add(4)).wrapping_add(written);
    e.mem.set_u32(writer.wrapping_add(4), position);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_1230,
            ni_b_spline_comp_point3_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1440,
            ni_b_spline_comp_point3_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a51270, ni_b_spline_comp_point3_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a51480,
            ni_b_spline_comp_point3_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a5_13c0, fn_00a513c0() -> Ptr),
        entry!(
            0x00a5_1400,
            ni_b_spline_comp_point3_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_1360, ni_b_spline_comp_point3_interpolator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a5_1280, ni_b_spline_comp_point3_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_b_spline_comp_point3_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a51270, &args![]).u32(),
            NI_B_SPLINE_COMP_POINT3_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_comp_point3_interpolator_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00a5_0960;
        const NAME_POINTER: u32 = 0x011f_4014;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_b_spline_comp_point3_interpolator_get_viewer_strings(
                &mut e,
                this,
                Ptr::new(strings),
            );
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    fn cp_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn cp_engine() -> Engine {
        let mut e = Engine::new();
        e.map(COMP_POINT3_DEFAULT, 4);
        e.set_global(COMP_POINT3_DEFAULT, 0.5f32);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for addr in [
            POINT3_INTERPOLATOR_CONSTRUCTOR,
            POINT3_INTERPOLATOR_COPY_MEMBERS,
            POINT3_INTERPOLATOR_LOAD_BINARY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(GET_COMPACTED_VALUE_DEGREE3, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[2] + 4 * i, 1.0 + i as f32);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn comp_point3_factory_builds_with_the_default_scalars() {
        let mut e = cp_engine();
        let object = e.call(0x00a5_13c0, &args![]).u32();
        assert_eq!(
            cp_calls_to(&e, POINT3_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![object, 0, 0xffff, 0]]
        );
        assert_eq!(e.mem.u32(object), COMP_POINT3_VTABLE);
        assert_eq!(e.mem.f32(object + COMP_OFFSET), 0.5);
        assert_eq!(e.mem.f32(object + COMP_HALF_RANGE), 0.5);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_13c0, &args![]).u32(), 0);
    }

    #[test]
    fn comp_point3_load_binary_reads_both_scalars_at_once() {
        const READ: u32 = 0x0300_0000;
        let mut e = cp_engine();
        e.register(READ, |_, _| 8u32.into_ret());
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x34);
        e.call(0x00a5_1400, &args![this, stream]);
        assert_eq!(
            cp_calls_to(&e, POINT3_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        let reads = cp_calls_to(&e, READ);
        assert_eq!(
            (reads[0][0], reads[0][1], reads[0][2], reads[0][4]),
            (reader, this + 0x2c, 8, 1)
        );
        assert_eq!(e.mem.u32(reader + 4), 8);
    }

    #[test]
    fn comp_point3_create_clone_copies_the_scalars() {
        let mut e = cp_engine();
        let this = e.mem.alloc(0x34);
        e.mem.set_f32(this + COMP_OFFSET, 1.5);
        e.mem.set_f32(this + COMP_HALF_RANGE, 2.5);
        let clone = e.call(0x00a5_1360, &args![this, 0x7000u32]).u32();
        assert_eq!(
            cp_calls_to(&e, POINT3_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.f32(clone + COMP_OFFSET), 1.5);
        assert_eq!(e.mem.f32(clone + COMP_HALF_RANGE), 2.5);
        e.map(0, 0x40);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_1360, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(e.mem.f32(COMP_HALF_RANGE), 2.5);
    }

    #[test]
    fn comp_point3_update_evaluates_the_spline_at_the_normalized_time() {
        let mut e = cp_engine();
        let this = e.mem.alloc(0x34);
        let point = e.mem.alloc(0xc);
        e.mem.set_f32(this + START_TIME, 1.0);
        e.mem.set_f32(this + END_TIME, 3.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + POINT3_HANDLE, 6);
        e.mem.set_f32(this + COMP_OFFSET, 0.25);
        e.mem.set_f32(this + COMP_HALF_RANGE, 0.75);
        assert!(e
            .call(0x00a5_1280, &args![this, 2.0f32, 0u32, point])
            .bool());
        let calls = cp_calls_to(&e, GET_COMPACTED_VALUE_DEGREE3);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], 0x4000);
        assert_eq!(calls[0][1], 0.5f32.to_bits());
        assert_eq!(
            &calls[0][3..],
            &[3, 0x5000, 6, 0.25f32.to_bits(), 0.75f32.to_bits()]
        );
        for i in 0..3 {
            assert_eq!(e.mem.f32(this + POINT3_VALUE + 4 * i), 1.0 + i as f32);
            assert_eq!(e.mem.f32(point + 4 * i), 1.0 + i as f32);
        }
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        // At the last time only the copy.
        e.mem.set_f32(this + POINT3_VALUE, 9.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_1280, &args![this, 2.0f32, 0u32, point])
            .bool());
        assert!(cp_calls_to(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(point), 9.0);
        // Handle 0xFFFF: no evaluation, the time still recorded.
        e.mem.set_u32(this + POINT3_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_1280, &args![this, 2.5f32, 0u32, point])
            .bool());
        assert!(cp_calls_to(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
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

    #[test]
    fn is_equal_compares_the_scalars_after_the_base() {
        let mut e = Engine::new();
        e.register(POINT3_INTERPOLATOR_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        assert!(e.call(0x00a5_1230, &args![this, other]).bool());
        assert_eq!(
            calls(&e, POINT3_INTERPOLATOR_IS_EQUAL),
            vec![vec![this, other]]
        );
        for i in 0..2u32 {
            let at = COMP_OFFSET + 4 * i;
            e.mem.set_f32(other + at, 1.0);
            assert!(!e.call(0x00a5_1230, &args![this, other]).bool());
            e.mem.set_f32(other + at, 0.0);
        }
        e.register(POINT3_INTERPOLATOR_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_1230, &args![this, other]).bool());
    }

    #[test]
    fn save_binary_writes_the_scalars_at_once() {
        const WRITE: u32 = 0x0300_0800;
        let mut e = Engine::new();
        e.register(POINT3_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            word(a[2])
        });
        e.call_log = Some(vec![]);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(0x60);
        e.call(0x00a5_1440, &args![this, stream]);
        assert_eq!(
            calls(&e, POINT3_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls(&e, WRITE)[0][..3],
            [writer, this + COMP_OFFSET, 4 * 2u32]
        );
        assert_eq!(e.mem.u32(writer + 4), 4 * 2u32);
    }
}
