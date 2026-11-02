//! `NiAnimation/niblendboolinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendBoolInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_BOOL_INTERPOLATOR_RTTI: u32 = 0x011f_407c;

// Translated from 00a56620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_bool_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_BOOL_INTERPOLATOR_RTTI)
}

// Translated from 00a56630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a37170`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x34 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_blend_bool_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_7170;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x34u32]);
    }
    this
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
/// The `NiBlendBoolInterpolator` vtable (its RTTI locator names
/// `.?AVNiBlendBoolInterpolator@@`).
const NI_BLEND_BOOL_INTERPOLATOR_VTABLE: u32 = 0x0109_a67c;
/// The invalid bool value (the byte 2).
const BOOL_VALUE_INVALID: u32 = 0x0109_a914;
/// The invalid time `StoreSingleValue` and `BlendValues` compare with
/// (`-FLT_MAX`).
const INVALID_TIME: u32 = 0x0109_696c;
/// `0.5`: the blended weight from which the result is true.
const HALF: u32 = 0x0101_6248;
/// The object size.
const NI_BLEND_BOOL_INTERPOLATOR_SIZE: u32 = 0x34;
/// The bool `Update` of an `NiInterpolator` (vtable +0xA0: the time, the
/// object and the address of the result byte).
const UPDATE_BOOL_SLOT: u32 = 0xa0;
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
/// `NiBlendBoolInterpolator::m_bBoolValue` (Xbox PDB) +0x30, a byte.
const BOOL_VALUE: u32 = 0x30;
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
fn finish_bool_interpolator(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, NI_BLEND_BOOL_INTERPOLATOR_VTABLE);
    let invalid = e.mem.u8(BOOL_VALUE_INVALID);
    e.mem.set_u8(this + BOOL_VALUE, invalid);
}

// Translated from 00a56560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::NiBlendBoolInterpolator` (Xbox PDB): the
/// `NiBlendInterpolator` base with the three arguments (the threshold
/// through an `FLD`/`FSTP` pair), the class's vtable and the invalid bool
/// value. Returns `this`.
pub fn ni_blend_bool_interpolator_ni_blend_bool_interpolator(
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
    finish_bool_interpolator(e, this.addr());
    this
}

// Translated from 00a565c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::CreateObject` (the class's stream factory;
/// Ghidra made no function of it): a new 0x34-byte object from the Gamebryo
/// allocator with the default `NiBlendInterpolator` constructor, the
/// class's vtable and the invalid value; null when the allocation fails.
pub fn ni_blend_bool_interpolator_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_BLEND_BOOL_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, &args![object]);
    finish_bool_interpolator(e, object);
    Ptr::new(object)
}

// Translated from 00a56650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::CreateClone` (Xbox PDB): a new object as
/// `CreateObject` makes it (null when the allocation fails), the base's
/// members copied into it (`00a37510`), then the bool value byte. The exe
/// stores the byte through the clone pointer even when it is null.
pub fn ni_blend_bool_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = ni_blend_bool_interpolator_create_object(e);
    e.call(
        NI_BLEND_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    let value = e.mem.u8(this.addr() + BOOL_VALUE);
    e.mem.set_u8(clone.addr().wrapping_add(BOOL_VALUE), value);
    clone
}

// Translated from 00a566a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::StoreSingleValue` (Xbox PDB): the single
/// interpolator's bool at `time` (the single time when manager controlled).
/// At the invalid time: false, nothing written. Otherwise the single
/// interpolator's `Update` (vtable +0xA0) writes `*out`; when it gives one
/// the class's value becomes `*out` and the answer is true, else `*out`
/// becomes 0, the value the invalid bool and the answer false.
pub fn ni_blend_bool_interpolator_store_single_value(
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
            UPDATE_BOOL_SLOT,
            &args![x87_store32(time_value), object, out],
        )
        .bool();
    if gave {
        let value = e.mem.u8(out.addr());
        e.mem.set_u8(base + BOOL_VALUE, value);
        return true;
    }
    e.mem.set_u8(out.addr(), 0);
    let invalid = e.mem.u8(BOOL_VALUE_INVALID);
    e.mem.set_u8(base + BOOL_VALUE, invalid);
    false
}

// Translated from 00a56710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::BlendValues` (Xbox PDB): the weighted vote of
/// the items' bools. The sum starts at 0.0 and the weight total at 1.0.
/// Each item with an interpolator and a normalized weight above 0.0 gives
/// its bool at `time` (its update time when manager controlled) through
/// `Update` (vtable +0xA0, into the stack slot of the `object` argument);
/// its weight times the bool's byte (`FIMUL`) is added to the sum. At the
/// invalid time, without a bool, or with a weight equal to 0.0 at the
/// second test, the weight comes off the total instead. Without any bool:
/// `*out` 0, the value the invalid bool, false. Otherwise the sum divided by
/// the total is compared with 0.5: not below it (or unordered) gives true,
/// below it false, as both the value and `*out`; the answer is true.
pub fn ni_blend_bool_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let base = this.addr();
    let mut sum = 0u32;
    let mut valid = false;
    let mut total = x87_store32(1.0);
    e.with_stack(4, |e, value| {
        // The exe passes the address of its `object` argument's stack slot.
        e.mem.set_u32(value.addr(), object.addr());
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
                    UPDATE_BOOL_SLOT,
                    &args![x87_quiet_bits(time_bits), object, value],
                )
                .bool();
            // The item is found again through the (re-read) array pointer.
            let item = e.mem.u32(base + INTERP_ARRAY).wrapping_add(offset);
            let weight = e.mem.u32(item + ITEM_NORMALIZED_WEIGHT);
            if gave {
                valid = true;
                let flag = f64::from(e.mem.u8(value.addr()));
                let product = x87_mul(x87_load32(weight), flag);
                sum = x87_store32(x87_add(product, x87_m32(sum)));
            } else {
                total = x87_store32(x87_sub(x87_load32(total), x87_m32(weight)));
            }
        }
    });
    if !valid {
        e.mem.set_u8(out.addr(), 0);
        let invalid = e.mem.u8(BOOL_VALUE_INVALID);
        e.mem.set_u8(base + BOOL_VALUE, invalid);
        return false;
    }
    let ratio = x87_store32(x87_div(x87_load32(sum), x87_m32(total)));
    let half = x87_m32(e.mem.u32(HALF));
    // FCOMP with 0.5, C0 and C2 tested: below (ordered) gives false.
    let result = !(x87_load32(ratio) < half);
    e.mem.set_u8(base + BOOL_VALUE, u8::from(result));
    e.mem.set_u8(out.addr(), u8::from(result));
    true
}

// Translated from 00a56880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::Update` (Xbox PDB): with one interpolator
/// `StoreSingleValue`, with more `ComputeNormalizedWeights` then
/// `BlendValues`, with none nothing (false); `m_fLastTime` becomes `time`.
pub fn ni_blend_bool_interpolator_update(
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
        result = ni_blend_bool_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            out,
        );
    } else if count > 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        result = ni_blend_bool_interpolator_blend_values(
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

// Translated from 00a568e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::LoadBinary` (Xbox PDB): the base's part, then
/// the bool value byte read through the stream's input (`+0x24C`, its read
/// function at +8: 1 byte, one component of size 1; the count it answers
/// moves the input's position at +4).
pub fn ni_blend_bool_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_BLEND_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let input = e.mem.u32(stream.addr() + STREAM_INPUT);
    let read = e.mem.u32(input + INPUT_READ);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 1);
        e.call(
            read,
            &args![input, this.addr() + BOOL_VALUE, 1u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(input + INPUT_POSITION);
    e.mem
        .set_u32(input + INPUT_POSITION, position.wrapping_add(count));
}

/// `NiInterpolator::IsBoolValueSupported` (vtable +0xA4, Xbox PDB slot
/// order).
const SLOT_IS_BOOL_VALUE_SUPPORTED: u32 = 0xa4;
/// `NiBlendInterpolator::IsEqual` (`00a36fd0`), `SaveBinary` (`00a37ec0`)
/// and `GetViewerStrings` (`00a38080`).
const NI_BLEND_INTERPOLATOR_IS_EQUAL: u32 = 0x00a3_6fd0;
const NI_BLEND_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a3_7ec0;
const NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
/// `NiGetViewerString` (`00a7d330`, the class name) and its byte form
/// (`00a7d410`, `(name, value)`), both `cdecl`; the name `"m_bBoolValue"`.
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_BYTE: u32 = 0x00a7_d410;
const BOOL_VALUE_NAME: u32 = 0x0109_a1bc;

// Translated from 00a565a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): the interpolator's `IsBoolValueSupported`
/// (vtable +0xA4); `this` is not read.
pub fn ni_blend_bool_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
) -> bool {
    e.vcall(interpolator.addr(), SLOT_IS_BOOL_VALUE_SUPPORTED, &args![])
        .bool()
}

// Translated from 00a565f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::IsEqual` (Xbox PDB): `NiBlendInterpolator`'s
/// `IsEqual`, then the bool value bytes equal.
pub fn ni_blend_bool_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(NI_BLEND_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    e.mem.u8(this.addr() + BOOL_VALUE) == e.mem.u8(other.addr() + BOOL_VALUE)
}

// Translated from 00a56920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::SaveBinary` (Xbox PDB): `NiBlendInterpolator`'s
/// `SaveBinary`, then the bool value (1 byte; the component size in the
/// stream argument's stack word).
pub fn ni_blend_bool_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    use crate::units::nianimation::niblendinterpolator::stream_write;
    e.call(NI_BLEND_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, argument| {
        stream_write(e, stream, this.addr() + BOOL_VALUE, 1, argument.addr());
    });
}

// Translated from 00a56960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendBoolInterpolator::GetViewerStrings` (Xbox PDB):
/// `NiBlendInterpolator`'s strings, then the class name and `m_bBoolValue`
/// (the value byte, zero-extended; both through the argument's stack word).
pub fn ni_blend_bool_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(
        NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.mem.u32(NI_BLEND_BOOL_INTERPOLATOR_RTTI);
        let text = e.call(VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, slot, text);
        let value = u32::from(e.mem.u8(this.addr() + BOOL_VALUE));
        let text = e
            .call(VIEWER_STRING_BYTE, &args![BOOL_VALUE_NAME, value])
            .u32();
        append_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a56620, ni_blend_bool_interpolator_get_rtti() -> Ptr),
        entry!(0x00a56630, ni_blend_bool_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a56560,
            ni_blend_bool_interpolator_ni_blend_bool_interpolator(Ptr, u32, f32, u32) -> Ptr
        ),
        entry!(0x00a565c0, ni_blend_bool_interpolator_create_object() -> Ptr),
        entry!(0x00a56650, ni_blend_bool_interpolator_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a566a0,
            ni_blend_bool_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a56710,
            ni_blend_bool_interpolator_blend_values(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a56880, ni_blend_bool_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
        entry!(0x00a568e0, ni_blend_bool_interpolator_load_binary(Ptr, Ptr)),
        entry!(
            0x00a5_65a0,
            ni_blend_bool_interpolator_interpolator_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_65f0,
            ni_blend_bool_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_6920,
            ni_blend_bool_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_6960,
            ni_blend_bool_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_bool_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a56620, &args![]).u32(),
            NI_BLEND_BOOL_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_blend_bool_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_7170, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a56630, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a56630, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x34]]);
        assert_eq!(calls_to(&e, 0x00a3_7170), vec![vec![this]; 2]);
    }

    fn calls_to_bool(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const MINUS_FLT_MAX: u32 = 0xff7f_ffff;

    fn bool_constants(e: &mut Engine) {
        e.map(BOOL_VALUE_INVALID, 4);
        e.set_global(BOOL_VALUE_INVALID, 2u32);
        e.map(INVALID_TIME, 4);
        e.set_global(INVALID_TIME, MINUS_FLT_MAX);
        e.map(HALF, 4);
        e.set_global(HALF, 0.5f32);
    }

    #[test]
    fn bool_constructors_and_factory_set_the_vtable_and_invalid_value() {
        let mut e = Engine::new();
        bool_constants(&mut e);
        e.register(NI_BLEND_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NI_BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, |_, a| {
            a[0].into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a5_6560, &args![this, 0u32, 0.25f32, 2u32]).u32(),
            this
        );
        assert_eq!(
            calls_to_bool(&e, NI_BLEND_INTERPOLATOR_CONSTRUCT),
            vec![vec![this, 0, 0.25f32.to_bits(), 2]]
        );
        assert_eq!(e.mem.u32(this), NI_BLEND_BOOL_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 2);
        let object = e.call(0x00a5_65c0, &args![]).u32();
        assert_eq!(calls_to_bool(&e, NI_ALLOCATE), vec![vec![0x34]]);
        assert_eq!(e.mem.u32(object), NI_BLEND_BOOL_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u8(object + BOOL_VALUE), 2);
        e.register(NI_BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.mem.set_u8(this + BOOL_VALUE, 1);
        let clone = e.call(0x00a5_6650, &args![this, 0x99u32]).u32();
        assert_eq!(
            calls_to_bool(&e, NI_BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x99]]
        );
        assert_eq!(e.mem.u8(clone + BOOL_VALUE), 1);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a5_65c0, &args![]).u32(), 0);
    }

    fn bool_interpolator(e: &mut Engine, update: u32) -> u32 {
        let vtable = e.mem.alloc(0xa4);
        e.mem.set_u32(vtable + UPDATE_BOOL_SLOT, update);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    fn bool_blender(e: &mut Engine, items: &[(u32, f32, f32)]) -> u32 {
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
    fn bool_store_single_value_reads_the_single_interpolator() {
        let mut e = Engine::new();
        bool_constants(&mut e);
        e.register(0x0500_0000, |e, a| {
            e.mem.set_u8(a[3], 1);
            true.into_ret()
        });
        e.register(0x0500_0010, |_, _| false.into_ret());
        let this = e.mem.alloc(0x34);
        let single = bool_interpolator(&mut e, 0x0500_0000);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, single);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a5_66a0, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 1);
        assert_eq!(
            calls_to_bool(&e, 0x0500_0000),
            vec![vec![single, 1.5f32.to_bits(), 7, out]]
        );
        e.mem.set_u8(this + FLAGS, 1);
        e.mem.set_u32(this + SINGLE_TIME, MINUS_FLT_MAX);
        e.mem.set_u8(out, 9);
        assert!(!e
            .call(0x00a5_66a0, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 9);
        e.mem.set_u8(this + FLAGS, 0);
        let refusing = bool_interpolator(&mut e, 0x0500_0010);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, refusing);
        assert!(!e
            .call(0x00a5_66a0, &args![this, 1.5f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 2);
    }

    #[test]
    fn bool_blend_values_votes_by_weight() {
        let mut e = Engine::new();
        bool_constants(&mut e);
        e.register(0x0500_0000, |e, a| {
            e.mem.set_u8(a[3], 1);
            true.into_ret()
        });
        e.register(0x0500_0008, |e, a| {
            e.mem.set_u8(a[3], 0);
            true.into_ret()
        });
        e.register(0x0500_0010, |_, _| false.into_ret());
        let yes = bool_interpolator(&mut e, 0x0500_0000);
        let no = bool_interpolator(&mut e, 0x0500_0008);
        let refusing = bool_interpolator(&mut e, 0x0500_0010);
        let out = e.mem.alloc(4);
        // 0.25 yes, 0.5 no, 0.25 refused: 0.25 / 0.75 is below 0.5.
        let this = bool_blender(
            &mut e,
            &[(yes, 0.25, 0.0), (no, 0.5, 0.0), (refusing, 0.25, 0.0)],
        );
        assert!(e
            .call(0x00a5_6710, &args![this, 1.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 0);
        // Exactly 0.5 is true.
        let this = bool_blender(&mut e, &[(yes, 0.5, 0.0), (no, 0.5, 0.0)]);
        assert!(e
            .call(0x00a5_6710, &args![this, 1.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 1);
        // Nothing valid.
        let this = bool_blender(&mut e, &[(refusing, 1.0, 0.0), (0, 1.0, 0.0)]);
        e.mem.set_u8(out, 7);
        assert!(!e
            .call(0x00a5_6710, &args![this, 1.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 2);
        // Manager controlled with the invalid update time: weight off.
        let this = bool_blender(
            &mut e,
            &[(yes, 0.5, 2.0), (no, 0.5, f32::from_bits(MINUS_FLT_MAX))],
        );
        e.mem.set_u8(this + FLAGS, 1);
        assert!(e
            .call(0x00a5_6710, &args![this, 1.0f32, 0x7u32, out])
            .bool());
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn bool_update_picks_single_or_blend_and_stores_the_time() {
        let mut e = Engine::new();
        bool_constants(&mut e);
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        e.register(0x0500_0000, |e, a| {
            e.mem.set_u8(a[3], 1);
            true.into_ret()
        });
        let yes = bool_interpolator(&mut e, 0x0500_0000);
        let out = e.mem.alloc(4);
        let this = bool_blender(&mut e, &[(yes, 1.0, 0.0)]);
        e.mem.set_u32(this + SINGLE_INTERPOLATOR, yes);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_6880, &args![this, 3.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 3.0);
        e.mem.set_u8(this + INTERP_COUNT, 1);
        assert!(e.call(0x00a5_6880, &args![this, 4.0f32, 0u32, out]).bool());
        assert!(calls_to_bool(&e, COMPUTE_NORMALIZED_WEIGHTS).is_empty());
        e.mem.set_u8(this + INTERP_COUNT, 3);
        assert!(e.call(0x00a5_6880, &args![this, 6.0f32, 0u32, out]).bool());
        assert_eq!(
            calls_to_bool(&e, COMPUTE_NORMALIZED_WEIGHTS),
            vec![vec![this]]
        );
        assert_eq!(e.mem.f32(this + LAST_TIME), 6.0);
    }

    #[test]
    fn bool_load_binary_reads_one_byte() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x0500_0020, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 1);
            e.mem.set_u8(a[1], 1);
            a[2].into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + INPUT_READ, 0x0500_0020);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a5_68e0, &args![this, stream]);
        let read = calls_to_bool(&e, 0x0500_0020);
        assert_eq!(read[0][..3], [input, this + BOOL_VALUE, 1]);
        assert_eq!(read[0][4], 1);
        assert_eq!(e.mem.u8(this + BOOL_VALUE), 1);
        assert_eq!(e.mem.u32(input + INPUT_POSITION), 1);
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
    fn interpolator_is_correct_type_asks_for_bool_support() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x40];
        slots[(SLOT_IS_BOOL_VALUE_SUPPORTED / 4) as usize] = SUPPORTED;
        e.put_vtable(VTABLE, &slots);
        e.register(SUPPORTED, |_, _| 0u32.into_ret());
        let interp = e.mem.alloc(8);
        e.mem.set_u32(interp, VTABLE);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a5_65a0, &args![0u32, interp]).bool());
        assert_eq!(calls(&e, SUPPORTED), vec![vec![interp]]);
    }

    #[test]
    fn is_equal_compares_the_value_bytes() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_IS_EQUAL, |_, _| 1u32.into_ret());
        let (a, b) = (e.mem.alloc(0x34), e.mem.alloc(0x34));
        e.mem.set_u8(a + BOOL_VALUE, 2);
        e.mem.set_u8(b + BOOL_VALUE, 2);
        assert!(e.call(0x00a5_65f0, &args![a, b]).bool());
        e.mem.set_u8(b + BOOL_VALUE, 1);
        assert!(!e.call(0x00a5_65f0, &args![a, b]).bool());
        e.register(NI_BLEND_INTERPOLATOR_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a5_65f0, &args![a, a]).bool());
    }

    #[test]
    fn save_binary_writes_the_value_byte() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        let (s, written) = stream(&mut e, 0, &[]);
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + BOOL_VALUE, 1);
        e.call(0x00a5_6920, &args![this, s]);
        assert_eq!(*written.borrow(), vec![1]);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_value() {
        let mut e = Engine::new();
        e.register(NI_BLEND_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        let (strings, stored) = viewer_strings(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_BLEND_BOOL_INTERPOLATOR_RTTI, 0x0109_a000);
        let this = e.mem.alloc(0x34);
        e.mem.set_u8(this + BOOL_VALUE, 0xff);
        e.call_log = Some(vec![]);
        e.call(0x00a5_6960, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_a000, BOOL_VALUE_NAME]);
        assert_eq!(
            calls(&e, VIEWER_STRING_BYTE),
            vec![vec![BOOL_VALUE_NAME, 0xff]]
        );
    }
}
