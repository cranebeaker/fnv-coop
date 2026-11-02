//! `Havok SDK/hkptrisampledheightfieldcollection.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpTriSampledHeightFieldCollection` (vtable `010ce390`, 0x40 bytes) is the shape collection
//! that exposes the two triangles of every cell of a height field as child shapes. Its
//! `hkpShapeContainer` base lives at `+0x10` (vtable `010ce370`): the container functions get the
//! address of that sub-object and step back by 0x10 to reach the collection. A child key packs
//! the cell as `z << 16 | x << 1 | second triangle`. The weld info (`m_weldingInfo`, an
//! `hkArray<u16>` at `+0x24`) holds one entry per key, indexed `((xRes - 1) * z + x) * 2 | second`.
//! The height field is an `hkpSampledHeightFieldShape` (vtable slots `0x1C` getAabbImpl, `0x34`
//! getHeightAtImpl, `0x38` getTriangleFlipImpl; `m_xRes` at `+0x10`, `m_intToFloatScale` at
//! `+0x20`). SSE code computes lane by lane in `f32`; `MINPS`/`MAXPS` keep the first operand
//! unless it compares less (greater), so a NaN yields the second operand.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpTriSampledHeightFieldCollection` (Xbox PDB), 0x40 bytes; offsets checked against the
    /// code below.
    pub struct HkpTriSampledHeightFieldCollection: 0x40 {
        /// `m_heightfield` (Xbox PDB).
        0x18 m_heightfield: Ptr,
        /// `m_radius` (Xbox PDB): the thickness added around every triangle.
        0x20 m_radius: f32,
        /// `m_weldingInfo` (Xbox PDB), the data pointer of the `hkArray<u16>`.
        0x24 m_welding_data: Ptr,
        /// The array's size.
        0x28 m_welding_size: u32,
        /// `m_triangleExtrusion` (Xbox PDB), an `hkVector4`.
        0x30 m_triangle_extrusion: [f32; 4],
    }
}

/// `hkpTriSampledHeightFieldCollection::getHeightFieldShape` (Xbox PDB, `00d26450`).
const GET_HEIGHT_FIELD_SHAPE: u32 = 0x00d2_6450;
/// The collection's vtable (`010ce390`) and its container sub-object's (`010ce370`).
const COLLECTION_VTABLE: u32 = 0x010c_e390;
const CONTAINER_VTABLE: u32 = 0x010c_e370;
/// The vtable of the triangle shape the child shape is built in (`010c749c`).
const TRIANGLE_VTABLE: u32 = 0x010c_749c;
/// The `hkpShapeCollection` constructor (Xbox PDB, `00d21410`, `RET 8`: collection type, shape
/// type...). Its two stack words are 7 and 2.
const SHAPE_COLLECTION_CONSTRUCTOR: u32 = 0x00d2_1410;
/// `hkReferencedObject::addReference` (`00c90510`, `this` = the object; the map gives no name).
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// The float at `01011d78` (0.0 in the file): the fourth lane of every vertex.
const VERTEX_LANE_FOUR: u32 = 0x0101_1d78;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

// Translated from 00d26450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldCollection::getHeightFieldShape` (Xbox PDB): the collection's
/// height field (`+0x18`).
pub fn get_height_field_shape(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldCollection>,
) -> Ptr {
    e.get(this, HkpTriSampledHeightFieldCollection::m_heightfield)
}

// Translated from 00d26460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collection's `getAabbImpl` (vtable slot `0x1C` of `010ce390` by the Xbox PDB class
/// layout; the map gives no name, unconfirmed), `(this, transform, tolerance, aabb)`: asks the
/// height field (slot `0x1C`) for its box under `transform` with the radius plus `tolerance`
/// (summed on the x87 stack, so in double precision and stored to `f32`), then widens it by
/// the triangle extrusion: `aabb.min = MINPS(min, extrusion + min)` and `aabb.max = MAXPS(max,
/// extrusion + max)`. The height field writes the box to a 32-byte local (min, then max).
pub fn fn_00d26460(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldCollection>,
    transform: Ptr,
    tolerance: f32,
    aabb: Ptr,
) {
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![this]).u32();
    let radius = e.get(this, HkpTriSampledHeightFieldCollection::m_radius);
    let sum = (f64::from(x87_quiet(radius)) + f64::from(x87_quiet(tolerance))) as f32;
    let (min, max) = e.with_stack(0x20, |e, local| {
        e.vcall(height_field, 0x1c, &args![transform, sum.to_bits(), local]);
        (load_vec(e, local.addr()), load_vec(e, local.addr() + 0x10))
    });
    let extrusion = load_vec(e, this.addr() + 0x30);
    let lowest = std::array::from_fn(|i| {
        let other = extrusion[i] + min[i];
        if min[i] < other {
            min[i]
        } else {
            other
        }
    });
    let highest = std::array::from_fn(|i| {
        let other = extrusion[i] + max[i];
        if max[i] > other {
            max[i]
        } else {
            other
        }
    });
    store_vec(e, aabb.addr(), lowest);
    store_vec(e, aabb.addr() + 0x10, highest);
}

// Translated from 00d264d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collection's `setWeldingInfo` (vtable slot `0x30` of `010ce390` by the Xbox PDB class
/// layout; the map gives no name, unconfirmed), `(this, key, info)`: stores the low 16 bits of
/// `info` at the weld array entry `((xRes - 1) * (key >> 16) + (key >> 1 & 0x7FFF)) * 2 |
/// (key & 1)`, all in 32-bit arithmetic.
pub fn fn_00d264d0(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldCollection>,
    key: u32,
    info: u16,
) {
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![this]).u32();
    let width = e.mem.u32(height_field + 0x10).wrapping_sub(1);
    let cell = width
        .wrapping_mul(key >> 16)
        .wrapping_add((key >> 1) & 0x7fff);
    let index = cell.wrapping_add(cell) | (key & 1);
    let array = e.get(this, HkpTriSampledHeightFieldCollection::m_welding_data);
    e.mem
        .set_u16(array.addr().wrapping_add(index.wrapping_mul(2)), info);
}

// Translated from 00d26510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's `getChildShape` (slot `0x14` of the container vtable `010ce370` by the Xbox
/// PDB class layout; the map gives no name, unconfirmed), `(this, key, buffer)`. `this` is the
/// container sub-object (collection + 0x10). Builds the triangle of cell `key` in `buffer` and
/// returns it:
///
/// * header (when `buffer` is not null; a null buffer would fault at address `0x20` further
///   down, which this translation keeps): reference count `+6` = 1, the collection's radius
///   (an `FLD`/`FSTP` copy) at `+0x10`, `+8` = 0, `+0xC` = 3, vtable `010c749c`, weld info
///   `+0x14` = 0, `+0x16` = 6, `+0x17` = 0 and 16 zero bytes at `+0x50`;
/// * the four corners `(x + (i >> 1), h, z + (i & 1), c)` for `i` = 0..3, `h` from the height
///   field (slot `0x34`, answered on the x87 stack), `c` the float at `01011d78`, each lane
///   multiplied by the height field's scale vector (`+0x20`, the first `MULPS` operand);
/// * the triangle's flip (slot `0x38` writes a byte into a local, which the exe does not
///   clear first; here the local starts at zero): with `f` = flip != 0 and `b` = key & 1 the
///   corners at `+0x20`, `+0x30` and `+0x40` are `v[3 * (b & !f)]`, `v[(f + 1) * b + 1]` and
///   `v[f - b + 2]`;
/// * the weld info at `+0x14` (the array entry of `setWeldingInfo` when the array is not
///   empty, else 0) and `+0x16` (0 when the array is not empty, else 6);
/// * `+0x17` = 0.0 < the sum of the squares of the extrusion's first three lanes (summed
///   `z + (y + x)`), and the extrusion vector copied to `+0x50`.
pub fn fn_00d26510(e: &mut Engine, this: Ptr, key: u32, buffer: Ptr) -> Ptr {
    let collection = Ptr::<HkpTriSampledHeightFieldCollection>::new(this.addr().wrapping_sub(0x10));
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    let x0 = (key >> 1) & 0x7fff;
    let z0 = key >> 16;
    let target = buffer.addr();
    if target != 0 {
        let radius = x87_quiet(e.mem.f32(this.addr() + 0x10));
        e.mem.set_u16(target + 6, 1);
        e.mem.set_f32(target + 0x10, radius);
        e.mem.set_u32(target + 8, 0);
        e.mem.set_u32(target + 0xc, 3);
        e.mem.set_u32(target, TRIANGLE_VTABLE);
        e.mem.set_u16(target + 0x14, 0);
        e.mem.set_u8(target + 0x16, 6);
        e.mem.write(target + 0x50, &[0; 16]);
        e.mem.set_u8(target + 0x17, 0);
    }
    let lane_four: f32 = e.global(VERTEX_LANE_FOUR);
    let mut corners = [[0f32; 4]; 4];
    for (i, corner) in corners.iter_mut().enumerate() {
        let z = (i as u32 & 1).wrapping_add(z0);
        let x = ((i as i32) >> 1).wrapping_add(x0 as i32);
        let height = e.vcall(height_field, 0x34, &args![x, z]).f32();
        let scale = load_vec(e, height_field + 0x20);
        let unscaled = [x as f32, height, z as i32 as f32, lane_four];
        *corner = std::array::from_fn(|lane| scale[lane] * unscaled[lane]);
    }
    let flip = e.with_stack(1, |e, local| {
        e.vcall(height_field, 0x38, &args![local]);
        u32::from(e.mem.u8(local.addr()) != 0)
    });
    let second = key & 1;
    let first = corners[(3 * (!flip & second)) as usize];
    let middle = corners[((flip + 1) * second + 1) as usize];
    let last = corners[(flip as i32 - second as i32 + 2) as usize];
    store_vec(e, target.wrapping_add(0x20), first);
    store_vec(e, target.wrapping_add(0x30), middle);
    store_vec(e, target.wrapping_add(0x40), last);
    let size = e.get(
        collection,
        HkpTriSampledHeightFieldCollection::m_welding_size,
    );
    let (welding, flags) = if size != 0 {
        let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
        let width = e.mem.u32(height_field + 0x10).wrapping_sub(1);
        let cell = width.wrapping_mul(z0).wrapping_add(x0);
        let index = cell.wrapping_add(cell) | second;
        let array = e.get(
            collection,
            HkpTriSampledHeightFieldCollection::m_welding_data,
        );
        (
            e.mem.u16(array.addr().wrapping_add(index.wrapping_mul(2))),
            0,
        )
    } else {
        (0, 6)
    };
    e.mem.set_u16(target.wrapping_add(0x14), welding);
    e.mem.set_u8(target.wrapping_add(0x16), flags);
    let extrusion = load_vec(e, collection.addr() + 0x30);
    let squares: Vec4 = std::array::from_fn(|lane| extrusion[lane] * extrusion[lane]);
    let sum = squares[2] + (squares[1] + squares[0]);
    e.mem.set_u8(target.wrapping_add(0x17), u8::from(0.0 < sum));
    store_vec(e, target.wrapping_add(0x50), extrusion);
    buffer
}

// Translated from 00d26700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldCollection::hkpTriSampledHeightFieldCollection` (Xbox PDB),
/// `(this, height field, radius)`: runs the `hkpShapeCollection` constructor (`00d21410`, stack
/// words 7 then 2), sets the vtables (`010ce390` at `+0`, `010ce370` at `+0x10`), clears the
/// weld array (`+0x24`/`+0x28`, capacity word `+0x2C` = `0x80000000`, the "does not own its
/// memory" flag), stores the radius (an `FLD`/`FSTP` copy) at `+0x20` and the height field at
/// `+0x18`, zeroes the extrusion (`+0x30`) and adds a reference to the height field
/// (`00c90510`). Returns `this`.
pub fn hkp_tri_sampled_height_field_collection_hkp_tri_sampled_height_field_collection(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldCollection>,
    height_field: Ptr,
    radius: f32,
) -> Ptr<HkpTriSampledHeightFieldCollection> {
    e.call(SHAPE_COLLECTION_CONSTRUCTOR, &args![this, 7u32, 2u32]);
    e.mem.set_u32(this.addr(), COLLECTION_VTABLE);
    e.mem.set_u32(this.addr() + 0x10, CONTAINER_VTABLE);
    e.mem.set_u32(this.addr() + 0x24, 0);
    e.mem.set_u32(this.addr() + 0x28, 0);
    e.mem.set_u32(this.addr() + 0x2c, 0x8000_0000);
    e.mem.set_f32(this.addr() + 0x20, x87_quiet(radius));
    e.set(
        this,
        HkpTriSampledHeightFieldCollection::m_heightfield,
        height_field,
    );
    store_vec(e, this.addr() + 0x30, [0.0; 4]);
    e.call(ADD_REFERENCE, &args![height_field]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_6450,
            get_height_field_shape(Ptr<HkpTriSampledHeightFieldCollection>) -> Ptr
        ),
        entry!(
            0x00d2_6460,
            fn_00d26460(Ptr<HkpTriSampledHeightFieldCollection>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2_64d0,
            fn_00d264d0(Ptr<HkpTriSampledHeightFieldCollection>, u32, u16)
        ),
        entry!(0x00d2_6510, fn_00d26510(Ptr, u32, Ptr) -> Ptr),
        entry!(
            0x00d2_6700,
            hkp_tri_sampled_height_field_collection_hkp_tri_sampled_height_field_collection(
                Ptr<HkpTriSampledHeightFieldCollection>,
                Ptr,
                f32,
            )
                -> Ptr<
                HkpTriSampledHeightFieldCollection,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Doubles standing in for the height field's virtual functions.
    const AABB_SLOT: u32 = 0x0900_001c;
    const HEIGHT_SLOT: u32 = 0x0900_0034;
    const FLIP_SLOT: u32 = 0x0900_0038;

    /// A collection (with the real `getHeightFieldShape`) over a height field whose x
    /// resolution is `x_res` and scale vector `(1, 2, 3, 4)`. Returns `(collection, height field)`.
    fn collection(e: &mut Engine, x_res: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, AABB_SLOT);
        e.mem.set_u32(vtable + 0x34, HEIGHT_SLOT);
        e.mem.set_u32(vtable + 0x38, FLIP_SLOT);
        let height_field = e.mem.alloc(0x40);
        e.mem.set_u32(height_field, vtable);
        e.mem.set_u32(height_field + 0x10, x_res);
        for (lane, scale) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(height_field + 0x20 + 4 * lane as u32, scale);
        }
        let collection = e.mem.alloc(0x40);
        e.mem.set_u32(collection + 0x18, height_field);
        (collection, height_field)
    }

    #[test]
    fn get_height_field_shape_returns_the_stored_pointer() {
        let mut e = engine();
        let collection = e.mem.alloc(0x40);
        e.mem.set_u32(collection + 0x18, 0x1234_5678);
        assert_eq!(e.call(0x00d2_6450, &args![collection]).u32(), 0x1234_5678);
    }

    /// Runs `fn_00d26460` with a height field that answers the box `(min, max)` and returns
    /// the arguments it saw and the box written.
    fn run_aabb(
        extrusion: [f32; 4],
        min: [f32; 4],
        max: [f32; 4],
        tolerance: f32,
    ) -> (Vec<u32>, [f32; 8]) {
        let mut e = engine();
        let (collection, _) = collection(&mut e, 4);
        e.mem.set_f32(collection + 0x20, 0.5);
        for (lane, value) in extrusion.into_iter().enumerate() {
            e.mem.set_f32(collection + 0x30 + 4 * lane as u32, value);
        }
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        let answer: Vec<f32> = min.into_iter().chain(max).collect();
        e.register_double(AABB_SLOT, move |e, a| {
            sink.borrow_mut().extend_from_slice(&a[..3]);
            for (i, value) in answer.iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, *value);
            }
            Ret::default()
        });
        let aabb = e.mem.alloc(0x20);
        e.call(0x00d2_6460, &args![collection, 0x7000u32, tolerance, aabb]);
        let written = std::array::from_fn(|i| e.mem.f32(aabb + 4 * i as u32));
        let seen = seen.borrow().clone();
        (seen, written)
    }

    #[test]
    fn aabb_asks_the_height_field_with_radius_plus_tolerance() {
        let (seen, _) = run_aabb([0.0; 4], [0.0; 4], [0.0; 4], 0.25);
        // The height field, the transform and 0.5 + 0.25 as bits (the fourth word is the local).

        assert_eq!(seen[1..], [0x7000, 0.75f32.to_bits()]);
    }

    #[test]
    fn aabb_widens_by_the_extrusion() {
        let (_, written) = run_aabb(
            [1.0, -2.0, 0.0, 3.0],
            [10.0, 20.0, 30.0, 40.0],
            [11.0, 21.0, 31.0, 41.0],
            0.0,
        );
        // Minimum: the smaller of the box corner and corner + extrusion.
        assert_eq!(written[..4], [10.0, 18.0, 30.0, 40.0]);
        // Maximum: the larger one.
        assert_eq!(written[4..], [12.0, 21.0, 31.0, 44.0]);
    }

    #[test]
    fn set_welding_info_stores_by_cell_and_triangle() {
        let mut e = engine();
        let (collection, _) = collection(&mut e, 5);
        let array = e.mem.alloc(0x100);
        e.mem.set_u32(collection + 0x24, array);
        // z = 3, x = 2, second triangle: ((5 - 1) * 3 + 2) * 2 | 1 = 29.
        let key = (3u32 << 16) | (2 << 1) | 1;
        e.call(0x00d2_64d0, &args![collection, key, 0xffff_beefu32]);
        assert_eq!(e.mem.u16(array + 29 * 2), 0xbeef);
        assert_eq!(e.mem.u16(array + 28 * 2), 0);
        assert_eq!(e.mem.u16(array + 30 * 2), 0);
        // First triangle of the same cell.
        e.call(0x00d2_64d0, &args![collection, key & !1, 7u32]);
        assert_eq!(e.mem.u16(array + 28 * 2), 7);
    }

    /// The triangle `fn_00d26510` builds for `key`: the buffer's bytes, with a height field
    /// whose height at `(x, z)` is `10 x + z`, flip byte `flip`, a weld array of `size`
    /// entries `0x100 + index` and the given extrusion.
    fn child(key: u32, flip: u8, size: u32, extrusion: [f32; 4]) -> (u32, Vec<u8>, Engine) {
        let mut e = engine();
        let (collection, _) = collection(&mut e, 5);
        e.mem.set_f32(collection + 0x20, 0.125);
        e.mem.set_u32(collection + 0x28, size);
        let array = e.mem.alloc(0x100);
        for index in 0..0x40u32 {
            e.mem.set_u16(array + 2 * index, 0x100 + index as u16);
        }
        e.mem.set_u32(collection + 0x24, array);
        for (lane, value) in extrusion.into_iter().enumerate() {
            e.mem.set_f32(collection + 0x30 + 4 * lane as u32, value);
        }
        e.register_double(HEIGHT_SLOT, |_, a| Ret {
            st0: f64::from(10 * a[1] + a[2]),
            ..Ret::default()
        });
        e.register_double(FLIP_SLOT, move |e, a| {
            e.mem.set_u8(a[1], flip);
            Ret::default()
        });
        let buffer = e.mem.alloc(0x60);
        e.mem.write(buffer, &[0xaa; 0x60]);
        let result = e
            .call(0x00d2_6510, &args![collection + 0x10, key, buffer])
            .u32();
        assert_eq!(result, buffer);
        let bytes = e.mem.bytes(buffer, 0x60);
        (buffer, bytes, e)
    }

    fn lane(bytes: &[u8], offset: usize) -> f32 {
        f32::from_bits(u32::from_le_bytes(
            bytes[offset..offset + 4].try_into().unwrap(),
        ))
    }

    /// The vertex `(x, z)` of the height field the doubles describe, scaled by `(1, 2, 3, 4)`.
    fn vertex(x: u32, z: u32) -> [f32; 3] {
        [x as f32, 2.0 * (10 * x + z) as f32, 3.0 * z as f32]
    }

    fn corner(bytes: &[u8], offset: usize) -> [f32; 3] {
        [
            lane(bytes, offset),
            lane(bytes, offset + 4),
            lane(bytes, offset + 8),
        ]
    }

    #[test]
    fn child_shape_fills_the_header() {
        let (buffer, bytes, e) = child(2 | (3 << 16), 0, 0, [0.0; 4]);
        assert_eq!(u16::from_le_bytes([bytes[6], bytes[7]]), 1);
        assert_eq!(lane(&bytes, 0x10), 0.125);
        assert_eq!(e.mem.u32(buffer + 8), 0);
        assert_eq!(e.mem.u32(buffer + 0xc), 3);
        assert_eq!(e.mem.u32(buffer), 0x010c_749c);
        // No weld entries: the weld word is 0 and the kind byte is 6.
        assert_eq!(bytes[0x14..0x16], [0, 0]);
        assert_eq!(bytes[0x16], 6);
        // The bytes the function does not touch keep their value.
        assert_eq!(bytes[4], 0xaa);
        assert_eq!(bytes[0x18], 0xaa);
    }

    #[test]
    fn child_shape_picks_the_corners_by_triangle_and_flip() {
        // Cell x = 1, z = 2: corners 0..3 are (1,2) (1,3) (2,2) (2,3).
        let corners = [vertex(1, 2), vertex(1, 3), vertex(2, 2), vertex(2, 3)];
        // (second triangle, flip) -> (v0, v1, v2) as indexes into the corners.
        for (second, flip, expected) in [
            (0u32, 0u8, [0usize, 1, 2]),
            (1, 0, [3, 2, 1]),
            (0, 1, [0, 1, 3]),
            (1, 1, [0, 3, 2]),
        ] {
            let (_, bytes, _) = child((2 << 16) | (1 << 1) | second, flip, 0, [0.0; 4]);
            for (slot, index) in expected.into_iter().enumerate() {
                assert_eq!(
                    corner(&bytes, 0x20 + 0x10 * slot),
                    corners[index],
                    "second {second} flip {flip} slot {slot}"
                );
            }
        }
    }

    #[test]
    fn child_shape_reads_the_weld_entry_when_the_array_is_not_empty() {
        // z = 2, x = 1, second: ((5 - 1) * 2 + 1) * 2 | 1 = 19.
        let (_, bytes, _) = child((2 << 16) | (1 << 1) | 1, 0, 3, [0.0; 4]);
        assert_eq!(u16::from_le_bytes([bytes[0x14], bytes[0x15]]), 0x100 + 19);
        assert_eq!(bytes[0x16], 0);
    }

    #[test]
    fn child_shape_flags_a_non_empty_extrusion_and_copies_it() {
        let (_, bytes, _) = child(0, 0, 0, [0.0, 0.0, 0.5, 9.0]);
        assert_eq!(bytes[0x17], 1);
        assert_eq!(
            [
                lane(&bytes, 0x50),
                lane(&bytes, 0x54),
                lane(&bytes, 0x58),
                lane(&bytes, 0x5c)
            ],
            [0.0, 0.0, 0.5, 9.0]
        );
        let (_, bytes, _) = child(0, 0, 0, [0.0, 0.0, 0.0, 9.0]);
        assert_eq!(bytes[0x17], 0);
        // A NaN sum is not greater than zero.
        let (_, bytes, _) = child(0, 0, 0, [f32::NAN, 0.0, 0.0, 0.0]);
        assert_eq!(bytes[0x17], 0);
    }

    #[test]
    fn construction_runs_the_base_and_references_the_height_field() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, SHAPE_COLLECTION_CONSTRUCTOR, 0);
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xaa; 0x40]);
        let result = e.call(0x00d2_6700, &args![this, 0x5555u32, 2.5f32]).u32();
        assert_eq!(result, this);
        assert_eq!(
            calls(&log, SHAPE_COLLECTION_CONSTRUCTOR),
            vec![vec![this, 7, 2]]
        );
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x5555]]);
        assert_eq!(e.mem.u32(this), 0x010c_e390);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_e370);
        assert_eq!(e.mem.u32(this + 0x18), 0x5555);
        assert_eq!(e.mem.f32(this + 0x20), 2.5);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x8000_0000);
        assert_eq!(e.mem.bytes(this + 0x30, 0x10), vec![0; 0x10]);
        // Fields the constructor does not touch keep their value.
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
    }
}
