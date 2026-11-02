//! `NiAnimation/niblendaccumtransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `-FLT_MAX` (`double`, `010241b0`): the invalid marker of transform parts.
const LOWEST: u32 = 0x0102_41b0;
/// `NiInterpolator::INVALID_FLOAT` (Xbox PDB static; `-FLT_MAX`).
const INVALID_FLOAT: u32 = 0x0109_696c;
/// `NiBlendInterpolator::INVALID_INDEX` (Xbox PDB static byte, `0109_6968`).
const INVALID_INDEX: u32 = 0x0109_6968;
/// The float `-FLT_MAX` (`01015f5c`) written into invalid transform parts,
/// and the one (`01096b8c`) a fresh transform's scale starts as.
const INVALID_TRANSFORM_PART: u32 = 0x0101_5f5c;
const INVALID_SCALE: u32 = 0x0109_6b8c;
/// The float `0.0` (`01011d78`) and `1e-5` (`010718c0`).
const ZERO: u32 = 0x0101_1d78;
const SMALL_WEIGHT: u32 = 0x0107_18c0;
/// Words copied as they are: the zero translation (3 words, `011f426c`),
/// the identity rotation (4 words, `011a9ea4`), the identity matrix (9
/// words, `011a9448`), and a fresh transform's translation (`011a8400`) and
/// rotation (`011f3704`).
const ZERO_TRANSLATE: u32 = 0x011f_426c;
const IDENTITY_ROTATE: u32 = 0x011a_9ea4;
const IDENTITY_MATRIX: u32 = 0x011a_9448;
const FRESH_TRANSLATE: u32 = 0x011a_8400;
const FRESH_ROTATE: u32 = 0x011f_3704;

/// `NiBlendAccumTransformInterpolator`'s vtable.
const VTABLE: u32 = 0x0109_6784;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free
/// (`00aa4060`), both `thiscall`, one word.
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
/// The compiler's vector constructor iterator (`00401050`, `stdcall (array,
/// element size, count, constructor)`) and the accumulation item's
/// constructor it is given (`00a362f0`).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
const ACCUM_ITEM_CONSTRUCTOR: u32 = 0x00a3_62f0;
/// The base class's functions (`niblendinterpolator.cpp`).
const BASE_DESTRUCTOR: u32 = 0x00a3_7170;
const BASE_ADD_INTERP_INFO: u32 = 0x00a3_6970;
const BASE_REMOVE_INTERP_INFO: u32 = 0x00a3_6a90;
const BASE_REALLOCATE_ARRAYS: u32 = 0x00a3_7690;
const BASE_COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// `NiQuatTransform::HierApply` (`00a38830`, `thiscall (this, &result,
/// other)`, answers the result), `HierInvert` (`00a386a0`, `thiscall (this,
/// &result)`), the transform product `00a38a00` (`thiscall (this, &result,
/// other)`, answers the result), the inverse `00a385d0` (`thiscall (this,
/// &result)`), and the transform constructor `00494260` (`thiscall`).
const HIER_APPLY: u32 = 0x00a3_8830;
const HIER_INVERT: u32 = 0x00a3_86a0;
const TRANSFORM_PRODUCT: u32 = 0x00a3_8a00;
const TRANSFORM_INVERSE: u32 = 0x00a3_85d0;
const TRANSFORM_CONSTRUCTOR: u32 = 0x0049_4260;
/// `NiQuaternion` helpers: the constructor from four floats (`00a6db50`,
/// `thiscall`, `RET 0x10`), `UnitInverse` (`00a6dc00`, `cdecl (&result,
/// quaternion)`), `operator*` with a quaternion (`00a2bcf0`, `thiscall (this,
/// &result, other)`), with a float (`00a39120`, `thiscall (this, &result,
/// float)`), `operator-` (`00a6dbd0`, `thiscall (this, &result)`) and
/// `ToRotation` (`004f0180`, `thiscall (this, &matrix)`). The matrix times a
/// point (`004b4500`, `thiscall (matrix, &result, point)`).
const QUATERNION_CONSTRUCTOR: u32 = 0x00a6_db50;
const QUATERNION_UNIT_INVERSE: u32 = 0x00a6_dc00;
const QUATERNION_PRODUCT: u32 = 0x00a2_bcf0;
const QUATERNION_SCALE: u32 = 0x00a3_9120;
const QUATERNION_NEGATE: u32 = 0x00a6_dbd0;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// The CRT square root `_CIsqrt` (`00ec6040`, argument and result in ST0).
const CRT_SQRT: u32 = 0x00ec_6040;
/// The interpolators' transform `Update` (vtable +0x8C, `(time, object,
/// NiQuatTransform*)`) and `GetActiveTimeRange` (vtable +0xC0, `(&begin,
/// &end)`).
const UPDATE_TRANSFORM_SLOT: u32 = 0x8c;
const ACTIVE_TIME_RANGE_SLOT: u32 = 0xc0;

/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` +8, `m_uFlags`
/// +0xC (bit 1 manager controlled), `m_ucArraySize` +0xD, `m_ucInterpCount`
/// +0xE, `m_ucSingleIdx` +0xF, `m_pkInterpArray` +0x14 (items of 0x18:
/// interpolator +0, normalized weight +8, update time +0x14),
/// `m_fSingleTime` +0x20.
const LAST_TIME: u32 = 0x8;
const FLAGS: u32 = 0xc;
const ARRAY_SIZE: u32 = 0xd;
const INTERP_COUNT: u32 = 0xe;
const SINGLE_INDEX: u32 = 0xf;
const INTERP_ARRAY: u32 = 0x14;
const SINGLE_TIME: u32 = 0x20;
const ITEM_SIZE: u32 = 0x18;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_UPDATE_TIME: u32 = 0x14;
/// `m_kAccumulatedTransformValue` +0x30, `m_pkAccumArray` +0x50, `m_bReset`
/// +0x54 (Xbox PDB).
const ACCUMULATED: u32 = 0x30;
const ACCUM_ARRAY: u32 = 0x50;
const RESET: u32 = 0x54;
/// `AccumArrayItem` on PC, 0x68 bytes: `m_fLastTime` +0, `m_kLastValue` +4,
/// `m_kDeltaValue` +0x24, `m_kRefFrame` +0x44 (the Xbox build aligns the
/// transforms to 16 bytes).
const ACCUM_ITEM_SIZE: u32 = 0x68;
const LAST_VALUE: u32 = 0x4;
const DELTA_VALUE: u32 = 0x24;
const REF_FRAME: u32 = 0x44;
/// `NiQuatTransform` (Xbox PDB): translate +0, rotate +0xC, scale +0x1C.
const ROTATE: u32 = 0xc;
const SCALE: u32 = 0x1c;

fn byte(e: &Engine, this: Ptr, offset: u32) -> u8 {
    e.mem.u8(this.addr().wrapping_add(offset))
}

fn lowest(e: &Engine) -> f64 {
    f64::from_bits(e.global::<u64>(LOWEST))
}

fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), value);
    }
}

/// The accumulation item of base item `index`.
fn accum_item(e: &Engine, this: Ptr, index: u32) -> u32 {
    e.mem
        .u32(this.addr() + ACCUM_ARRAY)
        .wrapping_add(index.wrapping_mul(ACCUM_ITEM_SIZE))
}

/// The base item `index`'s interpolator.
fn interpolator(e: &Engine, this: Ptr, index: u32) -> u32 {
    let array = e.mem.u32(this.addr() + INTERP_ARRAY);
    e.mem.u32(array.wrapping_add(index.wrapping_mul(ITEM_SIZE)))
}

/// Marks an accumulation item unset: the invalid marker in its last time,
/// last value (translation, rotation's +4 float, scale) and delta value
/// (the same parts), and the identity matrix as its reference frame.
fn reset_accum_item(e: &mut Engine, at: u32) {
    let invalid = x87_quiet_bits(e.mem.u32(INVALID_TRANSFORM_PART));
    for offset in [0u32, 0x4, 0x14, 0x20, 0x24, 0x34, 0x40] {
        e.mem.set_u32(at + offset, invalid);
    }
    copy_words(e, IDENTITY_MATRIX, at + REF_FRAME, 9);
}

/// Whether the transform at `at` has its scale, rotation (+0x10) and
/// translation all at the invalid marker (the inlined form of
/// `fn_00a39180`).
fn all_invalid(e: &Engine, at: u32) -> bool {
    let lowest = lowest(e);
    x87_load32(e.mem.u32(at + SCALE)) == lowest
        && x87_load32(e.mem.u32(at + 0x10)) == lowest
        && x87_load32(e.mem.u32(at)) == lowest
}

/// `this` transform combined with `other` by the function at `function`
/// (`thiscall (this, &result, other)`), the result's eight words copied to
/// `to`.
fn combine_into(e: &mut Engine, function: u32, this: u32, other: u32, to: u32) {
    e.with_stack(0x20, |e, result| {
        let answer = e.call(function, &args![this, result, other]).u32();
        copy_words(e, answer, to, 8);
    });
}

// Translated from 00a39180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiQuatTransform` test, `thiscall`): whether
/// the transform's scale (+0x1C), rotation float at +0x10 and translation
/// x (+0) are all the invalid marker `-FLT_MAX`.
pub fn fn_00a39180(e: &mut Engine, this: Ptr) -> bool {
    all_invalid(e, this.addr())
}

// Translated from 00a3f990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map's `NiQuatTransform::IsTransformInvalid` is a folded name:
/// the body reads the object at `this + 0x2C`; with one it writes its
/// 16-bit word at +0xA (zero-extended), its word at +0x14 and its byte at
/// +0x1D to the three outputs and answers its word at +0x24; without, the
/// outputs are 0 and the answer 0.
pub fn fn_00a3f990(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) -> u32 {
    let object = e.mem.u32(this.addr().wrapping_add(0x2c));
    if object == 0 {
        e.mem.set_u32(first.addr(), 0);
        e.mem.set_u32(second.addr(), 0);
        e.mem.set_u8(third.addr(), 0);
        return 0;
    }
    let value = u32::from(e.mem.u16(object + 0xa));
    e.mem.set_u32(first.addr(), value);
    let value = e.mem.u32(object + 0x14);
    e.mem.set_u32(second.addr(), value);
    let value = e.mem.u8(object + 0x1d);
    e.mem.set_u8(third.addr(), value);
    e.mem.u32(object + 0x24)
}

// Translated from 00a39290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::~NiBlendAccumTransformInterpolator`
/// (Xbox PDB): the class's vtable, the accumulation array freed through the
/// memory manager, then `NiBlendInterpolator`'s destructor.
pub fn ni_blend_accum_transform_interpolator_destructor(e: &mut Engine, this: Ptr) {
    let array = e.mem.u32(this.addr() + ACCUM_ARRAY);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, array]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

// Translated from 00a392b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::AddInterpInfo` (Xbox PDB): the base
/// `AddInterpInfo`; unless it answers `INVALID_INDEX`, the new item's
/// accumulation item is reset; with the interpolator now the only one the
/// interpolator is marked for reset (`m_bReset`); with two, the old single
/// item's last value is first applied onto the accumulated transform
/// (`HierApply`, when both are set) and that item reset too. Answers the
/// index.
pub fn ni_blend_accum_transform_interpolator_add_interp_info(
    e: &mut Engine,
    this: Ptr,
    interpolator: Ptr,
    weight: f32,
    priority: u32,
    ease_spinner: f32,
) -> u8 {
    let old_single = byte(e, this, SINGLE_INDEX);
    let index = e
        .call(
            BASE_ADD_INTERP_INFO,
            &args![
                this,
                interpolator,
                x87_quiet_bits(weight.to_bits()),
                priority,
                x87_quiet_bits(ease_spinner.to_bits())
            ],
        )
        .u8();
    if index == e.mem.u8(INVALID_INDEX) {
        return index;
    }
    let item = accum_item(e, this, u32::from(index));
    reset_accum_item(e, item);
    match byte(e, this, INTERP_COUNT) {
        1 => e.mem.set_u8(this.addr() + RESET, 1),
        2 => {
            let accumulated = this.addr() + ACCUMULATED;
            if !all_invalid(e, accumulated) {
                let last = accum_item(e, this, u32::from(old_single)) + LAST_VALUE;
                if !all_invalid(e, last) {
                    combine_into(e, HIER_APPLY, accumulated, last, accumulated);
                }
            }
            let old = accum_item(e, this, u32::from(old_single));
            reset_accum_item(e, old);
        }
        _ => {}
    }
    index
}

// Translated from 00a393c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::RemoveInterpInfo` (Xbox PDB): when
/// the single interpolator itself is removed and the accumulated transform
/// is set, the single item's last value (when set) is applied onto it
/// first (`HierApply`); then the base `RemoveInterpInfo`; when one
/// interpolator is left and the removed item had one, the new single item
/// is reset and the interpolator marked for reset. The "had one" flag is
/// kept in the index argument's stack byte.
pub fn ni_blend_accum_transform_interpolator_remove_interp_info(
    e: &mut Engine,
    this: Ptr,
    index: u32,
) {
    let accumulated = this.addr() + ACCUMULATED;
    if byte(e, this, INTERP_COUNT) == 1
        && index as u8 == byte(e, this, SINGLE_INDEX)
        && !all_invalid(e, accumulated)
    {
        let last = accum_item(e, this, u32::from(byte(e, this, SINGLE_INDEX))) + LAST_VALUE;
        if !all_invalid(e, last) {
            combine_into(e, HIER_APPLY, accumulated, last, accumulated);
        }
    }
    let had = interpolator(e, this, index & 0xff) != 0;
    e.call(BASE_REMOVE_INTERP_INFO, &args![this, index]);
    if byte(e, this, INTERP_COUNT) == 1 && had {
        let single = accum_item(e, this, u32::from(byte(e, this, SINGLE_INDEX)));
        e.mem.set_u8(this.addr() + RESET, 1);
        reset_accum_item(e, single);
    }
}

// Translated from 00a39ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::ReallocateArrays` (Xbox PDB): the base
/// `ReallocateArrays`; when it grew the arrays, a new accumulation array of
/// the new size (`size * 0x68` bytes, all ones on overflow, each item
/// constructed by `00a362f0`), the old items copied over (0x68 bytes each,
/// the old size of them), the old array freed and the new one kept.
/// Answers the base's answer.
pub fn ni_blend_accum_transform_interpolator_reallocate_arrays(e: &mut Engine, this: Ptr) -> bool {
    let old_size = byte(e, this, ARRAY_SIZE);
    let grew = e.call(BASE_REALLOCATE_ARRAYS, &args![this]).bool();
    if !grew {
        return false;
    }
    let size = u32::from(byte(e, this, ARRAY_SIZE));
    let bytes = size.checked_mul(ACCUM_ITEM_SIZE).unwrap_or(u32::MAX);
    let array = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    if array != 0 {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![array, ACCUM_ITEM_SIZE, size, ACCUM_ITEM_CONSTRUCTOR],
        );
    }
    let old = e.mem.u32(this.addr() + ACCUM_ARRAY);
    for i in 0..u32::from(old_size) {
        let offset = i * ACCUM_ITEM_SIZE;
        copy_words(
            e,
            old.wrapping_add(offset),
            array.wrapping_add(offset),
            0x1a,
        );
    }
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER, old]);
    e.mem.set_u32(this.addr() + ACCUM_ARRAY, array);
    true
}

// Translated from 00a39530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall (this, time, object)`, the single
/// interpolator's step of `StoreSingleValue`): unless `time` is the single
/// accumulation item's last time, when time went back (the item's last time
/// is after `time`) the item's delta value is computed once from the
/// interpolator's active range (`GetActiveTimeRange`, vtable +0xC0, and
/// `Update` at its begin and end: the end transform combined with the
/// inverse, `HierInvert`, of the begin transform) and, when the
/// accumulated transform is set, the delta is applied onto it
/// (`HierApply`); then the item's last value is updated at `time`. The last
/// time becomes `time`.
pub fn fn_00a39530(e: &mut Engine, this: Ptr, time: f32, object: Ptr) {
    let index = u32::from(byte(e, this, SINGLE_INDEX));
    let item = accum_item(e, this, index);
    let child = interpolator(e, this, index);
    let time_bits = x87_quiet_bits(time.to_bits());
    let time_value = x87_load32(time_bits);
    let last_time = x87_load32(e.mem.u32(item));
    if time_value != last_time {
        if last_time > time_value {
            let delta = item + DELTA_VALUE;
            if all_invalid(e, delta) {
                e.with_stack(0x90, |e, frame| {
                    let frame = frame.addr();
                    let (end, begin) = (frame, frame + 4);
                    let at_begin = frame + 0x8;
                    let at_end = frame + 0x28;
                    let inverse = frame + 0x48;
                    let result = frame + 0x68;
                    e.vcall(child, ACTIVE_TIME_RANGE_SLOT, &args![begin, end]);
                    e.call(TRANSFORM_CONSTRUCTOR, &args![at_begin]);
                    let begin_time = x87_quiet_bits(e.mem.u32(begin));
                    e.vcall(
                        child,
                        UPDATE_TRANSFORM_SLOT,
                        &args![begin_time, object, at_begin],
                    );
                    e.call(TRANSFORM_CONSTRUCTOR, &args![at_end]);
                    let end_time = x87_quiet_bits(e.mem.u32(end));
                    e.vcall(
                        child,
                        UPDATE_TRANSFORM_SLOT,
                        &args![end_time, object, at_end],
                    );
                    e.call(TRANSFORM_CONSTRUCTOR, &args![inverse]);
                    e.call(HIER_INVERT, &args![at_begin, inverse]);
                    let answer = e.call(HIER_APPLY, &args![at_end, result, inverse]).u32();
                    copy_words(e, answer, delta, 8);
                });
            }
            let accumulated = this.addr() + ACCUMULATED;
            if !all_invalid(e, accumulated) {
                combine_into(e, HIER_APPLY, accumulated, delta, accumulated);
            }
        }
        e.vcall(
            child,
            UPDATE_TRANSFORM_SLOT,
            &args![time_bits, object, item + LAST_VALUE],
        );
    }
    e.mem.set_u32(item, time_bits);
}

// Translated from 00a397b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::StoreSingleValue` (Xbox PDB): false at
/// the invalid time (the single time when manager controlled). Otherwise
/// `fn_00a39530`; when marked for reset or the accumulated transform is
/// unset, the accumulated transform becomes the inverse (`HierInvert`) of
/// the single item's last value (applied onto the accumulated transform
/// when that was set) and the reset mark is cleared. False when the
/// accumulated transform is still unset; otherwise the result is the
/// accumulated transform applied to the last value (`HierApply`), true.
pub fn ni_blend_accum_transform_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let mut time_bits = time.to_bits();
    if byte(e, this, FLAGS) & 1 != 0 {
        time_bits = x87_quiet_bits(e.mem.u32(this.addr() + SINGLE_TIME));
    }
    let time_value = x87_load32(time_bits);
    if x87_load32(e.mem.u32(INVALID_FLOAT)) == time_value {
        return false;
    }
    fn_00a39530(e, this, f32::from_bits(x87_store32(time_value)), object);
    let accumulated = this.addr() + ACCUMULATED;
    if byte(e, this, RESET) != 0 || all_invalid(e, accumulated) {
        e.with_stack(0x40, |e, frame| {
            let inverse = frame.addr();
            let result = inverse + 0x20;
            copy_words(e, FRESH_TRANSLATE, inverse, 3);
            copy_words(e, FRESH_ROTATE, inverse + ROTATE, 4);
            let scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
            e.mem.set_u32(inverse + SCALE, scale);
            let last = accum_item(e, this, u32::from(byte(e, this, SINGLE_INDEX))) + LAST_VALUE;
            e.call(HIER_INVERT, &args![last, inverse]);
            let source = if all_invalid(e, accumulated) {
                inverse
            } else {
                e.call(HIER_APPLY, &args![accumulated, result, inverse])
                    .u32()
            };
            copy_words(e, source, accumulated, 8);
        });
        e.mem.set_u8(this.addr() + RESET, 0);
    }
    if all_invalid(e, accumulated) {
        return false;
    }
    let last = accum_item(e, this, u32::from(byte(e, this, SINGLE_INDEX))) + LAST_VALUE;
    combine_into(e, HIER_APPLY, accumulated, last, out.addr());
    true
}

// Translated from 00a39f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall (this, index, time, object)`, one
/// item's step of `Update`): without an interpolator, or at its last time,
/// the item's delta becomes the identity (zero translation, identity
/// rotation, scale 1) and its last time `time`. Otherwise, when marked for
/// reset or its last value is unset, the last value is updated at `time`
/// and the reference frame becomes the rotation of the accumulated rotation
/// (identity when unset) times the last rotation's unit inverse (identity
/// when unset). Then, unless time went back without a reset, the delta is
/// the inverse of the last value times the new value at `time`, its
/// translation turned by the reference frame (when set). When time went
/// back the delta is computed across the loop: from the last value to the
/// range's end, the reference frame recomputed from the begin transform,
/// the last value becoming begin-inverse times the new value (translation
/// turned), and the delta then combined with it. Either way the new value
/// becomes the last value, the reset mark is cleared and the last time is
/// `time`. The index is the whole stack word the exe passes (its low byte
/// is the index).
pub fn fn_00a39f60(e: &mut Engine, this: Ptr, index: u32, time: f32, object: Ptr) {
    let index = index & 0xff;
    let item = accum_item(e, this, index);
    let child = interpolator(e, this, index);
    let time_bits = x87_quiet_bits(time.to_bits());
    let time_value = x87_load32(time_bits);
    if child == 0 || time_value == x87_load32(e.mem.u32(item)) {
        copy_words(e, ZERO_TRANSLATE, item + DELTA_VALUE, 3);
        copy_words(e, IDENTITY_ROTATE, item + DELTA_VALUE + ROTATE, 4);
        e.mem.set_u32(item + DELTA_VALUE + SCALE, x87_store32(1.0));
        e.mem.set_u32(item, time_bits);
        return;
    }
    let last = item + LAST_VALUE;
    let delta = item + DELTA_VALUE;
    let frame_matrix = item + REF_FRAME;
    let accumulated_rotate = this.addr() + ACCUMULATED + ROTATE;
    e.with_stack(0xe0, |e, frame| {
        let frame = frame.addr();
        // The exe's locals (offsets from its frame base `F`).
        let point = frame + 0x14;
        let first_quaternion = frame + 0x24;
        let range_begin = frame + 0x34;
        let range_end = frame + 0x38;
        let product = frame + 0x3c;
        let now = frame + 0x4c;
        let scratch = frame + 0x6c;
        let at_end = frame + 0x8c;
        let at_begin = frame + 0xac;
        let result = frame + 0xcc;
        let lowest = lowest(e);
        // `rotation` (4 words) times the unit inverse of the rotation at
        // `inverse_of` (identity when its +4 float is unset), its matrix
        // into the reference frame.
        let reference_frame = |e: &mut Engine, inverse_of: u32, left: u32, right: u32| {
            copy_words(e, IDENTITY_ROTATE, left, 4);
            if x87_load32(e.mem.u32(accumulated_rotate + 4)) != lowest {
                copy_words(e, accumulated_rotate, left, 4);
            }
            copy_words(e, IDENTITY_ROTATE, right, 4);
            if x87_load32(e.mem.u32(inverse_of + 4)) != lowest {
                let answer = e
                    .call(QUATERNION_UNIT_INVERSE, &args![result, inverse_of])
                    .u32();
                copy_words(e, answer, right, 4);
            }
            e.call(QUATERNION_PRODUCT, &args![left, product, right]);
            e.call(QUATERNION_TO_ROTATION, &args![product, frame_matrix]);
        };
        // The translation of the transform at `at` turned by the reference
        // frame, when set.
        let turn = |e: &mut Engine, at: u32| {
            if x87_load32(e.mem.u32(at)) != lowest {
                let answer = e
                    .call(MATRIX_TIMES_POINT, &args![frame_matrix, point, at])
                    .u32();
                copy_words(e, answer, at, 3);
            }
        };
        if byte(e, this, RESET) != 0 || all_invalid(e, last) {
            e.vcall(
                child,
                UPDATE_TRANSFORM_SLOT,
                &args![time_bits, object, last],
            );
            reference_frame(e, item + LAST_VALUE + ROTATE, frame + 0x24, point);
        }
        let last_time = x87_load32(e.mem.u32(item));
        if !(last_time > time_value) || byte(e, this, RESET) != 0 {
            e.call(TRANSFORM_CONSTRUCTOR, &args![now]);
            e.vcall(child, UPDATE_TRANSFORM_SLOT, &args![time_bits, object, now]);
            e.call(TRANSFORM_CONSTRUCTOR, &args![at_end]);
            e.call(TRANSFORM_INVERSE, &args![last, at_end]);
            combine_into(e, TRANSFORM_PRODUCT, at_end, now, delta);
            turn(e, delta);
        } else {
            e.vcall(
                child,
                ACTIVE_TIME_RANGE_SLOT,
                &args![range_begin, range_end],
            );
            e.call(TRANSFORM_CONSTRUCTOR, &args![at_end]);
            let end_time = x87_quiet_bits(e.mem.u32(range_end));
            e.vcall(
                child,
                UPDATE_TRANSFORM_SLOT,
                &args![end_time, object, at_end],
            );
            e.call(TRANSFORM_CONSTRUCTOR, &args![at_begin]);
            let begin_time = x87_quiet_bits(e.mem.u32(range_begin));
            e.vcall(
                child,
                UPDATE_TRANSFORM_SLOT,
                &args![begin_time, object, at_begin],
            );
            e.call(TRANSFORM_CONSTRUCTOR, &args![now]);
            e.vcall(child, UPDATE_TRANSFORM_SLOT, &args![time_bits, object, now]);
            e.call(TRANSFORM_CONSTRUCTOR, &args![scratch]);
            e.call(TRANSFORM_INVERSE, &args![last, scratch]);
            combine_into(e, TRANSFORM_PRODUCT, scratch, at_end, delta);
            turn(e, delta);
            reference_frame(e, at_begin + ROTATE, point, first_quaternion);
            e.call(TRANSFORM_INVERSE, &args![at_begin, scratch]);
            combine_into(e, TRANSFORM_PRODUCT, scratch, now, last);
            turn(e, last);
            combine_into(e, TRANSFORM_PRODUCT, delta, last, delta);
        }
        copy_words(e, now, last, 8);
    });
    e.mem.set_u8(this.addr() + RESET, 0);
    e.mem.set_u32(item, time_bits);
}

// Translated from 00a39960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall (this, time, object, out)`, the
/// blend of `Update`; the time and object words are not read): when the
/// accumulated transform is unset it becomes the identity (and the items'
/// last values are blended instead of their deltas). Every item with an
/// interpolator and a normalized weight above `1e-5` adds its weighted
/// delta (or last value): the translation (else its weight comes off the
/// translation total, from 1), the rotation (`NiQuaternion` scaled by the
/// weight, `00a39120`; each after the first flipped with `operator-` when
/// its dot product with the running sum is negative), the scale (else its
/// weight comes off the scale total). Nothing summed: false. Otherwise a
/// fresh transform gets the translation divided by its total, the rotation
/// sum normalized (`1 / sqrt` of its squared length), the scale sum divided
/// by its total when that is above `1e-5`; the accumulated transform
/// becomes itself times that transform (`00a38a00`), and the answer is
/// whether it is set, copied to `out` when it is. The float roundings follow
/// the exe's stores.
pub fn fn_00a39960(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32, out: Ptr) -> bool {
    let lowest = lowest(e);
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let operand = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    let one = x87_store32(1.0);
    let mut translate_weight = one;
    let mut scale_weight = one;
    let mut scale_sum = 0u32;
    let mut translate = [0u32; 3];
    for (i, word) in translate.iter_mut().enumerate() {
        *word = e.mem.u32(ZERO_TRANSLATE + 4 * i as u32);
    }
    e.with_stack(0xa0, |e, frame| {
        let frame = frame.addr();
        let sum = frame;
        let quaternion = frame + 0x10;
        let local = frame + 0x20;
        let result = frame + 0x40;
        let fresh = frame + 0x60;
        let product = frame + 0x80;
        e.call(QUATERNION_CONSTRUCTOR, &args![sum, 0u32, 0u32, 0u32, 0u32]);
        let accumulated = this.addr() + ACCUMULATED;
        let was_reset = all_invalid(e, accumulated);
        if was_reset {
            copy_words(e, ZERO_TRANSLATE, accumulated, 3);
            copy_words(e, IDENTITY_ROTATE, accumulated + ROTATE, 4);
            e.mem.set_u32(accumulated + SCALE, one);
        }
        let (mut translated, mut rotated, mut scaled) = (false, false, false);
        let mut first_rotation = true;
        let mut index = 0u8;
        while index < byte(e, this, ARRAY_SIZE) {
            let i = u32::from(index);
            index = index.wrapping_add(1);
            let entry = e
                .mem
                .u32(this.addr() + INTERP_ARRAY)
                .wrapping_add(i * ITEM_SIZE);
            let accum = accum_item(e, this, i);
            if e.mem.u32(entry) == 0 {
                continue;
            }
            let weight_at = entry + ITEM_NORMALIZED_WEIGHT;
            if !(load(e, SMALL_WEIGHT) < operand(e, weight_at)) {
                continue;
            }
            let source = if was_reset {
                accum + LAST_VALUE
            } else {
                accum + DELTA_VALUE
            };
            copy_words(e, source, local, 8);
            let x = load(e, local);
            if x == lowest {
                translate_weight =
                    x87_store32(x87_sub(x87_load32(translate_weight), operand(e, weight_at)));
            } else {
                let weight = x87_load32(x87_quiet_bits(e.mem.u32(weight_at)));
                translated = true;
                let px = x87_store32(x87_mul(x, weight));
                let py = x87_store32(x87_mul(load(e, local + 4), weight));
                let pz = x87_store32(x87_mul(weight, operand(e, local + 8)));
                translate[0] = x87_store32(x87_add(x87_load32(px), x87_m32(translate[0])));
                translate[1] = x87_store32(x87_add(x87_load32(translate[1]), x87_m32(py)));
                translate[2] = x87_store32(x87_add(x87_load32(translate[2]), x87_m32(pz)));
            }
            let rotation = local + ROTATE;
            let r1 = load(e, rotation + 4);
            if r1 != lowest {
                copy_words(e, rotation, quaternion, 4);
                if first_rotation {
                    first_rotation = false;
                } else {
                    let dot = x87_mul(r1, operand(e, sum + 4));
                    let dot = x87_add(dot, x87_mul(load(e, rotation), operand(e, sum)));
                    let dot = x87_add(dot, x87_mul(load(e, rotation + 8), operand(e, sum + 8)));
                    let dot = x87_add(dot, x87_mul(load(e, rotation + 12), operand(e, sum + 12)));
                    let dot = x87_load32(x87_store32(dot));
                    if dot < operand(e, ZERO) {
                        let answer = e.call(QUATERNION_NEGATE, &args![quaternion, result]).u32();
                        copy_words(e, answer, quaternion, 4);
                    }
                }
                let weight = x87_quiet_bits(e.mem.u32(weight_at));
                let answer = e
                    .call(QUATERNION_SCALE, &args![quaternion, result, weight])
                    .u32();
                copy_words(e, answer, quaternion, 4);
                let w = x87_add(load(e, quaternion), operand(e, sum));
                e.mem.set_u32(sum, x87_store32(w));
                rotated = true;
                for component in 1..4u32 {
                    let at = 4 * component;
                    let value = x87_add(load(e, sum + at), operand(e, quaternion + at));
                    e.mem.set_u32(sum + at, x87_store32(value));
                }
            }
            let scale = load(e, local + SCALE);
            if scale == lowest {
                scale_weight =
                    x87_store32(x87_sub(x87_load32(scale_weight), operand(e, weight_at)));
            } else {
                scaled = true;
                let weighted = x87_mul(scale, operand(e, weight_at));
                scale_sum = x87_store32(x87_add(weighted, x87_m32(scale_sum)));
            }
        }
        if !translated && !rotated && !scaled {
            return false;
        }
        let invalid_scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
        e.mem.set_u32(fresh + SCALE, invalid_scale);
        copy_words(e, FRESH_TRANSLATE, fresh, 3);
        copy_words(e, FRESH_ROTATE, fresh + ROTATE, 4);
        if translated {
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, x87_load32(translate_weight))));
            translate[0] = x87_store32(x87_mul(reciprocal, x87_m32(translate[0])));
            e.mem.set_u32(fresh, translate[0]);
            translate[1] = x87_store32(x87_mul(x87_load32(translate[1]), reciprocal));
            e.mem.set_u32(fresh + 4, translate[1]);
            translate[2] = x87_store32(x87_mul(reciprocal, x87_m32(translate[2])));
            e.mem.set_u32(fresh + 8, translate[2]);
        }
        if rotated {
            let qx = load(e, sum + 4);
            let qw = load(e, sum);
            let qy = load(e, sum + 8);
            let qz = load(e, sum + 12);
            let length = x87_add(x87_mul(qx, qx), x87_mul(qw, qw));
            let length = x87_add(length, x87_mul(qy, qy));
            let length = x87_add(length, x87_mul(qz, qz));
            let length = x87_load32(x87_store32(length));
            let root = e.call(CRT_SQRT, &args![length]).f64();
            let root = x87_load32(x87_store32(root));
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, root)));
            let w = x87_store32(x87_mul(reciprocal, operand(e, sum)));
            e.mem.set_u32(sum, w);
            e.mem.set_u32(fresh + ROTATE, w);
            for component in 1..3u32 {
                let at = 4 * component;
                let value = x87_store32(x87_mul(load(e, sum + at), reciprocal));
                e.mem.set_u32(sum + at, value);
                e.mem.set_u32(fresh + ROTATE + at, value);
            }
            let z = x87_store32(x87_mul(reciprocal, operand(e, sum + 12)));
            e.mem.set_u32(sum + 12, z);
            e.mem.set_u32(fresh + ROTATE + 12, z);
        }
        if scaled {
            let total = x87_load32(scale_weight);
            if total > load(e, SMALL_WEIGHT) {
                let scale = x87_mul(x87_div(1.0, total), x87_m32(scale_sum));
                e.mem.set_u32(fresh + SCALE, x87_store32(scale));
            }
        }
        let answer = e
            .call(TRANSFORM_PRODUCT, &args![accumulated, product, fresh])
            .u32();
        copy_words(e, answer, accumulated, 8);
        if all_invalid(e, accumulated) {
            return false;
        }
        copy_words(e, accumulated, out.addr(), 8);
        true
    })
}

// Translated from 00a3a580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::Update` (Xbox PDB): with one
/// interpolator `StoreSingleValue`; with more `ComputeNormalizedWeights`,
/// then each item with an interpolator and a non-zero weight steps its
/// accumulation (`fn_00a39f60`, at its update time when manager controlled,
/// unless that is the invalid time), then the blend (`fn_00a39960`); with
/// none false. `m_fLastTime` becomes `time`. The index word given to
/// `fn_00a39f60` is the exe's local: `this`'s upper three bytes with the
/// index in the low byte.
pub fn ni_blend_accum_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let time_bits = x87_quiet_bits(time.to_bits());
    let count = byte(e, this, INTERP_COUNT);
    let answer = if count == 1 {
        ni_blend_accum_transform_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            out,
        )
    } else if count > 0 {
        e.call(BASE_COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        let mut index = 0u8;
        while index < byte(e, this, ARRAY_SIZE) {
            let entry = e
                .mem
                .u32(this.addr() + INTERP_ARRAY)
                .wrapping_add(u32::from(index) * ITEM_SIZE);
            let mut item_time = time_bits;
            if e.mem.u32(entry) != 0 && !(0.0 == x87_m32(e.mem.u32(entry + ITEM_NORMALIZED_WEIGHT)))
            {
                if byte(e, this, FLAGS) & 1 != 0 {
                    item_time = x87_quiet_bits(e.mem.u32(entry + ITEM_UPDATE_TIME));
                }
                if x87_load32(e.mem.u32(INVALID_FLOAT)) != x87_load32(item_time) {
                    let word = (this.addr() & 0xffff_ff00) | u32::from(index);
                    fn_00a39f60(e, this, word, f32::from_bits(item_time), object);
                }
            }
            index = index.wrapping_add(1);
        }
        fn_00a39960(e, this, time_bits, object.addr(), out)
    } else {
        false
    };
    e.mem.set_u32(this.addr() + LAST_TIME, time_bits);
    answer
}

/// The `NiQuaternion` equality `operator==` uses (`00a92660`, `thiscall
/// (this, &other)`).
const QUATERNION_EQUAL: u32 = 0x00a9_2660;
/// The `NiBlendInterpolator` functions this unit's members build on: the
/// default constructor (`00a36890`), `IsEqual` (`00a36fd0`), `SaveBinary`
/// (`00a37ec0`), `LoadBinary` (`00a37980`), the member copy (`00a37510`,
/// `(this, clone, cloning)`) and `GetViewerStrings` (`00a38080`).
const BASE_CONSTRUCT_DEFAULT: u32 = 0x00a3_6890;
const BASE_IS_EQUAL: u32 = 0x00a3_6fd0;
const BASE_SAVE_BINARY: u32 = 0x00a3_7ec0;
const BASE_LOAD_BINARY: u32 = 0x00a3_7980;
const BASE_COPY_MEMBERS: u32 = 0x00a3_7510;
const BASE_GET_VIEWER_STRINGS: u32 = 0x00a3_8080;
/// `NiQuatTransform::SaveBinary` (`00a38b60`), `LoadBinary` (`00a38bb0`)
/// and `GetViewerStrings` (`00a38cb0`), and `NiMatrix3::SaveBinary`
/// (`00a59a90`) and `LoadBinary` (`00a59a30`), all `thiscall (object,
/// stream or strings)`.
const QUAT_TRANSFORM_SAVE_BINARY: u32 = 0x00a3_8b60;
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
const QUAT_TRANSFORM_GET_VIEWER_STRINGS: u32 = 0x00a3_8cb0;
const MATRIX_SAVE_BINARY: u32 = 0x00a5_9a90;
const MATRIX_LOAD_BINARY: u32 = 0x00a5_9a30;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator; the
/// class's size.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const OBJECT_SIZE: u32 = 0x58;
/// `NiBlendAccumTransformInterpolator::ms_RTTI` (its first word is the
/// class name).
const RTTI: u32 = 0x011f_371c;

// Translated from 00a391c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::operator==` (Xbox PDB name from the engine map;
/// `thiscall (this, &other)`): the three translation floats equal (x87
/// compares: a NaN is never equal), the rotations equal (`00a92660`) and the
/// scales equal. Answers 1 or 0 in the whole of `EAX`.
pub fn ni_quat_transform_operator_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (this, other) = (this.addr(), other.addr());
    let equal = |e: &Engine, offset: u32| {
        x87_load32(e.mem.u32(this + offset)) == x87_load32(e.mem.u32(other + offset))
    };
    if !equal(e, 0) || !equal(e, 4) || !equal(e, 8) {
        return false;
    }
    if !e
        .call(QUATERNION_EQUAL, &args![this + ROTATE, other + ROTATE])
        .bool()
    {
        return false;
    }
    equal(e, SCALE)
}

// Translated from 00a39230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiBlendAccumTransformInterpolator` default constructor (unnamed in
/// the engine map; the factory's tail jump): the `NiBlendInterpolator`
/// default constructor, the class's vtable, a fresh accumulated transform
/// (translation and rotation copied, the scale through the FPU), no
/// accumulation array and `m_bReset` clear. Answers `this`.
pub fn fn_00a39230(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCT_DEFAULT, &args![this]);
    let object = this.addr();
    e.mem.set_u32(object, VTABLE);
    copy_words(e, FRESH_TRANSLATE, object + ACCUMULATED, 3);
    copy_words(e, FRESH_ROTATE, object + ACCUMULATED + ROTATE, 4);
    let scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
    e.mem.set_u32(object + ACCUMULATED + SCALE, scale);
    e.mem.set_u32(object + ACCUM_ARRAY, 0);
    e.mem.set_u8(object + RESET, 0);
    this
}

// Translated from 00a394b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::GetValue` (Xbox PDB): false when the
/// accumulated transform is all invalid (scale, rotation float +0x10 and
/// translation x at `-FLT_MAX`). Otherwise its eight words copied to
/// `value`, or with exactly one interpolator those of the accumulated
/// transform applied onto the single accumulation item's last value
/// (`HierApply` into a local); true.
pub fn ni_blend_accum_transform_interpolator_get_value(
    e: &mut Engine,
    this: Ptr,
    value: Ptr,
) -> bool {
    let accumulated = this.addr() + ACCUMULATED;
    if all_invalid(e, accumulated) {
        return false;
    }
    if byte(e, this, INTERP_COUNT) == 1 {
        let last = accum_item(e, this, u32::from(byte(e, this, SINGLE_INDEX))) + LAST_VALUE;
        combine_into(e, HIER_APPLY, accumulated, last, value.addr());
    } else {
        copy_words(e, accumulated, value.addr(), 8);
    }
    true
}

// Translated from 00a39700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (registered through `NiStream::RegisterLoader` at
/// `00a2d8ad`; Ghidra made no function of it): 0x58 bytes from the Ni
/// allocator and, when that succeeds, the default constructor
/// (`fn_00a39230`, a tail jump). Null when the allocation fails.
pub fn fn_00a39700(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    fn_00a39230(e, Ptr::new(object))
}

// Translated from 00a39720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::SaveBinary` (Xbox PDB): the base
/// `SaveBinary`, then each accumulation item's last value and delta value
/// (`NiQuatTransform::SaveBinary`) and reference frame
/// (`NiMatrix3::SaveBinary`), then the accumulated transform.
pub fn ni_blend_accum_transform_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let item = accum_item(e, this, u32::from(i));
        e.call(
            QUAT_TRANSFORM_SAVE_BINARY,
            &args![item + LAST_VALUE, stream],
        );
        e.call(
            QUAT_TRANSFORM_SAVE_BINARY,
            &args![item + DELTA_VALUE, stream],
        );
        e.call(MATRIX_SAVE_BINARY, &args![item + REF_FRAME, stream]);
        i = i.wrapping_add(1);
    }
    e.call(
        QUAT_TRANSFORM_SAVE_BINARY,
        &args![this.addr() + ACCUMULATED, stream],
    );
}

// Translated from 00a39780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::IsEqual` (Xbox PDB): the base
/// `IsEqual`, then the accumulated transforms equal
/// (`ni_quat_transform_operator_equal`).
pub fn ni_blend_accum_transform_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    ni_quat_transform_operator_equal(
        e,
        Ptr::new(this.addr() + ACCUMULATED),
        Ptr::new(other.addr() + ACCUMULATED),
    )
}

/// A new accumulation array of `size` items (`size * 0x68` bytes from the
/// memory manager, all ones on overflow; each item constructed by
/// `00a362f0`), or 0 when the allocation fails.
fn new_accum_array(e: &mut Engine, size: u32) -> u32 {
    let bytes = size.checked_mul(ACCUM_ITEM_SIZE).unwrap_or(u32::MAX);
    let array = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    if array == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![array, ACCUM_ITEM_SIZE, size, ACCUM_ITEM_CONSTRUCTOR],
    );
    array
}

// Translated from 00a3a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`NiBlendAccumTransformInterpolator`'s member
/// copy, `thiscall (this, clone, cloning)`): the `NiBlendInterpolator`
/// member copy, a new accumulation array for the clone (of this one's array
/// size) with every item copied word for word, then the accumulated
/// transform copied.
pub fn fn_00a3a450(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning]);
    let size = u32::from(byte(e, this, ARRAY_SIZE));
    let array = new_accum_array(e, size);
    e.mem.set_u32(clone.addr().wrapping_add(ACCUM_ARRAY), array);
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let from = accum_item(e, this, u32::from(i));
        let to = accum_item(e, clone, u32::from(i));
        copy_words(e, from, to, 0x1a);
        i = i.wrapping_add(1);
    }
    copy_words(
        e,
        this.addr() + ACCUMULATED,
        clone.addr().wrapping_add(ACCUMULATED),
        8,
    );
}

// Translated from 00a3a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::LoadBinary` (Xbox PDB): the base
/// `LoadBinary`, a new accumulation array of the loaded array size, each
/// item's last value and delta value (`NiQuatTransform::LoadBinary`) and
/// reference frame (`NiMatrix3::LoadBinary`), then the accumulated
/// transform.
pub fn ni_blend_accum_transform_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let size = u32::from(byte(e, this, ARRAY_SIZE));
    let array = new_accum_array(e, size);
    e.mem.set_u32(this.addr() + ACCUM_ARRAY, array);
    let mut i = 0u8;
    while i < byte(e, this, ARRAY_SIZE) {
        let item = accum_item(e, this, u32::from(i));
        e.call(
            QUAT_TRANSFORM_LOAD_BINARY,
            &args![item + LAST_VALUE, stream],
        );
        e.call(
            QUAT_TRANSFORM_LOAD_BINARY,
            &args![item + DELTA_VALUE, stream],
        );
        e.call(MATRIX_LOAD_BINARY, &args![item + REF_FRAME, stream]);
        i = i.wrapping_add(1);
    }
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![this.addr() + ACCUMULATED, stream],
    );
}

// Translated from 00a3a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::CreateClone` (Xbox PDB): 0x58 bytes
/// from the Ni allocator, the default constructor (`fn_00a39230`) and the
/// member copy (`fn_00a3a450`) into it; when the allocation fails the copy
/// still runs, into address 0, and 0 is answered.
pub fn ni_blend_accum_transform_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        fn_00a3a450(e, this, Ptr::NULL, cloning);
        return Ptr::NULL;
    }
    let clone = fn_00a39230(e, Ptr::new(object));
    fn_00a3a450(e, this, clone, cloning);
    clone
}

// Translated from 00a3a6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendAccumTransformInterpolator::GetViewerStrings` (Xbox PDB): the
/// base strings, the class name (through the argument's stack word), then
/// the accumulated transform's strings (`NiQuatTransform::GetViewerStrings`).
pub fn ni_blend_accum_transform_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    use crate::units::nianimation::niblendinterpolator::append_viewer_string;
    e.call(BASE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(RTTI);
    let text = e.call(0x00a7_d330, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        append_viewer_string(e, strings, slot.addr(), text);
    });
    e.call(
        QUAT_TRANSFORM_GET_VIEWER_STRINGS,
        &args![this.addr() + ACCUMULATED, strings],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a39180, fn_00a39180(Ptr) -> bool),
        entry!(0x00a3f990, fn_00a3f990(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00a39290,
            ni_blend_accum_transform_interpolator_destructor(Ptr)
        ),
        entry!(
            0x00a392b0,
            ni_blend_accum_transform_interpolator_add_interp_info(Ptr, Ptr, f32, u32, f32) -> u8
        ),
        entry!(
            0x00a393c0,
            ni_blend_accum_transform_interpolator_remove_interp_info(Ptr, u32)
        ),
        entry!(
            0x00a39ec0,
            ni_blend_accum_transform_interpolator_reallocate_arrays(Ptr) -> bool
        ),
        entry!(0x00a39530, fn_00a39530(Ptr, f32, Ptr)),
        entry!(
            0x00a397b0,
            ni_blend_accum_transform_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a39f60, fn_00a39f60(Ptr, u32, f32, Ptr)),
        entry!(0x00a39960, fn_00a39960(Ptr, u32, u32, Ptr) -> bool),
        entry!(
            0x00a3a580,
            ni_blend_accum_transform_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(0x00a3_91c0, ni_quat_transform_operator_equal(Ptr, Ptr) -> bool),
        entry!(0x00a3_9230, fn_00a39230(Ptr) -> Ptr),
        entry!(
            0x00a3_94b0,
            ni_blend_accum_transform_interpolator_get_value(Ptr, Ptr) -> bool
        ),
        entry!(0x00a3_9700, fn_00a39700() -> Ptr),
        entry!(
            0x00a3_9720,
            ni_blend_accum_transform_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a3_9780,
            ni_blend_accum_transform_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(0x00a3_a450, fn_00a3a450(Ptr, Ptr, Ptr)),
        entry!(
            0x00a3_a4f0,
            ni_blend_accum_transform_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a3_a660,
            ni_blend_accum_transform_interpolator_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a3_a6b0,
            ni_blend_accum_transform_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const M: f32 = f32::MIN;
    const CHILD_VTABLE: u32 = 0x7000_0000;
    const CHILD_UPDATE: u32 = 0x7200_008c;
    const CHILD_RANGE: u32 = 0x7200_00c0;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Doubles for every callee: transforms are 8 floats; the product and
    /// `HierApply` add component-wise, the inverses negate, the constructor
    /// fills -FLT_MAX, the matrix doubles the point; quaternions: the unit
    /// inverse and `operator-` negate, the product copies `this`, scaling
    /// multiplies, `ToRotation` writes 7.0 in the matrix. A child's `Update`
    /// writes (10 * time, 0, 0, 1, 0, 0, 0, 2) unless its answer word at +0x30
    /// is 0; its range is (0, 1).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0102_4000u32,
            0x0109_6000,
            0x0101_5000,
            0x0101_1000,
            0x0107_1000,
            0x011f_4000,
            0x011a_8000,
            0x011a_9000,
            0x011f_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(LOWEST, -3.4028234663852886e38f64);
        e.mem.set_f32(INVALID_FLOAT, M);
        e.mem.set_f32(INVALID_SCALE, M);
        e.mem.set_f32(INVALID_TRANSFORM_PART, M);
        e.mem.set_f32(SMALL_WEIGHT, 1e-5);
        e.mem.set_u8(INVALID_INDEX, 0xff);
        set_floats(&mut e, IDENTITY_ROTATE, &[1.0, 0.0, 0.0, 0.0]);
        set_floats(
            &mut e,
            IDENTITY_MATRIX,
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        );
        set_floats(&mut e, FRESH_TRANSLATE, &[M, M, M]);
        set_floats(&mut e, FRESH_ROTATE, &[M, M, M, M]);
        e.register(HIER_APPLY, add);
        e.register(TRANSFORM_PRODUCT, add);
        e.register(HIER_INVERT, |e, a| {
            negate(e, a[0], a[1], 8);
            Ret::default()
        });
        e.register(TRANSFORM_INVERSE, |e, a| {
            negate(e, a[0], a[1], 8);
            Ret::default()
        });
        e.register(TRANSFORM_CONSTRUCTOR, |e, a| {
            for i in 0..8 {
                e.mem.set_f32(a[0] + 4 * i, M);
            }
            Ret::default()
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for i in 0..3 {
                let v = 2.0 * e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(QUATERNION_UNIT_INVERSE, |e, a| {
            negate(e, a[1], a[0], 4);
            a[0].into_ret()
        });
        e.register(QUATERNION_NEGATE, |e, a| {
            negate(e, a[0], a[1], 4);
            a[1].into_ret()
        });
        e.register(QUATERNION_PRODUCT, |e, a| {
            for i in 0..4 {
                let v = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(QUATERNION_SCALE, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i) * s;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(QUATERNION_CONSTRUCTOR, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        e.register(QUATERNION_TO_ROTATION, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            Ret::default()
        });
        let mut slots = vec![0u32; 0xc4 / 4];
        slots[0x8c / 4] = CHILD_UPDATE;
        slots[0xc0 / 4] = CHILD_RANGE;
        e.put_vtable(CHILD_VTABLE, &slots);
        e.register(CHILD_UPDATE, |e, a| {
            let t = f32::from_bits(a[1]);
            set_floats(e, a[3], &[10.0 * t, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0]);
            (e.mem.u32(a[0] + 0x30) != 0).into_ret()
        });
        e.register(CHILD_RANGE, |e, a| {
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        e
    }

    fn add(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..8 {
            let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
            e.mem.set_f32(a[1] + 4 * i, v);
        }
        a[1].into_ret()
    }

    fn negate(e: &mut Engine, from: u32, to: u32, n: u32) {
        for i in 0..n {
            let v = -e.mem.f32(from + 4 * i);
            e.mem.set_f32(to + 4 * i, v);
        }
    }

    fn child(e: &mut Engine) -> u32 {
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child, CHILD_VTABLE);
        e.mem.set_u32(child + 0x30, 1);
        child
    }

    /// An interpolator with `size` items (children in `children`), its
    /// accumulated transform unset.
    fn accum(e: &mut Engine, children: &[u32]) -> Ptr {
        let this: Ptr = Ptr::new(e.mem.alloc(0x58));
        let size = children.len() as u32;
        let items = e.mem.alloc(ITEM_SIZE * size + 4);
        let accums = e.mem.alloc(ACCUM_ITEM_SIZE * size + 4);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(items + ITEM_SIZE * i as u32, *c);
            reset_accum_item(e, accums + ACCUM_ITEM_SIZE * i as u32);
        }
        e.mem.set_u32(this.addr() + INTERP_ARRAY, items);
        e.mem.set_u32(this.addr() + ACCUM_ARRAY, accums);
        e.mem.set_u8(this.addr() + ARRAY_SIZE, size as u8);
        set_floats(e, this.addr() + ACCUMULATED, &[M, M, M, M, M, M, M, M]);
        this
    }

    #[test]
    fn all_invalid_test() {
        let mut e = engine();
        let t = e.mem.alloc(0x20);
        set_floats(&mut e, t, &[M, 0.0, 0.0, 0.0, M, 0.0, 0.0, M]);
        assert!(e.call(0x00a39180, &args![t]).bool());
        e.mem.set_f32(t, 1.0);
        assert!(!e.call(0x00a39180, &args![t]).bool());
    }

    #[test]
    fn object_fields_or_zeros() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out, 9);
        assert_eq!(
            e.call(0x00a3f990, &args![this, out, out + 4, out + 8])
                .u32(),
            0
        );
        assert_eq!(
            (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u8(out + 8)),
            (0, 0, 0)
        );
        let object = e.mem.alloc(0x28);
        e.mem.set_u16(object + 0xa, 0xbeef);
        e.mem.set_u32(object + 0x14, 0x1234);
        e.mem.set_u8(object + 0x1d, 5);
        e.mem.set_u32(object + 0x24, 0x7777);
        e.mem.set_u32(this + 0x2c, object);
        assert_eq!(
            e.call(0x00a3f990, &args![this, out, out + 4, out + 8])
                .u32(),
            0x7777
        );
        assert_eq!(
            (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u8(out + 8)),
            (0xbeef, 0x1234, 5)
        );
    }

    #[test]
    fn destructor_frees_the_accumulation_array() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        let this = accum(&mut e, &[0]);
        let array = e.mem.u32(this.addr() + ACCUM_ARRAY);
        e.call_log = Some(vec![]);
        e.call(0x00a39290, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, array]]
        );
        assert_eq!(calls(&e, BASE_DESTRUCTOR), vec![vec![this.addr()]]);
    }

    #[test]
    fn add_interp_info_resets_and_folds_the_old_single() {
        let mut e = engine();
        let this = accum(&mut e, &[0, 0]);
        // The base answers the invalid index: nothing more.
        e.register(BASE_ADD_INTERP_INFO, |_, _| 0xffu32.into_ret());
        {
            let at = accum_item(&e, this, 0);
            set_floats(&mut e, at, &[3.0]);
        }
        assert_eq!(
            e.call(0x00a392b0, &args![this, 0u32, 1.0f32, 2u32, 1.0f32])
                .u8(),
            0xff
        );
        assert_eq!(e.mem.f32(accum_item(&e, this, 0)), 3.0);
        // First interpolator: index 0, reset requested.
        e.register(BASE_ADD_INTERP_INFO, |e, a| {
            e.mem.set_u8(a[0] + INTERP_COUNT, 1);
            0u32.into_ret()
        });
        assert_eq!(
            e.call(0x00a392b0, &args![this, 0u32, 1.0f32, 2u32, 1.0f32])
                .u8(),
            0
        );
        assert_eq!(e.mem.f32(accum_item(&e, this, 0)), M);
        assert_eq!(e.mem.f32(accum_item(&e, this, 0) + REF_FRAME), 1.0);
        assert_eq!(e.mem.u8(this.addr() + RESET), 1);
        // Second: the old single's last value folds into the accumulation.
        set_floats(
            &mut e,
            this.addr() + ACCUMULATED,
            &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        let old = accum_item(&e, this, 0);
        set_floats(
            &mut e,
            old + LAST_VALUE,
            &[2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        e.register(BASE_ADD_INTERP_INFO, |e, a| {
            e.mem.set_u8(a[0] + INTERP_COUNT, 2);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a392b0, &args![this, 0u32, 1.0f32, 2u32, 1.0f32])
                .u8(),
            1
        );
        assert_eq!(calls(&e, HIER_APPLY).len(), 1);
        assert_eq!(e.mem.f32(this.addr() + ACCUMULATED), 3.0);
        assert_eq!(e.mem.f32(old + LAST_VALUE), M);
    }

    #[test]
    fn remove_interp_info_folds_the_single_and_resets_the_new_one() {
        let mut e = engine();
        let a = child(&mut e);
        let b = child(&mut e);
        let this = accum(&mut e, &[a, b]);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 1);
        e.mem.set_u8(this.addr() + SINGLE_INDEX, 0);
        set_floats(
            &mut e,
            this.addr() + ACCUMULATED,
            &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        {
            let at = accum_item(&e, this, 0) + LAST_VALUE;
            set_floats(&mut e, at, &[2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        }
        e.register(BASE_REMOVE_INTERP_INFO, |e, a| {
            e.mem.set_u8(a[0] + SINGLE_INDEX, 1);
            Ret::default()
        });
        {
            let at = accum_item(&e, this, 1);
            set_floats(&mut e, at, &[5.0]);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a393c0, &args![this, 0x1234_5600u32]);
        assert_eq!(
            calls(&e, BASE_REMOVE_INTERP_INFO),
            vec![vec![this.addr(), 0x1234_5600]]
        );
        assert_eq!(e.mem.f32(this.addr() + ACCUMULATED), 3.0);
        assert_eq!(e.mem.f32(accum_item(&e, this, 1)), M);
        assert_eq!(e.mem.u8(this.addr() + RESET), 1);
    }

    #[test]
    fn reallocate_arrays_grows_the_accumulation_array() {
        let mut e = engine();
        let this = accum(&mut e, &[0]);
        e.register(BASE_REALLOCATE_ARRAYS, |_, _| false.into_ret());
        assert!(!e.call(0x00a39ec0, &args![this]).bool());
        e.register(BASE_REALLOCATE_ARRAYS, |e, a| {
            e.mem.set_u8(a[0] + ARRAY_SIZE, 4);
            true.into_ret()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        let old = e.mem.u32(this.addr() + ACCUM_ARRAY);
        e.mem.set_f32(old, 6.5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a39ec0, &args![this]).bool());
        let new = e.mem.u32(this.addr() + ACCUM_ARRAY);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 4 * 0x68]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![new, 0x68, 4, ACCUM_ITEM_CONSTRUCTOR]]
        );
        assert_eq!(e.mem.f32(new), 6.5);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER, old]]
        );
    }

    #[test]
    fn single_step_updates_the_last_value_and_folds_a_loop() {
        let mut e = engine();
        let a = child(&mut e);
        let this = accum(&mut e, &[a]);
        let item = accum_item(&e, this, 0);
        // Same time: only the last time.
        e.mem.set_f32(item, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x00a39530, &args![this, 0.5f32, 0u32]);
        assert!(calls(&e, CHILD_UPDATE).is_empty());
        // Forward: the last value at the new time.
        e.call(0x00a39530, &args![this, 0.75f32, 0u32]);
        assert_eq!(e.mem.f32(item + LAST_VALUE), 7.5);
        assert_eq!(e.mem.f32(item), 0.75);
        // Backward with the delta unset: delta = end + (-begin) = 10 + 0.
        set_floats(
            &mut e,
            this.addr() + ACCUMULATED,
            &[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        );
        e.call_log = Some(vec![]);
        e.call(0x00a39530, &args![this, 0.25f32, 0u32]);
        assert_eq!(calls(&e, CHILD_RANGE).len(), 1);
        assert_eq!(e.mem.f32(item + DELTA_VALUE), 10.0);
        assert_eq!(e.mem.f32(this.addr() + ACCUMULATED), 11.0);
        assert_eq!(e.mem.f32(item + LAST_VALUE), 2.5);
        assert_eq!(e.mem.f32(item), 0.25);
    }

    #[test]
    fn store_single_value_rebases_on_reset() {
        let mut e = engine();
        let a = child(&mut e);
        let this = accum(&mut e, &[a]);
        // The invalid time: false.
        let out = e.mem.alloc(0x20);
        assert!(!e.call(0x00a397b0, &args![this, M, 0u32, out]).bool());
        // First run: accumulated = -last (2.0 at time 0.2) then out = accumulated + last.
        assert!(e.call(0x00a397b0, &args![this, 0.2f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this.addr() + ACCUMULATED), -2.0);
        assert_eq!(e.mem.f32(out), 0.0);
        assert_eq!(e.mem.u8(this.addr() + RESET), 0);
    }

    #[test]
    fn item_step_identity_without_interpolator_and_deltas_otherwise() {
        let mut e = engine();
        let a = child(&mut e);
        let this = accum(&mut e, &[0, a]);
        // No interpolator: identity delta.
        e.call(0x00a39f60, &args![this, 0u32, 0.5f32, 0u32]);
        let item0 = accum_item(&e, this, 0);
        assert_eq!(
            floats(&e, item0 + DELTA_VALUE, 8),
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(e.mem.f32(item0), 0.5);
        // Forward from an unset last value: last at 0.5 (5.0), delta = -5 + 5.
        let item = accum_item(&e, this, 1);
        e.mem.set_f32(item, 0.0);
        e.call(0x00a39f60, &args![this, 0xabcd_ef01u32, 0.5f32, 0u32]);
        assert_eq!(e.mem.f32(item + REF_FRAME), 7.0);
        assert_eq!(e.mem.f32(item + DELTA_VALUE), 0.0);
        assert_eq!(e.mem.f32(item + LAST_VALUE), 5.0);
        assert_eq!(e.mem.f32(item), 0.5);
        // Forward again: delta = -5 + 7.5, turned by the frame (doubled).
        e.call(0x00a39f60, &args![this, 1u32, 0.75f32, 0u32]);
        assert_eq!(e.mem.f32(item + DELTA_VALUE), 5.0);
        assert_eq!(e.mem.f32(item + LAST_VALUE), 7.5);
        // Backward: a loop through the range.
        e.call_log = Some(vec![]);
        e.call(0x00a39f60, &args![this, 1u32, 0.25f32, 0u32]);
        assert_eq!(calls(&e, CHILD_RANGE).len(), 1);
        assert_eq!(e.mem.f32(item + LAST_VALUE), 2.5);
        assert_eq!(e.mem.f32(item), 0.25);
        assert_eq!(e.mem.u8(this.addr() + RESET), 0);
    }

    #[test]
    fn blend_weights_the_deltas() {
        let mut e = engine();
        let a = child(&mut e);
        let b = child(&mut e);
        let this = accum(&mut e, &[a, b]);
        let items = e.mem.u32(this.addr() + INTERP_ARRAY);
        e.mem.set_f32(items + ITEM_NORMALIZED_WEIGHT, 0.25);
        e.mem
            .set_f32(items + ITEM_SIZE + ITEM_NORMALIZED_WEIGHT, 0.75);
        // The accumulation is unset: the last values are blended.
        {
            let at = accum_item(&e, this, 0) + LAST_VALUE;
            set_floats(&mut e, at, &[4.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0]);
        }
        {
            let at = accum_item(&e, this, 1) + LAST_VALUE;
            set_floats(&mut e, at, &[8.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 4.0]);
        }
        let out = e.mem.alloc(0x20);
        assert!(e.call(0x00a39960, &args![this, 0u32, 0u32, out]).bool());
        // Identity (0, 0, 0, 1, 0, 0, 0, 1) plus the blend (7, 0, 0, 1, 0, 0, 0, 3.5).
        assert_eq!(floats(&e, out, 8), [7.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 4.5]);
        // Nothing to blend: false.
        let this = accum(&mut e, &[a]);
        assert!(!e.call(0x00a39960, &args![this, 0u32, 0u32, out]).bool());
    }

    #[test]
    fn update_by_interpolator_count() {
        let mut e = engine();
        e.register(BASE_COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let a = child(&mut e);
        let b = child(&mut e);
        let this = accum(&mut e, &[a, b]);
        let out = e.mem.alloc(0x20);
        assert!(!e.call(0x00a3a580, &args![this, 0.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 0.5);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 2);
        let items = e.mem.u32(this.addr() + INTERP_ARRAY);
        e.mem.set_f32(items + ITEM_NORMALIZED_WEIGHT, 1.0);
        e.call_log = Some(vec![]);
        e.call(0x00a3a580, &args![this, 0.75f32, 0u32, out]);
        assert_eq!(calls(&e, BASE_COMPUTE_NORMALIZED_WEIGHTS).len(), 1);
        // Only the weighted item stepped: its last time is the new time.
        assert_eq!(e.mem.f32(accum_item(&e, this, 0)), 0.75);
        assert_eq!(e.mem.f32(accum_item(&e, this, 1)), M);
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 0.75);
        // One: the single value.
        e.mem.set_u8(this.addr() + INTERP_COUNT, 1);
        assert!(e.call(0x00a3a580, &args![this, 0.25f32, 0u32, out]).bool());
    }
}

#[cfg(test)]
mod a07_tests {
    use super::*;
    use crate::units::nianimation::niblendinterpolator::a07_test_support::{calls, viewer_strings};

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0102_4000u32, 0x0109_6000, 0x011a_8000, 0x011f_3000] {
            e.map(page, 0x1000);
        }
        e.set_global(LOWEST, f64::from(f32::MIN));
        e.mem.set_f32(INVALID_SCALE, 1.0);
        for i in 0..3 {
            e.mem.set_f32(FRESH_TRANSLATE + 4 * i, 0.0);
        }
        e.mem.set_f32(FRESH_ROTATE, 1.0);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e
    }

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// An accumulating interpolator with `size` accumulation items (and base
    /// items), its accumulated transform `accumulated`.
    fn accum(e: &mut Engine, size: u8, accumulated: [f32; 8]) -> Ptr {
        let this = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u8(this + ARRAY_SIZE, size);
        let items = e.mem.alloc(ITEM_SIZE * u32::from(size).max(1));
        e.mem.set_u32(this + INTERP_ARRAY, items);
        let array = e.mem.alloc(ACCUM_ITEM_SIZE * u32::from(size).max(1));
        e.mem.set_u32(this + ACCUM_ARRAY, array);
        set_floats(e, this + ACCUMULATED, &accumulated);
        Ptr::new(this)
    }

    #[test]
    fn transform_equality_compares_each_part() {
        let mut e = engine();
        e.register(QUATERNION_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let (a, b) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        let values = [1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        set_floats(&mut e, a, &values);
        set_floats(&mut e, b, &values);
        let equal = |e: &mut Engine| e.call(0x00a3_91c0, &args![a, b]);
        assert_eq!(equal(&mut e).u32(), 1);
        e.mem.set_f32(b + 0xc, 0.5);
        assert_eq!(equal(&mut e).u32(), 0, "rotation");
        e.mem.set_f32(b + 0xc, 1.0);
        e.mem.set_f32(b + 0x8, f32::NAN);
        e.mem.set_f32(a + 0x8, f32::NAN);
        assert_eq!(equal(&mut e).u32(), 0, "NaN");
        e.mem.set_f32(a + 0x8, 3.0);
        e.mem.set_f32(b + 0x8, 3.0);
        e.mem.set_f32(b + SCALE, 2.0);
        assert_eq!(equal(&mut e).u32(), 0, "scale");
    }

    #[test]
    fn constructor_and_factory_start_fresh() {
        let mut e = engine();
        e.register(BASE_CONSTRUCT_DEFAULT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let object = e.call(0x00a3_9700, &args![]).u32();
        assert_eq!(e.mem.u32(object), VTABLE);
        assert_eq!(e.mem.f32(object + ACCUMULATED + ROTATE), 1.0);
        assert_eq!(e.mem.f32(object + ACCUMULATED + SCALE), 1.0);
        assert_eq!(e.mem.u32(object + ACCUM_ARRAY), 0);
        assert_eq!(e.mem.u8(object + RESET), 0);
        let other = e.mem.alloc(OBJECT_SIZE);
        e.mem.set_u8(other + RESET, 1);
        assert_eq!(e.call(0x00a3_9230, &args![other]).u32(), other);
        assert_eq!(e.mem.u8(other + RESET), 0);
        e.register(NI_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a3_9700, &args![]).u32(), 0);
    }

    #[test]
    fn get_value_copies_or_applies_the_single_item() {
        let mut e = engine();
        e.register(HIER_APPLY, |e, a| {
            for i in 0..8 {
                let sum = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, sum);
            }
            a[1].into_ret()
        });
        let m = f32::MIN;
        let this = accum(&mut e, 2, [m, 0.0, 0.0, 0.0, m, 0.0, 0.0, m]);
        let value = e.mem.alloc(0x20);
        assert!(!e.call(0x00a3_94b0, &args![this, value]).bool());
        let this = accum(&mut e, 2, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert!(e.call(0x00a3_94b0, &args![this, value]).bool());
        assert_eq!(e.mem.f32(value + 0x1c), 8.0);
        e.mem.set_u8(this.addr() + INTERP_COUNT, 1);
        e.mem.set_u8(this.addr() + SINGLE_INDEX, 1);
        let last = accum_item(&e, this, 1) + LAST_VALUE;
        set_floats(&mut e, last, &[1.0; 8]);
        assert!(e.call(0x00a3_94b0, &args![this, value]).bool());
        assert_eq!(e.mem.f32(value), 2.0);
        assert_eq!(e.mem.f32(value + 0x1c), 9.0);
    }

    #[test]
    fn save_and_load_binary_walk_the_accumulation_items() {
        let mut e = engine();
        for f in [
            BASE_SAVE_BINARY,
            BASE_LOAD_BINARY,
            QUAT_TRANSFORM_SAVE_BINARY,
            QUAT_TRANSFORM_LOAD_BINARY,
            MATRIX_SAVE_BINARY,
            MATRIX_LOAD_BINARY,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        let this = accum(&mut e, 2, [0.0; 8]);
        let stream = 0x4000u32;
        e.call_log = Some(vec![]);
        e.call(0x00a3_9720, &args![this, stream]);
        let item = accum_item(&e, this, 1);
        let saves = calls(&e, QUAT_TRANSFORM_SAVE_BINARY);
        assert_eq!(saves.len(), 5);
        assert_eq!(saves[2], vec![item + LAST_VALUE, stream]);
        assert_eq!(saves[3], vec![item + DELTA_VALUE, stream]);
        assert_eq!(saves[4], vec![this.addr() + ACCUMULATED, stream]);
        assert_eq!(
            calls(&e, MATRIX_SAVE_BINARY)[1],
            vec![item + REF_FRAME, stream]
        );
        e.call_log = Some(vec![]);
        e.call(0x00a3_a4f0, &args![this, stream]);
        let array = e.mem.u32(this.addr() + ACCUM_ARRAY);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![array, ACCUM_ITEM_SIZE, 2, ACCUM_ITEM_CONSTRUCTOR]]
        );
        let loads = calls(&e, QUAT_TRANSFORM_LOAD_BINARY);
        assert_eq!(loads.len(), 5);
        assert_eq!(loads[0], vec![array + LAST_VALUE, stream]);
        assert_eq!(calls(&e, MATRIX_LOAD_BINARY).len(), 2);
    }

    #[test]
    fn is_equal_needs_the_base_and_the_accumulated_transform() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(QUATERNION_EQUAL, |_, _| 1u32.into_ret());
        let this = accum(&mut e, 0, [1.0; 8]);
        let other = accum(&mut e, 0, [1.0; 8]);
        assert!(e.call(0x00a3_9780, &args![this, other]).bool());
        e.mem.set_f32(other.addr() + ACCUMULATED, 2.0);
        assert!(!e.call(0x00a3_9780, &args![this, other]).bool());
        e.register(BASE_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a3_9780, &args![this, this]).bool());
    }

    #[test]
    fn copy_members_and_create_clone_copy_the_items_and_the_transform() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        e.register(BASE_CONSTRUCT_DEFAULT, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = accum(&mut e, 2, [3.0; 8]);
        let item = accum_item(&e, this, 1);
        e.mem.set_u32(item + 0x64, 0xabcd);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a3_a660, &args![this, 0x77u32]).u32();
        assert_eq!(
            calls(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
        assert_eq!(e.mem.u32(clone), VTABLE);
        let copied = accum_item(&e, Ptr::new(clone), 1);
        assert_eq!(e.mem.u32(copied + 0x64), 0xabcd);
        assert_eq!(e.mem.f32(clone + ACCUMULATED + SCALE), 3.0);
        let target = accum(&mut e, 0, [0.0; 8]);
        e.call(0x00a3_a450, &args![this, target, 0u32]);
        assert_eq!(e.mem.f32(target.addr() + ACCUMULATED), 3.0);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_transform() {
        let mut e = engine();
        e.register(BASE_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_GET_VIEWER_STRINGS, |_, _| Ret::default());
        let (strings, stored) = viewer_strings(&mut e);
        e.mem.set_u32(RTTI, 0x0109_6100);
        let this = accum(&mut e, 0, [0.0; 8]);
        e.call_log = Some(vec![]);
        e.call(0x00a3_a6b0, &args![this, strings]);
        assert_eq!(*stored.borrow(), vec![0x0109_6100]);
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_GET_VIEWER_STRINGS),
            vec![vec![this.addr() + ACCUMULATED, strings.addr()]]
        );
    }
}
