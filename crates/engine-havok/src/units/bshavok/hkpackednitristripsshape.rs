//! `BSHavok/hkpackednitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, maxps, minps, store_vec, zip_with, Vec4};

use super::bhkentity::{
    release_ni_object, stream_transfer, HEAP_ALLOCATE, INTERLOCKED_INCREMENT, MEMORY_MANAGER,
    MEMORY_MANAGER_ALLOCATE,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The base constructor (`00a5d3a0`) and base destructor (`00a5d3d0`) of the data object
/// (`this` only; called `NI_OBJECT_CONSTRUCT`/`NI_OBJECT_DESTRUCT` in other units).
const DATA_BASE_CONSTRUCT: u32 = 0x00a5_d3a0;
const DATA_BASE_DESTRUCT: u32 = 0x00a5_d3d0;
/// The memory manager's `Deallocate` (`00aa4060`, `this` = the manager, a pointer).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The vtable of the data object (`010c740c`).
const DATA_VTABLE: u32 = 0x010c_740c;
/// The shape base constructor (`hkpShapeCollection`, `00d21410`: `this`, the shape type and the
/// collection type), and the vtables the shape stores: the shape's (`010c755c`), its container
/// sub-object's (`010c753c`) and the two the destructor ends with.
const SHAPE_COLLECTION_CONSTRUCT: u32 = 0x00d2_1410;
const SHAPE_VTABLE: u32 = 0x010c_755c;
const CONTAINER_VTABLE: u32 = 0x010c_753c;
const BASE_ENDED_VTABLE: u32 = 0x010c_6fb0;
const SHAPE_ENDED_VTABLE: u32 = 0x0102_e388;
/// The stream's reader and writer objects (`stream + 0x24c` and `+ 0x250`) keep their transfer
/// functions at `+8` and `+0xC`; the stream's version number is at `+4`.
const READER_FUNCTION: u32 = 8;
const WRITER_FUNCTION: u32 = 0xc;
const READER_OBJECT: u32 = 0x24c;
const WRITER_OBJECT: u32 = 0x250;
/// The format words handed to the transfer functions as their fourth argument: the triangle data
/// (`011b1734` when writing, `011b1754` when reading) and the vertex data (`011b1730` when
/// writing, `011b1750` when reading).
const WRITE_TRIANGLE_FORMAT: u32 = 0x011b_1734;
const WRITE_VERTEX_FORMAT: u32 = 0x011b_1730;
const READ_TRIANGLE_FORMAT: u32 = 0x011b_1754;
const READ_VERTEX_FORMAT: u32 = 0x011b_1750;

// Translated from 00ca40f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ca_3b90`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ca40f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ca_3b90, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ca3980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ca3570`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn hk_packed_ni_tri_strips_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_3570;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

layout! {
    /// `hkPackedNiTriStripsData` (Xbox PDB), 0x30 bytes.
    pub struct HkPackedNiTriStripsData: 0x30 {
        /// `uiKeyCount` (Xbox PDB): the number of triangles (shape keys).
        0x08 uiKeyCount: u32,
        /// `uiVertCount` (Xbox PDB): the number of vertices.
        0x0c uiVertCount: u32,
        /// `usPartCount` (Xbox PDB): the number of sub-parts.
        0x10 usPartCount: u16,
        /// `paTriData` (Xbox PDB): `TriangleData*`, 8 bytes each (`us1`, `us2`, `us3`, unused).
        0x14 paTriData: u32,
        /// `paVertData` (Xbox PDB): `NiPoint3*`, 12 bytes each.
        0x18 paVertData: u32,
        /// `paSubPartData` (Xbox PDB): `SubPartData*`, 12 bytes each (`uiFilter`, `uiNumVerts`, `uiMaterial`).
        0x24 paSubPartData: u32,
    }

    /// `hkPackedNiTriStripsShape` (Xbox PDB), 0xb0 bytes. Its `hkpShapeContainer` sub-object
    /// (`hkpShapeCollection`'s second base) sits at +0x10: the container methods get that address
    /// as `this`.
    pub struct HkPackedNiTriStripsShape: 0xb0 {
        /// `m_disableWelding` (Xbox PDB, `hkBool`).
        0x14 m_disableWelding: u8,
        /// `fAabbScale` (Xbox PDB).
        0x80 fAabbScale: f32,
        /// `spTriStripsData` (Xbox PDB): `NiPointer<hkPackedNiTriStripsData>`.
        0x84 spTriStripsData: Ptr<HkPackedNiTriStripsData>,
        /// `fRadius` (Xbox PDB).
        0xa0 fRadius: f32,
    }
}

/// Offset of the `hkpShapeContainer` sub-object inside `hkPackedNiTriStripsShape`.
const CONTAINER_OFFSET: u32 = 0x10;
/// `AabbTransform` (Xbox PDB, `hkTransform`), `WorldAabb` (Xbox PDB, `hkAabb`: min then max)
/// and `vScale` (Xbox PDB, `hkVector4`).
const AABB_TRANSFORM: u32 = 0x20;
const WORLD_AABB: u32 = 0x60;
const SCALE: u32 = 0x90;

/// `hkTransform::isApproximatelyEqual` (`00cb2160`, `this` = the transform; stack
/// `(result, other, tolerance)`; answers the `result` address, holding an `hkBool`).
const TRANSFORM_IS_APPROXIMATELY_EQUAL: u32 = 0x00cb_2160;
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = out; `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The tolerance (`0.001f`) `getAabbImpl` compares the transforms with.
const TRANSFORM_TOLERANCE: u32 = 0x0101_7d00;
/// `FLT_MAX` as a `double`: a `WorldAabb` minimum equal to it marks the cache as unset.
const AABB_UNSET_LIMIT: u32 = 0x0104_e138;
/// The lane fill values of an empty box: the first three lanes of the minimum, of the maximum,
/// and the last lane of both.
const EMPTY_MIN_FILL: u32 = 0x0104_da48;
const EMPTY_MAX_FILL: u32 = 0x0104_e148;
const EMPTY_W_FILL: u32 = 0x0101_1d78;

/// The vtable of the triangle shape (`hkNormalTriangleShape` by its RTTI) that `getChildShape`
/// builds in the caller's buffer.
const TRIANGLE_SHAPE_VTABLE: u32 = 0x010c_74ec;

// Translated from 00ca3550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getType` (`hkpShape` slot `0x34`, Xbox PDB): the constant `0x17`.
/// The linker folded this body with another class's slot (`010c75f8`).
pub fn hk_packed_ni_tri_strips_shape_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x17
}

// Translated from 00ca35d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getNextKey` (`hkpShapeContainer` slot `0x0c`, Xbox PDB); `this` is
/// the container sub-object. A shape key holds the triangle index in the low 24 bits and the
/// sub-part in the top byte. The next key is the next triangle, or `0xffffffff` past the last
/// one; its sub-part moves on when the next triangle's first vertex index (`us1`) is not below
/// the `uiNumVerts` of the sub-parts `0..=part` added up.
pub fn hk_packed_ni_tri_strips_shape_get_next_key(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let next = (key & 0x00ff_ffff).wrapping_add(1);
    let mut part = key >> 24;
    if next >= e.get(data, HkPackedNiTriStripsData::uiKeyCount) {
        return 0xffff_ffff;
    }
    let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
    // Pairs of sub-parts are added two at a time (even and odd running sums), then a last
    // single one when `part` is even.
    let mut even_sum = 0u32;
    let mut odd_sum = 0u32;
    let mut pair_end = 0u32;
    if (part as i32).wrapping_add(1) >= 2 {
        let pairs = part.wrapping_add(1) >> 1;
        for pair in 0..pairs {
            even_sum = even_sum.wrapping_add(e.mem.u32(sub_parts + 0x18 * pair + 4));
            odd_sum = odd_sum.wrapping_add(e.mem.u32(sub_parts + 0x18 * pair + 0x10));
        }
        pair_end = pairs * 2;
    }
    let last = if (pair_end as i32) <= (part as i32) {
        e.mem.u32(sub_parts + pair_end * 12 + 4)
    } else {
        0
    };
    let triangles = e.get(data, HkPackedNiTriStripsData::paTriData);
    let first_vertex = u32::from(e.mem.u16(triangles + next * 8));
    let total = odd_sum.wrapping_add(even_sum).wrapping_add(last);
    if first_vertex >= total {
        part = part.wrapping_add(1);
    }
    (next & 0x00ff_ffff) | (part << 24)
}

// Translated from 00ca35f5 (decompiled, FalloutNV.exe 1.4.0.525)
/// The second half of `hkPackedNiTriStripsShape::getNextKey` (Ghidra made a function of it; its
/// one reference is the conditional jump at `00ca35eb` in `00ca35d0`, whose translation above
/// carries the same steps). It runs on the frame of `getNextKey`: `EBP` holds the data object,
/// `EDX` the sub-part of the key and the frame word at `ESP + 4` the next triangle index; the
/// uniform form takes them in that order. The sub-parts `0..=part` add up their `uiNumVerts`
/// (pairs first, through even and odd running sums, then a last single one when `part` is
/// even); the answer is the key of the next triangle, whose sub-part moves on when the
/// triangle's first vertex index (`us1`) is not below that sum. The exe also clears the key
/// argument slot of `getNextKey` (`ESP + 0x18`), which its `RET 4` drops unread.
pub fn fn_00ca35f5(
    e: &mut Engine,
    data: Ptr<HkPackedNiTriStripsData>,
    part: u32,
    next: u32,
) -> u32 {
    let mut even_sum = 0u32;
    let mut odd_sum = 0u32;
    let mut pair_end = 0u32;
    if (part.wrapping_add(1) as i32) >= 2 {
        let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
        let pairs = part.wrapping_add(1) >> 1;
        pair_end = pairs.wrapping_mul(2);
        let mut cursor = sub_parts.wrapping_add(0x10);
        for _ in 0..pairs {
            even_sum = even_sum.wrapping_add(e.mem.u32(cursor.wrapping_sub(0xc)));
            odd_sum = odd_sum.wrapping_add(e.mem.u32(cursor));
            cursor = cursor.wrapping_add(0x18);
        }
    }
    let last = if (pair_end as i32) <= (part as i32) {
        let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
        e.mem.u32(
            sub_parts
                .wrapping_add(pair_end.wrapping_mul(12))
                .wrapping_add(4),
        )
    } else {
        0
    };
    let triangles = e.get(data, HkPackedNiTriStripsData::paTriData);
    let first_vertex = u32::from(e.mem.u16(triangles.wrapping_add(next.wrapping_mul(8))));
    let total = odd_sum.wrapping_add(even_sum).wrapping_add(last);
    let part = if first_vertex >= total {
        part.wrapping_add(1)
    } else {
        part
    };
    (next & 0x00ff_ffff) | (part << 24)
}

// Translated from 00ca3670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getCollisionFilterInfo` (`hkpShapeContainer` slot `0x10`, Xbox
/// PDB); `this` is the container sub-object. The filter (`uiFilter`) of the key's sub-part
/// (the key's top byte), or 0 when that is not below `usPartCount`.
pub fn hk_packed_ni_tri_strips_shape_get_collision_filter_info(
    e: &mut Engine,
    this: Ptr,
    key: u32,
) -> u32 {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let part = key >> 24;
    if part >= u32::from(e.get(data, HkPackedNiTriStripsData::usPartCount)) {
        return 0;
    }
    let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
    e.mem.u32(sub_parts + part * 12)
}

// Translated from 00ca39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the triangle shape (`hkNormalTriangleShape` by its RTTI; `hkpTriangleShape`
/// layout, Xbox PDB) that this unit builds for `getChildShape`: radius at `+0x10`, reference
/// count 1, user data 0, shape type 3, welding info 0, welding type 6, not extruded, extrusion
/// zeroed, vtable `010c74ec`. Answers `this`.
pub fn hk_normal_triangle_shape_construct(e: &mut Engine, this: Ptr, radius: f32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0x10, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 3);
    e.mem.set_u16(me + 0x14, 0);
    e.mem.set_u8(me + 0x16, 6);
    for lane in 0..4 {
        e.mem.set_u32(me + 0x50 + 4 * lane, 0);
    }
    e.mem.set_u8(me + 0x17, 0);
    e.mem.set_u32(me, TRIANGLE_SHAPE_VTABLE);
    this
}

// Translated from 00ca3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::isWeldingEnabled` (`hkpShapeContainer` slot `0x18`, Xbox PDB);
/// `this` is the container sub-object, so the byte at `this + 4` is `m_disableWelding`. True when
/// it is 0. The linker folded this body with 12 other vtable slots.
pub fn hk_packed_ni_tri_strips_shape_is_welding_enabled(e: &mut Engine, this: Ptr) -> bool {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    e.get(shape, HkPackedNiTriStripsShape::m_disableWelding) == 0
}

// Translated from 00ca3c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca3c40 - /GS stack-cookie check (00ec408c) is not modeled
/// `hkPackedNiTriStripsShape::getChildShape` (`hkpShapeContainer` slot `0x14`, Xbox PDB);
/// `this` is the container sub-object; stack `(key, buffer)`. Answers 0 when the key's triangle
/// index (low 24 bits) is not below `uiKeyCount`. Otherwise it constructs a triangle shape in
/// `buffer` (`00ca39a0`, radius `fRadius`), copies the triangle's three vertices (`NiPoint3`s
/// through the x87, last lane 0) into `+0x20`, `+0x30` and `+0x40`, and, when the first lane of
/// `vScale` is not 1.0 (NaN included), multiplies each vertex lane by lane by `vScale`.
/// Answers `buffer`. A null `buffer` is not checked by the exe (it would write at address 0x20).
pub fn hk_packed_ni_tri_strips_shape_get_child_shape(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    buffer: Ptr,
) -> Ptr {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let index = key & 0x00ff_ffff;
    if e.get(data, HkPackedNiTriStripsData::uiKeyCount) <= index {
        return Ptr::new(0);
    }
    let triangle = e.get(data, HkPackedNiTriStripsData::paTriData) + index * 8;
    let out = if buffer.addr() == 0 {
        0
    } else {
        let radius = e.get(shape, HkPackedNiTriStripsShape::fRadius);
        hk_normal_triangle_shape_construct(e, buffer, x87_quiet(radius)).addr()
    };
    for corner in 0..3u32 {
        let vertex_index = u32::from(e.mem.u16(triangle + 2 * corner));
        let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
        let vertex = e.get(data, HkPackedNiTriStripsData::paVertData) + vertex_index * 12;
        for lane in 0..3 {
            let bits = x87_quiet_bits(e.mem.u32(vertex + 4 * lane));
            e.mem.set_u32(out + 0x20 + 0x10 * corner + 4 * lane, bits);
        }
        e.mem.set_u32(out + 0x20 + 0x10 * corner + 0xc, 0);
    }
    let scale = load_vec(e, shape.addr() + SCALE);
    if scale[0] != 1.0 {
        for corner in 0..3u32 {
            let at = out + 0x20 + 0x10 * corner;
            let vertex = load_vec(e, at);
            store_vec(e, at, zip_with(vertex, scale, |v, s| v * s));
        }
    }
    Ptr::new(out)
}

// Translated from 00ca37e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca37e0 - /GS stack-cookie check (00ec408c) is not modeled
/// `hkPackedNiTriStripsShape::getAabbImpl` (`hkpShape` slot `0x1c`, Xbox PDB); stack
/// `(transform, tolerance, out)`, `out` an `hkAabb` (min then max). When the cached `WorldAabb`
/// minimum's first lane is not `FLT_MAX` (compared as a `double`, NaN counts as not equal), the
/// stored `AabbTransform` is approximately equal to `transform` (`hkTransform::isApproximatelyEqual`
/// `00cb2160`, tolerance `0.001`) and `vScale`'s first lane equals `fAabbScale` (x87 compare, NaN
/// fails), the cached box is copied. Otherwise the box starts empty (minimum `+max`, maximum `-max`
/// fills, last lane 0) and takes the lane-wise `MINPS`/`MAXPS` of every vertex of the data
/// (loaded through the x87, last lane 0, multiplied by `vScale`, moved by `transform` through
/// `hkVector4::setTransformedPos` `00c7f640`). Finally the box grows by `fRadius + tolerance` (x87,
/// rounded to `float`) on every lane.
pub fn hk_packed_ni_tri_strips_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkPackedNiTriStripsShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let me = this.addr();
    let out = out.addr();
    e.with_stack(0x30, |e, scratch| {
        let vertex = scratch.addr();
        let moved = vertex + 0x10;
        let flag = vertex + 0x20;
        let unset_limit = e.mem.f64(AABB_UNSET_LIMIT);
        let mut cached = false;
        if f64::from(e.mem.f32(me + WORLD_AABB)) != unset_limit {
            let transform_tolerance = x87_quiet_bits(e.mem.u32(TRANSFORM_TOLERANCE));
            let result = e
                .call(
                    TRANSFORM_IS_APPROXIMATELY_EQUAL,
                    &args![me + AABB_TRANSFORM, flag, transform, transform_tolerance],
                )
                .u32();
            if e.mem.u8(result) != 0 {
                let scale = f64::from(e.mem.f32(me + SCALE));
                let aabb_scale = f64::from(e.get(this, HkPackedNiTriStripsShape::fAabbScale));
                cached = scale == aabb_scale;
            }
        }
        if cached {
            for lane in 0..8 {
                let word = e.mem.u32(me + WORLD_AABB + 4 * lane);
                e.mem.set_u32(out + 4 * lane, word);
            }
        } else {
            let (min_fill, max_fill, w_fill) = (
                e.mem.u32(EMPTY_MIN_FILL),
                e.mem.u32(EMPTY_MAX_FILL),
                e.mem.u32(EMPTY_W_FILL),
            );
            for (lane, word) in [min_fill, min_fill, min_fill, w_fill]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(out + 4 * lane as u32, word);
            }
            for (lane, word) in [max_fill, max_fill, max_fill, w_fill]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(out + 0x10 + 4 * lane as u32, word);
            }
            let scale = load_vec(e, me + SCALE);
            let mut data = e.get(this, HkPackedNiTriStripsShape::spTriStripsData);
            if e.get(data, HkPackedNiTriStripsData::uiVertCount) != 0 {
                let mut offset = 0u32;
                let mut done = 0u32;
                loop {
                    let source = e.get(data, HkPackedNiTriStripsData::paVertData) + offset;
                    let raw: Vec4 = [
                        x87_quiet(e.mem.f32(source)),
                        x87_quiet(e.mem.f32(source + 4)),
                        x87_quiet(e.mem.f32(source + 8)),
                        0.0,
                    ];
                    store_vec(e, vertex, zip_with(scale, raw, |s, v| s * v));
                    e.call(SET_TRANSFORMED_POS, &args![moved, transform, vertex]);
                    let corner = load_vec(e, moved);
                    let low = zip_with(load_vec(e, out), corner, minps);
                    store_vec(e, out, low);
                    let high = zip_with(load_vec(e, out + 0x10), corner, maxps);
                    store_vec(e, out + 0x10, high);
                    offset = offset.wrapping_add(0xc);
                    data = e.get(this, HkPackedNiTriStripsShape::spTriStripsData);
                    done += 1;
                    if done >= e.get(data, HkPackedNiTriStripsData::uiVertCount) {
                        break;
                    }
                }
            }
        }
        let radius = x87_quiet(e.get(this, HkPackedNiTriStripsShape::fRadius));
        let margin = (f64::from(radius) + f64::from(x87_quiet(tolerance))) as f32;
        let low = load_vec(e, out).map(|lane| lane - margin);
        store_vec(e, out, low);
        let high = load_vec(e, out + 0x10).map(|lane| lane + margin);
        store_vec(e, out + 0x10, high);
    });
}

// Translated from 00ca34b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_82bc.
pub fn hk_packed_ni_tri_strips_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_82bc
}

/// The product of two words, or `0xffffffff` when it does not fit in 32 bits (`MUL`, `SETO`,
/// `NEG`, `OR`: the size a failed multiplication hands to the allocator).
fn product_or_max(a: u32, b: u32) -> u32 {
    let wide = u64::from(a) * u64::from(b);
    if wide >> 32 != 0 {
        u32::MAX
    } else {
        wide as u32
    }
}

/// Calls the transfer function the stream's reader or writer object `object` keeps at
/// `object + function_offset` with `(object, buffer, size, format, flag)`, `format` being the
/// address of a format word, and adds the answer to the object's position (`+4`).
fn transfer_with_format(
    e: &mut Engine,
    object: u32,
    function_offset: u32,
    buffer: u32,
    size: u32,
    format: u32,
    flag: u32,
) {
    let function = e.mem.u32(object + function_offset);
    let moved = e
        .call(function, &args![object, buffer, size, format, flag])
        .u32();
    let position = e.mem.u32(object + 4);
    e.mem.set_u32(object + 4, position.wrapping_add(moved));
}

/// Resets the cached box of the shape (`+0x60` minimum, `+0x70` maximum) to the empty box: the
/// first three lanes of the minimum are the float at 0x0104da48, those of the maximum the float
/// at 0x0104e148, and the last lane of both the float at 0x01011d78.
fn reset_world_box(e: &mut Engine, shape: u32) {
    let min_fill = e.mem.f32(EMPTY_MIN_FILL);
    let max_fill = e.mem.f32(EMPTY_MAX_FILL);
    let w_fill = e.mem.f32(EMPTY_W_FILL);
    store_vec(
        e,
        shape + WORLD_AABB,
        [min_fill, min_fill, min_fill, w_fill],
    );
    store_vec(
        e,
        shape + WORLD_AABB + 0x10,
        [max_fill, max_fill, max_fill, w_fill],
    );
}

// Translated from 00ca34c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca34c0 - the FS:[0] exception frame is not modeled.
/// `fn_00ca34c0` (unnamed in the Xbox PDB): takes no arguments. Allocates 0x30 bytes from the
/// game heap (`00aa13e0`); when that gave a block, runs the data object's base constructor
/// (`00a5d3a0`) on it, clears the 16-bit word at `+0x10`, stores the data vtable 0x010c740c
/// and clears the words at `+8`, `+0xC`, `+0x14`, `+0x18`, `+0x20` and `+0x24`. Answers the
/// block, or 0. It is the factory of a `hkPackedNiTriStripsData` (unconfirmed).
pub fn fn_00ca34c0(e: &mut Engine) -> u32 {
    let block = e.call(HEAP_ALLOCATE, &args![0x30u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(DATA_BASE_CONSTRUCT, &args![block]);
    e.mem.set_u16(block + 0x10, 0);
    e.mem.set_u32(block, DATA_VTABLE);
    for offset in [0x8, 0xc, 0x14, 0x18, 0x20, 0x24] {
        e.mem.set_u32(block + offset, 0);
    }
    block
}

// Translated from 00ca3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca3570` (unnamed in the Xbox PDB): the destructor body of `hkPackedNiTriStripsData`
/// (the deleting destructor `00ca3980` calls it). Stores the data vtable, gives the blocks at
/// `+0x14`, `+0x18`, `+0x24` and `+0x20` back to the memory manager (`Deallocate`, `00aa4060`)
/// when they are set, and ends with the base destructor (`00a5d3d0`).
pub fn fn_00ca3570(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, DATA_VTABLE);
    for offset in [0x14, 0x18, 0x24, 0x20] {
        let block = e.mem.u32(this + offset);
        if block != 0 {
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
        }
    }
    e.call(DATA_BASE_DESTRUCT, &args![this]);
}

// Translated from 00ca36a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca36a0 - the /GS cookie check (00ec408c) is not modeled.
/// `fn_00ca36a0` (unnamed in the Xbox PDB): takes the address of a 64-byte transform. Copies
/// it to `AabbTransform` (`+0x20`), stores the first lane of `vScale` (`+0x90`, through the x87
/// stack) as `fAabbScale` (`+0x80`), resets the cached box (`+0x60`, `+0x70`) to the empty box
/// and grows it over every vertex of the data (the count at `+0xC`, 12-byte vertices at
/// `+0x18`): the vertex (floats loaded through the x87, last lane 0) is multiplied lane-wise
/// by `vScale`, moved by the transform through `hkVector4::setTransformedPos` (`00c7f640`) and
/// folded in with `MINPS`/`MAXPS`.
pub fn fn_00ca36a0(e: &mut Engine, this: u32, transform: u32) {
    for block in 0..4 {
        let lanes = load_vec(e, transform + 0x10 * block);
        store_vec(e, this + AABB_TRANSFORM + 0x10 * block, lanes);
    }
    let first_scale = x87_quiet_bits(e.mem.u32(this + SCALE));
    e.mem.set_u32(this + 0x80, first_scale);
    reset_world_box(e, this);
    let data = e.mem.u32(this + 0x84);
    if e.mem.u32(data + 0xc) == 0 {
        return;
    }
    e.with_stack(0x20, |e, scratch| {
        let vertex = scratch.addr();
        let moved = vertex + 0x10;
        let mut offset = 0u32;
        let mut done = 0u32;
        loop {
            let data = e.mem.u32(this + 0x84);
            let source = e.mem.u32(data + 0x18).wrapping_add(offset);
            let scale = load_vec(e, this + SCALE);
            let raw: Vec4 = [
                x87_quiet(e.mem.f32(source)),
                x87_quiet(e.mem.f32(source + 4)),
                x87_quiet(e.mem.f32(source + 8)),
                0.0,
            ];
            store_vec(e, vertex, zip_with(scale, raw, |s, v| s * v));
            e.call(SET_TRANSFORMED_POS, &args![moved, transform, vertex]);
            let corner = load_vec(e, moved);
            offset = offset.wrapping_add(0xc);
            let low = zip_with(load_vec(e, this + WORLD_AABB), corner, minps);
            store_vec(e, this + WORLD_AABB, low);
            let high = zip_with(load_vec(e, this + WORLD_AABB + 0x10), corner, maxps);
            store_vec(e, this + WORLD_AABB + 0x10, high);
            done += 1;
            let data = e.mem.u32(this + 0x84);
            if done >= e.mem.u32(data + 0xc) {
                break;
            }
        }
    });
}

// Translated from 00ca39f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca39f0 - the FS:[0] exception frame is not modeled.
/// `hkPackedNiTriStripsShape::hkPackedNiTriStripsShape` (Xbox PDB): takes the construction
/// info. Runs the shape base constructor (`00d21410`) on `(this, 0x1f, 3)`, stores the shape
/// and container vtables, takes a counted reference to the strips data the info holds at `+4`
/// (stored at `+0x84`, count raised through `InterlockedIncrement`), copies the float at info
/// `+8` (through the x87 stack) to `fRadius` (`+0xA0`) and the vector at info `+0x10` to
/// `vScale` (`+0x90`), resets the cached box and sets `m_disableWelding` (`+0x14`) to 1.
/// Answers `this`.
pub fn hk_packed_ni_tri_strips_shape_hk_packed_ni_tri_strips_shape(
    e: &mut Engine,
    this: u32,
    info: u32,
) -> u32 {
    e.call(SHAPE_COLLECTION_CONSTRUCT, &args![this, 0x1fu32, 3u32]);
    e.mem.set_u32(this, SHAPE_VTABLE);
    e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
    e.mem.set_u32(this + 0x84, 0);
    let data = e.mem.u32(info + 4);
    if data != 0 {
        e.mem.set_u32(this + 0x84, data);
        e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
    }
    let radius = x87_quiet_bits(e.mem.u32(info + 8));
    e.mem.set_u32(this + 0xa0, radius);
    let scale = load_vec(e, info + 0x10);
    store_vec(e, this + SCALE, scale);
    reset_world_box(e, this);
    e.mem.set_u8(this + 0x14, 1);
    this
}

// Translated from 00ca3b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca3b00` (unnamed in the Xbox PDB): a thunk that moves `this` back by 0x10 (from the
/// container sub-object to the shape) and jumps to the deleting destructor `00ca40f0` with the
/// same stack word. Answers what that answers.
pub fn fn_00ca3b00(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.call(0x00ca_40f0, &args![this.wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00ca3b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::CopyCinfo` (Xbox PDB): takes the construction info. Copies the
/// radius (`+0xA0`, through the x87 stack) to info `+8` and `vScale` (`+0x90`) to info `+0x10`;
/// when the info's strips data (`+4`) is not the shape's (`+0x84`), releases the info's
/// (decrement of its count at `+4`, virtual slot 4 when it reaches 0) and takes a counted
/// reference to the shape's.
pub fn hk_packed_ni_tri_strips_shape_copy_cinfo(e: &mut Engine, this: u32, info: u32) {
    let radius = x87_quiet_bits(e.mem.u32(this + 0xa0));
    e.mem.set_u32(info + 8, radius);
    let scale = load_vec(e, this + SCALE);
    store_vec(e, info + 0x10, scale);
    let old = e.mem.u32(info + 4);
    if old != e.mem.u32(this + 0x84) {
        if old != 0 {
            release_ni_object(e, old);
        }
        let data = e.mem.u32(this + 0x84);
        e.mem.set_u32(info + 4, data);
        if data != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![data + 4]);
        }
    }
}

// Translated from 00ca3b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca3b90 - the FS:[0] exception frame is not modeled.
/// `fn_00ca3b90` (unnamed in the Xbox PDB): the destructor body of `hkPackedNiTriStripsShape`
/// (the deleting destructor `00ca40f0` calls it). Stores the shape and container vtables,
/// releases the strips data (`+0x84`: decrement of its count at `+4`, virtual slot 4 when it
/// reaches 0) and clears the pointer, releases whatever the pointer then holds (nothing, as
/// the code is written), and ends with the vtables 0x010c6fb0 at `+0x10` and 0x0102e388 at `+0`.
pub fn fn_00ca3b90(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SHAPE_VTABLE);
    e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
    let data = e.mem.u32(this + 0x84);
    if data != 0 {
        release_ni_object(e, data);
        e.mem.set_u32(this + 0x84, 0);
    }
    let data = e.mem.u32(this + 0x84);
    if data != 0 {
        release_ni_object(e, data);
    }
    e.mem.set_u32(this + 0x10, BASE_ENDED_VTABLE);
    e.mem.set_u32(this, SHAPE_ENDED_VTABLE);
}

// Translated from 00ca3dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsData::SaveBinary` (Xbox PDB): takes the stream. Every piece goes
/// through the transfer function of the writer object (`stream + 0x250`, function at `+0xC`),
/// reloaded each time: the triangle count (`+8`), the triangle data (`+0x14`, 8 bytes each),
/// the vertex count (`+0xC`), a byte telling whether the compressed vertices (`+0x20`) are
/// set, the vertices (6 bytes each from `+0x20` when set, else 12 bytes each from `+0x18`), the
/// part count (16 bits at `+0x10`) and the parts (`+0x24`, 12 bytes each).
pub fn hk_packed_ni_tri_strips_data_save_binary(e: &mut Engine, this: u32, stream: u32) {
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    stream_transfer(e, writer, WRITER_FUNCTION, this + 8, 4, Some(4), 1);
    let triangle_count = e.mem.u32(this + 8);
    let triangles = e.mem.u32(this + 0x14);
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    transfer_with_format(
        e,
        writer,
        WRITER_FUNCTION,
        triangles,
        triangle_count.wrapping_mul(8),
        WRITE_TRIANGLE_FORMAT,
        4,
    );
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    stream_transfer(e, writer, WRITER_FUNCTION, this + 0xc, 4, Some(4), 1);
    let compressed = e.mem.u32(this + 0x20) != 0;
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), u8::from(compressed));
        stream_transfer(e, writer, WRITER_FUNCTION, flag.addr(), 1, Some(1), 1);
    });
    let vertex_count = e.mem.u32(this + 0xc);
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    if compressed {
        let vertices = e.mem.u32(this + 0x20);
        stream_transfer(
            e,
            writer,
            WRITER_FUNCTION,
            vertices,
            vertex_count.wrapping_mul(6),
            Some(2),
            1,
        );
    } else {
        let vertices = e.mem.u32(this + 0x18);
        transfer_with_format(
            e,
            writer,
            WRITER_FUNCTION,
            vertices,
            vertex_count.wrapping_mul(12),
            WRITE_VERTEX_FORMAT,
            1,
        );
    }
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    stream_transfer(e, writer, WRITER_FUNCTION, this + 0x10, 2, Some(2), 1);
    let part_count = u32::from(e.mem.u16(this + 0x10));
    let parts = e.mem.u32(this + 0x24);
    let writer = e.mem.u32(stream + WRITER_OBJECT);
    transfer_with_format(
        e,
        writer,
        WRITER_FUNCTION,
        parts,
        part_count.wrapping_mul(12),
        WRITE_VERTEX_FORMAT,
        1,
    );
}

// Translated from 00ca3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsData::LoadBinary` (Xbox PDB): takes the stream. The mirror of
/// `SaveBinary`, through the transfer function of the reader object (`stream + 0x24c`,
/// function at `+8`), reloaded each time. Reads the triangle count (`+8`) and allocates the
/// triangle data (`+0x14`) from the memory manager (`00aa3e40`, `count * 8` bytes, all ones
/// when that overflows). A stream version (`stream + 4`) below 0x21 holds 20-byte records of
/// which the first two words are kept as the 8-byte entries; a later one holds the entries.
/// Then the vertex count (`+0xC`), a byte that is read and dropped for versions from 0x22, the
/// vertex data (`+0x18`, 12 bytes each, allocated like the triangles), the part count (16 bits
/// at `+0x10`) and the parts (`+0x24`, 12 bytes each). The compressed vertices (`+0x20`) are
/// not read.
pub fn hk_packed_ni_tri_strips_data_load_binary(e: &mut Engine, this: u32, stream: u32) {
    let reader = e.mem.u32(stream + READER_OBJECT);
    stream_transfer(e, reader, READER_FUNCTION, this + 8, 4, Some(4), 1);
    let triangle_count = e.mem.u32(this + 8);
    let triangles = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, product_or_max(triangle_count, 8)],
        )
        .u32();
    e.mem.set_u32(this + 0x14, triangles);
    if e.mem.u32(stream + 4) < 0x21 {
        if e.mem.u32(this + 8) != 0 {
            let mut index = 0u32;
            loop {
                let reader = e.mem.u32(stream + READER_OBJECT);
                let (first, second) = e.with_stack(0x14, |e, record| {
                    transfer_with_format(
                        e,
                        reader,
                        READER_FUNCTION,
                        record.addr(),
                        0x14,
                        READ_TRIANGLE_FORMAT,
                        7,
                    );
                    (e.mem.u32(record.addr()), e.mem.u32(record.addr() + 4))
                });
                let entry = e.mem.u32(this + 0x14).wrapping_add(index.wrapping_mul(8));
                e.mem.set_u32(entry, first);
                index = index.wrapping_add(1);
                e.mem.set_u32(entry + 4, second);
                if index >= e.mem.u32(this + 8) {
                    break;
                }
            }
        }
    } else {
        let triangle_bytes = e.mem.u32(this + 8).wrapping_mul(8);
        let reader = e.mem.u32(stream + READER_OBJECT);
        transfer_with_format(
            e,
            reader,
            READER_FUNCTION,
            triangles,
            triangle_bytes,
            READ_TRIANGLE_FORMAT,
            4,
        );
    }
    let reader = e.mem.u32(stream + READER_OBJECT);
    stream_transfer(e, reader, READER_FUNCTION, this + 0xc, 4, Some(4), 1);
    if e.mem.u32(stream + 4) >= 0x22 {
        let reader = e.mem.u32(stream + READER_OBJECT);
        e.with_stack(4, |e, dropped| {
            stream_transfer(e, reader, READER_FUNCTION, dropped.addr(), 1, Some(1), 1);
        });
    }
    let vertex_count = e.mem.u32(this + 0xc);
    let vertices = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, product_or_max(vertex_count, 12)],
        )
        .u32();
    e.mem.set_u32(this + 0x18, vertices);
    let vertex_bytes = e.mem.u32(this + 0xc).wrapping_mul(12);
    let reader = e.mem.u32(stream + READER_OBJECT);
    transfer_with_format(
        e,
        reader,
        READER_FUNCTION,
        vertices,
        vertex_bytes,
        READ_VERTEX_FORMAT,
        1,
    );
    let reader = e.mem.u32(stream + READER_OBJECT);
    stream_transfer(e, reader, READER_FUNCTION, this + 0x10, 2, Some(2), 1);
    let part_count = u32::from(e.mem.u16(this + 0x10));
    let parts = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER, product_or_max(part_count, 12)],
        )
        .u32();
    e.mem.set_u32(this + 0x24, parts);
    let reader = e.mem.u32(stream + READER_OBJECT);
    transfer_with_format(
        e,
        reader,
        READER_FUNCTION,
        parts,
        part_count.wrapping_mul(12),
        READ_VERTEX_FORMAT,
        1,
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca34c0, fn_00ca34c0() -> u32),
        entry!(
            0x00ca_35f5,
            fn_00ca35f5(Ptr<HkPackedNiTriStripsData>, u32, u32) -> u32
        ),
        entry!(0x00ca3570, fn_00ca3570(u32)),
        entry!(0x00ca36a0, fn_00ca36a0(u32, u32)),
        entry!(
            0x00ca39f0,
            hk_packed_ni_tri_strips_shape_hk_packed_ni_tri_strips_shape(u32, u32) -> u32
        ),
        entry!(0x00ca3b00, fn_00ca3b00(u32, u32) -> u32),
        entry!(
            0x00ca3b20,
            hk_packed_ni_tri_strips_shape_copy_cinfo(u32, u32)
        ),
        entry!(0x00ca3b90, fn_00ca3b90(u32)),
        entry!(
            0x00ca3dc0,
            hk_packed_ni_tri_strips_data_save_binary(u32, u32)
        ),
        entry!(
            0x00ca3f10,
            hk_packed_ni_tri_strips_data_load_binary(u32, u32)
        ),
        entry!(0x00ca34b0, hk_packed_ni_tri_strips_data_get_r_t_t_i() -> u32),
        entry!(0x00ca_40f0, deleting_destructor_00ca40f0(Ptr, u8) -> Ptr),
        entry!(0x00ca3980, hk_packed_ni_tri_strips_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ca3550, hk_packed_ni_tri_strips_shape_get_type(Ptr) -> u32),
        entry!(0x00ca35d0, hk_packed_ni_tri_strips_shape_get_next_key(Ptr, u32) -> u32),
        entry!(0x00ca3670, hk_packed_ni_tri_strips_shape_get_collision_filter_info(Ptr, u32) -> u32),
        entry!(
            0x00ca37e0,
            hk_packed_ni_tri_strips_shape_get_aabb_impl(
                Ptr<HkPackedNiTriStripsShape>,
                Ptr,
                f32,
                Ptr,
            )
        ),
        entry!(0x00ca39a0, hk_normal_triangle_shape_construct(Ptr, f32) -> Ptr),
        entry!(0x00ca3b10, hk_packed_ni_tri_strips_shape_is_welding_enabled(Ptr) -> bool),
        entry!(0x00ca3c40, hk_packed_ni_tri_strips_shape_get_child_shape(Ptr, u32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00ca40f0_frees_only_when_bit_zero_is_set() {
        check(0x00ca_40f0, 0x00ca_3b90);
    }

    #[test]
    fn hk_packed_ni_tri_strips_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_3570, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ca3980, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ca3980, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00ca_3570), vec![vec![this]; 2]);
    }

    /// A shape (0xb0 bytes) pointing at a data block (0x30 bytes), both zeroed.
    fn shape_with_data(e: &mut Engine) -> (u32, u32) {
        let shape = e.mem.alloc(0xb0);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(shape + 0x84, data);
        (shape, data)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_type_is_0x17() {
        let mut e = engine();
        assert_eq!(e.call(0x00ca_3550, &args![0x1234u32]).u32(), 0x17);
    }

    #[test]
    fn is_welding_enabled_reads_the_disable_byte_of_the_container() {
        let mut e = engine();
        let (shape, _) = shape_with_data(&mut e);
        let container = shape + 0x10;
        assert!(e.call(0x00ca_3b10, &args![container]).bool());
        e.mem.set_u8(shape + 0x14, 1);
        assert!(!e.call(0x00ca_3b10, &args![container]).bool());
    }

    #[test]
    fn get_collision_filter_info_picks_the_sub_part_or_zero() {
        let mut e = engine();
        let (shape, data) = shape_with_data(&mut e);
        let sub_parts = e.mem.alloc(0x24);
        e.mem.set_u16(data + 0x10, 2);
        e.mem.set_u32(data + 0x24, sub_parts);
        e.mem.set_u32(sub_parts, 0x11);
        e.mem.set_u32(sub_parts + 0xc, 0x22);
        let container = shape + 0x10;
        let filter = |e: &mut Engine, key: u32| e.call(0x00ca_3670, &args![container, key]).u32();
        assert_eq!(filter(&mut e, 0x0000_0007), 0x11);
        assert_eq!(filter(&mut e, 0x0100_0005), 0x22);
        assert_eq!(filter(&mut e, 0x0200_0000), 0);
        assert_eq!(filter(&mut e, 0xff00_0000), 0);
    }

    #[test]
    fn get_next_key_advances_the_sub_part_past_its_vertex_count() {
        let mut e = engine();
        let (shape, data) = shape_with_data(&mut e);
        let sub_parts = e.mem.alloc(0x24);
        let triangles = e.mem.alloc(0x50);
        e.mem.set_u32(data + 8, 10);
        e.mem.set_u16(data + 0x10, 3);
        e.mem.set_u32(data + 0x14, triangles);
        e.mem.set_u32(data + 0x24, sub_parts);
        // uiNumVerts of the three sub-parts: 3, 4 and 5.
        for (part, count) in [3u32, 4, 5].into_iter().enumerate() {
            e.mem.set_u32(sub_parts + 12 * part as u32 + 4, count);
        }
        let container = shape + 0x10;
        let next = |e: &mut Engine, key: u32, first_vertex: u16| {
            e.mem
                .set_u16(triangles + 8 * ((key & 0xff_ffff) + 1), first_vertex);
            e.call(0x00ca_35d0, &args![container, key]).u32()
        };
        // Sub-part 0: limit 3.
        assert_eq!(next(&mut e, 0, 2), 0x0000_0001);
        assert_eq!(next(&mut e, 0, 3), 0x0100_0001);
        // Sub-part 1: limit 3 + 4.
        assert_eq!(next(&mut e, 0x0100_0004, 6), 0x0100_0005);
        assert_eq!(next(&mut e, 0x0100_0004, 7), 0x0200_0005);
        // Sub-part 2 (a pair and a single one): limit 3 + 4 + 5.
        assert_eq!(next(&mut e, 0x0200_0000, 11), 0x0200_0001);
        assert_eq!(next(&mut e, 0x0200_0000, 12), 0x0300_0001);
        // Past the last triangle.
        assert_eq!(
            e.call(0x00ca_35d0, &args![container, 9u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(
            e.call(0x00ca_35d0, &args![container, 0x0500_0009u32]).u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn triangle_constructor_fills_the_header_and_zeroes_the_extrusion() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        let got = e.call(0x00ca_39a0, &args![this, 0.5f32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_74ec);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.f32(this + 0x10), 0.5);
        assert_eq!(e.mem.u16(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x16), 6);
        assert_eq!(e.mem.u8(this + 0x17), 0);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(this + 0x50 + 4 * lane), 0);
        }
        // The vertices and the words in between are not touched.
        assert_eq!(e.mem.u32(this + 0x20), 0xffff_ffff);
        // A signalling NaN radius is quieted by the x87 copy.
        e.call(0x00ca_39a0, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
    }

    /// A shape with two triangles, three vertices, radius 0.25 and the given scale.
    fn shape_with_triangles(e: &mut Engine, scale: [f32; 4]) -> u32 {
        let (shape, data) = shape_with_data(e);
        let triangles = e.mem.alloc(0x10);
        let vertices = e.mem.alloc(0x24);
        e.mem.set_u32(data + 8, 2);
        e.mem.set_u32(data + 0x0c, 3);
        e.mem.set_u32(data + 0x14, triangles);
        e.mem.set_u32(data + 0x18, vertices);
        for (slot, vertex) in [0u16, 1, 2, 2, 0, 1].into_iter().enumerate() {
            e.mem
                .set_u16(triangles + 2 * slot as u32 + 2 * (slot as u32 / 3), vertex);
        }
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vertices + 4 * lane as u32, value);
        }
        e.mem.set_f32(shape + 0xa0, 0.25);
        for (lane, value) in scale.into_iter().enumerate() {
            e.mem.set_f32(shape + 0x90 + 4 * lane as u32, value);
        }
        shape
    }

    fn read_vertices(e: &Engine, buffer: u32) -> Vec<f32> {
        (0..3u32)
            .flat_map(|corner| (0..4u32).map(move |lane| (corner, lane)))
            .map(|(corner, lane)| e.mem.f32(buffer + 0x20 + 0x10 * corner + 4 * lane))
            .collect()
    }

    #[test]
    fn get_child_shape_builds_the_triangle_and_scales_only_when_needed() {
        let mut e = engine();
        let shape = shape_with_triangles(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let container = shape + 0x10;
        let buffer = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(buffer + 4 * word, 0xffff_ffff);
        }
        let got = e.call(0x00ca_3c40, &args![container, 0u32, buffer]).u32();
        assert_eq!(got, buffer);
        assert_eq!(e.mem.u32(buffer), 0x010c_74ec);
        assert_eq!(e.mem.f32(buffer + 0x10), 0.25);
        assert_eq!(
            read_vertices(&e, buffer),
            [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0, 7.0, 8.0, 9.0, 0.0]
        );
        // The second triangle (vertices 2, 0, 1), the sub-part byte of the key is ignored.
        e.call(0x00ca_3c40, &args![container, 0x0700_0001u32, buffer]);
        assert_eq!(
            read_vertices(&e, buffer),
            [7.0, 8.0, 9.0, 0.0, 1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0]
        );
        // A scale whose first lane is not 1 multiplies all four lanes of every vertex.
        e.mem.set_f32(shape + 0x90, 2.0);
        e.mem.set_f32(shape + 0x94, 3.0);
        e.mem.set_f32(shape + 0x98, 4.0);
        e.mem.set_f32(shape + 0x9c, 5.0);
        e.call(0x00ca_3c40, &args![container, 0u32, buffer]);
        assert_eq!(
            read_vertices(&e, buffer),
            [2.0, 6.0, 12.0, 0.0, 8.0, 15.0, 24.0, 0.0, 14.0, 24.0, 36.0, 0.0]
        );
    }

    #[test]
    fn get_child_shape_answers_null_past_the_last_triangle() {
        let mut e = engine();
        let shape = shape_with_triangles(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let buffer = e.mem.alloc(0x60);
        e.mem.set_u32(buffer, 0xdead_beef);
        let got = e
            .call(0x00ca_3c40, &args![shape + 0x10, 2u32, buffer])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(buffer), 0xdead_beef);
    }

    const FLT_MAX_FILL: u32 = 0x7f7f_ffee;

    /// Constants the AABB function reads, the two callee doubles and a shape with a cached box
    /// `(1, 2, 3, 4)` to `(5, 6, 7, 8)`, `AabbTransform` scale 2 and the given vertices.
    fn aabb_setup(e: &mut Engine, vertices: &[[f32; 3]], flag: u8) -> (u32, u32) {
        e.call_log = Some(vec![]);
        e.mem.set_f64(0x0104_e138, f64::from(f32::MAX));
        e.mem.set_u32(0x0101_7d00, 0x3a83_126f);
        e.mem.set_u32(0x0104_da48, FLT_MAX_FILL);
        e.mem.set_u32(0x0104_e148, 0xff7f_ffee);
        e.mem.set_u32(0x0101_1d78, 0);
        e.register_double(0x00cb_2160, move |e, a| {
            e.mem.set_u8(a[1], flag);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The "transform" of the test moves every point by 100 along x.
        e.register(0x00c7_f640, |e, a| {
            for lane in 0..4 {
                let value = e.mem.f32(a[2] + 4 * lane);
                let moved = if lane == 0 { value + 100.0 } else { value };
                e.mem.set_f32(a[0] + 4 * lane, moved);
            }
            Ret::default()
        });
        let (shape, data) = shape_with_data(e);
        let list = e.mem.alloc(0x40);
        for (index, vertex) in vertices.iter().enumerate() {
            for (lane, value) in vertex.iter().enumerate() {
                e.mem
                    .set_f32(list + 12 * index as u32 + 4 * lane as u32, *value);
            }
        }
        e.mem.set_u32(data + 0x0c, vertices.len() as u32);
        e.mem.set_u32(data + 0x18, list);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(shape + 0x60 + 4 * lane as u32, value);
        }
        e.mem.set_f32(shape + 0x80, 2.0);
        for lane in 0..4 {
            e.mem.set_f32(shape + 0x90 + 4 * lane, 2.0);
        }
        e.mem.set_f32(shape + 0xa0, 0.25);
        let transform = e.mem.alloc(0x40);
        (shape, transform)
    }

    fn read_box(e: &Engine, out: u32) -> [f32; 8] {
        std::array::from_fn(|lane| e.mem.f32(out + 4 * lane as u32))
    }

    #[test]
    fn get_aabb_copies_the_cached_box_when_transform_and_scale_match() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert_eq!(
            read_box(&e, out),
            [0.25, 1.25, 2.25, 3.25, 5.75, 6.75, 7.75, 8.75]
        );
        let checks = calls_to(&e, 0x00cb_2160);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][0], shape + 0x20);
        assert_eq!(checks[0][2..], [transform, 0x3a83_126f]);
        assert!(calls_to(&e, 0x00c7_f640).is_empty());
    }

    #[test]
    fn get_aabb_rebuilds_from_the_vertices_when_the_transforms_differ() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0], [-4.0, 5.0, 0.0]], 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        // Scaled by 2 and moved by 100 along x: (102, 4, 6, 0) and (92, 10, 0, 0).
        assert_eq!(
            read_box(&e, out),
            [91.25, 3.25, -0.75, -0.75, 102.75, 10.75, 6.75, 0.75]
        );
        assert_eq!(calls_to(&e, 0x00c7_f640).len(), 2);
    }

    #[test]
    fn get_aabb_skips_the_cache_check_when_the_minimum_is_float_max() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        e.mem.set_f32(shape + 0x60, f32::MAX);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert!(calls_to(&e, 0x00cb_2160).is_empty());
        assert_eq!(
            read_box(&e, out),
            [101.25, 3.25, 5.25, -0.75, 102.75, 4.75, 6.75, 0.75]
        );
    }

    #[test]
    fn get_aabb_rebuilds_when_the_scale_differs_from_the_cached_one() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        e.mem.set_f32(shape + 0x80, 3.0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert_eq!(calls_to(&e, 0x00c7_f640).len(), 1);
        assert_eq!(read_box(&e, out)[0], 101.25);
    }

    #[test]
    fn get_aabb_without_vertices_keeps_the_empty_fills_grown_by_the_margin() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[], 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        let got = read_box(&e, out);
        assert_eq!(got[0].to_bits(), FLT_MAX_FILL);
        assert_eq!(got[3], -0.75);
        assert_eq!(got[4].to_bits(), 0xff7f_ffee);
        assert_eq!(got[7], 0.75);
        assert!(calls_to(&e, 0x00c7_f640).is_empty());
    }

    #[test]
    fn hk_packed_ni_tri_strips_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ca_34b0, &args![]).u32(), 0x0126_82bc);
    }
}

#[cfg(test)]
mod data_tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{
        calls_to, interlocked_doubles, logging_engine, ni_object,
    };
    use crate::units::bshavok::bhkentity::INTERLOCKED_DECREMENT;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The double that stands for the transfer function of the stream objects.
    const TRANSFER_DOUBLE: u32 = 0x0900_0c00;
    /// The double that stands for virtual slot 4 (the delete) of the data objects.
    const DELETE_SLOT: u32 = 0x0900_0004;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the interlocked imports, the delete slot and the manager doubled (the
    /// allocator answers zeroed blocks).
    fn engine() -> Engine {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(DELETE_SLOT, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        e.register(DATA_BASE_CONSTRUCT, nothing);
        e.register(DATA_BASE_DESTRUCT, nothing);
        e.register(HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1].max(1)),
            ..Ret::default()
        });
        e
    }

    /// A stream whose reader and writer are the same object, with its transfer function
    /// at the double; answers `(stream, object)`.
    fn stream(e: &mut Engine, version: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + READER_FUNCTION, TRANSFER_DOUBLE);
        e.mem.set_u32(object + WRITER_FUNCTION, TRANSFER_DOUBLE);
        e.mem.set_u32(stream + 4, version);
        e.mem.set_u32(stream + READER_OBJECT, object);
        e.mem.set_u32(stream + WRITER_OBJECT, object);
        (stream, object)
    }

    /// Installs the reader double: every transfer fills the buffer with the next chunk.
    fn feed(e: &mut Engine, chunks: Vec<Vec<u8>>) {
        let queue = Rc::new(RefCell::new(VecDeque::from(chunks)));
        e.register_double(TRANSFER_DOUBLE, move |e, a| {
            let chunk = queue.borrow_mut().pop_front().unwrap_or_default();
            for (i, byte) in chunk.iter().enumerate().take(a[2] as usize) {
                e.mem.set_u8(a[1] + i as u32, *byte);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
    }

    /// Installs the writer double: every transfer records the bytes of its buffer.
    fn capture(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u8>>>> {
        let written = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&written);
        e.register_double(TRANSFER_DOUBLE, move |e, a| {
            let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            sink.borrow_mut().push(bytes);
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        written
    }

    fn word(value: u32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    fn sizes(e: &Engine) -> Vec<u32> {
        calls_to(e, TRANSFER_DOUBLE).iter().map(|c| c[2]).collect()
    }

    #[test]
    fn the_factory_builds_a_cleared_data_object() {
        let mut e = engine();
        let block = e.call(0x00ca_34c0, &args![]).u32();
        assert_ne!(block, 0);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), [[0x30]]);
        assert_eq!(calls_to(&e, DATA_BASE_CONSTRUCT), [[block]]);
        assert_eq!(e.mem.u32(block), DATA_VTABLE);
        for offset in [0x8, 0xc, 0x14, 0x18, 0x20, 0x24] {
            assert_eq!(e.mem.u32(block + offset), 0);
        }
    }

    #[test]
    fn the_factory_answers_zero_when_the_heap_does() {
        let mut e = engine();
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00ca_34c0, &args![]).u32(), 0);
        assert!(calls_to(&e, DATA_BASE_CONSTRUCT).is_empty());
    }

    #[test]
    fn the_data_destructor_frees_the_set_blocks_in_order() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, 0x100);
        e.mem.set_u32(this + 0x24, 0x300);
        e.mem.set_u32(this + 0x20, 0x400);
        e.call(0x00ca_3570, &args![this]);
        assert_eq!(e.mem.u32(this), DATA_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [
                [MEMORY_MANAGER, 0x100],
                [MEMORY_MANAGER, 0x300],
                [MEMORY_MANAGER, 0x400]
            ]
        );
        assert_eq!(calls_to(&e, DATA_BASE_DESTRUCT), [[this]]);
    }

    #[test]
    fn the_thunk_moves_this_back_to_the_shape() {
        let mut e = engine();
        e.register(0x00ca_40f0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00ca_3b00, &args![0x5010u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, 0x00ca_40f0), [[0x5000, 1]]);
    }

    /// The constants the box reset reads.
    fn box_constants(e: &mut Engine) {
        for (address, value) in [
            (EMPTY_MIN_FILL, 100.0f32),
            (EMPTY_MAX_FILL, -100.0),
            (EMPTY_W_FILL, 0.0),
        ] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
    }

    #[test]
    fn the_shape_constructor_takes_the_data_and_resets_the_box() {
        let mut e = engine();
        box_constants(&mut e);
        e.register(SHAPE_COLLECTION_CONSTRUCT, nothing);
        let data = ni_object(&mut e, 1, DELETE_SLOT);
        let info = e.mem.alloc(0x30);
        e.mem.set_u32(info + 4, data);
        e.mem.set_u32(info + 8, 0x7f80_0001);
        for lane in 0..4 {
            e.mem.set_f32(info + 0x10 + 4 * lane, 1.0 + lane as f32);
        }
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x84, 0x1234);
        assert_eq!(e.call(0x00ca_39f0, &args![this, info]).u32(), this);
        assert_eq!(calls_to(&e, SHAPE_COLLECTION_CONSTRUCT), [[this, 0x1f, 3]]);
        assert_eq!(e.mem.u32(this), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), CONTAINER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x84), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert_eq!(e.mem.u32(this + 0xa0), 0x7fc0_0001);
        assert_eq!(load_vec(&e, this + 0x90), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, this + 0x60), [100.0, 100.0, 100.0, 0.0]);
        assert_eq!(load_vec(&e, this + 0x70), [-100.0, -100.0, -100.0, 0.0]);
        assert_eq!(e.mem.u8(this + 0x14), 1);
    }

    #[test]
    fn the_shape_constructor_without_data_keeps_the_pointer_empty() {
        let mut e = engine();
        box_constants(&mut e);
        e.register(SHAPE_COLLECTION_CONSTRUCT, nothing);
        let info = e.mem.alloc(0x30);
        let this = e.mem.alloc(0xb0);
        e.call(0x00ca_39f0, &args![this, info]);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn copy_cinfo_swaps_the_data_and_copies_the_values() {
        let mut e = engine();
        let shape_data = ni_object(&mut e, 3, DELETE_SLOT);
        let info_data = ni_object(&mut e, 1, DELETE_SLOT);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x84, shape_data);
        e.mem.set_f32(this + 0xa0, 0.25);
        for lane in 0..4 {
            e.mem.set_f32(this + 0x90 + 4 * lane, 5.0 + lane as f32);
        }
        let info = e.mem.alloc(0x30);
        e.mem.set_u32(info + 4, info_data);
        e.call(0x00ca_3b20, &args![this, info]);
        assert_eq!(e.mem.f32(info + 8), 0.25);
        assert_eq!(load_vec(&e, info + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(info + 4), shape_data);
        assert_eq!(e.mem.u32(shape_data + 4), 4);
        // The info's old data lost its only reference and was deleted.
        assert_eq!(calls_to(&e, DELETE_SLOT), [[info_data]]);
        // The same data again changes nothing.
        e.call(0x00ca_3b20, &args![this, info]);
        assert_eq!(e.mem.u32(shape_data + 4), 4);
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 1);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn the_shape_destructor_releases_the_data_and_ends_with_its_vtables() {
        let mut e = engine();
        let data = ni_object(&mut e, 2, DELETE_SLOT);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x84, data);
        e.call(0x00ca_3b90, &args![this]);
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(e.mem.u32(this + 0x84), 0);
        assert!(calls_to(&e, DELETE_SLOT).is_empty());
        assert_eq!(e.mem.u32(this + 0x10), BASE_ENDED_VTABLE);
        assert_eq!(e.mem.u32(this), SHAPE_ENDED_VTABLE);
        // Without data nothing is released.
        let bare = e.mem.alloc(0xb0);
        e.call(0x00ca_3b90, &args![bare]);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 1);
    }

    #[test]
    fn the_shape_destructor_deletes_data_with_the_last_reference() {
        let mut e = engine();
        let data = ni_object(&mut e, 1, DELETE_SLOT);
        let this = e.mem.alloc(0xb0);
        e.mem.set_u32(this + 0x84, data);
        e.call(0x00ca_3b90, &args![this]);
        assert_eq!(calls_to(&e, DELETE_SLOT), [[data]]);
    }

    #[test]
    fn product_or_max_saturates_on_overflow() {
        assert_eq!(product_or_max(3, 8), 24);
        assert_eq!(product_or_max(0x2000_0000, 8), u32::MAX);
        assert_eq!(product_or_max(0x1fff_ffff, 8), 0xffff_fff8);
    }

    /// A data object with 2 triangles, 3 vertices and 2 parts; the compressed vertices are
    /// set when `compressed`.
    fn data_object(e: &mut Engine, compressed: bool) -> u32 {
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u16(this + 0x10, 2);
        let triangles = e.mem.alloc(16);
        let vertices = e.mem.alloc(36);
        let parts = e.mem.alloc(24);
        e.mem.set_u32(this + 0x14, triangles);
        e.mem.set_u32(this + 0x18, vertices);
        e.mem.set_u32(this + 0x24, parts);
        e.mem.set_u8(triangles, 0x11);
        e.mem.set_u8(vertices, 0x22);
        e.mem.set_u8(parts, 0x33);
        if compressed {
            let packed = e.mem.alloc(18);
            e.mem.set_u8(packed, 0x44);
            e.mem.set_u32(this + 0x20, packed);
        }
        this
    }

    #[test]
    fn saving_writes_the_pieces_in_order_with_full_vertices() {
        let mut e = engine();
        let written = capture(&mut e);
        let (stream, object) = stream(&mut e, 0x22);
        let this = data_object(&mut e, false);
        e.call(0x00ca_3dc0, &args![this, stream]);
        let pieces = written.borrow();
        assert_eq!(pieces.len(), 7);
        assert_eq!(pieces[0], word(2));
        assert_eq!((pieces[1].len(), pieces[1][0]), (16, 0x11));
        assert_eq!(pieces[2], word(3));
        assert_eq!(pieces[3], vec![0]);
        assert_eq!((pieces[4].len(), pieces[4][0]), (36, 0x22));
        assert_eq!(pieces[5], vec![2, 0]);
        assert_eq!((pieces[6].len(), pieces[6][0]), (24, 0x33));
        let calls = calls_to(&e, TRANSFER_DOUBLE);
        assert_eq!(calls[1][3], WRITE_TRIANGLE_FORMAT);
        assert_eq!(calls[1][4], 4);
        assert_eq!(calls[4][3], WRITE_VERTEX_FORMAT);
        assert_eq!(calls[6][3], WRITE_VERTEX_FORMAT);
        assert!(calls.iter().all(|c| c[0] == object));
        assert_eq!(e.mem.u32(object + 4), 4 + 16 + 4 + 1 + 36 + 2 + 24);
    }

    #[test]
    fn saving_writes_the_compressed_vertices_when_set() {
        let mut e = engine();
        let written = capture(&mut e);
        let (stream, _) = stream(&mut e, 0x22);
        let this = data_object(&mut e, true);
        e.call(0x00ca_3dc0, &args![this, stream]);
        let pieces = written.borrow();
        assert_eq!(pieces[3], vec![1]);
        assert_eq!((pieces[4].len(), pieces[4][0]), (18, 0x44));
    }

    /// The records of the old format: 20 bytes, the first two words are kept.
    fn old_record(first: u32, second: u32) -> Vec<u8> {
        let mut bytes = word(first);
        bytes.extend(word(second));
        bytes.extend([0xee; 12]);
        bytes
    }

    #[test]
    fn loading_an_old_stream_keeps_the_first_words_of_each_record() {
        let mut e = engine();
        feed(
            &mut e,
            vec![
                word(2),
                old_record(0xa1, 0xa2),
                old_record(0xb1, 0xb2),
                word(3),
                vec![0x55; 36],
                vec![2, 0],
                vec![0x66; 24],
            ],
        );
        let (stream, object) = stream(&mut e, 0x20);
        let this = e.mem.alloc(0x30);
        e.call(0x00ca_3f10, &args![this, stream]);
        assert_eq!(sizes(&e), [4, 0x14, 0x14, 4, 36, 2, 24]);
        let triangles = e.mem.u32(this + 0x14);
        assert_eq!(e.mem.u32(triangles), 0xa1);
        assert_eq!(e.mem.u32(triangles + 4), 0xa2);
        assert_eq!(e.mem.u32(triangles + 8), 0xb1);
        assert_eq!(e.mem.u32(triangles + 12), 0xb2);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x18)), 0x55);
        assert_eq!(e.mem.u16(this + 0x10), 2);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x24)), 0x66);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            [
                [MEMORY_MANAGER, 16],
                [MEMORY_MANAGER, 36],
                [MEMORY_MANAGER, 24]
            ]
        );
        let calls = calls_to(&e, TRANSFER_DOUBLE);
        assert_eq!(calls[1][3], READ_TRIANGLE_FORMAT);
        assert_eq!(calls[1][4], 7);
        assert_eq!(calls[4][3], READ_VERTEX_FORMAT);
        assert_eq!(e.mem.u32(object + 4), 4 + 40 + 4 + 36 + 2 + 24);
    }

    #[test]
    fn loading_a_new_stream_reads_the_triangles_in_one_piece_and_drops_a_byte() {
        let mut e = engine();
        feed(
            &mut e,
            vec![
                word(2),
                vec![0x77; 16],
                word(3),
                vec![1],
                vec![0x55; 36],
                vec![2, 0],
                vec![0x66; 24],
            ],
        );
        let (stream, _) = stream(&mut e, 0x22);
        let this = e.mem.alloc(0x30);
        e.call(0x00ca_3f10, &args![this, stream]);
        assert_eq!(sizes(&e), [4, 16, 4, 1, 36, 2, 24]);
        let calls = calls_to(&e, TRANSFER_DOUBLE);
        assert_eq!(calls[1][1], e.mem.u32(this + 0x14));
        assert_eq!(calls[1][3], READ_TRIANGLE_FORMAT);
        assert_eq!(calls[1][4], 4);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x14)), 0x77);
    }

    #[test]
    fn loading_an_empty_old_stream_reads_no_records() {
        let mut e = engine();
        feed(&mut e, vec![word(0), word(0), vec![0, 0]]);
        let (stream, _) = stream(&mut e, 0x10);
        let this = e.mem.alloc(0x30);
        e.call(0x00ca_3f10, &args![this, stream]);
        assert_eq!(sizes(&e), [4, 4, 0, 2, 0]);
    }

    #[test]
    fn an_overflowing_count_asks_the_manager_for_all_ones() {
        let mut e = engine();
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        feed(&mut e, vec![word(0x2000_0000)]);
        let (stream, _) = stream(&mut e, 0x22);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, 0);
        // The failed allocations leave null blocks; only the first request is looked at.
        e.call(0x00ca_3f10, &args![this, stream]);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE)[0],
            [MEMORY_MANAGER, u32::MAX]
        );
    }

    /// A shape over a unit scale and the identity-like transform double: the double moves
    /// every vertex by +1 on x.
    #[test]
    fn the_transform_setter_copies_the_transform_and_grows_the_box_over_the_vertices() {
        let mut e = engine();
        box_constants(&mut e);
        e.register(SET_TRANSFORMED_POS, |e, a| {
            // Out = vertex + (1, 0, 0, 0).
            let x = e.mem.f32(a[2]) + 1.0;
            e.mem.set_f32(a[0], x);
            for lane in 1..4 {
                let value = e.mem.f32(a[2] + 4 * lane);
                e.mem.set_f32(a[0] + 4 * lane, value);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0xb0);
        let data = e.mem.alloc(0x30);
        let vertices = e.mem.alloc(36);
        e.mem.set_u32(this + 0x84, data);
        e.mem.set_u32(data + 0xc, 3);
        e.mem.set_u32(data + 0x18, vertices);
        let points = [[0.0f32, 1.0, 2.0], [-3.0, 4.0, 0.5], [2.0, -1.0, 8.0]];
        for (i, point) in points.iter().enumerate() {
            for (lane, value) in point.iter().enumerate() {
                e.mem
                    .set_f32(vertices + 12 * i as u32 + 4 * lane as u32, *value);
            }
        }
        store_vec(&mut e, this + SCALE, [2.0, 1.0, 1.0, 5.0]);
        let transform = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_f32(transform + 4 * word, word as f32);
        }
        e.call(0x00ca_36a0, &args![this, transform]);
        for word in 0..16 {
            assert_eq!(e.mem.f32(this + AABB_TRANSFORM + 4 * word), word as f32);
        }
        assert_eq!(e.mem.f32(this + 0x80), 2.0);
        // Scaled x: 0, -6, 4; moved by 1: 1, -5, 5.
        assert_eq!(load_vec(&e, this + WORLD_AABB), [-5.0, -1.0, 0.5, 0.0]);
        assert_eq!(load_vec(&e, this + WORLD_AABB + 0x10), [5.0, 4.0, 8.0, 0.0]);
        let calls = calls_to(&e, SET_TRANSFORMED_POS);
        assert_eq!(calls.len(), 3);
        assert!(calls.iter().all(|c| c[1] == transform));
    }

    #[test]
    fn the_transform_setter_with_no_vertices_leaves_the_empty_box() {
        let mut e = engine();
        box_constants(&mut e);
        e.register(SET_TRANSFORMED_POS, nothing);
        let this = e.mem.alloc(0xb0);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x84, data);
        let transform = e.mem.alloc(0x40);
        e.call(0x00ca_36a0, &args![this, transform]);
        assert!(calls_to(&e, SET_TRANSFORMED_POS).is_empty());
        assert_eq!(load_vec(&e, this + WORLD_AABB), [100.0, 100.0, 100.0, 0.0]);
        assert_eq!(
            load_vec(&e, this + WORLD_AABB + 0x10),
            [-100.0, -100.0, -100.0, 0.0]
        );
    }
}

#[cfg(test)]
mod next_key_tail_tests {
    use super::*;

    /// A data object with `uiNumVerts` 3, 4, 5 and 6 for the sub-parts 0 to 3 and the first
    /// vertex index of triangle `t` set to `10 * t`.
    fn data_object(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x30);
        let sub_parts = e.mem.alloc(4 * 12);
        for (part, count) in [3u32, 4, 5, 6].into_iter().enumerate() {
            e.mem.set_u32(sub_parts + 12 * part as u32 + 4, count);
        }
        let triangles = e.mem.alloc(8 * 8);
        for triangle in 0..8u32 {
            e.mem
                .set_u16(triangles + 8 * triangle, (10 * triangle) as u16);
        }
        e.mem.set_u32(data + 0x24, sub_parts);
        e.mem.set_u32(data + 0x14, triangles);
        data
    }

    fn next_key(e: &mut Engine, data: u32, part: u32, next: u32) -> u32 {
        e.call(0x00ca_35f5, &args![data, part, next]).u32()
    }

    #[test]
    fn fn_00ca35f5_moves_to_the_next_sub_part_once_its_vertices_are_used() {
        let mut e = Engine::new();
        let data = data_object(&mut e);
        // Part 0 holds 3 vertices: triangle 0 (vertex 0) stays, triangle 1 (vertex 10) moves on.
        assert_eq!(next_key(&mut e, data, 0, 0), 0);
        assert_eq!(next_key(&mut e, data, 0, 1), 0x0100_0001);
        // Parts 0 and 1 hold 7 (one pair, no single one): vertex 0 stays, vertex 10 moves on.
        assert_eq!(next_key(&mut e, data, 1, 0), 0x0100_0000);
        assert_eq!(next_key(&mut e, data, 1, 1), 0x0200_0001);
        // Parts 0 to 2 hold 12 (a pair and a single one): vertex 10 stays, vertex 20 moves on.
        assert_eq!(next_key(&mut e, data, 2, 1), 0x0200_0001);
        assert_eq!(next_key(&mut e, data, 2, 2), 0x0300_0002);
        // Parts 0 to 3 hold 18 (two pairs).
        assert_eq!(next_key(&mut e, data, 3, 1), 0x0300_0001);
        assert_eq!(next_key(&mut e, data, 3, 2), 0x0400_0002);
    }
}
