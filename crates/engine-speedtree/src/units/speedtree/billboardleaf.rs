//! `SpeedTree/billboardleaf.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CBillboardLeaf` is a `CIdvCamera` (a vtable and a position) with a
//! texture angle, a packed colour and the leaf's frame. The colour is
//! stored as `0xAABBGGRR` with the alpha always 0xff.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The uniform form takes it as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)` (`cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The exception constructor `CBillboardLeaf::Parse` throws with
/// (`thiscall`: the string and a `bool`; unnamed in the engine map,
/// `SpeedTree/speedtreert.cpp`).
const PARSE_ERROR_FROM_STRING: u32 = 0x00b0_9480;
/// The CRT's invalid-parameter handler (`cdecl`, no arguments) that the
/// vector index check calls.
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// `cos(double)` (LIBCMT, `cdecl`).
const COS: u32 = 0x00ec_9f30;
/// `CIdvCamera::CIdvCamera` (`thiscall`): the camera vtable and a zero
/// position (unnamed in the engine map).
const CAMERA_CONSTRUCTOR: u32 = 0x00b3_f030;
/// `CIdvCamera`'s destructor body (`thiscall`): puts back the camera
/// vtable (unnamed in the engine map).
const CAMERA_DESTRUCTOR: u32 = 0x00b3_f0a0;
/// `CIdvCamera::operator=` (`thiscall`, `00b3f0c0`).
const CAMERA_ASSIGN: u32 = 0x00b3_f0c0;
/// `CVec3::operator-` (`thiscall`: `this`, the result, the other vector;
/// the result is `this - other`); unnamed in the engine map.
const VEC3_SUBTRACT: u32 = 0x00b1_3040;
/// `CVec3::Normalize` (Xbox PDB).
const VEC3_NORMALIZE: u32 = 0x00b1_2790;
/// `CVec3::AngleBetween` (Xbox PDB): `this`, the other vector; the angle
/// is a `float` in ST0.
const VEC3_ANGLE_BETWEEN: u32 = 0x00b1_2720;
/// `CVec3::operator*(float)` (`thiscall`: `this`, the result, the factor;
/// the result is `this * factor`); unnamed in the engine map.
const VEC3_SCALE_COPY: u32 = 0x00b0_cff0;
/// `CVec3::operator*=(float)` (`thiscall`: `this`, the factor); unnamed in
/// the engine map.
const VEC3_SCALE_IN_PLACE: u32 = 0x00b2_8440;
/// `CTreeFileAccess::ParseInt` (Xbox PDB, `thiscall`).
const PARSE_INT: u32 = 0x00b3_ed80;
/// The file-access function that reads a 32-bit word (`thiscall`; unnamed
/// in the engine map, `SpeedTree/fileaccess.cpp`).
const PARSE_WORD: u32 = 0x00b3_ede0;
/// `CTreeFileAccess::ParseFloat` (Xbox PDB, `thiscall`; `float` in ST0).
const PARSE_FLOAT: u32 = 0x00b3_ed20;
/// `CTreeFileAccess::ParseVector3` (Xbox PDB, `thiscall`: `this`, the
/// result buffer; returns it).
const PARSE_VECTOR3: u32 = 0x00b3_ef80;

/// `255.0` (`double`).
const BYTE_RANGE: u32 = 0x0101_e568;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// The `CBillboardLeaf` vtable.
const BILLBOARD_LEAF_VTABLE: u32 = 0x010a_58fc;
/// `"malformed billboard leaf"`.
const MESSAGE_MALFORMED: u32 = 0x010a_5904;
/// The throw information of the malformed-leaf exception.
const MALFORMED_THROW_INFO: u32 = 0x0117_0384;

layout! {
    /// `CVec3` (Xbox PDB), three `float`s.
    pub struct CVec3: 0xc {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
    }

    /// `CBillboardLeaf` (Xbox PDB), 0x4c bytes; a `CIdvCamera` base (vtable,
    /// `m_cPosition`) and the leaf's own fields.
    pub struct CBillboardLeaf: 0x4c {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_cPosition` (Xbox PDB, `CIdvCamera`): `CVec3`.
        0x04 m_cPosition: Inline<CVec3>,
        /// `m_ucAngleIndex` (Xbox PDB).
        0x10 m_ucAngleIndex: u8,
        /// `m_lColor` (Xbox PDB): `0xAABBGGRR`.
        0x14 m_lColor: u32,
        /// `m_ucColorScale` (Xbox PDB).
        0x18 m_ucColorScale: u8,
        /// `m_cNormal` (Xbox PDB): `CVec3`.
        0x1C m_cNormal: Inline<CVec3>,
        /// `m_cTangent` (Xbox PDB): `CVec3`.
        0x28 m_cTangent: Inline<CVec3>,
        /// `m_cBinormal` (Xbox PDB): `CVec3`.
        0x34 m_cBinormal: Inline<CVec3>,
        /// `m_ucTextureIndex` (Xbox PDB).
        0x40 m_ucTextureIndex: u8,
        /// `m_fWindWeight` (Xbox PDB).
        0x44 m_fWindWeight: f32,
        /// `m_nWindGroup` (Xbox PDB).
        0x48 m_nWindGroup: i32,
    }

    /// `CTreeFileAccess` (Xbox PDB `m_nIndex`, `m_vMemory`) as the PC code
    /// reads it: the read position and the byte vector's first and end
    /// pointers (the PC vector carries two more words than the Xbox one in
    /// front of them).
    pub struct CTreeFileAccess: 0x18 {
        /// `m_nIndex` (Xbox PDB): the next byte to read.
        0x00 m_nIndex: u32,
        /// `m_vMemory._Myfirst`: `unsigned char*`.
        0x10 m_vMemoryFirst: Ptr,
        /// `m_vMemory._Mylast`: `unsigned char*`.
        0x14 m_vMemoryLast: Ptr,
    }
}

/// Copies the three words of a `CVec3` the way the code moves them.
fn copy_vec3(e: &mut Engine, from: u32, to: u32) {
    for i in 0..3 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

// Translated from 00b3f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::scalar deleting destructor` (Xbox PDB): the camera
/// destructor body (the leaf adds nothing to destroy), then `operator
/// delete` when bit 0 of `flags` is set. Returns `this`.
pub fn cbillboard_leaf_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<CBillboardLeaf>,
    flags: u32,
) -> Ptr<CBillboardLeaf> {
    e.call(CAMERA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00b40150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::CBillboardLeaf` (identified from the vtable it stores
/// and the Xbox PDB layout; the engine map has no name): the camera
/// constructor, the leaf vtable and every field cleared, the angle index 0,
/// the colour -1 (opaque white), the wind group 0. Returns `this`.
pub fn fn_00b40150(e: &mut Engine, this: Ptr<CBillboardLeaf>) -> Ptr<CBillboardLeaf> {
    e.call(CAMERA_CONSTRUCTOR, &args![this]);
    e.set(this, CBillboardLeaf::vtable, BILLBOARD_LEAF_VTABLE);
    e.set(this, CBillboardLeaf::m_ucAngleIndex, 0);
    e.set(this, CBillboardLeaf::m_lColor, 0xffff_ffff);
    e.set(this, CBillboardLeaf::m_ucColorScale, 0);
    for field in [
        CBillboardLeaf::m_cNormal,
        CBillboardLeaf::m_cTangent,
        CBillboardLeaf::m_cBinormal,
    ] {
        let vector = this.at(field);
        e.set(vector, CVec3::z, 0.0);
        e.set(vector, CVec3::y, 0.0);
        e.set(vector, CVec3::x, 0.0);
    }
    e.set(this, CBillboardLeaf::m_ucTextureIndex, 0);
    e.set(this, CBillboardLeaf::m_fWindWeight, 0.0);
    e.set(this, CBillboardLeaf::m_nWindGroup, 0);
    this
}

// Translated from 00b40210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::AdjustStaticLighting` (Xbox PDB): darkens or lightens
/// the leaf's colour by the angle between two directions. Both are
/// `vector - vector_a` (the leaf's position minus `vector_a`, and `vector_b`
/// minus `vector_a`), normalised; the factor is
/// `((cos(angle) + 1) * 0.5) * (1 - weight) + weight`, and the colour
/// (`GetColor`) is multiplied by it and stored again (`SetColor` with the
/// colour-scale flag set).
pub fn cbillboard_leaf_adjust_static_lighting(
    e: &mut Engine,
    this: Ptr<CBillboardLeaf>,
    vector_a: Ptr,
    vector_b: Ptr,
    weight: f32,
) {
    // The exe's frame (`SUB ESP,0x80` under `EBP`): 0x80 bytes, the two directions at +0x68
    // and +0x74 (`EBP-0x18`, `EBP-0xc`), the colour at +0x54 and its scaled copy at +0x48
    // (`EBP-0x2c`, `EBP-0x38`).
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let (first, second, color, scaled) =
            (frame + 0x68, frame + 0x74, frame + 0x54, frame + 0x48);
        e.call(VEC3_SUBTRACT, &args![this.byte_add(4), first, vector_a]);
        e.call(VEC3_NORMALIZE, &args![first]);
        e.call(VEC3_SUBTRACT, &args![vector_b, second, vector_a]);
        e.call(VEC3_NORMALIZE, &args![second]);
        let angle = e.call(VEC3_ANGLE_BETWEEN, &args![first, second]).f32();
        let cosine = e.call(COS, &args![angle as f64]).f64() as f32;
        let one: f64 = e.global(ONE);
        let half: f64 = e.global(HALF);
        let shaped = ((cosine as f64 + one) * half) as f32;
        let factor = (shaped as f64 * (1.0 - weight as f64) + weight as f64) as f32;
        let color_ptr = cbillboard_leaf_get_color(e, this, Ptr::new(color));
        let scaled_ptr = e
            .call(
                VEC3_SCALE_COPY,
                &args![color_ptr, scaled, x87_quiet(factor)],
            )
            .ptr::<()>();
        cbillboard_leaf_set_color(e, this, scaled_ptr, 1);
    });
}

// Translated from 00b402d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::operator=` (identified from the vtable and the Xbox PDB
/// layout; the engine map has no name): unless `other` is `this`, assigns
/// the camera part (`00b3f0c0`) and copies the angle index, colour, colour
/// scale, normal, texture index, wind weight (through the x87, which quiets
/// a signalling NaN), wind group, tangent and binormal. Returns `this`.
pub fn fn_00b402d0(
    e: &mut Engine,
    this: Ptr<CBillboardLeaf>,
    other: Ptr<CBillboardLeaf>,
) -> Ptr<CBillboardLeaf> {
    if other != this {
        e.call(CAMERA_ASSIGN, &args![this, other]);
        let angle = e.get(other, CBillboardLeaf::m_ucAngleIndex);
        e.set(this, CBillboardLeaf::m_ucAngleIndex, angle);
        let color = e.get(other, CBillboardLeaf::m_lColor);
        e.set(this, CBillboardLeaf::m_lColor, color);
        let scale = e.get(other, CBillboardLeaf::m_ucColorScale);
        e.set(this, CBillboardLeaf::m_ucColorScale, scale);
        copy_vec3(e, other.addr() + 0x1c, this.addr() + 0x1c);
        let texture = e.get(other, CBillboardLeaf::m_ucTextureIndex);
        e.set(this, CBillboardLeaf::m_ucTextureIndex, texture);
        let wind_weight = x87_quiet(e.get(other, CBillboardLeaf::m_fWindWeight));
        e.set(this, CBillboardLeaf::m_fWindWeight, wind_weight);
        let wind_group = e.get(other, CBillboardLeaf::m_nWindGroup);
        e.set(this, CBillboardLeaf::m_nWindGroup, wind_group);
        copy_vec3(e, other.addr() + 0x28, this.addr() + 0x28);
        copy_vec3(e, other.addr() + 0x34, this.addr() + 0x34);
    }
    this
}

// Translated from 00b403a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b403a0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// A copy of the leaf at `this` (unnamed in the engine map): `operator new`
/// of 0x4c bytes, the constructor (`00b40150`) when the allocation worked,
/// then `operator=` from `this` into the new object, which is returned. A
/// failed allocation still runs the assignment on the null pointer, as the
/// exe does.
pub fn fn_00b403a0(e: &mut Engine, this: Ptr<CBillboardLeaf>) -> Ptr<CBillboardLeaf> {
    let memory = e
        .call(OPERATOR_NEW, &args![0x4cu32])
        .ptr::<CBillboardLeaf>();
    let copy = if memory.is_null() {
        Ptr::NULL
    } else {
        fn_00b40150(e, memory)
    };
    fn_00b402d0(e, copy, this);
    copy
}

// Translated from 00b40430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::GetColor` (Xbox PDB): writes the colour's red, green and
/// blue bytes as floats in 0..1 (each divided by 255) to `result` and
/// returns it.
pub fn cbillboard_leaf_get_color(e: &mut Engine, this: Ptr<CBillboardLeaf>, result: Ptr) -> Ptr {
    let color = e.get(this, CBillboardLeaf::m_lColor);
    let range: f64 = e.global(BYTE_RANGE);
    let red = ((color & 0xff) as f64 / range) as f32;
    let green = (((color & 0xff00) >> 8) as f64 / range) as f32;
    let blue = (((color & 0xff_0000) >> 16) as f64 / range) as f32;
    e.mem.set_f32(result.addr(), red);
    e.mem.set_f32(result.addr() + 4, green);
    e.mem.set_f32(result.addr() + 8, blue);
    result
}

// Translated from 00b404c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CBillboardLeaf::SetColor` (Xbox PDB): stores the colour at `color` (red,
/// green, blue as floats, 0..1) as `0xff` alpha and three bytes. With
/// `scaled` set the colour is first multiplied by the leaf's colour scale
/// over 255; each component is clamped to 0..1 (a NaN stays a NaN) and
/// truncated after the multiplication by 255.
pub fn cbillboard_leaf_set_color(
    e: &mut Engine,
    this: Ptr<CBillboardLeaf>,
    color: Ptr,
    scaled: u8,
) {
    // The exe's frame (`SUB ESP,0x1c` under `EBP`, one push): 0x20 bytes, the colour copy at
    // +0x14 (`EBP-0xc`).
    e.with_stack(0x20, |e, frame| {
        set_color_in_frame(e, this, color, scaled, frame.addr() + 0x14)
    });
}

/// [`cbillboard_leaf_set_color`] with its colour copy at `local`.
fn set_color_in_frame(
    e: &mut Engine,
    this: Ptr<CBillboardLeaf>,
    color: Ptr,
    scaled: u8,
    local: u32,
) {
    copy_vec3(e, color.addr(), local);
    let range: f64 = e.global(BYTE_RANGE);
    if scaled != 0 {
        let scale = e.get(this, CBillboardLeaf::m_ucColorScale);
        let factor = (scale as f64 / range) as f32;
        e.call(VEC3_SCALE_IN_PLACE, &args![local, x87_quiet(factor)]);
    }
    let one: f64 = e.global(ONE);
    let zero: f64 = e.global(ZERO);
    for i in 0..3 {
        let slot = local + 4 * i;
        // `FCOMP` then `TEST AH,0x41; JNZ`: only a value above 1 is clamped.
        if e.mem.f32(slot) as f64 > one {
            e.mem.set_f32(slot, 1.0);
        }
        // `FCOMP` then `TEST AH,5; JP`: only a value below 0 is clamped.
        if (e.mem.f32(slot) as f64) < zero {
            e.mem.set_f32(slot, 0.0);
        }
    }
    let red = e.mem.f32(local) as f64 * range;
    let red = e.call(FTOL, &args![red]).i32();
    let green = e.mem.f32(local + 4) as f64 * range;
    let green = e.call(FTOL, &args![green]).i32();
    let blue = e.mem.f32(local + 8) as f64 * range;
    let blue = e.call(FTOL, &args![blue]).i32();
    let packed = red
        .wrapping_add(green << 8)
        .wrapping_add(blue << 16)
        .wrapping_add(0xff00_0000u32 as i32);
    e.set(this, CBillboardLeaf::m_lColor, packed as u32);
}

// Translated from 00b405c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b405c0 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throw is.
/// `CBillboardLeaf::Parse` (Xbox PDB, `cdecl`): reads a leaf from the tree
/// file: a new leaf, then tagged fields until the end tag 0x1b5d:
///
/// * 0x1b5e the colour (a file word), 0x1b5f the angle index (a byte),
///   0x1b60 the colour scale (a byte), 0x1b62 the normal (`ParseVector3`),
///   0x1b63 the texture index (a byte), 0x1b64 the wind weight (a byte over
///   255), 0x1b65 the wind group (`ParseInt`), 0x1b67 the position
///   (`ParseVector3`), 0x1b68 a float that is read and dropped;
/// * any other tag, the end tag included as a first tag, throws
///   `"malformed billboard leaf"`.
///
/// Returns the leaf.
pub fn cbillboard_leaf_parse(e: &mut Engine, file: Ptr<CTreeFileAccess>) -> Ptr<CBillboardLeaf> {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x198`, the cookie): 0x1a8
    // bytes. The parsed vectors at +0x180 (tag 0x1b62, `EBP-0x28`) and +0x174 (tag 0x1b67,
    // `EBP-0x34`); the message string at +0x130 (`EBP-0x78`) and the exception at +0x14c
    // (`EBP-0x5c`).
    e.with_stack(0x1a8, |e, frame| parse_in_frame(e, file, frame.addr()))
}

/// [`cbillboard_leaf_parse`] in the exe's frame (`frame` is its lowest address).
fn parse_in_frame(e: &mut Engine, file: Ptr<CTreeFileAccess>, frame: u32) -> Ptr<CBillboardLeaf> {
    let memory = e
        .call(OPERATOR_NEW, &args![0x4cu32])
        .ptr::<CBillboardLeaf>();
    let leaf = if memory.is_null() {
        Ptr::NULL
    } else {
        fn_00b40150(e, memory)
    };
    let mut tag = e.call(PARSE_INT, &args![file]).u32();
    loop {
        // The compiler's switch: `tag - 0x1b5e` indexes a table of 11
        // entries (`00b407d0`); 0x1b61 and 0x1b66 lead to the throw too.
        match tag.wrapping_sub(0x1b5e) {
            0 => {
                let color = e.call(PARSE_WORD, &args![file]).u32();
                e.set(leaf, CBillboardLeaf::m_lColor, color);
            }
            1 => {
                let angle = fn_00b40800(e, file);
                e.set(leaf, CBillboardLeaf::m_ucAngleIndex, angle);
            }
            2 => {
                let scale = fn_00b40800(e, file);
                e.set(leaf, CBillboardLeaf::m_ucColorScale, scale);
            }
            4 | 9 => {
                let vector = if tag == 0x1b62 {
                    frame + 0x180
                } else {
                    frame + 0x174
                };
                let parsed = e.call(PARSE_VECTOR3, &args![file, vector]).u32();
                let target = if tag == 0x1b62 { 0x1c } else { 4 };
                copy_vec3(e, parsed, leaf.addr().wrapping_add(target));
            }
            5 => {
                let texture = fn_00b40800(e, file);
                e.set(leaf, CBillboardLeaf::m_ucTextureIndex, texture);
            }
            6 => {
                let weight = fn_00b40800(e, file);
                let range: f64 = e.global(BYTE_RANGE);
                e.set(
                    leaf,
                    CBillboardLeaf::m_fWindWeight,
                    (weight as f64 / range) as f32,
                );
            }
            7 => {
                let group = e.call(PARSE_INT, &args![file]).i32();
                e.set(leaf, CBillboardLeaf::m_nWindGroup, group);
            }
            10 => {
                // The float is read and dropped (`FSTP ST0`).
                e.call(PARSE_FLOAT, &args![file]);
            }
            _ => {
                let (text, exception) = (frame + 0x130, frame + 0x14c);
                e.call(STRING_FROM_C_STRING, &args![text, MESSAGE_MALFORMED]);
                e.call(PARSE_ERROR_FROM_STRING, &args![exception, text, 0u32]);
                e.call(CXX_THROW_EXCEPTION, &args![exception, MALFORMED_THROW_INFO]);
                unreachable!("__CxxThrowException does not return (00ec7894)")
            }
        }
        tag = e.call(PARSE_INT, &args![file]).u32();
        if tag == 0x1b5d {
            return leaf;
        }
    }
}

// Translated from 00b40800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the next byte of the tree file (unnamed in the engine map): takes
/// the position, advances it, and when the position is not below the
/// vector's size calls the CRT's invalid-parameter handler (the read then
/// goes on, as the exe's does if the handler returns). Returns the byte.
pub fn fn_00b40800(e: &mut Engine, this: Ptr<CTreeFileAccess>) -> u8 {
    let index = e.get(this, CTreeFileAccess::m_nIndex);
    e.set(this, CTreeFileAccess::m_nIndex, index.wrapping_add(1));
    let first = e.get(this, CTreeFileAccess::m_vMemoryFirst).addr();
    let last = e.get(this, CTreeFileAccess::m_vMemoryLast).addr();
    if index >= last.wrapping_sub(first) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u8(first.wrapping_add(index))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b3f070,
            cbillboard_leaf_scalar_deleting_destructor(
                Ptr<CBillboardLeaf>,
                u32,
            ) -> Ptr<CBillboardLeaf>
        ),
        entry!(
            0x00b40150,
            fn_00b40150(Ptr<CBillboardLeaf>) -> Ptr<CBillboardLeaf>
        ),
        entry!(
            0x00b40210,
            cbillboard_leaf_adjust_static_lighting(Ptr<CBillboardLeaf>, Ptr, Ptr, f32)
        ),
        entry!(
            0x00b402d0,
            fn_00b402d0(Ptr<CBillboardLeaf>, Ptr<CBillboardLeaf>) -> Ptr<CBillboardLeaf>
        ),
        entry!(
            0x00b403a0,
            fn_00b403a0(Ptr<CBillboardLeaf>) -> Ptr<CBillboardLeaf>
        ),
        entry!(
            0x00b40430,
            cbillboard_leaf_get_color(Ptr<CBillboardLeaf>, Ptr) -> Ptr
        ),
        entry!(
            0x00b404c0,
            cbillboard_leaf_set_color(Ptr<CBillboardLeaf>, Ptr, u8)
        ),
        entry!(
            0x00b405c0,
            cbillboard_leaf_parse(Ptr<CTreeFileAccess>) -> Ptr<CBillboardLeaf>
        ),
        entry!(0x00b40800, fn_00b40800(Ptr<CTreeFileAccess>) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Records the calls to `addresses` into one log; each double answers
    /// with `Ret::default()`.
    fn recorded(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let record = calls.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    /// An engine with the constants and a truncating `_ftol2_sse`.
    fn leaf_engine() -> Engine {
        let mut e = Engine::new();
        e.register(FTOL, |_, a| Ret {
            eax: f64::take(a, &mut 0) as i32 as u32,
            ..Ret::default()
        });
        for page in [0x0101_1000, 0x0101_2000, 0x0101_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(BYTE_RANGE, 255.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(HALF, 0.5f64);
        e
    }

    fn leaf(e: &mut Engine) -> Ptr<CBillboardLeaf> {
        e.new_object()
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_camera_destructor_and_deletes_on_bit_zero() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[CAMERA_DESTRUCTOR, OPERATOR_DELETE]);
        let this = leaf(&mut e);
        assert_eq!(
            e.call(0x00b3_f070, &args![this, 0u32])
                .ptr::<CBillboardLeaf>(),
            this
        );
        assert_eq!(
            *calls.borrow(),
            vec![(CAMERA_DESTRUCTOR, vec![this.addr()])]
        );
        calls.borrow_mut().clear();
        e.call(0x00b3_f070, &args![this, 1u32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (CAMERA_DESTRUCTOR, vec![this.addr()]),
                (OPERATOR_DELETE, vec![this.addr()])
            ]
        );
    }

    #[test]
    fn constructor_clears_the_leaf_and_sets_the_white_colour() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[CAMERA_CONSTRUCTOR]);
        let this = leaf(&mut e);
        for offset in (0..0x4c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdead_beef);
        }
        assert_eq!(
            e.call(0x00b4_0150, &args![this]).ptr::<CBillboardLeaf>(),
            this
        );
        assert_eq!(
            *calls.borrow(),
            vec![(CAMERA_CONSTRUCTOR, vec![this.addr()])]
        );
        assert_eq!(e.get(this, CBillboardLeaf::vtable), BILLBOARD_LEAF_VTABLE);
        assert_eq!(e.get(this, CBillboardLeaf::m_ucAngleIndex), 0);
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0xffff_ffff);
        assert_eq!(e.get(this, CBillboardLeaf::m_ucColorScale), 0);
        for offset in (0x1c..0x40).step_by(4) {
            assert_eq!(e.mem.u32(this.addr() + offset), 0);
        }
        assert_eq!(e.get(this, CBillboardLeaf::m_ucTextureIndex), 0);
        assert_eq!(e.get(this, CBillboardLeaf::m_fWindWeight), 0.0);
        assert_eq!(e.get(this, CBillboardLeaf::m_nWindGroup), 0);
        // The camera part is the camera constructor's.
        assert_eq!(e.mem.u32(this.addr() + 4), 0xdead_beef);
    }

    #[test]
    fn get_color_splits_the_bytes_over_255() {
        let mut e = leaf_engine();
        let this = leaf(&mut e);
        e.set(this, CBillboardLeaf::m_lColor, 0xff80_4020);
        let result = Ptr::<()>::new(e.mem.alloc(12));
        assert_eq!(
            e.call(0x00b4_0430, &args![this, result]).ptr::<()>(),
            result
        );
        assert_eq!(e.mem.f32(result.addr()), (0x20 as f64 / 255.0) as f32);
        assert_eq!(e.mem.f32(result.addr() + 4), (0x40 as f64 / 255.0) as f32);
        assert_eq!(e.mem.f32(result.addr() + 8), (0x80 as f64 / 255.0) as f32);
    }

    /// A `CVec3` of three floats in memory.
    fn vec3(e: &mut Engine, x: f32, y: f32, z: f32) -> Ptr<()> {
        let p = e.mem.alloc(12);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        e.mem.set_f32(p + 8, z);
        Ptr::new(p)
    }

    #[test]
    fn set_color_clamps_truncates_and_adds_the_alpha() {
        let mut e = leaf_engine();
        let this = leaf(&mut e);
        let color = vec3(&mut e, 2.0, -1.0, 0.5);
        e.call(0x00b4_04c0, &args![this, color, 0u32]);
        // 1.0 -> 255, -1 -> 0, 0.5 -> 127.5 -> 127.
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0xff7f_00ff);
        let color = vec3(&mut e, 0.0, 1.0, 1.0);
        e.call(0x00b4_04c0, &args![this, color, 0u32]);
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0xffff_ff00);
    }

    #[test]
    fn set_color_scales_by_the_colour_scale_first_when_asked() {
        let mut e = leaf_engine();
        e.register(VEC3_SCALE_IN_PLACE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        let this = leaf(&mut e);
        e.set(this, CBillboardLeaf::m_ucColorScale, 51);
        let color = vec3(&mut e, 1.0, 1.0, 0.0);
        e.call(0x00b4_04c0, &args![this, color, 1u32]);
        // 51 / 255 = 0.2 (as a float): 0.2 * 255 = 51.000001 -> 51, and
        // the source colour is untouched.
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0xff00_3333);
        assert_eq!(e.mem.f32(color.addr()), 1.0);
    }

    #[test]
    fn adjust_static_lighting_scales_the_colour_by_the_angle_factor() {
        let mut e = leaf_engine();
        e.register(VEC3_SUBTRACT, |_, _| Ret::default());
        e.register(VEC3_NORMALIZE, |_, _| Ret::default());
        e.register(VEC3_ANGLE_BETWEEN, |_, _| Ret {
            st0: std::f64::consts::PI,
            ..Ret::default()
        });
        e.register(COS, |_, a| Ret {
            st0: f64::take(a, &mut 0).cos(),
            ..Ret::default()
        });
        e.register(VEC3_SCALE_COPY, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VEC3_SCALE_IN_PLACE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[1]);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        let this = leaf(&mut e);
        e.set(this, CBillboardLeaf::m_ucColorScale, 255);
        e.set(this, CBillboardLeaf::m_lColor, 0xff33_00ff);
        let a = vec3(&mut e, 0.0, 0.0, 0.0);
        let b = vec3(&mut e, 0.0, 0.0, 0.0);
        // cos(pi) = -1: the angle term is 0 and the factor is the weight, 0.5.
        e.call(0x00b4_0210, &args![this, a, b, 0.5f32]);
        // red 1.0 * 0.5 -> 127, green 0, blue 0.2 * 0.5 -> 25.
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0xff19_007f);
    }

    #[test]
    fn adjust_static_lighting_passes_the_vectors_to_the_vector_helpers() {
        let mut e = leaf_engine();
        let calls = recorded(
            &mut e,
            &[VEC3_SUBTRACT, VEC3_NORMALIZE, VEC3_SCALE_IN_PLACE],
        );
        e.register(VEC3_ANGLE_BETWEEN, |_, _| Ret::default());
        e.register(COS, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register(VEC3_SCALE_COPY, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let this = leaf(&mut e);
        let a = vec3(&mut e, 1.0, 2.0, 3.0);
        let b = vec3(&mut e, 4.0, 5.0, 6.0);
        e.call(0x00b4_0210, &args![this, a, b, 0.0f32]);
        let log = calls.borrow();
        // this + 4 minus A, normalise, B minus A, normalise.
        assert_eq!(log.len(), 5);
        assert_eq!(log[0].0, VEC3_SUBTRACT);
        assert_eq!(log[0].1[0], this.addr() + 4);
        assert_eq!(log[0].1[2], a.addr());
        assert_eq!(log[1], (VEC3_NORMALIZE, vec![log[0].1[1]]));
        assert_eq!(log[2].0, VEC3_SUBTRACT);
        assert_eq!(log[2].1[0], b.addr());
        assert_eq!(log[2].1[2], a.addr());
        assert_eq!(log[3], (VEC3_NORMALIZE, vec![log[2].1[1]]));
        // The colour is stored through `SetColor`'s scaling by the leaf's
        // colour scale (0 here).
        assert_eq!(log[4].0, VEC3_SCALE_IN_PLACE);
    }

    #[test]
    fn assign_copies_every_field_and_quiets_the_wind_weight() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[CAMERA_ASSIGN]);
        let this = leaf(&mut e);
        let other = leaf(&mut e);
        e.set(other, CBillboardLeaf::m_ucAngleIndex, 3);
        e.set(other, CBillboardLeaf::m_lColor, 0x1234_5678);
        e.set(other, CBillboardLeaf::m_ucColorScale, 9);
        for i in 0..9 {
            e.mem.set_u32(other.addr() + 0x1c + 4 * i, 100 + i);
        }
        e.set(other, CBillboardLeaf::m_ucTextureIndex, 7);
        e.mem.set_u32(other.addr() + 0x44, 0x7f80_0001);
        e.set(other, CBillboardLeaf::m_nWindGroup, -4);
        assert_eq!(
            e.call(0x00b4_02d0, &args![this, other])
                .ptr::<CBillboardLeaf>(),
            this
        );
        assert_eq!(
            *calls.borrow(),
            vec![(CAMERA_ASSIGN, vec![this.addr(), other.addr()])]
        );
        assert_eq!(e.get(this, CBillboardLeaf::m_ucAngleIndex), 3);
        assert_eq!(e.get(this, CBillboardLeaf::m_lColor), 0x1234_5678);
        assert_eq!(e.get(this, CBillboardLeaf::m_ucColorScale), 9);
        for i in 0..9 {
            assert_eq!(e.mem.u32(this.addr() + 0x1c + 4 * i), 100 + i);
        }
        assert_eq!(e.get(this, CBillboardLeaf::m_ucTextureIndex), 7);
        // The wind weight went through FLD/FSTP: quiet bit set.
        assert_eq!(e.mem.u32(this.addr() + 0x44), 0x7fc0_0001);
        assert_eq!(e.get(this, CBillboardLeaf::m_nWindGroup), -4);
        // Assigning to itself does nothing.
        calls.borrow_mut().clear();
        e.call(0x00b4_02d0, &args![this, this]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn copy_makes_a_new_leaf_from_the_original() {
        let mut e = leaf_engine();
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        recorded(&mut e, &[CAMERA_CONSTRUCTOR, CAMERA_ASSIGN]);
        let original = leaf(&mut e);
        e.set(original, CBillboardLeaf::m_lColor, 0x0102_0304);
        e.set(original, CBillboardLeaf::m_nWindGroup, 6);
        let copy = e
            .call(0x00b4_03a0, &args![original])
            .ptr::<CBillboardLeaf>();
        assert_ne!(copy, original);
        assert_eq!(e.get(copy, CBillboardLeaf::vtable), BILLBOARD_LEAF_VTABLE);
        assert_eq!(e.get(copy, CBillboardLeaf::m_lColor), 0x0102_0304);
        assert_eq!(e.get(copy, CBillboardLeaf::m_nWindGroup), 6);
    }

    /// A tree file whose bytes are `bytes`, read position `index`.
    fn tree_file(e: &mut Engine, bytes: &[u8], index: u32) -> Ptr<CTreeFileAccess> {
        let file: Ptr<CTreeFileAccess> = e.new_object();
        let memory = e.mem.alloc(bytes.len().max(1) as u32 + 16);
        e.mem.write(memory, bytes);
        e.set(file, CTreeFileAccess::m_nIndex, index);
        e.set(file, CTreeFileAccess::m_vMemoryFirst, Ptr::new(memory));
        e.set(
            file,
            CTreeFileAccess::m_vMemoryLast,
            Ptr::new(memory + bytes.len() as u32),
        );
        file
    }

    #[test]
    fn read_byte_advances_the_position() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[INVALID_PARAMETER]);
        let file = tree_file(&mut e, &[10, 20, 30], 1);
        assert_eq!(e.call(0x00b4_0800, &args![file]).u8(), 20);
        assert_eq!(e.get(file, CTreeFileAccess::m_nIndex), 2);
        assert_eq!(e.call(0x00b4_0800, &args![file]).u8(), 30);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn read_byte_past_the_end_calls_the_invalid_parameter_handler_and_goes_on() {
        let mut e = leaf_engine();
        let calls = recorded(&mut e, &[INVALID_PARAMETER]);
        let file = tree_file(&mut e, &[10, 20, 30], 3);
        // The byte after the vector is still mapped memory (zero).
        assert_eq!(e.call(0x00b4_0800, &args![file]).u8(), 0);
        assert_eq!(*calls.borrow(), vec![(INVALID_PARAMETER, vec![])]);
        assert_eq!(e.get(file, CTreeFileAccess::m_nIndex), 4);
    }

    /// Doubles for the parse helpers: `ParseInt` answers the given tags and
    /// values in order, `ParseWord` 0x00cafe, `ParseFloat` is recorded and
    /// `ParseVector3` writes (1,2,3) then (4,5,6).
    fn parse_engine(ints: Vec<u32>) -> (Engine, Calls) {
        let mut e = leaf_engine();
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let calls = recorded(
            &mut e,
            &[
                CAMERA_CONSTRUCTOR,
                PARSE_FLOAT,
                STRING_FROM_C_STRING,
                PARSE_ERROR_FROM_STRING,
            ],
        );
        let mut next = 0;
        e.register_double(PARSE_INT, move |_, _| {
            next += 1;
            Ret {
                eax: ints[next - 1],
                ..Ret::default()
            }
        });
        e.register(PARSE_WORD, |_, _| Ret {
            eax: 0xcafe,
            ..Ret::default()
        });
        let mut vectors = 0;
        e.register_double(PARSE_VECTOR3, move |e, a| {
            for i in 0..3 {
                e.mem.set_f32(a[1] + 4 * i, (vectors * 3 + i + 1) as f32);
            }
            vectors += 1;
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        (e, calls)
    }

    #[test]
    fn parse_reads_the_tagged_fields_until_the_end_tag() {
        // tags: colour, angle, scale, normal, texture, wind weight, wind
        // group (+ its value), position, float, end.
        let tags = vec![
            0x1b5e, 0x1b5f, 0x1b60, 0x1b62, 0x1b63, 0x1b64, 0x1b65, 77, 0x1b67, 0x1b68, 0x1b5d,
        ];
        let (mut e, calls) = parse_engine(tags);
        let file = tree_file(&mut e, &[5, 6, 7, 8, 255], 0);
        let leaf = e.call(0x00b4_05c0, &args![file]).ptr::<CBillboardLeaf>();
        assert_eq!(e.get(leaf, CBillboardLeaf::vtable), BILLBOARD_LEAF_VTABLE);
        assert_eq!(e.get(leaf, CBillboardLeaf::m_lColor), 0xcafe);
        assert_eq!(e.get(leaf, CBillboardLeaf::m_ucAngleIndex), 5);
        assert_eq!(e.get(leaf, CBillboardLeaf::m_ucColorScale), 6);
        assert_eq!(e.mem.f32(leaf.addr() + 0x1c), 1.0);
        assert_eq!(e.mem.f32(leaf.addr() + 0x20), 2.0);
        assert_eq!(e.mem.f32(leaf.addr() + 0x24), 3.0);
        assert_eq!(e.get(leaf, CBillboardLeaf::m_ucTextureIndex), 7);
        assert_eq!(
            e.get(leaf, CBillboardLeaf::m_fWindWeight),
            (8.0f64 / 255.0) as f32
        );
        assert_eq!(e.get(leaf, CBillboardLeaf::m_nWindGroup), 77);
        assert_eq!(e.mem.f32(leaf.addr() + 4), 4.0);
        assert_eq!(e.mem.f32(leaf.addr() + 8), 5.0);
        assert_eq!(e.mem.f32(leaf.addr() + 12), 6.0);
        assert_eq!(e.get(file, CTreeFileAccess::m_nIndex), 4);
        let log = calls.borrow();
        assert_eq!(log.iter().filter(|c| c.0 == PARSE_FLOAT).count(), 1);
        assert!(log.iter().all(|c| c.0 != PARSE_ERROR_FROM_STRING));
    }

    #[test]
    fn parse_throws_on_an_unknown_tag() {
        for tag in [0x1b61, 0x1b66, 0x1b69, 0x1b5d, 0x1b5c, 0] {
            let (mut e, calls) = parse_engine(vec![tag]);
            e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
            let file = tree_file(&mut e, &[], 0);
            let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(0x00b4_05c0, &args![file]);
            }));
            assert!(thrown.is_err());
            let log = calls.borrow();
            let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
            assert_eq!(string.1[1], MESSAGE_MALFORMED);
            let error = log.iter().find(|c| c.0 == PARSE_ERROR_FROM_STRING).unwrap();
            assert_eq!(error.1[1], string.1[0]);
            assert_eq!(error.1[2], 0);
        }
    }

    #[test]
    fn parse_keeps_its_locals_at_the_exe_frame_offsets() {
        let (mut e, calls) = parse_engine(vec![0x1b62, 0x1b67, 0x1b5d]);
        let seen: Calls = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(PARSE_VECTOR3, move |_, a| {
            record.borrow_mut().push((PARSE_VECTOR3, a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let file = tree_file(&mut e, &[], 0);
        let frame = e.next_stack_local(0x1a8);
        e.call(0x00b4_05c0, &args![file]);
        // Tag 0x1b62 parses into +0x180, tag 0x1b67 into +0x174.
        let vectors: Vec<u32> = seen.borrow().iter().map(|c| c.1[1]).collect();
        assert_eq!(vectors, vec![frame + 0x180, frame + 0x174]);
        // The throw path: the message at +0x130, the exception at +0x14c.
        let (mut e, calls_throw) = parse_engine(vec![0]);
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        let file = tree_file(&mut e, &[], 0);
        let frame = e.next_stack_local(0x1a8);
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b4_05c0, &args![file]);
        }));
        assert!(thrown.is_err());
        let log = calls_throw.borrow();
        let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
        assert_eq!(string.1[0], frame + 0x130);
        let error = log.iter().find(|c| c.0 == PARSE_ERROR_FROM_STRING).unwrap();
        assert_eq!(error.1[0], frame + 0x14c);
        drop(calls);
    }

    #[test]
    fn set_color_and_adjust_static_lighting_use_the_exe_frame_offsets() {
        let mut e = leaf_engine();
        let calls = recorded(
            &mut e,
            &[VEC3_SUBTRACT, VEC3_NORMALIZE, VEC3_SCALE_IN_PLACE],
        );
        e.register(VEC3_ANGLE_BETWEEN, |_, _| Ret::default());
        e.register(COS, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        let scaled: Calls = Rc::new(RefCell::new(Vec::new()));
        let record = scaled.clone();
        e.register_double(VEC3_SCALE_COPY, move |_, a| {
            record.borrow_mut().push((VEC3_SCALE_COPY, a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = leaf(&mut e);
        let a = vec3(&mut e, 1.0, 2.0, 3.0);
        let b = vec3(&mut e, 4.0, 5.0, 6.0);
        let frame = e.next_stack_local(0x80);
        e.call(0x00b4_0210, &args![this, a, b, 0.0f32]);
        let log = calls.borrow();
        // The two directions at +0x68 and +0x74; the colour at +0x54, scaled into +0x48.
        assert_eq!(log[0].1[1], frame + 0x68);
        assert_eq!(log[2].1[1], frame + 0x74);
        assert_eq!(scaled.borrow()[0].1[..2], [frame + 0x54, frame + 0x48]);
        drop(log);
        // SetColor alone: its colour copy at +0x14 of its 0x20-byte frame.
        let color = vec3(&mut e, 0.5, 0.5, 0.5);
        let frame = e.next_stack_local(0x20);
        e.call(0x00b4_04c0, &args![this, color, 1u32]);
        let log = calls.borrow();
        let last = log.last().unwrap();
        assert_eq!((last.0, last.1[0]), (VEC3_SCALE_IN_PLACE, frame + 0x14));
    }
}
