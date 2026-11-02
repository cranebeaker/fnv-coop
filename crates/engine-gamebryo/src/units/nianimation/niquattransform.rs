//! `NiAnimation/niquattransform.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

layout! {
    /// `NiQuatTransform` (Xbox PDB), 0x20 bytes: the translation (three floats), the rotation
    /// (`NiQuaternion`, `m_fW m_fX m_fY m_fZ`) and the scale. An unset part holds `-FLT_MAX` in
    /// its first word that the code checks (translation `x`, rotation `m_fX`, scale). Floats
    /// are declared as their bit patterns.
    pub struct NiQuatTransform: 0x20 {
        /// `m_kTranslate` (Xbox PDB), `x` (`y` and `z` follow).
        0x00 m_kTranslate: u32,
        /// `m_kRotate` (Xbox PDB), `m_fW` (`m_fX`, `m_fY`, `m_fZ` follow).
        0x0C m_kRotate: u32,
        /// `m_kRotate.m_fX` (Xbox PDB).
        0x10 m_fRotateX: u32,
        /// `m_fScale` (Xbox PDB).
        0x1C m_fScale: u32,
    }
}

/// The float `-FLT_MAX` (`01096b8c`) an unset part is given.
const UNSET_FLOAT: u32 = 0x0109_6b8c;
/// The double `-FLT_MAX` (`010241b0`) the parts are compared with.
const UNSET_DOUBLE: u32 = 0x0102_41b0;
/// The words a result starts from: the translation (`011a8400`, three words) and the
/// rotation (`011f3704`, four words); and the nine words of the rotation matrix a result's
/// matrix local starts as (`011a9448`).
const TRANSLATE_DEFAULT: u32 = 0x011a_8400;
const ROTATE_DEFAULT: u32 = 0x011f_3704;
const MATRIX_DEFAULT: u32 = 0x011a_9448;
/// `NiQuaternion::UnitInverse` (`cdecl`, (result, quaternion); answers the result).
const QUATERNION_UNIT_INVERSE: u32 = 0x00a6_dc00;
/// `NiQuaternion::ToRotation` (`thiscall` on the quaternion, the 3x3 matrix to fill).
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
/// `NiQuaternion::operator*` (`thiscall` on the left quaternion, (result, right)).
const QUATERNION_MULTIPLY: u32 = 0x00a2_bcf0;
/// `NiQuaternion::Normalize` (`thiscall`, no stack words).
const QUATERNION_NORMALIZE: u32 = 0x00a3_8560;
/// `004b4500` (`thiscall` on a 3x3 matrix, (result, vector)): the matrix times the vector.
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
/// `NiColor::LoadBinary` and `NiColorA::LoadBinary` (`thiscall`, the stream; three and four
/// floats read in place, here the translation and the rotation).
const READ_THREE_FLOATS: u32 = 0x00a5_9be0;
const READ_FOUR_FLOATS: u32 = 0x00a6_9800;

/// The unset value as the x87 stack holds it (`FLD dword [01096b8c]`).
fn unset_float(e: &Engine) -> f64 {
    x87_load32(e.mem.u32(UNSET_FLOAT))
}

/// The compared value (`FLD qword [010241b0]`).
fn unset_double(e: &Engine) -> f64 {
    x87_load64(e.mem.u64(UNSET_DOUBLE))
}

/// Starts a result: the scale unset, then the default translation and rotation words.
fn start_result(e: &mut Engine, result: u32) {
    let unset = x87_store32(unset_float(e));
    e.mem.set_u32(result + 0x1c, unset);
    for word in 0..3 {
        let value = e.mem.u32(TRANSLATE_DEFAULT + 4 * word);
        e.mem.set_u32(result + 4 * word, value);
    }
    for word in 0..4 {
        let value = e.mem.u32(ROTATE_DEFAULT + 4 * word);
        e.mem.set_u32(result + 0xc + 4 * word, value);
    }
}

/// Copies the nine words of the default matrix to `matrix` (`REP MOVSD`).
fn copy_default_matrix(e: &mut Engine, matrix: u32) {
    for word in 0..9 {
        let value = e.mem.u32(MATRIX_DEFAULT + 4 * word);
        e.mem.set_u32(matrix + 4 * word, value);
    }
}

// Translated from 00a386a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::HierInvert` (Xbox PDB): fills `result` with the inverse of this
/// transform, part by part: the scale `1 / scale` (unset stays unset), the rotation the unit
/// inverse (`NiQuaternion::UnitInverse`; an unset rotation only sets the result's `m_fX`
/// unset), and the translation `-translation` turned by the inverse rotation's matrix
/// (`NiQuaternion::ToRotation`, the default matrix at `011a9448` when the rotation is unset)
/// and scaled by the inverse scale (1 when the scale is unset); an unset translation only
/// sets the result's `x` unset. Parts of `result` not named are left as they are.
pub fn ni_quat_transform_hier_invert(e: &mut Engine, this: Ptr<NiQuatTransform>, result: Ptr) {
    let t = this.addr();
    let r = result.addr();
    let unset = unset_double(e);
    let unset_value = unset_float(e);
    let mut inverse_scale = x87_store32(1.0);
    let scale = x87_load32(e.mem.u32(t + 0x1c));
    if unset == scale {
        e.mem.set_u32(r + 0x1c, x87_store32(unset_value));
    } else {
        inverse_scale = x87_store32(x87_div(1.0, x87_load32(e.mem.u32(t + 0x1c))));
        let value = x87_load32(inverse_scale);
        e.mem.set_u32(r + 0x1c, x87_store32(value));
    }
    e.with_stack(0x40, |e, locals| {
        let quaternion = locals.addr();
        let matrix = locals.addr() + 0x10;
        let rotate_x = x87_load32(e.mem.u32(t + 0x10));
        copy_default_matrix(e, matrix);
        if rotate_x == unset {
            e.mem.set_u32(r + 0x10, x87_store32(unset_value));
        } else {
            let inverse = e
                .call(QUATERNION_UNIT_INVERSE, &args![quaternion, t + 0xc])
                .u32();
            for word in 0..4 {
                let value = e.mem.u32(inverse + 4 * word);
                e.mem.set_u32(r + 0xc + 4 * word, value);
            }
            e.call(QUATERNION_TO_ROTATION, &args![r + 0xc, matrix]);
        }
        let translate_x = x87_load32(e.mem.u32(t));
        if translate_x == unset {
            e.mem.set_u32(r, x87_store32(unset_value));
            return;
        }
        let a = x87_load32(x87_store32(-x87_load32(e.mem.u32(t))));
        let b = x87_load32(x87_store32(-x87_load32(e.mem.u32(t + 4))));
        let c = x87_load32(x87_store32(-x87_load32(e.mem.u32(t + 8))));
        let m = |e: &Engine, i: u32| e.mem.u32(matrix + 4 * i);
        let p0 = x87_mul(x87_load32(m(e, 0)), a);
        let s0 = x87_add(p0, x87_mul(b, x87_m32(m(e, 1))));
        let p2 = x87_mul(x87_load32(m(e, 2)), c);
        let x0 = x87_store32(x87_add(p2, s0));
        let row = x87_add(
            x87_mul(x87_load32(m(e, 3)), a),
            x87_mul(x87_load32(m(e, 4)), b),
        );
        let x1 = x87_store32(x87_add(row, x87_mul(x87_load32(m(e, 5)), c)));
        let row = x87_add(
            x87_mul(a, x87_load32(m(e, 6))),
            x87_mul(b, x87_m32(m(e, 7))),
        );
        let x2 = x87_store32(x87_add(x87_mul(c, x87_load32(m(e, 8))), row));
        let ls = x87_load32(inverse_scale);
        e.mem.set_u32(r, x87_store32(x87_mul(x87_load32(x0), ls)));
        e.mem
            .set_u32(r + 4, x87_store32(x87_mul(x87_load32(x1), ls)));
        e.mem.set_u32(r + 8, x87_store32(x87_mul(ls, x87_m32(x2))));
    });
}

// Translated from 00a38830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::HierApply` (Xbox PDB; the result is returned through `result`):
/// `result` starts unset (scale `-FLT_MAX`, the default translation and rotation words);
/// with both scales set its scale is their product; with both rotations set its rotation is
/// `this * other` normalized (`NiQuaternion::operator*`, `Normalize`) and this rotation's
/// matrix (`ToRotation`) is used, the default matrix (`011a9448`) otherwise; an unset rotation
/// sets the result's `m_fX` unset. With both translations set the result's translation is
/// this translation plus the matrix times the other translation (`004b4500`), scaled by the
/// scale product (1 when either scale is unset); otherwise its `x` is unset. Answers `result`.
pub fn ni_quat_transform_hier_apply(
    e: &mut Engine,
    this: Ptr<NiQuatTransform>,
    result: Ptr,
    other: Ptr<NiQuatTransform>,
) -> Ptr {
    let (t, r, o) = (this.addr(), result.addr(), other.addr());
    let unset = unset_double(e);
    start_result(e, r);
    let mut factor = x87_store32(1.0);
    let unset_value = unset_float(e);
    let own_scale = x87_load32(e.mem.u32(t + 0x1c));
    if !(unset == own_scale) && !(x87_load32(e.mem.u32(o + 0x1c)) == unset) {
        factor = x87_store32(x87_mul(
            x87_load32(e.mem.u32(o + 0x1c)),
            x87_m32(e.mem.u32(t + 0x1c)),
        ));
        e.mem.set_u32(r + 0x1c, x87_store32(x87_load32(factor)));
    } else {
        e.mem.set_u32(r + 0x1c, x87_store32(unset_value));
    }
    e.with_stack(0x40, |e, locals| {
        let quaternion = locals.addr();
        let matrix = locals.addr() + 0x10;
        let own_rotate = x87_load32(e.mem.u32(t + 0x10));
        copy_default_matrix(e, matrix);
        if !(own_rotate == unset) && !(x87_load32(e.mem.u32(o + 0x10)) == unset) {
            e.call(QUATERNION_MULTIPLY, &args![t + 0xc, quaternion, o + 0xc]);
            e.call(QUATERNION_NORMALIZE, &args![quaternion]);
            for word in 0..4 {
                let value = e.mem.u32(quaternion + 4 * word);
                e.mem.set_u32(r + 0xc + 4 * word, value);
            }
            e.call(QUATERNION_TO_ROTATION, &args![t + 0xc, matrix]);
        } else {
            e.mem.set_u32(r + 0x10, x87_store32(unset_value));
        }
        let own_translate = x87_load32(e.mem.u32(t));
        if own_translate == unset || x87_load32(e.mem.u32(o)) == unset {
            e.mem.set_u32(r, x87_store32(unset_value));
            return;
        }
        let turned = e
            .call(MATRIX_TIMES_VECTOR, &args![matrix, quaternion, o])
            .u32();
        let f = x87_load32(factor);
        let w0 = x87_store32(x87_mul(f, x87_m32(e.mem.u32(turned))));
        let w1 = x87_store32(x87_mul(x87_load32(e.mem.u32(turned + 4)), f));
        let w2 = x87_store32(x87_mul(f, x87_m32(e.mem.u32(turned + 8))));
        for (word, w) in [w0, w1, w2].into_iter().enumerate() {
            let at = 4 * word as u32;
            let sum = x87_add(x87_load32(e.mem.u32(t + at)), x87_m32(w));
            e.mem.set_u32(r + at, x87_store32(sum));
        }
    });
    result
}

// Translated from 00a38a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `niquattransform.cpp`; by its body the transform
/// composition without turning the translation): `result` starts unset as in `HierApply`;
/// with both scales set its scale is their product; with both rotations set its rotation is
/// `this * other` normalized, otherwise its `m_fX` is unset; with both translations set its
/// translation is their sum, otherwise its `x` is unset. Answers `result`.
pub fn fn_00a38a00(
    e: &mut Engine,
    this: Ptr<NiQuatTransform>,
    result: Ptr,
    other: Ptr<NiQuatTransform>,
) -> Ptr {
    let (t, r, o) = (this.addr(), result.addr(), other.addr());
    let unset_value = unset_float(e);
    start_result(e, r);
    let unset = unset_double(e);
    if !(unset == x87_load32(e.mem.u32(t + 0x1c))) && !(x87_load32(e.mem.u32(o + 0x1c)) == unset) {
        let product = x87_mul(
            x87_load32(e.mem.u32(o + 0x1c)),
            x87_m32(e.mem.u32(t + 0x1c)),
        );
        e.mem.set_u32(r + 0x1c, x87_store32(product));
    } else {
        e.mem.set_u32(r + 0x1c, x87_store32(unset_value));
    }
    if !(x87_load32(e.mem.u32(t + 0x10)) == unset) && !(x87_load32(e.mem.u32(o + 0x10)) == unset) {
        e.with_stack(0x10, |e, quaternion| {
            let q = quaternion.addr();
            e.call(QUATERNION_MULTIPLY, &args![t + 0xc, q, o + 0xc]);
            e.call(QUATERNION_NORMALIZE, &args![q]);
            for word in 0..4 {
                let value = e.mem.u32(q + 4 * word);
                e.mem.set_u32(r + 0xc + 4 * word, value);
            }
        });
    } else {
        e.mem.set_u32(r + 0x10, x87_store32(unset_value));
    }
    if x87_load32(e.mem.u32(t)) == unset || x87_load32(e.mem.u32(o)) == unset {
        e.mem.set_u32(r, x87_store32(unset_value));
        return result;
    }
    for word in 0..3 {
        let at = 4 * word;
        let sum = x87_add(x87_load32(e.mem.u32(o + at)), x87_m32(e.mem.u32(t + at)));
        e.mem.set_u32(r + at, x87_store32(sum));
    }
    result
}

/// Reads one byte from the stream's input into `destination`, the byte-count word at `count`
/// set to 1 first.
fn read_byte(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 1);
    let done = e
        .call(function, &args![input, destination, 1u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a38bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::LoadBinary` (Xbox PDB): the translation (three floats, `00a59be0`), the
/// rotation (four floats, `00a69800`) and the 4-byte scale. Files older than version
/// `0x0a01006e` (`NiStream` +0xd8) follow with three bytes, whether the translation, the
/// rotation and the scale are set; a 0 makes that part unset (`-FLT_MAX`, `01096b8c`, in its
/// first checked word).
pub fn ni_quat_transform_load_binary(
    e: &mut Engine,
    this: Ptr<NiQuatTransform>,
    stream: Ptr<NiStream>,
) {
    let t = this.addr();
    e.call(READ_THREE_FLOATS, &args![this, stream]);
    e.call(READ_FOUR_FLOATS, &args![t + 0xc, stream]);
    // Stack words: the stream argument's slot and one local.
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let local = slots.addr() + 4;
        let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
        let function = e.get(input, NiBinaryStream::m_pfnRead);
        e.mem.set_u32(argument, 4);
        let done = e
            .call(function, &args![input, t + 0x1c, 4u32, argument, 1u32])
            .u32();
        let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            input,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
        if e.get(stream, NiStream::m_uiNifFileVersion) >= 0x0a01_006e {
            return;
        }
        for target in [t, t + 0x10, t + 0x1c] {
            read_byte(e, stream, argument, local);
            if e.mem.u8(argument) == 0 {
                let unset = x87_store32(unset_float(e));
                e.mem.set_u32(target, unset);
            }
        }
    });
}

/// `NiColor::SaveBinary` and `NiQuaternion::SaveBinary` (engine map; `thiscall`, the stream):
/// three and four floats written in place, here the translation and the rotation.
const WRITE_THREE_FLOATS: u32 = 0x00a6_9780;
const WRITE_FOUR_FLOATS: u32 = 0x00a6_e4a0;
/// `NiGetViewerString` (`cdecl`, the text), its float overload (`00a7d550`, `cdecl`, (name,
/// float)), and the point's and the quaternion's `GetViewerString` (`00a59af0`, `00a6e190`;
/// `thiscall` on the value, the name): each answers a new string.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const QUATERNION_GET_VIEWER_STRING: u32 = 0x00a6_e190;
/// The viewer-string array's `SetSize` (`0096ad30`, `thiscall`, the new size) and `SetAt`
/// (`0096ae90`, `thiscall`, (index, address of the string)).
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"NiQuatTransform"`, `"m_kTranslate"`, `"m_kRotate"` and `"m_fScale"`.
const CLASS_NAME_TEXT: u32 = 0x0109_6bbc;
const TRANSLATE_NAME_TEXT: u32 = 0x0109_6bac;
const ROTATE_NAME_TEXT: u32 = 0x0109_6ba0;
const SCALE_NAME_TEXT: u32 = 0x0109_6b94;

/// The inlined `Add` of the viewer-string array (`NiTArray`: capacity +0x8, size +0xA, step
/// +0xE): `text` goes into the stack word at `slot`; when the size has reached the capacity
/// the array is grown to size + step first; then the word's address is stored at the old
/// size (`SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr, slot: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    e.mem.set_u32(slot, text);
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a38b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::SaveBinary` (Xbox PDB): the translation (`00a69780`) and the rotation
/// (`00a6e4a0`), then the 4-byte scale through the output stream (`NiStream::m_pkOstr`
/// +0x250): its write function (`m_pfnWrite`, `(stream, source, size, &component size,
/// count)`) with the component size word (the stream argument's own stack slot) set to 4,
/// and the byte count it answers added to its position.
pub fn ni_quat_transform_save_binary(
    e: &mut Engine,
    this: Ptr<NiQuatTransform>,
    stream: Ptr<NiStream>,
) {
    let t = this.addr();
    e.call(WRITE_THREE_FLOATS, &args![this, stream]);
    e.call(WRITE_FOUR_FLOATS, &args![t + 0xc, stream]);
    e.with_stack(4, |e, slot| {
        let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
        let function = e.get(output, NiBinaryStream::m_pfnWrite);
        e.mem.set_u32(slot.addr(), 4);
        let done = e
            .call(function, &args![output, t + 0x1c, 4u32, slot, 1u32])
            .u32();
        let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            output,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(done),
        );
    });
}

// Translated from 00a38cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuatTransform::GetViewerStrings` (Xbox PDB): appends `"NiQuatTransform"` to the
/// viewer strings, then the translation's, the rotation's and the scale's strings for the
/// parts that are set (the checked word unequal to the double `-FLT_MAX` at `010241b0`; a
/// NaN is unequal, so it is listed).
pub fn ni_quat_transform_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiQuatTransform>,
    strings: Ptr,
) {
    let t = this.addr();
    // The two stack words that hold the strings: the local the function reserves, then the
    // strings argument's own slot.
    e.with_stack(8, |e, slots| {
        let local = slots.addr();
        let argument = local + 4;
        let text = e.call(NI_GET_VIEWER_STRING, &args![CLASS_NAME_TEXT]).u32();
        add_viewer_string(e, strings, local, text);
        if !(x87_load32(e.mem.u32(t)) == unset_double(e)) {
            let text = e
                .call(POINT3_GET_VIEWER_STRING, &args![t, TRANSLATE_NAME_TEXT])
                .u32();
            add_viewer_string(e, strings, argument, text);
        }
        if !(x87_load32(e.mem.u32(t + 0x10)) == unset_double(e)) {
            let text = e
                .call(
                    QUATERNION_GET_VIEWER_STRING,
                    &args![t + 0xc, ROTATE_NAME_TEXT],
                )
                .u32();
            add_viewer_string(e, strings, argument, text);
        }
        if !(x87_load32(e.mem.u32(t + 0x1c)) == unset_double(e)) {
            let scale = x87_quiet_bits(e.mem.u32(t + 0x1c));
            let text = e
                .call(NI_GET_VIEWER_STRING_FLOAT, &args![SCALE_NAME_TEXT, scale])
                .u32();
            add_viewer_string(e, strings, argument, text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a3_8b60,
            ni_quat_transform_save_binary(Ptr<NiQuatTransform>, Ptr<NiStream>)
        ),
        entry!(
            0x00a3_8cb0,
            ni_quat_transform_get_viewer_strings(Ptr<NiQuatTransform>, Ptr)
        ),
        entry!(
            0x00a3_86a0,
            ni_quat_transform_hier_invert(Ptr<NiQuatTransform>, Ptr)
        ),
        entry!(
            0x00a3_8830,
            ni_quat_transform_hier_apply(Ptr<NiQuatTransform>, Ptr, Ptr<NiQuatTransform>) -> Ptr
        ),
        entry!(
            0x00a3_8a00,
            fn_00a38a00(Ptr<NiQuatTransform>, Ptr, Ptr<NiQuatTransform>) -> Ptr
        ),
        entry!(
            0x00a3_8bb0,
            ni_quat_transform_load_binary(Ptr<NiQuatTransform>, Ptr<NiStream>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNSET: u32 = 0xff7f_ffff;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the constants: the default translation (7, 8, 9), rotation
    /// (1, 0, 0, 0) and matrix (identity).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_6000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.map(0x011a_8000, 0x2000);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(UNSET_FLOAT, UNSET);
        e.mem.set_u64(UNSET_DOUBLE, f64::from(f32::MIN).to_bits());
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(TRANSLATE_DEFAULT + 4 * i as u32, *v);
        }
        e.mem.set_f32(ROTATE_DEFAULT, 1.0);
        for i in 0..3 {
            e.mem.set_f32(MATRIX_DEFAULT + 16 * i, 1.0);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn transform(e: &mut Engine, values: [f32; 8]) -> u32 {
        let t = e.mem.alloc(0x20);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(t + 4 * i as u32, *v);
        }
        t
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn hier_invert_inverts_scale_rotation_and_translation() {
        let mut e = engine();
        // The inverse rotation's matrix: a quarter turn (x -> y).
        e.register(QUATERNION_UNIT_INVERSE, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            a[0].into_ret()
        });
        e.register(QUATERNION_TO_ROTATION, |e, a| {
            let m = [0.0f32, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
            for (i, v) in m.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        let this = transform(&mut e, [1.0, 2.0, 3.0, 0.5, 0.5, 0.5, 0.5, 2.0]);
        let result = e.mem.alloc(0x20);
        e.call(0x00a3_86a0, &args![this, result]);
        assert_eq!(e.mem.f32(result + 0x1c), 0.5);
        assert_eq!(e.mem.u32(result + 0xc), 0x100);
        assert_eq!(e.mem.u32(result + 0x18), 0x103);
        assert_eq!(
            calls_to(&e, QUATERNION_TO_ROTATION),
            vec![vec![
                result + 0xc,
                calls_to(&e, QUATERNION_TO_ROTATION)[0][1]
            ]]
        );
        // -(1, 2, 3) turned: (2, -1, -3), halved.
        assert_eq!(floats(&e, result, 3), vec![1.0, -0.5, -1.5]);
        // Everything unset: only the checked words become unset.
        let unset = transform(&mut e, [0.0; 8]);
        for at in [0u32, 0x10, 0x1c] {
            e.mem.set_u32(unset + at, UNSET);
        }
        let result = e.mem.alloc(0x20);
        e.mem.set_u32(result + 4, 0x55);
        e.call(0x00a3_86a0, &args![unset, result]);
        assert_eq!(e.mem.u32(result), UNSET);
        assert_eq!(e.mem.u32(result + 0x10), UNSET);
        assert_eq!(e.mem.u32(result + 0x1c), UNSET);
        assert_eq!(e.mem.u32(result + 4), 0x55);
        assert_eq!(calls_to(&e, QUATERNION_UNIT_INVERSE).len(), 1);
        // Unset scale and rotation, set translation: identity matrix, factor 1.
        e.mem.set_f32(unset, 4.0);
        e.mem.set_f32(unset + 4, -5.0);
        e.call(0x00a3_86a0, &args![unset, result]);
        assert_eq!(floats(&e, result, 3), vec![-4.0, 5.0, -0.0]);
    }

    #[test]
    fn hier_apply_combines_set_parts() {
        let mut e = engine();
        e.register(QUATERNION_MULTIPLY, |e, a| {
            e.mem.set_u32(a[1], 0x200);
            Ret::default()
        });
        e.register(QUATERNION_NORMALIZE, |e, a| {
            e.mem.set_u32(a[0] + 4, 0x201);
            Ret::default()
        });
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        e.register(MATRIX_TIMES_VECTOR, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) * 10.0;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        let this = transform(&mut e, [1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2.0]);
        let other = transform(&mut e, [1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 3.0]);
        let result = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00a3_8830, &args![this, result, other]).u32(),
            result
        );
        assert_eq!(e.mem.f32(result + 0x1c), 6.0);
        assert_eq!(e.mem.u32(result + 0xc), 0x200);
        assert_eq!(e.mem.u32(result + 0x10), 0x201);
        assert_eq!(calls_to(&e, QUATERNION_MULTIPLY)[0][0], this + 0xc);
        assert_eq!(calls_to(&e, QUATERNION_TO_ROTATION)[0][0], this + 0xc);
        assert_eq!(floats(&e, result, 3), vec![61.0, 62.0, 63.0]);
        // The other's scale and rotation unset: factor 1, the default matrix words copied in
        // (the turn is still the matrix call), the result's scale and m_fX unset.
        e.mem.set_u32(other + 0x1c, UNSET);
        e.mem.set_u32(other + 0x10, UNSET);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8830, &args![this, result, other]);
        assert_eq!(e.mem.u32(result + 0x1c), UNSET);
        assert_eq!(e.mem.u32(result + 0x10), UNSET);
        assert_eq!(e.mem.f32(result + 0xc), 1.0);
        assert!(calls_to(&e, QUATERNION_MULTIPLY).is_empty());
        assert_eq!(floats(&e, result, 3), vec![11.0, 12.0, 13.0]);
        // The other's translation unset: x unset, y and z the defaults.
        e.mem.set_u32(other, UNSET);
        e.call(0x00a3_8830, &args![this, result, other]);
        assert_eq!(e.mem.u32(result), UNSET);
        assert_eq!(floats(&e, result + 4, 2), vec![8.0, 9.0]);
    }

    #[test]
    fn composition_00a38a00_adds_translations() {
        let mut e = engine();
        e.register(QUATERNION_MULTIPLY, |e, a| {
            e.mem.set_u32(a[1] + 8, 0x300);
            Ret::default()
        });
        e.register(QUATERNION_NORMALIZE, |_, _| Ret::default());
        let this = transform(&mut e, [1.0, 2.0, 3.0, 1.0, 0.0, 0.0, 0.0, 2.0]);
        let other = transform(&mut e, [0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.25]);
        let result = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00a3_8a00, &args![this, result, other]).u32(),
            result
        );
        assert_eq!(e.mem.f32(result + 0x1c), 0.5);
        assert_eq!(e.mem.u32(result + 0x14), 0x300);
        assert_eq!(calls_to(&e, QUATERNION_MULTIPLY)[0][..1], [this + 0xc]);
        assert_eq!(calls_to(&e, QUATERNION_MULTIPLY)[0][2], other + 0xc);
        assert_eq!(floats(&e, result, 3), vec![1.5, 2.5, 3.5]);
        e.mem.set_u32(this, UNSET);
        e.mem.set_u32(this + 0x1c, UNSET);
        e.mem.set_u32(this + 0x10, UNSET);
        e.call(0x00a3_8a00, &args![this, result, other]);
        assert_eq!(e.mem.u32(result), UNSET);
        assert_eq!(e.mem.u32(result + 0x10), UNSET);
        assert_eq!(e.mem.u32(result + 0x1c), UNSET);
        assert_eq!(floats(&e, result + 4, 2), vec![8.0, 9.0]);
    }

    #[test]
    fn load_binary_reads_the_parts_and_old_set_flags() {
        let mut e = engine();
        e.register(READ_THREE_FLOATS, |_, _| Ret::default());
        e.register(READ_FOUR_FLOATS, |_, _| Ret::default());
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        let mut answers =
            std::collections::VecDeque::from(vec![2.0f32.to_bits(), 1, 0, 0, 3.0f32.to_bits()]);
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let v: u32 = answers.pop_front().unwrap();
            e.mem.write(a[1], &v.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        let this = transform(&mut e, [5.0; 8]);
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_006d);
        e.call(0x00a3_8bb0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, READ_THREE_FLOATS),
            vec![vec![this, stream.addr()]]
        );
        assert_eq!(
            calls_to(&e, READ_FOUR_FLOATS),
            vec![vec![this + 0xc, stream.addr()]]
        );
        assert_eq!(e.mem.f32(this), 5.0);
        assert_eq!(e.mem.u32(this + 0x10), UNSET);
        assert_eq!(e.mem.u32(this + 0x1c), UNSET);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 7);
        // A current file: the scale only.
        e.set(stream, NiStream::m_uiNifFileVersion, 0x0a01_006e);
        e.call(0x00a3_8bb0, &args![this, stream]);
        assert_eq!(e.mem.f32(this + 0x1c), 3.0);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 11);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    const UNSET: u32 = 0xff7f_ffff;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn save_binary_writes_translation_rotation_and_scale() {
        let mut e = Engine::new();
        e.register(WRITE_THREE_FLOATS, |_, _| Ret::default());
        e.register(WRITE_FOUR_FLOATS, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            assert_eq!(a[4], 1);
            a[2].into_ret()
        });
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, 0x0300_0000);
        e.set(output, NiBinaryStream::m_uiAbsoluteCurrentPos, 10);
        let this = Ptr::<NiQuatTransform>::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        e.call(0x00a3_8b60, &args![this, stream]);
        assert_eq!(
            calls_to(&e, WRITE_THREE_FLOATS),
            vec![vec![this.addr(), stream.addr()]]
        );
        assert_eq!(
            calls_to(&e, WRITE_FOUR_FLOATS),
            vec![vec![this.addr() + 0xc, stream.addr()]]
        );
        let write = calls_to(&e, 0x0300_0000);
        assert_eq!(write.len(), 1);
        assert_eq!(write[0][..3], [output.addr(), this.addr() + 0x1c, 4]);
        assert_eq!(e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos), 14);
    }

    #[test]
    fn get_viewer_strings_lists_the_set_parts() {
        let mut e = Engine::new();
        e.map(0x0102_4000, 0x1000);
        e.mem.set_u64(UNSET_DOUBLE, f64::from(f32::MIN).to_bits());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(POINT3_GET_VIEWER_STRING, |_, _| 0x7001u32.into_ret());
        e.register(QUATERNION_GET_VIEWER_STRING, |_, _| 0x7002u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| 0x7003u32.into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        // SetAt stores the string the slot holds and grows the size.
        e.register(STRINGS_SET_AT, |e, a| {
            let text = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 3);
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this, 1.0);
        e.mem.set_u32(this + 0x10, UNSET);
        // A NaN scale is unequal to the unset value: listed.
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        e.call_log = Some(vec![]);
        e.call(0x00a3_8cb0, &args![this, strings]);
        let listed: Vec<u32> = (0..3).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(listed, [0x7000, 0x7001, 0x7003]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING),
            vec![vec![CLASS_NAME_TEXT]]
        );
        assert_eq!(
            calls_to(&e, POINT3_GET_VIEWER_STRING),
            vec![vec![this, TRANSLATE_NAME_TEXT]]
        );
        assert!(calls_to(&e, QUATERNION_GET_VIEWER_STRING).is_empty());
        // The scale goes through the x87 stack: the signalling NaN comes out quiet.
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_FLOAT),
            vec![vec![SCALE_NAME_TEXT, 0x7fc0_0001]]
        );
        // The array was full at the third string: grown to size + step.
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 5]]);
        // All unset: only the class name.
        let unset = e.mem.alloc(0x20);
        for at in [0u32, 0x10, 0x1c] {
            e.mem.set_u32(unset + at, UNSET);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a3_8cb0, &args![unset, strings]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 1);
    }
}
