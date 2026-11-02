//! `Havok SDK/hkpheightfieldagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpHeightFieldAgent` (vtable `010ceaa8`) is the collision agent of a sphere-carrying shape
//! (body A) against a height field (body B). Its base is the Xbox PDB's `hkpCollisionAgent`:
//! reference count `+6` (u16), contact manager `+8`, and a 16-bit array (data `+0xC`, size
//! `+0x10`, capacity and flags `+0x14`) holding one contact point id per sphere (`0xFFFF` =
//! none). The slots of the two shapes are named here by what the code does with them (the
//! Xbox PDB has no record of the Havok classes): slot `0x2C` of body A's shape answers the
//! number of spheres, slot `0x30` fills a buffer with them (`vec4` centre and radius in the
//! fourth lane) and answers the buffer; slot `0x2C` of body B's shape (the height field)
//! takes a request `{spheres, count, tolerance}` and a result buffer of `count` vectors,
//! writing a separation distance into the fourth lane of each; slot `0x30` of body B's shape
//! casts a ray (`ray input`, body, collector).
//!
//! The memory router (`00c85750`) hands out the temporary buffers by moving the stack top it
//! holds in its first word; the functions give them back by writing the old top. The profile
//! records (`RDTSC` time stamps from the host clock) are appended to the thread's record
//! stream. SSE code computes lane by lane in `f32`; x87 code (`FLD`/`FSTP`) quiets a
//! signalling NaN and rounds at each `float` store.
//!
//! None of the functions has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, mul, rows_times, splat};
use crate::units::havok_sdk::hkpbvagent::{
    profile_start_record, symmetric_linear_cast, symmetric_process_collision,
    with_flip_body_collector, with_flip_collector, LABEL_END,
};
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{load_matrix, transform_point};
use crate::units::havok_sdk::hkpsphereshape::{
    load_vec, profile_record, store_vec, Vec4, PROFILE_END,
};

/// `hkTransform::setMulInverseMul` (`00cb22f0`): `this` is the output, then two transforms.
const SET_MUL_INVERSE_MUL: u32 = 0x00cb_22f0;
/// `hkpCollisionDispatcher` agent registration (`00cfbbe0`, `this` = dispatcher, `RET 0xC`):
/// `(agent function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// Vtable of the agent, of the ray-forwarding collector the linear cast builds (its slot 0 is
/// `addRayHit`) and the `hkReferencedObject` base the destructor ends with.
const AGENT_VTABLE: u32 = 0x010c_eaa8;
const RAY_COLLECTOR_VTABLE: u32 = 0x010c_ea14;
const BASE_VTABLE: u32 = 0x0102_e388;
/// Profile labels. The start records are `"LtHeightField"` with a second label that names the
/// first stage; the 12-byte records name the stages.
const LABEL_HEIGHT_FIELD: u32 = 0x010c_ea38;
const LABEL_GET_SPHERES: u32 = 0x010c_ea28;
const LABEL_COLLIDE: u32 = 0x010c_d514;
const LABEL_EXAMINE: u32 = 0x010c_ea1c;
const LABEL_CLOSEST_START: u32 = 0x010c_ea7c;
const LABEL_CLOSEST_GET_SPHERES: u32 = 0x010c_ea6c;
const LABEL_CLOSEST_TRANSFORM: u32 = 0x010c_ea60;
const LABEL_CLOSEST_COLLIDE: u32 = 0x010c_ea54;
const LABEL_CLOSEST_EXAMINE: u32 = 0x010c_ea48;
const LABEL_CAST_CLOSEST_POINTS: u32 = 0x010c_ea94;
const LABEL_CAST_SPHERES: u32 = 0x010c_ea84;
const LABEL_PROCESS: u32 = 0x010c_eae0;

/// Moves the memory router's stack top up by `size` bytes and answers the old top.
fn stack_take(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(router, top.wrapping_add(size));
    top
}

/// Writes `top` back as the memory router's stack top.
fn stack_reset(e: &mut Engine, top: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
}

/// The size of the buffer for `count` spheres: `count * 16` rounded up to 128 bytes.
fn sphere_buffer_size(count: i32) -> u32 {
    ((count as u32) << 4).wrapping_add(0x7f) & 0xffff_ff80
}

/// Moves the `count` spheres at `from` with the transform `rows` into the buffer at `to`
/// (centre `(y * row1 + x * row0) + (z * row2 + row3)`, the fourth lane copied), from the
/// last sphere down to the first. The loop runs at least once, as the exe's does.
fn transform_spheres(e: &mut Engine, rows: &[Vec4; 4], from: u32, to: u32, count: i32) {
    let mut index = count.wrapping_sub(1);
    loop {
        let offset = (index as u32) << 4;
        let sphere = load_vec(e, from.wrapping_add(offset));
        let mut moved = transform_point(rows, sphere);
        moved[3] = sphere[3];
        store_vec(e, to.wrapping_add(offset), moved);
        index = index.wrapping_sub(1);
        if index < 0 {
            break;
        }
    }
}

/// The rotation part of the transform `rows` applied to `v`: `(y * row1 + x * row0) + z *
/// row2`, all four lanes.
fn rotate(rows: &[Vec4; 4], v: Vec4) -> Vec4 {
    let by_y = mul(splat(v, 1), rows[1]);
    let by_x = mul(splat(v, 0), rows[0]);
    let by_z = mul(splat(v, 2), rows[2]);
    add(add(by_y, by_x), by_z)
}

// Translated from 00d31770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::cleanup` (Xbox PDB): gives every contact point id that is not
/// `0xFFFF` back to the contact manager (`[this + 8]`, slot `0x14`, with the id and `param`;
/// skipped altogether without a manager), then runs the agent's deleting destructor (slot 0,
/// argument 1).
pub fn hkp_height_field_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let this = this.addr();
    if e.mem.u32(this + 8) != 0 && e.mem.i32(this + 0x10) > 0 {
        let mut index = 0i32;
        loop {
            let data = e.mem.u32(this + 0xc);
            let id = e.mem.u16(data.wrapping_add((index as u32) << 1));
            if id != 0xffff {
                let manager = e.mem.u32(this + 8);
                e.vcall(manager, 0x14, &args![u32::from(id), param]);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.i32(this + 0x10) {
                break;
            }
        }
    }
    e.vcall(this, 0, &args![1u32]);
}

// Translated from 00d317d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::staticGetPenetrations` (Xbox PDB): a cdecl function over the two
/// bodies, the input and a collector. Profiled (`"LtHeightField"`). Moves body A's spheres
/// into body B's space (`setMulInverseMul` of the two motions, B's first), asks body B's shape
/// to collide them (the request holds the spheres, their count and the input's tolerance at
/// `+8`, an x87 load and store) and, for the first sphere whose result distance is below zero,
/// tells the collector (slot 4, with the two bodies) once.
pub fn hkp_height_field_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    profile_start_record(e, LABEL_HEIGHT_FIELD, LABEL_GET_SPHERES);
    let shape_a = e.mem.u32(body_a.addr());
    let shape_b = e.mem.u32(body_b.addr());
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let (count, size, top) = e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        e.call(SET_MUL_INVERSE_MUL, &args![matrix, motion_b, motion_a]);
        let count = e.vcall(shape_a, 0x2c, &args![]).u32() as i32;
        let size = sphere_buffer_size(count);
        let top = stack_take(e, size);
        let spheres = e.vcall(shape_a, 0x30, &args![top]).u32();
        let rows = load_matrix(e, matrix);
        transform_spheres(e, &rows, spheres, top, count);
        (count, size, top)
    });
    profile_record(e, LABEL_COLLIDE);
    let results = stack_take(e, size);
    let tolerance = x87_quiet_bits(e.mem.u32(input.addr() + 8));
    e.with_stack(0xc, |e, request| {
        let request = request.addr();
        e.mem.set_u32(request, top);
        e.mem.set_u32(request + 4, count as u32);
        e.mem.set_u32(request + 8, tolerance);
        e.vcall(shape_b, 0x2c, &args![request, results]);
    });
    profile_record(e, LABEL_EXAMINE);
    let mut remaining = count.wrapping_sub(1);
    if remaining >= 0 {
        let mut distance_at = results.wrapping_add(0xc);
        loop {
            // `FLDZ; FCOM`: below zero only when ordered.
            if e.mem.f32(distance_at) < 0.0 {
                e.vcall(collector.addr(), 4, &args![body_a, body_b]);
                break;
            }
            distance_at = distance_at.wrapping_add(0x10);
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
    }
    stack_reset(e, results);
    stack_reset(e, top);
    profile_record(e, LABEL_END);
}

// Translated from 00d319e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::staticGetClosestPoints` (Xbox PDB): a cdecl function over the two
/// bodies, the input and a collector. Profiled in stages. Moves body A's spheres into body
/// B's space, asks body B's shape to collide them as `staticGetPenetrations` does and, for
/// every sphere whose result distance is not above the input's tolerance (an x87 compare:
/// NaN is not below), builds a contact `{position, normal (distance in its fourth lane),
/// body A, body B}` and tells the collector (slot 4). The contact point is `(-radius -
/// distance) * direction + centre` (the result vector is the direction, the sphere buffer
/// holds the centre) moved to world space with body B's motion; the normal is the direction
/// rotated with it.
pub fn hkp_height_field_agent_static_get_closest_points(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    profile_start_record(e, LABEL_HEIGHT_FIELD, LABEL_CLOSEST_START);
    let shape_a = e.mem.u32(body_a.addr());
    let shape_b = e.mem.u32(body_b.addr());
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let (count, size, top) = e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        e.call(SET_MUL_INVERSE_MUL, &args![matrix, motion_b, motion_a]);
        let count = e.vcall(shape_a, 0x2c, &args![]).u32() as i32;
        let size = sphere_buffer_size(count);
        let top = stack_take(e, size);
        profile_record(e, LABEL_CLOSEST_GET_SPHERES);
        let spheres = e.vcall(shape_a, 0x30, &args![top]).u32();
        profile_record(e, LABEL_CLOSEST_TRANSFORM);
        let rows = load_matrix(e, matrix);
        transform_spheres(e, &rows, spheres, top, count);
        (count, size, top)
    });
    profile_record(e, LABEL_CLOSEST_COLLIDE);
    let results = stack_take(e, size);
    let tolerance_bits = x87_quiet_bits(e.mem.u32(input.addr() + 8));
    e.with_stack(0xc, |e, request| {
        let request = request.addr();
        e.mem.set_u32(request, top);
        e.mem.set_u32(request + 4, count as u32);
        e.mem.set_u32(request + 8, tolerance_bits);
        e.vcall(shape_b, 0x2c, &args![request, results]);
    });
    profile_record(e, LABEL_CLOSEST_EXAMINE);
    let tolerance = f32::from_bits(x87_quiet_bits(e.mem.u32(input.addr() + 8)));
    let mut remaining = count.wrapping_sub(1);
    if remaining >= 0 {
        e.with_stack(0x28, |e, contact| {
            let contact = contact.addr();
            let mut at = results;
            let offset = top.wrapping_sub(results);
            loop {
                let distance = e.mem.f32(at.wrapping_add(0xc));
                // `FCOMPP; TEST AH,1`: skipped when the tolerance is below the distance or
                // either is a NaN.
                if distance <= tolerance {
                    let direction = load_vec(e, at);
                    let centre = load_vec(e, at.wrapping_add(offset));
                    let radius = f64::from(x87_quiet(
                        e.mem.f32(at.wrapping_add(offset).wrapping_add(0xc)),
                    ));
                    let reach = ((-radius) - f64::from(x87_quiet(distance))) as f32;
                    let rows = load_matrix(e, motion_b);
                    let moved = add(mul([reach; 4], direction), centre);
                    e.mem.set_u32(contact + 0x24, body_b.addr());
                    e.mem.set_u32(contact + 0x20, body_a.addr());
                    store_vec(e, contact, transform_point(&rows, moved));
                    store_vec(e, contact + 0x10, rotate(&rows, direction));
                    let lane = x87_quiet_bits(distance.to_bits());
                    e.mem.set_u32(contact + 0x1c, lane);
                    e.vcall(collector.addr(), 4, &args![contact]);
                }
                at = at.wrapping_add(0x10);
                remaining = remaining.wrapping_sub(1);
                if remaining < 0 {
                    break;
                }
            }
        });
    }
    stack_reset(e, results);
    stack_reset(e, top);
    profile_record(e, LABEL_END);
}

// Translated from 00d31d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::staticLinearCast` (Xbox PDB): a cdecl function over the two bodies,
/// the input, the cast collector and the start-point collector. Profiled in stages. When
/// there is a start collector it runs `staticGetClosestPoints` with it. Then moves body A's
/// spheres into body B's space, builds a ray-forwarding collector (vtable `010cea14`, `1.0`
/// at `+4`, the sphere's world centre at `+0x10`, its radius at `+0x20`, the input's path
/// vector `input + 0x50` at `+0x30`, body A at `+0x40` and the cast collector at `+0x44`) and,
/// for each sphere, a ray input (from the centre, to the centre plus the path moved into B's
/// space, zero filter words at `+0x20` and `+0x24`, the radius at `+0x30` and the input's
/// float at `+0x60` at `+0x34`) and casts it with slot `0x30` of body B's shape (ray input,
/// body B, the collector). The words of the two records the exe does not write stay as the
/// frame had them.
pub fn hkp_height_field_agent_static_linear_cast(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    profile_start_record(e, LABEL_HEIGHT_FIELD, LABEL_CAST_CLOSEST_POINTS);
    if start_collector != 0 {
        hkp_height_field_agent_static_get_closest_points(
            e,
            body_a,
            body_b,
            input,
            Ptr::new(start_collector),
        );
    }
    profile_record(e, LABEL_GET_SPHERES);
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let shape_b = e.mem.u32(body_b.addr());
    let shape_a = e.mem.u32(body_a.addr());
    // The frame: the transform (0x40 bytes), the path vector moved into B's space, the
    // ray-forwarding collector (0x48) and the ray input (0x38).
    e.with_stack(0xe0, |e, frame| {
        let frame = frame.addr();
        let (path_at, collector, ray) = (frame + 0x40, frame + 0x50, frame + 0xa0);
        e.call(SET_MUL_INVERSE_MUL, &args![frame, motion_b, motion_a]);
        let count = e.vcall(shape_a, 0x2c, &args![]).u32() as i32;
        let size = sphere_buffer_size(count);
        let top = stack_take(e, size);
        let rows_b = [0, 0x10, 0x20].map(|offset| load_vec(e, motion_b + offset));
        let path = load_vec(e, input.addr() + 0x50);
        let moved_path = rows_times(path, rows_b[0], rows_b[1], rows_b[2]);
        store_vec(e, path_at, moved_path);
        let spheres = e.vcall(shape_a, 0x30, &args![top]).u32();
        profile_record(e, LABEL_CAST_SPHERES);
        let float_at_60 = x87_quiet_bits(e.mem.u32(input.addr() + 0x60));
        e.mem.set_u32(ray + 0x34, float_at_60);
        e.mem.set_u32(collector + 4, 1.0f32.to_bits());
        for lane in 0..4 {
            let word = e.mem.u32(input.addr() + 0x50 + 4 * lane);
            e.mem.set_u32(collector + 0x30 + 4 * lane, word);
        }
        e.mem.set_u32(ray + 0x20, 0);
        e.mem.set_u32(ray + 0x24, 0);
        e.mem.set_u32(collector, RAY_COLLECTOR_VTABLE);
        e.mem.set_u32(collector + 0x40, body_a.addr());
        e.mem.set_u32(collector + 0x44, cast_collector);
        if count > 0 {
            let mut sphere_at = spheres;
            let mut remaining = count;
            loop {
                let sphere = load_vec(e, sphere_at);
                let rows = load_matrix(e, frame);
                let radius = x87_quiet_bits(sphere[3].to_bits());
                e.mem.set_u32(ray + 0x30, radius);
                let by_x = mul(splat(sphere, 0), rows[0]);
                let by_y = mul(splat(sphere, 1), rows[1]);
                let by_z = mul(splat(sphere, 2), rows[2]);
                let moved = add(add(by_x, by_y), add(by_z, rows[3]));
                store_vec(e, ray, moved);
                store_vec(e, ray + 0x10, add(moved, load_vec(e, path_at)));
                let rows_a = load_matrix(e, motion_a);
                e.mem.set_u32(collector + 0x20, radius);
                store_vec(e, collector + 0x10, transform_point(&rows_a, sphere));
                e.vcall(shape_b, 0x30, &args![ray, body_b, collector]);
                sphere_at = sphere_at.wrapping_add(0x10);
                remaining = remaining.wrapping_sub(1);
                if remaining == 0 {
                    break;
                }
            }
        }
        stack_reset(e, top);
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d32030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::linearCast` (Xbox PDB): hands the five words (the two bodies, the
/// input, the cast collector and the start collector) on to `staticLinearCast`; `this` is not
/// used.
pub fn hkp_height_field_agent_linear_cast(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    hkp_height_field_agent_static_linear_cast(
        e,
        body_a,
        body_b,
        input,
        cast_collector,
        start_collector,
    );
}

// Translated from 00d32060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the agent (the map gives it no name; the Xbox PDB's
/// `hkpHeightFieldAgent::hkpHeightFieldAgent` is unconfirmed): reference count 1, the contact
/// manager `contact_manager` (the fourth word) at `+8`, the vtable and an empty 16-bit array
/// (data and size zero, capacity word `0x80000000`). When there is a contact manager it asks
/// body A's shape for the number of spheres (slot `0x2C`), reserves room for that many 16-bit
/// ids (`hkArrayUtil::_reserve` with the heap allocator: twice the capacity, or the count if
/// that is more) and fills the new ids with `0xFFFF` and sets the size. The second and third
/// words are not read. Returns `this`.
pub fn fn_00d32060(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    contact_manager: u32,
) -> Ptr {
    let this_at = this.addr();
    let ids = this_at + 0xc;
    e.mem.set_u16(this_at + 6, 1);
    e.mem.set_u32(this_at + 8, contact_manager);
    e.mem.set_u32(this_at, AGENT_VTABLE);
    e.mem.set_u32(ids, 0);
    e.mem.set_u32(ids + 4, 0);
    e.mem.set_u32(ids + 8, 0x8000_0000);
    if contact_manager != 0 {
        let shape = e.mem.u32(body_a.addr());
        let count = e.vcall(shape, 0x2c, &args![]).u32() as i32;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(ids + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, ids, wanted as u32, 2u32]);
        }
        let size = e.mem.i32(ids + 4);
        let data = e.mem.u32(ids);
        let missing = count.wrapping_sub(size);
        if missing > 0 {
            let first = data.wrapping_add((size as u32).wrapping_mul(2));
            for index in 0..missing as u32 {
                e.mem.set_u16(first.wrapping_add(index * 2), 0xffff);
            }
        }
        e.mem.set_u32(ids + 4, count as u32);
    }
    this
}

// Translated from 00d32150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::getPenetrations` (Xbox PDB): hands the two bodies, the input and the
/// collector on to `staticGetPenetrations`; `this` is not used.
pub fn hkp_height_field_agent_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    hkp_height_field_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

// Translated from 00d32170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::getClosestPoints` (Xbox PDB): hands the two bodies, the input and the
/// collector on to `staticGetClosestPoints`; `this` is not used.
pub fn hkp_height_field_agent_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    hkp_height_field_agent_static_get_closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d321e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the two agent entries of the height-field pair with the collision dispatcher
/// (cdecl, one word): the symmetric entry (`createFunc` `00d32190`, then the symmetric static
/// queries `00d32370`, `00d323b0`, `00d323f0`; flag bytes 1 and 1) for the type pair
/// `(0x19, 0x1a)` and the direct entry (`00d32110`, `00d317d0`, `00d319e0`, `00d31d30`; flag
/// bytes 0 and 1) for `(0x1a, 0x19)`. The two bytes after the flags are not set by the exe
/// (the callee copies the word they are in); here they are zero.
pub fn fn_00d321e0(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (words, flag_a, type_a, type_b) in [
            (
                [0x00d3_2190, 0x00d3_2370, 0x00d3_23b0, 0x00d3_23f0],
                1u8,
                0x19u32,
                0x1au32,
            ),
            (
                [0x00d3_2110, 0x00d3_17d0, 0x00d3_19e0, 0x00d3_1d30],
                0,
                0x1a,
                0x19,
            ),
        ] {
            for (index, word) in words.into_iter().enumerate() {
                e.mem.set_u32(entry + 4 * index as u32, word);
            }
            e.mem.set_u8(entry + 0x10, flag_a);
            e.mem.set_u8(entry + 0x11, 1);
            e.call(REGISTER_AGENT, &args![dispatcher, entry, type_a, type_b]);
        }
    });
}

// Translated from 00d32270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkHeightFieldRayForwardingCollector::addRayHit` (Xbox PDB): `this` is the collector the
/// linear cast builds, `cd_body` the body that was hit and `hit_info` the ray hit (normal
/// `+0`, hit fraction `+0x10`). Builds a contact `{position, normal, body A, hit body}` on
/// the stack and hands it to the cast collector (`this + 0x44`, slot 4). The normal is the
/// hit normal rotated by the hit body's motion (its fourth lane then holds the hit fraction,
/// an x87 load and store); the position is `hit fraction * path + start` (`this + 0x30`,
/// `this + 0x10`) moved back by the radius (`this + 0x20`) along the rotated normal. Then the
/// collector's own fraction (`this + 4`) becomes the cast collector's fraction (`+4`) when
/// that is below it (an x87 compare: when the collector's is not above, or either is a NaN,
/// it keeps its own); both go through an x87 load and store.
pub fn hk_height_field_ray_forwarding_collector_add_ray_hit(
    e: &mut Engine,
    this: Ptr,
    cd_body: Ptr,
    hit_info: Ptr,
) {
    let this = this.addr();
    let body_a = e.mem.u32(this + 0x40);
    let reach = -x87_quiet(e.mem.f32(this + 0x20));
    let fraction = e.mem.f32(hit_info.addr() + 0x10);
    let fraction_lane = x87_quiet_bits(fraction.to_bits());
    let motion = e.mem.u32(cd_body.addr() + 8);
    let cast_collector = e.mem.u32(this + 0x44);
    let path = load_vec(e, this + 0x30);
    let start = load_vec(e, this + 0x10);
    let point = add(mul([fraction; 4], path), start);
    let rows = load_matrix(e, motion);
    let normal_in = load_vec(e, hit_info.addr());
    let normal = rotate(&rows, normal_in);
    e.with_stack(0x28, |e, contact| {
        let contact = contact.addr();
        e.mem.set_u32(contact + 0x20, body_a);
        e.mem.set_u32(contact + 0x24, cd_body.addr());
        store_vec(e, contact + 0x10, normal);
        e.mem.set_u32(contact + 0x1c, fraction_lane);
        store_vec(e, contact, add(mul([reach; 4], normal), point));
        e.vcall(cast_collector, 4, &args![contact]);
    });
    let cast_fraction = x87_quiet(e.mem.f32(e.mem.u32(this + 0x44) + 4));
    let own_fraction = x87_quiet(e.mem.f32(this + 4));
    // `FCOM; TEST AH,0x41`: the cast collector's fraction is taken only when the collector's
    // own is above it.
    let kept = if own_fraction > cast_fraction {
        cast_fraction
    } else {
        own_fraction
    };
    e.mem.set_u32(this + 4, x87_quiet_bits(kept.to_bits()));
}

// Translated from 00d32370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the height-field pair (the map gives it no name;
/// cdecl): runs `staticGetPenetrations` with the two bodies swapped and, in place of the
/// caller's collector word, a flip-body collector (vtable `010ce81c`, byte 0 at `+4`, the
/// caller's word at `+8`) built on the stack.
pub fn fn_00d32370(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: u32) {
    with_flip_body_collector(e, collector, |e, flip| {
        hkp_height_field_agent_static_get_penetrations(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d323b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the height-field pair (the map gives it no
/// name; cdecl): runs `staticGetClosestPoints` with the two bodies swapped and a flip
/// collector (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's
/// word at `+8`).
pub fn fn_00d323b0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: u32) {
    with_flip_collector(e, collector, |e, flip| {
        hkp_height_field_agent_static_get_closest_points(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d323f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static linear cast of the height-field pair (the map gives it no name;
/// cdecl): runs `staticLinearCast` with the two bodies swapped, a copy of the input whose
/// path vector (`input + 0x50`) is negated, and flip-cast collectors (vtable `010ce80c`)
/// wrapping the caller's two (`start_collector` null stays null).
pub fn fn_00d323f0(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    symmetric_linear_cast(
        e,
        input,
        cast_collector,
        start_collector,
        |e, copy, first, second| {
            hkp_height_field_agent_static_linear_cast(
                e,
                body_b,
                body_a,
                Ptr::new(copy),
                first,
                second,
            );
        },
    );
}

// Translated from 00d324e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::processCollision` (Xbox PDB): profiled (`"TtHeightField"`). Moves
/// body A's spheres (as many as the agent has ids, `this + 0x10`) into body B's space, asks
/// body B's shape to collide them (as `staticGetPenetrations` does) and then, for each sphere
/// with its id at `this + 0xC` (16-bit, `0xFFFF` = none): when the result distance is above
/// the tolerance (`input + 8`, an x87 compare) an existing id is given back to the contact
/// manager (`[this + 8]`, slot `0x14`, with the id and `[output + 4]`) and cleared; otherwise
/// a contact is written at `[output]` (position `(-radius) * direction + centre` moved by
/// body B's motion, the direction rotated with it, the distance in the fourth lane of the
/// normal, and the id at `+0x20`), first asking the manager (slot `0xC`, with the bodies, the
/// input, the output, 0 and the contact) for an id when there is none, and `[output]` moves
/// on by `0x30` unless the manager answered `0xFFFF`.
pub fn hkp_height_field_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let this = this.addr();
    let output = output.addr();
    profile_record(e, LABEL_PROCESS);
    let shape_a = e.mem.u32(body_a.addr());
    let motion_a = e.mem.u32(body_a.addr() + 8);
    let shape_b = e.mem.u32(body_b.addr());
    let motion_b = e.mem.u32(body_b.addr() + 8);
    let (count, size, top, ids) = e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        e.call(SET_MUL_INVERSE_MUL, &args![matrix, motion_b, motion_a]);
        let count = e.mem.i32(this + 0x10);
        let size = sphere_buffer_size(count);
        let top = stack_take(e, size);
        let ids = e.mem.u32(this + 0xc);
        let spheres = e.vcall(shape_a, 0x30, &args![top]).u32();
        let rows = load_matrix(e, matrix);
        transform_spheres(e, &rows, spheres, top, count);
        (count, size, top, ids)
    });
    let results = stack_take(e, size);
    let tolerance_bits = x87_quiet_bits(e.mem.u32(input.addr() + 8));
    e.with_stack(0xc, |e, request| {
        let request = request.addr();
        e.mem.set_u32(request, top);
        e.mem.set_u32(request + 4, count as u32);
        e.mem.set_u32(request + 8, tolerance_bits);
        e.vcall(shape_b, 0x2c, &args![request, results]);
    });
    let tolerance = f32::from_bits(x87_quiet_bits(e.mem.u32(input.addr() + 8)));
    let mut remaining = count.wrapping_sub(1);
    if remaining >= 0 {
        let offset = top.wrapping_sub(results);
        let mut at = results;
        let mut id_at = ids;
        loop {
            let distance = e.mem.f32(at.wrapping_add(0xc));
            // `FCOMPP; TEST AH,5; JP`: the contact is made unless the tolerance is below the
            // distance (a NaN makes one).
            if tolerance < distance {
                let id = e.mem.u16(id_at);
                if id != 0xffff {
                    let word = e.mem.u32(output + 4);
                    let manager = e.mem.u32(this + 8);
                    e.vcall(manager, 0x14, &args![u32::from(id), word]);
                    e.mem.set_u16(id_at, 0xffff);
                }
            } else {
                let radius = x87_quiet(e.mem.f32(at.wrapping_add(offset).wrapping_add(0xc)));
                let point = e.mem.u32(output);
                let direction = load_vec(e, at);
                let centre = load_vec(e, at.wrapping_add(offset));
                let rows = load_matrix(e, motion_b);
                let moved = add(mul([-radius; 4], direction), centre);
                store_vec(e, point, transform_point(&rows, moved));
                store_vec(e, point + 0x10, rotate(&rows, direction));
                e.mem
                    .set_u32(point + 0x1c, x87_quiet_bits(distance.to_bits()));
                if e.mem.u16(id_at) == 0xffff {
                    let manager = e.mem.u32(this + 8);
                    let made = e
                        .vcall(
                            manager,
                            0xc,
                            &args![body_a, body_b, input, output, 0u32, point],
                        )
                        .u32() as u16;
                    e.mem.set_u16(id_at, made);
                    if made != 0xffff {
                        let next = e.mem.u32(output);
                        e.mem.set_u32(output, next.wrapping_add(0x30));
                    }
                } else {
                    let next = e.mem.u32(output);
                    e.mem.set_u32(output, next.wrapping_add(0x30));
                }
                let id = u32::from(e.mem.u16(id_at));
                e.mem.set_u32(point + 0x20, id);
            }
            at = at.wrapping_add(0x10);
            id_at = id_at.wrapping_add(2);
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
    }
    stack_reset(e, results);
    stack_reset(e, top);
    profile_record(e, PROFILE_END);
}

// Translated from 00d327e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpHeightFieldAgent::_scalar_deleting_destructor_` (Xbox PDB): sets the agent's vtable,
/// empties the id array (size 0) and, when its capacity word is not negative (the buffer is
/// owned), gives the buffer back to the heap allocator (`[router + 0x10]`, slot `0x10`, with
/// the data and twice the capacity); then clears the array (data 0, capacity word
/// `0x80000000`), leaves the `hkReferencedObject` vtable and, when bit 0 of `flags` is set,
/// frees the agent itself through the allocator (slot 8, with `this` and the 16-bit size at
/// `+4`). Returns `this`.
pub fn hkp_height_field_agent_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let this_at = this.addr();
    e.mem.set_u32(this_at, AGENT_VTABLE);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(this_at + 0x14);
    e.mem.set_u32(this_at + 0x10, 0);
    if capacity >= 0 {
        let owned = (capacity as u32) & 0x3fff_ffff;
        let bytes = owned.wrapping_add(owned);
        let data = e.mem.u32(this_at + 0xc);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this_at + 0xc, 0);
    e.mem.set_u32(this_at + 0x14, 0x8000_0000);
    e.mem.set_u32(this_at, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(this_at + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this_at, size]);
    }
    this
}

// Translated from 00d32850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpHeightFieldAgent>::processCollision` (Xbox PDB): runs
/// `hkpHeightFieldAgent::processCollision` with the bodies swapped, then turns what it
/// appended to the output around (see
/// `hkp_symmetric_agent_linear_cast_bv_agent_process_collision`, which does the same for the
/// bounding-volume agent).
pub fn hkp_symmetric_agent_linear_cast_height_field_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let output_at = output.addr();
    symmetric_process_collision(e, output_at, |e| {
        hkp_height_field_agent_process_collision(e, this, body_b, body_a, input, output);
    });
}

// Translated from 00d32970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpHeightFieldAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpHeightFieldAgent::getPenetrations` with the bodies swapped and a flip-body collector
/// (vtable `010ce81c`, byte 0 at `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_height_field_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: u32,
) {
    with_flip_body_collector(e, collector, |e, flip| {
        hkp_height_field_agent_get_penetrations(e, this, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d329b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpHeightFieldAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpHeightFieldAgent::getClosestPoints` with the bodies swapped and a flip collector
/// (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word at
/// `+8`).
pub fn hkp_symmetric_agent_linear_cast_height_field_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: u32,
) {
    with_flip_collector(e, collector, |e, flip| {
        hkp_height_field_agent_get_closest_points(e, this, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d329f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgent<hkpHeightFieldAgent>::linearCast` (Xbox PDB): runs
/// `hkpHeightFieldAgent::linearCast` with the bodies swapped, a copy of the input whose path
/// vector is negated and flip-cast collectors wrapping the caller's two (as `00d323f0` does).
pub fn hkp_symmetric_agent_height_field_agent_linear_cast(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    symmetric_linear_cast(
        e,
        input,
        cast_collector,
        start_collector,
        |e, copy, first, second| {
            hkp_height_field_agent_linear_cast(
                e,
                this,
                body_b,
                body_a,
                Ptr::new(copy),
                first,
                second,
            );
        },
    );
}

/// Allocates `size` bytes the way the agent create functions do: the thread's memory router
/// (`00c85750`) has the allocator at `+0x10`; its virtual slot `4` hands out the block, whose
/// size goes into the 16-bit word at `+4`.
fn allocate_agent_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00d32110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_manager`):
/// a create function of the height-field pair (code at `00d3222d` stores its address in an agent
/// record). Allocates the 0x18-byte agent (`allocate_agent_block`) and runs the constructor
/// `00d32060` on it with the four words in this order. Answers what the constructor answers, the
/// agent.
pub fn fn_00d32110(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    contact_manager: u32,
) -> u32 {
    let agent = allocate_agent_block(e, 0x18);
    fn_00d32060(
        e,
        Ptr::new(agent),
        body_a,
        body_b.addr(),
        input,
        contact_manager,
    )
    .addr()
}

// Translated from 00d32190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words `body_a`, `body_b`, `input`, `contact_manager`):
/// the create function of the flipped height-field agent (code at `00d321f3` stores its address
/// in an agent record). Allocates the 0x18-byte agent (`allocate_agent_block`), runs the
/// constructor `00d32060` with the two bodies exchanged (`body_b`, `body_a`, `input`,
/// `contact_manager`) and stores the vtable `010ceaf4`. Answers the agent.
pub fn fn_00d32190(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    contact_manager: u32,
) -> u32 {
    let agent = allocate_agent_block(e, 0x18);
    fn_00d32060(
        e,
        Ptr::new(agent),
        body_b,
        body_a.addr(),
        input,
        contact_manager,
    );
    e.mem.set_u32(agent, 0x010c_eaf4);
    agent
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d32110, fn_00d32110(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00d32190, fn_00d32190(Ptr, Ptr, u32, u32) -> u32),
        entry!(0x00d3_1770, hkp_height_field_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_17d0,
            hkp_height_field_agent_static_get_penetrations(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_19e0,
            hkp_height_field_agent_static_get_closest_points(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_1d30,
            hkp_height_field_agent_static_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d3_2030,
            hkp_height_field_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(0x00d3_2060, fn_00d32060(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x00d3_2150,
            hkp_height_field_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_2170,
            hkp_height_field_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_21e0, fn_00d321e0(Ptr)),
        entry!(
            0x00d3_2270,
            hk_height_field_ray_forwarding_collector_add_ray_hit(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_2370, fn_00d32370(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_23b0, fn_00d323b0(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d3_23f0, fn_00d323f0(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_24e0,
            hkp_height_field_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_27e0,
            hkp_height_field_agent_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00d3_2850,
            hkp_symmetric_agent_linear_cast_height_field_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(
            0x00d3_2970,
            hkp_symmetric_agent_linear_cast_height_field_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d3_29b0,
            hkp_symmetric_agent_linear_cast_height_field_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(
            0x00d3_29f0,
            hkp_symmetric_agent_height_field_agent_linear_cast(Ptr, Ptr, Ptr, Ptr, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_CAST_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for.
    fn install_allocator(e: &mut Engine, block: u32) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        sizes
    }

    /// A body whose shape's slot `0x2c` (the sphere count) is a double at `slot_function`
    /// answering 0.
    fn body_with_shape(e: &mut Engine, vtable: u32, slot_function: u32) -> u32 {
        e.put_vtable(vtable, &[0; 12]);
        e.mem.set_u32(vtable + 0x2c, slot_function);
        e.register(slot_function, |_, _| Ret::default());
        let shape = e.mem.alloc(8);
        e.mem.set_u32(shape, vtable);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        body
    }

    fn calls_of_00d32110(e: &Engine, address: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .count()
    }

    #[test]
    fn fn_00d32110_allocates_the_agent_and_runs_the_constructor_with_the_bodies_in_order() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let a = body_with_shape(&mut e, 0x0900_1000, 0x0900_2000);
        let b = body_with_shape(&mut e, 0x0900_1100, 0x0900_2100);
        e.call_log = Some(vec![]);
        let agent = e.call(0x00d3_2110, &args![a, b, 0u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x18]);
        assert_eq!(e.mem.u16(agent + 4), 0x18);
        assert_eq!(e.mem.u32(agent), AGENT_VTABLE);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(calls_of_00d32110(&e, 0x0900_2000), 1);
        assert_eq!(calls_of_00d32110(&e, 0x0900_2100), 0);
    }

    #[test]
    fn fn_00d32190_exchanges_the_bodies_and_stores_the_flipped_vtable() {
        let mut e = Engine::new();
        let sizes = install_allocator(&mut e, 0x0800_0000);
        let a = body_with_shape(&mut e, 0x0900_1000, 0x0900_2000);
        let b = body_with_shape(&mut e, 0x0900_1100, 0x0900_2100);
        e.call_log = Some(vec![]);
        let agent = e.call(0x00d3_2190, &args![a, b, 0u32, 0x88u32]).u32();
        assert_eq!(agent, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x18]);
        assert_eq!(e.mem.u32(agent), 0x010c_eaf4);
        assert_eq!(e.mem.u32(agent + 8), 0x88);
        assert_eq!(calls_of_00d32110(&e, 0x0900_2000), 0);
        assert_eq!(calls_of_00d32110(&e, 0x0900_2100), 1);
    }

    const SHAPE_A: u32 = 0x0900_0000;
    const SHAPE_B: u32 = 0x0910_0000;
    const COLLECTOR: u32 = 0x0920_0000;
    const MANAGER: u32 = 0x0930_0000;
    const CAST_COLLECTOR: u32 = 0x0940_0000;
    const ALLOCATOR: u32 = 0x0950_0000;
    const FLIP_SLOT: u32 = 0x0960_0000;
    const AGENT: u32 = 0x0970_0000;
    /// The sign masks the symmetric functions XOR vectors with.
    const NEGATE_ALL_MASK: u32 = 0x010c_44d0;
    const NEGATE_XYZ_MASK: u32 = 0x010c_4c00;

    /// Body A's motion (a quarter turn and a move), body B's, and what `setMulInverseMul`
    /// gives (a move).
    const ROWS_A: [Vec4; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [100.0, 200.0, 300.0, 1.0],
    ];
    const ROWS_B: [Vec4; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 2.0, 3.0, 1.0],
    ];
    const MOVE: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [10.0, 20.0, 30.0, 1.0],
    ];
    const SPHERES: [Vec4; 3] = [
        [1.0, 2.0, 3.0, 0.5],
        [4.0, 5.0, 6.0, 0.25],
        [7.0, 8.0, 9.0, 0.125],
    ];

    /// What the double of body B's slot `0x2C` saw: the request's spheres, count and
    /// tolerance, and the result buffer.
    type Requests = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The arguments of a call and the words behind some of its pointer arguments.
    type Seen = Rc<RefCell<Vec<(Vec<u32>, Vec<Vec<u32>>)>>>;

    struct Scene {
        e: Engine,
        log: Log,
        body_a: u32,
        body_b: u32,
        input: u32,
        collector: u32,
        stack: u32,
        router: u32,
        stream: u32,
        records: u32,
        requests: Requests,
    }

    /// Two bodies for the agent: A's shape has `distances.len()` spheres (from `SPHERES`), B's
    /// shape answers each with the direction `(1, 0, 0)` and the given distance. The memory
    /// router is a bump stack, `setMulInverseMul` gives `MOVE`, and the input holds the
    /// tolerance 1.0 at `+8`, the path `(4, 8, 16, 2)` at `+0x50` and 3.0 at `+0x60`.
    fn scene(distances: &[f32]) -> Scene {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x400);
        let stack = e.mem.alloc(0x1000);
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(SET_MUL_INVERSE_MUL, |e, a| {
            for (row, value) in MOVE.into_iter().enumerate() {
                store_vec(e, a[0] + 0x10 * row as u32, value);
            }
            Ret::default()
        });
        let spheres = e.mem.alloc(0x40);
        for (n, sphere) in SPHERES.into_iter().enumerate() {
            store_vec(&mut e, spheres + 0x10 * n as u32, sphere);
        }
        let shape_a = logging_object(&mut e, &log, SHAPE_A);
        log_double(&mut e, &log, SHAPE_A + 0x2c, distances.len() as u32);
        log_double(&mut e, &log, SHAPE_A + 0x30, spheres);
        let shape_b = logging_object(&mut e, &log, SHAPE_B);
        let requests: Requests = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&requests);
        let answers = distances.to_vec();
        e.register_double(SHAPE_B + 0x2c, move |e, a| {
            let request = vec![
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[1] + 8),
                a[2],
            ];
            sink.borrow_mut().push(request);
            for (n, distance) in answers.iter().enumerate() {
                store_vec(e, a[2] + 0x10 * n as u32, [1.0, 0.0, 0.0, *distance]);
            }
            Ret::default()
        });
        let collector = logging_object(&mut e, &log, COLLECTOR);
        let mut bodies = [0u32; 2];
        for (body, (shape, rows)) in bodies
            .iter_mut()
            .zip([(shape_a, ROWS_A), (shape_b, ROWS_B)])
        {
            let motion = e.mem.alloc(0x40);
            for (row, value) in rows.into_iter().enumerate() {
                store_vec(&mut e, motion + 0x10 * row as u32, value);
            }
            *body = e.mem.alloc(0x10);
            e.mem.set_u32(*body, shape);
            e.mem.set_u32(*body + 8, motion);
        }
        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 8, 1.0);
        store_vec(&mut e, input + 0x50, [4.0, 8.0, 16.0, 2.0]);
        e.mem.set_f32(input + 0x60, 3.0);
        Scene {
            e,
            log,
            body_a: bodies[0],
            body_b: bodies[1],
            input,
            collector,
            stack,
            router,
            stream,
            records,
            requests,
        }
    }

    /// The two labels of the 16-byte start record and the labels of the 12-byte records after
    /// it.
    fn stages(s: &Scene) -> ((u32, u32), Vec<u32>) {
        let start = (s.e.mem.u32(s.records), s.e.mem.u32(s.records + 0xc));
        let end = s.e.mem.u32(s.stream + 4);
        let later = (s.records + 0x10..end)
            .step_by(12)
            .map(|record| s.e.mem.u32(record))
            .collect();
        (start, later)
    }

    /// A double at `address` that answers `eax` and notes its arguments and `len` bytes (as
    /// words) behind each of its pointer arguments `blocks` (argument index, length).
    fn capture(e: &mut Engine, address: u32, eax: u32, blocks: &'static [(usize, u32)]) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            let words = blocks
                .iter()
                .map(|&(index, len)| (0..len / 4).map(|n| e.mem.u32(a[index] + 4 * n)).collect())
                .collect();
            sink.borrow_mut().push((a.to_vec(), words));
            Ret {
                eax,
                ..Ret::default()
            }
        });
        seen
    }

    /// The floats of a block of words.
    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    /// Makes the sign masks of the symmetric functions (the exe's data, absent in the tests).
    fn set_masks(e: &mut Engine) {
        for lane in 0..4u32 {
            e.set_global(NEGATE_ALL_MASK + 4 * lane, 0x8000_0000u32);
            e.set_global(
                NEGATE_XYZ_MASK + 4 * lane,
                if lane < 3 { 0x8000_0000u32 } else { 0 },
            );
        }
    }

    /// Puts a logging double in slot 1 of the flip collector `vtable`; it notes its arguments
    /// and the words at `+4` and `+8` of the collector.
    fn flip_probe(e: &mut Engine, vtable: u32) -> Rc<RefCell<Vec<(Vec<u32>, u32, u32)>>> {
        e.put_vtable(vtable, &[0, FLIP_SLOT]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(FLIP_SLOT, move |e, a| {
            sink.borrow_mut()
                .push((a.to_vec(), e.mem.u32(a[0] + 4), e.mem.u32(a[0] + 8)));
            Ret::default()
        });
        seen
    }

    #[test]
    fn cleanup_gives_back_the_ids_in_use_00d31770() {
        let mut s = scene(&[]);
        let this = logging_object(&mut s.e, &s.log, AGENT);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        let ids = s.e.mem.alloc(8);
        for (n, id) in [5u16, 0xffff, 7].into_iter().enumerate() {
            s.e.mem.set_u16(ids + 2 * n as u32, id);
        }
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u32(this + 0xc, ids);
        s.e.mem.set_u32(this + 0x10, 3);
        s.e.call(0x00d3_1770, &args![this, 0x55u32]);
        assert_eq!(
            calls(&s.log, MANAGER + 0x14),
            vec![vec![manager, 5, 0x55], vec![manager, 7, 0x55]]
        );
        assert_eq!(calls(&s.log, AGENT), vec![vec![this, 1]]);
        // Without a manager, or without ids, only the agent is deleted.
        s.log.borrow_mut().clear();
        s.e.mem.set_u32(this + 8, 0);
        s.e.call(0x00d3_1770, &args![this, 0x55u32]);
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u32(this + 0x10, 0);
        s.e.call(0x00d3_1770, &args![this, 0x55u32]);
        assert!(calls(&s.log, MANAGER + 0x14).is_empty());
        assert_eq!(calls(&s.log, AGENT).len(), 2);
    }

    #[test]
    fn static_get_penetrations_reports_the_first_sphere_below_zero_00d317d0() {
        let mut s = scene(&[0.5, -1.0, -2.0]);
        s.e.call(
            0x00d3_17d0,
            &args![s.body_a, s.body_b, s.input, s.collector],
        );
        assert_eq!(
            calls(&s.log, COLLECTOR + 4),
            vec![vec![s.collector, s.body_a, s.body_b]]
        );
        // The spheres, moved into B's space, were the request; the results follow them.
        let request = vec![s.stack, 3, 1.0f32.to_bits(), s.stack + 0x80];
        assert_eq!(*s.requests.borrow(), vec![request]);
        for (n, sphere) in SPHERES.into_iter().enumerate() {
            let moved = load_vec(&s.e, s.stack + 0x10 * n as u32);
            let expected = [
                sphere[0] + 10.0,
                sphere[1] + 20.0,
                sphere[2] + 30.0,
                sphere[3],
            ];
            assert_eq!(moved, expected);
        }
        assert_eq!(s.e.mem.u32(s.router), s.stack);
        let (start, later) = stages(&s);
        assert_eq!(start, (LABEL_HEIGHT_FIELD, LABEL_GET_SPHERES));
        assert_eq!(later, vec![LABEL_COLLIDE, LABEL_EXAMINE, LABEL_END]);
        // Zero, minus zero and NaN are not below zero.
        let mut s = scene(&[0.0, -0.0, f32::NAN]);
        s.e.call(
            0x00d3_17d0,
            &args![s.body_a, s.body_b, s.input, s.collector],
        );
        assert!(calls(&s.log, COLLECTOR + 4).is_empty());
    }

    #[test]
    fn static_get_closest_points_makes_a_contact_for_each_sphere_in_reach_00d319e0() {
        let mut s = scene(&[0.5, 3.0, 1.0]);
        let contacts = capture(&mut s.e, COLLECTOR + 4, 0, &[(1, 0x28)]);
        s.e.call(
            0x00d3_19e0,
            &args![s.body_a, s.body_b, s.input, s.collector],
        );
        let contacts = contacts.borrow();
        assert_eq!(contacts.len(), 2);
        // Sphere 0: centre (11, 22, 33), radius 0.5, direction (1, 0, 0), distance 0.5; the
        // point is (-0.5 - 0.5) * direction + centre = (10, 22, 33) in B's space, moved by B.
        let first = floats(&contacts[0].1[0]);
        assert_eq!(first[..8], [-21.0, 12.0, 36.0, 1.0, 0.0, 1.0, 0.0, 0.5]);
        assert_eq!(contacts[0].1[0][8..], [s.body_a, s.body_b]);
        // Sphere 2: centre (17, 28, 39), radius 0.125, distance 1.0 (the tolerance): the point
        // is (-0.125 - 1.0) * direction + centre.
        let third = floats(&contacts[1].1[0]);
        assert_eq!(third[..8], [-27.0, 17.875, 42.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
        let request = vec![s.stack, 3, 1.0f32.to_bits(), s.stack + 0x80];
        assert_eq!(*s.requests.borrow(), vec![request]);
        assert_eq!(s.e.mem.u32(s.router), s.stack);
        let (start, later) = stages(&s);
        assert_eq!(start, (LABEL_HEIGHT_FIELD, LABEL_CLOSEST_START));
        assert_eq!(
            later,
            vec![
                LABEL_CLOSEST_GET_SPHERES,
                LABEL_CLOSEST_TRANSFORM,
                LABEL_CLOSEST_COLLIDE,
                LABEL_CLOSEST_EXAMINE,
                LABEL_END
            ]
        );
        // A NaN distance or one above the tolerance makes none.
        let mut s = scene(&[f32::NAN, 1.5]);
        s.e.call(
            0x00d3_19e0,
            &args![s.body_a, s.body_b, s.input, s.collector],
        );
        assert!(calls(&s.log, COLLECTOR + 4).is_empty());
    }

    #[test]
    fn static_linear_cast_casts_a_ray_for_each_sphere_00d31d30() {
        let mut s = scene(&[5.0, 5.0]);
        let casts = capture(&mut s.e, SHAPE_B + 0x30, 0, &[(1, 0x38), (3, 0x48)]);
        s.e.call(
            0x00d3_1d30,
            &args![s.body_a, s.body_b, s.input, 0x7777u32, 0u32],
        );
        let casts = casts.borrow();
        assert_eq!(casts.len(), 2);
        assert_eq!(casts[0].0[2], s.body_b);
        // The ray: from the sphere in B's space, to it moved by the path (turned into B's
        // space), zero filter words, the radius and the input's float at +0x60.
        let ray = &casts[0].1[0];
        assert_eq!(
            floats(&ray[..8]),
            [11.0, 22.0, 33.0, 1.0, 19.0, 18.0, 49.0, 1.0]
        );
        assert_eq!(ray[8..10], [0, 0]);
        assert_eq!(floats(&ray[12..14]), [0.5, 3.0]);
        let second = &casts[1].1[0];
        assert_eq!(
            floats(&second[..8]),
            [14.0, 25.0, 36.0, 1.0, 22.0, 21.0, 52.0, 1.0]
        );
        assert_eq!(floats(&second[12..14]), [0.25, 3.0]);
        // The collector: vtable, 1.0, the sphere's centre in world space, its radius, the
        // path, body A and the cast collector.
        let collector = &casts[0].1[1];
        assert_eq!(collector[0], RAY_COLLECTOR_VTABLE);
        assert_eq!(floats(&collector[1..2]), [1.0]);
        assert_eq!(floats(&collector[4..8]), [98.0, 201.0, 303.0, 1.0]);
        assert_eq!(floats(&collector[8..9]), [0.5]);
        assert_eq!(floats(&collector[12..16]), [4.0, 8.0, 16.0, 2.0]);
        assert_eq!(collector[16..18], [s.body_a, 0x7777]);
        let collector = &casts[1].1[1];
        assert_eq!(floats(&collector[4..9]), [95.0, 204.0, 306.0, 1.0, 0.25]);
        assert_eq!(s.e.mem.u32(s.router), s.stack);
        assert!(s.requests.borrow().is_empty());
        let (start, later) = stages(&s);
        assert_eq!(start, (LABEL_HEIGHT_FIELD, LABEL_CAST_CLOSEST_POINTS));
        assert_eq!(
            later,
            vec![LABEL_GET_SPHERES, LABEL_CAST_SPHERES, LABEL_END]
        );
        // With a start collector the closest points are asked first.
        let mut s = scene(&[5.0]);
        s.e.call(
            0x00d3_1d30,
            &args![s.body_a, s.body_b, s.input, 0x7777u32, s.collector],
        );
        assert_eq!(s.requests.borrow().len(), 1);
        assert_eq!(calls(&s.log, SHAPE_B + 0x30).len(), 1);
    }

    #[test]
    fn the_member_queries_run_the_static_ones_00d32030_00d32150_00d32170() {
        let mut s = scene(&[-1.0]);
        s.e.call(
            0x00d3_2150,
            &args![0x77u32, s.body_a, s.body_b, s.input, s.collector],
        );
        assert_eq!(
            calls(&s.log, COLLECTOR + 4),
            vec![vec![s.collector, s.body_a, s.body_b]]
        );
        // The distance -1.0 is within the tolerance: one contact.
        s.log.borrow_mut().clear();
        s.e.call(
            0x00d3_2170,
            &args![0x77u32, s.body_a, s.body_b, s.input, s.collector],
        );
        assert_eq!(calls(&s.log, COLLECTOR + 4).len(), 1);
        s.log.borrow_mut().clear();
        s.e.call(
            0x00d3_2030,
            &args![0x77u32, s.body_a, s.body_b, s.input, 0x7777u32, 0u32],
        );
        assert_eq!(calls(&s.log, SHAPE_B + 0x30).len(), 1);
    }

    #[test]
    fn constructor_sizes_the_id_array_for_the_spheres_00d32060() {
        let mut s = scene(&[0.5, 0.5, 0.5]);
        let data = s.e.mem.alloc(0x10);
        let reserved = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reserved);
        s.e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let this = s.e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            s.e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let allocator = s.e.mem.u32(s.router + 0x10);
        let got =
            s.e.call(
                0x00d3_2060,
                &args![this, s.body_a, 0x1111u32, 0x2222u32, MANAGER],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(*reserved.borrow(), vec![vec![allocator, this + 0xc, 3, 2]]);
        assert_eq!(s.e.mem.u16(this + 6), 1);
        assert_eq!(s.e.mem.u32(this + 8), MANAGER);
        assert_eq!(s.e.mem.u32(this), AGENT_VTABLE);
        assert_eq!(s.e.mem.u32(this + 0xc), data);
        assert_eq!(s.e.mem.u32(this + 0x10), 3);
        let ids: Vec<u16> = (0..4).map(|n| s.e.mem.u16(data + 2 * n)).collect();
        assert_eq!(ids, vec![0xffff, 0xffff, 0xffff, 0]);
        // Without spheres nothing is reserved.
        let mut s = scene(&[]);
        s.e.register_double(ARRAY_RESERVE, |_, _| panic!("reserved"));
        let this = s.e.mem.alloc(0x30);
        s.e.call(0x00d3_2060, &args![this, s.body_a, 0u32, 0u32, MANAGER]);
        assert_eq!(s.e.mem.u32(this + 0x10), 0);
        // Without a contact manager the shape is not asked at all.
        s.e.call(0x00d3_2060, &args![this, s.body_a, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&s.log, SHAPE_A + 0x2c).len(), 1);
        assert_eq!(s.e.mem.u32(this + 0x14), 0x8000_0000);
        assert_eq!(s.e.mem.u32(this + 8), 0);
    }

    #[test]
    fn registration_builds_the_agent_entries_for_the_height_field_pair_00d321e0() {
        let mut s = scene(&[]);
        let seen = capture(&mut s.e, REGISTER_AGENT, 0, &[(1, 0x14)]);
        s.e.call(0x00d3_21e0, &args![0xd15bu32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].0[0], 0xd15b);
        assert_eq!(seen[0].0[2..], [0x19, 0x1a]);
        assert_eq!(
            seen[0].1[0][..5],
            [0x00d3_2190, 0x00d3_2370, 0x00d3_23b0, 0x00d3_23f0, 0x0101]
        );
        assert_eq!(seen[1].0[2..], [0x1a, 0x19]);
        assert_eq!(
            seen[1].1[0][..5],
            [0x00d3_2110, 0x00d3_17d0, 0x00d3_19e0, 0x00d3_1d30, 0x0100]
        );
    }

    #[test]
    fn add_ray_hit_forwards_a_contact_and_keeps_the_nearer_fraction_00d32270() {
        let mut s = scene(&[]);
        let cast = logging_object(&mut s.e, &s.log, CAST_COLLECTOR);
        let contacts = capture(&mut s.e, CAST_COLLECTOR + 4, 0, &[(1, 0x28)]);
        let this = s.e.mem.alloc(0x50);
        store_vec(&mut s.e, this + 0x10, [1.0, 2.0, 3.0, 0.0]);
        s.e.mem.set_f32(this + 0x20, 0.5);
        store_vec(&mut s.e, this + 0x30, [10.0, 20.0, 30.0, 0.0]);
        s.e.mem.set_u32(this + 0x40, s.body_a);
        s.e.mem.set_u32(this + 0x44, cast);
        let hit = s.e.mem.alloc(0x20);
        store_vec(&mut s.e, hit, [1.0, 0.0, 0.0, 0.0]);
        s.e.mem.set_f32(hit + 0x10, 0.5);
        // The collector's fraction 1.0 is above the cast collector's 0.75: it takes it.
        s.e.mem.set_f32(this + 4, 1.0);
        s.e.mem.set_f32(cast + 4, 0.75);
        s.e.call(0x00d3_2270, &args![this, s.body_b, hit]);
        {
            let contacts = contacts.borrow();
            assert_eq!(contacts.len(), 1);
            assert_eq!(contacts[0].0[0], cast);
            // The point is 0.5 * path + start = (6, 12, 18) moved back 0.5 along the normal
            // (1, 0, 0) turned by B's motion into (0, 1, 0); the normal's fourth lane is the
            // hit fraction.
            let contact = &contacts[0].1[0];
            assert_eq!(
                floats(&contact[..8]),
                [6.0, 11.5, 18.0, 0.0, 0.0, 1.0, 0.0, 0.5]
            );
            assert_eq!(contact[8..], [s.body_a, s.body_b]);
        }
        assert_eq!(s.e.mem.f32(this + 4), 0.75);
        // Its own 0.5 is kept when it is not above the cast collector's.
        s.e.mem.set_f32(this + 4, 0.5);
        s.e.call(0x00d3_2270, &args![this, s.body_b, hit]);
        assert_eq!(s.e.mem.f32(this + 4), 0.5);
        // Against a NaN it keeps its own; a signalling NaN of its own is quieted.
        s.e.mem.set_f32(cast + 4, f32::NAN);
        s.e.call(0x00d3_2270, &args![this, s.body_b, hit]);
        assert_eq!(s.e.mem.f32(this + 4), 0.5);
        s.e.mem.set_u32(cast + 4, 0x3f80_0000);
        s.e.mem.set_u32(this + 4, 0x7fa0_0001);
        s.e.call(0x00d3_2270, &args![this, s.body_b, hit]);
        assert_eq!(s.e.mem.u32(this + 4), 0x7fe0_0001);
    }

    #[test]
    fn process_collision_writes_contacts_and_returns_ids_00d324e0() {
        let mut s = scene(&[0.5, 5.0, f32::NAN]);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        let made = capture(&mut s.e, MANAGER + 0xc, 0x21, &[]);
        let this = s.e.mem.alloc(0x20);
        let ids = s.e.mem.alloc(8);
        for (n, id) in [0xffffu16, 7, 9].into_iter().enumerate() {
            s.e.mem.set_u16(ids + 2 * n as u32, id);
        }
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u32(this + 0xc, ids);
        s.e.mem.set_u32(this + 0x10, 3);
        let area = s.e.mem.alloc(0x100);
        let output = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(output, area);
        s.e.mem.set_u32(output + 4, 0x4444);
        s.e.call(
            0x00d3_24e0,
            &args![this, s.body_a, s.body_b, s.input, output],
        );
        // Sphere 0 has no id: the manager gives one and the contact is kept. Sphere 1 is
        // beyond the tolerance: its id goes back. Sphere 2 (NaN) keeps its id and makes one.
        assert_eq!(
            made.borrow()[0].0,
            vec![manager, s.body_a, s.body_b, s.input, output, 0, area]
        );
        assert_eq!(made.borrow().len(), 1);
        assert_eq!(
            calls(&s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x4444]]
        );
        let ids: Vec<u16> = (0..3).map(|n| s.e.mem.u16(ids + 2 * n)).collect();
        assert_eq!(ids, vec![0x21, 0xffff, 9]);
        assert_eq!(s.e.mem.u32(output), area + 0x60);
        assert_eq!(
            load_vec(&s.e, area),
            [-21.0, 12.5, 36.0, 1.0],
            "position of the first contact"
        );
        assert_eq!(load_vec(&s.e, area + 0x10), [0.0, 1.0, 0.0, 0.5]);
        assert_eq!(s.e.mem.u32(area + 0x20), 0x21);
        assert_eq!(load_vec(&s.e, area + 0x30), [-27.0, 18.875, 42.0, 1.0]);
        assert!(load_vec(&s.e, area + 0x40)[3].is_nan());
        assert_eq!(s.e.mem.u32(area + 0x50), 9);
        // The spheres were taken from body A's shape, as many as the agent has ids.
        assert!(calls(&s.log, SHAPE_A + 0x2c).is_empty());
        let request = vec![s.stack, 3, 1.0f32.to_bits(), s.stack + 0x80];
        assert_eq!(*s.requests.borrow(), vec![request]);
        assert_eq!(s.e.mem.u32(s.router), s.stack);
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_PROCESS, PROFILE_END]
        );
    }

    #[test]
    fn process_collision_does_not_advance_when_the_manager_refuses_an_id_00d324e0() {
        let mut s = scene(&[0.5]);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        capture(&mut s.e, MANAGER + 0xc, 0xffff, &[]);
        let this = s.e.mem.alloc(0x20);
        let ids = s.e.mem.alloc(8);
        s.e.mem.set_u16(ids, 0xffff);
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u32(this + 0xc, ids);
        s.e.mem.set_u32(this + 0x10, 1);
        let area = s.e.mem.alloc(0x100);
        let output = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(output, area);
        s.e.call(
            0x00d3_24e0,
            &args![this, s.body_a, s.body_b, s.input, output],
        );
        assert_eq!(s.e.mem.u16(ids), 0xffff);
        assert_eq!(s.e.mem.u32(output), area);
        assert_eq!(s.e.mem.u32(area + 0x20), 0xffff);
    }

    #[test]
    fn deleting_destructor_frees_the_owned_buffer_and_the_agent_00d327e0() {
        let mut s = scene(&[]);
        let allocator = s.e.mem.u32(s.router + 0x10);
        let this = s.e.mem.alloc(0x30);
        s.e.mem.set_u16(this + 4, 0x30);
        s.e.mem.set_u32(this + 0xc, 0x5555);
        s.e.mem.set_u32(this + 0x10, 3);
        s.e.mem.set_u32(this + 0x14, 5);
        let got = s.e.call(0x00d3_27e0, &args![this, 0xffff_ff01u32]).u32();
        assert_eq!(got, this);
        assert_eq!(
            calls(&s.log, ALLOCATOR + 0x10),
            vec![vec![allocator, 0x5555, 10]]
        );
        assert_eq!(
            calls(&s.log, ALLOCATOR + 8),
            vec![vec![allocator, this, 0x30]]
        );
        assert_eq!(s.e.mem.u32(this), BASE_VTABLE);
        assert_eq!(s.e.mem.u32(this + 0xc), 0);
        assert_eq!(s.e.mem.u32(this + 0x10), 0);
        assert_eq!(s.e.mem.u32(this + 0x14), 0x8000_0000);
        // A buffer that is not owned (negative capacity word) is not freed, nor is the agent
        // when bit 0 of the flags is clear.
        s.log.borrow_mut().clear();
        s.e.mem.set_u32(this + 0x14, 0x8000_0005);
        s.e.call(0x00d3_27e0, &args![this, 0xffff_ff00u32]);
        assert!(s.log.borrow().is_empty());
        assert_eq!(s.e.mem.u32(this + 0x14), 0x8000_0000);
    }

    #[test]
    fn symmetric_static_queries_swap_the_bodies_and_wrap_the_collector() {
        // 00d32370: getPenetrations with the flip-body collector (byte 0 at +4).
        let mut s = scene(&[-1.0]);
        let hits = flip_probe(&mut s.e, FLIP_BODY_COLLECTOR_VTABLE);
        s.e.call(0x00d3_2370, &args![s.body_b, s.body_a, s.input, 0x1234u32]);
        assert_eq!(hits.borrow().len(), 1);
        assert_eq!(hits.borrow()[0].0[1..], [s.body_a, s.body_b]);
        assert_eq!(hits.borrow()[0].1, 0);
        assert_eq!(hits.borrow()[0].2, 0x1234);
        // 00d32970: the same as a member function.
        s.e.call(
            0x00d3_2970,
            &args![0x77u32, s.body_b, s.body_a, s.input, 0x1235u32],
        );
        assert_eq!(hits.borrow().len(), 2);
        assert_eq!(hits.borrow()[1].0[1..], [s.body_a, s.body_b]);
        assert_eq!(hits.borrow()[1].2, 0x1235);
        // 00d323b0 and 00d329b0: getClosestPoints with the flip collector (the large float
        // at +4).
        let mut s = scene(&[0.5]);
        s.e.set_global(LARGE_FLOAT, 3.5f32);
        let hits = flip_probe(&mut s.e, FLIP_COLLECTOR_VTABLE);
        s.e.call(0x00d3_23b0, &args![s.body_b, s.body_a, s.input, 0x1236u32]);
        s.e.call(
            0x00d3_29b0,
            &args![0x77u32, s.body_b, s.body_a, s.input, 0x1237u32],
        );
        let hits = hits.borrow();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].1, 3.5f32.to_bits());
        assert_eq!((hits[0].2, hits[1].2), (0x1236, 0x1237));
    }

    #[test]
    fn symmetric_linear_casts_swap_the_bodies_and_negate_the_path() {
        let mut s = scene(&[5.0]);
        set_masks(&mut s.e);
        s.e.set_global(LARGE_FLOAT, 3.5f32);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        s.e.register_double(SHAPE_B + 0x30, move |e, a| {
            let collector: Vec<u32> = (0..18).map(|n| e.mem.u32(a[3] + 4 * n)).collect();
            let first: Vec<u32> = (0..9).map(|n| e.mem.u32(collector[17] + 4 * n)).collect();
            sink.borrow_mut().push((collector, first));
            Ret::default()
        });
        for (address, with_this) in [(0x00d3_23f0u32, false), (0x00d3_29f0, true)] {
            seen.borrow_mut().clear();
            let mut words = if with_this { vec![0x77u32] } else { vec![] };
            words.extend([s.body_b, s.body_a, s.input, 0x7777, 0]);
            s.e.call(address, &words);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            // The collector the static cast got holds the negated path, the flip-cast
            // collector wraps the caller's cast collector with the original path.
            assert_eq!(floats(&seen[0].0[12..16]), [-4.0, -8.0, -16.0, -2.0]);
            let first = &seen[0].1;
            assert_eq!(first[0], FLIP_CAST_COLLECTOR_VTABLE);
            assert_eq!(first[1], 3.5f32.to_bits());
            assert_eq!(floats(&first[4..8]), [4.0, 8.0, 16.0, 2.0]);
            assert_eq!(first[8], 0x7777);
            assert_eq!(seen[0].0[16], s.body_a);
        }
        // A start collector is wrapped too, and the closest points are asked with it.
        s.e.call(
            0x00d3_23f0,
            &args![s.body_b, s.body_a, s.input, 0x7777u32, 0x8888u32],
        );
        assert_eq!(s.requests.borrow().len(), 1);
    }

    #[test]
    fn symmetric_process_collision_turns_the_new_contacts_around_00d32850() {
        let mut s = scene(&[0.5]);
        set_masks(&mut s.e);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        capture(&mut s.e, MANAGER + 0xc, 5, &[]);
        let this = s.e.mem.alloc(0x20);
        let ids = s.e.mem.alloc(8);
        s.e.mem.set_u16(ids, 0xffff);
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u32(this + 0xc, ids);
        s.e.mem.set_u32(this + 0x10, 1);
        let area = s.e.mem.alloc(0x100);
        let output = s.e.mem.alloc(0x3100);
        s.e.mem.set_u32(output, area);
        s.e.call(
            0x00d3_2850,
            &args![this, s.body_b, s.body_a, s.input, output],
        );
        // The contact of the plain call is (-21, 12.5, 36, 1) with the normal (0, 1, 0, 0.5);
        // turned around, the point moves by 0.5 along the normal and the normal is negated.
        assert_eq!(load_vec(&s.e, area), [-21.0, 13.0, 36.0, 1.25]);
        assert_eq!(load_vec(&s.e, area + 0x10), [0.0, -1.0, 0.0, 0.5]);
        assert_eq!(s.e.mem.u32(output), area + 0x30);
        assert_eq!(s.e.mem.u16(ids), 5);
    }
}
