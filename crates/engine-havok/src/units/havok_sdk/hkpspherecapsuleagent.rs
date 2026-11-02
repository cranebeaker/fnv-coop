//! `Havok SDK/hkpspherecapsuleagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpSphereCapsuleAgent` is the collision agent of a sphere (body A) and a capsule
//! (body B). Its base is the Xbox PDB's `hkpCollisionAgent`: contact manager `+8` and the one
//! manifold contact point id (16-bit, `0xFFFF` = none) at `+0xC`. A body is an `hkpCdBody`
//! (shape `+0`, motion `+8`): the sphere's position is the translation row at `motion + 0x30`
//! and its radius sits at `shape + 0x10`; the capsule has its radius at `shape + 0x10` and its
//! end points at `shape + 0x20` and `shape + 0x30` (taken to world space by the motion's four
//! rows). The contact of all three queries is built by `contact`: the point of the capsule's
//! segment closest to the sphere's centre (`hkLineSegmentUtil::closestPointLineSeg`, `00d230e0`),
//! the normal from there to the centre (for a centre on the segment, a vector perpendicular to
//! the segment), and the position and penetration. `RSQRTSS` is the host CPU's estimate.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul};
use crate::units::havok_sdk::hkpbvagent::{
    symmetric_process_collision, with_flip_body_collector, with_flip_collector,
};
use crate::units::havok_sdk::hkpcapsulecapsuleagent::{store_ends, world_ends};
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_vec, store_vec, sum3, zip_with, Vec4,
};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: `"TtSphereCapsule"` and the shared end label.
const LABEL_SPHERE_CAPSULE: u32 = 0x010c_f158;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkLineSegmentUtil::closestPointLineSeg` (cdecl: the point, the segment's two ends, the
/// 16-byte result).
const CLOSEST_POINT_ON_SEGMENT: u32 = 0x00d2_30e0;
/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkpCollisionDispatcher` agent registration (`00cfbbe0`, `this` = dispatcher, `RET 0xC`):
/// `(agent function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;
/// The bounding-volume pair's linear cast (cdecl: two bodies, input, two collectors), which
/// the direct entry registers, and the symmetric static linear cast of the capsule-triangle
/// pair (`00d3ac80`) the symmetric entry registers.
const LINEAR_CAST: u32 = 0x00da_f8e0;
const SYMMETRIC_LINEAR_CAST: u32 = 0x00d3_ac80;

/// The contact `contact` finds: the position, and the unit normal whose fourth lane holds the
/// separating distance (the distance minus the radius sum).
struct Contact {
    position: Vec4,
    normal: Vec4,
}

/// The unit vector perpendicular to the capsule's axis the exe builds for a sphere centre on
/// the segment: `direction` is the end `+0x30` minus the end `+0x20`; of the absolute values
/// of its x, y and z, the smaller of x and y (x when they are equal or not comparable) is
/// compared with z, and the vector keeps the third component (z when the compare does not
/// find z smaller) of the direction in the place of the first one and the negated first in the
/// place of the third. Every load is through the x87 (a signalling NaN comes out quiet).
fn perpendicular(ends: [Vec4; 2]) -> Vec4 {
    let direction = zip_with(ends[1], ends[0], |end, start| end - start);
    let magnitude = |lane: usize| x87_quiet(direction[lane]).abs();
    let (x, y, z) = (magnitude(0), magnitude(1), magnitude(2));
    let (other, kept, least) = if x > y { (0, 1, y) } else { (1, 0, x) };
    let smallest = if least > z { kept } else { 2 };
    let mut normal = [0.0f32; 4];
    normal[other] = x87_quiet(direction[smallest]);
    normal[smallest] = f32::from_bits(x87_quiet_bits(direction[other].to_bits()) ^ 0x8000_0000);
    normal
}

/// The sphere-capsule contact of the bodies, or `None` when they are further apart than
/// `tolerance + radius sum` (the tolerance is the float at `input + 8`). The test is made on
/// squares: the squared distance of the sphere's centre from the closest point of the segment
/// (`z + (y + x)` of the squared lanes) against the square of the reach, which is rounded to a
/// `float` first; the bodies are apart when the square of the reach is not above the squared
/// distance (a NaN counts as touching). The distance is the CRT's `sqrt` (`00ec6040`) of the
/// squared distance when that is above zero (else `0`, with a perpendicular normal). The
/// position is `centre + (capsule radius - distance) * normal` and the normal's fourth lane
/// `distance - radius sum`; the x87 sums, differences and the product are done in `f64` and
/// rounded to a `float` where the exe stores one.
fn contact(e: &mut Engine, body_a: u32, body_b: u32, input: u32) -> Option<Contact> {
    let shape_a = e.mem.u32(body_a);
    let centre_at = e.mem.u32(body_a + 8) + 0x30;
    let shape_b = e.mem.u32(body_b);
    let ends = world_ends(e, body_b);
    let closest = e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        store_ends(e, frame, ends);
        e.call(
            CLOSEST_POINT_ON_SEGMENT,
            &args![centre_at, frame, frame + 0x10, frame + 0x20],
        );
        load_vec(e, frame + 0x20)
    });
    let radius_a = f64::from(e.mem.f32(shape_a + 0x10));
    let radius_b = e.mem.f32(shape_b + 0x10);
    let radius_sum = (radius_a + f64::from(radius_b)) as f32;
    let centre = load_vec(e, centre_at);
    let difference = zip_with(centre, closest, |centre, closest| centre - closest);
    let reach = (f64::from(e.mem.f32(input + 8)) + f64::from(radius_sum)) as f32;
    let distance_squared = sum3(mul(difference, difference));
    let reach_squared = (f64::from(reach) * f64::from(reach)) as f32;
    if reach_squared <= distance_squared {
        return None;
    }
    let (normal, distance) = if distance_squared > 0.0 {
        let root = e.call(X87_SQRT, &args![f64::from(distance_squared)]).f64() as f32;
        (difference, root)
    } else {
        (perpendicular(ends), 0.0)
    };
    let scale = inverse_length(e, sum3(mul(normal, normal)));
    let mut normal = mul([scale; 4], normal);
    let gap = (f64::from(radius_b) - f64::from(distance)) as f32;
    let position = add(mul([gap; 4], normal), centre);
    normal[3] = (f64::from(distance) - f64::from(radius_sum)) as f32;
    Some(Contact { position, normal })
}

// Translated from 00d3e2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereCapsuleAgent::cleanup` (Xbox PDB): gives the contact point id (`+0xC`) back to the
/// contact manager (`[this + 8]`, slot `0x14`, with the id and `param`) when it is not
/// `0xFFFF`, then runs the agent's deleting destructor (slot 0, argument 1).
pub fn hkp_sphere_capsule_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let id = e.mem.u16(this.addr() + 0xc);
    if id != 0xffff {
        let manager = e.mem.u32(this.addr() + 8);
        e.vcall(manager, 0x14, &args![u32::from(id), param]);
    }
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d3e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereCapsuleAgent::staticGetPenetrations` (Xbox PDB): a cdecl function over the bodies,
/// a word it does not read and the collector. Profiled (`"TtSphereCapsule"`). Finds the
/// closest point of the capsule's segment to the sphere's centre and tells the collector
/// (slot 4, with the two bodies) when the squared distance is below the square of the radius
/// sum (rounded to a `float`; an x87 compare, so a NaN is not below).
pub fn hkp_sphere_capsule_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_3: u32,
    collector: Ptr,
) {
    profile_record(e, LABEL_SPHERE_CAPSULE);
    let shape_a = e.mem.u32(body_a.addr());
    let centre_at = e.mem.u32(body_a.addr() + 8) + 0x30;
    let shape_b = e.mem.u32(body_b.addr());
    let ends = world_ends(e, body_b.addr());
    let closest = e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        store_ends(e, frame, ends);
        e.call(
            CLOSEST_POINT_ON_SEGMENT,
            &args![centre_at, frame, frame + 0x10, frame + 0x20],
        );
        load_vec(e, frame + 0x20)
    });
    let radius_sum =
        (f64::from(e.mem.f32(shape_b + 0x10)) + f64::from(e.mem.f32(shape_a + 0x10))) as f32;
    let centre = load_vec(e, centre_at);
    let difference = zip_with(centre, closest, |centre, closest| centre - closest);
    let distance_squared = sum3(mul(difference, difference));
    let reach_squared = (f64::from(radius_sum) * f64::from(radius_sum)) as f32;
    if reach_squared > distance_squared {
        e.vcall(collector.addr(), 4, &args![body_a, body_b]);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereCapsuleAgent::getPenetrations` (Xbox PDB): runs `staticGetPenetrations` on the
/// four words (`this` is not used).
pub fn hkp_sphere_capsule_agent_get_penetrations(
    e: &mut Engine,
    _this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    collector: Ptr,
) {
    hkp_sphere_capsule_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

// Translated from 00d3e530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sphere-capsule contact written to a 0x20-byte contact (the map gives the function no
/// name; cdecl, no profiling): when `contact` finds one, writes the position at `output`, the
/// normal (fourth lane: the separating distance) at `output + 0x10` and answers 1; else
/// answers 0 and leaves the output alone. The tolerance is the float at `input + 8`.
pub fn fn_00d3e530(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: Ptr) -> u32 {
    match contact(e, body_a.addr(), body_b.addr(), input.addr()) {
        Some(found) => {
            store_vec(e, output.addr(), found.position);
            store_vec(e, output.addr() + 0x10, found.normal);
            1
        }
        None => 0,
    }
}

/// The body of `getClosestPoints` (the virtual `00d3e850` and the cdecl `00d3ebe0` are the
/// same instructions): profiled; when `contact` finds a contact builds an `hkpCdPoint` on the
/// stack (the contact, then the two bodies at `+0x20` and `+0x24`) and passes it to the
/// collector's slot 4.
fn closest_points(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    profile_record(e, LABEL_SPHERE_CAPSULE);
    if let Some(found) = contact(e, body_a.addr(), body_b.addr(), input.addr()) {
        e.with_stack(0x28, |e, point| {
            let point = point.addr();
            store_vec(e, point, found.position);
            store_vec(e, point + 0x10, found.normal);
            e.mem.set_u32(point + 0x20, body_a.addr());
            e.mem.set_u32(point + 0x24, body_b.addr());
            e.vcall(collector.addr(), 4, &args![point]);
        });
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereCapsuleAgent::getClosestPoints` (Xbox PDB): see `closest_points` (`this` is not
/// used).
pub fn hkp_sphere_capsule_agent_get_closest_points(
    e: &mut Engine,
    _this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d3ebe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cdecl `getClosestPoints` of the sphere-capsule pair (the map gives it no name): the same
/// instructions as `00d3e850`, without `this`.
pub fn fn_00d3ebe0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    closest_points(e, body_a, body_b, input, collector);
}

/// Fills the 20-byte agent function entry the registrations build on the stack: the four
/// function words and the two flag bytes (`+0x10`, `+0x11`). The two bytes after them are not
/// set by the exe (the callee copies the word they are in); here they are zero.
fn fill_entry(e: &mut Engine, entry: u32, words: [u32; 4], flags: [u8; 2]) {
    for (index, word) in words.into_iter().enumerate() {
        e.mem.set_u32(entry + 4 * index as u32, word);
    }
    e.mem.set_u8(entry + 0x10, flags[0]);
    e.mem.set_u8(entry + 0x11, flags[1]);
}

// Translated from 00d3ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the sphere-capsule agent with a dispatcher (the map gives it no name; cdecl, one
/// word): first the symmetric entry (`createFunc` `00d3e2f0`, `getPenetrations` `00d3f040`,
/// `getClosestPoints` `00d3f4c0`, `linearCast` `00d3ac80`; flag bytes 1 and 0) for the shape
/// types (5, 1), then the direct entry (`createFunc` `00d3e270`, `getPenetrations` `00d3e330`,
/// `getClosestPoints` `00d3ebe0`, `linearCast` `00daf8e0`; flag bytes 0 and 0) for (1, 5),
/// through `00cfbbe0`, both built in the same 20-byte block on the stack.
pub fn fn_00d3ef70(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        fill_entry(
            e,
            entry,
            [0x00d3_e2f0, 0x00d3_f040, 0x00d3_f4c0, SYMMETRIC_LINEAR_CAST],
            [1, 0],
        );
        e.call(REGISTER_AGENT, &args![dispatcher, entry, 5u32, 1u32]);
        fill_entry(
            e,
            entry,
            [0x00d3_e270, 0x00d3_e330, 0x00d3_ebe0, LINEAR_CAST],
            [0, 0],
        );
        e.call(REGISTER_AGENT, &args![dispatcher, entry, 1u32, 5u32]);
    });
}

// Translated from 00d3f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereCapsuleAgent>::getPenetrations` (Xbox PDB): runs
/// `hkpSphereCapsuleAgent::getPenetrations` with the bodies swapped and, in place of the
/// caller's collector word `output`, a flip-body collector (vtable `010ce81c`, byte 0 at
/// `+4`, the caller's word at `+8`).
pub fn hkp_symmetric_agent_linear_cast_sphere_capsule_agent_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_sphere_capsule_agent_get_penetrations(
            e,
            this,
            body_b,
            body_a,
            input.addr(),
            Ptr::new(flip),
        );
    });
}

// Translated from 00d3f040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getPenetrations` of the sphere-capsule pair (the map gives it no
/// name; cdecl): runs `00d3e330` with the bodies swapped and a flip-body collector around the
/// caller's word `output`.
pub fn fn_00d3f040(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_body_collector(e, output, |e, flip| {
        hkp_sphere_capsule_agent_static_get_penetrations(
            e,
            body_b,
            body_a,
            input.addr(),
            Ptr::new(flip),
        );
    });
}

// Translated from 00d3f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereCapsuleAgent::processCollision` (Xbox PDB): profiled. The record to fill is the
/// output's first free contact point (`[output]`, 0x30 bytes). When `contact` finds a contact
/// it is written there; then, when the agent has no contact point id (`+0xC` is `0xFFFF`),
/// the contact manager (`[this + 8]`) is asked for one through its slot `0xC` (with the two
/// bodies, the input, the output, 0 and the record; its answer is the 16-bit id). With an id,
/// the record's `+0x20` holds it and `[output]` moves on by `0x30`. When the bodies are
/// apart and an id is held, the manager's slot `0x14` takes it back (with `[output + 4]`) and
/// the id is cleared.
pub fn hkp_sphere_capsule_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, LABEL_SPHERE_CAPSULE);
    let id_at = this.addr() + 0xc;
    let record = e.mem.u32(output.addr());
    match contact(e, body_a.addr(), body_b.addr(), input.addr()) {
        Some(found) => {
            store_vec(e, record, found.position);
            store_vec(e, record + 0x10, found.normal);
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
        }
        None => {
            let id = e.mem.u16(id_at);
            if id != 0xffff {
                let manager = e.mem.u32(this.addr() + 8);
                let second = e.mem.u32(output.addr() + 4);
                e.vcall(manager, 0x14, &args![u32::from(id), second]);
                e.mem.set_u16(id_at, 0xffff);
            }
        }
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3f480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereCapsuleAgent>::getClosestPoints` (Xbox PDB): runs
/// `hkpSphereCapsuleAgent::getClosestPoints` with the bodies swapped and a flip collector
/// (vtable `010ce7fc`, the large float through `FLD`/`FSTP` at `+4`, the caller's word
/// `output` at `+8`).
pub fn hkp_symmetric_agent_linear_cast_sphere_capsule_agent_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: u32,
) {
    with_flip_collector(e, output, |e, flip| {
        hkp_sphere_capsule_agent_get_closest_points(e, this, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The symmetric static `getClosestPoints` of the sphere-capsule pair (the map gives it no
/// name; cdecl): runs `00d3ebe0` with the bodies swapped and a flip collector around the
/// caller's word `output`.
pub fn fn_00d3f4c0(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, output: u32) {
    with_flip_collector(e, output, |e, flip| {
        fn_00d3ebe0(e, body_b, body_a, input, Ptr::new(flip));
    });
}

// Translated from 00d3f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSymmetricAgentLinearCast<hkpSphereCapsuleAgent>::processCollision` (Xbox PDB): runs
/// `hkpSphereCapsuleAgent::processCollision` with the bodies swapped, then turns what it
/// appended to the output around, as `00d46220` does for the bounding-volume pair.
pub fn hkp_symmetric_agent_linear_cast_sphere_capsule_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    symmetric_process_collision(e, output.addr(), |e| {
        hkp_sphere_capsule_agent_process_collision(e, this, body_b, body_a, input, output);
    });
}

// Translated from 00d3e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the agent class is unconfirmed): a collision agent creation function,
/// cdecl, four stack words (nothing popped): the first three are never read (`_unused_0`, `_unused_1`, `_unused_2`) and the last is the contact manager, `manager`. Allocates 0x10 bytes through the memory
/// router's allocator (`00c85750`; virtual function at +4 of the object at router + 0x10) and fills
/// the block: the size as a 16-bit word at +4, the count 1 as a 16-bit word at +6, `manager` at
/// +8, `0xffff` as a 16-bit word at +0xc and the vtable word `010cf120` at +0. Returns the block. Ghidra made no function here; the address is an immediate operand at 00d3ef83.
pub fn fn_00d3e2f0(
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
    e.mem.set_u32(block, 0x010c_f120);
    e.mem.set_u16(block + 0xc, 0xffff);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_e2b0, hkp_sphere_capsule_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_e330,
            hkp_sphere_capsule_agent_static_get_penetrations(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d3_e510,
            hkp_sphere_capsule_agent_get_penetrations(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d3_e530, fn_00d3e530(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d3_e850,
            hkp_sphere_capsule_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_ebe0, fn_00d3ebe0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d3_ef70, fn_00d3ef70(Ptr)),
        entry!(
            0x00d3_f000,
            hkp_symmetric_agent_linear_cast_sphere_capsule_agent_get_penetrations(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_f040, fn_00d3f040(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_f080,
            hkp_sphere_capsule_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_f480,
            hkp_symmetric_agent_linear_cast_sphere_capsule_agent_get_closest_points(
                Ptr, Ptr, Ptr, Ptr, u32
            )
        ),
        entry!(0x00d3_f4c0, fn_00d3f4c0(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00d3_f500,
            hkp_symmetric_agent_linear_cast_sphere_capsule_agent_process_collision(
                Ptr, Ptr, Ptr, Ptr, Ptr
            )
        ),
        entry!(0x00d3_e2f0, fn_00d3e2f0(u32, u32, u32, u32) -> u32),
    ]
}

/// Fixtures the tests of the sphere agents share.
#[cfg(test)]
pub(crate) mod fixtures {
    use crate::prelude::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::engine;
    use crate::units::havok_sdk::hkpcapsulecapsuleagent::fixtures::{shaped_body, IDENTITY};
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};

    /// An engine with the exe's data range mapped, the constants of the normalising code
    /// (zero, three, a half) and the sign masks of the symmetric turn-around (all four lanes
    /// at `010c44d0`, the first three at `010c4c00`).
    pub(crate) fn primed_engine() -> Engine {
        let mut e = engine();
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        for lane in 0..4u32 {
            e.mem.set_u32(0x010c_44d0 + 4 * lane, 0x8000_0000);
            let sign = if lane < 3 { 0x8000_0000 } else { 0 };
            e.mem.set_u32(0x010c_4c00 + 4 * lane, sign);
        }
        e
    }

    /// A body whose shape holds `radius` at `+0x10` and whose motion is not turned and sits at
    /// `centre` (the translation row at `+0x30`, with a fourth lane of 1).
    pub(crate) fn sphere_body(e: &mut Engine, radius: f32, centre: [f32; 3]) -> u32 {
        let rows = [
            IDENTITY[0],
            IDENTITY[1],
            IDENTITY[2],
            [centre[0], centre[1], centre[2], 1.0],
        ];
        shaped_body(e, radius, &[], rows)
    }

    /// The eight floats of a contact (position, normal).
    pub(crate) fn floats(e: &Engine, at: u32) -> [f32; 8] {
        std::array::from_fn(|lane| e.mem.f32(at + 4 * lane as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpbvagent::{
        FLIP_BODY_COLLECTOR_VTABLE, FLIP_COLLECTOR_VTABLE, LARGE_FLOAT,
    };
    use crate::units::havok_sdk::hkpcapsulecapsuleagent::fixtures::{
        shaped_body, vector_double, Seen, IDENTITY,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const COLLECTOR: u32 = 0x0910_0000;
    const OBJECT: u32 = 0x0920_0000;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;

    /// The sphere's centre, the capsule's end points (in the capsule's space, which is the
    /// world: its motion is not moved) and the world vectors the closest point function is
    /// handed.
    const CENTRE: Vec4 = [1.0, 2.0, 3.0, 1.0];
    const ENDS: [Vec4; 2] = [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 10.0, 0.0]];
    const WORLD: [Vec4; 3] = [
        [1.0, 2.0, 3.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 10.0, 1.0],
    ];

    /// A sphere (radius 1, at `(1, 2, 3)`) and a capsule (radius 1) whose closest point to the
    /// sphere's centre the double of `00d230e0` answers as `closest`.
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

    fn scene(closest: Vec4, ends: [Vec4; 2], tolerance: f32) -> Scene {
        let mut e = primed_engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let a = sphere_body(&mut e, 1.0, [1.0, 2.0, 3.0]);
        let b = shaped_body(&mut e, 1.0, &ends, IDENTITY);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, tolerance);
        let seen = vector_double(
            &mut e,
            &log,
            CLOSEST_POINT_ON_SEGMENT,
            &[(0, 1), (1, 1), (2, 1)],
            0,
            move |e, a| store_vec(e, a[3], closest),
        );
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

    /// The closest point `(1, 2, 1.5)` leaves the bodies touching (distance 1.5 against a
    /// radius sum of 2), `(1, 2, 0)` does not (distance 3).
    const NEAR: Vec4 = [1.0, 2.0, 1.5, 1.0];
    const FAR: Vec4 = [1.0, 2.0, 0.0, 1.0];

    fn check_world(s: &Scene) {
        assert_eq!(*s.seen.borrow(), vec![WORLD.to_vec()]);
    }

    fn check_profile(s: &Scene) {
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_SPHERE_CAPSULE, LABEL_END]
        );
    }

    /// The unit vector the exe makes of `raw`.
    fn unit(e: &Engine, raw: Vec4) -> Vec4 {
        let sum = raw[2] * raw[2] + (raw[1] * raw[1] + raw[0] * raw[0]);
        let scale = inverse_length(e, sum);
        raw.map(|lane| scale * lane)
    }

    /// The contact for a normal `raw` and a distance (capsule radius 1, radius sum 2).
    fn expected_contact(e: &Engine, raw: Vec4, distance: f32) -> [f32; 8] {
        let normal = unit(e, raw);
        let gap = 1.0 - distance;
        let mut contact = [0.0; 8];
        for lane in 0..4 {
            contact[lane] = gap * normal[lane] + CENTRE[lane];
            contact[4 + lane] = normal[lane];
        }
        contact[7] = distance - 2.0;
        contact
    }

    /// A collector double at slot 4 of a logging object; answers the object.
    fn collector(s: &mut Scene) -> u32 {
        logging_object(&mut s.e, &s.log, COLLECTOR)
    }

    #[test]
    fn cleanup_gives_the_id_back_and_deletes_00d3e2b0() {
        for (id, param) in [(7u16, 0x55u32), (0xffff, 0x66)] {
            let mut e = primed_engine();
            let log = new_log();
            let manager = logging_object(&mut e, &log, MANAGER);
            let this = logging_object(&mut e, &log, OBJECT);
            e.mem.set_u32(this + 8, manager);
            e.mem.set_u16(this + 0xc, id);
            e.call(0x00d3_e2b0, &args![this, param]);
            let taken = calls(&log, MANAGER + 0x14);
            if id == 0xffff {
                assert!(taken.is_empty());
            } else {
                assert_eq!(taken, vec![vec![manager, 7, 0x55]]);
            }
            assert_eq!(calls(&log, OBJECT), vec![vec![this, 1]]);
        }
    }

    /// Runs the penetration query `address` (`words` is its argument list for a scene) and
    /// checks the collector was told about the bodies exactly when `told`.
    fn check_penetration(address: u32, closest: Vec4, virtual_method: bool, told: bool) {
        let mut s = scene(closest, ENDS, 0.0);
        let collector = collector(&mut s);
        let words = if virtual_method {
            vec![0x4000_0000, s.a, s.b, 0xcace, collector]
        } else {
            vec![s.a, s.b, 0xcace, collector]
        };
        s.e.call(address, &words);
        check_world(&s);
        check_profile(&s);
        let slot = calls(&s.log, COLLECTOR + 4);
        if told {
            assert_eq!(slot, vec![vec![collector, s.a, s.b]]);
        } else {
            assert!(slot.is_empty());
        }
    }

    #[test]
    fn static_get_penetrations_tells_the_collector_of_touching_bodies_00d3e330() {
        check_penetration(0x00d3_e330, NEAR, false, true);
        check_penetration(0x00d3_e330, FAR, false, false);
        // Exactly touching (distance 2 against a radius sum of 2) is not penetrating.
        check_penetration(0x00d3_e330, [1.0, 2.0, 1.0, 1.0], false, false);
        // A NaN is not below the square.
        check_penetration(0x00d3_e330, [f32::NAN, 2.0, 1.5, 1.0], false, false);
    }

    #[test]
    fn get_penetrations_runs_the_static_version_00d3e510() {
        check_penetration(0x00d3_e510, NEAR, true, true);
        check_penetration(0x00d3_e510, FAR, true, false);
    }

    #[test]
    fn contact_is_written_for_a_close_capsule_00d3e530() {
        let mut s = scene(NEAR, ENDS, 0.0);
        let out = s.e.mem.alloc(0x20);
        s.e.mem.write(out, &[0xaa; 0x20]);
        let found = s.e.call(0x00d3_e530, &args![s.a, s.b, s.input, out]).u32();
        assert_eq!(found, 1);
        check_world(&s);
        let want = expected_contact(&s.e, [0.0, 0.0, 1.5, 0.0], 1.5);
        assert_eq!(floats(&s.e, out), want);
        // The normal points along +z and the separating distance is 1.5 - 2.
        assert!((want[6] - 1.0).abs() < 1e-5);
        assert_eq!(want[7], -0.5);
        // Too far apart: nothing is written.
        let mut s = scene(FAR, ENDS, 0.0);
        let out = s.e.mem.alloc(0x20);
        s.e.mem.write(out, &[0xaa; 0x20]);
        let found = s.e.call(0x00d3_e530, &args![s.a, s.b, s.input, out]).u32();
        assert_eq!(found, 0);
        assert_eq!(s.e.mem.bytes(out, 0x20), vec![0xaa; 0x20]);
    }

    #[test]
    fn the_tolerance_widens_the_reach_00d3e530() {
        // Distance 2.5: apart with tolerance 0, touching with tolerance 1.
        let closest = [1.0, 2.0, 0.5, 1.0];
        for (tolerance, found) in [(0.0f32, 0u32), (1.0, 1)] {
            let mut s = scene(closest, ENDS, tolerance);
            let out = s.e.mem.alloc(0x20);
            assert_eq!(
                s.e.call(0x00d3_e530, &args![s.a, s.b, s.input, out]).u32(),
                found
            );
        }
        // A NaN distance counts as touching (the compare is made the other way round).
        let mut s = scene([f32::NAN, 2.0, 1.5, 1.0], ENDS, 0.0);
        let out = s.e.mem.alloc(0x20);
        assert_eq!(
            s.e.call(0x00d3_e530, &args![s.a, s.b, s.input, out]).u32(),
            1
        );
    }

    #[test]
    fn a_centre_on_the_segment_gets_a_perpendicular_normal_00d3e530() {
        // The closest point is the centre: the normal is built from the capsule's axis
        // (end 1 minus end 0), here with each of the three choices of components.
        let cases: [(Vec4, Vec4); 3] = [
            // |x| <= |y|, x smaller than z is false: keeps z and y.
            ([2.0, 4.0, 3.0, 0.0], [0.0, 3.0, -4.0, 0.0]),
            // |x| > |y|, y larger than z: keeps x and y.
            ([3.0, 2.0, 1.0, 0.0], [2.0, -3.0, 0.0, 0.0]),
            // |x| <= |y|, x larger than z: the first and the negated second.
            ([5.0, 6.0, 4.0, 0.0], [-6.0, 5.0, 0.0, 0.0]),
        ];
        for (axis, raw) in cases {
            let ends = [[0.0; 4], axis];
            let mut s = scene(CENTRE, ends, 0.0);
            let out = s.e.mem.alloc(0x20);
            let found = s.e.call(0x00d3_e530, &args![s.a, s.b, s.input, out]).u32();
            assert_eq!(found, 1);
            assert_eq!(floats(&s.e, out), expected_contact(&s.e, raw, 0.0));
        }
    }

    /// Runs the closest point query `address` and checks the contact the collector got.
    fn check_closest_points(address: u32, virtual_method: bool) {
        for (closest, told) in [(NEAR, true), (FAR, false)] {
            let mut s = scene(closest, ENDS, 0.0);
            let collector = collector(&mut s);
            let got: Rc<RefCell<Vec<([f32; 8], u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
            let copy = Rc::clone(&got);
            s.e.register_double(COLLECTOR + 4, move |e, a| {
                let bodies = (e.mem.u32(a[1] + 0x20), e.mem.u32(a[1] + 0x24));
                copy.borrow_mut()
                    .push((floats(e, a[1]), bodies.0, bodies.1));
                Ret::default()
            });
            let words = if virtual_method {
                vec![0x4000_0000, s.a, s.b, s.input, collector]
            } else {
                vec![s.a, s.b, s.input, collector]
            };
            s.e.call(address, &words);
            check_world(&s);
            check_profile(&s);
            let want = expected_contact(&s.e, [0.0, 0.0, 1.5, 0.0], 1.5);
            if told {
                assert_eq!(*got.borrow(), vec![(want, s.a, s.b)]);
            } else {
                assert!(got.borrow().is_empty());
            }
        }
    }

    #[test]
    fn get_closest_points_hands_the_contact_to_the_collector_00d3e850() {
        check_closest_points(0x00d3_e850, true);
    }

    #[test]
    fn static_get_closest_points_hands_the_contact_to_the_collector_00d3ebe0() {
        check_closest_points(0x00d3_ebe0, false);
    }

    /// Runs the registration and returns what the registrar saw: `(dispatcher, entry, type a,
    /// type b)` and the entry's four words and two flag bytes.
    #[test]
    fn registration_adds_the_symmetric_then_the_direct_entry_00d3ef70() {
        let mut e = primed_engine();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[1] + 4 * n)));
            words.extend([0x10, 0x11].map(|at| u32::from(e.mem.u8(a[1] + at))));
            copy.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d3_ef70, &args![0xd15bu32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0][0], 0xd15b);
        assert_eq!(
            seen[0][2..],
            [
                5,
                1,
                0x00d3_e2f0,
                0x00d3_f040,
                0x00d3_f4c0,
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
                5,
                0x00d3_e270,
                0x00d3_e330,
                0x00d3_ebe0,
                0x00da_f8e0,
                0,
                0
            ]
        );
        assert_eq!(seen[0][1], seen[1][1]);
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

    /// Runs the symmetric penetration query `address` (the capsule first) on touching bodies.
    fn check_symmetric_penetration(address: u32, virtual_method: bool) {
        let mut s = scene(NEAR, ENDS, 0.0);
        let seen = flip_slot(&mut s, FLIP_BODY_COLLECTOR_VTABLE, false);
        let words = if virtual_method {
            vec![0x4000_0000, s.b, s.a, s.input, 0x55]
        } else {
            vec![s.b, s.a, s.input, 0x55]
        };
        s.e.call(address, &words);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[1..], [s.a, s.b]);
        assert_eq!(seen[0].1, vec![FLIP_BODY_COLLECTOR_VTABLE, 0, 0x55]);
        check_world(&s);
    }

    #[test]
    fn symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3f000() {
        check_symmetric_penetration(0x00d3_f000, true);
    }

    #[test]
    fn static_symmetric_get_penetrations_flips_the_bodies_and_the_collector_00d3f040() {
        check_symmetric_penetration(0x00d3_f040, false);
    }

    /// Runs the symmetric closest point query `address` (the capsule first).
    fn check_symmetric_closest_points(address: u32, virtual_method: bool) {
        let mut s = scene(NEAR, ENDS, 0.0);
        s.e.mem.set_u32(LARGE_FLOAT, SIGNALLING_NAN);
        let seen = flip_slot(&mut s, FLIP_COLLECTOR_VTABLE, true);
        let words = if virtual_method {
            vec![0x4000_0000, s.b, s.a, s.input, 0x55]
        } else {
            vec![s.b, s.a, s.input, 0x55]
        };
        s.e.call(address, &words);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].1, vec![FLIP_COLLECTOR_VTABLE, 0x7fe0_0001, 0x55]);
        let want = expected_contact(&s.e, [0.0, 0.0, 1.5, 0.0], 1.5);
        assert_eq!(seen[0].2, want);
        check_world(&s);
    }

    #[test]
    fn symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3f480() {
        check_symmetric_closest_points(0x00d3_f480, true);
    }

    #[test]
    fn static_symmetric_get_closest_points_flips_the_bodies_and_the_collector_00d3f4c0() {
        check_symmetric_closest_points(0x00d3_f4c0, false);
    }

    /// What a run of `processCollision` left behind.
    struct Processed {
        s: Scene,
        this: u32,
        output: u32,
        free: u32,
    }

    /// Runs `address` (`processCollision` `00d3f080`, or the symmetric `00d3f500` with the
    /// bodies in the other order) on a scene with the given closest point, contact point id and
    /// the id the contact manager makes; the manager also sets the output's time of impact to
    /// `new_time` (it starts at 0.5, with time of impact data to turn around).
    fn process(address: u32, closest: Vec4, id: u16, made: u32, new_time: f32) -> Processed {
        let mut s = scene(closest, ENDS, 0.0);
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
        let words = if address == 0x00d3_f500 {
            [this, s.b, s.a, s.input, output]
        } else {
            [this, s.a, s.b, s.input, output]
        };
        s.e.call(address, &words);
        check_world(&s);
        check_profile(&s);
        Processed {
            s,
            this,
            output,
            free,
        }
    }

    #[test]
    fn a_touching_contact_gets_an_id_and_is_added_00d3f080() {
        let p = process(0x00d3_f080, NEAR, 0xffff, 0x42, 0.5);
        let manager = p.s.e.mem.u32(p.this + 8);
        let created = calls(&p.s.log, MANAGER + 0xc);
        assert_eq!(created.len(), 1);
        assert_eq!(
            created[0],
            vec![manager, p.s.a, p.s.b, p.s.input, p.output, 0, p.free]
        );
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0x42);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 0x42);
        let want = expected_contact(&p.s.e, [0.0, 0.0, 1.5, 0.0], 1.5);
        assert_eq!(floats(&p.s.e, p.free), want);
    }

    #[test]
    fn a_contact_with_an_id_is_added_without_asking_00d3f080() {
        let p = process(0x00d3_f080, NEAR, 9, 0x42, 0.5);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 9);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 9);
    }

    #[test]
    fn a_contact_the_manager_gives_no_id_is_skipped_00d3f080() {
        let p = process(0x00d3_f080, NEAR, 0xffff, 0xffff, 0.5);
        assert_eq!(calls(&p.s.log, MANAGER + 0xc).len(), 1);
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
    }

    #[test]
    fn apart_bodies_give_their_id_back_00d3f080() {
        let p = process(0x00d3_f080, FAR, 7, 0x42, 0.5);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777]]
        );
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        // Without an id there is nothing to give back.
        let p = process(0x00d3_f080, FAR, 0xffff, 0x42, 0.5);
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
    }

    #[test]
    fn symmetric_process_collision_flips_the_new_contact_00d3f500() {
        let p = process(0x00d3_f500, NEAR, 0xffff, 0x42, 0.5);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        let want = expected_contact(&p.s.e, [0.0, 0.0, 1.5, 0.0], 1.5);
        let w = want[7];
        // Position += normal.w * normal, normal.xyz negated.
        let position: Vec4 = std::array::from_fn(|lane| w * want[4 + lane] + want[lane]);
        let normal = [-want[4], -want[5], -want[6], w];
        assert_eq!(load_vec(&p.s.e, p.free), position);
        assert_eq!(load_vec(&p.s.e, p.free + 0x10), normal);
        // The time did not change: the time of impact data stays.
        assert_eq!(load_vec(&p.s.e, p.output + 0x3020), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(p.s.e.mem.u8(p.output + 0x304a), 0x12);
    }

    #[test]
    fn symmetric_process_collision_turns_the_toi_data_when_the_time_changes_00d3f500() {
        let p = process(0x00d3_f500, NEAR, 0xffff, 0x42, 0.25);
        let (e, output) = (&p.s.e, p.output);
        assert_eq!(load_vec(e, output + 0x3020), [-1.0, -2.0, -3.0, 4.0]);
        assert_eq!(e.mem.u8(output + 0x304a), 0x21);
        assert_eq!(e.mem.u8(output + 0x3048), 2);
        assert_eq!(e.mem.u8(output + 0x3049), 1);
        let words: Vec<u16> = (0..4).map(|n| e.mem.u16(output + 0x3040 + 2 * n)).collect();
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
    fn fn_00d3e2f0_fills_the_block() {
        let (mut e, sizes) = allocator_engine();
        let block = e
            .call(0x00d3_e2f0, &args![1u32, 2u32, 3u32, 0x7777u32])
            .u32();
        assert_eq!(*sizes.borrow(), vec![0x10]);
        assert_eq!(e.mem.u16(block + 4), 0x10);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block + 8), 0x7777);
        assert_eq!(e.mem.u32(block), 0x010c_f120);
        assert_eq!(e.mem.u16(block + 0xc), 0xffff);
    }
}
