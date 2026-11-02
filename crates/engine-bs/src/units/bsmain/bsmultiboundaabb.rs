//! `BSMain/bsmultiboundaabb.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSMultiBoundAABB` is a `BSMultiBoundShape` (0xC bytes: the `NiObject`
//! base and `kCullResult` at +8) followed by an axis-aligned box: the centre
//! (`Center`, +0xC) and the half extents (`HalfExtents`, +0x18), 0x24 bytes in
//! all; its vtable is at `0101e8e4`. Its virtual slots (Xbox PDB names, slot
//! numbers checked against the vtable) are `QType` 0x8c, `CheckBound` 0x94
//! (a `BSMultiBound`) and 0x98 (a bound sphere), `WithinFrustum` 0x9c,
//! `CompletelyWithinFrustum` 0xa0, `GetNiBound` 0xa4, `MakeDebugGeom` 0xa8,
//! `QPointWithin` 0xb4 and `SetCenter` 0xb8.
//!
//! Float comparisons follow the exe's flag tests: the `FCOMP`/`FUCOMPP` +
//! `TEST AH,mask` + `JP`/`JZ` sequences are transcribed per instruction, so
//! unordered (NaN) operands take the branch the exe takes. Values the exe
//! copies through `FLD`/`FSTP` are quieted (a signaling NaN becomes quiet).
//!
//! Functions with the compiler's C++ exception-unwinding frame (`FS:[0]`) or
//! its stack-cookie check (`00ec408c`) are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiPoint3;

/// Address of the `BSMultiBoundAABB` vtable.
pub(crate) const BS_MULTI_BOUND_AABB_VTABLE: u32 = 0x0101_e8e4;
/// Size of a `BSMultiBoundAABB`.
pub(crate) const BS_MULTI_BOUND_AABB_SIZE: u32 = 0x24;

/// `BSMultiBoundShape::BSMultiBoundShape()` (thiscall).
const SHAPE_CONSTRUCT: u32 = 0x00c4_f980;
/// `BSMultiBoundShape::CopyMembers(this, target, cloneProcess)`.
const SHAPE_COPY_MEMBERS: u32 = 0x00c4_f9b0;
/// `BSMultiBoundShape::LoadBinary(this, stream)`.
const SHAPE_LOAD_BINARY: u32 = 0x00c4_f9f0;
/// The body the map names `BSMultiBoundShape::LinkObject` (identical-code
/// folding with the shape's `SaveBinary`): `(this, stream)`.
const SHAPE_SAVE_BINARY: u32 = 0x00c4_fa60;
/// `BSMultiBoundShape::IsEqual(this, other)`: bool.
const SHAPE_IS_EQUAL: u32 = 0x00c4_fa80;
/// `BSMultiBound::GetViewerStrings(this, strings)` (map name, body in the
/// shape's unit).
const SHAPE_GET_VIEWER_STRINGS: u32 = 0x00c4_fab0;
/// `NiColor::LoadBinary(this, stream)` (map name; the box loads its two
/// points with it).
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiColor::SaveBinary(this, stream)` (map name).
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
/// `NiPoint3::GetViewerString(this, name)` (Xbox PDB name): returns the
/// string.
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString(name)` (cdecl; Xbox PDB name): returns the string.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// The word `01202820`, the argument of the first viewer string.
const TYPE_NAME_WORD: u32 = 0x0120_2820;
/// Adds a string to the viewer strings (`this` the strings; the argument is
/// the address of a word holding the string).
const ADD_VIEWER_STRING: u32 = 0x0096_a610;
/// The viewer-string labels: `"Center"` and `"Half Extents"`.
const CENTER_LABEL: u32 = 0x010c_1040;
const HALF_EXTENTS_LABEL: u32 = 0x010c_1030;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `fabs` (`00ec6cde`), double in, double out.
const FABS: u32 = 0x00ec_6cde;
/// `NiPoint3` addition and subtraction (`multiboundmarkerdata.cpp` range in
/// the map; bodies confirmed): `(this, out, other)` writes `this + other` /
/// `this - other` to `out` and returns `out`.
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiPoint3` scaled by a float (`tes.cpp` range): `(this, out, scale)`.
const POINT_SCALE: u32 = 0x0045_bb20;
/// Length of a `NiPoint3` (`tes.cpp` range; the float is returned in `ST0`).
const POINT_LENGTH: u32 = 0x0045_7990;
/// The three unit axis points (`011a946c`, `011a9478`, `011a9484`).
const UNIT_AXES: [u32; 3] = [0x011a_946c, 0x011a_9478, 0x011a_9484];
/// Plane test (`0049da80`, `this` the plane, then a point): returns 2 when
/// the point is on the positive side by the plane's test.
const PLANE_SIDE_OF_POINT: u32 = 0x0049_da80;
/// `BSMultiBoundOBB::GetVertices(this, out)` (map name).
const OBB_GET_VERTICES: u32 = 0x00c3_7a00;
/// `NiTriShapeData::GetTriList` (Xbox PDB name; `this` the data).
const GET_TRI_LIST: u32 = 0x00a7_b850;
/// The double compared against (0.0, `01012060`) and the tolerance
/// (`01016408`, 0.01).
const ZERO: u32 = 0x0101_2060;
const TOLERANCE: u32 = 0x0101_6408;
/// The four colour words of the lines (`011a9be0`).
const LINE_COLOR: u32 = 0x011a_9be0;

/// Vtable slots (byte offsets).
const SLOT_TYPE: u32 = 0x8c;
const SLOT_CHECK_BOUND_SPHERE: u32 = 0x98;
const SLOT_GET_NI_BOUND: u32 = 0xa4;
const SLOT_SET_CENTER: u32 = 0xb8;

/// The edges of the debug box as pairs of vertex indices (12 edges).
const EDGE_VERTICES: [u16; 24] = [
    0, 1, 0, 2, 1, 3, 3, 2, 4, 5, 4, 6, 5, 7, 7, 6, 0, 4, 1, 5, 2, 6, 3, 7,
];
/// The triangles of the debug box as vertex indices (12 triangles).
const TRIANGLE_VERTICES: [u16; 36] = [
    2, 3, 1, 2, 1, 0, 5, 7, 6, 5, 6, 4, 7, 2, 6, 7, 3, 2, 4, 0, 5, 5, 0, 1, 6, 0, 4, 6, 2, 0, 3, 7,
    1, 7, 5, 1,
];

layout! {
    /// `BSMultiBoundAABB` (Xbox PDB), 0x24 bytes on PC.
    pub struct BSMultiBoundAABB: 0x24 {
        /// `Center` (Xbox PDB).
        0x0C Center: Inline<NiPoint3>,
        /// `HalfExtents` (Xbox PDB).
        0x18 HalfExtents: Inline<NiPoint3>,
    }
}

/// A float the exe copies through `FLD` and `FSTP`: a signaling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    (value as f64) as f32
}

/// `|value|` as the exe computes it for a float: copy through the x87 stack,
/// `fabs` of the double (`00ec6cde`), store as a float.
fn absolute(e: &mut Engine, value: f32) -> f32 {
    let magnitude = e.call(FABS, &args![quiet(value) as f64]).f64() as f32;
    quiet(magnitude)
}

/// Copies `count` words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The point test of `QPointWithin` and `FastPointWithin`: the point minus the
/// box centre (`00439ef0`), and for each axis the magnitude minus the half
/// extent must not exceed the tolerance `01016408` (the comparison is
/// "greater", so an unordered value passes).
fn point_within(e: &mut Engine, this: Ptr<BSMultiBoundAABB>, point: Ptr) -> bool {
    let center = this.at(BSMultiBoundAABB::Center);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    e.with_stack(12, |e, offset| {
        e.call(POINT_SUBTRACT, &args![point, offset, center]);
        let tolerance: f64 = e.global(TOLERANCE);
        for axis in 0..3u32 {
            let value = e.mem.f32(offset.addr() + 4 * axis);
            let magnitude = absolute(e, value);
            let extent = e.mem.f32(half.addr() + 4 * axis);
            if magnitude as f64 - extent as f64 > tolerance {
                return false;
            }
        }
        true
    })
}

// Translated from 00c37ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::BSMultiBoundAABB` (Xbox PDB): builds the shape base and
/// sets the vtable. Returns `this`.
pub fn bs_multi_bound_aabb_bs_multi_bound_aabb(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
) -> Ptr<BSMultiBoundAABB> {
    e.call(SHAPE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BS_MULTI_BOUND_AABB_VTABLE);
    this
}

/// Allocates a `BSMultiBoundAABB` and constructs it (null when the
/// allocation failed).
fn new_aabb(e: &mut Engine) -> Ptr<BSMultiBoundAABB> {
    let block = e
        .call(OPERATOR_NEW, &args![BS_MULTI_BOUND_AABB_SIZE])
        .ptr::<BSMultiBoundAABB>();
    if block.is_null() {
        Ptr::NULL
    } else {
        bs_multi_bound_aabb_bs_multi_bound_aabb(e, block)
    }
}

// Translated from 00c37ee0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c37ee0 - C++ exception unwinding frame (FS:[0]) is not translated.
/// `BSMultiBoundAABB::CreateClone` (Xbox PDB): allocates and constructs a
/// box, copies this one's members into it and returns it (null if the
/// allocation failed; `CopyMembers` still runs, as in the game).
pub fn bs_multi_bound_aabb_create_clone(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    clone_process: u32,
) -> Ptr<BSMultiBoundAABB> {
    let clone = new_aabb(e);
    bs_multi_bound_aabb_copy_members(e, this, clone, clone_process);
    clone
}

// Translated from 00c37f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::CopyMembers` (Xbox PDB): the shape's `CopyMembers`,
/// then the target's `SetCenter` (slot 0xb8) with this box's centre, then
/// the half extents copied word by word.
pub fn bs_multi_bound_aabb_copy_members(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    target: Ptr<BSMultiBoundAABB>,
    clone_process: u32,
) {
    e.call(SHAPE_COPY_MEMBERS, &args![this, target, clone_process]);
    let center = this.at(BSMultiBoundAABB::Center);
    e.vcall(target.addr(), SLOT_SET_CENTER, &args![center]);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    let target_half = target.at(BSMultiBoundAABB::HalfExtents);
    copy_words(e, half.addr(), target_half.addr(), 3);
}

// Translated from 00c37fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::LoadBinary` (Xbox PDB): the shape's `LoadBinary`, then
/// the centre and the half extents loaded as points.
pub fn bs_multi_bound_aabb_load_binary(e: &mut Engine, this: Ptr<BSMultiBoundAABB>, stream: Ptr) {
    e.call(SHAPE_LOAD_BINARY, &args![this, stream]);
    let center = this.at(BSMultiBoundAABB::Center);
    e.call(POINT_LOAD_BINARY, &args![center, stream]);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    e.call(POINT_LOAD_BINARY, &args![half, stream]);
}

// Translated from 00c38050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::SaveBinary` (Xbox PDB): the shape's save body, then
/// the centre and the half extents saved as points.
pub fn bs_multi_bound_aabb_save_binary(e: &mut Engine, this: Ptr<BSMultiBoundAABB>, stream: Ptr) {
    e.call(SHAPE_SAVE_BINARY, &args![this, stream]);
    let center = this.at(BSMultiBoundAABB::Center);
    e.call(POINT_SAVE_BINARY, &args![center, stream]);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    e.call(POINT_SAVE_BINARY, &args![half, stream]);
}

/// Whether the three floats at `a` and `b` are all equal (`FUCOMPP`: a NaN
/// is never equal).
fn points_equal(e: &Engine, a: u32, b: u32) -> bool {
    (0..3).all(|axis| e.mem.f32(a + 4 * axis) == e.mem.f32(b + 4 * axis))
}

// Translated from 00c38090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::IsEqual` (Xbox PDB): the shape's `IsEqual`, and when it
/// holds, both the centres and the half extents equal componentwise.
pub fn bs_multi_bound_aabb_is_equal(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    other: Ptr<BSMultiBoundAABB>,
) -> bool {
    if !e.call(SHAPE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let center = points_equal(
        e,
        other.at(BSMultiBoundAABB::Center).addr(),
        this.at(BSMultiBoundAABB::Center).addr(),
    );
    let half = points_equal(
        e,
        other.at(BSMultiBoundAABB::HalfExtents).addr(),
        this.at(BSMultiBoundAABB::HalfExtents).addr(),
    );
    center && half
}

// Translated from 00c381a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c381a0 - C++ exception unwinding frame (FS:[0]) is not translated.
/// Allocates and constructs a box (no arguments; the map has no name and
/// Ghidra's is a folded library name): a creator function. Returns the new
/// box, or null if the allocation failed.
pub fn fn_00c381a0(e: &mut Engine) -> Ptr<BSMultiBoundAABB> {
    new_aabb(e)
}

// Translated from 00c38220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::GetViewerStrings` (Xbox PDB): the base viewer strings,
/// then the string for the word at `01202820`, then the centre and half
/// extents strings, each added through `0096a610` with the address of a
/// local word holding the string.
pub fn bs_multi_bound_aabb_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    strings: Ptr,
) {
    e.call(SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    let type_name = e.mem.u32(TYPE_NAME_WORD);
    let first = e.call(GET_VIEWER_STRING, &args![type_name]).u32();
    add_viewer_string(e, strings, first);
    let center = this.at(BSMultiBoundAABB::Center);
    let second = e
        .call(POINT_GET_VIEWER_STRING, &args![center, CENTER_LABEL])
        .u32();
    add_viewer_string(e, strings, second);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    let third = e
        .call(POINT_GET_VIEWER_STRING, &args![half, HALF_EXTENTS_LABEL])
        .u32();
    add_viewer_string(e, strings, third);
}

/// Adds `string` to `strings` the way the exe does: through a local word
/// holding it.
fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(ADD_VIEWER_STRING, &args![strings, local]);
    });
}

// Translated from 00c382a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::QType` (Xbox PDB name of vtable slot 0x8c, which holds
/// this body): returns 1.
pub fn bs_multi_bound_aabb_q_type(_e: &mut Engine, _this: Ptr<BSMultiBoundAABB>) -> bool {
    true
}

// Translated from 00c382b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::CheckBound` for a bound sphere (Xbox PDB, slot 0x98;
/// `sphere` points to centre x, y, z and radius). Returns 2 when the sphere
/// is inside the box on every axis (distance minus (half extent minus
/// radius) not above zero), 1 when it overlaps it (distance minus (half
/// extent plus radius) not above zero), 0 otherwise. The two tests are
/// carried per axis and the later axes are skipped once neither holds; an
/// unordered value fails its test. The radius is copied through the x87
/// stack (quieted).
pub fn bs_multi_bound_aabb_check_bound(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    sphere: Ptr,
) -> u32 {
    let zero: f64 = e.global(ZERO);
    let radius = quiet(e.mem.f32(sphere.addr() + 0xc));
    let mut inside = true;
    let mut overlapping = true;
    for axis in 0..3u32 {
        if axis > 0 && !inside && !overlapping {
            break;
        }
        let point = e.mem.f32(sphere.addr() + 4 * axis);
        let center = e.mem.f32(this.addr() + 0xc + 4 * axis);
        let half = e.mem.f32(this.addr() + 0x18 + 4 * axis);
        let difference = (point as f64 - center as f64) as f32;
        let distance = absolute(e, difference) as f64;
        inside &= distance - (half as f64 - radius as f64) <= zero;
        overlapping &= distance - (half as f64 + radius as f64) <= zero;
    }
    if inside {
        2
    } else {
        overlapping as u32
    }
}

// Translated from 00c384f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::CheckBound` for another bound (`other` a
/// `BSMultiBound`; Xbox PDB, slot 0x94): asks its shape (word at +0xc) for
/// its type through slot 0x8c. Type 1 (the value this class's own `QType`
/// returns): the centre difference (`00439ef0`) magnitudes plus the other
/// half extents minus this box's must all be not above zero for 2, else 0
/// (an unordered value does not count as above). Type 2: 0 unless the
/// other's centre passes `FastPointWithin`, then the other shape's eight
/// vertices (`00c37a00`, named `BSMultiBoundOBB::GetVertices` in the map; the
/// type is not confirmed as the oriented box) must all pass: 2, else 0.
/// Type 3: the shape's `GetNiBound` (slot 0xa4) fills a bound sphere that is
/// checked with this box's slot 0x98. Any other type: 0.
pub fn bs_multi_bound_aabb_check_bound_ov2(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    other: Ptr,
) -> u32 {
    let shape = e.mem.u32(other.addr() + 0xc);
    match e.vcall(shape, SLOT_TYPE, &args![]).u32() {
        1 => {
            let zero: f64 = e.global(ZERO);
            let center = this.at(BSMultiBoundAABB::Center);
            let half = this.at(BSMultiBoundAABB::HalfExtents);
            e.with_stack(12, |e, offset| {
                e.call(POINT_SUBTRACT, &args![center, offset, shape + 0xc]);
                let mut excess = [0f32; 3];
                for axis in 0..3u32 {
                    let value = e.mem.f32(offset.addr() + 4 * axis);
                    let magnitude = absolute(e, value);
                    let other_half = e.mem.f32(shape + 0x18 + 4 * axis);
                    let own_half = e.mem.f32(half.addr() + 4 * axis);
                    excess[axis as usize] =
                        ((magnitude as f64 + other_half as f64) - own_half as f64) as f32;
                }
                if excess.iter().any(|value| *value as f64 > zero) {
                    0
                } else {
                    2
                }
            })
        }
        2 => {
            let center = e.mem.f32(shape + 0xc);
            let center_y = e.mem.f32(shape + 0x10);
            let center_z = e.mem.f32(shape + 0x14);
            if !bs_multi_bound_aabb_fast_point_within(e, this, center, center_y, center_z) {
                return 0;
            }
            e.with_stack(96, |e, vertices| {
                e.call(OBB_GET_VERTICES, &args![shape, vertices]);
                for vertex in 0..8u32 {
                    let at = vertices.addr() + 12 * vertex;
                    let (x, y, z) = (e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8));
                    if !bs_multi_bound_aabb_fast_point_within(e, this, x, y, z) {
                        return 0;
                    }
                }
                2
            })
        }
        3 => e.with_stack(16, |e, sphere| {
            e.vcall(shape, SLOT_GET_NI_BOUND, &args![sphere]);
            e.vcall(this.addr(), SLOT_CHECK_BOUND_SPHERE, &args![sphere])
                .u32()
        }),
        _ => 0,
    }
}

// Translated from 00c387f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::WithinFrustum` (Xbox PDB): `frustum` holds six planes
/// of 16 bytes and a plane mask word at +0x60. The eight vertices
/// ([`bs_multi_bound_aabb_get_vertices`]) are tested against every plane of
/// the mask with `0049da80`; the box is outside (false) as soon as all
/// tested vertices of a plane test 2 ("outside"). When the third half extent
/// is exactly zero only the odd vertices are tested. True otherwise.
pub fn bs_multi_bound_aabb_within_frustum(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    frustum: Ptr,
) -> bool {
    let zero: f64 = e.global(ZERO);
    let flat = e.mem.f32(this.addr() + 0x20) as f64 == zero;
    e.with_stack(96, |e, vertices| {
        bs_multi_bound_aabb_get_vertices(e, this, vertices);
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

// Translated from 00c38920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::CompletelyWithinFrustum` (Xbox PDB): false as soon as
/// any vertex tests 2 ("outside") against any plane of the frustum's mask
/// (word at +0x60), true otherwise.
pub fn bs_multi_bound_aabb_completely_within_frustum(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    frustum: Ptr,
) -> bool {
    e.with_stack(96, |e, vertices| {
        bs_multi_bound_aabb_get_vertices(e, this, vertices);
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

// Translated from 00c389e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::GetNiBound` (Xbox PDB name of vtable slot 0xa4, which
/// holds this body): fills the bound at `out` with the centre and, as the
/// radius, the length of the half extents (`00457990`, returned in `ST0`
/// and stored as a float).
pub fn bs_multi_bound_aabb_get_ni_bound(e: &mut Engine, this: Ptr<BSMultiBoundAABB>, out: Ptr) {
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    let radius = e.call(POINT_LENGTH, &args![half]).f64() as f32;
    let center = this.at(BSMultiBoundAABB::Center);
    copy_words(e, center.addr(), out.addr(), 3);
    e.mem.set_f32(out.addr() + 0xc, quiet(radius));
}

// Translated from 00c38a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::QPointWithin` (Xbox PDB): whether the point at `point`
/// is within the box, with the tolerance `01016408` on every axis.
pub fn bs_multi_bound_aabb_q_point_within(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    point: Ptr,
) -> bool {
    point_within(e, this, point)
}

// Translated from 00c38b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c38b00 - the stack-cookie check (00ec408c) at the end is not translated.
/// `BSMultiBoundAABB::MakeDebugGeom` (Xbox PDB, slot 0xa8): fills two
/// debug geometries from the box's eight vertices. `lines` gets its position
/// (+0x58) set to the box centre and 24 line vertices (each box edge's two
/// vertices relative to the centre, `00439ef0`) written into its data
/// (the data pointer is at +0xb8: positions at +0x20, colours at +0x28,
/// flags at +0x40), coloured with the four words at `011a9be0`, with the
/// flag bytes 1, 0 alternating. `triangles` gets the same position, 36
/// vertices (the box's 12 triangles) coloured with the four given colour
/// words, and its triangle list (`NiTriShapeData::GetTriList`) filled with
/// the identity indices 0..35.
#[allow(clippy::too_many_arguments)]
pub fn bs_multi_bound_aabb_make_debug_geom(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    lines: Ptr,
    triangles: Ptr,
    color_0: u32,
    color_1: u32,
    color_2: u32,
    color_3: u32,
) {
    let center = this.at(BSMultiBoundAABB::Center);
    e.with_stack(96, |e, vertices| {
        bs_multi_bound_aabb_get_vertices(e, this, vertices);
        let line_color = [
            e.mem.u32(LINE_COLOR),
            e.mem.u32(LINE_COLOR + 4),
            e.mem.u32(LINE_COLOR + 8),
            e.mem.u32(LINE_COLOR + 12),
        ];
        let triangle_color = [color_0, color_1, color_2, color_3];
        let line_data = e.mem.u32(lines.addr() + 0xb8);
        let positions = e.mem.u32(line_data + 0x20);
        let colors = e.mem.u32(line_data + 0x28);
        let flags = e.mem.u32(line_data + 0x40);
        copy_words(e, center.addr(), lines.addr() + 0x58, 3);
        e.with_stack(12, |e, offset| {
            for edge in 0..12u32 {
                for end in 0..2u32 {
                    let vertex = EDGE_VERTICES[(edge * 2 + end) as usize] as u32;
                    let relative = e
                        .call(
                            POINT_SUBTRACT,
                            &args![vertices.byte_add(12 * vertex), offset, center],
                        )
                        .u32();
                    let slot = edge * 2 + end;
                    copy_words(e, relative, positions + 12 * slot, 3);
                    for (word, value) in line_color.iter().enumerate() {
                        e.mem.set_u32(colors + 16 * slot + 4 * word as u32, *value);
                    }
                    if end == 0 {
                        e.mem.set_u8(flags + slot, 1);
                    } else {
                        e.mem.set_u8(flags + slot, 0);
                    }
                }
            }
            let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
            let positions = e.mem.u32(triangle_data + 0x20);
            let colors = e.mem.u32(triangle_data + 0x28);
            let list = e.call(GET_TRI_LIST, &args![triangle_data]).u32();
            copy_words(e, center.addr(), triangles.addr() + 0x58, 3);
            for triangle in 0..12u32 {
                for corner in 0..3u32 {
                    let slot = triangle * 3 + corner;
                    let vertex = TRIANGLE_VERTICES[slot as usize] as u32;
                    let relative = e
                        .call(
                            POINT_SUBTRACT,
                            &args![vertices.byte_add(12 * vertex), offset, center],
                        )
                        .u32();
                    copy_words(e, relative, positions + 12 * slot, 3);
                    for (word, value) in triangle_color.iter().enumerate() {
                        e.mem.set_u32(colors + 16 * slot + 4 * word as u32, *value);
                    }
                    e.mem.set_u16(list + 2 * slot, slot as u16);
                }
            }
        });
    });
}

// Translated from 00c39270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::GetVertices` (Xbox PDB): writes the eight corners of
/// the box (eight points of 12 bytes) at `out`. The three axis points are
/// the unit axes (`011a946c`, `011a9478`, `011a9484`) scaled by the half
/// extents (`0045bb20`); corner `n` is the centre plus or minus those three
/// in turn (`00439e90` adds, `00439ef0` subtracts): corner 0 adds all three,
/// and bit 2 of `n` makes the first one a subtraction, bit 1 the second and
/// bit 0 the third.
pub fn bs_multi_bound_aabb_get_vertices(e: &mut Engine, this: Ptr<BSMultiBoundAABB>, out: Ptr) {
    let center = this.at(BSMultiBoundAABB::Center);
    let half = this.at(BSMultiBoundAABB::HalfExtents);
    // The three axis points, then three scratch points for each corner.
    e.with_stack(12 * (3 + 24), |e, scratch| {
        let axis_points = [scratch.addr(), scratch.addr() + 12, scratch.addr() + 24];
        for axis in 0..3u32 {
            let extent = quiet(e.mem.f32(half.addr() + 4 * axis));
            e.call(
                POINT_SCALE,
                &args![
                    Ptr::<()>::new(UNIT_AXES[axis as usize]),
                    Ptr::<()>::new(axis_points[axis as usize]),
                    extent
                ],
            );
        }
        for corner in 0..8u32 {
            let mut current = center.addr();
            for axis in 0..3u32 {
                let operation = if corner >> (2 - axis) & 1 == 0 {
                    POINT_ADD
                } else {
                    POINT_SUBTRACT
                };
                let temporary = scratch.addr() + 36 + 12 * (corner * 3 + axis);
                current = e
                    .call(
                        operation,
                        &args![
                            Ptr::<()>::new(current),
                            Ptr::<()>::new(temporary),
                            Ptr::<()>::new(axis_points[axis as usize])
                        ],
                    )
                    .u32();
            }
            copy_words(e, current, out.addr() + 12 * corner, 3);
        }
    });
}

// Translated from 00c39560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiBoundAABB::FastPointWithin` (Xbox PDB): [`bs_multi_bound_aabb_q_point_within`]
/// with the point passed by value (three floats).
pub fn bs_multi_bound_aabb_fast_point_within(
    e: &mut Engine,
    this: Ptr<BSMultiBoundAABB>,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    e.with_stack(12, |e, point| {
        e.mem.set_f32(point.addr(), x);
        e.mem.set_f32(point.addr() + 4, y);
        e.mem.set_f32(point.addr() + 8, z);
        point_within(e, this, point)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c37ec0,
            bs_multi_bound_aabb_bs_multi_bound_aabb(Ptr<BSMultiBoundAABB>) -> Ptr<BSMultiBoundAABB>
        ),
        entry!(
            0x00c37ee0,
            bs_multi_bound_aabb_create_clone(Ptr<BSMultiBoundAABB>, u32) -> Ptr<BSMultiBoundAABB>
        ),
        entry!(
            0x00c37f70,
            bs_multi_bound_aabb_copy_members(Ptr<BSMultiBoundAABB>, Ptr<BSMultiBoundAABB>, u32)
        ),
        entry!(
            0x00c37fc0,
            bs_multi_bound_aabb_load_binary(Ptr<BSMultiBoundAABB>, Ptr)
        ),
        entry!(
            0x00c38050,
            bs_multi_bound_aabb_save_binary(Ptr<BSMultiBoundAABB>, Ptr)
        ),
        entry!(
            0x00c38090,
            bs_multi_bound_aabb_is_equal(Ptr<BSMultiBoundAABB>, Ptr<BSMultiBoundAABB>) -> bool
        ),
        entry!(0x00c381a0, fn_00c381a0() -> Ptr<BSMultiBoundAABB>),
        entry!(
            0x00c38220,
            bs_multi_bound_aabb_get_viewer_strings(Ptr<BSMultiBoundAABB>, Ptr)
        ),
        entry!(
            0x00c382a0,
            bs_multi_bound_aabb_q_type(Ptr<BSMultiBoundAABB>) -> bool
        ),
        entry!(
            0x00c382b0,
            bs_multi_bound_aabb_check_bound(Ptr<BSMultiBoundAABB>, Ptr) -> u32
        ),
        entry!(
            0x00c384f0,
            bs_multi_bound_aabb_check_bound_ov2(Ptr<BSMultiBoundAABB>, Ptr) -> u32
        ),
        entry!(
            0x00c387f0,
            bs_multi_bound_aabb_within_frustum(Ptr<BSMultiBoundAABB>, Ptr) -> bool
        ),
        entry!(
            0x00c38920,
            bs_multi_bound_aabb_completely_within_frustum(Ptr<BSMultiBoundAABB>, Ptr) -> bool
        ),
        entry!(
            0x00c389e0,
            bs_multi_bound_aabb_get_ni_bound(Ptr<BSMultiBoundAABB>, Ptr)
        ),
        entry!(
            0x00c38a20,
            bs_multi_bound_aabb_q_point_within(Ptr<BSMultiBoundAABB>, Ptr) -> bool
        ),
        entry!(
            0x00c38b00,
            bs_multi_bound_aabb_make_debug_geom(
                Ptr<BSMultiBoundAABB>,
                Ptr,
                Ptr,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x00c39270,
            bs_multi_bound_aabb_get_vertices(Ptr<BSMultiBoundAABB>, Ptr)
        ),
        entry!(
            0x00c39560,
            bs_multi_bound_aabb_fast_point_within(Ptr<BSMultiBoundAABB>, f32, f32, f32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A fake vtable the test objects use, and the doubles its slots run.
    const TEST_VTABLE: u32 = 0x7000_0000;
    const TYPE_DOUBLE: u32 = 0x7000_1000;
    const NI_BOUND_DOUBLE: u32 = 0x7000_1010;
    const CHECK_SPHERE_DOUBLE: u32 = 0x7000_1020;
    const SET_CENTER_DOUBLE: u32 = 0x7000_1030;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn word(a: &[u32], index: usize) -> f32 {
        f32::from_bits(a[index])
    }

    /// An engine with the constants the code compares against, the unit axes,
    /// and doubles for the point arithmetic, `fabs` and the allocator.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.set_global(ZERO, 0.0f64);
        e.map(TOLERANCE, 8);
        e.set_global(TOLERANCE, 0.01f64);
        e.map(UNIT_AXES[0], 0x24);
        e.mem.set_f32(UNIT_AXES[0], 1.0);
        e.mem.set_f32(UNIT_AXES[1] + 4, 1.0);
        e.mem.set_f32(UNIT_AXES[2] + 8, 1.0);
        e.register(FABS, |_, a| Ret {
            st0: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32).abs(),
            ..Ret::default()
        });
        e.register(POINT_ADD, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) + e.mem.f32(a[2] + 4 * axis);
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            ret(a[1])
        });
        e.register(POINT_SUBTRACT, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) - e.mem.f32(a[2] + 4 * axis);
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            ret(a[1])
        });
        e.register(POINT_SCALE, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) * word(a, 2);
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            ret(a[1])
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(SHAPE_CONSTRUCT, |_, _| ret(0));
        e.register(SHAPE_COPY_MEMBERS, |_, _| ret(0));
        e
    }

    /// A box with the given centre and half extents and the test vtable.
    fn aabb(e: &mut Engine, center: [f32; 3], half: [f32; 3]) -> Ptr<BSMultiBoundAABB> {
        let this: Ptr<BSMultiBoundAABB> = e.new_object();
        e.mem.set_u32(this.addr(), TEST_VTABLE);
        for axis in 0..3u32 {
            e.mem
                .set_f32(this.addr() + 0xc + 4 * axis, center[axis as usize]);
            e.mem
                .set_f32(this.addr() + 0x18 + 4 * axis, half[axis as usize]);
        }
        this
    }

    fn sphere(e: &mut Engine, values: [f32; 4]) -> Ptr {
        let at = e.mem.alloc(16);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        Ptr::new(at)
    }

    fn calls(e: &mut Engine) -> Vec<u32> {
        e.call_log.take().unwrap().iter().map(|c| c.0).collect()
    }

    /// A vtable (at `TEST_VTABLE`) with the given slots.
    fn test_vtable(e: &mut Engine, slots: &[(u32, u32)]) {
        let mut table = vec![0u32; 0x30];
        for (slot, target) in slots {
            table[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(TEST_VTABLE, &table);
    }

    #[test]
    fn constructor_builds_the_shape_and_sets_the_vtable() {
        let mut e = engine();
        let this: Ptr<BSMultiBoundAABB> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x00c37ec0, &args![this]).ptr::<BSMultiBoundAABB>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), BS_MULTI_BOUND_AABB_VTABLE);
        assert_eq!(calls(&mut e), vec![0x00c37ec0, SHAPE_CONSTRUCT]);
    }

    #[test]
    fn factory_allocates_and_constructs() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let made = e.call(0x00c381a0, &args![]).ptr::<BSMultiBoundAABB>();
        assert!(e.mem.block_size(made.addr()).is_some());
        assert_eq!(e.mem.u32(made.addr()), BS_MULTI_BOUND_AABB_VTABLE);
        assert_eq!(
            calls(&mut e),
            vec![0x00c381a0, OPERATOR_NEW, SHAPE_CONSTRUCT]
        );
    }

    #[test]
    fn clone_allocates_constructs_and_copies_the_members() {
        let mut e = engine();
        let centers = Rc::new(RefCell::new(vec![]));
        let log = centers.clone();
        e.register_double(SET_CENTER_DOUBLE, move |_, a| {
            log.borrow_mut().push((a[0], a[1]));
            ret(0)
        });
        // The clone gets the real vtable from its constructor.
        let mut table = vec![0u32; 0x30];
        table[(SLOT_SET_CENTER / 4) as usize] = SET_CENTER_DOUBLE;
        e.put_vtable(BS_MULTI_BOUND_AABB_VTABLE, &table);
        let this = aabb(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c37ee0, &args![this, 0x55u32])
            .ptr::<BSMultiBoundAABB>();
        assert_ne!(clone, this);
        assert_eq!(e.mem.u32(clone.addr()), BS_MULTI_BOUND_AABB_VTABLE);
        assert_eq!(
            calls(&mut e),
            vec![
                0x00c37ee0,
                OPERATOR_NEW,
                SHAPE_CONSTRUCT,
                SHAPE_COPY_MEMBERS,
                SET_CENTER_DOUBLE
            ]
        );
        assert_eq!(centers.borrow()[0], (clone.addr(), this.addr() + 0xc));
        assert_eq!(e.mem.f32(clone.addr() + 0x18), 4.0);
        assert_eq!(e.mem.f32(clone.addr() + 0x1c), 5.0);
        assert_eq!(e.mem.f32(clone.addr() + 0x20), 6.0);
    }

    #[test]
    fn copy_members_runs_the_shape_copy_then_set_center_then_copies_the_extents() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(SHAPE_COPY_MEMBERS, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0)
        });
        let centers = Rc::new(RefCell::new(vec![]));
        let log = centers.clone();
        e.register_double(SET_CENTER_DOUBLE, move |e, a| {
            log.borrow_mut()
                .push((a[0], e.mem.f32(a[1]), e.mem.f32(a[1] + 8)));
            ret(0)
        });
        let this = aabb(&mut e, [1.5, 0.0, -2.0], [7.0, 8.0, 9.0]);
        let target = aabb(&mut e, [0.0; 3], [0.0; 3]);
        test_vtable(&mut e, &[(SLOT_SET_CENTER, SET_CENTER_DOUBLE)]);
        e.call(0x00c37f70, &args![this, target, 0x77u32]);
        assert_eq!(*seen.borrow(), vec![vec![this.addr(), target.addr(), 0x77]]);
        assert_eq!(*centers.borrow(), vec![(target.addr(), 1.5, -2.0)]);
        assert_eq!(e.mem.f32(target.addr() + 0x18), 7.0);
        assert_eq!(e.mem.f32(target.addr() + 0x1c), 8.0);
        assert_eq!(e.mem.f32(target.addr() + 0x20), 9.0);
        // The centre is the target's own `SetCenter` job: not copied here.
        assert_eq!(e.mem.f32(target.addr() + 0xc), 0.0);
    }

    #[test]
    fn load_and_save_binary_handle_the_base_then_both_points() {
        let mut e = engine();
        for address in [
            SHAPE_LOAD_BINARY,
            SHAPE_SAVE_BINARY,
            POINT_LOAD_BINARY,
            POINT_SAVE_BINARY,
        ] {
            e.register(address, |_, _| ret(0));
        }
        let this = aabb(&mut e, [0.0; 3], [0.0; 3]);
        e.call_log = Some(vec![]);
        e.call(0x00c37fc0, &args![this, Ptr::<()>::new(0x99)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00c37fc0, vec![this.addr(), 0x99]),
                (SHAPE_LOAD_BINARY, vec![this.addr(), 0x99]),
                (POINT_LOAD_BINARY, vec![this.addr() + 0xc, 0x99]),
                (POINT_LOAD_BINARY, vec![this.addr() + 0x18, 0x99]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00c38050, &args![this, Ptr::<()>::new(0x98)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x00c38050, vec![this.addr(), 0x98]),
                (SHAPE_SAVE_BINARY, vec![this.addr(), 0x98]),
                (POINT_SAVE_BINARY, vec![this.addr() + 0xc, 0x98]),
                (POINT_SAVE_BINARY, vec![this.addr() + 0x18, 0x98]),
            ]
        );
    }

    #[test]
    fn is_equal_needs_the_base_and_every_component() {
        let mut e = engine();
        let base = Rc::new(RefCell::new(true));
        let answer = base.clone();
        e.register_double(SHAPE_IS_EQUAL, move |_, _| ret(*answer.borrow() as u32));
        let a = aabb(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        let b = aabb(&mut e, [1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert!(e.call(0x00c38090, &args![a, b]).bool());
        // Any one component differing breaks it.
        for offset in [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
            let value = e.mem.f32(b.addr() + offset);
            e.mem.set_f32(b.addr() + offset, value + 1.0);
            assert!(!e.call(0x00c38090, &args![a, b]).bool());
            e.mem.set_f32(b.addr() + offset, value);
        }
        // A NaN is never equal, even to itself.
        e.mem.set_f32(a.addr() + 0x10, f32::NAN);
        e.mem.set_f32(b.addr() + 0x10, f32::NAN);
        assert!(!e.call(0x00c38090, &args![a, b]).bool());
        e.mem.set_f32(a.addr() + 0x10, 2.0);
        e.mem.set_f32(b.addr() + 0x10, 2.0);
        // The shape's answer decides first.
        *base.borrow_mut() = false;
        assert!(!e.call(0x00c38090, &args![a, b]).bool());
    }

    #[test]
    fn viewer_strings_are_added_in_order_through_local_words() {
        let mut e = engine();
        e.map(TYPE_NAME_WORD, 4);
        e.mem.set_u32(TYPE_NAME_WORD, 0xaaaa);
        e.register(SHAPE_GET_VIEWER_STRINGS, |_, _| ret(0));
        e.register(GET_VIEWER_STRING, |_, a| ret(a[0] + 1));
        e.register(POINT_GET_VIEWER_STRING, |_, a| ret(a[1]));
        let added = Rc::new(RefCell::new(vec![]));
        let log = added.clone();
        e.register_double(ADD_VIEWER_STRING, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1])));
            ret(0)
        });
        let this = aabb(&mut e, [0.0; 3], [0.0; 3]);
        e.call_log = Some(vec![]);
        e.call(0x00c38220, &args![this, Ptr::<()>::new(0x500)]);
        assert_eq!(
            calls(&mut e),
            vec![
                0x00c38220,
                SHAPE_GET_VIEWER_STRINGS,
                GET_VIEWER_STRING,
                ADD_VIEWER_STRING,
                POINT_GET_VIEWER_STRING,
                ADD_VIEWER_STRING,
                POINT_GET_VIEWER_STRING,
                ADD_VIEWER_STRING,
            ]
        );
        assert_eq!(
            *added.borrow(),
            vec![
                (0x500, 0xaaab),
                (0x500, CENTER_LABEL),
                (0x500, HALF_EXTENTS_LABEL)
            ]
        );
    }

    #[test]
    fn type_is_one() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [0.0; 3]);
        assert_eq!(e.call(0x00c382a0, &args![this]).u32(), 1);
    }

    #[test]
    fn sphere_check_gives_inside_overlapping_or_outside() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 1.0, 1.0]);
        let inside = sphere(&mut e, [0.0, 0.0, 0.0, 0.5]);
        assert_eq!(e.call(0x00c382b0, &args![this, inside]).u32(), 2);
        // Poking out on x only: overlapping, not inside.
        let overlapping = sphere(&mut e, [1.2, 0.0, 0.0, 0.5]);
        assert_eq!(e.call(0x00c382b0, &args![this, overlapping]).u32(), 1);
        // Far away on x: fails both tests there and skips the other axes.
        let outside = sphere(&mut e, [5.0, 0.0, 0.0, 0.5]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c382b0, &args![this, outside]).u32(), 0);
        assert_eq!(calls(&mut e).iter().filter(|a| **a == FABS).count(), 1);
        // Outside only on z: all three axes are measured.
        let above = sphere(&mut e, [0.0, 0.0, 3.0, 0.5]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c382b0, &args![this, above]).u32(), 0);
        assert_eq!(calls(&mut e).iter().filter(|a| **a == FABS).count(), 3);
        // An unordered distance fails both tests ("not above zero" needs an
        // ordered comparison here).
        let unordered = sphere(&mut e, [f32::NAN, 0.0, 0.0, 0.5]);
        assert_eq!(e.call(0x00c382b0, &args![this, unordered]).u32(), 0);
    }

    /// A shape whose `QType` slot answers `kind`, wrapped in a bound whose
    /// shape pointer (+0xc) is it. The vtable is the shared test one.
    fn bound_of_kind(
        e: &mut Engine,
        center: [f32; 3],
        half: [f32; 3],
        kind: u32,
    ) -> (Ptr<BSMultiBoundAABB>, Ptr) {
        let shape = aabb(e, center, half);
        e.register_double(TYPE_DOUBLE, move |_, _| ret(kind));
        test_vtable(
            e,
            &[
                (SLOT_TYPE, TYPE_DOUBLE),
                (SLOT_GET_NI_BOUND, NI_BOUND_DOUBLE),
                (SLOT_CHECK_BOUND_SPHERE, CHECK_SPHERE_DOUBLE),
            ],
        );
        let bound = e.mem.alloc(0x10);
        e.mem.set_u32(bound + 0xc, shape.addr());
        (shape, Ptr::new(bound))
    }

    #[test]
    fn bound_check_type_one_compares_centre_distance_and_extents() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 1.0, 1.0]);
        let (shape, bound) = bound_of_kind(&mut e, [0.5, 0.0, 0.0], [0.25, 0.25, 0.25], 1);
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 2);
        // Sticking out on x.
        e.mem.set_f32(shape.addr() + 0xc, 2.0);
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 0);
        // Sticking out on z only.
        e.mem.set_f32(shape.addr() + 0xc, 0.0);
        e.mem.set_f32(shape.addr() + 0x14, 1.5);
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 0);
        // A NaN excess is not "above zero": it counts as inside.
        e.mem.set_f32(shape.addr() + 0x14, f32::NAN);
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 2);
    }

    #[test]
    fn bound_check_type_two_tests_the_centre_then_every_vertex() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 1.0, 1.0]);
        let (shape, bound) = bound_of_kind(&mut e, [0.0, 0.0, 0.0], [0.0; 3], 2);
        // The vertices the shape reports: eight equal points.
        let fill = Rc::new(RefCell::new(0.5f32));
        let value = fill.clone();
        e.register_double(OBB_GET_VERTICES, move |e, a| {
            assert_eq!(a[0], shape.addr());
            for corner in 0..8 {
                for axis in 0..3 {
                    e.mem
                        .set_f32(a[1] + 12 * corner + 4 * axis, *value.borrow());
                }
            }
            ret(0)
        });
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 2);
        // Vertices far out: 0.
        *fill.borrow_mut() = 4.0;
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 0);
        // A centre out of the box ends it before the vertices are asked for.
        e.mem.set_f32(shape.addr() + 0xc, 9.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c384f0, &args![this, bound]).u32(), 0);
        assert!(!calls(&mut e).contains(&OBB_GET_VERTICES));
    }

    #[test]
    fn bound_check_type_three_asks_the_shape_for_a_sphere() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 1.0, 1.0]);
        let (_, bound) = bound_of_kind(&mut e, [0.0; 3], [0.0; 3], 3);
        e.register_double(NI_BOUND_DOUBLE, |e, a| {
            for (index, value) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * index as u32, *value);
            }
            ret(0)
        });
        e.register_double(CHECK_SPHERE_DOUBLE, |e, a| {
            assert_eq!(e.mem.f32(a[1] + 12), 4.0);
            ret(a[0] ^ 0x1234)
        });
        assert_eq!(
            e.call(0x00c384f0, &args![this, bound]).u32(),
            this.addr() ^ 0x1234
        );
        // An unknown type is 0.
        let (_, other) = bound_of_kind(&mut e, [0.0; 3], [0.0; 3], 9);
        assert_eq!(e.call(0x00c384f0, &args![this, other]).u32(), 0);
    }

    /// A frustum: six 16-byte planes and the mask word at +0x60.
    fn frustum(e: &mut Engine, mask: u32) -> Ptr {
        let at = e.mem.alloc(0x68);
        e.mem.set_u32(at + 0x60, mask);
        Ptr::new(at)
    }

    /// Runs the frustum function `entry` on a box at the origin with `half`
    /// extents and the plane mask `mask`. The plane test answers 2
    /// ("outside") when `outside(plane, vertex)` holds, where `vertex` counts
    /// from the first vertex asked about. Returns the result and every
    /// (plane, vertex) pair asked, in order.
    fn frustum_case(
        mask: u32,
        half: [f32; 3],
        outside: impl Fn(u32, u32) -> bool + 'static,
        entry: u32,
    ) -> (bool, Vec<(u32, u32)>) {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], half);
        let frustum = frustum(&mut e, mask);
        let asked = Rc::new(RefCell::new(vec![]));
        let log = asked.clone();
        let mut first = None;
        e.register_double(PLANE_SIDE_OF_POINT, move |_, a| {
            let plane = (a[0] - frustum.addr()) / 16;
            let base = *first.get_or_insert(a[1]);
            let vertex = (a[1] - base) / 12;
            log.borrow_mut().push((plane, vertex));
            ret(if outside(plane, vertex) { 2 } else { 1 })
        });
        let result = e.call(entry, &args![this, frustum]).bool();
        let list = asked.borrow().clone();
        (result, list)
    }

    #[test]
    fn within_frustum_is_false_only_when_one_plane_has_every_vertex_outside() {
        // Mask 0: nothing is tested.
        let (result, asked) = frustum_case(0, [1.0; 3], |_, _| true, 0x00c387f0);
        assert!(result);
        assert!(asked.is_empty());
        // Plane 2 only, every vertex outside: not within.
        let (result, asked) = frustum_case(0b100, [1.0; 3], |_, _| true, 0x00c387f0);
        assert!(!result);
        assert_eq!(asked, (0..8).map(|v| (2, v)).collect::<Vec<_>>());
        // One vertex inside the plane keeps it.
        let (result, _) = frustum_case(0b100, [1.0; 3], |_, v| v != 5, 0x00c387f0);
        assert!(result);
        // The first plane that fails ends the test: plane 0 inside, plane 1
        // all outside, plane 5 never asked.
        let (result, asked) = frustum_case(0b100011, [1.0; 3], |plane, _| plane == 1, 0x00c387f0);
        assert!(!result);
        assert!(asked.iter().all(|(plane, _)| *plane <= 1));
    }

    #[test]
    fn within_frustum_tests_only_odd_vertices_of_a_flat_box() {
        // The first vertex asked is vertex 1, so the others are 2, 4, 6 on.
        let (result, asked) = frustum_case(0b1, [1.0, 1.0, 0.0], |_, _| true, 0x00c387f0);
        assert!(!result);
        assert_eq!(asked, vec![(0, 0), (0, 2), (0, 4), (0, 6)]);
        // A NaN third extent is not zero: every vertex is tested.
        let (_, asked) = frustum_case(0b1, [1.0, 1.0, f32::NAN], |_, _| true, 0x00c387f0);
        assert_eq!(asked.len(), 8);
    }

    #[test]
    fn completely_within_frustum_fails_on_the_first_vertex_outside() {
        let (result, asked) = frustum_case(0b11, [1.0; 3], |_, _| false, 0x00c38920);
        assert!(result);
        assert_eq!(asked.len(), 16);
        let (result, asked) = frustum_case(
            0b11,
            [1.0; 3],
            |plane, vertex| plane == 0 && vertex == 3,
            0x00c38920,
        );
        assert!(!result);
        assert_eq!(asked.last(), Some(&(0, 3)));
        assert_eq!(asked.len(), 4);
        // Mask 0: nothing tested, true.
        let (result, asked) = frustum_case(0, [1.0; 3], |_, _| true, 0x00c38920);
        assert!(result);
        assert!(asked.is_empty());
    }

    #[test]
    fn eight_corners_are_the_centre_plus_and_minus_the_extents() {
        let mut e = engine();
        let this = aabb(&mut e, [10.0, 20.0, 30.0], [1.0, 2.0, 3.0]);
        let out = e.mem.alloc(96);
        e.call(0x00c39270, &args![this, Ptr::<()>::new(out)]);
        let found: Vec<[f32; 3]> = (0..8)
            .map(|corner| {
                [
                    e.mem.f32(out + 12 * corner),
                    e.mem.f32(out + 12 * corner + 4),
                    e.mem.f32(out + 12 * corner + 8),
                ]
            })
            .collect();
        assert_eq!(
            found,
            vec![
                [11.0, 22.0, 33.0],
                [11.0, 22.0, 27.0],
                [11.0, 18.0, 33.0],
                [11.0, 18.0, 27.0],
                [9.0, 22.0, 33.0],
                [9.0, 22.0, 27.0],
                [9.0, 18.0, 33.0],
                [9.0, 18.0, 27.0],
            ]
        );
    }

    #[test]
    fn corners_call_the_scale_then_add_and_subtract_in_order() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 2.0, 3.0]);
        let out = e.mem.alloc(96);
        e.call_log = Some(vec![]);
        e.call(0x00c39270, &args![this, Ptr::<()>::new(out)]);
        let log = calls(&mut e);
        assert_eq!(&log[1..4], &[POINT_SCALE, POINT_SCALE, POINT_SCALE]);
        let operations = &log[4..];
        assert_eq!(operations.len(), 24);
        // Corner 0: three additions; corner 7: three subtractions; corner 5
        // (binary 101): subtract, add, subtract.
        assert_eq!(&operations[0..3], &[POINT_ADD; 3]);
        assert_eq!(&operations[21..24], &[POINT_SUBTRACT; 3]);
        assert_eq!(
            &operations[15..18],
            &[POINT_SUBTRACT, POINT_ADD, POINT_SUBTRACT]
        );
    }

    #[test]
    fn bound_is_the_centre_and_the_length_of_the_extents() {
        let mut e = engine();
        e.register(POINT_LENGTH, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                st0: 5.5,
                ..Ret::default()
            }
        });
        let this = aabb(&mut e, [1.0, 2.0, 3.0], [3.0, 4.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00c389e0, &args![this, Ptr::<()>::new(out)]);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        assert_eq!(e.mem.f32(out + 12), 5.5);
        // The centre words are copied as they are, even signaling NaNs.
        e.mem.set_u32(this.addr() + 0xc, 0x7f80_0001);
        e.call(0x00c389e0, &args![this, Ptr::<()>::new(out)]);
        assert_eq!(e.mem.u32(out), 0x7f80_0001);
    }

    #[test]
    fn point_within_allows_the_tolerance_and_passes_unordered_values() {
        let mut e = engine();
        let this = aabb(&mut e, [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
        let point = |e: &mut Engine, x: f32, y: f32, z: f32| {
            let at = e.mem.alloc(12);
            e.mem.set_f32(at, x);
            e.mem.set_f32(at + 4, y);
            e.mem.set_f32(at + 8, z);
            Ptr::<()>::new(at)
        };
        let inside = point(&mut e, 1.5, 1.5, 1.5);
        assert!(e.call(0x00c38a20, &args![this, inside]).bool());
        let on_the_face = point(&mut e, 2.005, 1.0, 1.0);
        assert!(e.call(0x00c38a20, &args![this, on_the_face]).bool());
        let outside_x = point(&mut e, 2.02, 1.0, 1.0);
        assert!(!e.call(0x00c38a20, &args![this, outside_x]).bool());
        let outside_z = point(&mut e, 1.0, 1.0, 3.5);
        assert!(!e.call(0x00c38a20, &args![this, outside_z]).bool());
        // The "greater than the tolerance" test is false for a NaN.
        let unordered = point(&mut e, f32::NAN, 1.0, 1.0);
        assert!(e.call(0x00c38a20, &args![this, unordered]).bool());
    }

    #[test]
    fn fast_point_within_takes_the_point_by_value() {
        let mut e = engine();
        let this = aabb(&mut e, [0.0; 3], [1.0, 1.0, 1.0]);
        assert!(e
            .call(0x00c39560, &args![this, 0.5f32, -0.5f32, 1.005f32])
            .bool());
        assert!(!e
            .call(0x00c39560, &args![this, 0.5f32, -0.5f32, 1.02f32])
            .bool());
        assert!(!e
            .call(0x00c39560, &args![this, -3.0f32, 0.0f32, 0.0f32])
            .bool());
    }

    /// A debug geometry: its data pointer at +0xb8, with positions at +0x20,
    /// colours at +0x28 and flags at +0x40, sized for `vertices`.
    fn geometry(e: &mut Engine, vertices: u32) -> (Ptr, u32, u32, u32) {
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x50);
        let positions = e.mem.alloc(12 * vertices);
        let colors = e.mem.alloc(16 * vertices);
        let flags = e.mem.alloc(vertices);
        e.mem.set_u32(geometry + 0xb8, data);
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x28, colors);
        e.mem.set_u32(data + 0x40, flags);
        (Ptr::new(geometry), positions, colors, flags)
    }

    #[test]
    fn debug_geometry_fills_lines_and_triangles_relative_to_the_centre() {
        let mut e = engine();
        let this = aabb(&mut e, [10.0, 20.0, 30.0], [1.0, 2.0, 3.0]);
        e.map(LINE_COLOR, 16);
        for word in 0..4 {
            e.mem.set_u32(LINE_COLOR + 4 * word, 0xc0 + word);
        }
        let (lines, line_positions, line_colors, line_flags) = geometry(&mut e, 24);
        let (triangles, triangle_positions, triangle_colors, _) = geometry(&mut e, 36);
        let list = e.mem.alloc(72);
        let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
        e.register_double(GET_TRI_LIST, move |_, a| {
            assert_eq!(a[0], triangle_data);
            ret(list)
        });
        e.call(
            0x00c38b00,
            &args![this, lines, triangles, 1u32, 2u32, 3u32, 4u32],
        );
        // The geometries sit at the centre.
        for geometry in [lines, triangles] {
            assert_eq!(e.mem.f32(geometry.addr() + 0x58), 10.0);
            assert_eq!(e.mem.f32(geometry.addr() + 0x5c), 20.0);
            assert_eq!(e.mem.f32(geometry.addr() + 0x60), 30.0);
        }
        let point = |e: &Engine, at: u32| [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)];
        // Edge 0 joins corners 0 and 1; edge 1 corners 0 and 2.
        assert_eq!(point(&e, line_positions), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, line_positions + 12), [1.0, 2.0, -3.0]);
        assert_eq!(point(&e, line_positions + 24), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, line_positions + 36), [1.0, -2.0, 3.0]);
        // Edge 11 joins corners 3 and 7.
        assert_eq!(point(&e, line_positions + 12 * 22), [1.0, -2.0, -3.0]);
        assert_eq!(point(&e, line_positions + 12 * 23), [-1.0, -2.0, -3.0]);
        for slot in 0..24 {
            assert_eq!(e.mem.u32(line_colors + 16 * slot), 0xc0);
            assert_eq!(e.mem.u32(line_colors + 16 * slot + 12), 0xc3);
            assert_eq!(e.mem.u8(line_flags + slot) as u32, 1 - slot % 2);
        }
        // Triangle 0 uses corners 2, 3, 1.
        assert_eq!(point(&e, triangle_positions), [1.0, -2.0, 3.0]);
        assert_eq!(point(&e, triangle_positions + 12), [1.0, -2.0, -3.0]);
        assert_eq!(point(&e, triangle_positions + 24), [1.0, 2.0, -3.0]);
        // The last triangle ends with corners 7, 5, 1.
        assert_eq!(point(&e, triangle_positions + 12 * 35), [1.0, 2.0, -3.0]);
        assert_eq!(point(&e, triangle_positions + 12 * 34), [-1.0, 2.0, -3.0]);
        assert_eq!(point(&e, triangle_positions + 12 * 33), [-1.0, -2.0, -3.0]);
        for slot in 0..36 {
            assert_eq!(e.mem.u32(triangle_colors + 16 * slot), 1);
            assert_eq!(e.mem.u32(triangle_colors + 16 * slot + 12), 4);
            assert_eq!(e.mem.u16(list + 2 * slot) as u32, slot);
        }
    }
}
