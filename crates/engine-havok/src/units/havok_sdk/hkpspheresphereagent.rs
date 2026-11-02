//! `Havok SDK/hkpspheresphereagent.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkpSphereSphereAgent` is the collision agent of two spheres. Its base is the Xbox PDB's
//! `hkpCollisionAgent`: contact manager `+8` and the one manifold contact point id (16-bit,
//! `0xFFFF` = none) at `+0xC`. A body is an `hkpCdBody` (shape `+0`, motion `+8`): a sphere's
//! centre is the translation row at `motion + 0x30` and its radius sits at `shape + 0x10`.
//! The bodies touch when the squared distance of the centres is below the square of the reach
//! (the radius sum, plus the tolerance at `input + 8` for the contact queries; the reach is
//! rounded to a `float` first, its square is not). The contact of `processCollision` and of
//! `getClosestPoints` has the normal `(A - B) / distance` (the constant at `01268370` when
//! the centres coincide), the position `centre of B + radius of B * normal` and, in the normal's
//! fourth lane, the separating distance. The x87 does the square root (`00ec6040`), the
//! division and the final sum of the distance and the radii.
//!
//! The functions that start a profiling record (`RDTSC` time stamps from the host clock) have
//! no /GS cookie or exception frame. The two `getClosestPoints` read one uninitialised stack
//! slot (see their marks).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{add, mul};
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec, sum3, zip_with, Vec4};
use crate::units::havok_sdk::run_00cf5070::profile_record;

/// Profiling labels: `"TtSphereSphere"` and the shared end label.
const LABEL_SPHERE_SPHERE: u32 = 0x010c_f1a4;
const LABEL_END: u32 = 0x010c_41f8;
/// The CRT square root (`00ec6040`, argument in ST0, result in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// The vector (the first row of the identity `hkMatrix3`) that is the normal of two spheres
/// with the same centre.
const COINCIDENT_NORMAL: u32 = 0x0126_8370;

/// What the two radii and the centres give: `(A - B)`, the radii and the centre of B.
struct Spheres {
    /// The sphere radius of A and B (the `float`s at `shape + 0x10`, taken through the x87).
    radius_a: f32,
    radius_b: f32,
    /// The centre of A minus the centre of B, and the centre of B.
    difference: Vec4,
    centre_b: Vec4,
}

/// Reads the radii and centres of the two bodies.
fn spheres(e: &Engine, body_a: u32, body_b: u32) -> Spheres {
    let radius_a = x87_quiet(e.mem.f32(e.mem.u32(body_a) + 0x10));
    let radius_b = x87_quiet(e.mem.f32(e.mem.u32(body_b) + 0x10));
    let centre_a = load_vec(e, e.mem.u32(body_a + 8) + 0x30);
    let centre_b = load_vec(e, e.mem.u32(body_b + 8) + 0x30);
    Spheres {
        radius_a,
        radius_b,
        difference: zip_with(centre_a, centre_b, |a, b| a - b),
        centre_b,
    }
}

/// `1 / sqrt(distance squared)` the way the exe makes it for a positive squared distance: the
/// CRT's `sqrt` rounded to a `float`, divided into one on the x87 and rounded again.
fn inverse_distance(e: &mut Engine, distance_squared: f32) -> f32 {
    let root = e.call(X87_SQRT, &args![f64::from(distance_squared)]).f64() as f32;
    (1.0 / f64::from(root)) as f32
}

/// The contact the sphere pair leaves: the position and the normal (fourth lane: the
/// separating distance).
struct Contact {
    position: Vec4,
    normal: Vec4,
}

/// The contact of `processCollision` (`00d3f780`) for squared distance `distance_squared` of
/// the centres (above zero: the normal is the difference times the inverse distance and the
/// separating distance the product of the squared distance and the inverse distance, less the
/// radius sum, all in `f64` until the store; else the normal is the constant at `01268370`
/// and the separating distance the negated radius sum).
fn process_contact(e: &mut Engine, s: &Spheres, distance_squared: f32) -> Contact {
    let radius_sum = f64::from(s.radius_a) + f64::from(s.radius_b);
    let (mut normal, separation) = if distance_squared > 0.0 {
        let inverse = inverse_distance(e, distance_squared);
        (
            mul([inverse; 4], s.difference),
            (f64::from(distance_squared) * f64::from(inverse) - radius_sum) as f32,
        )
    } else {
        (load_vec(e, COINCIDENT_NORMAL), (-radius_sum) as f32)
    };
    let position = add(mul([s.radius_b; 4], normal), s.centre_b);
    normal[3] = separation;
    Contact { position, normal }
}

// Translated from 00d3f660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereSphereAgent::cleanup` (Xbox PDB): gives the contact point id (`+0xC`) back to the
/// contact manager (`[this + 8]`, slot `0x14`, with the id and `param`) when it is not
/// `0xFFFF`, then runs the agent's deleting destructor (slot 0, argument 1).
pub fn hkp_sphere_sphere_agent_cleanup(e: &mut Engine, this: Ptr, param: u32) {
    let id = e.mem.u16(this.addr() + 0xc);
    if id != 0xffff {
        let manager = e.mem.u32(this.addr() + 8);
        e.vcall(manager, 0x14, &args![u32::from(id), param]);
    }
    e.vcall(this.addr(), 0, &args![1u32]);
}

// Translated from 00d3f6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereSphereAgent::staticGetPenetrations` (Xbox PDB): a cdecl function over the bodies,
/// a word it does not read and the collector. Profiled (`"TtSphereSphere"`). Tells the
/// collector (slot 4, with the two bodies) when the squared distance of the centres is below
/// the square of the radius sum (rounded to a `float`; an x87 compare, so a NaN is not below).
pub fn hkp_sphere_sphere_agent_static_get_penetrations(
    e: &mut Engine,
    body_a: Ptr,
    body_b: Ptr,
    _unused_3: u32,
    collector: Ptr,
) {
    profile_record(e, LABEL_SPHERE_SPHERE);
    let s = spheres(e, body_a.addr(), body_b.addr());
    let radius_sum = (f64::from(s.radius_a) + f64::from(s.radius_b)) as f32;
    let distance_squared = sum3(mul(s.difference, s.difference));
    let reach_squared = f64::from(radius_sum) * f64::from(radius_sum);
    if f64::from(distance_squared) < reach_squared {
        e.vcall(collector.addr(), 4, &args![body_a, body_b]);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3fcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereSphereAgent::getPenetrations` (Xbox PDB): runs `staticGetPenetrations` on the
/// four words (`this` is not used).
pub fn hkp_sphere_sphere_agent_get_penetrations(
    e: &mut Engine,
    _this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: u32,
    collector: Ptr,
) {
    hkp_sphere_sphere_agent_static_get_penetrations(e, body_a, body_b, input, collector);
}

// Translated from 00d3f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSphereSphereAgent::processCollision` (Xbox PDB): profiled. The record to fill is the
/// output's first free contact point (`[output]`, 0x30 bytes). When the spheres touch (see
/// the unit's description: the reach includes the tolerance) the contact is written there;
/// then, when the agent has no contact point id (`+0xC` is `0xFFFF`), the contact manager
/// (`[this + 8]`) is asked for one through its slot `0xC` (with the two bodies, the input, the
/// output, 0 and the record; its answer is the 16-bit id). With an id, the record's `+0x20`
/// holds it and `[output]` moves on by `0x30`. When the bodies are apart and an id is held,
/// the manager's slot `0x14` takes it back (with `[output + 4]`) and the id is cleared.
pub fn hkp_sphere_sphere_agent_process_collision(
    e: &mut Engine,
    this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    output: Ptr,
) {
    profile_record(e, LABEL_SPHERE_SPHERE);
    let id_at = this.addr() + 0xc;
    let s = spheres(e, body_a.addr(), body_b.addr());
    let reach = (f64::from(s.radius_a)
        + f64::from(s.radius_b)
        + f64::from(e.mem.f32(input.addr() + 8))) as f32;
    let distance_squared = sum3(mul(s.difference, s.difference));
    let reach_squared = f64::from(reach) * f64::from(reach);
    if f64::from(distance_squared) < reach_squared {
        let record = e.mem.u32(output.addr());
        let found = process_contact(e, &s, distance_squared);
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
    } else {
        let id = e.mem.u16(id_at);
        if id != 0xffff {
            let manager = e.mem.u32(this.addr() + 8);
            let second = e.mem.u32(output.addr() + 4);
            e.vcall(manager, 0x14, &args![u32::from(id), second]);
            e.mem.set_u16(id_at, 0xffff);
        }
    }
    profile_record(e, LABEL_END);
}

/// The body of `getClosestPoints` (the virtual `00d3f980` and the cdecl `00d3fb20` are the
/// same instructions): profiled; when the spheres touch builds an `hkpCdPoint` on the stack
/// (the position, the normal with the separating distance in its fourth lane, the bodies at
/// `+0x20` and `+0x24`) and passes it to the collector's slot 4. Unlike `processCollision`
/// the product of the squared distance and the inverse distance is stored as a `float` before
/// the radii are taken off, and for coincident centres the normal's fourth lane is not the
/// constant's but whatever the stack slot (`point + 0x1C`) held: it goes into the position's
/// fourth lane (here the slot of the zeroed block).
fn closest_points(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    profile_record(e, LABEL_SPHERE_SPHERE);
    let s = spheres(e, body_a.addr(), body_b.addr());
    let reach = (f64::from(s.radius_a)
        + f64::from(s.radius_b)
        + f64::from(e.mem.f32(input.addr() + 8))) as f32;
    let distance_squared = sum3(mul(s.difference, s.difference));
    let reach_squared = f64::from(reach) * f64::from(reach);
    if f64::from(distance_squared) < reach_squared {
        e.with_stack(0x28, |e, point| {
            let point = point.addr();
            let radius_sum = f64::from(s.radius_b) + f64::from(s.radius_a);
            let (mut normal, projected) = if distance_squared > 0.0 {
                let inverse = inverse_distance(e, distance_squared);
                (
                    mul([inverse; 4], s.difference),
                    (f64::from(distance_squared) * f64::from(inverse)) as f32,
                )
            } else {
                let constant = load_vec(e, COINCIDENT_NORMAL);
                let stale = f32::from_bits(e.mem.u32(point + 0x1c));
                ([constant[0], constant[1], constant[2], stale], 0.0)
            };
            let position = add(mul([s.radius_b; 4], normal), s.centre_b);
            normal[3] = (f64::from(projected) - radius_sum) as f32;
            store_vec(e, point, position);
            store_vec(e, point + 0x10, normal);
            e.mem.set_u32(point + 0x20, body_a.addr());
            e.mem.set_u32(point + 0x24, body_b.addr());
            e.vcall(collector.addr(), 4, &args![point]);
        });
    }
    profile_record(e, LABEL_END);
}

// Translated from 00d3f980 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d3f980 - for coincident centres the position's fourth lane comes from an uninitialised stack slot (modelled as the zeroed slot).
/// `hkpSphereSphereAgent::getClosestPoints` (Xbox PDB): see `closest_points` (`this` is not
/// used).
pub fn hkp_sphere_sphere_agent_get_closest_points(
    e: &mut Engine,
    _this: Ptr,
    body_a: Ptr,
    body_b: Ptr,
    input: Ptr,
    collector: Ptr,
) {
    closest_points(e, body_a, body_b, input, collector);
}

// Translated from 00d3fb20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d3fb20 - for coincident centres the position's fourth lane comes from an uninitialised stack slot (modelled as the zeroed slot).
/// The cdecl `getClosestPoints` of the sphere-sphere pair (the map gives it no name): the same
/// instructions as `00d3f980`, without `this`.
pub fn fn_00d3fb20(e: &mut Engine, body_a: Ptr, body_b: Ptr, input: Ptr, collector: Ptr) {
    closest_points(e, body_a, body_b, input, collector);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d3_f660, hkp_sphere_sphere_agent_cleanup(Ptr, u32)),
        entry!(
            0x00d3_f6a0,
            hkp_sphere_sphere_agent_static_get_penetrations(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d3_f780,
            hkp_sphere_sphere_agent_process_collision(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d3_f980,
            hkp_sphere_sphere_agent_get_closest_points(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d3_fb20, fn_00d3fb20(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d3_fcc0,
            hkp_sphere_sphere_agent_get_penetrations(Ptr, Ptr, Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, log_double, logging_object, monitor, new_log, profile_labels, Log,
    };
    use crate::units::havok_sdk::hkpspherecapsuleagent::fixtures::{
        floats, primed_engine, sphere_body,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    const MANAGER: u32 = 0x0900_0000;
    const COLLECTOR: u32 = 0x0910_0000;
    const OBJECT: u32 = 0x0920_0000;
    /// What the constant at `01268370` holds in the tests.
    const CONSTANT: Vec4 = [0.25, 0.5, 0.75, 7.0];

    /// The contacts the closest point collector got: the eight floats and the two bodies.
    type Points = Rc<RefCell<Vec<([f32; 8], u32, u32)>>>;

    /// Sphere A (radius 1) at `(0, 0, z)` and sphere B (radius 2) at the origin, with an input
    /// holding `tolerance` at `+8` and the thread's profile stream.
    struct Scene {
        e: Engine,
        log: Log,
        a: u32,
        b: u32,
        input: u32,
        stream: u32,
        records: u32,
    }

    fn scene(z: f32, tolerance: f32) -> Scene {
        let mut e = primed_engine();
        store_vec(&mut e, COINCIDENT_NORMAL, CONSTANT);
        let log = new_log();
        let (stream, records) = monitor(&mut e, 0x100);
        let a = sphere_body(&mut e, 1.0, [0.0, 0.0, z]);
        let b = sphere_body(&mut e, 2.0, [0.0, 0.0, 0.0]);
        let input = e.mem.alloc(0x20);
        e.mem.set_f32(input + 8, tolerance);
        Scene {
            e,
            log,
            a,
            b,
            input,
            stream,
            records,
        }
    }

    fn check_profile(s: &Scene) {
        assert_eq!(
            profile_labels(&s.e, s.stream, s.records),
            vec![LABEL_SPHERE_SPHERE, LABEL_END]
        );
    }

    #[test]
    fn cleanup_gives_the_id_back_and_deletes_00d3f660() {
        for (id, param) in [(7u16, 0x55u32), (0xffff, 0x66)] {
            let mut e = primed_engine();
            let log = new_log();
            let manager = logging_object(&mut e, &log, MANAGER);
            let this = logging_object(&mut e, &log, OBJECT);
            e.mem.set_u32(this + 8, manager);
            e.mem.set_u16(this + 0xc, id);
            e.call(0x00d3_f660, &args![this, param]);
            let taken = calls(&log, MANAGER + 0x14);
            if id == 0xffff {
                assert!(taken.is_empty());
            } else {
                assert_eq!(taken, vec![vec![manager, 7, 0x55]]);
            }
            assert_eq!(calls(&log, OBJECT), vec![vec![this, 1]]);
        }
    }

    /// Runs the penetration query `address` for sphere A at `(0, 0, z)` and checks the collector
    /// was told about the bodies exactly when `told`.
    fn check_penetration(address: u32, z: f32, virtual_method: bool, told: bool) {
        let mut s = scene(z, 0.0);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let words = if virtual_method {
            vec![0x4000_0000, s.a, s.b, 0xcace, collector]
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
    }

    #[test]
    fn static_get_penetrations_tells_the_collector_of_touching_bodies_00d3f6a0() {
        check_penetration(0x00d3_f6a0, 2.0, false, true);
        check_penetration(0x00d3_f6a0, 4.0, false, false);
        // Exactly touching (distance 3 against a radius sum of 3) is not penetrating.
        check_penetration(0x00d3_f6a0, 3.0, false, false);
        // The same centre is the deepest penetration.
        check_penetration(0x00d3_f6a0, 0.0, false, true);
        // A NaN is not below the square.
        check_penetration(0x00d3_f6a0, f32::NAN, false, false);
    }

    #[test]
    fn get_penetrations_runs_the_static_version_00d3fcc0() {
        check_penetration(0x00d3_fcc0, 2.0, true, true);
        check_penetration(0x00d3_fcc0, 4.0, true, false);
    }

    /// What a run of `processCollision` left behind.
    struct Processed {
        s: Scene,
        this: u32,
        output: u32,
        free: u32,
    }

    /// Runs `processCollision` for sphere A at `(0, 0, z)`, the contact point id `id` and the id
    /// the contact manager makes.
    fn process(z: f32, tolerance: f32, id: u16, made: u32) -> Processed {
        let mut s = scene(z, tolerance);
        let manager = logging_object(&mut s.e, &s.log, MANAGER);
        log_double(&mut s.e, &s.log, MANAGER + 0xc, made);
        let this = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(this + 8, manager);
        s.e.mem.set_u16(this + 0xc, id);
        let output = s.e.mem.alloc(0x40);
        let free = s.e.mem.alloc(0x100);
        s.e.mem.set_u32(output, free);
        s.e.mem.set_u32(output + 4, 0x7777);
        s.e.call(0x00d3_f780, &[this, s.a, s.b, s.input, output]);
        check_profile(&s);
        Processed {
            s,
            this,
            output,
            free,
        }
    }

    #[test]
    fn a_touching_contact_gets_an_id_and_is_added_00d3f780() {
        let p = process(2.0, 0.0, 0xffff, 0x42);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0xc),
            vec![vec![manager, p.s.a, p.s.b, p.s.input, p.output, 0, p.free]]
        );
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0x42);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 0x42);
        // Distance 2, inverse 0.5: normal (0, 0, 1), position 2 * normal + (0, 0, 0, 1) and the
        // separating distance 4 * 0.5 - 3.
        assert_eq!(
            floats(&p.s.e, p.free),
            [0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 1.0, -1.0]
        );
    }

    #[test]
    fn the_tolerance_widens_the_reach_00d3f780() {
        // Distance 4 against a radius sum of 3: apart, or touching with a tolerance above 1.
        let p = process(4.0, 1.0, 0xffff, 0x42);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        let p = process(4.0, 1.5, 0xffff, 0x42);
        assert_eq!(calls(&p.s.log, MANAGER + 0xc).len(), 1);
        assert_eq!(
            floats(&p.s.e, p.free),
            [0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 1.0, 1.0]
        );
    }

    #[test]
    fn coincident_centres_use_the_constant_as_the_normal_00d3f780() {
        let p = process(0.0, 0.0, 0xffff, 0x42);
        // Position: 2 * constant + (0, 0, 0, 1) (the fourth lane included); normal: the
        // constant with the negated radius sum in the fourth lane.
        assert_eq!(
            floats(&p.s.e, p.free),
            [0.5, 1.0, 1.5, 15.0, 0.25, 0.5, 0.75, -3.0]
        );
    }

    #[test]
    fn a_contact_with_an_id_is_added_without_asking_00d3f780() {
        let p = process(2.0, 0.0, 9, 0x42);
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 9);
        assert_eq!(p.s.e.mem.u32(p.output), p.free + 0x30);
        assert_eq!(p.s.e.mem.u32(p.free + 0x20), 9);
    }

    #[test]
    fn a_contact_the_manager_gives_no_id_is_skipped_00d3f780() {
        let p = process(2.0, 0.0, 0xffff, 0xffff);
        assert_eq!(calls(&p.s.log, MANAGER + 0xc).len(), 1);
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
    }

    #[test]
    fn apart_bodies_give_their_id_back_00d3f780() {
        let p = process(4.0, 0.0, 7, 0x42);
        let manager = p.s.e.mem.u32(p.this + 8);
        assert_eq!(
            calls(&p.s.log, MANAGER + 0x14),
            vec![vec![manager, 7, 0x7777]]
        );
        assert_eq!(p.s.e.mem.u16(p.this + 0xc), 0xffff);
        assert_eq!(p.s.e.mem.u32(p.output), p.free);
        // Without an id there is nothing to give back.
        let p = process(4.0, 0.0, 0xffff, 0x42);
        assert!(calls(&p.s.log, MANAGER + 0x14).is_empty());
        assert!(calls(&p.s.log, MANAGER + 0xc).is_empty());
    }

    /// Runs the closest point query `address` for sphere A at `(0, 0, z)` and returns what the
    /// collector got.
    fn closest(address: u32, z: f32, tolerance: f32, virtual_method: bool) -> (Points, u32, u32) {
        let mut s = scene(z, tolerance);
        let collector = logging_object(&mut s.e, &s.log, COLLECTOR);
        let got: Points = Rc::new(RefCell::new(Vec::new()));
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
        check_profile(&s);
        (got, s.a, s.b)
    }

    fn check_closest_points(address: u32, virtual_method: bool) {
        // Distance 2: normal (0, 0, 1), position (0, 0, 2, 1), separation 4 * 0.5 - 3.
        let (got, a, b) = closest(address, 2.0, 0.0, virtual_method);
        assert_eq!(
            *got.borrow(),
            vec![([0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 1.0, -1.0], a, b)]
        );
        // Apart, or touching through the tolerance.
        let (got, _, _) = closest(address, 4.0, 0.0, virtual_method);
        assert!(got.borrow().is_empty());
        let (got, _, _) = closest(address, 4.0, 1.0, virtual_method);
        assert!(got.borrow().is_empty());
        let (got, a, b) = closest(address, 4.0, 1.5, virtual_method);
        assert_eq!(
            *got.borrow(),
            vec![([0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 1.0, 1.0], a, b)]
        );
        // Coincident centres: the constant is the normal; the fourth lane of the position is
        // that of the centre (the stack slot reads zero here).
        let (got, a, b) = closest(address, 0.0, 0.0, virtual_method);
        assert_eq!(
            *got.borrow(),
            vec![([0.5, 1.0, 1.5, 1.0, 0.25, 0.5, 0.75, -3.0], a, b)]
        );
    }

    #[test]
    fn get_closest_points_hands_the_contact_to_the_collector_00d3f980() {
        check_closest_points(0x00d3_f980, true);
    }

    #[test]
    fn static_get_closest_points_hands_the_contact_to_the_collector_00d3fb20() {
        check_closest_points(0x00d3_fb20, false);
    }
}
