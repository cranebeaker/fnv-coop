//! Havok's ragdoll and limited-hinge constraints for the solver (B1 PR 8,
//! docs/PHYSICS.md "Ragdoll constraints"): the constraint atoms the NIFs'
//! `bhkRagdollConstraint`, `bhkLimitedHingeConstraint` and
//! `bhkBallAndSocketConstraint` data carry, and the Jacobian builder that
//! turns them into the solver's schemas (`hkpConstraintAtom` types in
//! Xbox PDB names, `00d6f460` with `00d6ced0`, `00dcee80` and `00dcefb0`).
//! Translated from FalloutNV.exe 1.4.0.525; names are the Xbox
//! prototype's (ADR-0002), addresses the PC's (ADR-0003).
//!
//! The atoms of each data object, in memory order (the builder walks them
//! so), as the game's constructors leave them (`00cdf2f0` and `00cdf460`
//! for the ragdoll, `00cb9090` and `00cb9170` for the limited hinge,
//! `00d54290` for the ball and socket) and its loader sets them from the
//! NIF (`00cc6830`: the axes, pivots, angle limits and `00cde9f0`, the most
//! friction torque):
//!
//! | Data | Atoms |
//! | --- | --- |
//! | ragdoll | transforms, ragdoll motors, angular friction (axes 0–2), twist limit (twist axis 0, reference axis 1), cone limit (A's 0 against B's 0), planes limit (A's 0 against B's 1), ball socket |
//! | limited hinge | transforms, angular motor, angular friction (axis 0), angular limit (axis 0), 2-D angular (free axis 0), ball socket |
//! | ball and socket | translations, ball socket |
//!
//! Everything here is in Havok units, positions relative to the caller's
//! origin (as [`crate::solver`]).

use crate::havok::{ufloat8, SolverInfo, UFLOAT8};
use crate::solver::{Accumulator, AngularJacobian, Jacobian, QueryIn, Schema, Step};
use crate::vec::*;
use crate::Vec3;

/// The single-precision epsilon the builders add (`1.1920929e-07`).
const EPS: f32 = 1.192_092_9e-7;

/// The angular limits' tau factor the ragdoll's constructor sets (`00cdf2f0`:
/// the float `01030ff0`, stored for the twist, the cone and the planes).
pub const RAGDOLL_TAU_FACTOR: f32 = 0.8;
/// The hinge limit's (`00cb9090`: `1.0`).
pub const HINGE_TAU_FACTOR: f32 = 1.0;
/// The cone atom's least angle in the ragdoll's constructor (`00cdf2f0`,
/// the float `01024100`): the file gives only the widest.
pub const CONE_LEAST: f32 = -100.0;
/// The offset the cone atom keeps in the joint's runtime data
/// (`00cdea00(1)` sets its `m_memOffsetToAngleOffset` to 56, the byte
/// after the atoms' results): its presence makes the builder recover the
/// cone limit's penetration slowly.
pub const CONE_STABILIZED: bool = true;

/// How a cone atom measures its angle (`m_angleMeasurementMode`, `+5`; the
/// tables at `010d33d8`): the angle between its two axes, or its
/// complement to a right angle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConeMode {
    /// Zero when the vectors are aligned.
    Aligned,
    /// Zero when they are perpendicular.
    Perpendicular,
}

/// One constraint atom after the transforms (types of `hkpConstraintAtom`,
/// PDB names; the builder's switch is on the number).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Atom {
    /// `hkpAngFrictionConstraintAtom` (0x11): friction torque about `count`
    /// of the first body's frame axes from `first`, at most `max_torque`;
    /// left out when 0.
    AngularFriction {
        first: usize,
        count: usize,
        max_torque: f32,
    },
    /// `hkpTwistLimitConstraintAtom` (0xf): the turn of the first body's
    /// `ref_axis` against the second's about the average of their
    /// `twist_axis`es.
    Twist {
        twist_axis: usize,
        ref_axis: usize,
        min: f32,
        max: f32,
        tau_factor: f32,
    },
    /// `hkpConeLimitConstraintAtom` (0x10): the angle between the first
    /// body's `axis` and the second's `ref_axis`.
    Cone {
        axis: usize,
        ref_axis: usize,
        mode: ConeMode,
        min: f32,
        max: f32,
        tau_factor: f32,
        /// A memory offset to the angle offset (`+6`, above 1): recover
        /// slowly.
        stabilized: bool,
    },
    /// `hkpAngLimitConstraintAtom` (0xe): the turn of the first body's
    /// next axis against the second's about `axis`.
    AngularLimit {
        axis: usize,
        min: f32,
        max: f32,
        tau_factor: f32,
    },
    /// `hkp2dAngConstraintAtom` (0xc): the first body's `free_axis` stays
    /// along the second's.
    Angular2d { free_axis: usize },
    /// `hkpBallSocketConstraintAtom` (5): the pivots meet. `stabilization`
    /// is its velocity stabilization factor (an hkUFloat8, `+3`).
    BallSocket { stabilization: f32 },
}

impl Atom {
    /// The slots it keeps in the joint's runtime data (a result and its
    /// solver data each; 0 for the ones that keep none).
    fn slots(&self) -> usize {
        match *self {
            Atom::AngularFriction { count, .. } => count,
            Atom::Twist { .. } | Atom::Cone { .. } | Atom::AngularLimit { .. } => 1,
            Atom::Angular2d { .. } => 2,
            Atom::BallSocket { .. } => 3,
        }
    }
}

/// The atoms of a ragdoll constraint's data for the angles the NIF gives:
/// the cone's widest, the plane's and the twist's range, the most friction
/// torque.
pub fn ragdoll_atoms(
    cone: f32,
    plane_range: (f32, f32),
    twist_range: (f32, f32),
    max_friction: f32,
) -> Vec<Atom> {
    vec![
        Atom::AngularFriction {
            first: 0,
            count: 3,
            max_torque: max_friction,
        },
        Atom::Twist {
            twist_axis: 0,
            ref_axis: 1,
            min: twist_range.0,
            max: twist_range.1,
            tau_factor: RAGDOLL_TAU_FACTOR,
        },
        Atom::Cone {
            axis: 0,
            ref_axis: 0,
            mode: ConeMode::Aligned,
            min: CONE_LEAST,
            max: cone,
            tau_factor: RAGDOLL_TAU_FACTOR,
            stabilized: CONE_STABILIZED,
        },
        Atom::Cone {
            axis: 0,
            ref_axis: 1,
            mode: ConeMode::Perpendicular,
            min: plane_range.0,
            max: plane_range.1,
            tau_factor: RAGDOLL_TAU_FACTOR,
            stabilized: false,
        },
        ball_socket(),
    ]
}

/// The atoms of a limited hinge's data for the angle range and the most
/// friction torque the NIF gives.
pub fn hinge_atoms(range: (f32, f32), max_friction: f32) -> Vec<Atom> {
    vec![
        Atom::AngularFriction {
            first: 0,
            count: 1,
            max_torque: max_friction,
        },
        Atom::AngularLimit {
            axis: 0,
            min: range.0,
            max: range.1,
            tau_factor: HINGE_TAU_FACTOR,
        },
        Atom::Angular2d { free_axis: 0 },
        ball_socket(),
    ]
}

/// The atoms of a ball and socket's data.
pub fn ball_atoms() -> Vec<Atom> {
    vec![ball_socket()]
}

/// A ball socket atom as every constructor leaves it (`00d54290`,
/// `00cdf460`, `00cb9170`: stabilization 1.0 stored through the hkUFloat8
/// conversion `00ca9360`, no most impulse).
fn ball_socket() -> Atom {
    Atom::BallSocket {
        stabilization: UFLOAT8[ufloat8(1.0) as usize],
    }
}

/// What a joint keeps from step to step (the constraint's runtime data):
/// each result slot's solver data (`m_internalSolverData`, `+4`), and the
/// cone limit's allowed angle offset (the 56 bytes past the atoms' results
/// the atom's `+6` names). A new constraint's runtime is taken as all
/// zero (the data's `Runtime` constructor isn't traced).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct JointRuntime {
    pub data: Vec<f32>,
    pub cone_offset: f32,
}

impl JointRuntime {
    pub fn new(atoms: &[Atom]) -> JointRuntime {
        JointRuntime {
            data: vec![0.0; atoms.iter().map(Atom::slots).sum()],
            cone_offset: 0.0,
        }
    }
}

/// A body's frame in the world (Havok units, relative to the caller's
/// origin): the origin of the body's model space and its axes as columns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    pub origin: Vec3,
    pub rotation: [Vec3; 3],
}

/// A joint's frames (`hkpSetLocalTransformsConstraintAtom`): in each
/// body's model space the three axes of the joint's frame and its pivot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frames {
    pub axes: [[Vec3; 3]; 2],
    pub pivots: [Vec3; 2],
}

/// A frame in the world: its axes and its position.
struct World {
    axes: [Vec3; 3],
    pos: Vec3,
}

fn world(pose: &Pose, axes: &[Vec3; 3], pivot: Vec3) -> World {
    World {
        axes: axes.map(|v| crate::solver::apply_cols(&pose.rotation, v)),
        pos: add(
            pose.origin,
            crate::solver::apply_cols(&pose.rotation, pivot),
        ),
    }
}

/// `hkMath::atan2Approximation` (`00cbff80`, the game's own polynomial):
/// the angle of (`x`, `y`), in (−π, π].
// Translated from 00cbff80 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn atan2(y: f32, x: f32) -> f32 {
    let c1 = f64::from_bits(0x3fbe_ff08_8000_0000); // 010c92f8: 0.121079
    let c2 = f64::from_bits(0x3fb7_f11c_8000_0000); // 010c92f0: 0.09352282
    let half_pi = f64::from_bits(0x3ff9_21fb_6000_0000); // 01030f38
    let pi = f64::from_bits(0x4009_21fb_6000_0000); // 0101ff40
    let poly = |t: f64| (t - t * c1 * t) - t * c2 * t * t;
    let (ay, ax) = (f64::from(y.abs()), f64::from(x.abs()));
    let eps = f64::from(EPS);
    let mut r = if ax < ay {
        half_pi - poly(ax / (ay + eps))
    } else {
        poly(ay / (ax + eps))
    };
    if x < 0.0 {
        r = pi - r;
    }
    if y < 0.0 {
        r = -r;
    }
    r as f32
}

/// What a builder needs besides the bodies.
pub struct Context<'a> {
    pub q: &'a QueryIn,
    /// The solver info's tau (`hkpConstraintQueryIn` `+0x40`).
    pub tau: f32,
}

/// Which accumulator a body is, and the bodies' accumulators (the first
/// body's index in the solve, the second's).
pub struct Sides<'a> {
    pub ids: (usize, usize),
    pub a: &'a Accumulator,
    pub b: &'a Accumulator,
}

fn angular_row(a: &Accumulator, b: &Accumulator, dir: Vec3) -> (AngularJacobian, f32) {
    let angular_a = crate::solver::apply_cols(&a.to_local, dir);
    let angular_b = crate::solver::apply_cols(&b.to_local, scale(dir, -1.0));
    let mut eff = 0.0;
    for k in 0..3 {
        eff += angular_a[k] * angular_a[k] * a.inv_inertia[k]
            + angular_b[k] * angular_b[k] * b.inv_inertia[k];
    }
    (
        AngularJacobian {
            angular_a,
            inv_jac_diag: 0.0,
            angular_b,
            rhs: 0.0,
        },
        eff,
    )
}

/// The unit axes the ball socket's three rows lie along (`01268370`).
const AXES: [Vec3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// The ball socket atom's three Jacobians (`00d6ced0`): along each world
/// axis, the pivots' velocity apart as a linear row, the right-hand side
/// the distance the pivots are apart less what the bodies' turning is
/// predicted to change it by in the stabilization time (the factor ÷ the
/// right-hand side's factor) — so with a factor of 1 the turning is
/// left out of what the row holds.
// Translated from 00d6ced0 (decompiled, FalloutNV.exe 1.4.0.525)
fn build_ball_socket(
    stabilization: f32,
    (fa, fb): (&World, &World),
    cx: &Context,
    s: &Sides,
    out: &mut Vec<Schema>,
) {
    let (a, b) = (s.a, s.b);
    let q = cx.q;
    let l = (1.0 / q.rhs_factor) * stabilization;
    let ra = sub(fa.pos, a.center);
    let rb = sub(fb.pos, b.center);
    // The angular velocities in the world (`004b4ab0`: the transpose of the
    // world-to-body columns applied).
    let to_world = |acc: &Accumulator| {
        crate::solver::apply_cols(&crate::solver::transpose_cols(acc.to_local), acc.angular)
    };
    let (wa, wb) = (to_world(a), to_world(b));
    let gap = sub(fb.pos, fa.pos);
    let err = add(gap, scale(sub(cross(wa, ra), cross(wb, rb)), l));
    for (k, e) in AXES.iter().enumerate() {
        let angular_a = crate::solver::apply_cols(&a.to_local, cross(ra, *e));
        let angular_b = crate::solver::apply_cols(&b.to_local, cross(*e, rb));
        let mut eff = a.inv_mass + b.inv_mass + EPS;
        for i in 0..3 {
            eff += angular_a[i] * angular_a[i] * a.inv_inertia[i]
                + angular_b[i] * angular_b[i] * b.inv_inertia[i];
        }
        out.push(Schema::Linear {
            jac: Jacobian {
                linear: *e,
                rhs: err[k] * q.rhs_factor,
                angular_a,
                inv_jac_diag: q.virt_mass_factor / eff,
                angular_b,
                extra: eff,
            },
        });
    }
}

/// The 2-D angular atom's two rows (`00d6f460` case 0xc, `00dcee80`):
/// the first body's free axis kept perpendicular to the second's other two
/// axes, each row turning about one of them.
// Translated from 00d6f460 and 00dcee80 (decompiled, FalloutNV.exe 1.4.0.525)
fn build_angular_2d(
    free_axis: usize,
    (fa, fb): (&World, &World),
    cx: &Context,
    s: &Sides,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    let a_axis = fa.axes[free_axis];
    let b1 = fb.axes[(free_axis + 1) % 3];
    let b2 = fb.axes[(free_axis + 2) % 3];
    // (direction, the axis it is held against)
    for (k, (dir, other)) in [(b2, b1), (scale(b1, -1.0), b2)].into_iter().enumerate() {
        let (mut row, eff) = angular_row(s.a, s.b, dir);
        row.inv_jac_diag = cx.q.virt_mass_factor / (eff + EPS);
        row.rhs = -dot(a_axis, other) * cx.q.rhs_factor;
        out.push(Schema::AngularRow {
            row,
            slot: slot + k,
        });
    }
}

/// A limit's row (`00d6f460` at `00d70646`): about `axis` (the first body
/// pushed along it, the second against), the angle `angle` against
/// `min`…`max`, with `tau` the atom's tau factor × the solver's (× for the
/// twist, the cosine of half the axes' angle squared).
fn push_limit(
    axis: Vec3,
    (min, max, angle, tau): (f32, f32, f32, f32),
    cx: &Context,
    s: &Sides,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    let (mut row, eff) = angular_row(s.a, s.b, axis);
    row.inv_jac_diag = tau / (eff + EPS);
    let neg_inv_dt = -cx.q.substep_inv_dt;
    row.rhs = neg_inv_dt * angle;
    out.push(Schema::AngularLimit {
        row,
        lower: min * neg_inv_dt * tau,
        upper: neg_inv_dt * max * tau,
        tau,
        slot,
    });
}

/// The angular limit atom (`00d6f460` case 0xe): the angle of the first
/// body's next axis in the second's plane about `axis`, kept within one
/// turn of what the last solve left (the solver data).
// Translated from 00d6f460 (decompiled, FalloutNV.exe 1.4.0.525)
#[allow(clippy::too_many_arguments)]
fn build_angular_limit(
    (axis, min, max, tau_factor): (usize, f32, f32, f32),
    (fa, fb): (&World, &World),
    cx: &Context,
    s: &Sides,
    rt: &JointRuntime,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    let a1 = fa.axes[(axis + 1) % 3];
    let b2 = fb.axes[(axis + 2) % 3];
    let bi = fb.axes[axis];
    let y = dot(b2, a1);
    let x = -dot(cross(bi, b2), a1);
    let mut angle = atan2(y, x);
    let previous = -rt.data[slot];
    let two_pi = 6.283_185_5;
    while std::f32::consts::PI < previous - angle {
        angle += two_pi;
    }
    while std::f32::consts::PI < angle - previous {
        angle -= two_pi;
    }
    push_limit(
        fa.axes[axis],
        (min, max, angle, tau_factor * cx.tau),
        cx,
        s,
        slot,
        out,
    );
}

/// The twist limit atom (`00d6f460` case 0xf): the turn of the first
/// body's reference axis against the second's about the average of their
/// twist axes; its tau scaled by the squared cosine of half the angle
/// between the twist axes.
// Translated from 00d6f460 (decompiled, FalloutNV.exe 1.4.0.525)
fn build_twist(
    (t, r, min, max, tau_factor): (usize, usize, f32, f32, f32),
    (fa, fb): (&World, &World),
    cx: &Context,
    s: &Sides,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    let sum = add(fa.axes[t], fb.axes[t]);
    let len = length(sum);
    let (half, w) = if len <= 1e-16 {
        (0.0, fb.axes[t])
    } else {
        (len * 0.5, scale(sum, 1.0 / len))
    };
    let br = fb.axes[r];
    let ar = fa.axes[r];
    let u = cross(w, br);
    let y = dot(ar, u);
    let x = dot(ar, cross(u, w));
    let angle = atan2(y, x);
    let tau = half * half * tau_factor * cx.tau;
    push_limit(w, (min, max, angle, tau), cx, s, slot, out);
}

/// The cone limit atom (`00d6f460` case 0x10): the angle between the first
/// body's axis and the second's reference axis (or its complement), about
/// their cross product; left out when they are parallel. A stabilized atom
/// keeps an allowed angle offset in the runtime data and moves the angle
/// by it, as a contact's allowed penetration moves its distance.
// Translated from 00d6f460 (decompiled, FalloutNV.exe 1.4.0.525)
#[allow(clippy::too_many_arguments)]
fn build_cone(
    atom: &Atom,
    (fa, fb): (&World, &World),
    cx: &Context,
    s: &Sides,
    rt: &mut JointRuntime,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    let Atom::Cone {
        axis,
        ref_axis,
        mode,
        min,
        max,
        tau_factor,
        stabilized,
    } = *atom
    else {
        return;
    };
    let a = fa.axes[axis];
    let b = fb.axes[ref_axis];
    let c = cross(a, b);
    let c2 = dot(c, c);
    if EPS > c2 {
        return;
    }
    let sin = c2.sqrt();
    let theta = atan2(sin, dot(a, b));
    // The tables at `010d33d8`: the angle's factor and offset, and the
    // axis's sign, by mode.
    let (mul, add_angle, sign) = match mode {
        ConeMode::Aligned => (1.0, 0.0, -1.0),
        ConeMode::Perpendicular => (-1.0, std::f32::consts::FRAC_PI_2, 1.0),
    };
    let mut angle = theta * mul + add_angle;
    let inv = 1.0 / sin;
    let axis_w = scale(c, inv * sign);
    if stabilized {
        let offset = rt.cone_offset;
        let mut moved = angle - offset;
        let expected = -rt.data[slot];
        let gap = expected - moved;
        let half = cx.q.frame_dt * 0.5;
        let step = if half < offset * 0.05 {
            offset * 0.05
        } else {
            half
        };
        let mut left = offset - step;
        moved += step;
        if (step + step + half) * 0.5 < -gap && expected != 0.0 {
            left -= gap;
            moved += gap;
        }
        rt.cone_offset = if EPS < left { left } else { EPS };
        angle = moved;
    }
    push_limit(
        axis_w,
        (min, max, angle, tau_factor * cx.tau),
        cx,
        s,
        slot,
        out,
    );
}

/// The angular friction atom (`00d6f460` case 0x11, `00dcefb0`): one row
/// per axis, holding the angular velocity about it where the last solve
/// left it (the solver data), at most the torque × the micro step.
// Translated from 00d6f460 and 00dcefb0 (decompiled, FalloutNV.exe 1.4.0.525)
fn build_angular_friction(
    (first, count, max_torque): (usize, usize, f32),
    fa: &World,
    cx: &Context,
    s: &Sides,
    rt: &JointRuntime,
    slot: usize,
    out: &mut Vec<Schema>,
) {
    if max_torque == 0.0 {
        return;
    }
    for j in 0..count {
        let (mut row, eff) = angular_row(s.a, s.b, fa.axes[first + j]);
        row.inv_jac_diag = 1.0 / (eff + EPS);
        row.rhs = rt.data[slot + j] * cx.q.substep_inv_dt;
        out.push(Schema::AngularFriction {
            row,
            max_impulse: cx.q.micro_step_dt * max_torque,
            slot: slot + j,
        });
    }
}

/// A joint's schemas (`00d6f460`'s walk over the atoms, after the header
/// that names the two bodies): `constraint` is the joint's index in the
/// export, `poses` the bodies' frames in the world.
// Translated from 00d6f460 (decompiled, FalloutNV.exe 1.4.0.525)
#[allow(clippy::too_many_arguments)]
pub fn build(
    atoms: &[Atom],
    frames: &Frames,
    poses: [&Pose; 2],
    constraint: usize,
    cx: &Context,
    s: &Sides,
    rt: &mut JointRuntime,
    out: &mut Vec<Schema>,
) {
    out.push(Schema::Header {
        a: s.ids.0,
        b: s.ids.1,
        constraint,
    });
    let fa = world(poses[0], &frames.axes[0], frames.pivots[0]);
    let fb = world(poses[1], &frames.axes[1], frames.pivots[1]);
    let both = (&fa, &fb);
    let mut slot = 0;
    for atom in atoms {
        match *atom {
            Atom::AngularFriction {
                first,
                count,
                max_torque,
            } => build_angular_friction((first, count, max_torque), &fa, cx, s, rt, slot, out),
            Atom::Twist {
                twist_axis,
                ref_axis,
                min,
                max,
                tau_factor,
            } => build_twist(
                (twist_axis, ref_axis, min, max, tau_factor),
                both,
                cx,
                s,
                slot,
                out,
            ),
            Atom::Cone { .. } => build_cone(atom, both, cx, s, rt, slot, out),
            Atom::AngularLimit {
                axis,
                min,
                max,
                tau_factor,
            } => build_angular_limit((axis, min, max, tau_factor), both, cx, s, rt, slot, out),
            Atom::Angular2d { free_axis } => build_angular_2d(free_axis, both, cx, s, slot, out),
            Atom::BallSocket { stabilization } => {
                build_ball_socket(stabilization, both, cx, s, out)
            }
        }
        slot += atom.slots();
    }
}

/// What the joints keep from the solve (`hkSolverExport` `00def570`, the
/// rows' arms): each angular row's solver data — its right-hand side × the
/// substep less the row on the integrated velocities × the step (the
/// angle expected after the step, negated; the friction's, × the last
/// scale). `first` is the index the joints' headers start at.
// Translated from 00def570 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn export(
    s: &SolverInfo,
    step: &Step,
    schemas: &[Schema],
    results: &[f32],
    accs: &[Accumulator],
    first: usize,
    runtimes: &mut [JointRuntime],
) {
    let frame = s.num_steps as f32 * step.substep_dt;
    let swept = |row: &AngularJacobian, a: &Accumulator, b: &Accumulator| {
        row.rhs * step.substep_dt
            - (dot(row.angular_a, a.sum_angular) + dot(row.angular_b, b.sum_angular)) * frame
    };
    let (mut ia, mut ib, mut c) = (0usize, 0usize, 0usize);
    let mut r = 0usize;
    for sch in schemas {
        match sch {
            Schema::Header { a, b, constraint } => {
                ia = *a;
                ib = *b;
                c = *constraint;
            }
            Schema::AngularRow { row, slot } | Schema::AngularLimit { row, slot, .. }
                if c >= first =>
            {
                runtimes[c - first].data[*slot] = swept(row, &accs[ia], &accs[ib]);
            }
            Schema::AngularFriction { row, slot, .. } if c >= first => {
                runtimes[c - first].data[*slot] = swept(row, &accs[ia], &accs[ib]) * results[r + 1];
            }
            _ => {}
        }
        r += sch.results();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::{Accumulator, QueryIn};

    #[test]
    fn the_games_atan2_follows_the_real_one() {
        let mut worst = 0.0f32;
        for k in 0..720 {
            let a = (k as f32 - 360.0) * 0.5f32.to_radians() * 0.999;
            let (y, x) = a.sin_cos();
            worst = worst.max((atan2(x, y) - x.atan2(y)).abs());
        }
        // A polynomial good to about six thousandths of a radian (0.34 degrees).
        assert!(worst < 8e-3, "{worst}");
        assert_eq!(atan2(0.0, 1.0), 0.0);
        assert!((atan2(1.0, 0.0) - std::f32::consts::FRAC_PI_2).abs() < 1e-3);
    }

    #[test]
    fn atoms_keep_the_slots_the_runtime_data_has() {
        let ragdoll = ragdoll_atoms(0.5, (-0.1, 0.1), (-0.2, 0.2), 1.0);
        // Three frictions, the twist, two cones and the ball socket's three.
        assert_eq!(JointRuntime::new(&ragdoll).data.len(), 3 + 1 + 1 + 1 + 3);
        let hinge = hinge_atoms((0.0, 1.0), 0.0);
        assert_eq!(JointRuntime::new(&hinge).data.len(), 1 + 1 + 2 + 3);
    }

    fn two_bodies() -> (Accumulator, Accumulator, [Pose; 2]) {
        let cols = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let a = Accumulator::dynamic([0.0; 3], cols, [0.0; 3], [0.0; 3], [1.0; 3], 1.0, 3, 1.0);
        let b = Accumulator::dynamic(
            [2.0, 0.0, 0.0],
            cols,
            [0.0; 3],
            [0.0; 3],
            [1.0; 3],
            1.0,
            3,
            1.0,
        );
        let poses = [
            Pose {
                origin: [0.0; 3],
                rotation: cols,
            },
            Pose {
                origin: [2.0, 0.0, 0.0],
                rotation: cols,
            },
        ];
        (a, b, poses)
    }

    #[test]
    fn a_ball_socket_pulls_the_pivots_together() {
        let (a, b, poses) = two_bodies();
        let solver = SolverInfo::new();
        let step = Step::new(&solver, 0.016, [0.0; 3]);
        let q = QueryIn::new(&solver, &step);
        let cx = Context {
            q: &q,
            tau: solver.tau,
        };
        // A's pivot at its origin, B's one unit to its side: a gap of 1 in x
        // after B's origin at 2.
        let frames = Frames {
            axes: [[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]; 2],
            pivots: [[0.0; 3], [-1.0, 0.0, 0.0]],
        };
        let atoms = ball_atoms();
        let mut rt = JointRuntime::new(&atoms);
        let mut out = Vec::new();
        build(
            &atoms,
            &frames,
            [&poses[0], &poses[1]],
            0,
            &cx,
            &Sides {
                ids: (1, 2),
                a: &a,
                b: &b,
            },
            &mut rt,
            &mut out,
        );
        // Header and three rows; the x row's right-hand side is the gap
        // (B's pivot at 1 less A's at 0) × the factor.
        assert_eq!(out.len(), 4);
        let Schema::Linear { jac } = &out[1] else {
            panic!("{:?}", out[1])
        };
        assert!((jac.rhs - 1.0 * q.rhs_factor).abs() < 1e-3, "{}", jac.rhs);
        assert!(jac.inv_jac_diag > 0.0);
    }
}
