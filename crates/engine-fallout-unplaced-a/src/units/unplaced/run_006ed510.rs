//! `(unplaced)/run_006ed510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006ed510` calls on the member (`006f25d0`).
const CALLEE_006F25D0: u32 = 0x006f_25d0;

// Translated from 006ed510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `006f25d0` with `ECX` = `this + 0x2c` and passes on nothing (the callee returns no value).
pub fn fn_006ed510(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_006F25D0, &args![this.byte_add(0x2c)]);
}

/// `00877a30` (`thiscall`, `RET 4`; unit `fallout/misc/main.cpp`).
const SET_INDEX: u32 = 0x0087_7a30;

// Translated from 006f2270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): calls `00877a30` on `this` with the
/// word at +8 minus 1 and answers its value (the exe ends on that call and leaves its `EAX`,
/// the address the callers read through: `00877a30` answers an array element's address).
pub fn fn_006f2270(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.u32(this.addr() + 8).wrapping_sub(1);
    e.call(SET_INDEX, &args![this, value]).u32()
}

// The rest of this unit is `PathSmoother` code (by its fields: the Xbox PDB's `PathSmoother`,
// 0xBC bytes there; on PC its scrap heap at +0x30 is 0x10 bytes, so the arrays sit 4 bytes
// lower): `pBadNodes` +0x00, `pRequest` +0x04, `StartLocation` +0x08, `GoalLocation` +0x14,
// `Triangles` +0x20, `Edges` +0x24, `pSolution` +0x28, `fSmoothingRadius` +0x2C,
// `TheScrapHeap` +0x30, `LeftVerts` +0x40, `RightVerts` +0x54, `LeftEdges` +0x68,
// `RightEdges` +0x7C, `PathEdges` +0x90, `StraightenedPath` +0xA4. Its records, made on the
// scrap heap: `UniqueVert` (0x40: `Location` +0, `ShrunkLocation` +0xC, `PushDelta` +0x18,
// `hNavMeshTri` +0x24, `EdgeList` +0x2C), `BoundsEdge` (0x1C: `pStartVert` +0,
// `pEndVert` +4, `hNavMeshEdge` +8, `bOpenEdge` +0x14, `iEdgeIndex` +0x18) and `PathEdge`
// (0x58: `pLeftVert` +0, `pRightVert` +4, `Center` +8, `BestPointOnEdge` +0x14,
// `LeftShrunkLocation` +0x20, `RightShrunkLocation` +0x2C, `fTriangleNormalDelta` +0x38,
// `hNavMeshEdge` +0x3C, `iPathEdgeIndex` +0x48, `pPrevLeftEdge` +0x4C, `pPrevRightEdge`
// +0x50, `eEnterLeaveWater` +0x54). The engine map has no names for these functions.

/// `PathSmoother` (Xbox PDB) fields at their PC offsets.
const START_LOCATION: u32 = 0x08;
const GOAL_LOCATION: u32 = 0x14;
const TRIANGLES: u32 = 0x20;
const EDGES: u32 = 0x24;
const SOLUTION: u32 = 0x28;
const SMOOTHING_RADIUS: u32 = 0x2c;
const SCRAP_HEAP: u32 = 0x30;
const LEFT_VERTS: u32 = 0x40;
const RIGHT_VERTS: u32 = 0x54;
const LEFT_EDGES: u32 = 0x68;
const RIGHT_EDGES: u32 = 0x7c;
const PATH_EDGES: u32 = 0x90;
const STRAIGHTENED_PATH: u32 = 0xa4;

/// The point helpers (thiscall on a point): the default constructor `006815c0` (answers
/// `this`), the equality `004390c0` (other's address; `AL`), the subtraction `00439ef0`
/// (out, other; answers out), `004b3800` (out, other; answers out; in `globalfunc.cpp` by
/// the engine map) and `NiPoint3::Dot` (`004b6190`, engine map; other; ST0).
const POINT_DEFAULT: u32 = 0x0068_15c0;
const POINT_EQUAL: u32 = 0x0043_90c0;
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_OP_004B3800: u32 = 0x004b_3800;
const POINT_DOT: u32 = 0x004b_6190;
/// The arrays' helpers (thiscall on the array): the count (`0044ddc0`), add a word from its
/// address (`007cb2e0`, the engine map's `BSSimpleArray<BGSBodyPart*,1024>::AddUninitialized`
/// body), set the reserved size (`0084b5c0`, the map's
/// `BSSimpleArray<Actor*,1024>::SetReservedSize` body), the clear `008454f0` (one word),
/// the emptiness test `0076b610` (`AL`), the element address `00877a30` (index), the
/// search arrays' element addresses (`006a7af0` on the edges, `006dad90` and `006b3160` on
/// the triangles; index) and the edges' last element (`006ced70`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
const ARRAY_ADD: u32 = 0x007c_b2e0;
const ARRAY_RESERVE: u32 = 0x0084_b5c0;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
const ARRAY_ELEMENT: u32 = 0x0087_7a30;
const EDGE_AT: u32 = 0x006a_7af0;
const TRIANGLE_AT: u32 = 0x006d_ad90;
const TRIANGLE_AT_006B3160: u32 = 0x006b_3160;
/// The triangle array's first element (`006b3180`, thiscall).
const FIRST_TRIANGLE: u32 = 0x006b_3180;
const LAST_EDGE: u32 = 0x006c_ed70;
/// The navigation-mesh helpers: the handle word of a mesh (`0068efb0`, thiscall), the two
/// end points of an edge of a triangle (`0068f0c0`, thiscall on the mesh: triangle, edge,
/// out, out), `NavMesh::GetMatchingEdge_ov3` (`0068f460`, engine map; thiscall: triangle,
/// edge, out mesh, out triangle, out edge), a triangle's normal (`006971e0`, thiscall on
/// the mesh: out, triangle) and the cdecl segment test `00696540` (four words).
const NAV_MESH_HANDLE: u32 = 0x0068_efb0;
const EDGE_POINTS: u32 = 0x0068_f0c0;
const GET_MATCHING_EDGE: u32 = 0x0068_f460;
const TRIANGLE_NORMAL: u32 = 0x0069_71e0;
const SEGMENT_TEST_00696540: u32 = 0x0069_6540;
/// The request's radius (`0069ef80`, thiscall; ST0), the pathing location constructor
/// (`006dcf20`, thiscall: point, mesh handle) and destructor (`004ff7e0`), and the
/// solution's add (`0069f920`, thiscall: location, point).
const REQUEST_RADIUS: u32 = 0x0069_ef80;
const LOCATION_CONSTRUCT: u32 = 0x006d_cf20;
const LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const SOLUTION_ADD: u32 = 0x0069_f920;
/// The point `PathSmoother` code compares with and passes as "none" (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;
/// `ScrapHeap::Allocate` (`00aa54a0`, engine map; thiscall: size, alignment), its alignment
/// word (`010a2720`) and the placement step `006e6da0` (cdecl size, block; answers the
/// block or null).
const SCRAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
const PLACE: u32 = 0x006e_6da0;
/// The record members' constructors: the edge handle (`006240d0`, thiscall) and the
/// scrap array of the vertex's edges (`006f2510`, thiscall: 0, 0, the scrap heap).
const HANDLE_CONSTRUCT: u32 = 0x0062_40d0;
const SCRAP_ARRAY_CONSTRUCT: u32 = 0x006f_2510;
/// The `double` 0.0 the comparisons load (`01012060`).
const DOUBLE_ZERO: u32 = 0x0101_2060;

/// The address of the last element of the array at `this + offset` ([`fn_006f2270`]).
fn last_of(e: &mut Engine, this: Ptr, offset: u32) -> u32 {
    let at = fn_006f2270(e, this.byte_add(offset));
    e.mem.u32(at)
}

/// Adds the word at `slot` to the array at `array` (`007cb2e0`).
fn add_word(e: &mut Engine, array: u32, slot: u32) {
    e.call(ARRAY_ADD, &args![array, slot]);
}

/// A `size`-byte block from the smoother's scrap heap and the placement step on it:
/// (block, placed).
fn scrap_block(e: &mut Engine, this: Ptr, size: u32) -> (u32, u32) {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let heap = this.addr() + SCRAP_HEAP;
    let block = e.call(SCRAP_ALLOCATE, &args![heap, size, alignment]).u32();
    let placed = e.call(PLACE, &args![size, block]).u32();
    (block, placed)
}

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for word in [0u32, 4, 8] {
        let value = e.mem.u32(from.wrapping_add(word));
        e.mem.set_u32(to.wrapping_add(word), value);
    }
}

// Translated from 006ed530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0x20`; called from `006cd8a0`, `006d1c60` and
/// `006d1f80`): the smoothing of one path. Stores the request, the start and goal points
/// (copied), the triangle and edge arrays, the solution and the bad nodes, and the request's
/// radius (`0069ef80`, as a `float`). With a single triangle it only adds the start and the
/// goal to the solution ([`fn_006ed730`] with the triangle's mesh handle) and answers 1.
/// Otherwise it reserves twice the edge count in the five vertex and edge arrays
/// (`0084b5c0`), clears them (`008454f0(0)`), runs the steps [`fn_006ed850`],
/// [`fn_006eef80`], [`fn_006ef920`] (whose answer it keeps), [`fn_006f02e0`],
/// [`fn_006f03a0`], [`fn_006f0450`], [`fn_006f0d20`], [`fn_006f10d0`] and
/// [`fn_006f1440`], and, when `always_finish` is set or the bad nodes are empty
/// (`0076b610`), [`fn_006f1c10`]`(1, 1)` and [`fn_006f1d30`]. Answers the kept byte.
pub fn fn_006ed530(
    e: &mut Engine,
    this: Ptr,
    request: u32,
    triangles: u32,
    edges: u32,
    start: Ptr,
    goal: Ptr,
    solution: u32,
    bad_nodes: u32,
    always_finish: u8,
) -> u8 {
    let base = this.addr();
    e.mem.set_u32(base + 4, request);
    copy_point(e, base + START_LOCATION, start.addr());
    copy_point(e, base + GOAL_LOCATION, goal.addr());
    e.mem.set_u32(base + TRIANGLES, triangles);
    e.mem.set_u32(base + EDGES, edges);
    e.mem.set_u32(base + SOLUTION, solution);
    e.mem.set_u32(base, bad_nodes);
    let request = e.mem.u32(base + 4);
    let radius = x87_store32(e.call(REQUEST_RADIUS, &args![request]).st0);
    e.mem.set_u32(base + SMOOTHING_RADIUS, radius);
    if e.call(ARRAY_COUNT, &args![triangles]).u32() == 1 {
        for point in [start, goal] {
            let triangle = e.call(TRIANGLE_AT_006B3160, &args![triangles, 0u32]).u32();
            let mesh = e.mem.u32(triangle);
            let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
            fn_006ed730(e, this, point.addr(), handle);
        }
        return 1;
    }
    for array in [LEFT_VERTS, RIGHT_VERTS, LEFT_EDGES, RIGHT_EDGES, PATH_EDGES] {
        let count = e.call(ARRAY_COUNT, &args![edges]).u32();
        e.call(ARRAY_RESERVE, &args![base + array, count << 1]);
    }
    for array in [LEFT_VERTS, RIGHT_VERTS, LEFT_EDGES, RIGHT_EDGES, PATH_EDGES] {
        e.call(ARRAY_CLEAR, &args![base + array, 0u32]);
    }
    fn_006ed850(e, this);
    fn_006eef80(e, this);
    let result = fn_006ef920(e, this);
    fn_006f02e0(e, this);
    fn_006f03a0(e, this);
    fn_006f0450(e, this);
    fn_006f0d20(e, this);
    fn_006f10d0(e, this);
    fn_006f1440(e, this);
    if always_finish != 0 || e.call(ARRAY_IS_EMPTY, &args![bad_nodes]).u8() != 0 {
        fn_006f1c10(e, this, 1, 1);
        fn_006f1d30(e, this);
    }
    result
}

// Translated from 006ed730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ed730 - the compiler's exception frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, `RET 8`): makes a pathing location on the stack from
/// `point` and the mesh handle (`006dcf20`, 0x28 bytes), adds it to the solution
/// ([`fn_006ed7a0`]) and destroys it (`004ff7e0`).
pub fn fn_006ed730(e: &mut Engine, this: Ptr, point: u32, handle: u32) {
    e.with_stack(0x28, |e, location| {
        e.call(LOCATION_CONSTRUCT, &args![location, point, handle]);
        fn_006ed7a0(e, this, location.addr());
        e.call(LOCATION_DESTRUCT, &args![location]);
    });
}

// Translated from 006ed7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`): adds `location` to the solution
/// (`0069f920` on `pSolution` with the point at `011f426c`).
pub fn fn_006ed7a0(e: &mut Engine, this: Ptr, location: u32) {
    let solution = e.mem.u32(this.addr() + SOLUTION);
    e.call(SOLUTION_ADD, &args![solution, location, ZERO_POINT]);
}

// Translated from 006ed7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0x10`; called from [`fn_006f10d0`]): whether the
/// segment test `00696540` passes for every path edge from `first` to `last` (inclusive,
/// unsigned): each with the edge's `LeftShrunkLocation` and `RightShrunkLocation` (+0x20,
/// +0x2C) and the two given words. A failing edge answers 0.
pub fn fn_006ed7d0(e: &mut Engine, this: Ptr, first: u32, last: u32, a: u32, b: u32) -> u8 {
    let edges = this.addr() + PATH_EDGES;
    let mut index = first;
    while index <= last {
        let right_at = e.call(ARRAY_ELEMENT, &args![edges, index]).u32();
        let right = e.mem.u32(right_at).wrapping_add(0x2c);
        let left_at = e.call(ARRAY_ELEMENT, &args![edges, index]).u32();
        let left = e.mem.u32(left_at).wrapping_add(0x20);
        if e.call(SEGMENT_TEST_00696540, &args![left, right, a, b])
            .u8()
            == 0
        {
            return 0;
        }
        index = index.wrapping_add(1);
    }
    1
}

/// The two end points of edge `index` of the edge array (`0068f0c0` on its mesh with the
/// triangle word at +4 and the edge word at +8), into `first` and `second`.
fn search_edge_points(e: &mut Engine, this: Ptr, index: u32, first: u32, second: u32) {
    let edges = e.mem.u32(this.addr() + EDGES);
    let edge = e.call(EDGE_AT, &args![edges, index]).u32();
    let edge_word = e.mem.u32(edge.wrapping_add(8));
    let triangle = e.call(EDGE_AT, &args![edges, index]).u32();
    let triangle_word = u32::from(e.mem.u16(triangle.wrapping_add(4)));
    let mesh = e.call(EDGE_AT, &args![edges, index]).u32();
    let mesh = e.mem.u32(mesh);
    e.call(
        EDGE_POINTS,
        &args![mesh, triangle_word, edge_word, first, second],
    );
}

/// Whether the point at `point` equals the vertex at `vertex` (`004390c0` on `point`).
fn same_point(e: &mut Engine, point: u32, vertex: u32) -> bool {
    e.call(POINT_EQUAL, &args![point, vertex]).u8() != 0
}

// Translated from 006ed850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): builds the vertex and edge lists. Adds the
/// start vertex ([`fn_006edbc0`]); then, for each search edge, takes its two end points
/// (`0068f0c0`). The first edge starts the lists ([`fn_006edd30`]), its points ordered by
/// the sign of the third word of `004b3800(a - b, a - start)` (positive: `(b, a)`). Each
/// later edge is joined to the side whose last vertex one of its points equals: `a` the last
/// left vertex adds `b` on the left ([`fn_006ee390`]), `a` the last right one `b` on the
/// right ([`fn_006ee150`]), then the same for `b` adding `a`; when none matches, the
/// previous triangle's matching edge (`NavMesh::GetMatchingEdge_ov3`, defaults 0, 0xFFFF,
/// -1) gives two points ordered by whether the first equals the last right vertex, and the
/// same four tests run against them. Ends with [`fn_006ee5d0`].
pub fn fn_006ed850(e: &mut Engine, this: Ptr) {
    fn_006edbc0(e, this);
    // The frame from EBP - 0x80; `at(k)` is EBP - k.
    e.with_stack(0x80, |e, frame| {
        let at = |k: u32| frame.addr() + 0x80 - k;
        let (a, b) = (at(0x10), at(0x1c));
        let mut index = 0u32;
        loop {
            let edges = e.mem.u32(this.addr() + EDGES);
            if index >= e.call(ARRAY_COUNT, &args![edges]).u32() {
                break;
            }
            e.call(POINT_DEFAULT, &args![a]);
            e.call(POINT_DEFAULT, &args![b]);
            search_edge_points(e, this, index, a, b);
            if index == 0 {
                let start = this.addr() + START_LOCATION;
                let to_start = e.call(POINT_SUBTRACT, &args![a, at(0x7c), start]).u32();
                let across = e.call(POINT_SUBTRACT, &args![a, at(0x70), b]).u32();
                e.call(POINT_OP_004B3800, &args![across, at(0x28), to_start]);
                let side = x87_load32(e.mem.u32(at(0x20)));
                if side > e.global::<f64>(DOUBLE_ZERO) {
                    fn_006edd30(e, this, b, a);
                } else {
                    fn_006edd30(e, this, a, b);
                }
            } else {
                join_edge(e, this, frame.addr(), index);
            }
            index += 1;
        }
    });
    fn_006ee5d0(e, this);
}

/// The joining tests of [`fn_006ed850`] for edge `index` (its points at `EBP-0x10` and
/// `EBP-0x1C` of the frame at `frame`, which is `EBP-0x80`).
fn join_edge(e: &mut Engine, this: Ptr, frame: u32, index: u32) {
    let at = |k: u32| frame + 0x80 - k;
    let (a, b) = (at(0x10), at(0x1c));
    let last_left = last_of(e, this, LEFT_VERTS);
    if same_point(e, a, last_left) {
        fn_006ee390(e, this, b, index);
        return;
    }
    let last_right = last_of(e, this, RIGHT_VERTS);
    if same_point(e, a, last_right) {
        fn_006ee150(e, this, b, index);
        return;
    }
    let last_left = last_of(e, this, LEFT_VERTS);
    if same_point(e, b, last_left) {
        fn_006ee390(e, this, a, index);
        return;
    }
    let last_right = last_of(e, this, RIGHT_VERTS);
    if same_point(e, b, last_right) {
        fn_006ee150(e, this, a, index);
        return;
    }
    let (mesh, triangle, edge) = (at(0x64), at(0x38), at(0x48));
    e.mem.set_u32(mesh, 0);
    e.mem.set_u16(triangle, 0xffff);
    e.mem.set_u32(edge, u32::MAX);
    let previous = index.wrapping_sub(1);
    let edges = e.mem.u32(this.addr() + EDGES);
    let search_edge = e.call(EDGE_AT, &args![edges, previous]).u32();
    let edge_word = e.mem.u32(search_edge.wrapping_add(8));
    let triangles = e.mem.u32(this.addr() + TRIANGLES);
    let search_triangle = e.call(TRIANGLE_AT, &args![triangles, previous]).u32();
    let triangle_word = u32::from(e.mem.u16(search_triangle.wrapping_add(4)));
    let search_triangle = e.call(TRIANGLE_AT, &args![triangles, previous]).u32();
    let search_mesh = e.mem.u32(search_triangle);
    e.call(
        GET_MATCHING_EDGE,
        &args![search_mesh, triangle_word, edge_word, mesh, triangle, edge],
    );
    let (c, d) = (at(0x34), at(0x44));
    e.call(POINT_DEFAULT, &args![c]);
    e.call(POINT_DEFAULT, &args![d]);
    let matching_mesh = e.mem.u32(mesh);
    let matching_triangle = u32::from(e.mem.u16(triangle));
    let matching_edge = e.mem.u32(edge);
    e.call(
        EDGE_POINTS,
        &args![matching_mesh, matching_triangle, matching_edge, c, d],
    );
    let (left, right) = (at(0x60), at(0x54));
    e.call(POINT_DEFAULT, &args![left]);
    e.call(POINT_DEFAULT, &args![right]);
    let last_right = last_of(e, this, RIGHT_VERTS);
    if e.call(POINT_EQUAL, &args![last_right, c]).u8() != 0 {
        copy_point(e, left, d);
        copy_point(e, right, c);
    } else {
        copy_point(e, left, c);
        copy_point(e, right, d);
    }
    if same_point(e, a, left) {
        fn_006ee390(e, this, b, index);
    } else if same_point(e, a, right) {
        fn_006ee150(e, this, b, index);
    } else if same_point(e, b, left) {
        fn_006ee390(e, this, a, index);
    } else if same_point(e, b, right) {
        fn_006ee150(e, this, a, index);
    }
}

/// Sets a new vertex's `hNavMeshTri` from the search triangle or edge record `record`: the
/// mesh handle of its word at +0 (`0068efb0`) at +0x24 and its 16-bit +4 at +0x28.
fn set_vertex_triangle(e: &mut Engine, vertex: u32, record: u32) {
    let mesh = e.mem.u32(record);
    let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
    e.mem.set_u32(vertex.wrapping_add(0x24), handle);
    let triangle = e.mem.u16(record.wrapping_add(4));
    e.mem.set_u16(vertex.wrapping_add(0x28), triangle);
}

// Translated from 006edbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): the start vertex: a new vertex
/// ([`fn_006edc40`]) at `StartLocation`, on the first search triangle (`006b3180` on
/// `Triangles`), added to both `LeftVerts` and `RightVerts`.
pub fn fn_006edbc0(e: &mut Engine, this: Ptr) {
    let vertex = fn_006edc40(e, this);
    let triangles = e.mem.u32(this.addr() + TRIANGLES);
    let first = e.call(FIRST_TRIANGLE, &args![triangles]).u32();
    set_vertex_triangle(e, vertex, first);
    copy_point(e, vertex, this.addr() + START_LOCATION);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), vertex);
        add_word(e, this.addr() + LEFT_VERTS, slot.addr());
        add_word(e, this.addr() + RIGHT_VERTS, slot.addr());
    });
}

// Translated from 006edc40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006edc40 - the compiler's exception frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): a new `UniqueVert` (0x40 bytes from the
/// smoother's scrap heap, constructed by [`fn_006edce0`] with the heap when the placement
/// step answers it). Answers the block.
pub fn fn_006edc40(e: &mut Engine, this: Ptr) -> u32 {
    let (block, placed) = scrap_block(e, this, 0x40);
    if placed != 0 {
        let heap = e
            .call(POINT_DEFAULT, &args![this.addr() + SCRAP_HEAP])
            .u32();
        fn_006edce0(e, Ptr::new(placed), heap);
    }
    block
}

// Translated from 006edce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`): the `UniqueVert` constructor: the members at
/// +0, +0xC, +0x18 and +0x24 (`006815c0`) and the edge list at +0x2C (`006f2510(0, 0,
/// heap)`). Answers `this`.
pub fn fn_006edce0(e: &mut Engine, this: Ptr, heap: u32) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 0xc, 0x18, 0x24] {
        e.call(POINT_DEFAULT, &args![base + offset]);
    }
    e.call(SCRAP_ARRAY_CONSTRUCT, &args![base + 0x2c, 0u32, 0u32, heap]);
    this
}

/// Fills a new `BoundsEdge`: `pStartVert` the last vertex of the array at `verts`,
/// `pEndVert` `end`, the edge handle from the last search edge `edge` (mesh handle at +8,
/// the 16-bit +4 at +0xC), the index word -1, `iEdgeIndex` 0 and `bOpenEdge` 1.
fn fill_bounds_edge(e: &mut Engine, this: Ptr, bounds: u32, verts: u32, end: u32, edge: u32) {
    let start = last_of(e, this, verts);
    e.mem.set_u32(bounds, start);
    e.mem.set_u32(bounds.wrapping_add(4), end);
    let mesh = e.mem.u32(edge);
    let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
    e.mem.set_u32(bounds.wrapping_add(8), handle);
    let triangle = e.mem.u16(edge.wrapping_add(4));
    e.mem.set_u16(bounds.wrapping_add(0xc), triangle);
    e.mem.set_u32(bounds.wrapping_add(0x10), u32::MAX);
    e.mem.set_u32(bounds.wrapping_add(0x18), 0);
    e.mem.set_u8(bounds.wrapping_add(0x14), 1);
}

/// The normal of search triangle `index` (`006971e0` on its mesh) into `out`.
fn triangle_normal(e: &mut Engine, this: Ptr, index: u32, out: u32) {
    let triangles = e.mem.u32(this.addr() + TRIANGLES);
    let triangle = e.call(TRIANGLE_AT, &args![triangles, index]).u32();
    let triangle_word = u32::from(e.mem.u16(triangle.wrapping_add(4)));
    let triangle = e.call(TRIANGLE_AT, &args![triangles, index]).u32();
    let mesh = e.mem.u32(triangle);
    e.call(TRIANGLE_NORMAL, &args![mesh, out, triangle_word]);
}

// Translated from 006edd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): starts the lists with the first edge: new
/// left and right vertices at `left` and `right` on the last search edge (`006ced70`), a
/// left and a right `BoundsEdge` from the last left / right vertex to them, and a
/// `PathEdge` between them (its edge handle and word +0x44 from the search edge,
/// `iPathEdgeIndex` 0, the two bounds edges, `fTriangleNormalDelta` the dot product of the
/// normals of search triangles 0 and 1), added to both vertices' edge lists; then the
/// vertices, bounds edges and path edge are added to their arrays.
pub fn fn_006edd30(e: &mut Engine, this: Ptr, left: u32, right: u32) {
    let base = this.addr();
    // The frame from EBP - 0x34; `at(k)` is EBP - k.
    e.with_stack(0x34, |e, frame| {
        let at = |k: u32| frame.addr() + 0x34 - k;
        let left_vertex = fn_006edc40(e, this);
        e.mem.set_u32(at(0x30), left_vertex);
        let edges = e.mem.u32(base + EDGES);
        let edge = e.call(LAST_EDGE, &args![edges]).u32();
        set_vertex_triangle(e, left_vertex, edge);
        copy_point(e, left_vertex, left);
        let right_vertex = fn_006edc40(e, this);
        e.mem.set_u32(at(0x2c), right_vertex);
        set_vertex_triangle(e, right_vertex, edge);
        copy_point(e, right_vertex, right);
        let left_bounds = fn_006edfc0(e, this);
        e.mem.set_u32(at(0x10), left_bounds);
        fill_bounds_edge(e, this, left_bounds, LEFT_VERTS, left_vertex, edge);
        let right_bounds = fn_006edfc0(e, this);
        e.mem.set_u32(at(0x14), right_bounds);
        fill_bounds_edge(e, this, right_bounds, RIGHT_VERTS, right_vertex, edge);
        let path = fn_006ee070(e, this);
        e.mem.set_u32(at(0x1c), path);
        e.mem.set_u32(path, left_vertex);
        e.mem.set_u32(path.wrapping_add(4), right_vertex);
        let mesh = e.mem.u32(edge);
        let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
        e.mem.set_u32(path.wrapping_add(0x3c), handle);
        let triangle = e.mem.u16(edge.wrapping_add(4));
        e.mem.set_u16(path.wrapping_add(0x40), triangle);
        let word = e.mem.u32(edge.wrapping_add(8));
        e.mem.set_u32(path.wrapping_add(0x44), word);
        e.mem.set_u32(path.wrapping_add(0x48), 0);
        e.mem.set_u32(path.wrapping_add(0x4c), left_bounds);
        e.mem.set_u32(path.wrapping_add(0x50), right_bounds);
        add_word(e, left_vertex.wrapping_add(0x2c), at(0x1c));
        add_word(e, right_vertex.wrapping_add(0x2c), at(0x1c));
        triangle_normal(e, this, 0, at(0x28));
        triangle_normal(e, this, 1, at(0xc));
        let dot = e.call(POINT_DOT, &args![at(0x28), at(0xc)]).st0;
        e.mem.set_u32(path.wrapping_add(0x38), x87_store32(dot));
        add_word(e, base + LEFT_VERTS, at(0x30));
        add_word(e, base + RIGHT_VERTS, at(0x2c));
        add_word(e, base + LEFT_EDGES, at(0x10));
        add_word(e, base + RIGHT_EDGES, at(0x14));
        add_word(e, base + PATH_EDGES, at(0x1c));
    });
}

// Translated from 006edfc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006edfc0 - the compiler's exception frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): a new `BoundsEdge` (0x1C bytes from the
/// smoother's scrap heap, constructed by [`fn_006ee050`] when placed). Answers the block.
pub fn fn_006edfc0(e: &mut Engine, this: Ptr) -> u32 {
    let (block, placed) = scrap_block(e, this, 0x1c);
    if placed != 0 {
        fn_006ee050(e, Ptr::new(placed));
    }
    block
}

// Translated from 006ee050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`): the `BoundsEdge` constructor: the edge handle at +8
/// (`006240d0`). Answers `this`.
pub fn fn_006ee050(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(HANDLE_CONSTRUCT, &args![this.addr() + 8]);
    this
}

// Translated from 006ee070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ee070 - the compiler's exception frame (FS:[0]) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): a new `PathEdge` (0x58 bytes from the
/// smoother's scrap heap, constructed by [`fn_006ee100`] when placed). Answers the block.
pub fn fn_006ee070(e: &mut Engine, this: Ptr) -> u32 {
    let (block, placed) = scrap_block(e, this, 0x58);
    if placed != 0 {
        fn_006ee100(e, Ptr::new(placed));
    }
    block
}

// Translated from 006ee100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`): the `PathEdge` constructor: the points at +8, +0x14,
/// +0x20 and +0x2C (`006815c0`) and the edge handle at +0x3C (`006240d0`). Answers `this`.
pub fn fn_006ee100(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [8u32, 0x14, 0x20, 0x2c] {
        e.call(POINT_DEFAULT, &args![base + offset]);
    }
    e.call(HANDLE_CONSTRUCT, &args![base + 0x3c]);
    this
}

/// The cell-side helpers of a new side vertex's bounds edge: `005582f0` (thiscall on the
/// edge's mesh: triangle; in `tesobjectcell.cpp` by the engine map) and `0068f200`
/// (thiscall on what it answers: the edge index; `AL`), and the triangle array's last
/// element (`006b31a0`, thiscall).
const MESH_TRIANGLE_005582F0: u32 = 0x0055_82f0;
const EDGE_IS_OPEN_0068F200: u32 = 0x0068_f200;
const LAST_TRIANGLE: u32 = 0x006b_31a0;

/// [`fn_006ee150`] (`left`) and [`fn_006ee390`]: a new vertex at `point` on search edge
/// `index`, a bounds edge from that side's last vertex to it, and a path edge across.
fn add_side_vertex(e: &mut Engine, this: Ptr, point: u32, index: u32, left: bool) {
    let base = this.addr();
    let (verts, other_verts, edges_of_side, other_edges, step) = if left {
        (LEFT_VERTS, RIGHT_VERTS, LEFT_EDGES, RIGHT_EDGES, 1)
    } else {
        (RIGHT_VERTS, LEFT_VERTS, RIGHT_EDGES, LEFT_EDGES, 2)
    };
    // The frame from EBP - 0x2C; `at(k)` is EBP - k.
    e.with_stack(0x2c, |e, frame| {
        let at = |k: u32| frame.addr() + 0x2c - k;
        let vertex = fn_006edc40(e, this);
        e.mem.set_u32(at(0x28), vertex);
        let edges = e.mem.u32(base + EDGES);
        let edge = e.call(EDGE_AT, &args![edges, index]).u32();
        set_vertex_triangle(e, vertex, edge);
        copy_point(e, vertex, point);
        let bounds = fn_006edfc0(e, this);
        e.mem.set_u32(at(0x24), bounds);
        let start = last_of(e, this, verts);
        e.mem.set_u32(bounds, start);
        e.mem.set_u32(bounds.wrapping_add(4), vertex);
        let mesh = e.mem.u32(edge);
        let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
        e.mem.set_u32(bounds.wrapping_add(8), handle);
        let triangle = e.mem.u16(edge.wrapping_add(4));
        e.mem.set_u16(bounds.wrapping_add(0xc), triangle);
        let word = e.mem.u32(edge.wrapping_add(8)) as i32;
        let next = word.wrapping_add(step).wrapping_rem(3);
        e.mem.set_u32(bounds.wrapping_add(0x10), next as u32);
        let edge_index = e.mem.u32(bounds.wrapping_add(0x10));
        let triangle = u32::from(e.mem.u16(edge.wrapping_add(4)));
        let mesh = e.mem.u32(edge);
        let cell = e.call(MESH_TRIANGLE_005582F0, &args![mesh, triangle]).u32();
        let open = e.call(EDGE_IS_OPEN_0068F200, &args![cell, edge_index]).u8();
        e.mem.set_u8(bounds.wrapping_add(0x14), open);
        let count = e.call(ARRAY_COUNT, &args![base + edges_of_side]).u32();
        e.mem.set_u32(bounds.wrapping_add(0x18), count);
        let path = fn_006ee070(e, this);
        e.mem.set_u32(at(0x14), path);
        if left {
            e.mem.set_u32(path, vertex);
            let other = last_of(e, this, other_verts);
            e.mem.set_u32(path.wrapping_add(4), other);
        } else {
            let other = last_of(e, this, other_verts);
            e.mem.set_u32(path, other);
            e.mem.set_u32(path.wrapping_add(4), vertex);
        }
        let mesh = e.mem.u32(edge);
        let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
        e.mem.set_u32(path.wrapping_add(0x3c), handle);
        let triangle = e.mem.u16(edge.wrapping_add(4));
        e.mem.set_u16(path.wrapping_add(0x40), triangle);
        let word = e.mem.u32(edge.wrapping_add(8));
        e.mem.set_u32(path.wrapping_add(0x44), word);
        let count = e.call(ARRAY_COUNT, &args![base + PATH_EDGES]).u32();
        e.mem.set_u32(path.wrapping_add(0x48), count);
        if left {
            e.mem.set_u32(path.wrapping_add(0x4c), bounds);
            let other = last_of(e, this, other_edges);
            e.mem.set_u32(path.wrapping_add(0x50), other);
        } else {
            let other = last_of(e, this, other_edges);
            e.mem.set_u32(path.wrapping_add(0x4c), other);
            e.mem.set_u32(path.wrapping_add(0x50), bounds);
        }
        add_word(e, vertex.wrapping_add(0x2c), at(0x14));
        let other = last_of(e, this, other_verts);
        add_word(e, other.wrapping_add(0x2c), at(0x14));
        triangle_normal(e, this, index, at(0x20));
        triangle_normal(e, this, index.wrapping_add(1), at(0xc));
        let dot = e.call(POINT_DOT, &args![at(0x20), at(0xc)]).st0;
        e.mem.set_u32(path.wrapping_add(0x38), x87_store32(dot));
        add_word(e, base + verts, at(0x28));
        add_word(e, base + edges_of_side, at(0x24));
        add_word(e, base + PATH_EDGES, at(0x14));
    });
}

// Translated from 006ee150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): adds `point` as a new left vertex on search
/// edge `index`: the vertex ([`fn_006edc40`], on the edge's triangle), a `BoundsEdge` from
/// the last left vertex to it (its edge handle from the search edge, `iEdgeIndex`-like word
/// +0x10 `(edge word + 1) % 3` (signed), `bOpenEdge` from `0068f200` on `005582f0`'s
/// answer, +0x18 the left-edge count) and a `PathEdge` from it to the last right vertex
/// (edge handle and +0x44 from the search edge, `iPathEdgeIndex` the path-edge count, the
/// new bounds edge and the last right one, the dot product of the normals of triangles
/// `index` and `index + 1`), added to both vertices' edge lists; then the vertex, bounds
/// edge and path edge are added to `LeftVerts`, `LeftEdges` and `PathEdges`.
pub fn fn_006ee150(e: &mut Engine, this: Ptr, point: u32, index: u32) {
    add_side_vertex(e, this, point, index, true);
}

// Translated from 006ee390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): the right-side twin of [`fn_006ee150`]: the
/// bounds edge from the last right vertex (word +0x10 `(edge word + 2) % 3`, +0x18 the
/// right-edge count), the path edge from the last left vertex to the new one (its previous
/// left edge the last left bounds edge, its previous right edge the new one), and the adds
/// to `RightVerts`, `RightEdges` and `PathEdges`.
pub fn fn_006ee390(e: &mut Engine, this: Ptr, point: u32, index: u32) {
    add_side_vertex(e, this, point, index, false);
}

/// Fills a closing `BoundsEdge` of [`fn_006ee5d0`]: from the last vertex of `verts` to
/// `vertex`, open, the edge handle from the last search triangle `triangle`, the word +0x10
/// -1 and +0x18 the count of `edges_of_side`.
fn fill_closing_edge(
    e: &mut Engine,
    this: Ptr,
    bounds: u32,
    verts: u32,
    edges_of_side: u32,
    vertex: u32,
    triangle: u32,
) {
    let start = last_of(e, this, verts);
    e.mem.set_u32(bounds, start);
    e.mem.set_u32(bounds.wrapping_add(4), vertex);
    e.mem.set_u8(bounds.wrapping_add(0x14), 1);
    let mesh = e.mem.u32(triangle);
    let handle = e.call(NAV_MESH_HANDLE, &args![mesh]).u32();
    e.mem.set_u32(bounds.wrapping_add(8), handle);
    let word = e.mem.u16(triangle.wrapping_add(4));
    e.mem.set_u16(bounds.wrapping_add(0xc), word);
    e.mem.set_u32(bounds.wrapping_add(0x10), u32::MAX);
    let count = e
        .call(ARRAY_COUNT, &args![this.addr() + edges_of_side])
        .u32();
    e.mem.set_u32(bounds.wrapping_add(0x18), count);
}

// Translated from 006ee5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): the goal vertex: a new vertex at
/// `GoalLocation` on the last search triangle (`006b31a0`), a left and a right
/// `BoundsEdge` closing each side on it (see [`fill_closing_edge`]); the vertex is added to
/// both vertex arrays and the bounds edges to `LeftEdges` and `RightEdges`.
pub fn fn_006ee5d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The frame from EBP - 0x14; `at(k)` is EBP - k.
    e.with_stack(0x14, |e, frame| {
        let at = |k: u32| frame.addr() + 0x14 - k;
        let vertex = fn_006edc40(e, this);
        e.mem.set_u32(at(0x4), vertex);
        let triangles = e.mem.u32(base + TRIANGLES);
        let triangle = e.call(LAST_TRIANGLE, &args![triangles]).u32();
        set_vertex_triangle(e, vertex, triangle);
        copy_point(e, vertex, base + GOAL_LOCATION);
        let left = fn_006edfc0(e, this);
        e.mem.set_u32(at(0xc), left);
        fill_closing_edge(e, this, left, LEFT_VERTS, LEFT_EDGES, vertex, triangle);
        let right = fn_006edfc0(e, this);
        e.mem.set_u32(at(0x8), right);
        fill_closing_edge(e, this, right, RIGHT_VERTS, RIGHT_EDGES, vertex, triangle);
        add_word(e, base + LEFT_VERTS, at(0x4));
        add_word(e, base + RIGHT_VERTS, at(0x4));
        add_word(e, base + LEFT_EDGES, at(0xc));
        add_word(e, base + RIGHT_EDGES, at(0x8));
    });
}

/// The vertex split's helpers: the handle copy `0069a690` (thiscall: other), the edge
/// handle assignment `006bc870` (thiscall: other), an array's first element address
/// (`006df160`, thiscall), its insert (`006f23d0`, thiscall: index, the element's address)
/// and its resize (`006f2290`, the map's `BSSimpleArray<TESObjectWEAP*,1024>::SetSize`
/// body; thiscall: size, 1).
const HANDLE_COPY: u32 = 0x0069_a690;
const HANDLE_ASSIGN: u32 = 0x006b_c870;
const ARRAY_FIRST: u32 = 0x006d_f160;
const ARRAY_INSERT: u32 = 0x006f_23d0;
const ARRAY_SET_SIZE: u32 = 0x006f_2290;

/// The word at the address the array at `array` answers for its first element.
fn first_of(e: &mut Engine, array: u32) -> u32 {
    let at = e.call(ARRAY_FIRST, &args![array]).u32();
    e.mem.u32(at)
}

/// The word at the address the array at `array` answers for its last element.
fn last_in(e: &mut Engine, array: u32) -> u32 {
    let at = fn_006f2270(e, Ptr::new(array));
    e.mem.u32(at)
}

/// Inserts `value` at `index` of the array at `array` (`006f23d0`, through the address of
/// a word holding it).
fn insert_value(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(ARRAY_INSERT, &args![array, index, slot]);
    });
}

// Translated from 006ee740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0x10`): splits vertex `index` of the left (`left`
/// set) or right vertices in two. The new vertex ([`fn_006edc40`]) takes the old one's
/// location and triangle handle (`0069a690`); the old one's `ShrunkLocation` becomes
/// `*old_shrunk` and the new one's `*new_shrunk`, each `PushDelta` the shrunk location minus
/// the location. With one path edge on the old vertex the new one gets it too; otherwise the
/// second half of the old vertex's path edges (from `count / 2`) move to the new vertex
/// (their left or right vertex set to it, added to its list) and the old list keeps the
/// first half (`006f2290`). The new vertex is inserted after the old one (`006f23d0`), a new
/// `BoundsEdge` from the old to the new vertex is inserted ([`fn_006eec00`]) with the
/// handle and open flag of the side's bounds edge `index`, whose successor now starts at the
/// new vertex. A new `PathEdge` between the two halves is chosen by the segment test
/// `00696540` (the old shrunk location against the far end of the new vertex's first path
/// edge, with the new location and shrunk location), takes the handle of the path edge the
/// new vertex's first path edge indexes and `fTriangleNormalDelta` 1.0, and is inserted at
/// that index ([`fn_006eecc0`]). Answers the new vertex.
pub fn fn_006ee740(
    e: &mut Engine,
    this: Ptr,
    index: u32,
    left: u8,
    old_shrunk: Ptr,
    new_shrunk: Ptr,
) -> u32 {
    let base = this.addr();
    let (verts, edges_of_side) = if left != 0 {
        (LEFT_VERTS, LEFT_EDGES)
    } else {
        (RIGHT_VERTS, RIGHT_EDGES)
    };
    // The frame from EBP - 0x40; `at(k)` is EBP - k.
    e.with_stack(0x40, |e, frame| {
        let at = |k: u32| frame.addr() + 0x40 - k;
        let old_at = e.call(ARRAY_ELEMENT, &args![base + verts, index]).u32();
        let old = e.mem.u32(old_at);
        let new = fn_006edc40(e, this);
        e.mem.set_u32(at(0x10), new);
        copy_point(e, new, old);
        e.call(
            HANDLE_COPY,
            &args![new.wrapping_add(0x24), old.wrapping_add(0x24)],
        );
        let old_list = old.wrapping_add(0x2c);
        let new_list = new.wrapping_add(0x2c);
        let count = e.call(ARRAY_COUNT, &args![old_list]).u32();
        copy_point(e, old.wrapping_add(0xc), old_shrunk.addr());
        let delta = e
            .call(POINT_SUBTRACT, &args![old.wrapping_add(0xc), at(0x2c), old])
            .u32();
        copy_point(e, old.wrapping_add(0x18), delta);
        copy_point(e, new.wrapping_add(0xc), new_shrunk.addr());
        let delta = e
            .call(POINT_SUBTRACT, &args![new.wrapping_add(0xc), at(0x38), new])
            .u32();
        copy_point(e, new.wrapping_add(0x18), delta);
        if count == 1 {
            let edge = e.call(ARRAY_ELEMENT, &args![old_list, 0u32]).u32();
            add_word(e, new_list, edge);
        } else {
            let half = count >> 1;
            let mut moving = half;
            while moving < count {
                let edge_at = e.call(ARRAY_ELEMENT, &args![old_list, moving]).u32();
                let edge = e.mem.u32(edge_at);
                if left != 0 {
                    e.mem.set_u32(edge, new);
                } else {
                    e.mem.set_u32(edge.wrapping_add(4), new);
                }
                let edge_at = e.call(ARRAY_ELEMENT, &args![old_list, moving]).u32();
                add_word(e, new_list, edge_at);
                moving += 1;
            }
            e.call(ARRAY_SET_SIZE, &args![old_list, half, 1u32]);
        }
        e.call(
            ARRAY_INSERT,
            &args![base + verts, index.wrapping_add(1), at(0x10)],
        );
        let bounds = fn_006edfc0(e, this);
        e.mem.set_u32(bounds, old);
        e.mem.set_u32(bounds.wrapping_add(4), new);
        fn_006eec00(e, this, index, bounds, left);
        let side_at = e
            .call(ARRAY_ELEMENT, &args![base + edges_of_side, index])
            .u32();
        let side = e.mem.u32(side_at);
        e.call(
            HANDLE_ASSIGN,
            &args![bounds.wrapping_add(8), side.wrapping_add(8)],
        );
        let side_at = e
            .call(ARRAY_ELEMENT, &args![base + edges_of_side, index])
            .u32();
        let side = e.mem.u32(side_at);
        let open = e.mem.u8(side.wrapping_add(0x14));
        e.mem.set_u8(bounds.wrapping_add(0x14), open);
        let next_at = e
            .call(
                ARRAY_ELEMENT,
                &args![base + edges_of_side, index.wrapping_add(1)],
            )
            .u32();
        let next = e.mem.u32(next_at);
        e.mem.set_u32(next, new);
        let path = fn_006ee070(e, this);
        let first = first_of(e, new_list);
        let far = if left != 0 {
            e.mem.u32(first.wrapping_add(4))
        } else {
            e.mem.u32(first)
        };
        let separated = e
            .call(
                SEGMENT_TEST_00696540,
                &args![
                    old.wrapping_add(0xc),
                    far.wrapping_add(0xc),
                    new,
                    new.wrapping_add(0xc)
                ],
            )
            .u8()
            != 0;
        match (left != 0, separated) {
            (true, true) => {
                e.mem.set_u32(path, new);
                let last = last_in(e, old_list);
                let right = e.mem.u32(last.wrapping_add(4));
                e.mem.set_u32(path.wrapping_add(4), right);
                e.mem.set_u32(path.wrapping_add(0x4c), bounds);
                let last = last_in(e, old_list);
                let previous_right = e.mem.u32(last.wrapping_add(0x50));
                e.mem.set_u32(path.wrapping_add(0x50), previous_right);
            }
            (true, false) => {
                e.mem.set_u32(path, old);
                let first = first_of(e, new_list);
                let right = e.mem.u32(first.wrapping_add(4));
                e.mem.set_u32(path.wrapping_add(4), right);
                let last = last_in(e, old_list);
                let previous_left = e.mem.u32(last.wrapping_add(0x4c));
                e.mem.set_u32(path.wrapping_add(0x4c), previous_left);
                let first = first_of(e, new_list);
                let previous_right = e.mem.u32(first.wrapping_add(0x50));
                e.mem.set_u32(path.wrapping_add(0x50), previous_right);
            }
            (false, true) => {
                let last = last_in(e, old_list);
                let left_vertex = e.mem.u32(last);
                e.mem.set_u32(path, left_vertex);
                e.mem.set_u32(path.wrapping_add(4), new);
                let last = last_in(e, old_list);
                let previous_left = e.mem.u32(last.wrapping_add(0x4c));
                e.mem.set_u32(path.wrapping_add(0x4c), previous_left);
                e.mem.set_u32(path.wrapping_add(0x50), bounds);
            }
            (false, false) => {
                let first = first_of(e, new_list);
                let left_vertex = e.mem.u32(first);
                e.mem.set_u32(path, left_vertex);
                e.mem.set_u32(path.wrapping_add(4), old);
                let first = first_of(e, new_list);
                let previous_left = e.mem.u32(first.wrapping_add(0x4c));
                e.mem.set_u32(path.wrapping_add(0x4c), previous_left);
                let last = last_in(e, old_list);
                let previous_right = e.mem.u32(last.wrapping_add(0x50));
                e.mem.set_u32(path.wrapping_add(0x50), previous_right);
            }
        }
        let first = first_of(e, new_list);
        let path_index = e.mem.u32(first.wrapping_add(0x48));
        let indexed_at = e
            .call(ARRAY_ELEMENT, &args![base + PATH_EDGES, path_index])
            .u32();
        let indexed = e.mem.u32(indexed_at);
        e.call(
            HANDLE_ASSIGN,
            &args![path.wrapping_add(0x3c), indexed.wrapping_add(0x3c)],
        );
        e.mem.set_u32(path.wrapping_add(0x38), 0x3f80_0000);
        fn_006eecc0(e, this, path_index, path);
        new
    })
}

// Translated from 006eec00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0xC`): inserts `bounds` at `index` of `LeftEdges`
/// (`left` set) or `RightEdges` (`006f23d0`, through the address of its argument word) and
/// renumbers that array's `iEdgeIndex` (+0x18) from `index` on.
pub fn fn_006eec00(e: &mut Engine, this: Ptr, index: u32, bounds: u32, left: u8) {
    let array = this.addr() + if left != 0 { LEFT_EDGES } else { RIGHT_EDGES };
    insert_value(e, array, index, bounds);
    let mut at = index;
    while at < e.call(ARRAY_COUNT, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT, &args![array, at]).u32();
        let edge = e.mem.u32(element);
        e.mem.set_u32(edge.wrapping_add(0x18), at);
        at += 1;
    }
}

// Translated from 006eecc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): inserts `path` at `index` of `PathEdges`
/// (`006f23d0`, through the address of its argument word) and renumbers the path edges'
/// `iPathEdgeIndex` (+0x48) from `index` on.
pub fn fn_006eecc0(e: &mut Engine, this: Ptr, index: u32, path: u32) {
    let array = this.addr() + PATH_EDGES;
    insert_value(e, array, index, path);
    let mut at = index;
    while at < e.call(ARRAY_COUNT, &args![array]).u32() {
        let element = e.call(ARRAY_ELEMENT, &args![array, at]).u32();
        let edge = e.mem.u32(element);
        e.mem.set_u32(edge.wrapping_add(0x48), at);
        at += 1;
    }
}

/// The address of element `index` of a bounds-edge array (`006a7ad0`, thiscall: index).
const BOUNDS_EDGE_AT: u32 = 0x006a_7ad0;

/// The bounds edge `index` of the array at `array` (the word at the address `006a7ad0`
/// answers).
fn bounds_edge(e: &mut Engine, array: u32, index: u32) -> u32 {
    let at = e.call(BOUNDS_EDGE_AT, &args![array, index]).u32();
    e.mem.u32(at)
}

// Translated from 006eed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): splits bounds edge `index` of the left (`left`
/// set) or right edges at a copy of its start vertex. The path edge it takes after is the
/// first one of the end vertex when the start vertex has none (`0076b610`), else the start
/// vertex's last one. The copy ([`fn_006edc40`]: location and triangle handle) is inserted
/// after vertex `index`; a new `BoundsEdge` from the start vertex to the copy (with the
/// split edge's handle and open flag) is inserted at `index` ([`fn_006eec00`]) and the split
/// edge now starts at the copy. A new `PathEdge` (`fTriangleNormalDelta` 1.0, the chosen path
/// edge's handle) joins the copy to the chosen edge's far vertex, with the new bounds edge
/// on the copy's side and the chosen edge's on the other, and is inserted at the chosen
/// edge's index (from the end vertex) or after it (from the start vertex)
/// ([`fn_006eecc0`]). Answers the copy.
pub fn fn_006eed30(e: &mut Engine, this: Ptr, index: u32, left: u8) -> u32 {
    let base = this.addr();
    let (array, verts) = if left != 0 {
        (base + LEFT_EDGES, base + LEFT_VERTS)
    } else {
        (base + RIGHT_EDGES, base + RIGHT_VERTS)
    };
    let split = bounds_edge(e, array, index);
    let start = e.mem.u32(split);
    let end = e.mem.u32(split.wrapping_add(4));
    let start_list = start.wrapping_add(0x2c);
    let end_list = end.wrapping_add(0x2c);
    let chosen = if e.call(ARRAY_IS_EMPTY, &args![start_list]).u8() != 0 {
        first_of(e, end_list)
    } else {
        last_in(e, start_list)
    };
    // The frame from EBP - 0x2C; `at(k)` is EBP - k.
    e.with_stack(0x2c, |e, frame| {
        let at = |k: u32| frame.addr() + 0x2c - k;
        let copy = fn_006edc40(e, this);
        e.mem.set_u32(at(0x14), copy);
        copy_point(e, copy, start);
        e.call(
            HANDLE_COPY,
            &args![copy.wrapping_add(0x24), start.wrapping_add(0x24)],
        );
        e.call(ARRAY_INSERT, &args![verts, index.wrapping_add(1), at(0x14)]);
        let bounds = fn_006edfc0(e, this);
        e.mem.set_u32(bounds, start);
        e.mem.set_u32(bounds.wrapping_add(4), copy);
        let split = bounds_edge(e, array, index);
        e.call(
            HANDLE_ASSIGN,
            &args![bounds.wrapping_add(8), split.wrapping_add(8)],
        );
        let split = bounds_edge(e, array, index);
        let open = e.mem.u8(split.wrapping_add(0x14));
        e.mem.set_u8(bounds.wrapping_add(0x14), open);
        let split = bounds_edge(e, array, index);
        e.mem.set_u32(split, copy);
        fn_006eec00(e, this, index, bounds, left);
        let path = fn_006ee070(e, this);
        e.mem.set_u32(path.wrapping_add(0x38), 0x3f80_0000);
        e.call(
            HANDLE_ASSIGN,
            &args![path.wrapping_add(0x3c), chosen.wrapping_add(0x3c)],
        );
        if left != 0 {
            e.mem.set_u32(path, copy);
            let right = e.mem.u32(chosen.wrapping_add(4));
            e.mem.set_u32(path.wrapping_add(4), right);
            let previous_right = e.mem.u32(chosen.wrapping_add(0x50));
            e.mem.set_u32(path.wrapping_add(0x50), previous_right);
            e.mem.set_u32(path.wrapping_add(0x4c), bounds);
        } else {
            let left_vertex = e.mem.u32(chosen);
            e.mem.set_u32(path, left_vertex);
            let previous_left = e.mem.u32(chosen.wrapping_add(0x4c));
            e.mem.set_u32(path.wrapping_add(0x4c), previous_left);
            e.mem.set_u32(path.wrapping_add(4), copy);
            e.mem.set_u32(path.wrapping_add(0x50), bounds);
        }
        if e.call(ARRAY_IS_EMPTY, &args![start_list]).u8() != 0 {
            let first = first_of(e, end_list);
            let path_index = e.mem.u32(first.wrapping_add(0x48));
            fn_006eecc0(e, this, path_index, path);
        } else {
            let last = last_in(e, start_list);
            let path_index = e.mem.u32(last.wrapping_add(0x48)).wrapping_add(1);
            fn_006eecc0(e, this, path_index, path);
        }
        copy
    })
}

// Translated from 006eef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): shrinks the vertices. The first left and
/// right vertices keep their locations as shrunk locations; every inner left vertex, then
/// every inner right one, goes through [`fn_006ef0c0`] with `fSmoothingRadius` (which
/// answers the next index to look at); the last left and right vertices keep their
/// locations too.
pub fn fn_006eef80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for verts in [LEFT_VERTS, RIGHT_VERTS] {
        let at = e.call(ARRAY_FIRST, &args![base + verts]).u32();
        let source = e.mem.u32(at);
        let at = e.call(ARRAY_FIRST, &args![base + verts]).u32();
        let vertex = e.mem.u32(at);
        copy_point(e, vertex.wrapping_add(0xc), source);
    }
    for (verts, left) in [(LEFT_VERTS, 1u8), (RIGHT_VERTS, 0)] {
        let mut index = 1u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![base + verts]).u32();
            if index >= count.wrapping_sub(1) {
                break;
            }
            let radius = x87_quiet_bits(e.mem.u32(base + SMOOTHING_RADIUS));
            index = fn_006ef0c0(e, this, index, f32::from_bits(radius), left);
        }
    }
    for verts in [LEFT_VERTS, RIGHT_VERTS] {
        let source = last_of(e, this, verts);
        let vertex = last_of(e, this, verts);
        copy_point(e, vertex.wrapping_add(0xc), source);
    }
}

/// The point helpers of the vertex shrink (thiscall on a point unless noted): the
/// normalisation `004a0c10`, the addition `00439e90` (out, other; answers out), the
/// scaling `0045bb20` (out, float; answers out), the negation `004a0bd0` (out; answers
/// out), the cdecl scaling `004a3760` (out, float, point; answers out), and the cdecl
/// float functions `004b5570` (two floats; ST0) and `0057dd70` (one float; ST0).
const POINT_NORMALIZE: u32 = 0x004a_0c10;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_SCALE: u32 = 0x0045_bb20;
const POINT_NEGATE: u32 = 0x004a_0bd0;
const SCALE_POINT: u32 = 0x004a_3760;
const ANGLE_004B5570: u32 = 0x004b_5570;
const CURVE_0057DD70: u32 = 0x0057_dd70;
/// The constants of [`fn_006ef0c0`], read at run time: the point the side directions are
/// combined with (`011a9484`), the `double`s at `0101ff40` and `01011588` (the angle's
/// offset and factor), `010217d8` (the straightness limit) and `01016ff0` (the radius
/// factor), and the `float`s at `01016248` (the midpoint factor) and `01013ea4` (the
/// spread factor).
const SIDE_AXIS: u32 = 0x011a_9484;
const ANGLE_OFFSET: u32 = 0x0101_ff40;
const ANGLE_FACTOR: u32 = 0x0101_1588;
const STRAIGHT_LIMIT: u32 = 0x0102_17d8;
const RADIUS_FACTOR: u32 = 0x0101_6ff0;
const MIDPOINT_FACTOR: u32 = 0x0101_6248;
const SPREAD_FACTOR: u32 = 0x0101_3ea4;

/// The record at index `index` of the array at `array` (`006a7ad0`).
fn record(e: &mut Engine, array: u32, index: u32) -> u32 {
    bounds_edge(e, array, index)
}

/// `vertex[from] - vertex[at]` of the vertex array `verts` into `out`, then copied to
/// `to`.
fn vertex_difference(e: &mut Engine, verts: u32, at: u32, from: u32, out: u32, to: u32) {
    let center = record(e, verts, at);
    let other = record(e, verts, from);
    let difference = e.call(POINT_SUBTRACT, &args![other, out, center]).u32();
    copy_point(e, to, difference);
}

// Translated from 006ef0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 0xC`; called from [`fn_006eef80`]): shrinks vertex
/// `index` of the left (`left` set) or right vertices by `radius`. The directions to the
/// previous and next vertex (from the one before or after those when they coincide; their
/// third words zeroed) are normalised (`004a0c10`), combined with the point at `011a9484`
/// (`004b3800`) into side normals, and their dot products give an angle (`004b5570`) whose
/// `(offset - angle) * factor` (`0101ff40`, `01011588`) feeds `0057dd70`, times `radius`.
/// When the directions are nearly opposite (dot not below `010217d8`) and both side normals
/// face the same way along the vertex's first path edge, the vertex is split
/// ([`fn_006ee740`]) at two shrunk points built from `radius * 01016ff0` along the
/// directions and `radius` along the normals, and the next index is `index + 2`.
/// Otherwise the vertex's `PushDelta` is the normal times `radius` plus (left) or the
/// negated direction times the offset minus (right); its `ShrunkLocation` comes from
/// [`fn_006f0050`] when both adjacent bounds edges are open, else location plus push
/// delta; when that succeeded and the segment test `00696540` finds the previous and this
/// vertex crossing, both shrunk locations are moved to their midpoint (`01016248`) plus and
/// minus their difference times `01013ea4`. Answers `index + 1`.
pub fn fn_006ef0c0(e: &mut Engine, this: Ptr, index: u32, radius: f32, left: u8) -> u32 {
    let base = this.addr();
    let (verts, edges_of_side) = if left != 0 {
        (base + LEFT_VERTS, base + LEFT_EDGES)
    } else {
        (base + RIGHT_VERTS, base + RIGHT_EDGES)
    };
    let radius = x87_quiet_bits(radius.to_bits());
    // The frame from EBP - 0x260; `at(k)` is EBP - k.
    e.with_stack(0x260, |e, frame| {
        let at = |k: u32| frame.addr() + 0x260 - k;
        let (before, after) = (at(0x18), at(0x24));
        let (after_normal, before_normal) = (at(0x34), at(0x4c));
        e.call(POINT_DEFAULT, &args![before]);
        e.call(POINT_DEFAULT, &args![after]);
        let previous = index.wrapping_sub(1);
        vertex_difference(e, verts, index, previous, at(0x98), before);
        vertex_difference(e, verts, index, index.wrapping_add(1), at(0xa4), after);
        if e.call(POINT_EQUAL, &args![before, ZERO_POINT]).u8() != 0 && index > 1 {
            vertex_difference(e, verts, index, index.wrapping_sub(2), at(0xb0), before);
        }
        if e.call(POINT_EQUAL, &args![after, ZERO_POINT]).u8() != 0 {
            let count = e.call(ARRAY_COUNT, &args![verts]).u32();
            if index < count.wrapping_sub(2) {
                vertex_difference(e, verts, index, index.wrapping_add(2), at(0xbc), after);
            }
        }
        e.mem.set_u32(before + 8, 0);
        e.mem.set_u32(after + 8, 0);
        e.call(POINT_NORMALIZE, &args![before]);
        e.call(POINT_NORMALIZE, &args![after]);
        e.call(POINT_OP_004B3800, &args![after, after_normal, SIDE_AXIS]);
        e.call(POINT_OP_004B3800, &args![before, before_normal, SIDE_AXIS]);
        let across = x87_store32(e.call(POINT_DOT, &args![before, after_normal]).st0);
        e.mem.set_u32(at(0xc), across);
        let along = x87_store32(e.call(POINT_DOT, &args![before, after]).st0);
        e.mem.set_u32(at(0x8), along);
        let angle = e
            .call(
                ANGLE_004B5570,
                &args![x87_quiet_bits(across), x87_quiet_bits(along)],
            )
            .st0;
        let offset_angle = x87_sub(e.global::<f64>(ANGLE_OFFSET), angle);
        let half = x87_store32(x87_mul(offset_angle, e.global::<f64>(ANGLE_FACTOR)));
        e.mem.set_u32(at(0x4), half);
        let shaped = e.call(CURVE_0057DD70, &args![x87_quiet_bits(half)]).st0;
        let offset = x87_store32(x87_mul(shaped, x87_m32(radius)));
        e.mem.set_u32(at(0x40), offset);
        let vertex = record(e, verts, index);
        let edge_direction = at(0x58);
        e.call(POINT_DEFAULT, &args![edge_direction]);
        let list = vertex.wrapping_add(0x2c);
        let first = e.call(ARRAY_ELEMENT, &args![list, 0u32]).u32();
        let first = e.mem.u32(first);
        if left != 0 {
            let from = e.mem.u32(first);
            let edge = e.call(ARRAY_ELEMENT, &args![list, 0u32]).u32();
            let edge = e.mem.u32(edge);
            let to = e.mem.u32(edge.wrapping_add(4));
            let direction = e.call(POINT_SUBTRACT, &args![to, at(0xc8), from]).u32();
            copy_point(e, edge_direction, direction);
        } else {
            let from = e.mem.u32(first.wrapping_add(4));
            let edge = e.call(ARRAY_ELEMENT, &args![list, 0u32]).u32();
            let edge = e.mem.u32(edge);
            let to = e.mem.u32(edge);
            let direction = e.call(POINT_SUBTRACT, &args![to, at(0xd4), from]).u32();
            copy_point(e, edge_direction, direction);
        }
        if x87_load32(along) >= e.global::<f64>(STRAIGHT_LIMIT) {
            let first_side = e.call(POINT_DOT, &args![before_normal, edge_direction]).st0;
            let second_side = e.call(POINT_DOT, &args![after_normal, edge_direction]).st0;
            if x87_mul(second_side, first_side) > e.global::<f64>(DOUBLE_ZERO) {
                let (old_point, new_point) = (at(0x64), at(0x70));
                e.call(POINT_DEFAULT, &args![old_point]);
                e.call(POINT_DEFAULT, &args![new_point]);
                let factor = e.global::<f64>(RADIUS_FACTOR);
                let scaled = |e: &mut Engine, out: u32, by: u32, point: u32| -> u32 {
                    e.call(SCALE_POINT, &args![out, by, point]).u32()
                };
                // Each `radius * factor` is stored to a float before it is pushed.
                let far = || x87_quiet_bits(x87_store32(x87_mul(x87_load32(radius), factor)));
                if left != 0 {
                    let a1 = scaled(e, at(0xf8), far(), before);
                    let a2 = scaled(e, at(0xe0), radius, before_normal);
                    let moved = e.call(POINT_SUBTRACT, &args![vertex, at(0xec), a2]).u32();
                    let point = e.call(POINT_SUBTRACT, &args![moved, at(0x104), a1]).u32();
                    copy_point(e, old_point, point);
                    let b1 = scaled(e, at(0x128), far(), after);
                    let b2 = scaled(e, at(0x110), radius, after_normal);
                    let moved = e.call(POINT_ADD, &args![vertex, at(0x11c), b2]).u32();
                    let point = e.call(POINT_SUBTRACT, &args![moved, at(0x134), b1]).u32();
                    copy_point(e, new_point, point);
                } else {
                    let a1 = scaled(e, at(0x158), far(), before);
                    let a2 = scaled(e, at(0x140), radius, before_normal);
                    let moved = e.call(POINT_ADD, &args![vertex, at(0x14c), a2]).u32();
                    let point = e.call(POINT_SUBTRACT, &args![moved, at(0x164), a1]).u32();
                    copy_point(e, old_point, point);
                    let b1 = scaled(e, at(0x188), far(), after);
                    let b2 = scaled(e, at(0x170), radius, after_normal);
                    let moved = e.call(POINT_SUBTRACT, &args![vertex, at(0x17c), b2]).u32();
                    let point = e.call(POINT_SUBTRACT, &args![moved, at(0x194), b1]).u32();
                    copy_point(e, new_point, point);
                }
                fn_006ee740(
                    e,
                    this,
                    index,
                    left,
                    Ptr::new(old_point),
                    Ptr::new(new_point),
                );
                return index.wrapping_add(2);
            }
        }
        let push = vertex.wrapping_add(0x18);
        if left != 0 {
            let normal_part = e
                .call(POINT_SCALE, &args![after_normal, at(0x1ac), radius])
                .u32();
            let along_part = e
                .call(
                    POINT_SCALE,
                    &args![after, at(0x1a0), x87_quiet_bits(offset)],
                )
                .u32();
            let delta = e
                .call(POINT_ADD, &args![along_part, at(0x1b8), normal_part])
                .u32();
            copy_point(e, push, delta);
        } else {
            let normal_part = e
                .call(POINT_SCALE, &args![after_normal, at(0x1dc), radius])
                .u32();
            let negated = e.call(POINT_NEGATE, &args![after, at(0x1c4)]).u32();
            let along_part = e
                .call(
                    POINT_SCALE,
                    &args![negated, at(0x1d0), x87_quiet_bits(offset)],
                )
                .u32();
            let delta = e
                .call(POINT_SUBTRACT, &args![along_part, at(0x1e8), normal_part])
                .u32();
            copy_point(e, push, delta);
        }
        let shrunk = vertex.wrapping_add(0xc);
        let previous_edge = record(e, edges_of_side, previous);
        let mut open = e.mem.u8(previous_edge.wrapping_add(0x14)) != 0;
        if open {
            let this_edge = record(e, edges_of_side, index);
            open = e.mem.u8(this_edge.wrapping_add(0x14)) != 0;
        }
        let placed = if open {
            let next = record(e, verts, index.wrapping_add(1));
            let before_vertex = record(e, verts, previous);
            let handle = e.with_stack(8, |e, slot| {
                e.call(HANDLE_COPY, &args![slot, vertex.wrapping_add(0x24)]);
                (e.mem.u32(slot.addr()), e.mem.u32(slot.addr() + 4))
            });
            fn_006f0050(
                e,
                this,
                Ptr::new(vertex),
                Ptr::new(shrunk),
                handle.0,
                handle.1,
                Ptr::new(push),
                Ptr::new(before_vertex.wrapping_add(0xc)),
                Ptr::new(next),
            )
        } else {
            let location = e.call(POINT_ADD, &args![vertex, at(0x1f8), push]).u32();
            copy_point(e, shrunk, location);
            1
        };
        if placed != 0 {
            let current = record(e, verts, index);
            let current_shrunk = record(e, verts, index).wrapping_add(0xc);
            let prior = record(e, verts, previous);
            let prior_shrunk = record(e, verts, previous).wrapping_add(0xc);
            if e.call(
                SEGMENT_TEST_00696540,
                &args![prior_shrunk, prior, current_shrunk, current],
            )
            .u8()
                != 0
            {
                let prior_shrunk = record(e, verts, previous).wrapping_add(0xc);
                let current_shrunk = record(e, verts, index).wrapping_add(0xc);
                let gap = at(0x8c);
                e.call(POINT_SUBTRACT, &args![current_shrunk, gap, prior_shrunk]);
                let current_shrunk = record(e, verts, index).wrapping_add(0xc);
                let prior_shrunk = record(e, verts, previous).wrapping_add(0xc);
                let sum = e
                    .call(POINT_ADD, &args![prior_shrunk, at(0x204), current_shrunk])
                    .u32();
                let half = x87_quiet_bits(e.mem.u32(MIDPOINT_FACTOR));
                let middle = at(0x80);
                e.call(POINT_SCALE, &args![sum, middle, half]);
                let spread = x87_quiet_bits(e.mem.u32(SPREAD_FACTOR));
                let part = e.call(POINT_SCALE, &args![gap, at(0x210), spread]).u32();
                let moved = e.call(POINT_ADD, &args![middle, at(0x21c), part]).u32();
                let prior_shrunk = record(e, verts, previous).wrapping_add(0xc);
                copy_point(e, prior_shrunk, moved);
                let spread = x87_quiet_bits(e.mem.u32(SPREAD_FACTOR));
                let part = e.call(POINT_SCALE, &args![gap, at(0x228), spread]).u32();
                let moved = e
                    .call(POINT_SUBTRACT, &args![middle, at(0x234), part])
                    .u32();
                let current_shrunk = record(e, verts, index).wrapping_add(0xc);
                copy_point(e, current_shrunk, moved);
            }
        }
        index.wrapping_add(1)
    })
}

/// The pathing-location side of [`fn_006f0050`]: the mesh word's two lookups `00690800`
/// and `006b7770` (thiscall on the handle's first word), the offset point added to the
/// probe points (`011a946c`), `PathingLocation::PathingLocation_ov7` (`006dcee0`;
/// thiscall: point, the second and first lookup), `PathingLocation::PathingLocation`
/// (`006dcbb0`), `Pathing::CheckLineOfSight` (`006d7350`; cdecl: from, to, out hit, 0;
/// `AL`), `Pathing::CheckLineOfSight_ov2` (`006d7490`; cdecl: location, point, 0, 0;
/// `AL`), all names from the engine map, and the location's point copy `005c3420`
/// (thiscall: out; answers out).
const MESH_LOOKUP_00690800: u32 = 0x0069_0800;
const MESH_LOOKUP_006B7770: u32 = 0x006b_7770;
const PROBE_OFFSET: u32 = 0x011a_946c;
const LOCATION_FROM_POINT: u32 = 0x006d_cee0;
const LOCATION_DEFAULT: u32 = 0x006d_cbb0;
const CHECK_LINE_OF_SIGHT: u32 = 0x006d_7350;
const CHECK_LINE_OF_SIGHT_TO_POINT: u32 = 0x006d_7490;
const LOCATION_POINT: u32 = 0x005c_3420;

// Translated from 006f0050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f0050 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, `RET 0x1C`; called from [`fn_006ef0c0`]): places a
/// vertex's shrunk location. Two pathing locations are made on the vertex's mesh (the
/// handle's first word): `vertex + push * 01013ea4 + 011a946c` and
/// `vertex - push + 011a946c`. When `Pathing::CheckLineOfSight` between them (a third,
/// default location taking its result) answers false, `*shrunk` becomes that third
/// location's point plus `push` and the answer is 1. Otherwise `*shrunk` is first the
/// vertex location; when the second location sees both `previous` and `next`
/// (`CheckLineOfSight_ov2`) the answer is 0, else `*shrunk` becomes `vertex + push` and
/// the answer is 1. The three locations are destroyed (`004ff7e0`).
pub fn fn_006f0050(
    e: &mut Engine,
    _this: Ptr,
    vertex: Ptr,
    shrunk: Ptr,
    handle_mesh: u32,
    _handle_triangle: u32,
    push: Ptr,
    previous: Ptr,
    next: Ptr,
) -> u8 {
    let (vertex, push) = (vertex.addr(), push.addr());
    // The frame from EBP - 0xF4; `at(k)` is EBP - k.
    e.with_stack(0xf4, |e, frame| {
        let at = |k: u32| frame.addr() + 0xf4 - k;
        let (near, far, hit) = (at(0x84), at(0x34), at(0x5c));
        let first = e.call(MESH_LOOKUP_00690800, &args![handle_mesh]).u32();
        let second = e.call(MESH_LOOKUP_006B7770, &args![handle_mesh]).u32();
        let spread = x87_quiet_bits(e.mem.u32(SPREAD_FACTOR));
        let scaled = e.call(POINT_SCALE, &args![push, at(0x94), spread]).u32();
        let moved = e.call(POINT_ADD, &args![vertex, at(0xa0), scaled]).u32();
        let probe = e
            .call(POINT_ADD, &args![moved, at(0xac), PROBE_OFFSET])
            .u32();
        e.call(LOCATION_FROM_POINT, &args![near, probe, second, first]);
        let first = e.call(MESH_LOOKUP_00690800, &args![handle_mesh]).u32();
        let second = e.call(MESH_LOOKUP_006B7770, &args![handle_mesh]).u32();
        let moved = e.call(POINT_SUBTRACT, &args![vertex, at(0xb8), push]).u32();
        let probe = e
            .call(POINT_ADD, &args![moved, at(0xc4), PROBE_OFFSET])
            .u32();
        e.call(LOCATION_FROM_POINT, &args![far, probe, second, first]);
        e.call(LOCATION_DEFAULT, &args![hit]);
        let clear = e
            .call(CHECK_LINE_OF_SIGHT, &args![near, far, hit, 0u32])
            .u8()
            != 0;
        let answer = if clear {
            copy_point(e, shrunk.addr(), vertex);
            let mut sees_both = e
                .call(
                    CHECK_LINE_OF_SIGHT_TO_POINT,
                    &args![far, previous, 0u32, 0u32],
                )
                .u8()
                != 0;
            if sees_both {
                sees_both = e
                    .call(CHECK_LINE_OF_SIGHT_TO_POINT, &args![far, next, 0u32, 0u32])
                    .u8()
                    != 0;
            }
            if sees_both {
                0
            } else {
                let location = e.call(POINT_ADD, &args![vertex, at(0xd0), push]).u32();
                copy_point(e, shrunk.addr(), location);
                1
            }
        } else {
            let point = e.call(LOCATION_POINT, &args![hit, at(0xe0)]).u32();
            let location = e.call(POINT_ADD, &args![point, at(0xec), push]).u32();
            copy_point(e, shrunk.addr(), location);
            1
        };
        for location in [hit, far, near] {
            e.call(LOCATION_DESTRUCT, &args![location]);
        }
        answer
    })
}

/// The bad-node report of [`fn_006ef920`]: the 0x24-byte link record's constructor
/// ([`fn_006efd50`]), the pointer temporary's constructor (`0042fb00`) and destructor
/// (`0042fa40`), its value (`00559450`), the mesh lookup `0069ad00` (thiscall on the path
/// edge's mesh word: out pointer; `AL`) and the bad-node array's add (`006cf4c0`,
/// thiscall: the record).
const POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const POINTER_DESTRUCT: u32 = 0x0042_fa40;
const POINTER_GET: u32 = 0x0055_9450;
const MESH_LOOKUP_0069AD00: u32 = 0x0069_ad00;
const BAD_NODES_ADD: u32 = 0x006c_f4c0;
/// The eh vector constructor helper `00401050` (stdcall: array, element size, count,
/// constructor).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// The `float` the inserted vertex's shrunk location is pushed by (`0101e2bc`, read at
/// run time).
const INSERT_PUSH_FACTOR: u32 = 0x0101_e2bc;

/// Reports path edge `index` as bad: when `pBadNodes` is set, a link record (at `record`,
/// 0x24 bytes) and a pointer temporary (at `pointer`) are made; when the edge's mesh word
/// resolves (`0069ad00`) the record gets the pointer's value, the edge's triangle (+0x40)
/// and edge word (+0x44) and is added to the bad nodes (`006cf4c0`); the temporary is
/// destroyed.
fn report_bad_edge(e: &mut Engine, this: Ptr, index: u32, record: u32, pointer: u32) {
    let base = this.addr();
    if e.mem.u32(base) == 0 {
        return;
    }
    fn_006efd50(e, Ptr::new(record));
    e.call(POINTER_CONSTRUCT, &args![pointer]);
    let edge_at = e
        .call(ARRAY_ELEMENT, &args![base + PATH_EDGES, index])
        .u32();
    let edge = e.mem.u32(edge_at);
    let mesh = e.mem.u32(edge.wrapping_add(0x3c));
    if e.call(MESH_LOOKUP_0069AD00, &args![mesh, pointer]).u8() != 0 {
        let value = e.call(POINTER_GET, &args![pointer]).u32();
        e.mem.set_u32(record, value);
        let edge_at = e
            .call(ARRAY_ELEMENT, &args![base + PATH_EDGES, index])
            .u32();
        let edge = e.mem.u32(edge_at);
        let triangle = e.mem.u16(edge.wrapping_add(0x40));
        e.mem.set_u16(record + 4, triangle);
        let edge_at = e
            .call(ARRAY_ELEMENT, &args![base + PATH_EDGES, index])
            .u32();
        let edge = e.mem.u32(edge_at);
        let word = e.mem.u32(edge.wrapping_add(0x44));
        e.mem.set_u32(record + 8, word);
        let bad_nodes = e.mem.u32(base);
        e.call(BAD_NODES_ADD, &args![bad_nodes, record]);
    }
    e.call(POINTER_DESTRUCT, &args![pointer]);
}

/// The previous left (+0x4C) and right (+0x50) bounds edges of path edge `index`.
fn path_bounds(e: &mut Engine, this: Ptr, index: u32) -> (u32, u32) {
    let edges = this.addr() + PATH_EDGES;
    let at = e.call(ARRAY_ELEMENT, &args![edges, index]).u32();
    let left = e.mem.u32(e.mem.u32(at).wrapping_add(0x4c));
    let at = e.call(ARRAY_ELEMENT, &args![edges, index]).u32();
    let right = e.mem.u32(e.mem.u32(at).wrapping_add(0x50));
    (left, right)
}

// Translated from 006ef920 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006ef920 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`; its answer is [`fn_006ed530`]'s): checks
/// the shrunk path. Along the path edges, where one side keeps its bounds edge from the
/// previous path edge, the other side's new bounds edge must pass [`fn_006efd80`] against
/// it (left kept: `(right, left, 0)`, right kept: `(left, right, 1)`); then no path edge's
/// two bounds edges may cross (`00696540` on their shrunk end points). A failure reports
/// that path edge as bad (see [`report_bad_edge`]) and answers 0; otherwise 1.
pub fn fn_006ef920(e: &mut Engine, this: Ptr) -> u8 {
    let edges = this.addr() + PATH_EDGES;
    // The frame from EBP - 0xAC; `at(k)` is EBP - k.
    e.with_stack(0xac, |e, frame| {
        let at = |k: u32| frame.addr() + 0xac - k;
        let (mut previous_left, mut previous_right) = path_bounds(e, this, 0);
        let mut index = 1u32;
        while index < e.call(ARRAY_COUNT, &args![edges]).u32() {
            let (left, right) = path_bounds(e, this, index);
            if left == previous_left && fn_006efd80(e, this, right, left, 0) == 0 {
                report_bad_edge(e, this, index, at(0x48), at(0x24));
                return 0;
            }
            if right == previous_right && fn_006efd80(e, this, left, right, 1) == 0 {
                report_bad_edge(e, this, index, at(0x70), at(0x4c));
                return 0;
            }
            previous_left = left;
            previous_right = right;
            index += 1;
        }
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![edges]).u32() {
            let (left, right) = path_bounds(e, this, index);
            let left_start = e.mem.u32(left).wrapping_add(0xc);
            let left_end = e.mem.u32(left.wrapping_add(4)).wrapping_add(0xc);
            let right_start = e.mem.u32(right).wrapping_add(0xc);
            let right_end = e.mem.u32(right.wrapping_add(4)).wrapping_add(0xc);
            if e.call(
                SEGMENT_TEST_00696540,
                &args![left_start, left_end, right_start, right_end],
            )
            .u8()
                != 0
            {
                report_bad_edge(e, this, index, at(0xa4), at(0x80));
                return 0;
            }
            index += 1;
        }
        1
    })
}

// Translated from 006efd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`): the link record's constructor: its two points at
/// +0xC (`00401050` with element size 0xC, count 2 and `006815c0`). Answers `this`.
pub fn fn_006efd50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this.addr() + 0xc, 0xcu32, 2u32, POINT_DEFAULT],
    );
    this
}

// Translated from 006efd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006efd80 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, `RET 0xC`): whether bounds edge `edge` may follow
/// `other` (`left` picks the side). When the end vertex of `other` lies across `edge`'s
/// shrunk segment (`00696540`), the vertex's push `d` (shrunk minus location) is probed:
/// from a pathing location at its shrunk location (`006dcf20` with its mesh handle), the
/// point `shrunk + d` must be visible (`CheckLineOfSight_ov2`), and from two more locations
/// at `shrunk + d` both shrunk end points of `edge`; then `edge` is split
/// ([`fn_006eed30`] at its `iEdgeIndex`), the new vertex placed at the shrunk location with
/// shrunk location `shrunk + d * 0101e2bc`, and the answer is 1; a failed probe answers 0
/// (the locations are destroyed, `004ff7e0`). Otherwise the answer is whether `other`'s
/// shrunk segment does not cross `edge`'s.
pub fn fn_006efd80(e: &mut Engine, this: Ptr, edge: u32, other: u32, left: u8) -> u8 {
    // The frame from EBP - 0xF4; `at(k)` is EBP - k.
    e.with_stack(0xf4, |e, frame| {
        let at = |k: u32| frame.addr() + 0xf4 - k;
        let (start, end) = (at(0x24), at(0x18));
        let edge_start = e.mem.u32(edge).wrapping_add(0xc);
        copy_point(e, start, edge_start);
        let edge_end = e.mem.u32(edge.wrapping_add(4)).wrapping_add(0xc);
        copy_point(e, end, edge_end);
        let vertex = e.mem.u32(other.wrapping_add(4));
        let shrunk = vertex.wrapping_add(0xc);
        if e.call(SEGMENT_TEST_00696540, &args![start, end, vertex, shrunk])
            .u8()
            == 0
        {
            let other_start = e.mem.u32(other).wrapping_add(0xc);
            let other_end = e.mem.u32(other.wrapping_add(4)).wrapping_add(0xc);
            let crosses = e
                .call(
                    SEGMENT_TEST_00696540,
                    &args![start, end, other_start, other_end],
                )
                .u8()
                != 0;
            return u8::from(!crosses);
        }
        let push = at(0x5c);
        e.call(POINT_SUBTRACT, &args![shrunk, push, vertex]);
        let (here, ahead, behind) = (at(0x50), at(0x84), at(0xac));
        let handle = e.mem.u32(vertex.wrapping_add(0x24));
        e.call(LOCATION_CONSTRUCT, &args![here, shrunk, handle]);
        let probe = e.call(POINT_ADD, &args![shrunk, at(0xbc), push]).u32();
        let visible = e
            .call(
                CHECK_LINE_OF_SIGHT_TO_POINT,
                &args![here, probe, 0u32, 0u32],
            )
            .u8()
            != 0;
        let mut answer = 0u8;
        if visible {
            let handle = e.mem.u32(vertex.wrapping_add(0x24));
            let point = e.call(POINT_ADD, &args![shrunk, at(0xc8), push]).u32();
            e.call(LOCATION_CONSTRUCT, &args![ahead, point, handle]);
            let handle = e.mem.u32(vertex.wrapping_add(0x24));
            let point = e.call(POINT_ADD, &args![shrunk, at(0xd4), push]).u32();
            e.call(LOCATION_CONSTRUCT, &args![behind, point, handle]);
            let sees_start = e
                .call(
                    CHECK_LINE_OF_SIGHT_TO_POINT,
                    &args![ahead, start, 0u32, 0u32],
                )
                .u8()
                != 0;
            let sees_both = sees_start
                && e.call(
                    CHECK_LINE_OF_SIGHT_TO_POINT,
                    &args![behind, end, 0u32, 0u32],
                )
                .u8()
                    != 0;
            if sees_both {
                let edge_index = e.mem.u32(edge.wrapping_add(0x18));
                let inserted = fn_006eed30(e, this, edge_index, left);
                copy_point(e, inserted, shrunk);
                let factor = x87_quiet_bits(e.mem.u32(INSERT_PUSH_FACTOR));
                let scaled = e.call(POINT_SCALE, &args![push, at(0xe0), factor]).u32();
                let moved = e.call(POINT_ADD, &args![shrunk, at(0xec), scaled]).u32();
                copy_point(e, inserted.wrapping_add(0xc), moved);
                answer = 1;
            }
            e.call(LOCATION_DESTRUCT, &args![behind]);
            e.call(LOCATION_DESTRUCT, &args![ahead]);
        }
        e.call(LOCATION_DESTRUCT, &args![here]);
        answer
    })
}

/// The path edge `index` (the word at the address `00877a30` answers on `PathEdges`).
fn path_edge(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let at = e
        .call(ARRAY_ELEMENT, &args![this.addr() + PATH_EDGES, index])
        .u32();
    e.mem.u32(at)
}

// Translated from 006f02e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): copies each path edge's left and right
/// vertices' shrunk locations into its `LeftShrunkLocation` and `RightShrunkLocation`.
pub fn fn_006f02e0(e: &mut Engine, this: Ptr) {
    let edges = this.addr() + PATH_EDGES;
    let mut index = 0u32;
    while index < e.call(ARRAY_COUNT, &args![edges]).u32() {
        for (vertex, field) in [(0u32, 0x20u32), (4, 0x2c)] {
            let edge = path_edge(e, this, index);
            let source = e.mem.u32(edge.wrapping_add(vertex)).wrapping_add(0xc);
            let edge = path_edge(e, this, index);
            copy_point(e, edge.wrapping_add(field), source);
        }
        index += 1;
    }
}

// Translated from 006f03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): sets each path edge's `Center` to the
/// midpoint of its shrunk locations (`(left + right) * 01016248`).
pub fn fn_006f03a0(e: &mut Engine, this: Ptr) {
    let edges = this.addr() + PATH_EDGES;
    // The frame from EBP - 0x20; `at(k)` is EBP - k.
    e.with_stack(0x20, |e, frame| {
        let at = |k: u32| frame.addr() + 0x20 - k;
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![edges]).u32() {
            let half = x87_quiet_bits(e.mem.u32(MIDPOINT_FACTOR));
            let right = path_edge(e, this, index).wrapping_add(0x2c);
            let left = path_edge(e, this, index).wrapping_add(0x20);
            let sum = e.call(POINT_ADD, &args![left, at(0x10), right]).u32();
            let center = e.call(POINT_SCALE, &args![sum, at(0x1c), half]).u32();
            let edge = path_edge(e, this, index);
            copy_point(e, edge.wrapping_add(8), center);
            index += 1;
        }
    });
}

/// The point helpers of [`fn_006f0450`] (thiscall on a point unless noted): the
/// constructor from three floats (`00416870`), `+=` (`0063c8a0`, other), `/=` by a float
/// (`004941c0`), the length (`00595c80`, ST0), the cdecl best-point function `006ba550`
/// (out, left, right, before, after; in `navmeshmath.cpp` by the engine map) and the cdecl
/// float function `00408840` (one float; ST0); the request's weight (`0051b8e0`, thiscall
/// on `pRequest`; ST0) and the `float` at `0106682c` (the near factor), read at run time.
const POINT_FROM_FLOATS: u32 = 0x0041_6870;
const POINT_ADD_ASSIGN: u32 = 0x0063_c8a0;
const POINT_DIVIDE: u32 = 0x0049_41c0;
const POINT_LENGTH: u32 = 0x0059_5c80;
const BEST_POINT_006BA550: u32 = 0x006b_a550;
const CURVE_00408840: u32 = 0x0040_8840;
const REQUEST_WEIGHT: u32 = 0x0051_b8e0;
const NEAR_FACTOR: u32 = 0x0106_682c;

/// The average of the path-edge centres at `index + offset` for `offsets` (the start
/// location for offset -1, the goal location for an offset of the count), into `out`;
/// `fallback` when none counts.
fn average_centers(
    e: &mut Engine,
    this: Ptr,
    out: u32,
    index: i32,
    offsets: std::ops::RangeInclusive<i32>,
    fallback: u32,
) {
    let base = this.addr();
    e.call(POINT_FROM_FLOATS, &args![out, 0u32, 0u32, 0u32]);
    let mut found = 0u32;
    for offset in offsets {
        let at = index.wrapping_add(offset);
        if at == -1 {
            found += 1;
            e.call(POINT_ADD_ASSIGN, &args![out, base + START_LOCATION]);
        } else if at == e.call(ARRAY_COUNT, &args![base + PATH_EDGES]).u32() as i32 {
            found += 1;
            e.call(POINT_ADD_ASSIGN, &args![out, base + GOAL_LOCATION]);
        } else if at >= 0 && at < e.call(ARRAY_COUNT, &args![base + PATH_EDGES]).u32() as i32 {
            found += 1;
            let center = path_edge(e, this, at as u32).wrapping_add(8);
            e.call(POINT_ADD_ASSIGN, &args![out, center]);
        }
    }
    if found != 0 {
        let divisor = x87_store32(f64::from(found));
        e.call(POINT_DIVIDE, &args![out, divisor]);
    } else {
        copy_point(e, out, fallback);
    }
}

/// `point(index) + offset` of a path edge field.
fn edge_field(e: &mut Engine, this: Ptr, index: u32, offset: u32) -> u32 {
    path_edge(e, this, index).wrapping_add(offset)
}

/// `BestPointOnEdge = left * left_factor + right * right_factor` of path edge `index`
/// (`0045bb20` on the right shrunk location first).
fn blend_best(
    e: &mut Engine,
    this: Ptr,
    frame_at: &dyn Fn(u32) -> u32,
    index: u32,
    right_factor: u32,
    left_factor: u32,
    outs: [u32; 3],
) {
    let right_factor = x87_quiet_bits(e.mem.u32(right_factor));
    let right = edge_field(e, this, index, 0x2c);
    let right_part = e
        .call(POINT_SCALE, &args![right, frame_at(outs[0]), right_factor])
        .u32();
    let left_factor = x87_quiet_bits(e.mem.u32(left_factor));
    let left = edge_field(e, this, index, 0x20);
    let left_part = e
        .call(POINT_SCALE, &args![left, frame_at(outs[1]), left_factor])
        .u32();
    let best = e
        .call(POINT_ADD, &args![left_part, frame_at(outs[2]), right_part])
        .u32();
    let target = edge_field(e, this, index, 0x14);
    copy_point(e, target, best);
}

// Translated from 006f0450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): places each path edge's
/// `BestPointOnEdge`. For path edge `i` (signed), the averages of the centres of the four
/// edges before and the four after (the start or goal location standing in for indices -1
/// and the count, the start or goal itself when nothing counts) give `006ba550`'s best
/// point between the edge's shrunk locations. When that lands on one of them: with `i > 0`
/// and the direction between the averages and the edge's direction nearly parallel
/// (`00408840` of their dot product above `010217d8`) the point becomes a blend of the
/// shrunk locations (`01013ea4` and `0106682c`, the larger weight on the side farther from
/// the previous edge's best point); otherwise it moves from the end it landed on toward
/// the other by `01013ea4` of the edge. Finally the point moves toward the right (request
/// weight `0051b8e0` positive) or left shrunk location by `00408840(weight) * 01011588`
/// of the way.
pub fn fn_006f0450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    // The frame from EBP - 0x148; `at(k)` is EBP - k.
    e.with_stack(0x148, |e, frame| {
        let at = |k: u32| frame.addr() + 0x148 - k;
        let (before, after) = (at(0x14), at(0x34));
        let mut index = 0i32;
        while index < e.call(ARRAY_COUNT, &args![base + PATH_EDGES]).u32() as i32 {
            let i = index as u32;
            average_centers(e, this, before, index, -4..=-1, base + START_LOCATION);
            average_centers(e, this, after, index, 1..=4, base + GOAL_LOCATION);
            let right = edge_field(e, this, i, 0x2c);
            let left = edge_field(e, this, i, 0x20);
            let best = e
                .call(
                    BEST_POINT_006BA550,
                    &args![at(0x88), left, right, before, after],
                )
                .u32();
            let target = edge_field(e, this, i, 0x14);
            copy_point(e, target, best);
            let left = edge_field(e, this, i, 0x20);
            let best_at = edge_field(e, this, i, 0x14);
            let mut on_end = e.call(POINT_EQUAL, &args![best_at, left]).u8() != 0;
            if !on_end {
                let right = edge_field(e, this, i, 0x2c);
                let best_at = edge_field(e, this, i, 0x14);
                on_end = e.call(POINT_EQUAL, &args![best_at, right]).u8() != 0;
            }
            if on_end {
                let way = at(0x50);
                e.call(POINT_SUBTRACT, &args![after, way, before]);
                e.call(POINT_NORMALIZE, &args![way]);
                let right = edge_field(e, this, i, 0x2c);
                let left = edge_field(e, this, i, 0x20);
                let across = at(0x5c);
                e.call(POINT_SUBTRACT, &args![left, across, right]);
                e.call(POINT_NORMALIZE, &args![across]);
                let parallel = index > 0 && {
                    let dot = x87_store32(e.call(POINT_DOT, &args![way, across]).st0);
                    let shaped = e.call(CURVE_00408840, &args![dot]).st0;
                    shaped > e.global::<f64>(STRAIGHT_LIMIT)
                };
                if parallel {
                    let previous_best = edge_field(e, this, i - 1, 0x14);
                    let left = edge_field(e, this, i, 0x20);
                    let to_left = e
                        .call(POINT_SUBTRACT, &args![left, at(0x94), previous_best])
                        .u32();
                    let left_distance = x87_store32(e.call(POINT_LENGTH, &args![to_left]).st0);
                    e.mem.set_u32(at(0x60), left_distance);
                    let previous_best = edge_field(e, this, i - 1, 0x14);
                    let right = edge_field(e, this, i, 0x2c);
                    let to_right = e
                        .call(POINT_SUBTRACT, &args![right, at(0xa0), previous_best])
                        .u32();
                    let right_distance = x87_store32(e.call(POINT_LENGTH, &args![to_right]).st0);
                    e.mem.set_u32(at(0x64), right_distance);
                    if x87_load32(right_distance) > x87_load32(left_distance) {
                        blend_best(
                            e,
                            this,
                            &at,
                            i,
                            SPREAD_FACTOR,
                            NEAR_FACTOR,
                            [0xb8, 0xac, 0xc4],
                        );
                    } else {
                        blend_best(
                            e,
                            this,
                            &at,
                            i,
                            NEAR_FACTOR,
                            SPREAD_FACTOR,
                            [0xdc, 0xd0, 0xe8],
                        );
                    }
                } else {
                    let left = edge_field(e, this, i, 0x20);
                    let best_at = edge_field(e, this, i, 0x14);
                    if e.call(POINT_EQUAL, &args![best_at, left]).u8() != 0 {
                        let left = edge_field(e, this, i, 0x20);
                        let right = edge_field(e, this, i, 0x2c);
                        let step = at(0x70);
                        e.call(POINT_SUBTRACT, &args![right, step, left]);
                        let spread = x87_quiet_bits(e.mem.u32(SPREAD_FACTOR));
                        let part = e.call(POINT_SCALE, &args![step, at(0xf4), spread]).u32();
                        let best_at = edge_field(e, this, i, 0x14);
                        e.call(POINT_ADD_ASSIGN, &args![best_at, part]);
                    } else {
                        let right = edge_field(e, this, i, 0x2c);
                        let best_at = edge_field(e, this, i, 0x14);
                        if e.call(POINT_EQUAL, &args![best_at, right]).u8() != 0 {
                            let right = edge_field(e, this, i, 0x2c);
                            let left = edge_field(e, this, i, 0x20);
                            let step = at(0x7c);
                            e.call(POINT_SUBTRACT, &args![left, step, right]);
                            let spread = x87_quiet_bits(e.mem.u32(SPREAD_FACTOR));
                            let part = e.call(POINT_SCALE, &args![step, at(0x100), spread]).u32();
                            let best_at = edge_field(e, this, i, 0x14);
                            e.call(POINT_ADD_ASSIGN, &args![best_at, part]);
                        }
                    }
                }
            }
            let request = e.mem.u32(base + 4);
            let weight = x87_store32(e.call(REQUEST_WEIGHT, &args![request]).st0);
            e.mem.set_u32(at(0x28), weight);
            let toward = if x87_load32(weight) > e.global::<f64>(DOUBLE_ZERO) {
                let best_at = edge_field(e, this, i, 0x14);
                let right = edge_field(e, this, i, 0x2c);
                e.call(POINT_SUBTRACT, &args![right, at(0x110), best_at])
                    .u32()
            } else {
                let best_at = edge_field(e, this, i, 0x14);
                let left = edge_field(e, this, i, 0x20);
                e.call(POINT_SUBTRACT, &args![left, at(0x11c), best_at])
                    .u32()
            };
            let direction = at(0x24);
            copy_point(e, direction, toward);
            let shaped = x87_store32(e.call(CURVE_00408840, &args![x87_quiet_bits(weight)]).st0);
            e.mem.set_u32(at(0x28), shaped);
            let amount = x87_store32(x87_mul(x87_load32(shaped), e.global::<f64>(ANGLE_FACTOR)));
            let part = e
                .call(
                    SCALE_POINT,
                    &args![at(0x128), x87_quiet_bits(amount), direction],
                )
                .u32();
            let best_at = edge_field(e, this, i, 0x14);
            e.call(POINT_ADD_ASSIGN, &args![best_at, part]);
            index += 1;
        }
    });
}

/// The water pass of [`fn_006f0d20`]: the scrap flag array's constructor (`006f2ca0`,
/// thiscall: size), element address (`006a62f0`, thiscall: index) and destructor
/// (`006cf610`), its search (`006cf700`, thiscall: the value's address, the first index,
/// the comparison `006cf760`; answers the index or -1), the triangle's water test
/// (`006a5ec0`, thiscall on what `005582f0` answers; `AL`) and the horizontal length
/// (`00589850`, thiscall on a point; ST0).
const FLAG_ARRAY_CONSTRUCT: u32 = 0x006f_2ca0;
const FLAG_AT: u32 = 0x006a_62f0;
const FLAG_ARRAY_DESTRUCT: u32 = 0x006c_f610;
const FLAG_FIND: u32 = 0x006c_f700;
const FLAG_COMPARE: u32 = 0x006c_f760;
const TRIANGLE_IS_WATER: u32 = 0x006a_5ec0;
const POINT_LENGTH_XY: u32 = 0x0058_9850;

// Translated from 006f0d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f0d20 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): marks the water stretches of the path. A
/// flag per path edge says whether its triangle is water (`006a5ec0`, when its mesh word
/// resolves through `0069ad00`); every `eEnterLeaveWater` (+0x54) is cleared. For each run
/// of set flags, the path edge before it gets 1 and the one after it 2, and the third word
/// of each `BestPointOnEdge` in the run is interpolated between the best points (or start
/// and goal) around it by horizontal distance (`00589850`).
pub fn fn_006f0d20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let edges = base + PATH_EDGES;
    // The frame from EBP - 0xA8; `at(k)` is EBP - k.
    e.with_stack(0xa8, |e, frame| {
        let at = |k: u32| frame.addr() + 0xa8 - k;
        let flags = at(0x20);
        let count = e.call(ARRAY_COUNT, &args![edges]).u32();
        e.call(FLAG_ARRAY_CONSTRUCT, &args![flags, count]);
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![edges]).u32() {
            let pointer = at(0x2c);
            e.call(POINTER_CONSTRUCT, &args![pointer]);
            let mesh_at = edge_field(e, this, index, 0x3c);
            let mesh = e.mem.u32(mesh_at);
            if e.call(MESH_LOOKUP_0069AD00, &args![mesh, pointer]).u8() != 0 {
                let edge = path_edge(e, this, index);
                let triangle = u32::from(e.mem.u16(edge.wrapping_add(0x40)));
                let resolved = e.call(POINTER_GET, &args![pointer]).u32();
                let cell = e
                    .call(MESH_TRIANGLE_005582F0, &args![resolved, triangle])
                    .u32();
                let water = e.call(TRIANGLE_IS_WATER, &args![cell]).u8();
                let flag = e.call(FLAG_AT, &args![flags, index]).u32();
                e.mem.set_u8(flag, water);
            }
            let edge = path_edge(e, this, index);
            e.mem.set_u32(edge.wrapping_add(0x54), 0);
            e.call(POINTER_DESTRUCT, &args![pointer]);
            index += 1;
        }
        let find = |e: &mut Engine, value_at: u32, value: u8, from: u32| -> u32 {
            e.mem.set_u8(value_at, value);
            e.call(FLAG_FIND, &args![flags, value_at, from, FLAG_COMPARE])
                .u32()
        };
        let mut start = find(e, at(0x61), 1, 0);
        while start != u32::MAX {
            let mut end = find(e, at(0x62), 0, start.wrapping_add(1));
            if end == u32::MAX {
                end = e.call(ARRAY_COUNT, &args![flags]).u32().wrapping_sub(1);
            } else {
                end = end.wrapping_sub(1);
            }
            let previous = if start == 0 {
                0
            } else {
                path_edge(e, this, start - 1)
            };
            let last = e.call(ARRAY_COUNT, &args![flags]).u32().wrapping_sub(1);
            let next = if end == last {
                0
            } else {
                path_edge(e, this, end.wrapping_add(1))
            };
            if previous != 0 {
                e.mem.set_u32(previous.wrapping_add(0x54), 1);
            }
            if next != 0 {
                e.mem.set_u32(next.wrapping_add(0x54), 2);
            }
            let (from, to) = (at(0x54), at(0x48));
            let from_source = if previous != 0 {
                previous.wrapping_add(0x14)
            } else {
                base + START_LOCATION
            };
            copy_point(e, from, from_source);
            let to_source = if next != 0 {
                next.wrapping_add(0x14)
            } else {
                base + GOAL_LOCATION
            };
            copy_point(e, to, to_source);
            let rise = x87_store32(x87_sub(
                x87_load32(e.mem.u32(to + 8)),
                x87_m32(e.mem.u32(from + 8)),
            ));
            e.mem.set_u32(at(0x58), rise);
            let span = e.call(POINT_SUBTRACT, &args![to, at(0x78), from]).u32();
            let total = x87_store32(e.call(POINT_LENGTH_XY, &args![span]).st0);
            e.mem.set_u32(at(0x30), total);
            let mut step = start;
            while step <= end {
                let best = edge_field(e, this, step, 0x14);
                let offset = e.call(POINT_SUBTRACT, &args![best, at(0x84), from]).u32();
                let distance = x87_store32(e.call(POINT_LENGTH_XY, &args![offset]).st0);
                e.mem.set_u32(at(0x60), distance);
                let scaled = x87_mul(x87_load32(rise), x87_m32(distance));
                let height = x87_add(
                    x87_div(scaled, x87_m32(total)),
                    x87_m32(e.mem.u32(from + 8)),
                );
                let edge = path_edge(e, this, step);
                e.mem.set_u32(edge.wrapping_add(0x1c), x87_store32(height));
                step = step.wrapping_add(1);
            }
            let last = e.call(ARRAY_COUNT, &args![flags]).u32().wrapping_sub(1);
            start = if end != last {
                find(e, at(0x85), 1, end.wrapping_add(1))
            } else {
                u32::MAX
            };
        }
        e.call(FLAG_ARRAY_DESTRUCT, &args![flags]);
    });
}

/// `StraightenedPath`'s add (`006cf1b0`, thiscall: the location) and the bend limit
/// (`01018a90`, a `double` read at run time).
const STRAIGHTENED_ADD: u32 = 0x006c_f1b0;
const BEND_LIMIT: u32 = 0x0101_8a90;

/// Adds a pathing location at `point` with mesh handle `handle` to `StraightenedPath`
/// (made at `location` on the stack, `006dcf20`, then destroyed).
fn add_straightened(e: &mut Engine, this: Ptr, location: u32, point: u32, handle: u32) {
    e.call(LOCATION_CONSTRUCT, &args![location, point, handle]);
    e.call(
        STRAIGHTENED_ADD,
        &args![this.addr() + STRAIGHTENED_PATH, location],
    );
    e.call(LOCATION_DESTRUCT, &args![location]);
}

/// The mesh handle of the triangle `get` answers on `Triangles` (`0068efb0`).
fn triangle_handle(e: &mut Engine, this: Ptr, get: u32) -> u32 {
    let triangles = e.mem.u32(this.addr() + TRIANGLES);
    let triangle = e.call(get, &args![triangles]).u32();
    let mesh = e.mem.u32(triangle);
    e.call(NAV_MESH_HANDLE, &args![mesh]).u32()
}

// Translated from 006f10d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f10d0 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): builds `StraightenedPath`: the start
/// (on the first triangle), then, walking the path edges (signed index `j` from 1 to the
/// count), the best point of edge `j - 1` (with its edge handle word) whenever the summed
/// bends `1 - fTriangleNormalDelta` since the last kept point pass `01018a90`, edge `j`
/// enters or leaves water (+0x54), or the straight line from the last kept point (or the
/// start) to edge `j`'s best point (or the goal) misses one of the edges between
/// ([`fn_006ed7d0`]); then the goal (on the last triangle).
pub fn fn_006f10d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let edges = base + PATH_EDGES;
    // The frame from EBP - 0xC8; `at(k)` is EBP - k.
    e.with_stack(0xc8, |e, frame| {
        let at = |k: u32| frame.addr() + 0xc8 - k;
        let handle = triangle_handle(e, this, FIRST_TRIANGLE);
        add_straightened(e, this, at(0x40), base + START_LOCATION, handle);
        let mut last = -1i32;
        let mut index = 1i32;
        let mut bends = 0u32;
        while index < e.call(ARRAY_COUNT, &args![edges]).u32().wrapping_add(1) as i32 {
            let j = index as u32;
            if j != e.call(ARRAY_COUNT, &args![edges]).u32() {
                let delta_at = edge_field(e, this, j - 1, 0x38);
                let delta = x87_load32(e.mem.u32(delta_at));
                bends = x87_store32(x87_add(x87_sub(1.0, delta), x87_m32(bends)));
            }
            let mut keep = x87_load32(bends) > e.global::<f64>(BEND_LIMIT);
            if !keep {
                let next = if j == e.call(ARRAY_COUNT, &args![edges]).u32() {
                    0
                } else {
                    path_edge(e, this, j)
                };
                if next != 0 && e.mem.u32(next.wrapping_add(0x54)) != 0 {
                    keep = true;
                } else {
                    let (from, to) = (at(0x60), at(0x54));
                    let from_source = if last == -1 {
                        base + START_LOCATION
                    } else {
                        edge_field(e, this, last as u32, 0x14)
                    };
                    copy_point(e, from, from_source);
                    let to_source = if j == e.call(ARRAY_COUNT, &args![edges]).u32() {
                        base + GOAL_LOCATION
                    } else {
                        edge_field(e, this, j, 0x14)
                    };
                    copy_point(e, to, to_source);
                    let first = (last as u32).wrapping_add(1);
                    keep = fn_006ed7d0(e, this, first, j - 1, from, to) == 0;
                }
            }
            if keep {
                let handle_at = edge_field(e, this, j - 1, 0x3c);
                let handle = e.mem.u32(handle_at);
                let point = edge_field(e, this, j - 1, 0x14);
                add_straightened(e, this, at(0x88), point, handle);
                bends = 0;
                last = index - 1;
            }
            index += 1;
        }
        let handle = triangle_handle(e, this, LAST_TRIANGLE);
        add_straightened(e, this, at(0xb0), base + GOAL_LOCATION, handle);
    });
}

/// The avoidance pass of [`fn_006f1440`]: the request's obstacle list (`0069e390`,
/// thiscall on `pRequest`; null when none), its element (`006a1400`, thiscall: index; 0x24
/// bytes), the obstacle's radius (`004a7bd0`, thiscall; ST0; the map's name for the body is
/// a folded one), the avoid-node array's constructor (`006f2d80`), add (`006f2d20`, the
/// node's address), element (`00558dd0`, index) and destructor (`006f2e40`), the
/// straightened path's element (`00691750`, index), its remove (`006f29e0`, thiscall:
/// first, count, 0) and insert (`006f28a0`, thiscall: index, location), the location from
/// a point and a location (`006dd0d0`, thiscall: point, location), the squared length
/// (`004a7290`, thiscall on a point; ST0), `NavMeshMath::SegmentCircleIntersection`
/// (`006ba9e0`, engine map; cdecl: from, to, centre, radius; `AL`) and the cdecl tangent
/// function `006ba8c0` (centre, radius, point, out, out; `AL`).
const OBSTACLES: u32 = 0x0069_e390;
const OBSTACLE_AT: u32 = 0x006a_1400;
const OBSTACLE_RADIUS: u32 = 0x004a_7bd0;
const AVOID_CONSTRUCT: u32 = 0x006f_2d80;
const AVOID_ADD: u32 = 0x006f_2d20;
const AVOID_AT: u32 = 0x0055_8dd0;
const AVOID_DESTRUCT: u32 = 0x006f_2e40;
const STRAIGHTENED_AT: u32 = 0x0069_1750;
const STRAIGHTENED_REMOVE: u32 = 0x006f_29e0;
const STRAIGHTENED_INSERT: u32 = 0x006f_28a0;
const LOCATION_FROM_POINT_AND_LOCATION: u32 = 0x006d_d0d0;
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
const SEGMENT_CIRCLE_INTERSECTION: u32 = 0x006b_a9e0;
const TANGENTS_006BA8C0: u32 = 0x006b_a8c0;

/// The point of straightened-path location `index` (`005c3420` into `out`).
fn straightened_point(e: &mut Engine, this: Ptr, index: u32, out: u32) -> u32 {
    let location = e
        .call(
            STRAIGHTENED_AT,
            &args![this.addr() + STRAIGHTENED_PATH, index],
        )
        .u32();
    e.call(LOCATION_POINT, &args![location, out]).u32()
}

/// The avoid node `index` of the array at `avoid` and its radius word.
fn avoid_node(e: &mut Engine, avoid: u32, index: u32) -> u32 {
    e.call(AVOID_AT, &args![avoid, index]).u32()
}

/// `006ba8c0` for avoid node `node_index` from the straightened point `point_index`
/// (into `out`), with the two tangent points written to `first` and `second` (the point
/// array `00401050` made at `first`).
fn tangents(
    e: &mut Engine,
    this: Ptr,
    avoid: u32,
    node_index: u32,
    point_index: u32,
    out: u32,
    first: u32,
) -> bool {
    e.call(VECTOR_CONSTRUCT, &args![first, 0xcu32, 2u32, POINT_DEFAULT]);
    let point = straightened_point(e, this, point_index, out);
    let node = avoid_node(e, avoid, node_index);
    let radius = x87_quiet_bits(e.mem.u32(node.wrapping_add(0xc)));
    let node = avoid_node(e, avoid, node_index);
    e.call(
        TANGENTS_006BA8C0,
        &args![node, radius, point, first, first + 0xc],
    )
    .u8()
        != 0
}

// Translated from 006f1440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f1440 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, plain `RET`): bends the straightened path around the
/// request's obstacles (`0069e390`; nothing without them). Each obstacle becomes an avoid
/// node (its point and radius `004a7bd0`). For each node, each straightened segment `s`
/// that meets its circle (`NavMeshMath::SegmentCircleIntersection`) gets the tangent
/// points from point `s` (`006ba8c0`; both the point itself when there are none), tested
/// with [`fn_006f1e90`]; the first later point `t` outside the circle ([`fn_006f1bc0`])
/// gives tangent points too. When both have tangents and one side passes on both ends (the
/// shorter pair by `004a7290` when both do), the points between are removed (`006f29e0`)
/// and the two tangent points of that side inserted after `s` (`006dd0d0`, `006f28a0`),
/// ending that node; when no side passes, `s` jumps to `t - 1`.
pub fn fn_006f1440(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let request = e.mem.u32(base + 4);
    let obstacles = e.call(OBSTACLES, &args![request]).u32();
    if obstacles == 0 {
        return;
    }
    let path = base + STRAIGHTENED_PATH;
    // The frame from EBP - 0x1C0; `at(k)` is EBP - k.
    e.with_stack(0x1c0, |e, frame| {
        let at = |k: u32| frame.addr() + 0x1c0 - k;
        let avoid = at(0x24);
        e.call(AVOID_CONSTRUCT, &args![avoid]);
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![obstacles]).u32() {
            let source = e.call(OBSTACLE_AT, &args![obstacles, index]).u32();
            let obstacle = at(0x50);
            for word in 0..9u32 {
                let value = e.mem.u32(source.wrapping_add(4 * word));
                e.mem.set_u32(obstacle + 4 * word, value);
            }
            // The exe sets a stack byte to 1 and tests it: the node is always added.
            let node = at(0x60);
            e.call(HANDLE_CONSTRUCT, &args![node]);
            let center = e.call(POINT_DEFAULT, &args![obstacle]).u32();
            copy_point(e, node, center);
            let radius = x87_store32(e.call(OBSTACLE_RADIUS, &args![obstacle]).st0);
            e.mem.set_u32(at(0x54), radius);
            e.call(AVOID_ADD, &args![avoid, node]);
            index += 1;
        }
        let mut node_index = 0u32;
        while node_index < e.call(ARRAY_COUNT, &args![avoid]).u32() {
            let mut segment = 0u32;
            loop {
                let count = e.call(ARRAY_COUNT, &args![path]).u32();
                if segment >= count.wrapping_sub(1) {
                    break;
                }
                let node = avoid_node(e, avoid, node_index);
                let radius = x87_quiet_bits(e.mem.u32(node.wrapping_add(0xc)));
                let node = avoid_node(e, avoid, node_index);
                let to = straightened_point(e, this, segment.wrapping_add(1), at(0xcc));
                let from = straightened_point(e, this, segment, at(0xd8));
                if e.call(SEGMENT_CIRCLE_INTERSECTION, &args![from, to, node, radius])
                    .u8()
                    == 0
                {
                    segment += 1;
                    continue;
                }
                let (near_first, near_second) = (at(0x80), at(0x74));
                if !tangents(e, this, avoid, node_index, segment, at(0xe4), near_first) {
                    let point = straightened_point(e, this, segment, at(0xf0));
                    copy_point(e, near_second, point);
                    copy_point(e, near_first, near_second);
                }
                let point = straightened_point(e, this, segment, at(0xfc));
                let first_near = fn_006f1e90(e, this, point, near_first);
                let point = straightened_point(e, this, segment, at(0x108));
                let second_near = fn_006f1e90(e, this, point, near_second);
                let mut far = segment.wrapping_add(1);
                while far < e.call(ARRAY_COUNT, &args![path]).u32() {
                    let point = straightened_point(e, this, far, at(0x114));
                    let node = avoid_node(e, avoid, node_index);
                    if fn_006f1bc0(e, Ptr::new(node), point) == 0 {
                        break;
                    }
                    far += 1;
                }
                if far >= e.call(ARRAY_COUNT, &args![path]).u32() {
                    segment += 1;
                    continue;
                }
                let (far_first, far_second) = (at(0xa0), at(0x94));
                if !tangents(e, this, avoid, node_index, far, at(0x120), far_first) {
                    segment += 1;
                    continue;
                }
                let point = straightened_point(e, this, far, at(0x12c));
                let first_far = fn_006f1e90(e, this, point, far_first);
                let point = straightened_point(e, this, far, at(0x138));
                let second_far = fn_006f1e90(e, this, point, far_second);
                let first_side = first_near != 0 && second_far != 0;
                let second_side = second_near != 0 && first_far != 0;
                let chosen = if first_side && !second_side {
                    Some((0u32, 1u32))
                } else if second_side && !first_side {
                    Some((1, 0))
                } else if first_side && second_side {
                    let gap = e
                        .call(POINT_SUBTRACT, &args![near_first, at(0x144), far_second])
                        .u32();
                    let first_length = x87_store32(e.call(POINT_LENGTH_SQUARED, &args![gap]).st0);
                    let gap = e
                        .call(POINT_SUBTRACT, &args![near_second, at(0x150), far_first])
                        .u32();
                    let second_length = x87_store32(e.call(POINT_LENGTH_SQUARED, &args![gap]).st0);
                    if x87_load32(second_length) > x87_load32(first_length) {
                        Some((0, 1))
                    } else {
                        Some((1, 0))
                    }
                } else {
                    None
                };
                let Some((near_pick, far_pick)) = chosen else {
                    // `s = t - 1`, then the loop's `s + 1`.
                    segment = far;
                    continue;
                };
                if far > segment.wrapping_add(1) {
                    let removed = far.wrapping_sub(segment).wrapping_sub(1);
                    e.call(
                        STRAIGHTENED_REMOVE,
                        &args![path, segment.wrapping_add(1), removed, 0u32],
                    );
                }
                for (point, slot, insert_at) in [
                    (
                        near_first + near_pick * 0xc,
                        at(0x178),
                        segment.wrapping_add(1),
                    ),
                    (
                        far_first + far_pick * 0xc,
                        at(0x1a0),
                        segment.wrapping_add(2),
                    ),
                ] {
                    let location = e.call(STRAIGHTENED_AT, &args![path, segment]).u32();
                    let made = e
                        .call(
                            LOCATION_FROM_POINT_AND_LOCATION,
                            &args![slot, point, location],
                        )
                        .u32();
                    e.call(STRAIGHTENED_INSERT, &args![path, insert_at, made]);
                    e.call(LOCATION_DESTRUCT, &args![slot]);
                }
                break;
            }
            node_index += 1;
        }
        e.call(AVOID_DESTRUCT, &args![avoid]);
    });
}

/// The solution's helpers for [`fn_006f1d30`]: its node count (`008b6800`, thiscall on
/// `pSolution`), node `index` (`006e7970`, thiscall: index), a node's location
/// (`00717e50`, thiscall; the map's name for the body is a folded one) and its direction
/// set (`006b0570`, thiscall: the direction's address); `NiPoint3::UnitizeGetLength`
/// (`00457910`, engine map; ST0) and the cdecl two-float function `0040ebd0` (ST0).
const SOLUTION_COUNT: u32 = 0x008b_6800;
const SOLUTION_NODE: u32 = 0x006e_7970;
const NODE_LOCATION: u32 = 0x0071_7e50;
const NODE_SET_DIRECTION: u32 = 0x006b_0570;
const UNITIZE_GET_LENGTH: u32 = 0x0045_7910;
const CURVE_0040EBD0: u32 = 0x0040_ebd0;
/// The cdecl triangle test `006b9cf0` (three corners, the point, an out word and two
/// floats; `AL`; unnamed in the engine map) and the `float` tolerance it is
/// given twice (`0106b158`, read at run time).
const POINT_IN_TRIANGLE_006B9CF0: u32 = 0x006b_9cf0;
const TRIANGLE_TOLERANCE: u32 = 0x0106_b158;

// Translated from 006f1bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall` on an avoid node, `RET 4`): whether `point` lies inside
/// the node's circle: its squared radius (x87 `FLD`/`FMUL` of `fRadius` at +0xC) above the
/// squared length (`004a7290`) of `point - Center`.
pub fn fn_006f1bc0(e: &mut Engine, this: Ptr, point: u32) -> u8 {
    e.with_stack(0xc, |e, out| {
        let offset = e.call(POINT_SUBTRACT, &args![point, out, this]).u32();
        let length = e.call(POINT_LENGTH_SQUARED, &args![offset]).st0;
        let radius = e.mem.u32(this.addr() + 0xc);
        let squared = x87_mul(x87_load32(radius), x87_m32(radius));
        u8::from(squared > length)
    })
}

/// A pathing location at `point` on the mesh of the triangle `get` answers, added to the
/// solution ([`fn_006ed7a0`]) and destroyed.
fn add_end_location(e: &mut Engine, this: Ptr, slot: u32, point: u32, get: u32) {
    let handle = triangle_handle(e, this, get);
    e.call(LOCATION_CONSTRUCT, &args![slot, point, handle]);
    fn_006ed7a0(e, this, slot);
    e.call(LOCATION_DESTRUCT, &args![slot]);
}

// Translated from 006f1c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f1c10 - the compiler's exception frame (FS:[0] unwind states) and the security-cookie word it pushes are not translated
/// Unnamed in the map (`thiscall`, `RET 8`): copies the straightened path into the solution
/// ([`fn_006ed7a0`]): the start (on the first triangle) when `with_start` is set, the inner
/// straightened locations, and the goal (on the last triangle) when `with_goal` is set.
pub fn fn_006f1c10(e: &mut Engine, this: Ptr, with_start: u8, with_goal: u8) {
    let base = this.addr();
    let path = base + STRAIGHTENED_PATH;
    // The frame from EBP - 0x64; `at(k)` is EBP - k.
    e.with_stack(0x64, |e, frame| {
        let at = |k: u32| frame.addr() + 0x64 - k;
        if with_start != 0 {
            add_end_location(e, this, at(0x34), base + START_LOCATION, FIRST_TRIANGLE);
        }
        let mut index = 1u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![path]).u32();
            if index >= count.wrapping_sub(1) {
                break;
            }
            let location = e.call(STRAIGHTENED_AT, &args![path, index]).u32();
            fn_006ed7a0(e, this, location);
            index += 1;
        }
        if with_goal != 0 {
            add_end_location(e, this, at(0x60), base + GOAL_LOCATION, LAST_TRIANGLE);
        }
    });
}

/// The point of solution node `index` (`006e7970`, `00717e50`, `005c3420` into `out`).
fn solution_point(e: &mut Engine, this: Ptr, index: u32, out: u32) -> u32 {
    let solution = e.mem.u32(this.addr() + SOLUTION);
    let node = e.call(SOLUTION_NODE, &args![solution, index]).u32();
    let location = e.call(NODE_LOCATION, &args![node]).u32();
    e.call(LOCATION_POINT, &args![location, out]).u32()
}

// Translated from 006f1d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, plain `RET`): gives each inner solution node a direction:
/// the unit vectors to the next node and from the previous one (`NiPoint3::UnitizeGetLength`,
/// lengths kept as `float`s), their sum times `0040ebd0(next length, previous length)`
/// times `01016248` (`006b0570`).
pub fn fn_006f1d30(e: &mut Engine, this: Ptr) {
    // The frame from EBP - 0x80; `at(k)` is EBP - k.
    e.with_stack(0x80, |e, frame| {
        let at = |k: u32| frame.addr() + 0x80 - k;
        let (ahead, behind) = (at(0x10), at(0x24));
        let mut index = 1u32;
        loop {
            let solution = e.mem.u32(this.addr() + SOLUTION);
            let count = e.call(SOLUTION_COUNT, &args![solution]).u32();
            if index >= count.wrapping_sub(1) {
                break;
            }
            let here = solution_point(e, this, index, at(0x40));
            let next = solution_point(e, this, index.wrapping_add(1), at(0x34));
            e.call(POINT_SUBTRACT, &args![next, ahead, here]);
            let ahead_length = x87_store32(e.call(UNITIZE_GET_LENGTH, &args![ahead]).st0);
            e.mem.set_u32(at(0x14), ahead_length);
            let previous = solution_point(e, this, index.wrapping_sub(1), at(0x58));
            let here = solution_point(e, this, index, at(0x4c));
            e.call(POINT_SUBTRACT, &args![here, behind, previous]);
            let behind_length = x87_store32(e.call(UNITIZE_GET_LENGTH, &args![behind]).st0);
            e.mem.set_u32(at(0x28), behind_length);
            let factor = x87_store32(
                e.call(
                    CURVE_0040EBD0,
                    &args![x87_quiet_bits(ahead_length), x87_quiet_bits(behind_length)],
                )
                .st0,
            );
            e.mem.set_u32(at(0x18), factor);
            let half = x87_quiet_bits(e.mem.u32(MIDPOINT_FACTOR));
            let sum = e.call(POINT_ADD, &args![ahead, at(0x64), behind]).u32();
            let scaled = e
                .call(POINT_SCALE, &args![sum, at(0x70), x87_quiet_bits(factor)])
                .u32();
            let direction = e.call(POINT_SCALE, &args![scaled, at(0x7c), half]).u32();
            let solution = e.mem.u32(this.addr() + SOLUTION);
            let node = e.call(SOLUTION_NODE, &args![solution, index]).u32();
            e.call(NODE_SET_DIRECTION, &args![node, direction]);
            index += 1;
        }
    });
}

// Translated from 006f1e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): whether `to` lies on the path's corridor
/// ([`fn_006f1fc0`]) and the segment from `from` to `to` crosses no bounds edge
/// ([`fn_006f1ed0`]).
pub fn fn_006f1e90(e: &mut Engine, this: Ptr, from: u32, to: u32) -> u8 {
    if fn_006f1fc0(e, this, to) != 0 && fn_006f1ed0(e, this, from, to) != 0 {
        1
    } else {
        0
    }
}

// Translated from 006f1ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 8`): whether the segment from `from` to `to`
/// crosses none of the left, then right, bounds edges' shrunk segments (`00696540`).
pub fn fn_006f1ed0(e: &mut Engine, this: Ptr, from: u32, to: u32) -> u8 {
    for edges in [LEFT_EDGES, RIGHT_EDGES] {
        let array = this.addr() + edges;
        let mut index = 0u32;
        while index < e.call(ARRAY_COUNT, &args![array]).u32() {
            let at = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            let end = e.mem.u32(e.mem.u32(at).wrapping_add(4)).wrapping_add(0xc);
            let at = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            let start = e.mem.u32(e.mem.u32(at)).wrapping_add(0xc);
            if e.call(SEGMENT_TEST_00696540, &args![from, to, start, end])
                .u8()
                != 0
            {
                return 0;
            }
            index += 1;
        }
    }
    1
}

/// `006b9cf0(a, b, c, point, out, tolerance, tolerance)`.
fn in_triangle(e: &mut Engine, corners: [u32; 3], point: u32, out: u32) -> bool {
    let tolerance = x87_quiet_bits(e.mem.u32(TRIANGLE_TOLERANCE));
    e.call(
        POINT_IN_TRIANGLE_006B9CF0,
        &args![corners[0], corners[1], corners[2], point, out, tolerance, tolerance],
    )
    .u8()
        != 0
}

/// The shrunk location of vertex word `side` (0 left, 4 right) of path edge `index`.
fn edge_shrunk(e: &mut Engine, this: Ptr, index: u32, side: u32) -> u32 {
    let edge = path_edge(e, this, index);
    e.mem.u32(edge.wrapping_add(side)).wrapping_add(0xc)
}

// Translated from 006f1fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`): whether `point` lies on the path's corridor:
/// true with no path edges (`0076b610`); otherwise true when one of the corridor's
/// triangles holds it (`006b9cf0` with tolerance `0106b158` twice): the start with the
/// first edge's right and left shrunk locations, each pair of path edges sharing their left
/// vertex (with the next edge's right) or, failing that, their right vertex (with the next
/// edge's left), and the goal with the last edge's left and right.
pub fn fn_006f1fc0(e: &mut Engine, this: Ptr, point: u32) -> u8 {
    let base = this.addr();
    let edges = base + PATH_EDGES;
    if e.call(ARRAY_IS_EMPTY, &args![edges]).u8() != 0 {
        return 1;
    }
    e.with_stack(4, |e, out| {
        let out = out.addr();
        let left = edge_shrunk(e, this, 0, 0);
        let right = edge_shrunk(e, this, 0, 4);
        if in_triangle(e, [base + START_LOCATION, right, left], point, out) {
            return 1;
        }
        let mut index = 0u32;
        loop {
            let count = e.call(ARRAY_COUNT, &args![edges]).u32();
            if index >= count.wrapping_sub(1) {
                break;
            }
            let this_edge = path_edge(e, this, index);
            let next_edge = path_edge(e, this, index.wrapping_add(1));
            if e.mem.u32(this_edge) == e.mem.u32(next_edge) {
                let next_right = edge_shrunk(e, this, index.wrapping_add(1), 4);
                let this_right = edge_shrunk(e, this, index, 4);
                let this_left = edge_shrunk(e, this, index, 0);
                if in_triangle(e, [this_left, this_right, next_right], point, out) {
                    return 1;
                }
            } else {
                let this_edge = path_edge(e, this, index);
                let next_edge = path_edge(e, this, index.wrapping_add(1));
                if e.mem.u32(this_edge.wrapping_add(4)) == e.mem.u32(next_edge.wrapping_add(4)) {
                    let next_left = edge_shrunk(e, this, index.wrapping_add(1), 0);
                    let this_right = edge_shrunk(e, this, index, 4);
                    let this_left = edge_shrunk(e, this, index, 0);
                    if in_triangle(e, [this_left, this_right, next_left], point, out) {
                        return 1;
                    }
                }
            }
            index += 1;
        }
        let last = last_in(e, edges);
        let last_right = e.mem.u32(last.wrapping_add(4)).wrapping_add(0xc);
        let last = last_in(e, edges);
        let last_left = e.mem.u32(last).wrapping_add(0xc);
        u8::from(in_triangle(
            e,
            [base + GOAL_LOCATION, last_left, last_right],
            point,
            out,
        ))
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_d510, fn_006ed510(Ptr)),
        entry!(0x006f_2270, fn_006f2270(Ptr) -> u32),
        entry!(
            0x006e_d530,
            fn_006ed530(Ptr, u32, u32, u32, Ptr, Ptr, u32, u32, u8) -> u8
        ),
        entry!(0x006e_d730, fn_006ed730(Ptr, u32, u32)),
        entry!(0x006e_d7a0, fn_006ed7a0(Ptr, u32)),
        entry!(0x006e_d7d0, fn_006ed7d0(Ptr, u32, u32, u32, u32) -> u8),
        entry!(0x006e_d850, fn_006ed850(Ptr)),
        entry!(0x006e_dbc0, fn_006edbc0(Ptr)),
        entry!(0x006e_dc40, fn_006edc40(Ptr) -> u32),
        entry!(0x006e_dce0, fn_006edce0(Ptr, u32) -> Ptr),
        entry!(0x006e_dd30, fn_006edd30(Ptr, u32, u32)),
        entry!(0x006e_dfc0, fn_006edfc0(Ptr) -> u32),
        entry!(0x006e_e050, fn_006ee050(Ptr) -> Ptr),
        entry!(0x006e_e070, fn_006ee070(Ptr) -> u32),
        entry!(0x006e_e100, fn_006ee100(Ptr) -> Ptr),
        entry!(0x006e_e150, fn_006ee150(Ptr, u32, u32)),
        entry!(0x006e_e390, fn_006ee390(Ptr, u32, u32)),
        entry!(0x006e_e5d0, fn_006ee5d0(Ptr)),
        entry!(0x006e_e740, fn_006ee740(Ptr, u32, u8, Ptr, Ptr) -> u32),
        entry!(0x006e_ec00, fn_006eec00(Ptr, u32, u32, u8)),
        entry!(0x006e_ecc0, fn_006eecc0(Ptr, u32, u32)),
        entry!(0x006e_ed30, fn_006eed30(Ptr, u32, u8) -> u32),
        entry!(0x006e_ef80, fn_006eef80(Ptr)),
        entry!(0x006e_f0c0, fn_006ef0c0(Ptr, u32, f32, u8) -> u32),
        entry!(0x006e_f920, fn_006ef920(Ptr) -> u8),
        entry!(0x006e_fd50, fn_006efd50(Ptr) -> Ptr),
        entry!(0x006e_fd80, fn_006efd80(Ptr, u32, u32, u8) -> u8),
        entry!(
            0x006f_0050,
            fn_006f0050(Ptr, Ptr, Ptr, u32, u32, Ptr, Ptr, Ptr) -> u8
        ),
        entry!(0x006f_02e0, fn_006f02e0(Ptr)),
        entry!(0x006f_03a0, fn_006f03a0(Ptr)),
        entry!(0x006f_0450, fn_006f0450(Ptr)),
        entry!(0x006f_0d20, fn_006f0d20(Ptr)),
        entry!(0x006f_10d0, fn_006f10d0(Ptr)),
        entry!(0x006f_1440, fn_006f1440(Ptr)),
        entry!(0x006f_1bc0, fn_006f1bc0(Ptr, u32) -> u8),
        entry!(0x006f_1c10, fn_006f1c10(Ptr, u8, u8)),
        entry!(0x006f_1d30, fn_006f1d30(Ptr)),
        entry!(0x006f_1e90, fn_006f1e90(Ptr, u32, u32) -> u8),
        entry!(0x006f_1ed0, fn_006f1ed0(Ptr, u32, u32) -> u8),
        entry!(0x006f_1fc0, fn_006f1fc0(Ptr, u32) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006ed510_passes_this_plus_2c_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_006F25D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_d510, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_006F25D0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x2c]);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_006f2270_passes_the_word_minus_one() {
        let mut e = Engine::new();
        e.register(SET_INDEX, |_, _| Ret::default());
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        e.call(0x006f_2270, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SET_INDEX, vec![this, u32::MAX])]);
    }

    #[test]
    fn fn_006f2270_answers_the_element_address() {
        let mut e = Engine::new();
        e.register(SET_INDEX, |_, a| (a[0] + 0x100 + 4 * a[1]).into_ret());
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 3);
        assert_eq!(e.call(0x006f_2270, &args![this]).u32(), this + 0x108);
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    // A model of the smoother's world: word arrays (buffer +4, count +8), search records
    // of 0xC bytes (mesh word, triangle +4, edge word +8), points of three floats.

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn make_array(e: &mut Engine, at: u32) {
        let buffer = e.mem.alloc(4 * 64);
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, 0);
    }

    fn push(e: &mut Engine, array: u32, value: u32) {
        let count = e.mem.u32(array + 8);
        let buffer = e.mem.u32(array + 4);
        e.mem.set_u32(buffer + 4 * count, value);
        e.mem.set_u32(array + 8, count + 1);
    }

    fn words(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    fn get(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set(e: &mut Engine, at: u32, p: [f32; 3]) {
        for (i, v) in p.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn point(e: &mut Engine, p: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0xc);
        set(e, at, p);
        at
    }

    fn binary(e: &mut Engine, a: u32, b: u32, out: u32, f: fn(f32, f32) -> f32) -> Ret {
        let (x, y) = (get(e, a), get(e, b));
        set(e, out, [f(x[0], y[0]), f(x[1], y[1]), f(x[2], y[2])]);
        out.into_ret()
    }

    /// The test points of edge words `16 * first + second`.
    const POINTS: [[f32; 3]; 6] = [
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [2.0, 0.0, 0.0],
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x0002_0000);
        e.map(0x0106_0000, 0x0001_0000);
        e.map(0x010a_2000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        for (address, value) in [
            (MIDPOINT_FACTOR, 0.5f32),
            (SPREAD_FACTOR, 0.25),
            (NEAR_FACTOR, 0.75),
            (INSERT_PUSH_FACTOR, 0.5),
            (TRIANGLE_TOLERANCE, 0.01),
        ] {
            e.mem.set_f32(address, value);
        }
        for (address, value) in [
            (DOUBLE_ZERO, 0.0f64),
            (STRAIGHT_LIMIT, 0.8),
            (BEND_LIMIT, 0.2),
            (ANGLE_OFFSET, std::f64::consts::PI),
            (ANGLE_FACTOR, 0.5),
            (RADIUS_FACTOR, 1.5),
        ] {
            e.set_global(address, value);
        }
        set(&mut e, SIDE_AXIS, [0.0, 0.0, 1.0]);
        // Arrays of words.
        e.register(ARRAY_COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        for address in [ARRAY_ELEMENT, BOUNDS_EDGE_AT] {
            e.register(address, |e, a| (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret());
        }
        e.register(ARRAY_FIRST, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        e.register(ARRAY_ADD, |e, a| {
            let value = e.mem.u32(a[1]);
            push(e, a[0], value);
            Ret::default()
        });
        e.register(ARRAY_INSERT, |e, a| {
            let value = e.mem.u32(a[2]);
            let buffer = e.mem.u32(a[0] + 4);
            let count = e.mem.u32(a[0] + 8);
            let mut i = count;
            while i > a[1] {
                let moved = e.mem.u32(buffer + 4 * (i - 1));
                e.mem.set_u32(buffer + 4 * i, moved);
                i -= 1;
            }
            e.mem.set_u32(buffer + 4 * a[1], value);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE, |_, _| Ret::default());
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        // Search arrays of 0xC-byte records.
        for address in [EDGE_AT, TRIANGLE_AT, TRIANGLE_AT_006B3160] {
            e.register(address, |e, a| {
                (e.mem.u32(a[0] + 4) + 0xc * a[1]).into_ret()
            });
        }
        e.register(FIRST_TRIANGLE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for address in [LAST_TRIANGLE, LAST_EDGE] {
            e.register(address, |e, a| {
                (e.mem.u32(a[0] + 4) + 0xc * (e.mem.u32(a[0] + 8) - 1)).into_ret()
            });
        }
        e.register(NAV_MESH_HANDLE, |_, a| (a[0] + 0x1000).into_ret());
        e.register(EDGE_POINTS, |e, a| {
            let (first, second) = ((a[2] >> 4) as usize, (a[2] & 15) as usize);
            set(e, a[3], POINTS[first]);
            set(e, a[4], POINTS[second]);
            Ret::default()
        });
        e.register(TRIANGLE_NORMAL, |e, a| {
            set(e, a[1], [0.0, 0.0, 1.0]);
            Ret::default()
        });
        e.register(MESH_TRIANGLE_005582F0, |_, _| 0x7777u32.into_ret());
        e.register(EDGE_IS_OPEN_0068F200, |_, _| 1u32.into_ret());
        // Scrap heap and members.
        e.register(SCRAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(PLACE, |_, a| a[1].into_ret());
        e.register(HANDLE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SCRAP_ARRAY_CONSTRUCT, |e, a| {
            make_array(e, a[0]);
            a[0].into_ret()
        });
        e.register(HANDLE_COPY, |e, a| {
            let (lo, hi) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], lo);
            e.mem.set_u32(a[0] + 4, hi);
            a[0].into_ret()
        });
        e.register(HANDLE_ASSIGN, |e, a| {
            let (lo, hi) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            e.mem.set_u32(a[0], lo);
            e.mem.set_u32(a[0] + 4, hi);
            a[0].into_ret()
        });
        // Points.
        e.register(POINT_DEFAULT, |_, a| a[0].into_ret());
        e.register(POINT_EQUAL, |e, a| {
            (get(e, a[0]) == get(e, a[1])).into_ret()
        });
        e.register(POINT_SUBTRACT, |e, a| {
            binary(e, a[0], a[2], a[1], |x, y| x - y)
        });
        e.register(POINT_ADD, |e, a| binary(e, a[0], a[2], a[1], |x, y| x + y));
        e.register(POINT_ADD_ASSIGN, |e, a| {
            binary(e, a[0], a[1], a[0], |x, y| x + y)
        });
        e.register(POINT_SCALE, |e, a| {
            let (p, f) = (get(e, a[0]), f32::from_bits(a[2]));
            set(e, a[1], [p[0] * f, p[1] * f, p[2] * f]);
            a[1].into_ret()
        });
        e.register(SCALE_POINT, |e, a| {
            let (p, f) = (get(e, a[2]), f32::from_bits(a[1]));
            set(e, a[0], [p[0] * f, p[1] * f, p[2] * f]);
            a[0].into_ret()
        });
        e.register(POINT_DIVIDE, |e, a| {
            let (p, f) = (get(e, a[0]), f32::from_bits(a[1]));
            set(e, a[0], [p[0] / f, p[1] / f, p[2] / f]);
            a[0].into_ret()
        });
        e.register(POINT_NEGATE, |e, a| {
            let p = get(e, a[0]);
            set(e, a[1], [-p[0], -p[1], -p[2]]);
            a[1].into_ret()
        });
        e.register(POINT_DOT, |e, a| {
            let (p, q) = (get(e, a[0]), get(e, a[1]));
            f64::from(p[0] * q[0] + p[1] * q[1] + p[2] * q[2]).into_ret()
        });
        e.register(POINT_OP_004B3800, |e, a| {
            let (p, q) = (get(e, a[0]), get(e, a[2]));
            set(
                e,
                a[1],
                [
                    p[1] * q[2] - p[2] * q[1],
                    p[2] * q[0] - p[0] * q[2],
                    p[0] * q[1] - p[1] * q[0],
                ],
            );
            a[1].into_ret()
        });
        e.register(POINT_NORMALIZE, |e, a| {
            let p = get(e, a[0]);
            let l = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            if l > 0.0 {
                set(e, a[0], [p[0] / l, p[1] / l, p[2] / l]);
            }
            Ret::default()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| {
            let p = get(e, a[0]);
            f64::from(p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).into_ret()
        });
        for address in [POINT_LENGTH, POINT_LENGTH_XY] {
            e.register(address, |e, a| {
                let p = get(e, a[0]);
                f64::from((p[0] * p[0] + p[1] * p[1]).sqrt()).into_ret()
            });
        }
        e.register(POINT_FROM_FLOATS, |e, a| {
            set(
                e,
                a[0],
                [
                    f32::from_bits(a[1]),
                    f32::from_bits(a[2]),
                    f32::from_bits(a[3]),
                ],
            );
            a[0].into_ret()
        });
        e
    }

    /// A smoother with empty arrays, a request, triangles and edges.
    fn smoother(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0xc0);
        for array in [
            LEFT_VERTS,
            RIGHT_VERTS,
            LEFT_EDGES,
            RIGHT_EDGES,
            PATH_EDGES,
            STRAIGHTENED_PATH,
        ] {
            make_array(e, this + array);
        }
        for field in [TRIANGLES, EDGES] {
            let array = e.mem.alloc(0x10);
            let buffer = e.mem.alloc(0xc * 16);
            e.mem.set_u32(array + 4, buffer);
            e.mem.set_u32(this + field, array);
        }
        e.mem.set_u32(this + SOLUTION, 0x5150);
        set(e, this + START_LOCATION, [-1.0, 0.0, 0.0]);
        set(e, this + GOAL_LOCATION, [3.0, 0.0, 0.0]);
        Ptr::new(this)
    }

    /// Adds a search record (mesh, triangle, edge word) to the search array at `field`.
    fn add_record(e: &mut Engine, this: Ptr, field: u32, mesh: u32, triangle: u16, word: u32) {
        let array = e.mem.u32(this.addr() + field);
        let count = e.mem.u32(array + 8);
        let record = e.mem.u32(array + 4) + 0xc * count;
        e.mem.set_u32(record, mesh);
        e.mem.set_u16(record + 4, triangle);
        e.mem.set_u32(record + 8, word);
        e.mem.set_u32(array + 8, count + 1);
    }

    #[test]
    fn solution_add_and_location_round_trip() {
        let mut e = engine();
        let this = smoother(&mut e);
        e.register(SOLUTION_ADD, |_, _| Ret::default());
        e.register(LOCATION_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCATION_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_d7a0, &args![this, 0x4444u32]);
        assert_eq!(
            calls_to(&e, SOLUTION_ADD),
            vec![vec![0x5150, 0x4444, ZERO_POINT]]
        );
        e.call(0x006e_d730, &args![this, 0x6000u32, 0x77u32]);
        let made = calls_to(&e, LOCATION_CONSTRUCT);
        assert_eq!(made[0][1..], [0x6000, 0x77]);
        assert_eq!(
            calls_to(&e, SOLUTION_ADD)[1],
            vec![0x5150, made[0][0], ZERO_POINT]
        );
        assert_eq!(calls_to(&e, LOCATION_DESTRUCT), vec![vec![made[0][0]]]);
    }

    #[test]
    fn one_triangle_only_adds_start_and_goal() {
        let mut e = engine();
        let this = smoother(&mut e);
        add_record(&mut e, this, TRIANGLES, 0x300, 4, 0);
        e.register(REQUEST_RADIUS, |_, _| 0.5f64.into_ret());
        e.register(SOLUTION_ADD, |_, _| Ret::default());
        e.register(LOCATION_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCATION_DESTRUCT, |_, _| Ret::default());
        let triangles = e.mem.u32(this.addr() + TRIANGLES);
        let edges = e.mem.u32(this.addr() + EDGES);
        let start = point(&mut e, [1.0, 2.0, 3.0]);
        let goal = point(&mut e, [4.0, 5.0, 6.0]);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x006e_d530,
                &args![this, 0x99u32, triangles, edges, start, goal, 0x5150u32, 0u32, 0u32],
            )
            .u8();
        assert_eq!(result, 1);
        assert_eq!(e.mem.f32(this.addr() + SMOOTHING_RADIUS), 0.5);
        assert_eq!(get(&e, this.addr() + GOAL_LOCATION), [4.0, 5.0, 6.0]);
        assert_eq!(e.mem.u32(this.addr() + 4), 0x99);
        let made = calls_to(&e, LOCATION_CONSTRUCT);
        assert_eq!(made.len(), 2);
        assert_eq!(made[0][1..], [start, 0x1300]);
        assert_eq!(made[1][1..], [goal, 0x1300]);
    }

    #[test]
    fn segment_checks_stop_at_the_first_failure() {
        let mut e = engine();
        let this = smoother(&mut e);
        for _ in 0..3 {
            let edge = e.mem.alloc(0x58);
            push(&mut e, this.addr() + PATH_EDGES, edge);
        }
        let second = words(&e, this.addr() + PATH_EDGES)[1];
        e.register_double(SEGMENT_TEST_00696540, move |_, a| {
            (a[0] != second + 0x20).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x006e_d7d0, &args![this, 0u32, 2u32, 7u32, 8u32])
                .u8(),
            0
        );
        let tests = calls_to(&e, SEGMENT_TEST_00696540);
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[1], vec![second + 0x20, second + 0x2c, 7, 8]);
        assert_eq!(
            e.call(0x006e_d7d0, &args![this, 2u32, 2u32, 7u32, 8u32])
                .u8(),
            1
        );
        assert_eq!(
            e.call(0x006e_d7d0, &args![this, 3u32, 2u32, 7u32, 8u32])
                .u8(),
            1
        );
    }

    /// Builds the lists of three search edges: p1-p2, p1-p3, p4-p3.
    fn built(e: &mut Engine) -> Ptr {
        let this = smoother(e);
        for (word, triangle) in [(0x12u32, 1u16), (0x13, 2), (0x43, 3)] {
            add_record(e, this, EDGES, 0x200, triangle, word);
        }
        for triangle in 0..4u16 {
            add_record(e, this, TRIANGLES, 0x300, triangle, 0);
        }
        e.call(0x006e_d850, &args![this]);
        this
    }

    fn location_of(e: &Engine, vertex: u32) -> [f32; 3] {
        get(e, vertex)
    }

    #[test]
    fn vertex_lists_follow_the_shared_points() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let this = built(&mut e);
        let base = this.addr();
        let left = words(&e, base + LEFT_VERTS);
        let right = words(&e, base + RIGHT_VERTS);
        let locations = |list: &[u32]| list.iter().map(|v| location_of(&e, *v)).collect::<Vec<_>>();
        assert_eq!(
            locations(&left),
            vec![[-1.0, 0.0, 0.0], POINTS[1], POINTS[4], [3.0, 0.0, 0.0]]
        );
        assert_eq!(
            locations(&right),
            vec![[-1.0, 0.0, 0.0], POINTS[2], POINTS[3], [3.0, 0.0, 0.0]]
        );
        assert_eq!(words(&e, base + LEFT_EDGES).len(), 3);
        assert_eq!(words(&e, base + RIGHT_EDGES).len(), 3);
        let paths = words(&e, base + PATH_EDGES);
        assert_eq!(paths.len(), 3);
        // The second path edge runs from the shared left vertex to the new right one.
        assert_eq!(e.mem.u32(paths[1]), left[1]);
        assert_eq!(e.mem.u32(paths[1] + 4), right[2]);
        assert_eq!(e.mem.u32(paths[1] + 0x48), 1);
        assert_eq!(e.mem.f32(paths[1] + 0x38), 1.0);
        // The third one from the new left vertex to the shared right one.
        assert_eq!(e.mem.u32(paths[2]), left[2]);
        assert_eq!(e.mem.u32(paths[2] + 4), right[2]);
        // Bounds edges: the right edge of the second step has index word (0x13 + 2) % 3,
        // the left one of the third step (0x43 + 1) % 3.
        let right_edges = words(&e, base + RIGHT_EDGES);
        assert_eq!(e.mem.u32(right_edges[1] + 0x10), 0);
        assert_eq!(e.mem.u8(right_edges[1] + 0x14), 1);
        let left_edges = words(&e, base + LEFT_EDGES);
        assert_eq!(e.mem.u32(left_edges[1] + 0x10), 2);
        assert_eq!(e.mem.u32(left_edges[2] + 0x10), u32::MAX);
        // Every vertex and path edge came from the scrap heap at +0x30.
        let heap = calls_to(&e, SCRAP_ALLOCATE);
        assert!(heap.iter().all(|w| w[0] == base + SCRAP_HEAP));
        assert_eq!(heap.iter().filter(|w| w[1] == 0x40).count(), 6);
        assert_eq!(heap.iter().filter(|w| w[1] == 0x58).count(), 3);
        assert_eq!(heap.iter().filter(|w| w[1] == 0x1c).count(), 6);
        // Each path edge sits in its two vertices' edge lists.
        assert_eq!(words(&e, left[1] + 0x2c), vec![paths[0], paths[1]]);
    }

    #[test]
    fn records_are_constructed_in_place() {
        let mut e = engine();
        let this = smoother(&mut e);
        e.call_log = Some(vec![]);
        let vertex = e.call(0x006e_dc40, &args![this]).u32();
        assert_eq!(e.mem.u32(vertex + 0x2c + 8), 0);
        assert_eq!(
            calls_to(&e, SCRAP_ARRAY_CONSTRUCT),
            vec![vec![vertex + 0x2c, 0, 0, this.addr() + SCRAP_HEAP]]
        );
        let bounds = e.call(0x006e_dfc0, &args![this]).u32();
        assert_eq!(calls_to(&e, HANDLE_CONSTRUCT), vec![vec![bounds + 8]]);
        let path = e.call(0x006e_e070, &args![this]).u32();
        assert_eq!(calls_to(&e, HANDLE_CONSTRUCT)[1], vec![path + 0x3c]);
        let defaults = calls_to(&e, POINT_DEFAULT);
        assert!(defaults.contains(&vec![path + 0x2c]));
        assert!(defaults.contains(&vec![vertex + 0x24]));
        assert_eq!(e.call(0x006e_dce0, &args![vertex, 9u32]).u32(), vertex);
        assert_eq!(e.call(0x006e_e050, &args![bounds]).u32(), bounds);
        assert_eq!(e.call(0x006e_e100, &args![path]).u32(), path);
    }

    #[test]
    fn inserts_renumber_their_arrays() {
        let mut e = engine();
        let this = smoother(&mut e);
        let base = this.addr();
        let mut bounds = vec![];
        for _ in 0..3 {
            let edge = e.mem.alloc(0x1c);
            push(&mut e, base + RIGHT_EDGES, edge);
            bounds.push(edge);
        }
        let new = e.mem.alloc(0x1c);
        e.call(0x006e_ec00, &args![this, 1u32, new, 0u32]);
        let right = words(&e, base + RIGHT_EDGES);
        assert_eq!(right, vec![bounds[0], new, bounds[1], bounds[2]]);
        assert_eq!(e.mem.u32(bounds[2] + 0x18), 3);
        assert_eq!(e.mem.u32(bounds[0] + 0x18), 0);
        let path = e.mem.alloc(0x58);
        e.call(0x006e_ecc0, &args![this, 0u32, path]);
        assert_eq!(words(&e, base + PATH_EDGES), vec![path]);
        assert_eq!(e.mem.u32(path + 0x48), 0);
    }

    #[test]
    fn link_record_constructs_two_points() {
        let mut e = engine();
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006e_fd50, &args![0x9000u32]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![0x900c, 0xc, 2, POINT_DEFAULT]]
        );
    }

    fn shrunk_as_location(e: &mut Engine, this: Ptr) {
        for array in [LEFT_VERTS, RIGHT_VERTS] {
            for vertex in words(e, this.addr() + array) {
                copy_point(e, vertex + 0xc, vertex);
            }
        }
    }

    #[test]
    fn shrinking_pushes_inner_vertices_off_the_path() {
        let mut e = engine();
        let this = built(&mut e);
        let base = this.addr();
        e.mem.set_f32(base + SMOOTHING_RADIUS, 0.5);
        for array in [LEFT_EDGES, RIGHT_EDGES] {
            for edge in words(&e, base + array) {
                e.mem.set_u8(edge + 0x14, 0);
            }
        }
        e.register(ANGLE_004B5570, |_, _| 1.0f64.into_ret());
        e.register(CURVE_0057DD70, |_, _| 2.0f64.into_ret());
        e.register(SEGMENT_TEST_00696540, |_, _| Ret::default());
        e.call(0x006e_ef80, &args![this]);
        let left = words(&e, base + LEFT_VERTS);
        assert_eq!(get(&e, left[0] + 0xc), [-1.0, 0.0, 0.0]);
        assert_eq!(get(&e, left[3] + 0xc), [3.0, 0.0, 0.0]);
        // Vertex 1 (0, 1, 0): next direction (1, 0, 0), its normal (0, -1, 0); offset
        // 2.0 * 0.5: push (1, -0.5, 0).
        assert_eq!(get(&e, left[1] + 0x18), [1.0, -0.5, 0.0]);
        assert_eq!(get(&e, left[1] + 0xc), [1.0, 0.5, 0.0]);
        assert_eq!(left.len(), 4);
    }

    #[test]
    fn vertex_split_moves_half_the_path_edges() {
        let mut e = engine();
        let this = built(&mut e);
        let base = this.addr();
        shrunk_as_location(&mut e, this);
        e.register(SEGMENT_TEST_00696540, |_, _| 1u32.into_ret());
        let left = words(&e, base + LEFT_VERTS);
        let paths = words(&e, base + PATH_EDGES);
        let old_shrunk = point(&mut e, [0.5, 1.0, 0.0]);
        let new_shrunk = point(&mut e, [0.5, 2.0, 0.0]);
        let new = e
            .call(
                0x006e_e740,
                &args![this, 1u32, 1u32, old_shrunk, new_shrunk],
            )
            .u32();
        assert_eq!(get(&e, new), POINTS[1]);
        assert_eq!(get(&e, left[1] + 0x18), [0.5, 0.0, 0.0]);
        assert_eq!(get(&e, new + 0x18), [0.5, 1.0, 0.0]);
        assert_eq!(words(&e, base + LEFT_VERTS)[2], new);
        assert_eq!(words(&e, left[1] + 0x2c), vec![paths[0]]);
        assert_eq!(words(&e, new + 0x2c), vec![paths[1]]);
        assert_eq!(e.mem.u32(paths[1]), new);
        // The new bounds edge (old to new) is left edge 1; the next one starts at `new`.
        let left_edges = words(&e, base + LEFT_EDGES);
        assert_eq!(left_edges.len(), 4);
        assert_eq!(e.mem.u32(left_edges[1]), left[1]);
        assert_eq!(e.mem.u32(left_edges[1] + 4), new);
        assert_eq!(e.mem.u32(left_edges[2]), new);
        // The joining path edge went in at the moved edge's index.
        let now = words(&e, base + PATH_EDGES);
        assert_eq!(now.len(), 4);
        assert_eq!(e.mem.u32(now[1]), new);
        assert_eq!(e.mem.f32(now[1] + 0x38), 1.0);
        assert_eq!(e.mem.u32(now[1] + 0x48), 1);
        assert_eq!(e.mem.u32(now[2] + 0x48), 2);
    }

    #[test]
    fn bounds_edge_split_copies_its_start() {
        let mut e = engine();
        let this = built(&mut e);
        let base = this.addr();
        let right = words(&e, base + RIGHT_VERTS);
        let paths = words(&e, base + PATH_EDGES);
        let copy = e.call(0x006e_ed30, &args![this, 1u32, 0u32]).u32();
        assert_eq!(get(&e, copy), POINTS[2]);
        assert_eq!(words(&e, base + RIGHT_VERTS)[2], copy);
        let right_edges = words(&e, base + RIGHT_EDGES);
        assert_eq!(right_edges.len(), 4);
        assert_eq!(e.mem.u32(right_edges[1]), right[1]);
        assert_eq!(e.mem.u32(right_edges[1] + 4), copy);
        assert_eq!(e.mem.u32(right_edges[2]), copy);
        let now = words(&e, base + PATH_EDGES);
        // From the start vertex's last path edge (index 0): inserted at 1.
        assert_eq!(now[0], paths[0]);
        assert_eq!(e.mem.u32(now[1] + 4), copy);
        assert_eq!(e.mem.u32(now[1]), e.mem.u32(paths[0]));
        assert_eq!(e.mem.u32(now[1] + 0x50), right_edges[1]);
    }

    fn bad_node_doubles(e: &mut Engine) {
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MESH_LOOKUP_0069AD00, |e, a| {
            e.mem.set_u32(a[1], a[0] + 0x10);
            1u32.into_ret()
        });
        e.register(BAD_NODES_ADD, |_, _| Ret::default());
        e.register(LOCATION_CONSTRUCT, |_, _| Ret::default());
        e.register(LOCATION_DESTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn path_check_passes_and_reports_a_bad_edge() {
        let mut e = engine();
        let this = built(&mut e);
        shrunk_as_location(&mut e, this);
        bad_node_doubles(&mut e);
        e.register(SEGMENT_TEST_00696540, |_, _| Ret::default());
        assert_eq!(e.call(0x006e_f920, &args![this]).u8(), 1);
        // Every segment crosses and no probe sees: path edge 1 is reported.
        e.register(SEGMENT_TEST_00696540, |_, _| 1u32.into_ret());
        e.register(CHECK_LINE_OF_SIGHT_TO_POINT, |_, _| Ret::default());
        e.mem.set_u32(this.addr(), 0xbad0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_f920, &args![this]).u8(), 0);
        let added = calls_to(&e, BAD_NODES_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], 0xbad0);
        let record = added[0][1];
        let paths = words(&e, this.addr() + PATH_EDGES);
        assert_eq!(e.mem.u16(record + 4), e.mem.u16(paths[1] + 0x40));
        assert_eq!(e.mem.u32(record), e.mem.u32(paths[1] + 0x3c) + 0x10);
    }

    #[test]
    fn bounds_check_inserts_a_vertex_when_the_probes_see() {
        let mut e = engine();
        let this = built(&mut e);
        shrunk_as_location(&mut e, this);
        bad_node_doubles(&mut e);
        e.register(SEGMENT_TEST_00696540, |_, _| 1u32.into_ret());
        e.register(CHECK_LINE_OF_SIGHT_TO_POINT, |_, _| 1u32.into_ret());
        let base = this.addr();
        let right_edges = words(&e, base + RIGHT_EDGES);
        let left_edges = words(&e, base + LEFT_EDGES);
        let other_end = e.mem.u32(left_edges[0] + 4);
        set(&mut e, other_end + 0xc, [0.0, 2.0, 0.0]);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x006e_fd80,
                &args![this, right_edges[1], left_edges[0], 0u32],
            )
            .u8();
        assert_eq!(result, 1);
        // The split copy of the right edge's start sits at the shrunk location, its own
        // shrunk location pushed on by half the push (0, 1, 0).
        let inserted = words(&e, base + RIGHT_VERTS)[2];
        assert_eq!(get(&e, inserted), [0.0, 2.0, 0.0]);
        assert_eq!(get(&e, inserted + 0xc), [0.0, 2.5, 0.0]);
        assert_eq!(calls_to(&e, LOCATION_DESTRUCT).len(), 3);
        // Without the crossing the answer is whether the other edge crosses.
        e.register(SEGMENT_TEST_00696540, |_, _| Ret::default());
        let result = e
            .call(
                0x006e_fd80,
                &args![this, right_edges[1], left_edges[0], 0u32],
            )
            .u8();
        assert_eq!(result, 1);
    }

    #[test]
    fn shrunk_location_from_the_line_of_sight() {
        let mut e = engine();
        for address in [
            MESH_LOOKUP_00690800,
            MESH_LOOKUP_006B7770,
            LOCATION_FROM_POINT,
            LOCATION_DEFAULT,
            LOCATION_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(CHECK_LINE_OF_SIGHT, |_, _| 1u32.into_ret());
        e.register(CHECK_LINE_OF_SIGHT_TO_POINT, |_, _| 1u32.into_ret());
        e.register(LOCATION_POINT, |e, a| {
            set(e, a[1], [9.0, 9.0, 9.0]);
            a[1].into_ret()
        });
        let vertex = point(&mut e, [1.0, 2.0, 3.0]);
        let shrunk = e.mem.alloc(0xc);
        let push = point(&mut e, [1.0, 0.0, 0.0]);
        let call = |e: &mut Engine| {
            e.call(
                0x006f_0050,
                &args![0u32, vertex, shrunk, 0x300u32, 0u32, push, 0x10u32, 0x20u32],
            )
            .u8()
        };
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e), 0);
        assert_eq!(get(&e, shrunk), [1.0, 2.0, 3.0]);
        assert_eq!(calls_to(&e, LOCATION_DESTRUCT).len(), 3);
        e.register(CHECK_LINE_OF_SIGHT_TO_POINT, |_, _| Ret::default());
        assert_eq!(call(&mut e), 1);
        assert_eq!(get(&e, shrunk), [2.0, 2.0, 3.0]);
        e.register(CHECK_LINE_OF_SIGHT, |_, _| Ret::default());
        assert_eq!(call(&mut e), 1);
        assert_eq!(get(&e, shrunk), [10.0, 9.0, 9.0]);
    }

    fn with_centers(e: &mut Engine) -> Ptr {
        let this = built(e);
        shrunk_as_location(e, this);
        e.call(0x006f_02e0, &args![this]);
        e.call(0x006f_03a0, &args![this]);
        this
    }

    #[test]
    fn path_edges_take_shrunk_ends_and_centres() {
        let mut e = engine();
        let this = with_centers(&mut e);
        let paths = words(&e, this.addr() + PATH_EDGES);
        assert_eq!(get(&e, paths[0] + 0x20), POINTS[1]);
        assert_eq!(get(&e, paths[0] + 0x2c), POINTS[2]);
        assert_eq!(get(&e, paths[0] + 8), [0.0, 0.0, 0.0]);
        assert_eq!(get(&e, paths[2] + 8), [1.0, -0.0, 0.0]);
    }

    #[test]
    fn best_points_come_from_the_averages() {
        let mut e = engine();
        let this = with_centers(&mut e);
        let befores = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = befores.clone();
        e.register_double(BEST_POINT_006BA550, move |e, a| {
            seen.borrow_mut().push(get(e, a[3]));
            binary(e, a[1], a[2], a[0], |x, y| (x + y) / 2.0);
            a[0].into_ret()
        });
        e.register(REQUEST_WEIGHT, |_, _| Ret::default());
        e.register(CURVE_00408840, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_0450, &args![this]);
        let paths = words(&e, this.addr() + PATH_EDGES);
        assert_eq!(get(&e, paths[1] + 0x14), [0.5, 0.0, 0.0]);
        // Edge 0: before averages only the start; after the centres of edges 1 and 2 and
        // the goal.
        assert_eq!(befores.borrow()[0], [-1.0, 0.0, 0.0]);
        assert_eq!(befores.borrow()[1], [-0.5, 0.0, 0.0]);
        // A best point on an end moves a quarter of the way to the other end.
        e.register(BEST_POINT_006BA550, |e, a| {
            binary(e, a[1], a[1], a[0], |x, _| x);
            a[0].into_ret()
        });
        e.call(0x006f_0450, &args![this]);
        assert_eq!(get(&e, paths[0] + 0x14), [0.0, 0.5, 0.0]);
    }

    #[test]
    fn water_runs_mark_their_ends_and_interpolate_heights() {
        let mut e = engine();
        let this = with_centers(&mut e);
        let paths = words(&e, this.addr() + PATH_EDGES);
        set(&mut e, paths[0] + 0x14, [0.0, 0.0, 0.0]);
        set(&mut e, paths[1] + 0x14, [1.0, 0.0, 9.0]);
        set(&mut e, paths[2] + 0x14, [2.0, 0.0, 4.0]);
        e.register(FLAG_ARRAY_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(a[1] + 4);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[1]);
            a[0].into_ret()
        });
        e.register(FLAG_AT, |e, a| (e.mem.u32(a[0] + 4) + a[1]).into_ret());
        e.register(FLAG_FIND, |e, a| {
            let (buffer, count) = (e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8));
            let value = e.mem.u8(a[1]);
            (a[2]..count)
                .find(|i| e.mem.u8(buffer + i) == value)
                .unwrap_or(u32::MAX)
                .into_ret()
        });
        e.register(FLAG_ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(POINTER_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(POINTER_GET, |_, _| 0x300u32.into_ret());
        e.register(MESH_LOOKUP_0069AD00, |_, _| 1u32.into_ret());
        e.register(MESH_TRIANGLE_005582F0, |_, a| a[1].into_ret());
        e.register(TRIANGLE_IS_WATER, |_, a| (a[0] == 2).into_ret());
        e.call(0x006f_0d20, &args![this]);
        assert_eq!(e.mem.u32(paths[0] + 0x54), 1);
        assert_eq!(e.mem.u32(paths[1] + 0x54), 0);
        assert_eq!(e.mem.u32(paths[2] + 0x54), 2);
        assert_eq!(e.mem.f32(paths[1] + 0x1c), 2.0);
    }

    fn straightened_doubles(e: &mut Engine) {
        e.register(LOCATION_CONSTRUCT, |e, a| {
            let p = get(e, a[1]);
            set(e, a[0], p);
            Ret::default()
        });
        e.register(LOCATION_DESTRUCT, |_, _| Ret::default());
        e.register(STRAIGHTENED_ADD, |e, a| {
            let copy = e.mem.alloc(0x28);
            let p = get(e, a[1]);
            set(e, copy, p);
            push(e, a[0], copy);
            Ret::default()
        });
        e.register(STRAIGHTENED_AT, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.u32(buffer + 4 * a[1]).into_ret()
        });
        e.register(LOCATION_POINT, |e, a| {
            let p = get(e, a[0]);
            set(e, a[1], p);
            a[1].into_ret()
        });
    }

    #[test]
    fn straightened_path_keeps_water_edges() {
        let mut e = engine();
        let this = with_centers(&mut e);
        straightened_doubles(&mut e);
        e.register(SEGMENT_TEST_00696540, |_, _| 1u32.into_ret());
        let paths = words(&e, this.addr() + PATH_EDGES);
        set(&mut e, paths[0] + 0x14, [0.0, 0.0, 7.0]);
        e.call(0x006f_10d0, &args![this]);
        let path = this.addr() + STRAIGHTENED_PATH;
        let points: Vec<_> = words(&e, path).iter().map(|p| get(&e, *p)).collect();
        assert_eq!(points, vec![[-1.0, 0.0, 0.0], [3.0, 0.0, 0.0]]);
        e.mem.set_u32(path + 8, 0);
        e.mem.set_u32(paths[1] + 0x54, 1);
        e.call(0x006f_10d0, &args![this]);
        let points: Vec<_> = words(&e, path).iter().map(|p| get(&e, *p)).collect();
        assert_eq!(
            points,
            vec![[-1.0, 0.0, 0.0], [0.0, 0.0, 7.0], [3.0, 0.0, 0.0]]
        );
    }

    #[test]
    fn avoidance_records_obstacles_and_skips_clear_segments() {
        let mut e = engine();
        let this = with_centers(&mut e);
        straightened_doubles(&mut e);
        e.register(OBSTACLES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_1440, &args![this]);
        assert!(calls_to(&e, AVOID_CONSTRUCT).is_empty());
        // One obstacle, a two-point straightened path the circle misses.
        let list = e.mem.alloc(0x10);
        make_array(&mut e, list);
        push(&mut e, list, 0);
        let obstacle = point(&mut e, [5.0, 6.0, 7.0]);
        e.register_double(OBSTACLES, move |_, _| list.into_ret());
        e.register_double(OBSTACLE_AT, move |_, _| obstacle.into_ret());
        e.map(obstacle + 0xc, 0x18);
        e.register(OBSTACLE_RADIUS, |_, _| 2.0f64.into_ret());
        e.register(AVOID_CONSTRUCT, |e, a| {
            make_array(e, a[0]);
            Ret::default()
        });
        e.register(AVOID_ADD, |e, a| {
            let node = e.mem.alloc(0x10);
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(node + 4 * word, value);
            }
            push(e, a[0], node);
            Ret::default()
        });
        e.register(AVOID_AT, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.u32(buffer + 4 * a[1]).into_ret()
        });
        e.register(AVOID_DESTRUCT, |_, _| Ret::default());
        e.register(SEGMENT_CIRCLE_INTERSECTION, |_, _| Ret::default());
        let path = this.addr() + STRAIGHTENED_PATH;
        for p in [[-1.0f32, 0.0, 0.0], [3.0, 0.0, 0.0]] {
            let location = point(&mut e, p);
            push(&mut e, path, location);
        }
        e.call_log = Some(vec![]);
        e.call(0x006f_1440, &args![this]);
        let tests = calls_to(&e, SEGMENT_CIRCLE_INTERSECTION);
        assert_eq!(tests.len(), 1);
        assert_eq!(get(&e, tests[0][2]), [5.0, 6.0, 7.0]);
        assert_eq!(f32::from_bits(tests[0][3]), 2.0);
        assert_eq!(words(&e, path).len(), 2);
        assert_eq!(calls_to(&e, AVOID_DESTRUCT).len(), 1);
    }

    #[test]
    fn circle_test_compares_squared_lengths() {
        let mut e = engine();
        let node = point(&mut e, [0.0, 0.0, 0.0]);
        e.map(node + 0xc, 4);
        e.mem.set_f32(node + 0xc, 2.0);
        let inside = point(&mut e, [1.0, 1.0, 0.0]);
        let outside = point(&mut e, [2.0, 0.0, 0.0]);
        assert_eq!(e.call(0x006f_1bc0, &args![node, inside]).u8(), 1);
        assert_eq!(e.call(0x006f_1bc0, &args![node, outside]).u8(), 0);
    }

    #[test]
    fn solution_gets_the_straightened_locations() {
        let mut e = engine();
        let this = with_centers(&mut e);
        straightened_doubles(&mut e);
        e.register(SOLUTION_ADD, |_, _| Ret::default());
        let path = this.addr() + STRAIGHTENED_PATH;
        for p in [[-1.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [3.0, 0.0, 0.0]] {
            let location = point(&mut e, p);
            push(&mut e, path, location);
        }
        e.call_log = Some(vec![]);
        e.call(0x006f_1c10, &args![this, 1u32, 0u32]);
        let added = calls_to(&e, SOLUTION_ADD);
        assert_eq!(added.len(), 2);
        assert_eq!(get(&e, added[0][1]), [-1.0, 0.0, 0.0]);
        assert_eq!(added[1][1], words(&e, path)[1]);
    }

    #[test]
    fn solution_directions_blend_the_neighbours() {
        let mut e = engine();
        let this = smoother(&mut e);
        let nodes: Vec<u32> = [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]
            .iter()
            .map(|p| point(&mut e, *p))
            .collect();
        let table = e.mem.alloc(0xc);
        for (i, n) in nodes.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *n);
        }
        e.register(SOLUTION_COUNT, |_, _| 3u32.into_ret());
        e.register_double(SOLUTION_NODE, move |e, a| {
            e.mem.u32(table + 4 * a[1]).into_ret()
        });
        e.register(NODE_LOCATION, |_, a| a[0].into_ret());
        e.register(LOCATION_POINT, |e, a| {
            let p = get(e, a[0]);
            set(e, a[1], p);
            a[1].into_ret()
        });
        e.register(UNITIZE_GET_LENGTH, |e, a| {
            let p = get(e, a[0]);
            let l = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            set(e, a[0], [p[0] / l, p[1] / l, p[2] / l]);
            f64::from(l).into_ret()
        });
        e.register(CURVE_0040EBD0, |_, _| 1.0f64.into_ret());
        e.register(NODE_SET_DIRECTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_1d30, &args![this]);
        let set_calls = calls_to(&e, NODE_SET_DIRECTION);
        assert_eq!(set_calls.len(), 1);
        assert_eq!(set_calls[0][0], nodes[1]);
        assert_eq!(get(&e, set_calls[0][1]), [0.5, 0.5, 0.0]);
    }

    #[test]
    fn corridor_and_crossing_tests() {
        let mut e = engine();
        let this = with_centers(&mut e);
        let target = point(&mut e, [0.25, 0.0, 0.0]);
        e.register_double(POINT_IN_TRIANGLE_006B9CF0, move |_, a| {
            (a[3] == target).into_ret()
        });
        e.register(SEGMENT_TEST_00696540, |_, _| Ret::default());
        let other = point(&mut e, [9.0, 9.0, 0.0]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_1fc0, &args![this, target]).u8(), 1);
        let first = &calls_to(&e, POINT_IN_TRIANGLE_006B9CF0)[0];
        assert_eq!(first[0], this.addr() + START_LOCATION);
        assert_eq!(e.call(0x006f_1fc0, &args![this, other]).u8(), 0);
        assert_eq!(e.call(0x006f_1ed0, &args![this, other, target]).u8(), 1);
        assert_eq!(e.call(0x006f_1e90, &args![this, other, target]).u8(), 1);
        e.register(SEGMENT_TEST_00696540, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x006f_1ed0, &args![this, other, target]).u8(), 0);
        assert_eq!(e.call(0x006f_1e90, &args![this, other, target]).u8(), 0);
        assert_eq!(e.call(0x006f_1e90, &args![this, other, other]).u8(), 0);
        // No path edges: always on the corridor.
        let empty = smoother(&mut e);
        assert_eq!(e.call(0x006f_1fc0, &args![empty, other]).u8(), 1);
    }
}
