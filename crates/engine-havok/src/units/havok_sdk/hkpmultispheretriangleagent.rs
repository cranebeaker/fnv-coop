//! `Havok SDK/hkpmultispheretriangleagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMultiSphereTriangleAgent` (Xbox PDB) works on body A's multi sphere shape (`m_numSpheres`
//! at `+0x10`, the spheres from `+0x20`, 16 bytes each with the radius in the fourth lane) and
//! body B's triangle shape (radius at `+0x10`, three 16-byte vertices from `+0x20`). Its object
//! holds the contact manager at `+8`, the 16-byte triangle cache `00cd7850` works with at
//! `+0xc` and, from `+0x1c`, eight 16-bit contact point ids (`0xffff` = none), one per sphere.
//!
//! Every function first transforms the triangle's vertices by body B's transform (the matrix
//! `[B + 8]`: three columns and the translation, 16 floats) and the sphere centres by body A's
//! (SSE: `(x * column0 + y * column1) + (z * column2 + translation)` in every lane), into one
//! block on the stack, then asks `00cd7850` (`hkpCollideTriangleUtil::closestPointTriangle`)
//! for the closest point of each transformed sphere centre to the transformed triangle. The
//! block is laid out as the exe's frames are: the vertices, then the spheres, then the result
//! (a unit direction and the distance at `+0x10`), the record handed to the collector and the
//! cache of the static functions.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpmultisphereagent::{
    flip_contacts, with_flipped_record, FLIPPED_CLOSEST_VTABLE, FLIPPED_PENETRATIONS_VTABLE,
};
use super::hkpsphereshape::{load_vec, profile_record, store_vec, Vec4, PROFILE_END};

/// `hkpCollideTriangleUtil::closestPointTriangle` (`00cd7850`, cdecl: point, vertices, cache,
/// result, feature).
const CLOSEST_POINT_TRIANGLE: u32 = 0x00cd_7850;
/// Builds the triangle cache of the static functions (`00cd7160`, cdecl: vertices, cache).
const BUILD_TRIANGLE_CACHE: u32 = 0x00cd_7160;
/// Registers one group of agent functions with the dispatcher (`00cfbbe0`, `this` = the
/// dispatcher, `(functions, type a, type b)`).
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The profiling labels the member `getClosestPoints` and the static one record at their start
/// (`010ceed4`) and `processCollision` records at its start (`010ceec0`); every one of them
/// records `PROFILE_END` at its end.
const CLOSEST_POINTS_LABEL: u32 = 0x010c_eed4;
const PROCESS_COLLISION_LABEL: u32 = 0x010c_eec0;

/// The agent: the contact manager, the triangle cache and the eight contact point ids.
const MANAGER: u32 = 0x08;
const CACHE: u32 = 0x0c;
const IDS: u32 = 0x1c;
const IDS_COUNT: u32 = 8;
const NO_ID: u16 = 0xffff;
/// The contact manager's vtable slots (byte offsets): add a contact point (`0xc`: both
/// bodies, input, output, 0 and the point, returns the 16-bit id) and remove one (`0x14`: the
/// id and the output's `+4` word).
const MANAGER_ADD_POINT: u32 = 0x0c;
const MANAGER_REMOVE_POINT: u32 = 0x14;
/// The collector's vtable slot `+4`: `addPoint` of the closest points collector, `(a, b)` of the
/// penetrations one.
const COLLECTOR_SLOT: u32 = 0x04;

/// The shapes.
const SPHERE_COUNT: u32 = 0x10;
const SPHERES: u32 = 0x20;
const SPHERE_RADIUS: u32 = 0x2c;
const TRIANGLE_RADIUS: u32 = 0x10;
const TRIANGLE_VERTICES: u32 = 0x20;

/// The frame: 3 vertices, up to 16 spheres, the result, the collector record and the cache.
const FRAME_SIZE: u32 = 0x1c0;
const FRAME_VERTICES: u32 = 0x00;
const FRAME_SPHERES: u32 = 0x30;
const FRAME_RESULT: u32 = 0x130;
const RESULT_DISTANCE: u32 = 0x10;
const FRAME_RECORD: u32 = 0x160;
const FRAME_CACHE: u32 = 0x190;
/// The record's normal and the contact point id.
const RECORD_NORMAL: u32 = 0x10;
const RECORD_DISTANCE: u32 = 0x1c;
const RECORD_ID: u32 = 0x20;
/// Bytes one contact point takes in the output.
const POINT_SIZE: u32 = 0x30;

type Matrix = [Vec4; 4];

fn load_matrix(e: &Engine, at: u32) -> Matrix {
    std::array::from_fn(|column| load_vec(e, at + 16 * column as u32))
}

/// `(x * column0 + y * column1) + (z * column2 + translation)` in every lane (the fourth lane
/// of `v` is not used).
fn transform_point(matrix: &Matrix, v: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        (v[0] * matrix[0][lane] + v[1] * matrix[1][lane])
            + (v[2] * matrix[2][lane] + matrix[3][lane])
    })
}

/// Transforms the triangle's vertices (three, last first) and the sphere centres (from the
/// last one down to 0, and as the exe's do-while loop does at least once, so index -1 for no
/// spheres) into `frame`; returns the sphere count.
fn transform_into(e: &mut Engine, frame: u32, body_a: u32, body_b: u32) -> i32 {
    let multi = e.mem.u32(body_a);
    let triangle = e.mem.u32(body_b);
    let transform_b = e.mem.u32(body_b + 8);
    let matrix = load_matrix(e, transform_b);
    for vertex in (0..3).rev() {
        let at = triangle + TRIANGLE_VERTICES + 16 * vertex;
        let v = load_vec(e, at);
        store_vec(
            e,
            frame + FRAME_VERTICES + 16 * vertex,
            transform_point(&matrix, v),
        );
    }
    let transform_a = e.mem.u32(body_a + 8);
    let matrix = load_matrix(e, transform_a);
    let count = e.mem.i32(multi + SPHERE_COUNT);
    let mut index = count - 1;
    loop {
        let offset = (index as u32).wrapping_mul(16);
        let v = load_vec(e, (multi + SPHERES).wrapping_add(offset));
        let centre = transform_point(&matrix, v);
        store_vec(e, (frame + FRAME_SPHERES).wrapping_add(offset), centre);
        index -= 1;
        if index < 0 {
            break;
        }
    }
    count
}

/// `float(a + b)` through the x87 stack: the sum in double precision, rounded to a `float`
/// where the exe stores it, with the x87 choice of NaN (#199).
fn x87_sum(a: f32, b: f32) -> f32 {
    x87_add(f64::from(a), f64::from(b)) as f32
}

/// `float(a - b)` likewise.
fn x87_difference(a: f32, b: f32) -> f32 {
    x87_sub(f64::from(a), f64::from(b)) as f32
}

/// Asks `00cd7850` for the closest point of sphere `index` of `frame` to the transformed
/// triangle, with `cache`, into the frame's result; returns the distance.
fn closest_point(e: &mut Engine, frame: u32, index: u32, cache: u32) -> f32 {
    let result = frame + FRAME_RESULT;
    e.call(
        CLOSEST_POINT_TRIANGLE,
        &args![
            frame + FRAME_SPHERES + 16 * index,
            frame + FRAME_VERTICES,
            cache,
            result,
            0u32
        ],
    );
    e.mem.f32(result + RESULT_DISTANCE)
}

/// What `getPenetrations` does with the frame: stops at the first sphere closer to the
/// triangle than the sum of both radii and tells the collector (`vcall` slot 4: both bodies).
fn find_penetration(
    e: &mut Engine,
    frame: u32,
    cache: u32,
    bodies: (u32, u32),
    collector: u32,
    count: i32,
) {
    let multi = e.mem.u32(bodies.0);
    let triangle = e.mem.u32(bodies.1);
    for index in 0..count.max(0) as u32 {
        let sum = x87_sum(
            e.mem.f32(triangle + TRIANGLE_RADIUS),
            e.mem.f32(multi + SPHERE_RADIUS + 16 * index),
        );
        let distance = closest_point(e, frame, index, cache);
        if distance < sum {
            e.vcall(collector, COLLECTOR_SLOT, &args![bodies.0, bodies.1]);
            return;
        }
    }
}

/// What `getClosestPoints` does with the frame: for every sphere whose distance is below the
/// sum of the radii plus the input's tolerance (the `float` at `input + 8`), builds a record
/// (position, normal and the distance minus the radii in the normal's fourth lane) in the frame
/// and hands it to the collector (`vcall` slot 4).
fn report_closest_points(
    e: &mut Engine,
    frame: u32,
    cache: u32,
    bodies: (u32, u32),
    input: u32,
    collector: u32,
    count: i32,
) {
    let multi = e.mem.u32(bodies.0);
    let triangle = e.mem.u32(bodies.1);
    for index in 0..count.max(0) as u32 {
        let sum = x87_sum(
            e.mem.f32(triangle + TRIANGLE_RADIUS),
            e.mem.f32(multi + SPHERE_RADIUS + 16 * index),
        );
        let distance = closest_point(e, frame, index, cache);
        let threshold = x87_sum(e.mem.f32(input + 8), sum);
        if distance < threshold {
            let scale = x87_difference(e.mem.f32(triangle + TRIANGLE_RADIUS), distance);
            let record = frame + FRAME_RECORD;
            write_record(
                e,
                frame,
                index,
                record,
                scale,
                x87_difference(distance, sum),
            );
            e.vcall(collector, COLLECTOR_SLOT, &args![record]);
        }
    }
}

/// Writes the contact point record: the sphere centre moved by `scale` along the result's
/// direction (SSE), then the direction with `distance_minus_radii` in its fourth lane.
fn write_record(
    e: &mut Engine,
    frame: u32,
    index: u32,
    record: u32,
    scale: f32,
    distance_minus_radii: f32,
) {
    let direction = load_vec(e, frame + FRAME_RESULT);
    let centre = load_vec(e, frame + FRAME_SPHERES + 16 * index);
    store_vec(
        e,
        record,
        std::array::from_fn(|lane| scale * direction[lane] + centre[lane]),
    );
    store_vec(e, record + RECORD_NORMAL, direction);
    e.mem
        .set_f32(record + RECORD_DISTANCE, distance_minus_radii);
}

// Translated from 00d37e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereTriangleAgent::cleanup` (Xbox PDB): `ECX` = agent, one stack word, `owner`.
/// Asks the contact manager (`[this + 8]`, vtable slot `0x14`: id, `owner`) to remove each of
/// the eight contact point ids that is not `0xffff`, then, if the agent is not null, calls its
/// own vtable slot 0 (the scalar deleting destructor) with 1.
pub fn hkp_multi_sphere_triangle_agent_cleanup(e: &mut Engine, this: Ptr, owner: u32) {
    let me = this.addr();
    for slot in 0..IDS_COUNT {
        let id = e.mem.u16(me + IDS + 2 * slot);
        if id != NO_ID {
            let manager = e.mem.u32(me + MANAGER);
            e.vcall(manager, MANAGER_REMOVE_POINT, &args![u32::from(id), owner]);
        }
    }
    if me != 0 {
        e.vcall(me, 0, &args![1u32]);
    }
}

// Translated from 00d37ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereTriangleAgent::getPenetrations` (Xbox PDB): `ECX` = agent, four stack words
/// (`body_a`, `body_b`, `input`, `collector`). Transforms the shapes (`transform_into`) and asks
/// `00cd7850` with the agent's cache (`+0xc`) for each sphere in turn; the first one whose
/// distance (strictly) is below the sum of its radius and the triangle's radius ends it with
/// `collector`'s vtable slot 4 called with `(body_a, body_b)`.
pub fn hkp_multi_sphere_triangle_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    _input: u32,
    collector: u32,
) {
    let cache = this.addr() + CACHE;
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let count = transform_into(e, frame, body_a, body_b);
        find_penetration(e, frame, cache, (body_a, body_b), collector, count);
    });
}

// Translated from 00d38130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getPenetrations` (cdecl, four stack words `body_a`, `body_b`, `input`,
/// `collector`; no map name): as [`hkp_multi_sphere_triangle_agent_get_penetrations`], but
/// with a cache of its own in the frame that `00cd7160` builds from the triangle's vertices
/// (`shape + 0x20`, the untransformed ones) after the transforms, even with no spheres.
pub fn fn_00d38130(e: &mut Engine, body_a: u32, body_b: u32, _input: u32, collector: u32) {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let count = transform_into(e, frame, body_a, body_b);
        let triangle = e.mem.u32(body_b);
        let cache = frame + FRAME_CACHE;
        e.call(
            BUILD_TRIANGLE_CACHE,
            &args![triangle + TRIANGLE_VERTICES, cache],
        );
        find_penetration(e, frame, cache, (body_a, body_b), collector, count);
    });
}

// Translated from 00d38390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereTriangleAgent::getClosestPoints` (Xbox PDB): `ECX` = agent, four stack words
/// (`body_a`, `body_b`, `input`, `collector`). Records the profiling label `010ceed4`, then
/// transforms the shapes and, for every sphere in turn, asks `00cd7850` with the agent's cache;
/// when the distance is (strictly; unordered counts as no) below `float(tolerance + sum)`,
/// where `tolerance` is the `float` at `input + 8` and `sum` the `float` sum of the triangle's
/// and the sphere's radius, it hands a record to `collector`'s vtable slot 4:
/// position `scale * direction + centre` (`scale` = `float(triangle radius - distance)`, SSE),
/// direction at `+0x10` with `float(distance - sum)` in its fourth lane. Ends with the label
/// `010c41f8`.
pub fn hkp_multi_sphere_triangle_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    profile_record(e, CLOSEST_POINTS_LABEL);
    let cache = this.addr() + CACHE;
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let count = transform_into(e, frame, body_a, body_b);
        report_closest_points(e, frame, cache, (body_a, body_b), input, collector, count);
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d386c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getClosestPoints` (cdecl, four stack words `body_a`, `body_b`, `input`,
/// `collector`; no map name): as [`hkp_multi_sphere_triangle_agent_get_closest_points`], but
/// with a cache of its own built by `00cd7160` from the triangle's vertices after the
/// transforms (even with no spheres).
pub fn fn_00d386c0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, collector: u32) {
    profile_record(e, CLOSEST_POINTS_LABEL);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let count = transform_into(e, frame, body_a, body_b);
        let triangle = e.mem.u32(body_b);
        let cache = frame + FRAME_CACHE;
        e.call(
            BUILD_TRIANGLE_CACHE,
            &args![triangle + TRIANGLE_VERTICES, cache],
        );
        report_closest_points(e, frame, cache, (body_a, body_b), input, collector, count);
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d38b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereTriangleAgent::processCollision` (Xbox PDB): `ECX` = agent, four stack
/// words (`body_a`, `body_b`, `input`, `output`). Records the label `010ceec0`, transforms the
/// shapes and walks the spheres from the first to the last, each with the id slot counted from
/// the end (sphere `k` of `n` uses `ids[n - 1 - k]`). A sphere is close when the distance
/// from `00cd7850` is (strictly) below `float(sum + tolerance)`; then the record (as in
/// `getClosestPoints`, but with the distance minus the radii at `+0x1c`) is written at the
/// output's write pointer (`[output]`); a slot that is `0xffff` gets the id the contact
/// manager's slot `0xc` returns for `(body_a, body_b, input, output, 0, record)`, and if that is
/// still `0xffff` the point is dropped; otherwise the write pointer advances by 0x30 and the
/// record's `+0x20` word is the id. A sphere that is not close and has an id has it removed
/// through the manager's slot `0x14` with `([output + 4])` and the slot set to `0xffff`. Ends
/// with the label `010c41f8`.
pub fn hkp_multi_sphere_triangle_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    output: u32,
) {
    profile_record(e, PROCESS_COLLISION_LABEL);
    let me = this.addr();
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let count = transform_into(e, frame, body_a, body_b);
        let multi = e.mem.u32(body_a);
        let triangle = e.mem.u32(body_b);
        for index in 0..count.max(0) as u32 {
            let slot = me + IDS + 2 * (count as u32 - 1 - index);
            let sum = x87_sum(
                e.mem.f32(multi + SPHERE_RADIUS + 16 * index),
                e.mem.f32(triangle + TRIANGLE_RADIUS),
            );
            let distance = closest_point(e, frame, index, me + CACHE);
            let threshold = x87_sum(sum, e.mem.f32(input + 8));
            let manager = e.mem.u32(me + MANAGER);
            if distance < threshold {
                let scale = x87_difference(e.mem.f32(triangle + TRIANGLE_RADIUS), distance);
                let record = e.mem.u32(output);
                write_record(
                    e,
                    frame,
                    index,
                    record,
                    scale,
                    x87_difference(distance, sum),
                );
                if e.mem.u16(slot) == NO_ID {
                    let id = e
                        .vcall(
                            manager,
                            MANAGER_ADD_POINT,
                            &args![body_a, body_b, input, output, 0u32, record],
                        )
                        .eax as u16;
                    e.mem.set_u16(slot, id);
                    if id == NO_ID {
                        continue;
                    }
                }
                let write_pointer = e.mem.u32(output);
                e.mem.set_u32(output, write_pointer + POINT_SIZE);
                e.mem
                    .set_u32(record + RECORD_ID, u32::from(e.mem.u16(slot)));
            } else {
                let id = e.mem.u16(slot);
                if id != NO_ID {
                    let extra = e.mem.u32(output + 4);
                    e.vcall(manager, MANAGER_REMOVE_POINT, &args![u32::from(id), extra]);
                    e.mem.set_u16(slot, NO_ID);
                }
            }
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d389f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the multi sphere triangle agent with `dispatcher` (cdecl, one stack word; no map
/// name): two calls of `00cfbbe0` (`this` = the dispatcher) with a 0x14-byte record on the
/// stack `{create, getPenetrations, getClosestPoints, linearCast, flipped byte, predictive
/// byte}`: first `(00d37e60, 00d38ac0, 00d38ed0, 00d3ac80, 1, 0)` for the types `(3, 0x15)`,
/// then `(00d37da0, 00d38130, 00d386c0, 00daf8e0, 0, 0)` for `(0x15, 3)`. The two bytes after
/// the record's flags are not written.
pub fn fn_00d389f0(e: &mut Engine, dispatcher: u32) {
    for (functions, flipped, first, second) in [
        (
            [0x00d3_7e60u32, 0x00d3_8ac0, 0x00d3_8ed0, 0x00d3_ac80],
            1u8,
            3u32,
            0x15u32,
        ),
        (
            [0x00d3_7da0u32, 0x00d3_8130, 0x00d3_86c0, 0x00da_f8e0],
            0u8,
            0x15u32,
            3u32,
        ),
    ] {
        e.with_stack(0x14, |e, record| {
            let record = record.addr();
            for (slot, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(record + 4 * slot as u32, function);
            }
            e.mem.set_u8(record + 0x10, flipped);
            e.mem.set_u8(record + 0x11, 0);
            e.call(REGISTER_AGENT, &args![dispatcher, record, first, second]);
        });
    }
}

// Translated from 00d38a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereTriangleAgent>::getPenetrations` (engine map):
/// `ECX` = agent, four stack words. Calls [`hkp_multi_sphere_triangle_agent_get_penetrations`] on
/// the same agent with the two bodies swapped, `input`, and a record `{vtable 010ce81c, byte 0,
/// +8 = collector}` in place of the collector.
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_triangle_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    with_flipped_record(
        e,
        FLIPPED_PENETRATIONS_VTABLE,
        false,
        collector,
        |e, record| {
            hkp_multi_sphere_triangle_agent_get_penetrations(e, this, body_b, body_a, input, record)
        },
    );
}

// Translated from 00d38ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `getPenetrations` function registered for the flipped pair (cdecl, four stack words; no
/// map name): [`fn_00d38130`] with the two bodies swapped, `input`, and a record `{vtable
/// 010ce81c, byte 0, +8 = collector}` in place of the collector.
pub fn fn_00d38ac0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, collector: u32) {
    with_flipped_record(
        e,
        FLIPPED_PENETRATIONS_VTABLE,
        false,
        collector,
        |e, record| fn_00d38130(e, body_b, body_a, input, record),
    );
}

// Translated from 00d38e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereTriangleAgent>::getClosestPoints` (engine map):
/// `ECX` = agent, four stack words. Calls [`hkp_multi_sphere_triangle_agent_get_closest_points`]
/// with the two bodies swapped, `input`, and a record `{vtable 010ce7fc, float 0x7f7fffee,
/// +8 = collector}` in place of the collector.
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_triangle_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    with_flipped_record(e, FLIPPED_CLOSEST_VTABLE, true, collector, |e, record| {
        hkp_multi_sphere_triangle_agent_get_closest_points(e, this, body_b, body_a, input, record)
    });
}

// Translated from 00d38ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `getClosestPoints` function registered for the flipped pair (cdecl, four stack words; no
/// map name): [`fn_00d386c0`] with the two bodies swapped, `input`, and a record `{vtable
/// 010ce7fc, float 0x7f7fffee, +8 = collector}` in place of the collector.
pub fn fn_00d38ed0(e: &mut Engine, body_a: u32, body_b: u32, input: u32, collector: u32) {
    with_flipped_record(e, FLIPPED_CLOSEST_VTABLE, true, collector, |e, record| {
        fn_00d386c0(e, body_b, body_a, input, record)
    });
}

// Translated from 00d38f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpMultiSphereTriangleAgent>::processCollision` (engine map):
/// `ECX` = agent, four stack words (`body_a`, `body_b`, `input`, `result`). Remembers `result`'s
/// write pointer and the `float` at `result + 0x3030`, calls
/// [`hkp_multi_sphere_triangle_agent_process_collision`] with the bodies swapped, then flips the
/// points it appended and the output state exactly as the multi sphere agent's symmetric
/// function does (`flip_contacts`).
pub fn hkp_symmetric_agent_linear_cast_multi_sphere_triangle_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: u32,
    body_b: u32,
    input: u32,
    result: u32,
) {
    let saved = e.mem.f32(result + 0x3030);
    let first_point = e.mem.u32(result);
    hkp_multi_sphere_triangle_agent_process_collision(e, this, body_b, body_a, input, result);
    flip_contacts(e, result, first_point, saved);
}

// Translated from 00d37e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first is the first collidable (`body_a`), the second and third are never read (`_unused_1`, `_unused_2`) and the last is the contact manager, `manager`. Allocates 0x2c bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffffffff` as a word at +0x1c, +0x20, +0x24 and +0x28 and the vtable word `010cee4c` at +0 (replaced by `010cee88` once the copy is done). Then calls `00cd7160` with the data at +0x20 of the shape (the word at +0 of the collidable) and the address of the block + 0xc. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d38a03.
pub fn fn_00d37e60(
    e: &mut Engine,
    body_a: u32,
    _unused_1: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x2cu32]).u32();
    e.mem.set_u16(block + 4, 0x2c);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    for offset in [0x1c, 0x20, 0x24, 0x28] {
        e.mem.set_u32(block + offset, 0xffff_ffff);
    }
    e.mem.set_u32(block, 0x010c_ee4c);
    let shape = e.mem.u32(body_a);
    e.call(0x00cd_7160, &args![shape + 0x20, block + 0xc]);
    e.mem.set_u32(block, 0x010c_ee88);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d37e10,
            hkp_multi_sphere_triangle_agent_cleanup(Ptr, u32)
        ),
        entry!(
            0x00d37ed0,
            hkp_multi_sphere_triangle_agent_get_penetrations(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d38130, fn_00d38130(u32, u32, u32, u32)),
        entry!(
            0x00d38390,
            hkp_multi_sphere_triangle_agent_get_closest_points(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d386c0, fn_00d386c0(u32, u32, u32, u32)),
        entry!(
            0x00d38b00,
            hkp_multi_sphere_triangle_agent_process_collision(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00d389f0, fn_00d389f0(u32)),
        entry!(
            0x00d38a80,
            hkp_symmetric_agent_linear_cast_multi_sphere_triangle_get_penetrations(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(0x00d38ac0, fn_00d38ac0(u32, u32, u32, u32)),
        entry!(
            0x00d38e90,
            hkp_symmetric_agent_linear_cast_multi_sphere_triangle_get_closest_points(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(0x00d38ed0, fn_00d38ed0(u32, u32, u32, u32)),
        entry!(
            0x00d38f10,
            hkp_symmetric_agent_linear_cast_multi_sphere_triangle_process_collision(
                Ptr, u32, u32, u32, u32
            )
        ),
        entry!(0x00d3_7e60, fn_00d37e60(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;
    type Shared<T> = Rc<RefCell<Vec<T>>>;
    /// A collector call: its words and the 0x20 bytes of the record it was given.
    type Hit = (Vec<u32>, [Vec4; 2]);
    /// A flipped record call: its words and the three words of the record.
    type FlippedHit = (Vec<u32>, [u32; 3]);

    const AGENT_VTABLE: u32 = 0x0030_0100;
    const MANAGER_OBJECT: u32 = 0x0030_0200;
    const MANAGER_VTABLE: u32 = 0x0030_0300;
    const COLLECTOR: u32 = 0x0030_0400;
    const COLLECTOR_VTABLE: u32 = 0x0030_0500;
    const FAKE_DESTRUCTOR: u32 = 0x7000_0100;
    const FAKE_ADD_POINT: u32 = 0x7000_0200;
    const FAKE_REMOVE_POINT: u32 = 0x7000_0204;
    const FAKE_COLLECT: u32 = 0x7000_0300;
    const FAKE_FLIPPED_COLLECT: u32 = 0x7000_0304;

    /// One call of the closest point function: the sphere centre, the three vertices it was
    /// given, the cache and the feature words.
    #[derive(Debug, Clone, PartialEq)]
    struct Closest {
        sphere: Vec4,
        vertices: [Vec4; 3],
        cache: u32,
        feature: u32,
    }

    struct World {
        e: Engine,
        agent: u32,
        multi: u32,
        triangle: u32,
        body_a: u32,
        body_b: u32,
        input: u32,
        output: u32,
        closest: Shared<Closest>,
    }

    /// Body A's multi sphere shape has two spheres, `(0, -1.5, 0)` radius 0.25 and `(0, 0, 0)`
    /// radius 0.5; its transform rotates `(x, y, z)` to `(-y, x, z)` and adds `(0.5, 0, 0, 0)`,
    /// so the centres become `(2, 0, 0, 0)` and `(0.5, 0, 0, 0)`. Body B's triangle shape has
    /// radius 0.75 and vertices `(0, 0, 0)`, `(1, 0, 0)`, `(0, 1, 0)` (fourth lane 7); its
    /// transform is the same rotation with the translation `(10, 20, 30, 1)`. The input's
    /// tolerance (`+8`) is 1. The double of the closest point function answers with the
    /// direction `(0, 0, 1, 5)` and the sphere centre's first lane as the distance.
    fn world() -> World {
        let mut e = Engine::new();
        e.put_vtable(AGENT_VTABLE, &[FAKE_DESTRUCTOR]);
        e.put_vtable(
            MANAGER_VTABLE,
            &[0, 0, 0, FAKE_ADD_POINT, 0, FAKE_REMOVE_POINT],
        );
        e.mem.set_u32(MANAGER_OBJECT, MANAGER_VTABLE);
        e.put_vtable(COLLECTOR_VTABLE, &[0, FAKE_COLLECT]);
        e.map(COLLECTOR, 8);
        e.mem.set_u32(COLLECTOR, COLLECTOR_VTABLE);
        e.register(FAKE_DESTRUCTOR, |_, _| Ret::default());
        e.register(FAKE_REMOVE_POINT, |_, _| Ret::default());
        e.register(FAKE_COLLECT, |_, _| Ret::default());
        e.register(FAKE_ADD_POINT, |_, _| Ret {
            eax: 0xffff,
            ..Ret::default()
        });
        let closest: Shared<Closest> = Rc::default();
        let seen = Rc::clone(&closest);
        e.register_double(CLOSEST_POINT_TRIANGLE, move |e, a| {
            let sphere = load_vec(e, a[0]);
            seen.borrow_mut().push(Closest {
                sphere,
                vertices: [
                    load_vec(e, a[1]),
                    load_vec(e, a[1] + 16),
                    load_vec(e, a[1] + 32),
                ],
                cache: a[2],
                feature: a[4],
            });
            store_vec(e, a[3], [0.0, 0.0, 1.0, 5.0]);
            e.mem.set_f32(a[3] + RESULT_DISTANCE, sphere[0]);
            Ret::default()
        });
        e.register(BUILD_TRIANGLE_CACHE, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            Ret::default()
        });

        let multi = e.mem.alloc(0x60);
        e.mem.set_u32(multi + SPHERE_COUNT, 2);
        store_vec(&mut e, multi + SPHERES, [0.0, -1.5, 0.0, 0.25]);
        store_vec(&mut e, multi + SPHERES + 16, [0.0, 0.0, 0.0, 0.5]);
        let transform_a = e.mem.alloc(0x40);
        store_vec(&mut e, transform_a, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, transform_a + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, transform_a + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, transform_a + 0x30, [0.5, 0.0, 0.0, 0.0]);
        let body_a = e.mem.alloc(0x10);
        e.mem.set_u32(body_a, multi);
        e.mem.set_u32(body_a + 8, transform_a);

        let triangle = e.mem.alloc(0x60);
        e.mem.set_f32(triangle + TRIANGLE_RADIUS, 0.75);
        store_vec(&mut e, triangle + TRIANGLE_VERTICES, [0.0, 0.0, 0.0, 7.0]);
        store_vec(
            &mut e,
            triangle + TRIANGLE_VERTICES + 16,
            [1.0, 0.0, 0.0, 7.0],
        );
        store_vec(
            &mut e,
            triangle + TRIANGLE_VERTICES + 32,
            [0.0, 1.0, 0.0, 7.0],
        );
        let transform_b = e.mem.alloc(0x40);
        store_vec(&mut e, transform_b, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, transform_b + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, transform_b + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, transform_b + 0x30, [10.0, 20.0, 30.0, 1.0]);
        let body_b = e.mem.alloc(0x10);
        e.mem.set_u32(body_b, triangle);
        e.mem.set_u32(body_b + 8, transform_b);

        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 1.0);
        let output = e.mem.alloc(0x3100);
        e.mem.set_u32(output, output + 0x100);
        e.mem.set_u32(output + 4, 0x1234);
        let agent = e.mem.alloc(0x40);
        e.mem.set_u32(agent, AGENT_VTABLE);
        e.mem.set_u32(agent + MANAGER, MANAGER_OBJECT);
        for slot in 0..IDS_COUNT {
            e.mem.set_u16(agent + IDS + 2 * slot, NO_ID);
        }
        World {
            e,
            agent,
            multi,
            triangle,
            body_a,
            body_b,
            input,
            output,
            closest,
        }
    }

    const TRANSFORMED_TRIANGLE: [Vec4; 3] = [
        [10.0, 20.0, 30.0, 1.0],
        [10.0, 21.0, 30.0, 1.0],
        [9.0, 20.0, 30.0, 1.0],
    ];

    /// Records the collector's calls: the words and, for a call with two words, the 0x20 bytes
    /// of the record at the second one.
    fn collector_log(w: &mut World, addr: u32) -> Shared<Hit> {
        let seen: Shared<Hit> = Rc::default();
        let log = Rc::clone(&seen);
        w.e.register_double(addr, move |e, a| {
            let record = if a.len() == 2 {
                [load_vec(e, a[1]), load_vec(e, a[1] + 16)]
            } else {
                [[0.0; 4]; 2]
            };
            log.borrow_mut().push((a.to_vec(), record));
            Ret::default()
        });
        seen
    }

    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.map(super::super::hkpsphereshape::PROFILE_SLOT & !0xfff, 0x1000);
        e.set_global(super::super::hkpsphereshape::PROFILE_SLOT, 3u32);
        e.register_double(super::super::hkpsphereshape::TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    #[test]
    fn cleanup_removes_each_live_id_then_destroys_the_agent() {
        let mut w = world();
        w.e.mem.set_u16(w.agent + IDS, 3);
        w.e.mem.set_u16(w.agent + IDS + 2 * 5, 0x1234);
        w.e.mem.set_u16(w.agent + IDS + 2 * 7, 0);
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d3_7e10, &args![w.agent, 0xabcdu32]);
        let log: Log = w.e.call_log.take().unwrap();
        let removed: Vec<_> = log.iter().filter(|c| c.0 == FAKE_REMOVE_POINT).collect();
        assert_eq!(removed.len(), 3);
        assert_eq!(removed[0].1, args![MANAGER_OBJECT, 3u32, 0xabcdu32]);
        assert_eq!(removed[1].1, args![MANAGER_OBJECT, 0x1234u32, 0xabcdu32]);
        assert_eq!(removed[2].1, args![MANAGER_OBJECT, 0u32, 0xabcdu32]);
        let destroyed: Vec<_> = log.iter().filter(|c| c.0 == FAKE_DESTRUCTOR).collect();
        assert_eq!(destroyed.len(), 1);
        assert_eq!(destroyed[0].1, args![w.agent, 1u32]);
        assert_eq!(log.last().map(|c| c.0), Some(FAKE_DESTRUCTOR));
    }

    #[test]
    fn get_penetrations_without_a_hit_asks_for_every_sphere() {
        let mut w = world();
        // Sums 0.25 and 0.5; the distances are 2 and 0.5, and 0.5 < 0.5 is false.
        w.e.mem.set_f32(w.triangle + TRIANGLE_RADIUS, 0.0);
        let hits = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(
            0x00d3_7ed0,
            &args![w.agent, w.body_a, w.body_b, w.input, COLLECTOR],
        );
        assert!(hits.borrow().is_empty());
        let closest = w.closest.borrow();
        assert_eq!(closest.len(), 2);
        assert_eq!(closest[0].sphere, [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(closest[1].sphere, [0.5, 0.0, 0.0, 0.0]);
        for call in closest.iter() {
            assert_eq!(call.vertices, TRANSFORMED_TRIANGLE);
            assert_eq!(call.cache, w.agent + CACHE);
            assert_eq!(call.feature, 0);
        }
    }

    #[test]
    fn get_penetrations_stops_at_the_first_hit() {
        let mut w = world();
        // Sums 2.75 and 3: sphere 0 at distance 2 hits.
        w.e.mem.set_f32(w.triangle + TRIANGLE_RADIUS, 2.5);
        let hits = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(
            0x00d3_7ed0,
            &args![w.agent, w.body_a, w.body_b, w.input, COLLECTOR],
        );
        assert_eq!(w.closest.borrow().len(), 1);
        let hits = hits.borrow();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].0, args![COLLECTOR, w.body_a, w.body_b]);
    }

    #[test]
    fn get_penetrations_with_nan_distance_never_hits() {
        let mut w = world();
        w.e.mem.set_f32(w.multi + SPHERES, f32::NAN);
        w.e.mem.set_f32(w.multi + SPHERES + 16, f32::NAN);
        w.e.mem.set_f32(w.triangle + TRIANGLE_RADIUS, 100.0);
        let hits = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(
            0x00d3_7ed0,
            &args![w.agent, w.body_a, w.body_b, w.input, COLLECTOR],
        );
        assert!(hits.borrow().is_empty());
        assert_eq!(w.closest.borrow().len(), 2);
    }

    #[test]
    fn static_get_penetrations_builds_its_own_cache() {
        let mut w = world();
        w.e.mem.set_f32(w.triangle + TRIANGLE_RADIUS, 2.5);
        let hits = collector_log(&mut w, FAKE_COLLECT);
        w.e.call_log = Some(vec![]);
        w.e.call(0x00d3_8130, &args![w.body_a, w.body_b, w.input, COLLECTOR]);
        let log: Log = w.e.call_log.take().unwrap();
        let builds: Vec<_> = log.iter().filter(|c| c.0 == BUILD_TRIANGLE_CACHE).collect();
        assert_eq!(builds.len(), 1);
        assert_eq!(builds[0].1[0], w.triangle + TRIANGLE_VERTICES);
        let closest = w.closest.borrow();
        assert_eq!(closest.len(), 1);
        assert_eq!(closest[0].cache, builds[0].1[1]);
        assert_eq!(hits.borrow().len(), 1);
    }

    #[test]
    fn static_functions_build_the_cache_even_without_spheres() {
        let mut w = world();
        w.e.mem.set_u32(w.multi + SPHERE_COUNT, 0);
        profile_stream(&mut w.e);
        let hits = collector_log(&mut w, FAKE_COLLECT);
        for address in [0x00d3_8130u32, 0x00d3_86c0] {
            w.e.call_log = Some(vec![]);
            w.e.call(address, &args![w.body_a, w.body_b, w.input, COLLECTOR]);
            let log: Log = w.e.call_log.take().unwrap();
            assert_eq!(
                log.iter().filter(|c| c.0 == BUILD_TRIANGLE_CACHE).count(),
                1,
                "{address:08x}"
            );
        }
        assert!(w.closest.borrow().is_empty());
        assert!(hits.borrow().is_empty());
    }

    /// Tolerance 1, triangle radius 0.75: the thresholds are 2 (sum 1) and 2.25 (sum 1.25)
    /// against the distances 2 and 0.5, so only sphere 1 is reported.
    fn check_closest_points(w: &World, records: &Shared<Hit>) {
        let records = records.borrow();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].0[0], COLLECTOR);
        // scale 0.25: position (0, 0, 0.25, 1.25) + centre (0.5, 0, 0, 0).
        assert_eq!(records[0].1[0], [0.5, 0.0, 0.25, 1.25]);
        // the direction, with 0.5 - 1.25 in its fourth lane.
        assert_eq!(records[0].1[1], [0.0, 0.0, 1.0, -0.75]);
        assert_eq!(w.closest.borrow().len(), 2);
    }

    #[test]
    fn get_closest_points_reports_the_spheres_within_the_tolerance() {
        let mut w = world();
        let stream = profile_stream(&mut w.e);
        let records = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(
            0x00d3_8390,
            &args![w.agent, w.body_a, w.body_b, w.input, COLLECTOR],
        );
        check_closest_points(&w, &records);
        assert_eq!(w.closest.borrow()[0].cache, w.agent + CACHE);
        assert_eq!(w.e.mem.u32(stream + 0x20), CLOSEST_POINTS_LABEL);
        assert_eq!(w.e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(w.e.mem.u32(stream + 4), stream + 0x38);
    }

    #[test]
    fn static_get_closest_points_does_the_same_with_its_own_cache() {
        let mut w = world();
        profile_stream(&mut w.e);
        let records = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(0x00d3_86c0, &args![w.body_a, w.body_b, w.input, COLLECTOR]);
        check_closest_points(&w, &records);
        let closest = w.closest.borrow();
        assert_ne!(closest[0].cache, w.agent + CACHE);
        assert_eq!(closest[0].cache, closest[1].cache);
    }

    #[test]
    fn get_closest_points_reports_nothing_for_nan_distances() {
        let mut w = world();
        profile_stream(&mut w.e);
        w.e.mem.set_f32(w.multi + SPHERES, f32::NAN);
        w.e.mem.set_f32(w.multi + SPHERES + 16, f32::NAN);
        let records = collector_log(&mut w, FAKE_COLLECT);
        w.e.call(
            0x00d3_8390,
            &args![w.agent, w.body_a, w.body_b, w.input, COLLECTOR],
        );
        assert!(records.borrow().is_empty());
    }

    /// Drives `processCollision` over the world with sphere 1 close (distance 0.5 against the
    /// threshold 2.25) and sphere 0 not (2 against 2).
    fn process(w: &mut World) -> Log {
        let stream = profile_stream(&mut w.e);
        w.e.call_log = Some(vec![]);
        w.e.call(
            0x00d3_8b00,
            &args![w.agent, w.body_a, w.body_b, w.input, w.output],
        );
        assert_eq!(w.e.mem.u32(stream + 0x20), PROCESS_COLLISION_LABEL);
        assert_eq!(w.e.mem.u32(stream + 0x2c), PROFILE_END);
        w.e.call_log.take().unwrap()
    }

    #[test]
    fn process_collision_adds_a_point_for_a_close_sphere_without_an_id() {
        let mut w = world();
        w.e.register(FAKE_ADD_POINT, |_, _| Ret {
            eax: 0x1_0009,
            ..Ret::default()
        });
        let log = process(&mut w);
        let adds: Vec<_> = log.iter().filter(|c| c.0 == FAKE_ADD_POINT).collect();
        assert_eq!(adds.len(), 1);
        let record = w.output + 0x100;
        assert_eq!(
            adds[0].1,
            args![
                MANAGER_OBJECT,
                w.body_a,
                w.body_b,
                w.input,
                w.output,
                0u32,
                record
            ]
        );
        // Sphere 1 is the second one tried, so its slot is ids[0].
        assert_eq!(w.e.mem.u16(w.agent + IDS), 9);
        assert_eq!(w.e.mem.u32(w.output), record + POINT_SIZE);
        assert_eq!(w.e.mem.u32(record + RECORD_ID), 9);
        assert_eq!(load_vec(&w.e, record), [0.5, 0.0, 0.25, 1.25]);
        assert_eq!(
            load_vec(&w.e, record + RECORD_NORMAL),
            [0.0, 0.0, 1.0, -0.75]
        );
        assert!(log.iter().all(|c| c.0 != FAKE_REMOVE_POINT));
        let closest: Vec<_> = log
            .iter()
            .filter(|c| c.0 == CLOSEST_POINT_TRIANGLE)
            .collect();
        assert_eq!(closest.len(), 2);
        assert_eq!(closest[0].1[2], w.agent + CACHE);
    }

    #[test]
    fn process_collision_drops_the_point_when_the_manager_gives_no_id() {
        let mut w = world();
        let log = process(&mut w);
        assert_eq!(log.iter().filter(|c| c.0 == FAKE_ADD_POINT).count(), 1);
        assert_eq!(w.e.mem.u16(w.agent + IDS), NO_ID);
        assert_eq!(w.e.mem.u32(w.output), w.output + 0x100);
        // The record was written all the same.
        assert_eq!(
            load_vec(&w.e, w.output + 0x100 + RECORD_NORMAL),
            [0.0, 0.0, 1.0, -0.75]
        );
    }

    #[test]
    fn process_collision_keeps_the_id_of_a_close_sphere_and_removes_the_other() {
        let mut w = world();
        // Sphere 0 (not close) uses ids[1]; sphere 1 (close) uses ids[0].
        w.e.mem.set_u16(w.agent + IDS + 2, 7);
        w.e.mem.set_u16(w.agent + IDS, 4);
        let log = process(&mut w);
        assert!(log.iter().all(|c| c.0 != FAKE_ADD_POINT));
        let removed: Vec<_> = log.iter().filter(|c| c.0 == FAKE_REMOVE_POINT).collect();
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].1, args![MANAGER_OBJECT, 7u32, 0x1234u32]);
        assert_eq!(w.e.mem.u16(w.agent + IDS + 2), NO_ID);
        assert_eq!(w.e.mem.u16(w.agent + IDS), 4);
        let record = w.output + 0x100;
        assert_eq!(w.e.mem.u32(w.output), record + POINT_SIZE);
        assert_eq!(w.e.mem.u32(record + RECORD_ID), 4);
    }

    #[test]
    fn process_collision_without_spheres_does_nothing() {
        let mut w = world();
        w.e.mem.set_u32(w.multi + SPHERE_COUNT, 0);
        let log = process(&mut w);
        assert!(log.iter().all(|c| c.0 != CLOSEST_POINT_TRIANGLE));
        assert_eq!(w.e.mem.u32(w.output), w.output + 0x100);
    }

    #[test]
    fn registration_makes_two_records() {
        let mut e = Engine::new();
        let seen: Shared<Vec<u32>> = Rc::default();
        let log = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|word| e.mem.u32(a[1] + 4 * word)));
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d3_89f0, &args![0x5555u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][0], 0x5555);
        assert_eq!(
            seen[0][2..],
            [3, 0x15, 0xd3_7e60, 0xd3_8ac0, 0xd3_8ed0, 0xd3_ac80, 1, 0]
        );
        assert_eq!(seen[1][0], 0x5555);
        assert_eq!(
            seen[1][2..],
            [0x15, 3, 0xd3_7da0, 0xd3_8130, 0xd3_86c0, 0xda_f8e0, 0, 0]
        );
    }

    /// The flipping records' vtables with a collector function at slot 4 that notes its words
    /// and the record's three words.
    fn flipped_vtables(w: &mut World) -> Shared<FlippedHit> {
        w.e.put_vtable(FLIPPED_PENETRATIONS_VTABLE, &[0, FAKE_FLIPPED_COLLECT]);
        w.e.put_vtable(FLIPPED_CLOSEST_VTABLE, &[0, FAKE_FLIPPED_COLLECT]);
        w.e.map(
            super::super::hkpmultisphereagent::MAX_FLOAT & !0xfff,
            0x1000,
        );
        w.e.mem.set_f32(
            super::super::hkpmultisphereagent::MAX_FLOAT,
            f32::from_bits(0x7f7f_ffee),
        );
        let seen: Shared<FlippedHit> = Rc::default();
        let log = Rc::clone(&seen);
        w.e.register_double(FAKE_FLIPPED_COLLECT, move |e, a| {
            let at = a[0];
            let record = [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)];
            log.borrow_mut().push((a.to_vec(), record));
            Ret::default()
        });
        seen
    }

    #[test]
    fn flipped_get_penetrations_swap_the_bodies_and_wrap_the_collector() {
        let mut w = world();
        let seen = flipped_vtables(&mut w);
        // Swapped, body B (the triangle) is the multi sphere side: put the shapes the other
        // way round in the bodies the functions are called with.
        w.e.mem.set_f32(w.triangle + TRIANGLE_RADIUS, 2.5);
        let (first, second) = (w.body_b, w.body_a);
        w.e.call(
            0x00d3_8a80,
            &args![w.agent, first, second, w.input, COLLECTOR],
        );
        {
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            // The inner function saw (body_b, body_a) = (second, first) swapped back.
            assert_eq!(seen[0].0[1..], [second, first]);
            assert_eq!(seen[0].1, [FLIPPED_PENETRATIONS_VTABLE, 0, COLLECTOR]);
        }
        w.e.call(0x00d3_8ac0, &args![first, second, w.input, COLLECTOR]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[1].0[1..], [second, first]);
        assert_eq!(seen[1].1, [FLIPPED_PENETRATIONS_VTABLE, 0, COLLECTOR]);
    }

    #[test]
    fn flipped_get_closest_points_wrap_the_collector_with_the_maximum_float() {
        let mut w = world();
        let seen = flipped_vtables(&mut w);
        profile_stream(&mut w.e);
        let (first, second) = (w.body_b, w.body_a);
        w.e.call(
            0x00d3_8e90,
            &args![w.agent, first, second, w.input, COLLECTOR],
        );
        w.e.call(0x00d3_8ed0, &args![first, second, w.input, COLLECTOR]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        for call in seen.iter() {
            assert_eq!(call.1, [FLIPPED_CLOSEST_VTABLE, 0x7f7f_ffee, COLLECTOR]);
            assert_eq!(call.0.len(), 2);
        }
    }

    #[test]
    fn flipped_process_collision_flips_the_new_point() {
        let mut w = world();
        let mask = super::super::hkpmultisphereagent::SIGN_MASK;
        w.e.map(mask & !0xfff, 0x1000);
        for (lane, bits) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            w.e.mem.set_u32(mask + 4 * lane as u32, bits);
        }
        profile_stream(&mut w.e);
        w.e.register(FAKE_ADD_POINT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        // Swapped back, the bodies are the ones the world's functions expect.
        let (first, second) = (w.body_b, w.body_a);
        w.e.call(
            0x00d3_8f10,
            &args![w.agent, first, second, w.input, w.output],
        );
        let record = w.output + 0x100;
        assert_eq!(w.e.mem.u32(w.output), record + POINT_SIZE);
        // position (0.5, 0, 0.25, 1.25) + (-0.75) * (0, 0, 1, -0.75), normal sign flipped
        // in its first three lanes.
        assert_eq!(
            load_vec(&w.e, record),
            [0.5, 0.0, 0.25 + -0.75 * 1.0, 1.25 + -0.75 * -0.75]
        );
        let normal = load_vec(&w.e, record + RECORD_NORMAL);
        assert_eq!(
            normal.map(f32::to_bits),
            [0x8000_0000, 0x8000_0000, 0xbf80_0000, 0xbf40_0000]
        );
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
    fn fn_00d37e60_fills_the_block_and_copies_the_shape_data() {
        let (mut e, sizes) = allocator_engine();
        let shape = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cd_7160, move |e, a| {
            // The block already holds the first vtable word while the data is copied.
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1] - 0xc)));
            Ret::default()
        });
        let block = e
            .call(0x00d3_7e60, &args![body, 0x2000u32, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x2c]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![shape + 0x20, block + 0xc], 0x010c_ee4c)]
        );
        assert_eq!(e.mem.u16(block + 4), 0x2c);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_ee88);
        for offset in [0x1c, 0x20, 0x24, 0x28] {
            assert_eq!(e.mem.u32(block + offset), 0xffff_ffff);
        }
    }
}
