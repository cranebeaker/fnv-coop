//! `NiAnimation/nimultitargettransformcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiMultiTargetTransformController::ms_RTTI` (the class's `NiRTTI` record).
const NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36bc;

// Translated from 00a2fbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_multi_target_transform_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI)
}

// Translated from 00a30280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a2fb70`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x40 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_multi_target_transform_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a2_fb70;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x40u32]);
    }
    this
}

layout! {
    /// `NiMultiTargetTransformController` (Xbox PDB), 0x40 bytes on PC as on
    /// the Xbox: an `NiInterpController` (an `NiTimeController`, 0x34 bytes)
    /// then the blend interpolators, their targets and their number.
    pub struct NiMultiTargetTransformController: 0x40 {
        /// `NiTimeController::m_uFlags` (Xbox PDB).
        0x08 m_uFlags: u16,
        /// `NiTimeController::m_fLoKeyTime` (Xbox PDB).
        0x14 m_fLoKeyTime: f32,
        /// `NiTimeController::m_fHiKeyTime` (Xbox PDB).
        0x18 m_fHiKeyTime: f32,
        /// `NiTimeController::m_pkTarget` (Xbox PDB).
        0x2C m_pkTarget: Ptr,
        /// `m_pkBlendInterps` (Xbox PDB): the array of `NiBlendTransformInterpolator`
        /// (0x30 bytes each; its element count sits in the word before it).
        0x34 m_pkBlendInterps: Ptr,
        /// `m_ppkTargets` (Xbox PDB): `NiAVObject *` per interpolator.
        0x38 m_ppkTargets: Ptr,
        /// `m_usNumInterps` (Xbox PDB).
        0x3C m_usNumInterps: u16,
    }
}

/// Size of one `NiBlendTransformInterpolator` (the elements of `m_pkBlendInterps`).
const BLEND_INTERPOLATOR_SIZE: u32 = 0x30;
/// Size of the `NiTransformInterpolator` that `CreatePoseInterpolator` makes.
const TRANSFORM_INTERPOLATOR_SIZE: u32 = 0x48;
/// `NiMultiTargetTransformController`'s vtable (stored by its constructors).
const NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE: u32 = 0x0109_6444;
/// The memory manager singleton (`this` of `Allocate` and `Deallocate`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`this` = the manager, `(size)`) and `Deallocate` (`(block)`).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The game's `operator new` (`(size)`), the allocator the interpolator array uses (`(size)`),
/// the table allocator of the targets (`(size)`), and the free of that table (`(block)`), the
/// sized `operator delete` (`(block, size)`) and the array delete (`(block, size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const ARRAY_NEW: u32 = 0x00aa_1420;
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const ARRAY_DELETE: u32 = 0x00aa_1490;
/// The C runtime's `_memset` (`(dest, value, size)`) and
/// `_vector_constructor_iterator_` (`(array, element size, count, constructor)`).
const MEMSET: u32 = 0x00ec_61c0;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The constructor of one `NiBlendTransformInterpolator` that the iterator runs.
const BLEND_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a4_09b0;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiBlendInterpolator`'s destructor body.
const BLEND_INTERPOLATOR_DESTRUCT: u32 = 0x00a3_7170;
/// `NiInterpController::ResetTimeExtrema`, `LoadBinary`, `GetViewerStrings` (Xbox PDB names) and its constructor.
const INTERP_CONTROLLER_RESET_TIME_EXTREMA: u32 = 0x00a2_f8f0;
const INTERP_CONTROLLER_LOAD_BINARY: u32 = 0x00a2_f910;
const INTERP_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a2_f980;
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
/// `NiTimeController`'s destructor body, `ProcessClone`, the member copy (through the jump at
/// `00a2f8a0`; `this` = source, `(destination, process)`), `IsEqual` (the body `NiPSysResetOnLoopCtlr`
/// shares) and `SaveBinary` (through the jump at `00c27120`).
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a2_f8a0;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00c2_83f0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00c2_7120;
/// The blend interpolator's member copy, reached through the jump at `00a40aa0`
/// (`this` = source, `(destination, process)`).
const BLEND_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a4_0aa0;
/// `NiTransformInterpolator`'s constructor (`this`, then the `NiQuatTransform` by value: 8 words).
const TRANSFORM_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_fae0;
/// `NiQuaternion::FromRotation` (Xbox PDB; `this` = the quaternion, `(matrix)`) and
/// `NiQuaternion::ToRotation` (Xbox PDB; `this` = the quaternion, `(matrix)`).
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
/// The cloning-process map lookup (`this` = the map, `(key, &out)`, answers whether found).
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `NiStream::ResolveLinkID` (Xbox PDB; `this` = the stream).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// `NiGetViewerString` (Xbox PDB; `(text)`) and `NiGetViewerString_ov6` (Xbox PDB; `(name, value)`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_OV6: u32 = 0x00a7_d490;
/// The viewer-string array's growth (`this` = array, `(new size)`) and store (`(index, &text)`).
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_usNumInterps"`.
const TEXT_NUM_INTERPS: u32 = 0x0109_652c;
/// The float `FLT_MAX` (`0109643c`) and `-FLT_MAX` (`01015f5c`) that `ResetTimeExtrema` starts
/// from, and the `double` constants `FLT_MAX` (`010231b0`) and `-FLT_MAX` (`010241b0`) it
/// compares with.
const FLT_MAX_FLOAT: u32 = 0x0109_643c;
const NEGATIVE_FLT_MAX_FLOAT: u32 = 0x0101_5f5c;
const FLT_MAX_DOUBLE: u32 = 0x0102_31b0;
const NEGATIVE_FLT_MAX_DOUBLE: u32 = 0x0102_41b0;
/// The float `-FLT_MAX` that `Update` starts the scale of its `NiQuatTransform` with.
const NEGATIVE_FLT_MAX_SCALE: u32 = 0x0109_6b8c;
/// The three words (an `NiPoint3` of invalid values) and four words (the invalid quaternion) that
/// `Update` starts the translate and rotate of its `NiQuatTransform` with (statics in the exe's
/// data; which Xbox PDB statics they are is not confirmed).
const INVALID_TRANSLATE_011A8400: u32 = 0x011a_8400;
const INVALID_ROTATE_011F3704: u32 = 0x011f_3704;
/// The 16-bit "no index" answer of `GetInterpolatorIndex` and `GetInterpolatorIndexFx`.
const INVALID_INDEX: u32 = 0x0109_6320;
/// The `NiRTTI` of `NiAVObject`, which `TargetIsRequiredType` looks for in the target's class chain.
const NI_AV_OBJECT_RTTI: u32 = 0x011f_4280;
/// Virtual slots of the interpolator (`NiInterpolator`, Xbox PDB names): `Update` (0x8c, the
/// overload that takes a transform), `GetActiveTimeRange` (0xc0), `GuaranteeTimeRange` (0xc4); of
/// `NiObject`: `ProcessClone` (0x68), `IsEqual` (0x5c), `GetRTTI` (0x08); of `NiStream`: `SaveLinkID` (0x30).
const SLOT_INTERPOLATOR_UPDATE: u32 = 0x8c;
const SLOT_GET_ACTIVE_TIME_RANGE: u32 = 0xc0;
const SLOT_GUARANTEE_TIME_RANGE: u32 = 0xc4;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_GET_RTTI: u32 = 0x08;
const SLOT_SAVE_LINK_ID: u32 = 0x30;

/// Address of element `index` of the interpolator array at `interpolators`.
fn interpolator_at(interpolators: u32, index: u32) -> u32 {
    interpolators.wrapping_add(index.wrapping_mul(BLEND_INTERPOLATOR_SIZE))
}

/// The `float` the exe copies with `FLD`/`FSTP` from the word at `source`: its bits, a signalling NaN quieted.
fn quiet_word(e: &Engine, source: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(source))
}

/// Copies `count` words from `source` to `destination`.
fn copy_words(e: &mut Engine, source: u32, destination: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// `strcmp(a, b) == 0` as the exe's inlined loop does it: compares the two strings
/// byte by byte, stopping at the first difference or at the common end.
fn same_text(e: &Engine, a: u32, b: u32) -> bool {
    let mut offset = 0u32;
    loop {
        let left = e.mem.u8(a.wrapping_add(offset));
        let right = e.mem.u8(b.wrapping_add(offset));
        if left != right {
            return false;
        }
        if left == 0 {
            return true;
        }
        offset = offset.wrapping_add(1);
    }
}

/// Appends `text` to the viewer-string array `strings` (the inline sequence of the
/// `GetViewerStrings` functions): when the 16-bit size (`+0xA`) has reached the capacity (`+8`) the
/// array is grown to the step (`+0xE`) plus the size first, then the string is stored at the old size.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// Makes the array of `count` `NiBlendTransformInterpolator`s the way the inline sequence of
/// `00a2feb0` and `00a30160` does: a block of the count word plus the elements (the size all
/// ones when `count * 0x30 + 4` overflows; a `count` of 0, which neither caller reaches, asks
/// the memory manager for 4 bytes), the count stored in the first word and the constructor run
/// on every element (`_vector_constructor_iterator_`). Answers the address of the first
/// element, 0 when the allocation answers 0.
fn new_blend_interpolators(e: &mut Engine, count: u32) -> u32 {
    let block = if count == 0 {
        e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 4u32]).u32()
    } else {
        let (product, overflow) = count.overflowing_mul(BLEND_INTERPOLATOR_SIZE);
        let size = if overflow { u32::MAX } else { product };
        let (sum, carry) = size.overflowing_add(4);
        let size = if carry { u32::MAX } else { sum };
        e.call(ARRAY_NEW, &args![size]).u32()
    };
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            block + 4,
            BLEND_INTERPOLATOR_SIZE,
            count,
            BLEND_INTERPOLATOR_CONSTRUCTOR
        ],
    );
    block + 4
}

// Translated from 00a2fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetInterpolatorCount` (Xbox PDB): the 16-bit
/// interpolator count at `+0x3c`.
pub fn ni_multi_target_transform_controller_get_interpolator_count(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
) -> u16 {
    e.get(this, NiMultiTargetTransformController::m_usNumInterps)
}

// Translated from 00a2fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetInterpolator` (Xbox PDB): the address of
/// element `index` (0x30 bytes each) of the interpolator array at `+0x34`.
pub fn ni_multi_target_transform_controller_get_interpolator(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    index: u16,
) -> Ptr {
    let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
    Ptr::new(interpolator_at(interpolators.addr(), u32::from(index)))
}

// Translated from 00a2fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::CreateBlendInterpolator` (Xbox PDB): answers the
/// address of element `index` of the interpolator array at `+0x34`, the same as
/// `GetInterpolator` (`RET 0x14`: the four words after the index are never read).
pub fn ni_multi_target_transform_controller_create_blend_interpolator(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    index: u16,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> Ptr {
    let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
    Ptr::new(interpolator_at(interpolators.addr(), u32::from(index)))
}

// Translated from 00a2fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GuaranteeTimeRange` (Xbox PDB): runs the
/// interpolator's virtual slot 0xc4 (`GuaranteeTimeRange`) on each of the `+0x3c`
/// interpolators, in order, with the two floats (copied through the x87 stack, so a
/// signalling NaN goes on quieted).
pub fn ni_multi_target_transform_controller_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    start: f32,
    end: f32,
) {
    if e.get(this, NiMultiTargetTransformController::m_usNumInterps) == 0 {
        return;
    }
    let mut index = 0u16;
    loop {
        let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
        let interpolator = interpolator_at(interpolators.addr(), u32::from(index));
        e.vcall(
            interpolator,
            SLOT_GUARANTEE_TIME_RANGE,
            &args![x87_quiet(start), x87_quiet(end)],
        );
        index = index.wrapping_add(1);
        if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
            break;
        }
    }
}

// Translated from 00a2fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::IsEqual` (Xbox PDB): false unless the time-controller
/// comparison (`00c283f0`, the body `NiPSysResetOnLoopCtlr::IsEqual` shares) holds and the
/// counts are equal; then for each index the two targets must be both null or both set, and
/// for two set ones the other's `IsEqual` (virtual slot 0x5c) must answer true for this one's.
pub fn ni_multi_target_transform_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    other: Ptr<NiMultiTargetTransformController>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.get(this, NiMultiTargetTransformController::m_usNumInterps);
    if e.get(other, NiMultiTargetTransformController::m_usNumInterps) != count {
        return false;
    }
    let mut index = 0u16;
    if 0 < count {
        loop {
            let offset = u32::from(index) * 4;
            let ours = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
            let target = e.mem.u32(ours.addr().wrapping_add(offset));
            let theirs = e.get(other, NiMultiTargetTransformController::m_ppkTargets);
            let other_target = e.mem.u32(theirs.addr().wrapping_add(offset));
            if target == 0 {
                if other_target != 0 {
                    return false;
                }
            } else {
                if other_target == 0 {
                    return false;
                }
                if !e.vcall(other_target, SLOT_IS_EQUAL, &args![target]).bool() {
                    return false;
                }
            }
            index = index.wrapping_add(1);
            if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                break;
            }
        }
    }
    true
}

// Translated from 00a2fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::~NiMultiTargetTransformController` (Xbox PDB; the
/// engine map has no name here, the scalar deleting destructor `00a30280` calls this body):
/// stores the class vtable, then deletes the interpolator array (`+0x34`; the vector
/// deleting destructor, virtual slot 0 of its first element with flags 3, when its count
/// word is not 0, else the block is given back to the memory manager), frees the
/// target table (`+0x38`) and runs the `NiTimeController` destructor body (`00a6d290`).
pub fn ni_multi_target_transform_controller_destructor(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
) {
    let interpolators = e
        .get(this, NiMultiTargetTransformController::m_pkBlendInterps)
        .addr();
    e.mem
        .set_u32(this.addr(), NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE);
    if interpolators != 0 {
        if e.mem.u32(interpolators - 4) != 0 {
            e.vcall(interpolators, 0, &args![3u32]);
        } else {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, interpolators - 4]);
        }
    }
    let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
    e.call(TABLE_FREE, &args![targets]);
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00a2fbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::Update` (Xbox PDB): when bit 3 of the flags byte at
/// `+8` is set, builds an invalid `NiQuatTransform` (0x20 bytes: translate, rotate as
/// `w, x, y, z`, scale) and, for each interpolator whose target is set and has bit 1 of its
/// flags (`+0x30`) set, asks the interpolator (virtual slot 0x8c) to fill it for the time
/// in the first float of `update_data`, the target and the transform. When it answers true:
/// a translate that is not the invalid value is copied to the target's local translate
/// (`+0x58`), a rotation whose second word (+0x10 of the transform) is not the invalid
/// value goes into the target's local rotation matrix (`+0x34`) through
/// `NiQuaternion::ToRotation` (`004f0180`), and a scale that is not the invalid value is
/// stored as its absolute value at the target's local scale (`+0x64`). The compares are the
/// exe's: an unordered (NaN) value counts as "not the invalid value".
pub fn ni_multi_target_transform_controller_update(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    update_data: Ptr,
) {
    if e.mem.u8(this.addr() + 8) >> 3 & 1 == 0 {
        return;
    }
    e.with_stack(0x20, |e, transform| {
        let transform = transform.addr();
        copy_words(e, INVALID_TRANSLATE_011A8400, transform, 3);
        copy_words(e, INVALID_ROTATE_011F3704, transform + 0xc, 4);
        let invalid_scale = quiet_word(e, NEGATIVE_FLT_MAX_SCALE);
        e.mem.set_u32(transform + 0x1c, invalid_scale);
        let invalid_value = e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE);
        let mut index = 0u32;
        if e.get(this, NiMultiTargetTransformController::m_usNumInterps) != 0 {
            loop {
                let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
                let target = e.mem.u32(targets.addr().wrapping_add(index * 4));
                if target != 0 && (e.mem.u32(target + 0x30) >> 1) & 1 != 0 {
                    let interpolators =
                        e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
                    let interpolator = interpolator_at(interpolators.addr(), index);
                    let time = quiet_word(e, update_data.addr());
                    let filled = e
                        .vcall(
                            interpolator,
                            SLOT_INTERPOLATOR_UPDATE,
                            &args![time, target, transform],
                        )
                        .bool();
                    if filled {
                        let translate_x = x87_load32(e.mem.u32(transform));
                        if translate_x != invalid_value {
                            let targets =
                                e.get(this, NiMultiTargetTransformController::m_ppkTargets);
                            let target = e.mem.u32(targets.addr().wrapping_add(index * 4));
                            copy_words(e, transform, target + 0x58, 3);
                        }
                        let rotate_marker = x87_load32(e.mem.u32(transform + 0x10));
                        if rotate_marker != invalid_value {
                            let targets =
                                e.get(this, NiMultiTargetTransformController::m_ppkTargets);
                            let target = e.mem.u32(targets.addr().wrapping_add(index * 4));
                            e.call(
                                QUATERNION_TO_ROTATION,
                                &args![transform + 0xc, target + 0x34],
                            );
                        }
                        let scale = x87_load32(e.mem.u32(transform + 0x1c));
                        if scale != invalid_value {
                            let targets =
                                e.get(this, NiMultiTargetTransformController::m_ppkTargets);
                            let target = e.mem.u32(targets.addr().wrapping_add(index * 4));
                            let magnitude = quiet_word(e, transform + 0x1c) & 0x7fff_ffff;
                            e.mem.set_u32(target + 0x64, x87_quiet_bits(magnitude));
                        }
                    }
                }
                index += 1;
                if index >= u32::from(e.get(this, NiMultiTargetTransformController::m_usNumInterps))
                {
                    break;
                }
            }
        }
    });
}

// Translated from 00a2fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetInterpolatorID` (Xbox PDB): the word at `+8`
/// (the name) of target number `index`.
pub fn ni_multi_target_transform_controller_get_interpolator_id(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    index: u16,
) -> u32 {
    let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
    let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
    e.mem.u32(target + 8)
}

// Translated from 00a2fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetInterpolatorIndex` (Xbox PDB): the index of the
/// first non-null target whose name (the string pointer at `+8`) equals the string `name`;
/// the "no index" word `0xffff` (read from `01096320`) when `name` is null or none matches.
pub fn ni_multi_target_transform_controller_get_interpolator_index(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    name: u32,
) -> u16 {
    if name == 0 {
        return e.global::<u16>(INVALID_INDEX);
    }
    let count = e.get(this, NiMultiTargetTransformController::m_usNumInterps);
    let mut index = 0u16;
    if 0 < count {
        let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
        loop {
            let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
            if target != 0 {
                let target_name = e.mem.u32(target + 8);
                if same_text(e, target_name, name) {
                    return index;
                }
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    e.global::<u16>(INVALID_INDEX)
}

// Translated from 00a2fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::ResetTimeExtrema` (Xbox PDB): unless bit 5 of the flags
/// byte at `+8` is set, resets the key-time range (`+0x14`, `+0x18`) to `FLT_MAX` and
/// `-FLT_MAX`, widens it with each interpolator's active time range (virtual slot 0xc0,
/// `(&low, &high)`, both started at `FLT_MAX` and `-FLT_MAX`), and when the range is still
/// `FLT_MAX` and `-FLT_MAX` runs `NiInterpController::ResetTimeExtrema` (`00a2f8f0`, a tail
/// jump) on the controller.
pub fn ni_multi_target_transform_controller_reset_time_extrema(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
) {
    if e.mem.u8(this.addr() + 8) >> 5 & 1 != 0 {
        return;
    }
    let low_start = quiet_word(e, FLT_MAX_FLOAT);
    e.mem.set_u32(this.addr() + 0x14, low_start);
    let high_start = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
    e.mem.set_u32(this.addr() + 0x18, high_start);
    let mut index = 0u16;
    if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
        e.with_stack(8, |e, range| {
            let low = range.addr();
            let high = range.addr() + 4;
            loop {
                let negative_max = quiet_word(e, NEGATIVE_FLT_MAX_FLOAT);
                e.mem.set_u32(high, negative_max);
                let positive_max = quiet_word(e, FLT_MAX_FLOAT);
                e.mem.set_u32(low, positive_max);
                let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
                let interpolator = interpolator_at(interpolators.addr(), u32::from(index));
                e.vcall(interpolator, SLOT_GET_ACTIVE_TIME_RANGE, &args![low, high]);
                let new_low = x87_load32(e.mem.u32(low));
                let old_low = x87_load32(e.mem.u32(this.addr() + 0x14));
                if new_low < old_low {
                    let bits = quiet_word(e, low);
                    e.mem.set_u32(this.addr() + 0x14, bits);
                }
                let new_high = x87_load32(e.mem.u32(high));
                let old_high = x87_load32(e.mem.u32(this.addr() + 0x18));
                if old_high < new_high {
                    let bits = quiet_word(e, high);
                    e.mem.set_u32(this.addr() + 0x18, bits);
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                    break;
                }
            }
        });
    }
    let low_key = x87_load32(e.mem.u32(this.addr() + 0x14));
    if low_key == e.global::<f64>(FLT_MAX_DOUBLE) {
        let high_key = x87_load32(e.mem.u32(this.addr() + 0x18));
        if high_key == e.global::<f64>(NEGATIVE_FLT_MAX_DOUBLE) {
            e.call(INTERP_CONTROLLER_RESET_TIME_EXTREMA, &args![this]);
        }
    }
}

// Translated from 00a2feb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the interpolator count `count` at
/// `+0x3c`; when it is not 0, makes the array of that many `NiBlendTransformInterpolator`s (`+0x34`;
/// the count word before the elements, each constructed by `00a409b0`), adds one reference
/// (`InterlockedIncrement` of the reference count at `+4`) to every element, and makes the target
/// table (`+0x38`, `count * 4` bytes) zeroed.
pub fn fn_00a2feb0(e: &mut Engine, this: Ptr<NiMultiTargetTransformController>, count: u16) {
    e.set(
        this,
        NiMultiTargetTransformController::m_usNumInterps,
        count,
    );
    if count == 0 {
        return;
    }
    let count = u32::from(count);
    let interpolators = new_blend_interpolators(e, count);
    e.set(
        this,
        NiMultiTargetTransformController::m_pkBlendInterps,
        Ptr::new(interpolators),
    );
    let mut offset = 0u32;
    let mut remaining = count;
    loop {
        let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
        e.call(
            INTERLOCKED_INCREMENT,
            &args![offset.wrapping_add(interpolators.addr()).wrapping_add(4)],
        );
        offset += BLEND_INTERPOLATOR_SIZE;
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    let targets = e.call(TABLE_ALLOCATE, &args![count * 4]).u32();
    e.set(
        this,
        NiMultiTargetTransformController::m_ppkTargets,
        Ptr::new(targets),
    );
    e.call(MEMSET, &args![targets, 0u32, count * 4]);
}

// Translated from 00a2ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::_vector_deleting_destructor_` (Xbox PDB): with bit 1 of
/// `flags` (the low byte is tested) set, `this` is the first of an array whose count is the word
/// before it: each element is destroyed (`NiBlendInterpolator`'s destructor body, `00a37170`)
/// from the last to the first, the block (`this - 4`) is deleted (`00aa1490`, `(block, 0x30)`)
/// when bit 0 is set, and the block's address is answered. Without bit 1, the one object is
/// destroyed and deleted (`00aa1460`, `(this, 0x30)`) when bit 0 is set; answers `this`.
pub fn ni_blend_transform_interpolator_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let flags = flags as u8;
    if flags & 2 != 0 {
        let block = this.addr().wrapping_sub(4);
        let mut remaining = e.mem.u32(block) as i32;
        let mut element = this
            .addr()
            .wrapping_add((remaining as u32).wrapping_mul(BLEND_INTERPOLATOR_SIZE));
        remaining = remaining.wrapping_sub(1);
        while remaining >= 0 {
            element = element.wrapping_sub(BLEND_INTERPOLATOR_SIZE);
            e.call(BLEND_INTERPOLATOR_DESTRUCT, &args![element]);
            remaining = remaining.wrapping_sub(1);
        }
        if flags & 1 != 0 {
            e.call(ARRAY_DELETE, &args![block, BLEND_INTERPOLATOR_SIZE]);
        }
        Ptr::new(block)
    } else {
        e.call(BLEND_INTERPOLATOR_DESTRUCT, &args![this]);
        if flags & 1 != 0 {
            e.call(SIZED_DELETE, &args![this, BLEND_INTERPOLATOR_SIZE]);
        }
        this
    }
}

// Translated from 00a2ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::CreatePoseInterpolator` (Xbox PDB): takes target number
/// `index`, makes a quaternion from its local rotation matrix (`+0x34`, `NiQuaternion::FromRotation`
/// `00a6df40`) and builds an `NiQuatTransform` of its local translate (`+0x58`), that quaternion
/// and its local scale (`+0x64`, copied through the x87 stack). Allocates 0x48 bytes (`00aa13e0`)
/// and, when that gives memory, constructs an `NiTransformInterpolator` (`00a3fae0`, the
/// transform passed by value as 8 words) in it and answers it; otherwise answers 0.
pub fn ni_multi_target_transform_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    index: u16,
) -> Ptr {
    let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
    let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
    let rotation = e.with_stack(0x10, |e, quaternion| {
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, target + 0x34]);
        [
            e.mem.u32(quaternion.addr()),
            e.mem.u32(quaternion.addr() + 4),
            e.mem.u32(quaternion.addr() + 8),
            e.mem.u32(quaternion.addr() + 12),
        ]
    });
    let translate = [
        e.mem.u32(target + 0x58),
        e.mem.u32(target + 0x5c),
        e.mem.u32(target + 0x60),
    ];
    let scale = quiet_word(e, target + 0x64);
    let object = e
        .call(OPERATOR_NEW, &args![TRANSFORM_INTERPOLATOR_SIZE])
        .u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(
        TRANSFORM_INTERPOLATOR_CONSTRUCT,
        &args![
            object,
            translate[0],
            translate[1],
            translate[2],
            rotation[0],
            rotation[1],
            rotation[2],
            rotation[3],
            scale
        ],
    )
    .ptr()
}

// Translated from 00a30070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the controller's member copy for a clone,
/// `this` being the source. Runs the `NiTimeController` copy (`00a2f8a0` jump to `00a6cfe0`,
/// `(destination, process)`), makes the destination's interpolator array of the source's count
/// (`00a2feb0`), then copies each source interpolator into the destination's (`00a40aa0`, a jump to
/// `00a37510`, `this` = the source element, `(destination element, process)`).
pub fn fn_00a30070(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    destination: Ptr<NiMultiTargetTransformController>,
    process: Ptr,
) {
    e.call(
        TIME_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, process],
    );
    let count = e.get(this, NiMultiTargetTransformController::m_usNumInterps);
    fn_00a2feb0(e, destination, count);
    let mut index = 0u16;
    if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
        loop {
            let destination_interpolators = e.get(
                destination,
                NiMultiTargetTransformController::m_pkBlendInterps,
            );
            let source_interpolators =
                e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
            e.call(
                BLEND_INTERPOLATOR_COPY_MEMBERS,
                &args![
                    interpolator_at(source_interpolators.addr(), u32::from(index)),
                    interpolator_at(destination_interpolators.addr(), u32::from(index)),
                    process
                ],
            );
            index = index.wrapping_add(1);
            if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                break;
            }
        }
    }
}

// Translated from 00a300d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::ProcessClone` (Xbox PDB): runs
/// `NiTimeController::ProcessClone` (`00a6d0c0`), looks this controller up in the cloning
/// process's map (the map is the first word of `process`; `00a53c30`, the result word
/// starting at 0) to find its clone, and then for each interpolator runs the interpolator's
/// `ProcessClone` (virtual slot 0x68, with `process`); when its target is set, the target's
/// clone from the same map (or 0 when not found) is stored at the same index of the clone's
/// target table (`+0x38`). The lookup's result word is kept between targets, as in the exe.
pub fn ni_multi_target_transform_controller_process_clone(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    process: Ptr,
) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, found| {
        let map = e.mem.u32(process.addr());
        e.call(CLONE_MAP_GET_AT, &args![map, this, found]);
        let clone = e.mem.u32(found.addr());
        let mut index = 0u16;
        if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
            loop {
                let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
                let interpolator = interpolator_at(interpolators.addr(), u32::from(index));
                e.vcall(interpolator, SLOT_PROCESS_CLONE, &args![process]);
                let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
                let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
                if target != 0 {
                    let map = e.mem.u32(process.addr());
                    let has_clone = e.call(CLONE_MAP_GET_AT, &args![map, target, found]).bool();
                    let clone_targets = e.mem.u32(clone + 0x38);
                    let value = if has_clone {
                        e.mem.u32(found.addr())
                    } else {
                        0
                    };
                    e.mem
                        .set_u32(clone_targets.wrapping_add(u32::from(index) * 4), value);
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                    break;
                }
            }
        }
    });
}

// Translated from 00a30160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fastcall`, no stack words): when the count (`+0x3c`) and the
/// interpolator array (`+0x34`) are both set, deletes the array (the vector deleting
/// destructor, virtual slot 0 of the first element with flags 3, when its count word is not 0,
/// else the block goes back to the memory manager), makes a new array of the same count
/// as `00a2feb0` does (constructed elements with one added reference each) and zeroes the
/// target table (`+0x38`, `count * 4` bytes), which it keeps.
pub fn fn_00a30160(e: &mut Engine, this: Ptr<NiMultiTargetTransformController>) {
    if e.get(this, NiMultiTargetTransformController::m_usNumInterps) == 0 {
        return;
    }
    let interpolators = e
        .get(this, NiMultiTargetTransformController::m_pkBlendInterps)
        .addr();
    if interpolators == 0 {
        return;
    }
    if e.mem.u32(interpolators - 4) == 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, interpolators - 4]);
    } else {
        e.vcall(interpolators, 0, &args![3u32]);
    }
    let count = u32::from(e.get(this, NiMultiTargetTransformController::m_usNumInterps));
    let new_interpolators = new_blend_interpolators(e, count);
    e.set(
        this,
        NiMultiTargetTransformController::m_pkBlendInterps,
        Ptr::new(new_interpolators),
    );
    let mut index = 0u16;
    if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
        loop {
            let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
            e.call(
                INTERLOCKED_INCREMENT,
                &args![interpolator_at(interpolators.addr(), u32::from(index)) + 4],
            );
            index = index.wrapping_add(1);
            if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                break;
            }
        }
    }
    let size = u32::from(e.get(this, NiMultiTargetTransformController::m_usNumInterps)) * 4;
    let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
    e.call(MEMSET, &args![targets, 0u32, size]);
}

// Translated from 00a30240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetInterpolatorIndexFx` (Xbox PDB): the index of the first
/// non-null target whose word at `+8` equals the word `id` points to; the "no index" word
/// `0xffff` (read from `01096320`) when none does.
pub fn ni_multi_target_transform_controller_get_interpolator_index_fx(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    id: Ptr,
) -> u16 {
    let count = e.get(this, NiMultiTargetTransformController::m_usNumInterps);
    let mut index = 0u16;
    if 0 < count {
        let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
        loop {
            let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
            if target != 0 && e.mem.u32(target + 8) == e.mem.u32(id.addr()) {
                return index;
            }
            index = index.wrapping_add(1);
            if index >= count {
                break;
            }
        }
    }
    e.global::<u16>(INVALID_INDEX)
}

// Translated from 00a302a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::TargetIsRequiredType` (Xbox PDB): whether the target
/// (`+0x2c`) is an `NiAVObject`: false for no target, else its `GetRTTI` (virtual slot 8)
/// chain of parent records (`+4`) is walked for the `NiAVObject` record (`011f4280`).
pub fn ni_multi_target_transform_controller_target_is_required_type(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
) -> bool {
    let target = e.get(this, NiMultiTargetTransformController::m_pkTarget);
    if target.is_null() {
        return false;
    }
    let mut rtti = e.vcall(target.addr(), SLOT_GET_RTTI, &args![]).u32();
    while rtti != 0 {
        if rtti == NI_AV_OBJECT_RTTI {
            return true;
        }
        rtti = e.mem.u32(rtti + 4);
    }
    false
}

// Translated from 00a302d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::CreateClone` (Xbox PDB): allocates 0x40 bytes
/// (`00aa13e0`); when that gives memory, runs the `NiInterpController` constructor
/// (`00a2f8c0`), stores this class's vtable and clears the interpolators, targets and count
/// (`+0x34`, `+0x38`, `+0x3c`); then runs the member copy `00a30070` from this controller
/// into it (into address 0 when there was no memory, as the exe does) and answers the clone.
pub fn ni_multi_target_transform_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    process: Ptr,
) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![0x40u32]).u32();
    if object == 0 {
        fn_00a30070(e, this, Ptr::NULL, process);
        return Ptr::NULL;
    }
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    let clone = Ptr::<NiMultiTargetTransformController>::new(object);
    e.mem
        .set_u32(object, NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE);
    e.set(
        clone,
        NiMultiTargetTransformController::m_pkBlendInterps,
        Ptr::NULL,
    );
    e.set(
        clone,
        NiMultiTargetTransformController::m_ppkTargets,
        Ptr::NULL,
    );
    e.set(
        clone,
        NiMultiTargetTransformController::m_usNumInterps,
        0u16,
    );
    fn_00a30070(e, this, clone, process);
    clone.cast()
}

// Translated from 00a303d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::LoadBinary` (Xbox PDB): runs
/// `NiInterpController::LoadBinary` (`00a2f910`), reads the 16-bit count into `+0x3c` through
/// the stream's input stream (`function(input, destination, 2, &sizes, 1)` with the sizes
/// word set to 2, the returned byte count added to the stream position), makes the arrays
/// for it (`00a2feb0`) and stores into each target slot the object that
/// `NiStream::ResolveLinkID` (`00a64620`) answers, one per interpolator.
pub fn ni_multi_target_transform_controller_load_binary(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    stream: Ptr<NiStream>,
) {
    e.call(INTERP_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 2);
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        let done = e
            .call(
                function,
                &args![input, this.addr() + 0x3c, 2u32, sizes, 1u32],
            )
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
    let count = e.get(this, NiMultiTargetTransformController::m_usNumInterps);
    fn_00a2feb0(e, this, count);
    let mut index = 0u16;
    if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
        loop {
            let link = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
            let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
            e.mem
                .set_u32(targets.addr().wrapping_add(u32::from(index) * 4), link);
            index = index.wrapping_add(1);
            if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                break;
            }
        }
    }
}

// Translated from 00a30440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::SaveBinary` (Xbox PDB): runs the `NiTimeController`
/// `SaveBinary` (`00c27120`, a jump to `00a6d730`), writes the 16-bit count at `+0x3c`
/// through the stream's output stream (`function(output, source, 2, &sizes, 1)`, sizes word
/// 2, the byte count added to its position) and then the stream's `SaveLinkID` (virtual slot
/// 0x30) for each target, in order.
pub fn ni_multi_target_transform_controller_save_binary(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    stream: Ptr<NiStream>,
) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 2);
        let function = e.get(output, NiBinaryStream::m_pfnWrite);
        let done = e
            .call(
                function,
                &args![output, this.addr() + 0x3c, 2u32, sizes, 1u32],
            )
            .u32();
        let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            output,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
    let mut index = 0u16;
    if 0 < e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
        loop {
            let targets = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
            let target = e.mem.u32(targets.addr().wrapping_add(u32::from(index) * 4));
            e.vcall(stream.addr(), SLOT_SAVE_LINK_ID, &args![target]);
            index = index.wrapping_add(1);
            if index >= e.get(this, NiMultiTargetTransformController::m_usNumInterps) {
                break;
            }
        }
    }
}

// Translated from 00a304b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMultiTargetTransformController::GetViewerStrings` (Xbox PDB): runs
/// `NiInterpController::GetViewerStrings` (`00a2f980`) on `strings`, then appends two strings:
/// the class name (the first word of the `NiRTTI` record at `011f36bc`, through
/// `NiGetViewerString`) and `"m_usNumInterps"` with the 16-bit count (through
/// `NiGetViewerString_ov6`); each append grows the array first when its size has reached its
/// capacity.
pub fn ni_multi_target_transform_controller_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiMultiTargetTransformController>,
    strings: Ptr,
) {
    e.call(INTERP_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, strings.addr(), text);
    let count = u32::from(e.get(this, NiMultiTargetTransformController::m_usNumInterps));
    let text = e
        .call(GET_VIEWER_STRING_OV6, &args![TEXT_NUM_INTERPS, count])
        .u32();
    append_viewer_string(e, strings.addr(), text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2fbd0, ni_multi_target_transform_controller_get_rtti() -> Ptr),
        entry!(0x00a30280, ni_multi_target_transform_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a2fa20,
            ni_multi_target_transform_controller_get_interpolator_count(
                Ptr<NiMultiTargetTransformController>,
            ) -> u16
        ),
        entry!(
            0x00a2fa30,
            ni_multi_target_transform_controller_get_interpolator(
                Ptr<NiMultiTargetTransformController>,
                u16,
            ) -> Ptr
        ),
        entry!(
            0x00a2fa50,
            ni_multi_target_transform_controller_create_blend_interpolator(
                Ptr<NiMultiTargetTransformController>,
                u16,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr
        ),
        entry!(
            0x00a2fa70,
            ni_multi_target_transform_controller_guarantee_time_range(
                Ptr<NiMultiTargetTransformController>,
                f32,
                f32,
            )
        ),
        entry!(
            0x00a2fae0,
            ni_multi_target_transform_controller_is_equal(
                Ptr<NiMultiTargetTransformController>,
                Ptr<NiMultiTargetTransformController>,
            ) -> bool
        ),
        entry!(
            0x00a2fb70,
            ni_multi_target_transform_controller_destructor(Ptr<NiMultiTargetTransformController>)
        ),
        entry!(
            0x00a2fbe0,
            ni_multi_target_transform_controller_update(Ptr<NiMultiTargetTransformController>, Ptr)
        ),
        entry!(
            0x00a2fd40,
            ni_multi_target_transform_controller_get_interpolator_id(
                Ptr<NiMultiTargetTransformController>,
                u16,
            ) -> u32
        ),
        entry!(
            0x00a2fd60,
            ni_multi_target_transform_controller_get_interpolator_index(
                Ptr<NiMultiTargetTransformController>,
                u32,
            ) -> u16
        ),
        entry!(
            0x00a2fde0,
            ni_multi_target_transform_controller_reset_time_extrema(
                Ptr<NiMultiTargetTransformController>,
            )
        ),
        entry!(
            0x00a2feb0,
            fn_00a2feb0(Ptr<NiMultiTargetTransformController>, u16)
        ),
        entry!(0x00a2ff70, ni_blend_transform_interpolator_vector_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a2ffe0,
            ni_multi_target_transform_controller_create_pose_interpolator(
                Ptr<NiMultiTargetTransformController>,
                u16,
            ) -> Ptr
        ),
        entry!(
            0x00a30070,
            fn_00a30070(
                Ptr<NiMultiTargetTransformController>,
                Ptr<NiMultiTargetTransformController>,
                Ptr,
            )
        ),
        entry!(
            0x00a300d0,
            ni_multi_target_transform_controller_process_clone(
                Ptr<NiMultiTargetTransformController>,
                Ptr,
            )
        ),
        entry!(
            0x00a30160,
            fn_00a30160(Ptr<NiMultiTargetTransformController>)
        ),
        entry!(
            0x00a30240,
            ni_multi_target_transform_controller_get_interpolator_index_fx(
                Ptr<NiMultiTargetTransformController>,
                Ptr,
            ) -> u16
        ),
        entry!(
            0x00a302a0,
            ni_multi_target_transform_controller_target_is_required_type(
                Ptr<NiMultiTargetTransformController>,
            ) -> bool
        ),
        entry!(
            0x00a302d0,
            ni_multi_target_transform_controller_create_clone(
                Ptr<NiMultiTargetTransformController>,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00a303d0,
            ni_multi_target_transform_controller_load_binary(
                Ptr<NiMultiTargetTransformController>,
                Ptr<NiStream>,
            )
        ),
        entry!(
            0x00a30440,
            ni_multi_target_transform_controller_save_binary(
                Ptr<NiMultiTargetTransformController>,
                Ptr<NiStream>,
            )
        ),
        entry!(
            0x00a304b0,
            ni_multi_target_transform_controller_get_viewer_strings(
                Ptr<NiMultiTargetTransformController>,
                Ptr,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<Vec<u32>>;

    fn calls_to(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The words of a vtable whose slot at byte offset `slot` is `target`.
    fn vtable_slots(slot: u32, target: u32) -> Vec<u32> {
        let mut slots = vec![0u32; (slot / 4 + 1) as usize];
        slots[(slot / 4) as usize] = target;
        slots
    }

    /// A zeroed 0x100-byte object whose vtable is `vtable`.
    fn object_with_vtable(e: &mut Engine, vtable: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A controller with `targets` in its target table and a matching count.
    fn controller_with_targets(
        e: &mut Engine,
        targets: &[u32],
    ) -> Ptr<NiMultiTargetTransformController> {
        let this = e.new_object::<NiMultiTargetTransformController>();
        let table = e.mem.alloc(4 * targets.len() as u32 + 4);
        for (index, target) in targets.iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *target);
        }
        e.set(
            this,
            NiMultiTargetTransformController::m_ppkTargets,
            Ptr::new(table),
        );
        e.set(
            this,
            NiMultiTargetTransformController::m_usNumInterps,
            targets.len() as u16,
        );
        this
    }

    /// A controller with `count` interpolators: a block of the count word and 0x30-byte
    /// elements whose first word is `vtable`, and a zeroed target table. Answers the first
    /// element and the table.
    fn controller_with_interpolators(
        e: &mut Engine,
        count: u16,
        vtable: u32,
    ) -> (Ptr<NiMultiTargetTransformController>, u32, u32) {
        let this = e.new_object::<NiMultiTargetTransformController>();
        let block = e.mem.alloc(4 + 0x30 * u32::from(count));
        e.mem.set_u32(block, u32::from(count));
        for index in 0..u32::from(count) {
            e.mem.set_u32(block + 4 + 0x30 * index, vtable);
        }
        let table = e.mem.alloc(4 * u32::from(count) + 4);
        e.set(
            this,
            NiMultiTargetTransformController::m_pkBlendInterps,
            Ptr::new(block + 4),
        );
        e.set(
            this,
            NiMultiTargetTransformController::m_ppkTargets,
            Ptr::new(table),
        );
        e.set(
            this,
            NiMultiTargetTransformController::m_usNumInterps,
            count,
        );
        (this, block + 4, table)
    }

    /// Doubles for the allocations and calls `new_blend_interpolators` and the loops around it make.
    fn allocation_doubles(e: &mut Engine) {
        e.register(ARRAY_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |_, _| Ret::default());
        e.register(MEMSET, |_, _| Ret::default());
    }

    #[test]
    fn get_interpolator_count_is_the_word_at_0x3c() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.set(this, NiMultiTargetTransformController::m_usNumInterps, 7u16);
        assert_eq!(e.call(0x00a2fa20, &args![this]).u16(), 7);
    }

    #[test]
    fn get_interpolator_scales_the_index_by_0x30() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.set(
            this,
            NiMultiTargetTransformController::m_pkBlendInterps,
            Ptr::new(0x4000),
        );
        assert_eq!(e.call(0x00a2fa30, &args![this, 3u16]).u32(), 0x4000 + 0x90);
        assert_eq!(e.call(0x00a2fa30, &args![this, 0u16]).u32(), 0x4000);
    }

    #[test]
    fn create_blend_interpolator_answers_the_same_element_whatever_the_other_words() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.set(
            this,
            NiMultiTargetTransformController::m_pkBlendInterps,
            Ptr::new(0x4000),
        );
        let answer = e.call(
            0x00a2fa50,
            &args![this, 2u16, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert_eq!(answer.u32(), 0x4000 + 0x60);
    }

    #[test]
    fn guarantee_time_range_runs_every_interpolator_with_quieted_floats() {
        const VTABLE: u32 = 0x0300_0000;
        const GUARANTEE: u32 = 0x0300_1000;
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &vtable_slots(SLOT_GUARANTEE_TIME_RANGE, GUARANTEE));
        e.register(GUARANTEE, |_, _| Ret::default());
        let (this, interpolators, _) = controller_with_interpolators(&mut e, 2, VTABLE);
        e.call_log = Some(vec![]);
        e.call(
            0x00a2fa70,
            &args![this, f32::from_bits(0x7f80_0001), 2.5f32],
        );
        assert_eq!(
            calls_to(&e, GUARANTEE),
            vec![
                vec![interpolators, 0x7fc0_0001, 2.5f32.to_bits()],
                vec![interpolators + 0x30, 0x7fc0_0001, 2.5f32.to_bits()],
            ]
        );
    }

    #[test]
    fn guarantee_time_range_does_nothing_without_interpolators() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a2fa70, &args![this, 1.0f32, 2.0f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    /// Two controllers whose targets are objects where `ours` and `theirs` say `true` (null where
    /// `false`); the time-controller comparison answers `parent` and each target comparison
    /// answers `target_answer`. Answers the result and the target comparisons made, as
    /// (the other side's target, this side's target) pairs.
    fn is_equal_case(
        parent: bool,
        ours: &[bool],
        theirs: &[bool],
        target_answer: bool,
    ) -> (bool, Calls) {
        const OTHER_VTABLE: u32 = 0x0300_0000;
        const TARGET_IS_EQUAL: u32 = 0x0300_1000;
        let mut e = Engine::new();
        e.put_vtable(OTHER_VTABLE, &vtable_slots(SLOT_IS_EQUAL, TARGET_IS_EQUAL));
        e.register_double(TIME_CONTROLLER_IS_EQUAL, move |_, _| parent.into_ret());
        e.register_double(TARGET_IS_EQUAL, move |_, _| target_answer.into_ret());
        let mut make = |present: &[bool]| -> Vec<u32> {
            present
                .iter()
                .map(|set| {
                    if *set {
                        object_with_vtable(&mut e, OTHER_VTABLE)
                    } else {
                        0
                    }
                })
                .collect()
        };
        let our_objects = make(ours);
        let their_objects = make(theirs);
        let this = controller_with_targets(&mut e, &our_objects);
        let other = controller_with_targets(&mut e, &their_objects);
        e.call_log = Some(vec![]);
        let answer = e.call(0x00a2fae0, &args![this, other]).bool();
        let compared = calls_to(&e, TARGET_IS_EQUAL);
        let expected_pairs: Calls = our_objects
            .iter()
            .zip(&their_objects)
            .filter(|(a, b)| **a != 0 && **b != 0)
            .map(|(a, b)| vec![*b, *a])
            .collect();
        assert!(compared.len() <= expected_pairs.len());
        assert_eq!(compared[..], expected_pairs[..compared.len()]);
        (answer, compared)
    }

    #[test]
    fn is_equal_is_false_when_the_time_controller_parts_differ() {
        assert!(!is_equal_case(false, &[false], &[false], true).0);
    }

    #[test]
    fn is_equal_is_false_when_the_counts_differ() {
        assert!(!is_equal_case(true, &[false, false], &[false], true).0);
    }

    #[test]
    fn is_equal_is_true_for_matching_null_and_equal_targets() {
        let (answer, compared) =
            is_equal_case(true, &[false, true, true], &[false, true, true], true);
        assert!(answer);
        assert_eq!(compared.len(), 2);
    }

    #[test]
    fn is_equal_is_true_for_two_empty_controllers() {
        assert!(is_equal_case(true, &[], &[], true).0);
    }

    #[test]
    fn is_equal_is_false_when_only_one_side_has_a_target() {
        assert!(!is_equal_case(true, &[false], &[true], true).0);
        assert!(!is_equal_case(true, &[true], &[false], true).0);
    }

    #[test]
    fn is_equal_is_false_when_a_target_comparison_fails() {
        let (answer, compared) = is_equal_case(true, &[true, true], &[true, true], false);
        assert!(!answer);
        assert_eq!(compared.len(), 1);
    }

    #[test]
    fn destructor_deletes_the_array_through_its_vector_destructor() {
        const VTABLE: u32 = 0x0300_0000;
        const VECTOR_DELETE: u32 = 0x0300_1000;
        let mut e = Engine::new();
        e.put_vtable(VTABLE, &vtable_slots(0, VECTOR_DELETE));
        e.register(VECTOR_DELETE, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (this, interpolators, table) = controller_with_interpolators(&mut e, 2, VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a2fb70, &args![this]);
        assert_eq!(
            e.mem.u32(this.addr()),
            NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE
        );
        assert_eq!(calls_to(&e, VECTOR_DELETE), vec![vec![interpolators, 3]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![table]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_DESTRUCT),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn destructor_gives_an_empty_array_block_back_to_the_memory_manager() {
        let mut e = Engine::new();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (this, interpolators, _) = controller_with_interpolators(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a2fb70, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, interpolators - 4]]
        );
    }

    #[test]
    fn destructor_without_an_array_only_frees_the_table_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| Ret::default());
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a2fb70, &args![this]);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![0]]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_DESTRUCT),
            vec![vec![this.addr()]]
        );
    }

    /// The globals `update` reads, with every "invalid" value `-FLT_MAX`.
    fn map_update_constants(e: &mut Engine) {
        let invalid = (-f32::MAX).to_bits();
        e.map(INVALID_TRANSLATE_011A8400, 12);
        for word in 0..3 {
            e.mem
                .set_u32(INVALID_TRANSLATE_011A8400 + 4 * word, invalid);
        }
        e.map(INVALID_ROTATE_011F3704, 16);
        for word in 0..4 {
            e.mem.set_u32(INVALID_ROTATE_011F3704 + 4 * word, invalid);
        }
        e.map(NEGATIVE_FLT_MAX_SCALE, 4);
        e.set_global(NEGATIVE_FLT_MAX_SCALE, -f32::MAX);
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        e.set_global(NEGATIVE_FLT_MAX_DOUBLE, -f64::from(f32::MAX));
    }

    const UPDATE_VTABLE: u32 = 0x0300_0000;
    const INTERPOLATOR_UPDATE: u32 = 0x0300_1000;

    /// A flagged controller with one interpolator and one target (flags `target_flags`);
    /// the target's local translate, rotation words and scale start as markers.
    fn update_fixture(
        e: &mut Engine,
        target_flags: u32,
    ) -> (Ptr<NiMultiTargetTransformController>, u32, u32) {
        map_update_constants(e);
        e.put_vtable(
            UPDATE_VTABLE,
            &vtable_slots(SLOT_INTERPOLATOR_UPDATE, INTERPOLATOR_UPDATE),
        );
        let (this, _, table) = controller_with_interpolators(e, 1, UPDATE_VTABLE);
        e.mem.set_u8(this.addr() + 8, 0x08);
        let target = e.mem.alloc(0x70);
        e.mem.set_u32(target + 0x30, target_flags);
        for word in 0..3 {
            e.mem.set_u32(target + 0x58 + 4 * word, 0xaaaa_0000 + word);
        }
        e.mem.set_u32(target + 0x64, 0xbbbb_0000);
        e.mem.set_u32(table, target);
        let update_data = e.mem.alloc(8);
        e.mem.set_f32(update_data, 1.25);
        (this, target, update_data)
    }

    #[test]
    fn update_does_nothing_when_the_active_flag_is_clear() {
        let mut e = Engine::new();
        let (this, _, update_data) = update_fixture(&mut e, 2);
        e.mem.set_u8(this.addr() + 8, 0x07);
        e.register(INTERPOLATOR_UPDATE, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert!(calls_to(&e, INTERPOLATOR_UPDATE).is_empty());
    }

    #[test]
    fn update_skips_null_targets_and_targets_without_the_flag() {
        let mut e = Engine::new();
        let (this, target, update_data) = update_fixture(&mut e, 1);
        e.register(INTERPOLATOR_UPDATE, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert!(calls_to(&e, INTERPOLATOR_UPDATE).is_empty());
        e.mem.set_u32(target + 0x30, 2);
        let table = e.get(this, NiMultiTargetTransformController::m_ppkTargets);
        e.mem.set_u32(table.addr(), 0);
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert!(calls_to(&e, INTERPOLATOR_UPDATE).is_empty());
    }

    #[test]
    fn update_copies_the_valid_parts_of_the_transform_to_the_target() {
        let mut e = Engine::new();
        let (this, target, update_data) = update_fixture(&mut e, 2);
        let seen_rotation = Rc::new(RefCell::new(Vec::new()));
        let seen = seen_rotation.clone();
        e.register_double(QUATERNION_TO_ROTATION, move |e, a| {
            for word in 0..4 {
                seen.borrow_mut().push(e.mem.u32(a[0] + 4 * word));
            }
            seen.borrow_mut().push(a[1]);
            Ret::default()
        });
        e.register(INTERPOLATOR_UPDATE, |e, a| {
            let transform = a[3];
            e.mem.set_f32(transform, 1.0);
            e.mem.set_f32(transform + 4, 2.0);
            e.mem.set_f32(transform + 8, 3.0);
            for (word, value) in [1.0f32, 0.5, 0.25, 0.125].iter().enumerate() {
                e.mem.set_f32(transform + 0xc + 4 * word as u32, *value);
            }
            e.mem.set_f32(transform + 0x1c, -2.0);
            true.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2fbe0, &args![this, update_data]);
        let calls = calls_to(&e, INTERPOLATOR_UPDATE);
        assert_eq!(calls.len(), 1);
        let interpolators = e.get(this, NiMultiTargetTransformController::m_pkBlendInterps);
        assert_eq!(
            calls[0][..3],
            [interpolators.addr(), 1.25f32.to_bits(), target]
        );
        assert_eq!(e.mem.f32(target + 0x58), 1.0);
        assert_eq!(e.mem.f32(target + 0x5c), 2.0);
        assert_eq!(e.mem.f32(target + 0x60), 3.0);
        assert_eq!(e.mem.f32(target + 0x64), 2.0);
        assert_eq!(
            *seen_rotation.borrow(),
            vec![
                1.0f32.to_bits(),
                0.5f32.to_bits(),
                0.25f32.to_bits(),
                0.125f32.to_bits(),
                target + 0x34
            ]
        );
    }

    #[test]
    fn update_leaves_the_target_alone_for_values_that_stay_invalid() {
        let mut e = Engine::new();
        let (this, target, update_data) = update_fixture(&mut e, 2);
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        e.register(INTERPOLATOR_UPDATE, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert_eq!(calls_to(&e, INTERPOLATOR_UPDATE).len(), 1);
        assert!(calls_to(&e, QUATERNION_TO_ROTATION).is_empty());
        assert_eq!(e.mem.u32(target + 0x58), 0xaaaa_0000);
        assert_eq!(e.mem.u32(target + 0x64), 0xbbbb_0000);
    }

    #[test]
    fn update_changes_nothing_when_the_interpolator_answers_false() {
        let mut e = Engine::new();
        let (this, target, update_data) = update_fixture(&mut e, 2);
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        e.register(INTERPOLATOR_UPDATE, |e, a| {
            e.mem.set_f32(a[3], 9.0);
            e.mem.set_f32(a[3] + 0x1c, 9.0);
            false.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert!(calls_to(&e, QUATERNION_TO_ROTATION).is_empty());
        assert_eq!(e.mem.u32(target + 0x58), 0xaaaa_0000);
        assert_eq!(e.mem.u32(target + 0x64), 0xbbbb_0000);
    }

    #[test]
    fn update_counts_a_nan_translate_and_scale_as_valid_and_clears_the_scale_sign() {
        let mut e = Engine::new();
        let (this, target, update_data) = update_fixture(&mut e, 2);
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        e.register(INTERPOLATOR_UPDATE, |e, a| {
            e.mem.set_u32(a[3], 0x7f80_0001);
            e.mem.set_u32(a[3] + 0x1c, 0xffc0_1234);
            true.into_ret()
        });
        e.call(0x00a2fbe0, &args![this, update_data]);
        assert_eq!(e.mem.u32(target + 0x58), 0x7f80_0001);
        assert_eq!(e.mem.u32(target + 0x64), 0x7fc0_1234);
    }

    #[test]
    fn get_interpolator_id_is_the_word_at_8_of_the_target() {
        let mut e = Engine::new();
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target + 8, 0x1234);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 8, 0x5678);
        let this = controller_with_targets(&mut e, &[target, other]);
        assert_eq!(e.call(0x00a2fd40, &args![this, 1u16]).u32(), 0x5678);
        assert_eq!(e.call(0x00a2fd40, &args![this, 0u16]).u32(), 0x1234);
    }

    #[test]
    fn get_interpolator_index_finds_the_first_target_with_that_name() {
        let mut e = Engine::new();
        e.map(INVALID_INDEX, 2);
        e.mem.set_u16(INVALID_INDEX, 0xffff);
        let names = [
            (b"Bip01\0".as_slice(), 0x6000),
            (b"Bip02\0".as_slice(), 0x6010),
        ];
        let mut targets = vec![0u32];
        for (name, at) in names {
            e.mem.alloc(1);
            e.mem.map(at, 0x10);
            e.mem.set_cstr(at, name);
            let target = e.mem.alloc(0x10);
            e.mem.set_u32(target + 8, at);
            targets.push(target);
        }
        let this = controller_with_targets(&mut e, &targets);
        e.mem.map(0x6100, 0x10);
        e.mem.set_cstr(0x6100, b"Bip02\0");
        assert_eq!(e.call(0x00a2fd60, &args![this, 0x6100u32]).u16(), 2);
        e.mem.set_cstr(0x6100, b"Bip01\0");
        assert_eq!(e.call(0x00a2fd60, &args![this, 0x6100u32]).u16(), 1);
        e.mem.set_cstr(0x6100, b"Bip0\0");
        assert_eq!(e.call(0x00a2fd60, &args![this, 0x6100u32]).u16(), 0xffff);
        assert_eq!(e.call(0x00a2fd60, &args![this, 0u32]).u16(), 0xffff);
    }

    /// The globals `reset_time_extrema` reads.
    fn map_extrema_constants(e: &mut Engine) {
        e.map(FLT_MAX_FLOAT, 4);
        e.set_global(FLT_MAX_FLOAT, f32::MAX);
        e.map(NEGATIVE_FLT_MAX_FLOAT, 4);
        e.set_global(NEGATIVE_FLT_MAX_FLOAT, -f32::MAX);
        e.map(FLT_MAX_DOUBLE, 8);
        e.set_global(FLT_MAX_DOUBLE, f64::from(f32::MAX));
        e.map(NEGATIVE_FLT_MAX_DOUBLE, 8);
        e.set_global(NEGATIVE_FLT_MAX_DOUBLE, -f64::from(f32::MAX));
    }

    const RANGE_VTABLE: u32 = 0x0300_0000;
    const ACTIVE_RANGE: u32 = 0x0300_1000;

    #[test]
    fn reset_time_extrema_widens_the_range_with_each_interpolator() {
        let mut e = Engine::new();
        map_extrema_constants(&mut e);
        e.put_vtable(
            RANGE_VTABLE,
            &vtable_slots(SLOT_GET_ACTIVE_TIME_RANGE, ACTIVE_RANGE),
        );
        let ranges = Rc::new(RefCell::new(vec![(2.0f32, 5.0f32), (1.0, 4.0)]));
        let queue = ranges.clone();
        e.register_double(ACTIVE_RANGE, move |e, a| {
            let (low, high) = queue.borrow_mut().remove(0);
            e.mem.set_f32(a[1], low);
            e.mem.set_f32(a[2], high);
            Ret::default()
        });
        e.register(INTERP_CONTROLLER_RESET_TIME_EXTREMA, |_, _| Ret::default());
        let (this, _, _) = controller_with_interpolators(&mut e, 2, RANGE_VTABLE);
        e.mem.set_f32(this.addr() + 0x14, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x00a2fde0, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x18), 5.0);
        assert!(calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA).is_empty());
    }

    #[test]
    fn reset_time_extrema_runs_the_base_when_the_range_stays_empty() {
        let mut e = Engine::new();
        map_extrema_constants(&mut e);
        e.put_vtable(
            RANGE_VTABLE,
            &vtable_slots(SLOT_GET_ACTIVE_TIME_RANGE, ACTIVE_RANGE),
        );
        e.register(ACTIVE_RANGE, |_, _| Ret::default());
        e.register(INTERP_CONTROLLER_RESET_TIME_EXTREMA, |_, _| Ret::default());
        let (this, _, _) = controller_with_interpolators(&mut e, 2, RANGE_VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a2fde0, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x14), f32::MAX);
        assert_eq!(e.mem.f32(this.addr() + 0x18), -f32::MAX);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA),
            vec![vec![this.addr()]]
        );
        let empty = e.new_object::<NiMultiTargetTransformController>();
        e.call(0x00a2fde0, &args![empty]);
        assert_eq!(calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA).len(), 2);
    }

    #[test]
    fn reset_time_extrema_does_nothing_when_the_manager_controls_the_range() {
        let mut e = Engine::new();
        map_extrema_constants(&mut e);
        e.register(INTERP_CONTROLLER_RESET_TIME_EXTREMA, |_, _| Ret::default());
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.mem.set_u8(this.addr() + 8, 0x20);
        e.mem.set_f32(this.addr() + 0x14, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x00a2fde0, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 3.0);
        assert!(calls_to(&e, INTERP_CONTROLLER_RESET_TIME_EXTREMA).is_empty());
    }

    #[test]
    fn fn_00a2feb0_makes_the_arrays_for_a_count() {
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a2feb0, &args![this, 3u16]);
        let interpolators = e
            .get(this, NiMultiTargetTransformController::m_pkBlendInterps)
            .addr();
        let table = e
            .get(this, NiMultiTargetTransformController::m_ppkTargets)
            .addr();
        assert_eq!(
            e.get(this, NiMultiTargetTransformController::m_usNumInterps),
            3
        );
        assert_eq!(e.mem.u32(interpolators - 4), 3);
        assert_eq!(calls_to(&e, ARRAY_NEW), vec![vec![0x30 * 3 + 4]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![interpolators, 0x30, 3, 0x00a4_09b0]]
        );
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![
                vec![interpolators + 4],
                vec![interpolators + 0x34],
                vec![interpolators + 0x64]
            ]
        );
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![12]]);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 12]]);
    }

    #[test]
    fn fn_00a2feb0_with_a_zero_count_only_stores_it() {
        let mut e = Engine::new();
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.set(this, NiMultiTargetTransformController::m_usNumInterps, 9u16);
        e.call_log = Some(vec![]);
        e.call(0x00a2feb0, &args![this, 0u16]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(
            e.get(this, NiMultiTargetTransformController::m_usNumInterps),
            0
        );
    }

    #[test]
    fn fn_00a2feb0_asks_the_memory_manager_for_four_bytes_for_an_empty_array() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, _| e.mem.alloc(4).into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let block = new_blend_interpolators(&mut e, 0);
        assert_eq!(e.mem.u32(block - 4), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 4]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![block, 0x30, 0, 0x00a4_09b0]]
        );
    }

    #[test]
    fn new_blend_interpolators_sizes_the_block_with_all_ones_on_overflow() {
        let mut e = Engine::new();
        e.register(ARRAY_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(new_blend_interpolators(&mut e, 0x0555_5556), 0);
        assert_eq!(new_blend_interpolators(&mut e, 0x0555_5555), 0);
        assert_eq!(
            calls_to(&e, ARRAY_NEW),
            vec![vec![u32::MAX], vec![0xffff_fff4]]
        );
    }

    #[test]
    fn vector_deleting_destructor_destroys_an_array_back_to_front() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_DELETE, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let block = e.mem.alloc(4 + 0x30 * 3);
        e.mem.set_u32(block, 3);
        let first = block + 4;
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2ff70, &args![first, 3u32]).u32(), block);
        assert_eq!(
            calls_to(&e, BLEND_INTERPOLATOR_DESTRUCT),
            vec![vec![first + 0x60], vec![first + 0x30], vec![first]]
        );
        assert_eq!(calls_to(&e, ARRAY_DELETE), vec![vec![block, 0x30]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00a2ff70, &args![first, 2u32]).u32(), block);
        assert_eq!(calls_to(&e, ARRAY_DELETE).len(), 1);
        assert_eq!(calls_to(&e, BLEND_INTERPOLATOR_DESTRUCT).len(), 6);
    }

    #[test]
    fn vector_deleting_destructor_with_an_empty_array_destroys_nothing() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_DELETE, |_, _| Ret::default());
        let block = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2ff70, &args![block + 4, 3u32]).u32(), block);
        assert!(calls_to(&e, BLEND_INTERPOLATOR_DESTRUCT).is_empty());
        assert_eq!(calls_to(&e, ARRAY_DELETE), vec![vec![block, 0x30]]);
    }

    #[test]
    fn vector_deleting_destructor_of_one_object_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_DELETE, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = 0x7000u32;
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2ff70, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00a2ff70, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x30]]);
        assert_eq!(
            calls_to(&e, BLEND_INTERPOLATOR_DESTRUCT),
            vec![vec![this]; 2]
        );
        assert!(calls_to(&e, ARRAY_DELETE).is_empty());
    }

    #[test]
    fn create_pose_interpolator_builds_the_transform_from_the_target() {
        let mut e = Engine::new();
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for (word, value) in [0x3f80_0000u32, 0x11, 0x22, 0x33].iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * word as u32, *value);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRANSFORM_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let target = e.mem.alloc(0x70);
        for word in 0..3 {
            e.mem.set_u32(target + 0x58 + 4 * word, 0xa0 + word);
        }
        e.mem.set_u32(target + 0x64, 0x7f80_0001);
        let spare = e.mem.alloc(0x10);
        let this = controller_with_targets(&mut e, &[spare, target]);
        e.call_log = Some(vec![]);
        let made = e.call(0x00a2ffe0, &args![this, 1u16]).u32();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(
            calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCT),
            vec![vec![
                made,
                0xa0,
                0xa1,
                0xa2,
                0x3f80_0000,
                0x11,
                0x22,
                0x33,
                0x7fc0_0001
            ]]
        );
        let from_rotation = calls_to(&e, QUATERNION_FROM_ROTATION);
        assert_eq!(from_rotation.len(), 1);
        assert_eq!(from_rotation[0][1], target + 0x34);
    }

    #[test]
    fn create_pose_interpolator_answers_null_without_memory() {
        let mut e = Engine::new();
        e.register(QUATERNION_FROM_ROTATION, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(TRANSFORM_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let target = e.mem.alloc(0x70);
        let this = controller_with_targets(&mut e, &[target]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2ffe0, &args![this, 0u16]).u32(), 0);
        assert!(calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCT).is_empty());
    }

    const COPY_VTABLE: u32 = 0x0300_0000;

    #[test]
    fn member_copy_copies_the_base_then_each_interpolator() {
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let (this, source_interpolators, _) = controller_with_interpolators(&mut e, 2, COPY_VTABLE);
        let destination = e.new_object::<NiMultiTargetTransformController>();
        let process = 0x7777u32;
        e.call_log = Some(vec![]);
        e.call(0x00a30070, &args![this, destination, process]);
        let destination_interpolators = e
            .get(
                destination,
                NiMultiTargetTransformController::m_pkBlendInterps,
            )
            .addr();
        assert_eq!(
            e.get(
                destination,
                NiMultiTargetTransformController::m_usNumInterps
            ),
            2
        );
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), process]]
        );
        assert_eq!(
            calls_to(&e, BLEND_INTERPOLATOR_COPY_MEMBERS),
            vec![
                vec![source_interpolators, destination_interpolators, process],
                vec![
                    source_interpolators + 0x30,
                    destination_interpolators + 0x30,
                    process
                ],
            ]
        );
    }

    #[test]
    fn member_copy_of_an_empty_controller_copies_only_the_base() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiMultiTargetTransformController>();
        let destination = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a30070, &args![this, destination, 0x7777u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn process_clone_stores_the_clones_of_the_targets_in_the_clones_table() {
        const PROCESS_VTABLE: u32 = 0x0300_0000;
        const INTERPOLATOR_PROCESS_CLONE: u32 = 0x0300_1000;
        const MAP: u32 = 0x7777;
        let mut e = Engine::new();
        e.put_vtable(
            PROCESS_VTABLE,
            &vtable_slots(SLOT_PROCESS_CLONE, INTERPOLATOR_PROCESS_CLONE),
        );
        e.register(INTERPOLATOR_PROCESS_CLONE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        let (this, interpolators, table) = controller_with_interpolators(&mut e, 3, PROCESS_VTABLE);
        let found_target = e.mem.alloc(0x10);
        let missing_target = e.mem.alloc(0x10);
        for (index, target) in [found_target, 0, missing_target].iter().enumerate() {
            e.mem.set_u32(table + 4 * index as u32, *target);
        }
        let clone = e.new_object::<NiMultiTargetTransformController>();
        let clone_table = e.mem.alloc(12);
        e.set(
            clone,
            NiMultiTargetTransformController::m_ppkTargets,
            Ptr::new(clone_table),
        );
        for index in 0..3 {
            e.mem.set_u32(clone_table + 4 * index, 0xdead_0000);
        }
        let controller = this.addr();
        let clone_address = clone.addr();
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            if a[1] == controller {
                e.mem.set_u32(a[2], clone_address);
                true.into_ret()
            } else if a[1] == found_target {
                e.mem.set_u32(a[2], 0xc1);
                true.into_ret()
            } else {
                false.into_ret()
            }
        });
        let process = e.mem.alloc(8);
        e.mem.set_u32(process, MAP);
        e.call_log = Some(vec![]);
        e.call(0x00a300d0, &args![this, process]);
        assert_eq!(
            calls_to(&e, INTERPOLATOR_PROCESS_CLONE),
            vec![
                vec![interpolators, process],
                vec![interpolators + 0x30, process],
                vec![interpolators + 0x60, process],
            ]
        );
        assert_eq!(
            calls_to(&e, CLONE_MAP_GET_AT)
                .iter()
                .map(|call| call[..2].to_vec())
                .collect::<Calls>(),
            vec![
                vec![MAP, controller],
                vec![MAP, found_target],
                vec![MAP, missing_target]
            ]
        );
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_PROCESS_CLONE),
            vec![vec![controller, process]]
        );
        assert_eq!(e.mem.u32(clone_table), 0xc1);
        assert_eq!(e.mem.u32(clone_table + 4), 0xdead_0000);
        assert_eq!(e.mem.u32(clone_table + 8), 0);
    }

    #[test]
    fn fn_00a30160_rebuilds_the_interpolators_and_clears_the_targets() {
        const VTABLE: u32 = 0x0300_0000;
        const VECTOR_DELETE: u32 = 0x0300_1000;
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        e.put_vtable(VTABLE, &vtable_slots(0, VECTOR_DELETE));
        e.register(VECTOR_DELETE, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (this, old, table) = controller_with_interpolators(&mut e, 2, VTABLE);
        e.call_log = Some(vec![]);
        e.call(0x00a30160, &args![this]);
        let new = e
            .get(this, NiMultiTargetTransformController::m_pkBlendInterps)
            .addr();
        assert_ne!(new, old);
        assert_eq!(e.mem.u32(new - 4), 2);
        assert_eq!(calls_to(&e, VECTOR_DELETE), vec![vec![old, 3]]);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(
            calls_to(&e, INTERLOCKED_INCREMENT),
            vec![vec![new + 4], vec![new + 0x34]]
        );
        assert_eq!(calls_to(&e, MEMSET), vec![vec![table, 0, 8]]);
        assert_eq!(
            e.get(this, NiMultiTargetTransformController::m_ppkTargets)
                .addr(),
            table
        );
    }

    #[test]
    fn fn_00a30160_returns_an_empty_block_to_the_memory_manager() {
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let (this, old, _) = controller_with_interpolators(&mut e, 2, 0);
        e.mem.set_u32(old - 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a30160, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, old - 4]]
        );
    }

    #[test]
    fn fn_00a30160_does_nothing_without_a_count_or_an_array() {
        let mut e = Engine::new();
        let empty = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a30160, &args![empty]);
        let counted = e.new_object::<NiMultiTargetTransformController>();
        e.set(
            counted,
            NiMultiTargetTransformController::m_usNumInterps,
            4u16,
        );
        e.call(0x00a30160, &args![counted]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn get_interpolator_index_fx_finds_the_target_with_the_same_word_at_8() {
        let mut e = Engine::new();
        e.map(INVALID_INDEX, 2);
        e.mem.set_u16(INVALID_INDEX, 0xffff);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first + 8, 0x55);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 8, 0x66);
        let this = controller_with_targets(&mut e, &[0, first, second]);
        let id = e.mem.alloc(4);
        e.mem.set_u32(id, 0x66);
        assert_eq!(e.call(0x00a30240, &args![this, id]).u16(), 2);
        e.mem.set_u32(id, 0x55);
        assert_eq!(e.call(0x00a30240, &args![this, id]).u16(), 1);
        e.mem.set_u32(id, 0x77);
        assert_eq!(e.call(0x00a30240, &args![this, id]).u16(), 0xffff);
        let empty = e.new_object::<NiMultiTargetTransformController>();
        assert_eq!(e.call(0x00a30240, &args![empty, id]).u16(), 0xffff);
    }

    #[test]
    fn target_is_required_type_walks_the_targets_class_chain() {
        const TARGET_VTABLE: u32 = 0x0300_0000;
        const GET_RTTI: u32 = 0x0300_1000;
        let mut e = Engine::new();
        e.put_vtable(TARGET_VTABLE, &vtable_slots(SLOT_GET_RTTI, GET_RTTI));
        e.mem.map(0x6000, 0x20);
        e.mem.set_u32(0x6004, 0x6010);
        e.mem.set_u32(0x6014, NI_AV_OBJECT_RTTI);
        e.mem.map(0x6100, 0x20);
        e.mem.set_u32(0x6104, 0x6110);
        e.mem.set_u32(0x6114, 0);
        e.register(GET_RTTI, |_, _| 0x6000u32.into_ret());
        let target = object_with_vtable(&mut e, TARGET_VTABLE);
        let this = e.new_object::<NiMultiTargetTransformController>();
        assert!(!e.call(0x00a302a0, &args![this]).bool());
        e.set(
            this,
            NiMultiTargetTransformController::m_pkTarget,
            Ptr::new(target),
        );
        assert!(e.call(0x00a302a0, &args![this]).bool());
        e.register(GET_RTTI, |_, _| 0x6100u32.into_ret());
        assert!(!e.call(0x00a302a0, &args![this]).bool());
        e.register(GET_RTTI, |_, _| Ret::default());
        assert!(!e.call(0x00a302a0, &args![this]).bool());
    }

    #[test]
    fn create_clone_makes_an_empty_controller_and_copies_into_it() {
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(INTERP_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(BLEND_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiMultiTargetTransformController>();
        let process = 0x7777u32;
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a302d0, &args![this, process]).u32();
        assert_ne!(clone, 0);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x40]]);
        assert_eq!(calls_to(&e, INTERP_CONTROLLER_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(
            e.mem.u32(clone),
            NI_MULTI_TARGET_TRANSFORM_CONTROLLER_VTABLE
        );
        assert_eq!(e.mem.u32(clone + 0x34), 0);
        assert_eq!(e.mem.u32(clone + 0x38), 0);
        assert_eq!(e.mem.u16(clone + 0x3c), 0);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, process]]
        );
    }

    #[test]
    fn create_clone_copies_into_address_0_when_there_is_no_memory() {
        let mut e = Engine::new();
        e.mem.map(0, 0x1000);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a302d0, &args![this, 0x7777u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 0x7777]]
        );
    }

    /// A stream whose input and output streams are 0x10-byte objects with the given
    /// read and write function addresses.
    fn stream_with_functions(e: &mut Engine, read: u32, write: u32) -> (Ptr<NiStream>, u32, u32) {
        let stream = e.new_object::<NiStream>();
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, read);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, write);
        e.set(stream, NiStream::m_pkIstr, input);
        e.set(stream, NiStream::m_pkOstr, output);
        (stream, input, output)
    }

    #[test]
    fn load_binary_reads_the_count_and_resolves_one_link_per_interpolator() {
        const READ: u32 = 0x0300_2000;
        let mut e = Engine::new();
        allocation_doubles(&mut e);
        e.register(INTERP_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        let sizes_seen = Rc::new(RefCell::new(Vec::new()));
        let seen = sizes_seen.clone();
        e.register_double(READ, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[3]));
            e.mem.set_u16(a[1], 3);
            2u32.into_ret()
        });
        let next_link = Rc::new(RefCell::new(0x100u32));
        let counter = next_link.clone();
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| {
            *counter.borrow_mut() += 1;
            (*counter.borrow()).into_ret()
        });
        let (stream, input, _) = stream_with_functions(&mut e, READ, 0);
        e.mem.set_u32(input + 4, 10);
        let this = e.new_object::<NiMultiTargetTransformController>();
        e.call_log = Some(vec![]);
        e.call(0x00a303d0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_LOAD_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
        assert_eq!(*sizes_seen.borrow(), vec![2]);
        assert_eq!(e.mem.u32(input + 4), 12);
        assert_eq!(
            e.get(this, NiMultiTargetTransformController::m_usNumInterps),
            3
        );
        let table = e
            .get(this, NiMultiTargetTransformController::m_ppkTargets)
            .addr();
        assert_eq!(
            [e.mem.u32(table), e.mem.u32(table + 4), e.mem.u32(table + 8)],
            [0x101, 0x102, 0x103]
        );
        assert_eq!(calls_to(&e, STREAM_RESOLVE_LINK_ID).len(), 3);
    }

    #[test]
    fn save_binary_writes_the_count_then_each_target_link() {
        const WRITE: u32 = 0x0300_2000;
        const STREAM_VTABLE: u32 = 0x0300_0000;
        const SAVE_LINK_ID: u32 = 0x0300_1000;
        let mut e = Engine::new();
        e.put_vtable(
            STREAM_VTABLE,
            &vtable_slots(SLOT_SAVE_LINK_ID, SAVE_LINK_ID),
        );
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK_ID, |_, _| Ret::default());
        let written = Rc::new(RefCell::new(Vec::new()));
        let seen = written.clone();
        e.register_double(WRITE, move |e, a| {
            seen.borrow_mut()
                .push((e.mem.u16(a[1]), e.mem.u32(a[3]), a[2]));
            2u32.into_ret()
        });
        let (stream, _, output) = stream_with_functions(&mut e, 0, WRITE);
        e.mem.set_u32(stream.addr(), STREAM_VTABLE);
        e.mem.set_u32(output + 4, 100);
        let this = controller_with_targets(&mut e, &[0x11, 0, 0x33]);
        e.call_log = Some(vec![]);
        e.call(0x00a30440, &args![this, stream]);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_SAVE_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
        assert_eq!(*written.borrow(), vec![(3, 2, 2)]);
        assert_eq!(e.mem.u32(output + 4), 102);
        assert_eq!(
            calls_to(&e, SAVE_LINK_ID),
            vec![
                vec![stream.addr(), 0x11],
                vec![stream.addr(), 0],
                vec![stream.addr(), 0x33]
            ]
        );
    }

    #[test]
    fn get_viewer_strings_appends_the_class_name_and_the_count() {
        const NAME: u32 = 0x5151;
        let mut e = Engine::new();
        e.map(NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI, 4);
        e.mem
            .set_u32(NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI, NAME);
        e.register(INTERP_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(GET_VIEWER_STRING_OV6, |_, _| 0x2222u32.into_ret());
        e.register(VIEWER_STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(Vec::new()));
        let seen = stored.clone();
        e.register_double(VIEWER_STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push((a[1], e.mem.u32(a[2])));
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        let this = controller_with_targets(&mut e, &[0, 0, 0]);
        e.call_log = Some(vec![]);
        e.call(0x00a304b0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, INTERP_CONTROLLER_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![NAME]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_OV6),
            vec![vec![TEXT_NUM_INTERPS, 3]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRINGS_GROW), vec![vec![strings, 5]]);
        assert_eq!(*stored.borrow(), vec![(1, 0x1111), (2, 0x2222)]);
    }

    #[test]
    fn ni_multi_target_transform_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a2fbd0, &args![]).u32(),
            NI_MULTI_TARGET_TRANSFORM_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_multi_target_transform_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a2_fb70, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a30280, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a30280, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x40]]);
        assert_eq!(calls_to(&e, 0x00a2_fb70), vec![vec![this]; 2]);
    }
}
