//! Continuous collision for free bodies: which bodies Havok sweeps against
//! the world, and what the simplified time of impact does to them.
//!
//! The game's world is continuous (`iSimType` 1, Bethesda's cinfo
//! `00c681c0`: `hkpContinuousSimulation`, `hkpContinuousSimulation::
//! collideInternal` `00d0ef40`, Xbox PDB). Each body has a quality
//! (`hkpCollidableQualityType`, [`crate::rigid::RigidSetup::quality`]); the
//! dispatcher's quality table (`hkpCollisionDispatcher::
//! initCollisionQualityInfo` `00cfb570`, at `+0x1bb0`) says, for two
//! bodies' qualities, which collision quality their agent works at
//! ([`pair_quality`]): 1 is a plain step, 2 the simplified time of impact,
//! 3 to 5 the full one. Every clutter model the game ships that was
//! looked at (tumbleweeds, bottles) carries quality 3, debris with
//! simplified time of impact (`00c8d4a0` names the cinfo's qualities; the
//! model's byte is the cinfo's, `00c8ea30`): against the world (quality
//! 0, fixed) that is 2.
//!
//! Traced here (`hkpContinuousSimulation::advanceTimeInternal` `00d0ed10`
//! → `handleAllToisTill` `00d0e9c0` → `handleSimpleToi` `00d0e210`): a
//! time-of-impact event queued by the narrowphase (`00d0d150`, 250 events
//! at most, the world's `sizeOfToiEventQueue`; the quality info's `+0x14`
//! byte marks it simple) is handled in time order; handling a simple one
//! sets each body of quality 3 to its pose at the event's time
//! (`hkSweptTransformUtil` lerp, `00cf1d50`, [`swept_pose`]: the swept
//! transform then starts and ends there, the velocities untouched),
//! drops the queued events of that body and its agents' predictions. The
//! contacts at that pose are made by the next collide and solved by the
//! next step. The rest of the frame's travel is lost.
//!
//! Not translated: how the narrowphase finds an event's time (the
//! predictive agents' linear cast with their cached separating plane,
//! `00cfedd0` and the agent tables at `dispatcher +0x16b8`/`+0x16dc`) and
//! the separation it stops at (the quality info's fractions of the
//! bodies' allowed penetration depth, `00cfb570`); here the first time a
//! body's feature points touch a triangle the step carried them through
//! ([`first_touch`], conservative advancement), and an event is made only
//! when a point ends the step behind the triangle. The full time of impact
//! (quality 3 to 5 against the world, `simulateToi` `00d100a0`: a
//! solve of the contacts at the time, then the rest of the step again) and
//! events between two bodies are not translated: bodies of those
//! qualities step as before.

use crate::ragdoll::{mat_vec, quat_mat, Quat};
use crate::vec::*;
use crate::{closest_on_triangle, Collider, Vec3};

/// `hkpCollidableQualityType` (Xbox PDB), as the cinfo stores it.
pub const FIXED: u8 = 0;
pub const KEYFRAMED: u8 = 1;
pub const DEBRIS: u8 = 2;
pub const DEBRIS_SIMPLE_TOI: u8 = 3;
pub const MOVING: u8 = 4;
pub const CRITICAL: u8 = 5;
pub const BULLET: u8 = 6;
pub const USER: u8 = 7;
pub const CHARACTER: u8 = 8;
pub const KEYFRAMED_REPORTING: u8 = 9;

/// Events the world's queue holds (`sizeOfToiEventQueue`, `00c90b80`; full
/// queue: an assert in `00d0d150`, the event lost).
pub const TOI_QUEUE: usize = 250;

/// The collision quality an agent between bodies of two qualities works
/// at (the table at `dispatcher +0x1bb0`, `00cfb570`): 0 none, 1 plain
/// step, 2 simplified time of impact, 3 time of impact, 4 higher, 5
/// forced, 6 character.
pub fn pair_quality(a: u8, b: u8) -> u8 {
    const TABLE: [[u8; 10]; 10] = [
        [0, 0, 1, 2, 4, 5, 4, 1, 6, 3],
        [0, 0, 1, 1, 3, 3, 3, 1, 3, 3],
        [1, 1, 1, 1, 1, 1, 3, 1, 1, 1],
        [2, 1, 1, 1, 1, 1, 3, 1, 1, 1],
        [4, 3, 1, 1, 1, 3, 3, 1, 3, 3],
        [5, 3, 1, 1, 3, 3, 3, 1, 3, 3],
        [4, 3, 3, 3, 3, 3, 3, 1, 3, 3],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [6, 3, 1, 1, 3, 3, 3, 1, 3, 3],
        [3, 3, 1, 1, 3, 3, 3, 1, 3, 3],
    ];
    match (TABLE.get(a as usize), b as usize) {
        (Some(row), b) if b < 10 => row[b],
        _ => 1,
    }
}

/// Whether a body of this quality gets a simplified time of impact
/// against the world's fixed bodies.
pub fn simple_toi_against_world(quality: u8) -> bool {
    pair_quality(FIXED, quality) == 2
}

/// A swept transform's rotation at the fraction `t` of its step
/// (`00cf1d50`): the two ends' sum normalized is the middle; each half
/// blends linearly towards it, the result normalized.
pub fn swept_rotation(q0: Quat, q1: Quat, t: f32) -> Quat {
    let norm = |q: Quat| {
        let l = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
        if l > 0.0 {
            q.map(|x| x / l)
        } else {
            [0.0, 0.0, 0.0, 1.0]
        }
    };
    // The end turned to the start's side, as the motion's quaternions are.
    let d = q0[0] * q1[0] + q0[1] * q1[1] + q0[2] * q1[2] + q0[3] * q1[3];
    let q1 = if d < 0.0 { q1.map(|x| -x) } else { q1 };
    let mid = norm([q0[0] + q1[0], q0[1] + q1[1], q0[2] + q1[2], q0[3] + q1[3]]);
    let blend = |a: Quat, b: Quat, s: f32| {
        norm([
            (1.0 - s) * a[0] + s * b[0],
            (1.0 - s) * a[1] + s * b[1],
            (1.0 - s) * a[2] + s * b[2],
            (1.0 - s) * a[3] + s * b[3],
        ])
    };
    if t >= 0.5 {
        blend(mid, q1, 2.0 * t - 1.0)
    } else {
        blend(q0, mid, 2.0 * t)
    }
}

/// A body's pose (centre of mass, turn) at the fraction `t` of a step from
/// `start` to `end` (`00cf1d50`).
pub fn swept_pose(start: (Vec3, Quat), end: (Vec3, Quat), t: f32) -> (Vec3, Quat) {
    (
        add(scale(start.0, 1.0 - t), scale(end.0, t)),
        swept_rotation(start.1, end.1, t),
    )
}

/// The first fraction of a step, from `start` to `end` (centre of mass and
/// turn), at which a point of the body (`points`: model-space offsets from
/// the centre of mass, each with its radius) touches a triangle of
/// `tris`, for a point the step carried through that triangle (it starts
/// in front, ends behind with the nearest point of the triangle straight
/// below it). Untranslated, see the module's notes: conservative
/// advancement on the distance, the points' speed bounded by the centre's
/// travel and the turn's arc.
pub fn first_touch(
    collider: &Collider,
    tris: &[u32],
    start: (Vec3, Quat),
    end: (Vec3, Quat),
    points: &[(Vec3, f32)],
) -> Option<f32> {
    let (r0, r1) = (quat_mat(start.1), quat_mat(end.1));
    let travel = length(sub(end.0, start.0));
    let dot_q = (start.1[0] * end.1[0]
        + start.1[1] * end.1[1]
        + start.1[2] * end.1[2]
        + start.1[3] * end.1[3])
        .abs()
        .min(1.0);
    let turn = 2.0 * dot_q.acos();
    let mut best: Option<f32> = None;
    for &(local, radius) in points {
        let p0 = add(start.0, mat_vec(&r0, local));
        let p1 = add(end.0, mat_vec(&r1, local));
        let bound = travel + turn * length(local);
        if bound < 1e-6 {
            continue;
        }
        for &t in tris {
            let [a, b, c] = collider.triangle(t);
            let r = radius + collider.shell(t);
            let mut face = normalize(cross(sub(b, a), sub(c, a)));
            if dot(face, sub(p0, a)) < 0.0 {
                face = scale(face, -1.0);
            }
            // Carried through: behind the plane at the end, the nearest
            // point of the triangle straight below.
            let s1 = dot(face, sub(p1, a));
            if s1 >= 0.0 {
                continue;
            }
            let q1 = closest_on_triangle(p1, a, b, c);
            if length(sub(p1, q1)) > -s1 + 1e-3 {
                continue;
            }
            let dist = |at: f32| {
                let (x, q) = swept_pose(start, end, at);
                let p = add(x, mat_vec(&quat_mat(q), local));
                length(sub(p, closest_on_triangle(p, a, b, c))) - r
            };
            // Already touching where the step began: the PSI's contact
            // has it (a bottle on its rail, clutter on the floor); a time
            // of impact is for a point that comes in from clear space, else
            // a body touching anything would be set back to its start every
            // step and never move.
            if dist(0.0) <= crate::havok::COLLISION_TOLERANCE * crate::HAVOK_UNIT {
                continue;
            }
            let mut at = 0.0f32;
            let mut found = false;
            for _ in 0..64 {
                let d = dist(at);
                if d <= 1e-3 {
                    found = true;
                    break;
                }
                at += d / bound;
                if at >= 1.0 {
                    break;
                }
            }
            if found && best.map_or(true, |b| at < b) {
                best = Some(at);
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_quality_table_is_symmetric_and_fixed_against_fixed_has_none() {
        for a in 0..10u8 {
            for b in 0..10u8 {
                assert_eq!(pair_quality(a, b), pair_quality(b, a), "{a} {b}");
            }
        }
        assert_eq!(pair_quality(FIXED, FIXED), 0);
    }

    #[test]
    fn clutter_of_quality_3_gets_a_simplified_toi_against_the_world() {
        assert!(simple_toi_against_world(DEBRIS_SIMPLE_TOI));
        assert!(!simple_toi_against_world(DEBRIS));
        assert!(!simple_toi_against_world(MOVING));
    }

    #[test]
    fn the_swept_pose_runs_from_the_start_to_the_end() {
        let a = ([0.0, 0.0, 10.0], [0.0, 0.0, 0.0, 1.0]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let b = ([0.0, 0.0, -10.0], [0.0, 0.0, h, h]);
        assert_eq!(swept_pose(a, b, 0.0).0, a.0);
        let (x, q) = swept_pose(a, b, 0.5);
        assert!(x[2].abs() < 1e-5);
        assert!((q[2] - 0.3826).abs() < 1e-2, "{q:?}");
        let e = swept_pose(a, b, 1.0);
        assert!((e.1[2] - h).abs() < 1e-5);
    }
}
