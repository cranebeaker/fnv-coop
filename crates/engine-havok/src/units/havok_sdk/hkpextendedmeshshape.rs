//! `Havok SDK/hkpextendedmeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpExtendedMeshShape` holds triangle subparts (0x60 bytes each) and convex-shape subparts
//! (0x40 bytes each); a shape key packs the subpart index in its high bits (the number of low
//! bits is `m_numBitsForSubpartIndex`) and the child index in the low bits, and bit 31 selects
//! the convex-shape subparts. The container functions (`getFirstKey`-style, vtable at
//! `010cfd20`) take the container embedded at shape `+0x10` as `this`. The field names below
//! follow the Havok SDK's structure; the PDB dump has no entry for the class, so they and the
//! container function roles (read off the vtable layout) are unconfirmed.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpExtendedMeshShape` (Havok SDK; not in the PDB dump, offsets checked against the
    /// code below).
    pub struct HkpExtendedMeshShape: 0x100 {
        /// The number of low key bits that hold the child index.
        0xb0 m_numBitsForSubpartIndex: u32,
        /// The data of the triangle subpart array (0x60-byte entries).
        0xb4 m_trianglesSubpartsData: u32,
        /// Its element count.
        0xb8 m_trianglesSubpartsSize: u32,
        /// The data of the convex-shape subpart array (0x40-byte entries).
        0xc0 m_shapesSubpartsData: u32,
        /// Its element count.
        0xc4 m_shapesSubpartsSize: u32,
        /// The data of the per-triangle welding info (`u16` each).
        0xcc m_weldingInfoData: u32,
        /// Its element count.
        0xd0 m_weldingInfoSize: u32,
        /// The welding type byte copied into every triangle.
        0xd8 m_weldingType: u8,
        /// The collision filter info used for keys without a material.
        0xdc m_defaultCollisionFilterInfo: u32,
        /// The convex radius copied into every triangle.
        0xe4 m_radius: u32,
    }
}

/// `hkpTriangleUtil::isDegenerate` (Xbox PDB, cdecl: result byte pointer, three vertices,
/// tolerance; returns the result byte pointer).
const IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// The degeneracy tolerance (a float).
const DEGENERACY_TOLERANCE: u32 = 0x011b_2e90;
/// The default convex radius of a triangle (a float).
const TRIANGLE_RADIUS: u32 = 0x011b_153c;
/// The vtable of a triangle shape, the vtable of a convex-transform shape and the mask that
/// clears the fourth lane of a vector (`ffffffff ffffffff ffffffff 00000000`).
const TRIANGLE_VTABLE: u32 = 0x010c_749c;
const CONVEX_TRANSFORM_VTABLE: u32 = 0x010d_0e5c;
const LANE_MASK: u32 = 0x010c_4c10;
/// `hkpConvexTransformShapeBase` constructor (Xbox PDB), `hkpConvexTransformShape`
/// constructor (Xbox PDB) and the routine that builds a transform from a rotation.
const CONVEX_TRANSFORM_BASE: u32 = 0x00c9_e340;
const CONVEX_TRANSFORM: u32 = 0x00d5_d7a0;
const SET_TRANSFORM: u32 = 0x00cb_2d90;
/// The shape type of a triangle shape.
const TRIANGLE_TYPE: u32 = 3;

/// The container (shape `+0x10`) seen as the shape.
fn shape_of(container: Ptr) -> Ptr<HkpExtendedMeshShape> {
    Ptr::new(container.addr().wrapping_sub(0x10))
}

/// Splits a key into its subpart index (above bit 31, below the child bits) and child index.
fn split_key(bits: u32, key: u32) -> (u32, u32) {
    (
        (key & 0x7fff_ffff) >> (32u32.wrapping_sub(bits) & 31),
        (0xffff_ffffu32 >> (bits & 31)) & key,
    )
}

// Translated from 00d57090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x60-byte triangle subpart from `source` to `dest`, four words at a time (cdecl;
/// the function is unnamed and its class unconfirmed).
pub fn fn_00d57090(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..24 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00d570d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x40-byte convex-shape subpart from `source` to `dest` (cdecl; the function is
/// unnamed and its class unconfirmed).
pub fn fn_00d570d0(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..16 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00d55530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::getMeshMaterial` (Xbox PDB): the address of the material of the
/// triangle or convex shape `key`, or 0 when there is none. The subpart is the embedded one
/// (`+0x20`) when there is exactly one triangle subpart, else entry `index` of the subpart
/// array (convex-shape subparts are not bounds checked). A subpart holds the material
/// base (`+4`), element type (`+1`: 1 is a byte, else a `u16`), the record stride (`+8`), the
/// material stride (`+2`, signed) and the first material (`+0xC`).
pub fn get_mesh_material(e: &mut Engine, this: Ptr<HkpExtendedMeshShape>, key: u32) -> u32 {
    let bits = e.get(this, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (index, child) = split_key(bits, key);
    let subpart = if (key as i32) < 0 {
        let array = e.get(this, HkpExtendedMeshShape::m_shapesSubpartsData);
        (index << 6).wrapping_add(array)
    } else {
        let count = e.get(this, HkpExtendedMeshShape::m_trianglesSubpartsSize);
        if count == 1 {
            this.addr() + 0x20
        } else if index >= count {
            return 0;
        } else {
            let array = e.get(this, HkpExtendedMeshShape::m_trianglesSubpartsData);
            index.wrapping_mul(0x60).wrapping_add(array)
        }
    };
    let materials = e.mem.u32(subpart + 4);
    let first = e.mem.u32(subpart + 0xc);
    let stride = i32::from(e.mem.u16(subpart + 2) as i16) as u32;
    if materials == 0 {
        return 0;
    }
    let record = u32::from(e.mem.u16(subpart + 8))
        .wrapping_mul(child)
        .wrapping_add(materials);
    let value = if e.mem.u8(subpart + 1) == 1 {
        u32::from(e.mem.u8(record))
    } else {
        u32::from(e.mem.u16(record))
    };
    value.wrapping_mul(stride).wrapping_add(first)
}

// Translated from 00d555d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's collision filter info of `key` (unconfirmed role): the first word of the
/// key's material (`getMeshMaterial` of the shape), or the shape's default collision filter
/// info when it has none.
pub fn fn_00d555d0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = shape_of(this);
    let material = e.call(0x00d5_5530, &args![shape, key]).u32();
    if material != 0 {
        e.mem.u32(material)
    } else {
        e.get(shape, HkpExtendedMeshShape::m_defaultCollisionFilterInfo)
    }
}

/// Whether the triangle shape `triangle` (its vertices at `+0x20`, `+0x30` and `+0x40`) is
/// degenerate; `result` is the scratch byte `isDegenerate` writes.
fn is_degenerate(e: &mut Engine, triangle: u32, result: u32) -> bool {
    let tolerance = x87_quiet_bits(e.mem.u32(DEGENERACY_TOLERANCE));
    let flag = e
        .call(
            IS_DEGENERATE,
            &args![
                result,
                triangle + 0x20,
                triangle + 0x30,
                triangle + 0x40,
                tolerance
            ],
        )
        .u32();
    e.mem.u8(flag) != 0
}

// Translated from 00d55b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's first shape key (unconfirmed role): 0xffffffff when there are no subparts,
/// else the key 0 of the first triangle subpart (or `0x80000000`, the first convex shape,
/// when there are no triangle subparts). A triangle child that is degenerate is skipped
/// through the container's slot `+0xC`.
pub fn fn_00d55b40(e: &mut Engine, this: Ptr) -> u32 {
    let shape = shape_of(this);
    let triangles = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize);
    let shapes = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize);
    if shapes.wrapping_add(triangles) == 0 {
        return 0xffff_ffff;
    }
    let key = if triangles != 0 { 0 } else { 0x8000_0000 };
    e.with_stack(0x210, |e, buffer| {
        let child = e.vcall(this.addr(), 0x14, &args![key, buffer]).u32();
        if e.mem.u32(child + 0xc) == TRIANGLE_TYPE && is_degenerate(e, child, buffer.addr() + 0x200)
        {
            e.vcall(this.addr(), 0xc, &args![key]).u32()
        } else {
            key
        }
    })
}

// Translated from 00d55bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's next shape key after `key` (unconfirmed role): the next child of the
/// subpart, else child 0 of the next subpart, else the first convex shape after the triangle
/// subparts, else 0xffffffff. A triangle child that is degenerate is skipped.
pub fn fn_00d55bd0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = shape_of(this);
    let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (mut index, mut child) = split_key(bits, key);
    let mut sign = key & 0x8000_0000;
    e.with_stack(0x210, |e, buffer| loop {
        child = child.wrapping_add(1);
        if sign == 0 {
            let array = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsData);
            let count = e
                .mem
                .u32(array.wrapping_add(index.wrapping_mul(0x60)) + 0x14);
            if (child as i32) >= (count as i32) {
                index = index.wrapping_add(1);
                child = 0;
                if index >= e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize) {
                    if e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize) == 0 {
                        return 0xffff_ffff;
                    }
                    index = 0;
                    sign = 0x8000_0000;
                    child = 0xffff_ffff;
                    continue;
                }
            }
        } else {
            let array = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsData);
            let count = e.mem.u32(array.wrapping_add(index << 6) + 0x18);
            if (child as i32) >= (count as i32) {
                index = index.wrapping_add(1);
                if index >= e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize) {
                    return 0xffff_ffff;
                }
                child = 0;
            }
        }
        let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
        let next = (index << (32u32.wrapping_sub(bits) & 31)) | sign | child;
        let found = e.vcall(this.addr(), 0x14, &args![next, buffer]).u32();
        if e.mem.u32(found + 0xc) != TRIANGLE_TYPE
            || !is_degenerate(e, found, buffer.addr() + 0x200)
        {
            return next;
        }
    })
}

/// The three vertex indices of the triangle whose index record starts at `record`, read as
/// bytes (type 1), `u16` (2) or `u32` (3); `selector` is the
/// record's winding bit. Other index types give zeros.
fn read_triangle_indices(e: &Engine, index_type: i32, record: u32, selector: i32) -> [u32; 3] {
    let second = i32::from(selector.wrapping_add(1) as i16) as u32;
    let third = i32::from((selector ^ 1).wrapping_add(1) as i16) as u32;
    match index_type {
        1 => [
            u32::from(e.mem.u8(record)),
            u32::from(e.mem.u8(record.wrapping_add(second))),
            u32::from(e.mem.u8(record.wrapping_add(third))),
        ],
        2 => [
            u32::from(e.mem.u16(record)),
            u32::from(e.mem.u16(record.wrapping_add(second.wrapping_mul(2)))),
            u32::from(e.mem.u16(record.wrapping_add(third.wrapping_mul(2)))),
        ],
        3 => [
            e.mem.u32(record),
            e.mem.u32(record.wrapping_add(second.wrapping_mul(4))),
            e.mem.u32(record.wrapping_add(third.wrapping_mul(4))),
        ],
        _ => [0, 0, 0],
    }
}

/// The triangle part of `getChildShape`: fills the triangle shape `out` from the triangle
/// `child` of `subpart`.
fn fill_triangle(
    e: &mut Engine,
    shape: Ptr<HkpExtendedMeshShape>,
    subpart: u32,
    child: u32,
    out: u32,
) {
    let vertices = e.mem.u32(subpart + 0x18);
    let vertex_stride = e.mem.u32(subpart + 0x1c);
    let record = e
        .mem
        .u32(subpart + 0x44)
        .wrapping_mul(child)
        .wrapping_add(e.mem.u32(subpart + 0x40));
    let selector = i32::from(e.mem.u8(subpart + 0x49) as i8) & (child as i32);
    let index_type = i32::from(e.mem.u8(subpart + 0x48) as i8);
    let indices = read_triangle_indices(e, index_type, record, selector);
    // The triangle shape's constructor defaults, then the shape's own values.
    e.mem.set_u16(out + 6, 1);
    let radius = x87_quiet_bits(e.mem.u32(TRIANGLE_RADIUS));
    e.mem.set_u32(out + 0x10, radius);
    e.mem.set_u32(out + 8, 0);
    e.mem.set_u32(out + 0xc, TRIANGLE_TYPE);
    e.mem.set_u32(out, TRIANGLE_VTABLE);
    e.mem.set_u16(out + 0x14, 0);
    e.mem.set_u8(out + 0x16, 6);
    for lane in 0..4 {
        e.mem.set_u32(out + 0x50 + 4 * lane, 0);
    }
    e.mem.set_u8(out + 0x17, 0);
    let welding_type = e.get(shape, HkpExtendedMeshShape::m_weldingType);
    e.mem.set_u8(out + 0x16, welding_type);
    let shape_radius = x87_quiet_bits(e.get(shape, HkpExtendedMeshShape::m_radius));
    e.mem.set_u32(out + 0x10, shape_radius);
    // The extrusion vector and whether it is non-zero (`0 < x*x + (y*y + z*z)` in this order).
    let extrusion: [f32; 4] =
        std::array::from_fn(|lane| e.mem.f32(subpart + 0x30 + 4 * lane as u32));
    let squares = extrusion.map(|lane| lane * lane);
    let length = squares[2] + (squares[1] + squares[0]);
    e.mem.set_u32(out + 0xc, TRIANGLE_TYPE);
    e.mem.set_u8(out + 0x17, u8::from(0.0 < length));
    for lane in 0..4 {
        let word = e.mem.u32(subpart + 0x30 + 4 * lane);
        e.mem.set_u32(out + 0x50 + 4 * lane, word);
    }
    // Welding info of the triangle.
    let welding = if e.get(shape, HkpExtendedMeshShape::m_weldingInfoSize) == 0 {
        0
    } else {
        let first = e.mem.u32(subpart + 0x4c).wrapping_add(child);
        let data = e.get(shape, HkpExtendedMeshShape::m_weldingInfoData);
        e.mem.u16(data.wrapping_add(first.wrapping_mul(2)))
    };
    e.mem.set_u16(out + 0x14, welding);
    // The scaled vertices; the fourth lane of every vertex is a stale stack word (scaled), so
    // it is read from a scratch block that is never written there.
    let scale: [f32; 4] =
        std::array::from_fn(|lane| e.mem.f32(shape.addr() + 0x80 + 4 * lane as u32));
    e.with_stack(0x20, |e, scratch| {
        let stale = e.mem.f32(scratch.addr() + 0xc);
        for (slot, index) in indices.into_iter().enumerate() {
            let address = index.wrapping_mul(vertex_stride).wrapping_add(vertices);
            let mut vertex = [0.0f32; 4];
            for (lane, value) in vertex.iter_mut().take(3).enumerate() {
                let bits = x87_quiet_bits(e.mem.u32(address + 4 * lane as u32));
                *value = f32::from_bits(bits);
            }
            vertex[3] = stale;
            for (lane, value) in vertex.iter().enumerate() {
                e.mem.set_f32(
                    out + 0x20 + 0x10 * slot as u32 + 4 * lane as u32,
                    value * scale[lane],
                );
            }
        }
    });
}

/// The convex-shape part of `getChildShape`: `child` of the (copied) subpart `local`; returns
/// the shape that is the answer, wrapped in a convex transform shape in `out` when the
/// subpart's kind asks for it.
fn convex_child(e: &mut Engine, local: u32, child: u32, out: Ptr) -> Ptr {
    let list = e.mem.u32(local + 0x14);
    let shape = e.mem.u32(list.wrapping_add(child.wrapping_mul(4)));
    let kind = e.mem.u32(local + 0x3c) & 0xc0ff_ffff;
    if kind == 0 {
        return Ptr::new(shape);
    }
    if kind == 1 {
        if out.is_null() {
            return Ptr::NULL;
        }
        let radius = x87_quiet(e.mem.f32(shape + 0x10)).to_bits();
        e.call(
            CONVEX_TRANSFORM_BASE,
            &args![out, 0xbu32, radius, shape, 0u32],
        );
        for lane in 0..4 {
            let word = e.mem.u32(local + 0x30 + 4 * lane) & e.mem.u32(LANE_MASK + 4 * lane);
            e.mem.set_u32(out.addr() + 0x20 + 4 * lane, word);
        }
        e.mem.set_u32(out.addr(), CONVEX_TRANSFORM_VTABLE);
        e.mem.set_u32(out.addr() + 0x1c, 0);
        return out;
    }
    e.with_stack(0x40, |e, transform| {
        for lane in 0..4 {
            let word = e.mem.u32(local + 0x30 + 4 * lane);
            e.mem.set_u32(transform.addr() + 0x30 + 4 * lane, word);
        }
        e.call(SET_TRANSFORM, &args![transform, local + 0x20]);
        if out.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(
                e.call(CONVEX_TRANSFORM, &args![out, shape, transform, 0u32])
                    .u32(),
            )
        }
    })
}

// Translated from 00d55ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::getChildShape` (Xbox PDB; `this` is the container at shape `+0x10`):
/// the shape of `key`. A triangle key fills the triangle shape `out` (type 3, from the three
/// vertices of the subpart's index record, scaled by the shape's scaling at `+0x80`) and
/// returns it; a null `out` crashes in the exe (it is written through) and is not supported.
/// A convex-shape key returns the stored shape itself (kind 0), or wraps it in a convex
/// transform shape built in `out` (kind 1: translation only, else with a rotation); the
/// wrapped forms return null for a null `out`.
pub fn get_child_shape(e: &mut Engine, this: Ptr, key: u32, out: Ptr) -> Ptr {
    let shape = shape_of(this);
    let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (index, child) = split_key(bits, key);
    if (key as i32) >= 0 {
        return e.with_stack(0x60, |e, local| {
            let subpart = if e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize) == 1 {
                this.addr() + 0x10
            } else {
                let array = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsData);
                e.call(
                    0x00d5_7090,
                    &args![local, index.wrapping_mul(0x60).wrapping_add(array)],
                );
                local.addr()
            };
            fill_triangle(e, shape, subpart, child, out.addr());
            out
        });
    }
    e.with_stack(0x40, |e, local| {
        let array = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsData);
        e.call(0x00d5_70d0, &args![local, (index << 6).wrapping_add(array)]);
        convex_child(e, local.addr(), child, out)
    })
}

/// The memory router (`hkmemoryrouter.cpp`, no map name): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_UTIL_RESERVE: u32 = 0x00c9_08c0;
/// The reference count increment and release of `hkReferencedObject` (no map names; `this` =
/// the object).
const REFERENCED_OBJECT_ADD: u32 = 0x00c9_0510;
const REFERENCED_OBJECT_RELEASE: u32 = 0x00c9_05b0;
/// `hkVector4::setTransformedPos` (Xbox PDB; `this` = out, then `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkQuaternion::set` (Xbox PDB; `this` = the quaternion, then the transform).
const QUATERNION_SET: u32 = 0x00cb_26e0;
/// `hkMatrix3::isApproximatelyEqual` (Xbox PDB; `this`, then `(result byte, other, tolerance)`;
/// answers the result pointer).
const MATRIX_IS_APPROXIMATELY_EQUAL: u32 = 0x00cd_4e00;
/// The quaternion angle (no map name; `this` = the quaternion; the angle comes back in `ST0`).
const QUATERNION_GET_ANGLE: u32 = 0x00c7_4c20;
/// `hkAabbUtil::calcAabb_ov3` (Xbox PDB, cdecl `(transform, half extents, center, tolerance,
/// out)`).
const CALC_AABB: u32 = 0x00d2_6230;
/// `hkpShapeCollection::hkpShapeCollection` (Xbox PDB; `this`, then `(shape type, flag)`).
const SHAPE_COLLECTION_CONSTRUCT: u32 = 0x00d2_1410;
/// `hkpShapeContainer::getNumChildShapes` (Xbox PDB; `this` = the container).
const CONTAINER_GET_NUM_CHILD_SHAPES: u32 = 0x00d1_f1e0;
/// The copy constructors of a triangle subpart (0x60 bytes) and of a convex-shape subpart
/// (0x40 bytes) (no map names; `this` = the new subpart, then the source).
const TRIANGLES_SUBPART_COPY: u32 = 0x00d4_abf0;
const SHAPES_SUBPART_COPY: u32 = 0x00d4_ad60;
/// The vtables of `hkpExtendedMeshShape` and of its embedded container (at `+0x10`), and the
/// shape type `hkpShapeCollection`'s constructor is given.
const EXTENDED_MESH_SHAPE_VTABLE: u32 = 0x010d_0ecc;
const EXTENDED_MESH_CONTAINER_VTABLE: u32 = 0x010d_0ea8;
const EXTENDED_MESH_SHAPE_TYPE: u32 = 0xe;
/// The identity transform global and the zero translation (16 bytes) a subpart's translation
/// is compared with.
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
const ZERO_TRANSLATION: u32 = 0x0126_7e30;
/// The comparison tolerance (a float) and the 16-byte mask that clears the sign bits.
const TOLERANCE: u32 = 0x0101_7d00;
const ABS_MASK: u32 = 0x0101_7d10;
/// The sign-bit mask (16 bytes) and the float that starts an empty box: the minimum lanes get
/// the float, the maximum lanes the float xor the lanes of the mask.
const SIGN_MASK: u32 = 0x010c_44d0;
const EMPTY_BOX_FILL: u32 = 0x0104_da48;
/// The fourth lane of a vector built from three products (a float).
const FOURTH_LANE: u32 = 0x0101_1d78;
/// The four factors (0.5) that turn minimum and maximum into center and half extents.
const HALF_FACTORS: u32 = 0x010c_71d0;
/// The identity quaternion (16 bytes) a translation-only convex subpart starts with.
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
/// The initial scaling (a float) and the initial half extents (a float).
const INITIAL_SCALING: u32 = 0x0101_2068;
const INITIAL_HALF_EXTENTS: u32 = 0x0104_e148;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, address: u32, vector: Vec4) {
    for (lane, value) in vector.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src`.
fn minps(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` per lane: `dst` when it is greater, else `src`.
fn maxps(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `FLD m32; FMUL m32; FSTP m32`: the product of two floats (`a` quieted by the load, `b` the
/// memory operand), with the x87 choice of NaN (#199).
fn x87_product(a: f32, b: f32) -> f32 {
    x87_mul(f64::from(x87_quiet(a)), x87_m32(b.to_bits())) as f32
}

fn router_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Starts an empty box at `aabb` (minimum, then maximum, 16 bytes each).
fn empty_aabb(e: &mut Engine, aabb: u32) {
    let fill = e.mem.u32(EMPTY_BOX_FILL);
    for lane in 0..4 {
        e.mem.set_u32(aabb + 4 * lane, fill);
    }
    for lane in 0..4 {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(aabb + 0x10 + 4 * lane, mask ^ fill);
    }
}

/// Grows the box at `aabb` by `low` and `high` (`MINPS` / `MAXPS` with the box as destination).
fn grow_aabb(e: &mut Engine, aabb: u32, low: Vec4, high: Vec4) {
    let new_low = zip_with(load_vec(e, aabb), low, minps);
    store_vec(e, aabb, new_low);
    let new_high = zip_with(load_vec(e, aabb + 0x10), high, maxps);
    store_vec(e, aabb + 0x10, new_high);
}

/// Stores the center `(high + low) * 0.5` at `+0xa0` and the half extents `(high - low) * 0.5`
/// at `+0x90` of the shape.
fn store_center_and_extents(e: &mut Engine, shape: u32, low: Vec4, high: Vec4) {
    let factors = load_vec(e, HALF_FACTORS);
    let center = zip_with(zip_with(high, low, |h, l| h + l), factors, |s, f| s * f);
    store_vec(e, shape + 0xa0, center);
    let extents = zip_with(zip_with(high, low, |h, l| h - l), factors, |s, f| s * f);
    store_vec(e, shape + 0x90, extents);
}

/// The corner `first * second` (lane by lane, three lanes; the fourth lane is the float at
/// `01011d78`) moved by `transform` (`setTransformedPos`) and grown into the box at `aabb`.
fn grow_by_scaled_corner(e: &mut Engine, first: u32, second: u32, transform: u32, aabb: u32) {
    let products: [f32; 3] = std::array::from_fn(|lane| {
        let offset = 4 * lane as u32;
        x87_product(e.mem.f32(first + offset), e.mem.f32(second + offset))
    });
    let fourth = e.mem.u32(FOURTH_LANE);
    let moved = e.with_stack(0x20, |e, scratch| {
        let vector = scratch.addr();
        let out = vector + 0x10;
        store_vec(e, vector, [products[0], products[1], products[2], 0.0]);
        e.mem.set_u32(vector + 0xc, fourth);
        e.call(SET_TRANSFORMED_POS, &args![out, transform, vector]);
        load_vec(e, out)
    });
    grow_aabb(e, aabb, moved, moved);
}

/// Whether the translation vector at `vector` differs from the reference at `01267e30` by more
/// than the tolerance (the float at `01017d00`) in any of the first three lanes (`SUBPS`,
/// `ANDPS` with the sign mask, `CMPLEPS`; a NaN lane counts as different).
fn translation_differs(e: &Engine, vector: u32) -> bool {
    let tolerance = e.mem.f32(TOLERANCE);
    (0..3).any(|lane| {
        let difference = e.mem.f32(vector + 4 * lane) - e.mem.f32(ZERO_TRANSLATION + 4 * lane);
        let magnitude = f32::from_bits(difference.to_bits() & e.mem.u32(ABS_MASK + 4 * lane));
        !matches!(
            magnitude.partial_cmp(&tolerance),
            Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
        )
    })
}

// Translated from 00d55520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::_getNumChildShapesInShapesSubpart` (Xbox PDB; a function of its
/// argument only): the number of children (`+0x18`) of the convex-shape subpart `part`.
pub fn get_num_child_shapes_in_shapes_subpart(e: &mut Engine, part: u32) -> u32 {
    e.mem.u32(part + 0x18)
}

// Translated from 00d55600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Register inputs first: `first` (`ECX`) and `second` (`EAX`) point to two
/// vectors whose first three lanes are multiplied (the fourth lane is the float at
/// `01011d78`), `aabb` (`ESI`) is a box (minimum at +0, maximum at +0x10); then the one stack
/// argument `transform` (the caller removes it). The product goes through
/// `hkVector4::setTransformedPos` with `transform` and the box grows by the result.
pub fn fn_00d55600(e: &mut Engine, first: u32, second: u32, aabb: u32, transform: u32) {
    grow_by_scaled_corner(e, first, second, transform, aabb);
}

// Translated from 00d55680 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl). The box of a triangle subpart: `aabb` starts empty, then each of the
/// `subpart` `+0x14` triangles has its three vertices (indices at `+0x40` with record stride
/// `+0x44`, bytes when the type byte `+0x48` is 1, 16-bit when 2, else 32-bit; vertices at
/// `+0x18` with stride `+0x1c`) scaled by the vector `scale` (the third one as `scale *
/// vertex`), moved by the identity transform (`01268370`) and grown into the box.
pub fn fn_00d55680(e: &mut Engine, subpart: u32, scale: u32, aabb: u32) {
    empty_aabb(e, aabb);
    let mut triangle = 0i32;
    if 0 < e.mem.i32(subpart + 0x14) {
        loop {
            let stride = e.mem.u32(subpart + 0x1c);
            let vertices = e.mem.u32(subpart + 0x18);
            let record = e
                .mem
                .u32(subpart + 0x44)
                .wrapping_mul(triangle as u32)
                .wrapping_add(e.mem.u32(subpart + 0x40));
            let [first, second, third] = match e.mem.i8(subpart + 0x48) {
                1 => [
                    u32::from(e.mem.u8(record)),
                    u32::from(e.mem.u8(record + 1)),
                    u32::from(e.mem.u8(record + 2)),
                ],
                2 => [
                    u32::from(e.mem.u16(record)),
                    u32::from(e.mem.u16(record + 2)),
                    u32::from(e.mem.u16(record + 4)),
                ],
                _ => [
                    e.mem.u32(record),
                    e.mem.u32(record + 4),
                    e.mem.u32(record + 8),
                ],
            };
            let corner = |index: u32| index.wrapping_mul(stride).wrapping_add(vertices);
            grow_by_scaled_corner(e, corner(first), scale, IDENTITY_TRANSFORM, aabb);
            grow_by_scaled_corner(e, corner(second), scale, IDENTITY_TRANSFORM, aabb);
            grow_by_scaled_corner(e, scale, corner(third), IDENTITY_TRANSFORM, aabb);
            triangle += 1;
            if triangle >= e.mem.i32(subpart + 0x14) {
                break;
            }
        }
    }
}

// Translated from 00d558b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Register inputs first: `aabb` (`ESI`), `subpart` (`EDI`, a convex-shape
/// subpart). `aabb` starts empty and grows by the box of every child shape (list at `+0x14`,
/// count at `+0x18`) taken with the subpart's transform: the rotation built from the quaternion
/// at `+0x20` and the translation at `+0x30` (the child's slot `0x1c` with `(transform,
/// tolerance 0.0, out)`).
pub fn fn_00d558b0(e: &mut Engine, aabb: u32, subpart: u32) {
    empty_aabb(e, aabb);
    e.with_stack(0x60, |e, scratch| {
        let transform = scratch.addr();
        let child_box = transform + 0x40;
        for lane in 0..4 {
            let word = e.mem.u32(subpart + 0x30 + 4 * lane);
            e.mem.set_u32(transform + 0x30 + 4 * lane, word);
        }
        e.call(SET_TRANSFORM, &args![transform, subpart + 0x20]);
        let mut child = 0i32;
        if 0 < e.mem.i32(subpart + 0x18) {
            loop {
                let list = e.mem.u32(subpart + 0x14);
                let shape = e.mem.u32(list.wrapping_add((child as u32).wrapping_mul(4)));
                e.vcall(shape, 0x1c, &args![transform, 0u32, child_box]);
                let low = load_vec(e, child_box);
                let high = load_vec(e, child_box + 0x10);
                grow_aabb(e, aabb, low, high);
                child += 1;
                if child >= e.mem.i32(subpart + 0x18) {
                    break;
                }
            }
        }
    });
}

// Translated from 00d55950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::recalcAabbExtents` (Xbox PDB; `ECX` = the shape): starts an empty
/// box, grows it by the box of every triangle subpart (`00d55680` with the scaling at `+0x80`)
/// widened by the radius (`+0xe4`) in every lane and by the box of every convex-shape subpart
/// (`00d558b0`), then stores the center (`+0xa0`) and the half extents (`+0x90`).
pub fn recalc_aabb_extents(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let radius = [e.mem.f32(me + 0xe4); 4];
    let fill = e.mem.u32(EMPTY_BOX_FILL);
    let mut low = [f32::from_bits(fill); 4];
    let mut high: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(e.mem.u32(SIGN_MASK + 4 * lane as u32) ^ fill));
    if 0 < e.mem.i32(me + 0xb8) {
        let mut done = 0i32;
        let mut offset = 0u32;
        loop {
            let subpart = e.mem.u32(me + 0xb4).wrapping_add(offset);
            let (corner_low, corner_high) = e.with_stack(0x20, |e, out| {
                fn_00d55680(e, subpart, me + 0x80, out.addr());
                (load_vec(e, out.addr()), load_vec(e, out.addr() + 0x10))
            });
            let new_low = zip_with(corner_low, radius, |c, r| c - r);
            let new_high = zip_with(corner_high, radius, |c, r| c + r);
            low = zip_with(low, new_low, minps);
            high = zip_with(high, new_high, maxps);
            done += 1;
            offset = offset.wrapping_add(0x60);
            if done >= e.mem.i32(me + 0xb8) {
                break;
            }
        }
    }
    if 0 < e.mem.i32(me + 0xc4) {
        let mut done = 0i32;
        let mut offset = 0u32;
        loop {
            let subpart = e.mem.u32(me + 0xc0).wrapping_add(offset);
            let (box_low, box_high) = e.with_stack(0x20, |e, out| {
                fn_00d558b0(e, out.addr(), subpart);
                (load_vec(e, out.addr()), load_vec(e, out.addr() + 0x10))
            });
            low = zip_with(low, box_low, minps);
            high = zip_with(high, box_high, maxps);
            offset = offset.wrapping_add(0x40);
            done += 1;
            if done >= e.mem.i32(me + 0xc4) {
                break;
            }
        }
    }
    store_center_and_extents(e, me, low, high);
}

// Translated from 00d55a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (role unconfirmed). Asks every child shape of every convex-shape subpart (list
/// at `+0x14`, count at `+0x18` of the subpart; the subparts are at `+0xc0`, count `+0xc4`) for
/// its slot `0x18` with `(query, 0x100)` and checks the answer against a limit that depends on
/// the subpart's kind (`+0x3c & 0xc0ffffff`: 0 gives 0x200, 1 gives 0x1d0, anything else
/// 0x1a0): an answer below 0 or above the limit makes the result -1, otherwise the result is
/// 0xf0. The second stack word is not read.
pub fn fn_00d55a90(e: &mut Engine, this: Ptr, query: u32, _unused_1: u32) -> i32 {
    let me = this.addr();
    let mut done = 0i32;
    if 0 < e.mem.i32(me + 0xc4) {
        let mut offset = 0u32;
        loop {
            let subpart = e.mem.u32(me + 0xc0).wrapping_add(offset);
            let mut child = 0i32;
            if 0 < e.mem.i32(subpart + 0x18) {
                loop {
                    let list = e.mem.u32(subpart + 0x14);
                    let shape = e.mem.u32(list.wrapping_add((child as u32).wrapping_mul(4)));
                    let answer = e.vcall(shape, 0x18, &args![query, 0x100u32]).i32();
                    let limit = match e.mem.u32(subpart + 0x3c) & 0xc0ff_ffff {
                        0 => 0x200,
                        1 => 0x1d0,
                        _ => 0x1a0,
                    };
                    if !(0..=limit).contains(&answer) {
                        return -1;
                    }
                    child += 1;
                    if child >= e.mem.i32(subpart + 0x18) {
                        break;
                    }
                }
            }
            offset = offset.wrapping_add(0x40);
            done += 1;
            if done >= e.mem.i32(me + 0xc4) {
                break;
            }
        }
    }
    0xf0
}

// Translated from 00d56040 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an `getAabb`-like slot; role unconfirmed). Passes `(transform, this + 0x90,
/// this + 0xa0, tolerance, out)` to `hkAabbUtil::calcAabb_ov3`, the half extents and the center
/// of the shape; the tolerance goes through the x87 stack (a signalling NaN is quieted).
pub fn fn_00d56040(e: &mut Engine, this: Ptr, transform: u32, tolerance: f32, out: u32) {
    let me = this.addr();
    e.call(
        CALC_AABB,
        &args![
            transform,
            me + 0x90,
            me + 0xa0,
            x87_quiet(tolerance).to_bits(),
            out
        ],
    );
}

/// Gives every triangle subpart its first welding index (`+0x4c`: the running total of the
/// children of the subparts before it) and makes the welding array (`+0xcc`, 16-bit entries)
/// as large as the total: reserved through `hkArrayUtil::_reserve` when its capacity is
/// smaller, then its size is the total.
fn rebuild_welding_offsets(e: &mut Engine, me: u32) {
    let mut total = 0i32;
    if 0 < e.mem.i32(me + 0xb8) {
        let mut done = 0i32;
        let mut offset = 0u32;
        loop {
            let subpart = e.mem.u32(me + 0xb4).wrapping_add(offset);
            e.mem.set_i32(subpart + 0x4c, total);
            total = total.wrapping_add(e.mem.i32(subpart + 0x14));
            done += 1;
            offset = offset.wrapping_add(0x60);
            if done >= e.mem.i32(me + 0xb8) {
                break;
            }
        }
    }
    let array = me + 0xcc;
    let allocator = router_allocator(e);
    if ((e.mem.u32(array + 8) & 0x3fff_ffff) as i32) < total {
        e.call(ARRAY_UTIL_RESERVE, &args![allocator, array, total, 2u32]);
    }
    let allocator = router_allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < total {
        let doubled = capacity.wrapping_mul(2);
        let wanted = if total >= doubled { total } else { doubled };
        e.call(ARRAY_UTIL_RESERVE, &args![allocator, array, wanted, 2u32]);
    }
    e.mem.set_i32(array + 4, total);
}

// Translated from 00d56070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a welding-info setter by its behaviour; unconfirmed). When the welding array
/// (`+0xcc`, size `+0xd0`) is empty it is built first (the offsets of the subparts, see
/// `00d56200`). For a key whose top bit is clear the 16-bit `info` is stored at the entry
/// `welding offset of the key's triangle subpart + child index`.
pub fn fn_00d56070(e: &mut Engine, this: Ptr, key: u32, info: u16) {
    let me = this.addr();
    if e.mem.u32(me + 0xd0) == 0 {
        rebuild_welding_offsets(e, me);
    }
    if (key as i32) >= 0 {
        let bits = e.mem.u32(me + 0xb0);
        let (index, child) = split_key(bits, key);
        let subparts = e.mem.u32(me + 0xb4);
        let first = e
            .mem
            .u32(index.wrapping_mul(0x60).wrapping_add(subparts) + 0x4c);
        let entry = child.wrapping_add(first);
        let welding = e.mem.u32(me + 0xcc);
        e.mem
            .set_u16(welding.wrapping_add(entry.wrapping_mul(2)), info);
    }
}

// Translated from 00d56160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::_getNumChildShapesInTrianglesSubpart` (Xbox PDB): counts the
/// children of the triangle subpart `part` (index `index`, `+0x14` children) that are not
/// degenerate: each child shape is made with `getChildShape` of the container (key = the
/// index shifted above the child bits, or the child) and tested with `isDegenerate`.
pub fn get_num_child_shapes_in_triangles_subpart(
    e: &mut Engine,
    this: Ptr,
    part: u32,
    index: u32,
) -> i32 {
    let me = this.addr();
    let mut count = 0i32;
    let mut child = 0i32;
    if 0 < e.mem.i32(part + 0x14) {
        e.with_stack(0x210, |e, buffer| loop {
            let shift = 32u32.wrapping_sub(e.mem.u32(me + 0xb0));
            let key = (index << (shift & 31)) | child as u32;
            let shape = get_child_shape(e, Ptr::new(me + 0x10), key, buffer);
            if !is_degenerate(e, shape.addr(), buffer.addr() + 0x200) {
                count += 1;
            }
            child += 1;
            if child >= e.mem.i32(part + 0x14) {
                break;
            }
        });
    }
    count
}

// Translated from 00d56200 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a welding-type setter by its behaviour; unconfirmed). Stores the low byte of
/// `welding_type` at `+0xd8`. A value other than 6 rebuilds the welding array (offsets of the
/// subparts, then the array reserved and sized as `00d56070` does); 6 empties it (size 0, block
/// freed through the allocator's slot `0x10` when owned, capacity word `0x80000000`).
pub fn fn_00d56200(e: &mut Engine, this: Ptr, welding_type: u32) {
    let me = this.addr();
    e.mem.set_u8(me + 0xd8, welding_type as u8);
    if welding_type != 6 {
        rebuild_welding_offsets(e, me);
        return;
    }
    let allocator = router_allocator(e);
    let capacity = e.mem.u32(me + 0xd4);
    e.mem.set_u32(me + 0xd0, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 0xcc);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(2)],
        );
    }
    e.mem.set_u32(me + 0xcc, 0);
    e.mem.set_u32(me + 0xd4, 0x8000_0000);
}

/// Writes the default triangle subpart (byte `+1` and 16-bit `+0xa` set to 1, the rest zero)
/// at `at` (not a null address).
fn init_triangles_subpart(e: &mut Engine, at: u32) {
    e.mem.set_u16(at + 8, 0);
    e.mem.set_u16(at + 2, 0);
    e.mem.set_u8(at, 0);
    e.mem.set_u8(at + 1, 1);
    e.mem.set_u16(at + 0xa, 1);
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 4, 0);
    e.mem.set_u32(at + 0x10, 0);
    e.mem.set_u8(at + 0x49, 0);
    for lane in 0..4 {
        e.mem.set_u32(at + 0x30 + 4 * lane, 0);
    }
    e.mem.set_u32(at + 0x50, 0);
}

/// `hkArray::reserve` with the growth of `hkArray::pushBack`: when the capacity (the capacity
/// word without its flags) is smaller than `wanted`, reserves the larger of `wanted` and
/// twice the capacity entries of `element_size` bytes.
fn reserve_for(e: &mut Engine, allocator: u32, array: u32, wanted: i32, element_size: u32) {
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_mul(2);
        let grown = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_UTIL_RESERVE,
            &args![allocator, array, grown, element_size],
        );
    }
}

// Translated from 00d562f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::expandOneTriangleSubparts` (Xbox PDB, `ECX` = the shape): makes room
/// for one more triangle subpart and answers its address. The first one is the embedded
/// subpart (`+0x20`, array of size 1 with capacity word `0x80000001`) and is left as the
/// constructor made it; later ones are reserved (`hkArrayUtil::_reserve`, 0x60-byte entries)
/// and initialised to the default subpart.
pub fn expand_one_triangle_subparts(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    if e.mem.u32(me + 0xb8) == 0 {
        e.mem.set_u32(me + 0xb4, me + 0x20);
        e.mem.set_u32(me + 0xb8, 1);
        e.mem.set_u32(me + 0xbc, 0x8000_0001);
        return me + 0x20;
    }
    let array = me + 0xb4;
    let allocator = router_allocator(e);
    let size = e.mem.i32(array + 4);
    reserve_for(e, allocator, array, size.wrapping_add(1), 0x60);
    let at = e
        .mem
        .u32(array + 4)
        .wrapping_mul(0x60)
        .wrapping_add(e.mem.u32(array));
    if at != 0 {
        init_triangles_subpart(e, at);
    }
    let count = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, count.wrapping_add(1));
    (size as u32)
        .wrapping_mul(0x60)
        .wrapping_add(e.mem.u32(array))
}

// Translated from 00d563a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::addTrianglesSubpart` (Xbox PDB, `ECX` = the shape): makes room for a
/// subpart (`expandOneTriangleSubparts`), copy-constructs it from `subpart` (`00d4abf0`), grows
/// the shape's box (center `+0xa0`, half extents `+0x90`) by the subpart's box (`00d55680`
/// with the scaling at `+0x80`) widened by the radius (`+0xe4`), and adds the number of
/// non-degenerate children of the new subpart (`_getNumChildShapesInTrianglesSubpart`) to
/// the child count at `+0xe0`.
pub fn add_triangles_subpart(e: &mut Engine, this: Ptr, subpart: u32) {
    let me = this.addr();
    let added = expand_one_triangle_subparts(e, this);
    e.call(TRIANGLES_SUBPART_COPY, &args![added, subpart]);
    let center = load_vec(e, me + 0xa0);
    let extents = load_vec(e, me + 0x90);
    let old_low = zip_with(center, extents, |c, h| c - h);
    let old_high = zip_with(extents, center, |h, c| h + c);
    let (corner_low, corner_high) = e.with_stack(0x20, |e, out| {
        fn_00d55680(e, added, me + 0x80, out.addr());
        (load_vec(e, out.addr()), load_vec(e, out.addr() + 0x10))
    });
    let radius = [e.mem.f32(me + 0xe4); 4];
    let new_high = zip_with(corner_high, radius, |c, r| c + r);
    let new_low = zip_with(corner_low, radius, |c, r| c - r);
    let low = zip_with(old_low, new_low, minps);
    let high = zip_with(old_high, new_high, maxps);
    store_center_and_extents(e, me, low, high);
    let last = e.mem.u32(me + 0xb8).wrapping_sub(1);
    let children = get_num_child_shapes_in_triangles_subpart(e, this, subpart, last);
    let total = e.mem.u32(me + 0xe0);
    e.mem
        .set_u32(me + 0xe0, total.wrapping_add(children as u32));
}

// Translated from 00d56470 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a `hkpExtendedMeshShape::ShapesSubpart` method; role from the body). With
/// `flag` not zero stores the subpart's transform kind in the word at `+0x3c`: `0x3f000000`
/// (the float 0.5 keeps the lane a number) with bit 0 set when the translation (`+0x30`) is not
/// the reference within the tolerance and bit 1 set unless the quaternion's angle (`+0x20`,
/// `00c74c20`) is below the tolerance. Returns `this`.
pub fn fn_00d56470(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    if flag == 0 {
        return this;
    }
    let differs = u32::from(translation_differs(e, me + 0x30));
    let angle = e.call(QUATERNION_GET_ANGLE, &args![me + 0x20]).f32();
    let rotated = if angle < e.mem.f32(TOLERANCE) { 0 } else { 2 };
    e.mem.set_u32(me + 0x3c, rotated | differs | 0x3f00_0000);
    this
}

/// The head every convex-shape subpart constructor writes: the defaults of the subpart
/// (`+0` and `+1` bytes set to 1, 16-bit `+0xa` set to 1, the rest zero) and an empty
/// shape list (data `+0x14`, size `+0x18`, capacity word `+0x1c` = `0x80000000`).
fn init_shapes_subpart(e: &mut Engine, this: u32) {
    e.mem.set_u8(this, 1);
    e.mem.set_u8(this + 1, 1);
    e.mem.set_u16(this + 8, 0);
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 4, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u16(this + 2, 0);
    e.mem.set_u16(this + 0xa, 1);
    e.mem.set_u32(this + 0x14, 0);
    e.mem.set_u32(this + 0x18, 0);
    e.mem.set_u32(this + 0x1c, 0x8000_0000);
}

/// Appends `count` shape references to the subpart's shape list: the list is reserved when
/// needed (`hkArrayUtil::_reserve`, 4-byte entries), the new entries are zeroed, then each
/// shape of the pointer array `shapes` gets its reference count raised and replaces the entry
/// (releasing what was there).
fn append_child_shapes(e: &mut Engine, this: u32, shapes: u32, count: i32) {
    let array = this + 0x14;
    let allocator = router_allocator(e);
    let old_size = e.mem.i32(array + 4);
    reserve_for(e, allocator, array, old_size.wrapping_add(count), 4);
    let mut slot = e
        .mem
        .u32(array)
        .wrapping_add(e.mem.u32(array + 4).wrapping_mul(4));
    for _ in 0..count.max(0) {
        if slot != 0 {
            e.mem.set_u32(slot, 0);
        }
        slot = slot.wrapping_add(4);
    }
    let size = e.mem.i32(array + 4);
    e.mem.set_i32(array + 4, size.wrapping_add(count));
    let mut slot = e
        .mem
        .u32(array)
        .wrapping_add((old_size as u32).wrapping_mul(4));
    for index in 0..count.max(0) as u32 {
        let shape = e.mem.u32(shapes + 4 * index);
        if shape != 0 {
            e.call(REFERENCED_OBJECT_ADD, &args![shape]);
        }
        let old = e.mem.u32(slot);
        if old != 0 {
            e.call(REFERENCED_OBJECT_RELEASE, &args![old]);
        }
        e.mem.set_u32(slot, shape);
        slot = slot.wrapping_add(4);
    }
}

// Translated from 00d56500 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: a `hkpExtendedMeshShape::ShapesSubpart` constructor taking `(child shapes,
/// count, translation)`. Makes the default subpart with the translation vector in `+0x30`,
/// appends the child shapes, stores the identity quaternion (`010c71b0`) at `+0x20` and sets
/// `+0x3c` to `0x3f000000`, with bit 0 set when the translation is not the reference within
/// the tolerance. Returns `this`.
pub fn fn_00d56500(e: &mut Engine, this: Ptr, shapes: u32, count: i32, translation: u32) -> Ptr {
    let me = this.addr();
    init_shapes_subpart(e, me);
    for lane in 0..4 {
        let word = e.mem.u32(translation + 4 * lane);
        e.mem.set_u32(me + 0x30 + 4 * lane, word);
    }
    append_child_shapes(e, me, shapes, count);
    for lane in 0..4 {
        let word = e.mem.u32(IDENTITY_QUATERNION + 4 * lane);
        e.mem.set_u32(me + 0x20 + 4 * lane, word);
    }
    let differs = u32::from(translation_differs(e, translation));
    e.mem.set_u32(me + 0x3c, differs | 0x3f00_0000);
    this
}

// Translated from 00d56660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::ShapesSubpart::ShapesSubpart` (Xbox PDB): the constructor taking
/// `(child shapes, count, transform)`. Makes the default subpart, sets the quaternion at `+0x20`
/// from the transform's rotation (`hkQuaternion::set`), copies its translation (`+0x30`),
/// appends the child shapes and sets `+0x3c` to `0x3f000000` with bit 0 set when the
/// translation is not the reference within the tolerance and bit 1 set unless the rotation is
/// approximately the identity (`hkMatrix3::isApproximatelyEqual`). Returns `this`.
pub fn shapes_subpart_construct(
    e: &mut Engine,
    this: Ptr,
    shapes: u32,
    count: i32,
    transform: u32,
) -> Ptr {
    let me = this.addr();
    init_shapes_subpart(e, me);
    e.call(QUATERNION_SET, &args![me + 0x20, transform]);
    for lane in 0..4 {
        let word = e.mem.u32(transform + 0x30 + 4 * lane);
        e.mem.set_u32(me + 0x30 + 4 * lane, word);
    }
    append_child_shapes(e, me, shapes, count);
    let differs = u32::from(translation_differs(e, me + 0x30));
    let tolerance = x87_quiet_bits(e.mem.u32(TOLERANCE));
    let rotation_is_identity = e.with_stack(4, |e, result| {
        let answered = e
            .call(
                MATRIX_IS_APPROXIMATELY_EQUAL,
                &args![transform, result, IDENTITY_TRANSFORM, tolerance],
            )
            .u32();
        e.mem.u8(answered) != 0
    });
    let rotated = if rotation_is_identity { 0 } else { 2 };
    e.mem.set_u32(me + 0x3c, differs | rotated | 0x3f00_0000);
    this
}

// Translated from 00d56800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the destructor of a convex-shape subpart by its behaviour; `ECX` = the
/// subpart). Releases every shape of the list (`+0x14`, size `+0x18`) and clears the entries,
/// sets the size to 0, frees the list's block through the allocator's slot `0x10` when it owns
/// it (capacity word `+0x1c` with the top bit clear) and leaves the list empty (capacity word
/// `0x80000000`).
pub fn fn_00d56800(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let allocator = router_allocator(e);
    let size = e.mem.i32(me + 0x18);
    let data = e.mem.u32(me + 0x14);
    for index in 0..size.max(0) as u32 {
        let slot = data.wrapping_add(index.wrapping_mul(4));
        let shape = e.mem.u32(slot);
        if shape != 0 {
            e.call(REFERENCED_OBJECT_RELEASE, &args![shape]);
        }
        e.mem.set_u32(slot, 0);
    }
    let capacity = e.mem.u32(me + 0x1c);
    e.mem.set_u32(me + 0x18, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(me + 0x14);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(me + 0x1c, 0x8000_0000);
    e.mem.set_u32(me + 0x14, 0);
}

// Translated from 00d56880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`ECX` = the shape). Makes room for one more convex-shape subpart in the array
/// at `+0xc0` (size `+0xc4`, 0x40-byte entries, reserved through `hkArrayUtil::_reserve`),
/// initialises it as the default subpart and answers its address.
pub fn fn_00d56880(e: &mut Engine, this: Ptr) -> u32 {
    let array = this.addr() + 0xc0;
    let allocator = router_allocator(e);
    let size = e.mem.i32(array + 4);
    reserve_for(e, allocator, array, size.wrapping_add(1), 0x40);
    let at = (e.mem.u32(array + 4) << 6).wrapping_add(e.mem.u32(array));
    if at != 0 {
        init_shapes_subpart(e, at);
    }
    let count = e.mem.u32(array + 4);
    e.mem.set_u32(array + 4, count.wrapping_add(1));
    ((size as u32) << 6).wrapping_add(e.mem.u32(array))
}

// Translated from 00d56900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`addShapesSubpart` by its behaviour; unconfirmed). Makes room for a convex-shape
/// subpart (`00d56880`), copy-constructs it from `subpart` (`00d4ad60`), grows the shape's box
/// (center `+0xa0`, half extents `+0x90`) by the subpart's box (`00d558b0`), adds the subpart's
/// child count (`_getNumChildShapesInShapesSubpart`) to `+0xe0` and answers the index of the
/// new subpart (`+0xc4` minus 1).
pub fn fn_00d56900(e: &mut Engine, this: Ptr, subpart: u32) -> i32 {
    let me = this.addr();
    let added = fn_00d56880(e, this);
    e.call(SHAPES_SUBPART_COPY, &args![added, subpart]);
    let center = load_vec(e, me + 0xa0);
    let extents = load_vec(e, me + 0x90);
    let old_low = zip_with(center, extents, |c, h| c - h);
    let old_high = zip_with(extents, center, |h, c| h + c);
    let (box_low, box_high) = e.with_stack(0x20, |e, out| {
        fn_00d558b0(e, out.addr(), added);
        (load_vec(e, out.addr()), load_vec(e, out.addr() + 0x10))
    });
    let low = zip_with(old_low, box_low, minps);
    let high = zip_with(old_high, box_high, maxps);
    store_center_and_extents(e, me, low, high);
    let children = get_num_child_shapes_in_shapes_subpart(e, subpart);
    let total = e.mem.u32(me + 0xe0);
    e.mem.set_u32(me + 0xe0, total.wrapping_add(children));
    e.mem.i32(me + 0xc4).wrapping_sub(1)
}

/// What both `hkpExtendedMeshShape` constructors write after the base constructor: the vtables
/// (`+0` and the container at `+0x10`), the embedded triangle subpart (`+0x20`) in its default
/// state, the empty arrays (triangle subparts `+0xb4`, convex-shape subparts `+0xc0`, welding
/// `+0xcc`; capacity words `0x80000000`) and the 16 bytes of zero at `+0x50`.
fn init_extended_mesh_shape(e: &mut Engine, me: u32) {
    e.mem.set_u32(me, EXTENDED_MESH_SHAPE_VTABLE);
    e.mem.set_u32(me + 0x10, EXTENDED_MESH_CONTAINER_VTABLE);
    e.mem.set_u8(me + 0x20, 0);
    e.mem.set_u8(me + 0x21, 1);
    e.mem.set_u16(me + 0x28, 0);
    e.mem.set_u32(me + 0x2c, 0);
    e.mem.set_u32(me + 0x24, 0);
    e.mem.set_u32(me + 0x30, 0);
    e.mem.set_u8(me + 0x69, 0);
    e.mem.set_u32(me + 0x70, 0);
    e.mem.set_u16(me + 0x22, 0);
    for lane in 0..4 {
        e.mem.set_u32(me + 0x50 + 4 * lane, 0);
    }
    e.mem.set_u16(me + 0x2a, 1);
    for array in [0xb4, 0xc0, 0xcc] {
        e.mem.set_u32(me + array, 0);
        e.mem.set_u32(me + array + 4, 0);
        e.mem.set_u32(me + array + 8, 0x8000_0000);
    }
}

// Translated from 00d569c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::hkpExtendedMeshShape` (Xbox PDB): runs `hkpShapeCollection`'s
/// constructor with `(0xe, 1)`, installs the vtables and the empty state, stores the radius
/// (`+0xe4`, through the x87 stack), the number of key bits for the subpart index (`+0xb0`), the
/// scaling (`+0x80`, the float at `01012068` in every lane), the half extents (`+0x90`, the
/// float at `0104e148` in every lane), a zero center (`+0xa0`), the welding type 6 (`+0xd8`)
/// and zero children (`+0xdc`, `+0xe0`). Returns `this`.
pub fn hkp_extended_mesh_shape_construct(
    e: &mut Engine,
    this: Ptr,
    radius: f32,
    key_bits: u32,
) -> Ptr {
    let me = this.addr();
    e.call(
        SHAPE_COLLECTION_CONSTRUCT,
        &args![me, EXTENDED_MESH_SHAPE_TYPE, 1u32],
    );
    init_extended_mesh_shape(e, me);
    e.mem.set_f32(me + 0xe4, x87_quiet(radius));
    let scaling = e.mem.u32(INITIAL_SCALING);
    for lane in 0..4 {
        e.mem.set_u32(me + 0x80 + 4 * lane, scaling);
    }
    let half_extents = e.mem.u32(INITIAL_HALF_EXTENTS);
    e.mem.set_u32(me + 0xdc, 0);
    e.mem.set_u32(me + 0xe0, 0);
    e.mem.set_u32(me + 0xb0, key_bits);
    e.mem.set_u8(me + 0xd8, 6);
    for lane in 0..4 {
        e.mem.set_u32(me + 0x90 + 4 * lane, half_extents);
    }
    for lane in 0..4 {
        e.mem.set_u32(me + 0xa0 + 4 * lane, 0);
    }
    this
}

/// Copies the fields of one 0x38-byte source subpart (`origin`) to a 0x60-byte triangle
/// subpart (`target`) of the shape being copy-built.
fn copy_source_subpart(e: &mut Engine, origin: u32, target: u32) {
    for (from, to) in [
        (0x1c, 0x14),
        (0x20, 0x04),
        (0x28, 0x0c),
        (0x00, 0x18),
        (0x04, 0x1c),
        (0x08, 0x20),
        (0x0c, 0x40),
        (0x14, 0x44),
        (0x34, 0x4c),
    ] {
        let word = e.mem.u32(origin + from);
        e.mem.set_u32(target + to, word);
    }
    let half = e.mem.u16(origin + 0x24);
    e.mem.set_u16(target + 8, half);
    let byte = e.mem.u8(origin + 0x11);
    e.mem.set_u8(target + 1, byte);
    let half = e.mem.u16(origin + 0x2c);
    e.mem.set_u16(target + 2, half);
    let half = e.mem.u16(origin + 0x30);
    e.mem.set_u16(target + 0xa, half);
    let byte = e.mem.u8(origin + 0x10);
    e.mem.set_u8(target + 0x48, byte);
    for lane in 0..4 {
        e.mem.set_u32(target + 0x30 + 4 * lane, 0);
    }
    let byte = e.mem.u8(origin + 0x18);
    e.mem.set_u8(target + 0x49, byte);
}

// Translated from 00d56ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::hkpExtendedMeshShape_ov2` (Xbox PDB): builds the shape from another
/// shape's data `source` (class unconfirmed; its subparts are 0x38 bytes each). Runs
/// `hkpShapeCollection`'s constructor with `(0xe, 1)`, installs the vtables and the empty
/// state, copies the scaling (`source + 0x20` to `+0x80`), the radius (`source + 0x50` to
/// `+0xe4`, through the x87 stack), the key bits (`source + 0x30` to `+0xb0`) and the welding
/// type (`source + 0x4c` byte to `+0xd8`), copies the welding array (`source + 0x40`, size
/// `+0x44`; a block is allocated through the allocator's slot `0xc`, which sees the byte size
/// and may enlarge it) and the triangle subparts (`source + 0x34`, count `+0x38`; one subpart
/// is the embedded one at `+0x20`), then recalculates the box (`recalcAabbExtents`) and stores
/// the child count of the container (`getNumChildShapes`) at `+0xe0`. Returns `this`.
pub fn hkp_extended_mesh_shape_construct_ov2(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let me = this.addr();
    e.call(
        SHAPE_COLLECTION_CONSTRUCT,
        &args![me, EXTENDED_MESH_SHAPE_TYPE, 1u32],
    );
    init_extended_mesh_shape(e, me);
    let triangles = me + 0xb4;
    for lane in 0..4 {
        let word = e.mem.u32(source + 0x20 + 4 * lane);
        e.mem.set_u32(me + 0x80 + 4 * lane, word);
    }
    let radius = x87_quiet(e.mem.f32(source + 0x50));
    e.mem.set_f32(me + 0xe4, radius);
    let key_bits = e.mem.u32(source + 0x30);
    e.mem.set_u32(me + 0xb0, key_bits);
    let welding_type = e.mem.u8(source + 0x4c);
    e.mem.set_u8(me + 0xd8, welding_type);
    let allocator = router_allocator(e);
    let capacity_word = e.mem.u32(me + 0xd4);
    let capacity = (capacity_word & 0x3fff_ffff) as i32;
    let welding_count = e.mem.i32(source + 0x44);
    if capacity < welding_count {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(me + 0xcc);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(2)],
            );
        }
        let (block, granted) = e.with_stack(4, |e, size| {
            e.mem
                .set_u32(size.addr(), (welding_count as u32).wrapping_mul(2));
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        e.mem.set_u32(me + 0xcc, block);
        e.mem.set_i32(me + 0xd4, granted / 2);
    }
    e.mem.set_i32(me + 0xd0, welding_count);
    let destination = e.mem.u32(me + 0xcc);
    let origin = e.mem.u32(source + 0x40);
    for index in 0..welding_count.max(0) as u32 {
        let word = e.mem.u16(origin.wrapping_add(2 * index));
        e.mem.set_u16(destination.wrapping_add(2 * index), word);
    }
    let subparts = e.mem.i32(source + 0x38);
    if subparts == 1 {
        e.mem.set_u32(triangles, me + 0x20);
        e.mem.set_u32(triangles + 4, 1);
        e.mem.set_u32(triangles + 8, 0x8000_0001);
    } else {
        let allocator = router_allocator(e);
        reserve_for(e, allocator, triangles, subparts, 0x60);
        let size = e.mem.u32(triangles + 4);
        let mut at = size.wrapping_mul(0x60).wrapping_add(e.mem.u32(triangles));
        for _ in 0..subparts.wrapping_sub(size as i32).max(0) {
            if at != 0 {
                init_triangles_subpart(e, at);
            }
            at = at.wrapping_add(0x60);
        }
        e.mem.set_i32(triangles + 4, subparts);
    }
    e.mem.set_u32(me + 0xdc, 0);
    for index in 0..e.mem.i32(source + 0x38).max(0) as u32 {
        let origin = e.mem.u32(source + 0x34).wrapping_add(index * 0x38);
        let target = e.mem.u32(triangles).wrapping_add(index * 0x60);
        copy_source_subpart(e, origin, target);
    }
    recalc_aabb_extents(e, this);
    let children = e
        .call(CONTAINER_GET_NUM_CHILD_SHAPES, &args![me + 0x10])
        .u32();
    e.mem.set_u32(me + 0xe0, children);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_5530,
            get_mesh_material(Ptr<HkpExtendedMeshShape>, u32) -> u32
        ),
        entry!(0x00d5_55d0, fn_00d555d0(Ptr, u32) -> u32),
        entry!(0x00d5_5b40, fn_00d55b40(Ptr) -> u32),
        entry!(0x00d5_5bd0, fn_00d55bd0(Ptr, u32) -> u32),
        entry!(0x00d5_5ce0, get_child_shape(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00d5_7090, fn_00d57090(u32, u32)),
        entry!(0x00d5_70d0, fn_00d570d0(u32, u32)),
        entry!(
            0x00d5_5520,
            get_num_child_shapes_in_shapes_subpart(u32) -> u32
        ),
        entry!(0x00d5_5600, fn_00d55600(u32, u32, u32, u32)),
        entry!(0x00d5_5680, fn_00d55680(u32, u32, u32)),
        entry!(0x00d5_58b0, fn_00d558b0(u32, u32)),
        entry!(0x00d5_5950, recalc_aabb_extents(Ptr)),
        entry!(0x00d5_5a90, fn_00d55a90(Ptr, u32, u32) -> i32),
        entry!(0x00d5_6040, fn_00d56040(Ptr, u32, f32, u32)),
        entry!(0x00d5_6070, fn_00d56070(Ptr, u32, u16)),
        entry!(
            0x00d5_6160,
            get_num_child_shapes_in_triangles_subpart(Ptr, u32, u32) -> i32
        ),
        entry!(0x00d5_6200, fn_00d56200(Ptr, u32)),
        entry!(0x00d5_62f0, expand_one_triangle_subparts(Ptr) -> u32),
        entry!(0x00d5_63a0, add_triangles_subpart(Ptr, u32)),
        entry!(0x00d5_6470, fn_00d56470(Ptr, u32) -> Ptr),
        entry!(0x00d5_6500, fn_00d56500(Ptr, u32, i32, u32) -> Ptr),
        entry!(
            0x00d5_6660,
            shapes_subpart_construct(Ptr, u32, i32, u32) -> Ptr
        ),
        entry!(0x00d5_6800, fn_00d56800(Ptr)),
        entry!(0x00d5_6880, fn_00d56880(Ptr) -> u32),
        entry!(0x00d5_6900, fn_00d56900(Ptr, u32) -> i32),
        entry!(
            0x00d5_69c0,
            hkp_extended_mesh_shape_construct(Ptr, f32, u32) -> Ptr
        ),
        entry!(
            0x00d5_6ab0,
            hkp_extended_mesh_shape_construct_ov2(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_DOUBLE: u32 = 0x0900_0400;
    const NEXT_DOUBLE: u32 = 0x0900_0410;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(TRIANGLE_RADIUS, 0x3d4c_cccd);
        e.mem.set_u32(DEGENERACY_TOLERANCE, 0x33d6_bf95);
        for (lane, word) in [!0u32, !0, !0, 0].into_iter().enumerate() {
            e.mem.set_u32(LANE_MASK + 4 * lane as u32, word);
        }
        e
    }

    fn put(e: &mut Engine, address: u32, words: &[u32]) {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(address + 4 * i as u32, *word);
        }
    }

    /// A shape with 8 key bits for the subpart index, scaling (1, 2, 3, 1) and the given
    /// triangle subpart count; the container is at `+0x10`.
    fn new_shape(e: &mut Engine, triangle_subparts: u32) -> u32 {
        let shape = e.mem.alloc(0x100);
        e.mem.set_u32(shape + 0xb0, 8);
        e.mem.set_u32(shape + 0xb8, triangle_subparts);
        put(
            e,
            shape + 0x80,
            &[0x3f80_0000, 0x4000_0000, 0x4040_0000, 0x3f80_0000],
        );
        e.mem.set_u8(shape + 0xd8, 3);
        e.mem.set_u32(shape + 0xdc, 0xf1f1);
        e.mem.set_u32(shape + 0xe4, 0x3dcc_cccd);
        shape
    }

    /// Fills the triangle subpart at `subpart`.
    fn triangle_subpart(e: &mut Engine, subpart: u32, vertices: u32, indices: (u32, u32, u8, u8)) {
        let (base, stride, index_type, mask) = indices;
        e.mem.set_u32(subpart + 0x18, vertices);
        e.mem.set_u32(subpart + 0x1c, 12);
        e.mem.set_u32(subpart + 0x40, base);
        e.mem.set_u32(subpart + 0x44, stride);
        e.mem.set_u8(subpart + 0x48, index_type);
        e.mem.set_u8(subpart + 0x49, mask);
    }

    /// Eight vertices (i, 10 + i, 20 + i).
    fn vertices(e: &mut Engine) -> u32 {
        let vertices = e.mem.alloc(0x60);
        for i in 0..8u32 {
            for lane in 0..3 {
                e.mem
                    .set_f32(vertices + 12 * i + 4 * lane, (i + 10 * lane) as f32);
            }
        }
        vertices
    }

    fn vertex(e: &Engine, out: u32, slot: u32) -> [f32; 3] {
        std::array::from_fn(|lane| e.mem.f32(out + 0x20 + 0x10 * slot + 4 * lane as u32))
    }

    #[test]
    fn copy_functions_copy_the_subpart_words() {
        let mut e = engine();
        let source = e.mem.alloc(0x60);
        for i in 0..24 {
            e.mem.set_u32(source + 4 * i, 0x1000 + i);
        }
        let dest = e.mem.alloc(0x70);
        e.call(0x00d5_7090, &args![dest, source]);
        assert_eq!(e.mem.bytes(dest, 0x60), e.mem.bytes(source, 0x60));
        assert_eq!(e.mem.u32(dest + 0x60), 0);
        let dest = e.mem.alloc(0x50);
        e.call(0x00d5_70d0, &args![dest, source]);
        assert_eq!(e.mem.bytes(dest, 0x40), e.mem.bytes(source, 0x40));
        assert_eq!(e.mem.u32(dest + 0x40), 0);
    }

    #[test]
    fn get_mesh_material_reads_the_subpart_material_table() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        // The embedded subpart: byte materials, record stride 1, material stride 4.
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x0303_0201]);
        e.mem.set_u8(shape + 0x21, 1);
        e.mem.set_u16(shape + 0x22, 4);
        e.mem.set_u32(shape + 0x24, table);
        e.mem.set_u16(shape + 0x28, 1);
        e.mem.set_u32(shape + 0x2c, 0x1000);
        // Child 2 holds the material 3: 3 * 4 + 0x1000.
        assert_eq!(e.call(0x00d5_5530, &args![shape, 2u32]).u32(), 0x100c);
        // 16-bit materials with a negative stride.
        e.mem.set_u8(shape + 0x21, 2);
        e.mem.set_u16(shape + 0x22, 0xfffe);
        e.mem.set_u16(shape + 0x28, 2);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 1u32]).u32(),
            0x1000u32.wrapping_add(0x0303u32.wrapping_mul(0xffff_fffe))
        );
        // No material table: none.
        e.mem.set_u32(shape + 0x24, 0);
        assert_eq!(e.call(0x00d5_5530, &args![shape, 1u32]).u32(), 0);
    }

    #[test]
    fn get_mesh_material_picks_the_subpart_by_the_key() {
        let mut e = engine();
        let shape = new_shape(&mut e, 2);
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x0000_0007]);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        // Triangle subpart 1.
        e.mem.set_u8(subparts + 0x60 + 1, 1);
        e.mem.set_u16(subparts + 0x60 + 2, 2);
        e.mem.set_u32(subparts + 0x60 + 4, table);
        e.mem.set_u16(subparts + 0x60 + 8, 1);
        e.mem.set_u32(subparts + 0x60 + 0xc, 0x2000);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 0x0100_0000u32]).u32(),
            0x200e
        );
        // A subpart index past the array has no material.
        assert_eq!(e.call(0x00d5_5530, &args![shape, 0x0200_0000u32]).u32(), 0);
        // Convex-shape subparts are indexed by the key without a bounds check.
        let convex = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, convex);
        e.mem.set_u8(convex + 0x40 + 1, 1);
        e.mem.set_u16(convex + 0x40 + 2, 1);
        e.mem.set_u32(convex + 0x40 + 4, table);
        e.mem.set_u16(convex + 0x40 + 8, 1);
        e.mem.set_u32(convex + 0x40 + 0xc, 0x10);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 0x8100_0000u32]).u32(),
            0x17
        );
    }

    #[test]
    fn collision_filter_info_falls_back_to_the_default() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x77]);
        e.mem.set_u8(shape + 0x21, 1);
        e.mem.set_u32(shape + 0x24, 0);
        e.mem.set_u16(shape + 0x28, 1);
        assert_eq!(
            e.call(0x00d5_55d0, &args![shape + 0x10, 0u32]).u32(),
            0xf1f1
        );
        // With a material table the answer is the material's first word.
        e.mem.set_u32(shape + 0x24, table);
        e.mem.set_u32(shape + 0x2c, table);
        assert_eq!(e.call(0x00d5_55d0, &args![shape + 0x10, 0u32]).u32(), 0x77);
    }

    #[test]
    fn get_child_shape_builds_a_scaled_triangle_from_an_embedded_list_subpart() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        let vertices = vertices(&mut e);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0102_0100, 0x0000_0302]);
        // Byte index records of three with the list mask 0: the bytes
        // 0 1 2 | 1 2 3 make record 1 the triangle (1, 2, 3).
        triangle_subpart(&mut e, shape + 0x20, vertices, (indices, 3, 1, 0));
        put(&mut e, shape + 0x50, &[0x3f80_0000, 0, 0, 0x7777_7777]);
        let out = e.mem.alloc(0x60);
        let got = e.call(0x00d5_5ce0, &args![shape + 0x10, 1u32, out]).u32();
        assert_eq!(got, out);
        assert_eq!(vertex(&e, out, 0), [1.0, 22.0, 63.0]);
        assert_eq!(vertex(&e, out, 1), [2.0, 24.0, 66.0]);
        assert_eq!(vertex(&e, out, 2), [3.0, 26.0, 69.0]);
        assert_eq!(e.mem.u16(out + 6), 1);
        assert_eq!(e.mem.u32(out), TRIANGLE_VTABLE);
        assert_eq!(e.mem.u32(out + 0xc), 3);
        assert_eq!(e.mem.u32(out + 0x10), 0x3dcc_cccd);
        assert_eq!(e.mem.u8(out + 0x16), 3);
        assert_eq!(e.mem.u8(out + 0x17), 1);
        assert_eq!(e.mem.u16(out + 0x14), 0);
        assert_eq!(e.mem.bytes(out + 0x50, 16), e.mem.bytes(shape + 0x50, 16));
    }

    #[test]
    fn get_child_shape_reads_strips_welding_info_and_wide_indices() {
        let mut e = engine();
        let shape = new_shape(&mut e, 2);
        let vertices = vertices(&mut e);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        // Subpart 1: 16-bit strip, one index per record; triangle 3 is (3, 1, 0).
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0001_0000, 0x0003_0002, 0x0001_0000]);
        triangle_subpart(&mut e, subparts + 0x60, vertices, (indices, 2, 2, 1));
        e.mem.set_u32(subparts + 0x60 + 0x4c, 2);
        let welding = e.mem.alloc(0x20);
        put(&mut e, welding, &[0, 0, 0x1234_0000, 0]);
        e.mem.set_u32(shape + 0xcc, welding);
        e.mem.set_u32(shape + 0xd0, 8);
        let out = e.mem.alloc(0x60);
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0x0100_0003u32, out]);
        assert_eq!(vertex(&e, out, 0), [3.0, 26.0, 69.0]);
        assert_eq!(vertex(&e, out, 1), [1.0, 22.0, 63.0]);
        assert_eq!(vertex(&e, out, 2), [0.0, 20.0, 60.0]);
        // Welding entry (2 + 3) of the shape.
        assert_eq!(e.mem.u16(out + 0x14), 0x1234);
        assert_eq!(e.mem.u8(out + 0x17), 0);
        // 32-bit indices in an embedded subpart.
        let shape = new_shape(&mut e, 1);
        let wide = e.mem.alloc(0x10);
        put(&mut e, wide, &[5, 6, 7]);
        triangle_subpart(&mut e, shape + 0x20, vertices, (wide, 12, 3, 0));
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0u32, out]);
        assert_eq!(vertex(&e, out, 0), [5.0, 30.0, 75.0]);
        assert_eq!(vertex(&e, out, 2), [7.0, 34.0, 81.0]);
        // An unknown index type reads vertex 0 three times.
        e.mem.set_u8(shape + 0x68, 9);
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0u32, out]);
        assert_eq!(vertex(&e, out, 1), [0.0, 20.0, 60.0]);
    }

    #[test]
    fn get_child_shape_returns_convex_shapes_plain_or_wrapped() {
        let mut e = engine();
        let shape = new_shape(&mut e, 0);
        let subparts = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, subparts);
        e.mem.set_u32(shape + 0xc4, 2);
        let list = e.mem.alloc(0x10);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(list + 4, child);
        e.mem.set_f32(child + 0x10, 1.5);
        // Subpart 1, kind 0: the stored shape itself.
        let subpart = subparts + 0x40;
        e.mem.set_u32(subpart + 0x14, list);
        put(
            &mut e,
            subpart + 0x30,
            &[0x3f80_0000, 0xc000_0000, 0x4040_0000, 0],
        );
        let out = e.mem.alloc(0x60);
        let key = 0x8100_0001u32;
        assert_eq!(
            e.call(0x00d5_5ce0, &args![shape + 0x10, key, out]).u32(),
            child
        );
        // Kind 1: a convex transform shape with a translation (lane 3 cleared).
        e.mem.set_u32(subpart + 0x3c, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CONVEX_TRANSFORM_BASE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let got = e.call(0x00d5_5ce0, &args![shape + 0x10, key, out]).u32();
        assert_eq!(got, out);
        assert_eq!(*seen.borrow(), vec![vec![out, 0xb, 0x3fc0_0000, child, 0]]);
        assert_eq!(e.mem.u32(out), CONVEX_TRANSFORM_VTABLE);
        assert_eq!(e.mem.u32(out + 0x1c), 0);
        assert_eq!(e.mem.bytes(out + 0x20, 12), e.mem.bytes(subpart + 0x30, 12));
        assert_eq!(e.mem.u32(out + 0x2c), 0);
        // A null output has nothing to build in.
        assert_eq!(
            e.call(0x00d5_5ce0, &args![shape + 0x10, key, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn get_child_shape_builds_a_transformed_convex_shape_for_other_kinds() {
        let mut e = engine();
        let shape = new_shape(&mut e, 0);
        let subparts = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, subparts);
        let list = e.mem.alloc(0x10);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(list, child);
        e.mem.set_u32(subparts + 0x14, list);
        e.mem.set_u32(subparts + 0x3c, 0x4000_0002);
        put(&mut e, subparts + 0x30, &[0x11, 0x22, 0x33, 0x4000_0002]);
        let transforms = Rc::new(RefCell::new(Vec::new()));
        let transform_log = Rc::clone(&transforms);
        e.register_double(SET_TRANSFORM, move |_, a| {
            transform_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let built = Rc::new(RefCell::new(Vec::new()));
        let built_log = Rc::clone(&built);
        e.register_double(CONVEX_TRANSFORM, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|i| e.mem.u32(a[2] + 0x30 + 4 * i)));
            built_log.borrow_mut().push(words);
            Ret {
                eax: 0x5151,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(0x60);
        let got = e
            .call(0x00d5_5ce0, &args![shape + 0x10, 0x8000_0000u32, out])
            .u32();
        assert_eq!(got, 0x5151);
        {
            let transforms = transforms.borrow();
            assert_eq!(transforms.len(), 1);
            let built = built.borrow();
            assert_eq!(built[0][0..2], [out, child]);
            assert_eq!(built[0][2], transforms[0][0]);
            assert_eq!(built[0][3], 0);
            assert_eq!(built[0][4..8], [0x11, 0x22, 0x33, 0x4000_0002]);
        }
        // Without an output there is nothing to construct.
        let got = e
            .call(0x00d5_5ce0, &args![shape + 0x10, 0x8000_0000u32, 0u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(built.borrow().len(), 1);
    }

    /// A container whose slot `+0x14` hands out triangle (or other) shapes and whose slot
    /// `+0xC` answers `0x4242`; `isDegenerate` answers from `degenerate`.
    fn container(
        e: &mut Engine,
        shape_type: u32,
        degenerate: Vec<bool>,
    ) -> (u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let shape = new_shape(e, 0);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, CHILD_DOUBLE);
        e.mem.set_u32(vtable + 0xc, NEXT_DOUBLE);
        e.mem.set_u32(shape + 0x10, vtable);
        let triangle = e.mem.alloc(0x60);
        e.mem.set_u32(triangle + 0xc, shape_type);
        let asked = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        e.register_double(CHILD_DOUBLE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: triangle,
                ..Ret::default()
            }
        });
        e.register_double(NEXT_DOUBLE, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        let flag = e.mem.alloc(4);
        let answers = RefCell::new(degenerate.into_iter());
        e.register_double(IS_DEGENERATE, move |e, a| {
            let answer = answers
                .borrow_mut()
                .next()
                .expect("unexpected isDegenerate");
            e.mem.set_u8(flag, u8::from(answer));
            assert_eq!(a[1..4], [triangle + 0x20, triangle + 0x30, triangle + 0x40]);
            assert_eq!(a[4], 0x33d6_bf95);
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        (shape + 0x10, asked)
    }

    #[test]
    fn first_key_is_the_first_child_unless_it_is_degenerate() {
        let mut e = engine();
        let (this, asked) = container(&mut e, 3, vec![false, true]);
        e.mem.set_u32(this - 0x10 + 0xb8, 1);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0);
        assert_eq!(asked.borrow()[0][..2], [this, 0]);
        // A degenerate first triangle defers to the next-key slot.
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0x4242);
        // With only convex subparts the first key is the first convex shape.
        let (this, asked) = container(&mut e, 1, vec![]);
        e.mem.set_u32(this - 0x10 + 0xc4, 3);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0x8000_0000);
        assert_eq!(asked.borrow()[0][1], 0x8000_0000);
        // No subparts at all: no key.
        e.mem.set_u32(this - 0x10 + 0xc4, 0);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0xffff_ffff);
    }

    #[test]
    fn next_key_walks_children_subparts_then_convex_shapes() {
        let mut e = engine();
        let (this, asked) = container(&mut e, 3, vec![false, false, true, false, false, false]);
        let shape = this - 0x10;
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        e.mem.set_u32(shape + 0xb8, 2);
        e.mem.set_u32(subparts + 0x14, 2);
        e.mem.set_u32(subparts + 0x60 + 0x14, 1);
        let convex = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, convex);
        e.mem.set_u32(convex + 0x18, 2);
        // Within a subpart.
        assert_eq!(e.call(0x00d5_5bd0, &args![this, 0u32]).u32(), 1);
        // Past its last child: the next subpart's first child.
        assert_eq!(e.call(0x00d5_5bd0, &args![this, 1u32]).u32(), 0x0100_0000);
        // Past the last triangle subpart there are no convex shapes yet: none.
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x0100_0000u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(asked.borrow().len(), 2);
        // A degenerate candidate (the third answer) is skipped.
        e.mem.set_u32(shape + 0xc4, 1);
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x0100_0000u32]).u32(),
            0x8000_0001
        );
        // The convex shapes are keyed by the high bit.
        e.mem.set_u32(shape + 0xc4, 2);
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x8000_0001u32]).u32(),
            0x8100_0000
        );
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x8100_0001u32]).u32(),
            0xffff_ffff
        );
    }

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SIZED_ALLOC_DOUBLE: u32 = 0x0900_0500;
    const FREE_DOUBLE: u32 = 0x0900_0510;
    const BOX_DOUBLE: u32 = 0x0900_0520;
    const ANSWER_DOUBLE: u32 = 0x0900_0530;

    fn new_log() -> CallLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// Registers a double at `address` that records its argument words and answers `result`.
    fn record(e: &mut Engine, log: &CallLog, address: u32, result: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with the float constants of the box code in memory.
    fn box_engine() -> Engine {
        let mut e = engine();
        e.mem.set_f32(EMPTY_BOX_FILL, 1.0e30);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_f32(HALF_FACTORS + 4 * lane, 0.5);
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.mem.set_f32(TOLERANCE, 0.001);
        store_vec(&mut e, IDENTITY_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        e.mem.set_f32(INITIAL_SCALING, 1.0);
        e.mem.set_f32(INITIAL_HALF_EXTENTS, 0.25);
        e
    }

    /// `setTransformedPos` double: records `[transform, vector bits]` and answers
    /// `vector + the translation at transform + 0x30`.
    fn move_double(e: &mut Engine, log: &CallLog) {
        let log = Rc::clone(log);
        e.register_double(SET_TRANSFORMED_POS, move |e, a| {
            let (out, transform, vector) = (a[0], a[1], a[2]);
            let mut words = vec![transform];
            words.extend((0..4).map(|lane| e.mem.u32(vector + 4 * lane)));
            log.borrow_mut().push((SET_TRANSFORMED_POS, words));
            for lane in 0..4 {
                let moved = e.mem.f32(vector + 4 * lane) + e.mem.f32(transform + 0x30 + 4 * lane);
                e.mem.set_f32(out + 4 * lane, moved);
            }
            Ret::default()
        });
    }

    /// `hkArrayUtil::_reserve` double: a new zeroed block of `capacity` entries holding the old
    /// entries; the capacity word becomes the capacity.
    fn reserve_double(e: &mut Engine, log: &CallLog) {
        let log = Rc::clone(log);
        e.register_double(ARRAY_UTIL_RESERVE, move |e, a| {
            log.borrow_mut().push((ARRAY_UTIL_RESERVE, a.to_vec()));
            let (array, capacity, element) = (a[1], a[2], a[3]);
            let old = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let block = e.mem.alloc(capacity * element);
            for byte in 0..size * element {
                let value = e.mem.u8(old + byte);
                e.mem.set_u8(block + byte, value);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
    }

    /// The memory router: its allocator's slot `0xc` hands out the requested size plus 4 and
    /// logs the request, slot `0x10` logs `[allocator, block, bytes]`. Answers the allocator.
    fn router(e: &mut Engine, log: &CallLog) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SIZED_ALLOC_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(log);
        e.register_double(SIZED_ALLOC_DOUBLE, move |e, a| {
            let wanted = e.mem.u32(a[1]);
            sink.borrow_mut().push((SIZED_ALLOC_DOUBLE, vec![wanted]));
            e.mem.set_u32(a[1], wanted + 4);
            Ret {
                eax: e.mem.alloc(wanted + 4),
                ..Ret::default()
            }
        });
        record(e, log, FREE_DOUBLE, 0);
        allocator
    }

    /// A shape object whose slot `0x1c` (the box) writes `low` and `high` to its output.
    fn box_child(e: &mut Engine, low: Vec4, high: Vec4) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, BOX_DOUBLE);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(child, vtable);
        store_vec(e, child + 0x10, low);
        store_vec(e, vtable + 0x20, high);
        child
    }

    /// Registers the box double of `box_child` shapes: it copies the shape's `+0x10` vector
    /// to the minimum and the vector after the vtable's slots to the maximum.
    fn box_double(e: &mut Engine, log: &CallLog) {
        let log = Rc::clone(log);
        e.register_double(BOX_DOUBLE, move |e, a| {
            log.borrow_mut().push((BOX_DOUBLE, a.to_vec()));
            let vtable = e.mem.u32(a[0]);
            for lane in 0..4 {
                let low = e.mem.u32(a[0] + 0x10 + 4 * lane);
                e.mem.set_u32(a[3] + 4 * lane, low);
                let high = e.mem.u32(vtable + 0x20 + 4 * lane);
                e.mem.set_u32(a[3] + 0x10 + 4 * lane, high);
            }
            Ret::default()
        });
    }

    /// A list of `children` shapes in a fresh block.
    fn shape_list(e: &mut Engine, children: &[u32]) -> u32 {
        let list = e.mem.alloc(4 * children.len() as u32 + 4);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *child);
        }
        list
    }

    /// Four vertices (1, 2, 3), (-1, 5, 0), (4, -2, 7), (0, 0, -9), 12 bytes apart.
    fn box_vertices(e: &mut Engine) -> u32 {
        let vertices = e.mem.alloc(0x30);
        for (index, vertex) in [
            [1.0, 2.0, 3.0],
            [-1.0, 5.0, 0.0],
            [4.0, -2.0, 7.0],
            [0.0, 0.0, -9.0],
        ]
        .into_iter()
        .enumerate()
        {
            for (lane, value) in vertex.into_iter().enumerate() {
                e.mem
                    .set_f32(vertices + 12 * index as u32 + 4 * lane as u32, value);
            }
        }
        vertices
    }

    /// Writes the `width`-byte unsigned `value` at `address`.
    fn put_index(e: &mut Engine, address: u32, width: u32, value: u32) {
        match width {
            1 => e.mem.set_u8(address, value as u8),
            2 => e.mem.set_u16(address, value as u16),
            _ => e.mem.set_u32(address, value),
        }
    }

    #[test]
    fn shapes_subpart_child_count_is_the_word_at_0x18() {
        let mut e = engine();
        let part = e.mem.alloc(0x40);
        e.mem.set_u32(part + 0x18, 7);
        assert_eq!(e.call(0x00d5_5520, &args![part]).u32(), 7);
    }

    #[test]
    fn fn_00d55600_grows_the_box_by_the_moved_product() {
        let mut e = box_engine();
        let log = new_log();
        move_double(&mut e, &log);
        let first = e.mem.alloc(0x10);
        store_vec(&mut e, first, [1.0, 2.0, 3.0, 9.0]);
        let second = e.mem.alloc(0x10);
        store_vec(&mut e, second, [4.0, 5.0, 6.0, 9.0]);
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform + 0x30, [10.0, 20.0, 30.0, 0.0]);
        let aabb = e.mem.alloc(0x20);
        store_vec(&mut e, aabb, [100.0; 4]);
        store_vec(&mut e, aabb + 0x10, [-100.0; 4]);
        e.call(0x00d5_5600, &args![first, second, aabb, transform]);
        let expected = vec![vec![
            transform,
            4.0f32.to_bits(),
            10.0f32.to_bits(),
            18.0f32.to_bits(),
            0,
        ]];
        assert_eq!(calls_to(&log, SET_TRANSFORMED_POS), expected);
        assert_eq!(load_vec(&e, aabb), [14.0, 30.0, 48.0, 0.0]);
        assert_eq!(load_vec(&e, aabb + 0x10), [14.0, 30.0, 48.0, 0.0]);
    }

    #[test]
    fn fn_00d55680_boxes_the_triangles_of_every_index_width() {
        let mut e = box_engine();
        let log = new_log();
        move_double(&mut e, &log);
        let vertices = box_vertices(&mut e);
        let scale = e.mem.alloc(0x10);
        store_vec(&mut e, scale, [2.0, 1.0, 1.0, 1.0]);
        let aabb = e.mem.alloc(0x20);
        let subpart = e.mem.alloc(0x60);
        e.mem.set_u32(subpart + 0x14, 2);
        e.mem.set_u32(subpart + 0x18, vertices);
        e.mem.set_u32(subpart + 0x1c, 12);
        for (kind, width) in [(1u8, 1u32), (2, 2), (3, 4)] {
            let indices = e.mem.alloc(0x20);
            for (slot, value) in [0u32, 1, 2, 1, 2, 3].into_iter().enumerate() {
                put_index(&mut e, indices + width * slot as u32, width, value);
            }
            e.mem.set_u32(subpart + 0x40, indices);
            e.mem.set_u32(subpart + 0x44, 3 * width);
            e.mem.set_u8(subpart + 0x48, kind);
            log.borrow_mut().clear();
            e.call(0x00d5_5680, &args![subpart, scale, aabb]);
            assert_eq!(calls_to(&log, SET_TRANSFORMED_POS).len(), 6);
            assert_eq!(load_vec(&e, aabb), [-2.0, -2.0, -9.0, 0.0]);
            assert_eq!(load_vec(&e, aabb + 0x10), [8.0, 5.0, 7.0, 0.0]);
        }
        // Without triangles the box stays empty.
        e.mem.set_u32(subpart + 0x14, 0);
        e.call(0x00d5_5680, &args![subpart, scale, aabb]);
        assert_eq!(load_vec(&e, aabb), [1.0e30; 4]);
        assert_eq!(load_vec(&e, aabb + 0x10), [-1.0e30; 4]);
    }

    #[test]
    fn fn_00d558b0_boxes_the_children_with_the_subpart_transform() {
        let mut e = box_engine();
        let log = new_log();
        box_double(&mut e, &log);
        let sink = Rc::clone(&log);
        e.register_double(SET_TRANSFORM, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|lane| e.mem.u32(a[0] + 0x30 + 4 * lane)));
            sink.borrow_mut().push((SET_TRANSFORM, words));
            Ret::default()
        });
        let first = box_child(&mut e, [-1.0, -2.0, -3.0, 0.0], [1.0, 2.0, 3.0, 0.0]);
        let second = box_child(&mut e, [0.0; 4], [5.0, 1.0, 1.0, 0.0]);
        let subpart = e.mem.alloc(0x40);
        let list = shape_list(&mut e, &[first, second]);
        e.mem.set_u32(subpart + 0x14, list);
        e.mem.set_u32(subpart + 0x18, 2);
        store_vec(&mut e, subpart + 0x30, [7.0, 8.0, 9.0, 1.0]);
        let aabb = e.mem.alloc(0x20);
        e.call(0x00d5_58b0, &args![aabb, subpart]);
        let transform = calls_to(&log, SET_TRANSFORM);
        assert_eq!(transform.len(), 1);
        assert_eq!(transform[0][1], subpart + 0x20);
        let translation = [7.0f32, 8.0, 9.0, 1.0].map(f32::to_bits);
        assert_eq!(transform[0][2..], translation);
        let boxes = calls_to(&log, BOX_DOUBLE);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[0][..3], [first, transform[0][0], 0]);
        assert_eq!(boxes[1][..3], [second, transform[0][0], 0]);
        assert_eq!(boxes[0][3], boxes[1][3]);
        assert_ne!(boxes[0][3], aabb);
        assert_eq!(load_vec(&e, aabb), [-1.0, -2.0, -3.0, 0.0]);
        assert_eq!(load_vec(&e, aabb + 0x10), [5.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn recalc_aabb_extents_boxes_triangle_and_convex_subparts() {
        let mut e = box_engine();
        let log = new_log();
        move_double(&mut e, &log);
        box_double(&mut e, &log);
        e.register_double(SET_TRANSFORM, |_, _| Ret::default());
        let shape = new_shape(&mut e, 1);
        store_vec(&mut e, shape + 0x80, [1.0; 4]);
        e.mem.set_f32(shape + 0xe4, 0.5);
        let vertices = box_vertices(&mut e);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0002_0100]);
        triangle_subpart(&mut e, shape + 0x20, vertices, (indices, 3, 1, 0));
        e.mem.set_u32(shape + 0x20 + 0x14, 1);
        e.mem.set_u32(shape + 0xb4, shape + 0x20);
        let child = box_child(&mut e, [-3.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 0.0]);
        let convex = e.mem.alloc(0x40);
        let list = shape_list(&mut e, &[child]);
        e.mem.set_u32(convex + 0x14, list);
        e.mem.set_u32(convex + 0x18, 1);
        e.mem.set_u32(shape + 0xc0, convex);
        e.mem.set_u32(shape + 0xc4, 1);
        e.call(0x00d5_5950, &args![shape]);
        assert_eq!(load_vec(&e, shape + 0xa0), [0.75, 1.5, 3.5, 0.0]);
        assert_eq!(load_vec(&e, shape + 0x90), [3.75, 4.0, 4.0, 0.5]);
    }

    #[test]
    fn fn_00d55a90_checks_each_child_answer_against_the_kind_limit() {
        let mut e = engine();
        let shape = new_shape(&mut e, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x18, ANSWER_DOUBLE);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, vtable);
        let subparts = e.mem.alloc(0x40);
        let list = shape_list(&mut e, &[child]);
        e.mem.set_u32(subparts + 0x14, list);
        e.mem.set_u32(subparts + 0x18, 1);
        e.mem.set_u32(shape + 0xc0, subparts);
        // No subparts: the fixed answer.
        assert_eq!(e.call(0x00d5_5a90, &args![shape, 5u32, 0u32]).i32(), 0xf0);
        e.mem.set_u32(shape + 0xc4, 1);
        let log = new_log();
        for (word, limit) in [
            (0x3f00_0000u32, 0x200i32),
            (0x3f00_0001, 0x1d0),
            (0x3f00_0002, 0x1a0),
        ] {
            e.mem.set_u32(subparts + 0x3c, word);
            for (answer, expected) in [(limit, 0xf0), (limit + 1, -1), (0, 0xf0), (-1, -1)] {
                record(&mut e, &log, ANSWER_DOUBLE, answer as u32);
                let got = e.call(0x00d5_5a90, &args![shape, 0x77u32, 0u32]).i32();
                assert_eq!(got, expected, "word {word:#x} answer {answer}");
            }
        }
        assert!(calls_to(&log, ANSWER_DOUBLE)
            .iter()
            .all(|words| *words == [child, 0x77, 0x100]));
    }

    #[test]
    fn fn_00d56040_passes_the_extents_and_the_quieted_tolerance_to_calc_aabb() {
        let mut e = engine();
        let log = new_log();
        record(&mut e, &log, CALC_AABB, 0);
        let shape = e.mem.alloc(0x100);
        e.call(
            0x00d5_6040,
            &args![shape, 0x4000u32, 0x7f80_0001u32, 0x5000u32],
        );
        assert_eq!(
            calls_to(&log, CALC_AABB),
            vec![vec![
                0x4000,
                shape + 0x90,
                shape + 0xa0,
                0x7fc0_0001,
                0x5000
            ]]
        );
    }

    #[test]
    fn fn_00d56070_builds_the_welding_offsets_then_stores_the_info() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        let shape = new_shape(&mut e, 2);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        e.mem.set_u32(subparts + 0x14, 2);
        e.mem.set_u32(subparts + 0x60 + 0x14, 3);
        e.call(0x00d5_6070, &args![shape, 0x0100_0002u32, 0x1234u16]);
        assert_eq!(e.mem.u32(subparts + 0x4c), 0);
        assert_eq!(e.mem.u32(subparts + 0x60 + 0x4c), 2);
        assert_eq!(e.mem.u32(shape + 0xd0), 5);
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![vec![allocator, shape + 0xcc, 5, 2]]
        );
        let welding = e.mem.u32(shape + 0xcc);
        assert_eq!(e.mem.u16(welding + 8), 0x1234);
        // A key with the top bit set stores nothing and the array is not rebuilt.
        let before = e.mem.bytes(welding, 10);
        e.call(0x00d5_6070, &args![shape, 0x8000_0002u32, 0x4321u16]);
        assert_eq!(e.mem.bytes(welding, 10), before);
        assert_eq!(calls_to(&log, ARRAY_UTIL_RESERVE).len(), 1);
    }

    #[test]
    fn get_num_child_shapes_in_triangles_subpart_skips_degenerate_children() {
        let mut e = engine();
        let shape = new_shape(&mut e, 2);
        let vertices = vertices(&mut e);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0102_0100, 0x0000_0302]);
        triangle_subpart(&mut e, subparts + 0x60, vertices, (indices, 3, 1, 0));
        e.mem.set_u32(subparts + 0x60 + 0x14, 2);
        let flag = e.mem.alloc(4);
        let answers = RefCell::new(vec![true, false].into_iter());
        e.register_double(IS_DEGENERATE, move |e, _| {
            let answer = answers
                .borrow_mut()
                .next()
                .expect("unexpected isDegenerate");
            e.mem.set_u8(flag, u8::from(answer));
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        let got = e
            .call(0x00d5_6160, &args![shape, subparts + 0x60, 1u32])
            .i32();
        assert_eq!(got, 1);
    }

    #[test]
    fn fn_00d56200_clears_the_welding_array_for_type_6_and_rebuilds_it_otherwise() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        let shape = new_shape(&mut e, 1);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 0xcc, data);
        e.mem.set_u32(shape + 0xd0, 3);
        e.mem.set_u32(shape + 0xd4, 4);
        e.call(0x00d5_6200, &args![shape, 6u32]);
        assert_eq!(calls_to(&log, FREE_DOUBLE), vec![vec![allocator, data, 8]]);
        assert_eq!(e.mem.u8(shape + 0xd8), 6);
        assert_eq!(e.mem.u32(shape + 0xcc), 0);
        assert_eq!(e.mem.u32(shape + 0xd0), 0);
        assert_eq!(e.mem.u32(shape + 0xd4), 0x8000_0000);
        // Memory that is not owned is not freed.
        e.mem.set_u32(shape + 0xd4, 0x8000_0004);
        e.call(0x00d5_6200, &args![shape, 6u32]);
        assert_eq!(calls_to(&log, FREE_DOUBLE).len(), 1);
        // Another type rebuilds the offsets and the array.
        e.mem.set_u32(shape + 0xb4, shape + 0x20);
        e.mem.set_u32(shape + 0x20 + 0x14, 4);
        e.mem.set_u32(shape + 0x20 + 0x4c, 99);
        e.call(0x00d5_6200, &args![shape, 3u32]);
        assert_eq!(e.mem.u8(shape + 0xd8), 3);
        assert_eq!(e.mem.u32(shape + 0x20 + 0x4c), 0);
        assert_eq!(e.mem.u32(shape + 0xd0), 4);
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![vec![allocator, shape + 0xcc, 4, 2]]
        );
    }

    #[test]
    fn expand_one_triangle_subparts_uses_the_embedded_subpart_first() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        let shape = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00d5_62f0, &args![shape]).u32(), shape + 0x20);
        assert_eq!(e.mem.u32(shape + 0xb4), shape + 0x20);
        assert_eq!(e.mem.u32(shape + 0xb8), 1);
        assert_eq!(e.mem.u32(shape + 0xbc), 0x8000_0001);
        assert!(log.borrow().is_empty());
        // The second one is reserved and initialised.
        e.mem.set_u8(shape + 0x20 + 0x20, 0x5a);
        let second = e.call(0x00d5_62f0, &args![shape]).u32();
        let data = e.mem.u32(shape + 0xb4);
        assert_eq!(second, data + 0x60);
        assert_eq!(e.mem.u32(shape + 0xb8), 2);
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![vec![allocator, shape + 0xb4, 2, 0x60]]
        );
        assert_eq!(e.mem.u8(data + 0x20), 0x5a);
        assert_eq!(e.mem.u8(second), 0);
        assert_eq!(e.mem.u8(second + 1), 1);
        assert_eq!(e.mem.u16(second + 0xa), 1);
        assert_eq!(e.mem.u8(second + 0x49), 0);
        assert_eq!(e.mem.bytes(second + 0x30, 16), vec![0; 16]);
    }

    #[test]
    fn add_triangles_subpart_copies_the_subpart_and_grows_the_box() {
        let mut e = box_engine();
        let log = new_log();
        move_double(&mut e, &log);
        let shape = new_shape(&mut e, 0);
        store_vec(&mut e, shape + 0x80, [1.0; 4]);
        e.mem.set_f32(shape + 0xe4, 0.5);
        e.mem.set_u32(shape + 0xe0, 5);
        let source = e.mem.alloc(0x60);
        let vertices = box_vertices(&mut e);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0102_0100, 0x0000_0302]);
        triangle_subpart(&mut e, source, vertices, (indices, 3, 1, 0));
        e.mem.set_u32(source + 0x14, 2);
        e.register_double(TRIANGLES_SUBPART_COPY, |e, a| {
            for byte in 0..0x60 {
                let value = e.mem.u8(a[1] + byte);
                e.mem.set_u8(a[0] + byte, value);
            }
            Ret::default()
        });
        let flag = e.mem.alloc(4);
        let answers = RefCell::new(vec![false, true].into_iter());
        e.register_double(IS_DEGENERATE, move |e, _| {
            let answer = answers
                .borrow_mut()
                .next()
                .expect("unexpected isDegenerate");
            e.mem.set_u8(flag, u8::from(answer));
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        e.call(0x00d5_63a0, &args![shape, source]);
        assert_eq!(e.mem.u32(shape + 0xb8), 1);
        assert_eq!(e.mem.u32(shape + 0xe0), 6);
        assert_eq!(e.mem.u32(shape + 0x20 + 0x18), vertices);
        assert_eq!(load_vec(&e, shape + 0xa0), [1.5, 1.5, -1.0, 0.0]);
        assert_eq!(load_vec(&e, shape + 0x90), [3.0, 4.0, 8.5, 0.5]);
    }

    #[test]
    fn fn_00d56470_sets_the_transform_kind_from_translation_and_angle() {
        let mut e = box_engine();
        let subpart = e.mem.alloc(0x40);
        e.mem.set_u32(subpart + 0x3c, 0x1111);
        let got = e.call(0x00d5_6470, &args![subpart, 0u32]).u32();
        assert_eq!(got, subpart);
        assert_eq!(e.mem.u32(subpart + 0x3c), 0x1111);
        for (angle, translation, expected) in [
            (0.0005, 0.0, 0x3f00_0000u32),
            (0.5, 0.0, 0x3f00_0002),
            (0.0005, 1.0, 0x3f00_0001),
            (0.5, f32::NAN, 0x3f00_0003),
        ] {
            e.register_double(QUATERNION_GET_ANGLE, move |_, _| Ret {
                st0: angle,
                ..Ret::default()
            });
            e.mem.set_f32(subpart + 0x30, translation);
            let got = e.call(0x00d5_6470, &args![subpart, 1u32]).u32();
            assert_eq!(got, subpart);
            assert_eq!(e.mem.u32(subpart + 0x3c), expected);
        }
    }

    #[test]
    fn fn_00d56500_builds_a_translated_convex_subpart() {
        let mut e = box_engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        record(&mut e, &log, REFERENCED_OBJECT_ADD, 0);
        record(&mut e, &log, REFERENCED_OBJECT_RELEASE, 0);
        let (first, second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        let shapes = shape_list(&mut e, &[first, 0, second]);
        let translation = e.mem.alloc(0x10);
        store_vec(&mut e, translation, [1.0, 0.0, 0.0, 0.0]);
        let this = e.mem.alloc(0x40);
        let got = e
            .call(0x00d5_6500, &args![this, shapes, 3i32, translation])
            .u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![vec![allocator, this + 0x14, 3, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 3);
        let list = e.mem.u32(this + 0x14);
        assert_eq!(e.mem.bytes(list, 12), e.mem.bytes(shapes, 12));
        assert_eq!(
            calls_to(&log, REFERENCED_OBJECT_ADD),
            vec![vec![first], vec![second]]
        );
        assert!(calls_to(&log, REFERENCED_OBJECT_RELEASE).is_empty());
        assert_eq!(e.mem.u8(this), 1);
        assert_eq!(e.mem.u8(this + 1), 1);
        assert_eq!(e.mem.u16(this + 0xa), 1);
        assert_eq!(load_vec(&e, this + 0x20), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, this + 0x30)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x3c), 0x3f00_0001);
    }

    #[test]
    fn shapes_subpart_construct_takes_rotation_and_translation_from_the_transform() {
        let mut e = box_engine();
        let log = new_log();
        router(&mut e, &log);
        reserve_double(&mut e, &log);
        record(&mut e, &log, QUATERNION_SET, 0);
        record(&mut e, &log, REFERENCED_OBJECT_ADD, 0);
        record(&mut e, &log, REFERENCED_OBJECT_RELEASE, 0);
        let child = e.mem.alloc(0x10);
        let shapes = shape_list(&mut e, &[child]);
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform + 0x30, [0.0, 0.0, 2.0, 5.0]);
        for (is_identity, expected) in [(1u8, 0x3f00_0001u32), (0, 0x3f00_0003)] {
            e.register_double(MATRIX_IS_APPROXIMATELY_EQUAL, move |e, a| {
                assert_eq!(a[0], transform);
                assert_eq!(a[2], IDENTITY_TRANSFORM);
                assert_eq!(a[3], 0.001f32.to_bits());
                e.mem.set_u8(a[1], is_identity);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
            let this = e.mem.alloc(0x40);
            let got = e
                .call(0x00d5_6660, &args![this, shapes, 1i32, transform])
                .u32();
            assert_eq!(got, this);
            assert_eq!(e.mem.u32(this + 0x3c), expected);
            assert_eq!(load_vec(&e, this + 0x30)[..3], [0.0, 0.0, 2.0]);
            assert_eq!(e.mem.u32(this + 0x18), 1);
            let list = e.mem.u32(this + 0x14);
            assert_eq!(e.mem.u32(list), child);
        }
        assert_eq!(
            calls_to(&log, QUATERNION_SET).len(),
            2,
            "one hkQuaternion::set per construction"
        );
        assert_eq!(calls_to(&log, QUATERNION_SET)[0][1], transform);
    }

    #[test]
    fn fn_00d56800_releases_the_shapes_and_frees_an_owned_list() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        record(&mut e, &log, REFERENCED_OBJECT_RELEASE, 0);
        let (first, third) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        let subpart = e.mem.alloc(0x40);
        let list = shape_list(&mut e, &[first, 0, third]);
        e.mem.set_u32(subpart + 0x14, list);
        e.mem.set_u32(subpart + 0x18, 3);
        e.mem.set_u32(subpart + 0x1c, 3);
        e.call(0x00d5_6800, &args![subpart]);
        assert_eq!(
            calls_to(&log, REFERENCED_OBJECT_RELEASE),
            vec![vec![first], vec![third]]
        );
        assert_eq!(e.mem.bytes(list, 12), vec![0; 12]);
        assert_eq!(calls_to(&log, FREE_DOUBLE), vec![vec![allocator, list, 12]]);
        assert_eq!(e.mem.u32(subpart + 0x18), 0);
        assert_eq!(e.mem.u32(subpart + 0x1c), 0x8000_0000);
        assert_eq!(e.mem.u32(subpart + 0x14), 0);
        // A list that is not owned is not freed.
        let list = shape_list(&mut e, &[first]);
        e.mem.set_u32(subpart + 0x14, list);
        e.mem.set_u32(subpart + 0x18, 1);
        e.mem.set_u32(subpart + 0x1c, 0x8000_0001);
        e.call(0x00d5_6800, &args![subpart]);
        assert_eq!(calls_to(&log, FREE_DOUBLE).len(), 1);
    }

    #[test]
    fn fn_00d56880_appends_a_default_convex_subpart() {
        let mut e = engine();
        let log = new_log();
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        let shape = e.mem.alloc(0x100);
        let first = e.call(0x00d5_6880, &args![shape]).u32();
        assert_eq!(first, e.mem.u32(shape + 0xc0));
        assert_eq!(e.mem.u32(shape + 0xc4), 1);
        assert_eq!(e.mem.u8(first), 1);
        assert_eq!(e.mem.u8(first + 1), 1);
        assert_eq!(e.mem.u16(first + 0xa), 1);
        assert_eq!(e.mem.u32(first + 0x1c), 0x8000_0000);
        e.mem.set_u32(first + 0x30, 0x77);
        let second = e.call(0x00d5_6880, &args![shape]).u32();
        assert_eq!(second, e.mem.u32(shape + 0xc0) + 0x40);
        assert_eq!(e.mem.u32(shape + 0xc4), 2);
        assert_eq!(e.mem.u32(e.mem.u32(shape + 0xc0) + 0x30), 0x77);
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![
                vec![allocator, shape + 0xc0, 1, 0x40],
                vec![allocator, shape + 0xc0, 2, 0x40]
            ]
        );
    }

    #[test]
    fn fn_00d56900_adds_a_convex_subpart_and_grows_the_box() {
        let mut e = box_engine();
        let log = new_log();
        router(&mut e, &log);
        reserve_double(&mut e, &log);
        box_double(&mut e, &log);
        e.register_double(SET_TRANSFORM, |_, _| Ret::default());
        e.register_double(SHAPES_SUBPART_COPY, |e, a| {
            for byte in 0..0x40 {
                let value = e.mem.u8(a[1] + byte);
                e.mem.set_u8(a[0] + byte, value);
            }
            Ret::default()
        });
        let shape = new_shape(&mut e, 0);
        e.mem.set_u32(shape + 0xe0, 7);
        let child = box_child(&mut e, [-2.0, -1.0, -1.0, 0.0], [3.0, 1.0, 1.0, 0.0]);
        let source = e.mem.alloc(0x40);
        let list = shape_list(&mut e, &[child]);
        e.mem.set_u32(source + 0x14, list);
        e.mem.set_u32(source + 0x18, 1);
        let got = e.call(0x00d5_6900, &args![shape, source]).i32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(shape + 0xc4), 1);
        assert_eq!(e.mem.u32(shape + 0xe0), 8);
        let added = e.mem.u32(shape + 0xc0);
        assert_eq!(e.mem.u32(added + 0x14), list);
        assert_eq!(load_vec(&e, shape + 0xa0), [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, shape + 0x90), [2.5, 1.0, 1.0, 0.0]);
    }

    #[test]
    fn hkp_extended_mesh_shape_construct_installs_the_empty_state() {
        let mut e = box_engine();
        let log = new_log();
        record(&mut e, &log, SHAPE_COLLECTION_CONSTRUCT, 0);
        let this = e.mem.alloc(0x100);
        // A signalling NaN radius is quieted by the x87 load.
        let got = e
            .call(
                0x00d5_69c0,
                &args![this, f32::from_bits(0x7f80_0001), 12u32],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_to(&log, SHAPE_COLLECTION_CONSTRUCT),
            vec![vec![this, 0xe, 1]]
        );
        assert_eq!(e.mem.u32(this), EXTENDED_MESH_SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), EXTENDED_MESH_CONTAINER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xe4), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0xb0), 12);
        assert_eq!(e.mem.u8(this + 0xd8), 6);
        assert_eq!(e.mem.u8(this + 0x21), 1);
        assert_eq!(e.mem.u16(this + 0x2a), 1);
        assert_eq!(load_vec(&e, this + 0x80), [1.0; 4]);
        assert_eq!(load_vec(&e, this + 0x90), [0.25; 4]);
        assert_eq!(load_vec(&e, this + 0xa0), [0.0; 4]);
        for array in [0xb4, 0xc0, 0xcc] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
    }

    #[test]
    fn hkp_extended_mesh_shape_construct_ov2_copies_the_source_data() {
        let mut e = box_engine();
        let log = new_log();
        move_double(&mut e, &log);
        e.register_double(SET_TRANSFORM, |_, _| Ret::default());
        let allocator = router(&mut e, &log);
        reserve_double(&mut e, &log);
        record(&mut e, &log, SHAPE_COLLECTION_CONSTRUCT, 0);
        record(&mut e, &log, CONTAINER_GET_NUM_CHILD_SHAPES, 11);
        let vertices = box_vertices(&mut e);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0002_0100]);
        let source = e.mem.alloc(0x80);
        store_vec(&mut e, source + 0x20, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u32(source + 0x30, 9);
        let subparts = e.mem.alloc(0x70);
        e.mem.set_u32(source + 0x34, subparts);
        e.mem.set_u32(source + 0x38, 2);
        let welding = e.mem.alloc(0x10);
        e.mem.set_u16(welding, 0x11);
        e.mem.set_u16(welding + 2, 0x22);
        e.mem.set_u16(welding + 4, 0x33);
        e.mem.set_u32(source + 0x40, welding);
        e.mem.set_u32(source + 0x44, 3);
        e.mem.set_u8(source + 0x4c, 5);
        e.mem.set_f32(source + 0x50, 0.5);
        for (field, value) in [
            (0x00, vertices),
            (0x04, 12),
            (0x08, 0x11),
            (0x0c, indices),
            (0x14, 3),
            (0x1c, 1),
            (0x20, 0x22),
            (0x28, 0x33),
        ] {
            e.mem.set_u32(subparts + field, value);
        }
        e.mem.set_u8(subparts + 0x10, 1);
        e.mem.set_u8(subparts + 0x11, 1);
        e.mem.set_u16(subparts + 0x24, 4);
        e.mem.set_u16(subparts + 0x2c, 5);
        e.mem.set_u16(subparts + 0x30, 6);
        e.mem.set_u32(subparts + 0x38 + 0x34, 1);
        let this = e.mem.alloc(0x100);
        let got = e.call(0x00d5_6ab0, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls_to(&log, SHAPE_COLLECTION_CONSTRUCT),
            vec![vec![this, 0xe, 1]]
        );
        assert_eq!(e.mem.u32(this), EXTENDED_MESH_SHAPE_VTABLE);
        assert_eq!(load_vec(&e, this + 0x80), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.f32(this + 0xe4), 0.5);
        assert_eq!(e.mem.u32(this + 0xb0), 9);
        assert_eq!(e.mem.u8(this + 0xd8), 5);
        // The welding block is asked for the byte size and may be larger.
        assert_eq!(calls_to(&log, SIZED_ALLOC_DOUBLE), vec![vec![6]]);
        assert_eq!(e.mem.u32(this + 0xd0), 3);
        assert_eq!(e.mem.u32(this + 0xd4), 5);
        let copied = e.mem.u32(this + 0xcc);
        assert_eq!(e.mem.bytes(copied, 6), e.mem.bytes(welding, 6));
        // Two subparts are reserved and filled in.
        assert_eq!(
            calls_to(&log, ARRAY_UTIL_RESERVE),
            vec![vec![allocator, this + 0xb4, 2, 0x60]]
        );
        assert_eq!(e.mem.u32(this + 0xb8), 2);
        let first = e.mem.u32(this + 0xb4);
        assert_eq!(e.mem.u32(first + 0x14), 1);
        assert_eq!(e.mem.u32(first + 0x04), 0x22);
        assert_eq!(e.mem.u16(first + 0x08), 4);
        assert_eq!(e.mem.u8(first + 0x01), 1);
        assert_eq!(e.mem.u32(first + 0x0c), 0x33);
        assert_eq!(e.mem.u16(first + 0x02), 5);
        assert_eq!(e.mem.u16(first + 0x0a), 6);
        assert_eq!(e.mem.u32(first + 0x18), vertices);
        assert_eq!(e.mem.u32(first + 0x1c), 12);
        assert_eq!(e.mem.u32(first + 0x20), 0x11);
        assert_eq!(e.mem.u32(first + 0x40), indices);
        assert_eq!(e.mem.u8(first + 0x48), 1);
        assert_eq!(e.mem.u32(first + 0x44), 3);
        assert_eq!(e.mem.u32(first + 0x60 + 0x4c), 1);
        // The box is recalculated and the child count comes from the container.
        assert_eq!(load_vec(&e, this + 0xa0), [1.5, 3.0, 10.5, 0.0]);
        assert_eq!(load_vec(&e, this + 0x90), [3.0, 7.5, 11.0, 0.5]);
        assert_eq!(e.mem.u32(this + 0xe0), 11);
        // One source subpart is the embedded one.
        e.mem.set_u32(source + 0x38, 1);
        e.mem.set_u32(source + 0x44, 0);
        let single = e.mem.alloc(0x100);
        e.call(0x00d5_6ab0, &args![single, source]);
        assert_eq!(e.mem.u32(single + 0xb4), single + 0x20);
        assert_eq!(e.mem.u32(single + 0xb8), 1);
        assert_eq!(e.mem.u32(single + 0xbc), 0x8000_0001);
        assert_eq!(e.mem.u32(single + 0x20 + 0x14), 1);
        assert_eq!(e.mem.u32(single + 0x20 + 0x18), vertices);
    }
}
