//! `Havok SDK/hkpsphereboxagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code is SSE: every lane is a single-precision operation, bit for bit as the exe
//! does it. The scalar parts that go through the x87 stack compute in `double` and round to
//! `float` where the exe stores. `RSQRTSS` is the host's estimate, refined the way the exe
//! refines it (`inverse_length`), so results can differ from the exe's in the last bit.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_bits, load_vec, minps, profile_record, store_bits, store_vec, sum3,
    zip_with, Vec4, PROFILE_END, SIGN_MASK,
};

/// `hkVector4::setTransformedInversePos` (`00c7f690`, `this` = the result, `(transform,
/// vector)`).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// The agent registry's register function (`00cfbbe0`, `this` = the registry, `(entry, first
/// type, second type)`).
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The profile label of the sphere-box functions, `"TtSphereBox"` (`010cf050`).
const PROFILE_SPHERE_BOX: u32 = 0x010c_f050;
/// The four-lane `0x7fffffff` mask (`01017d10`).
const ABS_MASK: u32 = 0x0101_7d10;
/// The mask that flips the sign of the first three lanes (`010c4c00`).
const NEGATE_XYZ_MASK: u32 = 0x010c_4c00;
/// The `float` the symmetric closest-point collector wrapper stores at `+4` (`0104da48`).
const SYMMETRIC_COLLECTOR_VALUE: u32 = 0x0104_da48;
/// The vtables of the symmetric wrappers' collectors for `getPenetrations` and
/// `getClosestPoints`.
const PENETRATION_COLLECTOR_VTABLE: u32 = 0x010c_e81c;
const CLOSEST_POINTS_COLLECTOR_VTABLE: u32 = 0x010c_e7fc;

/// The pairs the agent factory registers: the function table of the sphere-box agent.
const CREATE_AGENT: u32 = 0x00d3_c400;
const CREATE_BRIDGE_AGENT: u32 = 0x00d3_c380;
const SYMMETRIC_PENETRATIONS: u32 = 0x00d3_cfc0;
const SYMMETRIC_CLOSEST_POINTS: u32 = 0x00d3_d040;
const SYMMETRIC_IS_PENETRATING: u32 = 0x00d3_ac80;
const STATIC_PENETRATIONS: u32 = 0x00d3_c440;
const STATIC_CLOSEST_POINTS: u32 = 0x00d3_cc00;
const BRIDGE_IS_PENETRATING: u32 = 0x00da_f8e0;

/// What the sphere against box test finds: the contact normal (in world space) and the signed
/// distance between the two surfaces.
struct BoxHit {
    normal: [f32; 3],
    distance: f32,
}

/// The test `processCollision` and the two closest-point functions share. `a` is the sphere's
/// collidable and `b` the box's (each: shape at `+0`, transform at `+8`; the shape's radius at
/// `+0x10`, the box's half extents at `+0x20`; the transform's three axes then the translation
/// at `+0x30`). The sphere centre is taken to the box's frame; when it is outside the box the
/// normal points from the nearest point of the box to the centre (the refined reciprocal
/// length of the offset), and `None` is answered when the distance is above `max distance`,
/// the `float` at `input + 8`, read only on this path. When it is inside the box the face with
/// the largest (negative) offset gives the normal, flipped when the centre is on the negative
/// side. The distance is in both cases the length of the offset minus the two radii.
fn sphere_box_test(e: &mut Engine, a: u32, b: u32, input: u32) -> Option<BoxHit> {
    let shape_a = e.mem.u32(a);
    let transform_a = e.mem.u32(a + 8);
    let shape_b = e.mem.u32(b);
    let transform_b = e.mem.u32(b + 8);
    let centre = e.with_stack(0x10, |e, out| {
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![out, transform_b, transform_a + 0x30],
        );
        load_vec(e, out.addr())
    });
    let mask = load_bits(e, ABS_MASK);
    let absolute: Vec4 =
        std::array::from_fn(|lane| f32::from_bits(centre[lane].to_bits() & mask[lane]));
    let extents = load_vec(e, shape_b + 0x20);
    let nearest = zip_with(absolute, extents, minps);
    let offset = zip_with(nearest, absolute, |x, y| x - y);
    let outside = (0..3).any(|lane| offset[lane] < 0.0);
    let hit = if outside {
        let squares = zip_with(offset, offset, |x, y| x * y);
        let sum = sum3(squares);
        let inverse = inverse_length(e, sum);
        let length = sum * inverse;
        let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
        let radius_a = e.mem.f32(shape_a + 0x10);
        let distance = (f64::from(length) - (f64::from(radius_b) + f64::from(radius_a))) as f32;
        let max_distance = x87_quiet(e.mem.f32(input + 8));
        if max_distance < distance {
            return None;
        }
        let signs = load_bits(e, SIGN_MASK);
        let direction: [f32; 3] = std::array::from_fn(|lane| {
            let scaled = (inverse * offset[lane]).to_bits();
            f32::from_bits((scaled ^ (centre[lane].to_bits() & signs[lane])) ^ signs[lane])
        });
        let axis_x = load_vec(e, transform_b);
        let axis_y = load_vec(e, transform_b + 0x10);
        let axis_z = load_vec(e, transform_b + 0x20);
        let normal = std::array::from_fn(|lane| {
            (direction[1] * axis_y[lane] + direction[0] * axis_x[lane])
                + direction[2] * axis_z[lane]
        });
        BoxHit { normal, distance }
    } else {
        let radius_b = e.mem.f32(shape_b + 0x10);
        let radius_a = e.mem.f32(shape_a + 0x10);
        let difference: [f32; 3] = std::array::from_fn(|lane| absolute[lane] - extents[lane]);
        // The comparisons are the exe's `FCOM` / `JP` pairs: a branch is taken when the
        // left side is not below the right one, which includes unordered values.
        let (index, value) = if difference[1] < difference[0] {
            if difference[2] < difference[0] {
                (0, difference[0])
            } else {
                (2, difference[2])
            }
        } else if difference[2] < difference[1] {
            (1, difference[1])
        } else {
            (2, difference[2])
        };
        let axis = load_vec(e, transform_b + 0x10 * index as u32);
        let flip = centre[index] < 0.0;
        let normal = std::array::from_fn(|lane| if flip { -axis[lane] } else { axis[lane] });
        let distance = ((f64::from(value) - f64::from(radius_b)) - f64::from(radius_a)) as f32;
        BoxHit { normal, distance }
    };
    Some(hit)
}

/// The contact position of a hit: `centre + ((-distance) - radius of the sphere) * normal`,
/// with the sphere centre being the translation of `a`'s transform.
fn contact_position(e: &mut Engine, a: u32, hit: &BoxHit) -> [f32; 3] {
    let shape_a = e.mem.u32(a);
    let transform_a = e.mem.u32(a + 8);
    let radius_a = e.mem.f32(shape_a + 0x10);
    let scale = ((-f64::from(hit.distance)) - f64::from(radius_a)) as f32;
    let centre = load_vec(e, transform_a + 0x30);
    std::array::from_fn(|lane| scale * hit.normal[lane] + centre[lane])
}

/// Writes a hit as the exe lays a contact point out: the position at `+0` with `1.0` in the
/// fourth lane, the normal at `+0x10` with the distance in the fourth lane.
fn write_contact(e: &mut Engine, address: u32, position: [f32; 3], hit: &BoxHit) {
    for lane in 0..3u32 {
        e.mem.set_f32(address + 4 * lane, position[lane as usize]);
        e.mem
            .set_f32(address + 0x10 + 4 * lane, hit.normal[lane as usize]);
    }
    e.mem.set_f32(address + 0x1c, hit.distance);
    e.mem.set_f32(address + 0xc, 1.0);
}

// Translated from 00d3c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereBoxAgent::cleanup` (Xbox PDB): `this` in `ECX`, one stack word. When the contact
/// point id at `this + 0xc` is not `0xffff` the owner at `this + 8` is asked (slot `0x14`) to
/// remove it, with the stack word; then the deleting destructor (slot `0`, with flag `1`) is
/// called and its result returned.
pub fn hkp_sphere_box_agent_cleanup(e: &mut Engine, this: u32, owner_argument: u32) -> u32 {
    let id = e.mem.u16(this + 0xc);
    if id != 0xffff {
        let owner = e.mem.u32(this + 8);
        e.vcall(owner, 0x14, &args![u32::from(id), owner_argument]);
    }
    e.vcall(this, 0, &args![1u32]).u32()
}

// Translated from 00d3c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereBoxAgent::staticGetPenetrations` (Xbox PDB), cdecl `(a, b, _unused_2, collector)`
/// (the third word is never read). Between profile records, the sphere centre is taken to
/// the box's frame, clamped into the box (sign kept), and taken back to the world; when its
/// squared distance to the sphere centre is below the squared sum of the two radii (the box's
/// radius loaded through the x87) the collector's slot `4` is called with `(a, b)`.
pub fn hkp_sphere_box_agent_static_get_penetrations(
    e: &mut Engine,
    a: u32,
    b: u32,
    _unused_2: u32,
    collector: u32,
) {
    profile_record(e, PROFILE_SPHERE_BOX);
    let shape_a = e.mem.u32(a);
    let transform_a = e.mem.u32(a + 8);
    let shape_b = e.mem.u32(b);
    let transform_b = e.mem.u32(b + 8);
    let position_a = transform_a + 0x30;
    let centre = e.with_stack(0x10, |e, out| {
        e.call(
            SET_TRANSFORMED_INVERSE_POS,
            &args![out, transform_b, position_a],
        );
        load_vec(e, out.addr())
    });
    let radius_b = x87_quiet(e.mem.f32(shape_b + 0x10));
    let mask = load_bits(e, ABS_MASK);
    let extents = load_vec(e, shape_b + 0x20);
    let axis_x = load_vec(e, transform_b);
    let signs = load_bits(e, SIGN_MASK);
    let clamped: Vec4 = std::array::from_fn(|lane| {
        let absolute = f32::from_bits(centre[lane].to_bits() & mask[lane]);
        let nearest = minps(absolute, extents[lane]);
        f32::from_bits(nearest.to_bits() ^ (signs[lane] & centre[lane].to_bits()))
    });
    let axis_y = load_vec(e, transform_b + 0x10);
    let axis_z = load_vec(e, transform_b + 0x20);
    let translation = load_vec(e, transform_b + 0x30);
    let radius_a = e.mem.f32(shape_a + 0x10);
    let radius_sum = (f64::from(radius_b) + f64::from(radius_a)) as f32;
    let centre_a = load_vec(e, position_a);
    let world: Vec4 = std::array::from_fn(|lane| {
        let by_y = clamped[1] * axis_y[lane];
        let by_x = clamped[0] * axis_x[lane];
        let by_z = clamped[2] * axis_z[lane] + translation[lane];
        (by_y + by_x) + by_z - centre_a[lane]
    });
    let squares = zip_with(world, world, |x, y| x * y);
    let squared_distance = sum3(squares);
    let radius = f64::from(x87_quiet(radius_sum));
    if f64::from(squared_distance) < radius * radius {
        e.vcall(collector, 4, &args![a, b]);
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00d3c590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereBoxAgent::getPenetrations` (Xbox PDB), `RET 0x10`: passes its four stack words on
/// unchanged to `staticGetPenetrations` (`00d3c440`).
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_sphere_box_agent_get_penetrations(
    e: &mut Engine,
    _this: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) {
    hkp_sphere_box_agent_static_get_penetrations(e, a, b, c, d);
}

// Translated from 00d3c5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereBoxAgent::processCollision` (Xbox PDB): `this` in `ECX`; the stack words are the
/// two collidables `a` and `b`, the collision input (whose `float` at `+8` is the largest
/// distance that still counts) and the output record (`[out]` the cursor where the next
/// 0x30-byte contact goes, `[out + 4]` the owner's list handed to the removal call).
///
/// Between profile records: the sphere against box test. When the sphere is too far
/// away the contact point id at `this + 0xc` (if it is not `0xffff`) is removed through the
/// owner at `this + 8` (slot `0x14`, `(id, [out + 4])`) and the id is reset to `0xffff`.
/// Otherwise the contact is written at the cursor; if there is no id yet, the owner's slot `0xc`
/// creates it (`(a, b, input, out, 0, cursor)`, the low 16 bits of the result being the id);
/// with an id, `cursor + 0x20` gets it and the cursor moves on by `0x30`.
pub fn hkp_sphere_box_agent_process_collision(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    input: u32,
    out: u32,
) {
    profile_record(e, PROFILE_SPHERE_BOX);
    let cursor = e.mem.u32(out);
    match sphere_box_test(e, a, b, input) {
        None => {
            let id = e.mem.u16(this + 0xc);
            if id != 0xffff {
                let list = e.mem.u32(out + 4);
                let owner = e.mem.u32(this + 8);
                e.vcall(owner, 0x14, &args![u32::from(id), list]);
                e.mem.set_u16(this + 0xc, 0xffff);
            }
        }
        Some(hit) => {
            let position = contact_position(e, a, &hit);
            write_contact(e, cursor, position, &hit);
            if e.mem.u16(this + 0xc) == 0xffff {
                let owner = e.mem.u32(this + 8);
                let id = e
                    .vcall(owner, 0xc, &args![a, b, input, out, 0u32, cursor])
                    .u16();
                e.mem.set_u16(this + 0xc, id);
            }
            let id = e.mem.u16(this + 0xc);
            if id != 0xffff {
                e.mem.set_u32(cursor + 0x20, u32::from(id));
                let next = e.mem.u32(out).wrapping_add(0x30);
                e.mem.set_u32(out, next);
            }
        }
    }
    profile_record(e, PROFILE_END);
}

/// The body `getClosestPoints` and the static variant share (`00d3c910` and `00d3cc00` are
/// the same code up to their `RET`): between profile records, the sphere against box test; a
/// hit is laid out as a 0x30-byte record (the contact as `processCollision` writes it, then
/// the two collidables at `+0x20` and `+0x24`) and handed to the collector's slot `4`.
fn closest_points_body(e: &mut Engine, a: u32, b: u32, input: u32, collector: u32) {
    profile_record(e, PROFILE_SPHERE_BOX);
    if let Some(hit) = sphere_box_test(e, a, b, input) {
        let position = contact_position(e, a, &hit);
        e.with_stack(0x30, |e, point| {
            write_contact(e, point.addr(), position, &hit);
            e.mem.set_u32(point.addr() + 0x20, a);
            e.mem.set_u32(point.addr() + 0x24, b);
            e.vcall(collector, 4, &args![point]);
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00d3c910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereBoxAgent::getClosestPoints` (Xbox PDB), `RET 0x10`: `(a, b, input, collector)`.
/// The sphere against box test (`input + 8` the largest distance); a hit goes to the
/// collector's slot `4` as a 0x30-byte record.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_sphere_box_agent_get_closest_points(
    e: &mut Engine,
    _this: u32,
    a: u32,
    b: u32,
    input: u32,
    collector: u32,
) {
    closest_points_body(e, a, b, input, collector);
}

// Translated from 00d3cc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsphereboxagent.cpp`), cdecl `(a, b, input, collector)`: the static variant
/// of `getClosestPoints` (`00d3c910`), the same code with a plain `RET`.
pub fn fn_00d3cc00(e: &mut Engine, a: u32, b: u32, input: u32, collector: u32) {
    closest_points_body(e, a, b, input, collector);
}

// Translated from 00d3cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsphereboxagent.cpp`), cdecl `(registry)`, the registry being `this` of the
/// two register calls (`00cfbbe0`, each `(entry, type, type)`). The 0x18-byte entry holds four
/// function addresses, then two bytes (`+0x10`, `+0x11`); the first registers
/// `{00d3c400, 00d3cfc0, 00d3d040, 00d3ac80}` with bytes `1, 0` for the types `(4, 1)`, the
/// second `{00d3c380, 00d3c440, 00d3cc00, 00daf8e0}` with bytes `0, 0` for `(1, 4)`.
pub fn fn_00d3cef0(e: &mut Engine, registry: u32) {
    e.with_stack(0x18, |e, entry| {
        let base = entry.addr();
        for (slot, target) in [
            CREATE_AGENT,
            SYMMETRIC_PENETRATIONS,
            SYMMETRIC_CLOSEST_POINTS,
            SYMMETRIC_IS_PENETRATING,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(base + 4 * slot as u32, target);
        }
        e.mem.set_u8(base + 0x10, 1);
        e.mem.set_u8(base + 0x11, 0);
        e.call(REGISTER_AGENT, &args![registry, entry, 4u32, 1u32]);
        for (slot, target) in [
            CREATE_BRIDGE_AGENT,
            STATIC_PENETRATIONS,
            STATIC_CLOSEST_POINTS,
            BRIDGE_IS_PENETRATING,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(base + 4 * slot as u32, target);
        }
        e.mem.set_u8(base + 0x10, 0);
        e.mem.set_u8(base + 0x11, 0);
        e.call(REGISTER_AGENT, &args![registry, entry, 1u32, 4u32]);
    });
}

// Translated from 00d3cf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereBoxAgent>::getPenetrations` (Xbox PDB), `RET 0x10`:
/// `(a, b, c, collector)`. Wraps the collector in a 12-byte object (vtable `010ce81c`, a zero
/// byte at `+4`, the collector at `+8`) and calls `getPenetrations` (`00d3c590`) with the two
/// collidables swapped: `(b, a, c, wrapper)`.
/// Virtual method: the first word is `this` (`ECX` in the exe), left in `ECX` for the callee.
pub fn hkp_symmetric_agent_linear_cast_sphere_box_agent_get_penetrations(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    collector: u32,
) {
    e.with_stack(0xc, |e, wrapper| {
        e.mem.set_u32(wrapper.addr(), PENETRATION_COLLECTOR_VTABLE);
        e.mem.set_u8(wrapper.addr() + 4, 0);
        e.mem.set_u32(wrapper.addr() + 8, collector);
        hkp_sphere_box_agent_get_penetrations(e, this, b, a, c, wrapper.addr());
    });
}

// Translated from 00d3cfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsphereboxagent.cpp`), cdecl `(a, b, c, collector)`: the static variant of
/// the symmetric `getPenetrations` (`00d3cf80`); calls `staticGetPenetrations` (`00d3c440`)
/// with `(b, a, c, wrapper)`.
pub fn fn_00d3cfc0(e: &mut Engine, a: u32, b: u32, c: u32, collector: u32) {
    e.with_stack(0xc, |e, wrapper| {
        e.mem.set_u32(wrapper.addr(), PENETRATION_COLLECTOR_VTABLE);
        e.mem.set_u8(wrapper.addr() + 4, 0);
        e.mem.set_u32(wrapper.addr() + 8, collector);
        hkp_sphere_box_agent_static_get_penetrations(e, b, a, c, wrapper.addr());
    });
}

// Translated from 00d3d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereBoxAgent>::getClosestPoints` (Xbox PDB), `RET 0x10`:
/// `(a, b, c, collector)`. Wraps the collector in a 12-byte object (vtable `010ce7fc`, the
/// `float` at `0104da48` at `+4`, copied through the x87, the collector at `+8`) and calls
/// `getClosestPoints` (`00d3c910`) with `(b, a, c, wrapper)`.
/// Virtual method: the first word is `this` (`ECX` in the exe), left in `ECX` for the callee.
pub fn hkp_symmetric_agent_linear_cast_sphere_box_agent_get_closest_points(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    collector: u32,
) {
    e.with_stack(0xc, |e, wrapper| {
        let value = x87_quiet(e.mem.f32(SYMMETRIC_COLLECTOR_VALUE));
        e.mem.set_u32(wrapper.addr() + 8, collector);
        e.mem.set_f32(wrapper.addr() + 4, value);
        e.mem
            .set_u32(wrapper.addr(), CLOSEST_POINTS_COLLECTOR_VTABLE);
        hkp_sphere_box_agent_get_closest_points(e, this, b, a, c, wrapper.addr());
    });
}

// Translated from 00d3d040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`hkpsphereboxagent.cpp`), cdecl `(a, b, c, collector)`: the static variant of the
/// symmetric `getClosestPoints` (`00d3d000`); calls `00d3cc00` with `(b, a, c, wrapper)`.
pub fn fn_00d3d040(e: &mut Engine, a: u32, b: u32, c: u32, collector: u32) {
    e.with_stack(0xc, |e, wrapper| {
        let value = x87_quiet(e.mem.f32(SYMMETRIC_COLLECTOR_VALUE));
        e.mem.set_f32(wrapper.addr() + 4, value);
        e.mem.set_u32(wrapper.addr() + 8, collector);
        e.mem
            .set_u32(wrapper.addr(), CLOSEST_POINTS_COLLECTOR_VTABLE);
        fn_00d3cc00(e, b, a, c, wrapper.addr());
    });
}

// Translated from 00d3d080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereBoxAgent>::processCollision` (Xbox PDB): `this` in
/// `ECX`, stack words `(a, b, c, out)`, `RET 0x10`. Runs `processCollision` (`00d3c5b0`) with
/// the collidables swapped, then turns each contact it appended (0x30 bytes from the cursor
/// the call started at) around: the position moves by `distance * normal` (all four lanes,
/// the distance being the fourth lane of the normal vector) and the normal's first three
/// lanes change sign. When the `float` at `out + 0x3030` (read through the x87) changed, the
/// result record is flipped too: the vector at `+0x3020` loses the sign of its first three
/// lanes, the byte at `+0x304a` swaps its nibbles, the bytes at `+0x3048` and `+0x3049` swap,
/// the 16-bit entry at `+0x3040` swaps with the one at `+0x3040 + 2 * (old byte at +0x3049 + old
/// byte at +0x3048 - 1)`, and when the two swapped bytes ANDed give `2` the entries at `+0x3042`
/// and `+0x3044` swap.
pub fn hkp_symmetric_agent_linear_cast_sphere_box_agent_process_collision(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    c: u32,
    out: u32,
) {
    let saved = x87_quiet(e.mem.f32(out + 0x3030));
    let start = e.mem.u32(out);
    hkp_sphere_box_agent_process_collision(e, this, b, a, c, out);
    let negate = load_bits(e, NEGATE_XYZ_MASK);
    let mut contact = start;
    while contact < e.mem.u32(out) {
        let normal = load_vec(e, contact + 0x10);
        let position = load_vec(e, contact);
        let moved: Vec4 = std::array::from_fn(|lane| normal[3] * normal[lane] + position[lane]);
        store_vec(e, contact, moved);
        let bits = normal.map(f32::to_bits);
        let flipped: [u32; 4] = std::array::from_fn(|lane| bits[lane] ^ negate[lane]);
        store_bits(e, contact + 0x10, flipped);
        contact = contact.wrapping_add(0x30);
    }
    let current = x87_quiet(e.mem.f32(out + 0x3030));
    if saved != current {
        let vector = load_bits(e, out + 0x3020);
        let flipped: [u32; 4] = std::array::from_fn(|lane| vector[lane] ^ negate[lane]);
        store_bits(e, out + 0x3020, flipped);
        let nibbles = e.mem.u8(out + 0x304a);
        e.mem
            .set_u8(out + 0x304a, (nibbles >> 4) | ((nibbles & 0xf) << 4));
        let second = e.mem.u8(out + 0x3049);
        let first = e.mem.u8(out + 0x3048);
        e.mem.set_u8(out + 0x3049, first);
        e.mem.set_u8(out + 0x3048, second);
        let index = u32::from(second)
            .wrapping_add(u32::from(first))
            .wrapping_sub(1);
        let other = out.wrapping_add(0x3040).wrapping_add(index.wrapping_mul(2));
        let head = e.mem.u16(out + 0x3040);
        let swapped = e.mem.u16(other);
        e.mem.set_u16(out + 0x3040, swapped);
        e.mem.set_u16(other, head);
        let both = e.mem.u8(out + 0x3048) & e.mem.u8(out + 0x3049);
        if both == 2 {
            let left = e.mem.u16(out + 0x3042);
            let right = e.mem.u16(out + 0x3044);
            e.mem.set_u16(out + 0x3042, right);
            e.mem.set_u16(out + 0x3044, left);
        }
    }
}

// Translated from 00d3c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first three are never read (`_unused_0`, `_unused_1`, `_unused_2`) and the last is the contact manager, `manager`. Allocates 0x10 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc and the vtable word `010cf018` at +0. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3cf03.
pub fn fn_00d3c400(
    e: &mut Engine,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x10u32]).u32();
    e.mem.set_u16(block + 4, 0x10);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    e.mem.set_u32(block, 0x010c_f018);
    e.mem.set_u16(block + 0xc, 0xffff);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_c3c0, hkp_sphere_box_agent_cleanup(u32, u32) -> u32),
        entry!(
            0x00d3_c440,
            hkp_sphere_box_agent_static_get_penetrations(u32, u32, u32, u32)
        ),
        entry!(
            0x00d3_c590,
            hkp_sphere_box_agent_get_penetrations(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d3_c5b0,
            hkp_sphere_box_agent_process_collision(u32, u32, u32, u32, u32)
        ),
        entry!(
            0x00d3_c910,
            hkp_sphere_box_agent_get_closest_points(u32, u32, u32, u32, u32)
        ),
        entry!(0x00d3_cc00, fn_00d3cc00(u32, u32, u32, u32)),
        entry!(0x00d3_cef0, fn_00d3cef0(u32)),
        entry!(
            0x00d3_cf80,
            hkp_symmetric_agent_linear_cast_sphere_box_agent_get_penetrations(
                u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d3_cfc0, fn_00d3cfc0(u32, u32, u32, u32)),
        entry!(
            0x00d3_d000,
            hkp_symmetric_agent_linear_cast_sphere_box_agent_get_closest_points(
                u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d3_d040, fn_00d3d040(u32, u32, u32, u32)),
        entry!(
            0x00d3_d080,
            hkp_symmetric_agent_linear_cast_sphere_box_agent_process_collision(
                u32, u32, u32, u32, u32
            )
        ),
        entry!(0x00d3_c400, fn_00d3c400(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{
        HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    /// The doubles for the agent's deleting destructor, the collector's slot `4`, and the
    /// owner's slots `0xc` (create the contact point) and `0x14` (remove it).
    const DESTRUCTOR_DOUBLE: u32 = 0x0900_6000;
    const COLLECTOR_DOUBLE: u32 = 0x0900_6004;
    const CREATE_DOUBLE: u32 = 0x0900_600c;
    const REMOVE_DOUBLE: u32 = 0x0900_6014;
    const AGENT_VTABLE: u32 = 0x0900_6100;
    const OWNER_VTABLE: u32 = 0x0900_6200;
    const COLLECTOR_VTABLE: u32 = 0x0900_6300;

    type WordLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(NEGATE_XYZ_MASK + 4 * lane, 0x8000_0000);
        }
        e.mem.set_u32(NEGATE_XYZ_MASK + 12, 0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.put_vtable(AGENT_VTABLE, &[DESTRUCTOR_DOUBLE]);
        e.put_vtable(OWNER_VTABLE, &[0, 0, 0, CREATE_DOUBLE, 0, REMOVE_DOUBLE]);
        e.put_vtable(COLLECTOR_VTABLE, &[0, COLLECTOR_DOUBLE]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addr: u32) {
        e.register_double(addr, |_, _| Ret::default());
    }

    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-4,
            "{actual} is not near {expected}"
        );
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    /// The profile record stream for the thread and the global slot that finds it; returns the
    /// stream, which has room for two records.
    fn profile(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        stream
    }

    fn assert_profiled(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_SPHERE_BOX);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    /// The inverse transform double: the offset to the translation, taken on the three axes.
    fn inverse_position_double(e: &mut Engine) {
        e.register_double(SET_TRANSFORMED_INVERSE_POS, |e, a| {
            let origin = load_vec(e, a[1] + 0x30);
            let point = load_vec(e, a[2]);
            let offset: [f32; 3] = std::array::from_fn(|lane| point[lane] - origin[lane]);
            let local: [f32; 3] = std::array::from_fn(|axis| {
                let direction = load_vec(e, a[1] + 0x10 * axis as u32);
                direction[0] * offset[0] + direction[1] * offset[1] + direction[2] * offset[2]
            });
            store_vec(e, a[0], [local[0], local[1], local[2], 0.0]);
            Ret::default()
        });
    }

    /// A transform with the identity axes and the given translation.
    fn transform(e: &mut Engine, translation: [f32; 3]) -> u32 {
        let block = e.mem.alloc(0x40);
        store_vec(e, block, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, block + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, block + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(
            e,
            block + 0x30,
            [translation[0], translation[1], translation[2], 0.0],
        );
        block
    }

    fn collidable(e: &mut Engine, shape: u32, transform: u32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block, shape);
        e.mem.set_u32(block + 8, transform);
        block
    }

    /// A sphere of radius 0.5 centred at `centre` against the box with half extents 1 and
    /// radius 0.25 at the origin; returns the two collidables `(sphere, cuboid)`.
    fn scene(e: &mut Engine, centre: [f32; 3]) -> (u32, u32) {
        inverse_position_double(e);
        let sphere_shape = e.mem.alloc(0x30);
        e.mem.set_f32(sphere_shape + 0x10, 0.5);
        let box_shape = e.mem.alloc(0x30);
        e.mem.set_f32(box_shape + 0x10, 0.25);
        store_vec(e, box_shape + 0x20, [1.0, 1.0, 1.0, 0.0]);
        let sphere_transform = transform(e, centre);
        let box_transform = transform(e, [0.0; 3]);
        let sphere = collidable(e, sphere_shape, sphere_transform);
        let cuboid = collidable(e, box_shape, box_transform);
        (sphere, cuboid)
    }

    fn move_sphere(e: &mut Engine, sphere: u32, centre: [f32; 3]) {
        let transform = e.mem.u32(sphere + 8);
        store_vec(e, transform + 0x30, [centre[0], centre[1], centre[2], 0.0]);
    }

    /// A collision input whose largest distance (`+8`) is `max_distance`.
    fn input(e: &mut Engine, max_distance: f32) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_f32(block + 8, max_distance);
        block
    }

    /// An agent holding the contact point id `id`, and its owner.
    fn agent(e: &mut Engine, id: u16) -> (u32, u32) {
        let owner = e.mem.alloc(8);
        e.mem.set_u32(owner, OWNER_VTABLE);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, AGENT_VTABLE);
        e.mem.set_u32(this + 8, owner);
        e.mem.set_u16(this + 0xc, id);
        (this, owner)
    }

    /// An output record whose cursor points at a fresh buffer of two contacts; returns
    /// `(out, buffer)`.
    fn output(e: &mut Engine) -> (u32, u32) {
        let out = e.mem.alloc(0x3100);
        let buffer = e.mem.alloc(0x60);
        e.mem.set_u32(out, buffer);
        e.mem.set_u32(out + 4, 0xabcd);
        (out, buffer)
    }

    fn collector_object(e: &mut Engine) -> u32 {
        let block = e.mem.alloc(0xc);
        e.mem.set_u32(block, COLLECTOR_VTABLE);
        block
    }

    /// A collector double that keeps the 12 words at the record it is handed.
    fn recording_collector(e: &mut Engine) -> WordLog {
        let log: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&log);
        e.register_double(COLLECTOR_DOUBLE, move |e, a| {
            let words = (0..12).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            kept.borrow_mut().push(words);
            Ret::default()
        });
        log
    }

    #[test]
    fn cleanup_removes_the_contact_point_and_deletes_the_agent() {
        let mut e = engine();
        e.register_double(DESTRUCTOR_DOUBLE, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        noop(&mut e, REMOVE_DOUBLE);
        let (this, owner) = agent(&mut e, 9);
        let result = e.call(0x00d3_c3c0, &args![this, 0x77u32]);
        assert_eq!(result.u32(), this + 0x100);
        assert_eq!(calls(&e, REMOVE_DOUBLE), vec![vec![owner, 9, 0x77]]);
        assert_eq!(calls(&e, DESTRUCTOR_DOUBLE), vec![vec![this, 1]]);
        // Without a contact point id only the destructor runs.
        e.mem.set_u16(this + 0xc, 0xffff);
        e.call(0x00d3_c3c0, &args![this, 0x77u32]);
        assert_eq!(calls(&e, REMOVE_DOUBLE).len(), 1);
        assert_eq!(calls(&e, DESTRUCTOR_DOUBLE).len(), 2);
    }

    #[test]
    fn static_get_penetrations_reports_overlapping_pairs() {
        let mut e = engine();
        noop(&mut e, COLLECTOR_DOUBLE);
        let stream = profile(&mut e);
        let collector = collector_object(&mut e);
        // The sphere centre 1.5 along x: the nearest box point (1, 0, 0) is 0.5 away, below the
        // radius sum 0.75.
        let (sphere, cuboid) = scene(&mut e, [1.5, 0.0, 0.0]);
        e.call(0x00d3_c440, &args![sphere, cuboid, 0xdeadu32, collector]);
        assert_profiled(&e, stream);
        assert_eq!(
            calls(&e, COLLECTOR_DOUBLE),
            vec![vec![collector, sphere, cuboid]]
        );
        // Moved to 3.0 the distance 2.0 is above it.
        move_sphere(&mut e, sphere, [3.0, 0.0, 0.0]);
        e.call(0x00d3_c440, &args![sphere, cuboid, 0xdeadu32, collector]);
        assert_eq!(calls(&e, COLLECTOR_DOUBLE).len(), 1);
    }

    #[test]
    fn get_penetrations_forwards_to_the_static_test() {
        let mut e = engine();
        noop(&mut e, COLLECTOR_DOUBLE);
        profile(&mut e);
        let collector = collector_object(&mut e);
        let (sphere, cuboid) = scene(&mut e, [1.5, 0.0, 0.0]);
        e.call(
            0x00d3_c590,
            &args![VCALL_THIS, sphere, cuboid, 0xdeadu32, collector],
        );
        assert_eq!(
            calls(&e, COLLECTOR_DOUBLE),
            vec![vec![collector, sphere, cuboid]]
        );
    }

    #[test]
    fn process_collision_writes_the_contact_and_manages_the_point_id() {
        let mut e = engine();
        let stream = profile(&mut e);
        // Only the low 16 bits of the result are the id.
        e.register_double(CREATE_DOUBLE, |_, _| Ret {
            eax: 0x1_0005,
            ..Ret::default()
        });
        noop(&mut e, REMOVE_DOUBLE);
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let (this, owner) = agent(&mut e, 0xffff);
        let (out, buffer) = output(&mut e);
        let close = input(&mut e, 2.0);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, close, out]);
        assert_profiled(&e, stream);
        assert_eq!(
            calls(&e, CREATE_DOUBLE),
            vec![vec![owner, sphere, cuboid, close, out, 0, buffer]]
        );
        let position = load_vec(&e, buffer);
        near(position[0], 1.25);
        near(position[1], 0.0);
        near(position[2], 0.0);
        assert_eq!(position[3], 1.0);
        let normal = load_vec(&e, buffer + 0x10);
        near(normal[0], 1.0);
        near(normal[1], 0.0);
        near(normal[2], 0.0);
        near(normal[3], 1.25);
        assert_eq!(e.mem.u16(this + 0xc), 5);
        assert_eq!(e.mem.u32(buffer + 0x20), 5);
        assert_eq!(e.mem.u32(out), buffer + 0x30);
        // With an id the owner is not asked again and the next contact follows.
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, close, out]);
        assert_eq!(calls(&e, CREATE_DOUBLE).len(), 1);
        assert_eq!(e.mem.u32(buffer + 0x30 + 0x20), 5);
        assert_eq!(e.mem.u32(out), buffer + 0x60);
        // Too far: the point is removed with the owner's list and the id reset.
        let short = input(&mut e, 1.0);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, short, out]);
        assert_eq!(calls(&e, REMOVE_DOUBLE), vec![vec![owner, 5, 0xabcd]]);
        assert_eq!(e.mem.u16(this + 0xc), 0xffff);
        assert_eq!(e.mem.u32(out), buffer + 0x60);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, short, out]);
        assert_eq!(calls(&e, REMOVE_DOUBLE).len(), 1);
    }

    #[test]
    fn process_collision_keeps_the_cursor_when_no_id_is_made() {
        let mut e = engine();
        profile(&mut e);
        e.register_double(CREATE_DOUBLE, |_, _| Ret {
            eax: 0xffff,
            ..Ret::default()
        });
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let (this, _) = agent(&mut e, 0xffff);
        let (out, buffer) = output(&mut e);
        let close = input(&mut e, 2.0);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, close, out]);
        assert_eq!(calls(&e, CREATE_DOUBLE).len(), 1);
        assert_eq!(e.mem.u16(this + 0xc), 0xffff);
        assert_eq!(e.mem.u32(buffer + 0x20), 0);
        assert_eq!(e.mem.u32(out), buffer);
    }

    #[test]
    fn process_collision_inside_the_box_picks_the_nearest_face() {
        let mut e = engine();
        profile(&mut e);
        e.register_double(CREATE_DOUBLE, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        let (sphere, cuboid) = scene(&mut e, [0.5, -0.25, 0.0]);
        let (this, _) = agent(&mut e, 0xffff);
        let (out, buffer) = output(&mut e);
        // The largest distance is ignored inside the box.
        let ignored = input(&mut e, -1.0);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, ignored, out]);
        let position = load_vec(&e, buffer);
        near(position[0], 1.25);
        near(position[1], -0.25);
        near(position[2], 0.0);
        let normal = load_vec(&e, buffer + 0x10);
        near(normal[0], 1.0);
        near(normal[1], 0.0);
        near(normal[2], 0.0);
        near(normal[3], -1.25);
        // Below the centre of the bottom face the normal is flipped: the third axis wins the
        // tie of the first two, and the centre is on its negative side.
        move_sphere(&mut e, sphere, [0.0, 0.0, -0.9]);
        e.call(0x00d3_c5b0, &args![this, sphere, cuboid, ignored, out]);
        let position = load_vec(&e, buffer + 0x30);
        near(position[0], 0.0);
        near(position[1], 0.0);
        near(position[2], -1.25);
        let normal = load_vec(&e, buffer + 0x40);
        near(normal[0], 0.0);
        near(normal[1], 0.0);
        near(normal[2], -1.0);
        near(normal[3], -0.85);
    }

    /// `getClosestPoints` and its static twin: a hit is handed to the collector as a record.
    fn check_closest_points(address: u32) {
        let mut e = engine();
        let stream = profile(&mut e);
        let log = recording_collector(&mut e);
        let collector = collector_object(&mut e);
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let close = input(&mut e, 2.0);
        {
            let mut words = Vec::new();
            if address == 0x00d3_c910 {
                words.push(VCALL_THIS);
            }
            words.extend([sphere, cuboid, close, collector]);
            e.call(address, &words);
        }
        assert_profiled(&e, stream);
        let record = log.borrow()[0].clone();
        let values = floats(&record);
        near(values[0], 1.25);
        near(values[1], 0.0);
        near(values[2], 0.0);
        assert_eq!(values[3], 1.0);
        near(values[4], 1.0);
        near(values[5], 0.0);
        near(values[6], 0.0);
        near(values[7], 1.25);
        assert_eq!((record[8], record[9]), (sphere, cuboid));
        // Too far: nothing is reported.
        let short = input(&mut e, 1.0);
        {
            let mut words = Vec::new();
            if address == 0x00d3_c910 {
                words.push(VCALL_THIS);
            }
            words.extend([sphere, cuboid, short, collector]);
            e.call(address, &words);
        }
        assert_eq!(log.borrow().len(), 1);
        // Inside the box the largest distance is ignored.
        move_sphere(&mut e, sphere, [0.5, -0.25, 0.0]);
        let ignored = input(&mut e, -1.0);
        {
            let mut words = Vec::new();
            if address == 0x00d3_c910 {
                words.push(VCALL_THIS);
            }
            words.extend([sphere, cuboid, ignored, collector]);
            e.call(address, &words);
        }
        assert_eq!(log.borrow().len(), 2);
        let values = floats(&log.borrow()[1]);
        near(values[0], 1.25);
        near(values[1], -0.25);
        near(values[7], -1.25);
    }

    #[test]
    fn get_closest_points_hands_the_record_to_the_collector() {
        check_closest_points(0x00d3_c910);
    }

    #[test]
    fn static_get_closest_points_matches_the_virtual_one() {
        check_closest_points(0x00d3_cc00);
    }

    #[test]
    fn registry_entries_are_the_agent_function_tables() {
        let mut e = engine();
        let registered: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&registered);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words: Vec<u32> = (0..4).map(|slot| e.mem.u32(a[1] + 4 * slot)).collect();
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            words.extend_from_slice(&a[2..]);
            kept.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d3_cef0, &args![0x4242u32]);
        assert_eq!(
            *registered.borrow(),
            vec![
                vec![0xd3c400, 0xd3cfc0, 0xd3d040, 0xd3ac80, 1, 0, 4, 1],
                vec![0xd3c380, 0xd3c440, 0xd3cc00, 0xdaf8e0, 0, 0, 1, 4],
            ]
        );
        assert!(calls(&e, REGISTER_AGENT)
            .iter()
            .all(|arguments| arguments[0] == 0x4242));
    }

    /// The symmetric `getPenetrations` functions swap the collidables and wrap the collector.
    fn check_symmetric_penetrations(address: u32) {
        let mut e = engine();
        e.put_vtable(PENETRATION_COLLECTOR_VTABLE, &[0, COLLECTOR_DOUBLE]);
        profile(&mut e);
        let wrapped: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&wrapped);
        e.register_double(COLLECTOR_DOUBLE, move |e, a| {
            kept.borrow_mut().push(vec![
                a[1],
                a[2],
                u32::from(e.mem.u8(a[0] + 4)),
                e.mem.u32(a[0] + 8),
            ]);
            Ret::default()
        });
        let (sphere, cuboid) = scene(&mut e, [1.5, 0.0, 0.0]);
        // The vtable entry takes `this` first; its static twin does not.
        let mut words = Vec::new();
        if address == 0x00d3_cf80 {
            words.push(VCALL_THIS);
        }
        words.extend([cuboid, sphere, 0u32, 0x1234u32]);
        e.call(address, &words);
        assert_eq!(*wrapped.borrow(), vec![vec![sphere, cuboid, 0, 0x1234]]);
    }

    #[test]
    fn symmetric_get_penetrations_swaps_the_collidables() {
        check_symmetric_penetrations(0x00d3_cf80);
    }

    #[test]
    fn static_symmetric_get_penetrations_swaps_the_collidables() {
        check_symmetric_penetrations(0x00d3_cfc0);
    }

    /// The symmetric `getClosestPoints` functions swap the collidables and wrap the collector
    /// with the global `float` at `+4`.
    fn check_symmetric_closest_points(address: u32) {
        let mut e = engine();
        e.put_vtable(CLOSEST_POINTS_COLLECTOR_VTABLE, &[0, COLLECTOR_DOUBLE]);
        profile(&mut e);
        e.mem.set_f32(SYMMETRIC_COLLECTOR_VALUE, 7.5);
        let wrapped: WordLog = Rc::new(RefCell::new(Vec::new()));
        let kept = Rc::clone(&wrapped);
        e.register_double(COLLECTOR_DOUBLE, move |e, a| {
            kept.borrow_mut().push(vec![
                e.mem.u32(a[0] + 4),
                e.mem.u32(a[0] + 8),
                e.mem.u32(a[1] + 0x20),
                e.mem.u32(a[1] + 0x24),
            ]);
            Ret::default()
        });
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let close = input(&mut e, 2.0);
        // The vtable entry takes `this` first; its static twin does not.
        let mut words = Vec::new();
        if address == 0x00d3_d000 {
            words.push(VCALL_THIS);
        }
        words.extend([cuboid, sphere, close, 0x1234u32]);
        e.call(address, &words);
        assert_eq!(
            *wrapped.borrow(),
            vec![vec![7.5f32.to_bits(), 0x1234, sphere, cuboid]]
        );
    }

    #[test]
    fn symmetric_get_closest_points_swaps_the_collidables() {
        check_symmetric_closest_points(0x00d3_d000);
    }

    #[test]
    fn static_symmetric_get_closest_points_swaps_the_collidables() {
        check_symmetric_closest_points(0x00d3_d040);
    }

    #[test]
    fn symmetric_process_collision_turns_the_contacts_and_the_result_record_around() {
        let mut e = engine();
        profile(&mut e);
        // The owner's creation also changes the float at `out + 0x3030`, which makes the
        // result record be flipped.
        e.register_double(CREATE_DOUBLE, |e, a| {
            e.mem.set_f32(a[4] + 0x3030, 2.0);
            Ret {
                eax: 5,
                ..Ret::default()
            }
        });
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let (this, owner) = agent(&mut e, 0xffff);
        let (out, buffer) = output(&mut e);
        let close = input(&mut e, 2.0);
        e.mem.set_f32(out + 0x3030, 1.0);
        store_vec(&mut e, out + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u8(out + 0x3048, 2);
        e.mem.set_u8(out + 0x3049, 2);
        e.mem.set_u8(out + 0x304a, 0x12);
        for (index, value) in [0xa0u16, 0xa1, 0xa2, 0xa3].into_iter().enumerate() {
            e.mem.set_u16(out + 0x3040 + 2 * index as u32, value);
        }
        e.call(0x00d3_d080, &args![this, cuboid, sphere, close, out]);
        // The collidables are swapped for the creation call.
        assert_eq!(
            calls(&e, CREATE_DOUBLE),
            vec![vec![owner, sphere, cuboid, close, out, 0, buffer]]
        );
        assert_eq!(e.mem.u32(out), buffer + 0x30);
        // Position + distance * normal on all lanes; the normal loses the sign of x, y, z.
        let position = load_vec(&e, buffer);
        near(position[0], 2.5);
        near(position[1], 0.0);
        near(position[2], 0.0);
        near(position[3], 1.25 * 1.25 + 1.0);
        let normal = load_vec(&e, buffer + 0x10);
        near(normal[0], -1.0);
        near(normal[1], 0.0);
        near(normal[2], 0.0);
        near(normal[3], 1.25);
        // The result record: the vector, the nibbles, the counts and the entries.
        assert_eq!(load_vec(&e, out + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(e.mem.u8(out + 0x304a), 0x21);
        assert_eq!(e.mem.u16(out + 0x3040), 0xa3);
        assert_eq!(e.mem.u16(out + 0x3046), 0xa0);
        assert_eq!(e.mem.u16(out + 0x3042), 0xa2);
        assert_eq!(e.mem.u16(out + 0x3044), 0xa1);
    }

    #[test]
    fn symmetric_process_collision_leaves_an_unchanged_record_alone() {
        let mut e = engine();
        profile(&mut e);
        e.register_double(CREATE_DOUBLE, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        let (sphere, cuboid) = scene(&mut e, [3.0, 0.0, 0.0]);
        let (this, _) = agent(&mut e, 0xffff);
        let (out, _) = output(&mut e);
        let close = input(&mut e, 2.0);
        store_vec(&mut e, out + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_u8(out + 0x304a, 0x12);
        e.call(0x00d3_d080, &args![this, cuboid, sphere, close, out]);
        assert_eq!(load_vec(&e, out + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u8(out + 0x304a), 0x12);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The slot of the allocator's vtable (offset 4) that allocates, as a test double.
    const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0001;

    /// A memory router (`00c85750`) whose allocator allocates through a double that logs the
    /// size it was asked for and answers a zeroed block.
    fn allocator_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&sizes);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |e, a| {
            log.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        (e, sizes)
    }

    #[test]
    fn fn_00d3c400_fills_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e
            .call(0x00d3_c400, &args![1u32, 2u32, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x10]);
        assert_eq!(e.mem.u16(block + 4), 0x10);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_f018);
        assert_eq!(e.mem.u16(block + 0xc), 0xffff);
    }
}
