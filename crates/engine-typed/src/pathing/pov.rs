//! The point-of-view search's geometry: directions as cosine and sine pairs, arcs, spheres and their
//! tangents, edge references, and the segment tests the path smoother runs.

use super::layout::{Arc, EdgeLocation, EdgePair, MeshEdge, Point2, Sphere, Tangent};
use super::navmesh::{barycentric, cross_2d, xy_loaded, xy_operand};
use super::point::{abs, add, length, length_squared, length_xy_squared, root, scaled, subtract};
use super::{
    copy_point, fcom, fld, fld_local, fstp, m32, m32_local, m64_global, parity_even, through,
    FAMILY, ZERO_DOUBLE,
};
use crate::culling::layout::NiPoint3;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::x87::{x87_add, x87_div, x87_mul, x87_sub};
use engine_core::{args, typed_call, Engine};

/// The `double` of the float 0.001 the crossing test compares with (`01016978`).
const NEAR_ZERO: u32 = 0x0101_6978;

/// Callees.
const ARC_INSIDE: u32 = 0x006a_06f0;
const SINE: u32 = 0x004e_4470;
const COSINE: u32 = 0x004e_44b0;
const DECAL_VERTEX_NEW: u32 = 0x0062_40d0;
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const ARC_NEW: u32 = 0x0058_8c50;
const POINT2_SUBTRACT: u32 = 0x004e_8880;
const POINT2_SCALED: u32 = 0x0053_2fd0;
const POINT2_ADD: u32 = 0x004d_fe30;
const POINT_DIVIDE: u32 = 0x0049_41c0;

/// `ReturnThis` (`006815c0`) on a word.
#[inline(always)]
fn return_this(e: &mut Engine, p: Ptr) {
    typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, p));
}

// Typed from 006985f0
/// Copies a direction (`thiscall(this, other)`), each float through the x87 stack.
pub fn copy_pair(e: &mut Engine, this: Ptr<Point2>, other: Ptr<Point2>) -> Ptr<Point2> {
    let x = fstp(fld(e, other, Point2::x));
    e.set(this, Point2::x, x);
    let y = fstp(fld(e, other, Point2::y));
    e.set(this, Point2::y, y);
    this
}

// Typed from 00698620
/// A direction relative to another (`thiscall(this, out, other)`): `(x*ox + y*oy, y*ox - x*oy)`.
pub fn relative(
    e: &mut Engine,
    this: Ptr<Point2>,
    out: Ptr<Point2>,
    other: Ptr<Point2>,
) -> Ptr<Point2> {
    e.with_stack(8, |e, local| {
        let x = fstp(x87_add(
            x87_mul(fld(e, this, Point2::x), m32(e, other, Point2::x)),
            x87_mul(fld(e, this, Point2::y), m32(e, other, Point2::y)),
        ));
        let y = fstp(x87_sub(
            x87_mul(fld(e, this, Point2::y), m32(e, other, Point2::x)),
            x87_mul(fld(e, this, Point2::x), m32(e, other, Point2::y)),
        ));
        let local = local.cast::<Point2>();
        e.set(local, Point2::x, x);
        e.set(local, Point2::y, y);
        typed_call!(FAMILY, 0x0069_85f0, copy_pair(e, out, local));
    });
    out
}

// Typed from 006a0e90
/// `Angle::operator+` (`thiscall(this, out, other)`): the directions' angles added, `(x*ox - y*oy,
/// y*ox + x*oy)`.
pub fn rotated(
    e: &mut Engine,
    this: Ptr<Point2>,
    out: Ptr<Point2>,
    other: Ptr<Point2>,
) -> Ptr<Point2> {
    e.with_stack(8, |e, local| {
        return_this(e, local);
        let x = fstp(x87_sub(
            x87_mul(fld(e, this, Point2::x), m32(e, other, Point2::x)),
            x87_mul(fld(e, this, Point2::y), m32(e, other, Point2::y)),
        ));
        let y = fstp(x87_add(
            x87_mul(fld(e, this, Point2::y), m32(e, other, Point2::x)),
            x87_mul(fld(e, this, Point2::x), m32(e, other, Point2::y)),
        ));
        let local = local.cast::<Point2>();
        e.set(local, Point2::x, x);
        e.set(local, Point2::y, y);
        typed_call!(FAMILY, 0x0069_85f0, copy_pair(e, out, local));
    });
    out
}

// Typed from 00698680
/// A direction's quadrant bits (`thiscall(this)`): 3 for a negative y, then 1 toggled for a negative x.
pub fn quadrant(e: &mut Engine, this: Ptr<Point2>) -> u32 {
    let zero = m64_global(e, ZERO_DOUBLE);
    let mut bits = 0;
    if !parity_even(fcom(fld(e, this, Point2::y), zero), 0x05) {
        bits ^= 3;
    }
    if !parity_even(fcom(fld(e, this, Point2::x), zero), 0x05) {
        bits ^= 1;
    }
    bits
}

// Typed from 006a0660
/// `Arc::Contains` (`thiscall(arc, direction)`): both the arc's far end and the direction taken
/// relative to its near end (which end is which by the arc's flag), then `006a06f0`.
pub fn arc_contains(e: &mut Engine, this: Ptr<Arc>, direction: Ptr<Point2>) -> bool {
    let near = if e.get(this, Arc::flag) == 0 {
        this.at(Arc::first)
    } else {
        this.at(Arc::second)
    };
    let far = if e.get(this, Arc::flag) == 0 {
        this.at(Arc::second)
    } else {
        this.at(Arc::first)
    };
    e.with_stack(0x10, |e, locals| {
        let (far_relative, relative_direction) =
            (locals.cast::<Point2>(), locals.byte_add(8).cast::<Point2>());
        typed_call!(FAMILY, 0x0069_8620, relative(e, far, far_relative, near));
        typed_call!(
            FAMILY,
            0x0069_8620,
            relative(e, direction, relative_direction, near)
        );
        e.call(ARC_INSIDE, &args![relative_direction, far_relative])
            .u8()
            != 0
    })
}

// Typed from 006a04b0
/// A sphere's constructor (`thiscall(sphere, centre, radius)`): the centre copied as words, the
/// radius through the x87 stack.
pub fn sphere_new(
    e: &mut Engine,
    this: Ptr<Sphere>,
    center: Ptr<NiPoint3>,
    radius: f32,
) -> Ptr<Sphere> {
    copy_point(e, center, this.at(Sphere::center));
    e.set(this, Sphere::radius, through(radius));
    this
}

// Typed from 006a04e0
/// `Angle::Angle` (`thiscall(this, angle)`): its sine (`004e4470`) and cosine (`004e44b0`).
pub fn angle_new(e: &mut Engine, this: Ptr<Point2>, angle: f32) -> Ptr<Point2> {
    let s = fstp(e.call(SINE, &args![through(angle)]).f64());
    e.set(this, Point2::x, s);
    let c = fstp(e.call(COSINE, &args![through(angle)]).f64());
    e.set(this, Point2::y, c);
    this
}

// Typed from 006a05e0
/// An arc's constructor (`thiscall(arc, first, second, flag)`).
pub fn arc_new(
    e: &mut Engine,
    this: Ptr<Arc>,
    first: Ptr<Point2>,
    second: Ptr<Point2>,
    flag: u32,
) -> Ptr<Arc> {
    typed_call!(
        FAMILY,
        0x0069_85f0,
        copy_pair(e, this.at(Arc::first), first)
    );
    typed_call!(
        FAMILY,
        0x0069_85f0,
        copy_pair(e, this.at(Arc::second), second)
    );
    e.set(this, Arc::flag, flag);
    this
}

// Typed from 006a0620
/// An arc's assignment (`thiscall(arc, other)`).
pub fn arc_assign(e: &mut Engine, this: Ptr<Arc>, other: Ptr<Arc>) -> Ptr<Arc> {
    typed_call!(
        FAMILY,
        0x0069_85f0,
        copy_pair(e, this.at(Arc::first), other.at(Arc::first))
    );
    typed_call!(
        FAMILY,
        0x0069_85f0,
        copy_pair(e, this.at(Arc::second), other.at(Arc::second))
    );
    let flag = e.get(other, Arc::flag);
    e.set(this, Arc::flag, flag);
    this
}

// Typed from 006a0480
/// A search vertex's constructor (`thiscall(this)`): the point part (`006240d0`), then `ReturnThis` on
/// the part at +0xc.
pub fn vertex_new(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(DECAL_VERTEX_NEW, &args![this]);
    return_this(e, this.byte_add(0xc));
    this
}

// Typed from 006a0870
/// `SphereTangents::SphereTangents` (`thiscall(this)`): two arcs made by the vector constructor.
pub fn tangents_new(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCT,
        &args![this.byte_add(4), 0x14u32, 2u32, ARC_NEW],
    );
    this
}

// Typed from 006c4350
/// A mesh edge's copy constructor (`thiscall(this, other)`): `ReturnThis`, then the three fields.
pub fn mesh_edge_new(e: &mut Engine, this: Ptr<MeshEdge>, other: Ptr<MeshEdge>) -> Ptr<MeshEdge> {
    return_this(e, this.cast());
    copy_edge_fields(e, this, other);
    this
}

/// The three fields of a mesh edge copied.
#[inline(always)]
fn copy_edge_fields(e: &mut Engine, this: Ptr<MeshEdge>, other: Ptr<MeshEdge>) {
    let mesh = e.get(other, MeshEdge::mesh);
    e.set(this, MeshEdge::mesh, mesh);
    let t = e.get(other, MeshEdge::triangle);
    e.set(this, MeshEdge::triangle, t);
    let x = e.get(other, MeshEdge::extra);
    e.set(this, MeshEdge::extra, x);
}

// Typed from 006bc870
/// A mesh edge's assignment (`thiscall(this, other)`).
pub fn mesh_edge_copy(e: &mut Engine, this: Ptr<MeshEdge>, other: Ptr<MeshEdge>) -> Ptr<MeshEdge> {
    copy_edge_fields(e, this, other);
    this
}

// Typed from 0069a690
/// A triangle reference's assignment (`thiscall(this, other)`): the mesh and triangle.
pub fn triangle_ref_copy(
    e: &mut Engine,
    this: Ptr<MeshEdge>,
    other: Ptr<MeshEdge>,
) -> Ptr<MeshEdge> {
    let mesh = e.get(other, MeshEdge::mesh);
    e.set(this, MeshEdge::mesh, mesh);
    let t = e.get(other, MeshEdge::triangle);
    e.set(this, MeshEdge::triangle, t);
    this
}

// Typed from 0069a660
/// A triangle reference's constructor (`thiscall(this, mesh, triangle)`).
pub fn triangle_ref_set(
    e: &mut Engine,
    this: Ptr<MeshEdge>,
    mesh: u32,
    index: u16,
) -> Ptr<MeshEdge> {
    e.set(this, MeshEdge::mesh, mesh);
    e.set(this, MeshEdge::triangle, index);
    this
}

// Typed from 0069a6c0
/// A mesh edge's constructor (`thiscall(this, mesh, triangle, edge)`): the fields (the exe's `ReturnThis` first does nothing and is left out, as the translation leaves it).
pub fn mesh_edge_set(
    e: &mut Engine,
    this: Ptr<MeshEdge>,
    mesh: u32,
    index: u16,
    extra: u32,
) -> Ptr<MeshEdge> {
    e.set(this, MeshEdge::mesh, mesh);
    e.set(this, MeshEdge::triangle, index);
    e.set(this, MeshEdge::extra, extra);
    this
}

// Typed from 006b39a0
/// An edge pair's assignment (`thiscall(this, other)`): the triangle reference, the edge, three words.
pub fn edge_pair_copy(e: &mut Engine, this: Ptr<EdgePair>, other: Ptr<EdgePair>) -> Ptr<EdgePair> {
    typed_call!(
        FAMILY,
        0x0069_a690,
        triangle_ref_copy(e, this.cast(), other.cast())
    );
    typed_call!(
        FAMILY,
        0x006b_c870,
        mesh_edge_copy(e, this.at(EdgePair::edge), other.at(EdgePair::edge))
    );
    for f in [EdgePair::word_14, EdgePair::word_18, EdgePair::word_1c] {
        let w = e.get(other, f);
        e.set(this, f, w);
    }
    this
}

// Typed from 006a0c10
/// `EdgeLocation::EdgeLocation` (`thiscall(this, edge, point)`): the edge ([`mesh_edge_new`]) and the
/// point copied as words.
pub fn edge_location_new(
    e: &mut Engine,
    this: Ptr<EdgeLocation>,
    edge: Ptr<MeshEdge>,
    point: Ptr<NiPoint3>,
) -> Ptr<EdgeLocation> {
    typed_call!(
        FAMILY,
        0x006c_4350,
        mesh_edge_new(e, this.at(EdgeLocation::edge), edge)
    );
    copy_point(e, point, this.at(EdgeLocation::point));
    this
}

// Typed from 006a0e50
/// `Edge::Edge` (`thiscall(this, other)`): an edge location's copy.
pub fn edge_location_copy(
    e: &mut Engine,
    this: Ptr<EdgeLocation>,
    other: Ptr<EdgeLocation>,
) -> Ptr<EdgeLocation> {
    typed_call!(
        FAMILY,
        0x006c_4350,
        mesh_edge_new(e, this.at(EdgeLocation::edge), other.at(EdgeLocation::edge))
    );
    copy_point(
        e,
        other.at(EdgeLocation::point),
        this.at(EdgeLocation::point),
    );
    this
}

/// `NiPoint3::Dot` (`004b6190`).
#[inline(always)]
fn dot(e: &mut Engine, a: Ptr<NiPoint3>, b: Ptr<NiPoint3>) -> f32 {
    typed_call!(leaf::FAMILY, 0x004b_6190, leaf::point3_dot(e, a, b))
}

// Typed from 006b9b10
/// The distance from a point to a segment (`cdecl(point, start, end, inside)`): to the start when the
/// projection `t = (point - start) . (end - start)` is not above zero, to the end when it is not below
/// the squared length, else to the start plus `t / length` (in `double`, stored to a float) times the
/// segment; `inside` (when not null) says which.
pub fn distance_to_segment(
    e: &mut Engine,
    point: Ptr<NiPoint3>,
    start: Ptr<NiPoint3>,
    end: Ptr<NiPoint3>,
    inside: Ptr,
) -> f32 {
    e.with_stack(0x48, |e, locals| {
        let slot = |i: u32| locals.byte_add(i * 0xc).cast::<NiPoint3>();
        let (segment, offset, other, product, sum, last) =
            (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, end, segment, start));
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, point, offset, start));
        let t = f64::from(dot(e, offset, segment));
        let set_inside = |e: &mut Engine, v: u8| {
            if !inside.is_null() {
                e.mem.set_u8(inside.addr(), v);
            }
        };
        // FLDZ; FCOMP t; TEST AH,1: taken when zero is below t or unordered.
        if fcom(0.0, t) & 0x01 == 0 {
            set_inside(e, 0);
            let d = typed_call!(FAMILY, 0x0043_9ef0, subtract(e, point, other, start));
            return typed_call!(FAMILY, 0x0045_7990, length(e, d));
        }
        let squared = f64::from(dot(e, segment, segment));
        if fcom(t, squared) & 0x01 == 0 {
            set_inside(e, 0);
            let d = typed_call!(FAMILY, 0x0043_9ef0, subtract(e, point, other, end));
            return typed_call!(FAMILY, 0x0045_7990, length(e, d));
        }
        let fraction = fstp(x87_div(t, squared));
        let s = typed_call!(
            FAMILY,
            0x004a_3760,
            scaled(e, product, through(fraction), segment)
        );
        typed_call!(FAMILY, 0x0043_9e90, add(e, start, sum, s));
        set_inside(e, 1);
        let d = typed_call!(FAMILY, 0x0043_9ef0, subtract(e, point, last, sum));
        typed_call!(FAMILY, 0x0045_7990, length(e, d))
    })
}

/// `(a - o) x (b - o)` of three points by their x and y, stored to a float.
#[inline(always)]
fn side(e: &Engine, a: Ptr<NiPoint3>, b: Ptr<NiPoint3>, o: Ptr<NiPoint3>) -> f32 {
    fstp(cross_2d(xy_loaded(e, a), xy_loaded(e, b), xy_operand(e, o)))
}

/// `FLD p; FMUL q; FCOMP 0.0` and the status bits.
#[inline(always)]
fn product_status(e: &Engine, p: f32, q: f32) -> u8 {
    fcom(
        x87_mul(fld_local(p), m32_local(q)),
        m64_global(e, ZERO_DOUBLE),
    )
}

// Typed from 006ba240
/// `NavMeshMath::Find2DIntersectionStable` (`cdecl(a, b, c, d, out)`): where segment `ab` crosses
/// segment `cd` in x and y, `a + t * (b - a)` with `t` from the side tests; false when the ends of
/// either lie on one side (or a test is a NaN), or the side of `d` from `ab` or of `b` from `cd` is an
/// exact zero.
pub fn intersection(
    e: &mut Engine,
    a: Ptr<NiPoint3>,
    b: Ptr<NiPoint3>,
    c: Ptr<NiPoint3>,
    d: Ptr<NiPoint3>,
    out: Ptr<NiPoint3>,
) -> bool {
    let ab_d = side(e, a, b, d);
    let ab_c = side(e, a, b, c);
    let cd_a = side(e, c, d, a);
    let cd_b = side(e, c, d, b);
    if parity_even(product_status(e, ab_d, ab_c), 0x41) {
        return false;
    }
    if parity_even(product_status(e, cd_a, cd_b), 0x41) {
        return false;
    }
    let zero = m64_global(e, ZERO_DOUBLE);
    if !parity_even(fcom(fld_local(ab_d), zero), 0x44) {
        return false;
    }
    if !parity_even(fcom(fld_local(cd_b), zero), 0x44) {
        return false;
    }
    let t = fstp(x87_div(
        m32_local(cd_a),
        x87_sub(fld_local(cd_a), m32_local(cd_b)),
    ));
    e.with_stack(0x24, |e, locals| {
        let slot = |i: u32| locals.byte_add(i * 0xc).cast::<NiPoint3>();
        let diff = typed_call!(FAMILY, 0x0043_9ef0, subtract(e, b, slot(0), a));
        let s = typed_call!(FAMILY, 0x004a_3760, scaled(e, slot(1), through(t), diff));
        let r = typed_call!(FAMILY, 0x0043_9e90, add(e, a, slot(2), s));
        copy_point(e, r, out);
    });
    true
}

// Typed from 006b9e90
/// Whether the segment `p1 p2` crosses the segment `p3 p4` in x and y (`cdecl(p1, p2, p3, p4, out)`),
/// and where (`004e8880`, `00532fd0`, `004dfe30`, two floats to `out`); a side within 0.001 of the
/// other two's difference is measured again from `p2`.
pub fn crossing(
    e: &mut Engine,
    p1: Ptr<Point2>,
    p2: Ptr<Point2>,
    p3: Ptr<Point2>,
    p4: Ptr<Point2>,
    out: Ptr<Point2>,
) -> bool {
    let (q1, q2, q3, q4) = (p1.cast(), p2.cast(), p3.cast(), p4.cast());
    let s4 = side(e, q1, q2, q4);
    let s3 = side(e, q1, q2, q3);
    if parity_even(product_status(e, s4, s3), 0x05) {
        return false;
    }
    let s1 = side(e, q3, q4, q1);
    let mut s2 = fstp(x87_sub(
        x87_add(fld_local(s1), m32_local(s3)),
        m32_local(s4),
    ));
    let a = typed_call!(FAMILY, 0x0040_8840, abs(e, through(s2)));
    if !parity_even(fcom(f64::from(a), m64_global(e, NEAR_ZERO)), 0x05) {
        s2 = side(e, q3, q4, q2);
    }
    if parity_even(product_status(e, s1, s2), 0x05) {
        return false;
    }
    let t = fstp(x87_div(
        m32_local(s1),
        x87_sub(fld_local(s1), m32_local(s2)),
    ));
    e.with_stack(0x18, |e, locals| {
        let difference = e.call(POINT2_SUBTRACT, &args![p2, locals, p1]).u32();
        let s = e
            .call(
                POINT2_SCALED,
                &args![locals.byte_add(8), through(t), difference],
            )
            .u32();
        let r = Ptr::<Point2>::new(
            e.call(POINT2_ADD, &args![p1, locals.byte_add(0x10), s])
                .u32(),
        );
        let x = e.get(r, Point2::x);
        let y = e.get(r, Point2::y);
        e.set(out, Point2::x, x);
        e.set(out, Point2::y, y);
    });
    true
}

// Typed from 006b9cf0
/// The height difference over a triangle given by its corners (`cdecl(p1, p2, p3, point, out, above,
/// below)`): as `NavMesh::ComputeTriangleZForLocation`, with `height - point.z` tested and, when it
/// passes, stored to `out`.
#[allow(clippy::too_many_arguments)]
pub fn height_in(
    e: &mut Engine,
    p1: Ptr<NiPoint3>,
    p2: Ptr<NiPoint3>,
    p3: Ptr<NiPoint3>,
    point: Ptr<NiPoint3>,
    out: Ptr,
    above: f32,
    below: f32,
) -> bool {
    let mut difference = 0.0f32;
    let inside = barycentric(e, [p1, p2, p3], point, above, below, |e, height| {
        difference = fstp(x87_sub(fld_local(height), m32(e, point, NiPoint3::z)));
        difference
    });
    if inside {
        e.set(out.cast::<Point2>(), Point2::x, through(difference));
    }
    inside
}

// Typed from 006bab00
/// Whether a segment comes within a radius of a centre (`cdecl(centre, radius, a, b)`): the segment's
/// point nearest the centre (`a`, `b`, or `a + t * (b - a)` with `t` stored to a float), its squared
/// distance below `radius * radius`.
pub fn segment_within(
    e: &mut Engine,
    center: Ptr<NiPoint3>,
    radius: f32,
    a: Ptr<NiPoint3>,
    b: Ptr<NiPoint3>,
) -> bool {
    e.with_stack(0x48, |e, locals| {
        let slot = |i: u32| locals.byte_add(i * 0xc).cast::<NiPoint3>();
        let (segment, offset, nearest, product, sum, difference) =
            (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, b, segment, a));
        typed_call!(FAMILY, 0x0043_9ef0, subtract(e, center, offset, a));
        return_this(e, nearest.cast());
        let t = fstp(f64::from(dot(e, segment, offset)));
        let zero = m64_global(e, ZERO_DOUBLE);
        if !parity_even(fcom(fld_local(t), zero), 0x41) {
            copy_point(e, a, nearest);
        } else {
            let squared = fstp(f64::from(dot(e, segment, segment)));
            if fcom(fld_local(t), fld_local(squared)) & 0x01 == 0 {
                copy_point(e, b, nearest);
            } else {
                let fraction = fstp(x87_div(fld_local(t), m32_local(squared)));
                let s = typed_call!(
                    FAMILY,
                    0x004a_3760,
                    scaled(e, product, through(fraction), segment)
                );
                let p = typed_call!(FAMILY, 0x0043_9e90, add(e, a, sum, s));
                copy_point(e, p, nearest);
            }
        }
        let d = typed_call!(
            FAMILY,
            0x0043_9ef0,
            subtract(e, center, difference, nearest)
        );
        let squared = typed_call!(FAMILY, 0x004a_7290, length_squared(e, d));
        let r2 = x87_mul(fld_local(radius), m32_local(radius));
        fcom(r2, f64::from(squared)) & 0x41 == 0
    })
}

/// One tangent's two directions from the sphere difference `(dx, dy)`, the radius term `k`, the root
/// `w` and the inverse squared length `inv`: `(k*dx + dy*w) * inv, (k*dy - dx*w) * inv` on one side and
/// the signs of the `w` terms swapped on the other.
fn tangent(k: f32, dx: f32, dy: f32, w: f32, inv: f32, plus_first: bool) -> (f32, f32) {
    let a = x87_mul(fld_local(k), m32_local(dx));
    let b = x87_mul(fld_local(dy), m32_local(w));
    let first = if plus_first {
        x87_add(a, b)
    } else {
        x87_sub(a, b)
    };
    let x = fstp(x87_mul(first, m32_local(inv)));
    let c = x87_mul(fld_local(k), m32_local(dy));
    let d = x87_mul(fld_local(dx), m32_local(w));
    let second = if plus_first {
        x87_sub(c, d)
    } else {
        x87_add(c, d)
    };
    let y = fstp(x87_mul(second, m32_local(inv)));
    (x, y)
}

// Typed from 006bb570
/// `NavMeshPOVSearch::FindSphereTangents` (`cdecl(first, second, side, out)`): the tangents common to
/// two spheres in x and y, outer then inner, each with its side and both directions (the inner's
/// second one negated), counted at `out`. The centres' difference is divided by the radius of the
/// sphere that has one (`004941c0`), the other radius taken relative to it.
pub fn sphere_tangents(
    e: &mut Engine,
    first: Ptr<Sphere>,
    second: Ptr<Sphere>,
    which: u32,
    out: Ptr,
) -> u32 {
    e.with_stack(0xc, |e, diff| {
        let diff = diff.cast::<NiPoint3>();
        let mut count = 0u32;
        typed_call!(
            FAMILY,
            0x0043_9ef0,
            subtract(e, second.at(Sphere::center), diff, first.at(Sphere::center))
        );
        let mut r1 = through(e.get(first, Sphere::radius));
        let mut r2 = through(e.get(second, Sphere::radius));
        let zero = m64_global(e, ZERO_DOUBLE);
        if parity_even(fcom(fld(e, second, Sphere::radius), zero), 0x44) {
            r2 = 1.0;
            r1 = fstp(x87_div(
                fld(e, first, Sphere::radius),
                m32(e, second, Sphere::radius),
            ));
            let r = through(e.get(second, Sphere::radius));
            e.call(POINT_DIVIDE, &args![diff, r]);
        } else if parity_even(fcom(fld(e, first, Sphere::radius), zero), 0x44) {
            r1 = 1.0;
            r2 = fstp(x87_div(
                fld(e, second, Sphere::radius),
                m32(e, first, Sphere::radius),
            ));
            let r = through(e.get(first, Sphere::radius));
            e.call(POINT_DIVIDE, &args![diff, r]);
        }
        let squared = typed_call!(FAMILY, 0x0059_5c80, length_xy_squared(e, diff));
        let squared = fstp(f64::from(squared));
        if !parity_even(fcom(fld_local(squared), zero), 0x44) {
            e.mem.set_u32(out.addr(), 0);
            return 0;
        }
        let dx = e.get(diff, NiPoint3::x);
        let dy = e.get(diff, NiPoint3::y);
        let list = out.byte_add(4).cast::<Tangent>();
        let mut one = |e: &mut Engine, k: f32, outer: bool| {
            let k2 = fstp(x87_mul(fld_local(k), m32_local(k)));
            // FLD squared; FLD k2; FCOMPP; TEST AH,0x41; JP: skipped when k2 is above or unordered.
            if parity_even(fcom(fld_local(k2), fld_local(squared)), 0x41) {
                return;
            }
            let inv = fstp(x87_div(1.0, fld_local(squared)));
            let rest = fstp(x87_sub(fld_local(squared), m32_local(k2)));
            let w = typed_call!(FAMILY, 0x0040_19b0, root(e, through(rest)));
            let w = fstp(f64::from(w));
            let entry = list.index(count, 0x14);
            // The outer tangent's side is 0 for side 0; the inner one's the other way round.
            let flag = u32::from((which != 0) == outer);
            e.set(entry, Tangent::side, flag);
            let (x, y) = tangent(k, dx, dy, w, inv, which == 0);
            e.set(entry, Tangent::x, x);
            e.set(entry, Tangent::y, y);
            if outer {
                let x2 = fstp(fld(e, entry, Tangent::x));
                e.set(entry, Tangent::x2, x2);
                let y2 = fstp(fld(e, entry, Tangent::y));
                e.set(entry, Tangent::y2, y2);
            } else {
                let x2 = fstp(-fld(e, entry, Tangent::x));
                e.set(entry, Tangent::x2, x2);
                let y2 = fstp(-fld(e, entry, Tangent::y));
                e.set(entry, Tangent::y2, y2);
            }
            count += 1;
        };
        let outer = fstp(x87_sub(fld_local(r1), m32_local(r2)));
        one(e, outer, true);
        let inner = fstp(x87_add(fld_local(r1), m32_local(r2)));
        one(e, inner, false);
        e.mem.set_u32(out.addr(), count);
        count
    })
}
