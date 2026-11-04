//! `(unplaced)/run_00649ba0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerList;
use crate::units::fallout_shared_facegen::bsfacegenkeyframe::speech_check;

// Translated from 00649ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor (no Xbox name in the engine map): runs the
/// destructor body at `00649ce0` and frees 0x1c0 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn fn_00649ca0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0064_9ce0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x1c0u32]);
    }
    this
}

/// `00403e20`: the address a `Setting`'s float is loaded from (the
/// callers `FLD` from the returned pointer).
const SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
/// `00450410`: a `Setting`'s float, in ST0.
const SETTING_FLOAT: u32 = 0x0045_0410;
/// `RandomFloat` (`00476b70`, cdecl `(min, max)`, result in ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `004dff00` (cdecl, one `float`): a random chance test, returns a `bool`.
const RANDOM_CHANCE: u32 = 0x004d_ff00;
/// `005330e0` (cdecl `(float *value, float min, float max)`): clamps
/// `*value` into `[min, max]`.
const CLAMP_FLOAT: u32 = 0x0053_30e0;
/// `00653dc0` (cdecl `(float *values, count, flag)`): fills a float array.
const FILL_VALUES: u32 = 0x0065_3dc0;
/// `005dd4e0`: `BSFaceGenKeyframeMultiple` value `i` (`pQuantities[i]`, or
/// 0 past `iNumQuantities`), in ST0.
const KEYFRAME_VALUE: u32 = 0x005d_d4e0;
/// The spin lock's enter (`0040fbf0`, one word) and leave (`0040fba0`).
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
/// `NiTPointerList` helpers: the item count (`0044ddc0`, `+8`), the address
/// of the head item's element (`0073ac90`, `head + 8`) and remove-head
/// (`007b5390`).
const LIST_COUNT: u32 = 0x0044_ddc0;
const LIST_HEAD_ELEMENT: u32 = 0x0073_ac90;
const LIST_REMOVE_HEAD: u32 = 0x007b_5390;
/// `0076b610`: whether a list's count (`+8`) is zero.
const LIST_IS_EMPTY: u32 = 0x0076_b610;
/// `BSFaceGenKeyframeMultiple` constructor (`0064eb60`, `(type, count)`),
/// `BSFaceGenKeyframe::SetTime` (`0064eb30`, engine map),
/// `BSFaceGenKeyframeMultiple::AddToList` (`0064ed60`, engine map) and the
/// keyframe destructor (`0064ed00`).
const KEYFRAME_CONSTRUCT: u32 = 0x0064_eb60;
const KEYFRAME_SET_TIME_DIRECT: u32 = 0x0064_eb30;
const KEYFRAME_ADD_TO_LIST: u32 = 0x0064_ed60;
const KEYFRAME_DESTRUCT: u32 = 0x0064_ed00;
/// `MemoryManager::Allocate` wrapper (`00401000`, cdecl size).
const ALLOCATE: u32 = 0x0040_1000;

/// `BSFaceGenKeyframe` virtual slots used here (byte offsets, with the Xbox
/// PDB's names for them): `GetType` (+0x04), `GetTime` (+0x0C), `SetTime`
/// (+0x10), a `MergeMorphDataFrom` overload (+0x1C), `ClearData` (+0x20),
/// `CopyFrom` (+0x28), `Compare` (+0x2C), `SetToZero` (+0x38),
/// `UpdateToTarget` (+0x3C), `IsTargetAvailable` (+0x40); +0x00 is the
/// scalar deleting destructor.
const KF_GET_TYPE: u32 = 0x04;
const KF_GET_TIME: u32 = 0x0c;
const KF_SET_TIME: u32 = 0x10;
const KF_MERGE: u32 = 0x1c;
const KF_CLEAR_DATA: u32 = 0x20;
const KF_COPY_FROM: u32 = 0x28;
const KF_COMPARE: u32 = 0x2c;
const KF_SET_TO_ZERO: u32 = 0x38;
const KF_UPDATE_TO_TARGET: u32 = 0x3c;
const KF_IS_TARGET_AVAILABLE: u32 = 0x40;

/// `BSFaceGenAnimationData` virtual slots (byte offsets; each checked
/// against the PC vtable at `01064e1c`): `GetExpressionValue` (+0x94),
/// `SetModifierValue` (+0xA0), `ResetCurrentMorphs` (+0xB4, `0064a310`),
/// `ResetSpeechCurrentMorphs` (+0xB8, `0064a520`), `ResetAnimCurrentMorphs`
/// (+0xBC, `0064a770`), `AddModifiersKeyframe` (+0xF0, `0064ab40`),
/// `AddSpeechEmptyKeyframe` (+0xF4, `0064ad20`), `AddAnimEmptyKeyframe`
/// (+0xF8), `ClearKeyframeLists` (+0xFC, `0064ae60`),
/// `ClearSpeechKeyframeLists` (+0x100, `0064aec0`), `ClearAnimKeyframeLists`
/// (+0x104), `SetExpressionTarget(expression, value)` (+0x114, `0064b2b0`),
/// `ClearExpressionTarget` (+0x120, `0064b5e0`).
const VT_GET_EXPRESSION_VALUE: u32 = 0x94;
const VT_SET_MODIFIER_VALUE: u32 = 0xa0;
const VT_RESET_CURRENT_MORPHS: u32 = 0xb4;
const VT_RESET_SPEECH_CURRENT_MORPHS: u32 = 0xb8;
const VT_RESET_ANIM_CURRENT_MORPHS: u32 = 0xbc;
const VT_ADD_MODIFIERS_KEYFRAME: u32 = 0xf0;
const VT_ADD_SPEECH_EMPTY_KEYFRAME: u32 = 0xf4;
const VT_ADD_ANIM_EMPTY_KEYFRAME: u32 = 0xf8;
const VT_CLEAR_KEYFRAME_LISTS: u32 = 0xfc;
const VT_CLEAR_SPEECH_KEYFRAME_LISTS: u32 = 0x100;
const VT_CLEAR_ANIM_KEYFRAME_LISTS: u32 = 0x104;
const VT_SET_EXPRESSION_TARGET: u32 = 0x114;
const VT_CLEAR_EXPRESSION_TARGET: u32 = 0x120;

/// `BSFaceGenAnimationData`'s vtable.
const FACE_GEN_ANIMATION_DATA_VTABLE: u32 = 0x0106_4e1c;
/// `BSFaceGenImage`'s vtable.
const FACE_GEN_IMAGE_VTABLE: u32 = 0x0106_53c4;
/// `BSFaceGenAnimationData::pTag` (Xbox PDB static), the pointer
/// `InitSDM` makes and `KillSDM` frees.
const FACE_GEN_ANIMATION_TAG: u32 = 0x011d_5744;
/// The string `InitSDM` passes to the tag's constructor (`00438170`).
const FACE_GEN_ANIMATION_TAG_TEXT: u32 = 0x0106_4f48;
/// The global byte `0064bcb0` returns (written by the face-gen manager's
/// constructor and by `005dd860`).
const EYE_TRACKING_ENABLED: u32 = 0x011d_59e0;
/// The quaternion `GetModifierValuesAsRotation` compares its result with.
const ROTATION_REFERENCE: u32 = 0x011a_9ea4;

/// Settings (`SettingT<GameSettingCollection>`, names from their
/// initializers' strings).
const F_TRACK_EYE_XY: u32 = 0x011d_5950;
const F_TRACK_EYE_Z: u32 = 0x011d_58f4;
const F_TRACK_DEAD_ZONE_XY: u32 = 0x011d_58a0;
const F_TRACK_DEAD_ZONE_Z: u32 = 0x011d_5974;
const F_TRACK_SPEED: u32 = 0x011d_5924;
const F_BLINK_DOWN_TIME: u32 = 0x011d_595c;
const F_BLINK_UP_TIME: u32 = 0x011d_58b8;
const F_BLINK_DELAY_MIN: u32 = 0x011d_580c;
const F_BLINK_DELAY_MAX: u32 = 0x011d_5914;
const F_LOOK_DOWN_DISABLE_BLINKING_AMT: u32 = 0x011d_57f8;
const F_EYE_PITCH_MAX_ANGRY: u32 = 0x011d_57d0;
const F_EYE_PITCH_MIN_ANGRY: u32 = 0x011d_5944;
const F_EYE_HEADING_MAX_ANGRY: u32 = 0x011d_5750;
const F_EYE_HEADING_MIN_ANGRY: u32 = 0x011d_5968;
const F_EYE_PITCH_MAX_HAPPY: u32 = 0x011d_5874;
const F_EYE_PITCH_MIN_HAPPY: u32 = 0x011d_58e8;
const F_EYE_HEADING_MAX_HAPPY: u32 = 0x011d_5988;
const F_EYE_HEADING_MIN_HAPPY: u32 = 0x011d_5794;
const F_EYE_PITCH_MAX_SAD: u32 = 0x011d_57b8;
const F_EYE_PITCH_MIN_SAD: u32 = 0x011d_581c;
const F_EYE_HEADING_MAX_SAD: u32 = 0x011d_5894;
const F_EYE_HEADING_MIN_SAD: u32 = 0x011d_5930;
const F_EYE_HEADING_MAX_FEAR: u32 = 0x011d_58d8;
const F_EYE_HEADING_MIN_FEAR: u32 = 0x011d_5868;
const F_EYE_PITCH_MAX_NEUTRAL: u32 = 0x011d_5834;
const F_EYE_PITCH_MIN_NEUTRAL: u32 = 0x011d_58c4;
const F_EYE_HEADING_MAX_NEUTRAL: u32 = 0x011d_5908;
const F_EYE_HEADING_MIN_NEUTRAL: u32 = 0x011d_585c;

/// Constants: `0.0` (`double`), `1.0` (`double`), degrees to radians
/// (`double`), `pi / 2` and `-pi / 2` (`double`), `pi / 2` (`float`),
/// `-1.0` (`double`), `FLT_MAX` (`double`).
const ZERO: u32 = 0x0101_2060;
const ONE: u32 = 0x0101_2070;
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
const HALF_PI: u32 = 0x0106_4f50;
const MINUS_HALF_PI: u32 = 0x0106_4f58;
const HALF_PI_FLOAT: u32 = 0x0101_ff38;
const MINUS_ONE: u32 = 0x0101_a6b0;
const FLOAT_MAX: u32 = 0x0102_31b0;
/// `float` constants `ModifyEyeHeadingAndPitchBasedOnExpression` passes to
/// `RandomFloat` and the chance test (values from the exe at run time).
const EYE_CHANCE: u32 = 0x0101_7870;
const EYE_TIME_A: u32 = 0x0101_62c0;
const EYE_TIME_B: u32 = 0x0101_7718;
const EYE_TIME_C: u32 = 0x0101_6248;
const EYE_TIME_D: u32 = 0x0101_6088;
const EYE_TIME_E: u32 = 0x0101_e570;

const FLOAT_ZERO: u32 = 0;
const FLOAT_ONE: u32 = 0x3f80_0000;

layout! {
    /// `BSFaceGenKeyframeMultiple` (Xbox PDB), 0x14 bytes: vtable, type,
    /// time, the value array and its length.
    pub struct BSFaceGenKeyframeMultiple: 0x14 {
        /// `BSFaceGenKeyframe::eType` (Xbox PDB).
        0x04 eType: u32,
        /// `BSFaceGenKeyframe::fTime` (Xbox PDB).
        0x08 fTime: f32,
        /// `pQuantities` (Xbox PDB): `float *`.
        0x0C pQuantities: u32,
        /// `iNumQuantities` (Xbox PDB).
        0x10 iNumQuantities: u32,
    }

    /// `BSFaceGenAnimationData` (Xbox PDB), 0x1C0 bytes on PC (0x1A0 in the
    /// Xbox PDB). The fields up to +0x191 are at the PDB's offsets (checked
    /// against the PC code); the PC build's spin lock is at +0x1A0 (PDB
    /// +0x198).
    pub struct BSFaceGenAnimationData: 0x1C0 {
        /// `pTargetExpressionData` (Xbox PDB): `BSFaceGenKeyframeMultiple*`.
        0x0C pTargetExpressionData: u32,
        /// `CurrentTargetExpressionData` (Xbox PDB).
        0x10 CurrentTargetExpressionData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentAnimExpressionData` (Xbox PDB).
        0x24 CurrentAnimExpressionData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentAnimModifierData` (Xbox PDB).
        0x38 CurrentAnimModifierData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentAnimPhonemeData` (Xbox PDB).
        0x4C CurrentAnimPhonemeData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentAnimCustomMorphData` (Xbox PDB).
        0x60 CurrentAnimCustomMorphData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentExpressionData` (Xbox PDB).
        0x74 CurrentExpressionData: Inline<BSFaceGenKeyframeMultiple>,
        /// `SpeechModifierKeyframeList` (Xbox PDB).
        0x88 SpeechModifierKeyframeList: Inline<NiTPointerList>,
        /// `CurrentSpeechModifierData` (Xbox PDB).
        0x94 CurrentSpeechModifierData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentModifierData` (Xbox PDB).
        0xA8 CurrentModifierData: Inline<BSFaceGenKeyframeMultiple>,
        /// `SpeechPhonemeKeyframeList` (Xbox PDB).
        0xBC SpeechPhonemeKeyframeList: Inline<NiTPointerList>,
        /// `CurrentSpeechPhonemeData` (Xbox PDB).
        0xC8 CurrentSpeechPhonemeData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentPhonemeData` (Xbox PDB).
        0xDC CurrentPhonemeData: Inline<BSFaceGenKeyframeMultiple>,
        /// `SpeechCustomMorphKeyframeList` (Xbox PDB).
        0xF0 SpeechCustomMorphKeyframeList: Inline<NiTPointerList>,
        /// `CurrentSpeechCustomMorphData` (Xbox PDB).
        0xFC CurrentSpeechCustomMorphData: Inline<BSFaceGenKeyframeMultiple>,
        /// `CurrentCustomMorphData` (Xbox PDB).
        0x110 CurrentCustomMorphData: Inline<BSFaceGenKeyframeMultiple>,
        /// `fCurrentEyeHeading` (Xbox PDB).
        0x140 fCurrentEyeHeading: u32,
        /// `fCurrentEyePitch` (Xbox PDB).
        0x144 fCurrentEyePitch: u32,
        /// `fDesiredEyeHeading` (Xbox PDB).
        0x150 fDesiredEyeHeading: u32,
        /// `fDesiredEyePitch` (Xbox PDB).
        0x154 fDesiredEyePitch: u32,
        /// `fExpressionEyeOffsetTimer` (Xbox PDB).
        0x17C fExpressionEyeOffsetTimer: u32,
        /// `fExpressionEyeOffsetHeading` (Xbox PDB).
        0x180 fExpressionEyeOffsetHeading: u32,
        /// `fExpressionEyeOffsetPitch` (Xbox PDB).
        0x184 fExpressionEyeOffsetPitch: u32,
        /// `bShouldHeadTrack` (Xbox PDB).
        0x189 bShouldHeadTrack: u8,
        /// `bChangedLastUpdate` (Xbox PDB).
        0x18B bChangedLastUpdate: u8,
        /// `bMustUpdate` (Xbox PDB).
        0x18C bMustUpdate: u8,
        /// `bDead` (Xbox PDB).
        0x18E bDead: u8,
        /// `bAnimHeadCulled` (Xbox PDB).
        0x18F bAnimHeadCulled: u8,
        /// `bAnimHeadCulledLastFrame` (Xbox PDB).
        0x190 bAnimHeadCulledLastFrame: u8,
        /// `AnimLock` (Xbox PDB `BSSpinLock`, +0x198 there).
        0x1A0 AnimLock: Inline<()>,
    }

    /// `BSFaceGenImage` (Xbox PDB), 0x2C bytes on PC (its allocation in
    /// `0065b6e2`); the PC build's `std::vector` is 0x18 bytes, so the
    /// width and height sit 8 bytes past the PDB's.
    pub struct BSFaceGenImage: 0x2C {
        /// `spCompressedImage` (Xbox PDB): `NiPointer<NiSourceTexture>`.
        0x08 spCompressedImage: Inline<()>,
        /// `UncompressedImage` (Xbox PDB): `std::vector<BSFaceGenRGBA>`.
        0x0C UncompressedImage: Inline<()>,
        /// `iWidth` (Xbox PDB +0x1C).
        0x24 iWidth: u32,
        /// `iHeight` (Xbox PDB +0x20).
        0x28 iHeight: u32,
    }
}

type FaceData = Ptr<BSFaceGenAnimationData>;

/// The value of the `Setting` at `setting`, as the bits an `FLD` from the
/// pointer `00403e20` returns sees.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE_ADDRESS, &args![setting]).u32();
    e.mem.u32(value)
}

/// `00450410` on a setting, stored to a `float` (`FSTP dword`).
fn setting_float(e: &mut Engine, setting: u32) -> u32 {
    x87_store32(e.call(SETTING_FLOAT, &args![setting]).st0)
}

/// `RandomFloat(min, max)` stored to a `float`.
fn random_float(e: &mut Engine, min: u32, max: u32) -> u32 {
    x87_store32(e.call(RANDOM_FLOAT, &args![min, max]).st0)
}

/// `004dff00` on the `float` constant at `constant`.
fn random_chance(e: &mut Engine, constant: u32) -> bool {
    let chance = x87_quiet_bits(e.mem.u32(constant));
    e.call(RANDOM_CHANCE, &args![chance]).u8() != 0
}

/// A `float` constant pushed with `FLD dword` / `FSTP dword`.
fn constant_bits(e: &Engine, constant: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(constant))
}

fn keyframe_set_time(e: &mut Engine, keyframe: u32, time: u32) {
    e.vcall(keyframe, KF_SET_TIME, &args![time]);
}

fn keyframe_clear_data(e: &mut Engine, keyframe: u32) {
    e.vcall(keyframe, KF_CLEAR_DATA, &args![1u32]);
}

fn lock(e: &mut Engine, this: FaceData) {
    let lock = this.at(BSFaceGenAnimationData::AnimLock).addr();
    e.call(LOCK_ENTER, &args![lock, 0u32]);
}

fn unlock(e: &mut Engine, this: FaceData) {
    let lock = this.at(BSFaceGenAnimationData::AnimLock).addr();
    e.call(LOCK_LEAVE, &args![lock]);
}

fn field(this: FaceData, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

/// The element of a keyframe list's head item, or 0 when the list is empty.
fn list_head(e: &mut Engine, list: u32) -> u32 {
    if e.call(LIST_COUNT, &args![list]).u32() != 0 {
        let element = e.call(LIST_HEAD_ELEMENT, &args![list]).u32();
        e.mem.u32(element)
    } else {
        0
    }
}

/// Deletes every keyframe of `list` (`ClearSpeechKeyframeLists`' loop):
/// the head's element is deleted (scalar deleting destructor, flag 1), the
/// head removed, until the list is empty.
fn delete_list_keyframes(e: &mut Engine, list: u32) {
    if e.call(LIST_COUNT, &args![list]).u32() == 0 {
        return;
    }
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): this thread is inside the list.
    speech_check::enter(e, list, CLEAR_NAME);
    let element = e.call(LIST_HEAD_ELEMENT, &args![list]).u32();
    let mut keyframe = e.mem.u32(element);
    while keyframe != 0 {
        e.vcall(keyframe, 0, &args![1u32]);
        e.call(LIST_REMOVE_HEAD, &args![list]);
        keyframe = list_head(e, list);
    }
    speech_check::leave(e, list, CLEAR_NAME);
}

/// The names `NV_SPEECH_CHECK` gives the functions that use a speech list.
const CLEAR_NAME: &str = "ClearSpeechKeyframeLists 0064aec0";
const UPDATE_LIST_NAME: &str = "UpdateKeyframeList 0064ca60";

// Translated from 00649ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetMustUpdate` (Xbox PDB, vtable +0xD0): stores
/// `must_update` in `bMustUpdate`.
pub fn bs_face_gen_animation_data_set_must_update(e: &mut Engine, this: FaceData, must_update: u8) {
    e.set(this, BSFaceGenAnimationData::bMustUpdate, must_update);
}

// Translated from 00649bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetDead` (Xbox PDB, vtable +0xD4): `bDead`.
pub fn bs_face_gen_animation_data_get_dead(e: &mut Engine, this: FaceData) -> u8 {
    e.get(this, BSFaceGenAnimationData::bDead)
}

// Translated from 00649ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00649ce0 - the compiler's exception-unwinding frame (FS:[0] states 0xF..0) is not translated
/// `BSFaceGenAnimationData::~BSFaceGenAnimationData` (Xbox PDB): puts back
/// the class's vtable, clears all keyframe lists (`ClearKeyframeLists(1, 1,
/// 1, 1)`), destroys the members last to first (keyframes through
/// `0064ed00`, the three lists through `004a1a30`) and runs the
/// `NiExtraData` destructor (`00a7b300`).
pub fn bs_face_gen_animation_data_destructor(e: &mut Engine, this: FaceData) {
    e.mem.set_u32(this.addr(), FACE_GEN_ANIMATION_DATA_VTABLE);
    bs_face_gen_animation_data_clear_keyframe_lists(e, this, 1, 1, 1, 1);
    // `NV_SPEECH_CHECK` (diagnostic only): the lists are no longer watched.
    speech_check::forget_face(e, this.addr());
    for (offset, destructor) in [
        (0x110, KEYFRAME_DESTRUCT),
        (0xfc, KEYFRAME_DESTRUCT),
        (0xf0, 0x004a_1a30),
        (0xdc, KEYFRAME_DESTRUCT),
        (0xc8, KEYFRAME_DESTRUCT),
        (0xbc, 0x004a_1a30),
        (0xa8, KEYFRAME_DESTRUCT),
        (0x94, KEYFRAME_DESTRUCT),
        (0x88, 0x004a_1a30),
        (0x74, KEYFRAME_DESTRUCT),
        (0x60, KEYFRAME_DESTRUCT),
        (0x4c, KEYFRAME_DESTRUCT),
        (0x38, KEYFRAME_DESTRUCT),
        (0x24, KEYFRAME_DESTRUCT),
        (0x10, KEYFRAME_DESTRUCT),
    ] {
        e.call(destructor, &args![field(this, offset)]);
    }
    e.call(0x00a7_b300, &args![this]);
}

// Translated from 00649e40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00649e40 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::InitSDM` (Xbox PDB; matched by its body, the
/// class tag string `"FAD"`, not by a vtable or the engine map): allocates 4 bytes and, when
/// that succeeds, constructs them with `00438170` from the string at
/// `01064f48`; stores the result (or null) in `pTag` (`011d5744`).
pub fn bs_face_gen_animation_data_init_sdm(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![4u32]).u32();
    let tag = if block == 0 {
        0
    } else {
        e.call(0x0043_8170, &args![block, FACE_GEN_ANIMATION_TAG_TEXT])
            .u32()
    };
    e.set_global(FACE_GEN_ANIMATION_TAG, tag);
}

// Translated from 00649ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::KillSDM` (Xbox PDB; matched by its body, as
/// `InitSDM`): deletes `pTag` when set
/// (`004b9900` with flag 1) and clears it.
pub fn bs_face_gen_animation_data_kill_sdm(e: &mut Engine) {
    let tag: u32 = e.global(FACE_GEN_ANIMATION_TAG);
    if tag != 0 {
        e.call(0x004b_9900, &args![tag, 1u32]);
    }
    e.set_global(FACE_GEN_ANIMATION_TAG, 0u32);
}

/// The shared body of `GetTrackEyeXYMinMax` and `GetTrackEyeZMinMax`: the
/// setting in degrees converted to radians into `*max`, clamped into
/// `[0, pi / 2]`, and `*min = *max * -1`.
fn track_min_max(e: &mut Engine, setting: u32, min: Ptr, max: Ptr) {
    let degrees = setting_bits(e, setting);
    let radians = x87_mul(x87_load32(degrees), e.global::<f64>(DEGREES_TO_RADIANS));
    e.mem.set_u32(max.addr(), x87_store32(radians));
    if x87_load32(e.mem.u32(max.addr())) < e.global::<f64>(ZERO) {
        e.mem.set_u32(max.addr(), FLOAT_ZERO);
    }
    if x87_load32(e.mem.u32(max.addr())) > e.global::<f64>(HALF_PI) {
        let half_pi = constant_bits(e, HALF_PI_FLOAT);
        e.mem.set_u32(max.addr(), half_pi);
    }
    let negated = x87_mul(
        x87_load32(e.mem.u32(max.addr())),
        e.global::<f64>(MINUS_ONE),
    );
    e.mem.set_u32(min.addr(), x87_store32(negated));
}

/// The shared body of the two dead-zone getters: as [`track_min_max`] for
/// `setting`, then `*max` limited to the eye range's maximum (`eye_range`,
/// one of the two eye getters, on a stack pair), and `*min = *max * -1`.
fn dead_zone_min_max(
    e: &mut Engine,
    setting: u32,
    eye_range: fn(&mut Engine, Ptr, Ptr),
    min: Ptr,
    max: Ptr,
) {
    let degrees = setting_bits(e, setting);
    let radians = x87_mul(x87_load32(degrees), e.global::<f64>(DEGREES_TO_RADIANS));
    e.mem.set_u32(max.addr(), x87_store32(radians));
    if x87_load32(e.mem.u32(max.addr())) < e.global::<f64>(ZERO) {
        e.mem.set_u32(max.addr(), FLOAT_ZERO);
    }
    if x87_load32(e.mem.u32(max.addr())) > e.global::<f64>(HALF_PI) {
        let half_pi = constant_bits(e, HALF_PI_FLOAT);
        e.mem.set_u32(max.addr(), half_pi);
    }
    e.with_stack(8, |e, locals| {
        // [EBP-8] receives the minimum, [EBP-4] the maximum.
        let eye_min = locals;
        let eye_max = locals.byte_add(4);
        eye_range(e, eye_min, eye_max);
        let eye_max_bits = e.mem.u32(eye_max.addr());
        if x87_load32(eye_max_bits) < x87_load32(e.mem.u32(max.addr())) {
            e.mem.set_u32(max.addr(), x87_quiet_bits(eye_max_bits));
        }
    });
    let negated = x87_mul(
        x87_load32(e.mem.u32(max.addr())),
        e.global::<f64>(MINUS_ONE),
    );
    e.mem.set_u32(min.addr(), x87_store32(negated));
}

// Translated from 00649f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetTrackEyeXYMinMax` (Xbox PDB, static):
/// `fTrackEyeXY` in radians, clamped into `[0, pi / 2]`, into `*max`, and
/// its negation into `*min`.
pub fn bs_face_gen_animation_data_get_track_eye_xy_min_max(e: &mut Engine, min: Ptr, max: Ptr) {
    track_min_max(e, F_TRACK_EYE_XY, min, max);
}

// Translated from 00649f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetTrackEyeZMinMax` (Xbox PDB, static): as the
/// XY getter for `fTrackEyeZ`.
pub fn bs_face_gen_animation_data_get_track_eye_z_min_max(e: &mut Engine, min: Ptr, max: Ptr) {
    track_min_max(e, F_TRACK_EYE_Z, min, max);
}

// Translated from 00649fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetTrackDeadZoneXYMinMax` (Xbox PDB, static):
/// `fTrackDeadZoneXY` in radians, clamped into `[0, pi / 2]` and to at most
/// the XY eye range's maximum, into `*max`; its negation into `*min`.
pub fn bs_face_gen_animation_data_get_track_dead_zone_xy_min_max(
    e: &mut Engine,
    min: Ptr,
    max: Ptr,
) {
    dead_zone_min_max(
        e,
        F_TRACK_DEAD_ZONE_XY,
        bs_face_gen_animation_data_get_track_eye_xy_min_max,
        min,
        max,
    );
}

// Translated from 0064a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetTrackDeadZoneZMinMax` (Xbox PDB, static): as
/// the XY dead zone for `fTrackDeadZoneZ` and the Z eye range.
pub fn bs_face_gen_animation_data_get_track_dead_zone_z_min_max(
    e: &mut Engine,
    min: Ptr,
    max: Ptr,
) {
    dead_zone_min_max(
        e,
        F_TRACK_DEAD_ZONE_Z,
        bs_face_gen_animation_data_get_track_eye_z_min_max,
        min,
        max,
    );
}

/// One angle of `GetModifierValuesAsRotation`: exactly `FLT_MAX` becomes
/// 0, then the value is clamped into `[-pi / 2, pi / 2]` (a value not above
/// `-pi / 2`, NaN included, becomes `-pi / 2`) and stored as a `float`.
fn rotation_angle(e: &Engine, bits: u32) -> u32 {
    let mut bits = bits;
    if x87_load32(bits) == e.global::<f64>(FLOAT_MAX) {
        bits = FLOAT_ZERO;
    }
    let value = x87_load32(bits);
    let low_bits: u64 = e.global(MINUS_HALF_PI);
    let high_bits: u64 = e.global(HALF_PI);
    let clamped = if value > f64::from_bits(low_bits) {
        if value < f64::from_bits(high_bits) {
            value
        } else {
            x87_load64(high_bits)
        }
    } else {
        x87_load64(low_bits)
    };
    x87_store32(clamped)
}

// Translated from 0064a100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetModifierValuesAsRotation` (Xbox PDB, vtable
/// +0x9C): values 16, 15 and 14 of `CurrentModifierData`, each through
/// [`rotation_angle`], made into the quaternion `result` by `00a6e1f0`;
/// returns whether it differs from the quaternion at `011a9ea4`
/// (`fn_0064a290`).
pub fn bs_face_gen_animation_data_get_modifier_values_as_rotation(
    e: &mut Engine,
    this: FaceData,
    result: Ptr,
) -> bool {
    let modifiers = this.at(BSFaceGenAnimationData::CurrentModifierData).addr();
    let first = x87_store32(e.call(KEYFRAME_VALUE, &args![modifiers, 0x10u32]).st0);
    let second = x87_store32(e.call(KEYFRAME_VALUE, &args![modifiers, 0xfu32]).st0);
    let third = x87_store32(e.call(KEYFRAME_VALUE, &args![modifiers, 0xeu32]).st0);
    let first = rotation_angle(e, first);
    let second = rotation_angle(e, second);
    let third = rotation_angle(e, third);
    e.call(
        0x00a6_e1f0,
        &args![
            result,
            x87_quiet_bits(first),
            x87_quiet_bits(second),
            x87_quiet_bits(third)
        ],
    );
    fn_0064a290(e, result, Ptr::new(ROTATION_REFERENCE))
}

// Translated from 0064a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any of the first four floats of `this` and `other` differ (a NaN
/// differs from everything): an inequality of two quaternions, used by
/// `GetModifierValuesAsRotation`.
pub fn fn_0064a290(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    (0..4).any(|i| {
        let own = x87_load32(e.mem.u32(this.addr() + 4 * i));
        let theirs = x87_load32(e.mem.u32(other.addr() + 4 * i));
        theirs != own
    })
}

// Translated from 0064a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ResetCurrentMorphs` (Xbox PDB, vtable +0xB4):
/// under the animation lock, resets the speech morphs (vtable +0xB8, all
/// six arguments) and the animation morphs (+0xBC, the first five); when
/// `time` is exactly 0 also zeroes the time of and clears the selected
/// current keyframes (expression, modifiers with values 0 and 1 set back to
/// 1 when `reset_eyes`, phonemes, custom morphs).
#[allow(clippy::too_many_arguments)]
pub fn bs_face_gen_animation_data_reset_current_morphs(
    e: &mut Engine,
    this: FaceData,
    time: f32,
    expression: u8,
    phonemes: u8,
    modifiers: u8,
    custom_morphs: u8,
    reset_eyes: u8,
) {
    lock(e, this);
    let time_bits = x87_quiet_bits(time.to_bits());
    e.vcall(
        this.addr(),
        VT_RESET_SPEECH_CURRENT_MORPHS,
        &args![
            time_bits,
            u32::from(expression),
            u32::from(phonemes),
            u32::from(modifiers),
            u32::from(custom_morphs),
            u32::from(reset_eyes)
        ],
    );
    e.vcall(
        this.addr(),
        VT_RESET_ANIM_CURRENT_MORPHS,
        &args![
            time_bits,
            u32::from(expression),
            u32::from(phonemes),
            u32::from(modifiers),
            u32::from(custom_morphs)
        ],
    );
    if x87_load32(time.to_bits()) == e.global::<f64>(ZERO) {
        if expression != 0 {
            let keyframe = field(this, 0x74);
            keyframe_set_time(e, keyframe, FLOAT_ZERO);
            keyframe_clear_data(e, keyframe);
        }
        if modifiers != 0 {
            let keyframe = field(this, 0xa8);
            keyframe_set_time(e, keyframe, FLOAT_ZERO);
            keyframe_clear_data(e, keyframe);
            if reset_eyes != 0 {
                let keyframe = Ptr::new(keyframe);
                fn_0064a4d0(e, keyframe, 0, 1.0);
                fn_0064a4d0(e, keyframe, 1, 1.0);
            }
        }
        if phonemes != 0 {
            let keyframe = field(this, 0xdc);
            keyframe_set_time(e, keyframe, FLOAT_ZERO);
            keyframe_clear_data(e, keyframe);
        }
        if custom_morphs != 0 {
            let keyframe = field(this, 0x110);
            keyframe_set_time(e, keyframe, FLOAT_ZERO);
            keyframe_clear_data(e, keyframe);
        }
    }
    unlock(e, this);
}

// Translated from 0064a4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets value `index` of a `BSFaceGenKeyframeMultiple` to `value`: false
/// when `index` is not below `iNumQuantities` or the value is already equal
/// (a NaN is never equal, so it is stored), true after storing it.
pub fn fn_0064a4d0(
    e: &mut Engine,
    this: Ptr<BSFaceGenKeyframeMultiple>,
    index: u32,
    value: f32,
) -> bool {
    if index >= e.get(this, BSFaceGenKeyframeMultiple::iNumQuantities) {
        return false;
    }
    let values = e.get(this, BSFaceGenKeyframeMultiple::pQuantities);
    let slot = values.wrapping_add(index.wrapping_mul(4));
    let old = x87_load32(e.mem.u32(slot));
    if x87_load32(value.to_bits()) == old {
        return false;
    }
    e.mem.set_u32(slot, x87_quiet_bits(value.to_bits()));
    true
}

// Translated from 0064a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ResetSpeechCurrentMorphs` (Xbox PDB, vtable
/// +0xB8): clears the keyframe lists (vtable +0xFC with the first four
/// flags) and, per selected morph type, zeroes the current speech keyframe's
/// time and either eases back over `time` (a positive `time`: target
/// expression `-1` at 1.0 through +0x114, or an empty speech keyframe
/// through +0xF4 / a modifiers keyframe with values 0 and 1 at 1.0 through
/// +0xF0 when `reset_eyes`) or clears it at once (values 0 and 1 of the
/// modifiers set back to 1.0 through +0xA0 when `reset_eyes`).
#[allow(clippy::too_many_arguments)]
pub fn bs_face_gen_animation_data_reset_speech_current_morphs(
    e: &mut Engine,
    this: FaceData,
    time: f32,
    expression: u8,
    phonemes: u8,
    modifiers: u8,
    custom_morphs: u8,
    reset_eyes: u8,
) {
    e.vcall(
        this.addr(),
        VT_CLEAR_KEYFRAME_LISTS,
        &args![
            u32::from(expression),
            u32::from(phonemes),
            u32::from(modifiers),
            u32::from(custom_morphs)
        ],
    );
    let time_bits = x87_quiet_bits(time.to_bits());
    if expression != 0 {
        let keyframe = field(this, 0x10);
        keyframe_set_time(e, keyframe, FLOAT_ZERO);
        if x87_load32(time.to_bits()) > e.global::<f64>(ZERO) {
            e.vcall(
                this.addr(),
                VT_SET_EXPRESSION_TARGET,
                &args![u32::MAX, FLOAT_ONE],
            );
        } else {
            keyframe_clear_data(e, keyframe);
        }
    }
    if modifiers != 0 {
        let keyframe = field(this, 0x94);
        keyframe_set_time(e, keyframe, FLOAT_ZERO);
        if x87_load32(time.to_bits()) > e.global::<f64>(ZERO) {
            if reset_eyes != 0 {
                e.with_stack(0x44, |e, values| {
                    e.call(FILL_VALUES, &args![values, 0x11u32, 1u32]);
                    e.mem.set_u32(values.addr(), FLOAT_ONE);
                    e.mem.set_u32(values.addr() + 4, FLOAT_ONE);
                    e.vcall(
                        this.addr(),
                        VT_ADD_MODIFIERS_KEYFRAME,
                        &args![values, time_bits],
                    );
                });
            } else {
                e.vcall(
                    this.addr(),
                    VT_ADD_SPEECH_EMPTY_KEYFRAME,
                    &args![2u32, time_bits, 1u32],
                );
            }
        } else {
            keyframe_clear_data(e, keyframe);
            if reset_eyes != 0 {
                e.vcall(this.addr(), VT_SET_MODIFIER_VALUE, &args![0u32, FLOAT_ONE]);
                e.vcall(this.addr(), VT_SET_MODIFIER_VALUE, &args![1u32, FLOAT_ONE]);
            }
        }
    }
    if phonemes != 0 {
        let keyframe = field(this, 0xc8);
        keyframe_set_time(e, keyframe, FLOAT_ZERO);
        if x87_load32(time.to_bits()) > e.global::<f64>(ZERO) {
            e.vcall(
                this.addr(),
                VT_ADD_SPEECH_EMPTY_KEYFRAME,
                &args![0u32, time_bits, 1u32],
            );
        } else {
            keyframe_clear_data(e, keyframe);
        }
    }
    if custom_morphs != 0 {
        let keyframe = field(this, 0xfc);
        keyframe_set_time(e, keyframe, FLOAT_ZERO);
        if x87_load32(time.to_bits()) > e.global::<f64>(ZERO) {
            e.vcall(
                this.addr(),
                VT_ADD_SPEECH_EMPTY_KEYFRAME,
                &args![3u32, time_bits, 1u32],
            );
        } else {
            keyframe_clear_data(e, keyframe);
        }
    }
}

// Translated from 0064a770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ResetAnimCurrentMorphs` (Xbox PDB, vtable
/// +0xBC): clears the animation keyframe lists (vtable +0x104) and, per
/// selected morph type, zeroes the current animation keyframe's time and
/// either adds an empty animation keyframe over a positive `time` (vtable
/// +0xF8: type 1 expression, 2 modifiers, 0 phonemes, 3 custom morphs) or
/// clears the keyframe.
pub fn bs_face_gen_animation_data_reset_anim_current_morphs(
    e: &mut Engine,
    this: FaceData,
    time: f32,
    expression: u8,
    phonemes: u8,
    modifiers: u8,
    custom_morphs: u8,
) {
    e.vcall(
        this.addr(),
        VT_CLEAR_ANIM_KEYFRAME_LISTS,
        &args![
            u32::from(expression),
            u32::from(phonemes),
            u32::from(modifiers),
            u32::from(custom_morphs)
        ],
    );
    let time_bits = x87_quiet_bits(time.to_bits());
    for (selected, offset, morph_type) in [
        (expression, 0x24u32, 1u32),
        (modifiers, 0x38, 2),
        (phonemes, 0x4c, 0),
        (custom_morphs, 0x60, 3),
    ] {
        if selected == 0 {
            continue;
        }
        let keyframe = field(this, offset);
        keyframe_set_time(e, keyframe, FLOAT_ZERO);
        if x87_load32(time.to_bits()) > e.global::<f64>(ZERO) {
            e.vcall(
                this.addr(),
                VT_ADD_ANIM_EMPTY_KEYFRAME,
                &args![morph_type, time_bits, 1u32],
            );
        } else {
            keyframe_clear_data(e, keyframe);
        }
    }
}

// Translated from 0064a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetDead` (Xbox PDB, vtable +0xD8): on becoming
/// dead resets all morphs (vtable +0xB4, time 0 when `immediately`, else 1,
/// all five flags) and sets `bDead` and `bChangedLastUpdate`; on coming
/// back resets them without the eye flag, clears `bDead` and sets
/// `bChangedLastUpdate` and `bShouldHeadTrack`.
pub fn bs_face_gen_animation_data_set_dead(
    e: &mut Engine,
    this: FaceData,
    dead: u8,
    immediately: u8,
) {
    let was_dead = e.get(this, BSFaceGenAnimationData::bDead);
    if dead != 0 && was_dead == 0 {
        let time = if immediately != 0 {
            FLOAT_ZERO
        } else {
            FLOAT_ONE
        };
        e.vcall(
            this.addr(),
            VT_RESET_CURRENT_MORPHS,
            &args![time, 1u32, 1u32, 1u32, 1u32, 1u32],
        );
        e.set(this, BSFaceGenAnimationData::bDead, dead);
        e.set(this, BSFaceGenAnimationData::bChangedLastUpdate, 1u8);
    } else if dead == 0 && was_dead != 0 {
        e.set(this, BSFaceGenAnimationData::bDead, dead);
        let time = if immediately != 0 {
            FLOAT_ZERO
        } else {
            FLOAT_ONE
        };
        e.vcall(
            this.addr(),
            VT_RESET_CURRENT_MORPHS,
            &args![time, 1u32, 1u32, 1u32, 1u32, 0u32],
        );
        e.set(this, BSFaceGenAnimationData::bChangedLastUpdate, 1u8);
        e.set(this, BSFaceGenAnimationData::bShouldHeadTrack, 1u8);
    }
}

/// A keyframe of `count` values of `morph_type` on the stack (`0064eb60`),
/// `body` run on it, then its destructor (`0064ed00`).
fn with_stack_keyframe(
    e: &mut Engine,
    morph_type: u32,
    count: u32,
    body: impl FnOnce(&mut Engine, Ptr<BSFaceGenKeyframeMultiple>),
) {
    e.with_stack(0x14, |e, keyframe| {
        e.call(KEYFRAME_CONSTRUCT, &args![keyframe, morph_type, count]);
        body(e, keyframe.cast());
        e.call(KEYFRAME_DESTRUCT, &args![keyframe]);
    });
}

// Translated from 0064aa00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064aa00 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::AddPhonemeKeyframe` (Xbox PDB, vtable +0xEC):
/// unless dead, `values` is null or `time` is negative, and when all 16
/// values lie in `[0, 1]` (a NaN passes), builds a phoneme keyframe (type 0,
/// 16 values) at `time` with them and adds it to `SpeechPhonemeKeyframeList`.
pub fn bs_face_gen_animation_data_add_phoneme_keyframe(
    e: &mut Engine,
    this: FaceData,
    values: Ptr,
    time: f32,
) {
    // `NV_SPEECH_CHECK` (diagnostic only): the face's speech lists are watched.
    speech_check::track_face(e, this.addr());
    with_stack_keyframe(e, 0, 0x10, |e, keyframe| {
        if e.get(this, BSFaceGenAnimationData::bDead) != 0
            || values.is_null()
            || x87_load32(time.to_bits()) < e.global::<f64>(ZERO)
        {
            return;
        }
        for i in 0..0x10u32 {
            let value = x87_load32(e.mem.u32(values.addr().wrapping_add(4 * i)));
            if value < e.global::<f64>(ZERO) || value > e.global::<f64>(ONE) {
                return;
            }
        }
        e.call(
            KEYFRAME_SET_TIME_DIRECT,
            &args![keyframe, x87_quiet_bits(time.to_bits())],
        );
        for i in 0..0x10u32 {
            let value = x87_quiet_bits(e.mem.u32(values.addr().wrapping_add(4 * i)));
            fn_0064a4d0(e, keyframe, i, f32::from_bits(value));
        }
        let list = field(this, 0xbc);
        e.call(KEYFRAME_ADD_TO_LIST, &args![keyframe, list]);
    });
}

// Translated from 0064ab40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ab40 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::AddModifiersKeyframe` (Xbox PDB, vtable +0xF0):
/// unless dead, `values` is null or `time` is negative, builds a modifiers
/// keyframe (type 2, 17 values) at `time` with the 17 values and adds it to
/// `SpeechModifierKeyframeList`.
pub fn bs_face_gen_animation_data_add_modifiers_keyframe(
    e: &mut Engine,
    this: FaceData,
    values: Ptr,
    time: f32,
) {
    // `NV_SPEECH_CHECK` (diagnostic only): the face's speech lists are watched.
    speech_check::track_face(e, this.addr());
    with_stack_keyframe(e, 2, 0x11, |e, keyframe| {
        if e.get(this, BSFaceGenAnimationData::bDead) != 0
            || values.is_null()
            || x87_load32(time.to_bits()) < e.global::<f64>(ZERO)
        {
            return;
        }
        e.call(
            KEYFRAME_SET_TIME_DIRECT,
            &args![keyframe, x87_quiet_bits(time.to_bits())],
        );
        for i in 0..0x11u32 {
            let value = x87_quiet_bits(e.mem.u32(values.addr().wrapping_add(4 * i)));
            fn_0064a4d0(e, keyframe, i, f32::from_bits(value));
        }
        let list = field(this, 0x88);
        e.call(KEYFRAME_ADD_TO_LIST, &args![keyframe, list]);
    });
}

// Translated from 0064ad20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ad20 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::AddSpeechEmptyKeyframe` (Xbox PDB, vtable
/// +0xF4): unless dead, adds an empty keyframe at `time` to the speech list
/// of `morph_type`: 0 phonemes (16 values, `SpeechPhonemeKeyframeList`), 2
/// modifiers (17, `SpeechModifierKeyframeList`), 3 custom morphs (1,
/// `SpeechCustomMorphKeyframeList`); other types add nothing. The third
/// argument is not read.
pub fn bs_face_gen_animation_data_add_speech_empty_keyframe(
    e: &mut Engine,
    this: FaceData,
    morph_type: u32,
    time: f32,
    _unused_3: u32,
) {
    // `NV_SPEECH_CHECK` (diagnostic only): the face's speech lists are watched.
    speech_check::track_face(e, this.addr());
    if e.get(this, BSFaceGenAnimationData::bDead) != 0 {
        return;
    }
    let (count, list) = match morph_type {
        0 => (0x10, 0xbc),
        2 => (0x11, 0x88),
        3 => (1, 0xf0),
        _ => return,
    };
    with_stack_keyframe(e, morph_type, count, |e, keyframe| {
        e.call(
            KEYFRAME_SET_TIME_DIRECT,
            &args![keyframe, x87_quiet_bits(time.to_bits())],
        );
        e.call(KEYFRAME_ADD_TO_LIST, &args![keyframe, field(this, list)]);
    });
}

// Translated from 0064ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ClearKeyframeLists` (Xbox PDB, vtable +0xFC):
/// the speech lists (vtable +0x100) and the animation lists (+0x104), both
/// with the four flags.
pub fn bs_face_gen_animation_data_clear_keyframe_lists(
    e: &mut Engine,
    this: FaceData,
    expression: u8,
    phonemes: u8,
    modifiers: u8,
    custom_morphs: u8,
) {
    let flags = args![
        u32::from(expression),
        u32::from(phonemes),
        u32::from(modifiers),
        u32::from(custom_morphs)
    ];
    e.vcall(this.addr(), VT_CLEAR_SPEECH_KEYFRAME_LISTS, &flags);
    e.vcall(this.addr(), VT_CLEAR_ANIM_KEYFRAME_LISTS, &flags);
}

// Translated from 0064aec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ClearSpeechKeyframeLists` (Xbox PDB, vtable
/// +0x100): `expression` clears the expression target (vtable +0x120);
/// `modifiers`, `phonemes` and `custom_morphs` delete every keyframe of the
/// modifier, phoneme and custom-morph speech lists, in that order.
pub fn bs_face_gen_animation_data_clear_speech_keyframe_lists(
    e: &mut Engine,
    this: FaceData,
    expression: u8,
    phonemes: u8,
    modifiers: u8,
    custom_morphs: u8,
) {
    if expression != 0 {
        e.vcall(this.addr(), VT_CLEAR_EXPRESSION_TARGET, &[]);
    }
    if modifiers != 0 {
        delete_list_keyframes(e, field(this, 0x88));
    }
    if phonemes != 0 {
        delete_list_keyframes(e, field(this, 0xbc));
    }
    if custom_morphs != 0 {
        delete_list_keyframes(e, field(this, 0xf0));
    }
}

/// `pTargetExpressionData` made ready for a new target: cleared (slot
/// +0x20) when it exists, otherwise allocated (0x14 bytes), constructed as
/// an expression keyframe (type 1, 15 values) and zeroed (slot +0x38).
fn prepare_expression_target(e: &mut Engine, this: FaceData) {
    let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    if target != 0 {
        keyframe_clear_data(e, target);
        return;
    }
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let target = if block == 0 {
        0
    } else {
        e.call(KEYFRAME_CONSTRUCT, &args![block, 1u32, 0xfu32])
            .u32()
    };
    e.set(this, BSFaceGenAnimationData::pTargetExpressionData, target);
    // The game calls through the new pointer without checking it.
    e.vcall(target, KF_SET_TO_ZERO, &[]);
}

/// Ends a target change: when `CurrentTargetExpressionData` compares false
/// with the target (slot +0x2C), the target is cleared (vtable +0x120).
fn finish_expression_target(e: &mut Engine, this: FaceData) {
    let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    let current = field(this, 0x10);
    if e.vcall(current, KF_COMPARE, &args![target]).u8() == 0 {
        e.vcall(this.addr(), VT_CLEAR_EXPRESSION_TARGET, &[]);
    }
}

// Translated from 0064b2b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064b2b0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::SetExpressionTarget(expression, value)` (Xbox
/// PDB; on PC at vtable +0x114): for an expression in `[-1, 15]` and a
/// value in `[0, 1]` (a NaN passes), prepares `pTargetExpressionData`; then
/// for a real expression (not -1) and a nonzero value sets that value in it
/// and clears the target when it compares false with the current target
/// keyframe.
pub fn bs_face_gen_animation_data_set_expression_target(
    e: &mut Engine,
    this: FaceData,
    expression: i32,
    value: f32,
) {
    if !(-1..=0xf).contains(&expression) {
        return;
    }
    let level = x87_load32(value.to_bits());
    if level < e.global::<f64>(ZERO) || level > e.global::<f64>(ONE) {
        return;
    }
    prepare_expression_target(e, this);
    if expression == -1 || level == e.global::<f64>(ZERO) {
        return;
    }
    let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    fn_0064a4d0(
        e,
        Ptr::new(target),
        expression as u32,
        f32::from_bits(x87_quiet_bits(value.to_bits())),
    );
    finish_expression_target(e, this);
}

// Translated from 0064b400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064b400 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::SetExpressionTargetFromMood` (Xbox PDB, vtable
/// +0x11C): prepares `pTargetExpressionData`; unless dead, sets expression
/// `mood + 5` to 1.0 for moods 0..=7 (other moods set nothing) and clears
/// the target when it compares false with the current target keyframe. The
/// second argument is not read.
pub fn bs_face_gen_animation_data_set_expression_target_from_mood(
    e: &mut Engine,
    this: FaceData,
    mood: u32,
    _unused_2: u32,
) {
    prepare_expression_target(e, this);
    if e.get(this, BSFaceGenAnimationData::bDead) != 0 {
        return;
    }
    // The exe's jump table at 0064b5bc: mood n sets expression n + 5.
    if mood <= 7 {
        let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
        fn_0064a4d0(e, Ptr::new(target), mood + 5, 1.0);
    }
    finish_expression_target(e, this);
}

// Translated from 0064b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ClearExpressionTarget` (Xbox PDB, vtable +0x120):
/// deletes `pTargetExpressionData` when set (scalar deleting destructor,
/// flag 1) and clears it.
pub fn bs_face_gen_animation_data_clear_expression_target(e: &mut Engine, this: FaceData) {
    let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    if target != 0 {
        e.vcall(target, 0, &args![1u32]);
    }
    e.set(this, BSFaceGenAnimationData::pTargetExpressionData, 0u32);
}

/// The automatic blink of `Update`: three modifiers keyframes with values 0
/// and 1 (the eyelids) at 0, 1, 0, after a random delay in `[fBlinkDelayMin,
/// fBlinkDelayMax]`, `fBlinkDownTime` and `fBlinkUpTime`.
fn add_blink(e: &mut Engine, this: FaceData) {
    let max = e
        .call(SETTING_VALUE_ADDRESS, &args![F_BLINK_DELAY_MAX])
        .u32();
    let min = e
        .call(SETTING_VALUE_ADDRESS, &args![F_BLINK_DELAY_MIN])
        .u32();
    let span = x87_store32(x87_sub(x87_load32(e.mem.u32(max)), x87_m32(e.mem.u32(min))));
    e.with_stack(0x44, |e, values| {
        e.call(FILL_VALUES, &args![values, 0x11u32, 0u32]);
        e.mem.set_u32(values.addr(), FLOAT_ZERO);
        e.mem.set_u32(values.addr() + 4, FLOAT_ZERO);
        let min = e
            .call(SETTING_VALUE_ADDRESS, &args![F_BLINK_DELAY_MIN])
            .u32();
        let random = e.call(0x005c_5420, &[]).st0;
        let delay = x87_store32(x87_add(
            x87_mul(random, x87_m32(span)),
            x87_m32(e.mem.u32(min)),
        ));
        e.vcall(
            this.addr(),
            VT_ADD_MODIFIERS_KEYFRAME,
            &args![values, x87_quiet_bits(delay)],
        );

        e.call(FILL_VALUES, &args![values, 0x11u32, 0u32]);
        e.mem.set_u32(values.addr(), FLOAT_ONE);
        e.mem.set_u32(values.addr() + 4, FLOAT_ONE);
        let down = x87_quiet_bits(setting_bits(e, F_BLINK_DOWN_TIME));
        e.vcall(this.addr(), VT_ADD_MODIFIERS_KEYFRAME, &args![values, down]);

        e.call(FILL_VALUES, &args![values, 0x11u32, 0u32]);
        e.mem.set_u32(values.addr(), FLOAT_ZERO);
        e.mem.set_u32(values.addr() + 4, FLOAT_ZERO);
        let up = x87_quiet_bits(setting_bits(e, F_BLINK_UP_TIME));
        e.vcall(this.addr(), VT_ADD_MODIFIERS_KEYFRAME, &args![values, up]);
    });
}

/// One morph type of `Update`: `changed` says the speech list's keyframe
/// moved this update; the animation keyframe counts as available when the
/// head was culled this frame but not the last (otherwise slot +0x40 of
/// it). The combined keyframe copies the speech keyframe (merging the
/// animation one at 1.0 when available) or the animation one alone.
/// Returns whether anything changed.
fn combine_morphs(
    e: &mut Engine,
    this: FaceData,
    changed: bool,
    speech: u32,
    anim: u32,
    combined: u32,
) -> bool {
    let available = if e.get(this, BSFaceGenAnimationData::bAnimHeadCulledLastFrame) == 0
        && e.get(this, BSFaceGenAnimationData::bAnimHeadCulled) != 0
    {
        1
    } else {
        e.vcall(anim, KF_IS_TARGET_AVAILABLE, &[]).u8()
    };
    if changed {
        e.vcall(combined, KF_COPY_FROM, &args![speech]);
        if available != 0 {
            e.vcall(combined, KF_MERGE, &args![anim, FLOAT_ONE, 0u32, 0u32]);
        }
    } else if available != 0 {
        e.vcall(combined, KF_COPY_FROM, &args![anim]);
    }
    changed || available != 0
}

// Translated from 0064b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::Update` (Xbox PDB, vtable +0x128): under the
/// animation lock, adds an automatic blink when eye tracking is on
/// (`0064bcb0`), the face is alive, no modifiers keyframe is queued, the
/// blink settings are positive with `fBlinkDelayMin <= fBlinkDelayMax`, and
/// modifier 8 is below `fLookDownDisableBlinkingAmt`. With a positive
/// `seconds` it then moves the expression toward its target (slot +0x3C)
/// and steps the speech lists (`UpdateKeyframeList`), combining each with
/// its animation keyframe, and sets `bChangedLastUpdate` when anything
/// moved. Always copies `bAnimHeadCulled` to `bAnimHeadCulledLastFrame` and
/// returns `bChangedLastUpdate`.
pub fn bs_face_gen_animation_data_update(e: &mut Engine, this: FaceData, seconds: f32) -> u8 {
    lock(e, this);
    // `NV_SPEECH_CHECK` (diagnostic only, reads memory): the speech lists as Update finds them.
    speech_check::at(e, this.addr(), "Update 0064b630");
    e.with_stack(0xc, |e, local| {
        // A local `006815c0` constructs (it returns its `this`).
        e.call(0x0068_15c0, &args![local]);
    });
    let elapsed = f32::from_bits(x87_quiet_bits(seconds.to_bits()));
    let modifiers = field(this, 0xa8);
    let look_down = x87_store32(e.call(KEYFRAME_VALUE, &args![modifiers, 8u32]).st0);

    let zero = e.global::<f64>(ZERO);
    if fn_0064bcb0(e) != 0
        && e.get(this, BSFaceGenAnimationData::bDead) == 0
        && e.call(LIST_IS_EMPTY, &args![field(this, 0x88)]).u8() != 0
        && x87_load32(setting_bits(e, F_BLINK_DOWN_TIME)) > zero
        && x87_load32(setting_bits(e, F_BLINK_UP_TIME)) > zero
        && x87_load32(setting_bits(e, F_BLINK_DELAY_MIN)) > zero
    {
        let max = e
            .call(SETTING_VALUE_ADDRESS, &args![F_BLINK_DELAY_MAX])
            .u32();
        let min = e
            .call(SETTING_VALUE_ADDRESS, &args![F_BLINK_DELAY_MIN])
            .u32();
        let delay_max = x87_load32(e.mem.u32(max));
        let delay_min = x87_load32(e.mem.u32(min));
        if delay_min <= delay_max {
            let limit = x87_load32(setting_bits(e, F_LOOK_DOWN_DISABLE_BLINKING_AMT));
            if limit > x87_load32(look_down) {
                add_blink(e, this);
            }
        }
    }

    if x87_load32(seconds.to_bits()) > zero {
        e.set(this, BSFaceGenAnimationData::bChangedLastUpdate, 0u8);
        let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
        let changed = e
            .vcall(
                field(this, 0x10),
                KF_UPDATE_TO_TARGET,
                &args![elapsed.to_bits(), target],
            )
            .u8();
        let anim = field(this, 0x24);
        let available = if e.get(this, BSFaceGenAnimationData::bAnimHeadCulledLastFrame) == 0
            && e.get(this, BSFaceGenAnimationData::bAnimHeadCulled) != 0
        {
            1
        } else {
            e.vcall(anim, KF_IS_TARGET_AVAILABLE, &[]).u8()
        };
        if available == 0 && changed == 0 {
            e.vcall(this.addr(), VT_CLEAR_EXPRESSION_TARGET, &[]);
        }
        let expression = field(this, 0x74);
        if changed != 0 {
            e.vcall(expression, KF_COPY_FROM, &args![field(this, 0x10)]);
            if available != 0 {
                e.vcall(expression, KF_MERGE, &args![anim, FLOAT_ONE, 0u32, 0u32]);
            }
        } else if available != 0 {
            e.vcall(expression, KF_COPY_FROM, &args![anim]);
        }
        let expression_changed = changed != 0 || available != 0;

        let mut any = expression_changed;
        for (list, speech, anim, combined) in [
            (0x88u32, 0x94u32, 0x38u32, 0xa8u32),
            (0xbc, 0xc8, 0x4c, 0xdc),
            (0xf0, 0xfc, 0x60, 0x110),
        ] {
            // `NV_SPEECH_CHECK` (diagnostic only): this thread is inside the list.
            speech_check::enter(e, field(this, list), UPDATE_LIST_NAME);
            let moved = fn_0064ca60(
                e,
                elapsed,
                Ptr::new(field(this, list)),
                Ptr::new(field(this, speech)),
                0,
            ) != 0;
            speech_check::leave(e, field(this, list), UPDATE_LIST_NAME);
            let type_changed = combine_morphs(
                e,
                this,
                moved,
                field(this, speech),
                field(this, anim),
                field(this, combined),
            );
            any = any || type_changed;
        }
        e.set(
            this,
            BSFaceGenAnimationData::bChangedLastUpdate,
            u8::from(any),
        );
    }

    let culled = e.get(this, BSFaceGenAnimationData::bAnimHeadCulled);
    e.set(
        this,
        BSFaceGenAnimationData::bAnimHeadCulledLastFrame,
        culled,
    );
    unlock(e, this);
    e.get(this, BSFaceGenAnimationData::bChangedLastUpdate)
}

// Translated from 0064bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global byte at `011d59e0` (written by the face-gen manager's
/// constructor and by `005dd860`); `Update` and `SetEyePosition` take it as
/// "eye tracking and blinking on".
pub fn fn_0064bcb0(e: &mut Engine) -> u8 {
    e.global(EYE_TRACKING_ENABLED)
}

// Translated from 0064bda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::GetCurrentExpression` (Xbox PDB; matched by its
/// signature and body, not by a vtable or the engine map): the
/// expression in `-1..15` (asked from -1 on through vtable +0x94) with the
/// largest value in `(0, 1]`, the first on ties; -1 when none.
pub fn bs_face_gen_animation_data_get_current_expression(e: &mut Engine, this: FaceData) -> i32 {
    let mut best_index = -1i32;
    let mut best = FLOAT_ZERO;
    let zero = e.global::<f64>(ZERO);
    let one = e.global::<f64>(ONE);
    for expression in -1..0xf {
        let value = x87_store32(
            e.vcall(this.addr(), VT_GET_EXPRESSION_VALUE, &args![expression])
                .st0,
        );
        let level = x87_load32(value);
        // The exe tests the lower bound twice (`TEST AH,0x41`, then `TEST AH,1`).
        if level > zero && level >= zero && level <= one && x87_load32(best) < level {
            best = x87_quiet_bits(value);
            best_index = expression;
        }
    }
    best_index
}

// Translated from 0064be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::UpdateEyeTracking` (Xbox PDB; matched by its
/// signature and the fields it uses): under the
/// animation lock and unless dead, moves the current eye heading and pitch
/// toward the desired ones (offset by the current expression's eye drift,
/// `ModifyEyeHeadingAndPitchBasedOnExpression`, for an expression below 15)
/// by at most `fTrackSpeed * seconds` each, and applies them
/// (`SetEyePosition`). Returns whether it ran (not dead). The second
/// argument is not read.
pub fn bs_face_gen_animation_data_update_eye_tracking(
    e: &mut Engine,
    this: FaceData,
    seconds: f32,
    _unused_2: u32,
) -> u8 {
    lock(e, this);
    let alive = u8::from(e.get(this, BSFaceGenAnimationData::bDead) == 0);
    if alive != 0 {
        let expression = bs_face_gen_animation_data_get_current_expression(e, this);
        e.with_stack(0x10, |e, locals| {
            let heading = locals;
            let pitch = locals.byte_add(4);
            let heading_step = locals.byte_add(8);
            let pitch_step = locals.byte_add(0xc);
            let desired_heading = e.get(this, BSFaceGenAnimationData::fDesiredEyeHeading);
            let desired_pitch = e.get(this, BSFaceGenAnimationData::fDesiredEyePitch);
            e.mem
                .set_u32(heading.addr(), x87_quiet_bits(desired_heading));
            e.mem.set_u32(pitch.addr(), x87_quiet_bits(desired_pitch));
            if expression < 0xf {
                bs_face_gen_animation_data_modify_eye_heading_and_pitch_based_on_expression(
                    e,
                    this,
                    expression,
                    heading,
                    pitch,
                    f32::from_bits(x87_quiet_bits(seconds.to_bits())),
                );
            }
            let current_heading = e.get(this, BSFaceGenAnimationData::fCurrentEyeHeading);
            let step = x87_sub(
                x87_load32(e.mem.u32(heading.addr())),
                x87_m32(current_heading),
            );
            e.mem.set_u32(heading_step.addr(), x87_store32(step));
            let current_pitch = e.get(this, BSFaceGenAnimationData::fCurrentEyePitch);
            let step = x87_sub(x87_load32(e.mem.u32(pitch.addr())), x87_m32(current_pitch));
            e.mem.set_u32(pitch_step.addr(), x87_store32(step));
            let speed = setting_bits(e, F_TRACK_SPEED);
            let limit = x87_store32(x87_mul(x87_load32(speed), x87_m32(seconds.to_bits())));
            let limit = x87_quiet_bits(limit);
            let negative = limit ^ 0x8000_0000;
            e.call(CLAMP_FLOAT, &args![heading_step, negative, limit]);
            e.call(CLAMP_FLOAT, &args![pitch_step, negative, limit]);
            let current_heading = e.get(this, BSFaceGenAnimationData::fCurrentEyeHeading);
            let new_heading = x87_store32(x87_add(
                x87_load32(current_heading),
                x87_m32(e.mem.u32(heading_step.addr())),
            ));
            e.set(
                this,
                BSFaceGenAnimationData::fCurrentEyeHeading,
                new_heading,
            );
            let current_pitch = e.get(this, BSFaceGenAnimationData::fCurrentEyePitch);
            let new_pitch = x87_store32(x87_add(
                x87_load32(current_pitch),
                x87_m32(e.mem.u32(pitch_step.addr())),
            ));
            e.set(this, BSFaceGenAnimationData::fCurrentEyePitch, new_pitch);
        });
        let heading = e.get(this, BSFaceGenAnimationData::fCurrentEyeHeading);
        let pitch = e.get(this, BSFaceGenAnimationData::fCurrentEyePitch);
        bs_face_gen_animation_data_set_eye_position(
            e,
            this,
            f32::from_bits(x87_quiet_bits(heading)),
            f32::from_bits(x87_quiet_bits(pitch)),
        );
    }
    unlock(e, this);
    alive
}

/// The eye drift of `ModifyEyeHeadingAndPitchBasedOnExpression`: a random
/// heading offset in `[min, max]` of the two settings.
fn random_setting_range(e: &mut Engine, max_setting: u32, min_setting: u32) -> u32 {
    let max = setting_float(e, max_setting);
    let min = setting_float(e, min_setting);
    random_float(e, min, max)
}

// Translated from 0064bf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ModifyEyeHeadingAndPitchBasedOnExpression`
/// (Xbox PDB; matched by its signature and the fields it uses): counts
/// `fExpressionEyeOffsetTimer` down by `seconds`; when it
/// runs out (not above 0) picks a new drift for the expression (the exe's
/// two-level jump table at 0064c3fc/0064c3e4: expressions -1, 0, 8, 11 use
/// the angry eye settings, 2 and 4 the happy ones, 3, 9 and 12 the sad ones,
/// 5 the neutral ones, 1 and 6 a heading-only fear drift, 7, 10 and others
/// nothing) with a new random timer, and adds the drift
/// (`fExpressionEyeOffsetHeading`, `fExpressionEyeOffsetPitch`) to
/// `*heading` and `*pitch`.
pub fn bs_face_gen_animation_data_modify_eye_heading_and_pitch_based_on_expression(
    e: &mut Engine,
    this: FaceData,
    expression: i32,
    heading: Ptr,
    pitch: Ptr,
    seconds: f32,
) {
    use BSFaceGenAnimationData as F;
    let timer = e.get(this, F::fExpressionEyeOffsetTimer);
    let timer = x87_store32(x87_sub(x87_load32(timer), x87_m32(seconds.to_bits())));
    e.set(this, F::fExpressionEyeOffsetTimer, timer);
    if x87_load32(timer) <= e.global::<f64>(ZERO) {
        // Case index (expression + 1) through the byte map, then the target.
        let target = match expression.wrapping_add(1) as u32 {
            0 | 1 | 9 | 12 => 0,
            2 | 7 => 1,
            3 | 5 => 2,
            4 | 10 | 13 => 3,
            6 => 4,
            _ => 5,
        };
        match target {
            0 => {
                if random_chance(e, EYE_CHANCE) {
                    let (low, high) = (constant_bits(e, EYE_TIME_A), constant_bits(e, EYE_TIME_B));
                    let time = random_float(e, low, high);
                    e.set(this, F::fExpressionEyeOffsetTimer, time);
                    e.set(this, F::fExpressionEyeOffsetHeading, FLOAT_ZERO);
                    e.set(this, F::fExpressionEyeOffsetPitch, FLOAT_ZERO);
                } else {
                    let (low, high) = (constant_bits(e, EYE_TIME_C), constant_bits(e, EYE_TIME_D));
                    let time = random_float(e, low, high);
                    e.set(this, F::fExpressionEyeOffsetTimer, time);
                    let offset =
                        random_setting_range(e, F_EYE_PITCH_MAX_ANGRY, F_EYE_PITCH_MIN_ANGRY);
                    e.set(this, F::fExpressionEyeOffsetPitch, offset);
                    let offset =
                        random_setting_range(e, F_EYE_HEADING_MAX_ANGRY, F_EYE_HEADING_MIN_ANGRY);
                    e.set(this, F::fExpressionEyeOffsetHeading, offset);
                }
            }
            2 | 4 => {
                let (pitch_max, pitch_min, heading_max, heading_min) = if target == 2 {
                    (
                        F_EYE_PITCH_MAX_HAPPY,
                        F_EYE_PITCH_MIN_HAPPY,
                        F_EYE_HEADING_MAX_HAPPY,
                        F_EYE_HEADING_MIN_HAPPY,
                    )
                } else {
                    (
                        F_EYE_PITCH_MAX_NEUTRAL,
                        F_EYE_PITCH_MIN_NEUTRAL,
                        F_EYE_HEADING_MAX_NEUTRAL,
                        F_EYE_HEADING_MIN_NEUTRAL,
                    )
                };
                if random_chance(e, EYE_CHANCE) {
                    let (low, high) = (constant_bits(e, EYE_TIME_B), constant_bits(e, EYE_TIME_E));
                    let time = random_float(e, low, high);
                    e.set(this, F::fExpressionEyeOffsetTimer, time);
                    e.set(this, F::fExpressionEyeOffsetHeading, FLOAT_ZERO);
                    e.set(this, F::fExpressionEyeOffsetPitch, FLOAT_ZERO);
                } else {
                    let (low, high) = (constant_bits(e, EYE_TIME_C), constant_bits(e, EYE_TIME_D));
                    let time = random_float(e, low, high);
                    e.set(this, F::fExpressionEyeOffsetTimer, time);
                    let offset = random_setting_range(e, pitch_max, pitch_min);
                    e.set(this, F::fExpressionEyeOffsetPitch, offset);
                    let offset = random_setting_range(e, heading_max, heading_min);
                    e.set(this, F::fExpressionEyeOffsetHeading, offset);
                }
            }
            3 => {
                let (low, high) = (constant_bits(e, EYE_TIME_A), constant_bits(e, EYE_TIME_B));
                let time = random_float(e, low, high);
                e.set(this, F::fExpressionEyeOffsetTimer, time);
                if random_chance(e, EYE_CHANCE) {
                    e.set(this, F::fExpressionEyeOffsetHeading, FLOAT_ZERO);
                    e.set(this, F::fExpressionEyeOffsetPitch, FLOAT_ZERO);
                } else {
                    let offset = random_setting_range(e, F_EYE_PITCH_MAX_SAD, F_EYE_PITCH_MIN_SAD);
                    e.set(this, F::fExpressionEyeOffsetPitch, offset);
                    let offset =
                        random_setting_range(e, F_EYE_HEADING_MAX_SAD, F_EYE_HEADING_MIN_SAD);
                    e.set(this, F::fExpressionEyeOffsetHeading, offset);
                }
            }
            1 => {
                let (low, high) = (constant_bits(e, EYE_TIME_C), constant_bits(e, EYE_TIME_D));
                let time = random_float(e, low, high);
                e.set(this, F::fExpressionEyeOffsetTimer, time);
                e.set(this, F::fExpressionEyeOffsetPitch, FLOAT_ZERO);
                if random_chance(e, EYE_TIME_C) {
                    e.set(this, F::fExpressionEyeOffsetHeading, FLOAT_ZERO);
                } else {
                    let offset =
                        random_setting_range(e, F_EYE_HEADING_MAX_FEAR, F_EYE_HEADING_MIN_FEAR);
                    e.set(this, F::fExpressionEyeOffsetHeading, offset);
                }
            }
            _ => {}
        }
    }
    let drift = e.get(this, F::fExpressionEyeOffsetHeading);
    let value = x87_store32(x87_add(
        x87_load32(e.mem.u32(heading.addr())),
        x87_m32(drift),
    ));
    e.mem.set_u32(heading.addr(), value);
    let drift = e.get(this, F::fExpressionEyeOffsetPitch);
    let value = x87_store32(x87_add(x87_load32(e.mem.u32(pitch.addr())), x87_m32(drift)));
    e.mem.set_u32(pitch.addr(), value);
}

// Translated from 0064c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetEyePosition` (Xbox PDB; matched by its
/// signature and the fields it uses): when eye tracking is
/// on (`0064bcb0`), clamps `heading` and `pitch` into the eye ranges
/// (`GetTrackEyeXYMinMax`, `GetTrackEyeZMinMax`), writes them as modifier
/// values (a non-negative heading as value 10 over the maximum with 9 = 0, a
/// negative one or NaN as value 9 over the minimum with 10 = 0; the pitch
/// the same with 11 and 8) and stores them in `fCurrentEyeHeading` /
/// `fCurrentEyePitch`.
pub fn bs_face_gen_animation_data_set_eye_position(
    e: &mut Engine,
    this: FaceData,
    heading: f32,
    pitch: f32,
) {
    if fn_0064bcb0(e) == 0 {
        return;
    }
    let modifiers: Ptr<BSFaceGenKeyframeMultiple> = Ptr::new(field(this, 0xa8));
    e.with_stack(0x18, |e, locals| {
        // The stack arguments (clamped in place) and the four range locals.
        let heading_arg = locals;
        let pitch_arg = locals.byte_add(4);
        let xy_min = locals.byte_add(8);
        let xy_max = locals.byte_add(0xc);
        let z_min = locals.byte_add(0x10);
        let z_max = locals.byte_add(0x14);
        e.mem.set_u32(heading_arg.addr(), heading.to_bits());
        e.mem.set_u32(pitch_arg.addr(), pitch.to_bits());
        bs_face_gen_animation_data_get_track_eye_xy_min_max(e, xy_min, xy_max);
        bs_face_gen_animation_data_get_track_eye_z_min_max(e, z_min, z_max);
        let (low, high) = (
            x87_quiet_bits(e.mem.u32(xy_min.addr())),
            x87_quiet_bits(e.mem.u32(xy_max.addr())),
        );
        e.call(CLAMP_FLOAT, &args![heading_arg, low, high]);
        let (low, high) = (
            x87_quiet_bits(e.mem.u32(z_min.addr())),
            x87_quiet_bits(e.mem.u32(z_max.addr())),
        );
        e.call(CLAMP_FLOAT, &args![pitch_arg, low, high]);

        let zero = e.global::<f64>(ZERO);
        let heading = e.mem.u32(heading_arg.addr());
        if x87_load32(heading) >= zero {
            let ratio = x87_store32(x87_div(
                x87_load32(heading),
                x87_m32(e.mem.u32(xy_max.addr())),
            ));
            fn_0064a4d0(e, modifiers, 10, f32::from_bits(x87_quiet_bits(ratio)));
            fn_0064a4d0(e, modifiers, 9, 0.0);
        } else {
            fn_0064a4d0(e, modifiers, 10, 0.0);
            let ratio = x87_store32(x87_div(
                x87_load32(heading),
                x87_m32(e.mem.u32(xy_min.addr())),
            ));
            fn_0064a4d0(e, modifiers, 9, f32::from_bits(x87_quiet_bits(ratio)));
        }
        let pitch = e.mem.u32(pitch_arg.addr());
        if x87_load32(pitch) >= zero {
            let ratio = x87_store32(x87_div(x87_load32(pitch), x87_m32(e.mem.u32(z_max.addr()))));
            fn_0064a4d0(e, modifiers, 11, f32::from_bits(x87_quiet_bits(ratio)));
            fn_0064a4d0(e, modifiers, 8, 0.0);
        } else {
            fn_0064a4d0(e, modifiers, 11, 0.0);
            let ratio = x87_store32(x87_div(x87_load32(pitch), x87_m32(e.mem.u32(z_min.addr()))));
            fn_0064a4d0(e, modifiers, 8, f32::from_bits(x87_quiet_bits(ratio)));
        }
        e.set(
            this,
            BSFaceGenAnimationData::fCurrentEyeHeading,
            x87_quiet_bits(heading),
        );
        e.set(
            this,
            BSFaceGenAnimationData::fCurrentEyePitch,
            x87_quiet_bits(pitch),
        );
    });
}

// Translated from 0064ca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::UpdateKeyframeList` (Xbox PDB, static, cdecl;
/// matched by its signature and body):
/// advances `current` through the queued keyframes of `list` by `seconds`.
/// Returns 0 without a list or keyframe, or when the head keyframe's type
/// (slot +4) differs from `current`'s. With an empty list `current`'s time
/// is set to 0. Otherwise `current`'s time grows by `seconds`; every queued
/// keyframe whose time has been reached is merged in fully (slot +0x1C at
/// 1.0), its time taken off `current`'s, and deleted; a remaining head
/// keyframe is merged in by the fraction of its time reached, else
/// `current`'s time is set to 0. Returns whether any merge reported a
/// change.
pub fn fn_0064ca60(
    e: &mut Engine,
    seconds: f32,
    list: Ptr<NiTPointerList>,
    current: Ptr<BSFaceGenKeyframeMultiple>,
    flag: u8,
) -> u8 {
    let mut changed = 0u8;
    let (list, current) = (list.addr(), current.addr());
    if current == 0 || list == 0 {
        return 0;
    }
    if e.call(LIST_COUNT, &args![list]).u32() != 0 {
        let element = e.call(LIST_HEAD_ELEMENT, &args![list]).u32();
        if e.mem.u32(element) != 0 {
            let element = e.call(LIST_HEAD_ELEMENT, &args![list]).u32();
            let head = e.mem.u32(element);
            let head_type = e.vcall(head, KF_GET_TYPE, &[]).u32();
            let own_type = e.vcall(current, KF_GET_TYPE, &[]).u32();
            if head_type != own_type {
                return 0;
            }
        }
    }
    if e.call(LIST_COUNT, &args![list]).u32() == 0 {
        keyframe_set_time(e, current, FLOAT_ZERO);
        return 0;
    }
    let time = e.vcall(current, KF_GET_TIME, &[]).st0;
    let time = x87_store32(x87_add(time, x87_m32(seconds.to_bits())));
    keyframe_set_time(e, current, x87_quiet_bits(time));
    let element = e.call(LIST_HEAD_ELEMENT, &args![list]).u32();
    let mut keyframe = e.mem.u32(element);
    while keyframe != 0 {
        let own_time = e.vcall(current, KF_GET_TIME, &[]).st0;
        let its_time = e.vcall(keyframe, KF_GET_TIME, &[]).st0;
        // FCOMP: the loop goes on only while the queued time is not past
        // `current`'s (an unordered compare ends it).
        if !matches!(
            its_time.partial_cmp(&own_time),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        ) {
            break;
        }
        changed |= e
            .vcall(
                current,
                KF_MERGE,
                &args![keyframe, FLOAT_ONE, 1u32, u32::from(flag)],
            )
            .u8();
        let own_time = e.vcall(current, KF_GET_TIME, &[]).st0;
        let its_time = e.vcall(keyframe, KF_GET_TIME, &[]).st0;
        let rest = x87_store32(x87_sub(own_time, its_time));
        keyframe_set_time(e, current, x87_quiet_bits(rest));
        e.call(LIST_REMOVE_HEAD, &args![list]);
        if keyframe != 0 {
            e.vcall(keyframe, 0, &args![1u32]);
        }
        keyframe = list_head(e, list);
    }
    if keyframe != 0 {
        let own_time = e.vcall(current, KF_GET_TIME, &[]).st0;
        let its_time = e.vcall(keyframe, KF_GET_TIME, &[]).st0;
        let fraction = x87_store32(x87_div(own_time, its_time));
        changed |= e
            .vcall(
                current,
                KF_MERGE,
                &args![keyframe, x87_quiet_bits(fraction), 1u32, u32::from(flag)],
            )
            .u8();
    } else {
        keyframe_set_time(e, current, FLOAT_ZERO);
    }
    changed
}

// Translated from 0064cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetAnimHeadCulled` (Xbox PDB; matched by its
/// signature and the field it sets): when the value
/// changes, a newly culled head first resets the animation morphs at once
/// (vtable +0xBC, time 0, all four flags); stores `bAnimHeadCulled`.
pub fn bs_face_gen_animation_data_set_anim_head_culled(e: &mut Engine, this: FaceData, culled: u8) {
    if culled == e.get(this, BSFaceGenAnimationData::bAnimHeadCulled) {
        return;
    }
    if culled != 0 {
        e.vcall(
            this.addr(),
            VT_RESET_ANIM_CURRENT_MORPHS,
            &args![FLOAT_ZERO, 1u32, 1u32, 1u32, 1u32],
        );
    }
    e.set(this, BSFaceGenAnimationData::bAnimHeadCulled, culled);
}

// Translated from 0064ccf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ccf0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenImage::BSFaceGenImage` (Xbox PDB): the `NiRefObject` base
/// (`004968b0`), the vtable, an empty `spCompressedImage` (`00633c90`) and
/// `UncompressedImage` (`0065c750`), then `texture` assigned to the pointer
/// (`0066b0d0`); width and height from the texture (its vtable +0x94 and
/// +0x98) when the pointer holds one (`00559450`), else 0. Returns `this`.
pub fn bs_face_gen_image_constructor(
    e: &mut Engine,
    this: Ptr<BSFaceGenImage>,
    texture: Ptr,
) -> Ptr<BSFaceGenImage> {
    e.call(0x0049_68b0, &args![this]);
    e.mem.set_u32(this.addr(), FACE_GEN_IMAGE_VTABLE);
    let pointer = this.at(BSFaceGenImage::spCompressedImage).addr();
    e.call(0x0063_3c90, &args![pointer, 0u32]);
    let image = this.at(BSFaceGenImage::UncompressedImage).addr();
    e.call(0x0065_c750, &args![image]);
    e.call(0x0066_b0d0, &args![pointer, texture]);
    if e.call(0x0055_9450, &args![pointer]).u32() != 0 {
        let source = e.call(0x0055_9450, &args![pointer]).u32();
        let width = e.vcall(source, 0x94, &[]).u32();
        e.set(this, BSFaceGenImage::iWidth, width);
        let source = e.call(0x0055_9450, &args![pointer]).u32();
        let height = e.vcall(source, 0x98, &[]).u32();
        e.set(this, BSFaceGenImage::iHeight, height);
    } else {
        e.set(this, BSFaceGenImage::iHeight, 0u32);
        e.set(this, BSFaceGenImage::iWidth, 0u32);
    }
    this
}

/// `00649a50` (thiscall on a `BSFaceGenKeyframeMultiple`, `(index, value)`):
/// stores `value` at `pQuantities[index]` (by its body, unchecked).
const KEYFRAME_STORE_VALUE: u32 = 0x0064_9a50;
/// The point the head-tracking functions compare with and copy from
/// (`011f426c`, three floats; read at run time).
const HEAD_TRACK_ZERO_POINT: u32 = 0x011f_426c;
/// `004390c0` (thiscall `(const float *other)`): whether the three floats of
/// `this` equal those of `other` (by its body); `00439090` (thiscall, one
/// word) is its negation.
const POINT_EQUAL: u32 = 0x0043_90c0;
const POINT_NOT_EQUAL: u32 = 0x0043_9090;

/// The shared body of the `SetAnim*Value` setters: when `index` is below
/// `limit` (signed), stores `value` through `00649a50` in the keyframe at
/// `offset`.
fn set_anim_value(e: &mut Engine, this: FaceData, offset: u32, limit: i32, index: i32, value: f32) {
    if index < limit {
        let value = x87_quiet_bits(value.to_bits());
        e.call(
            KEYFRAME_STORE_VALUE,
            &args![field(this, offset), index, value],
        );
    }
}

// Translated from 00649be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetAnimExpressionValue` (Xbox PDB, vtable
/// +0xDC on PC): for `index < 15` (signed) stores `value` in
/// `CurrentAnimExpressionData` (`00649a50`).
pub fn bs_face_gen_animation_data_set_anim_expression_value(
    e: &mut Engine,
    this: FaceData,
    index: i32,
    value: f32,
) {
    set_anim_value(e, this, 0x24, 0xf, index, value);
}

// Translated from 00649c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetAnimModifierValue` (Xbox PDB, vtable +0xE0
/// on PC): for `index < 17` (signed) stores `value` in
/// `CurrentAnimModifierData` (`00649a50`).
pub fn bs_face_gen_animation_data_set_anim_modifier_value(
    e: &mut Engine,
    this: FaceData,
    index: i32,
    value: f32,
) {
    set_anim_value(e, this, 0x38, 0x11, index, value);
}

// Translated from 00649c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetAnimPhonemeValue` (Xbox PDB, vtable +0xE4 on
/// PC): for `index < 16` (signed) stores `value` in `CurrentAnimPhonemeData`
/// (`00649a50`).
pub fn bs_face_gen_animation_data_set_anim_phoneme_value(
    e: &mut Engine,
    this: FaceData,
    index: i32,
    value: f32,
) {
    set_anim_value(e, this, 0x4c, 0x10, index, value);
}

// Translated from 00649c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetAnimCustomMorphValue` (Xbox PDB, vtable
/// +0xE8 on PC): for `index < 1` (signed) stores `value` in
/// `CurrentAnimCustomMorphData` (`00649a50`).
pub fn bs_face_gen_animation_data_set_anim_custom_morph_value(
    e: &mut Engine,
    this: FaceData,
    index: i32,
    value: f32,
) {
    set_anim_value(e, this, 0x60, 1, index, value);
}

// Translated from 0064ac30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064ac30 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::AddCustomMorphKeyframe` (Xbox PDB, vtable +0x110
/// on PC): unless dead, `values` is null or `time` is negative (a NaN
/// passes), builds a custom-morph keyframe (type 3, one value) at `time`
/// with `values[0]` and adds it to `SpeechCustomMorphKeyframeList`.
pub fn bs_face_gen_animation_data_add_custom_morph_keyframe(
    e: &mut Engine,
    this: FaceData,
    values: Ptr,
    time: f32,
) {
    // `NV_SPEECH_CHECK` (diagnostic only): the face's speech lists are watched.
    speech_check::track_face(e, this.addr());
    with_stack_keyframe(e, 3, 1, |e, keyframe| {
        if e.get(this, BSFaceGenAnimationData::bDead) != 0
            || values.is_null()
            || x87_load32(time.to_bits()) < e.global::<f64>(ZERO)
        {
            return;
        }
        e.call(
            KEYFRAME_SET_TIME_DIRECT,
            &args![keyframe, x87_quiet_bits(time.to_bits())],
        );
        let value = x87_quiet_bits(e.mem.u32(values.addr()));
        fn_0064a4d0(e, keyframe, 0, f32::from_bits(value));
        let list = field(this, 0xf0);
        e.call(KEYFRAME_ADD_TO_LIST, &args![keyframe, list]);
    });
}

// Translated from 0064b0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::IsSpeechKeyframeListEmpty` (Xbox PDB, vtable
/// +0x108): whether the speech list of `morph_type` has no items (count
/// through `0044ddc0`): 0 phonemes (`SpeechPhonemeKeyframeList`), 2
/// modifiers (`SpeechModifierKeyframeList`), 3 custom morphs
/// (`SpeechCustomMorphKeyframeList`); true for any other type.
pub fn bs_face_gen_animation_data_is_speech_keyframe_list_empty(
    e: &mut Engine,
    this: FaceData,
    morph_type: u32,
) -> bool {
    let list = match morph_type {
        0 => 0xbc,
        2 => 0x88,
        3 => 0xf0,
        _ => return true,
    };
    e.call(LIST_COUNT, &args![field(this, list)]).u32() == 0
}

// Translated from 0064b160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064b160 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// `BSFaceGenAnimationData::SetExpressionTarget(const float *)` (Xbox PDB,
/// the second overload; vtable +0x118 on PC): when dead or `values` is null,
/// only clears the target (vtable +0x120). Otherwise deletes the old
/// `pTargetExpressionData` (scalar deleting destructor, flag 1; the field is
/// then overwritten), makes a new expression keyframe (0x14 bytes, type 1,
/// 15 values; null when the allocation fails), sets its 15 values from
/// `values` and clears the target when `CurrentTargetExpressionData`
/// compares false with it (slot +0x2C).
pub fn bs_face_gen_animation_data_set_expression_target_values(
    e: &mut Engine,
    this: FaceData,
    values: Ptr,
) {
    if e.get(this, BSFaceGenAnimationData::bDead) != 0 || values.is_null() {
        e.vcall(this.addr(), VT_CLEAR_EXPRESSION_TARGET, &[]);
        return;
    }
    let old = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    if old != 0 {
        e.vcall(old, 0, &args![1u32]);
    }
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let target = if block == 0 {
        0
    } else {
        e.call(KEYFRAME_CONSTRUCT, &args![block, 1u32, 0xfu32])
            .u32()
    };
    e.set(this, BSFaceGenAnimationData::pTargetExpressionData, target);
    for i in 0..0xfu32 {
        let value = x87_quiet_bits(e.mem.u32(values.addr().wrapping_add(4 * i)));
        let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
        fn_0064a4d0(e, Ptr::new(target), i, f32::from_bits(value));
    }
    finish_expression_target(e, this);
}

// Translated from 0064bcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::SetHeadTrackVector` (Xbox PDB, vtable +0xC0):
/// the vector arrives by value (`x`, `y`, `z`; the fourth word is not read).
/// When the current `HeadTrackVector` (+0x124) equals the point at
/// `011f426c` and the new one differs from it (`004390c0` / `00439090`), the
/// tracking state (+0x174) becomes 1 and its timer (+0x178) 0.0; when the
/// new vector equals that point the state becomes 0. Then stores the vector
/// and sets `bShouldHeadTrack`.
pub fn bs_face_gen_animation_data_set_head_track_vector(
    e: &mut Engine,
    this: FaceData,
    x: f32,
    y: f32,
    z: f32,
    _unused_4: u32,
) {
    // The by-value vector lives in the argument words; the comparisons take
    // its address.
    e.with_stack(0xc, |e, vector| {
        let vector = vector.addr();
        e.mem.set_u32(vector, x.to_bits());
        e.mem.set_u32(vector + 4, y.to_bits());
        e.mem.set_u32(vector + 8, z.to_bits());
        let current = field(this, 0x124);
        if e.call(POINT_EQUAL, &args![current, HEAD_TRACK_ZERO_POINT])
            .u8()
            != 0
            && e.call(POINT_NOT_EQUAL, &args![vector, HEAD_TRACK_ZERO_POINT])
                .u8()
                != 0
        {
            // BSFaceGenAnimationData::TrackingState (Xbox PDB) +0x174
            e.mem.set_u32(field(this, 0x174), 1);
            // BSFaceGenAnimationData::fJustAcquiredTimer (Xbox PDB) +0x178
            e.mem.set_u32(field(this, 0x178), FLOAT_ZERO);
        }
        if e.call(POINT_EQUAL, &args![vector, HEAD_TRACK_ZERO_POINT])
            .u8()
            != 0
        {
            e.mem.set_u32(field(this, 0x174), 0);
        }
    });
    // BSFaceGenAnimationData::HeadTrackVector (Xbox PDB) +0x124
    e.mem.set_u32(field(this, 0x124), x.to_bits());
    e.mem.set_u32(field(this, 0x128), y.to_bits());
    e.mem.set_u32(field(this, 0x12c), z.to_bits());
    e.set(this, BSFaceGenAnimationData::bShouldHeadTrack, 1u8);
}

// Translated from 0064bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenAnimationData::ClearHeadTracking` (Xbox PDB, vtable +0xC4):
/// copies the point at `011f426c` into `HeadTrackVector` (+0x124) and
/// stores `full_track_update` in `bFullTrackUpdate` (+0x188).
pub fn bs_face_gen_animation_data_clear_head_tracking(
    e: &mut Engine,
    this: FaceData,
    full_track_update: u8,
) {
    for word in [0u32, 4, 8] {
        let value = e.mem.u32(HEAD_TRACK_ZERO_POINT + word);
        e.mem.set_u32(field(this, 0x124 + word), value);
    }
    // BSFaceGenAnimationData::bFullTrackUpdate (Xbox PDB) +0x188
    e.mem.set_u8(field(this, 0x188), full_track_update);
}

/// `NiNew`-style allocation (`00aa13e0`, cdecl size) and the
/// `BSFaceGenAnimationData` constructor `00649680` the copy uses.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const FACE_GEN_ANIMATION_DATA_CONSTRUCT: u32 = 0x0064_9680;
/// `0064fcd0` (thiscall `(const BSFaceGenKeyframeMultiple *)`): copies a
/// keyframe into `this`; `0064ec10` (thiscall, one keyframe): the copy
/// constructor of a new keyframe.
const KEYFRAME_ASSIGN: u32 = 0x0064_fcd0;
const KEYFRAME_COPY_CONSTRUCT: u32 = 0x0064_ec10;
/// `NiTPointerList` helpers used by the copy: the head item (`00559450`),
/// the address of an item's element (`006317a0`, `(item)`), the next item
/// (`007b52d0`, `(item)`) and add-to-tail (`004ed8c0`, `(const T *)`).
const LIST_HEAD: u32 = 0x0055_9450;
const LIST_ITEM_ELEMENT: u32 = 0x0063_17a0;
const LIST_NEXT: u32 = 0x007b_52d0;
const LIST_ADD_TAIL: u32 = 0x004e_d8c0;
/// `BSFaceGenKeyframe::IsMultiple` (Xbox PDB slot +0x44).
const KF_IS_MULTIPLE: u32 = 0x44;

/// Copies a keyframe into a new 0x14-byte block (`0064ec10`), or null when
/// the allocation fails.
fn copy_keyframe(e: &mut Engine, keyframe: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        0
    } else {
        e.call(KEYFRAME_COPY_CONSTRUCT, &args![block, keyframe])
            .u32()
    }
}

// Translated from 0064c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0064c5a0 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// A copy of a `BSFaceGenAnimationData` (no Xbox name in the engine map;
/// called from `LinearFaceGenHeadLoad` and `TESNPC` code): a new object
/// (0x1C0 bytes through `00aa13e0`, constructed by `00649680`, null when
/// the allocation fails; the copy then writes through it unchecked); the
/// twelve current keyframes copied (`0064fcd0`, in the exe's order), the
/// head-track vector and the flags `bChangedLastUpdate`, `bFullTrackUpdate`,
/// `bMustUpdate`, `bAnimHeadCulled` and `bAnimHeadCulledLastFrame`; a copy of
/// `pTargetExpressionData` (or null); then each keyframe of the modifier,
/// phoneme and custom-morph speech lists, copied when its `IsMultiple`
/// (slot +0x44) is true and null otherwise, added to the same list of the
/// copy. Returns the copy.
pub fn fn_0064c5a0(e: &mut Engine, this: FaceData) -> FaceData {
    let block = e.call(NI_ALLOCATE, &args![0x1c0u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        e.call(FACE_GEN_ANIMATION_DATA_CONSTRUCT, &args![block])
            .u32()
    };
    // `NV_SPEECH_CHECK` (diagnostic only): both faces' speech lists are watched.
    speech_check::track_face(e, this.addr());
    speech_check::track_face(e, copy);
    for offset in [
        0x10u32, 0x24, 0x74, 0x94, 0x38, 0xa8, 0xc8, 0x4c, 0xdc, 0xfc, 0x60, 0x110,
    ] {
        e.call(
            KEYFRAME_ASSIGN,
            &args![copy.wrapping_add(offset), field(this, offset)],
        );
    }
    for offset in [0x124u32, 0x128, 0x12c] {
        let value = e.mem.u32(field(this, offset));
        e.mem.set_u32(copy.wrapping_add(offset), value);
    }
    for offset in [0x18bu32, 0x188, 0x18c, 0x18f, 0x190] {
        let value = e.mem.u8(field(this, offset));
        e.mem.set_u8(copy.wrapping_add(offset), value);
    }
    let target = e.get(this, BSFaceGenAnimationData::pTargetExpressionData);
    let target_copy = if target == 0 {
        0
    } else {
        copy_keyframe(e, target)
    };
    e.mem.set_u32(copy.wrapping_add(0xc), target_copy);
    for list in [0x88u32, 0xbc, 0xf0] {
        let source = field(this, list);
        let mut item = e.call(LIST_HEAD, &args![source]).u32();
        while item != 0 {
            let element = e.call(LIST_ITEM_ELEMENT, &args![source, item]).u32();
            let keyframe = e.mem.u32(element);
            let multiple = e.vcall(keyframe, KF_IS_MULTIPLE, &[]).u8() != 0;
            let added = if multiple {
                copy_keyframe(e, keyframe)
            } else {
                0
            };
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), added);
                e.call(LIST_ADD_TAIL, &args![copy.wrapping_add(list), slot]);
            });
            item = e.call(LIST_NEXT, &args![source, item]).u32();
        }
    }
    Ptr::new(copy)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_9ca0, fn_00649ca0(Ptr, u32) -> Ptr),
        entry!(
            0x0064_9ba0,
            bs_face_gen_animation_data_set_must_update(FaceData, u8)
        ),
        entry!(0x0064_9bc0, bs_face_gen_animation_data_get_dead(FaceData) -> u8),
        entry!(0x0064_9ce0, bs_face_gen_animation_data_destructor(FaceData)),
        entry!(0x0064_9e40, bs_face_gen_animation_data_init_sdm()),
        entry!(0x0064_9ec0, bs_face_gen_animation_data_kill_sdm()),
        entry!(
            0x0064_9f00,
            bs_face_gen_animation_data_get_track_eye_xy_min_max(Ptr, Ptr)
        ),
        entry!(
            0x0064_9f70,
            bs_face_gen_animation_data_get_track_eye_z_min_max(Ptr, Ptr)
        ),
        entry!(
            0x0064_9fe0,
            bs_face_gen_animation_data_get_track_dead_zone_xy_min_max(Ptr, Ptr)
        ),
        entry!(
            0x0064_a070,
            bs_face_gen_animation_data_get_track_dead_zone_z_min_max(Ptr, Ptr)
        ),
        entry!(0x0064_a100, bs_face_gen_animation_data_get_modifier_values_as_rotation(FaceData, Ptr) -> bool),
        entry!(0x0064_a290, fn_0064a290(Ptr, Ptr) -> bool),
        entry!(
            0x0064_a310,
            bs_face_gen_animation_data_reset_current_morphs(FaceData, f32, u8, u8, u8, u8, u8)
        ),
        entry!(
            0x0064_a4d0,
            fn_0064a4d0(Ptr<BSFaceGenKeyframeMultiple>, u32, f32) -> bool
        ),
        entry!(
            0x0064_a520,
            bs_face_gen_animation_data_reset_speech_current_morphs(
                FaceData, f32, u8, u8, u8, u8, u8
            )
        ),
        entry!(
            0x0064_a770,
            bs_face_gen_animation_data_reset_anim_current_morphs(FaceData, f32, u8, u8, u8, u8)
        ),
        entry!(
            0x0064_a920,
            bs_face_gen_animation_data_set_dead(FaceData, u8, u8)
        ),
        entry!(
            0x0064_aa00,
            bs_face_gen_animation_data_add_phoneme_keyframe(FaceData, Ptr, f32)
        ),
        entry!(
            0x0064_ab40,
            bs_face_gen_animation_data_add_modifiers_keyframe(FaceData, Ptr, f32)
        ),
        entry!(
            0x0064_ad20,
            bs_face_gen_animation_data_add_speech_empty_keyframe(FaceData, u32, f32, u32)
        ),
        entry!(
            0x0064_ae60,
            bs_face_gen_animation_data_clear_keyframe_lists(FaceData, u8, u8, u8, u8)
        ),
        entry!(
            0x0064_aec0,
            bs_face_gen_animation_data_clear_speech_keyframe_lists(FaceData, u8, u8, u8, u8)
        ),
        entry!(
            0x0064_b2b0,
            bs_face_gen_animation_data_set_expression_target(FaceData, i32, f32)
        ),
        entry!(
            0x0064_b400,
            bs_face_gen_animation_data_set_expression_target_from_mood(FaceData, u32, u32)
        ),
        entry!(
            0x0064_b5e0,
            bs_face_gen_animation_data_clear_expression_target(FaceData)
        ),
        entry!(0x0064_b630, bs_face_gen_animation_data_update(FaceData, f32) -> u8),
        entry!(0x0064_bcb0, fn_0064bcb0() -> u8),
        entry!(0x0064_bda0, bs_face_gen_animation_data_get_current_expression(FaceData) -> i32),
        entry!(0x0064_be40, bs_face_gen_animation_data_update_eye_tracking(FaceData, f32, u32) -> u8),
        entry!(
            0x0064_bf90,
            bs_face_gen_animation_data_modify_eye_heading_and_pitch_based_on_expression(
                FaceData, i32, Ptr, Ptr, f32
            )
        ),
        entry!(
            0x0064_c410,
            bs_face_gen_animation_data_set_eye_position(FaceData, f32, f32)
        ),
        entry!(
            0x0064_ca60,
            fn_0064ca60(f32, Ptr<NiTPointerList>, Ptr<BSFaceGenKeyframeMultiple>, u8) -> u8
        ),
        entry!(
            0x0064_cc90,
            bs_face_gen_animation_data_set_anim_head_culled(FaceData, u8)
        ),
        entry!(
            0x0064_ccf0,
            bs_face_gen_image_constructor(Ptr<BSFaceGenImage>, Ptr) -> Ptr<BSFaceGenImage>
        ),
        entry!(
            0x0064_9be0,
            bs_face_gen_animation_data_set_anim_expression_value(FaceData, i32, f32)
        ),
        entry!(
            0x0064_9c10,
            bs_face_gen_animation_data_set_anim_modifier_value(FaceData, i32, f32)
        ),
        entry!(
            0x0064_9c40,
            bs_face_gen_animation_data_set_anim_phoneme_value(FaceData, i32, f32)
        ),
        entry!(
            0x0064_9c70,
            bs_face_gen_animation_data_set_anim_custom_morph_value(FaceData, i32, f32)
        ),
        entry!(
            0x0064_ac30,
            bs_face_gen_animation_data_add_custom_morph_keyframe(FaceData, Ptr, f32)
        ),
        entry!(
            0x0064_b0f0,
            bs_face_gen_animation_data_is_speech_keyframe_list_empty(FaceData, u32) -> bool
        ),
        entry!(
            0x0064_b160,
            bs_face_gen_animation_data_set_expression_target_values(FaceData, Ptr)
        ),
        entry!(
            0x0064_bcc0,
            bs_face_gen_animation_data_set_head_track_vector(FaceData, f32, f32, f32, u32)
        ),
        entry!(
            0x0064_bd60,
            bs_face_gen_animation_data_clear_head_tracking(FaceData, u8)
        ),
        entry!(0x0064_c5a0, fn_0064c5a0(FaceData) -> FaceData),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn fn_00649ca0_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0064_9ca0, 0x0064_9ce0, 0x1c0);
    }

    const FACE_SLOTS: u32 = 0x7000_0000;
    const KEYFRAME_SLOTS: u32 = 0x7100_0000;
    const KEYFRAME_OFFSETS: [u32; 12] = [
        0x10, 0x24, 0x38, 0x4c, 0x60, 0x74, 0x94, 0xa8, 0xc8, 0xdc, 0xfc, 0x110,
    ];

    fn face_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_a000,
            0x0101_e000,
            0x0101_f000,
            0x0102_3000,
            0x0106_4000,
            0x011a_9000,
            0x011d_5000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        e.set_global(HALF_PI, std::f64::consts::FRAC_PI_2);
        e.set_global(MINUS_HALF_PI, -std::f64::consts::FRAC_PI_2);
        e.set_global(HALF_PI_FLOAT, std::f32::consts::FRAC_PI_2);
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(FLOAT_MAX, f64::from(f32::MAX));
        e.register(SETTING_VALUE_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SETTING_FLOAT, |e, a| {
            f64::from(e.mem.f32(a[0] + 4)).into_ret()
        });
        e.register(LOCK_ENTER, |_, _| Ret::default());
        e.register(LOCK_LEAVE, |_, _| Ret::default());
        // The list helpers on an `NiTPointerList` in memory (head, tail,
        // count; items next, previous, element).
        e.register(LIST_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LIST_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(LIST_HEAD_ELEMENT, |e, a| (e.mem.u32(a[0]) + 8).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let head = e.mem.u32(a[0]);
            let next = e.mem.u32(head);
            e.mem.set_u32(a[0], next);
            let count = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, count - 1);
            e.mem.u32(head + 8).into_ret()
        });
        e
    }

    fn set_setting(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn vtable(e: &mut Engine, base: u32, size: u32) -> u32 {
        let table = e.mem.alloc(size);
        for slot in (0..size).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
            e.register(base + slot, |_, _| Ret::default());
        }
        table
    }

    /// A keyframe whose vtable reads and writes its type and time in memory.
    fn keyframe_vtable(e: &mut Engine) -> u32 {
        let table = vtable(e, KEYFRAME_SLOTS, 0x50);
        e.register(KEYFRAME_SLOTS + KF_GET_TYPE, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(KEYFRAME_SLOTS + KF_GET_TIME, |e, a| {
            f64::from(e.mem.f32(a[0] + 8)).into_ret()
        });
        e.register(KEYFRAME_SLOTS + KF_SET_TIME, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        table
    }

    fn face(e: &mut Engine) -> FaceData {
        let this: FaceData = e.new_object();
        let table = vtable(e, FACE_SLOTS, 0x130);
        e.mem.set_u32(this.addr(), table);
        let keyframes = keyframe_vtable(e);
        for offset in KEYFRAME_OFFSETS {
            e.mem.set_u32(this.addr() + offset, keyframes);
        }
        this
    }

    /// Gives the keyframe at `keyframe` an array of `count` values.
    fn give_values(e: &mut Engine, keyframe: u32, count: u32) -> u32 {
        let values = e.mem.alloc(4 * count);
        e.mem.set_u32(keyframe + 0xc, values);
        e.mem.set_u32(keyframe + 0x10, count);
        values
    }

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    #[test]
    fn set_must_update_stores_the_byte() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call(0x0064_9ba0, &args![this, 2u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18c), 2);
    }

    #[test]
    fn get_dead_reads_the_byte() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.mem.set_u8(this.addr() + 0x18e, 1);
        assert_eq!(e.call(0x0064_9bc0, &args![this]).u8(), 1);
    }

    #[test]
    fn destructor_clears_lists_and_destroys_members_last_to_first() {
        let mut e = face_engine();
        let this = face(&mut e);
        for target in [KEYFRAME_DESTRUCT, 0x004a_1a30, 0x00a7_b300] {
            e.register(target, |_, _| Ret::default());
        }
        // The class vtable (put back first) with the two clear slots.
        e.mem.set_u32(
            FACE_GEN_ANIMATION_DATA_VTABLE + VT_CLEAR_SPEECH_KEYFRAME_LISTS,
            FACE_SLOTS + VT_CLEAR_SPEECH_KEYFRAME_LISTS,
        );
        e.mem.set_u32(
            FACE_GEN_ANIMATION_DATA_VTABLE + VT_CLEAR_ANIM_KEYFRAME_LISTS,
            FACE_SLOTS + VT_CLEAR_ANIM_KEYFRAME_LISTS,
        );
        e.call_log = Some(vec![]);
        e.call(0x0064_9ce0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), FACE_GEN_ANIMATION_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        let order: Vec<(u32, u32)> = log
            .iter()
            .filter(|(a, _)| matches!(*a, KEYFRAME_DESTRUCT | 0x004a_1a30 | 0x00a7_b300))
            .map(|(a, w)| (*a, w[0] - this.addr()))
            .collect();
        let k = KEYFRAME_DESTRUCT;
        let l = 0x004a_1a30;
        assert_eq!(
            order,
            vec![
                (k, 0x110),
                (k, 0xfc),
                (l, 0xf0),
                (k, 0xdc),
                (k, 0xc8),
                (l, 0xbc),
                (k, 0xa8),
                (k, 0x94),
                (l, 0x88),
                (k, 0x74),
                (k, 0x60),
                (k, 0x4c),
                (k, 0x38),
                (k, 0x24),
                (k, 0x10),
                (0x00a7_b300, 0),
            ]
        );
        // `ClearKeyframeLists(1, 1, 1, 1)` ran first.
        assert_eq!(
            log[1],
            (
                FACE_SLOTS + VT_CLEAR_SPEECH_KEYFRAME_LISTS,
                vec![this.addr(), 1, 1, 1, 1]
            )
        );
    }

    #[test]
    fn init_sdm_stores_the_constructed_tag_or_null() {
        let mut e = face_engine();
        e.register(ALLOCATE, |_, _| 0x0abc_0000u32.into_ret());
        e.register(0x0043_8170, |_, a| {
            assert_eq!(a[1], FACE_GEN_ANIMATION_TAG_TEXT);
            (a[0] + 1).into_ret()
        });
        e.call(0x0064_9e40, &[]);
        assert_eq!(e.global::<u32>(FACE_GEN_ANIMATION_TAG), 0x0abc_0001);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x0064_9e40, &[]);
        assert_eq!(e.global::<u32>(FACE_GEN_ANIMATION_TAG), 0);
    }

    #[test]
    fn kill_sdm_deletes_a_set_tag() {
        let mut e = face_engine();
        e.register(0x004b_9900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.set_global(FACE_GEN_ANIMATION_TAG, 0x1234u32);
        e.call(0x0064_9ec0, &[]);
        e.call(0x0064_9ec0, &[]);
        assert_eq!(calls_to(&e, 0x004b_9900), vec![vec![0x1234, 1]]);
        assert_eq!(e.global::<u32>(FACE_GEN_ANIMATION_TAG), 0);
    }

    fn range(e: &mut Engine, getter: u32) -> (u32, u32) {
        let pair = e.mem.alloc(8);
        e.call(getter, &args![pair, pair + 4]);
        (e.mem.u32(pair), e.mem.u32(pair + 4))
    }

    fn radians(degrees: f32) -> f32 {
        (f64::from(degrees) * f64::from_bits(0x3f91_df46_a000_0000)) as f32
    }

    #[test]
    fn track_eye_xy_range_is_clamped_to_a_quarter_turn() {
        let mut e = face_engine();
        set_setting(&mut e, F_TRACK_EYE_XY, 30.0);
        let (min, max) = range(&mut e, 0x0064_9f00);
        assert_eq!(max, bits(radians(30.0)));
        assert_eq!(min, bits(-radians(30.0)));
        set_setting(&mut e, F_TRACK_EYE_XY, -5.0);
        assert_eq!(range(&mut e, 0x0064_9f00), (bits(-0.0), 0));
        set_setting(&mut e, F_TRACK_EYE_XY, 100.0);
        let half_pi = std::f32::consts::FRAC_PI_2;
        assert_eq!(range(&mut e, 0x0064_9f00), (bits(-half_pi), bits(half_pi)));
    }

    #[test]
    fn track_eye_z_range_reads_its_own_setting() {
        let mut e = face_engine();
        set_setting(&mut e, F_TRACK_EYE_Z, 20.0);
        set_setting(&mut e, F_TRACK_EYE_XY, 70.0);
        let (min, max) = range(&mut e, 0x0064_9f70);
        assert_eq!((min, max), (bits(-radians(20.0)), bits(radians(20.0))));
    }

    #[test]
    fn dead_zone_xy_is_limited_by_the_eye_range() {
        let mut e = face_engine();
        set_setting(&mut e, F_TRACK_EYE_XY, 5.0);
        set_setting(&mut e, F_TRACK_DEAD_ZONE_XY, 10.0);
        let (min, max) = range(&mut e, 0x0064_9fe0);
        assert_eq!((min, max), (bits(-radians(5.0)), bits(radians(5.0))));
        set_setting(&mut e, F_TRACK_DEAD_ZONE_XY, 3.0);
        let (min, max) = range(&mut e, 0x0064_9fe0);
        assert_eq!((min, max), (bits(-radians(3.0)), bits(radians(3.0))));
    }

    #[test]
    fn dead_zone_z_is_limited_by_the_z_eye_range() {
        let mut e = face_engine();
        set_setting(&mut e, F_TRACK_EYE_Z, 4.0);
        set_setting(&mut e, F_TRACK_EYE_XY, 50.0);
        set_setting(&mut e, F_TRACK_DEAD_ZONE_Z, 12.0);
        let (min, max) = range(&mut e, 0x0064_a070);
        assert_eq!((min, max), (bits(-radians(4.0)), bits(radians(4.0))));
    }

    #[test]
    fn modifier_rotation_clamps_three_values_and_compares_the_result() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_VALUE, |_, a| match a[1] {
            0x10 => f64::from(f32::MAX).into_ret(),
            0xf => 3.0f64.into_ret(),
            _ => f64::NAN.into_ret(),
        });
        // The quaternion maker writes its three angles and a 1.
        e.register(0x00a6_e1f0, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            e.mem.set_f32(a[0] + 12, 1.0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.mem.alloc(16);
        let half_pi = std::f32::consts::FRAC_PI_2;
        let differs = e.call(0x0064_a100, &args![this, result]).bool();
        assert_eq!(
            calls_to(&e, 0x00a6_e1f0),
            vec![vec![result, 0, bits(half_pi), bits(-half_pi)]]
        );
        assert!(differs);
        // Equal to the reference quaternion: false.
        for (i, value) in [0.0f32, half_pi, -half_pi, 1.0].iter().enumerate() {
            e.mem.set_f32(ROTATION_REFERENCE + 4 * i as u32, *value);
        }
        assert!(!e.call(0x0064_a100, &args![this, result]).bool());
    }

    #[test]
    fn quaternion_inequality_compares_four_floats() {
        let mut e = face_engine();
        let a = e.mem.alloc(16);
        let b = e.mem.alloc(16);
        assert!(!e.call(0x0064_a290, &args![a, b]).bool());
        e.mem.set_f32(b + 12, 0.5);
        assert!(e.call(0x0064_a290, &args![a, b]).bool());
        e.mem.set_f32(b + 12, 0.0);
        e.mem.set_f32(a, f32::NAN);
        e.mem.set_f32(b, f32::NAN);
        assert!(e.call(0x0064_a290, &args![a, b]).bool());
    }

    #[test]
    fn reset_current_morphs_resets_both_and_clears_at_time_zero() {
        let mut e = face_engine();
        let this = face(&mut e);
        let values = give_values(&mut e, this.addr() + 0xa8, 17);
        e.call_log = Some(vec![]);
        e.call(
            0x0064_a310,
            &args![this, 0.0f32, 1u32, 1u32, 1u32, 1u32, 1u32],
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_RESET_SPEECH_CURRENT_MORPHS),
            vec![vec![this.addr(), 0, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_RESET_ANIM_CURRENT_MORPHS),
            vec![vec![this.addr(), 0, 1, 1, 1, 1]]
        );
        let cleared: Vec<u32> = calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA)
            .iter()
            .map(|w| w[0] - this.addr())
            .collect();
        assert_eq!(cleared, vec![0x74, 0xa8, 0xdc, 0x110]);
        assert_eq!(e.mem.f32(values), 1.0);
        assert_eq!(e.mem.f32(values + 4), 1.0);

        e.call_log = Some(vec![]);
        e.call(
            0x0064_a310,
            &args![this, 0.5f32, 1u32, 1u32, 1u32, 1u32, 1u32],
        );
        assert!(calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA).is_empty());
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn keyframe_value_setter_reports_a_change() {
        let mut e = face_engine();
        let keyframe = e.mem.alloc(0x14);
        let values = give_values(&mut e, keyframe, 2);
        assert!(!e.call(0x0064_a4d0, &args![keyframe, 2u32, 1.0f32]).bool());
        assert!(e.call(0x0064_a4d0, &args![keyframe, 1u32, 0.5f32]).bool());
        assert_eq!(e.mem.f32(values + 4), 0.5);
        assert!(!e.call(0x0064_a4d0, &args![keyframe, 1u32, 0.5f32]).bool());
        assert!(e.call(0x0064_a4d0, &args![keyframe, 1u32, f32::NAN]).bool());
        assert!(e.call(0x0064_a4d0, &args![keyframe, 1u32, f32::NAN]).bool());
    }

    #[test]
    fn reset_speech_morphs_eases_back_over_a_positive_time() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(FILL_VALUES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x0064_a520,
            &args![this, 0.5f32, 1u32, 1u32, 1u32, 1u32, 1u32],
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_KEYFRAME_LISTS),
            vec![vec![this.addr(), 1, 1, 1, 1]]
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_SET_EXPRESSION_TARGET),
            vec![vec![this.addr(), u32::MAX, FLOAT_ONE]]
        );
        let modifiers = calls_to(&e, FACE_SLOTS + VT_ADD_MODIFIERS_KEYFRAME);
        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0][2], bits(0.5));
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_ADD_SPEECH_EMPTY_KEYFRAME),
            vec![
                vec![this.addr(), 0, bits(0.5), 1],
                vec![this.addr(), 3, bits(0.5), 1]
            ]
        );
        assert!(calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA).is_empty());

        // Without the eye flag the modifiers get an empty keyframe (type 2).
        e.call_log = Some(vec![]);
        e.call(
            0x0064_a520,
            &args![this, 0.5f32, 0u32, 0u32, 1u32, 0u32, 0u32],
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_ADD_SPEECH_EMPTY_KEYFRAME),
            vec![vec![this.addr(), 2, bits(0.5), 1]]
        );
    }

    #[test]
    fn reset_speech_morphs_clears_at_once_without_time() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x0064_a520,
            &args![this, 0.0f32, 1u32, 1u32, 1u32, 1u32, 1u32],
        );
        let cleared: Vec<u32> = calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA)
            .iter()
            .map(|w| w[0] - this.addr())
            .collect();
        assert_eq!(cleared, vec![0x10, 0x94, 0xc8, 0xfc]);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_SET_MODIFIER_VALUE),
            vec![
                vec![this.addr(), 0, FLOAT_ONE],
                vec![this.addr(), 1, FLOAT_ONE]
            ]
        );
    }

    #[test]
    fn reset_anim_morphs_adds_empty_keyframes_or_clears() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_a770, &args![this, 0.25f32, 1u32, 1u32, 1u32, 1u32]);
        let types: Vec<u32> = calls_to(&e, FACE_SLOTS + VT_ADD_ANIM_EMPTY_KEYFRAME)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(types, vec![1, 2, 0, 3]);
        e.call_log = Some(vec![]);
        e.call(0x0064_a770, &args![this, -1.0f32, 0u32, 1u32, 0u32, 0u32]);
        let cleared: Vec<u32> = calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA)
            .iter()
            .map(|w| w[0] - this.addr())
            .collect();
        assert_eq!(cleared, vec![0x4c]);
    }

    #[test]
    fn set_dead_resets_morphs_on_each_change() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_a920, &args![this, 1u32, 1u32]);
        e.call(0x0064_a920, &args![this, 1u32, 1u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18e), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x18b), 1);
        e.call(0x0064_a920, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.u8(this.addr() + 0x18e), 0);
        assert_eq!(e.mem.u8(this.addr() + 0x189), 1);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_RESET_CURRENT_MORPHS),
            vec![
                vec![this.addr(), 0, 1, 1, 1, 1, 1],
                vec![this.addr(), FLOAT_ONE, 1, 1, 1, 1, 0]
            ]
        );
    }

    /// Doubles for the stack keyframe: the constructor gives it `count`
    /// values and its type, the others do nothing.
    fn stack_keyframe_doubles(e: &mut Engine) {
        e.register(KEYFRAME_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            let values = e.mem.alloc(4 * a[2]);
            e.mem.set_u32(a[0] + 0xc, values);
            e.mem.set_u32(a[0] + 0x10, a[2]);
            a[0].into_ret()
        });
        e.register(KEYFRAME_SET_TIME_DIRECT, |_, _| Ret::default());
        e.register(KEYFRAME_DESTRUCT, |_, _| Ret::default());
        // Records the values the keyframe holds when it is added.
        e.register(KEYFRAME_ADD_TO_LIST, |e, a| {
            let values = e.mem.u32(a[0] + 0xc);
            let last = e.mem.u32(a[0] + 0x10) - 1;
            e.mem.set_u32(0x011d_5000, e.mem.u32(values + 4 * last));
            Ret::default()
        });
    }

    #[test]
    fn add_phoneme_keyframe_checks_the_values_range() {
        let mut e = face_engine();
        let this = face(&mut e);
        stack_keyframe_doubles(&mut e);
        let values = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_f32(values + 4 * i, 0.5);
        }
        e.mem.set_f32(values + 60, 0.75);
        e.call_log = Some(vec![]);
        e.call(0x0064_aa00, &args![this, values, 2.0f32]);
        let added = calls_to(&e, KEYFRAME_ADD_TO_LIST);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], this.addr() + 0xbc);
        assert_eq!(e.mem.f32(0x011d_5000), 0.75);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT)[0][1..], [0, 0x10]);
        assert_eq!(calls_to(&e, KEYFRAME_SET_TIME_DIRECT)[0][1], bits(2.0));

        e.mem.set_f32(values + 8, 1.5);
        e.call_log = Some(vec![]);
        e.call(0x0064_aa00, &args![this, values, 2.0f32]);
        assert!(calls_to(&e, KEYFRAME_ADD_TO_LIST).is_empty());
        assert_eq!(calls_to(&e, KEYFRAME_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        e.mem.set_f32(values + 8, 0.5);
        e.call(0x0064_aa00, &args![this, values, -1.0f32]);
        assert!(calls_to(&e, KEYFRAME_ADD_TO_LIST).is_empty());
    }

    #[test]
    fn add_modifiers_keyframe_takes_seventeen_values_unchecked() {
        let mut e = face_engine();
        let this = face(&mut e);
        stack_keyframe_doubles(&mut e);
        let values = e.mem.alloc(0x44);
        e.mem.set_f32(values + 64, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x0064_ab40, &args![this, values, 0.5f32]);
        let added = calls_to(&e, KEYFRAME_ADD_TO_LIST);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], this.addr() + 0x88);
        assert_eq!(e.mem.f32(0x011d_5000), 7.0);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT)[0][1..], [2, 0x11]);
        e.mem.set_u8(this.addr() + 0x18e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0064_ab40, &args![this, values, 0.5f32]);
        assert!(calls_to(&e, KEYFRAME_ADD_TO_LIST).is_empty());
        assert_eq!(calls_to(&e, KEYFRAME_DESTRUCT).len(), 1);
    }

    #[test]
    fn add_speech_empty_keyframe_picks_the_list_by_type() {
        let mut e = face_engine();
        let this = face(&mut e);
        stack_keyframe_doubles(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_ad20, &args![this, 3u32, 1.0f32, 1u32]);
        e.call(0x0064_ad20, &args![this, 1u32, 1.0f32, 1u32]);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT)[0][1..], [3, 1]);
        let added = calls_to(&e, KEYFRAME_ADD_TO_LIST);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], this.addr() + 0xf0);
        e.mem.set_u8(this.addr() + 0x18e, 1);
        e.call(0x0064_ad20, &args![this, 0u32, 1.0f32, 1u32]);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT).len(), 1);
    }

    #[test]
    fn clear_keyframe_lists_clears_speech_then_anim() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_ae60, &args![this, 1u32, 0u32, 1u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.iter().skip(1).cloned().collect();
        assert_eq!(
            order,
            vec![
                (
                    FACE_SLOTS + VT_CLEAR_SPEECH_KEYFRAME_LISTS,
                    vec![this.addr(), 1, 0, 1, 0]
                ),
                (
                    FACE_SLOTS + VT_CLEAR_ANIM_KEYFRAME_LISTS,
                    vec![this.addr(), 1, 0, 1, 0]
                ),
            ]
        );
    }

    /// Puts keyframes (each with the keyframe vtable) into the list at `list`.
    fn fill_list(e: &mut Engine, list: u32, keyframes: &[u32]) {
        let mut next = 0;
        for keyframe in keyframes.iter().rev() {
            let item = e.mem.alloc(0xc);
            e.mem.set_u32(item, next);
            e.mem.set_u32(item + 8, *keyframe);
            next = item;
        }
        e.mem.set_u32(list, next);
        e.mem.set_u32(list + 8, keyframes.len() as u32);
    }

    fn new_keyframe(e: &mut Engine, this: FaceData, time: f32, morph_type: u32) -> u32 {
        let keyframe = e.mem.alloc(0x14);
        let table = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(keyframe, table);
        e.mem.set_u32(keyframe + 4, morph_type);
        e.mem.set_f32(keyframe + 8, time);
        keyframe
    }

    #[test]
    fn clear_speech_keyframe_lists_deletes_every_keyframe() {
        let mut e = face_engine();
        let this = face(&mut e);
        let first = new_keyframe(&mut e, this, 0.0, 2);
        let second = new_keyframe(&mut e, this, 0.0, 2);
        let third = new_keyframe(&mut e, this, 0.0, 3);
        fill_list(&mut e, this.addr() + 0x88, &[first, second]);
        fill_list(&mut e, this.addr() + 0xf0, &[third]);
        e.call_log = Some(vec![]);
        e.call(0x0064_aec0, &args![this, 1u32, 1u32, 1u32, 0u32]);
        assert_eq!(
            calls_to(&e, KEYFRAME_SLOTS),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x90), 0);
        assert_eq!(e.mem.u32(this.addr() + 0xf8), 1);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).len(),
            1
        );
    }

    #[test]
    fn set_expression_target_makes_the_target_and_sets_the_value() {
        let mut e = face_engine();
        let this = face(&mut e);
        let target = e.mem.alloc(0x14);
        let table = e.mem.u32(this.addr() + 0x10);
        e.mem.set_u32(target, table);
        let values = give_values(&mut e, target, 15);
        e.register_double(ALLOCATE, move |_, _| target.into_ret());
        e.register(KEYFRAME_CONSTRUCT, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0064_b2b0, &args![this, 16i32, 0.5f32]);
        e.call(0x0064_b2b0, &args![this, 3i32, 1.5f32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // No target yet: allocated, constructed (type 1, 15) and zeroed.
        e.call(0x0064_b2b0, &args![this, 3i32, 0.5f32]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), target);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT), vec![vec![target, 1, 0xf]]);
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS + KF_SET_TO_ZERO).len(), 1);
        assert_eq!(e.mem.f32(values + 12), 0.5);
        // `Compare` returned 0: the target is cleared.
        assert_eq!(
            calls_to(&e, KEYFRAME_SLOTS + KF_COMPARE),
            vec![vec![this.addr() + 0x10, target]]
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).len(),
            1
        );
        // An existing target is cleared; expression -1 sets nothing more.
        e.call_log = Some(vec![]);
        e.call(0x0064_b2b0, &args![this, -1i32, 0.5f32]);
        assert_eq!(
            calls_to(&e, KEYFRAME_SLOTS + KF_CLEAR_DATA),
            vec![vec![target, 1]]
        );
        assert!(calls_to(&e, KEYFRAME_SLOTS + KF_COMPARE).is_empty());
    }

    #[test]
    fn expression_target_from_mood_sets_mood_plus_five() {
        let mut e = face_engine();
        let this = face(&mut e);
        let target = new_keyframe(&mut e, this, 0.0, 1);
        let values = give_values(&mut e, target, 15);
        e.mem.set_u32(this.addr() + 0xc, target);
        e.register(KEYFRAME_SLOTS + KF_COMPARE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0064_b400, &args![this, 2u32, 0u32]);
        assert_eq!(e.mem.f32(values + 4 * 7), 1.0);
        e.call(0x0064_b400, &args![this, 9u32, 0u32]);
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS + KF_COMPARE).len(), 2);
        assert!(calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).is_empty());
        e.mem.set_u8(this.addr() + 0x18e, 1);
        e.call(0x0064_b400, &args![this, 0u32, 0u32]);
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS + KF_COMPARE).len(), 2);
        assert_eq!(e.mem.f32(values + 4 * 5), 0.0);
    }

    #[test]
    fn clear_expression_target_deletes_it() {
        let mut e = face_engine();
        let this = face(&mut e);
        let target = new_keyframe(&mut e, this, 0.0, 1);
        e.mem.set_u32(this.addr() + 0xc, target);
        e.call_log = Some(vec![]);
        e.call(0x0064_b5e0, &args![this]);
        e.call(0x0064_b5e0, &args![this]);
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS), vec![vec![target, 1]]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
    }

    #[test]
    fn update_without_time_only_copies_the_culled_flag() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_VALUE, |_, _| 0.0f64.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.mem.set_u8(this.addr() + 0x18b, 1);
        e.mem.set_u8(this.addr() + 0x18f, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_b630, &args![this, 0.0f32]).u8(), 1);
        assert_eq!(e.mem.u8(this.addr() + 0x190), 1);
        assert!(calls_to(&e, KEYFRAME_SLOTS + KF_UPDATE_TO_TARGET).is_empty());
        assert_eq!(calls_to(&e, 0x0068_15c0).len(), 1);
    }

    #[test]
    fn update_moves_the_expression_and_steps_the_lists() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_VALUE, |_, _| 0.0f64.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(KEYFRAME_SLOTS + KF_UPDATE_TO_TARGET, |_, _| 1u32.into_ret());
        e.mem.set_u32(this.addr() + 0xc, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_b630, &args![this, 0.1f32]).u8(), 1);
        assert_eq!(
            calls_to(&e, KEYFRAME_SLOTS + KF_UPDATE_TO_TARGET),
            vec![vec![this.addr() + 0x10, bits(0.1), 0x4444]]
        );
        let copies: Vec<(u32, u32)> = calls_to(&e, KEYFRAME_SLOTS + KF_COPY_FROM)
            .iter()
            .map(|w| (w[0] - this.addr(), w[1] - this.addr()))
            .collect();
        assert_eq!(copies, vec![(0x74, 0x10)]);
        // Each list was empty: the speech keyframe's time was zeroed.
        let zeroed: Vec<u32> = calls_to(&e, KEYFRAME_SLOTS + KF_SET_TIME)
            .iter()
            .map(|w| w[0] - this.addr())
            .collect();
        assert_eq!(zeroed, vec![0x94, 0xc8, 0xfc]);
        assert!(calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).is_empty());
    }

    #[test]
    fn update_with_nothing_moving_clears_the_target() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_VALUE, |_, _| 0.0f64.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.mem.set_u8(this.addr() + 0x18b, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_b630, &args![this, 0.1f32]).u8(), 0);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).len(),
            1
        );
        // Culled this frame and not the last: the animation keyframes
        // count as available and are copied in.
        e.mem.set_u8(this.addr() + 0x18f, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_b630, &args![this, 0.1f32]).u8(), 1);
        let copies: Vec<(u32, u32)> = calls_to(&e, KEYFRAME_SLOTS + KF_COPY_FROM)
            .iter()
            .map(|w| (w[0] - this.addr(), w[1] - this.addr()))
            .collect();
        assert_eq!(
            copies,
            vec![(0x74, 0x24), (0xa8, 0x38), (0xdc, 0x4c), (0x110, 0x60)]
        );
    }

    #[test]
    fn update_adds_a_blink() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_VALUE, |_, _| 0.0f64.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(FILL_VALUES, |_, _| Ret::default());
        e.register(0x005c_5420, |_, _| 0.5f64.into_ret());
        e.set_global(EYE_TRACKING_ENABLED, 1u8);
        set_setting(&mut e, F_BLINK_DOWN_TIME, 0.1);
        set_setting(&mut e, F_BLINK_UP_TIME, 0.2);
        set_setting(&mut e, F_BLINK_DELAY_MIN, 2.0);
        set_setting(&mut e, F_BLINK_DELAY_MAX, 6.0);
        set_setting(&mut e, F_LOOK_DOWN_DISABLE_BLINKING_AMT, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0064_b630, &args![this, 0.0f32]);
        let times: Vec<u32> = calls_to(&e, FACE_SLOTS + VT_ADD_MODIFIERS_KEYFRAME)
            .iter()
            .map(|w| w[2])
            .collect();
        assert_eq!(times, vec![bits(4.0), bits(0.1), bits(0.2)]);
        // Looking down far enough stops it.
        e.register(KEYFRAME_VALUE, |_, _| 0.75f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0064_b630, &args![this, 0.0f32]);
        assert!(calls_to(&e, FACE_SLOTS + VT_ADD_MODIFIERS_KEYFRAME).is_empty());
    }

    #[test]
    fn eye_tracking_switch_is_a_global_byte() {
        let mut e = face_engine();
        e.set_global(EYE_TRACKING_ENABLED, 3u8);
        assert_eq!(e.call(0x0064_bcb0, &[]).u8(), 3);
    }

    #[test]
    fn current_expression_is_the_strongest_in_range() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(FACE_SLOTS + VT_GET_EXPRESSION_VALUE, |_, a| {
            match a[1] as i32 {
                2 => 0.4f64,
                5 => 0.9,
                7 => 1.5,
                9 => 0.9,
                _ => 0.0,
            }
            .into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_bda0, &args![this]).i32(), 5);
        let asked: Vec<i32> = calls_to(&e, FACE_SLOTS + VT_GET_EXPRESSION_VALUE)
            .iter()
            .map(|w| w[1] as i32)
            .collect();
        assert_eq!(asked, (-1..15).collect::<Vec<_>>());
        e.register(FACE_SLOTS + VT_GET_EXPRESSION_VALUE, |_, _| {
            0.0f64.into_ret()
        });
        assert_eq!(e.call(0x0064_bda0, &args![this]).i32(), -1);
    }

    fn clamp_double(e: &mut Engine) {
        e.register(CLAMP_FLOAT, |e, a| {
            let value = e.mem.f32(a[0]);
            let (min, max) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            if value > max {
                e.mem.set_f32(a[0], max);
            } else if value < min {
                e.mem.set_f32(a[0], min);
            }
            Ret::default()
        });
    }

    #[test]
    fn eye_tracking_moves_the_eyes_at_the_track_speed() {
        let mut e = face_engine();
        let this = face(&mut e);
        clamp_double(&mut e);
        e.register(FACE_SLOTS + VT_GET_EXPRESSION_VALUE, |_, _| {
            0.0f64.into_ret()
        });
        set_setting(&mut e, F_TRACK_SPEED, 2.0);
        e.mem.set_f32(this.addr() + 0x17c, 10.0);
        e.mem.set_f32(this.addr() + 0x150, 1.0);
        e.mem.set_f32(this.addr() + 0x154, -1.0);
        assert_eq!(e.call(0x0064_be40, &args![this, 0.125f32, 0u32]).u8(), 1);
        assert_eq!(e.mem.f32(this.addr() + 0x140), 0.25);
        assert_eq!(e.mem.f32(this.addr() + 0x144), -0.25);
        assert_eq!(e.mem.f32(this.addr() + 0x17c), 9.875);
        e.mem.set_u8(this.addr() + 0x18e, 1);
        assert_eq!(e.call(0x0064_be40, &args![this, 0.125f32, 0u32]).u8(), 0);
        assert_eq!(e.mem.f32(this.addr() + 0x140), 0.25);
    }

    #[test]
    fn eye_drift_is_added_until_the_timer_runs_out() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(RANDOM_CHANCE, |_, _| 0u32.into_ret());
        // The random value is the middle of the range.
        e.register(RANDOM_FLOAT, |_, a| {
            ((f64::from(f32::from_bits(a[0])) + f64::from(f32::from_bits(a[1]))) / 2.0).into_ret()
        });
        e.mem.set_f32(EYE_TIME_C, 4.0);
        e.mem.set_f32(EYE_TIME_D, 6.0);
        set_setting(&mut e, F_EYE_PITCH_MIN_HAPPY, 0.0);
        set_setting(&mut e, F_EYE_PITCH_MAX_HAPPY, 0.5);
        set_setting(&mut e, F_EYE_HEADING_MIN_HAPPY, -1.0);
        set_setting(&mut e, F_EYE_HEADING_MAX_HAPPY, 0.0);
        let pair = e.mem.alloc(8);
        e.mem.set_f32(pair, 1.0);
        e.mem.set_f32(pair + 4, 2.0);
        e.mem.set_f32(this.addr() + 0x17c, 1.0);
        e.mem.set_f32(this.addr() + 0x180, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x0064_bf90, &args![this, 2i32, pair, pair + 4, 0.25f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x17c), 0.75);
        assert_eq!(e.mem.f32(pair), 1.5);
        assert!(calls_to(&e, RANDOM_FLOAT).is_empty());
        // Expression 2 (happy) once the timer runs out.
        e.call(0x0064_bf90, &args![this, 2i32, pair, pair + 4, 1.0f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x17c), 5.0);
        assert_eq!(e.mem.f32(this.addr() + 0x184), 0.25);
        assert_eq!(e.mem.f32(this.addr() + 0x180), -0.5);
        assert_eq!(e.mem.f32(pair), 1.0);
        assert_eq!(e.mem.f32(pair + 4), 2.25);
        // Expression 7 picks nothing new.
        e.call_log = Some(vec![]);
        e.call(0x0064_bf90, &args![this, 7i32, pair, pair + 4, 10.0f32]);
        assert!(calls_to(&e, RANDOM_FLOAT).is_empty());
        // Expression 6 (fear): heading only, chance asked with `EYE_TIME_C`.
        e.call(0x0064_bf90, &args![this, 6i32, pair, pair + 4, 10.0f32]);
        assert_eq!(calls_to(&e, RANDOM_CHANCE), vec![vec![bits(4.0)]]);
        assert_eq!(e.mem.f32(this.addr() + 0x184), 0.0);
    }

    #[test]
    fn eye_position_becomes_modifier_values() {
        let mut e = face_engine();
        let this = face(&mut e);
        clamp_double(&mut e);
        let values = give_values(&mut e, this.addr() + 0xa8, 17);
        e.set_global(EYE_TRACKING_ENABLED, 1u8);
        set_setting(&mut e, F_TRACK_EYE_XY, 45.0);
        set_setting(&mut e, F_TRACK_EYE_Z, 30.0);
        e.call(0x0064_c410, &args![this, 0.5f32, -1.0f32]);
        let xy = radians(45.0);
        let z = radians(30.0);
        assert_eq!(e.mem.f32(values + 40), (0.5f64 / f64::from(xy)) as f32);
        assert_eq!(e.mem.f32(values + 36), 0.0);
        assert_eq!(e.mem.f32(values + 44), 0.0);
        assert_eq!(e.mem.f32(values + 32), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x140), 0.5);
        assert_eq!(e.mem.f32(this.addr() + 0x144), -z);
        e.set_global(EYE_TRACKING_ENABLED, 0u8);
        e.call(0x0064_c410, &args![this, 0.0f32, 0.0f32]);
        assert_eq!(e.mem.f32(this.addr() + 0x140), 0.5);
    }

    #[test]
    fn keyframe_list_update_merges_reached_keyframes() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_SLOTS + KF_MERGE, |_, _| 1u32.into_ret());
        let list = this.addr() + 0x88;
        let current = this.addr() + 0x94;
        e.mem.set_u32(current + 4, 2);
        assert_eq!(
            e.call(0x0064_ca60, &args![0.25f32, 0u32, current, 0u32])
                .u8(),
            0
        );
        // Empty list: the time goes back to 0.
        e.mem.set_f32(current + 8, 3.0);
        assert_eq!(
            e.call(0x0064_ca60, &args![0.25f32, list, current, 0u32])
                .u8(),
            0
        );
        assert_eq!(e.mem.f32(current + 8), 0.0);
        // A head of another type: nothing.
        let other = new_keyframe(&mut e, this, 0.1, 0);
        fill_list(&mut e, list, &[other]);
        assert_eq!(
            e.call(0x0064_ca60, &args![0.25f32, list, current, 0u32])
                .u8(),
            0
        );
        // One reached keyframe (0.1 of 0.25) and one partly reached (1.0).
        let reached = new_keyframe(&mut e, this, 0.1, 2);
        let pending = new_keyframe(&mut e, this, 1.0, 2);
        fill_list(&mut e, list, &[reached, pending]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0064_ca60, &args![0.25f32, list, current, 1u32])
                .u8(),
            1
        );
        let merges = calls_to(&e, KEYFRAME_SLOTS + KF_MERGE);
        assert_eq!(merges.len(), 2);
        assert_eq!(merges[0], vec![current, reached, FLOAT_ONE, 1, 1]);
        let rest = (0.25f64 - f64::from(0.1f32)) as f32;
        assert_eq!(e.mem.f32(current + 8), rest);
        assert_eq!(
            merges[1],
            vec![current, pending, bits((f64::from(rest) / 1.0) as f32), 1, 1]
        );
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS), vec![vec![reached, 1]]);
        assert_eq!(e.mem.u32(list + 8), 1);
    }

    #[test]
    fn anim_head_culled_resets_anim_morphs_once() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0064_cc90, &args![this, 1u32]);
        e.call(0x0064_cc90, &args![this, 1u32]);
        e.call(0x0064_cc90, &args![this, 0u32]);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_RESET_ANIM_CURRENT_MORPHS),
            vec![vec![this.addr(), 0, 1, 1, 1, 1]]
        );
        assert_eq!(e.mem.u8(this.addr() + 0x18f), 0);
    }

    #[test]
    fn face_gen_image_takes_its_size_from_the_texture() {
        let mut e = face_engine();
        for target in [0x0049_68b0, 0x0063_3c90, 0x0065_c750] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        let texture = e.mem.alloc(8);
        e.put_vtable(0x7200_0000, &[0; 0x28]);
        e.mem.set_u32(0x7200_0000 + 0x94, 0x7300_0094);
        e.mem.set_u32(0x7200_0000 + 0x98, 0x7300_0098);
        e.register(0x7300_0094, |_, _| 64u32.into_ret());
        e.register(0x7300_0098, |_, _| 32u32.into_ret());
        e.mem.set_u32(texture, 0x7200_0000);
        let image: Ptr<BSFaceGenImage> = e.new_object();
        let result = e.call(0x0064_ccf0, &args![image, texture]).u32();
        assert_eq!(result, image.addr());
        assert_eq!(e.mem.u32(image.addr()), FACE_GEN_IMAGE_VTABLE);
        assert_eq!(e.mem.u32(image.addr() + 0x24), 64);
        assert_eq!(e.mem.u32(image.addr() + 0x28), 32);
        e.call(0x0064_ccf0, &args![image, 0u32]);
        assert_eq!(e.mem.u32(image.addr() + 0x24), 0);
        assert_eq!(e.mem.u32(image.addr() + 0x28), 0);
    }

    #[test]
    fn set_anim_values_store_below_each_limit() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.register(KEYFRAME_STORE_VALUE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for (address, offset, limit) in [
            (0x0064_9be0, 0x24, 15i32),
            (0x0064_9c10, 0x38, 17),
            (0x0064_9c40, 0x4c, 16),
            (0x0064_9c70, 0x60, 1),
        ] {
            e.call(address, &args![this, limit - 1, 0.5f32]);
            e.call(address, &args![this, limit, 0.5f32]);
            e.call(address, &args![this, -2i32, 0.25f32]);
            let stored = calls_to(&e, KEYFRAME_STORE_VALUE);
            let n = stored.len();
            assert_eq!(
                stored[n - 2..],
                [
                    vec![this.addr() + offset, (limit - 1) as u32, bits(0.5)],
                    vec![this.addr() + offset, -2i32 as u32, bits(0.25)],
                ]
            );
        }
        assert_eq!(calls_to(&e, KEYFRAME_STORE_VALUE).len(), 8);
    }

    #[test]
    fn add_custom_morph_keyframe_takes_one_value() {
        let mut e = face_engine();
        let this = face(&mut e);
        stack_keyframe_doubles(&mut e);
        let values = e.mem.alloc(4);
        e.mem.set_f32(values, 0.25);
        e.call_log = Some(vec![]);
        e.call(0x0064_ac30, &args![this, values, f32::NAN]);
        let added = calls_to(&e, KEYFRAME_ADD_TO_LIST);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][1], this.addr() + 0xf0);
        assert_eq!(e.mem.f32(0x011d_5000), 0.25);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT)[0][1..], [3, 1]);
        e.call_log = Some(vec![]);
        e.call(0x0064_ac30, &args![this, values, -0.5f32]);
        e.call(0x0064_ac30, &args![this, 0u32, 0.5f32]);
        assert!(calls_to(&e, KEYFRAME_ADD_TO_LIST).is_empty());
        assert_eq!(calls_to(&e, KEYFRAME_DESTRUCT).len(), 2);
    }

    #[test]
    fn speech_list_emptiness_by_type() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.mem.set_u32(this.addr() + 0xbc + 8, 1);
        let empty = |e: &mut Engine, t: u32| e.call(0x0064_b0f0, &args![this, t]).u8();
        assert_eq!(empty(&mut e, 0), 0);
        assert_eq!(empty(&mut e, 2), 1);
        assert_eq!(empty(&mut e, 3), 1);
        assert_eq!(empty(&mut e, 1), 1);
        e.mem.set_u32(this.addr() + 0x88 + 8, 2);
        assert_eq!(empty(&mut e, 2), 0);
    }

    #[test]
    fn expression_target_from_values_replaces_the_target() {
        let mut e = face_engine();
        let this = face(&mut e);
        let target = new_keyframe(&mut e, this, 0.0, 1);
        let target_values = give_values(&mut e, target, 15);
        e.register_double(ALLOCATE, move |_, _| target.into_ret());
        e.register(KEYFRAME_CONSTRUCT, |_, a| a[0].into_ret());
        let values = e.mem.alloc(60);
        e.mem.set_f32(values + 4 * 14, 0.75);
        // Dead: only the clear.
        e.mem.set_u8(this.addr() + 0x18e, 1);
        e.call_log = Some(vec![]);
        e.call(0x0064_b160, &args![this, values]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).len(),
            1
        );
        e.mem.set_u8(this.addr() + 0x18e, 0);
        let old = new_keyframe(&mut e, this, 0.0, 1);
        e.mem.set_u32(this.addr() + 0xc, old);
        e.call_log = Some(vec![]);
        e.call(0x0064_b160, &args![this, values]);
        assert_eq!(calls_to(&e, KEYFRAME_SLOTS), vec![vec![old, 1]]);
        assert_eq!(calls_to(&e, KEYFRAME_CONSTRUCT), vec![vec![target, 1, 0xf]]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), target);
        assert_eq!(e.mem.f32(target_values + 4 * 14), 0.75);
        assert_eq!(
            calls_to(&e, KEYFRAME_SLOTS + KF_COMPARE),
            vec![vec![this.addr() + 0x10, target]]
        );
        assert_eq!(
            calls_to(&e, FACE_SLOTS + VT_CLEAR_EXPRESSION_TARGET).len(),
            1
        );
    }

    fn point_doubles(e: &mut Engine) {
        e.map(0x011f_4000, 0x1000);
        e.register(POINT_EQUAL, |e, a| {
            let same = (0..3).all(|i| e.mem.f32(a[0] + 4 * i) == e.mem.f32(a[1] + 4 * i));
            same.into_ret()
        });
        e.register(POINT_NOT_EQUAL, |e, a| {
            let same = (0..3).all(|i| e.mem.f32(a[0] + 4 * i) == e.mem.f32(a[1] + 4 * i));
            (!same).into_ret()
        });
    }

    #[test]
    fn head_track_vector_starts_and_stops_tracking() {
        let mut e = face_engine();
        let this = face(&mut e);
        point_doubles(&mut e);
        e.mem.set_f32(this.addr() + 0x178, 3.0);
        e.call(0x0064_bcc0, &args![this, 1.0f32, 2.0f32, 3.0f32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x174), 1);
        assert_eq!(e.mem.f32(this.addr() + 0x178), 0.0);
        assert_eq!(e.mem.f32(this.addr() + 0x12c), 3.0);
        assert_eq!(e.mem.u8(this.addr() + 0x189), 1);
        // From a set vector to another: the state stays.
        e.mem.set_u32(this.addr() + 0x174, 5);
        e.call(0x0064_bcc0, &args![this, 4.0f32, 2.0f32, 3.0f32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x174), 5);
        assert_eq!(e.mem.f32(this.addr() + 0x124), 4.0);
        // Back to the zero point: the state becomes 0.
        e.call(0x0064_bcc0, &args![this, 0.0f32, 0.0f32, 0.0f32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0x174), 0);
    }

    #[test]
    fn clear_head_tracking_copies_the_zero_point() {
        let mut e = face_engine();
        let this = face(&mut e);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(HEAD_TRACK_ZERO_POINT + 8, 9.0);
        e.mem.set_f32(this.addr() + 0x124, 1.0);
        e.call(0x0064_bd60, &args![this, 1u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x124), 0.0);
        assert_eq!(e.mem.f32(this.addr() + 0x12c), 9.0);
        assert_eq!(e.mem.u8(this.addr() + 0x188), 1);
    }

    #[test]
    fn copy_duplicates_keyframes_flags_and_lists() {
        let mut e = face_engine();
        let this = face(&mut e);
        let copy = face(&mut e);
        e.register_double(NI_ALLOCATE, move |_, _| copy.addr().into_ret());
        e.register(FACE_GEN_ANIMATION_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(KEYFRAME_ASSIGN, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(KEYFRAME_COPY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 4, e.mem.u32(a[1] + 4));
            a[0].into_ret()
        });
        e.register(LIST_HEAD, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_ITEM_ELEMENT, |_, a| (a[1] + 8).into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[1]).into_ret());
        let added = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(LIST_ADD_TAIL, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        // A keyframe is multiple when its type is not 9.
        e.register(KEYFRAME_SLOTS + KF_IS_MULTIPLE, |e, a| {
            (e.mem.u32(a[0] + 4) != 9).into_ret()
        });
        e.mem.set_f32(this.addr() + 0x128, 5.0);
        e.mem.set_u8(this.addr() + 0x190, 1);
        let target = new_keyframe(&mut e, this, 0.0, 1);
        e.mem.set_u32(this.addr() + 0xc, target);
        let multiple = new_keyframe(&mut e, this, 0.0, 2);
        let single = new_keyframe(&mut e, this, 0.0, 9);
        fill_list(&mut e, this.addr() + 0x88, &[multiple, single]);
        e.call_log = Some(vec![]);
        let result = e.call(0x0064_c5a0, &args![this]).u32();
        assert_eq!(result, copy.addr());
        let assigned = calls_to(&e, KEYFRAME_ASSIGN);
        assert_eq!(assigned.len(), 12);
        assert_eq!(assigned[2], vec![copy.addr() + 0x74, this.addr() + 0x74]);
        assert_eq!(e.mem.f32(copy.addr() + 0x128), 5.0);
        assert_eq!(e.mem.u8(copy.addr() + 0x190), 1);
        let copied_target = e.mem.u32(copy.addr() + 0xc);
        assert_ne!(copied_target, 0);
        assert_eq!(e.mem.u32(copied_target + 4), 1);
        let added = added.borrow();
        assert_eq!(added.len(), 2);
        assert_eq!(added[0].0, copy.addr() + 0x88);
        assert_eq!(e.mem.u32(added[0].1 + 4), 2);
        assert_eq!(added[1], (copy.addr() + 0x88, 0));
    }
}
