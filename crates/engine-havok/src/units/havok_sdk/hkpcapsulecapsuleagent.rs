//! `Havok SDK/hkpcapsulecapsuleagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpCapsuleCapsuleAgent` (Xbox PDB; vtable `010cef90`'s methods are here) is the
//! collision agent of two capsule shapes. Its base is the Xbox PDB's `hkpCollisionAgent`:
//! reference count `+6`, contact manager `+8`, and the three manifold contact point ids
//! (16-bit, `0xFFFF` = none) at `+0xC`. A body passed to the agent functions is an
//! `hkpCdBody` (shape `+0`, motion `+8`); a capsule shape holds its radius at `+0x10` and
//! its two end points (`vec4`) at `+0x20` and `+0x30`. A motion starts with a 4-vector
//! matrix (three rotation rows and the translation).
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) are
//! otherwise complete: none of them has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, zip_with, Vec4};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: the capsule-capsule label and the shared end label.
pub(crate) const LABEL_CAPSULES: u32 = 0x010c_efcc;
pub(crate) const LABEL_END: u32 = 0x010c_41f8;
/// `hkCollideCapsuleUtil` functions (cdecl). Manifold: `(ends A, radius A, ends B, radius B,
/// contacts)`; closest point: `(ends A, radius A, ends B, radius B, tolerance, out)`; both
/// answer 0 when they found contacts.
const MANIFOLD_CAPS_VS_CAPS: u32 = 0x00cd_58b0;
const CLOSEST_POINT_CAPS_VS_CAPS: u32 = 0x00cd_6260;
/// `hkLineSegmentUtil::closestLineSegLineSeg` (cdecl: start A, direction A, start B,
/// direction B, result); the result's `+0x20` is the squared distance.
const CLOSEST_LINE_SEG_LINE_SEG: u32 = 0x00d2_2e40;
/// `hkpCollisionDispatcher` agent registration (`00cfbbe0`, `this` = dispatcher, `RET 0xC`):
/// `(agent function entry, type a, type b)`.
const REGISTER_AGENT: u32 = 0x00cf_bbe0;

/// The four rows of the matrix at the start of a motion.
pub(crate) fn load_matrix(e: &Engine, motion: u32) -> [Vec4; 4] {
    [0, 0x10, 0x20, 0x30].map(|offset| load_vec(e, motion + offset))
}

/// Lane-exact `x * row0 + y * row1 + z * row2 + row3` the way the exe's SSE loop does it:
/// `((y * r1) + (x * r0)) + ((z * r2) + t)`, every lane of the rows (the point's own fourth
/// lane is not used).
pub(crate) fn transform_point(rows: &[Vec4; 4], point: Vec4) -> Vec4 {
    let by_y = zip_with([point[1]; 4], rows[1], |a, b| a * b);
    let by_x = zip_with([point[0]; 4], rows[0], |a, b| a * b);
    let by_z = zip_with([point[2]; 4], rows[2], |a, b| a * b);
    let moved = zip_with(by_z, rows[3], |a, b| a + b);
    let first = zip_with(by_y, by_x, |a, b| a + b);
    zip_with(first, moved, |a, b| a + b)
}

/// The two end points (`+0x20`, `+0x30`) of the capsule shape of `body`, moved to world space
/// by the body's motion matrix.
pub(crate) fn world_ends(e: &Engine, body: u32) -> [Vec4; 2] {
    let shape = e.mem.u32(body);
    let motion = e.mem.u32(body + 8);
    let rows = load_matrix(e, motion);
    [0x20, 0x30].map(|offset| transform_point(&rows, load_vec(e, shape + offset)))
}

/// The capsule radius of the shape of `body`, as the bits an x87 load and store leaves.
pub(crate) fn radius_bits(e: &Engine, body: u32) -> u32 {
    let shape = e.mem.u32(body);
    x87_quiet_bits(e.mem.u32(shape + 0x10))
}

/// Stores the two end points at `at` and `at + 0x10`.
pub(crate) fn store_ends(e: &mut Engine, at: u32, ends: [Vec4; 2]) {
    store_vec(e, at, ends[0]);
    store_vec(e, at + 0x10, ends[1]);
}

// Translated from 00d3b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleCapsuleAgent::cleanup` (Xbox PDB): gives every contact point id that is not
/// `0xFFFF` back to the contact manager (`[this + 8]`, slot `0x14`, with the id and `param`),
/// then runs the agent's deleting destructor (slot 0, argument 1).
pub fn hkp_capsule_capsule_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    cleanup_agent(e, this.addr(), param);
}

/// The cleanup of the capsule agents: see `hkp_capsule_capsule_agent_cleanup`.
pub(crate) fn cleanup_agent(e: &mut Engine, this: u32, param: u32) {
    for index in 0..3 {
        let id = e.mem.u16(this + 0xc + 2 * index);
        if id != 0xffff {
            let manager = e.mem.u32(this + 8);
            e.vcall(manager, 0x14, &args![u32::from(id), param]);
        }
    }
    if this != 0 {
        e.vcall(this, 0, &args![1u32]);
    }
}

// Translated from 00d3b420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleCapsuleAgent::staticGetPenetrations` (Xbox PDB): a cdecl function over the
/// bodies, a word it does not read and the collector. Profiled (`"TtCapsCaps"`). Moves both
/// capsules' end points to world space, finds the closest points of the two segments
/// (`00d22e40`: start A, direction `end - start` of A, start B, direction of B, result) and
/// tells the collector (slot 4, with the two bodies) when the squared distance (result
/// `+0x20`) is below the square of the radius sum (an x87 comparison: NaN is not below).
pub fn hkp_capsule_capsule_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_3: u32,
    collector: Ptr,
) {
    profile_record(e, LABEL_CAPSULES);
    let a = world_ends(e, body_a.addr());
    let b = world_ends(e, body_b.addr());
    let direction_a = zip_with(a[1], a[0], |end, start| end - start);
    let direction_b = zip_with(b[1], b[0], |end, start| end - start);
    let squared = e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame, a[0]);
        store_vec(e, frame + 0x10, direction_a);
        store_vec(e, frame + 0x20, b[0]);
        store_vec(e, frame + 0x30, direction_b);
        e.call(
            CLOSEST_LINE_SEG_LINE_SEG,
            &args![
                frame,
                frame + 0x10,
                frame + 0x20,
                frame + 0x30,
                frame + 0x40
            ],
        );
        e.mem.f32(frame + 0x60)
    });
    let shape_a = e.mem.u32(body_a.addr());
    let shape_b = e.mem.u32(body_b.addr());
    let radius_sum =
        (f64::from(e.mem.f32(shape_b + 0x10)) + f64::from(e.mem.f32(shape_a + 0x10))) as f32;
    if f64::from(squared) < f64::from(radius_sum) * f64::from(radius_sum) {
        e.vcall(collector.addr(), 4, &args![body_a, body_b]);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleCapsuleAgent::getPenetrations` (Xbox PDB): `this` is not read; passes the four
/// stack words (two bodies, input, collector) to the static version `00d3b420`.
pub fn hkp_capsule_capsule_agent_get_penetrations(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    collector: Ptr,
) {
    e.call(0x00d3_b420, &args![body_a, body_b, input, collector]);
}

// Translated from 00d3b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unreferenced fragment of the capsule-capsule `processCollision` (the map gives it no
/// name) that takes its inputs in registers: `eax` is the first body, `ecx` the input
/// (`hkpProcessCollisionInput`, its tolerance at `+8`), `esi` the contact array (three
/// 32-byte contacts), and the stack word is the second body. Stores the tolerance (an x87
/// load and store of the float) in the distance lane `+0x1C` of each of the three contacts,
/// moves both capsules to world space and returns the status of
/// `hkCollideCapsuleUtilManifoldCapsVsCaps` (`00cd58b0`, 0 when it made contacts) on the
/// end points, the radii and the contact array.
pub fn fn_00d3b6f0(e: &mut Engine, eax: Ptr, ecx: Ptr, esi: Ptr, body_b: Ptr) -> u32 {
    let tolerance = x87_quiet_bits(e.mem.u32(ecx.addr() + 8));
    for slot in 0..3 {
        e.mem.set_u32(esi.addr() + 0x1c + 0x20 * slot, tolerance);
    }
    let a = world_ends(e, eax.addr());
    let b = world_ends(e, body_b.addr());
    let radius_a = radius_bits(e, eax.addr());
    let radius_b = radius_bits(e, body_b.addr());
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        store_ends(e, frame, a);
        store_ends(e, frame + 0x20, b);
        e.call(
            MANIFOLD_CAPS_VS_CAPS,
            &args![frame, radius_a, frame + 0x20, radius_b, esi],
        )
        .u32()
    })
}

// Translated from 00d3b8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unreferenced fragment of the capsule-capsule `getClosestPoints` (the map gives it no
/// name) that takes its inputs in registers: `eax` is the first body and `esi` the second;
/// the stack words are the input (tolerance at `+8`) and the output contact. Moves both
/// capsules to world space and returns the status of `hkCollideCapsuleUtilClostestPoint
/// CapsVsCaps` (`00cd6260`, 0 when it found a point) on the end points, the radii, the
/// tolerance (an x87 load and store of the float) and the output.
pub fn fn_00d3b8e0(e: &mut Engine, eax: Ptr, esi: Ptr, input: Ptr, out: Ptr) -> u32 {
    closest_point_status(e, eax.addr(), esi.addr(), input.addr(), out.addr())
}

/// The shared body of the closest point queries: both capsules in world space and the call to
/// `00cd6260`, whose status is returned.
fn closest_point_status(e: &mut Engine, body_a: u32, body_b: u32, input: u32, out: u32) -> u32 {
    let a = world_ends(e, body_a);
    let b = world_ends(e, body_b);
    let radius_a = radius_bits(e, body_a);
    let radius_b = radius_bits(e, body_b);
    let tolerance = x87_quiet_bits(e.mem.u32(input + 8));
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        store_ends(e, frame, a);
        store_ends(e, frame + 0x20, b);
        e.call(
            CLOSEST_POINT_CAPS_VS_CAPS,
            &args![frame, radius_a, frame + 0x20, radius_b, tolerance, out],
        )
        .u32()
    })
}

/// The body of both closest point entry points: profiled (`"TtCapsCaps"`). The exe's frame
/// holds an `hkpCdPoint` at `ESP + 0x70`: the 32-byte contact, then the two bodies at
/// `ESP + 0x90` and `ESP + 0x94` (`+0x20`, `+0x24` of the point), stored before the end points
/// are moved (`00d3bb36`, `00d3bb40`; `00d3bdc6`, `00d3bdd0` in the static twin). The status of
/// `closest_point_status` into the contact; when it is 0 the point is handed to the collector
/// (slot 4, `LEA EAX,[ESP + 0x70]`), whose `addCdPoint` (`00cab970`) walks both bodies'
/// parent chains.
fn get_closest_points(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    profile_record(e, LABEL_CAPSULES);
    e.with_stack(0x28, |e, point| {
        let point = point.addr();
        e.mem.set_u32(point + 0x20, body_a.addr());
        e.mem.set_u32(point + 0x24, body_b.addr());
        let status = closest_point_status(e, body_a.addr(), body_b.addr(), input.addr(), point);
        if status == 0 {
            e.vcall(collector.addr(), 4, &args![point]);
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d3baf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleCapsuleAgent::getClosestPoints` (Xbox PDB): `this` is not read; the two bodies,
/// the input and the collector are the stack words. Profiled (`"TtCapsCaps"`). Moves both
/// capsules to world space, asks `hkCollideCapsuleUtilClostestPointCapsVsCaps` (`00cd6260`)
/// with the radii, the input's tolerance (`+8`) and the contact of an `hkpCdPoint` on the
/// stack (the bodies at `+0x20`, `+0x24`), and gives the point to the collector (slot 4) when
/// the status is 0.
pub fn hkp_capsule_capsule_agent_get_closest_points(
    e: &mut Engine,
    _unused_this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    get_closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d3bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static (cdecl) twin of `getClosestPoints` (the map gives it no name): the same code
/// as `00d3baf0` without a `this`; it is the closest point function of the dispatcher entry
/// that `00d3c010` registers.
pub fn fn_00d3bd80(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    get_closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d3c010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the capsule-capsule agent with a dispatcher (the map gives it no name): one
/// agent function entry of 5 words, built on the stack and handed to `00cfbbe0` (`this` =
/// the dispatcher) for the shape types (5, 5): `createFunc` `00d3b380`, `getPenetrations`
/// `00d3b420`, `getClosestPoints` `00d3bd80`, `linearCast` `00daf8e0`, both flag bytes 0.
/// The two bytes after the flags are not set by the exe (the callee copies the word they are
/// in); here they are zero.
pub fn fn_00d3c010(e: &mut Engine, dispatcher: Ptr) {
    e.with_stack(0x14, |e, entry| {
        let entry = entry.addr();
        for (index, word) in [0x00d3_b380u32, 0x00d3_b420, 0x00d3_bd80, 0x00da_f8e0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(entry + 4 * index as u32, word);
        }
        e.mem.set_u8(entry + 0x10, 0);
        e.mem.set_u8(entry + 0x11, 0);
        e.call(REGISTER_AGENT, &args![dispatcher, entry, 5u32, 5u32]);
    });
}

// Translated from 00d3c060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleCapsuleAgent::processCollision` (Xbox PDB): profiled (`"TtCapsCaps"`). Fills the
/// distance lane (`+0x1C`) of three 32-byte contacts with the input's tolerance (`+8`, an x87
/// load and store), moves the capsules to world space and asks
/// `hkCollideCapsuleUtilManifoldCapsVsCaps` (`00cd58b0`) for contacts. For each contact `i`
/// with its id at `this + 0xC + 2 * i`: when its distance is below the tolerance (ordered
/// compare) the contact is added to the output (`[output]` is the next free 0x30-byte slot:
/// the 32 bytes, then the id as a word at `+0x20`), first asking the contact manager
/// (`[this + 8]`, slot `0xC`, with the bodies, input, output, 0 and the contact) for an id
/// when there is none; otherwise an existing id is given back to the manager (slot `0x14`,
/// with the id and `[output + 4]`) and cleared.
pub fn hkp_capsule_capsule_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, LABEL_CAPSULES);
    let tolerance = x87_quiet_bits(e.mem.u32(input.addr() + 8));
    let a = world_ends(e, body_a.addr());
    let b = world_ends(e, body_b.addr());
    let radius_a = radius_bits(e, body_a.addr());
    let radius_b = radius_bits(e, body_b.addr());
    // Three contacts at `+0`, the end points of A at `+0x60` and of B at `+0x80`.
    e.with_stack(0xa0, |e, frame| {
        let contacts = frame.addr();
        for slot in 0..3 {
            e.mem.set_u32(contacts + 0x1c + 0x20 * slot, tolerance);
        }
        store_ends(e, contacts + 0x60, a);
        store_ends(e, contacts + 0x80, b);
        e.call(
            MANIFOLD_CAPS_VS_CAPS,
            &args![
                contacts + 0x60,
                radius_a,
                contacts + 0x80,
                radius_b,
                contacts
            ],
        );
        for slot in 0..3u32 {
            let contact = contacts + 0x20 * slot;
            let id_at = this.addr() + 0xc + 2 * slot;
            let distance = e.mem.f32(contact + 0x1c);
            let limit = e.mem.f32(input.addr() + 8);
            if limit > distance {
                if e.mem.u16(id_at) == 0xffff {
                    let manager = e.mem.u32(this.addr() + 8);
                    let id = e
                        .vcall(
                            manager,
                            0xc,
                            &args![body_a, body_b, input, output, 0u32, contact],
                        )
                        .u32() as u16;
                    e.mem.set_u16(id_at, id);
                    if id == 0xffff {
                        continue;
                    }
                }
                let free = e.mem.u32(output.addr());
                e.mem.set_u32(output.addr(), free + 0x30);
                for word in 0..8 {
                    let value = e.mem.u32(contact + 4 * word);
                    e.mem.set_u32(free + 4 * word, value);
                }
                let id = u32::from(e.mem.u16(id_at));
                e.mem.set_u32(free + 0x20, id);
            } else if e.mem.u16(id_at) != 0xffff {
                let manager = e.mem.u32(this.addr() + 8);
                let id = u32::from(e.mem.u16(id_at));
                let second = e.mem.u32(output.addr() + 4);
                e.vcall(manager, 0x14, &args![id, second]);
                e.mem.set_u16(id_at, 0xffff);
            }
        }
    });
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_b3d0, hkp_capsule_capsule_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_b420,
            hkp_capsule_capsule_agent_static_get_penetrations(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d3_b6d0,
            hkp_capsule_capsule_agent_get_penetrations(Ptr, Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00d3_b6f0, fn_00d3b6f0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d3_b8e0, fn_00d3b8e0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d3_baf0,
            hkp_capsule_capsule_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_bd80, fn_00d3bd80(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d3_c010, fn_00d3c010(Ptr)),
        entry!(
            0x00d3_c060,
            hkp_capsule_capsule_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

/// Bodies and doubles the tests of the capsule agents share.
#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::Log;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The vectors a double saw behind its pointer arguments.
    pub(crate) type Seen = Rc<RefCell<Vec<Vec<Vec4>>>>;

    /// The rotation of the tests: a quarter turn about z and a move by `(10, 20, 30)`.
    pub(crate) const TURN: [Vec4; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [10.0, 20.0, 30.0, 1.0],
    ];
    pub(crate) const IDENTITY: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    /// A body whose motion is `rows` and whose shape holds `radius` at `+0x10` and `points`
    /// from `+0x20` on (a capsule's two end points, a triangle's three vertices).
    pub(crate) fn shaped_body(
        e: &mut Engine,
        radius: f32,
        points: &[Vec4],
        rows: [Vec4; 4],
    ) -> u32 {
        let shape = e.mem.alloc(0x60);
        e.mem.set_f32(shape + 0x10, radius);
        for (n, point) in points.iter().enumerate() {
            store_vec(e, shape + 0x20 + 0x10 * n as u32, *point);
        }
        let motion = e.mem.alloc(0x40);
        for (row, value) in rows.into_iter().enumerate() {
            store_vec(e, motion + 0x10 * row as u32, value);
        }
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body, shape);
        e.mem.set_u32(body + 8, motion);
        body
    }

    /// The capsule body A of the tests (radius 1, `(1, 2, 3)` to `(1, 2, 4)`, turned): its
    /// end points in world space are `(8, 21, 33, 1)` and `(8, 21, 34, 1)`.
    pub(crate) fn capsule_a(e: &mut Engine) -> u32 {
        shaped_body(e, 1.0, &[[1.0, 2.0, 3.0, 0.0], [1.0, 2.0, 4.0, 0.0]], TURN)
    }

    /// The bodies of the capsule-capsule tests: A and B (radius 2, `(0.5, 0.5, 0.5)` to
    /// `(2.5, 0.5, 0.5)`, not moved).
    pub(crate) fn bodies(e: &mut Engine) -> (u32, u32) {
        let a = capsule_a(e);
        let b = shaped_body(
            e,
            2.0,
            &[[0.5, 0.5, 0.5, 0.0], [2.5, 0.5, 0.5, 0.0]],
            IDENTITY,
        );
        (a, b)
    }

    /// The end points of the bodies of `bodies` in world space.
    pub(crate) fn world() -> Vec<Vec4> {
        vec![
            [8.0, 21.0, 33.0, 1.0],
            [8.0, 21.0, 34.0, 1.0],
            [0.5, 0.5, 0.5, 1.0],
            [2.5, 0.5, 0.5, 1.0],
        ]
    }

    /// A double at `address` that logs its arguments, notes the vectors behind the pointers
    /// at the argument positions `pointers` (`count` vectors each), runs `after` on the
    /// arguments and answers `eax`.
    pub(crate) fn vector_double(
        e: &mut Engine,
        log: &Log,
        address: u32,
        pointers: &'static [(usize, u32)],
        eax: u32,
        after: impl Fn(&mut Engine, &[u32]) + 'static,
    ) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        let log = Rc::clone(log);
        e.register_double(address, move |e, a| {
            log.borrow_mut().push((address, a.to_vec()));
            let vectors = pointers
                .iter()
                .flat_map(|&(index, count)| (0..count).map(move |n| (index, n)))
                .map(|(index, n)| load_vec(e, a[index] + 0x10 * n))
                .collect();
            copy.borrow_mut().push(vectors);
            after(e, a);
            Ret {
                eax,
                ..Ret::default()
            }
        });
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const COLLECTOR: u32 = 0x0910_0000;
    const OBJECT: u32 = 0x0920_0000;
    const SIGNALLING_NAN: u32 = 0x7fa0_0001;
    #[test]
    fn cleanup_gives_back_the_ids_in_use_00d3b3d0() {
        let mut e = engine();
        let log = new_log();
        let manager = logging_object(&mut e, &log, MANAGER);
        let this = logging_object(&mut e, &log, OBJECT);
        e.mem.set_u32(this + 8, manager);
        for (n, id) in [5u16, 0xffff, 7].into_iter().enumerate() {
            e.mem.set_u16(this + 0xc + 2 * n as u32, id);
        }
        e.call(0x00d3_b3d0, &[this, 0x99]);
        assert_eq!(
            calls(&log, MANAGER + 0x14),
            vec![vec![manager, 5, 0x99], vec![manager, 7, 0x99]]
        );
        assert_eq!(calls(&log, OBJECT), vec![vec![this, 1]]);
    }

    #[test]
    fn cleanup_without_ids_only_destroys_00d3b3d0() {
        let mut e = engine();
        let log = new_log();
        let manager = logging_object(&mut e, &log, MANAGER);
        let this = logging_object(&mut e, &log, OBJECT);
        e.mem.set_u32(this + 8, manager);
        for n in 0..3 {
            e.mem.set_u16(this + 0xc + 2 * n, 0xffff);
        }
        e.call(0x00d3_b3d0, &[this, 0x99]);
        assert!(calls(&log, MANAGER + 0x14).is_empty());
        assert_eq!(calls(&log, OBJECT).len(), 1);
    }

    #[test]
    fn penetration_is_reported_below_the_square_of_the_radius_sum_00d3b420() {
        // The radii are 1 and 2: the squared sum is 9.
        let cases: [(f32, bool); 5] = [
            (8.9, true),
            (0.0, true),
            (9.0, false),
            (9.5, false),
            (f32::NAN, false),
        ];
        for (squared, reported) in cases {
            let mut e = engine();
            let log = new_log();
            let (stream, records) = monitor(&mut e, 0x100);
            let (a, b) = bodies(&mut e);
            let collector = logging_object(&mut e, &log, COLLECTOR);
            let seen = vector_double(
                &mut e,
                &log,
                CLOSEST_LINE_SEG_LINE_SEG,
                &[(0, 1), (1, 1), (2, 1), (3, 1)],
                0,
                move |e, a| e.mem.set_f32(a[4] + 0x20, squared),
            );
            e.call(0x00d3_b420, &[a, b, 0x1234, collector]);
            // Start A, direction A, start B, direction B.
            assert_eq!(
                *seen.borrow(),
                vec![vec![
                    [8.0, 21.0, 33.0, 1.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.5, 0.5, 0.5, 1.0],
                    [2.0, 0.0, 0.0, 0.0],
                ]]
            );
            let told = calls(&log, COLLECTOR + 4);
            if reported {
                assert_eq!(told, vec![vec![collector, a, b]], "squared {squared}");
            } else {
                assert!(told.is_empty(), "squared {squared}");
            }
            assert_eq!(
                profile_labels(&e, stream, records),
                vec![LABEL_CAPSULES, LABEL_END]
            );
        }
    }

    #[test]
    fn get_penetrations_passes_the_words_to_the_static_version_00d3b6d0() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00d3_b420, 0);
        e.call(0x00d3_b6d0, &[0x1111, 0xa, 0xb, 0xc, 0xd]);
        assert_eq!(calls(&log, 0x00d3_b420), vec![vec![0xa, 0xb, 0xc, 0xd]]);
    }

    #[test]
    fn manifold_fragment_stores_the_tolerance_and_asks_for_contacts_00d3b6f0() {
        let mut e = engine();
        let log = new_log();
        let (a, b) = bodies(&mut e);
        // A signalling NaN radius is quieted by the x87 load and store.
        let shape_a = e.mem.u32(a);
        e.mem.set_u32(shape_a + 0x10, SIGNALLING_NAN);
        let seen = vector_double(
            &mut e,
            &log,
            MANIFOLD_CAPS_VS_CAPS,
            &[(0, 2), (2, 2)],
            0x77,
            |_, _| {},
        );
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input + 8, SIGNALLING_NAN);
        let contacts = e.mem.alloc(0x60);
        let result = e.call(0x00d3_b6f0, &[a, input, contacts, b]);
        assert_eq!(result.u32(), 0x77);
        for slot in 0..3 {
            assert_eq!(e.mem.u32(contacts + 0x1c + 0x20 * slot), 0x7fe0_0001);
        }
        assert_eq!(*seen.borrow(), vec![world()]);
        let call = &calls(&log, MANIFOLD_CAPS_VS_CAPS)[0];
        assert_eq!(call[1], 0x7fe0_0001);
        assert_eq!(call[3], 2.0f32.to_bits());
        assert_eq!(call[4], contacts);
    }

    #[test]
    fn closest_point_fragment_passes_the_tolerance_and_output_00d3b8e0() {
        let mut e = engine();
        let log = new_log();
        let (a, b) = bodies(&mut e);
        let seen = vector_double(
            &mut e,
            &log,
            CLOSEST_POINT_CAPS_VS_CAPS,
            &[(0, 2), (2, 2)],
            3,
            |_, _| {},
        );
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 0.25);
        let out = e.mem.alloc(0x20);
        let result = e.call(0x00d3_b8e0, &[a, b, input, out]);
        assert_eq!(result.u32(), 3);
        assert_eq!(*seen.borrow(), vec![world()]);
        let call = &calls(&log, CLOSEST_POINT_CAPS_VS_CAPS)[0];
        assert_eq!(call[1], 1.0f32.to_bits());
        assert_eq!(call[3], 2.0f32.to_bits());
        assert_eq!(call[4], 0.25f32.to_bits());
        assert_eq!(call[5], out);
    }

    /// Runs a closest point query (`address` has a `this` word first when `virtual_method`)
    /// whose utility answers `status`; checks the contact given to the collector.
    fn closest_points(address: u32, virtual_method: bool, status: u32) {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let (a, b) = bodies(&mut e);
        let collector = logging_object(&mut e, &log, COLLECTOR);
        let seen = vector_double(
            &mut e,
            &log,
            CLOSEST_POINT_CAPS_VS_CAPS,
            &[(0, 2), (2, 2)],
            status,
            |_, _| {},
        );
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 0.5);
        let words: Vec<u32> = if virtual_method {
            vec![0x1111, a, b, input, collector]
        } else {
            vec![a, b, input, collector]
        };
        e.call(address, &words);
        assert_eq!(*seen.borrow(), vec![world()]);
        let call = &calls(&log, CLOSEST_POINT_CAPS_VS_CAPS)[0];
        assert_eq!(call[4], 0.5f32.to_bits());
        let told = calls(&log, COLLECTOR + 4);
        if status == 0 {
            assert_eq!(told, vec![vec![collector, call[5]]]);
        } else {
            assert!(told.is_empty());
        }
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_CAPSULES, LABEL_END]
        );
    }

    #[test]
    fn closest_points_tell_the_collector_when_found_00d3baf0() {
        closest_points(0x00d3_baf0, true, 0);
        closest_points(0x00d3_baf0, true, 1);
    }

    #[test]
    fn static_closest_points_tell_the_collector_when_found_00d3bd80() {
        closest_points(0x00d3_bd80, false, 0);
        closest_points(0x00d3_bd80, false, 0xffff_ffff);
    }

    /// Runs a closest point query (`address` has a `this` word first when `virtual_method`)
    /// whose utility finds a point, and checks what the collector reads behind the point it
    /// is given: the utility's 32-byte contact, then the two bodies at `+0x20` and `+0x24`
    /// (the exe's `hkpCdPoint` at `ESP + 0x70`, bodies at `ESP + 0x90` and `ESP + 0x94`; the
    /// collector's `addCdPoint`, `00cab970`, walks their parent chains).
    fn check_collected_point(address: u32, virtual_method: bool) {
        let mut e = engine();
        let log = new_log();
        monitor(&mut e, 0x100);
        let (a, b) = bodies(&mut e);
        let collector = logging_object(&mut e, &log, COLLECTOR);
        // The utility writes a recognisable contact into its output and reports it found.
        let outputs: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let written = Rc::clone(&outputs);
        e.register_double(CLOSEST_POINT_CAPS_VS_CAPS, move |e, w| {
            written.borrow_mut().push(w[5]);
            for word in 0..8 {
                e.mem.set_u32(w[5] + 4 * word, 0xc000 + word);
            }
            Ret::default()
        });
        let got: Rc<RefCell<Vec<(u32, Vec<u32>, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&got);
        e.register_double(COLLECTOR + 4, move |e, w| {
            let point = w[1];
            let contact = (0..8).map(|word| e.mem.u32(point + 4 * word)).collect();
            let body_a = e.mem.u32(point + 0x20);
            let body_b = e.mem.u32(point + 0x24);
            copy.borrow_mut().push((point, contact, body_a, body_b));
            Ret::default()
        });
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, 0.5);
        let words: Vec<u32> = if virtual_method {
            vec![0x1111, a, b, input, collector]
        } else {
            vec![a, b, input, collector]
        };
        e.call(address, &words);
        assert_eq!(outputs.borrow().len(), 1);
        let output = outputs.borrow()[0];
        let contact: Vec<u32> = (0..8).map(|word| 0xc000 + word).collect();
        assert_eq!(*got.borrow(), vec![(output, contact, a, b)]);
    }

    #[test]
    fn closest_points_give_the_collector_the_bodies_after_the_contact_00d3baf0() {
        check_collected_point(0x00d3_baf0, true);
    }

    #[test]
    fn static_closest_points_give_the_collector_the_bodies_after_the_contact_00d3bd80() {
        check_collected_point(0x00d3_bd80, false);
    }

    #[test]
    fn registration_builds_the_agent_entry_for_capsule_pairs_00d3c010() {
        let mut e = engine();
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let copy = Rc::clone(&seen);
        e.register_double(REGISTER_AGENT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[1] + 4 * n)));
            words.extend([0x10, 0x11].map(|at| u32::from(e.mem.u8(a[1] + at))));
            copy.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00d3_c010, &[0xd15b]);
        assert_eq!(seen.borrow().len(), 1);
        let words = &seen.borrow()[0];
        assert_eq!(words[0], 0xd15b);
        assert_eq!(words[2..4], [5, 5]);
        assert_eq!(
            words[4..],
            [0x00d3_b380, 0x00d3_b420, 0x00d3_bd80, 0x00da_f8e0, 0, 0]
        );
    }

    /// The contacts the utility double makes: distances `0.5`, `2.0` and NaN, and recognisable
    /// words in the rest. Notes what the tolerance slots held before.
    fn make_contacts(e: &mut Engine, a: &[u32]) {
        let before: Vec<u32> = (0..3).map(|n| e.mem.u32(a[4] + 0x1c + 0x20 * n)).collect();
        assert_eq!(before, vec![1.0f32.to_bits(); 3]);
        for (n, distance) in [0.5f32, 2.0, f32::NAN].into_iter().enumerate() {
            let contact = a[4] + 0x20 * n as u32;
            for word in 0..7 {
                e.mem
                    .set_u32(contact + 4 * word, 0x1000 * (n as u32 + 1) + word);
            }
            e.mem.set_f32(contact + 0x1c, distance);
        }
    }

    /// What a run of `processCollision` left behind.
    struct Processed {
        e: Engine,
        log: Log,
        this: u32,
        output: u32,
        free: u32,
        contacts: u32,
        bodies: (u32, u32),
    }

    /// Runs `processCollision` with the given ids and the id the manager makes.
    fn process(ids: [u16; 3], made: u32) -> Processed {
        let mut e = engine();
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let (a, b) = bodies(&mut e);
        let manager = logging_object(&mut e, &log, MANAGER);
        log_double(&mut e, &log, MANAGER + 0xc, made);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, manager);
        for (n, id) in ids.into_iter().enumerate() {
            e.mem.set_u16(this + 0xc + 2 * n as u32, id);
        }
        let seen = vector_double(
            &mut e,
            &log,
            MANIFOLD_CAPS_VS_CAPS,
            &[(0, 2), (2, 2)],
            0,
            make_contacts,
        );
        let input = e.mem.alloc(0x100);
        e.mem.set_f32(input + 8, 1.0);
        let free = e.mem.alloc(0x100);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output, free);
        e.mem.set_u32(output + 4, 0x7777);
        e.call(0x00d3_c060, &[this, a, b, input, output]);
        assert_eq!(*seen.borrow(), vec![world()]);
        assert_eq!(
            profile_labels(&e, stream, records),
            vec![LABEL_CAPSULES, LABEL_END]
        );
        let contacts = calls(&log, MANIFOLD_CAPS_VS_CAPS)[0][4];
        Processed {
            e,
            log,
            this,
            output,
            free,
            contacts,
            bodies: (a, b),
        }
    }

    fn ids(e: &Engine, this: u32) -> Vec<u16> {
        (0..3).map(|n| e.mem.u16(this + 0xc + 2 * n)).collect()
    }

    #[test]
    fn a_new_close_contact_gets_an_id_and_is_added_00d3c060() {
        let Processed {
            e,
            log,
            this,
            output,
            free,
            contacts,
            bodies: (a, b),
        } = process([0xffff, 7, 0xffff], 0x42);
        let manager = e.mem.u32(this + 8);
        let created = calls(&log, MANAGER + 0xc);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..3], [manager, a, b]);
        assert_eq!(created[0][4..], [output, 0, contacts]);
        // The second contact is too far: its id is given back with `[output + 4]`.
        assert_eq!(calls(&log, MANAGER + 0x14), vec![vec![manager, 7, 0x7777]]);
        assert_eq!(ids(&e, this), vec![0x42, 0xffff, 0xffff]);
        assert_eq!(e.mem.u32(output), free + 0x30);
        for word in 0..7 {
            assert_eq!(e.mem.u32(free + 4 * word), 0x1000 + word);
        }
        assert_eq!(e.mem.f32(free + 0x1c), 0.5);
        assert_eq!(e.mem.u32(free + 0x20), 0x42);
    }

    #[test]
    fn contacts_with_ids_are_added_without_asking_and_far_ones_are_released_00d3c060() {
        let Processed {
            e,
            log,
            this,
            output,
            free,
            ..
        } = process([3, 7, 9], 0x42);
        let manager = e.mem.u32(this + 8);
        assert!(calls(&log, MANAGER + 0xc).is_empty());
        // Too far (2.0) and unordered (NaN) contacts give their ids back.
        assert_eq!(
            calls(&log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777], vec![manager, 9, 0x7777]]
        );
        assert_eq!(ids(&e, this), vec![3, 0xffff, 0xffff]);
        assert_eq!(e.mem.u32(output), free + 0x30);
        assert_eq!(e.mem.u32(free + 0x20), 3);
    }

    #[test]
    fn a_contact_the_manager_gives_no_id_is_skipped_00d3c060() {
        let Processed {
            e,
            log,
            this,
            output,
            free,
            ..
        } = process([0xffff, 0xffff, 0xffff], 0xffff);
        assert_eq!(calls(&log, MANAGER + 0xc).len(), 1);
        assert!(calls(&log, MANAGER + 0x14).is_empty());
        assert_eq!(ids(&e, this), vec![0xffff; 3]);
        assert_eq!(e.mem.u32(output), free);
        assert_eq!(e.mem.u32(free + 0x20), 0);
    }
}
