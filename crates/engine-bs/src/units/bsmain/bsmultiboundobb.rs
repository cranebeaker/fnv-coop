//! `BSMain/bsmultiboundobb.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c38000` (named `BSMultiBoundSphere::LinkObject` in the engine map): the base body
/// the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const LINK_OBJECT_TARGET_00C38000: u32 = 0x00c3_8000;

// Translated from 00c36430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::LinkObject` (engine map vtable name, vtable word 0101e9f4): forwards
/// `this` and the stream to `00c38000`, the `LinkObject` body of `BSMultiBoundAABB`'s table
/// (the engine map names that address `BSMultiBoundSphere::LinkObject`). Returns nothing: Xbox
/// PDB `?LinkObject@NiObject@@UAAXAAVNiStream@@@Z` is void; `stream` is the `NiStream`
/// reference as an address. Parameter count from the `RET 4` of both bodies.
pub fn bs_multi_bound_obb_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINK_OBJECT_TARGET_00C38000, &args![this, stream]);
}

// ---- Batch n067 ---------------------------------------------------------------------------
//
// A `BSMultiBoundOBB` is a `BSMultiBoundAABB` (0x24 bytes on PC: the shape base, `Center` at
// +0xC, `HalfExtents` at +0x18) followed by `Axis` (Xbox PDB, an `NiMatrix3`, nine floats at
// +0x24 on PC; +0x30 on the Xbox, whose points are 16-byte aligned): 0x48 bytes, vtable
// `0101e9a4`. Column `i` of `Axis` (the floats at +0x24, +0x30, +0x3C plus `4 * i`) is the
// box's axis `i`, along which it reaches `HalfExtents[i]` from the centre.
//
// Float comparisons follow the exe's flag tests (`FCOMP` + `TEST AH,mask` + `JP`/`JNZ`), so an
// unordered (NaN) value takes the branch the exe takes; every x87 operation goes through
// `crate::x87` in the exe's order, and values the exe copies with `FLD`/`FSTP` are quieted.

/// `Axis` (Xbox PDB), the nine floats of the box's orientation (PC offset).
const AXIS: u32 = 0x24;
/// `HalfExtents` (Xbox PDB) and `Center` of the `BSMultiBoundAABB` base.
const HALF_EXTENTS: u32 = 0x18;
const CENTER: u32 = 0x0c;
/// The `BSMultiBoundOBB` vtable and the object's size on PC.
const OBB_VTABLE: u32 = 0x0101_e9a4;
const OBB_SIZE: u32 = 0x48;
/// `BSMultiBoundAABB`'s constructor, `CopyMembers`, `LoadBinary`, `SaveBinary`, `IsEqual`,
/// `GetViewerStrings` and `GetVertices` (Xbox PDB names, `bsmultiboundaabb.obj`).
const AABB_CONSTRUCT: u32 = 0x00c3_7ec0;
const AABB_COPY_MEMBERS: u32 = 0x00c3_7f70;
const AABB_LOAD_BINARY: u32 = 0x00c3_7fc0;
const AABB_SAVE_BINARY: u32 = 0x00c3_8050;
const AABB_IS_EQUAL: u32 = 0x00c3_8090;
const AABB_GET_VIEWER_STRINGS: u32 = 0x00c3_8220;
const AABB_GET_VERTICES: u32 = 0x00c3_9270;
/// The body the map names `BSMultiBoundSphere::RegisterStreamables` (`00c38020`).
const REGISTER_STREAMABLES_00C38020: u32 = 0x00c3_8020;
/// `NiMatrix3::LoadBinary`, `SaveBinary`, `operator==` and `GetViewerString` (Xbox PDB).
const MATRIX_LOAD_BINARY: u32 = 0x00a5_9a30;
const MATRIX_SAVE_BINARY: u32 = 0x00a5_9a90;
const MATRIX_EQUALS: u32 = 0x004d_9ae0;
const MATRIX_GET_VIEWER_STRING: u32 = 0x00a5_98f0;
/// `NiGetViewerString` (cdecl, the name) and the word at `01202818` it is given.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const TYPE_NAME_WORD: u32 = 0x0120_2818;
/// Adds a string to the viewer strings (`this` the strings, the address of a word holding
/// the string).
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// `"Axis"`, the label of the matrix's viewer string.
const AXIS_LABEL: u32 = 0x0104_4b20;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiPoint3::Dot` (Xbox PDB; `this` and the other point, the result in `ST0`).
const DOT: u32 = 0x004b_6190;
/// `fabs` (`00ec6cde`, a `double` in and out).
const FABS: u32 = 0x00ec_6cde;
/// The `double` 0.0 (`01012060`) and the tolerance of `QPointWithin` (`01016408`, the
/// `double` nearest the float 0.01).
const ZERO: u32 = 0x0101_2060;
const TOLERANCE: u32 = 0x0101_6408;
/// The point the exe initialises its axis locals with before overwriting them (three words
/// at `011f426c`).
const START_POINT: u32 = 0x011f_426c;
/// `NiPlane::NiPlane` (Xbox PDB; no arguments).
const PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// Plane test (`0049da80`, `this` the plane, then a point): 2 for "outside".
const PLANE_SIDE_OF_POINT: u32 = 0x0049_da80;
/// Length of a point (`00457990`, the result in `ST0`).
const POINT_LENGTH: u32 = 0x0045_7990;
/// A point scaled in place by a float (`00439180`, `this` and the float; returns `this`).
const POINT_SCALE_IN_PLACE: u32 = 0x0043_9180;
/// `NiPoint3` addition and subtraction (`00439e90`, `00439ef0`): `(this, out, other)` writes
/// `this + other` / `this - other` to `out` and returns `out`.
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiTriShapeData::GetTriList` (Xbox PDB; `this` the data).
const GET_TRI_LIST: u32 = 0x00a7_b850;
/// The four colour words of the debug lines (`011a9be0`).
const LINE_COLOR: u32 = 0x011a_9be0;
/// Vtable slots (byte offsets): `QType`, `CheckBound` for a bound sphere, `GetNiBound`.
const SLOT_TYPE: u32 = 0x8c;
const SLOT_CHECK_BOUND_SPHERE: u32 = 0x98;
const SLOT_GET_NI_BOUND: u32 = 0xa4;
/// The box's edges as pairs of vertex indices (the 24 words `MakeDebugGeom` stores).
const EDGE_VERTICES: [u16; 24] = [
    0, 1, 0, 2, 1, 3, 3, 2, 4, 5, 4, 6, 5, 7, 7, 6, 0, 4, 1, 5, 2, 6, 3, 7,
];
/// The box's triangles as vertex indices (the 36 words `MakeDebugGeom` stores).
const TRIANGLE_VERTICES: [u16; 36] = [
    2, 3, 1, 2, 1, 0, 5, 7, 6, 5, 6, 4, 7, 2, 6, 7, 3, 2, 4, 0, 5, 5, 0, 1, 6, 0, 4, 6, 2, 0, 3, 7,
    1, 7, 5, 1,
];

/// Copies `count` words from `from` to `to` (`MOV` copies: no quieting).
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// Fills the three words at `to` from the start point (`011f426c`), as the exe initialises a
/// local point.
fn start_point(e: &mut Engine, to: u32) {
    copy_words(e, START_POINT, to, 3);
}

/// Writes axis `axis` (column `axis` of `Axis`) to the point at `to`, each float copied with
/// `FLD`/`FSTP`.
fn load_axis(e: &mut Engine, this: Ptr, axis: u32, to: u32) {
    for row in 0..3 {
        let value = e.mem.u32(this.addr() + AXIS + 4 * axis + 12 * row);
        e.mem.set_u32(to + 4 * row, x87_quiet_bits(value));
    }
}

/// `FLD float [value]; FSTP double [ESP]; CALL fabs; FSTP float`: the float's magnitude as
/// the exe stores it.
fn float_abs(e: &mut Engine, bits: u32) -> u32 {
    let magnitude = e.call(FABS, &args![x87_load32(bits)]).f64();
    x87_store32(magnitude)
}

/// The plane test of `CheckBound` and `QPointWithin`: for plane `plane` of the three at
/// `planes` (16 bytes each: the normal, then the constant at +0xC), `|Dot(normal, point) -
/// constant|` (stored as a float, then copied) minus the half extent on that axis, compared
/// with `limit`: true when it is above it (an unordered value is not).
fn plane_distance_exceeds(
    e: &mut Engine,
    this: Ptr,
    planes: u32,
    plane: u32,
    point: u32,
    limit: f64,
) -> bool {
    let normal = planes + 0x10 * plane;
    let dot = e.call(DOT, &args![normal, point]).f64();
    let offset = x87_store32(x87_sub(dot, x87_m32(e.mem.u32(normal + 0xc))));
    let distance = x87_quiet_bits(float_abs(e, offset));
    let extent = e.mem.u32(this.addr() + HALF_EXTENTS + 4 * plane);
    x87_sub(x87_load32(distance), x87_m32(extent)) > limit
}

/// The three `NiPlane`s of the box in a 48-byte local (each constructed, then
/// `GetPlanes`), passed to `body`.
fn with_planes<R>(e: &mut Engine, this: Ptr, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(0x30, |e, planes| {
        for plane in 0..3 {
            e.call(PLANE_CONSTRUCT, &args![planes.addr() + 0x10 * plane]);
        }
        bs_multi_bound_obb_get_planes(e, this, planes);
        body(e, planes.addr())
    })
}

// Translated from 00c36310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::BSMultiBoundOBB` (Xbox PDB): the `BSMultiBoundAABB` constructor
/// (`00c37ec0`), then the class's vtable (`0101e9a4`). Returns `this`.
pub fn bs_multi_bound_obb_bs_multi_bound_obb(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(AABB_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), OBB_VTABLE);
    this
}

/// `operator new(0x48)` and the constructor when the block is not null.
fn new_obb(e: &mut Engine) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![OBB_SIZE]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        bs_multi_bound_obb_bs_multi_bound_obb(e, Ptr::new(block))
    }
}

// Translated from 00c36330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c36330 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// `BSMultiBoundOBB::CreateClone` (Xbox PDB): a new box (`operator new(0x48)`, constructed
/// unless the allocation failed), then this box's members copied into it
/// (`CopyMembers`, which runs on a null clone too). Returns the clone.
pub fn bs_multi_bound_obb_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = new_obb(e);
    bs_multi_bound_obb_copy_members(e, this, clone, cloning);
    clone
}

// Translated from 00c363c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::CopyMembers` (Xbox PDB): the `BSMultiBoundAABB` part (`00c37f70`), then
/// the nine `Axis` words (`REP MOVSD`).
pub fn bs_multi_bound_obb_copy_members(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(AABB_COPY_MEMBERS, &args![this, clone, cloning]);
    copy_words(e, this.addr() + AXIS, clone.addr() + AXIS, 9);
}

// Translated from 00c36400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::LoadBinary` (Xbox PDB): the `BSMultiBoundAABB` part, then `Axis`
/// (`NiMatrix3::LoadBinary`).
pub fn bs_multi_bound_obb_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(AABB_LOAD_BINARY, &args![this, stream]);
    e.call(MATRIX_LOAD_BINARY, &args![this.addr() + AXIS, stream]);
}

// Translated from 00c36450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::RegisterStreamables` (Xbox PDB): the body at `00c38020`, its byte
/// normalised to true or false.
pub fn bs_multi_bound_obb_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(REGISTER_STREAMABLES_00C38020, &args![this, stream])
        .bool()
}

// Translated from 00c36480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::SaveBinary` (Xbox PDB): the `BSMultiBoundAABB` part, then `Axis`
/// (`NiMatrix3::SaveBinary`).
pub fn bs_multi_bound_obb_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(AABB_SAVE_BINARY, &args![this, stream]);
    e.call(MATRIX_SAVE_BINARY, &args![this.addr() + AXIS, stream]);
}

// Translated from 00c364b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::IsEqual` (Xbox PDB): the `BSMultiBoundAABB` test, and when it holds the
/// two `Axis` matrices compared (`NiMatrix3::operator==`, this box's first).
pub fn bs_multi_bound_obb_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let equal = e.call(AABB_IS_EQUAL, &args![this, other]).bool();
    if !equal {
        return false;
    }
    e.call(
        MATRIX_EQUALS,
        &args![this.addr() + AXIS, other.addr() + AXIS],
    )
    .bool()
}

// Translated from 00c36500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c36500 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
/// The class's stream creator (no arguments; unnamed in the map, Ghidra's name is a folded
/// library one): a new box from `operator new(0x48)`, constructed; null when the
/// allocation fails.
pub fn fn_00c36500(e: &mut Engine) -> Ptr {
    new_obb(e)
}

/// Adds `string` to `strings` through a local word holding it, as the exe does.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

// Translated from 00c36580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::GetViewerStrings` (Xbox PDB): the `BSMultiBoundAABB` strings, then the
/// string `NiGetViewerString` makes of the word at `01202818`, then `Axis`'s string
/// (`NiMatrix3::GetViewerString` with `"Axis"`), each added through `0096a610`.
pub fn bs_multi_bound_obb_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(AABB_GET_VIEWER_STRINGS, &args![this, strings]);
    let type_name = e.mem.u32(TYPE_NAME_WORD);
    let first = e.call(GET_VIEWER_STRING, &args![type_name]).u32();
    add_viewer_string(e, strings, first);
    let second = e
        .call(
            MATRIX_GET_VIEWER_STRING,
            &args![this.addr() + AXIS, AXIS_LABEL],
        )
        .u32();
    add_viewer_string(e, strings, second);
}

// Translated from 00c365e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot 0x8c of `BSMultiBoundOBB` (`QType`, Xbox PDB, in
/// the shape classes' tables): returns 2, the type the other shapes' `CheckBound` treat as
/// the oriented box.
pub fn fn_00c365e0(_e: &mut Engine, _this: Ptr) -> u32 {
    2
}

// Translated from 00c365f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::CheckBound` for a bound sphere (Xbox PDB, slot 0x98; `sphere` points to
/// the centre and the radius at +0xC). For each axis the distance between the centres
/// along it (`|Dot(sphere centre, axis) - Dot(axis, box centre)|`, stored as a float) is
/// tested: "inside" while it minus (half extent minus radius) is not above zero, "touching"
/// while it minus (half extent plus radius) is not; the later axes run only while either
/// holds, and the inside test only while it holds. Returns 2 when inside, else 1 when
/// touching, else 0. The axis points start as the start point (`011f426c`) and get their
/// floats through `FLD`/`FSTP`; the sphere's centre is copied word by word.
pub fn bs_multi_bound_obb_check_bound(e: &mut Engine, this: Ptr, sphere: Ptr) -> u32 {
    let center = this.addr() + CENTER;
    e.with_stack(48, |e, locals| {
        let sphere_center = locals.addr();
        copy_words(e, sphere.addr(), sphere_center, 3);
        let radius = x87_quiet_bits(e.mem.u32(sphere.addr() + 0xc));
        let mut inside = true;
        let mut touching = true;
        let axes = [locals.addr() + 12, locals.addr() + 24, locals.addr() + 36];
        for axis in axes {
            start_point(e, axis);
        }
        let zero: f64 = e.global(ZERO);
        for (index, axis) in (0u32..).zip(axes) {
            if index > 0 && !inside && !touching {
                break;
            }
            load_axis(e, this, index, axis);
            let own = e.call(DOT, &args![axis, center]).f64();
            let other = e.call(DOT, &args![sphere_center, axis]).f64();
            let offset = x87_store32(x87_sub(other, own));
            let distance = x87_m32(x87_quiet_bits(float_abs(e, offset)));
            let extent = x87_load32(e.mem.u32(this.addr() + HALF_EXTENTS + 4 * index));
            if inside {
                let reach = x87_sub(extent, x87_m32(radius));
                inside &= x87_sub(distance, reach) <= zero;
            }
            let reach = x87_add(
                x87_load32(e.mem.u32(this.addr() + HALF_EXTENTS + 4 * index)),
                x87_m32(radius),
            );
            touching &= x87_sub(distance, reach) <= zero;
        }
        if inside {
            2
        } else {
            touching as u32
        }
    })
}

/// The vertex test of `CheckBound` for another box: each of the eight vertices at
/// `vertices` must lie within this box's half extent on every axis (the tests run vertex
/// by vertex, axis by axis); 0 as soon as one does not, else 2.
fn vertices_within(e: &mut Engine, this: Ptr, vertices: u32) -> u32 {
    let zero: f64 = e.global(ZERO);
    with_planes(e, this, |e, planes| {
        for vertex in 0..8u32 {
            for plane in 0..3u32 {
                if plane_distance_exceeds(e, this, planes, plane, vertices + 12 * vertex, zero) {
                    return 0;
                }
            }
        }
        2
    })
}

// Translated from 00c369b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::CheckBound` for another bound (Xbox PDB `CheckBound_ov2`, slot 0x94;
/// `other` a `BSMultiBound`, its shape the word at +0xC): by the shape's `QType` (slot
/// 0x8c). Type 1: the shape's eight vertices (`BSMultiBoundAABB::GetVertices`) must all
/// lie within this box ([`vertices_within`]): 2, else 0. Type 2: the same with the oriented
/// box's vertices ([`bs_multi_bound_obb_get_vertices`]). Type 3: the shape's `GetNiBound`
/// (slot 0xa4) fills a bound that this box's slot 0x98 checks; its result is returned. Any
/// other type: 0. The shape word is read again in each branch, as the exe does.
pub fn bs_multi_bound_obb_check_bound_ov2(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let shape = e.mem.u32(other.addr() + 0xc);
    match e.vcall(shape, SLOT_TYPE, &[]).u32() {
        1 => {
            let shape = e.mem.u32(other.addr() + 0xc);
            e.with_stack(96, |e, vertices| {
                e.call(AABB_GET_VERTICES, &args![shape, vertices]);
                vertices_within(e, this, vertices.addr())
            })
        }
        2 => {
            let shape = e.mem.u32(other.addr() + 0xc);
            e.with_stack(96, |e, vertices| {
                bs_multi_bound_obb_get_vertices(e, Ptr::new(shape), vertices);
                vertices_within(e, this, vertices.addr())
            })
        }
        3 => {
            let shape = e.mem.u32(other.addr() + 0xc);
            e.with_stack(16, |e, bound| {
                e.vcall(shape, SLOT_GET_NI_BOUND, &args![bound]);
                e.vcall(this.addr(), SLOT_CHECK_BOUND_SPHERE, &args![bound])
                    .u32()
            })
        }
        _ => 0,
    }
}

// Translated from 00c36f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::WithinFrustum` (Xbox PDB): `frustum` holds six planes of 16 bytes and a
/// plane mask at +0x60 (read for each plane). The eight vertices are tested against each
/// plane of the mask with `0049da80`; false as soon as all tested vertices of a plane test 2
/// ("outside"). When the third half extent compares equal to 0.0 only the odd vertices are
/// tested. True otherwise.
pub fn bs_multi_bound_obb_within_frustum(e: &mut Engine, this: Ptr, frustum: Ptr) -> bool {
    e.with_stack(96, |e, vertices| {
        let zero: f64 = e.global(ZERO);
        let flat = x87_load32(e.mem.u32(this.addr() + 0x20)) == zero;
        bs_multi_bound_obb_get_vertices(e, this, vertices);
        for plane in 0..6u32 {
            if (1u32 << plane) & e.mem.u32(frustum.addr() + 0x60) == 0 {
                continue;
            }
            let mut all_outside = true;
            for vertex in 0..8u32 {
                if flat && vertex & 1 == 0 {
                    continue;
                }
                let side = e
                    .call(
                        PLANE_SIDE_OF_POINT,
                        &args![frustum.byte_add(plane << 4), vertices.byte_add(12 * vertex)],
                    )
                    .u32();
                if side != 2 {
                    all_outside = false;
                }
            }
            if all_outside {
                return false;
            }
        }
        true
    })
}

// Translated from 00c37050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::CompletelyWithinFrustum` (Xbox PDB): false as soon as any vertex tests
/// 2 ("outside", `0049da80`) against any plane of the frustum's mask (+0x60), true
/// otherwise.
pub fn bs_multi_bound_obb_completely_within_frustum(
    e: &mut Engine,
    this: Ptr,
    frustum: Ptr,
) -> bool {
    e.with_stack(96, |e, vertices| {
        bs_multi_bound_obb_get_vertices(e, this, vertices);
        for plane in 0..6u32 {
            if (1u32 << plane) & e.mem.u32(frustum.addr() + 0x60) == 0 {
                continue;
            }
            for vertex in 0..8u32 {
                let side = e
                    .call(
                        PLANE_SIDE_OF_POINT,
                        &args![frustum.byte_add(plane << 4), vertices.byte_add(12 * vertex)],
                    )
                    .u32();
                if side == 2 {
                    return false;
                }
            }
        }
        true
    })
}

// Translated from 00c37110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: vtable slot 0xa4 of `BSMultiBoundOBB` (`GetNiBound`, Xbox PDB,
/// in the shape classes' tables). Fills the bound at `out` with the centre and, as the
/// radius, the length of the half extents (`00457990`, stored as a float and copied).
pub fn fn_00c37110(e: &mut Engine, this: Ptr, out: Ptr) {
    let length = e
        .call(POINT_LENGTH, &args![this.addr() + HALF_EXTENTS])
        .f64();
    let radius = x87_store32(length);
    copy_words(e, this.addr() + CENTER, out.addr(), 3);
    e.mem.set_u32(out.addr() + 0xc, x87_quiet_bits(radius));
}

// Translated from 00c37150 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c37150 - the /GS stack-cookie check (00ec408c) at the end is not translated.
/// `BSMultiBoundOBB::MakeDebugGeom` (Xbox PDB, slot 0xa8): fills two debug geometries from
/// the box's eight vertices. `lines` gets its position (+0x58) set to the centre and 24 line
/// vertices (each edge's two vertices relative to the centre, `00439ef0`) written into its
/// data (the data at +0xB8: positions at +0x20, colours at +0x28, flags at +0x40),
/// coloured with the four words at `011a9be0` and flagged 1, 0 per edge. `triangles` gets
/// the same position, 36 vertices (the 12 triangles) coloured with the four given words,
/// and its triangle list (`NiTriShapeData::GetTriList`) filled with the indices 0..35.
#[allow(clippy::too_many_arguments)]
pub fn bs_multi_bound_obb_make_debug_geom(
    e: &mut Engine,
    this: Ptr,
    lines: Ptr,
    triangles: Ptr,
    color_0: u32,
    color_1: u32,
    color_2: u32,
    color_3: u32,
) {
    let center = this.addr() + CENTER;
    e.with_stack(96, |e, vertices| {
        bs_multi_bound_obb_get_vertices(e, this, vertices);
        let triangle_color = [color_0, color_1, color_2, color_3];
        let line_color = [
            e.mem.u32(LINE_COLOR),
            e.mem.u32(LINE_COLOR + 4),
            e.mem.u32(LINE_COLOR + 8),
            e.mem.u32(LINE_COLOR + 12),
        ];
        let line_data = e.mem.u32(lines.addr() + 0xb8);
        let positions = e.mem.u32(line_data + 0x20);
        let line_data = e.mem.u32(lines.addr() + 0xb8);
        let colors = e.mem.u32(line_data + 0x28);
        let line_data = e.mem.u32(lines.addr() + 0xb8);
        let flags = e.mem.u32(line_data + 0x40);
        copy_words(e, center, lines.addr() + 0x58, 3);
        // One 12-byte temporary per subtraction call site (five in the exe).
        e.with_stack(60, |e, temporaries| {
            for edge in 0..12u32 {
                for end in 0..2u32 {
                    let slot = edge * 2 + end;
                    let vertex = u32::from(EDGE_VERTICES[slot as usize]);
                    let relative = e
                        .call(
                            POINT_SUBTRACT,
                            &args![
                                vertices.addr() + 12 * vertex,
                                temporaries.addr() + 12 * end,
                                center
                            ],
                        )
                        .u32();
                    copy_words(e, relative, positions + 12 * slot, 3);
                    for (word, value) in (0u32..).zip(line_color) {
                        e.mem.set_u32(colors + 16 * slot + 4 * word, value);
                    }
                }
                e.mem.set_u8(flags + edge * 2, 1);
                e.mem.set_u8(flags + edge * 2 + 1, 0);
            }
            let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
            let positions = e.mem.u32(triangle_data + 0x20);
            let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
            let colors = e.mem.u32(triangle_data + 0x28);
            let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
            let list = e.call(GET_TRI_LIST, &args![triangle_data]).u32();
            copy_words(e, center, triangles.addr() + 0x58, 3);
            for triangle in 0..12u32 {
                for corner in 0..3u32 {
                    let slot = triangle * 3 + corner;
                    let vertex = u32::from(TRIANGLE_VERTICES[slot as usize]);
                    let relative = e
                        .call(
                            POINT_SUBTRACT,
                            &args![
                                vertices.addr() + 12 * vertex,
                                temporaries.addr() + 24 + 12 * corner,
                                center
                            ],
                        )
                        .u32();
                    copy_words(e, relative, positions + 12 * slot, 3);
                    for (word, value) in (0u32..).zip(triangle_color) {
                        e.mem.set_u32(colors + 16 * slot + 4 * word, value);
                    }
                }
                for corner in 0..3u32 {
                    let index = triangle * 3 + corner;
                    e.mem.set_u16(list + 2 * index, index as u16);
                }
            }
        });
    });
}

// Translated from 00c378c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::QPointWithin` (Xbox PDB): whether `point` lies within the box: on each
/// axis in turn (the planes of [`bs_multi_bound_obb_get_planes`]), `|Dot(normal, point) -
/// constant|` minus the half extent must not be above the tolerance `01016408` (an
/// unordered value passes); false at the first axis where it is.
pub fn bs_multi_bound_obb_q_point_within(e: &mut Engine, this: Ptr, point: Ptr) -> bool {
    let tolerance: f64 = e.global(TOLERANCE);
    with_planes(e, this, |e, planes| {
        for plane in 0..3u32 {
            if plane_distance_exceeds(e, this, planes, plane, point.addr(), tolerance) {
                return false;
            }
        }
        true
    })
}

// Translated from 00c37a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::GetVertices` (Xbox PDB): writes the eight corners of the box (points
/// of 12 bytes) at `out`. The three axis points (each starting as the start point
/// `011f426c`, then column `i` of `Axis` copied in) are scaled in place by their half
/// extents (`00439180`); corner `n` is the centre plus or minus the three in turn
/// (`00439e90` adds, `00439ef0` subtracts, each call on the previous one's result): bit 2 of
/// `n` makes the first a subtraction, bit 1 the second and bit 0 the third.
pub fn bs_multi_bound_obb_get_vertices(e: &mut Engine, this: Ptr, out: Ptr) {
    // The three axis points, then one temporary per add/subtract call (24).
    e.with_stack(12 * (3 + 24), |e, scratch| {
        // The exe's locals at -0xc, -0x24, -0x18: axes 0, 1, 2.
        let axes = [scratch.addr(), scratch.addr() + 12, scratch.addr() + 24];
        start_point(e, axes[0]);
        start_point(e, axes[1]);
        start_point(e, axes[2]);
        for (index, axis) in (0u32..).zip(axes) {
            load_axis(e, this, index, axis);
            let extent = x87_quiet_bits(e.mem.u32(this.addr() + HALF_EXTENTS + 4 * index));
            e.call(POINT_SCALE_IN_PLACE, &args![axis, extent]);
        }
        for corner in 0..8u32 {
            let mut current = this.addr() + CENTER;
            for (index, axis) in (0u32..).zip(axes) {
                let operation = if corner >> (2 - index) & 1 == 0 {
                    POINT_ADD
                } else {
                    POINT_SUBTRACT
                };
                let temporary = scratch.addr() + 36 + 12 * (corner * 3 + index);
                current = e.call(operation, &args![current, temporary, axis]).u32();
            }
            copy_words(e, current, out.addr() + 12 * corner, 3);
        }
    });
}

// Translated from 00c37e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundOBB::GetPlanes` (Xbox PDB): fills the three planes at `planes` (16 bytes
/// each): plane `i`'s normal is column `i` of `Axis` (built in a local point that starts as
/// `011f426c`, its floats copied with `FLD`/`FSTP`, then copied word by word), its constant
/// `Dot(normal, centre)` stored as a float and copied.
pub fn bs_multi_bound_obb_get_planes(e: &mut Engine, this: Ptr, planes: Ptr) {
    e.with_stack(12, |e, normal| {
        for plane in 0..3u32 {
            start_point(e, normal.addr());
            load_axis(e, this, plane, normal.addr());
            let target = planes.addr() + 0x10 * plane;
            copy_words(e, normal.addr(), target, 3);
            let dot = e.call(DOT, &args![normal, this.addr() + CENTER]).f64();
            let constant = x87_store32(dot);
            e.mem.set_u32(target + 0xc, x87_quiet_bits(constant));
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_6430, bs_multi_bound_obb_link_object(Ptr, Ptr)),
        entry!(0x00c3_6310, bs_multi_bound_obb_bs_multi_bound_obb(Ptr) -> Ptr),
        entry!(0x00c3_6330, bs_multi_bound_obb_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00c3_63c0, bs_multi_bound_obb_copy_members(Ptr, Ptr, Ptr)),
        entry!(0x00c3_6400, bs_multi_bound_obb_load_binary(Ptr, Ptr)),
        entry!(0x00c3_6450, bs_multi_bound_obb_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c3_6480, bs_multi_bound_obb_save_binary(Ptr, Ptr)),
        entry!(0x00c3_64b0, bs_multi_bound_obb_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c3_6500, fn_00c36500() -> Ptr),
        entry!(0x00c3_6580, bs_multi_bound_obb_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00c3_65e0, fn_00c365e0(Ptr) -> u32),
        entry!(0x00c3_65f0, bs_multi_bound_obb_check_bound(Ptr, Ptr) -> u32),
        entry!(0x00c3_69b0, bs_multi_bound_obb_check_bound_ov2(Ptr, Ptr) -> u32),
        entry!(0x00c3_6f20, bs_multi_bound_obb_within_frustum(Ptr, Ptr) -> bool),
        entry!(
            0x00c3_7050,
            bs_multi_bound_obb_completely_within_frustum(Ptr, Ptr) -> bool
        ),
        entry!(0x00c3_7110, fn_00c37110(Ptr, Ptr)),
        entry!(
            0x00c3_7150,
            bs_multi_bound_obb_make_debug_geom(Ptr, Ptr, Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00c3_78c0, bs_multi_bound_obb_q_point_within(Ptr, Ptr) -> bool),
        entry!(0x00c3_7a00, bs_multi_bound_obb_get_vertices(Ptr, Ptr)),
        entry!(0x00c3_7e00, bs_multi_bound_obb_get_planes(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_bs_multi_bound_obb_link_object_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(LINK_OBJECT_TARGET_00C38000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_6430, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == LINK_OBJECT_TARGET_00C38000)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
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

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_point(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (i, v) in (0u32..).zip(value) {
            e.mem.set_f32(at + 4 * i, v);
        }
    }

    /// An engine with the globals and the point arithmetic the box functions use.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.map(TOLERANCE, 8);
        e.set_global(TOLERANCE, 0.01f32 as f64);
        e.map(START_POINT, 12);
        e.register(DOT, |e, a| {
            let mut sum = 0.0f64;
            for i in 0..3 {
                sum += e.mem.f32(a[0] + 4 * i) as f64 * e.mem.f32(a[1] + 4 * i) as f64;
            }
            sum.into_ret()
        });
        e.register(FABS, |_, a| {
            f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
                .abs()
                .into_ret()
        });
        e.register(PLANE_CONSTRUCT, nothing);
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SCALE_IN_PLACE, |e, a| {
            let scale = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) * scale;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e
    }

    /// A box with the given centre and half extents; `axis` holds the nine `Axis` floats.
    fn obb(e: &mut Engine, center: [f32; 3], half: [f32; 3], axis: [f32; 9]) -> u32 {
        let this = e.mem.alloc(OBB_SIZE);
        set_point(e, this + CENTER, center);
        set_point(e, this + HALF_EXTENTS, half);
        for (i, v) in (0u32..).zip(axis) {
            e.mem.set_f32(this + AXIS + 4 * i, v);
        }
        this
    }

    const IDENTITY: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    /// Columns (0, 1, 0), (-1, 0, 0), (0, 0, 1).
    const TURNED: [f32; 9] = [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

    #[test]
    fn constructor_clone_factory_and_copy_members() {
        let mut e = Engine::new();
        e.register(AABB_CONSTRUCT, nothing);
        e.register(AABB_COPY_MEMBERS, nothing);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(OBB_SIZE);
        assert_eq!(e.call(0x00c3_6310, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), OBB_VTABLE);
        assert_eq!(calls_to(&e, AABB_CONSTRUCT), vec![vec![this]]);
        for i in 0..9 {
            e.mem.set_u32(this + AXIS + 4 * i, 0x7f80_0001 + i);
        }
        let clone = e.call(0x00c3_6330, &args![this, 0x77u32]).u32();
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(e.mem.u32(clone), OBB_VTABLE);
        assert_eq!(
            calls_to(&e, AABB_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        // `REP MOVSD`: the words as they are (no quieting).
        assert_eq!(e.mem.u32(clone + AXIS), 0x7f80_0001);
        assert_eq!(e.mem.u32(clone + AXIS + 32), 0x7f80_0009);
        let made = e.call(0x00c3_6500, &args![]).u32();
        assert_eq!(e.mem.u32(made), OBB_VTABLE);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00c3_6500, &args![]).u32(), 0);
    }

    #[test]
    fn load_save_register_and_viewer_strings_forward() {
        let mut e = Engine::new();
        for addr in [
            AABB_LOAD_BINARY,
            MATRIX_LOAD_BINARY,
            AABB_SAVE_BINARY,
            MATRIX_SAVE_BINARY,
            AABB_GET_VIEWER_STRINGS,
        ] {
            e.register(addr, nothing);
        }
        e.register(REGISTER_STREAMABLES_00C38020, |_, a| {
            (0x200 | a[1]).into_ret()
        });
        e.register(GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(MATRIX_GET_VIEWER_STRING, |_, _| 0x5555u32.into_ret());
        e.register_double(ADD_VIEWER_STRING, |e, a| {
            let string = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], string);
            Ret::default()
        });
        e.map(TYPE_NAME_WORD, 4);
        e.set_global(TYPE_NAME_WORD, 0x4440u32);
        e.call_log = Some(vec![]);
        e.call(0x00c3_6400, &args![0x100u32, 0x200u32]);
        assert_eq!(calls_to(&e, AABB_LOAD_BINARY), vec![vec![0x100, 0x200]]);
        assert_eq!(calls_to(&e, MATRIX_LOAD_BINARY), vec![vec![0x124, 0x200]]);
        e.call(0x00c3_6480, &args![0x100u32, 0x200u32]);
        assert_eq!(calls_to(&e, AABB_SAVE_BINARY), vec![vec![0x100, 0x200]]);
        assert_eq!(calls_to(&e, MATRIX_SAVE_BINARY), vec![vec![0x124, 0x200]]);
        assert!(e.call(0x00c3_6450, &args![0x100u32, 1u32]).bool());
        assert!(!e.call(0x00c3_6450, &args![0x100u32, 0u32]).bool());
        let strings = e.mem.alloc(4);
        e.call(0x00c3_6580, &args![0x100u32, strings]);
        assert_eq!(
            calls_to(&e, AABB_GET_VIEWER_STRINGS),
            vec![vec![0x100, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4440]]);
        assert_eq!(
            calls_to(&e, MATRIX_GET_VIEWER_STRING),
            vec![vec![0x124, AXIS_LABEL]]
        );
        assert_eq!(calls_to(&e, ADD_VIEWER_STRING).len(), 2);
        assert_eq!(e.mem.u32(strings), 0x5555);
        assert_eq!(e.call(0x00c3_65e0, &args![0x100u32]).u32(), 2);
    }

    #[test]
    fn is_equal_tests_the_matrix_only_after_the_box() {
        let mut e = Engine::new();
        e.register_double(AABB_IS_EQUAL, |_, a| u32::from(a[1] != 0x300).into_ret());
        e.register(MATRIX_EQUALS, |_, a| u32::from(a[1] == 0x224).into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c3_64b0, &args![0x100u32, 0x200u32]).bool());
        assert_eq!(calls_to(&e, MATRIX_EQUALS), vec![vec![0x124, 0x224]]);
        assert!(!e.call(0x00c3_64b0, &args![0x100u32, 0x300u32]).bool());
        assert_eq!(calls_to(&e, MATRIX_EQUALS).len(), 1);
        assert!(!e.call(0x00c3_64b0, &args![0x100u32, 0x400u32]).bool());
    }

    #[test]
    fn get_vertices_uses_the_axis_columns() {
        let mut e = engine();
        let this = obb(&mut e, [10.0, 20.0, 30.0], [1.0, 2.0, 3.0], TURNED);
        let out = e.mem.alloc(96);
        e.call(0x00c3_7a00, &args![this, out]);
        // Axes (0, 1, 0) * 1, (-1, 0, 0) * 2, (0, 0, 1) * 3.
        assert_eq!(point(&e, out), [8.0, 21.0, 33.0]);
        assert_eq!(point(&e, out + 12), [8.0, 21.0, 27.0]);
        assert_eq!(point(&e, out + 24), [12.0, 21.0, 33.0]);
        assert_eq!(point(&e, out + 48), [8.0, 19.0, 33.0]);
        assert_eq!(point(&e, out + 84), [12.0, 19.0, 27.0]);
    }

    #[test]
    fn get_planes_and_ni_bound() {
        let mut e = engine();
        let this = obb(&mut e, [1.0, 2.0, 3.0], [3.0, 4.0, 0.0], TURNED);
        let planes = e.mem.alloc(48);
        e.call(0x00c3_7e00, &args![this, planes]);
        assert_eq!(point(&e, planes), [0.0, 1.0, 0.0]);
        assert_eq!(e.mem.f32(planes + 0xc), 2.0);
        assert_eq!(point(&e, planes + 0x10), [-1.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(planes + 0x1c), -1.0);
        assert_eq!(e.mem.f32(planes + 0x2c), 3.0);
        e.register(POINT_LENGTH, |e, a| {
            let [x, y, z] = point(e, a[0]);
            ((x * x + y * y + z * z) as f64).sqrt().into_ret()
        });
        let out = e.mem.alloc(16);
        e.call(0x00c3_7110, &args![this, out]);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.f32(out + 0xc), 5.0);
    }

    #[test]
    fn check_bound_sphere_inside_touching_and_apart() {
        let mut e = engine();
        let this = obb(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], IDENTITY);
        let sphere = e.mem.alloc(16);
        set_point(&mut e, sphere, [0.5, 0.0, 0.0]);
        e.mem.set_f32(sphere + 0xc, 0.25);
        assert_eq!(e.call(0x00c3_65f0, &args![this, sphere]).u32(), 2);
        set_point(&mut e, sphere, [1.5, 0.0, 0.0]);
        e.mem.set_f32(sphere + 0xc, 1.0);
        assert_eq!(e.call(0x00c3_65f0, &args![this, sphere]).u32(), 1);
        // Apart on the first axis: the other two are not tested.
        set_point(&mut e, sphere, [5.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_65f0, &args![this, sphere]).u32(), 0);
        assert_eq!(calls_to(&e, DOT).len(), 2);
        // A NaN distance fails both tests.
        set_point(&mut e, sphere, [f32::NAN, 0.0, 0.0]);
        assert_eq!(e.call(0x00c3_65f0, &args![this, sphere]).u32(), 0);
    }

    fn shape_with_type(e: &mut Engine, kind: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0xc0);
        let type_function = 0x0090_0000 + kind;
        e.register_double(type_function, move |_, _| kind.into_ret());
        e.mem.set_u32(vtable + SLOT_TYPE, type_function);
        let shape = e.mem.alloc(0x48);
        e.mem.set_u32(shape, vtable);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 0xc, shape);
        (other, shape)
    }

    #[test]
    fn check_bound_other_box_and_bound() {
        let mut e = engine();
        let this = obb(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], IDENTITY);
        // Type 1: the axis-aligned box's vertices.
        let (other, shape) = shape_with_type(&mut e, 1);
        e.register_double(AABB_GET_VERTICES, |e, a| {
            for v in 0..8 {
                set_point(e, a[1] + 12 * v, [0.5, -0.5, 0.25]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 2);
        assert_eq!(calls_to(&e, AABB_GET_VERTICES)[0][0], shape);
        assert_eq!(calls_to(&e, PLANE_CONSTRUCT).len(), 3);
        e.register_double(AABB_GET_VERTICES, |e, a| {
            for v in 0..8 {
                set_point(e, a[1] + 12 * v, [0.5, 0.5, 0.5]);
            }
            set_point(e, a[1] + 12 * 5, [0.0, 0.0, 1.5]);
            Ret::default()
        });
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 0);
        // Type 2: an oriented box inside this one.
        let (other, shape) = shape_with_type(&mut e, 2);
        for (at, v) in [(CENTER, 0.0f32), (HALF_EXTENTS, 0.5)] {
            set_point(&mut e, shape + at, [v; 3]);
        }
        for (i, v) in (0u32..).zip(IDENTITY) {
            e.mem.set_f32(shape + AXIS + 4 * i, v);
        }
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 2);
        set_point(&mut e, shape + HALF_EXTENTS, [0.5, 2.0, 0.5]);
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 0);
        // Type 3: the bound through slot 0xa4, checked through this box's slot 0x98.
        let (other, shape) = shape_with_type(&mut e, 3);
        let vtable = e.mem.u32(shape);
        e.mem.set_u32(vtable + SLOT_GET_NI_BOUND, 0x0091_0000);
        e.register(0x0091_0000, nothing);
        let own_vtable = e.mem.alloc(0xc0);
        e.mem
            .set_u32(own_vtable + SLOT_CHECK_BOUND_SPHERE, 0x0091_0004);
        e.register(0x0091_0004, |_, _| 7u32.into_ret());
        e.mem.set_u32(this, own_vtable);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 7);
        let bound = calls_to(&e, 0x0091_0000)[0][1];
        assert_eq!(calls_to(&e, 0x0091_0004), vec![vec![this, bound]]);
        let (other, _) = shape_with_type(&mut e, 4);
        assert_eq!(e.call(0x00c3_69b0, &args![this, other]).u32(), 0);
    }

    #[test]
    fn frustum_tests() {
        let mut e = engine();
        let this = obb(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], IDENTITY);
        let frustum = e.mem.alloc(0x64);
        // Plane 0 has every vertex outside, plane 1 none.
        e.register_double(PLANE_SIDE_OF_POINT, move |_, a| {
            if a[0] == frustum {
                2u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.mem.set_u32(frustum + 0x60, 0b10);
        assert!(e.call(0x00c3_6f20, &args![this, frustum]).bool());
        assert!(e.call(0x00c3_7050, &args![this, frustum]).bool());
        e.mem.set_u32(frustum + 0x60, 0b11);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_6f20, &args![this, frustum]).bool());
        assert_eq!(calls_to(&e, PLANE_SIDE_OF_POINT).len(), 8);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_7050, &args![this, frustum]).bool());
        assert_eq!(calls_to(&e, PLANE_SIDE_OF_POINT).len(), 1);
        // A flat box (third half extent 0.0) tests only the odd vertices.
        e.mem.set_f32(this + HALF_EXTENTS + 8, 0.0);
        e.mem.set_u32(frustum + 0x60, 0b1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c3_6f20, &args![this, frustum]).bool());
        assert_eq!(calls_to(&e, PLANE_SIDE_OF_POINT).len(), 4);
    }

    #[test]
    fn q_point_within_uses_the_tolerance() {
        let mut e = engine();
        let this = obb(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0], TURNED);
        let p = e.mem.alloc(12);
        set_point(&mut e, p, [0.0, 1.005, 0.0]);
        assert!(e.call(0x00c3_78c0, &args![this, p]).bool());
        set_point(&mut e, p, [0.0, 0.0, 1.5]);
        assert!(!e.call(0x00c3_78c0, &args![this, p]).bool());
        set_point(&mut e, p, [f32::NAN, 0.0, 0.0]);
        assert!(e.call(0x00c3_78c0, &args![this, p]).bool());
    }

    #[test]
    fn make_debug_geom_fills_lines_and_triangles() {
        let mut e = engine();
        let this = obb(&mut e, [10.0, 0.0, 0.0], [1.0, 1.0, 1.0], IDENTITY);
        e.map(LINE_COLOR, 16);
        for i in 0..4 {
            e.set_global(LINE_COLOR + 4 * i, 0xc0 + i);
        }
        let geometry = |e: &mut Engine| {
            let object = e.mem.alloc(0xc0);
            let data = e.mem.alloc(0x44);
            e.mem.set_u32(object + 0xb8, data);
            for (field, size) in [(0x20u32, 36 * 12), (0x28, 36 * 16), (0x40, 24)] {
                let block = e.mem.alloc(size);
                e.mem.set_u32(data + field, block);
            }
            (object, data)
        };
        let (lines, line_data) = geometry(&mut e);
        let (triangles, triangle_data) = geometry(&mut e);
        let list = e.mem.alloc(72);
        e.register_double(GET_TRI_LIST, move |_, _| list.into_ret());
        e.call(
            0x00c3_7150,
            &args![this, lines, triangles, 1u32, 2u32, 3u32, 4u32],
        );
        assert_eq!(point(&e, lines + 0x58), [10.0, 0.0, 0.0]);
        assert_eq!(point(&e, triangles + 0x58), [10.0, 0.0, 0.0]);
        let positions = e.mem.u32(line_data + 0x20);
        // Edge 0 joins vertex 0 (+,+,+) and vertex 1 (+,+,-).
        assert_eq!(point(&e, positions), [1.0, 1.0, 1.0]);
        assert_eq!(point(&e, positions + 12), [1.0, 1.0, -1.0]);
        let colors = e.mem.u32(line_data + 0x28);
        assert_eq!(e.mem.u32(colors + 16 * 23 + 12), 0xc3);
        let flags = e.mem.u32(line_data + 0x40);
        assert_eq!((e.mem.u8(flags + 22), e.mem.u8(flags + 23)), (1, 0));
        let positions = e.mem.u32(triangle_data + 0x20);
        // Triangle 0 starts at vertex 2 (+,-,+).
        assert_eq!(point(&e, positions), [1.0, -1.0, 1.0]);
        let colors = e.mem.u32(triangle_data + 0x28);
        assert_eq!(e.mem.u32(colors + 16 * 35), 1);
        assert_eq!(e.mem.u32(colors + 16 * 35 + 12), 4);
        assert_eq!(e.mem.u16(list + 70), 35);
    }
}
