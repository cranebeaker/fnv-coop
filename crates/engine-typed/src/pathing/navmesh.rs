//! The navmesh: its triangles, vertices and edges, the edge that matches one across meshes, the
//! triangle under a point and the height on it, the grid of triangle lists, and the navmesh infos.

use super::layout::{
    link_field, vertex_field, EdgeInfo, MeshEdge, NavMesh, NavMeshGrid, NavMeshInfo,
    NavMeshTriangle, Point2, SimpleArray,
};
use super::point::{abs, length_squared, subtract};
use super::{
    copy_point, fcom, fld, fld_global, fld_local, fstp, low_word, m32, m32_local, m64_global,
    parity_even, pov, search, through, FAMILY, FLOAT_MAX, ZERO_DOUBLE,
};
use crate::culling::layout::NiPoint3;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{fistp_chop64, x87_add, x87_div, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The float 1/3 the centre scales by (`0102f200`).
const ONE_THIRD: u32 = 0x0102_f200;
/// `-FLT_MAX` (`01015f5c`).
const FLOAT_LOWEST: u32 = 0x0101_5f5c;
/// The doubles 50, -180, 64 and 40 the triangle search compares heights with.
const ABOVE_WATER: u32 = 0x0101_e2c0;
const LOWEST_HEIGHT: u32 = 0x0106_b160;
const HIGHEST_HEIGHT: u32 = 0x0102_40c0;
const CLOSE_HEIGHT: u32 = 0x0103_5810;
/// The floats 180 and 64 the closest-triangle search passes as the heights below and above.
const BELOW_PASSED: u32 = 0x0106_b158;
const ABOVE_PASSED: u32 = 0x0101_e6ec;

/// Callees.
const INFO_TO_MESH: u32 = 0x0068_f2e0;
const MAKE_HOLDER: u32 = 0x0046_4fc0;
const HOLDER_ASSIGN: u32 = 0x0042_f4c0;
const HOLDER_RELEASE: u32 = 0x0042_fa40;
const HOLDER_NEW: u32 = 0x0046_4ff0;
const WATER_HEIGHT: u32 = 0x0054_7250;
const HEIGHT_IN_TRIANGLE: u32 = 0x0069_7810;
const HEIGHT_NEAR_TRIANGLE: u32 = 0x0069_7730;
const FLOOR: u32 = 0x0040_6ce0;
const GRID_RANGE: u32 = 0x0069_6a10;
const CROSS: u32 = 0x0053_d1a0;
const IS_FORMLESS: u32 = 0x0068_f320;
const CELL_WORLD_SPACE: u32 = 0x0048_39c0;
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const MAP_LOOKUP: u32 = 0x0085_3130;
const IS_INTERIOR: u32 = 0x0042_5fd0;
const INTERIOR_INFO: u32 = 0x006b_78b0;
const CELL_Y: u32 = 0x0054_4c60;
const CELL_X: u32 = 0x0054_4c30;
const CELL_KEY: u32 = 0x0058_7410;
const CELL_WORLD: u32 = 0x0054_ddd0;
const KEY_HASH: u32 = 0x0046_4f30;
const PATHING_INFO_MAP: u32 = 0x0045_af00;
/// The pathing singleton's address.
const PATHING: u32 = 0x011d_ea10;
/// The type descriptors of the dynamic cast to a world space.
const CAST_FROM: u32 = 0x0118_3028;
const CAST_TO: u32 = 0x0118_3fd0;

/// No triangle.
pub const NO_TRIANGLE: u16 = 0xffff;

// Typed from 005582f0
/// `NavMesh::GetTriangle` (Xbox PDB, `thiscall(mesh, index)`).
pub fn triangle(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> Ptr<NavMeshTriangle> {
    typed_call!(
        FAMILY,
        0x0055_8dd0,
        triangle_at(e, this.at(NavMesh::triangles), u32::from(index))
    )
}

// Typed from 00558dd0
/// Triangle `index` of a triangle array (`thiscall(array, index)`): 0x10 bytes each, unchecked.
pub fn triangle_at(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> Ptr<NavMeshTriangle> {
    Ptr::new(e.get(this, SimpleArray::data)).index(index, 0x10)
}

// Typed from 005582d0
/// `NavMeshTriangle::GetVertex` (`thiscall(triangle, index)`).
pub fn vertex(e: &mut Engine, this: Ptr<NavMeshTriangle>, index: u32) -> u16 {
    e.get(this, vertex_field(index))
}

// Typed from 00558200
/// `NavMesh::GetTriangleCount` (`thiscall(mesh)`): the triangle array's count.
pub fn triangle_count(e: &mut Engine, this: Ptr<NavMesh>) -> u32 {
    typed_call!(
        leaf::FAMILY,
        0x0044_ddc0,
        leaf::word_at_8(e, this.at(NavMesh::triangles))
    )
}

/// Vertex `index` of triangle `t` of `mesh`, as a pointer to its point.
#[inline(always)]
fn corner(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    t: Ptr<NavMeshTriangle>,
    index: u32,
) -> Ptr<NiPoint3> {
    let v = typed_call!(FAMILY, 0x0055_82d0, vertex(e, t, index));
    typed_call!(FAMILY, 0x0068_f0a0, vertex_point(e, mesh, v))
}

// Typed from 00558220
/// `NavMesh::GetCenter` (`thiscall(mesh, out, triangle)`): the first corner copied, the other two
/// added (`0063c8a0`), the sum scaled by the float 1/3 (`00439180`), copied to `out`.
pub fn center(e: &mut Engine, this: Ptr<NavMesh>, out: Ptr<NiPoint3>, index: u16) -> Ptr<NiPoint3> {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    e.with_stack(0xc, |e, sum| {
        let sum = sum.cast::<NiPoint3>();
        let first = corner(e, this, t, 0);
        copy_point(e, first, sum);
        for i in 1..3 {
            let p = corner(e, this, t, i);
            typed_call!(FAMILY, 0x0063_c8a0, super::point::add_to(e, sum, p));
        }
        let third = fstp(fld_global(e, ONE_THIRD));
        typed_call!(FAMILY, 0x0043_9180, super::point::scale_by(e, sum, third));
        copy_point(e, sum, out);
    });
    out
}

// Typed from 0068f0a0
/// The point of vertex `index` (`thiscall(mesh, index)`): entry `index` of the vertex array.
pub fn vertex_point(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> Ptr<NiPoint3> {
    let p = typed_call!(
        FAMILY,
        0x006a_1440,
        entry_12(e, this.at(NavMesh::vertices), u32::from(index))
    );
    Ptr::new(p)
}

// Typed from 006a1440
/// Entry `index` of an array of 0xc-byte entries (`thiscall(array, index)`), unchecked.
pub fn entry_12(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> u32 {
    index
        .wrapping_mul(0xc)
        .wrapping_add(e.get(this, SimpleArray::data))
}

// Typed from 006a7af0
/// The same entry through [`entry_12`] (`thiscall(array, index)`).
pub fn entry_12_of(e: &mut Engine, this: Ptr<SimpleArray>, index: u32) -> u32 {
    typed_call!(FAMILY, 0x006a_1440, entry_12(e, this, index))
}

// Typed from 00691140
/// `NavMeshTriangle::GetFlag` (`thiscall(triangle, mask)`): whether any bit of `mask` is set.
pub fn has_flags(e: &mut Engine, this: Ptr<NavMeshTriangle>, mask: u32) -> bool {
    mask & e.get(this, NavMeshTriangle::flags) != 0
}

// Typed from 0068f1d0
/// Whether edge `edge` leads to another mesh (`thiscall(triangle, edge)`): flag bit `edge`, the shift
/// count taken mod 32 as `SHL` takes it.
pub fn edge_flag(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u32) -> bool {
    (1u32 << (edge & 31)) & e.get(this, NavMeshTriangle::flags) != 0
}

// Typed from 00693b70
/// Flag 0x40 (`thiscall(triangle)`).
pub fn flag_40(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::flags) & 0x40 != 0
}

// Typed from 00693b90
/// Flag 0x10 (`thiscall(triangle)`), through [`has_flags`].
pub fn flag_10(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    typed_call!(FAMILY, 0x0069_1140, has_flags(e, this, 0x10))
}

// Typed from 006a5ec0
/// Flag 0x200 (`thiscall(triangle)`), through [`has_flags`].
pub fn flag_200(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    typed_call!(FAMILY, 0x0069_1140, has_flags(e, this, 0x200))
}

// Typed from 0068f200
/// Whether edge `edge` links to a triangle (`thiscall(triangle, edge)`).
pub fn has_link(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: i32) -> bool {
    e.get(this, link_field(edge as u32)) != NO_TRIANGLE
}

// Typed from 0068f2c0
/// The triangle edge `edge` links to (`thiscall(triangle, edge)`).
pub fn link(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: i32) -> u16 {
    e.get(this, link_field(edge as u32))
}

// Typed from 0068f230
/// `NavMesh::GetEdgeExtraInfo` (Xbox PDB, `thiscall(mesh, triangle, edge)`): for an edge to another
/// mesh with a link, the edge-information entry the link indexes; else null. The triangle is looked up
/// again for each read, as the exe does.
pub fn edge_info(e: &mut Engine, this: Ptr<NavMesh>, index: u16, edge: i32) -> u32 {
    let triangles = this.at(NavMesh::triangles);
    let t = typed_call!(
        FAMILY,
        0x0055_8dd0,
        triangle_at(e, triangles, u32::from(index))
    );
    if !typed_call!(FAMILY, 0x0068_f1d0, edge_flag(e, t, edge as u32)) {
        return 0;
    }
    let t = typed_call!(
        FAMILY,
        0x0055_8dd0,
        triangle_at(e, triangles, u32::from(index))
    );
    if typed_call!(FAMILY, 0x0068_f2c0, link(e, t, edge)) == NO_TRIANGLE {
        return 0;
    }
    let t = typed_call!(
        FAMILY,
        0x0055_8dd0,
        triangle_at(e, triangles, u32::from(index))
    );
    let l = typed_call!(FAMILY, 0x0068_f2c0, link(e, t, edge));
    typed_call!(
        FAMILY,
        0x006a_1440,
        entry_12(e, this.at(NavMesh::edge_infos), u32::from(l))
    )
}

// Typed from 00693d30
/// `NavMesh::GetExtraEdgeInfo` (`thiscall(mesh, index)`): entry `index` of the edge information.
pub fn edge_info_at(e: &mut Engine, this: Ptr<NavMesh>, index: u16) -> u32 {
    typed_call!(
        FAMILY,
        0x006a_1440,
        entry_12(e, this.at(NavMesh::edge_infos), u32::from(index))
    )
}

// Typed from 0068efb0
/// The mesh's navmesh info (`thiscall(mesh)`).
pub fn info(e: &mut Engine, this: Ptr<NavMesh>) -> u32 {
    e.get(this, NavMesh::info)
}

// Typed from 0068f460
/// `NavMesh::GetMatchingEdge` (Xbox PDB, `thiscall(mesh, triangle, edge, out mesh, out triangle, out
/// edge)`): the triangle and edge on the other side of an edge. Within the mesh: the linked triangle
/// and its first edge, not leading out, that links back. Across meshes: the edge information's mesh
/// (`0068f2e0` resolves its info), its triangle, and that triangle's first outgoing edge whose
/// information names this mesh's info and this triangle. False for a triangle out of range, an edge
/// without a link or a match.
pub fn matching_edge(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    edge: i32,
    out_mesh: Ptr,
    out_triangle: Ptr,
    out_edge: Ptr,
) -> bool {
    if u32::from(index) >= typed_call!(FAMILY, 0x0055_8200, triangle_count(e, this)) {
        return false;
    }
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    if !typed_call!(FAMILY, 0x0068_f200, has_link(e, t, edge)) {
        return false;
    }
    if !typed_call!(FAMILY, 0x0068_f1d0, edge_flag(e, t, edge as u32)) {
        // Within the mesh: the out triangle is written, then read back.
        e.mem.set_u32(out_mesh.addr(), this.addr());
        let l = typed_call!(FAMILY, 0x0068_f2c0, link(e, t, edge));
        e.mem.set_u16(out_triangle.addr(), l);
        let other = e.mem.u16(out_triangle.addr());
        let t2 = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, other));
        let mut i: i32 = 0;
        while i < 3 {
            if !typed_call!(FAMILY, 0x0068_f1d0, edge_flag(e, t2, i as u32))
                && typed_call!(FAMILY, 0x0068_f2c0, link(e, t2, i)) == index
            {
                break;
            }
            i += 1;
        }
        if i >= 3 {
            return false;
        }
        e.mem.set_u32(out_edge.addr(), i as u32);
        return true;
    }
    let x = Ptr::<EdgeInfo>::new(typed_call!(
        FAMILY,
        0x0068_f230,
        edge_info(e, this, index, edge)
    ));
    if e.get(x, EdgeInfo::info) == 0 {
        return false;
    }
    e.with_stack(4, |e, mesh_slot| {
        let info_word = e.get(x, EdgeInfo::info);
        if e.call(INFO_TO_MESH, &args![info_word, mesh_slot]).u8() == 0 {
            return false;
        }
        let other = e.get(x, EdgeInfo::triangle);
        let mesh = Ptr::<NavMesh>::new(e.mem.u32(mesh_slot.addr()));
        if u32::from(other) >= typed_call!(FAMILY, 0x0055_8200, triangle_count(e, mesh)) {
            return false;
        }
        let t2 = typed_call!(FAMILY, 0x0055_82f0, triangle(e, mesh, other));
        let mut i: i32 = 0;
        while i < 3 {
            if typed_call!(FAMILY, 0x0068_f1d0, edge_flag(e, t2, i as u32)) {
                let mesh = Ptr::<NavMesh>::new(e.mem.u32(mesh_slot.addr()));
                let x2 = typed_call!(FAMILY, 0x0068_f230, edge_info(e, mesh, other, i));
                if x2 != 0 {
                    let x2 = Ptr::<EdgeInfo>::new(x2);
                    let own = typed_call!(FAMILY, 0x0068_efb0, info(e, this));
                    if e.get(x2, EdgeInfo::info) == own && e.get(x2, EdgeInfo::triangle) == index {
                        break;
                    }
                }
            }
            i += 1;
        }
        if i == 3 {
            return false;
        }
        let mesh = e.mem.u32(mesh_slot.addr());
        e.mem.set_u32(out_mesh.addr(), mesh);
        e.mem.set_u16(out_triangle.addr(), other);
        e.mem.set_u32(out_edge.addr(), i as u32);
        true
    })
}

// Typed from 0068f670
/// `NavMesh::GetMatchingEdge` (`thiscall(mesh, edge, out)`): [`matching_edge`] of an edge reference,
/// into the reference at `out`.
pub fn matching_edge_of(e: &mut Engine, this: Ptr<NavMesh>, edge: Ptr<MeshEdge>, out: Ptr) -> bool {
    let index = e.get(edge, MeshEdge::triangle);
    let which = e.get(edge, MeshEdge::extra) as i32;
    typed_call!(
        FAMILY,
        0x0068_f460,
        matching_edge(e, this, index, which, out, out.byte_add(4), out.byte_add(8))
    )
}

// Typed from 0068f3b0
/// `NavMesh::GetMatchingEdge` (`thiscall(mesh, triangle, edge, out holder, out triangle, out edge)`):
/// [`matching_edge`] with the mesh handed out in a counted holder (`00464fc0` makes a temporary one,
/// `0042f4c0` assigns it, `0042fa40` releases the temporary).
pub fn matching_edge_held(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    edge: i32,
    out_holder: Ptr,
    out_triangle: Ptr,
    out_edge: Ptr,
) -> bool {
    e.with_stack(4, |e, mesh_slot| {
        e.mem.set_u32(mesh_slot.addr(), 0);
        if !typed_call!(
            FAMILY,
            0x0068_f460,
            matching_edge(e, this, index, edge, mesh_slot, out_triangle, out_edge)
        ) {
            return false;
        }
        let mesh = e.mem.u32(mesh_slot.addr());
        e.with_stack(8, |e, temporary| {
            let held = e.call(MAKE_HOLDER, &args![temporary, mesh]).u32();
            e.call(HOLDER_ASSIGN, &args![out_holder, held]);
            e.call(HOLDER_RELEASE, &args![temporary]);
        });
        true
    })
}

// Typed from 0068f0c0
/// The points of an edge (`thiscall(mesh, triangle, edge, first, second)`): its vertex and the next
/// one (`(edge + 1) % 3`, a signed remainder), each point copied as words.
pub fn edge_points(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    edge: i32,
    first: Ptr<NiPoint3>,
    second: Ptr<NiPoint3>,
) -> bool {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    let a = typed_call!(FAMILY, 0x0055_82d0, vertex(e, t, edge as u32));
    let next = edge.wrapping_add(1) % 3;
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    let b = typed_call!(FAMILY, 0x0055_82d0, vertex(e, t, next as u32));
    let p = typed_call!(FAMILY, 0x0068_f0a0, vertex_point(e, this, a));
    copy_point(e, p, first);
    let p = typed_call!(FAMILY, 0x0068_f0a0, vertex_point(e, this, b));
    copy_point(e, p, second);
    true
}

// Typed from 0068efd0
/// An edge's two vertices (`thiscall(mesh, out, triangle, edge)`): one word, the edge's vertex in the
/// low half and the next one's in the high half.
pub fn edge(e: &mut Engine, this: Ptr<NavMesh>, out: Ptr, index: u16, which: i32) -> Ptr {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    let a = typed_call!(FAMILY, 0x0055_82d0, vertex(e, t, which as u32));
    let next = which.wrapping_add(1) % 3;
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    let b = typed_call!(FAMILY, 0x0055_82d0, vertex(e, t, next as u32));
    e.mem
        .set_u32(out.addr(), u32::from(a) | (u32::from(b) << 16));
    out
}

// Typed from 0068f040
/// `NavMesh::GetEdgeVertices` (`thiscall(mesh, out, triangle, edge)`): the points of the edge's two
/// vertices ([`edge`]), as two pointers at `out`.
pub fn edge_vertices(e: &mut Engine, this: Ptr<NavMesh>, out: Ptr, index: u16, which: i32) -> Ptr {
    e.with_stack(4, |e, pair| {
        typed_call!(FAMILY, 0x0068_efd0, edge(e, this, pair, index, which));
        let a = e.mem.u16(pair.addr());
        let pa = typed_call!(FAMILY, 0x0068_f0a0, vertex_point(e, this, a));
        let b = e.mem.u16(pair.addr() + 2);
        let pb = typed_call!(FAMILY, 0x0068_f0a0, vertex_point(e, this, b));
        e.mem.set_u32(out.addr(), pa.addr());
        e.mem.set_u32(out.addr() + 4, pb.addr());
    });
    out
}

/// `(a - o) x (b - o)` in x and y, each factor's second term a memory operand: `(a.x - o.x) * (b.y -
/// o.y) - (a.y - o.y) * (b.x - o.x)` (`FLD a.x; FSUB o.x; FLD b.y; FSUB o.y; FMULP; FLD a.y; FSUB o.y;
/// FLD b.x; FSUB o.x; FMULP; FSUBP`).
#[inline(always)]
pub(crate) fn cross_2d(a: [f64; 2], b: [f64; 2], o: [f64; 2]) -> f64 {
    let first = x87_mul(x87_sub(a[0], o[0]), x87_sub(b[1], o[1]));
    let second = x87_mul(x87_sub(a[1], o[1]), x87_sub(b[0], o[0]));
    x87_sub(first, second)
}

/// A point's x and y: the first loaded (`FLD`), the second as a memory operand.
#[inline(always)]
pub(crate) fn xy_loaded(e: &Engine, p: Ptr<NiPoint3>) -> [f64; 2] {
    [fld(e, p, NiPoint3::x), fld(e, p, NiPoint3::y)]
}

/// A point's x and y as memory operands.
#[inline(always)]
pub(crate) fn xy_operand(e: &Engine, p: Ptr<NiPoint3>) -> [f64; 2] {
    [m32(e, p, NiPoint3::x), m32(e, p, NiPoint3::y)]
}

/// The same with the second product's factors the other way round, `(b.x - o.x) * (a.y - o.y)`, as the
/// barycentric tests load them.
#[inline(always)]
fn cross_2d_swapped(a: [f64; 2], b: [f64; 2], o: [f64; 2]) -> f64 {
    let first = x87_mul(x87_sub(a[0], o[0]), x87_sub(b[1], o[1]));
    let second = x87_mul(x87_sub(b[0], o[0]), x87_sub(a[1], o[1]));
    x87_sub(first, second)
}

/// `(a - o) x (b - o)` with `a` and `b` loaded and `o` the memory operand.
#[inline(always)]
fn cross_at(e: &Engine, a: Ptr<NiPoint3>, b: Ptr<NiPoint3>, o: Ptr<NiPoint3>) -> f64 {
    let (a, b, o) = (xy_loaded(e, a), xy_loaded(e, b), xy_operand(e, o));
    cross_2d(a, b, o)
}

/// The barycentric height test the triangle height and `006b9cf0` share: with the corners `a`, `b`,
/// `c` and the point `p`, false when the triangle is flat in x and y (an ordered zero) or a weight is
/// below zero; else the height `w0 * a.z + w1 * b.z + w2 * c.z` (each product a memory operand) goes
/// to `store`, which returns the difference to test, and that has to lie in `[-below, above]` (a NaN
/// passes).
pub(crate) fn barycentric(
    e: &mut Engine,
    corners: [Ptr<NiPoint3>; 3],
    p: Ptr<NiPoint3>,
    above: f32,
    below: f32,
    store: impl FnOnce(&mut Engine, f32) -> f32,
) -> bool {
    let [a, b, c] = corners;
    let det = fstp(cross_2d_swapped(
        xy_loaded(e, b),
        xy_loaded(e, c),
        xy_operand(e, a),
    ));
    if !parity_even(fcom(fld_local(det), m64_global(e, ZERO_DOUBLE)), 0x44) {
        return false;
    }
    let sub = |e: &Engine, q: Ptr<NiPoint3>, f| fstp(x87_sub(fld(e, q, f), m32(e, p, f)));
    let bx = sub(e, b, NiPoint3::x);
    let cx = sub(e, c, NiPoint3::x);
    let by = sub(e, b, NiPoint3::y);
    let cy = sub(e, c, NiPoint3::y);
    let inverse = fstp(x87_div(1.0, fld_local(det)));
    let weight = |u: f32, v: f32, s: f32, t: f32| {
        let d = x87_sub(
            x87_mul(fld_local(u), m32_local(v)),
            x87_mul(fld_local(s), m32_local(t)),
        );
        fstp(x87_mul(d, m32_local(inverse)))
    };
    let zero = m64_global(e, ZERO_DOUBLE);
    let negative = |w: f32| !parity_even(fcom(fld_local(w), zero), 0x05);
    let w0 = weight(bx, cy, cx, by);
    if negative(w0) {
        return false;
    }
    let ax = sub(e, a, NiPoint3::x);
    let ay = sub(e, a, NiPoint3::y);
    let w1 = weight(cx, ay, ax, cy);
    if negative(w1) {
        return false;
    }
    let w2 = weight(ax, by, bx, ay);
    if negative(w2) {
        return false;
    }
    let z0 = x87_mul(fld_local(w0), m32(e, a, NiPoint3::z));
    let z1 = x87_mul(fld_local(w1), m32(e, b, NiPoint3::z));
    let z2 = x87_mul(fld_local(w2), m32(e, c, NiPoint3::z));
    let height = fstp(x87_add(x87_add(z0, z1), z2));
    let difference = fld_local(store(e, height));
    // FLD difference; FLD above; FCOMPP: taken on above >= difference or unordered.
    if !parity_even(fcom(fld_local(above), difference), 0x05) {
        return false;
    }
    // FLD below; FCHS; FLD difference; FCOMPP.
    parity_even(fcom(difference, -fld_local(below)), 0x05)
}

// Typed from 00697980
/// `NavMesh::ComputeTriangleZForLocation` (Xbox PDB, `thiscall(mesh, triangle, point, above, below,
/// out)`): the triangle's height under the point, stored to `out` (also when the difference is out of
/// range), and whether `point.z - height` lies in `[-below, above]`.
pub fn triangle_z(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    point: Ptr<NiPoint3>,
    above: f32,
    below: f32,
    out: Ptr,
) -> bool {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    let corners = [
        corner(e, this, t, 0),
        corner(e, this, t, 1),
        corner(e, this, t, 2),
    ];
    let out = out.cast::<Point2>();
    barycentric(e, corners, point, above, below, |e, height| {
        e.set(out, Point2::x, height);
        fstp(x87_sub(fld(e, point, NiPoint3::z), m32(e, out, Point2::x)))
    })
}

/// Copies the first two floats of a point through the x87 stack (`FLD`/`FSTP`), as the vertex arrays
/// of the edge searches hold them.
#[inline(always)]
fn store_xy(e: &mut Engine, from: Ptr<NiPoint3>, to: Ptr<Point2>) {
    let x = fstp(fld(e, from, NiPoint3::x));
    e.set(to, Point2::x, x);
    let y = fstp(fld(e, from, NiPoint3::y));
    e.set(to, Point2::y, y);
}

// Typed from 00698060
/// The edge of a triangle a segment crosses (`thiscall(mesh, triangle, skip, start, end, out)`): the
/// corners' x and y kept as floats (8 bytes each; the exe's vector constructor `00401050` runs `ReturnThis` over them, which does
/// nothing, and is left out), then the
/// first edge but `skip` that `006b9e90` finds crossing; -1 when none does.
pub fn triangle_edge(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    skip: i32,
    start: Ptr<Point2>,
    end: Ptr<Point2>,
    out: Ptr<Point2>,
) -> i32 {
    // The exe copies each corner to a point local first, as words; read from the corner itself here.
    e.with_stack(0x18, |e, locals| {
        let corners = locals.cast::<Point2>();
        for i in 0..3u32 {
            let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
            let p = corner(e, this, t, i);
            store_xy(e, p, corners.index(i, 8));
        }
        for j in 0..3i32 {
            if j == skip {
                continue;
            }
            let k = if j + 1 == 3 { 0 } else { j + 1 };
            let (pj, pk) = (corners.index(j as u32, 8), corners.index(k as u32, 8));
            if typed_call!(
                FAMILY,
                0x006b_9e90,
                pov::crossing(e, pj, pk, start, end, out)
            ) {
                return j;
            }
        }
        -1
    })
}

// Typed from 00698320
/// `NavMesh::FindCrossedEdge` (Xbox PDB, `thiscall(mesh, triangle, skip, start, end)`): the first edge
/// but `skip` that the segment from `start` to `end` crosses, with neither end exactly on it; -1 when
/// none does. The corners are copied as words (the exe's vector constructor over them runs `ReturnThis`, which does nothing, and is left out); each side test is a 2D cross product stored to a float.
pub fn crossed_edge(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    index: u16,
    skip: i32,
    start: Ptr<Point2>,
    end: Ptr<Point2>,
) -> i32 {
    e.with_stack(0x24, |e, corners| {
        let corners = corners.cast::<NiPoint3>();
        let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
        for i in 0..3u32 {
            let p = corner(e, this, t, i);
            copy_point(e, p, corners.index(i, 0xc));
        }
        let zero = m64_global(e, ZERO_DOUBLE);
        let (start, end) = (start.cast::<NiPoint3>(), end.cast::<NiPoint3>());
        for i in 0..3i32 {
            if i == skip {
                continue;
            }
            let k = if i + 1 == 3 { 0 } else { i + 1 };
            let vi = corners.index(i as u32, 0xc);
            let vk = corners.index(k as u32, 0xc);
            let side_end = fstp(cross_at(e, vi, vk, end));
            let side_start = fstp(cross_at(e, vi, vk, start));
            // (start - vi) x (end - vi) and (start - vk) x (end - vk): the corner is the operand.
            let across_i = fstp(cross_2d(
                xy_loaded(e, start),
                xy_loaded(e, end),
                xy_operand(e, vi),
            ));
            let across_k = fstp(cross_2d(
                xy_loaded(e, start),
                xy_loaded(e, end),
                xy_operand(e, vk),
            ));
            // Taken (next edge) when the product is above zero or unordered.
            let product = x87_mul(fld_local(side_end), m32_local(side_start));
            if parity_even(fcom(product, zero), 0x41) {
                continue;
            }
            let product = x87_mul(fld_local(across_i), m32_local(across_k));
            if parity_even(fcom(product, zero), 0x41) {
                continue;
            }
            // An ordered zero on either side: the next edge.
            if !parity_even(fcom(fld_local(side_end), zero), 0x44) {
                continue;
            }
            if !parity_even(fcom(fld_local(across_k), zero), 0x44) {
                continue;
            }
            return i;
        }
        -1
    })
}

/// A word slot on the guest stack read as a float.
#[inline(always)]
fn local_float(e: &Engine, slot: Ptr) -> f32 {
    e.get(slot.cast::<Point2>(), Point2::x)
}

// Typed from 00696a50
/// `NavMesh::FindTriangleForLocation` (Xbox PDB, `thiscall(mesh, point, callback)`): among the
/// triangles of the point's grid cell, not flagged 0x20, whose height `00697810` finds: a height
/// difference below 40 at once, else the smallest one; differences below -180 count only when the point
/// is under the water's height plus 50 (`00547250`), and those above 64 never. `0xffff` for none.
pub fn triangle_for(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr<NiPoint3>,
    callback: u32,
) -> u16 {
    e.with_stack(0xc, |e, locals| {
        let (list_slot, water, height) = (locals, locals.byte_add(4), locals.byte_add(8));
        e.mem.set_u32(list_slot.addr(), 0);
        typed_call!(
            FAMILY,
            0x0069_d370,
            grid_cell_at(e, this.at(NavMesh::grid), point, list_slot)
        );
        let mut best = NO_TRIANGLE;
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            return NO_TRIANGLE;
        }
        let count = typed_call!(
            leaf::FAMILY,
            0x0044_ddc0,
            leaf::word_at_8(e, Ptr::<()>::new(list))
        );
        let mut smallest = fstp(fld_global(e, FLOAT_MAX));
        let lowest = fstp(fld_global(e, FLOAT_LOWEST));
        e.set(water.cast::<Point2>(), Point2::x, lowest);
        let mesh_water = e.get(this, NavMesh::water);
        if mesh_water != 0 {
            e.call(WATER_HEIGHT, &args![mesh_water, point, water]);
        }
        let limit = fstp(x87_add(
            fld_local(local_float(e, water)),
            m64_global(e, ABOVE_WATER),
        ));
        // FLD limit; FLD point.z; FCOMPP: set when z is below the limit.
        let under = !parity_even(fcom(fld(e, point, NiPoint3::z), fld_local(limit)), 0x05);
        let mut i = 0u32;
        while i < count {
            let entry = typed_call!(
                FAMILY,
                0x0069_dec0,
                search::list_element(e, Ptr::new(list), i)
            );
            let t_index = e.mem.u16(entry);
            i += 1;
            let t = typed_call!(
                FAMILY,
                0x0055_8dd0,
                triangle_at(e, this.at(NavMesh::triangles), u32::from(t_index))
            );
            if typed_call!(FAMILY, 0x0069_1140, has_flags(e, t, 0x20)) {
                continue;
            }
            if e.call(
                HEIGHT_IN_TRIANGLE,
                &args![this, t_index, point, height, callback],
            )
            .u8()
                == 0
            {
                continue;
            }
            let d = fld_local(local_float(e, height));
            if !under && !parity_even(fcom(d, m64_global(e, LOWEST_HEIGHT)), 0x05) {
                continue;
            }
            if fcom(
                fld_local(local_float(e, height)),
                m64_global(e, HIGHEST_HEIGHT),
            ) & 0x41
                == 0
            {
                continue;
            }
            let h = local_float(e, height);
            let a = fstp(fld_local(typed_call!(
                FAMILY,
                0x0040_8840,
                abs(e, through(h))
            )));
            e.set(height.cast::<Point2>(), Point2::x, a);
            if fcom(fld_local(smallest), fld_local(a)) & 0x41 != 0 {
                continue;
            }
            if !parity_even(fcom(fld_local(a), m64_global(e, CLOSE_HEIGHT)), 0x05) {
                return t_index;
            }
            best = t_index;
            smallest = fstp(fld_local(a));
        }
        best
    })
}

// Typed from 00696cf0
/// `NavMesh::FindClosestTriangleForLocation` (Xbox PDB, `thiscall(mesh, point, out triangle, out
/// exact)`): over the point's grid cell, or every triangle when the point has no cell, the first
/// triangle whose height `00697730` finds (180 below, 64 above) within 40, else the one with the
/// smallest height difference (exact), else the one whose centre is nearest (not exact).
pub fn closest_triangle(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr<NiPoint3>,
    out_triangle: Ptr,
    out_exact: Ptr,
) -> bool {
    e.with_stack(4, |e, list_slot| {
        let mut nearest = fstp(fld_global(e, FLOAT_MAX));
        let mut nearest_index = NO_TRIANGLE;
        let mut found = NO_TRIANGLE;
        e.mem.set_u32(list_slot.addr(), 0);
        typed_call!(
            FAMILY,
            0x0069_d370,
            grid_cell_at(e, this.at(NavMesh::grid), point, list_slot)
        );
        let list = e.mem.u32(list_slot.addr());
        let close = m64_global(e, CLOSE_HEIGHT);
        // One candidate: 0 to go on, 1 when the search ends with it exact.
        let mut candidate = |e: &mut Engine, t_index: u16, smallest: &mut f32| -> bool {
            // Each candidate's height, difference and centre in a frame of their own (as the
            // translation keeps them; the exe reuses one set of locals).
            e.with_stack(0x20, |e, scratch| {
                let height = scratch;
                let difference = scratch.byte_add(4).cast::<NiPoint3>();
                let center_out = scratch.byte_add(0x10).cast::<NiPoint3>();
                let below = fstp(fld_global(e, BELOW_PASSED));
                let above = fstp(fld_global(e, ABOVE_PASSED));
                let hit = e
                    .call(
                        HEIGHT_NEAR_TRIANGLE,
                        &args![this, t_index, point, height, below, above],
                    )
                    .u8()
                    != 0;
                if hit {
                    let h = local_float(e, height);
                    let a = fstp(fld_local(typed_call!(
                        FAMILY,
                        0x0040_8840,
                        abs(e, through(h))
                    )));
                    e.set(height.cast::<Point2>(), Point2::x, a);
                    if fcom(fld_local(*smallest), fld_local(a)) & 0x41 == 0 {
                        if !parity_even(fcom(fld_local(a), close), 0x05) {
                            e.mem.set_u16(out_triangle.addr(), t_index);
                            e.mem.set_u8(out_exact.addr(), 1);
                            return true;
                        }
                        found = t_index;
                        *smallest = fstp(fld_local(a));
                    }
                } else {
                    let c = typed_call!(FAMILY, 0x0055_8220, center(e, this, center_out, t_index));
                    let d = typed_call!(FAMILY, 0x0043_9ef0, subtract(e, c, difference, point));
                    let squared = typed_call!(FAMILY, 0x004a_7290, length_squared(e, d));
                    if fcom(fld_local(nearest), fld_local(squared)) & 0x41 == 0 {
                        nearest_index = t_index;
                        nearest = fstp(fld_local(squared));
                    }
                }
                false
            })
        };
        if list != 0 {
            let count = typed_call!(
                leaf::FAMILY,
                0x0044_ddc0,
                leaf::word_at_8(e, Ptr::<()>::new(list))
            );
            let mut smallest = fstp(fld_global(e, FLOAT_MAX));
            for i in 0..count {
                let entry = typed_call!(
                    FAMILY,
                    0x0069_dec0,
                    search::list_element(e, Ptr::new(list), i)
                );
                let t_index = e.mem.u16(entry);
                let t = typed_call!(
                    FAMILY,
                    0x0055_8dd0,
                    triangle_at(e, this.at(NavMesh::triangles), u32::from(t_index))
                );
                if typed_call!(FAMILY, 0x0069_1140, has_flags(e, t, 0x20)) {
                    continue;
                }
                if candidate(e, t_index, &mut smallest) {
                    return true;
                }
            }
        } else {
            let count = typed_call!(
                leaf::FAMILY,
                0x0044_ddc0,
                leaf::word_at_8(e, this.at(NavMesh::triangles))
            );
            let mut smallest = fstp(fld_global(e, FLOAT_MAX));
            for i in 0..count {
                #[allow(clippy::cast_possible_truncation)]
                let t_index = i as u16;
                if candidate(e, t_index, &mut smallest) {
                    return true;
                }
            }
        }
        if found != NO_TRIANGLE {
            e.mem.set_u16(out_triangle.addr(), found);
            e.mem.set_u8(out_exact.addr(), 1);
            return true;
        }
        e.mem.set_u16(out_triangle.addr(), nearest_index);
        e.mem.set_u8(out_exact.addr(), 0);
        nearest_index != NO_TRIANGLE
    })
}

// Typed from 006969e0
/// The grid cells a box covers (`thiscall(mesh, min, max)`), through `00696a10`; always true.
pub fn grid_range(e: &mut Engine, this: Ptr<NavMesh>, min: Ptr, max: Ptr) -> bool {
    e.call(GRID_RANGE, &args![this.at(NavMesh::grid), min, max]);
    true
}

// Typed from 006967c0
/// The grid cell at column `x`, row `y` (`thiscall(grid, x, y)`), unchecked.
pub fn grid_cell(e: &mut Engine, this: Ptr<NavMeshGrid>, x: u32, y: u32) -> u32 {
    let cell = y
        .wrapping_mul(e.get(this, NavMeshGrid::side))
        .wrapping_add(x);
    (cell << 4).wrapping_add(e.get(this, NavMeshGrid::cells))
}

/// `00406ce0` (the float rounded down, in `ST0`) and `FISTP qword` under truncation: the low word.
fn floor_word(e: &mut Engine, value: f32) -> u32 {
    let down = e.call(FLOOR, &args![through(value)]).f64();
    low_word(fistp_chop64(down))
}

// Typed from 0069d370
/// `NavMeshGridView::GetTriListAtPosition` (`thiscall(grid, point, out)`): null when the point is
/// outside the grid's box in x or y (a NaN coordinate is inside), else the cell its offset from the
/// box's corner falls in, each index rounded down (`00406ce0`) and kept below the side.
pub fn grid_cell_at(e: &mut Engine, this: Ptr<NavMeshGrid>, point: Ptr<NiPoint3>, out: Ptr) {
    let px = fld(e, point, NiPoint3::x);
    let py = fld(e, point, NiPoint3::y);
    let outside = fcom(fld(e, this, NavMeshGrid::min_x), px) & 0x41 == 0
        || !parity_even(fcom(fld(e, this, NavMeshGrid::max_x), px), 0x05)
        || fcom(fld(e, this, NavMeshGrid::min_y), py) & 0x41 == 0
        || !parity_even(fcom(fld(e, this, NavMeshGrid::max_y), py), 0x05);
    if outside {
        e.mem.set_u32(out.addr(), 0);
        return;
    }
    let dx = fstp(x87_sub(px, m32(e, this, NavMeshGrid::min_x)));
    let dy = fstp(x87_sub(py, m32(e, this, NavMeshGrid::min_y)));
    let fx = fstp(x87_div(fld_local(dx), m32(e, this, NavMeshGrid::cell_x)));
    let fy = fstp(x87_div(fld_local(dy), m32(e, this, NavMeshGrid::cell_y)));
    let mut ix = floor_word(e, fx);
    let mut iy = floor_word(e, fy);
    let side = e.get(this, NavMeshGrid::side);
    if ix >= side {
        ix = e.get(this, NavMeshGrid::side).wrapping_sub(1);
    }
    if iy >= e.get(this, NavMeshGrid::side) {
        iy = e.get(this, NavMeshGrid::side).wrapping_sub(1);
    }
    let cell = typed_call!(FAMILY, 0x0069_67c0, grid_cell(e, this, ix, iy));
    e.mem.set_u32(out.addr(), cell);
}

// Typed from 006971e0
/// `NavMesh::GetPlaneNormal` (Xbox PDB, `thiscall(mesh, out, triangle)`): the cross product
/// (`0053d1a0`) of the edges from the first corner to the second and to the third.
pub fn plane_normal(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    out: Ptr<NiPoint3>,
    index: u16,
) -> Ptr<NiPoint3> {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, this, index));
    e.with_stack(0x18, |e, locals| {
        let (first, second) = (
            locals.cast::<NiPoint3>(),
            locals.byte_add(0xc).cast::<NiPoint3>(),
        );
        let a = corner(e, this, t, 0);
        let b = corner(e, this, t, 1);
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, b, first, a));
        let a = corner(e, this, t, 0);
        let c = corner(e, this, t, 2);
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, c, second, a));
        e.call(CROSS, &args![first, out, second]);
    });
    out
}

// Typed from 006cd5c0
/// `CheckLineOfSightFilterWater::CanCrossEdge` (`thiscall(filter, _, _, _, mesh, triangle, _)`): not for
/// a triangle flagged 0x200, 0x10 when the filter's byte says so, 0x80000000 or 0x1000.
#[allow(clippy::too_many_arguments)]
pub fn water_can_cross(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    mesh: Ptr<NavMesh>,
    index: u16,
    _unused_6: u32,
) -> bool {
    let t = typed_call!(FAMILY, 0x0055_82f0, triangle(e, mesh, index));
    if typed_call!(FAMILY, 0x006a_5ec0, flag_200(e, t)) {
        return false;
    }
    if typed_call!(FAMILY, 0x0069_3b90, flag_10(e, t)) && e.mem.u8(this.addr() + 4) != 0 {
        return false;
    }
    if typed_call!(FAMILY, 0x0069_1140, has_flags(e, t, 0x8000_0000)) {
        return false;
    }
    !typed_call!(FAMILY, 0x0069_1140, has_flags(e, t, 0x1000))
}

// Typed from 006a78d0
/// The triangle a mesh-and-triangle reference names (`thiscall(reference)`).
pub fn referenced_triangle(e: &mut Engine, this: Ptr<MeshEdge>) -> Ptr<NavMeshTriangle> {
    let mesh = Ptr::<NavMesh>::new(e.get(this, MeshEdge::mesh));
    let index = e.get(this, MeshEdge::triangle);
    typed_call!(FAMILY, 0x0055_82f0, triangle(e, mesh, index))
}

// Typed from 0069ad00
/// `NavMeshInfo::ResolveNavMesh` (`thiscall(info, holder)`): false for an info `0068f320` refuses;
/// else its navmesh into the holder (`00690890`), and whether the holder holds one.
pub fn resolve(e: &mut Engine, this: Ptr<NavMeshInfo>, holder: Ptr) -> bool {
    if e.call(IS_FORMLESS, &args![this]).u8() != 0 {
        return false;
    }
    let mesh = e.get(this, NavMeshInfo::navmesh);
    typed_call!(FAMILY, 0x0069_0890, hold(e, holder, mesh));
    typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, holder)) != 0
}

// Typed from 00690890
/// A navmesh into a counted holder (`cdecl(holder, mesh)`): a temporary holder of it (`00464ff0`),
/// assigned (`0042f4c0`) and released (`0042fa40`).
pub fn hold(e: &mut Engine, holder: Ptr, mesh: u32) {
    e.with_stack(4, |e, temporary| {
        e.call(HOLDER_NEW, &args![temporary, mesh]);
        e.call(HOLDER_ASSIGN, &args![holder, temporary]);
        e.call(HOLDER_RELEASE, &args![temporary]);
    });
}

// Typed from 00690830
/// The info's world space (`thiscall(info)`), else the world space of its cell's form (`004839c0`).
pub fn info_world_space(e: &mut Engine, this: Ptr<NavMeshInfo>) -> u32 {
    let w = e.get(this, NavMeshInfo::world_space);
    if w != 0 {
        return w;
    }
    let cell = e.get(this, NavMeshInfo::cell_id);
    e.call(CELL_WORLD_SPACE, &args![cell]).u32()
}

// Typed from 00690800
/// `NavMeshInfo::GetParentWorldSpace` (`thiscall(info)`): [`info_world_space`] cast to a world space
/// (`00ec43fb`).
pub fn parent_world_space(e: &mut Engine, this: Ptr<NavMeshInfo>) -> u32 {
    let w = typed_call!(FAMILY, 0x0069_0830, info_world_space(e, this));
    e.call(DYNAMIC_CAST, &args![w, 0u32, CAST_FROM, CAST_TO, 0u32])
        .u32()
}

// Typed from 0069dfb0
/// Whether the info's navmesh is loaded (`thiscall(info)`).
pub fn is_loaded(e: &mut Engine, this: Ptr<NavMeshInfo>) -> bool {
    e.get(this, NavMeshInfo::navmesh) != 0
}

// Typed from 006a1b60
/// The info's navmesh (`thiscall(info)`).
pub fn info_mesh(e: &mut Engine, this: Ptr<NavMeshInfo>) -> u32 {
    e.get(this, NavMeshInfo::navmesh)
}

// Typed from 00693d70
/// `Pathing::FindReferenceByFormId` (`thiscall(pathing, id)`): the reference the map at +0x1c holds
/// for the id (`00853130`), or null.
pub fn reference_by_id(e: &mut Engine, this: Ptr, id: u32) -> u32 {
    e.with_stack(4, |e, found| {
        if e.call(MAP_LOOKUP, &args![this.byte_add(0x1c), id, found])
            .u8()
            != 0
        {
            e.mem.u32(found.addr())
        } else {
            0
        }
    })
}

// Typed from 006b77e0
/// `NavMeshInfoMap::FindNavMeshInfoForCell` (`thiscall(map, cell)`): null for no cell; an interior's
/// through `006b78b0`; an exterior's by its world space and the key of its grid position.
pub fn info_for_cell(e: &mut Engine, this: Ptr, cell: Ptr) -> u32 {
    if cell.is_null() {
        return 0;
    }
    if e.call(IS_INTERIOR, &args![cell]).u8() != 0 {
        return e.call(INTERIOR_INFO, &args![this, cell]).u32();
    }
    let y = e.call(CELL_Y, &args![cell]).u32();
    let x = e.call(CELL_X, &args![cell]).u32();
    let key = e.call(CELL_KEY, &args![x, y]).u32();
    let world = e.call(CELL_WORLD, &args![cell]).u32();
    typed_call!(
        FAMILY,
        0x006b_7850,
        info_in_world(e, this, Ptr::new(world), key)
    )
}

// Typed from 006b7850
/// The navmesh info of a world space's cell (`thiscall(map, world, key)`): the world's map by its form
/// id (`00853130` on the map at +0x2c), then the info by the key's hash (`00464f30`); null when either
/// is missing.
pub fn info_in_world(e: &mut Engine, this: Ptr, world: Ptr, key: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let (map_slot, info_slot) = (locals, locals.byte_add(4));
        let id = typed_call!(leaf::FAMILY, 0x0084_e3a0, leaf::word_at_c(e, world));
        if e.call(MAP_LOOKUP, &args![this.byte_add(0x2c), id, map_slot])
            .u8()
            == 0
        {
            return 0;
        }
        let hashed = e.call(KEY_HASH, &args![key]).u32();
        let map = e.mem.u32(map_slot.addr());
        if e.call(MAP_LOOKUP, &args![map, hashed, info_slot]).u8() == 0 {
            return 0;
        }
        e.mem.u32(info_slot.addr())
    })
}

// Typed from 006d6f40
/// `Pathing::GetPotentialNavMeshInfoForLocation` (`cdecl(cell)`): [`info_for_cell`] on the pathing
/// singleton's info map (`0045af00`).
pub fn potential_info(e: &mut Engine, cell: u32) -> u32 {
    let pathing = e.global::<u32>(PATHING);
    let map = e.call(PATHING_INFO_MAP, &args![pathing]).u32();
    typed_call!(
        FAMILY,
        0x006b_77e0,
        info_for_cell(e, Ptr::new(map), Ptr::new(cell))
    )
}

// Typed from 006d6f60
/// `Pathing::GetPotentialNavMeshInfoForLocation` (`cdecl(world, key)`): [`info_in_world`] on the
/// pathing singleton's info map.
pub fn potential_info_in_world(e: &mut Engine, world: u32, key: u32) -> u32 {
    let pathing = e.global::<u32>(PATHING);
    let map = e.call(PATHING_INFO_MAP, &args![pathing]).u32();
    typed_call!(
        FAMILY,
        0x006b_7850,
        info_in_world(e, Ptr::new(map), Ptr::new(world), key)
    )
}
