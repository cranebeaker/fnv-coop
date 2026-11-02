//! `BSMain/bsocclusionplanlet store = |e.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSOcclusionPlane` is a rectangle in space (centre, two half extents and
//! a rotation) that hides what lies behind it. It keeps four corner vertices
//! (`GetVertices`), the six planes of the pyramid from a viewpoint through
//! those corners (`GetFrustumPlanes`) and, for each, a cache that is rebuilt
//! when the dirty flag is set or the viewpoint moves.
//!
//! The corner order is the one `GetVertices` writes: centre plus first axis
//! plus second axis, plus minus, minus minus, minus plus. The tests that
//! `WithinFrustum` and its siblings run use four helper routines whose
//! meaning comes from their bodies: `NiPoint3` subtraction (`00439ef0`),
//! addition (`00439e90`), scaling by a float (`0045bb20`), the dot product
//! (`004b6190`, result in `ST0`) and the point-against-plane test
//! (`0049da80`): the code compares its result with 2 for "the point is
//! behind the plane".
//!
//! C++ exception unwinding frames (constructor, `CreateClone`) and the stack
//! cookie check of `MakeDebugGeom` are not translated; those functions carry
//! the `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsmain::bscompoundfrustum::NiFrustumPlanes;
use std::cmp::Ordering;

// ---------------------------------------------------------------------------
// Constants

/// Address of the `BSOcclusionPlane` vtable.
pub(crate) const BS_OCCLUSION_PLANE_VTABLE: u32 = 0x0101_e794;
/// Size of a `BSOcclusionPlane` on PC (the Xbox build is 0x110).
pub(crate) const BS_OCCLUSION_PLANE_SIZE: u32 = 0xfc;
/// The run-time type information `RTTI == &...` tests compare `GetRTTI` (vtable
/// slot 8) against in `00c344c0` and `MakeDebugGeom`.
const OCCLUSION_PLANE_RTTI: u32 = 0x0120_30b0;
/// Vtable slot of `GetRTTI`.
const SLOT_GET_RTTI: u32 = 0x08;

/// Offset of `kRotation` (a `NiMatrix3`, nine floats by rows).
const ROTATION: u32 = 0x1c;
/// Offset of the cached corner vertices (four `NiPoint3`).
const CACHED_VERTICES: u32 = 0xa8;
/// Offset of the cached `NiFrustumPlanes` (0x64 bytes).
const CACHED_PLANES: u32 = 0x44;
/// Size of the cached `NiFrustumPlanes` in dwords.
const FRUSTUM_PLANES_DWORDS: u32 = 0x19;
/// Offset of the active-plane mask inside a `NiFrustumPlanes`.
const ACTIVE_PLANES: u32 = 0x60;
/// Size of a `NiPlane`.
const PLANE_SIZE: u32 = 0x10;

/// `NiObject::NiObject()`, thiscall, no arguments.
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiPlane::NiPlane()` (Xbox PDB).
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// `NiPlane::NiPlane_ov4(p0, p1, p2)` (Xbox PDB): the plane through three
/// points, built into `this`; returns `this`.
const NI_PLANE_FROM_POINTS: u32 = 0x00a6_99d0;
/// `NiFrustumPlanes::NiFrustumPlanes()` (Xbox PDB).
const NI_FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
/// The point-against-plane test (`this` the plane, then the point).
const PLANE_SIDE_OF_POINT: u32 = 0x0049_da80;
/// The value `PLANE_SIDE_OF_POINT` returns for "behind the plane".
const SIDE_BEHIND: i32 = 2;
/// `NiPoint3::Dot(other)` (Xbox PDB), result in `ST0`.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// `NiPoint3` subtraction: `this - other` into `out`; returns `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `NiPoint3` addition: `this + other` into `out`; returns `out`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `NiPoint3` scaled: `this * scale` into `out` (`scale` a float on the
/// stack); returns `out`.
const POINT_SCALE: u32 = 0x0045_bb20;
/// Normalizes the `NiPoint3` at `this` in place.
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// `result = scalar * vector` (cdecl: result, scalar, vector); returns
/// `result`.
const POINT_SCALED_BY: u32 = 0x004a_3760;
/// `NiMatrix3 * NiPoint3` (`this` the matrix): the product into `out`,
/// which is returned.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
/// `NiMatrix3::Inverse_ov2(out)` (Xbox PDB): bool.
const MATRIX_INVERSE: u32 = 0x004b_46a0;
/// Called with the inverse when `Inverse_ov2` fails (thiscall, no
/// arguments).
const MATRIX_CLEAR: u32 = 0x004b_4600;
/// `fabs` (`00ec6cde`), double in, double out.
const FABS: u32 = 0x00ec_6cde;
/// The CRT routine `00ec9f30`, which the exe's own tables register as the
/// cosine (double in, double out).
const COSINE: u32 = 0x00ec_9f30;
/// `memcpy(dst, src, size)`.
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiTriShapeData::GetTriList()` (Xbox PDB): the index array of the data.
const TRI_LIST: u32 = 0x00a7_b850;
/// `operator new(size)`, cdecl.
const OPERATOR_NEW: u32 = 0x00aa_13e0;

/// The camera-independent constants the code reads from memory.
/// The three `float`s the constructor copies into `kCenter`.
const DEFAULT_CENTER: u32 = 0x011f_426c;
/// The `float` the constructor stores in both half extents.
const DEFAULT_HALF_EXTENT: u32 = 0x0101_b268;
/// The nine `float`s the constructor copies into `kRotation`.
const DEFAULT_ROTATION: u32 = 0x011a_9448;
/// The `double` the frustum tests compare against (the code adds
/// "`-0.0001`" tolerance with it).
const PLANE_TOLERANCE: u32 = 0x010c_0f58;
/// `0.0` (`double`), read from the exe as the code does.
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// The lower and upper limits `00c344c0` puts on its signed distance (`double`s).
const DISTANCE_LOWER_LIMIT: u32 = 0x0107_2f80;
const DISTANCE_UPPER_LIMIT: u32 = 0x0102_0998;

// `MakeDebugGeom` constants (all `float`s unless noted).
/// How far the debug volume's far corners are pushed from the viewpoint.
const DEBUG_FAR_DISTANCE: u32 = 0x0105_00cc;
/// Half length of the marker lines through the plane's centre axis.
const DEBUG_MARKER_LENGTH: u32 = 0x0101_8f5c;
/// Colour components of the debug lines.
const DEBUG_VALUE_LOW: u32 = 0x0101_e2bc;
const DEBUG_VALUE_HIGH: u32 = 0x0101_8238;
const DEBUG_VALUE_DIM: u32 = 0x0103_0ff4;
/// The four words of the marker lines' colour; the first three are replaced
/// by the alpha argument.
const DEBUG_MARKER_COLOR: u32 = 0x011a_9be0;
/// Alpha of the triangles when the last argument is set / clear.
const DEBUG_TRIANGLE_ALPHA_SET: u32 = 0x0101_df44;

/// The line index pairs `MakeDebugGeom` uses (corner indices, two per line).
const DEBUG_LINE_CORNERS: [u16; 16] = [0, 1, 3, 2, 0, 3, 2, 1, 0, 4, 1, 5, 2, 6, 3, 7];
/// The triangle corner indices `MakeDebugGeom` uses (three per triangle).
const DEBUG_TRIANGLE_CORNERS: [u16; 30] = [
    0, 1, 2, 2, 3, 0, 5, 0, 4, 5, 1, 0, 5, 2, 1, 6, 2, 5, 6, 7, 2, 7, 3, 2, 0, 3, 7, 7, 4, 0,
];
/// Number of line segments and triangles `MakeDebugGeom` writes, and the
/// number of triangles of the geometry it pads out to.
const DEBUG_LINE_COUNT: u32 = 4;
const DEBUG_TRIANGLE_COUNT: u32 = 2;
const DEBUG_TRIANGLE_SLOTS: u32 = 10;
const DEBUG_LINE_SLOTS: u32 = 8;

layout! {
    /// `BSOcclusionPlane` (Xbox PDB), 0xFC bytes on PC (0x110 on the Xbox:
    /// the matrix and the cached planes sit at different offsets there).
    pub struct BSOcclusionPlane: 0xfc {
        /// `kCenter` (Xbox PDB): `NiPoint3`.
        0x08 kCenter: Inline<crate::types::NiPoint3>,
        /// `kHalfExtents.x` (Xbox PDB `kHalfExtents`, a `NiPoint2`).
        0x14 kHalfExtentX: f32,
        /// `kHalfExtents.y` (Xbox PDB).
        0x18 kHalfExtentY: f32,
        /// `kCullResult` (Xbox PDB): `BSMultiBoundShape::BSMBCullResult`.
        0x40 kCullResult: i32,
        /// `bDirtyVertices` (Xbox PDB).
        0xE4 bDirtyVertices: bool,
        /// `bDirtyPlanes` (Xbox PDB).
        0xE5 bDirtyPlanes: bool,
        /// `bDisabledPlanes[0]` (Xbox PDB).
        0xE6 bDisabledPlanes0: u8,
        /// `bDisabledPlanes[1]` (Xbox PDB).
        0xE7 bDisabledPlanes1: u8,
        /// `bDisabledPlanes[2]` (Xbox PDB).
        0xE8 bDisabledPlanes2: u8,
        /// `bDisabledPlanes[3]` (Xbox PDB).
        0xE9 bDisabledPlanes3: u8,
        /// `bTwoSided` (Xbox PDB).
        0xEA bTwoSided: bool,
        /// `pLinkedPlanes[0]` (Xbox PDB).
        0xEC pLinkedPlane0: Ptr,
        /// `pLinkedPlanes[1]` (Xbox PDB).
        0xF0 pLinkedPlane1: Ptr,
        /// `pLinkedPlanes[2]` (Xbox PDB).
        0xF4 pLinkedPlane2: Ptr,
        /// `pLinkedPlanes[3]` (Xbox PDB).
        0xF8 pLinkedPlane3: Ptr,
    }
}

type Plane = Ptr<BSOcclusionPlane>;

// ---------------------------------------------------------------------------
// Helpers

fn float_at(e: &Engine, address: u32) -> f32 {
    e.mem.f32(address)
}

/// Copies `words` dwords from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `NiPoint3::Dot` (result in `ST0`).
fn dot(e: &mut Engine, normal: u32, other: u32) -> f64 {
    e.call(NI_POINT3_DOT, &args![normal, other]).f64()
}

/// The point-against-plane test.
fn side_of_point(e: &mut Engine, plane: u32, point: u32) -> i32 {
    e.call(PLANE_SIDE_OF_POINT, &args![plane, point]).i32()
}

/// `fabs` through the CRT routine, rounded to `float` where the code stores
/// the result in one.
fn absolute(e: &mut Engine, value: f64) -> f32 {
    e.call(FABS, &args![value]).f64() as f32
}

/// The x87 test `FCOMP`, `TEST AH,1`, `JNZ`: taken when `a < b` or the two are
/// unordered (a NaN).
fn less_or_unordered(a: f64, b: f64) -> bool {
    !matches!(a.partial_cmp(&b), Some(Ordering::Greater | Ordering::Equal))
}

/// The two x87 tests on the crossing parameter `t`: `FCOMP` with 0 (fail when
/// `t < 0` or unordered) and with 1 (fail unless `t <= 1`).
fn outside_unit_range(t: f64, zero: f64, one: f64) -> bool {
    less_or_unordered(t, zero)
        || !matches!(t.partial_cmp(&one), Some(Ordering::Less | Ordering::Equal))
}

/// The address of plane `index` (16 bytes each) of a `NiFrustumPlanes`.
fn plane_at(planes: Ptr, index: u32) -> u32 {
    planes.addr() + index * PLANE_SIZE
}

/// The exact-class test `00c344c0` and `MakeDebugGeom` make: the object is
/// not null and its `GetRTTI` (vtable slot 8) returns the class's
/// information block.
fn is_exact_occlusion_plane(e: &mut Engine, this: Plane) -> bool {
    if this.is_null() {
        return false;
    }
    e.vcall(this.addr(), SLOT_GET_RTTI, &args![]).u32() == OCCLUSION_PLANE_RTTI
}

// ---------------------------------------------------------------------------
// Construction and cloning

// Translated from 00c335d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::BSOcclusionPlane` (Xbox PDB): builds the `NiObject`
/// base, sets the vtable, constructs the six planes of the cached frustum
/// (all active), takes the centre from the global `011f426c`, both half
/// extents from the float at `0101b268`, the nine rotation floats from
/// `011a9448`, clears the cull result and the linked planes and makes the
/// plane two sided. The dirty flags are left as they are, as in the game.
// Translation incomplete: 00c335d0 - C++ exception unwinding frame is not translated.
pub fn bs_occlusion_plane_bs_occlusion_plane(e: &mut Engine, this: Plane) -> Plane {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BS_OCCLUSION_PLANE_VTABLE);
    let planes = this.addr() + CACHED_PLANES;
    for index in 0..6 {
        e.call(NI_PLANE_CONSTRUCT, &args![planes + index * PLANE_SIZE]);
    }
    e.mem.set_u32(planes + ACTIVE_PLANES, 0x3f);
    copy_words(e, this.addr() + 0x08, DEFAULT_CENTER, 3);
    let half_extent = float_at(e, DEFAULT_HALF_EXTENT);
    e.set(this, BSOcclusionPlane::kHalfExtentX, half_extent);
    e.set(this, BSOcclusionPlane::kHalfExtentY, half_extent);
    copy_words(e, this.addr() + ROTATION, DEFAULT_ROTATION, 9);
    e.set(this, BSOcclusionPlane::kCullResult, 0);
    e.set(this, BSOcclusionPlane::bTwoSided, true);
    for slot in 0..4 {
        e.mem.set_u32(this.addr() + 0xec + 4 * slot, 0);
    }
    this
}

// Translated from 00c33730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::CreateClone` (Xbox PDB): allocates and constructs a
/// plane (null if the allocation failed), then copies this plane's members
/// into it (`CopyMembers` still runs on a null result, as in the game).
// Translation incomplete: 00c33730 - C++ exception unwinding frame is not translated.
pub fn bs_occlusion_plane_create_clone(e: &mut Engine, this: Plane, clone_process: u32) -> Plane {
    let block = e
        .call(OPERATOR_NEW, &args![BS_OCCLUSION_PLANE_SIZE])
        .ptr::<BSOcclusionPlane>();
    let clone = if block.is_null() {
        Ptr::NULL
    } else {
        bs_occlusion_plane_bs_occlusion_plane(e, block)
    };
    bs_occlusion_plane_copy_members(e, this, clone, clone_process);
    clone
}

// Translated from 00c337c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::CopyMembers` (Xbox PDB): copies the centre, the half
/// extents, the rotation and the cull result into `target` and marks its
/// vertices and planes dirty. The second argument is never read.
pub fn bs_occlusion_plane_copy_members(e: &mut Engine, this: Plane, target: Plane, _unused_2: u32) {
    copy_words(e, target.addr() + 0x08, this.addr() + 0x08, 3);
    e.set(target, BSOcclusionPlane::bDirtyVertices, true);
    e.set(target, BSOcclusionPlane::bDirtyPlanes, true);
    copy_words(e, target.addr() + 0x14, this.addr() + 0x14, 2);
    e.set(target, BSOcclusionPlane::bDirtyVertices, true);
    e.set(target, BSOcclusionPlane::bDirtyPlanes, true);
    copy_words(e, target.addr() + ROTATION, this.addr() + ROTATION, 9);
    e.set(target, BSOcclusionPlane::bDirtyVertices, true);
    e.set(target, BSOcclusionPlane::bDirtyPlanes, true);
    let cull_result = e.get(this, BSOcclusionPlane::kCullResult);
    e.set(target, BSOcclusionPlane::kCullResult, cull_result);
}

// ---------------------------------------------------------------------------
// Corners and planes

// Translated from 00c35c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::GetVertices` (Xbox PDB): writes the four corners
/// (four `NiPoint3`, 0x30 bytes) into `out`. With the dirty flag clear they
/// are the cached ones. Otherwise the two axes `a = (hx, 0, 0)` and
/// `b = (0, 0, hy)` (the half extents) are rotated by `kRotation`
/// (`NiMatrix3 * NiPoint3`), the corners are the centre `c` plus `a` plus
/// `b` (at `out+0x00`), `c+a-b` (`+0x0C`), `c-a-b` (`+0x18`) and `c-a+b`
/// (`+0x24`), each sum rounded to `float` step by step, and the result is
/// cached and the dirty flag cleared.
pub fn bs_occlusion_plane_get_vertices(e: &mut Engine, this: Plane, out: Ptr) {
    if !e.get(this, BSOcclusionPlane::bDirtyVertices) {
        e.call(MEMCPY, &args![out, this.addr() + CACHED_VERTICES, 0x30u32]);
        return;
    }
    e.with_stack(0xb0, |e, stack| {
        let s = stack.addr();
        // Slots: first axis, second axis, two rotated results, then the
        // intermediate vectors of the four corner sums.
        let (axis_a, axis_b) = (s, s + 0x0c);
        let rotated_a = s + 0x18;
        let rotated_b = s + 0x24;
        let temp = |n: u32| s + 0x30 + 0x0c * n;
        let half_x = e.mem.u32(this.addr() + 0x14);
        let half_y = e.mem.u32(this.addr() + 0x18);
        e.mem.set_u32(axis_a, half_x);
        e.mem.set_u32(axis_a + 4, 0);
        e.mem.set_u32(axis_a + 8, 0);
        e.mem.set_u32(axis_b, 0);
        e.mem.set_u32(axis_b + 4, 0);
        e.mem.set_u32(axis_b + 8, half_y);
        let rotation = this.addr() + ROTATION;
        let product = e
            .call(MATRIX_TIMES_POINT, &args![rotation, rotated_a, axis_a])
            .u32();
        copy_words(e, axis_a, product, 3);
        let product = e
            .call(MATRIX_TIMES_POINT, &args![rotation, rotated_b, axis_b])
            .u32();
        copy_words(e, axis_b, product, 3);

        let center = this.addr() + 0x08;
        // centre - a, + b  -> out+0x24
        let first = e
            .call(POINT_SUBTRACT, &args![center, temp(0), axis_a])
            .u32();
        let sum = e.call(POINT_ADD, &args![first, temp(1), axis_b]).u32();
        copy_words(e, out.addr() + 0x24, sum, 3);
        // centre - a, - b  -> out+0x18
        let first = e
            .call(POINT_SUBTRACT, &args![center, temp(2), axis_a])
            .u32();
        let sum = e.call(POINT_SUBTRACT, &args![first, temp(3), axis_b]).u32();
        copy_words(e, out.addr() + 0x18, sum, 3);
        // centre + a, - b  -> out+0x0C
        let first = e.call(POINT_ADD, &args![center, temp(4), axis_a]).u32();
        let sum = e.call(POINT_SUBTRACT, &args![first, temp(5), axis_b]).u32();
        copy_words(e, out.addr() + 0x0c, sum, 3);
        // centre + a, + b  -> out+0x00
        let first = e.call(POINT_ADD, &args![center, temp(6), axis_a]).u32();
        let sum = e.call(POINT_ADD, &args![first, temp(7), axis_b]).u32();
        copy_words(e, out.addr(), sum, 3);

        e.call(MEMCPY, &args![this.addr() + CACHED_VERTICES, out, 0x30u32]);
        e.set(this, BSOcclusionPlane::bDirtyVertices, false);
    });
}

// Translated from 00c33870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::GetFrustumPlanes` (Xbox PDB): fills `out` (a
/// `NiFrustumPlanes`) with the planes of the pyramid from the viewpoint
/// `(x, y, z)` through the four corners: plane 0 is the occlusion plane
/// itself (plane 1 stays untouched and inactive) and planes 2 to 5 are the
/// sides through the viewpoint. The winding of every plane depends on which
/// side of the occlusion plane the viewpoint is on. Planes the
/// `bDisabledPlanes` flags equal to 1 switch off (plane `n + 2` for flag
/// `n`). With the planes clean and the viewpoint unchanged the cached planes
/// are copied instead; otherwise the result is cached with the viewpoint.
pub fn bs_occlusion_plane_get_frustum_planes(
    e: &mut Engine,
    this: Plane,
    x: f32,
    y: f32,
    z: f32,
    out: Ptr,
) {
    let cached_view = this.addr() + 0xd8;
    if !e.get(this, BSOcclusionPlane::bDirtyPlanes)
        && float_at(e, cached_view) == x
        && float_at(e, cached_view + 4) == y
        && float_at(e, cached_view + 8) == z
    {
        copy_words(
            e,
            out.addr(),
            this.addr() + CACHED_PLANES,
            FRUSTUM_PLANES_DWORDS,
        );
        return;
    }
    e.with_stack(0x30 + 0x0c + 0x10 + 0x10, |e, stack| {
        let vertices = stack.addr();
        let vertex = |n: u32| vertices + 0x0c * n;
        let view = stack.addr() + 0x30;
        let side_plane = stack.addr() + 0x3c;
        let temp = stack.addr() + 0x4c;
        e.mem.set_f32(view, x);
        e.mem.set_f32(view + 4, y);
        e.mem.set_f32(view + 8, z);
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        e.mem.set_u32(out.addr() + ACTIVE_PLANES, 0x3f);
        e.call(
            NI_PLANE_FROM_POINTS,
            &args![side_plane, vertex(0), vertex(1), vertex(2)],
        );
        let side = side_of_point(e, side_plane, view);
        // Builds the plane through three points and stores it as plane
        // `index` of `out`.
        let store = |e: &mut Engine, index: u32, p0: u32, p1: u32, p2: u32| {
            let built = e.call(NI_PLANE_FROM_POINTS, &args![temp, p0, p1, p2]).u32();
            copy_words(e, plane_at(out, index), built, 4);
        };
        if side != 1 {
            store(e, 0, vertex(0), vertex(1), vertex(2));
            let mask = e.mem.u32(out.addr() + ACTIVE_PLANES);
            e.mem.set_u32(out.addr() + ACTIVE_PLANES, mask & !2);
            store(e, 2, vertex(0), vertex(1), view);
            store(e, 3, vertex(2), vertex(3), view);
            store(e, 4, vertex(3), vertex(0), view);
            store(e, 5, vertex(1), vertex(2), view);
        } else {
            store(e, 0, vertex(1), vertex(0), vertex(2));
            let mask = e.mem.u32(out.addr() + ACTIVE_PLANES);
            e.mem.set_u32(out.addr() + ACTIVE_PLANES, mask & !2);
            store(e, 2, vertex(1), vertex(0), view);
            store(e, 3, vertex(3), vertex(2), view);
            store(e, 4, vertex(0), vertex(3), view);
            store(e, 5, vertex(2), vertex(1), view);
        }
        for flag in 0..4u32 {
            if e.mem.u8(this.addr() + 0xe6 + flag) == 1 {
                let mask = e.mem.u32(out.addr() + ACTIVE_PLANES);
                e.mem
                    .set_u32(out.addr() + ACTIVE_PLANES, mask & !(1 << (flag + 2)));
            }
        }
        e.mem.set_f32(cached_view, x);
        e.mem.set_f32(cached_view + 4, y);
        e.mem.set_f32(cached_view + 8, z);
        copy_words(
            e,
            this.addr() + CACHED_PLANES,
            out.addr(),
            FRUSTUM_PLANES_DWORDS,
        );
        e.set(this, BSOcclusionPlane::bDirtyPlanes, false);
    });
}

// Translated from 00c358a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::ViewerOnCorrectSide` (Xbox PDB): true for a two-sided
/// plane. Otherwise the viewpoint `(x, y, z)` has to be behind the plane
/// through the first three corners. (The frustum planes the code builds
/// first are never read, but the calls are made.)
pub fn bs_occlusion_plane_viewer_on_correct_side(
    e: &mut Engine,
    this: Plane,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    if e.get(this, BSOcclusionPlane::bTwoSided) {
        return true;
    }
    e.with_stack(NiFrustumPlanes::SIZE + 0x30 + 0x10 + 0x0c, |e, stack| {
        let frustum = stack.addr();
        let vertices = frustum + NiFrustumPlanes::SIZE;
        let plane = vertices + 0x30;
        let view = plane + 0x10;
        e.mem.set_f32(view, x);
        e.mem.set_f32(view + 4, y);
        e.mem.set_f32(view + 8, z);
        e.call(NI_FRUSTUM_PLANES_CONSTRUCT, &args![frustum]);
        bs_occlusion_plane_get_frustum_planes(e, this, x, y, z, Ptr::new(frustum));
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        e.call(
            NI_PLANE_FROM_POINTS,
            &args![plane, vertices, vertices + 0x0c, vertices + 0x18],
        );
        side_of_point(e, plane, view) == SIDE_BEHIND
    })
}

// Translated from 00c35980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::GetJoinedPlaneType` (Xbox PDB): 2 when the first or
/// third linked plane is set, else 1 when the second or fourth is, else 0.
pub fn bs_occlusion_plane_get_joined_plane_type(e: &mut Engine, this: Plane) -> i32 {
    let linked = |e: &Engine, slot: u32| e.mem.u32(this.addr() + 0xec + 4 * slot);
    if linked(e, 0) != 0 || linked(e, 2) != 0 {
        2
    } else if linked(e, 1) != 0 || linked(e, 3) != 0 {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Tests against frustum planes

/// Sets the four `bDisabledPlanes` bytes from the four masks the pair tests
/// computed: `kMasks[j]` holds a bit for every plane that vertex `j` is behind.
fn store_pair_flags(e: &mut Engine, this: Plane, masks: &[u32; 4]) {
    let flag = |a: u32, b: u32| (a & b != 0) as u8;
    e.mem.set_u8(this.addr() + 0xe6, flag(masks[0], masks[1]));
    e.mem.set_u8(this.addr() + 0xe7, flag(masks[3], masks[2]));
    e.mem.set_u8(this.addr() + 0xe8, flag(masks[0], masks[3]));
    e.mem.set_u8(this.addr() + 0xe9, flag(masks[2], masks[1]));
}

// Translated from 00c35ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Where the segment from `start` to `end` crosses `plane`, and whether that
/// point lies inside every active plane of the `NiFrustumPlanes` `planes`
/// (inside meaning "more than the tolerance in front"). False when the
/// segment is parallel to `plane` or crosses it outside `0 <= t <= 1`.
/// (`this` is not read.)
pub fn fn_00c35ae0(
    e: &mut Engine,
    _this: Plane,
    start: Ptr,
    end: Ptr,
    plane: Ptr,
    planes: Ptr,
) -> bool {
    e.with_stack(0x30, |e, stack| {
        let s = stack.addr();
        let (difference, normal, scaled, point) = (s, s + 0x0c, s + 0x18, s + 0x24);
        e.call(POINT_SUBTRACT, &args![end, difference, start]);
        copy_words(e, normal, plane.addr(), 3);
        let denominator = dot(e, normal, difference) as f32;
        let (zero, one) = (e.global::<f64>(ZERO), e.global::<f64>(ONE));
        if denominator as f64 == zero {
            return false;
        }
        let constant = float_at(e, plane.addr() + 0x0c);
        let along = dot(e, normal, start.addr());
        let t = ((constant as f64 - along) / denominator as f64) as f32;
        if outside_unit_range(t as f64, zero, one) {
            return false;
        }
        let scaled_difference = e.call(POINT_SCALE, &args![difference, scaled, t]).u32();
        e.call(POINT_ADD, &args![start, point, scaled_difference]);
        let tolerance = e.global::<f64>(PLANE_TOLERANCE);
        let active = e.mem.u32(planes.addr() + ACTIVE_PLANES);
        for index in 0..6u32 {
            if (1 << index) & active != 0 {
                let candidate = plane_at(planes, index);
                let constant = float_at(e, candidate + 0x0c);
                let value = dot(e, candidate, point) - constant as f64;
                if value <= tolerance {
                    return false;
                }
            }
        }
        true
    })
}

// Translated from 00c359d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Where the segment from `start` to `end` crosses `plane`, and whether that
/// point is on the inside of `plane_a` and `plane_b` (not more than the
/// tolerance behind either). False when the segment is parallel to `plane`
/// or crosses it outside `0 <= t <= 1`. (`this` is not read.)
pub fn fn_00c359d0(
    e: &mut Engine,
    _this: Plane,
    start: Ptr,
    end: Ptr,
    plane: Ptr,
    plane_a: Ptr,
    plane_b: Ptr,
) -> bool {
    e.with_stack(0x30, |e, stack| {
        let s = stack.addr();
        let (difference, normal, scaled, point) = (s, s + 0x0c, s + 0x18, s + 0x24);
        e.call(POINT_SUBTRACT, &args![end, difference, start]);
        copy_words(e, normal, plane.addr(), 3);
        let denominator = dot(e, normal, difference) as f32;
        let (zero, one) = (e.global::<f64>(ZERO), e.global::<f64>(ONE));
        if denominator as f64 == zero {
            return false;
        }
        let constant = float_at(e, plane.addr() + 0x0c);
        let along = dot(e, normal, start.addr());
        let t = ((constant as f64 - along) / denominator as f64) as f32;
        if outside_unit_range(t as f64, zero, one) {
            return false;
        }
        let scaled_difference = e.call(POINT_SCALE, &args![difference, scaled, t]).u32();
        e.call(POINT_ADD, &args![start, point, scaled_difference]);
        let tolerance = e.global::<f64>(PLANE_TOLERANCE);
        for candidate in [plane_a, plane_b] {
            let constant = float_at(e, candidate.addr() + 0x0c);
            let value = dot(e, candidate.addr(), point) - constant as f64;
            if less_or_unordered(value, tolerance) {
                return false;
            }
        }
        true
    })
}

// Translated from 00c33cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::WithinFrustum` (Xbox PDB): tests the plane's four
/// corners against the active planes of `planes` (a `NiFrustumPlanes`).
/// False as soon as one active plane has all four corners behind it.
/// Otherwise it records in `bDisabledPlanes` which pairs of neighbouring
/// corners (0-1, 3-2, 0-3, 2-1) share a plane they are behind. If none of
/// the pairs is complete, or exactly one is missing and the corresponding
/// edge crosses no planes 2 to 5 inside the frustum, the result is true. In the
/// remaining cases it is true when a diagonal (0-2 or 1-3) crosses one of
/// planes 2 to 5 inside the frustum (`fn_00c35ae0`); when none does and
/// `test_extents` is set, true when the viewpoint `(x, y, z)` relative to
/// the plane's centre lies within the half extents along the first two
/// rotation rows; false otherwise.
pub fn bs_occlusion_plane_within_frustum(
    e: &mut Engine,
    this: Plane,
    planes: Ptr,
    test_extents: u8,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    e.with_stack(0x30 + 0x0c + 0x0c, |e, stack| {
        let vertices = stack.addr();
        let vertex = |n: u32| Ptr::new(vertices + 0x0c * n);
        let view = vertices + 0x30;
        let difference = view + 0x0c;
        e.mem.set_f32(view, x);
        e.mem.set_f32(view + 4, y);
        e.mem.set_f32(view + 8, z);
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));

        let active = e.mem.u32(planes.addr() + ACTIVE_PLANES);
        let mut masks = [0u32; 4];
        for index in 0..6u32 {
            if (1 << index) & active == 0 {
                continue;
            }
            let mut all_behind = true;
            for (corner, mask) in masks.iter_mut().enumerate() {
                let side = side_of_point(e, plane_at(planes, index), vertex(corner as u32).addr());
                if side == SIDE_BEHIND {
                    *mask |= 1 << index;
                } else {
                    all_behind = false;
                }
            }
            if all_behind {
                return false;
            }
        }
        store_pair_flags(e, this, &masks);

        let flag = |e: &Engine, n: u32| e.mem.u8(this.addr() + 0xe6 + n) != 0;
        let (f6, f7, f8, f9) = (flag(e, 0), flag(e, 1), flag(e, 2), flag(e, 3));
        // True when no plane 2 to 5 lets the edge from `a` to `b` through.
        let no_crossing = |e: &mut Engine, a: Ptr, b: Ptr| {
            for index in 2..6u32 {
                if fn_00c35ae0(e, this, a, b, Ptr::new(plane_at(planes, index)), planes) {
                    return false;
                }
            }
            true
        };
        let mut check_diagonals = false;
        if f6 && f7 && f8 && f9 {
            check_diagonals = true;
        } else if !f6 && f7 && f8 && f9 {
            check_diagonals = no_crossing(e, vertex(0), vertex(1));
        } else if f6 && !f7 && f8 && f9 {
            check_diagonals = no_crossing(e, vertex(3), vertex(2));
        } else if f6 && f7 && !f8 && f9 {
            check_diagonals = no_crossing(e, vertex(0), vertex(3));
        } else if f6 && f7 && f8 && !f9 {
            check_diagonals = no_crossing(e, vertex(2), vertex(1));
        }
        if !check_diagonals {
            return true;
        }
        for index in 2..6u32 {
            let candidate = Ptr::new(plane_at(planes, index));
            if fn_00c35ae0(e, this, vertex(0), vertex(2), candidate, planes)
                || fn_00c35ae0(e, this, vertex(1), vertex(3), candidate, planes)
            {
                return true;
            }
        }
        if test_extents == 0 {
            return false;
        }
        e.call(POINT_SUBTRACT, &args![view, difference, this.addr() + 0x08]);
        let first = dot(e, difference, this.addr() + ROTATION) as f32;
        let first = absolute(e, first as f64);
        let second = dot(e, difference, this.addr() + ROTATION + 0x0c) as f32;
        let second = absolute(e, second as f64);
        let half_x = e.get(this, BSOcclusionPlane::kHalfExtentX);
        let half_y = e.get(this, BSOcclusionPlane::kHalfExtentY);
        half_x >= first && half_y >= second
    })
}

// Translated from 00c34350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::WithinFrustumFullTest` (Xbox PDB): the corner tests of
/// `WithinFrustum` without the early exit and without the follow-up
/// geometry: records the pair flags in `bDisabledPlanes` and returns false
/// when some active plane of `planes` has all four corners behind it.
pub fn bs_occlusion_plane_within_frustum_full_test(
    e: &mut Engine,
    this: Plane,
    planes: Ptr,
) -> bool {
    e.with_stack(0x30, |e, stack| {
        let vertices = stack.addr();
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        let active = e.mem.u32(planes.addr() + ACTIVE_PLANES);
        let mut masks = [0u32; 4];
        let mut inside = true;
        for index in 0..6u32 {
            if (1 << index) & active == 0 {
                continue;
            }
            let mut all_behind = true;
            for corner in 0..4u32 {
                let side = side_of_point(e, plane_at(planes, index), vertices + 0x0c * corner);
                if side == SIDE_BEHIND {
                    masks[corner as usize] |= 1 << index;
                } else {
                    all_behind = false;
                }
            }
            if all_behind {
                inside = false;
            }
        }
        store_pair_flags(e, this, &masks);
        inside
    })
}

// Translated from 00c344c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Takes the signed distance `s` of the
/// `NiPoint3` `point` from the plane through the first three corners and
/// moves the point onto that plane (`point - s * normal`). The first plane
/// of `planes` adds its own signed distance to `s`. When that sum lies in
/// the range given by the two `double`s at `01072f80` (inclusive) and
/// `01020998` (exclusive) and the moved point is within the half extents
/// along the first two rotation rows (relative to the centre's x and y),
/// the result is whether this object is exactly a `BSOcclusionPlane` (its
/// `GetRTTI` returns the information block at `012030b0`). In every other
/// case the result is `WithinFrustum(planes, 1, moved point)`.
pub fn fn_00c344c0(e: &mut Engine, this: Plane, planes: Ptr, point: Ptr) -> bool {
    e.with_stack(0x30 + 0x10 + 0x0c + 0x0c, |e, stack| {
        let vertices = stack.addr();
        let plane = vertices + 0x30;
        let scaled = plane + 0x10;
        let projected = scaled + 0x0c;
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        e.call(
            NI_PLANE_FROM_POINTS,
            &args![plane, vertices, vertices + 0x0c, vertices + 0x18],
        );
        let plane_constant = float_at(e, plane + 0x0c) as f64;
        let signed_distance = (dot(e, plane, point.addr()) - plane_constant) as f32;
        let moved = e
            .call(POINT_SCALED_BY, &args![scaled, -signed_distance, plane])
            .u32();
        e.call(POINT_ADD, &args![point, projected, moved]);
        let first_constant = float_at(e, planes.addr() + 0x0c) as f64;
        let first_dot = dot(e, planes.addr(), point.addr());
        let combined = ((first_dot - first_constant) as f32) as f64 + signed_distance as f64;
        let combined = combined as f32;

        let lower = e.global::<f64>(DISTANCE_LOWER_LIMIT);
        let upper = e.global::<f64>(DISTANCE_UPPER_LIMIT);
        if combined as f64 >= lower && (combined as f64) < upper {
            let row_x = this.addr() + ROTATION;
            let row_y = this.addr() + ROTATION + 0x0c;
            let center_x = float_at(e, this.addr() + 0x08);
            let center_y = float_at(e, this.addr() + 0x0c);
            let offset_x = (dot(e, projected, row_x) - center_x as f64) as f32;
            let offset_y = (dot(e, projected, row_y) - center_y as f64) as f32;
            let magnitude_x = absolute(e, offset_x as f64);
            let half_x = e.get(this, BSOcclusionPlane::kHalfExtentX);
            if half_x >= magnitude_x {
                let magnitude_y = absolute(e, offset_y as f64);
                let half_y = e.get(this, BSOcclusionPlane::kHalfExtentY);
                if half_y >= magnitude_y {
                    return is_exact_occlusion_plane(e, this);
                }
            }
        }
        let x = float_at(e, projected);
        let y = float_at(e, projected + 4);
        let z = float_at(e, projected + 8);
        bs_occlusion_plane_within_frustum(e, this, planes, 1, x, y, z)
    })
}

// Translated from 00c347d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: true when no corner of the plane is behind any
/// active plane of the `NiFrustumPlanes` `planes`.
pub fn fn_00c347d0(e: &mut Engine, this: Plane, planes: Ptr) -> bool {
    e.with_stack(0x30, |e, stack| {
        let vertices = stack.addr();
        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        let active = e.mem.u32(planes.addr() + ACTIVE_PLANES);
        for index in 0..6u32 {
            if (1 << index) & active == 0 {
                continue;
            }
            for corner in 0..4u32 {
                if side_of_point(e, plane_at(planes, index), vertices + 0x0c * corner)
                    == SIDE_BEHIND
                {
                    return false;
                }
            }
        }
        true
    })
}

// Translated from 00c34890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::TestIntersection` (Xbox PDB): whether the sphere
/// `bound` (centre `NiPoint3` at +0, radius at +0xC) overlaps the plane's
/// rectangle. The vector from the plane's centre to the bound's centre is
/// taken into the plane's own frame (through the inverse of `kRotation`; a
/// matrix that cannot be inverted is replaced by what `004b4600` leaves) and
/// made absolute `(lx, ly, lz)`. With `r` the radius, `lx - r` must not
/// exceed the first half extent, `lz - r` not the second, and `ly - r` must
/// be at most zero. With `reach = cos(|ly - r| / r) * r` the sphere also
/// needs `lx - reach` within the first half extent and, as the game does,
/// `ly - reach` (not `lz - reach`) within the second.
pub fn bs_occlusion_plane_test_intersection(e: &mut Engine, this: Plane, bound: Ptr) -> bool {
    e.with_stack(0x24 + 0x24 + 0x0c + 0x0c, |e, stack| {
        let s = stack.addr();
        let (inverse, copy, difference, local) = (s, s + 0x24, s + 0x48, s + 0x54);
        let rotation = this.addr() + ROTATION;
        if !e.call(MATRIX_INVERSE, &args![rotation, inverse]).bool() {
            e.call(MATRIX_CLEAR, &args![inverse]);
        }
        copy_words(e, copy, inverse, 9);
        let offset = e
            .call(
                POINT_SUBTRACT,
                &args![bound, difference, this.addr() + 0x08],
            )
            .u32();
        e.call(MATRIX_TIMES_POINT, &args![copy, local, offset]);
        for axis in 0..3u32 {
            let value = float_at(e, local + 4 * axis);
            let magnitude = absolute(e, value as f64);
            e.mem.set_f32(local + 4 * axis, magnitude);
        }
        let radius = float_at(e, bound.addr() + 0x0c);
        let local_x = float_at(e, local);
        let local_y = float_at(e, local + 4);
        let local_z = float_at(e, local + 8);
        let past_x = (local_x as f64 - radius as f64) as f32;
        let past_y = (local_y as f64 - radius as f64) as f32;
        let past_z = (local_z as f64 - radius as f64) as f32;
        let half_x = e.get(this, BSOcclusionPlane::kHalfExtentX);
        let half_y = e.get(this, BSOcclusionPlane::kHalfExtentY);
        if less_or_unordered(half_x as f64, past_x as f64)
            || less_or_unordered(half_y as f64, past_z as f64)
        {
            return false;
        }
        if !matches!(
            (past_y as f64).partial_cmp(&e.global::<f64>(ZERO)),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            return false;
        }
        let depth = absolute(e, past_y as f64);
        let ratio = (depth as f64 / radius as f64) as f32;
        let cosine = e.call(COSINE, &args![ratio as f64]).f64() as f32;
        let reach = (cosine as f64 * radius as f64) as f32;
        // The second comparison uses the plane-normal component again
        // (`local_y`), as the game does.
        half_x as f64 >= local_x as f64 - reach as f64
            && half_y as f64 >= local_y as f64 - reach as f64
    })
}

// ---------------------------------------------------------------------------
// Debug geometry

// Translated from 00c34ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSOcclusionPlane::MakeDebugGeom` (Xbox PDB): fills two pieces of debug
/// geometry for the viewpoint `(view_x, view_y, view_z)`. `lines` is a line
/// geometry (its data object at +0xB8 holds the vertices at +0x20, colours at
/// +0x28 and per-vertex line flags at +0x40), `triangles` a triangle
/// geometry (vertices, colours and, through `00a7b850`, its index list).
///
/// The eight corners are the plane's four and four more pushed away from the
/// viewpoint by the float at `010500cc`. The first four lines are the edges
/// of the plane (corner pairs 0-1, 3-2, 0-3, 2-1), coloured by the cull
/// result (1: yellow or green according to the pair flags in
/// `bDisabledPlanes`; otherwise a dim red) with the last colour component
/// `alpha`. A plane of exactly this class adds two marker lines along the
/// second rotation row through the first and fourth corner. The rest of the
/// lines are switched off. The triangles are the first two of ten corner
/// triples in `(r, g, b)` with alpha 0.2 (`dim_triangles` set) or 0.1; the
/// other eight index triples repeat the last used index.
// Translation incomplete: 00c34ad0 - the stack-cookie check at the end (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn bs_occlusion_plane_make_debug_geom(
    e: &mut Engine,
    this: Plane,
    view_x: f32,
    view_y: f32,
    view_z: f32,
    lines: Ptr,
    triangles: Ptr,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
    dim_triangles: u8,
) {
    e.with_stack(0x300, |e, stack| {
        let s = stack.addr();
        let vertices = s;
        let vertex = |n: u32| vertices + 0x0c * n;
        let view = s + 0x60;
        let direction = s + 0x6c;
        let up = s + 0x78;
        let scratch = |n: u32| s + 0x90 + 0x0c * n;
        e.mem.set_f32(view, view_x);
        e.mem.set_f32(view + 4, view_y);
        e.mem.set_f32(view + 8, view_z);

        bs_occlusion_plane_get_vertices(e, this, Ptr::new(vertices));
        // Far corners: each corner pushed away from the viewpoint.
        let far_distance = float_at(e, DEBUG_FAR_DISTANCE);
        for corner in 0..4u32 {
            let offset = e
                .call(
                    POINT_SUBTRACT,
                    &args![vertex(corner), scratch(3 * corner), view],
                )
                .u32();
            copy_words(e, direction, offset, 3);
            e.call(POINT_NORMALIZE, &args![direction]);
            let pushed = e
                .call(
                    POINT_SCALE,
                    &args![direction, scratch(3 * corner + 1), far_distance],
                )
                .u32();
            let far = e
                .call(
                    POINT_ADD,
                    &args![vertex(corner), scratch(3 * corner + 2), pushed],
                )
                .u32();
            copy_words(e, vertex(4 + corner), far, 3);
        }

        let line_data = e.mem.u32(lines.addr() + 0xb8);
        let line_positions = e.mem.u32(line_data + 0x20);
        let line_colors = e.mem.u32(line_data + 0x28);
        let line_flags = e.mem.u32(line_data + 0x40);

        let marker_alpha = alpha;
        let marker_color = [
            marker_alpha,
            marker_alpha,
            marker_alpha,
            float_at(e, DEBUG_MARKER_COLOR + 0x0c),
        ];
        let low = float_at(e, DEBUG_VALUE_LOW);
        let high = float_at(e, DEBUG_VALUE_HIGH);
        let dim = float_at(e, DEBUG_VALUE_DIM);
        let cull_result = e.get(this, BSOcclusionPlane::kCullResult);

        let write_color = |e: &mut Engine, address: u32, color: [f32; 4]| {
            for (n, component) in color.iter().enumerate() {
                e.mem.set_f32(address + 4 * n as u32, *component);
            }
        };

        let mut line = 0u32;
        while line < DEBUG_LINE_COUNT {
            let color = if cull_result == 1 {
                if e.mem.u8(this.addr() + 0xe6 + line) != 0 {
                    [1.0, 1.0, low, alpha]
                } else {
                    [low, 1.0, low, alpha]
                }
            } else {
                [high, dim, dim, alpha]
            };
            for end in 0..2u32 {
                let slot = 2 * line + end;
                let corner = DEBUG_LINE_CORNERS[slot as usize] as u32;
                copy_words(e, line_positions + 0x0c * slot, vertex(corner), 3);
                write_color(e, line_colors + 0x10 * slot, color);
            }
            e.mem.set_u8(line_flags + 2 * line, 1);
            e.mem.set_u8(line_flags + 2 * line + 1, 0);
            line += 1;
        }

        if is_exact_occlusion_plane(e, this) {
            // The second rotation row: the plane's own axis.
            copy_words(e, up, this.addr() + ROTATION + 0x0c, 3);
            let marker_length = float_at(e, DEBUG_MARKER_LENGTH);
            for corner in [0u32, 3] {
                let scaled = e
                    .call(POINT_SCALE, &args![up, scratch(0), marker_length])
                    .u32();
                let end = e
                    .call(POINT_ADD, &args![vertex(corner), scratch(1), scaled])
                    .u32();
                copy_words(e, line_positions + 0x0c * (2 * line), end, 3);
                write_color(e, line_colors + 0x10 * (2 * line), marker_color);
                let scaled = e
                    .call(POINT_SCALE, &args![up, scratch(2), marker_length])
                    .u32();
                let end = e
                    .call(POINT_SUBTRACT, &args![vertex(corner), scratch(3), scaled])
                    .u32();
                copy_words(e, line_positions + 0x0c * (2 * line + 1), end, 3);
                write_color(e, line_colors + 0x10 * (2 * line + 1), marker_color);
                e.mem.set_u8(line_flags + 2 * line, 1);
                e.mem.set_u8(line_flags + 2 * line + 1, 0);
                line += 1;
            }
        }
        while line < DEBUG_LINE_SLOTS {
            e.mem.set_u8(line_flags + 2 * line, 0);
            e.mem.set_u8(line_flags + 2 * line + 1, 0);
            line += 1;
        }

        let triangle_data = e.mem.u32(triangles.addr() + 0xb8);
        let triangle_positions = e.mem.u32(triangle_data + 0x20);
        let triangle_colors = e.mem.u32(triangle_data + 0x28);
        let triangle_alpha = if dim_triangles != 0 {
            float_at(e, DEBUG_TRIANGLE_ALPHA_SET)
        } else {
            low
        };
        let index_list = e.call(TRI_LIST, &args![triangle_data]).u32();
        let color = [red, green, blue, triangle_alpha];
        let mut triangle = 0u32;
        while triangle < DEBUG_TRIANGLE_COUNT {
            for corner in 0..3u32 {
                let slot = 3 * triangle + corner;
                let source = DEBUG_TRIANGLE_CORNERS[slot as usize] as u32;
                copy_words(e, triangle_positions + 0x0c * slot, vertex(source), 3);
                write_color(e, triangle_colors + 0x10 * slot, color);
            }
            for corner in 0..3u32 {
                let slot = 3 * triangle + corner;
                e.mem.set_u16(index_list + 2 * slot, slot as u16);
            }
            triangle += 1;
        }
        let last_index = ((triangle - 1) * 3 + 2) as u16;
        while triangle < DEBUG_TRIANGLE_SLOTS {
            for corner in 0..3u32 {
                e.mem
                    .set_u16(index_list + 2 * (3 * triangle + corner), last_index);
            }
            triangle += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c335d0, bs_occlusion_plane_bs_occlusion_plane(Plane) -> Plane),
        entry!(0x00c33730, bs_occlusion_plane_create_clone(Plane, u32) -> Plane),
        entry!(
            0x00c337c0,
            bs_occlusion_plane_copy_members(Plane, Plane, u32)
        ),
        entry!(
            0x00c33870,
            bs_occlusion_plane_get_frustum_planes(Plane, f32, f32, f32, Ptr)
        ),
        entry!(0x00c33cf0, bs_occlusion_plane_within_frustum(Plane, Ptr, u8, f32, f32, f32) -> bool),
        entry!(0x00c34350, bs_occlusion_plane_within_frustum_full_test(Plane, Ptr) -> bool),
        entry!(0x00c344c0, fn_00c344c0(Plane, Ptr, Ptr) -> bool),
        entry!(0x00c347d0, fn_00c347d0(Plane, Ptr) -> bool),
        entry!(0x00c34890, bs_occlusion_plane_test_intersection(Plane, Ptr) -> bool),
        entry!(
            0x00c34ad0,
            bs_occlusion_plane_make_debug_geom(
                Plane, f32, f32, f32, Ptr, Ptr, f32, f32, f32, f32, u8
            )
        ),
        entry!(0x00c358a0, bs_occlusion_plane_viewer_on_correct_side(Plane, f32, f32, f32) -> bool),
        entry!(0x00c35980, bs_occlusion_plane_get_joined_plane_type(Plane) -> i32),
        entry!(0x00c359d0, fn_00c359d0(Plane, Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00c35ae0, fn_00c35ae0(Plane, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00c35c10, bs_occlusion_plane_get_vertices(Plane, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    /// The four corners of the flat test plane (centre at the origin, half
    /// extents 10 and 5, identity rotation), in `GetVertices` order.
    const CORNERS: [[f32; 3]; 4] = [
        [10.0, 0.0, 5.0],
        [10.0, 0.0, -5.0],
        [-10.0, 0.0, -5.0],
        [-10.0, 0.0, 5.0],
    ];
    /// A vtable slot target the tests give `GetRTTI`.
    const RTTI_TARGET: u32 = 0x00de_0001;
    /// Where the fake vtable of a plane lives.
    const FAKE_VTABLE: u32 = 0x7100_0000;
    /// Where the fake triangle index list lives.
    const FAKE_INDEX_LIST: u32 = 0x7200_0000;

    /// (end, start) coordinates of the subtractions the tests record.
    type Subtractions = Rc<RefCell<Vec<([f32; 3], [f32; 3])>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn vec3(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn put3(e: &mut Engine, at: u32, value: [f32; 3]) {
        for (n, component) in value.iter().enumerate() {
            e.mem.set_f32(at + 4 * n as u32, *component);
        }
    }

    fn dot3(a: [f32; 3], b: [f32; 3]) -> f64 {
        a.iter().zip(b).map(|(x, y)| *x as f64 * y as f64).sum()
    }

    /// An engine whose geometry helpers do real arithmetic in `float`.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        let globals = [
            DEFAULT_CENTER,
            DEFAULT_HALF_EXTENT,
            DEFAULT_ROTATION,
            PLANE_TOLERANCE,
            ZERO,
            ONE,
            DISTANCE_LOWER_LIMIT,
            DISTANCE_UPPER_LIMIT,
            DEBUG_FAR_DISTANCE,
            DEBUG_MARKER_LENGTH,
            DEBUG_VALUE_LOW,
            DEBUG_VALUE_HIGH,
            DEBUG_VALUE_DIM,
            DEBUG_MARKER_COLOR,
            DEBUG_TRIANGLE_ALPHA_SET,
        ];
        let pages: BTreeSet<u32> = globals.iter().map(|g| g & !0xfff).collect();
        for page in pages {
            e.map(page, 0x1000);
        }
        e.set_global(PLANE_TOLERANCE, -0.0001f64);
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(DISTANCE_LOWER_LIMIT, -5.0f64);
        e.set_global(DISTANCE_UPPER_LIMIT, 5.0f64);
        e.set_global(DEBUG_FAR_DISTANCE, 2500.0f32);
        e.set_global(DEBUG_MARKER_LENGTH, 30.0f32);
        e.set_global(DEBUG_VALUE_LOW, 0.1f32);
        e.set_global(DEBUG_VALUE_HIGH, 0.7f32);
        e.set_global(DEBUG_VALUE_DIM, 0.15f32);
        e.set_global(DEBUG_MARKER_COLOR + 0x0c, 0.5f32);
        e.set_global(DEBUG_TRIANGLE_ALPHA_SET, 0.2f32);

        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_PLANE_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_FRUSTUM_PLANES_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(POINT_SUBTRACT, |e, a| {
            let (this, other) = (vec3(e, a[0]), vec3(e, a[2]));
            put3(
                e,
                a[1],
                [this[0] - other[0], this[1] - other[1], this[2] - other[2]],
            );
            ret(a[1])
        });
        e.register(POINT_ADD, |e, a| {
            let (this, other) = (vec3(e, a[0]), vec3(e, a[2]));
            put3(
                e,
                a[1],
                [this[0] + other[0], this[1] + other[1], this[2] + other[2]],
            );
            ret(a[1])
        });
        e.register(POINT_SCALE, |e, a| {
            let (this, scale) = (vec3(e, a[0]), f32::from_bits(a[2]));
            put3(e, a[1], [this[0] * scale, this[1] * scale, this[2] * scale]);
            ret(a[1])
        });
        e.register(POINT_SCALED_BY, |e, a| {
            let (scale, vector) = (f32::from_bits(a[1]), vec3(e, a[2]));
            put3(
                e,
                a[0],
                [vector[0] * scale, vector[1] * scale, vector[2] * scale],
            );
            ret(a[0])
        });
        e.register(POINT_NORMALIZE, |e, a| {
            let v = vec3(e, a[0]);
            let length = dot3(v, v).sqrt() as f32;
            put3(e, a[0], [v[0] / length, v[1] / length, v[2] / length]);
            Ret::default()
        });
        e.register(NI_POINT3_DOT, |e, a| {
            ret_float(dot3(vec3(e, a[0]), vec3(e, a[1])))
        });
        // Behind (2) when the signed distance is negative, in front (1)
        // otherwise.
        e.register(PLANE_SIDE_OF_POINT, |e, a| {
            let distance = dot3(vec3(e, a[0]), vec3(e, a[1])) - e.mem.f32(a[0] + 0x0c) as f64;
            ret(if distance < 0.0 { 2 } else { 1 })
        });
        e.register(NI_PLANE_FROM_POINTS, |e, a| {
            let (p0, p1, p2) = (vec3(e, a[1]), vec3(e, a[2]), vec3(e, a[3]));
            let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let length = dot3(n, n).sqrt() as f32;
            let n = [n[0] / length, n[1] / length, n[2] / length];
            put3(e, a[0], n);
            e.mem.set_f32(a[0] + 0x0c, dot3(n, p0) as f32);
            ret(a[0])
        });
        e.register(FABS, |_, a| ret_float(f64::take(a, &mut 0).abs()));
        e.register(COSINE, |_, a| ret_float(f64::take(a, &mut 0).cos()));
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let v = vec3(e, a[2]);
            let row = |e: &Engine, n: u32| vec3(e, a[0] + 12 * n);
            let result = [
                dot3(row(e, 0), v) as f32,
                dot3(row(e, 1), v) as f32,
                dot3(row(e, 2), v) as f32,
            ];
            put3(e, a[1], result);
            ret(a[1])
        });
        // Rotations only: the inverse is the transpose.
        e.register(MATRIX_INVERSE, |e, a| {
            for row in 0..3u32 {
                for column in 0..3u32 {
                    let value = e.mem.f32(a[0] + 4 * (3 * row + column));
                    e.mem.set_f32(a[1] + 4 * (3 * column + row), value);
                }
            }
            ret(1)
        });
        e
    }

    /// A plane at the origin: half extents 10 and 5, identity rotation, both
    /// caches dirty.
    fn flat_plane(e: &mut Engine) -> Plane {
        let plane: Plane = e.new_object();
        e.set(plane, BSOcclusionPlane::kHalfExtentX, 10.0);
        e.set(plane, BSOcclusionPlane::kHalfExtentY, 5.0);
        let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (n, value) in identity.iter().enumerate() {
            e.mem
                .set_f32(plane.addr() + ROTATION + 4 * n as u32, *value);
        }
        e.set(plane, BSOcclusionPlane::bDirtyVertices, true);
        e.set(plane, BSOcclusionPlane::bDirtyPlanes, true);
        plane
    }

    /// Stores the corners in the vertex cache and clears the dirty flag.
    fn cache_corners(e: &mut Engine, plane: Plane, corners: [[f32; 3]; 4]) {
        for (n, corner) in corners.iter().enumerate() {
            put3(e, plane.addr() + CACHED_VERTICES + 12 * n as u32, *corner);
        }
        e.set(plane, BSOcclusionPlane::bDirtyVertices, false);
    }

    /// A `NiFrustumPlanes` with the given planes (normal, constant) and mask.
    fn frustum(e: &mut Engine, planes: &[([f32; 3], f32)], mask: u32) -> Ptr {
        let block = Ptr::new(e.mem.alloc(NiFrustumPlanes::SIZE));
        for (n, (normal, constant)) in planes.iter().enumerate() {
            put3(e, plane_at(block, n as u32), *normal);
            e.mem.set_f32(plane_at(block, n as u32) + 0x0c, *constant);
        }
        e.mem.set_u32(block.addr() + ACTIVE_PLANES, mask);
        block
    }

    fn flags(e: &Engine, plane: Plane) -> [u8; 4] {
        [0, 1, 2, 3].map(|n| e.mem.u8(plane.addr() + 0xe6 + n))
    }

    /// Replaces the point-against-plane test with a table: `masks[c]` has a
    /// bit for every plane of the frustum at `planes` that corner `c`
    /// (recognised by its coordinates) is behind.
    fn table_side(e: &mut Engine, planes: Ptr, masks: [u32; 4]) {
        let base = planes.addr();
        e.register_double(PLANE_SIDE_OF_POINT, move |e, a| {
            let index = (a[0] - base) / PLANE_SIZE;
            let point = vec3(e, a[1]);
            let corner = CORNERS.iter().position(|c| *c == point).unwrap();
            ret(if masks[corner] & (1 << index) != 0 {
                2
            } else {
                1
            })
        });
    }

    /// Records the (end, start) coordinates of every subtraction.
    fn record_subtractions(e: &mut Engine) -> Subtractions {
        let calls: Subtractions = Rc::default();
        let sink = calls.clone();
        e.register_double(POINT_SUBTRACT, move |e, a| {
            let (this, other) = (vec3(e, a[0]), vec3(e, a[2]));
            sink.borrow_mut().push((this, other));
            put3(
                e,
                a[1],
                [this[0] - other[0], this[1] - other[1], this[2] - other[2]],
            );
            ret(a[1])
        });
        calls
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = test_engine();
        for (n, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.set_global(DEFAULT_CENTER + 4 * n as u32, *value);
        }
        e.set_global(DEFAULT_HALF_EXTENT, 50.0f32);
        for n in 0..9u32 {
            e.set_global(DEFAULT_ROTATION + 4 * n, n as f32);
        }
        let built: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = built.clone();
        e.register_double(NI_PLANE_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        let plane: Plane = e.new_object();
        e.set(plane, BSOcclusionPlane::kCullResult, 77);
        e.set(plane, BSOcclusionPlane::bTwoSided, false);
        e.mem.set_u32(plane.addr() + 0xf4, 0x1234);
        let back = e.call(0x00c3_35d0, &args![plane]).ptr::<BSOcclusionPlane>();
        assert_eq!(back, plane);
        assert_eq!(e.mem.u32(plane.addr()), BS_OCCLUSION_PLANE_VTABLE);
        let expected: Vec<u32> = (0..6).map(|n| plane.addr() + 0x44 + 0x10 * n).collect();
        assert_eq!(*built.borrow(), expected);
        assert_eq!(e.mem.u32(plane.addr() + 0x44 + 0x60), 0x3f);
        assert_eq!(vec3(&e, plane.addr() + 0x08), [1.0, 2.0, 3.0]);
        assert_eq!(e.get(plane, BSOcclusionPlane::kHalfExtentX), 50.0);
        assert_eq!(e.get(plane, BSOcclusionPlane::kHalfExtentY), 50.0);
        for n in 0..9u32 {
            assert_eq!(e.mem.f32(plane.addr() + ROTATION + 4 * n), n as f32);
        }
        assert_eq!(e.get(plane, BSOcclusionPlane::kCullResult), 0);
        assert!(e.get(plane, BSOcclusionPlane::bTwoSided));
        assert_eq!(e.mem.u32(plane.addr() + 0xf4), 0);
    }

    #[test]
    fn create_clone_builds_a_copy() {
        let mut e = test_engine();
        let source = flat_plane(&mut e);
        put3(&mut e, source.addr() + 0x08, [1.0, 2.0, 3.0]);
        e.set(source, BSOcclusionPlane::kCullResult, 1);
        e.set(source, BSOcclusionPlane::bDirtyVertices, false);
        e.set(source, BSOcclusionPlane::bDirtyPlanes, false);
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c3_3730, &args![source, 9u32])
            .ptr::<BSOcclusionPlane>();
        assert!(!clone.is_null());
        assert_ne!(clone, source);
        assert_eq!(e.mem.u32(clone.addr()), BS_OCCLUSION_PLANE_VTABLE);
        assert_eq!(vec3(&e, clone.addr() + 0x08), [1.0, 2.0, 3.0]);
        assert_eq!(e.get(clone, BSOcclusionPlane::kHalfExtentX), 10.0);
        assert_eq!(e.get(clone, BSOcclusionPlane::kHalfExtentY), 5.0);
        assert_eq!(e.get(clone, BSOcclusionPlane::kCullResult), 1);
        assert!(e.get(clone, BSOcclusionPlane::bDirtyVertices));
        assert!(e.get(clone, BSOcclusionPlane::bDirtyPlanes));
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0xfc])));
    }

    #[test]
    fn copy_members_copies_the_shape_and_marks_the_caches_dirty() {
        let mut e = test_engine();
        let source = flat_plane(&mut e);
        put3(&mut e, source.addr() + 0x08, [4.0, 5.0, 6.0]);
        e.set(source, BSOcclusionPlane::kCullResult, 2);
        e.mem.set_f32(source.addr() + ROTATION + 4 * 4, 0.5);
        let target: Plane = e.new_object();
        e.call(0x00c3_37c0, &args![source, target, 0u32]);
        assert_eq!(vec3(&e, target.addr() + 0x08), [4.0, 5.0, 6.0]);
        assert_eq!(e.get(target, BSOcclusionPlane::kHalfExtentX), 10.0);
        assert_eq!(e.get(target, BSOcclusionPlane::kHalfExtentY), 5.0);
        assert_eq!(e.mem.f32(target.addr() + ROTATION + 4 * 4), 0.5);
        assert_eq!(e.get(target, BSOcclusionPlane::kCullResult), 2);
        assert!(e.get(target, BSOcclusionPlane::bDirtyVertices));
        assert!(e.get(target, BSOcclusionPlane::bDirtyPlanes));
        // Not copied.
        assert!(!e.get(target, BSOcclusionPlane::bTwoSided));
    }

    #[test]
    fn get_vertices_builds_the_corners_and_caches_them() {
        let mut e = test_engine();
        let plane = flat_plane(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.call(0x00c3_5c10, &args![plane, out]);
        for (n, corner) in CORNERS.iter().enumerate() {
            assert_eq!(vec3(&e, out.addr() + 12 * n as u32), *corner);
        }
        assert!(!e.get(plane, BSOcclusionPlane::bDirtyVertices));
        assert_eq!(vec3(&e, plane.addr() + CACHED_VERTICES + 0x24), CORNERS[3]);

        // Clean: the cache is copied without arithmetic.
        put3(&mut e, plane.addr() + CACHED_VERTICES, [9.0, 8.0, 7.0]);
        let again: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.call_log = Some(vec![]);
        e.call(0x00c3_5c10, &args![plane, again]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == POINT_ADD));
        assert_eq!(vec3(&e, again.addr()), [9.0, 8.0, 7.0]);
    }

    #[test]
    fn get_vertices_rotates_the_axes_and_adds_the_centre() {
        let mut e = test_engine();
        let plane = flat_plane(&mut e);
        put3(&mut e, plane.addr() + 0x08, [1.0, 2.0, 3.0]);
        // Rows (0,0,1), (0,1,0), (-1,0,0): a = (10,0,0) becomes (0,0,-10) and
        // b = (0,0,5) becomes (5,0,0).
        let rotation = [0.0f32, 0.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0];
        for (n, value) in rotation.iter().enumerate() {
            e.mem
                .set_f32(plane.addr() + ROTATION + 4 * n as u32, *value);
        }
        let out: Ptr = Ptr::new(e.mem.alloc(0x30));
        e.call(0x00c3_5c10, &args![plane, out]);
        assert_eq!(vec3(&e, out.addr()), [6.0, 2.0, -7.0]);
        assert_eq!(vec3(&e, out.addr() + 0x0c), [-4.0, 2.0, -7.0]);
        assert_eq!(vec3(&e, out.addr() + 0x18), [-4.0, 2.0, 13.0]);
        assert_eq!(vec3(&e, out.addr() + 0x24), [6.0, 2.0, 13.0]);
    }

    #[test]
    fn get_frustum_planes_orders_the_points_by_the_side_of_the_viewer() {
        let run = |view: [f32; 3]| {
            let mut e = test_engine();
            let seen: Rc<RefCell<Vec<[[f32; 3]; 3]>>> = Rc::default();
            let sink = seen.clone();
            // Every plane comes out with the normal (0, 1, 0) and constant 0.
            e.register_double(NI_PLANE_FROM_POINTS, move |e, a| {
                sink.borrow_mut()
                    .push([vec3(e, a[1]), vec3(e, a[2]), vec3(e, a[3])]);
                put3(e, a[0], [0.0, 1.0, 0.0]);
                ret(a[0])
            });
            let plane = flat_plane(&mut e);
            let out: Ptr = Ptr::new(e.mem.alloc(NiFrustumPlanes::SIZE));
            e.call(0x00c3_3870, &args![plane, view[0], view[1], view[2], out]);
            let result = seen.borrow().clone();
            result
        };
        let (v0, v1, v2, v3) = (CORNERS[0], CORNERS[1], CORNERS[2], CORNERS[3]);
        // A viewer at y = 20 is in front of that plane (side 1).
        let view = [0.0, 20.0, 0.0];
        assert_eq!(
            run(view),
            vec![
                [v0, v1, v2],
                [v1, v0, v2],
                [v1, v0, view],
                [v3, v2, view],
                [v0, v3, view],
                [v2, v1, view],
            ]
        );
        let view = [0.0, -20.0, 0.0];
        assert_eq!(
            run(view),
            vec![
                [v0, v1, v2],
                [v0, v1, v2],
                [v0, v1, view],
                [v2, v3, view],
                [v3, v0, view],
                [v1, v2, view],
            ]
        );
    }

    #[test]
    fn get_frustum_planes_builds_caches_and_reuses() {
        let mut e = test_engine();
        let plane = flat_plane(&mut e);
        e.mem.set_u8(plane.addr() + 0xe8, 1);
        let out: Ptr = Ptr::new(e.mem.alloc(NiFrustumPlanes::SIZE));
        e.call(0x00c3_3870, &args![plane, 0.0f32, 20.0f32, 0.0f32, out]);
        // The plane through the corners faces up; the viewer is above it, so
        // plane 0 faces down. Plane 1 stays inactive, flag 2 switches off
        // plane 4.
        assert_eq!(vec3(&e, out.addr()), [0.0, -1.0, 0.0]);
        assert_eq!(e.mem.u32(out.addr() + ACTIVE_PLANES), 0x3f & !2 & !(1 << 4));
        assert!(!e.get(plane, BSOcclusionPlane::bDirtyPlanes));
        assert_eq!(vec3(&e, plane.addr() + 0xd8), [0.0, 20.0, 0.0]);
        assert_eq!(vec3(&e, plane.addr() + CACHED_PLANES), [0.0, -1.0, 0.0]);

        // Same viewpoint, clean planes: the cache is copied and no plane is
        // built.
        e.mem.set_f32(plane.addr() + CACHED_PLANES, 0.25);
        let again: Ptr = Ptr::new(e.mem.alloc(NiFrustumPlanes::SIZE));
        e.call_log = Some(vec![]);
        e.call(0x00c3_3870, &args![plane, 0.0f32, 20.0f32, 0.0f32, again]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(addr, _)| *addr == NI_PLANE_FROM_POINTS));
        assert_eq!(e.mem.f32(again.addr()), 0.25);

        // A moved viewpoint rebuilds.
        e.call_log = Some(vec![]);
        e.call(0x00c3_3870, &args![plane, 0.0f32, -20.0f32, 0.0f32, again]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(addr, _)| *addr == NI_PLANE_FROM_POINTS));
        assert_eq!(vec3(&e, again.addr()), [0.0, 1.0, 0.0]);
        assert_eq!(vec3(&e, plane.addr() + 0xd8), [0.0, -20.0, 0.0]);

        // Dirty planes rebuild even for the same viewpoint.
        e.set(plane, BSOcclusionPlane::bDirtyPlanes, true);
        e.call_log = Some(vec![]);
        e.call(0x00c3_3870, &args![plane, 0.0f32, -20.0f32, 0.0f32, again]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(addr, _)| *addr == NI_PLANE_FROM_POINTS));
    }

    #[test]
    fn viewer_on_correct_side_needs_the_viewer_behind_a_one_sided_plane() {
        let mut e = test_engine();
        let plane = flat_plane(&mut e);
        // Two sided: true without any work.
        e.set(plane, BSOcclusionPlane::bTwoSided, true);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x00c3_58a0, &args![plane, 0.0f32, 20.0f32, 0.0f32])
            .bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        // One sided: the plane through the corners faces up (+y).
        e.set(plane, BSOcclusionPlane::bTwoSided, false);
        assert!(!e
            .call(0x00c3_58a0, &args![plane, 0.0f32, 20.0f32, 0.0f32])
            .bool());
        assert!(e
            .call(0x00c3_58a0, &args![plane, 0.0f32, -20.0f32, 0.0f32])
            .bool());
    }

    #[test]
    fn joined_plane_type_reads_the_linked_planes() {
        let mut e = test_engine();
        let plane: Plane = e.new_object();
        assert_eq!(e.call(0x00c3_5980, &args![plane]).i32(), 0);
        e.mem.set_u32(plane.addr() + 0xf8, 1);
        assert_eq!(e.call(0x00c3_5980, &args![plane]).i32(), 1);
        e.mem.set_u32(plane.addr() + 0xf0, 1);
        assert_eq!(e.call(0x00c3_5980, &args![plane]).i32(), 1);
        e.mem.set_u32(plane.addr() + 0xf4, 1);
        assert_eq!(e.call(0x00c3_5980, &args![plane]).i32(), 2);
        e.mem.set_u32(plane.addr() + 0xf4, 0);
        e.mem.set_u32(plane.addr() + 0xec, 1);
        assert_eq!(e.call(0x00c3_5980, &args![plane]).i32(), 2);
    }

    /// A segment from `start` to `end` in two heap blocks.
    fn segment(e: &mut Engine, start: [f32; 3], end: [f32; 3]) -> (Ptr, Ptr) {
        let (a, b) = (Ptr::new(e.mem.alloc(12)), Ptr::new(e.mem.alloc(12)));
        put3(e, a.addr(), start);
        put3(e, b.addr(), end);
        (a, b)
    }

    fn single_plane(e: &mut Engine, normal: [f32; 3], constant: f32) -> Ptr {
        let block = Ptr::new(e.mem.alloc(PLANE_SIZE));
        put3(e, block.addr(), normal);
        e.mem.set_f32(block.addr() + 0x0c, constant);
        block
    }

    #[test]
    fn crossing_inside_the_frustum_planes() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        let (start, end) = segment(&mut e, [0.0, -1.0, 0.0], [0.0, 1.0, 0.0]);
        let crossed = single_plane(&mut e, [0.0, 1.0, 0.0], 0.0);
        // The crossing point is (0, 0, 0), 10 in front of x = -10.
        let inside = frustum(&mut e, &[([1.0, 0.0, 0.0], -10.0)], 1);
        assert!(e
            .call(0x00c3_5ae0, &args![this, start, end, crossed, inside])
            .bool());
        // 10 behind x = 10.
        let outside = frustum(&mut e, &[([1.0, 0.0, 0.0], 10.0)], 1);
        assert!(!e
            .call(0x00c3_5ae0, &args![this, start, end, crossed, outside])
            .bool());
        // An inactive plane is not asked.
        let masked = frustum(&mut e, &[([1.0, 0.0, 0.0], 10.0)], 0);
        assert!(e
            .call(0x00c3_5ae0, &args![this, start, end, crossed, masked])
            .bool());
        // The crossing parameter has to be within 0 and 1.
        let far = single_plane(&mut e, [0.0, 1.0, 0.0], 5.0);
        assert!(!e
            .call(0x00c3_5ae0, &args![this, start, end, far, inside])
            .bool());
        // A segment parallel to the plane never crosses it.
        let (flat_start, flat_end) = segment(&mut e, [0.0, 1.0, 0.0], [3.0, 1.0, 0.0]);
        assert!(!e
            .call(
                0x00c3_5ae0,
                &args![this, flat_start, flat_end, crossed, inside]
            )
            .bool());
    }

    #[test]
    fn crossing_inside_two_planes() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        let (start, end) = segment(&mut e, [0.0, -1.0, 0.0], [0.0, 1.0, 0.0]);
        let crossed = single_plane(&mut e, [0.0, 1.0, 0.0], 0.0);
        let first = single_plane(&mut e, [1.0, 0.0, 0.0], -10.0);
        let second = single_plane(&mut e, [0.0, 0.0, 1.0], -10.0);
        assert!(e
            .call(
                0x00c3_59d0,
                &args![this, start, end, crossed, first, second]
            )
            .bool());
        let behind = single_plane(&mut e, [0.0, 0.0, 1.0], 10.0);
        assert!(!e
            .call(
                0x00c3_59d0,
                &args![this, start, end, crossed, first, behind]
            )
            .bool());
        assert!(!e
            .call(
                0x00c3_59d0,
                &args![this, start, end, crossed, behind, second]
            )
            .bool());
        let far = single_plane(&mut e, [0.0, 1.0, 0.0], -5.0);
        assert!(!e
            .call(0x00c3_59d0, &args![this, start, end, far, first, second])
            .bool());
        let (flat_start, flat_end) = segment(&mut e, [0.0, 1.0, 0.0], [3.0, 1.0, 0.0]);
        assert!(!e
            .call(
                0x00c3_59d0,
                &args![this, flat_start, flat_end, crossed, first, second]
            )
            .bool());
    }

    #[test]
    fn within_frustum_rejects_a_plane_with_every_corner_behind() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        // x - 100 < 0 for every corner.
        let planes = frustum(&mut e, &[([1.0, 0.0, 0.0], 100.0)], 1);
        e.mem.set_u8(this.addr() + 0xe6, 9);
        let result = e.call(
            0x00c3_3cf0,
            &args![this, planes, 1u8, 0.0f32, 0.0f32, 0.0f32],
        );
        assert!(!result.bool());
        // The flags are only written once every plane has been tested.
        assert_eq!(e.mem.u8(this.addr() + 0xe6), 9);
    }

    #[test]
    fn within_frustum_accepts_when_no_pair_of_flags_matches() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        // Corners 2 and 3 (x = -10) are behind plane 0, corners 0 and 1 in
        // front: only the flag for the pair 3-2 is set.
        let planes = frustum(&mut e, &[([1.0, 0.0, 0.0], 0.0)], 1);
        let result = e.call(
            0x00c3_3cf0,
            &args![this, planes, 0u8, 0.0f32, 0.0f32, 0.0f32],
        );
        assert!(result.bool());
        assert_eq!(flags(&e, this), [0, 1, 0, 0]);
    }

    /// Masks (the planes each corner is behind) that give the four pair
    /// flags in the order the code stores them; the first segment tested
    /// (end, start) tells which case ran.
    fn pair_flag_case(masks: [u32; 4], expected_flags: [u8; 4], first_edge: ([f32; 3], [f32; 3])) {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        // Three active planes with zero normals: no segment ever crosses them.
        let planes = frustum(&mut e, &[], 0b111);
        table_side(&mut e, planes, masks);
        let calls = record_subtractions(&mut e);
        let result = e.call(
            0x00c3_3cf0,
            &args![this, planes, 0u8, 0.0f32, 0.0f32, 0.0f32],
        );
        assert_eq!(flags(&e, this), expected_flags);
        // Nothing crosses and the extents are not asked for: false.
        assert!(!result.bool());
        assert_eq!(calls.borrow()[0], first_edge);
    }

    #[test]
    fn within_frustum_checks_the_missing_edge_of_a_nearly_complete_set() {
        let (v0, v1, v2, v3) = (CORNERS[0], CORNERS[1], CORNERS[2], CORNERS[3]);
        // Pair 0-1 missing: edge 0 to 1.
        pair_flag_case([0b001, 0b010, 0b110, 0b101], [0, 1, 1, 1], (v1, v0));
        // Pair 3-2 missing: edge 3 to 2.
        pair_flag_case([0b011, 0b101, 0b100, 0b010], [1, 0, 1, 1], (v2, v3));
        // Pair 0-3 missing: edge 0 to 3.
        pair_flag_case([0b011, 0b101, 0b100, 0b100], [1, 1, 0, 1], (v3, v0));
        // Pair 2-1 missing: edge 2 to 1.
        pair_flag_case([0b011, 0b001, 0b100, 0b110], [1, 1, 1, 0], (v1, v2));
        // All four pairs: no edge test, straight to the diagonal 0 to 2.
        pair_flag_case([0b011, 0b101, 0b110, 0b110], [1, 1, 1, 1], (v2, v0));
    }

    /// Corners 0 and 3 are behind plane 0 (z > 0), corners 1 and 2 behind
    /// plane 1 (z < 0), corners 2 and 3 behind plane 2 (x below the
    /// constant).
    fn three_plane_frustum(e: &mut Engine, third_constant: f32) -> Ptr {
        frustum(
            e,
            &[
                ([0.0, 0.0, -1.0], 0.0),
                ([0.0, 0.0, 1.0], 0.0),
                ([1.0, 0.0, 0.0], third_constant),
            ],
            0b111,
        )
    }

    #[test]
    fn within_frustum_finds_a_diagonal_through_the_frustum() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        let planes = three_plane_frustum(&mut e, 0.0);
        let result = e.call(
            0x00c3_3cf0,
            &args![this, planes, 0u8, 0.0f32, 0.0f32, 0.0f32],
        );
        // Pair flags (0,1,1,1); the edge 0-1 is parallel to the third plane
        // and the diagonal 0-2 crosses it at the origin, inside all three.
        assert_eq!(flags(&e, this), [0, 1, 1, 1]);
        assert!(result.bool());
    }

    #[test]
    fn within_frustum_falls_back_to_the_extents_test() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        // With the third plane at x = 5 the diagonals cross outside the
        // frustum.
        let planes = three_plane_frustum(&mut e, 5.0);
        let call = |e: &mut Engine, extents: u8, view: [f32; 3]| {
            e.call(
                0x00c3_3cf0,
                &args![this, planes, extents, view[0], view[1], view[2]],
            )
            .bool()
        };
        assert!(!call(&mut e, 0, [3.0, 2.0, 0.0]));
        // Offset (3, 2, 0) from the centre: |3| <= 10 and |2| <= 5.
        assert!(call(&mut e, 1, [3.0, 2.0, 0.0]));
        // Too far along the first rotation row, or the second.
        assert!(!call(&mut e, 1, [30.0, 2.0, 0.0]));
        assert!(!call(&mut e, 1, [3.0, 100.0, 0.0]));
    }

    #[test]
    fn full_test_records_every_pair_and_reports_a_blocking_plane() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        let planes = frustum(&mut e, &[([1.0, 0.0, 0.0], 0.0)], 1);
        assert!(e.call(0x00c3_4350, &args![this, planes]).bool());
        assert_eq!(flags(&e, this), [0, 1, 0, 0]);

        // Plane 1 has every corner behind it; the pair flags still come from
        // both planes: every pair shares plane 1.
        let blocking = frustum(
            &mut e,
            &[([1.0, 0.0, 0.0], 0.0), ([1.0, 0.0, 0.0], 100.0)],
            0b11,
        );
        assert!(!e.call(0x00c3_4350, &args![this, blocking]).bool());
        assert_eq!(flags(&e, this), [1, 1, 1, 1]);
        // Without active planes everything passes and no flag is set.
        let none = frustum(&mut e, &[], 0);
        assert!(e.call(0x00c3_4350, &args![this, none]).bool());
        assert_eq!(flags(&e, this), [0, 0, 0, 0]);
    }

    #[test]
    fn fn_00c347d0_needs_every_corner_in_front_of_every_active_plane() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        let all_in_front = frustum(&mut e, &[([1.0, 0.0, 0.0], -100.0)], 1);
        assert!(e.call(0x00c3_47d0, &args![this, all_in_front]).bool());
        let one_corner_behind = frustum(&mut e, &[([1.0, 0.0, 0.0], 0.0)], 1);
        assert!(!e.call(0x00c3_47d0, &args![this, one_corner_behind]).bool());
        let inactive = frustum(&mut e, &[([1.0, 0.0, 0.0], 0.0)], 0);
        assert!(e.call(0x00c3_47d0, &args![this, inactive]).bool());
    }

    /// Gives the plane's vtable a `GetRTTI` (slot 8) that answers the
    /// occlusion plane's information block, or something else.
    fn set_rtti(e: &mut Engine, plane: Plane, exact: bool) {
        e.put_vtable(FAKE_VTABLE, &[0, 0, RTTI_TARGET]);
        e.mem.set_u32(plane.addr(), FAKE_VTABLE);
        if exact {
            e.register(RTTI_TARGET, |_, _| ret(OCCLUSION_PLANE_RTTI));
        } else {
            e.register(RTTI_TARGET, |_, _| ret(0x1234));
        }
    }

    #[test]
    fn fn_00c344c0_answers_from_the_class_when_the_point_is_over_the_rectangle() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        // The point is 3 above the plane and over (2, 1).
        let point: Ptr = Ptr::new(e.mem.alloc(12));
        put3(&mut e, point.addr(), [2.0, 3.0, 1.0]);
        // Plane 0 contributes 3 - 3 = 0: the sum is 3, inside -5 to 5.
        let planes = frustum(&mut e, &[([0.0, 1.0, 0.0], 3.0)], 0);
        e.mem.set_u8(this.addr() + 0xe6, 9);
        set_rtti(&mut e, this, true);
        assert!(e.call(0x00c3_44c0, &args![this, planes, point]).bool());
        set_rtti(&mut e, this, false);
        assert!(!e.call(0x00c3_44c0, &args![this, planes, point]).bool());
        // WithinFrustum did not run.
        assert_eq!(e.mem.u8(this.addr() + 0xe6), 9);
    }

    #[test]
    fn fn_00c344c0_falls_back_to_within_frustum() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        set_rtti(&mut e, this, false);
        let point: Ptr = Ptr::new(e.mem.alloc(12));
        put3(&mut e, point.addr(), [2.0, 3.0, 1.0]);
        // The sum is 26: out of range, so WithinFrustum runs (it accepts:
        // there are no active planes) and writes the flags.
        let far = frustum(&mut e, &[([0.0, 1.0, 0.0], -20.0)], 0);
        e.mem.set_u8(this.addr() + 0xe6, 9);
        assert!(e.call(0x00c3_44c0, &args![this, far, point]).bool());
        assert_eq!(flags(&e, this), [0, 0, 0, 0]);

        // In range but over x = 20: outside the half extent, same fallback.
        put3(&mut e, point.addr(), [20.0, 3.0, 1.0]);
        let near = frustum(&mut e, &[([0.0, 1.0, 0.0], 3.0)], 0);
        e.mem.set_u8(this.addr() + 0xe6, 9);
        assert!(e.call(0x00c3_44c0, &args![this, near, point]).bool());
        assert_eq!(flags(&e, this), [0, 0, 0, 0]);

        // The centre's y moved to -20: outside the second half extent.
        put3(&mut e, point.addr(), [2.0, 3.0, 1.0]);
        e.mem.set_f32(this.addr() + 0x0c, -20.0);
        e.mem.set_u8(this.addr() + 0xe6, 9);
        assert!(e.call(0x00c3_44c0, &args![this, near, point]).bool());
        assert_eq!(flags(&e, this), [0, 0, 0, 0]);
    }

    #[test]
    fn test_intersection_checks_the_sphere_against_the_rectangle() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        let bound: Ptr = Ptr::new(e.mem.alloc(16));
        let check = |e: &mut Engine, centre: [f32; 3], radius: f32| {
            put3(e, bound.addr(), centre);
            e.mem.set_f32(bound.addr() + 0x0c, radius);
            e.call(0x00c3_4890, &args![this, bound]).bool()
        };
        assert!(check(&mut e, [3.0, 2.0, 1.0], 3.0));
        // Past the first half extent.
        assert!(!check(&mut e, [30.0, 0.0, 0.0], 3.0));
        // Past the second half extent (along the third axis).
        assert!(!check(&mut e, [0.0, 0.0, 30.0], 3.0));
        // Too far from the plane.
        assert!(!check(&mut e, [0.0, 10.0, 0.0], 3.0));
        // The final test compares the plane-normal component with the second
        // half extent: with it at 3, a sphere reaching 3.48 along the third
        // axis still passes.
        e.set(this, BSOcclusionPlane::kHalfExtentY, 3.0);
        assert!(check(&mut e, [0.0, 0.5, 5.5], 3.0));
    }

    #[test]
    fn test_intersection_uses_the_cleared_matrix_when_inversion_fails() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        e.register(MATRIX_INVERSE, |_, _| ret(0));
        let cleared: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = cleared.clone();
        e.register_double(MATRIX_CLEAR, move |e, a| {
            sink.borrow_mut().push(a[0]);
            for n in 0..9 {
                e.mem.set_f32(a[0] + 4 * n, 0.0);
            }
            Ret::default()
        });
        let bound: Ptr = Ptr::new(e.mem.alloc(16));
        // Far outside the rectangle, but the zero matrix maps it to the origin.
        put3(&mut e, bound.addr(), [30.0, 0.0, 0.0]);
        e.mem.set_f32(bound.addr() + 0x0c, 3.0);
        assert!(e.call(0x00c3_4890, &args![this, bound]).bool());
        assert_eq!(cleared.borrow().len(), 1);
    }

    /// A fake line or triangle geometry: the object whose +0xB8 points at its
    /// data, which holds the vertex (+0x20), colour (+0x28) and, for lines,
    /// flag (+0x40) arrays.
    fn geometry(e: &mut Engine, vertices: u32) -> (Ptr, u32, u32, u32) {
        let object = Ptr::new(e.mem.alloc(0x100));
        let data = e.mem.alloc(0x50);
        let positions = e.mem.alloc(12 * vertices);
        let colors = e.mem.alloc(16 * vertices);
        let line_flags = e.mem.alloc(2 * vertices);
        e.mem.set_u32(object.addr() + 0xb8, data);
        e.mem.set_u32(data + 0x20, positions);
        e.mem.set_u32(data + 0x28, colors);
        e.mem.set_u32(data + 0x40, line_flags);
        (object, positions, colors, line_flags)
    }

    fn color4(e: &Engine, at: u32) -> [f32; 4] {
        [0, 1, 2, 3].map(|n| e.mem.f32(at + 4 * n))
    }

    #[test]
    fn make_debug_geom_fills_the_lines_and_triangles() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        e.set(this, BSOcclusionPlane::kCullResult, 1);
        e.mem.set_u8(this.addr() + 0xe6, 1);
        set_rtti(&mut e, this, true);
        let (lines, line_positions, line_colors, line_flags) = geometry(&mut e, 16);
        let (triangles, tri_positions, tri_colors, _) = geometry(&mut e, 30);
        e.register(TRI_LIST, |_, _| ret(FAKE_INDEX_LIST));
        e.map(FAKE_INDEX_LIST, 0x1000);
        e.call_log = Some(vec![]);
        e.call(
            0x00c3_4ad0,
            &args![
                this, 0.0f32, 100.0f32, 0.0f32, lines, triangles, 0.2f32, 0.3f32, 0.4f32, 0.9f32,
                1u32
            ],
        );
        let log = e.call_log.take().unwrap();
        // Four far corners: each a normalised direction scaled by 2500.
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == POINT_NORMALIZE)
                .count(),
            4
        );
        let first_scale = log.iter().find(|(addr, _)| *addr == POINT_SCALE).unwrap();
        assert_eq!(f32::from_bits(first_scale.1[2]), 2500.0);

        // Lines: corner pairs (0,1), (3,2), (0,3), (2,1).
        let pairs = [(0, 1), (3, 2), (0, 3), (2, 1)];
        for (line, (a, b)) in pairs.iter().enumerate() {
            let slot = 2 * line as u32;
            assert_eq!(vec3(&e, line_positions + 12 * slot), CORNERS[*a]);
            assert_eq!(vec3(&e, line_positions + 12 * (slot + 1)), CORNERS[*b]);
            let expected = if line == 0 {
                [1.0, 1.0, 0.1, 0.9]
            } else {
                [0.1, 1.0, 0.1, 0.9]
            };
            assert_eq!(color4(&e, line_colors + 16 * slot), expected);
            assert_eq!(color4(&e, line_colors + 16 * (slot + 1)), expected);
            assert_eq!(e.mem.u8(line_flags + slot), 1);
            assert_eq!(e.mem.u8(line_flags + slot + 1), 0);
        }
        // The exact class adds two marker lines along the plane's own axis
        // (second rotation row, scaled by 30) through corners 0 and 3.
        let marker = [0.9, 0.9, 0.9, 0.5];
        let expected = [
            ([10.0, 30.0, 5.0], [10.0, -30.0, 5.0]),
            ([-10.0, 30.0, 5.0], [-10.0, -30.0, 5.0]),
        ];
        for (n, (up, down)) in expected.iter().enumerate() {
            let slot = 8 + 2 * n as u32;
            assert_eq!(vec3(&e, line_positions + 12 * slot), *up);
            assert_eq!(vec3(&e, line_positions + 12 * (slot + 1)), *down);
            assert_eq!(color4(&e, line_colors + 16 * slot), marker);
            assert_eq!(color4(&e, line_colors + 16 * (slot + 1)), marker);
            assert_eq!(e.mem.u8(line_flags + slot), 1);
            assert_eq!(e.mem.u8(line_flags + slot + 1), 0);
        }
        for slot in 12..16u32 {
            assert_eq!(e.mem.u8(line_flags + slot), 0);
        }

        // Triangles (0,1,2) and (2,3,0) in the given colour, alpha 0.2.
        let triangle_corners = [0, 1, 2, 2, 3, 0];
        for (slot, corner) in triangle_corners.iter().enumerate() {
            assert_eq!(vec3(&e, tri_positions + 12 * slot as u32), CORNERS[*corner]);
            assert_eq!(
                color4(&e, tri_colors + 16 * slot as u32),
                [0.2, 0.3, 0.4, 0.2]
            );
        }
        // The index list counts up for two triangles, then repeats the last
        // index for the other eight.
        let indices: Vec<u16> = (0..30)
            .map(|n| e.mem.u16(FAKE_INDEX_LIST + 2 * n))
            .collect();
        assert_eq!(&indices[..6], &[0, 1, 2, 3, 4, 5]);
        assert!(indices[6..].iter().all(|index| *index == 5));
    }

    #[test]
    fn make_debug_geom_without_the_exact_class_or_cull_result() {
        let mut e = test_engine();
        let this = flat_plane(&mut e);
        cache_corners(&mut e, this, CORNERS);
        set_rtti(&mut e, this, false);
        let (lines, line_positions, line_colors, line_flags) = geometry(&mut e, 16);
        let (triangles, _, tri_colors, _) = geometry(&mut e, 30);
        e.register(TRI_LIST, |_, _| ret(FAKE_INDEX_LIST));
        e.map(FAKE_INDEX_LIST, 0x1000);
        // Pre-set the flags the code has to write or clear.
        for slot in 0..16u32 {
            e.mem.set_u8(line_flags + slot, 7);
        }
        e.call(
            0x00c3_4ad0,
            &args![
                this, 0.0f32, 100.0f32, 0.0f32, lines, triangles, 0.2f32, 0.3f32, 0.4f32, 0.9f32,
                0u32
            ],
        );
        // Not cull result 1: the dim red colour, whatever the flags say.
        assert_eq!(color4(&e, line_colors), [0.7, 0.15, 0.15, 0.9]);
        assert_eq!(vec3(&e, line_positions), CORNERS[0]);
        // The four edge lines are on, the rest (no marker lines) off.
        for slot in 0..8u32 {
            assert_eq!(e.mem.u8(line_flags + slot), (slot % 2 == 0) as u8);
        }
        for slot in 8..16u32 {
            assert_eq!(e.mem.u8(line_flags + slot), 0);
        }
        // The last argument clear: triangle alpha 0.1.
        assert_eq!(color4(&e, tri_colors), [0.2, 0.3, 0.4, 0.1]);
    }
}
