//! `Havok SDK/hkpconvextranslateshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexTranslateShape` wraps a child convex shape (`m_childShape`,
//! +0x18) and moves it by `m_translation` (+0x20). Every query is forwarded
//! to the child through its vtable and the answer is moved by the
//! translation; rays and boxes are moved the other way. SSE code computes
//! lane by lane in `f32`; x87 code rounds at each `float` store.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{
    load_vec, profile_record, store_vec, sum3, zip_with, Vec4, OUT_SHAPE_KEYS, OUT_SHAPE_KEY_INDEX,
    PROFILE_END,
};

layout! {
    /// `hkpConvexTranslateShape` (Xbox PDB), 0x30 bytes.
    pub struct HkpConvexTranslateShape: 0x30 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_childShape` (Xbox PDB), the wrapped `hkpConvexShape`.
        0x18 m_childShape: u32,
        // +0x14 is the embedded container sub-object (`getContainer`);
        // +0x20 is `m_translation` (Xbox PDB), an `hkVector4`.
    }
}

const CONTAINER: u32 = 0x14;
const TRANSLATION: u32 = 0x20;

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
/// class word `hkpConvexTranslateShape::calcContentStatistics` passes to it, and the string
/// the collector's slot `0xC` is given first.
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
const TRANSLATE_SHAPE_CLASS: u32 = 0x0126_8940;
const CHILD_LABEL: u32 = 0x010c_6fec;

/// `hkTransform::setMul` (Xbox PDB, `this` = out; `(a, b)`).
const SET_MUL: u32 = 0x00cb_2240;
/// The begin label of `castRayImpl`'s profiling record.
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010d_1074;

fn translation(e: &Engine, this: Ptr<HkpConvexTranslateShape>) -> Vec4 {
    load_vec(e, this.addr() + TRANSLATION)
}

/// `translation + vector`, in place at `address` (the translation is the
/// first operand of the `ADDPS`).
fn add_translation(e: &mut Engine, this: Ptr<HkpConvexTranslateShape>, address: u32) {
    let moved = zip_with(translation(e, this), load_vec(e, address), |p, q| p + q);
    store_vec(e, address, moved);
}

// Translated from 00dd5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getNumCollisionSpheresImpl` (Xbox PDB): the
/// child's answer (its virtual function is tail-called with the child as
/// `this`).
pub fn hkp_convex_translate_shape_get_num_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
) -> u32 {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(child, SLOT_NUM_COLLISION_SPHERES, &args![]).u32()
}

// Translated from 00dd5ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getContainer` (Xbox PDB): the embedded
/// container at `this + 0x14`.
pub fn hkp_convex_translate_shape_get_container(
    _e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
) -> u32 {
    this.addr() + CONTAINER
}

// Translated from 00dd5eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getMaximumProjection` (Xbox PDB): the child's
/// maximum projection along `direction` (through `FLD`/`FSTP`, so quieted)
/// plus `m_translation . direction`, added on the x87 stack and returned
/// in `ST0`.
pub fn hkp_convex_translate_shape_get_maximum_projection(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    direction: Ptr,
) -> f32 {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    let projection = x87_quiet(
        e.vcall(child, SLOT_MAXIMUM_PROJECTION, &args![direction])
            .f32(),
    );
    let offset = sum3(zip_with(
        translation(e, this),
        load_vec(e, direction.addr()),
        |p, q| p * q,
    ));
    x87_quiet((offset as f64 + projection as f64) as f32)
}

// Translated from 00dd5f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getSupportingVertexImpl` (Xbox PDB): the
/// child's supporting vertex (written to `out`) moved by the translation.
pub fn hkp_convex_translate_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    direction: Ptr,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(child, SLOT_SUPPORTING_VERTEX, &args![direction, out]);
    add_translation(e, this, out.addr());
}

// Translated from 00dd5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::convertVertexIdsToVerticesImpl` (Xbox PDB): the
/// child converts the `count` ids into `out`, then every vertex is moved by
/// the translation (nothing is moved for `count <= 0`).
pub fn hkp_convex_translate_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(child, SLOT_CONVERT_VERTEX_IDS, &args![ids, count, out]);
    for index in 0..count.max(0) as u32 {
        add_translation(e, this, out.addr() + 0x10 * index);
    }
}

// Translated from 00dd5fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getCentreImpl` (Xbox PDB): the child's centre,
/// moved by the translation.
pub fn hkp_convex_translate_shape_get_centre_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(child, SLOT_CENTRE, &args![out]);
    add_translation(e, this, out.addr());
}

// Translated from 00dd5fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getFirstVertex` (Xbox PDB): the child's first
/// vertex, moved by the translation.
pub fn hkp_convex_translate_shape_get_first_vertex(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(child, SLOT_FIRST_VERTEX, &args![out]);
    add_translation(e, this, out.addr());
}

// Translated from 00dd6000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getCollisionSpheresImpl` (Xbox PDB): asks the
/// child to write its spheres (it answers a pointer to them), asks for their
/// number and writes each sphere plus the translation (the sphere is the
/// first operand of the `ADDPS`, so the `w` lane keeps the radius plus the
/// translation's `w`) to `out`. Returns `out`.
pub fn hkp_convex_translate_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    out: Ptr,
) -> Ptr {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    let spheres = e.vcall(child, SLOT_COLLISION_SPHERES, &args![out]).u32();
    let count = e.vcall(child, SLOT_NUM_COLLISION_SPHERES, &args![]).i32();
    let moved = translation(e, this);
    for index in 0..count.max(0) as u32 {
        let sphere = load_vec(e, spheres + 0x10 * index);
        store_vec(
            e,
            out.addr() + 0x10 * index,
            zip_with(sphere, moved, |p, q| p + q),
        );
    }
    out
}

// Translated from 00dd6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::getAabbImpl` (Xbox PDB): the child's box for
/// the same `transform` and `tolerance` (the tolerance passed on through
/// `FLD`/`FSTP`), then both corners moved by the translation rotated into
/// world space: `(t.y * c1 + t.x * c0) + t.z * c2` with `c0`, `c1`, `c2`
/// the columns of `transform` (+0, +0x10, +0x20).
pub fn hkp_convex_translate_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(
        child,
        SLOT_AABB,
        &args![transform, x87_quiet(tolerance), out],
    );
    let t = translation(e, this);
    let columns: [Vec4; 3] =
        std::array::from_fn(|index| load_vec(e, transform.addr() + 0x10 * index as u32));
    let world: Vec4 = std::array::from_fn(|lane| {
        (t[1] * columns[1][lane] + t[0] * columns[0][lane]) + t[2] * columns[2][lane]
    });
    for corner in 0..2 {
        let address = out.addr() + 0x10 * corner;
        let moved = zip_with(load_vec(e, address), world, |p, q| p + q);
        store_vec(e, address, moved);
    }
}

/// The 48-byte `hkpShapeRayCastInput` copy `castRayImpl` and
/// `castRayWithCollectorImpl` hand to the child, with `m_from` and `m_to`
/// moved by `-m_translation`.
fn moved_ray(e: &mut Engine, this: Ptr<HkpConvexTranslateShape>, input: u32, local: u32) {
    for word in 0..12 {
        let value = e.mem.u32(input + 4 * word);
        e.mem.set_u32(local + 4 * word, value);
    }
    let t = translation(e, this);
    for point in [0, 0x10] {
        let moved = zip_with(load_vec(e, input + point), t, |p, q| p - q);
        store_vec(e, local + point, moved);
    }
}

// Translated from 00dd60e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::castRayImpl` (Xbox PDB): casts the ray, moved
/// by `-m_translation`, at the child with `output.m_shapeKeyIndex` one
/// higher; on a hit (`*result != 0`) the shape key slot of this level is
/// set to 0. Returns `result`. A profiling record is written first and last.
pub fn hkp_convex_translate_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    let out = output.addr();
    e.with_stack(0x30, |e, local| {
        moved_ray(e, this, input.addr(), local.addr());
        let depth = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
        e.mem
            .set_u32(out + OUT_SHAPE_KEY_INDEX, depth.wrapping_add(1));
        e.vcall(child, SLOT_CAST_RAY, &args![result, local, output]);
        let depth = e.mem.u32(out + OUT_SHAPE_KEY_INDEX).wrapping_sub(1);
        e.mem.set_u32(out + OUT_SHAPE_KEY_INDEX, depth);
        if e.mem.u8(result.addr()) != 0 {
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

// Translated from 00dd61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::castRayWithCollectorImpl` (Xbox PDB), three
/// pushed words: the ray `input` moved by `-m_translation`, and a local
/// `hkpCdBody` for the child whose transform is the body's motion
/// transform (`cd_body + 8`) multiplied (`hkTransform::setMul`) by the
/// pure translation `m_translation`; it points back to `cd_body` as its
/// parent. The child's `castRayWithCollectorImpl` gets these and
/// `collector`.
pub fn hkp_convex_translate_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    input: Ptr,
    cd_body: Ptr,
    collector: Ptr,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    let t = translation(e, this);
    // Local copy of the ray (0x30), the translation transform (0x40), the
    // product (0x40) and the child's body (0x10).
    e.with_stack(0xc0, |e, block| {
        let base = block.addr();
        let (local_input, local_transform, local_world, child_body) =
            (base, base + 0x30, base + 0x70, base + 0xb0);
        moved_ray(e, this, input.addr(), local_input);
        let identity: [Vec4; 3] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        for (index, column) in identity.into_iter().enumerate() {
            store_vec(e, local_transform + 0x10 * index as u32, column);
        }
        store_vec(e, local_transform + 0x30, t);
        let motion = e.mem.u32(cd_body.addr() + 8);
        e.call(SET_MUL, &args![local_world, motion, local_transform]);
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
}

// Translated from 00dd5e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::calcSizeForSpu` (engine map): `ECX` = shape, two stack words.
/// Asks the child's slot `0x18` for its size with `limit - 0x30` as the limit; a negative
/// answer or one above that limit gives -1. When the child is stored right behind the
/// shape (at `this + 0x30`) the word at `+0x1c` is cleared and the result is the child's
/// size plus 0x30; otherwise the word at `+0x1c` receives the child's size and the result is
/// 0x30.
pub fn hkp_convex_translate_shape_calc_size_for_spu(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    first: u32,
    limit: i32,
) -> i32 {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    let room = limit.wrapping_sub(0x30);
    let size = e
        .vcall(child, SLOT_CALC_SIZE_FOR_SPU, &args![first, room])
        .i32();
    if !(0..=room).contains(&size) {
        return -1;
    }
    if child == this.addr() + 0x30 {
        e.mem.set_u32(this.addr() + 0x1c, 0);
        size.wrapping_add(0x30)
    } else {
        e.mem.set_u32(this.addr() + 0x1c, size as u32);
        0x30
    }
}

// Translated from 00dd5e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexTranslateShape::calcContentStatistics` (engine map): `ECX` = shape, the
/// collector and one more word (never read) on the stack. Calls the collector's slot `0xC`
/// with the label at `010c6fec`, the child shape and 0, then
/// `hkReferencedObject::calcContentStatistics` with the class at `01268940`.
pub fn hkp_convex_translate_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpConvexTranslateShape>,
    collector: u32,
    _unused_1: u32,
) {
    let child = e.get(this, HkpConvexTranslateShape::m_childShape);
    e.vcall(collector, 0xc, &args![CHILD_LABEL, child, 0u32]);
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, TRANSLATE_SHAPE_CLASS],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00dd5e60,
            hkp_convex_translate_shape_get_num_collision_spheres_impl(
                Ptr<HkpConvexTranslateShape>,
            ) -> u32
        ),
        entry!(
            0x00dd5ea0,
            hkp_convex_translate_shape_get_container(Ptr<HkpConvexTranslateShape>) -> u32
        ),
        entry!(
            0x00dd5eb0,
            hkp_convex_translate_shape_get_maximum_projection(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
            ) -> f32
        ),
        entry!(
            0x00dd5f10,
            hkp_convex_translate_shape_get_supporting_vertex_impl(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00dd5f50,
            hkp_convex_translate_shape_convert_vertex_ids_to_vertices_impl(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
                i32,
                Ptr,
            )
        ),
        entry!(
            0x00dd5fa0,
            hkp_convex_translate_shape_get_centre_impl(Ptr<HkpConvexTranslateShape>, Ptr)
        ),
        entry!(
            0x00dd5fd0,
            hkp_convex_translate_shape_get_first_vertex(Ptr<HkpConvexTranslateShape>, Ptr)
        ),
        entry!(
            0x00dd6000,
            hkp_convex_translate_shape_get_collision_spheres_impl(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00dd6060,
            hkp_convex_translate_shape_get_aabb_impl(Ptr<HkpConvexTranslateShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00dd60e0,
            hkp_convex_translate_shape_cast_ray_impl(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00dd61b0,
            hkp_convex_translate_shape_cast_ray_with_collector_impl(
                Ptr<HkpConvexTranslateShape>,
                Ptr,
                Ptr,
                Ptr,
            )
        ),
        entry!(
            0x00dd5e10,
            hkp_convex_translate_shape_calc_size_for_spu(
                Ptr<HkpConvexTranslateShape>,
                u32,
                i32,
            ) -> i32
        ),
        entry!(
            0x00dd5e70,
            hkp_convex_translate_shape_calc_content_statistics(
                Ptr<HkpConvexTranslateShape>,
                u32,
                u32,
            )
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

    /// A translate shape (translation `(1, 2, 3, 0)`) around a child whose
    /// vtable slots all point at fake addresses (`fake(slot)`).
    fn shape(e: &mut Engine) -> (Ptr<HkpConvexTranslateShape>, u32) {
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        let slots: Vec<u32> = (0..24).map(|index| fake(4 * index)).collect();
        e.put_vtable(CHILD_VTABLE, &slots);
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child, CHILD_VTABLE);
        let this = e.new_object::<HkpConvexTranslateShape>();
        e.set(this, HkpConvexTranslateShape::m_childShape, child);
        store_vec(e, this.addr() + TRANSLATION, [1.0, 2.0, 3.0, 0.0]);
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
        e.register_double(fake(SLOT_NUM_COLLISION_SPHERES), |_, _| ret(7));
        let count = e.call(0x00dd5e60, &args![this]).u32();
        assert_eq!(count, 7);
        assert_eq!(
            calls_to(&e, fake(SLOT_NUM_COLLISION_SPHERES)),
            vec![vec![child]]
        );
    }

    #[test]
    fn container_is_inside_the_shape() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        assert_eq!(e.call(0x00dd5ea0, &args![this]).u32(), this.addr() + 0x14);
    }

    #[test]
    fn maximum_projection_adds_the_translation_along_the_direction() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_MAXIMUM_PROJECTION), |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let direction = vector_at(&mut e, [0.0, 1.0, 2.0, 0.0]);
        let value = e.call(0x00dd5eb0, &args![this, direction]).f32();
        assert_eq!(value, 10.5);
        assert_eq!(
            calls_to(&e, fake(SLOT_MAXIMUM_PROJECTION)),
            vec![vec![child, direction]]
        );
    }

    #[test]
    fn supporting_vertex_is_moved_by_the_translation() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_SUPPORTING_VERTEX), |e, a| {
            store_vec(e, a[2], [10.0, 20.0, 30.0, 0.5]);
            Ret::default()
        });
        let direction = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00dd5f10, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out), [11.0, 22.0, 33.0, 0.5]);
        assert_eq!(
            calls_to(&e, fake(SLOT_SUPPORTING_VERTEX)),
            vec![vec![child, direction, out]]
        );
    }

    #[test]
    fn convert_vertex_ids_moves_every_vertex() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_CONVERT_VERTEX_IDS), |e, a| {
            for index in 0..a[2] {
                store_vec(e, a[3] + 0x10 * index, [index as f32, 0.0, 0.0, 1.0]);
            }
            Ret::default()
        });
        let ids = e.mem.alloc(8);
        let out = e.mem.alloc(0x40);
        e.call(0x00dd5f50, &args![this, ids, 3i32, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x20), [3.0, 2.0, 3.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x30), [0.0; 4]);
        assert_eq!(
            calls_to(&e, fake(SLOT_CONVERT_VERTEX_IDS)),
            vec![vec![child, ids, 3, out]]
        );
        // A count of zero or less moves nothing.
        e.call(0x00dd5f50, &args![this, ids, 0i32, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 1.0]);
    }

    #[test]
    fn centre_is_moved_by_the_translation() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.register_double(fake(SLOT_CENTRE), |e, a| {
            store_vec(e, a[1], [0.5, 0.5, 0.5, 0.25]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00dd5fa0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.5, 2.5, 3.5, 0.25]);
    }

    #[test]
    fn first_vertex_is_moved_by_the_translation() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.register_double(fake(SLOT_FIRST_VERTEX), |e, a| {
            store_vec(e, a[1], [-1.0, -2.0, -3.0, 1.0]);
            Ret::default()
        });
        let out = e.mem.alloc(0x10);
        e.call(0x00dd5fd0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn collision_spheres_are_moved_by_the_translation() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let source = e.mem.alloc(0x20);
        store_vec(&mut e, source, [1.0, 1.0, 1.0, 0.5]);
        store_vec(&mut e, source + 0x10, [2.0, 2.0, 2.0, 0.75]);
        e.register_double(fake(SLOT_COLLISION_SPHERES), move |_, _| ret(source));
        e.register_double(fake(SLOT_NUM_COLLISION_SPHERES), |_, _| ret(2));
        let out = e.mem.alloc(0x20);
        let returned = e.call(0x00dd6000, &args![this, out]).u32();
        assert_eq!(returned, out);
        assert_eq!(load_vec(&e, out), [2.0, 3.0, 4.0, 0.5]);
        assert_eq!(load_vec(&e, out + 0x10), [3.0, 4.0, 5.0, 0.75]);
    }

    #[test]
    fn aabb_is_moved_by_the_rotated_translation() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_AABB), |e, a| {
            store_vec(e, a[3], [-1.0, -1.0, -1.0, 0.0]);
            store_vec(e, a[3] + 0x10, [1.0, 1.0, 1.0, 0.0]);
            Ret::default()
        });
        // A rotation by 90 degrees about z: x -> y, y -> -x.
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00dd6060, &args![this, transform, 0.25f32, out]);
        // (1, 2, 3) rotated: (-2, 1, 3).
        assert_eq!(load_vec(&e, out), [-3.0, 0.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [-1.0, 2.0, 4.0, 0.0]);
        assert_eq!(
            calls_to(&e, fake(SLOT_AABB)),
            vec![vec![child, transform, 0.25f32.to_bits(), out]]
        );
    }

    #[test]
    fn cast_ray_moves_the_ray_and_tracks_the_shape_key_depth() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        let stream = profile_stream(&mut e);
        let seen = Rc::new(RefCell::new(None));
        let record = Rc::clone(&seen);
        e.register_double(fake(SLOT_CAST_RAY), move |e, a| {
            let depth = e.mem.u32(a[3] + OUT_SHAPE_KEY_INDEX);
            *record.borrow_mut() = Some((load_vec(e, a[2]), load_vec(e, a[2] + 0x10), depth));
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [5.0, 5.0, 5.0, 0.0]);
        store_vec(&mut e, input + 0x10, [1.0, 2.0, 3.0, 1.0]);
        store_vec(&mut e, input + 0x20, [9.0, 9.0, 9.0, 9.0]);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + OUT_SHAPE_KEY_INDEX, 2);
        e.mem.set_u32(output + OUT_SHAPE_KEYS + 8, 0xffff_ffff);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00dd60e0, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        let (from, to, depth) = seen.borrow().unwrap();
        assert_eq!(from, [4.0, 3.0, 2.0, 0.0]);
        assert_eq!(to, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(depth, 3);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEY_INDEX), 2);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEYS + 8), 0);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(calls_to(&e, fake(SLOT_CAST_RAY)).len(), 1);
        // A miss leaves the shape key slot alone.
        e.mem.set_u32(output + OUT_SHAPE_KEYS + 8, 0xffff_ffff);
        e.register_double(fake(SLOT_CAST_RAY), |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.call(0x00dd60e0, &args![this, result, input, output]);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEYS + 8), 0xffff_ffff);
    }

    #[test]
    fn cast_ray_with_collector_builds_a_child_body() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        let motion = e.mem.alloc(0x40);
        let cd_body = e.mem.alloc(0x10);
        e.mem.set_u32(cd_body + 8, motion);
        let transforms = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&transforms);
        e.register_double(SET_MUL, move |e, a| {
            record
                .borrow_mut()
                .push((a[1], load_vec(e, a[2]), load_vec(e, a[2] + 0x30)));
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(None));
        let body = Rc::clone(&seen);
        e.register_double(fake(SLOT_CAST_RAY_WITH_COLLECTOR), move |e, a| {
            *body.borrow_mut() = Some((
                load_vec(e, a[1]),
                load_vec(e, a[1] + 0x10),
                load_bits_of(e, a[2]),
                a[3],
            ));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [5.0, 5.0, 5.0, 0.0]);
        store_vec(&mut e, input + 0x10, [1.0, 2.0, 3.0, 1.0]);
        let collector = e.mem.alloc(0x10);
        e.call(0x00dd61b0, &args![this, input, cd_body, collector]);
        let (from, to, body_words, passed_collector) = seen.borrow().unwrap();
        assert_eq!(from, [4.0, 3.0, 2.0, 0.0]);
        assert_eq!(to, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(passed_collector, collector);
        assert_eq!(body_words[0], child);
        assert_eq!(body_words[1], 0);
        assert_eq!(body_words[3], cd_body);
        let recorded = transforms.borrow();
        assert_eq!(recorded.len(), 1);
        assert_eq!(recorded[0].0, motion);
        assert_eq!(recorded[0].1, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(recorded[0].2, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(body_words[2], calls_to(&e, SET_MUL)[0][0]);
    }

    fn load_bits_of(e: &Engine, address: u32) -> [u32; 4] {
        std::array::from_fn(|lane| e.mem.u32(address + 4 * lane as u32))
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
        e.call(0x00dd5e70, &args![this, collector, 0x9999u32]);
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
    fn calc_size_for_spu_keeps_a_separate_child_apart() {
        let mut e = Engine::new();
        let (this, child) = shape(&mut e);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x30));
        let result = e.call(0x00dd5e10, &args![this, 7u32, 0x200u32]).i32();
        assert_eq!(result, 0x30);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x30);
        assert_eq!(
            calls_to(&e, fake(SLOT_CALC_SIZE_FOR_SPU)),
            vec![vec![child, 7, 0x1d0]]
        );
    }

    #[test]
    fn calc_size_for_spu_adds_an_inline_child() {
        let mut e = Engine::new();
        shape(&mut e);
        let block = e.mem.alloc(0x60);
        e.mem.set_u32(block + 0x18, block + 0x30);
        e.mem.set_u32(block + 0x1c, 0x1234);
        e.mem.set_u32(block + 0x30, CHILD_VTABLE);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x20));
        let result = e.call(0x00dd5e10, &args![block, 7u32, 0x200u32]).i32();
        assert_eq!(result, 0x50);
        assert_eq!(e.mem.u32(block + 0x1c), 0);
    }

    #[test]
    fn calc_size_for_spu_rejects_negative_and_oversized_answers() {
        let mut e = Engine::new();
        let (this, _) = shape(&mut e);
        e.mem.set_u32(this.addr() + 0x1c, 0x55);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(-1i32 as u32));
        assert_eq!(e.call(0x00dd5e10, &args![this, 7u32, 0x200u32]).i32(), -1);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x1d1));
        assert_eq!(e.call(0x00dd5e10, &args![this, 7u32, 0x200u32]).i32(), -1);
        e.register_double(fake(SLOT_CALC_SIZE_FOR_SPU), |_, _| ret(0x1d0));
        assert_eq!(e.call(0x00dd5e10, &args![this, 7u32, 0x200u32]).i32(), 0x30);
        assert_eq!(e.mem.u32(this.addr() + 0x1c), 0x1d0);
    }
}
