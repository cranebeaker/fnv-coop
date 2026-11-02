//! `Havok SDK/hkaragdollutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The utilities work on a rag doll's rigid bodies (`hkpRigidBody`: transform at `+0xf0`, name at
//! `+0x74`, shape at `+0x10`, motion type byte at `+0xe8`) and constraints (`hkpConstraintInstance`:
//! data `+0xc`, the two bodies `+0x14` and `+0x18`). A constraint's data object answers its type
//! (virtual slot `+0x20`); the rag doll types are 0, 1, 2, 6, 7 and 8, and the pivots sit at a
//! type-dependent offset of the data. Several functions take their inputs in registers: those come
//! first, in the order the doc comment lists them, then the stack words. Arrays are the exe's
//! `hkArray` words (data, size, capacity word whose bit 31 says the buffer is not owned); the local
//! arrays of the larger functions live on the memory router's bump stack (`00c85750`: the word at
//! `+0` is the bump pointer, `+0x10` the allocator).
//!
//! SSE code computes lane by lane in `f32`; `RCPPS` is the host CPU's estimate.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::MEMORY_ROUTER;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, refined_reciprocal, shufps, store_vec, sum3, unpckhps, zip_with, Vec4,
};

/// The report sink (the global at `01268360`): slot `0xc` takes `(level, id, text, file, line)`,
/// slot `0x10` `(id, enabled)` and slot `0x14` `(result byte address, id)`. The source file name.
const SINK: u32 = 0x0126_8360;
const SOURCE_FILE: u32 = 0x010c_bd34;
/// The text builder: constructor `00cb1600` (thiscall, `(buffer, size)`), `operator<<(const
/// char*)` `00cc01c0` and `operator<<(int)` `00cc0280` (thiscall, one argument, answering the
/// builder), `operator<<(hkStringPtr const&)` `00cc05a0` and the destructor `00cc00e0`.
const TEXT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const TEXT_BUILDER_APPEND: u32 = 0x00cc_01c0;
const TEXT_BUILDER_APPEND_INT: u32 = 0x00cc_0280;
const TEXT_BUILDER_APPEND_STRING_PTR: u32 = 0x00cc_05a0;
const TEXT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
/// The messages: "Unsupported type of constraint in rag doll" (with and without the full stop),
/// "Unusual : Aligning constraint spaces of a stiff spring.", "Expecting ", " constraints, not ",
/// ". Can't create rag doll", "Some bones have no name - can't create rag doll", "Skeleton has no
/// associated rigid bodies - can't create rag doll", "Some RBS have duplicated names : ", "Found
/// more than one constraint between rigid bodies ", " and ", "Couldn't find constraint for bone :
/// " and "Rigid Body name could not be found".
const TEXT_UNSUPPORTED: u32 = 0x010c_bd50;
const TEXT_UNSUPPORTED_STOP: u32 = 0x010c_bd7c;
const TEXT_STIFF_SPRING: u32 = 0x010c_bda8;
const TEXT_EXPECTING: u32 = 0x010c_bde0;
const TEXT_CONSTRAINTS_NOT: u32 = 0x010c_bdec;
const TEXT_CANT_CREATE: u32 = 0x010c_be00;
const TEXT_NO_BONE_NAME: u32 = 0x010c_be18;
const TEXT_NO_BODIES: u32 = 0x010c_be70;
const TEXT_DUPLICATE_BODY: u32 = 0x010c_bef0;
const TEXT_MORE_THAN_ONE: u32 = 0x010c_beb0;
const TEXT_AND: u32 = 0x010c_bee8;
const TEXT_NO_CONSTRAINT: u32 = 0x010c_be48;
const TEXT_NO_BODY_NAME: u32 = 0x010c_7384;
/// The id the report of an unsupported constraint type carries.
const ID_UNSUPPORTED: u32 = 0xabba_bf3b;
/// `hkArrayUtil` growth routines: `00c90940` (cdecl `(allocator, array, element size)`) and
/// `_reserve` `00c908c0` (cdecl `(allocator, array, capacity, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkQsTransform::isApproximatelyEqual` (`00ce5120`, thiscall `(result byte, other, tolerance)`),
/// `setFromTransformNoScale` (`00ce5070`, thiscall `(transform)`), the conversion of a
/// `hkQsTransform` to a `hkTransform` (`00ce50b0`, thiscall `(out)`), `hkQuaternion::setMul`
/// (`00c66320`, thiscall `(a, b)`) and `hkVector4::setTransformedInversePos` (`00c7f690`, thiscall
/// `(transform, position)`).
const IS_APPROXIMATELY_EQUAL: u32 = 0x00ce_5120;
const SET_FROM_TRANSFORM_NO_SCALE: u32 = 0x00ce_5070;
const QS_TO_TRANSFORM: u32 = 0x00ce_50b0;
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The global identity transform and the tolerance (`0.001`, a float) `moveRigidBodyPivot` compares
/// against it with.
const IDENTITY_TRANSFORM: u32 = 0x0126_8700;
const TOLERANCE: u32 = 0x0101_7d00;
/// `hkpTransformShape` constructor (`00cf5990`, thiscall `(shape, transform)`), `removeReference`
/// (`00c905b0`), the two option-block initialisers (`00d733e0`, `00d73410`, thiscall), the
/// transform collapse (`00d748d0`, cdecl `(body, options, result)`), `hkpRigidBody::setTransform`
/// (`00ca8890`) and `setCenterOfMassLocal` (`00ca85f0`).
const TRANSFORM_SHAPE_CONSTRUCT: u32 = 0x00cf_5990;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
const OPTIONS_INIT_FIRST: u32 = 0x00d7_33e0;
const OPTIONS_INIT_SECOND: u32 = 0x00d7_3410;
const COLLAPSE_TRANSFORMS: u32 = 0x00d7_48d0;
const SET_TRANSFORM: u32 = 0x00ca_8890;
const SET_CENTER_OF_MASS_LOCAL: u32 = 0x00ca_85f0;
/// The id of the report the pivot move silences while it runs.
const SILENCED_ID: u32 = 0x2ff8_c16f;
/// Vector constants: four -0.5, the sign bits of all lanes, of the first three lanes, four 1.0, and
/// the all-ones mask of the first three lanes.
const HALF_NEGATIVE: u32 = 0x010c_71e0;
const SIGN_MASK: u32 = 0x010c_44d0;
const XYZ_SIGN_MASK: u32 = 0x010c_4c00;
const ONES: u32 = 0x010c_7170;
const XYZ_MASK: u32 = 0x010c_4c10;
/// The identity quaternion, the float `1.0` and the float `1e-6` the pivot check compares with.
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
const ONE: u32 = 0x0101_2068;
const PIVOT_THRESHOLD: u32 = 0x0109_59f4;
/// `hkString::strCmp` (`00cb0fa0`, cdecl `(first, second)`), the `hkStringPtr` constructor
/// (`00cb0a60`), destructor (`00cb0c00`), assignment from a C string (`00cb0c10`) and assignment
/// from another one (`00cb0c30`), all thiscall.
const STR_CMP: u32 = 0x00cb_0fa0;
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN_TEXT: u32 = 0x00cb_0c10;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c30;
/// The `hkaRagdollInstance` constructor (`00ce2510`, thiscall `(bodies, constraints, skeleton,
/// parents)`) and the bone marking (`00cdd410`, cdecl `(skeleton, root, marks, 1)`).
const RAGDOLL_INSTANCE_CONSTRUCT: u32 = 0x00ce_2510;
const MARK_BONES: u32 = 0x00cd_d410;
/// The vtable of the `hkaSkeleton` the construction makes.
const SKELETON_VTABLE: u32 = 0x010c_4dec;
/// Bits 0..29 of an array's capacity word, and bit 31: the buffer is not owned.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

fn mulps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

fn addps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

fn subps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

/// `XORPS` with a bit mask.
fn xorps(a: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(a[lane].to_bits() ^ mask[lane]))
}

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// `ANDPS` with a bit mask.
fn andps(a: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(a[lane].to_bits() & mask[lane]))
}

/// The rotation of `v` by the quaternion `q` the way the SSE code computes it, before the caller
/// doubles it: `(v * (w * w + half) + dot * q) + cross * w` with `dot` the sum `z + (y + x)` of
/// `q * v` and `cross` the `SHUFPS` cross product, `q.c9 * v.d2 - q.d2 * v.c9` or, with `inverse`,
/// `v.c9 * q.d2 - v.d2 * q.c9` (the inverse rotation).
fn rotation(q: Vec4, v: Vec4, half: Vec4, inverse: bool) -> Vec4 {
    let w = shufps(q, q, 0xff);
    let dot = sum3(mulps(q, v));
    let cross = if inverse {
        subps(
            mulps(shufps(v, v, 0xc9), shufps(q, q, 0xd2)),
            mulps(shufps(v, v, 0xd2), shufps(q, q, 0xc9)),
        )
    } else {
        subps(
            mulps(shufps(q, q, 0xc9), shufps(v, v, 0xd2)),
            mulps(shufps(q, q, 0xd2), shufps(v, v, 0xc9)),
        )
    };
    let scaled = mulps(v, addps(mulps(w, w), half));
    addps(addps(scaled, mulps([dot; 4], q)), mulps(cross, w))
}

/// The refined reciprocal of a scale `(x, y, z, w)` taken as `(x, y, z, 1.0)`, with the lanes
/// above the third cleared: `RCPPS` and one Newton step (`(r + r) - (r * x) * r`), ANDed with the
/// mask at `010c4c10`.
fn inverse_scale(e: &Engine, scale: Vec4) -> Vec4 {
    let ones = load_vec(e, ONES);
    let lanes = shufps(scale, unpckhps(scale, ones), 0xc4);
    let refined = lanes.map(refined_reciprocal);
    andps(refined, load_bits(e, XYZ_MASK))
}

/// Runs `body(builder, buffer)` between the construction and the destruction of a text builder over
/// a 512-byte buffer (`00cb1600`, `00cc00e0`); the builder is 12 bytes followed by the buffer.
fn with_builder(e: &mut Engine, body: impl FnOnce(&mut Engine, u32, u32)) {
    e.with_stack(0x20c, |e, block| {
        let builder = block.addr();
        let buffer = builder + 0xc;
        e.call(TEXT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
        body(e, builder, buffer);
        e.call(TEXT_BUILDER_DESTRUCT, &args![builder]);
    });
}

/// Hands `buffer` to the report sink's slot `0xc` as an error (level 1).
fn report(e: &mut Engine, id: u32, buffer: u32, line: u32) {
    let sink = e.mem.u32(SINK);
    e.vcall(sink, 0xc, &args![1u32, id, buffer, SOURCE_FILE, line]);
}

/// The report of an unsupported constraint type.
fn report_unsupported(e: &mut Engine, text: u32, line: u32) {
    with_builder(e, |e, builder, buffer| {
        e.call(TEXT_BUILDER_APPEND, &args![builder, text]);
        report(e, ID_UNSUPPORTED, buffer, line);
    });
}

/// The type of the constraint's data object (virtual slot `+0x20`).
fn constraint_type(e: &mut Engine, constraint: Ptr) -> (u32, u32) {
    let data = e.mem.u32(constraint.addr() + 0xc);
    let kind = e.vcall(data, 0x20, &args![]).u32();
    (data, kind)
}

// Translated from 00ce9ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the constraint's type is one the rag doll code handles (unnamed in the engine map).
/// `EAX` = the constraint (`hkpConstraintInstance`) comes first, then the stack word `report_error`
/// (read as a byte): the type of the data object (slot `+0x20` of the word at `+0xc`) is supported
/// when it is 0, 1, 2, 6, 7 or 8. Otherwise, with `report_error` not zero, "Unsupported type of
/// constraint in rag doll" is reported (level 1, id `0xabbabf3b`, line `0x81`). Answers the flag in
/// `AL`.
pub fn fn_00ce9ba0(e: &mut Engine, constraint: Ptr, report_error: u8) -> bool {
    let (_, kind) = constraint_type(e, constraint);
    if matches!(kind, 0 | 1 | 2 | 6 | 7 | 8) {
        return true;
    }
    if report_error != 0 {
        report_unsupported(e, TEXT_UNSUPPORTED, 0x81);
    }
    false
}

// Translated from 00ce9c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the first pivot of the constraint's data (unnamed in the engine map; it looks like
/// the sibling of `_accessConstraintPivotB`, `00ce9cf0`, for pivot A). `EAX` = the constraint. The
/// data object's type gives `data + 0x20` for 0 and 8, `data + 0x50` for 1, 2, 6 and 7; any other type
/// reports "Unsupported type of constraint in rag doll" (level 1, id `0xabbabf3b`, line `0xb4`) and
/// answers 0.
pub fn fn_00ce9c40(e: &mut Engine, constraint: Ptr) -> u32 {
    let (data, kind) = constraint_type(e, constraint);
    match kind {
        0 | 8 => data.wrapping_add(0x20),
        1 | 2 | 6 | 7 => data.wrapping_add(0x50),
        _ => {
            report_unsupported(e, TEXT_UNSUPPORTED, 0xb4);
            0
        }
    }
}

// Translated from 00ce9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_accessConstraintPivotB` (Xbox PDB): the address of the second pivot of the constraint's data.
/// `EAX` = the constraint. The data object's type gives `data + 0x30` for 0, `data + 0x90` for 1, 2, 6
/// and 7, `data + 0x20` for 8; any other type reports "Unsupported type of constraint in rag doll."
/// (level 1, id `0xabbabf3b`, line `0xe6`) and answers 0.
pub fn fn_00ce9cf0(e: &mut Engine, constraint: Ptr) -> u32 {
    let (data, kind) = constraint_type(e, constraint);
    match kind {
        0 => data.wrapping_add(0x30),
        1 | 2 | 6 | 7 => data.wrapping_add(0x90),
        8 => data.wrapping_add(0x20),
        _ => {
            report_unsupported(e, TEXT_UNSUPPORTED_STOP, 0xe6);
            0
        }
    }
}

// Translated from 00ce9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `moveRigidBodyPivot` (Xbox PDB). The inputs are in registers, first `EAX` = `transform` (a
/// `hkQsTransform`: translation `+0`, rotation `+0x10`, scale `+0x20`), then `ESI` = `body` (the
/// rigid body); answers 0. Unless `transform` is approximately the identity transform (`00ce5120`
/// against `01268700` with the tolerance at `01017d00`, copied through the x87 stack) the body's
/// transform (`+0xf0`, as a `hkQsTransform` by `00ce5070`) is composed with it: the new translation
/// is `2 * rotate(q_body, t) + t_body`, the new rotation `q_body * q` (`setMul`, `00c66320`) and the new
/// scale the product of the scales; the inverse of `transform` (translation `-2 * rotate^-1(q, t)`,
/// the conjugate of `q`, the reciprocal of the scale) is turned into a `hkTransform` (`00ce50b0`) and
/// wrapped, with the body's shape (`+0x10`), in a new `hkpTransformShape` (0x70 bytes, `00cf5990`)
/// that becomes the body's shape (slot `+0xc`; the extra reference is let go). With the report sink
/// told to ignore the id `0x2ff8c16f` (`+0x14` asks for the setting into a byte, `+0x10` turns it
/// off with the garbage upper bytes of the pointer as the exe pushes them), the transform collapse
/// (`00d748d0`) runs on the body, then the setting is restored with the dword the exe reads from its
/// frame (the byte answered plus the three stale bytes after it). The new transform is set on the body
/// (`00ca8890`), and unless the body's motion type byte (`+0xe8`) is 5 the centre of mass offset
/// (`+0x140`) is moved by the inverse transform (`00c7f690`) and set (`00ca85f0`).
///
/// The scratch frame follows the exe's: relative to its base `P`, the tolerance at `P`, the answered
/// byte at `P + 0xc` (the compare's result byte at `P + 0xf`), the body's transform at `P + 0x50`,
/// the new rotation at `P + 0xc0`, the new translation at `P + 0xb0`, the new scale at `P + 0xd0`,
/// the inverse at `P + 0x80`, the new `hkTransform` at `P + 0xe0`.
pub fn fn_00ce9880(e: &mut Engine, transform: Ptr, body: Ptr) -> u32 {
    let x = transform.addr();
    let b = body.addr();
    // The exe aligns ESP (`AND ESP,0xfffffff0`), then `SUB ESP,0x118` and three pushes: P is
    // 0x124 bytes under the 16-byte boundary (00ce9883..00ce9894).
    with_aligned_frame(e, 0x124, |e, p| {
        let tolerance = x87_quiet_bits(e.mem.u32(TOLERANCE));
        e.mem.set_u32(p, tolerance);
        let result = e
            .call(
                IS_APPROXIMATELY_EQUAL,
                &args![x, p + 0xf, IDENTITY_TRANSFORM, tolerance],
            )
            .u32();
        if e.mem.u8(result) == 0 {
            e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![p + 0x50, b + 0xf0]);
            let half = load_vec(e, HALF_NEGATIVE);
            let q_body = load_vec(e, p + 0x60);
            let t = load_vec(e, x);
            let spun = rotation(q_body, t, half, false);
            let t_body = load_vec(e, p + 0x50);
            let new_translation = addps(addps(spun, spun), t_body);
            store_vec(e, p + 0x30, t);
            store_vec(e, p + 0xb0, new_translation);
            e.call(QUATERNION_SET_MUL, &args![p + 0xc0, p + 0x60, x + 0x10]);
            let new_scale = mulps(load_vec(e, p + 0x70), load_vec(e, x + 0x20));
            let t = load_vec(e, p + 0x30);
            store_vec(e, p + 0xd0, new_scale);
            let q = load_vec(e, x + 0x10);
            let half = load_vec(e, HALF_NEGATIVE);
            let spun_back = rotation(q, t, half, true);
            let inverse_translation = xorps(addps(spun_back, spun_back), load_bits(e, SIGN_MASK));
            store_vec(e, p + 0x80, inverse_translation);
            let conjugate = xorps(load_vec(e, x + 0x10), load_bits(e, XYZ_SIGN_MASK));
            store_vec(e, p + 0x90, conjugate);
            let inverse = inverse_scale(e, load_vec(e, x + 0x20));
            for lane in 0..4 {
                let word = e.mem.u32(b + 0x140 + 4 * lane);
                e.mem.set_u32(p + 0x20 + 4 * lane, word);
            }
            store_vec(e, p + 0xa0, inverse);
            let sink = e.mem.u32(SINK);
            e.vcall(sink, 0x14, &args![p + 0xc, SILENCED_ID]);
            let sink = e.mem.u32(SINK);
            e.vcall(sink, 0x10, &args![SILENCED_ID, sink & 0xffff_ff00]);
            let shape = e.mem.u32(b + 0x10);
            e.call(QS_TO_TRANSFORM, &args![p + 0x80, p + 0xe0]);
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let block = e.vcall(allocator, 4, &args![0x70u32]).u32();
            e.mem.set_u16(block + 4, 0x70);
            let new_shape = e
                .call(TRANSFORM_SHAPE_CONSTRUCT, &args![block, shape, p + 0xe0])
                .u32();
            e.vcall(b, 0xc, &args![new_shape]);
            e.call(REMOVE_REFERENCE, &args![new_shape]);
            e.call(OPTIONS_INIT_FIRST, &args![p + 0x30]);
            e.call(OPTIONS_INIT_SECOND, &args![p + 0x10]);
            e.call(COLLAPSE_TRANSFORMS, &args![b, p + 0x30, p + 0x10]);
            let sink = e.mem.u32(SINK);
            let enabled = e.mem.u32(p + 0xc);
            e.vcall(sink, 0x10, &args![SILENCED_ID, enabled]);
            e.call(QS_TO_TRANSFORM, &args![p + 0xb0, p + 0xe0]);
            e.call(SET_TRANSFORM, &args![b, p + 0xe0]);
            if e.mem.u8(b + 0xe8) != 5 {
                e.call(
                    SET_TRANSFORMED_INVERSE_POS,
                    &args![p + 0x10, p + 0xe0, p + 0x20],
                );
                e.call(SET_CENTER_OF_MASS_LOCAL, &args![b, p + 0x10]);
            }
        }
    });
    0
}

// Translated from 00ce9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, stack `(constraints)` (an `hkArray` of constraints): for each
/// constraint (the size is read once) whose type is supported (`00ce9ba0` with the report on), takes
/// its first pivot (`00ce9c40`) as a `hkQsTransform` with the identity rotation (`010c71b0`) and the
/// scale `1.0` (`01012068`, four copies) in a 0x60-byte frame; when the squared length of the pivot's
/// first three lanes (`z * z + (y * y + x * x)`, as `ADDSS` sums them) is above the float at `010959f4`
/// (`1e-6`; the x87 compare is false for NaN and for equality) the pivot of the first body
/// (`+0x14`) is moved by it (`moveRigidBodyPivot`, `00ce9880`). Answers 1 when a constraint is not
/// supported or a move answers non-zero, else 0.
pub fn fn_00ce9db0(e: &mut Engine, constraints: Ptr) -> u32 {
    // The exe aligns ESP, then `SUB ESP,0x54` and three pushes: a 0x60-byte frame under the
    // boundary, the pivot transform at +0x30 (00ce9e6a).
    with_aligned_frame(e, 0x60, |e, frame| {
        let list = constraints.addr();
        let size = e.mem.i32(list + 4);
        let mut answer = 0;
        let mut index: i32 = 0;
        if size > 0 {
            loop {
                let data = e.mem.u32(list);
                let constraint = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                if !fn_00ce9ba0(e, Ptr::new(constraint), 1) {
                    answer = 1;
                } else {
                    let pivot = fn_00ce9c40(e, Ptr::new(constraint));
                    for lane in 0..4 {
                        let word = e.mem.u32(pivot + 4 * lane);
                        e.mem.set_u32(frame + 0x20 + 4 * lane, word);
                        e.mem.set_u32(frame + 0x30 + 4 * lane, word);
                        let rotation_word = e.mem.u32(IDENTITY_ROTATION + 4 * lane);
                        e.mem.set_u32(frame + 0x40 + 4 * lane, rotation_word);
                        let one = e.mem.u32(ONE);
                        e.mem.set_u32(frame + 0x50 + 4 * lane, one);
                    }
                    let body = e.mem.u32(constraint + 0x14);
                    let squared = mulps(load_vec(e, frame + 0x30), load_vec(e, frame + 0x30));
                    let length = sum3(squared);
                    let threshold = f64::from(x87_quiet(e.mem.f32(PIVOT_THRESHOLD)));
                    if threshold < f64::from(length) {
                        let moved = fn_00ce9880(e, Ptr::new(frame + 0x30), Ptr::new(body));
                        if moved != 0 {
                            answer = 1;
                        }
                    }
                }
                index += 1;
                if index >= size {
                    break;
                }
            }
        }
        answer
    })
}

// Translated from 00ce9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `alignConstraintSpacesToChildren` (Xbox PDB), stack `(constraints)`: for each constraint (the
/// size is read once) of a supported type (`00ce9ba0` with the report on) copies the vector at
/// `+0x120` of the first body (`+0x14`), clears the first pivot (`00ce9c40`, 16 bytes) and sets the
/// second pivot (`00ce9cf0`) to the copy moved into the space of the second body's transform
/// (`+0xf0` of the body at `+0x18`, `hkVector4::setTransformedInversePos`, `00c7f690`). Whatever the
/// support, a constraint whose type is 8 reports "Unusual : Aligning constraint spaces of a stiff
/// spring." (level 1, id `0xabba876d`, line `0x134`). Answers 0.
pub fn fn_00ce9e90(e: &mut Engine, constraints: Ptr) -> u32 {
    // The exe aligns ESP, then `SUB ESP,0x224` and three pushes: a 0x230-byte frame under the
    // boundary, the body vector at +0x10 (00ce9f0d).
    with_aligned_frame(e, 0x230, |e, frame| {
        let list = constraints.addr();
        let size = e.mem.i32(list + 4);
        let mut index: i32 = 0;
        if size > 0 {
            loop {
                let data = e.mem.u32(list);
                let constraint = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
                if fn_00ce9ba0(e, Ptr::new(constraint), 1) {
                    let body_a = e.mem.u32(constraint + 0x14);
                    let local = frame + 0x10;
                    for lane in 0..4 {
                        let word = e.mem.u32(body_a + 0x120 + 4 * lane);
                        e.mem.set_u32(local + 4 * lane, word);
                    }
                    let body_b = e.mem.u32(constraint + 0x18);
                    let pivot_a = fn_00ce9c40(e, Ptr::new(constraint));
                    for lane in 0..4 {
                        e.mem.set_u32(pivot_a + 4 * lane, 0);
                    }
                    let pivot_b = fn_00ce9cf0(e, Ptr::new(constraint));
                    e.call(
                        SET_TRANSFORMED_INVERSE_POS,
                        &args![pivot_b, body_b + 0xf0, local],
                    );
                }
                let (_, kind) = constraint_type(e, Ptr::new(constraint));
                if kind == 8 {
                    with_builder(e, |e, builder, buffer| {
                        e.call(TEXT_BUILDER_APPEND, &args![builder, TEXT_STIFF_SPRING]);
                        report(e, 0xabba_876d, buffer, 0x134);
                    });
                }
                index += 1;
                if index >= size {
                    break;
                }
            }
        }
        0
    })
}

/// Bumps the memory router's pointer past `bytes` (rounded up to 128) and answers where it was.
fn reserve_bytes(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let start = e.mem.u32(router);
    e.mem.set_u32(
        router,
        start.wrapping_add(bytes.wrapping_add(0x7f) & 0xffff_ff80),
    );
    start
}

/// Gives back the memory router's stack up to `start` and then, when `capacity_word` says the
/// buffer is owned (bit 31 clear), frees it through the `+0x10` allocator (`(buffer, capacity * 4)`).
fn release_raw(e: &mut Engine, start: u32, capacity_word: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if capacity_word as i32 >= 0 {
        let bytes = (capacity_word & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![start, bytes]);
    }
}

/// A `hkLocalArray<int>` the exe builds on the router's bump stack: the array's three words and
/// the router's bump pointer at its creation.
struct LocalInts {
    header: u32,
    start: u32,
}

/// An empty local array whose storage for `capacity` words comes off the router's bump stack (the
/// capacity word is `capacity | 0x80000000`, so the buffer is never freed).
fn local_ints_empty(e: &mut Engine, capacity: u32) -> LocalInts {
    let header = e.mem.alloc(12);
    e.mem.set_u32(header + 8, NOT_OWNED);
    let start = reserve_bytes(e, capacity.wrapping_mul(4));
    e.mem.set_u32(header, start);
    e.mem.set_u32(header + 8, capacity | NOT_OWNED);
    LocalInts { header, start }
}

/// A local array of `count` words, each `fill`: [`local_ints_empty`], then the capacity is checked
/// against `count` (it always is enough) through the router's `+0x10` allocator (`_reserve`, `00c908c0`,
/// would grow it to at least `count`), the words after the current size are filled and the size set.
fn local_ints_filled(e: &mut Engine, count: u32, fill: u32) -> LocalInts {
    let array = local_ints_empty(e, count);
    let header = array.header;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(header + 8) & CAPACITY_MASK) as i32;
    let wanted = count as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let grown = doubled.max(wanted);
        e.call(ARRAY_RESERVE, &args![allocator, header, grown, 4u32]);
    }
    let size = e.mem.u32(header + 4);
    let data = e.mem.u32(header);
    let missing = wanted.wrapping_sub(size as i32);
    if missing > 0 {
        for step in 0..missing as u32 {
            e.mem.set_u32(
                data.wrapping_add(size.wrapping_add(step).wrapping_mul(4)),
                fill,
            );
        }
    }
    e.mem.set_u32(header + 4, count);
    array
}

/// Ends a local array: when its data is still the storage taken at its start the size is cleared,
/// the router's pointer goes back to the start, the size is cleared and an owned buffer (the array
/// grew) is freed through the router's `+0x10` allocator (`(data, capacity * 4)`), then the array is
/// reset to no data and the capacity word `0x80000000`.
fn local_ints_release(e: &mut Engine, array: &LocalInts) {
    let header = array.header;
    if array.start == e.mem.u32(header) {
        e.mem.set_u32(header + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, array.start);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(header + 8);
    e.mem.set_u32(header + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(header);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(header, 0);
    e.mem.set_u32(header + 8, NOT_OWNED);
    e.mem.free(header);
}

/// Appends `value` to the array of words at `array`: the router is asked first, and when the size
/// equals the capacity the array grows through `00c90940` on the `+0x10` allocator.
fn push_back_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Gives a heap array of words (not on the bump stack) back: the size is cleared, an owned buffer is
/// freed through the router's `+0x10` allocator (`(data, capacity * 4)`) and the array is reset.
fn release_heap_ints(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

/// The index of `value` in the `count` words of the array at `list` (read once), or -1.
fn find_word(e: &Engine, list: u32, count: i32, value: u32) -> i32 {
    if count > 0 {
        let data = e.mem.u32(list);
        for index in 0..count {
            if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == value {
                return index;
            }
        }
    }
    -1
}

/// Makes the array of words at `array` hold exactly `count` words: when its capacity (bits 0..29)
/// is below `count` the old buffer is freed (when owned, `(data, capacity * 4)`) and a new one asked
/// from the router's `+0x10` allocator (slot `+0xc` with the address of the byte count, which it
/// rewrites with what it granted); the capacity becomes the granted bytes divided by 4 (rounded toward
/// zero). The size is set to `count`.
fn resize_word_array(e: &mut Engine, array: u32, count: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(array + 8);
    let capacity = (capacity_word & CAPACITY_MASK) as i32;
    if capacity < count as i32 {
        if capacity_word as i32 >= 0 {
            let data = e.mem.u32(array);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), count.wrapping_mul(4));
            let data = e.vcall(allocator, 0xc, &args![slot]).u32();
            e.mem.set_u32(array, data);
            let granted = e.mem.i32(slot.addr());
            e.mem
                .set_i32(array + 8, granted.wrapping_add((granted >> 31) & 3) >> 2);
        });
    }
    e.mem.set_u32(array + 4, count);
}

// Translated from 00cea030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollUtils::reorderForRagdoll` (Xbox PDB), stack `(bodies, constraints)` (two `hkArray`s of
/// pointers), answers 0, or 1 for a failure. With `n` bodies there must be `n - 1` constraints;
/// otherwise "Expecting <n-1> constraints, not <m>. Can't create rag doll" is reported (level 1, id
/// `0xabba54bb`, line `0x144`) and the answer is 1. Local arrays on the router's bump stack: `parent
/// constraint` (zeros) and `parent` (-1), with a raw `order` buffer between them. For each
/// constraint the indexes of its two bodies (`+0x14`, `+0x18`) in `bodies` are found; when either
/// is missing, or the first body already has a parent or a constraint recorded (the constraint
/// array's entry is not 0), everything is released and the answer is 1; otherwise `constraint[first] =
/// i` and `parent[first] = second`. The depth of each body (the length of its parent chain) is
/// counted, `order` is set to `0..n` and sorted by increasing depth (a selection sort that swaps
/// the depths and `order` together), and the bodies and constraints are rewritten in that order into
/// two raw buffers (the constraint of a body, taken from its constraint index, or 0) and then copied
/// back into the arrays (a short array is grown through the `+0x10` allocator's slot `+0xc`, after
/// freeing an owned old buffer). Everything is released in reverse order and the answer is 0.
pub fn hka_ragdoll_utils_reorder_for_ragdoll(e: &mut Engine, bodies: Ptr, constraints: Ptr) -> u32 {
    let bodies = bodies.addr();
    let constraints = constraints.addr();
    let n = e.mem.u32(bodies + 4);
    let m = e.mem.u32(constraints + 4);
    let expected = n.wrapping_sub(1);
    if m != expected {
        with_builder(e, |e, builder, buffer| {
            let b = e
                .call(TEXT_BUILDER_APPEND, &args![builder, TEXT_EXPECTING])
                .u32();
            let b = e.call(TEXT_BUILDER_APPEND_INT, &args![b, expected]).u32();
            let b = e
                .call(TEXT_BUILDER_APPEND, &args![b, TEXT_CONSTRAINTS_NOT])
                .u32();
            let b = e.call(TEXT_BUILDER_APPEND_INT, &args![b, m]).u32();
            e.call(TEXT_BUILDER_APPEND, &args![b, TEXT_CANT_CREATE]);
            report(e, 0xabba_54bb, buffer, 0x144);
        });
        return 1;
    }
    let n_signed = n as i32;
    let m_signed = m as i32;
    let by_constraint = local_ints_filled(e, n, 0);
    let order = reserve_bytes(e, n.wrapping_mul(4));
    let parent = local_ints_filled(e, n, 0xffff_ffff);
    let flagged_n = n | NOT_OWNED;
    let mut constraint_index: i32 = 0;
    if m_signed > 0 {
        loop {
            let constraint = e.mem.u32(
                e.mem
                    .u32(constraints)
                    .wrapping_add((constraint_index as u32).wrapping_mul(4)),
            );
            let body_b = e.mem.u32(constraint + 0x18);
            let body_a = e.mem.u32(constraint + 0x14);
            let count = e.mem.i32(bodies + 4);
            let first = find_word(e, bodies, count, body_a);
            let second = find_word(e, bodies, count, body_b);
            let rejected = first < 0
                || second < 0
                || e.mem.u32(
                    e.mem
                        .u32(parent.header)
                        .wrapping_add((first as u32).wrapping_mul(4)),
                ) != 0xffff_ffff
                || e.mem.u32(
                    e.mem
                        .u32(by_constraint.header)
                        .wrapping_add((first as u32).wrapping_mul(4)),
                ) != 0;
            if rejected {
                local_ints_release(e, &parent);
                release_raw(e, order, flagged_n);
                local_ints_release(e, &by_constraint);
                return 1;
            }
            let at = e
                .mem
                .u32(by_constraint.header)
                .wrapping_add((first as u32).wrapping_mul(4));
            e.mem.set_u32(at, constraint_index as u32);
            constraint_index += 1;
            let at = e
                .mem
                .u32(parent.header)
                .wrapping_add((first as u32).wrapping_mul(4));
            e.mem.set_u32(at, second as u32);
            if constraint_index >= m_signed {
                break;
            }
        }
    }
    let depth = local_ints_filled(e, n, 0);
    if n_signed > 0 {
        for body in 0..n_signed {
            let mut chain = e.mem.i32(
                e.mem
                    .u32(parent.header)
                    .wrapping_add((body as u32).wrapping_mul(4)),
            );
            if chain != -1 {
                loop {
                    let at = e
                        .mem
                        .u32(depth.header)
                        .wrapping_add((body as u32).wrapping_mul(4));
                    let value = e.mem.u32(at);
                    e.mem.set_u32(at, value.wrapping_add(1));
                    chain = e.mem.i32(
                        e.mem
                            .u32(parent.header)
                            .wrapping_add((chain as u32).wrapping_mul(4)),
                    );
                    if chain == -1 {
                        break;
                    }
                }
            }
        }
        for index in 0..n {
            e.mem
                .set_u32(order.wrapping_add(index.wrapping_mul(4)), index);
        }
    }
    let outer = n_signed.wrapping_sub(1);
    if outer > 0 {
        let mut first = 0u32;
        let mut remaining = outer;
        let mut start = 1i32;
        loop {
            let mut second = start;
            while second < n_signed {
                let depths = e.mem.u32(depth.header);
                let first_at = depths.wrapping_add(first.wrapping_mul(4));
                let second_at = depths.wrapping_add((second as u32).wrapping_mul(4));
                let first_depth = e.mem.i32(first_at);
                if first_depth > e.mem.i32(second_at) {
                    let second_depth = e.mem.u32(second_at);
                    e.mem.set_u32(first_at, second_depth);
                    e.mem.set_u32(second_at, first_depth as u32);
                    let first_order = e.mem.u32(order.wrapping_add(first.wrapping_mul(4)));
                    let second_order = e
                        .mem
                        .u32(order.wrapping_add((second as u32).wrapping_mul(4)));
                    e.mem
                        .set_u32(order.wrapping_add(first.wrapping_mul(4)), second_order);
                    e.mem.set_u32(
                        order.wrapping_add((second as u32).wrapping_mul(4)),
                        first_order,
                    );
                }
                second += 1;
            }
            start += 1;
            first = first.wrapping_add(1);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    local_ints_release(e, &depth);
    let sorted_bodies = reserve_bytes(e, n.wrapping_mul(4));
    let sorted_constraints = reserve_bytes(e, m.wrapping_mul(4));
    let flagged_m = m | NOT_OWNED;
    if n_signed > 0 {
        let mut out = sorted_constraints.wrapping_sub(4);
        for index in 0..n_signed {
            let at = (index as u32).wrapping_mul(4);
            let original = e.mem.u32(order.wrapping_add(at));
            let body = e
                .mem
                .u32(e.mem.u32(bodies).wrapping_add(original.wrapping_mul(4)));
            e.mem.set_u32(sorted_bodies.wrapping_add(at), body);
            if index > 0 {
                let entry = e.mem.i32(
                    e.mem
                        .u32(by_constraint.header)
                        .wrapping_add(original.wrapping_mul(4)),
                );
                if entry >= 0 {
                    let picked = e.mem.u32(
                        e.mem
                            .u32(constraints)
                            .wrapping_add((entry as u32).wrapping_mul(4)),
                    );
                    e.mem.set_u32(out, picked);
                } else {
                    e.mem.set_u32(out, 0);
                }
            }
            out = out.wrapping_add(4);
        }
    }
    resize_word_array(e, bodies, n);
    let data = e.mem.u32(bodies);
    if n_signed > 0 {
        for index in 0..n {
            let word = e.mem.u32(sorted_bodies.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(data.wrapping_add(index.wrapping_mul(4)), word);
        }
    }
    resize_word_array(e, constraints, m);
    let data = e.mem.u32(constraints);
    if m_signed > 0 {
        for index in 0..m {
            let word = e
                .mem
                .u32(sorted_constraints.wrapping_add(index.wrapping_mul(4)));
            e.mem
                .set_u32(data.wrapping_add(index.wrapping_mul(4)), word);
        }
    }
    release_raw(e, sorted_constraints, flagged_m);
    release_raw(e, sorted_bodies, flagged_n);
    local_ints_release(e, &parent);
    release_raw(e, order, flagged_n);
    local_ints_release(e, &by_constraint);
    0
}

/// Reports a failure of `createRagdollInstanceFromSkeleton`: the text builder with one message built
/// by `build`, handed to the sink at level 1 with `id` and `line`.
fn report_creation(e: &mut Engine, id: u32, line: u32, build: impl FnOnce(&mut Engine, u32)) {
    with_builder(e, |e, builder, buffer| {
        build(e, builder);
        report(e, id, buffer, line);
    });
}

/// The three local arrays `createRagdollInstanceFromSkeleton` ends with, in the order the exe
/// releases them: the parent indexes, the constraints and the bodies.
fn release_creation_arrays(
    e: &mut Engine,
    bodies: &LocalInts,
    constraints: &LocalInts,
    parents: &LocalInts,
) {
    local_ints_release(e, parents);
    local_ints_release(e, constraints);
    local_ints_release(e, bodies);
}

// Translated from 00cea810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollUtils::createRagdollInstanceFromSkeleton` (Xbox PDB), stack `(skeleton, bodies,
/// constraints)`: answers the new `hkaRagdollInstance`, or 0 after a reported failure. Three
/// empty local arrays on the router's bump stack (capacities from the skeleton's bone count `n`,
/// at `+0x1c`: the bodies `n`, the constraints `n - 1`, the parent indexes `n`) are filled bone by
/// bone from the skeleton's bone names (`+0x18`, 8 bytes each, the name word with bit 0 masked off)
/// and parent indexes (16-bit words at `+0xc`). A bone with no name reports "Some bones have no
/// name - can't create rag doll" (id `0xabbae6a2`, line `0x2a3`) and fails. The rigid body whose
/// name (`+0x74`, bit 0 masked) matches the bone's (`hkString::strCmp`, `00cb0fa0`) is taken; a
/// second match reports "Some RBS have duplicated names : <name>" (id `0xabba98aa`, line `0x26b`) and
/// is ignored. A bone without a body pushes -1 on the parent indexes. Otherwise the number of bodies
/// so far is pushed on the parent indexes and the body on the bodies; for a bone with a parent (not
/// `0xffff`) the parent's body is looked up through the parent indexes and the constraint whose bodies
/// are this one (`+0x14`) and the parent (`+0x18`) is searched: a second one reports "Found more than
/// one constraint between rigid bodies <a> and <b>" (id `0xabba82ff`, line `0x291`) and is ignored,
/// none reports "Couldn't find constraint for bone : <bone>" (id `0xabba9f9f`, line `0x298`) and
/// fails; the constraint is pushed on the constraints. No body at all reports "Skeleton has no
/// associated rigid bodies - can't create rag doll" (id `0xabbae6e4`, line `0x2aa`) and fails.
/// Otherwise a 0x30-byte `hkaRagdollInstance` is made (`00ce2510` with the bodies, constraints,
/// skeleton and parent indexes). Every exit releases the arrays, parent indexes first.
pub fn hka_ragdoll_utils_create_ragdoll_instance_from_skeleton(
    e: &mut Engine,
    skeleton: Ptr,
    bodies: Ptr,
    constraints: Ptr,
) -> u32 {
    let skeleton = skeleton.addr();
    let rigid_bodies = bodies.addr();
    let rigid_constraints = constraints.addr();
    let n = e.mem.u32(skeleton + 0x1c);
    let found_bodies = local_ints_empty(e, n);
    let found_constraints = local_ints_empty(e, n.wrapping_sub(1));
    let parents = local_ints_empty(e, n);
    let n_signed = n as i32;
    if n_signed > 0 {
        let mut bone_index: i32 = 0;
        loop {
            let bones = e.mem.u32(skeleton + 0x18);
            let bone = bones.wrapping_add((bone_index as u32).wrapping_mul(8));
            let parent_bone = u32::from(
                e.mem.u16(
                    e.mem
                        .u32(skeleton + 0xc)
                        .wrapping_add((bone_index as u32).wrapping_mul(2)),
                ),
            );
            if e.mem.u32(bone) & 0xffff_fffe == 0 {
                report_creation(e, 0xabba_e6a2, 0x2a3, |e, builder| {
                    e.call(TEXT_BUILDER_APPEND, &args![builder, TEXT_NO_BONE_NAME]);
                });
                release_creation_arrays(e, &found_bodies, &found_constraints, &parents);
                return 0;
            }
            let mut matched: u32 = 0;
            if e.mem.i32(rigid_bodies + 4) > 0 {
                let mut index: i32 = 0;
                loop {
                    let candidate = e.mem.u32(
                        e.mem
                            .u32(rigid_bodies)
                            .wrapping_add((index as u32).wrapping_mul(4)),
                    );
                    let name = e.mem.u32(candidate + 0x74) & 0xffff_fffe;
                    if name != 0 {
                        let bone_name = e.mem.u32(bone) & 0xffff_fffe;
                        if e.call(STR_CMP, &args![name, bone_name]).u32() == 0 {
                            if matched == 0 {
                                matched = candidate;
                            } else {
                                report_creation(e, 0xabba_98aa, 0x26b, |e, builder| {
                                    let b = e
                                        .call(
                                            TEXT_BUILDER_APPEND,
                                            &args![builder, TEXT_DUPLICATE_BODY],
                                        )
                                        .u32();
                                    e.call(TEXT_BUILDER_APPEND, &args![b, name]);
                                });
                            }
                        }
                    }
                    index += 1;
                    if index >= e.mem.i32(rigid_bodies + 4) {
                        break;
                    }
                }
            }
            if matched == 0 {
                push_back_word(e, parents.header, 0xffff_ffff);
            } else {
                let body_count = e.mem.u32(found_bodies.header + 4);
                push_back_word(e, parents.header, body_count);
                push_back_word(e, found_bodies.header, matched);
                if parent_bone as u16 != 0xffff {
                    let parent_slot =
                        e.mem.i32(e.mem.u32(parents.header).wrapping_add(
                            ((parent_bone as u16 as i16 as i32) as u32).wrapping_mul(4),
                        ));
                    let parent_body = e.mem.u32(
                        e.mem
                            .u32(found_bodies.header)
                            .wrapping_add((parent_slot as u32).wrapping_mul(4)),
                    );
                    let mut chosen: u32 = 0;
                    if e.mem.i32(rigid_constraints + 4) > 0 {
                        let mut index: i32 = 0;
                        loop {
                            let candidate = e.mem.u32(
                                e.mem
                                    .u32(rigid_constraints)
                                    .wrapping_add((index as u32).wrapping_mul(4)),
                            );
                            let first = e.mem.u32(candidate + 0x14);
                            let second = e.mem.u32(candidate + 0x18);
                            if first == matched && second == parent_body {
                                if chosen == 0 {
                                    chosen = candidate;
                                } else {
                                    report_creation(e, 0xabba_82ff, 0x291, |e, builder| {
                                        let b = e
                                            .call(
                                                TEXT_BUILDER_APPEND,
                                                &args![builder, TEXT_MORE_THAN_ONE],
                                            )
                                            .u32();
                                        let first_name = e.mem.u32(first + 0x74) & 0xffff_fffe;
                                        let second_name = e.mem.u32(second + 0x74) & 0xffff_fffe;
                                        let b = e
                                            .call(TEXT_BUILDER_APPEND, &args![b, first_name])
                                            .u32();
                                        let b =
                                            e.call(TEXT_BUILDER_APPEND, &args![b, TEXT_AND]).u32();
                                        e.call(TEXT_BUILDER_APPEND, &args![b, second_name]);
                                    });
                                }
                            }
                            index += 1;
                            if index >= e.mem.i32(rigid_constraints + 4) {
                                break;
                            }
                        }
                    }
                    if chosen == 0 {
                        report_creation(e, 0xabba_9f9f, 0x298, |e, builder| {
                            let b = e
                                .call(TEXT_BUILDER_APPEND, &args![builder, TEXT_NO_CONSTRAINT])
                                .u32();
                            e.call(TEXT_BUILDER_APPEND_STRING_PTR, &args![b, bone]);
                        });
                        release_creation_arrays(e, &found_bodies, &found_constraints, &parents);
                        return 0;
                    }
                    push_back_word(e, found_constraints.header, chosen);
                }
            }
            bone_index += 1;
            if bone_index >= n_signed {
                break;
            }
        }
    }
    if e.mem.u32(found_bodies.header + 4) == 0 {
        report_creation(e, 0xabba_e6e4, 0x2aa, |e, builder| {
            e.call(TEXT_BUILDER_APPEND, &args![builder, TEXT_NO_BODIES]);
        });
        release_creation_arrays(e, &found_bodies, &found_constraints, &parents);
        return 0;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
    e.mem.set_u16(block + 4, 0x30);
    let instance = e
        .call(
            RAGDOLL_INSTANCE_CONSTRUCT,
            &args![
                block,
                found_bodies.header,
                found_constraints.header,
                skeleton,
                parents.header
            ],
        )
        .u32();
    release_creation_arrays(e, &found_bodies, &found_constraints, &parents);
    instance
}

// Translated from 00ceaf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, stack `(this, root, out_first, out_second)`, answers nothing: makes the
/// two rag doll instances of a skeleton's two bone groups. `this` is read by offsets: `+0x2c` the
/// skeleton (bone count at its `+0x1c`), `+0x8` an array of rigid bodies, `+0x14` an array of
/// constraints and `+0x20` an array of body indexes by bone (negative: none). A byte buffer of one
/// byte per bone is asked from the router's `+0x10` allocator (slot `+0xc` with the address of the
/// byte count, which it rewrites with the size granted; with no bones none is asked) and marked by
/// `00cdd410` (skeleton, `root`, buffer, 1). Then, once for the marks 0 and 1, three empty arrays
/// (parent indexes `X`, bodies `Y`, constraints `Z`, growing through `00c90940`) are filled bone by
/// bone: a bone whose mark differs pushes -1 on `X`; a marked bone looks up its body and the
/// constraint before it (index - 1), a missing body pushes -1 on `X`, otherwise the number of bodies
/// so far goes on `X`, the body on `Y` and, unless the bone is `root` or the first, the constraint on
/// `Z`. A 0x30-byte `hkaRagdollInstance` (`00ce2510` with `Y`, `Z`, the skeleton and `X`) is made and
/// stored at the output address of the pass (`*out_first`, `*out_second`), and the arrays are given
/// back, `X` then `Z` then `Y`. Last the byte buffer is freed (`(buffer, capacity)`) when it is owned.
pub fn fn_00ceaf60(e: &mut Engine, this: Ptr, root: u32, out_first: u32, out_second: u32) {
    // The exe's frame (`SUB ESP,0x3c`, four pushes): 0x4c bytes, the allocation request word at
    // +0x18 (00ceaf9e), the three arrays at +0x28, +0x34 and +0x40 (00ceaff3..00ceb013).
    e.with_stack(0x4c, |e, frame| {
        let frame = frame.addr();
        let t = this.addr();
        let outputs = [out_first, out_second];
        let marks = [0u8, 1u8];
        let skeleton = e.mem.u32(t + 0x2c);
        let bone_count = e.mem.u32(skeleton + 0x1c);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let mut buffer: u32 = 0;
        let mut buffer_capacity: u32 = NOT_OWNED;
        if bone_count != 0 {
            let request = frame + 0x18;
            e.mem.set_u32(request, bone_count);
            buffer = e.vcall(allocator, 0xc, &args![request]).u32();
            let granted = e.mem.u32(request);
            if granted != 0 {
                buffer_capacity = granted;
            }
        }
        let skeleton = e.mem.u32(t + 0x2c);
        e.call(MARK_BONES, &args![skeleton, root, buffer, 1u32]);
        for pass in 0..2 {
            let arrays = frame + 0x28;
            let (x, y, z) = (arrays, arrays + 0xc, arrays + 0x18);
            for array in [x, y, z] {
                e.mem.set_u32(array + 8, NOT_OWNED);
            }
            let mark = marks[pass];
            let mut bone: i32 = 0;
            let skeleton = e.mem.u32(t + 0x2c);
            if e.mem.i32(skeleton + 0x1c) > 0 {
                loop {
                    if e.mem.u8(buffer.wrapping_add(bone as u32)) != mark {
                        push_back_word(e, x, 0xffff_ffff);
                    } else {
                        let index = e.mem.i32(
                            e.mem
                                .u32(t + 0x20)
                                .wrapping_add((bone as u32).wrapping_mul(4)),
                        );
                        let mut body: u32 = 0;
                        if index >= 0 {
                            body = e.mem.u32(
                                e.mem
                                    .u32(t + 8)
                                    .wrapping_add((index as u32).wrapping_mul(4)),
                            );
                        }
                        let before = index.wrapping_sub(1);
                        let mut constraint: u32 = 0;
                        if before >= 0 {
                            constraint = e.mem.u32(
                                e.mem
                                    .u32(t + 0x14)
                                    .wrapping_add((before as u32).wrapping_mul(4)),
                            );
                        }
                        if body == 0 {
                            push_back_word(e, x, 0xffff_ffff);
                        } else {
                            let bodies_so_far = e.mem.u32(y + 4);
                            push_back_word(e, x, bodies_so_far);
                            push_back_word(e, y, body);
                            if bone as u32 != root && bone != 0 {
                                push_back_word(e, z, constraint);
                            }
                        }
                    }
                    bone += 1;
                    let skeleton = e.mem.u32(t + 0x2c);
                    if bone >= e.mem.i32(skeleton + 0x1c) {
                        break;
                    }
                }
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let block = e.vcall(allocator, 4, &args![0x30u32]).u32();
            e.mem.set_u16(block + 4, 0x30);
            let skeleton = e.mem.u32(t + 0x2c);
            let instance = e
                .call(RAGDOLL_INSTANCE_CONSTRUCT, &args![block, y, z, skeleton, x])
                .u32();
            e.mem.set_u32(outputs[pass], instance);
            release_heap_ints(e, x);
            release_heap_ints(e, z);
            release_heap_ints(e, y);
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        if buffer_capacity as i32 >= 0 {
            e.vcall(
                allocator,
                0x10,
                &args![buffer, buffer_capacity & CAPACITY_MASK],
            );
        }
    })
}

/// Makes the array of 8-byte, 2-byte or 0x30-byte elements at `array` hold room for `wanted`: when
/// the capacity (bits 0..29) is smaller, `_reserve` (`00c908c0`) is asked on the router's `+0x10`
/// allocator for the larger of `wanted` and twice the capacity.
fn reserve_elements(e: &mut Engine, array: u32, wanted: i32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let grown = doubled.max(wanted);
        e.call(ARRAY_RESERVE, &args![allocator, array, grown, element_size]);
    }
}

// Translated from 00ceb2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollUtils::constructSkeletonForRagdoll` (Xbox PDB), stack `(bodies, constraints)` (two
/// `hkArray`s of pointers): answers the new `hkaSkeleton`. A 0x48-byte object is taken from the
/// router's `+0x10` allocator (size word `0x48` at `+4`, reference count 1 at `+6`, the vtable at
/// `010c4dec`), its name string (`+8`, `00cb0a60`) is constructed and its five arrays (parent
/// indexes `+0xc` of words, bones `+0x18` of 8 bytes, reference pose `+0x24` of 0x30 bytes, `+0x30`
/// and `+0x3c`, each empty with the capacity word `0x80000000`) are set. For `n` bodies the bones,
/// parents and pose get room (`_reserve`, `00c908c0`) and exactly `n` entries: extra bone strings
/// are destroyed (`00cb0c00`) and new ones constructed (`00cb0a60`, lock byte `+4` cleared). For each
/// body `i` the parent is the index of the body at `+0x18` of constraint `i - 1` among the bodies
/// before `i` (-1 for the root and when it is not found), stored as a 16-bit word. The body's
/// transform (`+0xf0`) becomes a `hkQsTransform` (`00ce5070`); for a body with a parent the pose is
/// relative to the parent: the translation is `2 * rotate(conj(q_parent), t_child) - 2 *
/// rotate^-1(q_parent, t_parent)` of the two transforms' parts, the rotation `conj(q_parent) *
/// q_child` (`setMul`, `00c66320`) and the scale the child's times the reciprocal of the parent's
/// (`RCPPS` with one Newton step); the root's pose is its own transform. The bone's name is the
/// body's name (`+0x74`, bit 0 masked, `00cb0c10`) or, with none, "Rigid Body name could not be
/// found" is reported (level 1, id `0xabbacc14`, line `0x22e`); the bone's lock byte is set for every
/// bone but the first. Last the skeleton's own name becomes a copy of the first bone's (`00cb0c30`).
pub fn hka_ragdoll_utils_construct_skeleton_for_ragdoll(
    e: &mut Engine,
    bodies: Ptr,
    constraints: Ptr,
) -> u32 {
    // The exe aligns ESP, then `SUB ESP,0x2b4` and three pushes: S is 0x2c0 bytes under the
    // boundary (00ceb2c3..00ceb2ce); the transforms at S + 0x60 and S + 0x90 (00ceb48f, 00ceb4a7).
    with_aligned_frame(e, 0x2c0, |e, frame| {
        let bodies = bodies.addr();
        let constraints = constraints.addr();
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let skeleton = e.vcall(allocator, 4, &args![0x48u32]).u32();
        e.mem.set_u16(skeleton + 6, 1);
        e.mem.set_u16(skeleton + 4, 0x48);
        e.mem.set_u32(skeleton, SKELETON_VTABLE);
        e.call(STRING_PTR_CONSTRUCT, &args![skeleton + 8]);
        for array in [0xc, 0x18, 0x24, 0x30, 0x3c] {
            e.mem.set_u32(skeleton + array, 0);
            e.mem.set_u32(skeleton + array + 4, 0);
            e.mem.set_u32(skeleton + array + 8, NOT_OWNED);
        }
        let n = e.mem.u32(bodies + 4);
        let n_signed = n as i32;
        // The bones: room for `n`, extra strings destroyed, new ones constructed.
        reserve_elements(e, skeleton + 0x18, n_signed, 8);
        let extra = e.mem.i32(skeleton + 0x1c).wrapping_sub(n_signed);
        let mut element = e.mem.u32(skeleton + 0x18).wrapping_add(n.wrapping_mul(8));
        if extra > 0 {
            for _ in 0..extra {
                e.call(STRING_PTR_DESTRUCT, &args![element]);
                element = element.wrapping_add(8);
            }
        }
        let size = e.mem.u32(skeleton + 0x1c);
        let missing = n_signed.wrapping_sub(size as i32);
        let mut element = e
            .mem
            .u32(skeleton + 0x18)
            .wrapping_add(size.wrapping_mul(8));
        if missing > 0 {
            for _ in 0..missing {
                if element != 0 {
                    e.call(STRING_PTR_CONSTRUCT, &args![element]);
                    e.mem.set_u8(element + 4, 0);
                }
                element = element.wrapping_add(8);
            }
        }
        e.mem.set_u32(skeleton + 0x1c, n);
        reserve_elements(e, skeleton + 0xc, n_signed, 2);
        e.mem.set_u32(skeleton + 0x10, n);
        reserve_elements(e, skeleton + 0x24, n_signed, 0x30);
        e.mem.set_u32(skeleton + 0x28, n);
        if n_signed > 0 {
            // The exe's frame: the child's `hkQsTransform` at `S + 0x60`, the parent's at `S + 0x90`,
            // the inverse of the parent's scale at `S + 0x50`, the conjugated rotation at `S + 0x40`.
            let mut index: i32 = 0;
            let mut offset: u32 = 0;
            loop {
                let mut parent: i32 = -1;
                if index != 0 {
                    let constraint = e.mem.u32(
                        e.mem
                            .u32(constraints)
                            .wrapping_add((index as u32).wrapping_mul(4))
                            .wrapping_sub(4),
                    );
                    let parent_body = e.mem.u32(constraint + 0x18);
                    if index > 0 {
                        let list = e.mem.u32(bodies);
                        for search in 0..index {
                            let candidate = e
                                .mem
                                .u32(list.wrapping_add((search as u32).wrapping_mul(4)));
                            if candidate == parent_body {
                                parent = search;
                                break;
                            }
                        }
                    }
                }
                let parents = e.mem.u32(skeleton + 0xc);
                e.mem.set_u16(
                    parents.wrapping_add((index as u32).wrapping_mul(2)),
                    parent as u16,
                );
                let list = e.mem.u32(bodies);
                let bones = e.mem.u32(skeleton + 0x18);
                let body = e.mem.u32(list.wrapping_add((index as u32).wrapping_mul(4)));
                let bone = bones.wrapping_add((index as u32).wrapping_mul(8));
                if parent >= 0 {
                    let parent_body = e
                        .mem
                        .u32(list.wrapping_add((parent as u32).wrapping_mul(4)));
                    e.call(
                        SET_FROM_TRANSFORM_NO_SCALE,
                        &args![frame + 0x60, body + 0xf0],
                    );
                    e.call(
                        SET_FROM_TRANSFORM_NO_SCALE,
                        &args![frame + 0x90, parent_body + 0xf0],
                    );
                    let half = load_vec(e, HALF_NEGATIVE);
                    let q_parent = load_vec(e, frame + 0xa0);
                    let t_parent = load_vec(e, frame + 0x90);
                    let pose = e.mem.u32(skeleton + 0x24).wrapping_add(offset);
                    let spun = rotation(q_parent, t_parent, half, true);
                    let negated = xorps(addps(spun, spun), load_bits(e, SIGN_MASK));
                    let conjugate = xorps(q_parent, load_bits(e, XYZ_SIGN_MASK));
                    let inverse = inverse_scale(e, load_vec(e, frame + 0xb0));
                    store_vec(e, frame + 0x50, inverse);
                    let t_child = load_vec(e, frame + 0x60);
                    store_vec(e, frame + 0x30, negated);
                    store_vec(e, frame + 0x40, conjugate);
                    let spun_child = rotation(conjugate, t_child, half, false);
                    let translation = addps(addps(spun_child, spun_child), negated);
                    store_vec(e, pose, translation);
                    e.call(
                        QUATERNION_SET_MUL,
                        &args![pose + 0x10, frame + 0x40, frame + 0x70],
                    );
                    let scale = mulps(load_vec(e, frame + 0x80), load_vec(e, frame + 0x50));
                    store_vec(e, pose + 0x20, scale);
                } else {
                    let pose = e.mem.u32(skeleton + 0x24).wrapping_add(offset);
                    e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![pose, body + 0xf0]);
                }
                let name = e.mem.u32(body + 0x74) & 0xffff_fffe;
                if name != 0 {
                    e.call(STRING_PTR_ASSIGN_TEXT, &args![bone, name]);
                } else {
                    with_builder(e, |e, builder, buffer| {
                        e.call(TEXT_BUILDER_APPEND, &args![builder, TEXT_NO_BODY_NAME]);
                        report(e, 0xabba_cc14, buffer, 0x22e);
                    });
                }
                e.mem.set_u8(bone + 4, u8::from(index != 0));
                offset = offset.wrapping_add(0x30);
                index += 1;
                if index >= n_signed {
                    break;
                }
            }
        }
        if e.mem.i32(skeleton + 0x1c) > 0 {
            let first_bone = e.mem.u32(skeleton + 0x18);
            e.call(STRING_PTR_ASSIGN, &args![skeleton + 8, first_bone]);
        }
        skeleton
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_9880, fn_00ce9880(Ptr, Ptr) -> u32),
        entry!(0x00ce_9ba0, fn_00ce9ba0(Ptr, u8) -> bool),
        entry!(0x00ce_9c40, fn_00ce9c40(Ptr) -> u32),
        entry!(0x00ce_9cf0, fn_00ce9cf0(Ptr) -> u32),
        entry!(0x00ce_9db0, fn_00ce9db0(Ptr) -> u32),
        entry!(0x00ce_9e90, fn_00ce9e90(Ptr) -> u32),
        entry!(
            0x00ce_a030,
            hka_ragdoll_utils_reorder_for_ragdoll(Ptr, Ptr) -> u32
        ),
        entry!(
            0x00ce_a810,
            hka_ragdoll_utils_create_ragdoll_instance_from_skeleton(Ptr, Ptr, Ptr) -> u32
        ),
        entry!(0x00ce_af60, fn_00ceaf60(Ptr, u32, u32, u32)),
        entry!(
            0x00ce_b2c0,
            hka_ragdoll_utils_construct_skeleton_for_ragdoll(Ptr, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const TYPE_SLOT: u32 = 0x0900_1000;
    const SINK_REPORT: u32 = 0x0900_200c;
    const SINK_SET_ENABLED: u32 = 0x0900_2010;
    const SINK_IS_ENABLED: u32 = 0x0900_2014;
    /// The slots of a fake allocator (`+0x10` of the router): allocate (`+4`), buffer (`+0xc`) and
    /// free (`+0x10`).
    const ALLOCATE_SLOT: u32 = 0x0900_3004;
    const BUFFER_SLOT: u32 = 0x0900_300c;
    const FREE_SLOT: u32 = 0x0900_3010;
    const SET_SHAPE_SLOT: u32 = 0x0900_4000;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
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

    fn answer(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that answers its first argument (`this`).
    fn answer_this(e: &mut Engine, address: u32) {
        e.register_double(address, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
    }

    fn words(e: &Engine, address: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|word| e.mem.u32(address + 4 * word))
            .collect()
    }

    fn bits(value: [f32; 4]) -> [u32; 4] {
        value.map(f32::to_bits)
    }

    /// Doubles answering the constraint types 0 to 10 at the fake slots `TYPE_SLOT + type`.
    fn install_types(e: &mut Engine) {
        for kind in 0..=10 {
            answer(e, TYPE_SLOT + kind, kind);
        }
    }

    /// A constraint whose data object has the given type; the data is 0x100 bytes.
    fn constraint(e: &mut Engine, kind: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, TYPE_SLOT + kind);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data, vtable);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0xc, data);
        constraint
    }

    fn data_of(e: &Engine, constraint: u32) -> u32 {
        e.mem.u32(constraint + 0xc)
    }

    /// The text builder doubles and a report sink whose slots `0xc`, `0x10` and `0x14` are
    /// `SINK_REPORT`, `SINK_SET_ENABLED` and `SINK_IS_ENABLED`; returns the sink.
    fn install_text(e: &mut Engine) -> u32 {
        for address in [
            TEXT_BUILDER_CONSTRUCT,
            TEXT_BUILDER_APPEND,
            TEXT_BUILDER_APPEND_INT,
            TEXT_BUILDER_APPEND_STRING_PTR,
            TEXT_BUILDER_DESTRUCT,
        ] {
            answer_this(e, address);
        }
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, SINK_REPORT);
        e.mem.set_u32(vtable + 0x10, SINK_SET_ENABLED);
        e.mem.set_u32(vtable + 0x14, SINK_IS_ENABLED);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(SINK, sink);
        answer(e, SINK_REPORT, 0);
        answer(e, SINK_SET_ENABLED, 0);
        e.register_double(SINK_IS_ENABLED, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        sink
    }

    /// The vector constants of the exe's data.
    fn install_vectors(e: &mut Engine) {
        store_vec(e, HALF_NEGATIVE, [-0.5; 4]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(
                XYZ_SIGN_MASK + 4 * lane,
                if lane < 3 { 0x8000_0000 } else { 0 },
            );
            e.mem
                .set_u32(XYZ_MASK + 4 * lane, if lane < 3 { 0xffff_ffff } else { 0 });
        }
        store_vec(e, ONES, [1.0; 4]);
        store_vec(e, IDENTITY_ROTATION, [0.0, 0.0, 0.0, 1.0]);
        e.mem.set_f32(ONE, 1.0);
        e.mem.set_f32(PIVOT_THRESHOLD, 1e-6);
        e.mem.set_f32(TOLERANCE, 0.001);
    }

    /// The report calls to the sink: the arguments after the sink itself.
    fn reports(e: &Engine) -> Vec<Vec<u32>> {
        calls(e, SINK_REPORT)
            .into_iter()
            .map(|args| args[1..].to_vec())
            .collect()
    }

    /// A memory router whose `+0x10` allocator has the fake slots `ALLOCATE_SLOT`, `BUFFER_SLOT`
    /// and `FREE_SLOT`; its bump pointer starts at a fresh block. Returns (router, allocator).
    fn install_router(e: &mut Engine) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        e.mem.set_u32(vtable + 0xc, BUFFER_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        answer(e, MEMORY_ROUTER, router);
        answer(e, FREE_SLOT, 0);
        (router, allocator)
    }

    #[test]
    fn supported_constraint_types_are_accepted_silently() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        for kind in [0, 1, 2, 6, 7, 8] {
            let c = constraint(&mut e, kind);
            assert!(fn_00ce9ba0(&mut e, Ptr::new(c), 1), "type {kind}");
        }
        assert!(reports(&e).is_empty());
        assert!(calls(&e, TEXT_BUILDER_CONSTRUCT).is_empty());
    }

    #[test]
    fn an_unsupported_constraint_type_is_reported_when_asked() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        for kind in [3, 4, 5, 9] {
            let c = constraint(&mut e, kind);
            assert!(!e.call(0x00ce_9ba0, &args![c, 0u32]).bool(), "type {kind}");
        }
        assert!(reports(&e).is_empty());
        let c = constraint(&mut e, 4);
        assert!(!e.call(0x00ce_9ba0, &args![c, 1u32]).bool());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        // The buffer is the 512 bytes after the 12-byte builder.
        let builder = calls(&e, TEXT_BUILDER_CONSTRUCT)[0][0];
        assert_eq!(
            sent[0],
            vec![1, 0xabba_bf3b, builder + 0xc, SOURCE_FILE, 0x81]
        );
        assert_eq!(
            calls(&e, TEXT_BUILDER_CONSTRUCT),
            vec![vec![builder, builder + 0xc, 0x200]]
        );
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_UNSUPPORTED]]
        );
        assert_eq!(calls(&e, TEXT_BUILDER_DESTRUCT), vec![vec![builder]]);
    }

    #[test]
    fn the_pivots_sit_at_a_type_dependent_offset_of_the_data() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        for (kind, first, second) in [
            (0u32, Some(0x20u32), Some(0x30u32)),
            (1, Some(0x50), Some(0x90)),
            (2, Some(0x50), Some(0x90)),
            (6, Some(0x50), Some(0x90)),
            (7, Some(0x50), Some(0x90)),
            (8, Some(0x20), Some(0x20)),
            (3, None, None),
            (9, None, None),
        ] {
            let c = constraint(&mut e, kind);
            let data = data_of(&e, c);
            assert_eq!(
                e.call(0x00ce_9c40, &args![c]).u32(),
                first.map_or(0, |offset| data + offset),
                "first pivot of type {kind}"
            );
            assert_eq!(
                e.call(0x00ce_9cf0, &args![c]).u32(),
                second.map_or(0, |offset| data + offset),
                "second pivot of type {kind}"
            );
        }
        // The two unsupported types of each accessor were reported: lines 0xb4 and 0xe6, the
        // second with the text that ends in a full stop.
        let sent = reports(&e);
        assert_eq!(sent.len(), 4);
        assert_eq!(sent[0][4], 0xb4);
        assert_eq!(sent[1][4], 0xe6);
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND)
                .iter()
                .map(|call| call[1])
                .collect::<Vec<_>>(),
            vec![
                TEXT_UNSUPPORTED,
                TEXT_UNSUPPORTED_STOP,
                TEXT_UNSUPPORTED,
                TEXT_UNSUPPORTED_STOP
            ]
        );
        assert!(sent.iter().all(|report| report[1] == 0xabba_bf3b));
    }

    /// The doubles `moveRigidBodyPivot` needs, logging what it passes on. The body's own transform
    /// is translation (1, 2, 3, 0), rotation (0, 0, 0, 1) and scale (2, 3, 4, 0); `equal` is the
    /// answer of the approximate compare. The transform to move by is translation (10, 20, 30, 0),
    /// rotation (0, 0, 0, 1) and scale (5, 6, 7, 1).
    struct Pivot {
        body: u32,
        transform: u32,
        logs: Log,
    }

    fn pivot_setup(e: &mut Engine, equal: u8, motion_type: u8) -> Pivot {
        install_vectors(e);
        install_text(e);
        let logs: Log = Rc::new(RefCell::new(Vec::new()));
        let compare = Rc::clone(&logs);
        e.register_double(IS_APPROXIMATELY_EQUAL, move |e, a| {
            e.mem.set_u8(a[1], equal);
            compare.borrow_mut().push(("equal", a.to_vec()));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(SET_FROM_TRANSFORM_NO_SCALE, |e, a| {
            store_vec(e, a[0], [1.0, 2.0, 3.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(e, a[0] + 0x20, [2.0, 3.0, 4.0, 0.0]);
            Ret::default()
        });
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            store_vec(e, a[0], [7.0, 8.0, 9.0, 10.0]);
            Ret::default()
        });
        let conversions = Rc::clone(&logs);
        e.register_double(QS_TO_TRANSFORM, move |e, a| {
            let mut seen = words(e, a[0], 12);
            seen.push(a[1]);
            conversions.borrow_mut().push(("to_transform", seen));
            Ret::default()
        });
        install_router(e);
        let shape_block = e.mem.alloc(0x80);
        answer(e, ALLOCATE_SLOT, shape_block);
        let shapes = Rc::clone(&logs);
        e.register_double(TRANSFORM_SHAPE_CONSTRUCT, move |_, a| {
            shapes.borrow_mut().push(("shape", a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        for (name, address) in [
            ("remove", REMOVE_REFERENCE),
            ("first_options", OPTIONS_INIT_FIRST),
            ("second_options", OPTIONS_INIT_SECOND),
            ("collapse", COLLAPSE_TRANSFORMS),
            ("set_transform", SET_TRANSFORM),
            ("inverse_pos", SET_TRANSFORMED_INVERSE_POS),
            ("centre", SET_CENTER_OF_MASS_LOCAL),
            ("set_shape", SET_SHAPE_SLOT),
        ] {
            let log = Rc::clone(&logs);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        let body_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(body_vtable + 0xc, SET_SHAPE_SLOT);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(body, body_vtable);
        e.mem.set_u32(body + 0x10, 0x00ab_cdef);
        e.mem.set_u8(body + 0xe8, motion_type);
        store_vec(e, body + 0x140, [0.5, 1.5, 2.5, 3.5]);
        let transform = e.mem.alloc(0x40);
        store_vec(e, transform, [10.0, 20.0, 30.0, 0.0]);
        store_vec(e, transform + 0x10, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, transform + 0x20, [5.0, 6.0, 7.0, 1.0]);
        Pivot {
            body,
            transform,
            logs,
        }
    }

    #[test]
    fn an_identity_pivot_leaves_the_body_alone() {
        let mut e = engine();
        let pivot = pivot_setup(&mut e, 1, 0);
        let got = e
            .call(0x00ce_9880, &args![pivot.transform, pivot.body])
            .u32();
        assert_eq!(got, 0);
        let logs = pivot.logs.borrow();
        assert_eq!(logs.len(), 1);
        // The compare got `this`, the result byte, the identity transform and the tolerance bits.
        assert_eq!(logs[0].1[0], pivot.transform);
        assert_eq!(logs[0].1[2], IDENTITY_TRANSFORM);
        assert_eq!(logs[0].1[3], 0.001f32.to_bits());
        assert!(calls(&e, SINK_IS_ENABLED).is_empty());
    }

    /// The frame base `P` the compare was given the result byte of (`P + 0xf`).
    fn frame_of(pivot: &Pivot) -> u32 {
        pivot.logs.borrow()[0].1[1] - 0xf
    }

    fn find(pivot: &Pivot, name: &str) -> Vec<Vec<u32>> {
        pivot
            .logs
            .borrow()
            .iter()
            .filter(|(tag, _)| *tag == name)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn moving_the_pivot_wraps_the_body_in_a_transform_shape() {
        let mut e = engine();
        let pivot = pivot_setup(&mut e, 0, 0);
        let (body, transform) = (pivot.body, pivot.transform);
        let got = e.call(0x00ce_9880, &args![transform, body]).u32();
        assert_eq!(got, 0);
        let p = frame_of(&pivot);
        // The two conversions: the inverse (translation -t, the conjugate and the reciprocal scale)
        // and the new transform (translation 2 * (t / 2) + (1, 2, 3), the rotation from setMul and
        // the product of the scales).
        let conversions = find(&pivot, "to_transform");
        assert_eq!(conversions.len(), 2);
        let inverse = &conversions[0];
        assert_eq!(inverse[0..4], bits([-10.0, -20.0, -30.0, -0.0]));
        assert_eq!(inverse[4..8], bits([-0.0, -0.0, -0.0, 1.0]));
        let reciprocal = |x: f32| refined_reciprocal(x).to_bits();
        assert_eq!(
            inverse[8..12],
            [reciprocal(5.0), reciprocal(6.0), reciprocal(7.0), 0]
        );
        assert_eq!(inverse[12], p + 0xe0);
        let converted = &conversions[1];
        assert_eq!(converted[0..4], bits([11.0, 22.0, 33.0, 0.0]));
        assert_eq!(converted[4..8], bits([7.0, 8.0, 9.0, 10.0]));
        assert_eq!(converted[8..12], bits([10.0, 18.0, 28.0, 0.0]));
        assert_eq!(converted[12], p + 0xe0);
        // The rotation product is `setMul(out, body rotation, transform rotation)`.
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL),
            vec![vec![p + 0xc0, p + 0x60, transform + 0x10]]
        );
        // The shape: a 0x70-byte block with its size word, the old shape and the inverse transform.
        let shapes = find(&pivot, "shape");
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0][1..], [0x00ab_cdef, p + 0xe0]);
        let block = shapes[0][0];
        assert_eq!(e.mem.u16(block + 4), 0x70);
        assert_eq!(find(&pivot, "set_shape"), vec![vec![body, block]]);
        assert_eq!(find(&pivot, "remove"), vec![vec![block]]);
        // The options and the collapse.
        assert_eq!(find(&pivot, "first_options"), vec![vec![p + 0x30]]);
        assert_eq!(find(&pivot, "second_options"), vec![vec![p + 0x10]]);
        assert_eq!(
            find(&pivot, "collapse"),
            vec![vec![body, p + 0x30, p + 0x10]]
        );
        assert_eq!(find(&pivot, "set_transform"), vec![vec![body, p + 0xe0]]);
        // The centre of mass: the 4 words at +0x140 were copied to P + 0x20 for the inverse move.
        assert_eq!(words(&e, p + 0x20, 4), bits([0.5, 1.5, 2.5, 3.5]).to_vec());
        assert_eq!(
            find(&pivot, "inverse_pos"),
            vec![vec![p + 0x10, p + 0xe0, p + 0x20]]
        );
        assert_eq!(find(&pivot, "centre"), vec![vec![body, p + 0x10]]);
    }

    #[test]
    fn the_pivot_frame_sits_0x124_bytes_under_a_16_byte_boundary() {
        let mut e = engine();
        let pivot = pivot_setup(&mut e, 1, 0);
        let raw = e.next_stack_local(0x134);
        e.call(0x00ce_9880, &args![pivot.transform, pivot.body]);
        let p = frame_of(&pivot);
        assert_eq!(p, aligned_frame_at(raw, 0x124));
        assert_eq!((p + 0x124) & 0xf, 0);
    }

    #[test]
    fn the_report_sink_is_silenced_around_the_collapse_and_restored() {
        let mut e = engine();
        let pivot = pivot_setup(&mut e, 0, 0);
        e.call(0x00ce_9880, &args![pivot.transform, pivot.body]);
        let p = frame_of(&pivot);
        let sink = e.mem.u32(SINK);
        assert_eq!(
            calls(&e, SINK_IS_ENABLED),
            vec![vec![sink, p + 0xc, 0x2ff8_c16f]]
        );
        // First turned off with the pointer's upper three bytes as the flag word, then restored with
        // the dword of the frame (the answered byte 1, the stale bytes and the compare's result 0).
        assert_eq!(
            calls(&e, SINK_SET_ENABLED),
            vec![
                vec![sink, 0x2ff8_c16f, sink & 0xffff_ff00],
                vec![sink, 0x2ff8_c16f, 1]
            ]
        );
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [SINK_IS_ENABLED, SINK_SET_ENABLED, COLLAPSE_TRANSFORMS].contains(address)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                SINK_IS_ENABLED,
                SINK_SET_ENABLED,
                COLLAPSE_TRANSFORMS,
                SINK_SET_ENABLED
            ]
        );
    }

    #[test]
    fn a_body_with_motion_type_five_keeps_its_centre_of_mass() {
        let mut e = engine();
        let pivot = pivot_setup(&mut e, 0, 5);
        e.call(0x00ce_9880, &args![pivot.transform, pivot.body]);
        assert_eq!(find(&pivot, "set_transform").len(), 1);
        assert!(find(&pivot, "inverse_pos").is_empty());
        assert!(find(&pivot, "centre").is_empty());
    }

    /// An `hkArray` header (data, size, capacity word) over a copy of `items`; the data buffer has
    /// room for `room` words.
    fn array(e: &mut Engine, items: &[u32], capacity_word: u32, room: u32) -> u32 {
        let data = e.mem.alloc(4 * room.max(items.len() as u32).max(1));
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * index as u32, *item);
        }
        let header = e.mem.alloc(12);
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, items.len() as u32);
        e.mem.set_u32(header + 8, capacity_word);
        header
    }

    /// A list of constraints or bodies with room for them all.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        array(e, items, items.len() as u32, 0)
    }

    /// The words of an `hkArray` (size at `+4`, data at `+0`).
    fn contents(e: &Engine, header: u32) -> Vec<u32> {
        words(e, e.mem.u32(header), e.mem.u32(header + 4))
    }

    #[test]
    fn pivots_above_the_threshold_are_moved_and_unsupported_constraints_counted() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        install_vectors(&mut e);
        e.register_double(IS_APPROXIMATELY_EQUAL, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let unsupported = constraint(&mut e, 3);
        let small = constraint(&mut e, 0);
        let big = constraint(&mut e, 0);
        let body = e.mem.alloc(0x200);
        for c in [unsupported, small, big] {
            e.mem.set_u32(c + 0x14, body);
        }
        {
            let at = data_of(&e, small) + 0x20;
            store_vec(&mut e, at, [0.0, 0.0, 0.0005, 0.0]);
        }
        {
            let at = data_of(&e, big) + 0x20;
            store_vec(&mut e, at, [1.0, 2.0, 3.0, 0.0]);
        }
        let constraints = list(&mut e, &[unsupported, small, big]);
        assert_eq!(e.call(0x00ce_9db0, &args![constraints]).u32(), 1);
        // The 0.0005 pivot has a squared length of 2.5e-7, below 1e-6: not moved. Only the pivot
        // (1, 2, 3) reached the compare, as the translation of a transform with the identity
        // rotation and unit scale.
        let compares = calls(&e, IS_APPROXIMATELY_EQUAL);
        assert_eq!(compares.len(), 1);
        let transform = compares[0][0];
        assert_eq!(words(&e, transform, 4), bits([1.0, 2.0, 3.0, 0.0]).to_vec());
        assert_eq!(
            words(&e, transform + 0x10, 4),
            bits([0.0, 0.0, 0.0, 1.0]).to_vec()
        );
        assert_eq!(words(&e, transform + 0x20, 4), vec![0x3f80_0000; 4]);
        // The unsupported constraint was reported once, with the flag on.
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0][4], 0x81);
    }

    #[test]
    fn supported_constraints_with_short_or_nan_pivots_move_nothing() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        install_vectors(&mut e);
        let tiny = constraint(&mut e, 1);
        let nan = constraint(&mut e, 2);
        let edge = constraint(&mut e, 8);
        let body = e.mem.alloc(0x200);
        for c in [tiny, nan, edge] {
            e.mem.set_u32(c + 0x14, body);
        }
        {
            let at = data_of(&e, tiny) + 0x50;
            store_vec(&mut e, at, [0.0, 0.0, 0.0, 0.0]);
        }
        {
            let at = data_of(&e, nan) + 0x50;
            store_vec(&mut e, at, [f32::NAN, 1.0, 1.0, 0.0]);
        }
        // 1e-3 squared is 1e-6 up to rounding: the exe compares strictly.
        e.mem.set_f32(PIVOT_THRESHOLD, 1.0);
        {
            let at = data_of(&e, edge) + 0x20;
            store_vec(&mut e, at, [1.0, 0.0, 0.0, 0.0]);
        }
        let constraints = list(&mut e, &[tiny, nan, edge]);
        assert_eq!(e.call(0x00ce_9db0, &args![constraints]).u32(), 0);
        assert!(calls(&e, IS_APPROXIMATELY_EQUAL).is_empty());
        assert!(reports(&e).is_empty());
        // An empty list answers 0 too.
        let none = list(&mut e, &[]);
        assert_eq!(e.call(0x00ce_9db0, &args![none]).u32(), 0);
    }

    #[test]
    fn the_pivot_transform_sits_at_0x30_of_a_0x60_byte_aligned_frame() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        install_vectors(&mut e);
        e.register_double(IS_APPROXIMATELY_EQUAL, |e, a| {
            e.mem.set_u8(a[1], 1);
            a[1].into_ret()
        });
        let big = constraint(&mut e, 0);
        let body = e.mem.alloc(0x200);
        e.mem.set_u32(big + 0x14, body);
        {
            let at = data_of(&e, big) + 0x20;
            store_vec(&mut e, at, [1.0, 2.0, 3.0, 0.0]);
        }
        let constraints = list(&mut e, &[big]);
        let raw = e.next_stack_local(0x70);
        e.call(0x00ce_9db0, &args![constraints]);
        let compares = calls(&e, IS_APPROXIMATELY_EQUAL);
        assert_eq!(compares[0][0], aligned_frame_at(raw, 0x60) + 0x30);
    }

    /// Records the inverse position calls: (arguments, the four words at the position).
    fn record_inverse_pos(e: &mut Engine) -> Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORMED_INVERSE_POS, move |e, a| {
            let position = words(e, a[2], 4);
            sink.borrow_mut().push((a.to_vec(), position));
            Ret::default()
        });
        log
    }

    #[test]
    fn aligning_a_constraint_clears_pivot_a_and_moves_pivot_b_into_the_second_bodys_space() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        let log = record_inverse_pos(&mut e);
        let c = constraint(&mut e, 0);
        let (body_a, body_b) = (e.mem.alloc(0x200), e.mem.alloc(0x200));
        e.mem.set_u32(c + 0x14, body_a);
        e.mem.set_u32(c + 0x18, body_b);
        store_vec(&mut e, body_a + 0x120, [1.0, 2.0, 3.0, 4.0]);
        let data = data_of(&e, c);
        store_vec(&mut e, data + 0x20, [9.0, 9.0, 9.0, 9.0]);
        let constraints = list(&mut e, &[c]);
        assert_eq!(e.call(0x00ce_9e90, &args![constraints]).u32(), 0);
        assert_eq!(words(&e, data + 0x20, 4), vec![0; 4]);
        let seen = log.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[..2], [data + 0x30, body_b + 0xf0]);
        assert_eq!(seen[0].1, bits([1.0, 2.0, 3.0, 4.0]).to_vec());
        assert!(reports(&e).is_empty());
    }

    #[test]
    fn the_aligned_vector_sits_at_0x10_of_a_0x230_byte_aligned_frame() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        let log = record_inverse_pos(&mut e);
        let c = constraint(&mut e, 0);
        let (body_a, body_b) = (e.mem.alloc(0x200), e.mem.alloc(0x200));
        e.mem.set_u32(c + 0x14, body_a);
        e.mem.set_u32(c + 0x18, body_b);
        let constraints = list(&mut e, &[c]);
        let raw = e.next_stack_local(0x240);
        e.call(0x00ce_9e90, &args![constraints]);
        assert_eq!(log.borrow()[0].0[2], aligned_frame_at(raw, 0x230) + 0x10);
    }

    #[test]
    fn aligning_a_stiff_spring_is_reported_as_unusual() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        let log = record_inverse_pos(&mut e);
        let c = constraint(&mut e, 8);
        let (body_a, body_b) = (e.mem.alloc(0x200), e.mem.alloc(0x200));
        e.mem.set_u32(c + 0x14, body_a);
        e.mem.set_u32(c + 0x18, body_b);
        let constraints = list(&mut e, &[c]);
        e.call(0x00ce_9e90, &args![constraints]);
        // Both pivots are at `data + 0x20`: the inverse move targets the cleared pivot.
        let data = data_of(&e, c);
        assert_eq!(log.borrow()[0].0[0], data + 0x20);
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0][1], 0xabba_876d);
        assert_eq!(sent[0][4], 0x134);
        let builder = calls(&e, TEXT_BUILDER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_STIFF_SPRING]]
        );
        assert_eq!(calls(&e, TEXT_BUILDER_DESTRUCT).len(), 1);
    }

    #[test]
    fn aligning_an_unsupported_constraint_only_reports_the_type() {
        let mut e = engine();
        install_types(&mut e);
        install_text(&mut e);
        let log = record_inverse_pos(&mut e);
        let c = constraint(&mut e, 3);
        let constraints = list(&mut e, &[c]);
        assert_eq!(e.call(0x00ce_9e90, &args![constraints]).u32(), 0);
        assert!(log.borrow().is_empty());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0][1], 0xabba_bf3b);
        assert_eq!(sent[0][4], 0x81);
    }

    /// Doubles for the router, the allocator buffer slot (answers a buffer of `requested + 4`
    /// bytes and writes that size back) and the growth routine (a 16-word buffer).
    fn install_arrays(e: &mut Engine) -> (u32, u32) {
        let (router, allocator) = install_router(e);
        e.register_double(BUFFER_SLOT, |e, a| {
            let requested = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], requested + 4);
            Ret {
                eax: e.mem.alloc(requested + 4),
                ..Ret::default()
            }
        });
        e.register_double(ARRAY_GROW, |e, a| {
            let buffer = e.mem.alloc(0x40);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        (router, allocator)
    }

    /// A constraint between two bodies (plain numbers: they are only compared).
    fn link(e: &mut Engine, first: u32, second: u32) -> u32 {
        let link = e.mem.alloc(0x20);
        e.mem.set_u32(link + 0x14, first);
        e.mem.set_u32(link + 0x18, second);
        link
    }

    #[test]
    fn reordering_sorts_the_bodies_by_depth_and_their_constraints_with_them() {
        let mut e = engine();
        install_text(&mut e);
        let (router, _) = install_arrays(&mut e);
        let before = e.mem.u32(router);
        let (b0, b1, b2) = (0x1000, 0x2000, 0x3000);
        // b1 hangs off b0 and b2 off b1, but the bodies come in the order b2, b0, b1.
        let ca = link(&mut e, b1, b0);
        let cb = link(&mut e, b2, b1);
        let bodies = array(&mut e, &[b2, b0, b1], 4, 4);
        let constraints = array(&mut e, &[ca, cb], 4, 4);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 0);
        assert_eq!(contents(&e, bodies), vec![b0, b1, b2]);
        assert_eq!(contents(&e, constraints), vec![ca, cb]);
        assert_eq!(e.mem.u32(router), before);
        assert!(calls(&e, BUFFER_SLOT).is_empty());
        assert!(calls(&e, FREE_SLOT).is_empty());
        assert!(reports(&e).is_empty());
    }

    #[test]
    fn reordering_a_sorted_skeleton_changes_nothing() {
        let mut e = engine();
        install_text(&mut e);
        let (router, _) = install_arrays(&mut e);
        let before = e.mem.u32(router);
        let (b0, b1, b2) = (0x1000, 0x2000, 0x3000);
        let c0 = link(&mut e, b1, b0);
        let c1 = link(&mut e, b2, b1);
        let bodies = array(&mut e, &[b0, b1, b2], 3, 3);
        let constraints = array(&mut e, &[c0, c1], 2, 2);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 0);
        assert_eq!(contents(&e, bodies), vec![b0, b1, b2]);
        assert_eq!(contents(&e, constraints), vec![c0, c1]);
        assert_eq!(e.mem.u32(router), before);
    }

    #[test]
    fn reordering_a_siblings_first_skeleton_orders_the_deeper_chain_last() {
        let mut e = engine();
        install_text(&mut e);
        install_arrays(&mut e);
        let (b0, b1, b2, b3) = (0x1000, 0x2000, 0x3000, 0x4000);
        // b0 is the root; b3 hangs off b2, which hangs off b0; b1 off b0: depths 0, 1, 1, 2 for the
        // bodies given in the order b3, b2, b1, b0.
        let c_b3 = link(&mut e, b3, b2);
        let c_b2 = link(&mut e, b2, b0);
        let c_b1 = link(&mut e, b1, b0);
        let bodies = array(&mut e, &[b3, b2, b1, b0], 4, 4);
        let constraints = array(&mut e, &[c_b3, c_b2, c_b1], 3, 3);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 0);
        // Selection sort by depth with swaps: depths (2, 1, 1, 0) become (0, 1, 1, 2) in the order
        // b0, then b1 (swapped in from index 2), then b2, then b3.
        assert_eq!(contents(&e, bodies), vec![b0, b1, b2, b3]);
        assert_eq!(contents(&e, constraints), vec![c_b1, c_b2, c_b3]);
    }

    #[test]
    fn a_wrong_constraint_count_is_reported_and_fails() {
        let mut e = engine();
        install_text(&mut e);
        let (router, _) = install_arrays(&mut e);
        let before = e.mem.u32(router);
        let c = link(&mut e, 0x1000, 0x2000);
        let bodies = array(&mut e, &[0x1000, 0x2000, 0x3000], 3, 3);
        let constraints = array(&mut e, &[c], 1, 1);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 1);
        let builder = calls(&e, TEXT_BUILDER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND),
            vec![
                vec![builder, TEXT_EXPECTING],
                vec![builder, TEXT_CONSTRAINTS_NOT],
                vec![builder, TEXT_CANT_CREATE]
            ]
        );
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND_INT),
            vec![vec![builder, 2], vec![builder, 1]]
        );
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0][1], 0xabba_54bb);
        assert_eq!(sent[0][4], 0x144);
        assert_eq!(e.mem.u32(router), before);
        assert_eq!(contents(&e, bodies), vec![0x1000, 0x2000, 0x3000]);
    }

    #[test]
    fn a_constraint_to_an_unknown_body_or_a_second_parent_fails() {
        for second_parent in [false, true] {
            let mut e = engine();
            install_text(&mut e);
            let (router, _) = install_arrays(&mut e);
            let before = e.mem.u32(router);
            let (b0, b1, b2) = (0x1000, 0x2000, 0x3000);
            let first = link(&mut e, b1, b0);
            let second = if second_parent {
                link(&mut e, b1, b2)
            } else {
                link(&mut e, 0x9999, b1)
            };
            let bodies = array(&mut e, &[b0, b1, b2], 3, 3);
            let constraints = array(&mut e, &[first, second], 2, 2);
            assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 1);
            assert_eq!(e.mem.u32(router), before);
            assert_eq!(contents(&e, bodies), vec![b0, b1, b2]);
            assert_eq!(contents(&e, constraints), vec![first, second]);
        }
    }

    #[test]
    fn a_missing_body_in_the_first_constraint_fails_too() {
        let mut e = engine();
        install_text(&mut e);
        let (router, _) = install_arrays(&mut e);
        let before = e.mem.u32(router);
        let bad = link(&mut e, 0x1000, 0x7777);
        let ok = link(&mut e, 0x2000, 0x1000);
        let bodies = array(&mut e, &[0x1000, 0x2000, 0x3000], 3, 3);
        let constraints = array(&mut e, &[bad, ok], 2, 2);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 1);
        assert_eq!(e.mem.u32(router), before);
    }

    #[test]
    fn short_arrays_are_replaced_through_the_allocator_when_the_result_is_copied_back() {
        let mut e = engine();
        install_text(&mut e);
        let (router, allocator) = install_arrays(&mut e);
        let before = e.mem.u32(router);
        let (b0, b1, b2) = (0x1000, 0x2000, 0x3000);
        let c0 = link(&mut e, b1, b0);
        let c1 = link(&mut e, b2, b1);
        // The bodies' buffer is owned and holds 2 words: too small for 3. The constraints' buffer is
        // not owned (bit 31) and holds 1: too small for 2, and not freed.
        let bodies = array(&mut e, &[b0, b1, b2], 2, 3);
        let old_bodies = e.mem.u32(bodies);
        let constraints = array(&mut e, &[c0, c1], 0x8000_0001, 2);
        assert_eq!(e.call(0x00ce_a030, &args![bodies, constraints]).u32(), 0);
        assert_eq!(calls(&e, FREE_SLOT), vec![vec![allocator, old_bodies, 8]]);
        let asked = calls(&e, BUFFER_SLOT);
        assert_eq!(asked.len(), 2);
        // The new buffers are 16 and 12 bytes (the double adds 4): capacities 4 and 3.
        assert_eq!(e.mem.u32(bodies + 8), 4);
        assert_eq!(e.mem.u32(constraints + 8), 3);
        assert_ne!(e.mem.u32(bodies), old_bodies);
        assert_eq!(contents(&e, bodies), vec![b0, b1, b2]);
        assert_eq!(contents(&e, constraints), vec![c0, c1]);
        assert_eq!(e.mem.u32(router), before);
    }

    /// A NUL-terminated string in memory.
    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let at = e.mem.alloc(value.len() as u32 + 1);
        e.mem.set_cstr(at, value);
        at
    }

    /// A rigid body with a name (or none) at `+0x74`.
    fn rigid_body(e: &mut Engine, name: Option<&[u8]>) -> u32 {
        let body = e.mem.alloc(0x200);
        if let Some(name) = name {
            let at = text(e, name);
            e.mem.set_u32(body + 0x74, at);
        }
        body
    }

    /// A skeleton with the bone names (a zero word for none) and the parent words.
    fn make_skeleton(e: &mut Engine, names: &[Option<&[u8]>], parents: &[u16]) -> u32 {
        let skeleton = e.mem.alloc(0x50);
        e.mem.set_u32(skeleton + 0x1c, names.len() as u32);
        let bones = e.mem.alloc(8 * names.len() as u32);
        for (index, name) in names.iter().enumerate() {
            if let Some(name) = name {
                let at = text(e, name);
                e.mem.set_u32(bones + 8 * index as u32, at);
            }
        }
        e.mem.set_u32(skeleton + 0x18, bones);
        let table = e.mem.alloc(2 * parents.len() as u32);
        for (index, parent) in parents.iter().enumerate() {
            e.mem.set_u16(table + 2 * index as u32, *parent);
        }
        e.mem.set_u32(skeleton + 0xc, table);
        skeleton
    }

    /// The doubles `createRagdollInstanceFromSkeleton` needs: the router, `strCmp` on the strings,
    /// the growth routine and the instance constructor, which records the three arrays it is given
    /// (bodies, constraints, parents) and answers `0x7777`.
    type Built = Rc<RefCell<Vec<(u32, Vec<u32>, Vec<u32>, Vec<u32>)>>>;

    fn instance_setup(e: &mut Engine) -> (u32, Built) {
        install_text(e);
        let (router, _) = install_arrays(e);
        e.register_double(STR_CMP, |e, a| Ret {
            eax: u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])),
            ..Ret::default()
        });
        let built: Built = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&built);
        e.register_double(RAGDOLL_INSTANCE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((
                a[3],
                contents(e, a[1]),
                contents(e, a[2]),
                contents(e, a[4]),
            ));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        (router, built)
    }

    #[test]
    fn an_instance_is_made_from_the_matching_bodies_and_constraints() {
        let mut e = engine();
        let (router, built) = instance_setup(&mut e);
        let block = e.mem.alloc(0x40);
        answer(&mut e, ALLOCATE_SLOT, block);
        let before = e.mem.u32(router);
        let names: [&[u8]; 3] = [b"root", b"spine", b"head"];
        let skeleton = make_skeleton(&mut e, &names.map(Some), &[0xffff, 0, 1]);
        let bodies_list: Vec<u32> = names
            .iter()
            .map(|name| rigid_body(&mut e, Some(*name)))
            .collect();
        let c1 = link(&mut e, bodies_list[1], bodies_list[0]);
        let c2 = link(&mut e, bodies_list[2], bodies_list[1]);
        let bodies = list(&mut e, &bodies_list);
        let constraints = list(&mut e, &[c1, c2]);
        let got = e
            .call(0x00ce_a810, &args![skeleton, bodies, constraints])
            .u32();
        assert_eq!(got, 0x7777);
        let made = built.borrow();
        assert_eq!(made.len(), 1);
        // The parent indexes hold the position of each body among the found ones.
        assert_eq!(
            made[0],
            (skeleton, bodies_list.clone(), vec![c1, c2], vec![0, 1, 2])
        );
        assert_eq!(e.mem.u16(block + 4), 0x30);
        assert_eq!(e.mem.u32(router), before);
        assert!(reports(&e).is_empty());
    }

    #[test]
    fn duplicated_names_and_constraints_are_reported_and_the_first_one_is_used() {
        let mut e = engine();
        let (router, built) = instance_setup(&mut e);
        let block = e.mem.alloc(0x40);
        answer(&mut e, ALLOCATE_SLOT, block);
        let before = e.mem.u32(router);
        let skeleton = make_skeleton(&mut e, &[Some(b"root"), Some(b"leg")], &[0xffff, 0]);
        let root = rigid_body(&mut e, Some(b"root"));
        let root_again = rigid_body(&mut e, Some(b"root"));
        let leg = rigid_body(&mut e, Some(b"leg"));
        let first = link(&mut e, leg, root);
        let second = link(&mut e, leg, root);
        let bodies = list(&mut e, &[root, root_again, leg]);
        let constraints = list(&mut e, &[first, second]);
        let got = e
            .call(0x00ce_a810, &args![skeleton, bodies, constraints])
            .u32();
        assert_eq!(got, 0x7777);
        assert_eq!(
            built.borrow()[0],
            (skeleton, vec![root, leg], vec![first], vec![0, 1])
        );
        let sent = reports(&e);
        assert_eq!(sent.len(), 2);
        assert_eq!((sent[0][1], sent[0][4]), (0xabba_98aa, 0x26b));
        assert_eq!((sent[1][1], sent[1][4]), (0xabba_82ff, 0x291));
        // "Some RBS have duplicated names : " and the name; "Found more than one constraint between
        // rigid bodies " the first body's name, " and " and the second's.
        let appended = calls(&e, TEXT_BUILDER_APPEND);
        let name_of = |body: u32| e.mem.u32(body + 0x74);
        assert_eq!(
            appended.iter().map(|call| call[1]).collect::<Vec<_>>(),
            vec![
                TEXT_DUPLICATE_BODY,
                name_of(root_again),
                TEXT_MORE_THAN_ONE,
                name_of(leg),
                TEXT_AND,
                name_of(root)
            ]
        );
        assert_eq!(e.mem.u32(router), before);
    }

    #[test]
    fn a_bone_without_a_name_fails_the_creation() {
        let mut e = engine();
        let (router, built) = instance_setup(&mut e);
        let before = e.mem.u32(router);
        let skeleton = make_skeleton(&mut e, &[Some(b"root"), None], &[0xffff, 0]);
        let root = rigid_body(&mut e, Some(b"root"));
        let bodies = list(&mut e, &[root]);
        let constraints = list(&mut e, &[]);
        assert_eq!(
            e.call(0x00ce_a810, &args![skeleton, bodies, constraints])
                .u32(),
            0
        );
        assert!(built.borrow().is_empty());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!((sent[0][1], sent[0][4]), (0xabba_e6a2, 0x2a3));
        assert_eq!(calls(&e, TEXT_BUILDER_APPEND)[0][1], TEXT_NO_BONE_NAME);
        assert_eq!(e.mem.u32(router), before);
    }

    #[test]
    fn a_skeleton_without_matching_bodies_fails_the_creation() {
        let mut e = engine();
        let (router, built) = instance_setup(&mut e);
        let before = e.mem.u32(router);
        let skeleton = make_skeleton(&mut e, &[Some(b"root"), Some(b"leg")], &[0xffff, 0]);
        let stray = rigid_body(&mut e, Some(b"other"));
        let nameless = rigid_body(&mut e, None);
        let bodies = list(&mut e, &[stray, nameless]);
        let constraints = list(&mut e, &[]);
        assert_eq!(
            e.call(0x00ce_a810, &args![skeleton, bodies, constraints])
                .u32(),
            0
        );
        assert!(built.borrow().is_empty());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!((sent[0][1], sent[0][4]), (0xabba_e6e4, 0x2aa));
        assert_eq!(calls(&e, TEXT_BUILDER_APPEND)[0][1], TEXT_NO_BODIES);
        assert_eq!(e.mem.u32(router), before);
        // A skeleton with no bones fails the same way, with no body asked for.
        let empty = skeleton_empty(&mut e);
        assert_eq!(
            e.call(0x00ce_a810, &args![empty, bodies, constraints])
                .u32(),
            0
        );
        assert_eq!(reports(&e).len(), 2);
    }

    fn skeleton_empty(e: &mut Engine) -> u32 {
        make_skeleton(e, &[], &[])
    }

    #[test]
    fn a_bone_without_a_constraint_to_its_parent_fails_the_creation() {
        let mut e = engine();
        let (router, built) = instance_setup(&mut e);
        let before = e.mem.u32(router);
        let skeleton = make_skeleton(&mut e, &[Some(b"root"), Some(b"leg")], &[0xffff, 0]);
        let root = rigid_body(&mut e, Some(b"root"));
        let leg = rigid_body(&mut e, Some(b"leg"));
        let elsewhere = link(&mut e, leg, 0x1234);
        let bodies = list(&mut e, &[root, leg]);
        let constraints = list(&mut e, &[elsewhere]);
        assert_eq!(
            e.call(0x00ce_a810, &args![skeleton, bodies, constraints])
                .u32(),
            0
        );
        assert!(built.borrow().is_empty());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!((sent[0][1], sent[0][4]), (0xabba_9f9f, 0x298));
        let builder = calls(&e, TEXT_BUILDER_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND),
            vec![vec![builder, TEXT_NO_CONSTRAINT]]
        );
        // The bone's own address (the second bone, 8 bytes in) goes to the string-pointer writer.
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(
            calls(&e, TEXT_BUILDER_APPEND_STRING_PTR),
            vec![vec![builder, bones + 8]]
        );
        assert_eq!(e.mem.u32(router), before);
        // With no constraints at all the search does not even start.
        let none = list(&mut e, &[]);
        assert_eq!(e.call(0x00ce_a810, &args![skeleton, bodies, none]).u32(), 0);
        assert_eq!(reports(&e).len(), 2);
    }

    /// A skeleton and its pieces for `fn_00ceaf60`: three bones, a rigid body per bone but the last
    /// (the index array: 0, 1, -1), a constraint per body but the first, and `this` pointing at them.
    struct Pieces {
        this: u32,
        bodies: [u32; 2],
        constraints: [u32; 1],
    }

    fn make_pieces(e: &mut Engine) -> Pieces {
        let this = e.mem.alloc(0x40);
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton + 0x1c, 3);
        e.mem.set_u32(this + 0x2c, skeleton);
        let bodies = [e.mem.alloc(0x10), e.mem.alloc(0x10)];
        let bodies_array = list(e, &bodies);
        let bodies_data = e.mem.u32(bodies_array);
        e.mem.set_u32(this + 8, bodies_data);
        let constraints = [e.mem.alloc(0x10)];
        let constraints_array = list(e, &constraints);
        let constraints_data = e.mem.u32(constraints_array);
        e.mem.set_u32(this + 0x14, constraints_data);
        let indexes = e.mem.alloc(12);
        for (index, value) in [0u32, 1, 0xffff_ffff].into_iter().enumerate() {
            e.mem.set_u32(indexes + 4 * index as u32, value);
        }
        e.mem.set_u32(this + 0x20, indexes);
        Pieces {
            this,
            bodies,
            constraints,
        }
    }

    #[test]
    fn the_two_marked_groups_of_bones_each_make_an_instance() {
        let mut e = engine();
        install_text(&mut e);
        let (_router, allocator) = install_arrays(&mut e);
        let pieces = make_pieces(&mut e);
        // The marks: bones 0 and 1 are 0, bone 2 is 1. The byte buffer comes from the allocator's
        // buffer slot; `00cdd410` fills it.
        e.register_double(MARK_BONES, |e, a| {
            for (index, mark) in [0u8, 0, 1].into_iter().enumerate() {
                e.mem.set_u8(a[2] + index as u32, mark);
            }
            Ret::default()
        });
        let built: Built = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&built);
        e.register_double(RAGDOLL_INSTANCE_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push((
                a[3],
                contents(e, a[1]),
                contents(e, a[2]),
                contents(e, a[4]),
            ));
            Ret {
                eax: 0x5000 + sink.borrow().len() as u32,
                ..Ret::default()
            }
        });
        let block = e.mem.alloc(0x40);
        answer(&mut e, ALLOCATE_SLOT, block);
        let outputs = e.mem.alloc(8);
        let skeleton = e.mem.u32(pieces.this + 0x2c);
        e.call(0x00ce_af60, &args![pieces.this, 0u32, outputs, outputs + 4]);
        // The mark buffer was asked for 3 bytes (the double granted 7: one more than asked, see
        // `install_arrays`), and given back with the granted size.
        let asked = calls(&e, BUFFER_SLOT);
        assert_eq!(asked.len(), 1);
        let marks_at = calls(&e, MARK_BONES)[0][2];
        assert_eq!(calls(&e, MARK_BONES), vec![vec![skeleton, 0, marks_at, 1]]);
        let made = built.borrow();
        assert_eq!(made.len(), 2);
        let (b0, b1) = (pieces.bodies[0], pieces.bodies[1]);
        // Group 0: bones 0 and 1 (bone 1's constraint, bone 0 is the root and gets none), bone 2
        // pushes -1.
        assert_eq!(
            made[0],
            (
                skeleton,
                vec![b0, b1],
                vec![pieces.constraints[0]],
                vec![0, 1, 0xffff_ffff]
            )
        );
        // Group 1: only bone 2 is marked 1, and it has no body.
        assert_eq!(made[1], (skeleton, vec![], vec![], vec![0xffff_ffff; 3]));
        assert_eq!(e.mem.u32(outputs), 0x5001);
        assert_eq!(e.mem.u32(outputs + 4), 0x5002);
        assert_eq!(e.mem.u16(block + 4), 0x30);
        // Every array that grew went back through the allocator: group 0's three arrays (16 words
        // each) and group 1's parent array; then the mark buffer with its granted size.
        let frees = calls(&e, FREE_SLOT);
        assert_eq!(frees.len(), 5);
        assert!(frees[..4]
            .iter()
            .all(|call| call[0] == allocator && call[2] == 64));
        assert_eq!(frees[4], vec![allocator, marks_at, 7]);
    }

    #[test]
    fn the_split_arrays_and_request_sit_in_the_exe_frame() {
        let mut e = engine();
        install_text(&mut e);
        let (_router, _allocator) = install_arrays(&mut e);
        let pieces = make_pieces(&mut e);
        e.register(MARK_BONES, |_, _| Ret::default());
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(RAGDOLL_INSTANCE_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(vec![a[1], a[2], a[4]]);
            Ret::default()
        });
        let block = e.mem.alloc(0x40);
        answer(&mut e, ALLOCATE_SLOT, block);
        let outputs = e.mem.alloc(8);
        let frame = e.next_stack_local(0x4c);
        e.call(0x00ce_af60, &args![pieces.this, 0u32, outputs, outputs + 4]);
        // Both passes build their arrays at +0x28 (x), +0x34 (y) and +0x40 (z).
        let arrays = vec![frame + 0x34, frame + 0x40, frame + 0x28];
        assert_eq!(*seen.borrow(), vec![arrays.clone(), arrays]);
        assert_eq!(calls(&e, BUFFER_SLOT)[0][1], frame + 0x18);
    }

    /// The doubles `constructSkeletonForRagdoll` needs. `setFromTransformNoScale` makes a
    /// `hkQsTransform` out of a test layout: translation from `+0x30`, rotation from `+0x20` and scale
    /// from `+0x10` of the source. `setMul` writes (1, 2, 3, 4) and records its two arguments'
    /// vectors. The skeleton block (0x48 bytes) comes from the allocator, and `_reserve` hands out
    /// exactly the capacity asked for. Returns the log: (tag, words).
    fn skeleton_setup(e: &mut Engine) -> (u32, Log, u32) {
        install_vectors(e);
        install_text(e);
        let (_, allocator) = install_router(e);
        let block = e.mem.alloc(0x48);
        answer(e, ALLOCATE_SLOT, block);
        let logs: Log = Rc::new(RefCell::new(Vec::new()));
        e.register_double(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(SET_FROM_TRANSFORM_NO_SCALE, |e, a| {
            for lane in 0..4 {
                let translation = e.mem.u32(a[1] + 0x30 + 4 * lane);
                e.mem.set_u32(a[0] + 4 * lane, translation);
                let rotation = e.mem.u32(a[1] + 0x20 + 4 * lane);
                e.mem.set_u32(a[0] + 0x10 + 4 * lane, rotation);
                let scale = e.mem.u32(a[1] + 0x10 + 4 * lane);
                e.mem.set_u32(a[0] + 0x20 + 4 * lane, scale);
            }
            Ret::default()
        });
        let multiplications = Rc::clone(&logs);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            let mut seen = a.to_vec();
            seen.extend(words(e, a[1], 4));
            seen.extend(words(e, a[2], 4));
            multiplications.borrow_mut().push(("set_mul", seen));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        for (name, address) in [
            ("construct", STRING_PTR_CONSTRUCT),
            ("destruct", STRING_PTR_DESTRUCT),
            ("assign_text", STRING_PTR_ASSIGN_TEXT),
            ("assign", STRING_PTR_ASSIGN),
        ] {
            let log = Rc::clone(&logs);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        (block, logs, allocator)
    }

    /// A rigid body whose transform area (`+0xf0`) holds, in the test layout of the
    /// `setFromTransformNoScale` double, scale `+0x10`, rotation `+0x20` and translation `+0x30`.
    fn posed_body(
        e: &mut Engine,
        name: Option<&[u8]>,
        translation: [f32; 4],
        rotation: [f32; 4],
        scale: [f32; 4],
    ) -> u32 {
        let body = rigid_body(e, name);
        store_vec(e, body + 0xf0 + 0x10, scale);
        store_vec(e, body + 0xf0 + 0x20, rotation);
        store_vec(e, body + 0xf0 + 0x30, translation);
        body
    }

    fn tagged(logs: &Log, name: &str) -> Vec<Vec<u32>> {
        logs.borrow()
            .iter()
            .filter(|(tag, _)| *tag == name)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn a_skeleton_is_built_with_poses_relative_to_the_parent() {
        let mut e = engine();
        let (skeleton, logs, allocator) = skeleton_setup(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let root = posed_body(
            &mut e,
            Some(b"root"),
            [1.0, 2.0, 3.0, 0.0],
            identity,
            [2.0, 2.0, 2.0, 1.0],
        );
        let child = posed_body(
            &mut e,
            Some(b"child"),
            [4.0, 6.0, 8.0, 0.0],
            identity,
            [4.0, 6.0, 8.0, 1.0],
        );
        let link_to_root = link(&mut e, child, root);
        let bodies = list(&mut e, &[root, child]);
        let constraints = list(&mut e, &[link_to_root]);
        let got = e.call(0x00ce_b2c0, &args![bodies, constraints]).u32();
        assert_eq!(got, skeleton);
        // The object: size word, reference count, vtable and the name string.
        assert_eq!(e.mem.u16(skeleton + 4), 0x48);
        assert_eq!(e.mem.u16(skeleton + 6), 1);
        assert_eq!(e.mem.u32(skeleton), SKELETON_VTABLE);
        let constructed = tagged(&logs, "construct");
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(
            constructed,
            vec![vec![skeleton + 8], vec![bones], vec![bones + 8]]
        );
        // Room for two of each (asked as bones of 8 bytes, parents of 2, poses of 0x30).
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![allocator, skeleton + 0x18, 2, 8],
                vec![allocator, skeleton + 0xc, 2, 2],
                vec![allocator, skeleton + 0x24, 2, 0x30]
            ]
        );
        assert_eq!(e.mem.u32(skeleton + 0x1c), 2);
        assert_eq!(e.mem.u32(skeleton + 0x10), 2);
        assert_eq!(e.mem.u32(skeleton + 0x28), 2);
        // The parents: none for the root, the root for the child.
        let parents = e.mem.u32(skeleton + 0xc);
        assert_eq!(e.mem.u16(parents), 0xffff);
        assert_eq!(e.mem.u16(parents + 2), 0);
        // The names and the lock bytes: only the first bone is free.
        assert_eq!(
            tagged(&logs, "assign_text"),
            vec![
                vec![bones, e.mem.u32(root + 0x74)],
                vec![bones + 8, e.mem.u32(child + 0x74)]
            ]
        );
        assert_eq!(e.mem.u8(bones + 4), 0);
        assert_eq!(e.mem.u8(bones + 8 + 4), 1);
        // The skeleton's own name is a copy of the first bone's.
        assert_eq!(tagged(&logs, "assign"), vec![vec![skeleton + 8, bones]]);
        // The poses: the root's own transform, the child's relative to it.
        let pose = e.mem.u32(skeleton + 0x24);
        assert_eq!(
            words(&e, pose, 12),
            [
                bits([1.0, 2.0, 3.0, 0.0]),
                bits(identity),
                bits([2.0, 2.0, 2.0, 1.0])
            ]
            .concat()
        );
        let half = refined_reciprocal(2.0);
        assert_eq!(
            words(&e, pose + 0x30, 4),
            bits([3.0, 4.0, 5.0, 0.0]).to_vec()
        );
        assert_eq!(
            words(&e, pose + 0x40, 4),
            bits([1.0, 2.0, 3.0, 4.0]).to_vec()
        );
        assert_eq!(
            words(&e, pose + 0x50, 4),
            bits([4.0 * half, 6.0 * half, 8.0 * half, 0.0]).to_vec()
        );
        // `setMul(pose rotation, conjugated parent rotation, child rotation)`.
        let multiplications = tagged(&logs, "set_mul");
        assert_eq!(multiplications.len(), 1);
        assert_eq!(multiplications[0][0], pose + 0x40);
        assert_eq!(multiplications[0][3..7], bits([-0.0, -0.0, -0.0, 1.0]));
        assert_eq!(multiplications[0][7..11], bits(identity));
        assert_eq!(multiplications[0][2], multiplications[0][1] + 0x30);
    }

    #[test]
    fn the_skeleton_transforms_sit_in_the_aligned_exe_frame() {
        let mut e = engine();
        let (_skeleton, _logs, _allocator) = skeleton_setup(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let root = posed_body(&mut e, Some(b"root"), [1.0; 4], identity, [1.0; 4]);
        let child = posed_body(&mut e, Some(b"child"), [2.0; 4], identity, [1.0; 4]);
        let link_to_root = link(&mut e, child, root);
        let bodies = list(&mut e, &[root, child]);
        let constraints = list(&mut e, &[link_to_root]);
        let raw = e.next_stack_local(0x2d0);
        e.call(0x00ce_b2c0, &args![bodies, constraints]);
        let frame = aligned_frame_at(raw, 0x2c0);
        let sources = calls(&e, SET_FROM_TRANSFORM_NO_SCALE);
        assert_eq!(sources[1][0], frame + 0x60);
        assert_eq!(sources[2][0], frame + 0x90);
    }

    #[test]
    fn a_body_without_a_name_is_reported_and_its_bone_keeps_the_default_name() {
        let mut e = engine();
        let (skeleton, logs, _) = skeleton_setup(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let nameless = posed_body(&mut e, None, [0.0; 4], identity, [1.0; 4]);
        let bodies = list(&mut e, &[nameless]);
        let constraints = list(&mut e, &[]);
        let got = e.call(0x00ce_b2c0, &args![bodies, constraints]).u32();
        assert_eq!(got, skeleton);
        assert!(tagged(&logs, "assign_text").is_empty());
        let sent = reports(&e);
        assert_eq!(sent.len(), 1);
        assert_eq!((sent[0][1], sent[0][4]), (0xabba_cc14, 0x22e));
        assert_eq!(calls(&e, TEXT_BUILDER_APPEND)[0][1], TEXT_NO_BODY_NAME);
        // A single bone: the lock byte is clear and the name is still copied to the skeleton.
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(e.mem.u8(bones + 4), 0);
        assert_eq!(tagged(&logs, "assign").len(), 1);
    }

    #[test]
    fn the_parent_is_found_among_the_earlier_bodies_or_is_none() {
        let mut e = engine();
        let (skeleton, logs, _) = skeleton_setup(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let body = |e: &mut Engine, name: &[u8]| {
            posed_body(
                e,
                Some(name),
                [1.0, 1.0, 1.0, 0.0],
                identity,
                [1.0, 1.0, 1.0, 1.0],
            )
        };
        let (b0, b1, b2) = (body(&mut e, b"a"), body(&mut e, b"b"), body(&mut e, b"c"));
        // The second constraint's parent is b0 (not the previous body); the third body's parent is not
        // among the bodies, so it is a root.
        let first = link(&mut e, b1, b0);
        let second = link(&mut e, b2, 0x9999);
        let bodies = list(&mut e, &[b0, b1, b2]);
        let constraints = list(&mut e, &[first, second]);
        e.call(0x00ce_b2c0, &args![bodies, constraints]);
        let parents = e.mem.u32(skeleton + 0xc);
        assert_eq!(
            (0..3)
                .map(|index| e.mem.u16(parents + 2 * index))
                .collect::<Vec<_>>(),
            vec![0xffff, 0, 0xffff]
        );
        // Only bone 1 had a parent: one `setMul`, and the lock bytes are 0, 1, 1.
        assert_eq!(tagged(&logs, "set_mul").len(), 1);
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(
            (0..3)
                .map(|index| e.mem.u8(bones + 8 * index + 4))
                .collect::<Vec<_>>(),
            vec![0, 1, 1]
        );
        // The root-like bone 2 got its own transform directly.
        let source_calls = calls(&e, SET_FROM_TRANSFORM_NO_SCALE);
        let pose = e.mem.u32(skeleton + 0x24);
        assert!(source_calls
            .iter()
            .any(|call| call[0] == pose + 0x60 && call[1] == b2 + 0xf0));
    }

    #[test]
    fn a_skeleton_of_no_bodies_has_no_bones_and_keeps_its_name_default() {
        let mut e = engine();
        let (skeleton, logs, _) = skeleton_setup(&mut e);
        let bodies = list(&mut e, &[]);
        let constraints = list(&mut e, &[]);
        assert_eq!(
            e.call(0x00ce_b2c0, &args![bodies, constraints]).u32(),
            skeleton
        );
        assert!(calls(&e, ARRAY_RESERVE).is_empty());
        assert!(tagged(&logs, "assign").is_empty());
        assert_eq!(tagged(&logs, "construct"), vec![vec![skeleton + 8]]);
        assert_eq!(e.mem.u32(skeleton + 0x1c), 0);
    }
}
