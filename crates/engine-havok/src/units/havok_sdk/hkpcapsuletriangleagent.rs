//! `Havok SDK/hkpcapsuletriangleagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpCapsuleTriangleAgent` (Xbox PDB) is the collision agent of a capsule (body A) and a
//! triangle (body B). Its base is the Xbox PDB's `hkpCollisionAgent`: contact manager `+8`,
//! the three manifold contact point ids (16-bit, `0xFFFF` = none) at `+0xC`, and from `+0x14`
//! a cache of five floats of the triangle (made by `00cd72a0`, handed on to
//! `hkCollideCapsuleUtilCapsVsTri`). The static (dispatcher) versions of the queries make
//! that cache on the stack instead. A body is an `hkpCdBody` (shape `+0`, motion `+8`); the
//! capsule shape holds its radius at `+0x10` and its end points at `+0x20` and `+0x30`, the
//! triangle shape its radius at `+0x10` and its vertices at `+0x20`, `+0x30` and `+0x40`.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpbvagent::{
    symmetric_linear_cast, symmetric_process_collision, with_flip_body_collector,
    with_flip_collector,
};
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{
    cleanup_agent, load_matrix, radius_bits, store_ends, transform_point, world_ends,
};
use crate::units::havok_sdk::hkpsphereshape::{copy_vec, load_vec, store_vec};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: `"TtCapsTriangle"`, `"TtCapsuleTri"` (`processCollision`) and the shared
/// end label.
pub(crate) const LABEL_TRIANGLE: u32 = 0x010c_ef80;
pub(crate) const LABEL_PROCESS: u32 = 0x010c_ef70;
pub(crate) const LABEL_END: u32 = 0x010c_41f8;
/// `hkCollideCapsuleUtilCapsVsTri` (cdecl, 9 words: capsule ends, capsule radius, triangle
/// vertices, triangle radius, cache, tolerance, search manifold, out contacts, features).
pub(crate) const CAPS_VS_TRI: u32 = 0x00cd_6520;
/// Makes the cache of a triangle (cdecl: the triangle's vertices, the cache).
pub(crate) const TRIANGLE_PREPARE: u32 = 0x00cd_72a0;
/// Replaces the normal's first three lanes of a contact (`this` = the contact, the vector).
const SET_NORMAL: u32 = 0x00ca_ad60;
/// The linear cast of the bounding-volume pair, which the direct entry registers (cdecl:
/// two bodies, input, two collectors).
const LINEAR_CAST: u32 = 0x00da_f8e0;
/// `hkpCollisionDispatcher` agent registrations (`this` = dispatcher, `RET 0xC`): `(agent
/// function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
const REGISTER_AGENT_OTHER: u32 = 0x00cf_bcc0;

/// The frame the queries build on the stack: three contacts of 0x20 bytes, three feature
/// entries of 8 bytes, the capsule's end points, the triangle's vertices (in world space), a
/// byte for the kind of the feature asked about, a temporary vector, the triangle's cache and
/// a copy of a contact followed by the two bodies (the `hkpCdPoint` the collector gets).
const OUT: u32 = 0;
const FEATURES: u32 = 0x60;
const ENDS: u32 = 0x80;
const TRIANGLE: u32 = 0xa0;
const KIND: u32 = 0xd0;
const TEMP: u32 = 0xe0;
const CACHE: u32 = 0xf0;
const COPY: u32 = 0x110;
const FRAME_SIZE: u32 = 0x138;

/// The two bodies, the input and the triangle's cache of a query.
#[derive(Clone, Copy)]
struct Pair {
    body_a: u32,
    body_b: u32,
    input: u32,
    cache: u32,
}

/// Moves the capsule's end points and the triangle's vertices to world space into `frame`
/// and runs `hkCollideCapsuleUtilCapsVsTri` (`00cd6520`) on them with the radii, the cache,
/// the input's tolerance (`+8`, an x87 load and store), `search_manifold`, the contacts at
/// `out` and the feature entries at `features` (0 for none).
fn collide(e: &mut Engine, frame: u32, out: u32, pair: Pair, search_manifold: u32, features: u32) {
    let ends = world_ends(e, pair.body_a);
    store_ends(e, frame + ENDS, ends);
    let shape = e.mem.u32(pair.body_b);
    let rows = load_matrix(e, e.mem.u32(pair.body_b + 8));
    for (n, offset) in [0x20u32, 0x30, 0x40].into_iter().enumerate() {
        let vertex = transform_point(&rows, load_vec(e, shape + offset));
        store_vec(e, frame + TRIANGLE + 0x10 * n as u32, vertex);
    }
    let radius_a = radius_bits(e, pair.body_a);
    let radius_b = radius_bits(e, pair.body_b);
    let tolerance = x87_quiet_bits(e.mem.u32(pair.input + 8));
    e.call(
        CAPS_VS_TRI,
        &args![
            frame + ENDS,
            radius_a,
            frame + TRIANGLE,
            radius_b,
            pair.cache,
            tolerance,
            search_manifold,
            out,
            features
        ],
    );
}

/// The cache of a static query: made by `00cd72a0` from the triangle's vertices
/// (`shape + 0x20`) into the frame.
fn make_cache(e: &mut Engine, frame: u32, body_b: u32) -> u32 {
    let shape = e.mem.u32(body_b);
    e.call(TRIANGLE_PREPARE, &args![shape + 0x20, frame + CACHE]);
    frame + CACHE
}

/// Asks the triangle shape (`[shape + 0x40]`, with the shape as `this`) about the feature
/// `index` of the contact at `contact`: the feature's vertex indices (`+2` of its entry), the
/// kind byte in the frame, the contact, both motions, the capsule shape and a copy of the
/// contact's normal vector (`+0x10`) in the frame, which the shape may change. Answers the
/// shape's answer (0 drop the contact, 1 take the changed vector).
fn ask_feature(e: &mut Engine, frame: u32, index: u32, pair: Pair, contact: u32) -> u32 {
    copy_vec(e, contact + 0x10, frame + TEMP);
    let shape_a = e.mem.u32(pair.body_a);
    let motion_a = e.mem.u32(pair.body_a + 8);
    let shape_b = e.mem.u32(pair.body_b);
    let motion_b = e.mem.u32(pair.body_b + 8);
    e.vcall(
        shape_b,
        0x40,
        &args![
            frame + FEATURES + 8 * index + 2,
            frame + KIND,
            contact,
            motion_b,
            shape_a,
            motion_a,
            frame + TEMP
        ],
    )
    .u32()
}

/// What the closest point queries do after `hkCollideCapsuleUtilCapsVsTri`: picks the contact
/// of the smaller gap (the second, in lane 3 of the normal at `+0x1C` of the contact; the
/// second also when the gaps are equal or unordered) when it is below the input's tolerance
/// (`+8`; x87 compares); then, when the input's word `+4` is set and the contact's feature
/// kind is not 0, asks the triangle shape about the feature: answer 0 drops the contact,
/// answer 1 puts the vector it changed into the contact's normal (`00caad60`). Answers the
/// address of the contact.
fn pick_contact(e: &mut Engine, frame: u32, pair: Pair) -> Option<u32> {
    let first = e.mem.f32(frame + OUT + 0x1c);
    let second = e.mem.f32(frame + OUT + 0x3c);
    let tolerance = e.mem.f32(pair.input + 8);
    let index = if second > first {
        if tolerance > first {
            0
        } else {
            return None;
        }
    } else if tolerance > second {
        1
    } else {
        return None;
    };
    let contact = frame + OUT + 0x20 * index;
    let kind = e.mem.u8(frame + FEATURES + 8 * index);
    e.mem.set_u8(frame + KIND, kind);
    if e.mem.u32(pair.input + 4) != 0 && kind != 0 {
        match ask_feature(e, frame, index, pair, contact) {
            0 => return None,
            1 => {
                e.call(SET_NORMAL, &args![contact, frame + TEMP]);
            }
            _ => {}
        }
    }
    Some(contact)
}

/// `getClosestPoints`: profiled (`"TtCapsTriangle"`); `this` gives the cache (`this + 0x14`)
/// of the virtual version, the static version makes it. The picked contact (see
/// `pick_contact`) is copied (32 bytes), followed by body A and body B (the exe's locals
/// right after the copy: EBP-0xec, then EBP-0xcc and -0xc8, set at the start), and that
/// `hkpCdPoint` is handed to the collector (slot 4, `addCdPoint`).
fn closest_points(
    e: &mut Engine,
    this: Option<u32>,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    profile_record(e, LABEL_TRIANGLE);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let cache = match this {
            Some(this) => this + 0x14,
            None => make_cache(e, frame, body_b),
        };
        let pair = Pair {
            body_a,
            body_b,
            input,
            cache,
        };
        collide(e, frame, frame + OUT, pair, 0, frame + FEATURES);
        if let Some(contact) = pick_contact(e, frame, pair) {
            copy_vec(e, contact, frame + COPY);
            copy_vec(e, contact + 0x10, frame + COPY + 0x10);
            e.mem.set_u32(frame + COPY + 0x20, body_a);
            e.mem.set_u32(frame + COPY + 0x24, body_b);
            e.vcall(collector, 4, &args![frame + COPY]);
        }
    });
    profile_record(e, LABEL_END);
}

/// `getPenetrations`: profiled (`"TtCapsTriangle"`); the cache as in `closest_points`; the
/// collector (slot 4, with the two bodies) is told when the gap of the first or of the second
/// contact is below 0 (ordered compares).
fn penetrations(
    e: &mut Engine,
    this: Option<u32>,
    body_a: u32,
    body_b: u32,
    input: u32,
    collector: u32,
) {
    profile_record(e, LABEL_TRIANGLE);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let cache = match this {
            Some(this) => this + 0x14,
            None => make_cache(e, frame, body_b),
        };
        let pair = Pair {
            body_a,
            body_b,
            input,
            cache,
        };
        collide(e, frame, frame + OUT, pair, 0, 0);
        let first = e.mem.f32(frame + OUT + 0x1c);
        let second = e.mem.f32(frame + OUT + 0x3c);
        if first < 0.0 || second < 0.0 {
            e.vcall(collector, 4, &args![body_a, body_b]);
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d39930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleTriangleAgent::cleanup` (Xbox PDB): gives every contact point id that is not
/// `0xFFFF` back to the contact manager (`[this + 8]`, slot `0x14`, with the id and `param`),
/// then runs the agent's deleting destructor (slot 0, argument 1).
pub fn hkp_capsule_triangle_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    cleanup_agent(e, this.addr(), param);
}

// Translated from 00d399f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A static helper of the unit (the map gives it no name): moves the capsule of body A and
/// the triangle of body B to world space and runs `hkCollideCapsuleUtilCapsVsTri`
/// (`00cd6520`) on them with the radii, `cache`, the input's tolerance (`+8`),
/// `search_manifold`, the contacts at `out` and no feature entries.
pub fn fn_00d399f0(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cache: u32,
    search_manifold: u32,
    out: u32,
) {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let pair = Pair {
            body_a: body_a.addr(),
            body_b: body_b.addr(),
            input: input.addr(),
            cache,
        };
        collide(e, frame.addr(), out, pair, search_manifold, 0);
    });
}

// Translated from 00d39bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A static closest point query of the unit (the map gives it no name), without profiling
/// and with the caller's `cache`: runs the capsule against the triangle (see `pick_contact`)
/// and, when a contact is picked, copies it (32 bytes) to `out` and answers 1; else answers 0.
pub fn fn_00d39bf0(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    cache: u32,
    out: Ptr,
) -> u32 {
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        let pair = Pair {
            body_a: body_a.addr(),
            body_b: body_b.addr(),
            input: input.addr(),
            cache,
        };
        collide(e, frame, frame + OUT, pair, 0, frame + FEATURES);
        match pick_contact(e, frame, pair) {
            Some(contact) => {
                copy_vec(e, contact, out.addr());
                copy_vec(e, contact + 0x10, out.addr() + 0x10);
                1
            }
            None => 0,
        }
    })
}

// Translated from 00d39ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleTriangleAgent::getClosestPoints` (Xbox PDB): profiled; the capsule of body A
/// against the triangle of body B with the agent's cache (`this + 0x14`); the picked contact
/// goes to the collector (see `closest_points`).
pub fn hkp_capsule_triangle_agent_get_closest_points(
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
        body_a.addr(),
        body_b.addr(),
        input.addr(),
        collector.addr(),
    );
}

// Translated from 00d3a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static (cdecl) `getClosestPoints` of the capsule-triangle pair (the map gives it no
/// name): as the virtual version, with a cache made on the stack by `00cd72a0`.
pub fn fn_00d3a280(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    closest_points(
        e,
        None,
        body_a.addr(),
        body_b.addr(),
        input.addr(),
        collector.addr(),
    );
}

// Translated from 00d3a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleTriangleAgent::getPenetrations` (Xbox PDB): profiled; the capsule of body A
/// against the triangle of body B with the agent's cache (`this + 0x14`); the collector is
/// told when a gap is below 0 (see `penetrations`).
pub fn hkp_capsule_triangle_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    penetrations(
        e,
        Some(this.addr()),
        body_a.addr(),
        body_b.addr(),
        input.addr(),
        collector.addr(),
    );
}

// Translated from 00d3a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static (cdecl) `getPenetrations` of the capsule-triangle pair (the map gives it no
/// name): as the virtual version, with a cache made on the stack by `00cd72a0`.
pub fn fn_00d3a8c0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    penetrations(
        e,
        None,
        body_a.addr(),
        body_b.addr(),
        input.addr(),
        collector.addr(),
    );
}

// Translated from 00d3ab60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function entry (the map gives the function no name; cdecl, `EAX` holds the
/// entry on return) with the direct functions of the capsule-triangle pair: `createFunc`
/// `00d398c0`, `getPenetrations` `00d3a8c0`, `getClosestPoints` `00d3a280`, `linearCast`
/// `00daf8e0`; both flag bytes (`+0x10`, `+0x11`) 0. The two bytes after them are not set.
pub fn fn_00d3ab60(e: &mut Engine, entry: Ptr) {
    let entry = entry.addr();
    for (index, word) in [0x00d3_98c0u32, 0x00d3_a8c0, 0x00d3_a280, 0x00da_f8e0]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(entry + 4 * index as u32, word);
    }
    e.mem.set_u8(entry + 0x10, 0);
    e.mem.set_u8(entry + 0x11, 0);
}

// Translated from 00d3ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an agent function entry (the map gives the function no name; cdecl, `EAX` holds the
/// entry on return) with the symmetric functions: `createFunc` `00d39980`, `getPenetrations`
/// `00d3b1a0`, `getClosestPoints` `00d3b220`, `linearCast` `00d3ac80`; the flag bytes `+0x10`
/// = 1 and `+0x11` = 0. The two bytes after them are not set.
pub fn fn_00d3ab90(e: &mut Engine, entry: Ptr) {
    let entry = entry.addr();
    for (index, word) in [0x00d3_9980u32, 0x00d3_b1a0, 0x00d3_b220, 0x00d3_ac80]
        .into_iter()
        .enumerate()
    {
        e.mem.set_u32(entry + 4 * index as u32, word);
    }
    e.mem.set_u8(entry + 0x10, 1);
    e.mem.set_u8(entry + 0x11, 0);
}

/// The registration of both entries with a dispatcher through `register` (`this` = the
/// dispatcher, `(entry, type a, type b)`): the symmetric entry for the types (3, 5), then the
/// direct entry for (5, 3), both built in the same 20-byte block on the stack. Its bytes
/// `+0x12` and `+0x13` are not set by the exe (the callee copies the word they are in); here
/// they are zero.
fn register_entries(e: &mut Engine, dispatcher: Ptr, register: u32) {
    e.with_stack(0x14, |e, entry| {
        fn_00d3ab90(e, entry);
        e.call(register, &args![dispatcher, entry, 3u32, 5u32]);
        fn_00d3ab60(e, entry);
        e.call(register, &args![dispatcher, entry, 5u32, 3u32]);
    });
}

// Translated from 00d3abc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the capsule-triangle agent with a dispatcher (the map gives it no name): the
/// symmetric entry (`00d3ab90`) for the shape types (3, 5) and the direct entry (`00d3ab60`)
/// for (5, 3), through `00cfbbe0`.
pub fn fn_00d3abc0(e: &mut Engine, dispatcher: Ptr) {
    register_entries(e, dispatcher, REGISTER_AGENT);
}

// Translated from 00d3ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00d3abc0`, through the other registration function `00cfbcc0`.
pub fn fn_00d3ac20(e: &mut Engine, dispatcher: Ptr) {
    register_entries(e, dispatcher, REGISTER_AGENT_OTHER);
}

// Translated from 00d3ac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static linear cast of the capsule-triangle pair (the map gives it no name):
/// runs the bounding-volume pair's linear cast `00daf8e0` with the two bodies swapped, a
/// copy of the input whose path vector (`input + 0x50`) is negated, and flip-cast collectors
/// (vtable `010ce80c`) wrapping the caller's two (`start_collector` null stays null).
pub fn fn_00d3ac80(
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
            e.call(LINEAR_CAST, &args![body_b, body_a, copy, first, second]);
        },
    );
}

/// Gives the id at `id_at` back to the contact manager of the agent (`[this + 8]`, slot
/// `0x14`, with the id and `[output + 4]`) and clears it, when there is one.
fn release_id(e: &mut Engine, this: u32, id_at: u32, output: u32) {
    let id = e.mem.u16(id_at);
    if id != 0xffff {
        let manager = e.mem.u32(this + 8);
        let second = e.mem.u32(output + 4);
        e.vcall(manager, 0x14, &args![u32::from(id), second]);
        e.mem.set_u16(id_at, 0xffff);
    }
}

/// One contact of `processCollision` whose gap is below the tolerance: asks the triangle
/// shape about its feature (see `pick_contact`; answer 0 gives the id back, answer 1 puts
/// the changed vector's first three lanes into the normal), gets an id from the contact
/// manager when there is none (slot `0xC`: the bodies, the input, the output, 0 and the
/// contact; an id of `0xFFFF` drops the contact) and appends the 32 bytes of the contact and
/// the id (a word at `+0x20`) at the output's next free contact point.
fn add_contact(e: &mut Engine, frame: u32, index: u32, pair: Pair, this: u32, output: u32) {
    let contact = frame + OUT + 0x20 * index;
    let id_at = this + 0xc + 2 * index;
    let kind = e.mem.u8(frame + FEATURES + 8 * index);
    e.mem.set_u8(frame + KIND, kind);
    if kind != 0 {
        match ask_feature(e, frame, index, pair, contact) {
            0 => {
                release_id(e, this, id_at, output);
                return;
            }
            1 => {
                let temp = load_vec(e, frame + TEMP);
                let normal_w = e.mem.f32(contact + 0x1c);
                store_vec(e, contact + 0x10, [temp[0], temp[1], temp[2], normal_w]);
            }
            _ => {}
        }
    }
    if e.mem.u16(id_at) == 0xffff {
        let manager = e.mem.u32(this + 8);
        let id = e
            .vcall(
                manager,
                0xc,
                &args![pair.body_a, pair.body_b, pair.input, output, 0u32, contact],
            )
            .u32() as u16;
        e.mem.set_u16(id_at, id);
        if id == 0xffff {
            return;
        }
    }
    let free = e.mem.u32(output);
    e.mem.set_u32(output, free + 0x30);
    copy_vec(e, contact, free);
    copy_vec(e, contact + 0x10, free + 0x10);
    let id = u32::from(e.mem.u16(id_at));
    e.mem.set_u32(free + 0x20, id);
}

// Translated from 00d3ad70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleTriangleAgent::processCollision` (Xbox PDB): profiled (`"TtCapsuleTri"`). Runs
/// `hkCollideCapsuleUtilCapsVsTri` (`00cd6520`) on the capsule of body A and the triangle of
/// body B with the agent's cache and the manifold search on, into three contacts and their
/// feature entries. For contact `i` (its id at `this + 0xC + 2 * i`): when the input's
/// tolerance (`+8`) is above its gap it is added to the output (see `add_contact`), else an
/// existing id goes back to the contact manager (slot `0x14`, with the id and
/// `[output + 4]`) and is cleared.
pub fn hkp_capsule_triangle_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    let (this, output) = (this.addr(), output.addr());
    let pair = Pair {
        body_a: body_a.addr(),
        body_b: body_b.addr(),
        input: input.addr(),
        cache: this + 0x14,
    };
    profile_record(e, LABEL_PROCESS);
    e.with_stack(FRAME_SIZE, |e, frame| {
        let frame = frame.addr();
        collide(e, frame, frame + OUT, pair, 1, frame + FEATURES);
        for index in 0..3 {
            let gap = e.mem.f32(frame + OUT + 0x20 * index + 0x1c);
            if e.mem.f32(pair.input + 8) > gap {
                add_contact(e, frame, index, pair, this, output);
            } else {
                release_id(e, this, this + 0xc + 2 * index, output);
            }
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d3b160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpCapsuleTriangleAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpCapsuleTriangleAgent::getPenetrations` with the bodies swapped and, in place of the
/// caller's collector word `output`, a flip-body collector (vtable `010ce81c`, byte 0 at
/// `+4`, the caller's word at `+8`). `this` goes on in `ECX`.
pub fn hkp_symmetric_agent_linear_cast_capsule_triangle_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_capsule_triangle_agent_get_penetrations(e, this, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3b1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the capsule-triangle pair (the map gives it no
/// name): runs `00d3a8c0` with the bodies swapped and a flip-body collector around the
/// caller's word `output`.
pub fn fn_00d3b1a0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        fn_00d3a8c0(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpCapsuleTriangleAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpCapsuleTriangleAgent::getClosestPoints` with the bodies swapped and a flip collector
/// (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word
/// `output` at `+8`). `this` goes on in `ECX`.
pub fn hkp_symmetric_agent_linear_cast_capsule_triangle_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_capsule_triangle_agent_get_closest_points(
            e,
            this,
            body_b,
            body_a,
            input,
            Ptr::new(flip),
        );
    });
}

// Translated from 00d3b220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the capsule-triangle pair (the map gives it no
/// name): runs `00d3a280` with the bodies swapped and a flip collector around the caller's
/// word `output`.
pub fn fn_00d3b220(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        fn_00d3a280(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3b260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpCapsuleTriangleAgent>::processCollision` (Xbox PDB): runs
/// `hkpCapsuleTriangleAgent::processCollision` with the bodies swapped, then turns what it
/// appended to the output around, as `00d46220` does for the bounding-volume pair.
pub fn hkp_symmetric_agent_linear_cast_capsule_triangle_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        hkp_capsule_triangle_agent_process_collision(e, this, body_b, body_a, input, output);
    });
}

// Translated from 00d39980 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first is the first collidable (`body_a`), the second and third are never read (`_unused_1`, `_unused_2`) and the last is the contact manager, `manager`. Allocates 0x28 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc, +0xe and +0x10 and the vtable word `010ceefc` at +0 (replaced by `010cef38` once the copy is done). Then calls `00cd72a0` with the data at +0x20 of the shape (the word at +0 of the collidable) and the address of the block + 0x14. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3ab94.
pub fn fn_00d39980(
    e: &mut Engine,
    body_a: u32,
    _unused_1: u32,
    _unused_2: u32,
    manager: u32,
) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x28u32]).u32();
    e.mem.set_u16(block + 4, 0x28);
    e.mem.set_u16(block + 6, 1);
    e.mem.set_u32(block + 8, manager);
    e.mem.set_u16(block + 0xc, 0xffff);
    e.mem.set_u16(block + 0xe, 0xffff);
    e.mem.set_u16(block + 0x10, 0xffff);
    e.mem.set_u32(block, 0x010c_eefc);
    let shape = e.mem.u32(body_a);
    e.call(0x00cd_72a0, &args![shape + 0x20, block + 0x14]);
    e.mem.set_u32(block, 0x010c_ef38);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_9930, hkp_capsule_triangle_agent_cleanup(Ptr, u32)),
        entry!(0x00d3_99f0, fn_00d399f0(Ptr, Ptr, Ptr, u32, u32, u32)),
        entry!(0x00d3_9bf0, fn_00d39bf0(Ptr, Ptr, Ptr, u32, Ptr) -> u32),
        entry!(
            0x00d3_9ef0,
            hkp_capsule_triangle_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_a280, fn_00d3a280(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d3_a620,
            hkp_capsule_triangle_agent_get_penetrations(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_a8c0, fn_00d3a8c0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d3_ab60, fn_00d3ab60(Ptr)),
        entry!(0x00d3_ab90, fn_00d3ab90(Ptr)),
        entry!(0x00d3_abc0, fn_00d3abc0(Ptr)),
        entry!(0x00d3_ac20, fn_00d3ac20(Ptr)),
        entry!(0x00d3_ac80, fn_00d3ac80(Ptr, Ptr, Ptr, u32, u32)),
        entry!(
            0x00d3_ad70,
            hkp_capsule_triangle_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_b160,
            hkp_symmetric_agent_linear_cast_capsule_triangle_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_b1a0, fn_00d3b1a0(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_b1e0,
            hkp_symmetric_agent_linear_cast_capsule_triangle_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_b220, fn_00d3b220(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_b260,
            hkp_symmetric_agent_linear_cast_capsule_triangle_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(0x00d3_9980, fn_00d39980(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };
    use crate::units::havok_sdk::hkpcapsulecapsuleagent::fixtures::{
        capsule_a, shaped_body, vector_double, Seen,
    };
    use crate::units::havok_sdk::hkpsphereshape::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const COLLECTOR: u32 = 0x0910_0000;
    const OBJECT: u32 = 0x0920_0000;
    /// The triangle shape's slot `0x40` and a second address that logs what the double saw.
    const ASK: u32 = 0x0930_0040;
    const ASK_SEEN: u32 = 0x0930_0041;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;

    /// A move by `(5, 6, 7)`.
    const SHIFT: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [5.0, 6.0, 7.0, 1.0],
    ];
    /// The triangle's vertices and their places after `SHIFT`.
    const VERTICES: [Vec4; 3] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];
    const WORLD_TRIANGLE: [Vec4; 3] = [
        [6.0, 6.0, 7.0, 1.0],
        [5.0, 7.0, 7.0, 1.0],
        [5.0, 6.0, 8.0, 1.0],
    ];
    /// What the shape's answer 1 leaves in the vector it was given.
    const CHANGED: Vec4 = [9.0, 8.0, 7.0, 6.0];

    struct Scene {
        e: Engine,
        log: Log,
        /// The vectors `hkCollideCapsuleUtilCapsVsTri` saw: the two end points, the three
        /// vertices.
        seen: Seen,
        a: u32,
        b: u32,
        input: u32,
        stream: u32,
        records: u32,
    }

    /// The bodies (the capsule A and a triangle B of radius 0.25), an input (tolerance 1.0,
    /// word `+4` = `flag`), a profile stream, a double of `hkCollideCapsuleUtilCapsVsTri`
    /// that makes contacts with the given `gaps` (lane 3 of the normal `(0.5, 0.25, 0.125)`
    /// of contact `i`, which lies at `(i + 1, 2, 3, 4)`) and feature entries of the given
    /// `kinds` (vertex indices `10 * i + 1`, 2, 3), a double of the cache maker, and the
    /// triangle shape's slot `0x40` answering `answer` (and, for 1, changing the vector to
    /// `CHANGED`).
    fn scene(gaps: [f32; 3], kinds: [u8; 3], flag: u32, answer: u32) -> Scene {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let a = capsule_a(&mut e);
        let b = shaped_body(&mut e, 0.25, &VERTICES, SHIFT);
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + 0x40, ASK);
        let shape_b = e.mem.u32(b);
        e.mem.set_u32(shape_b, vtable);
        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 8, 1.0);
        e.mem.set_u32(input + 4, flag);
        let seen = vector_double(
            &mut e,
            &log,
            CAPS_VS_TRI,
            &[(0, 2), (2, 3)],
            0,
            move |e, a| {
                for (n, gap) in gaps.into_iter().enumerate() {
                    let contact = a[7] + 0x20 * n as u32;
                    store_vec(e, contact, [n as f32 + 1.0, 2.0, 3.0, 4.0]);
                    store_vec(e, contact + 0x10, [0.5, 0.25, 0.125, gap]);
                    if a[8] != 0 {
                        let entry = a[8] + 8 * n as u32;
                        e.mem.set_u8(entry, kinds[n]);
                        e.mem.set_u16(entry + 2, 10 * n as u16 + 1);
                        e.mem.set_u16(entry + 4, 2);
                        e.mem.set_u16(entry + 6, 3);
                    }
                }
            },
        );
        log_double(&mut e, &log, TRIANGLE_PREPARE, 0);
        let copy = Rc::clone(&log);
        e.register_double(ASK, move |e, a| {
            copy.borrow_mut().push((ASK, a.to_vec()));
            let temp = (0..4).map(|n| e.mem.u32(a[7] + 4 * n)).collect::<Vec<_>>();
            let kind = u32::from(e.mem.u8(a[2]));
            let vertices = (0..3).map(|n| u32::from(e.mem.u16(a[1] + 2 * n)));
            copy.borrow_mut().push((
                ASK_SEEN,
                [kind].into_iter().chain(vertices).chain(temp).collect(),
            ));
            if answer == 1 {
                store_vec(e, a[7], CHANGED);
            }
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        Scene {
            e,
            log,
            seen,
            a,
            b,
            input,
            stream,
            records,
        }
    }

    /// The vectors the utility should have seen: the capsule's end points (A is turned) and
    /// the triangle's vertices in world space.
    fn world() -> Vec<Vec4> {
        let mut world = vec![[8.0, 21.0, 33.0, 1.0], [8.0, 21.0, 34.0, 1.0]];
        world.extend(WORLD_TRIANGLE);
        world
    }

    /// The contact `i` of `scene` with the given gap, as its eight floats.
    fn contact(i: usize, gap: f32) -> [f32; 8] {
        [i as f32 + 1.0, 2.0, 3.0, 4.0, 0.5, 0.25, 0.125, gap]
    }

    fn floats(e: &Engine, at: u32) -> [f32; 8] {
        std::array::from_fn(|n| e.mem.f32(at + 4 * n as u32))
    }

    #[test]
    fn cleanup_gives_back_the_ids_in_use_00d39930() {
        let mut e = engine();
        let log = new_log();
        let manager = logging_object(&mut e, &log, MANAGER);
        let this = logging_object(&mut e, &log, OBJECT);
        e.mem.set_u32(this + 8, manager);
        for (n, id) in [0xffffu16, 0x20, 0x21].into_iter().enumerate() {
            e.mem.set_u16(this + 0xc + 2 * n as u32, id);
        }
        e.call(0x00d3_9930, &[this, 0x77]);
        assert_eq!(
            calls(&log, MANAGER + 0x14),
            vec![vec![manager, 0x20, 0x77], vec![manager, 0x21, 0x77]]
        );
        assert_eq!(calls(&log, OBJECT), vec![vec![this, 1]]);
    }

    #[test]
    fn the_utility_helper_passes_cache_search_flag_and_output_00d399f0() {
        let mut s = scene([0.0; 3], [0; 3], 0, 0);
        // A signalling NaN radius is quieted by the x87 load and store.
        let shape_b = s.e.mem.u32(s.b);
        s.e.mem.set_u32(shape_b + 0x10, SIGNALLING_NAN);
        let out = s.e.mem.alloc(0x80);
        s.e.call(0x00d3_99f0, &[s.a, s.b, s.input, 0xcace, 1, out]);
        assert_eq!(*s.seen.borrow(), vec![world()]);
        let call = &calls(&s.log, CAPS_VS_TRI)[0];
        assert_eq!(call[1], 1.0f32.to_bits());
        assert_eq!(call[3], 0x7fe0_0001);
        assert_eq!(call[4..], [0xcace, 1.0f32.to_bits(), 1, out, 0]);
    }

    #[test]
    fn the_second_contact_wins_ties_and_unordered_gaps_00d39bf0() {
        // (gap 0, gap 1) -> the contact picked at tolerance 1.0.
        let cases: [(f32, f32, Option<usize>); 9] = [
            (0.5, 0.2, Some(1)),
            (0.2, 0.5, Some(0)),
            (0.3, 0.3, Some(1)),
            (f32::NAN, 0.4, Some(1)),
            (0.4, f32::NAN, None),
            (2.0, 3.0, None),
            (3.0, 2.0, None),
            (1.0, 1.0, None),
            (0.9, 1.0, Some(0)),
        ];
        for (gap_0, gap_1, picked) in cases {
            let mut s = scene([gap_0, gap_1, 0.0], [0; 3], 0, 0);
            let out = s.e.mem.alloc(0x20);
            let result = s.e.call(0x00d3_9bf0, &[s.a, s.b, s.input, 0xcace, out]);
            assert_eq!(*s.seen.borrow(), vec![world()]);
            let call = &calls(&s.log, CAPS_VS_TRI)[0];
            assert_eq!(call[4..7], [0xcace, 1.0f32.to_bits(), 0]);
            match picked {
                Some(index) => {
                    assert_eq!(result.u32(), 1, "gaps {gap_0} {gap_1}");
                    let gap = [gap_0, gap_1][index];
                    let got = floats(&s.e, out);
                    let want = contact(index, gap);
                    assert_eq!(got[..7], want[..7]);
                    assert_eq!(got[7].to_bits(), want[7].to_bits());
                }
                None => assert_eq!(result.u32(), 0, "gaps {gap_0} {gap_1}"),
            }
            assert!(calls(&s.log, ASK).is_empty());
        }
    }

    #[test]
    fn the_triangle_is_asked_about_a_picked_feature_00d39bf0() {
        // (flag, kind, answer) -> (asked, result, normal xyz changed).
        let cases: [(u32, u8, u32, bool, u32, bool); 6] = [
            (1, 2, 0, true, 0, false),
            (1, 2, 1, true, 1, true),
            (1, 2, 2, true, 1, false),
            (1, 0, 1, false, 1, false),
            (0, 2, 1, false, 1, false),
            (7, 0xff, 1, true, 1, true),
        ];
        for (flag, kind, answer, asked, result, changed) in cases {
            // The second contact is picked (gap 0.2 < 0.5).
            let mut s = scene([0.5, 0.2, 0.0], [0, kind, 0], flag, answer);
            let out = s.e.mem.alloc(0x20);
            let got = s.e.call(0x00d3_9bf0, &[s.a, s.b, s.input, 0xcace, out]);
            assert_eq!(got.u32(), result, "flag {flag} kind {kind} answer {answer}");
            let ask = calls(&s.log, ASK);
            assert_eq!(ask.len(), usize::from(asked));
            if asked {
                let util = &calls(&s.log, CAPS_VS_TRI)[0];
                let shape_b = s.e.mem.u32(s.b);
                let shape_a = s.e.mem.u32(s.a);
                let (features, contacts) = (util[8], util[7]);
                let call = &ask[0];
                assert_eq!(call[0], shape_b);
                assert_eq!(call[1], features + 8 + 2);
                assert_eq!(call[3], contacts + 0x20);
                assert_eq!(call[4], s.e.mem.u32(s.b + 8));
                assert_eq!(call[5], shape_a);
                assert_eq!(call[6], s.e.mem.u32(s.a + 8));
                let seen = &calls(&s.log, ASK_SEEN)[0];
                assert_eq!(seen[0], u32::from(kind));
                assert_eq!(seen[1..4], [11, 2, 3]);
                let normal = [0.5f32, 0.25, 0.125, 0.2].map(f32::to_bits);
                assert_eq!(seen[4..], normal);
            }
            if result == 1 {
                let normal = floats(&s.e, out)[4..].to_vec();
                if changed {
                    assert_eq!(normal, vec![9.0, 8.0, 7.0, 0.2]);
                } else {
                    assert_eq!(normal, vec![0.5, 0.25, 0.125, 0.2]);
                }
            }
        }
    }

    /// Runs the closest point query `address` (`virtual_method`: with a `this`) and checks
    /// the contact the collector got and the profile records.
    fn closest_points(address: u32, virtual_method: bool) {
        let mut s = scene([0.5, 0.2, 0.0], [0; 3], 0, 0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let got: Rc<RefCell<Vec<[f32; 8]>>> = Rc::new(RefCell::new(Vec::new()));
        let bodies: Rc<RefCell<Vec<[u32; 2]>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&got);
        let copy_bodies = Rc::clone(&bodies);
        s.e.register_double(COLLECTOR + 4, move |e, a| {
            copy.borrow_mut().push(floats(e, a[1]));
            copy_bodies
                .borrow_mut()
                .push([e.mem.u32(a[1] + 0x20), e.mem.u32(a[1] + 0x24)]);
            Ret::default()
        });
        let this = 0x4000_0000;
        let words = if virtual_method {
            vec![this, s.a, s.b, s.input, collector]
        } else {
            vec![s.a, s.b, s.input, collector]
        };
        s.e.call(address, &words);
        assert_eq!(*s.seen.borrow(), vec![world()]);
        assert_eq!(*got.borrow(), vec![contact(1, 0.2)]);
        // The hkpCdPoint carries the two bodies after the contact.
        assert_eq!(*bodies.borrow(), vec![[s.a, s.b]]);
        let util = &calls(&s.log, CAPS_VS_TRI)[0];
        if virtual_method {
            assert_eq!(util[4], this + 0x14);
            assert!(calls(&s.log, TRIANGLE_PREPARE).is_empty());
        } else {
            // The static version makes the cache from the triangle's vertices.
            let shape_b = s.e.mem.u32(s.b);
            let prepare = &calls(&s.log, TRIANGLE_PREPARE)[0];
            assert_eq!(prepare[0], shape_b + 0x20);
            assert_eq!(util[4], prepare[1]);
        }
        assert_eq!(util[6], 0);
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_TRIANGLE, LABEL_END]
        );
    }

    #[test]
    fn closest_points_hand_the_picked_contact_to_the_collector_00d39ef0() {
        closest_points(0x00d3_9ef0, true);
    }

    #[test]
    fn static_closest_points_make_their_cache_00d3a280() {
        closest_points(0x00d3_a280, false);
    }

    /// Runs `address` with a collector and no contact picked.
    fn closest_points_without_contact(address: u32, virtual_method: bool) {
        let mut s = scene([2.0, 3.0, 0.0], [0; 3], 0, 0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let words = if virtual_method {
            vec![0x4000_0000, s.a, s.b, s.input, collector]
        } else {
            vec![s.a, s.b, s.input, collector]
        };
        s.e.call(address, &words);
        assert!(calls(&s.log, COLLECTOR + 4).is_empty());
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_TRIANGLE, LABEL_END]
        );
    }

    #[test]
    fn closest_points_without_a_contact_tell_nobody_00d39ef0() {
        closest_points_without_contact(0x00d3_9ef0, true);
        closest_points_without_contact(0x00d3_a280, false);
    }

    #[test]
    fn closest_points_drop_a_contact_the_triangle_refuses_00d39ef0() {
        let mut s = scene([0.5, 0.2, 0.0], [0, 3, 0], 1, 0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        s.e.call(0x00d3_9ef0, &[0x4000_0000, s.a, s.b, s.input, collector]);
        assert_eq!(calls(&s.log, ASK).len(), 1);
        assert!(calls(&s.log, COLLECTOR + 4).is_empty());
    }

    /// Runs the penetration query `address` for the gaps and tells whether the collector was
    /// told, checking the profile records and the bodies it was given.
    fn penetration(address: u32, virtual_method: bool, gaps: [f32; 2]) -> bool {
        let mut s = scene([gaps[0], gaps[1], 0.0], [0; 3], 0, 0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let this = 0x4000_0000;
        let words = if virtual_method {
            vec![this, s.a, s.b, s.input, collector]
        } else {
            vec![s.a, s.b, s.input, collector]
        };
        s.e.call(address, &words);
        assert_eq!(*s.seen.borrow(), vec![world()]);
        let util = &calls(&s.log, CAPS_VS_TRI)[0];
        assert_eq!(util[6], 0);
        assert_eq!(util[8], 0);
        if virtual_method {
            assert_eq!(util[4], this + 0x14);
        } else {
            assert_eq!(calls(&s.log, TRIANGLE_PREPARE).len(), 1);
        }
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_TRIANGLE, LABEL_END]
        );
        let told = calls(&s.log, COLLECTOR + 4);
        if !told.is_empty() {
            assert_eq!(told, vec![vec![collector, s.a, s.b]]);
        }
        !told.is_empty()
    }

    /// (gap 0, gap 1) -> whether a gap is below 0 (a NaN is not).
    const PENETRATION_CASES: [(f32, f32, bool); 8] = [
        (-0.1, 5.0, true),
        (5.0, -0.1, true),
        (0.0, 0.0, false),
        (f32::NAN, -1.0, true),
        (f32::NAN, f32::NAN, false),
        (-1.0, f32::NAN, true),
        (0.1, 0.2, false),
        (-0.0, -0.0, false),
    ];

    #[test]
    fn penetration_is_reported_for_a_gap_below_zero_00d3a620() {
        for (first, second, told) in PENETRATION_CASES {
            assert_eq!(penetration(0x00d3_a620, true, [first, second]), told);
        }
    }

    #[test]
    fn static_penetration_is_reported_for_a_gap_below_zero_00d3a8c0() {
        for (first, second, told) in PENETRATION_CASES {
            assert_eq!(penetration(0x00d3_a8c0, false, [first, second]), told);
        }
    }

    #[test]
    fn the_direct_entry_holds_the_direct_functions_00d3ab60() {
        let mut e = engine();
        let entry = e.mem.alloc(0x14);
        e.mem.write(entry, &[0xaa; 0x14]);
        e.call(0x00d3_ab60, &[entry]);
        let words: Vec<u32> = (0..4).map(|n| e.mem.u32(entry + 4 * n)).collect();
        assert_eq!(
            words,
            vec![0x00d3_98c0, 0x00d3_a8c0, 0x00d3_a280, 0x00da_f8e0]
        );
        assert_eq!(e.mem.bytes(entry + 0x10, 4), vec![0, 0, 0xaa, 0xaa]);
    }

    #[test]
    fn the_symmetric_entry_holds_the_symmetric_functions_00d3ab90() {
        let mut e = engine();
        let entry = e.mem.alloc(0x14);
        e.mem.write(entry, &[0xaa; 0x14]);
        e.call(0x00d3_ab90, &[entry]);
        let words: Vec<u32> = (0..4).map(|n| e.mem.u32(entry + 4 * n)).collect();
        assert_eq!(
            words,
            vec![0x00d3_9980, 0x00d3_b1a0, 0x00d3_b220, 0x00d3_ac80]
        );
        assert_eq!(e.mem.bytes(entry + 0x10, 4), vec![1, 0, 0xaa, 0xaa]);
    }

    /// Runs the registration `address`, whose registrar `register` is a double that notes the
    /// call and the entry's words and flag bytes: `(dispatcher, entry, type a, type b, words,
    /// flags)`.
    fn registrations(address: u32, register: u32) -> Vec<Vec<u32>> {
        let mut e = engine();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(register, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[1] + 4 * n)));
            words.extend([0x10, 0x11].map(|at| u32::from(e.mem.u8(a[1] + at))));
            copy.borrow_mut().push(words);
            Ret::default()
        });
        e.call(address, &[0xd15b]);
        let result = seen.borrow().clone();
        result
    }

    fn check_registrations(seen: &[Vec<u32>]) {
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][0], 0xd15b);
        assert_eq!(
            seen[0][2..],
            [
                3,
                5,
                0x00d3_9980,
                0x00d3_b1a0,
                0x00d3_b220,
                0x00d3_ac80,
                1,
                0
            ]
        );
        assert_eq!(seen[1][0], 0xd15b);
        assert_eq!(
            seen[1][2..],
            [
                5,
                3,
                0x00d3_98c0,
                0x00d3_a8c0,
                0x00d3_a280,
                0x00da_f8e0,
                0,
                0
            ]
        );
        assert_eq!(seen[0][1], seen[1][1]);
    }

    #[test]
    fn registration_adds_the_symmetric_then_the_direct_entry_00d3abc0() {
        check_registrations(&registrations(0x00d3_abc0, REGISTER_AGENT));
    }

    #[test]
    fn registration_through_the_other_function_00d3ac20() {
        check_registrations(&registrations(0x00d3_ac20, REGISTER_AGENT_OTHER));
    }

    /// Runs the symmetric linear cast with the given start collector; returns what the cast
    /// saw: `(bodies, copy, first, second)` words and the path vector lanes of the copy.
    fn linear_cast(start: u32) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let mut e = engine();
        let seen: Rc<RefCell<Vec<(Vec<u32>, Vec<u32>, Vec<u32>)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(LINEAR_CAST, move |e, a| {
            let path = (0..4).map(|n| e.mem.u32(a[2] + 0x50 + 4 * n)).collect();
            let first = (0..3).map(|n| e.mem.u32(a[3] + 0x10 * (n + 1))).collect();
            copy.borrow_mut().push((a.to_vec(), path, first));
            Ret::default()
        });
        let input = e.mem.alloc(0x70);
        for lane in 0..4u32 {
            e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
        }
        for (n, bits) in [1.0f32, -2.0, 0.0, 4.0].into_iter().enumerate() {
            e.mem.set_f32(input + 0x50 + 4 * n as u32, bits);
        }
        e.call(0x00d3_ac80, &[0xa, 0xb, input, 0xc0, start]);
        let result = seen.borrow()[0].clone();
        assert_eq!(result.0[..2], [0xb, 0xa]);
        assert_ne!(result.0[2], input);
        result
    }

    #[test]
    fn symmetric_linear_cast_swaps_the_bodies_and_negates_the_path_00d3ac80() {
        let (words, path, _) = linear_cast(0xd0);
        assert_eq!(path, [-1.0f32, 2.0, -0.0, -4.0].map(f32::to_bits));
        assert_ne!(words[3], 0);
        assert_ne!(words[4], 0);
        let (words, _, _) = linear_cast(0);
        assert_ne!(words[3], 0);
        assert_eq!(words[4], 0);
    }

    /// The word the collector double of `flip_slot` sits at.
    const FLIP_SLOT_TARGET: u32 = 0x0940_0004;

    /// What the flip collector's slot 4 was asked: the arguments and, at the time of the
    /// call, the three words of the collector (`this`).
    type SlotCalls = Rc<RefCell<Vec<(Vec<u32>, Vec<u32>)>>>;

    /// Makes slot 4 of the vtable at `vtable` a double that notes its arguments and the three
    /// words of the collector it was called on.
    fn flip_slot(s: &mut Scene, vtable: u32) -> SlotCalls {
        let seen: SlotCalls = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        s.e.mem.set_u32(vtable + 4, FLIP_SLOT_TARGET);
        s.e.register_double(FLIP_SLOT_TARGET, move |e, a| {
            let words = (0..3).map(|n| e.mem.u32(a[0] + 4 * n)).collect();
            copy.borrow_mut().push((a.to_vec(), words));
            Ret::default()
        });
        seen
    }

    /// A scene whose first contact has a gap below 0 (and so counts for the penetration
    /// query, and, being below the tolerance 1.0, for the closest point query).
    fn flip_scene(vtable: u32) -> (Scene, SlotCalls, u32) {
        let mut s = scene([-0.5, 2.0, 2.0], [0; 3], 0, 0);
        s.e.mem.set_u32(LARGE_FLOAT, SIGNALLING_NAN);
        let seen = flip_slot(&mut s, vtable);
        let this = s.e.mem.alloc(0x40);
        (s, seen, this)
    }

    /// The flip body collector (vtable `010ce81c`, byte 0, the caller's word) as the
    /// penetration query told it about the capsule and the triangle.
    fn check_penetration_flip(s: &Scene, seen: &SlotCalls) {
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1..], [s.a, s.b]);
        assert_eq!(seen[0].1, vec![FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
        assert_eq!(*s.seen.borrow(), vec![world()]);
    }

    /// The flip collector (vtable `010ce7fc`, the quieted large float, the caller's word) as
    /// the closest point query handed the picked contact to it.
    fn check_closest_points_flip(s: &Scene, seen: &SlotCalls) {
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![FLIP_COLLECTOR_VTABLE, 0x7fe0_0001, 0x55]);
        let copy = seen[0].0[1];
        assert_eq!(floats(&s.e, copy), contact(0, -0.5));
        assert_eq!(*s.seen.borrow(), vec![world()]);
    }

    #[test]
    fn symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3b160() {
        let (mut s, seen, this) = flip_scene(FLIP_BODY_COLLECTOR_VTABLE);
        s.e.call(0x00d3_b160, &[this, s.b, s.a, s.input, 0x55]);
        check_penetration_flip(&s, &seen);
    }

    #[test]
    fn static_symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3b1a0() {
        let (mut s, seen, _) = flip_scene(FLIP_BODY_COLLECTOR_VTABLE);
        s.e.call(0x00d3_b1a0, &[s.b, s.a, s.input, 0x55]);
        check_penetration_flip(&s, &seen);
    }

    #[test]
    fn symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3b1e0() {
        let (mut s, seen, this) = flip_scene(FLIP_COLLECTOR_VTABLE);
        s.e.call(0x00d3_b1e0, &[this, s.b, s.a, s.input, 0x55]);
        check_closest_points_flip(&s, &seen);
    }

    #[test]
    fn static_symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3b220() {
        let (mut s, seen, _) = flip_scene(FLIP_COLLECTOR_VTABLE);
        s.e.call(0x00d3_b220, &[s.b, s.a, s.input, 0x55]);
        check_closest_points_flip(&s, &seen);
    }

    /// What a run of `processCollision` left behind.
    struct Processed {
        s: Scene,
        this: u32,
        output: u32,
        free: u32,
    }

    /// Runs `processCollision` (`00d3ad70`) on a scene with the given gaps, feature kinds,
    /// answer of the triangle shape, contact ids and the id the contact manager makes.
    fn process(gaps: [f32; 3], kinds: [u8; 3], answer: u32, ids: [u16; 3], made: u32) -> Processed {
        let mut s = scene(gaps, kinds, 0, answer);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        log_double(&mut s.e, &s.log, MANAGER + 0xc, made);
        let this = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(this + 8, manager);
        for (n, id) in ids.into_iter().enumerate() {
            s.e.mem.set_u16(this + 0xc + 2 * n as u32, id);
        }
        let free = s.e.mem.alloc(0x100);
        let output = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(output, free);
        s.e.mem.set_u32(output + 4, 0x7777);
        s.e.call(0x00d3_ad70, &[this, s.a, s.b, s.input, output]);
        assert_eq!(*s.seen.borrow(), vec![world()]);
        let util = &calls(&s.log, CAPS_VS_TRI)[0];
        assert_eq!(util[4..7], [this + 0x14, 1.0f32.to_bits(), 1]);
        assert_ne!(util[8], 0);
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_PROCESS, LABEL_END]
        );
        Processed {
            s,
            this,
            output,
            free,
        }
    }

    fn ids(p: &Processed) -> Vec<u16> {
        (0..3)
            .map(|n| p.s.e.mem.u16(p.this + 0xc + 2 * n))
            .collect()
    }

    #[test]
    fn a_new_close_contact_gets_an_id_and_is_added_00d3ad70() {
        let p = process([0.5, 2.0, f32::NAN], [0; 3], 0, [0xffff, 7, 0xffff], 0x42);
        let manager = p.s.e.mem.u32(p.this + 8);
        let util = &calls(&p.s.log, CAPS_VS_TRI)[0];
        let created = calls(&p.s.log, MANAGER + 0xc);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..3], [manager, p.s.a, p.s.b]);
        assert_eq!(created[0][3..], [p.s.input, p.output, 0, util[7]]);
        // The second contact is too far: its id is given back with `[output + 4]`.
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777]]
        );
        assert_eq!(ids(&p), vec![0x42, 0xffff, 0xffff]);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(floats(&p.s.e, p.free), contact(0, 0.5));
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 0x42);
        assert!(calls(&p.s.log, ASK).is_empty());
    }

    #[test]
    fn contacts_with_ids_are_added_without_asking_and_far_ones_are_released_00d3ad70() {
        let p = process([0.9, 2.0, f32::NAN], [0; 3], 0, [3, 7, 9], 0x42);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        // The too far (2.0) and the unordered (NaN) contact give their ids back.
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777], vec![manager, 9, 0x7777]]
        );
        assert_eq!(ids(&p), vec![3, 0xffff, 0xffff]);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 3);
    }

    #[test]
    fn a_contact_the_manager_gives_no_id_is_skipped_00d3ad70() {
        let p = process([0.5, 2.0, 2.0], [0; 3], 0, [0xffff; 3], 0xffff);
        assert_eq!(calls(&p.s.log, MANAGER + 0xc).len(), 1);
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert_eq!(ids(&p), vec![0xffff; 3]);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
    }

    #[test]
    fn the_triangle_decides_about_a_feature_of_a_close_contact_00d3ad70() {
        // Answer 0 drops the contact and gives its id back.
        let p = process([0.5, 2.0, 2.0], [3, 0, 0], 0, [5, 0xffff, 0xffff], 0x42);
        let manager = p.s.e.mem.u32(p.this + 8);
        let ask = calls(&p.s.log, ASK);
        assert_eq!(ask.len(), 1);
        let util = &calls(&p.s.log, CAPS_VS_TRI)[0];
        assert_eq!(ask[0][1], util[8] + 2);
        assert_eq!(ask[0][3], util[7]);
        assert_eq!(calls(&p.s.log, ASK_SEEN)[0][..4], [3, 1, 2, 3]);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 5, 0x7777]]
        );
        assert_eq!(ids(&p), vec![0xffff; 3]);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        // Answer 1 puts the changed vector's first three lanes into the normal.
        let p = process([0.5, 2.0, 2.0], [3, 0, 0], 1, [5, 0xffff, 0xffff], 0x42);
        assert_eq!(
            floats(&p.s.e, p.free),
            [1.0, 2.0, 3.0, 4.0, 9.0, 8.0, 7.0, 0.5]
        );
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 5);
        // Any other answer keeps the contact as it is.
        let p = process([0.5, 2.0, 2.0], [3, 0, 0], 2, [5, 0xffff, 0xffff], 0x42);
        assert_eq!(floats(&p.s.e, p.free), contact(0, 0.5));
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
    }

    /// Runs the symmetric `processCollision` (`00d3b260`) with the bodies in the other order
    /// on a scene whose contact manager sets the output's time of impact to `new_time` when
    /// it makes the first contact's id; returns the scene, the output and its first free
    /// contact point before the call.
    fn symmetric_process(new_time: f32) -> (Scene, u32, u32) {
        let mut s = scene([0.5, 2.0, 2.0], [0; 3], 0, 0);
        for lane in 0..4u32 {
            let sign = if lane < 3 { 0x8000_0000 } else { 0 };
            s.e.mem.set_u32(0x010c_4c00 + 4 * lane, sign);
        }
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        s.e.register_double(MANAGER + 0xc, move |e, a| {
            e.mem.set_f32(a[4] + 0x3030, new_time);
            Ret {
                eax: 0x42,
                ..Ret::default()
            }
        });
        let this = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(this + 8, manager);
        for n in 0..3 {
            s.e.mem.set_u16(this + 0xc + 2 * n, 0xffff);
        }
        let output = s.e.mem.alloc(0x3100);
        let free = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(output, free);
        s.e.mem.set_f32(output + 0x3030, 0.5);
        s.e.mem.set_u8(output + 0x3048, 1);
        s.e.mem.set_u8(output + 0x3049, 2);
        s.e.mem.set_u8(output + 0x304a, 0x12);
        for n in 0..4 {
            s.e.mem.set_u16(output + 0x3040 + 2 * n, 10 + n as u16);
        }
        store_vec(&mut s.e, output + 0x3020, [1.0, 2.0, 3.0, 4.0]);
        s.e.call(0x00d3_b260, &[this, s.b, s.a, s.input, output]);
        // The capsule is body A of the inner call.
        assert_eq!(*s.seen.borrow(), vec![world()]);
        (s, output, free)
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contacts_00d3b260() {
        let (s, output, free) = symmetric_process(0.5);
        assert_eq!(s.e.mem.u32(output), free + 0x30);
        // Position += normal.w * normal, normal.xyz negated.
        assert_eq!(load_vec(&s.e, free), [1.25, 2.125, 3.0625, 4.25]);
        assert_eq!(load_vec(&s.e, free + 0x10), [-0.5, -0.25, -0.125, 0.5]);
        // The time did not change: the TOI data stays.
        assert_eq!(load_vec(&s.e, output + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(s.e.mem.u8(output + 0x304a), 0x12);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes_00d3b260() {
        let (s, output, _) = symmetric_process(0.25);
        assert_eq!(load_vec(&s.e, output + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(s.e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(s.e.mem.u8(output + 0x3048), 2);
        assert_eq!(s.e.mem.u8(output + 0x3049), 1);
        let words: Vec<u16> = (0..4)
            .map(|n| s.e.mem.u16(output + 0x3040 + 2 * n))
            .collect();
        assert_eq!(words, vec![12, 11, 10, 13]);
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
    fn fn_00d39980_fills_the_block_and_copies_the_shape_data() {
        let (mut e, sizes) = allocator_engine();
        let shape = e.mem.alloc(0x40);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cd_72a0, move |e, a| {
            // The block already holds the first vtable word while the data is copied.
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[1] - 0x14)));
            Ret::default()
        });
        let block = e
            .call(0x00d3_9980, &args![body, 0x2000u32, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x28]);
        assert_eq!(
            *seen.borrow(),
            vec![(vec![shape + 0x20, block + 0x14], 0x010c_eefc)]
        );
        assert_eq!(e.mem.u16(block + 4), 0x28);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_ef38);
        for offset in [0xc, 0xe, 0x10] {
            assert_eq!(e.mem.u16(block + offset), 0xffff);
        }
    }
}
