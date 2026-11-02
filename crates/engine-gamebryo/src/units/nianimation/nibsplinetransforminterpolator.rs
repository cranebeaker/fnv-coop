//! `NiAnimation/nibsplinetransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3fe4;

// Translated from 00a4ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI)
}

// Translated from 00a50070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a57f80`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x48 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_b_spline_transform_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a5_7f80;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x48u32]);
    }
    this
}

/// `NiBSplineInterpolator`'s constructor (`00a57f20`; `thiscall`: data, basis data) and
/// member copy (`00a57fd0`), and `NiBSplineInterpolator::LoadBinary` (`00a58080`).
const B_SPLINE_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a5_7f20;
const B_SPLINE_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00a5_7fd0;
const B_SPLINE_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a5_8080;
/// `NiQuatTransform::LoadBinary` (`00a38bb0`; `thiscall` on the transform: stream).
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
/// The `NiBSplineTransformInterpolator` virtual table (`0109957c`).
const B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE: u32 = 0x0109_957c;
/// The words the constructor copies into the transform value: the invalid translation
/// (`011a8400`, three words), the invalid rotation (`011f3704`, four words) and the float
/// for the invalid scale (`01096b8c`, through the x87 stack).
const INVALID_TRANSLATE: u32 = 0x011a_8400;
const INVALID_ROTATE: u32 = 0x011f_3704;
const INVALID_SCALE: u32 = 0x0109_6b8c;
/// `NiQuaternion::FastNormalize`'s constants: the doubles at `01099680` and `01099690`, the
/// floats at `011f3fec` and `011f3ff0` (computed by the game's static initializers) and the
/// float at `01099688`.
const NORMALIZE_CENTER: u32 = 0x0109_9680;
const NORMALIZE_SECOND_LIMIT: u32 = 0x0109_9690;
const NORMALIZE_SLOPE: u32 = 0x011f_3fec;
const NORMALIZE_BASE: u32 = 0x011f_3ff0;
const NORMALIZE_THIRD_LIMIT: u32 = 0x0109_9688;

/// One 4-byte read into `destination` through the stream's input object (`NiStream`
/// `+0x24C`; read function at `+8`, `cdecl` (input, destination, 4, address of the
/// component size 4, 1)); the answer added to the input's word at `+4`.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 4);
        e.call(read, &args![input, destination, 4u32, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

// Translated from 00a4fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator`'s constructor (unnamed in the engine map; `thiscall`,
/// `RET 0x14`: data, translation handle, rotation handle, scale handle, basis data). The
/// `NiBSplineInterpolator` constructor (`00a57f20`, data and basis data), the class's
/// virtual table, the transform value set to the invalid translation, rotation and scale,
/// then the three control point handles. Answers `this`.
pub fn ni_b_spline_transform_interpolator_constructor(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    translate_handle: u32,
    rotate_handle: u32,
    scale_handle: u32,
    basis_data: u32,
) -> Ptr {
    e.call(
        B_SPLINE_INTERPOLATOR_CONSTRUCTOR,
        &args![this, data, basis_data],
    );
    let object = this.addr();
    e.mem
        .set_u32(object, B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE);
    for index in 0..3 {
        let word = e.global::<u32>(INVALID_TRANSLATE + 4 * index);
        e.mem.set_u32(object + 0x1c + 4 * index, word);
    }
    for index in 0..4 {
        let word = e.global::<u32>(INVALID_ROTATE + 4 * index);
        e.mem.set_u32(object + 0x28 + 4 * index, word);
    }
    let scale = x87_quiet_bits(e.global::<u32>(INVALID_SCALE));
    e.mem.set_u32(object + 0x38, scale);
    e.mem.set_u32(object + 0x3c, translate_handle);
    e.mem.set_u32(object + 0x40, rotate_handle);
    e.mem.set_u32(object + 0x44, scale_handle);
    this
}

// Translated from 00a4fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::CopyMembers` (unnamed in the engine map; `thiscall` on
/// the source, `RET 8`: clone, cloning process): the `NiBSplineInterpolator` copy
/// (`00a57fd0`), then the transform value (eight words, `REP MOVSD`) and the three control
/// point handles as they are.
pub fn ni_b_spline_transform_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    clone: Ptr,
    cloning: Ptr,
) {
    e.call(
        B_SPLINE_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    for offset in (0x1c..0x48).step_by(4) {
        let word = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.addr() + offset, word);
    }
}

// Translated from 00a50470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream):
/// `NiBSplineInterpolator::LoadBinary` (`00a58080`), the transform value
/// (`NiQuatTransform::LoadBinary`, `00a38bb0`, on `this + 0x1C`), then the three handles
/// (one 4-byte read each).
pub fn ni_b_spline_transform_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(B_SPLINE_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![this.addr() + 0x1c, stream],
    );
    read_word(e, stream, this.addr() + 0x3c);
    read_word(e, stream, this.addr() + 0x40);
    read_word(e, stream, this.addr() + 0x44);
}

/// One Newton step of [`ni_quaternion_fast_normalize`] on the stored estimate
/// `estimate` (float bits): `estimate * ((estimate^2 * length) - center) * slope + base`,
/// each intermediate stored as a float as the exe does.
fn newton_step(estimate: u32, length: f64, center: f64, slope: f64, base: f64) -> u32 {
    let value = x87_load32(estimate);
    let squared = x87_store32(x87_mul(x87_mul(value, value), length));
    let offset = x87_sub(x87_load32(squared), center);
    let factor = x87_store32(x87_add(x87_mul(offset, slope), base));
    x87_store32(x87_mul(value, x87_m32(factor)))
}

// Translated from 00a4ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::FastNormalize` (engine map; `this` in `ECX`, plain `RET`): scales the
/// quaternion (w, x, y, z at `+0`..`+0xC`) by an estimate of `1 / sqrt(length^2)`: a linear
/// guess around the double at `01099680` (slope and base from `011f3fec`, `011f3ff0`), one
/// Newton step when the squared length is at most the double at `01099690` and another
/// when it is at most the float at `01099688` (a NaN length takes none). x87 code, each
/// stored intermediate rounded to a float.
pub fn ni_quaternion_fast_normalize(e: &mut Engine, this: Ptr) {
    let q = this.addr();
    let y = x87_load32(e.mem.u32(q + 8));
    let x = x87_load32(e.mem.u32(q + 4));
    let z = x87_load32(e.mem.u32(q + 0xc));
    let w = x87_load32(e.mem.u32(q));
    let x_squared = x87_mul(x, x);
    let y_squared = x87_mul(y, y);
    let sum = x87_add(y_squared, x_squared);
    let z_squared = x87_mul(z, z);
    let sum = x87_add(sum, z_squared);
    let w_squared = x87_mul(w, w);
    let length = x87_load32(x87_store32(x87_add(sum, w_squared)));
    let center = x87_load64(e.mem.u64(NORMALIZE_CENTER));
    let slope = x87_load32(e.global::<u32>(NORMALIZE_SLOPE));
    let base = x87_load32(e.global::<u32>(NORMALIZE_BASE));
    let guess = x87_add(x87_mul(x87_sub(length, center), slope), base);
    let mut estimate = x87_store32(guess);
    // FCOM with the double: C3/C0 test, so at most the limit (ordered) iterates.
    if length <= f64::from_bits(e.mem.u64(NORMALIZE_SECOND_LIMIT)) {
        estimate = newton_step(estimate, length, center, slope, base);
        // FCOMP of the float limit with the length: no carry means limit >= length.
        if x87_load32(e.global::<u32>(NORMALIZE_THIRD_LIMIT)) >= length {
            estimate = newton_step(estimate, length, center, slope, base);
        }
    }
    let scale = x87_load32(estimate);
    let x = x87_load32(e.mem.u32(q + 4));
    e.mem.set_u32(q + 4, x87_store32(x87_mul(x, scale)));
    let y = x87_mul(scale, x87_m32(e.mem.u32(q + 8)));
    e.mem.set_u32(q + 8, x87_store32(y));
    let z = x87_load32(e.mem.u32(q + 0xc));
    e.mem.set_u32(q + 0xc, x87_store32(x87_mul(z, scale)));
    let w = x87_mul(scale, x87_m32(e.mem.u32(q)));
    e.mem.set_u32(q, x87_store32(w));
}

/// `NiBSplineTransformInterpolator` (Xbox PDB, 0x48 bytes; the PC code uses
/// the same offsets): `m_fLastTime` +0x08, `m_fStartTime` +0x0C,
/// `m_fEndTime` +0x10, `m_spData` +0x14, `m_spBasisData` +0x18,
/// `m_kTransformValue` (`NiQuatTransform`: translation +0x1C, rotation
/// +0x28 as w, x, y, z, scale +0x38) and the control point handles
/// `m_kTransCPHandle` +0x3C, `m_kRotCPHandle` +0x40, `m_kScaleCPHandle`
/// +0x44.
const LAST_TIME: u32 = 0x08;
const START_TIME: u32 = 0x0c;
const END_TIME: u32 = 0x10;
const SPLINE_DATA: u32 = 0x14;
const BASIS_DATA: u32 = 0x18;
const TRANSFORM: u32 = 0x1c;
const ROTATION: u32 = 0x28;
const ROTATION_X: u32 = 0x2c;
const SCALE: u32 = 0x38;
const TRANSLATE_HANDLE: u32 = 0x3c;
const ROTATE_HANDLE: u32 = 0x40;
const SCALE_HANDLE: u32 = 0x44;
/// A handle that names no control points.
const INVALID_HANDLE: u32 = 0xffff;
/// The class's size (the `CreateClone` and `CreateObject` allocations).
const NI_B_SPLINE_TRANSFORM_INTERPOLATOR_SIZE: u32 = 0x48;
/// The `double` -FLT_MAX (`010241b0`) a transform part holds while unset.
const INVALID_PART: u32 = 0x0102_41b0;
/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiBSplineInterpolator::IsEqual` (`00a57e90`), `::SaveBinary`
/// (`00a58160`) and `::GetViewerStrings` (`00a58200`), `thiscall`.
const B_SPLINE_INTERPOLATOR_IS_EQUAL: u32 = 0x00a5_7e90;
const B_SPLINE_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a5_8160;
const B_SPLINE_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_8200;
/// `00a391c0` (`thiscall` on a transform, `RET 4`): true when the
/// transform equals the one at its argument; `NiQuatTransform::SaveBinary`
/// (`00a38b60`) and `NiQuatTransform::GetViewerStrings` (`00a38cb0`), both
/// `thiscall` on the transform.
const QUAT_TRANSFORM_EQUALS: u32 = 0x00a3_91c0;
const QUAT_TRANSFORM_SAVE_BINARY: u32 = 0x00a3_8b60;
const QUAT_TRANSFORM_GET_VIEWER_STRINGS: u32 = 0x00a3_8cb0;
/// `NiBSplineData::GetValueDegree3` (`00a52550`; `thiscall` on the data,
/// `RET 0x14`: time, output floats, dimension, basis data, handle).
const GET_VALUE_DEGREE3: u32 = 0x00a5_2550;
/// `00a6db50` (`thiscall` on a quaternion, `RET 0x10`): stores its four
/// float arguments as w, x, y, z.
const QUATERNION_CONSTRUCT: u32 = 0x00a6_db50;
/// Virtual slot +0xE0 (`GetControlPointCount`, Xbox PDB name): the control
/// point count of a channel.
const SLOT_GET_CONTROL_POINT_COUNT: u32 = 0xe0;
/// `NiStream::m_pkOstr` (Xbox PDB) +0x250 and the output stream's position
/// (+4) and write function (+0xc).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and
/// `NiGetViewerString_ov8` (engine map; `00a7d510`, `cdecl`: label, value);
/// the viewer-string array's grow (`0096ad30`) and store (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_VALUE: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_kTransCPHandle"`, `"m_kRotCPHandle"` and `"m_kScaleCPHandle"`.
const LABEL_TRANSLATE_HANDLE: u32 = 0x0109_96bc;
const LABEL_ROTATE_HANDLE: u32 = 0x0109_96ac;
const LABEL_SCALE_HANDLE: u32 = 0x0109_9698;

/// Appends `text` to the viewer-string array at `strings` (grown by its
/// step when full), through a stack word holding it.
fn add_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// The transform value's three validity tests, in the exe's order: the
/// scale, the rotation's x and the translation's x each compared with the
/// invalid -FLT_MAX (`FUCOM`/`FUCOMPP`: a NaN counts as set); true when one
/// is set.
fn transform_is_set(e: &mut Engine, this: u32) -> bool {
    let invalid = f64::from_bits(e.mem.u64(INVALID_PART));
    for part in [SCALE, ROTATION_X, TRANSFORM] {
        if x87_load32(e.mem.u32(this.wrapping_add(part))) != invalid {
            return true;
        }
    }
    false
}

/// Copies the eight words of the transform value to `value` (`REP MOVSD`).
fn copy_transform(e: &mut Engine, this: u32, value: u32) {
    for index in 0..8 {
        let word = e.mem.u32(this.wrapping_add(TRANSFORM + 4 * index));
        e.mem.set_u32(value.wrapping_add(4 * index), word);
    }
}

// Translated from 00a4fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetDimension` (Xbox PDB; `this` is not
/// read): 3 for channel 0 (translation), 4 for 1 (rotation), 1 for 2
/// (scale), 0 otherwise.
pub fn ni_b_spline_transform_interpolator_get_dimension(
    _e: &mut Engine,
    _this: Ptr,
    channel: u16,
) -> u32 {
    match channel {
        0 => 3,
        1 => 4,
        2 => 1,
        _ => 0,
    }
}

// Translated from 00a4fe20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetDegree` (Xbox PDB; `this` is not
/// read): 3 for channels 0 to 2, 0 otherwise.
pub fn ni_b_spline_transform_interpolator_get_degree(
    _e: &mut Engine,
    _this: Ptr,
    channel: u16,
) -> u32 {
    match channel {
        0..=2 => 3,
        _ => 0,
    }
}

// Translated from 00a4fe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetControlHandle` (Xbox PDB): the
/// translation, rotation or scale handle for channel 0, 1 or 2, 0xFFFF
/// otherwise.
pub fn ni_b_spline_transform_interpolator_get_control_handle(
    e: &mut Engine,
    this: Ptr,
    channel: u16,
) -> u32 {
    let offset = match channel {
        0 => TRANSLATE_HANDLE,
        1 => ROTATE_HANDLE,
        2 => SCALE_HANDLE,
        _ => return INVALID_HANDLE,
    };
    e.mem.u32(this.addr().wrapping_add(offset))
}

// Translated from 00a4fe80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::SetControlHandle` (Xbox PDB; `RET 8`:
/// handle, channel): stores the handle for channel 0, 1 or 2; nothing for
/// another channel.
pub fn ni_b_spline_transform_interpolator_set_control_handle(
    e: &mut Engine,
    this: Ptr,
    handle: u32,
    channel: u16,
) {
    let offset = match channel {
        0 => TRANSLATE_HANDLE,
        1 => ROTATE_HANDLE,
        2 => SCALE_HANDLE,
        _ => return,
    };
    e.mem.set_u32(this.addr().wrapping_add(offset), handle);
}

// Translated from 00a50090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetChannelPosed` (Xbox PDB): for
/// channels 0, 1 and 2, true when the channel has no control points
/// (`GetControlPointCount`, slot +0xE0, given the whole channel word) and
/// its part of the transform value (translation x, rotation x, scale) is
/// not the invalid -FLT_MAX (`FCOMP` with the double: a NaN counts as
/// set); false otherwise.
pub fn ni_b_spline_transform_interpolator_get_channel_posed(
    e: &mut Engine,
    this: Ptr,
    channel: u32,
) -> bool {
    let part = match channel as u16 {
        0 => TRANSFORM,
        1 => ROTATION_X,
        2 => SCALE,
        _ => return false,
    };
    let object = this.addr();
    if e.vcall(object, SLOT_GET_CONTROL_POINT_COUNT, &args![channel])
        .u32()
        != 0
    {
        return false;
    }
    let invalid = f64::from_bits(e.mem.u64(INVALID_PART));
    x87_load32(e.mem.u32(object.wrapping_add(part))) != invalid
}

/// The constructor `CreateClone` and `CreateObject` run: no data, invalid
/// handles, no basis data.
fn construct_empty(e: &mut Engine, block: u32) -> u32 {
    ni_b_spline_transform_interpolator_constructor(
        e,
        Ptr::new(block),
        0,
        INVALID_HANDLE,
        INVALID_HANDLE,
        INVALID_HANDLE,
        0,
    )
    .addr()
}

// Translated from 00a50110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::CreateClone` (Xbox PDB; `RET 4`:
/// cloning process): a 0x48-byte interpolator from the constructor (no
/// data, handles 0xFFFF; 0 when the allocation fails), the
/// `NiBSplineInterpolator` copy (`00a57fd0`) into it, then the transform
/// value (eight words) and the three handles copied (also through a null
/// clone).
pub fn ni_b_spline_transform_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr,
    cloning: Ptr,
) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_B_SPLINE_TRANSFORM_INTERPOLATOR_SIZE])
        .u32();
    let clone = if block == 0 {
        0
    } else {
        construct_empty(e, block)
    };
    e.call(
        B_SPLINE_INTERPOLATOR_COPY_MEMBERS,
        &args![this, clone, cloning],
    );
    let source = this.addr();
    for offset in (TRANSFORM..TRANSLATE_HANDLE).step_by(4) {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(clone.wrapping_add(offset), word);
    }
    for offset in [TRANSLATE_HANDLE, ROTATE_HANDLE, SCALE_HANDLE] {
        let word = e.mem.u32(source.wrapping_add(offset));
        e.mem.set_u32(clone.wrapping_add(offset), word);
    }
    Ptr::new(clone)
}

// Translated from 00a50180 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::CreateObject` (Ghidra has no function
/// here; the loader's factory by its body): allocates 0x48 bytes and runs
/// the constructor on them (no data, handles 0xFFFF, no basis data).
/// Returns the object, or 0.
pub fn ni_b_spline_transform_interpolator_create_object(e: &mut Engine) -> Ptr {
    let block = e
        .call(NI_ALLOCATE, &args![NI_B_SPLINE_TRANSFORM_INTERPOLATOR_SIZE])
        .u32();
    if block == 0 {
        return Ptr::new(0);
    }
    Ptr::new(construct_empty(e, block))
}

// Translated from 00a501b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::IsEqual` (Xbox PDB):
/// `NiBSplineInterpolator::IsEqual`, equal transform values (`00a391c0`)
/// and the same three handles.
pub fn ni_b_spline_transform_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e
        .call(B_SPLINE_INTERPOLATOR_IS_EQUAL, &args![this, other])
        .bool()
    {
        return false;
    }
    let object = this.addr();
    let other = other.addr();
    if !e
        .call(
            QUAT_TRANSFORM_EQUALS,
            &args![
                object.wrapping_add(TRANSFORM),
                other.wrapping_add(TRANSFORM)
            ],
        )
        .bool()
    {
        return false;
    }
    [TRANSLATE_HANDLE, ROTATE_HANDLE, SCALE_HANDLE]
        .iter()
        .all(|offset| {
            e.mem.u32(other.wrapping_add(*offset)) == e.mem.u32(object.wrapping_add(*offset))
        })
}

// Translated from 00a50200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::Update` (Xbox PDB, the transform
/// `Update`; `RET 0xC`: time, target, address of the value; the target is
/// not read). At the time of the last update (`FUCOMP`: a NaN is never
/// equal) the transform value is copied out. Otherwise the normalized time
/// `(time - start) / (end - start)` (stored as a float) evaluates each
/// channel whose handle is not 0xFFFF through
/// `NiBSplineData::GetValueDegree3` (dimension 3, 4, 1): the translation
/// (three floats), the rotation (four floats made a quaternion by
/// `00a6db50` and normalized by `NiQuaternion::FastNormalize`) and the
/// scale; the transform value is then copied out. Either way the answer is
/// whether one part of the value is set (scale, rotation x, translation x
/// against -FLT_MAX); only the evaluating path stores `time` as the last
/// time, and only then.
pub fn ni_b_spline_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    _unused_2: u32,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let time_bits = x87_quiet_bits(time.to_bits());
    let last_time = x87_load32(e.mem.u32(object.wrapping_add(LAST_TIME)));
    if x87_load32(time_bits) == last_time {
        copy_transform(e, object, value.addr());
        return transform_is_set(e, object);
    }
    let start = e.mem.u32(object.wrapping_add(START_TIME));
    let offset = x87_sub(x87_load32(time_bits), x87_m32(start));
    let end = x87_load32(e.mem.u32(object.wrapping_add(END_TIME)));
    let span = x87_sub(end, x87_m32(start));
    let normalized = x87_store32(x87_div(offset, span));
    // The frame: the quaternion +0 (16 bytes), the evaluated floats +0x10.
    e.with_stack(0x20, |e, frame| {
        let quaternion = frame.addr();
        let output = quaternion + 0x10;
        let data = e.mem.u32(object.wrapping_add(SPLINE_DATA));
        let handle = e.mem.u32(object.wrapping_add(TRANSLATE_HANDLE));
        if handle != INVALID_HANDLE {
            let basis = e.mem.u32(object.wrapping_add(BASIS_DATA));
            e.call(
                GET_VALUE_DEGREE3,
                &args![data, normalized, output, 3u32, basis, handle],
            );
            for index in 0..3 {
                let word = x87_quiet_bits(e.mem.u32(output + 4 * index));
                e.mem.set_u32(quaternion + 4 * index, word);
                e.mem
                    .set_u32(object.wrapping_add(TRANSFORM + 4 * index), word);
            }
        }
        let handle = e.mem.u32(object.wrapping_add(ROTATE_HANDLE));
        if handle != INVALID_HANDLE {
            let basis = e.mem.u32(object.wrapping_add(BASIS_DATA));
            let data = e.mem.u32(object.wrapping_add(SPLINE_DATA));
            e.call(
                GET_VALUE_DEGREE3,
                &args![data, normalized, output, 4u32, basis, handle],
            );
            let [w, x, y, z] =
                [0u32, 4, 8, 12].map(|offset| x87_quiet_bits(e.mem.u32(output + offset)));
            e.call(QUATERNION_CONSTRUCT, &args![quaternion, w, x, y, z]);
            ni_quaternion_fast_normalize(e, Ptr::new(quaternion));
            for index in 0..4 {
                let word = e.mem.u32(quaternion + 4 * index);
                e.mem
                    .set_u32(object.wrapping_add(ROTATION + 4 * index), word);
            }
        }
        let handle = e.mem.u32(object.wrapping_add(SCALE_HANDLE));
        if handle != INVALID_HANDLE {
            let basis = e.mem.u32(object.wrapping_add(BASIS_DATA));
            let data = e.mem.u32(object.wrapping_add(SPLINE_DATA));
            e.call(
                GET_VALUE_DEGREE3,
                &args![data, normalized, output, 1u32, basis, handle],
            );
            let scale = x87_quiet_bits(e.mem.u32(output));
            e.mem.set_u32(object.wrapping_add(SCALE), scale);
        }
    });
    copy_transform(e, object, value.addr());
    if !transform_is_set(e, object) {
        return false;
    }
    e.mem.set_u32(object.wrapping_add(LAST_TIME), time_bits);
    true
}

// Translated from 00a503e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::SaveBinary` (Xbox PDB):
/// `NiBSplineInterpolator::SaveBinary`, `NiQuatTransform::SaveBinary` of the
/// transform value, then the three handles (4 bytes each through the
/// output stream's write function, the component size in the argument's own
/// stack word).
pub fn ni_b_spline_transform_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(B_SPLINE_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    let object = this.addr();
    e.call(
        QUAT_TRANSFORM_SAVE_BINARY,
        &args![object.wrapping_add(TRANSFORM), stream],
    );
    e.with_stack(4, |e, component_size| {
        for offset in [TRANSLATE_HANDLE, ROTATE_HANDLE, SCALE_HANDLE] {
            let output = e.mem.u32(stream.addr().wrapping_add(STREAM_OUTPUT));
            let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
            e.mem.set_u32(component_size.addr(), 4);
            let bytes = e
                .call(
                    write,
                    &args![
                        output,
                        object.wrapping_add(offset),
                        4u32,
                        component_size,
                        1u32
                    ],
                )
                .u32();
            let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
            e.mem.set_u32(
                output.wrapping_add(BINARY_STREAM_POSITION),
                position.wrapping_add(bytes),
            );
        }
    });
}

// Translated from 00a50500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineTransformInterpolator::GetViewerStrings` (Xbox PDB):
/// `NiBSplineInterpolator::GetViewerStrings`, the class name (the `NiRTTI`
/// record's first word through `NiGetViewerString`), the transform value's
/// strings (`NiQuatTransform::GetViewerStrings`), then the three handles
/// (`NiGetViewerString_ov8` with `"m_kTransCPHandle"`, `"m_kRotCPHandle"`,
/// `"m_kScaleCPHandle"`).
pub fn ni_b_spline_transform_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    let object = this.addr();
    let strings = strings.addr();
    e.call(
        B_SPLINE_INTERPOLATOR_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let name = e.mem.u32(NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    e.call(
        QUAT_TRANSFORM_GET_VIEWER_STRINGS,
        &args![object.wrapping_add(TRANSFORM), strings],
    );
    for (label, offset) in [
        (LABEL_TRANSLATE_HANDLE, TRANSLATE_HANDLE),
        (LABEL_ROTATE_HANDLE, ROTATE_HANDLE),
        (LABEL_SCALE_HANDLE, SCALE_HANDLE),
    ] {
        let handle = e.mem.u32(object.wrapping_add(offset));
        let text = e
            .call(NI_GET_VIEWER_STRING_VALUE, &args![label, handle])
            .u32();
        add_viewer_string(e, strings, text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4ffe0, ni_b_spline_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a50070, ni_b_spline_transform_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_fff0, ni_b_spline_transform_interpolator_constructor(Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00a4_fec0,
            ni_b_spline_transform_interpolator_copy_members(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a5_0470,
            ni_b_spline_transform_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_ff00, ni_quaternion_fast_normalize(Ptr)),
        entry!(
            0x00a4_fde0,
            ni_b_spline_transform_interpolator_get_dimension(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_fe20,
            ni_b_spline_transform_interpolator_get_degree(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_fe50,
            ni_b_spline_transform_interpolator_get_control_handle(Ptr, u16) -> u32
        ),
        entry!(
            0x00a4_fe80,
            ni_b_spline_transform_interpolator_set_control_handle(Ptr, u32, u16)
        ),
        entry!(
            0x00a5_0090,
            ni_b_spline_transform_interpolator_get_channel_posed(Ptr, u32) -> bool
        ),
        entry!(
            0x00a5_0110,
            ni_b_spline_transform_interpolator_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00a5_0180,
            ni_b_spline_transform_interpolator_create_object() -> Ptr
        ),
        entry!(
            0x00a5_01b0,
            ni_b_spline_transform_interpolator_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_0200,
            ni_b_spline_transform_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a5_03e0,
            ni_b_spline_transform_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a5_0500,
            ni_b_spline_transform_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4ffe0, &args![]).u32(),
            NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_b_spline_transform_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00a50070, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a50070, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x48]]);
        assert_eq!(calls_to(&e, 0x00a5_7f80), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod b037_tests {
    use super::*;

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
    fn constructor_sets_the_invalid_transform_and_the_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(INVALID_TRANSLATE, 12);
        e.map(INVALID_ROTATE, 16);
        e.map(INVALID_SCALE, 4);
        for index in 0..3 {
            e.set_global(INVALID_TRANSLATE + 4 * index, 0x100 + index);
        }
        for index in 0..4 {
            e.set_global(INVALID_ROTATE + 4 * index, 0x200 + index);
        }
        e.set_global(INVALID_SCALE, 0xff80_0001u32);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00a4_fff0,
                &args![this, 0x11u32, 1u32, 2u32, 3u32, 0x22u32],
            )
            .u32();
        assert_eq!(answer, this);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![this, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(this), B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + 0x1c), 0x100);
        assert_eq!(e.mem.u32(this + 0x24), 0x102);
        assert_eq!(e.mem.u32(this + 0x28), 0x200);
        assert_eq!(e.mem.u32(this + 0x34), 0x203);
        assert_eq!(e.mem.u32(this + 0x38), 0xffc0_0001);
        assert_eq!(
            (
                e.mem.u32(this + 0x3c),
                e.mem.u32(this + 0x40),
                e.mem.u32(this + 0x44)
            ),
            (1, 2, 3)
        );
    }

    #[test]
    fn copy_members_copies_the_transform_and_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        let clone = e.mem.alloc(0x4c);
        for offset in (0x1c..0x48).step_by(4) {
            e.mem.set_u32(this + offset, 0x7f80_0000 + offset);
        }
        e.mem.set_u32(clone + 0x48, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x00a4_fec0, &args![this, clone, 0x900u32]);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, clone, 0x900]]
        );
        for offset in (0x1c..0x48).step_by(4) {
            assert_eq!(e.mem.u32(clone + offset), 0x7f80_0000 + offset);
        }
        assert_eq!(e.mem.u32(clone + 0x48), 0x5555);
    }

    #[test]
    fn load_binary_reads_the_transform_and_three_handles() {
        let mut e = Engine::new();
        e.register(B_SPLINE_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_LOAD_BINARY, |_, _| Ret::default());
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[2], a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], a[1] & 0xff);
            4u32.into_ret()
        });
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        e.call(0x00a5_0470, &args![this, stream]);
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_LOAD_BINARY),
            vec![vec![this + 0x1c, stream]]
        );
        let reads: Vec<u32> = calls(&e, READ).iter().map(|c| c[1]).collect();
        assert_eq!(reads, vec![this + 0x3c, this + 0x40, this + 0x44]);
        assert_eq!(e.mem.u32(input + 4), 12);
    }

    fn normalize_constants(e: &mut Engine) {
        e.map(NORMALIZE_CENTER, 8);
        e.mem.set_u64(NORMALIZE_CENTER, 0.959066f64.to_bits());
        e.map(NORMALIZE_SECOND_LIMIT, 8);
        e.mem.set_u64(NORMALIZE_SECOND_LIMIT, 0.915212f64.to_bits());
        e.map(NORMALIZE_THIRD_LIMIT, 4);
        e.set_global(NORMALIZE_THIRD_LIMIT, 0.652_119_7f32.to_bits());
        // Slope and base of the tangent of 1/sqrt at the center.
        let center = 0.959066f64;
        let slope = -0.5 * center.powf(-1.5);
        let base = center.powf(-0.5);
        e.map(NORMALIZE_SLOPE, 8);
        e.set_global(NORMALIZE_SLOPE, (slope as f32).to_bits());
        e.set_global(NORMALIZE_BASE, (base as f32).to_bits());
    }

    fn normalized(scale: f32) -> f64 {
        let mut e = Engine::new();
        normalize_constants(&mut e);
        let q = e.mem.alloc(16);
        for (index, part) in [0.5f32, 0.5, 0.5, 0.5].iter().enumerate() {
            e.mem
                .set_u32(q + 4 * index as u32, (part * scale).to_bits());
        }
        e.call(0x00a4_ff00, &args![q]);
        (0..4)
            .map(|index| f64::from(e.mem.f32(q + 4 * index)).powi(2))
            .sum()
    }

    #[test]
    fn fast_normalize_brings_the_length_near_one() {
        // Lengths near 1 (no step), below the second limit (one step), below the third (two).
        for scale in [1.0f32, 0.95, 0.85, 0.7] {
            let length = normalized(scale);
            assert!((length - 1.0).abs() < 0.02, "{scale} {length}");
        }
    }

    #[test]
    fn fast_normalize_scales_every_component_alike() {
        let mut e = Engine::new();
        normalize_constants(&mut e);
        let q = e.mem.alloc(16);
        for (index, part) in [0.1f32, 0.2, 0.3, 0.9].iter().enumerate() {
            e.mem.set_u32(q + 4 * index as u32, part.to_bits());
        }
        e.call(0x00a4_ff00, &args![q]);
        let w = e.mem.f32(q);
        let z = e.mem.f32(q + 0xc);
        assert!((f64::from(z) / f64::from(w) - 9.0).abs() < 1e-5);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    const CONTROL_POINTS: u32 = 0x0350_0000;
    const WRITE: u32 = 0x0350_0010;
    const INVALID: u32 = 0xff7f_ffff;

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
        e.map(INVALID_PART, 8);
        e.mem
            .set_u64(INVALID_PART, (-f64::from(f32::MAX)).to_bits());
        e.call_log = Some(vec![]);
        e
    }

    /// An interpolator with a vtable whose `GetControlPointCount` answers
    /// the word at +0x48 of the vtable block, and the transform parts set
    /// to the invalid float.
    fn interpolator(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + SLOT_GET_CONTROL_POINT_COUNT, CONTROL_POINTS);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, vtable);
        for offset in (TRANSFORM..TRANSLATE_HANDLE).step_by(4) {
            e.mem.set_u32(this + offset, INVALID);
        }
        this
    }

    #[test]
    fn get_dimension_by_channel() {
        let mut e = Engine::new();
        let answers: Vec<u32> = (0..4u32)
            .map(|channel| e.call(0x00a4_fde0, &args![0u32, channel]).u32())
            .collect();
        assert_eq!(answers, [3, 4, 1, 0]);
        assert_eq!(e.call(0x00a4_fde0, &args![0u32, 0x1_0001u32]).u32(), 4);
    }

    #[test]
    fn get_degree_by_channel() {
        let mut e = Engine::new();
        let answers: Vec<u32> = (0..4u32)
            .map(|channel| e.call(0x00a4_fe20, &args![0u32, channel]).u32())
            .collect();
        assert_eq!(answers, [3, 3, 3, 0]);
    }

    #[test]
    fn get_control_handle_by_channel() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + TRANSLATE_HANDLE, 7);
        e.mem.set_u32(this + ROTATE_HANDLE, 8);
        e.mem.set_u32(this + SCALE_HANDLE, 9);
        let answers: Vec<u32> = (0..4u32)
            .map(|channel| e.call(0x00a4_fe50, &args![this, channel]).u32())
            .collect();
        assert_eq!(answers, [7, 8, 9, 0xffff]);
    }

    #[test]
    fn set_control_handle_by_channel() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x4c);
        for channel in 0..4u32 {
            e.call(0x00a4_fe80, &args![this, 0x10 + channel, channel]);
        }
        assert_eq!(
            [
                e.mem.u32(this + TRANSLATE_HANDLE),
                e.mem.u32(this + ROTATE_HANDLE),
                e.mem.u32(this + SCALE_HANDLE),
                e.mem.u32(this + 0x48)
            ],
            [0x10, 0x11, 0x12, 0]
        );
    }

    #[test]
    fn get_channel_posed_needs_no_control_points_and_a_set_part() {
        let mut e = engine();
        let mut counts = vec![0u32, 0, 0, 2, 0];
        e.register_double(CONTROL_POINTS, move |_, _| counts.remove(0).into_ret());
        let this = interpolator(&mut e);
        e.mem.set_f32(this + TRANSFORM, 1.0);
        e.mem.set_u32(this + SCALE, 0x7fc0_0000);
        // Translation set, rotation unset, scale a NaN (counts as set).
        assert!(e.call(0x00a5_0090, &args![this, 0u32]).bool());
        assert!(!e.call(0x00a5_0090, &args![this, 1u32]).bool());
        assert!(e.call(0x00a5_0090, &args![this, 2u32]).bool());
        // Control points: false.
        assert!(!e.call(0x00a5_0090, &args![this, 0u32]).bool());
        // The whole word goes to the slot; another channel asks nothing.
        assert!(e.call(0x00a5_0090, &args![this, 0x1_0000u32]).bool());
        assert!(!e.call(0x00a5_0090, &args![this, 3u32]).bool());
        let words: Vec<u32> = calls(&e, CONTROL_POINTS).iter().map(|c| c[1]).collect();
        assert_eq!(words, [0, 1, 2, 0, 0x1_0000]);
    }

    /// Maps the constructor's invalid transform words and doubles the base
    /// constructor.
    fn constructor_globals(e: &mut Engine) {
        e.register(B_SPLINE_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.map(INVALID_TRANSLATE, 12);
        e.map(INVALID_ROTATE, 16);
        e.map(INVALID_SCALE, 4);
        e.set_global(INVALID_SCALE, INVALID);
    }

    #[test]
    fn create_clone_copies_the_transform_and_handles() {
        let mut e = engine();
        constructor_globals(&mut e);
        e.register(B_SPLINE_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x4c);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        let this = e.mem.alloc(0x48);
        for offset in (TRANSFORM..0x48).step_by(4) {
            e.mem.set_u32(this + offset, 0x100 + offset);
        }
        assert_eq!(e.call(0x00a5_0110, &args![this, 0x900u32]).u32(), block);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_CONSTRUCTOR),
            vec![vec![block, 0, 0]]
        );
        assert_eq!(e.mem.u32(block), B_SPLINE_TRANSFORM_INTERPOLATOR_VTABLE);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this, block, 0x900]]
        );
        for offset in (TRANSFORM..0x48).step_by(4) {
            assert_eq!(e.mem.u32(block + offset), 0x100 + offset);
        }
        // A failed allocation copies into address 0.
        e.map(0, 0x100);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_0110, &args![this, 0x900u32]).u32(), 0);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_COPY_MEMBERS)[1],
            vec![this, 0, 0x900]
        );
        assert_eq!(e.mem.u32(SCALE_HANDLE), 0x100 + SCALE_HANDLE);
    }

    #[test]
    fn create_object_constructs_an_empty_interpolator() {
        let mut e = engine();
        constructor_globals(&mut e);
        let block = e.mem.alloc(0x48);
        e.register_double(NI_ALLOCATE, move |_, _| block.into_ret());
        assert_eq!(e.call(0x00a5_0180, &args![]).u32(), block);
        assert_eq!(
            [
                e.mem.u32(block + TRANSLATE_HANDLE),
                e.mem.u32(block + ROTATE_HANDLE),
                e.mem.u32(block + SCALE_HANDLE)
            ],
            [0xffff; 3]
        );
        assert_eq!(e.mem.u32(block + SCALE), INVALID);
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a5_0180, &args![]).u32(), 0);
        assert_eq!(calls(&e, B_SPLINE_INTERPOLATOR_CONSTRUCTOR).len(), 1);
    }

    #[test]
    fn is_equal_needs_base_transform_and_handles() {
        let mut e = engine();
        e.register(B_SPLINE_INTERPOLATOR_IS_EQUAL, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(QUAT_TRANSFORM_EQUALS, |e, a| {
            u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let this = e.mem.alloc(0x48);
        let other = e.mem.alloc(0x48);
        e.mem.set_u32(this + 4, 1);
        assert!(e.call(0x00a5_01b0, &args![this, other]).bool());
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_EQUALS),
            vec![vec![this + TRANSFORM, other + TRANSFORM]]
        );
        for offset in [TRANSLATE_HANDLE, ROTATE_HANDLE, SCALE_HANDLE, TRANSFORM] {
            e.mem.set_u32(other + offset, 5);
            assert!(!e.call(0x00a5_01b0, &args![this, other]).bool());
            e.mem.set_u32(other + offset, 0);
        }
        e.mem.set_u32(this + 4, 0);
        assert!(!e.call(0x00a5_01b0, &args![this, other]).bool());
    }

    #[test]
    fn update_at_the_last_time_copies_the_value() {
        let mut e = engine();
        let this = interpolator(&mut e);
        let value = e.mem.alloc(0x20);
        e.mem.set_f32(this + LAST_TIME, 2.0);
        // Everything invalid: copied, false.
        assert!(!e
            .call(0x00a5_0200, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.u32(value + 0x1c), INVALID);
        // Only the translation set: true, the time unchanged.
        e.mem.set_f32(this + TRANSFORM, 1.5);
        assert!(e
            .call(0x00a5_0200, &args![this, 2.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(value), 1.5);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
    }

    #[test]
    fn update_evaluates_each_channel_with_a_handle() {
        let mut e = engine();
        e.register(GET_VALUE_DEGREE3, |e, a| {
            for i in 0..a[3] {
                e.mem.set_f32(a[2] + 4 * i, 0.5 * (a[3] + i) as f32);
            }
            Ret::default()
        });
        e.register(QUATERNION_CONSTRUCT, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            Ret::default()
        });
        // FastNormalize's constants: no Newton steps, scale 1 at length 1.
        e.map(NORMALIZE_CENTER, 8);
        e.mem.set_u64(NORMALIZE_CENTER, 0f64.to_bits());
        e.map(NORMALIZE_SECOND_LIMIT, 8);
        e.mem.set_u64(NORMALIZE_SECOND_LIMIT, (-1f64).to_bits());
        e.map(NORMALIZE_SLOPE, 8);
        e.set_global(NORMALIZE_SLOPE, 0u32);
        e.set_global(NORMALIZE_BASE, 1.0f32.to_bits());
        let this = interpolator(&mut e);
        e.mem.set_f32(this + START_TIME, 1.0);
        e.mem.set_f32(this + END_TIME, 5.0);
        e.mem.set_u32(this + SPLINE_DATA, 0x6000);
        e.mem.set_u32(this + BASIS_DATA, 0x6100);
        e.mem.set_u32(this + TRANSLATE_HANDLE, 1);
        e.mem.set_u32(this + ROTATE_HANDLE, 2);
        e.mem.set_u32(this + SCALE_HANDLE, 0xffff);
        let value = e.mem.alloc(0x20);
        assert!(e
            .call(0x00a5_0200, &args![this, 2.0f32, 0u32, value])
            .bool());
        let evaluations = calls(&e, GET_VALUE_DEGREE3);
        assert_eq!(evaluations.len(), 2);
        assert_eq!(
            (evaluations[0][0], evaluations[0][1], evaluations[0][3]),
            (0x6000, 0.25f32.to_bits(), 3)
        );
        assert_eq!(evaluations[0][4..], [0x6100, 1]);
        assert_eq!(evaluations[1][3..], [4, 0x6100, 2]);
        let translate: Vec<f32> = (0..3)
            .map(|i| e.mem.f32(this + TRANSFORM + 4 * i))
            .collect();
        assert_eq!(translate, [1.5, 2.0, 2.5]);
        let rotate: Vec<f32> = (0..4).map(|i| e.mem.f32(this + ROTATION + 4 * i)).collect();
        assert_eq!(rotate, [2.0, 2.5, 3.0, 3.5]);
        assert_eq!(e.mem.u32(this + SCALE), INVALID);
        assert_eq!(e.mem.f32(value + 0xc), 2.0);
        assert_eq!(e.mem.f32(this + LAST_TIME), 2.0);
        // Only the scale handle, giving -FLT_MAX: false and the time kept.
        let this = interpolator(&mut e);
        e.register(GET_VALUE_DEGREE3, |e, a| {
            e.mem.set_u32(a[2], INVALID);
            Ret::default()
        });
        for offset in [TRANSLATE_HANDLE, ROTATE_HANDLE] {
            e.mem.set_u32(this + offset, 0xffff);
        }
        e.mem.set_f32(this + END_TIME, 1.0);
        e.mem.set_f32(this + LAST_TIME, 9.0);
        assert!(!e
            .call(0x00a5_0200, &args![this, 3.0f32, 0u32, value])
            .bool());
        assert_eq!(e.mem.f32(this + LAST_TIME), 9.0);
        assert_eq!(calls(&e, GET_VALUE_DEGREE3).len(), 3);
    }

    #[test]
    fn save_binary_writes_the_transform_and_handles() {
        let mut e = engine();
        e.register(B_SPLINE_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(QUAT_TRANSFORM_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            4u32.into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x48);
        e.call(0x00a5_03e0, &args![this, stream]);
        assert_eq!(
            calls(&e, B_SPLINE_INTERPOLATOR_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_SAVE_BINARY),
            vec![vec![this + TRANSFORM, stream]]
        );
        let written: Vec<u32> = calls(&e, WRITE).iter().map(|c| c[1]).collect();
        assert_eq!(written, [this + 0x3c, this + 0x40, this + 0x44]);
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 12);
    }

    #[test]
    fn get_viewer_strings_adds_name_transform_and_handles() {
        let mut e = engine();
        e.register(B_SPLINE_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(QUAT_TRANSFORM_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7100u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_VALUE, |_, a| {
            (0x7200 + a[1]).into_ret()
        });
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            e.mem.u32(a[2]).into_ret()
        });
        e.map(NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI, 4);
        e.mem
            .set_u32(NI_B_SPLINE_TRANSFORM_INTERPOLATOR_RTTI, 0x5151);
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + TRANSLATE_HANDLE, 1);
        e.mem.set_u32(this + ROTATE_HANDLE, 2);
        e.mem.set_u32(this + SCALE_HANDLE, 3);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a5_0500, &args![this, strings]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| *a != STRINGS_SET_AT)
            .collect();
        assert_eq!(
            order,
            [
                0x00a5_0500,
                B_SPLINE_INTERPOLATOR_GET_VIEWER_STRINGS,
                NI_GET_VIEWER_STRING,
                QUAT_TRANSFORM_GET_VIEWER_STRINGS,
                NI_GET_VIEWER_STRING_VALUE,
                NI_GET_VIEWER_STRING_VALUE,
                NI_GET_VIEWER_STRING_VALUE,
            ]
        );
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_VALUE),
            vec![
                vec![LABEL_TRANSLATE_HANDLE, 1],
                vec![LABEL_ROTATE_HANDLE, 2],
                vec![LABEL_SCALE_HANDLE, 3]
            ]
        );
        assert_eq!(
            calls(&e, QUAT_TRANSFORM_GET_VIEWER_STRINGS),
            vec![vec![this + TRANSFORM, strings]]
        );
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 4);
        assert_eq!(e.mem.u16(strings + 0xa), 4);
    }
}
