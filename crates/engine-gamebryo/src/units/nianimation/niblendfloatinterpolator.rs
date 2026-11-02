//! `NiAnimation/niblendfloatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendFloatInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3e58;

// Translated from 00a405e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_float_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_FLOAT_INTERPOLATOR_RTTI)
}

/// `NiBlendInterpolator::NiBlendInterpolator` (`00a370b0`: `this`, the
/// manager-controlled flag, the weight threshold and the array size).
const NI_BLEND_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_70b0;
/// The `NiBlendInterpolator` default constructor (`00a36890`, `this` only).
const NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT: u32 = 0x00a3_6890;
/// `NiBlendInterpolator`'s clone-member copy (`00a37510`: `this`, the clone
/// and the cloning process).
const NI_BLEND_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a3_7510;
/// `NiBlendInterpolator::ComputeNormalizedWeights` (`00a37260`).
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// `NiBlendInterpolator::LoadBinary` (`00a37980`).
const NI_BLEND_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a3_7980;
/// The Gamebryo allocator (`00aa13e0`, cdecl, the size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiBlendFloatInterpolator` vtable (its RTTI locator names
/// `.?AVNiBlendFloatInterpolator@@`).
const NI_BLEND_FLOAT_INTERPOLATOR_VTABLE: u32 = 0x0109_7414;
/// The float the class starts with and falls back to (`-FLT_MAX`).
const FLOAT_VALUE_INVALID: u32 = 0x0109_a918;
/// The invalid time `StoreSingleValue` and `BlendValues` compare with
/// (`-FLT_MAX`).
const INVALID_TIME: u32 = 0x0109_696c;
/// The object size.
const NI_BLEND_FLOAT_INTERPOLATOR_SIZE: u32 = 0x34;
/// The float `Update` of an `NiInterpolator` (vtable +0x9C: the time, the
/// object and the address of the result).
const UPDATE_FLOAT_SLOT: u32 = 0x9c;
/// `NiInterpolator::m_fLastTime` (Xbox PDB) +0x08.
const LAST_TIME: u32 = 0x08;
/// `NiBlendInterpolator::m_uFlags` (Xbox PDB) +0x0C; bit 0 is manager
/// controlled.
const FLAGS: u32 = 0x0c;
/// `NiBlendInterpolator::m_ucArraySize` (Xbox PDB) +0x0D.
const ARRAY_SIZE: u32 = 0x0d;
/// `NiBlendInterpolator::m_ucInterpCount` (Xbox PDB) +0x0E.
const INTERP_COUNT: u32 = 0x0e;
/// `NiBlendInterpolator::m_pkInterpArray` (Xbox PDB) +0x14.
const INTERP_ARRAY: u32 = 0x14;
/// `NiBlendInterpolator::m_pkSingleInterpolator` (Xbox PDB) +0x18.
const SINGLE_INTERPOLATOR: u32 = 0x18;
/// `NiBlendInterpolator::m_fSingleTime` (Xbox PDB) +0x20.
const SINGLE_TIME: u32 = 0x20;
/// `NiBlendFloatInterpolator::m_fFloatValue` (Xbox PDB) +0x30.
const FLOAT_VALUE: u32 = 0x30;
/// `InterpArrayItem` (Xbox PDB): 0x18 bytes, `m_spInterpolator` +0,
/// `m_fNormalizedWeight` +8, `m_fUpdateTime` +0x14.
const ITEM_SIZE: u32 = 0x18;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x08;
const ITEM_UPDATE_TIME: u32 = 0x14;
/// `NiStream`'s input stream (+0x24C) and that stream's read function
/// pointer (+8, cdecl: stream, buffer, bytes, component sizes, component
/// count; answers the bytes read, added to the stream's position at +4).
const STREAM_INPUT: u32 = 0x24c;
const INPUT_READ: u32 = 0x08;
const INPUT_POSITION: u32 = 0x04;

/// The rest of the constructors: the class's vtable and the invalid value.
fn finish_float_interpolator(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, NI_BLEND_FLOAT_INTERPOLATOR_VTABLE);
    let invalid = x87_quiet_bits(e.mem.u32(FLOAT_VALUE_INVALID));
    e.mem.set_u32(this + FLOAT_VALUE, invalid);
}

// Translated from 00a40520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::NiBlendFloatInterpolator` (Xbox PDB): the
/// `NiBlendInterpolator` base with the three arguments (the threshold
/// through an `FLD`/`FSTP` pair), the class's vtable and the invalid float
/// value. Returns `this`.
pub fn ni_blend_float_interpolator_ni_blend_float_interpolator(
    e: &mut Engine,
    this: Ptr,
    manager_controlled: u32,
    weight_threshold: f32,
    array_size: u32,
) -> Ptr {
    let threshold = x87_quiet_bits(weight_threshold.to_bits());
    e.call(
        NI_BLEND_INTERPOLATOR_CONSTRUCT,
        &args![this, manager_controlled, threshold, array_size],
    );
    finish_float_interpolator(e, this.addr());
    this
}

// Translated from 00a40580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::CreateObject` (the class's stream factory;
/// Ghidra made no function of it): a new 0x34-byte object from the Gamebryo
/// allocator with the default `NiBlendInterpolator` constructor, the
/// class's vtable and the invalid value; null when the allocation fails.
pub fn ni_blend_float_interpolator_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_BLEND_FLOAT_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, &args![object]);
    finish_float_interpolator(e, object);
    Ptr::new(object)
}

// Translated from 00a405f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::CreateClone` (Xbox PDB): a new object as
/// `CreateObject` makes it (null when the allocation fails), the base's
/// members copied into it (`00a37510`), then the float value (`FLD`/`FSTP`).
/// The exe stores the value through the clone pointer even when it is null.
pub fn ni_blend_float_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = ni_blend_float_interpolator_create_object(e);
    e.call(
        NI_BLEND_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    let value = x87_quiet_bits(e.mem.u32(this.addr() + FLOAT_VALUE));
    e.mem.set_u32(clone.addr().wrapping_add(FLOAT_VALUE), value);
    clone
}

// Translated from 00a40640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::StoreSingleValue` (Xbox PDB): the single
/// interpolator's float at `time` (the single time when manager
/// controlled). At the invalid time: false, nothing written. Otherwise the
/// single interpolator's `Update` (vtable +0x9C) writes the class's value;
/// when it gives one, `*out` is that value and the answer true, else the
/// value and `*out` become the invalid float and the answer false.
pub fn ni_blend_float_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let base = this.addr();
    let mut time_bits = time.to_bits();
    if e.mem.u8(base + FLAGS) & 1 != 0 {
        time_bits = x87_quiet_bits(e.mem.u32(base + SINGLE_TIME));
    }
    let time_value = x87_load32(time_bits);
    // FUCOMPP of the invalid time with the time: equal (not unordered).
    if x87_load32(e.mem.u32(INVALID_TIME)) == time_value {
        return false;
    }
    let single = e.mem.u32(base + SINGLE_INTERPOLATOR);
    let gave = e
        .vcall(
            single,
            UPDATE_FLOAT_SLOT,
            &args![x87_store32(time_value), object, base + FLOAT_VALUE],
        )
        .bool();
    if gave {
        let value = x87_quiet_bits(e.mem.u32(base + FLOAT_VALUE));
        e.mem.set_u32(out.addr(), value);
        return true;
    }
    let invalid = x87_quiet_bits(e.mem.u32(FLOAT_VALUE_INVALID));
    e.mem.set_u32(base + FLOAT_VALUE, invalid);
    let invalid = x87_quiet_bits(e.mem.u32(FLOAT_VALUE_INVALID));
    e.mem.set_u32(out.addr(), invalid);
    false
}

// Translated from 00a406c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::BlendValues` (Xbox PDB): the weighted sum of
/// the items' floats. The value starts at 0.0 and the weight total at 1.0.
/// Each item with an interpolator and a normalized weight above 0.0 gives
/// its float at `time` (its update time when manager controlled) through
/// `Update` (vtable +0x9C); its weight times the float is added to the
/// value. At the invalid time, without a float, or with a weight equal to
/// 0.0 at the second test, the weight comes off the total instead. The
/// value is then divided by the total; with at least one float it is
/// `*out` and the answer true, else the value and `*out` are the invalid
/// float and the answer false. The float roundings follow the exe's stores.
pub fn ni_blend_float_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + FLOAT_VALUE, 0);
    let mut valid = false;
    let mut total = x87_store32(1.0);
    e.with_stack(4, |e, value| {
        let mut index = 0u8;
        while index < e.mem.u8(base + ARRAY_SIZE) {
            let offset = u32::from(index) * ITEM_SIZE;
            index = index.wrapping_add(1);
            let item = e.mem.u32(base + INTERP_ARRAY).wrapping_add(offset);
            let interpolator = e.mem.u32(item);
            if interpolator == 0 {
                continue;
            }
            // FCOM of 0.0 with the weight: on only when 0.0 is below it.
            if !(0.0 < x87_m32(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT))) {
                continue;
            }
            let mut time_bits = x87_quiet_bits(time.to_bits());
            let subtract =
                if interpolator == 0 || 0.0 == x87_m32(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT)) {
                    true
                } else {
                    if e.mem.u8(base + FLAGS) & 1 != 0 {
                        time_bits = x87_quiet_bits(e.mem.u32(item + ITEM_UPDATE_TIME));
                    }
                    x87_load32(e.mem.u32(INVALID_TIME)) == x87_load32(time_bits)
                };
            if subtract {
                let weight = e.mem.u32(item + ITEM_NORMALIZED_WEIGHT);
                total = x87_store32(x87_sub(x87_load32(total), x87_m32(weight)));
                continue;
            }
            let gave = e
                .vcall(
                    interpolator,
                    UPDATE_FLOAT_SLOT,
                    &args![x87_quiet_bits(time_bits), object, value],
                )
                .bool();
            // The item is found again through the (re-read) array pointer.
            let item = e.mem.u32(base + INTERP_ARRAY).wrapping_add(offset);
            let weight = e.mem.u32(item + ITEM_NORMALIZED_WEIGHT);
            if gave {
                valid = true;
                let product = x87_mul(x87_load32(weight), x87_m32(e.mem.u32(value.addr())));
                let sum = x87_add(product, x87_m32(e.mem.u32(base + FLOAT_VALUE)));
                e.mem.set_u32(base + FLOAT_VALUE, x87_store32(sum));
            } else {
                total = x87_store32(x87_sub(x87_load32(total), x87_m32(weight)));
            }
        }
    });
    let quotient = x87_div(x87_load32(e.mem.u32(base + FLOAT_VALUE)), x87_m32(total));
    let quotient = x87_quiet_bits(x87_store32(quotient));
    e.mem.set_u32(base + FLOAT_VALUE, quotient);
    if !valid {
        let invalid = x87_quiet_bits(e.mem.u32(FLOAT_VALUE_INVALID));
        e.mem.set_u32(out.addr(), invalid);
        let invalid = x87_quiet_bits(e.mem.u32(FLOAT_VALUE_INVALID));
        e.mem.set_u32(base + FLOAT_VALUE, invalid);
        return false;
    }
    e.mem.set_u32(out.addr(), quotient);
    true
}

// Translated from 00a40810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::Update` (Xbox PDB): with one interpolator
/// `StoreSingleValue`, with more `ComputeNormalizedWeights` then
/// `BlendValues`, with none nothing (false); `m_fLastTime` becomes `time`.
pub fn ni_blend_float_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let base = this.addr();
    let count = e.mem.u8(base + INTERP_COUNT);
    let time_bits = x87_quiet_bits(time.to_bits());
    let mut result = false;
    if count == 1 {
        result = ni_blend_float_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            out,
        );
    } else if count > 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        result = ni_blend_float_interpolator_blend_values(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            out,
        );
    }
    e.mem.set_u32(base + LAST_TIME, time_bits);
    result
}

// Translated from 00a40870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::LoadBinary` (Xbox PDB): the base's part, then
/// the float value read through the stream's input (`+0x24C`, its read
/// function at +8: 4 bytes, one component of size 4; the count it answers
/// moves the input's position at +4).
pub fn ni_blend_float_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_BLEND_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let input = e.mem.u32(stream.addr() + STREAM_INPUT);
    let read = e.mem.u32(input + INPUT_READ);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(
            read,
            &args![input, this.addr() + FLOAT_VALUE, 4u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(input + INPUT_POSITION);
    e.mem
        .set_u32(input + INPUT_POSITION, position.wrapping_add(count));
}

/// `NiInterpolator::IsFloatValueSupported` (vtable +0xA8, Xbox PDB slot
/// order).
const SLOT_IS_FLOAT_VALUE_SUPPORTED: u32 = 0xa8;
/// `NiBlendInterpolator::IsEqual` (`00a36fd0`), `SaveBinary` (`00a37ec0`)
/// and `GetViewerStrings` (`00a38080`), all `thiscall (this, other or
/// stream or strings)`.
const NI_BLEND_INTERPOLATOR_IS_EQUAL: u32 = 0x00a3_6fd0;
const NI_BLEND_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a3_7ec0;
const NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
/// `NiGetViewerString` (`00a7d330`, the class name) and its float form
/// (`00a7d550`, `(name, value)`), both `cdecl`; the name `"m_fFloatValue"`.
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const FLOAT_VALUE_NAME: u32 = 0x0109_703c;

// Translated from 00a40560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): the interpolator's `IsFloatValueSupported`
/// (vtable +0xA8); `this` is not read.
pub fn ni_blend_float_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
) -> bool {
    e.vcall(interpolator.addr(), SLOT_IS_FLOAT_VALUE_SUPPORTED, &args![])
        .bool()
}

// Translated from 00a405b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::IsEqual` (Xbox PDB): `NiBlendInterpolator`'s
/// `IsEqual`, then the float values equal (x87 compare: a NaN is never
/// equal).
pub fn ni_blend_float_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(NI_BLEND_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    x87_load32(e.mem.u32(this.addr() + FLOAT_VALUE))
        == x87_load32(e.mem.u32(other.addr() + FLOAT_VALUE))
}

// Translated from 00a408b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::SaveBinary` (Xbox PDB): `NiBlendInterpolator`'s
/// `SaveBinary`, then the float value (4 bytes; the component size in the
/// stream argument's stack word).
pub fn ni_blend_float_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(NI_BLEND_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        stream_write(e, stream, this.addr() + FLOAT_VALUE, 4, argument.addr());
    });
}

// Translated from 00a408f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendFloatInterpolator::GetViewerStrings` (Xbox PDB):
/// `NiBlendInterpolator`'s strings, then the class name and
/// `m_fFloatValue` (through the argument's stack word).
pub fn ni_blend_float_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(
        NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.mem.u32(NI_BLEND_FLOAT_INTERPOLATOR_RTTI);
        let text = e.call(VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, slot, text);
        let value = x87_quiet_bits(e.mem.u32(this.addr() + FLOAT_VALUE));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![FLOAT_VALUE_NAME, value])
            .u32();
        append_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a405e0, ni_blend_float_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a40520,
            ni_blend_float_interpolator_ni_blend_float_interpolator(Ptr, u32, f32, u32) -> Ptr
        ),
        entry!(0x00a40580, ni_blend_float_interpolator_create_object() -> Ptr),
        entry!(0x00a405f0, ni_blend_float_interpolator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a40640,
            ni_blend_float_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a406c0,
            ni_blend_float_interpolator_blend_values(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a40810, ni_blend_float_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
        entry!(
            0x00a40870,
            ni_blend_float_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_0560,
            ni_blend_float_interpolator_interpolator_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a4_05b0,
            ni_blend_float_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a4_08b0,
            ni_blend_float_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_08f0,
            ni_blend_float_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_float_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a405e0, &args![]).u32(),
            NI_BLEND_FLOAT_INTERPOLATOR_RTTI
        );
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const MINUS_FLT_MAX: u32 = 0xff7f_ffff;

    fn constants(e: &mut Engine) {
        e.map(FLOAT_VALUE_INVALID, 4);
        e.set_global(FLOAT_VALUE_INVALID, MINUS_FLT_MAX);
        e.map(INVALID_TIME, 4);
        e.set_global(INVALID_TIME, MINUS_FLT_MAX);
    }

    #[test]
    fn constructors_and_factory_set_the_vtable_and_invalid_value() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(NI_BLEND_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, |_, a| {
            a[0].into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        let r = e.call(0x00a4_0520, &args![this, 1u32, 0.5f32, 3u32]).u32();
        assert_eq!(r, this);
        assert_eq!(
            calls_to(&e, NI_BLEND_INTERPOLATOR_CONSTRUCT),
            vec![vec![this, 1, 0.5f32.to_bits(), 3]]
        );
        assert_eq!(e.mem.u32(this), NI_BLEND_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + FLOAT_VALUE), MINUS_FLT_MAX);
        let object = e.call(0x00a4_0580, &args![]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x34]]);
        assert_eq!(
            calls_to(&e, NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT),
            vec![vec![object]]
        );
        assert_eq!(e.mem.u32(object), NI_BLEND_FLOAT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(object + FLOAT_VALUE), MINUS_FLT_MAX);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_0580, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_members_and_the_value() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, |_, a| {
            a[0].into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.mem.set_f32(this + FLOAT_VALUE, 2.5);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_05f0, &args![this, 0x99u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(
            calls_to(&e, NI_BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.f32(clone + FLOAT_VALUE), 2.5);
        assert_eq!(e.mem.u32(clone), NI_BLEND_FLOAT_INTERPOLATOR_VTABLE);
    }

    /// An interpolator whose `Update` (+0x9C) writes `value` when `gives`.
    fn interpolator(e: &mut Engine, update: u32) -> u32 {
        let vtable = e.mem.alloc(0xa0);
        e.mem.set_u32(vtable + UPDATE_FLOAT_SLOT, update);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn store_single_value_reads_the_single_interpolator() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(0x0500_0000, |e, a| {
            e.mem.set_f32(a[3], f32::from_bits(a[1]) * 2.0);
            true.into_ret()
        });
        e.register(0x0500_0010, |_, _| false.into_ret());
        let this = e.mem.alloc(0x34);
        let single = interpolator(&mut e, 0x0500_0000);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, single);
        let out = e.mem.alloc(4);
        assert!(e
            .call(0x00a4_0640, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.f32(out), 3.0);
        assert_eq!(e.mem.f32(this + FLOAT_VALUE), 3.0);
        // Manager controlled: the single time is used.
        e.mem.set_u8(this + FLAGS, 1);
        e.mem.set_f32(this + SINGLE_TIME, 4.0);
        assert!(e
            .call(0x00a4_0640, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.f32(out), 8.0);
        // The invalid time: nothing written.
        e.mem.set_u32(this + SINGLE_TIME, MINUS_FLT_MAX);
        assert!(!e
            .call(0x00a4_0640, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.f32(out), 8.0);
        // No value: both become invalid.
        e.mem.set_u8(this + FLAGS, 0);
        let refusing = interpolator(&mut e, 0x0500_0010);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, refusing);
        assert!(!e
            .call(0x00a4_0640, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u32(out), MINUS_FLT_MAX);
        assert_eq!(e.mem.u32(this + FLOAT_VALUE), MINUS_FLT_MAX);
    }

    /// An object with `items` (interpolator, normalized weight, update time).
    fn blender(e: &mut Engine, items: &[(u32, f32, f32)]) -> u32 {
        let this = e.mem.alloc(0x34);
        let array = e.mem.alloc(ITEM_SIZE * items.len() as u32 + 4);
        for (i, (interp, weight, time)) in items.iter().enumerate() {
            let item = array + ITEM_SIZE * i as u32;
            e.mem.set_u32(item, *interp);
            e.mem.set_f32(item + ITEM_NORMALIZED_WEIGHT, *weight);
            e.mem.set_f32(item + ITEM_UPDATE_TIME, *time);
        }
        e.mem.set_u32(this + INTERP_ARRAY, array);
        e.mem.set_u8(this + ARRAY_SIZE, items.len() as u8);
        this
    }

    #[test]
    fn blend_values_sums_the_weighted_floats() {
        let mut e = Engine::new();
        constants(&mut e);
        // Writes the time it gets plus 10.
        e.register(0x0500_0000, |e, a| {
            e.mem.set_f32(a[3], f32::from_bits(a[1]) + 10.0);
            true.into_ret()
        });
        e.register(0x0500_0010, |_, _| false.into_ret());
        let giving = interpolator(&mut e, 0x0500_0000);
        let refusing = interpolator(&mut e, 0x0500_0010);
        let this = blender(
            &mut e,
            &[
                (giving, 0.5, 0.0),
                (0, 0.25, 0.0),
                (giving, 0.0, 0.0),
                (refusing, 0.25, 0.0),
                (giving, 0.25, 0.0),
            ],
        );
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a4_06c0, &args![this, 2.0f32, 0x7u32, out])
            .bool());
        // (0.5 * 12 + 0.25 * 12) / (1 - 0.25) = 12.
        assert_eq!(e.mem.f32(out), 12.0);
        assert_eq!(e.mem.f32(this + FLOAT_VALUE), 12.0);
        assert_eq!(calls_to(&e, 0x0500_0010).len(), 1);
        assert_eq!(calls_to(&e, 0x0500_0000).len(), 2);
        // Manager controlled: each item's update time; one at the invalid
        // time counts off its weight.
        let this = blender(
            &mut e,
            &[
                (giving, 0.5, 1.0),
                (giving, 0.5, f32::from_bits(MINUS_FLT_MAX)),
            ],
        );
        e.mem.set_u8(this + FLAGS, 1);
        assert!(e
            .call(0x00a4_06c0, &args![this, 2.0f32, 0x7u32, out])
            .bool());
        // 0.5 * 11 / 0.5.
        assert_eq!(e.mem.f32(out), 11.0);
    }

    #[test]
    fn blend_values_without_any_float_is_invalid() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(0x0500_0010, |_, _| false.into_ret());
        let refusing = interpolator(&mut e, 0x0500_0010);
        let out = e.mem.alloc(4);
        let this = blender(&mut e, &[(refusing, 1.0, 0.0)]);
        assert!(!e
            .call(0x00a4_06c0, &args![this, 2.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u32(out), MINUS_FLT_MAX);
        assert_eq!(e.mem.u32(this + FLOAT_VALUE), MINUS_FLT_MAX);
        let this = blender(&mut e, &[]);
        assert!(!e
            .call(0x00a4_06c0, &args![this, 2.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u32(this + FLOAT_VALUE), MINUS_FLT_MAX);
    }

    #[test]
    fn update_picks_single_or_blend_and_stores_the_time() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        e.register(0x0500_0000, |e, a| {
            e.mem.set_f32(a[3], 5.0);
            true.into_ret()
        });
        let giving = interpolator(&mut e, 0x0500_0000);
        let out = e.mem.alloc(4);
        let this = blender(&mut e, &[(giving, 1.0, 0.0)]);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, giving);
        e.call_log = Some(vec![]);
        // No interpolators: false, time stored.
        assert!(!e.call(0x00a4_0810, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 3.0);
        assert!(calls_to(&e, 0x0500_0000).is_empty());
        e.mem.set_u8(this + INTERP_COUNT, 1);
        assert!(e.call(0x00a4_0810, &args![this, 4.0f32, 0u32, out]).bool());
        assert!(calls_to(&e, COMPUTE_NORMALIZED_WEIGHTS).is_empty());
        assert_eq!(e.mem.f32(this + LAST_TIME), 4.0);
        e.mem.set_u8(this + INTERP_COUNT, 2);
        assert!(e.call(0x00a4_0810, &args![this, 6.0f32, 0u32, out]).bool());
        assert_eq!(calls_to(&e, COMPUTE_NORMALIZED_WEIGHTS), vec![vec![this]]);
        assert_eq!(e.mem.f32(out), 5.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 6.0);
    }

    #[test]
    fn load_binary_reads_the_float_value() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x0500_0020, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_f32(a[1], 7.25);
            a[2].into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + INPUT_READ, 0x0500_0020);
        e.mem.set_u32(input + INPUT_POSITION, 0x10);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a4_0870, &args![this, stream]);
        assert_eq!(
            calls_to(&e, NI_BLEND_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        let read = calls_to(&e, 0x0500_0020);
        assert_eq!(read[0][..3], [input, this + FLOAT_VALUE, 4]);
        assert_eq!(read[0][4], 1);
        assert_eq!(e.mem.f32(this + FLOAT_VALUE), 7.25);
        assert_eq!(e.mem.u32(input + INPUT_POSITION), 0x14);
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{
        calls, stream, viewer_strings,
    };

    const VTABLE: u32 = 0x7400_0000;
    const SUPPORTED: u32 = 0x7500_0000;

    #[test]
    fn interpolator_is_correct_type_asks_for_float_support() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x40];
        slots[(SLOT_IS_FLOAT_VALUE_SUPPORTED / 4) as usize] = SUPPORTED;
        e.put_vtable(VTABLE, &slots);
        e.register(SUPPORTED, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let interp = e.mem.alloc(8);
        e.mem.set_u32(interp, VTABLE);
        e.mem.set_u32(interp + 4, 1);
        assert!(e.call(0x00a4_0560, &args![0u32, interp]).bool());
        e.mem.set_u32(interp + 4, 0);
        assert!(!e.call(0x00a4_0560, &args![0u32, interp]).bool());
    }

    #[test]
    fn is_equal_compares_the_float_value_after_the_base() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_IS_EQUAL, |_, _| 1u32.into_ret());
        let (a, b) = (e.mem.alloc(0x34), e.mem.alloc(0x34));
        e.mem.set_f32(a + FLOAT_VALUE, 2.5);
        e.mem.set_f32(b + FLOAT_VALUE, 2.5);
        assert!(e.call(0x00a4_05b0, &args![a, b]).bool());
        e.mem.set_f32(b + FLOAT_VALUE, f32::NAN);
        e.mem.set_f32(a + FLOAT_VALUE, f32::NAN);
        assert!(!e.call(0x00a4_05b0, &args![a, b]).bool());
        e.register(NI_BLEND_INTERPOLATOR_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a4_05b0, &args![a, a]).bool());
    }

    #[test]
    fn save_binary_writes_the_float_value() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        let (s, written) = stream(&mut e, 0, &[]);
        let this = e.mem.alloc(0x34);
        e.mem.set_f32(this + FLOAT_VALUE, -1.5);
        e.call_log = Some(vec![]);
        e.call(0x00a4_08b0, &args![this, s]);
        assert_eq!(*written.borrow(), (-1.5f32).to_le_bytes().to_vec());
        assert_eq!(
            calls(&e, NI_BLEND_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, s.addr()]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_value() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        let (strings, stored) = viewer_strings(&mut e);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(NI_BLEND_FLOAT_INTERPOLATOR_RTTI, 0x0109_7000);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + FLOAT_VALUE, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00a4_08f0, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_7000, FLOAT_VALUE_NAME]);
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT),
            vec![vec![FLOAT_VALUE_NAME, 0x7fc0_0001]]
        );
    }
}
