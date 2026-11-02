//! `NiAnimation/bsrotaccumtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSRotAccumTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ee0;

// Translated from 00a43ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_rot_accum_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI)
}

/// `00a3f7b0`: the jump target of `00a43a70`.
const JUMP_TARGET_00A43A70: u32 = 0x00a3_f7b0;

/// `00a3f870`: the jump target of `00a43a80`.
const JUMP_TARGET_00A43A80: u32 = 0x00a3_f870;

/// `00a3f8a0`: the jump target of `00a43a90`.
const JUMP_TARGET_00A43A90: u32 = 0x00a3_f8a0;

/// `00a3f8d0`: the jump target of `00a43aa0`.
const JUMP_TARGET_00A43AA0: u32 = 0x00a3_f8d0;

// Translated from 00a43a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a3f7b0` (engine map: `NiTransformInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSRotAccumTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43A70, &args![this, stream]);
}

// Translated from 00a43a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a3f870` (engine map: `NiTransformInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSRotAccumTransfInterpolator@@UAA_NAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A43A80, &args![this, stream]).bool()
}

// Translated from 00a43a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a3f8a0` (engine map: `NiTransformInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSRotAccumTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43A90, &args![this, stream]);
}

// Translated from 00a43aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a3f8d0` (engine map: `NiTransformInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSRotAccumTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_rot_accum_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A43AA0, &args![this, other]).bool()
}

/// `BSRotAccumTransfInterpolator`'s vtable (`01097af4`) and size.
const ROT_ACCUM_VTABLE: u32 = 0x0109_7af4;
const ROT_ACCUM_SIZE: u32 = 0x58;
/// `operator new` (`00aa13e0`, `cdecl (size)`) and the sized delete
/// (`00aa1460`, `cdecl (block, size)`).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiTransformInterpolator`'s constructor (`00a3fa30`, `thiscall`, one
/// word) and `CopyMembers` (`00a40360`, `thiscall (this, dest, cloning
/// process)`); `NiKeyBasedInterpolator`'s destructor (`00a3ca30`).
const TRANSFORM_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_fa30;
const TRANSFORM_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a4_0360;
const KEY_BASED_DESTRUCTOR: u32 = 0x00a3_ca30;
/// The base `LinkObject` this class calls (`00a43330`, a jump to
/// `00a95ca0`; the engine map names the folded thunk
/// `BSBlendTreadTransfInterpolator::LinkObject`), `thiscall (this, stream)`.
const BASE_LINK_OBJECT: u32 = 0x00a4_3330;
/// The identity quaternion (4 words, `011a9ea4`).
const IDENTITY_ROTATE: u32 = 0x011a_9ea4;
/// `InterlockedDecrement` (import slot, one word: the address).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `-FLT_MAX` (`double`, `010241b0`): the invalid marker of transform parts.
const LOWEST: u32 = 0x0102_41b0;
/// The interpolator's key-data getters, each `thiscall (this, &count,
/// &type, &size)` answering the keys (count from a `u16`, type a word,
/// size a byte; all zero and null without data): translation `00a3f990`,
/// rotation `0058cb00` and scale `00a3f9e0` (the engine map's names for
/// these folded bodies are wrong).
const GET_TRANSLATE_DATA: u32 = 0x00a3_f990;
const GET_ROTATE_DATA: u32 = 0x0058_cb00;
const GET_SCALE_DATA: u32 = 0x00a3_f9e0;
/// The key interpolations (`cdecl`): `NiPosKey::GenInterp` (`00a27490`,
/// `(&result, time, keys, type, count, &last index, size)`, answers the
/// result), `NiRotKey::GenInterp` (`00a28740`, the same with a quaternion)
/// and `NiFloatKey::GenInterp` (`00a26b40`, without the result pointer,
/// the float in ST0).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
const ROT_KEY_GEN_INTERP: u32 = 0x00a2_8740;
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiQuaternion::UnitInverse` (`00a6dc00`, `cdecl (&result, q)`),
/// `operator*` (`00a2bcf0`, `thiscall (this, &result, other)`),
/// `FromRotation` (`00a6df40`, `thiscall (this, &matrix)`) and `Normalize`
/// (`00a38560`, `thiscall`).
const QUATERNION_UNIT_INVERSE: u32 = 0x00a6_dc00;
const QUATERNION_PRODUCT: u32 = 0x00a2_bcf0;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const QUATERNION_NORMALIZE: u32 = 0x00a3_8560;

/// `NiTransformInterpolator` fields (Xbox PDB, PC offsets): `m_fLastTime`
/// +8, `m_kTransformValue` +0xC (`NiQuatTransform`: translate +0, rotate
/// +0xC, scale +0x1C), `m_spData` +0x2C, `m_usLastTransIdx` +0x30,
/// `m_usLastRotIdx` +0x32, `m_usLastScaleIdx` +0x34, `bPose` +0x44; this
/// class's `PrevKeyRot` +0x48 (`NiQuaternion`).
const LAST_TIME: u32 = 0x8;
const TRANSFORM_VALUE: u32 = 0xc;
const DATA: u32 = 0x2c;
const LAST_TRANS_INDEX: u32 = 0x30;
const LAST_ROT_INDEX: u32 = 0x32;
const LAST_SCALE_INDEX: u32 = 0x34;
const POSE: u32 = 0x44;
const PREV_KEY_ROT: u32 = 0x48;

fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), value);
    }
}

/// The interpolator's new-object fields: the class's vtable and the
/// identity quaternion in `PrevKeyRot`.
fn init_rot_accum(e: &mut Engine, object: u32) {
    e.mem.set_u32(object, ROT_ACCUM_VTABLE);
    copy_words(e, IDENTITY_ROTATE, object.wrapping_add(PREV_KEY_ROT), 4);
}

// Translated from 00a43e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class factory (no Ghidra function; the engine map has no name):
/// `operator new(0x58)`, the `NiTransformInterpolator` constructor (with
/// 0), the class's vtable and the identity rotation in `PrevKeyRot`. Null
/// when the allocation fails.
pub fn fn_00a43e40(e: &mut Engine) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![ROT_ACCUM_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(TRANSFORM_INTERPOLATOR_CONSTRUCTOR, &args![object, 0u32]);
    init_rot_accum(e, object);
    Ptr::new(object)
}

// Translated from 00a43ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::LinkObject` (Xbox PDB): the base
/// `LinkObject` (`00a43330`), then, when there is transform data with
/// rotation keys (`u16` count at data +8, keys at data +0x20),
/// `PrevKeyRot` takes the first key's quaternion (the 4 words after its
/// time).
pub fn bs_rot_accum_transf_interpolator_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let data = e.mem.u32(this.addr() + DATA);
    if data == 0 {
        return;
    }
    let count = e.mem.u16(data.wrapping_add(8));
    let keys = e.mem.u32(data.wrapping_add(0x20));
    if count != 0 {
        copy_words(e, keys.wrapping_add(4), this.addr() + PREV_KEY_ROT, 4);
    }
}

/// Whether the transform at `transform` is not wholly invalid: its scale,
/// its rotation's second float or its translation's first float differs
/// from `-FLT_MAX` (x87 equality: a NaN differs).
fn transform_is_valid(e: &Engine, transform: u32) -> bool {
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    let scale = e.mem.f32(transform.wrapping_add(0x1c)) as f64;
    if scale != lowest {
        return true;
    }
    let rotate = e.mem.f32(transform.wrapping_add(0x10)) as f64;
    if rotate != lowest {
        return true;
    }
    let translate = e.mem.f32(transform) as f64;
    translate != lowest
}

// Translated from 00a43b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::Update` (Xbox PDB; `thiscall (time,
/// target, &value)`, `RET 0xC`): with `bPose` the last time is set and the
/// current transform copied to `value` (true). At the last time the
/// current transform is copied and answered valid unless it is wholly
/// invalid. Otherwise the translation and scale keys are interpolated
/// into the transform (each key's last index kept), and the rotation keys
/// too: with a target, the rotation becomes the target's local rotation
/// (as a quaternion) times the key rotation times the inverse of
/// `PrevKeyRot`, normalized; without one, the key rotation itself; either
/// way `PrevKeyRot` takes the key rotation. The transform is copied to
/// `value`; unless it is wholly invalid the last time is set and true
/// answered.
///
/// The key-data getters write a byte into the size local; its other three
/// bytes, which the exe leaves as whatever its stack held, are zero here
/// (the interpolations read only the byte). The scale type goes into the
/// stack slot of `target`, as in the exe, after `target` was last read.
pub fn bs_rot_accum_transf_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let time_bits = x87_quiet_bits(time.to_bits());
    if e.mem.u8(object + POSE) != 0 {
        e.mem.set_u32(object + LAST_TIME, time_bits);
        copy_words(e, object + TRANSFORM_VALUE, value.addr(), 8);
        return true;
    }
    if time == e.mem.f32(object + LAST_TIME) {
        copy_words(e, object + TRANSFORM_VALUE, value.addr(), 8);
        return transform_is_valid(e, object + TRANSFORM_VALUE);
    }
    // The frame mirrors the exe's stack from its `ESP` after the pushes:
    // last index +0xC, count +0x10, size +0x14, type +0x18, a quaternion
    // (or the interpolated point) +0x1C, the key rotation +0x2C, the
    // product with the inverse +0x3C, the inverse +0x4C, products +0x5C,
    // the `target` slot +0x74.
    e.with_stack(0x78, |e, frame| {
        let frame = frame.addr();
        let last_index = frame + 0xc;
        let count = frame + 0x10;
        let size = frame + 0x14;
        let kind = frame + 0x18;
        let rotation = frame + 0x1c;
        let key_rotation = frame + 0x2c;
        let delta = frame + 0x3c;
        let inverse = frame + 0x4c;
        let product = frame + 0x5c;
        let target_slot = frame + 0x74;
        e.mem.set_u32(target_slot, target.addr());

        let keys = e
            .call(GET_TRANSLATE_DATA, &args![object, count, kind, size])
            .u32();
        let key_count = e.mem.u32(count);
        if key_count != 0 {
            let index = u32::from(e.mem.u16(object + LAST_TRANS_INDEX));
            e.mem.set_u32(last_index, index);
            let point = e
                .call(
                    POS_KEY_GEN_INTERP,
                    &args![
                        rotation,
                        time_bits,
                        keys,
                        e.mem.u32(kind),
                        key_count,
                        last_index,
                        e.mem.u32(size)
                    ],
                )
                .u32();
            copy_words(e, point, object + TRANSFORM_VALUE, 3);
            let index = e.mem.u16(last_index);
            e.mem.set_u16(object + LAST_TRANS_INDEX, index);
        }

        let keys = e
            .call(GET_ROTATE_DATA, &args![object, count, kind, size])
            .u32();
        let key_count = e.mem.u32(count);
        if key_count != 0 {
            let index = u32::from(e.mem.u16(object + LAST_ROT_INDEX));
            e.mem.set_u32(last_index, index);
            let result = e
                .call(
                    ROT_KEY_GEN_INTERP,
                    &args![
                        delta,
                        time_bits,
                        keys,
                        e.mem.u32(kind),
                        key_count,
                        last_index,
                        e.mem.u32(size)
                    ],
                )
                .u32();
            copy_words(e, result, key_rotation, 4);
            if e.mem.u32(target_slot) != 0 {
                let unit_inverse = e
                    .call(
                        QUATERNION_UNIT_INVERSE,
                        &args![inverse, object + PREV_KEY_ROT],
                    )
                    .u32();
                let result = e
                    .call(
                        QUATERNION_PRODUCT,
                        &args![key_rotation, product, unit_inverse],
                    )
                    .u32();
                copy_words(e, result, delta, 4);
                let matrix = e.mem.u32(target_slot).wrapping_add(0x34);
                e.call(QUATERNION_FROM_ROTATION, &args![rotation, matrix]);
                let result = e
                    .call(QUATERNION_PRODUCT, &args![rotation, product, delta])
                    .u32();
                copy_words(e, result, rotation, 4);
                e.call(QUATERNION_NORMALIZE, &args![rotation]);
                copy_words(e, rotation, object + TRANSFORM_VALUE + 0xc, 4);
            } else {
                copy_words(e, key_rotation, object + TRANSFORM_VALUE + 0xc, 4);
            }
            copy_words(e, key_rotation, object + PREV_KEY_ROT, 4);
            let index = e.mem.u16(last_index);
            e.mem.set_u16(object + LAST_ROT_INDEX, index);
        }

        let keys = e
            .call(GET_SCALE_DATA, &args![object, count, target_slot, size])
            .u32();
        let key_count = e.mem.u32(count);
        if key_count != 0 {
            let index = u32::from(e.mem.u16(object + LAST_SCALE_INDEX));
            e.mem.set_u32(last_index, index);
            let scale = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![
                        time_bits,
                        keys,
                        e.mem.u32(target_slot),
                        key_count,
                        last_index,
                        e.mem.u32(size)
                    ],
                )
                .f64();
            e.mem
                .set_u32(object + TRANSFORM_VALUE + 0x1c, x87_store32(scale));
            let index = e.mem.u16(last_index);
            e.mem.set_u16(object + LAST_SCALE_INDEX, index);
        }
    });
    copy_words(e, object + TRANSFORM_VALUE, value.addr(), 8);
    if !transform_is_valid(e, object + TRANSFORM_VALUE) {
        return false;
    }
    e.mem.set_u32(object + LAST_TIME, time_bits);
    true
}

// Translated from 00a43df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::scalar deleting destructor` (Xbox PDB):
/// the class's vtable written and the `NiTransformInterpolator` destructor
/// inlined (the transform data released: `InterlockedDecrement` of its
/// count and `DeleteThis`, vtable +4, at zero), the `NiKeyBasedInterpolator`
/// destructor, then the sized delete of 0x58 bytes when bit 0 of `flags`
/// is set. Answers `this`.
pub fn bs_rot_accum_transf_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let object = this.addr();
    e.mem.set_u32(object, ROT_ACCUM_VTABLE);
    let data = e.mem.u32(object + DATA);
    if data != 0
        && e.call(INTERLOCKED_DECREMENT, &args![data.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(data, 4, &args![]);
    }
    e.call(KEY_BASED_DESTRUCTOR, &args![object]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![object, ROT_ACCUM_SIZE]);
    }
    this
}

// Translated from 00a43e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::CreateClone` (Xbox PDB): a new object as
/// the factory builds it (null when the allocation fails), then the
/// `NiTransformInterpolator` members copied into it (`00a40360`) and its
/// `PrevKeyRot` set to this one's (into address 0 when the allocation
/// failed, as the exe does). Answers the new object.
pub fn bs_rot_accum_transf_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let object = e.call(OPERATOR_NEW, &args![ROT_ACCUM_SIZE]).u32();
    if object != 0 {
        e.call(TRANSFORM_INTERPOLATOR_CONSTRUCTOR, &args![object, 0u32]);
        init_rot_accum(e, object);
    }
    e.call(
        TRANSFORM_INTERPOLATOR_COPY_MEMBERS,
        &args![this, object, cloning],
    );
    copy_words(
        e,
        this.addr() + PREV_KEY_ROT,
        object.wrapping_add(PREV_KEY_ROT),
        4,
    );
    Ptr::new(object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a43ab0, bs_rot_accum_transf_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_3a70,
            bs_rot_accum_transf_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_3a80, bs_rot_accum_transf_interpolator_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_3a90,
            bs_rot_accum_transf_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_3aa0, bs_rot_accum_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_3e40, fn_00a43e40() -> Ptr),
        entry!(
            0x00a4_3ac0,
            bs_rot_accum_transf_interpolator_link_object(Ptr, Ptr)
        ),
        entry!(0x00a4_3b00, bs_rot_accum_transf_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
        entry!(0x00a4_3df0, bs_rot_accum_transf_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_3e90, bs_rot_accum_transf_interpolator_create_clone(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_rot_accum_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a43ab0, &args![]).u32(),
            BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a43a70_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3a70;
        const TARGET: u32 = 0x00a3_f7b0;
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
    fn thunk_00a43a80_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3a80;
        const TARGET: u32 = 0x00a3_f870;
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
    fn thunk_00a43a90_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3a90;
        const TARGET: u32 = 0x00a3_f8a0;
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
    fn thunk_00a43aa0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3aa0;
        const TARGET: u32 = 0x00a3_f8d0;
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

    type RotCallList = Vec<Vec<u32>>;

    fn rot_calls_to(e: &Engine, addr: u32) -> RotCallList {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn rot_engine() -> Engine {
        let mut e = Engine::new();
        e.map(IDENTITY_ROTATE, 0x10);
        for i in 0..4 {
            e.set_global(IDENTITY_ROTATE + 4 * i, 0x900 + i);
        }
        e.map(LOWEST, 8);
        e.set_global(LOWEST, (-(f32::MAX as f64)).to_bits());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for addr in [
            SIZED_DELETE,
            TRANSFORM_INTERPOLATOR_CONSTRUCTOR,
            TRANSFORM_INTERPOLATOR_COPY_MEMBERS,
            KEY_BASED_DESTRUCTOR,
            BASE_LINK_OBJECT,
            QUATERNION_NORMALIZE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn rot_accum_factory_builds_with_the_identity_previous_rotation() {
        let mut e = rot_engine();
        let object = e.call(0x00a4_3e40, &args![]).u32();
        assert_eq!(rot_calls_to(&e, OPERATOR_NEW), vec![vec![0x58]]);
        assert_eq!(
            rot_calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![object, 0]]
        );
        assert_eq!(e.mem.u32(object), ROT_ACCUM_VTABLE);
        for i in 0..4 {
            assert_eq!(e.mem.u32(object + PREV_KEY_ROT + 4 * i), 0x900 + i);
        }
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_3e40, &args![]).u32(), 0);
        assert!(rot_calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn rot_accum_link_object_takes_the_first_rotation_key() {
        let mut e = rot_engine();
        let this = e.mem.alloc(0x58);
        // No data: only the base call.
        e.call(0x00a4_3ac0, &args![this, 0x6000u32]);
        assert_eq!(rot_calls_to(&e, BASE_LINK_OBJECT), vec![vec![this, 0x6000]]);
        let data = e.mem.alloc(0x28);
        let keys = e.mem.alloc(0x14);
        for i in 0..4 {
            e.mem.set_u32(keys + 4 + 4 * i, 0x400 + i);
        }
        e.mem.set_u32(data + 0x20, keys);
        e.mem.set_u32(this + DATA, data);
        // No rotation keys: unchanged.
        e.call(0x00a4_3ac0, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(this + PREV_KEY_ROT), 0);
        e.mem.set_u16(data + 8, 1);
        e.call(0x00a4_3ac0, &args![this, 0x6000u32]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + PREV_KEY_ROT + 4 * i), 0x400 + i);
        }
    }

    fn rot_set_transform(e: &mut Engine, this: u32, value: f32) {
        for i in 0..8 {
            e.mem.set_f32(this + TRANSFORM_VALUE + 4 * i, value);
        }
    }

    #[test]
    fn rot_accum_update_in_pose_or_at_the_last_time_copies_the_transform() {
        let mut e = rot_engine();
        let this = e.mem.alloc(0x58);
        let value = e.mem.alloc(0x20);
        rot_set_transform(&mut e, this, 2.0);
        e.mem.set_u8(this + POSE, 1);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.5f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.5);
        assert_eq!(e.mem.f32(value + 0x1c), 2.0);
        // Same time, not posed: valid unless wholly invalid.
        e.mem.set_u8(this + POSE, 0);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.5f32, 0u32, value])
            .bool());
        rot_set_transform(&mut e, this, -f32::MAX);
        assert!(!e
            .call(0x00a4_3b00, &args![this, 0.5f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(value), -f32::MAX);
        // Only the translation's first float valid.
        e.mem.set_f32(this + TRANSFORM_VALUE, 1.0);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.5f32, 0u32, value])
            .bool());
        assert!(rot_calls_to(&e, GET_TRANSLATE_DATA).is_empty());
    }

    /// Doubles for the key getters (each with `count` keys, type 2, size 7,
    /// keys at `0x3000 + n`) and the interpolations.
    fn rot_update_engine(count: u32) -> Engine {
        let mut e = rot_engine();
        for (getter, keys) in [
            (GET_TRANSLATE_DATA, 0x3001u32),
            (GET_ROTATE_DATA, 0x3002),
            (GET_SCALE_DATA, 0x3003),
        ] {
            e.register_double(getter, move |e, a| {
                e.mem.set_u32(a[1], count);
                e.mem.set_u32(a[2], 2);
                e.mem.set_u8(a[3], 7);
                keys.into_ret()
            });
        }
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[0] + 4 * i, 10.0 + i as f32);
            }
            e.mem.set_u32(a[5], 11);
            a[0].into_ret()
        });
        e.register(ROT_KEY_GEN_INTERP, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0x500 + i);
            }
            e.mem.set_u32(a[5], 12);
            a[0].into_ret()
        });
        e.register(FLOAT_KEY_GEN_INTERP, |e, a| {
            e.mem.set_u32(a[4], 13);
            3.5f32.into_ret()
        });
        e.register(QUATERNION_UNIT_INVERSE, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0x600 + i);
            }
            a[0].into_ret()
        });
        e.register_double(QUATERNION_PRODUCT, |e, a| {
            for i in 0..4 {
                let value = e.mem.u32(a[0] + 4 * i) ^ e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0x7000 + i);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn rot_accum_update_interpolates_each_channel_without_a_target() {
        let mut e = rot_update_engine(4);
        let this = e.mem.alloc(0x58);
        let value = e.mem.alloc(0x20);
        e.mem.set_u16(this + LAST_TRANS_INDEX, 1);
        e.mem.set_u16(this + LAST_ROT_INDEX, 2);
        e.mem.set_u16(this + LAST_SCALE_INDEX, 3);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.75f32, 0u32, value])
            .bool());
        let pos = rot_calls_to(&e, POS_KEY_GEN_INTERP);
        assert_eq!(pos.len(), 1);
        assert_eq!(
            (pos[0][1], pos[0][2], pos[0][3], pos[0][4], pos[0][6]),
            (0.75f32.to_bits(), 0x3001, 2, 4, 7)
        );
        let float = rot_calls_to(&e, FLOAT_KEY_GEN_INTERP);
        assert_eq!(
            (
                float[0][0],
                float[0][1],
                float[0][2],
                float[0][3],
                float[0][5]
            ),
            (0.75f32.to_bits(), 0x3003, 2, 4, 7)
        );
        assert_eq!(e.mem.f32(this + TRANSFORM_VALUE), 10.0);
        assert_eq!(e.mem.f32(this + TRANSFORM_VALUE + 8), 12.0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + TRANSFORM_VALUE + 0xc + 4 * i), 0x500 + i);
            assert_eq!(e.mem.u32(this + PREV_KEY_ROT + 4 * i), 0x500 + i);
        }
        assert_eq!(e.mem.f32(this + TRANSFORM_VALUE + 0x1c), 3.5);
        assert_eq!(e.mem.u16(this + LAST_TRANS_INDEX), 11);
        assert_eq!(e.mem.u16(this + LAST_ROT_INDEX), 12);
        assert_eq!(e.mem.u16(this + LAST_SCALE_INDEX), 13);
        assert_eq!(e.mem.f32(value + 0x1c), 3.5);
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.75);
        assert!(rot_calls_to(&e, QUATERNION_PRODUCT).is_empty());
    }

    #[test]
    fn rot_accum_update_with_a_target_accumulates_the_rotation() {
        let mut e = rot_update_engine(4);
        let this = e.mem.alloc(0x58);
        let value = e.mem.alloc(0x20);
        let target = e.mem.alloc(0x9c);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.75f32, target, value])
            .bool());
        let from = rot_calls_to(&e, QUATERNION_FROM_ROTATION);
        assert_eq!(from[0][1], target + 0x34);
        assert_eq!(
            rot_calls_to(&e, QUATERNION_UNIT_INVERSE)[0][1],
            this + PREV_KEY_ROT
        );
        // (target) ^ (key ^ inverse), normalized in place.
        let normalized = rot_calls_to(&e, QUATERNION_NORMALIZE)[0][0];
        assert_eq!(normalized, from[0][0]);
        for i in 0..4 {
            assert_eq!(
                e.mem.u32(this + TRANSFORM_VALUE + 0xc + 4 * i),
                (0x7000 + i) ^ (0x500 + i) ^ (0x600 + i)
            );
            assert_eq!(e.mem.u32(this + PREV_KEY_ROT + 4 * i), 0x500 + i);
        }
    }

    #[test]
    fn rot_accum_update_without_keys_answers_false_for_an_invalid_transform() {
        let mut e = rot_update_engine(0);
        let this = e.mem.alloc(0x58);
        let value = e.mem.alloc(0x20);
        rot_set_transform(&mut e, this, -f32::MAX);
        e.mem.set_f32(this + LAST_TIME, 0.25);
        assert!(!e
            .call(0x00a4_3b00, &args![this, 0.75f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.25);
        assert!(rot_calls_to(&e, POS_KEY_GEN_INTERP).is_empty());
        assert!(rot_calls_to(&e, ROT_KEY_GEN_INTERP).is_empty());
        assert!(rot_calls_to(&e, FLOAT_KEY_GEN_INTERP).is_empty());
        // A NaN scale counts as valid.
        e.mem.set_u32(this + TRANSFORM_VALUE + 0x1c, 0x7fc0_0000);
        assert!(e
            .call(0x00a4_3b00, &args![this, 0.75f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.75);
    }

    #[test]
    fn rot_accum_scalar_deleting_destructor_releases_the_data() {
        const DELETE_THIS: u32 = 0x0300_0010;
        let mut e = rot_engine();
        e.register(DELETE_THIS, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, DELETE_THIS);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 4, 1);
        let this = e.mem.alloc(0x58);
        e.mem.set_u32(this + DATA, data);
        assert_eq!(e.call(0x00a4_3df0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ROT_ACCUM_VTABLE);
        assert_eq!(rot_calls_to(&e, DELETE_THIS), vec![vec![data]]);
        assert_eq!(rot_calls_to(&e, KEY_BASED_DESTRUCTOR), vec![vec![this]]);
        assert!(rot_calls_to(&e, SIZED_DELETE).is_empty());
        e.mem.set_u32(this + DATA, 0);
        e.call(0x00a4_3df0, &args![this, 1u32]);
        assert_eq!(rot_calls_to(&e, SIZED_DELETE), vec![vec![this, 0x58]]);
    }

    #[test]
    fn rot_accum_create_clone_copies_the_members_and_previous_rotation() {
        let mut e = rot_engine();
        let this = e.mem.alloc(0x58);
        for i in 0..4 {
            e.mem.set_u32(this + PREV_KEY_ROT + 4 * i, 0x800 + i);
        }
        let clone = e.call(0x00a4_3e90, &args![this, 0x7000u32]).u32();
        assert_eq!(e.mem.u32(clone), ROT_ACCUM_VTABLE);
        assert_eq!(
            rot_calls_to(&e, TRANSFORM_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x7000]]
        );
        for i in 0..4 {
            assert_eq!(e.mem.u32(clone + PREV_KEY_ROT + 4 * i), 0x800 + i);
        }
        e.map(0, 0x60);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_3e90, &args![this, 0x7000u32]).u32(), 0);
        assert_eq!(e.mem.u32(PREV_KEY_ROT), 0x800);
        assert!(rot_calls_to(&e, TRANSFORM_INTERPOLATOR_CONSTRUCTOR).is_empty());
    }
}
