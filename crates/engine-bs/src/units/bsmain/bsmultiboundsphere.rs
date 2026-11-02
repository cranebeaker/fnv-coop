//! `BSMain/bsmultiboundsphere.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c4fa10` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C4FA10: u32 = 0x00c4_fa10;

// Translated from 00c38000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map calls this `BSMultiBoundSphere::LinkObject`, but its vtable word 0101e934
/// sits in the table whose neighbours are `BSMultiBoundAABB::CreateClone` and `LoadBinary`, so
/// it is named by address. Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map), the `LinkObject` body shared by the
/// multibound shapes. Returns nothing: Xbox PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is
/// void; `stream` is the `NiStream` reference as an address. Parameter count from the `RET 4`
/// of both bodies.
pub fn fn_00c38000(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

// Translated from 00c4fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSMultiBoundSphere` vtable word 0101eadc, right after
/// `BSMultiBoundSphere::LoadBinary` (0101ead8). Forwards `this` and the stream to `00c4fa10`
/// (`bsmultiboundshape.obj`, unnamed in the engine map). Returns nothing: Xbox PDB
/// `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream` reference
/// as an address. Parameter count from the `RET 4` of both bodies.
pub fn fn_00c4fc50(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C4FA10, &args![this, stream]);
}

/// `BSMultiBoundShape::BSMultiBoundShape` (`00c4f980`, `thiscall`) and
/// `BSMultiBoundSphere`'s vtable (`0101ea8c`).
const SHAPE_CONSTRUCTOR: u32 = 0x00c4_f980;
const SPHERE_VTABLE: u32 = 0x0101_ea8c;

// Translated from 00c4fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::BSMultiBoundSphere` (Xbox PDB, the overload the
/// engine map calls `_ov2`): the `BSMultiBoundShape` constructor, then the
/// class's vtable. Answers `this`.
pub fn bs_multi_bound_sphere_ov2(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SHAPE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), SPHERE_VTABLE);
    this
}

// ---- Batch n067 ---------------------------------------------------------------------------
//
// A `BSMultiBoundSphere` is a `BSMultiBoundShape` (0xC bytes) followed by `Center` (Xbox PDB,
// +0xC) and `Radius` (+0x18): 0x1C bytes, vtable `0101ea8c`. Float comparisons follow the
// exe's flag tests, NaN included; x87 arithmetic goes through `crate::x87` in the exe's
// order, and floats the exe copies with `FLD`/`FSTP` are quieted.

/// `Center` and `Radius` (Xbox PDB, PC offsets) and the object's size.
const CENTER: u32 = 0x0c;
const RADIUS: u32 = 0x18;
const SPHERE_SIZE: u32 = 0x1c;
/// `BSMultiBoundShape::RegisterStreamables` (map name, `00c4fa30`) and the shape's
/// `CopyMembers`, `LoadBinary`, save body, `IsEqual` and viewer strings
/// (`bsmultiboundshape.obj`).
const SHAPE_REGISTER_STREAMABLES: u32 = 0x00c4_fa30;
const SHAPE_COPY_MEMBERS: u32 = 0x00c4_f9b0;
const SHAPE_LOAD_BINARY: u32 = 0x00c4_f9f0;
const SHAPE_SAVE_BINARY: u32 = 0x00c4_fa60;
const SHAPE_IS_EQUAL: u32 = 0x00c4_fa80;
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c4_fab0;
/// A point's `LoadBinary` and `SaveBinary` (`this` the point, the stream).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
/// Reads and writes a float through the stream (cdecl: the stream, the float's address;
/// `00a3da50` is `NiStreamSaveBinary<float>` in the map).
const STREAM_LOAD_FLOAT: u32 = 0x00a2_6df0;
const STREAM_SAVE_FLOAT: u32 = 0x00a3_da50;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiGetViewerString` (cdecl, the name) and the word at `01203108` it is given;
/// `NiGetViewerString_ov9` (map name; cdecl: a label and a float); a point's
/// `GetViewerString` (`this` the point, the label).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const TYPE_NAME_WORD: u32 = 0x0120_3108;
const FLOAT_VIEWER_STRING: u32 = 0x00a7_d550;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// Adds a string to the viewer strings (`this` the strings, the address of a word holding
/// the string).
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// The labels `"Center"` (`010c1040`) and `"Radius"` (`01015a9c`).
const CENTER_LABEL: u32 = 0x010c_1040;
const RADIUS_LABEL: u32 = 0x0101_5a9c;
/// `NiPoint3` subtraction (`00439ef0`: `this`, out, other; writes `this - other` to out and
/// returns out), its length (`00457990`) and squared length (`004a7290`), both in `ST0`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const POINT_SQUARED_LENGTH: u32 = 0x004a_7290;
/// `BSMultiBoundAABB::GetVertices` and `BSMultiBoundOBB::GetVertices` (Xbox PDB).
const AABB_GET_VERTICES: u32 = 0x00c3_9270;
const OBB_GET_VERTICES: u32 = 0x00c3_7a00;
/// `NiBound::WhichSide` (Xbox PDB; `this` the bound, the plane): 1 or 2 by the side, 0
/// when the plane cuts it.
const BOUND_WHICH_SIDE: u32 = 0x004b_6100;
/// The `double` 0.0 (`01012060`).
const ZERO: u32 = 0x0101_2060;
/// Vtable slots (byte offsets): `QType`, `SetCenter`.
const SLOT_TYPE: u32 = 0x8c;
const SLOT_SET_CENTER: u32 = 0xb8;

/// Copies `count` words from `from` to `to` (`MOV` copies: no quieting).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Adds `string` to `strings` through a local word holding it, as the exe does.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

// Translated from 00c38020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map calls this `BSMultiBoundSphere::RegisterStreamables`, but its vtable word
/// 0101e938 is `BSMultiBoundAABB`'s `RegisterStreamables` slot (between the AABB's
/// `LinkObject` and `SaveBinary` words), so it is named by address:
/// `BSMultiBoundShape::RegisterStreamables` (`00c4fa30`), its byte normalised to true or
/// false.
pub fn fn_00c38020(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(SHAPE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

/// `operator new(0x1c)` and the constructor when the block is not null.
fn new_sphere(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![SPHERE_SIZE]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        bs_multi_bound_sphere_ov2(e, Ptr::new(block))
    }
}

// Translated from 00c4fb30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4fb30 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSMultiBoundSphere::CreateClone` (Xbox PDB): a new sphere (`operator new(0x1c)`,
/// constructed unless the allocation failed), then this sphere's members copied into it
/// (`CopyMembers` runs on a null clone too). Returns the clone.
pub fn bs_multi_bound_sphere_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_sphere(e);
    bs_multi_bound_sphere_copy_members(e, this, clone, cloning);
    clone
}

// Translated from 00c4fbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::CopyMembers` (Xbox PDB): the shape's `CopyMembers`, the clone's
/// `SetCenter` (slot 0xb8) with this sphere's centre, then the radius (copied with
/// `FLD`/`FSTP` twice).
pub fn bs_multi_bound_sphere_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, cloning]);
    e.vcall(clone.addr(), SLOT_SET_CENTER, &args![this.addr() + CENTER]);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    e.mem.set_u32(clone.addr() + RADIUS, radius);
}

// Translated from 00c4fc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::LoadBinary` (Xbox PDB): the shape's part, the centre (a point's
/// `LoadBinary`), then the radius (`00a26df0`).
pub fn bs_multi_bound_sphere_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    e.call(POINT_LOAD_BINARY, &args![this.addr() + CENTER, stream]);
    e.call(STREAM_LOAD_FLOAT, &args![stream, this.addr() + RADIUS]);
}

// Translated from 00c4fc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSMultiBoundSphere`'s `RegisterStreamables` slot (vtable
/// word 0101eae0, after the `LinkObject` word 0101eadc): `BSMultiBoundShape::RegisterStreamables`
/// (`00c4fa30`), its byte normalised to true or false.
pub fn fn_00c4fc70(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(SHAPE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00c4fca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::SaveBinary` (Xbox PDB): the shape's save body, the centre (a
/// point's `SaveBinary`), then the radius (`NiStreamSaveBinary<float>`).
pub fn bs_multi_bound_sphere_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    e.call(POINT_SAVE_BINARY, &args![this.addr() + CENTER, stream]);
    e.call(STREAM_SAVE_FLOAT, &args![stream, this.addr() + RADIUS]);
}

// Translated from 00c4fce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::IsEqual` (Xbox PDB): the shape's `IsEqual`, and when it holds the
/// centres equal component by component and the radii equal (`FUCOMPP`: a NaN is never
/// equal; this sphere's radius is copied first).
pub fn bs_multi_bound_sphere_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(SHAPE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let center = (0..3).all(|axis| {
        x87_load32(e.mem.u32(this.addr() + CENTER + 4 * axis))
            == x87_load32(e.mem.u32(other.addr() + CENTER + 4 * axis))
    });
    let radius = x87_load32(x87_quiet_bits(e.mem.u32(this.addr() + RADIUS)))
        == x87_load32(e.mem.u32(other.addr() + RADIUS));
    center & radius
}

// Translated from 00c4fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c4fdb0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// The class's stream creator (no arguments; unnamed in the map): a new sphere from
/// `operator new(0x1c)`, constructed; null when the allocation fails.
pub fn fn_00c4fdb0(e: &mut Engine) -> Ptr {
    new_sphere(e)
}

// Translated from 00c4fe30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::GetViewerStrings` (Xbox PDB): the shape's strings, the string
/// `NiGetViewerString` makes of the word at `01203108`, the centre's (`"Center"`) and the
/// radius's (`00a7d550` with `"Radius"` and the radius, copied with `FLD`/`FSTP`), each
/// added through `0096a610`.
pub fn bs_multi_bound_sphere_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let type_name = e.mem.u32(TYPE_NAME_WORD);
    let first = e.call(GET_VIEWER_STRING, &args![type_name]).u32();
    add_viewer_string(e, strings, first);
    let second = e
        .call(
            POINT_GET_VIEWER_STRING,
            &args![this.addr() + CENTER, CENTER_LABEL],
        )
        .u32();
    add_viewer_string(e, strings, second);
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    let third = e
        .call(FLOAT_VIEWER_STRING, &args![RADIUS_LABEL, radius])
        .u32();
    add_viewer_string(e, strings, third);
}

// Translated from 00c4fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot 0x8c of `BSMultiBoundSphere` (`QType`, Xbox PDB,
/// in the shape classes' tables): returns 3, the type the other shapes' `CheckBound` treat
/// as a bound with a `GetNiBound`.
pub fn fn_00c4fec0(_e: &mut Engine, _this: Ptr) -> u32 {
    3
}

// Translated from 00c4fed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::CheckBound` for a bound sphere (Xbox PDB, slot 0x98; `sphere` points
/// to the centre and the radius at +0xC): the distance between the centres (`00439ef0`
/// then `00457990`, stored as a float) minus (this radius minus the other) not above zero
/// gives 2; else minus (this radius plus the other) not above zero gives 1; else 0 (an
/// unordered value is above).
pub fn bs_multi_bound_sphere_check_bound(e: &mut Engine, this: Ptr, sphere: Ptr) -> u32 {
    e.with_stack(12, |e, offset| {
        e.call(POINT_SUBTRACT, &args![this.addr() + CENTER, offset, sphere]);
        let distance = x87_store32(e.call(POINT_LENGTH, &args![offset]).f64());
        let zero: f64 = e.global(ZERO);
        let other = x87_quiet_bits(e.mem.u32(sphere.addr() + 0xc));
        let own = x87_load32(e.mem.u32(this.addr() + RADIUS));
        let reach = x87_sub(own, x87_m32(other));
        if x87_sub(x87_m32(distance), reach) <= zero {
            return 2;
        }
        let other = x87_quiet_bits(e.mem.u32(sphere.addr() + 0xc));
        let own = x87_load32(e.mem.u32(this.addr() + RADIUS));
        let reach = x87_add(own, x87_m32(other));
        if x87_sub(x87_m32(distance), reach) <= zero {
            1
        } else {
            0
        }
    })
}

/// The vertex test of `CheckBound` for a box: 0 as soon as one of the eight vertices at
/// `vertices` is farther from the centre than the radius (squared distance, `004a7290`,
/// stored as a float, against the radius squared, stored as a float; an unordered pair
/// passes), else 2.
fn vertices_within(e: &mut Engine, this: Ptr, vertices: u32) -> u32 {
    let radius = e.mem.u32(this.addr() + RADIUS);
    let radius_squared = x87_store32(x87_mul(
        x87_load32(radius),
        x87_m32(e.mem.u32(this.addr() + RADIUS)),
    ));
    e.with_stack(12, |e, offset| {
        for vertex in 0..8u32 {
            e.call(
                POINT_SUBTRACT,
                &args![this.addr() + CENTER, offset, vertices + 12 * vertex],
            );
            let squared = x87_store32(e.call(POINT_SQUARED_LENGTH, &args![offset]).f64());
            if x87_load32(radius_squared) < x87_load32(squared) {
                return 0;
            }
        }
        2
    })
}

// Translated from 00c4ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::CheckBound` for another bound (Xbox PDB `CheckBound_ov2`, slot
/// 0x94; `other` a `BSMultiBound`, its shape the word at +0xC): by the shape's `QType`
/// (slot 0x8c). Type 1: the axis-aligned box's eight vertices (`00c39270`) must all be
/// within the radius ([`vertices_within`]): 2, else 0. Type 2: the same with the oriented
/// box's vertices (`00c37a00`). Type 3: the distance between the centres plus the other
/// sphere's radius minus this one's not above zero gives 2, else 0. Any other type: 0. The
/// shape word is read again in each branch, as the exe does.
pub fn bs_multi_bound_sphere_check_bound_ov2(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let shape = e.mem.u32(other.addr() + 0xc);
    match e.vcall(shape, SLOT_TYPE, &[]).u32() {
        kind @ (1 | 2) => {
            let shape = e.mem.u32(other.addr() + 0xc);
            let get_vertices = if kind == 1 {
                AABB_GET_VERTICES
            } else {
                OBB_GET_VERTICES
            };
            e.with_stack(96, |e, vertices| {
                e.call(get_vertices, &args![shape, vertices]);
                vertices_within(e, this, vertices.addr())
            })
        }
        3 => {
            let shape = e.mem.u32(other.addr() + 0xc);
            e.with_stack(12, |e, offset| {
                e.call(
                    POINT_SUBTRACT,
                    &args![this.addr() + CENTER, offset, shape + CENTER],
                );
                let distance = x87_store32(e.call(POINT_LENGTH, &args![offset]).f64());
                let zero: f64 = e.global(ZERO);
                let reach = x87_add(x87_load32(distance), x87_m32(e.mem.u32(shape + RADIUS)));
                if x87_sub(reach, x87_m32(e.mem.u32(this.addr() + RADIUS))) <= zero {
                    2
                } else {
                    0
                }
            })
        }
        _ => 0,
    }
}

/// The frustum test of `WithinFrustum` and `CompletelyWithinFrustum`: the sphere copied
/// into a local bound (the centre word by word, the radius with `FLD`/`FSTP` twice), then
/// `NiBound::WhichSide` for each plane of the frustum's mask (+0x60, read for each plane);
/// false at the first plane where `fails` holds for the side, true otherwise.
fn frustum_test(e: &mut Engine, this: Ptr, frustum: Ptr, fails: impl Fn(u32) -> bool) -> bool {
    e.with_stack(16, |e, bound| {
        let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
        copy_words(e, this.addr() + CENTER, bound.addr(), 3);
        e.mem.set_u32(bound.addr() + 0xc, radius);
        for plane in 0..6u32 {
            if (1u32 << plane) & e.mem.u32(frustum.addr() + 0x60) == 0 {
                continue;
            }
            let side = e
                .call(
                    BOUND_WHICH_SIDE,
                    &args![bound, frustum.byte_add(plane << 4)],
                )
                .u32();
            if fails(side) {
                return false;
            }
        }
        true
    })
}

// Translated from 00c50210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::WithinFrustum` (Xbox PDB): false as soon as the sphere lies on side
/// 2 of a plane of the frustum's mask (`NiBound::WhichSide`), true otherwise.
pub fn bs_multi_bound_sphere_within_frustum(e: &mut Engine, this: Ptr, frustum: Ptr) -> bool {
    frustum_test(e, this, frustum, |side| side == 2)
}

// Translated from 00c502a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::CompletelyWithinFrustum` (Xbox PDB): false as soon as the sphere is
/// not on side 1 of a plane of the frustum's mask (`NiBound::WhichSide`), true otherwise.
pub fn bs_multi_bound_sphere_completely_within_frustum(
    e: &mut Engine,
    this: Ptr,
    frustum: Ptr,
) -> bool {
    frustum_test(e, this, frustum, |side| side != 1)
}

// Translated from 00c50330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::GetNiBound` (Xbox PDB): the centre and the radius (copied with
/// `FLD`/`FSTP` twice) into the bound at `out`.
pub fn bs_multi_bound_sphere_get_ni_bound(e: &mut Engine, this: Ptr, out: Ptr) {
    let radius = x87_quiet_bits(e.mem.u32(this.addr() + RADIUS));
    copy_words(e, this.addr() + CENTER, out.addr(), 3);
    e.mem.set_u32(out.addr() + 0xc, radius);
}

// Translated from 00c50370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::MakeDebugGeom` (Xbox PDB): does nothing (`RET 0x18`: the two
/// geometries and the four colour words).
#[allow(clippy::too_many_arguments)]
pub fn bs_multi_bound_sphere_make_debug_geom(
    _e: &mut Engine,
    _this: Ptr,
    _lines: Ptr,
    _triangles: Ptr,
    _color_0: u32,
    _color_1: u32,
    _color_2: u32,
    _color_3: u32,
) {
}

// Translated from 00c50380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundSphere::QPointWithin` (Xbox PDB): whether the radius is greater than the
/// distance from the centre to `point` (`point - centre`, `00439ef0`, then the length of
/// the result, stored as a float); an unordered pair is not.
pub fn bs_multi_bound_sphere_q_point_within(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    e.with_stack(12, |e, offset| {
        let difference = e
            .call(POINT_SUBTRACT, &args![point, offset, this.addr() + CENTER])
            .u32();
        let distance = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
        x87_load32(e.mem.u32(this.addr() + RADIUS)) > x87_load32(distance)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_8000, fn_00c38000(Ptr, Ptr)),
        entry!(0x00c4_fc50, fn_00c4fc50(Ptr, Ptr)),
        entry!(0x00c4_fb10, bs_multi_bound_sphere_ov2(Ptr) -> Ptr),
        entry!(0x00c3_8020, fn_00c38020(Ptr, Ptr) -> bool),
        entry!(0x00c4_fb30, bs_multi_bound_sphere_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00c4_fbc0,
            bs_multi_bound_sphere_copy_members(Ptr, Ptr, Ptr)
        ),
        entry!(0x00c4_fc10, bs_multi_bound_sphere_load_binary(Ptr, Ptr)),
        entry!(0x00c4_fc70, fn_00c4fc70(Ptr, Ptr) -> bool),
        entry!(0x00c4_fca0, bs_multi_bound_sphere_save_binary(Ptr, Ptr)),
        entry!(0x00c4_fce0, bs_multi_bound_sphere_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c4_fdb0, fn_00c4fdb0() -> Ptr),
        entry!(
            0x00c4_fe30,
            bs_multi_bound_sphere_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c4_fec0, fn_00c4fec0(Ptr) -> u32),
        entry!(0x00c4_fed0, bs_multi_bound_sphere_check_bound(Ptr, Ptr) -> u32),
        entry!(0x00c4_ff60, bs_multi_bound_sphere_check_bound_ov2(Ptr, Ptr) -> u32),
        entry!(0x00c5_0210, bs_multi_bound_sphere_within_frustum(Ptr, Ptr) -> bool),
        entry!(
            0x00c5_02a0,
            bs_multi_bound_sphere_completely_within_frustum(Ptr, Ptr) -> bool
        ),
        entry!(0x00c5_0330, bs_multi_bound_sphere_get_ni_bound(Ptr, Ptr)),
        entry!(
            0x00c5_0370,
            bs_multi_bound_sphere_make_debug_geom(Ptr, Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00c5_0380, bs_multi_bound_sphere_q_point_within(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_00c38000_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_8000, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn forwarder_fn_00c4fc50_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C4FA10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c4_fc50, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C4FA10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }

    #[test]
    fn sphere_constructor_runs_the_shape_constructor_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(SHAPE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00c4_fb10, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SPHERE_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == SHAPE_CONSTRUCTOR && call.1 == vec![this]));
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn set_point(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, v) in (0u32..).zip(value) {
            e.mem.set_f32(at + 4 * i, v);
        }
    }

    fn length(e: &Engine, at: u32) -> f64 {
        (0..3)
            .map(|i| f64::from(e.mem.f32(at + 4 * i)).powi(2))
            .sum::<f64>()
    }

    /// An engine with 0.0 at `01012060` and the point arithmetic the sphere uses.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| length(e, a[0]).sqrt().into_ret());
        e.register(POINT_SQUARED_LENGTH, |e, a| length(e, a[0]).into_ret());
        e
    }

    fn sphere(e: &mut Engine, center: [f32; 3], radius: f32) -> u32 {
        let this = e.mem.alloc(SPHERE_SIZE);
        set_point(e, this + CENTER, center);
        e.mem.set_f32(this + RADIUS, radius);
        this
    }

    #[test]
    fn clone_copy_members_and_factory() {
        let mut e = Engine::new();
        e.register(SHAPE_CONSTRUCTOR, nothing);
        e.register(SHAPE_COPY_MEMBERS, nothing);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let set_center = 0x0090_0010;
        e.register(set_center, nothing);
        let vtable = SPHERE_VTABLE;
        e.map(vtable, 0xc0);
        e.mem.set_u32(vtable + SLOT_SET_CENTER, set_center);
        let this = e.mem.alloc(SPHERE_SIZE);
        e.mem.set_u32(this + RADIUS, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c4_fb30, &args![this, 0x55u32]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(
            calls_to(&e, SHAPE_COPY_MEMBERS),
            vec![vec![this, clone, 0x55]]
        );
        assert_eq!(calls_to(&e, set_center), vec![vec![clone, this + CENTER]]);
        assert_eq!(e.mem.u32(clone + RADIUS), 0x7fc0_0001);
        let made = e.call(0x00c4_fdb0, &args![]).u32();
        assert_eq!(e.mem.u32(made), vtable);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c4_fdb0, &args![]).u32(), 0);
    }

    #[test]
    fn streaming_and_register_streamables() {
        let mut e = Engine::new();
        for addr in [
            SHAPE_LOAD_BINARY,
            POINT_LOAD_BINARY,
            STREAM_LOAD_FLOAT,
            SHAPE_SAVE_BINARY,
            POINT_SAVE_BINARY,
            STREAM_SAVE_FLOAT,
        ] {
            e.register(addr, nothing);
        }
        e.register(SHAPE_REGISTER_STREAMABLES, |_, a| (0x100 | a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c4_fc10, &args![0x100u32, 0x200u32]);
        e.call(0x00c4_fca0, &args![0x100u32, 0x200u32]);
        assert_eq!(calls_to(&e, SHAPE_LOAD_BINARY), vec![vec![0x100, 0x200]]);
        assert_eq!(calls_to(&e, POINT_LOAD_BINARY), vec![vec![0x10c, 0x200]]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FLOAT), vec![vec![0x200, 0x118]]);
        assert_eq!(calls_to(&e, SHAPE_SAVE_BINARY), vec![vec![0x100, 0x200]]);
        assert_eq!(calls_to(&e, POINT_SAVE_BINARY), vec![vec![0x10c, 0x200]]);
        assert_eq!(calls_to(&e, STREAM_SAVE_FLOAT), vec![vec![0x200, 0x118]]);
        for addr in [0x00c3_8020u32, 0x00c4_fc70] {
            assert!(e.call(addr, &args![0x100u32, 1u32]).bool());
            assert!(!e.call(addr, &args![0x100u32, 0u32]).bool());
        }
        assert_eq!(e.call(0x00c4_fec0, &args![0x100u32]).u32(), 3);
        e.call(
            0x00c5_0370,
            &args![0x100u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
    }

    #[test]
    fn is_equal_compares_centre_and_radius() {
        let mut e = Engine::new();
        e.register(SHAPE_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = sphere(&mut e, [1.0, 2.0, 3.0], 4.0);
        let b = sphere(&mut e, [1.0, 2.0, 3.0], 4.0);
        assert!(e.call(0x00c4_fce0, &args![a, b]).bool());
        e.mem.set_f32(b + CENTER + 8, 3.5);
        assert!(!e.call(0x00c4_fce0, &args![a, b]).bool());
        e.mem.set_f32(b + CENTER + 8, 3.0);
        e.mem.set_f32(b + RADIUS, f32::NAN);
        assert!(!e.call(0x00c4_fce0, &args![a, b]).bool());
        e.mem.set_f32(b + RADIUS, 4.0);
        e.register(SHAPE_IS_EQUAL, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00c4_fce0, &args![a, b]).bool());
    }

    #[test]
    fn viewer_strings() {
        let mut e = Engine::new();
        e.register(SHAPE_GET_VIEWER_STRINGS, nothing);
        e.register(GET_VIEWER_STRING, |_, _| 0x11u32.into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, _| 0x22u32.into_ret());
        e.register(FLOAT_VIEWER_STRING, |_, _| 0x33u32.into_ret());
        e.register(ADD_VIEWER_STRING, |e, a| e.mem.u32(a[1]).into_ret());
        e.map(TYPE_NAME_WORD, 4);
        e.set_global(TYPE_NAME_WORD, 0x77u32);
        let this = sphere(&mut e, [0.0; 3], 2.5);
        e.call_log = Some(vec![]);
        e.call(0x00c4_fe30, &args![this, 0x900u32]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x77]]);
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![vec![this + CENTER, CENTER_LABEL]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_VIEWER_STRING),
            vec![vec![RADIUS_LABEL, 2.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, ADD_VIEWER_STRING).len(), 3);
    }

    #[test]
    fn check_bound_sphere() {
        let mut e = engine();
        let this = sphere(&mut e, [0.0; 3], 2.0);
        let other = sphere(&mut e, [1.0, 0.0, 0.0], 0.5);
        let bound = other + CENTER;
        assert_eq!(e.call(0x00c4_fed0, &args![this, bound]).u32(), 2);
        e.mem.set_f32(bound + 0xc, 1.5);
        assert_eq!(e.call(0x00c4_fed0, &args![this, bound]).u32(), 1);
        e.mem.set_f32(bound, 4.0);
        assert_eq!(e.call(0x00c4_fed0, &args![this, bound]).u32(), 0);
        e.mem.set_f32(bound + 0xc, f32::NAN);
        assert_eq!(e.call(0x00c4_fed0, &args![this, bound]).u32(), 0);
    }

    fn other_of_type(e: &mut Engine, kind: u32, shape: u32) -> u32 {
        let vtable = e.mem.alloc(0xc0);
        let function = 0x0090_0100 + kind;
        e.register_double(function, move |_, _| kind.into_ret());
        e.mem.set_u32(vtable + SLOT_TYPE, function);
        e.mem.set_u32(shape, vtable);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 0xc, shape);
        other
    }

    #[test]
    fn check_bound_other_shapes() {
        let mut e = engine();
        let this = sphere(&mut e, [0.0; 3], 2.0);
        for (kind, get_vertices) in [(1, AABB_GET_VERTICES), (2, OBB_GET_VERTICES)] {
            let shape = e.mem.alloc(0x48);
            let other = other_of_type(&mut e, kind, shape);
            e.register_double(get_vertices, |e, a| {
                for v in 0..8 {
                    set_point(e, a[1] + 12 * v, [1.0, 1.0, 1.0]);
                }
                Ret::default()
            });
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c4_ff60, &args![this, other]).u32(), 2);
            assert_eq!(calls_to(&e, get_vertices)[0][0], shape);
            e.register_double(get_vertices, |e, a| {
                for v in 0..8 {
                    set_point(e, a[1] + 12 * v, [1.0, 1.0, 1.0]);
                }
                set_point(e, a[1] + 12 * 7, [2.0, 0.5, 0.0]);
                Ret::default()
            });
            assert_eq!(e.call(0x00c4_ff60, &args![this, other]).u32(), 0);
        }
        let shape = sphere(&mut e, [1.0, 0.0, 0.0], 1.0);
        let other = other_of_type(&mut e, 3, shape);
        assert_eq!(e.call(0x00c4_ff60, &args![this, other]).u32(), 2);
        e.mem.set_f32(shape + RADIUS, 1.25);
        assert_eq!(e.call(0x00c4_ff60, &args![this, other]).u32(), 0);
        let other = other_of_type(&mut e, 5, shape);
        assert_eq!(e.call(0x00c4_ff60, &args![this, other]).u32(), 0);
    }

    #[test]
    fn frustum_bound_and_point() {
        let mut e = engine();
        let this = sphere(&mut e, [1.0, 2.0, 3.0], 4.0);
        let frustum = e.mem.alloc(0x64);
        // Plane 1 answers side 2, the others side 1.
        e.register_double(BOUND_WHICH_SIDE, move |e, a| {
            assert_eq!(e.mem.f32(a[0] + 0xc), 4.0);
            u32::from(a[1] == frustum + 0x10).wrapping_add(1).into_ret()
        });
        e.mem.set_u32(frustum + 0x60, 0b101);
        assert!(e.call(0x00c5_0210, &args![this, frustum]).bool());
        assert!(e.call(0x00c5_02a0, &args![this, frustum]).bool());
        e.mem.set_u32(frustum + 0x60, 0b111);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c5_0210, &args![this, frustum]).bool());
        assert_eq!(calls_to(&e, BOUND_WHICH_SIDE).len(), 2);
        assert!(!e.call(0x00c5_02a0, &args![this, frustum]).bool());
        let out = e.mem.alloc(16);
        e.call(0x00c5_0330, &args![this, out]);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 0xc), 4.0);
        let point = e.mem.alloc(12);
        set_point(&mut e, point, [1.0, 2.0, 6.5]);
        assert!(e.call(0x00c5_0380, &args![this, point]).bool());
        set_point(&mut e, point, [1.0, 2.0, 7.0]);
        assert!(!e.call(0x00c5_0380, &args![this, point]).bool());
    }
}
