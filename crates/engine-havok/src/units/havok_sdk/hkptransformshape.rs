//! `Havok SDK/hkptransformshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpTransformShape` wraps a child shape (`m_childShape`, the `hkpSingleShapeContainer` at
//! `+0x10`) in a rigid transform (`m_transform`, an `hkTransform` at `+0x30`: three rotation
//! columns at `+0x30`, `+0x40`, `+0x50` and the translation at `+0x60`). SSE code computes
//! lane by lane in `f32`. Taking a vector into the child's space is the `SHUFPS`/`MULPS`
//! sequence `(v.y * cy + v.x * cx) + v.z * cz` with `cx = (c0.x, c1.x, c2.x, c2.y)`,
//! `cy = (c0.y, c1.y, c2.y, c2.w)` and `cz = (c0.z, c1.z, c2.z, c2.w)`; the fourth lane is
//! garbage but is kept, because it is passed on. The profiling records use the host clock for
//! `RDTSC` (see `run_00cf5070`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

layout! {
    /// `hkpTransformShape` (Xbox PDB), 0x70 bytes; offsets checked against the code below.
    pub struct HkpTransformShape: 0x70 {
        /// `m_childShape` (Xbox PDB): the `hkpSingleShapeContainer`'s shape, at +0x14 (its
        /// vtable is at +0x10).
        0x14 m_childShape: Ptr,
    }
}

/// `m_transform` (Xbox PDB) in the shape.
const TRANSFORM: u32 = 0x30;
/// `hkTransform::setMul` (Xbox PDB, `this` = result, then the two transforms).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// `hkVector4::setRotatedDir` (Xbox PDB, `this` = result, then the rotation and the vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// Profiling labels: `"TtrcTransform"` and the shared end label.
const LABEL_TRANSFORM: u32 = 0x010c_c448;
const LABEL_END: u32 = 0x010c_41f8;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// The vector `v` taken through the shape's rotation columns (the `UNPCKLPS`/`SHUFPS`/`MULPS`
/// sequence): every lane `i` is `(v.y * cy[i] + v.x * cx[i]) + v.z * cz[i]`.
fn through_columns(e: &Engine, this: Ptr<HkpTransformShape>, v: Vec4) -> Vec4 {
    let c0 = load_vec(e, this.addr() + TRANSFORM);
    let c1 = load_vec(e, this.addr() + TRANSFORM + 0x10);
    let c2 = load_vec(e, this.addr() + TRANSFORM + 0x20);
    let cx = [c0[0], c1[0], c2[0], c2[1]];
    let cy = [c0[1], c1[1], c2[1], c2[3]];
    let cz = [c0[2], c1[2], c2[2], c2[3]];
    std::array::from_fn(|i| (v[1] * cy[i] + v[0] * cx[i]) + v[2] * cz[i])
}

/// A point of the ray (`point` is the vector at `address`) taken into the child's space:
/// `(point - translation)` through the columns.
fn point_into_child_space(e: &Engine, this: Ptr<HkpTransformShape>, address: u32) -> Vec4 {
    let point = load_vec(e, address);
    let translation = load_vec(e, this.addr() + TRANSFORM + 0x30);
    let offset: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    through_columns(e, this, offset)
}

// Translated from 00cf58f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getContainer` (Xbox PDB): the shape container at `this + 0x10`.
pub fn get_container(_e: &mut Engine, this: Ptr<HkpTransformShape>) -> u32 {
    this.addr() + 0x10
}

// Translated from 00cf59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getAabbImpl` (Xbox PDB): composes `transform` with the shape's
/// transform into a local `hkTransform` (`hkTransform::setMul`) and asks the child for its
/// AABB with it (child slot `+0x1C`: the local transform, the tolerance as passed through the
/// x87 stack, and `out`).
pub fn get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    e.with_stack(0x50, |e, local| {
        e.call(
            TRANSFORM_SET_MUL,
            &args![local, transform, this.addr() + TRANSFORM],
        );
        let child = e.get(this, HkpTransformShape::m_childShape);
        e.vcall(
            child.addr(),
            0x1c,
            &args![local, x87_quiet(tolerance).to_bits(), out],
        );
    });
}

// Translated from 00cf5a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getMaximumProjection` (Xbox PDB): takes `direction` into the child's
/// space, asks the child (slot `+0xC`) for its maximum projection `s` along it, and returns
/// the projection of the transformed support: `s * local` is taken back through the rotation
/// columns, the translation is added, and the result is dotted with `direction`
/// (`z + (y + x)`).
pub fn get_maximum_projection(e: &mut Engine, this: Ptr<HkpTransformShape>, direction: Ptr) -> f32 {
    let v = load_vec(e, direction.addr());
    let local = through_columns(e, this, v);
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x18`, two pushes): 0x20 bytes from a
    // 16-byte boundary, the child-space direction at +0x10 (00cf5a90).
    let projection = e.with_stack(0x20 + 0x10, |e, raw| {
        let buffer = ((raw.addr() + 0xf) & !0xf) + 0x10;
        store_vec(e, buffer, local);
        let child = e.get(this, HkpTransformShape::m_childShape);
        x87_quiet(e.vcall(child.addr(), 0xc, &args![buffer]).f32())
    });
    let scaled: Vec4 = std::array::from_fn(|lane| projection * local[lane]);
    let c0 = load_vec(e, this.addr() + TRANSFORM);
    let c1 = load_vec(e, this.addr() + TRANSFORM + 0x10);
    let c2 = load_vec(e, this.addr() + TRANSFORM + 0x20);
    let c3 = load_vec(e, this.addr() + TRANSFORM + 0x30);
    let direction_vec = load_vec(e, direction.addr());
    let product: Vec4 = std::array::from_fn(|i| {
        let back = (scaled[1] * c1[i] + scaled[0] * c0[i]) + (scaled[2] * c2[i] + c3[i]);
        back * direction_vec[i]
    });
    x87_quiet(product[2] + (product[1] + product[0]))
}

// Translated from 00cf5b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::castRayImpl` (Xbox PDB; the `hkBool` result goes through the hidden
/// pointer `result`, which is returned): casts the ray against the child in the child's
/// space. The 0x30-byte input is copied with its `from` (`+0`) and `to` (`+0x10`) points
/// taken into the child's space, the output's child depth (`output + 0x40`) is raised for the
/// child's cast (slot `+0x20`), and on a hit the output's normal (`output + 0`) is rotated
/// back (`hkVector4::setRotatedDir`) and the child key recorded at
/// `output + 0x20 + depth * 4` is set to 0. Brackets the work with the `"TtrcTransform"` and
/// end profiling records.
pub fn cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, LABEL_TRANSFORM);
    let from = point_into_child_space(e, this, input.addr());
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x44`, three pushes): 0x50 bytes from a
    // 16-byte boundary, the normal's copy at +0x10 (00cf5c5d) and the child-space input at
    // +0x20 (00cf5b8c).
    e.with_stack(0x50 + 0x10, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        let local = frame + 0x20;
        let bytes = e.mem.bytes(input.addr(), 0x30);
        e.mem.write(local, &bytes);
        let to = point_into_child_space(e, this, input.addr() + 0x10);
        store_vec(e, local, from);
        store_vec(e, local + 0x10, to);
        let depth = e.mem.u32(output.addr() + 0x40);
        e.mem.set_u32(output.addr() + 0x40, depth.wrapping_add(1));
        let child = e.get(this, HkpTransformShape::m_childShape);
        e.vcall(child.addr(), 0x20, &args![result, local, output]);
        let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
        e.mem.set_u32(output.addr() + 0x40, depth);
        if e.mem.u8(result.addr()) != 0 {
            let normal = e.mem.bytes(output.addr(), 16);
            let copy = frame + 0x10;
            e.mem.write(copy, &normal);
            e.call(
                SET_ROTATED_DIR,
                &args![output, this.addr() + TRANSFORM, copy],
            );
            let depth = e.mem.u32(output.addr() + 0x40);
            e.mem
                .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), 0);
        }
    });
    profile_record(e, LABEL_END);
    result
}

// Translated from 00cf5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::castRayWithCollectorImpl` (Xbox PDB): casts the ray against the child
/// in the child's space, as `castRayImpl` does, and gives the child (slot `+0x24`) a body
/// record `(child, 0, composed transform, body)` where the transform is the body's transform
/// (`body + 8`) composed with the shape's (`hkTransform::setMul`). Brackets the work with the
/// `"TtrcTransform"` and end profiling records.
pub fn cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    input: Ptr,
    body: Ptr,
    collector: u32,
) {
    profile_record(e, LABEL_TRANSFORM);
    let from = point_into_child_space(e, this, input.addr());
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x84`, three pushes): 0x90 bytes from a
    // 16-byte boundary, the collector record at +0x10 (00cf5deb), the child-space input at
    // +0x20 (00cf5d7a) and the composed transform at +0x50 (00cf5dce).
    e.with_stack(0x90 + 0x10, |e, raw| {
        let frame = (raw.addr() + 0xf) & !0xf;
        let local = frame + 0x20;
        let to = point_into_child_space(e, this, input.addr() + 0x10);
        let bytes = e.mem.bytes(input.addr(), 0x30);
        e.mem.write(local, &bytes);
        store_vec(e, local, from);
        store_vec(e, local + 0x10, to);
        let composed = frame + 0x50;
        let body_transform = e.mem.u32(body.addr() + 8);
        e.call(
            TRANSFORM_SET_MUL,
            &args![composed, body_transform, this.addr() + TRANSFORM],
        );
        let child = e.get(this, HkpTransformShape::m_childShape);
        let record = frame + 0x10;
        e.mem.set_u32(record, child.addr());
        e.mem.set_u32(record + 4, 0);
        e.mem.set_u32(record + 8, composed);
        e.mem.set_u32(record + 12, body.addr());
        e.vcall(child.addr(), 0x24, &args![local, record, collector]);
    });
    profile_record(e, LABEL_END);
}

/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::calcContentStatistics` (Xbox PDB, `00c90360`: `this`, then the
/// collector and the class record).
const REFERENCED_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The word `calcContentStatistics` hands to it (`01268910`, zero in the file).
const CLASS_RECORD: u32 = 0x0126_8910;
/// The reference count calls on the child shape: `00c90510` adds one (`this` = the child),
/// `00c905b0` removes one (the map gives neither a name).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkQuaternion::set` (Xbox PDB, `00cb26e0`): `this` is the quaternion, then the rotation.
const QUATERNION_SET: u32 = 0x00cb_26e0;
/// The string `"ChildShape"` that `calcContentStatistics` hands to the statistics collector.
const CHILD_SHAPE_NAME: u32 = 0x010c_c40c;
/// The vtables the constructor and destructor write: the shape's (`010cc41c`), the container's
/// (`010c6fd0`, and `010c6fb0` once the child is released) and the base class's (`0102e388`).
const SHAPE_VTABLE: u32 = 0x010c_c41c;
const CONTAINER_VTABLE: u32 = 0x010c_6fd0;
const CONTAINER_BASE_VTABLE: u32 = 0x010c_6fb0;
const SHAPE_BASE_VTABLE: u32 = 0x0102_e388;

// Translated from 00cf58c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::calcContentStatistics` (Xbox PDB): asks the statistics `collector`
/// (slot `0xC`) to account the child shape (`+0x14`) under the name `"ChildShape"`
/// (`010cc40c`) with a last argument of 0, then runs `hkReferencedObject::calcContentStatistics`
/// (`00c90360`) with the collector and the class record `01268910`. The function ends
/// `RET 8`: the second stack word is not read.
pub fn calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    collector: Ptr,
    _unused_1: u32,
) {
    let child = e.get(this, HkpTransformShape::m_childShape);
    e.vcall(
        collector.addr(),
        0xc,
        &args![CHILD_SHAPE_NAME, child.addr(), 0u32],
    );
    e.call(
        REFERENCED_CALC_CONTENT_STATISTICS,
        &args![this, collector, CLASS_RECORD],
    );
}

// Translated from 00cf5900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::calcSizeForSpu` (Xbox PDB): asks the child (slot `0x18`) for its size
/// with `param` and the room left after the shape's own 0x70 bytes (`limit - 0x70`). A
/// negative answer or one above the room (signed compares) gives -1. When the child lives
/// inside the shape (`+0x14` = `this + 0x70`) the shape records 0 at `+0x18` and the size is
/// the child's plus 0x70; otherwise it records the child's size and the size is 0x70.
pub fn calc_size_for_spu(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    param: u32,
    limit: i32,
) -> i32 {
    let child = e.get(this, HkpTransformShape::m_childShape);
    let room = limit.wrapping_sub(0x70);
    let size = e.vcall(child.addr(), 0x18, &args![param, room]).i32();
    if !(0..=room).contains(&size) {
        return -1;
    }
    if e.mem.u32(this.addr() + 0x14) == this.addr().wrapping_add(0x70) {
        e.mem.set_u32(this.addr() + 0x18, 0);
        return size.wrapping_add(0x70);
    }
    e.mem.set_i32(this.addr() + 0x18, size);
    0x70
}

// Translated from 00cf5950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 64-byte `hkTransform` at `transform` to the shape's `m_transform` (`+0x30`, four
/// 16-byte vector copies, each read before it is written) and sets the shape's quaternion
/// (`+0x20`, `hkQuaternion::set`, `00cb26e0`) from the copy. The map gives it no name; that it
/// is `setTransform` is unconfirmed.
pub fn fn_00cf5950(e: &mut Engine, this: Ptr<HkpTransformShape>, transform: Ptr) {
    for offset in [0u32, 0x10, 0x20, 0x30] {
        let vector = e.mem.bytes(transform.addr() + offset, 0x10);
        e.mem.write(this.addr() + TRANSFORM + offset, &vector);
    }
    e.call(
        QUATERNION_SET,
        &args![this.addr() + 0x20, this.addr() + TRANSFORM],
    );
}

// Translated from 00cf5990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::hkpTransformShape` (Xbox PDB): fills the header (reference count
/// `+6` = 1, user data `+8` = 0, type `+0xC` = 0xF, vtables `010cc41c` at `+0` and `010c6fd0`
/// at `+0x10`), keeps `child` at `+0x14` and adds a reference to it (`00c90510`, called even
/// when `child` is null), then sets the transform (`00cf5950`). Returns `this`.
pub fn hkp_transform_shape_hkp_transform_shape(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    child: Ptr,
    transform: Ptr,
) -> Ptr<HkpTransformShape> {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 0xf);
    e.mem.set_u32(this.addr(), SHAPE_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, CONTAINER_VTABLE);
    e.mem.set_u32(this.addr() + 0x14, child.addr());
    e.call(ADD_REFERENCE, &args![child]);
    fn_00cf5950(e, this, transform);
    this
}

// Translated from 00cf5e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::~hkpTransformShape` as its scalar deleting destructor (Xbox PDB): puts
/// the container vtable back, removes a reference from the child when there is one
/// (`00c905b0`), sets the base vtables (`010c6fb0` at `+0x10`, `0102e388` at `+0`) and, when
/// bit 0 of `flags` is set, returns the block to the memory router's allocator (slot 8) with
/// the 16-bit size kept at `+4`. Returns `this`.
pub fn hkp_transform_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    flags: u8,
) -> Ptr<HkpTransformShape> {
    let child = e.mem.u32(this.addr() + 0x14);
    e.mem.set_u32(this.addr() + 0x10, CONTAINER_VTABLE);
    if child != 0 {
        e.call(REMOVE_REFERENCE, &args![child]);
    }
    e.mem.set_u32(this.addr() + 0x10, CONTAINER_BASE_VTABLE);
    e.mem.set_u32(this.addr(), SHAPE_BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_58f0, get_container(Ptr<HkpTransformShape>) -> u32),
        entry!(
            0x00cf_59e0,
            get_aabb_impl(Ptr<HkpTransformShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00cf_5a20,
            get_maximum_projection(Ptr<HkpTransformShape>, Ptr) -> f32
        ),
        entry!(
            0x00cf_5b20,
            cast_ray_impl(Ptr<HkpTransformShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cf_5cc0,
            cast_ray_with_collector_impl(Ptr<HkpTransformShape>, Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_58c0,
            calc_content_statistics(Ptr<HkpTransformShape>, Ptr, u32)
        ),
        entry!(
            0x00cf_5900,
            calc_size_for_spu(Ptr<HkpTransformShape>, u32, i32) -> i32
        ),
        entry!(0x00cf_5950, fn_00cf5950(Ptr<HkpTransformShape>, Ptr)),
        entry!(
            0x00cf_5990,
            hkp_transform_shape_hkp_transform_shape(
                Ptr<HkpTransformShape>,
                Ptr,
                Ptr,
            ) -> Ptr<HkpTransformShape>
        ),
        entry!(
            0x00cf_5e50,
            hkp_transform_shape_scalar_deleting_destructor(
                Ptr<HkpTransformShape>,
                u8,
            ) -> Ptr<HkpTransformShape>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_DOUBLE: u32 = 0x0900_0200;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e
    }

    /// A transform shape whose child has `CHILD_DOUBLE` in vtable slot `slot`; the rotation
    /// columns are c0 = (0, 1, 0, 0), c1 = (-1, 0, 0, 0), c2 = (0, 0, 1, 0) and the
    /// translation (1, 2, 3, 0).
    fn shape(e: &mut Engine, slot: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, CHILD_DOUBLE);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, vtable);
        let shape = e.mem.alloc(0x70);
        e.mem.set_u32(shape + 0x14, child);
        store_vec(e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, shape + 0x40, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, shape + 0x50, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, shape + 0x60, [1.0, 2.0, 3.0, 0.0]);
        (shape, child)
    }

    #[test]
    fn get_container_is_this_plus_0x10() {
        let mut e = engine();
        assert_eq!(e.call(0x00cf_58f0, &args![0x1000u32]).u32(), 0x1010);
    }

    #[test]
    fn get_aabb_impl_composes_the_transforms_and_asks_the_child() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x1c);
        let set_mul = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&set_mul);
        e.register_double(TRANSFORM_SET_MUL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let child_log = Rc::clone(&seen);
        e.register_double(CHILD_DOUBLE, move |_, a| {
            child_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        // A signalling NaN tolerance comes out quieted.
        e.call(0x00cf_59e0, &args![shape, transform, 0x7f80_0001u32, out]);
        let set_mul = set_mul.borrow();
        assert_eq!(set_mul.len(), 1);
        assert_eq!((set_mul[0][1], set_mul[0][2]), (transform, shape + 0x30));
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], child);
        assert_eq!(seen[0][1], set_mul[0][0]);
        assert_eq!(seen[0][2], 0x7fc0_0001);
        assert_eq!(seen[0][3], out);
    }

    #[test]
    fn get_maximum_projection_scales_the_childs_answer() {
        let mut e = engine();
        let (shape, _) = shape(&mut e, 0xc);
        let local = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&local);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            log.borrow_mut()
                .extend((0..4).map(|i| e.mem.f32(a[1] + 4 * i)));
            Ret {
                st0: 2.0,
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 2.0, 3.0, 0.0]);
        // The child-space direction sits at +0x10 of the exe's aligned 0x20-byte frame.
        let block = e.next_stack_local(0x30);
        let got = e.call(0x00cf_5a20, &args![shape, direction]).f32();
        assert_eq!(e.mem.f32(((block + 0xf) & !0xf) + 0x10), 2.0);
        // local = (2, -1, 3, 0); scaled = (4, -2, 6, 0); back (rotated by the columns, plus
        // the translation) = (-2 * -1 + 4 * 0 + 6 * 0 + 1, -2 * 0 + 4 * 1 + 0 + 2,
        // 0 + 0 + 6 + 3, ...) = (3, 6, 9); dotted with the direction: 3 + 12 + 27.
        assert_eq!(*local.borrow(), vec![2.0, -1.0, 3.0, 0.0]);
        assert_eq!(got, 42.0);
    }

    #[test]
    fn cast_ray_impl_transforms_the_ray_and_rotates_the_normal_on_a_hit() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x20);
        let seen = Rc::new(RefCell::new(Vec::new()));
        e.register_double(CHILD_DOUBLE, {
            let log = Rc::clone(&seen);
            move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..12).map(|i| e.mem.u32(a[2] + 4 * i)));
                words.push(e.mem.u32(a[3] + 0x40));
                log.borrow_mut().push(words);
                e.mem.set_u8(a[1], 1);
                Ret::default()
            }
        });
        let rotated = Rc::new(RefCell::new(Vec::new()));
        let rotated_log = Rc::clone(&rotated);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            rotated_log.borrow_mut().push((a.to_vec(), e.mem.u32(a[2])));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [2.0, 3.0, 4.0, 0.0]);
        store_vec(&mut e, input + 0x10, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(input + 0x24, 0x1234);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output, 0x3f80_0000);
        e.mem.set_u32(output + 0x40, 1);
        e.mem.set_u32(output + 0x20 + 4, 9);
        let result = e.mem.alloc(4);
        // The child-space input at +0x20 of the exe's aligned 0x50-byte frame.
        let block = e.next_stack_local(0x60);
        let frame = (block + 0xf) & !0xf;
        let got = e
            .call(0x00cf_5b20, &args![shape, result, input, output])
            .u32();
        assert_eq!(got, result);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let call = &seen[0];
        assert_eq!(call[2], frame + 0x20);
        assert_eq!((call[0], call[1], call[3]), (child, result, output));
        // from - translation = (1, 1, 1, 0) -> (1, -1, 1, 0); to - translation = 0.
        let floats: Vec<f32> = call[4..16].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(&floats[0..4], &[1.0, -1.0, 1.0, 0.0]);
        assert_eq!(&floats[4..8], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(call[4 + 9], 0x1234);
        assert_eq!(call[16], 2);
        assert_eq!(e.mem.u32(output + 0x40), 1);
        let rotated = rotated.borrow();
        assert_eq!(rotated.len(), 1);
        assert_eq!(
            &rotated[0].0[0..3],
            &[output, shape + 0x30, rotated[0].0[2]]
        );
        assert_eq!(rotated[0].1, 0x3f80_0000);
        assert_eq!(e.mem.u32(output + 0x20 + 4), 0);
    }

    #[test]
    fn cast_ray_impl_leaves_the_output_alone_on_a_miss() {
        let mut e = engine();
        let (shape, _) = shape(&mut e, 0x20);
        e.register_double(CHILD_DOUBLE, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.register_double(SET_ROTATED_DIR, |_, _| panic!("no hit, no rotation"));
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + 0x20, 5);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 7);
        e.call(0x00cf_5b20, &args![shape, result, input, output]);
        assert_eq!(e.mem.u32(output + 0x20), 5);
        assert_eq!(e.mem.u32(output + 0x40), 0);
    }

    #[test]
    fn cast_ray_with_collector_composes_the_body_transform() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x24);
        let composed = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&composed);
        e.register_double(TRANSFORM_SET_MUL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let child_log = Rc::clone(&seen);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            let ray: Vec<f32> = (0..8).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            child_log.borrow_mut().push((a.to_vec(), ray, record));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [2.0, 3.0, 4.0, 0.0]);
        store_vec(&mut e, input + 0x10, [3.0, 2.0, 1.0, 0.0]);
        let body = e.mem.alloc(0x10);
        let body_transform = e.mem.alloc(0x40);
        e.mem.set_u32(body + 8, body_transform);
        // The record at +0x10, the input at +0x20 and the transform at +0x50 of the exe's
        // aligned 0x90-byte frame.
        let block = e.next_stack_local(0xa0);
        let frame = (block + 0xf) & !0xf;
        e.call(0x00cf_5cc0, &args![shape, input, body, 0x5555u32]);
        assert_eq!(seen.borrow()[0].0[1..3], [frame + 0x20, frame + 0x10]);
        let composed = composed.borrow();
        assert_eq!(composed[0][0], frame + 0x50);
        assert_eq!(composed.len(), 1);
        assert_eq!(
            (composed[0][1], composed[0][2]),
            (body_transform, shape + 0x30)
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (words, ray, record) = &seen[0];
        assert_eq!(words[0], child);
        assert_eq!(words[3], 0x5555);
        assert_eq!(&ray[0..4], &[1.0, -1.0, 1.0, 0.0]);
        assert_eq!(&ray[4..8], &[0.0, -2.0, -2.0, 0.0]);
        assert_eq!(*record, vec![child, 0, composed[0][0], body]);
    }

    /// A double at `address` that logs the argument words of every call and answers `eax`.
    fn log_calls(e: &mut Engine, address: u32, eax: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn calc_content_statistics_names_the_child_and_runs_the_base() {
        let mut e = engine();
        let collector_slot = 0x0900_0300;
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, collector_slot);
        let collector = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        let named = log_calls(&mut e, collector_slot, 0);
        let base = log_calls(&mut e, REFERENCED_CALC_CONTENT_STATISTICS, 0);
        let shape = e.mem.alloc(0x70);
        e.mem.set_u32(shape + 0x14, 0x7777);
        e.call(0x00cf_58c0, &args![shape, collector, 0x1234u32]);
        assert_eq!(
            *named.borrow(),
            vec![vec![collector, 0x010c_c40c, 0x7777, 0]]
        );
        assert_eq!(*base.borrow(), vec![vec![shape, collector, 0x0126_8910]]);
    }

    /// Runs `calcSizeForSpu` on a shape whose child answers `size`; `inline` puts the child
    /// at `shape + 0x70`. Returns the result, the word at `+0x18` and the child's arguments.
    fn calc_size(size: i32, inline: bool, limit: i32) -> (i32, u32, Vec<Vec<u32>>) {
        let mut e = engine();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, CHILD_DOUBLE);
        let shape = e.mem.alloc(0x100);
        let child = if inline { shape + 0x70 } else { shape + 0xb0 };
        e.mem.set_u32(child, vtable);
        e.mem.set_u32(shape + 0x14, child);
        e.mem.set_u32(shape + 0x18, 0x5a5a);
        let asked = log_calls(&mut e, CHILD_DOUBLE, size as u32);
        let result = e.call(0x00cf_5900, &args![shape, 0x99u32, limit]).i32();
        let recorded = e.mem.u32(shape + 0x18);
        let asked = asked.borrow().clone();
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][..1], [child]);
        (result, recorded, asked)
    }

    #[test]
    fn calc_size_for_spu_passes_the_room_after_the_shape() {
        let (_, _, asked) = calc_size(0x20, false, 0x200);
        assert_eq!(asked[0][1..], [0x99, 0x190]);
    }

    #[test]
    fn calc_size_for_spu_refuses_negative_and_too_big_sizes() {
        let (result, recorded, _) = calc_size(-1, false, 0x200);
        assert_eq!((result, recorded), (-1, 0x5a5a));
        assert_eq!(calc_size(0x191, false, 0x200).0, -1);
        // The room itself still fits; the shape's fields stay as they were on a refusal.
        assert_eq!(calc_size(0x190, false, 0x200).0, 0x70);
        assert_eq!(calc_size(0x191, false, 0x200).1, 0x5a5a);
        // A limit below 0x70 leaves a negative room that nothing fits in.
        assert_eq!(calc_size(0, false, 0x10).0, -1);
    }

    #[test]
    fn calc_size_for_spu_records_the_child_size_unless_it_is_inline() {
        let (result, recorded, _) = calc_size(0x20, false, 0x200);
        assert_eq!((result, recorded), (0x70, 0x20));
        let (result, recorded, _) = calc_size(0x20, true, 0x200);
        assert_eq!((result, recorded), (0x90, 0));
    }

    #[test]
    fn set_transform_copies_the_matrix_and_sets_the_quaternion() {
        let mut e = engine();
        let quaternion = log_calls(&mut e, QUATERNION_SET, 0);
        let source = e.mem.alloc(0x40);
        for offset in 0..0x40 {
            e.mem.set_u8(source + offset, offset as u8 + 1);
        }
        let shape = e.mem.alloc(0x70);
        e.call(0x00cf_5950, &args![shape, source]);
        for offset in 0..0x40 {
            assert_eq!(e.mem.u8(shape + 0x30 + offset), offset as u8 + 1);
        }
        assert_eq!(*quaternion.borrow(), vec![vec![shape + 0x20, shape + 0x30]]);
    }

    #[test]
    fn construction_fills_the_header_and_references_the_child() {
        let mut e = engine();
        let added = log_calls(&mut e, ADD_REFERENCE, 0);
        log_calls(&mut e, QUATERNION_SET, 0);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x3c, 0xfeed);
        let shape = e.mem.alloc(0x70);
        e.mem.set_u16(shape + 4, 0x70);
        let result = e.call(0x00cf_5990, &args![shape, 0x4242u32, source]).u32();
        assert_eq!(result, shape);
        assert_eq!(e.mem.u16(shape + 4), 0x70);
        assert_eq!(e.mem.u16(shape + 6), 1);
        assert_eq!(e.mem.u32(shape + 8), 0);
        assert_eq!(e.mem.u32(shape + 0xc), 0xf);
        assert_eq!(e.mem.u32(shape), 0x010c_c41c);
        assert_eq!(e.mem.u32(shape + 0x10), 0x010c_6fd0);
        assert_eq!(e.mem.u32(shape + 0x14), 0x4242);
        assert_eq!(e.mem.u32(shape + 0x6c), 0xfeed);
        // The reference is added even to a null child.
        assert_eq!(*added.borrow(), vec![vec![0x4242]]);
        e.call(0x00cf_5990, &args![shape, 0u32, source]);
        assert_eq!(added.borrow()[1], vec![0]);
    }

    #[test]
    fn destruction_releases_the_child_and_frees_on_bit_zero() {
        for (child, flags, freed) in [
            (0x4242u32, 0xffff_fffeu32, false),
            (0, 1, true),
            (0x4242, 0x101, true),
        ] {
            let mut e = engine();
            let removed = log_calls(&mut e, REMOVE_REFERENCE, 0);
            let free_slot = 0x0900_0308;
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 8, free_slot);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            let freed_log = log_calls(&mut e, free_slot, 0);
            let shape = e.mem.alloc(0x70);
            e.mem.set_u16(shape + 4, 0x70);
            e.mem.set_u32(shape + 0x14, child);
            let result = e.call(0x00cf_5e50, &args![shape, flags]).u32();
            assert_eq!(result, shape);
            assert_eq!(e.mem.u32(shape), 0x0102_e388);
            assert_eq!(e.mem.u32(shape + 0x10), 0x010c_6fb0);
            let expected: Vec<Vec<u32>> = if child == 0 {
                vec![]
            } else {
                vec![vec![child]]
            };
            assert_eq!(*removed.borrow(), expected);
            let expected: Vec<Vec<u32>> = if freed {
                vec![vec![allocator, shape, 0x70]]
            } else {
                vec![]
            };
            assert_eq!(*freed_log.borrow(), expected);
        }
    }
}
