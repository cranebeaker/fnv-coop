//! `NiAnimation/nibsplinecompcolorinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineCompColorInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI: u32 = 0x011f_4024;

// Translated from 00a51770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_comp_color_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI)
}

// Translated from 00a51780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x3c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_comp_color_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

/// `NiBSplineCompColorInterpolator`'s vtable (RTTI
/// `.?AVNiBSplineCompColorInterpolator@@`) and size, and the `float` its
/// offset and half range start as (`01099c2c`).
const COMP_COLOR_VTABLE: u32 = 0x0109_9c34;
const COMP_COLOR_SIZE: u32 = 0x3c;
const COMP_COLOR_DEFAULT: u32 = 0x0109_9c2c;
/// `operator new` (`00aa13e0`, `cdecl (size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// The `NiBSplineColorInterpolator` constructor (`00a51ab0`, `thiscall
/// (this; data, handle, basis data)`), `CopyMembers` (`00a51a60`,
/// `thiscall (this; dest, cloning)`), `RegisterStreamables` (`00a50b10`),
/// `IsEqual` (`00a51cf0`), `LoadBinary` (`00a51d30`), `SaveBinary`
/// (`00a51ca0`) and `GetViewerStrings` (`00a51d80`).
const COLOR_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_1ab0;
const COLOR_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_1a60;
const COLOR_INTERPOLATOR_REGISTER_STREAMABLES: u32 = 0x00a5_0b10;
const COLOR_INTERPOLATOR_IS_EQUAL: u32 = 0x00a5_1cf0;
const COLOR_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_1d30;
const COLOR_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a5_1ca0;
const COLOR_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_1d80;
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
/// color value +0x1C (four floats), the control-point handle +0x2C; this
/// class's `m_afCompScalars`: offset +0x34, half range +0x38.
const LAST_TIME: u32 = 0x8;
const START_TIME: u32 = 0xc;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const COLOR_VALUE: u32 = 0x1c;
const COLOR_HANDLE: u32 = 0x2c;
const COMP_OFFSET: u32 = 0x34;
const COMP_HALF_RANGE: u32 = 0x38;
/// The handle that means "no control points".
const INVALID_HANDLE: u32 = 0xffff;

/// Builds a new `NiBSplineCompColorInterpolator` in a block from
/// `operator new`: the color interpolator constructor with no data, the
/// handle 0xFFFF and no basis data, the class's vtable, both scalars the
/// default (one `FLD` stored twice). Null when the allocation fails.
fn new_comp_color(e: &mut Engine) -> u32 {
    let object = e.call(OPERATOR_NEW, &args![COMP_COLOR_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(
        COLOR_INTERPOLATOR_CONSTRUCTOR,
        &args![object, 0u32, INVALID_HANDLE, 0u32],
    );
    let default = x87_quiet_bits(e.global::<u32>(COMP_COLOR_DEFAULT));
    e.mem.set_u32(object, COMP_COLOR_VTABLE);
    e.mem.set_u32(object + COMP_OFFSET, default);
    e.mem.set_u32(object + COMP_HALF_RANGE, default);
    object
}

/// One transfer of the two scalars through the stream's reader (+0x24C,
/// function at +8) or writer (+0x250, function at +0xC): `cdecl (file,
/// this + 0x34, 8, &4, 1)`, the answer added to the file's +4.
fn transfer_scalars(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
    file_offset: u32,
    function_offset: u32,
) {
    let file = e.mem.u32(stream.addr() + file_offset);
    let function = e.mem.u32(file.wrapping_add(function_offset));
    let scalars = this.addr() + COMP_OFFSET;
    let moved = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(function, &args![file, scalars, 8u32, size, 1u32])
            .u32()
    });
    let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(file.wrapping_add(4), position);
}

// Translated from 00a51210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::RegisterStreamables` (Xbox PDB, matched
/// by the engine map; `thiscall`, one word: the stream): the color
/// interpolator's `RegisterStreamables` answer (`00a50b10`).
pub fn ni_b_spline_comp_color_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(
        COLOR_INTERPOLATOR_REGISTER_STREAMABLES,
        &args![this, stream],
    )
    .bool()
}

// Translated from 00a51730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::IsEqual` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the other object): the color
/// interpolator's `IsEqual` (`00a51cf0`) and both scalars equal
/// (`FUCOMPP`; a NaN is unequal).
pub fn ni_b_spline_comp_color_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(COLOR_INTERPOLATOR_IS_EQUAL, &args![this, other])
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

// Translated from 00a517a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::Update` (Xbox PDB, matched by the
/// engine map; `thiscall (this; time, target, &color)`, `RET 0xC`): at the
/// last time (`FUCOMP` equal) the color is copied out. Otherwise, unless
/// the handle is 0xFFFF, the color becomes the spline data's compacted
/// degree-3 value (`00a526b0`, dimension 4, the two scalars, into a stack
/// buffer) at the normalized time `(time - start) / (end - start)` (x87,
/// stored as a `float`), its four floats copied in through the FPU; then
/// the color is copied out and the time (through the FPU) becomes the last
/// time. Always true.
pub fn ni_b_spline_comp_color_interpolator_update(
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
        let half_range = x87_quiet_bits(e.mem.u32(object + COMP_HALF_RANGE));
        let offset = x87_quiet_bits(e.mem.u32(object + COMP_OFFSET));
        let basis = e.mem.u32(object + BASIS_DATA);
        let start = e.mem.u32(object + START_TIME);
        let from_start = x87_sub(now, x87_m32(start));
        let span = x87_sub(x87_load32(e.mem.u32(object + END_TIME)), x87_m32(start));
        let normalized = x87_quiet_bits(x87_store32(x87_div(from_start, span)));
        let data = e.mem.u32(object + SPLINE_DATA);
        e.with_stack(0x10, |e, result| {
            e.call(
                GET_COMPACTED_VALUE_DEGREE3,
                &args![data, normalized, result, 4u32, basis, handle, offset, half_range],
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

// Translated from 00a51890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::CreateClone` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the cloning process): a new object as
/// the factory builds it, the color interpolator's members copied into it
/// (`00a51a60`), then both scalars (through the FPU); into address 0 when
/// the allocation failed, as the exe does. Answers the new object.
pub fn ni_b_spline_comp_color_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: u32,
) -> Ptr {
    let object = new_comp_color(e);
    e.call(
        COLOR_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    let offset = x87_quiet_bits(e.mem.u32(this.addr() + COMP_OFFSET));
    e.mem.set_u32(object.wrapping_add(COMP_OFFSET), offset);
    let half_range = x87_quiet_bits(e.mem.u32(this.addr() + COMP_HALF_RANGE));
    e.mem
        .set_u32(object.wrapping_add(COMP_HALF_RANGE), half_range);
    Ptr::new(object)
}

// Translated from 00a518f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The class's `CreateObject` (`cdecl`, no words; registered for
/// `"NiBSplineCompColorInterpolator"` by `00a2d8a0`; Ghidra has no
/// function here and the engine map no name): a new
/// `NiBSplineCompColorInterpolator` (no data, handle 0xFFFF, both scalars
/// the default `01099c2c`), null when the allocation fails.
pub fn fn_00a518f0(e: &mut Engine) -> Ptr {
    Ptr::new(new_comp_color(e))
}

// Translated from 00a51930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::LoadBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the color interpolator's
/// `LoadBinary` (`00a51d30`), then the two scalars read in one call to the
/// stream reader's function at its +8 (`cdecl (reader, this + 0x34, 8, &4,
/// 1)`), the answer added to the reader's +4.
pub fn ni_b_spline_comp_color_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLOR_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    transfer_scalars(e, this, stream, 0x24c, 8);
}

// Translated from 00a51970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::SaveBinary` (Xbox PDB, matched by the
/// engine map; `thiscall`, one word: the stream): the color interpolator's
/// `SaveBinary` (`00a51ca0`), then the two scalars written in one call to
/// the stream writer's function at its +0xC (`cdecl (writer, this + 0x34,
/// 8, &4, 1)`), the answer added to the writer's +4.
pub fn ni_b_spline_comp_color_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLOR_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    transfer_scalars(e, this, stream, 0x250, 0xc);
}

// Translated from 00a519b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineCompColorInterpolator::GetViewerStrings` (Xbox PDB, matched by
/// the engine map; `thiscall`, one word: the strings array): the color
/// interpolator's strings (`00a51d80`), then the class name (the first word
/// of the `NiRTTI` record) appended: when the 16-bit size (+0xA) has
/// reached the capacity (+0x8) the array grows to the step (+0xE) plus the
/// size first, then the string is stored at the old size.
pub fn ni_b_spline_comp_color_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    e.call(COLOR_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI);
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
            0x00a5_1210,
            ni_b_spline_comp_color_interpolator_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_1730,
            ni_b_spline_comp_color_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_17a0,
            ni_b_spline_comp_color_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_1890,
            ni_b_spline_comp_color_interpolator_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(0x00a5_18f0, fn_00a518f0() -> Ptr),
        entry!(
            0x00a5_1930,
            ni_b_spline_comp_color_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_1970,
            ni_b_spline_comp_color_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_19b0,
            ni_b_spline_comp_color_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a51770, ni_b_spline_comp_color_interpolator_get_rtti() -> Ptr),
        entry!(0x00a51780, ni_b_spline_comp_color_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_comp_color_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a51770, &args![]).u32(),
            NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_comp_color_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a51780, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a51780, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x3c]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
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
        e.map(COMP_COLOR_DEFAULT, 4);
        e.set_global(COMP_COLOR_DEFAULT, 0.5f32);
        e.register(OPERATOR_NEW, |e, a| word(e.mem.alloc(a[0])));
        e.register(COLOR_INTERPOLATOR_CONSTRUCTOR, |_, a| word(a[0]));
        e.register(COLOR_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(GET_COMPACTED_VALUE_DEGREE3, |e, a| {
            for i in 0..4 {
                e.mem.set_f32(a[2] + 4 * i, 2.0 + i as f32);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn register_streamables_answers_the_color_interpolator() {
        let mut e = engine();
        e.register(COLOR_INTERPOLATOR_REGISTER_STREAMABLES, |_, _| word(0x300));
        assert!(!e.call(0x00a5_1210, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_REGISTER_STREAMABLES),
            vec![vec![0x1000, 0x2000]]
        );
        e.register(COLOR_INTERPOLATOR_REGISTER_STREAMABLES, |_, _| word(1));
        assert!(e.call(0x00a5_1210, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn is_equal_compares_both_scalars() {
        let mut e = engine();
        e.register(COLOR_INTERPOLATOR_IS_EQUAL, |_, _| word(1));
        let this = e.mem.alloc(COMP_COLOR_SIZE);
        let other = e.mem.alloc(COMP_COLOR_SIZE);
        assert!(e.call(0x00a5_1730, &args![this, other]).bool());
        e.mem.set_f32(other + COMP_HALF_RANGE, 1.0);
        assert!(!e.call(0x00a5_1730, &args![this, other]).bool());
        e.mem.set_f32(other + COMP_HALF_RANGE, 0.0);
        e.mem.set_f32(other + COMP_OFFSET, f32::NAN);
        e.mem.set_f32(this + COMP_OFFSET, f32::NAN);
        assert!(!e.call(0x00a5_1730, &args![this, other]).bool());
        e.register(COLOR_INTERPOLATOR_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_1730, &args![this, this]).bool());
    }

    #[test]
    fn update_evaluates_the_compacted_color() {
        let mut e = engine();
        let this = e.mem.alloc(COMP_COLOR_SIZE);
        let color = e.mem.alloc(0x10);
        e.mem.set_f32(this + START_TIME, 1.0);
        e.mem.set_f32(this + END_TIME, 3.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x4000);
        e.mem.set_u32(this + BASIS_DATA, 0x5000);
        e.mem.set_u32(this + COLOR_HANDLE, 8);
        e.mem.set_f32(this + COMP_OFFSET, 0.25);
        e.mem.set_f32(this + COMP_HALF_RANGE, 0.75);
        assert!(e
            .call(0x00a5_17a0, &args![this, 2.0f32, 0u32, color])
            .bool());
        let call = &calls(&e, GET_COMPACTED_VALUE_DEGREE3)[0];
        assert_eq!((call[0], call[1]), (0x4000, 0.5f32.to_bits()));
        assert_eq!(
            call[3..],
            [4, 0x5000, 8, 0.25f32.to_bits(), 0.75f32.to_bits()]
        );
        for i in 0..4 {
            assert_eq!(e.mem.f32(this + COLOR_VALUE + 4 * i), 2.0 + i as f32);
            assert_eq!(e.mem.f32(color + 4 * i), 2.0 + i as f32);
        }
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        e.mem.set_f32(this + COLOR_VALUE, 7.0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_17a0, &args![this, 2.0f32, 0u32, color])
            .bool());
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(color), 7.0);
        e.mem.set_u32(this + COLOR_HANDLE, 0xffff);
        assert!(e
            .call(0x00a5_17a0, &args![this, 2.5f32, 0u32, color])
            .bool());
        assert!(calls(&e, GET_COMPACTED_VALUE_DEGREE3).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.5);
    }

    #[test]
    fn factory_and_create_clone_set_the_scalars() {
        let mut e = engine();
        let object = e.call(0x00a5_18f0, &args![]).u32();
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![object, 0, 0xffff, 0]]
        );
        assert_eq!(e.mem.u32(object), COMP_COLOR_VTABLE);
        assert_eq!(e.mem.f32(object + COMP_OFFSET), 0.5);
        assert_eq!(e.mem.f32(object + COMP_HALF_RANGE), 0.5);
        let this = e.mem.alloc(COMP_COLOR_SIZE);
        e.mem.set_f32(this + COMP_OFFSET, 1.5);
        e.mem.set_f32(this + COMP_HALF_RANGE, 2.5);
        let clone = e.call(0x00a5_1890, &args![this, 0x900u32]).u32();
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x900]]
        );
        assert_eq!(e.mem.f32(clone + COMP_OFFSET), 1.5);
        assert_eq!(e.mem.f32(clone + COMP_HALF_RANGE), 2.5);
        e.register(OPERATOR_NEW, |_, _| word(0));
        assert_eq!(e.call(0x00a5_18f0, &args![]).u32(), 0);
        e.map(0, 0x40);
        assert_eq!(e.call(0x00a5_1890, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(e.mem.f32(COMP_HALF_RANGE), 2.5);
    }

    #[test]
    fn load_and_save_binary_move_both_scalars_at_once() {
        const IO: u32 = 0x0300_0600;
        let mut e = engine();
        e.register(COLOR_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(COLOR_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(IO, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (8, 4, 1));
            word(8)
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, IO);
        e.mem.set_u32(file + 0xc, IO);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        let this = e.mem.alloc(COMP_COLOR_SIZE);
        e.call(0x00a5_1930, &args![this, stream]);
        e.call(0x00a5_1970, &args![this, stream]);
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        let io = calls(&e, IO);
        assert_eq!((io[0][1], io[1][1]), (this + 0x34, this + 0x34));
        assert_eq!(e.mem.u32(file + 4), 16);
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name() {
        let mut e = engine();
        e.map(NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI, 4);
        e.set_global(NI_B_SPLINE_COMP_COLOR_INTERPOLATOR_RTTI, 0x0109_cccc_u32);
        e.register(COLOR_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x61));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x61);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00a5_19b0, &args![0x1000u32, strings]);
        assert_eq!(
            calls(&e, COLOR_INTERPOLATOR_GET_VIEWER_STRINGS),
            vec![vec![0x1000, strings]]
        );
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_cccc]]);
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 4]]);
        assert_eq!(calls(&e, STRINGS_SET_AT)[0][..2], [strings, 1]);
    }
}
