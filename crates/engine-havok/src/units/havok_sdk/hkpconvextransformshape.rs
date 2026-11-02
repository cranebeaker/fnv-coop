//! `Havok SDK/hkpconvextransformshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexTransformShape` wraps a child convex shape (`m_childShape`,
//! +0x18) and places it with a transform stored at +0x20: the rotation
//! columns `c0`, `c1`, `c2` (+0x20, +0x30, +0x40) and the translation
//! (+0x50). Queries are forwarded to the child in the child's space (points
//! and directions go in through the inverse rotation, results come out
//! through `v.y * c1 + v.x * c0 + (v.z * c2 + t)`). SSE code computes lane
//! by lane in `f32`; x87 code rounds at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{
    copy_vec, load_vec, profile_record, store_vec, sum3, zip_with, Vec4, OUT_SHAPE_KEYS,
    OUT_SHAPE_KEY_INDEX, PROFILE_END,
};
use super::hkptriangleshape::inverse_rotate;

layout! {
    /// `hkpConvexTransformShape` (Xbox PDB), 0x60 bytes.
    pub struct HkpConvexTransformShape: 0x60 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_childShape` (Xbox PDB), the wrapped `hkpConvexShape`.
        0x18 m_childShape: u32,
        // +0x14 is the embedded container sub-object (`getContainer`);
        // +0x20 to +0x5f is the transform (`m_transform`, Xbox PDB).
    }
}

const CONTAINER: u32 = 0x14;
const TRANSFORM: u32 = 0x20;

/// The vtable the constructor installs.
const VTABLE: u32 = 0x010d_102c;
/// `hkpConvexShape` constructor (Xbox PDB, `this` in `ECX`; stack words
/// `type`, `radius` (a `float` bit pattern), `shape` and `flag`): it fills
/// in the radius, the type and the child shape and, when `flag` is 1, takes
/// a reference on the child (`00c9e340`; not in this unit).
const CONVEX_SHAPE_CONSTRUCTOR: u32 = 0x00c9_e340;
/// The shape type the constructor passes.
const SHAPE_TYPE: u32 = 0x0c;

/// Slots of the child's vtable (byte offsets) that this unit calls.
const SLOT_MAXIMUM_PROJECTION: u32 = 0x0c;
const SLOT_AABB: u32 = 0x1c;
const SLOT_CAST_RAY: u32 = 0x20;
const SLOT_CAST_RAY_WITH_COLLECTOR: u32 = 0x24;
const SLOT_NUM_COLLISION_SPHERES: u32 = 0x2c;
const SLOT_COLLISION_SPHERES: u32 = 0x30;
const SLOT_FIRST_VERTEX: u32 = 0x34;
const SLOT_SUPPORTING_VERTEX: u32 = 0x38;
const SLOT_CONVERT_VERTEX_IDS: u32 = 0x3c;
const SLOT_CENTRE: u32 = 0x44;
const SLOT_CALC_SIZE_FOR_SPU: u32 = 0x18;

/// `hkReferencedObject::calcContentStatistics` (`this` in `ECX`, collector, class), the
/// `hkClass` `hkpConvexTransformShape::calcContentStatistics` passes to it, and the string
/// the collector's slot `0xC` is given first.
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
const TRANSFORM_SHAPE_CLASS: u32 = 0x0126_8940;
const CHILD_LABEL: u32 = 0x010c_6fec;

/// `hkTransform::setMul` (Xbox PDB, `this` = out; `(a, b)`).
const SET_MUL: u32 = 0x00cb_2240;
/// `hkVector4::setRotatedDir` (Xbox PDB, `this` = out; `(transform,
/// vector)`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// Begin labels of the profiling records of `castRayImpl` and
/// `castRayWithCollectorImpl`.
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010d_1074;
const PROFILE_BEGIN_CAST_RAY_WITH_COLLECTOR: u32 = 0x010d_1084;

/// The stored transform: columns `c0`, `c1`, `c2` and the translation.
struct Placement {
    columns: [Vec4; 3],
    translation: Vec4,
}

impl Placement {
    fn load(e: &Engine, this: Ptr<HkpConvexTransformShape>) -> Self {
        let base = this.addr() + TRANSFORM;
        Placement {
            columns: std::array::from_fn(|index| load_vec(e, base + 0x10 * index as u32)),
            translation: load_vec(e, base + 0x30),
        }
    }

    /// `(v.y * c1 + v.x * c0) + (v.z * c2 + t)`, all four lanes.
    fn to_world(&self, v: Vec4) -> Vec4 {
        std::array::from_fn(|lane| {
            (v[1] * self.columns[1][lane] + v[0] * self.columns[0][lane])
                + (v[2] * self.columns[2][lane] + self.translation[lane])
        })
    }

    /// The inverse rotation of `v` (see `inverse_rotate`).
    fn rotate_in(&self, v: Vec4) -> Vec4 {
        inverse_rotate(self.columns, v)
    }

    /// A point in the child's space: the inverse rotation of `p - t`.
    fn point_in(&self, p: Vec4) -> Vec4 {
        self.rotate_in(zip_with(p, self.translation, |a, b| a - b))
    }
}

/// `[T.x, T.y, T.z, w]`: the transformed vertex with the `w` lane of the
/// original (the `UNPCKHPS`/`SHUFPS 0xc4` merge).
fn keep_w(moved: Vec4, original: Vec4) -> Vec4 {
    [moved[0], moved[1], moved[2], original[3]]
}

// Translated from 00d5d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getNumCollisionSpheresImpl` (Xbox PDB): the
/// child's answer (its virtual function is tail-called with the child as
/// `this`).
pub fn hkp_convex_transform_shape_get_num_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
) -> u32 {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    e.vcall(child, SLOT_NUM_COLLISION_SPHERES, &args![]).u32()
}

// Translated from 00d5d760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getContainer` (Xbox PDB): the embedded
/// container at `this + 0x14`.
pub fn hkp_convex_transform_shape_get_container(
    _e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
) -> u32 {
    this.addr() + CONTAINER
}

// Translated from 00d5d7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::hkpConvexTransformShape` (Xbox PDB), three
/// pushed words: constructs the base with type 12, the radius of `shape`
/// (through `FLD`/`FSTP`), `shape` and `flag`, installs the vtable and
/// copies the four vectors of `transform` to +0x20..+0x5f. Returns `this`.
pub fn hkp_convex_transform_shape_constructor(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    shape: Ptr,
    transform: Ptr,
    flag: u32,
) -> Ptr<HkpConvexTransformShape> {
    let radius = x87_quiet(e.mem.f32(shape.addr() + 0x10));
    e.call(
        CONVEX_SHAPE_CONSTRUCTOR,
        &args![this, SHAPE_TYPE, radius, shape, flag],
    );
    e.set(this, HkpConvexTransformShape::vtable, VTABLE);
    for index in 0..4 {
        copy_vec(
            e,
            transform.addr() + 0x10 * index,
            this.addr() + TRANSFORM + 0x10 * index,
        );
    }
    this
}

// Translated from 00d5d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getAabbImpl` (Xbox PDB): the child's box for
/// the transform `transform` combined with the stored one
/// (`hkTransform::setMul(transform, this + 0x20)`) and `tolerance` (passed
/// on through `FLD`/`FSTP`).
pub fn hkp_convex_transform_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    e.with_stack(0x40, |e, combined| {
        e.call(
            SET_MUL,
            &args![combined, transform, this.addr() + TRANSFORM],
        );
        e.vcall(
            child,
            SLOT_AABB,
            &args![combined, x87_quiet(tolerance), out],
        );
    });
}

// Translated from 00d5d840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getMaximumProjection` (Xbox PDB): takes
/// `direction` into the child's space (inverse rotation), asks the child
/// for its maximum projection along it, places the point `projection *
/// direction'` with the stored transform and returns its dot product with
/// `direction` (the three lanes added `z + (y + x)`, loaded as a `float`
/// into `ST0`).
pub fn hkp_convex_transform_shape_get_maximum_projection(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    direction: Ptr,
) -> f32 {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let placement = Placement::load(e, this);
    let direction_value = load_vec(e, direction.addr());
    let local = placement.rotate_in(direction_value);
    let projection = e.with_stack(0x10, |e, scratch| {
        store_vec(e, scratch.addr(), local);
        e.vcall(child, SLOT_MAXIMUM_PROJECTION, &args![scratch])
            .f32()
    });
    let scaled = local.map(|lane| projection * lane);
    let placed = placement.to_world(scaled);
    x87_quiet(sum3(zip_with(placed, direction_value, |p, q| p * q)))
}

// Translated from 00d5dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getSupportingVertexImpl` (Xbox PDB): the
/// child's supporting vertex for `direction` taken into the child's space,
/// placed with the stored transform; the `w` lane stays the child's.
pub fn hkp_convex_transform_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    direction: Ptr,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let placement = Placement::load(e, this);
    let local = placement.rotate_in(load_vec(e, direction.addr()));
    let vertex = e.with_stack(0x20, |e, scratch| {
        store_vec(e, scratch.addr(), local);
        e.vcall(
            child,
            SLOT_SUPPORTING_VERTEX,
            &args![scratch, scratch.addr() + 0x10],
        );
        load_vec(e, scratch.addr() + 0x10)
    });
    store_vec(e, out.addr(), keep_w(placement.to_world(vertex), vertex));
}

// Translated from 00d5dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::convertVertexIdsToVerticesImpl` (Xbox PDB):
/// the child converts the ids into `out`, then the vertices are placed with
/// the stored transform, last to first, keeping their `w` lanes. The loop
/// body runs once even for `count <= 0` (it then works on `out[count - 1]`).
pub fn hkp_convex_transform_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    e.vcall(child, SLOT_CONVERT_VERTEX_IDS, &args![ids, count, out]);
    let placement = Placement::load(e, this);
    let mut index = count.wrapping_sub(1);
    loop {
        let address = out.addr().wrapping_add((index as u32) << 4);
        let vertex = load_vec(e, address);
        store_vec(e, address, keep_w(placement.to_world(vertex), vertex));
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
    }
}

// Translated from 00d5de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getCentreImpl` (Xbox PDB): the child's centre,
/// placed with the stored transform (all four lanes).
pub fn hkp_convex_transform_shape_get_centre_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    e.vcall(child, SLOT_CENTRE, &args![out]);
    let placement = Placement::load(e, this);
    let centre = load_vec(e, out.addr());
    store_vec(e, out.addr(), placement.to_world(centre));
}

// Translated from 00d5dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getFirstVertex` (Xbox PDB): the child's first
/// vertex, placed with the stored transform (all four lanes).
pub fn hkp_convex_transform_shape_get_first_vertex(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let vertex = e.with_stack(0x10, |e, scratch| {
        e.vcall(child, SLOT_FIRST_VERTEX, &args![scratch]);
        load_vec(e, scratch.addr())
    });
    let placement = Placement::load(e, this);
    store_vec(e, out.addr(), placement.to_world(vertex));
}

// Translated from 00d5df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::getCollisionSpheresImpl` (Xbox PDB): the child
/// writes its spheres (answering a pointer to them) and their number is
/// asked; the spheres are placed with the stored transform into `out`,
/// last to first, keeping their `w` lanes (the radius). The loop body runs
/// once even for a count of 0 or less. Returns `out`.
pub fn hkp_convex_transform_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    out: Ptr,
) -> Ptr {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let spheres = e.vcall(child, SLOT_COLLISION_SPHERES, &args![out]).u32();
    let count = e.vcall(child, SLOT_NUM_COLLISION_SPHERES, &args![]).i32();
    let placement = Placement::load(e, this);
    let mut index = count.wrapping_sub(1);
    loop {
        let offset = (index as u32) << 4;
        let sphere = load_vec(e, spheres.wrapping_add(offset));
        store_vec(
            e,
            out.addr().wrapping_add(offset),
            keep_w(placement.to_world(sphere), sphere),
        );
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
    }
    out
}

/// The 48-byte `hkpShapeRayCastInput` copy handed to the child, with
/// `m_from` and `m_to` taken into the child's space.
fn local_ray(e: &mut Engine, placement: &Placement, input: u32, local: u32) {
    for word in 0..12 {
        let value = e.mem.u32(input + 4 * word);
        e.mem.set_u32(local + 4 * word, value);
    }
    for point in [0, 0x10] {
        let moved = placement.point_in(load_vec(e, input + point));
        store_vec(e, local + point, moved);
    }
}

// Translated from 00d5d940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::castRayImpl` (Xbox PDB): casts the ray, taken
/// into the child's space, at the child with `output.m_shapeKeyIndex` one
/// higher. On a hit (`*result != 0`) the normal in `output` is rotated back
/// with `hkVector4::setRotatedDir` (the stored transform) and the shape key
/// slot of this level is set to 0. Returns `result`. A profiling record is
/// written first and last.
pub fn hkp_convex_transform_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let placement = Placement::load(e, this);
    let out = output.addr();
    e.with_stack(0x40, |e, block| {
        let (local, temporary) = (block.addr(), block.addr() + 0x30);
        local_ray(e, &placement, input.addr(), local);
        let depth = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
        e.mem
            .set_u32(out + OUT_SHAPE_KEY_INDEX, depth.wrapping_add(1));
        e.vcall(child, SLOT_CAST_RAY, &args![result, local, output]);
        let depth = e.mem.u32(out + OUT_SHAPE_KEY_INDEX).wrapping_sub(1);
        e.mem.set_u32(out + OUT_SHAPE_KEY_INDEX, depth);
        if e.mem.u8(result.addr()) != 0 {
            copy_vec(e, out, temporary);
            e.call(
                SET_ROTATED_DIR,
                &args![output, this.addr() + TRANSFORM, temporary],
            );
            let depth = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
            e.mem.set_u32(
                out.wrapping_add(depth.wrapping_mul(4))
                    .wrapping_add(OUT_SHAPE_KEYS),
                0,
            );
        }
    });
    profile_record(e, PROFILE_END);
    result
}

// Translated from 00d5dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::castRayWithCollectorImpl` (Xbox PDB), three
/// pushed words: like the translate shape's: the ray `input` taken into the
/// child's space and a local `hkpCdBody` for the child whose transform is
/// the body's motion transform (`cd_body + 8`) multiplied
/// (`hkTransform::setMul`) by the stored one; it points back to `cd_body` as
/// its parent. The child's `castRayWithCollectorImpl` gets these and
/// `collector`. A profiling record is written first and last.
pub fn hkp_convex_transform_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    input: Ptr,
    cd_body: Ptr,
    collector: Ptr,
) {
    profile_record(e, PROFILE_BEGIN_CAST_RAY_WITH_COLLECTOR);
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let placement = Placement::load(e, this);
    // The ray copy (0x30), the product transform (0x40) and the child's
    // body (0x10).
    e.with_stack(0x80, |e, block| {
        let base = block.addr();
        let (local_input, local_world, child_body) = (base, base + 0x30, base + 0x70);
        local_ray(e, &placement, input.addr(), local_input);
        let motion = e.mem.u32(cd_body.addr() + 8);
        e.call(
            SET_MUL,
            &args![local_world, motion, this.addr() + TRANSFORM],
        );
        e.mem.set_u32(child_body, child);
        e.mem.set_u32(child_body + 4, 0);
        e.mem.set_u32(child_body + 8, local_world);
        e.mem.set_u32(child_body + 0xc, cd_body.addr());
        e.vcall(
            child,
            SLOT_CAST_RAY_WITH_COLLECTOR,
            &args![local_input, child_body, collector],
        );
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d5d730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::calcContentStatistics` (engine map): `ECX` = shape, the
/// collector and one more word (never read) on the stack. Calls the collector's slot `0xC`
/// with the label at `010c6fec`, the child shape and 0, then
/// `hkReferencedObject::calcContentStatistics` with the class at `01268940`.
pub fn hkp_convex_transform_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    collector: u32,
    _unused_1: u32,
) {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    e.vcall(collector, 0xc, &args![CHILD_LABEL, child, 0u32]);
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, TRANSFORM_SHAPE_CLASS],
    );
}

// Translated from 00d5d770 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function with no name in the engine map: `ECX` = shape, one stack word, the address of
/// four 16-byte vectors. Copies them bit for bit over the stored transform (`+0x20` to
/// `+0x5f`).
pub fn fn_00d5d770(e: &mut Engine, this: Ptr<HkpConvexTransformShape>, transform: u32) {
    for index in 0..4 {
        copy_vec(
            e,
            transform + 0x10 * index,
            this.addr() + TRANSFORM + 0x10 * index,
        );
    }
}

// Translated from 00d5e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTransformShape::calcSizeForSpu` (engine map): `ECX` = shape, two stack words.
/// Asks the child's slot `0x18` for its size with `limit - 0x60` as the limit; a negative
/// answer or one above that limit gives -1. When the child is stored right behind the
/// shape (at `this + 0x60`) the word at `+0x1c` is cleared and the result is the child's
/// size plus 0x60; otherwise the word at `+0x1c` receives the child's size and the result is
/// 0x60.
pub fn hkp_convex_transform_shape_calc_size_for_spu(
    e: &mut Engine,
    this: Ptr<HkpConvexTransformShape>,
    first: u32,
    limit: i32,
) -> i32 {
    let child = e.get(this, HkpConvexTransformShape::m_childShape);
    let room = limit.wrapping_sub(0x60);
    let size = e
        .vcall(child, SLOT_CALC_SIZE_FOR_SPU, &args![first, room])
        .i32();
    if !(0..=room).contains(&size) {
        return -1;
    }
    if child == this.addr() + 0x60 {
        e.mem.set_u32(this.addr() + 0x1c, 0);
        size.wrapping_add(0x60)
    } else {
        e.mem.set_u32(this.addr() + 0x1c, size as u32);
        0x60
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5d720,
            hkp_convex_transform_shape_get_num_collision_spheres_impl(
                Ptr<HkpConvexTransformShape>,
            ) -> u32
        ),
        entry!(
            0x00d5d760,
            hkp_convex_transform_shape_get_container(Ptr<HkpConvexTransformShape>) -> u32
        ),
        entry!(
            0x00d5d7a0,
            hkp_convex_transform_shape_constructor(
                Ptr<HkpConvexTransformShape>,
                Ptr,
                Ptr,
                u32,
            ) -> Ptr<HkpConvexTransformShape>
        ),
        entry!(
            0x00d5d800,
            hkp_convex_transform_shape_get_aabb_impl(Ptr<HkpConvexTransformShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d5d840,
            hkp_convex_transform_shape_get_maximum_projection(
                Ptr<HkpConvexTransformShape>,
                Ptr,
            ) -> f32
        ),
        entry!(
            0x00d5dc70,
            hkp_convex_transform_shape_get_supporting_vertex_impl(
                Ptr<HkpConvexTransformShape>,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00d5dd40,
            hkp_convex_transform_shape_convert_vertex_ids_to_vertices_impl(
                Ptr<HkpConvexTransformShape>,
                Ptr,
                i32,
                Ptr,
            )
        ),
        entry!(
            0x00d5de40,
            hkp_convex_transform_shape_get_centre_impl(Ptr<HkpConvexTransformShape>, Ptr)
        ),
        entry!(
            0x00d5dea0,
            hkp_convex_transform_shape_get_first_vertex(Ptr<HkpConvexTransformShape>, Ptr)
        ),
        entry!(
            0x00d5df10,
            hkp_convex_transform_shape_get_collision_spheres_impl(
                Ptr<HkpConvexTransformShape>,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00d5d940,
            hkp_convex_transform_shape_cast_ray_impl(
                Ptr<HkpConvexTransformShape>,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00d5dae0,
            hkp_convex_transform_shape_cast_ray_with_collector_impl(
                Ptr<HkpConvexTransformShape>,
                Ptr,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00d5d730,
            hkp_convex_transform_shape_calc_content_statistics(
                Ptr<HkpConvexTransformShape>,
                u32,
                u32,
            )
        ),
        entry!(0x00d5d770, fn_00d5d770(Ptr<HkpConvexTransformShape>, u32)),
        entry!(
            0x00d5e020,
            hkp_convex_transform_shape_calc_size_for_spu(
                Ptr<HkpConvexTransformShape>,
                u32,
                i32,
            ) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_VTABLE: u32 = 0x0200_0000;
    const FAKE_BASE: u32 = 0x7000_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The address a child vtable slot (byte offset) points at.
    fn fake(slot: u32) -> u32 {
        FAKE_BASE + slot
    }

    /// A transform shape placing its child with a rotation of 90 degrees
    /// about z (`c0 = (0, 1, 0)`, `c1 = (-1, 0, 0)`, `c2 = (0, 0, 1)`) and
    /// the translation `(10, 20, 30)`.
    fn shape(e: &mut Engine) -> (Ptr<HkpConvexTransformShape>, u32) {
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        let slots: Vec<u32> = (0..24).map(|index| fake(4 * index)).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child, CHILD_VTABLE);
        let this = e.new_object::<HkpConvexTransformShape>();
        e.set(this, HkpConvexTransformShape::m_childShape, child);
        let base = this.addr() + TRANSFORM;
        store_vec(e, base, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, base + 0x30, [10.0, 20.0, 30.0, 0.0]);
        e.call_log = Some(vec![]);
        (this, child)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, v);
        address
    }

    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        stream
    }

    #[test]
    fn num_collision_spheres_asks_the_child() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_NUM_COLLISION_SPHERES), |_, _| ret(5));
        assert_eq!(e.call(0x00d5d720, &args![this]).u32(), 5);
        assert_eq!(
            calls_to(&e, fake(SLOT_NUM_COLLISION_SPHERES)),
            vec![vec![child]]
        );
    }

    #[test]
    fn container_is_inside_the_shape() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        assert_eq!(e.call(0x00d5d760, &args![this]).u32(), this.addr() + 0x14);
    }

    #[test]
    fn constructor_builds_the_base_and_copies_the_transform() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register_double(CONVEX_SHAPE_CONSTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.new_object::<HkpConvexTransformShape>();
        let shape_object = e.mem.alloc(0x20);
        e.mem.set_f32(shape_object + 0x10, 0.25);
        let transform = e.mem.alloc(0x40);
        for index in 0..4u32 {
            store_vec(&mut e, transform + 0x10 * index, [index as f32; 4]);
        }
        let returned = e
            .call(0x00d5d7a0, &args![this, shape_object, transform, 1u32])
            .u32();
        assert_eq!(returned, this.addr());
        assert_eq!(
            calls_to(&e, CONVEX_SHAPE_CONSTRUCTOR),
            vec![vec![this.addr(), 0x0c, 0.25f32.to_bits(), shape_object, 1]]
        );
        assert_eq!(e.mem.u32(this.addr()), 0x010d_102c);
        for index in 0..4u32 {
            assert_eq!(
                load_vec(&e, this.addr() + 0x20 + 0x10 * index),
                [index as f32; 4]
            );
        }
    }

    #[test]
    fn aabb_combines_the_transforms_and_asks_the_child() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(SET_MUL, |_, _| Ret::default());
        e.register_double(fake(SLOT_AABB), |_, _| Ret::default());
        let transform = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        e.call(0x00d5d800, &args![this, transform, 0.5f32, out]);
        let multiply = calls_to(&e, SET_MUL);
        assert_eq!(multiply.len(), 1);
        assert_eq!(multiply[0][1..], [transform, this.addr() + 0x20]);
        assert_eq!(
            calls_to(&e, fake(SLOT_AABB)),
            vec![vec![child, multiply[0][0], 0.5f32.to_bits(), out]]
        );
    }

    #[test]
    fn maximum_projection_places_the_scaled_direction() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        let seen = Rc::new(RefCell::new(None));
        let record = Rc::clone(&seen);
        e.register_double(fake(SLOT_MAXIMUM_PROJECTION), move |e, a| {
            *record.borrow_mut() = Some(load_vec(e, a[1]));
            Ret {
                st0: 2.0,
                ..Ret::default()
            }
        });
        let direction = vector_at(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let value = e.call(0x00d5d840, &args![this, direction]).f32();
        // The child sees the direction rotated into its space: (2, -1, 3).
        assert_eq!(seen.borrow().unwrap()[..3], [2.0, -1.0, 3.0]);
        // 2 * (2, -1, 3) placed: (2, 4, 6) + (10, 20, 30), dotted with
        // (1, 2, 3).
        assert_eq!(value, 168.0);
        assert_eq!(calls_to(&e, fake(SLOT_MAXIMUM_PROJECTION)).len(), 1);
        assert_eq!(calls_to(&e, fake(SLOT_MAXIMUM_PROJECTION))[0][0], child);
    }

    #[test]
    fn supporting_vertex_is_placed_and_keeps_the_childs_w() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let seen = Rc::new(RefCell::new(None));
        let record = Rc::clone(&seen);
        e.register_double(fake(SLOT_SUPPORTING_VERTEX), move |e, a| {
            *record.borrow_mut() = Some(load_vec(e, a[1]));
            store_vec(e, a[2], [1.0, 0.0, 0.0, 0.5]);
            Ret::default()
        });
        let direction = vector_at(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d5dc70, &args![this, direction, out]);
        assert_eq!(seen.borrow().unwrap()[..3], [2.0, -1.0, 3.0]);
        assert_eq!(load_vec(&e, out), [10.0, 21.0, 30.0, 0.5]);
    }

    #[test]
    fn convert_vertex_ids_places_every_vertex_last_to_first() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_CONVERT_VERTEX_IDS), |e, a| {
            for index in 0..a[2] {
                store_vec(e, a[3] + 0x10 * index, [index as f32, 0.0, 0.0, 7.0]);
            }
            Ret::default()
        });
        let ids = e.mem.alloc(8);
        let out = e.mem.alloc(0x40);
        e.call(0x00d5dd40, &args![this, ids, 2i32, out]);
        assert_eq!(load_vec(&e, out), [10.0, 20.0, 30.0, 7.0]);
        assert_eq!(load_vec(&e, out + 0x10), [10.0, 21.0, 30.0, 7.0]);
        assert_eq!(
            calls_to(&e, fake(SLOT_CONVERT_VERTEX_IDS)),
            vec![vec![child, ids, 2, out]]
        );
        // A count of zero still runs the body once, on `out[-1]`.
        let block = e.mem.alloc(0x20);
        store_vec(&mut e, block, [1.0, 0.0, 0.0, 3.0]);
        e.call(0x00d5dd40, &args![this, ids, 0i32, block + 0x10]);
        assert_eq!(load_vec(&e, block), [10.0, 21.0, 30.0, 3.0]);
    }

    #[test]
    fn centre_is_placed_in_all_four_lanes() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.register_double(fake(SLOT_CENTRE), |e, a| {
            store_vec(e, a[1], [1.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00d5de40, &args![this, out]);
        assert_eq!(load_vec(&e, out), [10.0, 21.0, 30.0, 0.0]);
    }

    #[test]
    fn first_vertex_is_placed() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.register_double(fake(SLOT_FIRST_VERTEX), |e, a| {
            store_vec(e, a[1], [0.0, 1.0, 0.0, 0.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00d5dea0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [9.0, 20.0, 30.0, 0.0]);
    }

    #[test]
    fn collision_spheres_are_placed_and_keep_their_radius() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let source = e.mem.alloc(0x20);
        store_vec(&mut e, source, [1.0, 0.0, 0.0, 0.5]);
        store_vec(&mut e, source + 0x10, [0.0, 1.0, 0.0, 0.75]);
        e.register_double(fake(SLOT_COLLISION_SPHERES), move |_, _| ret(source));
        e.register_double(fake(SLOT_NUM_COLLISION_SPHERES), |_, _| ret(2));
        let out = e.mem.alloc(0x20);
        let returned = e.call(0x00d5df10, &args![this, out]).u32();
        assert_eq!(returned, out);
        assert_eq!(load_vec(&e, out), [10.0, 21.0, 30.0, 0.5]);
        assert_eq!(load_vec(&e, out + 0x10), [9.0, 20.0, 30.0, 0.75]);
    }

    #[test]
    fn cast_ray_takes_the_ray_in_and_rotates_the_normal_out() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let stream = profile_stream(&mut e);
        let seen = Rc::new(RefCell::new(None));
        let record = Rc::clone(&seen);
        e.register_double(fake(SLOT_CAST_RAY), move |e, a| {
            let depth = e.mem.u32(a[3] + OUT_SHAPE_KEY_INDEX);
            *record.borrow_mut() = Some((load_vec(e, a[2]), load_vec(e, a[2] + 0x10), depth));
            store_vec(e, a[3], [1.0, 0.0, 0.0, 0.0]);
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let rotations = Rc::new(RefCell::new(Vec::new()));
        let rotated = Rc::clone(&rotations);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            rotated.borrow_mut().push((a[0], a[1], load_vec(e, a[2])));
            store_vec(e, a[0], [0.0, 1.0, 0.0, 0.0]);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [10.0, 20.0, 31.0, 0.0]);
        store_vec(&mut e, input + 0x10, [11.0, 20.0, 30.0, 0.0]);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + OUT_SHAPE_KEY_INDEX, 1);
        e.mem.set_u32(output + OUT_SHAPE_KEYS + 4, 0xffff_ffff);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00d5d940, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        let (from, to, depth) = seen.borrow().unwrap();
        // (0, 0, 1) and (1, 0, 0) rotated into the child's space.
        assert_eq!(from[..3], [0.0, 0.0, 1.0]);
        assert_eq!(to[..3], [0.0, -1.0, 0.0]);
        assert_eq!(depth, 2);
        let rotations = rotations.borrow();
        assert_eq!(rotations.len(), 1);
        assert_eq!(rotations[0].0, output);
        assert_eq!(rotations[0].1, this.addr() + 0x20);
        assert_eq!(rotations[0].2, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, output), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEY_INDEX), 1);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEYS + 4), 0);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        // A miss rotates nothing.
        e.register_double(fake(SLOT_CAST_RAY), |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.call(0x00d5d940, &args![this, result, input, output]);
        assert_eq!(rotations.len(), 1);
        assert_eq!(calls_to(&e, SET_ROTATED_DIR).len(), 1);
    }

    #[test]
    fn cast_ray_with_collector_builds_a_child_body() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        let stream = profile_stream(&mut e);
        let motion = e.mem.alloc(0x40);
        let cd_body = e.mem.alloc(0x10);
        e.mem.set_u32(cd_body + 8, motion);
        e.register_double(SET_MUL, |_, _| Ret::default());
        let seen = Rc::new(RefCell::new(None));
        let record = Rc::clone(&seen);
        e.register_double(fake(SLOT_CAST_RAY_WITH_COLLECTOR), move |e, a| {
            let body: [u32; 4] = std::array::from_fn(|word| e.mem.u32(a[2] + 4 * word as u32));
            *record.borrow_mut() = Some((load_vec(e, a[1]), load_vec(e, a[1] + 0x10), body, a[3]));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [10.0, 20.0, 31.0, 0.0]);
        store_vec(&mut e, input + 0x10, [11.0, 20.0, 30.0, 0.0]);
        let collector = e.mem.alloc(0x10);
        e.call(0x00d5dae0, &args![this, input, cd_body, collector]);
        let (from, to, body, passed_collector) = seen.borrow().unwrap();
        assert_eq!(from[..3], [0.0, 0.0, 1.0]);
        assert_eq!(to[..3], [0.0, -1.0, 0.0]);
        assert_eq!(passed_collector, collector);
        let multiply = calls_to(&e, SET_MUL);
        assert_eq!(multiply.len(), 1);
        assert_eq!(multiply[0][1..], [motion, this.addr() + 0x20]);
        assert_eq!(body, [child, 0, multiply[0][0], cd_body]);
        assert_eq!(
            e.mem.u32(stream + 0x20),
            PROFILE_BEGIN_CAST_RAY_WITH_COLLECTOR
        );
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn calc_content_statistics_reports_the_child_then_the_base() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(0xc), |_, _| Ret::default());
        e.register_double(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, CHILD_VTABLE);
        e.call(0x00d5d730, &args![this, collector, 0x9999u32]);
        // The first entry is the call under test itself.
        let log = e.call_log.clone().unwrap()[1..].to_vec();
        assert_eq!(
            log,
            vec![
                (fake(0xc), vec![collector, 0x010c_6fec, child, 0]),
                (
                    REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                    vec![this.addr(), collector, 0x0126_8940]
                ),
            ]
        );
    }

    #[test]
    fn transform_setter_copies_four_vectors_bit_for_bit() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let source = e.mem.alloc(0x40);
        for word in 0..16u32 {
            e.mem.set_u32(source + 4 * word, 0x7fc0_0000 + word);
        }
        e.call(0x00d5d770, &args![this, source]);
        for word in 0..16u32 {
            assert_eq!(e.mem.u32(this.addr() + 0x20 + 4 * word), 0x7fc0_0000 + word);
        }
    }

    /// A shape whose child is stored right behind it (at `this + 0x60`).
    fn shape_with_inline_child(e: &mut Engine) -> u32 {
        shape(e);
        let block = e.mem.alloc(0x80);
        e.mem.set_u32(block + 0x18, block + 0x60);
        e.mem.set_u32(block + 0x1c, 0x1234);
        e.mem.set_u32(block + 0x60, CHILD_VTABLE);
        block
    }

    #[test]
    fn calc_size_for_spu_keeps_a_separate_child_apart() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x30));
        let result = e.call(0x00d5e020, &args![this, 7u32, 0x200u32]).i32();
        assert_eq!(result, 0x60);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x30);
        assert_eq!(
            calls_to(&e, fake(SLOT_CALC_SIZE_FOR_SPU)),
            vec![vec![child, 7, 0x1a0]]
        );
    }

    #[test]
    fn calc_size_for_spu_adds_an_inline_child() {
        let mut e = Engine::new();
        let block = shape_with_inline_child(&mut e);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x30));
        let result = e.call(0x00d5e020, &args![block, 7u32, 0x200u32]).i32();
        assert_eq!(result, 0x90);
        assert_eq!(e.mem.u32(block + 0x1c), 0);
    }

    #[test]
    fn calc_size_for_spu_rejects_negative_and_oversized_answers() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this.addr() + 0x1c, 0x55);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(-1i32 as u32));
        assert_eq!(e.call(0x00d5e020, &args![this, 7u32, 0x200u32]).i32(), -1);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x1a1));
        assert_eq!(e.call(0x00d5e020, &args![this, 7u32, 0x200u32]).i32(), -1);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x1a0));
        assert_eq!(e.call(0x00d5e020, &args![this, 7u32, 0x200u32]).i32(), 0x60);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x1a0);
    }
}
