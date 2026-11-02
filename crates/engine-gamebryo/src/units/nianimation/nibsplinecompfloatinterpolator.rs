//! `NiAnimation/nibsplinecompfloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineCompFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_401c;

// Translated from 00a51520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_comp_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI)
}

// Translated from 00a506f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x2c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_comp_float_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

/// `NiBSplineCompFloatInterpolator`'s vtable (`01099b2c`) and size, and
/// the `float` its offset and half range start as (`01099b24`).
const COMP_FLOAT_VTABLE: u32 = 0x0109_9b2c;
const COMP_FLOAT_SIZE: u32 = 0x2c;
const COMP_FLOAT_DEFAULT: u32 = 0x0109_9b24;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplineFloatInterpolator` constructor (`00a509f0`, `thiscall
/// (this, data, handle, basis data)`), `CopyMembers` (`00a50aa0`, `thiscall
/// (this, dest, cloning process)`) and `LoadBinary` (`00a50cd0`,
/// `thiscall (this, stream)`).
const FLOAT_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_09f0;
const FLOAT_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_0aa0;
const FLOAT_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_0cd0;
/// `m_afCompScalars` (Xbox PDB): the offset +0x24 and half range +0x28.
const COMP_OFFSET: u32 = 0x24;
const COMP_HALF_RANGE: u32 = 0x28;

/// Builds a new `NiBSplineCompFloatInterpolator` in a block from
/// `operator new`: the float interpolator constructor with no data, the
/// handle 0xFFFF and no basis data, the class's vtable, both scalars the
/// default (one `FLD` stored twice). Null when the allocation fails.
fn new_comp_float(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![COMP_FLOAT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(
        FLOAT_INTERPOLATOR_CONSTRUCTOR,
        &args![object, 0u32, 0xffffu32, 0u32],
    );
    let default = x87_quiet_bits(e.global::<u32>(COMP_FLOAT_DEFAULT));
    e.mem.set_u32(object, COMP_FLOAT_VTABLE);
    e.mem.set_u32(object + COMP_OFFSET, default);
    e.mem.set_u32(object + COMP_HALF_RANGE, default);
    object
}

// Translated from 00a51620 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (no Ghidra function; the engine map has no name): a
/// new `NiBSplineCompFloatInterpolator` (no data, handle 0xFFFF, both
/// scalars the default `01099b24`), null when the allocation fails.
pub fn fn_00a51620(e: &mut Engine) -> Ptr {
    Ptr::new(new_comp_float(e))
}

// Translated from 00a51660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::LoadBinary` (Xbox PDB): the float
/// interpolator's `LoadBinary`, then the two scalars read in one call to
/// the stream reader's function at its +8 (`cdecl (reader, this + 0x24, 8,
/// &4, 1)`), the answer added to the reader's +4.
pub fn ni_b_spline_comp_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(FLOAT_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
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

// Translated from 00a515c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::CreateClone` (Xbox PDB): a new object
/// as the factory builds it, the float interpolator's members copied into
/// it (`00a50aa0`), then both scalars (through the FPU); into address 0
/// when the allocation failed, as the exe does. Answers the new object.
pub fn ni_b_spline_comp_float_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let object = new_comp_float(e);
    e.call(
        FLOAT_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    let offset = x87_quiet_bits(e.mem.u32(this.addr() + COMP_OFFSET));
    e.mem.set_u32(object.wrapping_add(COMP_OFFSET), offset);
    let half_range = x87_quiet_bits(e.mem.u32(this.addr() + COMP_HALF_RANGE));
    e.mem
        .set_u32(object.wrapping_add(COMP_HALF_RANGE), half_range);
    Ptr::new(object)
}

/// The `NiBSplineFloatInterpolator` `IsEqual` (`00a50b30`), `SaveBinary`
/// (`00a50c70`) and `GetViewerStrings` (`00a50d30`); `thiscall`, one word
/// each.
const FLOAT_INTERPOLATOR_IS_EQUAL: u32 = 0x00a5_0b30;
const FLOAT_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a5_0c70;
const FLOAT_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_0d30;
/// `NiBSplineData::GetCompactedValueDegree3` (`00a526b0`, `thiscall
/// (data; t, &result, dimension, basis data, handle, offset, half range)`,
/// `RET 0x1C`).
const GET_COMPACTED_VALUE_DEGREE3: u32 = 0x00a5_26b0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`); the viewer-string
/// array's grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// Fields (Xbox PDB names, PC offsets): `m_fLastTime` +8, `m_fStartTime`
/// +0xC, `m_fEndTime` +0x10, `m_spData` +0x14, `m_spBasisData` +0x18, the
/// float value +0x1C and the control-point handle +0x20.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const FLOAT_VALUE: u32 = 0x1c;
const FLOAT_HANDLE: u32 = 0x20;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;

// Translated from 00a514e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::IsEqual` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the other object): the float
/// interpolator's `IsEqual` (`00a50b30`) and both scalars equal
/// (`FUCOMPP`; a NaN is unequal).
pub fn ni_b_spline_comp_float_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(FLOAT_INTERPOLATOR_IS_EQUAL, &args![this, other])
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

// Translated from 00a51530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::Update` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, target, &value)`, `RET 0xC`): at the
/// last time (`FUCOMP` equal) the value is copied out (through the FPU).
/// Otherwise, unless the handle is 0xFFFF, the spline data's compacted
/// degree-3 value (`00a526b0`, dimension 1, the two scalars) at the
/// normalized time `(time - start) / (end - start)` (x87, stored as a
/// `float`) is written straight into the value; then the value is copied
/// out and the time (through the FPU) becomes the last time. Always true.
pub fn ni_b_spline_comp_float_interpolator_update(
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
        let half_range = x87_quiet_bits(e.mem.u32(object + COMP_HALF_RANGE));
        let basis = e.mem.u32(object + BASIS_DATA);
        let offset = x87_quiet_bits(e.mem.u32(object + COMP_OFFSET));
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(now, x87_m32(start));
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let normalized = x87_quiet_bits(x87_store32(x87_div(from_start, span)));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.call(
            GET_COMPACTED_VALUE_DEGREE3,
            &args![
                data,
                normalized,
                object + FLOAT_VALUE,
                1u32,
                basis,
                handle,
                offset,
                half_range
            ],
        );
    }
    let current = x87_quiet_bits(e.mem.u32(object + FLOAT_VALUE));
    e.mem.set_u32(value.addr(), current);
    e.mem
        .set_u32(object + LAST_TIME, x87_quiet_bits(time.to_bits()));
    true
}

// Translated from 00a516a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the float interpolator's
/// `SaveBinary` (`00a50c70`), then the two scalars written in one call to
/// the stream writer's function at its +0xC (`cdecl (writer, this + 0x24,
/// 8, &4, 1)`), the answer added to the writer's +4.
pub fn ni_b_spline_comp_float_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(FLOAT_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
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

// Translated from 00a516e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompFloatInterpolator::GetViewerStrings` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the strings array): the float
/// interpolator's strings (`00a50d30`), then the class name (the first word
/// of the `NiRTTI` record) appended: when the 16-bit size (+0xA) has
/// reached the capacity (+0x8) the array grows to the step (+0xE) plus the
/// size first, then the string is stored at the old size.
pub fn ni_b_spline_comp_float_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(FLOAT_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI);
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

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_14e0,
            ni_b_spline_comp_float_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1530,
            ni_b_spline_comp_float_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_16a0,
            ni_b_spline_comp_float_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_16e0,
            ni_b_spline_comp_float_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a51520, ni_b_spline_comp_float_interpolator_get_rtti() -> Ptr),
        entry!(0x00a506f0, ni_b_spline_comp_float_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a5_1620, fn_00a51620() -> Ptr),
        entry!(
            0x00a5_1660,
            ni_b_spline_comp_float_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_15c0, ni_b_spline_comp_float_interpolator_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_comp_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a51520, &args![]).u32(),
            NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_comp_float_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a506f0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a506f0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x2c]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }

    fn cf_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn cf_engine() -> Engine {
        let mut e = Engine::new();
        e.map(COMP_FLOAT_DEFAULT, 4);
        e.set_global(COMP_FLOAT_DEFAULT, 0x7f80_0003u32);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for addr in [
            FLOAT_INTERPOLATOR_CONSTRUCTOR,
            FLOAT_INTERPOLATOR_COPY_MEMBERS,
            FLOAT_INTERPOLATOR_LOAD_BINARY,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn comp_float_factory_builds_with_the_default_scalars() {
        let mut e = cf_engine();
        let object = e.call(0x00a5_1620, &args![]).u32();
        assert_eq!(
            cf_calls_to(&e, FLOAT_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![object, 0, 0xffff, 0]]
        );
        assert_eq!(e.mem.u32(object), COMP_FLOAT_VTABLE);
        assert_eq!(e.mem.u32(object + COMP_OFFSET), 0x7fc0_0003);
        assert_eq!(e.mem.u32(object + COMP_HALF_RANGE), 0x7fc0_0003);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_1620, &args![]).u32(), 0);
    }

    #[test]
    fn comp_float_load_binary_reads_both_scalars_at_once() {
        const READ: u32 = 0x0300_0000;
        let mut e = cf_engine();
        e.register(READ, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            8u32.into_ret()
        });
        let reader = e.mem.alloc(0xc);
        e.mem.set_u32(reader + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, reader);
        let this = e.mem.alloc(0x2c);
        e.call(0x00a5_1660, &args![this, stream]);
        assert_eq!(
            cf_calls_to(&e, FLOAT_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        let reads = cf_calls_to(&e, READ);
        assert_eq!(
            (reads[0][0], reads[0][1], reads[0][2], reads[0][4]),
            (reader, this + 0x24, 8, 1)
        );
        assert_eq!(e.mem.u32(reader + 4), 8);
    }

    #[test]
    fn comp_float_create_clone_copies_the_scalars() {
        let mut e = cf_engine();
        let this = e.mem.alloc(0x2c);
        e.mem.set_f32(this + COMP_OFFSET, 1.5);
        e.mem.set_f32(this + COMP_HALF_RANGE, 2.5);
        let clone = e.call(0x00a5_15c0, &args![this, 0x7000u32]).u32();
        assert_eq!(
            cf_calls_to(&e, FLOAT_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        assert_eq!(e.mem.u32(clone), COMP_FLOAT_VTABLE);
        assert_eq!(e.mem.f32(clone + COMP_OFFSET), 1.5);
        assert_eq!(e.mem.f32(clone + COMP_HALF_RANGE), 2.5);
        e.map(0, 0x40);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_15c0, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(e.mem.f32(COMP_OFFSET), 1.5);
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
    fn is_equal_compares_both_scalars() {
        let mut e = Engine::new();
        e.register(FLOAT_INTERPOLATOR_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(COMP_FLOAT_SIZE);
        let other = e.mem.alloc(COMP_FLOAT_SIZE);
        assert!(e.call(0x00a5_14e0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_IS_EQUAL),
            vec![vec![this, other]]
        );
        e.mem.set_f32(other + COMP_OFFSET, 1.0);
        assert!(!e.call(0x00a5_14e0, &args![this, other]).bool());
        e.mem.set_f32(other + COMP_OFFSET, 0.0);
        e.mem.set_f32(other + COMP_HALF_RANGE, 1.0);
        assert!(!e.call(0x00a5_14e0, &args![this, other]).bool());
        e.register(FLOAT_INTERPOLATOR_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_14e0, &args![this, this]).bool());
    }

    #[test]
    fn update_evaluates_the_compacted_value() {
        let mut e = Engine::new();
        e.register(GET_COMPACTED_VALUE_DEGREE3, |e, a| {
            e.mem.set_f32(a[2], 6.5);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(COMP_FLOAT_SIZE);
        let value = e.mem.alloc(4);
        e.mem.set_f32(this + START_TIME, 0.0);
        e.mem.set_f32(this + END_TIME, 8.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + FLOAT_HANDLE, 1);
        e.mem.set_f32(this + COMP_OFFSET, 0.5);
        e.mem.set_f32(this + COMP_HALF_RANGE, 1.5);
        assert!(e
            .call(0x00a5_1530, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(
            calls(&e, GET_COMPACTED_VALUE_DEGREE3),
            vec![vec![
                0x4000,
                0.25f32.to_bits(),
                this + FLOAT_VALUE,
                1,
                0x5000,
                1,
                0.5f32.to_bits(),
                1.5f32.to_bits()
            ]]
        );
        assert_eq!(e.mem.f32(value), 6.5);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        e.mem.set_f32(this + FLOAT_VALUE, 3.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_1530, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(value), 3.0);
        e.mem.set_u32(this + FLOAT_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_1530, &args![this, 4.0f32, 0u32, value])
            .bool());
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 4.0);
    }

    #[test]
    fn save_binary_writes_both_scalars_at_once() {
        const WRITE: u32 = 0x0300_0700;
        let mut e = Engine::new();
        e.register(FLOAT_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (8, 4, 1));
            word(8)
        });
        e.call_log = Some(vec![]);
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + 0xc, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, writer);
        let this = e.mem.alloc(COMP_FLOAT_SIZE);
        e.call(0x00a5_16a0, &args![this, stream]);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls(&e, WRITE)[0][..2], [writer, this + 0x24]);
        assert_eq!(e.mem.u32(writer + 4), 8);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_COMP_FLOAT_INTERPOLATOR_RTTI, 0x0109_dddd_u32);
        e.register(FLOAT_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x71));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x71);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.call(0x00a5_16e0, &args![0x1000u32, strings]);
        assert_eq!(
            calls(&e, FLOAT_INTERPOLATOR_GET_VIEWER_STRINGS),
            vec![vec![0x1000, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_dddd]]);
        assert!(calls(&e, STRINGS_GROW).is_empty());
        assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 0]);
    }
}
