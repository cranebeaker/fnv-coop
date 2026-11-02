//! `NiAnimation/nitransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nianimation::nitransformdata::NiTransformData;
use crate::units::nimain::nimainsdm::{interlocked_decrement, interlocked_increment};
use crate::units::nimain::nistream::NiStream;

/// `NiTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3e50;

// Translated from 00a3fad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TRANSFORM_INTERPOLATOR_RTTI)
}

layout! {
    /// `NiTransformInterpolator` (Xbox PDB), 0x48 bytes on PC as on the Xbox: an
    /// `NiKeyBasedInterpolator` (vtable, count, last time), the transform value (an
    /// `NiQuatTransform`: translation, rotation `w x y z`, scale), the data and the last key
    /// indices. Floats are declared as their bit patterns.
    pub struct NiTransformInterpolator: 0x48 {
        /// `NiInterpolator::m_fLastTime` (Xbox PDB).
        0x08 m_fLastTime: u32,
        /// `m_kTransformValue.m_kTranslate` (Xbox PDB), three floats.
        0x0C m_kTranslate: u32,
        /// `m_kTransformValue.m_kRotate` (Xbox PDB), four floats (`m_fW` first).
        0x18 m_kRotate: u32,
        /// `m_kTransformValue.m_kRotate.m_fX` (Xbox PDB).
        0x1C m_fRotateX: u32,
        /// `m_kTransformValue.m_fScale` (Xbox PDB).
        0x28 m_fScale: u32,
        /// `m_spData` (Xbox PDB): `NiPointer<NiTransformData>`.
        0x2C m_spData: u32,
        /// `m_usLastTransIdx` (Xbox PDB).
        0x30 m_usLastTransIdx: u16,
        /// `m_usLastRotIdx` (Xbox PDB).
        0x32 m_usLastRotIdx: u16,
        /// `m_usLastScaleIdx` (Xbox PDB).
        0x34 m_usLastScaleIdx: u16,
        /// `m_origTranslate` (Xbox PDB), three floats.
        0x38 m_origTranslate: u32,
        /// `bPose` (Xbox PDB).
        0x44 bPose: u8,
    }
}

/// `NiTransformInterpolator`'s vtable.
const NI_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_7314;
/// The double `-FLT_MAX` an unset translation, rotation or scale holds (compared as such).
const UNSET_DOUBLE: u32 = 0x0102_41b0;
/// The floats a time range starts from: `01097308` (begin) and `-FLT_MAX` (`01015f5c`, end).
const RANGE_BEGIN_FLOAT: u32 = 0x0109_7308;
const RANGE_END_FLOAT: u32 = 0x0101_5f5c;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `00a3fa30` (`thiscall`, the data): the constructor from transform data.
const CONSTRUCT_FROM_DATA: u32 = 0x00a3_fa30;
/// `NiKeyBasedInterpolator`'s constructor and destructor body (`thiscall`, no stack words),
/// its `LoadBinary` (`00aa0ac0`, the stream) and its member copy (`00aa0ab0`, (clone, cloning
/// process)).
const KEY_BASED_CONSTRUCT: u32 = 0x00a3_ca10;
const KEY_BASED_DESTRUCT: u32 = 0x00a3_ca30;
const KEY_BASED_LOAD_BINARY: u32 = 0x00aa_0ac0;
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiQuatTransform::LoadBinary` (`thiscall` on the transform, the stream).
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
/// `NiStream::ResolveLinkID` (`thiscall`, no stack words).
const STREAM_RESOLVE_LINK_ID: u32 = 0x00a6_4620;
/// The three key getters (`thiscall`, (&count, &type, &size); answer the keys):
/// translation `00a3f990`, rotation `0058cb00`, scale `00a3f9e0` (the engine map's names on
/// them, `IsTransformInvalid`, `SetTranslate`, `SetRotate`, do not fit their use here).
const GET_TRANSLATION_KEYS: u32 = 0x00a3_f990;
const GET_ROTATION_KEYS: u32 = 0x0058_cb00;
const GET_SCALE_KEYS: u32 = 0x00a3_f9e0;
/// The key interpolations (`cdecl`): `NiPosKey::GenInterp` and `NiRotKey::GenInterp`
/// ((result, time, keys, type, count, &last index, size), answer the result) and
/// `NiFloatKey::GenInterp` ((time, keys, type, count, &last index, size), answers in `ST0`).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
const ROT_KEY_GEN_INTERP: u32 = 0x00a2_8740;
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;

/// Releases a counted object (count at `+4`): its slot `+4` runs when the count reaches 0.
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The compiler's `NiPointer` assignment of `new` to the word at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        interlocked_increment(e, new.wrapping_add(4));
    }
}

/// Copies the eight words of the transform value (+0xc) to `destination` (`REP MOVSD`).
fn copy_transform(e: &mut Engine, this: Ptr<NiTransformInterpolator>, destination: u32) {
    for word in 0..8 {
        let value = e.mem.u32(this.addr() + 0xc + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

/// Whether the scale, the rotation's `m_fX` and the translation's first float are all the
/// unset double `-FLT_MAX` (`010241b0`), compared in that order.
fn transform_unset(e: &Engine, this: Ptr<NiTransformInterpolator>) -> bool {
    let unset = x87_load64(e.mem.u64(UNSET_DOUBLE));
    x87_load32(e.get(this, NiTransformInterpolator::m_fScale)) == unset
        && x87_load32(e.get(this, NiTransformInterpolator::m_fRotateX)) == unset
        && x87_load32(e.get(this, NiTransformInterpolator::m_kTranslate)) == unset
}

// Translated from 00a3f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::LoadBinary` (Xbox PDB): `NiKeyBasedInterpolator::LoadBinary`,
/// the transform value (`NiQuatTransform::LoadBinary`), its translation kept as the original
/// translation, the data from the stream's link id (`NiStream::ResolveLinkID`, counted).
/// `bPose` is set when there is no data and the scale, the rotation's `m_fX` or the
/// translation's first float is not `-FLT_MAX` (the double at `010241b0`); cleared otherwise.
pub fn ni_transform_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    stream: Ptr<NiStream>,
) {
    e.call(KEY_BASED_LOAD_BINARY, &args![this, stream]);
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![this.addr() + 0xc, stream],
    );
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0xc + 4 * word);
        e.mem.set_u32(this.addr() + 0x38 + 4 * word, value);
    }
    let data = e.call(STREAM_RESOLVE_LINK_ID, &args![stream]).u32();
    assign_pointer(e, this.addr() + NiTransformInterpolator::m_spData.off, data);
    let pose = e.get(this, NiTransformInterpolator::m_spData) == 0 && !transform_unset(e, this);
    e.set(this, NiTransformInterpolator::bPose, u8::from(pose));
}

// Translated from 00a3fae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::NiTransformInterpolator` (Xbox PDB; `thiscall`, the
/// `NiQuatTransform` by value, `RET 0x20`): the `NiKeyBasedInterpolator` constructor, the
/// eight words of the transform as the transform value, the vtable `01097314`, no data, no
/// pose and the last key indices 0. Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn ni_transform_interpolator_construct(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    translate_x: u32,
    translate_y: u32,
    translate_z: u32,
    rotate_w: u32,
    rotate_x: u32,
    rotate_y: u32,
    rotate_z: u32,
    scale: u32,
) -> Ptr<NiTransformInterpolator> {
    e.call(KEY_BASED_CONSTRUCT, &args![this]);
    let words = [
        translate_x,
        translate_y,
        translate_z,
        rotate_w,
        rotate_x,
        rotate_y,
        rotate_z,
        scale,
    ];
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(this.addr() + 0xc + 4 * i as u32, *word);
    }
    e.mem.set_u32(this.addr(), NI_TRANSFORM_INTERPOLATOR_VTABLE);
    e.set(this, NiTransformInterpolator::m_spData, 0);
    e.set(this, NiTransformInterpolator::bPose, 0);
    e.set(this, NiTransformInterpolator::m_usLastTransIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastRotIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 0);
    this
}

/// Widens `[*begin, *end]` by the first and last keys' times (the first float of a key) of
/// `count` keys of `size` bytes at `keys`; each time goes through a `float` local first.
fn widen_range(e: &mut Engine, begin: u32, end: u32, keys: u32, count: u32, size: u32) {
    let first = x87_load32(x87_store32(x87_load32(e.mem.u32(keys))));
    if x87_load32(e.mem.u32(begin)) > first {
        e.mem.set_u32(begin, x87_store32(first));
    }
    let at = count.wrapping_sub(1).wrapping_mul(size).wrapping_add(keys);
    let last = x87_load32(x87_store32(x87_load32(e.mem.u32(at))));
    if x87_load32(e.mem.u32(end)) < last {
        e.mem.set_u32(end, x87_store32(last));
    }
}

// Translated from 00a3fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetActiveTimeRange` (Xbox PDB): the range starts at the float
/// `01097308` (begin) and `-FLT_MAX` (`01015f5c`, end) and is widened by the first and last
/// key of each key list of the data (`NiTransformData`): the translation keys (count +0xa,
/// size byte +0x1d, keys +0x24), the rotation keys (count +0x8, type +0x10, size byte +0x1c,
/// keys +0x20; for type 4, Euler keys, the three axis lists of the Euler key object: counts at
/// +0x14, size bytes at +0x2c, keys at +0x30), the scale keys (count +0xc, size byte +0x1e,
/// keys +0x28). Without data or keys the range is `[0, 0]`.
pub fn ni_transform_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let (begin, end) = (begin.addr(), end.addr());
    let start = x87_store32(x87_load32(e.mem.u32(RANGE_BEGIN_FLOAT)));
    e.mem.set_u32(begin, start);
    let stop = x87_store32(x87_load32(e.mem.u32(RANGE_END_FLOAT)));
    e.mem.set_u32(end, stop);
    let mut found = false;
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        let count = u32::from(e.mem.u16(data + 0xa));
        let size = u32::from(e.mem.u8(data + 0x1d));
        let keys = e.mem.u32(data + 0x24);
        if count != 0 {
            widen_range(e, begin, end, keys, count, size);
            found = true;
        }
        let count = u32::from(e.mem.u16(data + 0x8));
        let kind = e.mem.u32(data + 0x10);
        let size = u32::from(e.mem.u8(data + 0x1c));
        let keys = e.mem.u32(data + 0x20);
        if count != 0 {
            if kind == 4 {
                for axis in 0..3 {
                    let axis_count = e.mem.u32(keys + 0x14 + 4 * axis);
                    if axis_count != 0 {
                        let axis_keys = e.mem.u32(keys + 0x30 + 4 * axis);
                        let axis_size = u32::from(e.mem.u8(keys + 0x2c + axis));
                        widen_range(e, begin, end, axis_keys, axis_count, axis_size);
                        found = true;
                    }
                }
            } else {
                widen_range(e, begin, end, keys, count, size);
                found = true;
            }
        }
        let count = u32::from(e.mem.u16(data + 0xc));
        let size = u32::from(e.mem.u8(data + 0x1e));
        let keys = e.mem.u32(data + 0x28);
        if count != 0 {
            widen_range(e, begin, end, keys, count, size);
            return;
        }
    }
    if !found {
        e.mem.set_u32(begin, x87_store32(0.0));
        e.mem.set_u32(end, x87_store32(0.0));
    }
}

// Translated from 00a3fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::Update` (Xbox PDB; `RET 0xc`: time, the interpolation target,
/// which it does not read, and the `NiQuatTransform` to fill): a pose copies the transform
/// value out after taking `time` as its last time. At the last time the value is copied out
/// unchanged. Otherwise the translation, rotation and scale are interpolated from their keys
/// (`NiPosKey`, `NiRotKey`, `NiFloatKey::GenInterp`, each from the last key index, which it
/// updates; skipped for a list without keys) and the value copied out. Answers 0 when the
/// scale, the rotation's `m_fX` and the translation's first float are all `-FLT_MAX` (an
/// interpolation then does not take the time), 1 otherwise.
pub fn ni_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    time: f32,
    _unused_2: u32,
    value: Ptr,
) -> bool {
    let time_bits = time.to_bits();
    if e.get(this, NiTransformInterpolator::bPose) != 0 {
        let last = x87_store32(x87_load32(time_bits));
        e.set(this, NiTransformInterpolator::m_fLastTime, last);
        copy_transform(e, this, value.addr());
        return true;
    }
    let last = x87_load32(e.get(this, NiTransformInterpolator::m_fLastTime));
    if x87_load32(time_bits) == last {
        copy_transform(e, this, value.addr());
        return !transform_unset(e, this);
    }
    let time_word = x87_store32(x87_load32(time_bits));
    // The locals: last index (+0), count (+4), size (+8), type (+0xc), result (+0x10).
    e.with_stack(0x20, |e, locals| {
        let l = locals.addr();
        let (index, count, size, kind, result) = (l, l + 4, l + 8, l + 0xc, l + 0x10);
        let keys = e
            .call(GET_TRANSLATION_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastTransIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let out = e
                .call(
                    POS_KEY_GEN_INTERP,
                    &args![result, time_word, keys, k, n, index, s],
                )
                .u32();
            for word in 0..3 {
                let v = e.mem.u32(out + 4 * word);
                e.mem.set_u32(this.addr() + 0xc + 4 * word, v);
            }
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastTransIdx, updated);
        }
        let keys = e
            .call(GET_ROTATION_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastRotIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let out = e
                .call(
                    ROT_KEY_GEN_INTERP,
                    &args![result, time_word, keys, k, n, index, s],
                )
                .u32();
            for word in 0..4 {
                let v = e.mem.u32(out + 4 * word);
                e.mem.set_u32(this.addr() + 0x18 + 4 * word, v);
            }
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastRotIdx, updated);
        }
        let keys = e
            .call(GET_SCALE_KEYS, &args![this, count, kind, size])
            .u32();
        let n = e.mem.u32(count);
        if n != 0 {
            let last_index = u32::from(e.get(this, NiTransformInterpolator::m_usLastScaleIdx));
            e.mem.set_u32(index, last_index);
            let (k, s) = (e.mem.u32(kind), e.mem.u32(size));
            let scale = e
                .call(
                    FLOAT_KEY_GEN_INTERP,
                    &args![time_word, keys, k, n, index, s],
                )
                .f64();
            e.set(this, NiTransformInterpolator::m_fScale, x87_store32(scale));
            let updated = e.mem.u16(index);
            e.set(this, NiTransformInterpolator::m_usLastScaleIdx, updated);
        }
    });
    copy_transform(e, this, value.addr());
    if transform_unset(e, this) {
        return false;
    }
    let last = x87_store32(x87_load32(time_bits));
    e.set(this, NiTransformInterpolator::m_fLastTime, last);
    true
}

// Translated from 00a40360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitransforminterpolator.cpp`; by its body the member
/// copy): `00aa0ab0` (the key-based interpolator's copy), then the transform value, the
/// original translation, the data (counted) and the last key indices into `clone`.
pub fn fn_00a40360(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    clone: Ptr<NiTransformInterpolator>,
    cloning_process: Ptr,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, clone, cloning_process]);
    copy_transform(e, this, clone.addr() + 0xc);
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x38 + 4 * word);
        e.mem.set_u32(clone.addr() + 0x38 + 4 * word, value);
    }
    let data = e.get(this, NiTransformInterpolator::m_spData);
    assign_pointer(
        e,
        clone.addr() + NiTransformInterpolator::m_spData.off,
        data,
    );
    for field in [
        NiTransformInterpolator::m_usLastTransIdx,
        NiTransformInterpolator::m_usLastRotIdx,
        NiTransformInterpolator::m_usLastScaleIdx,
    ] {
        let value = e.get(this, field);
        e.set(clone, field, value);
    }
}

// Translated from 00a403f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiTransformInterpolator`): allocates 0x48 bytes (`operator new`) and, when
/// that succeeds, runs the constructor from data (`00a3fa30`) with no data on them, answering
/// its result; null otherwise.
pub fn fn_00a403f0(e: &mut Engine) -> Ptr<NiTransformInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(CONSTRUCT_FROM_DATA, &args![block, 0u32]).ptr()
}

// Translated from 00a40410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): releases the data
/// (when there is one; the word is left as it is), runs the `NiKeyBasedInterpolator`
/// destructor body (`00a3ca30`) and, when bit 0 of `flags` is set, the sized delete of 0x48
/// bytes. Answers `this`.
pub fn ni_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    flags: u32,
) -> Ptr<NiTransformInterpolator> {
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        release(e, data);
    }
    e.call(KEY_BASED_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

// Translated from 00a40460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::CreateClone` (Xbox PDB): a new interpolator (0x48 bytes,
/// `operator new`, the constructor from data `00a3fa30` with no data; null when the
/// allocation fails) given this one's members (`00a40360`, which runs on null too). Answers
/// the clone.
pub fn ni_transform_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    cloning_process: Ptr,
) -> Ptr<NiTransformInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let clone = if block != 0 {
        e.call(CONSTRUCT_FROM_DATA, &args![block, 0u32]).u32()
    } else {
        0
    };
    fn_00a40360(e, this, Ptr::new(clone), cloning_process);
    Ptr::new(clone)
}

/// `NiTransformData::GuaranteeKeysAtStartAndEnd` (`00a4df40`, `thiscall`, (begin, end)) and
/// `NiTransformData::GetSequenceData` (`00a4e410`, `thiscall`, (begin, end, &data)).
const DATA_GUARANTEE_KEYS: u32 = 0x00a4_df40;
const DATA_GET_SEQUENCE_DATA: u32 = 0x00a4_e410;
/// The key replacements of `NiTransformData` (`thiscall`, (keys, count, type); null keys
/// leave the list empty): rotation `00a4e2e0`, position `00a4e350`, scale `00a4e3b0`.
const DATA_SET_ROTATION_KEYS: u32 = 0x00a4_e2e0;
const DATA_SET_POSITION_KEYS: u32 = 0x00a4_e350;
const DATA_SET_SCALE_KEYS: u32 = 0x00a4_e3b0;
/// `NiObject::RegisterStreamables` (`00aa0ad0`), `NiObject::SaveBinary` (`00a95ca0`, the
/// engine map's name for it is folded code) and `NiObject::IsEqual` (`00c85800`, folded name
/// too), each `thiscall` with one word.
const OBJECT_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const OBJECT_SAVE_BINARY: u32 = 0x00a9_5ca0;
const OBJECT_IS_EQUAL: u32 = 0x00c8_5800;
/// `NiQuatTransform::SaveBinary` (`00a38b60`), `operator==` (`00a391c0`) and
/// `GetViewerStrings` (`00a38cb0`), `thiscall` on the transform with one word.
const QUAT_TRANSFORM_SAVE_BINARY: u32 = 0x00a3_8b60;
const QUAT_TRANSFORM_EQUAL: u32 = 0x00a3_91c0;
const QUAT_TRANSFORM_GET_VIEWER_STRINGS: u32 = 0x00a3_8cb0;
/// `NiInterpolator::GetSequenceInterpolator` (`00a57050`, `thiscall`, (begin, end)) and
/// `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`, `thiscall`, the strings).
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
const KEY_BASED_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the text) and the viewer-string array's grow
/// (`0096ad30`, `thiscall`, the new size) and set (`0096ae90`, `thiscall`, (index, &text)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `00439090` (`thiscall` on a point, the other point): whether the points differ.
const POINTS_DIFFER: u32 = 0x0043_9090;
/// The words a collapse starts its transform from: the translation (`011a8400`), the
/// rotation (`011f3704`, `w x y z`) and the scale float (`01096b8c`, copied through the x87
/// stack).
const DEFAULT_TRANSLATE: u32 = 0x011a_8400;
const DEFAULT_ROTATE: u32 = 0x011f_3704;
const DEFAULT_SCALE: u32 = 0x0109_6b8c;
/// The interpolator's virtual slot the posed test asks first (`thiscall`, the channel word;
/// a non-zero answer means the channel is not posed).
const SLOT_CHANNEL_KEYED: u32 = 0xe0;

/// The data word of `this` as transform data.
fn data_of(e: &Engine, this: Ptr<NiTransformInterpolator>) -> Ptr<NiTransformData> {
    Ptr::new(e.get(this, NiTransformInterpolator::m_spData))
}

// Translated from 00a3f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetKeyContent` (Xbox PDB; `thiscall`, the channel's low 16 bits;
/// `this` is not read): 1 for channel 0, 2 for channel 1, 0 for channel 2 and 6 for any
/// other (the compiler's `SETZ`/`DEC`/`AND 6`).
pub fn ni_transform_interpolator_get_key_content(
    _e: &mut Engine,
    _unused_0: u32,
    channel: u16,
) -> u32 {
    match channel {
        0 => 1,
        1 => 2,
        _ => u32::from(channel == 2).wrapping_sub(1) & 6,
    }
}

// Translated from 00a3f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetKeyCount` (Xbox PDB; `thiscall`, the channel's low 16 bits):
/// the data's position (channel 0), rotation (1) or scale (2) key count; 0 without data or
/// for another channel.
pub fn ni_transform_interpolator_get_key_count(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    channel: u16,
) -> u32 {
    let field = match channel {
        0 => NiTransformData::m_usNumPosKeys,
        1 => NiTransformData::m_usNumRotKeys,
        2 => NiTransformData::m_usNumScaleKeys,
        _ => return 0,
    };
    let data = data_of(e, this);
    if data.addr() == 0 {
        return 0;
    }
    u32::from(e.get(data, field))
}

// Translated from 00a3f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetKeyType` (Xbox PDB; `thiscall`, the channel's low 16 bits):
/// the data's position (channel 0), rotation (1) or scale (2) key type; 0 without data or for
/// another channel.
pub fn ni_transform_interpolator_get_key_type(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    channel: u16,
) -> u32 {
    let field = match channel {
        0 => NiTransformData::m_ePosType,
        1 => NiTransformData::m_eRotType,
        2 => NiTransformData::m_eScaleType,
        _ => return 0,
    };
    let data = data_of(e, this);
    if data.addr() == 0 {
        return 0;
    }
    e.get(data, field)
}

// Translated from 00a3f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetKeyArray` (Xbox PDB; `thiscall`, the channel's low 16 bits):
/// the data's position (channel 0), rotation (1) or scale (2) keys; null without data or for
/// another channel.
pub fn ni_transform_interpolator_get_key_array(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    channel: u16,
) -> u32 {
    let field = match channel {
        0 => NiTransformData::m_pkPosKeys,
        1 => NiTransformData::m_pkRotKeys,
        2 => NiTransformData::m_pkScaleKeys,
        _ => return 0,
    };
    let data = data_of(e, this);
    if data.addr() == 0 {
        return 0;
    }
    e.get(data, field)
}

// Translated from 00a3f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetKeyStride` (Xbox PDB; `thiscall`, the channel's low 16 bits;
/// a byte in `AL`): the data's position (channel 0), rotation (1) or scale (2) key size; 0
/// without data or for another channel.
pub fn ni_transform_interpolator_get_key_stride(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    channel: u16,
) -> u8 {
    let field = match channel {
        0 => NiTransformData::m_ucPosSize,
        1 => NiTransformData::m_ucRotSize,
        2 => NiTransformData::m_ucScaleSize,
        _ => return 0,
    };
    let data = data_of(e, this);
    if data.addr() == 0 {
        return 0;
    }
    e.get(data, field)
}

// Translated from 00a3f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GuaranteeTimeRange` (Xbox PDB; `thiscall`, (begin, end)): with
/// data, `NiTransformData::GuaranteeKeysAtStartAndEnd` (`00a4df40`) on it with the two times
/// (copied through the x87 stack).
pub fn ni_transform_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    begin: f32,
    end: f32,
) {
    let data = data_of(e, this);
    if data.addr() == 0 {
        return;
    }
    e.call(
        DATA_GUARANTEE_KEYS,
        &args![
            data,
            x87_quiet_bits(begin.to_bits()),
            x87_quiet_bits(end.to_bits())
        ],
    );
}

// Translated from 00a3f720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetChannelPosed` (Xbox PDB; `thiscall`, the channel word): for
/// channel 0, 1 or 2 (the low 16 bits) with data, a non-zero answer of the interpolator's
/// slot +0xe0 (given the whole word) means not posed; otherwise the channel is posed when its
/// value (translation x +0xc, rotation x +0x1c, scale +0x28) is not the unset double
/// `-FLT_MAX` (`010241b0`; a NaN counts as posed). Any other channel is not posed.
pub fn ni_transform_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    channel: u32,
) -> bool {
    let value = match channel & 0xffff {
        0 => NiTransformInterpolator::m_kTranslate,
        1 => NiTransformInterpolator::m_fRotateX,
        2 => NiTransformInterpolator::m_fScale,
        _ => return false,
    };
    if e.get(this, NiTransformInterpolator::m_spData) != 0
        && e.vcall(this.addr(), SLOT_CHANNEL_KEYED, &args![channel])
            .u32()
            != 0
    {
        return false;
    }
    let unset = x87_load64(e.mem.u64(UNSET_DOUBLE));
    x87_load32(e.get(this, value)) != unset
}

// Translated from 00a3f870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false
/// when `NiObject::RegisterStreamables` (`00aa0ad0`) is; otherwise the data's own (its slot
/// +0x54) when there is data, and true.
pub fn ni_transform_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    stream: u32,
) -> bool {
    if !e
        .call(OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        e.vcall(data, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a3f8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base
/// `SaveBinary` (`00a95ca0`), the transform value (`NiQuatTransform::SaveBinary`) and the
/// data's link (the stream's slot +0x30 with the data).
pub fn ni_transform_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    stream: u32,
) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    e.call(
        QUAT_TRANSFORM_SAVE_BINARY,
        &args![this.addr() + 0xc, stream],
    );
    let data = e.get(this, NiTransformInterpolator::m_spData);
    e.vcall(stream, 0x30, &args![data]);
}

// Translated from 00a3f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::IsEqual` (Xbox PDB; `thiscall`, the other object): equal when
/// `NiObject::IsEqual` (`00c85800`) and the transform values (`NiQuatTransform::operator==`)
/// are, and both have no data or both have data that is equal (the data's slot +0x5c).
pub fn ni_transform_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    other: Ptr<NiTransformInterpolator>,
) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if !e
        .call(
            QUAT_TRANSFORM_EQUAL,
            &args![this.addr() + 0xc, other.addr() + 0xc],
        )
        .bool()
    {
        return false;
    }
    let mine = e.get(this, NiTransformInterpolator::m_spData);
    let theirs = e.get(other, NiTransformInterpolator::m_spData);
    if (mine == 0) != (theirs == 0) {
        return false;
    }
    if mine == 0 {
        return true;
    }
    e.vcall(mine, 0x5c, &args![theirs]).bool()
}

// Translated from 00a3f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the data; by its body the data setter): the data
/// pointer takes `data` (the compiler's `NiPointer` assignment, counted) and the last
/// rotation, scale and translation key indices are reset to 0.
pub fn fn_00a3f930(e: &mut Engine, this: Ptr<NiTransformInterpolator>, data: u32) {
    assign_pointer(e, this.addr() + 0x2c, data);
    e.set(this, NiTransformInterpolator::m_usLastRotIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 0);
    e.set(this, NiTransformInterpolator::m_usLastTransIdx, 0);
}

// Translated from 00a3fd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetSequenceInterpolator` (Xbox PDB; `thiscall`, (begin, end)):
/// the base's sequence interpolator (`NiInterpolator::GetSequenceInterpolator`, `00a57050`);
/// when there is one and this interpolator has data, it gets the data's keys between the
/// times (`NiTransformData::GetSequenceData`, `00a4e410`, into a stack `NiPointer` that
/// starts null) through `00a3f930`, its last key indices are reset (again), and the stack
/// pointer is released. Answers the interpolator.
pub fn ni_transform_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    begin: f32,
    end: f32,
) -> u32 {
    let begin = x87_quiet_bits(begin.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    let interpolator = e
        .call(
            INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            &args![this, begin, end],
        )
        .u32();
    if interpolator == 0 {
        return 0;
    }
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data == 0 {
        return interpolator;
    }
    e.with_stack(4, |e, slot| {
        e.call(DATA_GET_SEQUENCE_DATA, &args![data, begin, end, slot]);
        let sequence_data = e.mem.u32(slot.addr());
        let target = Ptr::<NiTransformInterpolator>::new(interpolator);
        fn_00a3f930(e, target, sequence_data);
        e.set(target, NiTransformInterpolator::m_usLastRotIdx, 0);
        e.set(target, NiTransformInterpolator::m_usLastTransIdx, 0);
        e.set(target, NiTransformInterpolator::m_usLastScaleIdx, 0);
        let held = e.mem.u32(slot.addr());
        if held != 0 {
            release(e, held);
        }
    });
    interpolator
}

/// Whether the four floats at `a` and `b` are equal (`FUCOMPP` each, a NaN unequal), in
/// order, stopping at the first that is not.
fn quaternions_equal(e: &Engine, a: u32, b: u32) -> bool {
    (0..4u32).all(|i| x87_load32(e.mem.u32(a + 4 * i)) == x87_load32(e.mem.u32(b + 4 * i)))
}

// Translated from 00a3ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::Collapse` (Xbox PDB; `thiscall`, no stack words): with data,
/// each key list that holds one value is folded into a transform built on the stack (from
/// the translation `011a8400`, rotation `011f3704` and scale `01096b8c`), which then replaces
/// the transform value. Per part, in the order translation, rotation, scale: no keys clears
/// the data's list (`00a4e350`, `00a4e2e0`, `00a4e3b0` with null keys) and keeps the
/// interpolator's own value when it is set (not the double `-FLT_MAX` at `010241b0`); one key
/// (for rotation: of a type other than 4), or keys of type 1 or 5 that all hold the first
/// key's value (points compared by `00439090`, quaternions and floats `==`), also clears the
/// list but takes that value; anything else keeps the own value when set. When all three
/// lists are then empty the data is released and the pointer cleared.
pub fn ni_transform_interpolator_collapse(e: &mut Engine, this: Ptr<NiTransformInterpolator>) {
    if e.get(this, NiTransformInterpolator::m_spData) == 0 {
        return;
    }
    let at = this.addr();
    let unset = x87_load64(e.mem.u64(UNSET_DOUBLE));
    let own_set = |e: &Engine, offset: u32| x87_load32(e.mem.u32(at + offset)) != unset;
    let mut transform = [0u32; 8];
    for i in 0..3u32 {
        transform[i as usize] = e.mem.u32(DEFAULT_TRANSLATE + 4 * i);
    }
    for i in 0..4u32 {
        transform[3 + i as usize] = e.mem.u32(DEFAULT_ROTATE + 4 * i);
    }
    transform[7] = x87_quiet_bits(e.mem.u32(DEFAULT_SCALE));
    e.with_stack(0x10, |e, first| {
        let first = first.addr();
        // Translation.
        let data = data_of(e, this);
        let count = u32::from(e.get(data, NiTransformData::m_usNumPosKeys));
        let keys = e.get(data, NiTransformData::m_pkPosKeys);
        let kind = e.get(data, NiTransformData::m_ePosType);
        let mut keep_own = true;
        if count == 0 {
            e.call(DATA_SET_POSITION_KEYS, &args![data, 0u32, 0u32, 0u32]);
        } else {
            for i in 0..3u32 {
                let word = e.mem.u32(keys.wrapping_add(4 + 4 * i));
                e.mem.set_u32(first + 4 * i, word);
            }
            let fold = if count == 1 {
                true
            } else if kind == 1 || kind == 5 {
                let mut same = true;
                let mut i = 1u32;
                while i < count {
                    let size = u32::from(e.get(data, NiTransformData::m_ucPosSize));
                    let value = size.wrapping_mul(i).wrapping_add(keys).wrapping_add(4);
                    if e.call(POINTS_DIFFER, &args![value, first]).bool() {
                        same = false;
                    }
                    i += 1;
                    if !same {
                        break;
                    }
                }
                same
            } else {
                false
            };
            if fold {
                e.call(DATA_SET_POSITION_KEYS, &args![data, 0u32, 0u32, 0u32]);
                for i in 0..3u32 {
                    transform[i as usize] = e.mem.u32(first + 4 * i);
                }
                keep_own = false;
            }
        }
        if keep_own && own_set(e, 0xc) {
            for i in 0..3u32 {
                transform[i as usize] = e.mem.u32(at + 0xc + 4 * i);
            }
        }
        // Rotation.
        let data = data_of(e, this);
        let count = u32::from(e.get(data, NiTransformData::m_usNumRotKeys));
        let kind = e.get(data, NiTransformData::m_eRotType);
        let keys = e.get(data, NiTransformData::m_pkRotKeys);
        let mut keep_own = true;
        if count == 0 {
            e.call(DATA_SET_ROTATION_KEYS, &args![data, 0u32, 0u32, 0u32]);
        } else {
            for i in 0..4u32 {
                let word = e.mem.u32(keys.wrapping_add(4 + 4 * i));
                e.mem.set_u32(first + 4 * i, word);
            }
            let fold = if count == 1 && kind != 4 {
                true
            } else if kind == 1 || kind == 5 {
                let mut same = true;
                let mut i = 1u32;
                while i < u32::from(e.get(data, NiTransformData::m_usNumRotKeys)) {
                    let size = u32::from(e.get(data, NiTransformData::m_ucRotSize));
                    let value = size.wrapping_mul(i).wrapping_add(keys).wrapping_add(4);
                    if !quaternions_equal(e, first, value) {
                        same = false;
                    }
                    i += 1;
                    if !same {
                        break;
                    }
                }
                same
            } else {
                false
            };
            if fold {
                e.call(DATA_SET_ROTATION_KEYS, &args![data, 0u32, 0u32, 0u32]);
                for i in 0..4u32 {
                    transform[3 + i as usize] = e.mem.u32(first + 4 * i);
                }
                keep_own = false;
            }
        }
        if keep_own && own_set(e, 0x1c) {
            for i in 0..4u32 {
                transform[3 + i as usize] = e.mem.u32(at + 0x18 + 4 * i);
            }
        }
        // Scale.
        let data = data_of(e, this);
        let count = u32::from(e.get(data, NiTransformData::m_usNumScaleKeys));
        let kind = e.get(data, NiTransformData::m_eScaleType);
        let keys = e.get(data, NiTransformData::m_pkScaleKeys);
        let mut keep_own = true;
        if count == 0 {
            e.call(DATA_SET_SCALE_KEYS, &args![data, 0u32, 0u32, 0u32]);
        } else {
            let value = x87_quiet_bits(e.mem.u32(keys.wrapping_add(4)));
            let fold = if count == 1 {
                true
            } else if kind == 1 || kind == 5 {
                let mut same = true;
                let mut i = 1u32;
                while i < count {
                    let size = u32::from(e.get(data, NiTransformData::m_ucScaleSize));
                    let at_key = size.wrapping_mul(i).wrapping_add(keys).wrapping_add(4);
                    if x87_load32(e.mem.u32(at_key)) != x87_load32(value) {
                        same = false;
                    }
                    i += 1;
                    if !same {
                        break;
                    }
                }
                same
            } else {
                false
            };
            if fold {
                e.call(DATA_SET_SCALE_KEYS, &args![data, 0u32, 0u32, 0u32]);
                transform[7] = value;
                keep_own = false;
            }
        }
        if keep_own && own_set(e, 0x28) {
            transform[7] = x87_quiet_bits(e.get(this, NiTransformInterpolator::m_fScale));
        }
    });
    let data = data_of(e, this);
    if e.get(data, NiTransformData::m_usNumPosKeys) == 0
        && e.get(data, NiTransformData::m_usNumRotKeys) == 0
        && e.get(data, NiTransformData::m_usNumScaleKeys) == 0
        && data.addr() != 0
    {
        release(e, data.addr());
        e.set(this, NiTransformInterpolator::m_spData, 0);
    }
    for (i, word) in transform.iter().enumerate() {
        e.mem.set_u32(at + 0xc + 4 * i as u32, *word);
    }
}

// Translated from 00a404b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransformInterpolator::GetViewerStrings` (Xbox PDB; `thiscall`, the strings): the
/// parent's strings (`NiKeyBasedInterpolator::GetViewerStrings`, `00a3cbf0`), the class name
/// (the first word of the `NiRTTI` record at `011f3e50`, through `NiGetViewerString`)
/// appended (growing the array to the step +0xE plus the size when the 16-bit size +0xA has
/// reached the capacity +0x8; the text goes in through the strings argument's stack word),
/// the transform value's strings (`NiQuatTransform::GetViewerStrings`) and, with data, the
/// data's (its slot +0x60).
pub fn ni_transform_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiTransformInterpolator>,
    strings: u32,
) {
    e.call(KEY_BASED_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(NI_TRANSFORM_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
    e.call(
        QUAT_TRANSFORM_GET_VIEWER_STRINGS,
        &args![this.addr() + 0xc, strings],
    );
    let data = e.get(this, NiTransformInterpolator::m_spData);
    if data != 0 {
        e.vcall(data, 0x60, &args![strings]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3fad0, ni_transform_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a3_f7b0,
            ni_transform_interpolator_load_binary(Ptr<NiTransformInterpolator>, Ptr<NiStream>)
        ),
        entry!(
            0x00a3_fae0,
            ni_transform_interpolator_construct(
                Ptr<NiTransformInterpolator>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<NiTransformInterpolator>
        ),
        entry!(
            0x00a3_fb20,
            ni_transform_interpolator_get_active_time_range(Ptr<NiTransformInterpolator>, Ptr, Ptr)
        ),
        entry!(
            0x00a3_fdb0,
            ni_transform_interpolator_update(Ptr<NiTransformInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_0360,
            fn_00a40360(Ptr<NiTransformInterpolator>, Ptr<NiTransformInterpolator>, Ptr)
        ),
        entry!(0x00a4_03f0, fn_00a403f0() -> Ptr<NiTransformInterpolator>),
        entry!(
            0x00a4_0410,
            ni_transform_interpolator_scalar_deleting_destructor(
                Ptr<NiTransformInterpolator>,
                u32,
            )
                -> Ptr<NiTransformInterpolator>
        ),
        entry!(
            0x00a4_0460,
            ni_transform_interpolator_create_clone(
                Ptr<NiTransformInterpolator>,
                Ptr,
            ) -> Ptr<NiTransformInterpolator>
        ),
        entry!(
            0x00a3_f570,
            ni_transform_interpolator_get_key_content(u32, u16) -> u32
        ),
        entry!(
            0x00a3_f5b0,
            ni_transform_interpolator_get_key_count(Ptr<NiTransformInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_f600,
            ni_transform_interpolator_get_key_type(Ptr<NiTransformInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_f650,
            ni_transform_interpolator_get_key_array(Ptr<NiTransformInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_f6a0,
            ni_transform_interpolator_get_key_stride(Ptr<NiTransformInterpolator>, u16) -> u8
        ),
        entry!(
            0x00a3_f6f0,
            ni_transform_interpolator_guarantee_time_range(Ptr<NiTransformInterpolator>, f32, f32)
        ),
        entry!(
            0x00a3_f720,
            ni_transform_interpolator_get_channel_posed(Ptr<NiTransformInterpolator>, u32) -> bool
        ),
        entry!(
            0x00a3_f870,
            ni_transform_interpolator_register_streamables(
                Ptr<NiTransformInterpolator>,
                u32,
            ) -> bool
        ),
        entry!(
            0x00a3_f8a0,
            ni_transform_interpolator_save_binary(Ptr<NiTransformInterpolator>, u32)
        ),
        entry!(
            0x00a3_f8d0,
            ni_transform_interpolator_is_equal(
                Ptr<NiTransformInterpolator>,
                Ptr<NiTransformInterpolator>,
            ) -> bool
        ),
        entry!(0x00a3_f930, fn_00a3f930(Ptr<NiTransformInterpolator>, u32)),
        entry!(
            0x00a3_fd10,
            ni_transform_interpolator_get_sequence_interpolator(
                Ptr<NiTransformInterpolator>,
                f32,
                f32,
            ) -> u32
        ),
        entry!(
            0x00a3_ffd0,
            ni_transform_interpolator_collapse(Ptr<NiTransformInterpolator>)
        ),
        entry!(
            0x00a4_04b0,
            ni_transform_interpolator_get_viewer_strings(Ptr<NiTransformInterpolator>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3fad0, &args![]).u32(),
            NI_TRANSFORM_INTERPOLATOR_RTTI
        );
    }
}

#[cfg(test)]
mod interpolator_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const UNSET: u32 = 0xff7f_ffff;

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(0x0102_4000, 0x1000);
        e.map(0x0101_5000, 0x1000);
        e.map(0x0109_7000, 0x1000);
        e.mem.set_u64(UNSET_DOUBLE, f64::from(f32::MIN).to_bits());
        e.mem.set_u32(RANGE_END_FLOAT, UNSET);
        e.mem.set_u32(RANGE_BEGIN_FLOAT, 0x7f7f_ffff);
        e.put_vtable(0x0300_0000, &[0, 0x0300_0004]);
        e.register(0x0300_0004, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn counted(e: &mut Engine, count: u32) -> u32 {
        let o = e.mem.alloc(0x30);
        e.mem.set_u32(o, 0x0300_0000);
        e.mem.set_u32(o + 4, count);
        o
    }

    fn set_f(e: &mut Engine, at: u32, v: f32) {
        e.mem.set_u32(at, v.to_bits());
    }

    fn unset_transform(e: &mut Engine, this: Ptr<NiTransformInterpolator>) {
        e.set(this, NiTransformInterpolator::m_kTranslate, UNSET);
        e.set(this, NiTransformInterpolator::m_fRotateX, UNSET);
        e.set(this, NiTransformInterpolator::m_fScale, UNSET);
    }

    #[test]
    fn load_binary_resolves_the_data_and_decides_the_pose() {
        let mut e = engine();
        e.register(KEY_BASED_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |e, a| {
            for word in 0..8 {
                e.mem.set_u32(a[0] + 4 * word, 0x100 + word);
            }
            Ret::default()
        });
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
        let this = e.new_object::<NiTransformInterpolator>();
        let stream = e.new_object::<NiStream>();
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0x100);
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0x102);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 1);
        assert_eq!(
            calls_to(&e, QUAT_TRANSFORM_LOAD_BINARY),
            vec![vec![this.addr() + 0xc, stream.addr()]]
        );
        // With data: no pose, the data counted.
        let data = counted(&mut e, 0);
        e.register_double(STREAM_RESOLVE_LINK_ID, move |_, _| data.into_ret());
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), data);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        // No data and everything unset: no pose; the old data released.
        e.register(STREAM_RESOLVE_LINK_ID, |_, _| 0u32.into_ret());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |e, a| {
            for word in [0u32, 4, 7] {
                e.mem.set_u32(a[0] + 4 * word, UNSET);
            }
            Ret::default()
        });
        e.call(0x00a3_f7b0, &args![this, stream]);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![data]]);
    }

    #[test]
    fn constructor_takes_the_transform_words() {
        let mut e = engine();
        e.register(KEY_BASED_CONSTRUCT, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        e.mem.write(this.addr(), &[0xee; 0x48]);
        let r = e
            .call(
                0x00a3_fae0,
                &args![this, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
            )
            .u32();
        assert_eq!(r, this.addr());
        for word in 0..8 {
            assert_eq!(e.mem.u32(this.addr() + 0xc + 4 * word), word + 1);
        }
        assert_eq!(e.mem.u32(this.addr()), NI_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), 0);
        assert_eq!(e.get(this, NiTransformInterpolator::bPose), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x30), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x34), 0xeeee_0000);
    }

    /// Keys of `size` bytes whose times are `times`.
    fn keys(e: &mut Engine, times: &[f32], size: u32) -> u32 {
        let k = e.mem.alloc(size * times.len() as u32);
        for (i, t) in times.iter().enumerate() {
            set_f(e, k + size * i as u32, *t);
        }
        k
    }

    #[test]
    fn active_time_range_spans_every_key_list() {
        let mut e = engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let begin = e.mem.alloc(4);
        let end = e.mem.alloc(4);
        // No data: [0, 0].
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.0, 0.0));
        let data = e.mem.alloc(0x30);
        e.set(this, NiTransformInterpolator::m_spData, data);
        let translation = keys(&mut e, &[1.0, 2.0, 3.0], 0x10);
        e.mem.set_u16(data + 0xa, 3);
        e.mem.set_u8(data + 0x1d, 0x10);
        e.mem.set_u32(data + 0x24, translation);
        let scale = keys(&mut e, &[0.5, 2.5], 8);
        e.mem.set_u16(data + 0xc, 2);
        e.mem.set_u8(data + 0x1e, 8);
        e.mem.set_u32(data + 0x28, scale);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.5, 3.0));
        // Plain rotation keys.
        let rotation = keys(&mut e, &[-1.0, 9.0], 0x14);
        e.mem.set_u16(data + 0x8, 2);
        e.mem.set_u8(data + 0x1c, 0x14);
        e.mem.set_u32(data + 0x20, rotation);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (-1.0, 9.0));
        // Euler keys: the three axis lists (the second one empty).
        let euler = e.mem.alloc(0x40);
        let x = keys(&mut e, &[-4.0, 4.0], 8);
        let z = keys(&mut e, &[0.0, 1.0, 12.0], 8);
        e.mem.set_u32(euler + 0x14, 2);
        e.mem.set_u32(euler + 0x1c, 3);
        e.mem.set_u8(euler + 0x2c, 8);
        e.mem.set_u8(euler + 0x2e, 8);
        e.mem.set_u32(euler + 0x30, x);
        e.mem.set_u32(euler + 0x38, z);
        e.mem.set_u32(data + 0x10, 4);
        e.mem.set_u32(data + 0x20, euler);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (-4.0, 12.0));
        // Only empty lists: [0, 0].
        e.mem.set_u16(data + 0x8, 0);
        e.mem.set_u16(data + 0xa, 0);
        e.mem.set_u16(data + 0xc, 0);
        e.call(0x00a3_fb20, &args![this, begin, end]);
        assert_eq!((e.mem.f32(begin), e.mem.f32(end)), (0.0, 0.0));
    }

    #[test]
    fn update_poses_reuses_and_interpolates() {
        let mut e = engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let out = e.mem.alloc(0x20);
        for word in 0..8 {
            e.mem
                .set_u32(this.addr() + 0xc + 4 * word, 0x3f80_0000 + word);
        }
        // A pose: the value copied out, the time taken.
        e.set(this, NiTransformInterpolator::bPose, 1);
        assert!(e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f80_0007);
        assert_eq!(e.mem.f32(this.addr() + 8), 2.0);
        // At the last time: copied out; answers 0 only with everything unset.
        e.set(this, NiTransformInterpolator::bPose, 0);
        assert!(e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        unset_transform(&mut e, this);
        assert!(!e.call(0x00a3_fdb0, &args![this, 2.0f32, 0u32, out]).bool());
        // A new time: the key lists that have keys are interpolated.
        e.register(GET_TRANSLATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 2);
            e.mem.set_u32(a[2], 1);
            e.mem.set_u32(a[3], 0x10);
            0x7000u32.into_ret()
        });
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            assert_eq!((a[2], a[3], a[4], a[6]), (0x7000, 1, 2, 0x10));
            assert_eq!(e.mem.u32(a[5]), 5);
            e.mem.set_u32(a[5], 6);
            for word in 0..3 {
                e.mem
                    .set_u32(a[0] + 4 * word, (10.0f32 + word as f32).to_bits());
            }
            a[0].into_ret()
        });
        e.register(GET_ROTATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        e.register(GET_SCALE_KEYS, |e, a| {
            e.mem.set_u32(a[1], 3);
            e.mem.set_u32(a[2], 2);
            e.mem.set_u32(a[3], 8);
            0x8000u32.into_ret()
        });
        e.register(FLOAT_KEY_GEN_INTERP, |e, a| {
            assert_eq!(f32::from_bits(a[0]), 3.0);
            assert_eq!((a[1], a[2], a[3], a[5]), (0x8000, 2, 3, 8));
            e.mem.set_u32(a[4], 1);
            Ret {
                st0: 1.5,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        e.set(this, NiTransformInterpolator::m_usLastTransIdx, 5);
        assert!(e.call(0x00a3_fdb0, &args![this, 3.0f32, 0u32, out]).bool());
        assert!(calls_to(&e, ROT_KEY_GEN_INTERP).is_empty());
        assert_eq!(e.mem.f32(this.addr() + 0xc), 10.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 12.0);
        assert_eq!(e.get(this, NiTransformInterpolator::m_usLastTransIdx), 6);
        assert_eq!(e.mem.f32(this.addr() + 0x28), 1.5);
        assert_eq!(e.get(this, NiTransformInterpolator::m_usLastScaleIdx), 1);
        assert_eq!(e.mem.f32(out), 10.0);
        assert_eq!(e.mem.f32(this.addr() + 8), 3.0);
        // A new time without keys and everything unset: answers 0, the time not taken.
        e.register(GET_TRANSLATION_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        e.register(GET_SCALE_KEYS, |e, a| {
            e.mem.set_u32(a[1], 0);
            0u32.into_ret()
        });
        unset_transform(&mut e, this);
        assert!(!e.call(0x00a3_fdb0, &args![this, 4.0f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this.addr() + 8), 3.0);
    }

    #[test]
    fn copy_members_copies_the_value_and_counts_the_data() {
        let mut e = engine();
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        let clone = e.new_object::<NiTransformInterpolator>();
        for word in 0..14 {
            e.mem.set_u32(this.addr() + 0xc + 4 * word, 0x500 + word);
        }
        let data = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        e.set(this, NiTransformInterpolator::m_spData, data);
        e.set(clone, NiTransformInterpolator::m_spData, old);
        e.set(this, NiTransformInterpolator::m_usLastTransIdx, 1);
        e.set(this, NiTransformInterpolator::m_usLastRotIdx, 2);
        e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 3);
        e.call(0x00a4_0360, &args![this, clone, 0x99u32]);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x99]]
        );
        for word in 0..8 {
            assert_eq!(e.mem.u32(clone.addr() + 0xc + 4 * word), 0x500 + word);
        }
        assert_eq!(e.mem.u32(clone.addr() + 0x38), 0x50b);
        assert_eq!(e.mem.u32(clone.addr() + 0x40), 0x50d);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_spData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![old]]);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastTransIdx), 1);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastRotIdx), 2);
        assert_eq!(e.get(clone, NiTransformInterpolator::m_usLastScaleIdx), 3);
    }

    #[test]
    fn create_object_00a403f0_builds_without_data() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_FROM_DATA, |_, a| a[0].into_ret());
        let object = e.call(0x00a4_03f0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(calls_to(&e, CONSTRUCT_FROM_DATA), vec![vec![object, 0]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x48]]);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a4_03f0, &args![]).u32(), 0);
        assert_eq!(calls_to(&e, CONSTRUCT_FROM_DATA).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_releases_the_data() {
        let mut e = engine();
        e.register(KEY_BASED_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        let data = counted(&mut e, 1);
        e.set(this, NiTransformInterpolator::m_spData, data);
        assert_eq!(e.call(0x00a4_0410, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, 0x0300_0004), vec![vec![data]]);
        assert_eq!(calls_to(&e, KEY_BASED_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x48]]);
        e.set(this, NiTransformInterpolator::m_spData, 0);
        e.call(0x00a4_0410, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_FROM_DATA, |_, a| a[0].into_ret());
        e.register(KEY_BASED_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiTransformInterpolator>();
        e.mem.set_u32(this.addr() + 0x10, 0x1234);
        let clone = e.call(0x00a4_0460, &args![this, 0x77u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone + 0x10), 0x1234);
        assert_eq!(
            calls_to(&e, KEY_BASED_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Counting stand-ins for the imported `InterlockedIncrement` and `InterlockedDecrement`.
    fn counting(e: &mut Engine) {
        e.register(0x00fd_f1c0, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x00fd_f1c4, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
    }

    fn with_unset(e: &mut Engine) {
        e.map(UNSET_DOUBLE, 8);
        e.mem.set_u64(UNSET_DOUBLE, f64::from(f32::MIN).to_bits());
    }

    /// An interpolator with data whose counts, types, sizes and keys are set per channel.
    fn interpolator_with_data(
        e: &mut Engine,
    ) -> (Ptr<NiTransformInterpolator>, Ptr<NiTransformData>) {
        let this = e.new_object::<NiTransformInterpolator>();
        let data = e.new_object::<NiTransformData>();
        e.set(this, NiTransformInterpolator::m_spData, data.addr());
        e.set(data, NiTransformData::m_usNumPosKeys, 3);
        e.set(data, NiTransformData::m_usNumRotKeys, 4);
        e.set(data, NiTransformData::m_usNumScaleKeys, 5);
        e.set(data, NiTransformData::m_ePosType, 0x11);
        e.set(data, NiTransformData::m_eRotType, 0x12);
        e.set(data, NiTransformData::m_eScaleType, 0x13);
        e.set(data, NiTransformData::m_ucPosSize, 0x21);
        e.set(data, NiTransformData::m_ucRotSize, 0x22);
        e.set(data, NiTransformData::m_ucScaleSize, 0x23);
        e.set(data, NiTransformData::m_pkPosKeys, 0x31);
        e.set(data, NiTransformData::m_pkRotKeys, 0x32);
        e.set(data, NiTransformData::m_pkScaleKeys, 0x33);
        (this, data)
    }

    #[test]
    fn get_key_content_maps_the_channels() {
        let mut e = Engine::new();
        let content: Vec<u32> = (0..4u32)
            .map(|c| e.call(0x00a3_f570, &args![0u32, c | 0x7_0000]).u32())
            .collect();
        assert_eq!(content, vec![1, 2, 0, 6]);
    }

    #[test]
    fn the_key_getters_answer_the_data_channel() {
        let mut e = Engine::new();
        let (this, _) = interpolator_with_data(&mut e);
        let get =
            |e: &mut Engine, addr: u32, channel: u32| e.call(addr, &args![this, channel]).u32();
        let counts: Vec<u32> = (0..4).map(|c| get(&mut e, 0x00a3_f5b0, c)).collect();
        assert_eq!(counts, vec![3, 4, 5, 0]);
        let kinds: Vec<u32> = (0..4).map(|c| get(&mut e, 0x00a3_f600, c)).collect();
        assert_eq!(kinds, vec![0x11, 0x12, 0x13, 0]);
        let arrays: Vec<u32> = (0..4).map(|c| get(&mut e, 0x00a3_f650, c)).collect();
        assert_eq!(arrays, vec![0x31, 0x32, 0x33, 0]);
        let strides: Vec<u8> = (0..4)
            .map(|c| e.call(0x00a3_f6a0, &args![this, c]).u8())
            .collect();
        assert_eq!(strides, vec![0x21, 0x22, 0x23, 0]);
        // Without data every getter answers 0.
        e.set(this, NiTransformInterpolator::m_spData, 0);
        for addr in [0x00a3_f5b0, 0x00a3_f600, 0x00a3_f650, 0x00a3_f6a0] {
            assert_eq!(get(&mut e, addr, 1), 0);
        }
    }

    #[test]
    fn guarantee_time_range_runs_on_the_data() {
        let mut e = Engine::new();
        e.register(DATA_GUARANTEE_KEYS, |_, _| Ret::default());
        let (this, data) = interpolator_with_data(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a3_f6f0, &args![this, 1.0f32, 2.0f32]);
        assert_eq!(
            calls_to(&e, DATA_GUARANTEE_KEYS),
            vec![vec![data.addr(), 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        e.set(this, NiTransformInterpolator::m_spData, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a3_f6f0, &args![this, 1.0f32, 2.0f32]);
        assert!(calls_to(&e, DATA_GUARANTEE_KEYS).is_empty());
    }

    #[test]
    fn get_channel_posed_asks_the_keys_then_the_value() {
        const KEYED: u32 = 0x0300_0300;
        let mut e = Engine::new();
        with_unset(&mut e);
        let (this, _) = interpolator_with_data(&mut e);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + SLOT_CHANNEL_KEYED, KEYED);
        e.mem.set_u32(this.addr(), vtable);
        e.register(KEYED, |_, a| u32::from(a[1] & 0xffff == 1).into_ret());
        e.set(
            this,
            NiTransformInterpolator::m_kTranslate,
            1.0f32.to_bits(),
        );
        e.set(this, NiTransformInterpolator::m_fRotateX, 2.0f32.to_bits());
        e.set(this, NiTransformInterpolator::m_fScale, f32::MIN.to_bits());
        e.call_log = Some(vec![]);
        let posed = |e: &mut Engine, c: u32| e.call(0x00a3_f720, &args![this, c]).bool();
        assert!(posed(&mut e, 0x5_0000));
        assert_eq!(calls_to(&e, KEYED), vec![vec![this.addr(), 0x5_0000]]);
        // Channel 1 has keys; channel 2 holds the unset value; 3 is no channel.
        assert!(!posed(&mut e, 1));
        assert!(!posed(&mut e, 2));
        assert!(!posed(&mut e, 3));
        // Without data the keys are not asked.
        e.set(this, NiTransformInterpolator::m_spData, 0);
        e.call_log = Some(vec![]);
        assert!(posed(&mut e, 1));
        assert!(calls_to(&e, KEYED).is_empty());
    }

    #[test]
    fn register_streamables_and_save_binary_reach_the_data() {
        const DATA_SLOT: u32 = 0x0300_0310;
        const STREAM_SLOT: u32 = 0x0300_0320;
        let mut e = Engine::new();
        let (this, data) = interpolator_with_data(&mut e);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x54, DATA_SLOT);
        e.mem.set_u32(data.addr(), vtable);
        let stream = e.mem.alloc(8);
        let stream_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream_vtable + 0x30, STREAM_SLOT);
        e.mem.set_u32(stream, stream_vtable);
        for f in [
            DATA_SLOT,
            STREAM_SLOT,
            OBJECT_SAVE_BINARY,
            QUAT_TRANSFORM_SAVE_BINARY,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(OBJECT_REGISTER_STREAMABLES, |_, a| {
            (a[1] == 0x77).into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a3_f870, &args![this, 0x66u32]).bool());
        assert!(calls_to(&e, DATA_SLOT).is_empty());
        assert!(e.call(0x00a3_f870, &args![this, 0x77u32]).bool());
        assert_eq!(calls_to(&e, DATA_SLOT), vec![vec![data.addr(), 0x77]]);
        e.call(0x00a3_f8a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, OBJECT_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(
            calls_to(&e, QUAT_TRANSFORM_SAVE_BINARY),
            vec![vec![this.addr() + 0xc, stream]]
        );
        assert_eq!(calls_to(&e, STREAM_SLOT), vec![vec![stream, data.addr()]]);
    }

    #[test]
    fn is_equal_compares_object_transform_and_data() {
        const DATA_EQUAL: u32 = 0x0300_0330;
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(QUAT_TRANSFORM_EQUAL, |_, _| 1u32.into_ret());
        e.register(DATA_EQUAL, |_, a| (a[1] != 0x4444).into_ret());
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x5c, DATA_EQUAL);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, vtable);
        let a = e.new_object::<NiTransformInterpolator>();
        let b = e.new_object::<NiTransformInterpolator>();
        let equal = |e: &mut Engine| e.call(0x00a3_f8d0, &args![a, b]).bool();
        assert!(equal(&mut e));
        e.set(a, NiTransformInterpolator::m_spData, data);
        assert!(!equal(&mut e));
        e.set(b, NiTransformInterpolator::m_spData, 0x5555);
        assert!(equal(&mut e));
        e.set(b, NiTransformInterpolator::m_spData, 0x4444);
        assert!(!equal(&mut e));
        e.register(QUAT_TRANSFORM_EQUAL, |_, _| 0u32.into_ret());
        e.set(b, NiTransformInterpolator::m_spData, 0x5555);
        assert!(!equal(&mut e));
    }

    #[test]
    fn fn_00a3f930_takes_the_data_and_resets_the_indices() {
        let mut e = Engine::new();
        counting(&mut e);
        let this = e.new_object::<NiTransformInterpolator>();
        let data = e.mem.alloc(8);
        e.mem.set_u32(data + 4, 1);
        e.set(this, NiTransformInterpolator::m_usLastTransIdx, 3);
        e.set(this, NiTransformInterpolator::m_usLastRotIdx, 4);
        e.set(this, NiTransformInterpolator::m_usLastScaleIdx, 5);
        e.call(0x00a3_f930, &args![this, data]);
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u16(this.addr() + 0x30), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x32), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x34), 0);
    }

    #[test]
    fn get_sequence_interpolator_gives_the_sequence_data() {
        let mut e = Engine::new();
        counting(&mut e);
        let (this, data) = interpolator_with_data(&mut e);
        let sequence = e.new_object::<NiTransformInterpolator>();
        e.set(sequence, NiTransformInterpolator::m_usLastRotIdx, 9);
        let sequence_addr = sequence.addr();
        e.register_double(INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, move |_, _| {
            sequence_addr.into_ret()
        });
        let part = e.mem.alloc(8);
        e.mem.set_u32(part + 4, 1);
        e.register_double(DATA_GET_SEQUENCE_DATA, move |e, a| {
            // Fills the stack pointer, counted once as a `NiPointer` assignment would.
            e.mem.set_u32(a[3], part);
            let count = e.mem.u32(part + 4);
            e.mem.set_u32(part + 4, count + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e.call(0x00a3_fd10, &args![this, 0.5f32, 1.5f32]).u32();
        assert_eq!(answer, sequence_addr);
        assert_eq!(
            calls_to(&e, DATA_GET_SEQUENCE_DATA)[0][..3],
            [data.addr(), 0.5f32.to_bits(), 1.5f32.to_bits()]
        );
        assert_eq!(e.get(sequence, NiTransformInterpolator::m_spData), part);
        // Counted by the data setter, released by the stack pointer: one reference left.
        assert_eq!(e.mem.u32(part + 4), 2);
        assert_eq!(e.mem.u16(sequence_addr + 0x32), 0);
        // No data: the base's answer alone.
        e.set(this, NiTransformInterpolator::m_spData, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a3_fd10, &args![this, 0.5f32, 1.5f32]).u32(),
            sequence_addr
        );
        assert!(calls_to(&e, DATA_GET_SEQUENCE_DATA).is_empty());
    }

    /// An engine for the collapse: the defaults mapped, the key replacements emptying the
    /// list they replace, points compared by their words.
    fn collapse_engine() -> Engine {
        let mut e = Engine::new();
        counting(&mut e);
        with_unset(&mut e);
        e.map(DEFAULT_TRANSLATE, 12);
        e.map(DEFAULT_ROTATE, 16);
        e.map(DEFAULT_SCALE, 4);
        floats(&mut e, DEFAULT_TRANSLATE, &[-1.0, -2.0, -3.0]);
        floats(&mut e, DEFAULT_ROTATE, &[-4.0, -5.0, -6.0, -7.0]);
        e.mem.set_f32(DEFAULT_SCALE, -8.0);
        e.register(DATA_SET_POSITION_KEYS, |e, a| {
            e.mem.set_u16(a[0] + 0xa, 0);
            Ret::default()
        });
        e.register(DATA_SET_ROTATION_KEYS, |e, a| {
            e.mem.set_u16(a[0] + 8, 0);
            Ret::default()
        });
        e.register(DATA_SET_SCALE_KEYS, |e, a| {
            e.mem.set_u16(a[0] + 0xc, 0);
            Ret::default()
        });
        e.register(POINTS_DIFFER, |e, a| {
            let differ = (0..3).any(|i| e.mem.u32(a[0] + 4 * i) != e.mem.u32(a[1] + 4 * i));
            differ.into_ret()
        });
        e
    }

    #[test]
    fn collapse_folds_constant_keys_and_releases_empty_data() {
        let mut e = collapse_engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let data = e.new_object::<NiTransformData>();
        e.mem.set_u32(data.addr() + 4, 2);
        e.set(this, NiTransformInterpolator::m_spData, data.addr());
        // Two position keys of type 1 with one value, one rotation key, two scale keys of
        // type 5 with one value.
        let pos = e.mem.alloc(0x20);
        floats(&mut e, pos, &[0.0, 1.0, 2.0, 3.0, 1.0, 1.0, 2.0, 3.0]);
        e.set(data, NiTransformData::m_usNumPosKeys, 2);
        e.set(data, NiTransformData::m_ePosType, 1);
        e.set(data, NiTransformData::m_ucPosSize, 0x10);
        e.set(data, NiTransformData::m_pkPosKeys, pos);
        let rot = e.mem.alloc(0x14);
        floats(&mut e, rot, &[0.0, 0.5, 0.25, 0.125, 1.0]);
        e.set(data, NiTransformData::m_usNumRotKeys, 1);
        e.set(data, NiTransformData::m_eRotType, 1);
        e.set(data, NiTransformData::m_pkRotKeys, rot);
        let scale = e.mem.alloc(0x10);
        floats(&mut e, scale, &[0.0, 4.0, 1.0, 4.0]);
        e.set(data, NiTransformData::m_usNumScaleKeys, 2);
        e.set(data, NiTransformData::m_eScaleType, 5);
        e.set(data, NiTransformData::m_ucScaleSize, 8);
        e.set(data, NiTransformData::m_pkScaleKeys, scale);
        e.call_log = Some(vec![]);
        e.call(0x00a3_ffd0, &args![this]);
        assert_eq!(
            read(&e, this.addr() + 0xc, 8),
            vec![1.0, 2.0, 3.0, 0.5, 0.25, 0.125, 1.0, 4.0]
        );
        let differ = calls_to(&e, POINTS_DIFFER);
        assert_eq!(differ.len(), 1);
        assert_eq!(differ[0][0], pos + 0x14);
        // Every list emptied: the data is released (one reference left) and dropped.
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), 0);
        assert_eq!(e.mem.u32(data.addr() + 4), 1);
    }

    #[test]
    fn collapse_keeps_varying_keys_and_the_own_values() {
        let mut e = collapse_engine();
        let this = e.new_object::<NiTransformInterpolator>();
        let data = e.new_object::<NiTransformData>();
        e.mem.set_u32(data.addr() + 4, 2);
        e.set(this, NiTransformInterpolator::m_spData, data.addr());
        floats(
            &mut e,
            this.addr() + 0xc,
            &[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
        );
        // Position: no keys (own value kept); rotation: two different keys of type 1 (own
        // value kept); scale: two keys of type 2 (not folded; the own scale is unset).
        let rot = e.mem.alloc(0x28);
        floats(
            &mut e,
            rot,
            &[0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0],
        );
        e.set(data, NiTransformData::m_usNumRotKeys, 2);
        e.set(data, NiTransformData::m_eRotType, 1);
        e.set(data, NiTransformData::m_ucRotSize, 0x14);
        e.set(data, NiTransformData::m_pkRotKeys, rot);
        let scale = e.mem.alloc(0x10);
        e.set(data, NiTransformData::m_usNumScaleKeys, 2);
        e.set(data, NiTransformData::m_eScaleType, 2);
        e.set(data, NiTransformData::m_pkScaleKeys, scale);
        e.set(this, NiTransformInterpolator::m_fScale, f32::MIN.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00a3_ffd0, &args![this]);
        assert_eq!(
            read(&e, this.addr() + 0xc, 8),
            vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, -8.0]
        );
        assert_eq!(calls_to(&e, DATA_SET_POSITION_KEYS).len(), 1);
        assert!(calls_to(&e, DATA_SET_ROTATION_KEYS).is_empty());
        assert!(calls_to(&e, DATA_SET_SCALE_KEYS).is_empty());
        assert_eq!(e.get(this, NiTransformInterpolator::m_spData), data.addr());
        // Without data nothing happens.
        let empty = e.new_object::<NiTransformInterpolator>();
        e.call(0x00a3_ffd0, &args![empty]);
        assert_eq!(e.mem.u32(empty.addr() + 0xc), 0);
    }

    #[test]
    fn get_viewer_strings_adds_class_transform_and_data() {
        const DATA_STRINGS: u32 = 0x0300_0340;
        let mut e = Engine::new();
        e.map(NI_TRANSFORM_INTERPOLATOR_RTTI, 4);
        e.mem.set_u32(NI_TRANSFORM_INTERPOLATOR_RTTI, 0x0108_0000);
        for f in [
            KEY_BASED_GET_VIEWER_STRINGS,
            STRINGS_GROW,
            STRINGS_SET_AT,
            QUAT_TRANSFORM_GET_VIEWER_STRINGS,
            DATA_STRINGS,
        ] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        let (this, data) = interpolator_with_data(&mut e);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0x60, DATA_STRINGS);
        e.mem.set_u32(data.addr(), vtable);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 3);
        e.call_log = Some(vec![]);
        e.call(0x00a4_04b0, &args![this, strings]);
        assert_eq!(
            calls_to(&e, KEY_BASED_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0108_0000]]);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 2]);
        assert_eq!(
            calls_to(&e, QUAT_TRANSFORM_GET_VIEWER_STRINGS),
            vec![vec![this.addr() + 0xc, strings]]
        );
        assert_eq!(calls_to(&e, DATA_STRINGS), vec![vec![data.addr(), strings]]);
    }
}
