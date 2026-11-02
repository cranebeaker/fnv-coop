//! `Havok SDK/hkaskeletonmapperutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaSkeletonMapperUtils` (Xbox PDB name) builds the mapping between two skeletons. A
//! transform is 0x30 bytes: translation `+0`, rotation quaternion `+0x10`, scale `+0x20` (four
//! lanes each, the last lane of the scale is not used). The chain mapping the functions fill
//! is 0x70 bytes: four `u16` bone indices (start and end bone of each skeleton), a transform at
//! `+0x10` and a second one at `+0x40`; its class is unconfirmed. The class `Params` has its
//! copy constructor here (its field names are not in the Xbox PDB, so the fields are described
//! by offset). SSE code computes lane by lane in `f32`; a sum of three squares is
//! `z + (y + x)` the way the `SHUFPS`/`ADDSS` sequences add it; `RCPPS` and `RSQRTSS` are the
//! host CPU's estimates. x87 code computes in `f64` and rounds at each `float` store; a float
//! loaded through `FLD` goes through [`x87_quiet`].
//!
//! Functions that report an error build the message in a report builder (`00cb1600`, `00cc01c0`,
//! `00cc00e0`) and send it to the sink at `01268360` (slot `0xc`).
//!
//! Not translated: none of these functions has a C++ exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, cross, mul, splat, sub};
use crate::units::havok_sdk::hkpconvexradiusbuilder::normalising_scale;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, refined_reciprocal, shufps, store_bits, store_vec, sum3,
    unpckhps, Vec4, HALF_F32, ONES, SIGN_MASK, THREE_F32, ZERO_F32,
};

/// `(0, 0, 0, 1)`, the identity quaternion, and the `float` `1.0` the scales start from.
pub(crate) const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const FLOAT_ONE: u32 = 0x0101_2068;
/// Four `-0.5`: the term added to `w * w` in the quaternion rotation.
pub(crate) const ROTATION_BIAS: u32 = 0x010c_71e0;
/// Sign mask of the first three lanes (the conjugate), and the all-ones mask of the first three
/// lanes (lane 3 zero).
pub(crate) const CONJUGATE_MASK: u32 = 0x010c_4c00;
pub(crate) const XYZ_MASK: u32 = 0x010c_4c10;
/// The `float` the chain mapping uses as its length tolerance.
const CHAIN_TOLERANCE: u32 = 0x0107_18c0;
/// `hkQuaternion::setMul` (`00c66320`, `RET 8`): `this` = result, `(left, right)`.
pub(crate) const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The quaternion operation `00c74ac0` (`RET 8`, no name in the engine map): `this` = result,
/// `(first, second)`.
const QUATERNION_SET_MUL_2: u32 = 0x00c7_4ac0;
/// The quaternion operation `00c74b70` (`RET 8`, no name in the engine map): `this` = result,
/// `(first, second)`.
const QUATERNION_SET_MUL_3: u32 = 0x00c7_4b70;
/// The rotation between two directions (`00cd9d10`, `RET 8`, no name in the engine map): `this` =
/// result, `(first, second)`.
const ROTATION_BETWEEN: u32 = 0x00cd_9d10;
/// The chain end point of a bone (`00ce5560`, cdecl, no name in the engine map): `(bones, index,
/// first output vector, second output vector)`.
const BONE_POINTS: u32 = 0x00ce_5560;
/// The mapping validity check (`00ce50e0`, `RET 4`, `hkqstransform.cpp`, no name in the engine
/// map): `this` = the mapping, `(byte)`; answers a pointer to the byte.
const MAPPING_IS_VALID: u32 = 0x00ce_50e0;
/// The report builder: constructor (`00cb1600`, `this`, buffer, size), text append (`00cc01c0`,
/// `this`, text) and destructor (`00cc00e0`, `this`); the global at `01268360` holds the sink
/// object whose slot `0xc` takes `(1, id, text, file, line)`.
const REPORT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const REPORT_BUILDER_TEXT: u32 = 0x00cc_01c0;
const REPORT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const REPORT_SINK: u32 = 0x0126_8360;
/// `".\Mapper\hkaSkeletonMapperUtils.cpp"` and the messages of `computeMappingTransform`.
const REPORT_FILE: u32 = 0x010c_bab0;
const MESSAGE_TOLERANCE: u32 = 0x010c_bbd8;
const MESSAGE_ZERO_LENGTH: u32 = 0x010c_bb78;
const MESSAGE_UNSUPPORTED: u32 = 0x010c_bb58;
const MESSAGE_INVALID: u32 = 0x010c_bb44;

/// The report the functions of this unit write: a builder over a 0x200-byte buffer holding
/// `message`, sent to the sink at `01268360` (slot `0xc`) with `(1, id, text, file, line)`,
/// then destroyed.
fn report(e: &mut Engine, message: u32, id: u32, line: u32) {
    e.with_stack(0x40, |e, builder| {
        e.with_stack(0x20c, |e, buffer| {
            let (builder, buffer) = (builder.addr(), buffer.addr());
            e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
            e.call(REPORT_BUILDER_TEXT, &args![builder, message]);
            let sink = e.global::<u32>(REPORT_SINK);
            e.vcall(sink, 0xc, &args![1u32, id, buffer, REPORT_FILE, line]);
            e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

/// `XORPS`: the bits of each lane xor the mask.
pub(crate) fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// `ANDPS`: the bits of each lane and the mask.
pub(crate) fn and_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
}

/// `SQRTSS` of the sum of the first three squares of `v`.
fn length_of(v: Vec4) -> f32 {
    sum3(mul(v, v)).sqrt()
}

/// `2 * (cross * w + ((w * w + bias) * v + dot * q))` where `w` is lane 3 of the quaternion `q`
/// and `dot` the sum of the first three lanes of `q * v`: the quaternion rotation of `v` the
/// code unrolls, `cross_term` being the cross product in the operand order of the caller.
pub(crate) fn rotate_twice(cross_term: Vec4, v: Vec4, q: Vec4, bias: Vec4) -> Vec4 {
    let dot = sum3(mul(q, v));
    let w = splat(q, 3);
    let weighted = add(mul(w, w), bias);
    let r = add(mul(cross_term, w), add(mul(weighted, v), mul([dot; 4], q)));
    add(r, r)
}

/// The identity transform: translation zero, rotation `(0, 0, 0, 1)` from `010c71b0`, scale
/// the `float` at `01012068` in all four lanes.
fn store_identity_transform(e: &mut Engine, at: u32) {
    store_bits(e, at, [0; 4]);
    let rotation = load_bits(e, IDENTITY_QUATERNION);
    store_bits(e, at + 0x10, rotation);
    let one = e.mem.u32(FLOAT_ONE);
    store_bits(e, at + 0x20, [one; 4]);
}

// Translated from 00ce5430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x70-byte chain mapping (unconfirmed class, no name in the engine map):
/// the four bone indices zero and two identity transforms, at `+0x10` and `+0x40`. Returns
/// `this`.
pub fn fn_00ce5430(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at, 0);
    e.mem.set_u16(at + 4, 0);
    e.mem.set_u16(at + 2, 0);
    e.mem.set_u16(at + 6, 0);
    store_identity_transform(e, at + 0x10);
    store_identity_transform(e, at + 0x40);
    this
}

// Translated from 00ce55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scale factors between two skeletons for one mapping (no name in the engine map; cdecl, eight
/// words). Both outputs (`0x30` bytes each) start as identity transforms. `parameters` is the
/// mapper parameter block: `+0` the mode, `+4` and `+8` the bone indices of mode 1, `+0xC` and
/// `+0x10` two `float`s. Mode 0 measures the distance of the two points `00ce5560` gives for
/// each skeleton, mode 1 the length of the translation of one bone of each, any other mode uses
/// the two floats alone. The scale lanes (`+0x20`) of the outputs are `(L2 / L1 * +0x10) / +0xC`
/// and `(L1 / L2 * +0xC) / +0x10` (x87, rounded to `float`), or `+0x10 / +0xC` and `+0xC / +0x10`
/// without lengths. The second word is not read.
pub fn fn_00ce55d0(
    e: &mut Engine,
    parameters: Ptr,
    _unused_1: u32,
    bones_a: Ptr,
    index_a: u32,
    bones_b: Ptr,
    index_b: u32,
    out_a: Ptr,
    out_b: Ptr,
) {
    store_identity_transform(e, out_a.addr());
    store_identity_transform(e, out_b.addr());
    let params = parameters.addr();
    let mode = e.mem.u32(params);
    let lengths = match mode {
        0 => Some(e.with_stack(0x40, |e, block| {
            let at = block.addr();
            e.call(BONE_POINTS, &args![bones_a, index_a, at + 0x10, at]);
            e.call(BONE_POINTS, &args![bones_b, index_b, at + 0x30, at + 0x20]);
            let first = sub(load_vec(e, at), load_vec(e, at + 0x10));
            let second = sub(load_vec(e, at + 0x20), load_vec(e, at + 0x30));
            (length_of(first), length_of(second))
        })),
        1 => {
            let slot_a = e.mem.u32(params + 4).wrapping_mul(0x30);
            let slot_b = e.mem.u32(params + 8).wrapping_mul(0x30);
            let first = load_vec(e, bones_a.addr().wrapping_add(slot_a));
            let second = load_vec(e, bones_b.addr().wrapping_add(slot_b));
            Some((length_of(first), length_of(second)))
        }
        _ => None,
    };
    let c = f64::from(x87_quiet(e.mem.f32(params + 0xc)));
    let d = f64::from(x87_quiet(e.mem.f32(params + 0x10)));
    let (scale_a, scale_b) = match lengths {
        Some((first, second)) => {
            let first = f64::from(x87_quiet(first));
            let second = f64::from(x87_quiet(second));
            (
                (second / first * d / c) as f32,
                (first / second * c / d) as f32,
            )
        }
        None => ((d / c) as f32, (c / d) as f32),
    };
    store_bits(e, out_a.addr() + 0x20, [scale_a.to_bits(); 4]);
    store_bits(e, out_b.addr() + 0x20, [scale_b.to_bits(); 4]);
}

/// `out = inverse(a) * b` for two transforms (the mode 0 of `computeMappingTransform`, also used
/// by the pose conversions of `hkaSkeletonUtils`): translation
/// `rotate(conj(a.rotation), b.translation) - rotate(a.rotation, a.translation)` (the unrolled
/// quaternion rotation, doubled), rotation `conj(a.rotation) * b.rotation` (`00c66320`), scale
/// `b.scale * (1 / a.scale)` (`RCPPS` plus one Newton step, lane 3 cleared).
pub(crate) fn inverse_compose(e: &mut Engine, a: u32, b: u32, out: u32) {
    let bias = load_vec(e, ROTATION_BIAS);
    let translation = load_vec(e, a);
    let rotation = load_vec(e, a + 0x10);
    let scale = load_vec(e, a + 0x20);
    let forward = sub(
        mul(
            shufps(translation, translation, 0xc9),
            shufps(rotation, rotation, 0xd2),
        ),
        mul(
            shufps(translation, translation, 0xd2),
            shufps(rotation, rotation, 0xc9),
        ),
    );
    let moved = rotate_twice(forward, translation, rotation, bias);
    let negated = xor_bits(moved, load_bits(e, SIGN_MASK));
    let conjugate = xor_bits(load_vec(e, a + 0x10), load_bits(e, CONJUGATE_MASK));
    let ones = load_vec(e, ONES);
    let reciprocal_input = shufps(scale, unpckhps(scale, ones), 0xc4);
    let inverse_scale = and_bits(
        reciprocal_input.map(refined_reciprocal),
        load_bits(e, XYZ_MASK),
    );
    let b_translation = load_vec(e, b);
    let backward = rotate_twice(
        cross(conjugate, b_translation),
        b_translation,
        conjugate,
        bias,
    );
    store_vec(e, out, add(backward, negated));
    e.with_stack(0x10, |e, copy| {
        store_vec(e, copy.addr(), conjugate);
        e.call(
            QUATERNION_SET_MUL,
            &args![out + 0x10, copy.addr(), b + 0x10],
        );
    });
    let b_scale = load_vec(e, b + 0x20);
    store_vec(e, out + 0x20, mul(b_scale, inverse_scale));
}

// Translated from 00ce5a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::computeMappingTransform` (Xbox PDB name; cdecl): the transform `out`
/// that maps the bone transform `a` onto `b`. Mode 0: translation
/// `rotate(conj(a.rotation), b.translation) - rotate(a.rotation, a.translation)` (the unrolled
/// quaternion rotation, doubled), rotation `conj(a.rotation) * b.rotation` (`00c66320`), scale
/// `b.scale * (1 / a.scale)` (`RCPPS` plus one Newton step, lane 3 cleared). Mode 1 scales by
/// the ratio of the lengths of the two translations (`fallback_scale` when a length is not above
/// `tolerance`): rotation by `00c74ac0`, translation `b.translation - scale * a.translation`;
/// a negative tolerance, one zero length only, and any other mode are reported (ids
/// `abba39c3`, `abba2b94`, `abbaef91`). The transform is then checked (`00ce50e0`) and a
/// report (`abba10fc`) is sent when it is not valid.
pub fn hka_skeleton_mapper_utils_compute_mapping_transform(
    e: &mut Engine,
    mode: u32,
    a: Ptr,
    b: Ptr,
    out: Ptr,
    tolerance: f32,
    fallback_scale: f32,
) {
    let (a, b, out) = (a.addr(), b.addr(), out.addr());
    if mode == 0 {
        inverse_compose(e, a, b, out);
    } else if mode == 1 {
        let tolerance = x87_quiet(tolerance);
        if tolerance < 0.0 {
            report(e, MESSAGE_TOLERANCE, 0xabba_39c3, 0x34e);
        }
        let length_a = length_of(load_vec(e, a));
        let length_b = length_of(load_vec(e, b));
        let small_a = length_a <= tolerance;
        let small_b = length_b <= tolerance;
        let scale = if !small_a && !small_b {
            (f64::from(length_b) / f64::from(length_a)) as f32
        } else {
            if small_a != small_b {
                report(e, MESSAGE_ZERO_LENGTH, 0xabba_2b94, 0x35c);
            }
            x87_quiet(fallback_scale)
        };
        store_bits(e, out + 0x20, [scale.to_bits(); 4]);
        e.call(QUATERNION_SET_MUL_2, &args![out + 0x10, b + 0x10, a + 0x10]);
        let negated = f32::from_bits(scale.to_bits() ^ 0x8000_0000);
        let moved = mul([negated; 4], load_vec(e, a));
        store_vec(e, out, add(moved, load_vec(e, b)));
    } else {
        report(e, MESSAGE_UNSUPPORTED, 0xabba_ef91, 0x36c);
    }
    let valid = e.with_stack(4, |e, flag| {
        let answer = e.call(MAPPING_IS_VALID, &args![out, flag.addr()]).u32();
        e.mem.u8(answer)
    });
    if valid == 0 {
        report(e, MESSAGE_INVALID, 0xabba_10fc, 0x371);
    }
}

// Translated from 00ce5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a bone chain of a skeleton from bone `start` up its parents (the `i16` array whose data
/// pointer is at `skeleton + 0xC`) while the bone is above `stop` (signed compare). Starting
/// from a zero vector, each bone's transform (`transforms`, 0x30 bytes each) rotates the vector
/// by its quaternion and adds its translation (`out_vector`); `out_length` is the sum of the
/// translation lengths (x87 sum, stored as `float`). No name in the engine map.
pub fn fn_00ce5e60(
    e: &mut Engine,
    skeleton: Ptr,
    transforms: Ptr,
    stop: i16,
    start: i16,
    out_length: Ptr,
    out_vector: Ptr,
) {
    let (length_at, vector_at) = (out_length.addr(), out_vector.addr());
    e.mem.set_u32(length_at, 0);
    store_bits(e, vector_at, [0; 4]);
    let mut index = start;
    while index > stop {
        let slot = i32::from(index).wrapping_mul(0x30) as u32;
        let at = transforms.addr().wrapping_add(slot);
        let current = load_vec(e, vector_at);
        let rotation = load_vec(e, at + 0x10);
        let bias = load_vec(e, ROTATION_BIAS);
        let moved = rotate_twice(cross(rotation, current), current, rotation, bias);
        store_vec(e, vector_at, moved);
        let translation = load_vec(e, at);
        store_vec(e, vector_at, add(moved, translation));
        let total = f64::from(x87_quiet(e.mem.f32(length_at)));
        let step = f64::from(x87_quiet(length_of(load_vec(e, at))));
        e.mem.set_f32(length_at, (total + step) as f32);
        let parents = e.mem.u32(skeleton.addr() + 0xc);
        let parent_slot = (i32::from(index) as u32).wrapping_mul(2);
        index = e.mem.u16(parents.wrapping_add(parent_slot)) as i16;
    }
}

// Translated from 00ce5f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::computeChainMapping` (Xbox PDB name; cdecl): the mapping transform
/// of bone `index_a` of `bones_a` onto bone `index_b` of `bones_b` (0x30-byte transforms);
/// `tolerance` and `fallback_scale` go through `FLD`/`FSTP`.
pub fn hka_skeleton_mapper_utils_compute_chain_mapping(
    e: &mut Engine,
    mode: u32,
    bones_a: Ptr,
    index_a: i16,
    bones_b: Ptr,
    index_b: i16,
    out: Ptr,
    tolerance: f32,
    fallback_scale: f32,
) {
    let slot_a = i32::from(index_a).wrapping_mul(0x30) as u32;
    let slot_b = i32::from(index_b).wrapping_mul(0x30) as u32;
    hka_skeleton_mapper_utils_compute_mapping_transform(
        e,
        mode,
        Ptr::new(bones_a.addr().wrapping_add(slot_a)),
        Ptr::new(bones_b.addr().wrapping_add(slot_b)),
        out,
        x87_quiet(tolerance),
        x87_quiet(fallback_scale),
    );
}

// Translated from 00ce5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills one chain mapping `out` (no name in the engine map; cdecl, ten words) for a chain of
/// skeleton A from bone `start_a` to `end_a` and one of skeleton B from `start_b` to `end_b`:
/// the four bone indices (low words), the mapping transform of the start bones at `+0x10`
/// (tolerance from `010718c0`, fallback scale 1.0), and for the end: the rotation between the
/// two normalised chain vectors (`00cd9d10`) at `+0x40`, a quaternion from `00c74b70` over the
/// end bones' rotations at `+0x50`, and the reciprocal of A's chain length (x87, as a
/// `float`) in all lanes at `+0x60`. The chain vectors come from `00ce5e60`; the normalisation
/// is the refined `RSQRTSS`, zero for a zero sum.
pub fn fn_00ce5fe0(
    e: &mut Engine,
    mode: u32,
    skeleton_a: Ptr,
    bones_a: Ptr,
    start_a: u32,
    end_a: u32,
    skeleton_b: Ptr,
    bones_b: Ptr,
    start_b: u32,
    end_b: u32,
    out: Ptr,
) {
    let at = out.addr();
    e.mem.set_u16(at + 4, start_b as u16);
    e.mem.set_u16(at + 6, end_b as u16);
    e.mem.set_u16(at, start_a as u16);
    e.mem.set_u16(at + 2, end_a as u16);
    let tolerance = f32::from_bits(x87_quiet_bits(e.mem.u32(CHAIN_TOLERANCE)));
    hka_skeleton_mapper_utils_compute_chain_mapping(
        e,
        mode,
        bones_a,
        start_a as i16,
        bones_b,
        start_b as i16,
        Ptr::new(at + 0x10),
        tolerance,
        1.0,
    );
    e.with_stack(0x50, |e, block| {
        let base = block.addr();
        let (length_a, length_b) = (base, base + 4);
        let (vector_a, vector_b, rotation) = (base + 0x10, base + 0x20, base + 0x30);
        fn_00ce5e60(
            e,
            skeleton_a,
            bones_a,
            start_a as i16,
            end_a as i16,
            Ptr::new(length_a),
            Ptr::new(vector_a),
        );
        fn_00ce5e60(
            e,
            skeleton_b,
            bones_b,
            start_b as i16,
            end_b as i16,
            Ptr::new(length_b),
            Ptr::new(vector_b),
        );
        let zero = e.mem.f32(ZERO_F32);
        let half = e.mem.f32(HALF_F32);
        let three = e.mem.f32(THREE_F32);
        let chain_a = load_vec(e, vector_a);
        let scale = normalising_scale(sum3(mul(chain_a, chain_a)), zero, half, three);
        store_vec(e, vector_a, mul([scale; 4], chain_a));
        let chain_b = load_vec(e, vector_b);
        let scale = normalising_scale(sum3(mul(chain_b, chain_b)), zero, half, three);
        store_vec(e, vector_b, mul([scale; 4], chain_b));
        e.call(ROTATION_BETWEEN, &args![rotation, vector_b, vector_a]);
        let reciprocal = (1.0 / f64::from(x87_quiet(e.mem.f32(length_a)))) as f32;
        let rotation_value = load_bits(e, rotation);
        store_bits(e, at + 0x40, rotation_value);
        store_bits(e, at + 0x60, [reciprocal.to_bits(); 4]);
        let slot_end_a = i32::from(end_a as i16).wrapping_mul(0x30) as u32;
        let slot_end_b = i32::from(end_b as i16).wrapping_mul(0x30) as u32;
        let rotation_a = bones_a.addr().wrapping_add(slot_end_a) + 0x10;
        let rotation_b = bones_b.addr().wrapping_add(slot_end_b) + 0x10;
        e.call(
            QUATERNION_SET_MUL_3,
            &args![at + 0x50, rotation_a, rotation_b],
        );
    });
}

// Translated from 00ce9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::Params::Params` (Xbox PDB name), the copy constructor (`RET 4`,
/// `this` and the source): `+0..0xC` copied, the floats at `+0xC` and `+0x10` through
/// `FLD`/`FSTP`, three empty `hkArray`s (data, size, capacity and flags `0x80000000`) at
/// `+0x14`, `+0x20` and `+0x2C` filled with the source's 8-byte elements (reserved through
/// `hkArrayUtil::_reserve`), the bytes at `+0x38` and `+0x39` and the words `+0x3C..0x50`.
/// Returns `this`.
pub fn hka_skeleton_mapper_utils_params_params(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 4, 8] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xc, 0x10] {
        let word = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, word);
    }
    for array in [0x14, 0x20, 0x2c] {
        e.mem.set_u32(to + array, 0);
        e.mem.set_u32(to + array + 4, 0);
        e.mem.set_u32(to + array + 8, 0x8000_0000);
    }
    let flag = e.mem.u8(from + 0x38);
    e.mem.set_u8(to + 0x38, flag);
    let flag = e.mem.u8(from + 0x39);
    e.mem.set_u8(to + 0x39, flag);
    for offset in [0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for array in [0x14, 0x20, 0x2c] {
        copy_pair_array(e, to + array, from + array);
    }
    this
}

/// Fills the empty `hkArray` of 8-byte elements at `array` (data, size, capacity and flags)
/// from the one at `source`: the capacity is raised to the source's size (`_reserve` with
/// twice the capacity when that is larger), the first `min(size, source size)` elements and then
/// the rest are copied (a null destination is skipped), and the size is the source's.
fn copy_pair_array(e: &mut Engine, array: u32, source: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let source_size = e.mem.i32(source + 4);
    let size = e.mem.i32(array + 4);
    let common = if source_size > size {
        size
    } else {
        source_size
    };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < source_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if source_size < doubled {
            doubled
        } else {
            source_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let source_data = e.mem.u32(source);
    let data = e.mem.u32(array);
    if common > 0 {
        for index in 0..common as u32 {
            let word = e.mem.u32(source_data.wrapping_add(8 * index));
            e.mem.set_u32(data.wrapping_add(8 * index), word);
            let word = e.mem.u32(source_data.wrapping_add(8 * index + 4));
            e.mem.set_u32(data.wrapping_add(8 * index + 4), word);
        }
    }
    let mut from = source_data.wrapping_add((common as u32).wrapping_mul(8));
    let mut to = data.wrapping_add((common as u32).wrapping_mul(8));
    let mut remaining = source_size.wrapping_sub(common);
    while remaining > 0 {
        if to != 0 {
            let word = e.mem.u32(from);
            e.mem.set_u32(to, word);
            let word = e.mem.u32(from + 4);
            e.mem.set_u32(to + 4, word);
        }
        from = from.wrapping_add(8);
        to = to.wrapping_add(8);
        remaining -= 1;
    }
    e.mem.set_i32(array + 4, source_size);
}

// `hkaSkeletonMapperUtils::createMapping` (00ce61c0) and what it needs.
//
// The two outputs are mapper data blocks (unconfirmed class; read off the code): `+0` and `+4`
// the two skeletons (reference counted), `+8` an `hkArray` of 0x40-byte simple mappings (two
// `u16` bones, then a transform at `+0x10`), `+0x14` an `hkArray` of 0x70-byte chain mappings,
// `+0x20` an `hkArray` of `u16` unmapped bones, `+0x30` a transform, the byte `+0x60` and the
// mode word `+0x64`. A skeleton has its parent indices at `+0xc`, its 8-byte bone records (name
// pointer first) at `+0x18`, its bone count at `+0x1c` and its reference pose at `+0x24`.
// `Params` (the copy at `frame + 0xa8`): `+0` and `+4` the skeletons, `+8` the name compare
// function, `+0xc` the tolerance and `+0x10` the scale of the simple mappings, `+0x14`, `+0x20`
// and `+0x2c` arrays of name pairs (chains, A to B, B to A), the bytes `+0x38` (map by name or
// distance) and `+0x39` (chains from the hierarchy), `+0x3c` the mode and `+0x40` the block
// `00ce55d0` reads.

/// The chain mapping copy constructor (`00ce4a50`, `hkaskeletonmapper.cpp`, `this` = the
/// destination, `(source)`).
const CHAIN_MAPPING_COPY: u32 = 0x00ce_4a50;
/// `hkaSkeletonUtils::findBoneWithName` (`00cdd390`, cdecl `(skeleton, name, compare)`, the bone
/// in `AX`), `hkaSkeletonUtils::getBoneChain` (`00cde3d0`, cdecl `(result byte, skeleton, start,
/// end, chain array)`) and `hkaSkeletonUtils::transformLocalPoseToModelPose` (`00cddcf0`, cdecl
/// `(count, parents, local pose, model pose)`).
const FIND_BONE_WITH_NAME: u32 = 0x00cd_d390;
const GET_BONE_CHAIN: u32 = 0x00cd_e3d0;
const LOCAL_POSE_TO_MODEL_POSE: u32 = 0x00cd_dcf0;
/// The reference count of an `hkReferencedObject` (`hkreferencedobject.cpp`, `this` only, no
/// names in the engine map): `00c90510` takes a reference, `00c905b0` drops one.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `00c90940` (`hkarrayutil.cpp`, no name in the engine map, cdecl `(allocator, array, element
/// size)`): makes room for one more element.
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkOstream::operator<<` (`00cc05a0`, overload 6 in the engine map, `this` = the builder,
/// `(bone record)`, answers the builder).
const REPORT_BUILDER_BONE: u32 = 0x00cc_05a0;
/// `hkaSkeletonMapperUtils::Params::~Params` (`00c7c2b0`, `this` only).
const PARAMS_DESTRUCT: u32 = 0x00c7_c2b0;
/// `hkString::strCasecmp` (`00cb0fe0`), the compare used when none is given.
const STRING_CASE_COMPARE: u32 = 0x00cb_0fe0;
/// The `float` the nearest-bone search starts from (`100000.0`).
const NEAREST_START: u32 = 0x0101_7130;
/// What the sink's slot `0x1c` gets at the start: an id and `"hkaSkeletonMapperUtils::
/// createMapping"`.
const SCOPE_ID: u32 = 0x54e4_323e;
const SCOPE_NAME: u32 = 0x010c_bd0c;
/// The report texts: `"Couldn't find bones for mapping ("`, `"Couldn't make a chain ("`, `" - "`,
/// `")"`, `"). Start bone not found in either skeleton."`, `"). End bone not found in either
/// skeleton."`, `"). Start or end bone not found."`, `")  Perhaps a bone is mapped twice?"`,
/// `"Creating chain "`, `"Creating chains: "`, `"( "`, `", "`, `" )   ( "`, `" )"` and
/// `"Matching "`.
const TEXT_NO_BONES: u32 = 0x010c_bce8;
const TEXT_NO_CHAIN: u32 = 0x010c_bca4;
const TEXT_DASH: u32 = 0x0108_1aa8;
const TEXT_CLOSE: u32 = 0x0103_d078;
const TEXT_NO_START: u32 = 0x010c_bcbc;
const TEXT_NO_END: u32 = 0x010c_bc78;
const TEXT_NO_START_OR_END: u32 = 0x010c_bc00;
const TEXT_MAPPED_TWICE: u32 = 0x010c_bc44;
const TEXT_CREATING_CHAIN: u32 = 0x010c_bc68;
const TEXT_CREATING_CHAINS: u32 = 0x010c_bc20;
const TEXT_OPEN: u32 = 0x010c_bc34;
const TEXT_COMMA: u32 = 0x0101_2630;
const TEXT_BETWEEN: u32 = 0x010c_bc38;
const TEXT_END: u32 = 0x010c_bc40;
const TEXT_MATCHING: u32 = 0x010c_bbf4;
/// The buffer every report of `createMapping` is built in (`frame + 0x270`, 0x200 bytes).
const REPORT_BUFFER: u32 = 0x270;
/// The frame of `createMapping` (`AND ESP,-16; SUB ESP,0x464`), offsets from its aligned base.
const CREATE_FRAME_SIZE: u32 = 0x490;
/// The mask of the capacity in an `hkArray` capacity word and the flag of storage it does not
/// own.
const CAPACITY_BITS: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

/// One piece of a report: a text (`00cc01c0`) or a bone record (`00cc05a0`).
#[derive(Clone, Copy)]
enum ReportPiece {
    Text(u32),
    Bone(u32),
}

/// A report of `createMapping`: the builder at `builder` over the buffer at `frame + 0x270`
/// (`00cb1600`), the pieces chained through the builder each call answers, the sink's slot
/// `0xc` with `(level, id, buffer, file, line)`, then the builder's destructor.
fn mapping_report(
    e: &mut Engine,
    frame: u32,
    builder: u32,
    pieces: &[ReportPiece],
    level: u32,
    id: u32,
    line: u32,
) {
    let buffer = frame + REPORT_BUFFER;
    e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
    let mut current = builder;
    for piece in pieces {
        current = match *piece {
            ReportPiece::Text(text) => e.call(REPORT_BUILDER_TEXT, &args![current, text]),
            ReportPiece::Bone(bone) => e.call(REPORT_BUILDER_BONE, &args![current, bone]),
        }
        .u32();
    }
    let sink = e.global::<u32>(REPORT_SINK);
    e.vcall(sink, 0xc, &args![level, id, buffer, REPORT_FILE, line]);
    e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
}

/// The error report naming a pair of bones (level 1): the opening text, the first name, `" - "`,
/// the second name, then the closing text.
fn pair_report(e: &mut Engine, frame: u32, builder: u32, pieces: [u32; 4], id: u32, line: u32) {
    let [opening, start, end, close] = pieces;
    mapping_report(
        e,
        frame,
        builder,
        &[
            ReportPiece::Text(opening),
            ReportPiece::Text(start),
            ReportPiece::Text(TEXT_DASH),
            ReportPiece::Text(end),
            ReportPiece::Text(close),
        ],
        1,
        id,
        line,
    );
}

/// `size` bytes from the memory router's stack (`[router]`, rounded up to 0x80): answers the
/// old top.
fn router_stack_take(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(
        router,
        (size.wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top),
    );
    top
}

/// The allocator of the memory router (`router + 0x10`).
fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The local `hkArray` of `u16` at `array` set to `count` entries of `0xffff`: the stack block
/// (`count * 2`), the capacity word `capacity`, `_reserve` when that is too small, the fill.
/// Answers the stack top it took (kept to give back at the end).
fn stack_bone_map(e: &mut Engine, array: u32, count: u32, capacity: u32) -> u32 {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
    let top = router_stack_take(e, count.wrapping_mul(2));
    e.mem.set_u32(array, top);
    e.mem.set_u32(array + 8, capacity);
    let allocator = router_allocator(e);
    let room = (e.mem.u32(array + 8) & CAPACITY_BITS) as i32;
    if room < count as i32 {
        let doubled = room.wrapping_add(room);
        let wanted = if (count as i32) < doubled {
            doubled
        } else {
            count as i32
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 2u32]);
    }
    let size = e.mem.u32(array + 4);
    let missing = count.wrapping_sub(size) as i32;
    if missing > 0 {
        let start = e.mem.u32(array).wrapping_add(size.wrapping_mul(2));
        for index in 0..missing as u32 {
            e.mem.set_u16(start.wrapping_add(2 * index), 0xffff);
        }
    }
    e.mem.set_u32(array + 4, count);
    top
}

/// Gives a local bone map back: the size cleared when the stack top it took is its data, the
/// router's stack top restored, the storage freed through the allocator (virtual slot `0x10`,
/// `(data, bytes)`) when it is owned, then emptied.
fn release_bone_map(e: &mut Engine, array: u32, top: u32) {
    if top == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_BITS).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// Gives a stack block of `count` transforms back: the router's stack top restored, the block
/// freed when `capacity` says it is owned.
fn release_transforms(e: &mut Engine, block: u32, capacity: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, block);
    let allocator = router_allocator(e);
    if (capacity as i32) >= 0 {
        e.vcall(
            allocator,
            0x10,
            &args![block, (capacity & CAPACITY_BITS).wrapping_mul(0x30)],
        );
    }
}

/// An empty local `hkArray` of `u16` (the chains `getBoneChain` fills) at `array`.
fn clear_chain(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// Frees a chain filled by `getBoneChain`: the size cleared, the storage freed through the
/// allocator (slot `0x10`, `(data, capacity * 2)`) unless it is not owned, then emptied.
fn free_chain(e: &mut Engine, array: u32) {
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_BITS).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// The `u16` at `index` (signed) of the array whose data pointer is at `holder`.
fn bone_at(e: &Engine, holder: u32, index: i32) -> u16 {
    let data = e.mem.u32(holder);
    e.mem.u16(data.wrapping_add((index as u32).wrapping_mul(2)))
}

fn set_bone_at(e: &mut Engine, holder: u32, index: i32, value: u16) {
    let data = e.mem.u32(holder);
    e.mem
        .set_u16(data.wrapping_add((index as u32).wrapping_mul(2)), value);
}

/// The 16-bit bone index as a signed index (`MOVSX`).
fn signed(bone: u16) -> i32 {
    i32::from(bone as i16)
}

/// The transform of `bone` in an array of 0x30-byte transforms.
fn transform_of(base: u32, bone: u16) -> u32 {
    base.wrapping_add((signed(bone) as u32).wrapping_mul(0x30))
}

/// The bone record of `bone` (`skeleton + 0x18`, 8 bytes each).
fn bone_record(e: &Engine, skeleton: u32, bone: u16) -> u32 {
    e.mem
        .u32(skeleton + 0x18)
        .wrapping_add((signed(bone) as u32).wrapping_mul(8))
}

/// Points `slot` at `object`: a reference taken on the new one, the old one's dropped.
fn set_skeleton(e: &mut Engine, slot: u32, object: u32) {
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let old = e.mem.u32(slot);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(slot, object);
}

/// Room for one more element of `size` bytes in the `hkArray` at `array` (`00c90940` when it is
/// full).
fn make_room(e: &mut Engine, array: u32, size: u32) {
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_BITS {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, size]);
    }
}

/// Appends the simple mapping at `frame + 0x100` (the two bones, the transform at `+0x10`) to
/// the array at `array` (a null slot is skipped).
fn push_simple_mapping(e: &mut Engine, array: u32, frame: u32) {
    make_room(e, array, 0x40);
    let slot = (e.mem.u32(array + 4) << 6).wrapping_add(e.mem.u32(array));
    if slot != 0 {
        let first = e.mem.u16(frame + 0x100);
        e.mem.set_u16(slot, first);
        let second = e.mem.u16(frame + 0x102);
        e.mem.set_u16(slot + 2, second);
        for part in [0x10, 0x20, 0x30] {
            copy_vec(e, frame + 0x100 + part, slot + part);
        }
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Appends a copy (`00ce4a50`) of the chain mapping at `entry` to the array at `array` (a null
/// slot is skipped).
fn push_chain_mapping(e: &mut Engine, array: u32, entry: u32) {
    make_room(e, array, 0x70);
    let slot = e
        .mem
        .u32(array + 4)
        .wrapping_mul(0x70)
        .wrapping_add(e.mem.u32(array));
    if slot != 0 {
        e.call(CHAIN_MAPPING_COPY, &args![slot, entry]);
    }
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Appends the pair `(first, second)` to the 8-byte array at `array`.
fn push_pair(e: &mut Engine, array: u32, first: u32, second: u32) {
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_BITS {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(8));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(slot, first);
    e.mem.set_u32(slot + 4, second);
}

/// Appends `bone` to the `u16` array at `array`.
fn push_bone(e: &mut Engine, array: u32, bone: u32) {
    let allocator = router_allocator(e);
    if e.mem.u32(array + 4) == e.mem.u32(array + 8) & CAPACITY_BITS {
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 2u32]);
    }
    let slot = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(2));
    e.mem.set_u16(slot, bone as u16);
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// One simple mapping built at `frame + 0x100`: the bones `bone_a` and `bone_b`, the transform
/// at `frame + 0x110` starting as `start` (translation and scale bits) with the identity
/// rotation, then `computeChainMapping` from `bone_a` of `pose_a` onto `bone_b` of `pose_b` with
/// `(tolerance, scale)`.
fn simple_mapping(
    e: &mut Engine,
    frame: u32,
    mode: u32,
    (pose_a, bone_a): (u32, u16),
    (pose_b, bone_b): (u32, u16),
    (tolerance, scale): (f32, f32),
    start: ([u32; 4], [u32; 4]),
) {
    store_bits(e, frame + 0x110, start.0);
    let identity = load_bits(e, IDENTITY_QUATERNION);
    store_bits(e, frame + 0x120, identity);
    store_bits(e, frame + 0x130, start.1);
    e.mem.set_u16(frame + 0x100, bone_a);
    e.mem.set_u16(frame + 0x102, bone_b);
    hka_skeleton_mapper_utils_compute_chain_mapping(
        e,
        mode,
        Ptr::new(pose_a),
        bone_a as i16,
        Ptr::new(pose_b),
        bone_b as i16,
        Ptr::new(frame + 0x110),
        tolerance,
        scale,
    );
}

/// The four lanes of the `float` at `FLOAT_ONE` (`MOVSS`, `SHUFPS 0`).
fn one_lanes(e: &Engine) -> [u32; 4] {
    [e.mem.u32(FLOAT_ONE); 4]
}

/// The bones of `createMapping` it works with, after the swap.
struct MappingContext {
    frame: u32,
    local: u32,
    mode: u32,
    skeleton_a: u32,
    skeleton_b: u32,
    first: u32,
    second: u32,
    model_a: u32,
    model_b: u32,
    pose_a: u32,
    pose_b: u32,
    count_a: u32,
    count_b: u32,
}

impl MappingContext {
    fn a_map(&self) -> u32 {
        self.frame + 0x80
    }

    fn b_map(&self) -> u32 {
        self.frame + 0x90
    }

    fn compare(&self, e: &Engine) -> u32 {
        e.mem.u32(self.local + 8)
    }

    /// `findBoneWithName` in `skeleton` with the compare of the parameters.
    fn find(&self, e: &mut Engine, skeleton: u32, name: u32) -> u16 {
        let compare = self.compare(e);
        e.call(FIND_BONE_WITH_NAME, &args![skeleton, name, compare])
            .u32() as u16
    }
}

/// The simple mappings named by the parameters (`00ce66a0`, `00ce6897`).
fn named_simple_mappings(e: &mut Engine, c: &MappingContext) {
    let frame = c.frame;
    let count = e.mem.i32(c.local + 0x24);
    let mut index = 0;
    while index < count {
        let pair = e.mem.u32(c.local + 0x20).wrapping_add((index as u32) * 8);
        let name = e.mem.u32(pair);
        let bone_a = c.find(e, c.skeleton_a, name);
        let name = e.mem.u32(pair + 4);
        let bone_b = c.find(e, c.skeleton_b, name);
        if bone_a == 0xffff || bone_b == 0xffff {
            let names = [e.mem.u32(pair), e.mem.u32(pair + 4)];
            pair_report(
                e,
                frame,
                frame + 0xc,
                [TEXT_NO_BONES, names[0], names[1], TEXT_CLOSE],
                0xabba_1c64,
                0x7f,
            );
        } else {
            let scale = x87_quiet(e.mem.f32(c.local + 0x10));
            let tolerance = x87_quiet(e.mem.f32(c.local + 0xc));
            let start = ([0; 4], one_lanes(e));
            simple_mapping(
                e,
                frame,
                c.mode,
                (c.pose_a, bone_a),
                (c.pose_b, bone_b),
                (tolerance, scale),
                start,
            );
            push_simple_mapping(e, c.first + 8, frame);
            set_bone_at(e, c.b_map(), signed(bone_b), bone_a);
        }
        index += 1;
    }
    let count = e.mem.i32(c.local + 0x30);
    let mut index = 0;
    while index < count {
        let pair = e.mem.u32(c.local + 0x2c).wrapping_add((index as u32) * 8);
        let name = e.mem.u32(pair);
        let bone_b = c.find(e, c.skeleton_b, name);
        let name = e.mem.u32(pair + 4);
        let bone_a = c.find(e, c.skeleton_a, name);
        if bone_b == 0xffff || bone_a == 0xffff {
            let names = [e.mem.u32(pair), e.mem.u32(pair + 4)];
            pair_report(
                e,
                frame,
                frame + 0xc,
                [TEXT_NO_BONES, names[0], names[1], TEXT_CLOSE],
                0xabba_1c64,
                0x9e,
            );
        } else {
            let tolerance = x87_quiet(e.mem.f32(CHAIN_TOLERANCE));
            let start = ([0; 4], one_lanes(e));
            simple_mapping(
                e,
                frame,
                c.mode,
                (c.pose_b, bone_b),
                (c.pose_a, bone_a),
                (tolerance, 1.0),
                start,
            );
            push_simple_mapping(e, c.second + 8, frame);
            set_bone_at(e, c.a_map(), signed(bone_a), bone_b);
        }
        index += 1;
    }
}

/// The chains of mode 0, one per name pair of the parameters (`00ce6ab1`).
fn named_chains(e: &mut Engine, c: &MappingContext) {
    let count = e.mem.i32(c.local + 0x18);
    let mut index = 0;
    while index < count {
        let pair = e.mem.u32(c.local + 0x14).wrapping_add((index as u32) * 8);
        named_chain(e, c, pair);
        index += 1;
    }
}

/// One chain of mode 0: the start and end bones of the pair found in both skeletons (a bone
/// missing in one skeleton is taken through the other's bone map), both bone chains, and the
/// two chain mappings (`inverse(A) * B` of the end bones at `+0x40` and of the start bones at
/// `+0x10`, and the reverse for the second output); the inner bones of both chains are marked
/// `0xfffe` in the bone maps.
fn named_chain(e: &mut Engine, c: &MappingContext, pair: u32) {
    let frame = c.frame;
    let name = e.mem.u32(pair);
    let mut start_a = c.find(e, c.skeleton_a, name);
    let name = e.mem.u32(pair + 4);
    let mut end_a = c.find(e, c.skeleton_a, name);
    let name = e.mem.u32(pair);
    let mut start_b = c.find(e, c.skeleton_b, name);
    let name = e.mem.u32(pair + 4);
    let mut end_b = c.find(e, c.skeleton_b, name);
    let names = [e.mem.u32(pair), e.mem.u32(pair + 4)];
    if (start_a as i16) < 0 && (start_b as i16) < 0 {
        let pieces = [TEXT_NO_CHAIN, names[0], names[1], TEXT_NO_START];
        pair_report(e, frame, frame + 0x1cc, pieces, 0x3852_1c64, 0xb9);
        return;
    }
    if (end_a as i16) < 0 && (end_b as i16) < 0 {
        let pieces = [TEXT_NO_CHAIN, names[0], names[1], TEXT_NO_END];
        pair_report(e, frame, frame + 0x1b4, pieces, 0x3852_1c64, 0xbf);
        return;
    }
    if (start_a as i16) < 0 {
        start_a = bone_at(e, c.b_map(), signed(start_b));
    }
    if (end_a as i16) < 0 {
        end_a = bone_at(e, c.b_map(), signed(end_b));
    }
    if (start_b as i16) < 0 {
        start_b = bone_at(e, c.a_map(), signed(start_a));
    }
    if (end_b as i16) < 0 {
        end_b = bone_at(e, c.a_map(), signed(end_a));
    }
    let (chain_a, chain_b) = (frame + 0x1c, frame + 0xc);
    clear_chain(e, chain_a);
    clear_chain(e, chain_b);
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2a,
            c.skeleton_a,
            u32::from(start_a),
            u32::from(end_a),
            chain_a
        ],
    );
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2b,
            c.skeleton_b,
            u32::from(start_b),
            u32::from(end_b),
            chain_b
        ],
    );
    let close = [TEXT_NO_CHAIN, names[0], names[1], TEXT_CLOSE];
    if e.mem.u8(frame + 0x2a) == 0 || e.mem.u8(frame + 0x2b) == 0 {
        pair_report(e, frame, frame + 0x190, close, 0xabba_1c63, 0xd2);
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    let size_a = e.mem.i32(chain_a + 4);
    let size_b = e.mem.i32(chain_b + 4);
    if size_a < 2 || size_b < 2 {
        pair_report(e, frame, frame + 0x1c0, close, 0xabba_1c64, 0xdc);
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    // The inner bones must not be mapped yet (reported, not refused).
    let inner_a = size_a - 1;
    let mut index = 1;
    while index < inner_a {
        if bone_at(e, c.a_map(), signed(bone_at(e, chain_a, index))) != 0xffff {
            pair_report(e, frame, frame + 0x1d8, close, 0xabba_1c65, 0xe6);
        }
        index += 1;
    }
    let inner_b = size_b - 1;
    let mut index = 1;
    while index < inner_b {
        if bone_at(e, c.b_map(), signed(bone_at(e, chain_b, index))) != 0xffff {
            pair_report(e, frame, frame + 0x1a8, close, 0xabba_1c66, 0xef);
        }
        index += 1;
    }
    let pieces = [
        ReportPiece::Text(TEXT_CREATING_CHAIN),
        ReportPiece::Bone(bone_record(e, c.skeleton_a, start_a)),
        ReportPiece::Text(TEXT_DASH),
        ReportPiece::Bone(bone_record(e, c.skeleton_a, end_a)),
    ];
    mapping_report(e, frame, frame + 0x1e4, &pieces, 0, u32::MAX, 0xf9);

    let entry = frame + 0x1f0;
    let identity = load_bits(e, IDENTITY_QUATERNION);
    e.mem.set_u16(entry, start_a);
    store_bits(e, entry + 0x20, identity);
    store_bits(e, entry + 0x50, identity);
    e.mem.set_u16(entry + 2, end_a);
    e.mem.set_u16(entry + 4, start_b);
    e.mem.set_u16(entry + 6, end_b);
    inverse_compose(
        e,
        transform_of(c.model_a, end_a),
        transform_of(c.model_b, end_b),
        entry + 0x40,
    );
    inverse_compose(
        e,
        transform_of(c.model_a, start_a),
        transform_of(c.model_b, start_b),
        entry + 0x10,
    );
    push_chain_mapping(e, c.first + 0x14, entry);
    let identity = load_bits(e, IDENTITY_QUATERNION);
    e.mem.set_u16(entry, start_b);
    e.mem.set_u16(entry + 6, end_a);
    store_bits(e, entry + 0x20, identity);
    store_bits(e, entry + 0x50, identity);
    e.mem.set_u16(entry + 2, end_b);
    e.mem.set_u16(entry + 4, start_a);
    inverse_compose(
        e,
        transform_of(c.model_b, end_b),
        transform_of(c.model_a, end_a),
        entry + 0x40,
    );
    inverse_compose(
        e,
        transform_of(c.model_b, start_b),
        transform_of(c.model_a, start_a),
        entry + 0x10,
    );
    push_chain_mapping(e, c.second + 0x14, entry);
    let mut index = 1;
    while index < inner_a {
        let bone = bone_at(e, chain_a, index);
        set_bone_at(e, c.a_map(), signed(bone), 0xfffe);
        index += 1;
    }
    let mut index = 1;
    while index < inner_b {
        let bone = bone_at(e, chain_b, index);
        set_bone_at(e, c.b_map(), signed(bone), 0xfffe);
        index += 1;
    }
    free_chain(e, chain_b);
    free_chain(e, chain_a);
}

/// The chains of the other modes, one per group of four names (start and end in A, start and
/// end in B) of the parameters (`00ce7927`).
fn quad_chains(e: &mut Engine, c: &MappingContext) {
    let count = e.mem.i32(c.local + 0x18) / 2;
    if count <= 0 {
        return;
    }
    let mut offset = 0u32;
    let mut remaining = count;
    loop {
        let quad = e.mem.u32(c.local + 0x14).wrapping_add(offset);
        quad_chain(e, c, quad);
        offset = offset.wrapping_add(0x10);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
}

/// One chain of the other modes: all four bones must be found, both chains must exist and have
/// two bones at least; none of their bones may be mapped yet (reported, not refused). The two
/// chain mappings come from `00ce5fe0`, and every bone of both chains is marked `0xfffe`.
fn quad_chain(e: &mut Engine, c: &MappingContext, quad: u32) {
    let frame = c.frame;
    let name = e.mem.u32(quad);
    let start_a = c.find(e, c.skeleton_a, name);
    let name = e.mem.u32(quad + 4);
    let end_a = c.find(e, c.skeleton_a, name);
    let name = e.mem.u32(quad + 8);
    let start_b = c.find(e, c.skeleton_b, name);
    let name = e.mem.u32(quad + 0xc);
    let end_b = c.find(e, c.skeleton_b, name);
    let names = [0, 4, 8, 0xc].map(|offset| e.mem.u32(quad + offset));
    if (start_a as i16) < 0 || (end_a as i16) < 0 {
        let pieces = [TEXT_NO_CHAIN, names[0], names[1], TEXT_NO_START_OR_END];
        pair_report(e, frame, frame + 0x180, pieces, 0xabba_a938, 0x145);
        return;
    }
    if (start_b as i16) < 0 || (end_b as i16) < 0 {
        let pieces = [TEXT_NO_CHAIN, names[2], names[3], TEXT_NO_START_OR_END];
        pair_report(e, frame, frame + 0x260, pieces, 0xabba_a938, 0x14b);
        return;
    }
    let (chain_a, chain_b) = (frame + 0xc, frame + 0x1c);
    clear_chain(e, chain_a);
    clear_chain(e, chain_b);
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2b,
            c.skeleton_a,
            u32::from(start_a),
            u32::from(end_a),
            chain_a
        ],
    );
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2a,
            c.skeleton_b,
            u32::from(start_b),
            u32::from(end_b),
            chain_b
        ],
    );
    let close_a = [TEXT_NO_CHAIN, names[0], names[1], TEXT_CLOSE];
    let close_b = [TEXT_NO_CHAIN, names[2], names[3], TEXT_CLOSE];
    let failure = if e.mem.u8(frame + 0x2b) == 0 {
        Some((frame + 0x190, close_a, 0xabba_1c63, 0x157))
    } else if e.mem.u8(frame + 0x2a) == 0 {
        Some((frame + 0x1c0, close_b, 0xabba_1c63, 0x15d))
    } else if e.mem.i32(chain_a + 4) < 2 {
        Some((frame + 0x1e4, close_a, 0xabba_1d64, 0x167))
    } else if e.mem.i32(chain_b + 4) < 2 {
        Some((frame + 0x1a8, close_b, 0xabba_1d64, 0x16e))
    } else {
        None
    };
    if let Some((builder, pieces, id, line)) = failure {
        pair_report(e, frame, builder, pieces, id, line);
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    let size_a = e.mem.i32(chain_a + 4);
    let size_b = e.mem.i32(chain_b + 4);
    let twice_a = [TEXT_NO_CHAIN, names[0], names[1], TEXT_MAPPED_TWICE];
    let mut index = 0;
    while index < size_a {
        if bone_at(e, c.a_map(), signed(bone_at(e, chain_a, index))) != 0xffff {
            pair_report(e, frame, frame + 0x1d8, twice_a, 0xabba_1d65, 0x178);
        }
        index += 1;
    }
    let twice_b = [TEXT_NO_CHAIN, names[2], names[3], TEXT_MAPPED_TWICE];
    let mut index = 0;
    while index < size_b {
        if bone_at(e, c.b_map(), signed(bone_at(e, chain_b, index))) != 0xffff {
            pair_report(e, frame, frame + 0x1b4, twice_b, 0xabba_1d66, 0x181);
        }
        index += 1;
    }
    let pieces = [
        TEXT_CREATING_CHAINS,
        TEXT_OPEN,
        names[0],
        TEXT_COMMA,
        names[1],
        TEXT_BETWEEN,
        names[2],
        TEXT_COMMA,
        names[3],
        TEXT_END,
    ]
    .map(ReportPiece::Text);
    mapping_report(e, frame, frame + 0x1cc, &pieces, 0, u32::MAX, 0x18d);
    fn_00ce5430(e, Ptr::new(frame + 0x100));
    fn_00ce5430(e, Ptr::new(frame + 0x1f0));
    let (skeleton_a, skeleton_b) = (Ptr::new(c.skeleton_a), Ptr::new(c.skeleton_b));
    let (pose_a, pose_b) = (Ptr::new(c.pose_a), Ptr::new(c.pose_b));
    let (start_a, end_a) = (u32::from(start_a), u32::from(end_a));
    let (start_b, end_b) = (u32::from(start_b), u32::from(end_b));
    fn_00ce5fe0(
        e,
        c.mode,
        skeleton_a,
        pose_a,
        start_a,
        end_a,
        skeleton_b,
        pose_b,
        start_b,
        end_b,
        Ptr::new(frame + 0x100),
    );
    fn_00ce5fe0(
        e,
        c.mode,
        skeleton_b,
        pose_b,
        start_b,
        end_b,
        skeleton_a,
        pose_a,
        start_a,
        end_a,
        Ptr::new(frame + 0x1f0),
    );
    push_chain_mapping(e, c.second + 0x14, frame + 0x1f0);
    push_chain_mapping(e, c.first + 0x14, frame + 0x100);
    let mut index = 0;
    while index < size_a {
        let bone = bone_at(e, chain_a, index);
        set_bone_at(e, c.a_map(), signed(bone), 0xfffe);
        index += 1;
    }
    let mut index = 0;
    while index < size_b {
        let bone = bone_at(e, chain_b, index);
        set_bone_at(e, c.b_map(), signed(bone), 0xfffe);
        index += 1;
    }
    free_chain(e, chain_b);
    free_chain(e, chain_a);
}

/// The mappings found without names in the parameters (`00ce83dc`): each bone of A is paired
/// with every bone of B whose name the compare function finds equal (the parameters' function,
/// or `strCasecmp` when there is none and the mode is not 0 or the tolerance is not above 0),
/// or else with the nearest bone of B (model space translations, squared distance) when it is
/// nearer than the squared tolerance. Each pair of bones both still unmapped becomes a simple
/// mapping in both directions (tolerance from the parameters, scale 1.0) and is reported.
fn automatic_mappings(e: &mut Engine, c: &MappingContext) {
    let frame = c.frame;
    let given = c.compare(e);
    let tolerance = f64::from(x87_quiet(e.mem.f32(c.local + 0xc)));
    let pairs = frame + 0xc;
    clear_chain(e, pairs);
    let compare = if given != 0 {
        given
    } else if c.mode != 0 {
        STRING_CASE_COMPARE
    } else if 0.0 < tolerance {
        0
    } else {
        STRING_CASE_COMPARE
    };
    let squared = (tolerance * tolerance) as f32;
    store_vec(e, frame + 0x180, [squared, 0.0, 0.0, 0.0]);
    let mut bone = 0i32;
    while bone < c.count_a as i32 {
        let own = c.model_a.wrapping_add((bone as u32).wrapping_mul(0x30));
        if compare != 0 {
            let mut other = 0i32;
            while other < c.count_b as i32 {
                let names_a = e.mem.u32(c.skeleton_a + 0x18);
                let name_a = e
                    .mem
                    .u32(names_a.wrapping_add((bone as u32).wrapping_mul(8)));
                let names_b = e.mem.u32(c.skeleton_b + 0x18);
                let name_b = e
                    .mem
                    .u32(names_b.wrapping_add((other as u32).wrapping_mul(8)));
                let differs = e.call(compare, &args![name_a & !1, name_b & !1]).u32();
                if differs == 0 {
                    push_pair(e, pairs, bone as u32, other as u32);
                }
                other += 1;
            }
        } else {
            let mut nearest = x87_quiet(e.mem.f32(NEAREST_START));
            let mut found = u32::MAX;
            let mine = load_vec(e, own);
            let mut other = 0i32;
            while other < c.count_b as i32 {
                let theirs = c.model_b.wrapping_add((other as u32).wrapping_mul(0x30));
                let apart = sub(mine, load_vec(e, theirs));
                let distance = x87_quiet(sum3(mul(apart, apart)));
                if f64::from(nearest) > f64::from(distance) {
                    nearest = distance;
                    found = other as u32;
                }
                other += 1;
            }
            let limit = x87_quiet(load_vec(e, frame + 0x180)[0]);
            if f64::from(limit) > f64::from(x87_quiet(nearest)) {
                push_pair(e, pairs, bone as u32, found);
            }
        }
        bone += 1;
    }
    let count = e.mem.i32(pairs + 4);
    let mut index = 0;
    while index < count {
        let at = e
            .mem
            .u32(pairs)
            .wrapping_add((index as u32).wrapping_mul(8));
        let bone_a = e.mem.u16(at);
        let bone_b = e.mem.u16(at + 4);
        if bone_at(e, c.a_map(), signed(bone_a)) == 0xffff
            && bone_at(e, c.b_map(), signed(bone_b)) == 0xffff
        {
            let tolerance = x87_quiet(e.mem.f32(c.local + 0xc));
            store_bits(e, frame + 0x260, [0; 4]);
            let ones = one_lanes(e);
            store_bits(e, frame + 0x190, ones);
            simple_mapping(
                e,
                frame,
                c.mode,
                (c.pose_a, bone_a),
                (c.pose_b, bone_b),
                (tolerance, 1.0),
                ([0; 4], ones),
            );
            push_simple_mapping(e, c.first + 8, frame);
            let tolerance = x87_quiet(e.mem.f32(c.local + 0xc));
            let start = (load_bits(e, frame + 0x260), load_bits(e, frame + 0x190));
            simple_mapping(
                e,
                frame,
                c.mode,
                (c.pose_b, bone_b),
                (c.pose_a, bone_a),
                (tolerance, 1.0),
                start,
            );
            push_simple_mapping(e, c.second + 8, frame);
            set_bone_at(e, c.a_map(), signed(bone_a), bone_b);
            set_bone_at(e, c.b_map(), signed(bone_b), bone_a);
            let pieces = [
                ReportPiece::Text(TEXT_MATCHING),
                ReportPiece::Bone(bone_record(e, c.skeleton_a, bone_a)),
            ];
            mapping_report(e, frame, frame + 0x180, &pieces, 0, u32::MAX, 0x231);
        }
        index += 1;
    }
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(pairs + 8);
    e.mem.set_u32(pairs + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(pairs);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_BITS).wrapping_mul(8)],
        );
    }
}

/// The chains taken from the hierarchy (`00ce88f2`): for each mapped bone of A, its nearest
/// mapped ancestor; when that ancestor is not inside a chain (`0xfffe`), both skeletons have a
/// chain between them, not both of two bones, and no inner bone of B's chain is mapped, the two
/// chain mappings are made (as in mode 0) and the inner bones marked `0xfffe`.
fn hierarchy_chains(e: &mut Engine, c: &MappingContext) {
    let mut bone = 0i32;
    while bone < c.count_a as i32 {
        hierarchy_chain(e, c, bone);
        bone += 1;
    }
}

/// One bone of `hierarchy_chains` (the loop counter indexes the maps as a whole word, the
/// chains and records take its low word).
fn hierarchy_chain(e: &mut Engine, c: &MappingContext, bone: i32) {
    let frame = c.frame;
    let a_map = e.mem.u32(c.a_map());
    let mapped = e.mem.u16(a_map.wrapping_add((bone as u32).wrapping_mul(2)));
    if (mapped as i16) < 0 {
        return;
    }
    let parents = e.mem.u32(c.skeleton_a + 0xc);
    let mut parent = e
        .mem
        .u16(parents.wrapping_add((bone as u32).wrapping_mul(2)));
    if parent == 0xffff {
        return;
    }
    loop {
        let slot = (signed(parent) as u32).wrapping_mul(2);
        if e.mem.u16(a_map.wrapping_add(slot)) != 0xffff {
            break;
        }
        parent = e.mem.u16(parents.wrapping_add(slot));
        if parent == 0xffff {
            return;
        }
    }
    // Never taken: the walk above only stops on a bone.
    if parent == 0xffff {
        return;
    }
    let mapped_parent = e
        .mem
        .u16(a_map.wrapping_add((signed(parent) as u32).wrapping_mul(2)));
    if mapped_parent == 0xfffe {
        return;
    }
    let (chain_a, chain_b) = (frame + 0x1c, frame + 0xc);
    clear_chain(e, chain_a);
    clear_chain(e, chain_b);
    let own = bone as u16;
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2b,
            c.skeleton_a,
            u32::from(parent),
            u32::from(own),
            chain_a
        ],
    );
    e.call(
        GET_BONE_CHAIN,
        &args![
            frame + 0x2a,
            c.skeleton_b,
            u32::from(mapped_parent),
            u32::from(mapped),
            chain_b
        ],
    );
    if e.mem.u8(frame + 0x2b) == 0 || e.mem.u8(frame + 0x2a) == 0 {
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    let size_a = e.mem.i32(chain_a + 4);
    let size_b = e.mem.i32(chain_b + 4);
    if size_a == 2 && size_b == 2 {
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    let inner_b = size_b - 1;
    let b_map = e.mem.u32(c.b_map());
    let mut unmapped = true;
    let mut index = 1;
    while index < inner_b {
        let inner = bone_at(e, chain_b, index);
        let slot = (signed(inner) as u32).wrapping_mul(2);
        if e.mem.u16(b_map.wrapping_add(slot)) != 0xffff {
            unmapped = false;
        }
        index += 1;
    }
    if !unmapped {
        free_chain(e, chain_b);
        free_chain(e, chain_a);
        return;
    }
    let pieces = [
        ReportPiece::Text(TEXT_CREATING_CHAIN),
        ReportPiece::Bone(bone_record(e, c.skeleton_a, parent)),
        ReportPiece::Text(TEXT_DASH),
        ReportPiece::Bone(bone_record(e, c.skeleton_a, own)),
    ];
    mapping_report(e, frame, frame + 0x180, &pieces, 0, u32::MAX, 0x27d);
    let mapped_model = transform_of(c.model_b, mapped);
    let mapped_parent_model = transform_of(c.model_b, mapped_parent);
    let parent_model = transform_of(c.model_a, parent);
    let own_model = transform_of(c.model_a, own);
    let entry = frame + 0x1f0;
    fn_00ce5430(e, Ptr::new(entry));
    e.mem.set_u16(entry, parent);
    e.mem.set_u16(entry + 2, own);
    e.mem.set_u16(entry + 4, mapped_parent);
    e.mem.set_u16(entry + 6, mapped);
    inverse_compose(e, own_model, mapped_model, entry + 0x40);
    inverse_compose(e, parent_model, mapped_parent_model, entry + 0x10);
    push_chain_mapping(e, c.first + 0x14, entry);
    fn_00ce5430(e, Ptr::new(entry));
    e.mem.set_u16(entry, mapped_parent);
    e.mem.set_u16(entry + 6, own);
    e.mem.set_u16(entry + 2, mapped);
    e.mem.set_u16(entry + 4, parent);
    inverse_compose(e, mapped_model, own_model, entry + 0x40);
    inverse_compose(e, mapped_parent_model, parent_model, entry + 0x10);
    push_chain_mapping(e, c.second + 0x14, entry);
    let inner_a = size_a - 1;
    let mut index = 1;
    while index < inner_a {
        let inner = bone_at(e, chain_a, index);
        set_bone_at(e, c.a_map(), signed(inner), 0xfffe);
        index += 1;
    }
    let mut index = 1;
    while index < inner_b {
        let inner = bone_at(e, chain_b, index);
        set_bone_at(e, c.b_map(), signed(inner), 0xfffe);
        index += 1;
    }
    free_chain(e, chain_b);
    free_chain(e, chain_a);
}

// Translated from 00ce61c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::createMapping` (Xbox PDB name per the engine map; cdecl `(params,
/// mapping_a, mapping_b)`): fills the two mapper data blocks from the parameters.
///
/// Opens with the sink's slot `0x1c` (`54e4323e`, the function's name) and copies the
/// parameters (`Params::Params`). In mode 0, when A has more bones than B, the copy swaps the
/// skeletons and the two simple-mapping name lists and the outputs swap (the first output then
/// maps B onto A). Both skeletons' reference poses go to model space in blocks taken from the
/// memory router's stack (`00cddcf0`); mode 0 maps those, the other modes the reference poses.
/// Two bone maps (one `u16` per bone, `0xffff` unmapped) are kept, each output gets the mode and
/// both skeletons (reference counted). Then, in order: the simple mappings named by the two
/// lists (`computeChainMapping`; tolerance and scale from the parameters for A to B, `010718c0`
/// and 1.0 for B to A; missing bones reported `abba1c64`); the chains (mode 0: name pairs, the
/// other modes: groups of four names); when the byte `+0x38` is set, the automatic mappings;
/// when `+0x39` is set, the chains from the hierarchy; the unmapped bones of B go to the first
/// output's list, those of A to the second's. Both outputs get the byte `+0x60` set and the
/// identity at `+0x30` (always the outputs as passed); mode 1 then has `00ce55d0` fill their
/// scales. Ends with the sink's slot `0x20`, gives the stack blocks back and destroys the copy.
/// The reports name the bones, ids and lines as the exe does.
pub fn hka_skeleton_mapper_utils_create_mapping(
    e: &mut Engine,
    params: Ptr,
    mapping_a: Ptr,
    mapping_b: Ptr,
) {
    let (source, out_a, out_b) = (params.addr(), mapping_a.addr(), mapping_b.addr());
    e.with_stack(CREATE_FRAME_SIZE, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        create_mapping_in_frame(e, frame, source, out_a, out_b);
    });
}

/// The body of `createMapping` in its frame (aligned base `frame`).
fn create_mapping_in_frame(e: &mut Engine, frame: u32, source: u32, out_a: u32, out_b: u32) {
    let sink = e.global::<u32>(REPORT_SINK);
    e.vcall(sink, 0x1c, &args![SCOPE_ID, SCOPE_NAME]);
    let local = frame + 0xa8;
    hka_skeleton_mapper_utils_params_params(e, Ptr::new(local), Ptr::new(source));
    let swap = e.mem.u32(local + 0x3c) == 0 && {
        let (skeleton_a, skeleton_b) = (e.mem.u32(source), e.mem.u32(source + 4));
        e.mem.i32(skeleton_a + 0x1c) > e.mem.i32(skeleton_b + 0x1c)
    };
    let (first, second) = if swap {
        let (skeleton_a, skeleton_b) = (e.mem.u32(source), e.mem.u32(source + 4));
        e.mem.set_u32(local, skeleton_b);
        e.mem.set_u32(local + 4, skeleton_a);
        copy_pair_array(e, local + 0x20, source + 0x2c);
        copy_pair_array(e, local + 0x2c, source + 0x20);
        (out_b, out_a)
    } else {
        (out_a, out_b)
    };
    let skeleton_a = e.mem.u32(local);
    let skeleton_b = e.mem.u32(local + 4);
    let count_a = e.mem.u32(skeleton_a + 0x1c);
    let count_b = e.mem.u32(skeleton_b + 0x1c);
    let model_a = router_stack_take(e, count_a.wrapping_mul(0x30));
    let capacity_a = count_a | NOT_OWNED;
    let (parents, pose) = (e.mem.u32(skeleton_a + 0xc), e.mem.u32(skeleton_a + 0x24));
    e.call(
        LOCAL_POSE_TO_MODEL_POSE,
        &args![count_a, parents, pose, model_a],
    );
    let model_b = router_stack_take(e, count_b.wrapping_mul(0x30));
    let capacity_b = count_b | NOT_OWNED;
    let (parents, pose) = (e.mem.u32(skeleton_b + 0xc), e.mem.u32(skeleton_b + 0x24));
    e.call(
        LOCAL_POSE_TO_MODEL_POSE,
        &args![count_b, parents, pose, model_b],
    );
    let mode = e.mem.u32(local + 0x3c);
    let (pose_a, pose_b) = if mode == 0 {
        (model_a, model_b)
    } else {
        (e.mem.u32(skeleton_a + 0x24), e.mem.u32(skeleton_b + 0x24))
    };
    let top_a = stack_bone_map(e, frame + 0x80, count_a, capacity_a);
    let top_b = stack_bone_map(e, frame + 0x90, count_b, capacity_b);
    e.mem.set_u32(first + 0x64, mode);
    e.mem.set_u32(second + 0x64, mode);
    set_skeleton(e, first, skeleton_a);
    set_skeleton(e, first + 4, skeleton_b);
    set_skeleton(e, second, skeleton_b);
    set_skeleton(e, second + 4, skeleton_a);
    let c = MappingContext {
        frame,
        local,
        mode,
        skeleton_a,
        skeleton_b,
        first,
        second,
        model_a,
        model_b,
        pose_a,
        pose_b,
        count_a,
        count_b,
    };
    named_simple_mappings(e, &c);
    if mode == 0 {
        named_chains(e, &c);
    } else {
        quad_chains(e, &c);
    }
    if e.mem.u8(local + 0x38) != 0 {
        automatic_mappings(e, &c);
    }
    if e.mem.u8(local + 0x39) != 0 {
        hierarchy_chains(e, &c);
    }
    let mut bone = 0i32;
    while bone < count_b as i32 {
        if bone_at(e, c.b_map(), bone) == 0xffff {
            push_bone(e, first + 0x20, bone as u32);
        }
        bone += 1;
    }
    let mut bone = 0i32;
    while bone < count_a as i32 {
        if bone_at(e, c.a_map(), bone) == 0xffff {
            push_bone(e, second + 0x20, bone as u32);
        }
        bone += 1;
    }
    e.mem.set_u8(first + 0x60, 1);
    e.mem.set_u8(second + 0x60, 1);
    store_identity_transform(e, out_a + 0x30);
    store_identity_transform(e, out_b + 0x30);
    if mode == 1 {
        fn_00ce55d0(
            e,
            Ptr::new(local + 0x40),
            1,
            Ptr::new(model_a),
            count_a,
            Ptr::new(model_b),
            count_b,
            Ptr::new(out_a + 0x30),
            Ptr::new(out_b + 0x30),
        );
    }
    let sink = e.global::<u32>(REPORT_SINK);
    e.vcall(sink, 0x20, &args![]);
    release_bone_map(e, frame + 0x90, top_b);
    release_bone_map(e, frame + 0x80, top_a);
    release_transforms(e, model_b, capacity_b);
    release_transforms(e, model_a, capacity_a);
    e.call(PARAMS_DESTRUCT, &args![local]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_5430, fn_00ce5430(Ptr) -> Ptr),
        entry!(
            0x00ce_55d0,
            fn_00ce55d0(Ptr, u32, Ptr, u32, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00ce_5a70,
            hka_skeleton_mapper_utils_compute_mapping_transform(u32, Ptr, Ptr, Ptr, f32, f32)
        ),
        entry!(0x00ce_5e60, fn_00ce5e60(Ptr, Ptr, i16, i16, Ptr, Ptr)),
        entry!(
            0x00ce_5f90,
            hka_skeleton_mapper_utils_compute_chain_mapping(u32, Ptr, i16, Ptr, i16, Ptr, f32, f32)
        ),
        entry!(
            0x00ce_5fe0,
            fn_00ce5fe0(u32, Ptr, Ptr, u32, u32, Ptr, Ptr, u32, u32, Ptr)
        ),
        entry!(0x00ce_9580, hka_skeleton_mapper_utils_params_params(Ptr, Ptr) -> Ptr),
        entry!(
            0x00ce_61c0,
            hka_skeleton_mapper_utils_create_mapping(Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINK_SLOT: u32 = 0x0900_00c0;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_bits(&mut e, IDENTITY_QUATERNION, [0, 0, 0, 0x3f80_0000]);
        e.mem.set_f32(FLOAT_ONE, 1.0);
        store_vec(&mut e, ROTATION_BIAS, [-0.5; 4]);
        store_vec(&mut e, ONES, [1.0; 4]);
        store_bits(&mut e, SIGN_MASK, [0x8000_0000; 4]);
        store_bits(
            &mut e,
            CONJUGATE_MASK,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_bits(&mut e, XYZ_MASK, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_u32(CHAIN_TOLERANCE, 0x3727_c5ac);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The report builder and the sink at `01268360` as doubles.
    fn install_report(e: &mut Engine) {
        for address in [
            REPORT_BUILDER_CONSTRUCT,
            REPORT_BUILDER_TEXT,
            REPORT_BUILDER_DESTRUCT,
            SINK_SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SINK_SLOT);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
    }

    /// The reports sent to the sink, as `(id, line)`.
    fn reports(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, SINK_SLOT)
            .into_iter()
            .map(|args| {
                assert_eq!(args[1], 1);
                assert_eq!(args[4], REPORT_FILE);
                (args[2], args[5])
            })
            .collect()
    }

    /// The validity check answers `valid` through the byte it is given.
    fn install_validity(e: &mut Engine, valid: bool) {
        if valid {
            e.register(MAPPING_IS_VALID, |e, a| {
                e.mem.set_u8(a[1], 1);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        } else {
            e.register(MAPPING_IS_VALID, |e, a| {
                e.mem.set_u8(a[1], 0);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
    }

    /// `00c74ac0` and `00c66320` write their first operand into `this`.
    fn install_quaternion_doubles(e: &mut Engine) {
        e.register(QUATERNION_SET_MUL, |e, a| {
            let left = load_vec(e, a[1]);
            store_vec(e, a[0], left);
            Ret::default()
        });
        e.register(QUATERNION_SET_MUL_2, |e, a| {
            let left = load_vec(e, a[1]);
            store_vec(e, a[0], left);
            Ret::default()
        });
    }

    fn assert_identity(e: &Engine, at: u32) {
        assert_eq!(load_bits(e, at), [0; 4]);
        assert_eq!(load_bits(e, at + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_bits(e, at + 0x20), [0x3f80_0000; 4]);
    }

    fn assert_close(got: Vec4, want: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() <= tolerance,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    #[test]
    fn fn_00ce5430_makes_an_empty_chain_mapping() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        for word in 0..0x1c {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00ce_5430, &args![this]).u32(), this);
        for offset in [0, 2, 4, 6] {
            assert_eq!(e.mem.u16(this + offset), 0);
        }
        assert_identity(&e, this + 0x10);
        assert_identity(&e, this + 0x40);
    }

    #[test]
    fn fn_00ce55d0_scales_by_the_lengths_of_each_mode() {
        let mut e = engine();
        e.register(BONE_POINTS, |e, a| {
            let first = load_vec(e, a[0]);
            let second = load_vec(e, a[0] + 0x10);
            store_vec(e, a[2], first);
            store_vec(e, a[3], second);
            Ret::default()
        });
        let params = e.mem.alloc(0x14);
        e.mem.set_f32(params + 0xc, 2.0);
        e.mem.set_f32(params + 0x10, 4.0);
        let bones_a = e.mem.alloc(0x100);
        let bones_b = e.mem.alloc(0x100);
        let out_a = e.mem.alloc(0x30);
        let out_b = e.mem.alloc(0x30);
        // Mode 0: the two points of A are 5 apart, the two of B are 10 apart.
        store_vec(&mut e, bones_a, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, bones_a + 0x10, [4.0, 6.0, 3.0, 0.0]);
        store_vec(&mut e, bones_b, [0.0; 4]);
        store_vec(&mut e, bones_b + 0x10, [0.0, 0.0, 10.0, 0.0]);
        e.mem.set_u32(params, 0);
        let words = args![params, 0x1111u32, bones_a, 7u32, bones_b, 9u32, out_a, out_b];
        e.call(0x00ce_55d0, &words);
        let points = calls(&e, BONE_POINTS);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0][..2], [bones_a, 7]);
        assert_eq!(points[1][..2], [bones_b, 9]);
        assert_eq!(load_bits(&e, out_a), [0; 4]);
        assert_eq!(load_bits(&e, out_a + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(&e, out_a + 0x20), [4.0; 4]);
        assert_eq!(load_bits(&e, out_b), [0; 4]);
        assert_eq!(load_bits(&e, out_b + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(&e, out_b + 0x20), [0.25; 4]);
        // Mode 1: bone 1 of A (translation length 5) and bone 0 of B (length 10).
        e.mem.set_u32(params, 1);
        e.mem.set_u32(params + 4, 1);
        e.mem.set_u32(params + 8, 0);
        store_vec(&mut e, bones_a + 0x30, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, bones_b, [0.0, 0.0, 10.0, 0.0]);
        e.call(0x00ce_55d0, &words);
        assert_identity_with_scale(&e, out_a, 4.0);
        assert_identity_with_scale(&e, out_b, 0.25);
        // Any other mode: the two floats alone.
        e.mem.set_u32(params, 5);
        e.call(0x00ce_55d0, &words);
        assert_identity_with_scale(&e, out_a, 2.0);
        assert_identity_with_scale(&e, out_b, 0.5);
        assert_eq!(calls(&e, BONE_POINTS).len(), 2);
    }

    fn assert_identity_with_scale(e: &Engine, at: u32, scale: f32) {
        assert_eq!(load_bits(e, at), [0; 4]);
        assert_eq!(load_bits(e, at + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(e, at + 0x20), [scale; 4]);
    }

    /// Two transforms: `a` with translation (1, 2, 3), `b` with (5, 6, 7), both with the identity
    /// rotation; `a` scaled (2, 4, 8), `b` by one.
    fn two_transforms(e: &mut Engine) -> (u32, u32, u32) {
        let a = e.mem.alloc(0x30);
        let b = e.mem.alloc(0x30);
        let out = e.mem.alloc(0x30);
        for at in [a, b] {
            store_bits(e, at + 0x10, [0, 0, 0, 0x3f80_0000]);
        }
        store_vec(e, a, [1.0, 2.0, 3.0, 0.0]);
        store_vec(e, a + 0x20, [2.0, 4.0, 8.0, 1.0]);
        store_vec(e, b, [5.0, 6.0, 7.0, 0.0]);
        store_vec(e, b + 0x20, [1.0; 4]);
        (a, b, out)
    }

    #[test]
    fn compute_mapping_transform_mode_zero_inverts_the_first_transform() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        e.call(0x00ce_5a70, &args![0u32, a, b, out, 1.0e-5f32, 1.0f32]);
        // The translation is rotate(conjugate, b.t) - rotate(a.rotation, a.t) = (5, 6, 7) - (1, 2, 3).
        assert_close(load_vec(&e, out), [4.0, 4.0, 4.0, 0.0], 0.0);
        // The rotation is what `00c66320` made of the conjugate.
        let set_mul = calls(&e, QUATERNION_SET_MUL);
        assert_eq!(set_mul.len(), 1);
        assert_eq!(set_mul[0][0], out + 0x10);
        assert_eq!(set_mul[0][2], b + 0x10);
        assert_eq!(
            load_bits(&e, out + 0x10),
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0x3f80_0000]
        );
        // The scale is b.scale / a.scale (reciprocal estimate with one Newton step), lane 3 zero.
        let scale = load_vec(&e, out + 0x20);
        assert_close(scale, [0.5, 0.25, 0.125, 0.0], 1.0e-6);
        assert_eq!(scale[3].to_bits(), 0);
        assert!(reports(&e).is_empty());
        assert_eq!(calls(&e, MAPPING_IS_VALID).len(), 1);
        assert_eq!(calls(&e, MAPPING_IS_VALID)[0][0], out);
    }

    #[test]
    fn compute_mapping_transform_mode_one_scales_by_the_lengths_and_reports() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        store_vec(&mut e, a, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, b, [0.0, 0.0, 10.0, 0.0]);
        // Lengths 5 and 10: scale 2, translation b - 2 a.
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(load_vec(&e, out), [-6.0, -8.0, 10.0, 0.0]);
        let set_mul = calls(&e, QUATERNION_SET_MUL_2);
        assert_eq!(set_mul, vec![vec![out + 0x10, b + 0x10, a + 0x10]]);
        assert!(reports(&e).is_empty());
        // A zero length against a non-zero one is reported and the fallback scale is used.
        store_vec(&mut e, a, [0.0; 4]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [7.0; 4]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 10.0, 0.0]);
        assert_eq!(reports(&e), vec![(0xabba_2b94, 0x35c)]);
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[0][1], MESSAGE_ZERO_LENGTH);
        // Two zero lengths use the fallback without a report.
        store_vec(&mut e, b, [0.0; 4]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [7.0; 4]);
        assert_eq!(reports(&e).len(), 1);
        // A negative tolerance is reported, the work goes on with it (no length is small).
        store_vec(&mut e, a, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, b, [0.0, 0.0, 10.0, 0.0]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, -1.0f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(reports(&e)[1], (0xabba_39c3, 0x34e));
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[1][1], MESSAGE_TOLERANCE);
        // NaN tolerance: no report, no length is small.
        e.call(0x00ce_5a70, &args![1u32, a, b, out, f32::NAN, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(reports(&e).len(), 2);
    }

    #[test]
    fn compute_mapping_transform_reports_other_modes_and_invalid_results() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        e.mem.set_u32(out, 0x1234_5678);
        e.call(0x00ce_5a70, &args![2u32, a, b, out, 1.0e-5f32, 1.0f32]);
        assert_eq!(e.mem.u32(out), 0x1234_5678);
        assert_eq!(reports(&e), vec![(0xabba_ef91, 0x36c)]);
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[0][1], MESSAGE_UNSUPPORTED);
        // A mapping the validity check refuses is reported after the work.
        install_validity(&mut e, false);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 1.0f32]);
        assert_eq!(reports(&e)[1], (0xabba_10fc, 0x371));
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[1][1], MESSAGE_INVALID);
        assert_eq!(calls(&e, REPORT_BUILDER_CONSTRUCT)[1][2], 0x200);
    }

    /// Parent indices `parents` as the `i16` array a skeleton points to at `+0xC`.
    fn make_skeleton(e: &mut Engine, parents: &[i16]) -> u32 {
        let data = e.mem.alloc(2 * parents.len() as u32);
        for (index, parent) in parents.iter().enumerate() {
            e.mem.set_i16(data + 2 * index as u32, *parent);
        }
        let skeleton = e.mem.alloc(0x10);
        e.mem.set_u32(skeleton + 0xc, data);
        skeleton
    }

    /// Sets bone `index` of the transforms at `bones`.
    fn set_bone(e: &mut Engine, bones: u32, index: u32, translation: Vec4, rotation: Vec4) {
        store_vec(e, bones + 0x30 * index, translation);
        store_vec(e, bones + 0x30 * index + 0x10, rotation);
        store_vec(e, bones + 0x30 * index + 0x20, [1.0; 4]);
    }

    #[test]
    fn fn_00ce5e60_sums_the_chain_up_to_the_stop_bone() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[-1, 0, 1]);
        let bones = e.mem.alloc(0x90);
        let identity = [0.0, 0.0, 0.0, 1.0];
        set_bone(&mut e, bones, 0, [9.0, 9.0, 9.0, 0.0], identity);
        set_bone(&mut e, bones, 1, [0.0, 2.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones, 2, [1.0, 0.0, 0.0, 0.0], identity);
        let length = e.mem.alloc(4);
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(length, 5.0);
        store_vec(&mut e, vector, [8.0; 4]);
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, 2i16, length, vector],
        );
        assert_eq!(e.mem.f32(length), 3.0);
        assert_eq!(load_vec(&e, vector), [1.0, 2.0, 0.0, 0.0]);
        // A start at or below the stop bone leaves zeros.
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 2i16, 2i16, length, vector],
        );
        assert_eq!(e.mem.f32(length), 0.0);
        assert_eq!(load_bits(&e, vector), [0; 4]);
        // The stop bone is compared as a signed 16-bit value: a start of -1 never runs.
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, -1i16, length, vector],
        );
        assert_eq!(load_bits(&e, vector), [0; 4]);
        // A bone rotated about z (quaternion (0, 0, 0.6, 0.8)) turns the vector gathered so far.
        set_bone(&mut e, bones, 1, [0.0; 4], [0.0, 0.0, 0.6, 0.8]);
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, 2i16, length, vector],
        );
        assert_close(load_vec(&e, vector), [0.28, 0.96, 0.0, 0.0], 1.0e-6);
        assert_eq!(e.mem.f32(length), 1.0);
    }

    #[test]
    fn compute_chain_mapping_picks_the_bones_by_signed_index() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let bones_a = e.mem.alloc(0x90);
        let bones_b = e.mem.alloc(0x90);
        let identity = [0.0, 0.0, 0.0, 1.0];
        set_bone(&mut e, bones_a, 1, [3.0, 4.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_b, 0, [0.0, 0.0, 10.0, 0.0], identity);
        let out = e.mem.alloc(0x30);
        // Bone -1 of the array that starts one bone later is its first bone.
        e.call(
            0x00ce_5f90,
            &args![
                1u32,
                bones_a,
                1i16,
                bones_b + 0x30,
                -1i16,
                out,
                1.0e-5f32,
                7.0f32
            ],
        );
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(load_vec(&e, out), [-6.0, -8.0, 10.0, 0.0]);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL_2),
            vec![vec![out + 0x10, bones_b + 0x10, bones_a + 0x30 + 0x10]]
        );
    }

    #[test]
    fn fn_00ce5fe0_fills_a_chain_mapping() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let skeleton_a = make_skeleton(&mut e, &[-1, 0, 1]);
        let skeleton_b = make_skeleton(&mut e, &[-1, 0]);
        let bones_a = e.mem.alloc(0x90);
        let bones_b = e.mem.alloc(0x60);
        set_bone(&mut e, bones_a, 0, [3.0, 4.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_a, 1, [0.0, 3.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_a, 2, [4.0, 0.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_b, 0, [0.0, 0.0, 10.0, 0.0], identity);
        set_bone(&mut e, bones_b, 1, [0.0, 6.0, 0.0, 0.0], identity);
        let chains = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&chains);
        e.register_double(ROTATION_BETWEEN, move |e, a| {
            seen.borrow_mut()
                .push((load_vec(e, a[1]), load_vec(e, a[2])));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register(QUATERNION_SET_MUL_3, |e, a| {
            store_vec(e, a[0], [9.0; 4]);
            Ret::default()
        });
        let out = e.mem.alloc(0x70);
        e.call(
            0x00ce_5fe0,
            &args![
                1u32,
                skeleton_a,
                bones_a,
                0x0001_0000u32,
                2u32,
                skeleton_b,
                bones_b,
                0xffff_0000u32,
                1u32,
                out
            ],
        );
        // The bone indices are the low words of the arguments.
        for (offset, want) in [(0, 0), (2, 2), (4, 0), (6, 1)] {
            assert_eq!(e.mem.u16(out + offset), want);
        }
        // The start bones give scale 10 / 5 (the chains of 0x10000 and 0xffff0000 are bones 0).
        assert_eq!(load_vec(&e, out + 0x30), [2.0; 4]);
        // The chain vectors (4, 3, 0) and (0, 6, 0), normalised, in the order (B, A).
        let chains = chains.borrow();
        assert_eq!(chains.len(), 1);
        assert_close(chains[0].0, [0.0, 1.0, 0.0, 0.0], 1.0e-5);
        assert_close(chains[0].1, [0.8, 0.6, 0.0, 0.0], 1.0e-5);
        assert_eq!(load_vec(&e, out + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, out + 0x50), [9.0; 4]);
        let reciprocal = (1.0f64 / 7.0f64) as f32;
        assert_eq!(load_bits(&e, out + 0x60), [reciprocal.to_bits(); 4]);
        let set_mul = calls(&e, QUATERNION_SET_MUL_3);
        assert_eq!(
            set_mul,
            vec![vec![
                out + 0x50,
                bones_a + 0x60 + 0x10,
                bones_b + 0x30 + 0x10
            ]]
        );
    }

    #[test]
    fn params_copy_constructor_copies_the_arrays() {
        let mut e = engine();
        let router_block = e.mem.alloc(0x20);
        e.mem.set_u32(router_block + 0x10, 0x00a1_1000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router_block,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(8 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source = e.mem.alloc(0x54);
        for word in 0..0x15 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(source + 0xc, 0x7f80_0001);
        e.mem.set_u32(source + 0x10, 0x3fc0_0000);
        e.mem.set_u8(source + 0x38, 1);
        e.mem.set_u8(source + 0x39, 2);
        // Array 1: two pairs; array 2: empty; array 3: one pair.
        let first = e.mem.alloc(16);
        let third = e.mem.alloc(8);
        for (index, value) in [11u32, 12, 13, 14].into_iter().enumerate() {
            e.mem.set_u32(first + 4 * index as u32, value);
        }
        e.mem.set_u32(third, 31);
        e.mem.set_u32(third + 4, 32);
        e.mem.set_u32(source + 0x14, first);
        e.mem.set_u32(source + 0x18, 2);
        e.mem.set_u32(source + 0x1c, 2);
        e.mem.set_u32(source + 0x20, 0);
        e.mem.set_u32(source + 0x24, 0);
        e.mem.set_u32(source + 0x28, 0x8000_0000);
        e.mem.set_u32(source + 0x2c, third);
        e.mem.set_u32(source + 0x30, 1);
        e.mem.set_u32(source + 0x34, 1);
        let this = e.mem.alloc(0x54);
        let got = e.call(0x00ce_9580, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)],
            [0x100, 0x101, 0x102]
        );
        // The floats go through FLD/FSTP: the signalling NaN comes back quiet.
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0x3fc0_0000);
        assert_eq!([e.mem.u8(this + 0x38), e.mem.u8(this + 0x39)], [1, 2]);
        for offset in [0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50] {
            assert_eq!(e.mem.u32(this + offset), e.mem.u32(source + offset));
        }
        let data = e.mem.u32(this + 0x14);
        assert_ne!(data, first);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        let copied: Vec<u32> = (0..4).map(|word| e.mem.u32(data + 4 * word)).collect();
        assert_eq!(copied, vec![11, 12, 13, 14]);
        // The empty array stays empty with its flags.
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x8000_0000);
        let data = e.mem.u32(this + 0x2c);
        assert_eq!(e.mem.u32(this + 0x30), 1);
        assert_eq!([e.mem.u32(data), e.mem.u32(data + 4)], [31, 32]);
        // The arrays were reserved to their sizes through the allocator the router names.
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![0x00a1_1000, this + 0x14, 2, 8],
                vec![0x00a1_1000, this + 0x2c, 1, 8]
            ]
        );
    }

    // `createMapping` (00ce61c0).

    const BEGIN_SLOT: u32 = 0x0900_00d0;
    const END_SLOT: u32 = 0x0900_00e0;
    const DEALLOCATE: u32 = 0x0900_00f0;

    /// What the `createMapping` tests keep: the router (its stack top at `+0`) and the stack
    /// top it starts with.
    struct MapperRig {
        router: u32,
        stack: u32,
    }

    /// `_reserve` stand-in: storage for `capacity` elements of `size` bytes, the old elements
    /// copied, the capacity word owned.
    fn grow(e: &mut Engine, array: u32, capacity: u32, size: u32) {
        let data = e.mem.alloc(capacity * size);
        let old = e.mem.u32(array);
        for byte in 0..e.mem.u32(array + 4) * size {
            let value = e.mem.u8(old + byte);
            e.mem.set_u8(data + byte, value);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Every callee of `createMapping` outside this file as a double: the sink (slots `0xc`,
    /// `0x1c`, `0x20`), the report builder (its text and bone writers answer the builder), the
    /// router with a stack block and an allocator (slot `0x10` frees), the array growers, a name
    /// search by pointer, a chain walk up the parents, the pose conversion as a copy, the chain
    /// mapping copy and the quaternion helpers.
    fn mapper_engine() -> (Engine, MapperRig) {
        let mut e = engine();
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        e.mem.set_f32(NEAREST_START, 100_000.0);
        for address in [
            SINK_SLOT,
            BEGIN_SLOT,
            END_SLOT,
            DEALLOCATE,
            REPORT_BUILDER_CONSTRUCT,
            REPORT_BUILDER_DESTRUCT,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            PARAMS_DESTRUCT,
            ROTATION_BETWEEN,
            QUATERNION_SET_MUL_3,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        for address in [REPORT_BUILDER_TEXT, REPORT_BUILDER_BONE] {
            e.register(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        let vtable = e.mem.alloc(0x24);
        e.mem.set_u32(vtable + 0xc, SINK_SLOT);
        e.mem.set_u32(vtable + 0x1c, BEGIN_SLOT);
        e.mem.set_u32(vtable + 0x20, END_SLOT);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
        let allocator_vtable = e.mem.alloc(0x14);
        e.mem.set_u32(allocator_vtable + 0x10, DEALLOCATE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, allocator_vtable);
        let stack = e.mem.alloc(0x4000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            grow(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = e.mem.u32(a[1] + 8) & CAPACITY_BITS;
            let wanted = if capacity == 0 { 4 } else { capacity * 2 };
            grow(e, a[1], wanted, a[2]);
            Ret::default()
        });
        e.register(FIND_BONE_WITH_NAME, |e, a| {
            let bones = e.mem.u32(a[0] + 0x18);
            let count = e.mem.u32(a[0] + 0x1c);
            let found = (0..count).find(|&bone| e.mem.u32(bones + 8 * bone) == a[1]);
            Ret {
                eax: found.unwrap_or(0xffff),
                ..Ret::default()
            }
        });
        e.register(GET_BONE_CHAIN, |e, a| {
            let (start, parents) = (a[2] as u16 as i16, e.mem.u32(a[1] + 0xc));
            let mut chain = vec![];
            let mut bone = a[3] as u16 as i16;
            while bone >= 0 && bone != start {
                chain.push(bone);
                bone = e.mem.i16(parents + 2 * bone as u32);
            }
            let reached = bone == start;
            if reached {
                chain.push(start);
                chain.reverse();
                let data = e.mem.alloc(2 * chain.len() as u32);
                for (index, bone) in chain.iter().enumerate() {
                    e.mem.set_i16(data + 2 * index as u32, *bone);
                }
                e.mem.set_u32(a[4], data);
                e.mem.set_u32(a[4] + 4, chain.len() as u32);
                e.mem.set_u32(a[4] + 8, chain.len() as u32);
            }
            e.mem.set_u8(a[0], u8::from(reached));
            Ret::default()
        });
        e.register(LOCAL_POSE_TO_MODEL_POSE, |e, a| {
            for word in 0..a[0] * 12 {
                let value = e.mem.u32(a[2] + 4 * word);
                e.mem.set_u32(a[3] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(CHAIN_MAPPING_COPY, |e, a| {
            for word in 0..0x1c {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        (e, MapperRig { router, stack })
    }

    fn bone_name(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(16);
        e.mem.set_cstr(at, text);
        at
    }

    /// A skeleton with the given parents, bone names and reference translations (identity
    /// rotations, unit scales).
    fn skeleton(e: &mut Engine, parents: &[i16], names: &[u32], positions: &[Vec4]) -> u32 {
        let count = parents.len() as u32;
        let at = e.mem.alloc(0x30);
        let parent_list = e.mem.alloc(2 * count);
        let records = e.mem.alloc(8 * count);
        let pose = e.mem.alloc(0x30 * count);
        for bone in 0..count {
            let index = bone as usize;
            e.mem.set_i16(parent_list + 2 * bone, parents[index]);
            e.mem.set_u32(records + 8 * bone, names[index]);
            store_vec(e, pose + 0x30 * bone, positions[index]);
            store_vec(e, pose + 0x30 * bone + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(e, pose + 0x30 * bone + 0x20, [1.0; 4]);
        }
        e.mem.set_u32(at + 0xc, parent_list);
        e.mem.set_u32(at + 0x18, records);
        e.mem.set_u32(at + 0x1c, count);
        e.mem.set_u32(at + 0x24, pose);
        at
    }

    /// Parameters for two skeletons: no compare function, tolerance 0.1, scale 1.0, the three
    /// name lists empty, both bytes clear, the given mode.
    fn mapper_params(e: &mut Engine, skeleton_a: u32, skeleton_b: u32, mode: u32) -> u32 {
        let at = e.mem.alloc(0x54);
        e.mem.set_u32(at, skeleton_a);
        e.mem.set_u32(at + 4, skeleton_b);
        e.mem.set_f32(at + 0xc, 0.1);
        e.mem.set_f32(at + 0x10, 1.0);
        for list in [0x14, 0x20, 0x2c] {
            e.mem.set_u32(at + list + 8, NOT_OWNED);
        }
        e.mem.set_u32(at + 0x3c, mode);
        at
    }

    /// Sets the name list at `params + list` to `names` (two per 8-byte element).
    fn name_list(e: &mut Engine, params: u32, list: u32, names: &[u32]) {
        let data = e.mem.alloc(4 * names.len() as u32);
        for (index, name) in names.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *name);
        }
        let count = names.len() as u32 / 2;
        e.mem.set_u32(params + list, data);
        e.mem.set_u32(params + list + 4, count);
        e.mem.set_u32(params + list + 8, count);
    }

    fn mapper_output(e: &mut Engine) -> u32 {
        let at = e.mem.alloc(0x68);
        for list in [8, 0x14, 0x20] {
            e.mem.set_u32(at + list + 8, NOT_OWNED);
        }
        at
    }

    /// The `u16` list at `output + 0x20` and the first two words of each element of the arrays
    /// at `+8` (0x40 bytes each) and `+0x14` (0x70 bytes each, four words).
    fn unmapped(e: &Engine, output: u32) -> Vec<u16> {
        let data = e.mem.u32(output + 0x20);
        (0..e.mem.u32(output + 0x24))
            .map(|index| e.mem.u16(data + 2 * index))
            .collect()
    }

    fn simple_bones(e: &Engine, output: u32) -> Vec<[u16; 2]> {
        let data = e.mem.u32(output + 8);
        (0..e.mem.u32(output + 0xc))
            .map(|index| {
                let at = data + 0x40 * index;
                [e.mem.u16(at), e.mem.u16(at + 2)]
            })
            .collect()
    }

    fn chain_bones(e: &Engine, output: u32) -> Vec<[u16; 4]> {
        let data = e.mem.u32(output + 0x14);
        (0..e.mem.u32(output + 0x18))
            .map(|index| [0, 2, 4, 6].map(|offset| e.mem.u16(data + 0x70 * index + offset)))
            .collect()
    }

    /// The reports sent to the sink as `(level, id, line)`.
    fn mapper_reports(e: &Engine) -> Vec<(u32, u32, u32)> {
        calls(e, SINK_SLOT)
            .into_iter()
            .map(|args| {
                assert_eq!(args[4], REPORT_FILE);
                (args[1], args[2], args[5])
            })
            .collect()
    }

    fn create(e: &mut Engine, params: u32, out_a: u32, out_b: u32) {
        e.call(0x00ce_61c0, &args![params, out_a, out_b]);
    }

    /// What every run ends with: the scope closed, the stack top given back, the copy of the
    /// parameters destroyed, both outputs marked and given the identity at `+0x30`.
    fn assert_finished(e: &Engine, rig: &MapperRig, outputs: [u32; 2], mode: u32) {
        assert_eq!(calls(e, BEGIN_SLOT).len(), 1);
        assert_eq!(calls(e, BEGIN_SLOT)[0][1..], [SCOPE_ID, SCOPE_NAME]);
        assert_eq!(calls(e, END_SLOT).len(), 1);
        assert_eq!(calls(e, PARAMS_DESTRUCT).len(), 1);
        assert_eq!(e.mem.u32(rig.router), rig.stack);
        for output in outputs {
            assert_eq!(e.mem.u8(output + 0x60), 1);
            assert_eq!(e.mem.u32(output + 0x64), mode);
        }
    }

    #[test]
    fn create_mapping_maps_named_bones_and_lists_the_rest() {
        let (mut e, rig) = mapper_engine();
        let [r, s, h, x, missing] =
            [b"r", b"s", b"h", b"x", b"y"].map(|text| bone_name(&mut e, text));
        let a = skeleton(
            &mut e,
            &[-1, 0, 1],
            &[r, s, h],
            &[[0.0; 4], [0.0, 1.0, 0.0, 0.0], [0.0, 2.0, 0.0, 0.0]],
        );
        let b = skeleton(
            &mut e,
            &[-1, 0, 1, 2],
            &[r, s, h, x],
            &[
                [0.0; 4],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 4.0, 0.0, 0.0],
                [0.0, 5.0, 0.0, 0.0],
            ],
        );
        let params = mapper_params(&mut e, a, b, 0);
        name_list(&mut e, params, 0x20, &[s, s, h, missing]);
        name_list(&mut e, params, 0x2c, &[r, r]);
        let (out_a, out_b) = (mapper_output(&mut e), mapper_output(&mut e));
        create(&mut e, params, out_a, out_b);

        assert_finished(&e, &rig, [out_a, out_b], 0);
        assert_eq!([e.mem.u32(out_a), e.mem.u32(out_a + 4)], [a, b]);
        assert_eq!([e.mem.u32(out_b), e.mem.u32(out_b + 4)], [b, a]);
        assert_eq!(
            calls(&e, ADD_REFERENCE),
            vec![vec![a], vec![b], vec![b], vec![a]]
        );
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
        // `s` maps A's bone 1 onto B's bone 1 (B's translation minus A's); `h` has no partner.
        assert_eq!(simple_bones(&e, out_a), vec![[1, 1]]);
        let mapping = e.mem.u32(out_a + 8);
        assert_eq!(load_vec(&e, mapping + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(simple_bones(&e, out_b), vec![[0, 0]]);
        assert_eq!(mapper_reports(&e), vec![(1, 0xabba_1c64, 0x7f)]);
        // B's bones not reached go to the first output, A's to the second.
        assert_eq!(unmapped(&e, out_a), vec![0, 2, 3]);
        assert_eq!(unmapped(&e, out_b), vec![1, 2]);
        assert!(chain_bones(&e, out_a).is_empty());
        assert_identity(&e, out_a + 0x30);
        assert_identity(&e, out_b + 0x30);
        assert!(calls(&e, DEALLOCATE).is_empty());
    }

    #[test]
    fn create_mapping_swaps_the_larger_skeleton_and_builds_a_chain() {
        let (mut e, rig) = mapper_engine();
        let [r, p, q, t] = [b"r", b"p", b"q", b"t"].map(|text| bone_name(&mut e, text));
        let line = |count: usize| -> Vec<Vec4> {
            (0..count)
                .map(|bone| [0.0, bone as f32, 0.0, 0.0])
                .collect()
        };
        // A has four bones in a line, B three: the skeletons and the outputs swap.
        let a = skeleton(&mut e, &[-1, 0, 1, 2], &[r, p, q, t], &line(4));
        let b = skeleton(&mut e, &[-1, 0, 1], &[r, p, t], &line(3));
        let params = mapper_params(&mut e, a, b, 0);
        name_list(&mut e, params, 0x14, &[p, t]);
        name_list(&mut e, params, 0x20, &[r, r]);
        let (out_a, out_b) = (mapper_output(&mut e), mapper_output(&mut e));
        create(&mut e, params, out_a, out_b);

        assert_finished(&e, &rig, [out_a, out_b], 0);
        assert_eq!([e.mem.u32(out_b), e.mem.u32(out_b + 4)], [b, a]);
        assert_eq!([e.mem.u32(out_a), e.mem.u32(out_a + 4)], [a, b]);
        // The A-to-B list became the second list of the swapped copy: it maps onto `out_a`.
        assert_eq!(simple_bones(&e, out_a), vec![[0, 0]]);
        assert!(simple_bones(&e, out_b).is_empty());
        // The chain p..t: bones 1..2 of B, 1..3 of A.
        assert_eq!(chain_bones(&e, out_b), vec![[1, 2, 1, 3]]);
        assert_eq!(chain_bones(&e, out_a), vec![[1, 3, 1, 2]]);
        assert_eq!(mapper_reports(&e), vec![(0, u32::MAX, 0xf9)]);
        // A's bone 2 is inside the chain; B's bones 1 and 2 are neither mapped nor inside.
        assert_eq!(unmapped(&e, out_b), vec![0, 1, 3]);
        assert_eq!(unmapped(&e, out_a), vec![1, 2]);
        // Both chains the walk made are freed.
        let frees = calls(&e, DEALLOCATE);
        assert_eq!(frees.len(), 2);
        assert_eq!(frees[0][2], 6);
        assert_eq!(frees[1][2], 4);
        assert_identity(&e, out_a + 0x30);
        assert_identity(&e, out_b + 0x30);
    }

    #[test]
    fn create_mapping_matches_by_distance_and_chains_the_hierarchy() {
        let (mut e, rig) = mapper_engine();
        let names = [b"a0", b"a1", b"a2", b"a3", b"b0", b"b1", b"b2", b"b3"]
            .map(|text| bone_name(&mut e, text));
        let a = skeleton(
            &mut e,
            &[-1, 0, 1, 2],
            &names[..4],
            &[
                [0.0; 4],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
            ],
        );
        let b = skeleton(
            &mut e,
            &[-1, 0, 1, 2],
            &names[4..],
            &[
                [0.0; 4],
                [0.0, 1.0, 0.0, 0.0],
                [5.0, 5.0, 5.0, 0.0],
                [0.0, 3.0, 0.001, 0.0],
            ],
        );
        let params = mapper_params(&mut e, a, b, 0);
        e.mem.set_u8(params + 0x38, 1);
        e.mem.set_u8(params + 0x39, 1);
        let (out_a, out_b) = (mapper_output(&mut e), mapper_output(&mut e));
        create(&mut e, params, out_a, out_b);

        assert_finished(&e, &rig, [out_a, out_b], 0);
        // No compare and a positive tolerance: the nearest bone within 0.1 (bone 2 of A is 1.0
        // from its nearest).
        assert!(calls(&e, STRING_CASE_COMPARE).is_empty());
        assert_eq!(simple_bones(&e, out_a), vec![[0, 0], [1, 1], [3, 3]]);
        assert_eq!(simple_bones(&e, out_b), vec![[0, 0], [1, 1], [3, 3]]);
        // Bone 3's nearest mapped ancestor is bone 1: one chain 1..3 in both skeletons (the
        // chain 0..1 of bone 1 has two bones on both sides and is skipped).
        assert_eq!(chain_bones(&e, out_a), vec![[1, 3, 1, 3]]);
        assert_eq!(chain_bones(&e, out_b), vec![[1, 3, 1, 3]]);
        assert_eq!(
            mapper_reports(&e),
            vec![
                (0, u32::MAX, 0x231),
                (0, u32::MAX, 0x231),
                (0, u32::MAX, 0x231),
                (0, u32::MAX, 0x27d)
            ]
        );
        assert!(unmapped(&e, out_a).is_empty());
        assert!(unmapped(&e, out_b).is_empty());
        // The pair list (four pairs of 8 bytes) and the four chains are freed.
        let frees = calls(&e, DEALLOCATE);
        assert_eq!(frees.len(), 5);
        assert_eq!(frees[0][2], 32);
    }

    #[test]
    fn create_mapping_in_mode_1_builds_quad_chains_and_scales() {
        let (mut e, rig) = mapper_engine();
        let [r, p, t] = [b"r", b"p", b"t"].map(|text| bone_name(&mut e, text));
        let positions = [[0.0; 4], [0.0, 1.0, 0.0, 0.0], [0.0, 2.0, 0.0, 0.0]];
        let a = skeleton(&mut e, &[-1, 0, 1], &[r, p, t], &positions);
        let b = skeleton(&mut e, &[-1, 0, 1], &[r, p, t], &positions);
        let params = mapper_params(&mut e, a, b, 1);
        name_list(&mut e, params, 0x14, &[p, t, p, t]);
        // The block `00ce55d0` reads: another mode, the floats 2 and 4.
        e.mem.set_u32(params + 0x40, 5);
        e.mem.set_f32(params + 0x4c, 2.0);
        e.mem.set_f32(params + 0x50, 4.0);
        let (out_a, out_b) = (mapper_output(&mut e), mapper_output(&mut e));
        create(&mut e, params, out_a, out_b);

        assert_finished(&e, &rig, [out_a, out_b], 1);
        assert_eq!(chain_bones(&e, out_a), vec![[1, 2, 1, 2]]);
        assert_eq!(chain_bones(&e, out_b), vec![[1, 2, 1, 2]]);
        assert_eq!(mapper_reports(&e), vec![(0, u32::MAX, 0x18d)]);
        // The two chain-end rotations of `00ce5fe0`, one per chain mapping.
        assert_eq!(calls(&e, QUATERNION_SET_MUL_3).len(), 2);
        assert_eq!(unmapped(&e, out_a), vec![0]);
        assert_eq!(unmapped(&e, out_b), vec![0]);
        assert_eq!(calls(&e, DEALLOCATE).len(), 2);
        assert_identity_with_scale(&e, out_a + 0x30, 2.0);
        assert_identity_with_scale(&e, out_b + 0x30, 0.5);
    }
}
