//! `NiAnimation/bsblendtreadtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSBlendTreadTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ed0;

// Translated from 00a42fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_blend_tread_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI)
}

/// `00a37ec0`: the jump target of `00a40a10`.
const JUMP_TARGET_00A40A10: u32 = 0x00a3_7ec0;

/// `00a37980`: the jump target of `00a42fc0`.
const JUMP_TARGET_00A42FC0: u32 = 0x00a3_7980;

/// `00a36fd0`: the jump target of `00a42fd0`.
const JUMP_TARGET_00A42FD0: u32 = 0x00a3_6fd0;

/// `00a95ca0`: the jump target of `00a43330`.
const JUMP_TARGET_00A43330: u32 = 0x00a9_5ca0;

// Translated from 00a40a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a37ec0` (engine map: `NiBlendInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A40A10, &args![this, stream]);
}

// Translated from 00a42fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a37980` (engine map: `NiBlendInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A42FC0, &args![this, stream]);
}

// Translated from 00a42fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a36fd0` (engine map: `NiBlendInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSBlendTreadTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_blend_tread_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A42FD0, &args![this, other]).bool()
}

// Translated from 00a43330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::LinkObject` (Xbox PDB): a 5-byte jump to `00a95ca0` (engine map: `BSGrabIKNote::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43330, &args![this, stream]);
}

/// `BSBlendTreadTransfInterpolator`'s vtable (`01097904`).
const BLEND_TREAD_VTABLE: u32 = 0x0109_7904;
/// `NiBlendInterpolator::~NiBlendInterpolator` (`00a37170`, `thiscall`) and
/// `ComputeNormalizedWeights` (`00a37260`, `thiscall`).
const BLEND_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_7170;
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// `BSBlendTreadTransfInterpolator::StoreSingleValue` (`00a43040`) and
/// `BlendValues` (`00a43160`), both `thiscall (this, time, target,
/// &value)` answering a `bool`.
const STORE_SINGLE_VALUE: u32 = 0x00a4_3040;
const BLEND_VALUES: u32 = 0x00a4_3160;
/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` +8,
/// `m_ucInterpCount` +0xE.
const BLEND_LAST_TIME: u32 = 0x8;
const BLEND_INTERP_COUNT: u32 = 0xe;

// Translated from 00a42f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::~BSBlendTreadTransfInterpolator` (Xbox
/// PDB): the class's vtable, then the `NiBlendInterpolator` destructor (a
/// tail jump).
pub fn bs_blend_tread_transf_interpolator_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BLEND_TREAD_VTABLE);
    e.call(BLEND_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a432d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::Update` (Xbox PDB; `thiscall (time,
/// target, &value)`, `RET 0xC`): with one interpolator its value is stored
/// (`StoreSingleValue`); with more, the weights are normalized and the
/// values blended (`BlendValues`); with none nothing happens and false is
/// answered. The last time takes `time` (through the FPU) in every case;
/// the answer is the called function's.
pub fn bs_blend_tread_transf_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let count = e.mem.u8(object + BLEND_INTERP_COUNT);
    let time_bits = x87_quiet_bits(time.to_bits());
    let result = if count == 1 {
        e.call(STORE_SINGLE_VALUE, &args![this, time_bits, target, value])
            .bool()
    } else if count != 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        e.call(BLEND_VALUES, &args![this, time_bits, target, value])
            .bool()
    } else {
        false
    };
    e.mem.set_u32(object + BLEND_LAST_TIME, time_bits);
    result
}

/// `NiInterpolator::IsTransformValueSupported` (vtable +0xB8) and the
/// transform `Update` (+0x8C, `(time, target, NiQuatTransform*)`).
const SLOT_IS_TRANSFORM_VALUE_SUPPORTED: u32 = 0xb8;
const SLOT_UPDATE_TRANSFORM: u32 = 0x8c;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiBlendInterpolator` default constructor (`00a36890`, `this` only)
/// and its member copy (`00a37510`, `thiscall (this, clone, cloning)`).
const BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT: u32 = 0x00a3_6890;
const BLEND_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a3_7510;
/// The class's size.
const BLEND_TREAD_SIZE: u32 = 0x34;
/// `BSBlendTreadTransfInterpolator::fCurrLerp` (Xbox PDB) +0x30.
const CURR_LERP: u32 = 0x30;
/// `BSTreadTransfInterpolator::fDiffLerp` (Xbox PDB) +0x18 of the
/// interpolators blended.
const INTERPOLATOR_DIFF_LERP: u32 = 0x18;
/// `NiBlendInterpolator` fields (Xbox PDB): `m_uFlags` +0xC (bit 0 manager
/// controlled), `m_ucArraySize` +0xD, `m_pkInterpArray` +0x14 (items of
/// 0x18: interpolator +0, normalized weight +8, update time +0x14),
/// `m_pkSingleInterpolator` +0x18, `m_fSingleTime` +0x20.
const BLEND_FLAGS: u32 = 0xc;
const BLEND_ARRAY_SIZE: u32 = 0xd;
const BLEND_INTERP_ARRAY: u32 = 0x14;
const BLEND_SINGLE_INTERPOLATOR: u32 = 0x18;
const BLEND_SINGLE_TIME: u32 = 0x20;
const ITEM_SIZE: u32 = 0x18;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_UPDATE_TIME: u32 = 0x14;
/// `NiInterpolator::INVALID_FLOAT` (`0109696c`), the float marking an
/// invalid transform part (`01015f5c`), `-FLT_MAX` as a `double`
/// (`010241b0`) and the `double` `1.0` (`01012070`).
const INVALID_FLOAT: u32 = 0x0109_696c;
const INVALID_PART: u32 = 0x0101_5f5c;
const LOWEST: u32 = 0x0102_41b0;
const ONE: u32 = 0x0101_2070;
/// A fresh `NiQuatTransform`: translation (3 words, `011a8400`), rotation
/// (4 words, `011f3704`), scale `float` (`01096b8c`).
const FRESH_TRANSLATE: u32 = 0x011a_8400;
const FRESH_ROTATE: u32 = 0x011f_3704;
const FRESH_SCALE: u32 = 0x0109_6b8c;
/// `NiQuaternion::Normalize` (`00a38560`, `thiscall`, no stack words).
const QUATERNION_NORMALIZE: u32 = 0x00a3_8560;

// Translated from 00a39100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::InterpolatorIsCorrectType` (Xbox PDB;
/// `thiscall`, one stack word): the interpolator's
/// `IsTransformValueSupported` (vtable +0xB8); `this` is not read.
pub fn bs_blend_tread_transf_interpolator_interpolator_is_correct_type(
    e: &mut Engine,
    _this: Ptr,
    interpolator: Ptr,
) -> bool {
    e.vcall(
        interpolator.addr(),
        SLOT_IS_TRANSFORM_VALUE_SUPPORTED,
        &args![],
    )
    .bool()
}

// Translated from 00a42f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): 0x34 bytes from the Ni
/// allocator, the `NiBlendInterpolator` default constructor, `fCurrLerp`
/// 0.0 and the class's vtable. Null when the allocation fails.
pub fn fn_00a42f90(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![BLEND_TREAD_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, &args![object]);
    e.mem.set_u32(object + CURR_LERP, 0);
    e.mem.set_u32(object, BLEND_TREAD_VTABLE);
    Ptr::new(object)
}

// Translated from 00a42ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::CreateClone` (Xbox PDB): a new object as
/// the factory makes it (null when the allocation fails), the
/// `NiBlendInterpolator` members copied into it (`00a37510`), then
/// `fCurrLerp` (`FLD`/`FSTP`); the exe stores it through the clone pointer
/// even when that is null.
pub fn bs_blend_tread_transf_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let clone = fn_00a42f90(e);
    e.call(
        BLEND_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    let lerp = x87_quiet_bits(e.mem.u32(this.addr() + CURR_LERP));
    e.mem.set_u32(clone.addr().wrapping_add(CURR_LERP), lerp);
    clone
}

/// The three invalid markers both value functions leave in a transform:
/// translation x (+0), the rotation's second float (+0x10) and the scale
/// (+0x1C), each the float at `01015f5c` through the FPU.
fn invalidate_transform(e: &mut Engine, value: u32) {
    let invalid = x87_quiet_bits(e.mem.u32(INVALID_PART));
    e.mem.set_u32(value, invalid);
    e.mem.set_u32(value + 0x10, invalid);
    e.mem.set_u32(value + 0x1c, invalid);
}

/// The `fCurrLerp` wrap both value functions end with: `sum` (a float's
/// bits) is stored, then replaced by `sum - 1` when above 1 or `sum + 1`
/// when below 0 (the `double` 1.0 at `01012070`; NaN stays).
fn wrap_lerp(e: &mut Engine, object: u32, sum: u32) {
    let sum = x87_load32(sum);
    e.mem.set_u32(object + CURR_LERP, x87_store32(sum));
    let one = f64::from_bits(e.global::<u64>(ONE));
    if 1.0 < sum {
        e.mem
            .set_u32(object + CURR_LERP, x87_store32(x87_sub(sum, one)));
    } else if 0.0 > sum {
        e.mem
            .set_u32(object + CURR_LERP, x87_store32(x87_add(sum, one)));
    }
}

// Translated from 00a43040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::StoreSingleValue` (Xbox PDB; `thiscall
/// (time, target, &value)`): the time is the single time when manager
/// controlled (flag 1). At `INVALID_FLOAT`, or when the single
/// interpolator's transform `Update` (vtable +0x8C) fails, the value's three
/// parts are marked invalid and false is answered. Otherwise `fCurrLerp`
/// gains the interpolator's `fDiffLerp` (wrapped into 0..1), the value's
/// rotation is normalized (`00a38560` on a copy, copied back) unless its
/// second float is `-FLT_MAX`, and true is answered.
pub fn bs_blend_tread_transf_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let time = if e.mem.u8(object + BLEND_FLAGS) & 1 != 0 {
        x87_quiet_bits(e.mem.u32(object + BLEND_SINGLE_TIME))
    } else {
        x87_quiet_bits(time.to_bits())
    };
    let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
    if invalid == x87_load32(time) {
        invalidate_transform(e, value.addr());
        return false;
    }
    let single = e.mem.u32(object + BLEND_SINGLE_INTERPOLATOR);
    if !e
        .vcall(single, SLOT_UPDATE_TRANSFORM, &args![time, target, value])
        .bool()
    {
        invalidate_transform(e, value.addr());
        return false;
    }
    let single = e.mem.u32(object + BLEND_SINGLE_INTERPOLATOR);
    let sum = x87_add(
        x87_load32(e.mem.u32(single + INTERPOLATOR_DIFF_LERP)),
        x87_m32(e.mem.u32(object + CURR_LERP)),
    );
    wrap_lerp(e, object, x87_store32(sum));
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    let value = value.addr();
    if x87_load32(e.mem.u32(value + 0x10)) != lowest {
        e.with_stack(0x10, |e, rotation| {
            let rotation = rotation.addr();
            for i in 0..4 {
                let word = e.mem.u32(value + 0xc + 4 * i);
                e.mem.set_u32(rotation + 4 * i, word);
            }
            e.call(QUATERNION_NORMALIZE, &args![rotation]);
            for i in 0..4 {
                let word = e.mem.u32(rotation + 4 * i);
                e.mem.set_u32(value + 0xc + 4 * i, word);
            }
        });
    }
    true
}

// Translated from 00a43160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::BlendValues` (Xbox PDB; `thiscall
/// (time, target, &value)`): for every item with an interpolator and a
/// normalized weight above 0 (and not 0), at its time (its update time when
/// manager controlled) unless that is `INVALID_FLOAT`, the interpolator's
/// transform `Update` (vtable +0x8C) into a fresh transform local; each that
/// succeeds adds its `fDiffLerp` times its weight to a sum (from 0). The sum
/// plus `fCurrLerp` becomes `fCurrLerp` (wrapped into 0..1); the value's three
/// parts are marked invalid and true is answered. Every product and sum is
/// an x87 operation stored as a `float`, in the exe's order.
pub fn bs_blend_tread_transf_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let mut sum = 0u32;
    let mut i = 0u8;
    while i < e.mem.u8(object + BLEND_ARRAY_SIZE) {
        let entry = e
            .mem
            .u32(object + BLEND_INTERP_ARRAY)
            .wrapping_add(u32::from(i) * ITEM_SIZE);
        let interpolator = e.mem.u32(entry);
        let weight = x87_m32(e.mem.u32(entry + ITEM_NORMALIZED_WEIGHT));
        if interpolator != 0 && 0.0 < weight && 0.0 != weight {
            let item_time = if e.mem.u8(object + BLEND_FLAGS) & 1 != 0 {
                x87_quiet_bits(e.mem.u32(entry + ITEM_UPDATE_TIME))
            } else {
                x87_quiet_bits(time.to_bits())
            };
            let invalid = x87_load32(e.mem.u32(INVALID_FLOAT));
            if invalid != x87_load32(item_time) {
                let updated = e.with_stack(0x20, |e, transform| {
                    let transform = transform.addr();
                    for k in 0..3 {
                        let word = e.mem.u32(FRESH_TRANSLATE + 4 * k);
                        e.mem.set_u32(transform + 4 * k, word);
                    }
                    for k in 0..4 {
                        let word = e.mem.u32(FRESH_ROTATE + 4 * k);
                        e.mem.set_u32(transform + 0xc + 4 * k, word);
                    }
                    let scale = x87_quiet_bits(e.mem.u32(FRESH_SCALE));
                    e.mem.set_u32(transform + 0x1c, scale);
                    e.vcall(
                        interpolator,
                        SLOT_UPDATE_TRANSFORM,
                        &args![item_time, target, transform],
                    )
                    .bool()
                });
                if updated {
                    let interpolator = e.mem.u32(entry);
                    let product = x87_mul(
                        x87_load32(e.mem.u32(interpolator + INTERPOLATOR_DIFF_LERP)),
                        x87_m32(e.mem.u32(entry + ITEM_NORMALIZED_WEIGHT)),
                    );
                    sum = x87_store32(x87_add(product, x87_m32(sum)));
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let total = x87_add(x87_load32(e.mem.u32(object + CURR_LERP)), x87_m32(sum));
    wrap_lerp(e, object, x87_store32(total));
    invalidate_transform(e, value.addr());
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42fe0, bs_blend_tread_transf_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_0a10,
            bs_blend_tread_transf_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_2fc0,
            bs_blend_tread_transf_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_2fd0, bs_blend_tread_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_3330,
            bs_blend_tread_transf_interpolator_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a4_2f80,
            bs_blend_tread_transf_interpolator_destructor(Ptr)
        ),
        entry!(0x00a4_32d0, bs_blend_tread_transf_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
        entry!(
            0x00a3_9100,
            bs_blend_tread_transf_interpolator_interpolator_is_correct_type(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_2f90, fn_00a42f90() -> Ptr),
        entry!(
            0x00a4_2ff0,
            bs_blend_tread_transf_interpolator_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a4_3040,
            bs_blend_tread_transf_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a4_3160,
            bs_blend_tread_transf_interpolator_blend_values(Ptr, f32, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_blend_tread_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a42fe0, &args![]).u32(),
            BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a40a10_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_0a10;
        const TARGET: u32 = 0x00a3_7ec0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a42fc0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_2fc0;
        const TARGET: u32 = 0x00a3_7980;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a42fd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_2fd0;
        const TARGET: u32 = 0x00a3_6fd0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a43330_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3330;
        const TARGET: u32 = 0x00a9_5ca0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    fn blend_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn blend_tread_destructor_sets_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x34);
        e.call(0x00a4_2f80, &args![this]);
        assert_eq!(e.mem.u32(this), BLEND_TREAD_VTABLE);
        assert_eq!(
            blend_calls_to(&e, BLEND_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this]]
        );
    }

    #[test]
    fn blend_tread_update_stores_blends_or_does_nothing_by_count() {
        let mut e = Engine::new();
        e.register(STORE_SINGLE_VALUE, |_, _| 1u32.into_ret());
        e.register(BLEND_VALUES, |_, _| 0x101u32.into_ret());
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        let time = 0.5f32.to_bits();
        for (count, answer) in [(1u8, true), (3, true), (0, false)] {
            e.call_log = Some(vec![]);
            e.mem.set_u8(this + BLEND_INTERP_COUNT, count);
            e.mem.set_f32(this + BLEND_LAST_TIME, 0.0);
            let result = e
                .call(0x00a4_32d0, &args![this, 0.5f32, 0x6000u32, 0x7000u32])
                .bool();
            assert_eq!(result, answer);
            assert_eq!(e.mem.f32(this + BLEND_LAST_TIME), 0.5);
            let single = blend_calls_to(&e, STORE_SINGLE_VALUE);
            let blended = blend_calls_to(&e, BLEND_VALUES);
            match count {
                1 => {
                    assert_eq!(single, vec![vec![this, time, 0x6000, 0x7000]]);
                    assert!(blended.is_empty());
                }
                3 => {
                    assert_eq!(blended, vec![vec![this, time, 0x6000, 0x7000]]);
                    assert_eq!(
                        blend_calls_to(&e, COMPUTE_NORMALIZED_WEIGHTS),
                        vec![vec![this]]
                    );
                }
                _ => assert!(single.is_empty() && blended.is_empty()),
            }
        }
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::calls;

    const CHILD_VTABLE: u32 = 0x7400_0000;
    fn slot(offset: u32) -> u32 {
        0x7500_0000 + offset
    }

    /// The constants the value functions read, and a child vtable whose
    /// transform `Update` answers the child's word +0x10 (and writes the
    /// time into the transform's first float) and whose
    /// `IsTransformValueSupported` answers 1.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0109_6000u32,
            0x0101_5000,
            0x0102_4000,
            0x0101_2000,
            0x011a_8000,
            0x011f_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(INVALID_FLOAT, f32::MIN);
        e.mem.set_f32(INVALID_PART, f32::MIN);
        e.set_global(LOWEST, f64::from(f32::MIN));
        e.set_global(ONE, 1.0f64);
        e.mem.set_f32(FRESH_SCALE, 1.0);
        let slots: Vec<u32> = (0..0x40).map(|i| slot(4 * i)).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        e.register(slot(SLOT_IS_TRANSFORM_VALUE_SUPPORTED), |_, _| {
            1u32.into_ret()
        });
        e.register(slot(SLOT_UPDATE_TRANSFORM), |e, a| {
            e.mem.set_u32(a[3], a[1]);
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        e.register(QUATERNION_NORMALIZE, |e, a| {
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v * 0.5);
            }
            Ret::default()
        });
        e
    }

    /// A child interpolator answering `ok` with `fDiffLerp` `diff`.
    fn child(e: &mut Engine, diff: f32, ok: u32) -> u32 {
        let object = e.mem.alloc(0x24);
        e.mem.set_u32(object, CHILD_VTABLE);
        e.mem.set_u32(object + 0x10, ok);
        e.mem.set_f32(object + INTERPOLATOR_DIFF_LERP, diff);
        object
    }

    fn blend(e: &mut Engine, size: u8, lerp: f32) -> Ptr {
        let this = e.mem.alloc(BLEND_TREAD_SIZE);
        let array = e.mem.alloc(ITEM_SIZE * u32::from(size).max(1));
        e.mem.set_u32(this + BLEND_INTERP_ARRAY, array);
        e.mem.set_u8(this + BLEND_ARRAY_SIZE, size);
        e.mem.set_f32(this + CURR_LERP, lerp);
        Ptr::new(this)
    }

    fn put(e: &mut Engine, this: Ptr, index: u32, interpolator: u32, weight: f32) {
        let at = e.mem.u32(this.addr() + BLEND_INTERP_ARRAY) + index * ITEM_SIZE;
        e.mem.set_u32(at, interpolator);
        e.mem.set_f32(at + ITEM_NORMALIZED_WEIGHT, weight);
        e.mem.set_f32(at + ITEM_UPDATE_TIME, 9.0);
    }

    fn invalid_parts(e: &Engine, value: u32) -> [f32; 3] {
        [
            e.mem.f32(value),
            e.mem.f32(value + 0x10),
            e.mem.f32(value + 0x1c),
        ]
    }

    #[test]
    fn interpolator_is_correct_type_asks_the_interpolator() {
        let mut e = engine();
        let interp = child(&mut e, 0.0, 1);
        assert!(e.call(0x00a3_9100, &args![0u32, interp]).bool());
    }

    #[test]
    fn create_object_starts_with_no_lerp() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, |e, a| {
            e.mem.set_u32(a[0] + CURR_LERP, 7);
            Ret::default()
        });
        let object = e.call(0x00a4_2f90, &args![]).u32();
        assert_eq!(e.mem.u32(object), BLEND_TREAD_VTABLE);
        assert_eq!(e.mem.u32(object + CURR_LERP), 0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_2f90, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_members_and_the_lerp() {
        let mut e = engine();
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BLEND_INTERPOLATOR_CONSTRUCT_DEFAULT, |_, _| Ret::default());
        e.register(BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = blend(&mut e, 0, 0.75);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_2ff0, &args![this, 0x66u32]).u32();
        assert_eq!(e.mem.f32(clone + CURR_LERP), 0.75);
        assert_eq!(
            calls(&e, BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x66]]
        );
    }

    #[test]
    fn store_single_value_adds_the_diff_and_normalizes() {
        let mut e = engine();
        let this = blend(&mut e, 0, 0.75);
        let single = child(&mut e, 0.5, 1);
        e.mem
            .set_u32(this.addr() + BLEND_SINGLE_INTERPOLATOR, single);
        let value = e.mem.alloc(0x20);
        e.mem.set_f32(value + 0x10, 2.0);
        assert!(e
            .call(0x00a4_3040, &args![this, 3.0f32, 0u32, value])
            .bool());
        // 0.75 + 0.5 = 1.25 wraps to 0.25.
        assert_eq!(e.mem.f32(this.addr() + CURR_LERP), 0.25);
        assert_eq!(e.mem.f32(value), 3.0, "the update's time");
        assert_eq!(e.mem.f32(value + 0x10), 1.0, "normalized");
        // The rotation is left alone when its second float is -FLT_MAX.
        e.mem.set_f32(value + 0x10, f32::MIN);
        e.mem.set_f32(this.addr() + CURR_LERP, -0.75);
        assert!(e
            .call(0x00a4_3040, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this.addr() + CURR_LERP), 0.75, "-0.25 wraps");
        assert_eq!(e.mem.f32(value + 0x10), f32::MIN);
        // Manager controlled: the single time is used.
        e.mem.set_u8(this.addr() + BLEND_FLAGS, 1);
        e.mem.set_f32(this.addr() + BLEND_SINGLE_TIME, 6.0);
        e.call(0x00a4_3040, &args![this, 3.0f32, 0u32, value]);
        assert_eq!(e.mem.f32(value), 6.0);
    }

    #[test]
    fn store_single_value_marks_the_value_invalid_on_failure() {
        let mut e = engine();
        let this = blend(&mut e, 0, 0.0);
        let single = child(&mut e, 0.5, 0);
        e.mem
            .set_u32(this.addr() + BLEND_SINGLE_INTERPOLATOR, single);
        let value = e.mem.alloc(0x20);
        assert!(!e
            .call(0x00a4_3040, &args![this, 1.0f32, 0u32, value])
            .bool());
        assert_eq!(invalid_parts(&e, value), [f32::MIN; 3]);
        let value = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a4_3040, &args![this, f32::MIN, 0u32, value])
            .bool());
        assert_eq!(invalid_parts(&e, value), [f32::MIN; 3]);
        assert!(calls(&e, slot(SLOT_UPDATE_TRANSFORM)).is_empty());
        assert_eq!(e.mem.f32(this.addr() + CURR_LERP), 0.0);
    }

    #[test]
    fn blend_values_sums_weighted_diffs_of_the_updated_items() {
        let mut e = engine();
        let this = blend(&mut e, 4, 0.5);
        let (a, b, c) = (
            child(&mut e, 0.25, 1),
            child(&mut e, 4.0, 0),
            child(&mut e, 1.0, 1),
        );
        put(&mut e, this, 0, a, 0.5);
        put(&mut e, this, 1, b, 0.5);
        put(&mut e, this, 2, c, 0.0);
        put(&mut e, this, 3, 0, 1.0);
        let value = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00a4_3160, &args![this, 2.0f32, 0x77u32, value])
            .bool());
        // 0.5 + 0.25 * 0.5.
        assert_eq!(e.mem.f32(this.addr() + CURR_LERP), 0.625);
        assert_eq!(invalid_parts(&e, value), [f32::MIN; 3]);
        let updates = calls(&e, slot(SLOT_UPDATE_TRANSFORM));
        assert_eq!(updates.len(), 2, "items with a positive weight");
        assert_eq!(updates[0][..3], [a, 2.0f32.to_bits(), 0x77]);
        // Manager controlled: each item's own update time; above 1 wraps.
        e.mem.set_u8(this.addr() + BLEND_FLAGS, 1);
        e.mem.set_f32(this.addr() + CURR_LERP, 0.9375);
        e.call_log = Some(vec![]);
        e.call(0x00a4_3160, &args![this, 2.0f32, 0x77u32, value]);
        assert_eq!(e.mem.f32(this.addr() + CURR_LERP), 0.0625);
        assert_eq!(
            calls(&e, slot(SLOT_UPDATE_TRANSFORM))[0][1],
            9.0f32.to_bits()
        );
    }
}
