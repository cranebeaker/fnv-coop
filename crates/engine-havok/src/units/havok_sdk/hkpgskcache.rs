//! `Havok SDK/hkpgskcache.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is an `f32` operation, the three-component sums add as
//! `z + (y + x)`, and `MOVAPS` copies move the bit patterns unchanged.

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable slots of the convex shapes (`hkpConvexShape`): `+0x2c` the number of dimensions the
/// shape's vertices span (no arguments), `+0x38` the supporting vertex in a direction
/// (`(&direction, &vertexOut)`) and `+0x3c` the conversion of vertex ids into vertices
/// (`(&ids, count, &verticesOut)`).
const SLOT_DIMENSIONS: u32 = 0x2c;
const SLOT_SUPPORTING_VERTEX: u32 = 0x38;
const SLOT_CONVERT_VERTEX_IDS: u32 = 0x3c;

type Vec4 = [f32; 4];

layout! {
    /// `hkpGskCache` (Xbox PDB), 0xC bytes.
    pub struct HkpGskCache: 0xc {
        /// `m_vertices[0]` (Xbox PDB): the vertex id of shape A's simplex vertex.
        0x00 m_vertices_0: u16,
        /// `m_vertices[1]`: shape B's vertex id (or the triangle's first vertex).
        0x02 m_vertices_1: u16,
        0x04 m_vertices_2: u16,
        0x06 m_vertices_3: u16,
        /// `m_dimA` (Xbox PDB).
        0x08 m_dimA: u8,
        /// `m_dimB` (Xbox PDB).
        0x09 m_dimB: u8,
        /// `m_maxDimA` (high nibble) and `m_maxDimB` (low nibble) of the byte (Xbox PDB).
        0x0a m_maxDims: u8,
        /// `m_gskFlags` (Xbox PDB).
        0x0b m_gskFlags: u8,
    }
}

fn load(e: &Engine, at: u32) -> Vec4 {
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn store(e: &mut Engine, at: u32, value: Vec4) {
    for (lane, component) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, component);
    }
}

fn zip(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    [f(a[0], b[0]), f(a[1], b[1]), f(a[2], b[2]), f(a[3], b[3])]
}

/// The point `vertex` carried by the transform rows `r0`, `r1`, `r2` and translation `t` the way
/// the code multiplies it: `((y * r1 + x * r0) + (z * r2 + t))` in every lane.
fn transform_point(vertex: Vec4, rows: [Vec4; 3], translation: Vec4) -> Vec4 {
    let mut result = [0.0; 4];
    for (lane, slot) in result.iter_mut().enumerate() {
        let yx = vertex[1] * rows[1][lane] + vertex[0] * rows[0][lane];
        let zt = vertex[2] * rows[2][lane] + translation[lane];
        *slot = yx + zt;
    }
    result
}

/// The dimension counts of both shapes packed into one byte: shape A (clamped to 15 above 3)
/// in the low nibble, shape B in the high nibble.
fn pack_max_dims(dim_a: i32, dim_b: i32) -> u8 {
    let clamp = |dim: i32| if dim > 3 { 0xf } else { dim };
    (clamp(dim_a) as u8 & 0xf) | ((clamp(dim_b) as u8) << 4)
}

// Translated from 00dafcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskCache::init` (Xbox PDB): fills the cache for two convex shapes whose relative
/// transform is `transform` (an `hkTransform`: three rotation rows then the translation).
/// Takes vertex 0 of both shapes, carries B's vertex into A's space, asks A for its supporting
/// vertex towards it, carries the difference back through the transposed rotation and asks B
/// for its supporting vertex against it. The cache keeps the two vertex ids (the low words of
/// the supporting vertices), zero in the other two, a dimension of 1 for each shape and the
/// shapes' dimension counts packed into the maximum-dimension byte.
pub fn hkp_gsk_cache_init(
    e: &mut Engine,
    this: Ptr<HkpGskCache>,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
) {
    // The stack frame of the exe, at the same offsets.
    e.with_stack(0x70, |e, frame| {
        let base = frame.addr();
        let ids = base + 0xc;
        let direction_a = base + 0x10;
        let support_a = base + 0x20;
        let vertex_b = base + 0x30;
        let vertex_a = base + 0x40;
        let work = base + 0x50;
        let support_b = base + 0x60;

        e.vcall(
            shape_a.addr(),
            SLOT_CONVERT_VERTEX_IDS,
            &args![ids, 1u32, vertex_a],
        );
        e.vcall(
            shape_b.addr(),
            SLOT_CONVERT_VERTEX_IDS,
            &args![ids, 1u32, vertex_b],
        );
        let rows = [
            load(e, transform.addr()),
            load(e, transform.addr() + 0x10),
            load(e, transform.addr() + 0x20),
        ];
        let translation = load(e, transform.addr() + 0x30);
        let b_in_a = transform_point(load(e, vertex_b), rows, translation);
        let a = load(e, vertex_a);
        store(e, work, b_in_a);
        store(e, direction_a, zip(b_in_a, a, |x, y| x - y));
        e.vcall(
            shape_a.addr(),
            SLOT_SUPPORTING_VERTEX,
            &args![direction_a, support_a],
        );

        let support = load(e, support_a);
        let difference = zip(load(e, work), support, |x, y| x - y);
        let rows = [
            load(e, transform.addr()),
            load(e, transform.addr() + 0x10),
            load(e, transform.addr() + 0x20),
        ];
        // The columns of the rotation (the transposed rows), with the fourth lane the code's
        // shuffles leave: (a0, b0, c0, c1), (a1, b1, c1, c3), (a2, b2, c2, c3).
        let [r0, r1, r2] = rows;
        let columns = [
            [r0[0], r1[0], r2[0], r2[1]],
            [r0[1], r1[1], r2[1], r2[3]],
            [r0[2], r1[2], r2[2], r2[3]],
        ];
        let mut back = [0.0f32; 4];
        for (lane, slot) in back.iter_mut().enumerate() {
            let yx = difference[1] * columns[1][lane] + difference[0] * columns[0][lane];
            *slot = yx + difference[2] * columns[2][lane];
        }
        // XORPS with the sign mask: negate every lane.
        let negated = back.map(|lane| f32::from_bits(lane.to_bits() ^ 0x8000_0000));
        store(e, work, negated);
        e.vcall(
            shape_b.addr(),
            SLOT_SUPPORTING_VERTEX,
            &args![work, support_b],
        );

        let id_b = e.mem.u16(support_b + 0xc);
        let id_a = e.mem.u16(support_a + 0xc);
        e.set(this, HkpGskCache::m_vertices_1, id_b);
        e.set(this, HkpGskCache::m_vertices_0, id_a);
        e.set(this, HkpGskCache::m_vertices_3, 0);
        e.set(this, HkpGskCache::m_vertices_2, 0);
        e.set(this, HkpGskCache::m_dimA, 1);
        e.set(this, HkpGskCache::m_dimB, 1);
        let dim_a = e.vcall(shape_a.addr(), SLOT_DIMENSIONS, &args![]).i32();
        let dim_b = e.vcall(shape_b.addr(), SLOT_DIMENSIONS, &args![]).i32();
        e.set(this, HkpGskCache::m_maxDims, pack_max_dims(dim_a, dim_b));
        e.set(this, HkpGskCache::m_gskFlags, 0);
    });
}

// Translated from 00dafe70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGskCache::initTriangle` (Xbox PDB): when the byte at `triangle + 0x17` is set the shape
/// is treated as a general convex shape and `hkp_gsk_cache_init` runs. Otherwise the triangle's
/// three vertices (the vectors at `triangle + 0x20`, `+0x30` and `+0x40`) are carried through
/// `transform`, the normal `(p1 - p0) x (p2 - p1)` is flipped to point the same way as
/// `vertex0(A) - p0` (its dot product's sign), and A is asked for its supporting vertex in
/// that direction. The cache keeps A's vertex id, the triangle's vertex ids 0, 1, 2, a
/// dimension of 1 for A and 3 for the triangle, and A's dimension count (clamped to 15
/// above 3) in the low nibble of the maximum-dimension byte over a high nibble of 3.
pub fn hkp_gsk_cache_init_triangle(
    e: &mut Engine,
    this: Ptr<HkpGskCache>,
    shape_a: Ptr,
    triangle: Ptr,
    transform: Ptr,
) {
    if e.mem.u8(triangle.addr() + 0x17) != 0 {
        hkp_gsk_cache_init(e, this, shape_a, triangle, transform);
        return;
    }
    e.set(this, HkpGskCache::m_vertices_1, 0);
    e.set(this, HkpGskCache::m_vertices_2, 1);
    e.set(this, HkpGskCache::m_vertices_3, 2);

    let rows = [
        load(e, transform.addr()),
        load(e, transform.addr() + 0x10),
        load(e, transform.addr() + 0x20),
    ];
    let translation = load(e, transform.addr() + 0x30);
    // The loop runs from the last vertex to the first.
    let mut points = [[0.0f32; 4]; 3];
    for index in (0..3).rev() {
        let vertex = load(e, triangle.addr() + 0x20 + 0x10 * index as u32);
        points[index] = transform_point(vertex, rows, translation);
    }
    let [p0, p1, p2] = points;
    let first_edge = zip(p1, p0, |x, y| x - y);
    let second_edge = zip(p2, p1, |x, y| x - y);
    // Lane-wise `e1.zxy * e0.yzx - e1.yzx * e0.zxy` (the fourth lane cancels to 0 or NaN).
    #[allow(clippy::eq_op)] // the exe computes the fourth lane too (0, or NaN for inf/NaN input)
    let normal = [
        second_edge[2] * first_edge[1] - second_edge[1] * first_edge[2],
        second_edge[0] * first_edge[2] - second_edge[2] * first_edge[0],
        second_edge[1] * first_edge[0] - second_edge[0] * first_edge[1],
        second_edge[3] * first_edge[3] - second_edge[3] * first_edge[3],
    ];

    e.with_stack(0x80, |e, frame| {
        let base = frame.addr();
        let support = base + 0x20;
        let vertex = base + 0x30;
        let direction = base + 0x40;
        e.vcall(
            shape_a.addr(),
            SLOT_CONVERT_VERTEX_IDS,
            &args![this.addr() + 2, 1u32, vertex],
        );
        let offset = zip(load(e, vertex), p0, |x, y| x - y);
        let products = zip(offset, normal, |x, y| x * y);
        let dot = products[2] + (products[1] + products[0]);
        // ANDPS / XORPS with the sign mask: the normal takes the sign of the dot product.
        let sign = dot.to_bits() & 0x8000_0000;
        let flipped = normal.map(|lane| f32::from_bits(lane.to_bits() ^ sign));
        store(e, direction, flipped);
        e.vcall(
            shape_a.addr(),
            SLOT_SUPPORTING_VERTEX,
            &args![direction, support],
        );
        let id_a = e.mem.u16(support + 0xc);
        e.set(this, HkpGskCache::m_vertices_0, id_a);
    });
    e.set(this, HkpGskCache::m_dimA, 1);
    e.set(this, HkpGskCache::m_dimB, 3);
    let dim_a = e.vcall(shape_a.addr(), SLOT_DIMENSIONS, &args![]).i32();
    let clamped = if dim_a > 3 { 0xf } else { dim_a };
    e.set(this, HkpGskCache::m_maxDims, (clamped as u8 & 0xf) | 0x30);
    e.set(this, HkpGskCache::m_gskFlags, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dafcd0,
            hkp_gsk_cache_init(Ptr<HkpGskCache>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00dafe70,
            hkp_gsk_cache_init_triangle(Ptr<HkpGskCache>, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SHAPE_A_SUPPORT: u32 = 0x0900_a038;
    const SHAPE_A_CONVERT: u32 = 0x0900_a03c;
    const SHAPE_A_DIMENSIONS: u32 = 0x0900_a02c;
    const SHAPE_B_SUPPORT: u32 = 0x0900_b038;
    const SHAPE_B_CONVERT: u32 = 0x0900_b03c;
    const SHAPE_B_DIMENSIONS: u32 = 0x0900_b02c;

    type Seen = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// A convex shape double: converting vertex ids yields `vertex` (xyz, then the id in the
    /// low word of the fourth lane), the supporting vertex is `support` with id `support_id`,
    /// and it has `dimensions` dimensions. Calls are logged with the direction words read at
    /// call time.
    #[allow(clippy::too_many_arguments)]
    fn shape(
        e: &mut Engine,
        slots: [u32; 3],
        name: &'static str,
        vertex: [f32; 3],
        support: [f32; 3],
        support_id: u32,
        dimensions: u32,
        seen: &Seen,
    ) -> Ptr {
        let log = seen.clone();
        e.register_double(slots[1], move |e, a| {
            log.borrow_mut().push((name, vec![a[1], a[2]]));
            for (i, v) in vertex.into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, v);
            }
            e.mem.set_u32(a[3] + 12, 0x11);
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(slots[0], move |e, a| {
            let direction: Vec<u32> = (0..3).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut().push((name, direction));
            for (i, v) in support.into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, v);
            }
            e.mem.set_u32(a[2] + 12, support_id);
            Ret::default()
        });
        e.register_double(slots[2], move |_, _| Ret {
            eax: dimensions,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + SLOT_SUPPORTING_VERTEX, slots[0]);
        e.mem.set_u32(table + SLOT_CONVERT_VERTEX_IDS, slots[1]);
        e.mem.set_u32(table + SLOT_DIMENSIONS, slots[2]);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        Ptr::new(object)
    }

    /// A transform whose rows are `rows` and whose translation is `translation`.
    fn transform(e: &mut Engine, rows: [[f32; 3]; 3], translation: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x40);
        for (i, row) in rows.into_iter().enumerate() {
            for (j, v) in row.into_iter().enumerate() {
                e.mem.set_f32(at + 16 * i as u32 + 4 * j as u32, v);
            }
        }
        for (j, v) in translation.into_iter().enumerate() {
            e.mem.set_f32(at + 0x30 + 4 * j as u32, v);
        }
        e.mem.set_f32(at + 0x3c, 1.0);
        Ptr::new(at)
    }

    const PERMUTATION: [[f32; 3]; 3] = [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]];

    fn bits(values: [f32; 3]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    #[test]
    fn init_follows_the_supporting_vertices_through_the_rotation() {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let a = shape(
            &mut e,
            [SHAPE_A_SUPPORT, SHAPE_A_CONVERT, SHAPE_A_DIMENSIONS],
            "a",
            [1.0, 2.0, 3.0],
            [7.0, 8.0, 9.0],
            0x22,
            2,
            &seen,
        );
        let b = shape(
            &mut e,
            [SHAPE_B_SUPPORT, SHAPE_B_CONVERT, SHAPE_B_DIMENSIONS],
            "b",
            [4.0, 5.0, 6.0],
            [0.0, 0.0, 0.0],
            0x33,
            5,
            &seen,
        );
        let matrix = transform(&mut e, PERMUTATION, [10.0, 20.0, 30.0]);
        let cache: Ptr<HkpGskCache> = e.new_object();
        e.call(0x00dafcd0, &args![cache, a, b, matrix]);
        // B's vertex in A's space is (6 + 10, 4 + 20, 5 + 30) = (16, 24, 35); A is asked for
        // its supporting vertex towards (16, 24, 35) - (1, 2, 3).
        let seen = seen.borrow();
        let a_direction = seen.iter().find(|(name, v)| *name == "a" && v.len() == 3);
        assert_eq!(a_direction.unwrap().1, bits([15.0, 22.0, 32.0]));
        // d = (16, 24, 35) - (7, 8, 9) = (9, 16, 26); back through the columns it becomes
        // (d.y, d.z, d.x) = (16, 26, 9), and B is asked about its negation.
        let b_direction = seen.iter().find(|(name, v)| *name == "b" && v.len() == 3);
        assert_eq!(b_direction.unwrap().1, bits([-16.0, -26.0, -9.0]));
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_0), 0x22);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_1), 0x33);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_2), 0);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_3), 0);
        assert_eq!(e.get(cache, HkpGskCache::m_dimA), 1);
        assert_eq!(e.get(cache, HkpGskCache::m_dimB), 1);
        // Shape A has 2 dimensions (low nibble), shape B 5, clamped to 15 (high nibble).
        assert_eq!(e.get(cache, HkpGskCache::m_maxDims), 0xf2);
        assert_eq!(e.get(cache, HkpGskCache::m_gskFlags), 0);
    }

    #[test]
    fn dimension_nibbles_clamp_above_three() {
        assert_eq!(pack_max_dims(3, 3), 0x33);
        assert_eq!(pack_max_dims(4, 0), 0x0f);
        assert_eq!(pack_max_dims(0, 7), 0xf0);
    }

    /// A triangle shape double: vertices at +0x20, +0x30, +0x40 and the flag byte at +0x17.
    fn triangle(e: &mut Engine, vertices: [[f32; 3]; 3], flag: u8) -> Ptr {
        let at = e.mem.alloc(0x60);
        for (i, vertex) in vertices.into_iter().enumerate() {
            for (j, v) in vertex.into_iter().enumerate() {
                e.mem.set_f32(at + 0x20 + 0x10 * i as u32 + 4 * j as u32, v);
            }
        }
        e.mem.set_u8(at + 0x17, flag);
        Ptr::new(at)
    }

    const IDENTITY: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    fn triangle_scene(vertex: [f32; 3], dimensions: u32) -> (Engine, Ptr<HkpGskCache>, Seen) {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let a = shape(
            &mut e,
            [SHAPE_A_SUPPORT, SHAPE_A_CONVERT, SHAPE_A_DIMENSIONS],
            "a",
            vertex,
            [1.0, 1.0, 1.0],
            0x55,
            dimensions,
            &seen,
        );
        let tri = triangle(
            &mut e,
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            0,
        );
        let matrix = transform(&mut e, IDENTITY, [0.0, 0.0, 0.0]);
        let cache: Ptr<HkpGskCache> = e.new_object();
        e.call(0x00dafe70, &args![cache, a, tri, matrix]);
        (e, cache, seen)
    }

    #[test]
    fn init_triangle_flips_the_normal_towards_the_vertex_side() {
        // The triangle's normal is +z; A's vertex is below the plane (dot < 0).
        let (e, cache, seen) = triangle_scene([0.0, 0.0, -5.0], 2);
        let direction = seen
            .borrow()
            .iter()
            .find(|(_, v)| v.len() == 3)
            .unwrap()
            .1
            .clone();
        assert_eq!(direction, vec![0x8000_0000, 0, (-1.0f32).to_bits()]);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_0), 0x55);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_1), 0);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_2), 1);
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_3), 2);
        assert_eq!(e.get(cache, HkpGskCache::m_dimA), 1);
        assert_eq!(e.get(cache, HkpGskCache::m_dimB), 3);
        assert_eq!(e.get(cache, HkpGskCache::m_maxDims), 0x32);
        assert_eq!(e.get(cache, HkpGskCache::m_gskFlags), 0);
    }

    #[test]
    fn init_triangle_keeps_the_normal_when_the_vertex_is_above_and_clamps_dimensions() {
        let (e, cache, seen) = triangle_scene([0.0, 0.0, 5.0], 9);
        let direction = seen
            .borrow()
            .iter()
            .find(|(_, v)| v.len() == 3)
            .unwrap()
            .1
            .clone();
        assert_eq!(direction, vec![0, 0x8000_0000, 1.0f32.to_bits()]);
        assert_eq!(e.get(cache, HkpGskCache::m_maxDims), 0x3f);
    }

    #[test]
    fn init_triangle_with_the_flag_set_runs_the_general_init() {
        let mut e = Engine::new();
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let a = shape(
            &mut e,
            [SHAPE_A_SUPPORT, SHAPE_A_CONVERT, SHAPE_A_DIMENSIONS],
            "a",
            [0.0; 3],
            [0.0; 3],
            0x22,
            1,
            &seen,
        );
        let b = shape(
            &mut e,
            [SHAPE_B_SUPPORT, SHAPE_B_CONVERT, SHAPE_B_DIMENSIONS],
            "b",
            [0.0; 3],
            [0.0; 3],
            0x33,
            1,
            &seen,
        );
        // `triangle + 0x17` is non-zero: `b` stands for any shape with that byte set.
        e.mem.set_u8(b.addr() + 0x17, 1);
        let matrix = transform(&mut e, IDENTITY, [0.0; 3]);
        let cache: Ptr<HkpGskCache> = e.new_object();
        e.call(0x00dafe70, &args![cache, a, b, matrix]);
        // The general path sets both vertex ids, and dimension 1 for B (not 3).
        assert_eq!(e.get(cache, HkpGskCache::m_vertices_1), 0x33);
        assert_eq!(e.get(cache, HkpGskCache::m_dimB), 1);
    }
}
