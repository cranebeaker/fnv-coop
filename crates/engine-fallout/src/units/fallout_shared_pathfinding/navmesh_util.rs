//! `fallout shared/pathfinding/navmesh_util.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the navmesh's spatial helpers: the search grid that lists,
//! per cell, the triangles touching it, the point-to-triangle queries that
//! use it, edge/segment intersection tests, and a few small value-type
//! helpers (`NiPoint2`, a two-point segment, a path request's copy
//! operators). Nearly all of them are methods of `NavMesh`
//! ([`crate::units::fallout_shared_pathfinding::navmesh::NavMesh`], `this`
//! is the mesh) or of its embedded search grid at `NavMesh + 0xA8`
//! ([`NavMeshGrid`], `this` is the grid).
//!
//! Earlier batches fully translated 110 of the 113 entries. The remaining
//! three now have normal-path translations; compiler exception-unwind cleanup
//! is still partial in each. The PC disassembly also fixes the temporary map
//! constructor's immediate vtable address used by the topology repair.
//!
// translated: 006987d0 - normal-path obstacle-footprint scan; exception unwind omitted.
// translated: 00699b60 - normal-path edge-pair repair and nearest-vertex selection; exception unwind omitted.
// translated: 006bd040 - normal-path body; exception unwind omitted.
//
//! Conventions in this file:
//! - Methods the engine map leaves unnamed are `fn_<address>`, named by what
//!   the body does in their doc comment.
//! - `006815c0` is the `NiPoint3` default constructor: its body only returns
//!   `this`, and it is called on stack locals that are written before they
//!   are read, so those calls (and the `_vector_constructor_iterator_`
//!   (`00401050`) over such locals) are left out. The compiler's
//!   exception-unwinding frames and stack-cookie checks are not translated.
//! - `float` stores round to `f32`; the x87 intermediate results are
//!   computed in `f64` (see the guide).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmesh::{NavMesh, NavMeshGrid, NavMeshTriangle};

// ---------------------------------------------------------------------------
// Layouts and constants
// ---------------------------------------------------------------------------

/// Offset of the search grid (`MeshGrid`, a [`NavMeshGrid`]) inside a
/// `NavMesh`.
pub(crate) const MESH_GRID_OFFSET: u32 = 0xa8;
/// Offset of the `Triangles` array inside a `NavMesh`.
const MESH_TRIANGLES_OFFSET: u32 = 0x38;
/// Offset of the `ExtraEdgeInfo` array inside a `NavMesh`.
const MESH_EXTRA_EDGE_INFO_OFFSET: u32 = 0x48;
/// PC `00699b60`: the map of vertex index to packed edge pair and the array
/// of chosen vertex indices. The Xbox PDB labels at these offsets disagree
/// with the PC container operations, so these roles follow the PC body.
const MESH_EDGE_PAIR_MAP_OFFSET: u32 = 0x88;
const MESH_PREFERRED_VERTEX_ARRAY_OFFSET: u32 = 0x98;
/// `POVVertexData` (Xbox PDB), 0x28 bytes: ordinary and portal edge arrays.
/// PC `00699d20` and `00699f07` append through these offsets.
const POV_EDGES_OFFSET: u32 = 8;
const POV_PORTAL_EDGES_OFFSET: u32 = 0x18;
/// `NavMeshInfo::AdjacentNavMeshes` and `PreferredAdjacentNavMeshes`
/// (`BSSimpleArray<NavMeshInfo*>`, Xbox PDB) inside a `NavMeshInfo`.
const INFO_ADJACENT_NAV_MESHES: u32 = 0x24;
const INFO_PREFERRED_ADJACENT_NAV_MESHES: u32 = 0x34;
/// `EdgeExtraInfo::Portal.pOtherMesh` (Xbox PDB), the `NavMeshInfo*` of the
/// mesh on the other side of a portal edge.
const EDGE_INFO_OTHER_MESH: u32 = 0x04;

/// The `TES` instance pointer (`TES::GetNavMeshInfoMap` is called on it).
const TES_INSTANCE: u32 = 0x011d_ea10;
/// Type descriptors `__RTDynamicCast` casts between (`.?AVTESForm@@`,
/// `.?AVNavMesh@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_NAV_MESH: u32 = 0x0118_b6b4;
/// Virtual slot of `NavMesh` (byte offset) called with `1` before the extra
/// edge infos of a loaded mesh are repaired.
const NAV_MESH_SLOT_0XC8: u32 = 0xc8;

// Data the code reads from the exe.
/// `FLT_MAX` (`float`) and its negation.
const FLOAT_MAX: u32 = 0x0101_6970;
const NEGATIVE_FLOAT_MAX: u32 = 0x0101_5f5c;
/// `180.0f`, `64.0f` (used as the up and down tolerance of the triangle
/// height tests) and `0.5f`.
const HEIGHT_UP: u32 = 0x0106_b158;
const HEIGHT_DOWN: u32 = 0x0101_e6ec;
const HALF: u32 = 0x0101_6248;
/// `double` constants: `0.0`, `50.0`, `-180.0`, `64.0`, `40.0`, `180.0`.
const ZERO: u32 = 0x0101_2060;
const GROUND_MARGIN: u32 = 0x0101_e2c0;
const NEGATIVE_180: u32 = 0x0106_b160;
const SIXTY_FOUR: u32 = 0x0102_40c0;
const FORTY: u32 = 0x0103_5810;
const ONE_EIGHTY: u32 = 0x0104_ed58;
/// `0.5` (`double`), the numerator of the circumcenter's reciprocal.
const DOUBLE_HALF: u32 = 0x0101_1588;
/// Three `float`s: the vector the random-point search starts from (its `z`
/// is kept unchanged).
const RANDOM_POINT_BASE: u32 = 0x011f_426c;

// Functions of other units this one calls.
/// `BSSimpleArray::iSize` (`this`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Address of element `index` of a `BSSimpleArray` of 4-byte elements.
const ARRAY_ELEMENT_4: u32 = 0x0087_7a30;
/// Address of element `index` of a `BSSimpleArray<unsigned short>`.
const ARRAY_ELEMENT_2: u32 = 0x0069_dec0;
/// `BSSimpleArray` emptiness test (`iSize == 0`).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// Address of triangle `index` of the `Triangles` array (`this` is the array).
const TRIANGLES_ELEMENT: u32 = 0x0055_8dd0;
/// `BSSimpleArray::Remove(index, count)` (`this`, index, count).
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// Resets the array to the given size (`this`, size, keep buffer) and
/// reserves room (`this`, count).
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
/// Appends a `u16` (read through the pointer) to a `BSSimpleArray<unsigned
/// short>`.
const ARRAY_ADD_WORD: u32 = 0x0069_1820;
/// Appends a `u32` (read through the pointer) to a `BSSimpleArray<unsigned
/// int>`.
const ARRAY_ADD_DWORD: u32 = 0x007c_b2e0;
/// Index of the first element equal to the key (`this`, key pointer, start,
/// comparator) and the removal of `count` elements from `index`
/// (`this`, index, count) of a `BSSimpleArray<unsigned short>`.
const ARRAY_FIND_WORD: u32 = 0x0069_b240;
const WORD_COMPARATOR: u32 = 0x0069_b2a0;
/// `0069b1b0(a, b)`: whether the 16-bit words at `a` and `b` are equal (the comparator
/// `006987d0` passes for its triangle indices, 00698aa6).
const WORD_EQUALS_COMPARATOR: u32 = 0x0069_b1b0;
const WORD_ARRAY_REMOVE: u32 = 0x0069_add0;
/// `NiPointer` helpers: default constructor, constructor from a pointer
/// (cdecl: out, pointer; returns out), assignment, destructor, the held
/// pointer (`NiPointer::operator*`) and the word at `this`.
const POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const POINTER_CONSTRUCT_FROM_CDECL: u32 = 0x0046_4fc0;
const POINTER_ASSIGN: u32 = 0x0042_f4c0;
const POINTER_DESTRUCT: u32 = 0x0042_fa40;
const POINTER_DEREFERENCE: u32 = 0x0045_8b50;
const POINTER_GET: u32 = 0x0055_9450;
/// Looks a form up by its id (cdecl) and `__RTDynamicCast` (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TES::GetNavMeshInfoMap`.
const GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// Constructor and destructor of the `BSSimpleArray<NavMeshInfo*>` (vtable
/// `0106b1a4`) that collects the infos, and the map method that fills it.
const INFO_LIST_CONSTRUCTOR: u32 = 0x0069_aee0;
const INFO_LIST_DESTRUCTOR: u32 = 0x0069_af10;
const INFO_MAP_COLLECT: u32 = 0x006b_7910;
/// `NavMeshInfo` method: assigns `pNavMesh` to the `NiPointer<NavMesh>`
/// argument, true when it is non-null (`navmesh.cpp`).
const INFO_GET_NAV_MESH: u32 = 0x0069_0860;
/// `NavMesh::GetExtraEdgeInfo(index)`: the `EdgeExtraInfo` at `index` of the
/// extra info array.
const GET_EXTRA_EDGE_INFO: u32 = 0x0069_3d30;
/// `NavMesh::DeleteExtraEdgeInfo` (`this`, index).
const DELETE_EXTRA_EDGE_INFO: u32 = 0x0069_0e90;
/// `NavMesh::GetEdgeExtraInfo` (`this`, triangle, edge): null without info.
const GET_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `NavMesh::GetEdgeVertices` (`this`, out pair of vertex pointers,
/// triangle, edge).
const GET_EDGE_VERTICES: u32 = 0x0068_f040;
/// Appends an `EdgeExtraInfo` to the mesh's array, returns its index.
const ADD_EXTRA_EDGE_INFO: u32 = 0x0069_1020;
/// `EdgeExtraInfo` constructor (`this`).
const EDGE_INFO_CONSTRUCTOR: u32 = 0x0069_23c0;
/// `NavMesh::GetTriangleCount`, `NavMesh::GetTriangle(index)`,
/// `NavMeshTriangle::GetVertex(i)` and `NavMesh::GetCenter(out, triangle)`.
const TRIANGLE_COUNT: u32 = 0x0055_8200;
const GET_TRIANGLE: u32 = 0x0055_82f0;
const TRIANGLE_VERTEX: u32 = 0x0055_82d0;
const GET_CENTER: u32 = 0x0055_8220;
/// Address of vertex `index` of the mesh.
const VERTEX_POINTER: u32 = 0x0068_f0a0;
/// `NavMeshTriangle` methods: whether edge `edge` has extra info, whether
/// its neighbour slot is not `0xFFFF`, the neighbour slot, whether any edge
/// has extra info, a flag mask test, and the setters of a vertex, of an
/// edge's extra info slot and of an edge's plain neighbour.
const EDGE_HAS_INFO: u32 = 0x0068_f1d0;
const EDGE_HAS_NEIGHBOUR: u32 = 0x0068_f200;
const EDGE_SLOT: u32 = 0x0068_f2c0;
const ANY_EDGE_HAS_INFO: u32 = 0x0069_0e10;
const TRIANGLE_FLAGS_SET: u32 = 0x0069_1140;
const SET_VERTEX: u32 = 0x0069_0df0;
const SET_EDGE_INFO_SLOT: u32 = 0x0069_0e30;
const SET_EDGE_NEIGHBOUR: u32 = 0x0069_07d0;
/// Unpacks the six edge bits the triangle flags hold for an edge (`this`,
/// edge, out value, out flag, out flag) and stores them again (`this`, edge,
/// value, flag, flag).
const UNPACK_EDGE_BITS: u32 = 0x0069_1040;
const PACK_EDGE_BITS: u32 = 0x006a_d2f0;
/// Search grid methods in other files: `Clear`, setup from the mesh bounds
/// and triangle count (`this`, min, max, count), the cell list that holds a
/// point (`this`, point, out list pointer), the corner point of a cell
/// (`this`, out, x, y), the cell number/row/column of a point (`this`,
/// point, out row, out column).
const GRID_CLEAR: u32 = 0x0069_d2d0;
const GRID_SETUP: u32 = 0x0069_d800;
const GRID_CELL_LIST: u32 = 0x0069_d370;
const GRID_CELL_CORNER: u32 = 0x0069_d790;
const GRID_CELL_OF_POINT: u32 = 0x0069_12a0;
/// Forwarded to by `00696c00` (`this`, three words).
const GRID_FORWARDED: u32 = 0x0069_d4b0;
/// `floor` of a `float` (cdecl, result in ST0) and `fabs` of a `float`.
const FLOAT_FLOOR: u32 = 0x0040_6ce0;
const FLOAT_ABS: u32 = 0x0040_8840;
/// `NiPoint2(x, y)` constructor (`this`, x, y) and the 2D segment test of
/// `006ba010` (cdecl, four `NiPoint2*`).
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const SEGMENTS_INTERSECT_2D: u32 = 0x006b_a010;
/// `NiPoint3(x, y, z)` constructor (`this`, x, y, z).
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiPoint3` arithmetic: `this + other`, `this - other` (both: `this`,
/// out, other), `this * scalar` (`this`, out, scalar), `this += other`,
/// `this == other`, `this x other` (`this`, out, other), normalize in
/// place, squared length and squared length of x and y (result in ST0).
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_SCALE: u32 = 0x0045_bb20;
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
const POINT3_EQUALS: u32 = 0x0043_90c0;
const POINT3_CROSS: u32 = 0x004b_3800;
const POINT3_NORMALIZE: u32 = 0x004a_0c10;
const POINT3_SQUARED_LENGTH: u32 = 0x004a_7290;
const POINT3_SQUARED_LENGTH_2D: u32 = 0x0059_5c80;
/// `NiPoint3::UnitCross` (`this`, out, other).
const POINT3_UNIT_CROSS: u32 = 0x0053_d1a0;
/// Copy of a 16-byte structure (`this`, other): the base of the request
/// copy operators.
const COPY_STRUCT_16: u32 = 0x004a_3c90;
/// Height of the ground under a point in a cell (`this` is the cell, point,
/// out float).
const CELL_GROUND_HEIGHT: u32 = 0x0054_7250;
/// Random `float` in a range (cdecl: min, max) and the square root of a
/// `float` (cdecl, result in ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
const FLOAT_SQRT: u32 = 0x0040_19b0;
/// Ray test of a point against a triangle's three vertices (cdecl: three
/// vertex pointers, point, out float, two floats), the 2D test of two
/// segments (cdecl: four words and an out pointer) and the distance of a
/// point to a segment (cdecl: point, first end, second end; `float`).
const TRIANGLE_RAY_TEST: u32 = 0x006b_9cf0;
const SEGMENT_SEGMENT_TEST: u32 = 0x006b_9e90;
const SEGMENT_INTERSECTION_2D: u32 = 0x006b_a240;
const POINT_SEGMENT_DISTANCE: u32 = 0x006b_9940;
/// The `_vector_constructor_iterator_` (cdecl: array, element size, count,
/// constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// Constructor of the 16-byte element of the two-element array `006985c0`
/// builds.
const ELEMENT_16_CONSTRUCTOR: u32 = 0x006d_5320;
/// `NiPoint3` default constructor (`this`; only returns it).
const POINT3_DEFAULT_CONSTRUCTOR: u32 = 0x0068_15c0;

/// Vtables of the obstacle-query arrays/maps assembled by the functions
/// below (PC `FalloutNV.exe` 1.4.0.525).
const NAV_MESH_INFO_POINTER_ARRAY_VTABLE: u32 = 0x0106_b1a4;
const NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE: u32 = 0x0106_b1b8;
const BOOLEAN_ARRAY_VTABLE: u32 = 0x0106_b1cc;
const FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE: u32 = 0x0106_b1e0;
const FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE: u32 = 0x0106_b1f4;

/// "No triangle" in a 16-bit triangle index.
const NO_TRIANGLE: u16 = 0xffff;

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// `BSSimpleArray::iSize` (`0044ddc0`).
fn array_size<T>(e: &mut Engine, array: Ptr<T>) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// `NavMesh::GetTriangle(index)` (`005582f0`); the index is a `u16`.
fn triangle_of(e: &mut Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
    e.call(GET_TRIANGLE, &args![mesh, index as u16]).ptr()
}

/// `NavMeshTriangle::GetVertex(corner)` (`005582d0`).
fn corner_vertex_index(e: &mut Engine, triangle: Ptr<NavMeshTriangle>, corner: u32) -> u16 {
    e.call(TRIANGLE_VERTEX, &args![triangle, corner]).u16()
}

/// Address of vertex `index` of the mesh (`0068f0a0`).
fn vertex_pointer(e: &mut Engine, mesh: Ptr<NavMesh>, index: u16) -> Ptr {
    e.call(VERTEX_POINTER, &args![mesh, index]).ptr()
}

/// Address of corner `corner` of triangle `triangle`.
fn corner_pointer(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    triangle: Ptr<NavMeshTriangle>,
    corner: u32,
) -> Ptr {
    let index = corner_vertex_index(e, triangle, corner);
    vertex_pointer(e, mesh, index)
}

/// The three words at `address` as three `float`s.
fn read_point(e: &Engine, address: u32) -> [f32; 3] {
    [
        e.mem.f32(address),
        e.mem.f32(address + 4),
        e.mem.f32(address + 8),
    ]
}

fn write_point(e: &mut Engine, address: u32, point: [f32; 3]) {
    e.mem.set_f32(address, point[0]);
    e.mem.set_f32(address + 4, point[1]);
    e.mem.set_f32(address + 8, point[2]);
}

/// The search grid embedded in the mesh.
fn mesh_grid(mesh: Ptr<NavMesh>) -> Ptr<NavMeshGrid> {
    mesh.byte_add(MESH_GRID_OFFSET).cast()
}

/// The `Triangles` array embedded in the mesh.
fn mesh_triangles(mesh: Ptr<NavMesh>) -> Ptr {
    mesh.byte_add(MESH_TRIANGLES_OFFSET)
}

/// Adds one triangle-edge handle to the temporary record keyed by a vertex.
/// A new record gets the 6-byte (mesh, vertex) `key` copied into its first
/// six bytes before it is inserted. The map owns its nodes; the record
/// pointer is destroyed after the repair scan, before the map itself is
/// destroyed.
fn append_vertex_edge(
    e: &mut Engine,
    map: Ptr,
    mesh: Ptr<NavMesh>,
    vertex: u16,
    key: Ptr,
    edge_handle: Ptr,
    portal: bool,
) {
    e.with_stack(4, |e, found| {
        let mut record = Ptr::<()>::new(0);
        if fn_0069b4d0(e, map, mesh.addr(), vertex as u32, found) {
            record = Ptr::new(e.mem.u32(found.addr()));
        } else {
            let allocated = e.call(0x0040_1000, &args![0x28u32]).u32();
            if allocated != 0 {
                record = fn_0069a730(e, Ptr::new(allocated));
            }
            // 00699cea-00699cf1 (and 00699dc9, 00699ed1, 00699fb0): ECX = the
            // record, push &key, CALL 0069a690 copies the key into record+0.
            // The exe makes this call without a null test.
            e.call(0x0069_a690, &args![record, key]);
            fn_0069b3d0(e, map, mesh.addr(), vertex as u32, record.addr());
        }
        let offset = if portal {
            POV_PORTAL_EDGES_OFFSET
        } else {
            POV_EDGES_OFFSET
        };
        fn_0069b310(e, record.byte_add(offset), edge_handle);
    });
}

/// The condition `FCOMP` leaves for `TEST AH, mask`: bit C0 (`st0 < operand`),
/// C2 (unordered) and C3 (equal) of the x87 status word as `0x01`, `0x04`
/// and `0x40`.
fn compare_flags(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0
    }
}

/// `TEST AH, mask` followed by `JP` (jump on even parity of the result).
fn parity_even(flags: u8, mask: u8) -> bool {
    (flags & mask).count_ones() % 2 == 0
}

/// Truncation of `value` as `FISTP qword` (rounding control: truncate) and
/// the low 32 bits of the result; an out-of-range value or a NaN gives the
/// "integer indefinite" `0x8000000000000000`.
fn float_to_integer_low(value: f32) -> u32 {
    let value = value as f64;
    if value.is_nan() || value.abs() >= 9.223_372_036_854_776e18 {
        0
    } else {
        value as i64 as u32
    }
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

// Translated from 006953b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FixPortalsFromNavMeshDeletion` (Xbox PDB; cdecl): the mesh
/// info `deleted_info` is going away. For every `NavMeshInfo` the info map
/// holds, every entry of its adjacent and preferred-adjacent lists that is
/// `deleted_info` is removed, and, when that info's navmesh is loaded (it is
/// looked up by its form id when `pNavMesh` is null), the navmesh's extra
/// edge infos whose portal leads to `deleted_info` are deleted too
/// (`NavMesh::DeleteExtraEdgeInfo`, after a virtual call on the mesh with `1`).
/// Does nothing for a null `deleted_info`.
pub fn nav_mesh_fix_portals_from_nav_mesh_deletion(e: &mut Engine, deleted_info: Ptr) {
    if deleted_info.is_null() {
        return;
    }
    e.with_stack(0x10, |e, list| {
        e.call(INFO_LIST_CONSTRUCTOR, &args![list]);
        let tes: u32 = e.global(TES_INSTANCE);
        let map = e.call(GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
        e.call(INFO_MAP_COLLECT, &args![map, list]);
        let mut index = 0u32;
        while index < array_size(e, list) {
            let slot = e.call(ARRAY_ELEMENT_4, &args![list, index]).u32();
            let info = e.mem.u32(slot);
            let adjacent = Ptr::<()>::new(info + INFO_ADJACENT_NAV_MESHES);
            let preferred = Ptr::<()>::new(info + INFO_PREFERRED_ADJACENT_NAV_MESHES);
            let mut adjacent_count = array_size(e, adjacent) as i32;
            let mut preferred_count = array_size(e, preferred) as i32;
            let mut entry = 0i32;
            while entry < adjacent_count.wrapping_add(preferred_count) {
                let slot = if entry < adjacent_count {
                    e.call(ARRAY_ELEMENT_4, &args![adjacent, entry]).u32()
                } else {
                    e.call(
                        ARRAY_ELEMENT_4,
                        &args![preferred, entry.wrapping_sub(adjacent_count)],
                    )
                    .u32()
                };
                if e.mem.u32(slot) == deleted_info.addr() {
                    e.with_stack(4, |e, mesh_pointer| {
                        e.call(POINTER_CONSTRUCT, &args![mesh_pointer]);
                        let loaded = e
                            .call(
                                INFO_GET_NAV_MESH,
                                &args![Ptr::<()>::new(info), mesh_pointer],
                            )
                            .bool();
                        if !loaded {
                            let form_id = e.call(POINTER_GET, &args![Ptr::<()>::new(info)]).u32();
                            let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
                            let cast = e
                                .call(
                                    RT_DYNAMIC_CAST,
                                    &args![form, 0i32, TYPE_TES_FORM, TYPE_NAV_MESH, 0i32],
                                )
                                .u32();
                            e.with_stack(4, |e, temporary| {
                                let made = e
                                    .call(POINTER_CONSTRUCT_FROM_CDECL, &args![temporary, cast])
                                    .u32();
                                e.call(POINTER_ASSIGN, &args![mesh_pointer, made]);
                                e.call(POINTER_DESTRUCT, &args![temporary]);
                            });
                        }
                        if e.call(POINTER_DEREFERENCE, &args![mesh_pointer]).u32() != 0 {
                            let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                            e.vcall(mesh, NAV_MESH_SLOT_0XC8, &args![1u32]);
                            let mut extra = 0u32;
                            loop {
                                let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                let count = array_size(
                                    e,
                                    Ptr::<()>::new(mesh + MESH_EXTRA_EDGE_INFO_OFFSET),
                                );
                                if extra >= count {
                                    break;
                                }
                                let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                let edge_info = e
                                    .call(GET_EXTRA_EDGE_INFO, &args![mesh, extra & 0xffff])
                                    .u32();
                                if e.mem.u32(edge_info + EDGE_INFO_OTHER_MESH)
                                    == deleted_info.addr()
                                {
                                    let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                    e.call(DELETE_EXTRA_EDGE_INFO, &args![mesh, extra & 0xffff]);
                                    extra = extra.wrapping_sub(1);
                                }
                                extra = extra.wrapping_add(1);
                            }
                        }
                        if entry < adjacent_count {
                            e.call(ARRAY_REMOVE, &args![adjacent, entry, 1u32]);
                            adjacent_count -= 1;
                        } else {
                            e.call(
                                ARRAY_REMOVE,
                                &args![preferred, entry.wrapping_sub(adjacent_count), 1u32],
                            );
                            preferred_count -= 1;
                        }
                        entry -= 1;
                        e.call(POINTER_DESTRUCT, &args![mesh_pointer]);
                    });
                }
                entry += 1;
            }
            index += 1;
        }
        e.call(INFO_LIST_DESTRUCTOR, &args![list]);
    });
}

// Translated from 00695660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips triangle `triangle` of the mesh: vertices 0 and 1 are swapped, and
/// so are edges 0 and 1 (the plain neighbour or extra info slot, the extra
/// info flag and, when the triangle carries edge bits (`006957d0`), the six
/// packed edge bits of each edge).
pub fn fn_00695660(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16) {
    let tri = triangle_of(e, this, triangle as u32);
    let second_vertex = corner_vertex_index(e, tri, 1);
    let first_vertex = corner_vertex_index(e, tri, 0);
    e.call(SET_VERTEX, &args![tri, 1u32, first_vertex]);
    e.call(SET_VERTEX, &args![tri, 0u32, second_vertex]);
    let first_has_info = e.call(EDGE_HAS_INFO, &args![tri, 0u32]).bool();
    let second_has_info = e.call(EDGE_HAS_INFO, &args![tri, 1u32]).bool();
    let second_slot = e.call(EDGE_SLOT, &args![tri, 1u32]).u16();
    let first_slot = e.call(EDGE_SLOT, &args![tri, 0u32]).u16();
    if first_has_info {
        e.call(SET_EDGE_INFO_SLOT, &args![tri, 1u32, first_slot]);
    } else {
        e.call(SET_EDGE_NEIGHBOUR, &args![tri, 1u32, first_slot]);
    }
    if second_has_info {
        e.call(SET_EDGE_INFO_SLOT, &args![tri, 0u32, second_slot]);
    } else {
        e.call(SET_EDGE_NEIGHBOUR, &args![tri, 0u32, second_slot]);
    }
    if fn_006957d0(e, tri) {
        // Value word, then the two flag bytes, of edge 0 and edge 1.
        e.with_stack(16, |e, scratch| {
            let (value0, flag0_a, flag0_b) = (scratch, scratch.byte_add(4), scratch.byte_add(5));
            let (value1, flag1_a, flag1_b) = (
                scratch.byte_add(8),
                scratch.byte_add(12),
                scratch.byte_add(13),
            );
            e.call(
                UNPACK_EDGE_BITS,
                &args![tri, 0u32, value0, flag0_a, flag0_b],
            );
            e.call(
                UNPACK_EDGE_BITS,
                &args![tri, 1u32, value1, flag1_a, flag1_b],
            );
            let (v0, a0, b0) = (
                e.mem.u16(value0.addr()),
                e.mem.u8(flag0_a.addr()),
                e.mem.u8(flag0_b.addr()),
            );
            let (v1, a1, b1) = (
                e.mem.u16(value1.addr()),
                e.mem.u8(flag1_a.addr()),
                e.mem.u8(flag1_b.addr()),
            );
            e.call(PACK_EDGE_BITS, &args![tri, 0u32, v1, b1, a1]);
            e.call(PACK_EDGE_BITS, &args![tri, 1u32, v0, b0, a0]);
        });
    }
}

// Translated from 006957d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether any of the packed edge bits (flags
/// `0x0FFF0000`) is set.
pub fn fn_006957d0(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & 0x0fff_0000 != 0
}

// Translated from 006957f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounding box of the mesh's vertices as seen through its triangles:
/// stores the smallest x, y, z at `min` and the largest at `max` (three
/// floats each). Does nothing when the mesh has no triangles.
pub fn fn_006957f0(e: &mut Engine, this: Ptr<NavMesh>, min: Ptr, max: Ptr) {
    if e.call(ARRAY_IS_EMPTY, &args![mesh_triangles(this)]).bool() {
        return;
    }
    let first = triangle_of(e, this, 0);
    let start = corner_pointer(e, this, first, 0);
    let mut high = read_point(e, start.addr());
    let mut low = high;
    let mut index = 0u32;
    while index < e.call(TRIANGLE_COUNT, &args![this]).u32() {
        let tri = triangle_of(e, this, index);
        for corner in 0..3 {
            let vertex = corner_pointer(e, this, tri, corner);
            let vertex = read_point(e, vertex.addr());
            for axis in 0..3 {
                if low[axis] > vertex[axis] {
                    low[axis] = vertex[axis];
                } else if high[axis] < vertex[axis] {
                    high[axis] = vertex[axis];
                }
            }
        }
        index += 1;
    }
    write_point(e, min.addr(), low);
    write_point(e, max.addr(), high);
}

// Translated from 006959b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the mesh's search grid: clears it, and for a mesh with triangles
/// sizes it from the mesh bounds (`006957f0`, `0069d800`) and then lists
/// every triangle in every grid cell it touches. A triangle is listed in a
/// cell when
/// - all its vertices fall in one cell, or
/// - the cell is within the triangle's cell rectangle and one of the
///   vertices' cells, or one vertex lies inside the cell (the second
///   vertex is tested against `Min.y <= third vertex y`, as the game does),
///   or the ray test `00697730` hits one of the cell's four corners, or one
///   of the cell's four sides crosses one of the triangle's three edges
///   (`00696540`).
///
/// The per-triangle minimum and maximum x and y the code also computes are
/// never used and are left out; so is the "listed" flag it keeps.
pub fn fn_006959b0(e: &mut Engine, this: Ptr<NavMesh>) {
    let triangle_count = e.call(TRIANGLE_COUNT, &args![this]).u32() as i32;
    let grid = mesh_grid(this);
    e.call(GRID_CLEAR, &args![grid]);
    if triangle_count == 0 {
        return;
    }
    // 0: bounds min, 12: bounds max, 24: the four cell corners (12 bytes
    // each), 72: a corner read from the grid, 84: the height out parameter,
    // 96: the four points of the two segments (cell side, triangle edge).
    e.with_stack(144, |e, scratch| {
        let bounds_min = scratch;
        let bounds_max = scratch.byte_add(12);
        let corner_slots = [24u32, 36, 48, 60].map(|offset| scratch.byte_add(offset));
        let corner_out = scratch.byte_add(72);
        let height_out = scratch.byte_add(84);
        let segment_slots = [96u32, 108, 120, 132].map(|offset| scratch.byte_add(offset));

        fn_006957f0(e, this, bounds_min, bounds_max);
        e.call(
            GRID_SETUP,
            &args![grid, bounds_min, bounds_max, triangle_count],
        );
        let size = e.call(POINTER_GET, &args![grid]).u32();
        let column_length = e.get(grid, NavMeshGrid::fColumnSectionLen);
        let row_length = e.get(grid, NavMeshGrid::fRowSectionLen);
        let min_x = e.mem.f32(bounds_min.addr());
        let min_y = e.mem.f32(bounds_min.addr() + 4);
        let ray_limit: f32 = e.global(FLOAT_MAX);
        let ray_height: f32 = e.global(HEIGHT_UP);

        for index in 0..triangle_count {
            let triangle = index as u16 as u32;
            let tri = triangle_of(e, this, index as u32);
            let mut vertices = [[0f32; 3]; 3];
            let mut cells = [[0u32; 2]; 3];
            for corner in 0..3usize {
                let vertex = corner_pointer(e, this, tri, corner as u32);
                let vertex = read_point(e, vertex.addr());
                vertices[corner] = vertex;
                let column_position =
                    ((vertex[0] as f64 - min_x as f64) / column_length as f64) as f32;
                let floored = e.call(FLOAT_FLOOR, &args![column_position]).f32();
                cells[corner][0] = float_to_integer_low(floored);
                let row_position = ((vertex[1] as f64 - min_y as f64) / row_length as f64) as f32;
                let floored = e.call(FLOAT_FLOOR, &args![row_position]).f32();
                cells[corner][1] = float_to_integer_low(floored);
                if cells[corner][0] >= size {
                    cells[corner][0] = size.wrapping_sub(1);
                }
                if cells[corner][1] >= size {
                    cells[corner][1] = size.wrapping_sub(1);
                }
            }
            let (mut cell_min_x, mut cell_max_x) = (cells[0][0], cells[0][0]);
            let (mut cell_min_y, mut cell_max_y) = (cells[0][1], cells[0][1]);
            for cell in &cells[1..] {
                cell_min_x = cell_min_x.min(cell[0]);
                cell_max_x = cell_max_x.max(cell[0]);
                cell_min_y = cell_min_y.min(cell[1]);
                cell_max_y = cell_max_y.max(cell[1]);
            }
            if cell_min_x == cell_max_x && cell_min_y == cell_max_y {
                fn_00696500(e, grid, triangle, cell_min_x, cell_min_y);
                continue;
            }
            let mut cell_x = cell_min_x;
            while cell_x <= cell_max_x {
                let mut cell_y = cell_min_y;
                while cell_y <= cell_max_y {
                    let vertex_cell = cells.iter().any(|c| c[0] == cell_x && c[1] == cell_y);
                    if vertex_cell {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    // The cell's corners: (x, y), (x + 1, y), (x, y + 1),
                    // (x + 1, y + 1).
                    let mut corners = [[0f32; 3]; 4];
                    for (n, (dx, dy)) in [(0u32, 0u32), (1, 0), (0, 1), (1, 1)]
                        .into_iter()
                        .enumerate()
                    {
                        let corner = e
                            .call(
                                GRID_CELL_CORNER,
                                &args![
                                    grid,
                                    corner_out,
                                    cell_x.wrapping_add(dx),
                                    cell_y.wrapping_add(dy)
                                ],
                            )
                            .u32();
                        corners[n] = read_point(e, corner);
                        write_point(e, corner_slots[n].addr(), corners[n]);
                    }
                    let (low, high) = (corners[0], corners[3]);
                    let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
                    let inside = |v: [f32; 3], y: f32| {
                        v[0] <= high[0] && v[1] <= high[1] && low[0] <= v[0] && low[1] <= y
                    };
                    // The second vertex is tested against its own x and y
                    // but the third vertex's y for the low side.
                    let accepted = inside(v0, v0[1]) || inside(v1, v2[1]) || inside(v2, v2[1]);
                    if accepted {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    let mut hit = false;
                    for slot in corner_slots {
                        if fn_00697730(
                            e,
                            this,
                            triangle as u16,
                            slot,
                            height_out,
                            ray_limit,
                            ray_height,
                        ) {
                            hit = true;
                            break;
                        }
                    }
                    if hit {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    // Cell sides in the order bottom, right, top, left;
                    // triangle edges 0-1, 1-2, 2-0.
                    let sides = [(0usize, 1usize), (1, 3), (3, 2), (2, 0)];
                    let edges = [(0usize, 1usize), (1, 2), (2, 0)];
                    let mut crossing = false;
                    'sides: for (from, to) in sides {
                        write_point(e, segment_slots[0].addr(), corners[from]);
                        write_point(e, segment_slots[1].addr(), corners[to]);
                        for (edge_from, edge_to) in edges {
                            write_point(e, segment_slots[2].addr(), vertices[edge_from]);
                            write_point(e, segment_slots[3].addr(), vertices[edge_to]);
                            if fn_00696540(
                                e,
                                segment_slots[0],
                                segment_slots[1],
                                segment_slots[2],
                                segment_slots[3],
                            ) {
                                crossing = true;
                                break 'sides;
                            }
                        }
                    }
                    if crossing {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                    }
                    cell_y += 1;
                }
                cell_x += 1;
            }
        }
    });
}

// Translated from 00696500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: appends `triangle` (as a `u16`) to the triangle
/// list of cell `(x, y)`, cell number `y * iGridSize + x`.
pub fn fn_00696500(e: &mut Engine, this: Ptr<NavMeshGrid>, triangle: u32, x: u32, y: u32) {
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cell_number = y.wrapping_mul(size).wrapping_add(x);
    let cells = e.get(this, NavMeshGrid::GridData);
    let cell = cells.addr().wrapping_add(cell_number << 4);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), triangle);
        e.call(ARRAY_ADD_WORD, &args![Ptr::<()>::new(cell), slot]);
    });
}

// Translated from 00696540 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl: whether the 2D segments `a`-`b` and `c`-`d` intersect. Each point
/// (three floats; only x and y are used) is copied into a `NiPoint2` and the
/// four go to `006ba010`.
pub fn fn_00696540(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> bool {
    e.with_stack(32, |e, base| {
        // The copies are built d first, then c, b and a; the frame holds a at
        // EBP-0x20, b at -0x18, c at -0x10 and d at -0x08.
        let mut copies = [0u32; 4];
        for (slot, point) in [(3u32, d), (2, c), (1, b), (0, a)] {
            // 00696546..00696556 (and the three like it): `FLD`/`FSTP float` push y,
            // then x (a signalling NaN goes quiet).
            let y = x87_quiet(e.mem.f32(point.addr() + 4));
            let x = x87_quiet(e.mem.f32(point.addr()));
            // 00696561 (and the three like it): `PUSH EAX`, the constructor's result.
            copies[slot as usize] = e
                .call(POINT2_CONSTRUCT, &args![base.byte_add(8 * slot), x, y])
                .u32();
        }
        e.call(
            SEGMENTS_INTERSECT_2D,
            &args![copies[0], copies[1], copies[2], copies[3]],
        )
        .bool()
    })
}

// Translated from 006965d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lists triangles `first` to `last` (inclusive) in the search grid: for
/// each, finds the smallest and largest x and y of its vertices, converts
/// both to grid rows and columns (`006912a0`) and appends the triangle to
/// every cell of that rectangle. Returns true.
pub fn fn_006965d0(e: &mut Engine, this: Ptr<NavMesh>, first: u16, last: u16) -> bool {
    let grid = mesh_grid(this);
    let mut triangle = first as u32;
    while triangle <= last as u32 {
        let tri = triangle_of(e, this, triangle);
        let (low, high) = triangle_xy_bounds(e, this, tri);
        e.with_stack(48, |e, scratch| {
            let low_point = scratch;
            let high_point = scratch.byte_add(12);
            write_point(e, low_point.addr(), low);
            write_point(e, high_point.addr(), high);
            let (row_min, column_min) = (scratch.byte_add(24), scratch.byte_add(28));
            let (row_max, column_max) = (scratch.byte_add(32), scratch.byte_add(36));
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, low_point, row_min, column_min],
            );
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, high_point, row_max, column_max],
            );
            let (row_min, column_min) = (e.mem.u32(row_min.addr()), e.mem.u32(column_min.addr()));
            let (row_max, column_max) = (e.mem.u32(row_max.addr()), e.mem.u32(column_max.addr()));
            let mut row = row_min;
            while row <= row_max {
                let mut column = column_min;
                while column <= column_max {
                    let cell = fn_006967c0(e, grid, column, row);
                    if !cell.is_null() {
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u16(slot.addr(), triangle as u16);
                            e.call(ARRAY_ADD_WORD, &args![cell, slot]);
                        });
                    }
                    column = column.wrapping_add(1);
                }
                row = row.wrapping_add(1);
            }
        });
        triangle += 1;
    }
    true
}

/// Smallest and largest x and y of a triangle's three vertices (z is 0). The
/// minimum starts at `FLT_MAX` and the maximum at `-FLT_MAX`; each vertex
/// widens them with a strict comparison (`NiPoint3(x, y, 0)` constructor
/// `00416870`).
fn triangle_xy_bounds(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    tri: Ptr<NavMeshTriangle>,
) -> ([f32; 3], [f32; 3]) {
    let flt_max: f32 = e.global(FLOAT_MAX);
    let negative_flt_max: f32 = e.global(NEGATIVE_FLOAT_MAX);
    let (low, high) = e.with_stack(24, |e, scratch| {
        let high_slot = scratch.byte_add(12);
        e.call(POINT3_CONSTRUCT, &args![scratch, flt_max, flt_max, 0.0f32]);
        e.call(
            POINT3_CONSTRUCT,
            &args![high_slot, negative_flt_max, negative_flt_max, 0.0f32],
        );
        for corner in 0..3 {
            let vertex = corner_pointer(e, mesh, tri, corner);
            let vertex = read_point(e, vertex.addr());
            let low = read_point(e, scratch.addr());
            let high = read_point(e, high_slot.addr());
            if low[0] > vertex[0] {
                e.mem.set_f32(scratch.addr(), vertex[0]);
            }
            if high[0] < vertex[0] {
                e.mem.set_f32(high_slot.addr(), vertex[0]);
            }
            if low[1] > vertex[1] {
                e.mem.set_f32(scratch.addr() + 4, vertex[1]);
            }
            if high[1] < vertex[1] {
                e.mem.set_f32(high_slot.addr() + 4, vertex[1]);
            }
        }
        (
            read_point(e, scratch.addr()),
            read_point(e, high_slot.addr()),
        )
    });
    (low, high)
}

// Translated from 006967c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: the address of the triangle list of cell
/// `(x, y)`: `GridData + (y * iGridSize + x) * 16`.
pub fn fn_006967c0(e: &mut Engine, this: Ptr<NavMeshGrid>, x: u32, y: u32) -> Ptr {
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cell_number = y.wrapping_mul(size).wrapping_add(x);
    let cells = e.get(this, NavMeshGrid::GridData);
    Ptr::new((cell_number << 4).wrapping_add(cells.addr()))
}

// Translated from 006967f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `006965d0`: removes triangles `first` to `last` (inclusive)
/// from every grid cell of their rectangle (the triangle is looked up in
/// the cell's list with `0069b240` and removed with `0069add0`). Returns
/// true.
pub fn fn_006967f0(e: &mut Engine, this: Ptr<NavMesh>, first: u16, last: u16) -> bool {
    let grid = mesh_grid(this);
    let mut triangle = first as u32;
    while triangle <= last as u32 {
        let tri = triangle_of(e, this, triangle);
        let (low, high) = triangle_xy_bounds(e, this, tri);
        e.with_stack(48, |e, scratch| {
            let low_point = scratch;
            let high_point = scratch.byte_add(12);
            write_point(e, low_point.addr(), low);
            write_point(e, high_point.addr(), high);
            let (row_min, column_min) = (scratch.byte_add(24), scratch.byte_add(28));
            let (row_max, column_max) = (scratch.byte_add(32), scratch.byte_add(36));
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, low_point, row_min, column_min],
            );
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, high_point, row_max, column_max],
            );
            let (row_min, column_min) = (e.mem.u32(row_min.addr()), e.mem.u32(column_min.addr()));
            let (row_max, column_max) = (e.mem.u32(row_max.addr()), e.mem.u32(column_max.addr()));
            let mut row = row_min;
            while row <= row_max {
                let mut column = column_min;
                while column <= column_max {
                    let cell = fn_006967c0(e, grid, column, row);
                    e.with_stack(4, |e, key| {
                        e.mem.set_u32(key.addr(), triangle);
                        let index = e
                            .call(ARRAY_FIND_WORD, &args![cell, key, 0u32, WORD_COMPARATOR])
                            .u32();
                        e.call(WORD_ARRAY_REMOVE, &args![cell, index, 1u32]);
                    });
                    column = column.wrapping_add(1);
                }
                row = row.wrapping_add(1);
            }
        });
        triangle += 1;
    }
    true
}

// Translated from 006969e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the search grid's bounds to `min` and `max` (`00696a10` on the
/// embedded grid). Returns true.
pub fn fn_006969e0(e: &mut Engine, this: Ptr<NavMesh>, min: Ptr, max: Ptr) -> bool {
    fn_00696a10(e, mesh_grid(this), min, max);
    true
}

// Translated from 00696a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: copies `GridBoundsMin` to the three floats at `min`
/// and `GridBoundsMax` to those at `max`.
pub fn fn_00696a10(e: &mut Engine, this: Ptr<NavMeshGrid>, min: Ptr, max: Ptr) {
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x0c + 4 * word);
        e.mem.set_u32(min.addr() + 4 * word, value);
    }
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x18 + 4 * word);
        e.mem.set_u32(max.addr() + 4 * word, value);
    }
}

// Translated from 00696a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The triangle under the point at `point` (x, y, z floats): looks up the
/// grid cell's triangle list and returns the index of the triangle whose
/// surface is nearest in height (`00697810`, with `callback`, an optional
/// object whose virtual slot 0 is told about each hit), or `0xFFFF`.
/// Triangles with flag `0x20` are skipped; when the point is not more than
/// 50 above the ground of the mesh's parent cell (`00547250`) heights below
/// `-180` are accepted too, otherwise not; heights above `64` are never
/// accepted. A triangle within `40` returns at once, otherwise the nearest
/// wins.
pub fn fn_00696a50(e: &mut Engine, this: Ptr<NavMesh>, point: Ptr, callback: Ptr) -> u16 {
    e.with_stack(12, |e, scratch| {
        let list_slot = scratch;
        let ground_slot = scratch.byte_add(4);
        let height_slot = scratch.byte_add(8);
        let grid = mesh_grid(this);
        e.call(GRID_CELL_LIST, &args![grid, point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            return NO_TRIANGLE;
        }
        let count = array_size(e, Ptr::<()>::new(list));
        let mut best_height: f32 = e.global(FLOAT_MAX);
        let ground: f32 = e.global(NEGATIVE_FLOAT_MAX);
        e.mem.set_f32(ground_slot.addr(), ground);
        let cell = e.get(this, NavMesh::pParentCell);
        if !cell.is_null() {
            e.call(CELL_GROUND_HEIGHT, &args![cell, point, ground_slot]);
        }
        let margin: f64 = e.global(GROUND_MARGIN);
        let threshold = (e.mem.f32(ground_slot.addr()) as f64 + margin) as f32;
        let point_z = e.mem.f32(point.addr() + 8);
        // `FCOMPP` then `JP`: the point is below the threshold.
        let near_ground = point_z < threshold;
        let mut best = NO_TRIANGLE;
        let mut index = 0u32;
        while index < count {
            let triangle = e
                .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                .ptr::<()>();
            let triangle = e.mem.u16(triangle.addr());
            index += 1;
            let tri = e
                .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
                .ptr::<NavMeshTriangle>();
            if e.call(TRIANGLE_FLAGS_SET, &args![tri, 0x20u32]).bool() {
                continue;
            }
            if !fn_00697810(e, this, triangle, point, height_slot, callback) {
                continue;
            }
            let height = e.mem.f32(height_slot.addr());
            if !near_ground {
                let below: f64 = e.global(NEGATIVE_180);
                if (height as f64) < below {
                    continue;
                }
            }
            let above: f64 = e.global(SIXTY_FOUR);
            if height as f64 > above {
                continue;
            }
            let height = e.call(FLOAT_ABS, &args![height]).f32();
            e.mem.set_f32(height_slot.addr(), height);
            if best_height.partial_cmp(&height) != Some(std::cmp::Ordering::Greater) {
                continue;
            }
            let close: f64 = e.global(FORTY);
            if (height as f64) < close {
                return triangle;
            }
            best = triangle;
            best_height = height;
        }
        best
    })
}

// Translated from 00696c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards three words to `0069d4b0` on the mesh's search grid.
pub fn fn_00696c00(e: &mut Engine, this: Ptr<NavMesh>, first: u32, second: u32, third: u32) {
    e.call(
        GRID_FORWARDED,
        &args![mesh_grid(this), first, second, third],
    );
}

// Translated from 00696c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x54-byte structure: `00696c60` (the first 0x44
/// bytes), then the word at +0x50. Returns `this`.
pub fn fn_00696c30(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    fn_00696c60(e, this, other);
    let value = e.mem.u32(other.addr() + 0x50);
    e.mem.set_u32(this.addr() + 0x50, value);
    this
}

// Translated from 00696c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x44-byte structure: `00696ca0` (the first 0x20
/// bytes), then the eight words at +0x20 and the word at +0x40. Returns
/// `this`.
pub fn fn_00696c60(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    fn_00696ca0(e, this, other);
    for word in 0..8 {
        let value = e.mem.u32(other.addr() + 0x20 + 4 * word);
        e.mem.set_u32(this.addr() + 0x20 + 4 * word, value);
    }
    let value = e.mem.u32(other.addr() + 0x40);
    e.mem.set_u32(this.addr() + 0x40, value);
    this
}

// Translated from 00696ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x20-byte structure: the 16-byte part (`004a3c90`),
/// then the four words at +0x10 (the first is copied as a `float`).
/// Returns `this`.
pub fn fn_00696ca0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(COPY_STRUCT_16, &args![this, other]);
    let value = e.mem.f32(other.addr() + 0x10);
    e.mem.set_f32(this.addr() + 0x10, value);
    for word in 1..4 {
        let value = e.mem.u32(other.addr() + 0x10 + 4 * word);
        e.mem.set_u32(this.addr() + 0x10 + 4 * word, value);
    }
    this
}

/// State of `NavMesh::FindClosestTriangleForLocation`'s scan.
struct ClosestScan {
    /// Nearest triangle that has a height at the point (`0xFFFF`: none) and
    /// its absolute height difference.
    by_height: u16,
    height_distance: f32,
    /// Triangle whose center is nearest to the point and the squared
    /// distance.
    by_center: u16,
    center_distance: f32,
}

impl ClosestScan {
    fn new(e: &Engine) -> ClosestScan {
        ClosestScan {
            by_height: NO_TRIANGLE,
            height_distance: e.global(FLOAT_MAX),
            by_center: NO_TRIANGLE,
            center_distance: e.global(FLOAT_MAX),
        }
    }
}

/// One candidate triangle of `FindClosestTriangleForLocation`: the ray test
/// `00697730` (tolerances 180 and 64) gives a height difference; a triangle
/// within 40 ends the search (returns true after storing the answer); a
/// nearer one becomes `by_height`. Without a height, the squared distance
/// from the triangle's center (`GetCenter`, `-`, squared length) can become
/// `by_center`.
fn closest_candidate(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    scan: &mut ClosestScan,
    triangle: u16,
    point: Ptr,
    out_triangle: Ptr,
    out_found: Ptr,
) -> bool {
    let up: f32 = e.global(HEIGHT_UP);
    let down: f32 = e.global(HEIGHT_DOWN);
    e.with_stack(32, |e, scratch| {
        let height_slot = scratch;
        let center_difference = scratch.byte_add(4);
        let center_slot = scratch.byte_add(16);
        if fn_00697730(e, mesh, triangle, point, height_slot, up, down) {
            let height = e.mem.f32(height_slot.addr());
            let height = e.call(FLOAT_ABS, &args![height]).f32();
            e.mem.set_f32(height_slot.addr(), height);
            if scan.height_distance > height {
                let close: f64 = e.global(FORTY);
                if (height as f64) < close {
                    e.mem.set_u16(out_triangle.addr(), triangle);
                    e.mem.set_u8(out_found.addr(), 1);
                    return true;
                }
                scan.by_height = triangle;
                scan.height_distance = height;
            }
        } else {
            let center = e
                .call(GET_CENTER, &args![mesh, center_slot, triangle as u32])
                .ptr::<()>();
            e.call(POINT3_SUBTRACT, &args![center, center_difference, point]);
            let distance = e
                .call(POINT3_SQUARED_LENGTH, &args![center_difference])
                .f32();
            if scan.center_distance > distance {
                scan.by_center = triangle;
                scan.center_distance = distance;
            }
        }
        false
    })
}

// Translated from 00696cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindClosestTriangleForLocation` (Xbox PDB): the triangle for
/// the point at `point` (three floats). The candidates are the triangles of
/// the grid cell holding the point (without flag `0x20`), or all triangles
/// when the grid has no list for the point. The stored triangle is the
/// nearest one with a height at the point (stored `1` at `out_found`),
/// otherwise the one with the nearest center (stored `0`), otherwise
/// `0xFFFF` (stored `0`, result false).
pub fn nav_mesh_find_closest_triangle_for_location(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    out_triangle: Ptr,
    out_found: Ptr,
) -> bool {
    let mut scan = ClosestScan::new(e);
    e.with_stack(4, |e, list_slot| {
        e.call(GRID_CELL_LIST, &args![mesh_grid(this), point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            let count = array_size(e, mesh_triangles(this));
            let mut index = 0u32;
            while index < count {
                if closest_candidate(
                    e,
                    this,
                    &mut scan,
                    index as u16,
                    point,
                    out_triangle,
                    out_found,
                ) {
                    return true;
                }
                index += 1;
            }
        } else {
            let count = array_size(e, Ptr::<()>::new(list));
            let mut index = 0u32;
            while index < count {
                let slot = e
                    .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                    .u32();
                let triangle = e.mem.u16(slot);
                index += 1;
                let tri = e
                    .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
                    .ptr::<NavMeshTriangle>();
                if e.call(TRIANGLE_FLAGS_SET, &args![tri, 0x20u32]).bool() {
                    continue;
                }
                if closest_candidate(e, this, &mut scan, triangle, point, out_triangle, out_found) {
                    return true;
                }
            }
        }
        if scan.by_height != NO_TRIANGLE {
            e.mem.set_u16(out_triangle.addr(), scan.by_height);
            e.mem.set_u8(out_found.addr(), 1);
            return true;
        }
        if scan.by_center == NO_TRIANGLE {
            e.mem.set_u16(out_triangle.addr(), NO_TRIANGLE);
            e.mem.set_u8(out_found.addr(), 0);
            return false;
        }
        e.mem.set_u16(out_triangle.addr(), scan.by_center);
        e.mem.set_u8(out_found.addr(), 0);
        true
    })
}

// Translated from 00697010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scans the mesh's edges without neighbours and measures each midpoint's
/// squared 2D and 3D distance from `point`. An edge qualifies when its
/// midpoint's z lies strictly between `point.z - 64` and `point.z + 180`
/// (swapped when `flip` is set), or when its 3D squared distance is at most
/// `radius² - dz²`. The executable compares the planar distance with the previous
/// best 3D distance, then stores the new 3D distance; consequently, later
/// edges can replace earlier ones even when farther away. On success stores
/// the triangle at `out_triangle` (`u16`) and edge at `out_edge`.
pub fn fn_00697010(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    radius: f32,
    out_triangle: Ptr,
    out_edge: Ptr,
    flip: bool,
) -> bool {
    let mut best_triangle = 0u16;
    let mut best_edge = -1i32;
    let mut best: f32 = e.global(FLOAT_MAX);
    let radius_squared = (radius as f64 * radius as f64) as f32;
    let point_z = e.mem.f32(point.addr() + 8) as f64;
    let sixty_four: f64 = e.global(SIXTY_FOUR);
    let one_eighty: f64 = e.global(ONE_EIGHTY);
    let (mut low, mut high) = ((point_z - sixty_four) as f32, (point_z + one_eighty) as f32);
    if flip {
        low = (point_z - one_eighty) as f32;
        high = (point_z + sixty_four) as f32;
    }
    let half: f32 = e.global(HALF);
    let count = array_size(e, mesh_triangles(this));
    // 0: the edge's vertex pair, 8: the vertex sum, 20: the midpoint,
    // 32: the midpoint minus the point.
    e.with_stack(48, |e, scratch| {
        let pair = scratch;
        let sum = scratch.byte_add(8);
        let midpoint = scratch.byte_add(20);
        let difference = scratch.byte_add(32);
        let mut triangle = 0u32;
        while triangle < count {
            let tri = triangle_of(e, this, triangle);
            let mut edge = 0i32;
            while edge < 3 {
                if e.call(EDGE_HAS_NEIGHBOUR, &args![tri, edge]).bool() {
                    edge += 1;
                    continue;
                }
                e.call(GET_EDGE_VERTICES, &args![this, pair, triangle as u16, edge]);
                let first = e.mem.u32(pair.addr());
                let second = e.mem.u32(pair.addr() + 4);
                let total = e.call(POINT3_ADD, &args![first, sum, second]).ptr::<()>();
                e.call(POINT3_SCALE, &args![total, midpoint, half]);
                e.call(POINT3_SUBTRACT, &args![midpoint, difference, point]);
                let planar = e.call(POINT3_SQUARED_LENGTH_2D, &args![difference]).f32();
                let vertical = e.mem.f32(difference.addr() + 8);
                let vertical_squared = (vertical as f64 * vertical as f64) as f32;
                let total_squared = (planar as f64 + vertical_squared as f64) as f32;
                let midpoint_z = e.mem.f32(midpoint.addr() + 8);
                if best > planar
                    && ((high > midpoint_z && low < midpoint_z)
                        || total_squared as f64 <= radius_squared as f64 - vertical_squared as f64)
                {
                    best_triangle = triangle as u16;
                    best_edge = edge;
                    best = total_squared;
                }
                edge += 1;
            }
            triangle += 1;
        }
    });
    if best_edge == -1 {
        return false;
    }
    e.mem.set_u16(out_triangle.addr(), best_triangle);
    e.mem.set_i32(out_edge.addr(), best_edge);
    true
}

// Translated from 006971e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The unit normal of triangle `triangle`: `UnitCross` (`0053d1a0`) of the
/// edges `v1 - v0` and `v2 - v0` into the `NiPoint3` at `result`. Returns
/// `result`.
pub fn fn_006971e0(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr, triangle: u16) -> Ptr {
    let tri = triangle_of(e, this, triangle as u32);
    e.with_stack(24, |e, scratch| {
        let first_edge = scratch;
        let second_edge = scratch.byte_add(12);
        let first = corner_pointer(e, this, tri, 0);
        let second = corner_pointer(e, this, tri, 1);
        e.call(POINT3_SUBTRACT, &args![second, first_edge, first]);
        let first = corner_pointer(e, this, tri, 0);
        let third = corner_pointer(e, this, tri, 2);
        e.call(POINT3_SUBTRACT, &args![third, second_edge, first]);
        e.call(POINT3_UNIT_CROSS, &args![first_edge, result, second_edge]);
    });
    result
}

// Translated from 00697290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The circumcenter of triangle `triangle` in x and y, with the largest z of
/// its three vertices, stored at `result`. Returns `result`. (With the
/// edge vectors `a` and `b` from vertex 0, the center is `v0 + 0.5 *
/// (b.y |a|² - a.y |b|², a.x |b|² - b.x |a|²) / (a.x b.y - b.x a.y)`.)
pub fn fn_00697290(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr, triangle: u16) -> Ptr {
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let tri = triangle_of(e, this, triangle as u32);
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let [v0, v1, v2] = vertices;
    let (ax, ay) = (
        (v1[0] as f64 - v0[0] as f64) as f32,
        (v1[1] as f64 - v0[1] as f64) as f32,
    );
    let (bx, by) = (
        (v2[0] as f64 - v0[0] as f64) as f32,
        (v2[1] as f64 - v0[1] as f64) as f32,
    );
    let a_squared = (ax as f64 * ax as f64 + ay as f64 * ay as f64) as f32;
    let b_squared = (bx as f64 * bx as f64 + by as f64 * by as f64) as f32;
    let determinant = (ax as f64 * by as f64 - bx as f64 * ay as f64) as f32;
    let reciprocal_half = {
        let half: f64 = e.global(DOUBLE_HALF);
        (half / determinant as f64) as f32
    };
    let mut z = v2[2];
    if v1[2] <= v0[2] && v2[2] <= v0[2] {
        z = v0[2];
    } else if v2[2] <= v1[2] {
        z = v1[2];
    }
    let y = (((ax as f64 * b_squared as f64 - bx as f64 * a_squared as f64)
        * reciprocal_half as f64)
        + v0[1] as f64) as f32;
    let x = (v0[0] as f64
        - ((ay as f64 * b_squared as f64 - by as f64 * a_squared as f64) * reciprocal_half as f64))
        as f32;
    e.with_stack(12, |e, center| {
        e.call(POINT3_CONSTRUCT, &args![center, x, y, z]);
        let value = read_point(e, center.addr());
        write_point(e, result.addr(), value);
    });
    result
}

// Translated from 00697440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The circumcenter of triangle `triangle` in x and y (as `00697290`), with
/// z = 0, stored at the three floats at `result`.
pub fn fn_00697440(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, result: Ptr) {
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let tri = triangle_of(e, this, triangle as u32);
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let [v0, v1, v2] = vertices;
    let (ax, ay) = (
        (v1[0] as f64 - v0[0] as f64) as f32,
        (v1[1] as f64 - v0[1] as f64) as f32,
    );
    let (bx, by) = (
        (v2[0] as f64 - v0[0] as f64) as f32,
        (v2[1] as f64 - v0[1] as f64) as f32,
    );
    let a_squared = (ax as f64 * ax as f64 + ay as f64 * ay as f64) as f32;
    let b_squared = (bx as f64 * bx as f64 + by as f64 * by as f64) as f32;
    let determinant = (ax as f64 * by as f64 - bx as f64 * ay as f64) as f32;
    let half: f64 = e.global(DOUBLE_HALF);
    let reciprocal_half = (half / determinant as f64) as f32;
    let x = (v0[0] as f64
        - ((ay as f64 * b_squared as f64 - by as f64 * a_squared as f64) * reciprocal_half as f64))
        as f32;
    let y = (((ax as f64 * b_squared as f64 - bx as f64 * a_squared as f64)
        * reciprocal_half as f64)
        + v0[1] as f64) as f32;
    e.mem.set_f32(result.addr(), x);
    e.mem.set_f32(result.addr() + 4, y);
    e.mem.set_f32(result.addr() + 8, 0.0);
}

// Translated from 00697590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindPortalTriangles` (Xbox PDB): fills the word array `result`
/// (cleared and reserved for 100 entries) with the index of every triangle
/// that has an edge whose extra info leads to the `NavMeshInfo` `info`
/// (each triangle once). True when the array is not empty.
pub fn nav_mesh_find_portal_triangles(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    info: Ptr,
    result: Ptr,
) -> bool {
    e.call(ARRAY_SET_SIZE, &args![result, 0u32, 1u32]);
    e.call(ARRAY_SET_RESERVED_SIZE, &args![result, 100u32]);
    let mut triangle = 0u32;
    while triangle < array_size(e, mesh_triangles(this)) {
        let tri = triangle_of(e, this, triangle);
        if e.call(ANY_EDGE_HAS_INFO, &args![tri]).bool() {
            let mut edge = 0u32;
            while (edge as i32) < 3 {
                if e.call(EDGE_HAS_INFO, &args![tri, edge]).bool() {
                    let slot = e.call(EDGE_SLOT, &args![tri, edge]).u16();
                    let edge_info = e.call(GET_EXTRA_EDGE_INFO, &args![this, slot]).u32();
                    if e.mem.u32(edge_info + EDGE_INFO_OTHER_MESH) == info.addr() {
                        e.with_stack(4, |e, entry| {
                            e.mem.set_u32(entry.addr(), triangle);
                            e.call(ARRAY_ADD_DWORD, &args![result, entry]);
                        });
                        break;
                    }
                }
                edge += 1;
            }
        }
        triangle += 1;
    }
    array_size(e, result) != 0
}

// Translated from 00697670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `00696a50` finds a triangle for `point` (`second` is passed on as
/// its callback).
pub fn fn_00697670(e: &mut Engine, this: Ptr<NavMesh>, point: Ptr, second: Ptr) -> bool {
    fn_00696a50(e, this, point, second) != NO_TRIANGLE
}

// Translated from 006976a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `EdgeExtraInfo` of edge `edge` of triangle `triangle`, created when
/// the edge has none (a new info is appended to the mesh and the edge's
/// slot and flag are set to it), then reset to "no other mesh" (`pOtherMesh`
/// 0, triangle `0xFFFF`). Returns the info.
pub fn fn_006976a0(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, edge: i32) -> Ptr {
    let mut info = e
        .call(GET_EDGE_EXTRA_INFO, &args![this, triangle, edge])
        .ptr::<()>();
    if info.is_null() {
        let index = e.with_stack(12, |e, fresh| {
            e.call(EDGE_INFO_CONSTRUCTOR, &args![fresh]);
            e.call(ADD_EXTRA_EDGE_INFO, &args![this, fresh]).u16()
        });
        let tri = e
            .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
            .ptr::<NavMeshTriangle>();
        e.call(SET_EDGE_INFO_SLOT, &args![tri, edge, index]);
        info = e
            .call(GET_EDGE_EXTRA_INFO, &args![this, triangle, edge])
            .ptr();
    }
    e.mem.set_u32(info.addr() + EDGE_INFO_OTHER_MESH, 0);
    e.mem.set_u16(info.addr() + 8, 0xffff);
    info
}

// Translated from 00697730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray test of a point against triangle `triangle`: gathers the
/// triangle's three vertices (`NiPoint3` copies) and calls `006b9cf0`
/// (cdecl) with them, `point`, the out float `height` and the two limits.
/// Returns its result.
pub fn fn_00697730(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    height: Ptr,
    first_limit: f32,
    second_limit: f32,
) -> bool {
    let tri = triangle_of(e, this, triangle as u32);
    e.with_stack(36, |e, scratch| {
        for corner in 0..3u32 {
            let vertex = corner_pointer(e, this, tri, corner);
            let value = read_point(e, vertex.addr());
            write_point(e, scratch.addr() + 12 * corner, value);
        }
        e.call(
            TRIANGLE_RAY_TEST,
            &args![
                scratch,
                scratch.byte_add(12),
                scratch.byte_add(24),
                point,
                height,
                first_limit,
                second_limit
            ],
        )
        .bool()
    })
}

// Translated from 00697810 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00697730` with both limits `FLT_MAX`. When `callback` is not null
/// and the ray hits, `callback`'s virtual slot 0 is called with `point`,
/// the mesh, the triangle and the hit point (`point` with its z raised by
/// the height) and its result is returned; the plain hit result otherwise.
pub fn fn_00697810(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    height: Ptr,
    callback: Ptr,
) -> bool {
    let limit: f32 = e.global(FLOAT_MAX);
    if callback.is_null() {
        return fn_00697730(e, this, triangle, point, height, limit, limit);
    }
    let hit = fn_00697730(e, this, triangle, point, height, limit, limit);
    if !hit {
        return false;
    }
    let mut hit_point = read_point(e, point.addr());
    hit_point[2] = (hit_point[2] as f64 + e.mem.f32(height.addr()) as f64) as f32;
    e.with_stack(12, |e, slot| {
        write_point(e, slot.addr(), hit_point);
        e.vcall(
            callback.addr(),
            0,
            &args![point, this, triangle as u32, slot],
        )
        .bool()
    })
}

// Translated from 00697980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::ComputeTriangleZForLocation` (Xbox PDB): whether the point at
/// `point` (x, y, z) lies over triangle `triangle` (barycentric weights all
/// not negative, a flat triangle never) and within the vertical tolerances:
/// the triangle's height at the point is stored at `height_out`, and the
/// point may be at most `above` over and `below` under it.
pub fn nav_mesh_compute_triangle_z_for_location(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    above: f32,
    below: f32,
    height_out: Ptr,
) -> bool {
    let tri = triangle_of(e, this, triangle as u32);
    let a = corner_pointer(e, this, tri, 0);
    let b = corner_pointer(e, this, tri, 1);
    let c = corner_pointer(e, this, tri, 2);
    let (a, b, c) = (
        read_point(e, a.addr()),
        read_point(e, b.addr()),
        read_point(e, c.addr()),
    );
    let p = read_point(e, point.addr());
    let sub = |x: f32, y: f32| x as f64 - y as f64;
    let determinant =
        ((sub(b[0], a[0]) * sub(c[1], a[1])) - (sub(c[0], a[0]) * sub(b[1], a[1]))) as f32;
    let zero: f64 = e.global(ZERO);
    if determinant as f64 == zero {
        return false;
    }
    let b_x = (b[0] as f64 - p[0] as f64) as f32;
    let c_x = (c[0] as f64 - p[0] as f64) as f32;
    let b_y = (b[1] as f64 - p[1] as f64) as f32;
    let c_y = (c[1] as f64 - p[1] as f64) as f32;
    let reciprocal = (1.0 / determinant as f64) as f32;
    let weight_a =
        (((b_x as f64 * c_y as f64) - (c_x as f64 * b_y as f64)) * reciprocal as f64) as f32;
    if (weight_a as f64) < zero {
        return false;
    }
    let a_x = (a[0] as f64 - p[0] as f64) as f32;
    let a_y = (a[1] as f64 - p[1] as f64) as f32;
    let weight_b =
        (((c_x as f64 * a_y as f64) - (a_x as f64 * c_y as f64)) * reciprocal as f64) as f32;
    if (weight_b as f64) < zero {
        return false;
    }
    let weight_c =
        (((a_x as f64 * b_y as f64) - (b_x as f64 * a_y as f64)) * reciprocal as f64) as f32;
    if (weight_c as f64) < zero {
        return false;
    }
    let height = (weight_a as f64 * a[2] as f64
        + weight_b as f64 * b[2] as f64
        + weight_c as f64 * c[2] as f64) as f32;
    e.mem.set_f32(height_out.addr(), height);
    let difference = (p[2] as f64 - height as f64) as f32;
    // `above < difference` fails; `difference < -below` fails.
    if above < difference {
        return false;
    }
    if (difference as f64) < -(below as f64) {
        return false;
    }
    true
}

// Translated from 00697b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the point at `point` lies over any triangle of its grid cell
/// (`ComputeTriangleZForLocation` with `above` and `below`; the first
/// triangle that does stores its height at `height_out`). False when the
/// grid has no list for the point.
pub fn fn_00697b70(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    above: f32,
    below: f32,
    height_out: Ptr,
) -> bool {
    e.with_stack(4, |e, list_slot| {
        e.call(GRID_CELL_LIST, &args![mesh_grid(this), point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            return false;
        }
        let count = array_size(e, Ptr::<()>::new(list));
        let mut index = 0u32;
        while index < count {
            let slot = e
                .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                .u32();
            let triangle = e.mem.u16(slot);
            if nav_mesh_compute_triangle_z_for_location(
                e, this, triangle, point, above, below, height_out,
            ) {
                return true;
            }
            index += 1;
        }
        false
    })
}

// Translated from 00697c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random walkable point near `point`: up to 8 times draws a point in a
/// disc of radius `radius` (x uniform in `[-r, r]`, y uniform in the chord at
/// that x, z from the static vector `011f426c`) and keeps the first whose
/// position (added to `point`) is over a triangle (`00697670`); then adds
/// `point` to it, pushes it away from nearby border edges (`00697d40`, 64
/// units) and stores it at `result`. False (nothing stored) if all 8 draws
/// miss.
pub fn fn_00697c10(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    radius: f32,
    result: Ptr,
) -> bool {
    e.with_stack(48, |e, scratch| {
        // 0: the candidate offset, 12: candidate + point, 24: the pushed
        // point.
        let offset = scratch;
        let moved = scratch.byte_add(12);
        let pushed = scratch.byte_add(24);
        for word in 0..3 {
            let value = e.mem.u32(RANDOM_POINT_BASE + 4 * word);
            e.mem.set_u32(offset.addr() + 4 * word, value);
        }
        let mut found = false;
        let mut attempt = 0u32;
        while attempt < 8 && !found {
            let x = e.call(RANDOM_FLOAT, &args![-radius, radius]).f32();
            let remaining = ((radius as f64 * radius as f64) - (x as f64 * x as f64)) as f32;
            let half_chord = e.call(FLOAT_SQRT, &args![remaining]).f32();
            let y = e.call(RANDOM_FLOAT, &args![-half_chord, half_chord]).f32();
            e.mem.set_f32(offset.addr(), x);
            e.mem.set_f32(offset.addr() + 4, y);
            let candidate = e.call(POINT3_ADD, &args![offset, moved, point]).ptr::<()>();
            found = fn_00697670(e, this, candidate, Ptr::NULL);
            attempt += 1;
        }
        if !found {
            return false;
        }
        e.call(POINT3_ADD_ASSIGN, &args![offset, point]);
        let limit: f32 = e.global(HEIGHT_DOWN);
        let adjusted = fn_00697d40(e, this, pushed, offset, limit);
        let value = read_point(e, adjusted.addr());
        write_point(e, result.addr(), value);
        true
    })
}

// Translated from 00697d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the point at `position` away from the border edges of the mesh
/// near it: finds the triangle under the position (`00696a50`); for each
/// of its edges that has no neighbour, and for each border edge of its
/// neighbours, whose distance to the position (`006b9940`) is less than
/// `distance`, moves the result by the triangle's unit normal crossed with
/// the edge vector, normalized and scaled by `distance - edge distance`.
/// The result (the position when no triangle is found) is stored at
/// `result`; returns `result`.
pub fn fn_00697d40(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    position: Ptr,
    distance: f32,
) -> Ptr {
    let mut moved = read_point(e, position.addr());
    let found = fn_00696a50(e, this, position, Ptr::NULL);
    if found != NO_TRIANGLE {
        // 0: moved point, 12: normal, 24: edge vector, 36: cross product,
        // 48: scaled push, 64: the edge's vertex pair.
        e.with_stack(80, |e, scratch| {
            let normal = scratch.byte_add(12);
            let edge_vector = scratch.byte_add(24);
            let cross = scratch.byte_add(36);
            let push = scratch.byte_add(48);
            let pair = scratch.byte_add(64);
            let mut neighbours = [0u16; 3];
            let mut neighbour_count = 0usize;
            let tri = triangle_of(e, this, found as u32);
            let apply = |e: &mut Engine, triangle: u16, edge: i32, moved: &mut [f32; 3]| {
                e.call(GET_EDGE_VERTICES, &args![this, pair, triangle, edge]);
                let first = e.mem.u32(pair.addr());
                let second = e.mem.u32(pair.addr() + 4);
                let edge_distance = e
                    .call(POINT_SEGMENT_DISTANCE, &args![position, first, second])
                    .f32();
                if distance > edge_distance {
                    fn_006971e0(e, this, normal, triangle);
                    let difference = e
                        .call(POINT3_SUBTRACT, &args![first, edge_vector, second])
                        .ptr::<()>();
                    e.call(POINT3_CROSS, &args![normal, cross, difference]);
                    e.call(POINT3_NORMALIZE, &args![cross]);
                    let strength = (distance as f64 - edge_distance as f64) as f32;
                    let scaled = e
                        .call(POINT3_SCALE, &args![cross, push, strength])
                        .ptr::<()>();
                    write_point(e, scratch.addr(), *moved);
                    e.call(POINT3_ADD_ASSIGN, &args![scratch, scaled]);
                    *moved = read_point(e, scratch.addr());
                }
            };
            for edge in 0..3i32 {
                let mut neighbour = NO_TRIANGLE;
                if !e.call(EDGE_HAS_INFO, &args![tri, edge]).bool() {
                    neighbour = e.call(EDGE_SLOT, &args![tri, edge]).u16();
                }
                if neighbour != NO_TRIANGLE {
                    neighbours[neighbour_count] = neighbour;
                    neighbour_count += 1;
                } else {
                    apply(e, found, edge, &mut moved);
                }
            }
            for &neighbour in &neighbours[..neighbour_count] {
                let neighbour_tri = triangle_of(e, this, neighbour as u32);
                for edge in 0..3i32 {
                    if !e.call(EDGE_HAS_INFO, &args![neighbour_tri, edge]).bool()
                        && e.call(EDGE_SLOT, &args![neighbour_tri, edge]).u16() == NO_TRIANGLE
                    {
                        apply(e, neighbour, edge, &mut moved);
                    }
                }
            }
        });
    }
    write_point(e, result.addr(), moved);
    result
}

// Translated from 00698030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: the four packed bits of edge `edge` (bits
/// `16 + 6 * edge` upward of the flags).
pub fn fn_00698030(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u16) -> u32 {
    let shift = (edge as u32).wrapping_mul(6).wrapping_add(0x10);
    (e.get(this, NavMeshTriangle::TriangleFlags) >> (shift & 31)) & 0xf
}

// Translated from 00698060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first edge of triangle `triangle`, other than `skip`, for which the
/// 2D segment test `006b9e90` of the edge's two vertices (x and y only)
/// with the three words `first`, `second` and `third` succeeds; -1 if none.
pub fn fn_00698060(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    skip: i32,
    first: u32,
    second: u32,
    third: u32,
) -> i32 {
    e.with_stack(24, |e, points| {
        for corner in 0..3u32 {
            let tri = triangle_of(e, this, triangle as u32);
            let vertex = corner_pointer(e, this, tri, corner);
            let x = e.mem.u32(vertex.addr());
            let y = e.mem.u32(vertex.addr() + 4);
            e.mem.set_u32(points.addr() + 8 * corner, x);
            e.mem.set_u32(points.addr() + 8 * corner + 4, y);
        }
        for edge in 0..3i32 {
            if edge == skip {
                continue;
            }
            let next = if edge + 1 == 3 { 0 } else { edge + 1 };
            let hit = e
                .call(
                    SEGMENT_SEGMENT_TEST,
                    &args![
                        points.byte_add(8 * edge as u32),
                        points.byte_add(8 * next as u32),
                        first,
                        second,
                        third
                    ],
                )
                .bool();
            if hit {
                return edge;
            }
        }
        -1
    })
}

// Translated from 00698160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindInterectingEdge2` (Xbox PDB; the typo is the game's): the
/// first edge of triangle `triangle`, other than `skip` and other than a
/// degenerate (zero length) one, that the 2D segment test `006ba240`
/// (`first`, `point`) crosses. Writes the 20-byte result at `result`:
/// `{3, edge, x, y, z}` with the intersection point, or `{0, -1, ..}`
/// without a crossing (the three floats are not written then: the game
/// leaves uninitialized stack there). Returns `result`.
pub fn nav_mesh_find_interecting_edge2(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    triangle: u16,
    skip: i32,
    first: u32,
    point: Ptr,
) -> Ptr {
    e.with_stack(48, |e, scratch| {
        let points = scratch;
        let intersection = scratch.byte_add(36);
        for corner in 0..3u32 {
            let tri = triangle_of(e, this, triangle as u32);
            let vertex = corner_pointer(e, this, tri, corner);
            let value = read_point(e, vertex.addr());
            write_point(e, points.addr() + 12 * corner, value);
        }
        for edge in 0..3i32 {
            if edge == skip {
                continue;
            }
            let next = if edge + 1 == 3 { 0 } else { edge + 1 };
            let from = points.byte_add(12 * edge as u32);
            let to = points.byte_add(12 * next as u32);
            if e.call(POINT3_EQUALS, &args![from, to]).bool() {
                continue;
            }
            let hit = e
                .call(
                    SEGMENT_INTERSECTION_2D,
                    &args![from, to, first, point, intersection],
                )
                .bool();
            if hit {
                e.mem.set_u32(result.addr(), 3);
                e.mem.set_i32(result.addr() + 4, edge);
                for word in 0..3 {
                    let value = e.mem.u32(intersection.addr() + 4 * word);
                    e.mem.set_u32(result.addr() + 8 + 4 * word, value);
                }
                return;
            }
        }
        e.mem.set_u32(result.addr(), 0);
        e.mem.set_i32(result.addr() + 4, -1);
    });
    result
}

// Translated from 00698300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 20-byte intersection result: default-constructs the
/// `NiPoint3` at +8 (`006815c0`). Returns `this`.
pub fn fn_00698300(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![this.byte_add(8)]);
    this
}

// Translated from 00698320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first edge of triangle `triangle`, other than `skip`, that the
/// segment from `first` to `second` (x and y of two points) properly
/// crosses: the triangle edge's end points are on opposite sides (or
/// touching, but the first one off) of the segment's line and the segment's
/// end points are on opposite sides (or touching, but the second one off)
/// of the edge's line. -1 if there is none.
pub fn fn_00698320(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    skip: i32,
    first: Ptr,
    second: Ptr,
) -> i32 {
    let tri = triangle_of(e, this, triangle as u32);
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let p = read_point(e, first.addr());
    let q = read_point(e, second.addr());
    let zero: f64 = e.global(ZERO);
    for edge in 0..3i32 {
        if edge == skip {
            continue;
        }
        let next = if edge + 1 == 3 { 0 } else { edge + 1 };
        let a = vertices[edge as usize];
        let b = vertices[next as usize];
        let cross = |u: [f32; 3], v: [f32; 3], w: [f32; 3]| -> f32 {
            // (u.x - w.x) * (v.y - w.y) - (u.y - w.y) * (v.x - w.x)
            let first = (u[0] as f64 - w[0] as f64) * (v[1] as f64 - w[1] as f64);
            let second = (u[1] as f64 - w[1] as f64) * (v[0] as f64 - w[0] as f64);
            (first - second) as f32
        };
        let q_side = cross(a, b, q);
        let p_side = cross(a, b, p);
        let a_side = cross(p, q, a);
        let b_side = cross(p, q, b);
        // `JP` after `TEST AH, 0x41` on `st0 <=> 0.0`: taken (edge rejected)
        // when the value is greater than zero, or unordered.
        let rejected = |value: f64| {
            let flags = compare_flags(value, zero);
            parity_even(flags, 0x41)
        };
        if rejected(q_side as f64 * p_side as f64) || rejected(a_side as f64 * b_side as f64) {
            continue;
        }
        if q_side as f64 == zero || b_side as f64 == zero {
            continue;
        }
        return edge;
    }
    -1
}

// Translated from 00698580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a two-point segment: copies the three floats at `first`
/// to `this + 0` and those at `second` to `this + 0xC`. Returns `this`.
pub fn fn_00698580(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(first.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    for word in 0..3 {
        let value = e.mem.u32(second.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 12 + 4 * word, value);
    }
    this
}

// Translated from 006985c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object holding two 16-byte elements at +4: runs
/// `_vector_constructor_iterator_` over them with the element constructor
/// `006d5320`. Returns `this`.
pub fn fn_006985c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![this.byte_add(4), 0x10u32, 2u32, ELEMENT_16_CONSTRUCTOR],
    );
    this
}

// Translated from 006985f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` copy: copies the two floats at `other` to `this`. Returns
/// `this`.
pub fn fn_006985f0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let x = e.mem.f32(other.addr());
    let y = e.mem.f32(other.addr() + 4);
    e.mem.set_f32(this.addr(), x);
    e.mem.set_f32(this.addr() + 4, y);
    this
}

// Translated from 00698620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` product of `this` and `other` as complex numbers conjugated:
/// `result = (this.x * other.x + this.y * other.y,
/// this.y * other.x - this.x * other.y)`. Returns `result`.
pub fn fn_00698620(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let (ax, ay) = (e.mem.f32(this.addr()), e.mem.f32(this.addr() + 4));
    let (bx, by) = (e.mem.f32(other.addr()), e.mem.f32(other.addr() + 4));
    let x = (ax as f64 * bx as f64 + ay as f64 * by as f64) as f32;
    let y = (ay as f64 * bx as f64 - ax as f64 * by as f64) as f32;
    e.with_stack(8, |e, product| {
        e.mem.set_f32(product.addr(), x);
        e.mem.set_f32(product.addr() + 4, y);
        fn_006985f0(e, result, product);
    });
    result
}

// Translated from 00698680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the quadrant code of a 2D point: x-negative toggles bit 0 and
/// y-negative sets bits 0 and 1. The comparisons are strict, so zero is
/// treated as nonnegative.
pub fn fn_00698680(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = 0;
    if e.mem.f32(this.addr() + 4) < 0.0 {
        result ^= 3;
    }
    if e.mem.f32(this.addr()) < 0.0 {
        result ^= 1;
    }
    result
}

// Translated from 006986d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ensures a navmesh info entry exists for `this`. It asks the navmesh map
/// to add one only when the mesh has no info yet, then pauses the map.
pub fn fn_006986d0(e: &mut Engine, this: Ptr) -> bool {
    let allocator_state = e.call(0x0084_e3a0, &args![this]).u32();
    let tes = e.global::<Ptr>(TES_INSTANCE);
    let map = e.call(0x0045_af00, &args![tes]).ptr::<Ptr>();
    let info = e
        .call(0x0069_3d70, &args![map, allocator_state])
        .ptr::<Ptr>();
    if !e.call(0x0068_efb0, &args![this]).bool() {
        let allocation = e.call(0x0040_1000, &args![0x5cu32]).u32();
        let created = if allocation == 0 {
            Ptr::NULL
        } else {
            e.call(0x006b_46a0, &args![allocation]).ptr::<Ptr>()
        };
        e.call(0x006b_5560, &args![created, this]);
        e.call(0x006d_3100, &args![this, created]);
        e.call(0x005a_8040, &args![created, this]);
        let tes = e.global::<Ptr>(TES_INSTANCE);
        let map = e.call(0x0045_af00, &args![tes]).ptr::<Ptr>();
        e.call(0x006b_6940, &args![map, created]);
        true
    } else {
        e.call(0x006b_5560, &args![info, this]);
        false
    }
}

// Translated from 00698f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the obstacle cutter for this mesh and destroys its 0x4c-byte local
/// cutter object afterward.
pub fn nav_mesh_add_obstacle(e: &mut Engine, this: Ptr<NavMesh>, obstacle: Ptr, data: Ptr) -> bool {
    e.with_stack(0x4c, |e, cutter| {
        e.call(0x006b_bb80, &args![cutter]);
        let result = e
            .call(0x006b_bd30, &args![cutter, this, obstacle, data])
            .bool();
        e.call(0x006b_bc50, &args![cutter]);
        result
    })
}

// Translated from 00699000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the indexed obstacle undo record. If it is the newest entry, it
/// is undone when its state is 1, then deleted and removed. Older entries
/// are left in place when their referenced triangle range is empty; otherwise
/// later records are undone and replayed in index order.
pub fn fn_00699000(e: &mut Engine, this: Ptr<NavMesh>, index: u32, context: Ptr) -> u32 {
    let array = this.byte_add(0xd0);
    let record_at = |e: &mut Engine, index| {
        let entry = e.call(ARRAY_ELEMENT_4, &args![array, index]).u32();
        e.call(POINTER_GET, &args![entry]).ptr::<Ptr>()
    };
    // 0069903e..00699059: the record is fetched first, then the size.
    let record = record_at(e, index);
    let size = e.call(ARRAY_SIZE, &args![array]).u32();
    if index == size.wrapping_sub(1) {
        e.with_stack(0x4c, |e, cutter| {
            e.call(0x006b_bb80, &args![cutter]);
            if e.mem.u32(record.addr() + 8) == 1 {
                e.call(0x006b_c3e0, &args![cutter, this, record, context]);
            }
            e.call(0x006b_c8e0, &args![cutter, this, record]);
            fn_00699590(e, this, index);
            e.call(0x006b_bc50, &args![cutter]);
        });
    } else if fn_00699690(e, this, record.cast()) == 0 {
        e.with_stack(0x4c, |e, cutter| {
            e.call(0x006b_bb80, &args![cutter]);
            if e.mem.u32(record.addr() + 8) == 1 {
                e.call(0x006b_c3e0, &args![cutter, this, record, context]);
            }
            e.call(0x006b_bc50, &args![cutter]);
        });
    } else {
        e.with_stack(400, |e, captured| {
            e.call(
                0x00ec_782f,
                &args![captured, 4u32, 100u32, 0x0066_94e0u32, 0x0045_cec0u32],
            );
            let mut captured_count = 0;
            e.with_stack(0x4c, |e, cutter| {
                e.call(0x006b_bb80, &args![cutter]);
                // 00699170: the size is read again here.
                let mut cursor = e.call(ARRAY_SIZE, &args![array]).u32().wrapping_sub(1);
                while cursor != index.wrapping_sub(1) {
                    // 006991b4..006991e8, 0069921e, 00699262: the record is fetched
                    // anew (`00877a30`, `00559450`) for each use.
                    let state = record_at(e, cursor);
                    if e.mem.u32(state.addr() + 8) == 1 {
                        let undo = record_at(e, cursor);
                        e.call(0x006b_c3e0, &args![cutter, this, undo, context]);
                        if index < cursor {
                            let source = record_at(e, cursor).byte_add(0x0c);
                            e.call(
                                0x006e_5cc0,
                                &args![captured.byte_add(captured_count * 4), source],
                            );
                            captured_count += 1;
                        }
                    }
                    let undo = record_at(e, cursor);
                    e.call(0x006b_c8e0, &args![cutter, this, undo]);
                    fn_00699590(e, this, cursor);
                    cursor = cursor.wrapping_sub(1);
                }
                while cursor != u32::MAX {
                    // 006992c5, 006992ea: fetched again for the erase.
                    let state = record_at(e, cursor);
                    if e.mem.u32(state.addr() + 8) != 3 {
                        break;
                    }
                    let undo = record_at(e, cursor);
                    e.call(0x006b_c8e0, &args![cutter, this, undo]);
                    fn_00699590(e, this, cursor);
                    cursor = cursor.wrapping_sub(1);
                }
                for item in 0..captured_count {
                    e.call(
                        0x006b_bd30,
                        &args![cutter, this, captured.byte_add(item * 4), context],
                    );
                }
                e.call(0x006b_bc50, &args![cutter]);
            });
            e.call(0x00ec_5fce, &args![captured, 4u32, 100u32, 0x0045_cec0u32]);
        });
    }
    1
}

// Translated from 00699690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the greatest triangle type referenced by the obstacle record's
/// 16-bit triangle index array at +0x20 (2-byte elements, `005eeda0`) and its
/// saved-triangle array at +0x30 (12-byte elements, `006a1440`, the index the
/// first half-word); each array's count is read before every element.
pub fn fn_00699690(e: &mut Engine, this: Ptr<NavMesh>, obstacle: Ptr) -> u32 {
    let mut largest = 0;
    // 006996cc: the +0x20 array's element is `005eeda0`; 00699725: the +0x30
    // array's is `006a1440`.
    for (offset, element_at) in [(0x20, 0x005e_eda0), (0x30, 0x006a_1440)] {
        let array = obstacle.byte_add(offset);
        let mut index = 0u32;
        // 006996b8/00699711: the count (`0044ddc0`) before each element.
        while index < e.call(ARRAY_SIZE, &args![array]).u32() {
            let element = e.call(element_at, &args![array, index]).u32();
            let triangle_index = e.mem.u16(element) as u32;
            let triangle = e
                .call(GET_TRIANGLE, &args![this, triangle_index])
                .ptr::<NavMeshTriangle>();
            let triangle_type = fn_00699790(e, triangle.cast()) as u16;
            largest = fn_00699760(e, largest, triangle_type);
            index += 1;
        }
    }
    largest
}

// Translated from 006997b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks a triangle (16-bit key) up in the navmesh's triangle-to-obstacle
/// map, whose pointer is the word at `this + 0xe0` (the constructor stores
/// the heap map there, 0068ed10), assigning the mapped `NiPointer` to `out`
/// (`0069afb0`), and answers whether it was found (the lookup's value: the
/// exe ends on that call and leaves its `EAX`; the callers test `AL`).
pub fn fn_006997b0(e: &mut Engine, this: Ptr, key: u16, out: Ptr) -> bool {
    // 006997c3: `MOV ECX,[EDX+0xe0]` loads the map pointer (not `this + 0xe0`).
    let map = e.mem.u32(this.addr() + 0xe0);
    e.call(0x0069_afb0, &args![map, key, out]).bool()
}

// Translated from 006997e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects triangles intersecting the narrowed obstacle bounds, then marks
/// them as closed-door triangles and appends records for this door. Existing
/// records for the same door are preserved. Normal-path temporary destruction
/// is retained; compiler exception unwinding is omitted.
pub fn fn_006997e0(e: &mut Engine, this: Ptr<NavMesh>, door: Ptr, bounds: Ptr) -> bool {
    e.with_stack(52, |e, scratch| {
        let extent = scratch;
        let adjustment = scratch.byte_add(12);
        let triangles = scratch.byte_add(24);
        let reference = scratch.byte_add(40);
        let record = scratch.byte_add(44);
        e.call(
            0x0043_9ef0,
            &args![bounds.byte_add(0x4c), extent, bounds.byte_add(0x40)],
        );
        for axis in 0..3 {
            let initial = e.mem.u32(0x011f_426c + axis * 4);
            e.mem.set_u32(adjustment.addr() + axis * 4, initial);
        }
        let absolute = |e: &mut Engine, axis: u32| {
            let value = e.mem.f32(extent.addr() + axis * 4);
            e.call(0x0040_8840, &args![value]).f64()
        };
        let x = absolute(e, 0);
        let y = absolute(e, 1);
        // TEST AH,0x41 takes the alternate branch for <= and unordered.
        let candidate = if y > x { 0 } else { 1 };
        let smaller = absolute(e, candidate);
        let z = absolute(e, 2);
        let axis = if z > smaller { candidate } else { 2 };
        let value = -absolute(e, axis) * e.mem.f64(0x0106_b168);
        e.mem.set_f32(adjustment.addr() + axis * 4, value as f32);
        e.call(0x0069_17d0, &args![triangles]);
        e.call(0x0063_3c90, &args![reference, bounds]);
        // The same-unit spatial collector remains open; dispatch by address
        // until its translation is available, preserving its real ABI.
        e.call(0x0069_87d0, &args![this, reference, adjustment, triangles]);
        e.call(0x0045_cec0, &args![reference]);
        let mut index = 0u32;
        while index < e.call(ARRAY_SIZE, &args![triangles]).u32() {
            let element = e.call(0x005e_eda0, &args![triangles, index]).u32();
            let triangle_index = e.mem.u16(element);
            let existing = nav_mesh_get_closed_door_info_for_triangle(e, this, triangle_index);
            if existing.is_null() || e.mem.u32(existing.addr()) != door.addr() {
                let element = e.call(0x005e_eda0, &args![triangles, index]).u32();
                let triangle_index = e.mem.u16(element);
                let triangle = e.call(0x0055_82f0, &args![this, triangle_index]).u32();
                e.call(0x0069_1570, &args![triangle, 0x1000u32]);
                e.call(0x0069_2870, &args![record]);
                e.mem.set_u32(record.addr(), door.addr());
                let element = e.call(0x005e_eda0, &args![triangles, index]).u32();
                let triangle_index = e.mem.u16(element);
                e.mem.set_u16(record.addr() + 4, triangle_index);
                fn_0069b030(e, this.byte_add(0x68), record);
            }
            index = index.wrapping_add(1);
        }
        e.call(0x0069_1800, &args![triangles]);
    });
    true
}

// Translated from 00699af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first closed-door edge record at `NavMesh + 0x68` whose
/// triangle index matches `triangle`, or null if no entry matches.
pub fn nav_mesh_get_closed_door_info_for_triangle(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
) -> Ptr {
    let records = this.byte_add(0x68);
    let mut index = 0u32;
    // 00699b11: the size is read for every test of the loop.
    while index < e.call(ARRAY_SIZE, &args![records]).u32() {
        let record = e.call(0x0069_50d0, &args![records, index]).ptr::<()>();
        if e.mem.u16(record.addr() + 4) == triangle {
            // 00699b40: the match is fetched again; that call's result is returned.
            return e.call(0x0069_50d0, &args![records, index]).ptr::<()>();
        }
        index = index.wrapping_add(1);
    }
    Ptr::NULL
}

// Translated from 00699a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes records at `NavMesh + 0x68` whose first word matches `form_id`.
/// Before each removal, the record's triangle flag at bit 0x1000 is cleared
/// when its triangle index remains within the mesh's triangle count.
pub fn fn_00699a30(e: &mut Engine, this: Ptr<NavMesh>, form_id: u32) -> bool {
    let records = this.byte_add(0x68);
    let mut index = 0;
    while index < e.call(ARRAY_SIZE, &args![records]).u32() {
        let record = e.call(0x0069_50d0, &args![records, index]).ptr::<()>();
        if e.mem.u32(record.addr()) == form_id {
            let triangle = e.mem.u16(record.addr() + 4) as u32;
            if triangle < e.call(0x0055_8200, &args![this]).u32() {
                let triangle = e.call(0x0055_82f0, &args![this, triangle]).ptr::<()>();
                e.call(0x0051_9270, &args![triangle, 0x1000u32]);
            }
            e.call(0x0098_3720, &args![records, index, 1u32]);
            index = index.wrapping_sub(1);
        }
        index = index.wrapping_add(1);
    }
    true
}

// Translated from 0069a830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the unique neighboring edges around a triangle, plus its stored
/// extra-edge entry when present, to the supplied edge list.
pub fn fn_0069a830(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, edge_list: Ptr) -> u32 {
    let edge_map = this.byte_add(0x88);
    if e.call(0x0069_b080, &args![edge_map]).bool() {
        e.call(0x0069_9b60, &args![this]);
    }
    e.call(0x0084_54f0, &args![edge_list, 0u32]);
    let triangle_data = e.call(0x0055_82f0, &args![this, triangle]).ptr::<()>();
    let mut added = 0;
    for edge_index in 0..3 {
        let edge = e.call(0x0055_82d0, &args![triangle_data, edge_index]).u16();
        let exists = e.with_stack(8, |e, edge_storage| {
            e.mem.set_u16(edge_storage.addr(), edge);
            e.call(
                0x0069_b0a0,
                &args![edge_map, edge, edge_storage.byte_add(4)],
            )
            .bool()
        });
        if exists {
            e.with_stack(2, |e, edge_storage| {
                e.mem.set_u16(edge_storage.addr(), edge);
                e.call(0x0069_1820, &args![edge_list, edge_storage]);
            });
            added += 1;
        }
    }
    let extras = this.byte_add(0x98);
    if (triangle as u32) < e.call(ARRAY_SIZE, &args![extras]).u32() {
        let extra = e.call(0x005e_eda0, &args![extras, triangle]).ptr::<()>();
        if e.mem.u16(extra.addr()) != u16::MAX {
            e.call(0x0069_1820, &args![edge_list, extra]);
            added += 1;
        }
    }
    added
}

// Translated from 0069a940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds neighboring triangle edges that already exist in the map at
/// `NavMesh + 0xe4` to the supplied edge list, returning the number added.
pub fn fn_0069a940(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, edge_list: Ptr) -> u32 {
    e.call(0x0084_54f0, &args![edge_list, 0u32]);
    let triangle_data = e.call(0x0055_82f0, &args![this, triangle]).ptr::<()>();
    let mut added = 0;
    for edge_index in 0..3 {
        let edge = e.call(0x0055_82d0, &args![triangle_data, edge_index]).u16();
        let found = e.with_stack(2, |e, edge_storage| {
            e.mem.set_u16(edge_storage.addr(), edge);
            e.call(
                0x0069_b240,
                &args![this.byte_add(0xe4), edge_storage, 0u32, 0x0069_b1b0u32],
            )
            .u32()
        });
        if found != u32::MAX {
            e.with_stack(2, |e, edge_storage| {
                e.mem.set_u16(edge_storage.addr(), edge);
                e.call(0x0069_1820, &args![edge_list, edge_storage]);
            });
            added += 1;
        }
    }
    added
}

// Translated from 0069a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ensures the obstacle map at `this + 0x88` is not at end, then forwards
/// the key and value to its insertion helper and answers the helper's value
/// (the exe ends on that call and leaves its `EAX`; the caller tests `AL`).
pub fn fn_0069a9d0(e: &mut Engine, this: Ptr, key: u16, value: Ptr) -> bool {
    let map = this.byte_add(0x88);
    if e.call(0x0069_b080, &args![map]).bool() {
        // 0069a9ef: `00699b60` is a thiscall on `this` (ECX loaded at 0069a9ec).
        e.call(0x0069_9b60, &args![this]);
    }
    e.call(0x0069_b0a0, &args![map, key, value]).bool()
}

// Translated from 0069a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs embedded list members at `this + 8` and `this + 0x18`. The
/// empty `NiPoint3` constructor has no effects.
pub fn fn_0069a730(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0069_b2c0, &args![this.byte_add(8)]);
    e.call(0x0069_b2c0, &args![this.byte_add(0x18)]);
    this
}

// Translated from 0069a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the object destructor and deletes `this` when bit zero of `flags`
/// is set.
pub fn fn_0069a7a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0069_a7d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the embedded list members at `this + 0x18` and `this + 8`.
pub fn fn_0069a7d0(e: &mut Engine, this: Ptr) {
    e.call(0x0069_b2f0, &args![this.byte_add(0x18)]);
    e.call(0x0069_b2f0, &args![this.byte_add(8)]);
}

// Translated from 0069aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Repairs the neighboring mesh's portal record when it has one candidate
/// edge or when reversed endpoints identify a unique match. Normal reference
/// cleanup is retained; compiler exception unwinding is omitted.
pub fn fn_0069aa20(e: &mut Engine, this: Ptr<NavMesh>) -> bool {
    let mut triangle_index = 0u32;
    while triangle_index < e.call(0x0055_8200, &args![this]).u32() {
        let triangle = e
            .call(0x0055_82f0, &args![this, triangle_index as u16])
            .u32();
        if triangle != 0 && e.call(0x0069_0e10, &args![triangle]).bool() {
            for edge in 0..3i32 {
                if !e.call(0x0068_f1d0, &args![triangle, edge]).bool() {
                    continue;
                }
                let extra_index = e.call(0x0068_f2c0, &args![triangle, edge]).u16();
                let extra = e.call(0x0069_3d30, &args![this, extra_index]).u32();
                e.with_stack(76, |e, scratch| {
                    let reference = scratch;
                    let start = scratch.byte_add(4);
                    let end = scratch.byte_add(16);
                    let other_start = scratch.byte_add(28);
                    let other_end = scratch.byte_add(40);
                    let difference = scratch.byte_add(52);
                    let second_difference = scratch.byte_add(64);
                    e.call(0x0042_fb00, &args![reference]);
                    let holder = Ptr::new(e.mem.u32(extra + 4));
                    if fn_0069ad00(e, holder, reference) {
                        let other_index = e.mem.u16(extra + 8);
                        let other_mesh = e.call(POINTER_GET, &args![reference]).ptr::<NavMesh>();
                        let other_triangle =
                            e.call(0x0055_82f0, &args![other_mesh, other_index]).u32();
                        if other_triangle != 0 {
                            let mut candidates = 0;
                            let mut chosen = -1i32;
                            for candidate in 0..3i32 {
                                if e.call(0x0068_f1d0, &args![other_triangle, candidate])
                                    .bool()
                                {
                                    candidates += 1;
                                    chosen = candidate;
                                }
                            }
                            if candidates != 0 {
                                e.call(
                                    0x0068_f0c0,
                                    &args![this, triangle_index as u16, edge, start, end],
                                );
                                for candidate in 0..3i32 {
                                    let other_index = e.mem.u16(extra + 8);
                                    let other_mesh = e.call(POINTER_GET, &args![reference]).u32();
                                    e.call(
                                        0x0068_f0c0,
                                        &args![
                                            other_mesh,
                                            other_index,
                                            candidate,
                                            other_start,
                                            other_end
                                        ],
                                    );
                                    let delta = e
                                        .call(0x0043_9ef0, &args![start, difference, other_end])
                                        .u32();
                                    let distance = e.call(0x004a_7290, &args![delta]).f64();
                                    if distance < e.mem.f64(0x0106_b178) {
                                        let delta = e
                                            .call(
                                                0x0043_9ef0,
                                                &args![end, second_difference, other_start],
                                            )
                                            .u32();
                                        let distance = e.call(0x004a_7290, &args![delta]).f64();
                                        if distance < e.mem.f64(0x0106_b178) {
                                            chosen = candidate;
                                            candidates = 1;
                                            break;
                                        }
                                    }
                                }
                            }
                            if candidates == 1 {
                                let other_index = e.mem.u16(extra + 8);
                                let other_mesh =
                                    e.call(POINTER_GET, &args![reference]).ptr::<NavMesh>();
                                let target = fn_006976a0(e, other_mesh, other_index, chosen);
                                let mesh_info = e.call(0x0068_efb0, &args![this]).u32();
                                e.mem.set_u32(target.addr() + 4, mesh_info);
                                e.mem.set_u16(target.addr() + 8, triangle_index as u16);
                                e.mem.set_u32(target.addr(), 3);
                            }
                        }
                    }
                    e.call(0x0042_fa40, &args![reference]);
                });
            }
        }
        triangle_index = triangle_index.wrapping_add(1);
    }
    true
}

// Translated from 0069ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns false when the reference is in the prohibited state; otherwise
/// calls the add-reference helper and reports whether it returned non-null.
pub fn fn_0069ad00(e: &mut Engine, this: Ptr, reference: Ptr) -> bool {
    if e.call(0x0068_f320, &args![this]).bool() {
        return false;
    }
    e.call(
        0x0069_0890,
        &args![reference, Ptr::<()>::new(e.mem.u32(this.addr() + 0x54))],
    );
    let held = e.call(POINTER_GET, &args![reference]).u32();
    held != 0
}

// Translated from 0069ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the base map at `this` with `bucket_count` buckets, sets its
/// vtable, and returns `this`.
pub fn fn_0069ad40(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    e.call(0x0069_b360, &args![this, bucket_count]);
    // 0069ad56: MOV dword ptr [ECX], 0x0106b184 (an immediate address).
    e.mem.set_u32(this.addr(), 0x0106_b184);
    this
}

// Translated from 0069ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the contained map destructor and optionally frees the object.
pub fn fn_0069ad70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0069_b6d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the deleting-destructor vtable and forwards `count` to the embedded
/// array's vector-deleting destructor.
pub fn fn_0069ada0(e: &mut Engine, this: Ptr, array: Ptr, count: u32) -> Ptr {
    // `MOV dword ptr [EAX],0x106abe4`: the vtable's address itself.
    e.mem.set_u32(this.addr(), 0x0106_abe4);
    e.call(0x0069_2290, &args![this, array, count]);
    this
}

// Translated from 0069add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` 16-bit elements starting at `index`, shifting the tail
/// down and reducing the array size.
pub fn fn_0069add0(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let size = e.mem.u32(this.addr() + 8);
    let moved = e
        .call(0x0042_f5a0, &args![size - index - count, count])
        .u32();
    let buffer = e.mem.u32(this.addr() + 4);
    let destination = buffer + index * 2;
    // 0069ae09 / 0069ae39: `0072ba80` (RET 8) and `005efdd0` (RET 0xc) are thiscalls on
    // the array (ECX = `this`, from [EBP-8]).
    e.call(
        0x0072_ba80,
        &args![this, Ptr::<()>::new(destination), count],
    );
    e.call(
        0x005e_fdd0,
        &args![
            this,
            Ptr::<()>::new(destination),
            Ptr::<()>::new(buffer + size * 2 - moved * 2),
            moved,
        ],
    );
    e.mem.set_u32(this.addr() + 8, size - count);
}

// Translated from 0069aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts the 16-bit elements of a `BSSimpleArray` using the supplied C
/// comparison function.
pub fn fn_0069aeb0(e: &mut Engine, this: Ptr, comparator: Ptr) {
    let buffer = e.mem.u32(this.addr() + 4);
    let count = e.mem.u32(this.addr() + 8);
    e.call(
        0x00ec_6f20,
        &args![Ptr::<()>::new(buffer), count, 2u32, comparator],
    );
}

// Translated from 0069aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the embedded 16-bit array with vtable `0106b1a4`.
pub fn fn_0069aee0(e: &mut Engine, this: Ptr) -> Ptr {
    // `MOV dword ptr [EAX],0x106b1a4`: the vtable's address itself.
    e.mem.set_u32(this.addr(), 0x0106_b1a4);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006993c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Processes undo records from newest to oldest using one local memory-DC
/// object, then destroys that object.
pub fn fn_006993c0(e: &mut Engine, this: Ptr<NavMesh>) {
    e.with_stack(0x20, |e, context| {
        fn_00699450(e, context);
        let array = this.byte_add(0xd0);
        let count = e.call(ARRAY_SIZE, &args![array]).u32();
        for index in (0..count).rev() {
            fn_00699000(e, this, index, context);
        }
        fn_006994b0(e, context);
    });
}

// Translated from 00699450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the two embedded map members at offsets zero and 0x10.
pub fn fn_00699450(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0069_af30, &args![this]);
    e.call(0x0069_af30, &args![this.byte_add(0x10)]);
    this
}

// Translated from 006994b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the two embedded map members in reverse declaration order: the
/// one at 0x10 first, then the one at offset zero.
pub fn fn_006994b0(e: &mut Engine, this: Ptr) {
    // 006994dd-006994e3: ECX = this + 0x10, then 006994ef-006994f2: ECX = this.
    e.call(0x0069_af60, &args![this.byte_add(0x10)]);
    e.call(0x0069_af60, &args![this]);
}

// Translated from 00699510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the supplied operation while a cancellation-token source is alive,
/// returning the source's result after destroying it.
pub fn fn_00699510(e: &mut Engine, this: Ptr, operation: Ptr) -> u32 {
    e.with_stack(4, |e, token| {
        e.call(0x0063_3c90, &args![token, operation]);
        let result = e
            .call(0x006c_61b0, &args![this.byte_add(0xd0), token])
            .u32();
        e.call(0x0045_cec0, &args![token]);
        result
    })
}

// Translated from 00698fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the obstacle record for a mesh index and applies the undo at that
/// index. Returns the native `0xffffff00` sentinel when the mesh has no
/// matching record.
pub fn fn_00698fc0(e: &mut Engine, this: Ptr<NavMesh>, key: Ptr, context: Ptr) -> u32 {
    let found = e.with_stack(4, |e, out| fn_00699630(e, this, key, out));
    if found == u32::MAX {
        0xffff_ff00
    } else {
        fn_00699000(e, this, found, context)
    }
}

// Translated from 00699590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes one item from the obstacle undo array at `index`.
pub fn fn_00699590(e: &mut Engine, this: Ptr<NavMesh>, index: u32) {
    e.call(0x0069_b120, &args![this.byte_add(0xd0), index, 1]);
}

// Translated from 006995c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first obstacle undo entry whose held form pointer matches
/// `form`, or -1 when no entry matches.
pub fn fn_006995c0(e: &mut Engine, this: Ptr<NavMesh>, form: Ptr) -> u32 {
    let array = this.byte_add(0xd0);
    let mut index = 0u32;
    // 006995e4: the count (`0044ddc0`) is read before each element.
    while index < e.call(ARRAY_SIZE, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT_4, &args![array, index]).u32();
        let pointer = e.call(POINTER_GET, &args![element]).u32();
        if e.call(0x004b_0460, &args![Ptr::<()>::new(pointer + 0x0c), form])
            .bool()
        {
            return index;
        }
        index += 1;
    }
    u32::MAX
}

// Translated from 00699630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the obstacle record containing the mesh. On failure writes null
/// to `out_record` and returns -1; on success writes the record pointer.
pub fn fn_00699630(e: &mut Engine, this: Ptr<NavMesh>, key: Ptr, out_record: Ptr) -> u32 {
    let index = fn_006995c0(e, this, key);
    if index == u32::MAX {
        e.mem.set_u32(out_record.addr(), 0);
        u32::MAX
    } else {
        let element = e
            .call(ARRAY_ELEMENT_4, &args![this.byte_add(0xd0), index])
            .u32();
        let record = e.call(0x0055_9450, &args![element]).u32();
        e.mem.set_u32(out_record.addr(), record);
        index
    }
}

// Translated from 00699760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the greater of an unsigned word and an unsigned 32-bit value.
pub fn fn_00699760(_e: &mut Engine, value: u32, candidate: u16) -> u32 {
    value.max(candidate as u32)
}

// Translated from 00699790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the three-bit type encoded in bits 13 through 15 of the word at
/// `this + 0xC`.
pub fn fn_00699790(e: &mut Engine, this: Ptr) -> u32 {
    (e.mem.u32(this.addr() + 0x0c) & 0xe000) >> 13
}

// Translated from 0069a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a 6-byte edge record: 32-bit first value, then 16-bit edge.
pub fn fn_0069a660(e: &mut Engine, this: Ptr, value: u32, edge: u16) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    e.mem.set_u16(this.addr() + 4, edge);
    this
}

// Translated from 0069a690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 6-byte edge record at `other` to `this`.
pub fn fn_0069a690(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let value = e.mem.u32(other.addr());
    let edge = e.mem.u16(other.addr() + 4);
    e.mem.set_u32(this.addr(), value);
    e.mem.set_u16(this.addr() + 4, edge);
    this
}

// Translated from 0069a6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an edge record with a 32-bit value at +0, a word at +4,
/// and another 32-bit value at +8.
pub fn fn_0069a6c0(e: &mut Engine, this: Ptr, value: u32, edge: u16, extra: u32) -> Ptr {
    e.mem.set_u32(this.addr(), value);
    e.mem.set_u16(this.addr() + 4, edge);
    e.mem.set_u32(this.addr() + 8, extra);
    this
}

// Translated from 0069a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the low 14 bits of the word at `record + 4` and the low two bits
/// of the signed word at `record + 8` into one edge descriptor.
pub fn fn_0069a700(_e: &mut Engine, out: Ptr, record: Ptr) {
    let low = _e.mem.u16(record.addr() + 4) & 0x3fff;
    let high = (_e.mem.u16(record.addr() + 8) & 3) << 14;
    _e.mem.set_u16(out.addr(), low | high);
}

// Translated from 0069ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the unsigned 16-bit keys at `first` and `second`: 1 when the first
/// is greater (`CMP`/`JLE` at 0069ae7e), -1 when the second is greater
/// (0069ae97), 0 when they are equal (#197).
pub fn fn_0069ae60(e: &mut Engine, first: Ptr, second: Ptr) -> u32 {
    match e.mem.u16(first.addr()).cmp(&e.mem.u16(second.addr())) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => u32::MAX,
        std::cmp::Ordering::Equal => 0,
    }
}

// Translated from 0069af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `BSSimpleArray<NavMeshInfo*>` used by the obstacle query.
pub fn fn_0069af10(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), NAV_MESH_INFO_POINTER_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
    this
}

// Translated from 0069af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the `BSSimpleArray<NavMeshEdgeHandle>` storage at `this`.
pub fn fn_0069af30(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
    e.call(0x0069_ba70, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0069af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `BSSimpleArray<NavMeshEdgeHandle>` storage at `this`.
pub fn fn_0069af60(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 0069af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates storage for `count` three-float points (`count * 0xC`).
pub fn fn_0069af80(e: &mut Engine, _this: Ptr, count: u32) -> Ptr {
    Ptr::new(e.call(0x0040_1000, &args![count.wrapping_mul(12)]).u32())
}

// Translated from 0069afb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned short, NiPointer<ObstacleData>>>,
/// unsigned short, NiPointer<ObstacleData>>::GetAt` (Xbox PDB): assigns the
/// matching item's `NiPointer` (the item's +8, as `SetValue` 00691930 stores
/// it) to `out` with `NiPointer::operator=` `006e5cc0`, returning whether it
/// was found.
pub fn ni_t_map_base_obstacle_get_at(e: &mut Engine, this: Ptr, key: u16, out: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), 4, &args![u32::from(key)]).u32();
    let table = e.mem.u32(this.addr() + 8);
    let mut item = e.mem.u32(table + bucket * 4);
    while item != 0 {
        let stored_key = e.mem.u16(item + 4);
        if e.vcall(
            this.addr(),
            8,
            &args![u32::from(key), u32::from(stored_key)],
        )
        .bool()
        {
            // 0069b004..0069b00e: `ADD ECX,0x8; PUSH ECX; MOV ECX,[EBP+0xc]; CALL 006e5cc0`.
            e.call(0x006e_5cc0, &args![out, item + 8]);
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 0069b030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the six-byte edge handle at `value` to the array and returns its
/// former size.
pub fn fn_0069b030(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let index = e.call(0x0084_5460, &args![this]).u32();
    let target = e.mem.u32(this.addr() + 4) + index * 8;
    e.call(0x0069_27d0, &args![this, target, 1u32]);
    // The constructor call may relocate the backing buffer; PC reloads +4.
    let target = e.mem.u32(this.addr() + 4) + index * 8;
    fn_0069a690(e, Ptr::new(target), value);
    index
}

// Translated from 0069b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns whether the map's item count at `+0xC` is zero.
pub fn fn_0069b080(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xc) == 0
}

// Translated from 0069b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a 16-bit key and copies the item's packed value at `item + 6` to
/// `out`.
pub fn fn_0069b0a0(e: &mut Engine, this: Ptr, key: u16, out: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), 4, &args![u32::from(key)]).u32();
    let table = e.mem.u32(this.addr() + 8);
    let mut item = e.mem.u32(table + bucket * 4);
    while item != 0 {
        let stored_key = e.mem.u16(item + 4);
        if e.vcall(
            this.addr(),
            8,
            &args![u32::from(key), u32::from(stored_key)],
        )
        .bool()
        {
            e.mem.set_u32(out.addr(), e.mem.u32(item + 6));
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 0069b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `count` four-byte entries at `index`, destroying removed objects
/// and shifting the following entries down.
pub fn fn_0069b120(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let size = e.mem.u32(this.addr() + 8);
    let tail = size.wrapping_sub(index).wrapping_sub(count);
    let moved = e.call(0x0042_f5a0, &args![tail, count]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    // 0069b159 / 0069b18a: `00822820` (RET 8) and `0042fb60` (RET 0xc) are thiscalls on
    // the array (ECX = `this`, from [EBP-8]).
    e.call(0x0082_2820, &args![this, buffer + index * 4, count]);
    e.call(
        0x0042_fb60,
        &args![
            this,
            buffer + index * 4,
            buffer + size * 4 - moved * 4,
            moved
        ],
    );
    e.mem.set_u32(this.addr() + 8, size.wrapping_sub(count));
}

// Translated from 0069b1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl equality comparator for pointers to 16-bit triangle indices.
pub fn fn_0069b1b0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    e.mem.u16(first.addr()) == e.mem.u16(second.addr())
}

// Translated from 0069b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `BSSimpleArray<bool>` storage at `this`.
pub fn fn_0069b1d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BOOLEAN_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 0069b1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates storage for `count` 12-byte fast edge handles.
pub fn fn_0069b1f0(e: &mut Engine, _this: Ptr, count: u32) -> Ptr {
    Ptr::new(e.call(0x0040_1000, &args![count.wrapping_mul(12)]).u32())
}

// Translated from 0069b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `BSSimpleArray` reallocation (its vtable slot `+0xC`, `_Reallocate` in
/// the Xbox PDB): forwards the block and the byte size to the reallocation
/// `0042f5d0` and answers the new block (the exe ends on that call and leaves
/// its `EAX`); ECX (`this`) is unused by the body.
pub fn fn_0069b220(e: &mut Engine, _this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    e.call(0x0042_f5d0, &args![first, second]).ptr()
}

// Translated from 0069b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds a 16-bit value in a `BSSimpleArray`, calling `comparator(element,
/// key)` from `start`; returns -1 when there is no match.
pub fn fn_0069b240(e: &mut Engine, this: Ptr, key: Ptr, start: u32, comparator: u32) -> i32 {
    let count = e.mem.u32(this.addr() + 8);
    let buffer = e.mem.u32(this.addr() + 4);
    if buffer == 0 {
        return -1;
    }
    for index in start..count {
        if e.call(comparator, &args![buffer + index * 2, key]).bool() {
            return index as i32;
        }
    }
    -1
}

// Translated from 0069b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares the unsigned 16-bit value and 32-bit value at the two pointers.
pub fn fn_0069b2a0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    u32::from(e.mem.u16(first.addr())) == e.mem.u32(second.addr())
}

// Translated from 0069b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `BSSimpleArray<FastNavMeshEdgeHandle>` with an empty buffer.
pub fn fn_0069b2c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
    e.call(0x0069_ba70, &args![this, 0u32, 0u32]);
    this
}

// Translated from 0069b2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys `BSSimpleArray<FastNavMeshEdgeHandle>` at `this`.
pub fn fn_0069b2f0(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 0069b310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a 12-byte fast edge handle to the array, returning its old size.
pub fn fn_0069b310(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let index = e.call(0x0097_8bc0, &args![this]).u32();
    let element = e.mem.u32(this.addr() + 4) + index * 12;
    e.call(0x0069_b9d0, &args![this, element, 1u32]);
    e.call(0x006b_c870, &args![element, value]);
    index
}

// Translated from 0069b360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base constructor for the fast-vertex `NiTMap`: allocate and zero its
/// bucket table, set hash size/count and install the base vtable.
pub fn fn_0069b360(e: &mut Engine, this: Ptr, bucket_count: u32) -> Ptr {
    let bytes = bucket_count.wrapping_mul(4);
    e.mem
        .set_u32(this.addr(), FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, bucket_count);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let buckets = e.call(0x00aa_1070, &args![bytes]).u32();
    e.mem.set_u32(this.addr() + 8, buckets);
    e.call(0x0040_3d30, &args![buckets, 0u32, bytes]);
    this
}

// Translated from 0069b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Inserts or updates the fast-vertex map item for its two-word key.
pub fn fn_0069b3d0(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32, value: u32) {
    let bucket = e.vcall(this.addr(), 4, &args![key_low, key_high]).u32();
    let buckets = e.mem.u32(this.addr() + 8);
    let slot = buckets + bucket * 4;
    let mut item = e.mem.u32(slot);
    while item != 0 {
        if e.vcall(
            this.addr(),
            8,
            &args![e.mem.u32(item + 4), e.mem.u32(item + 8), key_low, key_high],
        )
        .bool()
        {
            e.mem.set_u32(item + 0xc, value);
            return;
        }
        item = e.mem.u32(item);
    }
    let item = e.vcall(this.addr(), 0x14, &[]).u32();
    // The native code constructs the key into a temporary and passes it by
    // value with `item` and `value` to the slot +0xC SetValue method.
    e.vcall(this.addr(), 0xc, &args![item, key_low, key_high, value]);
    e.mem.set_u32(item, e.mem.u32(slot));
    e.mem.set_u32(slot, item);
    let count = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(this.addr() + 0xc, count.wrapping_add(1));
}

// Translated from 0069b4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the fast-vertex map value for `key` and writes it to `out`.
pub fn fn_0069b4d0(e: &mut Engine, this: Ptr, key_low: u32, key_high: u32, out: Ptr) -> bool {
    let bucket = e.vcall(this.addr(), 4, &args![key_low, key_high]).u32();
    let mut item = e.mem.u32(e.mem.u32(this.addr() + 8) + bucket * 4);
    while item != 0 {
        if e.vcall(
            this.addr(),
            8,
            &args![e.mem.u32(item + 4), e.mem.u32(item + 8), key_low, key_high],
        )
        .bool()
        {
            e.mem.set_u32(out.addr(), e.mem.u32(item + 0xc));
            return true;
        }
        item = e.mem.u32(item);
    }
    false
}

// Translated from 0069b560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances the fast-vertex map iterator and copies the current key/value.
pub fn fn_0069b560(e: &mut Engine, this: Ptr, iterator: Ptr, key_out: Ptr, value_out: Ptr) {
    let item = e.mem.u32(iterator.addr());
    e.call(0x0069_a690, &args![key_out, item + 4]);
    e.mem.set_u32(value_out.addr(), e.mem.u32(item + 0xc));
    let next = e.mem.u32(item);
    if next != 0 {
        e.mem.set_u32(iterator.addr(), next);
        return;
    }
    let low = e.mem.u32(key_out.addr());
    let high = e.mem.u32(key_out.addr() + 4);
    let start = e
        .vcall(this.addr(), 4, &args![low, high])
        .u32()
        .wrapping_add(1);
    let bucket_count = e.mem.u32(this.addr() + 4);
    let buckets = e.mem.u32(this.addr() + 8);
    for bucket in start..bucket_count {
        let candidate = e.mem.u32(buckets + bucket * 4);
        if candidate != 0 {
            e.mem.set_u32(iterator.addr(), candidate);
            return;
        }
    }
    e.mem.set_u32(iterator.addr(), 0);
}

// Translated from 0069b620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hashes a fast-vertex handle and reduces it by the bucket count.
pub fn ni_t_map_fast_vertex_key_to_hash_index(
    e: &mut Engine,
    this: Ptr,
    low: u32,
    high: u32,
) -> u32 {
    let hash = e.with_stack(8, |e, key| {
        e.mem.set_u32(key.addr(), low);
        e.mem.set_u32(key.addr() + 4, high);
        fn_0069b640(e, key)
    });
    // `DIV` in 0069b620 clears EDX before dividing, so only EAX (the low
    // word of this 64-bit hash) participates in the unsigned remainder.
    (hash as u32) % e.mem.u32(this.addr() + 4)
}

// Translated from 0069b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 64-bit hash accumulator for the fast-vertex key.
pub fn fn_0069b640(e: &mut Engine, key: Ptr) -> u64 {
    let low = e.mem.u32(key.addr());
    let high = e.mem.u16(key.addr() + 4);
    let shifted = i64::from(low as i32).wrapping_shl(32);
    let shifted_high_word = (u32::from(high) << 16) as i32;
    shifted.wrapping_add(i64::from(shifted_high_word)) as u64
}

// Translated from 0069b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares two two-word fast-vertex keys through the game's key comparator.
pub fn ni_t_map_fast_vertex_is_keys_equal(
    e: &mut Engine,
    _this: Ptr,
    first_low: u32,
    first_high: u32,
    second_low: u32,
    second_high: u32,
) -> bool {
    e.with_stack(16, |e, keys| {
        e.mem.set_u32(keys.addr(), first_low);
        e.mem.set_u32(keys.addr() + 4, first_high);
        e.mem.set_u32(keys.addr() + 8, second_low);
        e.mem.set_u32(keys.addr() + 12, second_high);
        e.call(0x006d_4ca0, &args![keys, keys.byte_add(8)]).bool()
    })
}

// Translated from 0069b6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a fast-vertex key in the map item and writes its value at `+0xC`.
pub fn ni_t_map_fast_vertex_set_value(
    e: &mut Engine,
    _this: Ptr,
    item: Ptr,
    key_low: u32,
    key_high: u32,
    value: u32,
) {
    e.with_stack(8, |e, key| {
        e.mem.set_u32(key.addr(), key_low);
        e.mem.set_u32(key.addr() + 4, key_high);
        e.call(0x0069_a690, &args![item.byte_add(4), key]);
    });
    e.mem.set_u32(item.addr() + 0xc, value);
}

// Translated from 0069b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the fast-vertex map: removes all items and frees its buckets.
pub fn fn_0069b6d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_b184);
    e.call(0x0043_8af0, &args![this]);
    fn_0069b730(e, this);
}

// Translated from 0069b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the map base and frees its bucket table.
pub fn fn_0069b730(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE);
    e.call(0x0043_8af0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(0x00aa_10f0, &args![buckets]);
}

// Translated from 0069b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates and constructs a 16-byte map item.
pub fn fn_0069b760(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.call(0x0069_b8a0, &args![this.byte_add(0xc)]).u32())
}

// Translated from 0069b780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a map item's value and returns it to the allocator.
pub fn fn_0069b780(e: &mut Engine, this: Ptr, item: Ptr) {
    e.mem.set_u32(item.addr() + 0xc, 0);
    e.call(0x006b_8310, &args![this.byte_add(0xc), item]);
}

// Translated from 0069b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar-deleting destructor for `BSSimpleArray<NavMeshInfo*>`.
pub fn fn_0069b7b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069af10(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar-deleting destructor for `BSSimpleArray<NavMeshEdgeHandle>`.
pub fn fn_0069b7e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069af60(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar-deleting destructor for `BSSimpleArray<bool>`.
pub fn fn_0069b810(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069b1d0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar-deleting destructor for `BSSimpleArray<FastNavMeshEdgeHandle>`.
pub fn fn_0069b840(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069b2f0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069b870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar-deleting destructor for the fast-vertex map base.
pub fn fn_0069b870(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0069b730(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0069b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a 16-byte fast-vertex map item and constructs it when non-null.
pub fn fn_0069b8a0(e: &mut Engine, _this: Ptr) -> Ptr {
    let item = e.call(0x0040_1000, &args![0x10u32]).u32();
    if item == 0 {
        Ptr::NULL
    } else {
        e.call(0x0069_b920, &args![Ptr::<()>::new(item)]).ptr()
    }
}

/// This unit's translated functions, by exe address.
// Translated from 006bc8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the mesh and undo data, resets the operation, and reserves the
/// mesh's vertex and triangle arrays from the undo record's counts.
pub fn fn_006bc8e0(e: &mut Engine, this: Ptr, mesh: Ptr, undo: Ptr) -> bool {
    e.mem.set_u32(this.addr(), mesh.addr());
    e.mem.set_u32(this.addr() + 8, undo.addr());
    e.call(0x006b_ebd0, &args![this]);
    let undo = e.mem.u32(this.addr() + 8);
    let mesh = e.mem.u32(this.addr());
    let vertices = e.mem.u32(undo + 0x10);
    e.call(0x0069_4890, &args![mesh + 0x28, vertices, 1u32]);
    let undo = e.mem.u32(this.addr() + 8);
    let mesh = e.mem.u32(this.addr());
    let triangles = e.mem.u32(undo + 0x18);
    e.call(0x0069_49d0, &args![mesh + 0x38, triangles, 1u32]);
    true
}

// Translated from 006bd040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` the cutter: the navmesh at +0, the obstacle's
/// `NiPointer` at +4; plain `RET`): adds the obstacle's footprint to the cutter. The four
/// corners (x and y from the obstacle's +0x40/+0x44 and +0x4c/+0x50, z its +0x48, each built
/// by `00416870` in a temporary and copied) are turned by the obstacle's +0x1c matrix
/// (`004b4500`) and moved by its +0x10 translation (`00439e90`); `00697b70` on the navmesh may
/// replace each z. Each corner becomes a vertex (`00690fe0` on a `006240d0` copy of it, then
/// `006bf3c0` on the cutter with 1, 0, 0, appended to +0x1c) and each side a kind-1 edge
/// (`006bf4d0` on the cutter, appended to +0x30). An edge whose squared 2D length
/// (`00595c80` of its second end's position minus its first's, `00439ef0`) is above 64.0
/// gets the height index (`006ad3b0` of the obstacle's +0x6c minus +0x60) at +0x16, 1 at
/// +0x18 and +0x19, and the obstacle (`006e5cc0`) at +0x1c.
// Translation incomplete: 006bd040 - exception unwind cleanup for temporary point arrays and smart pointers.
pub fn fn_006bd040(e: &mut Engine, this: Ptr) {
    let obstacle_pointer = this.byte_add(4);
    // The frame's 0x108 bytes of locals, each at its `EBP` offset.
    e.with_stack(0x108, |e, frame| {
        let local = |offset: u32| frame.addr() + 0x108 - offset;
        let copy_point = |e: &mut Engine, from: u32, to: u32| {
            for word in 0..3u32 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(to + 4 * word, value);
            }
        };
        let corners = local(0x40);
        let vertices = local(0x10);
        let projected = local(0x78);
        let z_out = local(0x90);
        let record = local(0xa0);
        let edge_slot = local(0xac);
        let rotated = local(0xec);
        let moved = local(0xf8);
        let difference = local(0x104);
        // 006bd050..006bd05d: `_vector_constructor_iterator_` over the corners (0xc, 4, `006815c0`).
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![corners, 0xcu32, 4u32, POINT3_DEFAULT_CONSTRUCTOR],
        );
        // 006bd062..006bd1cb: z (+0x48), y, x pushed (`FLD`/`FSTP` float), each corner built in
        // its temporary (EBP-0xbc, -0xc8, -0xd4, -0xe0) and copied into the array.
        for (index, (temporary, x_offset, y_offset)) in [
            (0xbcu32, 0x40u32, 0x44u32),
            (0xc8, 0x40, 0x50),
            (0xd4, 0x4c, 0x50),
            (0xe0, 0x4c, 0x44),
        ]
        .into_iter()
        .enumerate()
        {
            let obstacle = e.call(POINTER_GET, &args![obstacle_pointer]).u32();
            let z = x87_quiet_bits(e.mem.u32(obstacle + 0x48));
            let obstacle = e.call(POINTER_GET, &args![obstacle_pointer]).u32();
            let y = x87_quiet_bits(e.mem.u32(obstacle + y_offset));
            let obstacle = e.call(POINTER_GET, &args![obstacle_pointer]).u32();
            let x = x87_quiet_bits(e.mem.u32(obstacle + x_offset));
            let built = e
                .call(POINT3_CONSTRUCT, &args![local(temporary), x, y, z])
                .u32();
            copy_point(e, built, corners + index as u32 * 12);
        }

        // 006bd1ce..006bd1f2: `FLD [ESI+0x6c]; FSUB [EAX+0x60]; FSTP` (ESI the first getter's
        // object, EAX the second's).
        let first_obstacle = e.call(POINTER_GET, &args![obstacle_pointer]).u32();
        let second_obstacle = e.call(POINTER_GET, &args![obstacle_pointer]).u32();
        let height_delta = x87_store32(x87_sub(
            x87_load32(e.mem.u32(first_obstacle + 0x6c)),
            x87_m32(e.mem.u32(second_obstacle + 0x60)),
        ));
        let height_index = e
            .call(
                0x006a_d3b0,
                &args![x87_quiet_bits(height_delta), 1u32, 0u32],
            )
            .u16();

        // 006bd20c..006bd219: `_vector_constructor_iterator_` over the projected corners.
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![projected, 0xcu32, 4u32, POINT3_DEFAULT_CONSTRUCTOR],
        );
        for index in 0..4u32 {
            let corner = corners + index * 12;
            let point = projected + index * 12;
            // 006bd246..006bd2ad: the first getter's +0x10 (the translation) is pushed before
            // the second getter's +0x1c (the matrix); `004b4500` into EBP-0xec, `00439e90` into
            // EBP-0xf8, copied into the projected corner.
            let translation = e.call(POINTER_GET, &args![obstacle_pointer]).u32() + 0x10;
            let matrix = e.call(POINTER_GET, &args![obstacle_pointer]).u32() + 0x1c;
            let turned = e.call(0x004b_4500, &args![matrix, rotated, corner]).u32();
            let sum = e.call(POINT3_ADD, &args![turned, moved, translation]).u32();
            copy_point(e, sum, point);
            // 006bd2b0..006bd30c: the z into EBP-0x90, `00697b70` on the navmesh with the
            // corner, the float at 0101e704, the height difference and that z (each float
            // `FLD`/`FSTP`); a true answer stores the z back.
            e.mem.set_u32(z_out, x87_quiet_bits(e.mem.u32(point + 8)));
            let limit = x87_quiet_bits(e.global::<u32>(0x0101_e704));
            let navmesh = e.mem.u32(this.addr());
            if e.call(
                0x0069_7b70,
                &args![navmesh, point, limit, x87_quiet_bits(height_delta), z_out],
            )
            .bool()
            {
                e.mem.set_u32(point + 8, x87_quiet_bits(e.mem.u32(z_out)));
            }
        }

        for index in 0..4u32 {
            // 006bd33d..006bd37e: `00690fe0` on a `006240d0` record (EBP-0xa0) holding a copy
            // of the corner.
            e.call(0x0062_40d0, &args![record]);
            copy_point(e, projected + index * 12, record);
            let navmesh = e.mem.u32(this.addr());
            let vertex_index = u32::from(e.call(0x0069_0fe0, &args![navmesh, record]).u16());
            // 006bd398..006bd39e: `MOV ECX,[EBP-0x108]`: the cutter (not the navmesh) makes
            // the vertex, from its scrap heap at +0xc.
            let vertex = e
                .call(0x006b_f3c0, &args![this, vertex_index, 1u32, 0u32, 0u32])
                .u32();
            let slot = vertices + index * 4;
            e.mem.set_u32(slot, vertex);
            e.call(ARRAY_ADD_DWORD, &args![this.byte_add(0x1c), slot]);
        }

        let mut edges = [0u32; 4];
        for index in 0..4u32 {
            let from = e.mem.u32(vertices + index * 4);
            let to = e.mem.u32(vertices + (index + 1) % 4 * 4);
            // 006bd413..006bd419: `MOV ECX,[EBP-0x108]`: the cutter makes the edge.
            let edge = e.call(0x006b_f4d0, &args![this, from, to, 1u32]).u32();
            e.mem.set_u32(edge_slot, edge);
            edges[index as usize] = edge;
            e.call(ARRAY_ADD_DWORD, &args![this.byte_add(0x30), edge_slot]);
        }

        for edge in edges {
            // 006bd476..006bd4c8: the first end's position is pushed, then `00439ef0` on the
            // second end's position (the second minus the first) into EBP-0x104.
            let navmesh = e.mem.u32(this.addr());
            let first_index = u32::from(e.mem.u16(e.mem.u32(edge)));
            let first_position = e.call(VERTEX_POINTER, &args![navmesh, first_index]).u32();
            let navmesh = e.mem.u32(this.addr());
            let second_index = u32::from(e.mem.u16(e.mem.u32(edge + 4)));
            let second_position = e.call(VERTEX_POINTER, &args![navmesh, second_index]).u32();
            let vector = e
                .call(
                    POINT3_SUBTRACT,
                    &args![second_position, difference, first_position],
                )
                .u32();
            let length = e.call(POINT3_SQUARED_LENGTH_2D, &args![vector]).f64();
            // 006bd4cd..006bd4d8: `FCOMP qword [010240c0]; TEST AH,0x41; JNZ`: only above 64.0
            // (not equal, not unordered).
            if length > e.global::<f64>(SIXTY_FOUR) {
                e.mem.set_u16(edge + 0x16, height_index);
                e.mem.set_u8(edge + 0x18, 1);
                e.mem.set_u8(edge + 0x19, 1);
                e.call(0x006e_5cc0, &args![edge + 0x1c, obstacle_pointer]);
            }
        }
    });
}

// Translated from 006987d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds triangle indices from the cells covered by a transformed obstacle
/// footprint when the triangle is not disabled, its bounds overlap, and its
/// center, vertical ray, or one of its edges intersects the footprint. The
/// three stack arguments are the reference smart-pointer, the adjustment
/// point, and the output triangle array. C++ exception unwinding is omitted.
// Translation incomplete: 006987d0 - exception unwind cleanup for temporary vectors and references.
pub fn fn_006987d0(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    reference: Ptr,
    adjustment: Ptr,
    triangles: Ptr,
) {
    e.with_stack(0x150, |e, scratch| {
        let grid = mesh_grid(this);
        let min_row = scratch.byte_add(0x00);
        let min_column = scratch.byte_add(0x04);
        let max_row = scratch.byte_add(0x08);
        let max_column = scratch.byte_add(0x0c);
        let reference_object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
        e.call(
            GRID_CELL_OF_POINT,
            &args![grid, reference_object.byte_add(0x58), min_row, min_column],
        );
        let reference_object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
        e.call(
            GRID_CELL_OF_POINT,
            &args![grid, reference_object.byte_add(0x64), max_row, max_column],
        );

        let corners = scratch.byte_add(0x10);
        for (index, (x_offset, y_offset, y_sign, x_sign)) in [
            (0x40, 0x44, -1.0f32, -1.0f32),
            (0x40, 0x50, 1.0, -1.0),
            (0x4c, 0x50, 1.0, 1.0),
            (0x4c, 0x44, -1.0, 1.0),
        ]
        .into_iter()
        .enumerate()
        {
            let object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
            let z = e.mem.f32(object.addr() + 0x48);
            let object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
            let y = e.mem.f32(object.addr() + y_offset) + e.mem.f32(adjustment.addr() + 4) * y_sign;
            let object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
            let x = e.mem.f32(object.addr() + x_offset) + e.mem.f32(adjustment.addr()) * x_sign;
            e.call(
                POINT3_CONSTRUCT,
                &args![corners.byte_add(index as u32 * 12), x, y, z],
            );
        }
        let first = e.call(POINTER_GET, &args![reference]).ptr::<()>();
        let second = e.call(POINTER_GET, &args![reference]).ptr::<()>();
        let height_delta = e.mem.f32(first.addr() + 0x6c) - e.mem.f32(second.addr() + 0x60);

        let mut row = e.mem.u32(min_row.addr());
        let last_row = e.mem.u32(max_row.addr());
        while row <= last_row {
            let mut column = e.mem.u32(min_column.addr());
            let last_column = e.mem.u32(max_column.addr());
            while column <= last_column {
                let cell_triangles = fn_006967c0(e, grid, column, row);
                if !cell_triangles.is_null() {
                    let mut item_index = 0;
                    while item_index < array_size(e, cell_triangles) {
                        let item = e
                            .call(ARRAY_ELEMENT_2, &args![cell_triangles, item_index])
                            .ptr::<()>();
                        if e.call(
                            ARRAY_FIND_WORD,
                            &args![triangles, item, 0u32, WORD_EQUALS_COMPARATOR],
                        )
                        .i32()
                            == -1
                        {
                            let triangle_index = e.mem.u16(item.addr());
                            let triangle = triangle_of(e, this, triangle_index as u32);
                            if !e.call(TRIANGLE_FLAGS_SET, &args![triangle, 0x20u32]).bool() {
                                let low = scratch.byte_add(0x70);
                                let high = scratch.byte_add(0x7c);
                                let float_max: f32 = e.global(FLOAT_MAX);
                                let neg_float_max: f32 = e.global(NEGATIVE_FLOAT_MAX);
                                write_point(e, low.addr(), [float_max; 3]);
                                write_point(e, high.addr(), [neg_float_max; 3]);
                                for vertex_index in 0..3u32 {
                                    let vertex = corner_pointer(e, this, triangle, vertex_index);
                                    for axis in 0..3u32 {
                                        let component = e.mem.f32(vertex.addr() + axis * 4);
                                        let previous_low = e.mem.f32(low.addr() + axis * 4);
                                        let previous_high = e.mem.f32(high.addr() + axis * 4);
                                        let low_value = e
                                            .call(0x0040_ebd0, &args![previous_low, component])
                                            .f32();
                                        let high_value = e
                                            .call(0x0040_4010, &args![previous_high, component])
                                            .f32();
                                        e.mem.set_f32(low.addr() + axis * 4, low_value);
                                        e.mem.set_f32(high.addr() + axis * 4, high_value);
                                    }
                                }
                                let z_pad: f64 = e.global(0x0102_e430);
                                let max_z = (e.mem.f32(high.addr() + 8) as f64 + z_pad) as f32;
                                e.mem.set_f32(high.addr() + 8, max_z);
                                let object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                let transformed_center = scratch.byte_add(0x88);
                                e.call(
                                    POINT3_ADD,
                                    &args![object.byte_add(0x64), transformed_center, adjustment],
                                );
                                let object = e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                let transformed_origin = scratch.byte_add(0x94);
                                e.call(
                                    POINT3_SUBTRACT,
                                    &args![object.byte_add(0x58), transformed_origin, adjustment],
                                );
                                let bounds_overlap = e
                                    .call(
                                        0x006d_93d0,
                                        &args![transformed_origin, transformed_center, low, high],
                                    )
                                    .bool();
                                if bounds_overlap {
                                    // 00698cb7..00698cc5: the triangle is pushed first, then the
                                    // out point (EBP-0x114): GetCenter(out, triangle).
                                    let accepted = e.call(
                                        GET_CENTER,
                                        &args![this, scratch.byte_add(0xac), triangle_index],
                                    );
                                    let center = accepted.ptr::<()>();
                                    let center_inside = e
                                        .call(
                                            0x006c_3570,
                                            &args![center, reference, height_delta, 128.0f32],
                                        )
                                        .bool();
                                    let mut intersects = center_inside;
                                    if !intersects {
                                        let object =
                                            e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                        let probe_point = object.byte_add(0x10);
                                        let height_out = scratch.byte_add(0x64);
                                        intersects = e
                                            .call(
                                                0x0069_7730,
                                                &args![
                                                    this,
                                                    triangle_index,
                                                    probe_point,
                                                    height_out,
                                                    height_delta,
                                                    128.0f32
                                                ],
                                            )
                                            .bool();
                                    }
                                    if !intersects {
                                        for edge in 0..3u32 {
                                            let next = if edge == 2 { 0 } else { edge + 1 };
                                            let first_diff = scratch.byte_add(0xb8);
                                            let first_local = scratch.byte_add(0xd0);
                                            let matrix_object =
                                                e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                            // 00698d81: ADD EAX,0x1c; the rotation matrix
                                            // 004b3ae0 reads is the object's +0x1c.
                                            let transform = matrix_object.byte_add(0x1c);
                                            let translation_object =
                                                e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                            let translation = translation_object.byte_add(0x10);
                                            let first_vertex =
                                                corner_pointer(e, this, triangle, edge);
                                            e.call(
                                                POINT3_SUBTRACT,
                                                &args![first_vertex, first_diff, translation],
                                            );
                                            let first_local = e
                                                .call(
                                                    0x004b_3ae0,
                                                    &args![first_local, first_diff, transform],
                                                )
                                                .ptr();
                                            let second_diff = scratch.byte_add(0xc4);
                                            let second_local = scratch.byte_add(0xdc);
                                            let matrix_object =
                                                e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                            // 00698dd5: ADD EAX,0x1c, as for the first vertex.
                                            let transform = matrix_object.byte_add(0x1c);
                                            let translation_object =
                                                e.call(POINTER_GET, &args![reference]).ptr::<()>();
                                            let translation = translation_object.byte_add(0x10);
                                            let second_vertex =
                                                corner_pointer(e, this, triangle, next);
                                            e.call(
                                                POINT3_SUBTRACT,
                                                &args![second_vertex, second_diff, translation],
                                            );
                                            let second_local = e
                                                .call(
                                                    0x004b_3ae0,
                                                    &args![second_local, second_diff, transform],
                                                )
                                                .ptr();
                                            for obstacle_edge in 0..4u32 {
                                                let obstacle_next = (obstacle_edge + 1) % 4;
                                                if fn_00696540(
                                                    e,
                                                    first_local,
                                                    second_local,
                                                    corners.byte_add(obstacle_edge * 12),
                                                    corners.byte_add(obstacle_next * 12),
                                                ) {
                                                    intersects = true;
                                                    break;
                                                }
                                            }
                                            if intersects {
                                                break;
                                            }
                                        }
                                    }
                                    if intersects {
                                        e.call(ARRAY_ADD_WORD, &args![triangles, item]);
                                    }
                                }
                            }
                        }
                        item_index += 1;
                    }
                }
                column = column.wrapping_add(1);
            }
            row = row.wrapping_add(1);
        }
    });
}

// Translated from 00699b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the mesh's boundary-edge map and its preferred-vertex array.
/// The temporary map groups ordinary and portal edges by endpoint. When a
/// vertex has exactly two edges, the PC code tests their winding and writes
/// the packed edge pair at that vertex into the member map at `+0x88`.
/// Each triangle then chooses a candidate vertex from that map, subject to
/// the planar-distance and vertical-distance limits.
///
/// Compiler SEH unwinding is omitted; normal-path record and map destruction
/// follows the PC disassembly.
// Translation incomplete: 00699b60 - compiler SEH unwind cleanup for allocated records and the temporary map.
pub fn fn_00699b60(e: &mut Engine, this: Ptr<NavMesh>) {
    let edge_pairs = this.byte_add(MESH_EDGE_PAIR_MAP_OFFSET);
    let preferred_vertices = this.byte_add(MESH_PREFERRED_VERTEX_ARRAY_OFFSET);
    e.call(0x0043_8af0, &args![edge_pairs]);
    e.call(0x0084_54f0, &args![preferred_vertices, 1u32]);
    e.with_stack(0x20, |e, vertex_map| {
        fn_0069ad40(e, vertex_map, 0x25);
        let triangle_count = e.call(TRIANGLE_COUNT, &args![this]).u32();
        for triangle_index in 0..triangle_count {
            let triangle = triangle_of(e, this, triangle_index);
            for edge in 0..3u32 {
                let slot = e.call(EDGE_SLOT, &args![triangle, edge]).u16();
                let portal = if slot == NO_TRIANGLE {
                    false
                } else if e.call(EDGE_HAS_INFO, &args![triangle, edge]).bool() {
                    true
                } else {
                    continue;
                };
                e.with_stack(24, |e, handles| {
                    let edge_handle = handles;
                    let vertex_handle = handles.byte_add(12);
                    fn_0069a6c0(e, edge_handle, this.addr(), triangle_index as u16, edge);
                    for corner in [edge, (edge + 1) % 3] {
                        let vertex = corner_vertex_index(e, triangle, corner);
                        fn_0069a660(e, vertex_handle, this.addr(), vertex);
                        append_vertex_edge(
                            e,
                            vertex_map,
                            this,
                            vertex,
                            vertex_handle,
                            edge_handle,
                            portal,
                        );
                    }
                });
            }
        }

        // The first iterator scan may add a packed pair to the member map.
        // A second scan destroys the separately allocated POV vertex records.
        let mut iterator = e.call(0x004b_9ba0, &args![vertex_map]).u32();
        while iterator != 0 {
            e.with_stack(0x90, |e, scratch| {
                let iterator_slot = scratch;
                let key = scratch.byte_add(4);
                let record_slot = scratch.byte_add(12);
                let first = scratch.byte_add(16);
                let second = scratch.byte_add(28);
                let first_vertices = scratch.byte_add(40);
                let second_vertices = scratch.byte_add(48);
                let first_vector = scratch.byte_add(56);
                let second_vector = scratch.byte_add(68);
                let cross = scratch.byte_add(80);
                let pair = scratch.byte_add(92);
                e.mem.set_u32(iterator_slot.addr(), iterator);
                fn_0069b560(e, vertex_map, iterator_slot, key, record_slot);
                iterator = e.mem.u32(iterator_slot.addr());
                let record = Ptr::<()>::new(e.mem.u32(record_slot.addr()));
                let edges = record.byte_add(POV_EDGES_OFFSET);
                let portal_edges = record.byte_add(POV_PORTAL_EDGES_OFFSET);
                let ordinary_count = array_size(e, edges);
                let portal_count = array_size(e, portal_edges);
                if ordinary_count.wrapping_add(portal_count) != 2 || ordinary_count == 0 {
                    return;
                }
                let first_element = e.call(0x006a_1440, &args![edges, 0u32]).ptr::<()>();
                e.call(0x006b_c870, &args![first, first_element]);
                let second_element = if ordinary_count == 2 {
                    e.call(0x006a_1440, &args![edges, 1u32]).ptr::<()>()
                } else {
                    e.call(0x006a_1440, &args![portal_edges, 0u32]).ptr::<()>()
                };
                e.call(0x006b_c870, &args![second, second_element]);
                e.call(
                    GET_EDGE_VERTICES,
                    &args![
                        this,
                        first_vertices,
                        e.mem.u16(first.addr() + 4),
                        e.mem.u32(first.addr() + 8)
                    ],
                );
                e.call(
                    GET_EDGE_VERTICES,
                    &args![
                        this,
                        second_vertices,
                        e.mem.u16(second.addr() + 4),
                        e.mem.u32(second.addr() + 8)
                    ],
                );
                let first_start = e.mem.u32(first_vertices.addr());
                let first_end = e.mem.u32(first_vertices.addr() + 4);
                let second_start = e.mem.u32(second_vertices.addr());
                let second_end = e.mem.u32(second_vertices.addr() + 4);
                let (common, outer_first, outer_second, packed_first, packed_second) =
                    if first_start == second_end {
                        (first_start, first_end, second_start, first, second)
                    } else if first_end == second_start {
                        (first_end, second_end, first_start, second, first)
                    } else {
                        return;
                    };
                e.call(POINT3_SUBTRACT, &args![outer_first, first_vector, common]);
                e.call(POINT3_SUBTRACT, &args![outer_second, second_vector, common]);
                e.mem.set_f32(first_vector.addr() + 8, 0.0);
                e.mem.set_f32(second_vector.addr() + 8, 0.0);
                e.call(POINT3_NORMALIZE, &args![first_vector]);
                e.call(POINT3_NORMALIZE, &args![second_vector]);
                e.call(POINT3_CROSS, &args![first_vector, cross, second_vector]);
                let dot = e
                    .call(0x004b_6190, &args![first_vector, second_vector])
                    .f32();
                let cross_z = e.mem.f32(cross.addr() + 8);
                let mut accept = false;
                if compare_flags(dot as f64, e.global::<f64>(ZERO)) & 0x41 == 0 {
                    let positive_limit: f64 = e.global(0x0101_ffa0);
                    if !parity_even(compare_flags(cross_z as f64, positive_limit), 0x05) {
                        accept = true;
                        // 0069a2f2-0069a35e can swap the two local vectors,
                        // but neither vector is read after this decision.
                    }
                } else {
                    let negative_limit: f64 = e.global(0x0106_b170);
                    accept = !parity_even(compare_flags(cross_z as f64, negative_limit), 0x05);
                }
                if accept || ordinary_count == 1 {
                    fn_0069a700(e, pair, packed_first);
                    fn_0069a700(e, pair.byte_add(2), packed_second);
                    e.call(
                        0x0069_4c50,
                        &args![
                            edge_pairs,
                            e.mem.u16(key.addr() + 4),
                            e.mem.u32(pair.addr())
                        ],
                    );
                }
            });
        }

        iterator = e.call(0x004b_9ba0, &args![vertex_map]).u32();
        while iterator != 0 {
            e.with_stack(16, |e, scratch| {
                let iterator_slot = scratch;
                let key = scratch.byte_add(4);
                let record_slot = scratch.byte_add(12);
                e.mem.set_u32(iterator_slot.addr(), iterator);
                fn_0069b560(e, vertex_map, iterator_slot, key, record_slot);
                iterator = e.mem.u32(iterator_slot.addr());
                let record = Ptr::<()>::new(e.mem.u32(record_slot.addr()));
                if !record.is_null() {
                    fn_0069a7a0(e, record, 1);
                }
            });
        }

        let radius: f32 = e.global(0x0101_e704);
        for triangle_index in 0..triangle_count {
            e.with_stack(80, |e, scratch| {
                let center = scratch;
                let vertex_copy = scratch.byte_add(12);
                let difference = scratch.byte_add(24);
                let candidate_difference = scratch.byte_add(36);
                let iterator_slot = scratch.byte_add(48);
                let candidate_key = scratch.byte_add(52);
                let candidate_value = scratch.byte_add(60);
                fn_00697290(e, this, center, triangle_index as u16);
                let triangle = triangle_of(e, this, triangle_index);
                let first_vertex = corner_vertex_index(e, triangle, 0);
                let first_point = vertex_pointer(e, this, first_vertex);
                write_point(e, vertex_copy.addr(), read_point(e, first_point.addr()));
                let planar_difference = e
                    .call(POINT3_SUBTRACT, &args![vertex_copy, difference, center])
                    .ptr::<()>();
                let planar_distance = e.call(0x0058_9850, &args![planar_difference]).f32();
                let widened = (planar_distance as f64 + radius as f64) as f32;
                let squared = (widened as f64 * widened as f64) as f32;
                let mut threshold = (squared as f64 * squared as f64) as f32;
                let mut best = NO_TRIANGLE;
                let mut candidate = e.call(0x004b_9ba0, &args![edge_pairs]).u32();
                while candidate != 0 {
                    e.mem.set_u32(iterator_slot.addr(), candidate);
                    e.call(
                        0x0069_4d30,
                        &args![edge_pairs, iterator_slot, candidate_key, candidate_value],
                    );
                    candidate = e.mem.u32(iterator_slot.addr());
                    let vertex = e.mem.u16(candidate_key.addr());
                    let point = vertex_pointer(e, this, vertex);
                    e.call(POINT3_SUBTRACT, &args![point, candidate_difference, center]);
                    let distance = e
                        .call(POINT3_SQUARED_LENGTH_2D, &args![candidate_difference])
                        .f32();
                    if compare_flags(threshold as f64, distance as f64) & 0x41 != 0 {
                        continue;
                    }
                    let vertical = e
                        .call(
                            FLOAT_ABS,
                            &args![e.mem.f32(candidate_difference.addr() + 8)],
                        )
                        .f32();
                    let height_limit: f64 = e.global(ONE_EIGHTY);
                    if parity_even(compare_flags(vertical as f64, height_limit), 0x05) {
                        continue;
                    }
                    threshold = distance;
                    best = vertex;
                }
                e.mem.set_u16(candidate_key.addr(), best);
                e.call(ARRAY_ADD_WORD, &args![preferred_vertices, candidate_key]);
            });
        }
        fn_0069b6d0(e, vertex_map);
    });
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0069aa20, fn_0069aa20(Ptr<NavMesh>) -> bool),
        entry!(0x006997e0, fn_006997e0(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x006bc8e0, fn_006bc8e0(Ptr, Ptr, Ptr) -> bool),
        entry!(0x006bd040, fn_006bd040(Ptr)),
        entry!(0x006987d0, fn_006987d0(Ptr<NavMesh>, Ptr, Ptr, Ptr)),
        entry!(0x00699b60, fn_00699b60(Ptr<NavMesh>)),
        entry!(0x006953b0, nav_mesh_fix_portals_from_nav_mesh_deletion(Ptr)),
        entry!(0x00695660, fn_00695660(Ptr<NavMesh>, u16)),
        entry!(0x006957d0, fn_006957d0(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x006957f0, fn_006957f0(Ptr<NavMesh>, Ptr, Ptr)),
        entry!(0x006959b0, fn_006959b0(Ptr<NavMesh>)),
        entry!(0x00696500, fn_00696500(Ptr<NavMeshGrid>, u32, u32, u32)),
        entry!(0x00696540, fn_00696540(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006965d0, fn_006965d0(Ptr<NavMesh>, u16, u16) -> bool),
        entry!(0x006967c0, fn_006967c0(Ptr<NavMeshGrid>, u32, u32) -> Ptr),
        entry!(0x006967f0, fn_006967f0(Ptr<NavMesh>, u16, u16) -> bool),
        entry!(0x006969e0, fn_006969e0(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x00696a10, fn_00696a10(Ptr<NavMeshGrid>, Ptr, Ptr)),
        entry!(0x00696a50, fn_00696a50(Ptr<NavMesh>, Ptr, Ptr) -> u16),
        entry!(0x00696c00, fn_00696c00(Ptr<NavMesh>, u32, u32, u32)),
        entry!(0x00696c30, fn_00696c30(Ptr, Ptr) -> Ptr),
        entry!(0x00696c60, fn_00696c60(Ptr, Ptr) -> Ptr),
        entry!(0x00696ca0, fn_00696ca0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00696cf0,
            nav_mesh_find_closest_triangle_for_location(Ptr<NavMesh>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00697010,
            fn_00697010(Ptr<NavMesh>, Ptr, f32, Ptr, Ptr, bool) -> bool
        ),
        entry!(0x006971e0, fn_006971e0(Ptr<NavMesh>, Ptr, u16) -> Ptr),
        entry!(0x00697290, fn_00697290(Ptr<NavMesh>, Ptr, u16) -> Ptr),
        entry!(0x00697440, fn_00697440(Ptr<NavMesh>, u16, Ptr)),
        entry!(
            0x00697590,
            nav_mesh_find_portal_triangles(Ptr<NavMesh>, Ptr, Ptr) -> bool
        ),
        entry!(0x00697670, fn_00697670(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x006976a0, fn_006976a0(Ptr<NavMesh>, u16, i32) -> Ptr),
        entry!(
            0x00697730,
            fn_00697730(Ptr<NavMesh>, u16, Ptr, Ptr, f32, f32) -> bool
        ),
        entry!(
            0x00697810,
            fn_00697810(Ptr<NavMesh>, u16, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00697980,
            nav_mesh_compute_triangle_z_for_location(Ptr<NavMesh>, u16, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(
            0x00697b70,
            fn_00697b70(Ptr<NavMesh>, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(0x00697c10, fn_00697c10(Ptr<NavMesh>, Ptr, f32, Ptr) -> bool),
        entry!(0x00697d40, fn_00697d40(Ptr<NavMesh>, Ptr, Ptr, f32) -> Ptr),
        entry!(0x00698030, fn_00698030(Ptr<NavMeshTriangle>, u16) -> u32),
        entry!(
            0x00698060,
            fn_00698060(Ptr<NavMesh>, u16, i32, u32, u32, u32) -> i32
        ),
        entry!(
            0x00698160,
            nav_mesh_find_interecting_edge2(Ptr<NavMesh>, Ptr, u16, i32, u32, Ptr) -> Ptr
        ),
        entry!(0x00698300, fn_00698300(Ptr) -> Ptr),
        entry!(
            0x00698320,
            fn_00698320(Ptr<NavMesh>, u16, i32, Ptr, Ptr) -> i32
        ),
        entry!(0x00698580, fn_00698580(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006985c0, fn_006985c0(Ptr) -> Ptr),
        entry!(0x006985f0, fn_006985f0(Ptr, Ptr) -> Ptr),
        entry!(0x00698620, fn_00698620(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00698680, fn_00698680(Ptr) -> u32),
        entry!(0x006986d0, fn_006986d0(Ptr) -> bool),
        entry!(
            0x00698f40,
            nav_mesh_add_obstacle(Ptr<NavMesh>, Ptr, Ptr) -> bool
        ),
        entry!(0x00698fc0, fn_00698fc0(Ptr<NavMesh>, Ptr, Ptr) -> u32),
        entry!(0x00699000, fn_00699000(Ptr<NavMesh>, u32, Ptr) -> u32),
        entry!(0x006993c0, fn_006993c0(Ptr<NavMesh>)),
        entry!(0x00699450, fn_00699450(Ptr) -> Ptr),
        entry!(0x006994b0, fn_006994b0(Ptr)),
        entry!(0x00699510, fn_00699510(Ptr, Ptr) -> u32),
        entry!(0x00699590, fn_00699590(Ptr<NavMesh>, u32)),
        entry!(0x006995c0, fn_006995c0(Ptr<NavMesh>, Ptr) -> u32),
        entry!(0x00699630, fn_00699630(Ptr<NavMesh>, Ptr, Ptr) -> u32),
        entry!(0x00699690, fn_00699690(Ptr<NavMesh>, Ptr) -> u32),
        entry!(0x00699760, fn_00699760(u32, u16) -> u32),
        entry!(0x00699790, fn_00699790(Ptr) -> u32),
        entry!(0x006997b0, fn_006997b0(Ptr, u16, Ptr) -> bool),
        entry!(0x00699a30, fn_00699a30(Ptr<NavMesh>, u32) -> bool),
        entry!(
            0x00699af0,
            nav_mesh_get_closed_door_info_for_triangle(Ptr<NavMesh>, u16) -> Ptr
        ),
        entry!(0x0069a660, fn_0069a660(Ptr, u32, u16) -> Ptr),
        entry!(0x0069a690, fn_0069a690(Ptr, Ptr) -> Ptr),
        entry!(0x0069a6c0, fn_0069a6c0(Ptr, u32, u16, u32) -> Ptr),
        entry!(0x0069a700, fn_0069a700(Ptr, Ptr)),
        entry!(0x0069a730, fn_0069a730(Ptr) -> Ptr),
        entry!(0x0069a7a0, fn_0069a7a0(Ptr, u32) -> Ptr),
        entry!(0x0069a7d0, fn_0069a7d0(Ptr)),
        entry!(0x0069a830, fn_0069a830(Ptr<NavMesh>, u16, Ptr) -> u32),
        entry!(0x0069a940, fn_0069a940(Ptr<NavMesh>, u16, Ptr) -> u32),
        entry!(0x0069a9d0, fn_0069a9d0(Ptr, u16, Ptr) -> bool),
        entry!(0x0069ad00, fn_0069ad00(Ptr, Ptr) -> bool),
        entry!(0x0069ad40, fn_0069ad40(Ptr, u32) -> Ptr),
        entry!(0x0069ad70, fn_0069ad70(Ptr, u32) -> Ptr),
        entry!(0x0069ada0, fn_0069ada0(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0069add0, fn_0069add0(Ptr, u32, u32)),
        entry!(0x0069ae60, fn_0069ae60(Ptr, Ptr) -> u32),
        entry!(0x0069aeb0, fn_0069aeb0(Ptr, Ptr)),
        entry!(0x0069aee0, fn_0069aee0(Ptr) -> Ptr),
        entry!(0x0069af10, fn_0069af10(Ptr) -> Ptr),
        entry!(0x0069af30, fn_0069af30(Ptr) -> Ptr),
        entry!(0x0069af60, fn_0069af60(Ptr)),
        entry!(0x0069af80, fn_0069af80(Ptr, u32) -> Ptr),
        entry!(0x0069afb0, ni_t_map_base_obstacle_get_at(Ptr, u16, Ptr) -> bool),
        entry!(0x0069b030, fn_0069b030(Ptr, Ptr) -> u32),
        entry!(0x0069b080, fn_0069b080(Ptr) -> bool),
        entry!(0x0069b0a0, fn_0069b0a0(Ptr, u16, Ptr) -> bool),
        entry!(0x0069b120, fn_0069b120(Ptr, u32, u32)),
        entry!(0x0069b1b0, fn_0069b1b0(Ptr, Ptr) -> bool),
        entry!(0x0069b1d0, fn_0069b1d0(Ptr)),
        entry!(0x0069b1f0, fn_0069b1f0(Ptr, u32) -> Ptr),
        entry!(0x0069b220, fn_0069b220(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0069b240, fn_0069b240(Ptr, Ptr, u32, u32) -> i32),
        entry!(0x0069b2a0, fn_0069b2a0(Ptr, Ptr) -> bool),
        entry!(0x0069b2c0, fn_0069b2c0(Ptr) -> Ptr),
        entry!(0x0069b2f0, fn_0069b2f0(Ptr)),
        entry!(0x0069b310, fn_0069b310(Ptr, Ptr) -> u32),
        entry!(0x0069b360, fn_0069b360(Ptr, u32) -> Ptr),
        entry!(0x0069b3d0, fn_0069b3d0(Ptr, u32, u32, u32)),
        entry!(0x0069b4d0, fn_0069b4d0(Ptr, u32, u32, Ptr) -> bool),
        entry!(0x0069b560, fn_0069b560(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x0069b620, ni_t_map_fast_vertex_key_to_hash_index(Ptr, u32, u32) -> u32),
        entry!(0x0069b640, fn_0069b640(Ptr) -> u64),
        entry!(0x0069b680, ni_t_map_fast_vertex_is_keys_equal(Ptr, u32, u32, u32, u32) -> bool),
        entry!(
            0x0069b6a0,
            ni_t_map_fast_vertex_set_value(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x0069b6d0, fn_0069b6d0(Ptr)),
        entry!(0x0069b730, fn_0069b730(Ptr)),
        entry!(0x0069b760, fn_0069b760(Ptr) -> Ptr),
        entry!(0x0069b780, fn_0069b780(Ptr, Ptr)),
        entry!(0x0069b7b0, fn_0069b7b0(Ptr, u32) -> Ptr),
        entry!(0x0069b7e0, fn_0069b7e0(Ptr, u32) -> Ptr),
        entry!(0x0069b810, fn_0069b810(Ptr, u32) -> Ptr),
        entry!(0x0069b840, fn_0069b840(Ptr, u32) -> Ptr),
        entry!(0x0069b870, fn_0069b870(Ptr, u32) -> Ptr),
        entry!(0x0069b8a0, fn_0069b8a0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    #[test]
    fn rebuild_edge_pairs_runs_triangle_preferred_vertex_pass() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0106_b000, 0x1000);
        e.mem.set_f64(DOUBLE_HALF, 0.5);
        e.mem.set_f32(0x0101_e704, 128.0);
        let mesh = e.new_object::<NavMesh>();
        let triangle = e.new_object::<NavMeshTriangle>();
        let points = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        write_point(&mut e, points[0], [0.0, 0.0, 0.0]);
        write_point(&mut e, points[1], [10.0, 0.0, 0.0]);
        write_point(&mut e, points[2], [0.0, 10.0, 0.0]);
        e.register_double(0x0043_8af0, |_, _| ret(0));
        e.register_double(0x0084_54f0, |_, _| ret(0));
        e.register_double(0x0069_b360, |e, a| {
            let map = a[0];
            let buckets = e.mem.alloc(a[1] * 4);
            for bucket in 0..a[1] {
                e.mem.set_u32(buckets + bucket * 4, 0);
            }
            e.mem.set_u32(map + 4, a[1]);
            e.mem.set_u32(map + 8, buckets);
            e.mem.set_u32(map + 12, 0);
            ret(map)
        });
        e.register_double(0x00aa_10f0, |_, _| ret(0));
        e.register_double(TRIANGLE_COUNT, |_, _| ret(1));
        e.register_double(GET_TRIANGLE, move |_, _| ret(triangle.addr()));
        e.register_double(EDGE_SLOT, |_, _| ret(0));
        e.register_double(EDGE_HAS_INFO, |_, _| ret(0));
        e.register_double(TRIANGLE_VERTEX, |_, a| ret(a[1]));
        e.register_double(VERTEX_POINTER, move |_, a| ret(points[a[1] as usize]));
        e.register_double(POINT3_CONSTRUCT, |e, a| {
            for axis in 0..3 {
                e.mem
                    .set_f32(a[0] + axis * 4, f32::from_bits(a[axis as usize + 1]));
            }
            ret(a[0])
        });
        let returned_difference = Rc::new(RefCell::new(0u32));
        let captured_difference = returned_difference.clone();
        e.register_double(POINT3_SUBTRACT, move |e, a| {
            let result = e.mem.alloc(12);
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + axis * 4) - e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
                e.mem.set_f32(result + axis * 4, value);
            }
            *captured_difference.borrow_mut() = result;
            ret(result)
        });
        e.register_double(0x0058_9850, move |e, a| {
            assert_eq!(a[0], *returned_difference.borrow());
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            ret_float((x * x + y * y).sqrt())
        });
        e.register_double(0x004b_9ba0, |_, _| ret(0));
        let appended = Rc::new(RefCell::new(Vec::new()));
        let recorded = appended.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            recorded.borrow_mut().push(e.mem.u16(a[1]));
            ret(0)
        });
        e.call(0x0069_9b60, &args![mesh]);
        assert_eq!(*appended.borrow(), vec![NO_TRIANGLE]);
    }

    #[test]
    fn rebuild_edge_pairs_repairs_boundary_portal_pairs_and_selects_vertex() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0104_e000, 0x1000);
        e.map(0x0106_b000, 0x1000);
        e.mem.set_f64(DOUBLE_HALF, 0.5);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(0x0101_ffa0, 0.1);
        e.mem.set_f64(0x0106_b170, -0.1);
        e.mem.set_f64(ONE_EIGHTY, 180.0);
        e.mem.set_f32(0x0101_e704, 128.0);
        let mesh = e.new_object::<NavMesh>();
        let triangle = e.new_object::<NavMeshTriangle>();
        let points = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        write_point(&mut e, points[0], [0.0, 0.0, 0.0]);
        write_point(&mut e, points[1], [0.0, 10.0, 0.0]);
        write_point(&mut e, points[2], [10.0, 0.0, 0.0]);
        let pair_map = mesh.byte_add(0x88);
        let pair_buckets = e.mem.alloc(0x25 * 4);
        for bucket in 0..0x25 {
            e.mem.set_u32(pair_buckets + bucket * 4, 0);
        }
        e.mem.set_u32(pair_map.addr() + 4, 0x25);
        e.mem.set_u32(pair_map.addr() + 8, pair_buckets);
        let hash = 0x00ab_2000;
        let equal = 0x00ab_2010;
        let allocate = 0x00ab_2020;
        e.mem.set_u32(0x0106_b184 + 4, hash);
        e.mem.set_u32(0x0106_b184 + 8, equal);
        e.mem.set_u32(0x0106_b184 + 0xc, 0x0069_b6a0);
        e.mem.set_u32(0x0106_b184 + 0x14, allocate);
        e.register(hash, |_, a| ret((a[2] & 0xffff) % 0x25));
        e.register(equal, |_, a| {
            ret((a[1] == a[3] && (a[2] & 0xffff) == (a[4] & 0xffff)) as u32)
        });
        e.register_double(allocate, |e, _| ret(e.mem.alloc(16)));
        e.register_double(0x0069_b360, |e, a| {
            let buckets = e.mem.alloc(a[1] * 4);
            for bucket in 0..a[1] {
                e.mem.set_u32(buckets + bucket * 4, 0);
            }
            e.mem.set_u32(a[0] + 4, a[1]);
            e.mem.set_u32(a[0] + 8, buckets);
            e.mem.set_u32(a[0] + 12, 0);
            ret(a[0])
        });
        e.register_double(0x0043_8af0, |_, _| ret(0));
        e.register_double(0x0084_54f0, |_, _| ret(0));
        e.register_double(0x00aa_10f0, |_, _| ret(0));
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        let deleted = Rc::new(RefCell::new(Vec::new()));
        let captured_deletes = deleted.clone();
        e.register_double(0x0040_1030, move |_, a| {
            captured_deletes.borrow_mut().push(a[0]);
            ret(0)
        });
        e.register_double(0x0069_ba70, |e, a| {
            let buffer = e.mem.alloc(8 * 12);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u32(a[0] + 12, 8);
            ret(a[0])
        });
        e.register_double(0x0097_8bc0, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register_double(0x0069_b9d0, |_, _| ret(0));
        e.register_double(0x006b_c870, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + word * 4, e.mem.u32(a[1] + word * 4));
            }
            ret(a[0])
        });
        e.register_double(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register_double(0x006a_1440, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 12));
        e.register_double(0x004b_9ba0, |e, a| {
            let map = a[0];
            let buckets = e.mem.u32(map + 8);
            for bucket in 0..e.mem.u32(map + 4) {
                let node = e.mem.u32(buckets + bucket * 4);
                if node != 0 {
                    return ret(node);
                }
            }
            ret(0)
        });
        e.register_double(0x0069_4c50, |e, a| {
            let buckets = e.mem.u32(a[0] + 8);
            let slot = buckets + (a[1] % 0x25) * 4;
            let node = e.mem.alloc(10);
            e.mem.set_u32(node, e.mem.u32(slot));
            e.mem.set_u16(node + 4, a[1] as u16);
            e.mem.set_u32(node + 6, a[2]);
            e.mem.set_u32(slot, node);
            ret(0)
        });
        e.register_double(0x0069_4d30, |e, a| {
            let node = e.mem.u32(a[1]);
            let key = e.mem.u16(node + 4);
            e.mem.set_u16(a[2], key);
            e.mem.set_u32(a[3], e.mem.u32(node + 6));
            let mut next = e.mem.u32(node);
            if next == 0 {
                let buckets = e.mem.u32(a[0] + 8);
                for bucket in (u32::from(key) + 1)..e.mem.u32(a[0] + 4) {
                    next = e.mem.u32(buckets + bucket * 4);
                    if next != 0 {
                        break;
                    }
                }
            }
            e.mem.set_u32(a[1], next);
            ret(0)
        });
        e.register_double(TRIANGLE_COUNT, |_, _| ret(1));
        e.register_double(GET_TRIANGLE, move |_, _| ret(triangle.addr()));
        e.register_double(EDGE_SLOT, |_, a| {
            ret(if a[1] == 1 { 0 } else { NO_TRIANGLE as u32 })
        });
        e.register_double(EDGE_HAS_INFO, |_, a| ret((a[1] == 1) as u32));
        e.register_double(TRIANGLE_VERTEX, |_, a| ret(a[1]));
        e.register_double(VERTEX_POINTER, move |_, a| ret(points[a[1] as usize]));
        e.register_double(GET_EDGE_VERTICES, move |e, a| {
            let edge = a[3] as usize;
            e.mem.set_u32(a[1], points[edge]);
            e.mem.set_u32(a[1] + 4, points[(edge + 1) % 3]);
            ret(a[1])
        });
        e.register_double(POINT3_CONSTRUCT, |e, a| {
            for axis in 0..3 {
                e.mem
                    .set_f32(a[0] + axis * 4, f32::from_bits(a[axis as usize + 1]));
            }
            ret(a[0])
        });
        e.register_double(POINT3_SUBTRACT, |e, a| {
            for axis in 0..3 {
                e.mem.set_f32(
                    a[1] + axis * 4,
                    e.mem.f32(a[0] + axis * 4) - e.mem.f32(a[2] + axis * 4),
                );
            }
            ret(a[1])
        });
        e.register_double(POINT3_NORMALIZE, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            let length = (x * x + y * y).sqrt();
            e.mem.set_f32(a[0], x / length);
            e.mem.set_f32(a[0] + 4, y / length);
            ret(a[0])
        });
        e.register_double(POINT3_CROSS, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            let other_x = e.mem.f32(a[2]);
            let other_y = e.mem.f32(a[2] + 4);
            e.mem.set_f32(a[1], 0.0);
            e.mem.set_f32(a[1] + 4, 0.0);
            e.mem.set_f32(a[1] + 8, x * other_y - y * other_x);
            ret(a[1])
        });
        e.register_double(0x004b_6190, |e, a| {
            ret_float(e.mem.f32(a[0]) * e.mem.f32(a[1]) + e.mem.f32(a[0] + 4) * e.mem.f32(a[1] + 4))
        });
        e.register_double(0x0058_9850, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            ret_float((x * x + y * y).sqrt())
        });
        e.register_double(POINT3_SQUARED_LENGTH_2D, |e, a| {
            let x = e.mem.f32(a[0]);
            let y = e.mem.f32(a[0] + 4);
            ret_float(x * x + y * y)
        });
        e.register_double(FLOAT_ABS, |_, a| ret_float(f32::from_bits(a[0]).abs()));
        let appended = Rc::new(RefCell::new(Vec::new()));
        let recorded = appended.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            recorded.borrow_mut().push(e.mem.u16(a[1]));
            ret(0)
        });
        e.call(0x0069_9b60, &args![mesh]);
        assert_eq!(*appended.borrow(), vec![0]);
        assert_ne!(e.mem.u32(pair_buckets), 0);
        assert_ne!(e.mem.u32(pair_buckets + 4), 0);
        assert_ne!(e.mem.u32(pair_buckets + 8), 0);
        assert_eq!(deleted.borrow().len(), 3);
        // 00699cf1 / 00699dd0 / 00699ed8 / 00699fb7: each new record holds
        // its (mesh, vertex) key in its first six bytes.
        let mut keys: Vec<(u32, u16)> = deleted
            .borrow()
            .iter()
            .map(|&record| (e.mem.u32(record), e.mem.u16(record + 4)))
            .collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![(mesh.addr(), 0), (mesh.addr(), 1), (mesh.addr(), 2)]
        );
    }

    #[test]
    fn obstacle_undo_reloads_smart_pointers_and_mesh_between_calls() {
        let mut e = Engine::new();
        e.map(0x0101_e000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        let operation = Ptr::new(e.mem.alloc(0x40));
        let mesh_a = e.mem.alloc(0x100);
        let mesh_b = e.mem.alloc(0x100);
        let mesh_c = e.mem.alloc(0x100);
        let pointer = e.mem.alloc(4);
        let translation_object = e.mem.alloc(0x100);
        let matrix_object = e.mem.alloc(0x100);
        let returned_transform = e.mem.alloc(12);
        let pointer_gets = Rc::new(RefCell::new(0usize));
        let get_index = pointer_gets.clone();
        e.mem.set_u32(operation.addr(), mesh_a);
        e.mem.set_u32(operation.addr() + 4, pointer);
        e.mem.set_f32(translation_object + 0x6c, 100.0);
        e.mem.set_f32(matrix_object + 0x60, 10.0);
        e.mem.set_f32(0x0101_e704, 500.0);
        e.mem.set_f64(SIXTY_FOUR, 64.0);
        e.register_double(POINTER_GET, move |e, a| {
            assert_eq!(a[0], operation.addr() + 4);
            let mut index = get_index.borrow_mut();
            let call = *index;
            *index += 1;
            if call == 13 {
                // The first returned object stays in ESI, while this second
                // getter changes its field before the native code reads it.
                e.mem.set_f32(translation_object + 0x6c, 125.0);
            }
            let object = if call % 2 == 0 || call == 12 {
                translation_object
            } else {
                matrix_object
            };
            ret(object)
        });
        let height_seen = Rc::new(RefCell::new(None));
        let captured_height = height_seen.clone();
        e.register_double(0x006a_d3b0, move |_, a| {
            *captured_height.borrow_mut() = Some(a.to_vec());
            ret(7)
        });
        let matrix_seen = Rc::new(RefCell::new(Vec::new()));
        let captured_matrix = matrix_seen.clone();
        e.register_double(0x004b_4500, move |e, a| {
            captured_matrix.borrow_mut().push((a[0], a[1]));
            e.mem.set_f32(returned_transform, 1.0);
            e.mem.set_f32(returned_transform + 4, 2.0);
            e.mem.set_f32(returned_transform + 8, 3.0);
            ret(returned_transform)
        });
        let add_receivers = Rc::new(RefCell::new(Vec::new()));
        let captured_add = add_receivers.clone();
        e.register_double(POINT3_ADD, move |e, a| {
            captured_add.borrow_mut().push((a[0], a[2]));
            let output = a[1];
            let receiver = a[0];
            let translation = a[2];
            for axis in 0..3 {
                let value = e.mem.f32(receiver + axis * 4) + e.mem.f32(translation + axis * 4);
                e.mem.set_f32(output + axis * 4, value);
            }
            ret(output)
        });
        e.register_double(POINT3_CONSTRUCT, |e, a| {
            for axis in 0..3 {
                e.mem
                    .set_f32(a[0] + axis * 4, f32::from_bits(a[axis as usize + 1]));
            }
            ret(a[0])
        });
        let projection_meshes = Rc::new(RefCell::new(Vec::new()));
        let captured_projection = projection_meshes.clone();
        e.register_double(0x0069_7b70, move |e, a| {
            captured_projection.borrow_mut().push(a[0]);
            e.mem.set_u32(operation.addr(), mesh_b);
            ret(0)
        });
        let vertex_calls = Rc::new(RefCell::new(Vec::new()));
        let captured_vertex = vertex_calls.clone();
        e.register_double(0x0069_0fe0, move |e, a| {
            captured_vertex.borrow_mut().push(a[0]);
            e.mem.set_u32(operation.addr(), mesh_c);
            ret(0)
        });
        let vertex_makers = Rc::new(RefCell::new(Vec::new()));
        let captured_vertex_maker = vertex_makers.clone();
        e.register_double(0x006b_f3c0, move |e, a| {
            captured_vertex_maker.borrow_mut().push(a[0]);
            ret(e.mem.alloc(8))
        });
        e.register_double(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| ret(0));
        e.register_double(0x0062_40d0, |_, a| ret(a[0]));
        e.register_double(0x006b_f4d0, |e, a| {
            let edge = e.mem.alloc(0x30);
            e.mem.set_u32(edge, a[1]);
            e.mem.set_u32(edge + 4, a[2]);
            ret(edge)
        });
        e.register_double(ARRAY_ADD_DWORD, |_, _| ret(0));
        e.register_double(VERTEX_POINTER, |e, a| {
            let point = e.mem.alloc(12);
            e.mem.set_f32(point, a[1] as f32);
            e.mem.set_f32(point + 4, 0.0);
            e.mem.set_f32(point + 8, 0.0);
            ret(point)
        });
        e.register_double(POINT3_SUBTRACT, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + axis * 4) - e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            ret(a[1])
        });
        e.register_double(POINT3_SQUARED_LENGTH_2D, |_, _| ret_float(0.0));
        e.register_double(0x006e_5cc0, |_, _| ret(0));

        fn_006bd040(&mut e, operation);

        assert_eq!(
            height_seen.borrow().as_ref().unwrap(),
            &vec![115.0f32.to_bits(), 1, 0]
        );
        assert_eq!(matrix_seen.borrow()[0].0, matrix_object + 0x1c);
        assert_eq!(
            add_receivers.borrow()[0],
            (returned_transform, translation_object + 0x10)
        );
        assert_eq!(projection_meshes.borrow()[0], mesh_a);
        assert_eq!(projection_meshes.borrow()[1], mesh_b);
        assert_eq!(vertex_calls.borrow()[0], mesh_b);
        // 006bd398: the vertex is made on the cutter, not on the navmesh `00690fe0` left at +0.
        assert_eq!(vertex_makers.borrow()[0], operation.addr());
    }

    /// The outdoor-route stop (`read of unmapped memory at 00000003` in `006bf1d0 < 006bf3c0 <
    /// 006bd040`): `006bf3c0` and `006bf4d0` take the cutter in `ECX` (`MOV ECX,[EBP-0x108]`
    /// at 006bd398 and 006bd413; they allocate from its scrap heap at +0xc), not the navmesh
    /// at its +0. Also: the two `_vector_constructor_iterator_` calls, each corner built in its
    /// own temporary and copied, `00690fe0` on a `006240d0` record holding a copy of the
    /// projected corner, the vertices appended from their array slots, and `00439ef0` on the
    /// second end's position with the first (006bd4c1).
    #[test]
    fn footprint_vertices_and_edges_are_made_on_the_cutter() {
        let mut e = Engine::new();
        e.map(0x0101_e000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        let cutter = e.mem.alloc(0x48);
        let navmesh = e.mem.alloc(0x100);
        let obstacle = e.mem.alloc(0x80);
        let obstacle_pointer = e.mem.alloc(4);
        e.mem.set_u32(cutter, navmesh);
        e.mem.set_u32(cutter + 4, obstacle_pointer);
        for (offset, value) in [
            (0x40, 1.0f32),
            (0x44, 2.0),
            (0x48, 3.0),
            (0x4c, 40.0),
            (0x50, 50.0),
            (0x60, 0.0),
            (0x6c, 6.0),
        ] {
            e.mem.set_f32(obstacle + offset, value);
        }
        e.mem.set_f32(0x0101_e704, 500.0);
        e.mem.set_f64(SIXTY_FOUR, 64.0);
        e.register_double(POINTER_GET, move |_, _| ret(obstacle));
        let constructed = recorder(&mut e, VECTOR_CONSTRUCTOR_ITERATOR, 0);
        let built = Rc::new(RefCell::new(Vec::new()));
        let built_log = built.clone();
        e.register_double(POINT3_CONSTRUCT, move |e, a| {
            built_log.borrow_mut().push(a[0]);
            for axis in 0..3 {
                e.mem.set_u32(a[0] + axis * 4, a[axis as usize + 1]);
            }
            ret(a[0])
        });
        e.register_double(0x006a_d3b0, |_, _| ret(7));
        // The matrix and the translation leave the corner as it is.
        e.register_double(0x004b_4500, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[2] + word * 4);
                e.mem.set_u32(a[1] + word * 4, value);
            }
            ret(a[1])
        });
        e.register_double(POINT3_ADD, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[0] + word * 4);
                e.mem.set_u32(a[1] + word * 4, value);
            }
            ret(a[1])
        });
        e.register_double(0x0069_7b70, |_, _| ret(0));
        let records = recorder(&mut e, 0x0062_40d0, 0);
        let looked_up = Rc::new(RefCell::new(Vec::new()));
        let looked_up_log = looked_up.clone();
        e.register_double(0x0069_0fe0, move |e, a| {
            let point = [e.mem.f32(a[1]), e.mem.f32(a[1] + 4)];
            looked_up_log.borrow_mut().push((a[0], a[1], point));
            ret(looked_up_log.borrow().len() as u32 + 10)
        });
        let made_vertices = Rc::new(RefCell::new(Vec::new()));
        let made_vertices_log = made_vertices.clone();
        e.register_double(0x006b_f3c0, move |e, a| {
            let vertex = e.mem.alloc(8);
            e.mem.set_u16(vertex, a[1] as u16);
            made_vertices_log.borrow_mut().push((a.to_vec(), vertex));
            ret(vertex)
        });
        let made_edges = Rc::new(RefCell::new(Vec::new()));
        let made_edges_log = made_edges.clone();
        e.register_double(0x006b_f4d0, move |e, a| {
            let edge = e.mem.alloc(0x30);
            e.mem.set_u32(edge, a[1]);
            e.mem.set_u32(edge + 4, a[2]);
            made_edges_log.borrow_mut().push((a.to_vec(), edge));
            ret(edge)
        });
        let appended = Rc::new(RefCell::new(Vec::new()));
        let appended_log = appended.clone();
        e.register_double(ARRAY_ADD_DWORD, move |e, a| {
            appended_log
                .borrow_mut()
                .push((a[0], a[1], e.mem.u32(a[1])));
            ret(0)
        });
        // Vertex `10 + n` lies at x = n.
        let positions = Rc::new(RefCell::new(Vec::new()));
        let positions_log = positions.clone();
        e.register_double(VERTEX_POINTER, move |e, a| {
            let point = e.mem.alloc(12);
            e.mem.set_f32(point, (a[1] - 10) as f32);
            positions_log.borrow_mut().push((a[1], point));
            ret(point)
        });
        let subtracted = recorder(&mut e, POINT3_SUBTRACT, 0);
        e.register_double(POINT3_SQUARED_LENGTH_2D, |_, _| ret_float(0.0));

        fn_006bd040(&mut e, Ptr::new(cutter));

        let pointer_constructor = POINT3_DEFAULT_CONSTRUCTOR;
        let calls = constructed.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(&calls[0][1..], &[0xc, 4, pointer_constructor]);
        assert_eq!(&calls[1][1..], &[0xc, 4, pointer_constructor]);
        let (corners, projected) = (calls[0][0], calls[1][0]);
        // Each corner is built in its own temporary (not in the array) and copied.
        let built = built.borrow();
        assert_eq!(built.len(), 4);
        for (index, temporary) in built.iter().enumerate() {
            assert!((0..4).all(|n| *temporary != corners + n * 12), "{index}");
        }
        let corner_values: Vec<[f32; 2]> = (0..4u32)
            .map(|n| [e.mem.f32(corners + n * 12), e.mem.f32(corners + n * 12 + 4)])
            .collect();
        assert_eq!(
            corner_values,
            vec![[1.0, 2.0], [1.0, 50.0], [40.0, 50.0], [40.0, 2.0]]
        );
        // `00690fe0` gets the navmesh and the record, a copy of the projected corner.
        let records = records.borrow();
        let looked_up = looked_up.borrow();
        assert_eq!(records.len(), 4);
        for (index, (mesh, record, point)) in looked_up.iter().enumerate() {
            assert_eq!(*mesh, navmesh);
            assert_eq!(*record, records[index][0]);
            assert_ne!(*record, projected + index as u32 * 12);
            assert_eq!(*point, corner_values[index]);
        }
        // The vertices and edges are made on the cutter.
        let made_vertices = made_vertices.borrow();
        let vertices: Vec<u32> = made_vertices.iter().map(|(_, vertex)| *vertex).collect();
        for (index, (arguments, _)) in made_vertices.iter().enumerate() {
            assert_eq!(arguments, &vec![cutter, index as u32 + 11, 1, 0, 0]);
        }
        let made_edges = made_edges.borrow();
        let edges: Vec<u32> = made_edges.iter().map(|(_, edge)| *edge).collect();
        for (index, (arguments, _)) in made_edges.iter().enumerate() {
            let next = vertices[(index + 1) % 4];
            assert_eq!(arguments, &vec![cutter, vertices[index], next, 1]);
        }
        let appended = appended.borrow();
        assert_eq!(appended.len(), 8);
        for index in 0..4 {
            assert_eq!(appended[index].0, cutter + 0x1c);
            assert_eq!(appended[index].2, vertices[index]);
            assert_eq!(appended[4 + index].0, cutter + 0x30);
            assert_eq!(appended[4 + index].2, edges[index]);
        }
        // 006bd4bf: `ECX` is the second end's position, the argument the first's.
        let positions = positions.borrow();
        let subtracted = subtracted.borrow();
        assert_eq!(subtracted.len(), 4);
        for index in 0..4 {
            let (first, second) = (positions[2 * index], positions[2 * index + 1]);
            assert_eq!(first.0, index as u32 + 11);
            assert_eq!(second.0, (index as u32 + 1) % 4 + 11);
            assert_eq!(subtracted[index][0], second.1);
            assert_eq!(subtracted[index][2], first.1);
        }
    }

    #[test]
    fn obstacle_triangle_collector_skips_disabled_triangles() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x3000);
        e.map(0x0102_e000, 0x1000);
        let mesh = Ptr::<NavMesh>::new(e.mem.alloc(0x200));
        let grid = mesh_grid(mesh);
        let cell = e.mem.alloc(16);
        let cell_items = e.mem.alloc(2);
        let output = Ptr::new(e.mem.alloc(16));
        let triangle = Ptr::<()>::new(e.mem.alloc(16));
        let reference = Ptr::new(e.mem.alloc(4));
        let adjustment = Ptr::new(e.mem.alloc(12));
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(grid.addr(), 1);
        e.mem.set_u32(grid.addr() + 0x24, cell);
        e.mem.set_u32(cell + 4, cell_items);
        e.mem.set_u32(cell + 8, 1);
        e.mem.set_u16(cell_items, 3);
        e.mem.set_f32(object + 0x40, -1.0);
        e.mem.set_f32(object + 0x44, -1.0);
        e.mem.set_f32(object + 0x48, 0.0);
        e.mem.set_f32(object + 0x4c, 1.0);
        e.mem.set_f32(object + 0x50, 1.0);
        e.mem.set_f32(object + 0x60, 0.0);
        e.mem.set_f32(object + 0x6c, 0.0);
        e.mem.set_f32(0x0101_6970, f32::MAX);
        e.mem.set_f32(0x0101_5f5c, -f32::MAX);
        e.mem.set_f64(0x0102_e430, 128.0);
        e.register_double(POINTER_GET, move |_, _| ret(object));
        e.register_double(GRID_CELL_OF_POINT, |e, a| {
            e.mem.set_u32(a[2], 0);
            e.mem.set_u32(a[3], 0);
            ret(0)
        });
        e.register_double(POINT3_CONSTRUCT, |e, a| {
            for axis in 0..3 {
                e.mem
                    .set_f32(a[0] + axis * 4, f32::from_bits(a[axis as usize + 1]));
            }
            ret(a[0])
        });
        e.register_double(ARRAY_SIZE, move |_, a| {
            ret(if a[0] == cell { 1 } else { 0 })
        });
        e.register_double(ARRAY_ELEMENT_2, move |_, a| ret(cell_items + a[1] * 2));
        e.register_double(ARRAY_FIND_WORD, |_, _| ret(u32::MAX));
        e.register_double(GET_TRIANGLE, move |_, _| ret(triangle.addr()));
        e.register_double(TRIANGLE_FLAGS_SET, |_, a| {
            assert_eq!(a[1], 0x20);
            ret(1)
        });

        fn_006987d0(&mut e, mesh, reference, adjustment, output);

        assert_eq!(e.mem.u32(output.addr() + 8), 0);
    }

    #[test]
    fn obstacle_triangle_collector_accepts_intersection_deduplicates_and_rejects_bounds() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x3000);
        e.map(0x0102_e000, 0x1000);
        let mesh = Ptr::<NavMesh>::new(e.mem.alloc(0x200));
        let grid = mesh_grid(mesh);
        let cells = e.mem.alloc(4 * 16);
        let mut cell_arrays = Vec::new();
        for cell_index in 0..4u32 {
            let cell = cells + cell_index * 16;
            let item = e.mem.alloc(2);
            e.mem.set_u32(cell + 4, item);
            e.mem.set_u32(cell + 8, 1);
            e.mem.set_u16(item, 3);
            cell_arrays.push(cell);
        }
        let output = Ptr::new(e.mem.alloc(16));
        let output_items = e.mem.alloc(8);
        e.mem.set_u32(output.addr() + 4, output_items);
        let reference = Ptr::new(e.mem.alloc(4));
        let adjustment = Ptr::new(e.mem.alloc(12));
        let object = e.mem.alloc(0x80);
        let corner_z_source = e.mem.alloc(0x80);
        let corner_y_source = e.mem.alloc(0x80);
        let corner_x_source = e.mem.alloc(0x80);
        let triangle = e.mem.alloc(16);
        let vertices = [e.mem.alloc(12), e.mem.alloc(12), e.mem.alloc(12)];
        e.mem.set_u32(grid.addr(), 2);
        e.mem.set_u32(grid.addr() + 0x24, cells);
        e.mem.set_f32(object + 0x40, -2.0);
        e.mem.set_f32(object + 0x44, -2.0);
        e.mem.set_f32(object + 0x48, 0.0);
        e.mem.set_f32(object + 0x4c, 2.0);
        e.mem.set_f32(object + 0x50, 2.0);
        for axis in 0..3 {
            e.mem.set_f32(object + 0x58 + axis * 4, -1.0);
            e.mem.set_f32(object + 0x64 + axis * 4, 1.0);
        }
        e.mem.set_f32(corner_z_source + 0x48, 5.0);
        e.mem.set_f32(corner_y_source + 0x44, -2.0);
        e.mem.set_f32(corner_x_source + 0x40, -2.0);
        // One triangle edge crosses the obstacle's vertical side at x=-2.
        for (axis, point) in [[-3.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
            .into_iter()
            .enumerate()
        {
            write_point(&mut e, vertices[axis], point);
        }
        e.mem.set_f32(0x0101_6970, f32::MAX);
        e.mem.set_f32(0x0101_5f5c, -f32::MAX);
        e.mem.set_f64(0x0102_e430, 128.0);

        let grid_calls = Rc::new(RefCell::new(0u32));
        let captured_grid_calls = grid_calls.clone();
        e.register_double(GRID_CELL_OF_POINT, move |e, a| {
            let mut calls = captured_grid_calls.borrow_mut();
            let upper = *calls > 0;
            *calls += 1;
            e.mem.set_u32(a[2], u32::from(upper));
            e.mem.set_u32(a[3], u32::from(upper));
            ret(0)
        });
        let pointer_gets = Rc::new(RefCell::new(0u32));
        let captured_pointer_gets = pointer_gets.clone();
        e.register_double(POINTER_GET, move |_, _| {
            let mut call = captured_pointer_gets.borrow_mut();
            let current = *call;
            *call += 1;
            ret(match current {
                2 => corner_z_source,
                3 => corner_y_source,
                4 => corner_x_source,
                _ => object,
            })
        });
        let corners_built = Rc::new(RefCell::new(Vec::new()));
        let captured_corners_built = corners_built.clone();
        e.register_double(POINT3_CONSTRUCT, move |e, a| {
            for axis in 0..3 {
                e.mem
                    .set_f32(a[0] + axis * 4, f32::from_bits(a[axis as usize + 1]));
            }
            captured_corners_built.borrow_mut().push([
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
                f32::from_bits(a[3]),
            ]);
            ret(a[0])
        });
        e.register_double(ARRAY_SIZE, move |e, a| {
            if cell_arrays.contains(&a[0]) {
                ret(1)
            } else {
                ret(e.mem.u32(a[0] + 8))
            }
        });
        e.register_double(ARRAY_ELEMENT_2, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 2));
        e.register_double(ARRAY_FIND_WORD, |e, a| {
            let key = e.mem.u16(a[1]);
            let count = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            for index in 0..count {
                if e.mem.u16(buffer + index * 2) == key {
                    return ret(index);
                }
            }
            ret(u32::MAX)
        });
        e.register_double(ARRAY_ADD_WORD, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let buffer = e.mem.u32(a[0] + 4);
            let value = e.mem.u16(a[1]);
            e.mem.set_u16(buffer + count * 2, value);
            e.mem.set_u32(a[0] + 8, count + 1);
            ret(count)
        });
        e.register_double(GET_TRIANGLE, move |_, _| ret(triangle));
        e.register_double(TRIANGLE_VERTEX, |_, a| ret(a[1]));
        e.register_double(VERTEX_POINTER, move |_, a| ret(vertices[a[1] as usize]));
        e.register_double(TRIANGLE_FLAGS_SET, |_, _| ret(0));
        e.register_double(0x0040_ebd0, |_, a| {
            ret_float(f32::min(f32::from_bits(a[0]), f32::from_bits(a[1])))
        });
        e.register_double(0x0040_4010, |_, a| {
            ret_float(f32::max(f32::from_bits(a[0]), f32::from_bits(a[1])))
        });
        e.register_double(POINT3_ADD, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + axis * 4) + e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            ret(a[1])
        });
        e.register_double(POINT3_SUBTRACT, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + axis * 4) - e.mem.f32(a[2] + axis * 4);
                e.mem.set_f32(a[1] + axis * 4, value);
            }
            ret(a[1])
        });
        let matrices = Rc::new(RefCell::new(Vec::new()));
        let captured_matrices = matrices.clone();
        e.register_double(0x004b_3ae0, move |e, a| {
            captured_matrices.borrow_mut().push(a[2]);
            for axis in 0..3 {
                let value = e.mem.f32(a[1] + axis * 4);
                e.mem.set_f32(a[0] + axis * 4, value);
            }
            ret(a[0])
        });
        let overlap_calls = Rc::new(RefCell::new(0u32));
        let captured_overlap_calls = overlap_calls.clone();
        e.register_double(0x006d_93d0, move |e, a| {
            assert_eq!(a.len(), 4);
            *captured_overlap_calls.borrow_mut() += 1;
            let overlaps = (0..3).all(|axis| {
                let origin = e.mem.f32(a[0] + axis * 4);
                let center = e.mem.f32(a[1] + axis * 4);
                let low = e.mem.f32(a[2] + axis * 4);
                let high = e.mem.f32(a[3] + axis * 4);
                center > low && origin < high
            });
            ret(overlaps as u32)
        });
        let center_calls = Rc::new(RefCell::new(0u32));
        let captured_center_calls = center_calls.clone();
        e.register_double(GET_CENTER, move |e, a| {
            *captured_center_calls.borrow_mut() += 1;
            write_point(e, a[1], [0.0; 3]);
            ret(a[1])
        });
        let center_hits = Rc::new(RefCell::new(0u32));
        let captured_center_hits = center_hits.clone();
        e.register_double(0x006c_3570, move |_, _| {
            *captured_center_hits.borrow_mut() += 1;
            ret(0)
        });
        e.register_double(0x0069_7730, |_, _| ret(0));
        e.register_double(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_f32(a[0], f32::from_bits(a[1]));
            e.mem.set_f32(a[0] + 4, f32::from_bits(a[2]));
            ret(a[0])
        });
        let segment_calls = Rc::new(RefCell::new(0u32));
        let captured_segment_calls = segment_calls.clone();
        e.register_double(SEGMENTS_INTERSECT_2D, move |e, a| {
            let points: Vec<[f32; 2]> = a
                .iter()
                .map(|pointer| [e.mem.f32(*pointer), e.mem.f32(*pointer + 4)])
                .collect();
            *captured_segment_calls.borrow_mut() += 1;
            let [[ax, ay], [bx, by], [cx, cy], [dx, dy]] = points.as_slice() else {
                unreachable!()
            };
            let cross = |p: [f32; 2], q: [f32; 2], r: [f32; 2]| {
                (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
            };
            let first = [*ax, *ay];
            let second = [*bx, *by];
            let third = [*cx, *cy];
            let fourth = [*dx, *dy];
            let crosses = cross(first, second, third) * cross(first, second, fourth) <= 0.0
                && cross(third, fourth, first) * cross(third, fourth, second) <= 0.0;
            ret(crosses as u32)
        });

        fn_006987d0(&mut e, mesh, reference, adjustment, output);
        assert_eq!(e.mem.u32(output.addr() + 8), 1);
        assert_eq!(e.mem.u16(output_items), 3);
        assert_eq!(*overlap_calls.borrow(), 1);
        assert_eq!(*center_calls.borrow(), 1);
        assert_eq!(*center_hits.borrow(), 1);
        assert_eq!(*segment_calls.borrow(), 1);
        // 00698d81 / 00698dd5: both edge vertices are rotated by the matrix
        // at the reference object's +0x1c.
        assert_eq!(*matrices.borrow(), vec![object + 0x1c, object + 0x1c]);
        assert_eq!(
            corners_built.borrow()[0],
            [-2.0, -2.0, 5.0],
            "corner fields come from fresh pointer getter returns per coordinate"
        );

        e.mem.set_u32(output.addr() + 8, 0);
        e.mem.set_f32(object + 0x58, 100.0);
        e.mem.set_f32(object + 0x64, 100.0);
        fn_006987d0(&mut e, mesh, reference, adjustment, output);
        assert_eq!(e.mem.u32(output.addr() + 8), 0);
        assert_eq!(*overlap_calls.borrow(), 2);
        assert_eq!(*center_calls.borrow(), 1);
        assert_eq!(*center_hits.borrow(), 1);
        assert_eq!(*segment_calls.borrow(), 1);
    }

    #[test]
    fn point_quadrant_sets_sign_bits_in_game_order() {
        let mut e = Engine::new();
        let point = e.new_object::<NavMesh>().cast::<()>();
        for (x, y, expected) in [
            (1.0, 1.0, 0),
            (-1.0, 1.0, 1),
            (-1.0, -1.0, 2),
            (1.0, -1.0, 3),
            (0.0, -0.0, 0),
        ] {
            e.mem.set_f32(point.addr(), x);
            e.mem.set_f32(point.addr() + 4, y);
            assert_eq!(fn_00698680(&mut e, point), expected);
        }
    }

    #[test]
    fn get_or_add_navmesh_info_updates_an_existing_info_without_allocating() {
        let mut e = Engine::new();
        let this = Ptr::new(0x1100);
        let tes = Ptr::<()>::new(0x2200);
        let map = Ptr::<()>::new(0x3300);
        let info = Ptr::<()>::new(0x4400);
        e.map(TES_INSTANCE & !0xfff, 0x1000);
        e.mem.set_u32(TES_INSTANCE, tes.addr());
        let _allocator = recorder(&mut e, 0x0084_e3a0, 0x5500);
        let maps = recorder(&mut e, GET_NAV_MESH_INFO_MAP, map.addr());
        let lookup = recorder(&mut e, 0x0069_3d70, info.addr());
        let exists = recorder(&mut e, 0x0068_efb0, 1);
        let update = recorder(&mut e, 0x006b_5560, 0);
        assert!(!fn_006986d0(&mut e, this));
        assert_eq!(&*maps.borrow(), &[vec![tes.addr()]]);
        assert_eq!(&*lookup.borrow(), &[vec![map.addr(), 0x5500]]);
        assert_eq!(&*exists.borrow(), &[vec![this.addr()]]);
        assert_eq!(&*update.borrow(), &[vec![info.addr(), this.addr()]]);
    }

    #[test]
    fn get_or_add_navmesh_info_creates_and_registers_when_missing() {
        let mut e = Engine::new();
        let this = Ptr::new(0x1100);
        let tes = Ptr::<()>::new(0x2200);
        let map = Ptr::<()>::new(0x3300);
        e.map(TES_INSTANCE & !0xfff, 0x1000);
        e.mem.set_u32(TES_INSTANCE, tes.addr());
        recorder(&mut e, 0x0084_e3a0, 0x5500);
        recorder(&mut e, GET_NAV_MESH_INFO_MAP, map.addr());
        recorder(&mut e, 0x0069_3d70, 0);
        recorder(&mut e, 0x0068_efb0, 0);
        recorder(&mut e, 0x0040_1000, 0);
        let update = recorder(&mut e, 0x006b_5560, 0);
        let link = recorder(&mut e, 0x006d_3100, 0);
        let pause = recorder(&mut e, 0x005a_8040, 0);
        let add = recorder(&mut e, 0x006b_6940, 0);
        assert!(fn_006986d0(&mut e, this));
        assert_eq!(&*update.borrow(), &[vec![0, this.addr()]]);
        assert_eq!(&*link.borrow(), &[vec![this.addr(), 0]]);
        assert_eq!(&*pause.borrow(), &[vec![0, this.addr()]]);
        assert_eq!(&*add.borrow(), &[vec![map.addr(), 0]]);
    }

    #[test]
    fn obstacle_record_removal_passes_the_embedded_array_and_one_item() {
        let mut e = Engine::new();
        let this = Ptr::new(0x1100);
        let index = 7;
        let removed = recorder(&mut e, 0x0069_b120, 0);
        fn_00699590(&mut e, this, index);
        assert_eq!(&*removed.borrow(), &[vec![this.addr() + 0xd0, index, 1]]);
    }

    #[test]
    fn obstacle_record_lookup_for_triangle_returns_the_first_match() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let records = mesh.byte_add(0x68);
        let first = Ptr::<()>::new(0x3300);
        e.map(mesh.addr(), 0x1000);
        e.map(first.addr(), 0x1000);
        e.mem.set_u16(first.addr() + 4, 7);
        recorder(&mut e, ARRAY_SIZE, 1);
        recorder(&mut e, 0x0069_50d0, first.addr());
        assert_eq!(
            nav_mesh_get_closed_door_info_for_triangle(&mut e, mesh, 7),
            first
        );
        assert_ne!(records.addr(), 0);
    }

    #[test]
    fn closed_door_triangle_lookup_returns_null_when_no_record_matches() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let first = Ptr::<()>::new(0x3300);
        e.map(mesh.addr(), 0x1000);
        e.map(first.addr(), 0x1000);
        e.mem.set_u16(first.addr() + 4, 3);
        let size = recorder(&mut e, ARRAY_SIZE, 1);
        recorder(&mut e, 0x0069_50d0, first.addr());
        assert_eq!(
            nav_mesh_get_closed_door_info_for_triangle(&mut e, mesh, 8),
            Ptr::NULL
        );
        // 00699b11: one size read per loop test (index 0, then 1).
        assert_eq!(size.borrow().len(), 2);
    }

    #[test]
    fn closed_door_triangle_lookup_scans_past_nonmatching_records() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let first = Ptr::<()>::new(0x3300);
        let second = Ptr::<()>::new(0x4400);
        e.map(mesh.addr(), 0x1000);
        e.map(first.addr(), 0x1000);
        e.map(second.addr(), 0x1000);
        e.mem.set_u16(first.addr() + 4, 3);
        e.mem.set_u16(second.addr() + 4, 9);
        recorder(&mut e, ARRAY_SIZE, 2);
        let records = [first.addr(), second.addr()];
        // 00699b40 fetches the match a second time: answer by the index.
        e.register_double(0x0069_50d0, move |_, a| ret(records[a[1] as usize]));
        assert_eq!(
            nav_mesh_get_closed_door_info_for_triangle(&mut e, mesh, 9),
            second
        );
    }

    /// 00699af0 reads the size for every loop test (00699b11) and fetches a
    /// match again (00699b40); that second fetch's result is what it returns.
    #[test]
    fn closed_door_triangle_lookup_rereads_the_size_and_returns_the_second_fetch() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let records = mesh.addr() + 0x68;
        let first = Ptr::<()>::new(0x3300);
        let second = Ptr::<()>::new(0x4400);
        let again = Ptr::<()>::new(0x5500);
        e.map(mesh.addr(), 0x1000);
        e.map(first.addr(), 0x1000);
        e.map(second.addr(), 0x1000);
        e.mem.set_u16(first.addr() + 4, 3);
        e.mem.set_u16(second.addr() + 4, 9);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(ARRAY_SIZE, move |_, a| {
            log.borrow_mut().push(vec![ARRAY_SIZE, a[0]]);
            ret(2)
        });
        let log = calls.clone();
        let fetched = Rc::new(RefCell::new(vec![
            first.addr(),
            second.addr(),
            again.addr(),
        ]));
        e.register_double(0x0069_50d0, move |_, a| {
            log.borrow_mut().push(vec![0x0069_50d0, a[0], a[1]]);
            ret(fetched.borrow_mut().remove(0))
        });
        assert_eq!(
            nav_mesh_get_closed_door_info_for_triangle(&mut e, mesh, 9),
            again
        );
        assert_eq!(
            *calls.borrow(),
            vec![
                vec![ARRAY_SIZE, records],
                vec![0x0069_50d0, records, 0],
                vec![ARRAY_SIZE, records],
                vec![0x0069_50d0, records, 1],
                vec![0x0069_50d0, records, 1],
            ]
        );
    }

    #[test]
    fn word_array_remove_shifts_elements_and_reduces_count() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.mem.set_u32(this.addr() + 4, 0x4400);
        e.mem.set_u32(this.addr() + 8, 6);
        let move_count = recorder(&mut e, 0x0042_f5a0, 4);
        let clear = recorder(&mut e, 0x0072_ba80, 0);
        let moved = recorder(&mut e, 0x005e_fdd0, 0);
        fn_0069add0(&mut e, this, 2, 1);
        assert_eq!(&*move_count.borrow(), &[vec![3, 1]]);
        assert_eq!(&*clear.borrow(), &[vec![this.addr(), 0x4404, 1]]);
        assert_eq!(&*moved.borrow(), &[vec![this.addr(), 0x4404, 0x4404, 4]]);
        assert_eq!(e.mem.u32(this.addr() + 8), 5);
    }

    #[test]
    fn obstacle_map_insertion_resets_an_end_iterator_before_insert() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let end = recorder(&mut e, 0x0069_b080, 1);
        let reset = recorder(&mut e, 0x0069_9b60, 0);
        let insert = recorder(&mut e, 0x0069_b0a0, 0);
        fn_0069a9d0(&mut e, this, 0x1234, Ptr::new(0x4400));
        assert_eq!(&*end.borrow(), &[vec![this.addr() + 0x88]]);
        assert_eq!(&*reset.borrow(), &[vec![this.addr()]]);
        assert_eq!(
            &*insert.borrow(),
            &[vec![this.addr() + 0x88, 0x1234, 0x4400]]
        );
    }

    #[test]
    fn two_embedded_lists_are_constructed_in_member_order() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let init = recorder(&mut e, 0x0069_b2c0, 0);
        assert_eq!(fn_0069a730(&mut e, this), this);
        assert_eq!(
            &*init.borrow(),
            &[vec![this.addr() + 8], vec![this.addr() + 0x18]]
        );
    }

    #[test]
    fn list_deleting_destructor_frees_only_when_flag_bit_zero_is_set() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let destroy = recorder(&mut e, 0x0069_a7d0, 0);
        let free = recorder(&mut e, 0x0040_1030, 0);
        assert_eq!(fn_0069a7a0(&mut e, this, 2), this);
        assert_eq!(&*destroy.borrow(), &[vec![this.addr()]]);
        assert!(free.borrow().is_empty());
        assert_eq!(fn_0069a7a0(&mut e, this, 3), this);
        assert_eq!(&*free.borrow(), &[vec![this.addr()]]);
    }

    #[test]
    fn two_embedded_list_destructors_run_at_their_offsets() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let destroy = recorder(&mut e, 0x0069_b2f0, 0);
        fn_0069a7d0(&mut e, this);
        assert_eq!(
            &*destroy.borrow(),
            &[vec![this.addr() + 0x18], vec![this.addr() + 8]]
        );
    }

    #[test]
    fn add_reference_stops_on_the_prohibited_state() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let reference = Ptr::<()>::new(0x4400);
        e.map(this.addr(), 0x1000);
        e.mem.set_u32(this.addr() + 0x54, 0x5500);
        recorder(&mut e, 0x0068_f320, 1);
        let add = recorder(&mut e, 0x0069_0890, 0);
        assert!(!fn_0069ad00(&mut e, this, reference));
        assert!(add.borrow().is_empty());
    }

    #[test]
    fn map_constructor_sets_its_vtable_after_base_initialization() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.map(0x0106_b000, 0x1000);
        // Distinct first vtable entry catches a mistaken dereference.
        e.mem.set_u32(0x0106_b184, 0x7788);
        let base = recorder(&mut e, 0x0069_b360, 0);
        assert_eq!(fn_0069ad40(&mut e, this, 0x4400), this);
        assert_eq!(&*base.borrow(), &[vec![this.addr(), 0x4400]]);
        assert_eq!(e.mem.u32(this.addr()), 0x0106_b184);
    }

    #[test]
    fn map_deleting_destructor_forwards_flags_and_returns_this() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        let destroy = recorder(&mut e, 0x0069_b6d0, 0);
        let free = recorder(&mut e, 0x0040_1030, 0);
        assert_eq!(fn_0069ad70(&mut e, this, 1), this);
        assert_eq!(&*destroy.borrow(), &[vec![this.addr()]]);
        assert_eq!(&*free.borrow(), &[vec![this.addr()]]);
    }

    #[test]
    fn array_deleting_destructor_sets_vtable_and_forwards_array_count() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.map(0x0106_a000, 0x1000);
        e.mem.set_u32(0x0106_abe4, 0x7788);
        let destroy = recorder(&mut e, 0x0069_2290, 0);
        assert_eq!(fn_0069ada0(&mut e, this, Ptr::new(0x4400), 5), this);
        assert_eq!(
            e.mem.u32(this.addr()),
            0x0106_abe4,
            "the vtable's address, not its first slot"
        );
        assert_eq!(&*destroy.borrow(), &[vec![this.addr(), 0x4400, 5]]);
    }

    #[test]
    fn sorted_array_wrapper_passes_two_byte_element_size() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.mem.set_u32(this.addr() + 4, 0x4400);
        e.mem.set_u32(this.addr() + 8, 7);
        let sort = recorder(&mut e, 0x00ec_6f20, 0);
        fn_0069aeb0(&mut e, this, Ptr::new(0x5500));
        assert_eq!(&*sort.borrow(), &[vec![0x4400, 7, 2, 0x5500]]);
    }

    #[test]
    fn word_array_constructor_sets_vtable_and_initializes_empty_storage() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.map(0x0106_b000, 0x1000);
        e.mem.set_u32(0x0106_b1a4, 0x7788);
        let initialize = recorder(&mut e, 0x006b_3eb0, 0);
        assert_eq!(fn_0069aee0(&mut e, this), this);
        assert_eq!(
            e.mem.u32(this.addr()),
            0x0106_b1a4,
            "the vtable's address, not its first slot"
        );
        assert_eq!(&*initialize.borrow(), &[vec![this.addr(), 0, 0]]);
    }

    #[test]
    fn obstacle_lookup_scans_held_forms_and_returns_the_matching_index() {
        let mut e = engine();
        let this = Ptr::new(0x200000);
        let key = Ptr::new(0x7777);
        let array = this.byte_add(0xd0);
        e.map(0x200000, 0x1000);
        let elements = Ptr::<()>::new(0x310000);
        let first_holder = Ptr::<()>::new(0x320000);
        let second_holder = Ptr::<()>::new(0x330000);
        e.map(elements.addr(), 0x1000);
        e.map(first_holder.addr(), 0x1000);
        e.map(second_holder.addr(), 0x1000);
        e.mem.set_u32(array.addr() + 4, elements.addr());
        e.mem.set_u32(array.addr() + 8, 2);
        e.mem.set_u32(elements.addr(), first_holder.addr());
        e.mem.set_u32(elements.addr() + 4, second_holder.addr());
        e.mem.set_u32(first_holder.addr(), 0x8800);
        e.mem.set_u32(second_holder.addr(), 0x9900);
        let compared = Rc::new(RefCell::new(Vec::new()));
        let captured = compared.clone();
        e.register_double(0x004b_0460, move |_, a| {
            captured.borrow_mut().push(a.to_vec());
            ret((a[0] == 0x33000c && a[1] == key.addr()) as u32)
        });
        let index = fn_006995c0(&mut e, this, key);
        assert_eq!(
            &*compared.borrow(),
            &[vec![0x32000c, key.addr()], vec![0x33000c, key.addr()]]
        );
        assert_eq!(index, 1);
    }

    #[test]
    fn obstacle_lookup_writes_the_record_pointer_and_returns_its_index() {
        let mut e = engine();
        let this = Ptr::new(0x200000);
        let key = Ptr::new(0x7777);
        let out = Ptr::new(0x300000);
        let array = this.byte_add(0xd0);
        e.map(0x200000, 0x1000);
        e.map(0x300000, 0x1000);
        let elements = Ptr::<()>::new(0x310000);
        let holder = Ptr::<()>::new(0x320000);
        e.map(elements.addr(), 0x1000);
        e.map(holder.addr(), 0x1000);
        let held_form = holder.addr();
        e.mem.set_u32(array.addr() + 4, elements.addr());
        e.mem.set_u32(array.addr() + 8, 1);
        e.mem.set_u32(elements.addr(), holder.addr());
        e.mem.set_u32(holder.addr(), held_form);
        e.register_double(0x004b_0460, move |_, a| {
            ret((a[0] == held_form + 0xc && a[1] == key.addr()) as u32)
        });
        assert_eq!(fn_006995c0(&mut e, this, key), 0);
        assert_eq!(fn_00699630(&mut e, this, key, out), 0);
        assert_eq!(e.mem.u32(out.addr()), held_form);
    }

    #[test]
    fn obstacle_lookup_reads_the_count_before_each_element() {
        // 006995e4: `0044ddc0` is the loop test, called again before every element.
        let mut e = Engine::new();
        let this = Ptr::<NavMesh>::new(0x200000);
        let key = Ptr::new(0x7777);
        let holder = Ptr::<()>::new(0x3300);
        e.map(this.addr(), 0x1000);
        e.map(holder.addr(), 0x100);
        let order = Rc::new(RefCell::new(Vec::new()));
        let log = order.clone();
        e.register_double(ARRAY_SIZE, move |_, a| {
            log.borrow_mut().push(vec![ARRAY_SIZE, a[0]]);
            ret(2)
        });
        let log = order.clone();
        e.register_double(ARRAY_ELEMENT_4, move |_, a| {
            log.borrow_mut().push(vec![ARRAY_ELEMENT_4, a[0], a[1]]);
            ret(holder.addr())
        });
        recorder(&mut e, POINTER_GET, 0x5500);
        let compared = recorder(&mut e, 0x004b_0460, 0);
        assert_eq!(fn_006995c0(&mut e, this, key), u32::MAX);
        let array = this.addr() + 0xd0;
        assert_eq!(
            *order.borrow(),
            vec![
                vec![ARRAY_SIZE, array],
                vec![ARRAY_ELEMENT_4, array, 0],
                vec![ARRAY_SIZE, array],
                vec![ARRAY_ELEMENT_4, array, 1],
                vec![ARRAY_SIZE, array],
            ]
        );
        assert_eq!(
            &*compared.borrow(),
            &[vec![0x550c, key.addr()], vec![0x550c, key.addr()]]
        );
    }

    #[test]
    fn obstacle_undo_removal_destroys_and_erases_the_newest_record() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let holder = Ptr::<()>::new(0x3300);
        let record = Ptr::<()>::new(0x4400);
        let context = Ptr::<()>::new(0x5500);
        e.map(mesh.addr(), 0x1000);
        e.map(holder.addr(), 4);
        e.map(record.addr(), 0x1000);
        recorder(&mut e, ARRAY_SIZE, 1);
        recorder(&mut e, ARRAY_ELEMENT_4, holder.addr());
        recorder(&mut e, POINTER_GET, record.addr());
        let construct = recorder(&mut e, 0x006b_bb80, 0);
        let undo = recorder(&mut e, 0x006b_c3e0, 0);
        let erase = recorder(&mut e, 0x006b_c8e0, 0);
        let remove = recorder(&mut e, 0x0069_b120, 0);
        let destroy = recorder(&mut e, 0x006b_bc50, 0);
        e.mem.set_u32(record.addr() + 8, 0);
        assert_eq!(fn_00699000(&mut e, mesh, 0, context), 1);
        let cutter = construct.borrow()[0][0];
        assert!(undo.borrow().is_empty());
        assert_eq!(
            &*erase.borrow(),
            &[vec![cutter, mesh.addr(), record.addr()]]
        );
        assert_eq!(&*remove.borrow(), &[vec![mesh.addr() + 0xd0, 0, 1]]);
        assert_eq!(&*destroy.borrow(), &[vec![cutter]]);
    }

    /// 00699000 on an older record whose triangles are in use: the record is
    /// fetched before the size (0069903e..00699059), the size is read again
    /// once the cutter is built (00699170), and in both loops every use of a
    /// record fetches it anew (`00877a30`, `00559450`), as the exe does.
    #[test]
    fn obstacle_undo_removal_refetches_records_while_replaying_newer_ones() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(e.mem.alloc(0x100));
        let array = mesh.addr() + 0xd0;
        let context = Ptr::<()>::new(0x5500);
        let holders = e.mem.alloc(16);
        // States: 0 is erased by the second loop (3), 1 is the index (undone,
        // not captured), 2 is only erased, 3 is undone and captured.
        let records: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x40)).collect();
        for (slot, (&record, state)) in records.iter().zip([3u32, 1, 2, 1]).enumerate() {
            e.mem.set_u32(holders + 4 * slot as u32, record);
            e.mem.set_u32(record + 8, state);
        }
        // 00699690 on record 1: one triangle of type 1 in its +0x20 array.
        let in_use = records[1] + 0x20;
        let element = e.mem.alloc(4);
        e.mem.set_u16(element, 0);
        let triangle = e.mem.alloc(16);
        e.mem.set_u32(triangle + 0xc, 0x2000);
        recorder(&mut e, 0x005e_eda0, element);
        recorder(&mut e, GET_TRIANGLE, triangle);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(ARRAY_SIZE, move |_, a| {
            if a[0] == array {
                log.borrow_mut().push(vec![ARRAY_SIZE, a[0]]);
                ret(4)
            } else {
                ret((a[0] == in_use) as u32)
            }
        });
        let log = calls.clone();
        e.register_double(ARRAY_ELEMENT_4, move |_, a| {
            log.borrow_mut().push(vec![ARRAY_ELEMENT_4, a[0], a[1]]);
            ret(holders + 4 * a[1])
        });
        let log = calls.clone();
        e.register_double(POINTER_GET, move |e, a| {
            log.borrow_mut().push(vec![POINTER_GET, a[0]]);
            ret(e.mem.u32(a[0]))
        });
        for address in [
            0x006b_c3e0,
            0x006e_5cc0,
            0x006b_c8e0,
            0x0069_b120,
            0x006b_bd30,
        ] {
            let log = calls.clone();
            e.register_double(address, move |_, a| {
                let mut entry = vec![address];
                entry.extend_from_slice(a);
                log.borrow_mut().push(entry);
                ret(0)
            });
        }
        let vector = recorder(&mut e, 0x00ec_782f, 0);
        let construct = recorder(&mut e, 0x006b_bb80, 0);
        recorder(&mut e, 0x006b_bc50, 0);
        recorder(&mut e, 0x00ec_5fce, 0);
        assert_eq!(fn_00699000(&mut e, mesh, 1, context), 1);
        let captured = vector.borrow()[0][0];
        let cutter = construct.borrow()[0][0];
        let (this, context) = (mesh.addr(), context.addr());
        let fetch = |index: u32| {
            [
                vec![ARRAY_ELEMENT_4, array, index],
                vec![POINTER_GET, holders + 4 * index],
            ]
        };
        let mut expected = Vec::new();
        expected.extend(fetch(1));
        expected.push(vec![ARRAY_SIZE, array]);
        expected.push(vec![ARRAY_SIZE, array]);
        // Cursor 3 (state 1, newer than the index): undone, captured, erased.
        expected.extend(fetch(3));
        expected.extend(fetch(3));
        expected.push(vec![0x006b_c3e0, cutter, this, records[3], context]);
        expected.extend(fetch(3));
        expected.push(vec![0x006e_5cc0, captured, records[3] + 0x0c]);
        expected.extend(fetch(3));
        expected.push(vec![0x006b_c8e0, cutter, this, records[3]]);
        expected.push(vec![0x0069_b120, array, 3, 1]);
        // Cursor 2 (state 2): erased.
        expected.extend(fetch(2));
        expected.extend(fetch(2));
        expected.push(vec![0x006b_c8e0, cutter, this, records[2]]);
        expected.push(vec![0x0069_b120, array, 2, 1]);
        // Cursor 1 (the index, state 1): undone, not captured, erased.
        expected.extend(fetch(1));
        expected.extend(fetch(1));
        expected.push(vec![0x006b_c3e0, cutter, this, records[1], context]);
        expected.extend(fetch(1));
        expected.push(vec![0x006b_c8e0, cutter, this, records[1]]);
        expected.push(vec![0x0069_b120, array, 1, 1]);
        // The second loop: cursor 0 (state 3) is erased.
        expected.extend(fetch(0));
        expected.extend(fetch(0));
        expected.push(vec![0x006b_c8e0, cutter, this, records[0]]);
        expected.push(vec![0x0069_b120, array, 0, 1]);
        // The captured record is cut again.
        expected.push(vec![0x006b_bd30, cutter, this, captured, context]);
        assert_eq!(*calls.borrow(), expected);
    }

    #[test]
    fn undo_processing_constructs_context_and_visits_nonempty_array() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let holder = Ptr::<()>::new(0x3300);
        let record = Ptr::<()>::new(0x4400);
        e.map(mesh.addr(), 0x1000);
        e.map(holder.addr(), 4);
        e.map(record.addr(), 0x1000);
        let sizes = Rc::new(RefCell::new(vec![1, 1]));
        let sizes_copy = sizes.clone();
        e.register_double(ARRAY_SIZE, move |_, _| {
            ret(sizes_copy.borrow_mut().remove(0))
        });
        recorder(&mut e, ARRAY_ELEMENT_4, holder.addr());
        recorder(&mut e, POINTER_GET, record.addr());
        let context_init = recorder(&mut e, 0x0069_af30, 0);
        let cutter_init = recorder(&mut e, 0x006b_bb80, 0);
        let erase = recorder(&mut e, 0x006b_c8e0, 0);
        let remove = recorder(&mut e, 0x0069_b120, 0);
        let context_destroy = recorder(&mut e, 0x0069_af60, 0);
        recorder(&mut e, 0x006b_bc50, 0);
        e.mem.set_u32(record.addr() + 8, 0);
        fn_006993c0(&mut e, mesh);
        assert_eq!(context_init.borrow().len(), 2);
        assert_eq!(context_destroy.borrow().len(), 2);
        assert_eq!(cutter_init.borrow().len(), 1);
        assert_eq!(
            &*erase.borrow(),
            &[vec![cutter_init.borrow()[0][0], mesh.addr(), record.addr()]]
        );
        assert_eq!(&*remove.borrow(), &[vec![mesh.addr() + 0xd0, 0, 1]]);
    }

    #[test]
    fn obstacle_type_scan_returns_greatest_triangle_type() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let obstacle = Ptr::<()>::new(0x3300);
        let item = Ptr::<()>::new(0x4400);
        let triangle = Ptr::<()>::new(0x5500);
        e.map(mesh.addr(), 0x1000);
        e.map(obstacle.addr(), 0x1000);
        e.map(item.addr(), 4);
        e.map(triangle.addr(), 0x1000);
        let size_calls = Rc::new(RefCell::new(0));
        let size_calls_copy = size_calls.clone();
        e.register_double(ARRAY_SIZE, move |_, _| {
            *size_calls_copy.borrow_mut() += 1;
            ret(1)
        });
        recorder(&mut e, 0x005e_eda0, item.addr());
        recorder(&mut e, 0x006a_1440, item.addr());
        recorder(&mut e, GET_TRIANGLE, triangle.addr());
        e.mem.set_u16(item.addr(), 0);
        e.mem.set_u32(triangle.addr() + 0xc, 0xa000);
        assert_eq!(fn_00699690(&mut e, mesh, obstacle), 5);
        // 006996b8/00699711: the count before each element and once to end each loop.
        assert_eq!(*size_calls.borrow(), 4);
    }

    #[test]
    fn obstacle_type_scan_reads_the_saved_triangles_as_12_byte_elements() {
        // 00699690: the +0x20 indices through `005eeda0` (2-byte), the +0x30 saved
        // triangles through `006a1440` (12-byte), each count read before every element.
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let obstacle = Ptr::<()>::new(0x3300);
        let indices = 0x4400u32;
        let saved = 0x4800u32;
        let triangles = 0x5500u32;
        e.map(mesh.addr(), 0x1000);
        e.map(obstacle.addr(), 0x100);
        e.map(indices, 0x100);
        e.map(saved, 0x100);
        e.map(triangles, 0x1000);
        e.mem.set_u32(obstacle.addr() + 0x24, indices);
        e.mem.set_u32(obstacle.addr() + 0x28, 1);
        e.mem.set_u32(obstacle.addr() + 0x34, saved);
        e.mem.set_u32(obstacle.addr() + 0x38, 2);
        e.mem.set_u16(indices, 1);
        // Two saved triangles, 2 and 3; a 2-byte stride would read the flag bytes 0x0101.
        e.mem.set_u16(saved, 2);
        e.mem.set_u16(saved + 2, 0x0101);
        e.mem.set_u16(saved + 12, 3);
        for (index, kind) in [(1u32, 1u32), (2, 2), (3, 7)] {
            e.mem.set_u32(triangles + index * 0x10 + 0xc, kind << 13);
        }
        let order = Rc::new(RefCell::new(Vec::new()));
        let log = order.clone();
        e.register_double(ARRAY_SIZE, move |e, a| {
            log.borrow_mut().push(vec![ARRAY_SIZE, a[0]]);
            ret(e.mem.u32(a[0] + 8))
        });
        let log = order.clone();
        e.register_double(0x005e_eda0, move |e, a| {
            log.borrow_mut().push(vec![0x005e_eda0, a[0], a[1]]);
            ret(e.mem.u32(a[0] + 4) + a[1] * 2)
        });
        let log = order.clone();
        e.register_double(0x006a_1440, move |e, a| {
            log.borrow_mut().push(vec![0x006a_1440, a[0], a[1]]);
            ret(e.mem.u32(a[0] + 4) + a[1] * 12)
        });
        e.register_double(GET_TRIANGLE, move |_, a| {
            ret(triangles + (a[1] & 0xff) * 0x10)
        });
        assert_eq!(fn_00699690(&mut e, mesh, obstacle), 7);
        let (first, second) = (obstacle.addr() + 0x20, obstacle.addr() + 0x30);
        assert_eq!(
            *order.borrow(),
            vec![
                vec![ARRAY_SIZE, first],
                vec![0x005e_eda0, first, 0],
                vec![ARRAY_SIZE, first],
                vec![ARRAY_SIZE, second],
                vec![0x006a_1440, second, 0],
                vec![ARRAY_SIZE, second],
                vec![0x006a_1440, second, 1],
                vec![ARRAY_SIZE, second],
            ]
        );
    }

    #[test]
    fn obstacle_map_lookup_forwards_key_and_output_pointer() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(0x100));
        let map = 0x0055_6600;
        let out = Ptr::<()>::new(0x4400);
        // 006997c3: the map is the word at +0xe0, not the address +0xe0.
        e.mem.set_u32(this.addr() + 0xe0, map);
        let lookup = recorder(&mut e, 0x0069_afb0, 1);
        fn_006997b0(&mut e, this, 7, out);
        assert_eq!(&*lookup.borrow(), &[vec![map, 7, out.addr()]]);
    }

    /// The outdoor-route stop (thread 4, `open function 00000025` in `0069afb0 < 006997b0`,
    /// reached from `006bcc60` for a kind-3 edge): the navmesh's +0xe0 holds the heap map the
    /// constructor makes (`006916f0` with 0x25 buckets, stored at 0068ed10), so the map's +4
    /// is its hash size 0x25. Handing `0069afb0` the address +0xe0 made it read the map
    /// pointer as the vtable and call the map's +4 word, 0x25. With the word loaded, the
    /// map's own hash (slot +4) and compare (slot +8) run and the found item's `NiPointer`
    /// at +8 is assigned to `out` (0069b00e).
    #[test]
    fn triangle_obstacle_lookup_uses_the_map_the_navmesh_points_to() {
        let mut e = Engine::new();
        let navmesh = e.mem.alloc(0x100);
        let map = e.mem.alloc(16);
        let vtable = e.mem.alloc(12);
        let table = e.mem.alloc(0x25 * 4);
        let item = e.mem.alloc(12);
        let out = e.mem.alloc(4);
        let (hash, equal) = (0x00ab_2000, 0x00ab_2010);
        e.mem.set_u32(navmesh + 0xe0, map);
        e.mem.set_u32(map, vtable);
        e.mem.set_u32(map + 4, 0x25);
        e.mem.set_u32(map + 8, table);
        e.mem.set_u32(map + 0xc, 1);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        let key = 0x00edu16;
        e.mem.set_u32(table + (u32::from(key) % 0x25) * 4, item);
        e.mem.set_u32(item, 0);
        e.mem.set_u16(item + 4, key);
        e.mem.set_u16(item + 6, 0xcccc);
        e.mem.set_u32(item + 8, 0x7788_0010);
        let get_at = funcs()
            .into_iter()
            .find(|(address, _)| *address == 0x0069_afb0)
            .unwrap()
            .1;
        e.register(0x0069_afb0, get_at);
        let hashed: Calls = Rc::new(RefCell::new(Vec::new()));
        let hashed_log = hashed.clone();
        e.register_double(hash, move |e, a| {
            hashed_log.borrow_mut().push(a.to_vec());
            ret(a[1] % e.mem.u32(a[0] + 4))
        });
        e.register(equal, |_, a| ret((a[1] == a[2]) as u32));
        let assign = recorder(&mut e, 0x006e_5cc0, out);
        assert!(fn_006997b0(&mut e, Ptr::new(navmesh), key, Ptr::new(out)));
        assert_eq!(&*hashed.borrow(), &[vec![map, u32::from(key)]]);
        assert_eq!(&*assign.borrow(), &[vec![out, item + 8]]);
    }

    #[test]
    fn missing_obstacle_map_record_returns_native_failure_sentinel() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        e.map(mesh.addr() + 0xd0, 0x1000);
        recorder(&mut e, ARRAY_SIZE, 0);
        let key = Ptr::<()>::new(0x3300);
        assert_eq!(fn_00698fc0(&mut e, mesh, key, Ptr::NULL), 0xffff_ff00);
    }

    #[test]
    fn unsigned_max_helper_keeps_the_larger_value() {
        let mut e = Engine::new();
        assert_eq!(fn_00699760(&mut e, 4, 9), 9);
        assert_eq!(fn_00699760(&mut e, 10, 9), 10);
        assert_eq!(fn_00699760(&mut e, 65_536, u16::MAX), 65_536);
    }

    #[test]
    fn add_obstacle_constructs_cuts_and_destroys_the_cutter_in_order() {
        let mut e = Engine::new();
        let mesh = Ptr::new(0x1100);
        let obstacle = Ptr::new(0x2200);
        let data = Ptr::new(0x3300);
        let ctor = recorder(&mut e, 0x006b_bb80, 0);
        let split = recorder(&mut e, 0x006b_bd30, 1);
        let dtor = recorder(&mut e, 0x006b_bc50, 0);
        assert!(nav_mesh_add_obstacle(&mut e, mesh, obstacle, data));
        let cutter = ctor.borrow()[0][0];
        assert_eq!(&*ctor.borrow(), &[vec![cutter]]);
        assert_eq!(
            &*split.borrow(),
            &[vec![cutter, mesh.addr(), obstacle.addr(), data.addr()]]
        );
        assert_eq!(&*dtor.borrow(), &[vec![cutter]]);
    }

    #[test]
    fn embedded_map_constructor_and_destructor_visit_members_in_order() {
        let mut e = Engine::new();
        let this = Ptr::new(0x2200);
        let first_init = recorder(&mut e, 0x0069_af30, 0);
        assert_eq!(fn_00699450(&mut e, this), this);
        assert_eq!(
            &*first_init.borrow(),
            &[vec![this.addr()], vec![this.addr() + 0x10]]
        );
        let destroyed = recorder(&mut e, 0x0069_af60, 0);
        fn_006994b0(&mut e, this);
        // 006994e0: the member at +0x10 is destroyed before the one at +0.
        assert_eq!(
            &*destroyed.borrow(),
            &[vec![this.addr() + 0x10], vec![this.addr()]]
        );
    }

    #[test]
    fn cancellation_scoped_operation_runs_then_destroys_its_token() {
        let mut e = Engine::new();
        let this = Ptr::new(0x3300);
        let operation = Ptr::new(0x4400);
        let call = recorder(&mut e, 0x0063_3c90, 0);
        let source = recorder(&mut e, 0x006c_61b0, 0x1234);
        let drop = recorder(&mut e, 0x0045_cec0, 0);
        assert_eq!(fn_00699510(&mut e, this, operation), 0x1234);
        let token = call.borrow()[0][0];
        assert_eq!(&*call.borrow(), &[vec![token, operation.addr()]]);
        assert_eq!(&*source.borrow(), &[vec![this.addr() + 0xd0, token]]);
        assert_eq!(&*drop.borrow(), &[vec![token]]);
    }

    #[test]
    fn obstacle_type_extracts_three_high_bits() {
        let mut e = Engine::new();
        let data = e.new_object::<NavMesh>().cast::<()>();
        for (bits, expected) in [(0, 0), (0x2000, 1), (0xa000, 5), (0xe000, 7)] {
            e.mem.set_u32(data.addr() + 0x0c, bits);
            assert_eq!(fn_00699790(&mut e, data), expected);
        }
    }

    #[test]
    fn edge_record_constructor_and_copy_touch_only_six_bytes() {
        let mut e = Engine::new();
        let source = e.new_object::<NavMesh>().cast::<()>();
        let copy = e.new_object::<NavMesh>().cast::<()>();
        e.mem.set_u32(source.addr() + 8, 0xfeed_beef);
        assert_eq!(fn_0069a660(&mut e, source, 0x1234_5678, 0x9abc), source);
        e.mem.set_u32(copy.addr(), 0xaaaa_aaaa);
        e.mem.set_u32(copy.addr() + 4, 0xbbbb_bbbb);
        e.mem.set_u32(copy.addr() + 8, 0xcccc_cccc);
        fn_0069a690(&mut e, copy, source);
        assert_eq!(e.mem.u32(copy.addr()), 0x1234_5678);
        assert_eq!(e.mem.u16(copy.addr() + 4), 0x9abc);
        assert_eq!(e.mem.u16(copy.addr() + 6), 0xbbbb);
        assert_eq!(e.mem.u32(copy.addr() + 8), 0xcccc_cccc);
    }

    #[test]
    fn edge_record_three_word_constructor_writes_fields_in_order() {
        let mut e = Engine::new();
        let object = e.new_object::<NavMesh>().cast::<()>();
        assert_eq!(fn_0069a6c0(&mut e, object, 0x1234, 0xabcd, 0x5678), object);
        assert_eq!(e.mem.u32(object.addr()), 0x1234);
        assert_eq!(e.mem.u16(object.addr() + 4), 0xabcd);
        assert_eq!(e.mem.u32(object.addr() + 8), 0x5678);
    }

    #[test]
    fn edge_descriptor_packs_only_the_defined_bits() {
        let mut e = Engine::new();
        let record = e.new_object::<NavMesh>().cast::<()>();
        let out = e.new_object::<NavMesh>().cast::<()>();
        e.mem.set_u16(record.addr() + 4, 0xffff);
        e.mem.set_u16(record.addr() + 8, 0xfffe);
        fn_0069a700(&mut e, out, record);
        assert_eq!(e.mem.u16(out.addr()), 0xbfff);
    }

    #[test]
    fn unsigned_word_comparator_orders_equal_and_unequal_keys() {
        let mut e = Engine::new();
        let a = e.new_object::<NavMesh>().cast::<()>();
        let b = e.new_object::<NavMesh>().cast::<()>();
        for (first, second, expected) in
            [(1, 2, u32::MAX), (2, 1, 1), (4, 4, 0), (0xffff, 0x0100, 1)]
        {
            e.mem.set_u16(a.addr(), first);
            e.mem.set_u16(b.addr(), second);
            assert_eq!(fn_0069ae60(&mut e, a, b), expected);
        }
    }

    #[test]
    fn closed_door_removal_clears_the_triangle_flag_then_erases_record() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let records = mesh.byte_add(0x68);
        let record = Ptr::<()>::new(0x3300);
        e.map(mesh.addr(), 0x1000);
        e.map(record.addr(), 0x1000);
        e.mem.set_u32(record.addr(), 0xabc);
        e.mem.set_u16(record.addr() + 4, 3);
        let sizes = Rc::new(RefCell::new(vec![1, 0]));
        let size_values = sizes.clone();
        e.register_double(ARRAY_SIZE, move |_, _| {
            ret(size_values.borrow_mut().remove(0))
        });
        e.register_double(0x0069_50d0, move |_, _| ret(record.addr()));
        let count = recorder(&mut e, 0x0055_8200, 4);
        let triangle = recorder(&mut e, 0x0055_82f0, 0x4400);
        let clear = recorder(&mut e, 0x0051_9270, 0);
        let remove = recorder(&mut e, 0x0098_3720, 0);
        assert!(fn_00699a30(&mut e, mesh, 0xabc));
        assert_eq!(&*count.borrow(), &[vec![mesh.addr()]]);
        assert_eq!(&*triangle.borrow(), &[vec![mesh.addr(), 3]]);
        assert_eq!(&*clear.borrow(), &[vec![0x4400, 0x1000]]);
        assert_eq!(&*remove.borrow(), &[vec![records.addr(), 0, 1]]);
    }

    #[test]
    fn triangle_edge_collection_adds_unique_edges_and_the_extra_entry() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let manager = Ptr::<()>::new(0x3300);
        let triangle_data = Ptr::<()>::new(0x4400);
        let extra = Ptr::<()>::new(0x5500);
        e.map(mesh.addr(), 0x1000);
        e.map(manager.addr(), 0x1000);
        e.map(triangle_data.addr(), 0x1000);
        e.map(extra.addr(), 0x1000);
        e.mem.set_u16(extra.addr(), 8);
        recorder(&mut e, 0x0069_b080, 0);
        recorder(&mut e, 0x0084_54f0, 0);
        recorder(&mut e, 0x0055_82f0, triangle_data.addr());
        let edge_indices = Rc::new(RefCell::new(0u32));
        let edge_indices_for_double = edge_indices.clone();
        e.register_double(0x0055_82d0, move |_, _| {
            let i = *edge_indices_for_double.borrow();
            *edge_indices_for_double.borrow_mut() += 1;
            ret([4u32, 5, 6][i as usize])
        });
        e.register_double(0x0069_b0a0, move |_, a| {
            let edge = a[1] as u16;
            ret((edge != 5) as u32)
        });
        let array_size = recorder(&mut e, ARRAY_SIZE, 1);
        recorder(&mut e, 0x005e_eda0, extra.addr());
        let added_values = Rc::new(RefCell::new(Vec::new()));
        let captured_values = added_values.clone();
        e.register_double(0x0069_1820, move |e, a| {
            captured_values.borrow_mut().push(e.mem.u16(a[1]));
            ret(0)
        });
        assert_eq!(fn_0069a830(&mut e, mesh, 0, manager), 3);
        assert_eq!(&*added_values.borrow(), &[4, 6, 8]);
        assert_eq!(&*array_size.borrow(), &[vec![mesh.addr() + 0x98]]);
    }

    #[test]
    fn triangle_edge_filter_appends_only_edges_found_in_the_map() {
        let mut e = Engine::new();
        let mesh = Ptr::<NavMesh>::new(0x200000);
        let manager = Ptr::<()>::new(0x3300);
        let triangle_data = Ptr::<()>::new(0x4400);
        e.map(mesh.addr(), 0x1000);
        e.map(manager.addr(), 0x1000);
        e.map(triangle_data.addr(), 0x1000);
        recorder(&mut e, 0x0084_54f0, 0);
        recorder(&mut e, 0x0055_82f0, triangle_data.addr());
        let edge_index = Rc::new(RefCell::new(0u32));
        let edge_index_copy = edge_index.clone();
        e.register_double(0x0055_82d0, move |_, _| {
            let i = *edge_index_copy.borrow();
            *edge_index_copy.borrow_mut() += 1;
            ret([9u32, 10, 11][i as usize])
        });
        let query = Rc::new(RefCell::new(Vec::new()));
        let query_copy = query.clone();
        let query_edges = Rc::new(RefCell::new(Vec::new()));
        let query_edges_copy = query_edges.clone();
        e.register_double(0x0069_b240, move |e, a| {
            query_copy.borrow_mut().push(a.to_vec());
            let edge = e.mem.u16(a[1]);
            query_edges_copy.borrow_mut().push(edge);
            ret(if edge == 10 { u32::MAX } else { 0 })
        });
        let values = Rc::new(RefCell::new(Vec::new()));
        let values_copy = values.clone();
        e.register_double(0x0069_1820, move |e, a| {
            values_copy.borrow_mut().push(e.mem.u16(a[1]));
            ret(0)
        });
        assert_eq!(fn_0069a940(&mut e, mesh, 2, manager), 2);
        assert_eq!(&*values.borrow(), &[9, 11]);
        assert_eq!(query.borrow().len(), 3);
        assert_eq!(&*query_edges.borrow(), &[9, 10, 11]);
        assert_eq!(query.borrow()[0][0], mesh.addr() + 0xe4);
    }

    fn arg_f32(word: u32) -> f32 {
        f32::from_bits(word)
    }

    /// A double that records its arguments and returns `result`.
    fn recorder(e: &mut Engine, address: u32, result: u32) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(result)
        });
        calls
    }

    fn read3(e: &Engine, address: u32) -> [f32; 3] {
        read_point(e, address)
    }

    /// An engine with the exe data this file reads mapped and set, and
    /// doubles for the small getters and the `NiPoint3` arithmetic that
    /// behave like the game's (on the real memory layout).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            FLOAT_MAX,
            NEGATIVE_FLOAT_MAX,
            HEIGHT_UP,
            HEIGHT_DOWN,
            HALF,
            ZERO,
            GROUND_MARGIN,
            NEGATIVE_180,
            SIXTY_FOUR,
            FORTY,
            ONE_EIGHTY,
            DOUBLE_HALF,
            RANDOM_POINT_BASE,
            TES_INSTANCE,
        ] {
            e.map(address & !0xfff, 0x1000);
        }
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(NEGATIVE_FLOAT_MAX, -f32::MAX);
        e.set_global(HEIGHT_UP, 180.0f32);
        e.set_global(HEIGHT_DOWN, 64.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(ZERO, 0.0f64);
        e.set_global(GROUND_MARGIN, 50.0f64);
        e.set_global(NEGATIVE_180, -180.0f64);
        e.set_global(SIXTY_FOUR, 64.0f64);
        e.set_global(FORTY, 40.0f64);
        e.set_global(ONE_EIGHTY, 180.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);

        // BSSimpleArray (buffer +4, size +8) and its element getters.
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(
            ARRAY_IS_EMPTY,
            |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32),
        );
        e.register(ARRAY_ELEMENT_4, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(ARRAY_ELEMENT_2, |e, a| ret(e.mem.u32(a[0] + 4) + 2 * a[1]));
        e.register(TRIANGLES_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        // NavMesh getters: triangles at +0x38, vertices at +0x28.
        e.register(TRIANGLE_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x38 + 8)));
        e.register(GET_TRIANGLE, |e, a| {
            ret(e.mem.u32(a[0] + 0x38 + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(VERTEX_POINTER, |e, a| {
            ret(e.mem.u32(a[0] + 0x28 + 4) + (a[1] & 0xffff) * 12)
        });
        // NavMeshTriangle getters and setters.
        e.register(TRIANGLE_VERTEX, |e, a| {
            ret(e.mem.u16(a[0] + 2 * a[1]) as u32)
        });
        e.register(EDGE_HAS_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 12) >> (a[1] & 31)) & 1)
        });
        e.register(EDGE_HAS_NEIGHBOUR, |e, a| {
            ret((e.mem.u16(a[0] + 6 + 2 * a[1]) != 0xffff) as u32)
        });
        e.register(EDGE_SLOT, |e, a| ret(e.mem.u16(a[0] + 6 + 2 * a[1]) as u32));
        e.register(ANY_EDGE_HAS_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 12) & 7 != 0) as u32)
        });
        e.register(TRIANGLE_FLAGS_SET, |e, a| {
            ret((e.mem.u32(a[0] + 12) & a[1] != 0) as u32)
        });
        e.register(SET_VERTEX, |e, a| {
            e.mem.set_u16(a[0] + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(SET_EDGE_INFO_SLOT, |e, a| {
            let flags = e.mem.u32(a[0] + 12);
            e.mem.set_u32(a[0] + 12, flags | 1 << (a[1] & 31));
            e.mem.set_u16(a[0] + 6 + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(SET_EDGE_NEIGHBOUR, |e, a| {
            let flags = e.mem.u32(a[0] + 12);
            e.mem.set_u32(a[0] + 12, flags & !(1 << (a[1] & 31)));
            e.mem.set_u16(a[0] + 6 + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(FLOAT_ABS, |_, a| ret_float(arg_f32(a[0]).abs()));
        e.register(FLOAT_FLOOR, |_, a| ret_float(arg_f32(a[0]).floor()));
        // NiPoint3 and NiPoint2 arithmetic.
        e.register(POINT3_CONSTRUCT, |e, a| {
            write_point(e, a[0], [arg_f32(a[1]), arg_f32(a[2]), arg_f32(a[3])]);
            ret(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(POINT3_ADD, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[1])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            ret(a[1])
        });
        e.register(POINT3_SCALE, |e, a| {
            let (p, s) = (read3(e, a[0]), arg_f32(a[2]));
            write_point(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            ret(a[1])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[1]));
            write_point(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[0])
        });
        e.register(POINT3_CROSS, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], cross3(p, q));
            ret(a[1])
        });
        e.register(POINT3_UNIT_CROSS, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            let c = cross3(p, q);
            let length = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
            write_point(e, a[1], [c[0] / length, c[1] / length, c[2] / length]);
            ret(a[1])
        });
        e.register(POINT3_NORMALIZE, |e, a| {
            let p = read3(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write_point(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
        e.register(POINT3_SQUARED_LENGTH, |e, a| {
            let p = read3(e, a[0]);
            ret_float(p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
        });
        e.register(POINT3_SQUARED_LENGTH_2D, |e, a| {
            let p = read3(e, a[0]);
            ret_float(p[0] * p[0] + p[1] * p[1])
        });
        e.register(POINT3_EQUALS, |e, a| {
            ret((read3(e, a[0]) == read3(e, a[1])) as u32)
        });
        e
    }

    fn cross3(p: [f32; 3], q: [f32; 3]) -> [f32; 3] {
        [
            p[1] * q[2] - p[2] * q[1],
            p[2] * q[0] - p[0] * q[2],
            p[0] * q[1] - p[1] * q[0],
        ]
    }

    fn put_array(e: &mut Engine, array: Ptr, data: &[u8], count: u32) {
        let buffer = e.mem.alloc(data.len().max(8) as u32);
        e.mem.write(buffer, data);
        e.mem.set_u32(array.addr() + 4, buffer);
        e.mem.set_u32(array.addr() + 8, count);
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn words16(values: &[u16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    type TriangleSpec = ([u16; 3], [u16; 3], u32);

    /// A mesh with the given vertices and `(vertices, neighbours, flags)`
    /// triangles.
    fn mesh_with(
        e: &mut Engine,
        vertices: &[[f32; 3]],
        triangles: &[TriangleSpec],
    ) -> Ptr<NavMesh> {
        let mesh: Ptr<NavMesh> = e.new_object();
        let vertex_bytes: Vec<u8> = vertices
            .iter()
            .flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes()))
            .collect();
        put_array(
            e,
            mesh.at(NavMesh::Vertices).cast(),
            &vertex_bytes,
            vertices.len() as u32,
        );
        let mut triangle_bytes = Vec::new();
        for (v, n, f) in triangles {
            for word in v.iter().chain(n.iter()) {
                triangle_bytes.extend_from_slice(&word.to_le_bytes());
            }
            triangle_bytes.extend_from_slice(&f.to_le_bytes());
        }
        put_array(
            e,
            mesh.at(NavMesh::Triangles).cast(),
            &triangle_bytes,
            triangles.len() as u32,
        );
        mesh
    }

    fn triangle_ptr(e: &Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
        Ptr::new(e.mem.u32(mesh.addr() + 0x38 + 4) + index * 16)
    }

    fn point(e: &mut Engine, p: [f32; 3]) -> Ptr {
        let address = e.mem.alloc(12);
        write_point(e, address, p);
        Ptr::new(address)
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    // --- 006953b0 ----------------------------------------------------

    #[test]
    fn fix_portals_removes_the_deleted_info_everywhere() {
        let mut e = engine();
        e.map(TES_INSTANCE & !0xfff, 0x1000);
        e.set_global(TES_INSTANCE, 0x7000u32);
        let deleted = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        let other_two = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let bare = e.mem.alloc(0x60);
        let bare_two = e.mem.alloc(0x60);
        // `info` is adjacent to `deleted` twice over and has a loaded
        // navmesh whose extra edge infos lead to `deleted`, `other`,
        // `deleted`.
        let mesh = e.mem.alloc(0x108);
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xc8, 0x0500_0000);
        e.mem.set_u32(mesh, vtable);
        let infos: Vec<u8> = [deleted, other, deleted]
            .iter()
            .flat_map(|target| words(&[0, *target, 0]))
            .collect();
        put_array(&mut e, Ptr::new(mesh + 0x48), &infos, 3);
        e.mem.set_u32(info + 0x54, mesh);
        put_array(&mut e, Ptr::new(info + 0x24), &words(&[deleted, other]), 2);
        put_array(
            &mut e,
            Ptr::new(info + 0x34),
            &words(&[other_two, deleted]),
            2,
        );
        // `bare` is not adjacent to anything that matters; a second one has
        // no navmesh loaded and is found through its form id.
        put_array(&mut e, Ptr::new(bare + 0x24), &words(&[other]), 1);
        let unloaded = bare_two;
        e.mem.set_u32(unloaded, 0x77);
        put_array(&mut e, Ptr::new(unloaded + 0x24), &words(&[deleted]), 1);
        let unloaded_mesh = e.mem.alloc(0x108);
        e.mem.set_u32(unloaded_mesh, vtable);
        put_array(
            &mut e,
            Ptr::new(unloaded_mesh + 0x48),
            &words(&[0, deleted, 0]),
            1,
        );

        let listed = [info, bare, unloaded];
        e.register(INFO_LIST_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INFO_LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_NAV_MESH_INFO_MAP, |_, a| {
            assert_eq!(a, [0x7000]);
            ret(0x1234)
        });
        let buffer = e.mem.alloc(16);
        for (i, info) in listed.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *info);
        }
        let map_calls = Rc::new(RefCell::new(Vec::new()));
        let log = map_calls.clone();
        e.register_double(INFO_MAP_COLLECT, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1] + 4, buffer);
            e.mem.set_u32(a[1] + 8, 3);
            Ret::default()
        });
        // NiPointer<NavMesh> stand-ins: the pointer is the word at +0.
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(POINTER_DEREFERENCE, |e, a| ret(e.mem.u32(a[0])));
        e.register(INFO_GET_NAV_MESH, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x54);
            e.mem.set_u32(a[1], mesh);
            ret((mesh != 0) as u32)
        });
        e.register(POINTER_CONSTRUCT_FROM_CDECL, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let lookups = recorder(&mut e, LOOKUP_FORM, 0x4000);
        let casts = recorder(&mut e, RT_DYNAMIC_CAST, unloaded_mesh);
        let slot_calls = recorder(&mut e, 0x0500_0000, 0);
        // Removal from a pointer array shifts the later entries down.
        e.register(ARRAY_REMOVE, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            for i in a[1]..size - 1 {
                let next = e.mem.u32(buffer + 4 * (i + 1));
                e.mem.set_u32(buffer + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        e.register(GET_EXTRA_EDGE_INFO, |e, a| {
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * a[1])
        });
        // Deleting an extra edge info moves the last one into the gap.
        let deleted_indexes = Rc::new(RefCell::new(Vec::new()));
        let log = deleted_indexes.clone();
        e.register_double(DELETE_EXTRA_EDGE_INFO, move |e, a| {
            log.borrow_mut().push((a[0], a[1]));
            let buffer = e.mem.u32(a[0] + 0x48 + 4);
            let size = e.mem.u32(a[0] + 0x48 + 8);
            for word in 0..3 {
                let last = e.mem.u32(buffer + 12 * (size - 1) + 4 * word);
                e.mem.set_u32(buffer + 12 * a[1] + 4 * word, last);
            }
            e.mem.set_u32(a[0] + 0x48 + 8, size - 1);
            Ret::default()
        });

        nav_mesh_fix_portals_from_nav_mesh_deletion(&mut e, Ptr::new(deleted));
        assert_eq!(map_calls.borrow().len(), 1);
        assert_eq!(map_calls.borrow()[0][0], 0x1234);
        // Both lists of `info` lost `deleted`; the others are untouched.
        assert_eq!(array_words(&e, info + 0x24), vec![other]);
        assert_eq!(array_words(&e, info + 0x34), vec![other_two]);
        assert_eq!(array_words(&e, bare + 0x24), vec![other]);
        assert_eq!(array_words(&e, unloaded + 0x24), Vec::<u32>::new());
        // `info`'s mesh: the extra infos to `deleted` were deleted (the
        // second one moved into the first slot and was checked again).
        assert_eq!(
            *deleted_indexes.borrow(),
            vec![(mesh, 0), (mesh, 0), (unloaded_mesh, 0)]
        );
        // One extra info is left, the one leading to `other`.
        assert_eq!(e.mem.u32(mesh + 0x48 + 8), 1);
        let buffer = e.mem.u32(mesh + 0x48 + 4);
        assert_eq!(e.mem.u32(buffer + 4), other);
        // The unloaded info's mesh was found through its form id.
        assert_eq!(*lookups.borrow(), vec![vec![0x77]]);
        assert_eq!(
            casts.borrow()[0],
            vec![0x4000, 0, TYPE_TES_FORM, TYPE_NAV_MESH, 0]
        );
        // The virtual call (slot 0xC8, argument 1) ran for both meshes.
        // (once per matching entry: `info` matches twice).
        assert_eq!(
            *slot_calls.borrow(),
            vec![vec![mesh, 1], vec![mesh, 1], vec![unloaded_mesh, 1]]
        );

        // A null info does nothing at all.
        e.call_log = Some(vec![]);
        nav_mesh_fix_portals_from_nav_mesh_deletion(&mut e, Ptr::NULL);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 0);
    }

    // --- 00695660 and 006957d0 -----------------------------------------

    fn triangle_words(e: &Engine, tri: Ptr<NavMeshTriangle>) -> ([u16; 3], [u16; 3], u32) {
        let a = tri.addr();
        (
            [e.mem.u16(a), e.mem.u16(a + 2), e.mem.u16(a + 4)],
            [e.mem.u16(a + 6), e.mem.u16(a + 8), e.mem.u16(a + 10)],
            e.mem.u32(a + 12),
        )
    }

    #[test]
    fn flip_swaps_vertices_and_the_first_two_edges() {
        let mut e = engine();
        // Edge 0 is a plain neighbour (5), edge 1 has extra info (slot 7).
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0b010)]);
        fn_00695660(&mut e, mesh, 0);
        let tri = triangle_ptr(&e, mesh, 0);
        // Edge 0 now has the info (flag set, slot 7); edge 1 the neighbour.
        assert_eq!(triangle_words(&e, tri), ([1, 0, 2], [7, 5, 9], 0b001));
    }

    #[test]
    fn flip_swaps_the_packed_edge_bits_when_present() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0x0001_0000)]);
        // Edge 0 unpacks to (3, 1, 0), edge 1 to (9, 0, 1).
        e.register(UNPACK_EDGE_BITS, |e, a| {
            let (value, flag_a, flag_b) = if a[1] == 0 { (3, 1, 0) } else { (9, 0, 1) };
            e.mem.set_u16(a[2], value);
            e.mem.set_u8(a[3], flag_a);
            e.mem.set_u8(a[4], flag_b);
            Ret::default()
        });
        let packs = recorder(&mut e, PACK_EDGE_BITS, 0);
        fn_00695660(&mut e, mesh, 0);
        let tri = triangle_ptr(&e, mesh, 0).addr();
        assert_eq!(
            *packs.borrow(),
            vec![vec![tri, 0, 9, 1, 0], vec![tri, 1, 3, 0, 1]]
        );

        // Without edge bits the pack function is not called.
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0)]);
        packs.borrow_mut().clear();
        fn_00695660(&mut e, mesh, 0);
        assert!(packs.borrow().is_empty());
    }

    #[test]
    fn packed_edge_bits_test() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 1, 2], [0; 3], 0x0001_0000),
                ([0, 1, 2], [0; 3], 0x0000_ffff),
                ([0, 1, 2], [0; 3], 0xf000_0000),
                ([0, 1, 2], [0; 3], 0x0800_0000),
            ],
        );
        let results: Vec<bool> = (0..4)
            .map(|i| {
                let tri = triangle_ptr(&e, mesh, i);
                fn_006957d0(&mut e, tri)
            })
            .collect();
        assert_eq!(results, vec![true, false, false, true]);
    }

    // --- 006957f0 -----------------------------------------------------

    #[test]
    fn bounds_cover_the_vertices_of_the_triangles() {
        let mut e = engine();
        // Vertex 4 belongs to no triangle and does not count.
        let mesh = mesh_with(
            &mut e,
            &[
                [1.0, 2.0, 3.0],
                [-4.0, 5.0, 6.0],
                [7.0, -8.0, 9.0],
                [0.0, 0.0, -10.0],
                [100.0, 100.0, 100.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([1, 2, 3], [0xffff; 3], 0)],
        );
        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        fn_006957f0(&mut e, mesh, min, max);
        assert_eq!(read_point(&e, min.addr()), [-4.0, -8.0, -10.0]);
        assert_eq!(read_point(&e, max.addr()), [7.0, 5.0, 9.0]);

        // A mesh without triangles leaves the outputs alone.
        let empty = mesh_with(&mut e, &[], &[]);
        let min = point(&mut e, [1.0, 2.0, 3.0]);
        let max = point(&mut e, [4.0, 5.0, 6.0]);
        fn_006957f0(&mut e, empty, min, max);
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);
    }

    // --- grid helpers ---------------------------------------------------

    const GRID_SIZE: u32 = 3;

    /// Sets up the grid of `mesh` like the game's setup would for this
    /// test: 3 by 3 cells of 10 by 10 from the origin, each an (empty)
    /// 16-byte array.
    fn set_up_grid(e: &mut Engine, mesh: Ptr<NavMesh>) -> Ptr<NavMeshGrid> {
        let grid = mesh_grid(mesh);
        e.set(grid, NavMeshGrid::iGridSize, GRID_SIZE);
        e.set(grid, NavMeshGrid::fColumnSectionLen, 10.0f32);
        e.set(grid, NavMeshGrid::fRowSectionLen, 10.0f32);
        let cells = e.mem.alloc(GRID_SIZE * GRID_SIZE * 16);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(cells));
        grid
    }

    /// Records the `(cell number, word)` of every `ArrayAdd` on a grid cell.
    fn record_cell_adds(e: &mut Engine, grid: Ptr<NavMeshGrid>) -> Rc<RefCell<Vec<(u32, u16)>>> {
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        let adds = Rc::new(RefCell::new(Vec::new()));
        let log = adds.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            log.borrow_mut()
                .push(((a[0] - cells) / 16, e.mem.u16(a[1])));
            Ret::default()
        });
        adds
    }

    /// The row/column of a point in the test grid (10 by 10 cells from the
    /// origin, clamped), as `006912a0` computes them.
    fn register_cell_of_point(e: &mut Engine) {
        e.register(GRID_CELL_OF_POINT, |e, a| {
            let (x, y) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let column = ((x / 10.0).floor() as i32).clamp(0, 2) as u32;
            let row = ((y / 10.0).floor() as i32).clamp(0, 2) as u32;
            e.mem.set_u32(a[2], row);
            e.mem.set_u32(a[3], column);
            Ret::default()
        });
    }

    fn inside_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
        let side = |u: [f32; 2], v: [f32; 2]| {
            (v[0] - u[0]) * (p[1] - u[1]) - (v[1] - u[1]) * (p[0] - u[0])
        };
        let (s1, s2, s3) = (side(a, b), side(b, c), side(c, a));
        (s1 >= 0.0 && s2 >= 0.0 && s3 >= 0.0) || (s1 <= 0.0 && s2 <= 0.0 && s3 <= 0.0)
    }

    fn segments_cross(a: [f32; 2], b: [f32; 2], c: [f32; 2], d: [f32; 2]) -> bool {
        let orient = |p: [f32; 2], q: [f32; 2], r: [f32; 2]| {
            (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
        };
        orient(a, b, c) * orient(a, b, d) < 0.0 && orient(c, d, a) * orient(c, d, b) < 0.0
    }

    // --- 006959b0 ------------------------------------------------------

    #[test]
    fn grid_lists_each_triangle_in_the_cells_it_touches() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                // 0: inside cell (1, 1) only.
                [12.0, 12.0, 0.0],
                [14.0, 12.0, 0.0],
                [12.0, 14.0, 0.0],
                // 1: a large triangle over most of the grid.
                [1.0, 1.0, 0.0],
                [25.0, 1.0, 0.0],
                [1.0, 25.0, 0.0],
                // 2: a thin sliver along the bottom row.
                [1.0, 1.0, 0.0],
                [29.0, 1.5, 0.0],
                [1.0, 2.0, 0.0],
                // 3: at the far corner; 30 is clamped to the last cell.
                [30.0, 30.0, 0.0],
                [29.5, 29.5, 0.0],
                [29.0, 30.0, 0.0],
            ],
            &[
                ([0, 1, 2], [0xffff; 3], 0),
                ([3, 4, 5], [0xffff; 3], 0),
                ([6, 7, 8], [0xffff; 3], 0),
                ([9, 10, 11], [0xffff; 3], 0),
            ],
        );
        let grid = mesh_grid(mesh);
        let clears = recorder(&mut e, GRID_CLEAR, 0);
        let setups = Rc::new(RefCell::new(Vec::new()));
        let log = setups.clone();
        e.register_double(GRID_SETUP, move |e, a| {
            log.borrow_mut()
                .push((a.to_vec(), read_point(e, a[1]), read_point(e, a[2])));
            let grid = Ptr::<NavMeshGrid>::new(a[0]);
            e.set(grid, NavMeshGrid::iGridSize, GRID_SIZE);
            e.set(grid, NavMeshGrid::fColumnSectionLen, 10.0f32);
            e.set(grid, NavMeshGrid::fRowSectionLen, 10.0f32);
            let cells = e.mem.alloc(GRID_SIZE * GRID_SIZE * 16);
            e.set(grid, NavMeshGrid::GridData, Ptr::new(cells));
            Ret::default()
        });
        e.register(GRID_CELL_CORNER, |e, a| {
            write_point(e, a[1], [a[2] as f32 * 10.0, a[3] as f32 * 10.0, 0.0]);
            ret(a[1])
        });
        e.register(TRIANGLE_RAY_TEST, |e, a| {
            let corner = |p: u32| [e.mem.f32(p), e.mem.f32(p + 4)];
            let hit = inside_triangle(corner(a[3]), corner(a[0]), corner(a[1]), corner(a[2]));
            e.mem.set_f32(a[4], 0.0);
            ret(hit as u32)
        });
        e.register(SEGMENTS_INTERSECT_2D, |e, a| {
            let p = |i: usize| [e.mem.f32(a[i]), e.mem.f32(a[i] + 4)];
            ret(segments_cross(p(0), p(1), p(2), p(3)) as u32)
        });
        // The cells are read after the setup double allocated them: record
        // them lazily.
        let adds = Rc::new(RefCell::new(Vec::new()));
        let log = adds.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            let cells = e.mem.u32(grid.addr() + 0x24);
            log.borrow_mut()
                .push(((a[0] - cells) / 16, e.mem.u16(a[1])));
            Ret::default()
        });

        fn_006959b0(&mut e, mesh);
        assert_eq!(*clears.borrow(), vec![vec![grid.addr()]]);
        let setups = setups.borrow();
        assert_eq!(setups.len(), 1);
        // The grid was sized from the bounds of the mesh and 4 triangles.
        assert_eq!(setups[0].0[3], 4);
        assert_eq!(setups[0].1, [1.0, 1.0, 0.0]);
        assert_eq!(setups[0].2, [30.0, 30.0, 0.0]);

        let cells_of = |triangle: u16| -> Vec<u32> {
            let mut cells: Vec<u32> = adds
                .borrow()
                .iter()
                .filter(|(_, t)| *t == triangle)
                .map(|(cell, _)| *cell)
                .collect();
            cells.sort();
            cells
        };
        assert_eq!(cells_of(0), vec![4]);
        // The large triangle: cell (1, 2) is the only one of its rectangle
        // it misses. Cells (2, 1) and (2, 2) are listed because the game
        // tests the second vertex's x range against the third vertex's y
        // for the low side of the cell, so the vertex at (25, 1) accepts the
        // whole right column.
        assert_eq!(cells_of(1), vec![0, 1, 2, 3, 4, 5, 6, 8]);
        // The sliver reaches the middle cell only through the edge crossing.
        assert_eq!(cells_of(2), vec![0, 1, 2]);
        assert_eq!(cells_of(3), vec![8]);
    }

    #[test]
    fn grid_of_an_empty_mesh_is_only_cleared() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let clears = recorder(&mut e, GRID_CLEAR, 0);
        let setups = recorder(&mut e, GRID_SETUP, 0);
        fn_006959b0(&mut e, mesh);
        assert_eq!(clears.borrow().len(), 1);
        assert!(setups.borrow().is_empty());
    }

    // --- 00696500, 00696540, 006967c0 -----------------------------------

    #[test]
    fn cell_add_appends_to_the_cell_of_the_coordinates() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = set_up_grid(&mut e, mesh);
        let adds = record_cell_adds(&mut e, grid);
        // Cell (2, 1) is number 1 * 3 + 2.
        fn_00696500(&mut e, grid, 0x1234, 2, 1);
        assert_eq!(*adds.borrow(), vec![(5, 0x1234)]);
    }

    #[test]
    fn segment_test_copies_the_points_to_2d_points_in_order() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENTS_INTERSECT_2D, move |e, a| {
            let points: Vec<[f32; 2]> = a
                .iter()
                .map(|p| [e.mem.f32(*p), e.mem.f32(*p + 4)])
                .collect();
            log.borrow_mut().push(points);
            ret(1)
        });
        let a = point(&mut e, [1.0, 2.0, 9.0]);
        let b = point(&mut e, [3.0, 4.0, 9.0]);
        let c = point(&mut e, [5.0, 6.0, 9.0]);
        let d = point(&mut e, [7.0, 8.0, 9.0]);
        assert!(fn_00696540(&mut e, a, b, c, d));
        assert_eq!(
            *seen.borrow(),
            vec![vec![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]]]
        );
        e.register(SEGMENTS_INTERSECT_2D, |_, _| ret(0));
        assert!(!fn_00696540(&mut e, a, b, c, d));
    }

    /// 00696540 pushes each coordinate with `FLD`/`FSTP float` (a signalling
    /// NaN goes quiet), builds d's copy first at EBP-0x08 and a's last at
    /// EBP-0x20, and hands `006ba010` the constructor's results (`PUSH EAX`).
    #[test]
    fn segment_test_pushes_quiet_coordinates_and_the_constructors_results() {
        let mut e = engine();
        let built: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = built.clone();
        e.register_double(POINT2_CONSTRUCT, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(a[0] ^ 0x8000_0000)
        });
        let tested = recorder(&mut e, SEGMENTS_INTERSECT_2D, 1);
        let a = point(&mut e, [1.0, 2.0, 9.0]);
        let b = point(&mut e, [3.0, 4.0, 9.0]);
        let c = point(&mut e, [5.0, 6.0, 9.0]);
        let d = point(&mut e, [7.0, 8.0, 9.0]);
        // d's x: a signalling NaN.
        e.mem.set_u32(d.addr(), 0x7fa0_0001);
        assert!(fn_00696540(&mut e, a, b, c, d));
        let built = built.borrow();
        let slots: Vec<u32> = built.iter().map(|call| call[0]).collect();
        // d, c, b, a: 8 bytes apart, downwards.
        assert_eq!(
            slots,
            vec![slots[3] + 24, slots[3] + 16, slots[3] + 8, slots[3]]
        );
        assert_eq!(built[0][1..], [0x7fe0_0001, 8.0f32.to_bits()]);
        assert_eq!(built[3][1..], [1.0f32.to_bits(), 2.0f32.to_bits()]);
        let results: Vec<u32> = [3, 2, 1, 0]
            .iter()
            .map(|&slot| slots[slot] ^ 0x8000_0000)
            .collect();
        assert_eq!(&*tested.borrow(), &[results]);
    }

    #[test]
    fn cell_address_is_row_major() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = set_up_grid(&mut e, mesh);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        assert_eq!(
            fn_006967c0(&mut e, grid, 1, 2).addr(),
            cells + (2 * 3 + 1) * 16
        );
        assert_eq!(fn_006967c0(&mut e, grid, 0, 0).addr(), cells);
    }

    // --- 006965d0 and 006967f0 ---------------------------------------------

    fn two_triangle_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[
                // A large triangle: x from 1 to 25, y from 1 to 25.
                [1.0, 1.0, 0.0],
                [25.0, 1.0, 0.0],
                [1.0, 25.0, 0.0],
                // A small one in the middle cell.
                [12.0, 12.0, 0.0],
                [14.0, 12.0, 0.0],
                [12.0, 14.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn range_add_lists_the_triangle_rectangles() {
        let mut e = engine();
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        register_cell_of_point(&mut e);
        let adds = record_cell_adds(&mut e, grid);
        assert!(fn_006965d0(&mut e, mesh, 0, 1));
        let adds = adds.borrow();
        // Triangle 0 covers rows 0 to 2 and columns 0 to 2, row by row;
        // triangle 1 only the middle cell.
        let first: Vec<(u32, u16)> = (0..9).map(|cell| (cell, 0)).collect();
        assert_eq!(adds[..9], first[..]);
        assert_eq!(adds[9..], [(4, 1)]);

        // A range of one triangle.
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        let adds = record_cell_adds(&mut e, grid);
        assert!(fn_006965d0(&mut e, mesh, 1, 1));
        assert_eq!(*adds.borrow(), vec![(4, 1)]);
    }

    #[test]
    fn range_remove_deletes_the_triangle_from_each_cell() {
        let mut e = engine();
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        register_cell_of_point(&mut e);
        let finds = Rc::new(RefCell::new(Vec::new()));
        let log = finds.clone();
        e.register_double(ARRAY_FIND_WORD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2], a[3]));
            ret(7)
        });
        let removes = recorder(&mut e, WORD_ARRAY_REMOVE, 0);
        assert!(fn_006967f0(&mut e, mesh, 1, 1));
        assert_eq!(
            *finds.borrow(),
            vec![(cells + 4 * 16, 1, 0, WORD_COMPARATOR)]
        );
        assert_eq!(*removes.borrow(), vec![vec![cells + 4 * 16, 7, 1]]);

        finds.borrow_mut().clear();
        removes.borrow_mut().clear();
        assert!(fn_006967f0(&mut e, mesh, 0, 0));
        assert_eq!(finds.borrow().len(), 9);
        assert_eq!(removes.borrow().len(), 9);
        // Triangle 0, cells in row-major order.
        assert_eq!(finds.borrow()[0], (cells, 0, 0, WORD_COMPARATOR));
        assert_eq!(finds.borrow()[8], (cells + 8 * 16, 0, 0, WORD_COMPARATOR));
    }

    // --- 006969e0, 00696a10 -----------------------------------------------

    #[test]
    fn grid_bounds_are_copied_out() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = mesh_grid(mesh);
        write_point(&mut e, grid.addr() + 0x0c, [1.0, 2.0, 3.0]);
        write_point(&mut e, grid.addr() + 0x18, [4.0, 5.0, 6.0]);
        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        fn_00696a10(&mut e, grid, min, max);
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);

        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        assert!(fn_006969e0(&mut e, mesh, min, max));
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);
    }

    // --- triangle lookup by point ----------------------------------------

    /// A mesh of `markers.len()` triangles; triangle `i` has the vertices
    /// `3i`, `3i + 1`, `3i + 2`, the first at `(markers[i], 0, 0)` and the
    /// others at the origin, so a test double of the ray test can tell the
    /// triangles apart by the first vertex's x. `flags` are the triangle
    /// flags (default 0).
    fn marked_mesh(e: &mut Engine, markers: &[f32], flags: &[(usize, u32)]) -> Ptr<NavMesh> {
        let vertices: Vec<[f32; 3]> = markers
            .iter()
            .flat_map(|m| [[*m, 0.0, 0.0], [0.0; 3], [0.0; 3]])
            .collect();
        let triangles: Vec<TriangleSpec> = (0..markers.len() as u16)
            .map(|i| {
                let flag = flags
                    .iter()
                    .find(|(t, _)| *t == i as usize)
                    .map_or(0, |f| f.1);
                ([3 * i, 3 * i + 1, 3 * i + 2], [0xffff; 3], flag)
            })
            .collect();
        mesh_with(e, &vertices, &triangles)
    }

    /// The ray test answers by the first vertex's x: `(marker, Some(height))`
    /// hits with that height, `(marker, None)` misses.
    fn register_ray_table(e: &mut Engine, table: &[(f32, Option<f32>)]) -> Calls {
        let table = table.to_vec();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(TRIANGLE_RAY_TEST, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let marker = e.mem.f32(a[0]);
            match table
                .iter()
                .find(|(m, _)| *m == marker)
                .and_then(|(_, h)| *h)
            {
                Some(height) => {
                    e.mem.set_f32(a[4], height);
                    ret(1)
                }
                None => ret(0),
            }
        });
        calls
    }

    /// Makes the grid answer with a cell list holding `triangles`, or with
    /// no list.
    fn register_cell_list(e: &mut Engine, triangles: Option<&[u16]>) {
        let list = triangles.map(|triangles| {
            let list = e.mem.alloc(16);
            put_array(
                e,
                Ptr::new(list),
                &words16(triangles),
                triangles.len() as u32,
            );
            list
        });
        e.register_double(GRID_CELL_LIST, move |e, a| {
            e.mem.set_u32(a[2], list.unwrap_or(0));
            Ret::default()
        });
    }

    #[test]
    fn triangle_for_point_picks_the_nearest_height() {
        let mut e = engine();
        let mesh = marked_mesh(
            &mut e,
            &[100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0],
            &[(0, 0x20)],
        );
        register_ray_table(
            &mut e,
            &[
                (100.0, Some(10.0)),
                (200.0, Some(50.0)),
                (300.0, Some(-60.0)),
                (400.0, Some(30.0)),
                (500.0, Some(70.0)),
                (600.0, Some(-200.0)),
                (700.0, None),
            ],
        );
        let at = point(&mut e, [0.0; 3]);
        let find = |e: &mut Engine, list: Option<&[u16]>| {
            register_cell_list(e, list);
            fn_00696a50(e, mesh, at, Ptr::NULL)
        };
        // Triangle 0 is flagged and ignored; 1 at distance 50 beats 2 at 60.
        assert_eq!(find(&mut e, Some(&[0, 1, 2])), 1);
        // A triangle within 40 ends the search at once.
        assert_eq!(find(&mut e, Some(&[1, 3, 2])), 3);
        // More than 64 above is never accepted.
        assert_eq!(find(&mut e, Some(&[4])), 0xffff);
        // No hit, no list.
        assert_eq!(find(&mut e, Some(&[6])), 0xffff);
        assert_eq!(find(&mut e, None), 0xffff);

        // Far below the point (-200): rejected unless the point is not
        // more than 50 above the ground of the parent cell.
        assert_eq!(find(&mut e, Some(&[5])), 0xffff);
        let cell = 0x9000u32;
        e.set(mesh, NavMesh::pParentCell, Ptr::new(cell));
        let ground = Rc::new(RefCell::new(-1000.0f32));
        let value = ground.clone();
        e.register_double(CELL_GROUND_HEIGHT, move |e, a| {
            assert_eq!(a[0], 0x9000);
            e.mem.set_f32(a[2], *value.borrow());
            Ret::default()
        });
        assert_eq!(find(&mut e, Some(&[5])), 0xffff);
        *ground.borrow_mut() = 1000.0;
        assert_eq!(find(&mut e, Some(&[5])), 5);
    }

    #[test]
    fn point_has_a_triangle_when_one_is_found() {
        let mut e = engine();
        let mesh = marked_mesh(&mut e, &[100.0], &[]);
        register_ray_table(&mut e, &[(100.0, Some(5.0))]);
        let at = point(&mut e, [0.0; 3]);
        register_cell_list(&mut e, Some(&[0]));
        assert!(fn_00697670(&mut e, mesh, at, Ptr::NULL));
        register_cell_list(&mut e, None);
        assert!(!fn_00697670(&mut e, mesh, at, Ptr::NULL));
    }

    #[test]
    fn grid_call_is_forwarded() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let calls = recorder(&mut e, GRID_FORWARDED, 0);
        fn_00696c00(&mut e, mesh, 1, 2, 3);
        assert_eq!(*calls.borrow(), vec![vec![mesh_grid(mesh).addr(), 1, 2, 3]]);
    }

    // --- copy operators ---------------------------------------------------

    fn filled(e: &mut Engine, size: u32) -> Ptr {
        let block = e.mem.alloc(size);
        for i in 0..size {
            e.mem.set_u8(block + i, (i + 1) as u8);
        }
        Ptr::new(block)
    }

    #[test]
    fn copy_operators_copy_their_parts() {
        let mut e = engine();
        e.register(COPY_STRUCT_16, |e, a| {
            for i in 0..16 {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        let source = filled(&mut e, 0x58);

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696ca0(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x20),
            e.mem.bytes(source.addr(), 0x20)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x20, 0x38)
            .iter()
            .all(|b| *b == 0));

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696c60(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x44),
            e.mem.bytes(source.addr(), 0x44)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x44, 0x14)
            .iter()
            .all(|b| *b == 0));

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696c30(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x44),
            e.mem.bytes(source.addr(), 0x44)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x44, 0xc)
            .iter()
            .all(|b| *b == 0));
        assert_eq!(
            e.mem.u32(target.addr() + 0x50),
            e.mem.u32(source.addr() + 0x50)
        );
        assert_eq!(e.mem.u32(target.addr() + 0x54), 0);
    }

    // --- 00696cf0 ------------------------------------------------------------

    /// The center of triangle `i` is `(10 i, 0, 0)`.
    fn register_centers(e: &mut Engine) {
        e.register(GET_CENTER, |e, a| {
            write_point(e, a[1], [10.0 * a[2] as f32, 0.0, 0.0]);
            ret(a[1])
        });
    }

    #[test]
    fn closest_triangle_prefers_a_height_then_the_nearest_center() {
        let mut e = engine();
        register_centers(&mut e);
        let at = point(&mut e, [0.0; 3]);
        let out_triangle = Ptr::new(e.mem.alloc(4));
        let out_found = Ptr::new(e.mem.alloc(4));
        let run = |e: &mut Engine,
                   mesh: Ptr<NavMesh>,
                   table: &[(f32, Option<f32>)],
                   list: Option<&[u16]>| {
            register_ray_table(e, table);
            register_cell_list(e, list);
            let result =
                nav_mesh_find_closest_triangle_for_location(e, mesh, at, out_triangle, out_found);
            (
                result,
                e.mem.u16(out_triangle.addr()),
                e.mem.u8(out_found.addr()),
            )
        };
        let mesh = marked_mesh(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0], &[]);
        // Without a cell list every triangle is scanned: 2 and 3 have a
        // height (50, 45), the nearer one wins although neither is within 40.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, Some(50.0)),
            (4.0, Some(-45.0)),
            (5.0, None),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 3, 1));
        // A height difference within 40 returns at once.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, Some(50.0)),
            (4.0, Some(30.0)),
            (5.0, Some(1.0)),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 3, 1));
        // Nothing has a height: the nearest center, triangle 0.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, None),
            (4.0, None),
            (5.0, None),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 0, 0));
        // With a list, the flagged triangle is skipped: only triangle 2 and
        // 4 are candidates and the center of 2 is nearer.
        let flagged = marked_mesh(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0], &[(2, 0x20)]);
        assert_eq!(run(&mut e, flagged, &table, Some(&[2, 4])), (true, 4, 0));
        // An empty mesh has nothing.
        let empty = marked_mesh(&mut e, &[], &[]);
        assert_eq!(run(&mut e, empty, &[], None), (false, 0xffff, 0));
    }

    // --- 00697010 -----------------------------------------------------------

    /// `GetEdgeVertices`: the addresses of the two vertices of an edge.
    fn register_edge_vertices(e: &mut Engine) {
        e.register(GET_EDGE_VERTICES, |e, a| {
            let triangle = e.mem.u32(a[0] + 0x38 + 4) + (a[2] & 0xffff) * 16;
            let vertices = e.mem.u32(a[0] + 0x28 + 4);
            let first = e.mem.u16(triangle + 2 * a[3]) as u32;
            let second = e.mem.u16(triangle + 2 * ((a[3] + 1) % 3)) as u32;
            e.mem.set_u32(a[1], vertices + 12 * first);
            e.mem.set_u32(a[1] + 4, vertices + 12 * second);
            ret(a[1])
        });
    }

    fn square_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        // Two triangles A B C and B C D of a square of side 10; the edge
        // B-C is shared.
        mesh_with(
            e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [10.0, 10.0, 0.0],
            ],
            &[
                ([0, 1, 2], [0xffff, 1, 0xffff], 0),
                ([1, 3, 2], [0xffff, 0xffff, 0], 0),
            ],
        )
    }

    #[test]
    fn border_edge_search_finds_the_nearest_edge_without_neighbour() {
        let mut e = engine();
        register_edge_vertices(&mut e);
        let mesh = square_mesh(&mut e);
        let out_triangle = Ptr::new(e.mem.alloc(4));
        let out_edge = Ptr::new(e.mem.alloc(4));
        let near = point(&mut e, [6.0, 1.0, 0.0]);
        assert!(fn_00697010(
            &mut e,
            mesh,
            near,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
        // The bottom edge A-B of triangle 0 (midpoint (5, 0)).
        assert_eq!(e.mem.u16(out_triangle.addr()), 0);
        assert_eq!(e.mem.i32(out_edge.addr()), 0);

        // The right edge of triangle 1 (B-D, edge 0, midpoint (10, 5)).
        let right = point(&mut e, [9.0, 5.0, 0.0]);
        assert!(fn_00697010(
            &mut e,
            mesh,
            right,
            1.0,
            out_triangle,
            out_edge,
            true
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 1);
        assert_eq!(e.mem.i32(out_edge.addr()), 0);

        // Above the height range, the large radius admits several edges.
        // The executable compares planar distance with the previous full
        // distance, so the later triangle 1 edge 1 replaces triangle 0 edge 0.
        let high = point(&mut e, [6.0, 1.0, 500.0]);
        e.mem.set_u16(out_triangle.addr(), 0x4444);
        assert!(!fn_00697010(
            &mut e,
            mesh,
            high,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 0x4444);
        assert!(fn_00697010(
            &mut e,
            mesh,
            high,
            1000.0,
            out_triangle,
            out_edge,
            false
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 1);
        assert_eq!(e.mem.i32(out_edge.addr()), 1);

        // A mesh whose edges all have neighbours has no border edge.
        let closed = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[([0, 1, 2], [0, 0, 0], 0)],
        );
        assert!(!fn_00697010(
            &mut e,
            closed,
            near,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
    }

    // --- 006971e0, 00697290, 00697440 ----------------------------------------

    #[test]
    fn triangle_normal_is_the_unit_cross_of_two_edges() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[[1.0, 1.0, 5.0], [3.0, 1.0, 5.0], [1.0, 4.0, 5.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        );
        let out = point(&mut e, [9.0; 3]);
        assert_eq!(fn_006971e0(&mut e, mesh, out, 0), out);
        assert_eq!(read_point(&e, out.addr()), [0.0, 0.0, 1.0]);
    }

    fn circumcenter_mesh(e: &mut Engine, z: [f32; 3]) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[[0.0, 0.0, z[0]], [4.0, 0.0, z[1]], [0.0, 4.0, z[2]]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn circumcenter_has_the_highest_z() {
        let mut e = engine();
        let out = point(&mut e, [9.0; 3]);
        for (z, expected) in [
            ([1.0, 5.0, 3.0], 5.0),
            ([7.0, 2.0, 3.0], 7.0),
            ([1.0, 9.0, 9.0], 9.0),
            ([1.0, 2.0, 3.0], 3.0),
        ] {
            let mesh = circumcenter_mesh(&mut e, z);
            assert_eq!(fn_00697290(&mut e, mesh, out, 0), out);
            assert_eq!(read_point(&e, out.addr()), [2.0, 2.0, expected]);
        }
    }

    #[test]
    fn circumcenter_in_the_plane_has_zero_z() {
        let mut e = engine();
        let mesh = circumcenter_mesh(&mut e, [1.0, 2.0, 3.0]);
        let out = point(&mut e, [9.0; 3]);
        fn_00697440(&mut e, mesh, 0, out);
        assert_eq!(read_point(&e, out.addr()), [2.0, 2.0, 0.0]);
    }

    // --- 00697590 ---------------------------------------------------------

    #[test]
    fn portal_triangles_list_the_triangles_leading_to_the_info() {
        let mut e = engine();
        let info = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        // Triangle 0: edge 1 (slot 0) and edge 2 (slot 1) both lead to
        // `info`; triangle 1 has no extra info; triangle 2: edge 0 (slot 2)
        // leads to `other`; triangle 3: edge 2 (slot 1) leads to `info`.
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 1, 2], [0xffff, 0, 1], 0b110),
                ([0, 1, 2], [0xffff; 3], 0),
                ([0, 1, 2], [2, 0xffff, 0xffff], 0b001),
                ([0, 1, 2], [0xffff, 0xffff, 1], 0b100),
            ],
        );
        let infos: Vec<u8> = [info, info, other]
            .iter()
            .flat_map(|target| words(&[0, *target, 0]))
            .collect();
        put_array(&mut e, Ptr::new(mesh.addr() + 0x48), &infos, 3);
        e.register(GET_EXTRA_EDGE_INFO, |e, a| {
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * (a[1] & 0xffff))
        });
        let sizes = recorder(&mut e, ARRAY_SET_SIZE, 0);
        let reserves = recorder(&mut e, ARRAY_SET_RESERVED_SIZE, 0);
        e.register(ARRAY_ADD_DWORD, |e, a| {
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        let result = e.mem.alloc(16);
        let buffer = e.mem.alloc(64);
        e.mem.set_u32(result + 4, buffer);
        assert!(nav_mesh_find_portal_triangles(
            &mut e,
            mesh,
            Ptr::new(info),
            Ptr::new(result)
        ));
        assert_eq!(array_words(&e, result), vec![0, 3]);
        assert_eq!(*sizes.borrow(), vec![vec![result, 0, 1]]);
        assert_eq!(*reserves.borrow(), vec![vec![result, 100]]);

        // Another info matches nothing.
        e.mem.set_u32(result + 8, 0);
        assert!(!nav_mesh_find_portal_triangles(
            &mut e,
            mesh,
            Ptr::new(0x1234),
            Ptr::new(result)
        ));
    }

    // --- 006976a0 ---------------------------------------------------------

    #[test]
    fn edge_info_is_created_on_demand_and_reset() {
        let mut e = engine();
        e.register(GET_EDGE_EXTRA_INFO, |e, a| {
            let triangle = e.mem.u32(a[0] + 0x38 + 4) + (a[1] & 0xffff) * 16;
            if (e.mem.u32(triangle + 12) >> a[2]) & 1 == 0 {
                return ret(0);
            }
            let slot = e.mem.u16(triangle + 6 + 2 * a[2]) as u32;
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * slot)
        });
        let constructs = recorder(&mut e, EDGE_INFO_CONSTRUCTOR, 0);
        e.register(ADD_EXTRA_EDGE_INFO, |e, a| {
            let size = e.mem.u32(a[0] + 0x48 + 8);
            e.mem.set_u32(a[0] + 0x48 + 8, size + 1);
            ret(size)
        });
        // Edge 1 of triangle 0 has no info; the mesh has room for 4.
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [0xffff; 3], 0)]);
        let buffer = e.mem.alloc(48);
        e.mem.set_u32(mesh.addr() + 0x48 + 4, buffer);
        e.mem.set_u32(buffer + 4, 0x5555);
        let info = fn_006976a0(&mut e, mesh, 0, 1);
        assert_eq!(constructs.borrow().len(), 1);
        // The new info is number 0: the edge now points at it.
        let tri = triangle_ptr(&e, mesh, 0);
        assert_eq!(
            triangle_words(&e, tri),
            ([0, 1, 2], [0xffff, 0, 0xffff], 0b010)
        );
        assert_eq!(info.addr(), buffer);
        // Reset to "no other mesh".
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(e.mem.u16(buffer + 8), 0xffff);

        // An existing info is only reset.
        e.mem.set_u32(buffer + 4, 0x7777);
        e.mem.set_u16(buffer + 8, 3);
        let again = fn_006976a0(&mut e, mesh, 0, 1);
        assert_eq!(again, info);
        assert_eq!(constructs.borrow().len(), 1);
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(e.mem.u16(buffer + 8), 0xffff);
    }

    // --- 00697730, 00697810 -------------------------------------------------

    #[test]
    fn ray_test_passes_copies_of_the_three_vertices() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[[9.0; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
            &[([1, 2, 3], [0xffff; 3], 0)],
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(TRIANGLE_RAY_TEST, move |e, a| {
            log.borrow_mut().push((
                [
                    read_point(e, a[0]),
                    read_point(e, a[1]),
                    read_point(e, a[2]),
                ],
                a[3],
                a[4],
                (arg_f32(a[5]), arg_f32(a[6])),
            ));
            ret(1)
        });
        let at = point(&mut e, [0.0; 3]);
        let out = Ptr::new(e.mem.alloc(4));
        assert!(fn_00697730(&mut e, mesh, 0, at, out, 3.5, 4.5));
        assert_eq!(
            *seen.borrow(),
            vec![(
                [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
                at.addr(),
                out.addr(),
                (3.5, 4.5)
            )]
        );
        e.register(TRIANGLE_RAY_TEST, |_, _| ret(0));
        assert!(!fn_00697730(&mut e, mesh, 0, at, out, 3.5, 4.5));
    }

    #[test]
    fn ray_test_with_a_callback_reports_the_hit_point() {
        let mut e = engine();
        let mesh = marked_mesh(&mut e, &[100.0], &[]);
        let table_calls = register_ray_table(&mut e, &[(100.0, Some(5.0))]);
        let at = point(&mut e, [1.0, 2.0, 10.0]);
        let out = Ptr::new(e.mem.alloc(4));
        // Without a callback the limits are FLT_MAX.
        assert!(fn_00697810(&mut e, mesh, 0, at, out, Ptr::NULL));
        let call = table_calls.borrow()[0].clone();
        assert_eq!((arg_f32(call[5]), arg_f32(call[6])), (f32::MAX, f32::MAX));

        // With a callback its virtual slot 0 gets the point, the mesh, the
        // triangle and the point raised by the height, and its answer counts.
        let callback = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(callback, vtable);
        e.mem.set_u32(vtable, 0x0500_0000);
        let hits = Rc::new(RefCell::new(Vec::new()));
        let log = hits.clone();
        e.register_double(0x0500_0000, move |e, a| {
            log.borrow_mut().push((a.to_vec(), read_point(e, a[4])));
            ret(0)
        });
        assert!(!fn_00697810(&mut e, mesh, 0, at, out, Ptr::new(callback)));
        assert_eq!(
            *hits.borrow(),
            vec![(
                vec![callback, at.addr(), mesh.addr(), 0, hits.borrow()[0].0[4]],
                [1.0, 2.0, 15.0]
            )]
        );

        // A miss never calls the callback.
        let miss = marked_mesh(&mut e, &[200.0], &[]);
        assert!(!fn_00697810(&mut e, miss, 0, at, out, Ptr::new(callback)));
        assert_eq!(hits.borrow().len(), 1);
    }

    // --- 00697980 ---------------------------------------------------------

    #[test]
    fn height_over_a_triangle() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 10.0],
                // A flat (degenerate) triangle.
                [0.0, 0.0, 0.0],
                [5.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        );
        let out = Ptr::new(e.mem.alloc(4));
        let check = |e: &mut Engine, triangle: u16, p: [f32; 3], above: f32, below: f32| {
            let p = point(e, p);
            nav_mesh_compute_triangle_z_for_location(e, mesh, triangle, p, above, below, out)
        };
        // Over (2, 2) the surface is at z = 2 (the third vertex has z = 10
        // and weight 0.2).
        assert!(check(&mut e, 0, [2.0, 2.0, 3.0], 5.0, 5.0));
        assert!((e.mem.f32(out.addr()) - 2.0).abs() < 1e-5);
        // 1 above the surface is within 0.5? No.
        assert!(!check(&mut e, 0, [2.0, 2.0, 3.0], 0.5, 5.0));
        // 8 under the surface is within 10 under, not 5.
        assert!(check(&mut e, 0, [2.0, 2.0, -6.0], 5.0, 10.0));
        assert!(!check(&mut e, 0, [2.0, 2.0, -6.0], 5.0, 5.0));
        // Outside the triangle (each weight can be the negative one).
        assert!(!check(&mut e, 0, [-1.0, 2.0, 0.0], 5.0, 5.0));
        assert!(!check(&mut e, 0, [2.0, -1.0, 0.0], 5.0, 5.0));
        assert!(!check(&mut e, 0, [8.0, 8.0, 0.0], 5.0, 5.0));
        // On the border counts.
        assert!(check(&mut e, 0, [5.0, 0.0, 0.0], 5.0, 5.0));
        // A flat triangle never.
        assert!(!check(&mut e, 1, [5.0, 0.0, 0.0], 5.0, 5.0));
    }

    #[test]
    fn height_search_in_the_cell_list() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [20.0, 20.0, 0.0],
                [30.0, 20.0, 0.0],
                [20.0, 30.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        );
        let out = Ptr::new(e.mem.alloc(4));
        let over_second = point(&mut e, [22.0, 22.0, 1.0]);
        register_cell_list(&mut e, Some(&[0, 1]));
        assert!(fn_00697b70(&mut e, mesh, over_second, 5.0, 5.0, out));
        assert_eq!(e.mem.f32(out.addr()), 0.0);
        let nowhere = point(&mut e, [50.0, 50.0, 1.0]);
        assert!(!fn_00697b70(&mut e, mesh, nowhere, 5.0, 5.0, out));
        register_cell_list(&mut e, None);
        assert!(!fn_00697b70(&mut e, mesh, over_second, 5.0, 5.0, out));
    }

    // --- 00697c10, 00697d40 -----------------------------------------------

    /// One triangle `(0,0,0) (10,0,0) (0,10,0)` and a second one so the
    /// first can have neighbours on all sides.
    fn neighbour_mesh(e: &mut Engine, neighbours: [u16; 3]) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [10.0, 10.0, 0.0],
            ],
            &[([0, 1, 2], neighbours, 0), ([1, 3, 2], [0, 0, 0], 0)],
        )
    }

    fn register_geometry(e: &mut Engine) {
        register_edge_vertices(e);
        // Distance of a point to a 2D segment.
        e.register(POINT_SEGMENT_DISTANCE, |e, a| {
            let p = read_point(e, a[0]);
            let (u, v) = (read_point(e, a[1]), read_point(e, a[2]));
            let (dx, dy) = (v[0] - u[0], v[1] - u[1]);
            let t =
                (((p[0] - u[0]) * dx + (p[1] - u[1]) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            let (cx, cy) = (u[0] + t * dx, u[1] + t * dy);
            ret_float(((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt())
        });
        // Any triangle is hit by any point; the triangle's height is 0.
        e.register(TRIANGLE_RAY_TEST, |e, a| {
            e.mem.set_f32(a[4], 0.0);
            ret(1)
        });
    }

    #[test]
    fn position_is_pushed_away_from_border_edges() {
        let mut e = engine();
        register_geometry(&mut e);
        register_cell_list(&mut e, Some(&[0]));
        // Triangle 0 (edges 0, 1, 2) has a neighbour only on edge 1.
        let mesh = neighbour_mesh(&mut e, [0xffff, 1, 0xffff]);
        let position = point(&mut e, [5.0, 1.0, 0.0]);
        let result = point(&mut e, [9.0; 3]);
        assert_eq!(fn_00697d40(&mut e, mesh, result, position, 3.0), result);
        // Edge 0 (the bottom, distance 1 < 3) pushes by 2 along the normal
        // (0, 0, 1) crossed with the edge vector (-10, 0, 0), normalized:
        // (0, -1, 0). Edge 2 (distance 5) is too far.
        assert_eq!(read_point(&e, result.addr()), [5.0, -1.0, 0.0]);
        // Neighbour 1 (edges 0, 1, 2 of the second triangle): its edge 1
        // and 2 have no neighbour: edge 1 (3 to 2) is the far side.
        // The position is left alone otherwise.
        assert_eq!(read_point(&e, position.addr()), [5.0, 1.0, 0.0]);

        // Without a triangle under the position nothing moves.
        register_cell_list(&mut e, None);
        let result = point(&mut e, [9.0; 3]);
        fn_00697d40(&mut e, mesh, result, position, 3.0);
        assert_eq!(read_point(&e, result.addr()), [5.0, 1.0, 0.0]);
    }

    #[test]
    fn random_point_is_kept_when_it_lies_over_the_mesh() {
        let mut e = engine();
        register_geometry(&mut e);
        // Both triangles are closed: nothing to push away from.
        let mesh = neighbour_mesh(&mut e, [1, 1, 1]);
        e.register(FLOAT_SQRT, |_, a| ret_float(arg_f32(a[0]).sqrt()));
        let draws = Rc::new(RefCell::new(Vec::new()));
        let log = draws.clone();
        let mut next = 0;
        let values = [3.0f32, 4.0];
        e.register_double(RANDOM_FLOAT, move |_, a| {
            log.borrow_mut().push((arg_f32(a[0]), arg_f32(a[1])));
            next += 1;
            ret_float(values[(next - 1) % 2])
        });
        // The base vector keeps its z.
        write_point(&mut e, RANDOM_POINT_BASE, [0.0, 0.0, 7.0]);
        let origin = point(&mut e, [1.0, 1.0, 0.0]);
        let result = point(&mut e, [9.0; 3]);
        register_cell_list(&mut e, Some(&[0]));
        assert!(fn_00697c10(&mut e, mesh, origin, 5.0, result));
        // x in [-5, 5], then y in [-3, 3] (the half chord at x = 3 is
        // sqrt(25 - 9) = 4: the second range is (-4, 4)).
        assert_eq!(*draws.borrow(), vec![(-5.0, 5.0), (-4.0, 4.0)]);
        // Offset (3, 4, 7) plus the origin.
        assert_eq!(read_point(&e, result.addr()), [4.0, 5.0, 7.0]);

        // If no draw lands over a triangle, 8 attempts are made.
        register_cell_list(&mut e, None);
        draws.borrow_mut().clear();
        let result = point(&mut e, [9.0; 3]);
        assert!(!fn_00697c10(&mut e, mesh, origin, 5.0, result));
        assert_eq!(draws.borrow().len(), 16);
        assert_eq!(read_point(&e, result.addr()), [9.0; 3]);
    }

    // --- 00698030 -----------------------------------------------------------

    #[test]
    fn packed_edge_value_is_four_bits_per_edge() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [0xffff; 3], 0x0a35_0000 | 0x2f)]);
        let tri = triangle_ptr(&e, mesh, 0);
        // Edge n's bits start at bit 16 + 6 n.
        assert_eq!(fn_00698030(&mut e, tri, 0), 0x5);
        assert_eq!(fn_00698030(&mut e, tri, 1), (0x0a35_002f >> 22) & 0xf);
        assert_eq!(fn_00698030(&mut e, tri, 2), 0);
    }

    // --- 00698060, 00698160, 00698300, 00698320 ---------------------------------

    fn crossing_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        // (0,0) (10,0) (0,10): edge 0 along the bottom, edge 1 the
        // hypotenuse, edge 2 up the left.
        mesh_with(
            e,
            &[[0.0, 0.0, 1.0], [10.0, 0.0, 2.0], [0.0, 10.0, 3.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn edge_segment_test_returns_the_first_edge_that_hits() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENT_SEGMENT_TEST, move |e, a| {
            log.borrow_mut().push((
                [
                    e.mem.f32(a[0]),
                    e.mem.f32(a[0] + 4),
                    e.mem.f32(a[1]),
                    e.mem.f32(a[1] + 4),
                ],
                a[2..].to_vec(),
            ));
            // Hits the segment (10, 0) - (0, 10) only.
            ret((e.mem.f32(a[0]) == 10.0) as u32)
        });
        assert_eq!(fn_00698060(&mut e, mesh, 0, -1, 7, 8, 9), 1);
        assert_eq!(
            *seen.borrow(),
            vec![
                ([0.0, 0.0, 10.0, 0.0], vec![7, 8, 9]),
                ([10.0, 0.0, 0.0, 10.0], vec![7, 8, 9]),
            ]
        );
        // The skipped edge is not tested; no other edge hits.
        seen.borrow_mut().clear();
        assert_eq!(fn_00698060(&mut e, mesh, 0, 1, 7, 8, 9), -1);
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(seen.borrow()[1].0, [0.0, 10.0, 0.0, 0.0]);
    }

    #[test]
    fn intersecting_edge_reports_type_edge_and_point() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENT_INTERSECTION_2D, move |e, a| {
            log.borrow_mut()
                .push((read_point(e, a[0]), read_point(e, a[1]), a[2], a[3]));
            if e.mem.f32(a[0]) == 10.0 {
                write_point(e, a[4], [5.0, 5.0, 0.0]);
                return ret(1);
            }
            ret(0)
        });
        let query = point(&mut e, [20.0, 20.0, 0.0]);
        let result = Ptr::new(e.mem.alloc(20));
        let returned = nav_mesh_find_interecting_edge2(&mut e, mesh, result, 0, -1, 0xabc, query);
        assert_eq!(returned, result);
        assert_eq!(e.mem.u32(result.addr()), 3);
        assert_eq!(e.mem.i32(result.addr() + 4), 1);
        assert_eq!(read_point(&e, result.addr() + 8), [5.0, 5.0, 0.0]);
        assert_eq!(
            seen.borrow()[1],
            ([10.0, 0.0, 2.0], [0.0, 10.0, 3.0], 0xabc, query.addr())
        );

        // Without a crossing: type 0, edge -1.
        e.register(SEGMENT_INTERSECTION_2D, |_, _| ret(0));
        let result = Ptr::new(e.mem.alloc(20));
        nav_mesh_find_interecting_edge2(&mut e, mesh, result, 0, -1, 0xabc, query);
        assert_eq!(e.mem.u32(result.addr()), 0);
        assert_eq!(e.mem.i32(result.addr() + 4), -1);

        // The zero-length edge 0 is never tested, and neither is the skipped edge.
        let degenerate = mesh_with(
            &mut e,
            &[[1.0; 3], [1.0; 3], [5.0, 5.0, 5.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        );
        let calls = recorder(&mut e, SEGMENT_INTERSECTION_2D, 0);
        nav_mesh_find_interecting_edge2(&mut e, degenerate, result, 0, 2, 0, query);
        assert_eq!(calls.borrow().len(), 1);
        calls.borrow_mut().clear();
        nav_mesh_find_interecting_edge2(&mut e, degenerate, result, 0, -1, 0, query);
        assert_eq!(calls.borrow().len(), 2);
    }

    #[test]
    fn intersection_result_constructor_builds_the_point() {
        let mut e = engine();
        let calls = recorder(&mut e, POINT3_DEFAULT_CONSTRUCTOR, 0);
        let result = Ptr::new(e.mem.alloc(20));
        assert_eq!(fn_00698300(&mut e, result), result);
        assert_eq!(*calls.borrow(), vec![vec![result.addr() + 8]]);
    }

    #[test]
    fn segment_crossing_edge() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let segment = |e: &mut Engine, from: [f32; 3], to: [f32; 3], skip: i32| {
            let (from, to) = (point(e, from), point(e, to));
            fn_00698320(e, mesh, 0, skip, from, to)
        };
        // Straight through the bottom edge.
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 3.0, 0.0], -1), 0);
        // The skipped edge is passed over, nothing else is crossed (the end
        // point inside the triangle lies off the hypotenuse and the left
        // edge).
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 3.0, 0.0], 0), -1);
        // Through the hypotenuse.
        assert_eq!(segment(&mut e, [4.0, 4.0, 0.0], [8.0, 8.0, 0.0], -1), 1);
        // Through the left edge.
        assert_eq!(segment(&mut e, [-3.0, 4.0, 0.0], [3.0, 4.0, 0.0], -1), 2);
        // Ending exactly on an edge is not a crossing of that edge (the
        // second end point lies on its line).
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 0.0, 0.0], 1), -1);
        // A segment on both sides of the hypotenuse's line but beyond the
        // triangle's end: the edge's end points are on the same side.
        assert_eq!(segment(&mut e, [20.0, 0.0, 0.0], [0.0, 20.0, 0.0], 0), -1);
        // Collinear segments never count.
        assert_eq!(segment(&mut e, [-5.0, 0.0, 0.0], [5.0, 0.0, 0.0], 1), -1);
    }

    // --- NiPoint2 and segment helpers ------------------------------------------

    #[test]
    fn segment_constructor_copies_both_points() {
        let mut e = engine();
        let first = point(&mut e, [1.0, 2.0, 3.0]);
        let second = point(&mut e, [4.0, 5.0, 6.0]);
        let target = Ptr::new(e.mem.alloc(24));
        assert_eq!(fn_00698580(&mut e, target, first, second), target);
        assert_eq!(read_point(&e, target.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, target.addr() + 12), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn two_element_array_is_vector_constructed() {
        let mut e = engine();
        let calls = recorder(&mut e, VECTOR_CONSTRUCTOR_ITERATOR, 0);
        let target = Ptr::new(e.mem.alloc(40));
        assert_eq!(fn_006985c0(&mut e, target), target);
        assert_eq!(
            *calls.borrow(),
            vec![vec![target.addr() + 4, 16, 2, ELEMENT_16_CONSTRUCTOR]]
        );
    }

    #[test]
    fn point2_copy_and_product() {
        let mut e = engine();
        let source = point(&mut e, [1.5, -2.5, 99.0]);
        let target = point(&mut e, [0.0, 0.0, 7.0]);
        assert_eq!(fn_006985f0(&mut e, target, source), target);
        // Only x and y are copied.
        assert_eq!(read_point(&e, target.addr()), [1.5, -2.5, 7.0]);

        let a = point(&mut e, [1.0, 2.0, 0.0]);
        let b = point(&mut e, [3.0, 4.0, 0.0]);
        let result = point(&mut e, [0.0, 0.0, 7.0]);
        assert_eq!(fn_00698620(&mut e, a, result, b), result);
        // (1 * 3 + 2 * 4, 2 * 3 - 1 * 4)
        assert_eq!(read_point(&e, result.addr()), [11.0, 2.0, 7.0]);

        // Five calls compared with FalloutNV.exe 1.4.0.525 through nv-call;
        // compare the result words bit for bit (source vectors/results are
        // in the translator audit, not repository artifacts).
        let vectors = [
            ([1.5, -2.5], [3.0, 4.0], [0xc0b0_0000, 0xc158_0000]),
            ([0.0, 0.0], [2.0, 3.0], [0x0000_0000, 0x0000_0000]),
            ([-10.0, 20.0], [0.25, -0.5], [0xc148_0000, 0x0000_0000]),
            ([0.1, 0.2], [0.3, 0.4], [0x3de1_47af, 0x3ca3_d70b]),
            (
                [10_000.125, -10_000.25],
                [0.000_125, -0.000_25],
                [0x4070_0148, 0x3fa0_0000],
            ),
        ];
        for (left, right, expected_bits) in vectors {
            let a = point(&mut e, [left[0], left[1], 0.0]);
            let b = point(&mut e, [right[0], right[1], 0.0]);
            let result = point(&mut e, [0.0; 3]);
            fn_00698620(&mut e, a, result, b);
            assert_eq!(
                [
                    e.mem.f32(result.addr()).to_bits(),
                    e.mem.f32(result.addr() + 4).to_bits()
                ],
                expected_bits
            );
        }
    }

    #[test]
    fn navmesh_info_array_destructor_sets_derived_vtable_and_clears() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0084_54f0, 0);
        fn_0069af10(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), NAV_MESH_INFO_POINTER_ARRAY_VTABLE);
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 1]]);
    }

    #[test]
    fn navmesh_edge_handle_array_constructor_sets_vtable_and_initializes() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0069_ba70, 0);
        assert_eq!(fn_0069af30(&mut e, this), this);
        assert_eq!(e.mem.u32(this.addr()), NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 0, 0]]);
    }

    #[test]
    fn navmesh_edge_handle_array_destructor_clears() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0084_54f0, 0);
        fn_0069af60(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE);
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 1]]);
    }

    #[test]
    fn navmesh_point_storage_allocation_multiplies_count_by_twelve() {
        let mut e = Engine::new();
        let calls = recorder(&mut e, 0x0040_1000, 0x7000);
        assert_eq!(fn_0069af80(&mut e, Ptr::NULL, 5), Ptr::new(0x7000));
        assert_eq!(&*calls.borrow(), &[vec![60]]);
    }

    #[test]
    fn obstacle_map_get_at_finds_and_assigns_pointer_value() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let table = e.mem.alloc(4);
        let vtable = e.mem.alloc(16);
        let item = e.mem.alloc(16);
        let out = Ptr::new(e.mem.alloc(4));
        let hash = 0x00ab_1000;
        let equal = 0x00ab_1010;
        e.map(vtable, 16);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 4, 1);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(table, item);
        e.mem.set_u32(item, 0);
        e.mem.set_u16(item + 4, 7);
        e.mem.set_u16(item + 6, 0xcccc);
        e.mem.set_u32(item + 8, 0x7788);
        e.register(hash, |_, _| ret(0));
        e.register(equal, |_, a| ret((a[1] == a[2]) as u32));
        // 0069b004..0069b00e: `006e5cc0` (this `out`) with the item's +8, the `NiPointer`
        // `SetValue` (00691930) stores; not `0042f4c0` (the +0x1c-adjusting assignment).
        let assign = recorder(&mut e, 0x006e_5cc0, out.addr());
        let wrong_assign = recorder(&mut e, POINTER_ASSIGN, 0);
        assert!(ni_t_map_base_obstacle_get_at(&mut e, map, 7, out));
        assert_eq!(&*assign.borrow(), &[vec![out.addr(), item + 8]]);
        assert!(wrong_assign.borrow().is_empty());
    }

    #[test]
    fn edge_handle_array_append_grows_and_copies_record() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let buffer = e.mem.alloc(24);
        let source = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr() + 4, buffer);
        e.mem.set_u32(source.addr(), 0x1122_3344);
        e.mem.set_u16(source.addr() + 4, 0x5566);
        recorder(&mut e, 0x0084_5460, 2);
        recorder(&mut e, 0x0069_27d0, 0);
        assert_eq!(fn_0069b030(&mut e, this, source), 2);
        assert_eq!(e.mem.u32(buffer + 16), 0x1122_3344);
        assert_eq!(e.mem.u16(buffer + 20), 0x5566);
    }

    #[test]
    fn edge_handle_append_reloads_buffer_after_constructor_relocation() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let old_buffer = e.mem.alloc(24);
        let new_buffer = e.mem.alloc(24);
        let source = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr() + 4, old_buffer);
        e.mem.set_u32(old_buffer + 16, 0xaaaa_bbbb);
        e.mem.set_u32(old_buffer + 20, 0xcccc_dddd);
        e.mem.set_u32(new_buffer + 20, 0xeeee_ffff);
        e.mem.set_u32(source.addr(), 0x1122_3344);
        e.mem.set_u16(source.addr() + 4, 0x5566);
        recorder(&mut e, 0x0084_5460, 2);
        e.register_double(0x0069_27d0, move |e, a| {
            assert_eq!(a, &[this.addr(), old_buffer + 16, 1]);
            e.mem.set_u32(this.addr() + 4, new_buffer);
            ret(0)
        });
        assert_eq!(fn_0069b030(&mut e, this, source), 2);
        assert_eq!(e.mem.u32(new_buffer + 16), 0x1122_3344);
        assert_eq!(e.mem.u32(new_buffer + 20), 0xeeee_5566);
        assert_eq!(e.mem.u32(old_buffer + 16), 0xaaaa_bbbb);
        assert_eq!(e.mem.u32(old_buffer + 20), 0xcccc_dddd);
    }

    #[test]
    fn undo_operation_reserves_arrays_and_reloads_mutated_members() {
        let mut e = Engine::new();
        let operation = Ptr::new(e.mem.alloc(12));
        let mesh = Ptr::new(e.mem.alloc(0x48));
        let undo = Ptr::new(e.mem.alloc(0x20));
        let replacement_mesh = e.mem.alloc(0x48);
        let replacement_undo = e.mem.alloc(0x20);
        e.mem.set_u32(undo.addr() + 0x10, 17);
        e.mem.set_u32(undo.addr() + 0x18, 19);
        e.mem.set_u32(replacement_undo + 0x18, 23);
        e.mem.set_u32(operation.addr() + 4, 0x1234_5678);
        e.register_double(0x006b_ebd0, move |e, a| {
            assert_eq!(a, &[operation.addr()]);
            assert_eq!(e.mem.u32(operation.addr()), mesh.addr());
            assert_eq!(e.mem.u32(operation.addr() + 8), undo.addr());
            ret(0)
        });
        e.register_double(0x0069_4890, move |e, a| {
            assert_eq!(a, &[mesh.addr() + 0x28, 17, 1]);
            e.mem.set_u32(operation.addr(), replacement_mesh);
            e.mem.set_u32(operation.addr() + 8, replacement_undo);
            ret(0)
        });
        let triangles = recorder(&mut e, 0x0069_49d0, 0);
        assert!(fn_006bc8e0(&mut e, operation, mesh, undo));
        assert_eq!(
            &*triangles.borrow(),
            &[vec![replacement_mesh + 0x38, 23, 1]]
        );
        assert_eq!(e.mem.u32(operation.addr() + 4), 0x1234_5678);
    }

    #[test]
    fn closed_door_collection_selects_smallest_extent_and_releases_temporaries() {
        for (extent, chosen) in [
            ([1.0f32, -2.0, 3.0], 0),
            ([3.0, -1.0, 2.0], 1),
            ([2.0, 3.0, -1.0], 2),
            ([3.0, 2.0, -1.0], 2),
            ([2.0, 2.0, 2.0], 2),
            ([f32::NAN, 2.0, 3.0], 1),
        ] {
            let mut e = Engine::new();
            let mesh = e.new_object::<NavMesh>();
            let bounds = Ptr::new(e.mem.alloc(0x58));
            e.map(0x011f_4000, 0x1000);
            e.map(0x0106_b000, 0x1000);
            e.mem.set_f64(0x0106_b168, 0.49000000953674316);
            for axis in 0..3 {
                e.mem.set_f32(0x011f_426c + axis * 4, 10.0 + axis as f32);
            }
            e.register_double(0x0043_9ef0, move |e, a| {
                assert_eq!(a[0], bounds.addr() + 0x4c);
                assert_eq!(a[2], bounds.addr() + 0x40);
                for (axis, value) in extent.iter().enumerate() {
                    e.mem.set_f32(a[1] + axis as u32 * 4, *value);
                }
                ret(a[1])
            });
            e.register(0x0040_8840, |_, a| ret_float(f32::from_bits(a[0]).abs()));
            recorder(&mut e, 0x0069_17d0, 0);
            recorder(&mut e, 0x0063_3c90, 0);
            e.register_double(0x0069_87d0, move |e, a| {
                assert_eq!(a[0], mesh.addr());
                for (axis, component) in extent.iter().enumerate() {
                    let expected = if axis == chosen {
                        (-f64::from(component.abs()) * 0.49000000953674316) as f32
                    } else {
                        10.0 + axis as f32
                    };
                    assert_eq!(e.mem.f32(a[2] + axis as u32 * 4), expected);
                }
                ret(0)
            });
            recorder(&mut e, ARRAY_SIZE, 0);
            let reference_destroy = recorder(&mut e, 0x0045_cec0, 0);
            let array_destroy = recorder(&mut e, 0x0069_1800, 0);
            assert!(fn_006997e0(&mut e, mesh, Ptr::new(0x1234), bounds));
            assert_eq!(reference_destroy.borrow().len(), 1);
            assert_eq!(array_destroy.borrow().len(), 1);
        }
    }

    #[test]
    fn closed_door_collection_appends_absent_and_different_door_records() {
        for old_door in [None, Some(0x1234u32), Some(0x5678u32)] {
            let mut e = Engine::new();
            let mesh = e.new_object::<NavMesh>();
            let bounds = Ptr::new(e.mem.alloc(0x58));
            let element = e.mem.alloc(2);
            let existing = e.mem.alloc(8);
            let buffer = e.mem.alloc(16);
            e.mem.set_u16(element, 7);
            e.mem.set_u16(existing + 4, 7);
            e.mem.set_u32(existing, old_door.unwrap_or(0));
            e.mem.set_u32(mesh.addr() + 0x6c, buffer);
            e.map(0x011f_4000, 0x1000);
            e.map(0x0106_b000, 0x1000);
            e.mem.set_f64(0x0106_b168, 0.49);
            e.register(0x0043_9ef0, |e, a| {
                for axis in 0..3 {
                    e.mem.set_f32(a[1] + axis * 4, 2.0);
                }
                ret(a[1])
            });
            e.register(0x0040_8840, |_, a| ret_float(f32::from_bits(a[0]).abs()));
            for address in [
                0x0069_17d0,
                0x0063_3c90,
                0x0069_87d0,
                0x0045_cec0,
                0x0069_1800,
                0x0069_2870,
                0x0069_27d0,
            ] {
                recorder(&mut e, address, 0);
            }
            e.register_double(ARRAY_SIZE, move |_, a| {
                ret(if a[0] == mesh.addr() + 0x68 {
                    u32::from(old_door.is_some())
                } else {
                    1
                })
            });
            recorder(&mut e, 0x005e_eda0, element);
            recorder(&mut e, 0x0069_50d0, existing);
            recorder(&mut e, 0x0055_82f0, 0x8765);
            recorder(&mut e, 0x0084_5460, 1);
            let flag = recorder(&mut e, 0x0069_1570, 0);
            assert!(fn_006997e0(&mut e, mesh, Ptr::new(0x1234), bounds));
            if old_door == Some(0x1234) {
                assert!(flag.borrow().is_empty());
                assert_eq!(e.mem.u32(buffer + 8), 0);
            } else {
                assert_eq!(&*flag.borrow(), &[vec![0x8765, 0x1000]]);
                assert_eq!(e.mem.u32(buffer + 8), 0x1234);
                assert_eq!(e.mem.u16(buffer + 12), 7);
            }
        }
    }

    #[test]
    fn portal_repair_covers_rejections_unique_fallback_and_endpoint_matching() {
        // mode: null source, inactive source, no portal, prohibited holder,
        // empty reference, missing neighbor, then the candidate cases.
        for mode in 0..15 {
            let mut e = Engine::new();
            let mesh = e.new_object::<NavMesh>();
            let neighbor = e.new_object::<NavMesh>();
            let source_triangle = e.mem.alloc(16);
            let neighbor_triangle = e.mem.alloc(16);
            let extra = e.mem.alloc(12);
            let holder = e.mem.alloc(0x58);
            let target = e.mem.alloc(12);
            e.mem.set_u32(extra + 4, holder);
            e.mem.set_u16(extra + 8, 7);
            e.mem.set_u32(holder + 0x54, 0x1111);
            e.mem.set_u32(target, 0xaaaa);
            e.map(0x0106_b000, 0x1000);
            e.mem.set_f64(0x0106_b178, 400.0);
            recorder(&mut e, 0x0055_8200, 1);
            e.register_double(0x0055_82f0, move |_, a| {
                if a[0] == mesh.addr() {
                    assert_eq!(a[1], 0);
                    ret(if mode == 0 { 0 } else { source_triangle })
                } else {
                    assert_eq!(a, &[neighbor.addr(), 7]);
                    ret(if mode == 5 { 0 } else { neighbor_triangle })
                }
            });
            recorder(&mut e, 0x0069_0e10, u32::from(mode != 1));
            e.register_double(0x0068_f1d0, move |_, a| {
                ret(u32::from(if a[0] == source_triangle {
                    mode != 2 && a[1] == 0
                } else {
                    let count = if mode == 6 {
                        0
                    } else if mode == 7 {
                        1
                    } else {
                        2
                    };
                    a[1] < count
                }))
            });
            recorder(&mut e, 0x0068_f2c0, 9);
            e.register_double(0x0069_3d30, move |_, a| {
                assert_eq!(a, &[mesh.addr(), 9]);
                ret(extra)
            });
            recorder(&mut e, 0x0042_fb00, 0);
            recorder(&mut e, 0x0068_f320, u32::from(mode == 3));
            e.register_double(0x0069_0890, |_, a| {
                assert_eq!(a[1], 0x1111);
                ret(0)
            });
            recorder(
                &mut e,
                POINTER_GET,
                if mode == 4 { 0 } else { neighbor.addr() },
            );
            e.register_double(0x0068_f0c0, move |e, a| {
                assert_eq!(a.len(), 5);
                let first = if a[0] == mesh.addr() {
                    assert_eq!(&a[1..3], &[0, 0]);
                    1
                } else {
                    assert_eq!(a[0], neighbor.addr());
                    assert_eq!(a[1], u32::from(e.mem.u16(extra + 8)));
                    assert!(a[2] < 3);
                    3
                };
                e.mem.set_u32(a[3], first);
                e.mem.set_u32(a[4], first + 1);
                ret(0)
            });
            e.register(0x0043_9ef0, |e, a| {
                assert_eq!(a.len(), 3);
                let endpoints = (e.mem.u32(a[0]), e.mem.u32(a[2]));
                assert!(matches!(endpoints, (1, 4) | (2, 3)));
                ret(a[1])
            });
            let distances = Rc::new(RefCell::new(0usize));
            let distance_count = distances.clone();
            e.register_double(0x004a_7290, move |e, _| {
                let mut count = distance_count.borrow_mut();
                if mode == 14 && *count == 0 {
                    // Reload the changed neighbor index for the next
                    // candidate and the final insertion.
                    e.mem.set_u16(extra + 8, 11);
                }
                let value = match mode {
                    9 => 1.0,                     // Both endpoints match on candidate zero.
                    10 if *count % 2 == 0 => 1.0, // Second endpoint rejects.
                    11 => f32::NAN,               // Unordered comparison rejects.
                    12 if *count >= 1 => 1.0,     // Candidate one matches.
                    13 if *count % 2 == 0 => 1.0,
                    13 => f32::NAN, // Second endpoint unordered rejects.
                    14 if *count >= 1 => 1.0,
                    _ => 400.0, // Equality rejects; single candidate survives.
                };
                *count += 1;
                ret_float(value)
            });
            let selected = recorder(&mut e, GET_EDGE_EXTRA_INFO, target);
            recorder(&mut e, 0x0068_efb0, 0x1234);
            let destroyed = recorder(&mut e, 0x0042_fa40, 0);
            assert!(fn_0069aa20(&mut e, mesh));
            assert_eq!(destroyed.borrow().len(), usize::from(mode >= 3));
            if [7, 9, 12, 14].contains(&mode) {
                let expected_edge = u32::from(mode == 12 || mode == 14);
                let expected_triangle = if mode == 14 { 11 } else { 7 };
                assert_eq!(
                    &*selected.borrow(),
                    &[vec![neighbor.addr(), expected_triangle, expected_edge]]
                );
                assert_eq!(e.mem.u32(target), 3);
                assert_eq!(e.mem.u32(target + 4), 0x1234);
                assert_eq!(e.mem.u16(target + 8), 0);
            } else {
                assert!(selected.borrow().is_empty());
                assert_eq!(e.mem.u32(target), 0xaaaa);
            }
            if mode == 6 {
                assert_eq!(*distances.borrow(), 0);
            }
        }
    }
    #[test]
    fn obstacle_map_is_empty_reads_item_count() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        assert!(fn_0069b080(&mut e, map));
        e.mem.set_u32(map.addr() + 0xc, 1);
        assert!(!fn_0069b080(&mut e, map));
    }

    #[test]
    fn obstacle_map_value_lookup_copies_packed_value() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let table = e.mem.alloc(4);
        let vtable = e.mem.alloc(16);
        let item = e.mem.alloc(16);
        let out = Ptr::new(e.mem.alloc(4));
        let hash = 0x00ab_1020;
        let equal = 0x00ab_1030;
        e.map(vtable, 16);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 8, table);
        e.mem.set_u32(table, item);
        e.mem.set_u32(item, 0);
        e.mem.set_u16(item + 4, 9);
        e.mem.set_u32(item + 6, 0x1234_5678);
        e.register(hash, |_, _| ret(0));
        e.register(equal, |_, a| ret((a[1] == a[2]) as u32));
        assert!(fn_0069b0a0(&mut e, map, 9, out));
        assert_eq!(e.mem.u32(out.addr()), 0x1234_5678);
    }

    #[test]
    fn array_remove_destroys_shifts_and_reduces_size() {
        let mut e = Engine::new();
        let array = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(array.addr() + 4, 0x4000);
        e.mem.set_u32(array.addr() + 8, 8);
        let move_count = recorder(&mut e, 0x0042_f5a0, 3);
        let destroy = recorder(&mut e, 0x0082_2820, 0);
        let shift = recorder(&mut e, 0x0042_fb60, 0);
        fn_0069b120(&mut e, array, 2, 2);
        assert_eq!(&*move_count.borrow(), &[vec![4, 2]]);
        assert_eq!(&*destroy.borrow(), &[vec![array.addr(), 0x4008, 2]]);
        assert_eq!(&*shift.borrow(), &[vec![array.addr(), 0x4008, 0x4014, 3]]);
        assert_eq!(e.mem.u32(array.addr() + 8), 6);
    }

    #[test]
    fn triangle_index_comparator_compares_values_not_addresses() {
        let mut e = Engine::new();
        let first = Ptr::new(e.mem.alloc(2));
        let second = Ptr::new(e.mem.alloc(2));
        e.mem.set_u16(first.addr(), 12);
        e.mem.set_u16(second.addr(), 12);
        assert!(fn_0069b1b0(&mut e, first, second));
        e.mem.set_u16(second.addr(), 13);
        assert!(!fn_0069b1b0(&mut e, first, second));
    }

    #[test]
    fn boolean_array_destructor_sets_vtable_and_clears() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0084_54f0, 0);
        fn_0069b1d0(&mut e, this);
        assert_eq!(e.mem.u32(this.addr()), BOOLEAN_ARRAY_VTABLE);
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 1]]);
    }

    #[test]
    fn fast_edge_storage_allocation_multiplies_count_by_twelve() {
        let mut e = Engine::new();
        let calls = recorder(&mut e, 0x0040_1000, 0x7100);
        assert_eq!(fn_0069b1f0(&mut e, Ptr::NULL, 3), Ptr::new(0x7100));
        assert_eq!(&*calls.borrow(), &[vec![36]]);
    }

    #[test]
    fn memory_helper_wrapper_forwards_both_arguments() {
        let mut e = Engine::new();
        let calls = recorder(&mut e, 0x0042_f5d0, 0);
        fn_0069b220(&mut e, Ptr::NULL, Ptr::new(0x12), Ptr::new(0x34));
        assert_eq!(&*calls.borrow(), &[vec![0x12, 0x34]]);
    }

    #[test]
    fn word_array_search_calls_comparator_from_start_and_returns_index() {
        let mut e = Engine::new();
        let array = Ptr::new(e.mem.alloc(16));
        let buffer = e.mem.alloc(8);
        let key = Ptr::new(e.mem.alloc(2));
        e.mem.set_u32(array.addr() + 4, buffer);
        e.mem.set_u32(array.addr() + 8, 4);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let captured = calls.clone();
        e.register_double(0x00ab_1040, move |_, a| {
            captured.borrow_mut().push(a.to_vec());
            ret((a[0] == buffer + 4 && a[1] == key.addr()) as u32)
        });
        assert_eq!(fn_0069b240(&mut e, array, key, 1, 0x00ab_1040), 2);
        assert_eq!(calls.borrow().len(), 2);
    }

    #[test]
    fn unsigned_word_to_dword_comparator_checks_numeric_value() {
        let mut e = Engine::new();
        let first = Ptr::new(e.mem.alloc(2));
        let second = Ptr::new(e.mem.alloc(4));
        e.mem.set_u16(first.addr(), 0xfffe);
        e.mem.set_u32(second.addr(), 0xfffe);
        assert!(fn_0069b2a0(&mut e, first, second));
        e.mem.set_u32(second.addr(), 0xffff_fffe);
        assert!(!fn_0069b2a0(&mut e, first, second));
    }

    #[test]
    fn fast_edge_array_constructor_sets_vtable_and_initializes() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0069_ba70, 0);
        assert_eq!(fn_0069b2c0(&mut e, this), this);
        assert_eq!(
            e.mem.u32(this.addr()),
            FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE
        );
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 0, 0]]);
    }

    #[test]
    fn fast_edge_array_destructor_sets_vtable_and_clears() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0084_54f0, 0);
        fn_0069b2f0(&mut e, this);
        assert_eq!(
            e.mem.u32(this.addr()),
            FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE
        );
        assert_eq!(&*calls.borrow(), &[vec![this.addr(), 1]]);
    }

    #[test]
    fn fast_edge_array_append_constructs_and_copies_item() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let buffer = e.mem.alloc(36);
        let source = Ptr::new(e.mem.alloc(12));
        e.mem.set_u32(this.addr() + 4, buffer);
        let grow = recorder(&mut e, 0x0097_8bc0, 2);
        let construct = recorder(&mut e, 0x0069_b9d0, 0);
        let copy = recorder(&mut e, 0x006b_c870, 0);
        assert_eq!(fn_0069b310(&mut e, this, source), 2);
        assert_eq!(&*grow.borrow(), &[vec![this.addr()]]);
        assert_eq!(&*construct.borrow(), &[vec![this.addr(), buffer + 24, 1]]);
        assert_eq!(&*copy.borrow(), &[vec![buffer + 24, source.addr()]]);
    }

    #[test]
    fn fast_vertex_map_base_constructor_allocates_and_zeroes_buckets() {
        let mut e = Engine::new();
        let this = Ptr::new(e.mem.alloc(16));
        let allocate = recorder(&mut e, 0x00aa_1070, 0x7200);
        let clear = recorder(&mut e, 0x0040_3d30, 0);
        assert_eq!(fn_0069b360(&mut e, this, 8), this);
        assert_eq!(e.mem.u32(this.addr()), FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 8);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x7200);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0);
        assert_eq!(&*allocate.borrow(), &[vec![32]]);
        assert_eq!(&*clear.borrow(), &[vec![0x7200, 0, 32]]);
    }

    #[test]
    fn fast_vertex_map_inserts_key_and_value_into_empty_bucket() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let vtable = e.mem.alloc(0x20);
        let buckets = e.mem.alloc(8);
        let item = e.mem.alloc(16);
        let hash = 0x00ab_1100;
        let equal = 0x00ab_1110;
        let allocate = 0x00ab_1130;
        e.map(vtable, 0x20);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        e.mem.set_u32(vtable + 0xc, 0x0069_b6a0);
        e.mem.set_u32(vtable + 0x14, allocate);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 4, 2);
        e.mem.set_u32(map.addr() + 8, buckets);
        e.mem.set_u32(map.addr() + 0xc, 0);
        e.register(hash, |_, _| ret(1));
        e.register(equal, |_, _| ret(0));
        e.register_double(allocate, move |_, _| ret(item));
        fn_0069b3d0(&mut e, map, 0x1122_3344, 0x5566, 0x7788_99aa);
        assert_eq!(e.mem.u32(buckets + 4), item);
        assert_eq!(e.mem.u32(item), 0);
        assert_eq!(e.mem.u32(item + 4), 0x1122_3344);
        assert_eq!(e.mem.u16(item + 8), 0x5566);
        assert_eq!(e.mem.u32(item + 0xc), 0x7788_99aa);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
    }

    #[test]
    fn fast_vertex_map_updates_existing_key_without_allocating() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let vtable = e.mem.alloc(0x20);
        let buckets = e.mem.alloc(8);
        let item = e.mem.alloc(16);
        let hash = 0x00ab_1160;
        let equal = 0x00ab_1170;
        let allocate = 0x00ab_1180;
        e.map(vtable, 0x20);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        e.mem.set_u32(vtable + 0xc, 0x0069_b6a0);
        e.mem.set_u32(vtable + 0x14, allocate);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 4, 2);
        e.mem.set_u32(map.addr() + 8, buckets);
        e.mem.set_u32(map.addr() + 0xc, 1);
        e.mem.set_u32(buckets, item);
        e.mem.set_u32(item + 4, 0x1122_3344);
        e.mem.set_u32(item + 8, 0x5566);
        e.register(hash, |_, _| ret(0));
        e.register(equal, |_, a| ret((a[1] == a[3] && a[2] == a[4]) as u32));
        let allocate = recorder(&mut e, allocate, 0x7fff);

        fn_0069b3d0(&mut e, map, 0x1122_3344, 0x5566, 0xaabb_ccdd);

        assert_eq!(e.mem.u32(item + 0xc), 0xaabb_ccdd);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 1);
        assert!(allocate.borrow().is_empty());
    }

    #[test]
    fn fast_vertex_map_lookup_returns_matching_value() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let vtable = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(4);
        let item = e.mem.alloc(16);
        let out = Ptr::new(e.mem.alloc(4));
        let hash = 0x00ab_1140;
        let equal = 0x00ab_1150;
        e.map(vtable, 0x10);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(vtable + 8, equal);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 8, buckets);
        e.mem.set_u32(buckets, item);
        e.mem.set_u32(item, 0);
        e.mem.set_u32(item + 4, 0x1122_3344);
        e.mem.set_u32(item + 8, 0x5566);
        e.mem.set_u32(item + 0xc, 0xaabb_ccdd);
        e.register(hash, |_, _| ret(0));
        e.register(equal, |_, a| ret((a[1] == a[3]) as u32));
        assert!(fn_0069b4d0(&mut e, map, 0x1122_3344, 0x5566, out));
        assert_eq!(e.mem.u32(out.addr()), 0xaabb_ccdd);
    }

    #[test]
    fn fast_vertex_map_lookup_miss_preserves_output_value() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let vtable = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(4);
        let out = Ptr::new(e.mem.alloc(4));
        let hash = 0x00ab_1190;
        e.map(vtable, 0x10);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 8, buckets);
        e.mem.set_u32(out.addr(), 0xdead_beef);
        e.register(hash, |_, _| ret(0));
        assert!(!fn_0069b4d0(&mut e, map, 0x1234_5678, 0x9abc, out));
        assert_eq!(e.mem.u32(out.addr()), 0xdead_beef);
    }

    #[test]
    fn fast_vertex_map_iterator_copies_current_key_value_and_follows_link() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let iterator = Ptr::new(e.mem.alloc(4));
        let key_out = Ptr::new(e.mem.alloc(8));
        let value_out = Ptr::new(e.mem.alloc(4));
        let item = e.mem.alloc(16);
        let next = e.mem.alloc(16);
        e.mem.set_u32(iterator.addr(), item);
        e.mem.set_u32(item, next);
        e.mem.set_u32(item + 4, 0x1234_5678);
        e.mem.set_u16(item + 8, 0x9abc);
        e.mem.set_u32(item + 0xc, 0x5566_7788);
        fn_0069b560(&mut e, map, iterator, key_out, value_out);
        assert_eq!(e.mem.u32(iterator.addr()), next);
        assert_eq!(e.mem.u32(key_out.addr()), 0x1234_5678);
        assert_eq!(e.mem.u16(key_out.addr() + 4), 0x9abc);
        assert_eq!(e.mem.u32(value_out.addr()), 0x5566_7788);

        let vtable = e.mem.alloc(8);
        let buckets = e.mem.alloc(12);
        let later_item = e.mem.alloc(16);
        let hash = 0x00ab_11a0;
        e.map(vtable, 8);
        e.mem.set_u32(vtable + 4, hash);
        e.mem.set_u32(map.addr(), vtable);
        e.mem.set_u32(map.addr() + 4, 3);
        e.mem.set_u32(map.addr() + 8, buckets);
        e.mem.set_u32(buckets + 4, later_item);
        e.mem.set_u32(next, 0);
        e.mem.set_u32(next + 4, 0x8765_4321);
        e.mem.set_u16(next + 8, 0xdef0);
        e.mem.set_u32(next + 0xc, 0x1122_3344);
        e.register(hash, |_, _| ret(0));
        fn_0069b560(&mut e, map, iterator, key_out, value_out);
        assert_eq!(e.mem.u32(iterator.addr()), later_item);
        assert_eq!(e.mem.u32(key_out.addr()), 0x8765_4321);
        assert_eq!(e.mem.u16(key_out.addr() + 4), 0xdef0);
        assert_eq!(e.mem.u32(value_out.addr()), 0x1122_3344);
    }

    #[test]
    fn fast_vertex_hash_index_uses_native_hash_and_remainder() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(map.addr() + 4, 7);
        let key = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(key.addr(), 2);
        e.mem.set_u16(key.addr() + 4, 3);
        let hash = fn_0069b640(&mut e, key);
        assert_eq!(
            ni_t_map_fast_vertex_key_to_hash_index(&mut e, map, 2, 3),
            (hash as u32) % 7
        );
    }

    #[test]
    fn fast_vertex_hash_shifts_the_signed_first_word_and_adds_high_word() {
        let mut e = Engine::new();
        let key = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(key.addr(), 2);
        e.mem.set_u16(key.addr() + 4, 3);
        assert_eq!(fn_0069b640(&mut e, key), 0x0000_0002_0003_0000);
        e.mem.set_u32(key.addr(), u32::MAX);
        assert_eq!(fn_0069b640(&mut e, key), 0xffff_ffff_0003_0000);
        e.mem.set_u32(key.addr(), 0);
        e.mem.set_u16(key.addr() + 4, 0x8000);
        assert_eq!(fn_0069b640(&mut e, key), 0xffff_ffff_8000_0000);
    }

    #[test]
    fn fast_vertex_key_equality_forwards_both_struct_addresses() {
        let mut e = Engine::new();
        let calls = Rc::new(RefCell::new(None));
        let captured = calls.clone();
        e.register_double(0x006d_4ca0, move |e, a| {
            *captured.borrow_mut() = Some((a[0], a[1], e.mem.u32(a[0]), e.mem.u32(a[1])));
            ret(1)
        });
        assert!(ni_t_map_fast_vertex_is_keys_equal(
            &mut e,
            Ptr::NULL,
            1,
            2,
            3,
            4
        ));
        let captured = calls.borrow().unwrap();
        assert_ne!(captured.0, captured.1);
        assert_eq!((captured.2, captured.3), (1, 3));
    }

    #[test]
    fn fast_vertex_set_value_copies_key_and_writes_value() {
        let mut e = Engine::new();
        let item = Ptr::new(e.mem.alloc(16));
        ni_t_map_fast_vertex_set_value(&mut e, Ptr::NULL, item, 0x1122_3344, 0x5566, 0xaabb_ccdd);
        assert_eq!(e.mem.u32(item.addr() + 4), 0x1122_3344);
        assert_eq!(e.mem.u16(item.addr() + 8), 0x5566);
        assert_eq!(e.mem.u32(item.addr() + 0xc), 0xaabb_ccdd);
    }

    #[test]
    fn fast_vertex_map_destructor_removes_items_and_frees_buckets() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x7300);
        let remove = recorder(&mut e, 0x0043_8af0, 0);
        let free = recorder(&mut e, 0x00aa_10f0, 0);
        fn_0069b6d0(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE);
        assert_eq!(&*remove.borrow(), &[vec![map.addr()], vec![map.addr()]]);
        assert_eq!(&*free.borrow(), &[vec![0x7300]]);
    }

    #[test]
    fn fast_vertex_map_base_destructor_removes_items_and_frees_buckets() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x7400);
        let remove = recorder(&mut e, 0x0043_8af0, 0);
        let free = recorder(&mut e, 0x00aa_10f0, 0);
        fn_0069b730(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), FAST_NAV_MESH_VERTEX_MAP_BASE_VTABLE);
        assert_eq!(&*remove.borrow(), &[vec![map.addr()]]);
        assert_eq!(&*free.borrow(), &[vec![0x7400]]);
    }

    #[test]
    fn fast_vertex_map_new_item_forwards_to_allocator() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let calls = recorder(&mut e, 0x0069_b8a0, 0x7500);
        assert_eq!(fn_0069b760(&mut e, map), Ptr::new(0x7500));
        assert_eq!(&*calls.borrow(), &[vec![map.addr() + 0xc]]);
    }

    #[test]
    fn fast_vertex_map_delete_item_clears_value_and_releases_item() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        let item = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(item.addr() + 0xc, 0x7788);
        let free = recorder(&mut e, 0x006b_8310, 0);
        fn_0069b780(&mut e, map, item);
        assert_eq!(e.mem.u32(item.addr() + 0xc), 0);
        assert_eq!(&*free.borrow(), &[vec![map.addr() + 0xc, item.addr()]]);
    }

    #[test]
    fn scalar_deleting_array_destructors_free_only_when_flag_is_set() {
        for (destroyer, vtable) in [
            (
                fn_0069b7b0 as fn(&mut Engine, Ptr, u32) -> Ptr,
                NAV_MESH_INFO_POINTER_ARRAY_VTABLE,
            ),
            (fn_0069b7e0, NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE),
            (fn_0069b810, BOOLEAN_ARRAY_VTABLE),
            (fn_0069b840, FAST_NAV_MESH_EDGE_HANDLE_ARRAY_VTABLE),
        ] {
            let mut e = Engine::new();
            let this = Ptr::new(e.mem.alloc(16));
            let clear = recorder(&mut e, 0x0084_54f0, 0);
            let free = recorder(&mut e, 0x0040_1030, 0);
            assert_eq!(destroyer(&mut e, this, 0), this);
            assert_eq!(e.mem.u32(this.addr()), vtable);
            assert_eq!(clear.borrow().len(), 1);
            assert!(free.borrow().is_empty());
            destroyer(&mut e, this, 1);
            assert_eq!(free.borrow().len(), 1);
        }
    }

    #[test]
    fn fast_vertex_map_scalar_deleting_destructor_obeys_delete_flag() {
        let mut e = Engine::new();
        let map = Ptr::new(e.mem.alloc(16));
        e.mem.set_u32(map.addr() + 8, 0x7600);
        let remove = recorder(&mut e, 0x0043_8af0, 0);
        let free_table = recorder(&mut e, 0x00aa_10f0, 0);
        let free_object = recorder(&mut e, 0x0040_1030, 0);
        assert_eq!(fn_0069b870(&mut e, map, 0), map);
        assert!(free_object.borrow().is_empty());
        fn_0069b870(&mut e, map, 1);
        assert_eq!(free_object.borrow().len(), 1);
        assert_eq!(remove.borrow().len(), 2);
        assert_eq!(free_table.borrow().len(), 2);
    }

    #[test]
    fn fast_vertex_item_allocation_skips_constructor_when_allocation_fails() {
        let mut e = Engine::new();
        let allocation = recorder(&mut e, 0x0040_1000, 0);
        assert_eq!(fn_0069b8a0(&mut e, Ptr::NULL), Ptr::NULL);
        assert_eq!(&*allocation.borrow(), &[vec![16]]);
    }

    #[test]
    fn fast_vertex_item_allocation_constructs_non_null_item() {
        let mut e = Engine::new();
        let item = e.mem.alloc(16);
        let allocation = recorder(&mut e, 0x0040_1000, item);
        let constructor = recorder(&mut e, 0x0069_b920, item);
        assert_eq!(fn_0069b8a0(&mut e, Ptr::NULL), Ptr::new(item));
        assert_eq!(&*allocation.borrow(), &[vec![16]]);
        assert_eq!(&*constructor.borrow(), &[vec![item]]);
    }

    #[test]
    fn obstacle_map_wrappers_answer_their_helpers() {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(0x3300);
        // 006997b0 reads the map pointer at +0xe0.
        e.map(this.addr(), 0x100);
        recorder(&mut e, 0x0069_afb0, 1);
        assert!(fn_006997b0(&mut e, this, 7, Ptr::new(0x4400)));
        recorder(&mut e, 0x0069_b080, 0);
        recorder(&mut e, 0x0069_b0a0, 1);
        assert!(fn_0069a9d0(&mut e, this, 0x1234, Ptr::new(0x4400)));
        recorder(&mut e, 0x0042_f5d0, 0x7000);
        assert_eq!(
            fn_0069b220(&mut e, Ptr::NULL, Ptr::new(0x12), Ptr::new(0x34)),
            Ptr::new(0x7000)
        );
    }
}
