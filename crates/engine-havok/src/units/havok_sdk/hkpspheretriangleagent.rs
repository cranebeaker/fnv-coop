//! `Havok SDK/hkpspheretriangleagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpSphereTriangleAgent` (Xbox PDB) is the collision agent of a sphere (body A) and a
//! triangle (body B). Its base is the Xbox PDB's `hkpCollisionAgent`: contact manager `+8`,
//! the one manifold contact point id (16-bit, `0xFFFF` = none) at `+0xC` and from `+0x10` the
//! triangle's cache (made by `00cd7160`, handed on to `hkpCollideTriangleUtil::
//! closestPointTriangle`, `00cd7850`). The static (dispatcher) versions of the queries make that
//! cache on the stack instead. A body is an `hkpCdBody` (shape `+0`, motion `+8`); the sphere
//! shape holds its radius at `+0x10`, the triangle shape its radius at `+0x10` and its
//! vertices at `+0x20`, `+0x30` and `+0x40`.
//!
//! The penetration queries move the sphere's centre into the triangle's space (the inverse
//! rotation of body B's motion, lane for lane as the exe's SSE code does it) and ask the
//! closest point function for the distance; the contact queries move the triangle's vertices
//! to world space instead. The bodies touch when the distance is below the reach: the radius
//! sum (a `float`), plus the input's tolerance (`+8`) for the contact queries, rounded to a
//! `float`. The contact has the normal the closest point function answers (a direction), the
//! position `centre + (triangle radius - distance) * normal` and, in the normal's fourth lane,
//! the distance less the radius sum. When the closest point lies on a vertex or an edge (the
//! feature kind byte is not 0), the triangle shape (slot `0x40`) may veto the contact or
//! change its normal.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul, sub};
use crate::units::havok_sdk::hkpbvagent::{
    symmetric_process_collision, with_flip_body_collector, with_flip_collector,
};
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{load_matrix, transform_point};
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_vec, movhlps, movlhps, shufps, store_vec, unpckhps, unpcklps, Vec4,
};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: `"TtSphereTriangle"` and the shared end label.
const LABEL_SPHERE_TRIANGLE: u32 = 0x010c_f0d4;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpCollideTriangleUtil::closestPointTriangle` (cdecl: point, vertices, cache, result
/// (a unit direction and the distance at `+0x10`), feature or 0; answers 0 for the face).
const CLOSEST_POINT_TRIANGLE: u32 = 0x00cd_7850;
/// Makes the cache of a triangle (cdecl: the triangle's vertices, the cache).
const BUILD_TRIANGLE_CACHE: u32 = 0x00cd_7160;
/// Replaces the normal's first three lanes of a contact (`this` = the contact, the vector).
const SET_NORMAL: u32 = 0x00ca_ad60;
/// The linear cast of the bounding-volume pair, which the direct entry registers (cdecl:
/// two bodies, input, two collectors), and the symmetric static linear cast of the
/// capsule-triangle pair (`00d3ac80`) the symmetric entry registers.
const LINEAR_CAST: u32 = 0x00da_f8e0;
const SYMMETRIC_LINEAR_CAST: u32 = 0x00d3_ac80;
/// `hkpCollisionDispatcher` agent registrations (`this` = dispatcher, `RET 0xC`): `(agent
/// function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
const REGISTER_AGENT_OTHER: u32 = 0x00cf_bcc0;
/// The triangle shape's slot `0x40`: asks whether a contact on a vertex or an edge stays.
const WELD_SLOT: u32 = 0x40;

/// The frame the queries build on the stack: the sphere's centre in the triangle's space, the
/// result of the closest point function (a direction and a distance), the feature (a count
/// byte and the 16-bit vertex indices), a byte for the kind of the feature, the triangle's
/// cache, the triangle's vertices in world space, copies of a normal and a position for the
/// triangle shape, and the `hkpCdPoint` handed to the collector.
const POINT: u32 = 0;
const RESULT: u32 = 0x10;
const FEATURE: u32 = 0x30;
const KIND: u32 = 0x38;
const CACHE: u32 = 0x40;
const VERTICES: u32 = 0x50;
const NORMAL_COPY: u32 = 0x80;
const POSITION_COPY: u32 = 0x90;
const CD_POINT: u32 = 0xa0;
const FRAME_SIZE: u32 = 0xd0;

/// The sphere centre `centre` in the triangle's space: the difference to the motion's
/// translation row (`rows[3]`) times the inverse rotation. The rows are re-arranged the way
/// the exe's `UNPCKLPS`/`UNPCKHPS`/`MOVLHPS`/`MOVHLPS`/`SHUFPS` sequence does it into the
/// three columns `(r0x, r1x, r2x, r2y)`, `(r0y, r1y, r2y, r2w)` and `(r0z, r1z, r2z, r2w)`;
/// every lane is `(y * column y + x * column x) + z * column z` (the fourth lane is whatever
/// the odd columns give).
fn into_triangle_space(rows: &[Vec4; 4], centre: Vec4) -> Vec4 {
    let difference = sub(centre, rows[3]);
    let low = unpcklps(rows[0], rows[1]);
    let high = unpckhps(rows[0], rows[1]);
    let x_column = movlhps(low, rows[2]);
    let y_column = shufps(movhlps(x_column, low), rows[2], 0xd4);
    let z_column = shufps(high, rows[2], 0xe4);
    let by_y = mul([difference[1]; 4], y_column);
    let by_x = mul([difference[0]; 4], x_column);
    let by_z = mul([difference[2]; 4], z_column);
    add(add(by_y, by_x), by_z)
}

/// Moves the three vertices of the triangle of `body_b` (`shape + 0x20`) to world space by
/// its motion's four rows into the frame at `at`.
fn store_world_vertices(e: &mut Engine, at: u32, body_b: u32) {
    let shape = e.mem.u32(body_b);
    let rows = load_matrix(e, e.mem.u32(body_b + 8));
    for n in 0..3 {
        let vertex = transform_point(&rows, load_vec(e, shape + 0x20 + 0x10 * n));
        store_vec(e, at + 0x10 * n, vertex);
    }
}

/// The radius of the shape of `body`, taken through the x87 (`FLD`).
fn radius(e: &Engine, body: u32) -> f32 {
    x87_quiet(e.mem.f32(e.mem.u32(body) + 0x10))
}

/// The radius sum of the two bodies' shapes, rounded to a `float` (`FSTP`).
fn radius_sum(e: &Engine, body_a: u32, body_b: u32) -> f32 {
    (f64::from(radius(e, body_a)) + f64::from(radius(e, body_b))) as f32
}

/// The cache the query uses: the agent's (`this + 0x10`) for the virtual functions; the static
/// ones build it from the triangle's vertices into the frame.
fn cache_of(e: &mut Engine, this: Option<u32>, frame: u32, body_b: u32) -> u32 {
    match this {
        Some(this) => this + 0x10,
        None => {
            let shape = e.mem.u32(body_b);
            e.call(BUILD_TRIANGLE_CACHE, &args![shape + 0x20, frame + CACHE]);
            frame + CACHE
        }
    }
}

/// `getPenetrations`: profiled (`"TtSphereTriangle"`); the collector (slot 4, with the two
/// bodies) is told when the radius sum is above the distance of the triangle from the sphere's
/// centre (an x87 compare: a NaN is not above).
fn penetrations(e: &mut Engine, this: Option<u32>, body_a: u32, body_b: u32, collector: u32) {
    profile_record(e, LABEL_SPHERE_TRIANGLE);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let shape = e.mem.u32(body_b);
        let cache = cache_of(e, this, frame, body_b);
        let rows = load_matrix(e, e.mem.u32(body_b + 8));
        let centre = load_vec(e, e.mem.u32(body_a + 8) + 0x30);
        store_vec(e, frame + POINT, into_triangle_space(&rows, centre));
        e.call(
            CLOSEST_POINT_TRIANGLE,
            &args![frame + POINT, shape + 0x20, cache, frame + RESULT, 0u32],
        );
        let distance = x87_quiet(e.mem.f32(frame + RESULT + 0x10));
        if radius_sum(e, body_a, body_b) > distance {
            e.vcall(collector, 4, &args![body_a, body_b]);
        }
    });
    profile_record(e, LABEL_END);
}

/// What the contact queries find: the position and the normal (fourth lane: the distance
/// less the radius sum) and the answer of the closest point function.
struct Touch {
    position: Vec4,
    normal: Vec4,
    answer: u32,
}

/// The sphere-triangle contact: moves the triangle's vertices to world space into the frame,
/// asks the closest point function about the sphere's centre (`feature`: where it tells what
/// the closest point lies on, or 0) and answers the contact when the reach (the radius sum
/// plus the input's tolerance at `+8`, as a `float`) is above the distance. The position is
/// `centre + (triangle radius - distance) * direction`, the normal the direction with the
/// distance less the radius sum in the fourth lane (the x87 sums are done in `f64` and rounded
/// to a `float` where the exe stores one).
fn touch(
    e: &mut Engine,
    frame: u32,
    bodies: (u32, u32),
    input: u32,
    cache: u32,
    feature: u32,
) -> Option<Touch> {
    let (body_a, body_b) = bodies;
    store_world_vertices(e, frame + VERTICES, body_b);
    let centre_at = e.mem.u32(body_a + 8) + 0x30;
    let answer = e
        .call(
            CLOSEST_POINT_TRIANGLE,
            &args![centre_at, frame + VERTICES, cache, frame + RESULT, feature],
        )
        .u32();
    let sum = radius_sum(e, body_a, body_b);
    let distance = x87_quiet(e.mem.f32(frame + RESULT + 0x10));
    let tolerance = x87_quiet(e.mem.f32(input + 8));
    let reach = (f64::from(tolerance) + f64::from(sum)) as f32;
    if reach > distance {
        let inside = (f64::from(radius(e, body_b)) - f64::from(distance)) as f32;
        let mut normal = load_vec(e, frame + RESULT);
        let position = add(mul([inside; 4], normal), load_vec(e, centre_at));
        normal[3] = (f64::from(distance) - f64::from(sum)) as f32;
        Some(Touch {
            position,
            normal,
            answer,
        })
    } else {
        None
    }
}

/// Asks the triangle shape (slot `0x40`, the shape as `this`) about the contact whose position
/// and normal are at `position` and `normal`: the feature's vertex indices (`+2` of the
/// feature), the kind byte in the frame, a copy of the position (the shape may not change
/// it), body B's motion, the sphere's shape and motion and a copy of the normal in the frame,
/// which the shape may change. Answers the shape's answer (0 drop the contact, 1 take the
/// changed vector).
fn ask_feature(e: &mut Engine, frame: u32, bodies: (u32, u32), position: u32, normal: u32) -> u32 {
    let (body_a, body_b) = bodies;
    copy_vec(e, normal, frame + NORMAL_COPY);
    copy_vec(e, position, frame + POSITION_COPY);
    let shape_a = e.mem.u32(body_a);
    let motion_a = e.mem.u32(body_a + 8);
    let shape_b = e.mem.u32(body_b);
    let motion_b = e.mem.u32(body_b + 8);
    e.vcall(
        shape_b,
        WELD_SLOT,
        &args![
            frame + FEATURE + 2,
            frame + KIND,
            frame + POSITION_COPY,
            motion_b,
            shape_a,
            motion_a,
            frame + NORMAL_COPY
        ],
    )
    .u32()
}

/// `getClosestPoints`: profiled (`"TtSphereTriangle"`). `this` gives the cache (`this + 0x10`)
/// of the virtual version, which does not ask the feature of the closest point; the static
/// version builds the cache, and when the input's word `+4` is set and the feature kind is not
/// 0 asks the triangle shape about the contact: answer 0 drops it, answer 1 puts the vector
/// it changed into the contact's normal (`00caad60`). The contact is built as an `hkpCdPoint`
/// (position, normal, the two bodies at `+0x20` and `+0x24`) and passed to the collector
/// (slot 4).
fn closest_points(
    e: &mut Engine,
    this: Option<u32>,
    bodies: (u32, u32),
    input: u32,
    collector: u32,
) {
    profile_record(e, LABEL_SPHERE_TRIANGLE);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let cache = cache_of(e, this, frame, bodies.1);
        let feature = if this.is_none() { frame + FEATURE } else { 0 };
        let Some(found) = touch(e, frame, bodies, input, cache, feature) else {
            return;
        };
        let point = frame + CD_POINT;
        store_vec(e, point, found.position);
        store_vec(e, point + 0x10, found.normal);
        e.mem.set_u32(point + 0x20, bodies.0);
        e.mem.set_u32(point + 0x24, bodies.1);
        if this.is_none() {
            let kind = e.mem.u8(frame + FEATURE);
            e.mem.set_u8(frame + KIND, kind);
            if e.mem.u32(input + 4) != 0 && kind != 0 {
                match ask_feature(e, frame, bodies, point, point + 0x10) {
                    0 => return,
                    1 => {
                        e.call(SET_NORMAL, &args![point, frame + NORMAL_COPY]);
                    }
                    _ => {}
                }
            }
        }
        e.vcall(collector, 4, &args![point]);
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d3d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereTriangleAgent::cleanup` (Xbox PDB): gives the contact point id (`+0xC`) back to
/// the contact manager (`[this + 8]`, slot `0x14`, with the id and `param`) when it is not
/// `0xFFFF`, then runs the agent's deleting destructor (slot 0, argument 1).
pub fn hkp_sphere_triangle_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let id = e.mem.u16(this.addr() + 0xc);
    if id != 0xffff {
        let manager = e.mem.u32(this.addr() + 8);
        e.vcall(manager, 0x14, &args![u32::from(id), param]);
    }
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d3d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereTriangleAgent::getPenetrations` (Xbox PDB): the sphere's centre goes into the
/// triangle's space, `00cd7850` finds its distance with the agent's cache (`this + 0x10`) and
/// the collector (slot 4, with the two bodies) is told when the radius sum is above it. The
/// word `input` is not used. Profiled (`"TtSphereTriangle"`).
pub fn hkp_sphere_triangle_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    _input: u32,
    collector: Ptr,
) {
    penetrations(
        e,
        Some(this.addr()),
        body_a.addr(),
        body_b.addr(),
        collector.addr(),
    );
}

// Translated from 00d3d3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getPenetrations` of the sphere-triangle pair (the map gives it no name; cdecl:
/// the bodies, a word it does not read and the collector): as `00d3d2a0` with the triangle's
/// cache built on the stack by `00cd7160`.
pub fn fn_00d3d3e0(e: &mut Engine, body_a: Ptr, body_b: Ptr, _input: u32, collector: Ptr) {
    penetrations(e, None, body_a.addr(), body_b.addr(), collector.addr());
}

// Translated from 00d3d520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sphere-triangle contact written to a 0x20-byte contact (the map gives it no name;
/// cdecl, no profiling; `cache` is the triangle's cache): when the bodies touch (see `touch`)
/// writes the position at `out` and the normal (fourth lane: the distance less the radius
/// sum) at `out + 0x10` and answers 1, or 2 when the closest point function's answer is not
/// 0 (the closest point is on a vertex or an edge); else answers 0 and leaves `out` alone.
pub fn fn_00d3d520(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cache: Ptr,
    out: Ptr,
) -> u32 {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let bodies = (body_a.addr(), body_b.addr());
        match touch(e, frame, bodies, input.addr(), cache.addr(), 0) {
            Some(found) => {
                store_vec(e, out.addr() + 0x10, found.normal);
                store_vec(e, out.addr(), found.position);
                1 + u32::from(found.answer != 0)
            }
            None => 0,
        }
    })
}

// Translated from 00d3d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereTriangleAgent::getClosestPoints` (Xbox PDB): see `closest_points` (the virtual
/// version, with the agent's cache; it does not ask the triangle shape).
pub fn hkp_sphere_triangle_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    closest_points(
        e,
        Some(this.addr()),
        (body_a.addr(), body_b.addr()),
        input.addr(),
        collector.addr(),
    );
}

// Translated from 00d3d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `getClosestPoints` of the sphere-triangle pair (the map gives it no name; cdecl):
/// see `closest_points` (the cache is built on the stack and the triangle shape is asked about
/// contacts on a vertex or an edge).
pub fn fn_00d3d8c0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    closest_points(
        e,
        None,
        (body_a.addr(), body_b.addr()),
        input.addr(),
        collector.addr(),
    );
}

/// Fills the 20-byte agent function entry the registrations build on the stack: the four
/// function words and the two flag bytes (`+0x10`, `+0x11`). The two bytes after them are not
/// set by the exe (the callee copies the word they are in); here they are zero. Answers the
/// entry's address, as the exe's fillers do in `EAX`.
fn fill_entry(e: &mut Engine, entry: u32, words: [u32; 4], flags: [u8; 2]) -> u32 {
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(entry + 4 * index as u32, word);
    }
    e.mem.set_u8(entry + 0x10, flags[0]);
    e.mem.set_u8(entry + 0x11, flags[1]);
    entry
}

// Translated from 00d3dbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function entry (the map gives the function no name; cdecl, `EAX` holds the
/// entry on return) with the direct functions of the sphere-triangle pair: `createFunc`
/// `00d3d1a0`, `getPenetrations` `00d3d3e0`, `getClosestPoints` `00d3d8c0`, `linearCast`
/// `00daf8e0`; both flag bytes (`+0x10`, `+0x11`) 0.
pub fn fn_00d3dbe0(e: &mut Engine, entry: Ptr) -> u32 {
    fill_entry(
        e,
        entry.addr(),
        [0x00d3_d1a0, 0x00d3_d3e0, 0x00d3_d8c0, LINEAR_CAST],
        [0, 0],
    )
}

// Translated from 00d3dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function entry (the map gives the function no name; cdecl, `EAX` holds the
/// entry on return) with the symmetric functions: `createFunc` `00d3d240`, `getPenetrations`
/// `00d3dd50`, `getClosestPoints` `00d3e110`, `linearCast` `00d3ac80`; the flag bytes `+0x10`
/// = 1 and `+0x11` = 0.
pub fn fn_00d3dc10(e: &mut Engine, entry: Ptr) -> u32 {
    fill_entry(
        e,
        entry.addr(),
        [0x00d3_d240, 0x00d3_dd50, 0x00d3_e110, SYMMETRIC_LINEAR_CAST],
        [1, 0],
    )
}

/// The registration of both entries with a dispatcher through `register` (`this` = the
/// dispatcher, `(entry, type a, type b)`): the symmetric entry for the types (3, 1), then the
/// direct entry for (1, 3), both built in the same 20-byte block on the stack. The exe zeroes
/// the two flag bytes before each filler, which sets them anyway; bytes `+0x12` and `+0x13`
/// are not set by the exe (the callee copies the word they are in); here they are zero.
fn register_entries(e: &mut Engine, dispatcher: Ptr, register: u32) {
    e.with_stack(0x14, |e, entry| {
        fn_00d3dc10(e, entry);
        e.call(register, &args![dispatcher, entry, 3u32, 1u32]);
        fn_00d3dbe0(e, entry);
        e.call(register, &args![dispatcher, entry, 1u32, 3u32]);
    });
}

// Translated from 00d3dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the sphere-triangle agent with a dispatcher (the map gives it no name): the
/// symmetric entry (`00d3dc10`) for the shape types (3, 1) and the direct entry (`00d3dbe0`)
/// for (1, 3), through `00cfbbe0`.
pub fn fn_00d3dc40(e: &mut Engine, dispatcher: Ptr) {
    register_entries(e, dispatcher, REGISTER_AGENT);
}

// Translated from 00d3dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d3dc40`, through the other registration function `00cfbcc0`.
pub fn fn_00d3dca0(e: &mut Engine, dispatcher: Ptr) {
    register_entries(e, dispatcher, REGISTER_AGENT_OTHER);
}

// Translated from 00d3dd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function that does nothing (the map gives it no name): `this` and four words, which it
/// pops (`RET 0x10`).
pub fn fn_00d3dd00(_e: &mut Engine, _this: Ptr, _a: u32, _b: u32, _c: u32, _d: u32) {}

// Translated from 00d3dd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereTriangleAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpSphereTriangleAgent::getPenetrations` with the bodies swapped and, in place of the
/// caller's collector word `output`, a flip-body collector (vtable `010ce81c`, byte 0 at
/// `+4`, the caller's word at `+8`). `this` goes on in `ECX`.
pub fn hkp_symmetric_agent_linear_cast_sphere_triangle_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_sphere_triangle_agent_get_penetrations(
            e,
            this,
            body_b,
            body_a,
            input.addr(),
            Ptr::new(flip),
        );
    });
}

// Translated from 00d3dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the sphere-triangle pair (the map gives it no
/// name; cdecl): runs `00d3d3e0` with the bodies swapped and a flip-body collector around the
/// caller's word `output`.
pub fn fn_00d3dd50(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        fn_00d3d3e0(e, body_b, body_a, input.addr(), Ptr::new(flip));
    });
}

// Translated from 00d3dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereTriangleAgent::processCollision` (Xbox PDB): profiled. The record to fill is the
/// output's first free contact point (`[output]`, 0x30 bytes). When the bodies touch (see
/// `touch`, with the agent's cache and the feature asked for) the contact is written there;
/// when the feature kind is not 0 the triangle shape (slot `0x40`) is asked about the contact
/// (unlike the static `getClosestPoints`, whatever the input says): answer 0 drops it (the
/// contact point id is given back as below and the function returns without its closing
/// profile record), answer 1 puts the vector it changed into the first three lanes of the
/// normal. Then, when the agent has no contact point id (`+0xC` is `0xFFFF`), the contact
/// manager (`[this + 8]`) is asked for one through its slot `0xC` (with the two bodies, the
/// input, the output, 0 and the record; its answer is the 16-bit id). With an id, the record's
/// `+0x20` holds it and `[output]` moves on by `0x30`. When the bodies are apart and an id is
/// held, the manager's slot `0x14` takes it back (with `[output + 4]`) and the id is cleared.
pub fn hkp_sphere_triangle_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, LABEL_SPHERE_TRIANGLE);
    let id_at = this.addr() + 0xc;
    let record = e.mem.u32(output.addr());
    let bodies = (body_a.addr(), body_b.addr());
    let finished = e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let cache = this.addr() + 0x10;
        let Some(found) = touch(e, frame, bodies, input.addr(), cache, frame + FEATURE) else {
            release_id(e, this.addr(), output.addr());
            return true;
        };
        store_vec(e, record, found.position);
        store_vec(e, record + 0x10, found.normal);
        let kind = e.mem.u8(frame + FEATURE);
        e.mem.set_u8(frame + KIND, kind);
        if kind != 0 {
            match ask_feature(e, frame, bodies, record, record + 0x10) {
                0 => {
                    release_id(e, this.addr(), output.addr());
                    return false;
                }
                1 => {
                    let changed = load_vec(e, frame + NORMAL_COPY);
                    let normal = load_vec(e, record + 0x10);
                    let kept = [changed[0], changed[1], changed[2], normal[3]];
                    store_vec(e, record + 0x10, kept);
                }
                _ => {}
            }
        }
        if e.mem.u16(id_at) == 0xffff {
            let manager = e.mem.u32(this.addr() + 8);
            let id = e
                .vcall(
                    manager,
                    0xc,
                    &args![body_a, body_b, input, output, 0u32, record],
                )
                .u16();
            e.mem.set_u16(id_at, id);
        }
        let id = e.mem.u16(id_at);
        if id != 0xffff {
            e.mem.set_u32(record + 0x20, u32::from(id));
            let free = e.mem.u32(output.addr());
            e.mem.set_u32(output.addr(), free.wrapping_add(0x30));
        }
        true
    });
    if finished {
        profile_record(e, LABEL_END);
    }
}

/// Gives the agent's contact point id back to the contact manager (`[this + 8]`, slot `0x14`,
/// with the id and `[output + 4]`) and clears it, when there is one.
fn release_id(e: &mut Engine, this: u32, output: u32) {
    let id = e.mem.u16(this + 0xc);
    if id != 0xffff {
        let manager = e.mem.u32(this + 8);
        let second = e.mem.u32(output + 4);
        e.vcall(manager, 0x14, &args![u32::from(id), second]);
        e.mem.set_u16(this + 0xc, 0xffff);
    }
}

// Translated from 00d3e0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereTriangleAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpSphereTriangleAgent::getClosestPoints` with the bodies swapped and a flip collector
/// (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word
/// `output` at `+8`). `this` goes on in `ECX`.
pub fn hkp_symmetric_agent_linear_cast_sphere_triangle_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_sphere_triangle_agent_get_closest_points(
            e,
            this,
            body_b,
            body_a,
            input,
            Ptr::new(flip),
        );
    });
}

// Translated from 00d3e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the sphere-triangle pair (the map gives it no
/// name; cdecl): runs `00d3d8c0` with the bodies swapped and a flip collector around the
/// caller's word `output`.
pub fn fn_00d3e110(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        fn_00d3d8c0(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3e150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereTriangleAgent>::processCollision` (Xbox PDB): runs
/// `hkpSphereTriangleAgent::processCollision` with the bodies swapped, then turns what it
/// appended to the output around, as `00d46220` does for the bounding-volume pair.
pub fn hkp_symmetric_agent_linear_cast_sphere_triangle_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        hkp_sphere_triangle_agent_process_collision(e, this, body_b, body_a, input, output);
    });
}

// Translated from 00d3d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first is the first collidable (`body_a`), the second and third are never read (`_unused_1`, `_unused_2`) and the last is the contact manager, `manager`. Allocates 0x20 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc and the vtable word `010cf060` at +0 (replaced by `010cf09c` once the copy is done). Then calls `00cd7160` with the data at +0x20 of the shape (the word at +0 of the collidable) and the address of the block + 0x10. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3dc14.
pub fn fn_00d3d240(
    e: &mut Engine,
    body_a: u32,
    _unused_1: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x20u32]).u32();
    e.mem.set_u16(block + 4, 0x20);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    e.mem.set_u16(block + 0xc, 0xffff);
    e.mem.set_u32(block, 0x010c_f060);
    let shape = e.mem.u32(body_a);
    e.call(0x00cd_7160, &args![shape + 0x20, block + 0x10]);
    e.mem.set_u32(block, 0x010c_f09c);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_d200, hkp_sphere_triangle_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_d2a0,
            hkp_sphere_triangle_agent_get_penetrations(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d3_d3e0, fn_00d3d3e0(Ptr, Ptr, u32, Ptr)),
        entry!(0x00d3_d520, fn_00d3d520(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d3_d6b0,
            hkp_sphere_triangle_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_d8c0, fn_00d3d8c0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d3_dbe0, fn_00d3dbe0(Ptr) -> u32),
        entry!(0x00d3_dc10, fn_00d3dc10(Ptr) -> u32),
        entry!(0x00d3_dc40, fn_00d3dc40(Ptr)),
        entry!(0x00d3_dca0, fn_00d3dca0(Ptr)),
        entry!(0x00d3_dd00, fn_00d3dd00(Ptr, u32, u32, u32, u32)),
        entry!(
            0x00d3_dd10,
            hkp_symmetric_agent_linear_cast_sphere_triangle_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_dd50, fn_00d3dd50(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_dd90,
            hkp_sphere_triangle_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_e0d0,
            hkp_symmetric_agent_linear_cast_sphere_triangle_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_e110, fn_00d3e110(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_e150,
            hkp_symmetric_agent_linear_cast_sphere_triangle_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(0x00d3_d240, fn_00d3d240(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };
    use crate::units::havok_sdk::hkpcapsulecapsuleagent::fixtures::{
        shaped_body, vector_double, Seen, TURN,
    };
    use crate::units::havok_sdk::hkpspherecapsuleagent::fixtures::{
        floats, primed_engine, sphere_body,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const COLLECTOR: u32 = 0x0910_0000;
    const OBJECT: u32 = 0x0920_0000;
    /// The address the triangle shape's slot `0x40` points to in the tests.
    const WELD: u32 = 0x0930_0040;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;

    /// The sphere's centre (radius 1) and the triangle's vertices in its own space (radius
    /// 0.5); the triangle's motion is a quarter turn about z and a move by `(10, 20, 30)`.
    const CENTRE: Vec4 = [4.0, 5.0, 6.0, 1.0];
    const TRIANGLE: [Vec4; 3] = [
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
    ];
    /// The vertices in world space, and the centre in the triangle's space.
    const WORLD_TRIANGLE: [Vec4; 3] = [
        [10.0, 20.0, 30.0, 1.0],
        [10.0, 22.0, 30.0, 1.0],
        [8.0, 20.0, 30.0, 1.0],
    ];
    const LOCAL_CENTRE: Vec4 = [-15.0, 6.0, -24.0, 0.0];

    /// What the double of `closestPointTriangle` answers: the direction and distance it
    /// writes, the feature kind byte and the two vertex indices (when asked for a feature)
    /// and its answer.
    #[derive(Clone, Copy)]
    struct Hit {
        direction: Vec4,
        distance: f32,
        answer: u32,
        kind: u8,
        indices: [u16; 2],
    }

    /// The closest point is over the face, at distance 1 (the radius sum is 1.5).
    const FACE: Hit = Hit {
        direction: [0.0, 0.6, 0.8, 0.0],
        distance: 1.0,
        answer: 0,
        kind: 0,
        indices: [0, 0],
    };
    /// The closest point is on the edge between the vertices 1 and 2.
    const EDGE: Hit = Hit {
        kind: 2,
        indices: [1, 2],
        answer: 1,
        ..FACE
    };

    /// The bodies, the input and the thread's profile stream, with the doubles of
    /// `closestPointTriangle` (noting the vectors behind its first two arguments: the point
    /// and the three vertices) and of the cache builder.
    struct Scene {
        e: Engine,
        log: Log,
        a: u32,
        b: u32,
        input: u32,
        stream: u32,
        records: u32,
        seen: Seen,
    }

    fn scene(hit: Hit, tolerance: f32) -> Scene {
        let mut e = primed_engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let a = sphere_body(&mut e, 1.0, [4.0, 5.0, 6.0]);
        let b = shaped_body(&mut e, 0.5, &TRIANGLE, TURN);
        let vtable = e.mem.alloc(0x44);
        e.mem.set_u32(vtable + WELD_SLOT, WELD);
        let shape = e.mem.u32(b);
        e.mem.set_u32(shape, vtable);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, tolerance);
        let seen = vector_double(
            &mut e,
            &log,
            CLOSEST_POINT_TRIANGLE,
            &[(0, 1), (1, 3)],
            hit.answer,
            move |e, a| {
                store_vec(e, a[3], hit.direction);
                e.mem.set_f32(a[3] + 0x10, hit.distance);
                if a[4] != 0 {
                    e.mem.set_u8(a[4], hit.kind);
                    e.mem.set_u16(a[4] + 2, hit.indices[0]);
                    e.mem.set_u16(a[4] + 4, hit.indices[1]);
                }
            },
        );
        log_double(&mut e, &log, BUILD_TRIANGLE_CACHE, 0);
        Scene {
            e,
            log,
            a,
            b,
            input,
            stream,
            records,
            seen,
        }
    }

    fn check_profile(s: &Scene) {
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_SPHERE_TRIANGLE, LABEL_END]
        );
    }

    /// The triangle shape of the scene.
    fn shape_of(s: &Scene) -> u32 {
        s.e.mem.u32(s.b)
    }

    /// What the weld double of slot `0x40` was asked.
    struct Asked {
        words: Vec<u32>,
        kind: u8,
        indices: [u16; 2],
        position: Vec4,
        normal: Vec4,
    }

    type AskedLog = Rc<RefCell<Vec<Asked>>>;

    /// Makes the triangle shape's slot `0x40` answer `answer` after changing the normal copy
    /// to `changed`.
    fn weld_double(s: &mut Scene, answer: u32, changed: Vec4) -> AskedLog {
        let asked: AskedLog = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&asked);
        s.e.register_double(WELD, move |e, a| {
            copy.borrow_mut().push(Asked {
                words: a.to_vec(),
                kind: e.mem.u8(a[2]),
                indices: [e.mem.u16(a[1]), e.mem.u16(a[1] + 2)],
                position: load_vec(e, a[3]),
                normal: load_vec(e, a[7]),
            });
            store_vec(e, a[7], changed);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        asked
    }

    /// The contact `FACE`-like hits give: the position `centre + (0.5 - distance) * direction`
    /// and the normal with `distance - 1.5` in the fourth lane.
    fn expected(hit: Hit) -> [f32; 8] {
        let inside = 0.5 - hit.distance;
        let mut contact = [0.0; 8];
        for lane in 0..4 {
            contact[lane] = inside * hit.direction[lane] + CENTRE[lane];
            contact[4 + lane] = hit.direction[lane];
        }
        contact[7] = hit.distance - 1.5;
        contact
    }

    #[test]
    fn cleanup_gives_the_id_back_and_deletes_00d3d200() {
        for (id, param) in [(7u16, 0x55u32), (0xffff, 0x66)] {
            let mut e = primed_engine();
            let log = new_log();
            let manager = logging_object(&mut e, &log, MANAGER);
            let this = logging_object(&mut e, &log, OBJECT);
            e.mem.set_u32(this + 8, manager);
            e.mem.set_u16(this + 0xc, id);
            e.call(0x00d3_d200, &args![this, param]);
            let taken = calls(&log, MANAGER + 0x14);
            if id == 0xffff {
                assert!(taken.is_empty());
            } else {
                assert_eq!(taken, vec![vec![manager, 7, 0x55]]);
            }
            assert_eq!(calls(&log, OBJECT), vec![vec![this, 1]]);
        }
    }

    /// Runs the penetration query `address` for a closest point at `distance` and checks the
    /// collector was told about the bodies exactly when `told`, and what the closest point
    /// function was asked.
    fn check_penetration(address: u32, distance: f32, virtual_method: bool, told: bool) {
        let mut s = scene(Hit { distance, ..FACE }, 0.0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let this = s.e.mem.alloc(0x40);
        let words = if virtual_method {
            vec![this, s.a, s.b, 0xcace, collector]
        } else {
            vec![s.a, s.b, 0xcace, collector]
        };
        s.e.call(address, &words);
        check_profile(&s);
        let slot = calls(&s.log, COLLECTOR + 4);
        if told {
            assert_eq!(slot, vec![vec![collector, s.a, s.b]]);
        } else {
            assert!(slot.is_empty());
        }
        let vertices = shape_of(&s) + 0x20;
        let asked = calls(&s.log, CLOSEST_POINT_TRIANGLE);
        assert_eq!(asked.len(), 1);
        assert_eq!(asked[0][1], vertices);
        assert_eq!(asked[0][4], 0);
        let built = calls(&s.log, BUILD_TRIANGLE_CACHE);
        if virtual_method {
            assert_eq!(asked[0][2], this + 0x10);
            assert!(built.is_empty());
        } else {
            assert_eq!(built, vec![vec![vertices, asked[0][2]]]);
        }
        assert_eq!(
            *s.seen.borrow(),
            vec![vec![LOCAL_CENTRE, TRIANGLE[0], TRIANGLE[1], TRIANGLE[2]]]
        );
    }

    #[test]
    fn get_penetrations_tells_the_collector_of_touching_bodies_00d3d2a0() {
        check_penetration(0x00d3_d2a0, 1.0, true, true);
        check_penetration(0x00d3_d2a0, 2.0, true, false);
        // A radius sum equal to the distance is not penetrating, and a NaN is not above.
        check_penetration(0x00d3_d2a0, 1.5, true, false);
        check_penetration(0x00d3_d2a0, f32::NAN, true, false);
    }

    #[test]
    fn static_get_penetrations_builds_the_cache_00d3d3e0() {
        check_penetration(0x00d3_d3e0, 1.0, false, true);
        check_penetration(0x00d3_d3e0, 2.0, false, false);
        check_penetration(0x00d3_d3e0, 1.5, false, false);
        check_penetration(0x00d3_d3e0, f32::NAN, false, false);
    }

    /// Runs the contact function with a hit and a tolerance; answers its answer and the
    /// contact it left (`-7` where it did not write).
    fn run_contact(hit: Hit, tolerance: f32) -> (u32, [f32; 8], Scene) {
        let mut s = scene(hit, tolerance);
        let out = s.e.mem.alloc(0x20);
        for lane in 0..8 {
            s.e.mem.set_f32(out + 4 * lane, -7.0);
        }
        let answer =
            s.e.call(0x00d3_d520, &args![s.a, s.b, s.input, 0x5555u32, out])
                .u32();
        let written = floats(&s.e, out);
        (answer, written, s)
    }

    #[test]
    fn a_contact_over_the_face_is_written_00d3d520() {
        let (answer, written, s) = run_contact(FACE, 0.0);
        assert_eq!(answer, 1);
        assert_eq!(written, expected(FACE));
        assert_eq!(
            *s.seen.borrow(),
            vec![vec![
                CENTRE,
                WORLD_TRIANGLE[0],
                WORLD_TRIANGLE[1],
                WORLD_TRIANGLE[2]
            ]]
        );
        // The cache is the caller's, and no feature is asked for.
        let asked = calls(&s.log, CLOSEST_POINT_TRIANGLE);
        assert_eq!(asked[0][2], 0x5555);
        assert_eq!(asked[0][4], 0);
        assert!(calls(&s.log, BUILD_TRIANGLE_CACHE).is_empty());
    }

    #[test]
    fn a_contact_on_an_edge_answers_two_00d3d520() {
        let (answer, written, _) = run_contact(EDGE, 0.0);
        assert_eq!(answer, 2);
        assert_eq!(written, expected(EDGE));
    }

    #[test]
    fn apart_bodies_leave_the_output_alone_00d3d520() {
        let far = Hit {
            distance: 2.0,
            ..FACE
        };
        let (answer, written, _) = run_contact(far, 0.0);
        assert_eq!(answer, 0);
        assert_eq!(written, [-7.0; 8]);
        // Exactly the reach is not touching either; the tolerance widens it.
        let at_reach = Hit {
            distance: 1.75,
            ..FACE
        };
        let (answer, _, _) = run_contact(at_reach, 0.25);
        assert_eq!(answer, 0);
        let (answer, written, _) = run_contact(at_reach, 0.5);
        assert_eq!(answer, 1);
        assert_eq!(written, expected(at_reach));
    }

    #[test]
    fn a_nan_distance_is_no_contact_00d3d520() {
        let nan = Hit {
            distance: f32::NAN,
            ..FACE
        };
        let (answer, written, _) = run_contact(nan, 1.0);
        assert_eq!(answer, 0);
        assert_eq!(written, [-7.0; 8]);
    }

    /// The contacts the closest point collector got: the eight floats and the two bodies.
    type Points = Rc<RefCell<Vec<([f32; 8], u32, u32)>>>;

    /// A closest point collector in the scene; the contacts it gets.
    fn point_collector(s: &mut Scene) -> (u32, Points) {
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let got: Points = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&got);
        s.e.register_double(COLLECTOR + 4, move |e, a| {
            let bodies = (e.mem.u32(a[1] + 0x20), e.mem.u32(a[1] + 0x24));
            copy.borrow_mut()
                .push((floats(e, a[1]), bodies.0, bodies.1));
            Ret::default()
        });
        (collector, got)
    }

    /// Runs the closest point query `address` on a hit; `input_word` is the input's word `+4`
    /// and the triangle shape answers a question about the contact with `answer` (changing the
    /// normal to `(0, 0, 1, 9)`).
    fn run_closest(
        address: u32,
        hit: Hit,
        input_word: u32,
        answer: u32,
    ) -> (Scene, Points, AskedLog) {
        let mut s = scene(hit, 0.0);
        s.e.mem.set_u32(s.input + 4, input_word);
        let asked = weld_double(&mut s, answer, [0.0, 0.0, 1.0, 9.0]);
        let (collector, got) = point_collector(&mut s);
        let words = if address == 0x00d3_d6b0 {
            let this = s.e.mem.alloc(0x40);
            vec![this, s.a, s.b, s.input, collector]
        } else {
            vec![s.a, s.b, s.input, collector]
        };
        s.e.call(address, &words);
        (s, got, asked)
    }

    fn check_closest_points(address: u32, virtual_method: bool) {
        let (s, got, _) = run_closest(address, FACE, 1, 0);
        check_profile(&s);
        assert_eq!(*got.borrow(), vec![(expected(FACE), s.a, s.b)]);
        assert_eq!(
            *s.seen.borrow(),
            vec![vec![
                CENTRE,
                WORLD_TRIANGLE[0],
                WORLD_TRIANGLE[1],
                WORLD_TRIANGLE[2]
            ]]
        );
        let asked = calls(&s.log, CLOSEST_POINT_TRIANGLE);
        let built = calls(&s.log, BUILD_TRIANGLE_CACHE);
        if virtual_method {
            // The agent's cache, and no feature asked for.
            assert_eq!(asked[0][4], 0);
            assert!(built.is_empty());
        } else {
            assert_eq!(built, vec![vec![shape_of(&s) + 0x20, asked[0][2]]]);
            assert_ne!(asked[0][4], 0);
        }
        let far = Hit {
            distance: 2.0,
            ..FACE
        };
        let (s, got, _) = run_closest(address, far, 1, 0);
        check_profile(&s);
        assert!(got.borrow().is_empty());
    }

    #[test]
    fn get_closest_points_hands_the_contact_to_the_collector_00d3d6b0() {
        check_closest_points(0x00d3_d6b0, true);
    }

    #[test]
    fn static_get_closest_points_hands_the_contact_to_the_collector_00d3d8c0() {
        check_closest_points(0x00d3_d8c0, false);
    }

    #[test]
    fn get_closest_points_does_not_ask_the_triangle_shape_00d3d6b0() {
        let (s, got, asked) = run_closest(0x00d3_d6b0, EDGE, 1, 0);
        assert!(asked.borrow().is_empty());
        assert_eq!(*got.borrow(), vec![(expected(EDGE), s.a, s.b)]);
    }

    /// Runs the static closest point query with the input word `input_word` and a triangle
    /// shape that answers `answer`.
    fn run_weld(input_word: u32, hit: Hit, answer: u32) -> (Scene, Points, AskedLog) {
        run_closest(0x00d3_d8c0, hit, input_word, answer)
    }

    #[test]
    fn the_triangle_shape_is_asked_about_an_edge_contact_00d3d8c0() {
        let (s, got, asked) = run_weld(1, EDGE, 2);
        let asked = asked.borrow();
        assert_eq!(asked.len(), 1);
        let want = expected(EDGE);
        assert_eq!(asked[0].kind, 2);
        assert_eq!(asked[0].indices, [1, 2]);
        assert_eq!(asked[0].position, [want[0], want[1], want[2], want[3]]);
        assert_eq!(asked[0].normal, [want[4], want[5], want[6], want[7]]);
        // `this` is the shape; then the motion of B, the sphere's shape and motion.
        let motion_b = s.e.mem.u32(s.b + 8);
        let shape_a = s.e.mem.u32(s.a);
        let motion_a = s.e.mem.u32(s.a + 8);
        assert_eq!(asked[0].words[0], shape_of(&s));
        assert_eq!(asked[0].words[4..7], [motion_b, shape_a, motion_a]);
        // Another answer keeps the contact as it is.
        assert_eq!(*got.borrow(), vec![(want, s.a, s.b)]);
        check_profile(&s);
    }

    #[test]
    fn an_answer_of_zero_drops_the_contact_00d3d8c0() {
        let (s, got, asked) = run_weld(1, EDGE, 0);
        assert_eq!(asked.borrow().len(), 1);
        assert!(got.borrow().is_empty());
        check_profile(&s);
    }

    #[test]
    fn an_answer_of_one_takes_the_changed_normal_00d3d8c0() {
        let (s, got, asked) = run_weld(1, EDGE, 1);
        assert_eq!(asked.borrow().len(), 1);
        let mut want = expected(EDGE);
        want[4..7].copy_from_slice(&[0.0, 0.0, 1.0]);
        assert_eq!(*got.borrow(), vec![(want, s.a, s.b)]);
    }

    #[test]
    fn the_triangle_shape_is_not_asked_without_the_input_word_or_a_feature_00d3d8c0() {
        let (s, got, asked) = run_weld(0, EDGE, 0);
        assert!(asked.borrow().is_empty());
        assert_eq!(*got.borrow(), vec![(expected(EDGE), s.a, s.b)]);
        let (s, got, asked) = run_weld(1, FACE, 0);
        assert!(asked.borrow().is_empty());
        assert_eq!(*got.borrow(), vec![(expected(FACE), s.a, s.b)]);
    }

    /// What a run of `processCollision` left behind.
    struct Processed {
        s: Scene,
        this: u32,
        output: u32,
        free: u32,
        asked: AskedLog,
    }

    /// Runs `address` (`processCollision` `00d3dd90`, or the symmetric `00d3e150` with the
    /// bodies in the other order) for a hit, the contact point id, the id the contact manager
    /// makes and the weld answer; the manager also sets the output's time of impact to
    /// `new_time` (it starts at 0.5, with time of impact data to turn around).
    fn process(address: u32, hit: Hit, id: u16, made: u32, weld: u32, new_time: f32) -> Processed {
        let mut s = scene(hit, 0.0);
        let asked = weld_double(&mut s, weld, [0.0, 0.0, 1.0, 9.0]);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        let log = Rc::clone(&s.log);
        s.e.register_double(MANAGER + 0xc, move |e, a| {
            log.borrow_mut().push((MANAGER + 0xc, a.to_vec()));
            e.mem.set_f32(a[4] + 0x3030, new_time);
            Ret {
                eax: made,
                ..Ret::default()
            }
        });
        let this = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u16(this + 0xc, id);
        let output = s.e.mem.alloc(0x3100);
        let free = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(output, free);
        s.e.mem.set_u32(output + 4, 0x7777);
        s.e.mem.set_f32(output + 0x3030, 0.5);
        s.e.mem.set_u8(output + 0x3048, 1);
        s.e.mem.set_u8(output + 0x3049, 2);
        s.e.mem.set_u8(output + 0x304a, 0x12);
        for n in 0..4 {
            s.e.mem.set_u16(output + 0x3040 + 2 * n, 10 + n as u16);
        }
        store_vec(&mut s.e, output + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        let words = if address == 0x00d3_e150 {
            [this, s.b, s.a, s.input, output]
        } else {
            [this, s.a, s.b, s.input, output]
        };
        s.e.call(address, &words);
        Processed {
            s,
            this,
            output,
            free,
            asked,
        }
    }

    #[test]
    fn a_touching_contact_gets_an_id_and_is_added_00d3dd90() {
        let p = process(0x00d3_dd90, FACE, 0xffff, 0x42, 2, 0.5);
        check_profile(&p.s);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0xc),
            vec![vec![manager, p.s.a, p.s.b, p.s.input, p.output, 0, p.free]]
        );
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0x42);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 0x42);
        assert_eq!(floats(&p.s.e, p.free), expected(FACE));
        // The cache is the agent's (`this + 0x10`) and a feature is asked for.
        let asked = calls(&p.s.log, CLOSEST_POINT_TRIANGLE);
        assert_eq!(asked[0][2], p.this + 0x10);
        assert_ne!(asked[0][4], 0);
        assert!(p.asked.borrow().is_empty());
    }

    #[test]
    fn a_contact_with_an_id_is_added_without_asking_00d3dd90() {
        let p = process(0x00d3_dd90, FACE, 9, 0x42, 2, 0.5);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 9);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 9);
    }

    #[test]
    fn a_contact_the_manager_gives_no_id_is_skipped_00d3dd90() {
        let p = process(0x00d3_dd90, FACE, 0xffff, 0xffff, 2, 0.5);
        assert_eq!(calls(&p.s.log, MANAGER + 0xc).len(), 1);
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        check_profile(&p.s);
    }

    #[test]
    fn apart_bodies_give_their_id_back_00d3dd90() {
        let far = Hit {
            distance: 2.0,
            ..FACE
        };
        let p = process(0x00d3_dd90, far, 7, 0x42, 2, 0.5);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777]]
        );
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        check_profile(&p.s);
        // Without an id there is nothing to give back.
        let p = process(0x00d3_dd90, far, 0xffff, 0x42, 2, 0.5);
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
    }

    #[test]
    fn the_triangle_shape_is_asked_about_an_edge_contact_even_without_the_input_word_00d3dd90() {
        let p = process(0x00d3_dd90, EDGE, 0xffff, 0x42, 2, 0.5);
        let asked = p.asked.borrow();
        assert_eq!(asked.len(), 1);
        let want = expected(EDGE);
        assert_eq!(asked[0].kind, 2);
        assert_eq!(asked[0].indices, [1, 2]);
        assert_eq!(asked[0].position, [want[0], want[1], want[2], want[3]]);
        assert_eq!(asked[0].normal, [want[4], want[5], want[6], want[7]]);
        assert_eq!(floats(&p.s.e, p.free), want);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        check_profile(&p.s);
    }

    #[test]
    fn an_answer_of_one_takes_the_changed_normal_00d3dd90() {
        let p = process(0x00d3_dd90, EDGE, 0xffff, 0x42, 1, 0.5);
        let mut want = expected(EDGE);
        want[4..7].copy_from_slice(&[0.0, 0.0, 1.0]);
        assert_eq!(floats(&p.s.e, p.free), want);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
    }

    #[test]
    fn an_answer_of_zero_drops_the_contact_and_the_id_without_the_closing_record_00d3dd90() {
        let p = process(0x00d3_dd90, EDGE, 7, 0x42, 0, 0.5);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(p.asked.borrow().len(), 1);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777]]
        );
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        // The exe leaves without its closing profile record.
        assert_eq!(
            profile_labels(&p.s.e, p.s.stream, p.s.records),
            vec![LABEL_SPHERE_TRIANGLE]
        );
        // Without an id nothing is given back.
        let p = process(0x00d3_dd90, EDGE, 0xffff, 0x42, 0, 0.5);
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
    }

    /// The word the collector double of the flip tests sits at.
    const FLIP_SLOT_TARGET: u32 = 0x0940_0004;

    /// What the flip collector's slot 4 was asked: the arguments, the three words of the
    /// collector and, when the first argument is a contact, its eight floats, all at the time
    /// of the call.
    type SlotCalls = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, [f32; 8])>>>;

    /// Makes slot 4 of the vtable at `vtable` a double that notes its arguments and the three
    /// words of the collector it was called on (and the contact, when `contact`).
    fn flip_slot(s: &mut Scene, vtable: u32, contact: bool) -> SlotCalls {
        let seen: SlotCalls = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        s.e.mem.set_u32(vtable + 4, FLIP_SLOT_TARGET);
        s.e.register_double(FLIP_SLOT_TARGET, move |e, a| {
            let words = (0..3).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            let found = if contact { floats(e, a[1]) } else { [0.0; 8] };
            copy.borrow_mut().push((a.to_vec(), words, found));
            Ret::default()
        });
        seen
    }

    /// Runs the symmetric penetration query `address` (the triangle first) on touching bodies.
    fn check_symmetric_penetration(address: u32, virtual_method: bool) {
        let mut s = scene(FACE, 0.0);
        let seen = flip_slot(&mut s, FLIP_BODY_COLLECTOR_VTABLE, false);
        let this = s.e.mem.alloc(0x40);
        let words = if virtual_method {
            vec![this, s.b, s.a, s.input, 0x55]
        } else {
            vec![s.b, s.a, s.input, 0x55]
        };
        s.e.call(address, &words);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1..], [s.a, s.b]);
        assert_eq!(seen[0].1, vec![FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
        check_profile(&s);
    }

    #[test]
    fn symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3dd10() {
        check_symmetric_penetration(0x00d3_dd10, true);
    }

    #[test]
    fn static_symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3dd50() {
        check_symmetric_penetration(0x00d3_dd50, false);
    }

    /// Runs the symmetric closest point query `address` (the triangle first).
    fn check_symmetric_closest_points(address: u32, virtual_method: bool) {
        let mut s = scene(FACE, 0.0);
        s.e.mem.set_u32(LARGE_FLOAT, SIGNALLING_NAN);
        let seen = flip_slot(&mut s, FLIP_COLLECTOR_VTABLE, true);
        let this = s.e.mem.alloc(0x40);
        let words = if virtual_method {
            vec![this, s.b, s.a, s.input, 0x55]
        } else {
            vec![s.b, s.a, s.input, 0x55]
        };
        s.e.call(address, &words);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![FLIP_COLLECTOR_VTABLE, 0x7fe0_0001, 0x55]);
        assert_eq!(seen[0].2, expected(FACE));
        check_profile(&s);
    }

    #[test]
    fn symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3e0d0() {
        check_symmetric_closest_points(0x00d3_e0d0, true);
    }

    #[test]
    fn static_symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3e110() {
        check_symmetric_closest_points(0x00d3_e110, false);
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contact_00d3e150() {
        let p = process(0x00d3_e150, FACE, 0xffff, 0x42, 2, 0.5);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        let want = expected(FACE);
        let w = want[7];
        // Position += normal.w * normal, normal.xyz negated.
        let position: Vec4 = std::array::from_fn(|lane| w * want[4 + lane] + want[lane]);
        let normal = [-want[4], -want[5], -want[6], w];
        assert_eq!(load_vec(&p.s.e, p.free), position);
        assert_eq!(load_vec(&p.s.e, p.free + 0x10), normal);
        // The time did not change: the time of impact data stays.
        assert_eq!(load_vec(&p.s.e, p.output + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(p.s.e.mem.u8(p.output + 0x304a), 0x12);
        check_profile(&p.s);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes_00d3e150() {
        let p = process(0x00d3_e150, FACE, 0xffff, 0x42, 2, 0.25);
        let (e, output) = (&p.s.e, p.output);
        assert_eq!(load_vec(e, output + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(e.mem.u8(output + 0x3048), 2);
        assert_eq!(e.mem.u8(output + 0x3049), 1);
        let words: Vec<u16> = (0..4).map(|n| e.mem.u16(output + 0x3040 + 2 * n)).collect();
        assert_eq!(words, vec![12, 11, 10, 13]);
    }

    /// The registrar double: logs `(dispatcher, entry, type a, type b)`, the entry's four
    /// words and two flag bytes.
    fn registrar(e: &mut Engine, address: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[1] + 4 * n)));
            words.extend([0x10, 0x11].map(|at| u32::from(e.mem.u8(a[1] + at))));
            copy.borrow_mut().push(words);
            Ret::default()
        });
        seen
    }

    fn check_registration(address: u32, registrar_at: u32) {
        let mut e = primed_engine();
        let seen = registrar(&mut e, registrar_at);
        e.call(address, &args![0xd15bu32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][0], 0xd15b);
        assert_eq!(
            seen[0][2..],
            [
                3,
                1,
                0x00d3_d240,
                0x00d3_dd50,
                0x00d3_e110,
                0x00d3_ac80,
                1,
                0
            ]
        );
        assert_eq!(seen[1][0], 0xd15b);
        assert_eq!(
            seen[1][2..],
            [
                1,
                3,
                0x00d3_d1a0,
                0x00d3_d3e0,
                0x00d3_d8c0,
                0x00da_f8e0,
                0,
                0
            ]
        );
        assert_eq!(seen[0][1], seen[1][1]);
    }

    #[test]
    fn registration_adds_the_symmetric_then_the_direct_entry_00d3dc40() {
        check_registration(0x00d3_dc40, REGISTER_AGENT);
    }

    #[test]
    fn the_other_registration_adds_the_same_entries_00d3dca0() {
        check_registration(0x00d3_dca0, REGISTER_AGENT_OTHER);
    }

    #[test]
    fn the_direct_entry_is_filled_00d3dbe0() {
        let mut e = primed_engine();
        let entry = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00d3_dbe0, &args![entry]).u32(), entry);
        let words: Vec<u32> = (0..4).map(|n| e.mem.u32(entry + 4 * n)).collect();
        assert_eq!(
            words,
            vec![0x00d3_d1a0, 0x00d3_d3e0, 0x00d3_d8c0, 0x00da_f8e0]
        );
        assert_eq!((e.mem.u8(entry + 0x10), e.mem.u8(entry + 0x11)), (0, 0));
    }

    #[test]
    fn the_symmetric_entry_is_filled_00d3dc10() {
        let mut e = primed_engine();
        let entry = e.mem.alloc(0x14);
        assert_eq!(e.call(0x00d3_dc10, &args![entry]).u32(), entry);
        let words: Vec<u32> = (0..4).map(|n| e.mem.u32(entry + 4 * n)).collect();
        assert_eq!(
            words,
            vec![0x00d3_d240, 0x00d3_dd50, 0x00d3_e110, 0x00d3_ac80]
        );
        assert_eq!((e.mem.u8(entry + 0x10), e.mem.u8(entry + 0x11)), (1, 0));
    }

    #[test]
    fn the_empty_function_does_nothing_00d3dd00() {
        let mut e = primed_engine();
        let log = new_log();
        let this = logging_object(&mut e, &log, OBJECT);
        let before = e.mem.u32(this);
        e.call(0x00d3_dd00, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this), before);
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
    fn fn_00d3d240_fills_the_block_and_copies_the_shape_data() {
        let (mut e, sizes) = allocator_engine();
        let shape = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cd_7160, move |e, a| {
            // The block already holds the first vtable word while the data is copied.
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1] - 0x10)));
            Ret::default()
        });
        let block = e
            .call(0x00d3_d240, &args![body, 0x2000u32, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x20]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![shape + 0x20, block + 0x10], 0x010c_f060)]
        );
        assert_eq!(e.mem.u16(block + 4), 0x20);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_f09c);
        assert_eq!(e.mem.u16(block + 0xc), 0xffff);
    }
}
